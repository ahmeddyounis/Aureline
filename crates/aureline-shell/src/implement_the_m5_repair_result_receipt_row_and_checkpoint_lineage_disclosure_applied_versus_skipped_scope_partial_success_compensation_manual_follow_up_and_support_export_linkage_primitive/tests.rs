use super::*;

fn clean_receipt_input() -> M5RepairResultReceiptRowResolutionInput {
    M5RepairResultReceiptRowResolutionInput {
        receipt_id: "receipt:test".to_owned(),
        repair_id: "repair:test".to_owned(),
        linked_finding_ids: vec!["finding:test".to_owned()],
        outcome_class: M5RepairOutcomeClass::RepairAppliedExact,
        applied_scope: vec!["file: test.json".to_owned()],
        skipped_scope: Vec::new(),
        checkpoint_state: M5RepairCheckpointState::CheckpointAvailable,
        checkpoint_ref: "checkpoint:test".to_owned(),
        reversal_class: M5RepairReversalClass::ExactReversal,
        follow_up_stated: true,
        reads_as_complete: true,
        reads_as_generic_success: false,
        support_export_available: true,
        proof_fresh: true,
    }
}

fn clean_lineage_input() -> M5CheckpointLineageDisclosureResolutionInput {
    M5CheckpointLineageDisclosureResolutionInput {
        disclosure_id: "lineage:test".to_owned(),
        repair_id: "repair:test".to_owned(),
        linked_finding_ids: vec!["finding:test".to_owned()],
        preview_ref: "preview:test".to_owned(),
        checkpoint_ref: "checkpoint:test".to_owned(),
        checkpoint_state: M5RepairCheckpointState::CheckpointAvailable,
        apply_ref: "apply:test".to_owned(),
        receipt_ref: "receipt:test".to_owned(),
        outcome_class: M5RepairOutcomeClass::RepairAppliedExact,
        reversal_class: M5RepairReversalClass::ExactReversal,
        reads_as_single_status: false,
        support_export_available: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_repair_receipt_lineage_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_PACKET_ID
    );
}

#[test]
fn receipt_clean_is_attributable_and_names_outcome() {
    let resolved = resolve_repair_result_receipt_row(clean_receipt_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.outcome_attributable);
    assert!(!resolved.collapses_outcome_into_generic_success);
    assert_eq!(resolved.outcome_class, "repair_applied_exact");
    assert_eq!(
        resolved.repair_disposition,
        Some(M5WorkspaceTrustRepairDisposition::ExactReversal)
    );
    assert!(resolved.checkpoint_present);
    assert_eq!(
        resolved.next_action,
        M5RepairReceiptReviewAction::ReviewReceipt
    );
}

#[test]
fn receipt_partial_success_is_first_class() {
    let mut input = clean_receipt_input();
    input.outcome_class = M5RepairOutcomeClass::RepairPartialSuccess;
    input.applied_scope = vec!["dep: a".to_owned()];
    input.skipped_scope = vec!["dep: b".to_owned()];
    input.reversal_class = M5RepairReversalClass::CompensatingReversal;
    input.reads_as_complete = false;
    let resolved = resolve_repair_result_receipt_row(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.is_partial_success);
    assert!(resolved.requires_follow_up);
    assert!(!resolved.skipped_scope.is_empty());
    assert_eq!(
        resolved.repair_disposition,
        Some(M5WorkspaceTrustRepairDisposition::ManualFollowUp)
    );
}

#[test]
fn receipt_failed_allows_empty_applied_scope_and_has_no_disposition() {
    let mut input = clean_receipt_input();
    input.outcome_class = M5RepairOutcomeClass::RepairFailed;
    input.applied_scope = Vec::new();
    input.skipped_scope = vec!["path: /managed".to_owned()];
    input.reversal_class = M5RepairReversalClass::AuditOnly;
    input.reads_as_complete = false;
    let resolved = resolve_repair_result_receipt_row(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(resolved.repair_disposition, None);
}

#[test]
fn receipt_repair_id_unstated_degrades() {
    let mut input = clean_receipt_input();
    input.repair_id = "  ".to_owned();
    let resolved = resolve_repair_result_receipt_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RepairResultReceiptRowDegradeReason::RepairIdUnstated)
    );
}

#[test]
fn receipt_findings_unstated_degrades() {
    let mut input = clean_receipt_input();
    input.linked_finding_ids = Vec::new();
    let resolved = resolve_repair_result_receipt_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RepairResultReceiptRowDegradeReason::LinkedFindingsUnstated)
    );
}

