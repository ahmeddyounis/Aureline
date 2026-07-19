//! Integration test: the embedded M05-800 ownership/explainer component packet
//! parses, validates, keeps distinct ownership roles separate, carries concrete
//! explainer citations, and narrows generated summaries when their supporting
//! truth is incomplete.

use aureline_graph::{
    current_m5_ownership_explainer_component_packet, CitationState, ComponentConsumerSurface,
    ExplainerSectionCitationKind, RoleType, SummaryGenerationMode,
};

#[test]
fn embedded_ownership_explainer_component_packet_parses() {
    let packet =
        current_m5_ownership_explainer_component_packet().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.ownership_cards.is_empty());
    assert!(!packet.explainer_section_cards.is_empty());
    assert!(!packet.consumer_projection_rows.is_empty());
}

#[test]
fn embedded_ownership_explainer_component_packet_has_no_violations() {
    let packet =
        current_m5_ownership_explainer_component_packet().expect("embedded packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_ownership_explainer_summary_matches_computed() {
    let packet =
        current_m5_ownership_explainer_component_packet().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
    assert!(packet.summary.ownership_consumer_present);
    assert!(packet.summary.explainer_consumer_present);
    assert!(packet.summary.all_ownership_cards_distinguish_roles);
    assert!(packet.summary.all_explainer_cards_cite_sources);
    assert!(packet.summary.generated_summaries_narrow_when_incomplete);
    assert!(packet.summary.all_components_have_copy_export);
}

#[test]
fn ownership_cards_keep_distinct_roles_separate() {
    let packet =
        current_m5_ownership_explainer_component_packet().expect("embedded packet must parse");

    // AC1: no card collapses distinct responsibilities into one ambiguous owner.
    for card in &packet.ownership_cards {
        assert!(
            card.distinguishes_roles(),
            "card {} collapsed roles",
            card.card_id
        );
        assert!(card.service_oncall_separation);
        // Protected-path / change-control links are attached and survive export.
        assert!(!card.escalation_refs.is_empty());
        assert!(card.preserves_truth_in_export());
    }

    // The primary auth card separates service_owner, oncall, reviewer, maintainer.
    let auth = packet
        .ownership_cards
        .iter()
        .find(|c| c.card_id == "ownership-card:team-auth")
        .expect("auth ownership card must be present");
    let roles: Vec<RoleType> = auth.role_assignments.iter().map(|r| r.role_type).collect();
    assert!(roles.contains(&RoleType::ServiceOwner));
    assert!(roles.contains(&RoleType::Oncall));
    assert!(roles.contains(&RoleType::Reviewer));
    assert!(roles.contains(&RoleType::Maintainer));
    assert!(auth.distinct_role_count() >= 4);
}

#[test]
fn explainer_cards_cite_sources_and_narrow_when_incomplete() {
    let packet =
        current_m5_ownership_explainer_component_packet().expect("embedded packet must parse");

    // AC2: every explainer card carries concrete citations, spans the architecture
    // explainer plus a secondary consumer, and preserves provenance across export.
    for card in &packet.explainer_section_cards {
        assert!(card.is_cited(), "card {} is uncited", card.card_id);
        assert!(card.preserves_truth_in_export());
        assert!(card
            .consumer_surfaces
            .contains(&ComponentConsumerSurface::ArchitectureExplainer));
        // A generated summary is never presented as uncited primary truth.
        if card.is_generated() {
            assert!(!card.generated_but_not_narrowed());
        }
    }

    // A well-supported generated_reviewed summary keeps full capability.
    let fresh = packet
        .explainer_section_cards
        .iter()
        .find(|c| c.card_id == "explainer-section:auth-login-flow")
        .expect("fresh explainer card must be present");
    assert_eq!(
        fresh.summary_generation_mode,
        SummaryGenerationMode::GeneratedReviewed
    );
    assert_eq!(fresh.citation_state, CitationState::Complete);
    assert!(!fresh.is_narrowed());
    assert!(fresh
        .citation_refs
        .iter()
        .any(|c| c.citation_kind == ExplainerSectionCitationKind::File));

    // AC3: a generated summary on partial citations + stale freshness narrows.
    let narrowed = packet
        .explainer_section_cards
        .iter()
        .find(|c| c.card_id == "explainer-section:billing-webhook-flow")
        .expect("narrowed explainer card must be present");
    assert_eq!(
        narrowed.summary_generation_mode,
        SummaryGenerationMode::Generated
    );
    assert!(narrowed.truth_incomplete());
    assert!(narrowed.is_narrowed());
    assert!(!narrowed.generated_but_not_narrowed());
}
