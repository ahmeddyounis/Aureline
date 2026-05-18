//! Shell projection for build-intelligence health, targets, receipts, and diffs.
//!
//! The shell consumes [`aureline_runtime::BuildIntelligenceSupportExport`]
//! without re-deriving target truth. This projection keeps adapter lane,
//! health, exactness, imported-versus-live, receipt, and refresh-diff fields in
//! one deterministic panel for run/test/debug/build pickers, CLI/headless
//! summaries, and support-export clipboard flows.

use serde::{Deserialize, Serialize};

use aureline_runtime::{
    AdapterHealthStrip, BuildIntelligenceReceipt, BuildIntelligenceRunConfigCard,
    BuildIntelligenceSupportExport, BuildIntelligenceTargetRow, DiscoveryDiffItem,
    DiscoveryDiffReview,
};

/// Stable record-kind tag carried by shell build-intelligence panels.
pub const BUILD_INTELLIGENCE_BETA_PANEL_RECORD_KIND: &str = "build_intelligence_beta_panel_record";

/// Schema version for the shell panel projection.
pub const BUILD_INTELLIGENCE_BETA_PANEL_SCHEMA_VERSION: u32 = 1;

/// Notice rendered above build-intelligence rows.
pub const BUILD_INTELLIGENCE_BETA_NOTICE: &str =
    "Build intelligence: imported, replayed, partial, or heuristic rows stay \
     inspectable, but high-trust dispatch requires current live discovery or \
     an explicit refresh/review posture.";

/// Adapter health row rendered by the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildIntelligencePanelHealthRow {
    /// Health strip id.
    pub strip_id: String,
    /// Lane token.
    pub lane_type_token: String,
    /// Adapter id.
    pub adapter_id: String,
    /// Adapter label.
    pub adapter_label: String,
    /// Health-state token.
    pub state_token: String,
    /// Health-reason token, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_reason_token: Option<String>,
    /// Last successful refresh timestamp, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_successful_refresh_at: Option<String>,
    /// Imported-versus-live token.
    pub imported_live_state_token: String,
    /// Repair action ref.
    pub repair_action_ref: String,
    /// Details action ref.
    pub details_action_ref: String,
    /// Continue-local action ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continue_local_action_ref: Option<String>,
    /// Inspect-only action ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inspect_only_action_ref: Option<String>,
}

impl BuildIntelligencePanelHealthRow {
    fn project(strip: &AdapterHealthStrip) -> Self {
        Self {
            strip_id: strip.strip_id.clone(),
            lane_type_token: strip.lane_type_token.clone(),
            adapter_id: strip.adapter_identity.adapter_id.clone(),
            adapter_label: strip.adapter_identity.adapter_label.clone(),
            state_token: strip.state_token.clone(),
            health_reason_token: strip.health_reason_token.clone(),
            last_successful_refresh_at: strip.last_successful_refresh_at.clone(),
            imported_live_state_token: strip.imported_live_state_token.clone(),
            repair_action_ref: strip.repair_action.action_ref.clone(),
            details_action_ref: strip.details_action.action_ref.clone(),
            continue_local_action_ref: strip
                .continue_local_action
                .as_ref()
                .map(|action| action.action_ref.clone()),
            inspect_only_action_ref: strip
                .inspect_only_action
                .as_ref()
                .map(|action| action.action_ref.clone()),
        }
    }
}

/// Target row rendered by the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildIntelligencePanelTargetRow {
    /// Target row id.
    pub row_id: String,
    /// Stable target id.
    pub stable_target_id: String,
    /// User-visible target label.
    pub display_name: String,
    /// Lane token.
    pub lane_type_token: String,
    /// Exactness token.
    pub exactness_status_token: String,
    /// Adapter health strip ref.
    pub adapter_health_strip_ref: String,
    /// Archetype binding, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archetype_binding: Option<String>,
    /// Framework binding, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub framework_binding: Option<String>,
    /// Imported-versus-live token.
    pub imported_live_state_token: String,
    /// Imported-versus-live note.
    pub imported_vs_live_note: String,
    /// Open-source action ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_source_action_ref: Option<String>,
    /// Open-config action ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_config_action_ref: Option<String>,
}

impl BuildIntelligencePanelTargetRow {
    fn project(row: &BuildIntelligenceTargetRow) -> Self {
        Self {
            row_id: row.row_id.clone(),
            stable_target_id: row.stable_target_id.clone(),
            display_name: row.display_name.clone(),
            lane_type_token: row.lane_type_token.clone(),
            exactness_status_token: row.exactness_status_token.clone(),
            adapter_health_strip_ref: row.adapter_health_strip_ref.clone(),
            archetype_binding: row.archetype_binding.clone(),
            framework_binding: row.framework_binding.clone(),
            imported_live_state_token: row.imported_live_state_token.clone(),
            imported_vs_live_note: row.imported_vs_live_note.clone(),
            open_source_action_ref: row
                .open_source_action
                .as_ref()
                .map(|action| action.action_ref.clone()),
            open_config_action_ref: row
                .open_config_action
                .as_ref()
                .map(|action| action.action_ref.clone()),
        }
    }
}

