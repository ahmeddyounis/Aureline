use super::*;

fn page() -> ContinuityClaimMatrixPage {
    seeded_continuity_claim_matrix_page()
}

#[test]
fn seeded_matrix_qualifies_stable_with_zero_defects() {
    let page = page();
    assert!(page.qualifies_stable());
    assert!(
        page.defects.is_empty(),
        "seeded defects: {:?}",
        page.defects
    );
    assert!(validate_continuity_claim_matrix_page(&page).is_ok());
}

#[test]
fn seeded_matrix_distinguishes_planes_and_lanes() {
    let page = page();
    assert!(page.distinguishes_control_and_data_plane());
    assert!(page.distinguishes_local_core_and_managed_lane());
    assert!(page.managed_rows_have_named_drill_owners());
    assert!(page.summary.control_plane_impairment_row_count >= 1);
    assert!(page.summary.data_plane_impairment_row_count >= 1);
    assert!(page.summary.local_core_row_count >= 1);
    assert!(page.summary.managed_lane_row_count >= 1);
}

#[test]
fn seeded_matrix_summary_counts_match_input() {
    let page = page();
    assert_eq!(page.summary.claim_row_count, 5);
    assert_eq!(page.summary.managed_scope_row_count, 4);
    assert_eq!(page.summary.local_core_row_count, 1);
    assert_eq!(page.summary.narrowed_row_count, 0);
    assert_eq!(page.summary.withdrawn_row_count, 0);
    assert_eq!(page.row_outcomes.len(), 5);
    assert!(page
        .row_outcomes
        .iter()
        .all(|outcome| outcome.qualification_token
            == ContinuityClaimQualificationClass::Stable.as_str()));
}

#[test]
fn local_only_row_is_out_of_managed_scope_and_stable() {
    let page = page();
    let outcome = page
        .row_outcome("continuity-row:local-desktop-core")
        .expect("local-core outcome");
    assert!(!outcome.in_managed_scope);
    assert!(!outcome.narrowed);
    assert!(outcome.narrow_reason_tokens.is_empty());
}

#[test]
fn drill_schedule_groups_every_packet_family() {
    let page = page();
    let families: BTreeSet<String> = page
        .drill_schedule
        .iter()
        .map(|entry| entry.packet_family_token.clone())
        .collect();
    assert!(families.contains("backup"));
    assert!(families.contains("restore"));
    assert!(families.contains("failover"));
    assert!(families.contains("snapshot_replication"));
    assert!(families.contains("local_core_continuity"));
    assert_eq!(page.summary.drill_family_count, page.drill_schedule.len());
    assert_eq!(page.summary.needs_drill_row_count, 0);
}

