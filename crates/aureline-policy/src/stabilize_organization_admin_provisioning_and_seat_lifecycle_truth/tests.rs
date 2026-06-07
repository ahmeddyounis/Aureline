use super::*;

fn page() -> OrganizationAdminTruthPage {
    seeded_organization_admin_truth_page()
}

#[test]
fn seeded_page_qualifies_stable() {
    let page = page();
    assert!(page.qualifies_stable(), "defects: {:?}", page.defects);
    assert!(page.defects.is_empty());
    assert!(validate_organization_admin_truth_page(&page).is_ok());
}

#[test]
fn seeded_page_covers_all_required_truth() {
    let page = page();
    assert!(page.covers_all_provisioning_classes());
    assert!(page.covers_all_lifecycle_flows());
    assert!(page.all_local_safety_guarantees_preserved());
    assert!(page.boundary_truth_visible_everywhere());
    assert_eq!(page.summary.rollout_ring_count, 2);
}

#[test]
fn seeded_provider_cards_cover_oidc_scim_signed_file_and_manual_paths() {
    let page = page();
    let classes: BTreeSet<&str> = page
        .provider_cards
        .iter()
        .map(|provider| provider.provisioning_class_token.as_str())
        .collect();
    assert!(classes.contains("oidc"));
    assert!(classes.contains("scim"));
    assert!(classes.contains("signed_file_bundle"));
    assert!(classes.contains("manual"));
}

#[test]
fn seeded_impact_previews_cover_required_lifecycle_flows() {
    let page = page();
    let flows: BTreeSet<&str> = page
        .impact_previews
        .iter()
        .map(|preview| preview.flow_class_token.as_str())
        .collect();
    assert!(flows.contains("seat_transfer"));
    assert!(flows.contains("suspension"));
    assert!(flows.contains("downgrade"));
    assert!(flows.contains("org_switch"));
    assert!(flows.contains("deprovision"));
}

#[test]
fn deprovision_preview_keeps_local_safety_and_export_explicit() {
    let page = page();
    let preview = page
        .impact_previews
        .iter()
        .find(|preview| preview.flow_class == LifecycleFlowClass::Deprovision)
        .expect("seeded page must include deprovision preview");
    assert_eq!(
        preview.failure_kind,
        Some(AdminFailureKind::DeprovisioningImpact)
    );
    assert!(preview.local_editing_preserved);
    assert!(preview.local_history_preserved);
    assert!(preview.unsaved_work_preserved);
    assert!(!preview.local_only_continuation_label.is_empty());
    assert!(!preview.export_rights_label.is_empty());
}

#[test]
fn seat_loss_is_not_generic_sign_in_failure() {
    let page = page();
    let suspension = page
        .impact_previews
        .iter()
        .find(|preview| preview.flow_class == LifecycleFlowClass::Suspension)
        .expect("seeded page must include suspension preview");
    assert_eq!(suspension.failure_kind, Some(AdminFailureKind::SeatLoss));
    assert_eq!(suspension.failure_kind_token, "seat_loss");
}

