#!/usr/bin/env python3
"""Validate the Beta interface-freeze register.

The Beta API/ops freeze decides, per governed interface surface, whether the
contract is still ``open``, ``soft_frozen`` (additive change expected), or
``hard_frozen`` (locked at a recorded version). Until now that posture was
implicit in scattered version constants -- there was no explicit, gated register,
so a frozen surface could change without anyone recording why.

This gate makes the freeze explicit and enforceable. It reads the checked-in
register at ``artifacts/governance/interface_freeze_register_beta.json`` and the
governed schema-family registry at ``schemas/registry/schema_registry.json`` and:

  - asserts every governed schema family declares a freeze state through a
    governed-source row, so a governed schema that lacks a freeze state is a hard
    failure rather than a silent gap;
  - asserts a ``hard_frozen`` surface cannot move past its frozen version without
    a recorded exception drawn from its allowed exception classes;
  - cross-checks every governed-source row's current version and schema reference
    against the governed registry so the freeze register cannot drift from the
    versions it claims to govern;
  - recomputes the summary block and checks the closed vocabularies; and
  - runs negative drills proving the coverage, hard-freeze, and exception-class
    rejections all fire.

The typed Rust consumer
(``aureline_governance::interface_freeze::current_interface_freeze_register``)
reads the same checked-in register and runs the same cross-check, so this gate
and ``cargo test -p aureline-governance`` agree without a cargo build in CI.
"""

from __future__ import annotations

import argparse
import copy
import dataclasses
import datetime as dt
import json
import sys
from pathlib import Path
from typing import Any


DEFAULT_REGISTER_REL = "artifacts/governance/interface_freeze_register_beta.json"
DEFAULT_SCHEMA_REGISTRY_REL = "schemas/registry/schema_registry.json"
DEFAULT_REPORT_REL = (
    "artifacts/governance/captures/interface_freeze_register_beta_validation_capture.json"
)

EXPECTED_SCHEMA_VERSION = 1
REGISTER_RECORD_KIND = "interface_freeze_register"

SURFACE_CLASSES = (
    "cli_headless",
    "settings_portable_state",
    "extension_sdk_manifest",
    "governed_export_packet",
)
FREEZE_STATE_CLASSES = ("open", "soft_frozen", "hard_frozen")
EXCEPTION_CLASSES = (
    "additive_backward_compatible",
    "security_fix",
    "defect_correction",
    "coordinated_breaking_change",
)
VERSION_SOURCE_CLASSES = ("governed_schema_registry", "declared")


@dataclasses.dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    ref: str

    def as_report(self) -> dict[str, str]:
        return dataclasses.asdict(self)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--register", default=DEFAULT_REGISTER_REL)
    parser.add_argument("--schema-registry", default=DEFAULT_SCHEMA_REGISTRY_REL)
    parser.add_argument("--report", default=None, help="Optional JSON validation capture path.")
    parser.add_argument(
        "--check",
        action="store_true",
        help="CI mode: validate and write the validation capture (default report path).",
    )
    parser.add_argument(
        "--no-capture",
        action="store_true",
        help="Validate without writing the validation capture.",
    )
    return parser.parse_args()


