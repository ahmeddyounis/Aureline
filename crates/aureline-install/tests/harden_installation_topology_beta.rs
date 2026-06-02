//! Tests for the hardened install-topology state-root audits and
//! silent-deployment lane.

use aureline_install::{
    seeded_harden_install_topology_page, validate_harden_install_topology_page, BinaryRootClass,
    FleetRolloutEvidenceClass, HardenInstallTopologyPage, ManagedFleetAuditRow, NarrowReasonToken,
    QualificationToken, RolloutRingClass, SilentDeploymentAuditRow, SilentInstallSupportClass,
    StateRootAuditEntry, StateRootIsolationClass, StateRootReviewClass, UpdaterOwnerClass,
    HARDEN_INSTALL_TOPOLOGY_PAGE_RECORD_KIND, HARDEN_INSTALL_TOPOLOGY_SCHEMA_VERSION,
    HARDEN_INSTALL_TOPOLOGY_SHARED_CONTRACT_REF, REQUIRED_FLEET_EVIDENCE,
};

fn seeded_page() -> HardenInstallTopologyPage {
    seeded_harden_install_topology_page()
}

#[test]
fn seeded_page_passes_validation() {
    let page = seeded_page();
    let report = validate_harden_install_topology_page(&page);

    assert!(
        report.passed,
        "seeded page failed validation: {:#?}",
        report.findings
    );
    assert!(report
        .coverage
        .rings_covered
        .contains(&RolloutRingClass::Pilot));
    assert!(report
        .coverage
        .rings_covered
        .contains(&RolloutRingClass::Broad));
    assert!(report
        .coverage
        .updater_owner_classes
        .contains(&UpdaterOwnerClass::ManagedFleet));
    assert!(report
        .coverage
        .binary_root_classes
        .contains(&BinaryRootClass::PerMachineProgramArea));
}

#[test]
fn seeded_page_audit_returns_no_defects() {
    let page = seeded_page();
    let defects = page.audit();

    assert!(
        defects.is_empty(),
        "seeded page has unexpected defects: {:#?}",
        defects
    );
    assert_eq!(page.overall_qualification(), QualificationToken::Stable);
}

#[test]
fn seeded_page_fleet_evidence_is_complete_on_all_managed_rows() {
    let page = seeded_page();

    for row in &page.managed_fleet_rows {
        assert!(
            row.fleet_evidence_complete(),
            "managed row '{}' is missing fleet evidence: {:?}",
            row.row_id,
            row.missing_fleet_evidence()
        );
        for required in REQUIRED_FLEET_EVIDENCE {
            assert!(
                row.fleet_evidence.contains(required),
                "managed row '{}' missing evidence class: {:?}",
                row.row_id,
                required
            );
        }
    }
}

#[test]
fn seeded_page_admin_view_complete_on_all_managed_rows() {
    let page = seeded_page();

    for row in &page.managed_fleet_rows {
        assert!(
            row.admin_view_complete,
            "managed row '{}' has admin_view_complete = false",
            row.row_id
        );
        assert!(
            !row.tenant_ref.trim().is_empty(),
            "managed row '{}' has empty tenant_ref",
            row.row_id
        );
        assert!(
            !row.policy_source_ref.trim().is_empty(),
            "managed row '{}' has empty policy_source_ref",
            row.row_id
        );
        assert!(
            !row.state_root_audit.is_empty(),
            "managed row '{}' has no state-root audit entries",
            row.row_id
        );
    }
}

#[test]
fn seeded_page_silent_deployment_rows_declare_limits_and_return_codes() {
    let page = seeded_page();

    for row in &page.silent_deployment_rows {
        assert!(
            row.limits_declared && !row.disclosed_limits.is_empty(),
            "silent-deployment row '{}' has no disclosed limits",
            row.row_id
        );
        assert!(
            row.return_code_families_named && !row.return_code_family_refs.is_empty(),
            "silent-deployment row '{}' has no return-code family refs",
            row.row_id
        );
    }
}

#[test]
fn seeded_page_support_export_excludes_private_material() {
    let page = seeded_page();
    let export = page.support_export_projection();

    assert!(export.raw_private_material_excluded);
    assert_eq!(
        export.shared_contract_ref,
        HARDEN_INSTALL_TOPOLOGY_SHARED_CONTRACT_REF
    );
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
}

