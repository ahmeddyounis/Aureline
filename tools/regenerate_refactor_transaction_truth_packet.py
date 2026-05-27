#!/usr/bin/env python3
"""Regenerate the M4 refactor transaction truth packet artifact and fixture corpus.

Mirrors the Rust unit-test sample input in
crates/aureline-language/src/refactor_transaction_truth_packet/mod.rs. The
generator is the canonical seed for the checked-in artifact and the
narrowed-below-stable fixture cases used by the integration tests in
crates/aureline-language/tests/refactor_transaction_truth_packet.rs.

Run from anywhere:
    python3 tools/regenerate_refactor_transaction_truth_packet.py
"""
import json
import os

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

DOC_REF = "docs/languages/m4/finalize-the-refactor-transaction-model-plus-preview-validate.md"
FIXTURE_DIR = "fixtures/language/m4/refactor_transaction_truth_packet"
ARTIFACT_PATH = "artifacts/language/m4/refactor_transaction_truth_packet.json"
SCHEMA_REF = "schemas/language/refactor_transaction_truth.schema.json"

LANES = [
    ("rename_symbol_lane", "rename"),
    ("extract_function_lane", "extract"),
    ("inline_symbol_lane", "inline"),
    ("move_symbol_lane", "move"),
    ("update_imports_lane", "imports"),
    ("cross_file_signature_change_lane", "signature"),
]

PHASES = ["preview", "validate", "apply", "rollback"]

LAUNCH_LANGUAGES_PER_LANE = {
    "rename_symbol_lane": "python",
    "extract_function_lane": "typescript_javascript",
    "inline_symbol_lane": "rust",
    "move_symbol_lane": "go",
    "update_imports_lane": "java_kotlin",
    "cross_file_signature_change_lane": "c_cpp",
}

CONSUMER_SURFACES = [
    "editor_language_pack",
    "framework_pack_panel",
    "language_settings",
    "cli_headless",
    "support_export",
    "release_proof_index",
    "help_about",
    "conformance_dashboard",
]

TIMESTAMP = "2026-05-26T12:00:00Z"
RENDERED_AT_BASE = "2026-05-26T12:00:0{}Z"


def base_row(row_id, lane, row_class):
    return {
        "row_id": row_id,
        "lane_class": lane,
        "row_class": row_class,
        "support_class": "launch_stable",
        "transaction_phase_class": "not_applicable",
        "preview_completeness_class": "not_applicable",
        "validation_outcome_class": "not_applicable",
        "rollback_path_class": "not_applicable",
        "launch_language_class": "not_applicable",
        "evidence_class": "fixture_repo_evidence",
        "known_limit_class": "none_declared",
        "downgrade_automation_class": "auto_narrow_on_missing_fixture",
        "confidence_class": "high_confidence",
        "evidence_refs": [FIXTURE_DIR],
        "disclosure_ref": f"{DOC_REF}#auto_narrow_on_missing_fixture",
        "raw_source_material_excluded": True,
        "secrets_excluded": True,
        "ambient_authority_excluded": True,
        "captured_at": TIMESTAMP,
    }


def quality_row(lane, prefix):
    row = base_row(f"row:{prefix}:quality", lane, "refactor_transaction_quality")
    row["evidence_class"] = "archetype_repo_evidence"
    row["downgrade_automation_class"] = "auto_block_on_missing_evidence"
    row["disclosure_ref"] = f"{DOC_REF}#auto_block_on_missing_evidence"
    row["evidence_refs"] = [DOC_REF, FIXTURE_DIR]
    return row


def phase_rows(lane, prefix):
    rows = []
    for phase in PHASES:
        row = base_row(f"row:{prefix}:phase:{phase}", lane, "transaction_phase_truth")
        row["transaction_phase_class"] = phase
        row["evidence_class"] = "conformance_suite_evidence"
        rows.append(row)
    return rows


