use super::*;

fn page() -> OpenVsPaidBoundaryPage {
    seeded_open_vs_paid_boundary_page()
}

#[test]
fn seeded_page_seeds_zero_defects_and_qualifies_stable() {
    let page = page();
    assert_eq!(page.defects.len(), 0);
    assert!(validate_open_vs_paid_boundary_page(&page).is_ok());
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        OpenVsPaidBoundaryQualificationClass::Stable.as_str()
    );
}

#[test]
fn seeded_page_passes_all_stability_conditions() {
    let page = page();
    assert!(page.local_core_independence_enforced());
    assert!(page.offboarding_disclosed_for_managed());
    assert!(page.usage_export_disclosed_for_managed());
    assert!(page.usage_export_schema_version_is_current());
}

#[test]
fn seeded_page_rows_are_all_stable() {
    let page = page();
    assert!(!page.rows.is_empty());
    for row in &page.rows {
        assert_eq!(
            row.qualification_token,
            OpenVsPaidBoundaryQualificationClass::Stable.as_str(),
            "row '{}' must qualify stable; got '{}'",
            row.capability_family_token,
            row.qualification_token
        );
        assert_eq!(
            row.narrow_reason_token,
            OpenVsPaidBoundaryNarrowReasonClass::NotNarrowed.as_str(),
            "row '{}' must have not_narrowed reason",
            row.capability_family_token
        );
    }
}

#[test]
fn seeded_page_rows_cover_all_local_core_families() {
    let page = page();
    let families: Vec<&str> = page
        .rows
        .iter()
        .map(|r| r.capability_family_token.as_str())
        .collect();
    for core in &CapabilityFamilyClass::LOCAL_CORE_FAMILIES {
        assert!(
            families.contains(&core.as_str()),
            "missing local-core family '{}'",
            core.as_str()
        );
    }
}

#[test]
fn seeded_page_local_core_rows_are_open_local() {
    let page = page();
    for row in &page.rows {
        if CapabilityFamilyClass::LOCAL_CORE_FAMILIES
            .iter()
            .any(|f| f.as_str() == row.capability_family_token)
        {
            assert_eq!(
                row.boundary_class_token,
                CapabilityBoundaryClass::OpenLocal.as_str(),
                "local-core row '{}' must be open_local",
                row.capability_family_token
            );
        }
    }
}

#[test]
fn seeded_page_managed_rows_carry_packets() {
    let page = page();
    for row in &page.rows {
        if row.boundary_class_token == CapabilityBoundaryClass::ManagedHosted.as_str()
            || row.boundary_class_token == CapabilityBoundaryClass::EnterpriseGoverned.as_str()
        {
            assert!(
                row.usage_export_packet.is_some(),
                "managed row '{}' must carry a usage-export packet",
                row.capability_family_token
            );
            assert!(
                row.offboarding_packet.is_some(),
                "managed row '{}' must carry an offboarding packet",
                row.capability_family_token
            );
        }
    }
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
fn support_export_wraps_clean_page() {
    let page = page();
    let export = OpenVsPaidBoundarySupportExport::from_page(
        "policy:open-vs-paid-boundary:support-export:fixture-001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
    assert_eq!(export.page.summary.withdrawn_row_count, 0);
}

#[test]
fn local_core_classified_managed_withdraws_packet() {
    let mut input = seeded_open_vs_paid_boundary_input();
    if let Some(row) = input
        .capability_rows
        .iter_mut()
        .find(|r| r.capability_family == CapabilityFamilyClass::EditorCore)
    {
        row.boundary_class = CapabilityBoundaryClass::ManagedHosted;
    }
    let page = OpenVsPaidBoundaryPage::new(
        "policy:open_vs_paid_boundary:test",
        "test page",
        "2026-06-01T00:00:00Z",
        input,
    );
    assert!(!page.qualifies_stable());
    assert!(!page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        OpenVsPaidBoundaryQualificationClass::Withdrawn.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason
            == OpenVsPaidBoundaryNarrowReasonClass::LocalCoreRequiresManagedPrerequisite
    }));
}

#[test]
fn inconsistent_surface_classification_narrows_to_beta() {
    let mut input = seeded_open_vs_paid_boundary_input();
    if let Some(row) = input
        .capability_rows
        .iter_mut()
        .find(|r| r.capability_family == CapabilityFamilyClass::Collaboration)
    {
        row.surfaces_consistent = false;
    }
    let page = OpenVsPaidBoundaryPage::new(
        "policy:open_vs_paid_boundary:test",
        "test page",
        "2026-06-01T00:00:00Z",
        input,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        OpenVsPaidBoundaryQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason
            == OpenVsPaidBoundaryNarrowReasonClass::CapabilityMisclassifiedAcrossSurfaces
    }));
}

