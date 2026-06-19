use super::*;

fn page() -> LocalityTenantCardPage {
    seeded_locality_tenant_card_page()
}

fn entry_mut<'a>(input: &'a mut LocalityTenantInput, row_id: &str) -> &'a mut LocalityTenantEntry {
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
    assert!(validate_locality_tenant_card_page(&page).is_ok());
}

#[test]
fn seeded_page_summary_counts_match_input() {
    let page = page();
    assert_eq!(page.summary.entry_count, 5);
    assert_eq!(page.summary.managed_scope_entry_count, 4);
    assert_eq!(page.summary.local_core_entry_count, 1);
    assert_eq!(page.summary.region_pinned_entry_count, 4);
    assert_eq!(page.summary.fail_closed_entry_count, 0);
    assert_eq!(page.summary.narrowed_entry_count, 0);
    assert_eq!(page.summary.withdrawn_entry_count, 0);
    assert!(page.summary.vocabulary_consistent);
    assert_eq!(page.descriptors.len(), 5);
    assert_eq!(page.tenant_cards.len(), 5);
    assert_eq!(page.row_outcomes.len(), 5);
}

#[test]
fn every_managed_row_is_projected_onto_all_required_surfaces() {
    let page = page();
    // Four managed rows across six surfaces, plus one local-core row across five.
    let managed_projections = 4 * LocalitySurfaceClass::ALL.len();
    let local_projections = LocalitySurfaceClass::LOCAL_CORE.len();
    assert_eq!(
        page.summary.surface_projection_count,
        managed_projections + local_projections
    );
}

#[test]
fn descriptor_carries_plain_language_locality_and_retention() {
    let page = page();
    let descriptor = page
        .descriptor("continuity-row:managed-cloud-sync")
        .expect("managed cloud descriptor");
    assert_eq!(
        descriptor.processing_location_plain,
        "in a single managed region"
    );
    assert_eq!(descriptor.region_pin_plain, "pinned to a single region");
    assert_eq!(descriptor.retention_plain, "vendor default retention");
    assert!(!descriptor.fail_closed_on_managed_lane);
    assert!(descriptor
        .locality_summary_line
        .contains("us-west managed region"));
}

#[test]
fn tenant_card_carries_plain_language_scope_and_keys() {
    let page = page();
    let card = page
        .tenant_card("continuity-row:self-hosted-restore")
        .expect("self-hosted card");
    assert_eq!(card.tenant_scope_plain, "customer-owned tenant");
    assert_eq!(
        card.tenant_isolation_plain,
        "inside the customer's own boundary"
    );
    assert_eq!(card.key_mode_plain, "customer-managed keys");
    assert!(card.boundary_verified);
}

#[test]
fn every_surface_renders_identical_vocabulary_for_a_row() {
    let page = page();
    let row_id = "continuity-row:managed-relay-failover";
    let descriptor = page.descriptor(row_id).expect("descriptor");
    let projections: Vec<&LocalitySurfaceProjection> = page
        .surface_projections
        .iter()
        .filter(|projection| projection.row_id == row_id)
        .collect();
    assert_eq!(projections.len(), LocalitySurfaceClass::ALL.len());
    for projection in projections {
        assert_eq!(
            projection.locality_summary_line,
            descriptor.locality_summary_line
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
    assert!(outcome.narrow_reason_tokens.is_empty());

    let descriptor = page
        .descriptor("continuity-row:local-desktop-core")
        .expect("local-core descriptor");
    assert_eq!(descriptor.processing_location_plain, "on this device");
    assert_eq!(descriptor.region_pin_plain, "not applicable");
    assert_eq!(descriptor.retention_plain, "retained on this device");
}

#[test]
fn unhonorable_region_pin_fails_closed_on_managed_lane() {
    let mut input = seeded_locality_tenant_input();
    let entry = entry_mut(&mut input, "continuity-row:managed-cloud-sync");
    entry.region_pin_honor = RegionPinHonorState::CannotHonor;
    entry.region_pin_honor_token = RegionPinHonorState::CannotHonor.as_str().to_owned();

    let page = LocalityTenantCardPage::new(
        "t:fail-closed",
        "fail-closed",
        "2026-06-01T00:00:00Z",
        input,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Withdrawn.as_str()
    );
    assert_eq!(page.summary.fail_closed_entry_count, 1);
    assert_eq!(page.summary.withdrawn_entry_count, 1);
    let outcome = page
        .row_outcome("continuity-row:managed-cloud-sync")
        .expect("outcome");
    assert!(outcome.claim_withheld);
    assert!(outcome.fail_closed);
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.narrow_reason == LocalityTenantNarrowReasonClass::RegionPinUnhonored));
    assert_eq!(page.fail_closed_descriptors().len(), 1);
}

