//! Tests for the frozen commercial-boundary-card set.

use super::*;

fn set() -> BoundaryCardSet {
    canonical_stable_commercial_boundary_card_set()
}

#[test]
fn canonical_set_validates_clean() {
    let s = set();
    let violations = s.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn checked_in_set_matches_canonical_builder() {
    let stable =
        current_stable_commercial_boundary_card_set().expect("checked-in set parses and validates");
    assert_eq!(
        stable,
        set(),
        "the checked-in artifact drifted from the canonical builder; regenerate it with the dump example"
    );
}

#[test]
fn set_covers_one_open_card_and_every_managed_lane() {
    let s = set();
    assert_eq!(s.inspection.local_open_card_count, 1);
    assert!(s.inspection.managed_lane_coverage_complete);
    assert_eq!(
        s.inspection.managed_paid_card_count,
        ServiceFamily::ALL.len()
    );
    assert_eq!(
        s.inspection.service_families_covered,
        ServiceFamily::ALL.len()
    );
    assert!(s.local_open_card().is_some());
    for family in ServiceFamily::ALL {
        assert!(
            s.card_for_family(family).is_some(),
            "missing card for {family:?}"
        );
    }
}

#[test]
fn every_card_keeps_a_local_safe_baseline() {
    let s = set();
    assert!(s.inspection.all_cards_local_safe_backed);
    for c in &s.cards {
        assert!(
            !c.local_safe_baseline.is_empty(),
            "card {} lost its baseline",
            c.card_id
        );
        assert!(c.local_safe_baseline.iter().all(|p| !p.trim().is_empty()));
    }
}

#[test]
fn the_open_core_makes_no_managed_claim_and_no_residual_dependency() {
    let s = set();
    let open = s.local_open_card().expect("open core card present");
    assert_eq!(open.boundary_class, BoundaryClass::LocalOpenSource);
    assert_eq!(open.service_family, None);
    assert!(open.residual_dependencies.is_empty());
    assert_eq!(open.declared_marketed_claim, MarketedClaim::LocalSafeOnly);
    assert_eq!(open.effective_marketed_claim, MarketedClaim::LocalSafeOnly);
    // The open core never upsells itself.
    assert!(!open
        .actions
        .iter()
        .any(|a| a.kind == BoundaryActionKind::LearnAboutPaid));
    // It holds in every profile, including air-gapped.
    assert!(open.holds_in(DeploymentProfile::AirGapped));
    assert!(open.holds_in(DeploymentProfile::IndividualLocal));
}

#[test]
fn every_managed_card_discloses_a_residual_dependency_honestly() {
    let s = set();
    assert!(s.inspection.all_managed_cards_disclose_residual);
    for c in s
        .cards
        .iter()
        .filter(|c| c.boundary_class.is_managed_paid())
    {
        assert!(
            !c.residual_dependencies.is_empty(),
            "managed card {} hides residual dependencies",
            c.card_id
        );
        // Every residual dependency states its vendor-hosted / self-host honesty.
        for dep in &c.residual_dependencies {
            assert!(
                dep.remains_vendor_hosted || dep.eliminated_under_self_host,
                "card {} residual dependency {:?} discloses nothing",
                c.card_id,
                dep.dependency_class
            );
            assert!(!dep.disclosure.trim().is_empty());
        }
    }
}

#[test]
fn no_card_overstates_the_open_boundary() {
    let s = set();
    assert!(s.inspection.all_cards_qualify_deployment_profile);
    for c in &s.cards {
        let q = &c.deployment_profile_qualifier;
        assert!(
            !q.holds_in_profiles.is_empty(),
            "card {} names no profile it holds in",
            c.card_id
        );
        // A profile is never both held-in and not-offered.
        for p in &q.not_offered_in_profiles {
            assert!(
                !q.holds_in_profiles.contains(p),
                "card {} double-claims profile {p:?}",
                c.card_id
            );
        }
    }
    // The managed AI gateway is honestly not offered air-gapped.
    let ai = s
        .card_for_family(ServiceFamily::AiGatewayFamily)
        .expect("ai gateway card");
    assert!(!ai.holds_in(DeploymentProfile::AirGapped));
    assert!(ai
        .deployment_profile_qualifier
        .not_offered_in_profiles
        .contains(&DeploymentProfile::AirGapped));
}

#[test]
fn procurement_and_support_packets_reuse_one_evidence_object() {
    let s = set();
    assert!(s.inspection.all_cards_link_procurement_evidence);
    let procurement = s
        .surface_bindings
        .iter()
        .find(|b| b.surface == BoundarySurface::ProcurementPacket)
        .expect("procurement binding");
    let support = s
        .surface_bindings
        .iter()
        .find(|b| b.surface == BoundarySurface::SupportAdminPacket)
        .expect("support binding");
    // Procurement and support project the same set of cards (same object model).
    assert_eq!(procurement.bound_card_ids, support.bound_card_ids);
    for c in &s.cards {
        assert!(
            !c.procurement_support_evidence.packet_kinds.is_empty(),
            "card {} links no procurement packet",
            c.card_id
        );
        assert!(!c
            .procurement_support_evidence
            .support_admin_packet_ref
            .trim()
            .is_empty());
    }
}

#[test]
fn export_and_procurement_are_never_outranked_by_upsell() {
    let s = set();
    assert!(s.inspection.upsell_never_outranks_truth);
    for c in &s.cards {
        assert!(
            c.upsell_never_outranks_truth(),
            "card {} let upsell outrank truth",
            c.card_id
        );
        // Export, continue-local, and procurement are always offered.
        for required in [
            BoundaryActionKind::ExportEvidence,
            BoundaryActionKind::ContinueLocal,
            BoundaryActionKind::ViewProcurementPacket,
        ] {
            assert!(
                c.actions.iter().any(|a| a.kind == required),
                "card {} missing {required:?}",
                c.card_id
            );
        }
        let protected_max = c
            .actions
            .iter()
            .filter(|a| a.kind.is_protected_priority())
            .map(|a| a.rank)
            .max()
            .expect("protected actions present");
        for upsell in c.actions.iter().filter(|a| a.kind.is_upsell_prompt()) {
            assert!(
                upsell.rank > protected_max,
                "card {} ranked an upsell above protected truth",
                c.card_id
            );
        }
    }
    // Managed cards carry an upsell; the open core does not.
    assert!(s
        .card_for_family(ServiceFamily::AiGatewayFamily)
        .unwrap()
        .actions
        .iter()
        .any(|a| a.kind.is_upsell_prompt()));
}

#[test]
fn no_number_crosses_the_boundary_bare() {
    let s = set();
    assert!(s.inspection.value_never_bare);
    for c in &s.cards {
        assert!(
            !c.as_of.trim().is_empty(),
            "card {} lost its as-of",
            c.card_id
        );
        // Boundary cards defer numbers to the metering surfaces.
        assert_eq!(
            c.cost_figure_disclosure,
            CostFigureDisclosure::DeferredToMeteringSurfaces
        );
        assert!(!c.cost_figure_disclosure.shows_number());
    }
}

#[test]
fn cards_project_the_control_plane_lanes() {
    let s = set();
    let violations = s.cross_check_against_control_plane();
    assert!(
        violations.is_empty(),
        "cards drifted from the control plane: {violations:?}"
    );
    // The managed cards declare the full managed claim before narrowing.
    for c in s
        .cards
        .iter()
        .filter(|c| c.boundary_class.is_managed_paid())
    {
        assert_eq!(c.declared_marketed_claim, MarketedClaim::ManagedFull);
    }
}

#[test]
fn evidence_status_narrows_managed_cards_but_never_the_open_core() {
    let mut s = set();
    // Current evidence: nothing narrowed.
    assert_eq!(s.inspection.narrowed_card_count, 0);
    assert_eq!(
        s.inspection.effective_full_card_count,
        ServiceFamily::ALL.len()
    );

    // Stale evidence narrows every managed card to managed_narrowed.
    s.apply_evidence_status(BoundaryEvidenceStatus::Stale);
    assert!(s.validate().is_empty(), "narrowed set still validates");
    assert_eq!(s.inspection.narrowed_card_count, ServiceFamily::ALL.len());
    assert_eq!(s.inspection.effective_full_card_count, 0);
    for c in s
        .cards
        .iter()
        .filter(|c| c.boundary_class.is_managed_paid())
    {
        assert_eq!(c.effective_marketed_claim, MarketedClaim::ManagedNarrowed);
        assert!(c.recovery_cue.is_some(), "narrowed card needs a cue");
    }
    // The open core never narrows; it stays at the local-safe claim with no cue.
    let open = s.local_open_card().unwrap();
    assert_eq!(open.effective_marketed_claim, MarketedClaim::LocalSafeOnly);
    assert!(open.recovery_cue.is_none());

    // Missing evidence drops managed cards to local-safe-only.
    s.apply_evidence_status(BoundaryEvidenceStatus::Missing);
    assert!(s.validate().is_empty());
    for c in s
        .cards
        .iter()
        .filter(|c| c.boundary_class.is_managed_paid())
    {
        assert_eq!(c.effective_marketed_claim, MarketedClaim::LocalSafeOnly);
    }
    // Every card keeps its local-safe baseline through narrowing — local core never blocked.
    for c in &s.cards {
        assert!(!c.local_safe_baseline.is_empty());
    }
}

#[test]
fn every_surface_is_bound() {
    let s = set();
    assert!(s.inspection.surface_coverage_complete);
    for surface in BoundarySurface::ALL {
        let binding = s
            .surface_bindings
            .iter()
            .find(|b| b.surface == surface)
            .unwrap_or_else(|| panic!("missing surface {surface:?}"));
        assert!(binding.projects_effective_claim);
        assert!(binding.renders_local_safe_baseline);
        assert!(binding.discloses_residual_dependencies);
        assert!(binding.names_deployment_profile_qualifier);
        assert!(binding.surfaces_evidence_before_upsell);
        assert!(!binding.bound_card_ids.is_empty());
    }
}

#[test]
fn emptying_a_local_safe_baseline_is_rejected() {
    let mut s = set();
    s.cards[0].local_safe_baseline.clear();
    s.inspection = BoundaryCardInspection::derive(&s.cards, &s.surface_bindings);
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "card.local_safe_baseline"),
        "expected a local-safe-baseline violation, got {violations:?}"
    );
}

