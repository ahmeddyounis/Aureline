#!/usr/bin/env python3
"""Regenerate the M4 finalize-request-workspace truth-packet artifact and fixtures.

Emits one canonical stable packet plus a fixture corpus (baseline +
narrowed/blocking postures) that the integration test consumes.
"""

from __future__ import annotations

import copy
import json
import os
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
ARTIFACT_PATH = REPO_ROOT / "artifacts/runtime/m4/finalize_request_workspace_and_api_request_execution_context_truth_packet.json"
FIXTURE_DIR = REPO_ROOT / "fixtures/runtime/m4/finalize_request_workspace_and_api_request_execution_context"

DOC_REF = "docs/runtime/m4/finalize-request-workspace-and-api-request-execution-context.md"
FIXTURE_REF = "fixtures/runtime/m4/finalize_request_workspace_and_api_request_execution_context"
CAPTURED_AT = "2026-05-27T12:00:00Z"

LANES = ["request_workspace_lane", "api_request_lane", "response_trust_lane", "data_action_lane"]
LANE_PREFIX = {
    "request_workspace_lane": "request_workspace",
    "api_request_lane": "api_request",
    "response_trust_lane": "response_trust",
    "data_action_lane": "data_action",
}
WEDGES = ["route_target_truth", "auth_source_truth", "approval_review_truth", "execution_context_reuse_truth"]
AUTH_SOURCES = ["os_keychain", "enterprise_vault", "delegated_identity", "session_only", "workspace_variable", "missing"]
CONNECTION_STATES = ["connected", "constrained", "offline_local_safe", "reauth_required", "reconciliation_pending", "service_unavailable"]
STREAMING_STATES = ["connecting", "headers_received", "streaming", "truncated", "complete", "partial", "timed_out", "policy_blocked"]
CONSUMER_SURFACES = [
    "request_editor_surface",
    "response_timeline_surface",
    "mutation_review_sheet",
    "replay_history_surface",
    "cli_headless_inspect",
    "support_export",
    "help_about",
    "conformance_dashboard",
]

PACKET_ID = "packet:m4:finalize_request_workspace_and_api_request_execution_context"
WORKFLOW_ID = "workflow.runtime.finalize_request_workspace_and_api_request_execution_context"


def disclosure(automation_token: str) -> str:
    return f"{DOC_REF}#{automation_token}"


def row_skeleton(row_id: str, lane: str, row_class: str) -> dict:
    return {
        "row_id": row_id,
        "lane_class": lane,
        "row_class": row_class,
        "support_class": "launch_stable",
        "wedge_class": "not_applicable",
        "auth_source_mode": "not_applicable",
        "connection_state_class": "not_applicable",
        "streaming_response_state_class": "not_applicable",
        "consumer_surface_class": "not_applicable",
        "evidence_class": "conformance_suite_evidence",
        "known_limit_class": "none_declared",
        "downgrade_automation_class": "none",
        "confidence_class": "high_confidence",
        "evidence_refs": [FIXTURE_REF],
        "approval_review_attested": False,
        "silent_deferred_queue_blocked": False,
        "raw_source_material_excluded": True,
        "secrets_excluded": True,
        "ambient_authority_excluded": True,
        "captured_at": CAPTURED_AT,
    }


