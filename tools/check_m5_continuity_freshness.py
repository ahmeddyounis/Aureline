#!/usr/bin/env python3
"""Continuity-proof freshness SLO and shiproom-blocker CI gate.

This gate is the proof-expiry automation for claimed managed, self-hosted, and
sovereign continuity rows. Against each dashboard's ``as_of`` clock it recomputes
every continuity proof packet's freshness-SLO state from ``captured_at`` and the
declared SLO window, re-derives the per-row narrowing, the shiproom promotion
verdict, and the summary, and fails when:

- a packet's declared freshness state is **fresher than the clock allows**
  (a green continuity row riding stale evidence);
- the checked-in dashboard disagrees with the recompute (the typed Rust model and
  this Python automation must agree, so the artifact cannot quietly drift);
- the structural shiproom invariants break (a local-core row holding promotion, a
  release-scope row with no rerun path, an uncovered stop reason); or
- the stale-evidence fixtures do not narrow or hold promotion as declared.

It reuses :func:`run_drill_packets.rebuild_dashboard` so the rerun tool and the
gate share one freshness engine, and writes a validation capture.
"""

from __future__ import annotations

import datetime as dt
import json
import pathlib
import sys

from jsonschema import Draft202012Validator
from referencing import Registry, Resource
from referencing.exceptions import NoSuchResource
from referencing.jsonschema import DRAFT202012

REPO_ROOT = pathlib.Path(__file__).resolve().parents[1]
sys.path.insert(0, str(REPO_ROOT / "tools" / "continuity"))

import run_drill_packets as engine  # noqa: E402

SCHEMA_PATH = REPO_ROOT / "schemas/continuity/continuity_freshness_slo_dashboard.schema.json"
ARTIFACT = REPO_ROOT / "artifacts/m5/continuity/freshness_slo_dashboard.json"
FIXTURE_DIR = REPO_ROOT / "fixtures/continuity/stale_evidence_cases"
CAPTURE_PATH = (
    REPO_ROOT
    / "artifacts/governance/captures/m5-continuity-freshness-slo_validation_capture.json"
)
AURELINE_SCHEMA_PREFIX = "https://aureline.dev/schemas/"
FRESHNESS_RANK = {"current": 3, "due_for_refresh": 2, "breached": 1, "missing": 0}

# Stale-evidence fixtures and the behaviour each one must prove.
FIXTURE_EXPECTATIONS = {
    "page.json": {"decision": "proceed", "narrowed": 0, "blocked": 0},
    "case_managed_backup_breached_hold.json": {
        "decision": "hold",
        "narrowed_row": "continuity-row:managed-cloud-sync",
        "narrowed_to": "beta",
    },
    "case_relay_packet_missing_hold.json": {
        "decision": "hold",
        "narrowed_row": "continuity-row:managed-relay-failover",
        "narrowed_to": "preview",
    },
    "case_owner_signoff_missing_beta.json": {
        "decision": "hold",
        "narrowed_row": "continuity-row:self-hosted-restore",
        "narrowed_to": "beta",
    },
    "case_no_rerun_path_beta.json": {
        "decision": "hold",
        "narrowed_row": "continuity-row:sovereign-airgap-snapshot",
        "narrowed_to": "beta",
    },
    "case_local_core_stays_green.json": {
        "decision": "hold",
        "local_core_green": "continuity-row:local-desktop-core",
    },
}


class Finding:
    def __init__(self, source: str, message: str) -> None:
        self.source = source
        self.message = message

    def __str__(self) -> str:
        return f"{self.source}: {self.message}"


def retrieve_aureline_schema(uri: str) -> Resource:
    if not uri.startswith(AURELINE_SCHEMA_PREFIX):
        raise NoSuchResource(ref=uri)
    rel = uri.removeprefix(AURELINE_SCHEMA_PREFIX)
    candidate = REPO_ROOT / "schemas" / rel
    if not candidate.exists():
        raise NoSuchResource(ref=uri)
    return Resource.from_contents(
        json.loads(candidate.read_text(encoding="utf-8")),
        default_specification=DRAFT202012,
    )


