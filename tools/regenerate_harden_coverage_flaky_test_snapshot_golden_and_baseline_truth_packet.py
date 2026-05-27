#!/usr/bin/env python3
"""Regenerate the M4 coverage / flaky-test / snapshot-golden / baseline-truth packet artifact and fixture corpus.

Mirrors the Rust unit-test sample input in
crates/aureline-runtime/src/harden_coverage_flaky_test_snapshot_golden_and_baseline/mod.rs.
The generator is the canonical seed for the checked-in artifact and the
narrowed-below-stable fixture cases used by the integration tests in
crates/aureline-runtime/tests/harden_coverage_flaky_test_snapshot_golden_and_baseline_truth_packet.rs.

Run from anywhere:
    python3 tools/regenerate_harden_coverage_flaky_test_snapshot_golden_and_baseline_truth_packet.py
"""
import json
import os

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

DOC_REF = "docs/runtime/m4/harden-coverage-flaky-test-snapshot-golden-and-baseline.md"
FIXTURE_DIR = (
    "fixtures/runtime/m4/harden_coverage_flaky_test_snapshot_golden_and_baseline"
)
ARTIFACT_PATH = (
    "artifacts/runtime/m4/"
    "harden_coverage_flaky_test_snapshot_golden_and_baseline_truth_packet.json"
)
SCHEMA_REF = (
    "schemas/runtime/"
    "harden_coverage_flaky_test_snapshot_golden_and_baseline_truth.schema.json"
)

PACKET_RECORD_KIND = (
    "harden_coverage_flaky_test_snapshot_golden_and_baseline_truth_stable_packet"
)
FIXTURE_RECORD_KIND = (
    "harden_coverage_flaky_test_snapshot_golden_and_baseline_truth_stable_case"
)

LANES = [
    ("coverage_lane", "coverage"),
    ("flaky_test_lane", "flaky"),
    ("snapshot_golden_lane", "snapshot"),
    ("baseline_truth_lane", "baseline"),
]

WEDGES = [
    "stability_verdict_separation",
    "quarantine_mute_renewal_truth",
    "ai_candidate_source_attribution",
    "coverage_impact_truth",
]

STABILITY_VERDICTS = [
    "stable",
    "flaky",
    "failing",
    "quarantined",
    "muted",
    "unknown",
]

QUARANTINE_MUTE_STATES = [
    "active",
    "expiring_soon",
    "expired_pending_renewal",
    "removed",
]

TEST_SOURCES = [
    "human_authored",
    "candidate_ai_test",
    "automated_baseline",
    "imported_ci_evidence",
]

TEST_SOURCES_REQUIRING_LINEAGE = {
    "candidate_ai_test",
    "automated_baseline",
    "imported_ci_evidence",
}

COVERAGE_IMPACTS = [
    "measured",
    "estimated",
    "stale",
    "not_comparable",
]

CANDIDATE_LINEAGES = [
    "session_attempt_bound",
    "review_checkpoint_bound",
    "imported_ci_bound",
]

CONSUMER_SURFACE_BINDINGS = [
    "coverage_surface",
    "flaky_triage_surface",
    "snapshot_golden_surface",
    "baseline_surface",
    "release_packet_surface",
]

SURFACES_REQUIRING_STABILITY_VERDICT = {
    "flaky_triage_surface",
    "release_packet_surface",
}

SURFACES_REQUIRING_QUARANTINE_MUTE = {
    "flaky_triage_surface",
    "release_packet_surface",
}

SURFACES_REQUIRING_TEST_SOURCE = {
    "coverage_surface",
    "flaky_triage_surface",
    "snapshot_golden_surface",
    "baseline_surface",
    "release_packet_surface",
}

SURFACES_REQUIRING_COVERAGE_IMPACT = {
    "coverage_surface",
    "release_packet_surface",
}

SURFACES_REQUIRING_CANDIDATE_LINEAGE = {
    "baseline_surface",
    "snapshot_golden_surface",
    "release_packet_surface",
}

