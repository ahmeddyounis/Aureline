use super::{
    audit_structured_config_policy_entitlement_certification,
    seeded_structured_config_policy_entitlement_certification,
    seeded_structured_config_policy_entitlement_certification_scenario, AdminAuditabilityState,
    ArtifactLaneClass, CertificationScenario, CertificationState,
    STRUCTURED_CONFIG_POLICY_ENTITLEMENT_CERTIFICATION_PATH,
};
use crate::structured_config_policy_bundle_and_entitlement_matrix::{
    ArtifactFamilyKind, DeploymentProfileKind, QualificationLabel,
};

#[test]
fn canonical_packet_is_clean() {
    let packet = seeded_structured_config_policy_entitlement_certification();
    let defects = audit_structured_config_policy_entitlement_certification(&packet);
    assert!(defects.is_empty(), "unexpected defects: {defects:?}");
}

#[test]
fn preview_and_beta_rows_do_not_publish_as_certified() {
    let packet = seeded_structured_config_policy_entitlement_certification();
    for row in &packet.artifact_rows {
        match row.claim_ceiling {
            QualificationLabel::Preview => {
                assert_eq!(row.published_state, CertificationState::RetestPending);
            }
            QualificationLabel::Beta => {
                assert_eq!(row.published_state, CertificationState::Limited);
            }
            QualificationLabel::Stable => {}
        }
    }
}

#[test]
fn signed_bundle_rows_keep_signed_path_reviewability() {
    let packet = seeded_structured_config_policy_entitlement_certification();
    for row in &packet.artifact_rows {
        if row.lane_class == ArtifactLaneClass::SignedBundleReview {
            assert!(row.signed_path_reviewable, "{:?}", row.family);
            assert_ne!(
                row.admin_auditability,
                AdminAuditabilityState::Incomplete,
                "{:?}",
                row.family
            );
        }
    }
}

#[test]
fn every_profile_carries_all_required_drills() {
    let packet = seeded_structured_config_policy_entitlement_certification();
    for row in &packet.profile_rows {
        assert_eq!(row.drills.len(), 6, "{:?}", row.profile);
    }
}

#[test]
fn stale_policy_scenario_narrows_managed_profiles_to_offline_only() {
    let packet = seeded_structured_config_policy_entitlement_certification_scenario(
        CertificationScenario::StalePolicy,
    );
    let managed = packet
        .profile_rows
        .iter()
        .find(|row| row.profile == DeploymentProfileKind::Managed)
        .expect("managed profile row");
    assert_eq!(managed.published_state, CertificationState::OfflineOnly);
    assert!(!managed.narrowing_reasons.is_empty());
}

#[test]
fn signer_rotation_scenario_blocks_trust_root_rows() {
    let packet = seeded_structured_config_policy_entitlement_certification_scenario(
        CertificationScenario::SignerRotation,
    );
    let trust_root = packet
        .artifact_rows
        .iter()
        .find(|row| row.family == ArtifactFamilyKind::TrustRootSignerUpdateArtifact)
        .expect("trust-root row");
    assert_eq!(
        trust_root.published_state,
        CertificationState::RetestPending
    );
    assert_eq!(
        trust_root.admin_auditability,
        AdminAuditabilityState::Incomplete
    );
}

#[test]
fn every_publication_surface_quotes_the_checked_in_packet_path() {
    let packet = seeded_structured_config_policy_entitlement_certification();
    for row in &packet.surface_rows {
        assert_eq!(
            row.packet_ref,
            STRUCTURED_CONFIG_POLICY_ENTITLEMENT_CERTIFICATION_PATH
        );
        assert!(row.shows_published_state);
        assert!(row.shows_evidence_age);
        assert!(row.shows_local_safe_floor);
        assert!(row.shows_supported_profiles);
        assert!(row.shows_narrowing_reasons);
    }
}