#[test]
fn missing_offboarding_disclosure_narrows_to_beta() {
    let mut input = seeded_open_vs_paid_boundary_input();
    if let Some(row) = input
        .capability_rows
        .iter_mut()
        .find(|r| r.capability_family == CapabilityFamilyClass::ManagedAiRouting)
    {
        row.offboarding_disclosed = false;
    }
    let page = OpenVsPaidBoundaryPage::new(
        "policy:open_vs_paid_boundary:test",
        "test page",
        "2026-06-01T00:00:00Z",
        input,
    );
    assert!(!page.qualifies_stable());
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == OpenVsPaidBoundaryNarrowReasonClass::OffboardingStateUndisclosed
    }));
}

#[test]
fn missing_usage_export_disclosure_narrows_to_beta() {
    let mut input = seeded_open_vs_paid_boundary_input();
    if let Some(row) = input
        .capability_rows
        .iter_mut()
        .find(|r| r.capability_family == CapabilityFamilyClass::AdminDashboard)
    {
        row.usage_export_disclosed = false;
    }
    let page = OpenVsPaidBoundaryPage::new(
        "policy:open_vs_paid_boundary:test",
        "test page",
        "2026-06-01T00:00:00Z",
        input,
    );
    assert!(!page.qualifies_stable());
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == OpenVsPaidBoundaryNarrowReasonClass::UsageExportUndisclosed
    }));
}

#[test]
fn stale_usage_export_schema_narrows_to_beta() {
    let mut input = seeded_open_vs_paid_boundary_input();
    input.usage_export_schema_version_current = false;
    let page = OpenVsPaidBoundaryPage::new(
        "policy:open_vs_paid_boundary:test",
        "test page",
        "2026-06-01T00:00:00Z",
        input,
    );
    assert!(!page.qualifies_stable());
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == OpenVsPaidBoundaryNarrowReasonClass::UsageExportSchemaVersionStale
    }));
}

#[test]
fn silent_entitlement_loss_narrows_to_beta() {
    let mut input = seeded_open_vs_paid_boundary_input();
    if let Some(row) = input
        .capability_rows
        .iter_mut()
        .find(|r| r.capability_family == CapabilityFamilyClass::ExtensionsMarketplace)
    {
        row.entitlement_loss_visible = false;
    }
    let page = OpenVsPaidBoundaryPage::new(
        "policy:open_vs_paid_boundary:test",
        "test page",
        "2026-06-01T00:00:00Z",
        input,
    );
    assert!(!page.qualifies_stable());
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == OpenVsPaidBoundaryNarrowReasonClass::ManagedCopyClaimedAsLocal
    }));
}

#[test]
fn re_audit_matches_embedded_defects() {
    let page = page();
    let reaudited = audit_open_vs_paid_boundary_page(&page);
    assert_eq!(
        reaudited.len(),
        page.defects.len(),
        "re-audit must match embedded defects"
    );
}

#[test]
fn qualification_class_checks() {
    assert!(OpenVsPaidBoundaryQualificationClass::Stable.is_stable());
    assert!(!OpenVsPaidBoundaryQualificationClass::Beta.is_stable());
    assert!(!OpenVsPaidBoundaryQualificationClass::Withdrawn.is_stable());
    assert!(OpenVsPaidBoundaryQualificationClass::Stable.is_claimable());
    assert!(OpenVsPaidBoundaryQualificationClass::Beta.is_claimable());
    assert!(!OpenVsPaidBoundaryQualificationClass::Withdrawn.is_claimable());
    assert!(!OpenVsPaidBoundaryQualificationClass::Preview.is_claimable());
}

#[test]
fn narrow_reason_withdrawal_sentinel_check() {
    assert!(
        OpenVsPaidBoundaryNarrowReasonClass::LocalCoreRequiresManagedPrerequisite
            .is_withdrawal_reason()
    );
    assert!(
        !OpenVsPaidBoundaryNarrowReasonClass::CapabilityMisclassifiedAcrossSurfaces
            .is_withdrawal_reason()
    );
    assert!(!OpenVsPaidBoundaryNarrowReasonClass::NotNarrowed.is_withdrawal_reason());
}

#[test]
fn capability_boundary_class_helpers() {
    assert!(CapabilityBoundaryClass::ManagedHosted.requires_offboarding_disclosure());
    assert!(CapabilityBoundaryClass::EnterpriseGoverned.requires_offboarding_disclosure());
    assert!(!CapabilityBoundaryClass::OpenLocal.requires_offboarding_disclosure());
    assert!(!CapabilityBoundaryClass::NotIncluded.requires_offboarding_disclosure());
    assert!(CapabilityBoundaryClass::OpenLocal.is_local_safe());
    assert!(CapabilityBoundaryClass::NotIncluded.is_local_safe());
    assert!(!CapabilityBoundaryClass::ManagedHosted.is_local_safe());
}

