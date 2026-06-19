use super::*;

fn dashboard() -> ContinuityFreshnessSloDashboard {
    seeded_continuity_freshness_slo_dashboard()
}

fn row<'a>(
    input: &'a mut ContinuityFreshnessSloInput,
    row_id: &str,
) -> &'a mut ContinuityFreshnessRow {
    input
        .rows
        .iter_mut()
        .find(|row| row.row_id == row_id)
        .unwrap_or_else(|| panic!("missing seeded row: {row_id}"))
}

fn set_slo_state(row: &mut ContinuityFreshnessRow, state: ContinuityFreshnessSloState) {
    row.proof_packet.slo_state = state;
    row.proof_packet.slo_state_token = state.as_str().to_owned();
}

#[test]
fn seeded_dashboard_is_clean_and_proceeds() {
    let dashboard = dashboard();
    assert!(
        dashboard.is_structurally_clean(),
        "defects: {:?}",
        dashboard.defects
    );
    assert!(dashboard.promotion.proceeds());
    assert!(dashboard.is_clean_and_proceeds());
    assert!(validate_continuity_freshness_slo_dashboard(&dashboard).is_ok());
    assert_eq!(
        audit_continuity_freshness_slo_dashboard(&dashboard).len(),
        0
    );
}

#[test]
fn seeded_summary_counts_match_input() {
    let dashboard = dashboard();
    assert_eq!(dashboard.summary.row_count, 5);
    assert_eq!(dashboard.summary.release_scope_row_count, 4);
    assert_eq!(dashboard.summary.local_core_row_count, 1);
    assert_eq!(dashboard.summary.due_for_refresh_row_count, 1);
    assert_eq!(dashboard.summary.breached_row_count, 0);
    assert_eq!(dashboard.summary.missing_row_count, 0);
    assert_eq!(dashboard.summary.narrowed_row_count, 0);
    assert_eq!(dashboard.summary.blocked_row_count, 0);
    assert_eq!(dashboard.summary.stop_rules_firing_count, 0);
    assert_eq!(dashboard.summary.automatable_rerun_row_count, 4);
    assert_eq!(dashboard.summary.overall_decision_token, "proceed");
}

#[test]
fn due_for_refresh_row_still_holds_its_label() {
    let dashboard = dashboard();
    let outcome = dashboard
        .row_outcome("continuity-row:managed-relay-failover")
        .expect("relay outcome");
    assert!(outcome.within_slo);
    assert!(!outcome.narrowed);
    assert!(!outcome.blocks_promotion);
    assert_eq!(outcome.row_state_token, "due_for_refresh");
    assert_eq!(outcome.effective_qualification_token, "stable");
}

#[test]
fn breached_packet_narrows_and_holds_promotion() {
    let mut input = seeded_continuity_freshness_slo_input();
    let target = row(&mut input, "continuity-row:managed-cloud-sync");
    set_slo_state(target, ContinuityFreshnessSloState::Breached);
    target.proof_packet.captured_at = Some("2026-01-01".to_owned());
    let dashboard = ContinuityFreshnessSloDashboard::new("d", "d", "2026-06-19T00:00:00Z", input);

    assert!(
        dashboard.is_structurally_clean(),
        "defects: {:?}",
        dashboard.defects
    );
    let outcome = dashboard
        .row_outcome("continuity-row:managed-cloud-sync")
        .expect("outcome");
    assert!(!outcome.within_slo);
    assert!(outcome.narrowed);
    assert!(outcome.blocks_promotion);
    assert_eq!(outcome.row_state_token, "narrowed_stale");
    assert_eq!(outcome.effective_qualification_token, "beta");
    assert!(outcome
        .active_stop_reason_tokens
        .contains(&"continuity_packet_freshness_breached".to_owned()));
    assert_eq!(dashboard.promotion.decision, "hold");
    assert!(dashboard
        .promotion
        .blocked_row_ids
        .contains(&"continuity-row:managed-cloud-sync".to_owned()));
    assert!(dashboard
        .promotion
        .firing_rule_ids
        .contains(&"continuity-stop:freshness-breached".to_owned()));
}

