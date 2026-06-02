//! Fixture-driven coverage for the stable search result-truth packet
//! (result identity, ranking reasons, action binding, scope counters,
//! and consumer projections).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_search::{
    current_stable_search_result_truth_packet, ActionFallbackModeClass, FactLabelClass,
    HistoryPolicyClass, SearchResultTruthConsumerSurface, SearchResultTruthFindingKind,
    SearchResultTruthPacket, SearchResultTruthPacketInput, SearchResultTruthPromotionState,
    SEARCH_RESULT_TRUTH_PACKET_ARTIFACT_DOC_REF, SEARCH_RESULT_TRUTH_PACKET_ARTIFACT_REF,
    SEARCH_RESULT_TRUTH_PACKET_DOC_REF, SEARCH_RESULT_TRUTH_PACKET_FIXTURE_DIR,
    SEARCH_RESULT_TRUTH_PACKET_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ResultTruthFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: SearchResultTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    fact_label_tokens: Vec<String>,
    result_kind_tokens: Vec<String>,
    contributing_stratum_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> ResultTruthFixture {
    let path = repo_root()
        .join(SEARCH_RESULT_TRUTH_PACKET_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "search_result_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = SearchResultTruthPacket::materialize(fixture.input.clone());
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
    assert_eq!(
        packet.fact_label_tokens(),
        expect
            .fact_label_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.result_kind_tokens(),
        expect
            .result_kind_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.contributing_stratum_tokens(),
        expect
            .contributing_stratum_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
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
    assert_exists(SEARCH_RESULT_TRUTH_PACKET_SCHEMA_REF);
    assert_exists(SEARCH_RESULT_TRUTH_PACKET_DOC_REF);
    assert_exists(SEARCH_RESULT_TRUTH_PACKET_ARTIFACT_DOC_REF);
    assert_exists(SEARCH_RESULT_TRUTH_PACKET_FIXTURE_DIR);
    assert_exists(SEARCH_RESULT_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_stable_fixture_certifies_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn dedupe_anchor_dropped_fixture_blocks_stable() {
    assert_fixture_matches("dedupe_anchor_dropped_blocks_stable.json");
}

#[test]
fn withheld_latency_direct_fallback_blocks_stable() {
    assert_fixture_matches("withheld_latency_direct_fallback_blocks_stable.json");
}

#[test]
fn captured_vs_live_dropped_fixture_blocks_stable() {
    assert_fixture_matches("captured_vs_live_dropped_blocks_stable.json");
}

#[test]
fn fact_label_dropped_fixture_blocks_stable() {
    assert_fixture_matches("fact_label_dropped_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_fact_label() {
    let packet = current_stable_search_result_truth_packet().expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        SearchResultTruthPromotionState::Stable
    );
    assert!(packet.validate().is_empty());

    let label_tokens: BTreeSet<&str> = packet.fact_label_tokens().into_iter().collect();
    for label in FactLabelClass::ALL {
        assert!(
            label_tokens.contains(label.as_str()),
            "checked-in packet must cover fact label {}",
            label.as_str()
        );
    }
    for surface in SearchResultTruthConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn closed_result_truth_tokens_are_pinned() {
    assert_eq!(FactLabelClass::Exact.as_str(), "exact");
    assert_eq!(FactLabelClass::ContextPromoted.as_str(), "context_promoted");
    assert_eq!(FactLabelClass::Semantic.as_str(), "semantic");
    assert_eq!(FactLabelClass::PartialIndex.as_str(), "partial_index");
    assert_eq!(FactLabelClass::WithheldLatency.as_str(), "withheld_latency");
    assert_eq!(FactLabelClass::PolicyHidden.as_str(), "policy_hidden");
    assert_eq!(
        ActionFallbackModeClass::OpenCapturedSnapshot.as_str(),
        "open_captured_snapshot"
    );
    assert_eq!(
        HistoryPolicyClass::SuppressForCapturedReplay.as_str(),
        "suppress_for_captured_replay"
    );
    assert_eq!(
        SearchResultTruthFindingKind::FactLabelSilentlyDropped.as_str(),
        "fact_label_silently_dropped"
    );
}
