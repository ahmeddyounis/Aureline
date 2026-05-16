#!/usr/bin/env python3
"""Validate the alpha workspace-template bundle schema and fixtures."""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator, FormatChecker


DEFAULT_SCHEMA_REL = "schemas/workspace/template_bundle.schema.json"
DEFAULT_FIXTURE_DIR_REL = "fixtures/workspace/m3/template_bundle"
DEFAULT_DOC_REL = "docs/workspace/m3/template_bundle_alpha.md"
DEFAULT_CONSUMER_REL = (
    "crates/aureline-shell/src/start_center/template_bundles/mod.rs"
)
DEFAULT_CRATE_MODULE_REL = "crates/aureline-workspace/src/templates/mod.rs"

REQUIRED_SOURCE_CLASSES = {"first_party", "community", "team_managed"}
REQUIRED_RUNTIME_SCOPES = {"local_only", "managed_cloud_required"}
REQUIRED_BYPASS_CONTINUITY = "equal_weight_with_apply"
REQUIRED_CONSUMER_SURFACES = {
    "start_center",
    "cli_headless_entry",
    "docs_workspace",
    "support_export",
}


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
    parser.add_argument("--fixture-dir", default=DEFAULT_FIXTURE_DIR_REL)
    parser.add_argument("--doc", default=DEFAULT_DOC_REL)
    parser.add_argument("--consumer", default=DEFAULT_CONSUMER_REL)
    parser.add_argument("--crate-module", default=DEFAULT_CRATE_MODULE_REL)
    parser.add_argument("--report", default=None)
    parser.add_argument(
        "--render-bundle-gallery",
        action="store_true",
        help="Print the deterministic bundle projection after validation.",
    )
    return parser.parse_args()


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise SystemExit(f"missing JSON file: {path}") from exc
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def schema_validate(
    schema: dict[str, Any], label: str, payload: dict[str, Any]
) -> list[Finding]:
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema, format_checker=FormatChecker())
    findings: list[Finding] = []
    for error in sorted(validator.iter_errors(payload), key=lambda item: list(item.path)):
        path = ".".join(str(part) for part in error.path) or "<root>"
        findings.append(
            Finding(
                severity="error",
                check_id="template_bundle_alpha.schema.validation_failed",
                message=f"{label}: {path}: {error.message}",
                remediation="Fix the bundle to validate against schemas/workspace/template_bundle.schema.json.",
                ref=label,
                details={"schema_path": list(error.schema_path)},
            )
        )
    return findings


