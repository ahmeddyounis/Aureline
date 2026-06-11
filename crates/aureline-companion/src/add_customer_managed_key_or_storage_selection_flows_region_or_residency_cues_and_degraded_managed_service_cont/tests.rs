use super::*;

const PACKET_ID: &str = "key-storage-residency-continuity-surface:stable:0001";
const PACKET_LABEL: &str =
    "Customer-Managed-Key and Storage Selection Flows, Region/Residency Cues, and Degraded Managed-Service Continuity";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn proof_freshness() -> ResidencyProofFreshness {
    ResidencyProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: MINTED_AT.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> KeyStorageResidencyContinuitySurfacePacket {
    canonical_key_storage_residency_continuity_surface(
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
        KeyStorageResidencySection::ALL.len()
    );
    for section in KeyStorageResidencySection::ALL {
        let row = packet
            .section_qualifications
            .iter()
            .find(|row| row.section == section)
            .expect("section present");
        assert_eq!(row.matrix_lane_ref, section.matrix_lane().as_str());
        assert_eq!(row.read_write_scope, section.bounded_scope());
    }
    let lane_of = |section: KeyStorageResidencySection| {
        packet
            .section_qualifications
            .iter()
            .find(|row| row.section == section)
            .map(|row| row.matrix_lane_ref.as_str())
            .unwrap()
    };
    // The first three sections inherit the residency_encryption lane; continuity
    // inherits the offboarding_continuity lane.
    assert_eq!(
        lane_of(KeyStorageResidencySection::KeyCustodySelection),
        M5CompanionMatrixLane::ResidencyEncryption.as_str()
    );
    assert_eq!(
        lane_of(KeyStorageResidencySection::StorageSelection),
        M5CompanionMatrixLane::ResidencyEncryption.as_str()
    );
    assert_eq!(
        lane_of(KeyStorageResidencySection::ResidencyCue),
        M5CompanionMatrixLane::ResidencyEncryption.as_str()
    );
    assert_eq!(
        lane_of(KeyStorageResidencySection::ManagedServiceContinuity),
        M5CompanionMatrixLane::OffboardingContinuity.as_str()
    );
}

#[test]
fn canonical_surface_handoffs_are_exact() {
    let packet = packet();
    assert!(packet.all_handoffs_exact());
    assert!(!packet.key_custody_selections.is_empty());
    assert!(!packet.storage_selections.is_empty());
    assert!(!packet.residency_cues.is_empty());
    assert!(!packet.continuity_rows.is_empty());
}

#[test]
fn every_section_is_read_only() {
    let packet = packet();
    assert!(packet
        .key_custody_selections
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
    assert!(packet
        .storage_selections
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
    assert!(packet
        .residency_cues
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
    assert!(packet
        .continuity_rows
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
}

#[test]
fn customer_managed_key_and_local_fallbacks_represented() {
    let packet = packet();
    assert!(packet.customer_managed_key_represented());
    assert!(packet.local_only_key_fallback_offered());
    assert!(packet.local_first_storage_fallback_offered());
}

#[test]
fn an_active_selection_is_present_in_each_selection_flow() {
    let packet = packet();
    assert!(packet
        .key_custody_selections
        .iter()
        .any(|item| item.selection_state == SelectionState::Active));
    assert!(packet
        .storage_selections
        .iter()
        .any(|item| item.selection_state == SelectionState::Active));
}

#[test]
fn encryption_claims_are_provable_or_labeled() {
    let packet = packet();
    assert!(packet.encryption_claims_honestly_qualified());
    assert!(packet
        .key_custody_selections
        .iter()
        .any(
            |item| item.encryption_posture == EncryptionPosture::EndToEndEncryptedVerified
                && item.claim_verified
        ));
}

#[test]
fn residency_claims_are_provable_or_labeled() {
    let packet = packet();
    assert!(packet.residency_claims_honestly_qualified());
    assert!(packet
        .residency_cues
        .iter()
        .any(|item| item.pin_state == ResidencyPinState::PinnedVerified && item.claim_verified));
    let unverified = packet
        .residency_cues
        .iter()
        .find(|item| item.pin_state == ResidencyPinState::PinnedUnverified)
        .expect("unverified pin present");
    assert!(unverified.proof_label_shown);
    assert!(!unverified.claim_verified);
}

#[test]
fn every_continuity_row_preserves_local_work() {
    let packet = packet();
    assert!(packet.local_work_never_stranded());
    // The corpus carries a fully-local capability and provider/admin-dependent ones.
    assert!(packet
        .continuity_rows
        .iter()
        .any(|item| item.continuity_posture == ContinuityPosture::LocalCoreContinuesUnaffected));
    assert!(packet
        .continuity_rows
        .iter()
        .any(|item| item.continuity_posture == ContinuityPosture::RequiresAdminContinuity));
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
        .contains(&ResidencyViolation::RequiredSectionMissing));
}

#[test]
fn section_lane_mismatch_fails() {
    let mut packet = packet();
    packet.section_qualifications[0].matrix_lane_ref = "offboarding_continuity".to_owned();
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::SectionLaneMismatch));
}

#[test]
fn read_only_item_with_write_scope_fails() {
    let mut packet = packet();
    packet.key_custody_selections[0].read_write_scope =
        CompanionReadWriteScope::BoundedWriteRelayedToHost;
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::ReadOnlyScopeViolated));
}

