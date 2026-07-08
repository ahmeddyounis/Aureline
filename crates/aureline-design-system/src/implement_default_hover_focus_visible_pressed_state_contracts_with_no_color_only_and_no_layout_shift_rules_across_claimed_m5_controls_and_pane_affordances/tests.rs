use super::*;

fn default_button() -> M5InteractiveStateResolutionInput {
    M5InteractiveStateResolutionInput {
        control_kind: M5InteractiveControlKind::PushButton,
        interactive_state: M5SharedComponentStateClass::Default,
        pointer_available: true,
        keyboard_focus_origin: false,
        reduced_motion_active: false,
        high_contrast_active: false,
        control_identity_ref: "control:command-bar.primary-action".to_owned(),
        state_style_ref: "token:state.push_button.default".to_owned(),
    }
}

// ---- interactive-state resolver -----------------------------------------

#[test]
fn default_state_is_resting_default_with_persistent_label() {
    let resolved = resolve_interactive_state_contract(&default_button()).expect("resolves");
    assert_eq!(
        resolved.presentation,
        M5InteractiveStatePresentation::RestingDefault
    );
    assert!(!resolved.focus_ring_shown);
    assert_eq!(
        resolved.required_non_color_cues,
        vec![M5InteractiveStateCue::PersistentStateLabel]
    );
    assert!(!resolved.required_non_color_cues.is_empty());
    assert!(resolved.no_color_only_signaling);
    assert!(resolved.stable_hit_target);
    assert!(resolved.no_interaction_breaking_layout_shift);
    assert!(resolved.focus_visible_for_keyboard);
    assert!(resolved.reduced_motion_safe);
    assert!(resolved.high_contrast_safe);
    assert!(resolved.driven_by_shared_state_contract);
    assert!(resolved
        .interaction_input_routes
        .contains(&M5InteractionInputRoute::KeyboardFocus));
    assert!(resolved
        .interaction_input_routes
        .contains(&M5InteractionInputRoute::AssistiveTechAnnounced));
}

#[test]
fn hover_state_carries_elevation_and_cursor_not_color() {
    let resolved = resolve_interactive_state_contract(&M5InteractiveStateResolutionInput {
        interactive_state: M5SharedComponentStateClass::Hover,
        ..default_button()
    })
    .expect("resolves");
    assert_eq!(
        resolved.presentation,
        M5InteractiveStatePresentation::PointerHover
    );
    assert!(resolved
        .required_non_color_cues
        .contains(&M5InteractiveStateCue::ElevationOrShadowShift));
    assert!(resolved
        .required_non_color_cues
        .contains(&M5InteractiveStateCue::PointerCursorAffordance));
    assert!(resolved
        .interaction_input_routes
        .contains(&M5InteractionInputRoute::PointerHover));
    // Hover is still keyboard-reachable, never pointer-only.
    assert!(resolved
        .interaction_input_routes
        .contains(&M5InteractionInputRoute::KeyboardFocus));
    assert!(!resolved.focus_ring_shown);
}

#[test]
fn focus_visible_shows_ring_only_from_keyboard_origin() {
    let keyboard = resolve_interactive_state_contract(&M5InteractiveStateResolutionInput {
        interactive_state: M5SharedComponentStateClass::FocusVisible,
        keyboard_focus_origin: true,
        pointer_available: false,
        ..default_button()
    })
    .expect("resolves");
    assert_eq!(
        keyboard.presentation,
        M5InteractiveStatePresentation::KeyboardFocusVisible
    );
    assert!(keyboard.focus_ring_shown);
    assert!(keyboard
        .required_non_color_cues
        .contains(&M5InteractiveStateCue::FocusRingOutline));
    assert!(keyboard
        .interaction_input_routes
        .contains(&M5InteractionInputRoute::FocusVisibleRing));

    // A pointer-origin focus keeps focus present and announced but suppresses the visible ring.
    let pointer = resolve_interactive_state_contract(&M5InteractiveStateResolutionInput {
        interactive_state: M5SharedComponentStateClass::FocusVisible,
        keyboard_focus_origin: false,
        ..default_button()
    })
    .expect("resolves");
    assert_eq!(
        pointer.presentation,
        M5InteractiveStatePresentation::KeyboardFocusVisible
    );
    assert!(!pointer.focus_ring_shown);
    assert!(pointer.focus_visible_for_keyboard);
}

