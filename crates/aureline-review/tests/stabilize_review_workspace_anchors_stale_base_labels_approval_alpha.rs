//! Fixture-driven coverage for review-stabilization packets.
//!
//! These tests load every fixture in
//! `fixtures/review/m4/stabilize-review-workspace-anchors-stale-base-labels-approval/`
//! and assert that:
//!
//! 1. Every fixture parses, validates, and projects without error.
//! 2. Stabilization-truth axes are surfaced as separable inspectable truths.
//! 3. Anchor stability records bind to the exact review-pack digest and base/head identity.
//! 4. Stale-pack, partial-scope, and slice-omitted states do not inherit green from adjacent provider rows.
//! 5. Approval invalidation carries replayable evidence refs and classes.
//! 6. Ownership signals remain split between advisory and enforceable classes.
//! 7. Bundle export and offline handoff preserve review-pack version, divergence labels, and replayable evidence.
//! 8. Support/export records keep every `raw_*_export_allowed` flag false and consumer-surface lists include both `support_export` and `audit_lane`.

use std::path::{Path, PathBuf};

use aureline_review::{
    DiffFileInput, DiffOpenTarget, DiffViewSurfacePacket, LandingCandidateInput,
    LandingCandidatePacket, ReviewStabilizationInput, ReviewStabilizationPacket,
    ReviewWorkspaceBetaInput, ReviewWorkspaceBetaPacket, ReviewWorkspaceSeedInput,
    ReviewWorkspaceSeedPacket,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct StabilizationFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    seed_fixture_ref: String,
    beta_workspace_input: ReviewWorkspaceBetaInput,
    landing_input: LandingCandidateInput,
    stabilization_input: ReviewStabilizationInput,
    expected: ExpectedStabilization,
}

#[derive(Debug, Deserialize)]
struct ExpectedStabilization {
    stabilized_current: bool,
    stabilized_stale_pack: bool,
    stabilized_partial_scope: bool,
    stabilized_slice_omitted: bool,
    stabilized_diverged_requires_review: bool,
    all_anchors_bound_exact: bool,
    any_anchor_drifted: bool,
    stale_base_blocks_landing: bool,
    approval_invalidated: bool,
    mergeability_blocking: bool,
    mergeability_provider_authoritative: bool,
    enforceable_ownership_present: bool,
    advisory_ownership_present: bool,
    ownership_conflict_present: bool,
    bundle_export_present: bool,
    bundle_import_present: bool,
    offline_handoff_present: bool,
    actionable: bool,
    invalidated: bool,
    command_count: usize,
    anchor_stability_count: usize,
    ownership_signal_count: usize,
    preview_capable: bool,
    support_export_reopenable: bool,
}

#[derive(Debug, Deserialize)]
struct ReviewWorkspaceSeedFixture {
    change_list_row: ChangeListRowFixture,
    workspace_seed: ReviewWorkspaceSeedInput,
    diff: DiffFileInput,
}

#[derive(Debug, Deserialize)]
struct ChangeListRowFixture {
    row_ref: String,
    file_state_token: String,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../fixtures/review/m4/stabilize-review-workspace-anchors-stale-base-labels-approval",
    )
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let mut paths: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("stabilization fixture directory")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
}

fn load_fixture(name: &str) -> StabilizationFixture {
    let path = fixtures_dir().join(name);
    let text =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
    serde_json::from_str(&text).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"))
}

fn seed_packet_for(seed_fixture_ref: &str) -> ReviewWorkspaceSeedPacket {
    let path = repo_root().join(seed_fixture_ref);
    let text =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("seed fixture {path:?}: {err}"));
    let fixture: ReviewWorkspaceSeedFixture =
        serde_yaml::from_str(&text).unwrap_or_else(|err| panic!("seed fixture {path:?}: {err}"));
    let open_target = DiffOpenTarget::from_change_list_row_parts(
        &fixture.diff.workspace_ref,
        &fixture.diff.truth_source_ref,
        &fixture.change_list_row.row_ref,
        &fixture.diff.group_token,
        fixture.diff.path.clone(),
        fixture.diff.original_path.clone(),
        &fixture.diff.status_code,
        &fixture.change_list_row.file_state_token,
    );
    let diff_packet = DiffViewSurfacePacket::from_file_input(open_target, fixture.diff);
    ReviewWorkspaceSeedPacket::from_diff_packet(fixture.workspace_seed, &diff_packet)
}

