use super::*;

fn seed() -> M5CollaborationControlSharedConsumersPacket {
    seeded_m5_collaboration_control_shared_consumers()
}

fn violations_of(packet: &M5CollaborationControlSharedConsumersPacket) -> Vec<&'static str> {
    packet.validate().iter().map(|v| v.as_str()).collect()
}

#[test]
fn seed_validates_clean() {
    let packet = seed();
    assert!(packet.validate().is_empty(), "{:?}", violations_of(&packet));
    assert_eq!(
        packet.packet_id,
        M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_PACKET_ID
    );
    assert_eq!(
        packet.record_kind,
        M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_RECORD_KIND
    );
    assert_eq!(packet.consumer_bindings.len(), 18);
}

#[test]
fn every_object_is_adopted_by_two_or_more_consumers() {
    let packet = seed();
    let mut object_consumers: BTreeMap<
        M5CollaborationControlObject,
        BTreeSet<M5CollaborationControlConsumerSurface>,
    > = BTreeMap::new();
    for binding in &packet.consumer_bindings {
        object_consumers
            .entry(binding.object)
            .or_default()
            .insert(binding.consumer);
    }
    assert_eq!(object_consumers.len(), 6, "all six objects adopted");
    for (object, consumers) in &object_consumers {
        assert!(
            consumers.len() >= 2,
            "object {} only adopted by {} consumers",
            object.as_str(),
            consumers.len()
        );
    }
}

#[test]
fn every_consumer_surface_and_representation_is_exercised() {
    let packet = seed();
    let consumers: BTreeSet<_> = packet
        .consumer_bindings
        .iter()
        .map(|b| b.consumer)
        .collect();
    for consumer in M5CollaborationControlConsumerSurface::ALL {
        assert!(
            consumers.contains(&consumer),
            "consumer {} missing",
            consumer.as_str()
        );
    }
    let representations: BTreeSet<_> = packet
        .consumer_bindings
        .iter()
        .map(|b| b.representation)
        .collect();
    for representation in CollaborationControlSharedRepresentation::ALL {
        assert!(
            representations.contains(&representation),
            "representation {} missing",
            representation.as_str()
        );
    }
}

#[test]
fn same_subject_carries_identical_vocabulary_across_surfaces() {
    let packet = seed();
    let mut subject_facets: BTreeMap<&str, &CollaborationControlSharedStateFacetValues> =
        BTreeMap::new();
    for binding in &packet.consumer_bindings {
        match subject_facets.get(binding.subject_id.as_str()) {
            None => {
                subject_facets.insert(binding.subject_id.as_str(), &binding.state_facets);
            }
            Some(existing) => assert_eq!(
                **existing, binding.state_facets,
                "vocabulary drift on {}",
                binding.subject_id
            ),
        }
    }
    // The six subjects each fan out to more than one consumer.
    assert_eq!(subject_facets.len(), 6);
}

#[test]
fn every_collaboration_control_role_word_is_a_frozen_role_token() {
    let packet = seed();
    for binding in &packet.consumer_bindings {
        assert!(
            binding
                .state_facets
                .collaboration_control_role_word_in_vocabulary(),
            "role word `{}` on {} is not a frozen role token",
            binding.state_facets.collaboration_control_role_word,
            binding.binding_id
        );
        assert!(binding.state_facets.all_present());
        assert!(binding.state_facets.authority_source_satisfied());
    }
}

#[test]
fn narrowed_bindings_disclose_and_full_bindings_do_not() {
    let packet = seed();
    for binding in &packet.consumer_bindings {
        let disclosure = binding.disclosure();
        if binding.is_narrowed() {
            assert_eq!(
                binding.vocabulary_state,
                CollaborationControlSharedVocabularyState::FacetsDisclosedNarrowed
            );
            let note = binding
                .narrow_note
                .as_ref()
                .expect("narrowed binding carries a note");
            assert_eq!(Some(note.reason), disclosure.narrow_reason);
            assert_eq!(Some(note.next_action), disclosure.narrow_next_action);
            assert!(!note.preserved_vocabulary_note.trim().is_empty());
            assert!(!note.next_action_label.trim().is_empty());
        } else {
            assert_eq!(
                binding.vocabulary_state,
                CollaborationControlSharedVocabularyState::FacetsPreserved
            );
            assert!(binding.narrow_note.is_none());
        }
        if matches!(
            binding.representation,
            CollaborationControlSharedRepresentation::RemoteProjected
        ) {
            assert!(!binding.remote_source_note.trim().is_empty());
        }
        if matches!(
            binding.representation,
            CollaborationControlSharedRepresentation::ExportedRedacted
        ) {
            assert!(!binding.export_detail_note.trim().is_empty());
        }
    }
}

