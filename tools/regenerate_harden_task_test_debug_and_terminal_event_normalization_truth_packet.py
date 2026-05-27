#!/usr/bin/env python3
"""Regenerate the M4 harden-task-test-debug-and-terminal-event-normalization truth packet artifact and fixture corpus.

Mirrors the Rust unit-test sample input in
crates/aureline-terminal/src/harden_task_test_debug_and_terminal_event_normalization/mod.rs.
The generator is the canonical seed for the checked-in artifact and the
narrowed-below-stable fixture cases used by the integration tests in
crates/aureline-terminal/tests/harden_task_test_debug_and_terminal_event_normalization_truth_packet.rs.

Run from anywhere:
    python3 tools/regenerate_harden_task_test_debug_and_terminal_event_normalization_truth_packet.py
"""
import json
import os

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

DOC_REF = "docs/runtime/m4/harden-task-test-debug-and-terminal-event-normalization.md"
FIXTURE_DIR = (
    "fixtures/runtime/m4/harden_task_test_debug_and_terminal_event_normalization"
)
ARTIFACT_PATH = (
    "artifacts/runtime/m4/"
    "harden_task_test_debug_and_terminal_event_normalization_truth_packet.json"
)
SCHEMA_REF = (
    "schemas/runtime/"
    "harden_task_test_debug_and_terminal_event_normalization_truth.schema.json"
)

LANES = [
    ("task_lane", "task"),
    ("test_lane", "test"),
    ("debug_lane", "debug"),
    ("terminal_lane", "terminal"),
]

WEDGES = [
    "envelope_canonicalization",
    "source_kind_negotiation",
    "lifecycle_normalization",
    "export_preservation",
]

ENVELOPE_FIELDS = [
    "event_id",
    "workspace_id",
    "target_id",
    "source_kind",
    "confidence",
    "timestamp",
    "execution_context_id",
    "payload_kind",
    "raw_payload_ref",
    "provenance",
]

SOURCE_KINDS = [
    "native",
    "bsp",
    "bazel_bep",
    "structured_output",
    "heuristic_parser",
]

LIFECYCLE_EVENTS = [
    "task_queued",
    "target_graph_ready",
    "task_started",
    "progress_updated",
    "diagnostic_emitted",
    "test_case_started",
    "test_case_finished",
    "artifact_published",
    "task_finished",
]

CONSUMER_SURFACE_BINDINGS = [
    "editor_run_surface",
    "task_panel",
    "test_runner_surface",
    "debug_surface",
    "terminal_pane",
    "cli_headless",
    "ai_tool_surface",
    "review_surface",
    "support_export",
]

CONSUMER_SURFACES = [
    "editor_run_surface",
    "task_panel",
    "test_runner_surface",
    "debug_surface",
    "terminal_pane",
    "cli_headless",
    "ai_tool_surface",
    "review_surface",
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
        "source_kind_class": "not_applicable",
        "lifecycle_event_class": "not_applicable",
        "consumer_surface_class": "not_applicable",
        "evidence_class": "fixture_repo_evidence",
        "known_limit_class": "none_declared",
        "downgrade_automation_class": "auto_narrow_on_lineage_break",
        "confidence_class": "high_confidence",
        "evidence_refs": [FIXTURE_DIR],
        "disclosure_ref": f"{DOC_REF}#auto_narrow_on_lineage_break",
        "attests_raw_payload_retained": False,
        "raw_source_material_excluded": True,
        "secrets_excluded": True,
        "ambient_authority_excluded": True,
        "captured_at": TIMESTAMP,
    }


def quality_row(lane, prefix):
    row = base_row(f"row:{prefix}:quality", lane, "event_normalization_quality")
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


def source_kind_rows(lane, prefix):
    rows = []
    for kind in SOURCE_KINDS:
        row = base_row(
            f"row:{prefix}:source_kind:{kind}", lane, "source_kind_binding"
        )
        row["source_kind_class"] = kind
        row["evidence_class"] = "conformance_suite_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_source_kind_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_source_kind_gap"
        rows.append(row)
    return rows


