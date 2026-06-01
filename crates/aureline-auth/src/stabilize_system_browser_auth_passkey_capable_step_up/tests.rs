use super::*;
use crate::passkey::validate_passkey_step_up_beta_page;
use crate::system_browser::beta::validate_system_browser_return_paths_beta_page;

fn page() -> SystemBrowserAuthStabilizePage {
    seeded_system_browser_auth_stabilize_page()
}

#[test]
fn seeded_page_seeds_zero_defects_and_qualifies_stable() {
    let page = page();
    assert_eq!(page.defects.len(), 0);
    assert!(validate_system_browser_auth_stabilize_page(&page).is_ok());
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        SystemBrowserAuthStabilizeQualificationClass::Stable.as_str()
    );
}

#[test]
fn seeded_page_passes_all_four_stability_conditions() {
    let page = page();
    assert!(page.defaults_to_system_browser_or_explicit_exception());
    assert!(page.passkey_step_up_present_when_claimed());
    assert!(page.reauth_and_recovery_preserve_target_action_identity());
    assert!(page.fallback_named_when_passkey_unavailable());
}

#[test]
fn seeded_page_embeds_clean_return_paths_beta_page() {
    let page = page();
    assert_eq!(page.return_paths_beta_page.defects.len(), 0);
    assert!(
        validate_system_browser_return_paths_beta_page(&page.return_paths_beta_page).is_ok()
    );
}

#[test]
fn seeded_page_embeds_clean_passkey_step_up_beta_page() {
    let page = page();
    assert_eq!(page.passkey_step_up_beta_page.defects.len(), 0);
    assert!(validate_passkey_step_up_beta_page(&page.passkey_step_up_beta_page).is_ok());
}

#[test]
fn seeded_page_rows_are_all_stable() {
    let page = page();
    assert!(!page.rows.is_empty());
    for row in &page.rows {
        assert_eq!(
            row.qualification_token,
            SystemBrowserAuthStabilizeQualificationClass::Stable.as_str(),
            "row '{}' must qualify stable; got '{}'",
            row.return_path_row_id,
            row.qualification_token
        );
        assert_eq!(
            row.narrow_reason_token,
            SystemBrowserAuthStabilizeNarrowReasonClass::NotNarrowed.as_str(),
            "row '{}' must have not_narrowed reason",
            row.return_path_row_id
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
    assert_eq!(page.summary.return_path_beta_defect_count, 0);
    assert_eq!(page.summary.passkey_beta_defect_count, 0);
}

#[test]
fn support_export_wraps_clean_page() {
    let page = page();
    let export = SystemBrowserAuthStabilizeSupportExport::from_page(
        "auth:stabilize-export:system-browser-auth:stable:0001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
    assert_eq!(export.page.summary.withdrawn_row_count, 0);
}

#[test]
fn injected_authority_widening_in_return_paths_page_withdraws_row() {
    use crate::system_browser::beta::{
        audit_rows, SystemBrowserReturnPathBetaSupportRow, SystemBrowserReturnPathsBetaPage,
    };

    let mut rp_page = seeded_system_browser_return_paths_beta_page();
    // Inject an authority-widening on the passkey-capable row.
    let row = rp_page
        .rows
        .iter_mut()
        .find(|r| r.passkey_capability_claimed)
        .unwrap();
    row.granted_authority_scope_token =
        crate::system_browser::beta::AuthorityScopeClass::TenantAdminScope
            .as_str()
            .to_owned();
    let support_rows: Vec<SystemBrowserReturnPathBetaSupportRow> =
        rp_page.rows.iter().map(SystemBrowserReturnPathBetaSupportRow::from_row).collect();
    let defects = audit_rows(&rp_page.rows, &support_rows);
    // Rebuild the page with injected defects to simulate a dirty page.
    let dirty_rp_page = SystemBrowserReturnPathsBetaPage {
        defects,
        ..rp_page
    };
    let passkey_page = seeded_passkey_step_up_beta_page();
    let page = SystemBrowserAuthStabilizePage::new(
        "auth:system_browser_auth_stabilize:test",
        "test page",
        "2026-06-01T00:00:00Z",
        dirty_rp_page,
        passkey_page,
    );
    assert!(!page.qualifies_stable());
    assert!(!page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        SystemBrowserAuthStabilizeQualificationClass::Withdrawn.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == SystemBrowserAuthStabilizeNarrowReasonClass::AuthorityWideningOnReturn
    }));
}

#[test]
fn injected_identity_widening_in_passkey_page_withdraws_row() {
    use crate::passkey::{
        audit_passkey_step_up_beta_rows, PasskeyBetaLaneClass, PasskeyStepUpBetaPage,
        PasskeyStepUpBetaSupportRow, PasskeyTargetActionPreservationClass,
    };

    let mut pk_page = seeded_passkey_step_up_beta_page();
    // Inject a widening on the recovery row.
    let row = pk_page
        .rows
        .iter_mut()
        .find(|r| r.lane.lane_token == PasskeyBetaLaneClass::RecoveryLane.as_str())
        .unwrap();
    row.target_action_preservation.preservation_token =
        PasskeyTargetActionPreservationClass::TargetActionWidened
            .as_str()
            .to_owned();
    let support_rows: Vec<PasskeyStepUpBetaSupportRow> =
        pk_page.rows.iter().map(PasskeyStepUpBetaSupportRow::from_row).collect();
    let defects = audit_passkey_step_up_beta_rows(&pk_page.rows, &support_rows);
    let dirty_pk_page = PasskeyStepUpBetaPage {
        defects,
        ..pk_page
    };
    let rp_page = seeded_system_browser_return_paths_beta_page();
    let page = SystemBrowserAuthStabilizePage::new(
        "auth:system_browser_auth_stabilize:test",
        "test page",
        "2026-06-01T00:00:00Z",
        rp_page,
        dirty_pk_page,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        SystemBrowserAuthStabilizeQualificationClass::Withdrawn.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == SystemBrowserAuthStabilizeNarrowReasonClass::IdentityWideningOnReturn
    }));
}

#[test]
fn narrow_reason_withdrawal_sentinel_check() {
    assert!(SystemBrowserAuthStabilizeNarrowReasonClass::AuthorityWideningOnReturn.is_withdrawal_reason());
    assert!(SystemBrowserAuthStabilizeNarrowReasonClass::IdentityWideningOnReturn.is_withdrawal_reason());
    assert!(!SystemBrowserAuthStabilizeNarrowReasonClass::ReturnPathsBetaPageHasDefects.is_withdrawal_reason());
    assert!(!SystemBrowserAuthStabilizeNarrowReasonClass::NotNarrowed.is_withdrawal_reason());
}

#[test]
fn qualification_class_stable_checks() {
    assert!(SystemBrowserAuthStabilizeQualificationClass::Stable.is_stable());
    assert!(!SystemBrowserAuthStabilizeQualificationClass::Beta.is_stable());
    assert!(!SystemBrowserAuthStabilizeQualificationClass::Withdrawn.is_stable());
    assert!(SystemBrowserAuthStabilizeQualificationClass::Stable.is_claimable());
    assert!(SystemBrowserAuthStabilizeQualificationClass::Beta.is_claimable());
    assert!(!SystemBrowserAuthStabilizeQualificationClass::Withdrawn.is_claimable());
}