#[test]
fn receipt_applied_scope_unstated_degrades_for_non_failure() {
    let mut input = clean_receipt_input();
    input.outcome_class = M5RepairOutcomeClass::RepairRegenerated;
    input.applied_scope = Vec::new();
    input.reversal_class = M5RepairReversalClass::RegenerateReversal;
    let resolved = resolve_repair_result_receipt_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RepairResultReceiptRowDegradeReason::AppliedScopeUnstated)
    );
}

#[test]
fn receipt_checkpoint_ref_unstated_degrades() {
    let mut input = clean_receipt_input();
    input.checkpoint_ref = "".to_owned();
    let resolved = resolve_repair_result_receipt_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RepairResultReceiptRowDegradeReason::CheckpointRefUnstated)
    );
}

#[test]
fn receipt_checkpoint_unresolved_degrades() {
    let mut input = clean_receipt_input();
    input.checkpoint_state = M5RepairCheckpointState::CheckpointUnknown;
    let resolved = resolve_repair_result_receipt_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RepairResultReceiptRowDegradeReason::CheckpointStateUnresolved)
    );
}

#[test]
fn receipt_reversal_unresolved_degrades() {
    let mut input = clean_receipt_input();
    input.reversal_class = M5RepairReversalClass::ReversalUnknown;
    let resolved = resolve_repair_result_receipt_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RepairResultReceiptRowDegradeReason::ReversalClassUnresolved)
    );
}

#[test]
fn receipt_follow_up_unstated_degrades() {
    let mut input = clean_receipt_input();
    input.outcome_class = M5RepairOutcomeClass::RepairManualRequired;
    input.reversal_class = M5RepairReversalClass::ManualFollowUp;
    input.reads_as_complete = false;
    input.follow_up_stated = false;
    let resolved = resolve_repair_result_receipt_row(input).unwrap();
    assert!(resolved.hides_follow_up);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RepairResultReceiptRowDegradeReason::FollowUpStateUnstated)
    );
}

#[test]
fn receipt_partial_shown_as_complete_degrades() {
    let mut input = clean_receipt_input();
    input.outcome_class = M5RepairOutcomeClass::RepairPartialSuccess;
    input.applied_scope = vec!["dep: a".to_owned()];
    input.skipped_scope = vec!["dep: b".to_owned()];
    input.reversal_class = M5RepairReversalClass::CompensatingReversal;
    input.follow_up_stated = true;
    input.reads_as_complete = true;
    let resolved = resolve_repair_result_receipt_row(input).unwrap();
    assert!(resolved.presents_partial_as_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RepairResultReceiptRowDegradeReason::PartialSuccessShownAsComplete)
    );
    assert_eq!(
        resolved.next_action,
        M5RepairReceiptReviewAction::ReviewSkippedScope
    );
}

#[test]
fn receipt_generic_success_collapse_degrades() {
    let mut input = clean_receipt_input();
    input.reads_as_generic_success = true;
    let resolved = resolve_repair_result_receipt_row(input).unwrap();
    assert!(resolved.collapses_outcome_into_generic_success);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RepairResultReceiptRowDegradeReason::OutcomeCollapsedIntoGenericSuccess)
    );
}

#[test]
fn receipt_export_missing_degrades() {
    let mut input = clean_receipt_input();
    input.support_export_available = false;
    let resolved = resolve_repair_result_receipt_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RepairResultReceiptRowDegradeReason::SupportExportPathMissing)
    );
    assert_eq!(
        resolved.next_action,
        M5RepairReceiptReviewAction::OpenSupportPacket
    );
}

#[test]
fn receipt_empty_id_and_forbidden_material_error() {
    let mut input = clean_receipt_input();
    input.receipt_id = "".to_owned();
    assert_eq!(
        resolve_repair_result_receipt_row(input).unwrap_err(),
        M5RepairReceiptLineageResolutionError::EmptyReceiptId
    );

    let mut input = clean_receipt_input();
    input.applied_scope = vec!["https://relay.internal/leak".to_owned()];
    assert_eq!(
        resolve_repair_result_receipt_row(input).unwrap_err(),
        M5RepairReceiptLineageResolutionError::ForbiddenMaterial
    );
}

