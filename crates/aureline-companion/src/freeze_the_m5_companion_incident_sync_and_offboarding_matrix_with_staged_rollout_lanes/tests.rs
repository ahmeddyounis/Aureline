use super::*;

const PACKET_ID: &str = "m5-companion-matrix:stable:0001";
const PACKET_LABEL: &str = "M5 Companion, Incident, Sync, Residency, and Offboarding Matrix";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn proof_freshness() -> M5CompanionMatrixProofFreshness {
    M5CompanionMatrixProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: MINTED_AT.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> M5CompanionMatrixPacket {
    canonical_m5_companion_matrix(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        proof_freshness(),
    )
}

#[test]
fn canonical_matrix_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn canonical_matrix_covers_every_lane_and_domain() {
    let packet = packet();
    assert_eq!(packet.lane_rows.len(), M5CompanionMatrixLane::ALL.len());
    for lane in M5CompanionMatrixLane::ALL {
        let row = packet
            .lane_rows
            .iter()
            .find(|row| row.lane == lane)
            .expect("lane present");
        assert_eq!(row.domain, lane.domain());
    }
    let domains: BTreeSet<_> = packet.lane_rows.iter().map(|row| row.domain).collect();
    assert_eq!(domains.len(), 5, "all five domains represented");
}

#[test]
fn missing_lane_fails_validation() {
    let mut packet = packet();
    packet
        .lane_rows
        .retain(|row| row.lane != M5CompanionMatrixLane::ManagedSync);
    assert!(packet
        .validate()
        .contains(&M5CompanionMatrixViolation::RequiredLaneMissing));
}

#[test]
fn lane_domain_mismatch_fails() {
    let mut packet = packet();
    packet.lane_rows[0].domain = M5CompanionMatrixDomain::Offboarding;
    assert!(packet
        .validate()
        .contains(&M5CompanionMatrixViolation::LaneDomainMismatch));
}

#[test]
fn incomplete_locality_disclosure_fails() {
    let mut packet = packet();
    packet.lane_rows[4]
        .locality_disclosure
        .requires_provider_or_admin_continuity
        .clear();
    assert!(packet
        .validate()
        .contains(&M5CompanionMatrixViolation::LocalityDisclosureIncomplete));
}

#[test]
fn stable_lane_missing_evidence_fails() {
    let mut packet = packet();
    let idx = packet
        .lane_rows
        .iter()
        .position(|row| row.qualification == M5CompanionQualificationClass::Stable)
        .expect("a stable lane exists");
    packet.lane_rows[idx].required_evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CompanionMatrixViolation::StableLaneMissingEvidence));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.lane_rows[1].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5CompanionMatrixViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.lane_rows[2].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5CompanionMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CompanionMatrixViolation::MissingSourceContracts));
}

#[test]
fn security_review_incomplete_fails() {
    let mut packet = packet();
    packet.security_review.offboarding_never_strands_local_work = false;
    assert!(packet
        .validate()
        .contains(&M5CompanionMatrixViolation::SecurityReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .preview_labs_label_for_unqualified_lanes = false;
    assert!(packet
        .validate()
        .contains(&M5CompanionMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5CompanionMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn downgrade_automation_holds_lane_on_invalid_evidence() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[M5CompanionMatrixLaneObservation {
        lane: M5CompanionMatrixLane::IncidentWorkspace,
        evidence_valid: false,
        proof_fresh: true,
        provider_or_admin_available: true,
        residency_and_encryption_verified: true,
        upstream_narrowed: false,
    }]);
    let row = packet
        .lane_rows
        .iter()
        .find(|row| row.lane == M5CompanionMatrixLane::IncidentWorkspace)
        .expect("incident lane present");
    assert_eq!(row.qualification, M5CompanionQualificationClass::Held);
    assert_eq!(row.rollout_stage, M5CompanionRolloutStage::Withheld);
    // The packet still validates: a held/withheld lane is narrowed, not removed.
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn downgrade_automation_narrows_lane_on_unverified_residency() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[M5CompanionMatrixLaneObservation {
        lane: M5CompanionMatrixLane::ResidencyEncryption,
        evidence_valid: true,
        proof_fresh: true,
        provider_or_admin_available: true,
        residency_and_encryption_verified: false,
        upstream_narrowed: false,
    }]);
    let row = packet
        .lane_rows
        .iter()
        .find(|row| row.lane == M5CompanionMatrixLane::ResidencyEncryption)
        .expect("residency lane present");
    // Beta narrows to Preview; staged rollout narrows to early access.
    assert_eq!(row.qualification, M5CompanionQualificationClass::Preview);
    assert_eq!(row.rollout_stage, M5CompanionRolloutStage::EarlyAccess);
}

#[test]
fn downgrade_automation_narrows_stable_lane_on_stale_proof() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[M5CompanionMatrixLaneObservation {
        lane: M5CompanionMatrixLane::OffboardingContinuity,
        evidence_valid: true,
        proof_fresh: false,
        provider_or_admin_available: true,
        residency_and_encryption_verified: true,
        upstream_narrowed: false,
    }]);
    let row = packet
        .lane_rows
        .iter()
        .find(|row| row.lane == M5CompanionMatrixLane::OffboardingContinuity)
        .expect("offboarding lane present");
    assert_eq!(row.qualification, M5CompanionQualificationClass::Beta);
    assert_eq!(row.rollout_stage, M5CompanionRolloutStage::StagedRollout);
}

#[test]
fn publishable_lanes_excludes_withheld() {
    let mut packet = packet();
    let total = packet.lane_rows.len();
    assert_eq!(packet.publishable_lanes().count(), total);
    packet.apply_downgrade_automation(&[M5CompanionMatrixLaneObservation {
        lane: M5CompanionMatrixLane::CompanionLightEdit,
        evidence_valid: false,
        proof_fresh: true,
        provider_or_admin_available: true,
        residency_and_encryption_verified: true,
        upstream_narrowed: false,
    }]);
    assert_eq!(packet.publishable_lanes().count(), total - 1);
}

#[test]
fn export_contains_no_forbidden_material() {
    let packet = packet();
    assert!(!packet
        .validate()
        .contains(&M5CompanionMatrixViolation::RawBoundaryMaterialInExport));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_companion_matrix_export()
        .expect("checked M5 companion matrix export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked = current_stable_m5_companion_matrix_export()
        .expect("checked M5 companion matrix export validates");
    assert_eq!(
        checked,
        packet(),
        "checked export drifted from canonical builder"
    );
}
