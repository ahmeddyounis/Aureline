use super::*;

fn seed() -> M5ArchivedEvidenceStatePacket {
    seeded_m5_archived_evidence_state()
}

fn violations_of(packet: &M5ArchivedEvidenceStatePacket) -> Vec<&'static str> {
    packet.validate().iter().map(|v| v.as_str()).collect()
}

#[test]
fn seed_validates_clean() {
    let packet = seed();
    assert!(packet.validate().is_empty(), "{:?}", violations_of(&packet));
    assert_eq!(packet.packet_id, M5_ARCHIVED_EVIDENCE_STATE_PACKET_ID);
    assert_eq!(packet.record_kind, M5_ARCHIVED_EVIDENCE_STATE_RECORD_KIND);
    assert_eq!(packet.state_bindings.len(), 15);
}

#[test]
fn every_object_class_is_stated_by_two_or_more_consumers() {
    let packet = seed();
    let mut object_consumers: BTreeMap<
        M5HistoricalReferenceObject,
        BTreeSet<M5HistoricalReferenceConsumerSurface>,
    > = BTreeMap::new();
    for binding in &packet.state_bindings {
        object_consumers
            .entry(binding.object_class)
            .or_default()
            .insert(binding.consumer);
    }
    assert_eq!(object_consumers.len(), 5, "all five object classes stated");
    for (object_class, consumers) in &object_consumers {
        assert!(
            consumers.len() >= 2,
            "object class {} only stated by {} consumers",
            object_class.as_str(),
            consumers.len()
        );
    }
}

#[test]
fn every_consumer_surface_and_state_is_exercised() {
    let packet = seed();
    let consumers: BTreeSet<_> = packet.state_bindings.iter().map(|b| b.consumer).collect();
    for consumer in M5HistoricalReferenceConsumerSurface::ALL {
        assert!(
            consumers.contains(&consumer),
            "consumer {} missing",
            consumer.as_str()
        );
    }
    let states: BTreeSet<_> = packet.state_bindings.iter().map(|b| b.state).collect();
    for state in ArchivedEvidenceState::ALL {
        assert!(states.contains(&state), "state {} missing", state.as_str());
    }
}

