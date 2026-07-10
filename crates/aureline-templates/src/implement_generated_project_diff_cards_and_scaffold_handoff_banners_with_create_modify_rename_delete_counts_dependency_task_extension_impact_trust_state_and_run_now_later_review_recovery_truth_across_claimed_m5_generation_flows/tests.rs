use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_scaffold_generation_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, SCAFFOLD_GENERATION_CONTROLS_PACKET_ID);
    assert_eq!(packet.record_kind, SCAFFOLD_GENERATION_CONTROLS_RECORD_KIND);
}

#[test]
fn every_component_carries_its_frozen_family() {
    let packet = seeded_scaffold_generation_controls();
    assert!(!packet.diff_cards.is_empty());
    assert!(!packet.handoff_banners.is_empty());
    for card in &packet.diff_cards {
        assert_eq!(
            card.component,
            M5ScaffoldComponentFamily::GeneratedProjectDiffCard
        );
    }
    for banner in &packet.handoff_banners {
        assert_eq!(
            banner.component,
            M5ScaffoldComponentFamily::ScaffoldHandoffBanner
        );
    }
}

#[test]
fn ac_change_kind_vocabulary_is_frozen_exactly() {
    // The acceptance criteria pin the exact create / modify / rename / delete vocabulary Aureline
    // uses for AI patches, importers, and refactors; assert the exact tokens.
    let tokens: Vec<&str> = DiffChangeKind::ALL.iter().map(|k| k.as_str()).collect();
    assert_eq!(tokens, vec!["created", "modified", "renamed", "deleted"]);
}

#[test]
fn diff_disposition_is_derived_never_asserted() {
    let packet = seeded_scaffold_generation_controls();
    for card in &packet.diff_cards {
        let disclosure = resolve_diff_disclosure(card.generated_zone_class, card.diff_review_state);
        assert_eq!(
            card.derived_review_disposition,
            disclosure.review_disposition
        );
        assert_eq!(card.derived_boundary_posture, disclosure.boundary_posture);
        assert_eq!(card.claims_reviewable, disclosure.is_reviewable);
        assert_eq!(card.claims_blocking, disclosure.is_blocking);
        assert_eq!(card.claims_user_owned_boundary, disclosure.is_user_owned);
    }
}

#[test]
fn handoff_outcome_is_derived_never_asserted() {
    let packet = seeded_scaffold_generation_controls();
    for banner in &packet.handoff_banners {
        let disclosure = resolve_handoff_disclosure(banner.outcome_class);
        assert_eq!(banner.derived_outcome_posture, disclosure.outcome_posture);
        assert_eq!(banner.claims_clean_create, disclosure.is_clean_create);
        assert_eq!(banner.claims_needs_recovery, disclosure.needs_recovery);
        assert_eq!(banner.claims_trusted, banner.trust_state.is_trusted());
    }
}

#[test]
fn only_clean_outcome_reads_as_clean_create() {
    for outcome in [
        M5HandoffOutcomeClass::PartialBootstrap,
        M5HandoffOutcomeClass::CreateFailed,
        M5HandoffOutcomeClass::ContinuedWithoutStarter,
        M5HandoffOutcomeClass::CreatedEmpty,
        M5HandoffOutcomeClass::ProvisioningPending,
    ] {
        assert!(
            !resolve_handoff_disclosure(outcome).is_clean_create,
            "{outcome:?}"
        );
    }
    assert!(resolve_handoff_disclosure(M5HandoffOutcomeClass::CreateSucceeded).is_clean_create);
}

#[test]
fn conflict_and_unavailable_and_blocked_are_blocking() {
    for state in [
        M5DiffReviewState::ConflictDetected,
        M5DiffReviewState::DiffUnavailable,
        M5DiffReviewState::Blocked,
    ] {
        assert!(
            resolve_diff_disclosure(M5GeneratedZoneClass::GeneratedOnly, state).is_blocking,
            "{state:?}"
        );
    }
    for state in [
        M5DiffReviewState::PreviewReady,
        M5DiffReviewState::ReviewRequired,
        M5DiffReviewState::NoChanges,
    ] {
        assert!(
            !resolve_diff_disclosure(M5GeneratedZoneClass::GeneratedOnly, state).is_blocking,
            "{state:?}"
        );
    }
}

