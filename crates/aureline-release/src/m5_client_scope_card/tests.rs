//! Inline tests for the M5 client-scope-card lane.

use super::*;

fn registry() -> M5ClientScopeCardRegistry {
    seeded_m5_client_scope_card_registry()
}

#[test]
fn canonical_registry_validates() {
    let registry = registry();
    assert!(registry.validate().is_empty(), "{:?}", registry.validate());
    assert_eq!(registry.registry_id, M5_CLIENT_SCOPE_CARD_REGISTRY_ID);
    assert_eq!(
        registry.record_kind,
        M5_CLIENT_SCOPE_CARD_REGISTRY_RECORD_KIND
    );
    assert_eq!(registry.schema_version, M5_CLIENT_SCOPE_CARD_SCHEMA_VERSION);
    assert_eq!(registry.cards.len(), 6);
    assert!(registry.conformance.all_hold());
    assert!(registry.vocabulary.matches_canonical());
}

#[test]
fn every_card_validates_and_its_guard_holds() {
    for card in registry().cards {
        assert!(
            card.validate().is_empty(),
            "{}: {:?}",
            card.card_id,
            card.validate()
        );
        assert!(card.guard.all_hold(), "{} guard failed", card.card_id);
    }
}

#[test]
fn all_four_surface_classes_are_covered() {
    let registry = registry();
    for surface in SurfaceClass::ALL {
        assert!(
            registry.cards.iter().any(|c| c.surface_class == surface),
            "surface class {} not covered",
            surface.as_str()
        );
    }
    assert!(registry.conformance.all_surface_classes_covered);
}

#[test]
fn only_desktop_card_carries_full_authority() {
    let registry = registry();
    for card in &registry.cards {
        if card.surface_class == SurfaceClass::Desktop {
            assert!(
                card.claims_full_authority,
                "{} should be full",
                card.card_id
            );
            assert!(card.authority_class.is_full_authority());
            assert!(card.parity_caveats.is_empty());
            assert!(card.blocked_actions.is_empty());
        } else {
            assert!(
                !card.claims_full_authority,
                "{} must not be full authority",
                card.card_id
            );
            assert!(!card.authority_class.is_full_authority());
        }
    }
    assert!(registry.conformance.only_desktop_carries_full_authority);
    assert_eq!(registry.summary.full_authority_cards, 1);
}

#[test]
fn narrowed_cards_state_their_limits() {
    // Every narrowed card carries at least one parity caveat and one blocked action, so a user
    // sees the limit before failing into it.
    for card in registry().cards.iter().filter(|c| c.is_narrowed()) {
        assert!(
            !card.parity_caveats.is_empty(),
            "{} hides parity caveats",
            card.card_id
        );
        assert!(
            !card.blocked_actions.is_empty(),
            "{} hides blocked actions",
            card.card_id
        );
    }
    assert!(registry().conformance.parity_caveats_present_when_narrowed);
    assert!(registry().conformance.narrowed_never_implies_desktop_parity);
}

#[test]
fn blocked_actions_track_the_authority_ladder() {
    let registry = registry();
    // A scoped-authority browser companion can observe and edit but cannot approve / administer.
    let companion = registry
        .card("client-scope-card:browser-companion")
        .unwrap();
    let blocked: Vec<AuthorityCapability> = companion
        .blocked_actions
        .iter()
        .map(|a| a.capability)
        .collect();
    assert_eq!(
        blocked,
        vec![
            AuthorityCapability::Approve,
            AuthorityCapability::Administer
        ]
    );
    assert_eq!(
        companion.granted_capabilities,
        vec![
            AuthorityCapability::Observe,
            AuthorityCapability::MutateInPlace
        ]
    );

    // A reference-only surface can only observe; everything else is blocked.
    let reference = registry
        .card("client-scope-card:browser-reference")
        .unwrap();
    let blocked: Vec<AuthorityCapability> = reference
        .blocked_actions
        .iter()
        .map(|a| a.capability)
        .collect();
    assert_eq!(
        blocked,
        vec![
            AuthorityCapability::MutateInPlace,
            AuthorityCapability::Approve,
            AuthorityCapability::Administer
        ]
    );

    // A handoff-only / not-provided surface blocks every capability.
    let unsupported = registry
        .card("client-scope-card:unsupported-handoff")
        .unwrap();
    assert_eq!(
        unsupported.blocked_actions.len(),
        AuthorityCapability::ALL.len()
    );
    assert!(unsupported.granted_capabilities.is_empty());
}