CONSUMER_SURFACES = [
    "coverage_surface",
    "flaky_triage_surface",
    "snapshot_golden_surface",
    "baseline_surface",
    "release_packet_surface",
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
        "stability_verdict_class": "not_applicable",
        "quarantine_mute_state_class": "not_applicable",
        "test_source_class": "not_applicable",
        "coverage_impact_class": "not_applicable",
        "candidate_lineage_class": "not_applicable",
        "consumer_surface_class": "not_applicable",
        "evidence_class": "fixture_repo_evidence",
        "known_limit_class": "none_declared",
        "downgrade_automation_class": "auto_narrow_on_lineage_break",
        "confidence_class": "high_confidence",
        "evidence_refs": [FIXTURE_DIR],
        "disclosure_ref": f"{DOC_REF}#auto_narrow_on_lineage_break",
        "attests_candidate_lineage_bound": False,
        "attests_stability_verdict_preserved": False,
        "attests_quarantine_mute_state_preserved": False,
        "attests_test_source_preserved": False,
        "attests_coverage_impact_preserved": False,
        "attests_candidate_lineage_preserved": False,
        "raw_source_material_excluded": True,
        "secrets_excluded": True,
        "ambient_authority_excluded": True,
        "captured_at": TIMESTAMP,
    }


def quality_row(lane, prefix):
    row = base_row(
        f"row:{prefix}:quality", lane, "coverage_flaky_snapshot_baseline_quality"
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


def stability_verdict_rows(lane, prefix):
    rows = []
    for verdict in STABILITY_VERDICTS:
        row = base_row(
            f"row:{prefix}:stability_verdict:{verdict}",
            lane,
            "stability_verdict_admission",
        )
        row["stability_verdict_class"] = verdict
        row["evidence_class"] = "automated_functional_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_stability_verdict_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_stability_verdict_gap"
        rows.append(row)
    return rows


def quarantine_mute_state_rows(lane, prefix):
    rows = []
    for state in QUARANTINE_MUTE_STATES:
        row = base_row(
            f"row:{prefix}:quarantine_mute_state:{state}",
            lane,
            "quarantine_mute_state_admission",
        )
        row["quarantine_mute_state_class"] = state
        row["evidence_class"] = "automated_functional_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_quarantine_mute_state_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_quarantine_mute_state_gap"
        rows.append(row)
    return rows


def test_source_rows(lane, prefix):
    rows = []
    for source in TEST_SOURCES:
        row = base_row(
            f"row:{prefix}:test_source:{source}",
            lane,
            "test_source_admission",
        )
        row["test_source_class"] = source
        row["evidence_class"] = "automated_functional_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_test_source_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_test_source_gap"
        row["attests_candidate_lineage_bound"] = (
            source in TEST_SOURCES_REQUIRING_LINEAGE
        )
        rows.append(row)
    return rows


def coverage_impact_rows(lane, prefix):
    rows = []
    for impact in COVERAGE_IMPACTS:
        row = base_row(
            f"row:{prefix}:coverage_impact:{impact}",
            lane,
            "coverage_impact_admission",
        )
        row["coverage_impact_class"] = impact
        row["evidence_class"] = "automated_functional_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_coverage_impact_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_coverage_impact_gap"
        rows.append(row)
    return rows


def candidate_lineage_rows(lane, prefix):
    rows = []
    for lineage in CANDIDATE_LINEAGES:
        row = base_row(
            f"row:{prefix}:candidate_lineage:{lineage}",
            lane,
            "candidate_lineage_admission",
        )
        row["candidate_lineage_class"] = lineage
        row["evidence_class"] = "automated_functional_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_candidate_lineage_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_candidate_lineage_gap"
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
        row["attests_stability_verdict_preserved"] = (
            surface in SURFACES_REQUIRING_STABILITY_VERDICT
        )
        row["attests_quarantine_mute_state_preserved"] = (
            surface in SURFACES_REQUIRING_QUARANTINE_MUTE
        )
        row["attests_test_source_preserved"] = (
            surface in SURFACES_REQUIRING_TEST_SOURCE
        )
        row["attests_coverage_impact_preserved"] = (
            surface in SURFACES_REQUIRING_COVERAGE_IMPACT
        )
        row["attests_candidate_lineage_preserved"] = (
            surface in SURFACES_REQUIRING_CANDIDATE_LINEAGE
        )
        rows.append(row)
    return rows


def lineage_row(lane, prefix):
    row = base_row(f"row:{prefix}:lineage_admission", lane, "lineage_admission")
    row["evidence_class"] = "automated_functional_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_lineage_break"
    row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_lineage_break"
    row["execution_context_id_binding"] = (
        f"exec:m4:{prefix}:coverage_quality_lineage"
    )
    return row


def lane_rows(lane, prefix):
    rows = [quality_row(lane, prefix)]
    rows.extend(wedge_rows(lane, prefix))
    rows.extend(stability_verdict_rows(lane, prefix))
    rows.extend(quarantine_mute_state_rows(lane, prefix))
    rows.extend(test_source_rows(lane, prefix))
    rows.extend(coverage_impact_rows(lane, prefix))
    rows.extend(candidate_lineage_rows(lane, prefix))
    rows.extend(consumer_surface_rows(lane, prefix))
    rows.append(lineage_row(lane, prefix))
    return rows


def projection(surface, packet_id):
    return {
        "consumer_surface": surface,
        "projection_ref": f"projection:{surface}",
        "coverage_quality_truth_packet_id_ref": packet_id,
        "rendered_at": "2026-05-26T12:00:01Z",
        "preserves_same_packet": True,
        "preserves_lane_vocabulary": True,
        "preserves_row_class_vocabulary": True,
        "preserves_support_class_vocabulary": True,
        "preserves_wedge_vocabulary": True,
        "preserves_stability_verdict_vocabulary": True,
        "preserves_quarantine_mute_state_vocabulary": True,
        "preserves_test_source_vocabulary": True,
        "preserves_coverage_impact_vocabulary": True,
        "preserves_candidate_lineage_vocabulary": True,
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
        "packet:m4:harden_coverage_flaky_test_snapshot_golden_and_baseline:stable"
    )
    workflow_id = (
        "workflow.runtime.harden_coverage_flaky_test_snapshot_golden_and_baseline.stable"
    )
    base = baseline_input(packet_id, workflow_id)
    base.update(
        {
            "record_kind": PACKET_RECORD_KIND,
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
    stability_verdict_tokens = sorted({row["stability_verdict_class"] for row in rows})
    quarantine_mute_state_tokens = sorted(
        {row["quarantine_mute_state_class"] for row in rows}
    )
    test_source_tokens = sorted({row["test_source_class"] for row in rows})
    coverage_impact_tokens = sorted({row["coverage_impact_class"] for row in rows})
    candidate_lineage_tokens = sorted(
        {row["candidate_lineage_class"] for row in rows}
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
        "stability_verdict_tokens": stability_verdict_tokens,
        "quarantine_mute_state_tokens": quarantine_mute_state_tokens,
        "test_source_tokens": test_source_tokens,
        "coverage_impact_tokens": coverage_impact_tokens,
        "candidate_lineage_tokens": candidate_lineage_tokens,
        "consumer_surface_tokens": consumer_surface_tokens,
        "known_limit_tokens": known_limit_tokens,
        "downgrade_automation_tokens": downgrade_automation_tokens,
        "evidence_class_tokens": evidence_class_tokens,
        "support_export_safe": promotion_state == "stable",
    }
    if expected_findings:
        expect["expected_finding_kinds"] = expected_findings
    return expect


def fixture(case_name, scenario, input_obj, promotion, finding_count, expected_findings=None):
    return {
        "record_kind": FIXTURE_RECORD_KIND,
        "schema_version": 1,
        "case_name": case_name,
        "scenario": scenario,
        "input": input_obj,
        "expect": expectations_from_input(
            input_obj, promotion, finding_count, expected_findings=expected_findings
        ),
    }


def baseline_case():
    pid = (
        "packet:m4:harden_coverage_flaky_test_snapshot_golden_and_baseline:"
        "baseline_stable"
    )
    wid = (
        "workflow.runtime.harden_coverage_flaky_test_snapshot_golden_and_baseline."
        "baseline_stable"
    )
    inp = baseline_input(pid, wid)
    return fixture(
        "baseline_stable",
        (
            "Baseline stable posture: all four coverage-quality lanes "
            "(coverage, flaky_test, snapshot_golden, baseline_truth) publish "
            "a coverage_flaky_snapshot_baseline_quality row at launch_stable "
            "plus the full four-wedge admission coverage "
            "(stability_verdict_separation, quarantine_mute_renewal_truth, "
            "ai_candidate_source_attribution, coverage_impact_truth), the "
            "full six stability-verdict admissions (stable, flaky, failing, "
            "quarantined, muted, unknown), the full four quarantine-mute "
            "state admissions (active, expiring_soon, expired_pending_renewal, "
            "removed), the full four test-source admissions (human_authored, "
            "candidate_ai_test, automated_baseline, imported_ci_evidence) "
            "each binding the required session/attempt + review-checkpoint "
            "lineage for candidate / automated / imported sources, the full "
            "four coverage-impact admissions (measured, estimated, stale, "
            "not_comparable), the full three candidate-lineage admissions "
            "(session_attempt_bound, review_checkpoint_bound, "
            "imported_ci_bound), the full five consumer-surface bindings "
            "(coverage_surface, flaky_triage_surface, snapshot_golden_surface, "
            "baseline_surface, release_packet_surface) each attesting the "
            "vocabularies it is required to preserve, and a lineage_admission "
            "row binding execution_context_id; every row binds support, "
            "known limit, downgrade automation, and evidence classes; "
            "narrowed rows carry their disclosure refs; and all twelve "
            "required consumer projections preserve the packet verbatim."
        ),
        inp,
        "stable",
        0,
    )


def unbound_evidence_case():
    pid = (
        "packet:m4:harden_coverage_flaky_test_snapshot_golden_and_baseline:"
        "launch_stable_with_unbound_evidence"
    )
    wid = (
        "workflow.runtime.harden_coverage_flaky_test_snapshot_golden_and_baseline."
        "launch_stable_with_unbound_evidence"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:coverage:quality":
            row["evidence_class"] = "evidence_unbound"
            break
    return fixture(
        "launch_stable_with_unbound_evidence_blocks_stable",
        (
            "The coverage lane's coverage_flaky_snapshot_baseline_quality "
            "row claims launch_stable while its evidence class is "
            "evidence_unbound; the packet blocks the stable claim because "
            "no automated_functional, conformance_suite, "
            "failure_recovery_drill, release_evidence_review, fixture_repo, "
            "benchmark, design_qa, or docs_disclosure evidence backs the "
            "row."
        ),
        inp,
        "blocks_stable",
        2,
        expected_findings=[
            "missing_evidence_class",
            "launch_stable_with_unbound_binding",
        ],
    )


def missing_stability_verdict_case():
    pid = (
        "packet:m4:harden_coverage_flaky_test_snapshot_golden_and_baseline:"
        "missing_stability_verdict_for_launch_stable"
    )
    wid = (
        "workflow.runtime.harden_coverage_flaky_test_snapshot_golden_and_baseline."
        "missing_stability_verdict_for_launch_stable"
    )
    inp = baseline_input(pid, wid)
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "stability_verdict_admission"
            and row["stability_verdict_class"] == "quarantined"
            and row["lane_class"] == "flaky_test_lane"
        )
    ]
    return fixture(
        "missing_stability_verdict_for_launch_stable_blocks_stable",
        (
            "The flaky_test lane claims launch_stable but drops its "
            "`quarantined` stability_verdict_admission row; the packet "
            "blocks the stable claim because every launch_stable lane MUST "
            "admit all six stability verdicts (stable, flaky, failing, "
            "quarantined, muted, unknown) so the verdict cannot collapse "
            "into a coarse pass/fail bit."
        ),
        inp,
        "blocks_stable",
        1,
        expected_findings=["missing_stability_verdict_coverage"],
    )


def missing_quarantine_mute_state_case():
    pid = (
        "packet:m4:harden_coverage_flaky_test_snapshot_golden_and_baseline:"
        "missing_quarantine_mute_state_for_launch_stable"
    )
    wid = (
        "workflow.runtime.harden_coverage_flaky_test_snapshot_golden_and_baseline."
        "missing_quarantine_mute_state_for_launch_stable"
    )
    inp = baseline_input(pid, wid)
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "quarantine_mute_state_admission"
            and row["quarantine_mute_state_class"] == "expired_pending_renewal"
            and row["lane_class"] == "flaky_test_lane"
        )
    ]
    return fixture(
        "missing_quarantine_mute_state_for_launch_stable_blocks_stable",
        (
            "The flaky_test lane claims launch_stable but drops its "
            "`expired_pending_renewal` quarantine_mute_state_admission row; "
            "the packet blocks the stable claim because muted and "
            "quarantined tests MUST carry explicit renewal / expiry / "
            "removal semantics so debt cannot hide indefinitely."
        ),
        inp,
        "blocks_stable",
        1,
        expected_findings=["missing_quarantine_mute_state_coverage"],
    )


