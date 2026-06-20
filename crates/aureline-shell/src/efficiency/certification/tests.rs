use super::*;

fn seeded_evidence() -> (Vec<EfficiencyLabCase>, Vec<SessionPressureCase>) {
    (seeded_lab_cases(), seeded_session_pressure_cases())
}

#[test]
fn seeded_packet_proceeds_with_current_evidence() {
    let packet = seeded_proof_packet();

    assert_eq!(packet.record_kind, M5_EFFICIENCY_PROOF_PACKET_RECORD_KIND);
    assert!(packet.promotion_proceeds(), "{:?}", packet.promotion_gate);
    assert!(packet.no_claim_outruns_evidence());
    assert!(packet.promotion_gate.blocking_row_ids.is_empty());

    // Both axes are covered: claimed profiles and surface families.
    assert!(packet.summary_counts.profile_rows >= 4);
    assert!(packet.summary_counts.surface_family_rows >= 8);
    assert_eq!(
        packet.summary_counts.total_rows,
        packet.summary_counts.profile_rows + packet.summary_counts.surface_family_rows
    );

    // Every profile row certifies against current evidence.
    for row in &packet.rows {
        if row.subject_kind == CertifiedSubjectKind::LaptopOrDesktopProfile.as_str() {
            assert!(
                row.is_certified(),
                "profile {} not certified",
                row.subject_token
            );
            assert_eq!(row.evidence_freshness, EvidenceFreshness::Current.as_str());
            assert!(row.protected_paths_preserved);
            assert!(row.hidden_work_suppressed);
            assert!(row.optional_work_sheds_first);
        }
    }
}

#[test]
fn certified_claim_bearing_rows_publish_to_required_surfaces() {
    let packet = seeded_proof_packet();
    for row in &packet.rows {
        let ceiling = EfficiencyClaimLevel::from_token(&row.published_claim_ceiling).unwrap();
        if row.is_certified() && ceiling.is_claim_bearing() {
            assert_eq!(
                row.publication_targets,
                REQUIRED_PUBLICATION_SURFACES
                    .iter()
                    .map(|s| (*s).to_owned())
                    .collect::<Vec<_>>(),
                "row {} must publish to every required surface",
                row.row_id
            );
        } else {
            assert!(
                row.publication_targets.is_empty(),
                "row {} should not publish a claim",
                row.row_id
            );
        }
    }
}

#[test]
fn companion_adjacent_quarantines_without_evidence_and_does_not_block() {
    let packet = seeded_proof_packet();
    let row = packet
        .row("cert.surface.companion_adjacent")
        .expect("companion-adjacent row present");

    assert_eq!(
        row.certification_state,
        CertificationState::Quarantined.as_str()
    );
    assert_eq!(
        row.effective_posture,
        EfficiencyClaimLevel::UndeclaredBadge.as_str()
    );
    assert_eq!(row.evidence_freshness, EvidenceFreshness::Missing.as_str());
    assert!(row.fired_narrowing_reasons.contains(
        &CertificationNarrowingReason::MissingEfficiencyEvidence
            .as_str()
            .to_owned()
    ));
    // It asserts no claim, so it does not hold promotion.
    assert!(!row.blocks_promotion());
    assert!(row.publication_targets.is_empty());
}

#[test]
fn stale_evidence_narrows_claim_bearing_rows_and_holds_promotion() {
    let (lab, session) = seeded_evidence();
    let subjects = seeded_certification_subjects();
    // The same evidence, certified against a far-future as_of, is stale.
    let packet = certify_m5_efficiency(
        "test.stale",
        "2027-06-20",
        "2027-06-20T00:00:00Z",
        &subjects,
        &lab,
        &session,
    );

    assert!(!packet.promotion_proceeds());
    assert_eq!(packet.promotion_gate.decision, "hold");
    assert!(!packet.promotion_gate.blocking_row_ids.is_empty());

    let row = packet.row("cert.profile.battery_ultrabook").unwrap();
    assert!(row.blocks_promotion());
    assert_eq!(row.evidence_freshness, EvidenceFreshness::Stale.as_str());
    assert_eq!(
        row.effective_posture,
        EfficiencyClaimLevel::StateDeclared.as_str()
    );
    assert!(row.fired_narrowing_reasons.contains(
        &CertificationNarrowingReason::StaleEfficiencyEvidence
            .as_str()
            .to_owned()
    ));
}

