use super::*;

fn clean_card_input() -> M5RepairTransactionPreviewCardResolutionInput {
    M5RepairTransactionPreviewCardResolutionInput {
        card_id: "repair-card:test".to_owned(),
        repair_id: "repair-0001".to_owned(),
        linked_finding_ids: vec!["finding-0001".to_owned()],
        prerequisites: vec!["workspace-trusted".to_owned()],
        prerequisites_stated: true,
        checkpoint_state: M5RepairCheckpointState::CheckpointAvailable,
        checkpoint_absence_disclosed: false,
        impact_scope: "rewrites 1 file".to_owned(),
        target_class: M5RepairTargetClass::LocalWorkspace,
        preview_state: M5RepairPreviewState::PreviewReady,
        reads_as_ready: true,
        reads_as_generic_target: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

fn clean_strip_input() -> M5RollbackClassStripResolutionInput {
    M5RollbackClassStripResolutionInput {
        strip_id: "rollback-strip:test".to_owned(),
        repair_id: "repair-0001".to_owned(),
        reversal_class: M5RepairReversalClass::ExactReversal,
        checkpoint_state: M5RepairCheckpointState::CheckpointAvailable,
        checkpoint_absence_disclosed: false,
        reads_as_reversible: true,
        reversal_limit_disclosed: true,
        reads_as_generic_undo: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_repair_preview_rollback_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_PACKET_ID
    );
}

#[test]
fn card_clean_names_transaction_and_is_reviewable() {
    let resolved = resolve_repair_transaction_preview_card(clean_card_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.transaction_reviewable);
    assert!(resolved.checkpoint_present);
    assert!(!resolved.checkpoint_absent);
    assert!(resolved.is_local_remote_or_managed);
    assert_eq!(resolved.target_class, "local_workspace");
    assert_eq!(
        resolved.repair_disposition,
        Some(M5WorkspaceTrustRepairDisposition::PreviewReady)
    );
    assert_eq!(
        resolved.next_action,
        M5RepairReviewAction::ReviewTransaction
    );
}

#[test]
fn card_checkpoint_absent_disclosed_stays_clean() {
    let mut input = clean_card_input();
    input.checkpoint_state = M5RepairCheckpointState::CheckpointMissing;
    input.checkpoint_absence_disclosed = true;
    let resolved = resolve_repair_transaction_preview_card(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.checkpoint_absent);
    assert!(!resolved.hides_checkpoint_absence);
    assert_eq!(
        resolved.repair_disposition,
        Some(M5WorkspaceTrustRepairDisposition::CheckpointMissing)
    );
}

#[test]
fn card_checkpoint_absence_hidden_degrades() {
    let mut input = clean_card_input();
    input.checkpoint_state = M5RepairCheckpointState::CheckpointMissing;
    input.checkpoint_absence_disclosed = false;
    let resolved = resolve_repair_transaction_preview_card(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.hides_checkpoint_absence);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RepairTransactionPreviewCardDegradeReason::CheckpointAbsenceHidden)
    );
    assert_eq!(
        resolved.next_action,
        M5RepairReviewAction::InspectCheckpoint
    );
}

#[test]
fn card_repair_id_and_findings_and_scope_degrade() {
    let mut input = clean_card_input();
    input.repair_id = "  ".to_owned();
    assert_eq!(
        resolve_repair_transaction_preview_card(input)
            .unwrap()
            .degrade_reason,
        Some(M5RepairTransactionPreviewCardDegradeReason::RepairIdUnstated)
    );

    let mut input = clean_card_input();
    input.linked_finding_ids.clear();
    assert_eq!(
        resolve_repair_transaction_preview_card(input)
            .unwrap()
            .degrade_reason,
        Some(M5RepairTransactionPreviewCardDegradeReason::LinkedFindingsUnstated)
    );

    let mut input = clean_card_input();
    input.impact_scope = "  ".to_owned();
    assert_eq!(
        resolve_repair_transaction_preview_card(input)
            .unwrap()
            .degrade_reason,
        Some(M5RepairTransactionPreviewCardDegradeReason::ImpactScopeUnstated)
    );
}

#[test]
fn card_target_unresolved_and_collapsed_degrade() {
    let mut input = clean_card_input();
    input.target_class = M5RepairTargetClass::TargetUnknown;
    assert_eq!(
        resolve_repair_transaction_preview_card(input)
            .unwrap()
            .degrade_reason,
        Some(M5RepairTransactionPreviewCardDegradeReason::TargetClassUnresolved)
    );

    let mut input = clean_card_input();
    input.reads_as_generic_target = true;
    let resolved = resolve_repair_transaction_preview_card(input).unwrap();
    assert!(resolved.collapses_target_into_generic);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RepairTransactionPreviewCardDegradeReason::TargetCollapsedIntoGeneric)
    );
}

#[test]
fn card_preview_not_ready_degrades() {
    let mut input = clean_card_input();
    input.preview_state = M5RepairPreviewState::PreviewIncomplete;
    input.reads_as_ready = true;
    let resolved = resolve_repair_transaction_preview_card(input).unwrap();
    assert!(!resolved.preview_ready);
    assert!(resolved.presents_incomplete_as_ready);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RepairTransactionPreviewCardDegradeReason::PreviewNotReady)
    );
}

