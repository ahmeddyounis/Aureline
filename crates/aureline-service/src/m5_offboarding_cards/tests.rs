//! Tests for the frozen offboarding-card set.

use super::*;

fn set() -> CardSet {
    canonical_stable_offboarding_card_set()
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
        current_stable_offboarding_card_set().expect("checked-in set parses and validates");
    assert_eq!(
        stable,
        set(),
        "the checked-in artifact drifted from the canonical builder; regenerate it with the dump example"
    );
}

#[test]
fn set_covers_every_lifecycle_event_exactly_once() {
    let s = set();
    assert!(s.inspection.lifecycle_vocab_complete);
    assert_eq!(s.inspection.card_count, LifecycleEvent::ALL.len());
    assert_eq!(
        s.inspection.lifecycle_events_covered,
        LifecycleEvent::ALL.len()
    );
    for event in LifecycleEvent::ALL {
        assert!(s.card_for(event).is_some(), "missing card for {event:?}");
    }
}

#[test]
fn every_card_keeps_a_local_safe_continuation_and_never_deletes_local_data() {
    let s = set();
    assert!(s.inspection.all_cards_local_safe_backed);
    assert!(s.inspection.never_deletes_local_artifacts);
    for c in &s.cards {
        assert!(
            !c.local_safe_continuation.is_empty(),
            "card {} lost its continuation",
            c.card_id
        );
        assert!(c
            .local_safe_continuation
            .iter()
            .all(|p| !p.trim().is_empty()));
        // A managed lifecycle event never deletes local data.
        assert!(
            !c.deletion_timeline.local_artifacts_deleted,
            "card {} deleted local artifacts",
            c.card_id
        );
    }
}

#[test]
fn every_card_states_the_required_offboarding_facts() {
    let s = set();
    assert!(s.inspection.all_cards_state_deletion_timeline);
    assert!(s.inspection.all_cards_name_owner_handoff);
    for c in &s.cards {
        // Event type, effective date, impacted features, export rights, deletion
        // timeline, and owner handoff are all present.
        assert!(
            !c.effective_at.trim().is_empty(),
            "card {} has effective date",
            c.card_id
        );
        assert!(
            !c.impacted_managed_features.is_empty(),
            "card {} names impacted features",
            c.card_id
        );
        assert!(
            !c.impacted_service_families.is_empty(),
            "card {} names impacted families",
            c.card_id
        );
        assert!(
            !c.export_rights.is_empty(),
            "card {} names export rights",
            c.card_id
        );
        assert!(
            !c.deletion_timeline
                .export_admissible_until
                .trim()
                .is_empty(),
            "card {} states the export deadline",
            c.card_id
        );
        assert_eq!(
            c.deletion_timeline.effective_at, c.effective_at,
            "card {} timeline effective date matches",
            c.card_id
        );
        assert!(
            !c.owner_handoff.instruction.trim().is_empty(),
            "card {} names the next-step owner",
            c.card_id
        );
    }
}

#[test]
fn org_switch_and_seat_loss_separate_local_from_tenant_scoped_state() {
    let s = set();
    assert!(s.inspection.all_cards_separate_local_from_tenant);
    for event in [LifecycleEvent::OrgSwitch, LifecycleEvent::SeatLoss] {
        let c = s.card_for(event).expect("card present");
        assert!(
            !c.artifact_separation.local_artifacts.is_empty(),
            "{event:?} names local artifacts"
        );
        assert!(
            !c.artifact_separation.tenant_scoped_managed_state.is_empty(),
            "{event:?} names tenant-scoped state"
        );
    }
}

