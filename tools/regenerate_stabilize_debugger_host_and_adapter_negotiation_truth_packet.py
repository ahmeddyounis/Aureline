#!/usr/bin/env python3
"""Regenerate the M4 stabilize-debugger-host / adapter-negotiation / attach-launch / crash-isolation truth packet artifact and fixture corpus.

Mirrors the Rust unit-test sample input in
crates/aureline-runtime/src/stabilize_debugger_host_and_adapter_negotiation/mod.rs.
The generator is the canonical seed for the checked-in artifact and the
narrowed-below-stable fixture cases used by the integration tests in
crates/aureline-runtime/tests/stabilize_debugger_host_and_adapter_negotiation_truth_packet.rs.

Run from anywhere:
    python3 tools/regenerate_stabilize_debugger_host_and_adapter_negotiation_truth_packet.py
"""
import json
import os

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

DOC_REF = "docs/runtime/m4/stabilize-debugger-host-adapter-negotiation-attach-launch.md"
FIXTURE_DIR = (
    "fixtures/runtime/m4/stabilize_debugger_host_and_adapter_negotiation"
)
ARTIFACT_PATH = (
    "artifacts/runtime/m4/"
    "stabilize_debugger_host_and_adapter_negotiation_truth_packet.json"
)
SCHEMA_REF = (
    "schemas/runtime/"
    "stabilize_debugger_host_and_adapter_negotiation_truth.schema.json"
)

LANES = [
    ("local_lane", "local"),
    ("remote_helper_lane", "remote"),
    ("container_lane", "container"),
    ("notebook_bridge_lane", "notebook"),
]

WEDGES = [
    "debugger_host",
    "adapter_negotiation",
    "attach_launch_flow",
    "crash_isolation",
]

ADAPTER_DESCRIPTOR_FIELDS = [
    "adapter_identity",
    "transport_class",
    "launch_attach_scope",
    "local_vs_remote_support_class",
    "chronology_replay_capability_class",
    "notebook_bridge_or_replay_only_limitation",
]

ATTACH_LAUNCH_PARITY_SURFACES = [
    "ui_surface",
    "cli_headless",
    "support_export",
    "docs_help",
]

CRASH_ISOLATION_ASSERTIONS = [
    "bounded_restart_budget",
    "session_quarantine_admission",
    "unrelated_language_host_unaffected",
    "unrelated_terminal_lane_unaffected",
    "unrelated_debug_session_unaffected",
]

CONSUMER_SURFACES = [
    "editor_debug_surface",
    "debug_session_panel",
    "breakpoint_surface",
    "watch_locals_surface",
    "crash_loop_quarantine_banner",
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
        "adapter_descriptor_field_class": "not_applicable",
        "attach_launch_parity_surface_class": "not_applicable",
        "attach_launch_posture_class": "not_applicable",
        "crash_isolation_assertion_class": "not_applicable",
        "evidence_class": "fixture_repo_evidence",
        "known_limit_class": "none_declared",
        "downgrade_automation_class": "auto_narrow_on_lineage_break",
        "confidence_class": "high_confidence",
        "evidence_refs": [FIXTURE_DIR],
        "disclosure_ref": f"{DOC_REF}#auto_narrow_on_lineage_break",
        "attests_crash_isolation_assertion": False,
        "raw_source_material_excluded": True,
        "secrets_excluded": True,
        "ambient_authority_excluded": True,
        "captured_at": TIMESTAMP,
    }


def quality_row(lane, prefix):
    row = base_row(f"row:{prefix}:quality", lane, "debugger_stabilization_quality")
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


def adapter_descriptor_rows(lane, prefix):
    rows = []
    for field in ADAPTER_DESCRIPTOR_FIELDS:
        row = base_row(
            f"row:{prefix}:descriptor:{field}", lane, "adapter_descriptor_field_binding"
        )
        row["adapter_descriptor_field_class"] = field
        row["evidence_class"] = "automated_functional_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_adapter_descriptor_field_gap"
        row["disclosure_ref"] = (
            f"{DOC_REF}#auto_narrow_on_adapter_descriptor_field_gap"
        )
        rows.append(row)
    return rows


