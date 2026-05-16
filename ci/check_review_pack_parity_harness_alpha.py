#!/usr/bin/env python3
"""Validate the alpha review-pack parity-harness schema, fixtures, and consumer wiring."""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator, FormatChecker


DEFAULT_SCHEMA_REL = "schemas/review/review_pack_parity_harness.schema.json"
DEFAULT_FIXTURE_DIR_REL = "fixtures/review/m3/review_pack_harness"
DEFAULT_DSL_FIXTURE_DIR_REL = "fixtures/review/m3/review_pack_dsl"
DEFAULT_DOC_REL = "docs/review/m3/review_pack_parity_alpha.md"
DEFAULT_REPORT_REL = "artifacts/review/m3/local_ci_parity_report.md"
DEFAULT_CONSUMER_REL = "crates/aureline-shell/src/review/parity_harness_inspector/mod.rs"
DEFAULT_CRATE_MODULE_REL = "crates/aureline-review/src/review_pack_parity_harness/mod.rs"

REQUIRED_AUTHORITIES = {
    "repo_first_party",
    "repo_team_shared",
    "repo_partner_signed",
    "repo_uncertified_community",
}
REQUIRED_VERDICTS = {"full_parity", "drift_downgraded"}
REQUIRED_FINDING_CLASSES = {
    "full_parity",
    "local_only_documented_match",
    "ci_only_documented_match",
    "drift_detected",
}
REQUIRED_CONSUMER = "parity_harness_inspector"


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
    parser.add_argument("--dsl-fixture-dir", default=DEFAULT_DSL_FIXTURE_DIR_REL)
    parser.add_argument("--doc", default=DEFAULT_DOC_REL)
    parser.add_argument("--report-artifact", default=DEFAULT_REPORT_REL)
    parser.add_argument("--consumer", default=DEFAULT_CONSUMER_REL)
    parser.add_argument("--crate-module", default=DEFAULT_CRATE_MODULE_REL)
    parser.add_argument("--report", default=None)
    parser.add_argument(
        "--render-gallery",
        action="store_true",
        help="Print the deterministic parity-harness inspector projection after validation.",
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
                check_id="review_pack_parity_harness_alpha.schema.validation_failed",
                message=f"{label}: {path}: {error.message}",
                remediation=(
                    "Fix the parity-harness record to validate against "
                    "schemas/review/review_pack_parity_harness.schema.json."
                ),
                ref=label,
                details={"schema_path": list(error.schema_path)},
            )
        )
    return findings


