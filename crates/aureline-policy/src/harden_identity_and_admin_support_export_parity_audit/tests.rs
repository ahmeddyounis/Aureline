use super::*;

// ---------------------------------------------------------------------------
// Seeded page invariants
// ---------------------------------------------------------------------------

#[test]
fn seeded_page_qualifies_stable() {
    let page = seeded_harden_identity_admin_page();
    assert!(
        page.qualifies_stable(),
        "seeded page must qualify stable; got '{}'; defects: {:?}",
        page.summary.overall_qualification_token,
        page.defects
    );
}

#[test]
fn seeded_page_has_zero_defects() {
    let page = seeded_harden_identity_admin_page();
    assert!(
        page.defects.is_empty(),
        "seeded page must have zero defects; got: {:?}",
        page.defects
    );
}

#[test]
fn seeded_page_has_five_rows() {
    let page = seeded_harden_identity_admin_page();
    assert_eq!(
        page.rows.len(),
        5,
        "seeded page must have 5 rows (one per required row class)"
    );
}

#[test]
fn seeded_page_covers_all_row_classes() {
    let page = seeded_harden_identity_admin_page();
    assert!(
        page.covers_all_required_row_classes(),
        "seeded page must cover all five required row classes"
    );
}

#[test]
fn seeded_page_all_rows_are_stable() {
    let page = seeded_harden_identity_admin_page();
    for row in &page.rows {
        assert_eq!(
            row.qualification_token, "stable",
            "row '{}' must qualify stable in the seeded page",
            row.row_id
        );
    }
}

#[test]
fn seeded_page_all_rows_exclude_raw_secret_material() {
    let page = seeded_harden_identity_admin_page();
    assert!(
        page.all_rows_exclude_raw_secret_material(),
        "all seeded rows must exclude raw secret or private-key material"
    );
}

#[test]
fn seeded_page_all_required_provisioning_classes_declared() {
    let page = seeded_harden_identity_admin_page();
    assert!(
        page.all_required_provisioning_classes_declared(),
        "all rows requiring a provisioning class must declare one"
    );
}

#[test]
fn seeded_page_all_rows_have_sync_freshness() {
    let page = seeded_harden_identity_admin_page();
    assert!(
        page.all_rows_have_sync_freshness(),
        "all seeded rows must carry a sync freshness token"
    );
}

#[test]
fn seeded_page_all_rows_have_local_tenant_scope() {
    let page = seeded_harden_identity_admin_page();
    assert!(
        page.all_rows_have_local_tenant_scope(),
        "all seeded rows must carry a local vs tenant scope token"
    );
}

#[test]
fn seeded_page_directory_provider_card_has_fallback_path() {
    let page = seeded_harden_identity_admin_page();
    let row = page
        .rows
        .iter()
        .find(|r| r.row_class == IdentityAdminRowClass::DirectoryProviderCard)
        .expect("seeded page must have a directory_provider_card row");
    assert!(
        !row.fallback_manual_path_label.is_empty(),
        "directory_provider_card row must name a fallback or manual path"
    );
}

#[test]
fn seeded_page_local_governance_path_has_local_core_continuity() {
    let page = seeded_harden_identity_admin_page();
    let row = page
        .rows
        .iter()
        .find(|r| r.row_class == IdentityAdminRowClass::LocalGovernancePath)
        .expect("seeded page must have a local_governance_path row");
    assert!(
        row.local_core_continuity_explicit,
        "local_governance_path row must carry local_core_continuity_explicit: true"
    );
}

#[test]
fn seeded_page_provisioning_failure_log_has_specific_failure_kind() {
    let page = seeded_harden_identity_admin_page();
    let row = page
        .rows
        .iter()
        .find(|r| r.row_class == IdentityAdminRowClass::ProvisioningFailureLog)
        .expect("seeded page must have a provisioning_failure_log row");
    assert!(
        !row.failure_kind_token.is_empty(),
        "provisioning_failure_log row must name a specific failure kind"
    );
    assert_eq!(
        row.failure_kind,
        Some(ProvisioningFailureKind::ProviderOutage),
        "provisioning_failure_log row failure kind must be provider_outage"
    );
}

#[test]
fn seeded_page_user_seat_lifecycle_has_action_lineage() {
    let page = seeded_harden_identity_admin_page();
    let row = page
        .rows
        .iter()
        .find(|r| r.row_class == IdentityAdminRowClass::UserSeatLifecycle)
        .expect("seeded page must have a user_seat_lifecycle row");
    assert!(
        row.action_lineage.is_some(),
        "user_seat_lifecycle row must carry an admin action/result lineage block"
    );
    let lineage = row.action_lineage.as_ref().unwrap();
    assert_eq!(lineage.action_token, "seat_transfer");
    assert_eq!(lineage.result_token, "succeeded");
}

