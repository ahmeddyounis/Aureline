//! Inline tests for the M5 assurance-center lane.

use super::*;

fn packet() -> M5AssuranceCenter {
    seeded_m5_assurance_center()
}

#[test]
fn canonical_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_ASSURANCE_CENTER_PACKET_ID);
    assert_eq!(packet.record_kind, M5_ASSURANCE_CENTER_RECORD_KIND);
    assert_eq!(packet.claim_cards.len(), AssuranceClaimSubject::ALL.len());
    assert_eq!(packet.control_proof_rows.len(), ControlId::ALL.len());
    assert_eq!(packet.overviews.len(), ClaimedPosture::ALL.len());
    assert!(packet.conformance.all_hold());
    assert!(packet.vocabulary.matches_canonical());
}

#[test]
fn canonical_packet_proves_every_claim() {
    // Acceptance: with every control proven, every claim stands proven and every profile honored.
    let packet = packet();
    for card in &packet.claim_cards {
        assert!(
            card.is_governed(),
            "claim `{}` not governed",
            card.subject.as_str()
        );
        assert_eq!(card.active_state, AssuranceClaimState::Proven);
        assert!(card.fallback.is_none());
        assert!(card.gaps.is_empty());
        assert_eq!(card.effective_qualification, QualificationClass::Stable);
    }
    for overview in &packet.overviews {
        assert_eq!(overview.effective_posture, overview.profile);
        assert_eq!(overview.gate_decision, DescriptorGate::Governed);
    }
    assert_eq!(
        packet.summary.proven_claims,
        AssuranceClaimSubject::ALL.len() as u32
    );
    assert_eq!(
        packet.summary.honored_profiles,
        ClaimedPosture::ALL.len() as u32
    );
    assert!(!packet.blocks_stable_promotion());
}

#[test]
fn every_control_is_governed_exactly_once_and_bound_to_evidence() {
    // Acceptance: each control names one evidence class, owner, and proof ref.
    let packet = packet();
    for control in ControlId::ALL {
        let rows: Vec<&ControlProofRow> = packet
            .control_proof_rows
            .iter()
            .filter(|r| r.control == control)
            .collect();
        assert_eq!(
            rows.len(),
            1,
            "control `{}` not governed once",
            control.as_str()
        );
        let row = rows[0];
        assert_eq!(row.proof_ref, control.proof_ref());
        assert_eq!(row.evidence_class, control.evidence_class());
        assert_eq!(row.owner_role, control.owner_role());
        assert!(!row.proof_ref.trim().is_empty());
    }
}

#[test]
fn claim_card_derives_state_from_controls_and_never_overstates() {
    // Acceptance: cards bind to proof and do not silently strengthen copy.
    let packet = packet();
    assert!(packet.conformance.claim_state_derived_from_controls);
    assert!(packet.conformance.no_claim_overstates_controls);
    for card in &packet.claim_cards {
        let worst = card
            .required_controls
            .iter()
            .filter_map(|c| packet.control(*c))
            .map(|r| r.effective_gate)
            .fold(DescriptorGate::Governed, worse_gate);
        assert_eq!(
            card.active_gate,
            worst,
            "claim `{}` overstates",
            card.subject.as_str()
        );
    }
}

#[test]
fn waiver_narrows_the_key_ownership_claim_and_discloses_the_exception() {
    // Acceptance: exception / waiver rows disclose mitigation, expiry, compensating control, action.
    let packet = seeded_m5_assurance_center_waiver_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let key = packet.claim(AssuranceClaimSubject::KeyOwnership).unwrap();
    assert!(key.is_narrowed());
    assert_eq!(key.active_state, AssuranceClaimState::ExceptionPending);
    assert_eq!(key.effective_qualification, QualificationClass::Beta);

    // The nearest truthful fallback is present, never overstated, and names what is still proven.
    let fallback = key
        .fallback
        .as_ref()
        .expect("narrowed claim carries a fallback");
    assert_ne!(fallback.fallback_state, AssuranceClaimState::Proven);
    assert!(posture_rank(fallback.fallback_posture) <= posture_rank(key.claimed_posture));
    assert!(fallback
        .still_proven_controls
        .contains(&ControlId::CustomerManagedKeyCustody));
    assert!(fallback.unmet_controls.contains(&ControlId::LocalKeyEscrow));

    // The exception row is disclosed in full.
    assert_eq!(packet.exception_waiver_rows.len(), 1);
    let row = &packet.exception_waiver_rows[0];
    assert_eq!(row.control, ControlId::LocalKeyEscrow);
    assert_eq!(row.governance_state, GovernanceState::Waived);
    assert!(!row.mitigation.trim().is_empty());
    assert!(!row.expiry.trim().is_empty());
    assert_ne!(row.compensating_control, row.control);
    assert_eq!(row.responsible_party, ResponsibleParty::Customer);
    assert!(row
        .affected_claims
        .contains(&AssuranceClaimSubject::KeyOwnership));

    // Only the key-ownership claim narrows; sovereign (which shares a key control) stays proven.
    let sovereign = packet
        .claim(AssuranceClaimSubject::SovereignDeployment)
        .unwrap();
    assert!(sovereign.is_governed());
    assert!(!packet.blocks_stable_promotion());
}

