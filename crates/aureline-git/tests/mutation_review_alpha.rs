//! Fixture-driven coverage for preview-first Git mutation flows.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use aureline_git::{
    GitMutationOperationKind, GitMutationOutcomeState, GitMutationPreviewState, GitMutationRequest,
    GitMutationService,
};
use aureline_history::ActorLineageClass;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MutationFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    setup_mode: String,
    operation: GitMutationOperationKind,
    paths: Vec<PathBuf>,
    expected: ExpectedMutation,
}

#[derive(Debug, Deserialize)]
struct ExpectedMutation {
    preview_state: String,
    consequence_class: String,
    checkpoint_required: bool,
    checkpoint_captured: bool,
    rollback_path_class: String,
    destructive_review_required: bool,
    protected_rows_require_review: bool,
    min_preview_diff_line_count: usize,
    result_state: String,
    status_after_apply: Option<String>,
    revert_preview_operation: Option<String>,
    revert_result_state: Option<String>,
    status_after_revert: Option<String>,
    activity_state_class: String,
    support_export_phase: String,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/git/mutation_review_alpha")
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
    run_git(root, &["add", "src/lib.rs"]);
    run_git(root, &["commit", "-q", "-m", "initial fixture commit"]);
}

fn build_case_root(mode: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    match mode {
        "unstaged_modified" => {
            seed_committed_repo(dir.path());
            fs::write(
                dir.path().join("src/lib.rs"),
                "pub fn answer() -> u32 {\n    42\n}\n",
            )
            .expect("modify source");
        }
        "staged_modified" => {
            seed_committed_repo(dir.path());
            fs::write(
                dir.path().join("src/lib.rs"),
                "pub fn answer() -> u32 {\n    42\n}\n",
            )
            .expect("modify source");
            run_git(dir.path(), &["add", "src/lib.rs"]);
        }
        "untracked_only" => {
            seed_committed_repo(dir.path());
            fs::write(dir.path().join("notes.txt"), "temporary\n").expect("write untracked file");
        }
        other => panic!("unsupported setup_mode: {other}"),
    }
    dir
}

fn status_code(root: &Path, path: &Path) -> Option<String> {
    let output = git_output(
        root,
        &["status", "--porcelain=v1", "--", &path.to_string_lossy()],
    );
    output.lines().next().map(|line| {
        line.chars()
            .take(2)
            .map(|ch| if ch == ' ' { '.' } else { ch })
            .collect()
    })
}

fn assert_status(root: &Path, path: &Path, expected: Option<&str>, label: &str) {
    assert_eq!(status_code(root, path).as_deref(), expected, "{label}");
}

fn parse_preview_state(value: &str) -> GitMutationPreviewState {
    match value {
        "ready_to_apply" => GitMutationPreviewState::ReadyToApply,
        "blocked" => GitMutationPreviewState::Blocked,
        "degraded" => GitMutationPreviewState::Degraded,
        other => panic!("unsupported preview state: {other}"),
    }
}

fn parse_result_state(value: &str) -> GitMutationOutcomeState {
    match value {
        "applied" => GitMutationOutcomeState::Applied,
        "reverted" => GitMutationOutcomeState::Reverted,
        "blocked_no_changes_made" => GitMutationOutcomeState::BlockedNoChangesMade,
        "failed" => GitMutationOutcomeState::Failed,
        other => panic!("unsupported result state: {other}"),
    }
}

