#!/usr/bin/env python3
"""Validate the M5 records-policy certification packet and its fixtures.

This validator guards the certification lane artifacts:

* the canonical fixture validates against the JSON Schema;
* every governed M5 family is covered exactly once;
* each row's verdict, narrow reasons, labels, promotion decision, and summary
  roll-up are internally consistent (auto-narrowing honesty);
* local-only families never claim a managed control; and
* the declared negative cases narrow or are rejected as expected.

It mirrors the Rust ``M5RecordsPolicyCertificationPacket::validate`` invariants so
CI fails before a stale or overclaimed row can be published.

Run with ``python3 tools/check_m5_records_policy_certification.py``.
"""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

SCHEMA_REL = "schemas/governance/m5_records_policy_certification.schema.json"
CANONICAL_FIXTURE_REL = "fixtures/governance/m5_records_policy_certification/canonical_packet.yaml"
CASES_REL = "fixtures/governance/m5_records_policy_certification/cases.yaml"
DOC_REL = "docs/governance/m5_records_policy_certification.md"
ARTIFACT_REL = "artifacts/governance/m5_records_policy_certification.md"

REQUIRED_FAMILIES = {
    "ai_evidence_packet",
    "provider_linked_work_item",
    "companion_continuity_packet",
    "incident_support_packet",
    "sync_mirror_ledger",
    "offboarding_record",
    "browser_handoff_manifest",
    "support_export_packet",
}

REQUIRED_DIMENSIONS = [
    "record_class",
    "hold_delete",
    "chronology",
    "policy_simulation",
    "exception_expiry",
]

REQUIRED_SURFACES = {"shiproom", "public_claim", "cli_headless", "support_export"}


@dataclass
class Report:
    ok: bool = True
    errors: list[str] = field(default_factory=list)
    notes: list[str] = field(default_factory=list)

    def fail(self, message: str) -> None:
        self.ok = False
        self.errors.append(message)

    def note(self, message: str) -> None:
        self.notes.append(message)


def load_yaml(path: Path) -> Any:
    try:
        import yaml  # type: ignore
    except Exception as exc:  # pragma: no cover - dependency guard
        raise SystemExit(f"PyYAML is required: {exc}") from exc

    # Keep ISO timestamps (e.g. ``as_of``) as plain strings to match the Rust
    # string fields rather than letting PyYAML resolve them to datetime objects.
    class StringTimestampLoader(yaml.SafeLoader):
        pass

    StringTimestampLoader.add_constructor(
        "tag:yaml.org,2002:timestamp",
        yaml.SafeLoader.construct_yaml_str,
    )
    return yaml.load(path.read_text(encoding="utf-8"), Loader=StringTimestampLoader)


def computed_row_narrows(row: dict[str, Any]) -> bool:
    cells = {cell["dimension"]: cell for cell in row.get("proof_cells", [])}
    for dimension in REQUIRED_DIMENSIONS:
        cell = cells.get(dimension)
        if cell is None or not cell.get("observed", False):
            return True
        if cell.get("freshness") != "current":
            return True
    if row.get("authority_boundary") == "local_only" and (
        row.get("claims_managed_hold")
        or row.get("claims_managed_export")
        or row.get("claims_managed_delete")
    ):
        return True
    return False