#[test]
fn support_export_carries_tenant_rollout_provisioning_and_local_safety_truth() {
    let page = page();
    let export = OrganizationAdminTruthSupportExport::from_page(
        "policy:organization-admin-truth:support-export:test",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert_eq!(
        export.org_tenant_ref,
        "org:acme-platform/tenant:sovereign-eu"
    );
    assert!(export.rollout_ring_refs.contains(&"ring:stable".to_owned()));
    assert!(export
        .provisioning_source_tokens
        .contains(&"oidc".to_owned()));
    assert!(export
        .provisioning_source_tokens
        .contains(&"scim".to_owned()));
    assert!(!export.seat_lifecycle_refs.is_empty());
    assert!(export
        .local_safety_guarantees
        .iter()
        .any(|note| note.contains("local history")));
    assert!(export.raw_private_material_excluded);
}

#[test]
fn missing_provisioning_class_coverage_narrows_to_preview() {
    let mut page = page();
    page.provider_cards
        .retain(|provider| provider.provisioning_class != OrganizationProvisioningClass::Manual);
    let page = OrganizationAdminTruthPage::new(
        "test:missing-provisioning",
        "missing provisioning class",
        "2026-06-01T00:00:00Z",
        page.overview,
        page.provider_cards,
        page.seat_lifecycle_rows,
        page.impact_previews,
        page.rollout_rings,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        OrganizationAdminTruthQualificationClass::Preview.as_str()
    );
    assert!(page.defects.iter().any(|defect| {
        defect.narrow_reason
            == OrganizationAdminTruthNarrowReasonClass::MissingProvisioningClassCoverage
    }));
}

#[test]
fn degraded_provider_without_specific_failure_narrows_to_beta() {
    let mut page = page();
    let provider = page
        .provider_cards
        .iter_mut()
        .find(|provider| provider.provider_state == ProviderStateClass::Degraded)
        .expect("seeded page must include degraded provider");
    provider.failure_kind = None;
    provider.failure_kind_token.clear();
    let page = OrganizationAdminTruthPage::new(
        "test:provider-failure-kind",
        "provider failure kind missing",
        "2026-06-01T00:00:00Z",
        page.overview,
        page.provider_cards,
        page.seat_lifecycle_rows,
        page.impact_previews,
        page.rollout_rings,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        OrganizationAdminTruthQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|defect| {
        defect.narrow_reason == OrganizationAdminTruthNarrowReasonClass::FailureKindNotSpecific
    }));
}

#[test]
fn generic_failure_kind_token_narrows_to_beta() {
    let mut page = page();
    let preview = page
        .impact_previews
        .iter_mut()
        .find(|preview| preview.flow_class == LifecycleFlowClass::OrgSwitch)
        .expect("seeded page must include org switch preview");
    preview.failure_kind = None;
    preview.failure_kind_token = "generic_admin_error".to_owned();
    let page = OrganizationAdminTruthPage::new(
        "test:generic-failure",
        "generic failure token",
        "2026-06-01T00:00:00Z",
        page.overview,
        page.provider_cards,
        page.seat_lifecycle_rows,
        page.impact_previews,
        page.rollout_rings,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        OrganizationAdminTruthQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|defect| {
        defect.narrow_reason == OrganizationAdminTruthNarrowReasonClass::FailureKindNotSpecific
    }));
}

#[test]
fn hidden_boundary_truth_narrows_to_beta() {
    let mut page = page();
    page.provider_cards[0].surface_visibility.about = false;
    let page = OrganizationAdminTruthPage::new(
        "test:hidden-boundary",
        "hidden boundary truth",
        "2026-06-01T00:00:00Z",
        page.overview,
        page.provider_cards,
        page.seat_lifecycle_rows,
        page.impact_previews,
        page.rollout_rings,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        OrganizationAdminTruthQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|defect| {
        defect.narrow_reason
            == OrganizationAdminTruthNarrowReasonClass::BoundaryVisibilityIncomplete
    }));
}

#[test]
fn local_safety_gap_withdraws_page() {
    let mut page = page();
    page.seat_lifecycle_rows[0].export_offboarding_available = false;
    let page = OrganizationAdminTruthPage::new(
        "test:local-safety",
        "local safety gap",
        "2026-06-01T00:00:00Z",
        page.overview,
        page.provider_cards,
        page.seat_lifecycle_rows,
        page.impact_previews,
        page.rollout_rings,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        OrganizationAdminTruthQualificationClass::Withdrawn.as_str()
    );
    assert!(page.defects.iter().any(|defect| {
        defect.narrow_reason == OrganizationAdminTruthNarrowReasonClass::LocalSafetyGuaranteeMissing
    }));
}

#[test]
fn raw_private_material_withdraws_page() {
    let mut page = page();
    page.rollout_rings[0].raw_private_material_excluded = false;
    let page = OrganizationAdminTruthPage::new(
        "test:raw-private-material",
        "raw private material",
        "2026-06-01T00:00:00Z",
        page.overview,
        page.provider_cards,
        page.seat_lifecycle_rows,
        page.impact_previews,
        page.rollout_rings,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        OrganizationAdminTruthQualificationClass::Withdrawn.as_str()
    );
    assert!(page.defects.iter().any(|defect| {
        defect.narrow_reason == OrganizationAdminTruthNarrowReasonClass::RawPrivateMaterialExposed
    }));
}
