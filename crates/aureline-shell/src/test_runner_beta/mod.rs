//! Beta test runner shell projection.
//!
//! This module is a thin shell consumer over
//! [`aureline_runtime::TestRunnerBetaSupportExport`]. The shell does not own
//! test-runner identity; it projects the runtime-minted records into reviewable
//! tree-row, inline-row, parity-row, and artifact-row blocks suitable for the
//! support-export clipboard action and the test-tree picker.

use serde::{Deserialize, Serialize};

use aureline_runtime::{
    InlineTestResultRow, TestArtifactIdentity, TestRunnerBetaParityState,
    TestRunnerBetaRerunParity, TestRunnerBetaSupportExport, TestTreeRow, TestTreeRowKind,
};

/// Stable record-kind tag carried in serialized test-runner-beta projections.
pub const TEST_RUNNER_BETA_PROJECTION_RECORD_KIND: &str = "test_runner_beta_projection_record";

/// Schema version for the projection payload.
pub const TEST_RUNNER_BETA_PROJECTION_SCHEMA_VERSION: u32 = 1;

/// Header notice rendered above the test-tree rows.
pub const TEST_RUNNER_BETA_NOTICE: &str = "Test runner (beta): tree, inline, and \
rerun-last share one identity. Surface disagreement requires review before \
dispatch; rerun-last is unavailable until the first attempt has been \
remembered.";

/// One reviewable tree row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestRunnerBetaTreeRowProjection {
    pub tree_row_id: String,
    pub row_kind_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_row_id: Option<String>,
    pub display_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_file_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_number: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canonical_test_item_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test_session_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_attempt_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_result_state_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerun_last_command_id: Option<String>,
    pub child_row_ids: Vec<String>,
}

impl TestRunnerBetaTreeRowProjection {
    fn project(row: &TestTreeRow) -> Self {
        Self {
            tree_row_id: row.tree_row_id.clone(),
            row_kind_token: row.row_kind_token.clone(),
            parent_row_id: row.parent_row_id.clone(),
            display_label: row.display_label.clone(),
            source_file_ref: row.source_file_ref.clone(),
            line_number: row.line_number,
            canonical_test_item_ref: row.canonical_test_item_ref.clone(),
            selector_ref: row.selector_ref.clone(),
            test_session_ref: row.test_session_ref.clone(),
            latest_attempt_ref: row.latest_attempt_ref.clone(),
            latest_result_state_token: row.latest_result_state_token.clone(),
            rerun_last_command_id: row.rerun_last_command_id.clone(),
            child_row_ids: row.child_row_ids.clone(),
        }
    }
}

/// One reviewable inline-result row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestRunnerBetaInlineRowProjection {
    pub inline_row_id: String,
    pub canonical_test_item_ref: String,
    pub selector_ref: String,
    pub test_session_ref: String,
    pub source_file_ref: String,
    pub line_number: u32,
    pub tree_row_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_attempt_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_result_state_token: Option<String>,
    pub summary: String,
}

impl TestRunnerBetaInlineRowProjection {
    fn project(row: &InlineTestResultRow) -> Self {
        Self {
            inline_row_id: row.inline_row_id.clone(),
            canonical_test_item_ref: row.canonical_test_item_ref.clone(),
            selector_ref: row.selector_ref.clone(),
            test_session_ref: row.test_session_ref.clone(),
            source_file_ref: row.source_file_ref.clone(),
            line_number: row.line_number,
            tree_row_ref: row.tree_row_ref.clone(),
            latest_attempt_ref: row.latest_attempt_ref.clone(),
            latest_result_state_token: row.latest_result_state_token.clone(),
            summary: row.summary.clone(),
        }
    }
}

/// One reviewable parity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestRunnerBetaParityRowProjection {
    pub parity_id: String,
    pub test_session_ref: String,
    pub canonical_test_item_ref: String,
    pub tree_row_ref: String,
    pub inline_row_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_attempt_ref: Option<String>,
    pub agreement_state_token: String,
    pub rerun_last_command_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerun_dispatch_state_token: Option<String>,
    pub summary: String,
}

