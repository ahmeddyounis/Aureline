use super::*;

fn packet() -> DoctorRepairContainerMaturityMatrix {
    current_doctor_repair_container_maturity_matrix().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        DOCTOR_REPAIR_CONTAINER_MATURITY_MATRIX_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        DOCTOR_REPAIR_CONTAINER_MATURITY_MATRIX_RECORD_KIND
    );
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn matrix_is_complete_and_unique() {
    let packet = packet();
    assert_eq!(
        packet.rows.len(),
        packet.capabilities.len() * packet.deployment_profiles.len()
    );
    for &capability in &packet.capabilities {
        for &profile in &packet.deployment_profiles {
            assert!(
                packet.row(capability, profile).is_some(),
                "missing cell {}/{}",
                capability.as_str(),
                profile.as_str()
            );
        }
    }
}

#[test]
fn every_row_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_rows_gate_consistent());
    for row in &packet.rows {
        assert_eq!(
            row.published_maturity,
            row.effective_maturity(),
            "row {} publishes beyond the gate",
            row.row_id
        );
        assert_eq!(
            row.narrowing_action,
            row.required_narrowing(),
            "row {} narrowing diverges from the gate",
            row.row_id
        );
    }
}

#[test]
fn every_row_carries_its_own_evidence() {
    let packet = packet();
    for row in &packet.rows {
        assert!(
            !row.scorecard_ref.trim().is_empty(),
            "row {} has no scorecard",
            row.row_id
        );
        assert!(
            !row.latency_corpus_ref.trim().is_empty(),
            "row {} has no diagnosis-latency corpus",
            row.row_id
        );
        assert!(
            !row.rollback_ref.trim().is_empty(),
            "row {} has no rollback path",
            row.row_id
        );
        assert!(
            !row.compatibility_ref.trim().is_empty(),
            "row {} has no compatibility story",
            row.row_id
        );
    }
}

#[test]
fn guided_repair_rows_carry_a_reversal_class() {
    let packet = packet();
    for row in &packet.rows {
        if row.capability.is_mutating_repair() {
            assert_ne!(
                row.reversal_class,
                ReversalClass::NotApplicable,
                "guided-repair row {} has no reversal class",
                row.row_id
            );
        }
        assert!(row.reversal_class_consistent());
    }
}

#[test]
fn export_projection_reflects_rows_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.rows.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_rows_gate_consistent,
        packet.all_rows_gate_consistent()
    );
    assert_eq!(
        projection.promotable_count,
        packet.promotable_rows().count()
    );
    assert_eq!(projection.narrowed_count, packet.narrowed_rows().count());
    assert_eq!(projection.withheld_count, packet.withheld_rows().count());
}

#[test]
fn published_maturities_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<MaturityClass> =
        packet.rows.iter().map(|r| r.published_maturity).collect();
    for maturity in MaturityClass::ALL {
        assert!(
            present.contains(&maturity),
            "no row publishes maturity {}",
            maturity.as_str()
        );
    }
}

#[test]
fn narrowing_actions_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<NarrowingAction> =
        packet.rows.iter().map(|r| r.narrowing_action).collect();
    for action in NarrowingAction::ALL {
        assert!(
            present.contains(&action),
            "no row exercises narrowing {}",
            action.as_str()
        );
    }
}

#[test]
fn freshness_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<EvidenceFreshness> =
        packet.rows.iter().map(|r| r.evidence_freshness).collect();
    for freshness in EvidenceFreshness::ALL {
        assert!(
            present.contains(&freshness),
            "no row exercises freshness {}",
            freshness.as_str()
        );
    }
}

#[test]
fn reversal_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ReversalClass> = packet.rows.iter().map(|r| r.reversal_class).collect();
    for reversal in ReversalClass::ALL {
        assert!(
            present.contains(&reversal),
            "no row exercises reversal class {}",
            reversal.as_str()
        );
    }
}

#[test]
fn support_parities_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<SupportParity> = packet.rows.iter().map(|r| r.support_parity).collect();
    for parity in SupportParity::ALL {
        assert!(
            present.contains(&parity),
            "no row exercises support parity {}",
            parity.as_str()
        );
    }
}

