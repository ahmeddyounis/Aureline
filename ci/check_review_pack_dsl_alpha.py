#!/usr/bin/env python3
"""Validate the alpha review-pack DSL schema, fixtures, and consumer wiring."""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

from jsonschema import Draft202012Validator, FormatChecker


DEFAULT_SCHEMA_REL = "schemas/review/review_pack.schema.json"
DEFAULT_FIXTURE_DIR_REL = "fixtures/review/m3/review_pack_dsl"
DEFAULT_DOC_REL = "docs/review/m3/review_pack_dsl_alpha.md"
DEFAULT_CONSUMER_REL = "crates/aureline-shell/src/review/review_pack_inspector/mod.rs"
DEFAULT_CRATE_MODULE_REL = "crates/aureline-review/src/review_pack_dsl/mod.rs"

REQUIRED_AUTHORITIES = {
    "repo_first_party",
    "repo_team_shared",
    "repo_partner_signed",
    "repo_uncertified_community",
}
REQUIRED_PARITY_CLASSES = {
    "local_and_ci_parity",
    "ci_only_documented",
    "local_only_documented",
    "parity_unknown_requires_review",
}
REQUIRED_CONSUMER = "review_pack_inspector"


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
        "--render-gallery",
        action="store_true",
        help="Print the deterministic review-pack inspector projection after validation.",
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
                check_id="review_pack_dsl_alpha.schema.validation_failed",
                message=f"{label}: {path}: {error.message}",
                remediation=(
                    "Fix the review-pack record to validate against "
                    "schemas/review/review_pack.schema.json."
                ),
                ref=label,
                details={"schema_path": list(error.schema_path)},
            )
        )
    return findings


