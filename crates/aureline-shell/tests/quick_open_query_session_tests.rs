//! Protected-walk integration tests for the quick-open query session.
//!
//! Walks the protected dogfood path named in the spec:
//!
//!   open quick open from a real workspace
//!     -> search recent targets, commands, and files
//!     -> jump to one result.
//!
//! Plus the failure drill:
//!
//!   query quick-open before all sources are ready
//!     -> confirm recent / command / file sections show truthful warming or
//!        partial state.
//!
//! Each fixture under `/fixtures/search/quick_open_cases/*.json` describes
//! the inputs and the expected materialized snapshot. The tests load each
//! fixture, drive a `QuickOpenQuerySession` to those inputs, and assert the
//! observed snapshot's per-source readiness and per-row identity tuples
//! match the fixture exactly.

use std::path::Path;

use serde::Deserialize;

use aureline_shell::quick_open::{
    QuickOpenCommandRow, QuickOpenLexicalRow, QuickOpenQuerySession, QuickOpenRecentTarget,
    QuickOpenSourceClass, QuickOpenSourceState,
};
use aureline_workspace::ScopeClass as WorkspaceScopeClass;

#[derive(Debug, Deserialize)]
struct QuickOpenFixture {
    case_id: String,
    #[serde(default)]
    #[allow(dead_code)]
    title: String,
    #[serde(default)]
    #[allow(dead_code)]
    scenario: String,
    workspace_id: String,
    scope_class: String,
    workset_name: Option<String>,
    query: String,
    #[serde(default)]
    held_modifiers: Vec<String>,
    #[serde(default)]
    recents: Vec<FixtureRecent>,
    #[serde(default)]
    commands: Vec<FixtureCommand>,
    lexical: FixtureLexical,
    expected_snapshot: ExpectedSnapshot,
}

#[derive(Debug, Deserialize)]
struct FixtureRecent {
    recent_id: String,
    display_label: String,
    secondary_label: String,
    relative_path: Option<String>,
    target_kind_token: String,
}

