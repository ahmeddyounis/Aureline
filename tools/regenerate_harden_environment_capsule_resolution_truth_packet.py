#!/usr/bin/env python3
"""Regenerate the M4 harden-environment-capsule-resolution truth packet
artifact and fixture corpus.

Mirrors the Rust unit-test sample input in
crates/aureline-runtime/src/harden_environment_capsule_resolution/mod.rs.
The generator is the canonical seed for the checked-in artifact and the
narrowed-below-stable fixture cases used by the integration tests in
crates/aureline-runtime/tests/harden_environment_capsule_resolution_truth_packet.rs.

Run from anywhere:
    python3 tools/regenerate_harden_environment_capsule_resolution_truth_packet.py
"""
import json
import os

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

DOC_REF = "docs/runtime/m4/harden-environment-capsule-resolution.md"
FIXTURE_DIR = "fixtures/runtime/m4/harden_environment_capsule_resolution"
ARTIFACT_PATH = (
    "artifacts/runtime/m4/"
    "harden_environment_capsule_resolution_truth_packet.json"
)
SCHEMA_REF = (
    "schemas/runtime/"
    "harden_environment_capsule_resolution_truth.schema.json"
)

LANES = [
    ("devcontainer_lane", "devcontainer"),
    ("nix_lane", "nix"),
    ("compose_lane", "compose"),
    ("shell_sdk_lane", "shell_sdk"),
    ("template_prebuild_lane", "template_prebuild"),
]

CAPSULE_FIELDS = [
    "host_or_base_image_identity",
    "target_plan",
    "resolved_toolchain_locks",
    "projected_environment_variables",
    "secret_references",
    "writable_mount_model",
    "service_startup_ordering",
    "trust_network_posture",
    "provenance",
]

FINGERPRINT_COMPONENTS = [
    "commit_or_tree_identity",
    "capsule_hash",
    "platform_arch",
    "policy_epoch",
    "extension_lock_digest",
    "critical_toolchain_digest",
]

INVALIDATION_REASONS = [
    "cold_path",
    "partially_warm_path",
    "fingerprint_mismatch",
    "untrusted_template_metadata",
    "blocked_hook",
    "stale_prebuild",
]

PROJECT_DOCTOR_FINDINGS = [
    "wrong_interpreter",
    "stale_prebuild",
    "blocked_activator",
    "drifted_toolchain",
    "untrusted_template_metadata",
]

CONSUMER_SURFACES = [
    "editor_run_surface",
    "terminal_pane",
    "task_panel",
    "cli_headless",
    "project_doctor",
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
        "capsule_field_class": "not_applicable",
        "prebuild_fingerprint_component_class": "not_applicable",
        "invalidation_reason_class": "not_applicable",
        "project_doctor_finding_class": "not_applicable",
        "evidence_class": "fixture_repo_evidence",
        "known_limit_class": "none_declared",
        "downgrade_automation_class": "auto_narrow_on_capsule_field_gap",
        "confidence_class": "high_confidence",
        "evidence_refs": [FIXTURE_DIR],
        "disclosure_ref": f"{DOC_REF}#auto_narrow_on_capsule_field_gap",
        "no_silent_prebuild_reuse": False,
        "raw_source_material_excluded": True,
        "secrets_excluded": True,
        "ambient_authority_excluded": True,
        "captured_at": TIMESTAMP,
    }


def quality_row(lane, prefix):
    row = base_row(
        f"row:{prefix}:quality", lane, "capsule_resolution_quality"
    )
    row["evidence_class"] = "release_evidence_review"
    row["downgrade_automation_class"] = "auto_block_on_missing_evidence"
    row["disclosure_ref"] = f"{DOC_REF}#auto_block_on_missing_evidence"
    row["evidence_refs"] = [DOC_REF, FIXTURE_DIR]
    return row


