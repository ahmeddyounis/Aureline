#!/usr/bin/env python3
"""Regenerate the M4 stabilize-the-test-explorer / inline-results / watch-mode / rerun / debug-from-test truth packet artifact and fixture corpus.

Mirrors the Rust unit-test sample input in
crates/aureline-runtime/src/stabilize_the_test_explorer_inline_results_watch_mode/mod.rs.
The generator is the canonical seed for the checked-in artifact and the
narrowed-below-stable fixture cases used by the integration tests in
crates/aureline-runtime/tests/stabilize_the_test_explorer_inline_results_watch_mode_truth_packet.rs.

Run from anywhere:
    python3 tools/regenerate_stabilize_the_test_explorer_inline_results_watch_mode_truth_packet.py
"""
import json
import os

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

DOC_REF = "docs/runtime/m4/stabilize-the-test-explorer-inline-results-watch-mode.md"
FIXTURE_DIR = (
    "fixtures/runtime/m4/stabilize_the_test_explorer_inline_results_watch_mode"
)
ARTIFACT_PATH = (
    "artifacts/runtime/m4/"
    "stabilize_the_test_explorer_inline_results_watch_mode_truth_packet.json"
)
SCHEMA_REF = (
    "schemas/runtime/"
    "stabilize_the_test_explorer_inline_results_watch_mode_truth.schema.json"
)

LANES = [
    ("local_lane", "local"),
    ("remote_helper_lane", "remote"),
    ("container_lane", "container"),
    ("notebook_lane", "notebook"),
]

WEDGES = [
    "test_explorer_identity_truth",
    "inline_results_truth",
    "watch_mode_truth",
    "rerun_debug_from_test_parity",
]

TEST_IDENTITIES = [
    "suite_identity",
    "case_identity",
    "template_identity",
    "invocation_identity",
]

DISCOVERY_POSTURES = [
    "partial_discovery_record",
    "loaded_versus_known_counts",
    "case_enumeration_at_runtime",
]

WATCH_MODE_SUPPORTS = [
    "live",
    "reduced",
    "polling",
    "unavailable",
]

SELECTOR_DURABILITIES = [
    "durable_id_selector",
    "trait_selector",
    "snapshot_scoped_query_selector",
]

CONSUMER_SURFACE_BINDINGS = [
    "test_explorer_surface",
    "inline_results_surface",
    "watch_mode_surface",
    "rerun_surface",
    "debug_from_test_surface",
]

SURFACES_REQUIRING_TEST_IDENTITY_ATTESTATION = {
    "test_explorer_surface",
    "inline_results_surface",
    "watch_mode_surface",
    "rerun_surface",
    "debug_from_test_surface",
}

SURFACES_REQUIRING_WATCH_MODE_SUPPORT_ATTESTATION = {
    "watch_mode_surface",
}

SURFACES_REQUIRING_DURABLE_SELECTOR_ATTESTATION = {
    "rerun_surface",
    "debug_from_test_surface",
}

CONSUMER_SURFACES = [
    "test_explorer_surface",
    "inline_results_surface",
    "watch_mode_surface",
    "rerun_surface",
    "debug_from_test_surface",
    "ai_tool_surface",
    "cli_headless",
    "evidence_export",
    "support_export",
    "release_proof_index",
    "help_about",
    "conformance_dashboard",
]

TIMESTAMP = "2026-05-26T12:00:00Z"


def base_row(row_id, lane, row_class):
    return {
        "row_id": row_id,
        "lane_class": lane,
        "row_class": row_class,
        "support_class": "launch_stable",
        "wedge_class": "not_applicable",
        "test_identity_class": "not_applicable",
        "discovery_posture_class": "not_applicable",
        "watch_mode_support_class": "not_applicable",
        "selector_durability_class": "not_applicable",
        "consumer_surface_class": "not_applicable",
        "evidence_class": "fixture_repo_evidence",
        "known_limit_class": "none_declared",
        "downgrade_automation_class": "auto_narrow_on_lineage_break",
        "confidence_class": "high_confidence",
        "evidence_refs": [FIXTURE_DIR],
        "disclosure_ref": f"{DOC_REF}#auto_narrow_on_lineage_break",
        "attests_test_identity_preserved": False,
        "attests_watch_mode_support_preserved": False,
        "attests_durable_selector_preserved": False,
        "raw_source_material_excluded": True,
        "secrets_excluded": True,
        "ambient_authority_excluded": True,
        "captured_at": TIMESTAMP,
    }


