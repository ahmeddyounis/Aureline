#!/usr/bin/env python3
"""Regenerate the M4 harden-breakpoint / call-stack / variables / watch / evaluate / debug-console fidelity truth packet artifact and fixture corpus.

Mirrors the Rust unit-test sample input in
crates/aureline-runtime/src/harden_breakpoint_call_stack_variables_watch_evaluate_and/mod.rs.
The generator is the canonical seed for the checked-in artifact and the
narrowed-below-stable fixture cases used by the integration tests in
crates/aureline-runtime/tests/harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_packet.rs.

Run from anywhere:
    python3 tools/regenerate_harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_packet.py
"""
import json
import os

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

DOC_REF = "docs/runtime/m4/harden-breakpoint-call-stack-variables-watch-evaluate-and.md"
FIXTURE_DIR = (
    "fixtures/runtime/m4/harden_breakpoint_call_stack_variables_watch_evaluate_and"
)
ARTIFACT_PATH = (
    "artifacts/runtime/m4/"
    "harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_packet.json"
)
SCHEMA_REF = (
    "schemas/runtime/"
    "harden_breakpoint_call_stack_variables_watch_evaluate_and_truth.schema.json"
)

LANES = [
    ("local_lane", "local"),
    ("remote_helper_lane", "remote"),
    ("container_lane", "container"),
    ("notebook_bridge_lane", "notebook"),
]

WEDGES = [
    "breakpoint_fidelity",
    "call_stack_fidelity",
    "variables_fidelity",
    "watch_fidelity",
    "evaluate_fidelity",
    "debug_console_fidelity",
]

INSPECTOR_STATES = [
    "live",
    "snapshot",
    "stale",
    "limited",
    "unavailable",
    "policy_blocked",
]

MAPPING_FIDELITY_BADGES = [
    "exact",
    "approximate",
    "partial",
    "unavailable",
    "stale",
    "mismatched",
]

INSPECTOR_SURFACES = [
    "breakpoint_surface",
    "call_stack_surface",
    "variables_surface",
    "watch_surface",
    "evaluate_surface",
    "debug_console_surface",
]

SURFACES_REQUIRING_INSPECTOR_STATE_ATTESTATION = {
    "variables_surface",
    "watch_surface",
    "evaluate_surface",
    "debug_console_surface",
}

SURFACES_REQUIRING_MAPPING_FIDELITY_ATTESTATION = {
    "call_stack_surface",
    "watch_surface",
    "evaluate_surface",
    "debug_console_surface",
}

CONSUMER_SURFACES = [
    "breakpoint_surface",
    "call_stack_surface",
    "variables_surface",
    "watch_surface",
    "evaluate_surface",
    "debug_console_surface",
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
        "inspector_state_class": "not_applicable",
        "mapping_fidelity_badge_class": "not_applicable",
        "inspector_surface_class": "not_applicable",
        "evidence_class": "fixture_repo_evidence",
        "known_limit_class": "none_declared",
        "downgrade_automation_class": "auto_narrow_on_lineage_break",
        "confidence_class": "high_confidence",
        "evidence_refs": [FIXTURE_DIR],
        "disclosure_ref": f"{DOC_REF}#auto_narrow_on_lineage_break",
        "attests_inspector_state_preserved": False,
        "attests_mapping_fidelity_preserved": False,
        "raw_source_material_excluded": True,
        "secrets_excluded": True,
        "ambient_authority_excluded": True,
        "captured_at": TIMESTAMP,
    }


def quality_row(lane, prefix):
    row = base_row(f"row:{prefix}:quality", lane, "debug_fidelity_quality")
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


def inspector_state_rows(lane, prefix):
    rows = []
    for state in INSPECTOR_STATES:
        row = base_row(
            f"row:{prefix}:inspector_state:{state}", lane, "inspector_state_admission"
        )
        row["inspector_state_class"] = state
        row["evidence_class"] = "automated_functional_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_inspector_state_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_inspector_state_gap"
        rows.append(row)
    return rows


