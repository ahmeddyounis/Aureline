use super::*;

fn page() -> KeyModeStoragePosturePage {
    seeded_key_mode_storage_posture_page()
}

fn entry_mut<'a>(
    input: &'a mut KeyModeStoragePostureInput,
    row_id: &str,
) -> &'a mut KeyModeStorageEntry {
    input
        .entries
        .iter_mut()
        .find(|entry| entry.row_id == row_id)
        .unwrap_or_else(|| panic!("missing seeded entry: {row_id}"))
}

#[test]
fn seeded_page_qualifies_stable_with_zero_defects() {
    let page = page();
    assert!(page.qualifies_stable());
    assert!(
        page.defects.is_empty(),
        "seeded defects: {:?}",
        page.defects
    );
    assert!(validate_key_mode_storage_posture_page(&page).is_ok());
}

#[test]
fn seeded_page_summary_counts_match_input() {
    let page = page();
    assert_eq!(page.summary.entry_count, 5);
    assert_eq!(page.summary.managed_scope_entry_count, 4);
    assert_eq!(page.summary.local_core_entry_count, 1);
    // self-hosted (customer-managed keys) + sovereign (customer-held root).
    assert_eq!(page.summary.customer_controlled_key_entry_count, 2);
    assert_eq!(page.summary.offline_trust_root_entry_count, 1);
    assert_eq!(page.summary.fail_closed_entry_count, 0);
    assert_eq!(page.summary.store_locked_entry_count, 0);
    assert_eq!(page.summary.narrowed_entry_count, 0);
    assert_eq!(page.summary.withdrawn_entry_count, 0);
    assert!(page.summary.vocabulary_consistent);
    assert!(page.summary.all_local_core_preserved);
    assert!(page.summary.raw_key_material_excluded);
    assert_eq!(page.key_descriptors.len(), 5);
    assert_eq!(page.storage_descriptors.len(), 5);
    assert_eq!(page.row_outcomes.len(), 5);
}

#[test]
fn every_managed_row_is_projected_onto_all_required_surfaces() {
    let page = page();
    // Four managed rows across six surfaces, plus one local-core row across five.
    let managed_projections = 4 * KeyPostureSurfaceClass::ALL.len();
    let local_projections = KeyPostureSurfaceClass::LOCAL_CORE.len();
    assert_eq!(
        page.summary.surface_projection_count,
        managed_projections + local_projections
    );
}

#[test]
fn descriptor_carries_plain_language_key_mode_and_trust_root() {
    let page = page();
    let descriptor = page
        .key_descriptor("continuity-row:self-hosted-restore")
        .expect("self-hosted descriptor");
    assert_eq!(descriptor.key_mode_plain, "customer-managed keys");
    assert_eq!(
        descriptor.trust_root_posture_plain,
        "customer-managed trust root"
    );
    assert_eq!(descriptor.key_availability_plain, "available");
    assert_eq!(descriptor.degraded_state_plain, "healthy");
    assert!(!descriptor.fail_closed_on_managed_lane);
    assert!(descriptor.local_core_preserved);
    assert!(descriptor.protects_durable_state);
}

#[test]
fn sovereign_row_uses_an_offline_trust_root() {
    let page = page();
    let descriptor = page
        .key_descriptor("continuity-row:sovereign-airgap-snapshot")
        .expect("sovereign descriptor");
    assert_eq!(descriptor.trust_root_posture_token, "offline_trust_root");
    assert_eq!(descriptor.key_mode_plain, "customer-held root key");
    let storage = page
        .storage_descriptor("continuity-row:sovereign-airgap-snapshot")
        .expect("sovereign storage");
    assert_eq!(storage.storage_encryption_token, "offline_sealed_encrypted");
    assert!(storage.key_mode_visible);
}

#[test]
fn storage_descriptor_names_the_key_mode_protecting_storage() {
    let page = page();
    let storage = page
        .storage_descriptor("continuity-row:managed-cloud-sync")
        .expect("managed storage");
    assert_eq!(
        storage.storage_encryption_plain,
        "encrypted with vendor-managed keys"
    );
    assert_eq!(storage.key_mode_plain, "vendor-managed keys");
    assert!(storage.key_mode_visible);
    assert!(storage.storage_summary_line.contains("Storage encrypted"));
}

#[test]
fn every_surface_renders_identical_vocabulary_for_a_row() {
    let page = page();
    let row_id = "continuity-row:managed-relay-failover";
    let key = page.key_descriptor(row_id).expect("descriptor");
    let storage = page.storage_descriptor(row_id).expect("storage");
    let projections: Vec<&KeyPostureSurfaceProjection> = page
        .surface_projections
        .iter()
        .filter(|projection| projection.row_id == row_id)
        .collect();
    assert_eq!(projections.len(), KeyPostureSurfaceClass::ALL.len());
    for projection in projections {
        assert_eq!(projection.key_summary_line, key.key_summary_line);
        assert_eq!(
            projection.storage_summary_line,
            storage.storage_summary_line
        );
    }
}