#[test]
fn support_export_wraps_seeded_page_without_raw_private_material() {
    let export = ContinuityClaimMatrixSupportExport::from_page(
        "continuity:claim-matrix:support-export:fixture-001",
        "2026-06-01T00:00:00Z",
        page(),
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
}

#[test]
fn stale_drill_evidence_narrows_managed_row_to_beta() {
    let mut input = seeded_continuity_claim_matrix_input();
    let row = managed_cloud_sync_row(&mut input);
    row.drill.evidence_state = DrillEvidenceStateClass::StaleNeedsDrill;
    row.drill.evidence_state_token = DrillEvidenceStateClass::StaleNeedsDrill.as_str().to_owned();

    let page = ContinuityClaimMatrixPage::new("t:stale", "stale", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(
        page.defects
            .iter()
            .any(|defect| defect.narrow_reason
                == ContinuityClaimNarrowReasonClass::DrillEvidenceStale)
    );
    assert_eq!(page.summary.needs_drill_row_count, 1);
}

#[test]
fn never_run_drill_holds_managed_row_at_preview() {
    let mut input = seeded_continuity_claim_matrix_input();
    let row = managed_cloud_sync_row(&mut input);
    row.drill.evidence_state = DrillEvidenceStateClass::NeverRun;
    row.drill.evidence_state_token = DrillEvidenceStateClass::NeverRun.as_str().to_owned();
    row.drill.last_drill_at = String::new();

    let page = ContinuityClaimMatrixPage::new("t:never", "never", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.narrow_reason == ContinuityClaimNarrowReasonClass::DrillNeverRun));
}

#[test]
fn hidden_vendor_failover_withdraws_sovereign_row() {
    let mut input = seeded_continuity_claim_matrix_input();
    let row = input
        .claim_rows
        .iter_mut()
        .find(|row| row.row_id == "continuity-row:sovereign-airgap-snapshot")
        .expect("sovereign row");
    row.restore_failover_hosting = RestoreFailoverHostingClass::VendorOperated;
    row.restore_failover_hosting_token = RestoreFailoverHostingClass::VendorOperated
        .as_str()
        .to_owned();
    row.external_dependency_disclosed = false;

    let page =
        ContinuityClaimMatrixPage::new("t:overclaim", "overclaim", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Withdrawn.as_str()
    );
    assert_eq!(page.summary.withdrawn_row_count, 1);
    let outcome = page
        .row_outcome("continuity-row:sovereign-airgap-snapshot")
        .expect("sovereign outcome");
    assert!(outcome.claim_withheld);
    assert!(page.defects.iter().any(|defect| defect.narrow_reason
        == ContinuityClaimNarrowReasonClass::SovereignContinuityOverclaimed));
}

#[test]
fn undisclosed_locality_narrows_to_beta() {
    let mut input = seeded_continuity_claim_matrix_input();
    let row = input
        .claim_rows
        .iter_mut()
        .find(|row| row.row_id == "continuity-row:managed-relay-failover")
        .expect("relay row");
    row.locality.processing_locality = LocalityClass::Undisclosed;
    row.locality.processing_locality_token = LocalityClass::Undisclosed.as_str().to_owned();

    let page =
        ContinuityClaimMatrixPage::new("t:locality", "locality", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(
        |defect| defect.narrow_reason == ContinuityClaimNarrowReasonClass::LocalityUndisclosed
    ));
}

#[test]
fn local_only_overclaiming_managed_family_holds_at_preview() {
    let mut input = seeded_continuity_claim_matrix_input();
    let row = input
        .claim_rows
        .iter_mut()
        .find(|row| row.row_id == "continuity-row:local-desktop-core")
        .expect("local row");
    row.continuity_packet_family = ContinuityPacketFamilyClass::Backup;
    row.continuity_packet_family_token = ContinuityPacketFamilyClass::Backup.as_str().to_owned();

    let page = ContinuityClaimMatrixPage::new(
        "t:local-overclaim",
        "local-overclaim",
        "2026-06-01T00:00:00Z",
        input,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(page.defects.iter().any(|defect| defect.narrow_reason
        == ContinuityClaimNarrowReasonClass::LocalOnlyOverclaimedAsManaged));
}

#[test]
fn undisclosed_partial_loss_narrows_to_beta() {
    let mut input = seeded_continuity_claim_matrix_input();
    let row = input
        .claim_rows
        .iter_mut()
        .find(|row| row.row_id == "continuity-row:self-hosted-restore")
        .expect("self-hosted row");
    row.partial_loss = PartialLossClass::Undisclosed;
    row.partial_loss_token = PartialLossClass::Undisclosed.as_str().to_owned();

    let page = ContinuityClaimMatrixPage::new(
        "t:partial-loss",
        "partial-loss",
        "2026-06-01T00:00:00Z",
        input,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.narrow_reason
            == ContinuityClaimNarrowReasonClass::PartialLossUndisclosed));
}

#[test]
fn reauditing_seeded_page_returns_zero_defects() {
    let defects = audit_continuity_claim_matrix_page(&page());
    assert!(defects.is_empty(), "re-audit defects: {defects:?}");
}

fn managed_cloud_sync_row(input: &mut ContinuityClaimMatrixInput) -> &mut ContinuityClaimRow {
    input
        .claim_rows
        .iter_mut()
        .find(|row| row.row_id == "continuity-row:managed-cloud-sync")
        .expect("managed cloud sync row")
}
