use super::*;

const PACKET_ID: &str = "m5-companion-certification:stable:0001";
const PACKET_LABEL: &str = "M5 Companion Lane Certification On Every Marketed Profile";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn proof_freshness() -> M5CompanionCertificationProofFreshness {
    M5CompanionCertificationProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: MINTED_AT.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> M5CompanionCertificationPacket {
    canonical_m5_companion_certification(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        proof_freshness(),
    )
    .expect("canonical certification builds")
}

fn lane(
    packet: &M5CompanionCertificationPacket,
    lane: M5CompanionMatrixLane,
) -> &CompanionLaneCertification {
    packet
        .lane_certifications
        .iter()
        .find(|cert| cert.lane == lane)
        .expect("lane present")
}

fn fresh_observation(lane: M5CompanionMatrixLane) -> CompanionCertificationObservation {
    CompanionCertificationObservation {
        lane,
        evidence_valid: true,
        proof_fresh: true,
        provider_or_admin_available: true,
        residency_and_encryption_verified: true,
        upstream_matrix_narrowed: false,
    }
}

#[test]
fn canonical_certification_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn canonical_covers_every_lane_and_profile() {
    let packet = packet();
    assert_eq!(
        packet.lane_certifications.len(),
        M5CompanionMatrixLane::ALL.len()
    );
    for cert in &packet.lane_certifications {
        assert_eq!(cert.domain, cert.lane.domain());
        assert_eq!(cert.profile_coverage.len(), MarketedM5Profile::ALL.len());
        for profile in MarketedM5Profile::ALL {
            assert!(cert
                .profile_coverage
                .iter()
                .any(|row| row.profile == profile));
        }
    }
}

#[test]
fn claim_never_exceeds_matrix_baseline() {
    let packet = packet();
    let matrix = current_stable_m5_companion_matrix_export().expect("matrix export");
    for cert in &packet.lane_certifications {
        let matrix_qual = matrix
            .lane_rows
            .iter()
            .find(|row| row.lane == cert.lane)
            .expect("matrix lane")
            .qualification;
        assert_eq!(cert.matrix_baseline_qualification, matrix_qual);
        assert!(qualification_rank(cert.claimed_qualification) <= qualification_rank(matrix_qual));
    }
}

#[test]
fn managed_lanes_only_marketed_on_managed_plane() {
    let packet = packet();
    for managed in [
        M5CompanionMatrixLane::ManagedSync,
        M5CompanionMatrixLane::ResidencyEncryption,
    ] {
        for row in &lane(&packet, managed).profile_coverage {
            assert_eq!(
                row.certified_on_profile,
                row.profile.provides_managed_plane(),
                "managed lane {:?} on {:?}",
                managed,
                row.profile
            );
        }
    }
}

#[test]
fn companion_lanes_not_marketed_air_gapped() {
    let packet = packet();
    for companion in [
        M5CompanionMatrixLane::CompanionNotification,
        M5CompanionMatrixLane::CompanionReview,
        M5CompanionMatrixLane::CompanionSessionFollow,
        M5CompanionMatrixLane::CompanionLightEdit,
    ] {
        let row = lane(&packet, companion)
            .profile_coverage
            .iter()
            .find(|row| row.profile == MarketedM5Profile::AirGappedOffline)
            .expect("air-gapped row");
        assert!(!row.certified_on_profile);
    }
}

#[test]
fn local_first_lanes_certified_even_air_gapped() {
    let packet = packet();
    for local_first in [
        M5CompanionMatrixLane::IncidentWorkspace,
        M5CompanionMatrixLane::OffboardingContinuity,
    ] {
        let row = lane(&packet, local_first)
            .profile_coverage
            .iter()
            .find(|row| row.profile == MarketedM5Profile::AirGappedOffline)
            .expect("air-gapped row");
        assert!(row.certified_on_profile);
        assert!(!row.requires_provider_or_admin_continuity);
    }
}

#[test]
fn every_row_preserves_local_core_continuity() {
    let packet = packet();
    assert!(packet
        .lane_certifications
        .iter()
        .flat_map(|cert| cert.profile_coverage.iter())
        .all(|row| row.local_core_continuity_preserved));
}

#[test]
fn claim_exceeding_matrix_baseline_fails() {
    let mut packet = packet();
    // Force a Beta lane to claim Stable above its matrix baseline.
    let cert = packet
        .lane_certifications
        .iter_mut()
        .find(|cert| cert.lane == M5CompanionMatrixLane::ManagedSync)
        .expect("managed sync lane");
    cert.claimed_qualification = M5CompanionQualificationClass::Stable;
    assert!(packet
        .validate()
        .contains(&M5CompanionCertificationViolation::ClaimExceedsMatrixBaseline));
}

#[test]
fn matrix_baseline_mismatch_fails() {
    let mut packet = packet();
    packet.lane_certifications[0].matrix_baseline_qualification =
        M5CompanionQualificationClass::Experimental;
    assert!(packet
        .validate()
        .contains(&M5CompanionCertificationViolation::MatrixBaselineMismatch));
}

#[test]
fn managed_lane_certified_off_plane_fails() {
    let mut packet = packet();
    let cert = packet
        .lane_certifications
        .iter_mut()
        .find(|cert| cert.lane == M5CompanionMatrixLane::ManagedSync)
        .expect("managed sync lane");
    let row = cert
        .profile_coverage
        .iter_mut()
        .find(|row| row.profile == MarketedM5Profile::AirGappedOffline)
        .expect("air-gapped row");
    row.certified_on_profile = true;
    row.profile_qualification = M5CompanionQualificationClass::Beta;
    assert!(packet
        .validate()
        .contains(&M5CompanionCertificationViolation::LaneNotMarketableOnProfile));
}

#[test]
fn profile_qualification_exceeding_claim_fails() {
    let mut packet = packet();
    let cert = packet
        .lane_certifications
        .iter_mut()
        .find(|cert| cert.lane == M5CompanionMatrixLane::CompanionSessionFollow)
        .expect("session follow lane");
    cert.profile_coverage[0].profile_qualification = M5CompanionQualificationClass::Stable;
    assert!(packet
        .validate()
        .contains(&M5CompanionCertificationViolation::ProfileQualificationExceedsClaim));
}

#[test]
fn inconsistent_claim_flag_fails() {
    let mut packet = packet();
    let row = &mut packet.lane_certifications[0].profile_coverage[0];
    row.certified_on_profile = !row.certified_on_profile;
    assert!(packet
        .validate()
        .contains(&M5CompanionCertificationViolation::ProfileClaimFlagInconsistent));
}

#[test]
fn stranded_local_work_fails() {
    let mut packet = packet();
    packet.lane_certifications[0].profile_coverage[0].local_core_continuity_preserved = false;
    assert!(packet
        .validate()
        .contains(&M5CompanionCertificationViolation::LocalCoreContinuityStranded));
}

#[test]
fn unlabeled_stale_row_fails() {
    let mut packet = packet();
    let row = &mut packet.lane_certifications[0].profile_coverage[0];
    row.freshness = CertificationFreshness::Stale;
    row.freshness_label_shown = false;
    assert!(packet
        .validate()
        .contains(&M5CompanionCertificationViolation::FreshnessStateNotLabeled));
}

#[test]
fn inconsistent_continuity_flag_fails() {
    let mut packet = packet();
    let cert = packet
        .lane_certifications
        .iter_mut()
        .find(|cert| cert.lane == M5CompanionMatrixLane::IncidentWorkspace)
        .expect("incident lane");
    // Incident workspace is local-first, so a certified row must not require continuity.
    let row = cert
        .profile_coverage
        .iter_mut()
        .find(|row| row.certified_on_profile)
        .expect("certified row");
    row.requires_provider_or_admin_continuity = true;
    assert!(packet
        .validate()
        .contains(&M5CompanionCertificationViolation::ContinuityFlagInconsistent));
}

#[test]
fn missing_locality_disclosure_fails() {
    let mut packet = packet();
    let cert = packet
        .lane_certifications
        .iter_mut()
        .find(|cert| cert.lane == M5CompanionMatrixLane::ManagedSync)
        .expect("managed sync lane");
    let row = cert
        .profile_coverage
        .iter_mut()
        .find(|row| row.certified_on_profile)
        .expect("certified row");
    row.locality_disclosure
        .requires_provider_or_admin_continuity
        .clear();
    assert!(packet
        .validate()
        .contains(&M5CompanionCertificationViolation::LocalityDisclosureIncomplete));
}

#[test]
fn missing_lane_fails() {
    let mut packet = packet();
    packet
        .lane_certifications
        .retain(|cert| cert.lane != M5CompanionMatrixLane::OffboardingContinuity);
    assert!(packet
        .validate()
        .contains(&M5CompanionCertificationViolation::RequiredLaneMissing));
}

#[test]
fn duplicate_lane_fails() {
    let mut packet = packet();
    let dup = packet.lane_certifications[0].clone();
    packet.lane_certifications.push(dup);
    assert!(packet
        .validate()
        .contains(&M5CompanionCertificationViolation::DuplicateLane));
}

#[test]
fn missing_downgrade_proof_stale_rule_fails() {
    let mut packet = packet();
    packet.lane_certifications[0]
        .downgrade_rules
        .retain(|rule| rule.trigger != M5CompanionDowngradeTrigger::ProofStale);
    assert!(packet
        .validate()
        .contains(&M5CompanionCertificationViolation::DowngradeRuleMissingProofStale));
}

#[test]
fn non_narrowing_downgrade_rule_fails() {
    let mut packet = packet();
    // Point a rule at the matrix baseline itself rather than below it.
    let cert = &mut packet.lane_certifications[0];
    let baseline = cert.matrix_baseline_qualification;
    cert.downgrade_rules[0].narrowed_to = baseline;
    assert!(packet
        .validate()
        .contains(&M5CompanionCertificationViolation::DowngradeRuleNotNarrowing));
}

#[test]
fn claimed_lane_missing_evidence_fails() {
    let mut packet = packet();
    packet.lane_certifications[0].evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CompanionCertificationViolation::ClaimedLaneMissingEvidence));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CompanionCertificationViolation::MissingSourceContracts));
}

