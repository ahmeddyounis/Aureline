use super::*;

use crate::m5_write_review_sheet_fallback_paths::WriteReviewFallbackAction;

fn seed() -> M5ConstrainedStateEvidencePacket {
    seeded_m5_constrained_state_evidence_packets()
}

fn violations_of(packet: &M5ConstrainedStateEvidencePacket) -> Vec<&'static str> {
    packet.validate().iter().map(|v| v.as_str()).collect()
}

#[test]
fn seed_validates_clean() {
    let packet = seed();
    assert!(packet.validate().is_empty(), "{:?}", violations_of(&packet));
    assert_eq!(packet.packet_id, M5_CONSTRAINED_STATE_EVIDENCE_PACKET_ID);
    assert_eq!(
        packet.record_kind,
        M5_CONSTRAINED_STATE_EVIDENCE_RECORD_KIND
    );
    assert_eq!(packet.evidence_bindings.len(), 16);
}

#[test]
fn every_object_class_is_preserved_by_two_or_more_channels() {
    let packet = seed();
    let mut object_channels: BTreeMap<
        M5ConstrainedFileStateObject,
        BTreeSet<EvidencePacketChannel>,
    > = BTreeMap::new();
    for binding in &packet.evidence_bindings {
        object_channels
            .entry(binding.object_class)
            .or_default()
            .insert(binding.channel);
    }
    assert_eq!(object_channels.len(), 6, "all six object classes preserved");
    for (object_class, channels) in &object_channels {
        assert!(
            channels.len() >= 2,
            "object class {} only preserved by {} channels",
            object_class.as_str(),
            channels.len()
        );
    }
}

#[test]
fn ac1_support_and_review_packets_and_both_forms() {
    // AC1: at least one support packet and one review / export packet preserve constrained-state and write-target
    // decisions in both human-readable and machine-readable form.
    let packet = seed();
    let channels: BTreeSet<_> = packet.evidence_bindings.iter().map(|b| b.channel).collect();
    for channel in EvidencePacketChannel::ALL {
        assert!(
            channels.contains(&channel),
            "channel {} missing",
            channel.as_str()
        );
    }
    assert!(
        channels.contains(&EvidencePacketChannel::SupportBundle),
        "no support bundle present"
    );
    assert!(
        channels.contains(&EvidencePacketChannel::ReviewExportPacket),
        "no review / export packet present"
    );
    for binding in &packet.evidence_bindings {
        assert!(
            binding.both_forms_present(),
            "binding {} is missing a dual form",
            binding.binding_id
        );
        assert!(
            binding.machine_readable_matches_binding(),
            "binding {} machine-readable record drifts from its typed decision",
            binding.binding_id
        );
    }
}

#[test]
fn ac2_no_packet_flattens_state_class_into_generic_read_only() {
    // AC2: exported packets remain intelligible without the live UI and do not flatten constrained-state truth into
    // generic read-only language.
    let packet = seed();
    for binding in &packet.evidence_bindings {
        assert!(
            binding.human_readable_names_state_class(),
            "binding {} flattens its state class",
            binding.binding_id
        );
        // The machine-readable record keeps the specific class token, never a generic read-only stand-in.
        assert_eq!(
            binding.dual_form.machine_readable.object_class_token,
            binding.object_class.as_str()
        );
        // The human line is a full sentence intelligible without the live UI.
        assert!(binding.dual_form.human_readable_line.len() > 40);
    }
    // A non-read-only class that reads generically as "read only" is rejected.
    let mut mutated = seed();
    let target = mutated
        .evidence_bindings
        .iter()
        .position(|b| b.object_class == M5ConstrainedFileStateObject::Generated)
        .unwrap();
    mutated.evidence_bindings[target]
        .dual_form
        .human_readable_line = "This object is read only.".to_owned();
    assert!(violations_of(&mutated).contains(&"human_readable_flattens_state_class"));
}