def candidate_ai_without_lineage_case():
    pid = (
        "packet:m4:harden_coverage_flaky_test_snapshot_golden_and_baseline:"
        "candidate_ai_test_without_lineage"
    )
    wid = (
        "workflow.runtime.harden_coverage_flaky_test_snapshot_golden_and_baseline."
        "candidate_ai_test_without_lineage"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if (
            row["row_class"] == "test_source_admission"
            and row["lane_class"] == "coverage_lane"
            and row["test_source_class"] == "candidate_ai_test"
        ):
            row["attests_candidate_lineage_bound"] = False
            break
    return fixture(
        "candidate_ai_test_without_lineage_blocks_stable",
        (
            "The coverage lane's candidate_ai_test test_source_admission "
            "row drops its session/attempt + review-checkpoint lineage "
            "attestation; the packet blocks the stable claim because "
            "AI-generated tests, automated baseline mutations, and imported "
            "CI evidence MUST attach to the same session/attempt and review "
            "checkpoint lineage before they can influence promotion or "
            "claim packets."
        ),
        inp,
        "blocks_stable",
        2,
        expected_findings=[
            "candidate_source_not_lineage_bound",
            "missing_test_source_coverage",
        ],
    )


def missing_coverage_impact_case():
    pid = (
        "packet:m4:harden_coverage_flaky_test_snapshot_golden_and_baseline:"
        "missing_coverage_impact_for_launch_stable"
    )
    wid = (
        "workflow.runtime.harden_coverage_flaky_test_snapshot_golden_and_baseline."
        "missing_coverage_impact_for_launch_stable"
    )
    inp = baseline_input(pid, wid)
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "coverage_impact_admission"
            and row["coverage_impact_class"] == "not_comparable"
            and row["lane_class"] == "coverage_lane"
        )
    ]
    return fixture(
        "missing_coverage_impact_for_launch_stable_blocks_stable",
        (
            "The coverage lane claims launch_stable but drops its "
            "`not_comparable` coverage_impact_admission row; the packet "
            "blocks the stable claim because coverage impact from AI / "
            "sandbox-run candidate tests MUST stay explicitly measured, "
            "estimated, stale, or not_comparable per target/environment "
            "family so a single passing run cannot silently upgrade a "
            "candidate into trusted stable coverage proof."
        ),
        inp,
        "blocks_stable",
        1,
        expected_findings=["missing_coverage_impact_coverage"],
    )


