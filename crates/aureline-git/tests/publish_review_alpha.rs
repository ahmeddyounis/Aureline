//! Fixture-driven coverage for preview-first Git publish flows.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use aureline_git::{
    GitPublishBackend, GitPublishBackendError, GitPublishCommandOutput, GitPublishMode,
    GitPublishOriginScope, GitPublishOutcomeState, GitPublishPreviewState, GitPublishRemoteState,
    GitPublishRequest, GitPublishRouteClass, GitPublishService,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PublishFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    setup_mode: String,
    after_preview_mutation: Option<String>,
    mode: GitPublishMode,
    remote_name: Option<String>,
    local_branch: Option<String>,
    target_branch: Option<String>,
    force_review_acknowledged: bool,
    expected_remote_oid: Option<String>,
    origin_scope: GitPublishOriginScope,
    route_class: GitPublishRouteClass,
    expected: ExpectedPublish,
}

#[derive(Debug, Deserialize)]
struct ExpectedPublish {
    preview_state: String,
    remote_state: String,
    route_class: String,
    origin_scope: String,
    route_disclosed: bool,
    target_disclosed: bool,
    result_state: String,
    activity_state_class: String,
    support_export_phase: String,
    local_state_preserved: bool,
    same_review_reopen_available: bool,
    merge_queue_supported: bool,
    review_platform_state: String,
    remote_ref_matches_local_after_apply: bool,
}

#[derive(Debug, Clone, Copy)]
struct RevListFailureBackend;