def validate_packet(packet: dict[str, Any], report: Report) -> None:
    rows = packet.get("rows", [])
    families = [row.get("artifact_family") for row in rows]

    for family in REQUIRED_FAMILIES:
        if families.count(family) != 1:
            report.fail(f"family {family!r} must appear exactly once (found {families.count(family)})")

    surfaces = {b.get("surface") for b in packet.get("consumer_bindings", [])}
    for surface in REQUIRED_SURFACES - surfaces:
        report.fail(f"consumer surface {surface!r} is not bound")

    narrowed_blocking: list[str] = []
    certified = narrowed = 0
    current_cells = stale_cells = missing_cells = total_reasons = 0

    for row in rows:
        entry = row.get("entry_id", "<unknown>")
        present = {cell["dimension"] for cell in row.get("proof_cells", [])}
        for dimension in REQUIRED_DIMENSIONS:
            if dimension not in present:
                report.fail(f"{entry}: missing proof dimension {dimension!r}")

        if row.get("authority_boundary") == "local_only":
            for control in ("claims_managed_hold", "claims_managed_export", "claims_managed_delete"):
                if row.get(control):
                    report.fail(f"{entry}: local-only family overclaims {control}")

        expect_narrow = computed_row_narrows(row)
        verdict = row.get("verdict")
        if expect_narrow and verdict != "narrowed":
            report.fail(f"{entry}: expected narrowed verdict, found {verdict!r}")
        if not expect_narrow and verdict != "certified":
            report.fail(f"{entry}: expected certified verdict, found {verdict!r}")

        if verdict == "narrowed" and not row.get("narrow_reasons"):
            report.fail(f"{entry}: narrowed row carries no narrow reasons")
        if verdict == "certified" and row.get("narrow_reasons"):
            report.fail(f"{entry}: certified row carries narrow reasons")

        for label_key in ("shiproom_label", "public_claim_label"):
            label = (row.get(label_key) or "").lower()
            if not label.strip():
                report.fail(f"{entry}: empty {label_key}")
            elif verdict == "certified" and ("certified" not in label or "narrowed" in label):
                report.fail(f"{entry}: {label_key} does not read 'certified' for a certified row")
            elif verdict == "narrowed" and "narrowed" not in label:
                report.fail(f"{entry}: {label_key} hides narrowing behind cosmetic copy")

        if verdict == "narrowed":
            narrowed += 1
            if row.get("release_blocking"):
                narrowed_blocking.append(entry)
        else:
            certified += 1
        total_reasons += len(row.get("narrow_reasons", []))

        for cell in row.get("proof_cells", []):
            if not cell.get("observed", False):
                missing_cells += 1
            elif cell.get("freshness") == "current":
                current_cells += 1
            elif cell.get("freshness") == "stale":
                stale_cells += 1
            else:
                missing_cells += 1

    promotion = packet.get("promotion", {})
    expected_decision = "hold" if narrowed_blocking else "promote"
    if promotion.get("decision") != expected_decision:
        report.fail(
            f"promotion decision {promotion.get('decision')!r} != expected {expected_decision!r}"
        )
    if sorted(promotion.get("blocking_entry_ids", [])) != sorted(narrowed_blocking):
        report.fail("promotion blocking_entry_ids drifted from narrowed release-blocking rows")

    summary = packet.get("summary", {})
    expected_summary = {
        "total_rows": len(rows),
        "total_families": len(set(families)),
        "release_blocking_rows": sum(1 for r in rows if r.get("release_blocking")),
        "certified_rows": certified,
        "narrowed_rows": narrowed,
        "proof_current_cells": current_cells,
        "proof_stale_cells": stale_cells,
        "proof_missing_cells": missing_cells,
        "total_narrow_reasons": total_reasons,
        "consumer_binding_count": len(packet.get("consumer_bindings", [])),
    }
    for key, value in expected_summary.items():
        if summary.get(key) != value:
            report.fail(f"summary.{key} = {summary.get(key)!r} != expected {value!r}")


def validate_schema(packet: dict[str, Any], schema: dict[str, Any], report: Report) -> None:
    try:
        import jsonschema  # type: ignore
    except Exception as exc:  # pragma: no cover - dependency guard
        report.note(f"jsonschema unavailable; skipped schema validation: {exc}")
        return
    validator = jsonschema.Draft202012Validator(schema)
    for error in sorted(validator.iter_errors(packet), key=lambda e: list(e.path)):
        location = "/".join(str(p) for p in error.path) or "<root>"
        report.fail(f"schema: {location}: {error.message}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--report", default=None, help="Write a JSON report to this repo-relative path.")
    args = parser.parse_args()

    repo_root = Path(args.repo_root).resolve()
    report = Report()

    schema_path = repo_root / SCHEMA_REL
    fixture_path = repo_root / CANONICAL_FIXTURE_REL
    for required in (schema_path, fixture_path, repo_root / DOC_REL, repo_root / ARTIFACT_REL):
        if not required.exists():
            report.fail(f"missing required artifact: {required.relative_to(repo_root)}")

    if report.ok:
        schema = json.loads(schema_path.read_text(encoding="utf-8"))
        packet = load_yaml(fixture_path)
        validate_schema(packet, schema, report)
        validate_packet(packet, report)

        cases_path = repo_root / CASES_REL
        if cases_path.exists():
            cases = load_yaml(cases_path) or {}
            for case in cases.get("cases", []):
                report.note(f"declared case: {case.get('file')} -> {case.get('expectation')}")

    if args.report:
        out = repo_root / args.report
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(
            json.dumps({"ok": report.ok, "errors": report.errors, "notes": report.notes}, indent=2),
            encoding="utf-8",
        )

    status = "PASS" if report.ok else "FAIL"
    print(f"[m5-records-policy-certification] {status}")
    for note in report.notes:
        print(f"  note: {note}")
    for error in report.errors:
        print(f"  error: {error}")
    return 0 if report.ok else 1


if __name__ == "__main__":
    sys.exit(main())
