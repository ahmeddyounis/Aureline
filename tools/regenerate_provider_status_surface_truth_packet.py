#!/usr/bin/env python3
"""Regenerate the M5 provider-status surface truth packet artifact and fixtures.

The packet binds the three reusable surface UI objects — provider-status
strip, capability-negotiation drawer, and result-provenance pill — across
the M5 framework, notebook, generated-source, preview, docs-linked, and
structured-artifact surfaces, reading the closed provider vocabulary frozen
by the sibling provider/refactor matrix packet.

Mirrors the Rust unit-test sample input in
crates/aureline-language/src/provider_status_surface_truth_packet/tests.rs and
seeds the checked-in artifact plus the narrowed-below-stable fixture cases
exercised by the integration tests in
crates/aureline-language/tests/provider_status_surface_truth_packet.rs.

Run from anywhere:
    python3 tools/regenerate_provider_status_surface_truth_packet.py
"""
import json
import os

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

DOC_REF = (
    "docs/m5/provider-status-strips-capability-negotiation-drawers-and-"
    "result-provenance-pills.md"
)
FIXTURE_DIR = "fixtures/language/m5/provider_status_surface_truth_packet"
ARTIFACT_PATH = "artifacts/language/m5/provider_status_surface_truth_packet.json"
SCHEMA_REF = "schemas/language/provider_status_surface_truth.schema.json"
MATRIX_SOURCE_REF = "artifacts/language/m5/provider_refactor_matrix_truth_packet.json"

TIMESTAMP = "2026-06-14T12:00:00Z"
RENDERED_AT_BASE = "2026-06-14T12:00:0{}Z"

