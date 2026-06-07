use super::*;
use crate::simulation::{audit_policy_simulation_beta_page, seeded_policy_simulation_beta_page};

fn page() -> PolicySimulationAndExpiryPage {
    seeded_policy_simulation_and_expiry_page()
}

#[test]
fn seeded_page_qualifies_stable() {
    let page = page();
    assert!(
        page.qualifies_stable(),
        "seeded page must qualify stable; defects: {:?}",
        page.defects
    );
    assert!(validate_policy_simulation_and_expiry_page(&page).is_ok());
    assert_eq!(
        page.summary.overall_qualification_token,
        PolicySimulationAndExpiryQualificationClass::Stable.as_str()
    );
}

#[test]
fn seeded_page_populates_all_stable_objects() {
    let page = page();
    assert!(!page.simulation_views.is_empty());
    assert!(!page.exception_preview_sheets.is_empty());
    assert!(!page.approval_history_rows.is_empty());
    assert!(!page.policy_diff_summaries.is_empty());
    assert!(!page.expiry_banners.is_empty());
    assert_eq!(
        page.summary.simulation_view_count,
        page.source_policy_simulation_page.simulations.len()
    );
}

#[test]
fn seeded_views_preview_policy_diff_consequence_and_expiry_links() {
    let page = page();
    for view in &page.simulation_views {
        assert!(
            !view.changed_keys_or_feature_areas.is_empty(),
            "view '{}' must list changed policy keys or feature areas",
            view.simulation_view_id
        );
        assert!(!view.previous_value_ref.is_empty());
        assert!(!view.simulated_value_ref.is_empty());
        assert!(!view.user_visible_consequence.is_empty());
        assert!(!view.affected_surface_refs.is_empty());
        assert!(!view.degraded_mode_consequences.is_empty());
        assert!(!view.stale_offline_notes.is_empty());
        assert!(!view.policy_diff_summary_refs.is_empty());
        assert!(!view.expiry_banner_refs.is_empty());
        assert!(!view.export_safe_lineage_refs.is_empty());
    }
}

#[test]
fn seeded_exception_sheets_name_bypass_owner_evidence_and_fallback() {
    let page = page();
    for sheet in &page.exception_preview_sheets {
        assert!(!sheet.exact_bypass_scope.is_empty());
        assert!(!sheet.scope_ref.is_empty());
        assert!(!sheet.created_at.is_empty());
        assert!(!sheet.owner_or_approver_ref.is_empty());
        assert!(!sheet.reason.is_empty());
        assert!(!sheet.mitigation.is_empty());
        assert!(!sheet.evidence_link_refs.is_empty());
        assert!(!sheet.expiry_target_at.is_empty());
        assert!(!sheet.fallback_behavior_on_lapse.is_empty());
        assert!(!sheet.export_safe_lineage_refs.is_empty());
    }
}

#[test]
fn seeded_approval_history_rows_are_bounded_and_revalidate_on_drift() {
    let page = page();
    assert!(page.approval_history_revalidates_on_material_drift());
    for row in &page.approval_history_rows {
        assert!(row.bounded_by_expiry);
        assert!(!row.source_ref.is_empty());
        assert!(!row.scope_ref.is_empty());
        assert!(!row.created_at.is_empty());
        assert!(!row.expires_at.is_empty());
        assert!(!row.review_target_at.is_empty());
        assert!(!row.revoke_action_ref.is_empty());
        assert!(!row.open_details_action_ref.is_empty());
        assert_eq!(row.reapproval_triggers.len(), 4);
        assert_eq!(row.reapproval_trigger_tokens.len(), 4);
        assert!(!row.export_safe_lineage_refs.is_empty());
    }
}

#[test]
fn seeded_diff_summaries_and_banners_have_stable_envelopes() {
    let page = page();
    for summary in &page.policy_diff_summaries {
        assert!(!summary.actor_ref.is_empty());
        assert!(!summary.source_ref.is_empty());
        assert!(!summary.scope_ref.is_empty());
        assert!(!summary.created_at.is_empty());
        assert!(!summary.review_target_at.is_empty());
        assert!(!summary.export_safe_lineage_refs.is_empty());
    }
    for banner in &page.expiry_banners {
        assert!(!banner.actor_or_source_ref.is_empty());
        assert!(!banner.scope_ref.is_empty());
        assert!(!banner.created_at.is_empty());
        assert!(!banner.exact_expiry_at.is_empty());
        assert!(!banner.export_safe_lineage_refs.is_empty());
    }
}

#[test]
fn seeded_review_packet_covers_required_projection_surfaces() {
    let page = page();
    assert!(page.covers_required_projection_surfaces());
    assert_eq!(page.review_packet.surface_projections.len(), 3);
    assert!(!page.review_packet.policy_diff_summary_refs.is_empty());
    assert!(!page.review_packet.simulation_outcome_refs.is_empty());
    assert!(!page.review_packet.approval_or_waiver_owner_refs.is_empty());
    assert!(!page.review_packet.expiry_banner_refs.is_empty());
    assert!(!page.review_packet.chronology_refs.is_empty());
    assert_eq!(page.review_packet.reapproval_trigger_tokens.len(), 4);
}

#[test]
fn support_export_wraps_stable_page() {
    let page = page();
    let export = PolicySimulationAndExpirySupportExport::from_page(
        "policy:simulation-expiry:support-export:fixture-001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.defect_counts_by_kind.is_empty());
    assert!(export.page.qualifies_stable());
}

#[test]
fn source_raw_private_material_withdraws_packet() {
    let mut beta_page = seeded_policy_simulation_beta_page();
    beta_page.exceptions[0].raw_justification_excluded = false;
    beta_page.defects = audit_policy_simulation_beta_page(&beta_page);
    let page = PolicySimulationAndExpiryPage::from_policy_simulation_page(
        "policy:simulation-expiry:drill:raw-private-material",
        "Drill - raw private material",
        "2026-06-01T00:00:00Z",
        beta_page,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        PolicySimulationAndExpiryQualificationClass::Withdrawn.as_str()
    );
    assert!(page.defects.iter().any(|defect| defect.defect_kind
        == PolicySimulationAndExpiryDefectKind::RawPrivateMaterialExposed));
}

#[test]
fn missing_diff_summary_narrows_to_review() {
    let mut page = page();
    page.policy_diff_summaries.clear();
    for view in page.simulation_views.iter_mut() {
        view.policy_diff_summary_refs.clear();
    }
    let defects = audit_policy_simulation_and_expiry_page(&page);
    assert!(defects.iter().any(|defect| defect.defect_kind
        == PolicySimulationAndExpiryDefectKind::SimulationViewMissingDiffSummary));
}

#[test]
fn indefinite_high_risk_memory_narrows_to_review() {
    let mut page = page();
    page.approval_history_rows[0].expires_at.clear();
    page.approval_history_rows[0].bounded_by_expiry = false;
    let defects = audit_policy_simulation_and_expiry_page(&page);
    assert!(defects.iter().any(|defect| defect.defect_kind
        == PolicySimulationAndExpiryDefectKind::IndefiniteHighRiskRememberedDecision));
}

#[test]
fn missing_projection_surface_narrows_to_review() {
    let mut page = page();
    page.review_packet.surface_projections.pop();
    page.review_packet.surface_projection_tokens.pop();
    let defects = audit_policy_simulation_and_expiry_page(&page);
    assert!(defects.iter().any(|defect| defect.defect_kind
        == PolicySimulationAndExpiryDefectKind::CrossSurfaceProjectionMissing));
}