#[test]
fn blocked_actions_carry_a_recovery_handoff() {
    let registry = registry();
    let reference = registry
        .card("client-scope-card:browser-reference")
        .unwrap();
    for action in &reference.blocked_actions {
        // The browser-reference surface recovers via a console handoff.
        assert_eq!(action.recovery, HandoffRequirement::ConsoleHandoffRequired);
    }
    let companion = registry
        .card("client-scope-card:browser-companion")
        .unwrap();
    for action in &companion.blocked_actions {
        assert_eq!(action.recovery, HandoffRequirement::DesktopHandoffRequired);
    }
    assert!(registry.conformance.blocked_actions_attributable);
}

#[test]
fn claim_state_matches_the_shared_narrowing_runtime() {
    let registry = registry();
    let desktop = registry.card("client-scope-card:desktop-full").unwrap();
    assert_eq!(desktop.claim_state, NarrowedClaimState::FullySupported);
    for card in registry.cards.iter().filter(|c| c.is_narrowed()) {
        // A client-scope narrowing always resolves to unsupported_client (it never blocks).
        assert_eq!(
            card.claim_state,
            NarrowedClaimState::UnsupportedClient,
            "{} claim state",
            card.card_id
        );
    }
    assert!(registry.conformance.claim_state_matches_narrowing_runtime);
}

#[test]
fn deep_link_and_handoff_disclosures_preserve_the_truth() {
    for card in registry().cards {
        for disclosure in card.disclosures.iter().filter(|d| {
            matches!(
                d.surface,
                DisclosureSurface::DeepLink | DisclosureSurface::Handoff
            )
        }) {
            assert_eq!(disclosure.authority_class, card.authority_class);
            assert_eq!(disclosure.handoff_requirement, card.handoff_requirement);
            assert_eq!(disclosure.claims_full_authority, card.claims_full_authority);
            assert_eq!(
                disclosure.blocked_action_count,
                card.blocked_actions.len() as u32
            );
            assert_eq!(
                disclosure.parity_caveat_count,
                card.parity_caveats.len() as u32
            );
            assert!(disclosure.implies_no_broader_authority);
        }
    }
    assert!(registry().conformance.deep_link_and_handoff_preserve_truth);
}

#[test]
fn every_disclosure_surface_is_projected_in_order() {
    for card in registry().cards {
        let surfaces: Vec<DisclosureSurface> = card.disclosures.iter().map(|d| d.surface).collect();
        assert_eq!(
            surfaces,
            DisclosureSurface::ALL.to_vec(),
            "{}",
            card.card_id
        );
    }
    assert!(registry().conformance.all_disclosures_projected);
}

#[test]
fn required_handoff_is_disclosed_everywhere() {
    for card in registry()
        .cards
        .iter()
        .filter(|c| !c.handoff_requirement.is_in_product())
    {
        for disclosure in &card.disclosures {
            assert!(
                disclosure.requires_handoff_disclosure,
                "{} hides handoff on {}",
                card.card_id,
                disclosure.surface.as_str()
            );
        }
    }
    // The desktop card needs no handoff, so it discloses none.
    let desktop = registry()
        .card("client-scope-card:desktop-full")
        .unwrap()
        .clone();
    assert!(desktop
        .disclosures
        .iter()
        .all(|d| !d.requires_handoff_disclosure));
}

#[test]
fn not_provided_authority_and_handoff_stay_explicit() {
    let registry = registry();
    let card = registry
        .card("client-scope-card:unsupported-not-provided")
        .unwrap();
    assert_eq!(card.authority_class, AuthorityClass::NotProvided);
    assert_eq!(card.handoff_requirement, HandoffRequirement::NotProvided);
    // A not-provided authority still blocks every action and is never read at parity by omission.
    assert!(!card.claims_full_authority);
    assert_eq!(card.blocked_actions.len(), AuthorityCapability::ALL.len());
    let caveat_facets: Vec<DescriptorFacet> = card.parity_caveats.iter().map(|c| c.facet).collect();
    assert!(caveat_facets.contains(&DescriptorFacet::AuthorityClass));
    assert!(caveat_facets.contains(&DescriptorFacet::HandoffRequirement));
}

