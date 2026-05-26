//! Fixture-driven coverage for the stable saved-query / filter-AST /
//! scope-pack / column-preset portability and collection-truth packet.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_search::{
    current_stable_collection_portability_truth_packet,
    CollectionPortabilityConsumerSurface, CollectionPortabilityFindingKind,
    CollectionPortabilityPromotionState, CollectionPortabilityTruthPacket,
    CollectionPortabilityTruthPacketInput,
    COLLECTION_PORTABILITY_TRUTH_ARTIFACT_DOC_REF, COLLECTION_PORTABILITY_TRUTH_DOC_REF,
    COLLECTION_PORTABILITY_TRUTH_FIXTURE_DIR,
    COLLECTION_PORTABILITY_TRUTH_PACKET_ARTIFACT_REF, COLLECTION_PORTABILITY_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CaseFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: CollectionPortabilityTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    reopen_state_tokens: Vec<String>,
    surface_family_tokens: Vec<String>,
    support_export_safe: bool,
    #[serde(default)]
    expected_finding_kinds: Vec<String>,
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

fn load_fixture(file_name: &str) -> CaseFixture {
    let path = repo_root()
        .join(COLLECTION_PORTABILITY_TRUTH_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "collection_portability_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = CollectionPortabilityTruthPacket::materialize(fixture.input.clone());
    assert_eq!(
        packet.promotion_state.as_str(),
        expect.promotion_state,
        "fixture {} expected promotion {}, got {:?}; findings: {:?}",
        fixture.case_name,
        expect.promotion_state,
        packet.promotion_state,
        packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.rows.len(),
        expect.row_count,
        "fixture {} row count drift",
        fixture.case_name
    );
    assert_eq!(
        packet.validation_findings.len(),
        expect.validation_finding_count,
        "fixture {} finding count drift; got {:?}",
        fixture.case_name,
        packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.reopen_state_tokens(),
        expect
            .reopen_state_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} reopen state tokens drift",
        fixture.case_name,
    );
    assert_eq!(
        packet.surface_family_tokens(),
        expect
            .surface_family_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} surface family tokens drift",
        fixture.case_name,
    );

    let export = packet.support_export(
        format!("support-export:{}", fixture.case_name),
        "2026-05-26T12:00:10Z",
    );
    assert_eq!(
        export.is_export_safe(),
        expect.support_export_safe,
        "fixture {} support-export safety drift",
        fixture.case_name
    );

    if !expect.expected_finding_kinds.is_empty() {
        let observed: BTreeSet<&str> = packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect();
        for kind in &expect.expected_finding_kinds {
            assert!(
                observed.contains(kind.as_str()),
                "fixture {} expected finding kind {kind}; observed {:?}",
                fixture.case_name,
                observed
            );
        }
    }
}

#[test]
fn schema_doc_fixture_and_artifact_exist_on_disk() {
    assert_exists(COLLECTION_PORTABILITY_TRUTH_SCHEMA_REF);
    assert_exists(COLLECTION_PORTABILITY_TRUTH_DOC_REF);
    assert_exists(COLLECTION_PORTABILITY_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(COLLECTION_PORTABILITY_TRUTH_FIXTURE_DIR);
    assert_exists(COLLECTION_PORTABILITY_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_fixture_certifies_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn scope_binding_mismatch_blocks_stable() {
    assert_fixture_matches("scope_binding_mismatch_blocks_stable.json");
}

#[test]
fn missing_projection_blocks_stable() {
    assert_fixture_matches("missing_projection_blocks_stable.json");
}

#[test]
fn projection_drops_filter_ast_blocks_stable() {
    assert_fixture_matches("projection_drops_filter_ast_blocks_stable.json");
}

#[test]
fn scope_counter_vocabulary_dropped_blocks_stable() {
    assert_fixture_matches("scope_counter_vocabulary_dropped_blocks_stable.json");
}

#[test]
fn batch_review_required_but_missing_blocks_stable() {
    assert_fixture_matches("batch_review_required_but_missing_blocks_stable.json");
}

#[test]
fn reopen_state_coverage_over_declared_blocks_stable() {
    assert_fixture_matches("reopen_state_coverage_over_declared_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_reopen_state() {
    let packet = current_stable_collection_portability_truth_packet()
        .expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        CollectionPortabilityPromotionState::Stable
    );
    assert!(packet.validate().is_empty());

    let reopen_tokens: BTreeSet<&str> = packet.reopen_state_tokens().into_iter().collect();
    for token in [
        "captured_scope_still_current",
        "recipient_must_re_resolve",
        "current_scope_changed_rebind_required",
        "incompatible_artifact_migration_required",
    ] {
        assert!(
            reopen_tokens.contains(token),
            "checked-in packet must cover reopen state {token}; observed {:?}",
            reopen_tokens
        );
    }

    for surface in CollectionPortabilityConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn closed_finding_tokens_are_pinned() {
    assert_eq!(
        CollectionPortabilityFindingKind::FilterAstIdMismatch.as_str(),
        "filter_ast_id_mismatch"
    );
    assert_eq!(
        CollectionPortabilityFindingKind::ScopePackBindingMismatch.as_str(),
        "scope_pack_binding_mismatch"
    );
    assert_eq!(
        CollectionPortabilityFindingKind::ScopeCounterVocabularyDropped.as_str(),
        "scope_counter_vocabulary_dropped"
    );
    assert_eq!(
        CollectionPortabilityFindingKind::BatchReviewRequiredButMissing.as_str(),
        "batch_review_required_but_missing"
    );
    assert_eq!(
        CollectionPortabilityFindingKind::ProjectionFilterAstDropped.as_str(),
        "projection_filter_ast_dropped"
    );
    assert_eq!(
        CollectionPortabilityFindingKind::ReopenStateCoverageOverDeclared.as_str(),
        "reopen_state_coverage_over_declared"
    );
    assert_eq!(
        CollectionPortabilityFindingKind::MissingConsumerProjection.as_str(),
        "missing_consumer_projection"
    );
}
