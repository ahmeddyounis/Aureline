use super::*;
use crate::simulation::{audit_policy_simulation_beta_page, seeded_policy_simulation_beta_page};

fn page() -> EffectivePolicyStabilizePage {
    seeded_effective_policy_stabilize_page()
}

#[test]
fn seeded_page_seeds_zero_defects_and_qualifies_stable() {
    let page = page();
    assert_eq!(page.defects.len(), 0);
    assert!(validate_effective_policy_stabilize_page(&page).is_ok());
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        EffectivePolicyStabilizeQualificationClass::Stable.as_str()
    );
}

#[test]
fn seeded_page_passes_all_six_stability_conditions() {
    let page = page();
    assert!(page.covers_required_change_classes());
    assert!(page.exceptions_are_bounded_and_attributable());
    assert!(page.remembered_decisions_have_explained_drift());
    assert!(page.action_time_policy_truth_is_preserved());
    assert!(page.exception_preview_links_are_present());
}

#[test]
fn seeded_page_embeds_clean_simulation_beta_page() {
    let page = page();
    let upstream_defects = audit_policy_simulation_beta_page(&page.simulation_beta_page);
    assert_eq!(upstream_defects.len(), 0);
    assert!(crate::validate_policy_simulation_beta_page(&page.simulation_beta_page).is_ok());
}

#[test]
fn seeded_page_rows_are_all_stable() {
    let page = page();
    assert!(!page.rows.is_empty());
    for row in &page.rows {
        assert_eq!(
            row.qualification_token,
            EffectivePolicyStabilizeQualificationClass::Stable.as_str(),
            "row '{}' must qualify stable; got '{}'",
            row.simulation_id,
            row.qualification_token
        );
        assert_eq!(
            row.narrow_reason_token,
            EffectivePolicyStabilizeNarrowReasonClass::NotNarrowed.as_str(),
            "row '{}' must have not_narrowed reason",
            row.simulation_id
        );
    }
}

#[test]
fn seeded_page_rows_cover_both_required_change_classes() {
    let page = page();
    let change_classes: Vec<&str> = page.rows.iter().map(|r| r.change_class_token.as_str()).collect();
    assert!(
        change_classes.contains(&"policy_bundle_change"),
        "missing policy_bundle_change row"
    );
    assert!(
        change_classes.contains(&"settings_lock_change"),
        "missing settings_lock_change row"
    );
}

#[test]
fn seeded_page_summary_counts_match_rows() {
    let page = page();
    assert_eq!(page.summary.row_count, page.rows.len());
    assert_eq!(page.summary.stable_row_count, page.rows.len());
    assert_eq!(page.summary.beta_row_count, 0);
    assert_eq!(page.summary.withdrawn_row_count, 0);
    assert_eq!(page.summary.simulation_beta_page_defect_count, 0);
}

#[test]
fn seeded_page_expiring_exception_count_matches_summary() {
    let page = page();
    // The seeded page has one exception in ExpiringSoon status.
    assert_eq!(page.summary.expiring_exception_count, 1);
}

#[test]
fn seeded_page_rows_link_exceptions_and_remembered_decisions() {
    let page = page();
    let total_exception_refs: usize = page.rows.iter().map(|r| r.exception_preview_count).sum();
    let total_memory_refs: usize = page.rows.iter().map(|r| r.remembered_decision_count).sum();
    assert!(
        total_exception_refs > 0,
        "rows must link exception_preview_refs"
    );
    assert!(
        total_memory_refs > 0,
        "rows must link remembered_decision_preview_refs"
    );
}

#[test]
fn support_export_wraps_clean_page() {
    let page = page();
    let export = EffectivePolicyStabilizeSupportExport::from_page(
        "policy:stabilize-export:effective-policy:stable:0001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
    assert_eq!(export.page.summary.withdrawn_row_count, 0);
    assert!(export.simulation_support_export.preserves_historical_truth);
}

#[test]
fn injected_raw_material_in_exception_withdraws_packet() {
    let mut beta_page = seeded_policy_simulation_beta_page();
    // Inject the raw_justification_excluded = false to trigger withdrawal.
    beta_page.exceptions[0].raw_justification_excluded = false;
    // Re-audit the beta page so defects are updated.
    beta_page.defects = audit_policy_simulation_beta_page(&beta_page);
    let page = EffectivePolicyStabilizePage::new(
        "policy:effective_policy_stabilize:test",
        "test page",
        "2026-06-01T00:00:00Z",
        beta_page,
    );
    assert!(!page.qualifies_stable());
    assert!(!page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        EffectivePolicyStabilizeQualificationClass::Withdrawn.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == EffectivePolicyStabilizeNarrowReasonClass::RawPrivateMaterialExposed
    }));
}