def load_json(path: Path, label: str) -> Any:
    if not path.exists():
        raise SystemExit(f"missing {label}: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"{label} is not valid JSON: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a mapping/object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be an array")
    return value


def generated_at_now() -> str:
    return (
        dt.datetime.now(dt.UTC)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def json_text(payload: dict[str, Any]) -> str:
    return json.dumps(payload, indent=2, sort_keys=True) + "\n"


def is_str(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


def is_version(value: Any) -> bool:
    return isinstance(value, int) and not isinstance(value, bool) and value >= 1


def computed_summary(rows: list[dict[str, Any]]) -> dict[str, int]:
    return {
        "total_rows": len(rows),
        "open_rows": sum(1 for r in rows if r.get("freeze_state") == "open"),
        "soft_frozen_rows": sum(1 for r in rows if r.get("freeze_state") == "soft_frozen"),
        "hard_frozen_rows": sum(1 for r in rows if r.get("freeze_state") == "hard_frozen"),
        "governed_schema_rows": sum(
            1 for r in rows if r.get("version_source") == "governed_schema_registry"
        ),
        "declared_rows": sum(1 for r in rows if r.get("version_source") == "declared"),
        "recorded_exception_count": sum(
            len(r.get("recorded_exceptions", []))
            for r in rows
            if isinstance(r.get("recorded_exceptions"), list)
        ),
    }


def validate_envelope(register: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    register_id = str(register.get("register_id", "<register>"))

    if register.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding("error", "register.schema_version", "register schema_version must be 1", register_id)
        )
    if register.get("record_kind") != REGISTER_RECORD_KIND:
        findings.append(
            Finding("error", "register.record_kind", "register record_kind is not supported", register_id)
        )
    if register.get("governed_schema_registry_ref") != DEFAULT_SCHEMA_REGISTRY_REL:
        findings.append(
            Finding(
                "error",
                "register.governed_ref",
                f"governed_schema_registry_ref must be {DEFAULT_SCHEMA_REGISTRY_REL}",
                register_id,
            )
        )
    for key, expected in (
        ("surface_classes", list(SURFACE_CLASSES)),
        ("freeze_state_classes", list(FREEZE_STATE_CLASSES)),
        ("exception_classes", list(EXCEPTION_CLASSES)),
    ):
        if list(register.get(key, [])) != expected:
            findings.append(
                Finding("error", "register.vocabulary", f"register.{key} is not the closed vocabulary", key)
            )
    return findings


def validate_row(row: dict[str, Any], governed: dict[str, dict[str, Any]]) -> list[Finding]:
    findings: list[Finding] = []
    schema_id = str(row.get("schema_id", "<row>"))

    for field in ("schema_id", "title", "schema_ref", "owner_ref", "rationale"):
        if not is_str(row.get(field)):
            findings.append(
                Finding("error", "row.empty_field", f"row {field} must be a non-empty string", schema_id)
            )

    if row.get("surface_class") not in SURFACE_CLASSES:
        findings.append(
            Finding("error", "row.surface_class_invalid", "row surface_class is outside the vocabulary", schema_id)
        )
    if row.get("freeze_state") not in FREEZE_STATE_CLASSES:
        findings.append(
            Finding(
                "error",
                "row.freeze_state_invalid",
                "every governed surface must carry a freeze state from the closed vocabulary",
                schema_id,
            )
        )
    version_source = row.get("version_source")
    if version_source not in VERSION_SOURCE_CLASSES:
        findings.append(
            Finding("error", "row.version_source_invalid", "row version_source is outside the vocabulary", schema_id)
        )

    frozen_at = row.get("frozen_at_version")
    current = row.get("current_version")
    if not is_version(frozen_at) or not is_version(current):
        findings.append(
            Finding("error", "row.version_invalid", "frozen_at_version and current_version must be integers >= 1", schema_id)
        )
    elif current < frozen_at:
        findings.append(
            Finding("error", "row.version_backwards", "current_version is older than frozen_at_version", schema_id)
        )

    allowed = row.get("allowed_exception_classes", [])
    if not isinstance(allowed, list) or any(item not in EXCEPTION_CLASSES for item in allowed):
        findings.append(
            Finding("error", "row.allowed_classes_invalid", "allowed_exception_classes must be from the vocabulary", schema_id)
        )

    findings.extend(validate_recorded_exceptions(row, allowed))
    findings.extend(validate_against_registry(row, governed))

    # Acceptance core: a hard-frozen surface may not move past its frozen
    # version without a recorded, allowed exception landing at the current
    # version.
    if (
        row.get("freeze_state") == "hard_frozen"
        and is_version(frozen_at)
        and is_version(current)
        and current != frozen_at
        and not change_is_authorized(row)
    ):
        findings.append(
            Finding(
                "error",
                "row.hard_freeze_changed_without_exception",
                "a hard-frozen surface changed version without a recorded exception",
                schema_id,
            )
        )

    return findings


def change_is_authorized(row: dict[str, Any]) -> bool:
    allowed = row.get("allowed_exception_classes", [])
    exceptions = row.get("recorded_exceptions", [])
    if not isinstance(allowed, list) or not isinstance(exceptions, list):
        return False
    current = row.get("current_version")
    return any(
        isinstance(exc, dict)
        and exc.get("to_version") == current
        and exc.get("exception_class") in allowed
        for exc in exceptions
    )


def validate_recorded_exceptions(row: dict[str, Any], allowed: Any) -> list[Finding]:
    findings: list[Finding] = []
    schema_id = str(row.get("schema_id", "<row>"))
    exceptions = row.get("recorded_exceptions")
    if not isinstance(exceptions, list):
        findings.append(
            Finding("error", "row.exceptions_type", "recorded_exceptions must be a list", schema_id)
        )
        return findings

    frozen_at = row.get("frozen_at_version")
    current = row.get("current_version")
    allowed_set = set(allowed) if isinstance(allowed, list) else set()
    for exc in exceptions:
        if not isinstance(exc, dict):
            findings.append(
                Finding("error", "row.exception_shape", "recorded exception must be an object", schema_id)
            )
            continue
        if exc.get("exception_class") not in allowed_set:
            findings.append(
                Finding(
                    "error",
                    "row.exception_class_not_allowed",
                    "a recorded exception uses a class the row does not allow",
                    schema_id,
                )
            )
        frm = exc.get("from_version")
        to = exc.get("to_version")
        if not is_version(frm) or not is_version(to):
            findings.append(
                Finding("error", "row.exception_version_range", "exception versions must be integers >= 1", schema_id)
            )
            continue
        if frm >= to or (is_version(frozen_at) and frm < frozen_at) or (
            is_version(current) and to > current
        ):
            findings.append(
                Finding("error", "row.exception_version_range", "exception version range is invalid for the row", schema_id)
            )
        if not is_str(exc.get("rationale")) or not is_str(exc.get("authority_ref")):
            findings.append(
                Finding("error", "row.exception_metadata", "exception must carry rationale and authority_ref", schema_id)
            )
    return findings


def validate_against_registry(
    row: dict[str, Any], governed: dict[str, dict[str, Any]]
) -> list[Finding]:
    findings: list[Finding] = []
    schema_id = str(row.get("schema_id", "<row>"))
    if row.get("version_source") != "governed_schema_registry":
        return findings
    governed_row = governed.get(schema_id)
    if governed_row is None:
        findings.append(
            Finding(
                "error",
                "row.unknown_governed_schema",
                "governed-source row names a schema absent from the governed registry",
                schema_id,
            )
        )
        return findings
    if row.get("current_version") != governed_row.get("schema_version"):
        findings.append(
            Finding(
                "error",
                "row.registry_version_mismatch",
                f"current_version disagrees with the governed registry "
                f"({row.get('current_version')!r} vs {governed_row.get('schema_version')!r})",
                schema_id,
            )
        )
    if row.get("schema_ref") != governed_row.get("schema_ref"):
        findings.append(
            Finding(
                "error",
                "row.registry_schema_ref_mismatch",
                "schema_ref disagrees with the governed registry",
                schema_id,
            )
        )
    return findings


def validate_coverage(
    rows: list[dict[str, Any]], governed: dict[str, dict[str, Any]]
) -> list[Finding]:
    findings: list[Finding] = []
    covered = {
        str(row.get("schema_id"))
        for row in rows
        if row.get("version_source") == "governed_schema_registry"
    }
    for schema_id in sorted(governed):
        if schema_id not in covered:
            findings.append(
                Finding(
                    "error",
                    "coverage.missing_freeze_state",
                    "governed schema family has no freeze state in the register",
                    schema_id,
                )
            )
    return findings


def validate_schema_refs(rows: list[dict[str, Any]], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    for row in rows:
        schema_ref = row.get("schema_ref")
        if not is_str(schema_ref):
            continue
        if not (repo_root / schema_ref).exists():
            findings.append(
                Finding(
                    "error",
                    "row.schema_ref_missing",
                    f"schema_ref file does not exist: {schema_ref}",
                    str(row.get("schema_id", "<row>")),
                )
            )
    return findings


def validate_summary(register: dict[str, Any], rows: list[dict[str, Any]]) -> list[Finding]:
    findings: list[Finding] = []
    summary = register.get("summary")
    if not isinstance(summary, dict):
        findings.append(
            Finding("error", "summary.missing", "register must carry a summary block", str(register.get("register_id")))
        )
        return findings
    expected = computed_summary(rows)
    for key, value in expected.items():
        if summary.get(key) != value:
            findings.append(
                Finding("error", "summary.count_mismatch", f"summary.{key} must equal {value}", key)
            )
    return findings


def validate_register(
    register: dict[str, Any], governed: dict[str, dict[str, Any]], repo_root: Path
) -> list[Finding]:
    findings = validate_envelope(register)
    rows = register.get("rows")
    if not isinstance(rows, list) or not rows:
        findings.append(
            Finding("error", "register.rows_empty", "register must enumerate at least one row", str(register.get("register_id")))
        )
        return findings

    seen: set[str] = set()
    for raw in rows:
        row = ensure_dict(raw, "register.rows[]")
        findings.extend(validate_row(row, governed))
        schema_id = str(row.get("schema_id", "<row>"))
        if schema_id in seen:
            findings.append(Finding("error", "row.duplicate_id", "schema ids must be unique", schema_id))
        seen.add(schema_id)

    findings.extend(validate_coverage(rows, governed))
    findings.extend(validate_schema_refs(rows, repo_root))
    findings.extend(validate_summary(register, rows))
    return findings


def index_governed(schema_registry: dict[str, Any]) -> dict[str, dict[str, Any]]:
    rows = ensure_list(schema_registry.get("rows"), "schema_registry.rows")
    index: dict[str, dict[str, Any]] = {}
    for raw in rows:
        row = ensure_dict(raw, "schema_registry.rows[]")
        schema_id = row.get("schema_id")
        if isinstance(schema_id, str) and schema_id:
            index[schema_id] = row
    return index


def run_negative_drills(
    register: dict[str, Any], governed: dict[str, dict[str, Any]], repo_root: Path
) -> tuple[list[dict[str, str]], list[Finding]]:
    results: list[dict[str, str]] = []
    findings: list[Finding] = []

    def record(drill_id: str, expected_check_id: str, fired: bool) -> None:
        results.append(
            {
                "drill_id": drill_id,
                "expected_check_id": expected_check_id,
                "status": "passed" if fired else "failed",
            }
        )
        if not fired:
            findings.append(
                Finding(
                    "error",
                    "negative_drill.not_rejected",
                    f"negative drill {drill_id} did not fire",
                    drill_id,
                )
            )

    # A governed family that loses its freeze row must be flagged.
    mutated = copy.deepcopy(register)
    governed_index = next(
        (
            i
            for i, row in enumerate(mutated["rows"])
            if row.get("version_source") == "governed_schema_registry"
        ),
        None,
    )
    if governed_index is not None:
        mutated["rows"].pop(governed_index)
        mutated["summary"] = computed_summary(mutated["rows"])
        check_ids = {f.check_id for f in validate_register(mutated, governed, repo_root)}
        record("missing_freeze_state_rejected", "coverage.missing_freeze_state",
               "coverage.missing_freeze_state" in check_ids)

    # A hard-frozen surface that changes version with no exception must be flagged.
    mutated = copy.deepcopy(register)
    hard_declared = next(
        (
            row
            for row in mutated["rows"]
            if row.get("freeze_state") == "hard_frozen" and row.get("version_source") == "declared"
        ),
        None,
    )
    if hard_declared is not None:
        hard_declared["current_version"] = int(hard_declared["frozen_at_version"]) + 1
        hard_declared["recorded_exceptions"] = []
        mutated["summary"] = computed_summary(mutated["rows"])
        check_ids = {f.check_id for f in validate_register(mutated, governed, repo_root)}
        record(
            "hard_freeze_change_rejected",
            "row.hard_freeze_changed_without_exception",
            "row.hard_freeze_changed_without_exception" in check_ids,
        )

    # A recorded exception of a class the row does not allow must be flagged.
    mutated = copy.deepcopy(register)
    target = mutated["rows"][0]
    allowed = set(target.get("allowed_exception_classes", []))
    disallowed = next((c for c in EXCEPTION_CLASSES if c not in allowed), None)
    if disallowed is not None:
        target["current_version"] = int(target["frozen_at_version"]) + 1
        target["recorded_exceptions"] = [
            {
                "exception_class": disallowed,
                "from_version": int(target["frozen_at_version"]),
                "to_version": int(target["current_version"]),
                "rationale": "Negative drill.",
                "authority_ref": "decision.drill",
            }
        ]
        mutated["summary"] = computed_summary(mutated["rows"])
        check_ids = {f.check_id for f in validate_register(mutated, governed, repo_root)}
        record("disallowed_exception_class_rejected", "row.exception_class_not_allowed",
               "row.exception_class_not_allowed" in check_ids)

    return results, findings


def write_report(
    path: Path,
    register: dict[str, Any],
    findings: list[Finding],
    drill_results: list[dict[str, str]],
) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": "interface_freeze_register_validation_capture",
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": generated_at_now(),
        "register_id": register.get("register_id"),
        "summary": register.get("summary"),
        "negative_drills": drill_results,
        "findings": [f.as_report() for f in findings],
    }
    path.write_text(json_text(payload), encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    register = ensure_dict(load_json(repo_root / args.register, "freeze register"), "freeze register")
    schema_registry = ensure_dict(
        load_json(repo_root / args.schema_registry, "governed schema registry"),
        "governed schema registry",
    )
    governed = index_governed(schema_registry)

    findings = validate_register(register, governed, repo_root)
    drill_results, drill_findings = run_negative_drills(register, governed, repo_root)
    findings.extend(drill_findings)

    report_rel = args.report
    if report_rel is None and not args.no_capture:
        report_rel = DEFAULT_REPORT_REL
    if report_rel:
        write_report(repo_root / report_rel, register, findings, drill_results)

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        for item in errors:
            print(f"ERROR [{item.check_id}] ({item.ref}): {item.message}", file=sys.stderr)
        return 1

    summary = register.get("summary", {})
    print(
        "beta interface-freeze register validated "
        f"({summary.get('total_rows')} rows, "
        f"{summary.get('hard_frozen_rows')} hard-frozen, "
        f"{summary.get('soft_frozen_rows')} soft-frozen, "
        f"{summary.get('open_rows')} open; "
        f"{summary.get('governed_schema_rows')} governed families covered; "
        f"{len(drill_results)} negative drills)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