def quality_row(lane, prefix):
    row = base_row(
        f"row:{prefix}:quality", lane, "test_explorer_stabilization_quality"
    )
    row["evidence_class"] = "release_evidence_review"
    row["downgrade_automation_class"] = "auto_block_on_missing_evidence"
    row["disclosure_ref"] = f"{DOC_REF}#auto_block_on_missing_evidence"
    row["evidence_refs"] = [DOC_REF, FIXTURE_DIR]
    return row


def wedge_rows(lane, prefix):
    rows = []
    for wedge in WEDGES:
        row = base_row(f"row:{prefix}:wedge:{wedge}", lane, "wedge_admission")
        row["wedge_class"] = wedge
        row["evidence_class"] = "conformance_suite_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_wedge_admission_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_wedge_admission_gap"
        rows.append(row)
    return rows


def test_identity_rows(lane, prefix):
    rows = []
    for identity in TEST_IDENTITIES:
        row = base_row(
            f"row:{prefix}:test_identity:{identity}", lane, "test_identity_admission"
        )
        row["test_identity_class"] = identity
        row["evidence_class"] = "automated_functional_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_test_identity_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_test_identity_gap"
        rows.append(row)
    return rows


def discovery_posture_rows(lane, prefix):
    rows = []
    for posture in DISCOVERY_POSTURES:
        row = base_row(
            f"row:{prefix}:discovery_posture:{posture}",
            lane,
            "discovery_posture_admission",
        )
        row["discovery_posture_class"] = posture
        row["evidence_class"] = "automated_functional_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_discovery_posture_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_discovery_posture_gap"
        rows.append(row)
    return rows


def watch_mode_support_rows(lane, prefix):
    rows = []
    for support in WATCH_MODE_SUPPORTS:
        row = base_row(
            f"row:{prefix}:watch_mode_support:{support}",
            lane,
            "watch_mode_support_admission",
        )
        row["watch_mode_support_class"] = support
        row["evidence_class"] = "automated_functional_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_watch_mode_support_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_watch_mode_support_gap"
        rows.append(row)
    return rows


def selector_durability_rows(lane, prefix):
    rows = []
    for durability in SELECTOR_DURABILITIES:
        row = base_row(
            f"row:{prefix}:selector_durability:{durability}",
            lane,
            "selector_durability_admission",
        )
        row["selector_durability_class"] = durability
        row["evidence_class"] = "automated_functional_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_selector_durability_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_selector_durability_gap"
        rows.append(row)
    return rows


def consumer_surface_rows(lane, prefix):
    rows = []
    for surface in CONSUMER_SURFACE_BINDINGS:
        row = base_row(
            f"row:{prefix}:consumer_surface:{surface}",
            lane,
            "consumer_surface_binding",
        )
        row["consumer_surface_class"] = surface
        row["evidence_class"] = "conformance_suite_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_consumer_surface_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_consumer_surface_gap"
        row["attests_test_identity_preserved"] = (
            surface in SURFACES_REQUIRING_TEST_IDENTITY_ATTESTATION
        )
        row["attests_watch_mode_support_preserved"] = (
            surface in SURFACES_REQUIRING_WATCH_MODE_SUPPORT_ATTESTATION
        )
        row["attests_durable_selector_preserved"] = (
            surface in SURFACES_REQUIRING_DURABLE_SELECTOR_ATTESTATION
        )
        rows.append(row)
    return rows


def lineage_row(lane, prefix):
    row = base_row(f"row:{prefix}:lineage_admission", lane, "lineage_admission")
    row["evidence_class"] = "automated_functional_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_lineage_break"
    row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_lineage_break"
    row["execution_context_id_binding"] = (
        f"exec:m4:{prefix}:test_explorer_lineage"
    )
    return row