#[test]
fn missing_policy_bundle_simulation_narrows_to_preview() {
    let mut beta_page = seeded_policy_simulation_beta_page();
    beta_page
        .simulations
        .retain(|s| s.request.change_class_token != "policy_bundle_change");
    beta_page.defects = audit_policy_simulation_beta_page(&beta_page);
    let page = EffectivePolicyStabilizePage::new(
        "policy:effective_policy_stabilize:test",
        "test page",
        "2026-06-01T00:00:00Z",
        beta_page,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        EffectivePolicyStabilizeQualificationClass::Preview.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == EffectivePolicyStabilizeNarrowReasonClass::RequiredChangeClassMissing
    }));
}

#[test]
fn exception_missing_expiry_narrows_to_beta() {
    let mut beta_page = seeded_policy_simulation_beta_page();
    beta_page.exceptions[0].time_horizon.expires_at.clear();
    beta_page.defects = audit_policy_simulation_beta_page(&beta_page);
    let page = EffectivePolicyStabilizePage::new(
        "policy:effective_policy_stabilize:test",
        "test page",
        "2026-06-01T00:00:00Z",
        beta_page,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        EffectivePolicyStabilizeQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == EffectivePolicyStabilizeNarrowReasonClass::ExceptionMissingBoundedExpiry
    }));
}

#[test]
fn unexplained_drift_narrows_to_beta() {
    let mut beta_page = seeded_policy_simulation_beta_page();
    // Force the first remembered decision to a state that requires a reason,
    // then clear the reasons to trigger the defect.
    beta_page.remembered_decisions[0].memory_state =
        crate::simulation::MemoryStateClass::ForceRetiredByPolicy;
    beta_page.remembered_decisions[0]
        .memory_state_token = crate::simulation::MemoryStateClass::ForceRetiredByPolicy
        .as_str()
        .to_owned();
    beta_page.remembered_decisions[0].invalidation_reasons.clear();
    beta_page.remembered_decisions[0]
        .invalidation_reason_tokens
        .clear();
    beta_page.defects = audit_policy_simulation_beta_page(&beta_page);
    let page = EffectivePolicyStabilizePage::new(
        "policy:effective_policy_stabilize:test",
        "test page",
        "2026-06-01T00:00:00Z",
        beta_page,
    );
    assert!(!page.qualifies_stable());
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason
            == EffectivePolicyStabilizeNarrowReasonClass::RememberedDecisionDriftUnexplained
    }));
}

#[test]
fn qualification_class_checks() {
    assert!(EffectivePolicyStabilizeQualificationClass::Stable.is_stable());
    assert!(!EffectivePolicyStabilizeQualificationClass::Beta.is_stable());
    assert!(!EffectivePolicyStabilizeQualificationClass::Withdrawn.is_stable());
    assert!(EffectivePolicyStabilizeQualificationClass::Stable.is_claimable());
    assert!(EffectivePolicyStabilizeQualificationClass::Beta.is_claimable());
    assert!(!EffectivePolicyStabilizeQualificationClass::Withdrawn.is_claimable());
}

#[test]
fn narrow_reason_withdrawal_sentinel_check() {
    assert!(
        EffectivePolicyStabilizeNarrowReasonClass::RawPrivateMaterialExposed
            .is_withdrawal_reason()
    );
    assert!(
        !EffectivePolicyStabilizeNarrowReasonClass::PolicySimulationBetaPageHasDefects
            .is_withdrawal_reason()
    );
    assert!(
        !EffectivePolicyStabilizeNarrowReasonClass::NotNarrowed.is_withdrawal_reason()
    );
}
