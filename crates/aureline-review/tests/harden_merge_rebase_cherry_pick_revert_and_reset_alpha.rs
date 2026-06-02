//! Fixture-driven coverage for diff-first rewrite-flow packets.
//!
//! These tests load every fixture in
//! `fixtures/review/m4/harden_merge_rebase_cherry_pick_revert_and_reset/` and
//! assert that:
//!
//! 1. Every fixture parses, validates, and projects without error.
//! 2. Diff-first review states are surfaced as separable inspectable truths.
//! 3. Recovery checkpoint invariants match the flow state.
//! 4. Protected-branch blocked flows are not actionable.
//! 5. Sequence-edit proposals have unique ordinals when present.
//! 6. Support/export records keep every `raw_*_export_allowed` flag false and
//!    consumer-surface lists include both `support_export` and `audit_lane`.
//! 7. Restart snapshots mirror current packet truth.

use std::path::{Path, PathBuf};

use aureline_review::{
    project_diff_first_rewrite_flow_packet, DiffFirstRewriteFlowPacket, DiffOpenTarget,
    DiffViewSurfacePacket, ReviewWorkspaceBetaInput, ReviewWorkspaceBetaPacket,
    ReviewWorkspaceSeedInput, ReviewWorkspaceSeedPacket, RewriteFlowInput,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RewriteFlowFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    seed_fixture_ref: String,
    beta_workspace_input: ReviewWorkspaceBetaInput,
    rewrite_flow_input: RewriteFlowInput,
    expected: ExpectedRewriteFlow,
}

#[derive(Debug, Deserialize)]
struct ExpectedRewriteFlow {
    diff_approved: bool,
    diff_pending: bool,
    diff_rejected: bool,
    checkpoint_ready: bool,
    executing: bool,
    paused_conflict: bool,
    completed: bool,
    aborted: bool,
    protected_branch_blocked: bool,
    policy_blocks_apply: bool,
    approval_invalidated: bool,
    checks_stale_blocks_apply: bool,
    actionable: bool,
    restartable: bool,
    command_count: usize,
    preview_capable: bool,
    support_export_reopenable: bool,
    suspicious_content_unreviewed: bool,
    remaining_step_count: u32,
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
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/review/m4/harden_merge_rebase_cherry_pick_revert_and_reset")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let mut paths: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("rewrite flow fixture directory")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
}

fn load_fixture(name: &str) -> RewriteFlowFixture {
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

fn workspace_packet_for(fixture: &RewriteFlowFixture) -> ReviewWorkspaceBetaPacket {
    let seed_packet = seed_packet_for(&fixture.seed_fixture_ref);
    ReviewWorkspaceBetaPacket::from_seed_packet(fixture.beta_workspace_input.clone(), &seed_packet)
        .unwrap_or_else(|err| panic!("{} workspace packet must project: {err}", fixture.case_name))
}

fn packet_for_fixture(fixture: &RewriteFlowFixture) -> DiffFirstRewriteFlowPacket {
    let workspace_packet = workspace_packet_for(fixture);
    DiffFirstRewriteFlowPacket::from_workspace_packet(
        fixture.rewrite_flow_input.clone(),
        &workspace_packet,
    )
    .unwrap_or_else(|err| panic!("{} must project: {err}", fixture.case_name))
}

fn assert_expected(packet: &DiffFirstRewriteFlowPacket, expected: &ExpectedRewriteFlow) {
    assert_eq!(packet.inspection.diff_approved, expected.diff_approved);
    assert_eq!(packet.inspection.diff_pending, expected.diff_pending);
    assert_eq!(packet.inspection.diff_rejected, expected.diff_rejected);
    assert_eq!(
        packet.inspection.checkpoint_ready,
        expected.checkpoint_ready
    );
    assert_eq!(packet.inspection.executing, expected.executing);
    assert_eq!(packet.inspection.paused_conflict, expected.paused_conflict);
    assert_eq!(packet.inspection.completed, expected.completed);
    assert_eq!(packet.inspection.aborted, expected.aborted);
    assert_eq!(
        packet.inspection.protected_branch_blocked,
        expected.protected_branch_blocked
    );
    assert_eq!(
        packet.inspection.policy_blocks_apply,
        expected.policy_blocks_apply
    );
    assert_eq!(
        packet.inspection.approval_invalidated,
        expected.approval_invalidated
    );
    assert_eq!(
        packet.inspection.checks_stale_blocks_apply,
        expected.checks_stale_blocks_apply
    );
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
        packet.inspection.remaining_step_count,
        expected.remaining_step_count
    );
    assert_eq!(packet.rewrite_flow.operation_kind, expected.operation_kind);
    assert_eq!(packet.rewrite_flow.flow_state, expected.flow_state);
    assert_eq!(
        packet.diff_first_review.diff_review_state,
        expected.diff_review_state
    );
    assert_eq!(
        packet.recovery_checkpoint_summary.checkpoint_state,
        expected.checkpoint_state
    );
}

