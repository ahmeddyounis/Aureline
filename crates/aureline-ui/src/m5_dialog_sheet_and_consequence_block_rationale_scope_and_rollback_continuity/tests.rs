use super::*;

fn clean_dialog_input() -> M5DialogResolutionInput {
    M5DialogResolutionInput {
        dialog_id: "dialog:test".to_owned(),
        dialog_title: "Approve this change set?".to_owned(),
        action_model: M5DialogActionModel::NamedSpecificActions,
        disposition: M5DecisionFeedbackDisposition::Warning,
        surface_context: M5DecisionActionSurfaceContext::ReviewConfirmation,
        focus_target: M5DialogFocusTarget::FocusesLeastDestructiveAction,
        reopen_origin: M5DialogReopenOrigin::FreshInvocation,
        rationale_present: true,
        scope_named: true,
        actions_explicitly_named: true,
        initial_focus_is_safe: true,
        cancel_path_present: true,
        focus_returns_on_reopen: true,
        help_or_docs_hook_present: true,
        proof_fresh: true,
    }
}

fn clean_consequence_input() -> M5ConsequenceResolutionInput {
    M5ConsequenceResolutionInput {
        consequence_id: "consequence:test".to_owned(),
        consequence_label: "3 approved files will be rewritten".to_owned(),
        disclosure: M5ConsequenceDisclosure::NamedBlastRadius,
        disposition: M5DecisionFeedbackDisposition::Warning,
        surface_context: M5DecisionActionSurfaceContext::ReviewConfirmation,
        blast_radius: M5ConsequenceBlastRadius::MultipleObjects,
        reversibility: M5ConsequenceReversibility::RollbackWithNamedSteps,
        affected_object_named: true,
        blast_radius_named: true,
        rollback_or_help_posture_stated: true,
        partial_or_irreversible_noted: true,
        avoids_generic_yes_no: true,
        explanation_reachable_by_keyboard_sr_export: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_dialog_consequence_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_DIALOG_CONSEQUENCE_CONTROLS_PACKET_ID);
}

#[test]
fn dialog_clean_names_rationale_scope_and_actions() {
    let resolved = resolve_dialog(clean_dialog_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.names_rationale_scope_and_explicit_actions);
    assert!(!resolved.action_model_is_generic_yes_no);
    assert_eq!(resolved.action_model, "named_specific_actions");
    assert_eq!(resolved.surface_context, "review_confirmation");
    assert_eq!(
        resolved.next_action,
        M5DialogConsequenceNextAction::ReviewRationaleAndScope
    );
}

#[test]
fn dialog_title_unstated_degrades() {
    let mut input = clean_dialog_input();
    input.dialog_title = "   ".to_owned();
    assert_eq!(
        resolve_dialog(input).unwrap().degrade_reason,
        Some(M5DialogDegradeReason::DialogTitleUnstated)
    );
}

#[test]
fn dialog_generic_yes_no_degrades() {
    let mut input = clean_dialog_input();
    input.action_model = M5DialogActionModel::GenericYesNoDisallowed;
    let resolved = resolve_dialog(input).unwrap();
    assert!(resolved.action_model_is_generic_yes_no);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DialogDegradeReason::GenericYesNoActionModel)
    );
}

#[test]
fn dialog_rationale_and_scope_missing_degrade() {
    let mut input = clean_dialog_input();
    input.rationale_present = false;
    assert_eq!(
        resolve_dialog(input).unwrap().degrade_reason,
        Some(M5DialogDegradeReason::RationaleUnstated)
    );

    let mut input = clean_dialog_input();
    input.scope_named = false;
    assert_eq!(
        resolve_dialog(input).unwrap().degrade_reason,
        Some(M5DialogDegradeReason::ScopeUnstated)
    );
}

#[test]
fn dialog_explicit_actions_unnamed_degrades() {
    let mut input = clean_dialog_input();
    input.actions_explicitly_named = false;
    assert_eq!(
        resolve_dialog(input).unwrap().degrade_reason,
        Some(M5DialogDegradeReason::ExplicitActionsUnnamed)
    );
}

