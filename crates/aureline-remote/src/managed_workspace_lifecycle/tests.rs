use super::*;

fn page() -> ManagedWorkspaceLifecyclePage {
    seeded_managed_workspace_lifecycle_page()
}

#[test]
fn seeded_page_produces_zero_defects_and_publishes_truthful() {
    let page = page();
    assert_eq!(
        page.defects.len(),
        0,
        "seeded page must be clean: {:?}",
        page.defects
    );
    assert!(validate_lifecycle_page(&page).is_ok());
    assert!(page.publishes_truthful());
    assert!(page.no_withheld_rows());
    assert_eq!(
        page.summary.overall_disposition_token,
        LifecycleDispositionClass::Truthful.as_str()
    );
}

#[test]
fn seeded_page_passes_all_truthfulness_conditions() {
    let page = page();
    assert!(
        page.covers_all_required_states(),
        "all ten required lifecycle states must be covered"
    );
    assert!(
        page.all_records_reach_required_surfaces(),
        "every record must reach every required surface"
    );
    assert!(
        page.no_continuity_overclaim(),
        "no record may claim exact continuity over a material change"
    );
    assert!(
        page.all_material_changes_carry_caveats(),
        "every material-change record must carry a caveat history"
    );
    assert!(
        page.all_outage_states_offer_local_safe_continuation(),
        "every outage/expiry state must offer local-safe continuation and a recovery option"
    );
}

#[test]
fn seeded_page_covers_every_required_state() {
    let page = page();
    let covered = page.snapshot.covered_states();
    for state in &REQUIRED_LIFECYCLE_STATES {
        assert!(
            covered.contains(state),
            "required lifecycle state '{state}' must be covered"
        );
    }
}

#[test]
fn seeded_page_record_count_equals_required_count() {
    let page = page();
    assert_eq!(
        page.snapshot.records.len(),
        REQUIRED_RECORD_COUNT,
        "snapshot must contain exactly {REQUIRED_RECORD_COUNT} records"
    );
}

#[test]
fn seeded_page_all_rows_publish_truthful() {
    let page = page();
    assert!(!page.rows.is_empty(), "page must have at least one row");
    for row in &page.rows {
        assert_eq!(
            row.disposition_token,
            LifecycleDispositionClass::Truthful.as_str(),
            "state '{}' must publish truthful; got '{}'",
            row.lifecycle_state_token,
            row.disposition_token
        );
        assert_eq!(
            row.narrow_reason_token,
            NarrowReasonClass::NotNarrowed.as_str(),
            "state '{}' must have not_narrowed reason; got '{}'",
            row.lifecycle_state_token,
            row.narrow_reason_token
        );
    }
}

#[test]
fn seeded_page_summary_counts_are_consistent() {
    let page = page();
    assert_eq!(page.summary.record_count, page.rows.len());
    assert_eq!(page.summary.truthful_count, page.rows.len());
    assert_eq!(page.summary.narrowed_count, 0);
    assert_eq!(page.summary.flagged_count, 0);
    assert_eq!(page.summary.withheld_count, 0);
    assert_eq!(
        page.summary.local_safe_continuation_count,
        page.rows.len(),
        "every seeded record offers local-safe continuation"
    );
}

#[test]
fn seeded_page_raw_private_material_excluded_on_all_records() {
    let page = page();
    for record in &page.snapshot.records {
        assert!(
            record.raw_private_material_excluded,
            "record '{}' must have raw_private_material_excluded: true",
            record.lifecycle_state_token
        );
    }
}

#[test]
fn seeded_page_every_record_declares_target_identity() {
    let page = page();
    for record in &page.snapshot.records {
        assert!(
            !record.target_identity_ref.is_empty(),
            "record '{}' must declare a target_identity_ref",
            record.lifecycle_state_token
        );
    }
}

#[test]
fn seeded_page_material_change_records_carry_caveats_and_avoid_exact_continuity() {
    let page = page();
    for record in &page.snapshot.records {
        if record.has_material_change() {
            assert!(
                !record.caveat_history.is_empty(),
                "material-change record '{}' must carry caveats",
                record.lifecycle_state_token
            );
            assert!(
                !record.continuity_class.claims_exact_continuity(),
                "material-change record '{}' must not claim exact continuity",
                record.lifecycle_state_token
            );
        }
    }
}

#[test]
fn seeded_page_outage_states_offer_recovery_and_local_safe_continuation() {
    let page = page();
    for record in &page.snapshot.records {
        if record.lifecycle_state.requires_local_safe_continuation() {
            assert!(
                record.local_safe_continuation_available,
                "outage state '{}' must offer local-safe continuation",
                record.lifecycle_state_token
            );
            assert!(
                !record.recovery_options.is_empty(),
                "outage state '{}' must offer at least one recovery option",
                record.lifecycle_state_token
            );
        }
    }
}