def cross_check_record(label: str, record: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    checks = record.get("checks", [])
    ownership_hints = record.get("ownership_hints", [])
    parity_observations = record.get("parity_observations", [])
    known_scopes = {
        hint.get("ownership_scope_id")
        for hint in ownership_hints
        if isinstance(hint, dict)
    }
    declared_parity_classes = {
        obs.get("parity_class")
        for obs in parity_observations
        if isinstance(obs, dict)
    }

    for check in checks:
        if not isinstance(check, dict):
            continue
        for scope_ref in check.get("ownership_scope_refs", []):
            if scope_ref not in known_scopes:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="review_pack_dsl.ownership.undeclared",
                        message=(
                            f"{label} check {check.get('check_id')} references undeclared "
                            f"ownership_scope_id {scope_ref}"
                        ),
                        remediation=(
                            "Declare the scope in ownership_hints or remove the reference."
                        ),
                        ref=label,
                    )
                )
        parity_class = check.get("parity_class")
        if parity_class and parity_class not in declared_parity_classes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="review_pack_dsl.parity.observation_missing",
                    message=(
                        f"{label} check {check.get('check_id')} declares parity_class "
                        f"{parity_class} without a matching parity_observation"
                    ),
                    remediation=(
                        "Add a parity_observations entry with the same parity_class."
                    ),
                    ref=label,
                )
            )

    consumers = set(record.get("consumer_surfaces", []))
    if REQUIRED_CONSUMER not in consumers:
        findings.append(
            Finding(
                severity="error",
                check_id="review_pack_dsl.consumer.inspector_missing",
                message=f"{label} consumer_surfaces does not include {REQUIRED_CONSUMER}",
                remediation="Wire review_pack_inspector as a consumer surface.",
                ref=label,
            )
        )

    invariants = record.get("review_invariants", {})
    for key in (
        "repo_anchor_pinned",
        "checks_pinned",
        "ownership_hints_pinned",
        "local_ci_parity_declared",
        "unsupported_fields_declared",
        "no_hidden_writes",
    ):
        if invariants.get(key) is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"review_pack_dsl.review.{key}_missing",
                    message=f"{label} review_invariants.{key} must be true",
                    remediation=(
                        "The record is a pre-execution review record; keep every review invariant true."
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
                    check_id=f"review_pack_dsl.support_export.{key}_widened",
                    message=f"{label} support_export.{key} must be false",
                    remediation=(
                        "Keep raw path, glob body, command line, and check-output export closed."
                    ),
                    ref=label,
                )
            )

    seen_check_ids: set[str] = set()
    for check in checks:
        if not isinstance(check, dict):
            continue
        check_id = check.get("check_id")
        if check_id in seen_check_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="review_pack_dsl.checks.duplicate_id",
                    message=f"{label} checks contains a duplicate check_id {check_id}",
                    remediation="Check ids must be unique within a review-pack record.",
                    ref=label,
                )
            )
        elif isinstance(check_id, str):
            seen_check_ids.add(check_id)

    seen_scope_ids: set[str] = set()
    for hint in ownership_hints:
        if not isinstance(hint, dict):
            continue
        scope_id = hint.get("ownership_scope_id")
        if scope_id in seen_scope_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="review_pack_dsl.ownership.duplicate_scope_id",
                    message=(
                        f"{label} ownership_hints duplicates ownership_scope_id {scope_id}"
                    ),
                    remediation="Ownership scope ids must be unique within a review-pack record.",
                    ref=label,
                )
            )
        elif isinstance(scope_id, str):
            seen_scope_ids.add(scope_id)

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
    authorities = {record.get("pack_authority_class") for record in records.values()}
    missing_authorities = REQUIRED_AUTHORITIES - authorities
    if missing_authorities:
        findings.append(
            Finding(
                severity="error",
                check_id="fixtures.coverage.authority_missing",
                message="fixtures must cover every required pack_authority_class",
                remediation="Seed review-pack fixtures for each pack authority class.",
                details={"missing": sorted(missing_authorities)},
            )
        )
    parity_classes: set[str] = set()
    for record in records.values():
        for observation in record.get("parity_observations", []):
            cls = observation.get("parity_class") if isinstance(observation, dict) else None
            if cls:
                parity_classes.add(cls)
    missing_parity = REQUIRED_PARITY_CLASSES - parity_classes
    if missing_parity:
        findings.append(
            Finding(
                severity="error",
                check_id="fixtures.coverage.parity_missing",
                message="fixtures must cover every required parity_class",
                remediation="Seed review-pack fixtures covering all four parity classes.",
                details={"missing": sorted(missing_parity)},
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
                check_id="docs.review_pack_dsl.missing",
                message=f"review-pack DSL reviewer doc is missing: {doc_path}",
                remediation="Author docs/review/m3/review_pack_dsl_alpha.md.",
                ref=str(doc_path),
            )
        )
    else:
        for ref in [
            DEFAULT_SCHEMA_REL,
            DEFAULT_FIXTURE_DIR_REL,
            "ci/check_review_pack_dsl_alpha.py",
            DEFAULT_CONSUMER_REL,
            DEFAULT_CRATE_MODULE_REL,
        ]:
            if ref not in doc:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="docs.review_pack_dsl.missing_ref",
                        message=f"review-pack DSL reviewer doc does not mention {ref}",
                        remediation="Document the schema, fixtures, validator, crate module, and first consumer together.",
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
                check_id="consumer.review_pack_dsl.missing",
                message=f"shell review-pack inspector consumer is missing: {consumer_path}",
                remediation=(
                    "Wire the first shell consumer at "
                    "crates/aureline-shell/src/review/review_pack_inspector/mod.rs."
                ),
                ref=str(consumer_path),
            )
        )
    else:
        for token in [
            "build_alpha_review_pack_rows",
            "render_alpha_review_pack_plaintext",
            "project_review_pack",
        ]:
            if token not in consumer:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="consumer.review_pack_dsl.missing_token",
                        message=f"shell review-pack inspector does not contain {token}",
                        remediation="Keep the alpha review-pack family wired into the first shell consumer.",
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
                check_id="crate.review_pack_dsl.missing",
                message=f"aureline-review review_pack_dsl module is missing: {crate_module_path}",
                remediation="Add the review_pack_dsl module to aureline-review.",
                ref=str(crate_module_path),
            )
        )
    else:
        for token in [
            "ReviewPackRecord",
            "project_review_pack",
            "REVIEW_PACK_ALPHA_RECORD_KIND",
            "REVIEW_PACK_PARITY_CLASSES",
            "REVIEW_PACK_AUTHORITY_CLASSES",
        ]:
            if token not in crate_module:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="crate.review_pack_dsl.missing_token",
                        message=f"aureline-review review_pack_dsl module does not export {token}",
                        remediation="Keep the alpha review-pack vocabulary exported from aureline-review.",
                        ref=str(crate_module_path),
                    )
                )
    return findings


def render_gallery(records: dict[str, dict[str, Any]]) -> str:
    lines = [
        "Review-pack DSL alpha gallery",
        "review_pack_id | authority | check_count | blocking | local_only | ci_only | parity_classes",
    ]
    for name in sorted(records):
        record = records[name]
        checks = record.get("checks", [])
        blocking = sum(1 for c in checks if c.get("severity_class") == "blocking")
        local_only = sum(1 for c in checks if c.get("parity_class") == "local_only_documented")
        ci_only = sum(1 for c in checks if c.get("parity_class") == "ci_only_documented")
        parity_classes = sorted(
            {
                o.get("parity_class")
                for o in record.get("parity_observations", [])
                if isinstance(o, dict)
            }
        )
        lines.append(
            " | ".join(
                [
                    record.get("review_pack_id", name),
                    record.get("pack_authority_class", "?"),
                    str(len(checks)),
                    str(blocking),
                    str(local_only),
                    str(ci_only),
                    ",".join(parity_classes) or "none",
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
    records = collect_records(fixture_dir)

    findings: list[Finding] = []
    for name, record in records.items():
        validation_payload = {
            key: value for key, value in record.items() if key != "$schema"
        }
        findings.extend(schema_validate(schema, name, validation_payload))
        findings.extend(cross_check_record(name, record))

    findings.extend(validate_coverage(records))
    findings.extend(validate_doc_and_consumer(doc_path, consumer_path, crate_module_path))

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
