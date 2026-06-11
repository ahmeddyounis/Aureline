use super::*;

const PACKET_ID: &str = "managed-sync-maturity-surface:stable:0001";
const PACKET_LABEL: &str =
    "Managed Sync Maturity: Snapshot Classes, Conflict Review, Device Registry, and End-to-End Encrypted Storage";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn proof_freshness() -> ManagedSyncProofFreshness {
    ManagedSyncProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: MINTED_AT.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> ManagedSyncMaturitySurfacePacket {
    canonical_managed_sync_maturity_surface(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        proof_freshness(),
    )
}

#[test]
fn canonical_surface_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn canonical_surface_covers_every_section_with_correct_lane() {
    let packet = packet();
    assert_eq!(
        packet.section_qualifications.len(),
        ManagedSyncSection::ALL.len()
    );
    for section in ManagedSyncSection::ALL {
        let row = packet
            .section_qualifications
            .iter()
            .find(|row| row.section == section)
            .expect("section present");
        assert_eq!(row.matrix_lane_ref, section.matrix_lane().as_str());
        assert_eq!(row.read_write_scope, section.bounded_scope());
    }
    // The first three sections inherit the managed_sync lane; encrypted storage
    // inherits the residency_encryption lane.
    let lane_of = |section: ManagedSyncSection| {
        packet
            .section_qualifications
            .iter()
            .find(|row| row.section == section)
            .map(|row| row.matrix_lane_ref.as_str())
            .unwrap()
    };
    assert_eq!(
        lane_of(ManagedSyncSection::SnapshotClass),
        M5CompanionMatrixLane::ManagedSync.as_str()
    );
    assert_eq!(
        lane_of(ManagedSyncSection::ConflictReview),
        M5CompanionMatrixLane::ManagedSync.as_str()
    );
    assert_eq!(
        lane_of(ManagedSyncSection::DeviceRegistry),
        M5CompanionMatrixLane::ManagedSync.as_str()
    );
    assert_eq!(
        lane_of(ManagedSyncSection::EncryptedStorage),
        M5CompanionMatrixLane::ResidencyEncryption.as_str()
    );
}

#[test]
fn canonical_surface_handoffs_are_exact() {
    let packet = packet();
    assert!(packet.all_handoffs_exact());
    assert!(!packet.snapshot_classes.is_empty());
    assert!(!packet.conflicts.is_empty());
    assert!(!packet.devices.is_empty());
    assert!(!packet.encrypted_storage.is_empty());
}

#[test]
fn every_section_is_read_only() {
    let packet = packet();
    assert!(packet
        .snapshot_classes
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
    assert!(packet
        .conflicts
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
    assert!(packet
        .devices
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
    assert!(packet
        .encrypted_storage
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
}

#[test]
fn local_core_is_authoritative_for_every_snapshot() {
    let packet = packet();
    assert!(packet
        .snapshot_classes
        .iter()
        .all(|item| item.local_authoritative));
}

#[test]
fn every_conflict_requires_user_review() {
    let packet = packet();
    assert!(packet.no_silent_server_authority());
    for item in &packet.conflicts {
        assert!(item.requires_user_review);
    }
    // The corpus carries a pending conflict awaiting review and a resolved one.
    assert!(packet
        .conflicts
        .iter()
        .any(|item| item.resolution_state == ConflictResolutionState::PendingReview));
    assert!(packet
        .conflicts
        .iter()
        .any(|item| item.resolution_state.is_resolved()));
}

#[test]
fn encryption_claims_are_provable_or_labeled() {
    let packet = packet();
    assert!(packet.encryption_claims_honestly_qualified());
    // The corpus carries a verified end-to-end claim and an honestly-unverified claim.
    assert!(packet
        .encrypted_storage
        .iter()
        .any(
            |item| item.encryption_posture == EncryptionPosture::EndToEndEncryptedVerified
                && item.claim_verified
        ));
    let unverified = packet
        .encrypted_storage
        .iter()
        .find(|item| item.encryption_posture == EncryptionPosture::ClaimedUnverified)
        .expect("unverified claim present");
    assert!(unverified.proof_label_shown);
    assert!(!unverified.claim_verified);
}

#[test]
fn customer_managed_key_is_represented() {
    let packet = packet();
    assert!(packet
        .encrypted_storage
        .iter()
        .any(|item| item.key_custody == KeyCustody::CustomerManagedKey));
}

#[test]
fn device_registry_records_this_device_and_pending_approval() {
    let packet = packet();
    assert!(packet.devices.iter().any(|item| item.this_device));
    assert!(packet
        .devices
        .iter()
        .any(|item| item.trust_state == SyncDeviceTrustState::PendingApproval));
}

#[test]
fn stale_items_are_honestly_labeled() {
    let packet = packet();
    assert!(packet.stale_state_honestly_labeled());
}

#[test]
fn missing_section_fails_validation() {
    let mut packet = packet();
    packet.section_qualifications.pop();
    assert!(packet
        .validate()
        .contains(&ManagedSyncViolation::RequiredSectionMissing));
}

#[test]
fn section_lane_mismatch_fails() {
    let mut packet = packet();
    packet.section_qualifications[0].matrix_lane_ref = "residency_encryption".to_owned();
    assert!(packet
        .validate()
        .contains(&ManagedSyncViolation::SectionLaneMismatch));
}

#[test]
fn read_only_item_with_write_scope_fails() {
    let mut packet = packet();
    packet.snapshot_classes[0].read_write_scope =
        CompanionReadWriteScope::BoundedWriteRelayedToHost;
    assert!(packet
        .validate()
        .contains(&ManagedSyncViolation::ReadOnlyScopeViolated));
}

#[test]
fn snapshot_without_local_authority_fails() {
    let mut packet = packet();
    packet.snapshot_classes[0].local_authoritative = false;
    assert!(packet
        .validate()
        .contains(&ManagedSyncViolation::SnapshotNotLocalAuthoritative));
}

#[test]
fn conflict_without_user_review_fails() {
    let mut packet = packet();
    packet.conflicts[0].requires_user_review = false;
    assert!(packet
        .validate()
        .contains(&ManagedSyncViolation::ConflictNotUserReviewed));
}

#[test]
fn verified_claim_without_verification_fails() {
    let mut packet = packet();
    let verified = packet
        .encrypted_storage
        .iter_mut()
        .find(|item| item.encryption_posture.is_verified_claim())
        .expect("verified claim present");
    verified.claim_verified = false;
    assert!(packet
        .validate()
        .contains(&ManagedSyncViolation::EncryptionClaimedButUnverified));
}

#[test]
fn unverified_claim_without_label_fails() {
    let mut packet = packet();
    let unverified = packet
        .encrypted_storage
        .iter_mut()
        .find(|item| item.encryption_posture == EncryptionPosture::ClaimedUnverified)
        .expect("unverified claim present");
    unverified.proof_label_shown = false;
    // The unverified claim was also stale-labeled; clear that to isolate the claim label.
    assert!(packet
        .validate()
        .contains(&ManagedSyncViolation::EncryptionClaimNotLabeled));
}

#[test]
fn missing_residency_region_fails() {
    let mut packet = packet();
    packet.encrypted_storage[0].residency_region_ref = String::new();
    assert!(packet
        .validate()
        .contains(&ManagedSyncViolation::ResidencyRegionMissing));
}

#[test]
fn unlabeled_stale_item_fails() {
    let mut packet = packet();
    packet.snapshot_classes[0].freshness = CompanionFreshnessState::Stale;
    packet.snapshot_classes[0].stale_label_shown = false;
    assert!(packet
        .validate()
        .contains(&ManagedSyncViolation::StaleStateNotLabeled));
}

#[test]
fn missing_handoff_ref_fails() {
    let mut packet = packet();
    packet.snapshot_classes[0].handoff.deep_link_ref = String::new();
    assert!(packet
        .validate()
        .contains(&ManagedSyncViolation::HandoffRefMissing));
}

#[test]
fn empty_section_content_fails() {
    let mut packet = packet();
    packet.conflicts.clear();
    assert!(packet
        .validate()
        .contains(&ManagedSyncViolation::SectionContentMissing));
}

#[test]
fn scope_contract_incomplete_fails() {
    let mut packet = packet();
    packet.scope_contract.no_silent_server_authority = false;
    assert!(packet
        .validate()
        .contains(&ManagedSyncViolation::ScopeContractIncomplete));
}

#[test]
fn inspectability_contract_incomplete_fails() {
    let mut packet = packet();
    packet
        .inspectability_contract
        .encryption_claim_provable_or_labeled = false;
    assert!(packet
        .validate()
        .contains(&ManagedSyncViolation::InspectabilityContractIncomplete));
}

#[test]
fn stale_state_honesty_incomplete_fails() {
    let mut packet = packet();
    packet.stale_state_honesty.never_show_stale_as_live = false;
    assert!(packet
        .validate()
        .contains(&ManagedSyncViolation::StaleStateHonestyIncomplete));
}

#[test]
fn locality_disclosure_incomplete_fails() {
    let mut packet = packet();
    packet.locality_disclosure.staged = String::new();
    assert!(packet
        .validate()
        .contains(&ManagedSyncViolation::LocalityDisclosureIncomplete));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ManagedSyncViolation::MissingSourceContracts));
}

#[test]
fn security_review_incomplete_fails() {
    let mut packet = packet();
    packet.security_review.e2ee_claimed_only_when_verifiable = false;
    assert!(packet
        .validate()
        .contains(&ManagedSyncViolation::SecurityReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .help_about_shows_encryption_and_residency_claim = false;
    assert!(packet
        .validate()
        .contains(&ManagedSyncViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ManagedSyncViolation::ProofFreshnessIncomplete));
}

fn healthy_observation() -> ManagedSyncObservation {
    ManagedSyncObservation {
        sync_provider_available: true,
        proof_fresh: true,
        admin_continuity_available: true,
        residency_and_encryption_verified: true,
        sync_inspectable: true,
        device_trust_intact: true,
        host_session_active: true,
        upstream_matrix_narrowed: false,
    }
}

#[test]
fn degradation_on_provider_unavailable_narrows_and_stales_items() {
    let mut packet = packet();
    packet.apply_managed_sync_degradation(&ManagedSyncObservation {
        sync_provider_available: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&ManagedSyncDegradedReason::SyncProviderUnavailable));
    assert!(packet
        .degraded_labels
        .contains(&ManagedSyncDegradedReason::FreshnessDowngradedToStale));
    // Every previously live/cached item is now stale and labeled.
    assert!(packet
        .snapshot_classes
        .iter()
        .all(|item| item.freshness == CompanionFreshnessState::Stale && item.stale_label_shown));
    // Beta snapshot section narrows to preview, staged_rollout to early_access.
    let snapshot = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == ManagedSyncSection::SnapshotClass)
        .expect("snapshot section present");
    assert_eq!(
        snapshot.qualification,
        M5CompanionQualificationClass::Preview
    );
    assert_eq!(snapshot.rollout_stage, M5CompanionRolloutStage::EarlyAccess);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_sync_uninspectable_unreconciles_and_narrows() {
    let mut packet = packet();
    packet.apply_managed_sync_degradation(&ManagedSyncObservation {
        sync_inspectable: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&ManagedSyncDegradedReason::SyncInspectionUnavailable));
    assert!(packet
        .degraded_labels
        .contains(&ManagedSyncDegradedReason::ReconciliationDowngraded));
    // Every snapshot, conflict, and device record narrows to unreconcilable.
    assert!(packet
        .snapshot_classes
        .iter()
        .all(|item| item.reconciliation == SyncReconciliationState::Unreconcilable));
    assert!(packet
        .conflicts
        .iter()
        .all(|item| item.reconciliation == SyncReconciliationState::Unreconcilable));
    assert!(packet
        .devices
        .iter()
        .all(|item| item.reconciliation == SyncReconciliationState::Unreconcilable));
    // The three managed-sync sections narrow; encrypted storage stays untouched.
    let snapshot = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == ManagedSyncSection::SnapshotClass)
        .expect("snapshot section present");
    assert_eq!(
        snapshot.qualification,
        M5CompanionQualificationClass::Preview
    );
    let encrypted = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == ManagedSyncSection::EncryptedStorage)
        .expect("encrypted section present");
    assert_eq!(
        encrypted.qualification,
        M5CompanionQualificationClass::Preview
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_residency_unverified_downgrades_claims_and_narrows() {
    let mut packet = packet();
    packet.apply_managed_sync_degradation(&ManagedSyncObservation {
        residency_and_encryption_verified: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&ManagedSyncDegradedReason::ResidencyOrEncryptionUnverified));
    assert!(packet
        .degraded_labels
        .contains(&ManagedSyncDegradedReason::EncryptionClaimDowngraded));
    // No verified encryption claim survives; every formerly-verified row is now
    // claimed-unverified, unverified, and labeled.
    assert!(packet
        .encrypted_storage
        .iter()
        .all(|item| !item.encryption_posture.is_verified_claim()));
    assert!(packet.encryption_claims_honestly_qualified());
    // Only the encrypted-storage section narrows.
    let encrypted = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == ManagedSyncSection::EncryptedStorage)
        .expect("encrypted section present");
    assert_eq!(
        encrypted.qualification,
        M5CompanionQualificationClass::Experimental
    );
    let snapshot = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == ManagedSyncSection::SnapshotClass)
        .expect("snapshot section present");
    assert_eq!(snapshot.qualification, M5CompanionQualificationClass::Beta);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_device_trust_narrowing_narrows_trust_and_sections() {
    let mut packet = packet();
    packet.apply_managed_sync_degradation(&ManagedSyncObservation {
        device_trust_intact: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&ManagedSyncDegradedReason::DeviceTrustNarrowed));
    // Trusted devices narrow to pending-approval.
    assert!(packet
        .devices
        .iter()
        .all(|item| item.trust_state != SyncDeviceTrustState::Trusted));
    // Conflict-review and device-registry sections narrow; snapshot stays stable-beta.
    let conflict = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == ManagedSyncSection::ConflictReview)
        .expect("conflict section present");
    assert_eq!(
        conflict.qualification,
        M5CompanionQualificationClass::Preview
    );
    let snapshot = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == ManagedSyncSection::SnapshotClass)
        .expect("snapshot section present");
    assert_eq!(snapshot.qualification, M5CompanionQualificationClass::Beta);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_admin_continuity_loss_narrows_device_and_encrypted() {
    let mut packet = packet();
    packet.apply_managed_sync_degradation(&ManagedSyncObservation {
        admin_continuity_available: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&ManagedSyncDegradedReason::AdminContinuityUnavailable));
    let device = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == ManagedSyncSection::DeviceRegistry)
        .expect("device section present");
    assert_eq!(device.qualification, M5CompanionQualificationClass::Preview);
    let encrypted = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == ManagedSyncSection::EncryptedStorage)
        .expect("encrypted section present");
    assert_eq!(
        encrypted.qualification,
        M5CompanionQualificationClass::Experimental
    );
    // The snapshot and conflict sections are untouched by admin continuity loss.
    let snapshot = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == ManagedSyncSection::SnapshotClass)
        .expect("snapshot section present");
    assert_eq!(snapshot.qualification, M5CompanionQualificationClass::Beta);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_inactive_host_unresolves_exact_handoffs() {
    let mut packet = packet();
    // Force one handoff to require an active host so the downgrade has an effect.
    packet.snapshot_classes[0].handoff.requires_active_host = true;
    packet.apply_managed_sync_degradation(&ManagedSyncObservation {
        host_session_active: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&ManagedSyncDegradedReason::HostSessionInactive));
    assert!(packet
        .degraded_labels
        .contains(&ManagedSyncDegradedReason::HandoffTargetUnresolved));
    assert!(packet
        .handoffs()
        .filter(|handoff| handoff.requires_active_host)
        .all(|handoff| handoff.resolution == CompanionHandoffResolution::Unresolved));
    assert!(packet
        .handoffs()
        .filter(|handoff| !handoff.requires_active_host)
        .all(|handoff| handoff.resolution == CompanionHandoffResolution::Exact));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn publishable_sections_excludes_withheld() {
    let mut packet = packet();
    let total = packet.section_qualifications.len();
    assert_eq!(packet.publishable_sections().count(), total);
    // Drive the preview encrypted-storage section down to withheld via repeated
    // residency-unverified narrowing.
    for _ in 0..4 {
        packet.apply_managed_sync_degradation(&ManagedSyncObservation {
            residency_and_encryption_verified: false,
            ..healthy_observation()
        });
    }
    let encrypted = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == ManagedSyncSection::EncryptedStorage)
        .expect("encrypted section present");
    assert_eq!(encrypted.rollout_stage, M5CompanionRolloutStage::Withheld);
    assert!(packet.publishable_sections().count() < total);
}

#[test]
fn export_contains_no_forbidden_material() {
    let packet = packet();
    assert!(!packet
        .validate()
        .contains(&ManagedSyncViolation::RawBoundaryMaterialInExport));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_managed_sync_maturity_surface_export()
        .expect("checked managed sync maturity export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked = current_stable_managed_sync_maturity_surface_export()
        .expect("checked managed sync maturity export validates");
    assert_eq!(
        checked,
        packet(),
        "checked export drifted from canonical builder"
    );
}

#[test]
fn markdown_summary_is_deterministic() {
    let packet = packet();
    let first = packet.render_markdown_summary();
    let second = packet.render_markdown_summary();
    assert_eq!(first, second);
    assert!(first.contains("## Snapshot classes"));
    assert!(first.contains("## Conflict review"));
    assert!(first.contains("## Device registry"));
    assert!(first.contains("## Encrypted storage"));
}
