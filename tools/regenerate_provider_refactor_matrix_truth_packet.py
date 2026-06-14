#!/usr/bin/env python3
"""Regenerate the M5 provider/refactor matrix truth packet artifact and fixtures.

Mirrors the Rust unit-test sample input in
crates/aureline-language/src/provider_refactor_matrix_truth_packet/tests.rs. The
generator is the canonical seed for the checked-in artifact and the
narrowed-below-stable fixture cases used by the integration tests in
crates/aureline-language/tests/provider_refactor_matrix_truth_packet.rs.

Run from anywhere:
    python3 tools/regenerate_provider_refactor_matrix_truth_packet.py
"""
import json
import os

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

DOC_REF = "docs/m5/freeze-the-language-provider-diagnostic-cluster-and-refactor-transaction-matrix.md"
FIXTURE_DIR = "fixtures/language/m5/provider_refactor_matrix_truth_packet"
ARTIFACT_PATH = "artifacts/language/m5/provider_refactor_matrix_truth_packet.json"
SCHEMA_REF = "schemas/language/provider_refactor_matrix_truth.schema.json"

TIMESTAMP = "2026-06-14T12:00:00Z"
RENDERED_AT_BASE = "2026-06-14T12:00:0{}Z"

# One posture per artifact-family lane. The matrix maps each family to the
# provider, capability, conflict, diagnostic source, provenance, semantic mode,
# refactor class, completeness, rollback, generated-artifact policy, and
# allowed downgrade label it may claim.
LANE_SPECS = [
    {
        "lane": "framework_pack_lane",
        "prefix": "framework",
        "provider": "framework_analyzer",
        "capability": "full_semantic_negotiated",
        "conflict": "arbitrated_winner_loser_preserved",
        "diagnostic": "framework_schema",
        "provenance": "live_semantic",
        "mode": "previewable_refactor",
        "refactor": "extract",
        "completeness": "complete",
        "rollback": "grouped_mutation_journal_revert",
        "generated": "not_generated",
        "downgrade_label": "full_to_partial_completeness",
    },
    {
        "lane": "notebook_cell_lane",
        "prefix": "notebook",
        "provider": "notebook_adapter",
        "capability": "partial_semantic_negotiated",
        "conflict": "single_provider_no_conflict",
        "diagnostic": "notebook_kernel",
        "provenance": "cached_semantic",
        "mode": "notebook_generated_bridge",
        "refactor": "notebook_generated_edit",
        "completeness": "partial",
        "rollback": "compensating_revert_via_workspace_diff",
        "generated": "regenerate_before_edit",
        "downgrade_label": "semantic_to_text_fallback",
    },
    {
        "lane": "generated_source_lane",
        "prefix": "generated",
        "provider": "generated_source_bridge",
        "capability": "text_fallback_negotiated",
        "conflict": "policy_override_recorded",
        "diagnostic": "generated_artifact_validation",
        "provenance": "imported_scan",
        "mode": "code_action_mutation",
        "refactor": "schema_codegen_rewrite",
        "completeness": "complete",
        "rollback": "regenerate_first_then_replay",
        "generated": "edit_with_regeneration_replay",
        "downgrade_label": "generated_edit_to_regenerate_first",
    },
    {
        "lane": "structured_artifact_lane",
        "prefix": "structured",
        "provider": "lsp_provider",
        "capability": "full_semantic_negotiated",
        "conflict": "single_provider_no_conflict",
        "diagnostic": "compiler_build",
        "provenance": "live_semantic",
        "mode": "semantic_rename",
        "refactor": "rename",
        "completeness": "complete",
        "rollback": "exact_undo_via_local_history_checkpoint",
        "generated": "not_generated",
        "downgrade_label": "previewable_to_compare_only",
    },
    {
        "lane": "code_understanding_graph_lane",
        "prefix": "graph",
        "provider": "semantic_graph_lane",
        "capability": "partial_semantic_negotiated",
        "conflict": "unresolved_disagreement_surfaced",
        "diagnostic": "lsp",
        "provenance": "partial_semantic",
        "mode": "compare_only",
        "refactor": "compare_only_no_mutation",
        "completeness": "unsupported",
        "rollback": "no_safe_rollback_available",
        "generated": "compare_only_generated",
        "downgrade_label": "provider_unavailable_text_only",
    },
]

CONSUMER_SURFACES = [
    "framework_pack_panel",
    "notebook_surface",
    "request_runner",
    "preview_surface",
    "docs_surface",
    "generated_artifact_surface",
    "support_export",
    "release_proof_index",
    "help_about",
    "conformance_dashboard",
]


