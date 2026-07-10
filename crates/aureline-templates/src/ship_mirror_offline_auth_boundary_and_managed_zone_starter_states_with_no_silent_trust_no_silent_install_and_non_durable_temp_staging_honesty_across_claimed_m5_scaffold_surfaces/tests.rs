use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_starter_boundary_states();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, STARTER_BOUNDARY_STATE_PACKET_ID);
    assert_eq!(packet.record_kind, STARTER_BOUNDARY_STATE_RECORD_KIND);
}

#[test]
fn ac_boundary_kind_vocabulary_is_frozen_exactly() {
    // The acceptance criteria pin the exact boundary variants a user must be able to tell apart
    // before any silent trust or install step; assert the exact tokens.
    let tokens: Vec<&str> = StarterBoundaryKind::ALL
        .iter()
        .map(|k| k.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "public_registry",
            "mirror_only",
            "offline_cache_only",
            "sign_in_required",
            "remote_or_managed_workspace",
            "non_durable_temp_staging",
        ]
    );
}

#[test]
fn access_and_availability_posture_are_derived_never_asserted() {
    let packet = seeded_starter_boundary_states();
    for state in &packet.boundary_states {
        let disclosure = resolve_starter_disclosure(state.boundary_kind, state.availability_state);
        assert_eq!(state.derived_access_posture, disclosure.access_posture);
        assert_eq!(
            state.derived_availability_posture,
            disclosure.availability_posture
        );
        assert_eq!(state.claims_requires_sign_in, disclosure.requires_sign_in);
        assert_eq!(
            state.claims_requires_managed_provisioning,
            disclosure.requires_managed_provisioning
        );
        assert_eq!(state.claims_is_non_durable, disclosure.is_non_durable);
        assert_eq!(
            state.claims_depends_on_mirror_or_cache,
            disclosure.depends_on_mirror_or_cache
        );
        assert_eq!(state.claims_reachable, disclosure.is_reachable);
    }
}

#[test]
fn only_public_registry_reads_as_direct_public_access() {
    for kind in [
        StarterBoundaryKind::MirrorOnly,
        StarterBoundaryKind::OfflineCacheOnly,
        StarterBoundaryKind::SignInRequired,
        StarterBoundaryKind::RemoteOrManagedWorkspace,
        StarterBoundaryKind::NonDurableTempStaging,
    ] {
        assert_ne!(
            resolve_starter_disclosure(kind, StarterAvailabilityState::Available).access_posture,
            StarterAccessPosture::DirectPublicAccess,
            "{kind:?}"
        );
    }
    assert_eq!(
        resolve_starter_disclosure(
            StarterBoundaryKind::PublicRegistry,
            StarterAvailabilityState::Available
        )
        .access_posture,
        StarterAccessPosture::DirectPublicAccess
    );
}

#[test]
fn sign_in_and_managed_and_non_durable_are_derived_from_kind() {
    assert!(
        resolve_starter_disclosure(
            StarterBoundaryKind::SignInRequired,
            StarterAvailabilityState::SignInPending
        )
        .requires_sign_in
    );
    assert!(
        resolve_starter_disclosure(
            StarterBoundaryKind::RemoteOrManagedWorkspace,
            StarterAvailabilityState::ProvisioningPending
        )
        .requires_managed_provisioning
    );
    assert!(
        resolve_starter_disclosure(
            StarterBoundaryKind::NonDurableTempStaging,
            StarterAvailabilityState::Unavailable
        )
        .is_non_durable
    );
}

#[test]
fn unavailable_and_blocked_are_not_reachable() {
    for state in [
        StarterAvailabilityState::SignInPending,
        StarterAvailabilityState::ProvisioningPending,
        StarterAvailabilityState::Unavailable,
    ] {
        assert!(
            !resolve_starter_disclosure(StarterBoundaryKind::PublicRegistry, state).is_reachable,
            "{state:?}"
        );
    }
    for state in [
        StarterAvailabilityState::Available,
        StarterAvailabilityState::MirrorReachableOnly,
        StarterAvailabilityState::CacheOnlyOffline,
    ] {
        assert!(
            resolve_starter_disclosure(StarterBoundaryKind::PublicRegistry, state).is_reachable,
            "{state:?}"
        );
    }
}

