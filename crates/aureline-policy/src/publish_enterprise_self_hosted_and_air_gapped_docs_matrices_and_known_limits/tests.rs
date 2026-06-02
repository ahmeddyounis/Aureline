use super::*;

// ---------------------------------------------------------------------------
// Seeded page invariants
// ---------------------------------------------------------------------------

#[test]
fn seeded_page_qualifies_stable() {
    let page = seeded_enterprise_docs_matrices_known_limits_page();
    assert!(
        page.qualifies_stable(),
        "seeded page must qualify stable; got '{}'; defects: {:?}",
        page.summary.overall_qualification_token,
        page.defects
    );
}

#[test]
fn seeded_page_has_zero_defects() {
    let page = seeded_enterprise_docs_matrices_known_limits_page();
    assert!(
        page.defects.is_empty(),
        "seeded page must have zero defects; got: {:?}",
        page.defects
    );
}

#[test]
fn seeded_page_has_five_rows() {
    let page = seeded_enterprise_docs_matrices_known_limits_page();
    assert_eq!(
        page.rows.len(),
        5,
        "seeded page must have 5 rows (one per enterprise profile)"
    );
}

#[test]
fn seeded_page_covers_all_enterprise_profiles() {
    let page = seeded_enterprise_docs_matrices_known_limits_page();
    assert!(
        page.covers_all_required_profiles(),
        "seeded page must cover all five required enterprise profiles"
    );
}

#[test]
fn seeded_page_all_rows_are_stable() {
    let page = seeded_enterprise_docs_matrices_known_limits_page();
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
    let page = seeded_enterprise_docs_matrices_known_limits_page();
    assert!(
        page.all_rows_preserve_local_core(),
        "all seeded rows must carry a non-blocking local-core continuity posture"
    );
}

#[test]
fn seeded_page_no_row_blocks_local_core() {
    let page = seeded_enterprise_docs_matrices_known_limits_page();
    assert!(
        page.no_row_blocks_local_core(),
        "no seeded row may declare a local-core posture that blocks local-core work"
    );
}

#[test]
fn seeded_page_no_withdrawn_rows() {
    let page = seeded_enterprise_docs_matrices_known_limits_page();
    assert!(
        page.no_withdrawn_rows(),
        "seeded page must have zero withdrawn rows"
    );
}

#[test]
fn seeded_page_individual_local_row_is_not_applicable() {
    let page = seeded_enterprise_docs_matrices_known_limits_page();
    let local_row = page
        .rows
        .iter()
        .find(|r| r.enterprise_profile == EnterpriseProfileClass::IndividualLocal)
        .expect("seeded page must contain an individual_local row");
    assert_eq!(
        local_row.docs.docs_state,
        DocsCompletenessClass::NotApplicable,
        "individual_local row must declare docs_state: not_applicable"
    );
    assert_eq!(
        local_row.matrix.matrix_state,
        MatrixCompletenessClass::NotApplicable,
        "individual_local row must declare matrix_state: not_applicable"
    );
    assert_eq!(
        local_row.known_limits.known_limits_state,
        KnownLimitCompletenessClass::NotApplicable,
        "individual_local row must declare known_limits_state: not_applicable"
    );
    assert_eq!(
        local_row.proof_currency.proof_currency,
        ProofCurrencyClass::NotApplicable,
        "individual_local row must declare proof_currency: not_applicable"
    );
}