def base_row(row_id, lane, row_class):
    return {
        "row_id": row_id,
        "lane_class": lane,
        "row_class": row_class,
        "support_class": "certified",
        "provider_family_class": "not_applicable",
        "capability_negotiation_class": "not_applicable",
        "conflict_class": "not_applicable",
        "diagnostic_source_class": "not_applicable",
        "result_provenance_class": "not_applicable",
        "semantic_layer_mode_class": "not_applicable",
        "refactor_transaction_class": "not_applicable",
        "completeness_class": "not_applicable",
        "generated_artifact_policy_class": "not_applicable",
        "downgrade_label_class": "not_applicable",
        "rollback_path_class": "not_applicable",
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


def lane_rows(spec):
    prefix = spec["prefix"]
    lane = spec["lane"]
    rows = []

    quality = base_row(f"row:{prefix}:quality", lane, "matrix_lane_quality")
    quality["provider_family_class"] = spec["provider"]
    quality["evidence_class"] = "archetype_repo_evidence"
    quality["downgrade_automation_class"] = "auto_block_on_missing_evidence"
    quality["disclosure_ref"] = f"{DOC_REF}#auto_block_on_missing_evidence"
    quality["evidence_refs"] = [DOC_REF, FIXTURE_DIR]
    rows.append(quality)

    capability = base_row(f"row:{prefix}:capability", lane, "capability_negotiation_admission")
    capability["capability_negotiation_class"] = spec["capability"]
    rows.append(capability)

    conflict = base_row(f"row:{prefix}:conflict", lane, "conflict_arbitration_admission")
    conflict["conflict_class"] = spec["conflict"]
    rows.append(conflict)

    diagnostic = base_row(f"row:{prefix}:diagnostic", lane, "diagnostic_source_admission")
    diagnostic["diagnostic_source_class"] = spec["diagnostic"]
    rows.append(diagnostic)

    provenance = base_row(f"row:{prefix}:provenance", lane, "result_provenance_admission")
    provenance["result_provenance_class"] = spec["provenance"]
    rows.append(provenance)

    semantic = base_row(f"row:{prefix}:semantic_mode", lane, "semantic_layer_mode_admission")
    semantic["provider_family_class"] = spec["provider"]
    semantic["semantic_layer_mode_class"] = spec["mode"]
    rows.append(semantic)

    refactor = base_row(f"row:{prefix}:refactor", lane, "refactor_transaction_admission")
    refactor["refactor_transaction_class"] = spec["refactor"]
    refactor["completeness_class"] = spec["completeness"]
    refactor["rollback_path_class"] = spec["rollback"]
    refactor["evidence_class"] = "conformance_suite_evidence"
    rows.append(refactor)

    generated = base_row(f"row:{prefix}:generated_policy", lane, "generated_artifact_policy_admission")
    generated["generated_artifact_policy_class"] = spec["generated"]
    rows.append(generated)

    downgrade = base_row(f"row:{prefix}:downgrade_label", lane, "downgrade_label_admission")
    downgrade["downgrade_label_class"] = spec["downgrade_label"]
    rows.append(downgrade)

    return rows


def projection(surface, packet_id, idx):
    return {
        "consumer_surface": surface,
        "projection_ref": f"projection:{surface}:stable",
        "matrix_packet_id_ref": packet_id,
        "rendered_at": RENDERED_AT_BASE.format(idx % 10),
        "preserves_same_packet": True,
        "preserves_lane_vocabulary": True,
        "preserves_row_class_vocabulary": True,
        "preserves_support_class_vocabulary": True,
        "preserves_provider_family_vocabulary": True,
        "preserves_capability_negotiation_vocabulary": True,
        "preserves_conflict_vocabulary": True,
        "preserves_diagnostic_source_vocabulary": True,
        "preserves_result_provenance_vocabulary": True,
        "preserves_semantic_layer_mode_vocabulary": True,
        "preserves_refactor_transaction_vocabulary": True,
        "preserves_completeness_vocabulary": True,
        "preserves_generated_artifact_policy_vocabulary": True,
        "preserves_downgrade_label_vocabulary": True,
        "preserves_rollback_path_vocabulary": True,
        "preserves_known_limit_vocabulary": True,
        "preserves_downgrade_automation_vocabulary": True,
        "preserves_evidence_class_vocabulary": True,
        "supports_json_export": True,
        "raw_private_material_excluded": True,
        "ambient_authority_excluded": True,
    }


def build_input(packet_id, workflow_id):
    rows = []
    for spec in LANE_SPECS:
        rows.extend(lane_rows(spec))
    projections = [
        projection(surface, packet_id, idx)
        for idx, surface in enumerate(CONSUMER_SURFACES)
    ]
    return {
        "packet_id": packet_id,
        "workflow_or_surface_id": workflow_id,
        "generated_at": TIMESTAMP,
        "covered_lanes": [spec["lane"] for spec in LANE_SPECS],
        "rows": rows,
        "consumer_projections": projections,
        "source_contract_refs": [DOC_REF, SCHEMA_REF],
    }


def build_artifact_packet():
    pkt_id = "packet:m5:provider_refactor_matrix:stable"
    workflow = "workflow.language.provider_refactor_matrix.stable"
    inp = build_input(pkt_id, workflow)
    return {
        "record_kind": "provider_refactor_matrix_truth_stable_packet",
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
        "provider_family_tokens": unique_tokens(rows, "provider_family_class"),
        "capability_negotiation_tokens": unique_tokens(rows, "capability_negotiation_class"),
        "conflict_tokens": unique_tokens(rows, "conflict_class"),
        "diagnostic_source_tokens": unique_tokens(rows, "diagnostic_source_class"),
        "result_provenance_tokens": unique_tokens(rows, "result_provenance_class"),
        "semantic_layer_mode_tokens": unique_tokens(rows, "semantic_layer_mode_class"),
        "refactor_transaction_tokens": unique_tokens(rows, "refactor_transaction_class"),
        "completeness_tokens": unique_tokens(rows, "completeness_class"),
        "generated_artifact_policy_tokens": unique_tokens(rows, "generated_artifact_policy_class"),
        "downgrade_label_tokens": unique_tokens(rows, "downgrade_label_class"),
        "rollback_path_tokens": unique_tokens(rows, "rollback_path_class"),
        "known_limit_tokens": unique_tokens(rows, "known_limit_class"),
        "downgrade_automation_tokens": unique_tokens(rows, "downgrade_automation_class"),
        "evidence_class_tokens": unique_tokens(rows, "evidence_class"),
        "support_export_safe": True,
    }
    expect.update(overrides)
    return expect


def build_baseline_fixture():
    pkt_id = "packet:m5:provider_refactor_matrix:baseline_stable"
    workflow = "workflow.language.provider_refactor_matrix.baseline_stable"
    inp = build_input(pkt_id, workflow)
    return {
        "record_kind": "provider_refactor_matrix_truth_stable_case",
        "schema_version": 1,
        "case_name": "baseline_stable",
        "scenario": (
            "Baseline stable posture: every artifact-family lane (framework "
            "pack, notebook cell, generated source, structured artifact, and "
            "code-understanding graph) carries a matrix_lane_quality row at "
            "certified that names its acting provider family, plus one "
            "admission row per matrix dimension: capability negotiation, "
            "conflict arbitration, diagnostic source, result provenance, "
            "semantic-layer mode, refactor transaction (binding refactor "
            "class, preview completeness, and rollback path together), "
            "generated-artifact policy, and allowed downgrade label. Every "
            "row binds support, known-limit, downgrade-automation, and "
            "evidence classes; narrowed rows carry their disclosure refs; "
            "mutating refactors bind a typed completeness and a safe rollback "
            "path; and all ten required consumer projections preserve the "
            "packet verbatim."
        ),
        "input": inp,
        "expect": expected_block(inp["rows"]),
    }


def with_modifier(case_name, scenario, mutate, expected_overrides=None,
                  expected_findings=None):
    pkt_id = f"packet:m5:provider_refactor_matrix:{case_name}"
    workflow = f"workflow.language.provider_refactor_matrix.{case_name}"
    inp = build_input(pkt_id, workflow)
    mutate(inp)
    overrides = {
        "promotion_state": "blocks_stable",
        "support_export_safe": False,
    }
    if expected_overrides:
        overrides.update(expected_overrides)
    expect = expected_block(inp["rows"], **overrides)
    if expected_findings:
        expect["expected_finding_kinds"] = expected_findings
    return {
        "record_kind": "provider_refactor_matrix_truth_stable_case",
        "schema_version": 1,
        "case_name": case_name,
        "scenario": scenario,
        "input": inp,
        "expect": expect,
    }


def mutate_certified_unbound_evidence(inp):
    # Drop evidence on the first quality row.
    inp["rows"][0]["evidence_class"] = "evidence_unbound"


def mutate_missing_semantic_mode(inp):
    # Drop the semantic_layer_mode_admission row for framework_pack_lane.
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["lane_class"] == "framework_pack_lane"
            and row["row_class"] == "semantic_layer_mode_admission"
        )
    ]


