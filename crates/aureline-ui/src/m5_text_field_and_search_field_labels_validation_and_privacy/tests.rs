use super::*;

fn clean_text_input() -> M5TextFieldResolutionInput {
    M5TextFieldResolutionInput {
        text_field_id: "text:test".to_owned(),
        label: "Full name".to_owned(),
        label_mode: M5FieldLabelMode::PersistentLabel,
        validation: M5FieldValidationState::Valid,
        validation_message_specific: true,
        disposition: M5CoreControlDisposition::Default,
        surface_context: M5FieldSurfaceContext::FormsSheet,
        focus_visible_offered: true,
        requires_reveal: false,
        reveal_offered: false,
        blocked_state_distinct: true,
        draft_preserved_across_interruption: true,
        validation_anchor_preserved: true,
        command_id: "command:forms.name".to_owned(),
        command_route_available: true,
        proof_fresh: true,
    }
}

fn clean_search_input() -> M5SearchFieldResolutionInput {
    M5SearchFieldResolutionInput {
        search_field_id: "search:test".to_owned(),
        label: "Search results".to_owned(),
        label_mode: M5FieldLabelMode::PersistentLabel,
        validation: M5FieldValidationState::Valid,
        validation_message_specific: true,
        disposition: M5CoreControlDisposition::Default,
        surface_context: M5FieldSurfaceContext::SearchBar,
        offers_search_icon: true,
        offers_clear: true,
        submit_model: M5SearchSubmitModel::SubmitAsYouType,
        scope_label: String::new(),
        retention_posture: M5SearchRetentionPosture::LiveNotRetained,
        privacy_disclosed: true,
        blocked_state_distinct: true,
        draft_preserved_across_interruption: true,
        command_id: "command:search.run".to_owned(),
        command_route_available: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_text_field_search_field_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_PACKET_ID
    );
}

#[test]
fn text_clean_names_label_and_validation() {
    let resolved = resolve_text_field(clean_text_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.label_and_validation_honest_at_a_glance);
    assert!(resolved.label_is_permanent);
    assert_eq!(resolved.label_mode, "persistent_label");
    assert_eq!(resolved.validation, "valid");
    assert_eq!(resolved.surface_context, "forms_sheet");
    assert_eq!(resolved.next_action, M5FieldNextAction::OpenCommandDetail);
}

#[test]
fn text_placeholder_only_degrades() {
    let mut input = clean_text_input();
    input.label_mode = M5FieldLabelMode::PlaceholderOnlyDisallowed;
    let resolved = resolve_text_field(input).unwrap();
    assert!(!resolved.label_is_permanent);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TextFieldDegradeReason::LabelIsPlaceholderOnly)
    );

    let mut input = clean_text_input();
    input.label = "   ".to_owned();
    assert_eq!(
        resolve_text_field(input).unwrap().degrade_reason,
        Some(M5TextFieldDegradeReason::LabelIsPlaceholderOnly)
    );
}

#[test]
fn text_vague_validation_degrades_but_specific_is_clean() {
    let mut input = clean_text_input();
    input.validation = M5FieldValidationState::InvalidBlocking;
    input.validation_message_specific = false;
    let resolved = resolve_text_field(input).unwrap();
    assert!(resolved.validation_is_flagging);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TextFieldDegradeReason::VagueValidationCopy)
    );

    let mut input = clean_text_input();
    input.validation = M5FieldValidationState::InvalidBlocking;
    input.validation_message_specific = true;
    assert!(resolve_text_field(input).unwrap().is_clean());
}

#[test]
fn text_focus_and_surface_and_label_mode_unresolved_degrade() {
    let mut input = clean_text_input();
    input.focus_visible_offered = false;
    assert_eq!(
        resolve_text_field(input).unwrap().degrade_reason,
        Some(M5TextFieldDegradeReason::FocusVisibleTreatmentMissing)
    );

    let mut input = clean_text_input();
    input.surface_context = M5FieldSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_text_field(input).unwrap().degrade_reason,
        Some(M5TextFieldDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_text_input();
    input.label_mode = M5FieldLabelMode::LabelUnresolved;
    assert_eq!(
        resolve_text_field(input).unwrap().degrade_reason,
        Some(M5TextFieldDegradeReason::LabelModeUnresolved)
    );

    let mut input = clean_text_input();
    input.validation = M5FieldValidationState::ValidationUnknown;
    assert_eq!(
        resolve_text_field(input).unwrap().degrade_reason,
        Some(M5TextFieldDegradeReason::ValidationStateUnresolved)
    );
}

#[test]
fn text_reveal_missing_degrades_but_offered_is_clean() {
    let mut input = clean_text_input();
    input.requires_reveal = true;
    input.reveal_offered = false;
    assert_eq!(
        resolve_text_field(input).unwrap().degrade_reason,
        Some(M5TextFieldDegradeReason::RevealAffordanceMissing)
    );

    let mut input = clean_text_input();
    input.requires_reveal = true;
    input.reveal_offered = true;
    assert!(resolve_text_field(input).unwrap().is_clean());
}

#[test]
fn text_locked_hidden_degrades_but_distinct_is_clean() {
    let mut input = clean_text_input();
    input.disposition = M5CoreControlDisposition::Locked;
    input.blocked_state_distinct = false;
    let resolved = resolve_text_field(input).unwrap();
    assert!(resolved.disposition_requires_distinct_treatment);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TextFieldDegradeReason::LockedOrDegradedHiddenBehindDisabled)
    );

    let mut input = clean_text_input();
    input.disposition = M5CoreControlDisposition::ReadOnly;
    input.blocked_state_distinct = true;
    let resolved = resolve_text_field(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.disposition_requires_distinct_treatment);
}

