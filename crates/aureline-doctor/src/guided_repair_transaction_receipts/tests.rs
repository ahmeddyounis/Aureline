use super::*;

fn packet() -> ProjectDoctorRepairTransactionReceipts {
    current_project_doctor_repair_transaction_receipts().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        PROJECT_DOCTOR_REPAIR_TRANSACTION_RECEIPTS_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        PROJECT_DOCTOR_REPAIR_TRANSACTION_RECEIPTS_RECORD_KIND
    );
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_records() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_receipt_declares_before_mutation() {
    let packet = packet();
    for receipt in &packet.receipts {
        assert!(
            receipt.repair_id.starts_with(DOCTOR_REPAIR_PREFIX),
            "{} repair prefix",
            receipt.receipt_id
        );
        assert!(receipt.receipt_id.starts_with(RECEIPT_ID_PREFIX));
        assert!(!receipt.initiating_findings.is_empty());
        for finding in &receipt.initiating_findings {
            assert!(finding.starts_with(DOCTOR_FINDING_PREFIX));
        }
        assert!(!receipt.impacted_state_classes.is_empty());
        assert!(!receipt.preconditions.is_empty());
        assert!(!receipt.verification_plan.is_empty());
        assert!(receipt.has_stage(RepairStage::Review));
    }
}

#[test]
fn receipts_link_findings_objects_results_and_support() {
    let packet = packet();
    for receipt in &packet.receipts {
        assert!(!receipt.initiating_findings.is_empty());
        assert!(!receipt.affected_objects.is_empty());
        assert!(!receipt.verification_results.is_empty());
        assert!(receipt.offers_support_path(), "{}", receipt.receipt_id);
    }
}

#[test]
fn stages_run_in_canonical_order() {
    let packet = packet();
    for receipt in &packet.receipts {
        let mut last: Option<u8> = None;
        for record in &receipt.stages {
            if let Some(prev) = last {
                assert!(
                    record.stage.order() > prev,
                    "{} stage {} out of order",
                    receipt.receipt_id,
                    record.stage.as_str()
                );
            }
            last = Some(record.stage.order());
        }
        assert!(
            !(receipt.has_stage(RepairStage::Rollback)
                && receipt.has_stage(RepairStage::Compensate)),
            "{} has both rollback and compensate",
            receipt.receipt_id
        );
    }
}

#[test]
fn checkpoint_stage_present_iff_checkpoint_captured() {
    let packet = packet();
    for receipt in &packet.receipts {
        assert_eq!(
            receipt.has_stage(RepairStage::Checkpoint),
            receipt.checkpoint.present,
            "{} checkpoint stage/disclosure mismatch",
            receipt.receipt_id
        );
        assert!(receipt.checkpoint.is_consistent(), "{}", receipt.receipt_id);
    }
}

#[test]
fn reversal_class_agrees_with_checkpoint_kind() {
    let packet = packet();
    for receipt in &packet.receipts {
        match receipt.reversal_class {
            ReversalClass::ReversibleTransactional => {
                assert_eq!(
                    receipt.checkpoint.checkpoint_kind,
                    CheckpointKind::TransactionalSnapshot
                );
            }
            ReversalClass::ReversibleWithSnapshot => {
                assert!(matches!(
                    receipt.checkpoint.checkpoint_kind,
                    CheckpointKind::FilesystemSnapshot | CheckpointKind::StateExport
                ));
            }
            ReversalClass::CompensatingOnly | ReversalClass::IrreversibleGuarded => {}
        }
    }
}

#[test]
fn no_checkpoint_never_promises_clean_reversibility() {
    let packet = packet();
    for receipt in packet.receipts.iter().filter(|r| !r.checkpoint.present) {
        assert!(
            !receipt.reversal_class.requires_checkpoint(),
            "{} promises reversibility without a checkpoint",
            receipt.receipt_id
        );
        assert!(
            receipt.offers_support_path(),
            "{} no support path",
            receipt.receipt_id
        );
        assert_ne!(
            receipt.completion_state,
            CompletionState::RolledBackExact,
            "{} claims exact rollback without a checkpoint",
            receipt.receipt_id
        );
    }
}

#[test]
fn durable_state_mutation_is_always_guarded() {
    let packet = packet();
    let mut durable = 0;
    for receipt in &packet.receipts {
        if receipt.mutates_durable_user_state {
            durable += 1;
            assert!(
                receipt.durable_state_is_guarded(),
                "{} unguarded durable mutation",
                receipt.receipt_id
            );
        }
    }
    assert!(durable >= 1, "corpus needs a durable-state receipt");
}