def validator() -> Draft202012Validator:
    schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))
    registry = Registry(retrieve=retrieve_aureline_schema)
    return Draft202012Validator(schema, registry=registry)


def embedded_dashboard(payload: dict) -> dict | None:
    kind = payload.get("record_kind")
    if kind == engine.DASHBOARD_RECORD_KIND:
        return payload
    if kind == "continuity_freshness_slo_support_export_record":
        return payload.get("dashboard")
    return None


def check_freshness_honesty(name: str, dashboard: dict, findings: list[Finding]) -> None:
    as_of = engine.parse_date(dashboard.get("as_of"))
    if as_of is None:
        findings.append(Finding(name, "dashboard as_of is not an ISO date"))
        return
    for row in dashboard.get("input", {}).get("rows", []):
        packet = row.get("proof_packet", {})
        declared = packet.get("slo_state")
        recomputed = engine.recompute_slo_state(packet, as_of)
        if declared not in FRESHNESS_RANK:
            findings.append(Finding(name, f"{row['row_id']}: unknown slo_state {declared}"))
            continue
        if FRESHNESS_RANK[declared] > FRESHNESS_RANK[recomputed]:
            findings.append(
                Finding(
                    name,
                    f"{row['row_id']}: packet declares '{declared}' but the clock says "
                    f"'{recomputed}' — a continuity claim may not ride evidence fresher "
                    "than its captured_at allows",
                )
            )


def check_recompute_parity(name: str, dashboard: dict, findings: list[Finding]) -> dict:
    """Rebuild the dashboard and fail on any drift from the checked-in record."""
    as_of = engine.parse_date(dashboard.get("as_of"))
    rebuilt = engine.rebuild_dashboard(dashboard, as_of)
    for field in ("summary", "promotion", "defects", "row_outcomes"):
        if dashboard.get(field) != rebuilt.get(field):
            findings.append(
                Finding(
                    name,
                    f"checked-in '{field}' disagrees with the freshness recompute; "
                    "regenerate the dashboard (tools/continuity/run_drill_packets.py)",
                )
            )
    return rebuilt


def check_structural(name: str, dashboard: dict, findings: list[Finding]) -> None:
    if dashboard.get("shared_contract_ref") != engine.SHARED_CONTRACT_REF:
        findings.append(Finding(name, "shared_contract_ref is not the canonical contract"))
    covered = {
        rule.get("trigger_reason_token")
        for rule in dashboard.get("input", {}).get("stop_rules", [])
    }
    for reason in engine.STOP_REASONS:
        if reason not in covered:
            findings.append(Finding(name, f"stop reason '{reason}' has no watching stop rule"))
    for outcome in dashboard.get("row_outcomes", []):
        if not outcome["in_release_scope"] and outcome["blocks_promotion"]:
            findings.append(
                Finding(name, f"{outcome['row_id']}: a local-core row may not hold promotion")
            )
    for row in dashboard.get("input", {}).get("rows", []):
        if engine.in_release_scope(row) and not (row.get("rerun") or {}).get("rerun_command_ref"):
            findings.append(
                Finding(name, f"{row['row_id']}: release-scope row names no rerun path")
            )


