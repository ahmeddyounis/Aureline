#!/usr/bin/env python3
"""Regenerate the M4 integrated-terminal stabilization truth packet artifact and fixture corpus.

Mirrors the Rust unit-test sample input in
crates/aureline-terminal/src/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export/mod.rs.
The generator is the canonical seed for the checked-in artifact and the
narrowed-below-stable fixture cases used by the integration tests in
crates/aureline-terminal/tests/stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_packet.rs.

Run from anywhere:
    python3 tools/regenerate_stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_packet.py
"""
import json
import os

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

DOC_REF = (
    "docs/runtime/m4/"
    "stabilize-integrated-terminal-boundaries-clipboard-posture-transcript-export.md"
)
FIXTURE_DIR = (
    "fixtures/runtime/m4/"
    "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export"
)
ARTIFACT_PATH = (
    "artifacts/runtime/m4/"
    "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_packet.json"
)
SCHEMA_REF = (
    "schemas/runtime/"
    "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth.schema.json"
)

LANES = [
    ("local_lane", "local"),
    ("remote_helper_lane", "remote"),
    ("container_lane", "container"),
    ("restored_lane", "restored"),
]

WEDGES = [
    "host_boundary_chip",
    "clipboard_posture",
    "transcript_export",
    "restore_no_rerun",
]

HOST_BOUNDARY_FIELDS = [
    "host_or_session_identity",
    "route_cue",
    "trust_state",
    "restore_state",
    "target_or_cwd_hint",
]

CLIPBOARD_POSTURE_SURFACES = [
    "clipboard_route_local_vs_remote",
    "bracketed_paste_state",
    "multiline_paste_guardrail",
    "admin_suppression",
    "high_risk_paste_review",
]

TRANSCRIPT_EXPORT_FIELDS = [
    "transcript_versus_live_session",
    "host_session_boundary_cue",
    "redaction_state",
]

CONSUMER_SURFACES = [
    "terminal_pane",
    "transcript_export_surface",
    "browser_handoff_surface",
    "restore_surface",
    "cli_headless",
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
        "host_boundary_field_class": "not_applicable",
        "clipboard_posture_class": "not_applicable",
        "transcript_export_field_class": "not_applicable",
        "evidence_class": "fixture_repo_evidence",
        "known_limit_class": "none_declared",
        "downgrade_automation_class": "auto_narrow_on_lineage_break",
        "confidence_class": "high_confidence",
        "evidence_refs": [FIXTURE_DIR],
        "disclosure_ref": f"{DOC_REF}#auto_narrow_on_lineage_break",
        "attests_no_silent_rerun": False,
        "raw_source_material_excluded": True,
        "secrets_excluded": True,
        "ambient_authority_excluded": True,
        "captured_at": TIMESTAMP,
    }


def quality_row(lane, prefix):
    row = base_row(f"row:{prefix}:quality", lane, "terminal_stabilization_quality")
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


def host_boundary_rows(lane, prefix):
    rows = []
    for field in HOST_BOUNDARY_FIELDS:
        row = base_row(
            f"row:{prefix}:host_boundary:{field}",
            lane,
            "host_boundary_field_binding",
        )
        row["host_boundary_field_class"] = field
        row["evidence_class"] = "automated_functional_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_host_boundary_field_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_host_boundary_field_gap"
        rows.append(row)
    return rows


def clipboard_rows(lane, prefix):
    rows = []
    for surface in CLIPBOARD_POSTURE_SURFACES:
        row = base_row(
            f"row:{prefix}:clipboard:{surface}", lane, "clipboard_posture_binding"
        )
        row["clipboard_posture_class"] = surface
        row["evidence_class"] = "conformance_suite_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_clipboard_posture_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_clipboard_posture_gap"
        rows.append(row)
    return rows


def transcript_rows(lane, prefix):
    rows = []
    for field in TRANSCRIPT_EXPORT_FIELDS:
        row = base_row(
            f"row:{prefix}:transcript_export:{field}",
            lane,
            "transcript_export_field_binding",
        )
        row["transcript_export_field_class"] = field
        row["evidence_class"] = "automated_functional_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_transcript_export_field_gap"
        row["disclosure_ref"] = (
            f"{DOC_REF}#auto_narrow_on_transcript_export_field_gap"
        )
        rows.append(row)
    return rows


