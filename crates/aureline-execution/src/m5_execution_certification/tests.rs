use super::*;

fn packet() -> M5ExecutionCertificationMatrix {
    current_m5_execution_certification_matrix().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_EXECUTION_CERTIFICATION_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_EXECUTION_CERTIFICATION_RECORD_KIND);
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
    assert_eq!(packet.certifications.len(), packet.lanes.len());
    for &lane in &packet.lanes {
        assert!(
            packet.certification(lane).is_some(),
            "missing row for lane {}",
            lane.as_str()
        );
    }
}

#[test]
fn every_lane_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_lanes_gate_consistent());
    for row in &packet.certifications {
        assert_eq!(
            row.published_qualification,
            row.effective_qualification(),
            "lane {} publishes beyond the gate",
            row.lane_id
        );
        assert_eq!(
            row.certification_decision,
            row.required_decision(),
            "lane {} decision diverges from the gate",
            row.lane_id
        );
        assert_eq!(
            row.downgrade_reasons,
            row.computed_downgrade_reasons(),
            "lane {} downgrade reasons diverge from the gate",
            row.lane_id
        );
    }
}

#[test]
fn every_lane_carries_its_own_evidence() {
    let packet = packet();
    for row in &packet.certifications {
        assert!(
            row.has_required_evidence(),
            "lane {} is missing required evidence refs",
            row.lane_id
        );
    }
}

#[test]
fn every_lane_binds_to_its_canonical_source_packet() {
    let packet = packet();
    for row in &packet.certifications {
        assert_eq!(
            row.packet_ref,
            row.lane.source_packet(),
            "lane {} certifies a packet other than its canonical source",
            row.lane_id
        );
    }
}

#[test]
fn narrowed_lanes_offer_a_downgrade_and_caveats() {
    let packet = packet();
    for row in &packet.certifications {
        if row.certification_decision.is_narrowed() {
            assert!(
                row.downgrade_path.is_offered(),
                "narrowed lane {} must offer a downgrade path",
                row.lane_id
            );
            assert!(
                !row.caveats.is_empty(),
                "narrowed lane {} must list a caveat",
                row.lane_id
            );
            assert!(
                !row.stale_or_missing_fields.is_empty(),
                "narrowed lane {} must name a stale or missing field",
                row.lane_id
            );
        }
    }
}

#[test]
fn export_projection_reflects_rows_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.lanes.len(), packet.certifications.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_lanes_gate_consistent,
        packet.all_lanes_gate_consistent()
    );
    assert_eq!(projection.certified_count, packet.certified_lanes().count());
    assert_eq!(projection.narrowed_count, packet.narrowed_lanes().count());
    assert_eq!(projection.withdrawn_count, packet.withdrawn_lanes().count());
    for (row, export) in packet.certifications.iter().zip(projection.lanes.iter()) {
        assert_eq!(export.packet_ref, row.packet_ref);
        assert_eq!(export.certified, row.is_certified());
        assert_eq!(export.downgraded, row.is_downgraded());
        assert_eq!(export.ops_adjacent, row.lane.is_ops_adjacent());
        assert_eq!(
            export.published_qualification,
            row.published_qualification.as_str()
        );
    }
}

#[test]
fn published_qualifications_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<QualificationLevel> = packet
        .certifications
        .iter()
        .map(|c| c.published_qualification)
        .collect();
    for level in QualificationLevel::ALL {
        assert!(
            present.contains(&level),
            "no lane publishes qualification {}",
            level.as_str()
        );
    }
}

#[test]
fn certification_decisions_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<CertificationDecision> = packet
        .certifications
        .iter()
        .map(|c| c.certification_decision)
        .collect();
    for decision in CertificationDecision::ALL {
        assert!(
            present.contains(&decision),
            "no lane exercises decision {}",
            decision.as_str()
        );
    }
}

#[test]
fn evidence_freshness_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<EvidenceFreshness> = packet
        .certifications
        .iter()
        .map(|c| c.evidence_freshness)
        .collect();
    for state in EvidenceFreshness::ALL {
        assert!(
            present.contains(&state),
            "no lane exercises freshness {}",
            state.as_str()
        );
    }
}

#[test]
fn profile_coverage_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ProfileCoverage> = packet
        .certifications
        .iter()
        .map(|c| c.profile_coverage)
        .collect();
    for state in ProfileCoverage::ALL {
        assert!(
            present.contains(&state),
            "no lane exercises coverage {}",
            state.as_str()
        );
    }
}

#[test]
fn drill_outcomes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<DrillOutcome> = packet
        .certifications
        .iter()
        .map(|c| c.drill_outcome)
        .collect();
    for outcome in DrillOutcome::ALL {
        assert!(
            present.contains(&outcome),
            "no lane exercises drill outcome {}",
            outcome.as_str()
        );
    }
}

