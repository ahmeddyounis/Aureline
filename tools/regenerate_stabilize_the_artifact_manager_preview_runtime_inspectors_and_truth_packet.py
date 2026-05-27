#!/usr/bin/env python3
"""Regenerate the M4 artifact-manager / preview-runtime-inspector / evidence-export truth-packet.

Emits one canonical stable packet plus a fixture corpus (baseline +
narrowed/blocking postures) that the integration test consumes.
"""

from __future__ import annotations

import copy
import json
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
ARTIFACT_PATH = (
    REPO_ROOT
    / "artifacts/runtime/m4/stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth_packet.json"
)
FIXTURE_DIR = (
    REPO_ROOT
    / "fixtures/runtime/m4/stabilize_the_artifact_manager_preview_runtime_inspectors_and"
)

DOC_REF = "docs/runtime/m4/stabilize-the-artifact-manager-preview-runtime-inspectors-and.md"
FIXTURE_REF = "fixtures/runtime/m4/stabilize_the_artifact_manager_preview_runtime_inspectors_and"
CAPTURED_AT = "2026-05-27T12:00:00Z"

LANES = [
    "artifact_manager_lane",
    "preview_runtime_inspector_lane",
    "signal_slice_lane",
    "evidence_export_lane",
]
LANE_PREFIX = {
    "artifact_manager_lane": "artifact_manager",
    "preview_runtime_inspector_lane": "preview_runtime_inspector",
    "signal_slice_lane": "signal_slice",
    "evidence_export_lane": "evidence_export",
}
WEDGES = [
    "artifact_chronology_replay_truth",
    "signal_slice_identity_truth",
    "evidence_export_review_truth",
    "cross_surface_evidence_lineage_truth",
]
SIGNAL_SLICE_KINDS = [
    "logs_slice",
    "metrics_slice",
    "traces_slice",
    "test_artifact_slice",
]
SLICE_FRESHNESS = [
    "live_stream",
    "buffered_replay",
    "cached_snapshot",
    "imported_evidence",
    "truncated_view",
    "exported_copy",
]
REPLAY_CHRONOLOGY_STATES = [
    "recorded",
    "not_recorded",
    "unsupported",
    "restart_with_recording_available",
    "partially_recorded",
]
RETENTION_CLASSES = [
    "session_only_retention",
    "session_plus_window_retention",
    "policy_bounded_retention",
    "archived_retention",
    "imported_external_retention",
]
CONSUMER_SURFACES = [
    "artifact_manager_surface",
    "preview_runtime_inspector_surface",
    "evidence_export_sheet_surface",
    "cli_headless_inspect",
    "support_export",
    "help_about",
    "conformance_dashboard",
]

PACKET_ID = "packet:m4:stabilize_the_artifact_manager_preview_runtime_inspectors_and"
WORKFLOW_ID = "workflow.runtime.stabilize_the_artifact_manager_preview_runtime_inspectors_and"


def disclosure(automation_token: str) -> str:
    return f"{DOC_REF}#{automation_token}"


def row_skeleton(row_id: str, lane: str, row_class: str) -> dict:
    return {
        "row_id": row_id,
        "lane_class": lane,
        "row_class": row_class,
        "support_class": "launch_stable",
        "wedge_class": "not_applicable",
        "signal_slice_kind_class": "not_applicable",
        "slice_freshness_class": "not_applicable",
        "replay_chronology_state_class": "not_applicable",
        "retention_class": "not_applicable",
        "consumer_surface_class": "not_applicable",
        "evidence_class": "conformance_suite_evidence",
        "known_limit_class": "none_declared",
        "downgrade_automation_class": "none",
        "confidence_class": "high_confidence",
        "evidence_refs": [FIXTURE_REF],
        "cross_surface_evidence_lineage_attested": False,
        "raw_source_material_excluded": True,
        "secrets_excluded": True,
        "ambient_authority_excluded": True,
        "captured_at": CAPTURED_AT,
    }


