use super::*;

fn clean_combobox_input() -> M5ComboboxResolutionInput {
    M5ComboboxResolutionInput {
        combobox_id: "combobox:test".to_owned(),
        label: "Plan tier".to_owned(),
        selected_value: "Standard tier".to_owned(),
        selected_value_disclosed: true,
        value_source: M5ComboboxValueSource::CanonicalOption,
        support_class_tag: String::new(),
        support_class_tagged: true,
        requires_filter: false,
        filter_offered: true,
        value_provenance: M5ControlValueProvenance::UserOverride,
        provenance_disclosed: true,
        keyboard_navigation_stable: true,
        disposition: M5CoreControlDisposition::Default,
        blocked_state_distinct: true,
        surface_context: M5ControlSurfaceContext::SettingsRow,
        command_id: "command:test.tier".to_owned(),
        command_route_available: true,
        proof_fresh: true,
    }
}

fn clean_toggle_input() -> M5ToggleResolutionInput {
    M5ToggleResolutionInput {
        toggle_id: "toggle:test".to_owned(),
        label: "Email me on completion".to_owned(),
        selected_state: "on".to_owned(),
        selected_state_disclosed: true,
        toggle_semantics: M5ToggleSemantics::CheckboxImmediate,
        apply_timing: M5ToggleApplyTiming::AppliesImmediately,
        selection_arity_explicit: true,
        group_exclusivity_enforced: true,
        value_provenance: M5ControlValueProvenance::UserOverride,
        provenance_disclosed: true,
        disposition: M5CoreControlDisposition::Default,
        blocked_state_distinct: true,
        surface_context: M5ControlSurfaceContext::SettingsRow,
        command_id: "command:test.notify".to_owned(),
        command_route_available: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_PACKET_ID
    );
}

#[test]
fn combobox_clean_names_value_and_source() {
    let resolved = resolve_combobox(clean_combobox_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.value_source_and_lock_honest_at_a_glance);
    assert!(resolved.selected_value_disclosed);
    assert_eq!(resolved.value_source, "canonical_option");
    assert_eq!(resolved.value_provenance, "user_override");
    assert_eq!(resolved.surface_context, "settings_row");
    assert_eq!(resolved.next_action, M5ControlNextAction::OpenCommandDetail);
}

#[test]
fn combobox_selected_value_and_surface_and_source_unresolved_degrade() {
    let mut input = clean_combobox_input();
    input.selected_value_disclosed = false;
    assert_eq!(
        resolve_combobox(input).unwrap().degrade_reason,
        Some(M5ComboboxDegradeReason::SelectedValueUnstated)
    );

    let mut input = clean_combobox_input();
    input.surface_context = M5ControlSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_combobox(input).unwrap().degrade_reason,
        Some(M5ComboboxDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_combobox_input();
    input.value_source = M5ComboboxValueSource::SourceUnknown;
    let resolved = resolve_combobox(input).unwrap();
    assert!(!resolved.value_source_resolved);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ComboboxDegradeReason::ValueSourceUnresolved)
    );
}

#[test]
fn combobox_filterability_missing_degrades_but_offered_is_clean() {
    let mut input = clean_combobox_input();
    input.value_source = M5ComboboxValueSource::FilteredSubset;
    input.requires_filter = true;
    input.filter_offered = false;
    assert_eq!(
        resolve_combobox(input).unwrap().degrade_reason,
        Some(M5ComboboxDegradeReason::FilterabilityMissing)
    );

    let mut input = clean_combobox_input();
    input.value_source = M5ComboboxValueSource::FilteredSubset;
    input.requires_filter = true;
    input.filter_offered = true;
    assert!(resolve_combobox(input).unwrap().is_clean());
}

#[test]
fn combobox_unverified_untagged_degrades_but_tagged_is_clean() {
    let mut input = clean_combobox_input();
    input.value_source = M5ComboboxValueSource::CustomUnverified;
    input.support_class_tagged = false;
    let resolved = resolve_combobox(input).unwrap();
    assert!(resolved.value_source_is_unverified);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ComboboxDegradeReason::UnverifiedValuePresentedAsCanonical)
    );

    let mut input = clean_combobox_input();
    input.value_source = M5ComboboxValueSource::RemoteBacked;
    input.support_class_tagged = true;
    assert!(resolve_combobox(input).unwrap().is_clean());
}

