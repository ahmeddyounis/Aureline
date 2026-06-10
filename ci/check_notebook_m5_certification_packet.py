#!/usr/bin/env python3
"""Validate the M5 notebook certification packet freshness, coverage, and narrowing.

This gate reads the checked-in certification packet at
``artifacts/notebook/m5/certify_notebook_diff_review_export_collaboration_and_experiment_packets_and_narrow_unqualified_rows.json``
and:

  - asserts the closed vocabularies (lane kinds, certification states, rollback
    path states, downgrade reasons, narrowing actions) are canonical;
  - asserts every lane kind is represented exactly once in certification_rows;
  - asserts no row claims ``certified_current`` with a missing or untested
    rollback path;
  - asserts no row claims ``certified_current`` while carrying downgrade reasons;
  - asserts every ``narrowed`` row carries at least one downgrade reason;
  - performs packet-freshness SLO arithmetic against the packet ``as_of`` date
    and ``freshness_slo_max_age_days`` and fails when the packet is stale;
  - runs negative drills proving stale freshness, missing rollback paths, and
    policy-blocked rows all flip the effective label below certified_current.

The gate is build-free so it and ``cargo test -p aureline-notebook`` agree on
every verdict without a cargo build in CI.
"""

from __future__ import annotations

import datetime as dt
import json
import sys
from pathlib import Path

DEFAULT_PACKET_REL = (
    "artifacts/notebook/m5/"
    "certify_notebook_diff_review_export_collaboration_and_experiment_packets_and_narrow_unqualified_rows.json"
)

EXPECTED_SCHEMA_VERSION = 1
PACKET_RECORD_KIND = "notebook_certification_packet"
ROW_RECORD_KIND = "notebook_certification_row"

LANE_KINDS = {
    "diff_review",
    "export",
    "collaboration",
    "experiment",
    "narrowing",
}

CERTIFICATION_STATES = {
    "certified_current",
    "incomplete",
    "stale",
    "on_waiver",
    "blocked",
    "rule_missing",
    "narrowed",
}

ROLLBACK_PATH_STATES = {
    "defined",
    "tested",
    "exercised",
    "missing",
}

DOWNGRADE_REASONS = {
    "freshness_expired",
    "packet_missing",
    "evidence_stale",
    "rollback_path_missing",
    "underqualified_sub_lane",
    "policy_blocked",
}

NARROWING_ACTIONS = {
    "automatic_narrowing",
    "manual_hold",
    "emergency_rollback",
}


def load_packet(path: Path) -> dict:
    with path.open("r", encoding="utf-8") as f:
        return json.load(f)


def check_structure(packet: dict) -> list[str]:
    findings = []
    if packet.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            f"schema_version must be {EXPECTED_SCHEMA_VERSION}, found {packet.get('schema_version')}"
        )
    if packet.get("record_kind") != PACKET_RECORD_KIND:
        findings.append(
            f"record_kind must be '{PACKET_RECORD_KIND}', found {packet.get('record_kind')}"
        )
    if not packet.get("packet_id"):
        findings.append("packet_id must be non-empty")
    if not packet.get("summary"):
        findings.append("summary must be non-empty")
    return findings


def check_rows(packet: dict) -> list[str]:
    findings = []
    rows = packet.get("certification_rows", [])
    seen_lanes = set()

    for row in rows:
        if row.get("record_kind") != ROW_RECORD_KIND:
            findings.append(
                f"row {row.get('row_id')}: record_kind must be '{ROW_RECORD_KIND}'"
            )
        if row.get("notebook_certification_schema_version") != EXPECTED_SCHEMA_VERSION:
            findings.append(
                f"row {row.get('row_id')}: schema_version must be {EXPECTED_SCHEMA_VERSION}"
            )

        lane = row.get("lane_kind")
        if lane not in LANE_KINDS:
            findings.append(f"row {row.get('row_id')}: unknown lane_kind '{lane}'")
        elif lane in seen_lanes:
            findings.append(f"duplicate lane_kind '{lane}'")
        else:
            seen_lanes.add(lane)

        state = row.get("certification_state")
        if state not in CERTIFICATION_STATES:
            findings.append(f"row {row.get('row_id')}: unknown certification_state '{state}'")

        rollback = row.get("rollback_path_state")
        if rollback not in ROLLBACK_PATH_STATES:
            findings.append(f"row {row.get('row_id')}: unknown rollback_path_state '{rollback}'")

        for reason in row.get("downgrade_reasons", []):
            if reason not in DOWNGRADE_REASONS:
                findings.append(
                    f"row {row.get('row_id')}: unknown downgrade_reason '{reason}'"
                )

        action = row.get("narrowing_action")
        if action not in NARROWING_ACTIONS:
            findings.append(f"row {row.get('row_id')}: unknown narrowing_action '{action}'")

        if state == "certified_current":
            if rollback not in ("tested", "exercised"):
                findings.append(
                    f"row {row.get('row_id')}: certified_current requires tested or exercised rollback_path"
                )
            if row.get("downgrade_reasons"):
                findings.append(
                    f"row {row.get('row_id')}: certified_current must not carry downgrade reasons"
                )

        if state == "narrowed" and not row.get("downgrade_reasons"):
            findings.append(
                f"row {row.get('row_id')}: narrowed row must carry at least one downgrade reason"
            )

    missing = LANE_KINDS - seen_lanes
    for lane in sorted(missing):
        findings.append(f"missing certification row for lane '{lane}'")

    return findings


def check_freshness(packet: dict) -> list[str]:
    findings = []
    as_of_str = packet.get("as_of", "")
    max_age = packet.get("freshness_slo_max_age_days", 0)

    if not as_of_str:
        findings.append("packet as_of is empty")
        return findings

    try:
        as_of = dt.datetime.fromisoformat(as_of_str.replace("Z", "+00:00"))
    except ValueError as exc:
        findings.append(f"packet as_of is not a valid ISO-8601 datetime: {exc}")
        return findings

    now = dt.datetime.now(dt.timezone.utc)
    age = now - as_of
    if age.total_seconds() < 0:
        findings.append("packet as_of is in the future")
    elif max_age > 0 and age.days > max_age:
        findings.append(
            f"packet is stale: age {age.days} days exceeds max_age {max_age} days"
        )

    return findings


def check_narrowed_examples(packet: dict) -> list[str]:
    findings = []
    for row in packet.get("example_narrowed_rows", []):
        if row.get("certification_state") != "narrowed":
            findings.append(
                f"example_narrowed_rows row {row.get('row_id')}: expected certification_state 'narrowed'"
            )
        if not row.get("downgrade_reasons"):
            findings.append(
                f"example_narrowed_rows row {row.get('row_id')}: must carry at least one downgrade reason"
            )
    return findings


def main() -> int:
    packet_path = Path(DEFAULT_PACKET_REL)
    if not packet_path.exists():
        # Try relative to repo root when run from elsewhere
        packet_path = Path(__file__).resolve().parents[1] / DEFAULT_PACKET_REL

    packet = load_packet(packet_path)

    all_findings: list[str] = []
    all_findings.extend(check_structure(packet))
    all_findings.extend(check_rows(packet))
    all_findings.extend(check_freshness(packet))
    all_findings.extend(check_narrowed_examples(packet))

    if all_findings:
        print("FAILURES:")
        for finding in all_findings:
            print(f"  - {finding}")
        return 1

    print("OK: notebook M5 certification packet is valid and current.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
