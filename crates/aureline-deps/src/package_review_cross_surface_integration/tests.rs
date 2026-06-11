use super::*;

fn packet() -> PackageReviewCrossSurfaceIntegration {
    current_package_review_cross_surface_integration().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        PACKAGE_REVIEW_CROSS_SURFACE_INTEGRATION_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        PACKAGE_REVIEW_CROSS_SURFACE_INTEGRATION_RECORD_KIND
    );
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn export_projection_includes_all_cards() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.cards.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_authority_within_bounds,
        packet.all_authority_within_bounds()
    );
}

#[test]
fn surfaces_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<SurfaceClass> = packet.cards.iter().map(|c| c.surface).collect();
    for surface in SurfaceClass::ALL {
        assert!(
            present.contains(&surface),
            "missing surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn write_authorities_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<WriteAuthority> =
        packet.cards.iter().map(|c| c.write_authority).collect();
    for authority in WriteAuthority::ALL {
        assert!(
            present.contains(&authority),
            "missing authority {}",
            authority.as_str()
        );
    }
}

#[test]
fn source_labels_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<SourceLabel> = packet.cards.iter().map(|c| c.source_label).collect();
    for label in SourceLabel::ALL {
        assert!(
            present.contains(&label),
            "missing source label {}",
            label.as_str()
        );
    }
}

#[test]
fn finding_truths_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<FindingTruth> = packet.cards.iter().map(|c| c.finding_truth).collect();
    for truth in FindingTruth::ALL {
        assert!(
            present.contains(&truth),
            "missing finding truth {}",
            truth.as_str()
        );
    }
}

#[test]
fn transition_kinds_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<TransitionKind> = packet.handoffs.iter().map(|h| h.transition).collect();
    for transition in TransitionKind::ALL {
        assert!(
            present.contains(&transition),
            "missing transition {}",
            transition.as_str()
        );
    }
}

#[test]
fn only_desktop_carries_full_mutation() {
    let packet = packet();
    for card in &packet.cards {
        if card.write_authority.permits_mutation() {
            assert_eq!(
                card.surface,
                SurfaceClass::DesktopWorkspace,
                "non-desktop card {} carries full mutation",
                card.card_id
            );
        }
    }
}

#[test]
fn companion_and_browser_cards_are_inspect_only() {
    let packet = packet();
    for card in &packet.cards {
        if card.surface.is_companion_or_browser() {
            assert!(
                card.write_authority.is_inspect_only(),
                "companion/browser card {} is not inspect-only",
                card.card_id
            );
            assert!(
                card.inspect_ref.is_some(),
                "companion/browser card {} omits inspect_ref",
                card.card_id
            );
        }
    }
}

#[test]
fn handoffs_resolve_and_preserve_truth() {
    let packet = packet();
    assert!(
        !packet.handoffs.is_empty(),
        "fixture must exercise handoffs"
    );
    for handoff in &packet.handoffs {
        assert!(
            packet.handoff_preserves_truth(handoff),
            "handoff {} drops truth",
            handoff.handoff_id
        );
    }
}

#[test]
fn live_finding_requires_local_current_source() {
    assert!(FindingTruth::Live.permitted_for(SourceLabel::Local, AdvisoryFreshness::Current));
    assert!(!FindingTruth::Live.permitted_for(SourceLabel::Imported, AdvisoryFreshness::Current));
    assert!(!FindingTruth::Live.permitted_for(SourceLabel::Local, AdvisoryFreshness::Stale));
    assert!(FindingTruth::Imported.permitted_for(SourceLabel::Mirrored, AdvisoryFreshness::Unknown));
}

#[test]
fn validate_flags_write_authority_overreach() {
    let mut packet = packet();
    if let Some(card) = packet
        .cards
        .iter_mut()
        .find(|c| c.surface.is_companion_or_browser())
    {
        card.write_authority = WriteAuthority::FullMutation;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            PackageReviewCrossSurfaceIntegrationViolation::WriteAuthorityOverreach { .. }
        )));
    }
}