#[test]
fn combobox_provenance_unresolved_and_undisclosed_degrade() {
    let mut input = clean_combobox_input();
    input.value_provenance = M5ControlValueProvenance::ProvenanceUnknown;
    assert_eq!(
        resolve_combobox(input).unwrap().degrade_reason,
        Some(M5ComboboxDegradeReason::ValueProvenanceUnresolved)
    );

    let mut input = clean_combobox_input();
    input.value_provenance = M5ControlValueProvenance::PolicyEnforced;
    input.provenance_disclosed = false;
    let resolved = resolve_combobox(input).unwrap();
    assert!(resolved.value_provenance_needs_disclosure);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ComboboxDegradeReason::ValueProvenanceUndisclosed)
    );

    let mut input = clean_combobox_input();
    input.value_provenance = M5ControlValueProvenance::PolicyEnforced;
    input.provenance_disclosed = true;
    assert!(resolve_combobox(input).unwrap().is_clean());
}

#[test]
fn combobox_keyboard_and_lock_and_trace_degrade() {
    let mut input = clean_combobox_input();
    input.keyboard_navigation_stable = false;
    assert_eq!(
        resolve_combobox(input).unwrap().degrade_reason,
        Some(M5ComboboxDegradeReason::KeyboardNavigationUnstable)
    );

    let mut input = clean_combobox_input();
    input.disposition = M5CoreControlDisposition::Locked;
    input.blocked_state_distinct = false;
    let resolved = resolve_combobox(input).unwrap();
    assert!(resolved.disposition_requires_distinct_treatment);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ComboboxDegradeReason::LockedOrReadOnlyHiddenBehindDisabled)
    );

    let mut input = clean_combobox_input();
    input.command_id = "   ".to_owned();
    assert_eq!(
        resolve_combobox(input).unwrap().degrade_reason,
        Some(M5ComboboxDegradeReason::CommandBindingUnstated)
    );

    let mut input = clean_combobox_input();
    input.command_route_available = false;
    assert_eq!(
        resolve_combobox(input).unwrap().degrade_reason,
        Some(M5ComboboxDegradeReason::CommandTracePathMissing)
    );
}

#[test]
fn combobox_empty_id_and_forbidden_material_error() {
    let mut input = clean_combobox_input();
    input.combobox_id = "".to_owned();
    assert_eq!(
        resolve_combobox(input).unwrap_err(),
        M5ControlResolutionError::EmptyComboboxId
    );

    let mut input = clean_combobox_input();
    input.selected_value = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_combobox(input).unwrap_err(),
        M5ControlResolutionError::ForbiddenMaterial
    );
}

#[test]
fn toggle_clean_names_semantics_and_timing() {
    let resolved = resolve_toggle(clean_toggle_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.semantics_and_timing_honest_at_a_glance);
    assert_eq!(resolved.toggle_semantics, "checkbox_immediate");
    assert_eq!(resolved.apply_timing, "applies_immediately");
    assert!(!resolved.apply_timing_is_deferred);
    assert_eq!(resolved.value_provenance, "user_override");
}

#[test]
fn toggle_switch_deferred_blur_degrades_but_immediate_is_clean() {
    let mut input = clean_toggle_input();
    input.toggle_semantics = M5ToggleSemantics::SwitchImmediate;
    input.apply_timing = M5ToggleApplyTiming::DeferredUntilSave;
    let resolved = resolve_toggle(input).unwrap();
    assert!(resolved.semantics_is_switch);
    assert!(resolved.apply_timing_is_deferred);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ToggleDegradeReason::SwitchBlurredWithDeferredCheckbox)
    );

    let mut input = clean_toggle_input();
    input.toggle_semantics = M5ToggleSemantics::SwitchImmediate;
    input.apply_timing = M5ToggleApplyTiming::AppliesImmediately;
    assert!(resolve_toggle(input).unwrap().is_clean());
}

#[test]
fn toggle_state_semantics_and_timing_unresolved_degrade() {
    let mut input = clean_toggle_input();
    input.selected_state_disclosed = false;
    assert_eq!(
        resolve_toggle(input).unwrap().degrade_reason,
        Some(M5ToggleDegradeReason::SelectedStateUnstated)
    );

    let mut input = clean_toggle_input();
    input.surface_context = M5ControlSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_toggle(input).unwrap().degrade_reason,
        Some(M5ToggleDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_toggle_input();
    input.toggle_semantics = M5ToggleSemantics::SemanticsUnknown;
    assert_eq!(
        resolve_toggle(input).unwrap().degrade_reason,
        Some(M5ToggleDegradeReason::ToggleSemanticsUnresolved)
    );

    let mut input = clean_toggle_input();
    input.apply_timing = M5ToggleApplyTiming::TimingUnknown;
    assert_eq!(
        resolve_toggle(input).unwrap().degrade_reason,
        Some(M5ToggleDegradeReason::ApplyTimingUnresolved)
    );
}

#[test]
fn toggle_arity_and_exclusivity_degrade() {
    let mut input = clean_toggle_input();
    input.selection_arity_explicit = false;
    assert_eq!(
        resolve_toggle(input).unwrap().degrade_reason,
        Some(M5ToggleDegradeReason::OneOfManyVersusMultiSelectAmbiguous)
    );

    let mut input = clean_toggle_input();
    input.toggle_semantics = M5ToggleSemantics::RadioExclusive;
    input.group_exclusivity_enforced = false;
    let resolved = resolve_toggle(input).unwrap();
    assert!(resolved.semantics_is_exclusive);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ToggleDegradeReason::GroupExclusivityLost)
    );
}