def check_expectation(name: str, dashboard: dict, findings: list[Finding]) -> None:
    expect = FIXTURE_EXPECTATIONS.get(name)
    if expect is None:
        return
    promotion = dashboard.get("promotion", {})
    if promotion.get("decision") != expect["decision"]:
        findings.append(
            Finding(name, f"expected decision {expect['decision']}, got {promotion.get('decision')}")
        )
    outcomes = {o["row_id"]: o for o in dashboard.get("row_outcomes", [])}
    if "narrowed" in expect:
        narrowed = sum(1 for o in outcomes.values() if o["narrowed"])
        if narrowed != expect["narrowed"]:
            findings.append(Finding(name, f"expected {expect['narrowed']} narrowed rows, got {narrowed}"))
    if "blocked" in expect:
        blocked = sum(1 for o in outcomes.values() if o["blocks_promotion"])
        if blocked != expect["blocked"]:
            findings.append(Finding(name, f"expected {expect['blocked']} blocked rows, got {blocked}"))
    if "narrowed_row" in expect:
        row = outcomes.get(expect["narrowed_row"])
        if row is None:
            findings.append(Finding(name, f"missing expected row {expect['narrowed_row']}"))
        else:
            if not row["narrowed"] or not row["blocks_promotion"]:
                findings.append(Finding(name, f"{expect['narrowed_row']} should narrow and hold promotion"))
            if row["effective_qualification_token"] != expect["narrowed_to"]:
                findings.append(
                    Finding(
                        name,
                        f"{expect['narrowed_row']} should narrow to {expect['narrowed_to']}, "
                        f"got {row['effective_qualification_token']}",
                    )
                )
    if "local_core_green" in expect:
        row = outcomes.get(expect["local_core_green"])
        if row is None or row["in_release_scope"] or row["narrowed"] or row["blocks_promotion"] or not row["within_slo"]:
            findings.append(
                Finding(
                    name,
                    f"{expect['local_core_green']} must stay within SLO and never narrow or block "
                    "when a managed row goes stale",
                )
            )


def main() -> int:
    if not SCHEMA_PATH.exists() or not ARTIFACT.exists():
        print("missing schema or dashboard artifact", file=sys.stderr)
        return 2

    findings: list[Finding] = []
    val = validator()

    files = [ARTIFACT] + sorted(FIXTURE_DIR.glob("*.json"))
    seen_artifact = False
    checked = 0
    for path in files:
        name = path.name
        try:
            payload = json.loads(path.read_text(encoding="utf-8"))
        except Exception as exc:
            findings.append(Finding(name, f"invalid json: {exc}"))
            continue

        for error in val.iter_errors(payload):
            location = ".".join(str(p) for p in error.path) or "<root>"
            findings.append(Finding(name, f"schema: {location}: {error.message}"))

        dashboard = embedded_dashboard(payload)
        if dashboard is None:
            continue  # summary records carry no rows to recompute
        checked += 1
        check_freshness_honesty(name, dashboard, findings)
        check_recompute_parity(name, dashboard, findings)
        check_structural(name, dashboard, findings)
        check_expectation(name, dashboard, findings)
        if path == ARTIFACT:
            seen_artifact = True

    if not seen_artifact:
        findings.append(Finding("artifact", "canonical dashboard artifact was not checked"))

    artifact_dashboard = json.loads(ARTIFACT.read_text(encoding="utf-8"))
    summary = artifact_dashboard.get("summary", {})
    capture = {
        "status": "pass" if not findings else "fail",
        "as_of": artifact_dashboard.get("as_of"),
        "dashboards_checked": checked,
        "summary": summary,
        "promotion": artifact_dashboard.get("promotion"),
        "findings": [str(f) for f in findings],
    }
    CAPTURE_PATH.parent.mkdir(parents=True, exist_ok=True)
    CAPTURE_PATH.write_text(json.dumps(capture, indent=1) + "\n", encoding="utf-8")

    if findings:
        print(f"FAILED: {len(findings)} continuity-freshness finding(s):", file=sys.stderr)
        for finding in findings:
            print(f"  - {finding}", file=sys.stderr)
        return 1

    print(
        f"OK: continuity freshness-SLO gate passed over {checked} dashboard(s); "
        f"promotion decision '{artifact_dashboard.get('promotion', {}).get('decision')}', "
        f"{summary.get('within_slo_row_count')} of {summary.get('row_count')} rows within SLO"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
