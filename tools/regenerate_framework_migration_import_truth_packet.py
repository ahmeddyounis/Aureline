#!/usr/bin/env python3
"""Regenerate the M4 framework migration and import guidance truth packet
artifact and fixture corpus.

Mirrors the Rust unit-test sample input in
crates/aureline-language/src/framework_migration_import_truth_packet/mod.rs.
The generator is the canonical seed for the checked-in artifact and the
narrowed-below-stable fixture cases used by the integration tests in
crates/aureline-language/tests/framework_migration_import_truth_packet.rs.

Run from anywhere:
    python3 tools/regenerate_framework_migration_import_truth_packet.py
"""
import json
import os

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

DOC_REF = "docs/languages/m4/finalize-framework-specific-migration-and-import-guidance-with.md"
FIXTURE_DIR = "fixtures/language/m4/framework_migration_import_truth_packet"
ARTIFACT_PATH = "artifacts/language/m4/framework_migration_import_truth_packet.json"
SCHEMA_REF = "schemas/language/framework_migration_import_truth.schema.json"

LANES = [
    ("framework_migration_guidance_lane", "framework", "python_launch_bundle"),
    ("import_guidance_lane", "import", "typescript_javascript_launch_bundle"),
    ("unsupported_gap_labeling_lane", "gap", "rust_launch_bundle"),
]

OUTCOME_LABELS = [
    "exact_match",
    "translated_match",
    "partial_match",
    "shimmed_match",
    "unsupported_gap",
]

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
        "outcome_label_class": "not_applicable",
        "rollback_checkpoint_class": "not_applicable",
        "diagnostic_preservation_class": "not_applicable",
        "launch_bundle_class": "not_applicable",
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
    row = base_row(f"row:{prefix}:quality", lane, "migration_guidance_quality")
    row["evidence_class"] = "archetype_repo_evidence"
    row["downgrade_automation_class"] = "auto_block_on_missing_evidence"
    row["disclosure_ref"] = f"{DOC_REF}#auto_block_on_missing_evidence"
    row["evidence_refs"] = [DOC_REF, FIXTURE_DIR]
    return row


def outcome_rows(lane, prefix):
    rows = []
    for label in OUTCOME_LABELS:
        row = base_row(
            f"row:{prefix}:outcome:{label}", lane, "outcome_label_truth"
        )
        row["outcome_label_class"] = label
        row["evidence_class"] = "framework_migration_evidence"
        rows.append(row)
    return rows


def rollback_row(lane, prefix):
    row = base_row(
        f"row:{prefix}:rollback:checkpoint_preserved",
        lane,
        "rollback_checkpoint_admission",
    )
    row["rollback_checkpoint_class"] = "checkpoint_preserved"
    row["evidence_class"] = "fixture_repo_evidence"
    return row


def diagnostic_row(lane, prefix):
    row = base_row(
        f"row:{prefix}:diagnostic:diagnostics_preserved",
        lane,
        "diagnostic_preservation_admission",
    )
    row["diagnostic_preservation_class"] = "diagnostics_preserved"
    row["evidence_class"] = "fixture_repo_evidence"
    return row


def bundle_row(lane, prefix, bundle):
    row = base_row(
        f"row:{prefix}:bundle:{bundle}",
        lane,
        "launch_bundle_coverage",
    )
    row["launch_bundle_class"] = bundle
    row["evidence_class"] = "archetype_repo_evidence"
    return row


def lane_rows(lane, prefix, bundle):
    rows = [quality_row(lane, prefix)]
    rows.extend(outcome_rows(lane, prefix))
    rows.append(rollback_row(lane, prefix))
    rows.append(diagnostic_row(lane, prefix))
    rows.append(bundle_row(lane, prefix, bundle))
    return rows


