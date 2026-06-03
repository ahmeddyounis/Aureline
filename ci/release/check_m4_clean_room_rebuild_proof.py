#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Check the clean-room rebuild proof artifact."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

ARTIFACT_PATH = Path("artifacts/release/m4/clean-room-rebuild-proof.json")
HOLDING_STATES = {"current", "on_waiver"}


def load_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        payload = json.load(handle)
    if not isinstance(payload, dict):
        raise SystemExit(f"artifact must be a JSON object: {path}")
    return payload


def compute_holds_claim(row: dict[str, Any]) -> bool:
    return row.get("family_state") in HOLDING_STATES and row.get("effective_label") == row.get("claim_label")


def rule_fires(rule: dict[str, Any], rows: list[dict[str, Any]]) -> bool:
    applies_to_states = set(rule.get("applies_to_states") or [])
    trigger_reason = rule.get("trigger_reason")
    return any(
        row.get("family_state") in applies_to_states
        and trigger_reason in (row.get("active_gap_reasons") or [])
        for row in rows
    )


def compute_summary(rows: list[dict[str, Any]], rules: list[dict[str, Any]]) -> dict[str, int]:
    return {
        "total_rows": len(rows),
        "rows_holding_claim": sum(1 for row in rows if compute_holds_claim(row)),
        "rows_narrowed": sum(1 for row in rows if not compute_holds_claim(row)),
        "rows_on_active_waiver": sum(1 for row in rows if row.get("family_state") == "on_waiver"),
        "total_active_gap_reasons": sum(len(row.get("active_gap_reasons") or []) for row in rows),
        "rules_firing": sum(1 for rule in rules if rule_fires(rule, rows)),
    }


def compute_blocking_rule_ids(rows: list[dict[str, Any]], rules: list[dict[str, Any]]) -> list[str]:
    return sorted(
        rule["rule_id"]
        for rule in rules
        if rule.get("blocks_publication") and rule_fires(rule, rows)
    )


def compute_blocking_row_ids(rows: list[dict[str, Any]], rules: list[dict[str, Any]]) -> list[str]:
    firing_reasons = {
        rule["trigger_reason"]
        for rule in rules
        if rule.get("blocks_publication") and rule_fires(rule, rows)
    }
    return sorted(
        {
            row["entry_id"]
            for row in rows
            if any(reason in firing_reasons for reason in (row.get("active_gap_reasons") or []))
        }
    )


def main() -> int:
    repo_root = Path(__file__).resolve().parents[2]
    artifact = load_json(repo_root / ARTIFACT_PATH)
    errors: list[str] = []

    if artifact.get("schema_version") != 1:
        errors.append("schema_version must be 1")
    if artifact.get("record_kind") != "clean_room_rebuild_proof":
        errors.append("record_kind must be clean_room_rebuild_proof")

    rows = artifact.get("rows")
    rules = artifact.get("rules")
    publication = artifact.get("publication") or {}
    summary = artifact.get("summary") or {}
    if not isinstance(rows, list):
        errors.append("rows must be an array")
        rows = []
    if not isinstance(rules, list):
        errors.append("rules must be an array")
        rules = []
    if not isinstance(publication, dict):
        errors.append("publication must be an object")
        publication = {}
    if not isinstance(summary, dict):
        errors.append("summary must be an object")
        summary = {}

    computed_summary = compute_summary(rows, rules)
    for field, value in computed_summary.items():
        if summary.get(field) != value:
            errors.append(f"summary.{field} expected {value} but found {summary.get(field)!r}")

    blocking_rule_ids = publication.get("blocking_rule_ids") or []
    blocking_row_ids = publication.get("blocking_row_ids") or []
    if blocking_rule_ids != sorted(blocking_rule_ids):
        errors.append("publication.blocking_rule_ids must be sorted")
    if blocking_row_ids != sorted(blocking_row_ids):
        errors.append("publication.blocking_row_ids must be sorted")

    computed_blocking_rule_ids = compute_blocking_rule_ids(rows, rules)
    computed_blocking_row_ids = compute_blocking_row_ids(rows, rules)
    if blocking_rule_ids != computed_blocking_rule_ids:
        errors.append(
            "publication.blocking_rule_ids disagrees with the computed blocking rules "
            f"({computed_blocking_rule_ids})"
        )
    if blocking_row_ids != computed_blocking_row_ids:
        errors.append(
            "publication.blocking_row_ids disagrees with the computed blocking rows "
            f"({computed_blocking_row_ids})"
        )

    expected_decision = "hold" if computed_blocking_rule_ids else "proceed"
    if publication.get("decision") != expected_decision:
        errors.append(
            f"publication.decision expected {expected_decision!r} but found {publication.get('decision')!r}"
        )

    if errors:
        for error in errors:
            print(f"error: {error}", file=sys.stderr)
        return 1

    print(f"ok: {ARTIFACT_PATH}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