def consumer_surface_missing_candidate_lineage_case():
    pid = (
        "packet:m4:harden_coverage_flaky_test_snapshot_golden_and_baseline:"
        "consumer_surface_missing_candidate_lineage_attestation"
    )
    wid = (
        "workflow.runtime.harden_coverage_flaky_test_snapshot_golden_and_baseline."
        "consumer_surface_missing_candidate_lineage_attestation"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if (
            row["row_class"] == "consumer_surface_binding"
            and row["lane_class"] == "baseline_truth_lane"
            and row["consumer_surface_class"] == "baseline_surface"
        ):
            row["attests_candidate_lineage_preserved"] = False
            break
    return fixture(
        "consumer_surface_missing_candidate_lineage_attestation_blocks_stable",
        (
            "The baseline_truth lane's baseline_surface "
            "consumer_surface_binding row stops attesting candidate-lineage "
            "preservation; the packet blocks the stable claim because the "
            "baseline_surface MUST attest preservation of the "
            "candidate-lineage vocabulary so AI-proposed baseline "
            "mutations stay distinguishable from ordinary baseline "
            "changes in release and support packets."
        ),
        inp,
        "blocks_stable",
        2,
        expected_findings=[
            "consumer_surface_missing_candidate_lineage_attestation",
            "missing_consumer_surface_coverage",
        ],
    )


def narrowed_row_missing_disclosure_ref_case():
    pid = (
        "packet:m4:harden_coverage_flaky_test_snapshot_golden_and_baseline:"
        "narrowed_row_missing_disclosure_ref"
    )
    wid = (
        "workflow.runtime.harden_coverage_flaky_test_snapshot_golden_and_baseline."
        "narrowed_row_missing_disclosure_ref"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:coverage:quality":
            row["support_class"] = "launch_stable_below"
            row.pop("disclosure_ref", None)
            break
    return fixture(
        "narrowed_row_missing_disclosure_ref_blocks_stable",
        (
            "The coverage lane's quality row narrows to launch_stable_below "
            "but drops its disclosure ref; the packet blocks the stable "
            "claim because every row narrowed below launch_stable must "
            "surface a disclosure ref so reviewers can see why the lane "
            "downgraded."
        ),
        inp,
        "blocks_stable",
        2,
        expected_findings=[
            "narrowed_row_missing_disclosure_ref",
            "downgrade_automation_missing_disclosure_ref",
        ],
    )


def projection_collapses_coverage_impact_case():
    pid = (
        "packet:m4:harden_coverage_flaky_test_snapshot_golden_and_baseline:"
        "projection_collapses_coverage_impact_vocabulary"
    )
    wid = (
        "workflow.runtime.harden_coverage_flaky_test_snapshot_golden_and_baseline."
        "projection_collapses_coverage_impact_vocabulary"
    )
    inp = baseline_input(pid, wid)
    for proj in inp["consumer_projections"]:
        if proj["consumer_surface"] == "help_about":
            proj["preserves_coverage_impact_vocabulary"] = False
            break
    return fixture(
        "projection_collapses_coverage_impact_vocabulary_blocks_stable",
        (
            "The help_about consumer projection drops the coverage-impact "
            "vocabulary; the packet blocks the stable claim because a "
            "downstream surface that collapses the four coverage-impact "
            "classes (measured, estimated, stale, not_comparable) cannot "
            "preserve coverage-quality truth verbatim."
        ),
        inp,
        "blocks_stable",
        3,
        expected_findings=[
            "coverage_impact_vocabulary_collapsed",
            "missing_consumer_projection",
            "consumer_projection_drift",
        ],
    )


def raw_source_material_case():
    pid = (
        "packet:m4:harden_coverage_flaky_test_snapshot_golden_and_baseline:"
        "raw_source_material"
    )
    wid = (
        "workflow.runtime.harden_coverage_flaky_test_snapshot_golden_and_baseline."
        "raw_source_material"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:coverage:quality":
            row["raw_source_material_excluded"] = False
            break
    return fixture(
        "raw_source_material_blocks_stable",
        (
            "The coverage lane's quality row admits raw test bodies, raw "
            "coverage payloads, raw snapshot byte streams, raw baseline "
            "diffs, raw runner scrollback bodies, raw command lines, or "
            "raw process environment bytes past the boundary; the packet "
            "blocks the stable claim because raw runtime material, "
            "secrets, and ambient credentials must never leak through "
            "the coverage-quality boundary."
        ),
        inp,
        "blocks_stable",
        1,
        expected_findings=["raw_source_material_present"],
    )


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
        missing_stability_verdict_case(),
        missing_quarantine_mute_state_case(),
        candidate_ai_without_lineage_case(),
        missing_coverage_impact_case(),
        consumer_surface_missing_candidate_lineage_case(),
        narrowed_row_missing_disclosure_ref_case(),
        projection_collapses_coverage_impact_case(),
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
            "# harden_coverage_flaky_test_snapshot_golden_and_baseline fixture corpus\n\n"
            "Fixture corpus for the M4 stable coverage / flaky-test / "
            "snapshot-golden / baseline-truth packet (`schemas/runtime/"
            "harden_coverage_flaky_test_snapshot_golden_and_baseline_truth.schema.json`).\n\n"
            "Each fixture is a `CoverageQualityTruthPacketInput` with an "
            "`expect` block that pins the materialized packet's promotion "
            "state, finding count, lane / row-class / support-class / wedge "
            "/ stability-verdict / quarantine-mute-state / test-source / "
            "coverage-impact / candidate-lineage / consumer-surface / "
            "known-limit / downgrade-automation / evidence-class tokens, "
            "and the support-export safety verdict. Tests in "
            "`crates/aureline-runtime/tests/"
            "harden_coverage_flaky_test_snapshot_golden_and_baseline_truth_packet.rs` "
            "load each case and assert that "
            "`CoverageQualityTruthPacket::materialize` agrees.\n\n"
            "Cases:\n\n"
            "- `baseline_stable.json` — All four lanes (coverage, flaky_test, "
            "snapshot_golden, baseline_truth) carry one "
            "`coverage_flaky_snapshot_baseline_quality` row at "
            "`launch_stable` plus the full four-wedge admission coverage, "
            "the full six stability-verdict admissions, the full four "
            "quarantine-mute-state admissions, the full four test-source "
            "admissions (with the required session/attempt + review-checkpoint "
            "lineage attestation for candidate / automated / imported "
            "sources), the full four coverage-impact admissions, the full "
            "three candidate-lineage admissions, the full five "
            "consumer-surface bindings each attesting the vocabularies it "
            "is required to preserve, and a lineage_admission row binding "
            "`execution_context_id`. All twelve required consumer "
            "projections preserve the packet verbatim.\n"
            "- `launch_stable_with_unbound_evidence_blocks_stable.json` — "
            "The coverage lane's quality row claims `launch_stable` while "
            "its evidence is `evidence_unbound`; the packet blocks the "
            "stable claim.\n"
            "- `missing_stability_verdict_for_launch_stable_blocks_stable.json` — "
            "The flaky_test lane claims `launch_stable` but the "
            "`quarantined` stability_verdict_admission row is missing; "
            "the packet blocks the stable claim because the stability "
            "verdict and quarantine-vs-mute state must remain separately "
            "observable.\n"
            "- `missing_quarantine_mute_state_for_launch_stable_blocks_stable.json` — "
            "The flaky_test lane claims `launch_stable` but the "
            "`expired_pending_renewal` quarantine_mute_state_admission row "
            "is missing; the packet blocks the stable claim because muted "
            "and quarantined tests must carry explicit renewal / expiry / "
            "removal semantics rather than indefinite hidden debt.\n"
            "- `candidate_ai_test_without_lineage_blocks_stable.json` — The "
            "coverage lane's `candidate_ai_test` test_source_admission row "
            "drops its session/attempt + review-checkpoint lineage "
            "attestation; the packet blocks the stable claim because "
            "AI-generated tests, automated baseline mutations, and imported "
            "CI evidence MUST attach to the same session/attempt and "
            "review-checkpoint lineage before they can influence "
            "promotion.\n"
            "- `missing_coverage_impact_for_launch_stable_blocks_stable.json` — "
            "The coverage lane claims `launch_stable` but the "
            "`not_comparable` coverage_impact_admission row is missing; "
            "the packet blocks the stable claim because coverage impact "
            "from AI / sandbox-run candidate tests must remain explicitly "
            "measured, estimated, stale, or not_comparable.\n"
            "- `consumer_surface_missing_candidate_lineage_attestation_blocks_stable.json` "
            "— The baseline_truth lane's `baseline_surface` "
            "consumer_surface_binding row stops attesting candidate-lineage "
            "preservation; the packet blocks the stable claim because "
            "baseline_surface MUST attest preservation of the "
            "candidate-lineage vocabulary so AI-proposed baseline mutations "
            "stay distinguishable from ordinary baseline changes.\n"
            "- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — "
            "The coverage lane's quality row narrows to "
            "`launch_stable_below` but drops its disclosure ref; the "
            "packet blocks the stable claim.\n"
            "- `projection_collapses_coverage_impact_vocabulary_blocks_stable.json` "
            "— The `help_about` consumer projection drops the "
            "coverage-impact vocabulary; the packet blocks the stable "
            "claim.\n"
            "- `raw_source_material_blocks_stable.json` — The coverage "
            "lane's quality row admits raw test bodies, raw coverage "
            "payloads, raw snapshot byte streams, raw baseline diffs, raw "
            "runner scrollback bodies, raw command lines, or raw env bytes "
            "past the boundary; the packet blocks the stable claim because "
            "raw runtime material must never leak through the "
            "coverage-quality boundary.\n"
        )


if __name__ == "__main__":
    main()