#[test]
fn stale_evidence_narrows_exactly_the_claim_that_reads_it() {
    // Acceptance: a stale proof narrows claims deterministically.
    let packet = seeded_m5_assurance_center_stale_evidence_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.conformance.stale_evidence_narrows_deterministically);

    let stale_control = ControlId::RegulatedAuditTrail;
    assert_eq!(
        packet.control(stale_control).unwrap().evidence_freshness,
        FreshnessState::Stale
    );
    let regulated = packet
        .claim(AssuranceClaimSubject::RegulatedOperation)
        .unwrap();
    assert!(regulated.is_narrowed());
    assert_eq!(regulated.active_state, AssuranceClaimState::UnderReview);
    assert_eq!(regulated.effective_qualification, QualificationClass::Beta);
    assert!(regulated
        .gaps
        .iter()
        .any(|g| g.control == stale_control && g.gap_kind == ClaimGapKind::ControlNarrowed));

    // Claims that do not require the stale control stay proven.
    let telemetry = packet
        .claim(AssuranceClaimSubject::TelemetryControl)
        .unwrap();
    assert!(telemetry.is_governed());
    assert!(!packet.blocks_stable_promotion());
}

#[test]
fn missing_evidence_blocks_exactly_the_claim_that_reads_it() {
    // Acceptance: a missing / failing proof blocks Stable promotion deterministically.
    let packet = seeded_m5_assurance_center_missing_evidence_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.conformance.missing_evidence_blocks_stable_promotion);

    let missing_control = ControlId::SovereignControlPlane;
    assert_eq!(
        packet.control(missing_control).unwrap().evidence_freshness,
        FreshnessState::Missing
    );
    let sovereign = packet
        .claim(AssuranceClaimSubject::SovereignDeployment)
        .unwrap();
    assert!(sovereign.is_blocked());
    assert_eq!(sovereign.active_state, AssuranceClaimState::Unproven);
    assert_eq!(
        sovereign.effective_qualification,
        QualificationClass::Unavailable
    );
    assert!(sovereign
        .gaps
        .iter()
        .any(|g| g.control == missing_control && g.gap_kind == ClaimGapKind::ControlBlocked));

    // The fallback still names the controls that remain proven, never claiming the blocked posture.
    let fallback = sovereign
        .fallback
        .as_ref()
        .expect("blocked claim carries a fallback");
    assert!(posture_rank(fallback.fallback_posture) < posture_rank(ClaimedPosture::Sovereign));

    assert!(packet.blocks_stable_promotion());
    assert_eq!(packet.summary.blocked_claims, 1);
}

#[test]
fn overview_effective_posture_auto_narrows_and_never_overstates() {
    // Guardrail: do not imply a posture the active path does not satisfy.
    let packet = seeded_m5_assurance_center_missing_evidence_blocked();
    for overview in &packet.overviews {
        assert!(
            posture_rank(overview.effective_posture) <= posture_rank(overview.profile),
            "profile `{}` overstates its effective posture",
            overview.profile.as_str()
        );
    }
    // The sovereign profile can no longer be honored once its claim is blocked.
    let sovereign = packet.overview(ClaimedPosture::Sovereign).unwrap();
    assert_ne!(sovereign.effective_posture, ClaimedPosture::Sovereign);
    assert!(sovereign
        .evaluation_actions
        .contains(&EvaluationAction::ExportEvaluationPacket));
}

#[test]
fn evaluation_packet_reuses_the_ui_vocabulary() {
    // Acceptance: exported evaluation packets reuse the same claim-state and proof vocabulary.
    for packet in [
        packet(),
        seeded_m5_assurance_center_waiver_narrowed(),
        seeded_m5_assurance_center_stale_evidence_narrowed(),
        seeded_m5_assurance_center_missing_evidence_blocked(),
    ] {
        assert!(packet.conformance.evaluation_packet_reuses_ui_vocabulary);
        let export = &packet.evaluation_packet;
        assert!(export.vocabulary.matches_canonical());
        // Every claim entry's state token matches the card it came from.
        for entry in &export.claims {
            let card = packet.claim(entry.subject).unwrap();
            assert_eq!(entry.active_state, card.active_state);
            assert_eq!(
                entry.fallback_state,
                card.fallback.as_ref().map(|f| f.fallback_state)
            );
            assert_eq!(entry.evidence_refs, card.evidence_refs);
        }
        for entry in &export.controls {
            let row = packet.control(entry.control).unwrap();
            assert_eq!(entry.proof_state, row.proof_state);
            assert_eq!(entry.evidence_freshness, row.evidence_freshness);
        }
    }
}

#[test]
fn channels_produce_identical_output() {
    let packet = packet();
    let desktop = packet.render_for_channel(AssuranceCenterChannel::DesktopUi);
    let cli = packet.render_for_channel(AssuranceCenterChannel::CliHeadless);
    let offline = packet.render_for_channel(AssuranceCenterChannel::OfflineMirror);
    assert_eq!(desktop, cli);
    assert_eq!(cli, offline);
    assert_eq!(desktop, packet.export_safe_json());
}