fn workspace_packet_for(fixture: &StabilizationFixture) -> ReviewWorkspaceBetaPacket {
    let seed_packet = seed_packet_for(&fixture.seed_fixture_ref);
    ReviewWorkspaceBetaPacket::from_seed_packet(fixture.beta_workspace_input.clone(), &seed_packet)
        .unwrap_or_else(|err| panic!("{} workspace packet must project: {err}", fixture.case_name))
}

fn landing_packet_for(fixture: &StabilizationFixture) -> LandingCandidatePacket {
    let workspace_packet = workspace_packet_for(fixture);
    LandingCandidatePacket::from_workspace_packet(fixture.landing_input.clone(), &workspace_packet)
        .unwrap_or_else(|err| panic!("{} landing packet must project: {err}", fixture.case_name))
}

fn stabilization_packet_for(fixture: &StabilizationFixture) -> ReviewStabilizationPacket {
    let workspace_packet = workspace_packet_for(fixture);
    let landing_packet = landing_packet_for(fixture);
    ReviewStabilizationPacket::from_workspace_and_landing_packets(
        fixture.stabilization_input.clone(),
        &workspace_packet,
        &landing_packet,
    )
    .unwrap_or_else(|err| panic!("{} must project: {err}", fixture.case_name))
}

fn assert_expected(
    packet: &ReviewStabilizationPacket,
    expected: &ExpectedStabilization,
    case_name: &str,
) {
    assert_eq!(
        packet.inspection.stabilized_current, expected.stabilized_current,
        "{case_name}: stabilized_current"
    );
    assert_eq!(
        packet.inspection.stabilized_stale_pack, expected.stabilized_stale_pack,
        "{case_name}: stabilized_stale_pack"
    );
    assert_eq!(
        packet.inspection.stabilized_partial_scope, expected.stabilized_partial_scope,
        "{case_name}: stabilized_partial_scope"
    );
    assert_eq!(
        packet.inspection.stabilized_slice_omitted, expected.stabilized_slice_omitted,
        "{case_name}: stabilized_slice_omitted"
    );
    assert_eq!(
        packet.inspection.stabilized_diverged_requires_review,
        expected.stabilized_diverged_requires_review,
        "{case_name}: stabilized_diverged_requires_review"
    );
    assert_eq!(
        packet.inspection.all_anchors_bound_exact, expected.all_anchors_bound_exact,
        "{case_name}: all_anchors_bound_exact"
    );
    assert_eq!(
        packet.inspection.any_anchor_drifted, expected.any_anchor_drifted,
        "{case_name}: any_anchor_drifted"
    );
    assert_eq!(
        packet.inspection.stale_base_blocks_landing, expected.stale_base_blocks_landing,
        "{case_name}: stale_base_blocks_landing"
    );
    assert_eq!(
        packet.inspection.approval_invalidated, expected.approval_invalidated,
        "{case_name}: approval_invalidated"
    );
    assert_eq!(
        packet.inspection.mergeability_blocking, expected.mergeability_blocking,
        "{case_name}: mergeability_blocking"
    );
    assert_eq!(
        packet.inspection.mergeability_provider_authoritative,
        expected.mergeability_provider_authoritative,
        "{case_name}: mergeability_provider_authoritative"
    );
    assert_eq!(
        packet.inspection.enforceable_ownership_present, expected.enforceable_ownership_present,
        "{case_name}: enforceable_ownership_present"
    );
    assert_eq!(
        packet.inspection.advisory_ownership_present, expected.advisory_ownership_present,
        "{case_name}: advisory_ownership_present"
    );
    assert_eq!(
        packet.inspection.ownership_conflict_present, expected.ownership_conflict_present,
        "{case_name}: ownership_conflict_present"
    );
    assert_eq!(
        packet.inspection.bundle_export_present, expected.bundle_export_present,
        "{case_name}: bundle_export_present"
    );
    assert_eq!(
        packet.inspection.bundle_import_present, expected.bundle_import_present,
        "{case_name}: bundle_import_present"
    );
    assert_eq!(
        packet.inspection.offline_handoff_present, expected.offline_handoff_present,
        "{case_name}: offline_handoff_present"
    );
    assert_eq!(
        packet.inspection.actionable, expected.actionable,
        "{case_name}: actionable"
    );
    assert_eq!(
        packet.inspection.invalidated, expected.invalidated,
        "{case_name}: invalidated"
    );
    assert_eq!(
        packet.inspection.command_count, expected.command_count,
        "{case_name}: command_count"
    );
    assert_eq!(
        packet.inspection.anchor_stability_count, expected.anchor_stability_count,
        "{case_name}: anchor_stability_count"
    );
    assert_eq!(
        packet.inspection.ownership_signal_count, expected.ownership_signal_count,
        "{case_name}: ownership_signal_count"
    );
    assert_eq!(
        packet.inspection.preview_capable, expected.preview_capable,
        "{case_name}: preview_capable"
    );
    assert_eq!(
        packet.inspection.support_export_reopenable, expected.support_export_reopenable,
        "{case_name}: support_export_reopenable"
    );
}

