//! Shell consumer for the enterprise drill baseline projection.
//!
//! The shell does not mint a parallel drill model. It consumes the auth-owned
//! [`aureline_auth::seeded_enterprise_drill_baseline_page`] projection, adds a
//! compact rendering summary, and exposes the same records to the headless
//! inspector, admin / settings center, support-export wrapper, diagnostics
//! views, and reviewer fixtures.

use serde::{Deserialize, Serialize};

pub use aureline_auth::{
    audit_enterprise_drill_baseline_page, seeded_enterprise_drill_baseline_page,
    validate_enterprise_drill_baseline_page, EnterpriseDrillBaselineDefect,
    EnterpriseDrillBaselineDefectKind, EnterpriseDrillBaselinePage, EnterpriseDrillBaselineSummary,
    EnterpriseDrillBaselineSupportExport, EnterpriseDrillClaimImpactClass,
    EnterpriseDrillEvidenceFreshnessClass, EnterpriseDrillKindClass, EnterpriseDrillOutcomeClass,
    EnterpriseDrillPacket, EnterpriseDrillProfileClass, EnterpriseRowFamilyClass,
    ENTERPRISE_DRILL_BASELINE_SCHEMA_VERSION, ENTERPRISE_DRILL_BASELINE_SHARED_CONTRACT_REF,
};

/// Stable record kind for [`EnterpriseDrillBaselineRenderSummary`] payloads.
pub const ENTERPRISE_DRILL_BASELINE_RENDER_RECORD_KIND: &str =
    "shell_enterprise_drill_baseline_render_record";

/// Shell-facing rendering summary for the drill baseline page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnterpriseDrillBaselineRenderSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Number of drill packets rendered.
    pub drill_packet_count: usize,
    /// Drill kind tokens rendered by the shell.
    pub drill_kinds_present: Vec<String>,
    /// Row family tokens rendered by the shell.
    pub row_families_present: Vec<String>,
    /// Profile tokens rendered by the shell.
    pub profiles_present: Vec<String>,
    /// Number of drills declaring a claim downgrade on stale / missing
    /// evidence.
    pub claim_downgrades_present: usize,
    /// Number of validator defects.
    pub defect_count: usize,
}

impl EnterpriseDrillBaselineRenderSummary {
    /// Builds the shell render summary from the drill baseline page.
    pub fn from_page(page: &EnterpriseDrillBaselinePage) -> Self {
        Self {
            record_kind: ENTERPRISE_DRILL_BASELINE_RENDER_RECORD_KIND.to_owned(),
            schema_version: ENTERPRISE_DRILL_BASELINE_SCHEMA_VERSION,
            shared_contract_ref: ENTERPRISE_DRILL_BASELINE_SHARED_CONTRACT_REF.to_owned(),
            drill_packet_count: page.summary.drill_packet_count,
            drill_kinds_present: page.summary.drill_kinds_present.clone(),
            row_families_present: page.summary.row_families_present.clone(),
            profiles_present: page.summary.profiles_present.clone(),
            claim_downgrades_present: page.summary.claim_downgrades_present,
            defect_count: page.summary.defect_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_and_shell_summary_covers_families_and_kinds() {
        let page = seeded_enterprise_drill_baseline_page();
        validate_enterprise_drill_baseline_page(&page).expect("seeded page validates");
        let summary = EnterpriseDrillBaselineRenderSummary::from_page(&page);
        assert_eq!(summary.drill_packet_count, page.drill_packets.len());
        assert_eq!(summary.defect_count, 0);
        for family in EnterpriseRowFamilyClass::ALL {
            assert!(summary
                .row_families_present
                .iter()
                .any(|token| token == family.as_str()));
        }
        for kind in EnterpriseDrillKindClass::ALL {
            assert!(summary
                .drill_kinds_present
                .iter()
                .any(|token| token == kind.as_str()));
        }
    }

    #[test]
    fn support_export_remains_metadata_safe() {
        let page = seeded_enterprise_drill_baseline_page();
        let export = EnterpriseDrillBaselineSupportExport::from_page(
            "support-export:enterprise-drill-baseline:shell",
            "2026-05-16T00:00:00Z",
            page,
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.no_public_endpoint_fallback_invariant);
        assert!(export.local_editing_preserved_invariant);
        assert!(export.sibling_lanes_unwidened_invariant);
        assert!(export.defect_kinds_present.is_empty());
    }
}
