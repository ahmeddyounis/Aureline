//! Fixture-driven coverage for the Git status alpha service.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

use aureline_git::status::{ConsumerProjectionBundle, GitStatusRequest, GitStatusService};

#[derive(Debug, Deserialize)]
struct GitStatusFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    setup_mode: String,
    expected: ExpectedStatus,
}

#[derive(Debug, Deserialize)]
struct ExpectedStatus {
    service_state: String,
    branch_state: String,
    branch_label: Option<String>,
    staged_count: u32,
    unstaged_count: u32,
    untracked_count: u32,
    total_changed_count: u32,
    shell_current_claim_narrowed: bool,
    activity_partition: String,
    review_local_diff_authority: String,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/git/status_alpha")
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
        other => panic!("unsupported setup_mode: {other}"),
    }
    dir
}

fn run_fixture(path: &Path) {
    let text = fs::read_to_string(path).expect("read fixture");
    let fixture: GitStatusFixture = serde_yaml::from_str(&text).expect("parse fixture");
    assert_eq!(fixture.record_kind, "git_status_alpha_case");
    assert_eq!(fixture.schema_version, 1);

    let dir = build_case_root(&fixture.setup_mode);
    let request = GitStatusRequest::with_observed_at(
        format!("workspace.fixture.{}", fixture.case_name),
        dir.path(),
        "mono:git:fixture",
    );
    let snapshot = GitStatusService::default().snapshot(&request);
    let bundle = ConsumerProjectionBundle::from_snapshot("mono:git:bundle", &snapshot);

    assert_eq!(
        snapshot.service_state.as_str(),
        fixture.expected.service_state
    );
    assert_eq!(snapshot.head.state.as_str(), fixture.expected.branch_state);
    assert_eq!(snapshot.head.branch_label, fixture.expected.branch_label);
    assert_eq!(
        snapshot.change_summary.staged_count,
        fixture.expected.staged_count
    );
    assert_eq!(
        snapshot.change_summary.unstaged_count,
        fixture.expected.unstaged_count
    );
    assert_eq!(
        snapshot.change_summary.untracked_count,
        fixture.expected.untracked_count
    );
    assert_eq!(
        snapshot.change_summary.total_changed_count,
        fixture.expected.total_changed_count
    );
    assert_eq!(
        bundle.shell.current_claim_narrowed,
        fixture.expected.shell_current_claim_narrowed
    );
    assert_eq!(
        bundle.activity.partition,
        fixture.expected.activity_partition
    );
    assert_eq!(
        bundle.review.local_diff_authority,
        fixture.expected.review_local_diff_authority
    );
    assert_eq!(bundle.shell.truth_source_ref, bundle.truth_source_ref);
    assert_eq!(bundle.activity.truth_source_ref, bundle.truth_source_ref);
    assert_eq!(bundle.review.truth_source_ref, bundle.truth_source_ref);
}

#[test]
fn protected_status_alpha_fixtures_match_service_contract() {
    for entry in fs::read_dir(fixtures_dir()).expect("fixture dir") {
        let path = entry.expect("fixture entry").path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("yaml") {
            run_fixture(&path);
        }
    }
}

#[test]
fn missing_git_backend_degrades_without_disappearing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let request =
        GitStatusRequest::with_observed_at("workspace.fixture.git-missing", dir.path(), "mono:git");
    let service = GitStatusService::new(aureline_git::status::SystemGitStatusBackend::new(
        dir.path().join("missing-git-binary"),
    ));
    let snapshot = service.snapshot(&request);
    let bundle = ConsumerProjectionBundle::from_snapshot("mono:bundle", &snapshot);

    assert_eq!(snapshot.service_state.as_str(), "git_unavailable");
    assert!(bundle.shell.current_claim_narrowed);
    assert_eq!(bundle.activity.partition, "needs_attention");
    assert_eq!(bundle.review.local_diff_authority, "unavailable_local_git");
}