#[test]
fn boundary_states_cover_every_kind_availability_owner_freshness_posture_and_recovery() {
    let packet = seeded_starter_boundary_states();
    for kind in StarterBoundaryKind::ALL {
        assert!(
            packet
                .boundary_states
                .iter()
                .any(|s| s.boundary_kind == kind),
            "missing boundary kind {}",
            kind.as_str()
        );
    }
    for availability in StarterAvailabilityState::ALL {
        assert!(
            packet
                .boundary_states
                .iter()
                .any(|s| s.availability_state == availability),
            "missing availability state {}",
            availability.as_str()
        );
    }
    for owner in StarterOwnerClass::ALL {
        assert!(
            packet
                .boundary_states
                .iter()
                .any(|s| s.owner_class == owner),
            "missing owner class {}",
            owner.as_str()
        );
    }
    for freshness in StarterFreshnessState::ALL {
        assert!(
            packet
                .boundary_states
                .iter()
                .any(|s| s.freshness_state == freshness),
            "missing freshness state {}",
            freshness.as_str()
        );
    }
    for posture in StarterAccessPosture::ALL {
        assert!(
            packet
                .boundary_states
                .iter()
                .any(|s| s.derived_access_posture == posture),
            "missing access posture {}",
            posture.as_str()
        );
    }
    for posture in StarterAvailabilityPosture::ALL {
        assert!(
            packet
                .boundary_states
                .iter()
                .any(|s| s.derived_availability_posture == posture),
            "missing availability posture {}",
            posture.as_str()
        );
    }
    for verb in StarterRecoveryVerb::ALL {
        assert!(
            packet
                .boundary_states
                .iter()
                .any(|s| s.recovery_verbs.contains(&verb)),
            "missing recovery verb {}",
            verb.as_str()
        );
    }
}

#[test]
fn every_state_offers_mandatory_actions_labels_recovery_and_keyboard_route() {
    let packet = seeded_starter_boundary_states();
    for state in &packet.boundary_states {
        for action in StarterStateAction::MANDATORY {
            assert!(state.state_actions.contains(&action));
        }
        assert!(state.offers_real_recovery());
        assert!(state.offers_continue_without_starter());
        assert!(state.declares_mandatory_labels());
        assert!(state
            .accessibility_routes
            .contains(&M5ScaffoldAccessibilityRoute::KeyboardFocusable));
        assert!(!state.applies_to_components.is_empty());
    }
}

#[test]
fn misrepresented_access_posture_fails() {
    let mut packet = seeded_starter_boundary_states();
    packet.boundary_states[0].claims_requires_sign_in = true;
    assert!(packet
        .validate()
        .contains(&StarterBoundaryStateViolation::AccessPostureMisrepresented));
}

#[test]
fn missing_source_or_owner_label_fails() {
    let mut packet = seeded_starter_boundary_states();
    packet.boundary_states[0].source_label = String::new();
    assert!(packet
        .validate()
        .contains(&StarterBoundaryStateViolation::SourceLabelMissing));

    let mut packet = seeded_starter_boundary_states();
    packet.boundary_states[0].owner_label = String::new();
    assert!(packet
        .validate()
        .contains(&StarterBoundaryStateViolation::OwnerLabelMissing));
}

#[test]
fn missing_trust_or_install_disclosure_fails() {
    let mut packet = seeded_starter_boundary_states();
    packet.boundary_states[0].trust_disclosure_note = String::new();
    assert!(packet
        .validate()
        .contains(&StarterBoundaryStateViolation::TrustDisclosureNoteMissing));

    let mut packet = seeded_starter_boundary_states();
    packet.boundary_states[0].install_disclosure_note = String::new();
    assert!(packet
        .validate()
        .contains(&StarterBoundaryStateViolation::InstallDisclosureNoteMissing));
}

#[test]
fn missing_sign_in_note_fails() {
    let mut packet = seeded_starter_boundary_states();
    let state = packet
        .boundary_states
        .iter_mut()
        .find(|s| s.disclosure().needs_sign_in_note)
        .expect("a sign-in-gated state present");
    state.sign_in_note = String::new();
    assert!(packet
        .validate()
        .contains(&StarterBoundaryStateViolation::SignInNoteMissing));
}

#[test]
fn missing_non_durable_note_fails() {
    let mut packet = seeded_starter_boundary_states();
    let state = packet
        .boundary_states
        .iter_mut()
        .find(|s| s.disclosure().needs_non_durable_note)
        .expect("a non-durable state present");
    state.non_durable_note = String::new();
    assert!(packet
        .validate()
        .contains(&StarterBoundaryStateViolation::NonDurableNoteMissing));
}

#[test]
fn state_without_real_recovery_fails() {
    let mut packet = seeded_starter_boundary_states();
    packet.boundary_states[0].recovery_verbs = vec![StarterRecoveryVerb::RetryWhenAvailable];
    let violations = packet.validate();
    assert!(violations.contains(&StarterBoundaryStateViolation::RealRecoveryPathMissing));
    assert!(violations.contains(&StarterBoundaryStateViolation::ContinueWithoutStarterMissing));
}

