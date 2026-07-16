use super::*;

fn seed() -> M5CrossActorConstrainedWriteEnforcementPacket {
    seeded_m5_cross_actor_constrained_write_enforcement()
}

fn violations_of(packet: &M5CrossActorConstrainedWriteEnforcementPacket) -> Vec<&'static str> {
    packet.validate().iter().map(|v| v.as_str()).collect()
}

#[test]
fn seed_validates_clean() {
    let packet = seed();
    assert!(packet.validate().is_empty(), "{:?}", violations_of(&packet));
    assert_eq!(
        packet.packet_id,
        M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_PACKET_ID
    );
    assert_eq!(
        packet.record_kind,
        M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_RECORD_KIND
    );
    assert_eq!(packet.gate_bindings.len(), 16);
}

#[test]
fn ai_repair_importer_and_direct_save_hit_the_same_reason_for_one_object() {
    // AC1: at least one AI path, one repair path, one importer path, and one direct edit / save path hit the same
    // blocked-write reason vocabulary for the same constrained object.
    let packet = seed();
    let managed: Vec<_> = packet
        .gate_bindings
        .iter()
        .filter(|b| b.object_profile_id == "managed/captured-snapshot-mirror")
        .collect();
    let actors: BTreeSet<_> = managed.iter().map(|b| b.actor).collect();
    for required in [
        MutationActor::AiApply,
        MutationActor::Repair,
        MutationActor::Importer,
        MutationActor::DirectEditSave,
    ] {
        assert!(actors.contains(&required), "missing actor {required:?}");
    }
    // All four resolve to the same blocked reason and safe next step — keyed to the state class.
    let reasons: BTreeSet<_> = managed
        .iter()
        .map(|b| b.resolution.blocked_write_reason)
        .collect();
    assert_eq!(reasons.len(), 1);
    assert!(reasons.contains(&BlockedWriteReason::ManagedSourceRequiresDetach));
    let steps: BTreeSet<_> = managed
        .iter()
        .map(|b| b.resolution.safe_next_step)
        .collect();
    assert_eq!(steps.len(), 1);
    assert!(steps.contains(&WriteReviewFallbackAction::DetachFromManagedSource));
}

#[test]
fn blocked_reason_is_a_pure_function_of_the_state_class() {
    for object_class in M5ConstrainedFileStateObject::ALL {
        let reason = BlockedWriteReason::for_object_class(object_class);
        assert_eq!(reason.object_class(), object_class);
        assert_eq!(reason.safe_next_step(), reason.safe_next_step());
    }
    // Every binding's resolution classifies its own object class, never the actor.
    let packet = seed();
    for binding in &packet.gate_bindings {
        assert!(binding.resolution_matches_object_class());
        assert_eq!(
            binding.resolution.blocked_write_reason,
            BlockedWriteReason::for_object_class(binding.object_class)
        );
    }
}

#[test]
fn every_actor_and_posture_is_exercised() {
    let packet = seed();
    let actors: BTreeSet<_> = packet.gate_bindings.iter().map(|b| b.actor).collect();
    for actor in MutationActor::ALL {
        assert!(actors.contains(&actor), "actor {} missing", actor.as_str());
    }
    let postures: BTreeSet<_> = packet.gate_bindings.iter().map(|b| b.posture).collect();
    for posture in GateEnforcementPosture::ALL {
        assert!(
            postures.contains(&posture),
            "posture {} missing",
            posture.as_str()
        );
    }
}

#[test]
fn every_blocked_reason_class_is_covered() {
    let packet = seed();
    let reasons: BTreeSet<_> = packet
        .gate_bindings
        .iter()
        .map(|b| b.resolution.blocked_write_reason)
        .collect();
    for reason in BlockedWriteReason::ALL {
        assert!(
            reasons.contains(&reason),
            "reason {} missing",
            reason.as_str()
        );
    }
}

#[test]
fn no_bypass_actor_can_silently_write_a_constrained_object() {
    // AC2: mutation-capable actors cannot silently write generated, managed, projection, or archived objects just
    // because they bypass direct typing.
    let packet = seed();
    for binding in &packet.gate_bindings {
        assert!(!binding.silently_writes_constrained_object_bypassing_direct_typing);
        assert!(binding.routed_through_shared_gate);
        // No direct-write action can even be represented; the only write-adjacent action opens the reviewed
        // transition.
        for action in &binding.allowed_actions {
            assert!(matches!(
                action,
                GateAction::InspectBlockedWriteReason
                    | GateAction::RevealCanonicalSourceAndWriteTarget
                    | GateAction::CopySafeNextStep
                    | GateAction::OpenSafeNextStepReview
            ));
        }
        // Every bypass actor is still routed through the same gate as a direct write.
        if binding.actor.bypasses_direct_typing() {
            assert!(binding.routed_through_shared_gate);
        }
    }
}