def parity_surface_rows(lane, prefix):
    rows = []
    for surface in ATTACH_LAUNCH_PARITY_SURFACES:
        row = base_row(
            f"row:{prefix}:parity:{surface}", lane, "attach_launch_parity_surface_binding"
        )
        row["attach_launch_parity_surface_class"] = surface
        row["attach_launch_posture_class"] = "supported"
        row["evidence_class"] = "conformance_suite_evidence"
        row["downgrade_automation_class"] = (
            "auto_narrow_on_attach_launch_parity_surface_gap"
        )
        row["disclosure_ref"] = (
            f"{DOC_REF}#auto_narrow_on_attach_launch_parity_surface_gap"
        )
        rows.append(row)
    return rows


def crash_isolation_rows(lane, prefix):
    rows = []
    for assertion in CRASH_ISOLATION_ASSERTIONS:
        row = base_row(
            f"row:{prefix}:crash_isolation:{assertion}",
            lane,
            "crash_isolation_assertion_binding",
        )
        row["crash_isolation_assertion_class"] = assertion
        row["evidence_class"] = "failure_recovery_drill_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_crash_isolation_assertion_gap"
        row["disclosure_ref"] = (
            f"{DOC_REF}#auto_narrow_on_crash_isolation_assertion_gap"
        )
        row["attests_crash_isolation_assertion"] = True
        rows.append(row)
    return rows


def lineage_row(lane, prefix):
    row = base_row(f"row:{prefix}:lineage_admission", lane, "lineage_admission")
    row["evidence_class"] = "automated_functional_evidence"
    row["downgrade_automation_class"] = "auto_narrow_on_lineage_break"
    row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_lineage_break"
    row["execution_context_id_binding"] = f"exec:m4:{prefix}:debugger_lineage"
    return row


def lane_rows(lane, prefix):
    rows = [quality_row(lane, prefix)]
    rows.extend(wedge_rows(lane, prefix))
    rows.extend(adapter_descriptor_rows(lane, prefix))
    rows.extend(parity_surface_rows(lane, prefix))
    rows.extend(crash_isolation_rows(lane, prefix))
    rows.append(lineage_row(lane, prefix))
    return rows


