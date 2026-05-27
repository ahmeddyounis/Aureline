#!/usr/bin/env python3
"""Regenerate the M4 inspector-parity truth packet artifact and fixture corpus.

Mirrors the Rust unit-test sample input in
crates/aureline-runtime/src/finalize_environment_and_toolchain_manager_parity_across_ui/mod.rs.
The generator is the canonical seed for the checked-in artifact and the
narrowed-below-stable fixture cases used by the integration tests in
crates/aureline-runtime/tests/finalize_environment_and_toolchain_manager_parity_across_ui_truth_packet.rs.

Run from anywhere:
    python3 tools/regenerate_finalize_environment_and_toolchain_manager_parity_across_ui_truth_packet.py
"""
import json
import os

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

DOC_REF = (
    "docs/runtime/m4/"
    "finalize-environment-and-toolchain-manager-parity-across-ui.md"
)
FIXTURE_DIR = (
    "fixtures/runtime/m4/"
    "finalize_environment_and_toolchain_manager_parity_across_ui"
)
ARTIFACT_PATH = (
    "artifacts/runtime/m4/"
    "finalize_environment_and_toolchain_manager_parity_across_ui_truth_packet.json"
)
SCHEMA_REF = (
    "schemas/runtime/"
    "finalize_environment_and_toolchain_manager_parity_across_ui_truth.schema.json"
)

LANES = [
    ("local_lane", "local"),
    ("remote_helper_lane", "remote"),
    ("container_lane", "container"),
    ("managed_lane", "managed"),
]

INSPECTOR_FIELDS = [
    "interpreter",
    "sdk",
    "shell",
    "container_target",
    "remote_target",
    "activator",
    "trust_state",
    "policy_source",
]

PARITY_SURFACES = ["ui", "cli_headless", "help_about", "support_export"]

RECOVERY_STATES = [
    "reconnect",
    "restore_no_rerun",
    "blocked_target",
    "degraded_helper",
    "artifact_provenance",
]

CONSUMER_SURFACES = [
    "editor_run_surface",
    "terminal_pane",
    "task_panel",
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
        "inspector_field_class": "not_applicable",
        "parity_surface_class": "not_applicable",
        "recovery_state_class": "not_applicable",
        "evidence_class": "fixture_repo_evidence",
        "known_limit_class": "none_declared",
        "downgrade_automation_class": "auto_narrow_on_lineage_break",
        "confidence_class": "high_confidence",
        "evidence_refs": [FIXTURE_DIR],
        "disclosure_ref": f"{DOC_REF}#auto_narrow_on_lineage_break",
        "restore_preserves_no_rerun": False,
        "raw_source_material_excluded": True,
        "secrets_excluded": True,
        "ambient_authority_excluded": True,
        "captured_at": TIMESTAMP,
    }


def quality_row(lane, prefix):
    row = base_row(f"row:{prefix}:quality", lane, "inspector_parity_quality")
    row["evidence_class"] = "release_evidence_review"
    row["downgrade_automation_class"] = "auto_block_on_missing_evidence"
    row["disclosure_ref"] = f"{DOC_REF}#auto_block_on_missing_evidence"
    row["evidence_refs"] = [DOC_REF, FIXTURE_DIR]
    return row


def field_rows(lane, prefix):
    rows = []
    for field in INSPECTOR_FIELDS:
        row = base_row(
            f"row:{prefix}:field:{field}", lane, "inspector_field_admission"
        )
        row["inspector_field_class"] = field
        row["evidence_class"] = "conformance_suite_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_inspector_field_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_inspector_field_gap"
        rows.append(row)
    return rows


def surface_rows(lane, prefix):
    rows = []
    for surface in PARITY_SURFACES:
        row = base_row(
            f"row:{prefix}:surface:{surface}", lane, "parity_surface_binding"
        )
        row["parity_surface_class"] = surface
        row["evidence_class"] = "conformance_suite_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_parity_surface_break"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_parity_surface_break"
        rows.append(row)
    return rows


