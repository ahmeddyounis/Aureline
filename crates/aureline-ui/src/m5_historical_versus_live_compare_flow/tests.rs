use super::*;

fn seed() -> M5HistoricalVersusLiveCompareFlowPacket {
    seeded_m5_historical_versus_live_compare_flow()
}

fn violations_of(packet: &M5HistoricalVersusLiveCompareFlowPacket) -> Vec<&'static str> {
    packet.validate().iter().map(|v| v.as_str()).collect()
}

#[test]
fn seed_validates_clean() {
    let packet = seed();
    assert!(packet.validate().is_empty(), "{:?}", violations_of(&packet));
    assert_eq!(
        packet.packet_id,
        M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_PACKET_ID
    );
    assert_eq!(
        packet.record_kind,
        M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_RECORD_KIND
    );
    assert_eq!(packet.compare_bindings.len(), 15);
}

#[test]
fn every_object_class_is_paired_by_two_or_more_consumers() {
    let packet = seed();
    let mut object_consumers: BTreeMap<
        M5HistoricalReferenceObject,
        BTreeSet<M5HistoricalReferenceConsumerSurface>,
    > = BTreeMap::new();
    for binding in &packet.compare_bindings {
        object_consumers
            .entry(binding.object_class)
            .or_default()
            .insert(binding.consumer);
    }
    assert_eq!(object_consumers.len(), 5, "all five object classes paired");
    for (object_class, consumers) in &object_consumers {
        assert!(
            consumers.len() >= 2,
            "object class {} only paired by {} consumers",
            object_class.as_str(),
            consumers.len()
        );
    }
}

