//! Fixture-driven coverage for the stabilized daily Git loop.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};

use serde::Deserialize;

use aureline_git::stabilize_the_daily_git_loop_status_diff_stage::DailyLoopBackend;
use aureline_git::{
    DailyLoopBackendError, DailyLoopBackendErrorClass, DailyLoopCommandOutput,
    DailyLoopOperationKind, DailyLoopPreviewState, DailyLoopRequest, DailyLoopResult,
    DailyLoopService, DailyLoopSnapshotState, DailyLoopSupportExportRecord,
};

#[derive(Debug, Clone, Default)]
struct RecordingBackend {
    calls: Arc<Mutex<Vec<Vec<String>>>>,
}

#[derive(Debug, Clone, Copy)]
struct PrivateFailureBackend;

impl DailyLoopBackend for PrivateFailureBackend {
    fn run_git(
        &self,
        _root: &Path,
        _args: &[&str],
    ) -> Result<DailyLoopCommandOutput, DailyLoopBackendError> {
        Err(DailyLoopBackendError::new(
            DailyLoopBackendErrorClass::Io,
            "/private/customer/token-value",
        ))
    }
}

impl DailyLoopBackend for RecordingBackend {
    fn run_git(
        &self,
        root: &Path,
        args: &[&str],
    ) -> Result<DailyLoopCommandOutput, DailyLoopBackendError> {
        self.calls
            .lock()
            .expect("recording backend lock")
            .push(args.iter().map(|arg| (*arg).to_string()).collect());

        let stdout = match args {
            ["rev-parse", "--absolute-git-dir"] => {
                format!("{}\n", root.join(".git").display()).into_bytes()
            }
            ["rev-parse", "--path-format=absolute", "--git-common-dir"] => {
                format!("{}\n", root.join(".git").display()).into_bytes()
            }
            ["rev-parse", "--show-toplevel"] => format!("{}\n", root.display()).into_bytes(),
            ["rev-parse", "--is-bare-repository"] | ["rev-parse", "--is-shallow-repository"] => {
                b"false\n".to_vec()
            }
            ["rev-parse", "--abbrev-ref", "HEAD"] => b"main\n".to_vec(),
            _ => Vec::new(),
        };

        Ok(DailyLoopCommandOutput {
            stdout,
            stderr: Vec::new(),
            status_code: Some(0),
            success: true,
        })
    }
}

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
        if preview.state == DailyLoopPreviewState::Ready && fixture.expected.outcome_state.is_some()
        {
            let result = service.apply(&preview, &request.actor_ref);
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
    let request =
        DailyLoopRequest::for_worktree(dir.path(), DailyLoopOperationKind::Status, vec![]);
    let service = DailyLoopService::default();
    let snapshot = service.snapshot(&request);
    assert_eq!(snapshot.state, DailyLoopSnapshotState::Current);
    assert!(!snapshot.path_statuses.is_empty());
}

#[test]
fn status_respects_the_reviewed_path_scope() {
    let dir = build_case_root("attached_dirty");
    let request = DailyLoopRequest::for_worktree(
        dir.path(),
        DailyLoopOperationKind::Status,
        vec![PathBuf::from("tracked.txt")],
    );

    let snapshot = DailyLoopService::default().snapshot(&request);

    assert_eq!(snapshot.state, DailyLoopSnapshotState::Current);
    assert_eq!(snapshot.path_statuses.len(), 1);
    assert_eq!(snapshot.path_statuses[0].path, "tracked.txt");
}

#[cfg(unix)]
#[test]
fn status_preserves_newline_bearing_repository_paths() {
    let dir = build_case_root("with_history");
    let path = "line\nbreak.txt";
    fs::write(dir.path().join(path), "newline path\n").expect("write newline path");
    let request =
        DailyLoopRequest::for_worktree(dir.path(), DailyLoopOperationKind::Status, vec![]);
    let snapshot = DailyLoopService::default().snapshot(&request);

    assert_eq!(snapshot.state, DailyLoopSnapshotState::Current);
    assert!(snapshot.path_statuses.iter().any(|row| row.path == path));
}

#[test]
fn status_not_a_repository() {
    let dir = build_case_root("not_a_repository");
    let request =
        DailyLoopRequest::for_worktree(dir.path(), DailyLoopOperationKind::Status, vec![]);
    let service = DailyLoopService::default();
    let snapshot = service.snapshot(&request);
    assert_eq!(snapshot.state, DailyLoopSnapshotState::NotRepository);
}