#[test]
fn text_draft_and_anchor_loss_degrade() {
    let mut input = clean_text_input();
    input.draft_preserved_across_interruption = false;
    assert_eq!(
        resolve_text_field(input).unwrap().degrade_reason,
        Some(M5TextFieldDegradeReason::DraftContinuityLost)
    );

    let mut input = clean_text_input();
    input.validation_anchor_preserved = false;
    assert_eq!(
        resolve_text_field(input).unwrap().degrade_reason,
        Some(M5TextFieldDegradeReason::ValidationAnchorLost)
    );
}

#[test]
fn text_command_and_trace_degrade() {
    let mut input = clean_text_input();
    input.command_id = "   ".to_owned();
    assert_eq!(
        resolve_text_field(input).unwrap().degrade_reason,
        Some(M5TextFieldDegradeReason::CommandBindingUnstated)
    );

    let mut input = clean_text_input();
    input.command_route_available = false;
    assert_eq!(
        resolve_text_field(input).unwrap().degrade_reason,
        Some(M5TextFieldDegradeReason::CommandTracePathMissing)
    );
}

#[test]
fn text_empty_id_and_forbidden_material_error() {
    let mut input = clean_text_input();
    input.text_field_id = "".to_owned();
    assert_eq!(
        resolve_text_field(input).unwrap_err(),
        M5FieldResolutionError::EmptyTextFieldId
    );

    let mut input = clean_text_input();
    input.label = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_text_field(input).unwrap_err(),
        M5FieldResolutionError::ForbiddenMaterial
    );
}

#[test]
fn search_clean_names_clear_submit_privacy() {
    let resolved = resolve_search_field(clean_search_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.clear_submit_privacy_honest_at_a_glance);
    assert!(resolved.offers_clear);
    assert!(resolved.offers_search_icon);
    assert!(resolved.submit_model_resolved);
    assert_eq!(resolved.submit_model, "submit_as_you_type");
    assert_eq!(resolved.retention_posture, "live_not_retained");
}

#[test]
fn search_placeholder_only_degrades() {
    let mut input = clean_search_input();
    input.label_mode = M5FieldLabelMode::PlaceholderOnlyDisallowed;
    assert_eq!(
        resolve_search_field(input).unwrap().degrade_reason,
        Some(M5SearchFieldDegradeReason::LabelIsPlaceholderOnly)
    );
}

#[test]
fn search_icon_clear_and_submit_model_degrade() {
    let mut input = clean_search_input();
    input.offers_search_icon = false;
    assert_eq!(
        resolve_search_field(input).unwrap().degrade_reason,
        Some(M5SearchFieldDegradeReason::SearchIconMissing)
    );

    let mut input = clean_search_input();
    input.offers_clear = false;
    assert_eq!(
        resolve_search_field(input).unwrap().degrade_reason,
        Some(M5SearchFieldDegradeReason::ClearAffordanceMissing)
    );

    let mut input = clean_search_input();
    input.submit_model = M5SearchSubmitModel::SubmitUnknown;
    let resolved = resolve_search_field(input).unwrap();
    assert!(!resolved.submit_model_resolved);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SearchFieldDegradeReason::SubmitModelUnresolved)
    );
}

#[test]
fn search_privacy_cue_missing_degrades_but_disclosed_is_clean() {
    let mut input = clean_search_input();
    input.retention_posture = M5SearchRetentionPosture::ProviderBackedRemote;
    input.privacy_disclosed = false;
    let resolved = resolve_search_field(input).unwrap();
    assert!(resolved.retention_needs_disclosure);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SearchFieldDegradeReason::PrivacyCueMissing)
    );

    let mut input = clean_search_input();
    input.retention_posture = M5SearchRetentionPosture::ProviderBackedRemote;
    input.privacy_disclosed = true;
    assert!(resolve_search_field(input).unwrap().is_clean());
}

