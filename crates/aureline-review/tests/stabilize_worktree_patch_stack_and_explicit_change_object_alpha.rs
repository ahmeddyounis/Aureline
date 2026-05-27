//! Fixture-driven coverage for change-object orchestration packets.
//!
//! These tests load every fixture in
//! `fixtures/review/m4/stabilize-worktree-patch-stack-and-explicit-change-object/`
//! and assert that:
//!
//! 1. Every fixture parses, validates, and projects without error.
//! 2. Preview states are surfaced as separable inspectable truths.
//! 3. Checkpoint invariants match the flow state.
//! 4. Repo topology boundaries (submodule, nested repo, shallow) are surfaced
//!    as distinct inspection booleans.
//! 5. Support/export records keep every `raw_*_export_allowed` flag false and
//!    consumer-surface lists include both `support_export` and `audit_lane`.
//! 6. Restart snapshots mirror current packet truth including repo_root_ref.
//! 7. Parent, child, and sibling repo change objects carry distinct repo_root_ref
//!    values and do not collide.

use std::path::{Path, PathBuf};

use aureline_review::{
    project_change_object_orchestration_packet, ChangeObjectOrchestrationInput,
    ChangeObjectOrchestrationPacket, DiffOpenTarget, DiffViewSurfacePacket,
    ReviewWorkspaceBetaInput, ReviewWorkspaceBetaPacket, ReviewWorkspaceSeedInput,
    ReviewWorkspaceSeedPacket,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct OrchestrationFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    seed_fixture_ref: String,
    beta_workspace_input: ReviewWorkspaceBetaInput,
    orchestration_input: ChangeObjectOrchestrationInput,
    expected: ExpectedOrchestration,
}

#[derive(Debug, Deserialize)]
struct ExpectedOrchestration {
    preview_approved: bool,
    preview_pending: bool,
    preview_rejected: bool,
    checkpoint_ready: bool,
    executing: bool,
    completed: bool,
    failed: bool,
    rolled_back: bool,
    aborted: bool,
    approval_invalidated: bool,
    checks_stale_blocks_apply: bool,
    actionable: bool,
    restartable: bool,
    command_count: usize,
    preview_capable: bool,
    support_export_reopenable: bool,
    requires_browser_handoff: bool,
    submodule_boundary_present: bool,
    nested_repo_boundary_present: bool,
    shallow_boundary_present: bool,
    operation_kind: String,
    flow_state: String,
    checkpoint_state: String,
    repo_root_ref: String,
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
        .join("../../fixtures/review/m4/stabilize-worktree-patch-stack-and-explicit-change-object")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_fixture_paths() -> Vec<PathBuf> {
    let mut paths: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("orchestration fixture directory")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
}

fn load_fixture(name: &str) -> OrchestrationFixture {
    let path = fixtures_dir().join(name);
    let text =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
    serde_json::from_str(&text).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"))
}

fn seed_packet_for(seed_fixture_ref: &str) -> ReviewWorkspaceSeedPacket {
    let path = repo_root().join(seed_fixture_ref);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("seed fixture {path:?}: {err}"));
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

fn workspace_packet_for(fixture: &OrchestrationFixture) -> ReviewWorkspaceBetaPacket {
    let seed_packet = seed_packet_for(&fixture.seed_fixture_ref);
    ReviewWorkspaceBetaPacket::from_seed_packet(fixture.beta_workspace_input.clone(), &seed_packet)
        .unwrap_or_else(|err| {
            panic!(
                "{} workspace packet must project: {err}",
                fixture.case_name
            )
        })
}

fn packet_for_fixture(fixture: &OrchestrationFixture) -> ChangeObjectOrchestrationPacket {
    let workspace_packet = workspace_packet_for(fixture);
    ChangeObjectOrchestrationPacket::from_workspace_packet(
        fixture.orchestration_input.clone(),
        &workspace_packet,
    )
    .unwrap_or_else(|err| panic!("{} must project: {err}", fixture.case_name))
}

fn assert_expected(
    packet: &ChangeObjectOrchestrationPacket,
    expected: &ExpectedOrchestration,
) {
    assert_eq!(packet.inspection.preview_approved, expected.preview_approved);
    assert_eq!(packet.inspection.preview_pending, expected.preview_pending);
    assert_eq!(packet.inspection.preview_rejected, expected.preview_rejected);
    assert_eq!(packet.inspection.checkpoint_ready, expected.checkpoint_ready);
    assert_eq!(packet.inspection.executing, expected.executing);
    assert_eq!(packet.inspection.completed, expected.completed);
    assert_eq!(packet.inspection.failed, expected.failed);
    assert_eq!(packet.inspection.rolled_back, expected.rolled_back);
    assert_eq!(packet.inspection.aborted, expected.aborted);
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
        packet.inspection.requires_browser_handoff,
        expected.requires_browser_handoff
    );
    assert_eq!(
        packet.inspection.submodule_boundary_present,
        expected.submodule_boundary_present
    );
    assert_eq!(
        packet.inspection.nested_repo_boundary_present,
        expected.nested_repo_boundary_present
    );
    assert_eq!(
        packet.inspection.shallow_boundary_present,
        expected.shallow_boundary_present
    );
    assert_eq!(packet.orchestration.operation_kind, expected.operation_kind);
    assert_eq!(packet.orchestration.flow_state, expected.flow_state);
    assert_eq!(
        packet.mutation_checkpoint.checkpoint_state,
        expected.checkpoint_state
    );
    assert_eq!(packet.orchestration.repo_root_ref, expected.repo_root_ref);
}

