use super::*;

fn page() -> OperatorSupportContinuityPage {
    seeded_operator_support_continuity_page()
}

fn summary_mut<'a>(
    input: &'a mut OperatorSupportContinuityInput,
    summary_id: &str,
) -> &'a mut ContinuityRowSummary {
    input
        .summaries
        .iter_mut()
        .find(|summary| summary.summary_id == summary_id)
        .unwrap_or_else(|| panic!("missing seeded summary: {summary_id}"))
}

fn rebuild(input: OperatorSupportContinuityInput) -> OperatorSupportContinuityPage {
    OperatorSupportContinuityPage::new("p", "p", "2026-06-19T00:00:00Z", input)
}

#[test]
fn seeded_page_is_clean_and_stable() {
    let page = page();
    assert!(page.is_structurally_clean(), "defects: {:?}", page.defects);
    assert!(page.qualifies_stable());
    assert!(page.names_every_active_row());
    assert!(page.every_surface_covered());
    assert!(validate_operator_support_continuity_page(&page).is_ok());
    assert_eq!(audit_operator_support_continuity_page(&page).len(), 0);
}

#[test]
fn seeded_summary_counts_match_input() {
    let page = page();
    assert_eq!(page.summary.summary_count, 5);
    assert_eq!(page.summary.release_scope_count, 4);
    assert_eq!(page.summary.local_core_count, 1);
    assert_eq!(page.summary.impaired_count, 1);
    assert_eq!(page.summary.operational_count, 4);
    assert_eq!(page.summary.narrowed_count, 0);
    assert_eq!(page.summary.withheld_count, 0);
    assert_eq!(page.summary.stale_evidence_count, 0);
    assert_eq!(page.summary.missing_evidence_count, 0);
    assert_eq!(page.summary.export_safe_count, 5);
    assert_eq!(page.summary.surfaces_fully_covered_count, 5);
    assert_eq!(page.summary.overall_qualification_token, "stable");
}

#[test]
fn truthfully_degraded_row_names_exact_fallback_and_stays_stable() {
    let page = page();
    let outcome = page
        .summary_outcome("continuity:operator-support:managed-relay")
        .expect("relay outcome");
    assert!(outcome.impaired);
    assert!(!outcome.narrowed);
    assert!(!outcome.withheld);
    assert_eq!(outcome.severity_token, "degraded");
    assert_eq!(
        outcome.degraded_state_token,
        "control_plane_impaired_local_core_preserved"
    );
    assert_eq!(outcome.narrower_fallback_token, "queue_and_reconcile");
    assert_eq!(outcome.effective_qualification_token, "stable");
}

#[test]
fn generic_degraded_wording_withholds_the_summary() {
    let mut input = seeded_operator_support_continuity_input();
    let relay = summary_mut(&mut input, "continuity:operator-support:managed-relay");
    relay.outage.status_phrasing = "Service degraded.".to_owned();
    let page = rebuild(input);

    let outcome = page
        .summary_outcome("continuity:operator-support:managed-relay")
        .expect("outcome");
    assert!(outcome.narrowed);
    assert!(outcome.withheld);
    assert_eq!(outcome.effective_qualification_token, "withdrawn");
    assert!(outcome
        .narrow_reason_tokens
        .contains(&"generic_degraded_wording_used".to_owned()));
    assert!(!page.is_structurally_clean());
    assert!(validate_operator_support_continuity_page(&page).is_err());
}

#[test]
fn undisclosed_locality_narrows_to_beta() {
    let mut input = seeded_operator_support_continuity_input();
    let row = summary_mut(&mut input, "continuity:operator-support:managed-cloud-sync");
    row.posture.storage_locality = LocalityClass::Undisclosed;
    row.posture.storage_locality_token = LocalityClass::Undisclosed.as_str().to_owned();
    let page = rebuild(input);

    let outcome = page
        .summary_outcome("continuity:operator-support:managed-cloud-sync")
        .expect("outcome");
    assert!(outcome.narrowed);
    assert_eq!(outcome.effective_qualification_token, "beta");
    assert!(outcome
        .narrow_reason_tokens
        .contains(&"locality_posture_missing".to_owned()));
}

#[test]
fn stale_evidence_narrows_to_beta() {
    let mut input = seeded_operator_support_continuity_input();
    let row = summary_mut(
        &mut input,
        "continuity:operator-support:self-hosted-restore",
    );
    row.evidence.evidence_state = OutageEvidenceStateClass::StaleNeedsRefresh;
    row.evidence.evidence_state_token = OutageEvidenceStateClass::StaleNeedsRefresh
        .as_str()
        .to_owned();
    let page = rebuild(input);

    let outcome = page
        .summary_outcome("continuity:operator-support:self-hosted-restore")
        .expect("outcome");
    assert!(outcome.narrowed);
    assert_eq!(outcome.effective_qualification_token, "beta");
    assert!(outcome
        .narrow_reason_tokens
        .contains(&"canonical_summary_stale".to_owned()));
    assert_eq!(page.summary.stale_evidence_count, 1);
}

#[test]
fn missing_evidence_narrows_to_preview() {
    let mut input = seeded_operator_support_continuity_input();
    let row = summary_mut(&mut input, "continuity:operator-support:sovereign-airgap");
    row.evidence.evidence_state = OutageEvidenceStateClass::Missing;
    row.evidence.evidence_state_token = OutageEvidenceStateClass::Missing.as_str().to_owned();
    row.evidence.last_refreshed_at = String::new();
    let page = rebuild(input);

    let outcome = page
        .summary_outcome("continuity:operator-support:sovereign-airgap")
        .expect("outcome");
    assert!(outcome.narrowed);
    assert_eq!(outcome.effective_qualification_token, "preview");
    assert!(outcome
        .narrow_reason_tokens
        .contains(&"canonical_summary_missing".to_owned()));
    assert_eq!(page.summary.missing_evidence_count, 1);
}

