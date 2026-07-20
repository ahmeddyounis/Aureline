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
        "{}: result state: {result:#?}",
        fixture.case_name,
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

#[test]
fn changed_bytes_with_the_same_status_shape_block_apply() {
    let dir = build_case_root("main_dirty");
    run_git(dir.path(), &["branch", "feature"]);
    let service = GitBranchService::default();
    let request = GitBranchRequest::with_observed_at(
        "workspace.fixture.byte-drift",
        dir.path(),
        GitBranchOperationKind::Switch,
        "feature",
        "2026-05-13T00:20:00Z",
    );
    let preview = service.preview(&request);
    assert!(preview.ready_to_apply());

    fs::write(
        dir.path().join("src/lib.rs"),
        "pub fn answer() -> u32 {\n    43\n}\n",
    )
    .expect("drift bytes without changing status shape");
    let result = service.apply(&preview, "2026-05-13T00:20:01Z");
    assert_eq!(
        result.outcome_state,
        GitBranchOutcomeState::BlockedNoChangesMade
    );
    assert_eq!(
        final_snapshot("byte-drift", dir.path())
            .head
            .branch_label
            .as_deref(),
        Some("main")
    );
}

#[test]
fn repository_config_drift_blocks_apply_without_switching() {
    let dir = build_case_root("two_branches_clean");
    let service = GitBranchService::default();
    let preview = service.preview(&GitBranchRequest::with_observed_at(
        "workspace.fixture.config-drift",
        dir.path(),
        GitBranchOperationKind::Switch,
        "feature",
        "2026-05-13T00:25:00Z",
    ));
    assert!(preview.ready_to_apply(), "{preview:#?}");

    run_git(dir.path(), &["config", "status.relativePaths", "false"]);
    let result = service.apply(&preview, "2026-05-13T00:25:01Z");
    assert_eq!(
        result.outcome_state,
        GitBranchOutcomeState::BlockedNoChangesMade
    );
    assert_eq!(
        final_snapshot("config-drift", dir.path())
            .head
            .branch_label
            .as_deref(),
        Some("main")
    );
}

#[test]
fn retargeted_branch_and_non_live_previews_cannot_apply() {
    let dir = build_case_root("two_commits_clean");
    run_git(dir.path(), &["branch", "feature"]);
    let service = GitBranchService::default();
    let request = GitBranchRequest::with_observed_at(
        "workspace.fixture.target-drift",
        dir.path(),
        GitBranchOperationKind::Switch,
        "feature",
        "2026-05-13T00:30:00Z",
    );
    let preview = service.preview(&request);
    assert!(preview.ready_to_apply());

    let exported = serde_json::to_string(&preview).expect("serialize inspection record");
    let restored = serde_json::from_str(&exported).expect("deserialize inspection record");
    assert!(!GitBranchService::default()
        .apply(&restored, "2026-05-13T00:30:01Z")
        .outcome_state
        .eq(&GitBranchOutcomeState::Applied));

    let mut tampered = preview.clone();
    tampered.workspace_ref = "workspace.tampered".to_string();
    assert_eq!(
        service
            .apply(&tampered, "2026-05-13T00:30:02Z")
            .outcome_state,
        GitBranchOutcomeState::BlockedNoChangesMade
    );

    let fresh = service.preview(&request);
    run_git(dir.path(), &["branch", "-f", "feature", "HEAD~1"]);
    assert_eq!(
        service.apply(&fresh, "2026-05-13T00:30:03Z").outcome_state,
        GitBranchOutcomeState::BlockedNoChangesMade
    );
    assert_eq!(
        final_snapshot("target-drift", dir.path())
            .head
            .branch_label
            .as_deref(),
        Some("main")
    );
}

#[cfg(unix)]
#[test]
fn branch_apply_does_not_execute_repository_checkout_hooks() {
    use std::os::unix::fs::PermissionsExt;

    let dir = build_case_root("two_branches_clean");
    let sentinel = dir.path().join("hook-ran");
    let hook = dir.path().join(".git/hooks/post-checkout");
    fs::write(
        &hook,
        format!("#!/bin/sh\ntouch '{}'\n", sentinel.display()),
    )
    .expect("write hook");
    let mut permissions = fs::metadata(&hook).expect("hook metadata").permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(&hook, permissions).expect("make hook executable");

    let service = GitBranchService::default();
    let preview = service.preview(&GitBranchRequest::with_observed_at(
        "workspace.fixture.hook",
        dir.path(),
        GitBranchOperationKind::Switch,
        "feature",
        "2026-05-13T00:40:00Z",
    ));
    let result = service.apply(&preview, "2026-05-13T00:40:01Z");
    assert_eq!(
        result.outcome_state,
        GitBranchOutcomeState::Applied,
        "{result:#?}"
    );
    assert!(!sentinel.exists(), "repository hook must not execute");
}