def lane_rows(lane, prefix):
    rows = [quality_row(lane, prefix)]
    rows.extend(wedge_rows(lane, prefix))
    rows.extend(test_identity_rows(lane, prefix))
    rows.extend(discovery_posture_rows(lane, prefix))
    rows.extend(watch_mode_support_rows(lane, prefix))
    rows.extend(selector_durability_rows(lane, prefix))
    rows.extend(consumer_surface_rows(lane, prefix))
    rows.append(lineage_row(lane, prefix))
    return rows


def projection(surface, packet_id):
    return {
        "consumer_surface": surface,
        "projection_ref": f"projection:{surface}",
        "test_explorer_stabilization_truth_packet_id_ref": packet_id,
        "rendered_at": "2026-05-26T12:00:01Z",
        "preserves_same_packet": True,
        "preserves_lane_vocabulary": True,
        "preserves_row_class_vocabulary": True,
        "preserves_support_class_vocabulary": True,
        "preserves_wedge_vocabulary": True,
        "preserves_test_identity_vocabulary": True,
        "preserves_discovery_posture_vocabulary": True,
        "preserves_watch_mode_support_vocabulary": True,
        "preserves_selector_durability_vocabulary": True,
        "preserves_consumer_surface_vocabulary": True,
        "preserves_known_limit_vocabulary": True,
        "preserves_downgrade_automation_vocabulary": True,
        "preserves_evidence_class_vocabulary": True,
        "supports_json_export": True,
        "raw_private_material_excluded": True,
        "ambient_authority_excluded": True,
    }


def baseline_input(packet_id, workflow_id):
    rows = []
    for lane, prefix in LANES:
        rows.extend(lane_rows(lane, prefix))
    return {
        "packet_id": packet_id,
        "workflow_or_surface_id": workflow_id,
        "generated_at": TIMESTAMP,
        "covered_lanes": [lane for lane, _ in LANES],
        "rows": rows,
        "consumer_projections": [
            projection(surface, packet_id) for surface in CONSUMER_SURFACES
        ],
        "source_contract_refs": [DOC_REF, SCHEMA_REF],
    }


def packet_artifact():
    packet_id = (
        "packet:m4:stabilize_the_test_explorer_inline_results_watch_mode:stable"
    )
    workflow_id = (
        "workflow.runtime.stabilize_the_test_explorer_inline_results_watch_mode.stable"
    )
    base = baseline_input(packet_id, workflow_id)
    base.update(
        {
            "record_kind": (
                "stabilize_the_test_explorer_inline_results_watch_mode_truth_stable_packet"
            ),
            "schema_version": 1,
            "promotion_state": "stable",
            "validation_findings": [],
        }
    )
    keys = [
        "record_kind",
        "schema_version",
        "packet_id",
        "workflow_or_surface_id",
        "generated_at",
        "covered_lanes",
        "rows",
        "consumer_projections",
        "source_contract_refs",
        "promotion_state",
        "validation_findings",
    ]
    return {key: base[key] for key in keys}


