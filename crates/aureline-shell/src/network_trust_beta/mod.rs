//! Shell consumer for the beta network-trust inspection page.
//!
//! The shell does not mint a parallel network-config model. It consumes the
//! auth-owned [`aureline_auth::seeded_network_trust_beta_page`] projection,
//! adds a compact rendering summary, and exposes the same records to the
//! headless inspector, support export, admin/settings center, and reviewer
//! fixtures.

use serde::{Deserialize, Serialize};

pub use aureline_auth::{
    audit_network_trust_beta_rows, seeded_network_trust_beta_page,
    validate_network_trust_beta_page, ClientCertificateStateClass, NetworkAuthorityClass,
    NetworkConsumerLaneClass, NetworkSettingLockClass, NetworkSettingSourceClass,
    NetworkTrustBetaDefect, NetworkTrustBetaDefectKind, NetworkTrustBetaFacetClass,
    NetworkTrustBetaPage, NetworkTrustBetaProfileBinding, NetworkTrustBetaProfileClass,
    NetworkTrustBetaRow, NetworkTrustBetaSummary, NetworkTrustBetaSupportExport,
    NetworkTrustBetaSupportRow, ProxyResolutionModeClass, SshHostProofClass, TrustStoreSourceClass,
    NETWORK_TRUST_BETA_DEFECT_RECORD_KIND, NETWORK_TRUST_BETA_PAGE_RECORD_KIND,
    NETWORK_TRUST_BETA_PROFILE_BINDING_RECORD_KIND, NETWORK_TRUST_BETA_ROW_RECORD_KIND,
    NETWORK_TRUST_BETA_SCHEMA_VERSION, NETWORK_TRUST_BETA_SHARED_CONTRACT_REF,
    NETWORK_TRUST_BETA_SOURCE_MATRIX_REF, NETWORK_TRUST_BETA_SUMMARY_RECORD_KIND,
    NETWORK_TRUST_BETA_SUPPORT_EXPORT_RECORD_KIND, NETWORK_TRUST_BETA_SUPPORT_ROW_RECORD_KIND,
};

/// Stable record kind for [`NetworkTrustBetaRenderSummary`] payloads.
pub const NETWORK_TRUST_BETA_RENDER_RECORD_KIND: &str = "shell_network_trust_beta_render_record";

/// Shell-facing rendering summary for the beta network-trust page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkTrustBetaRenderSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Number of rows rendered.
    pub row_count: usize,
    /// Facet tokens rendered by the shell.
    pub facets_present: Vec<String>,
    /// Profile tokens rendered by the shell.
    pub profiles_present: Vec<String>,
    /// Source tokens rendered by the shell.
    pub source_tokens_present: Vec<String>,
    /// Lock tokens rendered by the shell.
    pub lock_tokens_present: Vec<String>,
    /// Consumer-lane tokens rendered by the shell.
    pub consumer_lane_tokens_present: Vec<String>,
    /// Bindings with an inspectable effective value.
    pub effective_value_published_count: usize,
    /// Bindings that fail closed (missing input or managed-policy block).
    pub blocked_binding_count: usize,
    /// Number of validator defects.
    pub defect_count: usize,
}

impl NetworkTrustBetaRenderSummary {
    /// Builds the shell render summary from the beta page.
    pub fn from_page(page: &NetworkTrustBetaPage) -> Self {
        Self {
            record_kind: NETWORK_TRUST_BETA_RENDER_RECORD_KIND.to_owned(),
            schema_version: NETWORK_TRUST_BETA_SCHEMA_VERSION,
            shared_contract_ref: NETWORK_TRUST_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_count: page.summary.row_count,
            facets_present: page.summary.facets_present.clone(),
            profiles_present: page.summary.profiles_present.clone(),
            source_tokens_present: page.summary.source_tokens_present.clone(),
            lock_tokens_present: page.summary.lock_tokens_present.clone(),
            consumer_lane_tokens_present: page.summary.consumer_lane_tokens_present.clone(),
            effective_value_published_count: page.summary.effective_value_published_count,
            blocked_binding_count: page.summary.blocked_binding_count,
            defect_count: page.summary.defect_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_and_shell_summary_covers_facets_and_profiles() {
        let page = seeded_network_trust_beta_page();
        validate_network_trust_beta_page(&page).expect("seeded page validates");
        let summary = NetworkTrustBetaRenderSummary::from_page(&page);
        assert_eq!(summary.row_count, page.rows.len());
        assert_eq!(summary.defect_count, 0);
        assert!(summary.facets_present.contains(&"proxy".to_owned()));
        assert!(summary.facets_present.contains(&"client_certificate".to_owned()));
        assert!(summary.profiles_present.contains(&"enterprise_managed".to_owned()));
        assert!(summary.consumer_lane_tokens_present.contains(&"runtime".to_owned()));
        assert!(summary
            .consumer_lane_tokens_present
            .contains(&"provider".to_owned()));
    }

    #[test]
    fn support_export_remains_metadata_safe() {
        let page = seeded_network_trust_beta_page();
        let export = NetworkTrustBetaSupportExport::from_page(
            "support-export:network-trust-beta:shell",
            "2026-05-16T00:00:00Z",
            page,
        );
        assert!(export.raw_secret_or_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
    }
}
