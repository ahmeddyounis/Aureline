//! Fixture-driven coverage for quick-open and symbol-search ranking cards.
//!
//! The fixtures under `fixtures/search/ranking_reason_alpha/` pin the first
//! launch search surfaces that expose stable result IDs plus structured
//! ranking reasons. The same card payload feeds UI inspection and support
//! export, so the tests assert IDs and reasons without reading rendered row
//! strings.

use std::path::Path;

use serde::Deserialize;

use aureline_search::{SearchPlannerAlpha, SearchPlannerInputs};
use aureline_shell::palette::{
    QuickOpenCommandRow, QuickOpenLexicalRow, QuickOpenQuerySession, QuickOpenRecentTarget,
    QuickOpenSourceClass, QuickOpenSourceState,
};
use aureline_shell::search::{
    ranking_reason_cards_for_planned_result_set, ranking_reason_cards_for_quick_open_snapshot,
    RankingReasonCard, RankingReasonSupportExport,
};
use aureline_workspace::ScopeClass as WorkspaceScopeClass;

#[derive(Debug, Deserialize)]
struct CaseFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    surface: String,
    quick_open: Option<QuickOpenCase>,
    planner_input: Option<SearchPlannerInputs>,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct QuickOpenCase {
    workspace_id: String,
    scope_class: String,
    workset_name: Option<String>,
    query: String,
    observed_at: String,
    #[serde(default)]
    recents: Vec<RecentCase>,
    #[serde(default)]
    commands: Vec<CommandCase>,
    lexical: LexicalCase,
}

#[derive(Debug, Deserialize)]
struct RecentCase {
    recent_id: String,
    display_label: String,
    secondary_label: String,
    relative_path: Option<String>,
    target_kind_token: String,
}

#[derive(Debug, Deserialize)]
struct CommandCase {
    command_id: String,
    title: String,
    summary: String,
    dominant_side_effect_class: String,
    invocation_preview_class: String,
    disabled_reason_class: Option<String>,
}

#[derive(Debug, Deserialize)]
struct LexicalCase {
    state: String,
    #[serde(default)]
    partial_truth_causes: Vec<String>,
    #[serde(default)]
    rows: Vec<LexicalRowCase>,
}

#[derive(Debug, Deserialize)]
struct LexicalRowCase {
    relative_path: String,
    source_class: String,
    match_kind_token: String,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    support_redaction_class: String,
    cards: Vec<ExpectedCard>,
}

