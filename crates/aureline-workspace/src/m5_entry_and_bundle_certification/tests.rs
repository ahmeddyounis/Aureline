use super::*;

use crate::m5_entry_and_bundle_governance::current_m5_entry_bundle_governance_matrix;

fn packet() -> M5EntryBundleCertificationReport {
    current_m5_entry_bundle_certification_report().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_ENTRY_BUNDLE_CERTIFICATION_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        M5_ENTRY_BUNDLE_CERTIFICATION_RECORD_KIND
    );
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_claimed_lane_has_exactly_one_row() {
    let packet = packet();
    assert_eq!(packet.rows.len(), packet.lanes.len());
    for &lane in &packet.lanes {
        assert!(
            packet.row(lane).is_some(),
            "missing row for lane {}",
            lane.as_str()
        );
    }
}

#[test]
fn every_row_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_rows_gate_consistent());
    for row in &packet.rows {
        assert_eq!(row.published_label, row.effective_label(), "{}", row.row_id);
        assert_eq!(
            row.certification_decision,
            row.required_decision(),
            "{}",
            row.row_id
        );
        assert_eq!(
            row.downgrade_reasons,
            row.computed_downgrade_reasons(),
            "{}",
            row.row_id
        );
        assert_eq!(
            row.downgrade_path,
            row.computed_downgrade_path(),
            "{}",
            row.row_id
        );
    }
}

#[test]
fn every_row_covers_all_drills_with_evidence() {
    let packet = packet();
    for row in &packet.rows {
        assert!(row.covers_all_drills(), "{} misses a drill", row.row_id);
        for result in &row.drill_results {
            assert!(
                result.has_required_evidence(),
                "{} drill {} ran without evidence",
                row.row_id,
                result.drill.as_str()
            );
        }
    }
}

#[test]
fn every_row_carries_required_evidence_refs() {
    let packet = packet();
    for row in &packet.rows {
        assert!(row.has_required_evidence(), "{}", row.row_id);
    }
}

#[test]
fn certification_never_exceeds_governance() {
    // The cornerstone non-inheritance guarantee: the certification ingests the governance
    // matrix and never re-broadens a governance-narrowed lane.
    let packet = packet();
    let governance = current_m5_entry_bundle_governance_matrix().expect("governance packet parses");
    for row in &packet.rows {
        let lane = governance
            .lane_row(row.lane)
            .expect("lane has a governance row");
        assert_eq!(
            row.governance_claim, lane.published_assurance,
            "{} governance claim diverges from the matrix",
            row.row_id
        );
        assert!(
            row.published_label.rank() <= row.governance_claim.rank(),
            "{} certifies above its governance claim",
            row.row_id
        );
        assert_eq!(
            row.governance_row_ref, lane.lane_id,
            "{} binds to the wrong governance row",
            row.row_id
        );
    }
}

#[test]
fn narrowed_rows_offer_recovery_and_caveats() {
    let packet = packet();
    for row in &packet.rows {
        if row.certification_decision.is_narrowed() {
            assert!(row.downgrade_path.is_offered(), "{}", row.row_id);
            assert!(!row.caveats.is_empty(), "{}", row.row_id);
            assert!(!row.stale_or_missing_fields.is_empty(), "{}", row.row_id);
        }
    }
}

#[test]
fn every_required_consumer_surface_binds() {
    let packet = packet();
    for surface in CertificationConsumerSurface::REQUIRED {
        assert!(
            packet.has_binding_for(surface),
            "missing binding for {}",
            surface.as_str()
        );
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
    assert_eq!(projection.certified_count, packet.certified_rows().count());
    assert_eq!(projection.narrowed_count, packet.narrowed_rows().count());
    assert_eq!(projection.withheld_count, packet.withheld_rows().count());
    for (row, export) in packet.rows.iter().zip(projection.rows.iter()) {
        assert_eq!(export.published_label, row.published_label.as_str());
        assert_eq!(export.certified, row.is_certified());
        assert_eq!(export.downgraded, row.is_downgraded());
    }
}

#[test]
fn support_export_is_export_safe() {
    let packet = packet();
    let export = packet.support_export(
        "support:m5:entry-bundle-certification",
        "2026-06-11T13:00:00Z",
    );
    assert!(export.is_export_safe());
    assert_eq!(export.certification_packet_id_ref, packet.packet_id);
    assert!(export.raw_private_material_excluded);
}

#[test]
fn published_labels_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<EntryAssurance> = packet.rows.iter().map(|r| r.published_label).collect();
    for label in EntryAssurance::ALL {
        assert!(
            present.contains(&label),
            "no row publishes {}",
            label.as_str()
        );
    }
}

