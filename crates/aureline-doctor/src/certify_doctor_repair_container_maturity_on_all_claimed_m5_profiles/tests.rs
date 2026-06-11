use super::*;

fn packet() -> M5ProfileCertification {
    current_m5_profile_certification().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_PROFILE_CERTIFICATION_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_PROFILE_CERTIFICATION_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_claimed_profile_has_exactly_one_row() {
    let packet = packet();
    assert_eq!(packet.profiles.len(), packet.m5_profiles.len());
    for &profile in &packet.m5_profiles {
        assert!(
            packet.profile(profile).is_some(),
            "missing row for profile {}",
            profile.as_str()
        );
    }
}

#[test]
fn every_profile_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_profiles_gate_consistent());
    for row in &packet.profiles {
        assert_eq!(
            row.published_qualification,
            row.effective_qualification(),
            "profile {} publishes beyond the gate",
            row.profile_id
        );
        assert_eq!(
            row.certification_decision,
            row.required_decision(),
            "profile {} decision diverges from the gate",
            row.profile_id
        );
        assert_eq!(
            row.narrowing_reasons,
            row.computed_narrowing_reasons(),
            "profile {} narrowing reasons diverge from the gate",
            row.profile_id
        );
    }
}

#[test]
fn every_profile_carries_its_own_qualification_packet() {
    let packet = packet();
    for row in &packet.profiles {
        assert!(
            row.has_required_evidence(),
            "profile {} is missing required evidence refs",
            row.profile_id
        );
        assert!(
            !row.qualification_packet_ref.trim().is_empty(),
            "profile {} has no qualification packet",
            row.profile_id
        );
    }
}

#[test]
fn boundary_profiles_carry_a_boundary_proof_ref() {
    let packet = packet();
    for row in &packet.profiles {
        if row.boundary_proof.is_applicable() {
            assert!(
                row.boundary_ref_consistent(),
                "profile {} has a boundary but no boundary-proof ref",
                row.profile_id
            );
        }
    }
}

#[test]
fn export_projection_reflects_rows_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.profiles.len(), packet.profiles.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_profiles_gate_consistent,
        packet.all_profiles_gate_consistent()
    );
    assert_eq!(
        projection.promotable_count,
        packet.promotable_profiles().count()
    );
    assert_eq!(
        projection.narrowed_count,
        packet.narrowed_profiles().count()
    );
    assert_eq!(
        projection.failed_promotion_count,
        packet.failed_promotion_profiles().count()
    );
}

#[test]
fn published_qualifications_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<QualificationClass> = packet
        .profiles
        .iter()
        .map(|p| p.published_qualification)
        .collect();
    for qualification in QualificationClass::ALL {
        assert!(
            present.contains(&qualification),
            "no profile publishes qualification {}",
            qualification.as_str()
        );
    }
}

#[test]
fn certification_decisions_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<CertificationDecision> = packet
        .profiles
        .iter()
        .map(|p| p.certification_decision)
        .collect();
    for decision in CertificationDecision::ALL {
        assert!(
            present.contains(&decision),
            "no profile exercises decision {}",
            decision.as_str()
        );
    }
}

#[test]
fn freshness_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<CertificationFreshness> = packet
        .profiles
        .iter()
        .map(|p| p.evidence_freshness)
        .collect();
    for freshness in CertificationFreshness::ALL {
        assert!(
            present.contains(&freshness),
            "no profile exercises freshness {}",
            freshness.as_str()
        );
    }
}

#[test]
fn latency_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<DiagnosisLatencyState> = packet
        .profiles
        .iter()
        .map(|p| p.diagnosis_latency_state)
        .collect();
    for state in DiagnosisLatencyState::ALL {
        assert!(
            present.contains(&state),
            "no profile exercises latency state {}",
            state.as_str()
        );
    }
}

#[test]
fn engine_reachability_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<EngineReachability> = packet
        .profiles
        .iter()
        .map(|p| p.engine_reachability)
        .collect();
    for reachability in EngineReachability::ALL {
        assert!(
            present.contains(&reachability),
            "no profile exercises engine reachability {}",
            reachability.as_str()
        );
    }
}

#[test]
fn boundary_proof_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<BoundaryProof> =
        packet.profiles.iter().map(|p| p.boundary_proof).collect();
    for boundary in BoundaryProof::ALL {
        assert!(
            present.contains(&boundary),
            "no profile exercises boundary proof {}",
            boundary.as_str()
        );
    }
}

