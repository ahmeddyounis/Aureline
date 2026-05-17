//! Shell consumer for the offline policy-bundle and entitlement verifier
//! beta page.
//!
//! The shell does not mint a parallel verifier. It consumes the auth-owned
//! [`aureline_auth::seeded_offline_entitlement_verifier_beta_page`]
//! projection, adds a compact rendering summary, and exposes the same
//! records to the headless inspector, support export, admin/settings
//! center, and reviewer fixtures.

use serde::{Deserialize, Serialize};

pub use aureline_auth::{
    audit_offline_entitlement_verifier_beta_rows, seeded_offline_entitlement_verifier_beta_page,
    validate_offline_entitlement_verifier_beta_page, LocalEditingPreservationClass,
    ManagedCapabilityImpactClass, OfflineEntitlementVerifierBetaDefect,
    OfflineEntitlementVerifierBetaDefectKind, OfflineEntitlementVerifierBetaPage,
    OfflineEntitlementVerifierBetaProfileClass, OfflineEntitlementVerifierBetaRow,
    OfflineEntitlementVerifierBetaSummary, OfflineEntitlementVerifierBetaSupportExport,
    OfflineEntitlementVerifierBetaSupportRow, TrustAnchorSourceClass, VerifiedBundleKindClass,
    VerifierBundleSubject, VerifierOutcomeClass, VerifierRecoveryActionClass, VerifierTrustAnchor,
    OFFLINE_ENTITLEMENT_VERIFIER_BETA_SCHEMA_VERSION,
    OFFLINE_ENTITLEMENT_VERIFIER_BETA_SHARED_CONTRACT_REF,
};

/// Stable record kind for [`OfflineEntitlementVerifierBetaRenderSummary`] payloads.
pub const OFFLINE_ENTITLEMENT_VERIFIER_BETA_RENDER_RECORD_KIND: &str =
    "shell_offline_entitlement_verifier_beta_render_record";

/// Shell-facing rendering summary for the beta verifier page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineEntitlementVerifierBetaRenderSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Number of rows rendered.
    pub row_count: usize,
    /// Number of rows whose outcome verifies cleanly.
    pub verified_row_count: usize,
    /// Number of rows whose outcome requires a downgrade.
    pub failed_row_count: usize,
    /// Number of rows that preserve local editing.
    pub local_editing_preserved_row_count: usize,
    /// Number of rows that narrow managed authority.
    pub managed_authority_narrowed_row_count: usize,
    /// Profile tokens rendered by the shell.
    pub profiles_present: Vec<String>,
    /// Bundle-kind tokens rendered by the shell.
    pub bundle_kinds_present: Vec<String>,
    /// Outcome tokens rendered by the shell.
    pub outcomes_present: Vec<String>,
    /// Number of validator defects.
    pub defect_count: usize,
}

impl OfflineEntitlementVerifierBetaRenderSummary {
    /// Build the shell render summary from the beta page.
    pub fn from_page(page: &OfflineEntitlementVerifierBetaPage) -> Self {
        Self {
            record_kind: OFFLINE_ENTITLEMENT_VERIFIER_BETA_RENDER_RECORD_KIND.to_owned(),
            schema_version: OFFLINE_ENTITLEMENT_VERIFIER_BETA_SCHEMA_VERSION,
            shared_contract_ref: OFFLINE_ENTITLEMENT_VERIFIER_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_count: page.summary.row_count,
            verified_row_count: page.summary.verified_row_count,
            failed_row_count: page.summary.failed_row_count,
            local_editing_preserved_row_count: page.summary.local_editing_preserved_row_count,
            managed_authority_narrowed_row_count: page.summary.managed_authority_narrowed_row_count,
            profiles_present: page.summary.profiles_present.clone(),
            bundle_kinds_present: page.summary.bundle_kinds_present.clone(),
            outcomes_present: page.summary.outcomes_present.clone(),
            defect_count: page.summary.defect_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_and_summary_covers_profiles_kinds_and_outcomes() {
        let page = seeded_offline_entitlement_verifier_beta_page();
        validate_offline_entitlement_verifier_beta_page(&page).expect("seeded page validates");
        let summary = OfflineEntitlementVerifierBetaRenderSummary::from_page(&page);
        assert_eq!(summary.row_count, page.rows.len());
        assert_eq!(summary.defect_count, 0);

        for required in OfflineEntitlementVerifierBetaProfileClass::ALL {
            assert!(summary
                .profiles_present
                .iter()
                .any(|token| token == required.as_str()));
        }
        for required in VerifiedBundleKindClass::ALL {
            assert!(summary
                .bundle_kinds_present
                .iter()
                .any(|token| token == required.as_str()));
        }
        assert!(summary
            .outcomes_present
            .contains(&"verified_air_gapped".to_owned()));
        assert!(summary.outcomes_present.contains(&"expired".to_owned()));
        assert!(summary
            .outcomes_present
            .contains(&"signature_missing".to_owned()));
    }

    #[test]
    fn support_export_round_trips_through_validator() {
        let page = seeded_offline_entitlement_verifier_beta_page();
        let export = OfflineEntitlementVerifierBetaSupportExport::from_page(
            "offline-entitlement-verifier:support-export:shell",
            "2026-05-16T00:00:00Z",
            page,
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
        assert_eq!(export.support_rows.len(), export.page.rows.len());
    }
}