impl GitPublishBackend for RevListFailureBackend {
    fn run_git(
        &self,
        root: &Path,
        args: &[String],
    ) -> Result<GitPublishCommandOutput, GitPublishBackendError> {
        if args.first().is_some_and(|arg| arg == "rev-list") {
            return Ok(GitPublishCommandOutput {
                success: false,
                status_code: Some(128),
                stdout: Vec::new(),
                stderr: b"forced divergence inspection failure".to_vec(),
            });
        }
        let output = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(args)
            .output()
            .map_err(|_| GitPublishBackendError {
                message: "test Git command failed to launch".to_string(),
            })?;
        Ok(GitPublishCommandOutput {
            success: output.status.success(),
            status_code: output.status.code(),
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/git/publish_review_alpha")
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
    String::from_utf8_lossy(&output.stdout).trim().to_string()
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

fn add_local_commit(root: &Path, subject: &str, value: u32) {
    fs::write(
        root.join("src/lib.rs"),
        format!("pub fn answer() -> u32 {{\n    {value}\n}}\n"),
    )
    .expect("write local source");
    run_git(root, &["add", "src/lib.rs"]);
    run_git(root, &["commit", "-q", "-m", subject]);
}

fn init_bare_remote(parent: &Path) -> PathBuf {
    let remote = parent.join("remote.git");
    let status = Command::new("git")
        .args(["init", "--bare", "-q"])
        .arg(&remote)
        .status()
        .expect("git init bare launches");
    assert!(status.success(), "git init --bare failed");
    remote
}

fn build_case_root(mode: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    match mode {
        "upstream_one_ahead" => {
            seed_committed_repo(dir.path());
            let remote = init_bare_remote(dir.path());
            run_git(
                dir.path(),
                &["remote", "add", "origin", remote.to_str().unwrap()],
            );
            run_git(dir.path(), &["push", "-q", "-u", "origin", "main"]);
            add_local_commit(dir.path(), "local publish candidate", 2);
        }
        "missing_remote" => seed_committed_repo(dir.path()),
        "explicit_remote_new_branch" => {
            seed_committed_repo(dir.path());
            let remote = init_bare_remote(dir.path());
            run_git(
                dir.path(),
                &["remote", "add", "origin", remote.to_str().unwrap()],
            );
        }
        other => panic!("unsupported setup_mode: {other}"),
    }
    dir
}

fn request_for_fixture(fixture: &PublishFixture, root: &Path) -> GitPublishRequest {
    let mut request = GitPublishRequest::with_observed_at(
        format!("workspace.fixture.{}", fixture.case_name),
        root,
        "2026-05-13T00:00:00Z",
    )
    .with_mode(fixture.mode)
    .with_route(fixture.origin_scope, fixture.route_class)
    .with_launch_source_ref(format!("git.publish.sheet.{}", fixture.case_name));
    if let Some(remote_name) = &fixture.remote_name {
        request = request.with_remote_name(remote_name.clone());
    }
    if let Some(local_branch) = &fixture.local_branch {
        request = request.with_local_branch(local_branch.clone());
    }
    if let Some(target_branch) = &fixture.target_branch {
        request = request.with_target_branch(target_branch.clone());
    }
    if fixture.force_review_acknowledged {
        request = request.acknowledge_force_review();
    }
    if let Some(expected_remote_oid) = &fixture.expected_remote_oid {
        let expected_remote_oid = if expected_remote_oid == "$REMOTE_MAIN" {
            git_output(root, &["rev-parse", "refs/remotes/origin/main"])
        } else {
            expected_remote_oid.clone()
        };
        request = request.with_expected_remote_oid(expected_remote_oid);
    }
    request
}

fn parse_preview_state(value: &str) -> GitPublishPreviewState {
    match value {
        "ready_to_publish" => GitPublishPreviewState::ReadyToPublish,
        "blocked" => GitPublishPreviewState::Blocked,
        "degraded" => GitPublishPreviewState::Degraded,
        other => panic!("unsupported preview state: {other}"),
    }
}

fn parse_result_state(value: &str) -> GitPublishOutcomeState {
    match value {
        "published" => GitPublishOutcomeState::Published,
        "blocked_no_changes_made" => GitPublishOutcomeState::BlockedNoChangesMade,
        "failed" => GitPublishOutcomeState::Failed,
        other => panic!("unsupported result state: {other}"),
    }
}

fn parse_remote_state(value: &str) -> GitPublishRemoteState {
    match value {
        "existing_remote_ref" => GitPublishRemoteState::ExistingRemoteRef,
        "new_remote_ref" => GitPublishRemoteState::NewRemoteRef,
        "remote_missing" => GitPublishRemoteState::RemoteMissing,
        "local_ref_missing" => GitPublishRemoteState::LocalRefMissing,
        "invalid_target" => GitPublishRemoteState::InvalidTarget,
        "unknown" => GitPublishRemoteState::Unknown,
        other => panic!("unsupported remote state: {other}"),
    }
}

fn apply_after_preview_mutation(root: &Path, mutation: Option<&str>) {
    match mutation {
        Some("set_remote_url_missing") => {
            let missing = root.join("missing-remote.git");
            run_git(
                root,
                &["remote", "set-url", "origin", missing.to_str().unwrap()],
            );
        }
        Some("replace_remote_with_non_bare_directory") => {
            let remote = root.join("remote.git");
            let preserved = root.join("remote-storage.git");
            fs::rename(&remote, preserved).expect("preserve reviewed remote storage");
            fs::create_dir(&remote).expect("replace remote with rejecting non-bare directory");
        }
        Some(other) => panic!("unsupported after_preview_mutation: {other}"),
        None => {}
    }
}

fn remote_main_oid_if_present(root: &Path) -> Option<String> {
    let remote_url = git_output(root, &["remote", "get-url", "origin"]);
    let output = Command::new("git")
        .arg("-C")
        .arg(remote_url)
        .args(["rev-parse", "refs/heads/main"])
        .output()
        .expect("git rev-parse remote launches");
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn run_fixture(path: &Path) {
    let text = fs::read_to_string(path).expect("read fixture");
    let fixture: PublishFixture = serde_yaml::from_str(&text).expect("parse fixture");
    assert_eq!(fixture.record_kind, "git_publish_review_alpha_case");
    assert_eq!(fixture.schema_version, 1);

    let dir = build_case_root(&fixture.setup_mode);
    let request = request_for_fixture(&fixture, dir.path());
    let service = GitPublishService::default();
    let preview = service.preview(&request);

    assert_eq!(
        preview.preview_state,
        parse_preview_state(&fixture.expected.preview_state),
        "{}: preview state",
        fixture.case_name
    );
    assert_eq!(
        preview.target.remote_state,
        parse_remote_state(&fixture.expected.remote_state),
        "{}: remote state",
        fixture.case_name
    );
    assert_eq!(
        preview.route.route_class.as_str(),
        fixture.expected.route_class,
        "{}: route class",
        fixture.case_name
    );
    assert_eq!(
        preview.route.origin_scope.as_str(),
        fixture.expected.origin_scope,
        "{}: origin scope",
        fixture.case_name
    );
    assert_eq!(
        preview.route.route_disclosed, fixture.expected.route_disclosed,
        "{}: route disclosed",
        fixture.case_name
    );
    assert_eq!(
        preview.target.target_disclosed, fixture.expected.target_disclosed,
        "{}: target disclosed",
        fixture.case_name
    );
    assert_eq!(
        preview.merge_queue_supported, fixture.expected.merge_queue_supported,
        "{}: merge queue support",
        fixture.case_name
    );
    assert_eq!(
        preview.review_platform_state, fixture.expected.review_platform_state,
        "{}: review platform state",
        fixture.case_name
    );
    if preview.preview_state == GitPublishPreviewState::ReadyToPublish {
        assert!(
            preview.ready_to_publish(),
            "{}: preview is ready to publish",
            fixture.case_name
        );
    }

    let local_head_before_apply = git_output(dir.path(), &["rev-parse", "HEAD"]);
    apply_after_preview_mutation(dir.path(), fixture.after_preview_mutation.as_deref());
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
        "{}: support export phase",
        fixture.case_name
    );
    assert_eq!(
        result.failure_recovery.local_state_preserved, fixture.expected.local_state_preserved,
        "{}: local state preserved",
        fixture.case_name
    );
    assert_eq!(
        result.failure_recovery.same_review_reopen_available,
        fixture.expected.same_review_reopen_available,
        "{}: same review reopen",
        fixture.case_name
    );
    assert!(
        result.failure_can_reopen_review(),
        "{}: failed publish has a reopenable review",
        fixture.case_name
    );
    assert_eq!(
        git_output(dir.path(), &["rev-parse", "HEAD"]),
        local_head_before_apply,
        "{}: local HEAD preserved",
        fixture.case_name
    );

    if fixture.expected.remote_ref_matches_local_after_apply {
        assert_eq!(
            remote_main_oid_if_present(dir.path()).as_deref(),
            Some(local_head_before_apply.as_str()),
            "{}: remote main matches local HEAD",
            fixture.case_name
        );
    }
}

#[test]
fn publish_review_alpha_fixtures() {
    let mut fixture_paths = fs::read_dir(fixtures_dir())
        .expect("read fixture directory")
        .map(|entry| entry.expect("fixture entry").path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "yaml")
        })
        .collect::<Vec<_>>();
    fixture_paths.sort();
    assert!(!fixture_paths.is_empty(), "publish fixtures exist");
    for path in fixture_paths {
        run_fixture(&path);
    }
}

#[test]
fn exported_tampered_and_route_stale_previews_cannot_publish() {
    let dir = build_case_root("upstream_one_ahead");
    let request = GitPublishRequest::with_observed_at(
        "workspace.fixture.authority",
        dir.path(),
        "2026-05-13T00:20:00Z",
    );
    let service = GitPublishService::default();
    let preview = service.preview(&request);
    assert!(preview.ready_to_publish());

    let exported = serde_json::to_string(&preview).expect("serialize inspection record");
    let restored = serde_json::from_str(&exported).expect("deserialize inspection record");
    assert_eq!(
        GitPublishService::default()
            .apply(&restored, "2026-05-13T00:20:01Z")
            .outcome_state,
        GitPublishOutcomeState::BlockedNoChangesMade
    );

    let mut tampered = preview.clone();
    tampered.workspace_ref = "workspace.tampered".to_string();
    assert_eq!(
        service
            .apply(&tampered, "2026-05-13T00:20:02Z")
            .outcome_state,
        GitPublishOutcomeState::BlockedNoChangesMade
    );

    let fresh = service.preview(&request);
    let missing = dir.path().join("different-remote.git");
    run_git(
        dir.path(),
        &["remote", "set-url", "origin", missing.to_str().unwrap()],
    );
    assert_eq!(
        service.apply(&fresh, "2026-05-13T00:20:03Z").outcome_state,
        GitPublishOutcomeState::BlockedNoChangesMade
    );
}

#[test]
fn multiple_push_destinations_are_blocked_before_network_mutation() {
    let dir = build_case_root("explicit_remote_new_branch");
    let second = dir.path().join("second.git");
    assert!(Command::new("git")
        .args(["init", "--bare", "-q"])
        .arg(&second)
        .status()
        .expect("init second bare remote")
        .success());
    let third = dir.path().join("third.git");
    run_git(
        dir.path(),
        &[
            "remote",
            "set-url",
            "--add",
            "--push",
            "origin",
            second.to_str().unwrap(),
        ],
    );
    run_git(
        dir.path(),
        &[
            "remote",
            "set-url",
            "--add",
            "--push",
            "origin",
            third.to_str().unwrap(),
        ],
    );
    let preview = GitPublishService::default().preview(
        &GitPublishRequest::with_observed_at(
            "workspace.fixture.multiple-push-urls",
            dir.path(),
            "2026-05-13T00:30:00Z",
        )
        .with_remote_name("origin")
        .with_local_branch("main")
        .with_target_branch("main"),
    );
    assert_eq!(preview.preview_state, GitPublishPreviewState::Blocked);
    assert!(!preview.ready_to_publish());
}

#[test]
fn divergence_inspection_failure_blocks_publish() {
    let dir = build_case_root("upstream_one_ahead");
    let preview = GitPublishService::new(RevListFailureBackend).preview(
        &GitPublishRequest::with_observed_at(
            "workspace.fixture.divergence-failure",
            dir.path(),
            "2026-05-13T00:35:00Z",
        ),
    );
    assert_eq!(preview.preview_state, GitPublishPreviewState::Blocked);
    assert_eq!(preview.target.behind_count, None);
    assert_eq!(preview.target.ahead_count, None);
    assert!(preview
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("divergence could not be verified")));
}

#[test]
fn unsupported_remote_helpers_are_blocked_before_publish() {
    let dir = build_case_root("explicit_remote_new_branch");
    run_git(
        dir.path(),
        &["remote", "set-url", "origin", "ext::hostile-helper"],
    );
    let preview = GitPublishService::default().preview(
        &GitPublishRequest::with_observed_at(
            "workspace.fixture.remote-helper",
            dir.path(),
            "2026-05-13T00:36:00Z",
        )
        .with_remote_name("origin")
        .with_local_branch("main")
        .with_target_branch("main"),
    );
    assert_eq!(preview.preview_state, GitPublishPreviewState::Blocked);
    assert!(preview
        .blocked_reasons
        .iter()
        .any(|reason| reason.contains("URL protocol is not admitted")));
}

#[test]
fn repository_config_drift_blocks_before_remote_mutation() {
    let dir = build_case_root("upstream_one_ahead");
    let service = GitPublishService::default();
    let preview = service.preview(&GitPublishRequest::with_observed_at(
        "workspace.fixture.publish-config-drift",
        dir.path(),
        "2026-05-13T00:37:00Z",
    ));
    assert!(preview.ready_to_publish(), "{preview:#?}");
    let remote_before = remote_main_oid_if_present(dir.path());

    run_git(dir.path(), &["config", "status.relativePaths", "false"]);
    let result = service.apply(&preview, "2026-05-13T00:37:01Z");
    assert_eq!(
        result.outcome_state,
        GitPublishOutcomeState::BlockedNoChangesMade
    );
    assert_eq!(remote_main_oid_if_present(dir.path()), remote_before);
}

#[cfg(unix)]
#[test]
fn publish_apply_does_not_execute_repository_pre_push_hook() {
    use std::os::unix::fs::PermissionsExt;

    let dir = build_case_root("upstream_one_ahead");
    let sentinel = dir.path().join("hook-ran");
    let hook = dir.path().join(".git/hooks/pre-push");
    fs::write(
        &hook,
        format!("#!/bin/sh\ntouch '{}'\n", sentinel.display()),
    )
    .expect("write hook");
    let mut permissions = fs::metadata(&hook).expect("hook metadata").permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(&hook, permissions).expect("make hook executable");

    let service = GitPublishService::default();
    let preview = service.preview(&GitPublishRequest::with_observed_at(
        "workspace.fixture.hook",
        dir.path(),
        "2026-05-13T00:40:00Z",
    ));
    assert_eq!(
        service
            .apply(&preview, "2026-05-13T00:40:01Z")
            .outcome_state,
        GitPublishOutcomeState::Published
    );
    assert!(!sentinel.exists(), "repository hook must not execute");
}