impl TestRunnerBetaParityRowProjection {
    fn project(row: &TestRunnerBetaRerunParity) -> Self {
        Self {
            parity_id: row.parity_id.clone(),
            test_session_ref: row.test_session_ref.clone(),
            canonical_test_item_ref: row.canonical_test_item_ref.clone(),
            tree_row_ref: row.tree_row_ref.clone(),
            inline_row_ref: row.inline_row_ref.clone(),
            latest_attempt_ref: row.latest_attempt_ref.clone(),
            agreement_state_token: row.agreement_state_token.clone(),
            rerun_last_command_id: row.rerun_last_command_id.clone(),
            rerun_dispatch_state_token: row.rerun_dispatch_state_token.clone(),
            summary: row.summary.clone(),
        }
    }
}

/// One reviewable artifact identity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestRunnerBetaArtifactIdentityProjection {
    pub artifact_identity_id: String,
    pub artifact_kind_token: String,
    pub artifact_ref: String,
    pub test_session_ref: String,
    pub selector_ref: String,
    pub canonical_test_item_refs: Vec<String>,
    pub test_attempt_ref: String,
    pub identity_stability_token: String,
}

impl TestRunnerBetaArtifactIdentityProjection {
    fn project(identity: &TestArtifactIdentity) -> Self {
        Self {
            artifact_identity_id: identity.artifact_identity_id.clone(),
            artifact_kind_token: identity.artifact_kind_token.clone(),
            artifact_ref: identity.artifact_ref.clone(),
            test_session_ref: identity.test_session_ref.clone(),
            selector_ref: identity.selector_ref.clone(),
            canonical_test_item_refs: identity.canonical_test_item_refs.clone(),
            test_attempt_ref: identity.test_attempt_ref.clone(),
            identity_stability_token: identity.identity_stability_token.clone(),
        }
    }
}

/// Beta test-runner projection rendered into the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestRunnerBetaProjectionView {
    pub record_kind: String,
    pub schema_version: u32,
    pub workspace_id: String,
    pub support_export_id: String,
    pub captured_at: String,
    pub notice: String,
    pub framework_tokens: Vec<String>,
    pub tree_rows: Vec<TestRunnerBetaTreeRowProjection>,
    pub inline_rows: Vec<TestRunnerBetaInlineRowProjection>,
    pub parity_rows: Vec<TestRunnerBetaParityRowProjection>,
    pub artifact_rows: Vec<TestRunnerBetaArtifactIdentityProjection>,
    pub honesty_marker_present: bool,
    pub attempt_packet_refs: Vec<String>,
    pub attempt_support_export_refs: Vec<String>,
    pub summary_lines: Vec<String>,
}

impl TestRunnerBetaProjectionView {
    /// Project a beta test-runner surface from a runtime support export plus
    /// the matched tree and inline projections.
    pub fn project(
        export: &TestRunnerBetaSupportExport,
        tree_rows: &[TestTreeRow],
        inline_rows: &[InlineTestResultRow],
    ) -> Self {
        let tree_projections: Vec<TestRunnerBetaTreeRowProjection> = tree_rows
            .iter()
            .map(TestRunnerBetaTreeRowProjection::project)
            .collect();
        let inline_projections: Vec<TestRunnerBetaInlineRowProjection> = inline_rows
            .iter()
            .map(TestRunnerBetaInlineRowProjection::project)
            .collect();
        let parity_projections: Vec<TestRunnerBetaParityRowProjection> = export
            .parity_rows
            .iter()
            .map(TestRunnerBetaParityRowProjection::project)
            .collect();
        let artifact_projections: Vec<TestRunnerBetaArtifactIdentityProjection> = export
            .artifact_identities
            .iter()
            .map(TestRunnerBetaArtifactIdentityProjection::project)
            .collect();
        let framework_tokens = export
            .coverage_manifest
            .frameworks
            .iter()
            .map(|row| row.framework_token.clone())
            .collect();
        let honesty_marker_present = tree_rows
            .iter()
            .find(|row| row.row_kind == TestTreeRowKind::WorkspaceRoot)
            .map(|_| {
                export.parity_rows.iter().any(|row| {
                    row.agreement_state
                        == TestRunnerBetaParityState::SurfaceDisagreementRequiresReview
                })
            })
            .unwrap_or(false);
        Self {
            record_kind: TEST_RUNNER_BETA_PROJECTION_RECORD_KIND.to_owned(),
            schema_version: TEST_RUNNER_BETA_PROJECTION_SCHEMA_VERSION,
            workspace_id: export.workspace_id.clone(),
            support_export_id: export.support_export_id.clone(),
            captured_at: export.generated_at.clone(),
            notice: TEST_RUNNER_BETA_NOTICE.to_owned(),
            framework_tokens,
            tree_rows: tree_projections,
            inline_rows: inline_projections,
            parity_rows: parity_projections,
            artifact_rows: artifact_projections,
            honesty_marker_present,
            attempt_packet_refs: export.attempt_packet_refs.clone(),
            attempt_support_export_refs: export.attempt_support_export_refs.clone(),
            summary_lines: export.summary_lines.clone(),
        }
    }

