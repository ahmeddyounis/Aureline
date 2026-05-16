//! Shell consumer for the beta workspace-trust audit page.
//!
//! The shell does not mint a parallel trust model. It consumes the
//! auth-owned [`aureline_auth::seeded_workspace_trust_beta_page`] projection,
//! adds a compact rendering summary, and exposes the same page to the
//! headless inspector, support export, settings/trust center, and diagnostics
//! surfaces.

use serde::{Deserialize, Serialize};

pub use aureline_auth::{
    audit_workspace_trust_beta_rows, seeded_workspace_trust_beta_page,
    validate_workspace_trust_beta_page, WorkspaceTrustBetaDefect, WorkspaceTrustBetaDefectKind,
    WorkspaceTrustBetaLaneClass, WorkspaceTrustBetaPage, WorkspaceTrustBetaProfileAuthority,
    WorkspaceTrustBetaProfileClass, WorkspaceTrustBetaRow, WorkspaceTrustBetaSummary,
    WorkspaceTrustBetaSupportExport, WorkspaceTrustBetaSupportRow,
    WORKSPACE_TRUST_BETA_DEFECT_RECORD_KIND, WORKSPACE_TRUST_BETA_PAGE_RECORD_KIND,
    WORKSPACE_TRUST_BETA_ROW_RECORD_KIND, WORKSPACE_TRUST_BETA_SCHEMA_VERSION,
    WORKSPACE_TRUST_BETA_SHARED_CONTRACT_REF, WORKSPACE_TRUST_BETA_SUPPORT_EXPORT_RECORD_KIND,
    WORKSPACE_TRUST_BETA_SUPPORT_ROW_RECORD_KIND, WORKSPACE_TRUST_BETA_SURFACE_FAMILIES,
};

/// Stable record kind for [`WorkspaceTrustBetaRenderSummary`] payloads.
pub const WORKSPACE_TRUST_BETA_RENDER_RECORD_KIND: &str =
    "shell_workspace_trust_beta_render_record";

/// Shell-facing rendering summary for the beta workspace-trust page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceTrustBetaRenderSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Number of rows rendered.
    pub row_count: usize,
    /// Number of rows in the restricted-posture floor.
    pub restricted_floor_row_count: usize,
    /// Number of rows blocked or review-gated before trust.
    pub blocked_or_review_before_trust_count: usize,
    /// Number of run-capable or mutation-capable rows.
    pub run_or_mutation_capable_count: usize,
    /// Lane tokens rendered by the shell.
    pub lanes_present: Vec<String>,
    /// Profile tokens rendered by the shell.
    pub profiles_present: Vec<String>,
    /// Number of validator defects.
    pub defect_count: usize,
}

impl WorkspaceTrustBetaRenderSummary {
    /// Builds the shell render summary from the beta page.
    pub fn from_page(page: &WorkspaceTrustBetaPage) -> Self {
        Self {
            record_kind: WORKSPACE_TRUST_BETA_RENDER_RECORD_KIND.to_owned(),
            schema_version: WORKSPACE_TRUST_BETA_SCHEMA_VERSION,
            shared_contract_ref: WORKSPACE_TRUST_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_count: page.summary.row_count,
            restricted_floor_row_count: page.summary.restricted_floor_row_count,
            blocked_or_review_before_trust_count: page.summary.blocked_or_review_before_trust_count,
            run_or_mutation_capable_count: page.summary.run_or_mutation_capable_count,
            lanes_present: page.summary.lanes_present.clone(),
            profiles_present: page.summary.profiles_present.clone(),
            defect_count: page.summary.defect_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_and_shell_summary_covers_profiles() {
        let page = seeded_workspace_trust_beta_page();
        validate_workspace_trust_beta_page(&page).expect("seeded page validates");
        let summary = WorkspaceTrustBetaRenderSummary::from_page(&page);
        assert_eq!(
            summary.row_count,
            WORKSPACE_TRUST_BETA_SURFACE_FAMILIES.len()
        );
        assert_eq!(summary.defect_count, 0);
        assert!(summary.lanes_present.contains(&"run".to_owned()));
        assert!(summary.lanes_present.contains(&"provider".to_owned()));
        assert!(summary
            .profiles_present
            .contains(&"enterprise_managed".to_owned()));
        assert!(summary.blocked_or_review_before_trust_count > summary.restricted_floor_row_count);
    }

    #[test]
    fn support_export_remains_metadata_safe() {
        let page = seeded_workspace_trust_beta_page();
        let export = WorkspaceTrustBetaSupportExport::from_page(
            "support-export:workspace-trust-beta:shell",
            "2026-05-16T00:00:00Z",
            page,
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
    }
}