def preview_row(lane, prefix):
    row = base_row(f"row:{prefix}:preview_outcome", lane, "preview_outcome_admission")
    row["preview_completeness_class"] = "complete"
    row["evidence_class"] = "fixture_repo_evidence"
    return row


def validation_row(lane, prefix):
    row = base_row(f"row:{prefix}:validation_hook", lane, "validation_hook_admission")
    row["validation_outcome_class"] = "passed"
    row["evidence_class"] = "fixture_repo_evidence"
    return row


def rollback_row(lane, prefix):
    row = base_row(f"row:{prefix}:rollback_drill", lane, "rollback_drill_admission")
    row["rollback_path_class"] = "exact_undo_via_local_history_checkpoint"
    row["evidence_class"] = "fixture_repo_evidence"
    return row


def launch_language_row(lane, prefix, language):
    row = base_row(
        f"row:{prefix}:launch_language:{language}",
        lane,
        "launch_language_coverage",
    )
    row["launch_language_class"] = language
    row["evidence_class"] = "archetype_repo_evidence"
    return row


def lane_rows(lane, prefix):
    rows = [quality_row(lane, prefix)]
    rows.extend(phase_rows(lane, prefix))
    rows.append(preview_row(lane, prefix))
    rows.append(validation_row(lane, prefix))
    rows.append(rollback_row(lane, prefix))
    rows.append(launch_language_row(lane, prefix, LAUNCH_LANGUAGES_PER_LANE[lane]))
    return rows


def projection(surface, packet_id, idx):
    return {
        "consumer_surface": surface,
        "projection_ref": f"projection:{surface}:stable",
        "refactor_transaction_packet_id_ref": packet_id,
        "rendered_at": RENDERED_AT_BASE.format(idx),
        "preserves_same_packet": True,
        "preserves_lane_vocabulary": True,
        "preserves_row_class_vocabulary": True,
        "preserves_support_class_vocabulary": True,
        "preserves_transaction_phase_vocabulary": True,
        "preserves_preview_completeness_vocabulary": True,
        "preserves_validation_outcome_vocabulary": True,
        "preserves_rollback_path_vocabulary": True,
        "preserves_launch_language_vocabulary": True,
        "preserves_known_limit_vocabulary": True,
        "preserves_downgrade_automation_vocabulary": True,
        "preserves_evidence_class_vocabulary": True,
        "supports_json_export": True,
        "raw_private_material_excluded": True,
        "ambient_authority_excluded": True,
    }


def build_input(packet_id, workflow_id):
    rows = []
    for lane, prefix in LANES:
        rows.extend(lane_rows(lane, prefix))
    projections = [
        projection(surface, packet_id, idx)
        for idx, surface in enumerate(CONSUMER_SURFACES)
    ]
    return {
        "packet_id": packet_id,
        "workflow_or_surface_id": workflow_id,
        "generated_at": TIMESTAMP,
        "covered_lanes": [lane for lane, _ in LANES],
        "rows": rows,
        "consumer_projections": projections,
        "source_contract_refs": [DOC_REF, SCHEMA_REF],
    }


def build_artifact_packet():
    pkt_id = "packet:m4:refactor_transaction:stable"
    workflow = "workflow.language.refactor_transaction.stable"
    inp = build_input(pkt_id, workflow)
    artifact = {
        "record_kind": "refactor_transaction_truth_stable_packet",
        "schema_version": 1,
        "packet_id": pkt_id,
        "workflow_or_surface_id": workflow,
        "generated_at": TIMESTAMP,
        "covered_lanes": inp["covered_lanes"],
        "rows": inp["rows"],
        "consumer_projections": inp["consumer_projections"],
        "source_contract_refs": inp["source_contract_refs"],
        "promotion_state": "stable",
        "validation_findings": [],
    }
    return artifact


def write_json(path, payload):
    abs_path = os.path.join(REPO, path)
    os.makedirs(os.path.dirname(abs_path), exist_ok=True)
    with open(abs_path, "w") as fh:
        json.dump(payload, fh, indent=2)
        fh.write("\n")
    print(f"wrote {path}")


