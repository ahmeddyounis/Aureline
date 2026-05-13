//! Fixture-driven coverage for shell Git change-list projections.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use aureline_git::{
    BranchState, ChangeDiscovery, ChangeKind, ChangeSummary, GitChange, GitServiceState,
    GitStatusSnapshot, HeadIdentity, RepositoryIdentity,
};
use aureline_shell::git_changes::{
    GitChangeGroupKind, GitChangeListSurfaceBundle, GitChangeListViewport,
};

#[derive(Debug, Deserialize)]
struct ChangeListFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    viewport: GitChangeListViewport,
    input: ChangeListFixtureInput,
    expected: ExpectedChangeList,
}

#[derive(Debug, Deserialize)]
struct ChangeListFixtureInput {
    changes: Option<Vec<GitChange>>,
    generated_change_set: Option<GeneratedChangeSet>,
}

#[derive(Debug, Deserialize)]
struct GeneratedChangeSet {
    staged_count: usize,
    unstaged_count: usize,
    untracked_count: usize,
}

#[derive(Debug, Deserialize)]
struct ExpectedChangeList {
    staged_total_count: usize,
    unstaged_total_count: usize,
    staged_visible_count: usize,
    unstaged_visible_count: usize,
    staged_virtualized: bool,
    unstaged_virtualized: bool,
    first_staged_path: Option<String>,
    first_unstaged_path: Option<String>,
    required_state_tokens: Vec<String>,
    shared_chip_vocabulary: bool,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/git/change_list_alpha")
}

fn generated_changes(spec: &GeneratedChangeSet) -> Vec<GitChange> {
    let mut changes = Vec::with_capacity(
        spec.staged_count
            .saturating_add(spec.unstaged_count)
            .saturating_add(spec.untracked_count),
    );
    for index in 0..spec.staged_count {
        changes.push(change(
            format!("staged/file_{index:04}.rs"),
            "M.",
            ChangeKind::Modified,
            true,
            false,
        ));
    }
    for index in 0..spec.unstaged_count {
        changes.push(change(
            format!("unstaged/file_{index:04}.rs"),
            ".M",
            ChangeKind::Modified,
            false,
            true,
        ));
    }
    for index in 0..spec.untracked_count {
        changes.push(change(
            format!("untracked/file_{index:04}.txt"),
            "??",
            ChangeKind::Untracked,
            false,
            false,
        ));
    }
    changes
}

fn change(
    path: impl Into<PathBuf>,
    status_code: impl Into<String>,
    change_kind: ChangeKind,
    is_staged: bool,
    is_unstaged: bool,
) -> GitChange {
    GitChange {
        path: path.into(),
        original_path: None,
        status_code: status_code.into(),
        change_kind,
        is_staged,
        is_unstaged,
        is_conflicted: false,
    }
}

fn fixture_changes(input: ChangeListFixtureInput) -> Vec<GitChange> {
    match (input.changes, input.generated_change_set) {
        (Some(changes), None) => changes,
        (None, Some(spec)) => generated_changes(&spec),
        _ => panic!("fixture must provide exactly one change input"),
    }
}

fn snapshot_for_case(case_name: &str, changes: Vec<GitChange>) -> GitStatusSnapshot {
    GitStatusSnapshot {
        record_kind: aureline_git::GIT_STATUS_SNAPSHOT_RECORD_KIND.to_string(),
        schema_version: 1,
        service_ref: "service.git.status.alpha".to_string(),
        workspace_ref: format!("workspace.fixture.{case_name}"),
        requested_root: PathBuf::from("/fixtures/git/change-list-alpha"),
        repository: Some(RepositoryIdentity {
            repo_ref: "repo.local.change-list-alpha".to_string(),
            worktree_ref: "worktree.local.change-list-alpha".to_string(),
            repo_label: "change-list-alpha".to_string(),
            repo_root: PathBuf::from("/fixtures/git/change-list-alpha"),
            git_dir: PathBuf::from("/fixtures/git/change-list-alpha/.git"),
            common_dir: PathBuf::from("/fixtures/git/change-list-alpha/.git"),
        }),
        head: HeadIdentity {
            state: BranchState::Attached,
            branch_label: Some("main".to_string()),
            branch_ref: Some("refs/heads/main".to_string()),
            head_oid: Some("1111111111111111111111111111111111111111".to_string()),
            head_short_oid: Some("1111111".to_string()),
            upstream: Some("origin/main".to_string()),
            ahead: Some(0),
            behind: Some(0),
        },
        service_state: GitServiceState::Current,
        degraded_reason: None,
        discovery: ChangeDiscovery {
            status_available: true,
            branch_identity_available: true,
            change_list_available: true,
            current_claim_narrowed: false,
            coverage_label: "fixture status with path-level changes".to_string(),
        },
        change_summary: ChangeSummary::from_changes(&changes),
        changes,
        consumer_refs: Vec::new(),
        observed_at: "mono:git:change-list-fixture".to_string(),
    }
}

