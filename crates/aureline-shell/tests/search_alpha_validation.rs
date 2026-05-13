//! Combined alpha search validation coverage.
//!
//! This test proves the search review packet consumes ranking-reason cards,
//! palette discoverability rows, and keyboard-route audit rows instead of
//! cloning their state locally.

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

use aureline_commands::registry::seeded_registry;
use aureline_input::keybindings::PlatformClass;
use aureline_input::presets::KeymapPresetId;
use aureline_search::{
    PlannerDataPath, PlannerFreshnessClass, PlannerPathReadiness, PlannerPathSnapshot,
    PlannerRankingReason, PlannerTargetKind, SearchPlannerAlpha, SearchPlannerInputs,
    SearchQuerySession,
};
use aureline_shell::commands::CommandReviewRuntimeInputs;
use aureline_shell::help::keyboard_gap_audit::materialize_alpha_keyboard_gap_audit;
use aureline_shell::palette::{
    materialize_alpha_palette_query, AlphaFileCandidate, AlphaPaletteQueryInputs,
    AlphaRecentActionCandidate, AlphaSymbolCandidate, QuickOpenLexicalRow, QuickOpenQuerySession,
    QuickOpenSourceClass, QuickOpenSourceState,
};
use aureline_shell::search::{
    materialize_search_alpha_validation_packet, ranking_reason_cards_for_planned_result_set,
    ranking_reason_cards_for_quick_open_snapshot,
};
use aureline_workspace::ScopeClass as WorkspaceScopeClass;
use serde::Deserialize;

const SEARCH_ALPHA_KNOWN_LIMIT: &str =
    "known_limit:external_alpha.search_alpha_synthetic_and_partial_index_only";

#[derive(Debug, Deserialize)]
struct SearchKeyboardFixture {
    fixture_id: String,
    required_surface_ids: Vec<String>,
    required_ranking_reason_classes: Vec<String>,
    required_keyboard_surface_id: String,
    required_known_limit_ref: String,
    acceptance: SearchKeyboardAcceptance,
}

#[derive(Debug, Deserialize)]
struct SearchKeyboardAcceptance {
    result_ids_reused_from_card_contract: bool,
    ranking_reason_cards_keyboard_reachable: bool,
    planner_result_id_contract_required: bool,
    discoverability_row_kinds_required: Vec<String>,
}

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/accessibility/m2_search_keyboard/search_keyboard_parity.yaml")
}