def projection(surface, packet_id, idx):
    return {
        "consumer_surface": surface,
        "projection_ref": f"projection:{surface}:stable",
        "framework_migration_import_packet_id_ref": packet_id,
        "rendered_at": RENDERED_AT_BASE.format(idx),
        "preserves_same_packet": True,
        "preserves_lane_vocabulary": True,
        "preserves_row_class_vocabulary": True,
        "preserves_support_class_vocabulary": True,
        "preserves_outcome_label_vocabulary": True,
        "preserves_rollback_checkpoint_vocabulary": True,
        "preserves_diagnostic_preservation_vocabulary": True,
        "preserves_launch_bundle_vocabulary": True,
        "preserves_known_limit_vocabulary": True,
        "preserves_downgrade_automation_vocabulary": True,
        "preserves_evidence_class_vocabulary": True,
        "supports_json_export": True,
        "raw_private_material_excluded": True,
        "ambient_authority_excluded": True,
    }


def build_input(packet_id, workflow_id):
    rows = []
    for lane, prefix, bundle in LANES:
        rows.extend(lane_rows(lane, prefix, bundle))
    projections = [
        projection(surface, packet_id, idx)
        for idx, surface in enumerate(CONSUMER_SURFACES)
    ]
    return {
        "packet_id": packet_id,
        "workflow_or_surface_id": workflow_id,
        "generated_at": TIMESTAMP,
        "covered_lanes": [lane for lane, _, _ in LANES],
        "rows": rows,
        "consumer_projections": projections,
        "source_contract_refs": [DOC_REF, SCHEMA_REF],
    }


def build_artifact_packet():
    pkt_id = "packet:m4:framework_migration_import:stable"
    workflow = "workflow.language.framework_migration_import.stable"
    inp = build_input(pkt_id, workflow)
    artifact = {
        "record_kind": "framework_migration_import_truth_stable_packet",
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
        "outcome_label_tokens": unique_tokens(rows, "outcome_label_class"),
        "rollback_checkpoint_tokens": unique_tokens(rows, "rollback_checkpoint_class"),
        "diagnostic_preservation_tokens": unique_tokens(
            rows, "diagnostic_preservation_class"
        ),
        "launch_bundle_tokens": unique_tokens(rows, "launch_bundle_class"),
        "known_limit_tokens": unique_tokens(rows, "known_limit_class"),
        "downgrade_automation_tokens": unique_tokens(
            rows, "downgrade_automation_class"
        ),
        "evidence_class_tokens": unique_tokens(rows, "evidence_class"),
        "support_export_safe": True,
    }
    expect.update(overrides)
    return expect


def build_baseline_fixture():
    pkt_id = "packet:m4:framework_migration_import:baseline_stable"
    workflow = "workflow.language.framework_migration_import.baseline_stable"
    inp = build_input(pkt_id, workflow)
    return {
        "record_kind": "framework_migration_import_truth_stable_case",
        "schema_version": 1,
        "case_name": "baseline_stable",
        "scenario": (
            "Baseline stable posture: every migration lane "
            "(framework_migration_guidance, import_guidance, "
            "unsupported_gap_labeling) carries a migration_guidance_quality "
            "row at launch_stable plus all five outcome_label_truth rows "
            "binding exact_match, translated_match, partial_match, "
            "shimmed_match, and unsupported_gap; each lane also surfaces "
            "a rollback_checkpoint_admission row binding "
            "checkpoint_preserved, a diagnostic_preservation_admission "
            "row binding diagnostics_preserved, and a "
            "launch_bundle_coverage row binding the launch bundle under "
            "proof; every row binds support, evidence, known-limit, "
            "downgrade-automation, outcome-label, rollback-checkpoint, "
            "and diagnostic-preservation classes; narrowed rows carry "
            "their disclosure refs; and all eight required consumer "
            "projections preserve the packet verbatim."
        ),
        "input": inp,
        "expect": expected_block(inp["rows"]),
    }


