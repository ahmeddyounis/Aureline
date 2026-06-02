use super::*;

// ---------------------------------------------------------------------------
// Seeded page invariants
// ---------------------------------------------------------------------------

#[test]
fn seeded_page_qualifies_stable() {
    let page = seeded_backup_restore_failover_page();
    assert!(
        page.qualifies_stable(),
        "seeded page must qualify stable; got '{}'; defects: {:?}",
        page.summary.overall_qualification_token,
        page.defects
    );
}

#[test]
fn seeded_page_has_zero_defects() {
    let page = seeded_backup_restore_failover_page();
    assert!(
        page.defects.is_empty(),
        "seeded page must have zero defects; got: {:?}",
        page.defects
    );
}

#[test]
fn seeded_page_has_five_rows() {
    let page = seeded_backup_restore_failover_page();
    assert_eq!(
        page.rows.len(),
        5,
        "seeded page must have 5 rows (one per enterprise profile)"
    );
}

#[test]
fn seeded_page_covers_all_enterprise_profiles() {
    let page = seeded_backup_restore_failover_page();
    assert!(
        page.covers_all_required_profiles(),
        "seeded page must cover all five required enterprise profiles"
    );
}

#[test]
fn seeded_page_all_rows_are_stable() {
    let page = seeded_backup_restore_failover_page();
    for row in &page.rows {
        assert_eq!(
            row.qualification_token, "stable",
            "row '{}' must qualify stable in the seeded page",
            row.row_id
        );
    }
}

#[test]
fn seeded_page_all_rows_preserve_local_core() {
    let page = seeded_backup_restore_failover_page();
    assert!(
        page.all_rows_preserve_local_core(),
        "all seeded rows must carry a non-blocking local-core continuity posture"
    );
}

#[test]
fn seeded_page_no_row_blocks_local_core_by_failover() {
    let page = seeded_backup_restore_failover_page();
    assert!(
        page.no_row_blocks_local_core_by_failover(),
        "no seeded row may declare a failover behavior that blocks local-core work"
    );
}

#[test]
fn seeded_page_no_withdrawn_rows() {
    let page = seeded_backup_restore_failover_page();
    assert!(
        page.no_withdrawn_rows(),
        "seeded page must have zero withdrawn rows"
    );
}

#[test]
fn seeded_page_individual_local_row_is_not_applicable() {
    let page = seeded_backup_restore_failover_page();
    let local_row = page
        .rows
        .iter()
        .find(|r| r.enterprise_profile == EnterpriseProfileClass::IndividualLocal)
        .expect("seeded page must contain an individual_local row");
    assert_eq!(
        local_row.backup.backup_state,
        BackupStateClass::NotApplicable,
        "individual_local row must declare backup_state: not_applicable"
    );
    assert_eq!(
        local_row.restore.restore_test_posture,
        RestoreTestPostureClass::NotApplicable,
        "individual_local row must declare restore_test_posture: not_applicable"
    );
    assert_eq!(
        local_row.failover_continuity.failover_behavior,
        FailoverBehaviorClass::NotApplicable,
        "individual_local row must declare failover_behavior: not_applicable"
    );
}

#[test]
fn seeded_page_enterprise_rows_have_tenant_region_and_policy_source() {
    let page = seeded_backup_restore_failover_page();
    for row in page
        .rows
        .iter()
        .filter(|r| !r.enterprise_profile.is_local_only())
    {
        assert!(
            !row.tenant_region_owner_ref.is_empty(),
            "enterprise row '{}' must carry a tenant_region_owner_ref",
            row.row_id
        );
        assert!(
            !row.policy_source_ref.is_empty(),
            "enterprise row '{}' must carry a policy_source_ref",
            row.row_id
        );
        assert!(
            !row.dependency_class_token.is_empty(),
            "enterprise row '{}' must carry a dependency_class_token",
            row.row_id
        );
    }
}

#[test]
fn seeded_page_local_core_preserved_count_equals_five() {
    let page = seeded_backup_restore_failover_page();
    assert_eq!(
        page.summary.local_core_preserved_row_count, 5,
        "all 5 rows must carry a preserved local-core continuity posture"
    );
}

// ---------------------------------------------------------------------------
// Audit: local-core blocked by failover triggers withdrawal
// ---------------------------------------------------------------------------

#[test]
fn local_core_blocked_by_failover_triggers_withdrawal() {
    let mut rows = vec![row_self_hosted()];
    rows[0].failover_continuity.failover_behavior = FailoverBehaviorClass::LocalCoreBlocked;
    rows[0].failover_continuity.failover_behavior_token =
        FailoverBehaviorClass::LocalCoreBlocked.as_str().to_owned();
    let page = BackupRestoreFailoverPage::new(
        "test:withdrawal:local-core-blocked",
        "Test: local core blocked by failover",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        BackupRestoreFailoverQualificationClass::Withdrawn.as_str(),
        "local_core_blocked failover behavior must withdraw the packet"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason == BackupRestoreFailoverNarrowReasonClass::LocalCoreBlockedByFailover
        }),
        "defects must include local_core_blocked_by_failover"
    );
}