fn load_fixture() -> SearchKeyboardFixture {
    let path = fixture_path();
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

fn quick_open_cards() -> Vec<aureline_shell::search::RankingReasonCard> {
    let mut session = QuickOpenQuerySession::new(
        "ws-alpha",
        WorkspaceScopeClass::CurrentRepo,
        Option::<String>::None,
    );
    session.set_lexical(
        vec![QuickOpenLexicalRow {
            relative_path: "src/main.rs".to_string(),
            source_class: QuickOpenSourceClass::LexicalFilename,
            match_kind_token: "prefix_basename".to_string(),
        }],
        QuickOpenSourceState::Partial,
        vec![
            "hot_set_only".to_string(),
            "indexing_in_progress".to_string(),
        ],
    );
    session.open();
    session.set_query("main");
    let snapshot = session.export_snapshot("mono:search-alpha:quick-open");
    ranking_reason_cards_for_quick_open_snapshot(&snapshot)
}

fn symbol_cards() -> Vec<aureline_shell::search::RankingReasonCard> {
    let query_session = SearchQuerySession::for_local_text(
        "search:session:alpha:symbol",
        aureline_search::SearchSurface::SymbolSearch,
        "open workspace",
        aureline_search::ScopeClass::SelectedWorkset,
        "Selected workset",
        "search-planner-alpha",
        "partial",
        "mono:search-alpha:symbol",
    );
    let inputs = SearchPlannerInputs {
        query_session,
        planner_pass_id: "search:planner:alpha:symbol".to_string(),
        result_set_id: "search:result-set:alpha:symbol".to_string(),
        planner_version: "search-planner-alpha".to_string(),
        observed_at: "mono:search-alpha:symbol".to_string(),
        path_snapshots: vec![
            PlannerPathSnapshot {
                path_kind: PlannerDataPath::GraphBacked,
                snapshot_id: "search:snapshot:alpha:graph-warming".to_string(),
                readiness: PlannerPathReadiness::Warming,
                freshness: PlannerFreshnessClass::Unknown,
                index_epoch: None,
                graph_epoch: Some("graph:epoch:alpha:warming".to_string()),
                unavailable_reason: Some(aureline_search::PlannerUnavailableReason::GraphWarming),
                partial_truth_causes: vec!["indexing_in_progress".to_string()],
                rows: Vec::new(),
            },
            PlannerPathSnapshot {
                path_kind: PlannerDataPath::Structural,
                snapshot_id: "search:snapshot:alpha:structural".to_string(),
                readiness: PlannerPathReadiness::Partial,
                freshness: PlannerFreshnessClass::AuthoritativeLive,
                index_epoch: Some("struct:epoch:alpha".to_string()),
                graph_epoch: None,
                unavailable_reason: None,
                partial_truth_causes: vec![
                    "indexing_in_progress".to_string(),
                    "graph_pending".to_string(),
                ],
                rows: vec![aureline_search::PlannerCandidate {
                    candidate_id: "struct:symbol:open_workspace".to_string(),
                    canonical_id: "workspace:symbol:open_workspace".to_string(),
                    target_kind: PlannerTargetKind::Symbol,
                    title: "open_workspace".to_string(),
                    relative_path: Some("crates/aureline-workspace/src/lib.rs".to_string()),
                    symbol_ref: Some("rust:fn:open_workspace".to_string()),
                    ranking_reasons: vec![
                        PlannerRankingReason::StructuralSymbolMatch,
                        PlannerRankingReason::StructuralFallback,
                        PlannerRankingReason::SymbolKindPrior,
                    ],
                    partial_truth_causes: Vec::new(),
                    scope_truth: None,
                }],
            },
        ],
    };
    let output = SearchPlannerAlpha::plan(inputs);
    ranking_reason_cards_for_planned_result_set(&output.query_session, &output.result_set)
}

fn discoverability_snapshot() -> aureline_shell::palette::AlphaPaletteDiscoverabilitySnapshot {
    let registry = seeded_registry();
    let mut shortcuts = HashMap::new();
    shortcuts.insert(
        "cmd:workspace.open_folder".to_string(),
        vec!["Cmd+O".to_string()],
    );
    shortcuts.insert(
        "cmd:command_palette.open".to_string(),
        vec!["Cmd+Shift+P".to_string()],
    );
    let runtime = CommandReviewRuntimeInputs {
        client_scope: "desktop_product",
        workspace_trust_state: "trusted",
        execution_context_available: true,
        provider_linked: None,
        credential_available: None,
        policy_disabled: false,
        policy_blocked_in_context: false,
        labs_enabled: false,
    };
    let recents = vec![AlphaRecentActionCandidate {
        recent_action_id: "recent:workspace.open_folder:01".to_string(),
        label: "Open Folder".to_string(),
        category_or_path: "Workspace - recent command".to_string(),
        command_id: Some("cmd:workspace.open_folder".to_string()),
        target_refs: vec!["workspace-scope:folder:recent:01".to_string()],
    }];
    let symbols = vec![AlphaSymbolCandidate {
        symbol_name: "open_workspace".to_string(),
        symbol_kind: "function".to_string(),
        relative_path: "src/workspace.rs".to_string(),
        symbol_anchor_ref: "symbol-anchor:workspace:open_workspace:decl".to_string(),
        origin_source_badge: "symbol_index".to_string(),
        freshness_state: "partial_index".to_string(),
    }];
    let files = vec![AlphaFileCandidate {
        relative_path: "src/open_workspace.rs".to_string(),
        path_identity_ref: "path-identity:workspace:src_open_workspace_rs".to_string(),
        origin_source_badge: "file_index".to_string(),
        freshness_state: "hot_set_ready".to_string(),
    }];

    materialize_alpha_palette_query(AlphaPaletteQueryInputs {
        registry: &registry,
        query: "open",
        shortcuts_by_command_id: &shortcuts,
        runtime,
        recent_actions: &recents,
        symbols: &symbols,
        files: &files,
    })
}

#[test]
fn search_alpha_validation_combines_ranking_discoverability_and_keyboard_routes() {
    let fixture = load_fixture();
    let mut cards = quick_open_cards();
    cards.extend(symbol_cards());
    let keyboard_audit = materialize_alpha_keyboard_gap_audit(
        &seeded_registry(),
        KeymapPresetId::VsCode,
        PlatformClass::Macos,
    );
    let discoverability = discoverability_snapshot();
    let packet = materialize_search_alpha_validation_packet(
        "search-alpha-validation:test",
        "mono:search-alpha:validation",
        &cards,
        &keyboard_audit,
        &[discoverability],
        vec![SEARCH_ALPHA_KNOWN_LIMIT.to_string()],
    );

    assert_eq!(fixture.fixture_id, "a11y.search_alpha.keyboard_parity");
    assert!(fixture.acceptance.result_ids_reused_from_card_contract);
    assert!(fixture.acceptance.ranking_reason_cards_keyboard_reachable);
    assert!(fixture.acceptance.planner_result_id_contract_required);
    assert_eq!(packet.record_kind, "search_alpha_validation_packet");
    assert!(packet.passes_acceptance(), "{packet:#?}");
    assert_eq!(packet.validation_state, "accepted_with_known_limits");
    assert!(
        packet
            .known_limit_refs
            .contains(&fixture.required_known_limit_ref),
        "packet should cite the search alpha known limit"
    );
    assert_eq!(
        packet.keyboard_review.reason_detail_surface_id,
        fixture.required_keyboard_surface_id
    );

    let surface_ids: BTreeSet<&str> = packet
        .surface_reviews
        .iter()
        .map(|surface| surface.surface.as_str())
        .collect();
    for required in &fixture.required_surface_ids {
        assert!(
            surface_ids.contains(required.as_str()),
            "missing search validation surface {required}"
        );
    }

    let ranking_reasons: BTreeSet<&str> = packet
        .surface_reviews
        .iter()
        .flat_map(|surface| surface.ranking_reason_classes.iter())
        .map(String::as_str)
        .collect();
    for required in &fixture.required_ranking_reason_classes {
        assert!(
            ranking_reasons.contains(required.as_str()),
            "missing ranking reason {required}"
        );
    }

    let row_kinds: BTreeSet<&str> = packet
        .discoverability_review
        .row_kind_tokens
        .iter()
        .map(String::as_str)
        .collect();
    for required in &fixture.acceptance.discoverability_row_kinds_required {
        assert!(
            row_kinds.contains(required.as_str()),
            "missing discoverability row kind {required}"
        );
    }
}