#[test]
fn toggle_provenance_lock_and_trace_degrade() {
    let mut input = clean_toggle_input();
    input.value_provenance = M5ControlValueProvenance::ProvenanceUnknown;
    assert_eq!(
        resolve_toggle(input).unwrap().degrade_reason,
        Some(M5ToggleDegradeReason::ValueProvenanceUnresolved)
    );

    let mut input = clean_toggle_input();
    input.value_provenance = M5ControlValueProvenance::Imported;
    input.provenance_disclosed = false;
    assert_eq!(
        resolve_toggle(input).unwrap().degrade_reason,
        Some(M5ToggleDegradeReason::ValueProvenanceUndisclosed)
    );

    let mut input = clean_toggle_input();
    input.disposition = M5CoreControlDisposition::ReadOnly;
    input.blocked_state_distinct = false;
    assert_eq!(
        resolve_toggle(input).unwrap().degrade_reason,
        Some(M5ToggleDegradeReason::LockedOrReadOnlyHiddenBehindDisabled)
    );

    let mut input = clean_toggle_input();
    input.command_route_available = false;
    assert_eq!(
        resolve_toggle(input).unwrap().degrade_reason,
        Some(M5ToggleDegradeReason::CommandTracePathMissing)
    );
}

#[test]
fn toggle_empty_id_and_forbidden_material_error() {
    let mut input = clean_toggle_input();
    input.toggle_id = "   ".to_owned();
    assert_eq!(
        resolve_toggle(input).unwrap_err(),
        M5ControlResolutionError::EmptyToggleId
    );

    let mut input = clean_toggle_input();
    input.label = "see internal://notes".to_owned();
    assert_eq!(
        resolve_toggle(input).unwrap_err(),
        M5ControlResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_combobox_checkbox_radio_switch_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    packet.vocabulary_set.apply_timings.pop();
    assert!(packet
        .validate()
        .contains(&M5ComboboxToggleControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ComboboxToggleControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_TOGGLE_CONTROL_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5ComboboxToggleControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ControlAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5ComboboxToggleControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5ControlExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5ComboboxToggleControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    packet.controls_rows[0].toggle_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5ComboboxToggleControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    // Force a clean toggle to also read as a deferred switch — the packet must reject.
    let row = &mut packet.controls_rows[0];
    row.toggle_examples[0].degrade_reason = None;
    row.toggle_examples[0].semantics_is_switch = true;
    row.toggle_examples[0].apply_timing_is_deferred = true;
    assert!(packet
        .validate()
        .contains(&M5ComboboxToggleControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.value_source_or_provenance_truth_dropped = true,
            1 => row.switch_blurred_with_deferred_checkbox = true,
            2 => row.one_of_many_versus_multi_select_blurred = true,
            _ => row.locked_or_read_only_semantics_hidden_behind_disabled = true,
        }
        assert!(packet
            .validate()
            .contains(&M5ComboboxToggleControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn value_source_lock_and_timing_not_proven_when_source_unresolved_removed() {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    for row in &mut packet.controls_rows {
        row.combobox_examples
            .retain(|ex| ex.degrade_reason != Some(M5ComboboxDegradeReason::ValueSourceUnresolved));
    }
    assert!(packet
        .validate()
        .contains(&M5ComboboxToggleControlsViolation::ValueSourceLockAndTimingNotProven));
}

#[test]
fn value_source_lock_and_timing_not_proven_when_switch_blur_removed() {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    for row in &mut packet.controls_rows {
        row.toggle_examples.retain(|ex| {
            ex.degrade_reason != Some(M5ToggleDegradeReason::SwitchBlurredWithDeferredCheckbox)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ComboboxToggleControlsViolation::ValueSourceLockAndTimingNotProven));
}

#[test]
fn value_source_lock_and_timing_not_proven_when_timing_grammar_collapses() {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    // Drop every clean deferred toggle so only immediate timing remains.
    for row in &mut packet.controls_rows {
        row.toggle_examples
            .retain(|ex| !(ex.is_clean() && ex.apply_timing_is_deferred));
    }
    assert!(packet
        .validate()
        .contains(&M5ComboboxToggleControlsViolation::ValueSourceLockAndTimingNotProven));
}

#[test]
fn accessibility_and_distinct_state_not_proven_when_locked_hidden_removed() {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    for row in &mut packet.controls_rows {
        row.combobox_examples.retain(|ex| {
            ex.degrade_reason != Some(M5ComboboxDegradeReason::LockedOrReadOnlyHiddenBehindDisabled)
        });
        row.toggle_examples.retain(|ex| {
            ex.degrade_reason != Some(M5ToggleDegradeReason::LockedOrReadOnlyHiddenBehindDisabled)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ComboboxToggleControlsViolation::AccessibilityAndDistinctStateNotProven));
}

#[test]
fn accessibility_and_distinct_state_not_proven_when_keyboard_unstable_removed() {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    for row in &mut packet.controls_rows {
        row.combobox_examples.retain(|ex| {
            ex.degrade_reason != Some(M5ComboboxDegradeReason::KeyboardNavigationUnstable)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ComboboxToggleControlsViolation::AccessibilityAndDistinctStateNotProven));
}

#[test]
fn selection_state_and_editability_trace_not_proven_when_trace_removed() {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    for row in &mut packet.controls_rows {
        row.combobox_examples.retain(|ex| {
            ex.degrade_reason != Some(M5ComboboxDegradeReason::CommandTracePathMissing)
        });
        row.toggle_examples
            .retain(|ex| ex.degrade_reason != Some(M5ToggleDegradeReason::CommandTracePathMissing));
    }
    assert!(packet
        .validate()
        .contains(&M5ComboboxToggleControlsViolation::SelectionStateAndEditabilityTraceNotProven));
}

#[test]
fn selection_state_and_editability_trace_not_proven_when_provenance_grammar_collapses() {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    // Drop every clean non-user-origin provenance so the disclosed-non-user grammar no longer holds.
    for row in &mut packet.controls_rows {
        row.combobox_examples.retain(|ex| {
            !(ex.is_clean()
                && matches!(
                    ex.value_provenance.as_str(),
                    "policy_enforced" | "imported" | "detected"
                ))
        });
        row.toggle_examples.retain(|ex| {
            !(ex.is_clean()
                && matches!(
                    ex.value_provenance.as_str(),
                    "policy_enforced" | "imported" | "detected"
                ))
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ComboboxToggleControlsViolation::SelectionStateAndEditabilityTraceNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    packet
        .governance_review
        .switch_never_blurred_with_deferred_checkbox = false;
    assert!(packet
        .validate()
        .contains(&M5ComboboxToggleControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    packet
        .consumer_projection
        .support_export_reconstructs_selection_and_editability = false;
    assert!(packet
        .validate()
        .contains(&M5ComboboxToggleControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ComboboxToggleControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ComboboxToggleControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ComboboxToggleControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_combobox_checkbox_radio_switch_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_combobox_checkbox_radio_switch_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_combobox_checkbox_radio_switch_controls_export()
        .expect("checked M5 combobox / toggle-control controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_COMBOBOX_CHECKBOX_RADIO_SWITCH_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_combobox_checkbox_radio_switch_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_combobox_checkbox_radio_switch_controls_settings_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 6);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5CoreControlConsumerSurface::SettingsUi)
        .unwrap();
    assert_eq!(row.qualification, M5CoreControlQualificationClass::Beta);

    let preview = seeded_m5_combobox_checkbox_radio_switch_controls_entry_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 6);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5CoreControlConsumerSurface::EntryUi)
        .unwrap();
    assert_eq!(row.qualification, M5CoreControlQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5ComboboxToggleControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-combobox-checkbox-radio-switch-controls/settings_ui_beta_narrowed.json"
    )))
    .expect("settings-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_combobox_checkbox_radio_switch_controls_settings_ui_beta_narrowed()
    );

    let preview: M5ComboboxToggleControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-combobox-checkbox-radio-switch-controls/entry_ui_preview_narrowed.json"
    )))
    .expect("entry-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_combobox_checkbox_radio_switch_controls_entry_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_combobox_and_toggle_control() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5CoreControlFamily::Combobox,
            M5CoreControlFamily::ToggleControl
        ]
    );
}