#[test]
fn certification_decisions_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<AdmissionOutcome> = packet
        .rows
        .iter()
        .map(|r| r.certification_decision)
        .collect();
    for decision in AdmissionOutcome::ALL {
        assert!(
            present.contains(&decision),
            "no row exercises {}",
            decision.as_str()
        );
    }
}

#[test]
fn evidence_freshness_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<EvidenceFreshness> =
        packet.rows.iter().map(|r| r.evidence_freshness).collect();
    for state in EvidenceFreshness::ALL {
        assert!(
            present.contains(&state),
            "no row exercises {}",
            state.as_str()
        );
    }
}

#[test]
fn downgrade_paths_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<CertificationDowngradePath> =
        packet.rows.iter().map(|r| r.downgrade_path).collect();
    for path in CertificationDowngradePath::ALL {
        assert!(
            present.contains(&path),
            "no row exercises {}",
            path.as_str()
        );
    }
}

#[test]
fn downgrade_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<CertificationDowngradeReason> = packet
        .rows
        .iter()
        .flat_map(|r| r.downgrade_reasons.iter().copied())
        .collect();
    for reason in CertificationDowngradeReason::ALL {
        assert!(
            present.contains(&reason),
            "no row exercises {}",
            reason.as_str()
        );
    }
}

#[test]
fn drill_outcomes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<DrillOutcome> = packet
        .rows
        .iter()
        .flat_map(|r| r.drill_results.iter().map(|d| d.outcome))
        .collect();
    for outcome in DrillOutcome::ALL {
        assert!(
            present.contains(&outcome),
            "no drill exercises {}",
            outcome.as_str()
        );
    }
}

#[test]
fn certified_rows_are_whole() {
    let packet = packet();
    assert!(
        packet.certified_rows().count() >= 2,
        "fixture needs at least two certified rows to prove the gate is not a blanket downgrade"
    );
    for row in packet.certified_rows() {
        assert_eq!(row.governance_claim, EntryAssurance::Verified);
        assert_eq!(row.evidence_freshness, EvidenceFreshness::Current);
        assert_eq!(row.drill_ceiling(), EntryAssurance::Verified);
        assert!(row.downgrade_reasons.is_empty());
        assert!(row.caveats.is_empty());
        assert!(!row.downgrade_path.is_offered());
        assert!(!row.is_downgraded());
        assert!(!row.supported_profiles.is_empty());
    }
}

#[test]
fn governance_narrowed_row_adopts_narrowing() {
    let packet = packet();
    let row = packet
        .row(EntryBundleLane::WorkflowBundle)
        .expect("workflow-bundle row");
    assert_eq!(row.published_label, EntryAssurance::RetestPending);
    assert_eq!(row.certification_decision, AdmissionOutcome::AdmitRetest);
    assert_eq!(
        row.downgrade_path,
        CertificationDowngradePath::AdoptGovernanceNarrowing
    );
    assert_eq!(
        row.downgrade_reasons,
        vec![CertificationDowngradeReason::GovernanceNarrowed]
    );
}

#[test]
fn withheld_row_is_withheld_not_inherited() {
    let packet = packet();
    let row = packet
        .row(EntryBundleLane::WorkspaceAdmission)
        .expect("workspace-admission row");
    assert_eq!(row.published_label, EntryAssurance::Withheld);
    assert_eq!(row.certification_decision, AdmissionOutcome::Refuse);
    assert_eq!(row.downgrade_path, CertificationDowngradePath::WithholdRow);
    assert!(row.supported_profiles.is_empty());
    assert!(row
        .downgrade_reasons
        .contains(&CertificationDowngradeReason::GovernanceNarrowed));
    assert!(row
        .downgrade_reasons
        .contains(&CertificationDowngradeReason::DrillFailed));
}

#[test]
fn validate_flags_overstated_label() {
    let mut packet = packet();
    if let Some(row) = packet
        .rows
        .iter_mut()
        .find(|r| r.effective_label() != EntryAssurance::Verified)
    {
        row.published_label = EntryAssurance::Verified;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5EntryBundleCertificationViolation::OverstatedLabel { .. }
        )));
    }
}