def recovery_rows(lane, prefix):
    rows = []
    for state in RECOVERY_STATES:
        row = base_row(
            f"row:{prefix}:recovery:{state}", lane, "recovery_admission"
        )
        row["recovery_state_class"] = state
        row["evidence_class"] = "failure_recovery_drill_evidence"
        if state == "restore_no_rerun":
            row["downgrade_automation_class"] = "auto_narrow_on_silent_rerun"
            row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_silent_rerun"
            row["restore_preserves_no_rerun"] = True
        else:
            row["downgrade_automation_class"] = "auto_narrow_on_recovery_state_gap"
            row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_recovery_state_gap"
        rows.append(row)
    return rows


def toolchain_manager_row(lane, prefix):
    row = base_row(
        f"row:{prefix}:toolchain_manager_admission",
        lane,
        "toolchain_manager_admission",
    )
    row["evidence_class"] = "automated_functional_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_toolchain_manager_drift"
    row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_toolchain_manager_drift"
    row["toolchain_manager_id_binding"] = f"toolchain_manager:m4:{prefix}"
    return row


def lineage_row(lane, prefix):
    row = base_row(
        f"row:{prefix}:lineage_admission", lane, "lineage_admission"
    )
    row["evidence_class"] = "automated_functional_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_lineage_break"
    row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_lineage_break"
    row["execution_context_id_binding"] = f"exec:m4:{prefix}:lineage"
    return row


def lane_rows(lane, prefix):
    rows = [quality_row(lane, prefix)]
    rows.extend(field_rows(lane, prefix))
    rows.extend(surface_rows(lane, prefix))
    rows.extend(recovery_rows(lane, prefix))
    rows.append(toolchain_manager_row(lane, prefix))
    rows.append(lineage_row(lane, prefix))
    return rows


