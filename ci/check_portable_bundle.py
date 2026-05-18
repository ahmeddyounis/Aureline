#!/usr/bin/env python3
"""Validate portable bundle schema, fixtures, docs, and consumers."""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator, FormatChecker


DEFAULT_SCHEMA_REL = "schemas/change/portable_bundle.schema.json"
DEFAULT_FIXTURE_DIR_REL = "fixtures/review/m3/portable_bundle"
DEFAULT_DOC_REL = "docs/ux/m3/portable_bundle_and_shelf_beta.md"
DEFAULT_ARTIFACT_REL = "artifacts/support/m3/portable_bundle_handoff_review.md"
DEFAULT_CRATE_MODULE_REL = "crates/aureline-change-objects/src/portable_bundle/mod.rs"
DEFAULT_SHELL_CONSUMER_REL = "crates/aureline-shell/src/portable_bundle_inspector/mod.rs"
DEFAULT_SUPPORT_CONSUMER_REL = "crates/aureline-support/src/portable_bundle_handoff/mod.rs"

REQUIRED_PURPOSES = {
    "offline_review_handoff",
    "browser_companion_handoff",
    "incident_follow_up",
    "support_export",
}
REQUIRED_OBJECT_CLASSES = {"portable_bundle", "shelf_entry"}
REQUIRED_OPEN_MODES = {
    "inspect_offline",
    "compare_only_reopen",
    "desktop_resume_after_revalidation",
    "browser_companion_read_only",
    "support_export_inspect",
}
REQUIRED_CONSUMERS = {"portable_bundle_inspector", "support_export"}
RAW_EXPORT_KEYS = {
    "raw_path_export_allowed",
    "raw_remote_url_export_allowed",
    "raw_secret_export_allowed",
    "raw_credential_export_allowed",
}
AUTHORITY_KEYS = {
    "live_bearer_authority_included",
    "ambient_credentials_included",
    "secret_material_included",
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
    parser.add_argument("--artifact", default=DEFAULT_ARTIFACT_REL)
    parser.add_argument("--crate-module", default=DEFAULT_CRATE_MODULE_REL)
    parser.add_argument("--shell-consumer", default=DEFAULT_SHELL_CONSUMER_REL)
    parser.add_argument("--support-consumer", default=DEFAULT_SUPPORT_CONSUMER_REL)
    parser.add_argument("--report", default=None)
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
                check_id="portable_bundle.schema.validation_failed",
                message=f"{label}: {path}: {error.message}",
                remediation="Fix the portable bundle record to validate against schemas/change/portable_bundle.schema.json.",
                ref=label,
                details={"schema_path": list(error.schema_path)},
            )
        )
    return findings