#[test]
fn validate_flags_exceeds_governance() {
    let mut packet = packet();
    // Force a row to publish above its governance claim without changing the gate's other
    // inputs, so the dedicated guard fires.
    if let Some(row) = packet
        .rows
        .iter_mut()
        .find(|r| r.governance_claim == EntryAssurance::RetestPending)
    {
        row.governance_claim = EntryAssurance::Withheld;
        // published_label stays retest_pending, now above the withheld governance claim.
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5EntryBundleCertificationViolation::ExceedsGovernance { .. }
        )));
    }
}

#[test]
fn validate_flags_incomplete_drill_coverage() {
    let mut packet = packet();
    if let Some(row) = packet.rows.first_mut() {
        row.drill_results.pop();
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5EntryBundleCertificationViolation::IncompleteDrillCoverage { .. }
        )));
    }
}

#[test]
fn validate_flags_drill_missing_evidence() {
    let mut packet = packet();
    if let Some(result) = packet
        .rows
        .iter_mut()
        .flat_map(|r| r.drill_results.iter_mut())
        .find(|d| d.outcome.was_run())
    {
        result.evidence_ref = None;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5EntryBundleCertificationViolation::DrillMissingEvidence { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_consumer_binding() {
    let mut packet = packet();
    packet
        .consumer_bindings
        .retain(|b| b.consumer_surface != CertificationConsumerSurface::SupportExport);
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        M5EntryBundleCertificationViolation::MissingConsumerBinding { .. }
    )));
}

#[test]
fn validate_flags_binding_that_stops_narrowing() {
    let mut packet = packet();
    if let Some(binding) = packet.consumer_bindings.first_mut() {
        binding.narrows_on_downgrade = false;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5EntryBundleCertificationViolation::ConsumerBindingDrift { .. }
        )));
    }
}

#[test]
fn validate_flags_governance_packet_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet.rows.first_mut() {
        row.governance_packet_ref = "artifacts/workspace/m5/not-governance.json".to_owned();
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5EntryBundleCertificationViolation::GovernancePacketMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_rows = packet.summary.total_rows.wrapping_add(1);
    assert!(packet
        .validate()
        .contains(&M5EntryBundleCertificationViolation::SummaryMismatch));
}

#[test]
fn validate_flags_decision_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .rows
        .iter_mut()
        .find(|r| r.certification_decision != AdmissionOutcome::Refuse)
    {
        row.certification_decision = AdmissionOutcome::Refuse;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5EntryBundleCertificationViolation::DecisionMismatch { .. }
        )));
    }
}

#[test]
fn tokens_are_stable() {
    assert_eq!(
        CertificationDrill::BundleLifecycle.as_str(),
        "bundle_lifecycle"
    );
    assert_eq!(CertificationDrill::Downgrade.as_str(), "downgrade");
    assert_eq!(DrillOutcome::NotRun.as_str(), "not_run");
    assert_eq!(EvidenceFreshness::Missing.as_str(), "missing");
    assert_eq!(
        CertificationDowngradeReason::DrillFailed.as_str(),
        "drill_failed"
    );
    assert_eq!(CertificationDowngradePath::NoneNeeded.as_str(), "none");
    assert_eq!(
        CertificationDowngradePath::AdoptGovernanceNarrowing.as_str(),
        "adopt_governance_narrowing"
    );
    assert_eq!(
        CertificationConsumerSurface::MigrationCenter.as_str(),
        "migration_center"
    );
    assert_eq!(
        CertificationConsumerSurface::HelpAbout.as_str(),
        "help_about"
    );
}

#[test]
fn ceilings_hold_for_each_state() {
    assert_eq!(
        DrillOutcome::Narrowed.label_ceiling(),
        EntryAssurance::Bounded
    );
    assert_eq!(
        DrillOutcome::Failed.label_ceiling(),
        EntryAssurance::Withheld
    );
    assert_eq!(
        DrillOutcome::NotRun.label_ceiling(),
        EntryAssurance::Withheld
    );
    assert_eq!(
        EvidenceFreshness::Aging.label_ceiling(),
        EntryAssurance::Bounded
    );
    assert_eq!(
        EvidenceFreshness::Expired.label_ceiling(),
        EntryAssurance::RetestPending
    );
}