#[test]
fn ac3_redacted_packets_keep_omission_reason_and_state_class_and_fallback() {
    // AC3: redacted packets keep the omission reason and still preserve the state class and fallback decision.
    let packet = seed();
    let redacted: Vec<_> = packet
        .evidence_bindings
        .iter()
        .filter(|b| b.redaction.disposition.is_redacted())
        .collect();
    assert!(!redacted.is_empty(), "no redacted binding present");
    for binding in redacted {
        assert!(
            binding
                .redaction
                .omission_reason
                .as_deref()
                .is_some_and(|reason| !reason.trim().is_empty()),
            "redacted binding {} dropped its omission reason",
            binding.binding_id
        );
        assert!(binding.redaction.state_class_preserved);
        assert!(binding.redaction.fallback_decision_preserved);
        assert!(binding.preserves_state_class_and_fallback_when_redacted);
        // The state class and fallback decision are still present in the machine-readable record.
        assert_eq!(
            binding.dual_form.machine_readable.object_class_token,
            binding.object_class.as_str()
        );
        assert_eq!(
            binding.dual_form.machine_readable.resolved_decision_token,
            binding.resolved_decision.as_str()
        );
    }
}

#[test]
fn every_resolved_decision_including_cancel_is_preserved() {
    let packet = seed();
    let decisions: BTreeSet<_> = packet
        .evidence_bindings
        .iter()
        .map(|b| b.resolved_decision)
        .collect();
    for decision in ResolvedFallbackDecision::ALL {
        assert!(
            decisions.contains(&decision),
            "resolved decision {} not preserved",
            decision.as_str()
        );
    }
    assert!(
        decisions.contains(&ResolvedFallbackDecision::Cancelled),
        "cancellation not preserved"
    );
}

#[test]
fn every_blocked_reason_and_fallback_is_distinguishable() {
    let packet = seed();
    let reasons: BTreeSet<_> = packet
        .evidence_bindings
        .iter()
        .map(|b| b.blocked_write_reason)
        .collect();
    for reason in BlockedWriteReason::ALL {
        assert!(
            reasons.contains(&reason),
            "blocked-write reason {} not preserved",
            reason.as_str()
        );
    }
    let fallbacks: BTreeSet<_> = packet
        .evidence_bindings
        .iter()
        .map(|b| b.chosen_fallback_path)
        .collect();
    for fallback in WriteReviewFallbackAction::ALL {
        assert!(
            fallbacks.contains(&fallback),
            "fallback path {} not preserved",
            fallback.as_str()
        );
    }
}

#[test]
fn every_binding_reason_fallback_and_disposition_are_pure_functions_of_class() {
    let packet = seed();
    for binding in &packet.evidence_bindings {
        assert!(
            binding.blocked_reason_matches_class(),
            "blocked-reason/class drift on {}",
            binding.binding_id
        );
        assert!(
            binding.fallback_matches_reason(),
            "fallback/reason drift on {}",
            binding.binding_id
        );
        assert!(
            binding.disposition_matches_fallback(),
            "disposition/fallback drift on {}",
            binding.binding_id
        );
        assert!(
            binding.checkpoint_matches_fallback(),
            "checkpoint/fallback drift on {}",
            binding.binding_id
        );
        assert!(
            binding.resolved_decision_consistent(),
            "resolved-decision drift on {}",
            binding.binding_id
        );
    }
}

#[test]
fn six_entry_families_are_seeded() {
    let packet = seed();
    let entries: BTreeSet<_> = packet
        .evidence_bindings
        .iter()
        .map(|b| b.entry_id.as_str())
        .collect();
    for family in [
        "read-only-alias-path",
        "generated-derived-artifact",
        "policy-locked-managed-mirror",
        "managed-external-source",
        "projection-virtual-view",
        "captured-workspace-snapshot",
    ] {
        assert!(entries.contains(family), "entry family {family} missing");
    }
    assert_eq!(entries.len(), 6);
}

#[test]
fn same_entry_carries_identical_grammar_across_channels() {
    let packet = seed();
    let mut entry_grammar: BTreeMap<&str, &ConstrainedStateGrammar> = BTreeMap::new();
    for binding in &packet.evidence_bindings {
        match entry_grammar.get(binding.entry_id.as_str()) {
            None => {
                entry_grammar.insert(binding.entry_id.as_str(), &binding.constrained_grammar);
            }
            Some(existing) => assert_eq!(
                **existing, binding.constrained_grammar,
                "grammar drift on {}",
                binding.entry_id
            ),
        }
    }
    assert_eq!(entry_grammar.len(), 6);
}

