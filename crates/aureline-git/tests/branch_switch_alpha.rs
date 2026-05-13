//! Fixture-driven coverage for preview-first Git branch operations.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use aureline_git::{
    GitBranchOperationKind, GitBranchOutcomeState, GitBranchPreviewState, GitBranchRemoteState,
    GitBranchRequest, GitBranchService, GitBranchTargetKind, GitStatusRequest, GitStatusService,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BranchFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    setup_mode: String,
    operation: GitBranchOperationKind,
    target: String,
    start_point: Option<String>,
    track_remote: bool,
    expected: ExpectedBranch,
}

#[derive(Debug, Deserialize)]
struct ExpectedBranch {
    preview_state: String,
    target_kind: String,
    remote_state: String,
    current_work_warning_state: String,
    uncommitted_warning_required: bool,
    detached_head_disclosed: bool,
    missing_remote_disclosed: bool,
    result_state: String,
    final_head_state: String,
    final_branch_label: Option<String>,
    final_total_changed_count: u32,
    after_shell_branch_label: Option<String>,
    activity_state_class: String,
    support_export_phase: String,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/git/branch_switch_alpha")
}

fn run_git(root: &Path, args: &[&str]) {
    let status = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .status()
        .expect("git command launches");
    assert!(
        status.success(),
        "git {args:?} failed in {}",
        root.display()
    );
}

fn init_repo(root: &Path) {
    let status = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["init", "-q", "-b", "main"])
        .status()
        .expect("git init launches");
    if !status.success() {
        run_git(root, &["init", "-q"]);
        run_git(root, &["checkout", "-q", "-b", "main"]);
    }
    run_git(root, &["config", "user.email", "fixture@example.invalid"]);
    run_git(root, &["config", "user.name", "Fixture"]);
}

fn seed_committed_repo(root: &Path) {
    init_repo(root);
    fs::create_dir_all(root.join("src")).expect("create src dir");
    fs::write(
        root.join("src/lib.rs"),
        "pub fn answer() -> u32 {\n    1\n}\n",
    )
    .expect("write committed source");
    run_git(root, &["add", "src/lib.rs"]);
    run_git(root, &["commit", "-q", "-m", "initial fixture commit"]);
}

fn add_second_commit(root: &Path) {
    fs::write(
        root.join("src/lib.rs"),
        "pub fn answer() -> u32 {\n    2\n}\n",
    )
    .expect("write second source");
    run_git(root, &["add", "src/lib.rs"]);
    run_git(root, &["commit", "-q", "-m", "second fixture commit"]);
}

fn build_case_root(mode: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    match mode {
        "main_clean" => seed_committed_repo(dir.path()),
        "two_branches_clean" => {
            seed_committed_repo(dir.path());
            run_git(dir.path(), &["branch", "feature"]);
        }
        "main_dirty" => {
            seed_committed_repo(dir.path());
            fs::write(
                dir.path().join("src/lib.rs"),
                "pub fn answer() -> u32 {\n    42\n}\n",
            )
            .expect("modify source");
            fs::write(dir.path().join("notes.txt"), "untracked\n").expect("write notes");
        }
        "two_commits_clean" => {
            seed_committed_repo(dir.path());
            add_second_commit(dir.path());
        }
        other => panic!("unsupported setup_mode: {other}"),
    }
    dir
}

fn request_for_fixture(fixture: &BranchFixture, root: &Path) -> GitBranchRequest {
    let mut request = GitBranchRequest::with_observed_at(
        format!("workspace.fixture.{}", fixture.case_name),
        root,
        fixture.operation,
        fixture.target.clone(),
        "2026-05-13T00:00:00Z",
    )
    .with_track_remote(fixture.track_remote)
    .with_launch_source_ref(format!("git.branch.sheet.{}", fixture.case_name));
    if let Some(start_point) = &fixture.start_point {
        request = request.with_start_point(start_point.clone());
    }
    request
}

fn parse_preview_state(value: &str) -> GitBranchPreviewState {
    match value {
        "ready_to_apply" => GitBranchPreviewState::ReadyToApply,
        "blocked" => GitBranchPreviewState::Blocked,
        "degraded" => GitBranchPreviewState::Degraded,
        other => panic!("unsupported preview state: {other}"),
    }
}

fn parse_result_state(value: &str) -> GitBranchOutcomeState {
    match value {
        "applied" => GitBranchOutcomeState::Applied,
        "blocked_no_changes_made" => GitBranchOutcomeState::BlockedNoChangesMade,
        "failed" => GitBranchOutcomeState::Failed,
        other => panic!("unsupported result state: {other}"),
    }
}

fn parse_target_kind(value: &str) -> GitBranchTargetKind {
    match value {
        "local_branch" => GitBranchTargetKind::LocalBranch,
        "new_branch" => GitBranchTargetKind::NewBranch,
        "detached_head" => GitBranchTargetKind::DetachedHead,
        "remote_tracking_branch" => GitBranchTargetKind::RemoteTrackingBranch,
        "unknown" => GitBranchTargetKind::Unknown,
        other => panic!("unsupported target kind: {other}"),
    }
}

fn parse_remote_state(value: &str) -> GitBranchRemoteState {
    match value {
        "not_applicable" => GitBranchRemoteState::NotApplicable,
        "upstream_configured" => GitBranchRemoteState::UpstreamConfigured,
        "upstream_missing" => GitBranchRemoteState::UpstreamMissing,
        "target_remote_available" => GitBranchRemoteState::TargetRemoteAvailable,
        "target_remote_missing" => GitBranchRemoteState::TargetRemoteMissing,
        "target_remote_branch_missing" => GitBranchRemoteState::TargetRemoteBranchMissing,
        other => panic!("unsupported remote state: {other}"),
    }
}

