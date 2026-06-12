//! Diagnostics projection for M5 rollout-governance truth.
//!
//! The shell diagnostics surface consumes the shared M5 rollout-governance
//! summary so policy blocks, retest narrowing, and help/settings/support
//! projection refs stay aligned with the command-owned rollout packet.

use serde::{Deserialize, Serialize};

use crate::m5_rollout_governance::seeded_m5_rollout_governance_render_summary;

/// Diagnostics schema version for the M5 rollout panel.
pub const M5_ROLLOUT_DIAGNOSTICS_SCHEMA_VERSION: u32 = 1;

/// Diagnostics panel for M5 rollout governance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RolloutDiagnosticsPanel {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Total rows included in diagnostics.
    pub row_count: usize,
    /// Rows carrying an active kill switch.
    pub active_kill_switch_row_count: usize,
    /// Rows narrowed below stable wording.
    pub narrowed_row_count: usize,
    /// Rows the diagnostics view calls out directly.
    pub rows: Vec<M5RolloutDiagnosticsRow>,
}

/// Compact diagnostics row for one M5 rollout row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RolloutDiagnosticsRow {
    /// Stable command id.
    pub command_id: String,
    /// Human-facing family label.
    pub display_label: String,
    /// Effective lifecycle label.
    pub effective_state_label: String,
    /// Rollout ring and cohort.
    pub rollout_scope: String,
    /// Owning person or team ref.
    pub owner_ref: String,
    /// Active kill-switch source class, when present.
    pub active_kill_switch_source: Option<String>,
    /// Settings projection ref quoting the same row.
    pub settings_projection_ref: String,
    /// Support-export projection ref quoting the same row.
    pub support_export_projection_ref: String,
}

/// Materializes the default M5 rollout diagnostics panel.
pub fn materialize_m5_rollout_diagnostics_panel() -> M5RolloutDiagnosticsPanel {
    let summary = seeded_m5_rollout_governance_render_summary();
    let rows = summary
        .rows
        .into_iter()
        .filter(|row| row.active_kill_switch_source.is_some() || !row.stable_claim_allowed)
        .map(|row| M5RolloutDiagnosticsRow {
            command_id: row.command_id,
            display_label: row.display_label,
            effective_state_label: row.effective_state_label,
            rollout_scope: format!("{} / {}", row.rollout_ring, row.cohort),
            owner_ref: row.owner_ref,
            active_kill_switch_source: row.active_kill_switch_source,
            settings_projection_ref: row.settings_projection_ref,
            support_export_projection_ref: row.support_export_projection_ref,
        })
        .collect::<Vec<_>>();

    M5RolloutDiagnosticsPanel {
        record_kind: "m5_rollout_diagnostics_panel".to_string(),
        schema_version: M5_ROLLOUT_DIAGNOSTICS_SCHEMA_VERSION,
        row_count: rows.len(),
        active_kill_switch_row_count: summary.active_kill_switch_row_count,
        narrowed_row_count: summary.narrowed_row_count,
        rows,
    }
}

/// Builds stable human-readable diagnostics lines for the panel.
pub fn diagnostics_lines(panel: &M5RolloutDiagnosticsPanel) -> Vec<String> {
    let mut lines = vec![
        "M5 rollout governance".to_string(),
        format!("rows: {}", panel.row_count),
        format!("active_kill_switch_rows: {}", panel.active_kill_switch_row_count),
        format!("narrowed_rows: {}", panel.narrowed_row_count),
    ];
    for row in &panel.rows {
        lines.push(format!(
            "  {} state={} scope={} kill_switch={} settings_ref={}",
            row.command_id,
            row.effective_state_label,
            row.rollout_scope,
            row.active_kill_switch_source.as_deref().unwrap_or("none"),
            row.settings_projection_ref
        ));
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostics_panel_surfaces_policy_blocks_and_narrowing() {
        let panel = materialize_m5_rollout_diagnostics_panel();
        assert!(panel.row_count >= 1);
        assert!(panel.active_kill_switch_row_count >= 1);
        assert!(panel.rows.iter().any(|row| {
            row.command_id == "cmd:sync.push_workspace_state"
                && row.active_kill_switch_source.is_some()
        }));
    }

    #[test]
    fn diagnostics_lines_name_settings_projection_refs() {
        let panel = materialize_m5_rollout_diagnostics_panel();
        let lines = diagnostics_lines(&panel);
        assert!(lines.iter().any(|line| line.contains("M5 rollout governance")));
        assert!(lines
            .iter()
            .any(|line| line.contains("cmd:docs_browser.open_external")));
        assert!(lines.iter().any(|line| line.contains("settings_ref=")));
    }
}
