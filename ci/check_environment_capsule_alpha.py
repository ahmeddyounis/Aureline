#!/usr/bin/env python3
"""Validate the environment-capsule alpha schema and workspace-template seed."""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

import yaml
from jsonschema import Draft202012Validator, FormatChecker


DEFAULT_SCHEMA_REL = "schemas/runtime/environment_capsule_alpha.schema.json"
DEFAULT_SEED_REL = "artifacts/templates/workspace_template_seed.yaml"
DEFAULT_TSJS_BUNDLE_REL = "artifacts/bundles/tsjs_launch_bundle_alpha.yaml"
DEFAULT_PYTHON_BUNDLE_REL = "artifacts/bundles/python_launch_bundle_alpha.yaml"
DEFAULT_FIXTURE_REL = "fixtures/runtime/environment_capsule_alpha/manifest.json"
DEFAULT_START_CENTER_REL = "crates/aureline-shell/src/start_center/mod.rs"
DEFAULT_DOC_REL = "docs/runtime/environment_capsule_alpha.md"


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
    parser.add_argument("--tsjs-bundle", default=DEFAULT_TSJS_BUNDLE_REL)
    parser.add_argument("--python-bundle", default=DEFAULT_PYTHON_BUNDLE_REL)
    parser.add_argument("--fixture", default=DEFAULT_FIXTURE_REL)
    parser.add_argument("--start-center", default=DEFAULT_START_CENTER_REL)
    parser.add_argument("--doc", default=DEFAULT_DOC_REL)
    parser.add_argument("--report", default=None)
    parser.add_argument(
        "--render-template-gallery",
        action="store_true",
        help="Print the deterministic template projection after validation.",
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
                check_id="environment_capsule_alpha.schema.validation_failed",
                message=f"{path}: {error.message}",
                remediation="Update the seed artifact or schema so the alpha contract validates.",
                details={"schema_path": list(error.schema_path)},
            )
        )
    return findings


def collect_capsules(seed: dict[str, Any], findings: list[Finding]) -> dict[str, dict[str, Any]]:
    capsules: dict[str, dict[str, Any]] = {}
    for idx, raw_capsule in enumerate(
        ensure_list(seed.get("environment_capsules"), "seed.environment_capsules")
    ):
        capsule = ensure_dict(raw_capsule, f"seed.environment_capsules[{idx}]")
        capsule_id = ensure_str(capsule.get("capsule_id"), f"capsule[{idx}].capsule_id")
        if capsule_id in capsules:
            findings.append(
                Finding(
                    severity="error",
                    check_id="capsule.id.duplicate",
                    message=f"duplicate capsule_id: {capsule_id}",
                    remediation="Give each alpha capsule one stable id.",
                    ref=capsule_id,
                )
            )
        capsules[capsule_id] = capsule
        fingerprint = ensure_dict(
            capsule.get("compatibility_fingerprint"),
            f"{capsule_id}.compatibility_fingerprint",
        )
        if capsule.get("capsule_hash") != fingerprint.get("capsule_hash"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="capsule.hash.fingerprint_mismatch",
                    message=f"{capsule_id} capsule_hash must match compatibility_fingerprint.capsule_hash",
                    remediation="Use one capsule hash token across the capsule and its compatibility fingerprint.",
                    ref=capsule_id,
                )
            )
        trust = ensure_dict(capsule.get("trust_and_policy"), f"{capsule_id}.trust_and_policy")
        if trust.get("secret_values_projected_by_default") is not False:
            findings.append(
                Finding(
                    severity="error",
                    check_id="capsule.secrets.default_projection_not_false",
                    message=f"{capsule_id} must not project secret values by default",
                    remediation="Set secret_values_projected_by_default to false.",
                    ref=capsule_id,
                )
            )
        for var_idx, raw_variable in enumerate(
            ensure_list(capsule.get("environment_variables"), f"{capsule_id}.environment_variables")
        ):
            variable = ensure_dict(raw_variable, f"{capsule_id}.environment_variables[{var_idx}]")
            variable_name = ensure_str(
                variable.get("variable_name"),
                f"{capsule_id}.environment_variables[{var_idx}].variable_name",
            )
            if variable.get("raw_value_included") is not False:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="capsule.variable.raw_value_included",
                        message=f"{capsule_id}.{variable_name} includes a raw environment value",
                        remediation="Use value_digest or credential_alias_ref; never include raw env values in the alpha seed.",
                        ref=capsule_id,
                    )
                )
            if variable.get("value_source") == "secret_alias":
                if not variable.get("credential_alias_ref") or not variable.get("secret_class"):
                    findings.append(
                        Finding(
                            severity="error",
                            check_id="capsule.variable.secret_alias_missing_marker",
                            message=f"{capsule_id}.{variable_name} must carry credential_alias_ref and secret_class",
                            remediation="Name the credential alias and secret class without copying secret material.",
                            ref=capsule_id,
                        )
                    )
    return capsules