#[test]
fn evidence_provenance_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<EvidenceProvenance> = packet
        .certifications
        .iter()
        .map(|c| c.evidence_provenance)
        .collect();
    for state in EvidenceProvenance::ALL {
        assert!(
            present.contains(&state),
            "no lane exercises provenance {}",
            state.as_str()
        );
    }
}

#[test]
fn downgrade_paths_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<DowngradePath> = packet
        .certifications
        .iter()
        .map(|c| c.downgrade_path)
        .collect();
    for path in DowngradePath::ALL {
        assert!(
            present.contains(&path),
            "no lane exercises downgrade path {}",
            path.as_str()
        );
    }
}

#[test]
fn downgrade_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<DowngradeReason> = packet
        .certifications
        .iter()
        .flat_map(|c| c.downgrade_reasons.iter().copied())
        .collect();
    for reason in DowngradeReason::ALL {
        assert!(
            present.contains(&reason),
            "no lane exercises downgrade reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn certified_lanes_are_clean() {
    let packet = packet();
    assert!(
        packet.certified_lanes().count() >= 2,
        "fixture needs at least two certified lanes to prove the gate is not a blanket downgrade"
    );
    for row in packet.certified_lanes() {
        assert_eq!(row.capability_floor(), QualificationLevel::Certified);
        assert!(row.downgrade_reasons.is_empty());
        assert!(row.caveats.is_empty());
        assert!(row.stale_or_missing_fields.is_empty());
        assert!(!row.downgrade_path.is_offered());
        assert!(!row.supported_profiles.is_empty());
        assert!(!row.is_downgraded());
    }
}

#[test]
fn ceilings_hold_for_each_state() {
    assert_eq!(
        EvidenceFreshness::Stale.qualification_ceiling(),
        QualificationLevel::LifecycleProvisional
    );
    assert_eq!(
        EvidenceFreshness::Expired.qualification_ceiling(),
        QualificationLevel::Withdrawn
    );
    assert_eq!(
        ProfileCoverage::Partial.qualification_ceiling(),
        QualificationLevel::ProfileQualified
    );
    assert_eq!(
        ProfileCoverage::Absent.qualification_ceiling(),
        QualificationLevel::Withdrawn
    );
    assert_eq!(
        DrillOutcome::PartiallyPassed.qualification_ceiling(),
        QualificationLevel::ProfileQualified
    );
    assert_eq!(
        DrillOutcome::Failed.qualification_ceiling(),
        QualificationLevel::Withdrawn
    );
    assert_eq!(
        EvidenceProvenance::Unverified.qualification_ceiling(),
        QualificationLevel::LifecycleProvisional
    );
    assert_eq!(
        EvidenceProvenance::Unverifiable.qualification_ceiling(),
        QualificationLevel::Withdrawn
    );
}

#[test]
fn stale_evidence_lane_is_downgraded_not_left_green() {
    // A stale managed-workspace lane is downgraded automatically rather than left green.
    let packet = packet();
    let row = packet
        .certification(CertifiedLane::ManagedWorkspaceLifecycle)
        .expect("managed-workspace-lifecycle row");
    assert!(row.evidence_freshness.is_stale_trigger());
    assert!(row.is_downgraded());
    assert!(row.lane.is_ops_adjacent());
    assert!(row
        .downgrade_reasons
        .contains(&DowngradeReason::StaleEvidence));
    assert_eq!(
        row.published_qualification,
        QualificationLevel::LifecycleProvisional
    );
}

#[test]
fn clean_lane_certifies() {
    // A fresh, fully-covered, drill-passing, verified lane certifies cleanly — the gate is
    // not a blanket downgrade.
    let packet = packet();
    let row = packet
        .certification(CertifiedLane::BuildIntelligence)
        .expect("build-intelligence row");
    assert_eq!(row.published_qualification, QualificationLevel::Certified);
    assert_eq!(row.certification_decision, CertificationDecision::Certify);
    assert!(row.downgrade_reasons.is_empty());
}

#[test]
fn dead_evidence_is_withdrawn() {
    // An expired, absent, drill-failing, unverifiable lane drops to withdrawn rather than
    // inheriting a broader claim.
    let packet = packet();
    let row = packet
        .certification(CertifiedLane::LiveResourceContext)
        .expect("live-resource-context row");
    assert_eq!(row.published_qualification, QualificationLevel::Withdrawn);
    assert_eq!(row.certification_decision, CertificationDecision::Withdraw);
    assert!(row.lane.is_ops_adjacent());
    assert!(row.downgrade_path.is_offered());
    assert!(row.supported_profiles.is_empty());
    assert_eq!(row.downgrade_reasons, DowngradeReason::ALL.to_vec());
}

#[test]
fn ops_adjacent_lanes_never_publish_above_their_evidence() {
    // An underqualified managed-workspace, cluster-context, mutation, or live-resource lane
    // narrows safely instead of inheriting a broader local or desktop claim.
    let packet = packet();
    for row in &packet.certifications {
        if row.lane.is_ops_adjacent() {
            assert_eq!(
                row.published_qualification,
                row.effective_qualification(),
                "ops-adjacent lane {} publishes beyond its evidence",
                row.lane_id
            );
        }
    }
}

#[test]
fn validate_flags_overstated_qualification() {
    let mut packet = packet();
    if let Some(row) = packet
        .certifications
        .iter_mut()
        .find(|c| c.effective_qualification() != QualificationLevel::Certified)
    {
        row.published_qualification = QualificationLevel::Certified;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ExecutionCertificationViolation::OverstatedQualification { .. }
        )));
    }
}