#[test]
fn dialog_focus_cancel_and_return_degrade() {
    let mut input = clean_dialog_input();
    input.initial_focus_is_safe = false;
    assert_eq!(
        resolve_dialog(input).unwrap().degrade_reason,
        Some(M5DialogDegradeReason::SafeInitialFocusMissing)
    );

    let mut input = clean_dialog_input();
    input.focus_target = M5DialogFocusTarget::FocusTargetUnknown;
    assert_eq!(
        resolve_dialog(input).unwrap().degrade_reason,
        Some(M5DialogDegradeReason::SafeInitialFocusMissing)
    );

    let mut input = clean_dialog_input();
    input.cancel_path_present = false;
    assert_eq!(
        resolve_dialog(input).unwrap().degrade_reason,
        Some(M5DialogDegradeReason::CancelPathMissing)
    );

    let mut input = clean_dialog_input();
    input.focus_returns_on_reopen = false;
    assert_eq!(
        resolve_dialog(input).unwrap().degrade_reason,
        Some(M5DialogDegradeReason::FocusReturnBrokenOnReopen)
    );
}

#[test]
fn dialog_reopen_origin_and_help_hook_degrade() {
    let mut input = clean_dialog_input();
    input.reopen_origin = M5DialogReopenOrigin::OriginUnknown;
    assert_eq!(
        resolve_dialog(input).unwrap().degrade_reason,
        Some(M5DialogDegradeReason::ReopenOriginUnresolved)
    );

    let mut input = clean_dialog_input();
    input.help_or_docs_hook_present = false;
    assert_eq!(
        resolve_dialog(input).unwrap().degrade_reason,
        Some(M5DialogDegradeReason::HelpDocsHookMissing)
    );
}

#[test]
fn dialog_surface_unresolved_degrades() {
    let mut input = clean_dialog_input();
    input.surface_context = M5DecisionActionSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_dialog(input).unwrap().degrade_reason,
        Some(M5DialogDegradeReason::SurfaceContextUnresolved)
    );
}

#[test]
fn dialog_empty_id_and_forbidden_material_error() {
    let mut input = clean_dialog_input();
    input.dialog_id = "".to_owned();
    assert_eq!(
        resolve_dialog(input).unwrap_err(),
        M5DialogConsequenceResolutionError::EmptyDialogId
    );

    let mut input = clean_dialog_input();
    input.dialog_title = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_dialog(input).unwrap_err(),
        M5DialogConsequenceResolutionError::ForbiddenMaterial
    );
}

#[test]
fn consequence_clean_names_blast_and_rollback() {
    let resolved = resolve_consequence(clean_consequence_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.names_blast_radius_and_rollback_posture);
    assert!(resolved.blast_radius_is_broad);
    assert!(!resolved.disclosure_is_disallowed);
    assert_eq!(resolved.blast_radius, "multiple_objects");
    assert_eq!(resolved.surface_context, "review_confirmation");
    assert_eq!(
        resolved.next_action,
        M5DialogConsequenceNextAction::ReviewConsequenceBlock
    );
}

#[test]
fn consequence_label_unstated_degrades() {
    let mut input = clean_consequence_input();
    input.consequence_label = "   ".to_owned();
    assert_eq!(
        resolve_consequence(input).unwrap().degrade_reason,
        Some(M5ConsequenceDegradeReason::ConsequenceLabelUnstated)
    );
}

#[test]
fn consequence_disclosure_disallowed_degrades() {
    let mut input = clean_consequence_input();
    input.disclosure = M5ConsequenceDisclosure::GenericYesNoDisallowed;
    let resolved = resolve_consequence(input).unwrap();
    assert!(resolved.disclosure_is_disallowed);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ConsequenceDegradeReason::DisclosureModelDisallowed)
    );
}

