#!/usr/bin/env python3
"""Validate the prebuild descriptor alpha schema and warm-start seed."""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

import yaml
from jsonschema import Draft202012Validator, FormatChecker


DEFAULT_SCHEMA_REL = "schemas/runtime/prebuild_descriptor_alpha.schema.json"
DEFAULT_SEED_REL = "artifacts/templates/warm_start_descriptor_seed.yaml"
DEFAULT_TEMPLATE_SEED_REL = "artifacts/templates/workspace_template_seed.yaml"
DEFAULT_TSJS_BUNDLE_REL = "artifacts/bundles/tsjs_launch_bundle_alpha.yaml"
DEFAULT_PYTHON_BUNDLE_REL = "artifacts/bundles/python_launch_bundle_alpha.yaml"
DEFAULT_FIXTURE_REL = "fixtures/runtime/prebuild_descriptor_alpha/manifest.json"
DEFAULT_START_CENTER_REL = "crates/aureline-shell/src/start_center/mod.rs"
DEFAULT_DOC_REL = "docs/runtime/prebuild_descriptor_alpha.md"


@dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    ref: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        if payload["ref"] is None:
            payload.pop("ref")
        if not payload["details"]:
            payload.pop("details")
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--schema", default=DEFAULT_SCHEMA_REL)
    parser.add_argument("--seed", default=DEFAULT_SEED_REL)
    parser.add_argument("--template-seed", default=DEFAULT_TEMPLATE_SEED_REL)
    parser.add_argument("--tsjs-bundle", default=DEFAULT_TSJS_BUNDLE_REL)
    parser.add_argument("--python-bundle", default=DEFAULT_PYTHON_BUNDLE_REL)
    parser.add_argument("--fixture", default=DEFAULT_FIXTURE_REL)
    parser.add_argument("--start-center", default=DEFAULT_START_CENTER_REL)
    parser.add_argument("--doc", default=DEFAULT_DOC_REL)
    parser.add_argument("--report", default=None)
    parser.add_argument(
        "--render-warm-start-gallery",
        action="store_true",
        help="Print the deterministic warm-start projection after validation.",
    )
    return parser.parse_args()


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise SystemExit(f"missing JSON file: {path}") from exc
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def load_yaml(path: Path) -> Any:
    try:
        return yaml.safe_load(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise SystemExit(f"missing YAML file: {path}") from exc
    except yaml.YAMLError as exc:
        raise SystemExit(f"invalid YAML at {path}: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be an object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a list")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    clean = strip_fragment(ref)
    return bool(clean) and (repo_root / clean).exists()


def add_missing_ref_finding(
    findings: list[Finding], label: str, repo_root: Path, ref: str
) -> None:
    if not artifact_ref_exists(repo_root, ref):
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.missing_ref",
                message=f"{label} does not resolve: {ref}",
                remediation="Seed the referenced artifact or fix the reference.",
                ref=ref,
            )
        )


def schema_validate(schema: dict[str, Any], payload: dict[str, Any]) -> list[Finding]:
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema, format_checker=FormatChecker())
    findings: list[Finding] = []
    for error in sorted(validator.iter_errors(payload), key=lambda item: list(item.path)):
        path = ".".join(str(part) for part in error.path) or "<root>"
        findings.append(
            Finding(
                severity="error",
                check_id="prebuild_descriptor_alpha.schema.validation_failed",
                message=f"{path}: {error.message}",
                remediation="Update the seed artifact or schema so the alpha contract validates.",
                details={"schema_path": list(error.schema_path)},
            )
        )
    return findings


def collect_environment_capsules(
    template_seed: dict[str, Any],
) -> dict[str, dict[str, Any]]:
    capsules: dict[str, dict[str, Any]] = {}
    for idx, raw_capsule in enumerate(
        ensure_list(template_seed.get("environment_capsules"), "template_seed.environment_capsules")
    ):
        capsule = ensure_dict(raw_capsule, f"template_seed.environment_capsules[{idx}]")
        capsules[ensure_str(capsule.get("capsule_id"), f"capsule[{idx}].capsule_id")] = capsule
    return capsules


def collect_workspace_templates(
    template_seed: dict[str, Any],
) -> dict[str, dict[str, Any]]:
    templates: dict[str, dict[str, Any]] = {}
    for idx, raw_template in enumerate(
        ensure_list(template_seed.get("workspace_templates"), "template_seed.workspace_templates")
    ):
        template = ensure_dict(raw_template, f"template_seed.workspace_templates[{idx}]")
        templates[ensure_str(template.get("template_id"), f"template[{idx}].template_id")] = template
    return templates