#[test]
fn seeded_page_enterprise_rows_have_tenant_region_and_policy_source() {
    let page = seeded_enterprise_docs_matrices_known_limits_page();
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
fn seeded_page_sovereignty_profiles_have_current_proof() {
    let page = seeded_enterprise_docs_matrices_known_limits_page();
    assert!(
        page.all_sovereignty_profiles_have_current_proof(),
        "all sovereignty profiles in the seeded page must carry current proof"
    );
}

#[test]
fn seeded_page_local_core_preserved_count_equals_five() {
    let page = seeded_enterprise_docs_matrices_known_limits_page();
    assert_eq!(
        page.summary.local_core_preserved_row_count,
        5,
        "all 5 rows must carry a preserved local-core continuity posture"
    );
}

#[test]
fn seeded_page_docs_current_count_equals_five() {
    let page = seeded_enterprise_docs_matrices_known_limits_page();
    assert_eq!(
        page.summary.docs_current_row_count,
        5,
        "all 5 rows must carry current or not_applicable docs"
    );
}

#[test]
fn seeded_page_matrix_complete_count_equals_five() {
    let page = seeded_enterprise_docs_matrices_known_limits_page();
    assert_eq!(
        page.summary.matrix_complete_row_count,
        5,
        "all 5 rows must carry complete or not_applicable matrices"
    );
}

#[test]
fn seeded_page_known_limits_fully_disclosed_count_equals_five() {
    let page = seeded_enterprise_docs_matrices_known_limits_page();
    assert_eq!(
        page.summary.known_limits_fully_disclosed_row_count,
        5,
        "all 5 rows must carry fully_disclosed or not_applicable known limits"
    );
}

// ---------------------------------------------------------------------------
// Audit: local-core blocked by default triggers withdrawal
// ---------------------------------------------------------------------------

#[test]
fn local_core_blocked_by_default_triggers_withdrawal() {
    let mut rows = vec![row_self_hosted()];
    rows[0].local_core_posture = LocalCoreContinuityPostureClass::BlockedByDefault;
    rows[0].local_core_posture_token =
        LocalCoreContinuityPostureClass::BlockedByDefault.as_str().to_owned();
    let page = EnterpriseDocsMatricesKnownLimitsPage::new(
        "test:withdrawal:local-core-blocked",
        "Test: local core blocked by default",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        EnterpriseDocsMatricesKnownLimitsQualificationClass::Withdrawn.as_str(),
        "blocked_by_default local-core posture must withdraw the packet"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason
                == EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::LocalCoreBlockedByDefault
        }),
        "defects must include local_core_blocked_by_default"
    );
}

// ---------------------------------------------------------------------------
// Audit: aspirational proof on sovereignty profile triggers withdrawal
// ---------------------------------------------------------------------------

#[test]
fn aspirational_proof_on_sovereign_profile_triggers_withdrawal() {
    let mut rows = vec![row_self_hosted()];
    rows[0].proof_currency.proof_currency = ProofCurrencyClass::Aspirational;
    rows[0].proof_currency.proof_currency_token =
        ProofCurrencyClass::Aspirational.as_str().to_owned();
    let page = EnterpriseDocsMatricesKnownLimitsPage::new(
        "test:withdrawal:aspirational-proof",
        "Test: aspirational proof on sovereign profile",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        EnterpriseDocsMatricesKnownLimitsQualificationClass::Withdrawn.as_str(),
        "aspirational proof on sovereignty profile must withdraw the packet"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason
                == EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::AspirationalProofOnSovereignProfile
        }),
        "defects must include aspirational_proof_on_sovereign_profile"
    );
}

#[test]
fn aspirational_proof_on_air_gapped_triggers_withdrawal() {
    let mut rows = vec![row_air_gapped()];
    rows[0].proof_currency.proof_currency = ProofCurrencyClass::Aspirational;
    rows[0].proof_currency.proof_currency_token =
        ProofCurrencyClass::Aspirational.as_str().to_owned();
    let page = EnterpriseDocsMatricesKnownLimitsPage::new(
        "test:withdrawal:aspirational-proof-air-gapped",
        "Test: aspirational proof on air-gapped profile",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        EnterpriseDocsMatricesKnownLimitsQualificationClass::Withdrawn.as_str(),
        "aspirational proof on air-gapped profile must withdraw the packet"
    );
}

// ---------------------------------------------------------------------------
// Audit: missing enterprise profile triggers preview narrowing
// ---------------------------------------------------------------------------

#[test]
fn missing_profile_narrows_to_preview() {
    let rows = vec![row_individual_local()];
    let page = EnterpriseDocsMatricesKnownLimitsPage::new(
        "test:preview:missing-profile",
        "Test: missing enterprise profiles",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        EnterpriseDocsMatricesKnownLimitsQualificationClass::Preview.as_str(),
        "missing enterprise profiles must narrow to preview"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason
                == EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::ProfileCoverageGap
        }),
        "defects must include profile_coverage_gap"
    );
}

// ---------------------------------------------------------------------------
// Audit: docs deficiencies narrow to beta
// ---------------------------------------------------------------------------

