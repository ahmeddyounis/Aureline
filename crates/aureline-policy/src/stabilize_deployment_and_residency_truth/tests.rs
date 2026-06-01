use super::*;

fn page() -> DeploymentResidencyStabilizePage {
    seeded_deployment_residency_stabilize_page()
}

#[test]
fn seeded_page_seeds_zero_defects_and_qualifies_stable() {
    let page = page();
    assert_eq!(page.defects.len(), 0);
    assert!(validate_deployment_residency_stabilize_page(&page).is_ok());
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        DeploymentResidencyStabilizeQualificationClass::Stable.as_str()
    );
}

#[test]
fn seeded_page_passes_all_five_stability_conditions() {
    let page = page();
    assert!(page.vocabulary_is_consistent());
    assert!(page.residual_dependency_ledger_is_complete());
    assert!(page.plane_separation_is_enforced());
    assert!(page.mirror_artifact_rows_are_present());
    assert!(page.sign_out_scope_is_declared());
}

#[test]
fn seeded_page_rows_are_all_stable() {
    let page = page();
    assert!(!page.rows.is_empty());
    for row in &page.rows {
        assert_eq!(
            row.qualification_token,
            DeploymentResidencyStabilizeQualificationClass::Stable.as_str(),
            "row '{}' must qualify stable; got '{}'",
            row.profile_token,
            row.qualification_token
        );
        assert_eq!(
            row.narrow_reason_token,
            DeploymentResidencyStabilizeNarrowReasonClass::NotNarrowed.as_str(),
            "row '{}' must have not_narrowed reason",
            row.profile_token
        );
    }
}

#[test]
fn seeded_page_rows_cover_all_five_profiles() {
    let page = page();
    let profiles: Vec<&str> = page.rows.iter().map(|r| r.profile_token.as_str()).collect();
    assert!(profiles.contains(&"individual_local"), "missing individual_local row");
    assert!(profiles.contains(&"managed_cloud"), "missing managed_cloud row");
    assert!(profiles.contains(&"enterprise_online"), "missing enterprise_online row");
    assert!(profiles.contains(&"self_hosted"), "missing self_hosted row");
    assert!(profiles.contains(&"air_gapped"), "missing air_gapped row");
}

#[test]
fn seeded_page_summary_counts_match_rows() {
    let page = page();
    assert_eq!(page.summary.row_count, page.rows.len());
    assert_eq!(page.summary.stable_row_count, page.rows.len());
    assert_eq!(page.summary.beta_row_count, 0);
    assert_eq!(page.summary.preview_row_count, 0);
    assert_eq!(page.summary.withdrawn_row_count, 0);
    assert_eq!(page.summary.defect_count, 0);
}

#[test]
fn seeded_page_plane_strips_all_separated() {
    let page = page();
    assert_eq!(
        page.summary.plane_strip_count,
        page.summary.plane_strips_separated_count,
        "all strips must have plane separation verified"
    );
}

#[test]
fn air_gapped_row_has_mirror_artifact_rows() {
    let page = page();
    let row = page
        .rows
        .iter()
        .find(|r| r.profile_token == "air_gapped")
        .expect("air_gapped row must be present");
    assert!(row.mirror_artifact_row_count > 0, "air_gapped must carry mirror artifact rows");
    assert!(
        row.sovereignty_claim_evidenced,
        "air_gapped must carry sovereignty evidence"
    );
}

#[test]
fn self_hosted_row_has_sovereignty_evidence() {
    let page = page();
    let row = page
        .rows
        .iter()
        .find(|r| r.profile_token == "self_hosted")
        .expect("self_hosted row must be present");
    assert!(
        row.sovereignty_claim_evidenced,
        "self_hosted must carry sovereignty evidence"
    );
}

#[test]
fn individual_local_row_requires_no_residual_deps() {
    let page = page();
    let row = page
        .rows
        .iter()
        .find(|r| r.profile_token == "individual_local")
        .expect("individual_local row must be present");
    assert_eq!(
        row.residual_dependency_row_count,
        0,
        "individual_local must carry zero residual dependency rows"
    );
    assert_eq!(
        row.qualification_token,
        DeploymentResidencyStabilizeQualificationClass::Stable.as_str(),
        "individual_local must still qualify stable"
    );
}