#[test]
fn export_and_local_continuation_are_never_buried() {
    let s = set();
    assert!(s.inspection.export_never_buried);
    for c in &s.cards {
        assert!(
            c.export_never_buried(),
            "card {} buried export or local continuation",
            c.card_id
        );
        // Export and local continuation are always offered.
        assert!(
            c.actions
                .iter()
                .any(|a| a.kind == CardActionKind::ExportNow),
            "card {} offers export",
            c.card_id
        );
        assert!(
            c.actions
                .iter()
                .any(|a| a.kind == CardActionKind::ContinueLocal),
            "card {} offers local continuation",
            c.card_id
        );
        // Any upgrade prompt ranks strictly below export and continue-local.
        let protected_max = c
            .actions
            .iter()
            .filter(|a| a.kind.is_protected_priority())
            .map(|a| a.rank)
            .max()
            .expect("protected actions present");
        for upgrade in c.actions.iter().filter(|a| a.kind.is_upgrade_prompt()) {
            assert!(
                upgrade.rank > protected_max,
                "card {} ranked an upgrade prompt above export",
                c.card_id
            );
        }
    }
    // The grace and cancellation cards carry an upgrade prompt; seat loss and org
    // switch do not.
    assert!(s
        .card_for(LifecycleEvent::GracePeriod)
        .unwrap()
        .actions
        .iter()
        .any(|a| a.kind.is_upgrade_prompt()));
    assert!(!s
        .card_for(LifecycleEvent::OrgSwitch)
        .unwrap()
        .actions
        .iter()
        .any(|a| a.kind.is_upgrade_prompt()));
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
        // The grace and cancellation cards show a bound final figure; seat loss and
        // org switch suppress it.
        match c.lifecycle_event {
            LifecycleEvent::GracePeriod | LifecycleEvent::Cancellation => {
                assert_eq!(
                    c.final_usage_disclosure,
                    FinalUsageDisclosure::BoundToUnitAsOfScope
                );
                assert!(c.final_usage_disclosure.shows_number());
            }
            LifecycleEvent::SeatLoss | LifecycleEvent::OrgSwitch => {
                assert_eq!(
                    c.final_usage_disclosure,
                    FinalUsageDisclosure::SuppressedNoManagedNumber
                );
                assert!(!c.final_usage_disclosure.shows_number());
            }
        }
    }
}

#[test]
fn the_four_events_stay_distinct() {
    let s = set();
    assert!(s.inspection.distinctness_complete);
    for c in &s.cards {
        assert!(c.distinct_from_sign_in_failure);
        assert!(c.not_a_generic_account_error);
        assert!(!c.must_not_collapse_with.contains(&c.lifecycle_event));
        for other in LifecycleEvent::ALL {
            if other != c.lifecycle_event {
                assert!(
                    c.must_not_collapse_with.contains(&other),
                    "card {} must stay distinct from {other:?}",
                    c.card_id
                );
            }
        }
    }
}

#[test]
fn the_marketed_claim_narrows_from_the_event_cap() {
    let s = set();
    for c in &s.cards {
        assert_eq!(c.declared_marketed_claim, MarketedClaim::ManagedFull);
        assert_eq!(c.effective_marketed_claim, c.lifecycle_event.claim_cap());
    }
    // Grace and org switch narrow to managed-narrowed; seat loss and cancellation
    // narrow to local-safe-only.
    assert_eq!(
        s.card_for(LifecycleEvent::GracePeriod)
            .unwrap()
            .effective_marketed_claim,
        MarketedClaim::ManagedNarrowed
    );
    assert_eq!(
        s.card_for(LifecycleEvent::OrgSwitch)
            .unwrap()
            .effective_marketed_claim,
        MarketedClaim::ManagedNarrowed
    );
    assert_eq!(s.inspection.local_safe_only_card_count, 2);
    assert_eq!(s.inspection.narrowed_card_count, LifecycleEvent::ALL.len());
}

#[test]
fn cards_project_their_control_plane_managed_state() {
    let s = set();
    let violations = s.cross_check_against_control_plane();
    assert!(
        violations.is_empty(),
        "cards drifted from the control-plane matrix: {violations:?}"
    );
    // Only cancellation has no mapped managed state.
    assert_eq!(
        s.card_for(LifecycleEvent::Cancellation)
            .unwrap()
            .related_managed_state,
        None
    );
    assert_eq!(
        s.card_for(LifecycleEvent::SeatLoss)
            .unwrap()
            .related_managed_state,
        Some(ManagedStateClass::SeatRemoved)
    );
}

#[test]
fn every_consumer_surface_is_bound() {
    let s = set();
    for surface in ConsumerSurface::ALL {
        let binding = s
            .surface_bindings
            .iter()
            .find(|b| b.consumer_surface == surface)
            .unwrap_or_else(|| panic!("missing surface {surface:?}"));
        assert!(binding.projects_effective_claim);
        assert!(binding.renders_local_safe_continuation);
        assert!(binding.names_owner_handoff);
        assert!(binding.surfaces_export_before_upgrade);
        assert!(!binding.bound_card_ids.is_empty());
    }
}