def collect_descriptors(
    seed: dict[str, Any],
    capsules: dict[str, dict[str, Any]],
    templates: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> dict[str, dict[str, Any]]:
    descriptors: dict[str, dict[str, Any]] = {}
    for idx, raw_descriptor in enumerate(
        ensure_list(seed.get("prebuild_descriptors"), "seed.prebuild_descriptors")
    ):
        descriptor = ensure_dict(raw_descriptor, f"seed.prebuild_descriptors[{idx}]")
        descriptor_id = ensure_str(descriptor.get("descriptor_id"), f"descriptor[{idx}].descriptor_id")
        if descriptor_id in descriptors:
            findings.append(
                Finding(
                    severity="error",
                    check_id="descriptor.id.duplicate",
                    message=f"duplicate descriptor_id: {descriptor_id}",
                    remediation="Give each warm-start descriptor one stable id.",
                    ref=descriptor_id,
                )
            )
        descriptors[descriptor_id] = descriptor

        compatibility = ensure_dict(
            descriptor.get("compatibility_fingerprint"),
            f"{descriptor_id}.compatibility_fingerprint",
        )
        capsule_ref = ensure_str(compatibility.get("capsule_ref"), f"{descriptor_id}.capsule_ref")
        if capsule_ref not in capsules:
            findings.append(
                Finding(
                    severity="error",
                    check_id="descriptor.capsule_ref.missing",
                    message=f"{descriptor_id} references missing capsule {capsule_ref}",
                    remediation="Reuse a capsule id from the workspace-template seed.",
                    ref=descriptor_id,
                )
            )
        else:
            capsule = capsules[capsule_ref]
            if capsule.get("capsule_hash") != compatibility.get("capsule_hash"):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="descriptor.capsule_hash.mismatch",
                        message=f"{descriptor_id} capsule_hash does not match {capsule_ref}",
                        remediation="Use the same capsule hash token as the environment-capsule seed.",
                        ref=descriptor_id,
                    )
                )

        for template_ref in ensure_list(
            descriptor.get("workspace_template_refs"),
            f"{descriptor_id}.workspace_template_refs",
        ):
            template_ref = ensure_str(template_ref, f"{descriptor_id}.workspace_template_refs[]")
            if template_ref not in templates:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="descriptor.workspace_template_ref.missing",
                        message=f"{descriptor_id} references missing template {template_ref}",
                        remediation="Reuse a template id from the workspace-template seed.",
                        ref=descriptor_id,
                    )
                )
                continue
            template_capsule = templates[template_ref].get("environment_capsule_ref")
            if template_capsule != capsule_ref:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="descriptor.template_capsule.mismatch",
                        message=f"{descriptor_id} capsule ref does not match template {template_ref}",
                        remediation="Keep descriptor and template capsule refs aligned.",
                        ref=descriptor_id,
                    )
                )

        validate_descriptor_states(descriptor_id, descriptor, findings)
    return descriptors