#[test]
fn support_export_wraps_clean_page() {
    let page = page();
    let export = DeploymentResidencyStabilizeSupportExport::from_page(
        "policy:stabilize-export:deployment-residency:stable:0001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
    assert_eq!(export.page.summary.withdrawn_row_count, 0);
}

#[test]
fn vocabulary_inconsistent_narrows_to_beta() {
    let mut input = seeded_deployment_residency_input();
    input.vocabulary_consistent_across_surfaces = false;
    let page = DeploymentResidencyStabilizePage::new(
        "policy:deployment_residency_stabilize:test",
        "test page",
        "2026-06-01T00:00:00Z",
        input,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        DeploymentResidencyStabilizeQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason
            == DeploymentResidencyStabilizeNarrowReasonClass::VocabularyInconsistentAcrossSurfaces
    }));
}

#[test]
fn incomplete_residual_dep_ledger_narrows_to_beta() {
    let mut input = seeded_deployment_residency_input();
    // Mark managed_cloud as not covering vendor services.
    if let Some(row) = input
        .profile_rows
        .iter_mut()
        .find(|r| r.profile_class == DeploymentProfileClass::ManagedCloud)
    {
        row.residual_deps_cover_vendor_services = false;
    }
    let page = DeploymentResidencyStabilizePage::new(
        "policy:deployment_residency_stabilize:test",
        "test page",
        "2026-06-01T00:00:00Z",
        input,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        DeploymentResidencyStabilizeQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason
            == DeploymentResidencyStabilizeNarrowReasonClass::ResidualDependencyLedgerIncomplete
    }));
}

#[test]
fn missing_plane_separation_narrows_to_beta() {
    let mut input = seeded_deployment_residency_input();
    if let Some(strip) = input.plane_status_strips.first_mut() {
        strip.control_data_plane_separated = false;
    }
    let page = DeploymentResidencyStabilizePage::new(
        "policy:deployment_residency_stabilize:test",
        "test page",
        "2026-06-01T00:00:00Z",
        input,
    );
    assert!(!page.qualifies_stable());
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == DeploymentResidencyStabilizeNarrowReasonClass::PlaneSeparationMissing
    }));
}

#[test]
fn missing_continue_local_path_narrows_to_beta() {
    let mut input = seeded_deployment_residency_input();
    if let Some(strip) = input.plane_status_strips.first_mut() {
        strip.continue_local_path_preserved = false;
    }
    let page = DeploymentResidencyStabilizePage::new(
        "policy:deployment_residency_stabilize:test",
        "test page",
        "2026-06-01T00:00:00Z",
        input,
    );
    assert!(!page.qualifies_stable());
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == DeploymentResidencyStabilizeNarrowReasonClass::PlaneSeparationMissing
    }));
}

#[test]
fn absent_mirror_artifact_rows_narrows_to_beta() {
    let mut input = seeded_deployment_residency_input();
    if let Some(row) = input
        .profile_rows
        .iter_mut()
        .find(|r| r.profile_class == DeploymentProfileClass::AirGapped)
    {
        row.mirror_artifact_row_count = 0;
    }
    let page = DeploymentResidencyStabilizePage::new(
        "policy:deployment_residency_stabilize:test",
        "test page",
        "2026-06-01T00:00:00Z",
        input,
    );
    assert!(!page.qualifies_stable());
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == DeploymentResidencyStabilizeNarrowReasonClass::MirrorArtifactRowsAbsent
    }));
}

#[test]
fn sign_out_scope_undeclared_narrows_to_beta() {
    let mut input = seeded_deployment_residency_input();
    if let Some(row) = input
        .profile_rows
        .iter_mut()
        .find(|r| r.profile_class == DeploymentProfileClass::ManagedCloud)
    {
        row.sign_out_scope_declared = false;
    }
    let page = DeploymentResidencyStabilizePage::new(
        "policy:deployment_residency_stabilize:test",
        "test page",
        "2026-06-01T00:00:00Z",
        input,
    );
    assert!(!page.qualifies_stable());
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == DeploymentResidencyStabilizeNarrowReasonClass::SignOutScopeUndeclared
    }));
}