    /// Deterministic plaintext block for the support-export clipboard action.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Test runner (beta)\n");
        out.push_str(&format!("Workspace: {}\n", self.workspace_id));
        out.push_str(&format!("Export: {}\n", self.support_export_id));
        out.push_str(&format!("Captured at: {}\n", self.captured_at));
        out.push_str(&format!("Notice: {}\n", self.notice));
        out.push_str(&format!(
            "Frameworks: {}\n",
            self.framework_tokens.join(",")
        ));
        out.push_str(&format!("Tree rows: {}\n", self.tree_rows.len()));
        for row in &self.tree_rows {
            let mut line = format!(
                "  - [{}] {} (id={})",
                row.row_kind_token, row.display_label, row.tree_row_id
            );
            if let Some(canonical) = &row.canonical_test_item_ref {
                line.push_str(&format!(" canonical={canonical}"));
            }
            if let Some(selector) = &row.selector_ref {
                line.push_str(&format!(" selector={selector}"));
            }
            if let Some(session) = &row.test_session_ref {
                line.push_str(&format!(" session={session}"));
            }
            if let Some(state) = &row.latest_result_state_token {
                line.push_str(&format!(" state={state}"));
            }
            line.push('\n');
            out.push_str(&line);
        }
        out.push_str(&format!("Inline rows: {}\n", self.inline_rows.len()));
        for row in &self.inline_rows {
            out.push_str(&format!(
                "  - {} selector={} session={} @ {}:{} -> tree={} ({})\n",
                row.canonical_test_item_ref,
                row.selector_ref,
                row.test_session_ref,
                row.source_file_ref,
                row.line_number,
                row.tree_row_ref,
                row.summary
            ));
        }
        out.push_str(&format!("Parity rows: {}\n", self.parity_rows.len()));
        for row in &self.parity_rows {
            out.push_str(&format!(
                "  - {} session={} canonical={} tree={} inline={} command={} state={}\n",
                row.parity_id,
                row.test_session_ref,
                row.canonical_test_item_ref,
                row.tree_row_ref,
                row.inline_row_ref,
                row.rerun_last_command_id,
                row.agreement_state_token
            ));
        }
        out.push_str(&format!("Artifact rows: {}\n", self.artifact_rows.len()));
        for row in &self.artifact_rows {
            out.push_str(&format!(
                "  - [{}] {} session={} selector={} attempt={} stability={}\n",
                row.artifact_kind_token,
                row.artifact_ref,
                row.test_session_ref,
                row.selector_ref,
                row.test_attempt_ref,
                row.identity_stability_token
            ));
        }
        out.push_str(&format!(
            "Attempt packets: {}\n",
            self.attempt_packet_refs.len()
        ));
        for line in &self.summary_lines {
            out.push_str(line);
            out.push('\n');
        }
        out
    }
}

#[cfg(test)]
mod tests;