#[test]
fn a_managed_card_hiding_residual_dependencies_is_rejected() {
    let mut s = set();
    let idx = s
        .cards
        .iter()
        .position(|c| c.boundary_class.is_managed_paid())
        .expect("a managed card is present");
    s.cards[idx].residual_dependencies.clear();
    s.inspection = BoundaryCardInspection::derive(&s.cards, &s.surface_bindings);
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "card.residual_dependencies"),
        "expected a residual-dependencies violation, got {violations:?}"
    );
}

#[test]
fn overstating_the_open_core_with_a_residual_dependency_is_rejected() {
    let mut s = set();
    let idx = s
        .cards
        .iter()
        .position(|c| c.boundary_class == BoundaryClass::LocalOpenSource)
        .expect("open core card present");
    s.cards[idx]
        .residual_dependencies
        .push(ResidualDependency::new(
            DependencyClass::HostedControlPlaneReachability,
            true,
            false,
            "forged dependency on the open core",
        ));
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "card.residual_dependencies"),
        "expected an open-core residual violation, got {violations:?}"
    );
}

#[test]
fn burying_export_beneath_upsell_is_rejected() {
    let mut s = set();
    let idx = s
        .cards
        .iter()
        .position(|c| c.actions.iter().any(|a| a.kind.is_upsell_prompt()))
        .expect("a card with an upsell prompt is present");
    for action in &mut s.cards[idx].actions {
        if action.kind == BoundaryActionKind::LearnAboutPaid {
            action.rank = 0;
        }
    }
    s.inspection = BoundaryCardInspection::derive(&s.cards, &s.surface_bindings);
    let violations = s.validate();
    assert!(
        violations.iter().any(|v| v.field == "card.actions"),
        "expected an actions violation, got {violations:?}"
    );
}