#[test]
fn lineage_clean_is_traceable_end_to_end() {
    let resolved = resolve_checkpoint_lineage_disclosure(clean_lineage_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.lineage_traceable);
    assert!(resolved.lineage_complete);
    assert!(!resolved.severs_lineage);
    assert_eq!(
        resolved.next_action,
        M5RepairReceiptReviewAction::InspectCheckpointLineage
    );
}

#[test]
fn lineage_finding_unstated_degrades_and_severs() {
    let mut input = clean_lineage_input();
    input.linked_finding_ids = Vec::new();
    let resolved = resolve_checkpoint_lineage_disclosure(input).unwrap();
    assert!(resolved.severs_lineage);
    assert!(!resolved.lineage_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CheckpointLineageDisclosureDegradeReason::FindingLinkUnstated)
    );
}

#[test]
fn lineage_preview_apply_result_stages_degrade() {
    for (mutate, expected) in [
        (
            0u8,
            M5CheckpointLineageDisclosureDegradeReason::PreviewRefUnstated,
        ),
        (
            1u8,
            M5CheckpointLineageDisclosureDegradeReason::ApplyRefUnstated,
        ),
        (
            2u8,
            M5CheckpointLineageDisclosureDegradeReason::ResultRefUnstated,
        ),
    ] {
        let mut input = clean_lineage_input();
        match mutate {
            0 => input.preview_ref = "".to_owned(),
            1 => input.apply_ref = "".to_owned(),
            _ => input.receipt_ref = "".to_owned(),
        }
        let resolved = resolve_checkpoint_lineage_disclosure(input).unwrap();
        assert_eq!(resolved.degrade_reason, Some(expected));
    }
}

#[test]
fn lineage_stages_collapsed_degrades() {
    let mut input = clean_lineage_input();
    input.reads_as_single_status = true;
    let resolved = resolve_checkpoint_lineage_disclosure(input).unwrap();
    assert!(resolved.collapses_stages_into_single_status);
    assert!(resolved.severs_lineage);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CheckpointLineageDisclosureDegradeReason::StagesCollapsedIntoSingleStatus)
    );
}

#[test]
fn lineage_checkpoint_unresolved_degrades() {
    let mut input = clean_lineage_input();
    input.checkpoint_state = M5RepairCheckpointState::CheckpointUnknown;
    let resolved = resolve_checkpoint_lineage_disclosure(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CheckpointLineageDisclosureDegradeReason::CheckpointStateUnresolved)
    );
}