def cross_check_record(label: str, record: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    validation = record.get("validation_state", {})
    freshness_class = validation.get("freshness_class")
    staleness_labels = validation.get("staleness_labels", [])
    open_modes = set(record.get("open_modes", []))
    if freshness_class != "validation_current":
        if not staleness_labels:
            findings.append(
                Finding(
                    severity="error",
                    check_id="portable_bundle.validation.stale_label_missing",
                    message=f"{label} has stale validation without stale labels",
                    remediation="Add at least one staleness_labels entry.",
                    ref=label,
                )
            )
        if "compare_only_reopen" not in open_modes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="portable_bundle.validation.compare_only_missing",
                    message=f"{label} has stale validation without compare_only_reopen",
                    remediation="Stale/imported bundles must be reopenable compare-only.",
                    ref=label,
                )
            )

    if "inspect_offline" not in open_modes:
        findings.append(
            Finding(
                severity="error",
                check_id="portable_bundle.open.inspect_offline_missing",
                message=f"{label} cannot be inspected offline",
                remediation="Add inspect_offline to open_modes.",
                ref=label,
            )
        )

    consumers = set(record.get("consumer_surfaces", []))
    missing_consumers = REQUIRED_CONSUMERS - consumers
    if missing_consumers:
        findings.append(
            Finding(
                severity="error",
                check_id="portable_bundle.consumer.required_missing",
                message=f"{label} is missing required consumer surfaces",
                remediation="Wire portable_bundle_inspector and support_export.",
                ref=label,
                details={"missing": sorted(missing_consumers)},
            )
        )

    authority = record.get("authority_state", {})
    for key in AUTHORITY_KEYS:
        if authority.get(key) is not False:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"portable_bundle.authority.{key}_included",
                    message=f"{label} authority_state.{key} must be false",
                    remediation="Portable bundles must not carry live authority, ambient credentials, or secret material.",
                    ref=label,
                )
            )

    redaction = record.get("redaction_profile", {})
    for key in RAW_EXPORT_KEYS:
        if redaction.get(key) is not False:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"portable_bundle.redaction.{key}_widened",
                    message=f"{label} redaction_profile.{key} must be false",
                    remediation="Keep raw path, remote URL, secret, and credential export closed.",
                    ref=label,
                )
            )

    for index, diff_ref in enumerate(record.get("diff_refs", [])):
        if diff_ref.get("raw_diff_body_included") is not False:
            findings.append(
                Finding(
                    severity="error",
                    check_id="portable_bundle.diff.raw_body_included",
                    message=f"{label} diff_refs[{index}].raw_diff_body_included must be false",
                    remediation="Use opaque body_ref or diff_ref entries; do not inline raw diff bodies.",
                    ref=label,
                )
            )

    invariants = record.get("review_invariants", {})
    for key in (
        "open_import_export_parity",
        "target_identity_pinned",
        "diff_refs_preserved",
        "evidence_refs_preserved",
        "no_live_provider_authority",
        "secrets_excluded",
        "stale_validation_labeled",
        "redaction_profile_declared",
    ):
        if invariants.get(key) is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"portable_bundle.invariant.{key}_missing",
                    message=f"{label} review_invariants.{key} must be true",
                    remediation="Portable bundles are pre-execution review records; keep all invariants true.",
                    ref=label,
                )
            )

    return findings


def collect_records(fixture_dir: Path) -> dict[str, dict[str, Any]]:
    if not fixture_dir.exists():
        raise SystemExit(f"missing fixture dir: {fixture_dir}")
    records: dict[str, dict[str, Any]] = {}
    for path in sorted(fixture_dir.glob("*.json")):
        records[path.name] = load_json(path)
    if not records:
        raise SystemExit(f"no fixtures in {fixture_dir}")
    return records


def validate_coverage(records: dict[str, dict[str, Any]]) -> list[Finding]:
    findings: list[Finding] = []
    purposes = {record.get("handoff_purpose_class") for record in records.values()}
    missing_purposes = REQUIRED_PURPOSES - purposes
    if missing_purposes:
        findings.append(
            Finding(
                severity="error",
                check_id="portable_bundle.coverage.purpose_missing",
                message="fixtures must cover required handoff purposes",
                remediation="Add fixtures for missing handoff_purpose_class values.",
                details={"missing": sorted(missing_purposes)},
            )
        )
    object_classes = {record.get("object_class") for record in records.values()}
    missing_objects = REQUIRED_OBJECT_CLASSES - object_classes
    if missing_objects:
        findings.append(
            Finding(
                severity="error",
                check_id="portable_bundle.coverage.object_class_missing",
                message="fixtures must cover portable_bundle and shelf_entry",
                remediation="Add a fixture for each object_class.",
                details={"missing": sorted(missing_objects)},
            )
        )
    open_modes: set[str] = set()
    for record in records.values():
        open_modes.update(record.get("open_modes", []))
    missing_open_modes = REQUIRED_OPEN_MODES - open_modes
    if missing_open_modes:
        findings.append(
            Finding(
                severity="error",
                check_id="portable_bundle.coverage.open_mode_missing",
                message="fixtures must cover required open modes",
                remediation="Add fixtures covering offline inspect, compare-only, desktop resume, browser read-only, and support inspect.",
                details={"missing": sorted(missing_open_modes)},
            )
        )
    if not any(
        record.get("validation_state", {}).get("staleness_labels")
        for record in records.values()
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="portable_bundle.coverage.stale_label_missing",
                message="fixtures must cover stale-validation labels",
                remediation="Add a stale/imported fixture with staleness_labels.",
            )
        )
    return findings


