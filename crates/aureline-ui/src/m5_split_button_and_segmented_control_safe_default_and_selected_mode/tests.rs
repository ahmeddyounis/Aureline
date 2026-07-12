use super::*;

fn clean_split_input() -> M5SplitButtonResolutionInput {
    M5SplitButtonResolutionInput {
        split_button_id: "split:test".to_owned(),
        primary_action_label: "Submit request".to_owned(),
        default_posture: M5SplitDefaultPosture::PrimaryDefaultSafe,
        default_emphasis: M5ButtonEmphasis::Primary,
        emphasis_stated: true,
        disposition: M5CoreControlDisposition::Default,
        surface_context: M5SplitSegmentedSurfaceContext::PaneHeader,
        alternate_visibility: M5SplitAlternateVisibility::AdjacentMenuVisible,
        scope_impact: M5SplitScopeImpact::SingleTarget,
        scope_disclosed: true,
        stale_state_promoted_riskier_alternate: false,
        blocked_state_distinct: true,
        command_id: "command:forms.submit".to_owned(),
        command_route_available: true,
        proof_fresh: true,
    }
}

fn clean_segmented_input() -> M5SegmentedControlResolutionInput {
    M5SegmentedControlResolutionInput {
        segmented_control_id: "segmented:test".to_owned(),
        group_label: "Layout mode".to_owned(),
        selected_segment_label: "Comfortable".to_owned(),
        mode: M5SegmentedMode::ModeToggle,
        disposition: M5CoreControlDisposition::Default,
        surface_context: M5SplitSegmentedSurfaceContext::PaneHeader,
        selected_state_explicit: true,
        keyboard_cycling_available: true,
        oversized_segment_set: false,
        masquerades_as_navigation: false,
        scope_impact: M5SplitScopeImpact::SingleTarget,
        scope_disclosed: true,
        blocked_state_distinct: true,
        command_id: "command:forms.layout".to_owned(),
        command_route_available: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_split_button_segmented_control_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_PACKET_ID
    );
}

#[test]
fn split_clean_names_posture_and_is_safe() {
    let resolved = resolve_split_button(clean_split_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.default_is_safe_at_a_glance);
    assert!(resolved.alternate_visible);
    assert!(!resolved.stale_state_promoted_riskier_alternate);
    assert_eq!(resolved.default_posture, "primary_default_safe");
    assert_eq!(resolved.disposition, "default");
    assert_eq!(resolved.surface_context, "pane_header");
    assert_eq!(
        resolved.next_action,
        M5SplitSegmentedNextAction::OpenCommandDetail
    );
}

#[test]
fn split_primary_unstated_degrades() {
    let mut input = clean_split_input();
    input.primary_action_label = "   ".to_owned();
    assert_eq!(
        resolve_split_button(input).unwrap().degrade_reason,
        Some(M5SplitButtonDegradeReason::PrimaryActionUnstated)
    );
}

#[test]
fn split_riskier_alternate_default_degrades() {
    let mut input = clean_split_input();
    input.stale_state_promoted_riskier_alternate = true;
    assert_eq!(
        resolve_split_button(input).unwrap().degrade_reason,
        Some(M5SplitButtonDegradeReason::RiskierAlternateBecameDefault)
    );
}

#[test]
fn split_alternate_hidden_degrades() {
    let mut input = clean_split_input();
    input.alternate_visibility = M5SplitAlternateVisibility::AlternateHidden;
    let resolved = resolve_split_button(input).unwrap();
    assert!(!resolved.alternate_visible);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SplitButtonDegradeReason::AlternateHiddenBehindDefault)
    );
}

#[test]
fn split_broadened_scope_undisclosed_degrades() {
    let mut input = clean_split_input();
    input.scope_impact = M5SplitScopeImpact::WholeBatch;
    input.scope_disclosed = false;
    let resolved = resolve_split_button(input).unwrap();
    assert!(resolved.scope_needs_disclosure);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SplitButtonDegradeReason::BroadenedScopeUndisclosed)
    );
}

#[test]
fn split_disclosed_batch_scope_is_clean() {
    let mut input = clean_split_input();
    input.scope_impact = M5SplitScopeImpact::WholeBatch;
    input.scope_disclosed = true;
    let resolved = resolve_split_button(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.scope_needs_disclosure);
    assert!(resolved.scope_disclosed);
}