#[test]
fn unevidenced_sovereignty_claim_withdraws_packet() {
    let mut input = seeded_deployment_residency_input();
    if let Some(row) = input
        .profile_rows
        .iter_mut()
        .find(|r| r.profile_class == DeploymentProfileClass::SelfHosted)
    {
        row.sovereignty_claim_evidenced = false;
    }
    let page = DeploymentResidencyStabilizePage::new(
        "policy:deployment_residency_stabilize:test",
        "test page",
        "2026-06-01T00:00:00Z",
        input,
    );
    assert!(!page.qualifies_stable());
    assert!(!page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        DeploymentResidencyStabilizeQualificationClass::Withdrawn.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason
            == DeploymentResidencyStabilizeNarrowReasonClass::ImpliedSovereigntyUnproven
    }));
}

#[test]
fn re_audit_matches_embedded_defects() {
    let page = page();
    let reaudited = audit_deployment_residency_stabilize_page(&page);
    assert_eq!(
        reaudited.len(),
        page.defects.len(),
        "re-audit must match embedded defects"
    );
}

#[test]
fn qualification_class_checks() {
    assert!(DeploymentResidencyStabilizeQualificationClass::Stable.is_stable());
    assert!(!DeploymentResidencyStabilizeQualificationClass::Beta.is_stable());
    assert!(!DeploymentResidencyStabilizeQualificationClass::Withdrawn.is_stable());
    assert!(DeploymentResidencyStabilizeQualificationClass::Stable.is_claimable());
    assert!(DeploymentResidencyStabilizeQualificationClass::Beta.is_claimable());
    assert!(!DeploymentResidencyStabilizeQualificationClass::Withdrawn.is_claimable());
    assert!(!DeploymentResidencyStabilizeQualificationClass::Preview.is_claimable());
}

#[test]
fn narrow_reason_withdrawal_sentinel_check() {
    assert!(
        DeploymentResidencyStabilizeNarrowReasonClass::ImpliedSovereigntyUnproven
            .is_withdrawal_reason()
    );
    assert!(
        !DeploymentResidencyStabilizeNarrowReasonClass::ResidualDependencyLedgerIncomplete
            .is_withdrawal_reason()
    );
    assert!(!DeploymentResidencyStabilizeNarrowReasonClass::NotNarrowed.is_withdrawal_reason());
}

#[test]
fn deployment_profile_class_helpers() {
    assert!(DeploymentProfileClass::IndividualLocal.is_local_only());
    assert!(!DeploymentProfileClass::ManagedCloud.is_local_only());
    assert!(!DeploymentProfileClass::IndividualLocal.requires_residual_dep_coverage());
    assert!(DeploymentProfileClass::ManagedCloud.requires_residual_dep_coverage());
    assert!(DeploymentProfileClass::SelfHosted.claims_sovereignty());
    assert!(DeploymentProfileClass::AirGapped.claims_sovereignty());
    assert!(!DeploymentProfileClass::ManagedCloud.claims_sovereignty());
    assert!(!DeploymentProfileClass::IndividualLocal.claims_sovereignty());
}

#[test]
fn mirror_offline_state_class_helpers() {
    assert!(MirrorOfflineStateClass::OnlineMirrorOnly.requires_mirror_artifact_rows());
    assert!(MirrorOfflineStateClass::OfflineAirGapped.requires_mirror_artifact_rows());
    assert!(!MirrorOfflineStateClass::OnlineLiveAllowed.requires_mirror_artifact_rows());
    assert!(!MirrorOfflineStateClass::NotApplicable.requires_mirror_artifact_rows());
}

#[test]
fn tenant_org_scope_class_helpers() {
    assert!(TenantOrgScopeClass::CustomerTenant.requires_sign_out_scope_declaration());
    assert!(TenantOrgScopeClass::SharedMultiTenant.requires_sign_out_scope_declaration());
    assert!(!TenantOrgScopeClass::SingleUserLocal.requires_sign_out_scope_declaration());
    assert!(!TenantOrgScopeClass::NotApplicable.requires_sign_out_scope_declaration());
}