def expectations_from_input(
    input_obj, promotion_state, finding_count, expected_findings=None
):
    rows = input_obj["rows"]
    lane_tokens = sorted({row["lane_class"] for row in rows})
    row_class_tokens = sorted({row["row_class"] for row in rows})
    support_class_tokens = sorted({row["support_class"] for row in rows})
    wedge_tokens = sorted({row["wedge_class"] for row in rows})
    test_identity_tokens = sorted({row["test_identity_class"] for row in rows})
    discovery_posture_tokens = sorted(
        {row["discovery_posture_class"] for row in rows}
    )
    watch_mode_support_tokens = sorted(
        {row["watch_mode_support_class"] for row in rows}
    )
    selector_durability_tokens = sorted(
        {row["selector_durability_class"] for row in rows}
    )
    consumer_surface_tokens = sorted(
        {row["consumer_surface_class"] for row in rows}
    )
    known_limit_tokens = sorted({row["known_limit_class"] for row in rows})
    downgrade_automation_tokens = sorted(
        {row["downgrade_automation_class"] for row in rows}
    )
    evidence_class_tokens = sorted({row["evidence_class"] for row in rows})
    expect = {
        "promotion_state": promotion_state,
        "validation_finding_count": finding_count,
        "row_count": len(rows),
        "lane_tokens": lane_tokens,
        "row_class_tokens": row_class_tokens,
        "support_class_tokens": support_class_tokens,
        "wedge_tokens": wedge_tokens,
        "test_identity_tokens": test_identity_tokens,
        "discovery_posture_tokens": discovery_posture_tokens,
        "watch_mode_support_tokens": watch_mode_support_tokens,
        "selector_durability_tokens": selector_durability_tokens,
        "consumer_surface_tokens": consumer_surface_tokens,
        "known_limit_tokens": known_limit_tokens,
        "downgrade_automation_tokens": downgrade_automation_tokens,
        "evidence_class_tokens": evidence_class_tokens,
        "support_export_safe": promotion_state == "stable",
    }
    if expected_findings:
        expect["expected_finding_kinds"] = expected_findings
    return expect


