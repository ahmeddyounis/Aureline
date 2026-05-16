//! Shell consumer for the region / tenant / key-mode beta projection.
//!
//! The shell does not mint a parallel region, tenant, or key-mode model. It
//! consumes the auth-owned [`aureline_auth::seeded_region_tenant_key_mode_beta_page`]
//! projection, adds a compact rendering summary, and exposes the same records
//! to the headless inspector, admin/settings center, support-export wrapper,
//! diagnostics views, and reviewer fixtures.

use serde::{Deserialize, Serialize};

pub use aureline_auth::{
    audit_region_tenant_key_mode_beta_page, seeded_region_tenant_key_mode_beta_page,
    validate_region_tenant_key_mode_beta_page, KeyModeRow, KeyModeStateClass,
    ManagedActionLaneClass, ProcessingLocationDisclosure, RegionDisclosureRow,
    RegionPinningStateClass, RegionTenantDrillKindClass, RegionTenantDrillOutcomeClass,
    RegionTenantDrillPacket, RegionTenantKeyModeBetaDefect, RegionTenantKeyModeBetaDefectKind,
    RegionTenantKeyModeBetaPage, RegionTenantKeyModeBetaProfileClass,
    RegionTenantKeyModeBetaSummary, RegionTenantKeyModeBetaSupportExport, TenantBoundaryRow,
    TenantBoundaryStateClass, REGION_TENANT_KEY_MODE_BETA_SCHEMA_VERSION,
    REGION_TENANT_KEY_MODE_BETA_SHARED_CONTRACT_REF,
};

/// Stable record kind for [`RegionTenantKeyModeBetaRenderSummary`] payloads.
pub const REGION_TENANT_KEY_MODE_BETA_RENDER_RECORD_KIND: &str =
    "shell_region_tenant_key_mode_beta_render_record";

/// Shell-facing rendering summary for the region / tenant / key-mode beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionTenantKeyModeBetaRenderSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Number of region disclosure rows rendered.
    pub region_row_count: usize,
    /// Number of tenant boundary rows rendered.
    pub tenant_row_count: usize,
    /// Number of key-mode rows rendered.
    pub key_mode_row_count: usize,
    /// Number of drill packets rendered.
    pub drill_packet_count: usize,
    /// Profile tokens rendered by the shell.
    pub profiles_present: Vec<String>,
    /// Managed action lane tokens rendered by the shell.
    pub managed_lanes_present: Vec<String>,
    /// Drill axes rendered by the shell.
    pub drill_axes_present: Vec<String>,
    /// Drill kind tokens rendered by the shell.
    pub drill_kinds_present: Vec<String>,
    /// Number of validator defects.
    pub defect_count: usize,
}

impl RegionTenantKeyModeBetaRenderSummary {
    /// Builds the shell render summary from the beta page.
    pub fn from_page(page: &RegionTenantKeyModeBetaPage) -> Self {
        Self {
            record_kind: REGION_TENANT_KEY_MODE_BETA_RENDER_RECORD_KIND.to_owned(),
            schema_version: REGION_TENANT_KEY_MODE_BETA_SCHEMA_VERSION,
            shared_contract_ref: REGION_TENANT_KEY_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
            region_row_count: page.summary.region_row_count,
            tenant_row_count: page.summary.tenant_row_count,
            key_mode_row_count: page.summary.key_mode_row_count,
            drill_packet_count: page.summary.drill_packet_count,
            profiles_present: page.summary.profiles_present.clone(),
            managed_lanes_present: page.summary.managed_lanes_present.clone(),
            drill_axes_present: page.summary.drill_axes_present.clone(),
            drill_kinds_present: page.summary.drill_kinds_present.clone(),
            defect_count: page.summary.defect_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_and_shell_summary_covers_profiles_and_axes() {
        let page = seeded_region_tenant_key_mode_beta_page();
        validate_region_tenant_key_mode_beta_page(&page).expect("seeded page validates");
        let summary = RegionTenantKeyModeBetaRenderSummary::from_page(&page);
        assert_eq!(summary.region_row_count, page.region_rows.len());
        assert_eq!(summary.tenant_row_count, page.tenant_rows.len());
        assert_eq!(summary.key_mode_row_count, page.key_mode_rows.len());
        assert_eq!(summary.drill_packet_count, page.drill_packets.len());
        assert_eq!(summary.defect_count, 0);
        assert!(summary.profiles_present.contains(&"connected".to_owned()));
        assert!(summary
            .profiles_present
            .contains(&"enterprise_managed".to_owned()));
        for axis in ["region", "tenant", "key_mode"] {
            assert!(summary
                .drill_axes_present
                .iter()
                .any(|token| token == axis));
        }
    }

    #[test]
    fn support_export_remains_metadata_safe() {
        let page = seeded_region_tenant_key_mode_beta_page();
        let export = RegionTenantKeyModeBetaSupportExport::from_page(
            "support-export:region-tenant-key-mode:shell",
            "2026-05-16T00:00:00Z",
            page,
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.no_public_endpoint_fallback_invariant);
        assert!(export.local_editing_preserved_invariant);
        assert!(export.defect_kinds_present.is_empty());
    }
}
