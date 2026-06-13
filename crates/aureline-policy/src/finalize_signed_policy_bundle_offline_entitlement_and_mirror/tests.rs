use super::*;

#[test]
fn seeded_page_qualifies_stable() {
    let page = seeded_finalize_signed_policy_bundle_page();
    assert!(
        page.qualifies_stable(),
        "seeded page must qualify stable; got '{}'; defects: {:?}",
        page.summary.overall_qualification_token,
        page.defects
    );
}

#[test]
fn seeded_page_has_zero_defects() {
    let page = seeded_finalize_signed_policy_bundle_page();
    assert!(
        page.defects.is_empty(),
        "seeded page must have zero defects; got: {:?}",
        page.defects
    );
}

#[test]
fn seeded_page_has_twenty_rows() {
    let page = seeded_finalize_signed_policy_bundle_page();
    assert_eq!(
        page.rows.len(),
        20,
        "seeded page must have 20 rows (5 flows x 4 bundle kinds)"
    );
}

#[test]
fn seeded_page_covers_required_flows_kinds_and_delivery_sources() {
    let page = seeded_finalize_signed_policy_bundle_page();
    assert!(page.covers_all_required_import_flows());
    assert!(page.covers_all_required_bundle_kinds());
    assert!(page.covers_all_required_delivery_sources());
}

#[test]
fn seeded_page_rows_are_stable_and_inspectable() {
    let page = seeded_finalize_signed_policy_bundle_page();
    for row in &page.rows {
        assert_eq!(
            row.qualification_token, "stable",
            "row '{}' must be stable",
            row.row_id
        );
    }
    assert!(page.all_epoch_states_inspectable());
    assert!(page.all_rows_have_inspectable_envelopes());
    assert!(page.all_simulation_packets_have_affected_surfaces());
    assert!(page.all_rows_explicit_on_local_core_continuity());
}

#[test]
fn seeded_page_stale_rows_are_explicit_and_pause_privileged_operations() {
    let page = seeded_finalize_signed_policy_bundle_page();
    assert!(page.stale_rows_are_explicitly_labeled());
    assert!(page.stale_rows_have_declared_grace_windows());
    assert!(page.stale_rows_deny_new_privileged_operations());
}

#[test]
fn seeded_page_emergency_disable_rows_declare_minimum_version() {
    let page = seeded_finalize_signed_policy_bundle_page();
    assert!(page.emergency_disable_rows_declare_required_minimum_version());
}

#[test]
fn seeded_page_lifecycle_events_cover_required_classes() {
    let page = seeded_finalize_signed_policy_bundle_page();
    assert_eq!(page.lifecycle_events.len(), 5);
    assert!(page.lifecycle_events_cover_required_classes());
}

#[test]
fn staleness_disguised_as_auth_failure_triggers_withdrawal() {
    let mut rows = seeded_rows();
    let row = rows
        .iter_mut()
        .find(|row| {
            row.import_flow == BundleImportFlowClass::OfflineGrace
                && row.bundle_kind == BundleKindClass::AdminPolicyBundle
        })
        .expect("offline grace admin policy row");
    row.grace_state.staleness_label.clear();

    let verifier_page = seeded_offline_entitlement_verifier_beta_page();
    let page = FinalizeSignedPolicyBundlePage::new(
        "test:withdrawal:staleness-disguised",
        "Test: staleness disguised as auth failure",
        "2026-06-01T00:00:00Z",
        rows,
        seeded_lifecycle_events(&seeded_rows()),
        verifier_page,
    );

    assert_eq!(
        page.summary.overall_qualification_token,
        FinalizeSignedPolicyBundleQualificationClass::Withdrawn.as_str()
    );
    assert!(page.defects.iter().any(|defect| {
        defect.narrow_reason
            == FinalizeSignedPolicyBundleNarrowReasonClass::StalenessDisguisedAsAuthFailure
    }));
}

