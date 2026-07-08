use super::*;

fn ready_leader_sequence() -> M5SequenceHelpStripResolutionInput {
    M5SequenceHelpStripResolutionInput {
        help_state: M5SequenceHelpState::Ready,
        step_kind: M5SequenceStepKind::LeaderKey,
        command_backing: M5CommandBackingState::KeybindingRoute,
        current_mode_or_leader_ref: "Leader (Space)".to_owned(),
        valid_next_keys: vec!["f".to_owned(), "g".to_owned(), "s".to_owned()],
        cancel_key: "Esc".to_owned(),
        example_command_ref: Some("command:leader.find-file".to_owned()),
        screen_reader_announcement: "Leader active. Press f, g, or s. Escape to cancel.".to_owned(),
        cheat_sheet_ref: "cheatsheet:leader-keys".to_owned(),
        strip_identity_ref: "strip:leader-overlay:leader-root".to_owned(),
    }
}

// ---- sequence-help-strip resolver ---------------------------------------

#[test]
fn ready_leader_sequence_shows_next_keys_and_stays_keyboard_first() {
    let resolved = resolve_sequence_help_strip(&ready_leader_sequence()).expect("resolves");
    assert_eq!(resolved.help_posture, M5SequenceHelpPosture::ReadyForInput);
    assert!(resolved.is_awaiting_more);
    assert!(!resolved.is_dead_end);
    assert!(!resolved.is_ambiguous);
    assert!(resolved.shows_next_keys);
    assert!(resolved.is_command_backed);
    assert!(resolved.example_command_available);
    assert!(resolved.cancel_available);
    assert!(resolved.cheat_sheet_available);
    assert!(resolved.shows_current_mode_or_leader);
    assert!(resolved.explains_next_keys_or_dead_end);
    assert!(resolved.shows_cancel_key);
    assert!(resolved.never_requires_pointer_hover);
    assert!(resolved.provides_screen_reader_announcement);
    assert!(resolved.keeps_full_cheat_sheet_reachable);
    assert!(resolved.preserves_command_backing_honestly);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5SequenceHelpAction::ShowValidNextKeys,
            M5SequenceHelpAction::RunExampleCommand,
            M5SequenceHelpAction::CancelSequence,
            M5SequenceHelpAction::OpenFullCheatSheet,
        ]
    );
}

#[test]
fn dead_end_no_binding_never_fails_silently() {
    let resolved = resolve_sequence_help_strip(&M5SequenceHelpStripResolutionInput {
        help_state: M5SequenceHelpState::NoBinding,
        step_kind: M5SequenceStepKind::PrefixArgument,
        command_backing: M5CommandBackingState::NoCommandBacking,
        valid_next_keys: vec![],
        example_command_ref: None,
        ..ready_leader_sequence()
    })
    .expect("resolves");
    assert_eq!(resolved.help_posture, M5SequenceHelpPosture::UnboundDeadEnd);
    assert!(resolved.is_dead_end);
    assert!(!resolved.is_awaiting_more);
    assert!(!resolved.shows_next_keys);
    assert!(!resolved.is_command_backed);
    assert!(!resolved.example_command_available);
    // A dead end still keeps cancel and the full cheat sheet reachable — never silent.
    assert!(resolved.cancel_available);
    assert!(resolved.cheat_sheet_available);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5SequenceHelpAction::CancelSequence,
            M5SequenceHelpAction::OpenFullCheatSheet,
        ]
    );
}

#[test]
fn conflicting_binding_offers_resolve_action() {
    let resolved = resolve_sequence_help_strip(&M5SequenceHelpStripResolutionInput {
        help_state: M5SequenceHelpState::ConflictingBinding,
        step_kind: M5SequenceStepKind::Chord,
        command_backing: M5CommandBackingState::PaletteEntry,
        valid_next_keys: vec!["1".to_owned(), "2".to_owned()],
        example_command_ref: Some("command:palette.resolve-binding".to_owned()),
        ..ready_leader_sequence()
    })
    .expect("resolves");
    assert_eq!(
        resolved.help_posture,
        M5SequenceHelpPosture::ConflictingBinding
    );
    assert!(resolved.is_ambiguous);
    assert!(resolved
        .available_actions
        .contains(&M5SequenceHelpAction::ResolveConflictingBinding));
    assert!(resolved
        .available_actions
        .contains(&M5SequenceHelpAction::CancelSequence));
    assert!(resolved
        .available_actions
        .contains(&M5SequenceHelpAction::OpenFullCheatSheet));
}