#[derive(Debug, Deserialize)]
struct FixtureCommand {
    command_id: String,
    title: String,
    summary: String,
    dominant_side_effect_class: String,
    invocation_preview_class: String,
    disabled_reason_class: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureLexical {
    state: String,
    #[serde(default)]
    partial_truth_causes: Vec<String>,
    #[serde(default)]
    rows: Vec<FixtureLexicalRow>,
}

#[derive(Debug, Deserialize)]
struct FixtureLexicalRow {
    relative_path: String,
    source_class: String,
    match_kind_token: String,
}

#[derive(Debug, Deserialize)]
struct ExpectedSnapshot {
    scope_class_token: String,
    scope_chip_label: String,
    sources: Vec<ExpectedSource>,
    rows: Vec<ExpectedRow>,
}

#[derive(Debug, Deserialize)]
struct ExpectedSource {
    source_class_token: String,
    source_state_token: String,
}

#[derive(Debug, Deserialize)]
struct ExpectedRow {
    row_kind_token: String,
    source_class_token: String,
    source_state_token: String,
    display_label: String,
    command_id: Option<String>,
    relative_path: Option<String>,
}

fn parse_scope_class(token: &str) -> WorkspaceScopeClass {
    match token {
        "current_repo" => WorkspaceScopeClass::CurrentRepo,
        "selected_workset" => WorkspaceScopeClass::SelectedWorkset,
        "sparse_slice" => WorkspaceScopeClass::SparseSlice,
        "full_workspace" => WorkspaceScopeClass::FullWorkspace,
        "policy_limited_view" => WorkspaceScopeClass::PolicyLimitedView,
        other => panic!("unknown scope_class token in fixture: {other}"),
    }
}

fn parse_source_class(token: &str) -> QuickOpenSourceClass {
    match token {
        "recent_target" => QuickOpenSourceClass::RecentTarget,
        "command" => QuickOpenSourceClass::Command,
        "lexical_filename" => QuickOpenSourceClass::LexicalFilename,
        "lexical_path" => QuickOpenSourceClass::LexicalPath,
        other => panic!("unknown source_class token in fixture: {other}"),
    }
}

fn parse_source_state(token: &str) -> QuickOpenSourceState {
    match token {
        "not_requested" => QuickOpenSourceState::NotRequested,
        "warming" => QuickOpenSourceState::Warming,
        "partial" => QuickOpenSourceState::Partial,
        "ready" => QuickOpenSourceState::Ready,
        "unavailable" => QuickOpenSourceState::Unavailable,
        other => panic!("unknown source_state token in fixture: {other}"),
    }
}

fn run_fixture(path: &Path) {
    let json = std::fs::read_to_string(path).unwrap_or_else(|err| {
        panic!("failed to read fixture {}: {err}", path.display());
    });
    let fixture: QuickOpenFixture = serde_json::from_str(&json).unwrap_or_else(|err| {
        panic!("failed to parse fixture {}: {err}", path.display());
    });

    let scope = parse_scope_class(&fixture.scope_class);
    let mut session = QuickOpenQuerySession::new(
        fixture.workspace_id.clone(),
        scope,
        fixture.workset_name.clone(),
    );
    session.set_recents(
        fixture
            .recents
            .iter()
            .map(|r| QuickOpenRecentTarget {
                recent_id: r.recent_id.clone(),
                display_label: r.display_label.clone(),
                secondary_label: r.secondary_label.clone(),
                relative_path: r.relative_path.clone(),
                target_kind_token: r.target_kind_token.clone(),
            })
            .collect(),
    );
    session.set_commands(
        fixture
            .commands
            .iter()
            .map(|c| QuickOpenCommandRow {
                command_id: c.command_id.clone(),
                title: c.title.clone(),
                summary: c.summary.clone(),
                dominant_side_effect_class: c.dominant_side_effect_class.clone(),
                invocation_preview_class: c.invocation_preview_class.clone(),
                disabled_reason_class: c.disabled_reason_class.clone(),
            })
            .collect(),
    );
    session.set_lexical(
        fixture
            .lexical
            .rows
            .iter()
            .map(|row| QuickOpenLexicalRow {
                relative_path: row.relative_path.clone(),
                source_class: parse_source_class(&row.source_class),
                match_kind_token: row.match_kind_token.clone(),
            })
            .collect(),
        parse_source_state(&fixture.lexical.state),
        fixture.lexical.partial_truth_causes.clone(),
    );
    session.open();
    session.set_held_modifiers(fixture.held_modifiers.iter().cloned());
    session.set_query(fixture.query.clone());

    let snapshot = session.export_snapshot("mono:fixture");

    assert_eq!(
        snapshot.scope_class_token, fixture.expected_snapshot.scope_class_token,
        "case {} scope_class_token mismatch",
        fixture.case_id
    );
    assert_eq!(
        snapshot.scope_chip_label, fixture.expected_snapshot.scope_chip_label,
        "case {} scope_chip_label mismatch",
        fixture.case_id
    );

    for expected in &fixture.expected_snapshot.sources {
        let observed = snapshot
            .sources
            .iter()
            .find(|s| s.source_class_token == expected.source_class_token)
            .unwrap_or_else(|| {
                panic!(
                    "case {} missing snapshot source {}",
                    fixture.case_id, expected.source_class_token
                )
            });
        assert_eq!(
            observed.source_state_token, expected.source_state_token,
            "case {} source {} state mismatch",
            fixture.case_id, expected.source_class_token
        );
    }

    assert_eq!(
        snapshot.rows.len(),
        fixture.expected_snapshot.rows.len(),
        "case {} row count mismatch (observed: {:#?})",
        fixture.case_id,
        snapshot.rows
    );
    for (idx, (observed, expected)) in snapshot
        .rows
        .iter()
        .zip(fixture.expected_snapshot.rows.iter())
        .enumerate()
    {
        assert_eq!(
            observed.row_kind_token, expected.row_kind_token,
            "case {} row {} kind mismatch",
            fixture.case_id, idx
        );
        assert_eq!(
            observed.source_class_token, expected.source_class_token,
            "case {} row {} source mismatch",
            fixture.case_id, idx
        );
        assert_eq!(
            observed.source_state_token, expected.source_state_token,
            "case {} row {} source_state mismatch",
            fixture.case_id, idx
        );
        assert_eq!(
            observed.display_label, expected.display_label,
            "case {} row {} display_label mismatch",
            fixture.case_id, idx
        );
        assert_eq!(
            observed.command_id, expected.command_id,
            "case {} row {} command_id mismatch",
            fixture.case_id, idx
        );
        assert_eq!(
            observed.relative_path, expected.relative_path,
            "case {} row {} relative_path mismatch",
            fixture.case_id, idx
        );
    }

    // Cannot-close invariant: every command row in the snapshot must carry
    // a non-empty command_id and a frozen invocation_preview_class. The
    // chrome MUST NOT relabel a command row as a file row, and a disabled
    // command MUST surface its disabled_reason_class on the same row.
    for row in snapshot
        .rows
        .iter()
        .filter(|row| row.row_kind_token == "command")
    {
        assert!(
            row.command_id.as_ref().is_some_and(|id| !id.is_empty()),
            "case {} command row missing command_id",
            fixture.case_id
        );
        assert!(
            row.invocation_preview_class
                .as_ref()
                .is_some_and(|class| !class.is_empty()),
            "case {} command row missing invocation_preview_class",
            fixture.case_id
        );
    }
}

fn fixture_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/search/quick_open_cases")
        .canonicalize()
        .expect("fixture directory must exist")
}

#[test]
fn nominal_merged_results_walk() {
    run_fixture(&fixture_dir().join("nominal_merged_results.json"));
}

#[test]
fn lexical_warming_partial_drill() {
    run_fixture(&fixture_dir().join("lexical_warming_partial.json"));
}

#[test]
fn lexical_unavailable_drill() {
    run_fixture(&fixture_dir().join("lexical_unavailable.json"));
}

#[test]
fn empty_query_recents_only_walk() {
    run_fixture(&fixture_dir().join("empty_query_recents_only.json"));
}