def cross_check_bundle(label: str, bundle: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    bypass = bundle.get("bypass_review", {})
    if bypass.get("bypass_continuity_class") != REQUIRED_BYPASS_CONTINUITY:
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.bypass.continuity_mismatch",
                message=f"{label} bypass_continuity_class is not {REQUIRED_BYPASS_CONTINUITY}",
                remediation="Keep bypass continuity at equal weight with apply.",
                ref=label,
            )
        )
    routes = bypass.get("open_without_starter_route_ids", [])
    if not routes:
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.bypass.missing_route",
                message=f"{label} carries no open-without-starter routes",
                remediation="Advertise at least one open-without-starter bypass route id.",
                ref=label,
            )
        )

    consumers = set(bundle.get("consumer_surfaces", []))
    if "start_center" not in consumers:
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.consumer.start_center_missing",
                message=f"{label} consumer_surfaces does not include start_center",
                remediation="Wire the bundle into Start Center so the first product surface stays bound.",
                ref=label,
            )
        )

    invariants = bundle.get("review_invariants", {})
    for key in ("reviewed_before_use", "inspectable_before_execution", "no_writes_before_review"):
        if invariants.get(key) is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"bundle.review.{key}_missing",
                    message=f"{label} review_invariants.{key} must be true",
                    remediation="The bundle is a pre-execution review record; keep every review invariant true.",
                    ref=label,
                )
            )

    support_export = bundle.get("support_export", {})
    for key in (
        "raw_secret_export_allowed",
        "raw_command_export_allowed",
        "raw_url_export_allowed",
    ):
        if support_export.get(key) is not False:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"bundle.support_export.{key}_widened",
                    message=f"{label} support_export.{key} must be false",
                    remediation="Keep raw secret, command, and URL export closed on the alpha bundle.",
                    ref=label,
                )
            )

    runtime = bundle.get("target_runtime_review", {})
    runtime_scope = runtime.get("runtime_scope_class")
    host = runtime.get("host_boundary_class")
    side_effects = bundle.get("side_effect_review", {})
    if runtime_scope == "managed_cloud_required":
        if host != "host_managed_workspace_required":
            findings.append(
                Finding(
                    severity="error",
                    check_id="bundle.runtime.managed_cloud_host_mismatch",
                    message=f"{label} managed_cloud_required runtime must use host_managed_workspace_required",
                    remediation="Pair managed_cloud_required with host_managed_workspace_required.",
                    ref=label,
                )
            )
        if side_effects.get("required_remote_provisioning_class") != "managed_workspace_required":
            findings.append(
                Finding(
                    severity="error",
                    check_id="bundle.runtime.managed_cloud_provisioning_missing",
                    message=f"{label} managed_cloud_required runtime must require managed_workspace_required provisioning",
                    remediation="Provision the managed workspace when the runtime scope demands it.",
                    ref=label,
                )
            )
        if side_effects.get("required_managed_service_class") == "no_managed_service_required":
            findings.append(
                Finding(
                    severity="error",
                    check_id="bundle.runtime.managed_cloud_managed_service_missing",
                    message=f"{label} managed_cloud_required runtime must declare a managed-service class",
                    remediation="Bind a managed-service class when the runtime scope demands it.",
                    ref=label,
                )
            )
        if side_effects.get("required_network_egress_class") == "no_network_egress_required":
            findings.append(
                Finding(
                    severity="error",
                    check_id="bundle.runtime.managed_cloud_egress_missing",
                    message=f"{label} managed_cloud_required runtime must declare network egress",
                    remediation="Managed-cloud runtimes have egress; declare it instead of hiding it.",
                    ref=label,
                )
            )
    elif runtime_scope == "local_only":
        if side_effects.get("required_remote_provisioning_class") not in {
            "no_remote_provisioning_required",
            "remote_provisioning_unknown_requires_review",
        }:
            findings.append(
                Finding(
                    severity="error",
                    check_id="bundle.runtime.local_remote_provisioning_widened",
                    message=f"{label} local_only runtime cannot require remote provisioning",
                    remediation="Keep local_only runtime bundles free of remote provisioning.",
                    ref=label,
                )
            )
        if side_effects.get("required_managed_service_class") not in {
            "no_managed_service_required",
            "managed_service_class_unknown_requires_review",
        }:
            findings.append(
                Finding(
                    severity="error",
                    check_id="bundle.runtime.local_managed_service_widened",
                    message=f"{label} local_only runtime cannot require a managed service",
                    remediation="Keep local_only runtime bundles free of managed-service requirements.",
                    ref=label,
                )
            )

    trust_notes = bundle.get("trust_review", {}).get("trust_notes", [])
    if (
        bundle.get("source_review", {}).get("source_class")
        in {"community", "uncertified"}
        and not trust_notes
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.trust.community_notes_missing",
                message=f"{label} community / uncertified source must carry at least one trust note",
                remediation="Add a trust note explaining the unreviewed signer continuity.",
                ref=label,
            )
        )

    return findings


def collect_bundles(fixture_dir: Path) -> dict[str, dict[str, Any]]:
    if not fixture_dir.exists():
        raise SystemExit(f"missing fixture dir: {fixture_dir}")
    bundles: dict[str, dict[str, Any]] = {}
    for path in sorted(fixture_dir.glob("*.json")):
        bundles[path.name] = load_json(path)
    if not bundles:
        raise SystemExit(f"no fixtures in {fixture_dir}")
    return bundles


def validate_coverage(bundles: dict[str, dict[str, Any]]) -> list[Finding]:
    findings: list[Finding] = []
    source_classes = {
        bundle.get("source_review", {}).get("source_class") for bundle in bundles.values()
    }
    missing_sources = REQUIRED_SOURCE_CLASSES - source_classes
    if missing_sources:
        findings.append(
            Finding(
                severity="error",
                check_id="fixtures.coverage.source_class_missing",
                message="fixtures must cover required source classes",
                remediation="Seed a bundle for every required source class.",
                details={"missing": sorted(missing_sources)},
            )
        )
    runtime_scopes = {
        bundle.get("target_runtime_review", {}).get("runtime_scope_class")
        for bundle in bundles.values()
    }
    missing_scopes = REQUIRED_RUNTIME_SCOPES - runtime_scopes
    if missing_scopes:
        findings.append(
            Finding(
                severity="error",
                check_id="fixtures.coverage.runtime_scope_missing",
                message="fixtures must cover required runtime-scope classes",
                remediation="Seed a bundle for every required runtime-scope class.",
                details={"missing": sorted(missing_scopes)},
            )
        )
    return findings


