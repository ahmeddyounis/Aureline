use super::*;

fn page() -> DeprovisionPreservesBetaPage {
    seeded_deprovision_preserves_beta_page()
}

#[test]
fn seeded_page_seeds_zero_defects_and_qualifies_stable() {
    let page = page();
    assert_eq!(page.defects.len(), 0, "expected zero defects; got: {:?}", page.defects);
    assert!(validate_deprovision_preserves_beta_page(&page).is_ok());
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        DeprovisionProofQualificationClass::Stable.as_str()
    );
}

#[test]
fn seeded_page_covers_all_four_stability_invariants() {
    let page = page();
    assert!(
        page.local_editing_preserved_across_all_exit_events(),
        "local editing must be preserved-unchanged for every managed exit event"
    );
    assert!(
        page.prior_export_opportunity_present_for_all_exits(),
        "every managed exit event must offer a prior export opportunity"
    );
    assert!(
        page.org_affordance_removal_gives_notice(),
        "every org-affordance removal must be accompanied by explicit notice"
    );
    assert!(
        page.all_required_profiles_covered(),
        "all four required deployment profiles must be covered for every exit event"
    );
}

#[test]
fn seeded_page_rows_are_all_stable() {
    let page = page();
    assert!(!page.rows.is_empty());
    for row in &page.rows {
        assert_eq!(
            row.qualification_token,
            DeprovisionProofQualificationClass::Stable.as_str(),
            "row '{}' must qualify stable; got '{}'",
            row.row_id,
            row.qualification_token
        );
        assert_eq!(
            row.narrow_reason_token,
            DeprovisionProofNarrowReasonClass::NotNarrowed.as_str(),
            "row '{}' must have not_narrowed reason",
            row.row_id
        );
    }
}

#[test]
fn seeded_page_summary_counts_match_rows() {
    let page = page();
    assert_eq!(page.summary.row_count, page.rows.len());
    assert_eq!(page.summary.stable_row_count, page.rows.len());
    assert_eq!(page.summary.beta_row_count, 0);
    assert_eq!(page.summary.withdrawn_row_count, 0);
}

#[test]
fn seeded_page_covers_all_exit_events() {
    let page = page();
    let required_events: Vec<&str> = ManagedExitEventClass::MANAGED_EXIT_EVENTS
        .iter()
        .map(|e| e.as_str())
        .collect();
    for event in &required_events {
        assert!(
            page.summary.exit_events_covered.iter().any(|e| e == event),
            "missing required exit event in summary: {event}"
        );
    }
}

#[test]
fn seeded_page_covers_all_required_profiles() {
    let page = page();
    let required_profiles: Vec<&str> = DeprovisionPreservesBetaProfileClass::ALL
        .iter()
        .map(|p| p.as_str())
        .collect();
    for profile in &required_profiles {
        assert!(
            page.summary.profiles_covered.iter().any(|p| p == profile),
            "missing required profile in summary: {profile}"
        );
    }
}

