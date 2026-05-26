//! Fixture-driven coverage for the stable public-proof publication truth packet
//! covering search, graph, and docs surfaces with known limits and downgrade
//! automation.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_graph::{
    current_stable_public_proof_publication_truth_packet, KnownLimitClass, ProofArtifactClass,
    PublicProofDowngradeAutomationClass, PublicProofPublicationConsumerSurface,
    PublicProofPublicationFindingKind, PublicProofPublicationPromotionState,
    PublicProofPublicationTruthPacket, PublicProofPublicationTruthPacketInput,
    PublicationLaneClass, PublicationRowClass, PublicationState,
    PUBLIC_PROOF_PUBLICATION_TRUTH_ARTIFACT_DOC_REF, PUBLIC_PROOF_PUBLICATION_TRUTH_DOC_REF,
    PUBLIC_PROOF_PUBLICATION_TRUTH_FIXTURE_DIR, PUBLIC_PROOF_PUBLICATION_TRUTH_PACKET_ARTIFACT_REF,
    PUBLIC_PROOF_PUBLICATION_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PublicProofPublicationFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: PublicProofPublicationTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    lane_tokens: Vec<String>,
    row_class_tokens: Vec<String>,
    publication_state_tokens: Vec<String>,
    known_limit_tokens: Vec<String>,
    downgrade_automation_tokens: Vec<String>,
    proof_artifact_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> PublicProofPublicationFixture {
    let path = repo_root()
        .join(PUBLIC_PROOF_PUBLICATION_TRUTH_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
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
        fixture.record_kind, "public_proof_publication_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = PublicProofPublicationTruthPacket::materialize(fixture.input.clone());
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
    assert_token_set_matches(&packet.lane_tokens(), &expect.lane_tokens, "lane");
    assert_token_set_matches(
        &packet.row_class_tokens(),
        &expect.row_class_tokens,
        "row_class",
    );
    assert_token_set_matches(
        &packet.publication_state_tokens(),
        &expect.publication_state_tokens,
        "publication_state",
    );
    assert_token_set_matches(
        &packet.known_limit_tokens(),
        &expect.known_limit_tokens,
        "known_limit",
    );
    assert_token_set_matches(
        &packet.downgrade_automation_tokens(),
        &expect.downgrade_automation_tokens,
        "downgrade_automation",
    );
    assert_token_set_matches(
        &packet.proof_artifact_tokens(),
        &expect.proof_artifact_tokens,
        "proof_artifact",
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
    assert_exists(PUBLIC_PROOF_PUBLICATION_TRUTH_SCHEMA_REF);
    assert_exists(PUBLIC_PROOF_PUBLICATION_TRUTH_DOC_REF);
    assert_exists(PUBLIC_PROOF_PUBLICATION_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(PUBLIC_PROOF_PUBLICATION_TRUTH_FIXTURE_DIR);
    assert_exists(PUBLIC_PROOF_PUBLICATION_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn published_stable_with_unbound_limit_blocks_stable() {
    assert_fixture_matches("published_stable_with_unbound_limit_blocks_stable.json");
}

#[test]
fn published_stable_with_unbound_automation_blocks_stable() {
    assert_fixture_matches("published_stable_with_unbound_automation_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_downgrade_automation_blocks_stable() {
    assert_fixture_matches("projection_collapses_downgrade_automation_blocks_stable.json");
}

#[test]
fn raw_query_material_blocks_stable() {
    assert_fixture_matches("raw_query_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet = current_stable_public_proof_publication_truth_packet()
        .expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        PublicProofPublicationPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in PublicationLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for publication lane {}",
            required.as_str()
        );
    }
    for surface in PublicProofPublicationConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn closed_public_proof_publication_tokens_are_pinned() {
    assert_eq!(
        PublicationLaneClass::DocsPublicProofLane.as_str(),
        "docs_public_proof_lane"
    );
    assert_eq!(
        PublicationRowClass::DowngradeAutomation.as_str(),
        "downgrade_automation"
    );
    assert_eq!(
        PublicationState::WithheldPendingGap.as_str(),
        "withheld_pending_gap"
    );
    assert_eq!(KnownLimitClass::LimitUnbound.as_str(), "limit_unbound");
    assert_eq!(
        PublicProofDowngradeAutomationClass::AutomationUnbound.as_str(),
        "automation_unbound"
    );
    assert_eq!(
        ProofArtifactClass::DashboardPublishedTruthRow.as_str(),
        "dashboard_published_truth_row"
    );
    assert_eq!(
        PublicProofPublicationConsumerSurface::DashboardPublishedTruth.as_str(),
        "dashboard_published_truth"
    );
    assert_eq!(
        PublicProofPublicationFindingKind::DowngradeAutomationVocabularyCollapsed.as_str(),
        "downgrade_automation_vocabulary_collapsed"
    );
}