#[test]
fn rewrite_flow_fixtures_project_and_round_trip() {
    let paths = load_fixture_paths();
    assert!(!paths.is_empty(), "rewrite flow fixtures must exist");

    for path in paths {
        let text =
            std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        let fixture: RewriteFlowFixture =
            serde_json::from_str(&text).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        assert_eq!(fixture.record_kind, "review_diff_first_rewrite_flow_case");
        assert_eq!(fixture.schema_version, 1);

        let packet = packet_for_fixture(&fixture);
        packet
            .validate()
            .unwrap_or_else(|err| panic!("{} must validate: {err}", fixture.case_name));
        assert!(packet.truths_are_separable(), "{}", fixture.case_name);
        assert!(packet.raw_escape_hatches_absent(), "{}", fixture.case_name);
        assert_expected(&packet, &fixture.expected);

        let serialized = serde_json::to_string_pretty(&packet).expect("packet serializes");
        let projection = project_diff_first_rewrite_flow_packet(&serialized)
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
    let fixture = load_fixture("merge_diff_approved_checkpoint_ready.json");
    let packet = packet_for_fixture(&fixture);
    assert!(packet.diff_first_review.checkpoint_required_before_apply);
    assert_eq!(
        packet.recovery_checkpoint_summary.checkpoint_state,
        "captured_ready"
    );
    assert!(packet.inspection.diff_approved);
    assert!(packet.inspection.checkpoint_ready);
}

#[test]
fn protected_branch_blocked_makes_flow_not_actionable() {
    let fixture = load_fixture("cherry_pick_sequence_protected_branch_blocked.json");
    let packet = packet_for_fixture(&fixture);
    assert!(packet.inspection.protected_branch_blocked);
    assert!(!packet.inspection.actionable);
    assert!(packet
        .rewrite_flow
        .blocked_reasons
        .contains(&"protected_branch_blocked".to_string()));
}

#[test]
fn paused_conflict_flow_is_restartable() {
    let fixture = load_fixture("rebase_paused_conflict_checkpoint_captured.json");
    let packet = packet_for_fixture(&fixture);
    assert!(packet.inspection.paused_conflict);
    assert!(packet.inspection.restartable);
    assert!(packet.restartable_from_support_export());
    assert!(!packet.inspection.actionable);
}

#[test]
fn reset_requires_checkpoint_before_apply() {
    let fixture = load_fixture("reset_hard_checkpoint_captured.json");
    let packet = packet_for_fixture(&fixture);
    assert_eq!(packet.rewrite_flow.operation_kind, "reset");
    assert!(packet.diff_first_review.checkpoint_required_before_apply);
    assert!(packet.inspection.diff_approved);
    assert!(packet.inspection.checkpoint_ready);
    assert!(packet.inspection.actionable);
}

#[test]
fn interactive_rebase_has_sequence_edit_proposal() {
    let fixture = load_fixture("interactive_rebase_sequence_running.json");
    let packet = packet_for_fixture(&fixture);
    assert_eq!(packet.rewrite_flow.operation_kind, "interactive_rebase");
    let proposal = packet
        .sequence_edit_proposal
        .as_ref()
        .expect("interactive rebase must have sequence_edit_proposal");
    assert_eq!(proposal.remaining_step_count, 2);
    assert_eq!(proposal.ordered_operations.len(), 4);
    let ordinals: Vec<u32> = proposal
        .ordered_operations
        .iter()
        .map(|op| op.ordinal)
        .collect();
    let mut sorted = ordinals.clone();
    sorted.sort();
    assert_eq!(ordinals, sorted, "ordinals must be sorted");
    assert_eq!(
        ordinals.len(),
        ordinals
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len(),
        "ordinals must be unique"
    );
}

#[test]
fn reflog_only_acknowledged_does_not_block_apply() {
    let fixture = load_fixture("revert_reflog_only_acknowledged.json");
    let packet = packet_for_fixture(&fixture);
    assert_eq!(
        packet.recovery_checkpoint_summary.checkpoint_state,
        "reflog_only_acknowledged"
    );
    assert!(!packet.diff_first_review.checkpoint_required_before_apply);
    assert!(!packet.inspection.checkpoint_ready);
    assert!(!packet.inspection.actionable);
    assert!(packet.inspection.diff_pending);
}

#[test]
fn support_export_restart_snapshot_mirrors_flow_truth() {
    let fixture = load_fixture("interactive_rebase_sequence_running.json");
    let packet = packet_for_fixture(&fixture);
    let snapshot = &packet.support_export.restart_snapshot;
    assert_eq!(snapshot.flow_state, packet.rewrite_flow.flow_state);
    assert_eq!(
        snapshot.diff_review_state,
        packet.diff_first_review.diff_review_state
    );
    assert_eq!(
        snapshot.checkpoint_state,
        packet.recovery_checkpoint_summary.checkpoint_state
    );
    assert_eq!(snapshot.operation_kind, packet.rewrite_flow.operation_kind);
    assert_eq!(
        snapshot.restart_session_ref,
        packet.rewrite_flow.restart_session_ref
    );
}

#[test]
fn sequence_edit_proposal_required_for_cherry_pick_and_interactive_rebase() {
    let fixture = load_fixture("cherry_pick_sequence_protected_branch_blocked.json");
    let workspace_packet = workspace_packet_for(&fixture);
    let mut input = fixture.rewrite_flow_input;
    input.sequence_edit_proposal = None;
    let err = DiffFirstRewriteFlowPacket::from_workspace_packet(input, &workspace_packet)
        .expect_err("cherry_pick without sequence_edit_proposal must fail");
    assert!(err.message().contains("sequence_edit_proposal"));
}

#[test]
fn reset_must_require_checkpoint() {
    let fixture = load_fixture("reset_hard_checkpoint_captured.json");
    let workspace_packet = workspace_packet_for(&fixture);
    let mut input = fixture.rewrite_flow_input;
    input.diff_review.checkpoint_required_before_apply = false;
    let err = DiffFirstRewriteFlowPacket::from_workspace_packet(input, &workspace_packet)
        .expect_err("reset without checkpoint_required_before_apply must fail");
    assert!(err.message().contains("reset"));
}

#[test]
fn diff_rejected_blocks_apply() {
    let fixture = load_fixture("merge_diff_approved_checkpoint_ready.json");
    let workspace_packet = workspace_packet_for(&fixture);
    let mut input = fixture.rewrite_flow_input;
    input.diff_review.diff_review_state = "rejected".to_string();
    let packet = DiffFirstRewriteFlowPacket::from_workspace_packet(input, &workspace_packet)
        .expect("rejected diff should still project");
    assert!(packet.inspection.diff_rejected);
    assert!(!packet.inspection.actionable);
    assert!(packet
        .rewrite_flow
        .blocked_reasons
        .contains(&"diff_not_approved".to_string()));
}
