#!/usr/bin/env python3
"""Validate archive-search boundary and destruction-receipt alpha fixtures."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


BOUNDARY_REL = "artifacts/governance/archive_redaction_boundary_alpha.yaml"
RECEIPT_SCHEMA_REL = "schemas/governance/destruction_receipt_alpha.schema.json"
RECEIPT_MANIFEST_REL = "fixtures/governance/destruction_receipt_alpha/manifest.yaml"
ADMIN_MANIFEST_REL = "fixtures/admin/archive_search_boundary_alpha/manifest.yaml"

REQUIRED_RESULT_CLASSES = {
    "completed",
    "partial",
    "blocked_by_hold",
    "policy_retained",
    "outside_platform_scope",
    "manual_local_capture_required",
    "omitted_by_redaction",
}

REQUIRED_LOCALITY_CLASSES = {
    "local_only",
    "managed_copy",
    "archived",
    "held",
    "receipt_only",
    "outside_platform_scope",
    "redacted_boundary",
}

REQUIRED_RECEIPT_STATES = {
    "available",
    "pending_after_hold_clear",
    "pending_policy_floor",
    "not_available_outside_scope",
    "manual_local_action_required",
    "omitted_by_redaction",
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
    parser.add_argument("--report", default=None)
    return parser.parse_args()


def render_yaml_as_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing YAML file: {path}")
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-e",
            (
                "payload = YAML.safe_load(File.read(ARGV[0]), permitted_classes: [], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML at {path}: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {path}: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be an object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a list")
    return value


def path_exists(repo_root: Path, ref: str) -> bool:
    clean = ref.split("#", 1)[0]
    return bool(clean) and (repo_root / clean).exists()


def require_path(repo_root: Path, ref: str, findings: list[Finding], check_id: str) -> None:
    if "/" in ref and not path_exists(repo_root, ref):
        findings.append(
            Finding(
                severity="error",
                check_id=check_id,
                message=f"path reference does not resolve: {ref}",
                remediation="Fix the path or add the referenced artifact.",
                ref=ref,
            )
        )


def validate_boundary(repo_root: Path, boundary: dict[str, Any], findings: list[Finding]) -> None:
    if boundary.get("schema_version") != 1:
        findings.append(
            Finding(
                "error",
                "boundary.schema_version",
                "archive boundary schema_version must be 1",
                "Restore schema_version: 1 or update this validator with the new shape.",
                BOUNDARY_REL,
            )
        )

    for field_name in (
        "human_contract_ref",
        "destruction_receipt_schema_ref",
        "record_class_registry_ref",
        "record_state_contract_ref",
        "retention_deletion_contract_ref",
        "usage_export_offboarding_contract_ref",
    ):
        ref = str(boundary.get(field_name, ""))
        require_path(repo_root, ref, findings, f"boundary.{field_name}.missing")

    observed_results = {
        str(row.get("result_class"))
        for row in ensure_list(boundary.get("result_vocabulary"), "boundary.result_vocabulary")
        if isinstance(row, dict)
    }
    missing_results = REQUIRED_RESULT_CLASSES - observed_results
    if missing_results:
        findings.append(
            Finding(
                "error",
                "boundary.result_vocabulary.missing",
                "archive boundary is missing required result classes",
                "Add the required result classes to artifacts/governance/archive_redaction_boundary_alpha.yaml.",
                BOUNDARY_REL,
                {"missing": sorted(missing_results)},
            )
        )

    observed_localities = {
        str(row.get("locality_class"))
        for row in ensure_list(boundary.get("locality_vocabulary"), "boundary.locality_vocabulary")
        if isinstance(row, dict)
    }
    missing_localities = REQUIRED_LOCALITY_CLASSES - observed_localities
    if missing_localities:
        findings.append(
            Finding(
                "error",
                "boundary.locality_vocabulary.missing",
                "archive boundary is missing required locality classes",
                "Add every locality class required by admin and support review.",
                BOUNDARY_REL,
                {"missing": sorted(missing_localities)},
            )
        )

    rows = ensure_list(boundary.get("boundary_rows"), "boundary.boundary_rows")
    if not rows:
        findings.append(
            Finding(
                "error",
                "boundary.rows.empty",
                "archive boundary must declare boundary rows",
                "Add at least one row for record, redaction, search, and receipt posture.",
                BOUNDARY_REL,
            )
        )
    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"boundary.boundary_rows[{idx}]")
        row_ref = row.get("row_id", f"{BOUNDARY_REL}#{idx}")
        for field_name in ("record_class_id", "record_class_ref", "retention_owner_ref", "locality_class"):
            if not row.get(field_name):
                findings.append(
                    Finding(
                        "error",
                        f"boundary.rows.{field_name}.missing",
                        f"boundary row is missing {field_name}",
                        "Populate record class, retention owner, locality, and redaction/search posture.",
                        str(row_ref),
                    )
                )
        search = ensure_dict(row.get("archive_search"), f"boundary row {row_ref}.archive_search")
        for field_name in ("searchable", "exportable", "receipt_only", "coverage_summary"):
            if field_name not in search or search.get(field_name) in ("", None):
                findings.append(
                    Finding(
                        "error",
                        f"boundary.rows.archive_search.{field_name}.missing",
                        f"archive-search posture is missing {field_name}",
                        "Declare searchability, exportability, receipt-only posture, and coverage summary.",
                        str(row_ref),
                    )
                )
        redaction = ensure_dict(row.get("redaction_boundary"), f"boundary row {row_ref}.redaction_boundary")
        if not redaction.get("raw_payloads_excluded"):
            findings.append(
                Finding(
                    "error",
                    "boundary.rows.redaction.raw_payloads",
                    "redaction boundary must exclude raw payloads",
                    "Set raw_payloads_excluded true and list omitted data classes.",
                    str(row_ref),
                )
            )
        require_path(repo_root, str(row.get("record_class_ref", "")), findings, "boundary.rows.record_class_ref.missing")


def schema_validator(schema_path: Path):
    try:
        from jsonschema import Draft202012Validator, FormatChecker  # type: ignore
    except Exception as exc:  # pragma: no cover
        raise SystemExit(f"python jsonschema is required: {exc}") from exc
    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    return Draft202012Validator(schema, format_checker=FormatChecker())


def validate_receipt_payload(
    payload: dict[str, Any],
    fixture_ref: str,
    expected_result: str,
    expected_state: str,
    findings: list[Finding],
) -> None:
    if payload.get("result_class") != expected_result:
        findings.append(
            Finding(
                "error",
                "receipt.fixture.result_mismatch",
                "receipt fixture result_class does not match manifest",
                "Align the manifest case and fixture result_class.",
                fixture_ref,
                {"expected": expected_result, "actual": payload.get("result_class")},
            )
        )
    if payload.get("receipt_state") != expected_state:
        findings.append(
            Finding(
                "error",
                "receipt.fixture.state_mismatch",
                "receipt fixture receipt_state does not match manifest",
                "Align the manifest case and fixture receipt_state.",
                fixture_ref,
                {"expected": expected_state, "actual": payload.get("receipt_state")},
            )
        )

    if expected_state == "available":
        if not payload.get("emitted_receipt_ref") or not payload.get("executed_at"):
            findings.append(
                Finding(
                    "error",
                    "receipt.available.requires_receipt_and_time",
                    "available receipts must carry emitted_receipt_ref and executed_at",
                    "Add the emitted receipt ref and timezone-aware execution chronology.",
                    fixture_ref,
                )
            )
    else:
        if payload.get("emitted_receipt_ref") is not None or payload.get("executed_at") is not None:
            findings.append(
                Finding(
                    "error",
                    "receipt.unavailable.no_execution_claim",
                    "non-available receipt disclosures must not claim an emitted receipt or execution time",
                    "Set emitted_receipt_ref and executed_at to null until destruction executes.",
                    fixture_ref,
                )
            )

    counts = ensure_dict(payload.get("artifact_counts"), f"{fixture_ref}.artifact_counts")
    count_expectations = {
        "destroyed_ref_count": len(ensure_list(payload.get("destroyed_refs"), f"{fixture_ref}.destroyed_refs")),
        "retained_ref_count": len(ensure_list(payload.get("retained_refs"), f"{fixture_ref}.retained_refs")),
        "skipped_held_ref_count": len(ensure_list(payload.get("skipped_held_refs"), f"{fixture_ref}.skipped_held_refs")),
        "outside_scope_ref_count": len(ensure_list(payload.get("outside_scope_refs"), f"{fixture_ref}.outside_scope_refs")),
        "manual_local_capture_count": len(ensure_list(payload.get("manual_local_capture_refs"), f"{fixture_ref}.manual_local_capture_refs")),
        "omitted_by_redaction_count": len(ensure_list(payload.get("omitted_by_redaction_refs"), f"{fixture_ref}.omitted_by_redaction_refs")),
    }
    for field_name, expected in count_expectations.items():
        if counts.get(field_name) != expected:
            findings.append(
                Finding(
                    "error",
                    f"receipt.counts.{field_name}",
                    f"{field_name} does not match the listed refs",
                    "Update artifact_counts so it is mechanically checkable.",
                    fixture_ref,
                    {"expected": expected, "actual": counts.get(field_name)},
                )
            )

    coverage = ensure_dict(payload.get("receipt_coverage"), f"{fixture_ref}.receipt_coverage")
    if not coverage.get("covers_full_requested_scope"):
        if not coverage.get("unavailable_reason") or not coverage.get("partial_reason_classes"):
            findings.append(
                Finding(
                    "error",
                    "receipt.coverage.partial_reason_missing",
                    "partial or unavailable receipt coverage must name reason classes",
                    "Add unavailable_reason and partial_reason_classes.",
                    fixture_ref,
                )
            )

    required_ref_list_by_result = {
        "completed": "destroyed_refs",
        "partial": "retained_refs",
        "blocked_by_hold": "skipped_held_refs",
        "policy_retained": "retained_refs",
        "outside_platform_scope": "outside_scope_refs",
        "manual_local_capture_required": "manual_local_capture_refs",
        "omitted_by_redaction": "omitted_by_redaction_refs",
    }
    list_name = required_ref_list_by_result.get(expected_result)
    if list_name and not payload.get(list_name):
        findings.append(
            Finding(
                "error",
                f"receipt.result.{expected_result}.refs_missing",
                f"{expected_result} receipt disclosure must list {list_name}",
                "Add the refs that explain the result class.",
                fixture_ref,
            )
        )

    custody = ensure_dict(payload.get("chain_of_custody"), f"{fixture_ref}.chain_of_custody")
    if not custody.get("verifier_refs"):
        findings.append(
            Finding(
                "error",
                "receipt.custody.verifier_missing",
                "receipt record must list verifier refs",
                "Add verifier refs so support can reconstruct custody.",
                fixture_ref,
            )
        )
    mirror = ensure_dict(payload.get("mirror_or_lag"), f"{fixture_ref}.mirror_or_lag")
    if not mirror.get("mirror_or_lag_note"):
        findings.append(
            Finding(
                "error",
                "receipt.mirror_or_lag.note_missing",
                "receipt record must carry a mirror or lag note",
                "Add a note even when no lag exists.",
                fixture_ref,
            )
        )


def validate_receipts(repo_root: Path, findings: list[Finding]) -> set[str]:
    manifest_path = repo_root / RECEIPT_MANIFEST_REL
    manifest = ensure_dict(render_yaml_as_json(manifest_path), RECEIPT_MANIFEST_REL)
    validator = schema_validator(repo_root / RECEIPT_SCHEMA_REL)
    receipt_ids: set[str] = set()
    observed_results: set[str] = set()

    for raw_case in ensure_list(manifest.get("cases"), "receipt manifest cases"):
        case = ensure_dict(raw_case, "receipt manifest case")
        fixture_name = str(case.get("fixture", ""))
        fixture_path = manifest_path.parent / fixture_name
        payload = ensure_dict(render_yaml_as_json(fixture_path), str(fixture_path))

        errors = sorted(validator.iter_errors(payload), key=lambda err: list(err.path))
        for error in errors[:5]:
            findings.append(
                Finding(
                    "error",
                    "receipt.schema.invalid",
                    error.message,
                    "Update the fixture or the destruction receipt schema in the same change.",
                    str(fixture_path.relative_to(repo_root)),
                    {"path": [str(part) for part in error.path]},
                )
            )

        expected_result = str(case.get("result_class"))
        expected_state = str(case.get("receipt_state"))
        validate_receipt_payload(
            payload,
            str(fixture_path.relative_to(repo_root)),
            expected_result,
            expected_state,
            findings,
        )
        receipt_ids.add(str(payload.get("receipt_record_id")))
        emitted = payload.get("emitted_receipt_ref")
        if emitted:
            receipt_ids.add(str(emitted))
        observed_results.add(expected_result)

    missing_results = REQUIRED_RESULT_CLASSES - observed_results
    if missing_results:
        findings.append(
            Finding(
                "error",
                "receipt.fixtures.required_results_missing",
                "destruction receipt fixtures do not cover the required result classes",
                "Add fixtures for every required result class.",
                RECEIPT_MANIFEST_REL,
                {"missing": sorted(missing_results)},
            )
        )
    return receipt_ids


def validate_admin_fixture(repo_root: Path, receipt_ids: set[str], findings: list[Finding]) -> None:
    manifest_path = repo_root / ADMIN_MANIFEST_REL
    manifest = ensure_dict(render_yaml_as_json(manifest_path), ADMIN_MANIFEST_REL)
    for raw_case in ensure_list(manifest.get("cases"), "admin manifest cases"):
        case = ensure_dict(raw_case, "admin manifest case")
        fixture_name = str(case.get("fixture", ""))
        fixture_path = manifest_path.parent / fixture_name
        payload = ensure_dict(render_yaml_as_json(fixture_path), str(fixture_path))

        vocabulary = set(ensure_list(payload.get("result_vocabulary"), f"{fixture_name}.result_vocabulary"))
        missing_results = REQUIRED_RESULT_CLASSES - vocabulary
        if missing_results:
            findings.append(
                Finding(
                    "error",
                    "admin.result_vocabulary.missing",
                    "admin archive boundary fixture is missing required result classes",
                    "Add every result token to the admin fixture result_vocabulary.",
                    str(fixture_path.relative_to(repo_root)),
                    {"missing": sorted(missing_results)},
                )
            )

        rows = ensure_list(payload.get("review_rows"), f"{fixture_name}.review_rows")
        observed_results = {str(row.get("result_class")) for row in rows if isinstance(row, dict)}
        observed_localities = {str(row.get("locality_class")) for row in rows if isinstance(row, dict)}
        observed_states = {str(row.get("receipt_state")) for row in rows if isinstance(row, dict)}

        for expected, observed, label in (
            (set(case.get("required_result_classes", [])), observed_results, "result"),
            (set(case.get("required_locality_classes", [])), observed_localities, "locality"),
            (set(case.get("required_receipt_states", [])), observed_states, "receipt_state"),
        ):
            missing = expected - observed
            if missing:
                findings.append(
                    Finding(
                        "error",
                        f"admin.{label}.missing",
                        f"admin review fixture is missing required {label} values",
                        "Add review rows that exercise the required boundary values.",
                        str(fixture_path.relative_to(repo_root)),
                        {"missing": sorted(missing)},
                    )
                )

        for raw_row in rows:
            row = ensure_dict(raw_row, "admin review row")
            row_ref = str(row.get("row_id"))
            for field_name in (
                "record_class_id",
                "result_class",
                "retention_owner_ref",
                "locality_class",
                "redaction_boundary_ref",
                "receipt_state",
                "result_summary",
            ):
                if not row.get(field_name):
                    findings.append(
                        Finding(
                            "error",
                            f"admin.row.{field_name}.missing",
                            f"admin review row is missing {field_name}",
                            "Populate record class, result, owner, locality, redaction, receipt, and summary fields.",
                            row_ref,
                        )
                    )
            for bool_field in ("searchable", "exportable", "receipt_only"):
                if not isinstance(row.get(bool_field), bool):
                    findings.append(
                        Finding(
                            "error",
                            f"admin.row.{bool_field}.missing",
                            f"admin review row must declare boolean {bool_field}",
                            "Set searchable, exportable, and receipt_only explicitly.",
                            row_ref,
                        )
                    )
            if not row.get("chain_of_custody_refs"):
                findings.append(
                    Finding(
                        "error",
                        "admin.row.chain_of_custody_refs.missing",
                        "admin review row must carry chain-of-custody refs",
                        "Add audit, receipt, hold, or verifier refs.",
                        row_ref,
                    )
                )
            unknown_receipts = [
                receipt
                for receipt in ensure_list(row.get("destruction_receipt_record_refs"), f"{row_ref}.destruction_receipt_record_refs")
                if receipt not in receipt_ids
            ]
            if unknown_receipts:
                findings.append(
                    Finding(
                        "error",
                        "admin.row.receipt_ref_unknown",
                        "admin review row references unknown destruction receipt fixture ids",
                        "Add matching receipt fixtures or correct the refs.",
                        row_ref,
                        {"unknown": unknown_receipts},
                    )
                )


def render_summary(findings: list[Finding]) -> str:
    errors = [finding for finding in findings if finding.severity == "error"]
    lines = [
        "[archive-destruction-alpha] validation complete",
        f"errors: {len(errors)}",
    ]
    for finding in errors[:20]:
        ref = f" ({finding.ref})" if finding.ref else ""
        lines.append(f"- {finding.check_id}{ref}: {finding.message}")
    return "\n".join(lines) + "\n"


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    findings: list[Finding] = []

    for rel in (BOUNDARY_REL, RECEIPT_SCHEMA_REL, RECEIPT_MANIFEST_REL, ADMIN_MANIFEST_REL):
        if not (repo_root / rel).exists():
            findings.append(
                Finding(
                    "error",
                    "required_artifact.missing",
                    f"required artifact is missing: {rel}",
                    "Seed the archive boundary, receipt schema, manifests, and fixtures.",
                    rel,
                )
            )
    if not findings:
        boundary = ensure_dict(render_yaml_as_json(repo_root / BOUNDARY_REL), BOUNDARY_REL)
        validate_boundary(repo_root, boundary, findings)
        receipt_ids = validate_receipts(repo_root, findings)
        validate_admin_fixture(repo_root, receipt_ids, findings)

    sys.stdout.write(render_summary(findings))

    if args.report:
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(
            json.dumps(
                {
                    "findings": [finding.as_report() for finding in findings],
                    "artifact_refs": {
                        "boundary": BOUNDARY_REL,
                        "receipt_schema": RECEIPT_SCHEMA_REL,
                        "receipt_manifest": RECEIPT_MANIFEST_REL,
                        "admin_manifest": ADMIN_MANIFEST_REL,
                    },
                },
                indent=2,
                sort_keys=True,
            )
            + "\n",
            encoding="utf-8",
        )

    return 1 if any(finding.severity == "error" for finding in findings) else 0


if __name__ == "__main__":
    raise SystemExit(main())