#[test]
fn local_core_row_is_out_of_managed_scope_and_stable() {
    let page = page();
    let outcome = page
        .row_outcome("continuity-row:local-desktop-core")
        .expect("local-core outcome");
    assert!(!outcome.in_managed_scope);
    assert!(!outcome.narrowed);
    assert!(!outcome.fail_closed);
    assert!(outcome.local_core_preserved);
    assert!(outcome.narrow_reason_tokens.is_empty());

    let descriptor = page
        .key_descriptor("continuity-row:local-desktop-core")
        .expect("local-core descriptor");
    assert_eq!(descriptor.key_mode_plain, "local OS keystore");
    assert_eq!(
        descriptor.trust_root_posture_plain,
        "OS keystore trust root"
    );
    assert_eq!(descriptor.store_lock_plain, "unlocked");
}

#[test]
fn customer_key_unavailable_fails_closed_on_managed_lane() {
    let mut input = seeded_key_mode_storage_posture_input();
    let entry = entry_mut(&mut input, "continuity-row:self-hosted-restore");
    entry.key_availability = KeyAvailabilityState::CustomerKeyUnavailable;
    entry.key_availability_token = KeyAvailabilityState::CustomerKeyUnavailable
        .as_str()
        .to_owned();

    let page = KeyModeStoragePosturePage::new("t:cku", "cku", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Withdrawn.as_str()
    );
    assert_eq!(page.summary.fail_closed_entry_count, 1);
    assert_eq!(page.summary.withdrawn_entry_count, 1);
    // Fail-closed narrows ONLY the protected managed lane; local-core survives.
    assert!(page.summary.all_local_core_preserved);

    let outcome = page
        .row_outcome("continuity-row:self-hosted-restore")
        .expect("outcome");
    assert!(outcome.claim_withheld);
    assert!(outcome.fail_closed);
    assert!(outcome.local_core_preserved);
    assert_eq!(outcome.degraded_state_token, "managed_lane_fail_closed");

    // The local-core row stays stable and untouched by the managed key failure.
    let local = page
        .row_outcome("continuity-row:local-desktop-core")
        .expect("local outcome");
    assert!(!local.narrowed);

    let descriptor = page
        .key_descriptor("continuity-row:self-hosted-restore")
        .expect("descriptor");
    assert_eq!(
        descriptor.degraded_state_plain,
        "managed lane failed closed; local-safe work preserved"
    );
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.narrow_reason == KeyPostureNarrowReasonClass::CustomerKeyUnavailable));
    assert_eq!(page.fail_closed_descriptors().len(), 1);
}

#[test]
fn trust_root_mismatch_fails_closed() {
    let mut input = seeded_key_mode_storage_posture_input();
    let entry = entry_mut(&mut input, "continuity-row:sovereign-airgap-snapshot");
    entry.key_availability = KeyAvailabilityState::TrustRootMismatch;
    entry.key_availability_token = KeyAvailabilityState::TrustRootMismatch.as_str().to_owned();

    let page = KeyModeStoragePosturePage::new("t:trm", "trm", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Withdrawn.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.narrow_reason == KeyPostureNarrowReasonClass::TrustRootMismatch));
}

#[test]
fn key_material_lost_fails_closed() {
    let mut input = seeded_key_mode_storage_posture_input();
    let entry = entry_mut(&mut input, "continuity-row:managed-cloud-sync");
    entry.key_availability = KeyAvailabilityState::KeyMaterialLost;
    entry.key_availability_token = KeyAvailabilityState::KeyMaterialLost.as_str().to_owned();

    let page = KeyModeStoragePosturePage::new("t:kml", "kml", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Withdrawn.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.narrow_reason == KeyPostureNarrowReasonClass::KeyMaterialLost));
}