# One posture per surface. Each surface renders all three UI objects bound to
# the provider family, locality, lifecycle state, capability, conflict,
# scope, freshness, recovery action, provenance anchor, and downgrade label
# it may claim.
LANE_SPECS = [
    {
        "surface": "framework_surface",
        "prefix": "framework",
        "provider": "framework_analyzer",
        "locality": "workspace_local_process",
        "lifecycle": "ready_live",
        "capability": "full_semantic_negotiated",
        "detail_route": "open_negotiation_drawer",
        "conflict": "arbitrated_winner_loser_preserved",
        "has_loser": True,
        "result_form": "arbitrated_winner_result",
        "scope": "full_workspace_scope",
        "freshness": "fresh_live",
        "recovery": "retry_request",
        "anchor": "framework_aware_result",
        "provenance": "live_semantic",
        "pill_completeness": "not_applicable",
        "downgrade_label": "full_to_partial_completeness",
    },
    {
        "surface": "notebook_surface",
        "prefix": "notebook",
        "provider": "notebook_adapter",
        "locality": "notebook_kernel_session",
        "lifecycle": "degraded_partial",
        "capability": "partial_semantic_negotiated",
        "detail_route": "open_capability_inspector",
        "conflict": "single_provider_no_conflict",
        "has_loser": False,
        "result_form": "single_provider_result",
        "scope": "open_cells_scope",
        "freshness": "cached_recent",
        "recovery": "restart_provider",
        "anchor": "completion_result",
        "provenance": "cached_semantic",
        "pill_completeness": "not_applicable",
        "downgrade_label": "semantic_to_text_fallback",
    },
    {
        "surface": "generated_source_surface",
        "prefix": "generated",
        "provider": "generated_source_bridge",
        "locality": "in_process_engine",
        "lifecycle": "ready_live",
        "capability": "text_fallback_negotiated",
        "detail_route": "open_scope_limit_detail",
        "conflict": "policy_override_recorded",
        "has_loser": False,
        "result_form": "policy_override_result",
        "scope": "sparse_index_scope",
        "freshness": "imported_snapshot",
        "recovery": "regenerate_from_source",
        "anchor": "definition_result",
        "provenance": "imported_scan",
        "pill_completeness": "not_applicable",
        "downgrade_label": "generated_edit_to_regenerate_first",
    },
    {
        "surface": "preview_surface",
        "prefix": "preview",
        "provider": "lsp_provider",
        "locality": "local_host_subprocess",
        "lifecycle": "ready_live",
        "capability": "full_semantic_negotiated",
        "detail_route": "open_negotiation_drawer",
        "conflict": "single_provider_no_conflict",
        "has_loser": False,
        "result_form": "single_provider_result",
        "scope": "full_workspace_scope",
        "freshness": "fresh_live",
        "recovery": "rerun_preview",
        "anchor": "rename_preview",
        "provenance": "live_semantic",
        "pill_completeness": "complete",
        "downgrade_label": "previewable_to_compare_only",
    },
    {
        "surface": "docs_linked_surface",
        "prefix": "docs",
        "provider": "ai_overlay",
        "locality": "remote_managed_service",
        "lifecycle": "restarting",
        "capability": "partial_semantic_negotiated",
        "detail_route": "open_provider_health_panel",
        "conflict": "single_provider_no_conflict",
        "has_loser": False,
        "result_form": "fused_result",
        "scope": "single_file_scope",
        "freshness": "stale_pending_refresh",
        "recovery": "refresh_result",
        "anchor": "hover_doc_result",
        "provenance": "stale_pending_refresh",
        "pill_completeness": "not_applicable",
        "downgrade_label": "provider_unavailable_text_only",
    },
    {
        "surface": "structured_artifact_surface",
        "prefix": "structured",
        "provider": "lsp_provider",
        "locality": "local_host_subprocess",
        "lifecycle": "ready_live",
        "capability": "full_semantic_negotiated",
        "detail_route": "open_capability_inspector",
        "conflict": "unresolved_disagreement_surfaced",
        "has_loser": True,
        "result_form": "unresolved_disagreement_result",
        "scope": "workset_subset_scope",
        "freshness": "fresh_live",
        "recovery": "retry_request",
        "anchor": "reference_result",
        "provenance": "live_semantic",
        "pill_completeness": "not_applicable",
        "downgrade_label": "full_to_partial_completeness",
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

STRIP = "provider_status_strip"
DRAWER = "capability_negotiation_drawer"
PILL = "result_provenance_pill"


def base_row(row_id, surface, object_kind, row_class):
    return {
        "row_id": row_id,
        "surface_lane": surface,
        "object_kind": object_kind,
        "row_class": row_class,
        "support_class": "certified",
        "provider_family_class": "not_applicable",
        "provider_locality_class": "not_applicable",
        "provider_lifecycle_state_class": "not_applicable",
        "provider_display_label_class": "not_applicable",
        "capability_negotiation_class": "not_applicable",
        "capability_detail_route_class": "not_applicable",
        "participant_role_class": "not_applicable",
        "conflict_class": "not_applicable",
        "selected_result_form_class": "not_applicable",
        "scope_limit_class": "not_applicable",
        "freshness_class": "not_applicable",
        "recovery_action_class": "not_applicable",
        "provenance_anchor_target_class": "not_applicable",
        "result_provenance_class": "not_applicable",
        "completeness_class": "not_applicable",
        "downgrade_label_class": "not_applicable",
        "evidence_class": "fixture_repo_evidence",
        "known_limit_class": "none_declared",
        "downgrade_automation_class": "auto_narrow_on_missing_fixture",
        "confidence_class": "high_confidence",
        "evidence_refs": [FIXTURE_DIR],
        "disclosure_ref": f"{DOC_REF}#auto_narrow_on_missing_fixture",
        "provenance_requires_raw_logs": False,
        "raw_source_material_excluded": True,
        "secrets_excluded": True,
        "ambient_authority_excluded": True,
        "captured_at": TIMESTAMP,
    }


def lane_rows(spec):
    prefix = spec["prefix"]
    surface = spec["surface"]
    rows = []

    # Provider-status strip.
    strip_presence = base_row(
        f"row:{prefix}:strip:presence", surface, STRIP, "surface_object_presence"
    )
    strip_presence["provider_family_class"] = spec["provider"]
    strip_presence["provider_display_label_class"] = "human_readable_lane_label"
    strip_presence["evidence_class"] = "archetype_repo_evidence"
    strip_presence["downgrade_automation_class"] = "auto_block_on_missing_evidence"
    strip_presence["disclosure_ref"] = f"{DOC_REF}#auto_block_on_missing_evidence"
    strip_presence["evidence_refs"] = [DOC_REF, FIXTURE_DIR]
    rows.append(strip_presence)

    lane_state = base_row(
        f"row:{prefix}:strip:lane_state", surface, STRIP, "provider_lane_state_admission"
    )
    lane_state["provider_family_class"] = spec["provider"]
    lane_state["provider_display_label_class"] = "provider_family_with_locality_label"
    lane_state["provider_locality_class"] = spec["locality"]
    lane_state["provider_lifecycle_state_class"] = spec["lifecycle"]
    rows.append(lane_state)

    route = base_row(
        f"row:{prefix}:strip:detail_route", surface, STRIP, "capability_detail_route_admission"
    )
    route["capability_detail_route_class"] = spec["detail_route"]
    route["capability_negotiation_class"] = spec["capability"]
    rows.append(route)

    # Capability-negotiation drawer.
    drawer_presence = base_row(
        f"row:{prefix}:drawer:presence", surface, DRAWER, "surface_object_presence"
    )
    drawer_presence["provider_family_class"] = spec["provider"]
    drawer_presence["provider_display_label_class"] = "human_readable_lane_label"
    rows.append(drawer_presence)

    winner = base_row(
        f"row:{prefix}:drawer:winner", surface, DRAWER, "participating_provider_admission"
    )
    winner["participant_role_class"] = "selected_winner"
    winner["conflict_class"] = spec["conflict"]
    rows.append(winner)

    if spec["has_loser"]:
        loser = base_row(
            f"row:{prefix}:drawer:loser", surface, DRAWER, "participating_provider_admission"
        )
        loser["participant_role_class"] = "preserved_loser"
        loser["conflict_class"] = spec["conflict"]
        rows.append(loser)

    result = base_row(
        f"row:{prefix}:drawer:result", surface, DRAWER, "negotiation_result_admission"
    )
    result["selected_result_form_class"] = spec["result_form"]
    rows.append(result)

    scope = base_row(
        f"row:{prefix}:drawer:scope_freshness", surface, DRAWER, "scope_and_freshness_admission"
    )
    scope["scope_limit_class"] = spec["scope"]
    scope["freshness_class"] = spec["freshness"]
    rows.append(scope)

    recovery = base_row(
        f"row:{prefix}:drawer:recovery", surface, DRAWER, "drawer_recovery_action_admission"
    )
    recovery["recovery_action_class"] = spec["recovery"]
    rows.append(recovery)

    # Result-provenance pill.
    pill_presence = base_row(
        f"row:{prefix}:pill:presence", surface, PILL, "surface_object_presence"
    )
    pill_presence["provider_family_class"] = spec["provider"]
    pill_presence["provider_display_label_class"] = "human_readable_lane_label"
    rows.append(pill_presence)

    anchor = base_row(
        f"row:{prefix}:pill:anchor", surface, PILL, "provenance_anchor_admission"
    )
    anchor["provenance_anchor_target_class"] = spec["anchor"]
    anchor["result_provenance_class"] = spec["provenance"]
    anchor["completeness_class"] = spec["pill_completeness"]
    anchor["evidence_class"] = "conformance_suite_evidence"
    rows.append(anchor)

    downgrade = base_row(
        f"row:{prefix}:pill:downgrade", surface, PILL, "provenance_downgrade_admission"
    )
    downgrade["downgrade_label_class"] = spec["downgrade_label"]
    rows.append(downgrade)

    return rows


def projection(surface, packet_id, idx):
    return {
        "consumer_surface": surface,
        "projection_ref": f"projection:{surface}:stable",
        "surface_packet_id_ref": packet_id,
        "rendered_at": RENDERED_AT_BASE.format(idx % 10),
        "preserves_same_packet": True,
        "preserves_surface_lane_vocabulary": True,
        "preserves_object_kind_vocabulary": True,
        "preserves_row_class_vocabulary": True,
        "preserves_support_class_vocabulary": True,
        "preserves_provider_family_vocabulary": True,
        "preserves_provider_locality_vocabulary": True,
        "preserves_provider_lifecycle_state_vocabulary": True,
        "preserves_provider_display_label_vocabulary": True,
        "preserves_capability_negotiation_vocabulary": True,
        "preserves_capability_detail_route_vocabulary": True,
        "preserves_participant_role_vocabulary": True,
        "preserves_conflict_vocabulary": True,
        "preserves_selected_result_form_vocabulary": True,
        "preserves_scope_limit_vocabulary": True,
        "preserves_freshness_vocabulary": True,
        "preserves_recovery_action_vocabulary": True,
        "preserves_provenance_anchor_target_vocabulary": True,
        "preserves_result_provenance_vocabulary": True,
        "preserves_completeness_vocabulary": True,
        "preserves_downgrade_label_vocabulary": True,
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
        "covered_surfaces": [spec["surface"] for spec in LANE_SPECS],
        "rows": rows,
        "consumer_projections": projections,
        "source_contract_refs": [MATRIX_SOURCE_REF, DOC_REF, SCHEMA_REF],
    }


def build_artifact_packet():
    pkt_id = "packet:m5:provider_status_surface:stable"
    workflow = "workflow.language.provider_status_surface.stable"
    inp = build_input(pkt_id, workflow)
    return {
        "record_kind": "provider_status_surface_truth_stable_packet",
        "schema_version": 1,
        "packet_id": pkt_id,
        "workflow_or_surface_id": workflow,
        "generated_at": TIMESTAMP,
        "covered_surfaces": inp["covered_surfaces"],
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
        "surface_lane_tokens": unique_tokens(rows, "surface_lane"),
        "object_kind_tokens": unique_tokens(rows, "object_kind"),
        "row_class_tokens": unique_tokens(rows, "row_class"),
        "support_class_tokens": unique_tokens(rows, "support_class"),
        "provider_family_tokens": unique_tokens(rows, "provider_family_class"),
        "provider_locality_tokens": unique_tokens(rows, "provider_locality_class"),
        "provider_lifecycle_state_tokens": unique_tokens(rows, "provider_lifecycle_state_class"),
        "provider_display_label_tokens": unique_tokens(rows, "provider_display_label_class"),
        "capability_negotiation_tokens": unique_tokens(rows, "capability_negotiation_class"),
        "capability_detail_route_tokens": unique_tokens(rows, "capability_detail_route_class"),
        "participant_role_tokens": unique_tokens(rows, "participant_role_class"),
        "conflict_tokens": unique_tokens(rows, "conflict_class"),
        "selected_result_form_tokens": unique_tokens(rows, "selected_result_form_class"),
        "scope_limit_tokens": unique_tokens(rows, "scope_limit_class"),
        "freshness_tokens": unique_tokens(rows, "freshness_class"),
        "recovery_action_tokens": unique_tokens(rows, "recovery_action_class"),
        "provenance_anchor_target_tokens": unique_tokens(rows, "provenance_anchor_target_class"),
        "result_provenance_tokens": unique_tokens(rows, "result_provenance_class"),
        "completeness_tokens": unique_tokens(rows, "completeness_class"),
        "downgrade_label_tokens": unique_tokens(rows, "downgrade_label_class"),
        "known_limit_tokens": unique_tokens(rows, "known_limit_class"),
        "downgrade_automation_tokens": unique_tokens(rows, "downgrade_automation_class"),
        "evidence_class_tokens": unique_tokens(rows, "evidence_class"),
        "support_export_safe": True,
    }
    expect.update(overrides)
    return expect


def build_baseline_fixture():
    pkt_id = "packet:m5:provider_status_surface:baseline_stable"
    workflow = "workflow.language.provider_status_surface.baseline_stable"
    inp = build_input(pkt_id, workflow)
    return {
        "record_kind": "provider_status_surface_truth_stable_case",
        "schema_version": 1,
        "case_name": "baseline_stable",
        "scenario": (
            "Baseline stable posture: every surface (framework, notebook, "
            "generated source, preview, docs-linked, and structured artifact) "
            "carries a provider-status strip, a capability-negotiation drawer, "
            "and a result-provenance pill. Each strip names a concrete acting "
            "provider family with a human-readable label, binds where the "
            "provider runs and what lifecycle state it is in, and offers an "
            "inspectable capability-detail route instead of an opaque spinner. "
            "Each drawer lists the participating providers — preserving the "
            "losing provider wherever a conflict is arbitrated or unresolved — "
            "names the selected winner / fused result form, the scope limit, "
            "the freshness, and a retry / restart recovery action. Each pill "
            "anchors provenance to a definition, reference, completion, rename "
            "preview, or framework-aware result without forcing raw logs, and "
            "a rename-preview anchor binds a typed, complete preview. Every "
            "row binds support, known-limit, downgrade-automation, and "
            "evidence classes, narrowed rows carry disclosure refs, and all "
            "ten required consumer projections preserve the packet verbatim."
        ),
        "input": inp,
        "expect": expected_block(inp["rows"]),
    }


def with_modifier(case_name, scenario, mutate, expected_overrides=None,
                  expected_findings=None):
    pkt_id = f"packet:m5:provider_status_surface:{case_name}"
    workflow = f"workflow.language.provider_status_surface.{case_name}"
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
        "record_kind": "provider_status_surface_truth_stable_case",
        "schema_version": 1,
        "case_name": case_name,
        "scenario": scenario,
        "input": inp,
        "expect": expect,
    }


def first_row(inp, surface, row_class):
    for row in inp["rows"]:
        if row["surface_lane"] == surface and row["row_class"] == row_class:
            return row
    raise KeyError(f"no {row_class} row for {surface}")


def mutate_certified_unbound_evidence(inp):
    # Drop evidence on the first strip-presence row.
    inp["rows"][0]["evidence_class"] = "evidence_unbound"


def mutate_missing_lane_state(inp):
    # Drop the provider-lane-state row for framework_surface.
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["surface_lane"] == "framework_surface"
            and row["row_class"] == "provider_lane_state_admission"
        )
    ]


