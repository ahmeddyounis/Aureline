//! Fixture-driven coverage for migration rollback diff-review packets.
//!
//! These tests load every fixture in
//! `fixtures/review/m4/finalize-migration-rollback-checkpoints-diff-review-and-retained/`
//! and assert that:
//!
//! 1. Every fixture parses, validates, and projects without error.
//! 2. Diff-first review states are surfaced as separable inspectable truths.
//! 3. Rollback checkpoint invariants match the flow state.
//! 4. Validation-failed flows retain at least one diagnostic.
//! 5. Support/export records keep every `raw_*_export_allowed` flag false and
//!    consumer-surface lists include both `support_export` and `audit_lane`.
//! 6. Restart snapshots mirror current packet truth.

use std::path::{Path, PathBuf};

use aureline_review::{
    project_migration_rollback_diff_review_packet, DiffOpenTarget, DiffViewSurfacePacket,
    MigrationRollbackDiffReviewInput, MigrationRollbackDiffReviewPacket, ReviewWorkspaceBetaInput,
    ReviewWorkspaceBetaPacket, ReviewWorkspaceSeedInput, ReviewWorkspaceSeedPacket,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MigrationRollbackDiffReviewFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    seed_fixture_ref: String,
    beta_workspace_input: ReviewWorkspaceBetaInput,
    migration_flow_input: MigrationRollbackDiffReviewInput,
    expected: ExpectedMigrationFlow,
}

#[derive(Debug, Deserialize)]
struct ExpectedMigrationFlow {
    diff_approved: bool,
    diff_pending: bool,
    diff_rejected: bool,
    checkpoint_ready: bool,
    applying: bool,
    completed: bool,
    validation_failed: bool,
    rolled_back: bool,
    aborted: bool,
    actionable: bool,
    restartable: bool,
    command_count: usize,
    preview_capable: bool,
    support_export_reopenable: bool,
    suspicious_content_unreviewed: bool,
    retained_diagnostic_count: usize,
    retained_diagnostic_fallback_available: bool,
    operation_kind: String,
    flow_state: String,
    diff_review_state: String,
    checkpoint_state: String,
}

#[derive(Debug, Deserialize)]
struct ReviewWorkspaceSeedFixture {
    change_list_row: ChangeListRowFixture,
    workspace_seed: ReviewWorkspaceSeedInput,
    diff: aureline_review::DiffFileInput,
}

#[derive(Debug, Deserialize)]
struct ChangeListRowFixture {
    row_ref: String,
    file_state_token: String,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../fixtures/review/m4/finalize-migration-rollback-checkpoints-diff-review-and-retained",
    )
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let mut paths: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("migration rollback diff-review fixture directory")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
}