#[test]
fn every_state_role_word_is_a_frozen_role_token() {
    let packet = seed();
    for binding in &packet.evidence_bindings {
        assert!(
            binding.constrained_grammar.state_role_word_in_vocabulary(),
            "role word `{}` on {} is not a frozen role token",
            binding.constrained_grammar.state_role_word,
            binding.binding_id
        );
        assert!(binding.constrained_grammar.all_present());
        assert!(binding
            .constrained_grammar
            .write_disposition_constrained_satisfied());
        assert!(binding
            .constrained_grammar
            .canonical_source_and_write_target_present());
    }
}

#[test]
fn actions_are_closed_and_no_direct_write_leaks() {
    let packet = seed();
    for binding in &packet.evidence_bindings {
        assert!(binding.has_base_actions());
        assert!(binding.action_set_is_closed());
        assert!(binding.reviewed_fallback_replay_present());
        assert!(!binding.allowed_actions.iter().any(|a| {
            let token = a.as_str();
            token.contains("write_in_place")
                || token.contains("save")
                || token.contains("apply")
                || token.contains("sync")
                || token.contains("direct_write")
        }));
    }
}

#[test]
fn every_binding_renders_canonical_source_and_write_target() {
    let packet = seed();
    for binding in &packet.evidence_bindings {
        assert!(
            binding.renders_canonical_source_and_write_target(),
            "binding {} leaves canonical source / write target unstated",
            binding.binding_id
        );
        assert!(binding.canonical_source_join.all_present());
        assert!(binding.preserved_versus_lost.all_present());
    }
}

#[test]
fn accessibility_state_is_discoverable_for_every_binding() {
    let packet = seed();
    for binding in &packet.evidence_bindings {
        assert!(
            binding.accessibility_state_discoverable(),
            "binding {} is not keyboard/screen-reader discoverable",
            binding.binding_id
        );
    }
}