#[test]
fn local_core_posture_blocked_by_default_triggers_withdrawal() {
    let mut rows = vec![row_enterprise_online()];
    rows[0].failover_continuity.local_core_posture =
        LocalCoreContinuityPostureClass::BlockedByDefault;
    rows[0].failover_continuity.local_core_posture_token =
        LocalCoreContinuityPostureClass::BlockedByDefault
            .as_str()
            .to_owned();
    let page = BackupRestoreFailoverPage::new(
        "test:withdrawal:local-core-posture-blocked",
        "Test: local core posture blocked by default",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        BackupRestoreFailoverQualificationClass::Withdrawn.as_str(),
        "blocked_by_default local-core posture must withdraw the packet"
    );
}

// ---------------------------------------------------------------------------
// Audit: missing enterprise profile triggers preview narrowing
// ---------------------------------------------------------------------------

#[test]
fn missing_profile_narrows_to_preview() {
    let rows = vec![row_individual_local()];
    let page = BackupRestoreFailoverPage::new(
        "test:preview:missing-profile",
        "Test: missing enterprise profiles",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        BackupRestoreFailoverQualificationClass::Preview.as_str(),
        "missing enterprise profiles must narrow to preview"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason == BackupRestoreFailoverNarrowReasonClass::ProfileCoverageGap
        }),
        "defects must include profile_coverage_gap"
    );
}

// ---------------------------------------------------------------------------
// Audit: backup state deficiencies narrow to beta
// ---------------------------------------------------------------------------

#[test]
fn unverified_backup_state_narrows_to_beta() {
    let mut rows = seeded_rows();
    rows[1].backup.backup_state = BackupStateClass::Unverified;
    rows[1].backup.backup_state_token = BackupStateClass::Unverified.as_str().to_owned();
    let page = BackupRestoreFailoverPage::new(
        "test:beta:unverified-backup",
        "Test: unverified backup state",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert!(
        page.summary.overall_qualification_token
            == BackupRestoreFailoverQualificationClass::Beta.as_str()
            || page.summary.overall_qualification_token
                == BackupRestoreFailoverQualificationClass::Withdrawn.as_str(),
        "unverified backup state must narrow to at least beta"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason == BackupRestoreFailoverNarrowReasonClass::BackupStateUnverified
        }),
        "defects must include backup_state_unverified"
    );
}

#[test]
fn restore_never_drilled_narrows_to_beta() {
    let mut rows = seeded_rows();
    rows[2].restore.restore_test_posture = RestoreTestPostureClass::NeverTested;
    rows[2].restore.restore_test_posture_token =
        RestoreTestPostureClass::NeverTested.as_str().to_owned();
    let page = BackupRestoreFailoverPage::new(
        "test:beta:restore-never-drilled",
        "Test: restore never drilled",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert!(
        page.summary.overall_qualification_token
            == BackupRestoreFailoverQualificationClass::Beta.as_str()
            || page.summary.overall_qualification_token
                == BackupRestoreFailoverQualificationClass::Withdrawn.as_str(),
        "restore never drilled must narrow to at least beta"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason == BackupRestoreFailoverNarrowReasonClass::RestoreNeverDrilled
        }),
        "defects must include restore_never_drilled"
    );
}

// ---------------------------------------------------------------------------
// Audit: missing tenant/region ref narrows to beta
// ---------------------------------------------------------------------------

#[test]
fn missing_tenant_region_ref_narrows_to_beta() {
    let mut rows = seeded_rows();
    rows[3].tenant_region_owner_ref.clear();
    let page = BackupRestoreFailoverPage::new(
        "test:beta:missing-tenant-region",
        "Test: missing tenant/region ownership ref",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert!(
        page.summary.overall_qualification_token
            == BackupRestoreFailoverQualificationClass::Beta.as_str()
            || page.summary.overall_qualification_token
                == BackupRestoreFailoverQualificationClass::Withdrawn.as_str(),
        "missing tenant/region ref must narrow to at least beta"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason
                == BackupRestoreFailoverNarrowReasonClass::TenantRegionOwnershipNotDeclared
        }),
        "defects must include tenant_region_ownership_not_declared"
    );
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

#[test]
fn support_export_always_excludes_raw_private_material() {
    let page = seeded_backup_restore_failover_page();
    let export = BackupRestoreFailoverSupportExport::from_page(
        "export:test:001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(
        export.raw_private_material_excluded,
        "support export must always report raw_private_material_excluded: true"
    );
}

#[test]
fn support_export_has_correct_record_kind() {
    let page = seeded_backup_restore_failover_page();
    let export = BackupRestoreFailoverSupportExport::from_page(
        "export:test:002",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert_eq!(
        export.record_kind,
        BACKUP_RESTORE_FAILOVER_SUPPORT_EXPORT_RECORD_KIND
    );
}

// ---------------------------------------------------------------------------
// Re-audit helpers are consistent
// ---------------------------------------------------------------------------

#[test]
fn validate_returns_ok_for_seeded_page() {
    let page = seeded_backup_restore_failover_page();
    let result = validate_backup_restore_failover_page(&page);
    assert!(
        result.is_ok(),
        "validate must return Ok for the seeded page; defects: {:?}",
        result.err()
    );
}

#[test]
fn audit_returns_zero_defects_for_seeded_page() {
    let page = seeded_backup_restore_failover_page();
    let defects = audit_backup_restore_failover_page(&page);
    assert!(
        defects.is_empty(),
        "re-audit of seeded page must return zero defects; got: {:?}",
        defects
    );
}
