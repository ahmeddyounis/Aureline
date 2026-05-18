//! Fixture-driven coverage for saved-query artifact privacy.
//!
//! The drill verifies the schema, docs, fixtures, and Rust artifact layer that
//! preserve local-first search retention defaults, permission-safe deep links,
//! and raw-query-free support exports.

use std::path::{Path, PathBuf};

use aureline_search::{
    QueryHistoryEntry, SavedQuery, SavedQueryPrivacyClass, SavedQuerySharePolicy,
    SavedQuerySourceClass, ScopePackBinding, SearchArtifactMaterializationInput, SearchArtifactSet,
    SearchArtifactValidationFinding, SearchArtifactValidationFindingKind, SearchCollectionSnapshot,
    SearchDeepLink, SearchExportDestination, SearchPlannerAlpha, SearchPlannerInputs,
    SearchRedactionProfile, SearchRetentionMode, SearchRetentionWideningBasis,
    SearchScopeCountsRecord, SearchSyncClass, QUERY_HISTORY_SCHEMA_REF,
    SAVED_QUERY_EXPORT_PRIVACY_DOC_REF, SAVED_QUERY_PRIVACY_FIXTURE_DIR, SAVED_QUERY_SCHEMA_REF,
    SEARCH_EXPORT_SNAPSHOT_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CaseFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: CaseInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct CaseInput {
    artifact: ArtifactInput,
    planner: SearchPlannerInputs,
}

#[derive(Debug, Deserialize)]
struct ArtifactInput {
    saved_query_id: String,
    history_id: String,
    scope_binding_id: String,
    deep_link_id: String,
    snapshot_id: String,
    display_name: String,
    source_class: SavedQuerySourceClass,
    privacy_class: SavedQueryPrivacyClass,
    share_policy: SavedQuerySharePolicy,
    destination: SearchExportDestination,
    #[serde(default)]
    selected_result_ids: Vec<String>,
    #[serde(default)]
    scope_counts: Option<SearchScopeCountsRecord>,
    #[serde(default)]
    retention_mode: Option<SearchRetentionMode>,
    #[serde(default)]
    sync_class: Option<SearchSyncClass>,
    #[serde(default)]
    redaction_profile: Option<SearchRedactionProfile>,
    #[serde(default)]
    retention_widening_basis: Option<SearchRetentionWideningBasis>,
    created_at: String,
    last_used_at: String,
    exported_at: String,
    #[serde(default)]
    expires_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    retention_mode: String,
    sync_class: String,
    redaction_profile: String,
    retention_widening_basis: String,
    saved_has_raw_query_text: bool,
    saved_has_query_hash: bool,
    history_stored_text_mode: String,
    deep_link_result_semantics: String,
    deep_link_rerun_required: bool,
    deep_link_re_resolves_under_permissions: bool,
    deep_link_access_widening_allowed: bool,
    snapshot_result_semantics: String,
    snapshot_literal_query_included: bool,
    snapshot_raw_query_free_by_default: bool,
    snapshot_partiality_reasons: Vec<String>,
    validation_findings: Vec<String>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root canonicalizes")
}

fn assert_exists(rel: &str) {
    let path = repo_root().join(rel);
    assert!(
        path.exists(),
        "expected path to exist on disk: {} ({})",
        rel,
        path.display()
    );
}

fn load_fixtures() -> Vec<(PathBuf, CaseFixture)> {
    let fixtures_dir = repo_root().join(SAVED_QUERY_PRIVACY_FIXTURE_DIR);
    let mut paths: Vec<_> = std::fs::read_dir(&fixtures_dir)
        .unwrap_or_else(|err| panic!("fixtures dir must exist at {fixtures_dir:?}: {err}"))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();

    assert!(
        !paths.is_empty(),
        "at least one saved-query privacy beta fixture must exist"
    );

    paths
        .into_iter()
        .map(|path| {
            let payload = std::fs::read_to_string(&path)
                .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
            let fixture = serde_json::from_str(&payload)
                .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
            (path, fixture)
        })
        .collect()
}

fn materialize(fixture: &CaseFixture) -> SearchArtifactSet {
    let output = SearchPlannerAlpha::plan(fixture.input.planner.clone());
    let artifact = &fixture.input.artifact;
    SearchArtifactSet::materialize(SearchArtifactMaterializationInput {
        saved_query_id: artifact.saved_query_id.clone(),
        history_id: artifact.history_id.clone(),
        scope_binding_id: artifact.scope_binding_id.clone(),
        deep_link_id: artifact.deep_link_id.clone(),
        snapshot_id: artifact.snapshot_id.clone(),
        display_name: artifact.display_name.clone(),
        source_class: artifact.source_class,
        privacy_class: artifact.privacy_class,
        share_policy: artifact.share_policy,
        destination: artifact.destination,
        query_session: output.query_session,
        result_set: output.result_set,
        selected_result_ids: artifact.selected_result_ids.clone(),
        scope_counts: artifact.scope_counts.clone(),
        retention_mode: artifact.retention_mode,
        sync_class: artifact.sync_class,
        redaction_profile: artifact.redaction_profile,
        retention_widening_basis: artifact.retention_widening_basis,
        created_at: artifact.created_at.clone(),
        last_used_at: artifact.last_used_at.clone(),
        exported_at: artifact.exported_at.clone(),
        expires_at: artifact.expires_at.clone(),
    })
    .expect("fixture artifacts materialize")
}

#[test]
fn schema_doc_and_fixture_paths_exist() {
    assert_exists(SAVED_QUERY_SCHEMA_REF);
    assert_exists(QUERY_HISTORY_SCHEMA_REF);
    assert_exists(SEARCH_EXPORT_SNAPSHOT_SCHEMA_REF);
    assert_exists(SAVED_QUERY_EXPORT_PRIVACY_DOC_REF);
    assert_exists(SAVED_QUERY_PRIVACY_FIXTURE_DIR);
}

#[test]
fn saved_query_privacy_fixtures_match_expected_artifacts() {
    for (path, fixture) in load_fixtures() {
        assert_eq!(
            fixture.record_kind, "saved_query_privacy_beta_case",
            "unexpected record kind in {path:?}"
        );
        assert_eq!(fixture.schema_version, 1, "schema version in {path:?}");
        assert!(
            !fixture.case_name.trim().is_empty(),
            "case name should be reviewable in {path:?}"
        );
        assert!(
            !fixture.scenario.trim().is_empty(),
            "scenario should be reviewable in {path:?}"
        );

        let artifacts = materialize(&fixture);
        assert_scope_binding(&artifacts.scope_binding, &fixture.expect, path.as_path());
        assert_saved_query(&artifacts.saved_query, &fixture.expect, path.as_path());
        assert_history_entry(&artifacts.history_entry, &fixture.expect, path.as_path());
        assert_deep_link(&artifacts.deep_link, &fixture.expect, path.as_path());
        assert_snapshot(
            &artifacts.collection_snapshot,
            &fixture.expect,
            path.as_path(),
        );
        assert_eq!(
            finding_tokens(&artifacts.validate()),
            fixture.expect.validation_findings,
            "validation findings mismatch in {path:?}"
        );
    }
}

#[test]
fn raw_query_text_in_support_snapshot_is_rejected() {
    let fixture = load_fixtures()
        .into_iter()
        .find(|(_, fixture)| fixture.case_name == "support_export_redacted_snapshot")
        .expect("support fixture exists")
        .1;
    let mut artifacts = materialize(&fixture);
    artifacts.collection_snapshot.literal_query_text_included = true;

    let findings = artifacts.collection_snapshot.validate();
    assert!(findings.iter().any(|finding| {
        finding.finding_kind == SearchArtifactValidationFindingKind::RawQueryMaterialNotLocalOnly
    }));
}

#[test]
fn deep_links_that_widen_access_are_rejected() {
    let fixture = load_fixtures().remove(0).1;
    let mut artifacts = materialize(&fixture);
    artifacts.deep_link.access_widening_allowed = true;

    let findings = artifacts.deep_link.validate();
    assert!(findings.iter().any(|finding| {
        finding.finding_kind == SearchArtifactValidationFindingKind::DeepLinkWouldWidenAccess
    }));
}

fn assert_scope_binding(binding: &ScopePackBinding, expect: &ExpectedFixture, path: &Path) {
    assert_eq!(
        binding.retention_mode.as_str(),
        expect.retention_mode,
        "scope retention mismatch in {path:?}"
    );
    assert_eq!(
        binding.sync_class.as_str(),
        expect.sync_class,
        "scope sync mismatch in {path:?}"
    );
    assert_eq!(
        binding.redaction_profile.as_str(),
        expect.redaction_profile,
        "scope redaction mismatch in {path:?}"
    );
    assert_eq!(
        binding.retention_widening_basis.as_str(),
        expect.retention_widening_basis,
        "scope widening basis mismatch in {path:?}"
    );
}

fn assert_saved_query(saved_query: &SavedQuery, expect: &ExpectedFixture, path: &Path) {
    assert_eq!(
        saved_query.retention_mode.as_str(),
        expect.retention_mode,
        "saved retention mismatch in {path:?}"
    );
    assert_eq!(
        saved_query.sync_class.as_str(),
        expect.sync_class,
        "saved sync mismatch in {path:?}"
    );
    assert_eq!(
        saved_query.redaction_profile.as_str(),
        expect.redaction_profile,
        "saved redaction mismatch in {path:?}"
    );
    assert_eq!(
        saved_query.retention_widening_basis.as_str(),
        expect.retention_widening_basis,
        "saved widening basis mismatch in {path:?}"
    );
    assert_eq!(
        saved_query.contains_raw_query_text(),
        expect.saved_has_raw_query_text,
        "saved raw text presence mismatch in {path:?}"
    );
    assert_eq!(
        saved_query.query_hash.is_some(),
        expect.saved_has_query_hash,
        "saved query hash presence mismatch in {path:?}"
    );
}

fn assert_history_entry(history: &QueryHistoryEntry, expect: &ExpectedFixture, path: &Path) {
    assert_eq!(
        history.stored_text_mode.as_str(),
        expect.history_stored_text_mode,
        "history stored text mode mismatch in {path:?}"
    );
    assert_eq!(
        history.retention_mode.as_str(),
        expect.retention_mode,
        "history retention mismatch in {path:?}"
    );
    assert_eq!(
        history.sync_class.as_str(),
        expect.sync_class,
        "history sync mismatch in {path:?}"
    );
}

fn assert_deep_link(deep_link: &SearchDeepLink, expect: &ExpectedFixture, path: &Path) {
    assert_eq!(
        deep_link.result_semantics.as_str(),
        expect.deep_link_result_semantics,
        "deep-link result semantics mismatch in {path:?}"
    );
    assert_eq!(
        deep_link.rerun_required, expect.deep_link_rerun_required,
        "deep-link rerun flag mismatch in {path:?}"
    );
    assert_eq!(
        deep_link.recipient_re_resolves_under_current_permissions,
        expect.deep_link_re_resolves_under_permissions,
        "deep-link re-resolution flag mismatch in {path:?}"
    );
    assert_eq!(
        deep_link.access_widening_allowed, expect.deep_link_access_widening_allowed,
        "deep-link access widening flag mismatch in {path:?}"
    );
}

fn assert_snapshot(snapshot: &SearchCollectionSnapshot, expect: &ExpectedFixture, path: &Path) {
    assert_eq!(
        snapshot.result_semantics.as_str(),
        expect.snapshot_result_semantics,
        "snapshot result semantics mismatch in {path:?}"
    );
    assert_eq!(
        snapshot.literal_query_text_included, expect.snapshot_literal_query_included,
        "snapshot literal query flag mismatch in {path:?}"
    );
    assert_eq!(
        snapshot.export_avoids_raw_query_by_default(),
        expect.snapshot_raw_query_free_by_default,
        "snapshot raw-query-free default mismatch in {path:?}"
    );
    assert_eq!(
        snapshot.partiality_reasons, expect.snapshot_partiality_reasons,
        "snapshot partiality reasons mismatch in {path:?}"
    );
}

fn finding_tokens(findings: &[SearchArtifactValidationFinding]) -> Vec<String> {
    findings
        .iter()
        .map(|finding| finding.finding_kind.as_str().to_string())
        .collect()
}