#[test]
fn locked_store_on_managed_lane_holds_at_preview() {
    let mut input = seeded_key_mode_storage_posture_input();
    let entry = entry_mut(&mut input, "continuity-row:managed-cloud-sync");
    entry.store_lock = StoreLockState::Locked;
    entry.store_lock_token = StoreLockState::Locked.as_str().to_owned();

    let page = KeyModeStoragePosturePage::new("t:lock", "lock", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert_eq!(page.summary.store_locked_entry_count, 1);
    let outcome = page
        .row_outcome("continuity-row:managed-cloud-sync")
        .expect("outcome");
    assert!(!outcome.fail_closed);
    assert_eq!(outcome.degraded_state_token, "store_locked_degraded");
    assert!(page.defects.iter().any(
        |defect| defect.narrow_reason == KeyPostureNarrowReasonClass::StoreLockedOnManagedLane
    ));
}

#[test]
fn encrypted_without_named_key_mode_narrows_to_beta() {
    let mut input = seeded_key_mode_storage_posture_input();
    let entry = entry_mut(&mut input, "continuity-row:managed-relay-failover");
    entry.storage_encryption = StorageEncryptionClass::EncryptedKeyModeOpaque;
    entry.storage_encryption_token = StorageEncryptionClass::EncryptedKeyModeOpaque
        .as_str()
        .to_owned();

    let page = KeyModeStoragePosturePage::new("t:opaque", "opaque", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    let storage = page
        .storage_descriptor("continuity-row:managed-relay-failover")
        .expect("storage");
    assert!(!storage.key_mode_visible);
    assert!(
        page.defects
            .iter()
            .any(|defect| defect.narrow_reason
                == KeyPostureNarrowReasonClass::EncryptionPostureOpaque)
    );
}

#[test]
fn undisclosed_encryption_narrows_to_beta() {
    let mut input = seeded_key_mode_storage_posture_input();
    let entry = entry_mut(&mut input, "continuity-row:managed-cloud-sync");
    entry.storage_encryption = StorageEncryptionClass::EncryptionUndisclosed;
    entry.storage_encryption_token = StorageEncryptionClass::EncryptionUndisclosed
        .as_str()
        .to_owned();

    let page = KeyModeStoragePosturePage::new("t:enc", "enc", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.narrow_reason
            == KeyPostureNarrowReasonClass::EncryptionPostureUndisclosed));
}

#[test]
fn stale_key_evidence_holds_at_preview() {
    let mut input = seeded_key_mode_storage_posture_input();
    let entry = entry_mut(&mut input, "continuity-row:managed-cloud-sync");
    entry.key_evidence_state = KeyEvidenceStateClass::StaleNeedsRecheck;
    entry.key_evidence_state_token = KeyEvidenceStateClass::StaleNeedsRecheck.as_str().to_owned();

    let page = KeyModeStoragePosturePage::new("t:stale", "stale", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.narrow_reason == KeyPostureNarrowReasonClass::KeyEvidenceStale));
}

#[test]
fn missing_key_evidence_narrows_to_beta() {
    let mut input = seeded_key_mode_storage_posture_input();
    let entry = entry_mut(&mut input, "continuity-row:managed-relay-failover");
    entry.key_evidence_state = KeyEvidenceStateClass::Missing;
    entry.key_evidence_state_token = KeyEvidenceStateClass::Missing.as_str().to_owned();
    entry.key_posture_evidence_ref = String::new();

    let page = KeyModeStoragePosturePage::new("t:miss", "miss", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(
        |defect| defect.narrow_reason == KeyPostureNarrowReasonClass::KeyPostureEvidenceMissing
    ));
}

#[test]
fn self_hosted_relying_on_vendor_keys_holds_at_preview() {
    let mut input = seeded_key_mode_storage_posture_input();
    let entry = entry_mut(&mut input, "continuity-row:self-hosted-restore");
    entry.key_mode = KeyModeClass::VendorManagedKeys;
    entry.key_mode_token = KeyModeClass::VendorManagedKeys.as_str().to_owned();

    let page =
        KeyModeStoragePosturePage::new("t:mismatch", "mismatch", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.narrow_reason == KeyPostureNarrowReasonClass::ProfileKeyModeMismatch));
}

#[test]
fn incomplete_surface_projection_narrows_to_beta() {
    let mut input = seeded_key_mode_storage_posture_input();
    let entry = entry_mut(&mut input, "continuity-row:managed-cloud-sync");
    entry
        .projected_surfaces
        .retain(|surface| *surface != KeyPostureSurfaceClass::SupportCenter);

    let page =
        KeyModeStoragePosturePage::new("t:surface", "surface", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.narrow_reason == KeyPostureNarrowReasonClass::SurfaceReuseIncomplete));
}

#[test]
fn tampered_projection_vocabulary_is_detected_on_reaudit() {
    let mut page = page();
    page.surface_projections[0].key_summary_line = "drifted vocabulary".to_owned();
    let defects = audit_key_mode_storage_posture_page(&page);
    assert!(defects.iter().any(
        |defect| defect.narrow_reason == KeyPostureNarrowReasonClass::KeyStorageVocabularyDrift
    ));
    assert!(validate_key_mode_storage_posture_page(&page).is_err());
}

#[test]
fn support_export_wraps_seeded_page_without_raw_key_material() {
    let export = KeyModeStoragePostureSupportExport::from_page(
        "continuity:key-posture:support-export:fixture-001",
        "2026-06-01T00:00:00Z",
        page(),
    );
    assert!(export.raw_key_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.page.qualifies_stable());
}

#[test]
fn reauditing_seeded_page_returns_zero_defects() {
    let defects = audit_key_mode_storage_posture_page(&page());
    assert!(defects.is_empty(), "re-audit defects: {defects:?}");
}

#[test]
fn page_round_trips_through_json() {
    let page = page();
    let json = serde_json::to_string(&page).expect("serialize");
    let restored: KeyModeStoragePosturePage = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(page, restored);
}