def validate_descriptor_states(
    descriptor_id: str, descriptor: dict[str, Any], findings: list[Finding]
) -> None:
    source = ensure_dict(descriptor.get("source_identity"), f"{descriptor_id}.source_identity")
    freshness = ensure_dict(descriptor.get("freshness"), f"{descriptor_id}.freshness")
    target = ensure_dict(descriptor.get("target"), f"{descriptor_id}.target")
    warm = ensure_dict(
        descriptor.get("warm_start_descriptor"), f"{descriptor_id}.warm_start_descriptor"
    )
    safety = ensure_dict(descriptor.get("safety"), f"{descriptor_id}.safety")
    drift_markers = ensure_list(descriptor.get("drift_markers"), f"{descriptor_id}.drift_markers")
    launch_refs = ensure_list(descriptor.get("launch_bundle_refs"), f"{descriptor_id}.launch_bundle_refs")
    project_refs = ensure_list(
        descriptor.get("project_entry_review_refs"),
        f"{descriptor_id}.project_entry_review_refs",
    )
    review_surfaces = ensure_list(descriptor.get("review_surfaces"), f"{descriptor_id}.review_surfaces")

    for label, obj, field_name in [
        ("source", source, "source_class"),
        ("freshness", freshness, "freshness_state"),
        ("target", target, "target_class"),
        ("warm_start", warm, "warm_start_state"),
    ]:
        ensure_str(obj.get(field_name), f"{descriptor_id}.{label}.{field_name}")

    if safety.get("raw_secret_values_included") is not False:
        findings.append(
            Finding(
                severity="error",
                check_id="descriptor.safety.raw_secrets",
                message=f"{descriptor_id} includes raw secret material",
                remediation="Only metadata, aliases, and evidence refs are allowed in the seed.",
                ref=descriptor_id,
            )
        )
    if safety.get("raw_command_lines_included") is not False:
        findings.append(
            Finding(
                severity="error",
                check_id="descriptor.safety.raw_commands",
                message=f"{descriptor_id} includes raw command lines",
                remediation="Use command refs or fallback action ids instead of raw commands.",
                ref=descriptor_id,
            )
        )
    if warm.get("materializer_claim") != "metadata_only_no_materializer_claim":
        findings.append(
            Finding(
                severity="error",
                check_id="descriptor.materializer.overclaim",
                message=f"{descriptor_id} claims a materializer beyond alpha metadata",
                remediation="Keep this seed metadata-only until a runtime materializer consumes it.",
                ref=descriptor_id,
            )
        )
    if "start_center" not in review_surfaces:
        findings.append(
            Finding(
                severity="error",
                check_id="descriptor.review_surface.start_center_missing",
                message=f"{descriptor_id} is not exposed through the first Start Center consumer",
                remediation="Add start_center to review_surfaces or wire another protected consumer.",
                ref=descriptor_id,
            )
        )
    if not launch_refs and not project_refs:
        findings.append(
            Finding(
                severity="error",
                check_id="descriptor.review_reachability.missing",
                message=f"{descriptor_id} is not reachable from launch-bundle or project-entry review",
                remediation="Add launch_bundle_refs or project_entry_review_refs.",
                ref=descriptor_id,
            )
        )
    if freshness.get("freshness_state") == "stale":
        if warm.get("invalidation_reason") is None or not drift_markers:
            findings.append(
                Finding(
                    severity="error",
                    check_id="descriptor.stale_without_drift",
                    message=f"{descriptor_id} is stale but lacks invalidation and drift markers",
                    remediation="Stale warm-start descriptors must explain why reuse is narrowed.",
                    ref=descriptor_id,
                )
            )
        if warm.get("reuse_state") not in {"rejected_drift", "rejected_policy", "rejected_trust"}:
            findings.append(
                Finding(
                    severity="error",
                    check_id="descriptor.stale_not_rejected",
                    message=f"{descriptor_id} is stale but does not reject reuse",
                    remediation="Use a rejected reuse state for stale prebuild metadata.",
                    ref=descriptor_id,
                )
            )
    if warm.get("warm_start_state") == "live_resume_candidate":
        if warm.get("resume_capability") == "not_resume_capable":
            findings.append(
                Finding(
                    severity="error",
                    check_id="descriptor.resume_capability.missing",
                    message=f"{descriptor_id} claims live resume without resume capability",
                    remediation="Use a resume-capable state or avoid live_resume_candidate.",
                    ref=descriptor_id,
                )
            )
        if safety.get("secret_revalidation_required") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id="descriptor.resume_revalidation.missing",
                    message=f"{descriptor_id} resume candidate lacks credential revalidation",
                    remediation="Managed or remote resume candidates must revalidate secrets/routes.",
                    ref=descriptor_id,
                )
            )
        if target.get("target_class") not in {
            "managed_workspace",
            "remote_workspace_vm",
            "ssh_remote",
            "prebuild_runtime",
            "devcontainer",
        }:
            findings.append(
                Finding(
                    severity="error",
                    check_id="descriptor.resume_target.invalid",
                    message=f"{descriptor_id} resume candidate targets {target.get('target_class')}",
                    remediation="Use a remote, managed, prebuild, or container target class for resume metadata.",
                    ref=descriptor_id,
                )
            )


