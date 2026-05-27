#!/usr/bin/env python3
"""Regenerate the M4 execution-plane certification truth-packet artifact and fixtures."""

from __future__ import annotations

import copy
import json
import os
from pathlib import Path
from collections import OrderedDict

REPO_ROOT = Path(__file__).resolve().parent.parent
ARTIFACT_PATH = REPO_ROOT / "artifacts/runtime/m4/publish_execution_plane_certification_packets_for_local_remote_truth_packet.json"
FIXTURE_DIR = REPO_ROOT / "fixtures/runtime/m4/publish_execution_plane_certification_packets_for_local_remote"

DOC_REF = "docs/runtime/m4/publish-execution-plane-certification-packets-for-local-remote.md"
FIXTURE_REF = "fixtures/runtime/m4/publish_execution_plane_certification_packets_for_local_remote"
CAPTURED_AT = "2026-05-27T12:00:00Z"

LANES = ["local_lane", "remote_helper_lane", "enterprise_network_lane"]
LANE_PREFIX = {
    "local_lane": "local",
    "remote_helper_lane": "remote",
    "enterprise_network_lane": "enterprise",
}
SURFACES = [
    "terminal", "task", "test", "debug", "artifact",
    "request_workspace", "preview", "cli_headless",
    "docs_help", "support_export", "conformance_dashboard",
]
ROUTE_STATES = ["local_route", "remote_helper_route", "enterprise_network_route", "route_drift", "blocked_target"]
RECONNECT_STATES = ["reconnect_required", "reconnect_honest", "restore_no_rerun"]
DEGRADED_HELPER_STATES = ["capability_degraded", "helper_offline", "helper_skew"]
ARTIFACT_PROVENANCE_STATES = ["provenance_tracked", "provenance_missing"]
CONSUMER_SURFACES = [
    "editor_run_surface", "terminal_pane", "task_panel", "cli_headless",
    "support_export", "release_proof_index", "help_about", "conformance_dashboard",
]

PACKET_ID = "packet:m4:publish_execution_plane_certification"
WORKFLOW_ID = "workflow.runtime.publish_execution_plane_certification"
RECORD_KIND = "publish_execution_plane_certification_packets_for_local_remote_truth_stable_packet"


def disclosure(automation_token: str) -> str:
    return f"{DOC_REF}#{automation_token}"


def row_skeleton(row_id: str, lane: str, row_class: str) -> dict:
    return {
        "row_id": row_id,
        "lane_class": lane,
        "row_class": row_class,
        "support_class": "launch_stable",
        "surface_binding_class": "not_applicable",
        "route_state_class": "not_applicable",
        "reconnect_state_class": "not_applicable",
        "degraded_helper_state_class": "not_applicable",
        "artifact_provenance_state_class": "not_applicable",
        "evidence_class": "conformance_suite_evidence",
        "known_limit_class": "none_declared",
        "downgrade_automation_class": "none",
        "confidence_class": "high_confidence",
        "evidence_refs": [FIXTURE_REF],
        "disclosure_ref": None,
        "execution_context_id_binding": None,
        "restore_preserves_no_rerun": False,
        "raw_source_material_excluded": True,
        "secrets_excluded": True,
        "ambient_authority_excluded": True,
        "captured_at": CAPTURED_AT,
    }