#[test]
fn a_narrowed_card_can_never_read_full_authority_on_a_disclosure() {
    let mut card = seeded_browser_companion_card();
    assert!(card.validate().is_empty());
    // Hand-edit a deep-link disclosure to claim full authority — the guard must catch it.
    card.disclosures[1].claims_full_authority = true;
    let violations = card.validate();
    assert!(
        violations.contains(&M5ClientScopeCardViolation::DisclosureImpliesBroaderAuthority)
            || violations.contains(&M5ClientScopeCardViolation::DisclosureDiverged),
        "{violations:?}"
    );
}

#[test]
fn dropping_a_blocked_action_from_a_card_is_caught() {
    let mut card = seeded_browser_reference_card();
    assert!(card.validate().is_empty());
    card.blocked_actions.pop();
    let violations = card.validate();
    assert!(
        violations.contains(&M5ClientScopeCardViolation::BlockedActionDrift),
        "{violations:?}"
    );
}

#[test]
fn promoting_a_narrowed_card_to_full_authority_is_caught() {
    let mut card = seeded_headless_card();
    assert!(card.validate().is_empty());
    card.claims_full_authority = true;
    let violations = card.validate();
    assert!(
        violations.contains(&M5ClientScopeCardViolation::NarrowedClaimsFullAuthority)
            || violations.contains(&M5ClientScopeCardViolation::AuthorityParityDrift),
        "{violations:?}"
    );
}

#[test]
fn vocabulary_is_frozen() {
    let vocab = ClientScopeCardVocabulary::canonical();
    assert!(vocab.matches_canonical());
    assert_eq!(vocab.surface_classes.len(), SurfaceClass::ALL.len());
    assert_eq!(
        vocab.disclosure_surfaces.len(),
        DisclosureSurface::ALL.len()
    );
    assert_eq!(vocab.capabilities.len(), AuthorityCapability::ALL.len());
    for needle in ["desktop", "browser_companion", "headless", "unsupported"] {
        assert!(
            vocab.surface_classes.iter().any(|s| s == needle),
            "missing {needle}"
        );
    }
    for needle in ["discovery", "deep_link", "handoff", "companion"] {
        assert!(
            vocab.disclosure_surfaces.iter().any(|s| s == needle),
            "missing {needle}"
        );
    }
}

#[test]
fn export_carries_no_raw_material() {
    let json = registry().export_safe_json();
    for needle in [
        "credential",
        "secret",
        "password",
        "api_key",
        "raw_payload",
        "bearer_token",
    ] {
        assert!(!json.contains(needle), "found {needle} in export");
    }
}

#[test]
fn registry_round_trips_through_json() {
    let registry = registry();
    let json = registry.export_safe_json();
    let restored: M5ClientScopeCardRegistry = serde_json::from_str(&json).unwrap();
    assert_eq!(registry, restored);
    assert!(restored.validate().is_empty());
}

#[test]
fn card_round_trips_through_json() {
    for card in registry().cards {
        let json = card.export_safe_json();
        let restored: ClientScopeCard = serde_json::from_str(&json).unwrap();
        assert_eq!(card, restored);
    }
}

#[test]
fn summary_counts_match() {
    let registry = registry();
    let s = &registry.summary;
    assert_eq!(s.total_cards, 6);
    assert_eq!(s.full_authority_cards, 1);
    assert_eq!(s.narrowed_cards, 5);
    let expected_disclosures: u32 = registry
        .cards
        .iter()
        .map(|c| c.disclosures.len() as u32)
        .sum();
    assert_eq!(s.total_disclosure_projections, expected_disclosures);
    let expected_blocked: u32 = registry
        .cards
        .iter()
        .map(|c| c.blocked_actions.len() as u32)
        .sum();
    assert_eq!(s.total_blocked_actions, expected_blocked);
}

#[test]
fn markdown_summary_is_deterministic() {
    let registry = registry();
    assert_eq!(
        registry.render_markdown_summary(),
        registry.render_markdown_summary()
    );
    assert!(registry
        .render_markdown_summary()
        .contains("client-scope card parity"));
}