#[test]
fn support_and_export_bindings_point_at_canonical_contracts() {
    let packet = seed();
    for binding in &packet.consumer_bindings {
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
fn disclosure_resolver_matches_representation() {
    assert!(
        !resolve_collaboration_control_shared_render_disclosure(
            CollaborationControlSharedRepresentation::DesktopFull
        )
        .needs_narrow_note
    );
    let compact = resolve_collaboration_control_shared_render_disclosure(
        CollaborationControlSharedRepresentation::CompactNarrowed,
    );
    assert_eq!(
        compact.narrow_reason,
        Some(CollaborationControlSharedNarrowReason::CompactionNarrowed)
    );
    assert!(compact.needs_narrow_note);
    assert!(!compact.needs_remote_source_note);
    let remote = resolve_collaboration_control_shared_render_disclosure(
        CollaborationControlSharedRepresentation::RemoteProjected,
    );
    assert!(remote.needs_remote_source_note);
    let exported = resolve_collaboration_control_shared_render_disclosure(
        CollaborationControlSharedRepresentation::ExportedRedacted,
    );
    assert!(exported.needs_export_detail_note);
}

#[test]
fn vocabulary_drift_is_rejected() {
    let mut packet = seed();
    // Reword one surface of a multi-binding subject to a different (still-valid) role token.
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| b.binding_id == "ccsc-grant-view")
        .unwrap();
    packet.consumer_bindings[target]
        .state_facets
        .collaboration_control_role_word = "paste_secret_guard_disclosure".to_owned();
    assert!(
        violations_of(&packet).contains(&"collaboration_control_vocabulary_drift_across_surfaces")
    );
}

#[test]
fn collaboration_control_role_word_outside_vocabulary_is_rejected() {
    let mut packet = seed();
    packet.consumer_bindings[0]
        .state_facets
        .collaboration_control_role_word = "totally_made_up".to_owned();
    let tokens = violations_of(&packet);
    assert!(tokens.contains(&"collaboration_control_role_word_outside_vocabulary"));
    // Rewording one surface also trips drift, which is expected and fine.
}

#[test]
fn dropped_authority_source_on_gate_role_is_rejected() {
    let mut packet = seed();
    // ccsc-sterm-view carries the control_authority_disclosure gate role, which must always keep a real
    // authority source and never collapse to a presence-as-authority masquerade sentinel.
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| b.binding_id == "ccsc-sterm-view")
        .unwrap();
    packet.consumer_bindings[target]
        .state_facets
        .authority_source_word = "presence_shown_as_control_authority".to_owned();
    assert!(violations_of(&packet).contains(&"authority_source_missing_for_gate_role"));
}

#[test]
fn reuse_below_two_consumers_is_rejected() {
    let mut packet = seed();
    // Drop every session-restore-view binding except one, leaving the object with one consumer.
    let mut kept_one = false;
    packet.consumer_bindings.retain(|b| {
        if b.object == M5CollaborationControlObject::SessionRestoreView {
            if kept_one {
                return false;
            }
            kept_one = true;
        }
        true
    });
    assert!(violations_of(&packet).contains(&"object_reuse_unproven"));
}

#[test]
fn missing_canonical_reference_on_export_is_rejected() {
    let mut packet = seed();
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| consumer_must_reference_canonical(b.consumer))
        .unwrap();
    packet.consumer_bindings[target].source_contract_refs =
        vec![M5_COLLABORATION_CONTROL_MATRIX_DOC_REF.to_owned()];
    assert!(violations_of(&packet).contains(&"support_export_reference_missing"));
}

#[test]
fn missing_narrow_note_is_rejected() {
    let mut packet = seed();
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| b.is_narrowed())
        .unwrap();
    packet.consumer_bindings[target].narrow_note = None;
    assert!(violations_of(&packet).contains(&"narrow_note_missing"));
}

#[test]
fn unexpected_narrow_note_on_full_binding_is_rejected() {
    let mut packet = seed();
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| !b.is_narrowed())
        .unwrap();
    packet.consumer_bindings[target].narrow_note = Some(CollaborationControlSharedNarrowNote {
        reason: CollaborationControlSharedNarrowReason::CompactionNarrowed,
        preserved_vocabulary_note: "x".to_owned(),
        next_action: CollaborationControlSharedNarrowNextAction::ExpandInDesktop,
        next_action_label: "x".to_owned(),
    });
    assert!(violations_of(&packet).contains(&"unexpected_narrow_note"));
}