#[test]
fn seeded_page_policy_target_dry_run_is_hybrid_scope() {
    let page = seeded_harden_identity_admin_page();
    let row = page
        .rows
        .iter()
        .find(|r| r.row_class == IdentityAdminRowClass::PolicyTargetDryRun)
        .expect("seeded page must have a policy_target_dry_run row");
    assert_eq!(
        row.local_tenant_scope,
        LocalTenantScopeClass::HybridLocalTenant,
        "policy_target_dry_run row must be hybrid_local_tenant scope"
    );
}

#[test]
fn seeded_page_local_governance_path_is_local_only_scope() {
    let page = seeded_harden_identity_admin_page();
    let row = page
        .rows
        .iter()
        .find(|r| r.row_class == IdentityAdminRowClass::LocalGovernancePath)
        .expect("seeded page must have a local_governance_path row");
    assert_eq!(
        row.local_tenant_scope,
        LocalTenantScopeClass::LocalStateOnly,
        "local_governance_path row must be local_state_only scope"
    );
}

// ---------------------------------------------------------------------------
// Audit: raw secret material triggers withdrawal
// ---------------------------------------------------------------------------

#[test]
fn raw_secret_in_row_triggers_withdrawal() {
    let mut rows = seeded_rows();
    if let Some(row) = rows.first_mut() {
        row.raw_secret_or_private_material_excluded = false;
    }
    let page = HardenIdentityAdminPage::new(
        "test:withdrawal:raw-secret",
        "Test: raw secret material in row",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        HardenIdentityAdminQualificationClass::Withdrawn.as_str(),
        "raw secret in row must withdraw the packet"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason
                == HardenIdentityAdminNarrowReasonClass::RawSecretOrPrivateMaterialExposed
        }),
        "defects must include raw_secret_or_private_material_exposed"
    );
}

// ---------------------------------------------------------------------------
// Audit: missing row class triggers preview narrowing
// ---------------------------------------------------------------------------

#[test]
fn missing_row_class_narrows_to_preview() {
    // Only include directory_provider_card and user_seat_lifecycle rows.
    let rows: Vec<IdentityAdminRow> = seeded_rows()
        .into_iter()
        .filter(|r| {
            r.row_class == IdentityAdminRowClass::DirectoryProviderCard
                || r.row_class == IdentityAdminRowClass::UserSeatLifecycle
        })
        .collect();
    let page = HardenIdentityAdminPage::new(
        "test:preview:missing-row-class",
        "Test: required row class missing",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        HardenIdentityAdminQualificationClass::Preview.as_str(),
        "missing required row class must narrow to preview"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason == HardenIdentityAdminNarrowReasonClass::MissingRowClassCoverage
        }),
        "defects must include missing_row_class_coverage"
    );
}

// ---------------------------------------------------------------------------
// Audit: empty provisioning class narrows to beta
// ---------------------------------------------------------------------------

#[test]
fn empty_provisioning_class_narrows_to_beta() {
    let mut rows = seeded_rows();
    if let Some(row) = rows
        .iter_mut()
        .find(|r| r.row_class == IdentityAdminRowClass::DirectoryProviderCard)
    {
        row.provisioning_class_token.clear();
    }
    let page = HardenIdentityAdminPage::new(
        "test:beta:empty-provisioning-class",
        "Test: empty provisioning class",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert!(
        page.summary.overall_qualification_token
            == HardenIdentityAdminQualificationClass::Beta.as_str()
            || page.summary.overall_qualification_token
                == HardenIdentityAdminQualificationClass::Withdrawn.as_str(),
        "empty provisioning class must narrow to at least beta"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason == HardenIdentityAdminNarrowReasonClass::MissingProvisioningClass
        }),
        "defects must include missing_provisioning_class"
    );
}

// ---------------------------------------------------------------------------
// Audit: missing local governance continuity narrows to beta
// ---------------------------------------------------------------------------

#[test]
fn missing_local_governance_continuity_narrows_to_beta() {
    let mut rows = seeded_rows();
    if let Some(row) = rows
        .iter_mut()
        .find(|r| r.row_class == IdentityAdminRowClass::LocalGovernancePath)
    {
        row.local_core_continuity_explicit = false;
    }
    let page = HardenIdentityAdminPage::new(
        "test:beta:missing-local-governance-continuity",
        "Test: missing local governance continuity",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert!(
        page.summary.overall_qualification_token
            == HardenIdentityAdminQualificationClass::Beta.as_str()
            || page.summary.overall_qualification_token
                == HardenIdentityAdminQualificationClass::Withdrawn.as_str(),
        "missing local governance continuity must narrow to at least beta"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason == HardenIdentityAdminNarrowReasonClass::LocalCoreContinuityNotExplicit
        }),
        "defects must include local_core_continuity_not_explicit"
    );
}