#[test]
fn security_review_incomplete_fails() {
    let mut packet = packet();
    packet.security_review.no_lane_greener_than_matrix = false;
    assert!(packet
        .validate()
        .contains(&M5CompanionCertificationViolation::SecurityReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.unqualified_rows_labeled = false;
    assert!(packet
        .validate()
        .contains(&M5CompanionCertificationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5CompanionCertificationViolation::ProofFreshnessIncomplete));
}

#[test]
fn narrowed_qualification_lookup_uses_rules() {
    let packet = packet();
    let cert = lane(&packet, M5CompanionMatrixLane::ManagedSync);
    let narrowed = cert.narrowed_qualification(M5CompanionDowngradeTrigger::ProofStale);
    assert!(qualification_rank(narrowed) < qualification_rank(cert.claimed_qualification));
    // A trigger with no rule leaves the claim unchanged.
    assert_eq!(
        cert.narrowed_qualification(M5CompanionDowngradeTrigger::TrustNarrowing),
        cert.claimed_qualification
    );
}

#[test]
fn proof_stale_automation_narrows_and_labels_every_row() {
    let mut packet = packet();
    let observations = M5CompanionMatrixLane::ALL
        .into_iter()
        .map(|l| CompanionCertificationObservation {
            proof_fresh: false,
            ..fresh_observation(l)
        })
        .collect::<Vec<_>>();
    packet.apply_downgrade_automation(&observations);
    assert!(packet
        .degraded_labels
        .contains(&CompanionCertificationDegradedReason::ProofStale));
    assert!(packet
        .degraded_labels
        .contains(&CompanionCertificationDegradedReason::FreshnessDowngradedToStale));
    // Every row is now stale and labeled.
    assert!(packet
        .lane_certifications
        .iter()
        .flat_map(|cert| cert.profile_coverage.iter())
        .all(|row| row.freshness == CertificationFreshness::Stale && row.freshness_label_shown));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn evidence_invalid_automation_holds_and_withholds_lane() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[CompanionCertificationObservation {
        evidence_valid: false,
        ..fresh_observation(M5CompanionMatrixLane::ManagedSync)
    }]);
    assert!(packet
        .degraded_labels
        .contains(&CompanionCertificationDegradedReason::EvidenceInvalid));
    let cert = lane(&packet, M5CompanionMatrixLane::ManagedSync);
    assert_eq!(
        cert.claimed_qualification,
        M5CompanionQualificationClass::Held
    );
    assert!(cert.profile_coverage.iter().all(|row| {
        !row.certified_on_profile && row.rollout_stage == M5CompanionRolloutStage::Withheld
    }));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn residency_unverified_automation_narrows_only_managed_profiles() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[CompanionCertificationObservation {
        residency_and_encryption_verified: false,
        ..fresh_observation(M5CompanionMatrixLane::ResidencyEncryption)
    }]);
    assert!(packet
        .degraded_labels
        .contains(&CompanionCertificationDegradedReason::ResidencyOrEncryptionUnverified));
    let cert = lane(&packet, M5CompanionMatrixLane::ResidencyEncryption);
    // The two managed profiles narrowed one step from Beta to Preview.
    for row in &cert.profile_coverage {
        if row.profile.provides_managed_plane() {
            assert_eq!(
                row.profile_qualification,
                M5CompanionQualificationClass::Preview
            );
        } else {
            assert!(!row.certified_on_profile);
        }
    }
    // Other lanes are untouched.
    let incident = lane(&packet, M5CompanionMatrixLane::IncidentWorkspace);
    assert_eq!(
        incident.claimed_qualification,
        M5CompanionQualificationClass::Stable
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn export_contains_no_forbidden_material() {
    let packet = packet();
    assert!(!packet
        .validate()
        .contains(&M5CompanionCertificationViolation::RawBoundaryMaterialInExport));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_companion_certification_export()
        .expect("checked companion certification export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked = current_stable_m5_companion_certification_export()
        .expect("checked companion certification export validates");
    assert_eq!(
        checked,
        packet(),
        "checked export drifted from canonical builder"
    );
}
