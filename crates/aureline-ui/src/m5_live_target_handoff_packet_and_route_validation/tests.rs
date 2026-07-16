use super::*;

fn seed() -> M5LiveTargetHandoffPacket {
    seeded_m5_live_target_handoff()
}

fn violations_of(packet: &M5LiveTargetHandoffPacket) -> Vec<&'static str> {
    packet.validate().iter().map(|v| v.as_str()).collect()
}

#[test]
fn seed_validates_clean() {
    let packet = seed();
    assert!(packet.validate().is_empty(), "{:?}", violations_of(&packet));
    assert_eq!(packet.packet_id, M5_LIVE_TARGET_HANDOFF_PACKET_ID);
    assert_eq!(packet.record_kind, M5_LIVE_TARGET_HANDOFF_RECORD_KIND);
    assert_eq!(packet.handoff_bindings.len(), 15);
}

#[test]
fn every_object_class_is_handed_off_by_two_or_more_consumers() {
    let packet = seed();
    let mut object_consumers: BTreeMap<
        M5HistoricalReferenceObject,
        BTreeSet<M5HistoricalReferenceConsumerSurface>,
    > = BTreeMap::new();
    for binding in &packet.handoff_bindings {
        object_consumers
            .entry(binding.object_class)
            .or_default()
            .insert(binding.consumer);
    }
    assert_eq!(
        object_consumers.len(),
        5,
        "all five object classes handed off"
    );
    for (object_class, consumers) in &object_consumers {
        assert!(
            consumers.len() >= 2,
            "object class {} only handed off by {} consumers",
            object_class.as_str(),
            consumers.len()
        );
    }
}

#[test]
fn every_consumer_surface_and_outcome_is_exercised() {
    let packet = seed();
    let consumers: BTreeSet<_> = packet.handoff_bindings.iter().map(|b| b.consumer).collect();
    for consumer in M5HistoricalReferenceConsumerSurface::ALL {
        assert!(
            consumers.contains(&consumer),
            "consumer {} missing",
            consumer.as_str()
        );
    }
    let outcomes: BTreeSet<_> = packet.handoff_bindings.iter().map(|b| b.outcome).collect();
    for outcome in HandoffOutcome::ALL {
        assert!(
            outcomes.contains(&outcome),
            "outcome {} missing",
            outcome.as_str()
        );
    }
}

#[test]
fn same_profile_carries_identical_grammar_across_surfaces() {
    let packet = seed();
    let mut profile_grammar: BTreeMap<&str, &HandoffHistoricalGrammar> = BTreeMap::new();
    for binding in &packet.handoff_bindings {
        match profile_grammar.get(binding.snapshot_profile_id.as_str()) {
            None => {
                profile_grammar.insert(
                    binding.snapshot_profile_id.as_str(),
                    &binding.historical_grammar,
                );
            }
            Some(existing) => assert_eq!(
                **existing, binding.historical_grammar,
                "grammar drift on {}",
                binding.snapshot_profile_id
            ),
        }
    }
    assert_eq!(profile_grammar.len(), 5);
}

#[test]
fn every_historical_role_word_is_a_frozen_role_token() {
    let packet = seed();
    for binding in &packet.handoff_bindings {
        assert!(
            binding
                .historical_grammar
                .historical_role_word_in_vocabulary(),
            "role word `{}` on {} is not a frozen role token",
            binding.historical_grammar.historical_role_word,
            binding.binding_id
        );
        assert!(binding.historical_grammar.all_present());
        assert!(binding
            .historical_grammar
            .mutation_blocked_posture_satisfied());
    }
}

#[test]
fn actions_are_analysis_only_and_open_live_matches_cleared_outcome() {
    let packet = seed();
    for binding in &packet.handoff_bindings {
        assert!(binding.has_analysis_only_base_actions());
        assert!(binding.action_set_is_analysis_only());
        assert!(binding.open_live_action_matches_outcome());
        // No apply / sync action exists in the closed action enum.
        assert!(!binding
            .allowed_actions
            .iter()
            .any(|a| a.as_str().contains("apply") || a.as_str().contains("sync")));
        // Only a cleared handoff offers open-current-live-object.
        let offers = binding
            .allowed_actions
            .contains(&HandoffAction::OpenCurrentLiveObject);
        let expected = binding.outcome == HandoffOutcome::HandoffCleared;
        assert_eq!(offers, expected, "on {}", binding.binding_id);
    }
}

