//! Shell consumer for the secret-broker beta projection.
//!
//! The shell does not mint a parallel broker model. It consumes the
//! auth-owned [`aureline_auth::seeded_secret_broker_beta_page`] projection,
//! adds a compact rendering summary, and exposes the same records to the
//! headless inspector, support export, admin/settings center, and reviewer
//! fixtures.

use serde::{Deserialize, Serialize};

pub use aureline_auth::{
    audit_secret_broker_beta_page, seeded_secret_broker_beta_page,
    validate_secret_broker_beta_page, ConsumerAuditOutcomeClass, HandleLifecycleStateClass,
    HandleProjectionModeClass, SecretBrokerBetaDefect, SecretBrokerBetaDefectKind,
    SecretBrokerBetaHandleRow, SecretBrokerBetaPage, SecretBrokerBetaProfileClass,
    SecretBrokerBetaSummary, SecretBrokerBetaSupportExport, SecretConsumerAuditEvent,
    SecretConsumerIdentity, SecretReferenceMode, VaultAdapterClass, VaultBinding,
    VaultSignatureStateClass, SECRET_BROKER_BETA_CONSUMER_AUDIT_RECORD_KIND,
    SECRET_BROKER_BETA_DEFECT_RECORD_KIND, SECRET_BROKER_BETA_HANDLE_ROW_RECORD_KIND,
    SECRET_BROKER_BETA_PAGE_RECORD_KIND, SECRET_BROKER_BETA_SCHEMA_VERSION,
    SECRET_BROKER_BETA_SHARED_CONTRACT_REF, SECRET_BROKER_BETA_SOURCE_MATRIX_REF,
    SECRET_BROKER_BETA_SUMMARY_RECORD_KIND, SECRET_BROKER_BETA_SUPPORT_EXPORT_RECORD_KIND,
};

/// Stable record kind for [`SecretBrokerBetaRenderSummary`] payloads.
pub const SECRET_BROKER_BETA_RENDER_RECORD_KIND: &str = "shell_secret_broker_beta_render_record";

/// Shell-facing rendering summary for the secret-broker beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretBrokerBetaRenderSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Number of handle rows rendered.
    pub handle_row_count: usize,
    /// Number of consumer-identity audit events rendered.
    pub consumer_audit_count: usize,
    /// Profile tokens rendered.
    pub profiles_present: Vec<String>,
    /// Vault adapter tokens rendered.
    pub vault_adapters_present: Vec<String>,
    /// Reference-mode tokens rendered.
    pub reference_modes_present: Vec<String>,
    /// Projection-mode tokens rendered.
    pub projection_modes_present: Vec<String>,
    /// Lifecycle-state tokens rendered.
    pub lifecycle_states_present: Vec<String>,
    /// Audit-outcome tokens rendered.
    pub audit_outcomes_present: Vec<String>,
    /// Number of validator defects.
    pub defect_count: usize,
}

impl SecretBrokerBetaRenderSummary {
    /// Builds the shell render summary from the beta page.
    pub fn from_page(page: &SecretBrokerBetaPage) -> Self {
        Self {
            record_kind: SECRET_BROKER_BETA_RENDER_RECORD_KIND.to_owned(),
            schema_version: SECRET_BROKER_BETA_SCHEMA_VERSION,
            shared_contract_ref: SECRET_BROKER_BETA_SHARED_CONTRACT_REF.to_owned(),
            handle_row_count: page.summary.handle_row_count,
            consumer_audit_count: page.summary.consumer_audit_count,
            profiles_present: page.summary.profiles_present.clone(),
            vault_adapters_present: page.summary.vault_adapters_present.clone(),
            reference_modes_present: page.summary.reference_modes_present.clone(),
            projection_modes_present: page.summary.projection_modes_present.clone(),
            lifecycle_states_present: page.summary.lifecycle_states_present.clone(),
            audit_outcomes_present: page.summary.audit_outcomes_present.clone(),
            defect_count: page.summary.defect_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_and_shell_summary_covers_profiles_and_adapters() {
        let page = seeded_secret_broker_beta_page();
        validate_secret_broker_beta_page(&page).expect("seeded page validates");
        let summary = SecretBrokerBetaRenderSummary::from_page(&page);
        assert_eq!(summary.handle_row_count, page.handle_rows.len());
        assert_eq!(summary.consumer_audit_count, page.consumer_audit.len());
        assert_eq!(summary.defect_count, 0);
        assert!(summary.profiles_present.contains(&"connected".to_owned()));
        assert!(summary
            .profiles_present
            .contains(&"enterprise_managed".to_owned()));
        assert!(summary
            .vault_adapters_present
            .contains(&"enterprise_vault_self_hosted_mirror".to_owned()));
        assert!(summary
            .audit_outcomes_present
            .contains(&"denied_by_plaintext_requested".to_owned()));
    }

    #[test]
    fn support_export_remains_metadata_safe() {
        let page = seeded_secret_broker_beta_page();
        let export = SecretBrokerBetaSupportExport::from_page(
            "support-export:secret-broker:shell",
            "2026-05-16T05:00:00Z",
            page,
        );
        assert!(export.raw_secret_values_excluded);
        assert!(export.raw_handle_ids_excluded);
        assert!(export.consumer_lineage_preserved);
        assert!(export.defect_kinds_present.is_empty());
    }
}