#[test]
fn blocking_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<BlockingReason> = packet
        .rows
        .iter()
        .flat_map(|r| r.blocking_reasons.iter().copied())
        .collect();
    for reason in BlockingReason::ALL {
        assert!(
            present.contains(&reason),
            "no row exercises blocking reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn promotable_rows_are_clean() {
    let packet = packet();
    assert!(
        packet.promotable_rows().count() > 0,
        "fixture needs a certified row"
    );
    for row in packet.promotable_rows() {
        assert!(row.evidence_freshness.is_current());
        assert!(row.blocking_reasons.is_empty());
        assert_eq!(row.published_maturity, MaturityClass::Certified);
        assert_eq!(row.narrowing_action, NarrowingAction::None);
    }
}

#[test]
fn gate_narrows_stale_and_expired_rows() {
    assert_eq!(
        EvidenceFreshness::Stale.maturity_ceiling(),
        MaturityClass::Provisional
    );
    assert_eq!(
        EvidenceFreshness::Expired.maturity_ceiling(),
        MaturityClass::Underqualified
    );
    assert_eq!(
        EvidenceFreshness::Unknown.maturity_ceiling(),
        MaturityClass::Provisional
    );
    assert_eq!(
        EvidenceFreshness::Current.maturity_ceiling(),
        MaturityClass::Certified
    );
}

#[test]
fn blocking_reason_ceilings_hold() {
    assert_eq!(
        BlockingReason::Stale.maturity_ceiling(),
        MaturityClass::Provisional
    );
    assert_eq!(
        BlockingReason::EngineUnavailable.maturity_ceiling(),
        MaturityClass::Provisional
    );
    assert_eq!(
        BlockingReason::LatencySloBreached.maturity_ceiling(),
        MaturityClass::Provisional
    );
    assert_eq!(
        BlockingReason::MissingProofCorpus.maturity_ceiling(),
        MaturityClass::Underqualified
    );
    assert_eq!(
        BlockingReason::MissingRollbackPath.maturity_ceiling(),
        MaturityClass::Underqualified
    );
    assert_eq!(
        BlockingReason::BoundaryUnverified.maturity_ceiling(),
        MaturityClass::Underqualified
    );
}

#[test]
fn missing_evidence_blocks_promotion() {
    assert!(BlockingReason::MissingProofCorpus.is_missing_evidence());
    assert!(BlockingReason::MissingRollbackPath.is_missing_evidence());
    assert!(BlockingReason::BoundaryUnverified.is_missing_evidence());
    assert!(!BlockingReason::Stale.is_missing_evidence());
    assert!(!BlockingReason::EngineUnavailable.is_missing_evidence());
    assert!(!BlockingReason::LatencySloBreached.is_missing_evidence());
}

#[test]
fn validate_flags_overstated_published_maturity() {
    let mut packet = packet();
    if let Some(row) = packet
        .rows
        .iter_mut()
        .find(|r| r.evidence_freshness != EvidenceFreshness::Current)
    {
        row.published_maturity = MaturityClass::Certified;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            DoctorRepairContainerMaturityMatrixViolation::OverstatedPublishedMaturity { .. }
        )));
    }
}

#[test]
fn validate_flags_narrowing_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .rows
        .iter_mut()
        .find(|r| r.narrowing_action != NarrowingAction::WithholdFromPublication)
    {
        row.narrowing_action = NarrowingAction::WithholdFromPublication;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            DoctorRepairContainerMaturityMatrixViolation::NarrowingActionMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_promoted_row_not_clean() {
    let mut packet = packet();
    // Add a blocking reason to a promotable row without changing its published
    // maturity: it is now internally inconsistent and not clean.
    if let Some(row) = packet.rows.iter_mut().find(|r| r.is_promotable()) {
        row.blocking_reasons.push(BlockingReason::Stale);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            DoctorRepairContainerMaturityMatrixViolation::OverstatedPublishedMaturity { .. }
        )));
    }
}

#[test]
fn validate_flags_repair_lane_missing_reversal_class() {
    let mut packet = packet();
    if let Some(row) = packet
        .rows
        .iter_mut()
        .find(|r| r.capability.is_mutating_repair())
    {
        row.reversal_class = ReversalClass::NotApplicable;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            DoctorRepairContainerMaturityMatrixViolation::RepairLaneMissingReversalClass { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_matrix_cell() {
    let mut packet = packet();
    let removed = packet.rows.pop();
    assert!(removed.is_some());
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        DoctorRepairContainerMaturityMatrixViolation::MissingMatrixCell { .. }
    )));
}

#[test]
fn validate_flags_unclaimed_capability_row() {
    let mut packet = packet();
    packet
        .capabilities
        .retain(|c| *c != RecoveryCapability::ContainerBoundary);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        DoctorRepairContainerMaturityMatrixViolation::UnclaimedCapabilityRow { .. }
    )));
    assert!(violations.iter().any(|v| matches!(
        v,
        DoctorRepairContainerMaturityMatrixViolation::ClosedVocabularyMismatch {
            field: "capabilities"
        }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_rows = packet.summary.total_rows.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&DoctorRepairContainerMaturityMatrixViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(RecoveryCapability::ProjectDoctor.as_str(), "project_doctor");
    assert_eq!(RecoveryCapability::GuidedRepair.as_str(), "guided_repair");
    assert_eq!(
        RecoveryCapability::ContainerBoundary.as_str(),
        "container_boundary"
    );
    assert_eq!(
        DeploymentProfile::LocalWorkspace.as_str(),
        "local_workspace"
    );
    assert_eq!(DeploymentProfile::RemoteSsh.as_str(), "remote_ssh");
    assert_eq!(DeploymentProfile::Container.as_str(), "container");
    assert_eq!(DeploymentProfile::Devcontainer.as_str(), "devcontainer");
    assert_eq!(MaturityClass::Certified.as_str(), "certified");
    assert_eq!(ReversalClass::Checkpointed.as_str(), "checkpointed");
    assert_eq!(
        NarrowingAction::WithholdFromPublication.as_str(),
        "withhold_from_publication"
    );
}

#[test]
fn maturity_rank_orders_low_to_high() {
    assert!(MaturityClass::Unsupported.rank() < MaturityClass::Underqualified.rank());
    assert!(MaturityClass::Underqualified.rank() < MaturityClass::Provisional.rank());
    assert!(MaturityClass::Provisional.rank() < MaturityClass::Certified.rank());
    assert_eq!(
        MaturityClass::Certified.min(MaturityClass::Provisional),
        MaturityClass::Provisional
    );
    assert_eq!(
        MaturityClass::Underqualified.min(MaturityClass::Certified),
        MaturityClass::Underqualified
    );
}