def quality_row(lane: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(f"row:{prefix}:quality", lane, "evidence_export_quality")
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
        "artifact_chronology_replay_truth": "auto_narrow_on_replay_chronology_gap",
        "signal_slice_identity_truth": "auto_narrow_on_signal_slice_identity_gap",
        "evidence_export_review_truth": "auto_narrow_on_evidence_export_review_gap",
        "cross_surface_evidence_lineage_truth": "auto_narrow_on_cross_surface_evidence_lineage_drift",
    }[wedge]
    row["downgrade_automation_class"] = automation
    row["disclosure_ref"] = disclosure(automation)
    if wedge == "cross_surface_evidence_lineage_truth":
        row["cross_surface_evidence_lineage_attested"] = True
    return row


def signal_slice_kind_row(lane: str, kind: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(
        f"row:{prefix}:signal_slice_kind:{kind}", lane, "signal_slice_kind_admission"
    )
    row["signal_slice_kind_class"] = kind
    row["downgrade_automation_class"] = "auto_narrow_on_signal_slice_kind_gap"
    row["disclosure_ref"] = disclosure("auto_narrow_on_signal_slice_kind_gap")
    return row


def slice_freshness_row(lane: str, freshness: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(
        f"row:{prefix}:slice_freshness:{freshness}", lane, "slice_freshness_admission"
    )
    row["slice_freshness_class"] = freshness
    row["evidence_class"] = "failure_recovery_drill_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_slice_freshness_gap"
    row["disclosure_ref"] = disclosure("auto_narrow_on_slice_freshness_gap")
    return row


def replay_chronology_row(lane: str, state: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(
        f"row:{prefix}:replay_chronology:{state}", lane, "replay_chronology_admission"
    )
    row["replay_chronology_state_class"] = state
    row["evidence_class"] = "fixture_repo_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_replay_chronology_gap"
    row["disclosure_ref"] = disclosure("auto_narrow_on_replay_chronology_gap")
    return row


def retention_row(lane: str, retention: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(
        f"row:{prefix}:retention:{retention}", lane, "retention_class_admission"
    )
    row["retention_class"] = retention
    row["downgrade_automation_class"] = "auto_narrow_on_retention_class_gap"
    row["disclosure_ref"] = disclosure("auto_narrow_on_retention_class_gap")
    return row


def consumer_surface_row(lane: str, surface: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(
        f"row:{prefix}:consumer_surface:{surface}", lane, "consumer_surface_binding"
    )
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
    row["execution_context_id_binding"] = f"exec:m4:artifact_manager:{prefix}:lineage"
    return row


def lane_rows(lane: str) -> list:
    rows = [quality_row(lane)]
    for wedge in WEDGES:
        rows.append(wedge_row(lane, wedge))
    for kind in SIGNAL_SLICE_KINDS:
        rows.append(signal_slice_kind_row(lane, kind))
    for freshness in SLICE_FRESHNESS:
        rows.append(slice_freshness_row(lane, freshness))
    for state in REPLAY_CHRONOLOGY_STATES:
        rows.append(replay_chronology_row(lane, state))
    for retention in RETENTION_CLASSES:
        rows.append(retention_row(lane, retention))
    for surface in CONSUMER_SURFACES:
        rows.append(consumer_surface_row(lane, surface))
    rows.append(lineage_row(lane))
    return rows


def projection(surface: str, packet_id: str = PACKET_ID) -> dict:
    return {
        "consumer_surface": surface,
        "projection_ref": f"projection:{surface}",
        "evidence_export_packet_id_ref": packet_id,
        "rendered_at": "2026-05-27T12:00:01Z",
        "preserves_same_packet": True,
        "preserves_lane_vocabulary": True,
        "preserves_row_class_vocabulary": True,
        "preserves_support_class_vocabulary": True,
        "preserves_wedge_vocabulary": True,
        "preserves_signal_slice_kind_vocabulary": True,
        "preserves_slice_freshness_vocabulary": True,
        "preserves_replay_chronology_state_vocabulary": True,
        "preserves_retention_class_vocabulary": True,
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
        "record_kind": "stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth_stable_packet",
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
        "record_kind": "stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth_stable_case",
        "schema_version": 1,
        "case_name": case_name,
        "scenario": scenario,
        "input": input_obj,
        "expect": expect,
    }


def baseline_input() -> dict:
    return build_input(
        packet_id=f"{PACKET_ID}:baseline_stable",
        workflow_id=f"{WORKFLOW_ID}.baseline_stable",
    )


def baseline_expect(row_count: int) -> dict:
    return {
        "promotion_state": "stable",
        "validation_finding_count": 0,
        "row_count": row_count,
        "lane_tokens": LANES,
        "row_class_tokens": [
            "evidence_export_quality",
            "wedge_admission",
            "signal_slice_kind_admission",
            "slice_freshness_admission",
            "replay_chronology_admission",
            "retention_class_admission",
            "consumer_surface_binding",
            "lineage_admission",
        ],
        "support_class_tokens": ["launch_stable"],
        "wedge_tokens": ["not_applicable"] + WEDGES,
        "signal_slice_kind_tokens": ["not_applicable"] + SIGNAL_SLICE_KINDS,
        "slice_freshness_tokens": ["not_applicable"] + SLICE_FRESHNESS,
        "replay_chronology_state_tokens": ["not_applicable"] + REPLAY_CHRONOLOGY_STATES,
        "retention_class_tokens": ["not_applicable"] + RETENTION_CLASSES,
        "consumer_surface_tokens": ["not_applicable"] + CONSUMER_SURFACES,
        "known_limit_tokens": ["none_declared"],
        "downgrade_automation_tokens": [
            "auto_block_on_missing_evidence",
            "auto_narrow_on_consumer_surface_gap",
            "auto_narrow_on_cross_surface_evidence_lineage_drift",
            "auto_narrow_on_evidence_export_review_gap",
            "auto_narrow_on_lineage_break",
            "auto_narrow_on_replay_chronology_gap",
            "auto_narrow_on_retention_class_gap",
            "auto_narrow_on_signal_slice_identity_gap",
            "auto_narrow_on_signal_slice_kind_gap",
            "auto_narrow_on_slice_freshness_gap",
        ],
        "evidence_class_tokens": [
            "automated_functional_evidence",
            "conformance_suite_evidence",
            "failure_recovery_drill_evidence",
            "fixture_repo_evidence",
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
        projection_row["evidence_export_packet_id_ref"] = packet_id
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
        "A launch_stable evidence_export_quality row leaves its evidence class unbound; the packet refuses promotion and the support-export wrapper rejects it.",
        inp,
        expect,
    )


def fixture_missing_signal_slice_kind() -> dict:
    inp = with_packet_id(
        baseline_input(),
        f"{PACKET_ID}:missing_signal_slice_kind",
        f"{WORKFLOW_ID}.missing_signal_slice_kind",
    )
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "signal_slice_kind_admission"
            and row["signal_slice_kind_class"] == "traces_slice"
            and row["lane_class"] == "signal_slice_lane"
        )
    ]
    expect = baseline_expect(len(inp["rows"]))
    expect["promotion_state"] = "blocks_stable"
    expect["validation_finding_count"] = 1
    expect["expected_finding_kinds"] = ["missing_signal_slice_kind_coverage"]
    expect["support_export_safe"] = False
    return fixture_envelope(
        "missing_signal_slice_kind_for_launch_stable_blocks_stable",
        "The signal_slice_lane drops its `traces_slice` admission; the packet refuses promotion because every launch_stable lane MUST cover every signal-slice kind so traces are never silently merged into the logs or metrics narrative.",
        inp,
        expect,
    )


def fixture_missing_slice_freshness() -> dict:
    inp = with_packet_id(
        baseline_input(),
        f"{PACKET_ID}:missing_slice_freshness",
        f"{WORKFLOW_ID}.missing_slice_freshness",
    )
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "slice_freshness_admission"
            and row["slice_freshness_class"] == "exported_copy"
            and row["lane_class"] == "preview_runtime_inspector_lane"
        )
    ]
    expect = baseline_expect(len(inp["rows"]))
    expect["promotion_state"] = "blocks_stable"
    expect["validation_finding_count"] = 1
    expect["expected_finding_kinds"] = ["missing_slice_freshness_coverage"]
    expect["support_export_safe"] = False
    return fixture_envelope(
        "missing_slice_freshness_for_launch_stable_blocks_stable",
        "The preview_runtime_inspector_lane drops its `exported_copy` slice-freshness admission; the packet refuses promotion because every launch_stable lane MUST distinguish live stream, buffered replay, cached snapshot, imported evidence, truncated view, and exported copy so users do not mistake an exported copy for live runtime truth.",
        inp,
        expect,
    )


def fixture_missing_replay_chronology_state() -> dict:
    inp = with_packet_id(
        baseline_input(),
        f"{PACKET_ID}:missing_replay_chronology_state",
        f"{WORKFLOW_ID}.missing_replay_chronology_state",
    )
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "replay_chronology_admission"
            and row["replay_chronology_state_class"] == "restart_with_recording_available"
            and row["lane_class"] == "artifact_manager_lane"
        )
    ]
    expect = baseline_expect(len(inp["rows"]))
    expect["promotion_state"] = "blocks_stable"
    expect["validation_finding_count"] = 1
    expect["expected_finding_kinds"] = ["missing_replay_chronology_state_coverage"]
    expect["support_export_safe"] = False
    return fixture_envelope(
        "missing_replay_chronology_state_for_launch_stable_blocks_stable",
        "The artifact_manager_lane drops its `restart_with_recording_available` replay-chronology admission; the packet refuses promotion because every launch_stable lane MUST preserve the `recorded`, `not_recorded`, `unsupported`, `restart_with_recording_available`, and `partially_recorded` chronology states so the lane never silently collapses chronology posture into a single on/off bit.",
        inp,
        expect,
    )