#[test]
fn gate_fails_closed_on_actor_context_drift() {
    // AC3 (fail closed): a fail-closed posture names a fail-closed reason and offers no write path.
    let packet = seed();
    let fail_closed: Vec<_> = packet
        .gate_bindings
        .iter()
        .filter(|b| b.posture == GateEnforcementPosture::FailClosedOnActorDrift)
        .collect();
    assert!(!fail_closed.is_empty(), "at least one fail-closed binding");
    for binding in &fail_closed {
        assert!(binding.fail_closed_reason.is_some());
        assert!(!binding
            .allowed_actions
            .contains(&GateAction::OpenSafeNextStepReview));
    }
    // Both fail-closed reasons are exercised across the seed.
    let reasons: BTreeSet<_> = packet
        .gate_bindings
        .iter()
        .filter_map(|b| b.fail_closed_reason)
        .collect();
    assert!(reasons.contains(&FailClosedReason::ActorContextDrifted));
    assert!(reasons.contains(&FailClosedReason::ExactWriteTargetNotTruthfullyExplainable));
}

#[test]
fn support_trace_preserves_actor_reason_and_fallback() {
    // AC3 (trace): support / export traces preserve actor, blocked reason, and chosen fallback path.
    let packet = seed();
    for binding in &packet.gate_bindings {
        assert!(binding.trace_consistent());
        assert_eq!(binding.trace.actor, binding.actor);
        assert_eq!(
            binding.trace.blocked_write_reason,
            binding.resolution.blocked_write_reason
        );
        assert_eq!(
            binding.trace.chosen_fallback_path,
            binding.resolution.safe_next_step
        );
    }
}

#[test]
fn write_disposition_matches_safe_next_step_and_is_write_constrained() {
    let packet = seed();
    for binding in &packet.gate_bindings {
        assert!(
            binding
                .resolution
                .write_disposition_matches_safe_next_step(),
            "binding {} write disposition mismatches safe next step",
            binding.binding_id
        );
        assert!(binding
            .resolution
            .safe_next_step
            .required_write_disposition()
            .is_write_constrained());
        assert!(binding.resolution.write_disposition_satisfied());
        assert!(binding.resolution.checkpoint_matches_safe_next_step());
    }
}

#[test]
fn multi_state_objects_keep_every_state_visible() {
    let packet = seed();
    let multi: Vec<_> = packet
        .gate_bindings
        .iter()
        .filter(|b| b.is_multi_state())
        .collect();
    assert!(!multi.is_empty(), "at least one multi-state binding");
    for binding in &multi {
        assert!(binding.multi_state_facets_consistent());
        assert_eq!(
            binding.co_applicable_states.len(),
            binding.resolution.co_applicable_state_labels.len()
        );
    }
    let has_generated_plus_policy = packet.gate_bindings.iter().any(|b| {
        b.object_class == M5ConstrainedFileStateObject::Generated
            && b.co_applicable_states
                .contains(&M5ConstrainedFileStateObject::PolicyLocked)
    });
    let has_managed_plus_snapshot = packet.gate_bindings.iter().any(|b| {
        b.object_class == M5ConstrainedFileStateObject::Managed
            && b.co_applicable_states
                .contains(&M5ConstrainedFileStateObject::CapturedSnapshot)
    });
    assert!(
        has_generated_plus_policy,
        "Generated + Policy locked present"
    );
    assert!(
        has_managed_plus_snapshot,
        "Managed + Captured snapshot present"
    );
}

#[test]
fn hidden_multi_state_facet_is_rejected() {
    let mut packet = seed();
    let target = packet
        .gate_bindings
        .iter()
        .position(|b| b.is_multi_state())
        .unwrap();
    packet.gate_bindings[target]
        .resolution
        .co_applicable_state_labels
        .clear();
    assert!(violations_of(&packet).contains(&"multi_state_facet_hidden"));
}

#[test]
fn same_object_carries_identical_resolution_across_actors() {
    let packet = seed();
    let mut profile_resolution: BTreeMap<&str, &GateResolution> = BTreeMap::new();
    for binding in &packet.gate_bindings {
        match profile_resolution.get(binding.object_profile_id.as_str()) {
            None => {
                profile_resolution.insert(binding.object_profile_id.as_str(), &binding.resolution);
            }
            Some(existing) => assert_eq!(
                **existing, binding.resolution,
                "resolution drift on {}",
                binding.object_profile_id
            ),
        }
    }
    assert_eq!(profile_resolution.len(), 6);
}

