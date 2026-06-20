#!/usr/bin/env python3
"""Continuity certification CI gate.

This gate is the certification automation for claimed managed, self-hosted, and
sovereign continuity rows. For every certification report it independently
recomputes the per-row certification verdict from the declared evidence states —
folding locality/tenant/key disclosure, control-plane/data-plane degradation,
backup/restore/failover drills, restore-identity/partial-loss semantics,
mirror/offline continuity, and continuity-proof freshness into one verdict — and
fails when:

- the checked-in report disagrees with the recompute (the typed Rust model and
  this Python automation must agree, so the artifact cannot quietly drift);
- a structural certification invariant breaks (a required dimension missing, an
  incoherent evidence ref, a single reference-environment drill standing in for
  more than one row, a local-core row narrowed, or a verdict not reused across
  the required surfaces);
- a row claims fresher continuity-proof freshness than the freshness-SLO
  dashboard records (the certification claim may not ride evidence the freshness
  gate already marked breached or missing); or
- the certification fixtures do not narrow or withdraw as declared.

It writes a validation capture consumed by release/support surfaces.
"""

from __future__ import annotations

import json
import pathlib
import sys

from jsonschema import Draft202012Validator
from referencing import Registry, Resource
from referencing.exceptions import NoSuchResource
from referencing.jsonschema import DRAFT202012

REPO_ROOT = pathlib.Path(__file__).resolve().parents[1]
SCHEMA_PATH = REPO_ROOT / "schemas/continuity/continuity_certification_report.schema.json"
ARTIFACT = REPO_ROOT / "artifacts/m5/continuity/certification/certified_rows.json"
FIXTURE_DIR = REPO_ROOT / "fixtures/continuity/certification_cases"
FRESHNESS_ARTIFACT = REPO_ROOT / "artifacts/m5/continuity/freshness_slo_dashboard.json"
CAPTURE_PATH = (
    REPO_ROOT
    / "artifacts/governance/captures/m5-continuity-certification_validation_capture.json"
)
AURELINE_SCHEMA_PREFIX = "https://aureline.dev/schemas/"

SHARED_CONTRACT_REF = "continuity:m5_continuity_certification:v1"
REPORT_RECORD_KIND = "continuity_certification_report_record"
SUPPORT_EXPORT_RECORD_KIND = "continuity_certification_support_export_record"

QUALIFICATION_RANK = {"stable": 0, "beta": 1, "preview": 2, "withdrawn": 3}
RANK_QUALIFICATION = {rank: name for name, rank in QUALIFICATION_RANK.items()}

REQUIRED_DIMENSIONS = [
    "locality_tenant_key",
    "control_data_plane_degradation",
    "backup_restore_failover",
    "restore_identity_partial_loss",
    "drill_freshness_slo",
]
DIMENSION_NARROW_REASON = {
    "locality_tenant_key": "locality_tenant_key_uncertified",
    "control_data_plane_degradation": "control_data_plane_degradation_uncertified",
    "backup_restore_failover": "backup_restore_failover_uncertified",
    "restore_identity_partial_loss": "restore_identity_partial_loss_uncertified",
    "mirror_offline_continuity": "mirror_offline_continuity_uncertified",
    "drill_freshness_slo": "drill_freshness_uncertified",
}
STATE_FLOOR = {
    "current": None,
    "not_applicable": None,
    "stale": "beta",
    "partial": "beta",
    "missing": "preview",
    "profile_mismatched": "withdrawn",
}
STALE_OR_MISSING_STATES = {"stale", "partial", "missing", "profile_mismatched"}

