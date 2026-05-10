//! Fixture-driven coverage for the scope-truth chip card projection on
//! the live shell's open and search foundations.
//!
//! Each case under `fixtures/workspace/scope_truth_cases/*.json` exercises
//! one row of the scope-truth contract: nominal full workspace, narrowed
//! workset, sparse-slice partial index, policy-limited admin view, and an
//! outside-current-scope search row. The fixtures drive both the bare
//! [`ScopeClass`]-only projection path and the full
//! [`WorksetArtifactRecord`]-driven projection path so the chip vocabulary
//! cannot drift between callers.

use std::path::Path;

use serde::Deserialize;

use aureline_shell::scope_truth::{
    project_outside_scope_truth_chip_card, project_scope_truth_chip_card,
    project_scope_truth_chip_card_for_artifact, ScopeCountsRecord, ScopeTruthChipCard,
    ScopeTruthSurfaceClass,
};
use aureline_shell::scope_truth::counts::ScopeCountsInputs;
use aureline_workspace::{ScopeClass as WorkspaceScopeClass, WorksetArtifactRecord};

#[derive(Debug, Clone, Deserialize)]
struct ScopeTruthFixture {
    #[allow(dead_code)]
    record_kind: String,
    #[allow(dead_code)]
    schema_version: u32,
    case_name: String,
    #[allow(dead_code)]
    scenario: String,
    input: FixtureInput,
    artifact: WorksetArtifactRecord,
    expect: ExpectBlock,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum FixtureInput {
    /// Project from the full workset artifact.
    WorksetArtifact {
        workspace_id: String,
        surface_class: String,
        chip_id: String,
        emitted_at: String,
        counts: FixtureCountsInput,
    },
    /// Project from the bare scope class with an optional workset name.
    BareScopeClass {
        workspace_id: String,
        surface_class: String,
        scope_class: String,
        #[serde(default)]
        workset_name: Option<String>,
        emitted_at: String,
        counts: FixtureCountsInput,
    },
    /// Project an outside-current-scope chip from the artifact.
    OutsideScopeChip {
        workspace_id: String,
        surface_class: String,
        chip_id: String,
        emitted_at: String,
        counts: FixtureCountsInput,
    },
}

#[derive(Debug, Clone, Deserialize)]
struct FixtureCountsInput {
    visible_in_view: u64,
    #[serde(default)]
    loaded_in_scope: Option<u64>,
    #[serde(default)]
    all_matching_in_workspace: Option<u64>,
    scope_covers_workspace: bool,
    readiness_is_ready: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct ExpectBlock {
    scope_class_token: String,
    presentation_state_token: String,
    chip_label: String,
    partial_scope: bool,
    #[serde(default)]
    workset_id: Option<String>,
    #[serde(default)]
    workset_name: Option<String>,
    #[serde(default)]
    root_count: Option<u32>,
    #[serde(default)]
    member_count: Option<u32>,
    outside_current_scope_marker_visible: bool,
    #[serde(default)]
    hidden_result_count_class: Option<String>,
    #[serde(default)]
    hidden_result_count: Option<u64>,
    #[serde(default)]
    partial_index_note: Option<String>,
    must_offer_action_tokens: Vec<String>,
    #[serde(default)]
    must_not_offer_action_tokens: Vec<String>,
    counts: ExpectCountsBlock,
}

#[derive(Debug, Clone, Deserialize)]
struct ExpectCountsBlock {
    counts_class_token: String,
    visible_in_view: u64,
    #[serde(default)]
    loaded_in_scope: Option<u64>,
    #[serde(default)]
    all_matching_in_workspace: Option<u64>,
}

fn fixtures_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/workspace/scope_truth_cases")
}

fn surface_class_from(token: &str) -> ScopeTruthSurfaceClass {
    match token {
        "explorer" => ScopeTruthSurfaceClass::Explorer,
        "quick_open" => ScopeTruthSurfaceClass::QuickOpen,
        "search_shell" => ScopeTruthSurfaceClass::SearchShell,
        "docs_browser" => ScopeTruthSurfaceClass::DocsBrowser,
        "open_flow_sheet" => ScopeTruthSurfaceClass::OpenFlowSheet,
        "support_packet" => ScopeTruthSurfaceClass::SupportPacket,
        other => panic!("unsupported surface_class token: {other}"),
    }
}

fn workspace_scope_from(token: &str) -> WorkspaceScopeClass {
    match token {
        "current_repo" => WorkspaceScopeClass::CurrentRepo,
        "selected_workset" => WorkspaceScopeClass::SelectedWorkset,
        "sparse_slice" => WorkspaceScopeClass::SparseSlice,
        "full_workspace" => WorkspaceScopeClass::FullWorkspace,
        "policy_limited_view" => WorkspaceScopeClass::PolicyLimitedView,
        other => panic!("unsupported scope_class token: {other}"),
    }
}

fn counts_from(input: &FixtureCountsInput) -> ScopeCountsRecord {
    ScopeCountsRecord::derive(ScopeCountsInputs {
        visible_in_view: input.visible_in_view,
        loaded_in_scope: input.loaded_in_scope,
        all_matching_in_workspace: input.all_matching_in_workspace,
        scope_covers_workspace: input.scope_covers_workspace,
        readiness_is_ready: input.readiness_is_ready,
    })
}

fn project_card(fixture: &ScopeTruthFixture) -> ScopeTruthChipCard {
    match &fixture.input {
        FixtureInput::WorksetArtifact {
            workspace_id,
            surface_class,
            chip_id,
            emitted_at,
            counts,
        } => project_scope_truth_chip_card_for_artifact(
            workspace_id.clone(),
            surface_class_from(surface_class),
            &fixture.artifact,
            counts_from(counts),
            chip_id.clone(),
            emitted_at.clone(),
        ),
        FixtureInput::BareScopeClass {
            workspace_id,
            surface_class,
            scope_class,
            workset_name,
            emitted_at,
            counts,
        } => project_scope_truth_chip_card(
            workspace_id.clone(),
            surface_class_from(surface_class),
            workspace_scope_from(scope_class),
            workset_name.as_deref(),
            counts_from(counts),
            emitted_at.clone(),
        ),
        FixtureInput::OutsideScopeChip {
            workspace_id,
            surface_class,
            chip_id,
            emitted_at,
            counts,
        } => project_outside_scope_truth_chip_card(
            workspace_id.clone(),
            surface_class_from(surface_class),
            &fixture.artifact,
            counts_from(counts),
            chip_id.clone(),
            emitted_at.clone(),
        ),
    }
}

fn assert_fixture(path: &Path, fixture: &ScopeTruthFixture) {
    let card = project_card(fixture);

    assert_eq!(
        card.scope_class_token, fixture.expect.scope_class_token,
        "scope_class_token mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        card.presentation_state_token, fixture.expect.presentation_state_token,
        "presentation_state_token mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        card.chip_label, fixture.expect.chip_label,
        "chip_label mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        card.partial_scope, fixture.expect.partial_scope,
        "partial_scope mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        card.workset_id, fixture.expect.workset_id,
        "workset_id mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        card.workset_name, fixture.expect.workset_name,
        "workset_name mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        card.root_count, fixture.expect.root_count,
        "root_count mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        card.member_count, fixture.expect.member_count,
        "member_count mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        card.outside_current_scope_marker_visible,
        fixture.expect.outside_current_scope_marker_visible,
        "outside_current_scope_marker_visible mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        card.hidden_result_count_class, fixture.expect.hidden_result_count_class,
        "hidden_result_count_class mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        card.hidden_result_count, fixture.expect.hidden_result_count,
        "hidden_result_count mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        card.partial_index_note, fixture.expect.partial_index_note,
        "partial_index_note mismatch in {path:?} ({})",
        fixture.case_name
    );
    for token in &fixture.expect.must_offer_action_tokens {
        assert!(
            card.offered_action_tokens.iter().any(|t| t == token),
            "missing required action {token} in {path:?} ({})\nactions: {:?}",
            fixture.case_name,
            card.offered_action_tokens
        );
    }
    for token in &fixture.expect.must_not_offer_action_tokens {
        assert!(
            !card.offered_action_tokens.iter().any(|t| t == token),
            "forbidden action {token} present in {path:?} ({})\nactions: {:?}",
            fixture.case_name,
            card.offered_action_tokens
        );
    }
    assert_eq!(
        card.counts.counts_class_token, fixture.expect.counts.counts_class_token,
        "counts_class_token mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        card.counts.visible_in_view, fixture.expect.counts.visible_in_view,
        "visible_in_view mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        card.counts.loaded_in_scope, fixture.expect.counts.loaded_in_scope,
        "loaded_in_scope mismatch in {path:?} ({})",
        fixture.case_name
    );
    assert_eq!(
        card.counts.all_matching_in_workspace, fixture.expect.counts.all_matching_in_workspace,
        "all_matching_in_workspace mismatch in {path:?} ({})",
        fixture.case_name
    );

    // Survives serialization/export round-trip.
    let json = serde_json::to_string(&card).expect("scope-truth chip card must serialize");
    let parsed: ScopeTruthChipCard =
        serde_json::from_str(&json).expect("scope-truth chip card must round-trip");
    assert_eq!(parsed, card, "round-trip mismatch in {path:?}");
}

#[test]
fn scope_truth_chip_cases_match_fixtures() {
    let dir = fixtures_dir();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("scope_truth_cases dir must exist")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    assert!(
        paths.len() >= 4,
        "expected at least 4 scope-truth fixtures, found {}",
        paths.len()
    );

    for path in &paths {
        let payload = std::fs::read_to_string(path).expect("fixture must read");
        let fixture: ScopeTruthFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("failed to parse fixture {path:?}: {err}"));
        assert_fixture(path, &fixture);
    }
}