def mutate_mutating_refactor_unsafe_rollback(inp):
    # Make the structured-artifact rename refactor lose its safe rollback.
    for row in inp["rows"]:
        if (
            row["lane_class"] == "structured_artifact_lane"
            and row["row_class"] == "refactor_transaction_admission"
        ):
            row["rollback_path_class"] = "no_safe_rollback_available"


def mutate_narrowed_no_disclosure(inp):
    # Narrow the first quality row below certified without a disclosure ref.
    inp["rows"][0]["support_class"] = "certified_below"
    inp["rows"][0]["disclosure_ref"] = None


def mutate_dimension_on_wrong_row_class(inp):
    # Bind a conflict class on the framework capability-negotiation row.
    for row in inp["rows"]:
        if (
            row["lane_class"] == "framework_pack_lane"
            and row["row_class"] == "capability_negotiation_admission"
        ):
            row["conflict_class"] = "single_provider_no_conflict"


def mutate_projection_collapse(inp):
    for proj in inp["consumer_projections"]:
        if proj["consumer_surface"] == "help_about":
            proj["preserves_provider_family_vocabulary"] = False


def mutate_raw_source(inp):
    inp["rows"][0]["raw_source_material_excluded"] = False


def main():
    artifact = build_artifact_packet()
    write_json(ARTIFACT_PATH, artifact)

    fixtures = [
        build_baseline_fixture(),
        with_modifier(
            "certified_with_unbound_evidence_blocks_stable",
            (
                "A row claiming certified but binding evidence_unbound is "
                "refused: the validator emits missing_evidence_class plus "
                "certified_with_unbound_binding and the packet blocks stable "
                "instead of inheriting the adjacent certified rows."
            ),
            mutate_certified_unbound_evidence,
            expected_overrides={"validation_finding_count": 2},
            expected_findings=[
                "missing_evidence_class",
                "certified_with_unbound_binding",
            ],
        ),
        with_modifier(
            "missing_semantic_mode_admission_blocks_stable",
            (
                "A lane claiming certified but missing its "
                "semantic_layer_mode_admission row is refused: the validator "
                "emits missing_semantic_layer_mode_coverage and the packet "
                "blocks stable until the semantic-mode row is restored, so an "
                "artifact family cannot claim a semantic posture it never "
                "enumerated."
            ),
            mutate_missing_semantic_mode,
            expected_overrides={"validation_finding_count": 1},
            expected_findings=["missing_semantic_layer_mode_coverage"],
        ),
        with_modifier(
            "mutating_refactor_without_safe_rollback_blocks_stable",
            (
                "A mutating refactor (rename) admission row whose rollback "
                "path is no_safe_rollback_available is refused: the validator "
                "emits mutation_bypasses_preview_or_rollback so AI-planned, "
                "schema/codegen, organize-imports, and notebook/generated "
                "edits cannot bypass typed preview and rollback checkpoints."
            ),
            mutate_mutating_refactor_unsafe_rollback,
            expected_overrides={"validation_finding_count": 1},
            expected_findings=["mutation_bypasses_preview_or_rollback"],
        ),
        with_modifier(
            "narrowed_row_missing_disclosure_ref_blocks_stable",
            (
                "A row narrowed below certified without a disclosure ref is "
                "refused: the validator emits narrowed_row_missing_disclosure_ref "
                "(and, because the row still binds a non-`none` downgrade "
                "automation, downgrade_automation_missing_disclosure_ref) and "
                "the packet blocks stable until the narrowing is disclosed."
            ),
            mutate_narrowed_no_disclosure,
            expected_overrides={"validation_finding_count": 2},
            expected_findings=[
                "narrowed_row_missing_disclosure_ref",
                "downgrade_automation_missing_disclosure_ref",
            ],
        ),
        with_modifier(
            "dimension_bound_on_wrong_row_class_blocks_stable",
            (
                "A capability_negotiation_admission row that also binds a "
                "conflict class is refused: the validator emits "
                "conflict_not_permitted_on_row_class so each matrix dimension "
                "stays owned by exactly one admission row class and cannot be "
                "smuggled onto another."
            ),
            mutate_dimension_on_wrong_row_class,
            expected_overrides={"validation_finding_count": 1},
            expected_findings=["conflict_not_permitted_on_row_class"],
        ),
        with_modifier(
            "projection_collapses_provider_family_vocabulary_blocks_stable",
            (
                "A consumer projection that collapses the provider-family "
                "vocabulary is refused: the validator emits "
                "provider_family_vocabulary_collapsed plus "
                "consumer_projection_drift and missing_consumer_projection and "
                "the packet blocks stable because surfaces MUST preserve the "
                "closed provider-family vocabulary that distinguishes LSP, "
                "framework analyzer, semantic graph, notebook adapter, "
                "generated-source bridge, AI overlay, and text fallback."
            ),
            mutate_projection_collapse,
            expected_overrides={"validation_finding_count": 3},
            expected_findings=[
                "provider_family_vocabulary_collapsed",
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
                "bodies, refactor diffs, generated artifact bodies, notebook "
                "outputs, provider payloads, secrets, and ambient credentials "
                "from its evidence surface."
            ),
            mutate_raw_source,
            expected_overrides={"validation_finding_count": 1},
            expected_findings=["raw_source_material_present"],
        ),
    ]

    for fx in fixtures:
        write_json(os.path.join(FIXTURE_DIR, fx["case_name"] + ".json"), fx)


if __name__ == "__main__":
    main()