#[test]
fn actions_are_safe_and_open_matches_posture() {
    let packet = seed();
    for binding in &packet.gate_bindings {
        assert!(binding.has_safe_base_actions());
        assert!(binding.open_action_matches_posture());
        let offers = binding
            .allowed_actions
            .contains(&GateAction::OpenSafeNextStepReview);
        assert_eq!(offers, !binding.is_narrowed(), "on {}", binding.binding_id);
    }
}

#[test]
fn accessibility_state_is_discoverable_for_every_binding() {
    let packet = seed();
    for binding in &packet.gate_bindings {
        assert!(
            binding.accessibility_state_discoverable(),
            "binding {} is not keyboard/screen-reader discoverable",
            binding.binding_id
        );
    }
}

#[test]
fn narrowed_bindings_disclose_and_enforced_bindings_do_not() {
    let packet = seed();
    for binding in &packet.gate_bindings {
        let disclosure = binding.disclosure();
        if binding.is_narrowed() {
            assert_eq!(
                binding.parity_state,
                GateParityState::ContentDisclosedNarrowed
            );
            let note = binding
                .narrow_note
                .as_ref()
                .expect("narrowed binding carries a note");
            assert_eq!(Some(note.reason), disclosure.narrow_reason);
            assert_eq!(Some(note.next_action), disclosure.narrow_next_action);
            assert!(!note.preserved_content_note.trim().is_empty());
            assert!(!note.next_action_label.trim().is_empty());
        } else {
            assert_eq!(binding.parity_state, GateParityState::ContentPreserved);
            assert!(binding.narrow_note.is_none());
            assert!(binding.fail_closed_reason.is_none());
        }
        if matches!(binding.posture, GateEnforcementPosture::ExportRedacted) {
            assert!(!binding.export_detail_note.trim().is_empty());
        }
    }
}

#[test]
fn export_bindings_point_at_canonical_contracts() {
    let packet = seed();
    for binding in &packet.gate_bindings {
        if posture_must_reference_canonical(binding.posture) {
            assert!(
                binding.points_at_canonical_contracts(),
                "binding {} must point at canonical contracts",
                binding.binding_id
            );
        }
    }
}

#[test]
fn disclosure_resolver_matches_posture() {
    let enforced = resolve_gate_render_disclosure(GateEnforcementPosture::EnforcedGate);
    assert!(!enforced.needs_narrow_note);
    assert!(enforced.offers_open_safe_next_step);
    assert!(!enforced.is_fail_closed);

    let fail_closed =
        resolve_gate_render_disclosure(GateEnforcementPosture::FailClosedOnActorDrift);
    assert_eq!(
        fail_closed.narrow_reason,
        Some(GateNarrowReason::FailedClosedOnActorContextDrift)
    );
    assert!(fail_closed.needs_narrow_note);
    assert!(!fail_closed.offers_open_safe_next_step);
    assert!(fail_closed.is_fail_closed);

    let exported = resolve_gate_render_disclosure(GateEnforcementPosture::ExportRedacted);
    assert!(exported.needs_export_detail_note);
    assert!(!exported.offers_open_safe_next_step);
    assert!(!exported.is_fail_closed);
}

#[test]
fn resolution_drift_across_actors_is_rejected() {
    let mut packet = seed();
    let target = packet
        .gate_bindings
        .iter()
        .position(|b| b.binding_id == "caw-managed-importer")
        .unwrap();
    packet.gate_bindings[target]
        .resolution
        .exact_write_target_word = "some_other_target".to_owned();
    assert!(violations_of(&packet).contains(&"blocked_reason_drift_across_actors"));
}

#[test]
fn dropped_write_disposition_is_rejected() {
    let mut packet = seed();
    packet.gate_bindings[0].resolution.write_disposition_word = "directly_writable".to_owned();
    let v = violations_of(&packet);
    assert!(v.contains(&"write_disposition_missing_for_constrained_object"));
    assert!(v.contains(&"write_disposition_safe_next_step_mismatch"));
}

#[test]
fn checkpoint_safe_next_step_mismatch_is_rejected() {
    let mut packet = seed();
    packet.gate_bindings[0].resolution.checkpoint_undo_class =
        CheckpointUndoClass::OverlayPatchRevertible;
    assert!(violations_of(&packet).contains(&"checkpoint_safe_next_step_mismatch"));
}

#[test]
fn not_routed_through_shared_gate_is_rejected() {
    let mut packet = seed();
    packet.gate_bindings[0].routed_through_shared_gate = false;
    assert!(violations_of(&packet).contains(&"not_routed_through_shared_gate"));
}