def collect_templates(seed: dict[str, Any], findings: list[Finding]) -> dict[str, dict[str, Any]]:
    templates: dict[str, dict[str, Any]] = {}
    for idx, raw_template in enumerate(
        ensure_list(seed.get("workspace_templates"), "seed.workspace_templates")
    ):
        template = ensure_dict(raw_template, f"seed.workspace_templates[{idx}]")
        template_id = ensure_str(template.get("template_id"), f"template[{idx}].template_id")
        if template_id in templates:
            findings.append(
                Finding(
                    severity="error",
                    check_id="template.id.duplicate",
                    message=f"duplicate template_id: {template_id}",
                    remediation="Give each workspace template one stable id.",
                    ref=template_id,
                )
            )
        templates[template_id] = template
        if not ensure_list(template.get("launch_bundle_refs"), f"{template_id}.launch_bundle_refs"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="template.launch_bundle_refs.empty",
                    message=f"{template_id} must be reachable from at least one launch bundle",
                    remediation="Add a launch_bundle_ref to the template seed.",
                    ref=template_id,
                )
            )
        if not ensure_list(template.get("bypass_path_ids"), f"{template_id}.bypass_path_ids"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="template.bypass_paths.empty",
                    message=f"{template_id} must keep a non-starter path available",
                    remediation="Add at least one bypass path id.",
                    ref=template_id,
                )
            )
    return templates


