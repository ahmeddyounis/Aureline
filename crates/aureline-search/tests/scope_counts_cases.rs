//! Fixture-driven coverage for scope-aware counts and hidden-scope disclosure.
//!
//! The cases under `fixtures/search/scope_counts_alpha/` exercise the first
//! search lane's empty-state distinctions and candidate scope markers. The
//! test validates the runtime projection rather than only the serialized
//! examples.

use std::path::Path;

use serde::Deserialize;

use aureline_reactive_state::ReadinessLabel;
use aureline_search::{
    HiddenScopeDisclosure, LexicalIndexInputs, LexicalIndexState, LexicalQuery,
    ScopeCandidateTruthRecord, ScopeTruthSurface, SearchNoResultsState, SearchScopeCountsRecord,
    WorkspaceSearchScope,
};
use aureline_workspace::{
    IncludedRootRef, MemberRef, MemberRefKind, MembershipPolicy, NarrowingCause, PartialTruthLabel,
    PatternEntry, PatternKind, PolicyLimitation, PortabilityMetadata, ReadinessMetadata,
    ReadinessState, ScopeClass as WorkspaceScopeClass, ScopeMode, SourceClass as WksSourceClass,
    WorksetArtifactRecord, WorksetArtifactRecordKind, WorksetPortabilityClass,
    WorkspaceLifecycleState, WorkspaceReadinessInputs, WorkspaceRootKind,
};

#[derive(Debug, Deserialize)]
struct ScopeCountsFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    workspace_id: String,
    observed_at: String,
    readiness_label: String,
    scope: ScopeFixture,
    files: Vec<String>,
    query: String,
    #[serde(default)]
    candidate: Option<CandidateFixture>,
    expect: ExpectFixture,
}

