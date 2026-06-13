use super::*;

fn page() -> DeploymentProfileContinuityPage {
    seeded_deployment_profile_continuity_page()
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
    assert!(validate_deployment_profile_continuity_page(&page).is_ok());
}

#[test]
fn seeded_page_covers_all_claimed_profiles() {
    let page = page();
    assert!(page.covers_claimed_profiles());
    assert_eq!(page.input.claimed_profiles.len(), 5);
    assert_eq!(page.summary.claimed_profile_count, 5);
    assert_eq!(page.summary.deployment_summary_card_count, 5);
    assert_eq!(page.summary.local_safe_fallback_card_count, 5);
}

#[test]
fn seeded_page_exposes_residual_dependencies_and_mirror_truth() {
    let page = page();
    assert!(page.residual_vendor_dependencies_disclosed());
    assert!(page.mirror_freshness_truth_complete());
    assert!(page.local_safe_fallback_truth_complete());
    assert!(page.all_required_surfaces_reuse_facts());
}

#[test]
fn seeded_page_current_profile_is_self_hosted_and_mirror_backed() {
    let page = page();
    assert_eq!(page.summary.current_profile_token, "self_hosted");
    let self_hosted = page
        .input
        .deployment_summary_cards
        .iter()
        .find(|card| card.profile == DeploymentProfileClass::SelfHosted)
        .expect("self_hosted card");
    assert_eq!(
        self_hosted.mirror_offline_state,
        MirrorOfflineStateClass::OnlineMirrorOnly
    );
    assert_eq!(self_hosted.vendor_dependency_refs.len(), 2);
}

#[test]
fn support_export_wraps_seeded_page_without_raw_private_material() {
    let export = DeploymentProfileContinuitySupportExport::from_page(
        "policy:deployment-profile-continuity:support-export:fixture-001",
        "2026-06-01T00:00:00Z",
        page(),
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
}

#[test]
fn hidden_self_hosted_vendor_dependency_triggers_withdrawal() {
    let mut input = seeded_deployment_profile_continuity_input();
    input
        .residual_dependency_rows
        .retain(|row| row.dependency_ref != "dependency:self-hosted:model-gateway");
    let page = DeploymentProfileContinuityPage::new(
        "test:hidden-self-hosted-dependency",
        "Hidden self-hosted dependency",
        "2026-06-01T00:00:00Z",
        input,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        DeploymentProfileContinuityQualificationClass::Withdrawn.as_str()
    );
    assert!(page.defects.iter().any(|defect| {
        defect.narrow_reason
            == DeploymentProfileContinuityNarrowReasonClass::SovereignBoundaryOverclaimed
    }));
}

#[test]
fn missing_air_gapped_mirror_freshness_narrows_to_beta() {
    let mut input = seeded_deployment_profile_continuity_input();
    input
        .mirror_freshness_cards
        .retain(|card| card.profile != DeploymentProfileClass::AirGapped);
    let page = DeploymentProfileContinuityPage::new(
        "test:missing-airgapped-freshness",
        "Missing air-gapped freshness",
        "2026-06-01T00:00:00Z",
        input,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        DeploymentProfileContinuityQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|defect| {
        defect.narrow_reason
            == DeploymentProfileContinuityNarrowReasonClass::MirrorFreshnessTruthMissing
    }));
}

#[test]
fn missing_air_gapped_local_safe_card_narrows_to_preview() {
    let mut input = seeded_deployment_profile_continuity_input();
    input
        .local_safe_fallback_cards
        .retain(|card| card.profile != DeploymentProfileClass::AirGapped);
    let page = DeploymentProfileContinuityPage::new(
        "test:missing-airgapped-fallback",
        "Missing air-gapped local-safe fallback",
        "2026-06-01T00:00:00Z",
        input,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        DeploymentProfileContinuityQualificationClass::Preview.as_str()
    );
    assert!(page.defects.iter().any(|defect| {
        defect.narrow_reason == DeploymentProfileContinuityNarrowReasonClass::MissingProfileCoverage
    }));
}

#[test]
fn incomplete_surface_reuse_narrows_to_beta() {
    let mut input = seeded_deployment_profile_continuity_input();
    let row = input
        .surface_reuse_rows
        .iter_mut()
        .find(|row| row.fact_family == FactFamilyClass::ResidualDependency)
        .expect("residual dependency surface reuse row");
    row.surface_visibility.help = false;

    let page = DeploymentProfileContinuityPage::new(
        "test:surface-reuse-gap",
        "Surface reuse gap",
        "2026-06-01T00:00:00Z",
        input,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        DeploymentProfileContinuityQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|defect| {
        defect.narrow_reason
            == DeploymentProfileContinuityNarrowReasonClass::SurfaceFactReuseMissing
    }));
}

#[test]
fn reauditing_seeded_page_returns_zero_defects() {
    let defects = audit_deployment_profile_continuity_page(&page());
    assert!(defects.is_empty(), "re-audit defects: {defects:?}");
}