def projection(surface, packet_id):
    return {
        "consumer_surface": surface,
        "projection_ref": f"projection:{surface}",
        "debugger_stabilization_truth_packet_id_ref": packet_id,
        "rendered_at": "2026-05-26T12:00:01Z",
        "preserves_same_packet": True,
        "preserves_lane_vocabulary": True,
        "preserves_row_class_vocabulary": True,
        "preserves_support_class_vocabulary": True,
        "preserves_wedge_vocabulary": True,
        "preserves_adapter_descriptor_field_vocabulary": True,
        "preserves_attach_launch_parity_surface_vocabulary": True,
        "preserves_attach_launch_posture_vocabulary": True,
        "preserves_crash_isolation_assertion_vocabulary": True,
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
    packet_id = "packet:m4:stabilize_debugger_host_and_adapter_negotiation:stable"
    workflow_id = (
        "workflow.runtime.stabilize_debugger_host_and_adapter_negotiation.stable"
    )
    base = baseline_input(packet_id, workflow_id)
    base.update(
        {
            "record_kind": (
                "stabilize_debugger_host_and_adapter_negotiation_truth_stable_packet"
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
    adapter_descriptor_field_tokens = sorted(
        {row["adapter_descriptor_field_class"] for row in rows}
    )
    attach_launch_parity_surface_tokens = sorted(
        {row["attach_launch_parity_surface_class"] for row in rows}
    )
    attach_launch_posture_tokens = sorted(
        {row["attach_launch_posture_class"] for row in rows}
    )
    crash_isolation_assertion_tokens = sorted(
        {row["crash_isolation_assertion_class"] for row in rows}
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
        "adapter_descriptor_field_tokens": adapter_descriptor_field_tokens,
        "attach_launch_parity_surface_tokens": attach_launch_parity_surface_tokens,
        "attach_launch_posture_tokens": attach_launch_posture_tokens,
        "crash_isolation_assertion_tokens": crash_isolation_assertion_tokens,
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
        "packet:m4:stabilize_debugger_host_and_adapter_negotiation:baseline_stable"
    )
    wid = (
        "workflow.runtime.stabilize_debugger_host_and_adapter_negotiation.baseline_stable"
    )
    inp = baseline_input(pid, wid)
    return {
        "record_kind": (
            "stabilize_debugger_host_and_adapter_negotiation_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "baseline_stable",
        "scenario": (
            "Baseline stable posture: all four debugger lanes (local, "
            "remote_helper, container, notebook_bridge) publish a "
            "debugger_stabilization_quality row at launch_stable plus the full "
            "four-wedge admission coverage (debugger_host, adapter_negotiation, "
            "attach_launch_flow, crash_isolation), the full six adapter-descriptor "
            "field bindings (adapter_identity, transport_class, "
            "launch_attach_scope, local_vs_remote_support_class, "
            "chronology_replay_capability_class, "
            "notebook_bridge_or_replay_only_limitation), the full four "
            "attach/launch parity-surface bindings (ui_surface, cli_headless, "
            "support_export, docs_help) all carrying the supported posture, the "
            "full five crash-isolation assertion bindings "
            "(bounded_restart_budget, session_quarantine_admission, "
            "unrelated_language_host_unaffected, "
            "unrelated_terminal_lane_unaffected, "
            "unrelated_debug_session_unaffected) all attesting their assertion, "
            "and a lineage_admission row binding execution_context_id; every "
            "row binds support, known limit, downgrade automation, and evidence "
            "classes; narrowed rows carry their disclosure refs; and all eleven "
            "required consumer projections preserve the packet verbatim."
        ),
        "input": inp,
        "expect": expectations_from_input(inp, "stable", 0),
    }


def unbound_evidence_case():
    pid = (
        "packet:m4:stabilize_debugger_host_and_adapter_negotiation:"
        "launch_stable_with_unbound_evidence"
    )
    wid = (
        "workflow.runtime.stabilize_debugger_host_and_adapter_negotiation."
        "launch_stable_with_unbound_evidence"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:local:quality":
            row["evidence_class"] = "evidence_unbound"
            break
    return {
        "record_kind": (
            "stabilize_debugger_host_and_adapter_negotiation_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "launch_stable_with_unbound_evidence_blocks_stable",
        "scenario": (
            "The local lane's debugger_stabilization_quality row claims "
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


def missing_adapter_descriptor_field_case():
    pid = (
        "packet:m4:stabilize_debugger_host_and_adapter_negotiation:"
        "missing_adapter_descriptor_field_for_launch_stable"
    )
    wid = (
        "workflow.runtime.stabilize_debugger_host_and_adapter_negotiation."
        "missing_adapter_descriptor_field_for_launch_stable"
    )
    inp = baseline_input(pid, wid)
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "adapter_descriptor_field_binding"
            and row["adapter_descriptor_field_class"]
            == "notebook_bridge_or_replay_only_limitation"
            and row["lane_class"] == "local_lane"
        )
    ]
    return {
        "record_kind": (
            "stabilize_debugger_host_and_adapter_negotiation_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "missing_adapter_descriptor_field_for_launch_stable_blocks_stable",
        "scenario": (
            "The local lane claims launch_stable but drops its "
            "notebook_bridge_or_replay_only_limitation "
            "adapter_descriptor_field_binding row; the packet blocks the "
            "stable claim because every launch_stable lane must serialize all "
            "six adapter/backend descriptor fields (adapter_identity, "
            "transport_class, launch_attach_scope, local_vs_remote_support_class, "
            "chronology_replay_capability_class, "
            "notebook_bridge_or_replay_only_limitation) so reviewers cannot "
            "infer support from button presence."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            1,
            expected_findings=["missing_adapter_descriptor_field_coverage"],
        ),
    }


def attach_launch_posture_drift_case():
    pid = (
        "packet:m4:stabilize_debugger_host_and_adapter_negotiation:"
        "attach_launch_posture_drift"
    )
    wid = (
        "workflow.runtime.stabilize_debugger_host_and_adapter_negotiation."
        "attach_launch_posture_drift"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if (
            row["row_class"] == "attach_launch_parity_surface_binding"
            and row["lane_class"] == "local_lane"
            and row["attach_launch_parity_surface_class"] == "cli_headless"
        ):
            row["attach_launch_posture_class"] = "limited"
            row["disclosure_ref"] = (
                f"{DOC_REF}#auto_narrow_on_attach_launch_posture_drift"
            )
            break
    return {
        "record_kind": (
            "stabilize_debugger_host_and_adapter_negotiation_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "attach_launch_posture_drift_blocks_stable",
        "scenario": (
            "The local lane's cli_headless parity-surface row reports the "
            "limited attach/launch posture while the other three parity "
            "surfaces (ui_surface, support_export, docs_help) report "
            "supported; the packet blocks the stable claim because the "
            "attach/launch posture label MUST propagate verbatim to UI, "
            "CLI/headless, support export, and docs/help without drift."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            1,
            expected_findings=["attach_launch_posture_drift"],
        ),
    }


def crash_isolation_not_attested_case():
    pid = (
        "packet:m4:stabilize_debugger_host_and_adapter_negotiation:"
        "crash_isolation_assertion_not_attested"
    )
    wid = (
        "workflow.runtime.stabilize_debugger_host_and_adapter_negotiation."
        "crash_isolation_assertion_not_attested"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if (
            row["row_class"] == "crash_isolation_assertion_binding"
            and row["lane_class"] == "local_lane"
            and row["crash_isolation_assertion_class"] == "bounded_restart_budget"
        ):
            row["attests_crash_isolation_assertion"] = False
            break
    return {
        "record_kind": (
            "stabilize_debugger_host_and_adapter_negotiation_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "crash_isolation_assertion_not_attested_blocks_stable",
        "scenario": (
            "The local lane's bounded_restart_budget "
            "crash_isolation_assertion_binding row stops attesting the "
            "assertion; the packet blocks the stable claim because every "
            "launch_stable lane MUST attest all five crash-isolation "
            "assertions (bounded_restart_budget, session_quarantine_admission, "
            "unrelated_language_host_unaffected, "
            "unrelated_terminal_lane_unaffected, "
            "unrelated_debug_session_unaffected) so adapter crashes, protocol "
            "violations, or hangs degrade only the affected session."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            2,
            expected_findings=[
                "crash_isolation_assertion_not_attested",
                "missing_crash_isolation_assertion_coverage",
            ],
        ),
    }


def narrowed_row_missing_disclosure_ref_case():
    pid = (
        "packet:m4:stabilize_debugger_host_and_adapter_negotiation:"
        "narrowed_row_missing_disclosure_ref"
    )
    wid = (
        "workflow.runtime.stabilize_debugger_host_and_adapter_negotiation."
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
            "stabilize_debugger_host_and_adapter_negotiation_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "narrowed_row_missing_disclosure_ref_blocks_stable",
        "scenario": (
            "The local lane's debugger_stabilization_quality row narrows to "
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


def projection_collapses_attach_launch_posture_vocabulary_case():
    pid = (
        "packet:m4:stabilize_debugger_host_and_adapter_negotiation:"
        "projection_collapses_attach_launch_posture_vocabulary"
    )
    wid = (
        "workflow.runtime.stabilize_debugger_host_and_adapter_negotiation."
        "projection_collapses_attach_launch_posture_vocabulary"
    )
    inp = baseline_input(pid, wid)
    for proj in inp["consumer_projections"]:
        if proj["consumer_surface"] == "help_about":
            proj["preserves_attach_launch_posture_vocabulary"] = False
            break
    return {
        "record_kind": (
            "stabilize_debugger_host_and_adapter_negotiation_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "projection_collapses_attach_launch_posture_vocabulary_blocks_stable",
        "scenario": (
            "The help_about consumer projection drops the attach/launch "
            "posture vocabulary; the packet blocks the stable claim because "
            "a downstream surface that collapses the five attach/launch "
            "posture labels (supported, limited, view_only, unsupported, "
            "policy_blocked) cannot preserve attach/launch truth verbatim."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            3,
            expected_findings=[
                "attach_launch_posture_vocabulary_collapsed",
                "missing_consumer_projection",
                "consumer_projection_drift",
            ],
        ),
    }


def raw_source_material_case():
    pid = (
        "packet:m4:stabilize_debugger_host_and_adapter_negotiation:"
        "raw_source_material"
    )
    wid = (
        "workflow.runtime.stabilize_debugger_host_and_adapter_negotiation."
        "raw_source_material"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:local:quality":
            row["raw_source_material_excluded"] = False
            break
    return {
        "record_kind": (
            "stabilize_debugger_host_and_adapter_negotiation_truth_stable_case"
        ),
        "schema_version": 1,
        "case_name": "raw_source_material_blocks_stable",
        "scenario": (
            "The local lane's debugger_stabilization_quality row admits raw "
            "debugger payloads, raw stack frames, raw memory bytes, raw "
            "command lines, raw process environment bytes, or raw scrollback "
            "bodies past the boundary; the packet blocks the stable claim "
            "because raw runtime material, secrets, and ambient credentials "
            "must never leak through the debugger boundary."
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
        missing_adapter_descriptor_field_case(),
        attach_launch_posture_drift_case(),
        crash_isolation_not_attested_case(),
        narrowed_row_missing_disclosure_ref_case(),
        projection_collapses_attach_launch_posture_vocabulary_case(),
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
            "# stabilize_debugger_host_and_adapter_negotiation fixture corpus\n\n"
            "Fixture corpus for the M4 stable debugger host / adapter "
            "negotiation / attach-launch / crash-isolation truth packet "
            "(`schemas/runtime/"
            "stabilize_debugger_host_and_adapter_negotiation_truth.schema.json`).\n\n"
            "Each fixture is a `DebuggerStabilizationTruthPacketInput` with an "
            "`expect` block that pins the materialized packet's promotion "
            "state, finding count, lane and row-class token sets, "
            "support-class, wedge, adapter-descriptor-field, attach/launch "
            "parity-surface, attach/launch posture, crash-isolation-assertion, "
            "known-limit, downgrade-automation, and evidence-class tokens, and "
            "the support-export safety verdict. Tests in "
            "`crates/aureline-runtime/tests/"
            "stabilize_debugger_host_and_adapter_negotiation_truth_packet.rs` "
            "load each case and assert that "
            "`DebuggerStabilizationTruthPacket::materialize` agrees.\n\n"
            "Cases:\n\n"
            "- `baseline_stable.json` — All four debugger lanes (local, "
            "remote_helper, container, notebook_bridge) carry one "
            "`debugger_stabilization_quality` row at `launch_stable` plus the "
            "full four-wedge admission coverage (debugger_host, "
            "adapter_negotiation, attach_launch_flow, crash_isolation), the "
            "full six adapter-descriptor-field bindings (adapter_identity, "
            "transport_class, launch_attach_scope, "
            "local_vs_remote_support_class, "
            "chronology_replay_capability_class, "
            "notebook_bridge_or_replay_only_limitation), the full four "
            "attach/launch parity-surface bindings (ui_surface, cli_headless, "
            "support_export, docs_help) all carrying the supported posture, "
            "the full five crash-isolation assertion bindings "
            "(bounded_restart_budget, session_quarantine_admission, "
            "unrelated_language_host_unaffected, "
            "unrelated_terminal_lane_unaffected, "
            "unrelated_debug_session_unaffected), and a lineage_admission "
            "row binding `execution_context_id`. All eleven required "
            "consumer projections preserve the packet verbatim.\n"
            "- `launch_stable_with_unbound_evidence_blocks_stable.json` — The "
            "local lane's quality row claims `launch_stable` while its "
            "evidence is `evidence_unbound`; the packet blocks the stable "
            "claim.\n"
            "- `missing_adapter_descriptor_field_for_launch_stable_blocks_stable.json` — "
            "The local lane claims `launch_stable` but the "
            "`notebook_bridge_or_replay_only_limitation` descriptor-field "
            "binding is missing; the packet blocks the stable claim.\n"
            "- `attach_launch_posture_drift_blocks_stable.json` — The local "
            "lane's `cli_headless` parity surface reports `limited` while "
            "other surfaces report `supported`; the packet blocks the "
            "stable claim because the attach/launch posture label MUST "
            "propagate verbatim across UI, CLI/headless, support export, "
            "and docs/help without drift.\n"
            "- `crash_isolation_assertion_not_attested_blocks_stable.json` — "
            "The local lane's `bounded_restart_budget` crash-isolation "
            "assertion row stops attesting the assertion; the packet blocks "
            "the stable claim because crash isolation MUST be attested.\n"
            "- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — "
            "The local lane's quality row narrows to `launch_stable_below` "
            "but drops its disclosure ref; the packet blocks the stable "
            "claim.\n"
            "- `projection_collapses_attach_launch_posture_vocabulary_blocks_stable.json` "
            "— The `help_about` consumer projection drops the attach/launch "
            "posture vocabulary; the packet blocks the stable claim.\n"
            "- `raw_source_material_blocks_stable.json` — The local lane's "
            "quality row admits raw debugger payloads, raw stack frames, "
            "raw memory bytes, raw command lines, env bytes, or scrollback "
            "bodies past the boundary; the packet blocks the stable claim "
            "because raw runtime material must never leak through the "
            "debugger boundary.\n"
        )


if __name__ == "__main__":
    main()
