#!/usr/bin/env python3
"""Validate the alpha prebuild fingerprint schema, fixtures, and consumer wiring."""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator, FormatChecker


DEFAULT_FINGERPRINT_SCHEMA_REL = "schemas/workspace/prebuild_fingerprint.schema.json"
DEFAULT_INVALIDATION_SCHEMA_REL = "schemas/workspace/prebuild_invalidation_reason.schema.json"
DEFAULT_FIXTURE_DIR_REL = "fixtures/workspace/m3/prebuild_fingerprint"
DEFAULT_DOC_REL = "docs/workspace/m3/prebuild_fingerprint_alpha.md"
DEFAULT_CONSUMER_REL = (
    "crates/aureline-shell/src/start_center/prebuild_fingerprints/mod.rs"
)
DEFAULT_CRATE_MODULE_REL = "crates/aureline-workspace/src/prebuilds/mod.rs"

REQUIRED_RECORD_KINDS = {
    "prebuild_fingerprint_record",
    "prebuild_reuse_decision_record",
    "prebuild_disclosure_record",
}
REQUIRED_PATH_CLASSES = {
    "resume_live_workspace",
    "clone_fresh",
    "reuse_cached_prebuild",
}
REQUIRED_DISCLOSURE_RESIDUE_CLASSES = {
    "raw_secret_material",
    "raw_credential_bodies",
    "uncommitted_workspace_edits",
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
    parser.add_argument("--schema", default=DEFAULT_FINGERPRINT_SCHEMA_REL)
    parser.add_argument(
        "--invalidation-schema", default=DEFAULT_INVALIDATION_SCHEMA_REL
    )
    parser.add_argument("--fixture-dir", default=DEFAULT_FIXTURE_DIR_REL)
    parser.add_argument("--doc", default=DEFAULT_DOC_REL)
    parser.add_argument("--consumer", default=DEFAULT_CONSUMER_REL)
    parser.add_argument("--crate-module", default=DEFAULT_CRATE_MODULE_REL)
    parser.add_argument("--report", default=None)
    parser.add_argument(
        "--render-gallery",
        action="store_true",
        help="Print the deterministic prebuild fingerprint projection.",
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
                check_id="prebuild_fingerprint_alpha.schema.validation_failed",
                message=f"{label}: {path}: {error.message}",
                remediation="Fix the fixture or schema so the alpha contract validates.",
                ref=label,
                details={"schema_path": list(error.schema_path)},
            )
        )
    return findings


def cross_check_fingerprint(label: str, record: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    redaction = record.get("redaction_and_portability", {})
    if redaction.get("broadened_capture_approved") is not False:
        findings.append(
            Finding(
                severity="error",
                check_id="fingerprint.redaction.broadened_capture",
                message=f"{label} broadened_capture_approved must remain false",
                remediation="Keep raw secret and residue export closed on the alpha fingerprint.",
                ref=label,
            )
        )
    required = {
        "raw_secret_material",
        "raw_credential_bodies",
        "raw_environment_values",
        "machine_unique_trust_anchors",
        "uncommitted_workspace_edits",
    }
    excluded = set(redaction.get("excluded_residue_classes", []))
    missing = required - excluded
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="fingerprint.redaction.missing_exclusion",
                message=f"{label} excluded_residue_classes missing {sorted(missing)}",
                remediation="Add the required residue exclusions to the fingerprint.",
                ref=label,
            )
        )
    return findings


def cross_check_reuse_decision(label: str, record: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    outcome = record.get("reuse_outcome")
    requested = record.get("requested_path")
    materialization = record.get("source_materialization_class")
    if outcome == "reuse_allowed":
        if record.get("invalidation_bundle_refs"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="decision.reuse_allowed.invalidations_present",
                    message=f"{label} reuse_allowed must clear invalidation_bundle_refs",
                    remediation="Drop invalidation refs from reuse_allowed decisions.",
                    ref=label,
                )
            )
        if record.get("required_revalidations"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="decision.reuse_allowed.revalidations_present",
                    message=f"{label} reuse_allowed must clear required_revalidations",
                    remediation="Drop revalidations from reuse_allowed decisions.",
                    ref=label,
                )
            )
    if requested == "resume_live_workspace" and materialization in {
        "prebuilt_snapshot",
        "stale_prebuild_snapshot",
    } and outcome != "resume_live_denied":
        findings.append(
            Finding(
                severity="error",
                check_id="decision.resume_live.snapshot_widened",
                message=f"{label} resume_live_workspace on snapshot must resolve to resume_live_denied",
                remediation="Stale snapshots cannot masquerade as live resume.",
                ref=label,
            )
        )
    return findings


