//! Fixture-driven coverage for the stabilized daily Git loop.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

use aureline_git::{
    DailyLoopOperationKind, DailyLoopRequest, DailyLoopService, DailyLoopSnapshotState,
    DailyLoopPreviewState,
};

#[derive(Debug, Deserialize)]
struct DailyLoopFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    setup_mode: String,
    operation: String,
    #[serde(default)]
    paths: Vec<String>,
    expected: ExpectedDailyLoop,
}

#[derive(Debug, Deserialize)]
struct ExpectedDailyLoop {
    snapshot_state: Option<String>,
    preview_state: Option<String>,
    outcome_state: Option<String>,
    path_status_count: Option<u32>,
    stash_entry_count: Option<u32>,
    history_commit_count: Option<u32>,
    blame_line_count: Option<u32>,
    target_repo_root_present: bool,
    target_worktree_root_present: bool,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/git/m4/daily_loop_beta")
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
    fs::write(root.join("tracked.txt"), "one\n").expect("write tracked file");
    run_git(root, &["add", "tracked.txt"]);
    run_git(root, &["commit", "-q", "-m", "initial fixture commit"]);
}

fn build_case_root(mode: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    match mode {
        "attached_dirty" => {
            seed_committed_repo(dir.path());
            fs::write(dir.path().join("tracked.txt"), "one\ntwo\n").expect("modify tracked file");
            fs::write(dir.path().join("staged.txt"), "staged\n").expect("write staged file");
            run_git(dir.path(), &["add", "staged.txt"]);
            fs::write(dir.path().join("untracked.txt"), "untracked\n")
                .expect("write untracked file");
        }
        "detached_head" => {
            seed_committed_repo(dir.path());
            run_git(dir.path(), &["checkout", "-q", "--detach", "HEAD"]);
        }
        "not_a_repository" => {
            fs::write(dir.path().join("plain.txt"), "plain\n").expect("write plain file");
        }
        "with_stash" => {
            seed_committed_repo(dir.path());
            fs::write(dir.path().join("stashed.txt"), "stashed\n").expect("write stashed file");
            run_git(dir.path(), &["add", "stashed.txt"]);
            run_git(dir.path(), &["stash", "push", "-m", "fixture stash"]);
        }
        "with_history" => {
            seed_committed_repo(dir.path());
            fs::write(dir.path().join("second.txt"), "second\n").expect("write second file");
            run_git(dir.path(), &["add", "second.txt"]);
            run_git(dir.path(), &["commit", "-q", "-m", "second commit"]);
        }
        other => panic!("unsupported setup_mode: {other}"),
    }
    dir
}

fn run_fixture(path: &Path) {
    let text = fs::read_to_string(path).expect("read fixture");
    let fixture: DailyLoopFixture = serde_yaml::from_str(&text).expect("parse fixture");
    assert_eq!(fixture.record_kind, "git_daily_loop_beta_case");
    assert_eq!(fixture.schema_version, 1);

    let dir = build_case_root(&fixture.setup_mode);
    let root = dir.path().to_path_buf();

    let kind = parse_operation(&fixture.operation);
    let paths: Vec<PathBuf> = fixture.paths.iter().map(PathBuf::from).collect();
    let request = DailyLoopRequest::for_worktree(&root, kind, paths);
    let service = DailyLoopService::default();

    if kind.is_mutation() {
        let preview = service.preview(&request);
        if let Some(expected_state) = &fixture.expected.preview_state {
            assert_eq!(
                preview.state.as_str(),
                expected_state,
                "case {} preview state mismatch",
                fixture.case_name
            );
        }
        if preview.state == DailyLoopPreviewState::Ready && fixture.expected.outcome_state.is_some() {
            let result = service.apply(&preview, "fixture:actor");
            if let Some(expected_outcome) = &fixture.expected.outcome_state {
                assert_eq!(
                    result.outcome.as_str(),
                    expected_outcome,
                    "case {} outcome state mismatch",
                    fixture.case_name
                );
            }
        }
    } else {
        let snapshot = service.snapshot(&request);
        if let Some(expected_state) = &fixture.expected.snapshot_state {
            assert_eq!(
                snapshot.state.as_str(),
                expected_state,
                "case {} snapshot state mismatch",
                fixture.case_name
            );
        }
        if let Some(expected_count) = fixture.expected.path_status_count {
            assert_eq!(
                snapshot.path_statuses.len() as u32,
                expected_count,
                "case {} path_status count mismatch",
                fixture.case_name
            );
        }
        if let Some(expected_count) = fixture.expected.stash_entry_count {
            assert_eq!(
                snapshot.stash_entries.len() as u32,
                expected_count,
                "case {} stash_entry count mismatch",
                fixture.case_name
            );
        }
        if let Some(expected_count) = fixture.expected.history_commit_count {
            assert_eq!(
                snapshot.history_commits.len() as u32,
                expected_count,
                "case {} history_commit count mismatch",
                fixture.case_name
            );
        }
        if let Some(expected_count) = fixture.expected.blame_line_count {
            assert_eq!(
                snapshot.blame_lines.len() as u32,
                expected_count,
                "case {} blame_line count mismatch",
                fixture.case_name
            );
        }
    }

    assert!(
        fixture.expected.target_repo_root_present,
        "case {} expected repo_root",
        fixture.case_name
    );
    assert!(
        fixture.expected.target_worktree_root_present,
        "case {} expected worktree_root",
        fixture.case_name
    );
}