#[test]
fn consequence_blast_and_reversibility_unresolved_degrade() {
    let mut input = clean_consequence_input();
    input.blast_radius = M5ConsequenceBlastRadius::RadiusUnknown;
    assert_eq!(
        resolve_consequence(input).unwrap().degrade_reason,
        Some(M5ConsequenceDegradeReason::BlastRadiusUnresolved)
    );

    let mut input = clean_consequence_input();
    input.blast_radius_named = false;
    assert_eq!(
        resolve_consequence(input).unwrap().degrade_reason,
        Some(M5ConsequenceDegradeReason::BlastRadiusUnresolved)
    );

    let mut input = clean_consequence_input();
    input.reversibility = M5ConsequenceReversibility::ReversibilityUnknown;
    assert_eq!(
        resolve_consequence(input).unwrap().degrade_reason,
        Some(M5ConsequenceDegradeReason::ReversibilityUnresolved)
    );
}

#[test]
fn consequence_rollback_and_note_and_generic_and_screenshot_degrade() {
    let mut input = clean_consequence_input();
    input.rollback_or_help_posture_stated = false;
    assert_eq!(
        resolve_consequence(input).unwrap().degrade_reason,
        Some(M5ConsequenceDegradeReason::RollbackPostureUnstated)
    );

    let mut input = clean_consequence_input();
    input.reversibility = M5ConsequenceReversibility::IrreversibleAndStated;
    input.partial_or_irreversible_noted = false;
    assert_eq!(
        resolve_consequence(input).unwrap().degrade_reason,
        Some(M5ConsequenceDegradeReason::PartialOrIrreversibleNoteMissing)
    );

    let mut input = clean_consequence_input();
    input.avoids_generic_yes_no = false;
    assert_eq!(
        resolve_consequence(input).unwrap().degrade_reason,
        Some(M5ConsequenceDegradeReason::GenericYesNoAmbiguity)
    );

    let mut input = clean_consequence_input();
    input.explanation_reachable_by_keyboard_sr_export = false;
    assert_eq!(
        resolve_consequence(input).unwrap().degrade_reason,
        Some(M5ConsequenceDegradeReason::ExplanationReachableOnlyViaScreenshot)
    );
}