#[test]
fn only_user_owned_zone_reads_as_user_owned() {
    for zone in [
        M5GeneratedZoneClass::GeneratedOnly,
        M5GeneratedZoneClass::GeneratedThenEdited,
        M5GeneratedZoneClass::RuntimeOnly,
        M5GeneratedZoneClass::MixedZone,
        M5GeneratedZoneClass::ZoneUnknown,
    ] {
        assert!(
            !resolve_diff_disclosure(zone, M5DiffReviewState::PreviewReady).is_user_owned,
            "{zone:?}"
        );
    }
    assert!(
        resolve_diff_disclosure(
            M5GeneratedZoneClass::UserOwned,
            M5DiffReviewState::PreviewReady
        )
        .is_user_owned
    );
}

#[test]
fn diff_cards_cover_every_zone_review_disposition_posture_change_and_source() {
    let packet = seeded_scaffold_generation_controls();
    for zone in M5GeneratedZoneClass::ALL {
        assert!(
            packet
                .diff_cards
                .iter()
                .any(|c| c.generated_zone_class == zone),
            "missing zone {}",
            zone.as_str()
        );
    }
    for state in M5DiffReviewState::ALL {
        assert!(
            packet
                .diff_cards
                .iter()
                .any(|c| c.diff_review_state == state),
            "missing review state {}",
            state.as_str()
        );
    }
    for disposition in DiffReviewDisposition::ALL {
        assert!(
            packet
                .diff_cards
                .iter()
                .any(|c| c.derived_review_disposition == disposition),
            "missing disposition {}",
            disposition.as_str()
        );
    }
    for posture in GeneratedBoundaryPosture::ALL {
        assert!(
            packet
                .diff_cards
                .iter()
                .any(|c| c.derived_boundary_posture == posture),
            "missing boundary posture {}",
            posture.as_str()
        );
    }
    for kind in DiffChangeKind::ALL {
        assert!(
            packet.diff_cards.iter().any(|c| c.count_for(kind) > 0),
            "missing change kind {}",
            kind.as_str()
        );
    }
    for source in DiffSourceKind::ALL {
        assert!(
            packet.diff_cards.iter().any(|c| c.source_kind == source),
            "missing source kind {}",
            source.as_str()
        );
    }
}

#[test]
fn handoff_banners_cover_every_outcome_trust_posture_and_recovery() {
    let packet = seeded_scaffold_generation_controls();
    for outcome in M5HandoffOutcomeClass::ALL {
        assert!(
            packet
                .handoff_banners
                .iter()
                .any(|b| b.outcome_class == outcome),
            "missing outcome {}",
            outcome.as_str()
        );
    }
    for trust in HandoffTrustState::ALL {
        assert!(
            packet
                .handoff_banners
                .iter()
                .any(|b| b.trust_state == trust),
            "missing trust state {}",
            trust.as_str()
        );
    }
    for posture in HandoffOutcomePosture::ALL {
        assert!(
            packet
                .handoff_banners
                .iter()
                .any(|b| b.derived_outcome_posture == posture),
            "missing outcome posture {}",
            posture.as_str()
        );
    }
    for recovery in M5HandoffRecoveryAction::ALL {
        assert!(
            packet
                .handoff_banners
                .iter()
                .any(|b| b.recovery_actions.contains(&recovery)),
            "missing recovery action {}",
            recovery.as_str()
        );
    }
}