#[test]
fn cleared_handoffs_clear_every_precondition_and_blocked_ones_fail_one() {
    let packet = seed();
    for binding in &packet.handoff_bindings {
        let cleared = binding.handoff_request.precondition_check.all_cleared();
        assert_eq!(
            cleared,
            binding.outcome == HandoffOutcome::HandoffCleared,
            "precondition / outcome mismatch on {}",
            binding.binding_id
        );
        if let Some(note) = &binding.blocker_note {
            assert!(
                note.reason
                    .supported_by(&binding.handoff_request.precondition_check),
                "blocker {} not supported by a failed precondition on {}",
                note.reason.as_str(),
                binding.binding_id
            );
        }
    }
}

#[test]
fn no_handoff_widens_authority_beyond_direct_open() {
    let packet = seed();
    for binding in &packet.handoff_bindings {
        assert!(
            binding.handoff_request.authority_not_widened(),
            "handoff {} widens authority ({} > {})",
            binding.binding_id,
            binding.handoff_request.requested_authority_class.as_str(),
            binding.handoff_request.direct_open_authority_class.as_str()
        );
        assert!(!binding.widens_authority_beyond_direct_open);
    }
}

#[test]
fn accessibility_state_is_discoverable_for_every_binding() {
    let packet = seed();
    for binding in &packet.handoff_bindings {
        assert!(
            binding.accessibility_state_discoverable(),
            "binding {} is not keyboard/screen-reader discoverable",
            binding.binding_id
        );
    }
}

#[test]
fn blocked_bindings_disclose_and_cleared_bindings_do_not() {
    let packet = seed();
    for binding in &packet.handoff_bindings {
        let disclosure = binding.disclosure();
        if binding.is_blocked() {
            assert_eq!(
                binding.parity_state,
                HandoffParityState::HandoffBlockedDisclosed
            );
            let note = binding
                .blocker_note
                .as_ref()
                .expect("blocked binding carries a blocker note");
            assert!(binding
                .outcome
                .allowed_blocker_reasons()
                .contains(&note.reason));
            assert_eq!(Some(note.next_action), disclosure.blocker_next_action);
            assert_eq!(note.fallback_behavior, disclosure.fallback_behavior);
            assert!(!note.explanation.trim().is_empty());
            assert!(!note.preserved_historical_note.trim().is_empty());
            assert!(!note.next_action_label.trim().is_empty());
        } else {
            assert_eq!(
                binding.parity_state,
                HandoffParityState::HandoffClearedCompleted
            );
            assert!(binding.blocker_note.is_none());
        }
    }
}

#[test]
fn support_and_export_bindings_point_at_canonical_contracts() {
    let packet = seed();
    for binding in &packet.handoff_bindings {
        if consumer_must_reference_canonical(binding.consumer) {
            assert!(
                binding.points_at_canonical_contracts(),
                "binding {} must point at canonical contracts",
                binding.binding_id
            );
        }
    }
}

#[test]
fn disclosure_resolver_matches_outcome() {
    let cleared = resolve_handoff_render_disclosure(HandoffOutcome::HandoffCleared);
    assert!(!cleared.needs_blocker_note);
    assert!(cleared.offers_open_live_target);
    assert!(cleared.requires_cleared_preconditions);
    assert_eq!(
        cleared.fallback_behavior,
        HandoffFallbackBehavior::OpenValidatedLiveTarget
    );

    let needs = resolve_handoff_render_disclosure(HandoffOutcome::BlockedNeedsPrerequisite);
    assert!(needs.needs_blocker_note);
    assert!(!needs.offers_open_live_target);
    assert_eq!(
        needs.blocker_next_action,
        Some(HandoffBlockerNextAction::SatisfyPrerequisiteThenRetry)
    );
    assert_eq!(
        needs.fallback_behavior,
        HandoffFallbackBehavior::OfferPrerequisiteThenRetry
    );

    let target = resolve_handoff_render_disclosure(HandoffOutcome::BlockedTargetUnavailable);
    assert!(target.needs_blocker_note);
    assert!(!target.offers_open_live_target);
    assert_eq!(
        target.blocker_next_action,
        Some(HandoffBlockerNextAction::InspectHistoricalPacketOnly)
    );
    assert_eq!(
        target.fallback_behavior,
        HandoffFallbackBehavior::MetadataOnlyExit
    );

    let policy = resolve_handoff_render_disclosure(HandoffOutcome::BlockedByPolicy);
    assert!(policy.needs_blocker_note);
    assert!(!policy.offers_open_live_target);
    assert_eq!(
        policy.fallback_behavior,
        HandoffFallbackBehavior::MetadataOnlyExit
    );
}