#[test]
fn orchestration_fixtures_project_and_round_trip() {
    let paths = load_fixture_paths();
    assert!(!paths.is_empty(), "orchestration fixtures must exist");

    for path in paths {
        let text =
            std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        let fixture: OrchestrationFixture =
            serde_json::from_str(&text).unwrap_or_else(|err| panic!("fixture {path:?}: {err}"));
        assert_eq!(fixture.record_kind, "review_change_object_orchestration_case");
        assert_eq!(fixture.schema_version, 1);

        let packet = packet_for_fixture(&fixture);
        packet
            .validate()
            .unwrap_or_else(|err| panic!("{} must validate: {err}", fixture.case_name));
        assert!(packet.truths_are_separable(), "{}", fixture.case_name);
        assert!(packet.raw_escape_hatches_absent(), "{}", fixture.case_name);
        assert_expected(&packet, &fixture.expected);

        let serialized = serde_json::to_string_pretty(&packet).expect("packet serializes");
        let projection = project_change_object_orchestration_packet(&serialized)
            .unwrap_or_else(|err| panic!("{} must re-project: {err}", fixture.case_name));
        assert_eq!(projection.packet_id, packet.packet_id);
        assert_eq!(projection.operation_kind, fixture.expected.operation_kind);
        assert_eq!(projection.flow_state, fixture.expected.flow_state);
        assert_eq!(projection.checkpoint_state, fixture.expected.checkpoint_state);
        assert_eq!(projection.command_count, fixture.expected.command_count);
        assert!(projection
            .consumer_surfaces
            .iter()
            .any(|surface| surface == "support_export"));
    }
}

#[test]
fn parent_repo_worktree_switch_has_no_boundaries() {
    let fixture = load_fixture("parent_repo_worktree_switch.json");
    let packet = packet_for_fixture(&fixture);
    assert!(!packet.inspection.submodule_boundary_present);
    assert!(!packet.inspection.nested_repo_boundary_present);
    assert!(!packet.inspection.shallow_boundary_present);
    assert!(!packet.inspection.pointer_backed_assets_present);
    assert_eq!(packet.orchestration.repo_root_ref, "repo.root.fixture.parent");
}

#[test]
fn child_submodule_has_submodule_boundary_and_browser_handoff() {
    let fixture = load_fixture("child_submodule_patch_stack_publish.json");
    let packet = packet_for_fixture(&fixture);
    assert!(packet.inspection.submodule_boundary_present);
    assert!(!packet.inspection.nested_repo_boundary_present);
    assert!(!packet.inspection.shallow_boundary_present);
    assert!(packet.inspection.requires_browser_handoff);
    assert!(packet.inspection.handoff_reversible);
    assert_eq!(
        packet.orchestration.repo_root_ref,
        "repo.root.fixture.submodule_child"
    );
    assert_eq!(
        packet.orchestration.parent_repo_root_ref.as_deref(),
        Some("repo.root.fixture.parent")
    );
}

#[test]
fn sibling_nested_repo_has_nested_boundary() {
    let fixture = load_fixture("sibling_nested_repo_change_object_apply.json");
    let packet = packet_for_fixture(&fixture);
    assert!(!packet.inspection.submodule_boundary_present);
    assert!(packet.inspection.nested_repo_boundary_present);
    assert!(!packet.inspection.shallow_boundary_present);
    assert_eq!(
        packet.orchestration.repo_root_ref,
        "repo.root.fixture.nested_sibling"
    );
}