#[test]
fn search_retention_unresolved_degrades() {
    let mut input = clean_search_input();
    input.retention_posture = M5SearchRetentionPosture::RetentionUnknown;
    assert_eq!(
        resolve_search_field(input).unwrap().degrade_reason,
        Some(M5SearchFieldDegradeReason::RetentionUnresolved)
    );
}

#[test]
fn search_blocked_hidden_degrades_but_distinct_is_clean() {
    let mut input = clean_search_input();
    input.submit_model = M5SearchSubmitModel::SubmitBlocked;
    input.blocked_state_distinct = false;
    let resolved = resolve_search_field(input).unwrap();
    assert!(resolved.submit_is_blocked);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SearchFieldDegradeReason::BlockedStateHiddenBehindDisabled)
    );

    let mut input = clean_search_input();
    input.submit_model = M5SearchSubmitModel::SubmitBlocked;
    input.blocked_state_distinct = true;
    assert!(resolve_search_field(input).unwrap().is_clean());
}

#[test]
fn search_draft_command_and_trace_degrade() {
    let mut input = clean_search_input();
    input.draft_preserved_across_interruption = false;
    assert_eq!(
        resolve_search_field(input).unwrap().degrade_reason,
        Some(M5SearchFieldDegradeReason::DraftContinuityLost)
    );

    let mut input = clean_search_input();
    input.command_id = "  ".to_owned();
    assert_eq!(
        resolve_search_field(input).unwrap().degrade_reason,
        Some(M5SearchFieldDegradeReason::CommandBindingUnstated)
    );

    let mut input = clean_search_input();
    input.command_route_available = false;
    assert_eq!(
        resolve_search_field(input).unwrap().degrade_reason,
        Some(M5SearchFieldDegradeReason::CommandTracePathMissing)
    );
}