#[test]
fn missing_local_key_fallback_fails() {
    let mut packet = packet();
    packet
        .key_custody_selections
        .retain(|item| item.offered_custody != KeyCustody::LocalOnlyNoKeyEscrow);
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::LocalKeyFallbackMissing));
}

#[test]
fn missing_local_storage_fallback_fails() {
    let mut packet = packet();
    packet
        .storage_selections
        .retain(|item| !item.offered_location.is_local_fallback());
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::LocalStorageFallbackMissing));
}

#[test]
fn verified_encryption_claim_without_verification_fails() {
    let mut packet = packet();
    let verified = packet
        .key_custody_selections
        .iter_mut()
        .find(|item| item.encryption_posture.is_verified_claim())
        .expect("verified claim present");
    verified.claim_verified = false;
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::EncryptionClaimedButUnverified));
}

#[test]
fn unverified_residency_pin_without_label_fails() {
    let mut packet = packet();
    let unverified = packet
        .residency_cues
        .iter_mut()
        .find(|item| item.pin_state == ResidencyPinState::PinnedUnverified)
        .expect("unverified pin present");
    unverified.proof_label_shown = false;
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::ResidencyClaimNotLabeled));
}

#[test]
fn verified_residency_pin_without_verification_fails() {
    let mut packet = packet();
    let verified = packet
        .residency_cues
        .iter_mut()
        .find(|item| item.pin_state.is_verified_claim())
        .expect("verified pin present");
    verified.claim_verified = false;
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::ResidencyClaimedButUnverified));
}

#[test]
fn missing_residency_region_fails() {
    let mut packet = packet();
    packet.residency_cues[0].residency_region_ref = String::new();
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::ResidencyRegionMissing));
}

#[test]
fn stranded_local_work_fails() {
    let mut packet = packet();
    packet.continuity_rows[0].local_work_preserved = false;
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::LocalWorkStranded));
}

#[test]
fn unlabeled_stale_item_fails() {
    let mut packet = packet();
    packet.key_custody_selections[0].freshness = CompanionFreshnessState::Stale;
    packet.key_custody_selections[0].stale_label_shown = false;
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::StaleStateNotLabeled));
}

#[test]
fn missing_handoff_ref_fails() {
    let mut packet = packet();
    packet.key_custody_selections[0].handoff.deep_link_ref = String::new();
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::HandoffRefMissing));
}

#[test]
fn empty_section_content_fails() {
    let mut packet = packet();
    packet.continuity_rows.clear();
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::SectionContentMissing));
}

#[test]
fn scope_contract_incomplete_fails() {
    let mut packet = packet();
    packet
        .scope_contract
        .selection_applied_by_local_core_not_surface = false;
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::ScopeContractIncomplete));
}

#[test]
fn provability_contract_incomplete_fails() {
    let mut packet = packet();
    packet
        .provability_contract
        .residency_claim_provable_or_labeled = false;
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::ProvabilityContractIncomplete));
}

#[test]
fn stale_state_honesty_incomplete_fails() {
    let mut packet = packet();
    packet.stale_state_honesty.never_show_stale_as_live = false;
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::StaleStateHonestyIncomplete));
}

#[test]
fn continuity_contract_incomplete_fails() {
    let mut packet = packet();
    packet.continuity_contract.local_work_never_stranded = false;
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::ContinuityContractIncomplete));
}