def restore_no_rerun_row(lane, prefix):
    row = base_row(
        f"row:{prefix}:restore_no_rerun", lane, "restore_no_rerun_attestation"
    )
    row["evidence_class"] = "failure_recovery_drill_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_restore_admits_silent_rerun"
    row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_restore_admits_silent_rerun"
    row["attests_no_silent_rerun"] = True
    return row


def lineage_row(lane, prefix):
    row = base_row(f"row:{prefix}:lineage_admission", lane, "lineage_admission")
    row["evidence_class"] = "automated_functional_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_lineage_break"
    row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_lineage_break"
    row["execution_context_id_binding"] = f"exec:m4:{prefix}:terminal_session_lineage"
    return row


def lane_rows(lane, prefix):
    rows = [quality_row(lane, prefix)]
    rows.extend(wedge_rows(lane, prefix))
    rows.extend(host_boundary_rows(lane, prefix))
    rows.extend(clipboard_rows(lane, prefix))
    rows.extend(transcript_rows(lane, prefix))
    rows.append(restore_no_rerun_row(lane, prefix))
    rows.append(lineage_row(lane, prefix))
    return rows


def projection(surface, packet_id):
    return {
        "consumer_surface": surface,
        "projection_ref": f"projection:{surface}",
        "terminal_stabilization_truth_packet_id_ref": packet_id,
        "rendered_at": "2026-05-26T12:00:01Z",
        "preserves_same_packet": True,
        "preserves_lane_vocabulary": True,
        "preserves_row_class_vocabulary": True,
        "preserves_support_class_vocabulary": True,
        "preserves_wedge_vocabulary": True,
        "preserves_host_boundary_field_vocabulary": True,
        "preserves_clipboard_posture_vocabulary": True,
        "preserves_transcript_export_field_vocabulary": True,
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
        "packet:m4:"
        "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export:stable"
    )
    workflow_id = (
        "workflow.runtime."
        "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export.stable"
    )
    base = baseline_input(packet_id, workflow_id)
    base.update(
        {
            "record_kind": (
                "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_stable_packet"
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


def expectations_from_input(input_obj, promotion_state, finding_count, expected_findings=None):
    rows = input_obj["rows"]
    lane_tokens = sorted({row["lane_class"] for row in rows})
    row_class_tokens = sorted({row["row_class"] for row in rows})
    support_class_tokens = sorted({row["support_class"] for row in rows})
    wedge_tokens = sorted({row["wedge_class"] for row in rows})
    host_boundary_field_tokens = sorted(
        {row["host_boundary_field_class"] for row in rows}
    )
    clipboard_posture_tokens = sorted(
        {row["clipboard_posture_class"] for row in rows}
    )
    transcript_export_field_tokens = sorted(
        {row["transcript_export_field_class"] for row in rows}
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
        "host_boundary_field_tokens": host_boundary_field_tokens,
        "clipboard_posture_tokens": clipboard_posture_tokens,
        "transcript_export_field_tokens": transcript_export_field_tokens,
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
        "packet:m4:"
        "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export:"
        "baseline_stable"
    )
    wid = (
        "workflow.runtime."
        "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export."
        "baseline_stable"
    )
    inp = baseline_input(pid, wid)
    return {
        "record_kind": (
            "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "baseline_stable",
        "scenario": (
            "Baseline stable posture: all four terminal-session lanes (local, "
            "remote_helper, container, restored) publish a "
            "terminal_stabilization_quality row at launch_stable plus the full "
            "four-wedge admission coverage (host_boundary_chip, clipboard_posture, "
            "transcript_export, restore_no_rerun), the full five host-boundary "
            "field bindings (host_or_session_identity, route_cue, trust_state, "
            "restore_state, target_or_cwd_hint), the full five clipboard-posture "
            "surface bindings (clipboard_route_local_vs_remote, "
            "bracketed_paste_state, multiline_paste_guardrail, admin_suppression, "
            "high_risk_paste_review), the full three transcript-export field "
            "bindings (transcript_versus_live_session, host_session_boundary_cue, "
            "redaction_state), a restore_no_rerun_attestation row attesting no "
            "silent rerun, and a lineage_admission row binding "
            "execution_context_id; every row binds support, known limit, downgrade "
            "automation, and evidence classes; narrowed rows carry their "
            "disclosure refs; and all nine required consumer projections preserve "
            "the packet verbatim."
        ),
        "input": inp,
        "expect": expectations_from_input(inp, "stable", 0),
    }


def unbound_evidence_case():
    pid = (
        "packet:m4:"
        "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export:"
        "launch_stable_with_unbound_evidence"
    )
    wid = (
        "workflow.runtime."
        "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export."
        "launch_stable_with_unbound_evidence"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:local:quality":
            row["evidence_class"] = "evidence_unbound"
            break
    return {
        "record_kind": (
            "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "launch_stable_with_unbound_evidence_blocks_stable",
        "scenario": (
            "The local lane's terminal_stabilization_quality row claims "
            "launch_stable while its evidence class is evidence_unbound; the "
            "packet blocks the stable claim because no evidence backs the row."
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


def missing_clipboard_posture_case():
    pid = (
        "packet:m4:"
        "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export:"
        "missing_clipboard_posture_for_launch_stable"
    )
    wid = (
        "workflow.runtime."
        "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export."
        "missing_clipboard_posture_for_launch_stable"
    )
    inp = baseline_input(pid, wid)
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "clipboard_posture_binding"
            and row["clipboard_posture_class"] == "high_risk_paste_review"
            and row["lane_class"] == "local_lane"
        )
    ]
    return {
        "record_kind": (
            "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "missing_clipboard_posture_for_launch_stable_blocks_stable",
        "scenario": (
            "The local lane claims launch_stable but drops its "
            "high_risk_paste_review clipboard_posture_binding row; the packet "
            "blocks the stable claim because every launch_stable lane must bind "
            "all five clipboard-posture surfaces (clipboard_route_local_vs_remote, "
            "bracketed_paste_state, multiline_paste_guardrail, admin_suppression, "
            "high_risk_paste_review) so copy, paste, and protocol-driven clipboard "
            "writes never bypass the explicit policy."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            1,
            expected_findings=["missing_clipboard_posture_coverage"],
        ),
    }


def restore_admits_silent_rerun_case():
    pid = (
        "packet:m4:"
        "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export:"
        "restore_admits_silent_rerun"
    )
    wid = (
        "workflow.runtime."
        "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export."
        "restore_admits_silent_rerun"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if (
            row["row_id"] == "row:restored:restore_no_rerun"
            and row["row_class"] == "restore_no_rerun_attestation"
        ):
            row["attests_no_silent_rerun"] = False
            break
    return {
        "record_kind": (
            "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "restore_admits_silent_rerun_blocks_stable",
        "scenario": (
            "The restored lane's restore_no_rerun_attestation row stops "
            "attesting no_silent_rerun and admits silent rerun; the packet blocks "
            "the stable claim because restored sessions are transcript-only and "
            "must never silently rerun."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            2,
            expected_findings=[
                "restore_no_rerun_attestation_admits_silent_rerun",
                "missing_restore_no_rerun_attestation",
            ],
        ),
    }


def narrowed_row_missing_disclosure_ref_case():
    pid = (
        "packet:m4:"
        "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export:"
        "narrowed_row_missing_disclosure_ref"
    )
    wid = (
        "workflow.runtime."
        "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export."
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
            "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "narrowed_row_missing_disclosure_ref_blocks_stable",
        "scenario": (
            "The local lane's terminal_stabilization_quality row narrows to "
            "launch_stable_below but drops its disclosure ref; the packet blocks "
            "the stable claim because every row narrowed below launch_stable must "
            "surface a disclosure ref so reviewers can see why the lane "
            "downgraded."
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


def projection_collapses_clipboard_posture_case():
    pid = (
        "packet:m4:"
        "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export:"
        "projection_collapses_clipboard_posture_vocabulary"
    )
    wid = (
        "workflow.runtime."
        "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export."
        "projection_collapses_clipboard_posture_vocabulary"
    )
    inp = baseline_input(pid, wid)
    for proj in inp["consumer_projections"]:
        if proj["consumer_surface"] == "help_about":
            proj["preserves_clipboard_posture_vocabulary"] = False
            break
    return {
        "record_kind": (
            "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "projection_collapses_clipboard_posture_vocabulary_blocks_stable",
        "scenario": (
            "The help_about consumer projection drops the clipboard-posture "
            "vocabulary; the packet blocks the stable claim because a "
            "downstream surface that collapses the five canonical clipboard "
            "surfaces (clipboard_route_local_vs_remote, bracketed_paste_state, "
            "multiline_paste_guardrail, admin_suppression, "
            "high_risk_paste_review) cannot preserve terminal-stabilization "
            "truth verbatim."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            3,
            expected_findings=[
                "clipboard_posture_vocabulary_collapsed",
                "missing_consumer_projection",
                "consumer_projection_drift",
            ],
        ),
    }


def raw_source_material_case():
    pid = (
        "packet:m4:"
        "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export:"
        "raw_source_material"
    )
    wid = (
        "workflow.runtime."
        "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export."
        "raw_source_material"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:local:quality":
            row["raw_source_material_excluded"] = False
            break
    return {
        "record_kind": (
            "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "raw_source_material_blocks_stable",
        "scenario": (
            "The local lane's terminal_stabilization_quality row admits raw "
            "command lines, raw process environment bytes, or raw scrollback "
            "bodies past the boundary; the packet blocks the stable claim "
            "because raw runtime material, secrets, and ambient credentials "
            "must never leak through the terminal-stabilization boundary."
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
        missing_clipboard_posture_case(),
        restore_admits_silent_rerun_case(),
        narrowed_row_missing_disclosure_ref_case(),
        projection_collapses_clipboard_posture_case(),
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
            "# stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export fixture corpus\n\n"
            "Fixture corpus for the M4 stable integrated-terminal stabilization "
            "truth packet (`schemas/runtime/"
            "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth.schema.json`).\n\n"
            "Each fixture is a `TerminalStabilizationTruthPacketInput` with an "
            "`expect` block that pins the materialized packet's promotion state, "
            "finding count, lane and row-class token sets, support-class, "
            "wedge, host-boundary-field, clipboard-posture, transcript-export-field, "
            "known-limit, downgrade-automation, and evidence-class tokens, and "
            "the support-export safety verdict. Tests in `crates/aureline-terminal/tests/"
            "stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export_truth_packet.rs` "
            "load each case and assert that `TerminalStabilizationTruthPacket::materialize` "
            "agrees.\n\n"
            "Cases:\n\n"
            "- `baseline_stable.json` — All four terminal-session lanes (local, "
            "remote_helper, container, restored) carry one "
            "`terminal_stabilization_quality` row at `launch_stable` plus the "
            "full four-wedge admission coverage (host_boundary_chip, "
            "clipboard_posture, transcript_export, restore_no_rerun), the full "
            "five host-boundary field bindings (host_or_session_identity, "
            "route_cue, trust_state, restore_state, target_or_cwd_hint), the "
            "full five clipboard-posture surface bindings "
            "(clipboard_route_local_vs_remote, bracketed_paste_state, "
            "multiline_paste_guardrail, admin_suppression, "
            "high_risk_paste_review), the full three transcript-export field "
            "bindings (transcript_versus_live_session, host_session_boundary_cue, "
            "redaction_state), a restore_no_rerun_attestation row attesting "
            "`attests_no_silent_rerun: true`, and a lineage_admission row "
            "binding `execution_context_id`. All nine required consumer "
            "projections preserve the packet verbatim.\n"
            "- `launch_stable_with_unbound_evidence_blocks_stable.json` — The "
            "local lane's quality row claims `launch_stable` while its "
            "evidence is `evidence_unbound`; the packet blocks the stable "
            "claim.\n"
            "- `missing_clipboard_posture_for_launch_stable_blocks_stable.json` "
            "— The local lane claims `launch_stable` but the "
            "`high_risk_paste_review` clipboard-posture binding is missing; the "
            "packet blocks the stable claim.\n"
            "- `restore_admits_silent_rerun_blocks_stable.json` — The restored "
            "lane's restore_no_rerun_attestation row stops attesting "
            "`attests_no_silent_rerun: true`; the packet blocks the stable "
            "claim because restored sessions must be transcript-only and "
            "never silently rerun.\n"
            "- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — The "
            "local lane's quality row narrows to `launch_stable_below` but "
            "drops its disclosure ref; the packet blocks the stable claim.\n"
            "- `projection_collapses_clipboard_posture_vocabulary_blocks_stable.json` "
            "— The `help_about` consumer projection drops the clipboard-posture "
            "vocabulary; the packet blocks the stable claim.\n"
            "- `raw_source_material_blocks_stable.json` — The local lane's "
            "quality row admits raw command lines, env bytes, or scrollback "
            "bodies past the boundary; the packet blocks the stable claim "
            "because raw runtime material must never leak through the "
            "terminal-stabilization boundary.\n"
        )


if __name__ == "__main__":
    main()