def lifecycle_rows(lane, prefix):
    rows = []
    for event in LIFECYCLE_EVENTS:
        row = base_row(
            f"row:{prefix}:lifecycle:{event}", lane, "lifecycle_event_binding"
        )
        row["lifecycle_event_class"] = event
        row["evidence_class"] = "automated_functional_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_lifecycle_event_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_lifecycle_event_gap"
        rows.append(row)
    return rows


def consumer_rows(lane, prefix):
    rows = []
    for surface in CONSUMER_SURFACE_BINDINGS:
        row = base_row(
            f"row:{prefix}:consumer:{surface}", lane, "consumer_surface_binding"
        )
        row["consumer_surface_class"] = surface
        row["evidence_class"] = "conformance_suite_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_consumer_surface_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_consumer_surface_gap"
        rows.append(row)
    return rows


def retention_row(lane, prefix):
    row = base_row(
        f"row:{prefix}:retention", lane, "raw_payload_retention_attestation"
    )
    row["evidence_class"] = "failure_recovery_drill_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_export_flattening"
    row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_export_flattening"
    row["attests_raw_payload_retained"] = True
    return row


def lineage_row(lane, prefix):
    row = base_row(f"row:{prefix}:lineage_admission", lane, "lineage_admission")
    row["evidence_class"] = "automated_functional_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_lineage_break"
    row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_lineage_break"
    row["execution_context_id_binding"] = f"exec:m4:{prefix}:event_normalization"
    return row


def lane_rows(lane, prefix):
    rows = [quality_row(lane, prefix)]
    rows.extend(wedge_rows(lane, prefix))
    rows.extend(envelope_field_rows(lane, prefix))
    rows.extend(source_kind_rows(lane, prefix))
    rows.extend(lifecycle_rows(lane, prefix))
    rows.extend(consumer_rows(lane, prefix))
    rows.append(retention_row(lane, prefix))
    rows.append(lineage_row(lane, prefix))
    return rows