#[test]
fn missing_evidence_blocks_a_claim_bearing_profile() {
    let (lab, session) = seeded_evidence();
    let subject = CertificationSubject {
        row_id: "test.profile.no_evidence".to_owned(),
        subject_kind: CertifiedSubjectKind::LaptopOrDesktopProfile,
        subject_token: "unproven_laptop".to_owned(),
        subject_label: "Unproven laptop".to_owned(),
        claimed_states: vec![EfficiencyState::EfficiencyAware],
        published_claim_ceiling: EfficiencyClaimLevel::CertifiedLowPower,
        required_drills: CertificationDrill::ALL.to_vec(),
        lab_profile: None,
        session_case_id: None,
        governance_row_ref: String::new(),
    };
    let packet = certify_m5_efficiency(
        "test.missing",
        "2026-06-20",
        "2026-06-20T00:00:00Z",
        &[subject],
        &lab,
        &session,
    );

    let row = packet.row("test.profile.no_evidence").unwrap();
    // A claim-bearing ceiling with no evidence narrows to the floor and blocks.
    assert!(row.blocks_promotion());
    assert_eq!(
        row.effective_posture,
        EfficiencyClaimLevel::UndeclaredBadge.as_str()
    );
    assert!(!packet.promotion_proceeds());
}

#[test]
fn partial_coverage_narrows_when_a_required_binding_is_absent() {
    let (lab, session) = seeded_evidence();
    // A subject with a session posture but no trace: the trace-bound drills are
    // partial, not missing, because some evidence exists.
    let subject = CertificationSubject {
        row_id: "test.partial".to_owned(),
        subject_kind: CertifiedSubjectKind::M5SurfaceFamily,
        subject_token: "partial_surface".to_owned(),
        subject_label: "Partially evidenced surface".to_owned(),
        claimed_states: vec![EfficiencyState::EfficiencyAware],
        published_claim_ceiling: EfficiencyClaimLevel::QualifiedLowPower,
        required_drills: CertificationDrill::ALL.to_vec(),
        lab_profile: None,
        session_case_id: Some("battery-saver".to_owned()),
        governance_row_ref: String::new(),
    };
    let packet = certify_m5_efficiency(
        "test.partial",
        "2026-06-20",
        "2026-06-20T00:00:00Z",
        &[subject],
        &lab,
        &session,
    );
    let row = packet.row("test.partial").unwrap();
    assert!(row.fired_narrowing_reasons.contains(
        &CertificationNarrowingReason::PartialEvidenceCoverage
            .as_str()
            .to_owned()
    ));
    assert_eq!(
        row.effective_posture,
        EfficiencyClaimLevel::StateDeclared.as_str()
    );
    assert!(row.blocks_promotion());
    // The session drill that *did* have evidence still passed.
    let session_drill = row
        .drill_results
        .iter()
        .find(|r| r.drill == CertificationDrill::SessionAwareShedding.as_str())
        .unwrap();
    assert!(session_drill.passed);
}

#[test]
fn surface_family_rows_align_with_governance_rows() {
    let packet = seeded_proof_packet();
    for row in &packet.rows {
        if row.subject_kind == CertifiedSubjectKind::M5SurfaceFamily.as_str() {
            assert!(
                !row.governance_row_ref.is_empty(),
                "surface row {} must cite a governance row",
                row.row_id
            );
            assert!(row.governance_row_ref.starts_with("eff."));
        }
    }
}

#[test]
fn declared_vocabulary_is_complete_and_round_trips() {
    let packet = seeded_proof_packet();
    assert_eq!(packet.claim_levels.len(), EfficiencyClaimLevel::ALL.len());
    assert_eq!(packet.drills.len(), CertificationDrill::ALL.len());
    assert_eq!(
        packet.evidence_kinds.len(),
        CertificationEvidenceKind::ALL.len()
    );
    assert_eq!(
        packet.narrowing_reasons.len(),
        CertificationNarrowingReason::ALL.len()
    );
    assert_eq!(
        packet.certification_states.len(),
        CertificationState::ALL.len()
    );

    for level in EfficiencyClaimLevel::ALL {
        assert_eq!(
            EfficiencyClaimLevel::from_token(level.as_str()),
            Some(level)
        );
    }
    for drill in CertificationDrill::ALL {
        assert_eq!(CertificationDrill::from_token(drill.as_str()), Some(drill));
    }
    for reason in CertificationNarrowingReason::ALL {
        assert_eq!(
            CertificationNarrowingReason::from_token(reason.as_str()),
            Some(reason)
        );
    }
}

#[test]
fn packet_serializes_round_trip() {
    let packet = seeded_proof_packet();
    let json = serde_json::to_string(&packet).expect("serialize");
    let parsed: M5EfficiencyProofPacket = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed, packet);
}