#[test]
fn every_component_offers_mandatory_actions_labels_and_keyboard_route() {
    let packet = seeded_scaffold_generation_controls();
    for card in &packet.diff_cards {
        for action in DiffCardAction::MANDATORY {
            assert!(card.card_actions.contains(&action));
        }
        assert!(card.offers_rollback_generated());
        assert!(card.declares_mandatory_labels());
        assert!(card
            .accessibility_routes
            .contains(&M5ScaffoldAccessibilityRoute::KeyboardFocusable));
    }
    for banner in &packet.handoff_banners {
        for action in HandoffBannerAction::MANDATORY {
            assert!(banner.banner_actions.contains(&action));
        }
        assert!(banner.offers_real_recovery());
        assert!(banner.declares_mandatory_labels());
        assert!(banner
            .accessibility_routes
            .contains(&M5ScaffoldAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn misrepresented_diff_disposition_fails() {
    let mut packet = seeded_scaffold_generation_controls();
    packet.diff_cards[0].claims_blocking = true;
    assert!(packet
        .validate()
        .contains(&ScaffoldGenerationControlsViolation::DiffDispositionMisrepresented));
}

#[test]
fn misrepresented_handoff_outcome_fails() {
    let mut packet = seeded_scaffold_generation_controls();
    packet.handoff_banners[0].claims_clean_create = false;
    assert!(packet
        .validate()
        .contains(&ScaffoldGenerationControlsViolation::HandoffOutcomeMisrepresented));
}

#[test]
fn missing_boundary_note_fails() {
    let mut packet = seeded_scaffold_generation_controls();
    packet.diff_cards[0].boundary_note = String::new();
    assert!(packet
        .validate()
        .contains(&ScaffoldGenerationControlsViolation::DiffBoundaryNoteMissing));
}

#[test]
fn missing_rollback_note_fails() {
    let mut packet = seeded_scaffold_generation_controls();
    packet.diff_cards[0].rollback_note = String::new();
    assert!(packet
        .validate()
        .contains(&ScaffoldGenerationControlsViolation::DiffRollbackNoteMissing));
}

#[test]
fn diff_card_without_rollback_action_fails() {
    let mut packet = seeded_scaffold_generation_controls();
    packet.diff_cards[0]
        .card_actions
        .retain(|a| *a != DiffCardAction::RollbackGenerated);
    assert!(packet
        .validate()
        .contains(&ScaffoldGenerationControlsViolation::DiffRollbackRecoveryMissing));
}

#[test]
fn handoff_banner_without_real_recovery_fails() {
    let mut packet = seeded_scaffold_generation_controls();
    packet.handoff_banners[0].recovery_actions = vec![
        M5HandoffRecoveryAction::OpenWorkspace,
        M5HandoffRecoveryAction::NoRecoveryNeeded,
    ];
    assert!(packet
        .validate()
        .contains(&ScaffoldGenerationControlsViolation::HandoffRecoveryPathMissing));
}

#[test]
fn missing_mandatory_diff_action_fails() {
    let mut packet = seeded_scaffold_generation_controls();
    packet.diff_cards[0]
        .card_actions
        .retain(|a| *a != DiffCardAction::ReviewOwnershipBoundary);
    assert!(packet
        .validate()
        .contains(&ScaffoldGenerationControlsViolation::DiffCardActionsIncomplete));
}

#[test]
fn missing_mandatory_handoff_action_fails() {
    let mut packet = seeded_scaffold_generation_controls();
    packet.handoff_banners[0]
        .banner_actions
        .retain(|a| *a != HandoffBannerAction::RunLater);
    assert!(packet
        .validate()
        .contains(&ScaffoldGenerationControlsViolation::HandoffBannerActionsIncomplete));
}

#[test]
fn missing_optional_setup_note_fails() {
    let mut packet = seeded_scaffold_generation_controls();
    packet.handoff_banners[0].optional_setup_note = String::new();
    assert!(packet
        .validate()
        .contains(&ScaffoldGenerationControlsViolation::HandoffOptionalSetupNoteMissing));
}

#[test]
fn missing_trust_note_fails() {
    let mut packet = seeded_scaffold_generation_controls();
    let banner = packet
        .handoff_banners
        .iter_mut()
        .find(|b| b.trust_state.needs_trust_note())
        .expect("a not-fully-trusted banner present");
    banner.trust_note = String::new();
    assert!(packet
        .validate()
        .contains(&ScaffoldGenerationControlsViolation::HandoffTrustNoteMissing));
}

#[test]
fn each_hard_invariant_fails_when_set() {
    let mut packet = seeded_scaffold_generation_controls();
    packet.diff_cards[0].hides_generated_versus_user_owned_boundary = true;
    assert!(packet
        .validate()
        .contains(&ScaffoldGenerationControlsViolation::GeneratedBoundaryHidden));

    let mut packet = seeded_scaffold_generation_controls();
    packet.handoff_banners[0].hides_side_effect_or_trust_state = true;
    assert!(packet
        .validate()
        .contains(&ScaffoldGenerationControlsViolation::SideEffectOrTrustStateHidden));

    let mut packet = seeded_scaffold_generation_controls();
    packet.handoff_banners[0].assumes_safest_next_step_without_recovery = true;
    assert!(packet
        .validate()
        .contains(&ScaffoldGenerationControlsViolation::SafestNextStepAssumedWithoutRecovery));

    let mut packet = seeded_scaffold_generation_controls();
    packet.diff_cards[0].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&ScaffoldGenerationControlsViolation::AlternateStateLabelInvented));
}

#[test]
fn deep_link_action_without_resolvable_kind_fails() {
    let mut packet = seeded_scaffold_generation_controls();
    let card = packet
        .diff_cards
        .iter_mut()
        .find(|c| c.card_actions.contains(&DiffCardAction::OpenDeepLink))
        .expect("a card offering a deep link");
    card.deep_link_kind = DeepLinkKind::NoDeepLink;
    card.deep_link_ref = String::new();
    assert!(packet
        .validate()
        .contains(&ScaffoldGenerationControlsViolation::DeepLinkUnresolved));
}

#[test]
fn missing_context_note_fails() {
    let mut packet = seeded_scaffold_generation_controls();
    packet.handoff_banners[0].context_note = String::new();
    assert!(packet
        .validate()
        .contains(&ScaffoldGenerationControlsViolation::ContextNoteMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_scaffold_generation_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ScaffoldGenerationControlsViolation::MissingSourceContracts));
}

#[test]
fn generation_review_incomplete_fails() {
    let mut packet = seeded_scaffold_generation_controls();
    packet
        .generation_review
        .conflict_or_failure_never_shown_as_clean = false;
    assert!(packet
        .validate()
        .contains(&ScaffoldGenerationControlsViolation::GenerationReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_scaffold_generation_controls();
    packet
        .consumer_projection
        .support_export_shows_component_truth = false;
    assert!(packet
        .validate()
        .contains(&ScaffoldGenerationControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_scaffold_generation_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ScaffoldGenerationControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_component() {
    let summary = seeded_scaffold_generation_controls().render_markdown_summary();
    for card in seeded_scaffold_generation_controls().diff_cards {
        assert!(summary.contains(&card.card_name));
    }
    for banner in seeded_scaffold_generation_controls().handoff_banners {
        assert!(summary.contains(&banner.banner_name));
    }
}

#[test]
fn matrix_csv_has_a_line_per_component() {
    let packet = seeded_scaffold_generation_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + packet.diff_cards.len() + packet.handoff_banners.len()
    );
    assert!(lines[0].starts_with("component,id,frozen_state,"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_scaffold_generation_controls_export()
        .expect("checked scaffold-generation controls export validates");
    assert_eq!(
        from_disk,
        seeded_scaffold_generation_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn scenario_fixtures_validate_and_keep_full_coverage() {
    for packet in [
        seeded_scaffold_generation_controls_diff_card_conflict(),
        seeded_scaffold_generation_controls_handoff_banner_partial(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn checked_scenario_fixtures_validate_and_match_seed_builders() {
    let conflict: GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-generated-project-diff-card-scaffold-handoff-banner-controls/diff_card_conflict.json"
        )))
        .expect("diff-card-conflict fixture parses");
    assert!(conflict.validate().is_empty());
    assert_eq!(
        conflict,
        seeded_scaffold_generation_controls_diff_card_conflict()
    );

    let partial: GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-generated-project-diff-card-scaffold-handoff-banner-controls/handoff_banner_partial.json"
        )))
        .expect("handoff-banner-partial fixture parses");
    assert!(partial.validate().is_empty());
    assert_eq!(
        partial,
        seeded_scaffold_generation_controls_handoff_banner_partial()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_scaffold_generation_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
}