#[test]
fn missing_bundle_kind_narrows_to_preview() {
    let rows: Vec<_> = seeded_rows()
        .into_iter()
        .filter(|row| row.bundle_kind != BundleKindClass::TrustRootSignerUpdate)
        .collect();
    let lifecycle_events: Vec<_> = seeded_lifecycle_events(&seeded_rows())
        .into_iter()
        .filter(|event| event.bundle_kind != BundleKindClass::TrustRootSignerUpdate)
        .collect();
    let verifier_page = seeded_offline_entitlement_verifier_beta_page();
    let page = FinalizeSignedPolicyBundlePage::new(
        "test:preview:missing-bundle-kind",
        "Test: missing bundle kind",
        "2026-06-01T00:00:00Z",
        rows,
        lifecycle_events,
        verifier_page,
    );

    assert_eq!(
        page.summary.overall_qualification_token,
        FinalizeSignedPolicyBundleQualificationClass::Preview.as_str()
    );
    assert!(page.defects.iter().any(|defect| {
        defect.narrow_reason == FinalizeSignedPolicyBundleNarrowReasonClass::BundleKindCoverageGap
    }));
}

#[test]
fn missing_lifecycle_class_narrows_to_preview() {
    let rows = seeded_rows();
    let lifecycle_events: Vec<_> = seeded_lifecycle_events(&rows)
        .into_iter()
        .filter(|event| event.event_class != BundleLifecycleEventClass::SignerRotationReview)
        .collect();
    let verifier_page = seeded_offline_entitlement_verifier_beta_page();
    let page = FinalizeSignedPolicyBundlePage::new(
        "test:preview:missing-lifecycle-class",
        "Test: missing lifecycle class",
        "2026-06-01T00:00:00Z",
        rows,
        lifecycle_events,
        verifier_page,
    );

    assert_eq!(
        page.summary.overall_qualification_token,
        FinalizeSignedPolicyBundleQualificationClass::Preview.as_str()
    );
    assert!(page.defects.iter().any(|defect| {
        defect.narrow_reason == FinalizeSignedPolicyBundleNarrowReasonClass::LifecycleCoverageGap
    }));
}

#[test]
fn emergency_disable_without_required_minimum_version_narrows_to_beta() {
    let mut rows = seeded_rows();
    let row = rows
        .iter_mut()
        .find(|row| {
            row.import_flow == BundleImportFlowClass::ManualImport
                && row.bundle_kind == BundleKindClass::EmergencyDisableBundle
        })
        .expect("manual emergency-disable row");
    row.envelope_review.required_minimum_version.clear();

    let lifecycle_events = seeded_lifecycle_events(&rows);
    let verifier_page = seeded_offline_entitlement_verifier_beta_page();
    let page = FinalizeSignedPolicyBundlePage::new(
        "test:beta:missing-min-version",
        "Test: missing minimum version",
        "2026-06-01T00:00:00Z",
        rows,
        lifecycle_events,
        verifier_page,
    );

    assert_eq!(
        page.summary.overall_qualification_token,
        FinalizeSignedPolicyBundleQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|defect| {
        defect.narrow_reason
            == FinalizeSignedPolicyBundleNarrowReasonClass::RequiredMinimumVersionMissing
    }));
}

#[test]
fn support_export_always_excludes_raw_private_material() {
    let page = seeded_finalize_signed_policy_bundle_page();
    let export = FinalizeSignedPolicyBundleSupportExport::from_page(
        "export:test:001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(export.raw_private_material_excluded);
    assert_eq!(export.lifecycle_event_counts_by_class.len(), 5);
}

#[test]
fn validate_returns_ok_for_seeded_page() {
    let page = seeded_finalize_signed_policy_bundle_page();
    let result = validate_finalize_signed_policy_bundle_page(&page);
    assert!(result.is_ok(), "validate must return Ok; got {result:?}");
}

#[test]
fn audit_returns_zero_defects_for_seeded_page() {
    let page = seeded_finalize_signed_policy_bundle_page();
    let defects = audit_finalize_signed_policy_bundle_page(&page);
    assert!(defects.is_empty(), "re-audit defects: {defects:?}");
}
