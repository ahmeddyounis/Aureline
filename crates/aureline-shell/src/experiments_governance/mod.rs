//! Shell consumer for the beta experiments / flags / Labs governance UI
//! projection.
//!
//! This module is the live-shell view onto the settings-owned beta
//! governance page. It does NOT mint a parallel inventory or vocabulary.
//! It builds the page from
//! [`aureline_settings::build_default_labs_governance_beta_page`], adds a
//! tiny shell-facing rendering summary, and exposes the same page +
//! CLI + support-export records to the headless inspector and the
//! diagnostics surface.
//!
//! The settings root pane, Help/About panel, diagnostics panel, support
//! export packet, release center, and command palette all read these
//! records, so UI, CLI/headless inspection, and docs use the same
//! lifecycle vocabulary and never claim "stable" behind a hidden Labs,
//! Preview, or Beta dependency.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub use aureline_settings::{
    build_default_labs_governance_beta_page, project_labs_governance_beta_cli,
    project_labs_governance_beta_support_export, validate_labs_governance_beta_page,
    validate_labs_governance_beta_support_export, HostSurfaceAssignment, HostSurfaceClass,
    KillSwitchPathProjection, LabsGovernanceBetaBadge, LabsGovernanceBetaCliProjection,
    LabsGovernanceBetaCliRow, LabsGovernanceBetaPage, LabsGovernanceBetaRow,
    LabsGovernanceBetaSupportExport, LabsGovernanceBetaSupportExportRow,
    LabsGovernanceBetaValidationError, VisibleMarkerToken, LABS_GOVERNANCE_BETA_SCHEMA_VERSION,
    LABS_GOVERNANCE_BETA_SHARED_CONTRACT_REF,
};

/// Record kind for the shell-facing rendering summary.
pub const EXPERIMENTS_GOVERNANCE_RENDER_RECORD_KIND: &str =
    "shell_experiments_governance_beta_render_record";

/// Shell-facing rendering summary for the beta governance page.
///
/// This is the row count and lifecycle breakdown the shell renders into
/// the settings root, Help / About, and diagnostics surfaces. It does
/// not own the lifecycle vocabulary — every count is keyed by the
/// closed inventory token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentsGovernanceRenderSummary {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Total row count rendered on the page.
    pub row_count: usize,
    /// Number of non-Stable rows (Labs, Preview, Beta, Deprecated,
    /// DisabledByPolicy, Retired).
    pub non_stable_row_count: usize,
    /// Number of rows that count toward the "experiments in flight"
    /// attention chip surfaced by the diagnostics and settings root
    /// surfaces.
    pub attention_chip_row_count: usize,
    /// Number of rows with at least one saved-artifact dependency
    /// marker.
    pub saved_artifact_dependency_row_count: usize,
    /// Lifecycle counts keyed by controlled token.
    pub lifecycle_counts: BTreeMap<String, usize>,
}

impl ExperimentsGovernanceRenderSummary {
    /// Build the summary from a beta governance page.
    pub fn from_page(page: &LabsGovernanceBetaPage) -> Self {
        let row_count = page.rows.len();
        let non_stable_row_count = page
            .rows
            .iter()
            .filter(|r| r.effective_lifecycle_state != "Stable")
            .count();
        let attention_chip_row_count = page
            .badges
            .iter()
            .filter(|b| b.counts_toward_attention_chip)
            .count();
        let saved_artifact_dependency_row_count = page
            .rows
            .iter()
            .filter(|r| r.saved_artifact_dependency_present)
            .count();
        Self {
            record_kind: EXPERIMENTS_GOVERNANCE_RENDER_RECORD_KIND.to_owned(),
            schema_version: LABS_GOVERNANCE_BETA_SCHEMA_VERSION,
            shared_contract_ref: LABS_GOVERNANCE_BETA_SHARED_CONTRACT_REF.to_owned(),
            row_count,
            non_stable_row_count,
            attention_chip_row_count,
            saved_artifact_dependency_row_count,
            lifecycle_counts: page.lifecycle_counts.clone(),
        }
    }
}

/// Build the seeded beta governance page from the checked-in inventory.
pub fn seeded_experiments_governance_beta_page() -> LabsGovernanceBetaPage {
    build_default_labs_governance_beta_page().expect("checked-in inventory always builds")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_and_summary_covers_lifecycle_counts() {
        let page = seeded_experiments_governance_beta_page();
        validate_labs_governance_beta_page(&page).expect("seeded page validates");
        let summary = ExperimentsGovernanceRenderSummary::from_page(&page);
        assert_eq!(summary.row_count, page.rows.len());
        assert!(summary.non_stable_row_count >= 1);
        for state in [
            "Labs",
            "Preview",
            "Beta",
            "Stable",
            "Deprecated",
            "DisabledByPolicy",
            "Retired",
        ] {
            assert!(summary.lifecycle_counts.contains_key(state));
        }
    }

    #[test]
    fn support_export_round_trips_through_validator() {
        let page = seeded_experiments_governance_beta_page();
        let export = project_labs_governance_beta_support_export(
            "support-export:experiments-labs-governance:shell",
            "2026-05-15T00:00:00Z",
            page,
        );
        validate_labs_governance_beta_support_export(&export).expect("export validates");
    }
}