def validate_cross_refs(
    repo_root: Path,
    seed: dict[str, Any],
    descriptors: dict[str, dict[str, Any]],
    bundle_paths: list[Path],
    findings: list[Finding],
) -> None:
    for key, ref in ensure_dict(seed.get("contract_refs"), "seed.contract_refs").items():
        add_missing_ref_finding(findings, f"seed.contract_refs.{key}", repo_root, ensure_str(ref, key))

    expected_bundles: dict[str, dict[str, Any]] = {}
    for bundle_path in bundle_paths:
        bundle = ensure_dict(load_yaml(bundle_path), str(bundle_path))
        expected_bundles[ensure_str(bundle.get("bundle_id"), f"{bundle_path}.bundle_id")] = bundle

    for descriptor_id, descriptor in descriptors.items():
        freshness = ensure_dict(descriptor.get("freshness"), f"{descriptor_id}.freshness")
        add_missing_ref_finding(
            findings,
            f"{descriptor_id}.freshness.evidence_ref",
            repo_root,
            ensure_str(freshness.get("evidence_ref"), f"{descriptor_id}.freshness.evidence_ref"),
        )
        for bundle_ref in descriptor.get("launch_bundle_refs", []):
            if bundle_ref not in expected_bundles:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="descriptor.launch_bundle_ref.unknown",
                        message=f"{descriptor_id} references unknown launch bundle {bundle_ref}",
                        remediation="Point launch_bundle_refs at a checked-in alpha launch bundle.",
                        ref=descriptor_id,
                    )
                )
                continue
            bundle_rows = expected_bundles[bundle_ref].get("prebuild_descriptor_refs", [])
            if not any(row.get("descriptor_ref") == descriptor_id for row in bundle_rows):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="descriptor.launch_bundle_backref.missing",
                        message=f"{bundle_ref} does not reference {descriptor_id}",
                        remediation="Keep launch bundle and warm-start descriptor refs bidirectional.",
                        ref=bundle_ref,
                    )
                )

    for bundle_id, bundle in expected_bundles.items():
        rows = ensure_list(bundle.get("prebuild_descriptor_refs"), f"{bundle_id}.prebuild_descriptor_refs")
        if not rows:
            findings.append(
                Finding(
                    severity="error",
                    check_id="bundle.prebuild_descriptor_refs.empty",
                    message=f"{bundle_id} must reference at least one warm-start descriptor",
                    remediation="Add prebuild_descriptor_refs to the launch bundle manifest.",
                    ref=bundle_id,
                )
            )
        for idx, raw_row in enumerate(rows):
            row = ensure_dict(raw_row, f"{bundle_id}.prebuild_descriptor_refs[{idx}]")
            descriptor_ref = ensure_str(
                row.get("descriptor_ref"), f"{bundle_id}.prebuild_descriptor_refs[{idx}].descriptor_ref"
            )
            manifest_ref = ensure_str(
                row.get("manifest_ref"), f"{bundle_id}.prebuild_descriptor_refs[{idx}].manifest_ref"
            )
            add_missing_ref_finding(
                findings,
                f"{bundle_id}.prebuild_descriptor_refs[{idx}].manifest_ref",
                repo_root,
                manifest_ref,
            )
            if descriptor_ref not in descriptors:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="bundle.prebuild_descriptor_ref.unknown",
                        message=f"{bundle_id} references unknown descriptor {descriptor_ref}",
                        remediation="Use a descriptor_id from artifacts/templates/warm_start_descriptor_seed.yaml.",
                        ref=bundle_id,
                    )
                )
                continue
            descriptor = descriptors[descriptor_ref]
            capsule_ref = descriptor["compatibility_fingerprint"]["capsule_ref"]
            if row.get("environment_capsule_ref") != capsule_ref:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="bundle.prebuild_descriptor_capsule.mismatch",
                        message=f"{bundle_id} capsule ref does not match {descriptor_ref}",
                        remediation="Use the same environment_capsule_ref on bundle and descriptor seed.",
                        ref=bundle_id,
                    )
                )


def validate_fixture(
    repo_root: Path,
    fixture_path: Path,
    descriptors: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> None:
    fixture = ensure_dict(load_json(fixture_path), "fixture")
    add_missing_ref_finding(
        findings,
        "fixture.seed_manifest_ref",
        repo_root,
        ensure_str(fixture.get("seed_manifest_ref"), "fixture.seed_manifest_ref"),
    )
    expected_descriptors = set(
        ensure_list(fixture.get("expected_descriptor_ids"), "fixture.expected_descriptor_ids")
    )
    missing_descriptors = expected_descriptors - set(descriptors)
    if missing_descriptors:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture.expected_descriptors.missing",
                message="fixture expected descriptor ids are missing from the seed",
                remediation="Update the seed artifact or fixture expectations together.",
                details={"missing": sorted(missing_descriptors)},
            )
        )

    for state in ensure_list(fixture.get("required_warm_start_states"), "fixture.required_warm_start_states"):
        if not any(
            descriptor["warm_start_descriptor"]["warm_start_state"] == state
            for descriptor in descriptors.values()
        ):
            findings.append(
                Finding(
                    severity="error",
                    check_id="fixture.required_warm_start_state.missing",
                    message=f"required warm-start state is missing: {state}",
                    remediation="Seed a descriptor that exercises this acceptance state.",
                    ref=state,
                )
            )

    for target_class in ensure_list(fixture.get("required_target_classes"), "fixture.required_target_classes"):
        if not any(descriptor["target"]["target_class"] == target_class for descriptor in descriptors.values()):
            findings.append(
                Finding(
                    severity="error",
                    check_id="fixture.required_target_class.missing",
                    message=f"required target class is missing: {target_class}",
                    remediation="Seed a descriptor that exercises this target boundary.",
                    ref=target_class,
                )
            )