#[test]
fn undisclosed_processing_location_narrows_to_beta() {
    let mut input = seeded_locality_tenant_input();
    let entry = entry_mut(&mut input, "continuity-row:managed-relay-failover");
    entry.processing_location = LocalityClass::Undisclosed;
    entry.processing_location_token = LocalityClass::Undisclosed.as_str().to_owned();

    let page = LocalityTenantCardPage::new("t:loc", "loc", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|defect| defect.narrow_reason
        == LocalityTenantNarrowReasonClass::ProcessingLocationUndisclosed));
}

#[test]
fn undisclosed_retention_narrows_to_beta() {
    let mut input = seeded_locality_tenant_input();
    let entry = entry_mut(&mut input, "continuity-row:managed-cloud-sync");
    entry.retention_class = RetentionClass::RetentionUndisclosed;
    entry.retention_class_token = RetentionClass::RetentionUndisclosed.as_str().to_owned();

    let page = LocalityTenantCardPage::new("t:ret", "ret", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.narrow_reason
            == LocalityTenantNarrowReasonClass::RetentionClassUndisclosed));
}

#[test]
fn missing_region_pin_on_managed_row_holds_at_preview() {
    let mut input = seeded_locality_tenant_input();
    let entry = entry_mut(&mut input, "continuity-row:managed-cloud-sync");
    entry.region_pin = RegionPinClass::Unpinned;
    entry.region_pin_token = RegionPinClass::Unpinned.as_str().to_owned();

    let page = LocalityTenantCardPage::new("t:pin", "pin", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(page.defects.iter().any(|defect| defect.narrow_reason
        == LocalityTenantNarrowReasonClass::RegionPinUndeclaredOnManaged));
}

#[test]
fn unverified_tenant_boundary_holds_at_preview() {
    let mut input = seeded_locality_tenant_input();
    let entry = entry_mut(&mut input, "continuity-row:managed-relay-failover");
    entry.tenant_isolation = TenantIsolationClass::IsolationUnverified;
    entry.tenant_isolation_token = TenantIsolationClass::IsolationUnverified
        .as_str()
        .to_owned();

    let page = LocalityTenantCardPage::new("t:iso", "iso", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    let card = page
        .tenant_card("continuity-row:managed-relay-failover")
        .expect("card");
    assert!(!card.boundary_verified);
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.narrow_reason
            == LocalityTenantNarrowReasonClass::TenantBoundaryUnverified));
}

#[test]
fn self_hosted_claiming_vendor_region_is_withdrawn() {
    let mut input = seeded_locality_tenant_input();
    let entry = entry_mut(&mut input, "continuity-row:self-hosted-restore");
    entry.storage_location = LocalityClass::MultiRegion;
    entry.storage_location_token = LocalityClass::MultiRegion.as_str().to_owned();

    let page =
        LocalityTenantCardPage::new("t:overclaim", "overclaim", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Withdrawn.as_str()
    );
    assert!(page.defects.iter().any(|defect| defect.narrow_reason
        == LocalityTenantNarrowReasonClass::SelfHostedLocalityOverclaimed));
}

#[test]
fn incomplete_surface_projection_narrows_to_beta() {
    let mut input = seeded_locality_tenant_input();
    let entry = entry_mut(&mut input, "continuity-row:managed-cloud-sync");
    entry
        .projected_surfaces
        .retain(|surface| *surface != LocalitySurfaceClass::SupportExport);

    let page = LocalityTenantCardPage::new("t:surface", "surface", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|defect| defect.narrow_reason
        == LocalityTenantNarrowReasonClass::SurfaceProjectionIncomplete));
}

#[test]
fn tampered_projection_vocabulary_is_detected_on_reaudit() {
    let mut page = page();
    page.surface_projections[0].locality_summary_line = "drifted vocabulary".to_owned();
    let defects = audit_locality_tenant_card_page(&page);
    assert!(defects
        .iter()
        .any(|defect| defect.narrow_reason
            == LocalityTenantNarrowReasonClass::LocalityVocabularyDrift));
    assert!(validate_locality_tenant_card_page(&page).is_err());
}

#[test]
fn support_export_wraps_seeded_page_without_raw_private_material() {
    let export = LocalityTenantSupportExport::from_page(
        "continuity:locality-tenant:support-export:fixture-001",
        "2026-06-01T00:00:00Z",
        page(),
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.page.qualifies_stable());
}

#[test]
fn reauditing_seeded_page_returns_zero_defects() {
    let defects = audit_locality_tenant_card_page(&page());
    assert!(defects.is_empty(), "re-audit defects: {defects:?}");
}

#[test]
fn page_round_trips_through_json() {
    let page = page();
    let json = serde_json::to_string(&page).expect("serialize");
    let restored: LocalityTenantCardPage = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(page, restored);
}
