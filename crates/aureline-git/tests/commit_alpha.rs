//! Fixture-driven coverage for local Git commit flows.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use aureline_git::{
    GitCommitAuthorInput, GitCommitAuthorState, GitCommitBackend, GitCommitBackendError,
    GitCommitCommandOutput, GitCommitMode, GitCommitOutcomeState, GitCommitPreviewState,
    GitCommitRequest, GitCommitService,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CommitFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    setup_mode: String,
    mode: GitCommitMode,
    message: String,
    author: AuthorFixture,
    history_guardrail_acknowledged: bool,
    squash_target: Option<String>,
    expected: ExpectedCommit,
}

#[derive(Debug, Deserialize)]
struct AuthorFixture {
    source: String,
    display_name: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ExpectedCommit {
    preview_state: String,
    author_state: String,
    guardrail_state: String,
    explicit_ack_required: bool,
    staged_count: usize,
    remaining_worktree_count: usize,
    result_state: String,
    commit_count_after_apply: usize,
    remaining_status_count_after_apply: usize,
    final_subject_prefix: Option<String>,
    publish_queue_state: String,
    activity_state_class: String,
    support_export_phase: String,
    journal_source_class: String,
}

#[derive(Debug, Clone, Default)]
struct InspectingGitBackend {
    calls: Arc<Mutex<Vec<Vec<String>>>>,
    stage_calls: Arc<AtomicUsize>,
    corrupt_stage_at: Option<usize>,
}

impl InspectingGitBackend {
    fn with_corrupt_stage_at(corrupt_stage_at: usize) -> Self {
        Self {
            corrupt_stage_at: Some(corrupt_stage_at),
            ..Self::default()
        }
    }
}

impl GitCommitBackend for InspectingGitBackend {
    fn run_git(
        &self,
        root: &Path,
        args: &[String],
    ) -> Result<GitCommitCommandOutput, GitCommitBackendError> {
        self.calls.lock().expect("calls lock").push(args.to_vec());
        let output = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(args)
            .output()
            .map_err(|_| GitCommitBackendError {
                message: "test Git backend failed".to_string(),
            })?;
        let mut stdout = output.stdout;
        if args.first().is_some_and(|arg| arg == "ls-files")
            && args.iter().any(|arg| arg == "--stage")
        {
            let stage_call = self.stage_calls.fetch_add(1, Ordering::SeqCst) + 1;
            if self
                .corrupt_stage_at
                .is_some_and(|corrupt_at| stage_call >= corrupt_at)
            {
                stdout.extend_from_slice(b"tampered-logical-stage\0");
            }
        }
        Ok(GitCommitCommandOutput {
            success: output.status.success(),
            status_code: output.status.code(),
            stdout,
            stderr: output.stderr,
        })
    }
}

#[derive(Debug, Clone, Default)]
struct CommitIndexRaceBackend {
    injected: Arc<AtomicBool>,
}

impl GitCommitBackend for CommitIndexRaceBackend {
    fn run_git(
        &self,
        root: &Path,
        args: &[String],
    ) -> Result<GitCommitCommandOutput, GitCommitBackendError> {
        if args.iter().any(|arg| arg == "commit") && !self.injected.swap(true, Ordering::SeqCst) {
            fs::write(root.join("raced.txt"), "unreviewed staged bytes\n")
                .expect("inject raced file");
            run_git(root, &["add", "raced.txt"]);
        }
        let output = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(args)
            .output()
            .map_err(|_| GitCommitBackendError {
                message: "test Git backend failed".to_string(),
            })?;
        Ok(GitCommitCommandOutput {
            success: output.status.success(),
            status_code: output.status.code(),
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/git/commit_alpha")
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

fn git_output(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .expect("git command launches");
    assert!(
        output.status.success(),
        "git {args:?} failed in {}: {}",
        root.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).to_string()
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
    fs::write(root.join("README.md"), "fixture\n").expect("write readme");
    run_git(root, &["add", "src/lib.rs", "README.md"]);
    run_git(root, &["commit", "-q", "-m", "initial fixture commit"]);
}

fn build_case_root(mode: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    match mode {
        "staged_with_unstaged" => {
            seed_committed_repo(dir.path());
            fs::write(
                dir.path().join("src/lib.rs"),
                "pub fn answer() -> u32 {\n    42\n}\n",
            )
            .expect("modify staged source");
            run_git(dir.path(), &["add", "src/lib.rs"]);
            fs::write(dir.path().join("README.md"), "fixture\nunstaged\n")
                .expect("modify unstaged readme");
        }
        "staged_only" => {
            seed_committed_repo(dir.path());
            fs::write(
                dir.path().join("src/lib.rs"),
                "pub fn answer() -> u32 {\n    42\n}\n",
            )
            .expect("modify source");
            run_git(dir.path(), &["add", "src/lib.rs"]);
        }
        "two_commits_staged" => {
            seed_committed_repo(dir.path());
            fs::write(dir.path().join("src/other.rs"), "pub fn other() {}\n")
                .expect("write second file");
            run_git(dir.path(), &["add", "src/other.rs"]);
            run_git(dir.path(), &["commit", "-q", "-m", "second fixture commit"]);
            fs::write(
                dir.path().join("src/lib.rs"),
                "pub fn answer() -> u32 {\n    100\n}\n",
            )
            .expect("modify source");
            run_git(dir.path(), &["add", "src/lib.rs"]);
        }
        other => panic!("unsupported setup_mode: {other}"),
    }
    dir
}

fn request_for_fixture(fixture: &CommitFixture, root: &Path) -> GitCommitRequest {
    let mut request = GitCommitRequest::with_observed_at(
        format!("workspace.fixture.{}", fixture.case_name),
        root,
        fixture.mode,
        fixture.message.clone(),
        "2026-05-13T00:00:00Z",
    )
    .with_launch_source_ref(format!("git.commit.sheet.{}", fixture.case_name));
    request = match fixture.author.source.as_str() {
        "repository_config" => request,
        "explicit" => request.with_author(GitCommitAuthorInput {
            display_name: fixture.author.display_name.clone(),
            email: fixture.author.email.clone(),
        }),
        other => panic!("unsupported author source: {other}"),
    };
    if fixture.history_guardrail_acknowledged {
        request = request.acknowledge_history_guardrail();
    }
    if let Some(target) = &fixture.squash_target {
        request = request.with_squash_target(target.clone());
    }
    request
}

fn parse_preview_state(value: &str) -> GitCommitPreviewState {
    match value {
        "ready_to_commit" => GitCommitPreviewState::ReadyToCommit,
        "blocked" => GitCommitPreviewState::Blocked,
        "degraded" => GitCommitPreviewState::Degraded,
        other => panic!("unsupported preview state: {other}"),
    }
}

fn parse_author_state(value: &str) -> GitCommitAuthorState {
    match value {
        "resolved" => GitCommitAuthorState::Resolved,
        "missing" => GitCommitAuthorState::Missing,
        "invalid" => GitCommitAuthorState::Invalid,
        other => panic!("unsupported author state: {other}"),
    }
}

fn parse_result_state(value: &str) -> GitCommitOutcomeState {
    match value {
        "committed" => GitCommitOutcomeState::Committed,
        "blocked_no_changes_made" => GitCommitOutcomeState::BlockedNoChangesMade,
        "failed" => GitCommitOutcomeState::Failed,
        other => panic!("unsupported result state: {other}"),
    }
}

fn commit_count(root: &Path) -> usize {
    git_output(root, &["rev-list", "--count", "HEAD"])
        .trim()
        .parse()
        .expect("commit count parses")
}

fn status_line_count(root: &Path) -> usize {
    git_output(root, &["status", "--porcelain=v1"])
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count()
}

fn head_subject(root: &Path) -> String {
    git_output(root, &["log", "-1", "--pretty=%s"])
        .trim()
        .to_string()
}

fn run_fixture(path: &Path) {
    let text = fs::read_to_string(path).expect("read fixture");
    let fixture: CommitFixture = serde_yaml::from_str(&text).expect("parse fixture");
    assert_eq!(fixture.record_kind, "git_commit_alpha_case");
    assert_eq!(fixture.schema_version, 1);

    let dir = build_case_root(&fixture.setup_mode);
    let request = request_for_fixture(&fixture, dir.path());
    let service = GitCommitService::default();
    let preview = service.preview(&request);

    assert_eq!(
        preview.preview_state,
        parse_preview_state(&fixture.expected.preview_state),
        "{}: preview state",
        fixture.case_name
    );
    assert_eq!(
        preview.author.state,
        parse_author_state(&fixture.expected.author_state),
        "{}: author state",
        fixture.case_name
    );
    assert_eq!(
        preview.history_guardrail.guardrail_state, fixture.expected.guardrail_state,
        "{}: guardrail state",
        fixture.case_name
    );
    assert_eq!(
        preview.history_guardrail.explicit_ack_required, fixture.expected.explicit_ack_required,
        "{}: explicit ack required",
        fixture.case_name
    );
    assert_eq!(
        preview.scope.staged_count, fixture.expected.staged_count,
        "{}: staged count",
        fixture.case_name
    );
    assert_eq!(
        preview.scope.remaining_worktree_path_labels.len(),
        fixture.expected.remaining_worktree_count,
        "{}: remaining worktree count",
        fixture.case_name
    );
    if preview.preview_state == GitCommitPreviewState::ReadyToCommit {
        assert!(
            preview.ready_to_commit(),
            "{}: preview is ready to commit",
            fixture.case_name
        );
    }
    assert!(preview.publish_readiness.local_only_commit);
    assert!(!preview.publish_readiness.publish_now_supported);

    let preflight_head = preview.history_guardrail.preflight_head_oid.clone();
    let result = service.apply(&preview, "2026-05-13T00:00:01Z");
    assert_eq!(
        result.outcome_state,
        parse_result_state(&fixture.expected.result_state),
        "{}: result state",
        fixture.case_name
    );
    assert_eq!(
        result.publish_readiness.queue_state, fixture.expected.publish_queue_state,
        "{}: publish queue state",
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
        result.commit_journal.source_class, fixture.expected.journal_source_class,
        "{}: journal source class",
        fixture.case_name
    );
    assert!(
        result.attribution_is_exportable(),
        "{}: activity and support attribution",
        fixture.case_name
    );
    assert_eq!(
        commit_count(dir.path()),
        fixture.expected.commit_count_after_apply,
        "{}: commit count",
        fixture.case_name
    );
    assert_eq!(
        status_line_count(dir.path()),
        fixture.expected.remaining_status_count_after_apply,
        "{}: status line count",
        fixture.case_name
    );
    if let Some(prefix) = &fixture.expected.final_subject_prefix {
        assert!(
            head_subject(dir.path()).starts_with(prefix),
            "{}: final subject prefix",
            fixture.case_name
        );
    }
    if fixture.mode == GitCommitMode::Amend
        && fixture.expected.result_state == "committed"
        && preflight_head.is_some()
    {
        assert_ne!(
            result.commit_oid, preflight_head,
            "{}: amend changed HEAD",
            fixture.case_name
        );
    }
    if fixture.mode == GitCommitMode::Squash && fixture.expected.result_state == "committed" {
        assert!(
            result.history_guardrail.deferred_squash_marker,
            "{}: squash marker is explicit",
            fixture.case_name
        );
    }
}

#[test]
fn protected_commit_alpha_fixtures_match_git_service_contract() {
    let mut fixtures: Vec<_> = fs::read_dir(fixtures_dir())
        .expect("fixture directory")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "yaml"))
        .collect();
    fixtures.sort();
    assert!(!fixtures.is_empty(), "commit fixtures must exist");

    for path in fixtures {
        run_fixture(&path);
    }
}

#[test]
fn staged_scope_drift_blocks_apply_without_creating_commit() {
    let dir = build_case_root("staged_only");
    let service = GitCommitService::default();
    let request = GitCommitRequest::with_observed_at(
        "workspace.fixture.scope-drift",
        dir.path(),
        GitCommitMode::Normal,
        "commit before drift",
        "2026-05-13T00:10:00Z",
    );
    let preview = service.preview(&request);
    assert!(preview.ready_to_commit());

    fs::write(
        dir.path().join("src/lib.rs"),
        "pub fn answer() -> u32 {\n    7\n}\n",
    )
    .expect("drift staged source");
    run_git(dir.path(), &["add", "src/lib.rs"]);

    let result = service.apply(&preview, "2026-05-13T00:10:01Z");
    assert_eq!(
        result.outcome_state,
        GitCommitOutcomeState::BlockedNoChangesMade
    );
    assert_eq!(commit_count(dir.path()), 1);
    assert!(result
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("staged scope drifted")));
}

#[test]
fn preview_is_non_mutating_and_exported_or_tampered_records_have_no_authority() {
    let dir = build_case_root("staged_only");
    let service = GitCommitService::default();
    let request = GitCommitRequest::with_observed_at(
        "workspace.fixture.authority",
        dir.path(),
        GitCommitMode::Normal,
        "reviewed commit",
        "2026-05-13T00:20:00Z",
    );
    let objects_before = git_output(dir.path(), &["count-objects", "-v"]);
    let preview = service.preview(&request);
    assert!(preview.ready_to_commit());
    assert_eq!(
        git_output(dir.path(), &["count-objects", "-v"]),
        objects_before,
        "preview must not write tree objects"
    );

    let exported = serde_json::to_string(&preview).expect("serialize inspection record");
    let restored = serde_json::from_str(&exported).expect("deserialize inspection record");
    assert_eq!(
        GitCommitService::default()
            .apply(&restored, "2026-05-13T00:20:01Z")
            .outcome_state,
        GitCommitOutcomeState::BlockedNoChangesMade
    );

    let mut tampered = preview.clone();
    tampered.workspace_ref = "workspace.tampered".to_string();
    assert_eq!(
        service
            .apply(&tampered, "2026-05-13T00:20:02Z")
            .outcome_state,
        GitCommitOutcomeState::BlockedNoChangesMade
    );
    assert_eq!(commit_count(dir.path()), 1);
}

#[test]
fn source_head_drift_blocks_even_when_index_bytes_are_unchanged() {
    let dir = build_case_root("two_commits_staged");
    let service = GitCommitService::default();
    let request = GitCommitRequest::with_observed_at(
        "workspace.fixture.head-drift",
        dir.path(),
        GitCommitMode::Normal,
        "must not land on a different head",
        "2026-05-13T00:30:00Z",
    );
    let preview = service.preview(&request);
    assert!(preview.ready_to_commit());
    let parent = git_output(dir.path(), &["rev-parse", "HEAD~1"]);
    run_git(
        dir.path(),
        &["update-ref", "refs/heads/main", parent.trim()],
    );

    assert_eq!(
        service
            .apply(&preview, "2026-05-13T00:30:01Z")
            .outcome_state,
        GitCommitOutcomeState::BlockedNoChangesMade
    );
}

#[test]
fn last_moment_index_race_fails_the_postcondition_after_commit() {
    let dir = build_case_root("staged_only");
    let service = GitCommitService::new(CommitIndexRaceBackend::default());
    let preview = service.preview(&GitCommitRequest::with_observed_at(
        "workspace.fixture.index-race",
        dir.path(),
        GitCommitMode::Normal,
        "reviewed staged content only",
        "2026-05-13T00:35:00Z",
    ));
    assert!(preview.ready_to_commit(), "{preview:#?}");

    let result = service.apply(&preview, "2026-05-13T00:35:01Z");
    assert_eq!(result.outcome_state, GitCommitOutcomeState::Failed);
    assert_eq!(
        commit_count(dir.path()),
        2,
        "Git completed the raced commit"
    );
    assert_eq!(
        git_output(dir.path(), &["show", "HEAD:raced.txt"]),
        "unreviewed staged bytes\n",
        "the failed result must disclose a recoverable post-mutation race"
    );
    assert!(result
        .failure_reason
        .as_deref()
        .is_some_and(|reason| reason.contains("postcondition")));
}

#[cfg(unix)]
#[test]
fn commit_apply_does_not_execute_repository_hooks() {
    use std::os::unix::fs::PermissionsExt;

    let dir = build_case_root("staged_only");
    let sentinel = dir.path().join("hook-ran");
    for hook_name in ["pre-commit", "post-commit"] {
        let hook = dir.path().join(".git/hooks").join(hook_name);
        fs::write(
            &hook,
            format!("#!/bin/sh\ntouch '{}'\n", sentinel.display()),
        )
        .expect("write hook");
        let mut permissions = fs::metadata(&hook).expect("hook metadata").permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(&hook, permissions).expect("make hook executable");
    }

    let service = GitCommitService::default();
    let preview = service.preview(&GitCommitRequest::with_observed_at(
        "workspace.fixture.hook",
        dir.path(),
        GitCommitMode::Normal,
        "hook-free commit",
        "2026-05-13T00:40:00Z",
    ));
    assert_eq!(
        service
            .apply(&preview, "2026-05-13T00:40:01Z")
            .outcome_state,
        GitCommitOutcomeState::Committed
    );
    assert!(!sentinel.exists(), "repository hooks must not execute");
}

#[test]
fn split_index_logical_stage_is_reviewed_and_committed() {
    let dir = build_case_root("staged_only");
    run_git(dir.path(), &["config", "core.splitIndex", "true"]);
    run_git(dir.path(), &["update-index", "--split-index"]);
    assert!(
        fs::read_dir(dir.path().join(".git"))
            .expect("read Git directory")
            .filter_map(Result::ok)
            .any(|entry| entry
                .file_name()
                .to_string_lossy()
                .starts_with("sharedindex.")),
        "fixture must use split-index storage"
    );

    let service = GitCommitService::default();
    let preview = service.preview(&GitCommitRequest::with_observed_at(
        "workspace.fixture.split-index",
        dir.path(),
        GitCommitMode::Normal,
        "commit split index",
        "2026-05-13T00:50:00Z",
    ));
    assert!(preview.ready_to_commit(), "{preview:#?}");
    assert_eq!(
        service
            .apply(&preview, "2026-05-13T00:50:01Z")
            .outcome_state,
        GitCommitOutcomeState::Committed
    );
}

#[test]
fn logical_stage_drift_blocks_even_when_raw_index_bytes_match() {
    let dir = build_case_root("staged_only");
    let index_path = dir.path().join(".git/index");
    let index_before = fs::read(&index_path).expect("read index before preview");
    let backend = InspectingGitBackend::with_corrupt_stage_at(3);
    let service = GitCommitService::new(backend);
    let preview = service.preview(&GitCommitRequest::with_observed_at(
        "workspace.fixture.logical-stage-drift",
        dir.path(),
        GitCommitMode::Normal,
        "must bind logical stage",
        "2026-05-13T00:55:00Z",
    ));
    assert!(preview.ready_to_commit(), "{preview:#?}");
    assert_eq!(
        fs::read(&index_path).expect("read stable index"),
        index_before
    );

    let result = service.apply(&preview, "2026-05-13T00:55:01Z");
    assert_eq!(
        result.outcome_state,
        GitCommitOutcomeState::BlockedNoChangesMade
    );
    assert_eq!(
        fs::read(index_path).expect("read unchanged index"),
        index_before
    );
    assert_eq!(commit_count(dir.path()), 1);
}

#[test]
fn squash_target_is_resolved_once_during_preview() {
    let dir = build_case_root("two_commits_staged");
    let backend = InspectingGitBackend::default();
    let service = GitCommitService::new(backend.clone());
    let preview = service.preview(
        &GitCommitRequest::with_observed_at(
            "workspace.fixture.single-squash-resolution",
            dir.path(),
            GitCommitMode::Squash,
            "follow-up details",
            "2026-05-13T01:00:00Z",
        )
        .acknowledge_history_guardrail()
        .with_squash_target("HEAD~1"),
    );
    assert!(preview.ready_to_commit(), "{preview:#?}");
    let squash_resolutions = backend
        .calls
        .lock()
        .expect("calls lock")
        .iter()
        .filter(|args| {
            args.first().is_some_and(|arg| arg == "rev-parse")
                && args.last().is_some_and(|arg| arg == "HEAD~1^{commit}")
        })
        .count();
    assert_eq!(squash_resolutions, 1);
}

#[test]
fn initial_commit_transitions_reviewed_unborn_head_to_attached_branch() {
    let dir = tempfile::tempdir().expect("tempdir");
    init_repo(dir.path());
    fs::write(dir.path().join("README.md"), "initial contents\n").expect("write initial file");
    run_git(dir.path(), &["add", "README.md"]);

    let service = GitCommitService::default();
    let preview = service.preview(&GitCommitRequest::with_observed_at(
        "workspace.fixture.initial-commit",
        dir.path(),
        GitCommitMode::Normal,
        "initial reviewed commit",
        "2026-05-13T01:10:00Z",
    ));
    assert!(preview.ready_to_commit(), "{preview:#?}");

    let result = service.apply(&preview, "2026-05-13T01:10:01Z");
    assert_eq!(result.outcome_state, GitCommitOutcomeState::Committed);
    assert_eq!(commit_count(dir.path()), 1);
    assert_eq!(
        git_output(dir.path(), &["symbolic-ref", "--short", "HEAD"]).trim(),
        "main"
    );
}

#[test]
fn merge_commit_verifies_every_reviewed_parent() {
    let dir = tempfile::tempdir().expect("tempdir");
    seed_committed_repo(dir.path());
    run_git(dir.path(), &["switch", "-q", "-c", "side"]);
    fs::write(dir.path().join("side.txt"), "side\n").expect("write side file");
    run_git(dir.path(), &["add", "side.txt"]);
    run_git(dir.path(), &["commit", "-q", "-m", "side commit"]);
    let side_oid = git_output(dir.path(), &["rev-parse", "HEAD"]);

    run_git(dir.path(), &["switch", "-q", "main"]);
    fs::write(dir.path().join("main.txt"), "main\n").expect("write main file");
    run_git(dir.path(), &["add", "main.txt"]);
    run_git(dir.path(), &["commit", "-q", "-m", "main commit"]);
    let main_oid = git_output(dir.path(), &["rev-parse", "HEAD"]);
    run_git(dir.path(), &["merge", "--no-commit", "side"]);

    let service = GitCommitService::default();
    let preview = service.preview(&GitCommitRequest::with_observed_at(
        "workspace.fixture.merge-parents",
        dir.path(),
        GitCommitMode::Normal,
        "reviewed merge commit",
        "2026-05-13T01:20:00Z",
    ));
    assert!(preview.ready_to_commit(), "{preview:#?}");
    let result = service.apply(&preview, "2026-05-13T01:20:01Z");
    assert_eq!(result.outcome_state, GitCommitOutcomeState::Committed);

    let parent_line = git_output(dir.path(), &["rev-list", "--parents", "-n", "1", "HEAD"]);
    let parents = parent_line.split_whitespace().skip(1).collect::<Vec<_>>();
    assert_eq!(parents, vec![main_oid.trim(), side_oid.trim()]);
}
