use super::*;

fn page() -> QualificationMatrixPage {
    seeded_qualification_matrix_page()
}

#[test]
fn seeded_page_produces_zero_defects_and_qualifies_stable() {
    let page = page();
    assert_eq!(
        page.defects.len(),
        0,
        "seeded page must be clean: {:?}",
        page.defects
    );
    assert!(validate_qualification_matrix_page(&page).is_ok());
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        QualificationTierClass::Stable.as_str()
    );
}

#[test]
fn seeded_page_passes_all_six_stability_conditions() {
    let page = page();
    assert!(
        page.covers_all_required_rows(),
        "all 22 required rows must be covered"
    );
    assert!(
        page.all_rows_declare_local_core_continuity(),
        "all rows must declare local-core continuity"
    );
    assert!(
        page.all_rows_have_typed_failure_downgrade(),
        "all rows must carry a typed failure-mode downgrade class"
    );
    assert!(
        page.all_no_account_rows_declare_compatibility(),
        "all local_oss, air_gapped, and accessibility rows must declare \
         no-account compatibility"
    );
}

#[test]
fn seeded_page_covers_all_required_surface_profile_pairs() {
    let page = page();
    let covered = page.snapshot.covered_row_keys();
    for (surface, profile) in &REQUIRED_SURFACE_PROFILE_PAIRS {
        let key = format!("{surface}:{profile}");
        assert!(
            covered.contains(key.as_str()),
            "required surface×profile row '{key}' must be covered"
        );
    }
}

#[test]
fn seeded_page_covers_all_required_accessibility_features() {
    let page = page();
    let covered = page.snapshot.covered_row_keys();
    for feature in &REQUIRED_ACCESSIBILITY_FEATURES {
        let key = format!("accessibility:{feature}");
        assert!(
            covered.contains(key.as_str()),
            "required accessibility row '{key}' must be covered"
        );
    }
}

#[test]
fn seeded_page_row_count_equals_required_row_count() {
    let page = page();
    assert_eq!(
        page.snapshot.records.len(),
        REQUIRED_ROW_COUNT,
        "snapshot must contain exactly {} records",
        REQUIRED_ROW_COUNT
    );
}

#[test]
fn seeded_page_all_matrix_rows_qualify_stable() {
    let page = page();
    assert!(!page.rows.is_empty(), "page must have at least one row");
    for row in &page.rows {
        assert_eq!(
            row.qualification_token,
            QualificationTierClass::Stable.as_str(),
            "row '{}' must qualify stable; got '{}'",
            row.row_key,
            row.qualification_token
        );
        assert_eq!(
            row.narrow_reason_token,
            NarrowReasonClass::NotNarrowed.as_str(),
            "row '{}' must have not_narrowed reason; got '{}'",
            row.row_key,
            row.narrow_reason_token
        );
    }
}

#[test]
fn seeded_page_summary_counts_are_consistent() {
    let page = page();
    assert_eq!(page.summary.row_count, page.rows.len());
    assert_eq!(page.summary.stable_row_count, page.rows.len());
    assert_eq!(page.summary.beta_row_count, 0);
    assert_eq!(page.summary.preview_row_count, 0);
    assert_eq!(page.summary.withdrawn_row_count, 0);
    assert_eq!(
        page.summary.local_core_continuity_declared_count,
        page.rows.len(),
        "every row must declare local-core continuity"
    );
}

#[test]
fn seeded_page_raw_private_material_excluded_on_all_records() {
    let page = page();
    for record in &page.snapshot.records {
        assert!(
            record.raw_private_material_excluded,
            "record '{}' must have raw_private_material_excluded: true",
            record.row_key
        );
    }
}

#[test]
fn seeded_page_local_oss_rows_are_no_account_compatible() {
    let page = page();
    for record in &page.snapshot.records {
        if record.profile_token == DeploymentProfileClass::LocalOss.as_str()
            || record.profile_token == DeploymentProfileClass::AirGapped.as_str()
            || record.surface_token == MatrixSurfaceClass::Accessibility.as_str()
        {
            assert!(
                record.no_account_local_compatible,
                "record '{}' (profile='{}') must declare no-account local-use \
                 compatibility",
                record.row_key, record.profile_token
            );
        }
    }
}

#[test]
fn seeded_page_local_oss_and_air_gapped_surface_rows_use_local_only_or_air_gapped_dependency() {
    let page = page();
    for record in &page.snapshot.records {
        if record.profile_token == DeploymentProfileClass::LocalOss.as_str()
            || record.profile_token == DeploymentProfileClass::AirGapped.as_str()
        {
            let dep = record.dependency_class_token.as_str();
            assert!(
                dep == DependencyClass::LocalOnly.as_str()
                    || dep == DependencyClass::AirGapped.as_str()
                    || dep == DependencyClass::Network.as_str(),
                "record '{}' (profile='{}') has unexpected dependency class '{}'",
                record.row_key,
                record.profile_token,
                dep
            );
        }
    }
}

