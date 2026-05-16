//! Shell consumer for the beta system-browser default + passkey step-up +
//! return-path labeling projection.
//!
//! This module is the live-shell view onto the auth-owned beta page minted in
//! [`aureline_auth::seeded_system_browser_return_paths_beta_page`]. It does
//! NOT mint a parallel claim vocabulary; it builds the page from the
//! auth-owned seeded inputs, adds a tiny shell-facing rendering summary, and
//! exposes the same page + support-export records to the headless inspector
//! and the diagnostics surface.
//!
//! Settings root, Help/About, activity center, and support-export packets all
//! read these records, so UI, CLI/headless inspection, and docs use the same
//! claim, return-mode, and passkey-posture vocabulary.

use serde::{Deserialize, Serialize};

pub use aureline_auth::{
    audit_system_browser_return_paths_beta_rows, seeded_system_browser_return_paths_beta_page,
    validate_system_browser_return_paths_beta_page, AuthorityScopeClass, PasskeyStepUpBlock,
    PasskeyStepUpPostureClass, ReturnPathLabel, SystemBrowserPolicyExceptionClass,
    SystemBrowserReturnPathBetaAxis, SystemBrowserReturnPathBetaDefect,
    SystemBrowserReturnPathBetaDefectKind, SystemBrowserReturnPathBetaRow,
    SystemBrowserReturnPathBetaSupportRow, SystemBrowserReturnPathsBetaPage,
    SystemBrowserReturnPathsBetaSummary, SystemBrowserReturnPathsBetaSupportExport,
    SYSTEM_BROWSER_RETURN_PATHS_BETA_SCHEMA_VERSION,
    SYSTEM_BROWSER_RETURN_PATHS_BETA_SHARED_CONTRACT_REF,
};

/// Record-kind tag carried on serialized [`SystemBrowserReturnPathsRenderSummary`] payloads.
pub const SYSTEM_BROWSER_RETURN_PATHS_RENDER_RECORD_KIND: &str =
    "shell_system_browser_return_paths_beta_render_record";

/// Shell-facing rendering summary for the beta page.
///
/// This is the row count and posture breakdown the shell renders into the
/// activity center, settings root, and Help/About surfaces. It does not own
/// the claim vocabulary — every count is keyed by the closed token from
/// `aureline-auth`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemBrowserReturnPathsRenderSummary {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub row_count: usize,
    pub system_browser_default_row_count: usize,
    pub explicit_exception_row_count: usize,
    pub passkey_capable_row_count: usize,
    pub return_modes_present: Vec<String>,
    pub policy_exceptions_present: Vec<String>,
    pub passkey_postures_present: Vec<String>,
    pub defect_count: usize,
}

impl SystemBrowserReturnPathsRenderSummary {
    /// Build the summary from a beta page.
    pub fn from_page(page: &SystemBrowserReturnPathsBetaPage) -> Self {
        Self {
            record_kind: SYSTEM_BROWSER_RETURN_PATHS_RENDER_RECORD_KIND.to_owned(),
            schema_version: SYSTEM_BROWSER_RETURN_PATHS_BETA_SCHEMA_VERSION,
            shared_contract_ref: SYSTEM_BROWSER_RETURN_PATHS_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_count: page.summary.row_count,
            system_browser_default_row_count: page.summary.system_browser_default_row_count,
            explicit_exception_row_count: page.summary.explicit_exception_row_count,
            passkey_capable_row_count: page.summary.passkey_capable_row_count,
            return_modes_present: page.summary.return_modes_present.clone(),
            policy_exceptions_present: page.summary.policy_exceptions_present.clone(),
            passkey_postures_present: page.summary.passkey_postures_present.clone(),
            defect_count: page.summary.defect_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_and_summary_covers_postures() {
        let page = seeded_system_browser_return_paths_beta_page();
        validate_system_browser_return_paths_beta_page(&page).expect("seeded page validates");
        let summary = SystemBrowserReturnPathsRenderSummary::from_page(&page);
        assert_eq!(summary.row_count, page.rows.len());
        assert_eq!(summary.defect_count, 0);
        assert!(summary.system_browser_default_row_count >= 1);
        assert!(summary.passkey_capable_row_count >= 1);
        assert!(summary
            .policy_exceptions_present
            .contains(&"system_browser_default_no_exception".to_owned()));
        assert!(summary
            .policy_exceptions_present
            .contains(&"admin_policy_device_code_required".to_owned()));
        assert!(summary
            .passkey_postures_present
            .contains(&"passkey_capable_offered".to_owned()));
    }

    #[test]
    fn support_export_round_trips_through_validator() {
        let page = seeded_system_browser_return_paths_beta_page();
        let export = SystemBrowserReturnPathsBetaSupportExport::from_page(
            "support-export:system-browser-return-paths:shell",
            "2026-05-15T00:00:00Z",
            page,
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
    }
}