#[test]
fn admin_only_material_withholds_the_summary() {
    let mut input = seeded_operator_support_continuity_input();
    let row = summary_mut(&mut input, "continuity:operator-support:managed-cloud-sync");
    row.redaction = SummaryRedaction::new(false, true);
    let page = rebuild(input);

    let outcome = page
        .summary_outcome("continuity:operator-support:managed-cloud-sync")
        .expect("outcome");
    assert!(!outcome.export_safe);
    assert!(outcome.withheld);
    assert_eq!(outcome.effective_qualification_token, "withdrawn");
    assert!(outcome
        .narrow_reason_tokens
        .contains(&"admin_only_material_leaked".to_owned()));
}

#[test]
fn impaired_lane_without_fallback_narrows() {
    let mut input = seeded_operator_support_continuity_input();
    let relay = summary_mut(&mut input, "continuity:operator-support:managed-relay");
    relay.outage.narrower_fallback = DegradedFallbackClass::NotDeclared;
    relay.outage.narrower_fallback_token = DegradedFallbackClass::NotDeclared.as_str().to_owned();
    let page = rebuild(input);

    let outcome = page
        .summary_outcome("continuity:operator-support:managed-relay")
        .expect("outcome");
    assert!(outcome.narrowed);
    assert!(outcome
        .narrow_reason_tokens
        .contains(&"narrower_fallback_undeclared".to_owned()));
}

#[test]
fn local_core_summary_never_narrows_when_a_managed_row_breaks() {
    let mut input = seeded_operator_support_continuity_input();
    // A managed row loses its backing evidence ...
    let managed = summary_mut(&mut input, "continuity:operator-support:managed-cloud-sync");
    managed.evidence.evidence_state = OutageEvidenceStateClass::Missing;
    managed.evidence.evidence_state_token = OutageEvidenceStateClass::Missing.as_str().to_owned();
    managed.evidence.last_refreshed_at = String::new();
    let page = rebuild(input);

    // ... the managed row narrows ...
    let managed_outcome = page
        .summary_outcome("continuity:operator-support:managed-cloud-sync")
        .expect("managed outcome");
    assert!(managed_outcome.narrowed);

    // ... but the local-core summary stays stable and export-safe.
    let local = page
        .summary_outcome("continuity:operator-support:local-desktop-core")
        .expect("local outcome");
    assert!(!local.in_release_scope);
    assert!(!local.narrowed);
    assert!(!local.withheld);
    assert_eq!(local.effective_qualification_token, "stable");
    assert!(local.narrow_reason_tokens.is_empty());
}

#[test]
fn unnamed_active_row_narrows_to_preview() {
    let mut input = seeded_operator_support_continuity_input();
    let row = summary_mut(&mut input, "continuity:operator-support:managed-cloud-sync");
    row.active_continuity_row_id = String::new();
    row.active_continuity_row_label = String::new();
    let page = rebuild(input);

    assert!(!page.names_every_active_row());
    let outcome = page
        .summary_outcome("continuity:operator-support:managed-cloud-sync")
        .expect("outcome");
    assert_eq!(outcome.effective_qualification_token, "preview");
    assert!(outcome
        .narrow_reason_tokens
        .contains(&"active_continuity_row_unnamed".to_owned()));
}

#[test]
fn incomplete_surface_reuse_narrows_release_scope_row() {
    let mut input = seeded_operator_support_continuity_input();
    let row = summary_mut(
        &mut input,
        "continuity:operator-support:self-hosted-restore",
    );
    row.surface_coverage.support_export = false;
    let page = rebuild(input);

    let outcome = page
        .summary_outcome("continuity:operator-support:self-hosted-restore")
        .expect("outcome");
    assert!(outcome.narrowed);
    assert!(outcome
        .narrow_reason_tokens
        .contains(&"surface_reuse_incomplete".to_owned()));
}

#[test]
fn sovereign_shared_tenant_is_a_profile_mismatch() {
    let mut input = seeded_operator_support_continuity_input();
    let row = summary_mut(&mut input, "continuity:operator-support:sovereign-airgap");
    row.posture.tenant_scope = TenantScopeClass::SharedMultiTenant;
    row.posture.tenant_scope_token = TenantScopeClass::SharedMultiTenant.as_str().to_owned();
    let page = rebuild(input);

    let outcome = page
        .summary_outcome("continuity:operator-support:sovereign-airgap")
        .expect("outcome");
    assert!(outcome.narrowed);
    assert!(outcome
        .narrow_reason_tokens
        .contains(&"profile_mismatch".to_owned()));
    assert_eq!(outcome.effective_qualification_token, "preview");
}

#[test]
fn support_export_excludes_raw_material_and_lists_reasons() {
    let mut input = seeded_operator_support_continuity_input();
    let relay = summary_mut(&mut input, "continuity:operator-support:managed-relay");
    relay.outage.status_phrasing = "Service degraded.".to_owned();
    let page = rebuild(input);

    let export = OperatorSupportContinuitySupportExport::from_page(
        "continuity:operator-support:support-export:fixture-001",
        "2026-06-19T00:00:00Z",
        page,
    );
    assert!(export.raw_private_material_excluded);
    assert!(export
        .narrow_reasons_present
        .contains(&SummaryNarrowReasonClass::GenericDegradedWordingUsed));
    assert!(export
        .defect_counts_by_narrow_reason
        .contains_key("generic_degraded_wording_used"));
}

#[test]
fn every_narrow_reason_has_a_stable_token() {
    for reason in SummaryNarrowReasonClass::ALL {
        assert!(!reason.as_str().is_empty());
    }
}
