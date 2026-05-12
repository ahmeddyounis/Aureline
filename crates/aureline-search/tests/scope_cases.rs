//! Fixture-driven coverage for the workset/slice-aware search scope.
//!
//! Each fixture under `fixtures/search/scope_cases/` describes either a
//! single scope projection (full-workspace default, current-repo default,
//! workset artifact) or the workset-switch failure drill, paired with the
//! exact in-scope / out-of-scope partition the resolver MUST produce. The
//! test loads every fixture and asserts the projection round-trips, so the
//! protected-row truth vocabulary cannot drift without a fixture update.

use std::path::Path;

use serde::Deserialize;

use aureline_search::{glob_matches_relative_path, WorkspaceSearchScope};
use aureline_workspace::{
    MemberRef, MemberRefKind, MembershipPolicy, NarrowingCause, PartialTruthLabel, PatternEntry,
    PatternKind, PolicyLimitation, PortabilityMetadata, ReadinessMetadata, ReadinessState,
    SourceClass as WksSourceClass, WorksetArtifactRecord, WorksetArtifactRecordKind,
    WorksetPortabilityClass,
};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CaseFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    #[serde(default)]
    scenario: String,
    workspace_id: String,
    scope_kind: String,
    #[serde(default)]
    workset_artifact: Option<WorksetArtifactFixture>,
    #[serde(default)]
    from_workset_artifact: Option<WorksetArtifactFixture>,
    #[serde(default)]
    to_workset_artifact: Option<WorksetArtifactFixture>,
    #[serde(default)]
    files: Vec<String>,
    expect: serde_json::Value,
}