# Certification fixtures and the behaviour each one must prove.
FIXTURE_EXPECTATIONS = {
    "page.json": {"decision": "certified", "narrowed": 0, "withdrawn": 0},
    "case_backup_drill_stale_narrows.json": {
        "decision": "narrowed",
        "narrowed_row": "continuity-row:managed-cloud-sync",
        "narrowed_to": "beta",
    },
    "case_freshness_breached_narrows.json": {
        "decision": "narrowed",
        "narrowed_row": "continuity-row:managed-relay-failover",
        "narrowed_to": "beta",
    },
    "case_restore_identity_missing_narrows.json": {
        "decision": "narrowed",
        "narrowed_row": "continuity-row:self-hosted-restore",
        "narrowed_to": "preview",
    },
    "case_mirror_offline_missing_narrows.json": {
        "decision": "narrowed",
        "narrowed_row": "continuity-row:sovereign-airgap-snapshot",
        "narrowed_to": "preview",
    },
    "case_profile_mismatch_withdrawn.json": {
        "decision": "withdrawn",
        "narrowed_row": "continuity-row:sovereign-airgap-snapshot",
        "narrowed_to": "withdrawn",
    },
    "case_local_core_stays_certified.json": {
        "decision": "narrowed",
        "local_core_certified": "continuity-row:local-desktop-core",
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


def embedded_report(payload: dict) -> dict | None:
    kind = payload.get("record_kind")
    if kind == REPORT_RECORD_KIND:
        return payload
    if kind == SUPPORT_EXPORT_RECORD_KIND:
        return payload.get("report")
    return None


def in_scope(row: dict) -> bool:
    return (
        row.get("profile_class_token") != "local_only"
        or row.get("continuity_lane_token") == "managed_lane"
        or bool(row.get("has_claimed_managed_dependency"))
    )


def required_dimensions(row: dict) -> list[str]:
    if not in_scope(row):
        return []
    dims = list(REQUIRED_DIMENSIONS)
    if row.get("requires_offline_continuity"):
        dims.append("mirror_offline_continuity")
    return dims


def evidence_for(row: dict, dimension: str) -> dict | None:
    for cell in row.get("evidence", []):
        if cell.get("dimension_token") == dimension:
            return cell
    return None


def drill_ref(row: dict) -> str | None:
    cell = evidence_for(row, "backup_restore_failover")
    if cell and cell.get("evidence_ref"):
        return cell["evidence_ref"]
    return None


def surface_all_visible(sv: dict) -> bool:
    return all(
        sv.get(k)
        for k in (
            "about",
            "help",
            "service_health",
            "support_export",
            "docs_public_truth",
            "partner_qualification",
        )
    )


def surface_local_core_visible(sv: dict) -> bool:
    return all(sv.get(k) for k in ("about", "help", "service_health", "docs_public_truth"))


def shared_drill_refs(rows: list[dict]) -> set[str]:
    counts: dict[str, int] = {}
    for row in rows:
        if not in_scope(row):
            continue
        ref = drill_ref(row)
        if ref:
            counts[ref] = counts.get(ref, 0) + 1
    return {ref for ref, count in counts.items() if count > 1}


def cap(rank: int, floor: str | None) -> int:
    if floor is None:
        return rank
    return max(rank, QUALIFICATION_RANK[floor])


def recompute_row_outcome(row: dict, shared: set[str]) -> dict:
    scope = in_scope(row)
    sv = row.get("surface_visibility", {})
    shared_drill = scope and (drill_ref(row) in shared)

    claimed = row.get("claimed_qualification_token", "stable")
    effective_rank = QUALIFICATION_RANK[claimed]
    reasons: set[str] = set()
    stale_or_missing: set[str] = set()

    if scope:
        for cell in row.get("evidence", []):
            state = cell.get("state_token")
            floor = STATE_FLOOR.get(state)
            effective_rank = cap(effective_rank, floor)
            if state == "profile_mismatched":
                reasons.add("continuity_profile_mismatch")
            elif floor is not None:
                reasons.add(DIMENSION_NARROW_REASON[cell["dimension_token"]])
            if state in STALE_OR_MISSING_STATES:
                stale_or_missing.add(cell["dimension_token"])
        for dimension in required_dimensions(row):
            if evidence_for(row, dimension) is None:
                effective_rank = cap(effective_rank, "preview")
                reasons.add("required_evidence_missing")
                stale_or_missing.add(dimension)
        if not surface_all_visible(sv):
            effective_rank = cap(effective_rank, "beta")
            reasons.add("surface_reuse_incomplete")
        if shared_drill:
            effective_rank = cap(effective_rank, "beta")
            reasons.add("shared_reference_drill_reused")
    else:
        # local-core: only the declared cells inform stale/missing; never narrows.
        for cell in row.get("evidence", []):
            if cell.get("state_token") in STALE_OR_MISSING_STATES:
                stale_or_missing.add(cell["dimension_token"])

    effective = RANK_QUALIFICATION[effective_rank]
    if not scope:
        effective = claimed
    if effective == "withdrawn":
        verdict = "withdrawn"
    elif effective != claimed:
        verdict = "narrowed"
    else:
        verdict = "certified"

    return {
        "record_kind": "certified_row_outcome_record",
        "schema_version": 1,
        "shared_contract_ref": SHARED_CONTRACT_REF,
        "row_id": row["row_id"],
        "profile_class_token": row.get("profile_class_token"),
        "in_certification_scope": scope,
        "verdict_token": verdict,
        "certified": verdict == "certified",
        "narrowed": verdict != "certified",
        "claimed_qualification_token": claimed,
        "effective_qualification_token": effective,
        "narrow_reason_tokens": sorted(reasons),
        "stale_or_missing_dimension_tokens": sorted(stale_or_missing),
    }


def recompute(report: dict) -> dict:
    rows = report.get("input", {}).get("rows", [])
    shared = shared_drill_refs(rows)
    return {row["row_id"]: recompute_row_outcome(row, shared) for row in rows}


def check_recompute_parity(name: str, report: dict, findings: list[Finding]) -> dict:
    recomputed = recompute(report)
    declared = {o["row_id"]: o for o in report.get("row_outcomes", [])}
    if set(declared) != set(recomputed):
        findings.append(Finding(name, "row_outcomes do not cover the same rows as the input"))
    for row_id, expected in recomputed.items():
        got = declared.get(row_id)
        if got != expected:
            findings.append(
                Finding(
                    name,
                    f"{row_id}: checked-in outcome disagrees with the certification recompute; "
                    "regenerate (cargo run -p aureline-continuity --example "
                    "dump_m5_continuity_certification_fixtures -- page)",
                )
            )
    return recomputed


def check_structural(name: str, report: dict, findings: list[Finding]) -> None:
    if report.get("shared_contract_ref") != SHARED_CONTRACT_REF:
        findings.append(Finding(name, "shared_contract_ref is not the canonical contract"))
    rows = report.get("input", {}).get("rows", [])
    shared = shared_drill_refs(rows)
    for row in rows:
        scope = in_scope(row)
        for cell in row.get("evidence", []):
            requires_ref = cell.get("state_token") not in ("missing", "not_applicable")
            has_ref = bool(cell.get("evidence_ref"))
            if requires_ref != has_ref:
                findings.append(
                    Finding(name, f"{row['row_id']}:{cell['dimension_token']}: evidence ref incoherent with state")
                )
        if scope:
            for dimension in required_dimensions(row):
                if evidence_for(row, dimension) is None:
                    findings.append(
                        Finding(name, f"{row['row_id']}: required dimension '{dimension}' missing")
                    )
            if drill_ref(row) in shared:
                findings.append(
                    Finding(name, f"{row['row_id']}: a single reference drill may not stand in for more than one row")
                )
            if not surface_all_visible(row.get("surface_visibility", {})):
                findings.append(Finding(name, f"{row['row_id']}: certification verdict not reused across all surfaces"))
        else:
            if not surface_local_core_visible(row.get("surface_visibility", {})):
                findings.append(Finding(name, f"{row['row_id']}: local-core verdict not reused across in-product surfaces"))
    for outcome in report.get("row_outcomes", []):
        if not outcome["in_certification_scope"] and outcome["narrowed"]:
            findings.append(
                Finding(name, f"{outcome['row_id']}: a local-core row may not narrow or withdraw")
            )


def check_freshness_parity(name: str, report: dict, freshness_states: dict, findings: list[Finding]) -> None:
    """A certification row may not claim fresher freshness than the SLO dashboard."""
    if not freshness_states:
        return
    for row in report.get("input", {}).get("rows", []):
        if not in_scope(row):
            continue
        cell = evidence_for(row, "drill_freshness_slo")
        if cell is None:
            continue
        slo = freshness_states.get(row["row_id"])
        if slo is None:
            continue
        state = cell.get("state_token")
        if slo == "breached" and state in ("current", "not_applicable"):
            findings.append(
                Finding(
                    name,
                    f"{row['row_id']}: certifies drill_freshness '{state}' but the freshness SLO "
                    "dashboard records 'breached'",
                )
            )
        if slo == "missing" and state != "missing":
            findings.append(
                Finding(
                    name,
                    f"{row['row_id']}: certifies drill_freshness '{state}' but the freshness SLO "
                    "dashboard records 'missing'",
                )
            )


def check_expectation(name: str, report: dict, findings: list[Finding]) -> None:
    expect = FIXTURE_EXPECTATIONS.get(name)
    if expect is None:
        return
    summary = report.get("summary", {})
    if summary.get("overall_decision_token") != expect["decision"]:
        findings.append(
            Finding(name, f"expected decision {expect['decision']}, got {summary.get('overall_decision_token')}")
        )
    outcomes = {o["row_id"]: o for o in report.get("row_outcomes", [])}
    if "narrowed" in expect:
        narrowed = sum(1 for o in outcomes.values() if o["narrowed"])
        if narrowed != expect["narrowed"]:
            findings.append(Finding(name, f"expected {expect['narrowed']} narrowed rows, got {narrowed}"))
    if "withdrawn" in expect:
        withdrawn = sum(1 for o in outcomes.values() if o["verdict_token"] == "withdrawn")
        if withdrawn != expect["withdrawn"]:
            findings.append(Finding(name, f"expected {expect['withdrawn']} withdrawn rows, got {withdrawn}"))
    if "narrowed_row" in expect:
        row = outcomes.get(expect["narrowed_row"])
        if row is None:
            findings.append(Finding(name, f"missing expected row {expect['narrowed_row']}"))
        elif not row["narrowed"] or row["effective_qualification_token"] != expect["narrowed_to"]:
            findings.append(
                Finding(
                    name,
                    f"{expect['narrowed_row']} should narrow to {expect['narrowed_to']}, "
                    f"got {row['effective_qualification_token']}",
                )
            )
    if "local_core_certified" in expect:
        row = outcomes.get(expect["local_core_certified"])
        if row is None or row["in_certification_scope"] or row["narrowed"] or not row["certified"]:
            findings.append(
                Finding(
                    name,
                    f"{expect['local_core_certified']} must stay certified and never narrow when a managed row goes stale",
                )
            )


def freshness_state_map() -> dict:
    if not FRESHNESS_ARTIFACT.exists():
        return {}
    payload = json.loads(FRESHNESS_ARTIFACT.read_text(encoding="utf-8"))
    rows = payload.get("input", {}).get("rows", [])
    return {row["row_id"]: row.get("proof_packet", {}).get("slo_state") for row in rows}


def main() -> int:
    if not SCHEMA_PATH.exists() or not ARTIFACT.exists():
        print("missing schema or certified-row registry artifact", file=sys.stderr)
        return 2

    findings: list[Finding] = []
    val = validator()
    freshness_states = freshness_state_map()

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

        report = embedded_report(payload)
        if report is None:
            continue  # summary records carry no rows to recompute
        checked += 1
        check_recompute_parity(name, report, findings)
        check_structural(name, report, findings)
        check_freshness_parity(name, report, freshness_states, findings)
        check_expectation(name, report, findings)
        if path == ARTIFACT:
            seen_artifact = True

    if not seen_artifact:
        findings.append(Finding("artifact", "canonical certified-row registry was not checked"))

    artifact_report = json.loads(ARTIFACT.read_text(encoding="utf-8"))
    summary = artifact_report.get("summary", {})
    capture = {
        "status": "pass" if not findings else "fail",
        "as_of": artifact_report.get("as_of"),
        "reports_checked": checked,
        "summary": summary,
        "findings": [str(f) for f in findings],
    }
    CAPTURE_PATH.parent.mkdir(parents=True, exist_ok=True)
    CAPTURE_PATH.write_text(json.dumps(capture, indent=1) + "\n", encoding="utf-8")

    if findings:
        print(f"FAILED: {len(findings)} continuity-certification finding(s):", file=sys.stderr)
        for finding in findings:
            print(f"  - {finding}", file=sys.stderr)
        return 1

    print(
        f"OK: continuity certification gate passed over {checked} report(s); "
        f"overall decision '{summary.get('overall_decision_token')}', "
        f"{summary.get('certified_row_count')} of {summary.get('row_count')} rows certified"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