fn final_snapshot(case_name: &str, root: &Path) -> aureline_git::GitStatusSnapshot {
    let request = GitStatusRequest::with_observed_at(
        format!("workspace.fixture.{case_name}"),
        root,
        "2026-05-13T00:00:02Z",
    );
    GitStatusService::default().snapshot(&request)
}

fn run_fixture(path: &Path) {
    let text = fs::read_to_string(path).expect("read fixture");
    let fixture: BranchFixture = serde_yaml::from_str(&text).expect("parse fixture");
    assert_eq!(fixture.record_kind, "git_branch_switch_alpha_case");
    assert_eq!(fixture.schema_version, 1);

    let dir = build_case_root(&fixture.setup_mode);
    let request = request_for_fixture(&fixture, dir.path());
    let service = GitBranchService::default();
    let preview = service.preview(&request);

    assert_eq!(
        preview.preview_state,
        parse_preview_state(&fixture.expected.preview_state),
        "{}: preview state",
        fixture.case_name
    );
    assert_eq!(
        preview.target.target_kind,
        parse_target_kind(&fixture.expected.target_kind),
        "{}: target kind",
        fixture.case_name
    );
    assert_eq!(
        preview.target.remote_state,
        parse_remote_state(&fixture.expected.remote_state),
        "{}: remote state",
        fixture.case_name
    );
    assert_eq!(
        preview.current_work.warning_state, fixture.expected.current_work_warning_state,
        "{}: current-work warning state",
        fixture.case_name
    );
    assert_eq!(
        preview.current_work.uncommitted_warning_required,
        fixture.expected.uncommitted_warning_required,
        "{}: current-work warning required",
        fixture.case_name
    );
    assert_eq!(
        preview.target.detached_head_disclosed, fixture.expected.detached_head_disclosed,
        "{}: detached-head disclosure",
        fixture.case_name
    );
    assert_eq!(
        preview.target.missing_remote_disclosed, fixture.expected.missing_remote_disclosed,
        "{}: missing-remote disclosure",
        fixture.case_name
    );
    if preview.preview_state == GitBranchPreviewState::ReadyToApply {
        assert!(
            preview.ready_to_apply(),
            "{}: preview is ready to apply",
            fixture.case_name
        );
    }

    let result = service.apply(&preview, "2026-05-13T00:00:01Z");
    assert_eq!(
        result.outcome_state,
        parse_result_state(&fixture.expected.result_state),
        "{}: result state",
        fixture.case_name
    );
    assert_eq!(
        result.activity.state_class, fixture.expected.activity_state_class,
        "{}: activity state",
        fixture.case_name
    );
    assert_eq!(
        result.support_export.phase, fixture.expected.support_export_phase,
        "{}: support phase",
        fixture.case_name
    );
    assert_eq!(
        result.support_export.branch_journal_ref.as_deref(),
        Some(result.branch_journal.branch_journal_ref.as_str()),
        "{}: support and journal attribution",
        fixture.case_name
    );
    assert!(
        result.branch_identity_synchronized(),
        "{}: branch identity synchronized",
        fixture.case_name
    );
    assert_eq!(
        result
            .after_shell
            .as_ref()
            .and_then(|shell| shell.branch_label.clone()),
        fixture.expected.after_shell_branch_label,
        "{}: result shell branch",
        fixture.case_name
    );

    let snapshot = final_snapshot(&fixture.case_name, dir.path());
    assert_eq!(
        snapshot.head.state.as_str(),
        fixture.expected.final_head_state,
        "{}: final head state",
        fixture.case_name
    );
    assert_eq!(
        snapshot.head.branch_label, fixture.expected.final_branch_label,
        "{}: final branch label",
        fixture.case_name
    );
    assert_eq!(
        snapshot.change_summary.total_changed_count, fixture.expected.final_total_changed_count,
        "{}: final changed count",
        fixture.case_name
    );
}

#[test]
fn protected_branch_switch_fixtures_match_git_service_contract() {
    let mut fixtures: Vec<_> = fs::read_dir(fixtures_dir())
        .expect("fixture directory")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "yaml"))
        .collect();
    fixtures.sort();
    assert!(!fixtures.is_empty(), "branch fixtures must exist");

    for path in fixtures {
        run_fixture(&path);
    }
}

#[test]
fn current_work_drift_blocks_apply_without_switching() {
    let dir = build_case_root("two_branches_clean");
    let service = GitBranchService::default();
    let request = GitBranchRequest::with_observed_at(
        "workspace.fixture.branch-drift",
        dir.path(),
        GitBranchOperationKind::Switch,
        "feature",
        "2026-05-13T00:10:00Z",
    );
    let preview = service.preview(&request);
    assert!(preview.ready_to_apply());

    fs::write(dir.path().join("notes.txt"), "created after preview\n").expect("write drift");

    let result = service.apply(&preview, "2026-05-13T00:10:01Z");
    assert_eq!(
        result.outcome_state,
        GitBranchOutcomeState::BlockedNoChangesMade
    );
    let snapshot = final_snapshot("branch-drift", dir.path());
    assert_eq!(snapshot.head.branch_label.as_deref(), Some("main"));
}