#[test]
fn stash_list_with_stash() {
    let dir = build_case_root("with_stash");
    let request =
        DailyLoopRequest::for_worktree(dir.path(), DailyLoopOperationKind::StashCapture, vec![]);
    let service = DailyLoopService::default();
    let snapshot = service.snapshot(&request);
    assert_eq!(snapshot.state, DailyLoopSnapshotState::Current);
    assert_eq!(snapshot.stash_entries.len(), 1);
    let entry = &snapshot.stash_entries[0];
    assert_eq!(entry.creator, "actor:git:stash");
    assert!(!entry.stash_entry_id.is_empty());
    let canonical_root = fs::canonicalize(dir.path()).expect("canonical fixture root");
    assert_eq!(entry.source_repo.repo_root, canonical_root);
    assert_eq!(entry.source_worktree.worktree_root, canonical_root);
}

#[test]
fn history_with_commits() {
    let dir = build_case_root("with_history");
    let request =
        DailyLoopRequest::for_worktree(dir.path(), DailyLoopOperationKind::History, vec![]);
    let service = DailyLoopService::default();
    let snapshot = service.snapshot(&request);
    assert_eq!(snapshot.state, DailyLoopSnapshotState::Current);
    assert_eq!(snapshot.history_commits.len(), 2);
}

#[test]
fn history_respects_the_reviewed_path_scope() {
    let dir = build_case_root("with_history");
    let request = DailyLoopRequest::for_worktree(
        dir.path(),
        DailyLoopOperationKind::History,
        vec![PathBuf::from("second.txt")],
    );

    let snapshot = DailyLoopService::default().snapshot(&request);

    assert_eq!(snapshot.state, DailyLoopSnapshotState::Current);
    assert_eq!(snapshot.history_commits.len(), 1);
    assert_eq!(snapshot.history_commits[0].summary, "second commit");
}