#[test]
fn missing_required_row_narrows_to_preview() {
    let mut snapshot = seeded_qualification_snapshot();
    // Remove the desktop_local:local_oss row.
    snapshot
        .records
        .retain(|r| r.row_key != "desktop_local:local_oss");
    let page = QualificationMatrixPage::new(
        "remote:qualification_matrix:desktop:test-preview",
        "test",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        QualificationTierClass::Preview.as_str()
    );
    assert!(
        page.defects
            .iter()
            .any(|d| d.narrow_reason == NarrowReasonClass::RequiredRowMissing),
        "defect list must contain a required_row_missing defect"
    );
}

#[test]
fn raw_private_material_on_any_record_withdraws_packet() {
    let mut snapshot = seeded_qualification_snapshot();
    if let Some(record) = snapshot
        .records
        .iter_mut()
        .find(|r| r.row_key == "desktop_local:managed")
    {
        record.raw_private_material_excluded = false;
    }
    let page = QualificationMatrixPage::new(
        "remote:qualification_matrix:desktop:test-withdrawn",
        "test",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        QualificationTierClass::Withdrawn.as_str()
    );
    assert!(
        page.defects
            .iter()
            .any(|d| d.narrow_reason == NarrowReasonClass::RawPrivateMaterialExposed),
        "defect list must contain a raw_private_material_exposed defect"
    );
}

#[test]
fn missing_local_core_continuity_narrows_to_beta() {
    let mut snapshot = seeded_qualification_snapshot();
    if let Some(record) = snapshot
        .records
        .iter_mut()
        .find(|r| r.row_key == "remote_helper:managed")
    {
        record.local_core_continuity_allowed = false;
    }
    let page = QualificationMatrixPage::new(
        "remote:qualification_matrix:desktop:test-beta-continuity",
        "test",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        QualificationTierClass::Beta.as_str()
    );
    assert!(
        page.defects
            .iter()
            .any(|d| d.narrow_reason == NarrowReasonClass::LocalCoreContinuityUndeclared),
        "defect list must contain a local_core_continuity_undeclared defect"
    );
}

#[test]
fn support_export_wraps_page_cleanly() {
    let page = page();
    let export = QualificationMatrixSupportExport::from_page(
        "remote:qualification_matrix:desktop:export-default",
        "2026-06-01T00:00:00Z",
        page.clone(),
    );
    assert_eq!(
        export.record_kind,
        QUALIFICATION_MATRIX_SUPPORT_EXPORT_RECORD_KIND
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
    assert!(export.page.qualifies_stable());
}

#[test]
fn audit_function_returns_empty_defects_for_seeded_page() {
    let page = page();
    let re_audit = audit_qualification_matrix_page(&page);
    assert!(
        re_audit.is_empty(),
        "re-audit of seeded page must produce zero defects: {re_audit:?}"
    );
}

#[test]
fn seeded_page_accessibility_rows_all_use_local_only_dependency() {
    let page = page();
    for record in &page.snapshot.records {
        if record.surface_token == MatrixSurfaceClass::Accessibility.as_str() {
            assert_eq!(
                record.dependency_class_token,
                DependencyClass::LocalOnly.as_str(),
                "accessibility record '{}' must use local_only dependency",
                record.row_key
            );
            assert_eq!(
                record.failure_downgrade_token,
                FailureDowngradeClass::NotApplicable.as_str(),
                "accessibility record '{}' must use not_applicable failure downgrade",
                record.row_key
            );
        }
    }
}

#[test]
fn narrow_reason_tokens_are_stable_and_non_empty() {
    assert!(!NarrowReasonClass::NotNarrowed.as_str().is_empty());
    assert!(!NarrowReasonClass::RawPrivateMaterialExposed
        .as_str()
        .is_empty());
    assert!(!NarrowReasonClass::RequiredRowMissing.as_str().is_empty());
    assert!(!NarrowReasonClass::LocalCoreContinuityUndeclared
        .as_str()
        .is_empty());
    assert!(!NarrowReasonClass::DependencyClassUndeclared
        .as_str()
        .is_empty());
    assert!(!NarrowReasonClass::NoAccountCompatibilityUndeclared
        .as_str()
        .is_empty());
    assert!(!NarrowReasonClass::FailureDowngradeUndeclared
        .as_str()
        .is_empty());
}

#[test]
fn qualification_tier_tokens_are_stable_and_non_empty() {
    assert_eq!(QualificationTierClass::Stable.as_str(), "stable");
    assert_eq!(QualificationTierClass::Beta.as_str(), "beta");
    assert_eq!(QualificationTierClass::Preview.as_str(), "preview");
    assert_eq!(QualificationTierClass::Withdrawn.as_str(), "withdrawn");
}