#[test]
fn controlled_vocabulary_is_frozen() {
    let vocab = AssuranceCenterVocabulary::canonical();
    assert_eq!(vocab.claim_subjects.len(), AssuranceClaimSubject::ALL.len());
    assert_eq!(vocab.controls.len(), ControlId::ALL.len());
    for needle in [
        "local_first_continuity",
        "telemetry_control",
        "key_ownership",
        "data_residency",
        "regulated_operation",
        "air_gap_containment",
        "sovereign_deployment",
    ] {
        assert!(vocab.claim_subjects.contains(&needle.to_owned()));
    }
    // The claim-state grammar is exactly the governance matrix's assurance-claim family.
    for needle in [
        "proven",
        "attested",
        "under_review",
        "exception_pending",
        "unproven",
    ] {
        assert!(vocab.claim_states.contains(&needle.to_owned()));
    }
    for needle in ["managed", "self_hosted", "regulated", "sovereign"] {
        assert!(vocab.deployment_profiles.contains(&needle.to_owned()));
    }
}

#[test]
fn claims_csv_enumerates_claim_control_and_proof() {
    let csv = packet().render_claims_csv();
    let header = csv.lines().next().unwrap();
    assert!(header.starts_with("claim,claimed_posture,active_state,"));
    assert!(header.contains("proof_ref"));
    assert!(header.contains("gap_kind"));
    for subject in AssuranceClaimSubject::ALL {
        assert!(csv.contains(&format!(
            "{},{}",
            subject.as_str(),
            subject.claimed_posture().as_str()
        )));
    }
    assert!(csv.contains("artifacts/release-proof/m5-assurance-route-governance/"));
}

#[test]
fn overview_markdown_names_profiles_claims_and_controls() {
    let md = seeded_m5_assurance_center_waiver_narrowed().render_overview_markdown();
    assert!(md.contains("# M5 Assurance Center"));
    assert!(md.contains("Deployment-profile overviews"));
    assert!(md.contains("Claim cards"));
    assert!(md.contains("Control proof"));
    assert!(md.contains("Exceptions / waivers"));
    assert!(md.contains("key_ownership"));
}

#[test]
fn packet_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: M5AssuranceCenter = serde_json::from_str(&json).expect("packet deserializes");
    assert_eq!(parsed, packet);
    assert!(parsed.validate().is_empty());
}

#[test]
fn tampered_claim_state_is_rejected() {
    let mut packet = seeded_m5_assurance_center_stale_evidence_narrowed();
    let idx = packet
        .claim_cards
        .iter()
        .position(|c| c.is_narrowed())
        .expect("a narrowed claim exists");
    packet.claim_cards[idx].active_state = AssuranceClaimState::Proven;
    packet.claim_cards[idx].active_gate = DescriptorGate::Governed;
    let violations = packet.validate();
    assert!(
        violations.contains(&M5AssuranceCenterViolation::ClaimCardDrift)
            || violations.contains(&M5AssuranceCenterViolation::ClaimOverstatesControls),
        "{violations:?}"
    );
}

#[test]
fn tampered_control_freshness_is_rejected() {
    let mut packet = packet();
    packet.control_proof_rows[0].evidence_freshness = FreshnessState::Stale;
    let violations = packet.validate();
    assert!(
        violations.contains(&M5AssuranceCenterViolation::ControlGateDrift)
            || violations.contains(&M5AssuranceCenterViolation::SummaryDrift)
            || violations.contains(&M5AssuranceCenterViolation::ClaimCardDrift),
        "{violations:?}"
    );
}

#[test]
fn dropping_a_control_is_rejected() {
    let mut packet = packet();
    packet
        .control_proof_rows
        .retain(|r| r.control != ControlId::TelemetryEgressGate);
    let violations = packet.validate();
    assert!(violations.contains(&M5AssuranceCenterViolation::ControlNotGoverned));
}

#[test]
fn fallback_is_present_exactly_when_not_governed() {
    for packet in [
        packet(),
        seeded_m5_assurance_center_waiver_narrowed(),
        seeded_m5_assurance_center_stale_evidence_narrowed(),
        seeded_m5_assurance_center_missing_evidence_blocked(),
    ] {
        assert!(packet.conformance.fallback_present_when_not_governed);
        for card in &packet.claim_cards {
            assert_eq!(card.is_governed(), card.fallback.is_none());
        }
    }
}

#[test]
fn export_carries_no_raw_material() {
    for packet in [
        packet(),
        seeded_m5_assurance_center_waiver_narrowed(),
        seeded_m5_assurance_center_stale_evidence_narrowed(),
        seeded_m5_assurance_center_missing_evidence_blocked(),
    ] {
        assert!(packet.conformance.export_carries_no_raw_material);
        assert!(!packet
            .export_safe_json()
            .to_ascii_lowercase()
            .contains("bearer_token"));
    }
}