/// Run-config row rendered by the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildIntelligencePanelRunConfigRow {
    /// Run-config card id.
    pub card_id: String,
    /// Command id.
    pub command_id: String,
    /// Stable target id.
    pub stable_target_id: String,
    /// Lane token.
    pub lane_type_token: String,
    /// Exactness token.
    pub exactness_status_token: String,
    /// High-trust posture token.
    pub high_trust_action_posture_token: String,
    /// Imported-versus-live note.
    pub imported_vs_live_note: String,
}

impl BuildIntelligencePanelRunConfigRow {
    fn project(card: &BuildIntelligenceRunConfigCard) -> Self {
        Self {
            card_id: card.card_id.clone(),
            command_id: card.command_id.clone(),
            stable_target_id: card.stable_target_id.clone(),
            lane_type_token: card.lane_type_token.clone(),
            exactness_status_token: card.exactness_status_token.clone(),
            high_trust_action_posture_token: card.high_trust_action_posture_token.clone(),
            imported_vs_live_note: card.imported_vs_live_note.clone(),
        }
    }
}

/// Receipt row rendered by the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildIntelligencePanelReceiptRow {
    /// Receipt id.
    pub receipt_id: String,
    /// Command id.
    pub command_id: String,
    /// Run id.
    pub run_id: String,
    /// Stable target id.
    pub stable_target_id: String,
    /// Lane token.
    pub lane_type_token: String,
    /// Environment or host label.
    pub environment_or_host: String,
    /// Artifact-source token.
    pub artifact_source_token: String,
    /// Imported-versus-live token.
    pub imported_live_state_token: String,
    /// High-trust posture token.
    pub high_trust_action_posture_token: String,
    /// Imported or replayed note.
    pub imported_or_replayed_note: String,
}

impl BuildIntelligencePanelReceiptRow {
    fn project(receipt: &BuildIntelligenceReceipt) -> Self {
        Self {
            receipt_id: receipt.receipt_id.clone(),
            command_id: receipt.command_id.clone(),
            run_id: receipt.run_id.clone(),
            stable_target_id: receipt.stable_target_id.clone(),
            lane_type_token: receipt.lane_type_token.clone(),
            environment_or_host: receipt.environment_or_host.clone(),
            artifact_source_token: receipt.artifact_source_token.clone(),
            imported_live_state_token: receipt.imported_live_state_token.clone(),
            high_trust_action_posture_token: receipt.high_trust_action_posture_token.clone(),
            imported_or_replayed_note: receipt.imported_or_replayed_note.clone(),
        }
    }
}

/// Discovery-diff summary rendered by the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildIntelligencePanelDiffRow {
    /// Diff id.
    pub diff_id: String,
    /// Previous refresh id.
    pub previous_refresh_id: String,
    /// Current refresh id.
    pub current_refresh_id: String,
    /// Added count.
    pub added_count: usize,
    /// Removed count.
    pub removed_count: usize,
    /// Renamed count.
    pub renamed_count: usize,
    /// Downgraded-confidence count.
    pub downgraded_confidence_count: usize,
    /// Newly heuristic count.
    pub newly_heuristic_count: usize,
    /// Newly exact count.
    pub newly_exact_count: usize,
    /// Now-unresolved count.
    pub now_unresolved_count: usize,
    /// Flattened review lines.
    pub review_lines: Vec<String>,
}

impl BuildIntelligencePanelDiffRow {
    fn project(diff: &DiscoveryDiffReview) -> Self {
        let mut review_lines = Vec::new();
        append_diff_lines(&mut review_lines, &diff.added);
        append_diff_lines(&mut review_lines, &diff.removed);
        append_diff_lines(&mut review_lines, &diff.renamed);
        append_diff_lines(&mut review_lines, &diff.downgraded_confidence);
        append_diff_lines(&mut review_lines, &diff.newly_heuristic);
        append_diff_lines(&mut review_lines, &diff.newly_exact);
        append_diff_lines(&mut review_lines, &diff.now_unresolved);
        Self {
            diff_id: diff.diff_id.clone(),
            previous_refresh_id: diff.previous_refresh_id.clone(),
            current_refresh_id: diff.current_refresh_id.clone(),
            added_count: diff.added.len(),
            removed_count: diff.removed.len(),
            renamed_count: diff.renamed.len(),
            downgraded_confidence_count: diff.downgraded_confidence.len(),
            newly_heuristic_count: diff.newly_heuristic.len(),
            newly_exact_count: diff.newly_exact.len(),
            now_unresolved_count: diff.now_unresolved.len(),
            review_lines,
        }
    }
}