#[test]
fn missing_packet_narrows_to_preview() {
    let mut input = seeded_continuity_freshness_slo_input();
    let target = row(&mut input, "continuity-row:self-hosted-restore");
    set_slo_state(target, ContinuityFreshnessSloState::Missing);
    target.proof_packet.captured_at = None;
    target.proof_packet.evidence_refs = Vec::new();
    let dashboard = ContinuityFreshnessSloDashboard::new("d", "d", "2026-06-19T00:00:00Z", input);

    assert!(
        dashboard.is_structurally_clean(),
        "defects: {:?}",
        dashboard.defects
    );
    let outcome = dashboard
        .row_outcome("continuity-row:self-hosted-restore")
        .expect("outcome");
    assert_eq!(outcome.row_state_token, "narrowed_missing");
    assert_eq!(outcome.effective_qualification_token, "preview");
    assert!(outcome.blocks_promotion);
    assert_eq!(dashboard.promotion.decision, "hold");
}

#[test]
fn missing_owner_signoff_narrows_release_scope_row() {
    let mut input = seeded_continuity_freshness_slo_input();
    row(&mut input, "continuity-row:managed-relay-failover").owner_signoff_present = false;
    let dashboard = ContinuityFreshnessSloDashboard::new("d", "d", "2026-06-19T00:00:00Z", input);

    let outcome = dashboard
        .row_outcome("continuity-row:managed-relay-failover")
        .expect("outcome");
    assert_eq!(outcome.row_state_token, "narrowed_unowned");
    assert!(outcome.narrowed);
    assert!(outcome.blocks_promotion);
    assert!(outcome
        .active_stop_reason_tokens
        .contains(&"drill_owner_signoff_missing".to_owned()));
}

#[test]
fn no_rerun_path_narrows_without_a_structural_defect() {
    let mut input = seeded_continuity_freshness_slo_input();
    let target = row(&mut input, "continuity-row:sovereign-airgap-snapshot");
    target.rerun.rerun_class = RerunAutomationClass::NoRerunPath;
    target.rerun.rerun_class_token = RerunAutomationClass::NoRerunPath.as_str().to_owned();
    // A concrete ref is still named, so the structural rerun-declared check passes
    // while the operational rerun-path-unavailable stop reason narrows the claim.
    let dashboard = ContinuityFreshnessSloDashboard::new("d", "d", "2026-06-19T00:00:00Z", input);

    assert!(
        dashboard.is_structurally_clean(),
        "defects: {:?}",
        dashboard.defects
    );
    let outcome = dashboard
        .row_outcome("continuity-row:sovereign-airgap-snapshot")
        .expect("outcome");
    assert!(!outcome.rerun_automatable);
    assert!(outcome.narrowed);
    assert!(outcome.blocks_promotion);
    assert!(outcome
        .active_stop_reason_tokens
        .contains(&"rerun_path_unavailable".to_owned()));
}

#[test]
fn local_core_row_never_blocks_when_a_managed_row_goes_stale() {
    let mut input = seeded_continuity_freshness_slo_input();
    let managed = row(&mut input, "continuity-row:managed-cloud-sync");
    set_slo_state(managed, ContinuityFreshnessSloState::Breached);
    managed.proof_packet.captured_at = Some("2026-01-01".to_owned());
    let dashboard = ContinuityFreshnessSloDashboard::new("d", "d", "2026-06-19T00:00:00Z", input);

    // The managed row holds promotion ...
    assert_eq!(dashboard.promotion.decision, "hold");
    // ... but the local-core row stays within SLO and never blocks or narrows.
    let local = dashboard
        .row_outcome("continuity-row:local-desktop-core")
        .expect("local outcome");
    assert!(!local.in_release_scope);
    assert!(local.within_slo);
    assert!(!local.narrowed);
    assert!(!local.blocks_promotion);
    assert!(local.active_stop_reason_tokens.is_empty());
    assert!(!dashboard
        .promotion
        .blocked_row_ids
        .contains(&"continuity-row:local-desktop-core".to_owned()));
    assert!(dashboard.is_structurally_clean());
}