#[test]
fn support_and_export_bindings_point_at_canonical_contracts() {
    let packet = seed();
    for binding in &packet.evidence_bindings {
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
fn disclosure_resolver_matches_class() {
    use M5ConstrainedFileStateObject::*;

    let read_only = resolve_evidence_disclosure(ReadOnly);
    assert_eq!(
        read_only.blocked_write_reason,
        BlockedWriteReason::ReadOnlyPathNotDirectlyWritable
    );
    assert_eq!(
        read_only.chosen_fallback_path,
        WriteReviewFallbackAction::DuplicateToEditableCopy
    );

    let projection = resolve_evidence_disclosure(Projection);
    assert_eq!(
        projection.chosen_fallback_path,
        WriteReviewFallbackAction::CreateOverlayPatch
    );
    assert_eq!(
        projection.checkpoint_undo_class,
        CheckpointUndoClass::OverlayPatchRevertible
    );
}

#[test]
fn resolved_decision_maps_to_fallback_action() {
    for action in WriteReviewFallbackAction::ALL {
        let decision = ResolvedFallbackDecision::from_taken_fallback_action(action);
        assert_eq!(decision.taken_fallback_action(), Some(action));
        assert!(!decision.is_cancelled());
    }
    assert!(ResolvedFallbackDecision::Cancelled
        .taken_fallback_action()
        .is_none());
    assert!(ResolvedFallbackDecision::Cancelled.is_cancelled());
}

#[test]
fn grammar_drift_is_rejected() {
    let mut packet = seed();
    let target = packet
        .evidence_bindings
        .iter()
        .position(|b| b.binding_id == "cse-ro-review")
        .unwrap();
    packet.evidence_bindings[target]
        .constrained_grammar
        .state_class_label_word = "totally_different".to_owned();
    assert!(violations_of(&packet).contains(&"grammar_drift_across_channels"));
}

#[test]
fn state_role_word_outside_vocabulary_is_rejected() {
    let mut packet = seed();
    packet.evidence_bindings[0]
        .constrained_grammar
        .state_role_word = "totally_made_up".to_owned();
    assert!(violations_of(&packet).contains(&"state_role_word_outside_vocabulary"));
}

#[test]
fn unconstrained_write_disposition_on_gate_role_is_rejected() {
    let mut packet = seed();
    let target = packet
        .evidence_bindings
        .iter()
        .position(|b| b.binding_id == "cse-ro-support")
        .unwrap();
    packet.evidence_bindings[target]
        .constrained_grammar
        .write_disposition_word = "directly_writable".to_owned();
    assert!(violations_of(&packet).contains(&"write_disposition_unconstrained_for_gate_role"));
}

#[test]
fn blocked_reason_class_mismatch_is_rejected() {
    let mut packet = seed();
    packet.evidence_bindings[0].blocked_write_reason =
        BlockedWriteReason::ManagedSourceRequiresDetach;
    assert!(violations_of(&packet).contains(&"blocked_reason_class_mismatch"));
}

#[test]
fn machine_readable_drift_is_rejected() {
    let mut packet = seed();
    packet.evidence_bindings[0]
        .dual_form
        .machine_readable
        .resolved_decision_token = "cancelled".to_owned();
    assert!(violations_of(&packet).contains(&"machine_readable_record_mismatch"));
}

#[test]
fn missing_human_readable_line_is_rejected() {
    let mut packet = seed();
    packet.evidence_bindings[0].dual_form.human_readable_line = String::new();
    let tokens = violations_of(&packet);
    assert!(tokens.contains(&"dual_form_incomplete"));
}

#[test]
fn redaction_without_omission_reason_is_rejected() {
    let mut packet = seed();
    let target = packet
        .evidence_bindings
        .iter()
        .position(|b| b.redaction.disposition.is_redacted())
        .unwrap();
    packet.evidence_bindings[target].redaction.omission_reason = None;
    let tokens = violations_of(&packet);
    assert!(tokens.contains(&"redaction_omission_reason_dropped"));
}

#[test]
fn not_redacted_with_stray_omission_reason_is_rejected() {
    let mut packet = seed();
    let target = packet
        .evidence_bindings
        .iter()
        .position(|b| !b.redaction.disposition.is_redacted())
        .unwrap();
    packet.evidence_bindings[target].redaction.omission_reason = Some("stray reason".to_owned());
    assert!(violations_of(&packet).contains(&"redaction_record_inconsistent"));
}

#[test]
fn redacted_dropping_state_class_is_rejected() {
    let mut packet = seed();
    let target = packet
        .evidence_bindings
        .iter()
        .position(|b| b.redaction.disposition.is_redacted())
        .unwrap();
    packet.evidence_bindings[target].preserves_state_class_and_fallback_when_redacted = false;
    assert!(violations_of(&packet).contains(&"redacted_state_class_or_fallback_dropped"));
}

#[test]
fn resolved_decision_inconsistent_is_rejected() {
    let mut packet = seed();
    // Set a non-cancel decision that does not match the chosen fallback path.
    let target = packet
        .evidence_bindings
        .iter()
        .position(|b| b.object_class == M5ConstrainedFileStateObject::ReadOnly)
        .unwrap();
    packet.evidence_bindings[target].resolved_decision =
        ResolvedFallbackDecision::DetachedFromManagedSource;
    assert!(violations_of(&packet).contains(&"resolved_decision_inconsistent"));
}

#[test]
fn constrained_state_not_classified_is_rejected() {
    let mut packet = seed();
    packet.evidence_bindings[0].constrained_state_explicitly_classified = false;
    assert!(violations_of(&packet).contains(&"constrained_state_not_classified"));
}

#[test]
fn missing_reviewed_fallback_replay_is_rejected() {
    let mut packet = seed();
    packet.evidence_bindings[0]
        .allowed_actions
        .retain(|a| *a != EvidenceAction::OpenReviewedFallbackReplay);
    assert!(violations_of(&packet).contains(&"reviewed_fallback_replay_missing"));
}

#[test]
fn missing_base_action_is_rejected() {
    let mut packet = seed();
    packet.evidence_bindings[0]
        .allowed_actions
        .retain(|a| *a != EvidenceAction::ExportEvidencePacket);
    assert!(violations_of(&packet).contains(&"base_actions_missing"));
}

#[test]
fn undiscoverable_accessibility_state_is_rejected() {
    let mut packet = seed();
    packet.evidence_bindings[0].accessibility_routes =
        vec![M5ConstrainedFileStateAccessibilityRoute::HighZoomReflow];
    assert!(violations_of(&packet).contains(&"accessibility_state_undiscoverable"));
}

#[test]
fn reuse_below_two_channels_is_rejected() {
    let mut packet = seed();
    let mut kept_one = false;
    packet.evidence_bindings.retain(|b| {
        if b.object_class == M5ConstrainedFileStateObject::Projection {
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
        .evidence_bindings
        .iter()
        .position(|b| consumer_must_reference_canonical(b.consumer))
        .unwrap();
    packet.evidence_bindings[target].source_contract_refs =
        vec![M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF.to_owned()];
    assert!(violations_of(&packet).contains(&"support_export_reference_missing"));
}

#[test]
fn incomplete_preserved_versus_lost_is_rejected() {
    let mut packet = seed();
    packet.evidence_bindings[0].preserved_versus_lost.lost = String::new();
    assert!(violations_of(&packet).contains(&"preserved_versus_lost_incomplete"));
}

#[test]
fn each_guardrail_is_enforced() {
    let cases: [(fn(&mut EvidencePacketBinding), &str); 5] = [
        (
            |b| b.flattens_constrained_state_into_generic_read_only_language = true,
            "human_readable_flattens_state_class",
        ),
        (
            |b| b.lets_one_constrained_state_class_hide_another = true,
            "lets_one_constrained_state_class_hide_another",
        ),
        (
            |b| b.silently_falls_back_to_lossy_direct_write = true,
            "silently_falls_back_to_lossy_direct_write",
        ),
        (
            |b| b.gives_ai_automation_import_or_repair_a_hidden_bypass = true,
            "gives_ai_automation_import_or_repair_a_hidden_bypass",
        ),
        (
            |b| b.presents_as_directly_writable_or_hides_recovery_path = true,
            "presents_as_directly_writable_or_hides_recovery_path",
        ),
    ];
    for (mutate, token) in cases {
        let mut packet = seed();
        mutate(&mut packet.evidence_bindings[0]);
        assert!(violations_of(&packet).contains(&token), "expected {token}");
    }
}

#[test]
fn object_class_coverage_gap_is_rejected() {
    let mut packet = seed();
    packet
        .evidence_bindings
        .retain(|b| b.object_class != M5ConstrainedFileStateObject::CapturedSnapshot);
    assert!(violations_of(&packet).contains(&"object_class_coverage_missing"));
}

#[test]
fn channel_coverage_gap_is_rejected() {
    let mut packet = seed();
    packet
        .evidence_bindings
        .retain(|b| b.channel != EvidencePacketChannel::DocsHelpExample);
    assert!(violations_of(&packet).contains(&"channel_coverage_missing"));
}

#[test]
fn missing_support_bundle_is_rejected() {
    let mut packet = seed();
    packet
        .evidence_bindings
        .retain(|b| b.channel != EvidencePacketChannel::SupportBundle);
    let tokens = violations_of(&packet);
    assert!(tokens.contains(&"support_bundle_missing"));
}

#[test]
fn redaction_coverage_gap_is_rejected() {
    let mut packet = seed();
    for binding in &mut packet.evidence_bindings {
        binding.redaction = RedactionRecord {
            disposition: RedactionDisposition::NotRedacted,
            omission_reason: None,
            state_class_preserved: true,
            fallback_decision_preserved: true,
        };
    }
    assert!(violations_of(&packet).contains(&"redaction_coverage_missing"));
}

#[test]
fn resolved_decision_coverage_gap_is_rejected() {
    let mut packet = seed();
    packet
        .evidence_bindings
        .retain(|b| b.resolved_decision != ResolvedFallbackDecision::Cancelled);
    assert!(violations_of(&packet).contains(&"resolved_decision_coverage_missing"));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = seed();
    packet
        .source_contract_refs
        .retain(|r| r != M5_CONSTRAINED_STATE_EVIDENCE_DOC_REF);
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
    assert_eq!(lines.len(), 1 + packet.evidence_bindings.len());
    assert!(lines[0].starts_with(
        "object_class,channel,consumer,blocked_write_reason,chosen_fallback_path,resolved_decision,write_disposition,redaction_disposition,entry_id"
    ));
}

#[test]
fn csv_preserves_reason_and_decision_vocabulary() {
    let packet = seed();
    let csv = packet.render_matrix_csv();
    assert!(csv.contains(",read_only_path_not_directly_writable,"));
    assert!(csv.contains(",generated_artifact_regenerate_only,"));
    assert!(csv.contains(",managed_source_requires_detach,"));
    assert!(csv.contains(",cancelled,"));
    assert!(csv.contains(",redacted_keep_state_class_and_fallback,"));
    assert!(csv.contains(",support_bundle,"));
    assert!(csv.contains(",review_export_packet,"));
}

#[test]
fn markdown_summary_lists_every_binding() {
    let packet = seed();
    let summary = packet.render_markdown_summary();
    for binding in &packet.evidence_bindings {
        assert!(summary.contains(&binding.binding_id));
    }
}

#[test]
fn health_dashboard_surfaces_the_lane() {
    let packet = seed();
    let dashboard = packet.render_health_dashboard();
    let value: serde_json::Value = serde_json::from_str(&dashboard).expect("dashboard parses");
    assert_eq!(
        value["record_kind"],
        serde_json::json!(M5_CONSTRAINED_STATE_EVIDENCE_DASHBOARD_RECORD_KIND)
    );
    assert_eq!(
        value["support_export_ref"],
        serde_json::json!(M5_CONSTRAINED_STATE_EVIDENCE_ARTIFACT_REF)
    );
    assert_eq!(value["channels"].as_array().unwrap().len(), 4);
    assert_eq!(value["blocked_write_reasons"].as_array().unwrap().len(), 6);
    assert_eq!(value["resolved_decisions"].as_array().unwrap().len(), 6);
    assert_eq!(value["redaction_dispositions"].as_array().unwrap().len(), 2);
    assert_eq!(value["entry_families"].as_array().unwrap().len(), 6);
    assert_eq!(value["redacted_binding_count"], serde_json::json!(3));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_constrained_state_evidence_export()
        .expect("checked M5 constrained-state evidence export validates");
    assert_eq!(from_disk.packet_id, M5_CONSTRAINED_STATE_EVIDENCE_PACKET_ID);
    assert_eq!(
        from_disk,
        seed(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_health_dashboard_matches_render() {
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../dashboards/m5-constrained-state-evidence-health.json"
    ));
    let on_disk_value: serde_json::Value =
        serde_json::from_str(on_disk).expect("checked dashboard parses");
    let rendered_value: serde_json::Value =
        serde_json::from_str(&seed().render_health_dashboard()).expect("rendered dashboard parses");
    assert_eq!(
        on_disk_value, rendered_value,
        "checked health dashboard drifted from the render"
    );
}

#[test]
fn narrowed_fixture_variants_validate_and_keep_coverage() {
    let redaction = seeded_m5_constrained_state_evidence_packets_redaction_narrowed();
    assert!(
        redaction.validate().is_empty(),
        "{:?}",
        violations_of(&redaction)
    );
    assert_eq!(redaction.evidence_bindings.len(), 16);

    let cancelled = seeded_m5_constrained_state_evidence_packets_cancelled_decision_narrowed();
    assert!(
        cancelled.validate().is_empty(),
        "{:?}",
        violations_of(&cancelled)
    );
    assert_eq!(cancelled.evidence_bindings.len(), 16);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_builders() {
    let redaction: M5ConstrainedStateEvidencePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/editor/m5-constrained-state-evidence/redaction_narrowed.json"
    )))
    .expect("redaction fixture parses");
    assert!(redaction.validate().is_empty());
    assert_eq!(
        redaction,
        seeded_m5_constrained_state_evidence_packets_redaction_narrowed()
    );

    let cancelled: M5ConstrainedStateEvidencePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/editor/m5-constrained-state-evidence/cancelled_decision_narrowed.json"
    )))
    .expect("cancelled fixture parses");
    assert!(cancelled.validate().is_empty());
    assert_eq!(
        cancelled,
        seeded_m5_constrained_state_evidence_packets_cancelled_decision_narrowed()
    );
}