#[test]
fn every_consumer_surface_and_outcome_is_exercised() {
    let packet = seed();
    let consumers: BTreeSet<_> = packet.compare_bindings.iter().map(|b| b.consumer).collect();
    for consumer in M5HistoricalReferenceConsumerSurface::ALL {
        assert!(
            consumers.contains(&consumer),
            "consumer {} missing",
            consumer.as_str()
        );
    }
    let outcomes: BTreeSet<_> = packet.compare_bindings.iter().map(|b| b.outcome).collect();
    for outcome in CompareOutcome::ALL {
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
    let mut profile_grammar: BTreeMap<&str, &CompareHistoricalGrammar> = BTreeMap::new();
    for binding in &packet.compare_bindings {
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
    for binding in &packet.compare_bindings {
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
fn actions_are_analysis_only_and_open_live_matches_outcome() {
    let packet = seed();
    for binding in &packet.compare_bindings {
        assert!(binding.has_analysis_only_base_actions());
        assert!(binding.action_set_is_analysis_only());
        assert!(binding.open_live_action_matches_outcome());
        // No apply / sync action exists in the closed action enum.
        assert!(!binding
            .allowed_actions
            .iter()
            .any(|a| a.as_str().contains("apply") || a.as_str().contains("sync")));
        // A confirmed / approximate pairing offers open-current-live-object; a missing / policy-blocked
        // pairing never does.
        let offers = binding
            .allowed_actions
            .contains(&CompareAction::OpenCurrentLiveObject);
        let expected = matches!(
            binding.outcome,
            CompareOutcome::LiveTargetPaired | CompareOutcome::ApproximatePairing
        );
        assert_eq!(offers, expected, "on {}", binding.binding_id);
    }
}

#[test]
fn identity_and_freshness_are_labeled_and_match_outcome() {
    let packet = seed();
    for binding in &packet.compare_bindings {
        let disclosure = binding.disclosure();
        assert_eq!(
            binding.identity_match_state,
            disclosure.identity_match_state
        );
        assert!(!binding.drift_summary.trim().is_empty());
        let verifiable =
            binding.freshness_drift_state != CompareFreshnessDriftState::FreshnessUnverifiable;
        assert_eq!(
            verifiable, disclosure.requires_live_freshness,
            "freshness label mismatch on {}",
            binding.binding_id
        );
    }
}

#[test]
fn accessibility_state_is_discoverable_for_every_binding() {
    let packet = seed();
    for binding in &packet.compare_bindings {
        assert!(
            binding.accessibility_state_discoverable(),
            "binding {} is not keyboard/screen-reader discoverable",
            binding.binding_id
        );
    }
}

#[test]
fn narrowed_bindings_disclose_and_full_bindings_do_not() {
    let packet = seed();
    for binding in &packet.compare_bindings {
        let disclosure = binding.disclosure();
        if binding.is_narrowed() {
            assert_eq!(
                binding.parity_state,
                CompareParityState::PairNarrowedDisclosed
            );
            let note = binding
                .mismatch_note
                .as_ref()
                .expect("narrowed binding carries a mismatch note");
            assert!(binding
                .outcome
                .allowed_mismatch_reasons()
                .contains(&note.reason));
            assert_eq!(Some(note.next_action), disclosure.narrow_next_action);
            assert!(!note.explanation.trim().is_empty());
            assert!(!note.preserved_grammar_note.trim().is_empty());
            assert!(!note.next_action_label.trim().is_empty());
        } else {
            assert_eq!(binding.parity_state, CompareParityState::PairPreserved);
            assert!(binding.mismatch_note.is_none());
        }
    }
}

#[test]
fn support_and_export_bindings_point_at_canonical_contracts() {
    let packet = seed();
    for binding in &packet.compare_bindings {
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
    let paired = resolve_compare_render_disclosure(CompareOutcome::LiveTargetPaired);
    assert!(!paired.needs_mismatch_note);
    assert!(paired.offers_open_live_target);
    assert_eq!(
        paired.identity_match_state,
        CompareIdentityMatchState::SameObjectIdentity
    );
    assert!(paired.requires_live_freshness);

    let approximate = resolve_compare_render_disclosure(CompareOutcome::ApproximatePairing);
    assert!(approximate.needs_mismatch_note);
    assert!(approximate.offers_open_live_target);
    assert_eq!(
        approximate.identity_match_state,
        CompareIdentityMatchState::ApproximateIdentity
    );
    assert_eq!(
        approximate.narrow_next_action,
        Some(CompareNarrowNextAction::OpenApproximatePairingDetail)
    );

    let missing = resolve_compare_render_disclosure(CompareOutcome::LiveTargetMissing);
    assert!(missing.needs_mismatch_note);
    assert!(!missing.offers_open_live_target);
    assert!(!missing.requires_live_freshness);
    assert_eq!(
        missing.identity_match_state,
        CompareIdentityMatchState::IdentityUnverifiable
    );

    let policy = resolve_compare_render_disclosure(CompareOutcome::PolicyBlockedPairing);
    assert!(policy.needs_mismatch_note);
    assert!(!policy.offers_open_live_target);
    assert_eq!(
        policy.narrow_next_action,
        Some(CompareNarrowNextAction::InspectHistoricalPacketOnly)
    );
}

#[test]
fn reviewed_mutation_handoff_is_seeded_and_complete() {
    let packet = seed();
    let with_handoff: Vec<_> = packet
        .compare_bindings
        .iter()
        .filter(|b| b.reviewed_mutation_handoff.is_some())
        .collect();
    assert!(
        !with_handoff.is_empty(),
        "at least one binding demonstrates the reviewed-mutation-handoff takeover"
    );
    for binding in with_handoff {
        let handoff = binding.reviewed_mutation_handoff.as_ref().unwrap();
        assert!(!handoff.reviewed_path_id.trim().is_empty());
        assert!(!handoff.reviewed_path_label.trim().is_empty());
        // The compare flow itself still never implies apply / sync is safe.
        assert!(!binding.implies_apply_or_sync_historical_snapshot_is_safe);
    }
}

#[test]
fn grammar_drift_is_rejected() {
    let mut packet = seed();
    let target = packet
        .compare_bindings
        .iter()
        .position(|b| b.binding_id == "hvlc-retirement-shell")
        .unwrap();
    packet.compare_bindings[target]
        .historical_grammar
        .historical_role_word = "capture_time_attribution".to_owned();
    assert!(violations_of(&packet).contains(&"compare_grammar_drift_across_surfaces"));
}

#[test]
fn historical_role_word_outside_vocabulary_is_rejected() {
    let mut packet = seed();
    packet.compare_bindings[0]
        .historical_grammar
        .historical_role_word = "totally_made_up".to_owned();
    assert!(violations_of(&packet).contains(&"historical_role_word_outside_vocabulary"));
}

#[test]
fn dropped_mutation_blocked_posture_on_gate_role_is_rejected() {
    let mut packet = seed();
    // hvlc-retirement-release carries the snapshot_labeling gate role, which must always keep a real
    // mutation-blocked posture.
    let target = packet
        .compare_bindings
        .iter()
        .position(|b| b.binding_id == "hvlc-retirement-release")
        .unwrap();
    packet.compare_bindings[target]
        .historical_grammar
        .mutation_blocked_posture_word = "editable".to_owned();
    assert!(violations_of(&packet).contains(&"mutation_blocked_posture_missing_for_gate_role"));
}

#[test]
fn implies_apply_or_sync_safe_is_rejected() {
    let mut packet = seed();
    packet.compare_bindings[0].implies_apply_or_sync_historical_snapshot_is_safe = true;
    assert!(violations_of(&packet).contains(&"implies_apply_or_sync_historical_snapshot_is_safe"));
}

#[test]
fn historical_side_not_mutation_blocked_is_rejected() {
    let mut packet = seed();
    packet.compare_bindings[0].historical_side_mutation_blocked = false;
    assert!(violations_of(&packet).contains(&"historical_side_not_mutation_blocked"));
}

#[test]
fn open_live_action_mismatch_is_rejected() {
    let mut packet = seed();
    // Add an open-current-live-object action to a narrowed (missing / policy) binding, which must not offer it.
    let target = packet
        .compare_bindings
        .iter()
        .position(|b| !b.disclosure().offers_open_live_target)
        .unwrap();
    packet.compare_bindings[target]
        .allowed_actions
        .push(CompareAction::OpenCurrentLiveObject);
    assert!(violations_of(&packet).contains(&"open_live_action_outcome_mismatch"));
}

#[test]
fn missing_analysis_only_base_action_is_rejected() {
    let mut packet = seed();
    packet.compare_bindings[0]
        .allowed_actions
        .retain(|a| *a != CompareAction::ExportComparison);
    assert!(violations_of(&packet).contains(&"analysis_only_base_actions_missing"));
}

#[test]
fn undiscoverable_accessibility_state_is_rejected() {
    let mut packet = seed();
    packet.compare_bindings[0].accessibility_routes =
        vec![M5HistoricalReferenceAccessibilityRoute::HighZoomReflow];
    assert!(violations_of(&packet).contains(&"accessibility_state_undiscoverable"));
}

#[test]
fn reuse_below_two_consumers_is_rejected() {
    let mut packet = seed();
    let mut kept_one = false;
    packet.compare_bindings.retain(|b| {
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
        .compare_bindings
        .iter()
        .position(|b| consumer_must_reference_canonical(b.consumer))
        .unwrap();
    packet.compare_bindings[target].source_contract_refs =
        vec![M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF.to_owned()];
    assert!(violations_of(&packet).contains(&"support_export_reference_missing"));
}

#[test]
fn missing_mismatch_note_is_rejected() {
    let mut packet = seed();
    let target = packet
        .compare_bindings
        .iter()
        .position(|b| b.is_narrowed())
        .unwrap();
    packet.compare_bindings[target].mismatch_note = None;
    assert!(violations_of(&packet).contains(&"mismatch_note_missing"));
}

#[test]
fn unexpected_mismatch_note_on_full_binding_is_rejected() {
    let mut packet = seed();
    let target = packet
        .compare_bindings
        .iter()
        .position(|b| !b.is_narrowed())
        .unwrap();
    packet.compare_bindings[target].mismatch_note = Some(CompareMismatchNote {
        reason: CompareMismatchReason::MissingLiveTarget,
        explanation: "x".to_owned(),
        preserved_grammar_note: "x".to_owned(),
        next_action: CompareNarrowNextAction::InspectHistoricalPacketOnly,
        next_action_label: "x".to_owned(),
    });
    assert!(violations_of(&packet).contains(&"unexpected_mismatch_note"));
}

#[test]
fn mismatch_reason_not_allowed_for_outcome_is_rejected() {
    let mut packet = seed();
    // Find an approximate-pairing binding and give it a reason only allowed for a missing target.
    let target = packet
        .compare_bindings
        .iter()
        .position(|b| b.outcome == CompareOutcome::ApproximatePairing)
        .unwrap();
    if let Some(note) = packet.compare_bindings[target].mismatch_note.as_mut() {
        note.reason = CompareMismatchReason::MissingLiveTarget;
    }
    assert!(violations_of(&packet).contains(&"mismatch_reason_not_allowed_for_outcome"));
}

#[test]
fn each_guardrail_is_enforced() {
    let cases: [(fn(&mut CompareFlowBinding), &str); 5] = [
        (
            |b| b.collapses_snapshot_and_live_into_one_ambiguous_view = true,
            "collapses_snapshot_and_live_into_one_ambiguous_view",
        ),
        (
            |b| b.implies_apply_or_sync_historical_snapshot_is_safe = true,
            "implies_apply_or_sync_historical_snapshot_is_safe",
        ),
        (
            |b| b.reopens_live_target_without_validating_identity_trust_route_and_authority = true,
            "reopens_live_target_without_validating_identity_trust_route_and_authority",
        ),
        (
            |b| b.dead_ends_on_missing_or_mismatched_target = true,
            "dead_ends_on_missing_or_mismatched_target",
        ),
        (
            |b| b.leaves_historical_side_mutable_or_unlabeled = true,
            "leaves_historical_side_mutable_or_unlabeled",
        ),
    ];
    for (mutate, token) in cases {
        let mut packet = seed();
        mutate(&mut packet.compare_bindings[0]);
        assert!(violations_of(&packet).contains(&token), "expected {token}");
    }
}

#[test]
fn object_class_coverage_gap_is_rejected() {
    let mut packet = seed();
    packet
        .compare_bindings
        .retain(|b| b.object_class != M5HistoricalReferenceObject::ReviewIncidentSnapshot);
    assert!(violations_of(&packet).contains(&"object_class_coverage_missing"));
}

#[test]
fn outcome_coverage_gap_is_rejected() {
    let mut packet = seed();
    // Drop every approximate pairing, leaving that outcome uncovered.
    packet
        .compare_bindings
        .retain(|b| b.outcome != CompareOutcome::ApproximatePairing);
    assert!(violations_of(&packet).contains(&"outcome_coverage_missing"));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = seed();
    packet
        .source_contract_refs
        .retain(|r| r != M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_DOC_REF);
    assert!(violations_of(&packet).contains(&"missing_source_contracts"));
}

#[test]
fn reviewed_mutation_handoff_incomplete_is_rejected() {
    let mut packet = seed();
    let target = packet
        .compare_bindings
        .iter()
        .position(|b| b.reviewed_mutation_handoff.is_some())
        .unwrap();
    packet.compare_bindings[target].reviewed_mutation_handoff = Some(ReviewedMutationHandoff {
        reviewed_path_id: String::new(),
        reviewed_path_label: String::new(),
    });
    assert!(violations_of(&packet).contains(&"reviewed_mutation_handoff_incomplete"));
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
    assert_eq!(lines.len(), 1 + packet.compare_bindings.len());
    assert!(lines[0].starts_with(
        "object_class,consumer,outcome,identity_match_state,freshness_drift_state,historical_role_word,parity_state"
    ));
}

#[test]
fn markdown_summary_lists_every_profile() {
    let packet = seed();
    let summary = packet.render_markdown_summary();
    for binding in &packet.compare_bindings {
        assert!(summary.contains(&binding.binding_id));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_historical_versus_live_compare_flow_export()
        .expect("checked M5 compare-flow export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seed(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_fixture_variants_validate_and_keep_coverage() {
    let missing = seeded_m5_historical_versus_live_compare_flow_missing_target_narrowed();
    assert!(
        missing.validate().is_empty(),
        "{:?}",
        violations_of(&missing)
    );
    assert_eq!(missing.compare_bindings.len(), 15);

    let policy = seeded_m5_historical_versus_live_compare_flow_policy_blocked_narrowed();
    assert!(policy.validate().is_empty(), "{:?}", violations_of(&policy));
    assert_eq!(policy.compare_bindings.len(), 15);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_builders() {
    let missing: M5HistoricalVersusLiveCompareFlowPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m5-historical-versus-live-compare/missing_target_narrowed.json"
        )))
        .expect("missing-target fixture parses");
    assert!(missing.validate().is_empty());
    assert_eq!(
        missing,
        seeded_m5_historical_versus_live_compare_flow_missing_target_narrowed()
    );

    let policy: M5HistoricalVersusLiveCompareFlowPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m5-historical-versus-live-compare/policy_blocked_narrowed.json"
        )))
        .expect("policy-blocked fixture parses");
    assert!(policy.validate().is_empty());
    assert_eq!(
        policy,
        seeded_m5_historical_versus_live_compare_flow_policy_blocked_narrowed()
    );
}