#[test]
fn pressed_state_carries_inset_not_color() {
    let resolved = resolve_interactive_state_contract(&M5InteractiveStateResolutionInput {
        interactive_state: M5SharedComponentStateClass::PressedActive,
        ..default_button()
    })
    .expect("resolves");
    assert_eq!(
        resolved.presentation,
        M5InteractiveStatePresentation::PressedOrActive
    );
    assert!(resolved
        .required_non_color_cues
        .contains(&M5InteractiveStateCue::PressInsetOrDepression));
    assert!(resolved
        .interaction_input_routes
        .contains(&M5InteractionInputRoute::PressActivation));
}

#[test]
fn resolver_rejects_non_interactive_state() {
    for state in [
        M5SharedComponentStateClass::Selected,
        M5SharedComponentStateClass::Current,
        M5SharedComponentStateClass::Disabled,
        M5SharedComponentStateClass::ReadOnly,
        M5SharedComponentStateClass::Loading,
        M5SharedComponentStateClass::Pending,
        M5SharedComponentStateClass::WarningError,
        M5SharedComponentStateClass::Locked,
        M5SharedComponentStateClass::Degraded,
    ] {
        assert_eq!(
            resolve_interactive_state_contract(&M5InteractiveStateResolutionInput {
                interactive_state: state,
                ..default_button()
            }),
            Err(M5InteractiveStateResolutionError::NonInteractiveState),
            "state {} was not rejected as non-interactive",
            state.as_str()
        );
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_interactive_state_contract(&M5InteractiveStateResolutionInput {
            control_identity_ref: " ".to_owned(),
            ..default_button()
        }),
        Err(M5InteractiveStateResolutionError::EmptyControlIdentity)
    );
    assert_eq!(
        resolve_interactive_state_contract(&M5InteractiveStateResolutionInput {
            state_style_ref: "".to_owned(),
            ..default_button()
        }),
        Err(M5InteractiveStateResolutionError::EmptyStateStyleRef)
    );
    assert_eq!(
        resolve_interactive_state_contract(&M5InteractiveStateResolutionInput {
            state_style_ref: "token:https://evil.example/x".to_owned(),
            ..default_button()
        }),
        Err(M5InteractiveStateResolutionError::ForbiddenStateMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_interactive_state_contract_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_INTERACTIVE_STATE_CONTRACT_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_control_kind() {
    let packet = seeded_m5_interactive_state_contract_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.control_kind).collect();
    for control in M5InteractiveControlKind::ALL {
        assert!(
            present.contains(&control),
            "missing control kind {}",
            control.as_str()
        );
    }
    assert_eq!(packet.rows.len(), M5InteractiveControlKind::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_export_and_labels() {
    let packet = seeded_m5_interactive_state_contract_packet();
    for row in &packet.rows {
        for part in M5InteractiveStateAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5InteractiveStateExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for label in M5ComponentStateRequiredLabel::MANDATORY {
            assert!(row.required_labels.contains(&label));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5ComponentStateAccessibilityRoute::KeyboardFocusable));
        assert!(row
            .accessibility_routes
            .contains(&M5ComponentStateAccessibilityRoute::NonColorEncoded));
        assert!(!row.state_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_interactive_state_contract_packet();
    let cases: Vec<&M5InteractiveStateResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.state_examples.iter())
        .collect();

    for state in interactive_states() {
        assert!(
            cases.iter().any(|c| c.resolved.interactive_state == state),
            "no example exercises interactive state {}",
            state.as_str()
        );
    }
    for posture in M5InteractiveStatePresentation::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.presentation == posture),
            "no example exercises presentation {}",
            posture.as_str()
        );
    }
    for cue in M5InteractiveStateCue::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.required_non_color_cues.contains(&cue)),
            "no example exercises non-color cue {}",
            cue.as_str()
        );
    }
    for route in M5InteractionInputRoute::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.interaction_input_routes.contains(&route)),
            "no example exercises interaction route {}",
            route.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity_and_guarantees() {
    let packet = seeded_m5_interactive_state_contract_packet();
    for row in &packet.rows {
        for case in &row.state_examples {
            assert!(
                case.is_self_consistent(),
                "state case for {} drifted",
                row.control_kind.as_str()
            );
            assert!(
                case.preserves_identity(),
                "state case for {} lost identity",
                row.control_kind.as_str()
            );
            assert!(
                case.preserves_guarantees(),
                "state case for {} lost a guarantee",
                row.control_kind.as_str()
            );
        }
    }
}