#[test]
fn disabled_sequence_is_named_and_not_awaiting() {
    let resolved = resolve_sequence_help_strip(&M5SequenceHelpStripResolutionInput {
        help_state: M5SequenceHelpState::DisabledInContext,
        step_kind: M5SequenceStepKind::TerminalAction,
        command_backing: M5CommandBackingState::UnboundHint,
        valid_next_keys: vec![],
        example_command_ref: Some("command:editor.record-macro".to_owned()),
        ..ready_leader_sequence()
    })
    .expect("resolves");
    assert_eq!(
        resolved.help_posture,
        M5SequenceHelpPosture::DisabledInContext
    );
    assert!(resolved.is_disabled);
    assert!(!resolved.is_awaiting_more);
    assert!(resolved.is_command_backed);
    assert!(resolved.example_command_available);
}

#[test]
fn open_sequence_without_next_keys_is_rejected() {
    for state in [
        M5SequenceHelpState::Ready,
        M5SequenceHelpState::AwaitingNextKey,
        M5SequenceHelpState::PartialMatch,
    ] {
        assert_eq!(
            resolve_sequence_help_strip(&M5SequenceHelpStripResolutionInput {
                help_state: state,
                valid_next_keys: vec![],
                ..ready_leader_sequence()
            }),
            Err(M5SequenceHelpStripResolutionError::MissingNextKeysForOpenSequence),
            "{} without next keys should be rejected",
            M5SequenceHelpPosture::from_state(state).as_str()
        );
    }
}

#[test]
fn command_backed_state_without_example_is_rejected() {
    assert_eq!(
        resolve_sequence_help_strip(&M5SequenceHelpStripResolutionInput {
            example_command_ref: None,
            ..ready_leader_sequence()
        }),
        Err(M5SequenceHelpStripResolutionError::MissingExampleForBackedState)
    );
    assert_eq!(
        resolve_sequence_help_strip(&M5SequenceHelpStripResolutionInput {
            example_command_ref: Some("  ".to_owned()),
            ..ready_leader_sequence()
        }),
        Err(M5SequenceHelpStripResolutionError::MissingExampleForBackedState)
    );
}

#[test]
fn unbacked_state_with_example_is_rejected() {
    assert_eq!(
        resolve_sequence_help_strip(&M5SequenceHelpStripResolutionInput {
            help_state: M5SequenceHelpState::NoBinding,
            command_backing: M5CommandBackingState::NoCommandBacking,
            valid_next_keys: vec![],
            example_command_ref: Some("command:something".to_owned()),
            ..ready_leader_sequence()
        }),
        Err(M5SequenceHelpStripResolutionError::ExampleCommandOnUnbackedState)
    );
}

#[test]
fn resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_sequence_help_strip(&M5SequenceHelpStripResolutionInput {
            current_mode_or_leader_ref: " ".to_owned(),
            ..ready_leader_sequence()
        }),
        Err(M5SequenceHelpStripResolutionError::EmptyCurrentModeOrLeader)
    );
    assert_eq!(
        resolve_sequence_help_strip(&M5SequenceHelpStripResolutionInput {
            cancel_key: "".to_owned(),
            ..ready_leader_sequence()
        }),
        Err(M5SequenceHelpStripResolutionError::EmptyCancelKey)
    );
    assert_eq!(
        resolve_sequence_help_strip(&M5SequenceHelpStripResolutionInput {
            screen_reader_announcement: "".to_owned(),
            ..ready_leader_sequence()
        }),
        Err(M5SequenceHelpStripResolutionError::EmptyScreenReaderAnnouncement)
    );
    assert_eq!(
        resolve_sequence_help_strip(&M5SequenceHelpStripResolutionInput {
            cheat_sheet_ref: "".to_owned(),
            ..ready_leader_sequence()
        }),
        Err(M5SequenceHelpStripResolutionError::EmptyCheatSheetRef)
    );
    assert_eq!(
        resolve_sequence_help_strip(&M5SequenceHelpStripResolutionInput {
            strip_identity_ref: "".to_owned(),
            ..ready_leader_sequence()
        }),
        Err(M5SequenceHelpStripResolutionError::EmptyStripIdentity)
    );
    assert_eq!(
        resolve_sequence_help_strip(&M5SequenceHelpStripResolutionInput {
            example_command_ref: Some("command:https://evil.example/x".to_owned()),
            ..ready_leader_sequence()
        }),
        Err(M5SequenceHelpStripResolutionError::ForbiddenSequenceMaterial)
    );
}