#[test]
fn lineage_empty_id_and_forbidden_material_error() {
    let mut input = clean_lineage_input();
    input.disclosure_id = "   ".to_owned();
    assert_eq!(
        resolve_checkpoint_lineage_disclosure(input).unwrap_err(),
        M5RepairReceiptLineageResolutionError::EmptyDisclosureId
    );

    let mut input = clean_lineage_input();
    input.preview_ref = "bearer abc".to_owned();
    assert_eq!(
        resolve_checkpoint_lineage_disclosure(input).unwrap_err(),
        M5RepairReceiptLineageResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_repair_receipt_lineage_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_repair_receipt_lineage_controls();
    packet.vocabulary_set.repair_outcomes.pop();
    assert!(packet
        .validate()
        .contains(&M5RepairReceiptLineageControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_repair_receipt_lineage_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RepairReceiptLineageControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_repair_receipt_lineage_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_REPAIR_RESULT_RECEIPT_ROW_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5RepairReceiptLineageControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_repair_receipt_lineage_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5RepairReceiptLineageAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5RepairReceiptLineageControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_repair_receipt_lineage_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5RepairReceiptLineageExportField::RepairDispositions);
    assert!(packet
        .validate()
        .contains(&M5RepairReceiptLineageControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_repair_receipt_lineage_controls();
    packet.controls_rows[0]
        .checkpoint_lineage_disclosure_examples
        .clear();
    assert!(packet
        .validate()
        .contains(&M5RepairReceiptLineageControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_receipt_example_fails() {
    let mut packet = seeded_m5_repair_receipt_lineage_controls();
    let row = &mut packet.controls_rows[0];
    row.repair_result_receipt_row_examples[0].degrade_reason = None;
    row.repair_result_receipt_row_examples[0].collapses_outcome_into_generic_success = true;
    assert!(packet
        .validate()
        .contains(&M5RepairReceiptLineageControlsViolation::DishonestExample));
}

#[test]
fn dishonest_clean_lineage_example_fails() {
    let mut packet = seeded_m5_repair_receipt_lineage_controls();
    let row = &mut packet.controls_rows[0];
    row.checkpoint_lineage_disclosure_examples[0].degrade_reason = None;
    row.checkpoint_lineage_disclosure_examples[0].severs_lineage = true;
    assert!(packet
        .validate()
        .contains(&M5RepairReceiptLineageControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_repair_receipt_lineage_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.collapses_outcomes_into_generic_success = true,
            1 => row.hides_partial_success_or_follow_up = true,
            2 => row.severs_receipt_from_checkpoint_lineage = true,
            _ => row.requires_feature_local_translation_for_support_export = true,
        }
        assert!(packet
            .validate()
            .contains(&M5RepairReceiptLineageControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn outcome_attributability_not_proven_when_generic_example_removed() {
    let mut packet = seeded_m5_repair_receipt_lineage_controls();
    for row in &mut packet.controls_rows {
        row.repair_result_receipt_row_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5RepairResultReceiptRowDegradeReason::OutcomeCollapsedIntoGenericSuccess)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5RepairReceiptLineageControlsViolation::OutcomeAttributabilityNotProven));
}

#[test]
fn outcome_attributability_not_proven_when_partial_outcome_uncovered() {
    let mut packet = seeded_m5_repair_receipt_lineage_controls();
    for row in &mut packet.controls_rows {
        row.repair_result_receipt_row_examples
            .retain(|ex| !(ex.is_clean() && ex.outcome_class == "repair_partial_success"));
    }
    assert!(packet
        .validate()
        .contains(&M5RepairReceiptLineageControlsViolation::OutcomeAttributabilityNotProven));
}

#[test]
fn lineage_traceability_not_proven_when_stage_collapse_removed() {
    let mut packet = seeded_m5_repair_receipt_lineage_controls();
    for row in &mut packet.controls_rows {
        row.checkpoint_lineage_disclosure_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5CheckpointLineageDisclosureDegradeReason::StagesCollapsedIntoSingleStatus)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5RepairReceiptLineageControlsViolation::LineageTraceabilityNotProven));
}

#[test]
fn lineage_traceability_not_proven_when_partial_lineage_uncovered() {
    let mut packet = seeded_m5_repair_receipt_lineage_controls();
    for row in &mut packet.controls_rows {
        row.checkpoint_lineage_disclosure_examples
            .retain(|ex| !(ex.is_clean() && ex.outcome_class == "repair_partial_success"));
    }
    assert!(packet
        .validate()
        .contains(&M5RepairReceiptLineageControlsViolation::LineageTraceabilityNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_repair_receipt_lineage_controls();
    packet
        .governance_review
        .outcome_never_collapsed_into_generic_success = false;
    assert!(packet
        .validate()
        .contains(&M5RepairReceiptLineageControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_repair_receipt_lineage_controls();
    packet
        .consumer_projection
        .partial_success_legible_without_logs = false;
    assert!(packet
        .validate()
        .contains(&M5RepairReceiptLineageControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_repair_receipt_lineage_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5RepairReceiptLineageControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_repair_receipt_lineage_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5RepairReceiptLineageControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_repair_receipt_lineage_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5RepairReceiptLineageControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_repair_receipt_lineage_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_repair_receipt_lineage_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_repair_receipt_lineage_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_repair_receipt_lineage_controls_export()
        .expect("checked M5 repair-receipt-lineage controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_REPAIR_RECEIPT_LINEAGE_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_repair_receipt_lineage_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_repair_receipt_lineage_controls_doctor_ui_beta_narrowed();
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

    let preview = seeded_m5_repair_receipt_lineage_controls_safe_mode_ui_preview_narrowed();
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
    let beta: M5RepairReceiptLineageControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-repair-result-receipt-row-checkpoint-lineage-disclosure-controls/doctor_ui_beta_narrowed.json"
    )))
    .expect("doctor-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_repair_receipt_lineage_controls_doctor_ui_beta_narrowed()
    );

    let preview: M5RepairReceiptLineageControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-repair-result-receipt-row-checkpoint-lineage-disclosure-controls/safe_mode_ui_preview_narrowed.json"
    )))
    .expect("safe-mode-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_repair_receipt_lineage_controls_safe_mode_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_the_repair_result_receipt_row() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [M5WorkspaceTrustRepairComponentFamily::RepairResultReceiptRow]
    );
}
