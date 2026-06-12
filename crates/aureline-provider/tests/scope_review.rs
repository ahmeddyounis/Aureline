//! Integration coverage for the canonical provider scope-review packet.

use aureline_provider::{
    seeded_provider_scope_review_page, validate_provider_scope_review_page,
    ProviderScopeReviewDefectKind, ProviderScopeReviewPage, ProviderScopeReviewSupportExport,
    ScopeReviewDecisionClass, ScopeReviewDowngradeActionClass,
    ScopeReviewInvalidationTriggerClass, ScopeReviewSurfaceClass,
    PROVIDER_SCOPE_REVIEW_PAGE_RECORD_KIND, PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
};

#[test]
fn seeded_page_round_trips_through_serde() {
    let page = seeded_provider_scope_review_page();
    let json = serde_json::to_string(&page).expect("serialize");
    let parsed: ProviderScopeReviewPage = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(parsed.record_kind, PROVIDER_SCOPE_REVIEW_PAGE_RECORD_KIND);
    assert_eq!(parsed.schema_version, PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION);
    assert_eq!(parsed.resolutions.len(), page.resolutions.len());
    assert_eq!(parsed.alternatives.len(), page.alternatives.len());
    assert_eq!(
        parsed.consumer_projections.len(),
        page.consumer_projections.len()
    );
}

#[test]
fn seeded_page_validates_and_covers_required_scope_review_axes() {
    let page = seeded_provider_scope_review_page();
    validate_provider_scope_review_page(&page).expect("seeded page validates");
    let report = page.validate();
    assert!(report.passed, "seeded page defects: {:#?}", report.defects);

    for decision in [
        ScopeReviewDecisionClass::Allowed,
        ScopeReviewDecisionClass::Denied,
        ScopeReviewDecisionClass::BrowserOnly,
        ScopeReviewDecisionClass::LocalDraftOnly,
    ] {
        assert!(
            page.summary
                .decision_classes_present
                .contains(&decision.as_str().to_owned()),
            "missing decision coverage: {decision:?}"
        );
    }

    for surface in [
        ScopeReviewSurfaceClass::Desktop,
        ScopeReviewSurfaceClass::CliHeadless,
        ScopeReviewSurfaceClass::Companion,
        ScopeReviewSurfaceClass::SupportExport,
    ] {
        assert!(
            page.summary
                .consumer_surfaces_present
                .contains(&surface.as_str().to_owned()),
            "missing consumer coverage: {surface:?}"
        );
    }

    for trigger in [
        ScopeReviewInvalidationTriggerClass::ActorClassRevoked,
        ScopeReviewInvalidationTriggerClass::HostMismatchDetected,
        ScopeReviewInvalidationTriggerClass::TenantSwitchDetected,
        ScopeReviewInvalidationTriggerClass::OrgMembershipLost,
        ScopeReviewInvalidationTriggerClass::ProviderHealthDegraded,
    ] {
        assert!(
            page.summary
                .invalidation_triggers_present
                .contains(&trigger.as_str().to_owned()),
            "missing invalidation trigger coverage: {trigger:?}"
        );
    }
}

#[test]
fn support_export_preserves_effective_scope_without_raw_material() {
    let page = seeded_provider_scope_review_page();
    let export = ProviderScopeReviewSupportExport::from_page(
        "provider-scope-review:support-export:test",
        "2026-06-12T20:15:00Z",
        &page,
    );

    assert!(export.raw_scope_material_excluded);
    assert_eq!(export.resolution_summaries.len(), page.resolutions.len());
    assert_eq!(
        export.consumer_surface_summaries.len(),
        page.consumer_projections.len()
    );

    let json = serde_json::to_string(&export).expect("serialize export");
    assert!(!json.contains("Bearer "));
    assert!(!json.contains("https://"));
    assert!(!json.contains("\"raw_scope_material_present\":true"));
}

#[test]
fn browser_only_resolution_requires_alternative_and_handoff_refs() {
    let mut page = seeded_provider_scope_review_page();
    let resolution = page
        .resolutions
        .iter_mut()
        .find(|row| row.resolution_id == "scope_review:resolution:code_host:browser_only:merge")
        .expect("browser-only resolution");
    resolution.browser_handoff_packet_ref = None;
    resolution.least_privilege_alternative_refs.clear();

    let report = page.validate();
    assert!(!report.passed);
    assert!(report.defects.iter().any(|defect| {
        defect.defect_kind == ProviderScopeReviewDefectKind::BrowserOnlyWithoutHandoffRef
    }));
    assert!(report.defects.iter().any(|defect| {
        defect.defect_kind == ProviderScopeReviewDefectKind::AlternativeCoverageBroken
    }));
}

#[test]
fn consumer_projection_must_mirror_canonical_decision_object() {
    let mut page = seeded_provider_scope_review_page();
    let projection = page
        .consumer_projections
        .iter_mut()
        .find(|row| row.surface_class == ScopeReviewSurfaceClass::CliHeadless)
        .expect("cli projection");
    projection.projected_decision_summary = "CLI-local wording drifted from canonical review".into();

    let report = page.validate();
    assert!(!report.passed);
    assert!(report.defects.iter().any(|defect| {
        defect.defect_kind == ProviderScopeReviewDefectKind::ConsumerProjectionDrift
    }));
}

#[test]
fn host_mismatch_and_tenant_switch_must_force_visible_downgrades() {
    let mut page = seeded_provider_scope_review_page();
    let event = page
        .invalidation_events
        .iter_mut()
        .find(|row| {
            row.invalidation_trigger_class == ScopeReviewInvalidationTriggerClass::TenantSwitchDetected
        })
        .expect("tenant-switch invalidation");
    event.downgrade_action_class = ScopeReviewDowngradeActionClass::NoDowngradeRequired;
    event.repair_hook_ref = None;

    let report = page.validate();
    assert!(!report.passed);
    assert!(report.defects.iter().any(|defect| {
        defect.defect_kind == ProviderScopeReviewDefectKind::InvalidationBroken
    }));
}
