#!/usr/bin/env python3
"""Regenerate the M4 build-target hardening truth-packet artifact and fixtures.

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
    / "artifacts/runtime/m4/harden_build_target_discovery_adapter_confidence_labels_and_truth_packet.json"
)
FIXTURE_DIR = (
    REPO_ROOT
    / "fixtures/runtime/m4/harden_build_target_discovery_adapter_confidence_labels_and"
)

DOC_REF = "docs/runtime/m4/harden-build-target-discovery-adapter-confidence-labels-and.md"
FIXTURE_REF = "fixtures/runtime/m4/harden_build_target_discovery_adapter_confidence_labels_and"
CAPTURED_AT = "2026-05-27T12:00:00Z"

LANES = ["run_lane", "test_lane", "debug_lane", "target_graph_snapshot_lane"]
LANE_PREFIX = {
    "run_lane": "run",
    "test_lane": "test",
    "debug_lane": "debug",
    "target_graph_snapshot_lane": "target_graph_snapshot",
}
WEDGES = [
    "build_target_discovery_truth",
    "adapter_confidence_label_truth",
    "target_graph_snapshot_truth",
    "cross_surface_target_parity_truth",
]
DISCOVERY_SOURCES = [
    "native_protocol",
    "structured_adapter",
    "heuristic_parser",
    "imported_metadata",
    "user_declared",
    "resolver_unavailable",
]
DISCOVERY_FRESHNESS = [
    "fresh_probe",
    "recent_within_session",
    "imported_authoritative",
    "stale_imported",
    "unknown",
]
ADAPTER_LABELS = [
    "adapter_authoritative_match",
    "adapter_probed_consistent",
    "adapter_probed_divergent",
    "adapter_inferred_from_session",
    "adapter_unreachable",
]
TARGET_GRAPH_SNAPSHOTS = [
    "live_snapshot",
    "session_cached_snapshot",
    "imported_snapshot",
    "archived_snapshot",
    "snapshot_unavailable",
]
CONSUMER_SURFACES = [
    "run_surface",
    "test_surface",
    "debug_surface",
    "cli_headless_inspect",
    "support_export",
    "help_about",
    "conformance_dashboard",
]

PACKET_ID = "packet:m4:harden_build_target_discovery_adapter_confidence_labels_and"
WORKFLOW_ID = "workflow.runtime.harden_build_target_discovery_adapter_confidence_labels_and"


def disclosure(automation_token: str) -> str:
    return f"{DOC_REF}#{automation_token}"


def row_skeleton(row_id: str, lane: str, row_class: str) -> dict:
    return {
        "row_id": row_id,
        "lane_class": lane,
        "row_class": row_class,
        "support_class": "launch_stable",
        "wedge_class": "not_applicable",
        "discovery_source_class": "not_applicable",
        "discovery_freshness_class": "not_applicable",
        "adapter_confidence_label_class": "not_applicable",
        "target_graph_snapshot_class": "not_applicable",
        "consumer_surface_class": "not_applicable",
        "evidence_class": "conformance_suite_evidence",
        "known_limit_class": "none_declared",
        "downgrade_automation_class": "none",
        "confidence_class": "high_confidence",
        "evidence_refs": [FIXTURE_REF],
        "cross_surface_target_parity_attested": False,
        "raw_source_material_excluded": True,
        "secrets_excluded": True,
        "ambient_authority_excluded": True,
        "captured_at": CAPTURED_AT,
    }


def quality_row(lane: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(f"row:{prefix}:quality", lane, "build_target_hardening_quality")
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
        "build_target_discovery_truth": "auto_narrow_on_discovery_source_gap",
        "adapter_confidence_label_truth": "auto_narrow_on_adapter_confidence_label_gap",
        "target_graph_snapshot_truth": "auto_narrow_on_target_graph_snapshot_gap",
        "cross_surface_target_parity_truth": "auto_narrow_on_cross_surface_target_drift",
    }[wedge]
    row["downgrade_automation_class"] = automation
    row["disclosure_ref"] = disclosure(automation)
    if wedge == "cross_surface_target_parity_truth":
        row["cross_surface_target_parity_attested"] = True
    return row


def discovery_source_row(lane: str, source: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(
        f"row:{prefix}:discovery_source:{source}", lane, "discovery_source_admission"
    )
    row["discovery_source_class"] = source
    row["downgrade_automation_class"] = "auto_narrow_on_discovery_source_gap"
    row["disclosure_ref"] = disclosure("auto_narrow_on_discovery_source_gap")
    return row


def discovery_freshness_row(lane: str, freshness: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(
        f"row:{prefix}:discovery_freshness:{freshness}",
        lane,
        "discovery_freshness_admission",
    )
    row["discovery_freshness_class"] = freshness
    row["evidence_class"] = "failure_recovery_drill_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_discovery_freshness_gap"
    row["disclosure_ref"] = disclosure("auto_narrow_on_discovery_freshness_gap")
    return row


def adapter_label_row(lane: str, label: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(
        f"row:{prefix}:adapter_label:{label}", lane, "adapter_confidence_label_admission"
    )
    row["adapter_confidence_label_class"] = label
    row["downgrade_automation_class"] = "auto_narrow_on_adapter_confidence_label_gap"
    row["disclosure_ref"] = disclosure("auto_narrow_on_adapter_confidence_label_gap")
    return row


def snapshot_row(lane: str, snapshot: str) -> dict:
    prefix = LANE_PREFIX[lane]
    row = row_skeleton(
        f"row:{prefix}:target_graph_snapshot:{snapshot}",
        lane,
        "target_graph_snapshot_admission",
    )
    row["target_graph_snapshot_class"] = snapshot
    row["evidence_class"] = "fixture_repo_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_target_graph_snapshot_gap"
    row["disclosure_ref"] = disclosure("auto_narrow_on_target_graph_snapshot_gap")
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
    row["execution_context_id_binding"] = f"exec:m4:build_target:{prefix}:lineage"
    return row


def lane_rows(lane: str) -> list:
    rows = [quality_row(lane)]
    for wedge in WEDGES:
        rows.append(wedge_row(lane, wedge))
    for source in DISCOVERY_SOURCES:
        rows.append(discovery_source_row(lane, source))
    for freshness in DISCOVERY_FRESHNESS:
        rows.append(discovery_freshness_row(lane, freshness))
    for label in ADAPTER_LABELS:
        rows.append(adapter_label_row(lane, label))
    for snapshot in TARGET_GRAPH_SNAPSHOTS:
        rows.append(snapshot_row(lane, snapshot))
    for surface in CONSUMER_SURFACES:
        rows.append(consumer_surface_row(lane, surface))
    rows.append(lineage_row(lane))
    return rows


def projection(surface: str, packet_id: str = PACKET_ID) -> dict:
    return {
        "consumer_surface": surface,
        "projection_ref": f"projection:{surface}",
        "build_target_hardening_packet_id_ref": packet_id,
        "rendered_at": "2026-05-27T12:00:01Z",
        "preserves_same_packet": True,
        "preserves_lane_vocabulary": True,
        "preserves_row_class_vocabulary": True,
        "preserves_support_class_vocabulary": True,
        "preserves_wedge_vocabulary": True,
        "preserves_discovery_source_vocabulary": True,
        "preserves_discovery_freshness_vocabulary": True,
        "preserves_adapter_confidence_label_vocabulary": True,
        "preserves_target_graph_snapshot_vocabulary": True,
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
        "record_kind": "harden_build_target_discovery_adapter_confidence_labels_and_truth_stable_packet",
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
        "record_kind": "harden_build_target_discovery_adapter_confidence_labels_and_truth_stable_case",
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
            "build_target_hardening_quality",
            "wedge_admission",
            "discovery_source_admission",
            "discovery_freshness_admission",
            "adapter_confidence_label_admission",
            "target_graph_snapshot_admission",
            "consumer_surface_binding",
            "lineage_admission",
        ],
        "support_class_tokens": ["launch_stable"],
        "wedge_tokens": ["not_applicable"] + WEDGES,
        "discovery_source_tokens": ["not_applicable"] + DISCOVERY_SOURCES,
        "discovery_freshness_tokens": ["not_applicable"] + DISCOVERY_FRESHNESS,
        "adapter_confidence_label_tokens": ["not_applicable"] + ADAPTER_LABELS,
        "target_graph_snapshot_tokens": ["not_applicable"] + TARGET_GRAPH_SNAPSHOTS,
        "consumer_surface_tokens": ["not_applicable"] + CONSUMER_SURFACES,
        "known_limit_tokens": ["none_declared"],
        "downgrade_automation_tokens": [
            "auto_block_on_missing_evidence",
            "auto_narrow_on_discovery_source_gap",
            "auto_narrow_on_adapter_confidence_label_gap",
            "auto_narrow_on_target_graph_snapshot_gap",
            "auto_narrow_on_cross_surface_target_drift",
            "auto_narrow_on_discovery_freshness_gap",
            "auto_narrow_on_consumer_surface_gap",
            "auto_narrow_on_lineage_break",
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
        projection_row["build_target_hardening_packet_id_ref"] = packet_id
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
        "A launch_stable build_target_hardening_quality row leaves its evidence class unbound; the packet refuses promotion and the support-export wrapper rejects it.",
        inp,
        expect,
    )


def fixture_missing_discovery_source() -> dict:
    inp = with_packet_id(
        baseline_input(),
        f"{PACKET_ID}:missing_discovery_source",
        f"{WORKFLOW_ID}.missing_discovery_source",
    )
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "discovery_source_admission"
            and row["discovery_source_class"] == "resolver_unavailable"
            and row["lane_class"] == "run_lane"
        )
    ]
    expect = baseline_expect(len(inp["rows"]))
    expect["promotion_state"] = "blocks_stable"
    expect["validation_finding_count"] = 1
    expect["expected_finding_kinds"] = ["missing_discovery_source_coverage"]
    expect["support_export_safe"] = False
    return fixture_envelope(
        "missing_discovery_source_for_launch_stable_blocks_stable",
        "The run_lane drops its `resolver_unavailable` discovery-source admission; the packet refuses promotion because every launch_stable lane MUST cover every discovery source including the explicit resolver-unavailable cue.",
        inp,
        expect,
    )


def fixture_missing_adapter_label() -> dict:
    inp = with_packet_id(
        baseline_input(),
        f"{PACKET_ID}:missing_adapter_label",
        f"{WORKFLOW_ID}.missing_adapter_label",
    )
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "adapter_confidence_label_admission"
            and row["adapter_confidence_label_class"] == "adapter_unreachable"
            and row["lane_class"] == "debug_lane"
        )
    ]
    expect = baseline_expect(len(inp["rows"]))
    expect["promotion_state"] = "blocks_stable"
    expect["validation_finding_count"] = 1
    expect["expected_finding_kinds"] = ["missing_adapter_confidence_label_coverage"]
    expect["support_export_safe"] = False
    return fixture_envelope(
        "missing_adapter_confidence_label_for_launch_stable_blocks_stable",
        "The debug_lane drops its `adapter_unreachable` adapter-confidence label admission; the packet refuses promotion because every launch_stable lane MUST distinctly carry every adapter-confidence label including the unreachable cue.",
        inp,
        expect,
    )


def fixture_missing_target_graph_snapshot() -> dict:
    inp = with_packet_id(
        baseline_input(),
        f"{PACKET_ID}:missing_target_graph_snapshot",
        f"{WORKFLOW_ID}.missing_target_graph_snapshot",
    )
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "target_graph_snapshot_admission"
            and row["target_graph_snapshot_class"] == "snapshot_unavailable"
            and row["lane_class"] == "target_graph_snapshot_lane"
        )
    ]
    expect = baseline_expect(len(inp["rows"]))
    expect["promotion_state"] = "blocks_stable"
    expect["validation_finding_count"] = 1
    expect["expected_finding_kinds"] = ["missing_target_graph_snapshot_coverage"]
    expect["support_export_safe"] = False
    return fixture_envelope(
        "missing_target_graph_snapshot_for_launch_stable_blocks_stable",
        "The target_graph_snapshot_lane drops its `snapshot_unavailable` admission; the packet refuses promotion because every launch_stable lane MUST cover every target-graph snapshot class so downstream dispatch never silently widens scope on a missing snapshot.",
        inp,
        expect,
    )


def fixture_cross_surface_target_parity_without_attestation() -> dict:
    inp = with_packet_id(
        baseline_input(),
        f"{PACKET_ID}:cross_surface_target_parity_without_attestation",
        f"{WORKFLOW_ID}.cross_surface_target_parity_without_attestation",
    )
    for row in inp["rows"]:
        if (
            row["row_class"] == "wedge_admission"
            and row["wedge_class"] == "cross_surface_target_parity_truth"
            and row["lane_class"] == "debug_lane"
        ):
            row["cross_surface_target_parity_attested"] = False
            break
    expect = baseline_expect(len(inp["rows"]))
    expect["promotion_state"] = "blocks_stable"
    expect["validation_finding_count"] = 1
    expect["expected_finding_kinds"] = ["cross_surface_target_parity_not_attested"]
    expect["support_export_safe"] = False
    return fixture_envelope(
        "cross_surface_target_parity_without_attestation_blocks_stable",
        "A debug_lane wedge_admission for cross_surface_target_parity_truth fails to attest run/test/debug/snapshot target parity; the packet refuses promotion so surfaces cannot quietly fork local copies of target identity.",
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
            and row["lane_class"] == "run_lane"
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
        "The run_lane lineage_admission row drops its `execution_context_id` binding; the packet refuses promotion because every launch_stable lane MUST thread one stable lineage object through emitted target-graph snapshots, support packets, and evidence exports.",
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


def fixture_projection_collapses_adapter_label() -> dict:
    inp = with_packet_id(
        baseline_input(),
        f"{PACKET_ID}:projection_collapses_adapter_label",
        f"{WORKFLOW_ID}.projection_collapses_adapter_label",
    )
    for projection_row in inp["consumer_projections"]:
        if projection_row["consumer_surface"] == "help_about":
            projection_row["preserves_adapter_confidence_label_vocabulary"] = False
    expect = baseline_expect(len(inp["rows"]))
    expect["promotion_state"] = "blocks_stable"
    expect["validation_finding_count"] = 3
    expect["expected_finding_kinds"] = [
        "missing_consumer_projection",
        "consumer_projection_drift",
        "adapter_confidence_label_vocabulary_collapsed",
    ]
    expect["support_export_safe"] = False
    return fixture_envelope(
        "projection_collapses_adapter_confidence_label_vocabulary_blocks_stable",
        "The Help/About consumer projection collapses the adapter-confidence label vocabulary; the packet refuses promotion because every required consumer projection MUST preserve the closed adapter-confidence label vocabulary verbatim.",
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
        "A row admits raw discovery payloads, adapter handshake bodies, or command lines past the boundary; the packet refuses promotion because the lane is metadata-only — raw discovery material MUST stay behind the boundary.",
        inp,
        expect,
    )


FIXTURES = [
    (
        "baseline_stable.json",
        lambda: fixture_envelope(
            "baseline_stable",
            "Baseline stable posture: all four lanes (run_lane, test_lane, debug_lane, target_graph_snapshot_lane) publish a build_target_hardening_quality row at launch_stable plus the full four-wedge admission coverage (build_target_discovery_truth, adapter_confidence_label_truth, target_graph_snapshot_truth, cross_surface_target_parity_truth with cross_surface_target_parity_attested), the full six discovery-source admission coverage (native_protocol, structured_adapter, heuristic_parser, imported_metadata, user_declared, resolver_unavailable), the full five discovery-freshness admission coverage (fresh_probe, recent_within_session, imported_authoritative, stale_imported, unknown), the full five adapter-confidence label admission coverage (adapter_authoritative_match, adapter_probed_consistent, adapter_probed_divergent, adapter_inferred_from_session, adapter_unreachable), the full five target-graph snapshot admission coverage (live_snapshot, session_cached_snapshot, imported_snapshot, archived_snapshot, snapshot_unavailable), the full seven consumer-surface binding coverage, and a lineage_admission row binding execution_context_id; every row binds support, known limit, downgrade automation, and evidence classes; narrowed rows carry their disclosure refs; and all seven required consumer projections preserve the packet verbatim.",
            baseline_input(),
            baseline_expect(len(baseline_input()["rows"])),
        ),
    ),
    (
        "launch_stable_with_unbound_evidence_blocks_stable.json",
        fixture_launch_stable_with_unbound_evidence,
    ),
    (
        "missing_discovery_source_for_launch_stable_blocks_stable.json",
        fixture_missing_discovery_source,
    ),
    (
        "missing_adapter_confidence_label_for_launch_stable_blocks_stable.json",
        fixture_missing_adapter_label,
    ),
    (
        "missing_target_graph_snapshot_for_launch_stable_blocks_stable.json",
        fixture_missing_target_graph_snapshot,
    ),
    (
        "cross_surface_target_parity_without_attestation_blocks_stable.json",
        fixture_cross_surface_target_parity_without_attestation,
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
        "projection_collapses_adapter_confidence_label_vocabulary_blocks_stable.json",
        fixture_projection_collapses_adapter_label,
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
        "# harden_build_target_discovery_adapter_confidence_labels_and fixture corpus\n\n"
        "Fixture corpus for the M4 stable build-target hardening truth packet "
        "(`schemas/runtime/harden_build_target_discovery_adapter_confidence_labels_and_truth.schema.json`).\n\n"
        "Each fixture is a `BuildTargetHardeningTruthPacketInput` with an `expect` block that pins the "
        "materialized packet's promotion state, finding count, lane and row-class token sets, support-class, "
        "wedge, discovery-source, discovery-freshness, adapter-confidence label, target-graph snapshot, "
        "consumer-surface, known-limit, downgrade-automation, and evidence-class tokens, and the support-export "
        "safety verdict. Tests in "
        "`crates/aureline-runtime/tests/harden_build_target_discovery_adapter_confidence_labels_and_truth_packet.rs` "
        "load each case and assert that materialization matches the expectation block.\n\n"
        "Regenerate via:\n\n"
        "```bash\npython3 tools/regenerate_harden_build_target_discovery_adapter_confidence_labels_and_truth_packet.py\n```\n"
    )


if __name__ == "__main__":
    main()