fn assert_group(
    bundle: &GitChangeListSurfaceBundle,
    kind: GitChangeGroupKind,
    total_count: usize,
    visible_count: usize,
    virtualized: bool,
    first_path: Option<&str>,
    case_name: &str,
) {
    let group = bundle
        .group(kind)
        .unwrap_or_else(|| panic!("{case_name}: missing {} group", kind.as_str()));
    assert_eq!(
        group.total_count,
        total_count,
        "{case_name}: {} total count mismatch",
        kind.as_str()
    );
    assert_eq!(
        group.visible_count,
        visible_count,
        "{case_name}: {} visible count mismatch",
        kind.as_str()
    );
    assert_eq!(
        group.virtualization.virtualized,
        virtualized,
        "{case_name}: {} virtualization mismatch",
        kind.as_str()
    );
    if let Some(first_path) = first_path {
        let row = group.rows.first().unwrap_or_else(|| {
            panic!("{case_name}: {} group must have a first row", kind.as_str())
        });
        assert_eq!(
            row.display_path,
            first_path,
            "{case_name}: {} first visible path mismatch",
            kind.as_str()
        );
    }
}

#[test]
fn protected_change_list_fixtures_match_shell_projection() {
    let mut fixtures: Vec<_> = std::fs::read_dir(fixtures_dir())
        .expect("fixture directory")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "yaml"))
        .collect();
    fixtures.sort();
    assert!(!fixtures.is_empty(), "change-list fixtures must exist");

    for path in fixtures {
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let fixture: ChangeListFixture = serde_yaml::from_str(&text)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
        assert_eq!(fixture.record_kind, "git_change_list_alpha_case");
        assert_eq!(fixture.schema_version, 1);

        let changes = fixture_changes(fixture.input);
        let snapshot = snapshot_for_case(&fixture.case_name, changes);
        let bundle = GitChangeListSurfaceBundle::from_snapshot(
            "mono:git:change-list-bundle",
            &snapshot,
            fixture.viewport,
        );

        assert_group(
            &bundle,
            GitChangeGroupKind::Staged,
            fixture.expected.staged_total_count,
            fixture.expected.staged_visible_count,
            fixture.expected.staged_virtualized,
            fixture.expected.first_staged_path.as_deref(),
            &fixture.case_name,
        );
        assert_group(
            &bundle,
            GitChangeGroupKind::Unstaged,
            fixture.expected.unstaged_total_count,
            fixture.expected.unstaged_visible_count,
            fixture.expected.unstaged_virtualized,
            fixture.expected.first_unstaged_path.as_deref(),
            &fixture.case_name,
        );
        assert_eq!(
            bundle.has_shared_file_state_vocabulary(),
            fixture.expected.shared_chip_vocabulary,
            "{}: shared chip vocabulary mismatch",
            fixture.case_name
        );

        for token in &fixture.expected.required_state_tokens {
            assert!(
                bundle
                    .groups
                    .iter()
                    .flat_map(|group| group.rows.iter())
                    .any(|row| row.file_state_token == *token),
                "{}: missing visible state token {token}",
                fixture.case_name
            );
        }
    }
}