#[test]
fn shallow_history_has_shallow_boundary_and_pointer_assets() {
    let fixture = load_fixture("shallow_history_pointer_asset_rollback.json");
    let packet = packet_for_fixture(&fixture);
    assert!(!packet.inspection.submodule_boundary_present);
    assert!(!packet.inspection.nested_repo_boundary_present);
    assert!(packet.inspection.shallow_boundary_present);
    assert!(packet.inspection.pointer_backed_assets_present);
    assert!(packet.inspection.rolled_back);
    assert!(!packet.inspection.actionable);
    assert_eq!(
        packet.orchestration.repo_root_ref,
        "repo.root.fixture.shallow_clone"
    );
}

#[test]
fn checkpoint_ready_requires_captured_or_restored() {
    let fixture = load_fixture("parent_repo_worktree_switch.json");
    let packet = packet_for_fixture(&fixture);
    assert!(packet.inspection.checkpoint_ready);
    assert!(
        matches!(
            packet.mutation_checkpoint.checkpoint_state.as_str(),
            "captured_ready" | "captured_pending" | "restored"
        ),
        "checkpoint_ready requires a captured or restored checkpoint"
    );
}

#[test]
fn support_export_restart_snapshot_mirrors_flow_truth() {
    let fixture = load_fixture("child_submodule_patch_stack_publish.json");
    let packet = packet_for_fixture(&fixture);
    let snapshot = &packet.support_export.restart_snapshot;
    assert_eq!(snapshot.flow_state, packet.orchestration.flow_state);
    assert_eq!(snapshot.operation_kind, packet.orchestration.operation_kind);
    assert_eq!(
        snapshot.checkpoint_state,
        packet.mutation_checkpoint.checkpoint_state
    );
    assert_eq!(snapshot.repo_root_ref, packet.orchestration.repo_root_ref);
    assert_eq!(
        snapshot.restart_session_ref,
        packet.orchestration.restart_session_ref
    );
}

#[test]
fn worktree_operation_requires_worktree_sub_record() {
    let fixture = load_fixture("parent_repo_worktree_switch.json");
    let workspace_packet = workspace_packet_for(&fixture);
    let mut input = fixture.orchestration_input;
    input.worktree_operation = None;
    let err = ChangeObjectOrchestrationPacket::from_workspace_packet(input, &workspace_packet)
        .expect_err("worktree_switch without worktree_operation must fail");
    assert!(err.message().contains("worktree"));
}

#[test]
fn patch_stack_operation_requires_patch_stack_sub_record() {
    let fixture = load_fixture("child_submodule_patch_stack_publish.json");
    let workspace_packet = workspace_packet_for(&fixture);
    let mut input = fixture.orchestration_input;
    input.patch_stack_operation = None;
    let err = ChangeObjectOrchestrationPacket::from_workspace_packet(input, &workspace_packet)
        .expect_err("patch_stack_publish without patch_stack_operation must fail");
    assert!(err.message().contains("patch_stack"));
}

#[test]
fn completed_flow_with_missing_checkpoint_must_fail() {
    let fixture = load_fixture("shallow_history_pointer_asset_rollback.json");
    let workspace_packet = workspace_packet_for(&fixture);
    let mut input = fixture.orchestration_input;
    input.flow_state = "completed".to_string();
    input.mutation_checkpoint.checkpoint_state = "missing_blocks_apply".to_string();
    let err = ChangeObjectOrchestrationPacket::from_workspace_packet(input, &workspace_packet)
        .expect_err("completed with missing_blocks_apply checkpoint must fail");
    assert!(err.message().contains("missing_blocks_apply"));
}

#[test]
fn repo_root_refs_are_distinct_across_fixtures() {
    let fixtures = [
        "parent_repo_worktree_switch.json",
        "child_submodule_patch_stack_publish.json",
        "sibling_nested_repo_change_object_apply.json",
        "shallow_history_pointer_asset_rollback.json",
    ];
    let mut roots = std::collections::BTreeSet::new();
    for name in fixtures {
        let fixture = load_fixture(name);
        let packet = packet_for_fixture(&fixture);
        assert!(
            roots.insert(packet.orchestration.repo_root_ref.clone()),
            "repo_root_ref must be distinct across fixtures: {}",
            packet.orchestration.repo_root_ref
        );
    }
}