def projection(surface, packet_id):
    return {
        "consumer_surface": surface,
        "projection_ref": f"projection:{surface}",
        "event_normalization_truth_packet_id_ref": packet_id,
        "rendered_at": "2026-05-26T12:00:01Z",
        "preserves_same_packet": True,
        "preserves_lane_vocabulary": True,
        "preserves_row_class_vocabulary": True,
        "preserves_support_class_vocabulary": True,
        "preserves_wedge_vocabulary": True,
        "preserves_envelope_field_vocabulary": True,
        "preserves_source_kind_vocabulary": True,
        "preserves_lifecycle_event_vocabulary": True,
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
        "packet:m4:harden_task_test_debug_and_terminal_event_normalization:stable"
    )
    workflow_id = (
        "workflow.runtime.harden_task_test_debug_and_terminal_event_normalization.stable"
    )
    base = baseline_input(packet_id, workflow_id)
    base.update(
        {
            "record_kind": (
                "harden_task_test_debug_and_terminal_event_normalization_truth_stable_packet"
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
    source_kind_tokens = sorted({row["source_kind_class"] for row in rows})
    lifecycle_event_tokens = sorted({row["lifecycle_event_class"] for row in rows})
    consumer_surface_binding_tokens = sorted(
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
        "envelope_field_tokens": envelope_field_tokens,
        "source_kind_tokens": source_kind_tokens,
        "lifecycle_event_tokens": lifecycle_event_tokens,
        "consumer_surface_binding_tokens": consumer_surface_binding_tokens,
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
        "packet:m4:harden_task_test_debug_and_terminal_event_normalization:baseline_stable"
    )
    wid = (
        "workflow.runtime.harden_task_test_debug_and_terminal_event_normalization"
        ".baseline_stable"
    )
    inp = baseline_input(pid, wid)
    return {
        "record_kind": (
            "harden_task_test_debug_and_terminal_event_normalization_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "baseline_stable",
        "scenario": (
            "Baseline stable posture: all four event-normalization lanes "
            "(task, test, debug, terminal) publish an "
            "event_normalization_quality row at launch_stable plus the full "
            "four-wedge admission coverage (envelope_canonicalization, "
            "source_kind_negotiation, lifecycle_normalization, "
            "export_preservation), the full ten envelope-field bindings "
            "(event_id, workspace_id, target_id, source_kind, confidence, "
            "timestamp, execution_context_id, payload_kind, raw_payload_ref, "
            "provenance), the full five source-kind bindings (native, bsp, "
            "bazel_bep, structured_output, heuristic_parser), the full nine "
            "lifecycle-event bindings (task_queued, target_graph_ready, "
            "task_started, progress_updated, diagnostic_emitted, "
            "test_case_started, test_case_finished, artifact_published, "
            "task_finished), the full nine consumer-surface bindings "
            "(editor_run_surface, task_panel, test_runner_surface, "
            "debug_surface, terminal_pane, cli_headless, ai_tool_surface, "
            "review_surface, support_export), a raw_payload_retention_attestation "
            "row attesting preserved source_kind, confidence, and raw payload "
            "retention, and a lineage_admission row binding "
            "execution_context_id; every row binds support, known limit, "
            "downgrade automation, and evidence classes; narrowed rows carry "
            "their disclosure refs; and all twelve required consumer projections "
            "preserve the packet verbatim."
        ),
        "input": inp,
        "expect": expectations_from_input(inp, "stable", 0),
    }


def unbound_evidence_case():
    pid = (
        "packet:m4:harden_task_test_debug_and_terminal_event_normalization:"
        "launch_stable_with_unbound_evidence"
    )
    wid = (
        "workflow.runtime.harden_task_test_debug_and_terminal_event_normalization."
        "launch_stable_with_unbound_evidence"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:task:quality":
            row["evidence_class"] = "evidence_unbound"
            break
    return {
        "record_kind": (
            "harden_task_test_debug_and_terminal_event_normalization_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "launch_stable_with_unbound_evidence_blocks_stable",
        "scenario": (
            "The task lane's event_normalization_quality row claims "
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


def missing_source_kind_case():
    pid = (
        "packet:m4:harden_task_test_debug_and_terminal_event_normalization:"
        "missing_source_kind_for_launch_stable"
    )
    wid = (
        "workflow.runtime.harden_task_test_debug_and_terminal_event_normalization."
        "missing_source_kind_for_launch_stable"
    )
    inp = baseline_input(pid, wid)
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "source_kind_binding"
            and row["source_kind_class"] == "bazel_bep"
            and row["lane_class"] == "task_lane"
        )
    ]
    return {
        "record_kind": (
            "harden_task_test_debug_and_terminal_event_normalization_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "missing_source_kind_for_launch_stable_blocks_stable",
        "scenario": (
            "The task lane claims launch_stable but drops its bazel_bep "
            "source_kind_binding row; the packet blocks the stable claim "
            "because every launch_stable lane must bind all five canonical "
            "source kinds (native, bsp, bazel_bep, structured_output, "
            "heuristic_parser) so adapter capability negotiation and raw "
            "payload retention stay adapter-isolated rather than flattened "
            "into one undifferentiated event stream."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            1,
            expected_findings=["missing_source_kind_coverage"],
        ),
    }


def missing_lifecycle_event_case():
    pid = (
        "packet:m4:harden_task_test_debug_and_terminal_event_normalization:"
        "missing_lifecycle_event_for_launch_stable"
    )
    wid = (
        "workflow.runtime.harden_task_test_debug_and_terminal_event_normalization."
        "missing_lifecycle_event_for_launch_stable"
    )
    inp = baseline_input(pid, wid)
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "lifecycle_event_binding"
            and row["lifecycle_event_class"] == "test_case_finished"
            and row["lane_class"] == "test_lane"
        )
    ]
    return {
        "record_kind": (
            "harden_task_test_debug_and_terminal_event_normalization_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "missing_lifecycle_event_for_launch_stable_blocks_stable",
        "scenario": (
            "The test lane claims launch_stable but drops its "
            "test_case_finished lifecycle_event_binding row; the packet "
            "blocks the stable claim because every launch_stable lane must "
            "bind all nine canonical lifecycle events (task_queued, "
            "target_graph_ready, task_started, progress_updated, "
            "diagnostic_emitted, test_case_started, test_case_finished, "
            "artifact_published, task_finished)."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            1,
            expected_findings=["missing_lifecycle_event_coverage"],
        ),
    }


def retention_flattening_case():
    pid = (
        "packet:m4:harden_task_test_debug_and_terminal_event_normalization:"
        "retention_admits_flattening"
    )
    wid = (
        "workflow.runtime.harden_task_test_debug_and_terminal_event_normalization."
        "retention_admits_flattening"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:task:retention":
            row["attests_raw_payload_retained"] = False
            break
    return {
        "record_kind": (
            "harden_task_test_debug_and_terminal_event_normalization_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "retention_admits_flattening_blocks_stable",
        "scenario": (
            "The task lane's raw_payload_retention_attestation row stops "
            "attesting that source_kind, confidence, and the adapter raw "
            "payload reference are preserved through replay, export, and "
            "support packets; the packet blocks the stable claim because "
            "imported, heuristic, and native event streams must not be "
            "flattened into one undifferentiated execution ledger."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            2,
            expected_findings=[
                "raw_payload_retention_attestation_admits_flattening",
                "missing_raw_payload_retention_attestation",
            ],
        ),
    }


def narrowed_row_missing_disclosure_ref_case():
    pid = (
        "packet:m4:harden_task_test_debug_and_terminal_event_normalization:"
        "narrowed_row_missing_disclosure_ref"
    )
    wid = (
        "workflow.runtime.harden_task_test_debug_and_terminal_event_normalization."
        "narrowed_row_missing_disclosure_ref"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:task:quality":
            row["support_class"] = "launch_stable_below"
            row.pop("disclosure_ref", None)
            break
    return {
        "record_kind": (
            "harden_task_test_debug_and_terminal_event_normalization_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "narrowed_row_missing_disclosure_ref_blocks_stable",
        "scenario": (
            "The task lane's event_normalization_quality row narrows to "
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


def projection_collapses_source_kind_vocabulary_case():
    pid = (
        "packet:m4:harden_task_test_debug_and_terminal_event_normalization:"
        "projection_collapses_source_kind_vocabulary"
    )
    wid = (
        "workflow.runtime.harden_task_test_debug_and_terminal_event_normalization."
        "projection_collapses_source_kind_vocabulary"
    )
    inp = baseline_input(pid, wid)
    for proj in inp["consumer_projections"]:
        if proj["consumer_surface"] == "help_about":
            proj["preserves_source_kind_vocabulary"] = False
            break
    return {
        "record_kind": (
            "harden_task_test_debug_and_terminal_event_normalization_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "projection_collapses_source_kind_vocabulary_blocks_stable",
        "scenario": (
            "The help_about consumer projection drops the source-kind "
            "vocabulary; the packet blocks the stable claim because a "
            "downstream surface that collapses the five canonical source "
            "kinds (native, bsp, bazel_bep, structured_output, "
            "heuristic_parser) cannot preserve event-normalization truth "
            "verbatim."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            3,
            expected_findings=[
                "source_kind_vocabulary_collapsed",
                "missing_consumer_projection",
                "consumer_projection_drift",
            ],
        ),
    }


def raw_source_material_case():
    pid = (
        "packet:m4:harden_task_test_debug_and_terminal_event_normalization:"
        "raw_source_material"
    )
    wid = (
        "workflow.runtime.harden_task_test_debug_and_terminal_event_normalization."
        "raw_source_material"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:task:quality":
            row["raw_source_material_excluded"] = False
            break
    return {
        "record_kind": (
            "harden_task_test_debug_and_terminal_event_normalization_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "raw_source_material_blocks_stable",
        "scenario": (
            "The task lane's event_normalization_quality row admits raw "
            "command lines, raw process environment bytes, raw scrollback "
            "bodies, or raw capsule bodies past the boundary; the packet "
            "blocks the stable claim because raw runtime material, secrets, "
            "and ambient credentials must never leak through the "
            "event-normalization boundary."
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
        missing_source_kind_case(),
        missing_lifecycle_event_case(),
        retention_flattening_case(),
        narrowed_row_missing_disclosure_ref_case(),
        projection_collapses_source_kind_vocabulary_case(),
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
            "# harden_task_test_debug_and_terminal_event_normalization fixture corpus\n\n"
            "Fixture corpus for the M4 stable event-normalization truth packet "
            "(`schemas/runtime/harden_task_test_debug_and_terminal_event_normalization_truth.schema.json`).\n\n"
            "Each fixture is an `EventNormalizationTruthPacketInput` with an "
            "`expect` block that pins the materialized packet's promotion "
            "state, finding count, lane and row-class token sets, "
            "support-class, wedge, envelope-field, source-kind, "
            "lifecycle-event, consumer-surface-binding, known-limit, "
            "downgrade-automation, and evidence-class tokens, and the "
            "support-export safety verdict. Tests in "
            "`crates/aureline-terminal/tests/harden_task_test_debug_and_terminal_event_normalization_truth_packet.rs` "
            "load each case and assert that `EventNormalizationTruthPacket::materialize` "
            "agrees.\n\n"
            "Cases:\n\n"
            "- `baseline_stable.json` — All four event-normalization lanes "
            "(task, test, debug, terminal) carry one `event_normalization_quality` "
            "row at `launch_stable` plus the full four-wedge admission "
            "coverage (envelope_canonicalization, source_kind_negotiation, "
            "lifecycle_normalization, export_preservation), the full ten "
            "envelope-field bindings, the full five source-kind bindings, "
            "the full nine lifecycle-event bindings, the full nine "
            "consumer-surface bindings, a raw_payload_retention_attestation "
            "row, and a lineage_admission row binding "
            "`execution_context_id`. All twelve required consumer projections "
            "preserve the packet verbatim.\n"
            "- `launch_stable_with_unbound_evidence_blocks_stable.json` — "
            "The task lane's quality row claims `launch_stable` while its "
            "evidence is `evidence_unbound`; the packet blocks the stable "
            "claim.\n"
            "- `missing_source_kind_for_launch_stable_blocks_stable.json` — "
            "The task lane claims `launch_stable` but the `bazel_bep` "
            "source-kind binding is missing; the packet blocks the stable "
            "claim.\n"
            "- `missing_lifecycle_event_for_launch_stable_blocks_stable.json` "
            "— The test lane claims `launch_stable` but the "
            "`test_case_finished` lifecycle binding is missing; the packet "
            "blocks the stable claim.\n"
            "- `retention_admits_flattening_blocks_stable.json` — The task "
            "lane's raw-payload retention attestation stops attesting that "
            "source_kind, confidence, and raw payload retention are "
            "preserved; the packet blocks the stable claim because "
            "imported, heuristic, and native event streams must not be "
            "flattened into one undifferentiated ledger.\n"
            "- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — "
            "The task lane's quality row narrows to `launch_stable_below` "
            "but drops its disclosure ref; the packet blocks the stable "
            "claim.\n"
            "- `projection_collapses_source_kind_vocabulary_blocks_stable.json` "
            "— The `help_about` consumer projection drops the source-kind "
            "vocabulary; the packet blocks the stable claim.\n"
            "- `raw_source_material_blocks_stable.json` — The task lane's "
            "quality row admits raw command lines, env bytes, scrollback "
            "bodies, or capsule bodies past the boundary; the packet "
            "blocks the stable claim because raw runtime material must "
            "never leak through the event-normalization boundary.\n"
        )


if __name__ == "__main__":
    main()