#[test]
fn forged_effective_claim_is_rejected() {
    let mut s = set();
    let idx = s
        .cards
        .iter()
        .position(|c| c.boundary_class.is_managed_paid())
        .unwrap();
    s.cards[idx].effective_marketed_claim = MarketedClaim::LocalSafeOnly;
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "card.effective_marketed_claim"),
        "expected an effective-claim violation, got {violations:?}"
    );
}

#[test]
fn a_profile_both_held_and_not_offered_is_rejected() {
    let mut s = set();
    let idx = s
        .cards
        .iter()
        .position(|c| c.boundary_class.is_managed_paid())
        .unwrap();
    let held = s.cards[idx].deployment_profile_qualifier.holds_in_profiles[0];
    s.cards[idx]
        .deployment_profile_qualifier
        .not_offered_in_profiles
        .push(held);
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "card.deployment_profile_qualifier.not_offered_in_profiles"),
        "expected a profile-qualifier violation, got {violations:?}"
    );
}

#[test]
fn dropping_a_managed_lane_card_is_rejected() {
    let mut s = set();
    let idx = s
        .cards
        .iter()
        .position(|c| c.boundary_class.is_managed_paid())
        .unwrap();
    s.cards.remove(idx);
    s.inspection = BoundaryCardInspection::derive(&s.cards, &s.surface_bindings);
    let violations = s.validate();
    assert!(
        violations.iter().any(|v| v.field == "cards"),
        "expected a cards violation, got {violations:?}"
    );
}

#[test]
fn missing_surface_is_rejected() {
    let mut s = set();
    s.surface_bindings
        .retain(|b| b.surface != BoundarySurface::ReleaseCenter);
    s.inspection = BoundaryCardInspection::derive(&s.cards, &s.surface_bindings);
    let violations = s.validate();
    assert!(
        violations.iter().any(|v| v.field == "surface_bindings"),
        "expected a surface-binding violation, got {violations:?}"
    );
}

#[test]
fn an_empty_procurement_evidence_is_rejected() {
    let mut s = set();
    s.cards[0].procurement_support_evidence.packet_kinds.clear();
    s.inspection = BoundaryCardInspection::derive(&s.cards, &s.surface_bindings);
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "card.procurement_support_evidence.packet_kinds"),
        "expected a procurement-evidence violation, got {violations:?}"
    );
}

#[test]
fn export_json_round_trips() {
    let s = set();
    let json = s.export_safe_json();
    let parsed: BoundaryCardSet =
        serde_json::from_str(&json).expect("set round-trips through JSON");
    assert_eq!(parsed, s);
}