def mutate_opaque_spinner_route(inp):
    row = first_row(inp, "framework_surface", "capability_detail_route_admission")
    row["capability_detail_route_class"] = "opaque_spinner_only"


def mutate_losing_provider_not_preserved(inp):
    # Drop the preserved-loser participant on the structured-artifact drawer
    # while the conflict stays unresolved.
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["surface_lane"] == "structured_artifact_surface"
            and row["row_class"] == "participating_provider_admission"
            and row["participant_role_class"] == "preserved_loser"
        )
    ]


def mutate_raw_process_name_only_label(inp):
    inp["rows"][0]["provider_display_label_class"] = "raw_process_name_only"


def mutate_dimension_on_wrong_row_class(inp):
    # Bind a scope limit on the framework strip lane-state row.
    row = first_row(inp, "framework_surface", "provider_lane_state_admission")
    row["scope_limit_class"] = "full_workspace_scope"


def mutate_projection_collapse(inp):
    for proj in inp["consumer_projections"]:
        if proj["consumer_surface"] == "help_about":
            proj["preserves_result_provenance_vocabulary"] = False


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
            "status_strip_missing_lane_state_blocks_stable",
            (
                "A surface whose provider-status strip is certified but is "
                "missing its provider-lane-state row is refused: the validator "
                "emits missing_provider_lane_state_coverage so a strip cannot "
                "claim it shows where a provider runs and what state it is in "
                "while omitting that row."
            ),
            mutate_missing_lane_state,
            expected_overrides={"validation_finding_count": 1},
            expected_findings=["missing_provider_lane_state_coverage"],
        ),
        with_modifier(
            "opaque_spinner_detail_route_blocks_stable",
            (
                "A capability-detail route that resolves to an opaque loading "
                "spinner is refused: the validator emits "
                "capability_detail_route_is_opaque_spinner so notebook, "
                "generated, workset, and sparse-scope limits cannot be hidden "
                "behind a generic spinner instead of an inspectable route."
            ),
            mutate_opaque_spinner_route,
            expected_overrides={"validation_finding_count": 1},
            expected_findings=["capability_detail_route_is_opaque_spinner"],
        ),
        with_modifier(
            "losing_provider_not_preserved_blocks_stable",
            (
                "A capability-negotiation drawer that surfaces a provider "
                "disagreement but drops the losing provider is refused: the "
                "validator emits losing_provider_not_preserved so disagreement "
                "is never collapsed into a ranking-only result and the losing "
                "provider and downgrade reason stay inspectable."
            ),
            mutate_losing_provider_not_preserved,
            expected_overrides={"validation_finding_count": 1},
            expected_findings=["losing_provider_not_preserved"],
        ),
        with_modifier(
            "raw_process_name_only_label_blocks_stable",
            (
                "A provider-status strip whose only user-facing label is a raw "
                "internal process name is refused: the validator emits "
                "raw_process_name_only_label so raw process names are never "
                "exposed as the only provider label."
            ),
            mutate_raw_process_name_only_label,
            expected_overrides={"validation_finding_count": 1},
            expected_findings=["raw_process_name_only_label"],
        ),
        with_modifier(
            "dimension_bound_on_wrong_row_class_blocks_stable",
            (
                "A provider-lane-state row that also binds a scope limit is "
                "refused: the validator emits "
                "scope_limit_not_permitted_on_row_class so each UI-object "
                "dimension stays owned by exactly one admission row class and "
                "cannot be smuggled onto another."
            ),
            mutate_dimension_on_wrong_row_class,
            expected_overrides={"validation_finding_count": 1},
            expected_findings=["scope_limit_not_permitted_on_row_class"],
        ),
        with_modifier(
            "projection_collapses_result_provenance_vocabulary_blocks_stable",
            (
                "A consumer projection that collapses the result-provenance "
                "vocabulary is refused: the validator emits "
                "result_provenance_vocabulary_collapsed plus "
                "consumer_projection_drift and missing_consumer_projection and "
                "the packet blocks stable because surfaces MUST preserve the "
                "closed result-provenance vocabulary that distinguishes live, "
                "cached, partial, text-heuristic, imported, and stale results."
            ),
            mutate_projection_collapse,
            expected_overrides={"validation_finding_count": 3},
            expected_findings=[
                "result_provenance_vocabulary_collapsed",
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