#[test]
fn same_profile_carries_identical_grammar_across_surfaces() {
    let packet = seed();
    let mut profile_grammar: BTreeMap<&str, &ArchiveStateHistoricalGrammar> = BTreeMap::new();
    for binding in &packet.state_bindings {
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
    for binding in &packet.state_bindings {
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
fn actions_are_closed_and_remove_open_match_state() {
    let packet = seed();
    for binding in &packet.state_bindings {
        assert!(binding.has_base_actions());
        assert!(binding.action_set_is_closed());
        assert!(binding.remove_action_matches_state());
        assert!(binding.open_live_action_matches_state());
        // No apply / sync / restore action exists in the closed action enum.
        assert!(!binding.allowed_actions.iter().any(|a| {
            a.as_str().contains("apply")
                || a.as_str().contains("sync")
                || a.as_str().contains("restore")
        }));
        // Only an available archive offers open-current-live-object.
        let offers_open = binding
            .allowed_actions
            .contains(&ArchiveStateAction::OpenCurrentLiveObject);
        assert_eq!(
            offers_open,
            binding.state == ArchivedEvidenceState::PreservedAvailable,
            "on {}",
            binding.binding_id
        );
        // Remove is offered only for Expired / RetentionWindowEnded.
        let offers_remove = binding
            .allowed_actions
            .contains(&ArchiveStateAction::RemoveArchivedObject);
        let expected_remove = matches!(
            binding.state,
            ArchivedEvidenceState::Expired | ArchivedEvidenceState::RetentionWindowEnded
        );
        assert_eq!(offers_remove, expected_remove, "on {}", binding.binding_id);
    }
}

#[test]
fn content_presence_matches_state_for_every_binding() {
    let packet = seed();
    for binding in &packet.state_bindings {
        assert!(
            binding.content_presence_matches_state(),
            "content presence mismatch on {}",
            binding.binding_id
        );
    }
}

#[test]
fn removed_and_metadata_only_bindings_render_metadata_instead_of_dead_link() {
    let packet = seed();
    for binding in &packet.state_bindings {
        if !binding.content_bytes_present {
            assert!(
                binding.renders_metadata_instead_of_dead_link(),
                "binding {} dead-links instead of rendering metadata",
                binding.binding_id
            );
            let note = binding
                .removal_note
                .as_ref()
                .expect("content-gone binding carries a removal note");
            assert!(!note.explanation.trim().is_empty());
            assert!(binding.historical_grammar.capture_context_present());
        }
    }
}

#[test]
fn removal_outcomes_are_attributable() {
    let packet = seed();
    for binding in &packet.state_bindings {
        if let Some(note) = &binding.removal_note {
            assert!(
                note.removal_attribution.all_present(),
                "binding {} removal outcome is not attributable",
                binding.binding_id
            );
            assert!(note.reason.supported_by(binding.state));
        }
    }
}

#[test]
fn accessibility_state_is_discoverable_for_every_binding() {
    let packet = seed();
    for binding in &packet.state_bindings {
        assert!(
            binding.accessibility_state_discoverable(),
            "binding {} is not keyboard/screen-reader discoverable",
            binding.binding_id
        );
    }
}

#[test]
fn available_and_disclosing_bindings_split_correctly() {
    let packet = seed();
    for binding in &packet.state_bindings {
        let disclosure = binding.disclosure();
        if binding.discloses_removal_or_expiry() {
            assert_eq!(
                binding.parity_state,
                ArchiveStateParity::RemovalOrExpiryDisclosed
            );
            let note = binding
                .removal_note
                .as_ref()
                .expect("disclosing binding carries a removal note");
            assert!(binding
                .state
                .allowed_removal_reasons()
                .contains(&note.reason));
            assert_eq!(Some(note.next_action), disclosure.removal_next_action);
            assert!(!note.explanation.trim().is_empty());
            assert!(!note.preserved_metadata_note.trim().is_empty());
            assert!(!note.next_action_label.trim().is_empty());
        } else {
            assert_eq!(
                binding.parity_state,
                ArchiveStateParity::ArchiveStatePresented
            );
            assert!(binding.removal_note.is_none());
        }
    }
}

#[test]
fn support_and_export_bindings_point_at_canonical_contracts() {
    let packet = seed();
    for binding in &packet.state_bindings {
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
fn disclosure_resolver_matches_state() {
    let available =
        resolve_archive_state_render_disclosure(ArchivedEvidenceState::PreservedAvailable);
    assert!(!available.needs_removal_note);
    assert!(available.offers_open_live_target);
    assert!(!available.offers_remove_action);
    assert!(available.expects_content_bytes_present);

    let expired = resolve_archive_state_render_disclosure(ArchivedEvidenceState::Expired);
    assert!(expired.needs_removal_note);
    assert!(expired.offers_remove_action);
    assert!(!expired.offers_open_live_target);
    assert_eq!(
        expired.removal_next_action,
        Some(RemovalExpiryNextAction::RemoveThroughReviewedCleanup)
    );

    let removed = resolve_archive_state_render_disclosure(ArchivedEvidenceState::Removed);
    assert!(removed.needs_removal_note);
    assert!(!removed.offers_remove_action);
    assert!(!removed.expects_content_bytes_present);
    assert_eq!(
        removed.removal_next_action,
        Some(RemovalExpiryNextAction::InspectMetadataOnly)
    );

    let retention =
        resolve_archive_state_render_disclosure(ArchivedEvidenceState::RetentionWindowEnded);
    assert!(retention.offers_remove_action);
    assert!(retention.expects_content_bytes_present);

    let missing = resolve_archive_state_render_disclosure(ArchivedEvidenceState::MissingLiveTarget);
    assert!(missing.needs_removal_note);
    assert!(!missing.offers_open_live_target);
    assert!(missing.expects_content_bytes_present);

    let metadata_only =
        resolve_archive_state_render_disclosure(ArchivedEvidenceState::MetadataOnly);
    assert!(metadata_only.needs_removal_note);
    assert!(!metadata_only.expects_content_bytes_present);
}

#[test]
fn grammar_drift_is_rejected() {
    let mut packet = seed();
    let target = packet
        .state_bindings
        .iter()
        .position(|b| b.binding_id == "aes-retirement-shell")
        .unwrap();
    packet.state_bindings[target]
        .historical_grammar
        .historical_role_word = "capture_time_attribution".to_owned();
    assert!(violations_of(&packet).contains(&"state_grammar_drift_across_surfaces"));
}

#[test]
fn historical_role_word_outside_vocabulary_is_rejected() {
    let mut packet = seed();
    packet.state_bindings[0]
        .historical_grammar
        .historical_role_word = "totally_made_up".to_owned();
    assert!(violations_of(&packet).contains(&"historical_role_word_outside_vocabulary"));
}

#[test]
fn dropped_mutation_blocked_posture_on_gate_role_is_rejected() {
    let mut packet = seed();
    // aes-runbook-archive carries the snapshot_labeling gate role, which must always keep a real
    // mutation-blocked posture.
    let target = packet
        .state_bindings
        .iter()
        .position(|b| b.binding_id == "aes-runbook-archive")
        .unwrap();
    packet.state_bindings[target]
        .historical_grammar
        .mutation_blocked_posture_word = "editable".to_owned();
    assert!(violations_of(&packet).contains(&"mutation_blocked_posture_missing_for_gate_role"));
}

#[test]
fn content_presence_mismatch_is_rejected() {
    let mut packet = seed();
    // Flip the content flag on an available archive, which must keep its bytes present.
    let target = packet
        .state_bindings
        .iter()
        .position(|b| b.state == ArchivedEvidenceState::PreservedAvailable)
        .unwrap();
    packet.state_bindings[target].content_bytes_present = false;
    assert!(violations_of(&packet).contains(&"content_presence_mismatch"));
}

#[test]
fn parity_state_mismatch_is_rejected() {
    let mut packet = seed();
    let target = packet
        .state_bindings
        .iter()
        .position(|b| b.state == ArchivedEvidenceState::PreservedAvailable)
        .unwrap();
    packet.state_bindings[target].parity_state = ArchiveStateParity::RemovalOrExpiryDisclosed;
    assert!(violations_of(&packet).contains(&"parity_state_mismatch"));
}

#[test]
fn missing_state_label_is_rejected() {
    let mut packet = seed();
    packet.state_bindings[0].state_label = String::new();
    assert!(violations_of(&packet).contains(&"state_label_missing"));
}

#[test]
fn historical_side_not_mutation_blocked_is_rejected() {
    let mut packet = seed();
    packet.state_bindings[0].historical_side_mutation_blocked = false;
    assert!(violations_of(&packet).contains(&"historical_side_not_mutation_blocked"));
}

#[test]
fn remove_action_state_mismatch_is_rejected() {
    let mut packet = seed();
    // Add a remove action to a preserved-available binding, which must not offer it.
    let target = packet
        .state_bindings
        .iter()
        .position(|b| b.state == ArchivedEvidenceState::PreservedAvailable)
        .unwrap();
    packet.state_bindings[target]
        .allowed_actions
        .push(ArchiveStateAction::RemoveArchivedObject);
    assert!(violations_of(&packet).contains(&"remove_action_state_mismatch"));
}

#[test]
fn open_live_action_state_mismatch_is_rejected() {
    let mut packet = seed();
    // Add an open-current-live-object action to a disclosing binding, which must not offer it.
    let target = packet
        .state_bindings
        .iter()
        .position(|b| b.discloses_removal_or_expiry())
        .unwrap();
    packet.state_bindings[target]
        .allowed_actions
        .push(ArchiveStateAction::OpenCurrentLiveObject);
    assert!(violations_of(&packet).contains(&"open_live_action_state_mismatch"));
}

#[test]
fn missing_base_action_is_rejected() {
    let mut packet = seed();
    packet.state_bindings[0]
        .allowed_actions
        .retain(|a| *a != ArchiveStateAction::ExportEvidence);
    assert!(violations_of(&packet).contains(&"base_actions_missing"));
}

#[test]
fn undiscoverable_accessibility_state_is_rejected() {
    let mut packet = seed();
    packet.state_bindings[0].accessibility_routes =
        vec![M5HistoricalReferenceAccessibilityRoute::HighZoomReflow];
    assert!(violations_of(&packet).contains(&"accessibility_state_undiscoverable"));
}

#[test]
fn reuse_below_two_consumers_is_rejected() {
    let mut packet = seed();
    let mut kept_one = false;
    packet.state_bindings.retain(|b| {
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
        .state_bindings
        .iter()
        .position(|b| consumer_must_reference_canonical(b.consumer))
        .unwrap();
    packet.state_bindings[target].source_contract_refs =
        vec![M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF.to_owned()];
    assert!(violations_of(&packet).contains(&"support_export_reference_missing"));
}

#[test]
fn missing_removal_note_is_rejected() {
    let mut packet = seed();
    let target = packet
        .state_bindings
        .iter()
        .position(|b| b.discloses_removal_or_expiry())
        .unwrap();
    packet.state_bindings[target].removal_note = None;
    assert!(violations_of(&packet).contains(&"removal_note_missing"));
}

#[test]
fn unexpected_removal_note_on_available_binding_is_rejected() {
    let mut packet = seed();
    let target = packet
        .state_bindings
        .iter()
        .position(|b| !b.discloses_removal_or_expiry())
        .unwrap();
    packet.state_bindings[target].removal_note = Some(RemovalExpiryNote {
        reason: RemovalExpiryReason::RetentionWindowElapsed,
        explanation: "x".to_owned(),
        preserved_metadata_note: "x".to_owned(),
        removal_attribution: RemovalAttribution {
            retention_or_deletion_receipt_ref: "x".to_owned(),
            retirement_closure_ledger_ref: "x".to_owned(),
            support_packet_manifest_ref: "x".to_owned(),
        },
        next_action: RemovalExpiryNextAction::InspectMetadataOnly,
        next_action_label: "x".to_owned(),
    });
    assert!(violations_of(&packet).contains(&"unexpected_removal_note"));
}

#[test]
fn removal_reason_not_allowed_for_state_is_rejected() {
    let mut packet = seed();
    // A Removed binding cannot name RetentionWindowElapsed (allowed only for Expired / RetentionWindowEnded).
    let target = packet
        .state_bindings
        .iter()
        .position(|b| b.state == ArchivedEvidenceState::Removed)
        .unwrap();
    if let Some(note) = packet.state_bindings[target].removal_note.as_mut() {
        note.reason = RemovalExpiryReason::RetentionWindowElapsed;
    }
    assert!(violations_of(&packet).contains(&"removal_reason_not_allowed_for_state"));
}

#[test]
fn removal_attribution_incomplete_is_rejected() {
    let mut packet = seed();
    let target = packet
        .state_bindings
        .iter()
        .position(|b| b.discloses_removal_or_expiry())
        .unwrap();
    if let Some(note) = packet.state_bindings[target].removal_note.as_mut() {
        note.removal_attribution.support_packet_manifest_ref = String::new();
    }
    assert!(violations_of(&packet).contains(&"removal_attribution_incomplete"));
}

#[test]
fn each_guardrail_is_enforced() {
    let cases: [(fn(&mut ArchivedEvidenceStateBinding), &str); 5] = [
        (
            |b| b.reopens_live_target_without_validating_identity_trust_route_and_authority = true,
            "reopens_live_target_without_validating_identity_trust_route_and_authority",
        ),
        (
            |b| b.degrades_to_generic_dead_link = true,
            "degrades_to_generic_dead_link",
        ),
        (
            |b| b.removes_content_without_attribution = true,
            "removes_content_without_attribution",
        ),
        (
            |b| b.presents_expired_or_removed_as_live_or_current = true,
            "presents_expired_or_removed_as_live_or_current",
        ),
        (
            |b| b.drops_removal_or_expiry_vocabulary_in_export = true,
            "drops_removal_or_expiry_vocabulary_in_export",
        ),
    ];
    for (mutate, token) in cases {
        let mut packet = seed();
        mutate(&mut packet.state_bindings[0]);
        assert!(violations_of(&packet).contains(&token), "expected {token}");
    }
}

#[test]
fn metadata_fallback_missing_is_rejected() {
    let mut packet = seed();
    // A content-gone binding whose removal note is stripped of its explanation dead-links.
    let target = packet
        .state_bindings
        .iter()
        .position(|b| !b.content_bytes_present)
        .unwrap();
    if let Some(note) = packet.state_bindings[target].removal_note.as_mut() {
        note.explanation = String::new();
    }
    let tokens = violations_of(&packet);
    assert!(tokens.contains(&"metadata_fallback_missing"));
}

#[test]
fn object_class_coverage_gap_is_rejected() {
    let mut packet = seed();
    packet
        .state_bindings
        .retain(|b| b.object_class != M5HistoricalReferenceObject::ReviewIncidentSnapshot);
    assert!(violations_of(&packet).contains(&"object_class_coverage_missing"));
}

#[test]
fn state_coverage_gap_is_rejected() {
    let mut packet = seed();
    // Drop every metadata-only binding, leaving that state uncovered.
    packet
        .state_bindings
        .retain(|b| b.state != ArchivedEvidenceState::MetadataOnly);
    assert!(violations_of(&packet).contains(&"state_coverage_missing"));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = seed();
    packet
        .source_contract_refs
        .retain(|r| r != M5_ARCHIVED_EVIDENCE_STATE_DOC_REF);
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
    assert_eq!(lines.len(), 1 + packet.state_bindings.len());
    assert!(lines[0].starts_with(
        "object_class,consumer,state,content_bytes_present,removal_reason,parity_state,state_label"
    ));
}

#[test]
fn csv_preserves_expired_and_removed_vocabulary() {
    let packet = seed();
    let csv = packet.render_matrix_csv();
    assert!(csv.contains(",expired,"));
    assert!(csv.contains(",removed,"));
    assert!(csv.contains(",retention_window_ended,"));
    assert!(csv.contains(",missing_live_target,"));
    assert!(csv.contains(",metadata_only,"));
}

#[test]
fn markdown_summary_lists_every_binding() {
    let packet = seed();
    let summary = packet.render_markdown_summary();
    for binding in &packet.state_bindings {
        assert!(summary.contains(&binding.binding_id));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_archived_evidence_state_export()
        .expect("checked M5 archived-evidence-state export validates");
    assert_eq!(from_disk.packet_id, M5_ARCHIVED_EVIDENCE_STATE_PACKET_ID);
    assert_eq!(
        from_disk,
        seed(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_fixture_variants_validate_and_keep_coverage() {
    let expired = seeded_m5_archived_evidence_state_expired_narrowed();
    assert!(
        expired.validate().is_empty(),
        "{:?}",
        violations_of(&expired)
    );
    assert_eq!(expired.state_bindings.len(), 15);

    let removed = seeded_m5_archived_evidence_state_removed_narrowed();
    assert!(
        removed.validate().is_empty(),
        "{:?}",
        violations_of(&removed)
    );
    assert_eq!(removed.state_bindings.len(), 15);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_builders() {
    let expired: M5ArchivedEvidenceStatePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/recovery/m5-archived-evidence-state/expired_narrowed.json"
    )))
    .expect("expired fixture parses");
    assert!(expired.validate().is_empty());
    assert_eq!(
        expired,
        seeded_m5_archived_evidence_state_expired_narrowed()
    );

    let removed: M5ArchivedEvidenceStatePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/recovery/m5-archived-evidence-state/removed_narrowed.json"
    )))
    .expect("removed fixture parses");
    assert!(removed.validate().is_empty());
    assert_eq!(
        removed,
        seeded_m5_archived_evidence_state_removed_narrowed()
    );
}
