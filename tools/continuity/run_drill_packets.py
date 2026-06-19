#!/usr/bin/env python3
"""Rerun and refresh continuity drill packets and the freshness-SLO dashboard.

This is the continuity-proof freshness rehearsal path. It moves continuity truth
from one-time packet generation to ongoing evidence freshness: it recomputes each
proof packet's freshness-SLO state from its ``captured_at`` date against an
``as_of`` clock, re-derives the per-row narrowing, the shiproom promotion verdict,
and the dashboard summary, and writes a consistent dashboard back — so a stale
locality / tenant / key / failover continuity row can be rerun or refreshed
without manual artifact surgery.

The freshness recompute and narrowing here mirror the typed model in
``crates/aureline-continuity/src/m5_continuity_freshness_slo``. The CI gate
``tools/check_m5_continuity_freshness.py`` reuses :func:`rebuild_dashboard` and
asserts it reproduces the checked-in Rust-generated artifact exactly, so the two
implementations cannot drift.

Subcommands::

    # Recompute the canonical dashboard's freshness against today and report drift
    run_drill_packets.py --check

    # Refresh every packet's freshness state against an explicit clock and write
    run_drill_packets.py --refresh --as-of 2026-09-01 --write

    # Record a fresh drill for one row (rerun rehearsal) and write the result
    run_drill_packets.py --rerun continuity-row:managed-cloud-sync \
        --captured-at 2026-09-01 --write

    # Regenerate the dashboard and stale-evidence fixtures from the Rust example
    run_drill_packets.py --regenerate
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import pathlib
import subprocess
import sys
from typing import Any

REPO_ROOT = pathlib.Path(__file__).resolve().parents[2]
DASHBOARD_ARTIFACT = REPO_ROOT / "artifacts/m5/continuity/freshness_slo_dashboard.json"
FIXTURE_DIR = REPO_ROOT / "fixtures/continuity/stale_evidence_cases"
SHARED_CONTRACT_REF = "continuity:m5_continuity_freshness_slo:v1"

DASHBOARD_RECORD_KIND = "continuity_freshness_slo_dashboard_record"
SUMMARY_RECORD_KIND = "continuity_freshness_slo_summary_record"
ROW_OUTCOME_RECORD_KIND = "continuity_freshness_row_outcome_record"

STOP_REASONS = (
    "continuity_packet_freshness_breached",
    "continuity_packet_missing",
    "drill_owner_signoff_missing",
    "rerun_path_unavailable",
    "continuity_evidence_unqualified",
)

# Lifecycle ranks: a narrower label ranks higher; narrowing takes the max.
QUALIFICATION_RANK = {"stable": 0, "beta": 1, "preview": 2, "withdrawn": 3}
RANK_QUALIFICATION = {rank: label for label, rank in QUALIFICATION_RANK.items()}

# The example regenerates these dashboards; (subcommand, output filename).
EXAMPLE = "dump_m5_continuity_freshness_slo_fixtures"
FIXTURE_TARGETS = [
    ("page", "page.json"),
    ("summary", "summary.json"),
    ("support-export", "support_export.json"),
    ("case-managed-backup-breached-hold", "case_managed_backup_breached_hold.json"),
    ("case-relay-packet-missing-hold", "case_relay_packet_missing_hold.json"),
    ("case-owner-signoff-missing-beta", "case_owner_signoff_missing_beta.json"),
    ("case-no-rerun-path-beta", "case_no_rerun_path_beta.json"),
    ("case-local-core-stays-green", "case_local_core_stays_green.json"),
]


def parse_date(value: Any) -> dt.date | None:
    if not isinstance(value, str) or not value:
        return None
    head = value.split("T", 1)[0]
    try:
        return dt.date.fromisoformat(head)
    except ValueError:
        return None


def recompute_slo_state(packet: dict[str, Any], as_of: dt.date) -> str:
    """Recompute a packet's freshness-SLO state from ``captured_at`` vs ``as_of``.

    Mirrors ``ContinuityFreshnessSloState`` derivation: a packet with no capture
    is ``missing``; one older than its target is ``breached``; one inside the warn
    window is ``due_for_refresh``; otherwise it is ``current``.
    """
    captured = parse_date(packet.get("captured_at"))
    if captured is None:
        return "missing"
    slo = packet.get("freshness_slo") or {}
    target = slo.get("target_max_age_days")
    warn = slo.get("warn_within_days")
    if not isinstance(target, int) or not isinstance(warn, int):
        return "missing"
    age = (as_of - captured).days
    if age > target:
        return "breached"
    if (target - age) <= warn:
        return "due_for_refresh"
    return "current"


def is_within_slo(state: str) -> bool:
    return state in ("current", "due_for_refresh")


def is_local_core(row: dict[str, Any]) -> bool:
    return (
        row.get("continuity_lane") == "local_core"
        and row.get("profile_class") == "local_only"
    )


def in_release_scope(row: dict[str, Any]) -> bool:
    return not is_local_core(row)


def claim_above_cutline(row: dict[str, Any]) -> bool:
    return row.get("claimed_qualification") in ("stable", "beta")


def has_capture(packet: dict[str, Any]) -> bool:
    return packet.get("captured_at") is not None and bool(packet.get("evidence_refs"))


def row_state(row: dict[str, Any], slo_state: str) -> str:
    if not in_release_scope(row):
        return "due_for_refresh" if slo_state == "due_for_refresh" else "fresh"
    if slo_state == "missing":
        return "narrowed_missing"
    if slo_state == "breached":
        return "narrowed_stale"
    if not row.get("owner_signoff_present", False):
        return "narrowed_unowned"
    return "due_for_refresh" if slo_state == "due_for_refresh" else "fresh"


def narrowed_floor(state: str) -> str | None:
    if state in ("fresh", "due_for_refresh"):
        return None
    if state in ("narrowed_stale", "narrowed_unowned"):
        return "beta"
    if state == "narrowed_missing":
        return "preview"
    return None


def stop_reasons(row: dict[str, Any], slo_state: str) -> list[str]:
    reasons: list[str] = []
    if not in_release_scope(row):
        return reasons
    if slo_state == "missing":
        reasons.append("continuity_packet_missing")
    elif slo_state == "breached":
        reasons.append("continuity_packet_freshness_breached")
    if not row.get("owner_signoff_present", False):
        reasons.append("drill_owner_signoff_missing")
    if (row.get("rerun") or {}).get("rerun_class") == "no_rerun_path":
        reasons.append("rerun_path_unavailable")
    if row.get("evidence_unqualified", False):
        reasons.append("continuity_evidence_unqualified")
    return reasons


def narrower(label: str, floor: str) -> str:
    return RANK_QUALIFICATION[max(QUALIFICATION_RANK[label], QUALIFICATION_RANK[floor])]


def effective_qualification(row: dict[str, Any], slo_state: str) -> str:
    effective = row.get("claimed_qualification", "stable")
    floor = narrowed_floor(row_state(row, slo_state))
    if floor is not None:
        effective = narrower(effective, floor)
    if in_release_scope(row):
        if (row.get("rerun") or {}).get("rerun_class") == "no_rerun_path":
            effective = narrower(effective, "beta")
        if row.get("evidence_unqualified", False):
            effective = narrower(effective, "preview")
    return effective


def rerun_automatable(row: dict[str, Any]) -> bool:
    return (row.get("rerun") or {}).get("rerun_class") in (
        "automated_rerun",
        "scripted_refresh",
    )


def blocks_promotion(row: dict[str, Any], slo_state: str) -> bool:
    return (
        in_release_scope(row)
        and claim_above_cutline(row)
        and bool(stop_reasons(row, slo_state))
    )


def rebuild_dashboard(dashboard: dict[str, Any], as_of: dt.date) -> dict[str, Any]:
    """Recompute freshness, narrowing, verdict, and summary for a dashboard.

    Returns a new dashboard dict; ``captured_at`` dates and structural fields are
    preserved, while every freshness-derived field is recomputed against ``as_of``.
    """
    result = json.loads(json.dumps(dashboard))  # deep copy
    body = result.get("input", {})
    rows = body.get("rows", [])
    stop_rules = body.get("stop_rules", [])

    outcomes: list[dict[str, Any]] = []
    blocked_row_ids: list[str] = []
    blocked_reasons: set[str] = set()

    for row in rows:
        packet = row.get("proof_packet", {})
        slo_state = recompute_slo_state(packet, as_of)
        packet["slo_state"] = slo_state
        packet["slo_state_token"] = slo_state
        state = row_state(row, slo_state)
        effective = effective_qualification(row, slo_state)
        reasons = sorted(set(stop_reasons(row, slo_state)))
        blocks = blocks_promotion(row, slo_state)
        if blocks:
            blocked_row_ids.append(row["row_id"])
            blocked_reasons.update(reasons)
        outcomes.append(
            {
                "record_kind": ROW_OUTCOME_RECORD_KIND,
                "schema_version": 1,
                "shared_contract_ref": SHARED_CONTRACT_REF,
                "row_id": row["row_id"],
                "profile_class_token": row.get("profile_class"),
                "in_release_scope": in_release_scope(row),
                "within_slo": is_within_slo(slo_state),
                "slo_state_token": slo_state,
                "row_state_token": state,
                "claimed_qualification_token": row.get("claimed_qualification"),
                "effective_qualification_token": effective,
                "narrowed": effective != row.get("claimed_qualification"),
                "blocks_promotion": blocks,
                "rerun_automatable": rerun_automatable(row),
                "active_stop_reason_tokens": reasons,
            }
        )

    firing_rule_ids = [
        rule["rule_id"]
        for rule in stop_rules
        if rule.get("blocks_promotion")
        and rule.get("trigger_reason_token") in blocked_reasons
    ]
    decision = "hold" if blocked_row_ids else "proceed"

    defects = audit_defects(body, rows, stop_rules, outcomes, decision)

    def count_state(state: str) -> int:
        return sum(
            1 for o in outcomes if o["slo_state_token"] == state
        )

    summary = {
        "record_kind": SUMMARY_RECORD_KIND,
        "schema_version": 1,
        "shared_contract_ref": SHARED_CONTRACT_REF,
        "overall_decision_token": decision,
        "row_count": len(rows),
        "release_scope_row_count": sum(1 for r in rows if in_release_scope(r)),
        "local_core_row_count": sum(1 for r in rows if is_local_core(r)),
        "within_slo_row_count": sum(1 for o in outcomes if o["within_slo"]),
        "due_for_refresh_row_count": count_state("due_for_refresh"),
        "breached_row_count": count_state("breached"),
        "missing_row_count": count_state("missing"),
        "narrowed_row_count": sum(1 for o in outcomes if o["narrowed"]),
        "blocked_row_count": sum(1 for o in outcomes if o["blocks_promotion"]),
        "stop_rules_firing_count": len(firing_rule_ids),
        "automatable_rerun_row_count": sum(
            1 for r in rows if in_release_scope(r) and rerun_automatable(r)
        ),
        "defect_count": len(defects),
    }

    result["as_of"] = body.get("as_of", result.get("as_of"))
    result["summary"] = summary
    result["promotion"] = {
        "decision": decision,
        "firing_rule_ids": firing_rule_ids,
        "blocked_row_ids": blocked_row_ids,
    }
    result["defects"] = defects
    result["row_outcomes"] = outcomes
    return result


def audit_defects(
    body: dict[str, Any],
    rows: list[dict[str, Any]],
    stop_rules: list[dict[str, Any]],
    outcomes: list[dict[str, Any]],
    decision: str,
) -> list[dict[str, Any]]:
    defects: list[dict[str, Any]] = []

    def add(kind: str, source: str, note: str) -> None:
        defects.append(
            {
                "record_kind": "continuity_freshness_defect_record",
                "schema_version": 1,
                "shared_contract_ref": SHARED_CONTRACT_REF,
                "defect_id": f"continuity:defect:freshness-slo:{kind}:{source}",
                "defect_kind": kind,
                "defect_kind_token": kind,
                "source": source,
                "note": note,
            }
        )

    for row in rows:
        packet = row.get("proof_packet", {})
        slo = packet.get("freshness_slo", {})
        if slo.get("warn_within_days", 0) > slo.get("target_max_age_days", 0):
            add(
                "freshness_window_inconsistent",
                row["row_id"],
                "a freshness SLO warn window may not exceed its target max age",
            )
        state = packet.get("slo_state")
        capture = has_capture(packet)
        if state != "missing" and not capture:
            add(
                "packet_state_capture_incoherent",
                row["row_id"],
                "a packet whose SLO state is not missing must carry a capture date and evidence ref",
            )
        if state == "missing" and capture:
            add(
                "packet_state_capture_incoherent",
                row["row_id"],
                "a packet marked missing may not carry a capture date or evidence ref",
            )
        if in_release_scope(row) and not (row.get("rerun") or {}).get("rerun_command_ref"):
            add(
                "rerun_path_undeclared",
                row["row_id"],
                "every release-scope row must name a rerun tool or command so evidence can be refreshed without manual surgery",
            )

    for outcome in outcomes:
        if not outcome["in_release_scope"] and outcome["blocks_promotion"]:
            add(
                "local_core_marked_blocking",
                outcome["row_id"],
                "a local-core continuity row may not hold promotion when a managed row goes stale",
            )

    covered = {rule.get("trigger_reason_token") for rule in stop_rules}
    for reason in STOP_REASONS:
        if reason not in covered:
            add(
                "stop_reason_uncovered",
                reason,
                "every continuity stop reason must be watched by a shiproom stop rule",
            )

    expected_hold = any(o["blocks_promotion"] for o in outcomes)
    if expected_hold != (decision == "hold"):
        add(
            "promotion_verdict_incoherent",
            "dashboard:promotion",
            "the promotion decision must be hold when any row holds promotion and proceed otherwise",
        )
    return defects


def rel(path: pathlib.Path) -> str:
    try:
        return str(path.relative_to(REPO_ROOT))
    except ValueError:
        return str(path)


def load(path: pathlib.Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def dump(path: pathlib.Path, payload: dict[str, Any]) -> None:
    path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")


def cmd_regenerate() -> int:
    FIXTURE_DIR.mkdir(parents=True, exist_ok=True)
    DASHBOARD_ARTIFACT.parent.mkdir(parents=True, exist_ok=True)
    for subcommand, filename in FIXTURE_TARGETS:
        try:
            out = subprocess.run(
                [
                    "cargo",
                    "run",
                    "-q",
                    "-p",
                    "aureline-continuity",
                    "--example",
                    EXAMPLE,
                    "--",
                    subcommand,
                ],
                cwd=REPO_ROOT,
                check=True,
                capture_output=True,
                text=True,
            ).stdout
        except (OSError, subprocess.CalledProcessError) as exc:
            stderr = getattr(exc, "stderr", "")
            print(f"regenerate failed for {subcommand}: {exc}\n{stderr}", file=sys.stderr)
            return 2
        (FIXTURE_DIR / filename).write_text(out, encoding="utf-8")
    # The canonical dashboard artifact is the seeded page.
    DASHBOARD_ARTIFACT.write_text(
        (FIXTURE_DIR / "page.json").read_text(encoding="utf-8"), encoding="utf-8"
    )
    print(f"regenerated {len(FIXTURE_TARGETS)} fixtures and the dashboard artifact")
    return 0


def cmd_recompute(args: argparse.Namespace) -> int:
    path = pathlib.Path(args.dashboard) if args.dashboard else DASHBOARD_ARTIFACT
    dashboard = load(path)
    if args.as_of:
        as_of = parse_date(args.as_of)
    else:
        as_of = parse_date(dashboard.get("as_of"))
    if as_of is None:
        print("could not determine an as_of date", file=sys.stderr)
        return 2

    if args.rerun:
        row = next(
            (r for r in dashboard.get("input", {}).get("rows", []) if r["row_id"] == args.rerun),
            None,
        )
        if row is None:
            print(f"no such row: {args.rerun}", file=sys.stderr)
            return 2
        captured = args.captured_at or as_of.isoformat()
        row["proof_packet"]["captured_at"] = captured
        if not row["proof_packet"].get("evidence_refs"):
            row["proof_packet"]["evidence_refs"] = [
                f"{row['proof_packet']['packet_id']}:{captured}"
            ]
        row.setdefault("rerun", {})["last_rerun_at"] = captured

    rebuilt = rebuild_dashboard(dashboard, as_of)

    drift = rebuilt["summary"] != dashboard.get("summary") or rebuilt["promotion"] != dashboard.get("promotion")
    if args.write:
        dump(path, rebuilt)
        print(f"wrote refreshed dashboard to {rel(path)} (as_of {as_of})")
        return 0

    decision = rebuilt["promotion"]["decision"]
    narrowed = rebuilt["summary"]["narrowed_row_count"]
    print(
        f"as_of {as_of}: decision={decision} narrowed={narrowed} "
        f"blocked={rebuilt['summary']['blocked_row_count']} "
        f"defects={rebuilt['summary']['defect_count']}"
    )
    if args.check and drift:
        print(
            "DRIFT: the checked-in dashboard does not match the recompute; "
            "run with --rerun/--refresh --write or --regenerate",
            file=sys.stderr,
        )
        return 1
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--regenerate", action="store_true", help="rebuild fixtures from the Rust example")
    mode.add_argument("--refresh", action="store_true", help="recompute freshness against --as-of")
    mode.add_argument("--check", action="store_true", help="report drift without writing")
    mode.add_argument("--rerun", metavar="ROW_ID", help="record a fresh drill for one row")
    parser.add_argument("--as-of", help="evaluation clock date (YYYY-MM-DD)")
    parser.add_argument("--captured-at", help="capture date for --rerun (defaults to --as-of)")
    parser.add_argument("--dashboard", help="dashboard JSON path (defaults to the canonical artifact)")
    parser.add_argument("--write", action="store_true", help="write the recomputed dashboard back")
    args = parser.parse_args()

    if args.regenerate:
        return cmd_regenerate()
    if args.refresh and args.write is False and args.check is False:
        # A bare --refresh implies writing the refreshed dashboard.
        args.write = True
    return cmd_recompute(args)


if __name__ == "__main__":
    raise SystemExit(main())