#[test]
fn deleting_local_artifacts_is_rejected() {
    let mut s = set();
    s.cards[0].deletion_timeline.local_artifacts_deleted = true;
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "card.deletion_timeline.local_artifacts_deleted"),
        "expected a local-artifacts violation, got {violations:?}"
    );
}

#[test]
fn emptying_a_local_safe_continuation_is_rejected() {
    let mut s = set();
    s.cards[0].local_safe_continuation.clear();
    s.inspection = OffboardingCardInspection::derive(&s.cards, &s.surface_bindings);
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "card.local_safe_continuation"),
        "expected a local-safe-continuation violation, got {violations:?}"
    );
}

#[test]
fn collapsing_local_and_tenant_state_is_rejected() {
    let mut s = set();
    s.cards[0]
        .artifact_separation
        .tenant_scoped_managed_state
        .clear();
    s.inspection = OffboardingCardInspection::derive(&s.cards, &s.surface_bindings);
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "card.artifact_separation.tenant_scoped_managed_state"),
        "expected an artifact-separation violation, got {violations:?}"
    );
}

#[test]
fn burying_export_beneath_upgrade_is_rejected() {
    let mut s = set();
    let idx = s
        .cards
        .iter()
        .position(|c| c.actions.iter().any(|a| a.kind.is_upgrade_prompt()))
        .expect("a card with an upgrade prompt is present");
    // Re-rank the upgrade prompt above the export action.
    for action in &mut s.cards[idx].actions {
        if action.kind == CardActionKind::UpgradeOrRenew {
            action.rank = 0;
        }
    }
    s.inspection = OffboardingCardInspection::derive(&s.cards, &s.surface_bindings);
    let violations = s.validate();
    assert!(
        violations.iter().any(|v| v.field == "card.actions"),
        "expected an actions violation, got {violations:?}"
    );
}

#[test]
fn dropping_a_distinctness_is_rejected() {
    let mut s = set();
    s.cards[0]
        .must_not_collapse_with
        .retain(|e| *e != LifecycleEvent::SeatLoss && *e != LifecycleEvent::OrgSwitch);
    s.inspection = OffboardingCardInspection::derive(&s.cards, &s.surface_bindings);
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "card.must_not_collapse_with"),
        "expected a distinctness violation, got {violations:?}"
    );
}

#[test]
fn forged_effective_claim_is_rejected() {
    let mut s = set();
    s.cards[0].effective_marketed_claim = MarketedClaim::ManagedFull;
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "card.effective_marketed_claim"),
        "expected an effective-claim violation, got {violations:?}"
    );
}

#[test]
fn dropping_an_event_card_is_rejected() {
    let mut s = set();
    s.cards.remove(0);
    s.inspection = OffboardingCardInspection::derive(&s.cards, &s.surface_bindings);
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
        .retain(|b| b.consumer_surface != ConsumerSurface::ClaimPublicTruthAutomation);
    s.inspection = OffboardingCardInspection::derive(&s.cards, &s.surface_bindings);
    let violations = s.validate();
    assert!(
        violations.iter().any(|v| v.field == "surface_bindings"),
        "expected a surface-binding violation, got {violations:?}"
    );
}

#[test]
fn forged_related_managed_state_is_rejected() {
    let mut s = set();
    // Cancellation has no mapped managed state; forge one onto it.
    let idx = s
        .cards
        .iter()
        .position(|c| c.lifecycle_event == LifecycleEvent::Cancellation)
        .expect("cancellation card present");
    s.cards[idx].related_managed_state = Some(ManagedStateClass::SeatRemoved);
    let violations = s.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.field == "card.related_managed_state"),
        "expected a related-managed-state violation, got {violations:?}"
    );
}

#[test]
fn export_json_round_trips() {
    let s = set();
    let json = s.export_safe_json();
    let parsed: CardSet = serde_json::from_str(&json).expect("set round-trips through JSON");
    assert_eq!(parsed, s);
}