def mapping_fidelity_badge_rows(lane, prefix):
    rows = []
    for badge in MAPPING_FIDELITY_BADGES:
        row = base_row(
            f"row:{prefix}:mapping_fidelity_badge:{badge}",
            lane,
            "mapping_fidelity_badge_admission",
        )
        row["mapping_fidelity_badge_class"] = badge
        row["evidence_class"] = "automated_functional_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_mapping_fidelity_badge_gap"
        row["disclosure_ref"] = (
            f"{DOC_REF}#auto_narrow_on_mapping_fidelity_badge_gap"
        )
        rows.append(row)
    return rows


def inspector_surface_rows(lane, prefix):
    rows = []
    for surface in INSPECTOR_SURFACES:
        row = base_row(
            f"row:{prefix}:inspector_surface:{surface}",
            lane,
            "inspector_surface_binding",
        )
        row["inspector_surface_class"] = surface
        row["evidence_class"] = "conformance_suite_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_inspector_surface_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_inspector_surface_gap"
        row["attests_inspector_state_preserved"] = (
            surface in SURFACES_REQUIRING_INSPECTOR_STATE_ATTESTATION
        )
        row["attests_mapping_fidelity_preserved"] = (
            surface in SURFACES_REQUIRING_MAPPING_FIDELITY_ATTESTATION
        )
        rows.append(row)
    return rows


def lineage_row(lane, prefix):
    row = base_row(f"row:{prefix}:lineage_admission", lane, "lineage_admission")
    row["evidence_class"] = "automated_functional_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_lineage_break"
    row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_lineage_break"
    row["execution_context_id_binding"] = f"exec:m4:{prefix}:debug_fidelity_lineage"
    return row


def lane_rows(lane, prefix):
    rows = [quality_row(lane, prefix)]
    rows.extend(wedge_rows(lane, prefix))
    rows.extend(inspector_state_rows(lane, prefix))
    rows.extend(mapping_fidelity_badge_rows(lane, prefix))
    rows.extend(inspector_surface_rows(lane, prefix))
    rows.append(lineage_row(lane, prefix))
    return rows


