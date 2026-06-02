//! Fixture-driven coverage for the stable docs-browser truth packet
//! (source descriptors, result objects, machine-readable enums, symbol-linked
//! flows, and consumer projections).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_docs::{
    current_stable_docs_browser_truth_packet, seeded_stable_docs_browser_truth_packet_input,
    DocsBrowserConsumerSurface, DocsBrowserFindingKind, DocsBrowserFreshnessState,
    DocsBrowserPromotionState, DocsBrowserSourceClass, DocsBrowserTruthPacket,
    DocsBrowserTruthPacketInput, DocsBrowserVersionMatchState,
    DOCS_BROWSER_TRUTH_PACKET_ARTIFACT_DOC_REF, DOCS_BROWSER_TRUTH_PACKET_ARTIFACT_REF,
    DOCS_BROWSER_TRUTH_PACKET_DOC_REF, DOCS_BROWSER_TRUTH_PACKET_FIXTURE_DIR,
    DOCS_BROWSER_TRUTH_PACKET_MILESTONE_DOC_REF, DOCS_BROWSER_TRUTH_PACKET_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DocsBrowserFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: DocsBrowserTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
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

fn load_fixture(file_name: &str) -> DocsBrowserFixture {
    let path = repo_root()
        .join(DOCS_BROWSER_TRUTH_PACKET_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "docs_browser_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let packet = DocsBrowserTruthPacket::materialize(fixture.input.clone());
    assert_eq!(
        packet.promotion_state.as_str(),
        fixture.expect.promotion_state,
        "fixture {} expected promotion {}, got {:?}; findings: {:?}",
        fixture.case_name,
        fixture.expect.promotion_state,
        packet.promotion_state,
        packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect::<Vec<_>>()
    );

    if !fixture.expect.expected_finding_kinds.is_empty() {
        let observed: BTreeSet<&str> = packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect();
        for kind in &fixture.expect.expected_finding_kinds {
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
fn doc_fixture_schema_and_artifact_exist_on_disk() {
    assert_exists(DOCS_BROWSER_TRUTH_PACKET_DOC_REF);
    assert_exists(DOCS_BROWSER_TRUTH_PACKET_MILESTONE_DOC_REF);
    assert_exists(DOCS_BROWSER_TRUTH_PACKET_ARTIFACT_DOC_REF);
    assert_exists(DOCS_BROWSER_TRUTH_PACKET_FIXTURE_DIR);
    assert_exists(DOCS_BROWSER_TRUTH_PACKET_ARTIFACT_REF);
    assert_exists(DOCS_BROWSER_TRUTH_PACKET_SCHEMA_REF);
}

#[test]
fn baseline_stable_fixture_certifies_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn missing_required_source_class_fixture_blocks_stable() {
    assert_fixture_matches("missing_required_source_class_blocks_stable.json");
}

#[test]
fn symbol_flow_drops_split_step_fixture_blocks_stable() {
    assert_fixture_matches("symbol_flow_drops_split_step_blocks_stable.json");
}

#[test]
fn result_source_ref_unpinned_fixture_blocks_stable() {
    assert_fixture_matches("result_source_ref_unpinned_blocks_stable.json");
}

#[test]
fn consumer_projection_drops_source_class_fixture_blocks_stable() {
    assert_fixture_matches("consumer_projection_drops_source_class_blocks_stable.json");
}

#[test]
fn live_external_handoff_missing_packet_fixture_blocks_stable() {
    assert_fixture_matches("live_external_handoff_missing_packet_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_required_surfaces() {
    let packet = current_stable_docs_browser_truth_packet()
        .expect("checked-in docs-browser truth packet validates");
    assert_eq!(packet.promotion_state, DocsBrowserPromotionState::Stable);
    assert!(packet.validate().is_empty());

    for surface in DocsBrowserConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }

    let source_class_tokens: BTreeSet<&str> = packet.source_class_tokens().into_iter().collect();
    for required in DocsBrowserSourceClass::REQUIRED {
        assert!(
            source_class_tokens.contains(required.as_str()),
            "checked-in packet must cover source class {}",
            required.as_str()
        );
    }
}

#[test]
fn artifact_file_matches_seeded_packet() {
    let path = repo_root().join(DOCS_BROWSER_TRUTH_PACKET_ARTIFACT_REF);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("artifact {path:?} must read: {err}"));
    let from_file: DocsBrowserTruthPacket = serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("artifact {path:?} must parse: {err}"));
    let from_seed =
        DocsBrowserTruthPacket::materialize(seeded_stable_docs_browser_truth_packet_input());
    assert_eq!(
        from_file, from_seed,
        "checked-in docs-browser truth packet drifted from the in-code seed; \
         regenerate with `cargo run -q -p aureline-docs --bin aureline_docs_browser_truth_packet -- packet > artifacts/search/m4/docs_browser_truth_packet.json`",
    );
}

#[test]
fn closed_docs_browser_tokens_are_pinned() {
    assert_eq!(DocsBrowserSourceClass::ProjectDocs.as_str(), "project_docs");
    assert_eq!(
        DocsBrowserSourceClass::LiveExternalDocs.as_str(),
        "live_external_docs"
    );
    assert_eq!(
        DocsBrowserVersionMatchState::IncompatibleDriftDetected.as_str(),
        "incompatible_drift_detected"
    );
    assert_eq!(
        DocsBrowserFreshnessState::DegradedCached.as_str(),
        "degraded_cached"
    );
    assert_eq!(
        DocsBrowserFindingKind::SymbolFlowIdentityLost.as_str(),
        "symbol_flow_identity_lost"
    );
    assert_eq!(
        DocsBrowserConsumerSurface::BrowserHandoffPacket.as_str(),
        "browser_handoff_packet"
    );
}
