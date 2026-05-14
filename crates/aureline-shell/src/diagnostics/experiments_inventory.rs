//! Diagnostics projection for experiments inventory truth.
//!
//! The shell diagnostics surface consumes
//! [`aureline_settings::experiments`] records directly so rollout state,
//! policy disables, kill-switch precedence, and saved-artifact dependency
//! warnings stay aligned with settings CLI and support exports.

use std::collections::BTreeMap;

use aureline_settings::experiments::{
    inspect_default_inventory, ExperimentsInventoryError, ExperimentsInventoryInspectionRecord,
};
use serde::{Deserialize, Serialize};

/// Shell diagnostics schema version for experiments inventory projections.
pub const EXPERIMENTS_INVENTORY_DIAGNOSTICS_SCHEMA_VERSION: u32 = 1;

/// Diagnostics panel record for experiments and capability dependencies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentsInventoryDiagnosticsPanel {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Source inventory artifact ref.
    pub source_inventory_ref: String,
    /// Inventory as-of date.
    pub as_of: String,
    /// Lifecycle counts keyed by controlled token.
    pub lifecycle_counts: BTreeMap<String, usize>,
    /// Number of rows included in diagnostics.
    pub row_count: usize,
    /// Number of dependency warnings included in diagnostics.
    pub artifact_dependency_warning_count: usize,
    /// Rows blocked by policy or retired as tombstones.
    pub blocked_rows: Vec<ExperimentsInventoryDiagnosticsRow>,
    /// Rows with saved-artifact dependency markers.
    pub dependency_rows: Vec<ExperimentsInventoryDiagnosticsRow>,
}

/// Compact diagnostics row for one capability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentsInventoryDiagnosticsRow {
    /// Stable capability id.
    pub capability_id: String,
    /// Effective lifecycle state token.
    pub effective_lifecycle_state: String,
    /// Owner ref.
    pub owner: String,
    /// Cohort or ring.
    pub cohort_or_ring: String,
    /// Review or expiry date.
    pub review_or_expiry_date: String,
    /// Winning disable source class, if any.
    pub winning_disable_source: Option<String>,
    /// Dependency marker count.
    pub artifact_dependency_count: usize,
    /// Fallback path from disable source or first dependency marker.
    pub fallback_path: Option<String>,
}

impl ExperimentsInventoryDiagnosticsPanel {
    fn from_inventory(record: &ExperimentsInventoryInspectionRecord) -> Self {
        let blocked_rows = record
            .rows
            .iter()
            .filter(|row| {
                matches!(
                    row.effective_lifecycle_state.as_str(),
                    "DisabledByPolicy" | "Retired"
                )
            })
            .map(ExperimentsInventoryDiagnosticsRow::from_row)
            .collect();
        let dependency_rows = record
            .rows
            .iter()
            .filter(|row| row.saved_artifact_dependency_present)
            .map(ExperimentsInventoryDiagnosticsRow::from_row)
            .collect();

        Self {
            record_kind: "experiments_inventory_diagnostics_panel".to_owned(),
            schema_version: EXPERIMENTS_INVENTORY_DIAGNOSTICS_SCHEMA_VERSION,
            source_inventory_ref: record.source_inventory_ref.clone(),
            as_of: record.as_of.clone(),
            lifecycle_counts: record.lifecycle_counts.clone(),
            row_count: record.rows.len(),
            artifact_dependency_warning_count: record.artifact_dependency_warnings.len(),
            blocked_rows,
            dependency_rows,
        }
    }
}

impl ExperimentsInventoryDiagnosticsRow {
    fn from_row(row: &aureline_settings::experiments::ExperimentsInventoryRowInspection) -> Self {
        Self {
            capability_id: row.capability_id.clone(),
            effective_lifecycle_state: row.effective_lifecycle_state.clone(),
            owner: row.owner.clone(),
            cohort_or_ring: row.cohort_or_ring.clone(),
            review_or_expiry_date: row.review_or_expiry_date.clone(),
            winning_disable_source: row
                .winning_disable_source
                .as_ref()
                .map(|source| source.source_class.clone()),
            artifact_dependency_count: row.artifact_dependency_count,
            fallback_path: row
                .winning_disable_source
                .as_ref()
                .map(|source| source.fallback_path.clone())
                .or_else(|| {
                    row.dependency_markers
                        .first()
                        .map(|marker| marker.fallback_path.clone())
                }),
        }
    }
}

/// Materializes the default experiments inventory diagnostics panel.
pub fn materialize_experiments_inventory_diagnostics_panel(
) -> Result<ExperimentsInventoryDiagnosticsPanel, ExperimentsInventoryError> {
    let record = inspect_default_inventory()?;
    Ok(ExperimentsInventoryDiagnosticsPanel::from_inventory(
        &record,
    ))
}

/// Builds stable human-readable diagnostics lines for the panel.
pub fn diagnostics_lines(record: &ExperimentsInventoryDiagnosticsPanel) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push("Experiments inventory".to_owned());
    lines.push(format!("source: {}", record.source_inventory_ref));
    lines.push(format!("as_of: {}", record.as_of));
    lines.push(format!("rows: {}", record.row_count));
    lines.push(format!(
        "artifact_dependency_warnings: {}",
        record.artifact_dependency_warning_count
    ));

    if !record.blocked_rows.is_empty() {
        lines.push("blocked_or_retired:".to_owned());
        for row in &record.blocked_rows {
            let source = row.winning_disable_source.as_deref().unwrap_or("tombstone");
            lines.push(format!(
                "  {} state={} source={} fallback={}",
                row.capability_id,
                row.effective_lifecycle_state,
                source,
                row.fallback_path.as_deref().unwrap_or("none")
            ));
        }
    }

    if !record.dependency_rows.is_empty() {
        lines.push("artifact_dependencies:".to_owned());
        for row in &record.dependency_rows {
            lines.push(format!(
                "  {} markers={} ring={}",
                row.capability_id, row.artifact_dependency_count, row.cohort_or_ring
            ));
        }
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostics_panel_surfaces_policy_disable_and_dependencies() {
        let panel =
            materialize_experiments_inventory_diagnostics_panel().expect("panel materializes");

        assert_eq!(panel.row_count, 7);
        assert_eq!(panel.artifact_dependency_warning_count, 8);
        assert!(panel.blocked_rows.iter().any(|row| {
            row.capability_id == "alpha.managed_cloud_daily_driver"
                && row.winning_disable_source.as_deref() == Some("admin_policy_ceiling")
        }));
        assert!(panel
            .dependency_rows
            .iter()
            .any(|row| row.capability_id == "settings.legacy_global_ai_toggle"));
    }

    #[test]
    fn diagnostics_lines_remain_export_safe_and_actionable() {
        let panel =
            materialize_experiments_inventory_diagnostics_panel().expect("panel materializes");
        let lines = diagnostics_lines(&panel);

        assert!(lines
            .iter()
            .any(|line| line
                .contains("source: artifacts/governance/experiments_inventory_alpha.yaml")));
        assert!(lines.iter().any(|line| {
            line.contains("alpha.managed_cloud_daily_driver")
                && line.contains("admin_policy_ceiling")
                && line.contains("Continue locally")
        }));
        assert!(lines
            .iter()
            .any(|line| line.contains("artifact_dependencies:")));
    }
}
