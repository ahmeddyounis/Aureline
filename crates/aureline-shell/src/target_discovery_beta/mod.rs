//! Beta target-discovery shell projection.
//!
//! This module is a thin shell consumer over the canonical
//! [`aureline_runtime::TargetDiscoveryBetaSupportExport`]. The shell does not
//! own target-discovery truth; it projects the runtime-minted records into
//! reviewable rows and a deterministic plaintext block suitable for the
//! support-export clipboard action and the run / test / debug / build pickers
//! that need a one-glance "where did this target come from and what can I do
//! with it" panel before dispatch.

use serde::{Deserialize, Serialize};

use aureline_runtime::{
    ProtectedActionDecisionRow, TargetDiscoveryBetaProjection, TargetDiscoveryBetaRow,
    TargetDiscoveryBetaSupportExport,
};

/// Stable record-kind tag carried in serialized projections.
pub const TARGET_DISCOVERY_BETA_PANEL_RECORD_KIND: &str = "target_discovery_beta_panel_record";

/// Schema version of the shell panel projection.
pub const TARGET_DISCOVERY_BETA_PANEL_SCHEMA_VERSION: u32 = 1;

/// Header notice rendered above the rows.
pub const TARGET_DISCOVERY_BETA_NOTICE: &str =
    "Target discovery (beta): rows discovered by heuristic parsers or with \
     stale freshness cannot dispatch protected run / test / debug / build \
     actions until the discovery is refreshed or replaced.";

/// One reviewable target-discovery panel row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetDiscoveryBetaPanelRow {
    pub row_id: String,
    pub card_ref: String,
    pub execution_context_ref: String,
    pub target_id: String,
    pub target_class_token: String,
    pub lane_token: String,
    pub discovery_source_token: String,
    pub discovery_source_label: String,
    pub discovery_freshness_token: String,
    pub alpha_discovery_confidence_token: String,
    pub target_confidence_level_token: String,
    pub host_boundary_cue_token: String,
    pub host_boundary_label: String,
    pub supported_capability_tokens: Vec<String>,
    pub decision_summaries: Vec<String>,
    pub blocks_all_protected_dispatch: bool,
    pub explanation_summary: String,
    pub inspect_action_ref: String,
    pub change_target_action_ref: String,
}

impl TargetDiscoveryBetaPanelRow {
    fn project(row: &TargetDiscoveryBetaRow) -> Self {
        let decision_summaries = row
            .protected_action_decisions
            .iter()
            .map(|d| format!("{}={}", d.action_token, d.decision_token))
            .collect();
        Self {
            row_id: row.row_id.clone(),
            card_ref: row.card_ref.clone(),
            execution_context_ref: row.execution_context_ref.clone(),
            target_id: row.target_id.clone(),
            target_class_token: row.target_class_token.clone(),
            lane_token: row.lane_token.clone(),
            discovery_source_token: row.discovery_source_token.clone(),
            discovery_source_label: row.discovery_source_label.clone(),
            discovery_freshness_token: row.discovery_freshness_token.clone(),
            alpha_discovery_confidence_token: row.alpha_discovery_confidence_token.clone(),
            target_confidence_level_token: row.target_confidence_level_token.clone(),
            host_boundary_cue_token: row.host_boundary_cue_token.clone(),
            host_boundary_label: row.host_boundary_label.clone(),
            supported_capability_tokens: row.supported_capability_tokens.clone(),
            decision_summaries,
            blocks_all_protected_dispatch: row.blocks_all_protected_dispatch(),
            explanation_summary: row.explanation_summary.clone(),
            inspect_action_ref: row.inspect_action_ref.clone(),
            change_target_action_ref: row.change_target_action_ref.clone(),
        }
    }
}

/// Beta target-discovery panel rendered into the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetDiscoveryBetaPanel {
    pub record_kind: String,
    pub schema_version: u32,
    pub projection_id: String,
    pub workspace_id: String,
    pub generated_at: String,
    pub notice: String,
    pub rows: Vec<TargetDiscoveryBetaPanelRow>,
    pub any_row_blocks_protected_dispatch: bool,
}

