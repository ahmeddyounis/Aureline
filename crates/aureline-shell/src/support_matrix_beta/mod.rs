//! Beta debugger and execution-context support-matrix shell projection.
//!
//! Thin shell consumer over the canonical
//! [`aureline_runtime::SupportMatrixBetaManifest`]. The shell does not own
//! support-matrix truth; it projects the runtime-minted rows into reviewable
//! plaintext lines suitable for the support-export clipboard action, the
//! Help / About panel, and the migration / partner / release packets that
//! quote which run / test / debug / execution-context capabilities each
//! claimed wedge supports today and how support narrows when state drifts.

use serde::{Deserialize, Serialize};

use aureline_runtime::{
    SupportMatrixBetaManifest, SupportMatrixBetaSupportExport, SupportMatrixWedgeRow,
};

/// Stable record-kind tag carried in serialised panel projections.
pub const SUPPORT_MATRIX_BETA_PANEL_RECORD_KIND: &str = "support_matrix_beta_panel_record";

/// Schema version of the shell panel projection.
pub const SUPPORT_MATRIX_BETA_PANEL_SCHEMA_VERSION: u32 = 1;

/// Header notice rendered above the matrix rows.
pub const SUPPORT_MATRIX_BETA_NOTICE: &str =
    "Beta debugger and execution-context support matrix: rows in Preview or \
     Limited classes cannot dispatch protected run / test / debug work \
     without explicit review.";

/// One reviewable matrix row projected into the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportMatrixBetaPanelRow {
    pub wedge_token: String,
    pub wedge_label: String,
    pub launch_class_token: String,
    pub launch_summary: String,
    pub attach_class_token: String,
    pub attach_summary: String,
    pub test_class_token: String,
    pub test_claimed_framework_tokens: Vec<String>,
    pub test_previewed_framework_tokens: Vec<String>,
    pub test_summary: String,
    pub context_overall_class_token: String,
    pub context_lane_lines: Vec<String>,
    pub downgrade_rule_tokens: Vec<String>,
    pub allows_protected_dispatch: bool,
}

impl SupportMatrixBetaPanelRow {
    fn project(row: &SupportMatrixWedgeRow) -> Self {
        let context_lane_lines = row
            .execution_context
            .lanes
            .iter()
            .map(|lane| format!("{}={}", lane.lane_token, lane.class_token))
            .collect();
        Self {
            wedge_token: row.wedge_token.clone(),
            wedge_label: row.wedge_label.clone(),
            launch_class_token: row.launch.class_token.clone(),
            launch_summary: row.launch.summary.clone(),
            attach_class_token: row.attach.class_token.clone(),
            attach_summary: row.attach.summary.clone(),
            test_class_token: row.test.class_token.clone(),
            test_claimed_framework_tokens: row.test.claimed_framework_tokens.clone(),
            test_previewed_framework_tokens: row.test.previewed_framework_tokens.clone(),
            test_summary: row.test.summary.clone(),
            context_overall_class_token: row.execution_context.overall_class_token.clone(),
            context_lane_lines,
            downgrade_rule_tokens: row.downgrade_rule_tokens.clone(),
            allows_protected_dispatch: row.allows_protected_dispatch(),
        }
    }
}

/// Beta support-matrix panel rendered into the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportMatrixBetaPanel {
    pub record_kind: String,
    pub schema_version: u32,
    pub manifest_id: String,
    pub generated_at: String,
    pub notice: String,
    pub rows: Vec<SupportMatrixBetaPanelRow>,
    pub any_row_blocks_protected_dispatch: bool,
}

impl SupportMatrixBetaPanel {
    /// Builds a shell panel directly from a runtime manifest.
    pub fn from_manifest(manifest: &SupportMatrixBetaManifest) -> Self {
        let rows: Vec<SupportMatrixBetaPanelRow> = manifest
            .rows
            .iter()
            .map(SupportMatrixBetaPanelRow::project)
            .collect();
        let any_row_blocks_protected_dispatch =
            rows.iter().any(|row| !row.allows_protected_dispatch);
        Self {
            record_kind: SUPPORT_MATRIX_BETA_PANEL_RECORD_KIND.to_owned(),
            schema_version: SUPPORT_MATRIX_BETA_PANEL_SCHEMA_VERSION,
            manifest_id: manifest.manifest_id.clone(),
            generated_at: manifest.generated_at.clone(),
            notice: SUPPORT_MATRIX_BETA_NOTICE.to_owned(),
            rows,
            any_row_blocks_protected_dispatch,
        }
    }

    /// Convenience: build the shell panel from a runtime support export.
    pub fn from_support_export(export: &SupportMatrixBetaSupportExport) -> Self {
        Self::from_manifest(&export.manifest)
    }

    /// Deterministic plaintext block for the support-export clipboard
    /// action and CLI / Help / About surfaces. Iterates rows and lanes in
    /// their canonical order.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Beta debugger and execution-context support matrix\n");
        out.push_str(&format!("Manifest: {}\n", self.manifest_id));
        out.push_str(&format!("Captured at: {}\n", self.generated_at));
        out.push_str(&format!("Notice: {}\n", self.notice));
        out.push_str(&format!(
            "Rows: {} (any protected dispatch blocked: {})\n",
            self.rows.len(),
            self.any_row_blocks_protected_dispatch
        ));
        for row in &self.rows {
            out.push_str(&format!("\nWedge: {} ({})\n", row.wedge_token, row.wedge_label));
            out.push_str(&format!(
                "  Launch: {} — {}\n",
                row.launch_class_token, row.launch_summary
            ));
            out.push_str(&format!(
                "  Attach: {} — {}\n",
                row.attach_class_token, row.attach_summary
            ));
            out.push_str(&format!("  Test: {}\n", row.test_class_token));
            out.push_str(&format!(
                "    Claimed frameworks: {}\n",
                if row.test_claimed_framework_tokens.is_empty() {
                    "(none)".to_owned()
                } else {
                    row.test_claimed_framework_tokens.join(",")
                }
            ));
            if !row.test_previewed_framework_tokens.is_empty() {
                out.push_str(&format!(
                    "    Previewed frameworks: {}\n",
                    row.test_previewed_framework_tokens.join(",")
                ));
            }
            out.push_str(&format!("    Summary: {}\n", row.test_summary));
            out.push_str(&format!(
                "  Execution-context rollup: {}\n",
                row.context_overall_class_token
            ));
            for line in &row.context_lane_lines {
                out.push_str(&format!("    - {line}\n"));
            }
            out.push_str("  Downgrade rules:\n");
            for token in &row.downgrade_rule_tokens {
                out.push_str(&format!("    - {token}\n"));
            }
            out.push_str(&format!(
                "  Protected dispatch allowed: {}\n",
                row.allows_protected_dispatch
            ));
        }
        out
    }
}

#[cfg(test)]
mod tests;
