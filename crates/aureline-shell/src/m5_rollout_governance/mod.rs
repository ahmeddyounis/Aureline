//! Shell projection for M5 rollout-governance truth.
//!
//! The shell consumes the command-owned M5 rollout inventory and capability
//! state truth so Help/About, diagnostics, and settings-adjacent inspectors can
//! quote the same owner, cohort, expiry, kill-switch, and lifecycle posture
//! that the command routes enforce.

use aureline_commands::{
    current_m5_capability_state_truth_export, current_m5_rollout_inventory_export,
    M5CapabilityProjectionSurfaceClass, M5CapabilityStateTruthPacket, M5RolloutConsumerSurfaceClass,
    M5RolloutInventoryPacket, M5RolloutInventoryRow,
};
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Record kind for the shell-facing rollout summary.
pub const M5_ROLLOUT_GOVERNANCE_RENDER_RECORD_KIND: &str =
    "shell_m5_rollout_governance_render_record";

/// One shell-facing rollout row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RolloutGovernanceRenderRow {
    /// Stable command id anchoring the row.
    pub command_id: String,
    /// Human-facing command-family label.
    pub display_label: String,
    /// Effective lifecycle label shown by stable-facing surfaces.
    pub effective_state_label: String,
    /// Rollout ring currently governing the row.
    pub rollout_ring: String,
    /// Named cohort currently governing the row.
    pub cohort: String,
    /// Review or expiry date.
    pub review_or_expiry_date: String,
    /// Owning person or team ref.
    pub owner_ref: String,
    /// Active kill-switch source class, when present.
    pub active_kill_switch_source: Option<String>,
    /// Settings projection ref consuming the same row.
    pub settings_projection_ref: String,
    /// Help/About projection ref consuming the same row.
    pub help_about_projection_ref: String,
    /// Diagnostics projection ref consuming the same row.
    pub diagnostics_projection_ref: String,
    /// Support-export projection ref consuming the same row.
    pub support_export_projection_ref: String,
    /// Docs/release projection ref consuming the same row.
    pub docs_release_projection_ref: String,
    /// Whether the row may still publish stable-looking wording.
    pub stable_claim_allowed: bool,
}

/// Shell-facing rollout-governance summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RolloutGovernanceRenderSummary {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version quoted from the command-owned packet.
    pub schema_version: u32,
    /// Shared rollout packet ref.
    pub source_rollout_inventory_ref: String,
    /// Source capability-state packet ref.
    pub source_capability_state_ref: String,
    /// Total row count under audit.
    pub row_count: usize,
    /// Rows with an active kill switch.
    pub active_kill_switch_row_count: usize,
    /// Rows narrowed below stable wording.
    pub narrowed_row_count: usize,
    /// Rows visible to settings inspectors.
    pub settings_inspector_visible_row_count: usize,
    /// Rows visible to Help/About.
    pub help_about_visible_row_count: usize,
    /// Rows visible to diagnostics.
    pub diagnostics_visible_row_count: usize,
    /// Rows visible to support exports.
    pub support_export_visible_row_count: usize,
    /// Render rows.
    pub rows: Vec<M5RolloutGovernanceRenderRow>,
}

fn capability_packet() -> M5CapabilityStateTruthPacket {
    current_m5_capability_state_truth_export().expect("checked M5 capability-state truth validates")
}

fn surface_ref(row: &M5RolloutInventoryRow, surface: M5RolloutConsumerSurfaceClass) -> String {
    row.surfaced_in
        .iter()
        .find(|entry| entry.surface_class == surface)
        .map(|entry| entry.surface_ref.clone())
        .unwrap_or_else(|| format!("m5-rollout:{}:{}", row.command_id, surface.as_str()))
}

fn settings_projection_ref(
    capability_packet: &M5CapabilityStateTruthPacket,
    command_id: &str,
) -> String {
    capability_packet
        .rows
        .iter()
        .find(|row| row.command_id == command_id)
        .and_then(|row| {
            row.projection_rows
                .iter()
                .find(|projection| {
                    projection.surface_class == M5CapabilityProjectionSurfaceClass::SettingsRow
                })
                .map(|projection| projection.projection_ref.clone())
        })
        .unwrap_or_else(|| format!("projection:{}:settings_row", command_id))
}

impl M5RolloutGovernanceRenderSummary {
    /// Builds the shell summary from the shared command-owned packets.
    pub fn from_packets(
        rollout_packet: &M5RolloutInventoryPacket,
        capability_packet: &M5CapabilityStateTruthPacket,
    ) -> Self {
        let rows = rollout_packet
            .rows
            .iter()
            .map(|row| M5RolloutGovernanceRenderRow {
                command_id: row.command_id.clone(),
                display_label: row.display_label.clone(),
                effective_state_label: row.effective_state_class.display_label().to_string(),
                rollout_ring: row.rollout_ring.clone(),
                cohort: row.cohort.clone(),
                review_or_expiry_date: row.review_or_expiry_date.clone(),
                owner_ref: row.owner_ref.clone(),
                active_kill_switch_source: row
                    .active_kill_switches()
                    .first()
                    .map(|kill| kill.source_class.as_str().to_string()),
                settings_projection_ref: settings_projection_ref(capability_packet, &row.command_id),
                help_about_projection_ref: surface_ref(row, M5RolloutConsumerSurfaceClass::HelpAbout),
                diagnostics_projection_ref: surface_ref(row, M5RolloutConsumerSurfaceClass::Diagnostics),
                support_export_projection_ref: surface_ref(
                    row,
                    M5RolloutConsumerSurfaceClass::SupportExport,
                ),
                docs_release_projection_ref: surface_ref(row, M5RolloutConsumerSurfaceClass::DocsRelease),
                stable_claim_allowed: row.stable_claim_allowed,
            })
            .collect::<Vec<_>>();

        Self {
            record_kind: M5_ROLLOUT_GOVERNANCE_RENDER_RECORD_KIND.to_string(),
            schema_version: rollout_packet.schema_version,
            source_rollout_inventory_ref: rollout_packet.schema_ref.clone(),
            source_capability_state_ref: capability_packet.schema_ref.clone(),
            row_count: rows.len(),
            active_kill_switch_row_count: rows
                .iter()
                .filter(|row| row.active_kill_switch_source.is_some())
                .count(),
            narrowed_row_count: rows.iter().filter(|row| !row.stable_claim_allowed).count(),
            settings_inspector_visible_row_count: rows
                .iter()
                .filter(|row| !row.settings_projection_ref.is_empty())
                .count(),
            help_about_visible_row_count: rows
                .iter()
                .filter(|row| !row.help_about_projection_ref.is_empty())
                .count(),
            diagnostics_visible_row_count: rows
                .iter()
                .filter(|row| !row.diagnostics_projection_ref.is_empty())
                .count(),
            support_export_visible_row_count: rows
                .iter()
                .filter(|row| !row.support_export_projection_ref.is_empty())
                .count(),
            rows,
        }
    }
}

/// Builds the seeded shell rollout-governance summary.
pub fn seeded_m5_rollout_governance_render_summary() -> M5RolloutGovernanceRenderSummary {
    let rollout_packet =
        current_m5_rollout_inventory_export().expect("checked M5 rollout inventory validates");
    let capability_packet = capability_packet();
    M5RolloutGovernanceRenderSummary::from_packets(&rollout_packet, &capability_packet)
}