def validate_doc_and_consumer(
    doc_path: Path, consumer_path: Path, crate_module_path: Path
) -> list[Finding]:
    findings: list[Finding] = []
    doc = doc_path.read_text(encoding="utf-8") if doc_path.exists() else ""
    if not doc:
        findings.append(
            Finding(
                severity="error",
                check_id="docs.template_bundle.missing",
                message=f"workspace template-bundle doc is missing: {doc_path}",
                remediation="Author docs/workspace/m3/template_bundle_alpha.md.",
                ref=str(doc_path),
            )
        )
    else:
        for ref in [
            DEFAULT_SCHEMA_REL,
            DEFAULT_FIXTURE_DIR_REL,
            "ci/check_template_bundle_alpha.py",
            DEFAULT_CONSUMER_REL,
        ]:
            if ref not in doc:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="docs.template_bundle.missing_ref",
                        message=f"workspace template-bundle doc does not mention {ref}",
                        remediation="Document the schema, fixtures, validator, and first consumer together.",
                        ref=str(doc_path),
                    )
                )
    consumer = (
        consumer_path.read_text(encoding="utf-8") if consumer_path.exists() else ""
    )
    if not consumer:
        findings.append(
            Finding(
                severity="error",
                check_id="consumer.template_bundle.missing",
                message=f"start-center template-bundle consumer is missing: {consumer_path}",
                remediation="Wire the first shell consumer at crates/aureline-shell/src/start_center/template_bundles/mod.rs.",
                ref=str(consumer_path),
            )
        )
    else:
        for token in [
            "build_alpha_template_bundle_rows",
            "render_alpha_template_bundle_plaintext",
            "project_workspace_template_bundle",
        ]:
            if token not in consumer:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="consumer.template_bundle.missing_token",
                        message=f"start-center template-bundle consumer does not contain {token}",
                        remediation="Keep the alpha bundle wired into the first shell consumer.",
                        ref=str(consumer_path),
                    )
                )
    crate_module = (
        crate_module_path.read_text(encoding="utf-8") if crate_module_path.exists() else ""
    )
    if not crate_module:
        findings.append(
            Finding(
                severity="error",
                check_id="crate.template_bundle.missing",
                message=f"aureline-workspace templates module is missing: {crate_module_path}",
                remediation="Add the templates module to aureline-workspace.",
                ref=str(crate_module_path),
            )
        )
    else:
        for token in [
            "WorkspaceTemplateBundleRecord",
            "project_workspace_template_bundle",
            "TEMPLATE_BUNDLE_BYPASS_CONTINUITY_CLASS_EQUAL_WEIGHT",
        ]:
            if token not in crate_module:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="crate.template_bundle.missing_token",
                        message=f"aureline-workspace templates module does not export {token}",
                        remediation="Keep the alpha bundle vocabulary exported from aureline-workspace.",
                        ref=str(crate_module_path),
                    )
                )
    return findings


def render_bundle_gallery(bundles: dict[str, dict[str, Any]]) -> str:
    lines = [
        "Workspace template bundle alpha gallery",
        "bundle_id | source | support | runtime | egress | bypass_routes",
    ]
    for name in sorted(bundles):
        bundle = bundles[name]
        source = bundle.get("source_review", {})
        support = bundle.get("support_review", {})
        runtime = bundle.get("target_runtime_review", {})
        side_effects = bundle.get("side_effect_review", {})
        bypass = bundle.get("bypass_review", {})
        lines.append(
            " | ".join(
                [
                    bundle.get("bundle_id", name),
                    f"{source.get('source_class', '?')}:{source.get('signature_state', '?')}",
                    support.get("support_class", "?"),
                    f"{runtime.get('runtime_scope_class', '?')}/{runtime.get('host_boundary_class', '?')}",
                    side_effects.get("required_network_egress_class", "?"),
                    ",".join(bypass.get("open_without_starter_route_ids", [])),
                ]
            )
        )
    return "\n".join(lines) + "\n"


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    schema_path = repo_root / args.schema
    fixture_dir = repo_root / args.fixture_dir
    doc_path = repo_root / args.doc
    consumer_path = repo_root / args.consumer
    crate_module_path = repo_root / args.crate_module

    schema = load_json(schema_path)
    bundles = collect_bundles(fixture_dir)

    findings: list[Finding] = []
    for name, bundle in bundles.items():
        validation_payload = {
            key: value for key, value in bundle.items() if key != "$schema"
        }
        findings.extend(schema_validate(schema, name, validation_payload))
        findings.extend(cross_check_bundle(name, bundle))

    findings.extend(validate_coverage(bundles))
    findings.extend(validate_doc_and_consumer(doc_path, consumer_path, crate_module_path))

    report = {
        "status": "pass" if not findings else "fail",
        "schema_ref": args.schema,
        "fixture_dir": args.fixture_dir,
        "bundle_count": len(bundles),
        "findings": [finding.as_report() for finding in findings],
    }
    if args.report:
        Path(args.report).write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    if args.render_bundle_gallery:
        print(render_bundle_gallery(bundles), end="")
    if findings:
        print(json.dumps(report, indent=2), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