def capsule_field_rows(lane, prefix):
    rows = []
    for field in CAPSULE_FIELDS:
        row = base_row(
            f"row:{prefix}:field:{field}", lane, "capsule_field_admission"
        )
        row["capsule_field_class"] = field
        row["evidence_class"] = "conformance_suite_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_capsule_field_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_capsule_field_gap"
        rows.append(row)
    return rows


def fingerprint_rows(lane, prefix):
    rows = []
    for component in FINGERPRINT_COMPONENTS:
        row = base_row(
            f"row:{prefix}:fingerprint:{component}",
            lane,
            "prebuild_fingerprint_admission",
        )
        row["prebuild_fingerprint_component_class"] = component
        row["evidence_class"] = "conformance_suite_evidence"
        row["downgrade_automation_class"] = "auto_narrow_on_fingerprint_gap"
        row["disclosure_ref"] = f"{DOC_REF}#auto_narrow_on_fingerprint_gap"
        rows.append(row)
    return rows


def invalidation_rows(lane, prefix):
    rows = []
    for reason in INVALIDATION_REASONS:
        row = base_row(
            f"row:{prefix}:invalidation:{reason}",
            lane,
            "invalidation_reason_admission",
        )
        row["invalidation_reason_class"] = reason
        row["evidence_class"] = "failure_recovery_drill_evidence"
        row["downgrade_automation_class"] = (
            "auto_narrow_on_invalidation_reason_gap"
        )
        row["disclosure_ref"] = (
            f"{DOC_REF}#auto_narrow_on_invalidation_reason_gap"
        )
        rows.append(row)
    return rows


def project_doctor_rows(lane, prefix):
    rows = []
    for finding in PROJECT_DOCTOR_FINDINGS:
        row = base_row(
            f"row:{prefix}:doctor:{finding}",
            lane,
            "project_doctor_finding_admission",
        )
        row["project_doctor_finding_class"] = finding
        row["evidence_class"] = "failure_recovery_drill_evidence"
        row["downgrade_automation_class"] = (
            "auto_narrow_on_project_doctor_finding_gap"
        )
        row["disclosure_ref"] = (
            f"{DOC_REF}#auto_narrow_on_project_doctor_finding_gap"
        )
        rows.append(row)
    return rows


def materialized_identity_row(lane, prefix):
    row = base_row(
        f"row:{prefix}:materialized_identity_admission",
        lane,
        "materialized_identity_admission",
    )
    row["evidence_class"] = "automated_functional_evidence"
    row["downgrade_automation_class"] = (
        "auto_narrow_on_materialized_identity_drift"
    )
    row["disclosure_ref"] = (
        f"{DOC_REF}#auto_narrow_on_materialized_identity_drift"
    )
    row["requested_artifact_identity_binding"] = (
        f"capsule_request:m4:{prefix}:requested"
    )
    row["materialized_runtime_identity_binding"] = (
        f"capsule_instance:m4:{prefix}:materialized"
    )
    row["no_silent_prebuild_reuse"] = True
    return row


def lane_rows(lane, prefix):
    rows = [quality_row(lane, prefix)]
    rows.extend(capsule_field_rows(lane, prefix))
    rows.extend(fingerprint_rows(lane, prefix))
    rows.extend(invalidation_rows(lane, prefix))
    rows.extend(project_doctor_rows(lane, prefix))
    rows.append(materialized_identity_row(lane, prefix))
    return rows