#[test]
fn card_empty_id_and_forbidden_material_error() {
    let mut input = clean_card_input();
    input.card_id = "".to_owned();
    assert_eq!(
        resolve_repair_transaction_preview_card(input).unwrap_err(),
        M5RepairPreviewRollbackResolutionError::EmptyCardId
    );

    let mut input = clean_card_input();
    input.impact_scope = "leaked https://relay.internal/session".to_owned();
    assert_eq!(
        resolve_repair_transaction_preview_card(input).unwrap_err(),
        M5RepairPreviewRollbackResolutionError::ForbiddenMaterial
    );
}

#[test]
fn strip_clean_exact_is_reversible() {
    let resolved = resolve_rollback_class_strip(clean_strip_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.reversal_truth_explicit);
    assert!(resolved.is_exact_reversal);
    assert!(resolved.permits_reversible_claim);
    assert!(!resolved.overclaims_reversibility);
    assert_eq!(resolved.reversal_class, "exact_reversal");
    assert_eq!(
        resolved.repair_disposition,
        Some(M5WorkspaceTrustRepairDisposition::ExactReversal)
    );
}

#[test]
fn strip_compensate_never_claims_reversible() {
    let mut input = clean_strip_input();
    input.reversal_class = M5RepairReversalClass::CompensatingReversal;
    input.reads_as_reversible = false;
    input.reversal_limit_disclosed = true;
    let resolved = resolve_rollback_class_strip(input).unwrap();
    assert!(resolved.is_clean());
    assert!(!resolved.permits_reversible_claim);
    assert!(!resolved.claims_reversible);
    assert_eq!(
        resolved.repair_disposition,
        Some(M5WorkspaceTrustRepairDisposition::Compensate)
    );
}

#[test]
fn strip_reversibility_overclaimed_degrades() {
    let mut input = clean_strip_input();
    input.reversal_class = M5RepairReversalClass::ManualFollowUp;
    input.reads_as_reversible = true;
    let resolved = resolve_rollback_class_strip(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.overclaims_reversibility);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RollbackClassStripDegradeReason::ReversibilityOverclaimed)
    );
    assert_eq!(
        resolved.next_action,
        M5RepairReviewAction::ReviewReversalClass
    );
}

#[test]
fn strip_reversal_limit_hidden_degrades() {
    let mut input = clean_strip_input();
    input.reversal_class = M5RepairReversalClass::CompensatingReversal;
    input.reads_as_reversible = false;
    input.reversal_limit_disclosed = false;
    let resolved = resolve_rollback_class_strip(input).unwrap();
    assert!(resolved.hides_reversal_limit);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RollbackClassStripDegradeReason::ReversalLimitHidden)
    );
}

#[test]
fn strip_reversal_unresolved_has_no_disposition() {
    let mut input = clean_strip_input();
    input.reversal_class = M5RepairReversalClass::ReversalUnknown;
    let resolved = resolve_rollback_class_strip(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RollbackClassStripDegradeReason::ReversalClassUnresolved)
    );
    assert_eq!(resolved.repair_disposition, None);
}

#[test]
fn strip_generic_undo_and_review_path_degrade() {
    let mut input = clean_strip_input();
    input.reads_as_generic_undo = true;
    let resolved = resolve_rollback_class_strip(input).unwrap();
    assert!(resolved.collapses_into_generic_undo);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RollbackClassStripDegradeReason::CollapsedIntoGenericUndo)
    );

    let mut input = clean_strip_input();
    input.detail_command_available = false;
    assert_eq!(
        resolve_rollback_class_strip(input).unwrap().degrade_reason,
        Some(M5RollbackClassStripDegradeReason::ReviewPathMissing)
    );
}