#[test]
fn consequence_empty_id_and_forbidden_material_error() {
    let mut input = clean_consequence_input();
    input.consequence_id = "   ".to_owned();
    assert_eq!(
        resolve_consequence(input).unwrap_err(),
        M5DialogConsequenceResolutionError::EmptyConsequenceId
    );

    let mut input = clean_consequence_input();
    input.consequence_label = "see internal://notes".to_owned();
    assert_eq!(
        resolve_consequence(input).unwrap_err(),
        M5DialogConsequenceResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_dialog_consequence_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_dialog_consequence_controls();
    packet.vocabulary_set.blast_radii.pop();
    assert!(packet
        .validate()
        .contains(&M5DialogConsequenceControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_dialog_consequence_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DialogConsequenceControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_dialog_consequence_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_DIALOG_SHEET_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5DialogConsequenceControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_dialog_consequence_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5DialogConsequenceAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5DialogConsequenceControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_dialog_consequence_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5DialogConsequenceExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5DialogConsequenceControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_dialog_consequence_controls();
    packet.controls_rows[0].consequence_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5DialogConsequenceControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_dialog_consequence_controls();
    // Force a clean dialog to also read as generic Yes/No — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.dialog_examples[0].degrade_reason = None;
    row.dialog_examples[0].action_model_is_generic_yes_no = true;
    assert!(packet
        .validate()
        .contains(&M5DialogConsequenceControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_dialog_consequence_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.dialog_uses_generic_yes_no_in_high_risk = true,
            1 => row.dialog_focus_fails_to_return_on_reopen = true,
            2 => row.consequence_omits_named_blast_radius = true,
            _ => row.consequence_reduces_to_generic_yes_no = true,
        }
        assert!(packet
            .validate()
            .contains(&M5DialogConsequenceControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn dialog_actions_not_proven_when_generic_example_removed() {
    let mut packet = seeded_m5_dialog_consequence_controls();
    for row in &mut packet.controls_rows {
        row.dialog_examples
            .retain(|ex| ex.degrade_reason != Some(M5DialogDegradeReason::GenericYesNoActionModel));
    }
    assert!(packet
        .validate()
        .contains(&M5DialogConsequenceControlsViolation::DialogRationaleScopeActionsNotProven));
}

#[test]
fn dialog_actions_not_proven_when_action_grammar_collapses() {
    let mut packet = seeded_m5_dialog_consequence_controls();
    // Drop every clean destructive-confirm-named dialog so the action grammar no longer covers it.
    for row in &mut packet.controls_rows {
        row.dialog_examples
            .retain(|ex| !(ex.is_clean() && ex.action_model == "destructive_confirm_named"));
    }
    assert!(packet
        .validate()
        .contains(&M5DialogConsequenceControlsViolation::DialogRationaleScopeActionsNotProven));
}

#[test]
fn focus_stability_not_proven_when_focus_example_removed() {
    let mut packet = seeded_m5_dialog_consequence_controls();
    for row in &mut packet.controls_rows {
        row.dialog_examples.retain(|ex| {
            ex.degrade_reason != Some(M5DialogDegradeReason::FocusReturnBrokenOnReopen)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5DialogConsequenceControlsViolation::FocusAndCancelStabilityNotProven));
}

#[test]
fn consequence_explainable_not_proven_when_blast_example_removed() {
    let mut packet = seeded_m5_dialog_consequence_controls();
    for row in &mut packet.controls_rows {
        row.consequence_examples.retain(|ex| {
            ex.degrade_reason != Some(M5ConsequenceDegradeReason::BlastRadiusUnresolved)
        });
    }
    assert!(packet.validate().contains(
        &M5DialogConsequenceControlsViolation::ConsequenceExplainableWithoutScreenshotsNotProven
    ));
}

#[test]
fn consequence_explainable_not_proven_when_screenshot_example_removed() {
    let mut packet = seeded_m5_dialog_consequence_controls();
    for row in &mut packet.controls_rows {
        row.consequence_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5ConsequenceDegradeReason::ExplanationReachableOnlyViaScreenshot)
        });
    }
    assert!(packet.validate().contains(
        &M5DialogConsequenceControlsViolation::ConsequenceExplainableWithoutScreenshotsNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_dialog_consequence_controls();
    packet
        .governance_review
        .consequence_never_reduces_to_generic_yes_no = false;
    assert!(packet
        .validate()
        .contains(&M5DialogConsequenceControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_dialog_consequence_controls();
    packet
        .consumer_projection
        .support_export_reads_single_dialog_consequence_source = false;
    assert!(packet
        .validate()
        .contains(&M5DialogConsequenceControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_dialog_consequence_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DialogConsequenceControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_dialog_consequence_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5DialogConsequenceControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_dialog_consequence_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5DialogConsequenceControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_dialog_consequence_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_dialog_consequence_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_dialog_consequence_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_dialog_consequence_controls_export()
        .expect("checked M5 dialog / consequence controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_DIALOG_CONSEQUENCE_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_dialog_consequence_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_dialog_consequence_controls_review_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 6);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5DecisionFeedbackConsumerSurface::ReviewUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5DecisionFeedbackQualificationClass::Beta
    );

    let preview = seeded_m5_dialog_consequence_controls_updates_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 6);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5DecisionFeedbackConsumerSurface::UpdatesUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5DecisionFeedbackQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5DialogConsequenceControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-dialog-sheet-and-consequence-block-controls/review_ui_beta_narrowed.json"
    )))
    .expect("review-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_dialog_consequence_controls_review_ui_beta_narrowed()
    );

    let preview: M5DialogConsequenceControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-dialog-sheet-and-consequence-block-controls/updates_ui_preview_narrowed.json"
    )))
    .expect("updates-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_dialog_consequence_controls_updates_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_dialog_and_consequence() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5DecisionFeedbackFamily::DialogSheet,
            M5DecisionFeedbackFamily::ConsequenceBlock
        ]
    );
}