#[test]
fn reviewed_authority_handoff_is_seeded_and_complete() {
    let packet = seed();
    let with_handoff: Vec<_> = packet
        .handoff_bindings
        .iter()
        .filter(|b| b.reviewed_authority_handoff.is_some())
        .collect();
    assert!(
        !with_handoff.is_empty(),
        "at least one binding demonstrates the reviewed-authority-handoff takeover"
    );
    for binding in with_handoff {
        let handoff = binding.reviewed_authority_handoff.as_ref().unwrap();
        assert!(!handoff.reviewed_path_id.trim().is_empty());
        assert!(!handoff.reviewed_path_label.trim().is_empty());
    }
}

#[test]
fn grammar_drift_is_rejected() {
    let mut packet = seed();
    let target = packet
        .handoff_bindings
        .iter()
        .position(|b| b.binding_id == "lth-retirement-shell")
        .unwrap();
    packet.handoff_bindings[target]
        .historical_grammar
        .historical_role_word = "capture_time_attribution".to_owned();
    assert!(violations_of(&packet).contains(&"handoff_grammar_drift_across_surfaces"));
}

#[test]
fn historical_role_word_outside_vocabulary_is_rejected() {
    let mut packet = seed();
    packet.handoff_bindings[0]
        .historical_grammar
        .historical_role_word = "totally_made_up".to_owned();
    assert!(violations_of(&packet).contains(&"historical_role_word_outside_vocabulary"));
}

#[test]
fn dropped_mutation_blocked_posture_on_gate_role_is_rejected() {
    let mut packet = seed();
    // lth-retirement-release carries the snapshot_labeling gate role, which must always keep a real
    // mutation-blocked posture.
    let target = packet
        .handoff_bindings
        .iter()
        .position(|b| b.binding_id == "lth-retirement-release")
        .unwrap();
    packet.handoff_bindings[target]
        .historical_grammar
        .mutation_blocked_posture_word = "editable".to_owned();
    assert!(violations_of(&packet).contains(&"mutation_blocked_posture_missing_for_gate_role"));
}

#[test]
fn precondition_outcome_mismatch_is_rejected() {
    let mut packet = seed();
    // Break a cleared handoff's precondition without changing the outcome.
    let target = packet
        .handoff_bindings
        .iter()
        .position(|b| b.outcome == HandoffOutcome::HandoffCleared)
        .unwrap();
    packet.handoff_bindings[target]
        .handoff_request
        .precondition_check
        .route_available = false;
    assert!(violations_of(&packet).contains(&"precondition_outcome_mismatch"));
}

#[test]
fn widened_authority_is_rejected() {
    let mut packet = seed();
    packet.handoff_bindings[0]
        .handoff_request
        .requested_authority_class = LiveTargetAuthorityClass::ElevatedAdmin;
    packet.handoff_bindings[0]
        .handoff_request
        .direct_open_authority_class = LiveTargetAuthorityClass::ReadOnlyInspect;
    let tokens = violations_of(&packet);
    assert!(tokens.contains(&"authority_widened_beyond_direct_open"));
    // The declared no-widen guardrail flag must also be recomputed, or it is flagged inconsistent.
    assert!(tokens.contains(&"authority_widen_flag_inconsistent"));
}

#[test]
fn widen_guardrail_flag_is_rejected() {
    let mut packet = seed();
    packet.handoff_bindings[0].widens_authority_beyond_direct_open = true;
    let tokens = violations_of(&packet);
    assert!(tokens.contains(&"widens_authority_beyond_direct_open"));
    assert!(tokens.contains(&"authority_widen_flag_inconsistent"));
}

#[test]
fn fallback_mismatch_is_rejected() {
    let mut packet = seed();
    let target = packet
        .handoff_bindings
        .iter()
        .position(|b| b.outcome == HandoffOutcome::HandoffCleared)
        .unwrap();
    packet.handoff_bindings[target]
        .handoff_request
        .fallback_behavior = HandoffFallbackBehavior::MetadataOnlyExit;
    assert!(violations_of(&packet).contains(&"fallback_behavior_mismatch"));
}

#[test]
fn historical_side_not_mutation_blocked_is_rejected() {
    let mut packet = seed();
    packet.handoff_bindings[0].historical_side_mutation_blocked = false;
    assert!(violations_of(&packet).contains(&"historical_side_not_mutation_blocked"));
}