#[test]
fn locality_disclosure_incomplete_fails() {
    let mut packet = packet();
    packet.locality_disclosure.staged = String::new();
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::LocalityDisclosureIncomplete));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::MissingSourceContracts));
}

#[test]
fn security_review_incomplete_fails() {
    let mut packet = packet();
    packet.security_review.local_work_never_stranded = false;
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::SecurityReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .help_about_shows_encryption_and_residency_claim = false;
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ResidencyViolation::ProofFreshnessIncomplete));
}

fn healthy_observation() -> ResidencyContinuityObservation {
    ResidencyContinuityObservation {
        key_management_available: true,
        storage_provider_available: true,
        proof_fresh: true,
        residency_verified: true,
        encryption_verified: true,
        admin_continuity_available: true,
        managed_service_available: true,
        host_session_active: true,
        upstream_matrix_narrowed: false,
    }
}

#[test]
fn degradation_on_managed_service_degraded_narrows_and_stales_items() {
    let mut packet = packet();
    packet.apply_residency_continuity_degradation(&ResidencyContinuityObservation {
        managed_service_available: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&ResidencyDegradedReason::ManagedServiceDegraded));
    assert!(packet
        .degraded_labels
        .contains(&ResidencyDegradedReason::FreshnessDowngradedToStale));
    // Every previously live/cached item is now stale and labeled.
    assert!(packet
        .key_custody_selections
        .iter()
        .all(|item| item.freshness == CompanionFreshnessState::Stale && item.stale_label_shown));
    // Every non-local continuity capability is marked degraded; the local one is not.
    for item in &packet.continuity_rows {
        if item.continuity_posture == ContinuityPosture::LocalCoreContinuesUnaffected {
            assert!(!item.degraded);
        } else {
            assert!(item.degraded);
        }
    }
    // Local work is still preserved everywhere.
    assert!(packet.local_work_never_stranded());
    // The key-custody section narrows preview → experimental.
    let key = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == KeyStorageResidencySection::KeyCustodySelection)
        .expect("key section present");
    assert_eq!(
        key.qualification,
        M5CompanionQualificationClass::Experimental
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_key_management_loss_narrows_non_local_selections() {
    let mut packet = packet();
    packet.apply_residency_continuity_degradation(&ResidencyContinuityObservation {
        key_management_available: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&ResidencyDegradedReason::KeyManagementUnavailable));
    assert!(packet
        .degraded_labels
        .contains(&ResidencyDegradedReason::SelectionNarrowedToLocalFallback));
    // The local-only key option keeps its available state; non-local ones narrow.
    let local = packet
        .key_custody_selections
        .iter()
        .find(|item| item.offered_custody == KeyCustody::LocalOnlyNoKeyEscrow)
        .expect("local key present");
    assert_eq!(local.selection_state, SelectionState::Available);
    assert!(packet
        .key_custody_selections
        .iter()
        .filter(|item| item.offered_custody != KeyCustody::LocalOnlyNoKeyEscrow)
        .all(|item| item.selection_state == SelectionState::RequiresAdminApproval));
    // Only the key-custody section narrows.
    let key = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == KeyStorageResidencySection::KeyCustodySelection)
        .expect("key section present");
    assert_eq!(
        key.qualification,
        M5CompanionQualificationClass::Experimental
    );
    let storage = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == KeyStorageResidencySection::StorageSelection)
        .expect("storage section present");
    assert_eq!(
        storage.qualification,
        M5CompanionQualificationClass::Preview
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_storage_loss_narrows_non_local_storage_selections() {
    let mut packet = packet();
    packet.apply_residency_continuity_degradation(&ResidencyContinuityObservation {
        storage_provider_available: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&ResidencyDegradedReason::StorageProviderUnavailable));
    assert!(packet
        .degraded_labels
        .contains(&ResidencyDegradedReason::SelectionNarrowedToLocalFallback));
    // Local-first/local-only storage stays; non-local-fallback options narrow.
    assert!(packet
        .storage_selections
        .iter()
        .filter(|item| item.offered_location.is_local_fallback())
        .all(
            |item| item.selection_state != SelectionState::RequiresAdminApproval
                || item.offered_location == StorageLocationKind::HybridLocalFirst
        ));
    assert!(packet
        .storage_selections
        .iter()
        .filter(|item| !item.offered_location.is_local_fallback()
            && item.offered_location == StorageLocationKind::CustomerManagedBucket)
        .all(|item| item.selection_state == SelectionState::RequiresAdminApproval));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_residency_unverified_downgrades_pins_and_narrows() {
    let mut packet = packet();
    packet.apply_residency_continuity_degradation(&ResidencyContinuityObservation {
        residency_verified: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&ResidencyDegradedReason::ResidencyUnverified));
    assert!(packet
        .degraded_labels
        .contains(&ResidencyDegradedReason::ResidencyClaimDowngraded));
    // No verified residency pin survives.
    assert!(packet
        .residency_cues
        .iter()
        .all(|item| !item.pin_state.is_verified_claim()));
    assert!(packet.residency_claims_honestly_qualified());
    // Storage residency claims also dropped to unverified-and-labeled.
    assert!(packet
        .storage_selections
        .iter()
        .all(|item| !item.claim_verified || item.proof_label_shown));
    // Residency-cue and storage sections narrow; key-custody stays preview.
    let residency = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == KeyStorageResidencySection::ResidencyCue)
        .expect("residency section present");
    assert_eq!(
        residency.qualification,
        M5CompanionQualificationClass::Experimental
    );
    let key = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == KeyStorageResidencySection::KeyCustodySelection)
        .expect("key section present");
    assert_eq!(key.qualification, M5CompanionQualificationClass::Preview);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_encryption_unverified_downgrades_claims() {
    let mut packet = packet();
    packet.apply_residency_continuity_degradation(&ResidencyContinuityObservation {
        encryption_verified: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&ResidencyDegradedReason::EncryptionUnverified));
    assert!(packet
        .degraded_labels
        .contains(&ResidencyDegradedReason::EncryptionClaimDowngraded));
    assert!(packet
        .key_custody_selections
        .iter()
        .all(|item| !item.encryption_posture.is_verified_claim()));
    assert!(packet.encryption_claims_honestly_qualified());
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_admin_continuity_loss_narrows_key_residency_and_continuity() {
    let mut packet = packet();
    packet.apply_residency_continuity_degradation(&ResidencyContinuityObservation {
        admin_continuity_available: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&ResidencyDegradedReason::AdminContinuityUnavailable));
    let key = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == KeyStorageResidencySection::KeyCustodySelection)
        .expect("key section present");
    assert_eq!(
        key.qualification,
        M5CompanionQualificationClass::Experimental
    );
    let continuity = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == KeyStorageResidencySection::ManagedServiceContinuity)
        .expect("continuity section present");
    assert_eq!(
        continuity.qualification,
        M5CompanionQualificationClass::Preview
    );
    // Storage section is untouched by admin continuity loss.
    let storage = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == KeyStorageResidencySection::StorageSelection)
        .expect("storage section present");
    assert_eq!(
        storage.qualification,
        M5CompanionQualificationClass::Preview
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_inactive_host_unresolves_exact_handoffs() {
    let mut packet = packet();
    packet.key_custody_selections[0]
        .handoff
        .requires_active_host = true;
    packet.apply_residency_continuity_degradation(&ResidencyContinuityObservation {
        host_session_active: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&ResidencyDegradedReason::HostSessionInactive));
    assert!(packet
        .degraded_labels
        .contains(&ResidencyDegradedReason::HandoffTargetUnresolved));
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
    // Drive the preview residency-cue section down to withheld via repeated
    // residency-unverified narrowing.
    for _ in 0..4 {
        packet.apply_residency_continuity_degradation(&ResidencyContinuityObservation {
            residency_verified: false,
            ..healthy_observation()
        });
    }
    let residency = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == KeyStorageResidencySection::ResidencyCue)
        .expect("residency section present");
    assert_eq!(residency.rollout_stage, M5CompanionRolloutStage::Withheld);
    assert!(packet.publishable_sections().count() < total);
}

#[test]
fn export_contains_no_forbidden_material() {
    let packet = packet();
    assert!(!packet
        .validate()
        .contains(&ResidencyViolation::RawBoundaryMaterialInExport));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_key_storage_residency_continuity_surface_export()
        .expect("checked export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked = current_stable_key_storage_residency_continuity_surface_export()
        .expect("checked export validates");
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
    assert!(first.contains("## Key custody selection"));
    assert!(first.contains("## Storage selection"));
    assert!(first.contains("## Residency cues"));
    assert!(first.contains("## Managed-service continuity"));
}