def quality_row(lane: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(f"row:{prefix}:quality", lane, "execution_context_reuse_quality")
    row["evidence_class"] = "release_evidence_review"
    row["downgrade_automation_class"] = "auto_block_on_missing_evidence"
    row["evidence_refs"] = [DOC_REF, FIXTURE_REF]
    row["disclosure_ref"] = disclosure("auto_block_on_missing_evidence")
    return row


def wedge_row(lane: str, wedge: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(f"row:{prefix}:wedge:{wedge}", lane, "wedge_admission")
    row["wedge_class"] = wedge
    automation = {
        "route_target_truth": "auto_narrow_on_route_target_gap",
        "auth_source_truth": "auto_narrow_on_auth_source_gap",
        "approval_review_truth": "auto_narrow_on_approval_review_drift",
        "execution_context_reuse_truth": "auto_narrow_on_lineage_break",
    }[wedge]
    row["downgrade_automation_class"] = automation
    row["disclosure_ref"] = disclosure(automation)
    if wedge == "approval_review_truth":
        row["approval_review_attested"] = True
    return row


def auth_source_row(lane: str, mode: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(f"row:{prefix}:auth_source:{mode}", lane, "auth_source_admission")
    row["auth_source_mode"] = mode
    row["downgrade_automation_class"] = "auto_narrow_on_auth_source_gap"
    row["disclosure_ref"] = disclosure("auto_narrow_on_auth_source_gap")
    return row


def connection_state_row(lane: str, state: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(f"row:{prefix}:connection_state:{state}", lane, "connection_state_admission")
    row["connection_state_class"] = state
    row["evidence_class"] = "failure_recovery_drill_evidence"
    if state == "reconciliation_pending":
        row["silent_deferred_queue_blocked"] = True
        row["downgrade_automation_class"] = "auto_narrow_on_silent_queue_dispatch"
        row["disclosure_ref"] = disclosure("auto_narrow_on_silent_queue_dispatch")
    else:
        row["downgrade_automation_class"] = "auto_narrow_on_connection_state_gap"
        row["disclosure_ref"] = disclosure("auto_narrow_on_connection_state_gap")
    return row


def streaming_state_row(lane: str, state: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(f"row:{prefix}:streaming_state:{state}", lane, "streaming_response_state_admission")
    row["streaming_response_state_class"] = state
    row["downgrade_automation_class"] = "auto_narrow_on_streaming_state_gap"
    row["disclosure_ref"] = disclosure("auto_narrow_on_streaming_state_gap")
    return row


def consumer_surface_row(lane: str, surface: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(f"row:{prefix}:consumer_surface:{surface}", lane, "consumer_surface_binding")
    row["consumer_surface_class"] = surface
    row["downgrade_automation_class"] = "auto_narrow_on_consumer_surface_gap"
    row["disclosure_ref"] = disclosure("auto_narrow_on_consumer_surface_gap")
    return row


def lineage_row(lane: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(f"row:{prefix}:lineage_admission", lane, "lineage_admission")
    row["evidence_class"] = "automated_functional_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_lineage_break"
    row["disclosure_ref"] = disclosure("auto_narrow_on_lineage_break")
    row["execution_context_id_binding"] = f"exec:m4:request:{prefix}:lineage"
    return row


def lane_rows(lane: str) -> list:
    rows = [quality_row(lane)]
    for wedge in WEDGES:
        rows.append(wedge_row(lane, wedge))
    for mode in AUTH_SOURCES:
        rows.append(auth_source_row(lane, mode))
    for state in CONNECTION_STATES:
        rows.append(connection_state_row(lane, state))
    for state in STREAMING_STATES:
        rows.append(streaming_state_row(lane, state))
    for surface in CONSUMER_SURFACES:
        rows.append(consumer_surface_row(lane, surface))
    rows.append(lineage_row(lane))
    return rows


def projection(surface: str, packet_id: str = PACKET_ID) -> dict:
    return {
        "consumer_surface": surface,
        "projection_ref": f"projection:{surface}",
        "request_execution_context_packet_id_ref": packet_id,
        "rendered_at": "2026-05-27T12:00:01Z",
        "preserves_same_packet": True,
        "preserves_lane_vocabulary": True,
        "preserves_row_class_vocabulary": True,
        "preserves_support_class_vocabulary": True,
        "preserves_wedge_vocabulary": True,
        "preserves_auth_source_vocabulary": True,
        "preserves_connection_state_vocabulary": True,
        "preserves_streaming_response_state_vocabulary": True,
        "preserves_consumer_surface_vocabulary": True,
        "preserves_known_limit_vocabulary": True,
        "preserves_downgrade_automation_vocabulary": True,
        "preserves_evidence_class_vocabulary": True,
        "supports_json_export": True,
        "raw_private_material_excluded": True,
        "ambient_authority_excluded": True,
    }


def build_input(packet_id: str = PACKET_ID, workflow_id: str = WORKFLOW_ID) -> dict:
    rows = []
    for lane in LANES:
        rows.extend(lane_rows(lane))
    return {
        "packet_id": packet_id,
        "workflow_or_surface_id": workflow_id,
        "generated_at": CAPTURED_AT,
        "covered_lanes": list(LANES),
        "rows": rows,
        "consumer_projections": [projection(s, packet_id) for s in CONSUMER_SURFACES],
        "source_contract_refs": [DOC_REF],
    }


def build_canonical_packet() -> dict:
    inp = build_input()
    return {
        "record_kind": "finalize_request_workspace_and_api_request_execution_context_truth_stable_packet",
        "schema_version": 1,
        "packet_id": inp["packet_id"],
        "workflow_or_surface_id": inp["workflow_or_surface_id"],
        "generated_at": inp["generated_at"],
        "covered_lanes": inp["covered_lanes"],
        "rows": inp["rows"],
        "consumer_projections": inp["consumer_projections"],
        "source_contract_refs": inp["source_contract_refs"],
        "promotion_state": "stable",
        "validation_findings": [],
    }


# ---------- fixtures ----------

def fixture_envelope(case_name: str, scenario: str, input_obj: dict, expect: dict) -> dict:
    return {
        "record_kind": "finalize_request_workspace_and_api_request_execution_context_truth_stable_case",
        "schema_version": 1,
        "case_name": case_name,
        "scenario": scenario,
        "input": input_obj,
        "expect": expect,
    }


def baseline_input() -> dict:
    inp = build_input(
        packet_id=f"{PACKET_ID}:baseline_stable",
        workflow_id=f"{WORKFLOW_ID}.baseline_stable",
    )
    return inp


def baseline_expect(row_count: int) -> dict:
    return {
        "promotion_state": "stable",
        "validation_finding_count": 0,
        "row_count": row_count,
        "lane_tokens": LANES,
        "row_class_tokens": [
            "execution_context_reuse_quality",
            "wedge_admission",
            "auth_source_admission",
            "connection_state_admission",
            "streaming_response_state_admission",
            "consumer_surface_binding",
            "lineage_admission",
        ],
        "support_class_tokens": ["launch_stable"],
        "wedge_tokens": [
            "not_applicable",
            "route_target_truth",
            "auth_source_truth",
            "approval_review_truth",
            "execution_context_reuse_truth",
        ],
        "auth_source_tokens": ["not_applicable"] + AUTH_SOURCES,
        "connection_state_tokens": ["not_applicable"] + CONNECTION_STATES,
        "streaming_response_state_tokens": ["not_applicable"] + STREAMING_STATES,
        "consumer_surface_tokens": ["not_applicable"] + CONSUMER_SURFACES,
        "known_limit_tokens": ["none_declared"],
        "downgrade_automation_tokens": [
            "auto_block_on_missing_evidence",
            "auto_narrow_on_route_target_gap",
            "auto_narrow_on_auth_source_gap",
            "auto_narrow_on_approval_review_drift",
            "auto_narrow_on_lineage_break",
            "auto_narrow_on_connection_state_gap",
            "auto_narrow_on_silent_queue_dispatch",
            "auto_narrow_on_streaming_state_gap",
            "auto_narrow_on_consumer_surface_gap",
        ],
        "evidence_class_tokens": [
            "automated_functional_evidence",
            "conformance_suite_evidence",
            "failure_recovery_drill_evidence",
            "release_evidence_review",
        ],
        "support_export_safe": True,
        "expected_finding_kinds": [],
    }


def with_packet_id(inp: dict, packet_id: str, workflow_id: str) -> dict:
    out = copy.deepcopy(inp)
    out["packet_id"] = packet_id
    out["workflow_or_surface_id"] = workflow_id
    for projection_row in out["consumer_projections"]:
        projection_row["request_execution_context_packet_id_ref"] = packet_id
    return out


def fixture_launch_stable_with_unbound_evidence() -> dict:
    inp = with_packet_id(
        baseline_input(),
        f"{PACKET_ID}:launch_stable_with_unbound_evidence",
        f"{WORKFLOW_ID}.launch_stable_with_unbound_evidence",
    )
    inp["rows"][0]["evidence_class"] = "evidence_unbound"
    expect = baseline_expect(len(inp["rows"]))
    expect["promotion_state"] = "blocks_stable"
    expect["validation_finding_count"] = 2
    if "evidence_unbound" not in expect["evidence_class_tokens"]:
        expect["evidence_class_tokens"] = sorted(
            set(expect["evidence_class_tokens"] + ["evidence_unbound"])
        )
    expect["expected_finding_kinds"] = [
        "missing_evidence_class",
        "launch_stable_with_unbound_binding",
    ]
    expect["support_export_safe"] = False
    return fixture_envelope(
        "launch_stable_with_unbound_evidence_blocks_stable",
        "A launch_stable execution_context_reuse_quality row leaves its evidence class unbound; the packet refuses promotion and the support-export wrapper rejects it.",
        inp,
        expect,
    )


def fixture_missing_auth_source() -> dict:
    inp = with_packet_id(
        baseline_input(),
        f"{PACKET_ID}:missing_auth_source",
        f"{WORKFLOW_ID}.missing_auth_source",
    )
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "auth_source_admission"
            and row["auth_source_mode"] == "missing"
            and row["lane_class"] == "api_request_lane"
        )
    ]
    expect = baseline_expect(len(inp["rows"]))
    expect["promotion_state"] = "blocks_stable"
    expect["validation_finding_count"] = 1
    expect["expected_finding_kinds"] = ["missing_auth_source_coverage"]
    expect["support_export_safe"] = False
    return fixture_envelope(
        "missing_auth_source_for_launch_stable_blocks_stable",
        "The api_request_lane drops its `missing` auth-source admission; the packet refuses promotion because every launch_stable lane MUST cover every auth-source mode including the explicit missing-credential cue.",
        inp,
        expect,
    )


def fixture_missing_streaming_state() -> dict:
    inp = with_packet_id(
        baseline_input(),
        f"{PACKET_ID}:missing_streaming_state",
        f"{WORKFLOW_ID}.missing_streaming_state",
    )
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "streaming_response_state_admission"
            and row["streaming_response_state_class"] == "policy_blocked"
            and row["lane_class"] == "response_trust_lane"
        )
    ]
    expect = baseline_expect(len(inp["rows"]))
    expect["promotion_state"] = "blocks_stable"
    expect["validation_finding_count"] = 1
    expect["expected_finding_kinds"] = ["missing_streaming_response_state_coverage"]
    expect["support_export_safe"] = False
    return fixture_envelope(
        "missing_streaming_state_for_launch_stable_blocks_stable",
        "The response_trust_lane drops its `policy_blocked` streaming-response-state admission; the packet refuses promotion because every launch_stable lane MUST cover all eight streaming-response states distinctly.",
        inp,
        expect,
    )


def fixture_approval_review_without_attestation() -> dict:
    inp = with_packet_id(
        baseline_input(),
        f"{PACKET_ID}:approval_review_without_attestation",
        f"{WORKFLOW_ID}.approval_review_without_attestation",
    )
    for row in inp["rows"]:
        if (
            row["row_class"] == "wedge_admission"
            and row["wedge_class"] == "approval_review_truth"
            and row["lane_class"] == "data_action_lane"
        ):
            row["approval_review_attested"] = False
            break
    expect = baseline_expect(len(inp["rows"]))
    expect["promotion_state"] = "blocks_stable"
    expect["validation_finding_count"] = 1
    expect["expected_finding_kinds"] = ["mutation_review_binding_missing_approval"]
    expect["support_export_safe"] = False
    return fixture_envelope(
        "approval_review_truth_without_attestation_blocks_stable",
        "A data_action_lane wedge_admission for approval_review_truth fails to attest mutation-review enforcement; the packet refuses promotion so destructive/data actions never bypass the explicit review sheet.",
        inp,
        expect,
    )


def fixture_silent_deferred_queue() -> dict:
    inp = with_packet_id(
        baseline_input(),
        f"{PACKET_ID}:silent_deferred_queue_admitted",
        f"{WORKFLOW_ID}.silent_deferred_queue_admitted",
    )
    for row in inp["rows"]:
        if (
            row["row_class"] == "connection_state_admission"
            and row["connection_state_class"] == "reconciliation_pending"
            and row["lane_class"] == "api_request_lane"
        ):
            row["silent_deferred_queue_blocked"] = False
            break
    expect = baseline_expect(len(inp["rows"]))
    expect["promotion_state"] = "blocks_stable"
    expect["validation_finding_count"] = 1
    expect["expected_finding_kinds"] = ["silent_deferred_queue_admitted"]
    expect["support_export_safe"] = False
    return fixture_envelope(
        "silent_deferred_queue_admitted_blocks_stable",
        "An api_request_lane reconciliation_pending connection-state admission fails to attest that the lane blocks silent deferred-queue dispatch of non-idempotent or destructive intents; the packet refuses promotion.",
        inp,
        expect,
    )


def fixture_lineage_missing_execution_context_id() -> dict:
    inp = with_packet_id(
        baseline_input(),
        f"{PACKET_ID}:lineage_missing_execution_context_id",
        f"{WORKFLOW_ID}.lineage_missing_execution_context_id",
    )
    for row in inp["rows"]:
        if (
            row["row_class"] == "lineage_admission"
            and row["lane_class"] == "request_workspace_lane"
        ):
            row.pop("execution_context_id_binding", None)
            break
    expect = baseline_expect(len(inp["rows"]))
    expect["promotion_state"] = "blocks_stable"
    expect["validation_finding_count"] = 2
    expect["expected_finding_kinds"] = [
        "lineage_admission_missing_execution_context_id",
        "missing_lineage_admission",
    ]
    expect["support_export_safe"] = False
    return fixture_envelope(
        "lineage_admission_missing_execution_context_id_blocks_stable",
        "The request_workspace_lane lineage_admission row drops its `execution_context_id` binding; the packet refuses promotion because every launch_stable lane MUST thread one stable lineage object through event streams, support packets, approval tickets, and evidence exports.",
        inp,
        expect,
    )


def fixture_narrowed_row_missing_disclosure() -> dict:
    inp = with_packet_id(
        baseline_input(),
        f"{PACKET_ID}:narrowed_row_missing_disclosure",
        f"{WORKFLOW_ID}.narrowed_row_missing_disclosure",
    )
    inp["rows"][0]["support_class"] = "launch_stable_below"
    inp["rows"][0].pop("disclosure_ref", None)
    expect = baseline_expect(len(inp["rows"]))
    expect["promotion_state"] = "blocks_stable"
    expect["validation_finding_count"] = 2
    expect["support_class_tokens"] = sorted(set(expect["support_class_tokens"] + ["launch_stable_below"]))
    expect["expected_finding_kinds"] = [
        "narrowed_row_missing_disclosure_ref",
        "downgrade_automation_missing_disclosure_ref",
    ]
    expect["support_export_safe"] = False
    return fixture_envelope(
        "narrowed_row_missing_disclosure_ref_blocks_stable",
        "A row narrowed to launch_stable_below drops its disclosure_ref; the packet refuses promotion because narrowed rows MUST always surface a disclosure ref so docs/help and release packets explain the gap.",
        inp,
        expect,
    )


def fixture_projection_collapses_streaming_vocab() -> dict:
    inp = with_packet_id(
        baseline_input(),
        f"{PACKET_ID}:projection_collapses_streaming",
        f"{WORKFLOW_ID}.projection_collapses_streaming",
    )
    for projection_row in inp["consumer_projections"]:
        if projection_row["consumer_surface"] == "help_about":
            projection_row["preserves_streaming_response_state_vocabulary"] = False
    expect = baseline_expect(len(inp["rows"]))
    expect["promotion_state"] = "blocks_stable"
    expect["validation_finding_count"] = 3
    expect["expected_finding_kinds"] = [
        "missing_consumer_projection",
        "consumer_projection_drift",
        "streaming_response_state_vocabulary_collapsed",
    ]
    expect["support_export_safe"] = False
    return fixture_envelope(
        "projection_collapses_streaming_response_state_vocabulary_blocks_stable",
        "The Help/About consumer projection collapses the streaming-response-state vocabulary; the packet refuses promotion because every required consumer projection MUST preserve the closed streaming-response-state vocabulary verbatim.",
        inp,
        expect,
    )


def fixture_raw_source_material() -> dict:
    inp = with_packet_id(
        baseline_input(),
        f"{PACKET_ID}:raw_source_material",
        f"{WORKFLOW_ID}.raw_source_material",
    )
    inp["rows"][0]["raw_source_material_excluded"] = False
    expect = baseline_expect(len(inp["rows"]))
    expect["promotion_state"] = "blocks_stable"
    expect["validation_finding_count"] = 1
    expect["expected_finding_kinds"] = ["raw_source_material_present"]
    expect["support_export_safe"] = False
    return fixture_envelope(
        "raw_source_material_blocks_stable",
        "A row admits raw request/response bodies, headers, or cookies past the boundary; the packet refuses promotion because the lane is metadata-only — raw payload material MUST stay behind the boundary.",
        inp,
        expect,
    )


FIXTURES = [
    ("baseline_stable.json", lambda: fixture_envelope(
        "baseline_stable",
        "Baseline stable posture: all four lanes (request_workspace, api_request, response_trust, data_action) publish an execution_context_reuse_quality row at launch_stable plus the full four-wedge admission coverage (route_target_truth, auth_source_truth, approval_review_truth, execution_context_reuse_truth) with approval_review_attested where required, the full six auth-source mode admission coverage (os_keychain, enterprise_vault, delegated_identity, session_only, workspace_variable, missing), the full six connection-state admission coverage (connected, constrained, offline_local_safe, reauth_required, reconciliation_pending with silent_deferred_queue_blocked, service_unavailable), the full eight streaming-response-state admission coverage (connecting, headers_received, streaming, truncated, complete, partial, timed_out, policy_blocked), the full eight consumer-surface binding coverage, and a lineage_admission row binding execution_context_id; every row binds support, known limit, downgrade automation, and evidence classes; narrowed rows carry their disclosure refs; and all eight required consumer projections preserve the packet verbatim.",
        baseline_input(),
        baseline_expect(len(baseline_input()["rows"])),
    )),
    ("launch_stable_with_unbound_evidence_blocks_stable.json", fixture_launch_stable_with_unbound_evidence),
    ("missing_auth_source_for_launch_stable_blocks_stable.json", fixture_missing_auth_source),
    ("missing_streaming_state_for_launch_stable_blocks_stable.json", fixture_missing_streaming_state),
    ("approval_review_truth_without_attestation_blocks_stable.json", fixture_approval_review_without_attestation),
    ("silent_deferred_queue_admitted_blocks_stable.json", fixture_silent_deferred_queue),
    ("lineage_admission_missing_execution_context_id_blocks_stable.json", fixture_lineage_missing_execution_context_id),
    ("narrowed_row_missing_disclosure_ref_blocks_stable.json", fixture_narrowed_row_missing_disclosure),
    ("projection_collapses_streaming_response_state_vocabulary_blocks_stable.json", fixture_projection_collapses_streaming_vocab),
    ("raw_source_material_blocks_stable.json", fixture_raw_source_material),
]


def write_json(path: Path, payload: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w") as fh:
        json.dump(payload, fh, indent=2)
        fh.write("\n")


def main() -> None:
    write_json(ARTIFACT_PATH, build_canonical_packet())
    for name, builder in FIXTURES:
        write_json(FIXTURE_DIR / name, builder())
    readme = FIXTURE_DIR / "README.md"
    readme.write_text(
        "# finalize_request_workspace_and_api_request_execution_context fixture corpus\n\n"
        "Fixture corpus for the M4 stable request-workspace and API-request execution-context reuse truth packet "
        "(`schemas/runtime/finalize_request_workspace_and_api_request_execution_context_truth.schema.json`).\n\n"
        "Each fixture is a `RequestExecutionContextTruthPacketInput` with an `expect` block that pins the materialized "
        "packet's promotion state, finding count, lane and row-class token sets, support-class, wedge, auth-source, "
        "connection-state, streaming-response-state, consumer-surface, known-limit, downgrade-automation, and "
        "evidence-class tokens, and the support-export safety verdict. Tests in "
        "`crates/aureline-runtime/tests/finalize_request_workspace_and_api_request_execution_context_truth_packet.rs` "
        "load each case and assert that materialization matches the expectation block.\n\n"
        "Regenerate via:\n\n"
        "```bash\npython3 tools/regenerate_finalize_request_workspace_and_api_request_execution_context_truth_packet.py\n```\n"
    )


if __name__ == "__main__":
    main()