#[test]
fn open_live_action_mismatch_is_rejected() {
    let mut packet = seed();
    // Add an open-current-live-object action to a blocked binding, which must not offer it.
    let target = packet
        .handoff_bindings
        .iter()
        .position(|b| b.is_blocked())
        .unwrap();
    packet.handoff_bindings[target]
        .allowed_actions
        .push(HandoffAction::OpenCurrentLiveObject);
    assert!(violations_of(&packet).contains(&"open_live_action_outcome_mismatch"));
}

#[test]
fn missing_analysis_only_base_action_is_rejected() {
    let mut packet = seed();
    packet.handoff_bindings[0]
        .allowed_actions
        .retain(|a| *a != HandoffAction::ExportHandoffPacket);
    assert!(violations_of(&packet).contains(&"analysis_only_base_actions_missing"));
}

#[test]
fn undiscoverable_accessibility_state_is_rejected() {
    let mut packet = seed();
    packet.handoff_bindings[0].accessibility_routes =
        vec![M5HistoricalReferenceAccessibilityRoute::HighZoomReflow];
    assert!(violations_of(&packet).contains(&"accessibility_state_undiscoverable"));
}

#[test]
fn reuse_below_two_consumers_is_rejected() {
    let mut packet = seed();
    let mut kept_one = false;
    packet.handoff_bindings.retain(|b| {
        if b.object_class == M5HistoricalReferenceObject::ImportedOfflineRouteEvidence {
            if kept_one {
                return false;
            }
            kept_one = true;
        }
        true
    });
    assert!(violations_of(&packet).contains(&"object_class_reuse_unproven"));
}

#[test]
fn missing_canonical_reference_on_export_is_rejected() {
    let mut packet = seed();
    let target = packet
        .handoff_bindings
        .iter()
        .position(|b| consumer_must_reference_canonical(b.consumer))
        .unwrap();
    packet.handoff_bindings[target].source_contract_refs =
        vec![M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF.to_owned()];
    assert!(violations_of(&packet).contains(&"support_export_reference_missing"));
}

#[test]
fn missing_blocker_note_is_rejected() {
    let mut packet = seed();
    let target = packet
        .handoff_bindings
        .iter()
        .position(|b| b.is_blocked())
        .unwrap();
    packet.handoff_bindings[target].blocker_note = None;
    assert!(violations_of(&packet).contains(&"blocker_note_missing"));
}

#[test]
fn unexpected_blocker_note_on_cleared_binding_is_rejected() {
    let mut packet = seed();
    let target = packet
        .handoff_bindings
        .iter()
        .position(|b| !b.is_blocked())
        .unwrap();
    packet.handoff_bindings[target].blocker_note = Some(HandoffBlockerNote {
        reason: HandoffBlockerReason::TargetDoesNotExist,
        explanation: "x".to_owned(),
        preserved_historical_note: "x".to_owned(),
        fallback_behavior: HandoffFallbackBehavior::MetadataOnlyExit,
        next_action: HandoffBlockerNextAction::InspectHistoricalPacketOnly,
        next_action_label: "x".to_owned(),
    });
    assert!(violations_of(&packet).contains(&"unexpected_blocker_note"));
}

#[test]
fn blocker_reason_not_allowed_for_outcome_is_rejected() {
    let mut packet = seed();
    // Find a needs-prerequisite binding and give it a reason only allowed for a target-unavailable outcome.
    let target = packet
        .handoff_bindings
        .iter()
        .position(|b| b.outcome == HandoffOutcome::BlockedNeedsPrerequisite)
        .unwrap();
    if let Some(note) = packet.handoff_bindings[target].blocker_note.as_mut() {
        note.reason = HandoffBlockerReason::TargetDoesNotExist;
    }
    let tokens = violations_of(&packet);
    assert!(tokens.contains(&"blocker_reason_not_allowed_for_outcome"));
}

#[test]
fn blocker_reason_not_supported_by_precondition_is_rejected() {
    let mut packet = seed();
    // A needs-prerequisite binding whose precondition fails route, but claims a trust blocker.
    let target = packet
        .handoff_bindings
        .iter()
        .position(|b| {
            b.outcome == HandoffOutcome::BlockedNeedsPrerequisite
                && b.blocker_note.as_ref().map(|n| n.reason)
                    == Some(HandoffBlockerReason::RouteUnavailable)
        })
        .unwrap();
    if let Some(note) = packet.handoff_bindings[target].blocker_note.as_mut() {
        // TrustPostureInsufficient is allowed for the outcome, but the precondition only fails route.
        note.reason = HandoffBlockerReason::TrustPostureInsufficient;
    }
    assert!(violations_of(&packet).contains(&"blocker_reason_not_supported_by_precondition"));
}