/// Build-intelligence panel rendered into shell surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildIntelligenceBetaPanel {
    /// Stable record kind.
    pub record_kind: String,
    /// Panel schema version.
    pub schema_version: u32,
    /// Support-export id.
    pub support_export_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Export timestamp.
    pub generated_at: String,
    /// Header notice.
    pub notice: String,
    /// Adapter-health rows.
    pub health_rows: Vec<BuildIntelligencePanelHealthRow>,
    /// Target rows.
    pub target_rows: Vec<BuildIntelligencePanelTargetRow>,
    /// Run-config rows.
    pub run_config_rows: Vec<BuildIntelligencePanelRunConfigRow>,
    /// Receipt rows.
    pub receipt_rows: Vec<BuildIntelligencePanelReceiptRow>,
    /// Discovery-diff rows.
    pub diff_rows: Vec<BuildIntelligencePanelDiffRow>,
}

impl BuildIntelligenceBetaPanel {
    /// Builds a shell panel from a runtime support export.
    pub fn from_support_export(export: &BuildIntelligenceSupportExport) -> Self {
        Self {
            record_kind: BUILD_INTELLIGENCE_BETA_PANEL_RECORD_KIND.to_owned(),
            schema_version: BUILD_INTELLIGENCE_BETA_PANEL_SCHEMA_VERSION,
            support_export_id: export.support_export_id.clone(),
            workspace_id: export.workspace_id.clone(),
            generated_at: export.generated_at.clone(),
            notice: BUILD_INTELLIGENCE_BETA_NOTICE.to_owned(),
            health_rows: export
                .adapter_health_strips
                .iter()
                .map(BuildIntelligencePanelHealthRow::project)
                .collect(),
            target_rows: export
                .target_rows
                .iter()
                .map(BuildIntelligencePanelTargetRow::project)
                .collect(),
            run_config_rows: export
                .run_config_cards
                .iter()
                .map(BuildIntelligencePanelRunConfigRow::project)
                .collect(),
            receipt_rows: export
                .receipts
                .iter()
                .map(BuildIntelligencePanelReceiptRow::project)
                .collect(),
            diff_rows: export
                .discovery_diffs
                .iter()
                .map(BuildIntelligencePanelDiffRow::project)
                .collect(),
        }
    }

    /// Deterministic plaintext block for shell, CLI/headless, and support.
    pub fn render_plaintext(&self) -> String {
        let mut out = format!(
            "Build intelligence\nWorkspace: {}\nExport: {}\nCaptured at: {}\nNotice: {}\n",
            self.workspace_id, self.support_export_id, self.generated_at, self.notice
        );
        out.push_str("Adapter health\n");
        for row in &self.health_rows {
            out.push_str(&format!(
                "- {} | lane={} | adapter={} | state={} | reason={}\n",
                row.strip_id,
                row.lane_type_token,
                row.adapter_id,
                row.state_token,
                row.health_reason_token.as_deref().unwrap_or("none"),
            ));
            if let Some(action_ref) = &row.continue_local_action_ref {
                out.push_str(&format!("  continue-local: {action_ref}\n"));
            }
            if let Some(action_ref) = &row.inspect_only_action_ref {
                out.push_str(&format!("  inspect-only: {action_ref}\n"));
            }
        }
        out.push_str("Targets\n");
        for row in &self.target_rows {
            out.push_str(&format!(
                "- {} ({}) | lane={} | exactness={} | provenance={} | {}\n",
                row.stable_target_id,
                row.display_name,
                row.lane_type_token,
                row.exactness_status_token,
                row.imported_live_state_token,
                row.imported_vs_live_note,
            ));
        }
        out.push_str("Run configs\n");
        for row in &self.run_config_rows {
            out.push_str(&format!(
                "- {} | command={} | target={} | posture={}\n",
                row.card_id,
                row.command_id,
                row.stable_target_id,
                row.high_trust_action_posture_token,
            ));
        }
        out.push_str("Receipts\n");
        for row in &self.receipt_rows {
            out.push_str(&format!(
                "- {} | command={} | target={} | lane={} | artifact={} | provenance={} | posture={} | {}\n",
                row.receipt_id,
                row.command_id,
                row.stable_target_id,
                row.lane_type_token,
                row.artifact_source_token,
                row.imported_live_state_token,
                row.high_trust_action_posture_token,
                row.imported_or_replayed_note,
            ));
        }
        out.push_str("Discovery diffs\n");
        for row in &self.diff_rows {
            out.push_str(&format!(
                "- {} | added={} removed={} renamed={} downgraded={} newly_heuristic={} newly_exact={} now_unresolved={}\n",
                row.diff_id,
                row.added_count,
                row.removed_count,
                row.renamed_count,
                row.downgraded_confidence_count,
                row.newly_heuristic_count,
                row.newly_exact_count,
                row.now_unresolved_count,
            ));
            for line in &row.review_lines {
                out.push_str(&format!("  - {line}\n"));
            }
        }
        out
    }
}

fn append_diff_lines(lines: &mut Vec<String>, items: &[DiscoveryDiffItem]) {
    lines.extend(
        items
            .iter()
            .map(|item| format!("{}: {}", item.change_token, item.summary)),
    );
}

#[cfg(test)]
mod tests;