#[test]
fn stale_docs_narrows_to_beta() {
    let mut rows = seeded_rows();
    rows[1].docs.docs_state = DocsCompletenessClass::Stale;
    rows[1].docs.docs_state_token = DocsCompletenessClass::Stale.as_str().to_owned();
    let page = EnterpriseDocsMatricesKnownLimitsPage::new(
        "test:beta:stale-docs",
        "Test: stale docs",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert!(
        page.summary.overall_qualification_token
            == EnterpriseDocsMatricesKnownLimitsQualificationClass::Beta.as_str()
            || page.summary.overall_qualification_token
                == EnterpriseDocsMatricesKnownLimitsQualificationClass::Withdrawn.as_str(),
        "stale docs must narrow to at least beta"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason == EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::DocsStale
        }),
        "defects must include docs_stale"
    );
}

#[test]
fn missing_docs_narrows_to_beta() {
    let mut rows = seeded_rows();
    rows[2].docs.docs_state = DocsCompletenessClass::Missing;
    rows[2].docs.docs_state_token = DocsCompletenessClass::Missing.as_str().to_owned();
    let page = EnterpriseDocsMatricesKnownLimitsPage::new(
        "test:beta:missing-docs",
        "Test: missing docs",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert!(
        page.summary.overall_qualification_token
            == EnterpriseDocsMatricesKnownLimitsQualificationClass::Beta.as_str()
            || page.summary.overall_qualification_token
                == EnterpriseDocsMatricesKnownLimitsQualificationClass::Withdrawn.as_str(),
        "missing docs must narrow to at least beta"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason == EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::DocsMissing
        }),
        "defects must include docs_missing"
    );
}

// ---------------------------------------------------------------------------
// Audit: matrix deficiencies narrow to beta
// ---------------------------------------------------------------------------

#[test]
fn partial_matrix_narrows_to_beta() {
    let mut rows = seeded_rows();
    rows[3].matrix.matrix_state = MatrixCompletenessClass::Partial;
    rows[3].matrix.matrix_state_token = MatrixCompletenessClass::Partial.as_str().to_owned();
    let page = EnterpriseDocsMatricesKnownLimitsPage::new(
        "test:beta:partial-matrix",
        "Test: partial matrix",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert!(
        page.summary.overall_qualification_token
            == EnterpriseDocsMatricesKnownLimitsQualificationClass::Beta.as_str()
            || page.summary.overall_qualification_token
                == EnterpriseDocsMatricesKnownLimitsQualificationClass::Withdrawn.as_str(),
        "partial matrix must narrow to at least beta"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason == EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::MatrixPartial
        }),
        "defects must include matrix_partial"
    );
}

#[test]
fn missing_matrix_narrows_to_beta() {
    let mut rows = seeded_rows();
    rows[4].matrix.matrix_state = MatrixCompletenessClass::Missing;
    rows[4].matrix.matrix_state_token = MatrixCompletenessClass::Missing.as_str().to_owned();
    let page = EnterpriseDocsMatricesKnownLimitsPage::new(
        "test:beta:missing-matrix",
        "Test: missing matrix",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert!(
        page.summary.overall_qualification_token
            == EnterpriseDocsMatricesKnownLimitsQualificationClass::Beta.as_str()
            || page.summary.overall_qualification_token
                == EnterpriseDocsMatricesKnownLimitsQualificationClass::Withdrawn.as_str(),
        "missing matrix must narrow to at least beta"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason == EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::MatrixMissing
        }),
        "defects must include matrix_missing"
    );
}

// ---------------------------------------------------------------------------
// Audit: known-limits deficiencies narrow to beta
// ---------------------------------------------------------------------------

#[test]
fn partially_disclosed_known_limits_narrows_to_beta() {
    let mut rows = seeded_rows();
    rows[1].known_limits.known_limits_state = KnownLimitCompletenessClass::PartiallyDisclosed;
    rows[1].known_limits.known_limits_state_token =
        KnownLimitCompletenessClass::PartiallyDisclosed.as_str().to_owned();
    let page = EnterpriseDocsMatricesKnownLimitsPage::new(
        "test:beta:partially-disclosed-known-limits",
        "Test: partially disclosed known limits",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert!(
        page.summary.overall_qualification_token
            == EnterpriseDocsMatricesKnownLimitsQualificationClass::Beta.as_str()
            || page.summary.overall_qualification_token
                == EnterpriseDocsMatricesKnownLimitsQualificationClass::Withdrawn.as_str(),
        "partially disclosed known limits must narrow to at least beta"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason
                == EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::KnownLimitsPartiallyDisclosed
        }),
        "defects must include known_limits_partially_disclosed"
    );
}