#[test]
fn each_guardrail_is_enforced() {
    let cases: [(fn(&mut LiveTargetHandoffBinding), &str); 4] = [
        (
            |b| b.reopens_live_target_without_validating_identity_trust_route_and_authority = true,
            "reopens_live_target_without_validating_identity_trust_route_and_authority",
        ),
        (
            |b| b.dead_ends_when_target_unavailable = true,
            "dead_ends_when_target_unavailable",
        ),
        (
            |b| b.leaks_secret_or_ambient_credential = true,
            "leaks_secret_or_ambient_credential",
        ),
        (
            |b| b.presents_snapshot_as_current_live_object = true,
            "presents_snapshot_as_current_live_object",
        ),
    ];
    for (mutate, token) in cases {
        let mut packet = seed();
        mutate(&mut packet.handoff_bindings[0]);
        assert!(violations_of(&packet).contains(&token), "expected {token}");
    }
}

#[test]
fn object_class_coverage_gap_is_rejected() {
    let mut packet = seed();
    packet
        .handoff_bindings
        .retain(|b| b.object_class != M5HistoricalReferenceObject::ReviewIncidentSnapshot);
    assert!(violations_of(&packet).contains(&"object_class_coverage_missing"));
}

#[test]
fn outcome_coverage_gap_is_rejected() {
    let mut packet = seed();
    // Drop every needs-prerequisite handoff, leaving that outcome uncovered.
    packet
        .handoff_bindings
        .retain(|b| b.outcome != HandoffOutcome::BlockedNeedsPrerequisite);
    assert!(violations_of(&packet).contains(&"outcome_coverage_missing"));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = seed();
    packet
        .source_contract_refs
        .retain(|r| r != M5_LIVE_TARGET_HANDOFF_DOC_REF);
    assert!(violations_of(&packet).contains(&"missing_source_contracts"));
}

#[test]
fn reviewed_authority_handoff_incomplete_is_rejected() {
    let mut packet = seed();
    let target = packet
        .handoff_bindings
        .iter()
        .position(|b| b.reviewed_authority_handoff.is_some())
        .unwrap();
    packet.handoff_bindings[target].reviewed_authority_handoff = Some(ReviewedAuthorityHandoff {
        reviewed_path_id: String::new(),
        reviewed_path_label: String::new(),
    });
    assert!(violations_of(&packet).contains(&"reviewed_authority_handoff_incomplete"));
}

#[test]
fn export_json_is_boundary_safe() {
    let json = seed().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_binding() {
    let packet = seed();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.handoff_bindings.len());
    assert!(lines[0].starts_with(
        "object_class,consumer,outcome,route_class,trust_posture,requested_authority,direct_open_authority,parity_state"
    ));
}

#[test]
fn markdown_summary_lists_every_binding() {
    let packet = seed();
    let summary = packet.render_markdown_summary();
    for binding in &packet.handoff_bindings {
        assert!(summary.contains(&binding.binding_id));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_live_target_handoff_export()
        .expect("checked M5 live-target-handoff export validates");
    assert_eq!(from_disk.packet_id, M5_LIVE_TARGET_HANDOFF_PACKET_ID);
    assert_eq!(
        from_disk,
        seed(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_fixture_variants_validate_and_keep_coverage() {
    let blocked = seeded_m5_live_target_handoff_blocked_target_narrowed();
    assert!(
        blocked.validate().is_empty(),
        "{:?}",
        violations_of(&blocked)
    );
    assert_eq!(blocked.handoff_bindings.len(), 15);

    let needs = seeded_m5_live_target_handoff_needs_prerequisite_narrowed();
    assert!(needs.validate().is_empty(), "{:?}", violations_of(&needs));
    assert_eq!(needs.handoff_bindings.len(), 15);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_builders() {
    let blocked: M5LiveTargetHandoffPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/recovery/m5-live-target-handoff/blocked_target_narrowed.json"
    )))
    .expect("blocked-target fixture parses");
    assert!(blocked.validate().is_empty());
    assert_eq!(
        blocked,
        seeded_m5_live_target_handoff_blocked_target_narrowed()
    );

    let needs: M5LiveTargetHandoffPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/recovery/m5-live-target-handoff/needs_prerequisite_narrowed.json"
    )))
    .expect("needs-prerequisite fixture parses");
    assert!(needs.validate().is_empty());
    assert_eq!(
        needs,
        seeded_m5_live_target_handoff_needs_prerequisite_narrowed()
    );
}