#[test]
fn capability_family_class_helpers() {
    assert!(CapabilityFamilyClass::EditorCore.is_local_core());
    assert!(CapabilityFamilyClass::LocalSafeAi.is_local_core());
    assert!(!CapabilityFamilyClass::Collaboration.is_local_core());
    assert!(!CapabilityFamilyClass::AdminDashboard.is_local_core());
}

#[test]
fn usage_export_packet_construction() {
    let packet = UsageExportPacket::new(
        "policy:usage_export:editor_core:test",
        CapabilityFamilyClass::EditorCore,
        UsageExportAvailabilityClass::Full,
        false,
        ExportRetentionClass::UserOwnedImmediate,
        "core_quota",
        true,
        "Test export packet",
    );
    assert_eq!(packet.record_kind, USAGE_EXPORT_PACKET_RECORD_KIND);
    assert_eq!(packet.schema_version, OPEN_VS_PAID_BOUNDARY_SCHEMA_VERSION);
    assert_eq!(packet.availability_token, "full");
    assert!(!packet.partial);
    assert!(packet.tenant_scoped_data_excluded);
}

#[test]
fn offboarding_packet_construction() {
    let packet = OffboardingPacket::new(
        "policy:offboarding:collaboration:test",
        CapabilityFamilyClass::Collaboration,
        OffboardingOutcomeClass::LocalOnly,
        OffboardingOutcomeClass::PolicyRetained,
        OffboardingOutcomeClass::Completed,
        OffboardingOutcomeClass::PolicyRetained,
        GraceWindowStateClass::Active,
        true,
        "Test offboarding packet",
    );
    assert_eq!(packet.record_kind, OFFBOARDING_PACKET_RECORD_KIND);
    assert_eq!(packet.schema_version, OPEN_VS_PAID_BOUNDARY_SCHEMA_VERSION);
    assert_eq!(packet.local_data_outcome_token, "local_only");
    assert_eq!(packet.grace_window_state_token, "active");
    assert!(packet.platform_retention_disclosed);
}

#[test]
fn offboarding_outcome_class_tokens() {
    assert_eq!(OffboardingOutcomeClass::LocalOnly.as_str(), "local_only");
    assert_eq!(OffboardingOutcomeClass::ManagedCopy.as_str(), "managed_copy");
    assert_eq!(OffboardingOutcomeClass::Queued.as_str(), "queued");
    assert_eq!(OffboardingOutcomeClass::Partial.as_str(), "partial");
    assert_eq!(
        OffboardingOutcomeClass::BlockedByHold.as_str(),
        "blocked_by_hold"
    );
    assert_eq!(
        OffboardingOutcomeClass::PolicyRetained.as_str(),
        "policy_retained"
    );
    assert_eq!(
        OffboardingOutcomeClass::OutsidePlatformScope.as_str(),
        "outside_platform_scope"
    );
    assert_eq!(OffboardingOutcomeClass::Completed.as_str(), "completed");
}

#[test]
fn usage_export_availability_class_tokens() {
    assert_eq!(UsageExportAvailabilityClass::Full.as_str(), "full");
    assert_eq!(UsageExportAvailabilityClass::Partial.as_str(), "partial");
    assert_eq!(
        UsageExportAvailabilityClass::Unavailable.as_str(),
        "unavailable"
    );
}

#[test]
fn export_retention_class_tokens() {
    assert_eq!(
        ExportRetentionClass::UserOwnedImmediate.as_str(),
        "user_owned_immediate"
    );
    assert_eq!(
        ExportRetentionClass::TenantRetainedPolicy.as_str(),
        "tenant_retained_policy"
    );
    assert_eq!(
        ExportRetentionClass::BillingRetained.as_str(),
        "billing_retained"
    );
    assert_eq!(
        ExportRetentionClass::GraceWindowExportable.as_str(),
        "grace_window_exportable"
    );
    assert_eq!(
        ExportRetentionClass::ExpiredUnavailable.as_str(),
        "expired_unavailable"
    );
}

#[test]
fn grace_window_state_class_tokens() {
    assert_eq!(GraceWindowStateClass::Active.as_str(), "active");
    assert_eq!(GraceWindowStateClass::Expired.as_str(), "expired");
    assert_eq!(GraceWindowStateClass::ExportOnly.as_str(), "export_only");
    assert_eq!(GraceWindowStateClass::Degraded.as_str(), "degraded");
}