#[test]
fn stabilization_fixtures_project_and_round_trip() {
    let paths = load_fixture_paths();
    assert!(!paths.is_empty(), "stabilization fixtures must exist");

    for path in paths {
        let text =
            std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        let fixture: StabilizationFixture =
            serde_json::from_str(&text).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        assert_eq!(fixture.record_kind, "review_stabilization_case");
        assert_eq!(fixture.schema_version, 1);

        let packet = stabilization_packet_for(&fixture);
        packet
            .validate()
            .unwrap_or_else(|err| panic!("{} must validate: {err}", fixture.case_name));
        assert!(packet.truths_are_separable(), "{}", fixture.case_name);
        assert!(packet.raw_escape_hatches_absent(), "{}", fixture.case_name);
        assert!(
            packet.ownership_signals_properly_split(),
            "{}",
            fixture.case_name
        );

        assert_expected(&packet, &fixture.expected, &fixture.case_name);

        // Round-trip through JSON and re-validate.
        let json = serde_json::to_string(&packet).expect("serialization must succeed");
        let reparsed: ReviewStabilizationPacket =
            serde_json::from_str(&json).expect("re-deserialization must succeed");
        reparsed
            .validate()
            .unwrap_or_else(|err| panic!("{} round-trip must validate: {err}", fixture.case_name));
        assert_expected(&reparsed, &fixture.expected, &fixture.case_name);
    }
}

#[test]
fn stale_pack_does_not_inherit_green_from_provider() {
    let fixture = load_fixture("stabilized_stale_pack.json");
    let packet = stabilization_packet_for(&fixture);

    assert!(
        !packet.inspection.all_anchors_bound_exact,
        "stale pack must not claim exact anchor binding"
    );
    assert!(
        packet.inspection.stabilized_stale_pack,
        "stale pack must be explicitly flagged"
    );
    assert!(
        !packet.inspection.stabilized_current,
        "stale pack must not inherit current status"
    );
}

#[test]
fn approval_invalidation_preserves_replay_evidence() {
    let fixture = load_fixture("approval_invalidated_with_replay_evidence.json");
    let packet = stabilization_packet_for(&fixture);

    let invalidation = packet
        .approval_invalidation
        .expect("approval invalidation must be present");
    assert_eq!(invalidation.replay_evidence_refs.len(), 2);
    assert_eq!(invalidation.replay_evidence_classes.len(), 2);
    assert!(invalidation
        .replay_evidence_classes
        .contains(&"local_ci_evidence".to_string()));
    assert!(invalidation
        .replay_evidence_classes
        .contains(&"ai_review_evidence".to_string()));
}

#[test]
fn ownership_conflict_detects_advisory_enforceable_mismatch() {
    let fixture = load_fixture("ownership_conflict_with_offline_handoff.json");
    let packet = stabilization_packet_for(&fixture);

    assert!(packet.inspection.ownership_conflict_present);
    assert!(packet.inspection.enforceable_ownership_present);
    assert!(packet.inspection.advisory_ownership_present);
    assert!(packet.inspection.bundle_export_present);
    assert!(packet.inspection.offline_handoff_present);

    let export = packet.bundle_export.expect("bundle export must be present");
    assert_eq!(export.review_pack_version, "review.pack.version.1.0.0");
    assert!(!export.divergence_labels.is_empty());
    assert!(!export.replay_evidence_refs.is_empty());

    let handoff = packet
        .offline_handoff
        .expect("offline handoff must be present");
    assert_eq!(handoff.review_pack_version, "review.pack.version.1.0.0");
    assert!(!handoff.divergence_labels.is_empty());
    assert!(!handoff.replay_evidence_refs.is_empty());
}

#[test]
fn base_head_mismatch_rejects_stabilization() {
    let mut fixture = load_fixture("stabilized_current.json");
    fixture.stabilization_input.base_revision_ref = "git.rev.mismatch".to_string();

    let workspace_packet = workspace_packet_for(&fixture);
    let landing_packet = landing_packet_for(&fixture);
    let result = ReviewStabilizationPacket::from_workspace_and_landing_packets(
        fixture.stabilization_input,
        &workspace_packet,
        &landing_packet,
    );

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message().contains("base_revision_ref must match"));
}