def cross_check_record(
    label: str, record: dict[str, Any], dsl_pack_ids: set[str]
) -> list[Finding]:
    findings: list[Finding] = []
    findings_list = record.get("check_parity_findings", [])
    drift_downgrades = record.get("drift_downgrades", [])
    drift_check_refs = {
        d.get("check_ref")
        for d in drift_downgrades
        if isinstance(d, dict)
    }
    finding_check_refs = {
        f.get("check_ref")
        for f in findings_list
        if isinstance(f, dict)
    }

    for downgrade in drift_downgrades:
        if not isinstance(downgrade, dict):
            continue
        check_ref = downgrade.get("check_ref")
        if check_ref not in finding_check_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="parity_harness.downgrade.unknown_check_ref",
                    message=(
                        f"{label} drift_downgrades references check_ref {check_ref} "
                        "that is not present in check_parity_findings"
                    ),
                    remediation=(
                        "Ensure every drift_downgrades entry names a check that the "
                        "harness actually observed."
                    ),
                    ref=label,
                )
            )
        if downgrade.get("downgrade_class") == "no_downgrade":
            findings.append(
                Finding(
                    severity="error",
                    check_id="parity_harness.downgrade.no_downgrade_listed",
                    message=(
                        f"{label} drift_downgrades carries a no_downgrade entry; "
                        "only real downgrades belong on the list"
                    ),
                    remediation="Remove the no_downgrade entry.",
                    ref=label,
                )
            )

    has_drift = False
    for finding in findings_list:
        if not isinstance(finding, dict):
            continue
        check_ref = finding.get("check_ref")
        if finding.get("parity_finding_class") == "drift_detected":
            has_drift = True
            if check_ref not in drift_check_refs:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="parity_harness.drift.downgrade_missing",
                        message=(
                            f"{label} finding {check_ref} reports drift_detected "
                            "but no drift_downgrades entry was recorded"
                        ),
                        remediation=(
                            "Add a drift_downgrades entry for the drifted check so "
                            "the parity claim is downgraded explicitly."
                        ),
                        ref=label,
                    )
                )

    overall_verdict = record.get("overall_verdict_class")
    row_downgrade = record.get("row_downgrade_class")
    if has_drift and row_downgrade == "no_downgrade":
        findings.append(
            Finding(
                severity="error",
                check_id="parity_harness.row.drift_without_row_downgrade",
                message=(
                    f"{label} row_downgrade_class=no_downgrade but a finding "
                    "reports drift_detected"
                ),
                remediation=(
                    "Set row_downgrade_class to the matching downgrade so the row "
                    "cannot preserve a green claim under drift."
                ),
                ref=label,
            )
        )
    if overall_verdict == "drift_downgraded" and row_downgrade == "no_downgrade":
        findings.append(
            Finding(
                severity="error",
                check_id="parity_harness.row.verdict_without_row_downgrade",
                message=(
                    f"{label} overall_verdict_class=drift_downgraded but "
                    "row_downgrade_class=no_downgrade"
                ),
                remediation=(
                    "Drift verdicts must downgrade the row; set row_downgrade_class."
                ),
                ref=label,
            )
        )
    if overall_verdict == "full_parity" and has_drift:
        findings.append(
            Finding(
                severity="error",
                check_id="parity_harness.verdict.full_parity_with_drift",
                message=(
                    f"{label} overall_verdict_class=full_parity but a finding "
                    "reports drift_detected"
                ),
                remediation=(
                    "Change the verdict to drift_downgraded or remove the drift_detected finding."
                ),
                ref=label,
            )
        )

    lanes = {
        lane.get("lane_class")
        for lane in record.get("harness_lane_observations", [])
        if isinstance(lane, dict)
    }
    for required in ("local_lane", "ci_lane"):
        if required not in lanes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="parity_harness.lane.missing",
                    message=f"{label} harness_lane_observations is missing {required}",
                    remediation="Both local_lane and ci_lane must be observed.",
                    ref=label,
                )
            )

    consumers = set(record.get("consumer_surfaces", []))
    if REQUIRED_CONSUMER not in consumers:
        findings.append(
            Finding(
                severity="error",
                check_id="parity_harness.consumer.inspector_missing",
                message=f"{label} consumer_surfaces does not include {REQUIRED_CONSUMER}",
                remediation="Wire parity_harness_inspector as a consumer surface.",
                ref=label,
            )
        )

    invariants = record.get("review_invariants", {})
    for key in (
        "review_pack_ref_pinned",
        "harness_lanes_pinned",
        "check_findings_pinned",
        "drift_downgrades_pinned",
        "overall_verdict_pinned",
        "no_hidden_writes",
    ):
        if invariants.get(key) is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"parity_harness.review.{key}_missing",
                    message=f"{label} review_invariants.{key} must be true",
                    remediation=(
                        "Keep every parity-harness review invariant true; "
                        "the record is pre-publication review truth for the parity claim."
                    ),
                    ref=label,
                )
            )

    support_export = record.get("support_export", {})
    for key in (
        "raw_path_export_allowed",
        "raw_glob_body_export_allowed",
        "raw_command_export_allowed",
        "raw_check_output_export_allowed",
    ):
        if support_export.get(key) is not False:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"parity_harness.support_export.{key}_widened",
                    message=f"{label} support_export.{key} must be false",
                    remediation=(
                        "Keep raw path, glob body, command line, and check-output export closed."
                    ),
                    ref=label,
                )
            )

    review_pack_ref = record.get("review_pack_ref")
    if review_pack_ref and dsl_pack_ids and review_pack_ref not in dsl_pack_ids:
        findings.append(
            Finding(
                severity="error",
                check_id="parity_harness.review_pack_ref.unknown",
                message=(
                    f"{label} review_pack_ref {review_pack_ref} does not match any "
                    "checked-in review-pack DSL fixture"
                ),
                remediation=(
                    "Mint a parity-harness record only for review-pack DSL fixtures "
                    "that exist under fixtures/review/m3/review_pack_dsl/."
                ),
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


def collect_dsl_pack_ids(dsl_dir: Path) -> set[str]:
    pack_ids: set[str] = set()
    if not dsl_dir.exists():
        return pack_ids
    for path in sorted(dsl_dir.glob("*.json")):
        record = load_json(path)
        if isinstance(record, dict) and isinstance(record.get("review_pack_id"), str):
            pack_ids.add(record["review_pack_id"])
    return pack_ids


def validate_coverage(records: dict[str, dict[str, Any]]) -> list[Finding]:
    findings: list[Finding] = []
    authorities = {record.get("pack_authority_class") for record in records.values()}
    missing_authorities = REQUIRED_AUTHORITIES - authorities
    if missing_authorities:
        findings.append(
            Finding(
                severity="error",
                check_id="fixtures.coverage.authority_missing",
                message="fixtures must cover every required pack_authority_class",
                remediation="Seed parity-harness fixtures for each pack authority class.",
                details={"missing": sorted(missing_authorities)},
            )
        )
    verdicts = {record.get("overall_verdict_class") for record in records.values()}
    missing_verdicts = REQUIRED_VERDICTS - verdicts
    if missing_verdicts:
        findings.append(
            Finding(
                severity="error",
                check_id="fixtures.coverage.verdict_missing",
                message=(
                    "fixtures must cover both a full_parity run and a "
                    "drift_downgraded run"
                ),
                remediation=(
                    "Seed at least one parity-harness fixture per verdict class so "
                    "the drift-downgrade path is provable."
                ),
                details={"missing": sorted(missing_verdicts)},
            )
        )
    finding_classes: set[str] = set()
    for record in records.values():
        for finding in record.get("check_parity_findings", []):
            cls = finding.get("parity_finding_class") if isinstance(finding, dict) else None
            if cls:
                finding_classes.add(cls)
    missing_finding_classes = REQUIRED_FINDING_CLASSES - finding_classes
    if missing_finding_classes:
        findings.append(
            Finding(
                severity="error",
                check_id="fixtures.coverage.finding_class_missing",
                message=(
                    "fixtures must cover full_parity, local_only_documented_match, "
                    "ci_only_documented_match, and drift_detected findings"
                ),
                remediation="Spread the parity-harness fixtures across each finding class.",
                details={"missing": sorted(missing_finding_classes)},
            )
        )
    return findings


def validate_doc_consumer_and_artifact(
    doc_path: Path,
    consumer_path: Path,
    crate_module_path: Path,
    report_artifact_path: Path,
) -> list[Finding]:
    findings: list[Finding] = []
    doc = doc_path.read_text(encoding="utf-8") if doc_path.exists() else ""
    if not doc:
        findings.append(
            Finding(
                severity="error",
                check_id="docs.review_pack_parity.missing",
                message=f"parity-harness reviewer doc is missing: {doc_path}",
                remediation="Author docs/review/m3/review_pack_parity_alpha.md.",
                ref=str(doc_path),
            )
        )
    else:
        for ref in [
            DEFAULT_SCHEMA_REL,
            DEFAULT_FIXTURE_DIR_REL,
            "ci/check_review_pack_parity_harness_alpha.py",
            DEFAULT_CONSUMER_REL,
            DEFAULT_CRATE_MODULE_REL,
            DEFAULT_REPORT_REL,
        ]:
            if ref not in doc:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="docs.review_pack_parity.missing_ref",
                        message=f"parity-harness reviewer doc does not mention {ref}",
                        remediation="Document the schema, fixtures, validator, crate module, consumer, and parity report together.",
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
                check_id="consumer.parity_harness.missing",
                message=f"shell parity-harness inspector consumer is missing: {consumer_path}",
                remediation=(
                    "Wire the first shell consumer at "
                    "crates/aureline-shell/src/review/parity_harness_inspector/mod.rs."
                ),
                ref=str(consumer_path),
            )
        )
    else:
        for token in [
            "build_alpha_parity_harness_rows",
            "render_alpha_parity_harness_plaintext",
            "project_review_pack_parity_harness",
        ]:
            if token not in consumer:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="consumer.parity_harness.missing_token",
                        message=f"shell parity-harness inspector does not contain {token}",
                        remediation="Keep the alpha parity-harness family wired into the first shell consumer.",
                        ref=str(consumer_path),
                    )
                )

    crate_module = (
        crate_module_path.read_text(encoding="utf-8")
        if crate_module_path.exists()
        else ""
    )
    if not crate_module:
        findings.append(
            Finding(
                severity="error",
                check_id="crate.parity_harness.missing",
                message=f"aureline-review review_pack_parity_harness module is missing: {crate_module_path}",
                remediation="Add the review_pack_parity_harness module to aureline-review.",
                ref=str(crate_module_path),
            )
        )
    else:
        for token in [
            "ReviewPackParityHarnessRecord",
            "project_review_pack_parity_harness",
            "REVIEW_PACK_PARITY_HARNESS_ALPHA_RECORD_KIND",
            "REVIEW_PACK_PARITY_HARNESS_PARITY_FINDING_CLASSES",
            "REVIEW_PACK_PARITY_HARNESS_AUTHORITY_CLASSES",
        ]:
            if token not in crate_module:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="crate.parity_harness.missing_token",
                        message=f"aureline-review review_pack_parity_harness module does not export {token}",
                        remediation="Keep the alpha parity-harness vocabulary exported from aureline-review.",
                        ref=str(crate_module_path),
                    )
                )

    report_text = (
        report_artifact_path.read_text(encoding="utf-8")
        if report_artifact_path.exists()
        else ""
    )
    if not report_text:
        findings.append(
            Finding(
                severity="error",
                check_id="artifact.local_ci_parity_report.missing",
                message=f"local-CI parity report artifact is missing: {report_artifact_path}",
                remediation=(
                    "Land artifacts/review/m3/local_ci_parity_report.md summarising "
                    "the parity-harness runs for docs, support, and partner packets."
                ),
                ref=str(report_artifact_path),
            )
        )
    else:
        for token in [
            DEFAULT_SCHEMA_REL,
            DEFAULT_FIXTURE_DIR_REL,
            "drift_downgraded",
            "full_parity",
        ]:
            if token not in report_text:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="artifact.local_ci_parity_report.missing_token",
                        message=f"local-CI parity report does not mention {token}",
                        remediation=(
                            "Keep the report consumable by docs, support, and partner packets."
                        ),
                        ref=str(report_artifact_path),
                    )
                )

    return findings