#[test]
fn safe_next_step_not_keyed_to_state_class_is_rejected() {
    let mut packet = seed();
    packet.gate_bindings[0].safe_next_step_keyed_to_state_class = false;
    assert!(violations_of(&packet).contains(&"safe_next_step_not_keyed_to_state_class"));
}

#[test]
fn fail_closed_reason_posture_mismatch_is_rejected() {
    let mut packet = seed();
    // An enforced binding must not carry a fail-closed reason.
    let target = packet
        .gate_bindings
        .iter()
        .position(|b| b.posture == GateEnforcementPosture::EnforcedGate)
        .unwrap();
    packet.gate_bindings[target].fail_closed_reason = Some(FailClosedReason::ActorContextDrifted);
    assert!(violations_of(&packet).contains(&"fail_closed_reason_posture_mismatch"));
}

#[test]
fn inconsistent_trace_is_rejected() {
    let mut packet = seed();
    packet.gate_bindings[0].trace.actor = MutationActor::AutomationRecipe;
    // Binding 0 is a direct-save actor, so the trace no longer matches.
    let bad = packet.gate_bindings[0].actor != MutationActor::AutomationRecipe;
    assert!(bad);
    assert!(violations_of(&packet).contains(&"trace_inconsistent"));
}

#[test]
fn open_action_mismatch_is_rejected() {
    let mut packet = seed();
    let target = packet
        .gate_bindings
        .iter()
        .position(|b| b.is_narrowed())
        .unwrap();
    packet.gate_bindings[target]
        .allowed_actions
        .push(GateAction::OpenSafeNextStepReview);
    assert!(violations_of(&packet).contains(&"open_action_posture_mismatch"));
}

#[test]
fn missing_safe_base_action_is_rejected() {
    let mut packet = seed();
    packet.gate_bindings[0]
        .allowed_actions
        .retain(|a| *a != GateAction::CopySafeNextStep);
    assert!(violations_of(&packet).contains(&"safe_base_actions_missing"));
}

#[test]
fn undiscoverable_accessibility_state_is_rejected() {
    let mut packet = seed();
    packet.gate_bindings[0].accessibility_routes =
        vec![M5ConstrainedFileStateAccessibilityRoute::HighZoomReflow];
    assert!(violations_of(&packet).contains(&"accessibility_state_undiscoverable"));
}

#[test]
fn actor_parity_gap_is_rejected() {
    let mut packet = seed();
    // Remove every AI-apply binding from the managed profile, breaking the four-actor parity for that object.
    packet.gate_bindings.retain(|b| {
        !(b.object_profile_id == "managed/captured-snapshot-mirror"
            && b.actor == MutationActor::AiApply)
    });
    assert!(violations_of(&packet).contains(&"actor_parity_unproven"));
}

#[test]
fn missing_canonical_reference_on_export_is_rejected() {
    let mut packet = seed();
    let target = packet
        .gate_bindings
        .iter()
        .position(|b| posture_must_reference_canonical(b.posture))
        .unwrap();
    packet.gate_bindings[target].source_contract_refs =
        vec![M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF.to_owned()];
    assert!(violations_of(&packet).contains(&"export_reference_missing"));
}

#[test]
fn missing_narrow_note_is_rejected() {
    let mut packet = seed();
    let target = packet
        .gate_bindings
        .iter()
        .position(|b| b.is_narrowed())
        .unwrap();
    packet.gate_bindings[target].narrow_note = None;
    assert!(violations_of(&packet).contains(&"narrow_note_missing"));
}

#[test]
fn unexpected_narrow_note_on_enforced_binding_is_rejected() {
    let mut packet = seed();
    let target = packet
        .gate_bindings
        .iter()
        .position(|b| !b.is_narrowed())
        .unwrap();
    packet.gate_bindings[target].narrow_note = Some(GateNarrowNote {
        reason: GateNarrowReason::FailedClosedOnActorContextDrift,
        preserved_content_note: "x".to_owned(),
        next_action: GateNarrowNextAction::ResolveActorContextThenRetry,
        next_action_label: "x".to_owned(),
    });
    assert!(violations_of(&packet).contains(&"unexpected_narrow_note"));
}