#[test]
fn split_locked_hidden_behind_disabled_degrades() {
    let mut input = clean_split_input();
    input.disposition = M5CoreControlDisposition::Locked;
    input.blocked_state_distinct = false;
    let resolved = resolve_split_button(input).unwrap();
    assert!(resolved.disposition_is_blocked);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SplitButtonDegradeReason::LockedOrDegradedHiddenBehindDisabled)
    );
}

#[test]
fn split_locked_distinct_is_clean() {
    let mut input = clean_split_input();
    input.disposition = M5CoreControlDisposition::Locked;
    input.blocked_state_distinct = true;
    let resolved = resolve_split_button(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.disposition_is_blocked);
    assert!(resolved.blocked_state_distinct);
}

#[test]
fn split_color_only_and_unresolved_degrade() {
    let mut input = clean_split_input();
    input.emphasis_stated = false;
    assert_eq!(
        resolve_split_button(input).unwrap().degrade_reason,
        Some(M5SplitButtonDegradeReason::EmphasisEncodedByColorAlone)
    );

    let mut input = clean_split_input();
    input.surface_context = M5SplitSegmentedSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_split_button(input).unwrap().degrade_reason,
        Some(M5SplitButtonDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_split_input();
    input.default_posture = M5SplitDefaultPosture::PostureUnknown;
    assert_eq!(
        resolve_split_button(input).unwrap().degrade_reason,
        Some(M5SplitButtonDegradeReason::DefaultPostureUnresolved)
    );

    let mut input = clean_split_input();
    input.alternate_visibility = M5SplitAlternateVisibility::VisibilityUnknown;
    assert_eq!(
        resolve_split_button(input).unwrap().degrade_reason,
        Some(M5SplitButtonDegradeReason::AlternateVisibilityUnresolved)
    );
}

#[test]
fn split_command_and_trace_degrade() {
    let mut input = clean_split_input();
    input.command_id = "   ".to_owned();
    assert_eq!(
        resolve_split_button(input).unwrap().degrade_reason,
        Some(M5SplitButtonDegradeReason::CommandBindingUnstated)
    );

    let mut input = clean_split_input();
    input.command_route_available = false;
    assert_eq!(
        resolve_split_button(input).unwrap().degrade_reason,
        Some(M5SplitButtonDegradeReason::CommandTracePathMissing)
    );
}

#[test]
fn split_empty_id_and_forbidden_material_error() {
    let mut input = clean_split_input();
    input.split_button_id = "".to_owned();
    assert_eq!(
        resolve_split_button(input).unwrap_err(),
        M5SplitSegmentedResolutionError::EmptySplitButtonId
    );

    let mut input = clean_split_input();
    input.primary_action_label = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_split_button(input).unwrap_err(),
        M5SplitSegmentedResolutionError::ForbiddenMaterial
    );
}

#[test]
fn segmented_clean_exposes_selected_mode() {
    let resolved = resolve_segmented_control(clean_segmented_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.selected_mode_explicit_at_a_glance);
    assert!(resolved.selected_segment_shown);
    assert!(resolved.keyboard_cycling_available);
    assert!(!resolved.masquerades_as_navigation);
    assert_eq!(resolved.mode, "mode_toggle");
    assert_eq!(resolved.selected_segment_label, "Comfortable");
}

#[test]
fn segmented_group_and_selected_unstated_degrade() {
    let mut input = clean_segmented_input();
    input.group_label = "  ".to_owned();
    assert_eq!(
        resolve_segmented_control(input).unwrap().degrade_reason,
        Some(M5SegmentedControlDegradeReason::GroupLabelUnstated)
    );

    let mut input = clean_segmented_input();
    input.selected_segment_label = "  ".to_owned();
    let resolved = resolve_segmented_control(input).unwrap();
    assert!(!resolved.selected_segment_shown);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SegmentedControlDegradeReason::SelectedSegmentUnstated)
    );
}

#[test]
fn segmented_stealth_navigation_degrades() {
    let mut input = clean_segmented_input();
    input.masquerades_as_navigation = true;
    assert_eq!(
        resolve_segmented_control(input).unwrap().degrade_reason,
        Some(M5SegmentedControlDegradeReason::UsedAsStealthNavigation)
    );
}

#[test]
fn segmented_selected_color_only_and_keyboard_missing_degrade() {
    let mut input = clean_segmented_input();
    input.selected_state_explicit = false;
    assert_eq!(
        resolve_segmented_control(input).unwrap().degrade_reason,
        Some(M5SegmentedControlDegradeReason::SelectedStateEncodedByColorAlone)
    );

    let mut input = clean_segmented_input();
    input.keyboard_cycling_available = false;
    assert_eq!(
        resolve_segmented_control(input).unwrap().degrade_reason,
        Some(M5SegmentedControlDegradeReason::KeyboardCyclingMissing)
    );
}

