//! Beta run / debug launch and attach profile shell projection.
//!
//! This module is a thin shell consumer over the canonical
//! [`aureline_runtime::LaunchProfileSupportExport`]. The shell does not
//! own launch-profile lifecycle truth; it projects the runtime-minted
//! records into reviewable rows and a deterministic plaintext block
//! suitable for the support-export clipboard action and the run-and-debug
//! picker.

use serde::{Deserialize, Serialize};

use aureline_runtime::{
    LaunchProfileEdit, LaunchProfilePreview, LaunchProfilePreviewState, LaunchProfileSupportExport,
    LaunchProfileSupportRow,
};

/// Stable record-kind tag carried in serialized run-debug-profile projections.
pub const RUN_DEBUG_PROFILES_BETA_PROJECTION_RECORD_KIND: &str =
    "run_debug_profiles_beta_projection_record";

/// Schema version for the projection payload.
pub const RUN_DEBUG_PROFILES_BETA_PROJECTION_SCHEMA_VERSION: u32 = 1;

/// Header notice rendered above the profile rows.
pub const RUN_DEBUG_PROFILES_BETA_NOTICE: &str =
    "Run / debug profiles (beta): edits are durable and reversible. Drift, \
     unreachable targets, or invalid configurations must be reviewed before \
     dispatch.";

/// One reviewable profile row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunDebugProfilesBetaRow {
    pub profile_id: String,
    pub display_name: String,
    pub current_revision_id: String,
    pub mode_token: String,
    pub kind_token: String,
    pub canonical_target_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adapter_id: Option<String>,
    pub capsule_id: String,
    pub revision_count: usize,
    pub edit_lineage_summary: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_state_token: Option<String>,
    pub requires_review_before_dispatch: bool,
    pub current_target_reachable: bool,
    pub side_effect_disclosure_tokens: Vec<String>,
    pub drift_field_paths: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invalid_reason_token: Option<String>,
    pub summary: String,
}

impl RunDebugProfilesBetaRow {
    fn project(row: &LaunchProfileSupportRow) -> Self {
        let preview = row.latest_preview.as_ref();
        let preview_state_token = preview.map(|p| p.state_token.clone());
        let requires_review_before_dispatch = preview
            .map(|p| p.requires_review_before_dispatch)
            .unwrap_or(false);
        let current_target_reachable = preview.map(|p| p.current_target_reachable).unwrap_or(false);
        let side_effect_disclosure_tokens = preview
            .map(|p| p.side_effect_disclosure_tokens.clone())
            .unwrap_or_default();
        let drift_field_paths = preview
            .map(|p| p.drift_rows.iter().map(|r| r.field_path.clone()).collect())
            .unwrap_or_default();
        let invalid_reason_token = preview.and_then(|p| p.invalid_reason_token.clone());
        let summary = preview
            .map(summarize_preview)
            .unwrap_or_else(|| format!("Profile `{}` (no current preview)", row.display_name));
        Self {
            profile_id: row.profile_id.clone(),
            display_name: row.display_name.clone(),
            current_revision_id: row.current_revision_id.clone(),
            mode_token: row.mode_token.clone(),
            kind_token: row.kind_token.clone(),
            canonical_target_id: row.canonical_target_id.clone(),
            adapter_id: row.adapter_id.clone(),
            capsule_id: row.capsule_id.clone(),
            revision_count: row.revision_count,
            edit_lineage_summary: row.edit_lineage.iter().map(summarize_edit).collect(),
            preview_state_token,
            requires_review_before_dispatch,
            current_target_reachable,
            side_effect_disclosure_tokens,
            drift_field_paths,
            invalid_reason_token,
            summary,
        }
    }
}

fn summarize_edit(edit: &LaunchProfileEdit) -> String {
    format!(
        "{} @ {}: {}",
        edit.edit_class_token, edit.observed_at, edit.summary
    )
}

fn summarize_preview(preview: &LaunchProfilePreview) -> String {
    preview.summary_headline.clone()
}

/// Beta run-debug-profile projection rendered into the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunDebugProfilesBetaProjection {
    pub record_kind: String,
    pub schema_version: u32,
    pub workspace_id: String,
    pub export_id: String,
    pub captured_at: String,
    pub notice: String,
    pub profiles: Vec<RunDebugProfilesBetaRow>,
    pub honesty_marker_present: bool,
    pub export_safe_summary: String,
}

impl RunDebugProfilesBetaProjection {
    /// Project a run-debug profile beta surface from a runtime support export.
    pub fn project(export: &LaunchProfileSupportExport) -> Self {
        let profiles: Vec<RunDebugProfilesBetaRow> = export
            .profile_rows
            .iter()
            .map(RunDebugProfilesBetaRow::project)
            .collect();
        let ready_token = LaunchProfilePreviewState::ReadyToDispatch.as_str();
        let honesty_marker_present = export.honesty_marker_present
            || profiles.iter().any(|row| {
                row.preview_state_token
                    .as_deref()
                    .is_some_and(|token| token != ready_token)
            });
        Self {
            record_kind: RUN_DEBUG_PROFILES_BETA_PROJECTION_RECORD_KIND.to_owned(),
            schema_version: RUN_DEBUG_PROFILES_BETA_PROJECTION_SCHEMA_VERSION,
            workspace_id: export.workspace_id.clone(),
            export_id: export.export_id.clone(),
            captured_at: export.generated_at.clone(),
            notice: RUN_DEBUG_PROFILES_BETA_NOTICE.to_owned(),
            profiles,
            honesty_marker_present,
            export_safe_summary: export.summary_headline.clone(),
        }
    }

    /// Deterministic plaintext block for the support-export clipboard action.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Run / debug profiles (beta)\n");
        out.push_str(&format!("Workspace: {}\n", self.workspace_id));
        out.push_str(&format!("Export: {}\n", self.export_id));
        out.push_str(&format!("Captured at: {}\n", self.captured_at));
        out.push_str(&format!("Notice: {}\n", self.notice));
        out.push_str(&format!("Profiles: {}\n", self.profiles.len()));
        for row in &self.profiles {
            out.push_str(&format!("\nProfile: {}\n", row.display_name));
            out.push_str(&format!("  Id: {}\n", row.profile_id));
            out.push_str(&format!(
                "  Mode: {} | Kind: {} | Revision: {}\n",
                row.mode_token, row.kind_token, row.current_revision_id
            ));
            out.push_str(&format!("  Target: {}\n", row.canonical_target_id));
            if let Some(adapter) = &row.adapter_id {
                out.push_str(&format!("  Adapter: {adapter}\n"));
            }
            out.push_str(&format!("  Capsule: {}\n", row.capsule_id));
            if let Some(state) = &row.preview_state_token {
                out.push_str(&format!(
                    "  Preview: {} (review required: {})\n",
                    state, row.requires_review_before_dispatch
                ));
            }
            if !row.side_effect_disclosure_tokens.is_empty() {
                out.push_str(&format!(
                    "  Side effects: {}\n",
                    row.side_effect_disclosure_tokens.join(",")
                ));
            }
            if !row.drift_field_paths.is_empty() {
                out.push_str(&format!("  Drift: {}\n", row.drift_field_paths.join(",")));
            }
            if let Some(token) = &row.invalid_reason_token {
                out.push_str(&format!("  Invalid reason: {token}\n"));
            }
            out.push_str("  Edit lineage:\n");
            for line in &row.edit_lineage_summary {
                out.push_str(&format!("    - {line}\n"));
            }
            out.push_str(&format!("  Summary: {}\n", row.summary));
        }
        out
    }
}

#[cfg(test)]
mod tests;