#[test]
fn page_without_managed_rows_is_narrowed_to_preview() {
    let mut page = seeded_page();
    page.managed_fleet_rows.clear();

    let defects = page.audit();
    assert!(!defects.is_empty());
    assert!(defects
        .iter()
        .any(|d| d.narrow_reason_token == NarrowReasonToken::NoManagedRows));
}

#[test]
fn page_with_incomplete_admin_view_is_withdrawn() {
    let mut page = seeded_page();
    page.managed_fleet_rows[0].admin_view_complete = false;

    let defects = page.audit();
    assert!(!defects.is_empty());
    assert!(defects
        .iter()
        .any(|d| d.narrow_reason_token == NarrowReasonToken::AdminViewIncomplete));

    // Withdrawal is immediate — no other defects are added after the withdrawal defect.
    assert_eq!(defects.len(), 1);
}

#[test]
fn row_with_empty_tenant_ref_produces_defect() {
    let mut page = seeded_page();
    page.managed_fleet_rows[0].tenant_ref.clear();

    let defects = page.audit();
    assert!(defects
        .iter()
        .any(|d| d.narrow_reason_token == NarrowReasonToken::TenantNotNamed));
}

#[test]
fn row_with_empty_policy_source_produces_defect() {
    let mut page = seeded_page();
    page.managed_fleet_rows[0].policy_source_ref.clear();

    let defects = page.audit();
    assert!(defects
        .iter()
        .any(|d| d.narrow_reason_token == NarrowReasonToken::PolicySourceNotNamed));
}

#[test]
fn row_with_no_state_root_audit_produces_defect() {
    let mut page = seeded_page();
    page.managed_fleet_rows[0].state_root_audit.clear();

    let defects = page.audit();
    assert!(defects
        .iter()
        .any(|d| d.narrow_reason_token == NarrowReasonToken::StateRootsNotAudited));
}

#[test]
fn row_with_missing_fleet_evidence_produces_defect() {
    let mut page = seeded_page();
    // Remove ring assignment evidence from first managed row.
    page.managed_fleet_rows[0]
        .fleet_evidence
        .retain(|ev| *ev != FleetRolloutEvidenceClass::RingAssignment);

    let defects = page.audit();
    assert!(defects
        .iter()
        .any(|d| d.narrow_reason_token == NarrowReasonToken::FleetEvidenceIncomplete));
}

#[test]
fn silent_row_without_limits_produces_beta_defect() {
    let mut page = seeded_page();
    page.silent_deployment_rows[0].limits_declared = false;
    page.silent_deployment_rows[0].disclosed_limits.clear();

    let defects = page.audit();
    assert!(defects
        .iter()
        .any(|d| d.narrow_reason_token == NarrowReasonToken::SilentLimitsNotDeclared));
}

#[test]
fn silent_row_without_return_codes_produces_beta_defect() {
    let mut page = seeded_page();
    page.silent_deployment_rows[0].return_code_families_named = false;
    page.silent_deployment_rows[0]
        .return_code_family_refs
        .clear();

    let defects = page.audit();
    assert!(defects
        .iter()
        .any(|d| d.narrow_reason_token == NarrowReasonToken::ReturnCodesNotNamed));
}

#[test]
fn record_kind_and_contract_ref_are_stable() {
    let page = seeded_page();
    assert_eq!(page.record_kind, HARDEN_INSTALL_TOPOLOGY_PAGE_RECORD_KIND);
    assert_eq!(page.schema_version, HARDEN_INSTALL_TOPOLOGY_SCHEMA_VERSION);
    assert_eq!(
        page.shared_contract_ref,
        HARDEN_INSTALL_TOPOLOGY_SHARED_CONTRACT_REF
    );
}

#[test]
fn airgap_row_uses_mirror_verification_review_class() {
    let page = seeded_page();
    let airgap = page
        .managed_fleet_rows
        .iter()
        .find(|r| r.platform_token == "air_gap_bundle_target")
        .expect("air-gap managed row");

    assert_eq!(
        airgap.binary_root_class,
        BinaryRootClass::OfflineBundleExtractedProgramArea
    );
    assert!(airgap.state_root_audit.iter().any(|entry| {
        entry.review_class == StateRootReviewClass::MirrorVerificationReviewRequired
    }));
    assert!(airgap
        .state_root_audit
        .iter()
        .any(|entry| { entry.isolation_class == StateRootIsolationClass::MirrorMetadataOwned }));
}
