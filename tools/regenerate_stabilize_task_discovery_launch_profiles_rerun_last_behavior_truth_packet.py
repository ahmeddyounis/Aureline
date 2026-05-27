#!/usr/bin/env python3
"""Regenerate the M4 stabilize-task-discovery / launch-profiles / rerun-last / task-event truth packet artifact and fixture corpus.

Mirrors the Rust unit-test sample input in
crates/aureline-runtime/src/stabilize_task_discovery_launch_profiles_rerun_last_behavior/mod.rs.
The generator is the canonical seed for the checked-in artifact and the
narrowed-below-stable fixture cases used by the integration tests in
crates/aureline-runtime/tests/stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_packet.rs.

Run from anywhere:
    python3 tools/regenerate_stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_packet.py
"""
import json
import os

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

DOC_REF = "docs/runtime/m4/stabilize-task-discovery-launch-profiles-rerun-last-behavior.md"
FIXTURE_DIR = (
    "fixtures/runtime/m4/stabilize_task_discovery_launch_profiles_rerun_last_behavior"
)
ARTIFACT_PATH = (
    "artifacts/runtime/m4/"
    "stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_packet.json"
)
SCHEMA_REF = (
    "schemas/runtime/"
    "stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth.schema.json"
)

LANES = [
    ("local_lane", "local"),
    ("remote_helper_lane", "remote"),
    ("notebook_lane", "notebook"),
    ("imported_provider_lane", "imported"),
]

WEDGES = [
    "task_discovery",
    "launch_profile",
    "rerun_last",
    "task_event",
]

ENVELOPE_FIELDS = [
    "event_id",
    "execution_context_ref",
    "adapter_identity",
    "provider_identity",
    "confidence_flag",
    "fallback_flag",
]

DOWNSTREAM_SURFACES = [
    "problems",
    "output_channel",
    "evidence_export",
    "rerun_surface",
]

CONSUMER_SURFACES = [
    "editor_run_surface",
    "task_panel",
    "problems_panel",
    "output_channel",
    "evidence_export",
    "rerun_surface",
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
        "envelope_field_class": "not_applicable",
        "downstream_surface_class": "not_applicable",
        "evidence_class": "fixture_repo_evidence",
        "known_limit_class": "none_declared",
        "downgrade_automation_class": "auto_narrow_on_lineage_break",
        "confidence_class": "high_confidence",
        "evidence_refs": [FIXTURE_DIR],
        "disclosure_ref": f"{DOC_REF}#auto_narrow_on_lineage_break",
        "additive_detail_preserved": False,
        "raw_source_material_excluded": True,
        "secrets_excluded": True,
        "ambient_authority_excluded": True,
        "captured_at": TIMESTAMP,
    }


def quality_row(lane, prefix):
    row = base_row(f"row:{prefix}:quality", lane, "task_event_truth_quality")
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


def envelope_field_rows(lane, prefix):
    rows = []
    for field in ENVELOPE_FIELDS:
        row = base_row(
            f"row:{prefix}:field:{field}", lane, "envelope_field_binding"
        )
        row["envelope_field_class"] = field
        row["evidence_class"] = "automated_functional_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_envelope_field_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_envelope_field_gap"
        rows.append(row)
    return rows


def surface_rows(lane, prefix):
    rows = []
    for surface in DOWNSTREAM_SURFACES:
        row = base_row(
            f"row:{prefix}:surface:{surface}", lane, "surface_binding"
        )
        row["downstream_surface_class"] = surface
        row["evidence_class"] = "conformance_suite_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_downstream_surface_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_downstream_surface_gap"
        rows.append(row)
    return rows


def additive_detail_row(lane, prefix):
    row = base_row(
        f"row:{prefix}:additive_detail", lane, "additive_detail_preservation"
    )
    row["evidence_class"] = "failure_recovery_drill_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_additive_detail_dropped"
    row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_additive_detail_dropped"
    row["additive_detail_preserved"] = True
    return row