def validate_cross_refs(
    repo_root: Path,
    seed: dict[str, Any],
    capsules: dict[str, dict[str, Any]],
    templates: dict[str, dict[str, Any]],
    bundle_paths: list[Path],
    findings: list[Finding],
) -> None:
    for key, ref in ensure_dict(seed.get("contract_refs"), "seed.contract_refs").items():
        add_missing_ref_finding(findings, f"seed.contract_refs.{key}", repo_root, ensure_str(ref, key))

    expected_bundles: dict[str, dict[str, Any]] = {}
    for bundle_path in bundle_paths:
        bundle = ensure_dict(load_yaml(bundle_path), str(bundle_path))
        bundle_id = ensure_str(bundle.get("bundle_id"), f"{bundle_path}.bundle_id")
        expected_bundles[bundle_id] = bundle

    binding_pairs = {
        (
            ensure_str(binding.get("launch_bundle_ref"), "launch_bundle_binding.launch_bundle_ref"),
            ensure_str(binding.get("workspace_template_ref"), "launch_bundle_binding.workspace_template_ref"),
            ensure_str(binding.get("environment_capsule_ref"), "launch_bundle_binding.environment_capsule_ref"),
        )
        for binding in ensure_list(seed.get("launch_bundle_bindings"), "seed.launch_bundle_bindings")
        if isinstance(binding, dict)
    }

    for template_id, template in templates.items():
        capsule_ref = ensure_str(template.get("environment_capsule_ref"), f"{template_id}.environment_capsule_ref")
        if capsule_ref not in capsules:
            findings.append(
                Finding(
                    severity="error",
                    check_id="template.capsule_ref.missing",
                    message=f"{template_id} references missing capsule {capsule_ref}",
                    remediation="Add the capsule to environment_capsules or fix the template reference.",
                    ref=template_id,
                )
            )
        for bundle_ref in ensure_list(template.get("launch_bundle_refs"), f"{template_id}.launch_bundle_refs"):
            bundle_ref = ensure_str(bundle_ref, f"{template_id}.launch_bundle_refs[]")
            if bundle_ref not in expected_bundles:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="template.launch_bundle_ref.unknown",
                        message=f"{template_id} references unknown launch bundle {bundle_ref}",
                        remediation="Point template launch_bundle_refs at a checked-in alpha launch bundle.",
                        ref=template_id,
                    )
                )
            if (bundle_ref, template_id, capsule_ref) not in binding_pairs:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="template.launch_binding.missing",
                        message=f"{template_id} lacks a matching launch_bundle_bindings row for {bundle_ref}",
                        remediation="Add a launch_bundle_bindings row tying bundle, template, and capsule.",
                        ref=template_id,
                    )
                )

    for bundle_id, bundle in expected_bundles.items():
        refs = ensure_list(bundle.get("workspace_template_refs"), f"{bundle_id}.workspace_template_refs")
        if not refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="bundle.workspace_template_refs.empty",
                    message=f"{bundle_id} must reference at least one workspace template seed",
                    remediation="Add workspace_template_refs to the launch bundle manifest.",
                    ref=bundle_id,
                )
            )
        for idx, raw_ref in enumerate(refs):
            ref_row = ensure_dict(raw_ref, f"{bundle_id}.workspace_template_refs[{idx}]")
            template_ref = ensure_str(ref_row.get("template_ref"), f"{bundle_id}.workspace_template_refs[{idx}].template_ref")
            capsule_ref = ensure_str(ref_row.get("environment_capsule_ref"), f"{bundle_id}.workspace_template_refs[{idx}].environment_capsule_ref")
            manifest_ref = ensure_str(ref_row.get("manifest_ref"), f"{bundle_id}.workspace_template_refs[{idx}].manifest_ref")
            add_missing_ref_finding(findings, f"{bundle_id}.workspace_template_refs[{idx}].manifest_ref", repo_root, manifest_ref)
            if template_ref not in templates:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="bundle.workspace_template_ref.unknown",
                        message=f"{bundle_id} references unknown workspace template {template_ref}",
                        remediation="Use a template_id from artifacts/templates/workspace_template_seed.yaml.",
                        ref=bundle_id,
                    )
                )
                continue
            template = templates[template_ref]
            if bundle_id not in template.get("launch_bundle_refs", []):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="bundle.template_backref.missing",
                        message=f"{template_ref} does not list {bundle_id} in launch_bundle_refs",
                        remediation="Keep launch bundle and workspace-template seed references bidirectional.",
                        ref=bundle_id,
                    )
                )
            if template.get("environment_capsule_ref") != capsule_ref:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="bundle.template_capsule_mismatch",
                        message=f"{bundle_id} capsule ref does not match {template_ref}",
                        remediation="Use the same environment_capsule_ref on bundle and template seed.",
                        ref=bundle_id,
                    )
                )


def validate_fixture(
    repo_root: Path,
    fixture_path: Path,
    capsules: dict[str, dict[str, Any]],
    templates: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> None:
    fixture = ensure_dict(load_json(fixture_path), "fixture")
    add_missing_ref_finding(
        findings,
        "fixture.seed_manifest_ref",
        repo_root,
        ensure_str(fixture.get("seed_manifest_ref"), "fixture.seed_manifest_ref"),
    )
    expected_capsules = set(
        ensure_list(fixture.get("expected_capsule_ids"), "fixture.expected_capsule_ids")
    )
    expected_templates = set(
        ensure_list(fixture.get("expected_template_ids"), "fixture.expected_template_ids")
    )
    missing_capsules = expected_capsules - set(capsules)
    missing_templates = expected_templates - set(templates)
    if missing_capsules:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture.expected_capsules.missing",
                message="fixture expected capsule ids are missing from the seed",
                remediation="Update the seed artifact or fixture expectations together.",
                details={"missing": sorted(missing_capsules)},
            )
        )
    if missing_templates:
        findings.append(
            Finding(
                severity="error",
                check_id="fixture.expected_templates.missing",
                message="fixture expected template ids are missing from the seed",
                remediation="Update the seed artifact or fixture expectations together.",
                details={"missing": sorted(missing_templates)},
            )
        )