#[test]
fn support_export_wraps_clean_page() {
    let page = page();
    let export = DeprovisionPreservesBetaSupportExport::from_page(
        "auth:deprovision-preserves-export:stable:0001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
    assert_eq!(export.page.summary.withdrawn_row_count, 0);
}

#[test]
fn injected_silently_purged_local_editing_withdraws_row() {
    let mut rows = seeded_deprovision_preserves_beta_page().rows;
    // Inject a silent purge on one managed exit row.
    let row = rows
        .iter_mut()
        .find(|r| r.exit_event_token == ManagedExitEventClass::Deprovision.as_str())
        .unwrap();
    row.local_work_survival.local_editing_token =
        LocalWorkPreservationClass::SilentlyPurged.as_str().to_owned();

    let dirty_page = DeprovisionPreservesBetaPage::new(
        "auth:deprovision_preserves:test",
        "test page",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert!(!dirty_page.qualifies_stable());
    assert!(!dirty_page.no_withdrawn_rows());
    assert_eq!(
        dirty_page.summary.overall_qualification_token,
        DeprovisionProofQualificationClass::Withdrawn.as_str()
    );
    assert!(dirty_page.defects.iter().any(|d| {
        d.narrow_reason == DeprovisionProofNarrowReasonClass::LocalWorkSilentlyPurged
    }));
}

#[test]
fn injected_blocking_managed_exit_withdraws_row() {
    let mut rows = seeded_deprovision_preserves_beta_page().rows;
    // Inject a blocking exit — editing set to read-only (not preserved-unchanged).
    let row = rows
        .iter_mut()
        .find(|r| r.exit_event_token == ManagedExitEventClass::SignOut.as_str())
        .unwrap();
    row.local_work_survival.local_editing_token =
        LocalWorkPreservationClass::PreservedReadOnly.as_str().to_owned();

    let dirty_page = DeprovisionPreservesBetaPage::new(
        "auth:deprovision_preserves:test",
        "test page",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert!(!dirty_page.qualifies_stable());
    assert!(dirty_page.defects.iter().any(|d| {
        d.narrow_reason == DeprovisionProofNarrowReasonClass::ManagedExitBlocksLocalCore
    }));
}

#[test]
fn injected_missing_export_opportunity_narrows_to_beta() {
    let mut rows = seeded_deprovision_preserves_beta_page().rows;
    let row = rows
        .iter_mut()
        .find(|r| r.exit_event_token == ManagedExitEventClass::OrgSwitch.as_str())
        .unwrap();
    row.local_work_survival.prior_export_opportunity = false;

    let dirty_page = DeprovisionPreservesBetaPage::new(
        "auth:deprovision_preserves:test",
        "test page",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert!(!dirty_page.qualifies_stable());
    assert!(dirty_page.defects.iter().any(|d| {
        d.narrow_reason == DeprovisionProofNarrowReasonClass::ExportPathUnavailableBeforeClose
    }));
}

#[test]
fn injected_data_bearing_affordance_removed_without_notice_narrowed() {
    let mut rows = seeded_deprovision_preserves_beta_page().rows;
    let row = rows
        .iter_mut()
        .find(|r| r.exit_event_token == ManagedExitEventClass::SeatLoss.as_str())
        .unwrap();
    row.org_affordance.collab_session_token =
        OrgAffordanceClass::RemovedWithoutNotice.as_str().to_owned();

    let dirty_page = DeprovisionPreservesBetaPage::new(
        "auth:deprovision_preserves:test",
        "test page",
        "2026-06-01T00:00:00Z",
        rows,
    );
    assert!(!dirty_page.qualifies_stable());
    assert!(dirty_page.defects.iter().any(|d| {
        d.narrow_reason
            == DeprovisionProofNarrowReasonClass::DataBearingAffordanceRemovedWithoutNotice
    }));
}

#[test]
fn withdrawal_reason_sentinel_check() {
    assert!(DeprovisionProofNarrowReasonClass::LocalWorkSilentlyPurged.is_withdrawal_reason());
    assert!(DeprovisionProofNarrowReasonClass::ManagedExitBlocksLocalCore.is_withdrawal_reason());
    assert!(
        !DeprovisionProofNarrowReasonClass::ExportPathUnavailableBeforeClose
            .is_withdrawal_reason()
    );
    assert!(!DeprovisionProofNarrowReasonClass::NotNarrowed.is_withdrawal_reason());
}

#[test]
fn qualification_class_stable_checks() {
    assert!(DeprovisionProofQualificationClass::Stable.is_stable());
    assert!(!DeprovisionProofQualificationClass::Beta.is_stable());
    assert!(!DeprovisionProofQualificationClass::Withdrawn.is_stable());
    assert!(DeprovisionProofQualificationClass::Stable.is_claimable());
    assert!(DeprovisionProofQualificationClass::Beta.is_claimable());
    assert!(!DeprovisionProofQualificationClass::Withdrawn.is_claimable());
}

#[test]
fn preservation_class_no_silent_purge_check() {
    assert!(LocalWorkPreservationClass::PreservedUnchanged.satisfies_no_silent_purge());
    assert!(LocalWorkPreservationClass::PreservedReadOnly.satisfies_no_silent_purge());
    assert!(LocalWorkPreservationClass::ExportAvailableThenClosed.satisfies_no_silent_purge());
    assert!(LocalWorkPreservationClass::NotApplicable.satisfies_no_silent_purge());
    assert!(!LocalWorkPreservationClass::SilentlyPurged.satisfies_no_silent_purge());
}

#[test]
fn managed_exit_event_requires_proof_check() {
    assert!(ManagedExitEventClass::SignOut.requires_local_work_preservation_proof());
    assert!(ManagedExitEventClass::OrgSwitch.requires_local_work_preservation_proof());
    assert!(ManagedExitEventClass::SeatLoss.requires_local_work_preservation_proof());
    assert!(ManagedExitEventClass::Deprovision.requires_local_work_preservation_proof());
    assert!(
        !ManagedExitEventClass::AccountFreeLocalNoManagedExit
            .requires_local_work_preservation_proof()
    );
}