def lineage_row(lane, prefix):
    row = base_row(f"row:{prefix}:lineage_admission", lane, "lineage_admission")
    row["evidence_class"] = "automated_functional_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_lineage_break"
    row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_lineage_break"
    row["execution_context_id_binding"] = f"exec:m4:{prefix}:task_event_lineage"
    return row


def lane_rows(lane, prefix):
    rows = [quality_row(lane, prefix)]
    rows.extend(wedge_rows(lane, prefix))
    rows.extend(envelope_field_rows(lane, prefix))
    rows.extend(surface_rows(lane, prefix))
    rows.append(additive_detail_row(lane, prefix))
    rows.append(lineage_row(lane, prefix))
    return rows


def projection(surface, packet_id):
    return {
        "consumer_surface": surface,
        "projection_ref": f"projection:{surface}",
        "task_event_truth_packet_id_ref": packet_id,
        "rendered_at": "2026-05-26T12:00:01Z",
        "preserves_same_packet": True,
        "preserves_lane_vocabulary": True,
        "preserves_row_class_vocabulary": True,
        "preserves_support_class_vocabulary": True,
        "preserves_wedge_vocabulary": True,
        "preserves_envelope_field_vocabulary": True,
        "preserves_downstream_surface_vocabulary": True,
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
        "packet:m4:stabilize_task_discovery_launch_profiles_rerun_last_behavior:stable"
    )
    workflow_id = (
        "workflow.runtime.stabilize_task_discovery_launch_profiles_rerun_last_behavior.stable"
    )
    base = baseline_input(packet_id, workflow_id)
    base.update(
        {
            "record_kind": (
                "stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_stable_packet"
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
    envelope_field_tokens = sorted({row["envelope_field_class"] for row in rows})
    downstream_surface_tokens = sorted(
        {row["downstream_surface_class"] for row in rows}
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
        "envelope_field_tokens": envelope_field_tokens,
        "downstream_surface_tokens": downstream_surface_tokens,
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
        "packet:m4:stabilize_task_discovery_launch_profiles_rerun_last_behavior:baseline_stable"
    )
    wid = (
        "workflow.runtime.stabilize_task_discovery_launch_profiles_rerun_last_behavior"
        ".baseline_stable"
    )
    inp = baseline_input(pid, wid)
    return {
        "record_kind": (
            "stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "baseline_stable",
        "scenario": (
            "Baseline stable posture: all four task-event lanes (local, "
            "remote_helper, notebook, imported_provider) publish a "
            "task_event_truth_quality row at launch_stable plus the full "
            "four-wedge admission coverage (task_discovery, launch_profile, "
            "rerun_last, task_event), the full six envelope-field bindings "
            "(event_id, execution_context_ref, adapter_identity, "
            "provider_identity, confidence_flag, fallback_flag), the full "
            "four downstream surface bindings (problems, output_channel, "
            "evidence_export, rerun_surface), an additive_detail_preservation "
            "row attesting preserved additive detail, and a lineage_admission "
            "row binding execution_context_id; every row binds support, known "
            "limit, downgrade automation, and evidence classes; narrowed rows "
            "carry their disclosure refs; and all eleven required consumer "
            "projections preserve the packet verbatim."
        ),
        "input": inp,
        "expect": expectations_from_input(inp, "stable", 0),
    }


def unbound_evidence_case():
    pid = (
        "packet:m4:stabilize_task_discovery_launch_profiles_rerun_last_behavior:"
        "launch_stable_with_unbound_evidence"
    )
    wid = (
        "workflow.runtime.stabilize_task_discovery_launch_profiles_rerun_last_behavior."
        "launch_stable_with_unbound_evidence"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:local:quality":
            row["evidence_class"] = "evidence_unbound"
            break
    return {
        "record_kind": (
            "stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "launch_stable_with_unbound_evidence_blocks_stable",
        "scenario": (
            "The local lane's task_event_truth_quality row claims launch_stable "
            "while its evidence class is evidence_unbound; the packet blocks "
            "the stable claim because no automated_functional, "
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


def missing_envelope_field_case():
    pid = (
        "packet:m4:stabilize_task_discovery_launch_profiles_rerun_last_behavior:"
        "missing_envelope_field_for_launch_stable"
    )
    wid = (
        "workflow.runtime.stabilize_task_discovery_launch_profiles_rerun_last_behavior."
        "missing_envelope_field_for_launch_stable"
    )
    inp = baseline_input(pid, wid)
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "envelope_field_binding"
            and row["envelope_field_class"] == "fallback_flag"
            and row["lane_class"] == "local_lane"
        )
    ]
    return {
        "record_kind": (
            "stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "missing_envelope_field_for_launch_stable_blocks_stable",
        "scenario": (
            "The local lane claims launch_stable but drops its fallback_flag "
            "envelope_field_binding row; the packet blocks the stable claim "
            "because every launch_stable lane must bind all six canonical "
            "envelope fields (event_id, execution_context_ref, "
            "adapter_identity, provider_identity, confidence_flag, "
            "fallback_flag) so downstream Problems, output-channel, "
            "evidence-export, and rerun surfaces do not invent a second truth "
            "model."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            1,
            expected_findings=["missing_envelope_field_coverage"],
        ),
    }


def additive_detail_flattening_case():
    pid = (
        "packet:m4:stabilize_task_discovery_launch_profiles_rerun_last_behavior:"
        "additive_detail_admits_flattening"
    )
    wid = (
        "workflow.runtime.stabilize_task_discovery_launch_profiles_rerun_last_behavior."
        "additive_detail_admits_flattening"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:local:additive_detail":
            row["additive_detail_preserved"] = False
            break
    return {
        "record_kind": (
            "stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "additive_detail_admits_flattening_blocks_stable",
        "scenario": (
            "The local lane's additive_detail_preservation row stops attesting "
            "that additive detail is preserved (admits flattening into display "
            "text); the packet blocks the stable claim because local, "
            "remote/helper, notebook, and imported-provider runs must "
            "serialize into one task-event vocabulary with additive detail "
            "preserved rather than flattened into display text."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            2,
            expected_findings=[
                "additive_detail_row_admits_flattening",
                "missing_additive_detail_preservation",
            ],
        ),
    }


def narrowed_row_missing_disclosure_ref_case():
    pid = (
        "packet:m4:stabilize_task_discovery_launch_profiles_rerun_last_behavior:"
        "narrowed_row_missing_disclosure_ref"
    )
    wid = (
        "workflow.runtime.stabilize_task_discovery_launch_profiles_rerun_last_behavior."
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
            "stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "narrowed_row_missing_disclosure_ref_blocks_stable",
        "scenario": (
            "The local lane's task_event_truth_quality row narrows to "
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


def projection_collapses_envelope_field_vocabulary_case():
    pid = (
        "packet:m4:stabilize_task_discovery_launch_profiles_rerun_last_behavior:"
        "projection_collapses_envelope_field_vocabulary"
    )
    wid = (
        "workflow.runtime.stabilize_task_discovery_launch_profiles_rerun_last_behavior."
        "projection_collapses_envelope_field_vocabulary"
    )
    inp = baseline_input(pid, wid)
    for proj in inp["consumer_projections"]:
        if proj["consumer_surface"] == "help_about":
            proj["preserves_envelope_field_vocabulary"] = False
            break
    return {
        "record_kind": (
            "stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "projection_collapses_envelope_field_vocabulary_blocks_stable",
        "scenario": (
            "The help_about consumer projection drops the envelope-field "
            "vocabulary; the packet blocks the stable claim because a "
            "downstream surface that collapses the six canonical envelope "
            "fields (event_id, execution_context_ref, adapter_identity, "
            "provider_identity, confidence_flag, fallback_flag) cannot "
            "preserve task-event truth verbatim."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            3,
            expected_findings=[
                "envelope_field_vocabulary_collapsed",
                "missing_consumer_projection",
                "consumer_projection_drift",
            ],
        ),
    }


def raw_source_material_case():
    pid = (
        "packet:m4:stabilize_task_discovery_launch_profiles_rerun_last_behavior:"
        "raw_source_material"
    )
    wid = (
        "workflow.runtime.stabilize_task_discovery_launch_profiles_rerun_last_behavior."
        "raw_source_material"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:local:quality":
            row["raw_source_material_excluded"] = False
            break
    return {
        "record_kind": (
            "stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "raw_source_material_blocks_stable",
        "scenario": (
            "The local lane's task_event_truth_quality row admits raw command "
            "lines, raw process environment bytes, or raw capsule bodies past "
            "the boundary; the packet blocks the stable claim because raw "
            "runtime material, secrets, and ambient credentials must never "
            "leak through the task-event boundary."
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
        missing_envelope_field_case(),
        additive_detail_flattening_case(),
        narrowed_row_missing_disclosure_ref_case(),
        projection_collapses_envelope_field_vocabulary_case(),
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
            "# stabilize_task_discovery_launch_profiles_rerun_last_behavior fixture corpus\n\n"
            "Fixture corpus for the M4 stable task-discovery / launch-profile / "
            "rerun-last / task-event truth packet (`schemas/runtime/"
            "stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth.schema.json`).\n\n"
            "Each fixture is a `TaskEventTruthPacketInput` with an `expect` "
            "block that pins the materialized packet's promotion state, "
            "finding count, lane and row-class token sets, support-class, "
            "wedge, envelope-field, downstream-surface, known-limit, "
            "downgrade-automation, and evidence-class tokens, and the "
            "support-export safety verdict. Tests in `crates/aureline-runtime/tests/"
            "stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_packet.rs` "
            "load each case and assert that `TaskEventTruthPacket::materialize` "
            "agrees.\n\n"
            "Cases:\n\n"
            "- `baseline_stable.json` — All four task-event lanes (local, "
            "remote_helper, notebook, imported_provider) carry one "
            "`task_event_truth_quality` row at `launch_stable` plus the full "
            "four-wedge admission coverage (task_discovery, launch_profile, "
            "rerun_last, task_event), the full six envelope-field bindings "
            "(event_id, execution_context_ref, adapter_identity, "
            "provider_identity, confidence_flag, fallback_flag), the full "
            "four downstream surface bindings (problems, output_channel, "
            "evidence_export, rerun_surface), an additive_detail_preservation "
            "row, and a lineage_admission row binding "
            "`execution_context_id`. All eleven required consumer projections "
            "preserve the packet verbatim.\n"
            "- `launch_stable_with_unbound_evidence_blocks_stable.json` — The "
            "local lane's quality row claims `launch_stable` while its "
            "evidence is `evidence_unbound`; the packet blocks the stable "
            "claim.\n"
            "- `missing_envelope_field_for_launch_stable_blocks_stable.json` — "
            "The local lane claims `launch_stable` but the `fallback_flag` "
            "envelope-field binding is missing; the packet blocks the stable "
            "claim.\n"
            "- `additive_detail_admits_flattening_blocks_stable.json` — The "
            "local lane's additive-detail row stops attesting that additive "
            "detail is preserved; the packet blocks the stable claim because "
            "local, remote/helper, notebook, and imported-provider runs must "
            "preserve additive detail rather than flatten it into display "
            "text.\n"
            "- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — The "
            "local lane's quality row narrows to `launch_stable_below` but "
            "drops its disclosure ref; the packet blocks the stable claim.\n"
            "- `projection_collapses_envelope_field_vocabulary_blocks_stable.json` "
            "— The `help_about` consumer projection drops the envelope-field "
            "vocabulary; the packet blocks the stable claim.\n"
            "- `raw_source_material_blocks_stable.json` — The local lane's "
            "quality row admits raw command lines, env bytes, or capsule "
            "bodies past the boundary; the packet blocks the stable claim "
            "because raw runtime material must never leak through the "
            "task-event boundary.\n"
        )


if __name__ == "__main__":
    main()