#[test]
fn search_empty_id_and_forbidden_material_error() {
    let mut input = clean_search_input();
    input.search_field_id = "   ".to_owned();
    assert_eq!(
        resolve_search_field(input).unwrap_err(),
        M5FieldResolutionError::EmptySearchFieldId
    );

    let mut input = clean_search_input();
    input.scope_label = "see internal://notes".to_owned();
    assert_eq!(
        resolve_search_field(input).unwrap_err(),
        M5FieldResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_text_field_search_field_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_text_field_search_field_controls();
    packet.vocabulary_set.search_retention_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5TextFieldSearchFieldControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_text_field_search_field_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5TextFieldSearchFieldControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_text_field_search_field_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_SEARCH_FIELD_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5TextFieldSearchFieldControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_text_field_search_field_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5FieldAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5TextFieldSearchFieldControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_text_field_search_field_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5FieldExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5TextFieldSearchFieldControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_text_field_search_field_controls();
    packet.controls_rows[0].search_field_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5TextFieldSearchFieldControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_text_field_search_field_controls();
    // Force a clean search field to also read as dropping its clear affordance — the packet must reject.
    let row = &mut packet.controls_rows[0];
    row.search_field_examples[0].degrade_reason = None;
    row.search_field_examples[0].offers_clear = false;
    assert!(packet
        .validate()
        .contains(&M5TextFieldSearchFieldControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_text_field_search_field_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.placeholder_text_replaces_label = true,
            1 => row.vague_validation_copy_used = true,
            2 => row.clear_submit_or_privacy_truth_dropped = true,
            _ => row.locked_or_degraded_semantics_hidden_behind_disabled = true,
        }
        assert!(packet
            .validate()
            .contains(&M5TextFieldSearchFieldControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn labeling_not_proven_when_placeholder_example_removed() {
    let mut packet = seeded_m5_text_field_search_field_controls();
    for row in &mut packet.controls_rows {
        row.text_field_examples.retain(|ex| {
            ex.degrade_reason != Some(M5TextFieldDegradeReason::LabelIsPlaceholderOnly)
        });
        row.search_field_examples.retain(|ex| {
            ex.degrade_reason != Some(M5SearchFieldDegradeReason::LabelIsPlaceholderOnly)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5TextFieldSearchFieldControlsViolation::LabelingAndValidationCopyNotProven));
}

#[test]
fn labeling_not_proven_when_label_grammar_collapses() {
    let mut packet = seeded_m5_text_field_search_field_controls();
    // Drop every clean floating-label field so the label grammar no longer covers it.
    for row in &mut packet.controls_rows {
        row.text_field_examples
            .retain(|ex| !(ex.is_clean() && ex.label_mode == "floating_label"));
        row.search_field_examples
            .retain(|ex| !(ex.is_clean() && ex.label_mode == "floating_label"));
    }
    assert!(packet
        .validate()
        .contains(&M5TextFieldSearchFieldControlsViolation::LabelingAndValidationCopyNotProven));
}

#[test]
fn clear_submit_privacy_not_proven_when_privacy_example_removed() {
    let mut packet = seeded_m5_text_field_search_field_controls();
    for row in &mut packet.controls_rows {
        row.search_field_examples
            .retain(|ex| ex.degrade_reason != Some(M5SearchFieldDegradeReason::PrivacyCueMissing));
    }
    assert!(packet.validate().contains(
        &M5TextFieldSearchFieldControlsViolation::ClearSubmitPrivacyBlockedTruthNotProven
    ));
}

#[test]
fn clear_submit_privacy_not_proven_when_clear_example_removed() {
    let mut packet = seeded_m5_text_field_search_field_controls();
    for row in &mut packet.controls_rows {
        row.search_field_examples.retain(|ex| {
            ex.degrade_reason != Some(M5SearchFieldDegradeReason::ClearAffordanceMissing)
        });
    }
    assert!(packet.validate().contains(
        &M5TextFieldSearchFieldControlsViolation::ClearSubmitPrivacyBlockedTruthNotProven
    ));
}

#[test]
fn clear_submit_privacy_not_proven_when_retention_grammar_collapses() {
    let mut packet = seeded_m5_text_field_search_field_controls();
    // Drop every clean disclosed-retention search so only live remains.
    for row in &mut packet.controls_rows {
        row.search_field_examples.retain(|ex| {
            !(ex.is_clean()
                && matches!(
                    ex.retention_posture.as_str(),
                    "cached_results_disclosed" | "provider_backed_remote" | "export_sensitive"
                ))
        });
    }
    assert!(packet.validate().contains(
        &M5TextFieldSearchFieldControlsViolation::ClearSubmitPrivacyBlockedTruthNotProven
    ));
}

#[test]
fn continuity_not_proven_when_draft_lost_removed() {
    let mut packet = seeded_m5_text_field_search_field_controls();
    for row in &mut packet.controls_rows {
        row.text_field_examples
            .retain(|ex| ex.degrade_reason != Some(M5TextFieldDegradeReason::DraftContinuityLost));
        row.search_field_examples.retain(|ex| {
            ex.degrade_reason != Some(M5SearchFieldDegradeReason::DraftContinuityLost)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5TextFieldSearchFieldControlsViolation::DraftAndValidationContinuityNotProven));
}

#[test]
fn continuity_not_proven_when_anchor_lost_removed() {
    let mut packet = seeded_m5_text_field_search_field_controls();
    for row in &mut packet.controls_rows {
        row.text_field_examples
            .retain(|ex| ex.degrade_reason != Some(M5TextFieldDegradeReason::ValidationAnchorLost));
    }
    assert!(packet
        .validate()
        .contains(&M5TextFieldSearchFieldControlsViolation::DraftAndValidationContinuityNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_text_field_search_field_controls();
    packet
        .governance_review
        .search_discloses_retention_and_privacy_cues = false;
    assert!(packet
        .validate()
        .contains(&M5TextFieldSearchFieldControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_text_field_search_field_controls();
    packet
        .consumer_projection
        .support_export_reads_single_control_source = false;
    assert!(packet
        .validate()
        .contains(&M5TextFieldSearchFieldControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_text_field_search_field_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5TextFieldSearchFieldControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_text_field_search_field_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5TextFieldSearchFieldControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_text_field_search_field_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5TextFieldSearchFieldControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_text_field_search_field_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_text_field_search_field_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_text_field_search_field_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_text_field_search_field_controls_export()
        .expect("checked M5 text-field / search-field controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_TEXT_FIELD_SEARCH_FIELD_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_text_field_search_field_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_text_field_search_field_controls_settings_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 6);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5CoreControlConsumerSurface::SettingsUi)
        .unwrap();
    assert_eq!(row.qualification, M5CoreControlQualificationClass::Beta);

    let preview = seeded_m5_text_field_search_field_controls_search_ui_preview_narrowed();
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
    let beta: M5TextFieldSearchFieldControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-text-field-search-field-controls/settings_ui_beta_narrowed.json"
    )))
    .expect("settings-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_text_field_search_field_controls_settings_ui_beta_narrowed()
    );

    let preview: M5TextFieldSearchFieldControlsPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-text-field-search-field-controls/search_ui_preview_narrowed.json"
    )))
        .expect("search-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_text_field_search_field_controls_search_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_text_field_and_search_field() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5CoreControlFamily::TextField,
            M5CoreControlFamily::SearchField
        ]
    );
}