#[test]
fn missing_required_state_flags_the_packet() {
    let mut snapshot = seeded_lifecycle_snapshot();
    snapshot
        .records
        .retain(|r| r.lifecycle_state_token != "resumed");
    let page = ManagedWorkspaceLifecyclePage::new(
        "remote:managed_workspace_lifecycle:test-flagged",
        "test",
        "2026-06-11T00:00:00Z",
        snapshot,
    );
    assert!(!page.publishes_truthful());
    assert_eq!(
        page.summary.overall_disposition_token,
        LifecycleDispositionClass::Flagged.as_str()
    );
    assert!(
        page.defects
            .iter()
            .any(|d| d.narrow_reason == NarrowReasonClass::RequiredStateMissing),
        "defect list must contain a required_state_missing defect"
    );
}

#[test]
fn raw_private_material_on_any_record_withholds_packet() {
    let mut snapshot = seeded_lifecycle_snapshot();
    if let Some(record) = snapshot
        .records
        .iter_mut()
        .find(|r| r.lifecycle_state_token == "ready")
    {
        record.raw_private_material_excluded = false;
    }
    let page = ManagedWorkspaceLifecyclePage::new(
        "remote:managed_workspace_lifecycle:test-withheld",
        "test",
        "2026-06-11T00:00:00Z",
        snapshot,
    );
    assert!(!page.publishes_truthful());
    assert_eq!(
        page.summary.overall_disposition_token,
        LifecycleDispositionClass::Withheld.as_str()
    );
    assert!(
        page.defects
            .iter()
            .any(|d| d.narrow_reason == NarrowReasonClass::RawPrivateMaterialExposed),
        "defect list must contain a raw_private_material_exposed defect"
    );
}

#[test]
fn exact_continuity_over_material_change_narrows_packet() {
    let mut snapshot = seeded_lifecycle_snapshot();
    if let Some(record) = snapshot
        .records
        .iter_mut()
        .find(|r| r.lifecycle_state_token == "resumed")
    {
        // Claim exact continuity while declaring a materially changed image.
        record.image_provenance = ProvenanceClass::SuccessorImage;
        record.image_provenance_token = ProvenanceClass::SuccessorImage.as_str().to_owned();
        record.provenance_changed = true;
        record.continuity_class = ContinuityClass::ExactContinuity;
        record.continuity_class_token = ContinuityClass::ExactContinuity.as_str().to_owned();
        record.caveat_history = vec![CaveatClass::ImageChanged];
    }
    let page = ManagedWorkspaceLifecyclePage::new(
        "remote:managed_workspace_lifecycle:test-overclaim",
        "test",
        "2026-06-11T00:00:00Z",
        snapshot,
    );
    assert!(!page.publishes_truthful());
    assert_eq!(
        page.summary.overall_disposition_token,
        LifecycleDispositionClass::Narrowed.as_str()
    );
    assert!(
        page.defects
            .iter()
            .any(|d| d.narrow_reason == NarrowReasonClass::ContinuityOverclaim),
        "defect list must contain a continuity_overclaim defect"
    );
    let resumed_row = page
        .rows
        .iter()
        .find(|r| r.lifecycle_state_token == "resumed")
        .expect("resumed row present");
    assert_eq!(
        resumed_row.disposition_token,
        LifecycleDispositionClass::Narrowed.as_str()
    );
}

#[test]
fn material_change_without_caveats_narrows_packet() {
    let mut snapshot = seeded_lifecycle_snapshot();
    if let Some(record) = snapshot
        .records
        .iter_mut()
        .find(|r| r.lifecycle_state_token == "rebuild_required")
    {
        record.caveat_history.clear();
    }
    let page = ManagedWorkspaceLifecyclePage::new(
        "remote:managed_workspace_lifecycle:test-no-caveats",
        "test",
        "2026-06-11T00:00:00Z",
        snapshot,
    );
    assert!(!page.publishes_truthful());
    assert!(
        page.defects
            .iter()
            .any(|d| d.narrow_reason == NarrowReasonClass::CaveatHistoryMissing),
        "defect list must contain a caveat_history_missing defect"
    );
}

#[test]
fn expired_state_without_local_safe_continuation_narrows_packet() {
    let mut snapshot = seeded_lifecycle_snapshot();
    if let Some(record) = snapshot
        .records
        .iter_mut()
        .find(|r| r.lifecycle_state_token == "expired")
    {
        record.local_safe_continuation_available = false;
        record.recovery_options.clear();
    }
    let page = ManagedWorkspaceLifecyclePage::new(
        "remote:managed_workspace_lifecycle:test-no-local-safe",
        "test",
        "2026-06-11T00:00:00Z",
        snapshot,
    );
    assert!(!page.publishes_truthful());
    assert!(
        page.defects
            .iter()
            .any(|d| d.narrow_reason == NarrowReasonClass::LocalSafeContinuationUnavailable),
        "defect list must contain a local_safe_continuation_unavailable defect"
    );
}