#[test]
fn commit_preview_blocked_no_message() {
    let dir = build_case_root("attached_dirty");
    let request =
        DailyLoopRequest::for_worktree(dir.path(), DailyLoopOperationKind::Commit, vec![]);
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
fn diff_snapshot_preserves_every_scoped_path() {
    let dir = tempfile::tempdir().expect("tempdir");
    let backend = RecordingBackend::default();
    let calls = Arc::clone(&backend.calls);
    let service = DailyLoopService::new(backend);
    let request = DailyLoopRequest::for_worktree(
        dir.path(),
        DailyLoopOperationKind::Diff,
        vec![PathBuf::from("src/one.rs"), PathBuf::from("src/two.rs")],
    );

    let snapshot = service.snapshot(&request);

    assert_eq!(snapshot.state, DailyLoopSnapshotState::Current);
    let calls = calls.lock().expect("recording backend lock");
    let diff_call = calls
        .iter()
        .find(|args| args.first().map(String::as_str) == Some("diff"))
        .expect("diff command recorded");
    assert_eq!(
        diff_call,
        &[
            "diff",
            "--quiet",
            "--no-ext-diff",
            "--no-textconv",
            "--",
            "src/one.rs",
            "src/two.rs"
        ]
    );
}

#[test]
fn nonempty_diff_is_partial_instead_of_fabricating_file_rows() {
    let dir = build_case_root("attached_dirty");
    let request = DailyLoopRequest::for_worktree(
        dir.path(),
        DailyLoopOperationKind::Diff,
        vec![PathBuf::from("tracked.txt")],
    );

    let snapshot = DailyLoopService::default().snapshot(&request);

    assert_eq!(snapshot.state, DailyLoopSnapshotState::PartialOmitted);
    assert!(snapshot.diff_files.is_empty());
    assert!(snapshot
        .degraded_reason
        .as_deref()
        .is_some_and(|reason| reason.contains("structured file/hunk rows are unavailable")));
}

#[test]
fn blame_repeats_provenance_for_noncontiguous_lines_from_one_commit() {
    let dir = tempfile::tempdir().expect("tempdir");
    init_repo(dir.path());
    run_git(dir.path(), &["config", "user.name", "First Author"]);
    run_git(
        dir.path(),
        &["config", "user.email", "first@example.invalid"],
    );
    fs::write(
        dir.path().join("alternating.txt"),
        "old first\nold middle\nold third\n",
    )
    .expect("write first blame revision");
    run_git(dir.path(), &["add", "alternating.txt"]);
    run_git(dir.path(), &["commit", "-q", "-m", "first blame revision"]);

    run_git(dir.path(), &["config", "user.name", "Second Author"]);
    run_git(
        dir.path(),
        &["config", "user.email", "second@example.invalid"],
    );
    fs::write(
        dir.path().join("alternating.txt"),
        "old first\nnew middle\nold third\n",
    )
    .expect("write second blame revision");
    run_git(dir.path(), &["add", "alternating.txt"]);
    run_git(dir.path(), &["commit", "-q", "-m", "second blame revision"]);

    let request = DailyLoopRequest::for_worktree(
        dir.path(),
        DailyLoopOperationKind::Blame,
        vec![PathBuf::from("alternating.txt")],
    );
    let snapshot = DailyLoopService::default().snapshot(&request);

    assert_eq!(snapshot.state, DailyLoopSnapshotState::Current);
    assert_eq!(snapshot.blame_lines.len(), 3);
    assert_eq!(snapshot.blame_lines[0].author_name, "First Author");
    assert_eq!(snapshot.blame_lines[1].author_name, "Second Author");
    assert_eq!(snapshot.blame_lines[2].author_name, "First Author");
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

#[test]
fn support_export_v2_fixture_matches_the_redacted_record_shape() {
    let path = fixtures_dir().join("support_export_redacted_v2.json");
    let text = fs::read_to_string(path).expect("read support export fixture");
    let record: DailyLoopSupportExportRecord =
        serde_json::from_str(&text).expect("parse support export fixture");

    assert_eq!(record.record_kind, "git_daily_loop_support_export_record");
    assert_eq!(record.schema_version, 2);
    assert!(!record.raw_path_export_allowed);
    assert!(!record.raw_ref_name_export_allowed);
    assert!(record.target.workspace_ref_digest.starts_with("sha256:"));
    assert!(record
        .omitted_fields
        .contains(&"affected_paths".to_string()));
}

#[test]
fn support_identity_digests_cover_the_complete_identity() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut first_target =
        DailyLoopRequest::for_worktree(dir.path(), DailyLoopOperationKind::Status, Vec::new())
            .target;
    let common_prefix = "x".repeat(600);
    first_target.workspace_ref = format!("{common_prefix}a");
    let first = DailyLoopSupportExportRecord::from_result(&DailyLoopResult::completed(
        &first_target,
        DailyLoopOperationKind::Status,
        Vec::new(),
    ));

    let mut second_target = first_target;
    second_target.workspace_ref = format!("{common_prefix}b");
    let second = DailyLoopSupportExportRecord::from_result(&DailyLoopResult::completed(
        &second_target,
        DailyLoopOperationKind::Status,
        Vec::new(),
    ));

    assert_ne!(
        first.target.workspace_ref_digest, second.target.workspace_ref_digest,
        "equal-length identities with a shared prefix must not collide"
    );
}

#[test]
fn daily_stage_reuses_exact_patch_authority_and_blocks_stale_evidence() {
    let dir = build_case_root("attached_dirty");
    let request = DailyLoopRequest::for_worktree(
        dir.path(),
        DailyLoopOperationKind::Stage,
        vec![PathBuf::from("tracked.txt")],
    );
    let service = DailyLoopService::default();
    let preview = service.preview(&request);
    assert_eq!(preview.state, DailyLoopPreviewState::Ready);

    fs::write(dir.path().join("tracked.txt"), "raced bytes\n").expect("inject drift");
    let result = service.apply(&preview, &request.actor_ref);

    assert_eq!(result.outcome.as_str(), "blocked_no_changes_made");
    let status = Command::new("git")
        .arg("-C")
        .arg(dir.path())
        .args(["status", "--porcelain=v1", "--", "tracked.txt"])
        .output()
        .expect("status launches");
    assert_eq!(String::from_utf8_lossy(&status.stdout), " M tracked.txt\n");
}

#[test]
fn serialized_or_tampered_daily_preview_is_not_apply_authority() {
    let dir = build_case_root("attached_dirty");
    let request = DailyLoopRequest::for_worktree(
        dir.path(),
        DailyLoopOperationKind::Stage,
        vec![PathBuf::from("untracked.txt")],
    );
    let service = DailyLoopService::default();
    let preview = service.preview(&request);
    assert_eq!(preview.state, DailyLoopPreviewState::Ready);

    let json = serde_json::to_string(&preview).expect("serialize daily preview");
    assert!(!json.contains("apply_authority"));
    let restored = serde_json::from_str(&json).expect("deserialize daily preview");
    let restored_result = service.apply(&restored, &request.actor_ref);
    assert_eq!(restored_result.outcome.as_str(), "blocked_no_changes_made");

    let mut tampered = preview;
    tampered.target.worktree.display_label = "different target".to_string();
    let tampered_result = service.apply(&tampered, &request.actor_ref);
    assert_eq!(tampered_result.outcome.as_str(), "blocked_no_changes_made");
}

#[test]
fn daily_support_export_v2_omits_raw_target_paths_and_refs() {
    let dir = build_case_root("attached_dirty");
    let request = DailyLoopRequest::for_worktree(
        dir.path(),
        DailyLoopOperationKind::Stage,
        vec![PathBuf::from("untracked.txt")],
    );
    let service = DailyLoopService::default();
    let preview = service.preview(&request);
    let result = service.apply(&preview, &request.actor_ref);
    let mut private_result = result;
    private_result.target.workspace_ref = "workspace:private-customer".to_string();
    private_result.target.repo.repo_ref = "repo:/private/customer/repository".to_string();
    private_result.target.repo.display_label = "private-customer-repository".to_string();
    private_result.target.worktree.worktree_ref =
        "worktree:/private/customer/repository".to_string();
    private_result.target.worktree.head_label = "customers/secret-launch".to_string();
    private_result.target.worktree.display_label = "secret-worktree".to_string();
    private_result.observed_at = "private-observation\n/private/customer".to_string();
    let export = DailyLoopSupportExportRecord::from_result(&private_result);
    let json = serde_json::to_string(&export).expect("serialize support export");

    assert_eq!(export.schema_version, 2);
    assert!(!export.raw_path_export_allowed);
    assert!(!export.raw_ref_name_export_allowed);
    assert_eq!(export.affected_path_count, 1);
    assert_eq!(export.observed_at, "unavailable");
    let private_path = dir.path().to_string_lossy().into_owned();
    for private in [
        private_path.as_str(),
        "private-customer",
        "secret-launch",
        "secret-worktree",
        "untracked.txt",
    ] {
        assert!(!json.contains(private), "support export leaked {private}");
    }
}

#[test]
fn unsafe_refs_and_oversized_path_scopes_fail_before_backend_execution() {
    let dir = tempfile::tempdir().expect("tempdir");
    let backend = RecordingBackend::default();
    let calls = Arc::clone(&backend.calls);
    let service = DailyLoopService::new(backend);

    let unsafe_ref =
        DailyLoopRequest::for_worktree(dir.path(), DailyLoopOperationKind::History, Vec::new())
            .with_commit_ref("--output=/private/leak");
    let snapshot = service.snapshot(&unsafe_ref);
    assert_eq!(snapshot.state, DailyLoopSnapshotState::RefreshFailed);
    assert!(calls.lock().expect("calls lock").is_empty());

    let oversized = DailyLoopRequest::for_worktree(
        dir.path(),
        DailyLoopOperationKind::Stage,
        (0..=4096)
            .map(|index| PathBuf::from(format!("src/{index}.rs")))
            .collect(),
    );
    let preview = service.preview(&oversized);
    assert_eq!(preview.state, DailyLoopPreviewState::Blocked);
    assert!(calls.lock().expect("calls lock").is_empty());
}

#[test]
fn stash_mutations_are_narrowed_to_inspect_only_until_authority_exists() {
    let dir = build_case_root("with_stash");
    let request =
        DailyLoopRequest::for_worktree(dir.path(), DailyLoopOperationKind::StashDrop, Vec::new())
            .with_stash_entry_ref("stash@{0}");
    let preview = DailyLoopService::default().preview(&request);

    assert_eq!(preview.state, DailyLoopPreviewState::Blocked);
    assert!(preview
        .blocked_reason
        .as_deref()
        .is_some_and(|reason| reason.contains("inspect-only")));
}

#[test]
fn stash_snapshot_identity_uses_the_stable_object_id_not_the_moving_selector() {
    let dir = build_case_root("with_stash");
    let expected_oid = Command::new("git")
        .arg("-C")
        .arg(dir.path())
        .args(["rev-parse", "refs/stash"])
        .output()
        .expect("stash oid command launches");
    assert!(expected_oid.status.success());
    let expected_oid = String::from_utf8(expected_oid.stdout)
        .expect("stash oid is UTF-8")
        .trim()
        .to_string();
    let expected_epoch = Command::new("git")
        .arg("-C")
        .arg(dir.path())
        .args(["show", "-s", "--format=%ct", "refs/stash"])
        .output()
        .expect("stash timestamp command launches");
    assert!(expected_epoch.status.success());
    let expected_epoch = String::from_utf8(expected_epoch.stdout)
        .expect("stash timestamp is UTF-8")
        .trim()
        .to_string();
    let request = DailyLoopRequest::for_worktree(
        dir.path(),
        DailyLoopOperationKind::StashCapture,
        Vec::new(),
    );

    let snapshot = DailyLoopService::default().snapshot(&request);

    assert_eq!(snapshot.stash_entries.len(), 1);
    assert_eq!(
        snapshot.stash_entries[0].stash_entry_id,
        format!("git.stash.object.{expected_oid}")
    );
    assert_eq!(
        snapshot.stash_entries[0].minted_at,
        format!("{expected_epoch}Z")
    );
    assert_eq!(
        snapshot.stash_entries[0].updated_at,
        snapshot.stash_entries[0].minted_at
    );
    assert_ne!(snapshot.stash_entries[0].stash_entry_id, "stash@{0}");
}

#[test]
fn apply_actor_must_match_the_actor_bound_into_review_authority() {
    let dir = build_case_root("attached_dirty");
    let mut request = DailyLoopRequest::for_worktree(
        dir.path(),
        DailyLoopOperationKind::Stage,
        vec![PathBuf::from("untracked.txt")],
    );
    request.actor_ref = "actor:reviewer".to_string();
    let service = DailyLoopService::default();
    let preview = service.preview(&request);
    assert_eq!(preview.state, DailyLoopPreviewState::Ready);

    let result = service.apply(&preview, "actor:different");

    assert_eq!(result.outcome.as_str(), "blocked_no_changes_made");
    assert!(result
        .outcome_reason
        .as_deref()
        .is_some_and(|reason| reason.contains("apply actor does not match")));
    let status = Command::new("git")
        .arg("-C")
        .arg(dir.path())
        .args(["status", "--porcelain=v1", "--", "untracked.txt"])
        .output()
        .expect("status launches");
    assert_eq!(
        String::from_utf8_lossy(&status.stdout),
        "?? untracked.txt\n"
    );
}

#[test]
fn linked_worktree_keeps_common_repository_identity_and_exact_worktree_identity() {
    let repository = build_case_root("with_history");
    let linked_parent = tempfile::tempdir().expect("linked parent");
    let linked_root = linked_parent.path().join("linked-worktree");
    let linked_root_text = linked_root.to_string_lossy().into_owned();
    run_git(
        repository.path(),
        &[
            "worktree",
            "add",
            "-q",
            "-b",
            "linked-test",
            &linked_root_text,
        ],
    );

    let mut request =
        DailyLoopRequest::for_worktree(&linked_root, DailyLoopOperationKind::Status, vec![]);
    request.target.repo.repo_root = repository.path().to_path_buf();
    request.target.repo.git_dir = repository.path().join(".git");
    let snapshot = DailyLoopService::default().snapshot(&request);

    assert_eq!(snapshot.state, DailyLoopSnapshotState::Current);
    assert_eq!(
        snapshot.target.repo.repo_root,
        fs::canonicalize(repository.path()).expect("canonical repository root")
    );
    assert_eq!(
        snapshot.target.worktree.worktree_root,
        fs::canonicalize(&linked_root).expect("canonical linked-worktree root")
    );
    assert!(snapshot.target.worktree.is_linked);
    assert_eq!(
        snapshot.target.repo.repo_ref,
        snapshot.target.worktree.repo_ref
    );
}

#[test]
fn backend_private_error_details_are_not_projected() {
    let dir = tempfile::tempdir().expect("tempdir");
    let request =
        DailyLoopRequest::for_worktree(dir.path(), DailyLoopOperationKind::Status, vec![]);
    let snapshot = DailyLoopService::new(PrivateFailureBackend).snapshot(&request);
    let reason = snapshot.degraded_reason.expect("degraded reason");

    assert!(!reason.contains("private"));
    assert!(!reason.contains("token-value"));
    assert!(reason.contains("io"));
}