#[test]
fn posture_maps_one_to_one_from_help_state() {
    for state in M5SequenceHelpState::ALL {
        assert_eq!(
            M5SequenceHelpPosture::from_state(state).as_str(),
            match state {
                M5SequenceHelpState::Ready => "ready_for_input",
                M5SequenceHelpState::AwaitingNextKey => "awaiting_next_key",
                M5SequenceHelpState::PartialMatch => "partial_sequence",
                M5SequenceHelpState::NoBinding => "unbound_dead_end",
                M5SequenceHelpState::ConflictingBinding => "conflicting_binding",
                M5SequenceHelpState::DisabledInContext => "disabled_in_context",
            }
        );
    }
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_sequence_help_strip_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SEQUENCE_HELP_STRIP_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_sequence_help_strip_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5SequenceHelpConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(packet.rows.len(), M5SequenceHelpConsumerSurface::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_sequence_help_strip_packet();
    for row in &packet.rows {
        for part in M5SequenceHelpAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5SequenceHelpExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5TeachingAccessibilityRoute::KeyboardFocusable));
        assert!(!row.sequence_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_sequence_help_strip_packet();
    let cases: Vec<&M5SequenceHelpStripResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.sequence_examples.iter())
        .collect();

    for state in M5SequenceHelpState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.help_state == state),
            "no example exercises help state {}",
            state.as_str()
        );
    }
    for kind in M5SequenceStepKind::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.step_kind == kind),
            "no example exercises step kind {}",
            kind.as_str()
        );
    }
    for action in M5SequenceHelpAction::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises action {}",
            action.as_str()
        );
    }
}