def projection(surface, packet_id):
    return {
        "consumer_surface": surface,
        "projection_ref": f"projection:{surface}",
        "capsule_resolution_packet_id_ref": packet_id,
        "rendered_at": "2026-05-26T12:00:01Z",
        "preserves_same_packet": True,
        "preserves_lane_vocabulary": True,
        "preserves_row_class_vocabulary": True,
        "preserves_support_class_vocabulary": True,
        "preserves_capsule_field_vocabulary": True,
        "preserves_prebuild_fingerprint_vocabulary": True,
        "preserves_invalidation_reason_vocabulary": True,
        "preserves_project_doctor_finding_vocabulary": True,
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
    packet_id = "packet:m4:harden_environment_capsule_resolution:stable"
    workflow_id = "workflow.runtime.harden_environment_capsule_resolution.stable"
    base = baseline_input(packet_id, workflow_id)
    base.update(
        {
            "record_kind":
                "harden_environment_capsule_resolution_truth_stable_packet",
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


def expectations_from_input(
    input_obj, promotion_state, finding_count, expected_findings=None
):
    rows = input_obj["rows"]
    expect = {
        "promotion_state": promotion_state,
        "validation_finding_count": finding_count,
        "row_count": len(rows),
        "lane_tokens": sorted({row["lane_class"] for row in rows}),
        "row_class_tokens": sorted({row["row_class"] for row in rows}),
        "support_class_tokens": sorted({row["support_class"] for row in rows}),
        "capsule_field_tokens": sorted(
            {row["capsule_field_class"] for row in rows}
        ),
        "prebuild_fingerprint_tokens": sorted(
            {row["prebuild_fingerprint_component_class"] for row in rows}
        ),
        "invalidation_reason_tokens": sorted(
            {row["invalidation_reason_class"] for row in rows}
        ),
        "project_doctor_finding_tokens": sorted(
            {row["project_doctor_finding_class"] for row in rows}
        ),
        "known_limit_tokens": sorted({row["known_limit_class"] for row in rows}),
        "downgrade_automation_tokens": sorted(
            {row["downgrade_automation_class"] for row in rows}
        ),
        "evidence_class_tokens": sorted(
            {row["evidence_class"] for row in rows}
        ),
        "support_export_safe": promotion_state == "stable",
    }
    if expected_findings:
        expect["expected_finding_kinds"] = expected_findings
    return expect


def baseline_case():
    pid = (
        "packet:m4:harden_environment_capsule_resolution:baseline_stable"
    )
    wid = (
        "workflow.runtime.harden_environment_capsule_resolution.baseline_stable"
    )
    inp = baseline_input(pid, wid)
    return {
        "record_kind":
            "harden_environment_capsule_resolution_truth_stable_case",
        "schema_version": 1,
        "case_name": "baseline_stable",
        "scenario": (
            "Baseline stable posture: all five capsule-resolution lanes "
            "(devcontainer, nix, compose, shell_sdk, template_prebuild) "
            "publish a capsule_resolution_quality row at launch_stable plus "
            "the full nine-field typed-capsule coverage (host_or_base_image_"
            "identity, target_plan, resolved_toolchain_locks, "
            "projected_environment_variables, secret_references, "
            "writable_mount_model, service_startup_ordering, "
            "trust_network_posture, provenance), the full six-component "
            "prebuild fingerprint coverage (commit_or_tree_identity, "
            "capsule_hash, platform_arch, policy_epoch, extension_lock_digest, "
            "critical_toolchain_digest), the full six-reason invalidation "
            "coverage (cold_path, partially_warm_path, fingerprint_mismatch, "
            "untrusted_template_metadata, blocked_hook, stale_prebuild), the "
            "full five-finding project-doctor coverage (wrong_interpreter, "
            "stale_prebuild, blocked_activator, drifted_toolchain, "
            "untrusted_template_metadata), and a "
            "materialized_identity_admission row binding both the requested "
            "and materialized identities while attesting "
            "no_silent_prebuild_reuse; every row binds support, known limit, "
            "downgrade automation, and evidence classes; narrowed rows carry "
            "their disclosure refs; and all nine required consumer "
            "projections preserve the packet verbatim."
        ),
        "input": inp,
        "expect": expectations_from_input(inp, "stable", 0),
    }


def unbound_evidence_case():
    pid = (
        "packet:m4:harden_environment_capsule_resolution:"
        "launch_stable_with_unbound_evidence"
    )
    wid = (
        "workflow.runtime.harden_environment_capsule_resolution."
        "launch_stable_with_unbound_evidence"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:devcontainer:quality":
            row["evidence_class"] = "evidence_unbound"
            break
    return {
        "record_kind":
            "harden_environment_capsule_resolution_truth_stable_case",
        "schema_version": 1,
        "case_name": "launch_stable_with_unbound_evidence_blocks_stable",
        "scenario": (
            "The devcontainer lane's capsule_resolution_quality row claims "
            "launch_stable while its evidence class is evidence_unbound; "
            "the packet blocks the stable claim because no automated, "
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


def missing_fingerprint_component_case():
    pid = (
        "packet:m4:harden_environment_capsule_resolution:"
        "missing_prebuild_fingerprint_component"
    )
    wid = (
        "workflow.runtime.harden_environment_capsule_resolution."
        "missing_prebuild_fingerprint_component"
    )
    inp = baseline_input(pid, wid)
    inp["rows"] = [
        row
        for row in inp["rows"]
        if not (
            row["row_class"] == "prebuild_fingerprint_admission"
            and row["prebuild_fingerprint_component_class"]
            == "critical_toolchain_digest"
            and row["lane_class"] == "devcontainer_lane"
        )
    ]
    return {
        "record_kind":
            "harden_environment_capsule_resolution_truth_stable_case",
        "schema_version": 1,
        "case_name":
            "missing_prebuild_fingerprint_component_blocks_stable",
        "scenario": (
            "The devcontainer lane claims launch_stable but drops its "
            "critical_toolchain_digest prebuild_fingerprint_admission row; "
            "the packet blocks the stable claim because every launch_stable "
            "lane must admit all six prebuild fingerprint components "
            "(commit_or_tree_identity, capsule_hash, platform_arch, "
            "policy_epoch, extension_lock_digest, critical_toolchain_digest) "
            "before reusing any prebuild artifact."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            1,
            expected_findings=["missing_prebuild_fingerprint_coverage"],
        ),
    }


def materialized_identity_silent_reuse_case():
    pid = (
        "packet:m4:harden_environment_capsule_resolution:"
        "materialized_identity_admits_silent_prebuild_reuse"
    )
    wid = (
        "workflow.runtime.harden_environment_capsule_resolution."
        "materialized_identity_admits_silent_prebuild_reuse"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if (
            row["row_class"] == "materialized_identity_admission"
            and row["lane_class"] == "devcontainer_lane"
        ):
            row["no_silent_prebuild_reuse"] = False
            break
    return {
        "record_kind":
            "harden_environment_capsule_resolution_truth_stable_case",
        "schema_version": 1,
        "case_name":
            "materialized_identity_admits_silent_prebuild_reuse_blocks_stable",
        "scenario": (
            "The devcontainer lane's materialized_identity_admission row "
            "drops the no_silent_prebuild_reuse attestation; the packet "
            "blocks the stable claim because a reused prebuild must always "
            "surface a visible invalidation reason — silent reuse is "
            "refused even when the requested and materialized identities "
            "match."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            2,
            expected_findings=[
                "materialized_identity_admission_admits_silent_prebuild_reuse",
                "missing_materialized_identity_admission",
            ],
        ),
    }


def narrowed_row_missing_disclosure_ref_case():
    pid = (
        "packet:m4:harden_environment_capsule_resolution:"
        "narrowed_row_missing_disclosure_ref"
    )
    wid = (
        "workflow.runtime.harden_environment_capsule_resolution."
        "narrowed_row_missing_disclosure_ref"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:devcontainer:quality":
            row["support_class"] = "launch_stable_below"
            row.pop("disclosure_ref", None)
            break
    return {
        "record_kind":
            "harden_environment_capsule_resolution_truth_stable_case",
        "schema_version": 1,
        "case_name": "narrowed_row_missing_disclosure_ref_blocks_stable",
        "scenario": (
            "The devcontainer lane's capsule_resolution_quality row "
            "narrows to launch_stable_below but drops its disclosure ref; "
            "the packet blocks the stable claim because every row narrowed "
            "below launch_stable must surface a disclosure ref so reviewers "
            "can see why the lane downgraded."
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


def projection_collapses_invalidation_reason_vocabulary_case():
    pid = (
        "packet:m4:harden_environment_capsule_resolution:"
        "projection_collapses_invalidation_reason_vocabulary"
    )
    wid = (
        "workflow.runtime.harden_environment_capsule_resolution."
        "projection_collapses_invalidation_reason_vocabulary"
    )
    inp = baseline_input(pid, wid)
    for proj in inp["consumer_projections"]:
        if proj["consumer_surface"] == "project_doctor":
            proj["preserves_invalidation_reason_vocabulary"] = False
            break
    return {
        "record_kind":
            "harden_environment_capsule_resolution_truth_stable_case",
        "schema_version": 1,
        "case_name":
            "projection_collapses_invalidation_reason_vocabulary_blocks_stable",
        "scenario": (
            "The project_doctor consumer projection drops the "
            "invalidation-reason vocabulary; the packet blocks the stable "
            "claim because a downstream surface that collapses the six "
            "invalidation reasons (cold_path, partially_warm_path, "
            "fingerprint_mismatch, untrusted_template_metadata, "
            "blocked_hook, stale_prebuild) cannot preserve "
            "capsule-resolution truth verbatim."
        ),
        "input": inp,
        "expect": expectations_from_input(
            inp,
            "blocks_stable",
            3,
            expected_findings=[
                "invalidation_reason_vocabulary_collapsed",
                "missing_consumer_projection",
                "consumer_projection_drift",
            ],
        ),
    }


def raw_source_material_case():
    pid = (
        "packet:m4:harden_environment_capsule_resolution:raw_source_material"
    )
    wid = (
        "workflow.runtime.harden_environment_capsule_resolution."
        "raw_source_material"
    )
    inp = baseline_input(pid, wid)
    for row in inp["rows"]:
        if row["row_id"] == "row:devcontainer:quality":
            row["raw_source_material_excluded"] = False
            break
    return {
        "record_kind":
            "harden_environment_capsule_resolution_truth_stable_case",
        "schema_version": 1,
        "case_name": "raw_source_material_blocks_stable",
        "scenario": (
            "The devcontainer lane's capsule_resolution_quality row admits "
            "raw command lines, raw process environment bytes, or raw "
            "capsule bodies past the boundary; the packet blocks the stable "
            "claim because raw runtime material, secrets, and ambient "
            "credentials must never leak through the capsule-resolution "
            "boundary."
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
        missing_fingerprint_component_case(),
        materialized_identity_silent_reuse_case(),
        narrowed_row_missing_disclosure_ref_case(),
        projection_collapses_invalidation_reason_vocabulary_case(),
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
            "# harden_environment_capsule_resolution fixture corpus\n\n"
            "Fixture corpus for the M4 stable capsule-resolution truth "
            "packet (`schemas/runtime/"
            "harden_environment_capsule_resolution_truth.schema.json`).\n\n"
            "Each fixture is a "
            "`CapsuleResolutionTruthPacketInput` with an `expect` block "
            "that pins the materialized packet's promotion state, finding "
            "count, lane and row-class token sets, support-class, "
            "capsule-field, prebuild-fingerprint, invalidation-reason, "
            "project-doctor-finding, known-limit, downgrade-automation, "
            "and evidence-class tokens, and the support-export safety "
            "verdict. Tests in `crates/aureline-runtime/tests/"
            "harden_environment_capsule_resolution_truth_packet.rs` "
            "load each case and assert that materialization matches the "
            "expectation block.\n\n"
            "Regenerate via:\n\n"
            "```bash\n"
            "python3 tools/"
            "regenerate_harden_environment_capsule_resolution_truth_packet.py\n"
            "```\n"
        )


if __name__ == "__main__":
    main()