def cross_check_disclosure(label: str, record: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    if record.get("stale_snapshot_must_not_be_labeled_live_resume") is not True:
        findings.append(
            Finding(
                severity="error",
                check_id="disclosure.invariant.resume_live",
                message=f"{label} stale_snapshot_must_not_be_labeled_live_resume must remain true",
                remediation="Keep the resume-live invariant constant on disclosure records.",
                ref=label,
            )
        )
    excluded = set(record.get("excluded_residue_classes", []))
    missing = REQUIRED_DISCLOSURE_RESIDUE_CLASSES - excluded
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="disclosure.residue.missing",
                message=f"{label} disclosure excluded_residue_classes missing {sorted(missing)}",
                remediation="Add the required residue exclusions to the disclosure.",
                ref=label,
            )
        )
    state = record.get("disclosure_state")
    if state == "fresh_clone" and record.get("fresh_clone_required") is not True:
        findings.append(
            Finding(
                severity="error",
                check_id="disclosure.fresh_clone.flag",
                message=f"{label} fresh_clone disclosure must set fresh_clone_required",
                remediation="A fresh_clone state must require a fresh clone.",
                ref=label,
            )
        )
    if state in {"stale_prebuild_rebuild_required", "local_override_rebuild_required"}:
        if record.get("rebuild_required") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id="disclosure.rebuild.flag",
                    message=f"{label} rebuild-required disclosure must set rebuild_required",
                    remediation="Disclosures that require rebuild must say so.",
                    ref=label,
                )
            )
    if state == "resume_denied_snapshot_available" and record.get("requested_path") != "resume_live_workspace":
        findings.append(
            Finding(
                severity="error",
                check_id="disclosure.resume_denied.path",
                message=f"{label} resume_denied_snapshot_available must wrap a resume_live_workspace request",
                remediation="Use the resume_denied state only on resume_live_workspace requests.",
                ref=label,
            )
        )
    return findings