impl TargetDiscoveryBetaPanel {
    /// Builds a shell panel directly from a runtime projection.
    pub fn from_projection(projection: &TargetDiscoveryBetaProjection) -> Self {
        let rows: Vec<TargetDiscoveryBetaPanelRow> = projection
            .rows
            .iter()
            .map(TargetDiscoveryBetaPanelRow::project)
            .collect();
        Self {
            record_kind: TARGET_DISCOVERY_BETA_PANEL_RECORD_KIND.to_owned(),
            schema_version: TARGET_DISCOVERY_BETA_PANEL_SCHEMA_VERSION,
            projection_id: projection.projection_id.clone(),
            workspace_id: projection.workspace_id.clone(),
            generated_at: projection.generated_at.clone(),
            notice: TARGET_DISCOVERY_BETA_NOTICE.to_owned(),
            rows,
            any_row_blocks_protected_dispatch: projection.any_row_blocks_protected_dispatch,
        }
    }

    /// Convenience: build the shell panel from a runtime support export.
    pub fn from_support_export(export: &TargetDiscoveryBetaSupportExport) -> Self {
        Self::from_projection(&export.projection)
    }

    /// Deterministic plaintext block for the support-export clipboard action
    /// and CLI surfaces. Iterates rows and decisions in their canonical order.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Target discovery (beta)\n");
        out.push_str(&format!("Workspace: {}\n", self.workspace_id));
        out.push_str(&format!("Projection: {}\n", self.projection_id));
        out.push_str(&format!("Captured at: {}\n", self.generated_at));
        out.push_str(&format!("Notice: {}\n", self.notice));
        out.push_str(&format!(
            "Rows: {} (any protected dispatch blocked: {})\n",
            self.rows.len(),
            self.any_row_blocks_protected_dispatch
        ));
        for row in &self.rows {
            out.push_str(&format!("\nTarget: {}\n", row.target_id));
            out.push_str(&format!(
                "  Class: {} | Lane: {} | Boundary: {} ({})\n",
                row.target_class_token,
                row.lane_token,
                row.host_boundary_cue_token,
                row.host_boundary_label,
            ));
            out.push_str(&format!(
                "  Source: {} ({})\n",
                row.discovery_source_token, row.discovery_source_label
            ));
            out.push_str(&format!(
                "  Freshness: {} | Confidence: {} (alpha: {})\n",
                row.discovery_freshness_token,
                row.target_confidence_level_token,
                row.alpha_discovery_confidence_token,
            ));
            out.push_str(&format!(
                "  Capabilities: {}\n",
                row.supported_capability_tokens.join(",")
            ));
            out.push_str("  Decisions:\n");
            for line in &row.decision_summaries {
                out.push_str(&format!("    - {line}\n"));
            }
            out.push_str(&format!("  Inspect: {}\n", row.inspect_action_ref));
            out.push_str(&format!("  Change: {}\n", row.change_target_action_ref));
            out.push_str(&format!("  Summary: {}\n", row.explanation_summary));
        }
        out
    }
}

/// Helper for surfaces deciding whether to surface a protected action button.
pub fn decision_label_for(decision: &ProtectedActionDecisionRow) -> &'static str {
    use aureline_runtime::ProtectedActionDecisionClass::*;
    match decision.decision {
        Allowed => "Allowed",
        RequiresReview => "Review required",
        BlockedHeuristicTarget => "Blocked: heuristic discovery",
        BlockedImportedTarget => "Blocked: imported target",
        BlockedUnsupportedCapability => "Blocked: capability unsupported",
        BlockedResolverUnavailable => "Blocked: resolver unavailable",
        BlockedFreshnessStale => "Blocked: discovery stale",
    }
}

#[cfg(test)]
mod tests;