#[cfg(unix)]
#[test]
fn branch_apply_never_overwrites_an_ignored_worktree_file() {
    let dir = build_case_root("main_clean");
    fs::write(dir.path().join(".gitignore"), "ignored.txt\n").expect("write ignore rule");
    run_git(dir.path(), &["add", ".gitignore"]);
    run_git(dir.path(), &["commit", "-q", "-m", "ignore local artifact"]);
    run_git(dir.path(), &["switch", "-q", "-c", "feature"]);
    fs::write(dir.path().join("ignored.txt"), "tracked target bytes\n").expect("write target file");
    run_git(dir.path(), &["add", "-f", "ignored.txt"]);
    run_git(dir.path(), &["commit", "-q", "-m", "track target artifact"]);
    run_git(dir.path(), &["switch", "-q", "main"]);
    fs::write(dir.path().join("ignored.txt"), "private local bytes\n")
        .expect("write ignored local file");

    let service = GitBranchService::default();
    let preview = service.preview(&GitBranchRequest::with_observed_at(
        "workspace.fixture.ignored-collision",
        dir.path(),
        GitBranchOperationKind::Switch,
        "feature",
        "2026-05-13T00:50:00Z",
    ));
    assert!(preview.ready_to_apply(), "{preview:#?}");

    let result = service.apply(&preview, "2026-05-13T00:50:01Z");
    assert_eq!(result.outcome_state, GitBranchOutcomeState::Failed);
    assert_eq!(
        fs::read_to_string(dir.path().join("ignored.txt")).expect("read preserved file"),
        "private local bytes\n"
    );
    assert_eq!(
        final_snapshot("ignored-collision", dir.path())
            .head
            .branch_label
            .as_deref(),
        Some("main")
    );
}

#[cfg(unix)]
#[test]
fn symlink_evidence_hashes_the_link_payload_without_following_its_target() {
    use std::os::unix::fs::symlink;

    let dir = build_case_root("two_branches_clean");
    let external = tempfile::tempdir().expect("external tempdir");
    let first_target = external.path().join("first-target");
    let second_target = external.path().join("second-target");
    fs::write(&first_target, "outside v1\n").expect("write first external target");
    fs::write(&second_target, "outside v2\n").expect("write second external target");
    symlink(&first_target, dir.path().join("reviewed-link")).expect("create reviewed symlink");

    let service = GitBranchService::default();
    let request = GitBranchRequest::with_observed_at(
        "workspace.fixture.symlink-payload",
        dir.path(),
        GitBranchOperationKind::Switch,
        "feature",
        "2026-05-13T00:52:00Z",
    );
    let preview = service.preview(&request);
    assert!(preview.ready_to_apply(), "{preview:#?}");
    fs::write(&first_target, "outside target changed after review\n")
        .expect("mutate followed target only");
    assert_eq!(
        service
            .apply(&preview, "2026-05-13T00:52:01Z")
            .outcome_state,
        GitBranchOutcomeState::Applied,
        "changing only followed bytes must not change symlink payload evidence"
    );

    run_git(dir.path(), &["switch", "-q", "main"]);
    let fresh = service.preview(&request);
    assert!(fresh.ready_to_apply(), "{fresh:#?}");
    fs::remove_file(dir.path().join("reviewed-link")).expect("remove reviewed symlink");
    symlink(&second_target, dir.path().join("reviewed-link")).expect("retarget reviewed symlink");
    assert_eq!(
        service.apply(&fresh, "2026-05-13T00:52:02Z").outcome_state,
        GitBranchOutcomeState::BlockedNoChangesMade,
        "changing the symlink payload must invalidate review"
    );
}

#[cfg(unix)]
#[test]
fn special_worktree_files_are_rejected_from_branch_authority() {
    let dir = build_case_root("two_branches_clean");
    fs::remove_file(dir.path().join("src/lib.rs")).expect("remove tracked regular file");
    assert!(Command::new("mkfifo")
        .arg(dir.path().join("src/lib.rs"))
        .status()
        .expect("mkfifo launches")
        .success());

    let preview = GitBranchService::default().preview(&GitBranchRequest::with_observed_at(
        "workspace.fixture.special-file",
        dir.path(),
        GitBranchOperationKind::Switch,
        "feature",
        "2026-05-13T00:53:00Z",
    ));
    assert_eq!(preview.preview_state, GitBranchPreviewState::Blocked);
    assert!(!preview.ready_to_apply());
}

#[cfg(unix)]
#[test]
fn non_utf8_status_paths_degrade_branch_review_before_apply() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    let dir = build_case_root("two_branches_clean");
    let invalid_name = OsString::from_vec(b"invalid-\xff-name".to_vec());
    if let Err(error) = fs::write(dir.path().join(invalid_name), "unreviewable path bytes\n") {
        if cfg!(target_os = "macos") {
            // APFS rejects this fixture before Git can observe it. The raw
            // parser unit test covers the fail-closed path on this platform.
            return;
        }
        panic!("write non-UTF-8 path: {error}");
    }

    let preview = GitBranchService::default().preview(&GitBranchRequest::with_observed_at(
        "workspace.fixture.non-utf8-path",
        dir.path(),
        GitBranchOperationKind::Switch,
        "feature",
        "2026-05-13T00:55:00Z",
    ));
    assert_eq!(preview.preview_state, GitBranchPreviewState::Degraded);
    assert!(!preview.ready_to_apply());
}