#[test]
fn strip_empty_id_and_forbidden_material_error() {
    let mut input = clean_strip_input();
    input.strip_id = "   ".to_owned();
    assert_eq!(
        resolve_rollback_class_strip(input).unwrap_err(),
        M5RepairPreviewRollbackResolutionError::EmptyStripId
    );

    let mut input = clean_strip_input();
    input.repair_id = "repair://leak".to_owned();
    assert_eq!(
        resolve_rollback_class_strip(input).unwrap_err(),
        M5RepairPreviewRollbackResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_repair_preview_rollback_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_repair_preview_rollback_controls();
    packet.vocabulary_set.reversal_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5RepairPreviewRollbackControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_repair_preview_rollback_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RepairPreviewRollbackControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_repair_preview_rollback_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_ROLLBACK_CLASS_STRIP_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5RepairPreviewRollbackControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_repair_preview_rollback_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5RepairPreviewRollbackAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5RepairPreviewRollbackControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_repair_preview_rollback_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5RepairPreviewRollbackExportField::RepairDispositions);
    assert!(packet
        .validate()
        .contains(&M5RepairPreviewRollbackControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_repair_preview_rollback_controls();
    packet.controls_rows[0]
        .rollback_class_strip_examples
        .clear();
    assert!(packet
        .validate()
        .contains(&M5RepairPreviewRollbackControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_repair_preview_rollback_controls();
    // Force a clean strip to also read as overclaiming reversibility — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.rollback_class_strip_examples[0].degrade_reason = None;
    row.rollback_class_strip_examples[0].overclaims_reversibility = true;
    assert!(packet
        .validate()
        .contains(&M5RepairPreviewRollbackControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_repair_preview_rollback_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.hides_checkpoint_absence_or_reversal_limits = true,
            1 => row.collapses_reversal_classes_into_generic_success = true,
            2 => row.implies_reversibility_without_exact_or_regenerate = true,
            _ => row.hides_target_class_or_impact_scope = true,
        }
        assert!(packet
            .validate()
            .contains(&M5RepairPreviewRollbackControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn transaction_grammar_not_proven_when_checkpoint_hidden_example_removed() {
    let mut packet = seeded_m5_repair_preview_rollback_controls();
    for row in &mut packet.controls_rows {
        row.repair_transaction_preview_card_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5RepairTransactionPreviewCardDegradeReason::CheckpointAbsenceHidden)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5RepairPreviewRollbackControlsViolation::TransactionGrammarNotProven));
}

#[test]
fn transaction_grammar_not_proven_when_target_uncovered() {
    let mut packet = seeded_m5_repair_preview_rollback_controls();
    // Drop every clean managed-workspace card so the required target coverage breaks.
    for row in &mut packet.controls_rows {
        row.repair_transaction_preview_card_examples
            .retain(|ex| !(ex.is_clean() && ex.target_class == "managed_workspace"));
    }
    assert!(packet
        .validate()
        .contains(&M5RepairPreviewRollbackControlsViolation::TransactionGrammarNotProven));
}

#[test]
fn reversal_truth_not_proven_when_overclaim_example_removed() {
    let mut packet = seeded_m5_repair_preview_rollback_controls();
    for row in &mut packet.controls_rows {
        row.rollback_class_strip_examples.retain(|ex| {
            ex.degrade_reason != Some(M5RollbackClassStripDegradeReason::ReversibilityOverclaimed)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5RepairPreviewRollbackControlsViolation::ReversalTruthNotProven));
}

#[test]
fn reversal_truth_not_proven_when_non_reversible_uncovered() {
    let mut packet = seeded_m5_repair_preview_rollback_controls();
    // Drop every clean compensate / manual / audit strip so the non-reversible coverage breaks.
    for row in &mut packet.controls_rows {
        row.rollback_class_strip_examples.retain(|ex| {
            !(ex.is_clean()
                && matches!(
                    ex.reversal_class.as_str(),
                    "compensating_reversal" | "manual_follow_up" | "audit_only"
                ))
        });
    }
    assert!(packet
        .validate()
        .contains(&M5RepairPreviewRollbackControlsViolation::ReversalTruthNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_repair_preview_rollback_controls();
    packet
        .governance_review
        .checkpoint_presence_or_absence_visible_before_apply = false;
    assert!(packet
        .validate()
        .contains(&M5RepairPreviewRollbackControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_repair_preview_rollback_controls();
    packet
        .consumer_projection
        .support_export_reads_single_repair_source = false;
    assert!(packet
        .validate()
        .contains(&M5RepairPreviewRollbackControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_repair_preview_rollback_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5RepairPreviewRollbackControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_repair_preview_rollback_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5RepairPreviewRollbackControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_repair_preview_rollback_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5RepairPreviewRollbackControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_repair_preview_rollback_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_repair_preview_rollback_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_repair_preview_rollback_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_repair_preview_rollback_controls_export()
        .expect("checked M5 repair-preview-rollback controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_REPAIR_PREVIEW_ROLLBACK_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_repair_preview_rollback_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_repair_preview_rollback_controls_doctor_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 5);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5WorkspaceTrustRepairConsumerSurface::DoctorUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5WorkspaceTrustRepairQualificationClass::Beta
    );

    let preview = seeded_m5_repair_preview_rollback_controls_safe_mode_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 5);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5WorkspaceTrustRepairConsumerSurface::SafeModeUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5WorkspaceTrustRepairQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5RepairPreviewRollbackControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-repair-transaction-preview-card-rollback-class-strip-controls/doctor_ui_beta_narrowed.json"
    )))
    .expect("doctor-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_repair_preview_rollback_controls_doctor_ui_beta_narrowed()
    );

    let preview: M5RepairPreviewRollbackControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-repair-transaction-preview-card-rollback-class-strip-controls/safe_mode_ui_preview_narrowed.json"
    )))
    .expect("safe-mode-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_repair_preview_rollback_controls_safe_mode_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_the_two_repair_components() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5WorkspaceTrustRepairComponentFamily::RepairTransactionPreviewCard,
            M5WorkspaceTrustRepairComponentFamily::RollbackClassStrip,
        ]
    );
}