def with_modifier(
    case_name,
    scenario,
    mutate,
    expected_overrides=None,
    expected_findings=None,
):
    pkt_id = f"packet:m4:framework_migration_import:{case_name}"
    workflow = f"workflow.language.framework_migration_import.{case_name}"
    inp = build_input(pkt_id, workflow)
    mutate(inp)
    overrides = {
        "promotion_state": "blocks_stable",
        "support_export_safe": False,
    }
    if expected_overrides:
        overrides.update(expected_overrides)
    expect = expected_block(inp["rows"], **overrides)
    expect["validation_finding_count"] = overrides.get(
        "validation_finding_count", 1
    )
    if expected_findings:
        expect["expected_finding_kinds"] = expected_findings
    return {
        "record_kind": "framework_migration_import_truth_stable_case",
        "schema_version": 1,
        "case_name": case_name,
        "scenario": scenario,
        "input": inp,
        "expect": expect,
    }


def mutate_launch_stable_unbound_evidence(inp):
    # Drop evidence on the first quality row.
    inp["rows"][0]["evidence_class"] = "evidence_unbound"


def mutate_missing_outcome_label(inp):
    # Drop the unsupported_gap outcome_label_truth row for
    # framework_migration_guidance_lane.
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["lane_class"] == "framework_migration_guidance_lane"
            and row["row_class"] == "outcome_label_truth"
            and row["outcome_label_class"] == "unsupported_gap"
        )
    ]


def mutate_narrowed_no_disclosure(inp):
    # Narrow the first quality row below launch_stable without a disclosure ref.
    inp["rows"][0]["support_class"] = "launch_stable_below"
    inp["rows"][0]["disclosure_ref"] = None


def mutate_projection_collapse(inp):
    for projection in inp["consumer_projections"]:
        if projection["consumer_surface"] == "help_about":
            projection["preserves_outcome_label_vocabulary"] = False


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
                "evidence_class_tokens": [
                    "archetype_repo_evidence",
                    "evidence_unbound",
                    "fixture_repo_evidence",
                    "framework_migration_evidence",
                ],
            },
            expected_findings=[
                "missing_evidence_class",
                "launch_stable_with_unbound_binding",
            ],
        ),
        with_modifier(
            "missing_outcome_label_for_launch_stable_blocks_stable",
            (
                "A lane claiming launch_stable but missing the "
                "unsupported_gap outcome_label_truth row is refused: the "
                "validator emits missing_outcome_label_coverage and the "
                "packet blocks stable until the outcome label is restored."
            ),
            mutate_missing_outcome_label,
            expected_overrides={
                "validation_finding_count": 1,
                "row_count": 26,
            },
            expected_findings=["missing_outcome_label_coverage"],
        ),
        with_modifier(
            "narrowed_row_missing_disclosure_ref_blocks_stable",
            (
                "A row narrowed below launch_stable without a disclosure ref "
                "is refused: the validator emits "
                "narrowed_row_missing_disclosure_ref (and, because the row "
                "still binds a non-`none` downgrade automation, "
                "downgrade_automation_missing_disclosure_ref) and the packet "
                "blocks stable until the narrowing is disclosed."
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
            "projection_collapses_outcome_label_vocabulary_blocks_stable",
            (
                "A consumer projection that collapses the outcome-label "
                "vocabulary is refused: the validator emits "
                "outcome_label_vocabulary_collapsed plus "
                "consumer_projection_drift and the packet blocks stable "
                "until the projection preserves the closed vocabulary."
            ),
            mutate_projection_collapse,
            expected_overrides={
                "validation_finding_count": 3,
            },
            expected_findings=[
                "outcome_label_vocabulary_collapsed",
                "consumer_projection_drift",
                "missing_consumer_projection",
            ],
        ),
        with_modifier(
            "raw_source_material_blocks_stable",
            (
                "A row that admits raw source bodies past the boundary is "
                "refused: the validator emits raw_source_material_present "
                "and the packet blocks stable until the row excludes raw "
                "source material from its evidence surface."
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