def render_gallery(records: dict[str, dict[str, Any]]) -> str:
    lines = [
        "Review-pack parity-harness alpha gallery",
        "parity_harness_id | review_pack_ref | authority | verdict | row_downgrade | findings | drift | downgrades",
    ]
    for name in sorted(records):
        record = records[name]
        findings_list = record.get("check_parity_findings", [])
        drift = sum(
            1
            for f in findings_list
            if isinstance(f, dict) and f.get("parity_finding_class") == "drift_detected"
        )
        lines.append(
            " | ".join(
                [
                    record.get("parity_harness_id", name),
                    record.get("review_pack_ref", "?"),
                    record.get("pack_authority_class", "?"),
                    record.get("overall_verdict_class", "?"),
                    record.get("row_downgrade_class", "?"),
                    str(len(findings_list)),
                    str(drift),
                    str(len(record.get("drift_downgrades", []))),
                ]
            )
        )
    return "\n".join(lines) + "\n"


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    schema_path = repo_root / args.schema
    fixture_dir = repo_root / args.fixture_dir
    dsl_fixture_dir = repo_root / args.dsl_fixture_dir
    doc_path = repo_root / args.doc
    consumer_path = repo_root / args.consumer
    crate_module_path = repo_root / args.crate_module
    report_artifact_path = repo_root / args.report_artifact

    schema = load_json(schema_path)
    records = collect_records(fixture_dir)
    dsl_pack_ids = collect_dsl_pack_ids(dsl_fixture_dir)

    findings: list[Finding] = []
    for name, record in records.items():
        validation_payload = {
            key: value for key, value in record.items() if key != "$schema"
        }
        findings.extend(schema_validate(schema, name, validation_payload))
        findings.extend(cross_check_record(name, record, dsl_pack_ids))

    findings.extend(validate_coverage(records))
    findings.extend(
        validate_doc_consumer_and_artifact(
            doc_path, consumer_path, crate_module_path, report_artifact_path
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
    if args.render_gallery:
        print(render_gallery(records), end="")
    if findings:
        print(json.dumps(report, indent=2), file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