#[test]
fn validate_flags_decision_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .certifications
        .iter_mut()
        .find(|c| c.certification_decision != CertificationDecision::Withdraw)
    {
        row.certification_decision = CertificationDecision::Withdraw;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ExecutionCertificationViolation::DecisionMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_downgrade_reasons_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet.certifications.iter_mut().find(|c| {
        !c.downgrade_reasons
            .contains(&DowngradeReason::StaleEvidence)
    }) {
        row.downgrade_reasons.push(DowngradeReason::StaleEvidence);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ExecutionCertificationViolation::DowngradeReasonsMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_source_packet_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet.certifications.first_mut() {
        row.packet_ref = "artifacts/execution/m5/not-the-source.json".to_owned();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ExecutionCertificationViolation::SourcePacketMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_downgrade_path() {
    let mut packet = packet();
    if let Some(row) = packet
        .certifications
        .iter_mut()
        .find(|c| c.certification_decision.is_narrowed())
    {
        row.downgrade_path = DowngradePath::NoneNeeded;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ExecutionCertificationViolation::MissingDowngradePath { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_lane_row() {
    let mut packet = packet();
    let removed = packet.certifications.pop();
    assert!(removed.is_some());
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5ExecutionCertificationViolation::MissingLaneRow { .. })));
}

#[test]
fn validate_flags_unclaimed_lane_row() {
    let mut packet = packet();
    packet
        .lanes
        .retain(|l| *l != CertifiedLane::LiveResourceContext);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5ExecutionCertificationViolation::UnclaimedLaneRow { .. }
    )));
    assert!(violations.iter().any(|v| matches!(
        v,
        M5ExecutionCertificationViolation::ClosedVocabularyMismatch { field: "lanes" }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_lanes = packet.summary.total_lanes.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5ExecutionCertificationViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(
        CertifiedLane::BuildIntelligence.as_str(),
        "build_intelligence"
    );
    assert_eq!(
        CertifiedLane::LiveResourceContext.as_str(),
        "live_resource_context"
    );
    assert_eq!(QualificationLevel::Certified.as_str(), "certified");
    assert_eq!(QualificationLevel::Withdrawn.as_str(), "withdrawn");
    assert_eq!(EvidenceFreshness::Expired.as_str(), "expired");
    assert_eq!(ProfileCoverage::Minimal.as_str(), "minimal");
    assert_eq!(DrillOutcome::PartiallyPassed.as_str(), "partially_passed");
    assert_eq!(EvidenceProvenance::Unverifiable.as_str(), "unverifiable");
    assert_eq!(DowngradePath::RefreshEvidence.as_str(), "refresh_evidence");
    assert_eq!(DowngradePath::NoneNeeded.as_str(), "none");
    assert_eq!(
        DowngradeReason::DrillRegression.as_str(),
        "drill_regression"
    );
    assert_eq!(
        CertificationDecision::ProvisionLifecycle.as_str(),
        "provision_lifecycle"
    );
}

#[test]
fn qualification_rank_orders_low_to_high() {
    assert!(QualificationLevel::Withdrawn.rank() < QualificationLevel::LifecycleProvisional.rank());
    assert!(
        QualificationLevel::LifecycleProvisional.rank()
            < QualificationLevel::ProfileQualified.rank()
    );
    assert!(QualificationLevel::ProfileQualified.rank() < QualificationLevel::Certified.rank());
    assert_eq!(
        QualificationLevel::Certified.min(QualificationLevel::LifecycleProvisional),
        QualificationLevel::LifecycleProvisional
    );
}

#[test]
fn source_packets_point_at_real_b16_packets() {
    // Every certified lane binds to one of the five canonical B16 execution-truth packets,
    // so the certification aggregates landed packets rather than a parallel spreadsheet.
    for lane in CertifiedLane::ALL {
        assert!(
            lane.source_packet().starts_with("artifacts/execution/m5/"),
            "lane {} does not bind to an execution m5 packet",
            lane.as_str()
        );
    }
}