#[derive(Debug, Deserialize, Clone)]
struct WorksetArtifactFixture {
    workset_id: String,
    workset_name: String,
    scope_class: String,
    root_refs: Vec<String>,
    patterns: Vec<PatternFixture>,
    membership_policy: String,
    member_partial_truth: String,
    #[serde(default)]
    policy_limited: bool,
    #[serde(default)]
    partial_index_note: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct PatternFixture {
    kind: String,
    pattern: String,
    #[serde(default)]
    applies_to_root_ref: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ExpectedSingle {
    scope_class_token: String,
    chip_label: String,
    presentation_state_token: String,
    partial_scope: bool,
    is_policy_limited: bool,
    in_scope: Vec<String>,
    out_of_scope: Vec<String>,
    all_workspace_count: u64,
    in_scope_count: u64,
    include_pattern_count: u32,
    exclude_pattern_count: u32,
    #[serde(default)]
    partial_index_note: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ExpectedSwitch {
    before_switch: ExpectedSwitchSide,
    after_switch: ExpectedSwitchSide,
}

#[derive(Debug, Deserialize)]
struct ExpectedSwitchSide {
    scope_class_token: String,
    chip_label: String,
    in_scope: Vec<String>,
    out_of_scope: Vec<String>,
}

fn parse_scope_class(token: &str) -> aureline_workspace::ScopeClass {
    use aureline_workspace::ScopeClass;
    match token {
        "current_repo" => ScopeClass::CurrentRepo,
        "selected_workset" => ScopeClass::SelectedWorkset,
        "sparse_slice" => ScopeClass::SparseSlice,
        "full_workspace" => ScopeClass::FullWorkspace,
        "policy_limited_view" => ScopeClass::PolicyLimitedView,
        other => panic!("unknown scope_class token: {other}"),
    }
}

fn parse_partial_truth(token: &str) -> PartialTruthLabel {
    match token {
        "loaded" => PartialTruthLabel::Loaded,
        "manifest_known" => PartialTruthLabel::ManifestKnown,
        "cached" => PartialTruthLabel::Cached,
        "unavailable" => PartialTruthLabel::Unavailable,
        other => panic!("unknown member_partial_truth token: {other}"),
    }
}

fn parse_membership_policy(token: &str) -> MembershipPolicy {
    match token {
        "explicit_root_list" => MembershipPolicy::ExplicitRootList,
        "glob_pattern" => MembershipPolicy::GlobPattern,
        "dependency_graph_reachable" => MembershipPolicy::DependencyGraphReachable,
        "manifest_driven" => MembershipPolicy::ManifestDriven,
        other => panic!("unknown membership_policy token: {other}"),
    }
}

fn pattern_entry(pattern: &PatternFixture) -> PatternEntry {
    let kind = match pattern.kind.as_str() {
        "include" => PatternKind::Include,
        "exclude" => PatternKind::Exclude,
        other => panic!("unknown pattern kind: {other}"),
    };
    PatternEntry {
        pattern_kind: kind,
        pattern: pattern.pattern.clone(),
        applies_to_root_ref: pattern.applies_to_root_ref.clone(),
    }
}

fn build_artifact(fixture: &WorksetArtifactFixture) -> WorksetArtifactRecord {
    let scope_class = parse_scope_class(&fixture.scope_class);
    let member_partial = parse_partial_truth(&fixture.member_partial_truth);
    let policy_limitation = if matches!(
        scope_class,
        aureline_workspace::ScopeClass::PolicyLimitedView
    ) || fixture.policy_limited
    {
        Some(PolicyLimitation {
            underlying_workset_ref: format!("{}:underlying", fixture.workset_id),
            policy_ref: "policy:test:admin".to_string(),
            narrowing_cause: NarrowingCause::AdminPolicy,
            visible_member_count: fixture.root_refs.len() as u32,
            hidden_member_count: 1,
            hidden_member_list_visible: false,
        })
    } else {
        None
    };
    WorksetArtifactRecord {
        record_kind: WorksetArtifactRecordKind::WorksetArtifactRecord,
        workset_artifact_schema_version: 1,
        workset_id: fixture.workset_id.clone(),
        workset_name: fixture.workset_name.clone(),
        presentation_subtitle: None,
        scope_class,
        workspace_ref: Some("wksp:test".to_string()),
        root_refs: fixture.root_refs.clone(),
        patterns: fixture.patterns.iter().map(pattern_entry).collect(),
        membership_policy: parse_membership_policy(&fixture.membership_policy),
        member_refs: fixture
            .root_refs
            .iter()
            .map(|root_id| MemberRef {
                ref_kind: MemberRefKind::Root,
                ref_id: root_id.clone(),
                partial_truth: member_partial,
                presentation_label: Some(root_id.clone()),
            })
            .collect(),
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
            hidden_result_count_known: true,
            hidden_result_count: Some(0),
            partial_index_note: fixture.partial_index_note.clone(),
        },
        parent_workset_ref: None,
        manifest_source_ref: None,
        created_at: "mono:0".to_string(),
        updated_at: "mono:1".to_string(),
        notes: None,
    }
}

fn build_scope(workspace_id: &str, fixture: &CaseFixture) -> WorkspaceSearchScope {
    match fixture.scope_kind.as_str() {
        "full_workspace_default" => WorkspaceSearchScope::for_full_workspace(workspace_id),
        "current_repo_default" => WorkspaceSearchScope::for_current_repo(workspace_id),
        "workset_artifact" => {
            let artifact = build_artifact(
                fixture
                    .workset_artifact
                    .as_ref()
                    .expect("workset_artifact required for workset_artifact scope_kind"),
            );
            artifact
                .validate()
                .expect("synthesized artifact must validate");
            WorkspaceSearchScope::from_workset_artifact(workspace_id, &artifact)
        }
        other => panic!("unknown scope_kind: {other}"),
    }
}

fn assert_partition(
    scope: &WorkspaceSearchScope,
    files: &[String],
    expected_in_scope: &[String],
    expected_out_of_scope: &[String],
    case_name: &str,
) {
    let outcome = scope.filter_files(files.iter().cloned());
    let mut got_in = outcome.in_scope.clone();
    let mut got_out = outcome.out_of_scope.clone();
    got_in.sort();
    got_out.sort();
    let mut want_in = expected_in_scope.to_vec();
    let mut want_out = expected_out_of_scope.to_vec();
    want_in.sort();
    want_out.sort();
    assert_eq!(got_in, want_in, "in_scope mismatch in {case_name}");
    assert_eq!(got_out, want_out, "out_of_scope mismatch in {case_name}");
}

#[test]
fn scope_cases_match_expected_partition() {
    let fixtures_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/search/scope_cases");
    let mut fixtures: Vec<_> = std::fs::read_dir(&fixtures_dir)
        .unwrap_or_else(|err| panic!("fixtures dir must exist at {fixtures_dir:?}: {err}"))
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
        .collect();
    fixtures.sort();

    assert!(
        !fixtures.is_empty(),
        "at least one search_scope_case fixture must exist"
    );

    for path in fixtures {
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
        let fixture: CaseFixture = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
        assert_eq!(
            fixture.record_kind, "search_scope_case",
            "unexpected record_kind in {path:?}"
        );
        assert_eq!(
            fixture.schema_version, 1,
            "unexpected schema_version in {path:?}"
        );

        match fixture.scope_kind.as_str() {
            "workset_switch" => verify_switch(&fixture, &path),
            _ => verify_single(&fixture, &path),
        }
    }
}

fn verify_single(fixture: &CaseFixture, path: &Path) {
    let expected: ExpectedSingle = serde_json::from_value(fixture.expect.clone())
        .unwrap_or_else(|err| panic!("expect block in {path:?} must parse: {err}"));
    let scope = build_scope(&fixture.workspace_id, fixture);

    assert_eq!(
        scope.scope_class().as_str(),
        expected.scope_class_token,
        "scope_class_token mismatch in {path:?}"
    );
    assert_eq!(
        scope.chip_label(),
        expected.chip_label,
        "chip_label mismatch in {path:?}"
    );
    assert_eq!(
        scope.presentation_state().as_str(),
        expected.presentation_state_token,
        "presentation_state_token mismatch in {path:?}"
    );
    assert_eq!(
        scope.is_partial_scope(),
        expected.partial_scope,
        "partial_scope mismatch in {path:?}"
    );
    assert_eq!(
        scope.is_policy_limited(),
        expected.is_policy_limited,
        "is_policy_limited mismatch in {path:?}"
    );
    assert_eq!(
        scope.partial_index_note(),
        expected.partial_index_note.as_deref(),
        "partial_index_note mismatch in {path:?}"
    );

    let metadata = scope.project_metadata();
    assert_eq!(
        metadata.include_pattern_count, expected.include_pattern_count,
        "include_pattern_count mismatch in {path:?}"
    );
    assert_eq!(
        metadata.exclude_pattern_count, expected.exclude_pattern_count,
        "exclude_pattern_count mismatch in {path:?}"
    );

    let outcome = scope.filter_files(fixture.files.iter().cloned());
    assert_eq!(
        outcome.all_workspace_count, expected.all_workspace_count,
        "all_workspace_count mismatch in {path:?}"
    );
    assert_eq!(
        outcome.in_scope_count, expected.in_scope_count,
        "in_scope_count mismatch in {path:?}"
    );
    assert_partition(
        &scope,
        &fixture.files,
        &expected.in_scope,
        &expected.out_of_scope,
        &fixture.case_name,
    );
}

fn verify_switch(fixture: &CaseFixture, path: &Path) {
    let expected: ExpectedSwitch = serde_json::from_value(fixture.expect.clone())
        .unwrap_or_else(|err| panic!("expect block in {path:?} must parse: {err}"));
    let from_artifact = build_artifact(
        fixture
            .from_workset_artifact
            .as_ref()
            .expect("from_workset_artifact required for workset_switch"),
    );
    let to_artifact = build_artifact(
        fixture
            .to_workset_artifact
            .as_ref()
            .expect("to_workset_artifact required for workset_switch"),
    );
    from_artifact
        .validate()
        .expect("from artifact must validate");
    to_artifact.validate().expect("to artifact must validate");

    let from_scope =
        WorkspaceSearchScope::from_workset_artifact(&fixture.workspace_id, &from_artifact);
    let to_scope = WorkspaceSearchScope::from_workset_artifact(&fixture.workspace_id, &to_artifact);

    assert_eq!(
        from_scope.scope_class().as_str(),
        expected.before_switch.scope_class_token,
        "before scope_class_token mismatch in {path:?}"
    );
    assert_eq!(
        from_scope.chip_label(),
        expected.before_switch.chip_label,
        "before chip_label mismatch in {path:?}"
    );
    assert_partition(
        &from_scope,
        &fixture.files,
        &expected.before_switch.in_scope,
        &expected.before_switch.out_of_scope,
        &format!("{}::before_switch", fixture.case_name),
    );

    assert_eq!(
        to_scope.scope_class().as_str(),
        expected.after_switch.scope_class_token,
        "after scope_class_token mismatch in {path:?}"
    );
    assert_eq!(
        to_scope.chip_label(),
        expected.after_switch.chip_label,
        "after chip_label mismatch in {path:?}"
    );
    assert_partition(
        &to_scope,
        &fixture.files,
        &expected.after_switch.in_scope,
        &expected.after_switch.out_of_scope,
        &format!("{}::after_switch", fixture.case_name),
    );

    // Failure-drill invariant: the chip MUST update before any from-scope row
    // can leak through. Capture the contract at row level by asserting the
    // 'from' rows are now classified as out-of-scope under the 'to' scope.
    // Use the canonical pattern primitive directly so the assertion does
    // not re-derive scope semantics.
    for row in &expected.before_switch.in_scope {
        assert!(
            !to_scope.matches_relative_path(row),
            "row {row} from before_switch still matches the after_switch scope in {path:?}"
        );
        // Equivalent assertion using the freestanding primitive — pins the
        // contract exposed to callers that bypass the WorkspaceSearchScope
        // wrapper.
        assert!(
            !glob_matches_relative_path(row, to_scope.patterns()) || to_scope.patterns().is_empty(),
            "primitive check leaked row {row} into the after_switch scope in {path:?}"
        );
    }
}