#[test]
fn some_dead_end_or_ambiguous_keeps_cancel_and_cheat_sheet() {
    let packet = seeded_m5_sequence_help_strip_packet();
    assert!(packet
        .rows
        .iter()
        .flat_map(|row| row.sequence_examples.iter())
        .any(|c| (c.resolved.is_dead_end || c.resolved.is_ambiguous)
            && c.resolved.cancel_available
            && c.resolved.cheat_sheet_available));
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity_and_parity() {
    let packet = seeded_m5_sequence_help_strip_packet();
    for row in &packet.rows {
        for case in &row.sequence_examples {
            assert!(
                case.is_self_consistent(),
                "sequence case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "sequence case for {} lost identity",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_keyboard_parity(),
                "sequence case for {} lost keyboard parity",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_sequence_help_strip_packet();
    packet
        .rows
        .retain(|row| row.consumer_surface != M5SequenceHelpConsumerSurface::ModalOperatorStrip);
    assert!(packet
        .validate()
        .contains(&M5SequenceHelpStripViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_sequence_help_strip_packet();
    packet.vocabulary_set.help_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5SequenceHelpStripViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_sequence_help_strip_packet();
    packet.rows[0]
        .anatomy_parts
        .retain(|p| *p != M5SequenceHelpAnatomyPart::ValidNextKeysCue);
    assert!(packet
        .validate()
        .contains(&M5SequenceHelpStripViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_missing_fails() {
    let mut packet = seeded_m5_sequence_help_strip_packet();
    packet.rows[0]
        .export_fields
        .retain(|f| *f != M5SequenceHelpExportField::CancelKey);
    assert!(packet
        .validate()
        .contains(&M5SequenceHelpStripViolation::MandatoryExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_sequence_help_strip_packet();
    packet.rows[0].sequence_examples[0]
        .resolved
        .is_awaiting_more = false;
    assert!(packet
        .validate()
        .contains(&M5SequenceHelpStripViolation::ExampleResolutionDrift));
}

#[test]
fn sequence_example_missing_fails() {
    let mut packet = seeded_m5_sequence_help_strip_packet();
    packet.rows[1].sequence_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5SequenceHelpStripViolation::SequenceExampleMissing));
}

#[test]
fn help_state_coverage_unproven_fails() {
    let mut packet = seeded_m5_sequence_help_strip_packet();
    for row in &mut packet.rows {
        row.sequence_examples = vec![M5SequenceHelpStripResolutionCase::resolved(
            ready_leader_sequence(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5SequenceHelpStripViolation::HelpStateCoverageUnproven));
}

#[test]
fn step_kind_coverage_unproven_fails() {
    let mut packet = seeded_m5_sequence_help_strip_packet();
    for row in &mut packet.rows {
        row.sequence_examples = vec![M5SequenceHelpStripResolutionCase::resolved(
            ready_leader_sequence(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5SequenceHelpStripViolation::StepKindCoverageUnproven));
}

#[test]
fn posture_coverage_unproven_fails() {
    let mut packet = seeded_m5_sequence_help_strip_packet();
    // Every example ready-for-input → no dead-end, conflicting, or disabled posture.
    for row in &mut packet.rows {
        row.sequence_examples = vec![M5SequenceHelpStripResolutionCase::resolved(
            ready_leader_sequence(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5SequenceHelpStripViolation::PostureCoverageUnproven));
}

#[test]
fn action_coverage_unproven_fails() {
    let mut packet = seeded_m5_sequence_help_strip_packet();
    for row in &mut packet.rows {
        row.sequence_examples = vec![M5SequenceHelpStripResolutionCase::resolved(
            ready_leader_sequence(),
        )];
    }
    // ready_leader_sequence never exercises resolve-conflicting-binding.
    assert!(packet
        .validate()
        .contains(&M5SequenceHelpStripViolation::ActionCoverageUnproven));
}

#[test]
fn non_silent_parity_unproven_fails() {
    let mut packet = seeded_m5_sequence_help_strip_packet();
    // Only open (awaiting-more) examples → no dead-end or ambiguous case proves non-silence.
    for row in &mut packet.rows {
        row.sequence_examples = vec![M5SequenceHelpStripResolutionCase::resolved(
            ready_leader_sequence(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5SequenceHelpStripViolation::NonSilentParityUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_sequence_help_strip_packet();
    packet.rows[0].fails_silently_on_partial_or_ambiguous = true;
    assert!(packet
        .validate()
        .contains(&M5SequenceHelpStripViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_sequence_help_strip_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SequenceHelpStripViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_sequence_help_strip_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SequenceHelpStripViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_sequence_help_strip_packet();
    packet
        .governance_review
        .partial_or_ambiguous_sequences_never_fail_silently = false;
    assert!(packet
        .validate()
        .contains(&M5SequenceHelpStripViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_sequence_help_strip_packet();
    packet.consumer_projection.action_set_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5SequenceHelpStripViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_sequence_help_strip_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5SequenceHelpStripViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_sequence_help_strip_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5SequenceHelpStripViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_sequence_help_strip_packet().render_markdown_summary();
    for surface in M5SequenceHelpConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_sequence_help_strip_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5SequenceHelpConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5SequenceHelpConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_sequence_help_strip_export()
        .expect("checked M5 sequence help strip primitive export validates");
    assert_eq!(from_disk.packet_id, M5_SEQUENCE_HELP_STRIP_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_sequence_help_strip_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_sequence_help_strip_command_palette_sequence_hint_beta_narrowed(),
        seeded_m5_sequence_help_strip_support_sequence_export_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.rows.len(), M5SequenceHelpConsumerSurface::ALL.len());
    }

    let palette = seeded_m5_sequence_help_strip_command_palette_sequence_hint_beta_narrowed();
    let row = palette
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5SequenceHelpConsumerSurface::CommandPaletteSequenceHint)
        .expect("command-palette-sequence-hint row present");
    assert_eq!(row.qualification, M5TeachingQualificationClass::Beta);

    let support = seeded_m5_sequence_help_strip_support_sequence_export_preview_narrowed();
    let row = support
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5SequenceHelpConsumerSurface::SupportSequenceExport)
        .expect("support-sequence-export row present");
    assert_eq!(row.qualification, M5TeachingQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let palette: M5SequenceHelpStripPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-sequence-help-strip-primitive/command_palette_sequence_hint_beta_narrowed.json"
    )))
    .expect("command-palette-sequence-hint fixture parses");
    assert!(palette.validate().is_empty());
    assert_eq!(
        palette,
        seeded_m5_sequence_help_strip_command_palette_sequence_hint_beta_narrowed()
    );

    let support: M5SequenceHelpStripPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-sequence-help-strip-primitive/support_sequence_export_preview_narrowed.json"
    )))
    .expect("support-sequence-export fixture parses");
    assert!(support.validate().is_empty());
    assert_eq!(
        support,
        seeded_m5_sequence_help_strip_support_sequence_export_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_sequence_help_strip_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