def projection(surface, packet_id):
    return {
        "consumer_surface": surface,
        "projection_ref": f"projection:{surface}",
        "debug_fidelity_truth_packet_id_ref": packet_id,
        "rendered_at": "2026-05-26T12:00:01Z",
        "preserves_same_packet": True,
        "preserves_lane_vocabulary": True,
        "preserves_row_class_vocabulary": True,
        "preserves_support_class_vocabulary": True,
        "preserves_wedge_vocabulary": True,
        "preserves_inspector_state_vocabulary": True,
        "preserves_mapping_fidelity_badge_vocabulary": True,
        "preserves_inspector_surface_vocabulary": True,
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
        "packet:m4:harden_breakpoint_call_stack_variables_watch_evaluate_and:stable"
    )
    workflow_id = (
        "workflow.runtime.harden_breakpoint_call_stack_variables_watch_evaluate_and.stable"
    )
    base = baseline_input(packet_id, workflow_id)
    base.update(
        {
            "record_kind": (
                "harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_stable_packet"
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
    inspector_state_tokens = sorted({row["inspector_state_class"] for row in rows})
    mapping_fidelity_badge_tokens = sorted(
        {row["mapping_fidelity_badge_class"] for row in rows}
    )
    inspector_surface_tokens = sorted({row["inspector_surface_class"] for row in rows})
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
        "inspector_state_tokens": inspector_state_tokens,
        "mapping_fidelity_badge_tokens": mapping_fidelity_badge_tokens,
        "inspector_surface_tokens": inspector_surface_tokens,
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
        "packet:m4:harden_breakpoint_call_stack_variables_watch_evaluate_and:baseline_stable"
    )
    wid = (
        "workflow.runtime.harden_breakpoint_call_stack_variables_watch_evaluate_and.baseline_stable"
    )
    inp = baseline_input(pid, wid)
    return {
        "record_kind": (
            "harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "baseline_stable",
        "scenario": (
            "Baseline stable posture: all four debug-fidelity lanes (local, "
            "remote_helper, container, notebook_bridge) publish a "
            "debug_fidelity_quality row at launch_stable plus the full six-wedge "
            "admission coverage (breakpoint_fidelity, call_stack_fidelity, "
            "variables_fidelity, watch_fidelity, evaluate_fidelity, "
            "debug_console_fidelity), the full six inspector-state admissions "
            "(live, snapshot, stale, limited, unavailable, policy_blocked), the "
            "full six mapping-fidelity badge admissions (exact, approximate, "
            "partial, unavailable, stale, mismatched), the full six "
            "inspector-surface bindings (breakpoint_surface, call_stack_surface, "
            "variables_surface, watch_surface, evaluate_surface, "
            "debug_console_surface) each attesting the inspector-state and "
            "mapping-fidelity vocabularies it is required to preserve, and a "
            "lineage_admission row binding execution_context_id; every row "
            "binds support, known limit, downgrade automation, and evidence "
            "classes; narrowed rows carry their disclosure refs; and all twelve "
            "required consumer projections preserve the packet verbatim."
        ),
        "input": inp,
        "expect": expectations_from_input(inp, "stable", 0),
    }


def unbound_evidence_case():
    pid = (
        "packet:m4:harden_breakpoint_call_stack_variables_watch_evaluate_and:"
        "launch_stable_with_unbound_evidence"
    )
    wid = (
        "workflow.runtime.harden_breakpoint_call_stack_variables_watch_evaluate_and."
        "launch_stable_with_unbound_evidence"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:local:quality":
            row["evidence_class"] = "evidence_unbound"
            break
    return {
        "record_kind": (
            "harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "launch_stable_with_unbound_evidence_blocks_stable",
        "scenario": (
            "The local lane's debug_fidelity_quality row claims launch_stable "
            "while its evidence class is evidence_unbound; the packet blocks the "
            "stable claim because no automated_functional, conformance_suite, "
            "failure_recovery_drill, release_evidence_review, fixture_repo, "
            "benchmark, design_qa, or docs_disclosure evidence backs the row."
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


def missing_inspector_state_case():
    pid = (
        "packet:m4:harden_breakpoint_call_stack_variables_watch_evaluate_and:"
        "missing_inspector_state_for_launch_stable"
    )
    wid = (
        "workflow.runtime.harden_breakpoint_call_stack_variables_watch_evaluate_and."
        "missing_inspector_state_for_launch_stable"
    )
    inp = baseline_input(pid, wid)
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "inspector_state_admission"
            and row["inspector_state_class"] == "policy_blocked"
            and row["lane_class"] == "local_lane"
        )
    ]
    return {
        "record_kind": (
            "harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "missing_inspector_state_for_launch_stable_blocks_stable",
        "scenario": (
            "The local lane claims launch_stable but drops its policy_blocked "
            "inspector_state_admission row; the packet blocks the stable claim "
            "because every launch_stable lane MUST admit all six inspector "
            "states (live, snapshot, stale, limited, unavailable, policy_blocked) "
            "so variables, watches, evaluate, and console-linked inspector rows "
            "cannot infer freshness from generic error copy."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            1,
            expected_findings=["missing_inspector_state_coverage"],
        ),
    }


def missing_mapping_fidelity_badge_case():
    pid = (
        "packet:m4:harden_breakpoint_call_stack_variables_watch_evaluate_and:"
        "missing_mapping_fidelity_badge_for_launch_stable"
    )
    wid = (
        "workflow.runtime.harden_breakpoint_call_stack_variables_watch_evaluate_and."
        "missing_mapping_fidelity_badge_for_launch_stable"
    )
    inp = baseline_input(pid, wid)
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "mapping_fidelity_badge_admission"
            and row["mapping_fidelity_badge_class"] == "mismatched"
            and row["lane_class"] == "local_lane"
        )
    ]
    return {
        "record_kind": (
            "harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "missing_mapping_fidelity_badge_for_launch_stable_blocks_stable",
        "scenario": (
            "The local lane claims launch_stable but drops its mismatched "
            "mapping_fidelity_badge_admission row; the packet blocks the stable "
            "claim because every launch_stable lane MUST admit all six mapping "
            "fidelity badges (exact, approximate, partial, unavailable, stale, "
            "mismatched) so stack, watch, evaluate, and debug-console flows "
            "preserve the badge instead of flattening it into generic error "
            "copy."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            1,
            expected_findings=["missing_mapping_fidelity_badge_coverage"],
        ),
    }


def inspector_surface_missing_state_attestation_case():
    pid = (
        "packet:m4:harden_breakpoint_call_stack_variables_watch_evaluate_and:"
        "inspector_surface_missing_state_attestation"
    )
    wid = (
        "workflow.runtime.harden_breakpoint_call_stack_variables_watch_evaluate_and."
        "inspector_surface_missing_state_attestation"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if (
            row["row_class"] == "inspector_surface_binding"
            and row["lane_class"] == "local_lane"
            and row["inspector_surface_class"] == "watch_surface"
        ):
            row["attests_inspector_state_preserved"] = False
            break
    return {
        "record_kind": (
            "harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "inspector_surface_missing_state_attestation_blocks_stable",
        "scenario": (
            "The local lane's watch_surface inspector_surface_binding row stops "
            "attesting inspector-state preservation; the packet blocks the "
            "stable claim because variables_surface, watch_surface, "
            "evaluate_surface, and debug_console_surface MUST attest "
            "preservation of the live / snapshot / stale / limited / "
            "unavailable / policy_blocked vocabulary so the inspector state "
            "survives into export and support packets."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            2,
            expected_findings=[
                "inspector_surface_missing_inspector_state_attestation",
                "missing_inspector_surface_coverage",
            ],
        ),
    }


def narrowed_row_missing_disclosure_ref_case():
    pid = (
        "packet:m4:harden_breakpoint_call_stack_variables_watch_evaluate_and:"
        "narrowed_row_missing_disclosure_ref"
    )
    wid = (
        "workflow.runtime.harden_breakpoint_call_stack_variables_watch_evaluate_and."
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
            "harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "narrowed_row_missing_disclosure_ref_blocks_stable",
        "scenario": (
            "The local lane's debug_fidelity_quality row narrows to "
            "launch_stable_below but drops its disclosure ref; the packet "
            "blocks the stable claim because every row narrowed below "
            "launch_stable must surface a disclosure ref so reviewers can see "
            "why the lane downgraded."
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


def projection_collapses_inspector_state_vocabulary_case():
    pid = (
        "packet:m4:harden_breakpoint_call_stack_variables_watch_evaluate_and:"
        "projection_collapses_inspector_state_vocabulary"
    )
    wid = (
        "workflow.runtime.harden_breakpoint_call_stack_variables_watch_evaluate_and."
        "projection_collapses_inspector_state_vocabulary"
    )
    inp = baseline_input(pid, wid)
    for proj in inp["consumer_projections"]:
        if proj["consumer_surface"] == "help_about":
            proj["preserves_inspector_state_vocabulary"] = False
            break
    return {
        "record_kind": (
            "harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "projection_collapses_inspector_state_vocabulary_blocks_stable",
        "scenario": (
            "The help_about consumer projection drops the inspector-state "
            "vocabulary; the packet blocks the stable claim because a "
            "downstream surface that collapses the six inspector-state labels "
            "(live, snapshot, stale, limited, unavailable, policy_blocked) "
            "cannot preserve debug-fidelity truth verbatim."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            3,
            expected_findings=[
                "inspector_state_vocabulary_collapsed",
                "missing_consumer_projection",
                "consumer_projection_drift",
            ],
        ),
    }


def raw_source_material_case():
    pid = (
        "packet:m4:harden_breakpoint_call_stack_variables_watch_evaluate_and:"
        "raw_source_material"
    )
    wid = (
        "workflow.runtime.harden_breakpoint_call_stack_variables_watch_evaluate_and."
        "raw_source_material"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:local:quality":
            row["raw_source_material_excluded"] = False
            break
    return {
        "record_kind": (
            "harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "raw_source_material_blocks_stable",
        "scenario": (
            "The local lane's debug_fidelity_quality row admits raw debugger "
            "payloads, raw stack frames, raw memory bytes, raw watch "
            "expressions, raw evaluate input/output, raw console scrollback "
            "bodies, raw command lines, or raw process environment bytes past "
            "the boundary; the packet blocks the stable claim because raw "
            "runtime material, secrets, and ambient credentials must never "
            "leak through the debug-fidelity boundary."
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
        missing_inspector_state_case(),
        missing_mapping_fidelity_badge_case(),
        inspector_surface_missing_state_attestation_case(),
        narrowed_row_missing_disclosure_ref_case(),
        projection_collapses_inspector_state_vocabulary_case(),
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
            "# harden_breakpoint_call_stack_variables_watch_evaluate_and fixture corpus\n\n"
            "Fixture corpus for the M4 stable harden-breakpoint / call-stack / "
            "variables / watch / evaluate / debug-console fidelity truth packet "
            "(`schemas/runtime/"
            "harden_breakpoint_call_stack_variables_watch_evaluate_and_truth.schema.json`).\n\n"
            "Each fixture is a `DebugFidelityTruthPacketInput` with an "
            "`expect` block that pins the materialized packet's promotion "
            "state, finding count, lane and row-class token sets, "
            "support-class, wedge, inspector-state, mapping-fidelity badge, "
            "inspector-surface, known-limit, downgrade-automation, and "
            "evidence-class tokens, and the support-export safety verdict. "
            "Tests in `crates/aureline-runtime/tests/"
            "harden_breakpoint_call_stack_variables_watch_evaluate_and_truth_packet.rs` "
            "load each case and assert that "
            "`DebugFidelityTruthPacket::materialize` agrees.\n\n"
            "Cases:\n\n"
            "- `baseline_stable.json` — All four debug-fidelity lanes (local, "
            "remote_helper, container, notebook_bridge) carry one "
            "`debug_fidelity_quality` row at `launch_stable` plus the full "
            "six-wedge admission coverage (breakpoint_fidelity, "
            "call_stack_fidelity, variables_fidelity, watch_fidelity, "
            "evaluate_fidelity, debug_console_fidelity), the full six "
            "inspector-state admissions (live, snapshot, stale, limited, "
            "unavailable, policy_blocked), the full six mapping-fidelity "
            "badge admissions (exact, approximate, partial, unavailable, "
            "stale, mismatched), the full six inspector-surface bindings "
            "(breakpoint_surface, call_stack_surface, variables_surface, "
            "watch_surface, evaluate_surface, debug_console_surface), each "
            "attesting the inspector-state and mapping-fidelity vocabularies "
            "it is required to preserve, and a lineage_admission row binding "
            "`execution_context_id`. All twelve required consumer "
            "projections preserve the packet verbatim.\n"
            "- `launch_stable_with_unbound_evidence_blocks_stable.json` — The "
            "local lane's quality row claims `launch_stable` while its "
            "evidence is `evidence_unbound`; the packet blocks the stable "
            "claim.\n"
            "- `missing_inspector_state_for_launch_stable_blocks_stable.json` — "
            "The local lane claims `launch_stable` but the `policy_blocked` "
            "inspector_state_admission row is missing; the packet blocks the "
            "stable claim.\n"
            "- `missing_mapping_fidelity_badge_for_launch_stable_blocks_stable.json` — "
            "The local lane claims `launch_stable` but the `mismatched` "
            "mapping_fidelity_badge_admission row is missing; the packet "
            "blocks the stable claim.\n"
            "- `inspector_surface_missing_state_attestation_blocks_stable.json` — "
            "The local lane's `watch_surface` inspector_surface_binding row "
            "stops attesting inspector-state preservation; the packet blocks "
            "the stable claim because the four state-bearing surfaces "
            "(variables, watch, evaluate, debug_console) MUST attest "
            "preservation of the inspector-state vocabulary.\n"
            "- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — "
            "The local lane's quality row narrows to `launch_stable_below` "
            "but drops its disclosure ref; the packet blocks the stable "
            "claim.\n"
            "- `projection_collapses_inspector_state_vocabulary_blocks_stable.json` "
            "— The `help_about` consumer projection drops the "
            "inspector-state vocabulary; the packet blocks the stable "
            "claim.\n"
            "- `raw_source_material_blocks_stable.json` — The local lane's "
            "quality row admits raw debugger payloads, raw stack frames, "
            "raw memory bytes, raw watch expressions, raw evaluate input/"
            "output, raw console scrollback bodies, raw command lines, or "
            "raw env bytes past the boundary; the packet blocks the stable "
            "claim because raw runtime material must never leak through the "
            "debug-fidelity boundary.\n"
        )


if __name__ == "__main__":
    main()
