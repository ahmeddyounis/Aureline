//! Shell consumer for the beta OIDC system-browser sign-in, recovery, and
//! session-continuity projection.
//!
//! The shell does not mint a parallel OIDC model. It consumes the auth-owned
//! [`aureline_auth::seeded_oidc_system_browser_beta_page`] projection, adds a
//! compact rendering summary, and exposes the same records to the headless
//! inspector, support export, settings root, activity center, and reviewer
//! fixtures.

use serde::{Deserialize, Serialize};

pub use aureline_auth::{
    audit_oidc_system_browser_beta_rows, seeded_oidc_system_browser_beta_page,
    validate_oidc_system_browser_beta_page, OidcAuthorityScopeClass, OidcIdentityOutageBlock,
    OidcIdentityOutageClass, OidcIssuerDisclosure, OidcIssuerSourceClass, OidcRecoveryActionClass,
    OidcReturnPathLabel, OidcSessionContinuityBlock, OidcSessionStateClass,
    OidcSignOutContinuityClass, OidcSystemBrowserBetaAxis, OidcSystemBrowserBetaDefect,
    OidcSystemBrowserBetaDefectKind, OidcSystemBrowserBetaPage,
    OidcSystemBrowserBetaProfileClass, OidcSystemBrowserBetaRow, OidcSystemBrowserBetaSummary,
    OidcSystemBrowserBetaSupportExport, OidcSystemBrowserBetaSupportRow, OidcTenantBinding,
    OidcTenantBindingClass, OIDC_SYSTEM_BROWSER_BETA_SCHEMA_VERSION,
    OIDC_SYSTEM_BROWSER_BETA_SHARED_CONTRACT_REF,
};

/// Stable record kind for [`OidcSystemBrowserBetaRenderSummary`] payloads.
pub const OIDC_SYSTEM_BROWSER_BETA_RENDER_RECORD_KIND: &str =
    "shell_oidc_system_browser_beta_render_record";

/// Shell-facing rendering summary for the beta OIDC page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcSystemBrowserBetaRenderSummary {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub row_count: usize,
    pub claimed_enterprise_row_count: usize,
    pub outage_or_denial_row_count: usize,
    pub signed_out_row_count: usize,
    pub profiles_present: Vec<String>,
    pub issuer_sources_present: Vec<String>,
    pub session_states_present: Vec<String>,
    pub recovery_actions_present: Vec<String>,
    pub defect_count: usize,
}

impl OidcSystemBrowserBetaRenderSummary {
    /// Build the summary from the beta page.
    pub fn from_page(page: &OidcSystemBrowserBetaPage) -> Self {
        Self {
            record_kind: OIDC_SYSTEM_BROWSER_BETA_RENDER_RECORD_KIND.to_owned(),
            schema_version: OIDC_SYSTEM_BROWSER_BETA_SCHEMA_VERSION,
            shared_contract_ref: OIDC_SYSTEM_BROWSER_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_count: page.summary.row_count,
            claimed_enterprise_row_count: page.summary.claimed_enterprise_row_count,
            outage_or_denial_row_count: page.summary.outage_or_denial_row_count,
            signed_out_row_count: page.summary.signed_out_row_count,
            profiles_present: page.summary.profiles_present.clone(),
            issuer_sources_present: page.summary.issuer_sources_present.clone(),
            session_states_present: page.summary.session_states_present.clone(),
            recovery_actions_present: page.summary.recovery_actions_present.clone(),
            defect_count: page.summary.defect_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_and_summary_covers_states() {
        let page = seeded_oidc_system_browser_beta_page();
        validate_oidc_system_browser_beta_page(&page).expect("seeded page validates");
        let summary = OidcSystemBrowserBetaRenderSummary::from_page(&page);
        assert_eq!(summary.row_count, page.rows.len());
        assert_eq!(summary.defect_count, 0);
        assert!(summary.claimed_enterprise_row_count >= 1);
        assert!(summary.outage_or_denial_row_count >= 1);
        assert!(summary
            .session_states_present
            .contains(&"signed_in_active".to_owned()));
        assert!(summary
            .session_states_present
            .contains(&"identity_outage_managed_blocked".to_owned()));
        assert!(summary
            .session_states_present
            .contains(&"signed_out_local_intact".to_owned()));
        assert!(summary
            .issuer_sources_present
            .contains(&"managed_enterprise_issuer".to_owned()));
    }

    #[test]
    fn support_export_round_trips_through_validator() {
        let page = seeded_oidc_system_browser_beta_page();
        let export = OidcSystemBrowserBetaSupportExport::from_page(
            "support-export:oidc-system-browser:shell",
            "2026-05-16T00:00:00Z",
            page,
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
    }
}