#[test]
fn undisclosed_known_limits_narrows_to_beta() {
    let mut rows = seeded_rows();
    rows[2].known_limits.known_limits_state = KnownLimitCompletenessClass::Undisclosed;
    rows[2].known_limits.known_limits_state_token =
        KnownLimitCompletenessClass::Undisclosed.as_str().to_owned();
    let page = EnterpriseDocsMatricesKnownLimitsPage::new(
        "test:beta:undisclosed-known-limits",
        "Test: undisclosed known limits",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert!(
        page.summary.overall_qualification_token
            == EnterpriseDocsMatricesKnownLimitsQualificationClass::Beta.as_str()
            || page.summary.overall_qualification_token
                == EnterpriseDocsMatricesKnownLimitsQualificationClass::Withdrawn.as_str(),
        "undisclosed known limits must narrow to at least beta"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason
                == EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::KnownLimitsUndisclosed
        }),
        "defects must include known_limits_undisclosed"
    );
}

// ---------------------------------------------------------------------------
// Audit: stale proof on sovereignty profile narrows to beta
// ---------------------------------------------------------------------------

#[test]
fn stale_proof_on_sovereign_profile_narrows_to_beta() {
    let mut rows = seeded_rows();
    rows[1].proof_currency.proof_currency = ProofCurrencyClass::Stale;
    rows[1].proof_currency.proof_currency_token = ProofCurrencyClass::Stale.as_str().to_owned();
    let page = EnterpriseDocsMatricesKnownLimitsPage::new(
        "test:beta:stale-proof",
        "Test: stale proof on sovereign profile",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert!(
        page.summary.overall_qualification_token
            == EnterpriseDocsMatricesKnownLimitsQualificationClass::Beta.as_str()
            || page.summary.overall_qualification_token
                == EnterpriseDocsMatricesKnownLimitsQualificationClass::Withdrawn.as_str(),
        "stale proof on sovereignty profile must narrow to at least beta"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason == EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::ProofStale
        }),
        "defects must include proof_stale"
    );
}

// ---------------------------------------------------------------------------
// Audit: missing tenant/region ref narrows to beta
// ---------------------------------------------------------------------------

#[test]
fn missing_tenant_region_ref_narrows_to_beta() {
    let mut rows = seeded_rows();
    rows[3].tenant_region_owner_ref.clear();
    let page = EnterpriseDocsMatricesKnownLimitsPage::new(
        "test:beta:missing-tenant-region",
        "Test: missing tenant/region ownership ref",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert!(
        page.summary.overall_qualification_token
            == EnterpriseDocsMatricesKnownLimitsQualificationClass::Beta.as_str()
            || page.summary.overall_qualification_token
                == EnterpriseDocsMatricesKnownLimitsQualificationClass::Withdrawn.as_str(),
        "missing tenant/region ref must narrow to at least beta"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason
                == EnterpriseDocsMatricesKnownLimitsNarrowReasonClass::TenantRegionOwnershipNotDeclared
        }),
        "defects must include tenant_region_ownership_not_declared"
    );
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

#[test]
fn support_export_always_excludes_raw_private_material() {
    let page = seeded_enterprise_docs_matrices_known_limits_page();
    let export = EnterpriseDocsMatricesKnownLimitsSupportExport::from_page(
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
    let page = seeded_enterprise_docs_matrices_known_limits_page();
    let export = EnterpriseDocsMatricesKnownLimitsSupportExport::from_page(
        "export:test:002",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert_eq!(
        export.record_kind,
        ENTERPRISE_DOCS_MATRICES_KNOWN_LIMITS_SUPPORT_EXPORT_RECORD_KIND
    );
}

// ---------------------------------------------------------------------------
// Re-audit helpers are consistent
// ---------------------------------------------------------------------------

#[test]
fn validate_returns_ok_for_seeded_page() {
    let page = seeded_enterprise_docs_matrices_known_limits_page();
    let result = validate_enterprise_docs_matrices_known_limits_page(&page);
    assert!(
        result.is_ok(),
        "validate must return Ok for the seeded page; defects: {:?}",
        result.err()
    );
}

#[test]
fn audit_returns_zero_defects_for_seeded_page() {
    let page = seeded_enterprise_docs_matrices_known_limits_page();
    let defects = audit_enterprise_docs_matrices_known_limits_page(&page);
    assert!(
        defects.is_empty(),
        "re-audit of seeded page must return zero defects; got: {:?}",
        defects
    );
}