#[derive(Debug, Deserialize)]
struct ExpectedCard {
    result_id: String,
    surface: String,
    row_kind_token: String,
    title: String,
    target_ref: String,
    source_class_token: String,
    readiness_state: String,
    result_truth_class: String,
    partiality_class: String,
    ranking_reason_classes: Vec<String>,
    dominant_signal_classes: Vec<String>,
    partial_truth_causes: Vec<String>,
    omitted_source_notes: Vec<String>,
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

fn quick_open_cards(case: &QuickOpenCase) -> Vec<RankingReasonCard> {
    let mut session = QuickOpenQuerySession::new(
        case.workspace_id.clone(),
        parse_scope_class(&case.scope_class),
        case.workset_name.clone(),
    );
    session.set_recents(
        case.recents
            .iter()
            .map(|recent| QuickOpenRecentTarget {
                recent_id: recent.recent_id.clone(),
                display_label: recent.display_label.clone(),
                secondary_label: recent.secondary_label.clone(),
                relative_path: recent.relative_path.clone(),
                target_kind_token: recent.target_kind_token.clone(),
            })
            .collect(),
    );
    session.set_commands(
        case.commands
            .iter()
            .map(|command| QuickOpenCommandRow {
                command_id: command.command_id.clone(),
                title: command.title.clone(),
                summary: command.summary.clone(),
                dominant_side_effect_class: command.dominant_side_effect_class.clone(),
                invocation_preview_class: command.invocation_preview_class.clone(),
                disabled_reason_class: command.disabled_reason_class.clone(),
            })
            .collect(),
    );
    session.set_lexical(
        case.lexical
            .rows
            .iter()
            .map(|row| QuickOpenLexicalRow {
                relative_path: row.relative_path.clone(),
                source_class: parse_source_class(&row.source_class),
                match_kind_token: row.match_kind_token.clone(),
            })
            .collect(),
        parse_source_state(&case.lexical.state),
        case.lexical.partial_truth_causes.clone(),
    );
    session.open();
    session.set_query(case.query.clone());
    let snapshot = session.export_snapshot(case.observed_at.clone());
    ranking_reason_cards_for_quick_open_snapshot(&snapshot)
}

fn assert_cards(case_name: &str, cards: &[RankingReasonCard], expected_cards: &[ExpectedCard]) {
    assert_eq!(
        cards.len(),
        expected_cards.len(),
        "case {case_name} card count mismatch"
    );
    for expected in expected_cards {
        let card = cards
            .iter()
            .find(|card| card.result_id == expected.result_id)
            .unwrap_or_else(|| {
                panic!(
                    "case {case_name} missing card for result_id {}",
                    expected.result_id
                )
            });
        assert_eq!(card.surface, expected.surface, "surface mismatch");
        assert_eq!(
            card.row_kind_token, expected.row_kind_token,
            "row kind mismatch for {}",
            expected.result_id
        );
        assert_eq!(card.title, expected.title, "title mismatch");
        assert_eq!(card.target_ref, expected.target_ref, "target ref mismatch");
        assert_eq!(
            card.source_class_token, expected.source_class_token,
            "source mismatch"
        );
        assert_eq!(
            card.readiness_state, expected.readiness_state,
            "readiness mismatch"
        );
        assert_eq!(
            card.result_truth_class, expected.result_truth_class,
            "truth class mismatch"
        );
        assert_eq!(
            card.partiality_class, expected.partiality_class,
            "partiality mismatch"
        );
        assert_eq!(
            card.ranking_reason_classes, expected.ranking_reason_classes,
            "ranking reasons mismatch"
        );
        assert_eq!(
            card.dominant_signals
                .iter()
                .map(|signal| signal.signal_class.as_str())
                .collect::<Vec<_>>(),
            expected
                .dominant_signal_classes
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            "dominant signals mismatch",
        );
        assert_eq!(
            card.partial_truth_causes, expected.partial_truth_causes,
            "partial truth mismatch"
        );
        assert_eq!(
            card.omitted_source_notes, expected.omitted_source_notes,
            "omitted source notes mismatch"
        );
    }
}

fn run_fixture(path: &Path) {
    let payload = std::fs::read_to_string(path).unwrap_or_else(|err| {
        panic!("failed to read fixture {}: {err}", path.display());
    });
    let fixture: CaseFixture = serde_json::from_str(&payload).unwrap_or_else(|err| {
        panic!("failed to parse fixture {}: {err}", path.display());
    });
    assert_eq!(fixture.record_kind, "ranking_reason_alpha_case");
    assert_eq!(fixture.schema_version, 1);

    let cards = match fixture.surface.as_str() {
        "quick_open" => quick_open_cards(
            fixture
                .quick_open
                .as_ref()
                .expect("quick_open fixture body"),
        ),
        "symbol_search" => {
            let output = SearchPlannerAlpha::plan(
                fixture
                    .planner_input
                    .clone()
                    .expect("planner_input fixture body"),
            );
            ranking_reason_cards_for_planned_result_set(&output.query_session, &output.result_set)
        }
        other => panic!("unknown surface in fixture: {other}"),
    };

    assert_cards(&fixture.case_name, &cards, &fixture.expect.cards);

    let support_export = RankingReasonSupportExport::from_cards(
        format!("support:search-ranking-reasons:{}", fixture.case_name),
        "mono:ranking:support",
        fixture.expect.support_redaction_class.clone(),
        cards.clone(),
    );
    assert_eq!(
        support_export.redaction_class, fixture.expect.support_redaction_class,
        "support redaction class mismatch"
    );
    assert_eq!(
        support_export.result_ids,
        fixture
            .expect
            .cards
            .iter()
            .map(|card| card.result_id.clone())
            .collect::<Vec<_>>(),
        "support export must include result IDs directly"
    );
    assert!(
        support_export
            .cards
            .iter()
            .all(|card| !card.ranking_reason_classes.is_empty()),
        "support export must include structured ranking reasons"
    );
}

fn fixture_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/search/ranking_reason_alpha")
        .canonicalize()
        .expect("fixture directory must exist")
}

#[test]
fn quick_open_hot_set_file_card_matches_fixture() {
    run_fixture(&fixture_dir().join("quick_open_hot_set_file_card.json"));
}

#[test]
fn symbol_search_structural_fallback_card_matches_fixture() {
    run_fixture(&fixture_dir().join("symbol_search_structural_fallback_card.json"));
}