def unique_tokens(rows, field):
    return sorted({row[field] for row in rows})


def expected_block(rows, **overrides):
    expect = {
        "promotion_state": "stable",
        "validation_finding_count": 0,
        "row_count": len(rows),
        "lane_tokens": unique_tokens(rows, "lane_class"),
        "row_class_tokens": unique_tokens(rows, "row_class"),
        "support_class_tokens": unique_tokens(rows, "support_class"),
        "transaction_phase_tokens": unique_tokens(rows, "transaction_phase_class"),
        "preview_completeness_tokens": unique_tokens(rows, "preview_completeness_class"),
        "validation_outcome_tokens": unique_tokens(rows, "validation_outcome_class"),
        "rollback_path_tokens": unique_tokens(rows, "rollback_path_class"),
        "launch_language_tokens": unique_tokens(rows, "launch_language_class"),
        "known_limit_tokens": unique_tokens(rows, "known_limit_class"),
        "downgrade_automation_tokens": unique_tokens(rows, "downgrade_automation_class"),
        "evidence_class_tokens": unique_tokens(rows, "evidence_class"),
        "support_export_safe": True,
    }
    expect.update(overrides)
    return expect


def build_baseline_fixture():
    pkt_id = "packet:m4:refactor_transaction:baseline_stable"
    workflow = "workflow.language.refactor_transaction.baseline_stable"
    inp = build_input(pkt_id, workflow)
    return {
        "record_kind": "refactor_transaction_truth_stable_case",
        "schema_version": 1,
        "case_name": "baseline_stable",
        "scenario": (
            "Baseline stable posture: every refactor-class lane carries a "
            "refactor_transaction_quality row at launch_stable plus all four "
            "transaction_phase_truth rows for preview, validate, apply, and "
            "rollback; each lane also surfaces a preview_outcome_admission row "
            "with bound preview_completeness_class, a validation_hook_admission "
            "row with bound validation_outcome_class, a rollback_drill_admission "
            "row with bound rollback_path_class, and a launch_language_coverage "
            "row binding the launch language under proof; every row binds "
            "support, evidence, known-limit, downgrade-automation, "
            "preview-completeness, validation-outcome, and rollback-path "
            "classes; narrowed rows carry their disclosure refs; and all eight "
            "required consumer projections preserve the packet verbatim."
        ),
        "input": inp,
        "expect": expected_block(inp["rows"]),
    }


def with_modifier(case_name, scenario, mutate, expected_overrides=None,
                  expected_findings=None):
    pkt_id = f"packet:m4:refactor_transaction:{case_name}"
    workflow = f"workflow.language.refactor_transaction.{case_name}"
    inp = build_input(pkt_id, workflow)
    mutate(inp)
    overrides = {
        "promotion_state": "blocks_stable",
        "support_export_safe": False,
    }
    if expected_overrides:
        overrides.update(expected_overrides)
    expect = expected_block(inp["rows"], **overrides)
    # validation_finding_count: we record at least the number of declared
    # expected_finding_kinds; the rust packet may emit more, so the test runner
    # asserts exact counts. We'll record the exact expected count below.
    expect["validation_finding_count"] = overrides.get("validation_finding_count", 1)
    if expected_findings:
        expect["expected_finding_kinds"] = expected_findings
    return {
        "record_kind": "refactor_transaction_truth_stable_case",
        "schema_version": 1,
        "case_name": case_name,
        "scenario": scenario,
        "input": inp,
        "expect": expect,
    }


def mutate_launch_stable_unbound_evidence(inp):
    # Drop evidence on the first quality row.
    inp["rows"][0]["evidence_class"] = "evidence_unbound"


def mutate_missing_phase(inp):
    # Drop the rollback transaction_phase_truth row for rename_symbol_lane.
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["lane_class"] == "rename_symbol_lane"
            and row["row_class"] == "transaction_phase_truth"
            and row["transaction_phase_class"] == "rollback"
        )
    ]


