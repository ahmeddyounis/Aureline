//! Inline unit coverage for the derived-explanation citation-sets packet.

use super::*;

fn stable_packet() -> DerivedExplanationCitationPacket {
    DerivedExplanationCitationPacket::materialize(seeded_stable_derived_explanation_citation_input())
}

#[test]
fn seeded_packet_is_clean_stable() {
    let packet = stable_packet();
    assert_eq!(packet.record_kind, DERIVED_EXPLANATION_CITATION_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        DERIVED_EXPLANATION_CITATION_SCHEMA_VERSION
    );
    assert_eq!(
        packet.promotion_state,
        DerivedExplanationCitationPromotionState::Stable
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.is_clean_stable());
    assert!(packet.is_stable());
}

#[test]
fn seeded_packet_binds_a_citation_set_to_every_required_surface() {
    let packet = stable_packet();
    let covered = packet.covered_surfaces();
    for surface in DerivedExplanationSurface::REQUIRED {
        assert!(
            covered.contains(&surface),
            "missing citation set for surface {}",
            surface.as_str()
        );
        assert!(
            packet.has_projection_for(surface),
            "missing projection for surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn the_six_surface_tokens_are_pinned() {
    let expected = [
        "docs_browser_explanation",
        "ai_answer",
        "glossary_card",
        "guided_tour_step",
        "architecture_explainer",
        "support_export_note",
    ];
    let observed: Vec<&str> = DerivedExplanationSurface::ALL
        .iter()
        .map(|surface| surface.as_str())
        .collect();
    assert_eq!(observed, expected);
}

#[test]
fn every_set_either_cites_evidence_or_labels_inference() {
    let packet = stable_packet();
    for set in &packet.citation_sets {
        match set.basis {
            CitationBasis::DirectCitation => {
                assert!(
                    set.has_any_citation(),
                    "direct citation {} must name evidence",
                    set.citation_set_id
                );
                assert!(set.inference_label.is_none());
            }
            CitationBasis::LabeledInference => {
                assert!(!set.has_any_citation());
                assert!(
                    set.inference_label.is_some(),
                    "labeled inference {} must carry a label",
                    set.citation_set_id
                );
                assert_eq!(
                    set.trust_class,
                    DocsContractTrustClass::DerivedInferenceOnly
                );
            }
        }
        assert!(set.basis_consistent());
        assert!(set.trust_consistent());
    }
}

#[test]
fn citation_basis_survives_redaction() {
    let packet = stable_packet();
    let redacted = packet
        .citation_sets
        .iter()
        .find(|set| set.redaction.withholds_content())
        .expect("seed carries a redacted support-export note");
    assert!(redacted.basis_preserved_through_redaction());
    assert!(redacted.has_any_citation() || redacted.inference_label.is_some());
}

#[test]
fn support_export_preserves_every_citation_basis() {
    let packet = stable_packet();
    let projection = packet
        .consumer_projections
        .iter()
        .find(|projection| projection.surface == DerivedExplanationSurface::SupportExportNote)
        .expect("seed carries a support-export projection");
    for set in &packet.citation_sets {
        assert!(
            projection
                .citation_set_id_refs
                .contains(&set.citation_set_id),
            "support export drops citation set {}",
            set.citation_set_id
        );
    }
}

#[test]
fn direct_citation_without_evidence_blocks() {
    let mut input = seeded_stable_derived_explanation_citation_input();
    if let Some(set) = input
        .citation_sets
        .iter_mut()
        .find(|set| set.explanation_surface == DerivedExplanationSurface::AiAnswer)
    {
        set.cited_files.clear();
        set.cited_symbols.clear();
        set.cited_docs.clear();
    }
    let packet = DerivedExplanationCitationPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DerivedExplanationCitationPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind
            == DerivedExplanationCitationValidationKind::CitationBasisMissing));
}

#[test]
fn inference_claiming_authority_blocks() {
    let mut input = seeded_stable_derived_explanation_citation_input();
    if let Some(set) = input
        .citation_sets
        .iter_mut()
        .find(|set| set.explanation_surface == DerivedExplanationSurface::ArchitectureExplainer)
    {
        set.trust_class = DocsContractTrustClass::FirstPartyAuthoritative;
    }
    let packet = DerivedExplanationCitationPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DerivedExplanationCitationPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind
            == DerivedExplanationCitationValidationKind::BasisTrustInconsistent));
}

#[test]
fn stale_direct_citation_narrows_without_blocking() {
    let mut input = seeded_stable_derived_explanation_citation_input();
    if let Some(set) = input
        .citation_sets
        .iter_mut()
        .find(|set| set.explanation_surface == DerivedExplanationSurface::DocsBrowserExplanation)
    {
        set.freshness = DocsContractFreshnessState::Stale;
    }
    let packet = DerivedExplanationCitationPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        DerivedExplanationCitationPromotionState::NarrowedBelowStable
    );
    assert!(packet.is_stable(), "narrowing must not block");
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind
            == DerivedExplanationCitationValidationKind::CitationFreshnessNarrowed));
}

#[test]
fn promotion_state_mismatch_is_detected() {
    let mut packet = stable_packet();
    packet.promotion_state = DerivedExplanationCitationPromotionState::BlocksStable;
    assert!(packet.validate().iter().any(|finding| finding.finding_kind
        == DerivedExplanationCitationValidationKind::PromotionStateMismatch));
}

#[test]
fn support_export_round_trips_and_is_export_safe() {
    let packet = stable_packet();
    let export = packet.support_export("export:test", "2026-06-26T00:00:00Z");
    assert!(export.is_export_safe());
    let json = serde_json::to_string(&export).expect("export serializes");
    let parsed: DerivedExplanationCitationSupportExport =
        serde_json::from_str(&json).expect("export round-trips");
    assert_eq!(parsed, export);
}

#[test]
fn checked_in_packet_validates() {
    let packet = current_stable_derived_explanation_citation_packet()
        .expect("seeded packet certifies stable");
    assert!(packet.validate().is_empty());
}