#[test]
fn each_guardrail_is_enforced() {
    let cases: [(fn(&mut CollaborationControlSharedConsumerBinding), &str); 5] = [
        (
            |b| b.acquires_terminal_or_debug_control_from_presence_without_an_explicit_grant = true,
            "acquires_terminal_or_debug_control_from_presence_without_an_explicit_grant",
        ),
        (
            |b| {
                b.allows_more_than_one_active_driver_on_a_sensitive_surface =
                    true
            },
            "allows_more_than_one_active_driver_on_a_sensitive_surface",
        ),
        (
            |b| b.starts_recording_retention_or_guest_scope_widening_silently = true,
            "starts_recording_retention_or_guest_scope_widening_silently",
        ),
        (
            |b| b.replays_prior_terminal_or_debug_input_on_join_or_restore = true,
            "replays_prior_terminal_or_debug_input_on_join_or_restore",
        ),
        (
            |b| {
                b.reveals_raw_secrets_command_text_variable_bodies_or_clipboard_contents_without_a_guard = true
            },
            "reveals_raw_secrets_command_text_variable_bodies_or_clipboard_contents_without_a_guard",
        ),
    ];
    for (mutate, token) in cases {
        let mut packet = seed();
        mutate(&mut packet.consumer_bindings[0]);
        assert!(violations_of(&packet).contains(&token), "expected {token}");
    }
}

#[test]
fn object_coverage_gap_is_rejected() {
    let mut packet = seed();
    packet
        .consumer_bindings
        .retain(|b| b.object != M5CollaborationControlObject::SessionRestoreView);
    assert!(violations_of(&packet).contains(&"object_coverage_missing"));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = seed();
    packet
        .source_contract_refs
        .retain(|r| r != M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_DOC_REF);
    assert!(violations_of(&packet).contains(&"missing_source_contracts"));
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
    assert_eq!(lines.len(), 1 + packet.consumer_bindings.len());
    assert!(lines[0].starts_with(
        "object,consumer,representation,collaboration_control_role_word,vocabulary_state"
    ));
}

#[test]
fn markdown_summary_lists_every_binding() {
    let packet = seed();
    let summary = packet.render_markdown_summary();
    for binding in &packet.consumer_bindings {
        assert!(summary.contains(&binding.binding_id));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_collaboration_control_shared_consumers_export()
        .expect("checked M5 collaboration-control shared-consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_COLLABORATION_CONTROL_SHARED_CONSUMERS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seed(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_fixture_variants_validate_and_keep_coverage() {
    let compact = seeded_m5_collaboration_control_shared_consumers_compact_remote_narrowed();
    assert!(
        compact.validate().is_empty(),
        "{:?}",
        violations_of(&compact)
    );
    assert_eq!(compact.consumer_bindings.len(), 18);

    let exported = seeded_m5_collaboration_control_shared_consumers_exported_redaction_narrowed();
    assert!(
        exported.validate().is_empty(),
        "{:?}",
        violations_of(&exported)
    );
    assert_eq!(exported.consumer_bindings.len(), 18);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_builders() {
    let compact: M5CollaborationControlSharedConsumersPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/collaboration/m5-collaboration-control-shared-consumers/compact_remote_narrowed.json"
    )))
    .expect("compact fixture parses");
    assert!(compact.validate().is_empty());
    assert_eq!(
        compact,
        seeded_m5_collaboration_control_shared_consumers_compact_remote_narrowed()
    );

    let exported: M5CollaborationControlSharedConsumersPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/collaboration/m5-collaboration-control-shared-consumers/exported_redaction_narrowed.json"
    )))
    .expect("exported fixture parses");
    assert!(exported.validate().is_empty());
    assert_eq!(
        exported,
        seeded_m5_collaboration_control_shared_consumers_exported_redaction_narrowed()
    );
}

#[test]
fn deferred_intent_no_silent_queue_invariants_are_enforced() {
    // The seed declares the deferred-intent / no-silent-queued-grant posture on every axis.
    let packet = seed();
    assert!(
        packet
            .trust_review
            .deferred_intent_never_queues_control_grants_presenter_handoffs_or_terminal_input
    );
    assert!(
        packet
            .trust_review
            .refused_control_actions_explain_instead_of_replaying_as_idempotent_background_writes
    );
    assert!(
        packet
            .consumer_projection
            .deferred_intent_and_outbox_systems_blocked_from_queueing_sensitive_control_actions
    );
    assert!(packet.downgrade_triggers.contains(
        &M5CollaborationControlSharedConsumersDowngradeTrigger::DeferredIntentQueuedASensitiveControlActionWithoutAFreshLiveReview
    ));

    // Dropping the deferred-intent trust invariant narrows the lane rather than passing silently.
    let mut broken = seed();
    broken
        .trust_review
        .deferred_intent_never_queues_control_grants_presenter_handoffs_or_terminal_input = false;
    assert!(violations_of(&broken).contains(&"trust_review_incomplete"));

    // Dropping the deferred-intent projection invariant likewise blocks promotion.
    let mut broken_projection = seed();
    broken_projection
        .consumer_projection
        .deferred_intent_and_outbox_systems_blocked_from_queueing_sensitive_control_actions = false;
    assert!(violations_of(&broken_projection).contains(&"consumer_projection_incomplete"));
}