#[test]
fn narrowing_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<NarrowingReason> = packet
        .profiles
        .iter()
        .flat_map(|p| p.narrowing_reasons.iter().copied())
        .collect();
    for reason in NarrowingReason::ALL {
        assert!(
            present.contains(&reason),
            "no profile exercises narrowing reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn promotable_profiles_are_clean() {
    let packet = packet();
    assert!(
        packet.promotable_profiles().count() > 0,
        "fixture needs a certified profile"
    );
    for row in packet.promotable_profiles() {
        assert!(row.evidence_freshness.is_current());
        assert_eq!(row.diagnosis_latency_state, DiagnosisLatencyState::Green);
        assert_eq!(row.capability_floor(), QualificationClass::Certified);
        assert!(row.narrowing_reasons.is_empty());
        assert_eq!(row.published_qualification, QualificationClass::Certified);
        assert_eq!(row.certification_decision, CertificationDecision::Promote);
    }
}

#[test]
fn ceilings_hold_for_each_state() {
    assert_eq!(
        CertificationFreshness::Stale.qualification_ceiling(),
        QualificationClass::Provisional
    );
    assert_eq!(
        CertificationFreshness::Expired.qualification_ceiling(),
        QualificationClass::Underqualified
    );
    assert_eq!(
        DiagnosisLatencyState::Red.qualification_ceiling(),
        QualificationClass::Underqualified
    );
    assert_eq!(
        DiagnosisLatencyState::Amber.qualification_ceiling(),
        QualificationClass::Provisional
    );
    assert_eq!(
        EngineReachability::Blocked.qualification_ceiling(),
        QualificationClass::Underqualified
    );
    assert_eq!(
        EngineReachability::Degraded.qualification_ceiling(),
        QualificationClass::Provisional
    );
    assert_eq!(
        BoundaryProof::Unverified.qualification_ceiling(),
        QualificationClass::Underqualified
    );
    assert_eq!(
        BoundaryProof::Partial.qualification_ceiling(),
        QualificationClass::Provisional
    );
}

#[test]
fn validate_flags_overstated_published_qualification() {
    let mut packet = packet();
    if let Some(row) = packet
        .profiles
        .iter_mut()
        .find(|p| p.effective_qualification() != QualificationClass::Certified)
    {
        row.published_qualification = QualificationClass::Certified;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ProfileCertificationViolation::OverstatedPublishedQualification { .. }
        )));
    }
}

#[test]
fn validate_flags_decision_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .profiles
        .iter_mut()
        .find(|p| p.certification_decision != CertificationDecision::FailPromotion)
    {
        row.certification_decision = CertificationDecision::FailPromotion;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5ProfileCertificationViolation::DecisionMismatch { .. })));
    }
}

#[test]
fn validate_flags_narrowing_reasons_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet.profiles.iter_mut().find(|p| {
        !p.narrowing_reasons
            .contains(&NarrowingReason::EngineBlocked)
    }) {
        row.narrowing_reasons.push(NarrowingReason::EngineBlocked);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ProfileCertificationViolation::NarrowingReasonsMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_boundary_proof_ref() {
    let mut packet = packet();
    if let Some(row) = packet
        .profiles
        .iter_mut()
        .find(|p| p.boundary_proof.is_applicable())
    {
        row.container_boundary_ref = None;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ProfileCertificationViolation::MissingBoundaryProofRef { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_profile_row() {
    let mut packet = packet();
    let removed = packet.profiles.pop();
    assert!(removed.is_some());
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5ProfileCertificationViolation::MissingProfileRow { .. })));
}

#[test]
fn validate_flags_unclaimed_profile_row() {
    let mut packet = packet();
    packet.m5_profiles.retain(|p| *p != M5Profile::Incident);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5ProfileCertificationViolation::UnclaimedProfileRow { .. }
    )));
    assert!(violations.iter().any(|v| matches!(
        v,
        M5ProfileCertificationViolation::ClosedVocabularyMismatch {
            field: "m5_profiles"
        }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_profiles = packet.summary.total_profiles.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5ProfileCertificationViolation::SummaryMismatch));
}

#[test]
fn fail_promotion_withholds_the_claim() {
    let packet = packet();
    let failed: Vec<_> = packet.failed_promotion_profiles().collect();
    assert!(!failed.is_empty(), "fixture needs a withheld profile");
    for row in failed {
        assert_eq!(row.published_qualification, QualificationClass::Unsupported);
        assert_eq!(
            row.certification_decision,
            CertificationDecision::FailPromotion
        );
    }
}

#[test]
fn tokens_are_stable() {
    assert_eq!(M5Profile::Notebook.as_str(), "notebook");
    assert_eq!(M5Profile::RequestApi.as_str(), "request_api");
    assert_eq!(M5Profile::RemotePreview.as_str(), "remote_preview");
    assert_eq!(M5Profile::Incident.as_str(), "incident");
    assert_eq!(QualificationClass::Certified.as_str(), "certified");
    assert_eq!(DiagnosisLatencyState::Red.as_str(), "red");
    assert_eq!(EngineReachability::Blocked.as_str(), "blocked");
    assert_eq!(BoundaryProof::Unverified.as_str(), "unverified");
    assert_eq!(
        NarrowingReason::BoundaryProofMissing.as_str(),
        "boundary_proof_missing"
    );
    assert_eq!(
        CertificationDecision::FailPromotion.as_str(),
        "fail_promotion"
    );
}

#[test]
fn qualification_rank_orders_low_to_high() {
    assert!(QualificationClass::Unsupported.rank() < QualificationClass::Underqualified.rank());
    assert!(QualificationClass::Underqualified.rank() < QualificationClass::Provisional.rank());
    assert!(QualificationClass::Provisional.rank() < QualificationClass::Certified.rank());
    assert_eq!(
        QualificationClass::Certified.min(QualificationClass::Provisional),
        QualificationClass::Provisional
    );
    assert!(QualificationClass::Underqualified.is_underqualified_or_worse());
    assert!(QualificationClass::Unsupported.is_underqualified_or_worse());
    assert!(!QualificationClass::Provisional.is_underqualified_or_worse());
}