fn run_fixture(path: &Path) {
    let text = fs::read_to_string(path).expect("read fixture");
    let fixture: MutationFixture = serde_yaml::from_str(&text).expect("parse fixture");
    assert_eq!(fixture.record_kind, "git_mutation_review_alpha_case");
    assert_eq!(fixture.schema_version, 1);

    let dir = build_case_root(&fixture.setup_mode);
    let request = GitMutationRequest::with_observed_at(
        format!("workspace.fixture.{}", fixture.case_name),
        dir.path(),
        fixture.operation,
        fixture.paths.clone(),
        "2026-05-13T00:00:00Z",
    )
    .with_launch_source_ref(format!("git.change.row.{}", fixture.case_name));
    let service = GitMutationService::default();
    let preview = service.preview(&request);

    assert_eq!(
        preview.preview_state,
        parse_preview_state(&fixture.expected.preview_state),
        "{}: preview state",
        fixture.case_name
    );
    assert_eq!(
        preview.consequence_class, fixture.expected.consequence_class,
        "{}: consequence class",
        fixture.case_name
    );
    assert_eq!(
        preview.checkpoint.checkpoint_required, fixture.expected.checkpoint_required,
        "{}: checkpoint required",
        fixture.case_name
    );
    assert_eq!(
        preview.checkpoint.checkpoint_captured, fixture.expected.checkpoint_captured,
        "{}: checkpoint captured",
        fixture.case_name
    );
    assert_eq!(
        preview.checkpoint.rollback_path_class, fixture.expected.rollback_path_class,
        "{}: rollback path class",
        fixture.case_name
    );
    assert_eq!(
        preview.destructive_review_required, fixture.expected.destructive_review_required,
        "{}: destructive review",
        fixture.case_name
    );
    assert_eq!(
        preview
            .scope
            .targets
            .iter()
            .all(|target| target.protected_review_required),
        fixture.expected.protected_rows_require_review,
        "{}: protected row cue",
        fixture.case_name
    );
    assert!(
        preview.scope.all_rows_have_visible_scope_and_preview(),
        "{}: visible scope and preview refs",
        fixture.case_name
    );
    assert!(
        preview.diff_preview.diff_line_count >= fixture.expected.min_preview_diff_line_count,
        "{}: preview diff line count",
        fixture.case_name
    );
    if preview.preview_state == GitMutationPreviewState::ReadyToApply {
        assert!(
            preview.destructive_actions_have_checkpoint(),
            "{}: destructive checkpoint posture",
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
    assert!(
        result.attribution_is_exportable(),
        "{}: activity and support attribution",
        fixture.case_name
    );
    let history_packet = result.local_history_alpha_packet("2026-05-13T00:00:01Z");
    history_packet
        .validate()
        .expect("Git result projects export-safe history lineage");
    assert_eq!(
        history_packet.actor_lineage_rows[0].actor_lineage_class,
        ActorLineageClass::GitMutation,
        "{}: local-history Git lineage class",
        fixture.case_name
    );
    assert!(
        !history_packet.export_safety.raw_snapshot_bodies_included,
        "{}: Git lineage export omits raw snapshots",
        fixture.case_name
    );
    assert_status(
        dir.path(),
        &fixture.paths[0],
        fixture.expected.status_after_apply.as_deref(),
        &format!("{}: status after apply", fixture.case_name),
    );

    if let Some(expected_revert_kind) = fixture.expected.revert_preview_operation.as_deref() {
        let revert_preview = service.preview_revert(&result, "2026-05-13T00:00:02Z");
        assert_eq!(
            revert_preview.operation.as_str(),
            expected_revert_kind,
            "{}: revert operation kind",
            fixture.case_name
        );
        assert!(
            revert_preview.ready_to_apply(),
            "{}: revert preview ready",
            fixture.case_name
        );
        let reverted = service.apply(&revert_preview, "2026-05-13T00:00:03Z");
        assert_eq!(
            reverted.outcome_state,
            parse_result_state(
                fixture
                    .expected
                    .revert_result_state
                    .as_deref()
                    .expect("revert result state")
            ),
            "{}: revert result state",
            fixture.case_name
        );
        assert_eq!(reverted.support_export.phase, "revert");
        assert!(reverted.attribution_is_exportable());
        assert_status(
            dir.path(),
            &fixture.paths[0],
            fixture.expected.status_after_revert.as_deref(),
            &format!("{}: status after revert", fixture.case_name),
        );
    }
}

#[test]
fn protected_mutation_review_fixtures_match_git_service_contract() {
    let mut fixtures: Vec<_> = fs::read_dir(fixtures_dir())
        .expect("fixture directory")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "yaml"))
        .collect();
    fixtures.sort();
    assert!(!fixtures.is_empty(), "mutation review fixtures must exist");

    for path in fixtures {
        run_fixture(&path);
    }
}