#[test]
fn exact_rollback_requires_checkpoint() {
    let packet = packet();
    for receipt in packet.receipts_in_state(CompletionState::RolledBackExact) {
        assert!(receipt.checkpoint.present, "{}", receipt.receipt_id);
        assert!(receipt.has_stage(RepairStage::Rollback));
    }
}

#[test]
fn compensating_rollback_uses_compensate_stage() {
    let packet = packet();
    for receipt in packet.receipts_in_state(CompletionState::RolledBackCompensating) {
        assert!(
            receipt.has_stage(RepairStage::Compensate),
            "{}",
            receipt.receipt_id
        );
        assert!(!receipt.has_stage(RepairStage::Rollback));
    }
}

#[test]
fn fixed_receipts_verify_cleanly() {
    let packet = packet();
    for receipt in packet.receipts_in_state(CompletionState::Fixed) {
        assert_eq!(
            receipt.stage(RepairStage::Verify).map(|s| s.status),
            Some(StageStatus::Passed)
        );
        assert!(receipt
            .verification_results
            .iter()
            .all(|r| r.outcome == VerificationOutcome::Passed));
        assert!(!receipt.completion_state.is_rollback());
    }
}

#[test]
fn receipts_are_cross_surface_stable() {
    let packet = packet();
    for receipt in &packet.receipts {
        assert!(
            receipt.is_cross_surface_stable(),
            "{} not cross-surface stable",
            receipt.receipt_id
        );
    }
}

#[test]
fn receipts_carry_locale_invariant_keys() {
    let packet = packet();
    for receipt in &packet.receipts {
        for required in REQUIRED_MACHINE_MEANING_KEYS {
            assert!(
                receipt.machine_meaning_keys.iter().any(|k| k == required),
                "{} missing machine-meaning key {required}",
                receipt.receipt_id
            );
        }
    }
}

#[test]
fn receipts_are_read_only_and_metadata_safe() {
    let packet = packet();
    for receipt in &packet.receipts {
        assert!(receipt.raw_private_material_excluded);
        assert_eq!(receipt.redaction_class, "metadata_safe_default");
    }
}

#[test]
fn failure_families_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<FailureFamily> =
        packet.receipts.iter().map(|r| r.failure_family).collect();
    for family in FailureFamily::ALL {
        assert!(present.contains(&family), "no receipt for family {family}");
    }
}

#[test]
fn completion_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<CompletionState> =
        packet.receipts.iter().map(|r| r.completion_state).collect();
    for state in CompletionState::ALL {
        assert!(
            present.contains(&state),
            "no receipt in completion state {}",
            state.as_str()
        );
    }
}

#[test]
fn reversal_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ReversalClass> =
        packet.receipts.iter().map(|r| r.reversal_class).collect();
    for class in ReversalClass::ALL {
        assert!(
            present.contains(&class),
            "no receipt with reversal class {}",
            class.as_str()
        );
    }
}

#[test]
fn checkpoint_kinds_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<CheckpointKind> = packet
        .receipts
        .iter()
        .map(|r| r.checkpoint.checkpoint_kind)
        .collect();
    for kind in CheckpointKind::ALL {
        assert!(
            present.contains(&kind),
            "no receipt with checkpoint kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn host_boundaries_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<HostBoundary> = packet.receipts.iter().map(|r| r.host_boundary).collect();
    for boundary in HostBoundary::ALL {
        assert!(
            present.contains(&boundary),
            "no receipt on host boundary {}",
            boundary.as_str()
        );
    }
}

#[test]
fn export_projection_reflects_receipts() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.receipts.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.with_checkpoint_count,
        packet
            .receipts
            .iter()
            .filter(|r| r.checkpoint.present)
            .count()
    );
    assert_eq!(
        projection.rolled_back_count,
        packet
            .receipts
            .iter()
            .filter(|r| r.completion_state.is_rollback())
            .count()
    );
    assert_eq!(projection.cross_surface_stable_count, packet.receipts.len());
}

#[test]
fn validate_flags_reversibility_without_checkpoint() {
    let mut packet = packet();
    if let Some(receipt) = packet.receipts.iter_mut().find(|r| !r.checkpoint.present) {
        receipt.reversal_class = ReversalClass::ReversibleTransactional;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorRepairTransactionReceiptsViolation::ReversibilityWithoutCheckpoint { .. }
        )));
    }
}

#[test]
fn validate_flags_unguarded_durable_mutation() {
    let mut packet = packet();
    // Find a receipt that mutates durable state but has no checkpoint (guarded
    // only by its irreversible-guarded class + support path); strip the support
    // path and downgrade the class so the durable mutation is unguarded.
    if let Some(receipt) = packet
        .receipts
        .iter_mut()
        .find(|r| r.mutates_durable_user_state && !r.checkpoint.present)
    {
        receipt.reversal_class = ReversalClass::CompensatingOnly;
        receipt.support_paths = Vec::new();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorRepairTransactionReceiptsViolation::UnguardedDurableMutation { .. }
        )));
    }
}