def fixture_cross_surface_evidence_lineage_without_attestation() -> dict:
    inp = with_packet_id(
        baseline_input(),
        f"{PACKET_ID}:cross_surface_evidence_lineage_without_attestation",
        f"{WORKFLOW_ID}.cross_surface_evidence_lineage_without_attestation",
    )
    for row in inp["rows"]:
        if (
            row["row_class"] == "wedge_admission"
            and row["wedge_class"] == "cross_surface_evidence_lineage_truth"
            and row["lane_class"] == "evidence_export_lane"
        ):
            row["cross_surface_evidence_lineage_attested"] = False
            break
    expect = baseline_expect(len(inp["rows"]))
    expect["promotion_state"] = "blocks_stable"
    expect["validation_finding_count"] = 1
    expect["expected_finding_kinds"] = ["cross_surface_evidence_lineage_not_attested"]
    expect["support_export_safe"] = False
    return fixture_envelope(
        "cross_surface_evidence_lineage_without_attestation_blocks_stable",
        "An evidence_export_lane wedge_admission for cross_surface_evidence_lineage_truth fails to attest artifact-manager/inspector/export lineage parity; the packet refuses promotion so surfaces cannot quietly fork local copies of artifact / signal-slice identity.",
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
            and row["lane_class"] == "artifact_manager_lane"
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
        "The artifact_manager_lane lineage_admission row drops its `execution_context_id` binding; the packet refuses promotion because every launch_stable lane MUST thread one stable lineage object through emitted artifacts, signal slices, evidence exports, and support packets.",
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
    expect["support_class_tokens"] = sorted(
        set(expect["support_class_tokens"] + ["launch_stable_below"])
    )
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


def fixture_projection_collapses_slice_freshness() -> dict:
    inp = with_packet_id(
        baseline_input(),
        f"{PACKET_ID}:projection_collapses_slice_freshness",
        f"{WORKFLOW_ID}.projection_collapses_slice_freshness",
    )
    for projection_row in inp["consumer_projections"]:
        if projection_row["consumer_surface"] == "help_about":
            projection_row["preserves_slice_freshness_vocabulary"] = False
    expect = baseline_expect(len(inp["rows"]))
    expect["promotion_state"] = "blocks_stable"
    expect["validation_finding_count"] = 3
    expect["expected_finding_kinds"] = [
        "missing_consumer_projection",
        "consumer_projection_drift",
        "slice_freshness_vocabulary_collapsed",
    ]
    expect["support_export_safe"] = False
    return fixture_envelope(
        "projection_collapses_slice_freshness_vocabulary_blocks_stable",
        "The Help/About consumer projection collapses the slice-freshness vocabulary; the packet refuses promotion because every required consumer projection MUST preserve the closed slice-freshness vocabulary verbatim so users do not mistake exported copies for live runtime truth.",
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
        "A row admits raw log bodies, raw trace payloads, raw test-artifact bytes, or raw command lines past the boundary; the packet refuses promotion because the lane is metadata-only — raw evidence material MUST stay behind the boundary and be referenced through stable IDs and manifests.",
        inp,
        expect,
    )


FIXTURES = [
    (
        "baseline_stable.json",
        lambda: fixture_envelope(
            "baseline_stable",
            "Baseline stable posture: all four lanes (artifact_manager_lane, preview_runtime_inspector_lane, signal_slice_lane, evidence_export_lane) publish an evidence_export_quality row at launch_stable plus the full four-wedge admission coverage (artifact_chronology_replay_truth, signal_slice_identity_truth, evidence_export_review_truth, cross_surface_evidence_lineage_truth with cross_surface_evidence_lineage_attested), the full four signal-slice kind admission coverage (logs_slice, metrics_slice, traces_slice, test_artifact_slice), the full six slice-freshness admission coverage (live_stream, buffered_replay, cached_snapshot, imported_evidence, truncated_view, exported_copy), the full five replay-chronology state admission coverage (recorded, not_recorded, unsupported, restart_with_recording_available, partially_recorded), the full five retention-class admission coverage (session_only_retention, session_plus_window_retention, policy_bounded_retention, archived_retention, imported_external_retention), the full seven consumer-surface binding coverage, and a lineage_admission row binding execution_context_id; every row binds support, known limit, downgrade automation, and evidence classes; narrowed rows carry their disclosure refs; and all seven required consumer projections preserve the packet verbatim.",
            baseline_input(),
            baseline_expect(len(baseline_input()["rows"])),
        ),
    ),
    (
        "launch_stable_with_unbound_evidence_blocks_stable.json",
        fixture_launch_stable_with_unbound_evidence,
    ),
    (
        "missing_signal_slice_kind_for_launch_stable_blocks_stable.json",
        fixture_missing_signal_slice_kind,
    ),
    (
        "missing_slice_freshness_for_launch_stable_blocks_stable.json",
        fixture_missing_slice_freshness,
    ),
    (
        "missing_replay_chronology_state_for_launch_stable_blocks_stable.json",
        fixture_missing_replay_chronology_state,
    ),
    (
        "cross_surface_evidence_lineage_without_attestation_blocks_stable.json",
        fixture_cross_surface_evidence_lineage_without_attestation,
    ),
    (
        "lineage_admission_missing_execution_context_id_blocks_stable.json",
        fixture_lineage_missing_execution_context_id,
    ),
    (
        "narrowed_row_missing_disclosure_ref_blocks_stable.json",
        fixture_narrowed_row_missing_disclosure,
    ),
    (
        "projection_collapses_slice_freshness_vocabulary_blocks_stable.json",
        fixture_projection_collapses_slice_freshness,
    ),
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
        "# stabilize_the_artifact_manager_preview_runtime_inspectors_and fixture corpus\n\n"
        "Fixture corpus for the M4 stable artifact-manager / preview-runtime-inspector / evidence-export truth packet "
        "(`schemas/runtime/stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth.schema.json`).\n\n"
        "Each fixture is an `EvidenceExportTruthPacketInput` with an `expect` block that pins the "
        "materialized packet's promotion state, finding count, lane and row-class token sets, support-class, "
        "wedge, signal-slice kind, slice-freshness, replay-chronology state, retention class, "
        "consumer-surface, known-limit, downgrade-automation, and evidence-class tokens, and the support-export "
        "safety verdict. Tests in "
        "`crates/aureline-runtime/tests/stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth_packet.rs` "
        "load each case and assert that materialization matches the expectation block.\n\n"
        "Regenerate via:\n\n"
        "```bash\npython3 tools/regenerate_stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth_packet.py\n```\n"
    )


if __name__ == "__main__":
    main()