def validate_docs_and_consumers(paths: list[Path]) -> list[Finding]:
    findings: list[Finding] = []
    required_tokens = [
        DEFAULT_SCHEMA_REL,
        DEFAULT_FIXTURE_DIR_REL,
        DEFAULT_CRATE_MODULE_REL,
        DEFAULT_SHELL_CONSUMER_REL,
        DEFAULT_SUPPORT_CONSUMER_REL,
    ]
    for path in paths:
        text = path.read_text(encoding="utf-8") if path.exists() else ""
        if not text:
            findings.append(
                Finding(
                    severity="error",
                    check_id="portable_bundle.file.missing",
                    message=f"required portable bundle file is missing: {path}",
                    remediation="Create the documented portable bundle contract file.",
                    ref=str(path),
                )
            )
            continue
        if path.name.endswith(".md"):
            for token in required_tokens:
                if token not in text:
                    findings.append(
                        Finding(
                            severity="error",
                            check_id="portable_bundle.doc.missing_ref",
                            message=f"{path} does not mention {token}",
                            remediation="Document schema, fixtures, crate, shell consumer, and support consumer together.",
                            ref=str(path),
                        )
                    )
    return findings


def validate_code_tokens(paths: dict[str, Path]) -> list[Finding]:
    findings: list[Finding] = []
    expected = {
        "crate": [
            "PortableBundleRecord",
            "project_portable_bundle",
            "PORTABLE_BUNDLE_RECORD_KIND",
            "current_portable_bundle_fixture_projections",
        ],
        "shell": [
            "build_portable_bundle_rows",
            "render_portable_bundle_plaintext",
            "current_portable_bundle_fixture_projections",
        ],
        "support": [
            "compile_portable_bundle_support_export_envelope",
            "PortableBundleSupportExportEnvelope",
            "PORTABLE_BUNDLE_SUPPORT_ENVELOPE_RECORD_KIND",
        ],
    }
    for key, path in paths.items():
        text = path.read_text(encoding="utf-8") if path.exists() else ""
        if not text:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"portable_bundle.{key}.missing",
                    message=f"portable bundle {key} file is missing",
                    remediation="Wire the portable bundle consumer.",
                    ref=str(path),
                )
            )
            continue
        for token in expected[key]:
            if token not in text:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"portable_bundle.{key}.missing_token",
                        message=f"{path} does not contain {token}",
                        remediation="Keep the portable bundle code path wired to the shared contract.",
                        ref=str(path),
                    )
                )
    return findings


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    schema_path = repo_root / args.schema
    fixture_dir = repo_root / args.fixture_dir
    doc_path = repo_root / args.doc
    artifact_path = repo_root / args.artifact
    crate_module_path = repo_root / args.crate_module
    shell_consumer_path = repo_root / args.shell_consumer
    support_consumer_path = repo_root / args.support_consumer

    schema = load_json(schema_path)
    records = collect_records(fixture_dir)

    findings: list[Finding] = []
    for name, record in records.items():
        validation_payload = {
            key: value for key, value in record.items() if key != "$schema"
        }
        findings.extend(schema_validate(schema, name, validation_payload))
        findings.extend(cross_check_record(name, record))

    findings.extend(validate_coverage(records))
    findings.extend(validate_docs_and_consumers([doc_path, artifact_path]))
    findings.extend(
        validate_code_tokens(
            {
                "crate": crate_module_path,
                "shell": shell_consumer_path,
                "support": support_consumer_path,
            }
        )
    )

    report = {
        "status": "pass" if not findings else "fail",
        "schema_ref": args.schema,
        "fixture_dir": args.fixture_dir,
        "record_count": len(records),
        "findings": [finding.as_report() for finding in findings],
    }
    if args.report:
        Path(args.report).write_text(
            json.dumps(report, indent=2) + "\n", encoding="utf-8"
        )
    if findings:
        print(json.dumps(report, indent=2), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
