//! Fixture-driven coverage for the stable search benchmark corpus,
//! ranking evaluation, and certified-archetype query-pack truth packet.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_search::{
    current_stable_search_benchmark_corpus_truth_packet, BenchmarkCorpusClass,
    CertifiedArchetypeClass, CorpusConsumerSurface, CorpusFindingKind, CorpusPromotionState,
    EvaluationDowngradeState, ProvenanceClass, QueryPackClass, RankingMetricClass,
    RetentionPolicyClass, SearchBenchmarkCorpusTruthPacket, SearchBenchmarkCorpusTruthPacketInput,
    SEARCH_BENCHMARK_CORPUS_TRUTH_ARTIFACT_DOC_REF, SEARCH_BENCHMARK_CORPUS_TRUTH_DOC_REF,
    SEARCH_BENCHMARK_CORPUS_TRUTH_FIXTURE_DIR, SEARCH_BENCHMARK_CORPUS_TRUTH_PACKET_ARTIFACT_REF,
    SEARCH_BENCHMARK_CORPUS_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CorpusFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: SearchBenchmarkCorpusTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    archetype_tokens: Vec<String>,
    corpus_class_tokens: Vec<String>,
    query_pack_tokens: Vec<String>,
    metric_tokens: Vec<String>,
    retention_policy_tokens: Vec<String>,
    provenance_tokens: Vec<String>,
    downgrade_state_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> CorpusFixture {
    let path = repo_root()
        .join(SEARCH_BENCHMARK_CORPUS_TRUTH_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_metric_set_matches(observed: &[&str], expected: &[String]) {
    let observed: BTreeSet<&str> = observed.iter().copied().collect();
    let expected: BTreeSet<&str> = expected.iter().map(String::as_str).collect();
    assert_eq!(
        observed, expected,
        "metric token set drift: observed={observed:?}, expected={expected:?}"
    );
}

fn assert_token_set_matches(observed: &[&str], expected: &[String], label: &str) {
    let observed: BTreeSet<&str> = observed.iter().copied().collect();
    let expected: BTreeSet<&str> = expected.iter().map(String::as_str).collect();
    assert_eq!(
        observed, expected,
        "{label} token set drift: observed={observed:?}, expected={expected:?}"
    );
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "search_benchmark_corpus_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = SearchBenchmarkCorpusTruthPacket::materialize(fixture.input.clone());
    assert_eq!(
        packet.promotion_state.as_str(),
        expect.promotion_state,
        "fixture {} expected promotion {}, got {:?}",
        fixture.case_name,
        expect.promotion_state,
        packet.promotion_state
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
    assert_token_set_matches(
        &packet.archetype_tokens(),
        &expect.archetype_tokens,
        "archetype",
    );
    assert_token_set_matches(
        &packet.corpus_class_tokens(),
        &expect.corpus_class_tokens,
        "corpus_class",
    );
    assert_token_set_matches(
        &packet.query_pack_tokens(),
        &expect.query_pack_tokens,
        "query_pack",
    );
    assert_metric_set_matches(&packet.metric_tokens(), &expect.metric_tokens);
    assert_token_set_matches(
        &packet.retention_policy_tokens(),
        &expect.retention_policy_tokens,
        "retention_policy",
    );
    assert_token_set_matches(
        &packet.provenance_tokens(),
        &expect.provenance_tokens,
        "provenance",
    );
    assert_token_set_matches(
        &packet.downgrade_state_tokens(),
        &expect.downgrade_state_tokens,
        "downgrade_state",
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
    assert_exists(SEARCH_BENCHMARK_CORPUS_TRUTH_SCHEMA_REF);
    assert_exists(SEARCH_BENCHMARK_CORPUS_TRUTH_DOC_REF);
    assert_exists(SEARCH_BENCHMARK_CORPUS_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(SEARCH_BENCHMARK_CORPUS_TRUTH_FIXTURE_DIR);
    assert_exists(SEARCH_BENCHMARK_CORPUS_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn metric_regression_fixture_blocks_stable() {
    assert_fixture_matches("metric_regression_without_waiver_blocks_stable.json");
}

#[test]
fn imported_pack_without_provenance_fixture_blocks_stable() {
    assert_fixture_matches("imported_pack_without_provenance_blocks_stable.json");
}

#[test]
fn corpus_redacted_undisclosed_fixture_blocks_stable() {
    assert_fixture_matches("corpus_redacted_undisclosed_blocks_stable.json");
}

#[test]
fn metric_vocabulary_collapsed_fixture_blocks_stable() {
    assert_fixture_matches("metric_vocabulary_collapsed_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_certifies_every_archetype_corpus() {
    let packet =
        current_stable_search_benchmark_corpus_truth_packet().expect("checked-in packet validates");
    assert_eq!(packet.promotion_state, CorpusPromotionState::Stable);
    assert!(packet.validate().is_empty());

    let required_corpus_classes = [
        BenchmarkCorpusClass::FileLookupCorpus,
        BenchmarkCorpusClass::SymbolNavigationCorpus,
        BenchmarkCorpusClass::DocsLookupCorpus,
        BenchmarkCorpusClass::SemanticRecallCorpus,
    ];
    for archetype in CertifiedArchetypeClass::ALL {
        for corpus_class in required_corpus_classes {
            assert!(
                packet.rows.iter().any(|row| row.archetype == archetype
                    && row.corpus_class == corpus_class),
                "certified archetype {} × corpus class {} must have at least one row in the stable packet",
                archetype.as_str(),
                corpus_class.as_str()
            );
        }
    }
    for surface in CorpusConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn closed_corpus_truth_tokens_are_pinned() {
    assert_eq!(
        BenchmarkCorpusClass::SemanticRecallCorpus.as_str(),
        "semantic_recall_corpus"
    );
    assert_eq!(
        QueryPackClass::GoldenQueryPack.as_str(),
        "golden_query_pack"
    );
    assert_eq!(RankingMetricClass::RecallAt50.as_str(), "recall_at_50");
    assert_eq!(
        RetentionPolicyClass::PublishedExternal.as_str(),
        "published_external"
    );
    assert_eq!(
        ProvenanceClass::CommunityContributed.as_str(),
        "community_contributed"
    );
    assert_eq!(
        EvaluationDowngradeState::CorpusRedacted.as_str(),
        "corpus_redacted"
    );
    assert_eq!(
        CorpusFindingKind::ImportedCorpusWithoutProvenance.as_str(),
        "imported_corpus_without_provenance"
    );
    assert_eq!(
        CorpusConsumerSurface::BenchmarkLab.as_str(),
        "benchmark_lab"
    );
}