#[test]
fn validate_flags_exact_rollback_without_checkpoint() {
    let mut packet = packet();
    if let Some(receipt) = packet
        .receipts
        .iter_mut()
        .find(|r| r.completion_state == CompletionState::RolledBackExact)
    {
        receipt.checkpoint = CheckpointDisclosure {
            present: false,
            checkpoint_kind: CheckpointKind::None,
            checkpoint_ref: String::new(),
        };
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorRepairTransactionReceiptsViolation::ExactRollbackWithoutCheckpoint { .. }
        )));
    }
}

#[test]
fn validate_flags_no_checkpoint_without_support_path() {
    let mut packet = packet();
    if let Some(receipt) = packet
        .receipts
        .iter_mut()
        .find(|r| !r.checkpoint.present && r.reversal_class == ReversalClass::CompensatingOnly)
    {
        receipt.support_paths = Vec::new();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorRepairTransactionReceiptsViolation::NoCheckpointWithoutSupportPath { .. }
        )));
    }
}

#[test]
fn validate_flags_checkpoint_stage_mismatch() {
    let mut packet = packet();
    if let Some(receipt) = packet.receipts.iter_mut().find(|r| !r.checkpoint.present) {
        receipt.checkpoint = CheckpointDisclosure {
            present: true,
            checkpoint_kind: CheckpointKind::TransactionalSnapshot,
            checkpoint_ref: "checkpoint:injected".to_owned(),
        };
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorRepairTransactionReceiptsViolation::CheckpointStageMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_reversal_checkpoint_mismatch() {
    let mut packet = packet();
    if let Some(receipt) = packet
        .receipts
        .iter_mut()
        .find(|r| r.reversal_class == ReversalClass::ReversibleTransactional)
    {
        receipt.checkpoint.checkpoint_kind = CheckpointKind::FilesystemSnapshot;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorRepairTransactionReceiptsViolation::ReversalCheckpointMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_completion_state_inconsistent() {
    let mut packet = packet();
    if let Some(receipt) = packet
        .receipts
        .iter_mut()
        .find(|r| r.completion_state == CompletionState::Fixed)
    {
        // A fixed receipt with a failed verification result is inconsistent.
        receipt.completion_state = CompletionState::VerificationInconclusive;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorRepairTransactionReceiptsViolation::CompletionStateInconsistent { .. }
        )));
    }
}

#[test]
fn validate_flags_stages_out_of_order() {
    let mut packet = packet();
    if let Some(receipt) = packet.receipts.first_mut() {
        receipt.stages.reverse();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorRepairTransactionReceiptsViolation::StagesOutOfOrder { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_required_stage() {
    let mut packet = packet();
    if let Some(receipt) = packet.receipts.first_mut() {
        receipt.stages.retain(|s| s.stage != RepairStage::Verify);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorRepairTransactionReceiptsViolation::MissingRequiredStage { .. }
        )));
    }
}

#[test]
fn validate_flags_finding_prefix() {
    let mut packet = packet();
    if let Some(receipt) = packet.receipts.first_mut() {
        receipt.initiating_findings = vec!["illegal.finding".to_owned()];
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            ProjectDoctorRepairTransactionReceiptsViolation::FindingCodePrefix { .. }
        )));
    }
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.receipt_count = packet.summary.receipt_count.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&ProjectDoctorRepairTransactionReceiptsViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(FailureFamily::NotebookKernel.as_str(), "notebook_kernel");
    assert_eq!(FailureFamily::IncidentPacket.as_str(), "incident_packet");
    assert_eq!(HostBoundary::Devcontainer.as_str(), "devcontainer");
    assert_eq!(
        CheckpointKind::TransactionalSnapshot.as_str(),
        "transactional_snapshot"
    );
    assert_eq!(
        ReversalClass::CompensatingOnly.as_str(),
        "compensating_only"
    );
    assert_eq!(RepairStage::Compensate.as_str(), "compensate");
    assert_eq!(StageStatus::Inconclusive.as_str(), "inconclusive");
    assert_eq!(VerificationOutcome::Failed.as_str(), "failed");
    assert_eq!(
        CompletionState::ReducedButNotResolved.as_str(),
        "reduced_but_not_resolved"
    );
    assert_eq!(ParitySurface::DesktopReceipt.as_str(), "desktop_receipt");
}

#[test]
fn rollback_and_compensate_share_terminal_slot() {
    assert_eq!(
        RepairStage::Rollback.order(),
        RepairStage::Compensate.order()
    );
    assert!(RepairStage::Rollback.order() > RepairStage::Verify.order());
}