#[test]
fn missing_surface_coverage_flags_the_record() {
    let mut snapshot = seeded_lifecycle_snapshot();
    if let Some(record) = snapshot
        .records
        .iter_mut()
        .find(|r| r.lifecycle_state_token == "ready")
    {
        record
            .surfaces_present
            .retain(|s| *s != SurfaceClass::SupportExport);
    }
    let page = ManagedWorkspaceLifecyclePage::new(
        "remote:managed_workspace_lifecycle:test-surface-gap",
        "test",
        "2026-06-11T00:00:00Z",
        snapshot,
    );
    assert!(!page.publishes_truthful());
    assert!(
        page.defects
            .iter()
            .any(|d| d.narrow_reason == NarrowReasonClass::SurfaceCoverageIncomplete),
        "defect list must contain a surface_coverage_incomplete defect"
    );
}

#[test]
fn empty_target_identity_withholds_record() {
    let mut snapshot = seeded_lifecycle_snapshot();
    if let Some(record) = snapshot
        .records
        .iter_mut()
        .find(|r| r.lifecycle_state_token == "ready")
    {
        record.target_identity_ref.clear();
    }
    let page = ManagedWorkspaceLifecyclePage::new(
        "remote:managed_workspace_lifecycle:test-no-identity",
        "test",
        "2026-06-11T00:00:00Z",
        snapshot,
    );
    assert!(!page.publishes_truthful());
    assert_eq!(
        page.summary.overall_disposition_token,
        LifecycleDispositionClass::Withheld.as_str()
    );
    assert!(
        page.defects
            .iter()
            .any(|d| d.narrow_reason == NarrowReasonClass::TargetIdentityUndeclared),
        "defect list must contain a target_identity_undeclared defect"
    );
}

#[test]
fn support_export_wraps_page_cleanly() {
    let page = page();
    let export = LifecycleSupportExport::from_page(
        "remote:managed_workspace_lifecycle:export-default",
        "2026-06-11T00:00:00Z",
        page.clone(),
    );
    assert_eq!(
        export.record_kind,
        MANAGED_WORKSPACE_LIFECYCLE_SUPPORT_EXPORT_RECORD_KIND
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
    assert!(export.page.publishes_truthful());
}

#[test]
fn audit_function_returns_empty_defects_for_seeded_page() {
    let page = page();
    let re_audit = audit_lifecycle_page(&page);
    assert!(
        re_audit.is_empty(),
        "re-audit of seeded page must produce zero defects: {re_audit:?}"
    );
}

#[test]
fn lifecycle_state_tokens_match_required_order() {
    let states = [
        LifecycleStateClass::Provision,
        LifecycleStateClass::Warm,
        LifecycleStateClass::Ready,
        LifecycleStateClass::Suspended,
        LifecycleStateClass::Resumed,
        LifecycleStateClass::Reconnecting,
        LifecycleStateClass::RebuildRequired,
        LifecycleStateClass::RecreateRequired,
        LifecycleStateClass::Expired,
        LifecycleStateClass::LocalSafeContinuation,
    ];
    for (state, expected) in states.iter().zip(REQUIRED_LIFECYCLE_STATES.iter()) {
        assert_eq!(state.as_str(), *expected);
    }
}

#[test]
fn narrow_reason_tokens_are_stable_and_non_empty() {
    for reason in [
        NarrowReasonClass::NotNarrowed,
        NarrowReasonClass::RawPrivateMaterialExposed,
        NarrowReasonClass::TargetIdentityUndeclared,
        NarrowReasonClass::RequiredStateMissing,
        NarrowReasonClass::SurfaceCoverageIncomplete,
        NarrowReasonClass::ContinuityOverclaim,
        NarrowReasonClass::CaveatHistoryMissing,
        NarrowReasonClass::LocalSafeContinuationUnavailable,
    ] {
        assert!(!reason.as_str().is_empty());
    }
}

#[test]
fn disposition_tokens_are_stable() {
    assert_eq!(LifecycleDispositionClass::Truthful.as_str(), "truthful");
    assert_eq!(LifecycleDispositionClass::Narrowed.as_str(), "narrowed");
    assert_eq!(LifecycleDispositionClass::Flagged.as_str(), "flagged");
    assert_eq!(LifecycleDispositionClass::Withheld.as_str(), "withheld");
}