#[test]
fn segmented_oversized_and_scope_continuity_degrade() {
    let mut input = clean_segmented_input();
    input.oversized_segment_set = true;
    assert_eq!(
        resolve_segmented_control(input).unwrap().degrade_reason,
        Some(M5SegmentedControlDegradeReason::OversizedSegmentSet)
    );

    let mut input = clean_segmented_input();
    input.scope_impact = M5SplitScopeImpact::CrossSurface;
    input.scope_disclosed = false;
    assert_eq!(
        resolve_segmented_control(input).unwrap().degrade_reason,
        Some(M5SegmentedControlDegradeReason::ModeScopeContinuityBroken)
    );
}

#[test]
fn segmented_mode_unresolved_and_command_degrade() {
    let mut input = clean_segmented_input();
    input.mode = M5SegmentedMode::ModeUnknown;
    assert_eq!(
        resolve_segmented_control(input).unwrap().degrade_reason,
        Some(M5SegmentedControlDegradeReason::ModeUnresolved)
    );

    let mut input = clean_segmented_input();
    input.command_id = "  ".to_owned();
    assert_eq!(
        resolve_segmented_control(input).unwrap().degrade_reason,
        Some(M5SegmentedControlDegradeReason::CommandBindingUnstated)
    );

    let mut input = clean_segmented_input();
    input.command_route_available = false;
    assert_eq!(
        resolve_segmented_control(input).unwrap().degrade_reason,
        Some(M5SegmentedControlDegradeReason::CommandTracePathMissing)
    );
}