// ---------------------------------------------------------------------------
// Audit: missing directory provider fallback narrows to beta
// ---------------------------------------------------------------------------

#[test]
fn missing_directory_provider_fallback_narrows_to_beta() {
    let mut rows = seeded_rows();
    if let Some(row) = rows
        .iter_mut()
        .find(|r| r.row_class == IdentityAdminRowClass::DirectoryProviderCard)
    {
        row.fallback_manual_path_label.clear();
    }
    let page = HardenIdentityAdminPage::new(
        "test:beta:missing-directory-fallback",
        "Test: missing directory provider fallback path",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert!(
        page.summary.overall_qualification_token
            == HardenIdentityAdminQualificationClass::Beta.as_str()
            || page.summary.overall_qualification_token
                == HardenIdentityAdminQualificationClass::Withdrawn.as_str(),
        "missing directory provider fallback must narrow to at least beta"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason == HardenIdentityAdminNarrowReasonClass::MissingFallbackManualPath
        }),
        "defects must include missing_fallback_manual_path"
    );
}

// ---------------------------------------------------------------------------
// Audit: generic failure kind narrows to beta
// ---------------------------------------------------------------------------

#[test]
fn generic_failure_kind_narrows_to_beta() {
    let mut rows = seeded_rows();
    if let Some(row) = rows
        .iter_mut()
        .find(|r| r.row_class == IdentityAdminRowClass::ProvisioningFailureLog)
    {
        row.failure_kind_token = "generic_error".to_owned();
        row.failure_kind = None;
    }
    let page = HardenIdentityAdminPage::new(
        "test:beta:generic-failure-kind",
        "Test: generic failure kind",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert!(
        page.summary.overall_qualification_token
            == HardenIdentityAdminQualificationClass::Beta.as_str()
            || page.summary.overall_qualification_token
                == HardenIdentityAdminQualificationClass::Withdrawn.as_str(),
        "generic failure kind must narrow to at least beta"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason == HardenIdentityAdminNarrowReasonClass::GenericFailureKindUsed
        }),
        "defects must include generic_failure_kind_used"
    );
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

#[test]
fn support_export_always_excludes_raw_private_material() {
    let page = seeded_harden_identity_admin_page();
    let export = HardenIdentityAdminSupportExport::from_page(
        "export:test:001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(
        export.raw_secret_or_private_material_excluded,
        "support export must always report raw_secret_or_private_material_excluded: true"
    );
}

#[test]
fn support_export_has_correct_record_kind() {
    let page = seeded_harden_identity_admin_page();
    let export = HardenIdentityAdminSupportExport::from_page(
        "export:test:002",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert_eq!(
        export.record_kind,
        HARDEN_IDENTITY_ADMIN_SUPPORT_EXPORT_RECORD_KIND
    );
}

// ---------------------------------------------------------------------------
// Re-audit helpers are consistent
// ---------------------------------------------------------------------------

#[test]
fn validate_returns_ok_for_seeded_page() {
    let page = seeded_harden_identity_admin_page();
    let result = validate_harden_identity_admin_page(&page);
    assert!(
        result.is_ok(),
        "validate must return Ok for the seeded page; defects: {:?}",
        result.err()
    );
}

#[test]
fn audit_returns_zero_defects_for_seeded_page() {
    let page = seeded_harden_identity_admin_page();
    let defects = audit_harden_identity_admin_page(&page);
    assert!(
        defects.is_empty(),
        "re-audit of seeded page must return zero defects; got: {:?}",
        defects
    );
}

// ---------------------------------------------------------------------------
// Scope invariants
// ---------------------------------------------------------------------------

#[test]
fn seeded_page_has_all_three_scopes_represented() {
    let page = seeded_harden_identity_admin_page();
    let scopes: BTreeSet<&str> = page
        .rows
        .iter()
        .map(|r| r.local_tenant_scope_token.as_str())
        .collect();
    assert!(scopes.contains("local_state_only"));
    assert!(scopes.contains("tenant_scoped"));
    assert!(scopes.contains("hybrid_local_tenant"));
}

#[test]
fn seeded_page_has_all_four_provisioning_classes_represented() {
    let page = seeded_harden_identity_admin_page();
    let classes: BTreeSet<&str> = page
        .rows
        .iter()
        .map(|r| r.provisioning_class_token.as_str())
        .collect();
    assert!(classes.contains("oidc"));
    assert!(classes.contains("scim"));
    assert!(classes.contains("signed_file_bundle"));
    assert!(classes.contains("manual"));
}