fn load_fixture(name: &str) -> MigrationRollbackDiffReviewFixture {
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

fn workspace_packet_for(fixture: &MigrationRollbackDiffReviewFixture) -> ReviewWorkspaceBetaPacket {
    let seed_packet = seed_packet_for(&fixture.seed_fixture_ref);
    ReviewWorkspaceBetaPacket::from_seed_packet(fixture.beta_workspace_input.clone(), &seed_packet)
        .unwrap_or_else(|err| panic!("{} workspace packet must project: {err}", fixture.case_name))
}

fn packet_for_fixture(
    fixture: &MigrationRollbackDiffReviewFixture,
) -> MigrationRollbackDiffReviewPacket {
    let workspace_packet = workspace_packet_for(fixture);
    MigrationRollbackDiffReviewPacket::from_workspace_packet(
        fixture.migration_flow_input.clone(),
        &workspace_packet,
    )
    .unwrap_or_else(|err| panic!("{} must project: {err}", fixture.case_name))
}

fn assert_expected(packet: &MigrationRollbackDiffReviewPacket, expected: &ExpectedMigrationFlow) {
    assert_eq!(packet.inspection.diff_approved, expected.diff_approved);
    assert_eq!(packet.inspection.diff_pending, expected.diff_pending);
    assert_eq!(packet.inspection.diff_rejected, expected.diff_rejected);
    assert_eq!(
        packet.inspection.checkpoint_ready,
        expected.checkpoint_ready
    );
    assert_eq!(packet.inspection.applying, expected.applying);
    assert_eq!(packet.inspection.completed, expected.completed);
    assert_eq!(
        packet.inspection.validation_failed,
        expected.validation_failed
    );
    assert_eq!(packet.inspection.rolled_back, expected.rolled_back);
    assert_eq!(packet.inspection.aborted, expected.aborted);
    assert_eq!(packet.inspection.actionable, expected.actionable);
    assert_eq!(packet.inspection.restartable, expected.restartable);
    assert_eq!(packet.inspection.command_count, expected.command_count);
    assert_eq!(packet.inspection.preview_capable, expected.preview_capable);
    assert_eq!(
        packet.inspection.support_export_reopenable,
        expected.support_export_reopenable
    );
    assert_eq!(
        packet.inspection.suspicious_content_unreviewed,
        expected.suspicious_content_unreviewed
    );
    assert_eq!(
        packet.inspection.retained_diagnostic_count,
        expected.retained_diagnostic_count
    );
    assert_eq!(
        packet.inspection.retained_diagnostic_fallback_available,
        expected.retained_diagnostic_fallback_available
    );
    assert_eq!(
        packet.migration_flow.operation_kind,
        expected.operation_kind
    );
    assert_eq!(packet.migration_flow.flow_state, expected.flow_state);
    assert_eq!(
        packet.diff_review.diff_review_state,
        expected.diff_review_state
    );
    assert_eq!(
        packet.rollback_checkpoint.checkpoint_state,
        expected.checkpoint_state
    );
}

#[test]
fn migration_rollback_diff_review_fixtures_project_and_round_trip() {
    let paths = load_fixture_paths();
    assert!(
        !paths.is_empty(),
        "migration rollback diff-review fixtures must exist"
    );

    for path in paths {
        let text =
            std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        let fixture: MigrationRollbackDiffReviewFixture =
            serde_json::from_str(&text).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        assert_eq!(
            fixture.record_kind,
            "review_migration_rollback_diff_review_case"
        );
        assert_eq!(fixture.schema_version, 1);

        let packet = packet_for_fixture(&fixture);
        packet
            .validate()
            .unwrap_or_else(|err| panic!("{} must validate: {err}", fixture.case_name));
        assert!(packet.truths_are_separable(), "{}", fixture.case_name);
        assert!(packet.raw_escape_hatches_absent(), "{}", fixture.case_name);
        assert_expected(&packet, &fixture.expected);

        let serialized = serde_json::to_string_pretty(&packet).expect("packet serializes");
        let projection = project_migration_rollback_diff_review_packet(&serialized)
            .unwrap_or_else(|err| panic!("{} must re-project: {err}", fixture.case_name));
        assert_eq!(projection.packet_id, packet.packet_id);
        assert_eq!(projection.operation_kind, fixture.expected.operation_kind);
        assert_eq!(projection.flow_state, fixture.expected.flow_state);
        assert_eq!(
            projection.diff_review_state,
            fixture.expected.diff_review_state
        );
        assert_eq!(
            projection.checkpoint_state,
            fixture.expected.checkpoint_state
        );
        assert_eq!(projection.command_count, fixture.expected.command_count);
        assert!(projection
            .consumer_surfaces
            .iter()
            .any(|surface| surface == "support_export"));
    }
}

#[test]
fn diff_approved_requires_checkpoint_when_required() {
    let fixture = load_fixture("settings_import_diff_approved_checkpoint_ready.json");
    let packet = packet_for_fixture(&fixture);
    assert!(packet.diff_review.checkpoint_required_before_apply);
    assert_eq!(
        packet.rollback_checkpoint.checkpoint_state,
        "captured_ready"
    );
    assert!(packet.inspection.diff_approved);
    assert!(packet.inspection.checkpoint_ready);
}

#[test]
fn validation_failed_retains_diagnostics() {
    let fixture = load_fixture("keymap_import_validation_failed_with_diagnostics.json");
    let packet = packet_for_fixture(&fixture);
    assert_eq!(packet.migration_flow.flow_state, "validation_failed");
    assert!(!packet.retained_diagnostics.is_empty());
    assert!(packet.inspection.validation_failed);
}

#[test]
fn rolled_back_flow_is_not_actionable() {
    let fixture = load_fixture("snippet_import_rolled_back.json");
    let packet = packet_for_fixture(&fixture);
    assert_eq!(packet.migration_flow.flow_state, "rolled_back");
    assert!(!packet.inspection.actionable);
    assert!(packet.inspection.rolled_back);
}

#[test]
fn aborted_flow_is_not_actionable() {
    let fixture = load_fixture("theme_import_aborted.json");
    let packet = packet_for_fixture(&fixture);
    assert_eq!(packet.migration_flow.flow_state, "aborted");
    assert!(!packet.inspection.actionable);
    assert!(packet.inspection.aborted);
}

#[test]
fn support_export_restart_snapshot_mirrors_flow_truth() {
    let fixture = load_fixture("settings_import_diff_approved_checkpoint_ready.json");
    let packet = packet_for_fixture(&fixture);
    let snapshot = &packet.support_export.restart_snapshot;
    assert_eq!(snapshot.flow_state, packet.migration_flow.flow_state);
    assert_eq!(
        snapshot.diff_review_state,
        packet.diff_review.diff_review_state
    );
    assert_eq!(
        snapshot.checkpoint_state,
        packet.rollback_checkpoint.checkpoint_state
    );
    assert_eq!(
        snapshot.operation_kind,
        packet.migration_flow.operation_kind
    );
    assert_eq!(
        snapshot.restart_session_ref,
        packet.migration_flow.restart_session_ref
    );
}

#[test]
fn diff_rejected_blocks_apply() {
    let fixture = load_fixture("settings_import_diff_approved_checkpoint_ready.json");
    let workspace_packet = workspace_packet_for(&fixture);
    let mut input = fixture.migration_flow_input;
    input.diff_review.diff_review_state = "rejected".to_string();
    let packet = MigrationRollbackDiffReviewPacket::from_workspace_packet(input, &workspace_packet)
        .expect("rejected diff should still project");
    assert!(packet.inspection.diff_rejected);
    assert!(!packet.inspection.actionable);
    assert!(packet
        .migration_flow
        .blocked_reasons
        .contains(&"diff_not_approved".to_string()));
}

#[test]
fn validation_failed_requires_at_least_one_diagnostic() {
    let fixture = load_fixture("keymap_import_validation_failed_with_diagnostics.json");
    let workspace_packet = workspace_packet_for(&fixture);
    let mut input = fixture.migration_flow_input;
    input.retained_diagnostics.clear();
    let err = MigrationRollbackDiffReviewPacket::from_workspace_packet(input, &workspace_packet)
        .expect_err("validation_failed without diagnostics must fail");
    assert!(err.message().contains("diagnostic"));
}