#[test]
fn segmented_empty_id_and_forbidden_material_error() {
    let mut input = clean_segmented_input();
    input.segmented_control_id = "   ".to_owned();
    assert_eq!(
        resolve_segmented_control(input).unwrap_err(),
        M5SplitSegmentedResolutionError::EmptySegmentedControlId
    );

    let mut input = clean_segmented_input();
    input.selected_segment_label = "see internal://notes".to_owned();
    assert_eq!(
        resolve_segmented_control(input).unwrap_err(),
        M5SplitSegmentedResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_split_button_segmented_control_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_split_button_segmented_control_controls();
    packet.vocabulary_set.alternate_visibilities.pop();
    assert!(packet
        .validate()
        .contains(&M5SplitButtonSegmentedControlControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_split_button_segmented_control_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SplitButtonSegmentedControlControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_split_button_segmented_control_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_SPLIT_BUTTON_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5SplitButtonSegmentedControlControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_split_button_segmented_control_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5SplitSegmentedAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5SplitButtonSegmentedControlControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_split_button_segmented_control_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5SplitSegmentedExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5SplitButtonSegmentedControlControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_split_button_segmented_control_controls();
    packet.controls_rows[0].segmented_control_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5SplitButtonSegmentedControlControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_split_button_segmented_control_controls();
    // Force a clean split button to also read as hiding an alternate — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.split_button_examples[0].degrade_reason = None;
    row.split_button_examples[0].alternate_visible = false;
    assert!(packet
        .validate()
        .contains(&M5SplitButtonSegmentedControlControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_split_button_segmented_control_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.split_buttons_default_to_riskier_alternate = true,
            1 => row.alternate_actions_hidden_behind_default = true,
            2 => row.segmented_controls_masquerade_as_navigation = true,
            _ => row.locked_or_degraded_semantics_hidden_behind_disabled = true,
        }
        assert!(packet
            .validate()
            .contains(&M5SplitButtonSegmentedControlControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn safe_default_not_proven_when_riskier_example_removed() {
    let mut packet = seeded_m5_split_button_segmented_control_controls();
    for row in &mut packet.controls_rows {
        row.split_button_examples.retain(|ex| {
            ex.degrade_reason != Some(M5SplitButtonDegradeReason::RiskierAlternateBecameDefault)
        });
    }
    assert!(packet.validate().contains(
        &M5SplitButtonSegmentedControlControlsViolation::SafeDefaultAndAlternateBehaviorNotProven
    ));
}

#[test]
fn safe_default_not_proven_when_posture_grammar_collapses() {
    let mut packet = seeded_m5_split_button_segmented_control_controls();
    // Drop every clean explicit-alternate split so the posture grammar no longer covers it.
    for row in &mut packet.controls_rows {
        row.split_button_examples
            .retain(|ex| !(ex.is_clean() && ex.default_posture == "explicit_alternate"));
    }
    assert!(packet.validate().contains(
        &M5SplitButtonSegmentedControlControlsViolation::SafeDefaultAndAlternateBehaviorNotProven
    ));
}

#[test]
fn selected_mode_not_proven_when_stealth_nav_example_removed() {
    let mut packet = seeded_m5_split_button_segmented_control_controls();
    for row in &mut packet.controls_rows {
        row.segmented_control_examples.retain(|ex| {
            ex.degrade_reason != Some(M5SegmentedControlDegradeReason::UsedAsStealthNavigation)
        });
    }
    assert!(packet.validate().contains(
        &M5SplitButtonSegmentedControlControlsViolation::SelectedModeAndKeyboardTruthNotProven
    ));
}

#[test]
fn selected_mode_not_proven_when_keyboard_example_removed() {
    let mut packet = seeded_m5_split_button_segmented_control_controls();
    for row in &mut packet.controls_rows {
        row.segmented_control_examples.retain(|ex| {
            ex.degrade_reason != Some(M5SegmentedControlDegradeReason::KeyboardCyclingMissing)
        });
    }
    assert!(packet.validate().contains(
        &M5SplitButtonSegmentedControlControlsViolation::SelectedModeAndKeyboardTruthNotProven
    ));
}

#[test]
fn traceability_not_proven_when_undisclosed_scope_removed() {
    let mut packet = seeded_m5_split_button_segmented_control_controls();
    for row in &mut packet.controls_rows {
        row.split_button_examples.retain(|ex| {
            ex.degrade_reason != Some(M5SplitButtonDegradeReason::BroadenedScopeUndisclosed)
        });
        row.segmented_control_examples.retain(|ex| {
            ex.degrade_reason != Some(M5SegmentedControlDegradeReason::ModeScopeContinuityBroken)
        });
    }
    assert!(packet.validate().contains(
        &M5SplitButtonSegmentedControlControlsViolation::DefaultAndModeTraceabilityNotProven
    ));
}

#[test]
fn traceability_not_proven_when_clean_segmenteds_lose_command_route() {
    let mut packet = seeded_m5_split_button_segmented_control_controls();
    for row in &mut packet.controls_rows {
        for s in &mut row.segmented_control_examples {
            if s.is_clean() {
                s.command_route_available = false;
            }
        }
    }
    assert!(packet.validate().contains(
        &M5SplitButtonSegmentedControlControlsViolation::DefaultAndModeTraceabilityNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_split_button_segmented_control_controls();
    packet
        .governance_review
        .segmented_never_masquerades_as_navigation = false;
    assert!(packet
        .validate()
        .contains(&M5SplitButtonSegmentedControlControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_split_button_segmented_control_controls();
    packet
        .consumer_projection
        .support_export_reads_single_control_source = false;
    assert!(packet
        .validate()
        .contains(&M5SplitButtonSegmentedControlControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_split_button_segmented_control_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5SplitButtonSegmentedControlControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_split_button_segmented_control_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5SplitButtonSegmentedControlControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_split_button_segmented_control_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5SplitButtonSegmentedControlControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_split_button_segmented_control_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_split_button_segmented_control_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_split_button_segmented_control_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_split_button_segmented_control_controls_export()
        .expect("checked M5 split-button / segmented-control controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_SPLIT_BUTTON_SEGMENTED_CONTROL_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_split_button_segmented_control_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_split_button_segmented_control_controls_review_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 6);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5CoreControlConsumerSurface::ReviewUi)
        .unwrap();
    assert_eq!(row.qualification, M5CoreControlQualificationClass::Beta);

    let preview = seeded_m5_split_button_segmented_control_controls_search_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 6);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5CoreControlConsumerSurface::SearchUi)
        .unwrap();
    assert_eq!(row.qualification, M5CoreControlQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5SplitButtonSegmentedControlControlsPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-split-button-segmented-control-controls/review_ui_beta_narrowed.json"
        )
    ))
    .expect("review-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_split_button_segmented_control_controls_review_ui_beta_narrowed()
    );

    let preview: M5SplitButtonSegmentedControlControlsPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-split-button-segmented-control-controls/search_ui_preview_narrowed.json"
        )
    ))
    .expect("search-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_split_button_segmented_control_controls_search_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_split_button_and_segmented_control() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5CoreControlFamily::SplitButton,
            M5CoreControlFamily::SegmentedControl
        ]
    );
}