def quality_row(lane: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(f"row:{prefix}:quality", lane, "execution_plane_certification_quality")
    row["evidence_class"] = "release_evidence_review"
    row["downgrade_automation_class"] = "auto_block_on_missing_evidence"
    row["evidence_refs"] = [DOC_REF, FIXTURE_REF]
    row["disclosure_ref"] = disclosure("auto_block_on_missing_evidence")
    return row


def surface_row(lane: str, surface: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(f"row:{prefix}:surface:{surface}", lane, "surface_binding")
    row["surface_binding_class"] = surface
    row["evidence_class"] = "conformance_suite_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_lineage_break"
    row["disclosure_ref"] = disclosure("auto_narrow_on_lineage_break")
    return row


def target_admission_row(lane: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(f"row:{prefix}:target_admission", lane, "target_admission")
    row["evidence_class"] = "automated_functional_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_target_unreachable"
    row["disclosure_ref"] = disclosure("auto_narrow_on_target_unreachable")
    return row


def route_state_row(lane: str, state: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(f"row:{prefix}:route:{state}", lane, "route_admission")
    row["route_state_class"] = state
    row["evidence_class"] = "failure_recovery_drill_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_route_drift"
    row["disclosure_ref"] = disclosure("auto_narrow_on_route_drift")
    return row


def restore_rerun_row(lane: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(f"row:{prefix}:restore_rerun_honesty", lane, "restore_rerun_honesty")
    row["evidence_class"] = "failure_recovery_drill_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_silent_rerun"
    row["restore_preserves_no_rerun"] = True
    row["disclosure_ref"] = disclosure("auto_narrow_on_silent_rerun")
    return row


def reconnect_state_row(lane: str, state: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(f"row:{prefix}:reconnect:{state}", lane, "reconnect_admission")
    row["reconnect_state_class"] = state
    row["evidence_class"] = "conformance_suite_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_reconnect_gap"
    row["disclosure_ref"] = disclosure("auto_narrow_on_reconnect_gap")
    return row


def degraded_helper_state_row(lane: str, state: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(f"row:{prefix}:degraded_helper:{state}", lane, "degraded_helper_admission")
    row["degraded_helper_state_class"] = state
    row["evidence_class"] = "failure_recovery_drill_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_degraded_helper"
    row["disclosure_ref"] = disclosure("auto_narrow_on_degraded_helper")
    return row


def artifact_provenance_state_row(lane: str, state: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(f"row:{prefix}:artifact_provenance:{state}", lane, "artifact_provenance_admission")
    row["artifact_provenance_state_class"] = state
    row["evidence_class"] = "automated_functional_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_artifact_provenance_gap"
    row["disclosure_ref"] = disclosure("auto_narrow_on_artifact_provenance_gap")
    return row


def lineage_row(lane: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(f"row:{prefix}:lineage_admission", lane, "lineage_admission")
    row["evidence_class"] = "automated_functional_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_lineage_break"
    row["execution_context_id_binding"] = f"exec:m4:{prefix}:lineage"
    row["disclosure_ref"] = disclosure("auto_narrow_on_lineage_break")
    return row


def projection(surface: str) -> dict:
    return {
        "consumer_surface": surface,
        "projection_ref": f"projection:{surface}",
        "execution_plane_packet_id_ref": PACKET_ID,
        "rendered_at": "2026-05-27T12:00:01Z",
        "preserves_same_packet": True,
        "preserves_lane_vocabulary": True,
        "preserves_row_class_vocabulary": True,
        "preserves_support_class_vocabulary": True,
        "preserves_surface_binding_vocabulary": True,
        "preserves_route_state_vocabulary": True,
        "preserves_reconnect_state_vocabulary": True,
        "preserves_degraded_helper_state_vocabulary": True,
        "preserves_artifact_provenance_state_vocabulary": True,
        "preserves_known_limit_vocabulary": True,
        "preserves_downgrade_automation_vocabulary": True,
        "preserves_evidence_class_vocabulary": True,
        "supports_json_export": True,
        "raw_private_material_excluded": True,
        "ambient_authority_excluded": True,
    }


def lane_rows(lane: str) -> list:
    out = [quality_row(lane)]
    for surface in SURFACES:
        out.append(surface_row(lane, surface))
    out.append(target_admission_row(lane))
    for state in ROUTE_STATES:
        out.append(route_state_row(lane, state))
    out.append(restore_rerun_row(lane))
    for state in RECONNECT_STATES:
        out.append(reconnect_state_row(lane, state))
    for state in DEGRADED_HELPER_STATES:
        out.append(degraded_helper_state_row(lane, state))
    for state in ARTIFACT_PROVENANCE_STATES:
        out.append(artifact_provenance_state_row(lane, state))
    out.append(lineage_row(lane))
    return out


def build_packet() -> dict:
    rows = []
    for lane in LANES:
        rows.extend(lane_rows(lane))
    return {
        "record_kind": RECORD_KIND,
        "schema_version": 1,
        "packet_id": PACKET_ID,
        "workflow_or_surface_id": WORKFLOW_ID,
        "generated_at": CAPTURED_AT,
        "covered_lanes": LANES,
        "rows": rows,
        "consumer_projections": [projection(s) for s in CONSUMER_SURFACES],
        "source_contract_refs": [DOC_REF],
        "promotion_state": "stable",
        "validation_findings": [],
    }


def collect_tokens(rows, key):
    tokens = set()
    for r in rows:
        val = r.get(key)
        if val is not None:
            tokens.add(val)
    return sorted(tokens)


def build_expect(rows):
    return {
        "promotion_state": "stable",
        "validation_finding_count": 0,
        "row_count": len(rows),
        "lane_tokens": collect_tokens(rows, "lane_class"),
        "row_class_tokens": collect_tokens(rows, "row_class"),
        "support_class_tokens": collect_tokens(rows, "support_class"),
        "surface_binding_tokens": collect_tokens(rows, "surface_binding_class"),
        "route_state_tokens": collect_tokens(rows, "route_state_class"),
        "reconnect_state_tokens": collect_tokens(rows, "reconnect_state_class"),
        "degraded_helper_state_tokens": collect_tokens(rows, "degraded_helper_state_class"),
        "artifact_provenance_state_tokens": collect_tokens(rows, "artifact_provenance_state_class"),
        "known_limit_tokens": collect_tokens(rows, "known_limit_class"),
        "downgrade_automation_tokens": collect_tokens(rows, "downgrade_automation_class"),
        "evidence_class_tokens": collect_tokens(rows, "evidence_class"),
        "support_export_safe": True,
        "expected_finding_kinds": [],
    }


def fixture_baseline() -> dict:
    packet = build_packet()
    return {
        "record_kind": "publish_execution_plane_certification_packets_for_local_remote_truth_stable_case",
        "schema_version": 1,
        "case_name": "baseline_stable",
        "scenario": "Baseline stable posture: all three lanes (local, remote_helper, enterprise_network) publish an execution_plane_certification_quality row at launch_stable plus full surface-binding coverage, target admission, full route-state admission coverage, restore-rerun-honesty, full reconnect-state admission coverage, full degraded-helper-state admission coverage, full artifact-provenance-state admission coverage, and lineage admission binding execution_context_id; every row binds support, known limit, downgrade automation, and evidence classes; narrowed rows carry disclosure refs; and all eight required consumer projections preserve the packet verbatim.",
        "input": {
            "packet_id": packet["packet_id"],
            "workflow_or_surface_id": packet["workflow_or_surface_id"],
            "generated_at": packet["generated_at"],
            "covered_lanes": packet["covered_lanes"],
            "rows": packet["rows"],
            "consumer_projections": packet["consumer_projections"],
            "source_contract_refs": packet["source_contract_refs"],
        },
        "expect": build_expect(packet["rows"]),
    }


def fixture_launch_stable_with_unbound_evidence() -> dict:
    fixture = fixture_baseline()
    fixture["case_name"] = "launch_stable_with_unbound_evidence_blocks_stable"
    fixture["scenario"] = "A launch_stable quality row with evidence_unbound is refused."
    fixture["input"]["rows"][0]["evidence_class"] = "evidence_unbound"
    fixture["expect"] = build_expect(fixture["input"]["rows"])
    fixture["expect"]["promotion_state"] = "blocks_stable"
    fixture["expect"]["validation_finding_count"] = 2
    fixture["expect"]["expected_finding_kinds"] = [
        "missing_evidence_class",
        "launch_stable_with_unbound_binding",
    ]
    fixture["expect"]["support_export_safe"] = False
    return fixture


def fixture_missing_route_admission() -> dict:
    fixture = fixture_baseline()
    fixture["case_name"] = "missing_route_admission_for_launch_stable_blocks_stable"
    fixture["scenario"] = "The local_lane dropping its blocked_target route admission triggers missing_route_admission."
    rows = fixture["input"]["rows"]
    prefix = "local"
    fixture["input"]["rows"] = [
        r for r in rows
        if not (r["row_id"] == f"row:{prefix}:route:blocked_target")
    ]
    fixture["expect"] = build_expect(fixture["input"]["rows"])
    fixture["expect"]["promotion_state"] = "blocks_stable"
    fixture["expect"]["validation_finding_count"] = 1
    fixture["expect"]["expected_finding_kinds"] = ["missing_route_admission"]
    fixture["expect"]["support_export_safe"] = False
    return fixture


def fixture_missing_reconnect_admission() -> dict:
    fixture = fixture_baseline()
    fixture["case_name"] = "missing_reconnect_admission_for_launch_stable_blocks_stable"
    fixture["scenario"] = "The remote_helper_lane dropping its restore_no_rerun reconnect admission triggers missing_reconnect_admission."
    rows = fixture["input"]["rows"]
    prefix = "remote"
    fixture["input"]["rows"] = [
        r for r in rows
        if not (r["row_id"] == f"row:{prefix}:reconnect:restore_no_rerun")
    ]
    fixture["expect"] = build_expect(fixture["input"]["rows"])
    fixture["expect"]["promotion_state"] = "blocks_stable"
    fixture["expect"]["validation_finding_count"] = 1
    fixture["expect"]["expected_finding_kinds"] = ["missing_reconnect_admission"]
    fixture["expect"]["support_export_safe"] = False
    return fixture


def fixture_missing_degraded_helper_admission() -> dict:
    fixture = fixture_baseline()
    fixture["case_name"] = "missing_degraded_helper_admission_for_launch_stable_blocks_stable"
    fixture["scenario"] = "The enterprise_network_lane dropping its helper_skew degraded-helper admission triggers missing_degraded_helper_admission."
    rows = fixture["input"]["rows"]
    prefix = "enterprise"
    fixture["input"]["rows"] = [
        r for r in rows
        if not (r["row_id"] == f"row:{prefix}:degraded_helper:helper_skew")
    ]
    fixture["expect"] = build_expect(fixture["input"]["rows"])
    fixture["expect"]["promotion_state"] = "blocks_stable"
    fixture["expect"]["validation_finding_count"] = 1
    fixture["expect"]["expected_finding_kinds"] = ["missing_degraded_helper_admission"]
    fixture["expect"]["support_export_safe"] = False
    return fixture


def fixture_missing_artifact_provenance_admission() -> dict:
    fixture = fixture_baseline()
    fixture["case_name"] = "missing_artifact_provenance_admission_for_launch_stable_blocks_stable"
    fixture["scenario"] = "The local_lane dropping its provenance_missing artifact-provenance admission triggers missing_artifact_provenance_admission."
    rows = fixture["input"]["rows"]
    prefix = "local"
    fixture["input"]["rows"] = [
        r for r in rows
        if not (r["row_id"] == f"row:{prefix}:artifact_provenance:provenance_missing")
    ]
    fixture["expect"] = build_expect(fixture["input"]["rows"])
    fixture["expect"]["promotion_state"] = "blocks_stable"
    fixture["expect"]["validation_finding_count"] = 1
    fixture["expect"]["expected_finding_kinds"] = ["missing_artifact_provenance_admission"]
    fixture["expect"]["support_export_safe"] = False
    return fixture


def fixture_lineage_missing_id() -> dict:
    fixture = fixture_baseline()
    fixture["case_name"] = "lineage_admission_missing_execution_context_id_blocks_stable"
    fixture["scenario"] = "Dropping the execution_context_id_binding on the local_lane lineage row triggers lineage_admission_missing_execution_context_id and missing_lineage_admission."
    rows = fixture["input"]["rows"]
    for r in rows:
        if r["row_id"] == "row:local:lineage_admission":
            r["execution_context_id_binding"] = None
    fixture["expect"] = build_expect(fixture["input"]["rows"])
    fixture["expect"]["promotion_state"] = "blocks_stable"
    fixture["expect"]["validation_finding_count"] = 2
    fixture["expect"]["expected_finding_kinds"] = [
        "lineage_admission_missing_execution_context_id",
        "missing_lineage_admission",
    ]
    fixture["expect"]["support_export_safe"] = False
    return fixture


def fixture_narrowed_row_missing_disclosure() -> dict:
    fixture = fixture_baseline()
    fixture["case_name"] = "narrowed_row_missing_disclosure_ref_blocks_stable"
    fixture["scenario"] = "A launch_stable_below row without a disclosure ref triggers narrowed_row_missing_disclosure_ref and downgrade_automation_missing_disclosure_ref."
    rows = fixture["input"]["rows"]
    for r in rows:
        if r["row_id"] == "row:local:quality":
            r["support_class"] = "launch_stable_below"
            r["disclosure_ref"] = None
    fixture["expect"] = build_expect(fixture["input"]["rows"])
    fixture["expect"]["promotion_state"] = "blocks_stable"
    fixture["expect"]["validation_finding_count"] = 2
    fixture["expect"]["expected_finding_kinds"] = [
        "narrowed_row_missing_disclosure_ref",
        "downgrade_automation_missing_disclosure_ref",
    ]
    fixture["expect"]["support_export_safe"] = False
    return fixture


def fixture_projection_collapses_route_state() -> dict:
    fixture = fixture_baseline()
    fixture["case_name"] = "projection_collapses_route_state_vocabulary_blocks_stable"
    fixture["scenario"] = "A Help/About projection that collapses the route-state vocabulary triggers route_state_vocabulary_collapsed, plus missing_consumer_projection and consumer_projection_drift."
    for p in fixture["input"]["consumer_projections"]:
        if p["consumer_surface"] == "help_about":
            p["preserves_route_state_vocabulary"] = False
    fixture["expect"]["promotion_state"] = "blocks_stable"
    fixture["expect"]["validation_finding_count"] = 3
    fixture["expect"]["expected_finding_kinds"] = [
        "missing_consumer_projection",
        "consumer_projection_drift",
        "route_state_vocabulary_collapsed",
    ]
    fixture["expect"]["support_export_safe"] = False
    return fixture


def fixture_raw_source_material() -> dict:
    fixture = fixture_baseline()
    fixture["case_name"] = "raw_source_material_blocks_stable"
    fixture["scenario"] = "Admitting raw command lines or process environment bytes past the boundary triggers raw_source_material_present."
    rows = fixture["input"]["rows"]
    for r in rows:
        if r["row_id"] == "row:local:quality":
            r["raw_source_material_excluded"] = False
    fixture["expect"] = build_expect(fixture["input"]["rows"])
    fixture["expect"]["promotion_state"] = "blocks_stable"
    fixture["expect"]["validation_finding_count"] = 1
    fixture["expect"]["expected_finding_kinds"] = ["raw_source_material_present"]
    fixture["expect"]["support_export_safe"] = False
    return fixture


def main():
    ARTIFACT_PATH.parent.mkdir(parents=True, exist_ok=True)
    FIXTURE_DIR.mkdir(parents=True, exist_ok=True)

    packet = build_packet()
    with open(ARTIFACT_PATH, "w") as f:
        json.dump(packet, f, indent=2)

    fixtures = [
        fixture_baseline(),
        fixture_launch_stable_with_unbound_evidence(),
        fixture_missing_route_admission(),
        fixture_missing_reconnect_admission(),
        fixture_missing_degraded_helper_admission(),
        fixture_missing_artifact_provenance_admission(),
        fixture_lineage_missing_id(),
        fixture_narrowed_row_missing_disclosure(),
        fixture_projection_collapses_route_state(),
        fixture_raw_source_material(),
    ]

    for fixture in fixtures:
        name = fixture["case_name"]
        path = FIXTURE_DIR / f"{name}.json"
        with open(path, "w") as f:
            json.dump(fixture, f, indent=2)

    readme_path = FIXTURE_DIR / "README.md"
    with open(readme_path, "w") as f:
        f.write("# Execution-plane certification truth fixtures\n\n")
        f.write("This directory contains fixture cases consumed by the integration test.\n\n")
        f.write("| fixture | what it proves |\n")
        f.write("|---|---|\n")
        for fixture in fixtures:
            f.write(f"| `{fixture['case_name']}.json` | {fixture['scenario']} |\n")

    print(f"Wrote {ARTIFACT_PATH}")
    print(f"Wrote {len(fixtures)} fixtures to {FIXTURE_DIR}")


if __name__ == "__main__":
    main()