def validate_consumer_and_docs(
    start_center_path: Path,
    doc_path: Path,
    findings: list[Finding],
) -> None:
    start_center = start_center_path.read_text(encoding="utf-8")
    for token in [
        "WORKSPACE_TEMPLATE_SEED_MANIFEST",
        "build_workspace_template_seed_rows",
        "render_workspace_template_seed_plaintext",
    ]:
        if token not in start_center:
            findings.append(
                Finding(
                    severity="error",
                    check_id="start_center.consumer.missing_token",
                    message=f"Start Center consumer does not contain {token}",
                    remediation="Keep the workspace-template seed wired into the first shell consumer.",
                    ref=str(start_center_path),
                )
            )
    doc = doc_path.read_text(encoding="utf-8")
    for ref in [
        DEFAULT_SCHEMA_REL,
        DEFAULT_SEED_REL,
        DEFAULT_START_CENTER_REL,
        "ci/check_environment_capsule_alpha.py",
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


def render_template_gallery(seed: dict[str, Any], capsules: dict[str, dict[str, Any]]) -> str:
    lines = [
        "Workspace template seed gallery",
        "template_id | capsule | launch_bundles | target | toolchains | variables | prebuild",
    ]
    for template in seed["workspace_templates"]:
        capsule = capsules[template["environment_capsule_ref"]]
        variables = capsule["environment_variables"]
        secret_aliases = sum(1 for item in variables if item["value_source"] == "secret_alias")
        toolchains = ",".join(
            f"{item['toolchain_id']}:{item['toolchain_class']}" for item in capsule["toolchains"]
        )
        lines.append(
            " | ".join(
                [
                    template["template_id"],
                    template["environment_capsule_ref"],
                    ",".join(template["launch_bundle_refs"]),
                    f"{capsule['target_plan']['target_class']}/{capsule['target_plan']['boundary_class']}",
                    toolchains,
                    f"vars={len(variables)} secret_aliases={secret_aliases} raw_values=0",
                    f"{template['prebuild_reuse_policy']['reuse_state']}/{template['prebuild_reuse_policy']['stale_behavior']}",
                ]
            )
        )
    return "\n".join(lines) + "\n"


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    schema_path = repo_root / args.schema
    seed_path = repo_root / args.seed
    fixture_path = repo_root / args.fixture
    start_center_path = repo_root / args.start_center
    doc_path = repo_root / args.doc

    schema = ensure_dict(load_json(schema_path), "schema")
    seed = ensure_dict(load_yaml(seed_path), "seed")

    findings = schema_validate(schema, seed)
    capsules = collect_capsules(seed, findings)
    templates = collect_templates(seed, findings)
    validate_cross_refs(
        repo_root,
        seed,
        capsules,
        templates,
        [repo_root / args.tsjs_bundle, repo_root / args.python_bundle],
        findings,
    )
    validate_fixture(repo_root, fixture_path, capsules, templates, findings)
    validate_consumer_and_docs(start_center_path, doc_path, findings)

    report = {
        "status": "pass" if not findings else "fail",
        "schema_ref": args.schema,
        "seed_ref": args.seed,
        "capsule_count": len(capsules),
        "template_count": len(templates),
        "findings": [finding.as_report() for finding in findings],
    }
    if args.report:
        Path(args.report).write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    if args.render_template_gallery:
        print(render_template_gallery(seed, capsules), end="")
    if findings:
        print(json.dumps(report, indent=2), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
