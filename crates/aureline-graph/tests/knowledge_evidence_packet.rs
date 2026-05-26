//! Fixture-driven coverage for the stable knowledge-plane evidence
//! packet (shared identity model, topology view, impact cards,
//! ownership cards, architecture explainer snapshots, and consumer
//! projections).

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_graph::{
    current_stable_knowledge_evidence_packet, ExplainerSourceClass, KnowledgeConsumerSurface,
    KnowledgeEvidencePacket, KnowledgeEvidencePacketInput, KnowledgeFindingKind,
    KnowledgePromotionState, NoImpactState, OwnershipClass,
    KNOWLEDGE_EVIDENCE_PACKET_ARTIFACT_DOC_REF, KNOWLEDGE_EVIDENCE_PACKET_ARTIFACT_REF,
    KNOWLEDGE_EVIDENCE_PACKET_DOC_REF, KNOWLEDGE_EVIDENCE_PACKET_FIXTURE_DIR,
    KNOWLEDGE_EVIDENCE_PACKET_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct KnowledgeFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: KnowledgeEvidencePacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    impact_card_count: usize,
    ownership_card_count: usize,
    explainer_count: usize,
    ownership_class_tokens: Vec<String>,
    no_impact_state_tokens: Vec<String>,
    explainer_source_class_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> KnowledgeFixture {
    let path = repo_root()
        .join(KNOWLEDGE_EVIDENCE_PACKET_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "graph_knowledge_evidence_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = KnowledgeEvidencePacket::materialize(fixture.input.clone());
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
        packet.impact_cards.len(),
        expect.impact_card_count,
        "fixture {} impact card count drift",
        fixture.case_name
    );
    assert_eq!(
        packet.ownership_cards.len(),
        expect.ownership_card_count,
        "fixture {} ownership card count drift",
        fixture.case_name
    );
    assert_eq!(
        packet.explainer_snapshots.len(),
        expect.explainer_count,
        "fixture {} explainer count drift",
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

    let observed_ownership: BTreeSet<&str> = packet.ownership_class_tokens().into_iter().collect();
    let expected_ownership: BTreeSet<&str> = expect
        .ownership_class_tokens
        .iter()
        .map(String::as_str)
        .collect();
    assert_eq!(
        observed_ownership, expected_ownership,
        "fixture {} ownership-class tokens drift",
        fixture.case_name
    );

    let observed_no_impact: BTreeSet<&str> = packet.no_impact_state_tokens().into_iter().collect();
    let expected_no_impact: BTreeSet<&str> = expect
        .no_impact_state_tokens
        .iter()
        .map(String::as_str)
        .collect();
    assert_eq!(
        observed_no_impact, expected_no_impact,
        "fixture {} no-impact tokens drift",
        fixture.case_name
    );

    let observed_explainer: BTreeSet<&str> =
        packet.explainer_source_class_tokens().into_iter().collect();
    let expected_explainer: BTreeSet<&str> = expect
        .explainer_source_class_tokens
        .iter()
        .map(String::as_str)
        .collect();
    assert_eq!(
        observed_explainer, expected_explainer,
        "fixture {} explainer source-class tokens drift",
        fixture.case_name
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
fn doc_fixture_schema_and_artifact_exist_on_disk() {
    assert_exists(KNOWLEDGE_EVIDENCE_PACKET_DOC_REF);
    assert_exists(KNOWLEDGE_EVIDENCE_PACKET_ARTIFACT_DOC_REF);
    assert_exists(KNOWLEDGE_EVIDENCE_PACKET_FIXTURE_DIR);
    assert_exists(KNOWLEDGE_EVIDENCE_PACKET_ARTIFACT_REF);
    assert_exists(KNOWLEDGE_EVIDENCE_PACKET_SCHEMA_REF);
}

#[test]
fn baseline_stable_fixture_certifies_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn no_impact_collapsed_on_out_of_scope_fixture_blocks_stable() {
    assert_fixture_matches("no_impact_collapsed_on_out_of_scope_blocks_stable.json");
}

#[test]
fn ownership_partiality_note_missing_fixture_blocks_stable() {
    assert_fixture_matches("ownership_partiality_note_missing_blocks_stable.json");
}

#[test]
fn generated_explainer_missing_citations_or_inference_fixture_blocks_stable() {
    assert_fixture_matches("generated_explainer_missing_citations_or_inference_blocks_stable.json");
}

#[test]
fn consumer_projection_drops_node_edge_identity_fixture_blocks_stable() {
    assert_fixture_matches("consumer_projection_drops_node_edge_identity_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_required_surfaces() {
    let packet = current_stable_knowledge_evidence_packet().expect("checked-in packet validates");
    assert_eq!(packet.promotion_state, KnowledgePromotionState::Stable);
    assert!(packet.validate().is_empty());

    for surface in KnowledgeConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }

    // Canonical packet must bind every closed ownership class.
    let ownership_tokens: BTreeSet<&str> = packet.ownership_class_tokens().into_iter().collect();
    for required in OwnershipClass::ALL {
        assert!(
            ownership_tokens.contains(required.as_str()),
            "checked-in packet must cover ownership class {}",
            required.as_str()
        );
    }

    // Canonical packet must bind every closed no-impact state.
    let no_impact_tokens: BTreeSet<&str> = packet.no_impact_state_tokens().into_iter().collect();
    for required in [
        NoImpactState::VisibleImpactPresent,
        NoImpactState::NoImpactInWorkspaceOrSlice,
        NoImpactState::ImpactOutsideSliceDisclosed,
    ] {
        assert!(
            no_impact_tokens.contains(required.as_str()),
            "checked-in packet must cover no-impact state {}",
            required.as_str()
        );
    }

    // Canonical packet must cover both explainer source classes.
    let explainer_tokens: BTreeSet<&str> =
        packet.explainer_source_class_tokens().into_iter().collect();
    for required in [
        ExplainerSourceClass::Curated,
        ExplainerSourceClass::Generated,
    ] {
        assert!(
            explainer_tokens.contains(required.as_str()),
            "checked-in packet must cover explainer source class {}",
            required.as_str()
        );
    }
}

#[test]
fn closed_knowledge_tokens_are_pinned() {
    assert_eq!(
        OwnershipClass::CuratedFirstParty.as_str(),
        "curated_first_party"
    );
    assert_eq!(
        NoImpactState::ImpactOutsideSliceDisclosed.as_str(),
        "impact_outside_slice_disclosed"
    );
    assert_eq!(ExplainerSourceClass::Generated.as_str(), "generated");
    assert_eq!(
        KnowledgeFindingKind::NoImpactCollapsedOnOutOfScopeImpact.as_str(),
        "no_impact_collapsed_on_out_of_scope_impact"
    );
    assert_eq!(
        KnowledgeFindingKind::OwnershipPartialityNoteMissing.as_str(),
        "ownership_partiality_note_missing"
    );
    assert_eq!(
        KnowledgeFindingKind::GeneratedExplainerMissingCitationsOrInference.as_str(),
        "generated_explainer_missing_citations_or_inference"
    );
    assert_eq!(
        KnowledgeConsumerSurface::ReviewExplainerCard.as_str(),
        "review_explainer_card"
    );
}