#[test]
fn inconsistent_freshness_window_is_a_defect() {
    let mut input = seeded_continuity_freshness_slo_input();
    let target = row(&mut input, "continuity-row:managed-cloud-sync");
    target.proof_packet.freshness_slo.warn_within_days =
        target.proof_packet.freshness_slo.target_max_age_days + 1;
    let dashboard = ContinuityFreshnessSloDashboard::new("d", "d", "2026-06-19T00:00:00Z", input);

    assert!(!dashboard.is_structurally_clean());
    assert!(dashboard
        .defects
        .iter()
        .any(|d| d.defect_kind == ContinuityFreshnessDefectKind::FreshnessWindowInconsistent));
    assert!(validate_continuity_freshness_slo_dashboard(&dashboard).is_err());
}

#[test]
fn undeclared_rerun_path_is_a_defect_for_release_scope_rows() {
    let mut input = seeded_continuity_freshness_slo_input();
    row(&mut input, "continuity-row:managed-cloud-sync")
        .rerun
        .rerun_command_ref = String::new();
    let dashboard = ContinuityFreshnessSloDashboard::new("d", "d", "2026-06-19T00:00:00Z", input);

    assert!(dashboard
        .defects
        .iter()
        .any(|d| d.defect_kind == ContinuityFreshnessDefectKind::RerunPathUndeclared));
}

#[test]
fn missing_stop_rule_is_a_defect() {
    let mut input = seeded_continuity_freshness_slo_input();
    input
        .stop_rules
        .retain(|rule| rule.trigger_reason != ContinuityStopReason::ContinuityPacketMissing);
    let dashboard = ContinuityFreshnessSloDashboard::new("d", "d", "2026-06-19T00:00:00Z", input);

    assert!(dashboard
        .defects
        .iter()
        .any(|d| d.defect_kind == ContinuityFreshnessDefectKind::StopReasonUncovered));
}

#[test]
fn support_export_excludes_raw_material_and_lists_stop_reasons() {
    let mut input = seeded_continuity_freshness_slo_input();
    let target = row(&mut input, "continuity-row:managed-cloud-sync");
    set_slo_state(target, ContinuityFreshnessSloState::Breached);
    target.proof_packet.captured_at = Some("2026-01-01".to_owned());
    let dashboard = ContinuityFreshnessSloDashboard::new("d", "d", "2026-06-19T00:00:00Z", input);

    let export = ContinuityFreshnessSloSupportExport::from_dashboard(
        "continuity:freshness-slo:support-export:fixture-001",
        "2026-06-19T00:00:00Z",
        dashboard,
    );
    assert!(export.raw_private_material_excluded);
    assert!(export
        .stop_reasons_present
        .contains(&"continuity_packet_freshness_breached".to_owned()));
}

#[test]
fn every_stop_reason_has_a_token_and_rule() {
    let input = seeded_continuity_freshness_slo_input();
    for reason in ContinuityStopReason::ALL {
        assert!(input
            .stop_rules
            .iter()
            .any(|rule| rule.trigger_reason == reason));
    }
}

#[test]
fn freshness_slo_state_ranks_are_ordered() {
    assert!(
        ContinuityFreshnessSloState::Current.freshness_rank()
            > ContinuityFreshnessSloState::DueForRefresh.freshness_rank()
    );
    assert!(
        ContinuityFreshnessSloState::DueForRefresh.freshness_rank()
            > ContinuityFreshnessSloState::Breached.freshness_rank()
    );
    assert!(
        ContinuityFreshnessSloState::Breached.freshness_rank()
            > ContinuityFreshnessSloState::Missing.freshness_rank()
    );
    assert!(ContinuityFreshnessSloState::Current.is_within_slo());
    assert!(ContinuityFreshnessSloState::Breached.forces_narrowing());
}