#[test]
fn state_without_continue_without_starter_fails() {
    let mut packet = seeded_starter_boundary_states();
    packet.boundary_states[0]
        .recovery_verbs
        .retain(|v| *v != StarterRecoveryVerb::ContinueWithoutStarter);
    assert!(packet
        .validate()
        .contains(&StarterBoundaryStateViolation::ContinueWithoutStarterMissing));
}

#[test]
fn missing_mandatory_action_fails() {
    let mut packet = seeded_starter_boundary_states();
    packet.boundary_states[0]
        .state_actions
        .retain(|a| *a != StarterStateAction::ReviewTrustAndInstallSteps);
    assert!(packet
        .validate()
        .contains(&StarterBoundaryStateViolation::StateActionsIncomplete));
}

#[test]
fn each_hard_invariant_fails_when_set() {
    let mut packet = seeded_starter_boundary_states();
    packet.boundary_states[0].hides_starter_source_or_owner = true;
    assert!(packet
        .validate()
        .contains(&StarterBoundaryStateViolation::StarterSourceOrOwnerHidden));

    let mut packet = seeded_starter_boundary_states();
    packet.boundary_states[0].hides_mirror_offline_or_managed_dependency = true;
    assert!(packet
        .validate()
        .contains(&StarterBoundaryStateViolation::MirrorOfflineOrManagedDependencyHidden));

    let mut packet = seeded_starter_boundary_states();
    packet.boundary_states[0].performs_silent_trust_or_install = true;
    assert!(packet
        .validate()
        .contains(&StarterBoundaryStateViolation::SilentTrustOrInstallPerformed));

    let mut packet = seeded_starter_boundary_states();
    packet.boundary_states[0].omits_recovery_or_continue_without_starter_path = true;
    assert!(packet
        .validate()
        .contains(&StarterBoundaryStateViolation::RecoveryOrContinueWithoutStarterOmitted));

    let mut packet = seeded_starter_boundary_states();
    packet.boundary_states[0].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&StarterBoundaryStateViolation::AlternateStateLabelInvented));
}

#[test]
fn deep_link_action_without_resolvable_kind_fails() {
    let mut packet = seeded_starter_boundary_states();
    let state = packet
        .boundary_states
        .iter_mut()
        .find(|s| s.state_actions.contains(&StarterStateAction::OpenDeepLink))
        .expect("a state offering a deep link");
    state.deep_link_kind = DeepLinkKind::NoDeepLink;
    state.deep_link_ref = String::new();
    assert!(packet
        .validate()
        .contains(&StarterBoundaryStateViolation::DeepLinkUnresolved));
}

#[test]
fn missing_context_note_fails() {
    let mut packet = seeded_starter_boundary_states();
    packet.boundary_states[0].context_note = String::new();
    assert!(packet
        .validate()
        .contains(&StarterBoundaryStateViolation::ContextNoteMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_starter_boundary_states();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&StarterBoundaryStateViolation::MissingSourceContracts));
}

#[test]
fn boundary_review_incomplete_fails() {
    let mut packet = seeded_starter_boundary_states();
    packet.boundary_review.trust_step_disclosed_before_prompt = false;
    assert!(packet
        .validate()
        .contains(&StarterBoundaryStateViolation::BoundaryReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_starter_boundary_states();
    packet
        .consumer_projection
        .boundary_visible_before_trust_or_install = false;
    assert!(packet
        .validate()
        .contains(&StarterBoundaryStateViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_starter_boundary_states();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&StarterBoundaryStateViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_state() {
    let summary = seeded_starter_boundary_states().render_markdown_summary();
    for state in seeded_starter_boundary_states().boundary_states {
        assert!(summary.contains(&state.state_name));
    }
}

#[test]
fn matrix_csv_has_a_line_per_state() {
    let packet = seeded_starter_boundary_states();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.boundary_states.len());
    assert!(lines[0].starts_with("boundary_state,id,boundary_kind,"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_starter_boundary_state_export()
        .expect("checked starter boundary state export validates");
    assert_eq!(
        from_disk,
        seeded_starter_boundary_states(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn scenario_fixtures_validate_and_keep_full_coverage() {
    for packet in [
        seeded_starter_boundary_states_mirror_only_offline(),
        seeded_starter_boundary_states_sign_in_required(),
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
    let mirror: StarterBoundaryStateControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-starter-boundary-state-controls/mirror_only_offline.json"
    )))
    .expect("mirror-only-offline fixture parses");
    assert!(mirror.validate().is_empty());
    assert_eq!(mirror, seeded_starter_boundary_states_mirror_only_offline());

    let sign_in: StarterBoundaryStateControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-starter-boundary-state-controls/sign_in_required.json"
    )))
    .expect("sign-in-required fixture parses");
    assert!(sign_in.validate().is_empty());
    assert_eq!(sign_in, seeded_starter_boundary_states_sign_in_required());
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_starter_boundary_states().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
}