#[test]
fn validate_flags_overstated_finding_truth() {
    let mut packet = packet();
    if let Some(card) = packet
        .cards
        .iter_mut()
        .find(|c| c.finding_truth == FindingTruth::Imported)
    {
        card.finding_truth = FindingTruth::Live;
        card.source_label = SourceLabel::Imported;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            PackageReviewCrossSurfaceIntegrationViolation::OverstatedFindingTruth { .. }
        )));
    }
}

#[test]
fn validate_flags_mutation_state_without_authority() {
    let mut packet = packet();
    if let Some(card) = packet
        .cards
        .iter_mut()
        .find(|c| c.write_authority.is_inspect_only())
    {
        card.review_state = ReviewState::Applied;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            PackageReviewCrossSurfaceIntegrationViolation::MutationStateWithoutAuthority { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_inspect_ref() {
    let mut packet = packet();
    if let Some(card) = packet
        .cards
        .iter_mut()
        .find(|c| c.surface.is_companion_or_browser())
    {
        card.inspect_ref = None;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            PackageReviewCrossSurfaceIntegrationViolation::MissingInspectRef { .. }
        )));
    }
}

#[test]
fn validate_flags_transition_surface_mismatch() {
    let mut packet = packet();
    if let Some(handoff) = packet
        .handoffs
        .iter_mut()
        .find(|h| h.transition == TransitionKind::BrowserHandoff)
    {
        handoff.to_surface = SurfaceClass::DesktopWorkspace;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            PackageReviewCrossSurfaceIntegrationViolation::TransitionSurfaceMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_handoff_drops_truth() {
    let mut packet = packet();
    if let Some(handoff) = packet.handoffs.first_mut() {
        handoff.review_state = match handoff.review_state {
            ReviewState::Applied => ReviewState::NotStarted,
            _ => ReviewState::Applied,
        };
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            PackageReviewCrossSurfaceIntegrationViolation::HandoffDropsTruth { .. }
        )));
    }
}

#[test]
fn validate_flags_handoff_unknown_card() {
    let mut packet = packet();
    if let Some(handoff) = packet.handoffs.first_mut() {
        handoff.card_id = "card:does-not-exist".to_owned();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            PackageReviewCrossSurfaceIntegrationViolation::HandoffUnknownCard { .. }
        )));
    }
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_cards = packet.summary.total_cards.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&PackageReviewCrossSurfaceIntegrationViolation::SummaryMismatch));
}

#[test]
fn surface_tokens_are_stable() {
    assert_eq!(SurfaceClass::DesktopWorkspace.as_str(), "desktop_workspace");
    assert_eq!(
        SurfaceClass::FrameworkPackHealth.as_str(),
        "framework_pack_health"
    );
    assert_eq!(SurfaceClass::ReviewWorkspace.as_str(), "review_workspace");
    assert_eq!(SurfaceClass::IncidentBundle.as_str(), "incident_bundle");
    assert_eq!(SurfaceClass::CompanionInspect.as_str(), "companion_inspect");
    assert_eq!(SurfaceClass::BrowserHandoff.as_str(), "browser_handoff");
}

#[test]
fn surface_max_authority_holds_invariants() {
    assert!(SurfaceClass::DesktopWorkspace
        .max_write_authority()
        .permits_mutation());
    assert!(!SurfaceClass::ReviewWorkspace
        .max_write_authority()
        .permits_mutation());
    for surface in [
        SurfaceClass::FrameworkPackHealth,
        SurfaceClass::IncidentBundle,
        SurfaceClass::CompanionInspect,
        SurfaceClass::BrowserHandoff,
    ] {
        assert!(
            surface.max_write_authority().is_inspect_only(),
            "{} must be inspect-only",
            surface.as_str()
        );
    }
}