def baseline_case():
    pid = (
        "packet:m4:stabilize_the_test_explorer_inline_results_watch_mode:baseline_stable"
    )
    wid = (
        "workflow.runtime.stabilize_the_test_explorer_inline_results_watch_mode."
        "baseline_stable"
    )
    inp = baseline_input(pid, wid)
    return {
        "record_kind": (
            "stabilize_the_test_explorer_inline_results_watch_mode_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "baseline_stable",
        "scenario": (
            "Baseline stable posture: all four test-explorer lanes (local, "
            "remote_helper, container, notebook) publish a "
            "test_explorer_stabilization_quality row at launch_stable plus "
            "the full four-wedge admission coverage "
            "(test_explorer_identity_truth, inline_results_truth, "
            "watch_mode_truth, rerun_debug_from_test_parity), the full four "
            "test-identity admissions (suite_identity, case_identity, "
            "template_identity, invocation_identity), the full three "
            "discovery-posture admissions (partial_discovery_record, "
            "loaded_versus_known_counts, case_enumeration_at_runtime), the "
            "full four watch-mode support admissions (live, reduced, "
            "polling, unavailable), the full three selector-durability "
            "admissions (durable_id_selector, trait_selector, "
            "snapshot_scoped_query_selector), the full five "
            "consumer-surface bindings (test_explorer_surface, "
            "inline_results_surface, watch_mode_surface, rerun_surface, "
            "debug_from_test_surface) each attesting the vocabularies it "
            "is required to preserve, and a lineage_admission row binding "
            "execution_context_id; every row binds support, known limit, "
            "downgrade automation, and evidence classes; narrowed rows "
            "carry their disclosure refs; and all twelve required consumer "
            "projections preserve the packet verbatim."
        ),
        "input": inp,
        "expect": expectations_from_input(inp, "stable", 0),
    }


def unbound_evidence_case():
    pid = (
        "packet:m4:stabilize_the_test_explorer_inline_results_watch_mode:"
        "launch_stable_with_unbound_evidence"
    )
    wid = (
        "workflow.runtime.stabilize_the_test_explorer_inline_results_watch_mode."
        "launch_stable_with_unbound_evidence"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:local:quality":
            row["evidence_class"] = "evidence_unbound"
            break
    return {
        "record_kind": (
            "stabilize_the_test_explorer_inline_results_watch_mode_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "launch_stable_with_unbound_evidence_blocks_stable",
        "scenario": (
            "The local lane's test_explorer_stabilization_quality row claims "
            "launch_stable while its evidence class is evidence_unbound; the "
            "packet blocks the stable claim because no automated_functional, "
            "conformance_suite, failure_recovery_drill, "
            "release_evidence_review, fixture_repo, benchmark, design_qa, "
            "or docs_disclosure evidence backs the row."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            2,
            expected_findings=[
                "missing_evidence_class",
                "launch_stable_with_unbound_binding",
            ],
        ),
    }


def missing_watch_mode_support_case():
    pid = (
        "packet:m4:stabilize_the_test_explorer_inline_results_watch_mode:"
        "missing_watch_mode_support_for_launch_stable"
    )
    wid = (
        "workflow.runtime.stabilize_the_test_explorer_inline_results_watch_mode."
        "missing_watch_mode_support_for_launch_stable"
    )
    inp = baseline_input(pid, wid)
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "watch_mode_support_admission"
            and row["watch_mode_support_class"] == "polling"
            and row["lane_class"] == "local_lane"
        )
    ]
    return {
        "record_kind": (
            "stabilize_the_test_explorer_inline_results_watch_mode_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "missing_watch_mode_support_for_launch_stable_blocks_stable",
        "scenario": (
            "The local lane claims launch_stable but drops its polling "
            "watch_mode_support_admission row; the packet blocks the stable "
            "claim because every launch_stable lane MUST admit all four "
            "watch-mode support classes (live, reduced, polling, "
            "unavailable) so the surface cannot collapse the vocabulary "
            "down to watch on / off."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            1,
            expected_findings=["missing_watch_mode_support_coverage"],
        ),
    }


def missing_selector_durability_case():
    pid = (
        "packet:m4:stabilize_the_test_explorer_inline_results_watch_mode:"
        "missing_selector_durability_for_launch_stable"
    )
    wid = (
        "workflow.runtime.stabilize_the_test_explorer_inline_results_watch_mode."
        "missing_selector_durability_for_launch_stable"
    )
    inp = baseline_input(pid, wid)
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "selector_durability_admission"
            and row["selector_durability_class"] == "snapshot_scoped_query_selector"
            and row["lane_class"] == "local_lane"
        )
    ]
    return {
        "record_kind": (
            "stabilize_the_test_explorer_inline_results_watch_mode_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "missing_selector_durability_for_launch_stable_blocks_stable",
        "scenario": (
            "The local lane claims launch_stable but drops its "
            "snapshot_scoped_query_selector selector_durability_admission "
            "row; the packet blocks the stable claim because rerun, "
            "debug-from-test, saved selectors, AI tool plans, and exported "
            "test packets MUST operate on durable IDs, traits, or "
            "snapshot-scoped queries instead of display labels or "
            "transient row order."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            1,
            expected_findings=["missing_selector_durability_coverage"],
        ),
    }


def consumer_surface_missing_durable_selector_case():
    pid = (
        "packet:m4:stabilize_the_test_explorer_inline_results_watch_mode:"
        "consumer_surface_missing_durable_selector_attestation"
    )
    wid = (
        "workflow.runtime.stabilize_the_test_explorer_inline_results_watch_mode."
        "consumer_surface_missing_durable_selector_attestation"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if (
            row["row_class"] == "consumer_surface_binding"
            and row["lane_class"] == "local_lane"
            and row["consumer_surface_class"] == "rerun_surface"
        ):
            row["attests_durable_selector_preserved"] = False
            break
    return {
        "record_kind": (
            "stabilize_the_test_explorer_inline_results_watch_mode_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": (
            "consumer_surface_missing_durable_selector_attestation_blocks_stable"
        ),
        "scenario": (
            "The local lane's rerun_surface consumer_surface_binding row "
            "stops attesting durable-selector preservation; the packet "
            "blocks the stable claim because rerun_surface and "
            "debug_from_test_surface MUST attest preservation of the "
            "durable-selector vocabulary so saved selectors, AI tool plans, "
            "and exported test packets keep referencing durable IDs / "
            "traits / snapshot-scoped queries instead of display labels."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            2,
            expected_findings=[
                "consumer_surface_missing_durable_selector_attestation",
                "missing_consumer_surface_coverage",
            ],
        ),
    }


def narrowed_row_missing_disclosure_ref_case():
    pid = (
        "packet:m4:stabilize_the_test_explorer_inline_results_watch_mode:"
        "narrowed_row_missing_disclosure_ref"
    )
    wid = (
        "workflow.runtime.stabilize_the_test_explorer_inline_results_watch_mode."
        "narrowed_row_missing_disclosure_ref"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:local:quality":
            row["support_class"] = "launch_stable_below"
            row.pop("disclosure_ref", None)
            break
    return {
        "record_kind": (
            "stabilize_the_test_explorer_inline_results_watch_mode_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "narrowed_row_missing_disclosure_ref_blocks_stable",
        "scenario": (
            "The local lane's test_explorer_stabilization_quality row "
            "narrows to launch_stable_below but drops its disclosure ref; "
            "the packet blocks the stable claim because every row narrowed "
            "below launch_stable must surface a disclosure ref so reviewers "
            "can see why the lane downgraded."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            2,
            expected_findings=[
                "narrowed_row_missing_disclosure_ref",
                "downgrade_automation_missing_disclosure_ref",
            ],
        ),
    }


def projection_collapses_selector_durability_vocabulary_case():
    pid = (
        "packet:m4:stabilize_the_test_explorer_inline_results_watch_mode:"
        "projection_collapses_selector_durability_vocabulary"
    )
    wid = (
        "workflow.runtime.stabilize_the_test_explorer_inline_results_watch_mode."
        "projection_collapses_selector_durability_vocabulary"
    )
    inp = baseline_input(pid, wid)
    for proj in inp["consumer_projections"]:
        if proj["consumer_surface"] == "help_about":
            proj["preserves_selector_durability_vocabulary"] = False
            break
    return {
        "record_kind": (
            "stabilize_the_test_explorer_inline_results_watch_mode_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": (
            "projection_collapses_selector_durability_vocabulary_blocks_stable"
        ),
        "scenario": (
            "The help_about consumer projection drops the "
            "selector-durability vocabulary; the packet blocks the stable "
            "claim because a downstream surface that collapses the three "
            "durable selector classes (durable_id_selector, trait_selector, "
            "snapshot_scoped_query_selector) cannot preserve test-explorer "
            "truth verbatim."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            3,
            expected_findings=[
                "selector_durability_vocabulary_collapsed",
                "missing_consumer_projection",
                "consumer_projection_drift",
            ],
        ),
    }


def raw_source_material_case():
    pid = (
        "packet:m4:stabilize_the_test_explorer_inline_results_watch_mode:"
        "raw_source_material"
    )
    wid = (
        "workflow.runtime.stabilize_the_test_explorer_inline_results_watch_mode."
        "raw_source_material"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:local:quality":
            row["raw_source_material_excluded"] = False
            break
    return {
        "record_kind": (
            "stabilize_the_test_explorer_inline_results_watch_mode_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "raw_source_material_blocks_stable",
        "scenario": (
            "The local lane's test_explorer_stabilization_quality row "
            "admits raw test source bodies, raw runner scrollback bodies, "
            "raw stack frames, raw command lines, or raw process "
            "environment bytes past the boundary; the packet blocks the "
            "stable claim because raw runtime material, secrets, and "
            "ambient credentials must never leak through the test-explorer "
            "boundary."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            1,
            expected_findings=["raw_source_material_present"],
        ),
    }


def write_json(path, payload):
    full = os.path.join(REPO, path)
    os.makedirs(os.path.dirname(full), exist_ok=True)
    with open(full, "w", encoding="utf-8") as fh:
        json.dump(payload, fh, indent=2)
        fh.write("\n")


def main():
    write_json(ARTIFACT_PATH, packet_artifact())

    cases = [
        baseline_case(),
        unbound_evidence_case(),
        missing_watch_mode_support_case(),
        missing_selector_durability_case(),
        consumer_surface_missing_durable_selector_case(),
        narrowed_row_missing_disclosure_ref_case(),
        projection_collapses_selector_durability_vocabulary_case(),
        raw_source_material_case(),
    ]
    for case in cases:
        path = os.path.join(FIXTURE_DIR, f"{case['case_name']}.json")
        write_json(path, case)

    readme_path = os.path.join(FIXTURE_DIR, "README.md")
    full_readme = os.path.join(REPO, readme_path)
    os.makedirs(os.path.dirname(full_readme), exist_ok=True)
    with open(full_readme, "w", encoding="utf-8") as fh:
        fh.write(
            "# stabilize_the_test_explorer_inline_results_watch_mode fixture corpus\n\n"
            "Fixture corpus for the M4 stable stabilize-the-test-explorer / "
            "inline-results / watch-mode / rerun / debug-from-test truth "
            "packet (`schemas/runtime/"
            "stabilize_the_test_explorer_inline_results_watch_mode_truth.schema.json`).\n\n"
            "Each fixture is a `TestExplorerStabilizationTruthPacketInput` "
            "with an `expect` block that pins the materialized packet's "
            "promotion state, finding count, lane and row-class token sets, "
            "support-class, wedge, test-identity, discovery-posture, "
            "watch-mode-support, selector-durability, consumer-surface, "
            "known-limit, downgrade-automation, and evidence-class tokens, "
            "and the support-export safety verdict. Tests in "
            "`crates/aureline-runtime/tests/"
            "stabilize_the_test_explorer_inline_results_watch_mode_truth_packet.rs` "
            "load each case and assert that "
            "`TestExplorerStabilizationTruthPacket::materialize` agrees.\n\n"
            "Cases:\n\n"
            "- `baseline_stable.json` — All four test-explorer lanes "
            "(local, remote_helper, container, notebook) carry one "
            "`test_explorer_stabilization_quality` row at `launch_stable` "
            "plus the full four-wedge admission coverage "
            "(test_explorer_identity_truth, inline_results_truth, "
            "watch_mode_truth, rerun_debug_from_test_parity), the full "
            "four test-identity admissions (suite_identity, case_identity, "
            "template_identity, invocation_identity), the full three "
            "discovery-posture admissions (partial_discovery_record, "
            "loaded_versus_known_counts, case_enumeration_at_runtime), the "
            "full four watch-mode-support admissions (live, reduced, "
            "polling, unavailable), the full three selector-durability "
            "admissions (durable_id_selector, trait_selector, "
            "snapshot_scoped_query_selector), the full five "
            "consumer-surface bindings (test_explorer_surface, "
            "inline_results_surface, watch_mode_surface, rerun_surface, "
            "debug_from_test_surface), each attesting the vocabularies it "
            "is required to preserve, and a lineage_admission row binding "
            "`execution_context_id`. All twelve required consumer "
            "projections preserve the packet verbatim.\n"
            "- `launch_stable_with_unbound_evidence_blocks_stable.json` — "
            "The local lane's quality row claims `launch_stable` while "
            "its evidence is `evidence_unbound`; the packet blocks the "
            "stable claim.\n"
            "- `missing_watch_mode_support_for_launch_stable_blocks_stable.json` — "
            "The local lane claims `launch_stable` but the `polling` "
            "watch_mode_support_admission row is missing; the packet "
            "blocks the stable claim.\n"
            "- `missing_selector_durability_for_launch_stable_blocks_stable.json` — "
            "The local lane claims `launch_stable` but the "
            "`snapshot_scoped_query_selector` "
            "selector_durability_admission row is missing; the packet "
            "blocks the stable claim.\n"
            "- `consumer_surface_missing_durable_selector_attestation_blocks_stable.json` "
            "— The local lane's `rerun_surface` consumer_surface_binding "
            "row stops attesting durable-selector preservation; the packet "
            "blocks the stable claim because rerun_surface and "
            "debug_from_test_surface MUST attest preservation of the "
            "durable-selector vocabulary.\n"
            "- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — "
            "The local lane's quality row narrows to "
            "`launch_stable_below` but drops its disclosure ref; the "
            "packet blocks the stable claim.\n"
            "- `projection_collapses_selector_durability_vocabulary_blocks_stable.json` "
            "— The `help_about` consumer projection drops the "
            "selector-durability vocabulary; the packet blocks the stable "
            "claim.\n"
            "- `raw_source_material_blocks_stable.json` — The local lane's "
            "quality row admits raw test source bodies, raw runner "
            "scrollback bodies, raw stack frames, raw command lines, or "
            "raw env bytes past the boundary; the packet blocks the "
            "stable claim because raw runtime material must never leak "
            "through the test-explorer boundary.\n"
        )


if __name__ == "__main__":
    main()