#[test]
fn missing_control_kind_fails() {
    let mut packet = seeded_m5_interactive_state_contract_packet();
    packet
        .rows
        .retain(|row| row.control_kind != M5InteractiveControlKind::PaneSplitter);
    assert!(packet
        .validate()
        .contains(&M5InteractiveStateContractViolation::RequiredControlMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_interactive_state_contract_packet();
    packet.vocabulary_set.presentations.pop();
    assert!(packet
        .validate()
        .contains(&M5InteractiveStateContractViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_interactive_state_contract_packet();
    packet.rows[0]
        .anatomy_parts
        .retain(|p| *p != M5InteractiveStateAnatomyPart::NonColorCueSetCue);
    assert!(packet
        .validate()
        .contains(&M5InteractiveStateContractViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_missing_fails() {
    let mut packet = seeded_m5_interactive_state_contract_packet();
    packet.rows[0]
        .export_fields
        .retain(|f| *f != M5InteractiveStateExportField::NonColorCues);
    assert!(packet
        .validate()
        .contains(&M5InteractiveStateContractViolation::MandatoryExportMissing));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_interactive_state_contract_packet();
    packet.rows[0]
        .required_labels
        .retain(|l| *l != M5ComponentStateRequiredLabel::KeyboardRoute);
    assert!(packet
        .validate()
        .contains(&M5InteractiveStateContractViolation::MandatoryLabelMissing));
}

#[test]
fn accessibility_route_missing_fails() {
    let mut packet = seeded_m5_interactive_state_contract_packet();
    packet.rows[0]
        .accessibility_routes
        .retain(|r| *r != M5ComponentStateAccessibilityRoute::NonColorEncoded);
    assert!(packet
        .validate()
        .contains(&M5InteractiveStateContractViolation::AccessibilityRouteMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_interactive_state_contract_packet();
    packet.rows[0].state_examples[0].resolved.focus_ring_shown = true;
    assert!(packet
        .validate()
        .contains(&M5InteractiveStateContractViolation::ExampleResolutionDrift));
}

#[test]
fn state_example_missing_fails() {
    let mut packet = seeded_m5_interactive_state_contract_packet();
    packet.rows[1].state_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5InteractiveStateContractViolation::StateExampleMissing));
}

#[test]
fn interactive_state_coverage_unproven_fails() {
    let mut packet = seeded_m5_interactive_state_contract_packet();
    for row in &mut packet.rows {
        row.state_examples = vec![M5InteractiveStateResolutionCase::resolved(default_button())];
    }
    assert!(packet
        .validate()
        .contains(&M5InteractiveStateContractViolation::InteractiveStateCoverageUnproven));
}

#[test]
fn presentation_and_cue_and_route_coverage_unproven_fails() {
    let mut packet = seeded_m5_interactive_state_contract_packet();
    // Every example a resting-default → no hover/focus/pressed posture, no focus-ring/inset cue,
    // no pointer-hover/focus-ring/press route.
    for row in &mut packet.rows {
        row.state_examples = vec![M5InteractiveStateResolutionCase::resolved(default_button())];
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5InteractiveStateContractViolation::PresentationCoverageUnproven));
    assert!(violations.contains(&M5InteractiveStateContractViolation::CueCoverageUnproven));
    assert!(violations.contains(&M5InteractiveStateContractViolation::RouteCoverageUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_interactive_state_contract_packet();
    packet.rows[0].signals_state_by_color_only = true;
    assert!(packet
        .validate()
        .contains(&M5InteractiveStateContractViolation::RowInvariantViolated));
}

#[test]
fn stable_control_missing_proof_fails() {
    let mut packet = seeded_m5_interactive_state_contract_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5InteractiveStateContractViolation::StableControlMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_interactive_state_contract_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5InteractiveStateContractViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_interactive_state_contract_packet();
    packet.governance_review.state_meaning_never_color_only = false;
    assert!(packet
        .validate()
        .contains(&M5InteractiveStateContractViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_interactive_state_contract_packet();
    packet
        .consumer_projection
        .non_color_cue_set_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5InteractiveStateContractViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_interactive_state_contract_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5InteractiveStateContractViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_interactive_state_contract_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5InteractiveStateContractViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_control_kind() {
    let summary = seeded_m5_interactive_state_contract_packet().render_markdown_summary();
    for control in M5InteractiveControlKind::ALL {
        assert!(
            summary.contains(control.label()),
            "summary missing control {}",
            control.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_control() {
    let csv = seeded_m5_interactive_state_contract_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5InteractiveControlKind::ALL.len());
    assert!(lines[0].starts_with("control_kind,qualification,owner,"));
    for control in M5InteractiveControlKind::ALL {
        assert!(
            csv.contains(control.as_str()),
            "csv missing control {}",
            control.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_interactive_state_contract_export()
        .expect("checked M5 interactive state contract primitive export validates");
    assert_eq!(from_disk.packet_id, M5_INTERACTIVE_STATE_CONTRACT_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_interactive_state_contract_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_controls_visible() {
    for packet in [
        seeded_m5_interactive_state_contract_pane_splitter_beta_narrowed(),
        seeded_m5_interactive_state_contract_quick_action_card_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.rows.len(), M5InteractiveControlKind::ALL.len());
    }

    let splitter = seeded_m5_interactive_state_contract_pane_splitter_beta_narrowed();
    let row = splitter
        .rows
        .iter()
        .find(|r| r.control_kind == M5InteractiveControlKind::PaneSplitter)
        .expect("pane-splitter row present");
    assert_eq!(row.qualification, M5ComponentStateQualificationClass::Beta);

    let card = seeded_m5_interactive_state_contract_quick_action_card_preview_narrowed();
    let row = card
        .rows
        .iter()
        .find(|r| r.control_kind == M5InteractiveControlKind::QuickActionCard)
        .expect("quick-action-card row present");
    assert_eq!(
        row.qualification,
        M5ComponentStateQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let splitter: M5InteractiveStateContractPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-interactive-state-contract-primitive/pane_splitter_beta_narrowed.json"
    )))
    .expect("pane-splitter fixture parses");
    assert!(splitter.validate().is_empty());
    assert_eq!(
        splitter,
        seeded_m5_interactive_state_contract_pane_splitter_beta_narrowed()
    );

    let card: M5InteractiveStateContractPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-interactive-state-contract-primitive/quick_action_card_preview_narrowed.json"
    )))
    .expect("quick-action-card fixture parses");
    assert!(card.validate().is_empty());
    assert_eq!(
        card,
        seeded_m5_interactive_state_contract_quick_action_card_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_interactive_state_contract_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
