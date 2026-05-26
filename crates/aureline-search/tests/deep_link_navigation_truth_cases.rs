//! Fixture-driven coverage for the stable deep-link remap and
//! navigation-continuity truth packet.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_search::{
    current_stable_deep_link_navigation_truth_packet, DeepLinkNavigationTruthConsumerSurface,
    DeepLinkNavigationTruthFindingKind, DeepLinkNavigationTruthPacket,
    DeepLinkNavigationTruthPacketInput, DeepLinkNavigationTruthPromotionState,
    DEEP_LINK_NAVIGATION_TRUTH_ARTIFACT_DOC_REF, DEEP_LINK_NAVIGATION_TRUTH_DOC_REF,
    DEEP_LINK_NAVIGATION_TRUTH_FIXTURE_DIR, DEEP_LINK_NAVIGATION_TRUTH_PACKET_ARTIFACT_REF,
    DEEP_LINK_NAVIGATION_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CaseFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: DeepLinkNavigationTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    outcome_tokens: Vec<String>,
    drift_state_tokens: Vec<String>,
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
        .join(DEEP_LINK_NAVIGATION_TRUTH_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "deep_link_navigation_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = DeepLinkNavigationTruthPacket::materialize(fixture.input.clone());
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
        packet.outcome_tokens(),
        expect
            .outcome_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.drift_state_tokens(),
        expect
            .drift_state_tokens
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
    assert_exists(DEEP_LINK_NAVIGATION_TRUTH_SCHEMA_REF);
    assert_exists(DEEP_LINK_NAVIGATION_TRUTH_DOC_REF);
    assert_exists(DEEP_LINK_NAVIGATION_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(DEEP_LINK_NAVIGATION_TRUTH_FIXTURE_DIR);
    assert_exists(DEEP_LINK_NAVIGATION_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_fixture_certifies_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn continuity_mismatch_blocks_stable() {
    assert_fixture_matches("continuity_mismatch_blocks_stable.json");
}

#[test]
fn missing_projection_blocks_stable() {
    assert_fixture_matches("missing_projection_blocks_stable.json");
}

#[test]
fn projection_drops_drift_state_blocks_stable() {
    assert_fixture_matches("projection_drops_drift_state_blocks_stable.json");
}

#[test]
fn outcome_coverage_over_declared_blocks_stable() {
    assert_fixture_matches("outcome_coverage_over_declared_blocks_stable.json");
}

#[test]
fn destination_visibility_drift_blocks_stable() {
    assert_fixture_matches("destination_visibility_drift_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_outcome() {
    let packet = current_stable_deep_link_navigation_truth_packet()
        .expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        DeepLinkNavigationTruthPromotionState::Stable
    );
    assert!(packet.validate().is_empty());

    let outcome_tokens: BTreeSet<&str> = packet.outcome_tokens().into_iter().collect();
    for token in [
        "resolved_exact",
        "remapped",
        "recoverable_placeholder",
        "failed_explicit_reason",
    ] {
        assert!(
            outcome_tokens.contains(token),
            "checked-in packet must cover outcome {token}; observed {:?}",
            outcome_tokens
        );
    }

    for surface in DeepLinkNavigationTruthConsumerSurface::REQUIRED {
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
        DeepLinkNavigationTruthFindingKind::ContinuityRemapPacketMismatch.as_str(),
        "continuity_remap_packet_mismatch"
    );
    assert_eq!(
        DeepLinkNavigationTruthFindingKind::DestinationVisibilityDropped.as_str(),
        "destination_visibility_dropped"
    );
    assert_eq!(
        DeepLinkNavigationTruthFindingKind::DriftStateCoverageOverDeclared.as_str(),
        "drift_state_coverage_over_declared"
    );
    assert_eq!(
        DeepLinkNavigationTruthFindingKind::MissingConsumerProjection.as_str(),
        "missing_consumer_projection"
    );
    assert_eq!(
        DeepLinkNavigationTruthFindingKind::ProjectionDriftStateDropped.as_str(),
        "projection_drift_state_dropped"
    );
}