def projection(surface, packet_id, suffix=""):
    return {
        "consumer_surface": surface,
        "projection_ref": f"projection:{surface}{suffix}",
        "inspector_parity_packet_id_ref": packet_id,
        "rendered_at": "2026-05-26T12:00:01Z",
        "preserves_same_packet": True,
        "preserves_lane_vocabulary": True,
        "preserves_row_class_vocabulary": True,
        "preserves_support_class_vocabulary": True,
        "preserves_inspector_field_vocabulary": True,
        "preserves_parity_surface_vocabulary": True,
        "preserves_recovery_state_vocabulary": True,
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
        "packet:m4:finalize_environment_and_toolchain_manager_parity_across_ui:stable"
    )
    workflow_id = (
        "workflow.runtime."
        "finalize_environment_and_toolchain_manager_parity_across_ui.stable"
    )
    base = baseline_input(packet_id, workflow_id)
    base.update(
        {
            "record_kind":
                "finalize_environment_and_toolchain_manager_parity_across_ui_truth_stable_packet",
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
    inspector_field_tokens = sorted({row["inspector_field_class"] for row in rows})
    parity_surface_tokens = sorted({row["parity_surface_class"] for row in rows})
    recovery_state_tokens = sorted({row["recovery_state_class"] for row in rows})
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
        "inspector_field_tokens": inspector_field_tokens,
        "parity_surface_tokens": parity_surface_tokens,
        "recovery_state_tokens": recovery_state_tokens,
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
        "packet:m4:finalize_environment_and_toolchain_manager_parity_across_ui:"
        "baseline_stable"
    )
    wid = (
        "workflow.runtime."
        "finalize_environment_and_toolchain_manager_parity_across_ui.baseline_stable"
    )
    inp = baseline_input(pid, wid)
    return {
        "record_kind":
            "finalize_environment_and_toolchain_manager_parity_across_ui_truth_stable_case",
        "schema_version": 1,
        "case_name": "baseline_stable",
        "scenario": (
            "Baseline stable posture: all four execution-context lanes "
            "(local, remote_helper, container, managed) publish an "
            "inspector_parity_quality row at launch_stable plus the full "
            "eight-field inspector coverage (interpreter, sdk, shell, "
            "container_target, remote_target, activator, trust_state, "
            "policy_source), the full four-surface parity binding coverage "
            "(ui, cli_headless, help_about, support_export), the full "
            "five-recovery-state admission coverage (reconnect, "
            "restore_no_rerun, blocked_target, degraded_helper, "
            "artifact_provenance), a toolchain_manager_admission row binding "
            "a manager id, and a lineage_admission row binding "
            "execution_context_id; every row binds support, known limit, "
            "downgrade automation, and evidence classes; narrowed rows carry "
            "their disclosure refs; and all eight required consumer "
            "projections preserve the packet verbatim."
        ),
        "input": inp,
        "expect": expectations_from_input(inp, "stable", 0),
    }


def unbound_evidence_case():
    pid = (
        "packet:m4:finalize_environment_and_toolchain_manager_parity_across_ui:"
        "launch_stable_with_unbound_evidence"
    )
    wid = (
        "workflow.runtime."
        "finalize_environment_and_toolchain_manager_parity_across_ui."
        "launch_stable_with_unbound_evidence"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:local:quality":
            row["evidence_class"] = "evidence_unbound"
            break
    return {
        "record_kind":
            "finalize_environment_and_toolchain_manager_parity_across_ui_truth_stable_case",
        "schema_version": 1,
        "case_name": "launch_stable_with_unbound_evidence_blocks_stable",
        "scenario": (
            "The local lane's inspector_parity_quality row claims "
            "launch_stable while its evidence class is evidence_unbound; the "
            "packet blocks the stable claim because no automated_functional, "
            "conformance_suite, failure_recovery_drill, release_evidence_review, "
            "fixture_repo, benchmark, design_qa, or docs_disclosure evidence "
            "backs the row."
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


def missing_inspector_field_case():
    pid = (
        "packet:m4:finalize_environment_and_toolchain_manager_parity_across_ui:"
        "missing_inspector_field_for_launch_stable"
    )
    wid = (
        "workflow.runtime."
        "finalize_environment_and_toolchain_manager_parity_across_ui."
        "missing_inspector_field_for_launch_stable"
    )
    inp = baseline_input(pid, wid)
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "inspector_field_admission"
            and row["inspector_field_class"] == "policy_source"
            and row["lane_class"] == "local_lane"
        )
    ]
    return {
        "record_kind":
            "finalize_environment_and_toolchain_manager_parity_across_ui_truth_stable_case",
        "schema_version": 1,
        "case_name": "missing_inspector_field_for_launch_stable_blocks_stable",
        "scenario": (
            "The local lane claims launch_stable but drops its policy_source "
            "inspector_field_admission row; the packet blocks the stable "
            "claim because every launch_stable lane must admit all eight "
            "inspector fields (interpreter, sdk, shell, container_target, "
            "remote_target, activator, trust_state, policy_source) so the "
            "inspector returns the same fields across the UI, CLI/headless, "
            "Help/About, and support/export parity surfaces."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            1,
            expected_findings=["missing_inspector_field_coverage"],
        ),
    }


def narrowed_row_missing_disclosure_ref_case():
    pid = (
        "packet:m4:finalize_environment_and_toolchain_manager_parity_across_ui:"
        "narrowed_row_missing_disclosure_ref"
    )
    wid = (
        "workflow.runtime."
        "finalize_environment_and_toolchain_manager_parity_across_ui."
        "narrowed_row_missing_disclosure_ref"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:local:quality":
            row["support_class"] = "launch_stable_below"
            row.pop("disclosure_ref", None)
            break
    return {
        "record_kind":
            "finalize_environment_and_toolchain_manager_parity_across_ui_truth_stable_case",
        "schema_version": 1,
        "case_name": "narrowed_row_missing_disclosure_ref_blocks_stable",
        "scenario": (
            "The local lane's inspector_parity_quality row narrows to "
            "launch_stable_below but drops its disclosure ref; the packet "
            "blocks the stable claim because every row narrowed below "
            "launch_stable must surface a disclosure ref so reviewers can "
            "see why the lane downgraded."
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


def projection_collapses_parity_surface_vocabulary_case():
    pid = (
        "packet:m4:finalize_environment_and_toolchain_manager_parity_across_ui:"
        "projection_collapses_parity_surface_vocabulary"
    )
    wid = (
        "workflow.runtime."
        "finalize_environment_and_toolchain_manager_parity_across_ui."
        "projection_collapses_parity_surface_vocabulary"
    )
    inp = baseline_input(pid, wid)
    for proj in inp["consumer_projections"]:
        if proj["consumer_surface"] == "help_about":
            proj["preserves_parity_surface_vocabulary"] = False
            break
    return {
        "record_kind":
            "finalize_environment_and_toolchain_manager_parity_across_ui_truth_stable_case",
        "schema_version": 1,
        "case_name": "projection_collapses_parity_surface_vocabulary_blocks_stable",
        "scenario": (
            "The help_about consumer projection drops the parity-surface "
            "vocabulary; the packet blocks the stable claim because a "
            "downstream surface that collapses the four parity surfaces "
            "(ui, cli_headless, help_about, support_export) cannot "
            "preserve inspector-parity truth verbatim."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            3,
            expected_findings=[
                "parity_surface_vocabulary_collapsed",
                "missing_consumer_projection",
                "consumer_projection_drift",
            ],
        ),
    }


def raw_source_material_case():
    pid = (
        "packet:m4:finalize_environment_and_toolchain_manager_parity_across_ui:"
        "raw_source_material"
    )
    wid = (
        "workflow.runtime."
        "finalize_environment_and_toolchain_manager_parity_across_ui."
        "raw_source_material"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:local:quality":
            row["raw_source_material_excluded"] = False
            break
    return {
        "record_kind":
            "finalize_environment_and_toolchain_manager_parity_across_ui_truth_stable_case",
        "schema_version": 1,
        "case_name": "raw_source_material_blocks_stable",
        "scenario": (
            "The local lane's inspector_parity_quality row admits raw "
            "command lines, raw process environment bytes, or raw capsule "
            "bodies past the boundary; the packet blocks the stable claim "
            "because raw runtime material, secrets, and ambient credentials "
            "must never leak through the inspector-parity boundary."
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
        missing_inspector_field_case(),
        narrowed_row_missing_disclosure_ref_case(),
        projection_collapses_parity_surface_vocabulary_case(),
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
            "# finalize_environment_and_toolchain_manager_parity_across_ui "
            "fixture corpus\n\n"
            "Fixture corpus for the M4 stable inspector-parity truth packet "
            "(`schemas/runtime/"
            "finalize_environment_and_toolchain_manager_parity_across_ui_truth.schema.json`).\n\n"
            "Each fixture is an "
            "`InspectorParityTruthPacketInput` with an `expect` block that "
            "pins the materialized packet's promotion state, finding count, "
            "lane and row-class token sets, support-class, inspector-field, "
            "parity-surface, recovery-state, known-limit, downgrade-automation, "
            "and evidence-class tokens, and the support-export safety verdict. "
            "Tests in `crates/aureline-runtime/tests/"
            "finalize_environment_and_toolchain_manager_parity_across_ui_truth_packet.rs` "
            "load each case and assert that materialization matches the "
            "expectation block.\n\n"
            "Regenerate via:\n\n"
            "```bash\n"
            "python3 tools/"
            "regenerate_finalize_environment_and_toolchain_manager_parity_across_ui_truth_packet.py\n"
            "```\n"
        )


if __name__ == "__main__":
    main()