def cross_check_record(label: str, record: dict[str, Any]) -> list[Finding]:
    kind = record.get("record_kind")
    if kind == "prebuild_fingerprint_record":
        return cross_check_fingerprint(label, record)
    if kind == "prebuild_reuse_decision_record":
        return cross_check_reuse_decision(label, record)
    if kind == "prebuild_disclosure_record":
        return cross_check_disclosure(label, record)
    return [
        Finding(
            severity="error",
            check_id="fixture.record_kind.unknown",
            message=f"{label} unknown record_kind: {kind}",
            remediation="Use one of prebuild_fingerprint_record, prebuild_reuse_decision_record, prebuild_disclosure_record.",
            ref=label,
        )
    ]


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
    kinds = {record.get("record_kind") for record in records.values()}
    missing_kinds = REQUIRED_RECORD_KINDS - kinds
    if missing_kinds:
        findings.append(
            Finding(
                severity="error",
                check_id="fixtures.coverage.record_kind_missing",
                message="fixtures must cover all alpha record kinds",
                remediation="Seed a fixture for every required record kind.",
                details={"missing": sorted(missing_kinds)},
            )
        )
    paths = {
        record.get("requested_path")
        for record in records.values()
        if record.get("requested_path")
    }
    missing_paths = REQUIRED_PATH_CLASSES - paths
    if missing_paths:
        findings.append(
            Finding(
                severity="error",
                check_id="fixtures.coverage.path_missing",
                message="fixtures must cover required prebuild paths",
                remediation="Seed a fixture exercising each acceptance entry path.",
                details={"missing": sorted(missing_paths)},
            )
        )
    if not any(
        record.get("reuse_outcome") == "resume_live_denied"
        for record in records.values()
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="fixtures.coverage.resume_live_denied_missing",
                message="fixtures must include a resume_live_denied decision",
                remediation="Seed a stale-snapshot resume request that is denied.",
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
                check_id="docs.prebuild_fingerprint.missing",
                message=f"workspace prebuild fingerprint doc is missing: {doc_path}",
                remediation="Author docs/workspace/m3/prebuild_fingerprint_alpha.md.",
                ref=str(doc_path),
            )
        )
    else:
        for ref in [
            DEFAULT_FINGERPRINT_SCHEMA_REL,
            DEFAULT_FIXTURE_DIR_REL,
            "ci/check_prebuild_fingerprint_alpha.py",
            DEFAULT_CONSUMER_REL,
            DEFAULT_CRATE_MODULE_REL,
        ]:
            if ref not in doc:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="docs.prebuild_fingerprint.missing_ref",
                        message=f"prebuild fingerprint doc does not mention {ref}",
                        remediation="Document schema, fixtures, validator, consumer, and crate module together.",
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
                check_id="consumer.prebuild_fingerprint.missing",
                message=f"start-center prebuild fingerprint consumer is missing: {consumer_path}",
                remediation="Wire the first shell consumer at crates/aureline-shell/src/start_center/prebuild_fingerprints/mod.rs.",
                ref=str(consumer_path),
            )
        )
    else:
        for token in [
            "build_alpha_prebuild_fingerprint_rows",
            "render_alpha_prebuild_fingerprint_plaintext",
            "project_prebuild_fingerprint_alpha",
        ]:
            if token not in consumer:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="consumer.prebuild_fingerprint.missing_token",
                        message=f"start-center prebuild fingerprint consumer does not contain {token}",
                        remediation="Keep the alpha vocabulary wired into the first shell consumer.",
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
                check_id="crate.prebuild_fingerprint.missing",
                message=f"aureline-workspace prebuilds module is missing: {crate_module_path}",
                remediation="Add the prebuilds module to aureline-workspace.",
                ref=str(crate_module_path),
            )
        )
    else:
        for token in [
            "PrebuildFingerprintRecord",
            "PrebuildReuseDecisionRecord",
            "PrebuildDisclosureRecord",
            "project_prebuild_fingerprint_alpha",
            "PREBUILD_FINGERPRINT_ALPHA_SCHEMA_VERSION",
        ]:
            if token not in crate_module:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="crate.prebuild_fingerprint.missing_token",
                        message=f"aureline-workspace prebuilds module does not export {token}",
                        remediation="Keep the alpha vocabulary exported from aureline-workspace.",
                        ref=str(crate_module_path),
                    )
                )
    return findings


def render_gallery(records: dict[str, dict[str, Any]]) -> str:
    lines = [
        "Prebuild fingerprint alpha gallery",
        "fixture | record_kind | requested_path | outcome_or_state | freshness | host/arch",
    ]
    for name in sorted(records):
        record = records[name]
        kind = record.get("record_kind", "?")
        path = record.get("requested_path", "fingerprint")
        outcome = record.get("reuse_outcome") or record.get("disclosure_state") or "fingerprint"
        freshness = (
            record.get("freshness", {}).get("freshness_age_class")
            if kind == "prebuild_fingerprint_record"
            else record.get("freshness_age_class", "?")
        )
        host = (
            record.get("environment_identity", {}).get("host_class")
            if kind == "prebuild_fingerprint_record"
            else record.get("host_class", "?")
        )
        arch = (
            record.get("environment_identity", {}).get("platform_arch")
            if kind == "prebuild_fingerprint_record"
            else record.get("platform_arch", "?")
        )
        lines.append(f"{name} | {kind} | {path} | {outcome} | {freshness} | {host}/{arch}")
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
    records = collect_records(fixture_dir)

    findings: list[Finding] = []
    for name, record in records.items():
        payload = {key: value for key, value in record.items() if key != "$schema"}
        findings.extend(schema_validate(schema, name, payload))
        findings.extend(cross_check_record(name, record))

    findings.extend(validate_coverage(records))
    findings.extend(validate_doc_and_consumer(doc_path, consumer_path, crate_module_path))

    report = {
        "status": "pass" if not findings else "fail",
        "schema_ref": args.schema,
        "fixture_dir": args.fixture_dir,
        "fixture_count": len(records),
        "findings": [finding.as_report() for finding in findings],
    }
    if args.report:
        Path(args.report).write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    if args.render_gallery:
        print(render_gallery(records), end="")
    if findings:
        print(json.dumps(report, indent=2), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