def validate_consumer_and_docs(
    start_center_path: Path,
    doc_path: Path,
    findings: list[Finding],
) -> None:
    start_center = start_center_path.read_text(encoding="utf-8")
    for token in [
        "WARM_START_DESCRIPTOR_SEED_MANIFEST",
        "build_warm_start_descriptor_seed_rows",
        "render_warm_start_descriptor_seed_plaintext",
    ]:
        if token not in start_center:
            findings.append(
                Finding(
                    severity="error",
                    check_id="start_center.consumer.missing_token",
                    message=f"Start Center consumer does not contain {token}",
                    remediation="Keep the warm-start seed wired into the first shell consumer.",
                    ref=str(start_center_path),
                )
            )
    doc = doc_path.read_text(encoding="utf-8")
    for ref in [
        DEFAULT_SCHEMA_REL,
        DEFAULT_SEED_REL,
        DEFAULT_START_CENTER_REL,
        "ci/check_prebuild_descriptor_alpha.py",
    ]:
        if ref not in doc:
            findings.append(
                Finding(
                    severity="error",
                    check_id="docs.runtime_alpha.missing_ref",
                    message=f"runtime alpha doc does not mention {ref}",
                    remediation="Document the schema, seed artifact, validator, and first consumer together.",
                    ref=str(doc_path),
                )
            )


def render_warm_start_gallery(seed: dict[str, Any]) -> str:
    lines = [
        "Warm-start descriptor seed gallery",
        "descriptor_id | source | freshness | target | warm_state | resume | claim",
    ]
    for descriptor in seed["prebuild_descriptors"]:
        source = descriptor["source_identity"]
        freshness = descriptor["freshness"]
        target = descriptor["target"]
        warm = descriptor["warm_start_descriptor"]
        lines.append(
            " | ".join(
                [
                    descriptor["descriptor_id"],
                    f"{source['source_class']}:{source['source_ref']}",
                    f"{freshness['freshness_state']}/{freshness['age_class']}",
                    f"{target['target_class']}/{target['boundary_class']}",
                    f"{warm['warm_start_state']}/{warm['reuse_state']}",
                    warm["resume_capability"],
                    f"{warm['materializer_claim']}:{warm['live_runtime_claim']}",
                ]
            )
        )
    return "\n".join(lines) + "\n"


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    schema_path = repo_root / args.schema
    seed_path = repo_root / args.seed
    template_seed_path = repo_root / args.template_seed
    fixture_path = repo_root / args.fixture
    start_center_path = repo_root / args.start_center
    doc_path = repo_root / args.doc

    schema = ensure_dict(load_json(schema_path), "schema")
    seed = ensure_dict(load_yaml(seed_path), "seed")
    template_seed = ensure_dict(load_yaml(template_seed_path), "template_seed")

    findings = schema_validate(schema, seed)
    capsules = collect_environment_capsules(template_seed)
    templates = collect_workspace_templates(template_seed)
    descriptors = collect_descriptors(seed, capsules, templates, findings)
    validate_cross_refs(
        repo_root,
        seed,
        descriptors,
        [repo_root / args.tsjs_bundle, repo_root / args.python_bundle],
        findings,
    )
    validate_fixture(repo_root, fixture_path, descriptors, findings)
    validate_consumer_and_docs(start_center_path, doc_path, findings)

    report = {
        "status": "pass" if not findings else "fail",
        "schema_ref": args.schema,
        "seed_ref": args.seed,
        "descriptor_count": len(descriptors),
        "findings": [finding.as_report() for finding in findings],
    }
    if args.report:
        Path(args.report).write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    if args.render_warm_start_gallery:
        print(render_warm_start_gallery(seed), end="")
    if findings:
        print(json.dumps(report, indent=2), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