#[test]
fn each_guardrail_is_enforced() {
    let cases: [(fn(&mut CrossActorGateBinding), &str); 5] = [
        (
            |b| b.silently_writes_constrained_object_bypassing_direct_typing = true,
            "actor_silently_writes_constrained_object_bypassing_direct_typing",
        ),
        (
            |b| b.gives_ai_automation_import_or_repair_flows_a_hidden_bypass = true,
            "gives_ai_automation_import_or_repair_flows_a_hidden_bypass",
        ),
        (
            |b| b.uses_actor_specific_free_form_blocked_reason = true,
            "uses_actor_specific_free_form_blocked_reason",
        ),
        (
            |b| b.leaves_exact_write_target_or_canonical_source_unstated = true,
            "leaves_exact_write_target_or_canonical_source_unstated",
        ),
        (
            |b| b.lets_one_state_class_hide_another_when_both_materially_affect_behavior = true,
            "lets_one_state_class_hide_another_when_both_materially_affect_behavior",
        ),
    ];
    for (mutate, token) in cases {
        let mut packet = seed();
        mutate(&mut packet.gate_bindings[0]);
        assert!(violations_of(&packet).contains(&token), "expected {token}");
    }
}

#[test]
fn actor_coverage_gap_is_rejected() {
    let mut packet = seed();
    packet
        .gate_bindings
        .retain(|b| b.actor != MutationActor::AutomationRecipe);
    assert!(violations_of(&packet).contains(&"actor_coverage_missing"));
}

#[test]
fn blocked_reason_coverage_gap_is_rejected() {
    let mut packet = seed();
    packet
        .gate_bindings
        .retain(|b| b.object_class != M5ConstrainedFileStateObject::Projection);
    assert!(violations_of(&packet).contains(&"blocked_reason_coverage_missing"));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = seed();
    packet
        .source_contract_refs
        .retain(|r| r != M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_DOC_REF);
    assert!(violations_of(&packet).contains(&"missing_source_contracts"));
}

#[test]
fn actor_helpers_classify_bypass_and_ai_flows() {
    assert!(!MutationActor::DirectEditSave.bypasses_direct_typing());
    for actor in [
        MutationActor::AiApply,
        MutationActor::AutomationRecipe,
        MutationActor::Importer,
        MutationActor::Repair,
        MutationActor::CodeAction,
    ] {
        assert!(
            actor.bypasses_direct_typing(),
            "{} should bypass",
            actor.as_str()
        );
    }
    assert!(MutationActor::AiApply.is_ai_automation_import_or_repair());
    assert!(MutationActor::Repair.is_ai_automation_import_or_repair());
    assert!(!MutationActor::DirectEditSave.is_ai_automation_import_or_repair());
    assert!(!MutationActor::CodeAction.is_ai_automation_import_or_repair());
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
    assert_eq!(lines.len(), 1 + packet.gate_bindings.len());
    assert!(lines[0].starts_with(
        "object_class,co_applicable_states,actor,blocked_write_reason,safe_next_step,posture,checkpoint_undo_class,parity_state"
    ));
}

#[test]
fn markdown_summary_lists_every_binding() {
    let packet = seed();
    let summary = packet.render_markdown_summary();
    for binding in &packet.gate_bindings {
        assert!(summary.contains(&binding.binding_id));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_cross_actor_constrained_write_enforcement_export()
        .expect("checked M5 cross-actor constrained-write enforcement export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_CROSS_ACTOR_CONSTRAINED_WRITE_ENFORCEMENT_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seed(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_fixture_variants_validate_and_keep_coverage() {
    let fail_closed = seeded_m5_cross_actor_constrained_write_enforcement_fail_closed_narrowed();
    assert!(
        fail_closed.validate().is_empty(),
        "{:?}",
        violations_of(&fail_closed)
    );
    assert_eq!(fail_closed.gate_bindings.len(), 16);

    let export = seeded_m5_cross_actor_constrained_write_enforcement_export_redacted_narrowed();
    assert!(export.validate().is_empty(), "{:?}", violations_of(&export));
    assert_eq!(export.gate_bindings.len(), 16);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_builders() {
    let fail_closed: M5CrossActorConstrainedWriteEnforcementPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/editor/m5-cross-actor-constrained-write-enforcement/fail_closed_narrowed.json"
        )))
        .expect("fail-closed fixture parses");
    assert!(fail_closed.validate().is_empty());
    assert_eq!(
        fail_closed,
        seeded_m5_cross_actor_constrained_write_enforcement_fail_closed_narrowed()
    );

    let export: M5CrossActorConstrainedWriteEnforcementPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/editor/m5-cross-actor-constrained-write-enforcement/export_redacted_narrowed.json"
        )))
        .expect("export-redacted fixture parses");
    assert!(export.validate().is_empty());
    assert_eq!(
        export,
        seeded_m5_cross_actor_constrained_write_enforcement_export_redacted_narrowed()
    );
}