def mutate_narrowed_no_disclosure(inp):
    # Narrow the first quality row below launch_stable without a disclosure ref.
    inp["rows"][0]["support_class"] = "launch_stable_below"
    inp["rows"][0]["disclosure_ref"] = None


def mutate_projection_collapse(inp):
    for projection in inp["consumer_projections"]:
        if projection["consumer_surface"] == "help_about":
            projection["preserves_rollback_path_vocabulary"] = False


def mutate_raw_source(inp):
    inp["rows"][0]["raw_source_material_excluded"] = False


def main():
    # 1) Build and write the canonical artifact packet.
    artifact = build_artifact_packet()
    write_json(ARTIFACT_PATH, artifact)

    # 2) Build and write fixtures.
    fixtures = [
        build_baseline_fixture(),
        with_modifier(
            "launch_stable_with_unbound_evidence_blocks_stable",
            (
                "A row claiming launch_stable but binding evidence_unbound is "
                "refused: the validator emits missing_evidence_class plus "
                "launch_stable_with_unbound_binding and the packet blocks "
                "stable instead of inheriting the adjacent certified rows."
            ),
            mutate_launch_stable_unbound_evidence,
            expected_overrides={
                "validation_finding_count": 2,
                "evidence_class_tokens": ["archetype_repo_evidence", "conformance_suite_evidence",
                                           "evidence_unbound", "fixture_repo_evidence"],
            },
            expected_findings=[
                "missing_evidence_class",
                "launch_stable_with_unbound_binding",
            ],
        ),
        with_modifier(
            "missing_transaction_phase_for_launch_stable_blocks_stable",
            (
                "A lane claiming launch_stable but missing the rollback "
                "transaction_phase_truth row is refused: the validator emits "
                "missing_transaction_phase_coverage and the packet blocks "
                "stable until the phase row is restored."
            ),
            mutate_missing_phase,
            expected_overrides={
                "validation_finding_count": 1,
                "row_count": 53,
            },
            expected_findings=["missing_transaction_phase_coverage"],
        ),
        with_modifier(
            "narrowed_row_missing_disclosure_ref_blocks_stable",
            (
                "A row narrowed below launch_stable without a disclosure ref is "
                "refused: the validator emits narrowed_row_missing_disclosure_ref "
                "(and, because the row still binds a non-`none` downgrade "
                "automation, downgrade_automation_missing_disclosure_ref) and the "
                "packet blocks stable until the narrowing is disclosed."
            ),
            mutate_narrowed_no_disclosure,
            expected_overrides={
                "validation_finding_count": 2,
                "support_class_tokens": ["launch_stable", "launch_stable_below"],
            },
            expected_findings=[
                "narrowed_row_missing_disclosure_ref",
                "downgrade_automation_missing_disclosure_ref",
            ],
        ),
        with_modifier(
            "projection_collapses_rollback_path_vocabulary_blocks_stable",
            (
                "A consumer projection that collapses the rollback-path "
                "vocabulary is refused: the validator emits "
                "rollback_path_vocabulary_collapsed plus consumer_projection_drift "
                "and the packet blocks stable until the projection preserves "
                "the closed vocabulary."
            ),
            mutate_projection_collapse,
            expected_overrides={
                "validation_finding_count": 3,
            },
            expected_findings=[
                "rollback_path_vocabulary_collapsed",
                "consumer_projection_drift",
                "missing_consumer_projection",
            ],
        ),
        with_modifier(
            "raw_source_material_blocks_stable",
            (
                "A row that admits raw source bodies past the boundary is "
                "refused: the validator emits raw_source_material_present and "
                "the packet blocks stable until the row excludes raw source "
                "material from its evidence surface."
            ),
            mutate_raw_source,
            expected_overrides={
                "validation_finding_count": 1,
            },
            expected_findings=["raw_source_material_present"],
        ),
    ]

    for fx in fixtures:
        write_json(os.path.join(FIXTURE_DIR, fx["case_name"] + ".json"), fx)


if __name__ == "__main__":
    main()
