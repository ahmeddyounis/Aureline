//! Shell consumer for the admin-audit export beta projection.
//!
//! The shell does not mint a parallel provisioning or audit model. It consumes
//! the auth-owned [`aureline_auth::seeded_admin_audit_export_beta_page`]
//! projection, adds a compact rendering summary, and exposes the same records
//! to the headless inspector, support export, admin/settings center, and
//! reviewer fixtures.

use serde::{Deserialize, Serialize};

pub use aureline_auth::{
    audit_admin_audit_export_beta_page, seeded_admin_audit_export_beta_page,
    validate_admin_audit_export_beta_page, AdminAuditExportBetaDefect,
    AdminAuditExportBetaDefectKind, AdminAuditExportBetaPage,
    AdminAuditExportBetaProfileClass, AdminAuditExportBetaSummary,
    AdminAuditExportBetaSupportExport, EntitlementChangeClass, EntitlementChangeEvent,
    PolicyBundleHistoryEvent, PolicyBundleTransitionClass, ProvisioningEvent,
    ProvisioningEventClass, ProvisioningFreshnessClass, ProvisioningLifecycleStateClass,
    ProvisioningProvenance, ProvisioningSourceClass, ProvisioningSubjectKindClass,
    ADMIN_AUDIT_EXPORT_BETA_DEFECT_RECORD_KIND, ADMIN_AUDIT_EXPORT_BETA_PAGE_RECORD_KIND,
    ADMIN_AUDIT_EXPORT_BETA_SCHEMA_VERSION, ADMIN_AUDIT_EXPORT_BETA_SHARED_CONTRACT_REF,
    ADMIN_AUDIT_EXPORT_BETA_SOURCE_MATRIX_REF, ADMIN_AUDIT_EXPORT_BETA_SUMMARY_RECORD_KIND,
    ADMIN_AUDIT_EXPORT_BETA_SUPPORT_EXPORT_RECORD_KIND, ENTITLEMENT_CHANGE_EVENT_RECORD_KIND,
    POLICY_BUNDLE_HISTORY_EVENT_RECORD_KIND, PROVISIONING_EVENT_RECORD_KIND,
};

/// Stable record kind for [`AdminAuditExportBetaRenderSummary`] payloads.
pub const ADMIN_AUDIT_EXPORT_BETA_RENDER_RECORD_KIND: &str =
    "shell_admin_audit_export_beta_render_record";

/// Shell-facing rendering summary for the admin-audit export beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminAuditExportBetaRenderSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Number of provisioning events rendered.
    pub provisioning_event_count: usize,
    /// Number of policy-bundle history events rendered.
    pub policy_bundle_event_count: usize,
    /// Number of entitlement-change events rendered.
    pub entitlement_change_count: usize,
    /// Profile tokens rendered by the shell.
    pub profiles_present: Vec<String>,
    /// Provisioning source tokens rendered by the shell.
    pub provisioning_sources_present: Vec<String>,
    /// Lifecycle-state tokens rendered by the shell.
    pub lifecycle_states_present: Vec<String>,
    /// Freshness tokens rendered by the shell.
    pub freshness_states_present: Vec<String>,
    /// Policy-bundle transition tokens rendered by the shell.
    pub policy_bundle_transitions_present: Vec<String>,
    /// Number of validator defects.
    pub defect_count: usize,
}

impl AdminAuditExportBetaRenderSummary {
    /// Builds the shell render summary from the beta page.
    pub fn from_page(page: &AdminAuditExportBetaPage) -> Self {
        Self {
            record_kind: ADMIN_AUDIT_EXPORT_BETA_RENDER_RECORD_KIND.to_owned(),
            schema_version: ADMIN_AUDIT_EXPORT_BETA_SCHEMA_VERSION,
            shared_contract_ref: ADMIN_AUDIT_EXPORT_BETA_SHARED_CONTRACT_REF.to_owned(),
            provisioning_event_count: page.summary.provisioning_event_count,
            policy_bundle_event_count: page.summary.policy_bundle_event_count,
            entitlement_change_count: page.summary.entitlement_change_count,
            profiles_present: page.summary.profiles_present.clone(),
            provisioning_sources_present: page.summary.provisioning_sources_present.clone(),
            lifecycle_states_present: page.summary.lifecycle_states_present.clone(),
            freshness_states_present: page.summary.freshness_states_present.clone(),
            policy_bundle_transitions_present: page
                .summary
                .policy_bundle_transitions_present
                .clone(),
            defect_count: page.summary.defect_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_and_shell_summary_covers_profiles_and_sources() {
        let page = seeded_admin_audit_export_beta_page();
        validate_admin_audit_export_beta_page(&page).expect("seeded page validates");
        let summary = AdminAuditExportBetaRenderSummary::from_page(&page);
        assert_eq!(
            summary.provisioning_event_count,
            page.provisioning_events.len()
        );
        assert_eq!(summary.defect_count, 0);
        assert!(summary
            .profiles_present
            .contains(&"mirror_only".to_owned()));
        assert!(summary
            .profiles_present
            .contains(&"enterprise_managed".to_owned()));
        assert!(summary
            .provisioning_sources_present
            .contains(&"scim_managed_endpoint".to_owned()));
        assert!(summary
            .provisioning_sources_present
            .contains(&"signed_file_import".to_owned()));
        assert!(summary
            .policy_bundle_transitions_present
            .contains(&"replaced_by_successor".to_owned()));
    }

    #[test]
    fn support_export_remains_metadata_safe() {
        let page = seeded_admin_audit_export_beta_page();
        let export = AdminAuditExportBetaSupportExport::from_page(
            "support-export:admin-audit:shell",
            "2026-05-16T00:00:00Z",
            page,
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
    }
}