#[derive(Debug, Deserialize)]
struct ScopeFixture {
    scope_class: String,
    #[serde(default)]
    workset_id: Option<String>,
    workset_name: String,
    #[serde(default)]
    root_refs: Vec<String>,
    #[serde(default)]
    patterns: Vec<PatternFixture>,
    #[serde(default)]
    partial_index_note: Option<String>,
    #[serde(default)]
    policy_limited: bool,
    #[serde(default)]
    hidden_result_count_known: bool,
    #[serde(default)]
    hidden_result_count: Option<u64>,
    #[serde(default)]
    policy_hidden_member_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct PatternFixture {
    kind: String,
    pattern: String,
}

#[derive(Debug, Deserialize)]
struct CandidateFixture {
    surface_token: String,
    repo_or_module_ref: String,
    freshness_token: String,
    outside_current_scope: bool,
    policy_limited: bool,
}

#[derive(Debug, Deserialize)]
struct ExpectFixture {
    counts_class_token: String,
    visible_rows: u64,
    loaded_rows: u64,
    all_matching_rows: u64,
    hidden_by_current_scope_rows: u64,
    hidden_by_policy_rows: u64,
    empty_state_token: String,
    hidden_reason_tokens: Vec<String>,
    warning_tokens: Vec<String>,
    #[serde(default)]
    candidate_scope_label: Option<String>,
    #[serde(default)]
    candidate_repo_or_module_ref: Option<String>,
    #[serde(default)]
    candidate_freshness_token: Option<String>,
}

fn parse_readiness_label(token: &str) -> ReadinessLabel {
    match token {
        "exact" => ReadinessLabel::Exact,
        "partial" => ReadinessLabel::Partial,
        "unavailable" => ReadinessLabel::Unavailable,
        other => panic!("unknown readiness_label token: {other}"),
    }
}

fn parse_workspace_scope_class(token: &str) -> WorkspaceScopeClass {
    match token {
        "current_repo" => WorkspaceScopeClass::CurrentRepo,
        "selected_workset" => WorkspaceScopeClass::SelectedWorkset,
        "sparse_slice" => WorkspaceScopeClass::SparseSlice,
        "full_workspace" => WorkspaceScopeClass::FullWorkspace,
        "policy_limited_view" => WorkspaceScopeClass::PolicyLimitedView,
        other => panic!("unknown scope_class token: {other}"),
    }
}

fn parse_pattern_kind(token: &str) -> PatternKind {
    match token {
        "include" => PatternKind::Include,
        "exclude" => PatternKind::Exclude,
        other => panic!("unknown pattern kind: {other}"),
    }
}

fn search_scope(workspace_id: &str, fixture: &ScopeFixture) -> WorkspaceSearchScope {
    let scope_class = parse_workspace_scope_class(&fixture.scope_class);
    if scope_class == WorkspaceScopeClass::FullWorkspace {
        return WorkspaceSearchScope::for_full_workspace(workspace_id);
    }
    if scope_class == WorkspaceScopeClass::CurrentRepo {
        return WorkspaceSearchScope::for_current_repo(workspace_id);
    }

    let root_refs = if fixture.root_refs.is_empty() {
        vec!["repo:default".to_string()]
    } else {
        fixture.root_refs.clone()
    };
    let policy_limitation =
        if fixture.policy_limited || scope_class == WorkspaceScopeClass::PolicyLimitedView {
            Some(PolicyLimitation {
                underlying_workset_ref: format!(
                    "{}:underlying",
                    fixture.workset_id.as_deref().unwrap_or("wks:test")
                ),
                policy_ref: "policy:scope-counts:test".to_string(),
                narrowing_cause: NarrowingCause::AdminPolicy,
                visible_member_count: root_refs.len() as u32,
                hidden_member_count: fixture.policy_hidden_member_count.unwrap_or(1),
                hidden_member_list_visible: false,
            })
        } else {
            None
        };
    let member_refs = root_refs
        .iter()
        .map(|root_ref| MemberRef {
            ref_kind: MemberRefKind::Root,
            ref_id: root_ref.clone(),
            partial_truth: PartialTruthLabel::Loaded,
            presentation_label: Some(root_ref.clone()),
        })
        .collect();
    let artifact = WorksetArtifactRecord {
        record_kind: WorksetArtifactRecordKind::WorksetArtifactRecord,
        workset_artifact_schema_version: 1,
        workset_id: fixture
            .workset_id
            .clone()
            .unwrap_or_else(|| "wks:scope-counts:test".to_string()),
        scope_id: Some(format!(
            "scope:{}",
            fixture.workset_id.as_deref().unwrap_or("scope-counts-test")
        )),
        workset_name: fixture.workset_name.clone(),
        presentation_subtitle: None,
        scope_class,
        scope_mode: ScopeMode::Sparse,
        workspace_ref: Some(workspace_id.to_string()),
        root_refs: root_refs.clone(),
        included_roots: root_refs
            .iter()
            .map(|root_ref| IncludedRootRef {
                root_ref: root_ref.clone(),
                root_kind: WorkspaceRootKind::LocalRepoRoot,
                partial_truth: PartialTruthLabel::Loaded,
                presentation_label: Some(root_ref.clone()),
            })
            .collect(),
        patterns: fixture
            .patterns
            .iter()
            .map(|pattern| PatternEntry {
                pattern_kind: parse_pattern_kind(&pattern.kind),
                pattern: pattern.pattern.clone(),
                applies_to_root_ref: None,
            })
            .collect(),
        membership_policy: MembershipPolicy::GlobPattern,
        member_refs,
        policy_limitation,
        portability: PortabilityMetadata {
            source_class: WksSourceClass::WorkspaceShared,
            portability_class: WorksetPortabilityClass::PortableWithRebinding,
            includes_machine_local_refs: false,
            includes_managed_provider_refs: false,
            requires_rebinding_on_import: true,
            profile_sync_group_ref: None,
        },
        readiness: ReadinessMetadata {
            readiness_state: ReadinessState::Ready,
            hidden_result_count_known: fixture.hidden_result_count_known,
            hidden_result_count: fixture.hidden_result_count,
            partial_index_note: fixture.partial_index_note.clone(),
        },
        parent_workset_ref: None,
        manifest_source_ref: None,
        created_at: "mono:scope-counts:created".to_string(),
        updated_at: "mono:scope-counts:updated".to_string(),
        notes: None,
    };
    artifact.validate().expect("fixture artifact must validate");
    WorkspaceSearchScope::from_workset_artifact(workspace_id, &artifact)
}

fn surface_from(token: &str) -> ScopeTruthSurface {
    match token {
        "search_results" => ScopeTruthSurface::SearchResults,
        "graph_candidate" => ScopeTruthSurface::GraphCandidate,
        "ai_context_candidate" => ScopeTruthSurface::AiContextCandidate,
        other => panic!("unknown candidate surface token: {other}"),
    }
}

fn build_candidate_truth(
    scope: &WorkspaceSearchScope,
    counts: SearchScopeCountsRecord,
    hidden: Option<HiddenScopeDisclosure>,
    empty_state: SearchNoResultsState,
    fixture: &CandidateFixture,
) -> ScopeCandidateTruthRecord {
    ScopeCandidateTruthRecord::new(
        surface_from(&fixture.surface_token),
        scope.chip_label(),
        scope.scope_class().as_str(),
        Some(scope.stable_scope_id().to_string()),
        Some(scope.scope_mode().as_str().to_string()),
        Some(fixture.repo_or_module_ref.clone()),
        fixture.freshness_token.clone(),
        fixture.outside_current_scope,
        fixture.policy_limited,
        counts,
        empty_state,
        hidden,
        Vec::new(),
    )
}

#[test]
fn scope_counts_alpha_cases_match_runtime_projection() {
    let fixtures_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/search/scope_counts_alpha");
    let mut fixtures: Vec<_> = std::fs::read_dir(&fixtures_dir)
        .unwrap_or_else(|err| panic!("fixtures dir must exist at {fixtures_dir:?}: {err}"))
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixtures.sort();
    assert!(
        !fixtures.is_empty(),
        "at least one scope_counts_alpha fixture must exist"
    );

    for path in fixtures {
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let fixture: ScopeCountsFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
        assert_eq!(fixture.record_kind, "search_scope_counts_alpha_case");
        assert_eq!(fixture.schema_version, 1);

        let scope = search_scope(&fixture.workspace_id, &fixture.scope);
        let index = LexicalIndexState::from_inputs(LexicalIndexInputs {
            readiness_inputs: WorkspaceReadinessInputs {
                workspace_id: fixture.workspace_id.clone(),
                lifecycle_state_token: WorkspaceLifecycleState::Ready.as_str(),
                watcher_health_token: Some("healthy"),
                hot_index_ready: true,
                command_graph_ready: true,
                observed_at: fixture.observed_at.clone(),
            },
            readiness_label: parse_readiness_label(&fixture.readiness_label),
            files: fixture.files.clone(),
            scope: Some(scope.clone()),
        });
        let results = aureline_search::lexical::query::run_query(
            &index,
            &LexicalQuery::new(fixture.query.clone()),
        );

        assert_eq!(
            results.counts.counts_class_token, fixture.expect.counts_class_token,
            "counts class mismatch in {:?} ({})",
            path, fixture.case_name
        );
        assert_eq!(results.counts.visible_rows, fixture.expect.visible_rows);
        assert_eq!(results.counts.loaded_rows, Some(fixture.expect.loaded_rows));
        assert_eq!(
            results.counts.all_matching_rows,
            Some(fixture.expect.all_matching_rows)
        );
        assert_eq!(
            results.counts.hidden_by_current_scope_rows,
            fixture.expect.hidden_by_current_scope_rows
        );
        assert_eq!(
            results.counts.hidden_by_policy_rows,
            fixture.expect.hidden_by_policy_rows
        );
        assert_eq!(
            results.empty_state.as_str(),
            fixture.expect.empty_state_token
        );

        let hidden_reason_tokens = results
            .hidden_scope_disclosure
            .as_ref()
            .map(|hidden| hidden.reason_tokens.clone())
            .unwrap_or_default();
        assert_eq!(hidden_reason_tokens, fixture.expect.hidden_reason_tokens);
        let warning_tokens = results
            .scope_warnings
            .iter()
            .map(|warning| warning.warning_kind_token.clone())
            .collect::<Vec<_>>();
        assert_eq!(warning_tokens, fixture.expect.warning_tokens);

        if let Some(candidate) = fixture.candidate.as_ref() {
            let truth = build_candidate_truth(
                &scope,
                results.counts.clone(),
                results.hidden_scope_disclosure.clone(),
                results.empty_state,
                candidate,
            );
            assert_eq!(
                Some(truth.candidate_scope_label.as_str()),
                fixture.expect.candidate_scope_label.as_deref()
            );
            assert_eq!(
                truth.repo_or_module_ref.as_deref(),
                fixture.expect.candidate_repo_or_module_ref.as_deref()
            );
            assert_eq!(
                truth.freshness_token,
                fixture
                    .expect
                    .candidate_freshness_token
                    .as_deref()
                    .unwrap_or("")
            );
        }
    }
}