fn parse_operation(op: &str) -> DailyLoopOperationKind {
    match op {
        "status" => DailyLoopOperationKind::Status,
        "diff" => DailyLoopOperationKind::Diff,
        "stage" => DailyLoopOperationKind::Stage,
        "unstage" => DailyLoopOperationKind::Unstage,
        "commit" => DailyLoopOperationKind::Commit,
        "amend" => DailyLoopOperationKind::Amend,
        "stash_capture" => DailyLoopOperationKind::StashCapture,
        "stash_apply" => DailyLoopOperationKind::StashApply,
        "stash_pop" => DailyLoopOperationKind::StashPop,
        "stash_drop" => DailyLoopOperationKind::StashDrop,
        "stash_branch_from" => DailyLoopOperationKind::StashBranchFrom,
        "blame" => DailyLoopOperationKind::Blame,
        "history" => DailyLoopOperationKind::History,
        other => panic!("unsupported operation: {other}"),
    }
}

#[test]
fn status_attached_dirty() {
    let dir = build_case_root("attached_dirty");
    let request = DailyLoopRequest::for_worktree(dir.path(), DailyLoopOperationKind::Status, vec![]);
    let service = DailyLoopService::default();
    let snapshot = service.snapshot(&request);
    assert_eq!(snapshot.state, DailyLoopSnapshotState::Current);
    assert!(!snapshot.path_statuses.is_empty());
}

#[test]
fn status_not_a_repository() {
    let dir = build_case_root("not_a_repository");
    let request = DailyLoopRequest::for_worktree(dir.path(), DailyLoopOperationKind::Status, vec![]);
    let service = DailyLoopService::default();
    let snapshot = service.snapshot(&request);
    assert_eq!(snapshot.state, DailyLoopSnapshotState::NotRepository);
}

#[test]
fn stash_list_with_stash() {
    let dir = build_case_root("with_stash");
    let request = DailyLoopRequest::for_worktree(dir.path(), DailyLoopOperationKind::StashCapture, vec![]);
    let service = DailyLoopService::default();
    let snapshot = service.snapshot(&request);
    assert_eq!(snapshot.state, DailyLoopSnapshotState::Current);
    assert_eq!(snapshot.stash_entries.len(), 1);
    let entry = &snapshot.stash_entries[0];
    assert_eq!(entry.creator, "actor:git:stash");
    assert!(!entry.stash_entry_id.is_empty());
    assert_eq!(entry.source_repo.repo_root, dir.path());
    assert_eq!(entry.source_worktree.worktree_root, dir.path());
}

#[test]
fn history_with_commits() {
    let dir = build_case_root("with_history");
    let request = DailyLoopRequest::for_worktree(dir.path(), DailyLoopOperationKind::History, vec![]);
    let service = DailyLoopService::default();
    let snapshot = service.snapshot(&request);
    assert_eq!(snapshot.state, DailyLoopSnapshotState::Current);
    assert_eq!(snapshot.history_commits.len(), 2);
}

#[test]
fn commit_preview_blocked_no_message() {
    let dir = build_case_root("attached_dirty");
    let request = DailyLoopRequest::for_worktree(dir.path(), DailyLoopOperationKind::Commit, vec![]);
    let service = DailyLoopService::default();
    let preview = service.preview(&request);
    assert_eq!(preview.state, DailyLoopPreviewState::Blocked);
}

#[test]
fn stage_preview_ready() {
    let dir = build_case_root("attached_dirty");
    let request = DailyLoopRequest::for_worktree(
        dir.path(),
        DailyLoopOperationKind::Stage,
        vec![PathBuf::from("untracked.txt")],
    );
    let service = DailyLoopService::default();
    let preview = service.preview(&request);
    assert_eq!(preview.state, DailyLoopPreviewState::Ready);
    assert!(!preview.affected_paths.is_empty());
}

#[test]
fn fixture_files() {
    let dir = fixtures_dir();
    if !dir.exists() {
        return;
    }
    let entries = fs::read_dir(&dir).expect("read fixtures dir");
    for entry in entries {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            run_fixture(&path);
        }
    }
}
