//! Shell projection for M5 rollout-governance truth.
//!
//! The shell consumes the command-owned M5 command-truth index so Help/About,
//! diagnostics, and settings-adjacent inspectors can quote the same owner,
//! cohort, expiry, kill-switch, lifecycle, and narrowing posture that the
//! command routes enforce.

use aureline_commands::{current_m5_command_truth_index_export, M5CommandTruthIndexPacket};
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
    /// Release-center projection ref consuming the same row.
    pub release_center_projection_ref: String,
    /// Public-truth projection ref consuming the same row.
    pub public_truth_projection_ref: String,
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
    /// Shared command-truth index ref.
    pub source_command_truth_index_ref: String,
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
    /// Rows visible to release-center truth.
    pub release_center_visible_row_count: usize,
    /// Rows visible to public-truth packs.
    pub public_truth_visible_row_count: usize,
    /// Render rows.
    pub rows: Vec<M5RolloutGovernanceRenderRow>,
}

fn truth_index_packet() -> M5CommandTruthIndexPacket {
    current_m5_command_truth_index_export().expect("checked M5 command truth index validates")
}

impl M5RolloutGovernanceRenderSummary {
    /// Builds the shell summary from the shared command-owned packets.
    pub fn from_packet(packet: &M5CommandTruthIndexPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| M5RolloutGovernanceRenderRow {
                command_id: row.command_id.clone(),
                display_label: row.display_label.clone(),
                effective_state_label: row.lifecycle_label.clone(),
                rollout_ring: row.rollout_ring.clone(),
                cohort: row.cohort.clone(),
                review_or_expiry_date: row.review_or_expiry_date.clone(),
                owner_ref: row.owner_ref.clone(),
                active_kill_switch_source: row.active_kill_switch_source.clone(),
                settings_projection_ref: row.settings_projection_ref.clone(),
                help_about_projection_ref: row.help_about_projection_ref.clone(),
                diagnostics_projection_ref: row.diagnostics_projection_ref.clone(),
                support_export_projection_ref: row.support_export_projection_ref.clone(),
                release_center_projection_ref: row.release_center_projection_ref.clone(),
                public_truth_projection_ref: row.public_truth_projection_ref.clone(),
                stable_claim_allowed: row.stable_wording_allowed,
            })
            .collect::<Vec<_>>();

        Self {
            record_kind: M5_ROLLOUT_GOVERNANCE_RENDER_RECORD_KIND.to_string(),
            schema_version: packet.schema_version,
            source_command_truth_index_ref: packet.schema_ref.clone(),
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
            diagnostics_visible_row_count: packet
                .rows
                .iter()
                .filter(|row| !row.diagnostics_projection_ref.is_empty())
                .count(),
            support_export_visible_row_count: rows
                .iter()
                .filter(|row| !row.support_export_projection_ref.is_empty())
                .count(),
            release_center_visible_row_count: rows
                .iter()
                .filter(|row| !row.release_center_projection_ref.is_empty())
                .count(),
            public_truth_visible_row_count: rows
                .iter()
                .filter(|row| !row.public_truth_projection_ref.is_empty())
                .count(),
            rows,
        }
    }
}

/// Builds the seeded shell rollout-governance summary.
pub fn seeded_m5_rollout_governance_render_summary() -> M5RolloutGovernanceRenderSummary {
    let packet = truth_index_packet();
    M5RolloutGovernanceRenderSummary::from_packet(&packet)
}
