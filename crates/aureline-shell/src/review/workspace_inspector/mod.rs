//! Shell review-workspace inspector — beta consumer of the durable
//! review-workspace packet.
//!
//! This module reads the checked-in beta review-workspace fixtures,
//! projects them through `aureline-review`, and renders deterministic
//! rows for the review workspace inspector. The inspector shows whether
//! anchors are durable, check freshness is browser-independent, browser
//! handoff is typed and reversible, and support/export can reopen the
//! context.

use std::fmt;

use aureline_review::{
    project_review_workspace_beta_packet, DiffFileInput, DiffOpenTarget, DiffViewSurfacePacket,
    ReviewWorkspaceBetaError, ReviewWorkspaceBetaInput, ReviewWorkspaceBetaPacket,
    ReviewWorkspaceBetaProjection, ReviewWorkspaceBetaValidationError, ReviewWorkspaceSeedInput,
    ReviewWorkspaceSeedPacket,
};
use serde::Deserialize;

const BETA_REVIEW_WORKSPACE_ROWS: &[(&str, &str, &str)] = &[
    (
        "fixtures/review/m3/review_workspace_beta/local_workspace_with_reversible_browser_handoff.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m3/review_workspace_beta/local_workspace_with_reversible_browser_handoff.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/workspace_seed_alpha/local_diff_with_work_item_link.yaml"
        )),
    ),
    (
        "fixtures/review/m3/review_workspace_beta/stale_check_blocks_operator_truth.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m3/review_workspace_beta/stale_check_blocks_operator_truth.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/workspace_seed_alpha/local_diff_with_work_item_link.yaml"
        )),
    ),
];

/// Presentation label rendered for the review-workspace inspector surface.
pub const REVIEW_WORKSPACE_INSPECTOR_PRESENTATION_LABEL: &str = "Review workspace inspector";

/// Presentation subtitle rendered for the review-workspace inspector surface.
pub const REVIEW_WORKSPACE_INSPECTOR_PRESENTATION_SUBTITLE: &str =
    "Inspect durable anchors, check freshness, browser handoff, and support reopen packets.";

#[derive(Debug, Deserialize)]
struct ReviewWorkspaceBetaFixture {
    beta_input: ReviewWorkspaceBetaInput,
}

#[derive(Debug, Deserialize)]
struct ReviewWorkspaceSeedFixture {
    change_list_row: ChangeListRowFixture,
    workspace_seed: ReviewWorkspaceSeedInput,
    diff: DiffFileInput,
}

#[derive(Debug, Deserialize)]
struct ChangeListRowFixture {
    row_ref: String,
    file_state_token: String,
}

/// One review-workspace row rendered in the inspector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewWorkspaceInspectorRow {
    /// Fixture path that produced this row.
    pub source_ref: &'static str,
    /// Stable packet identity.
    pub packet_id: String,
    /// Review workspace identity.
    pub review_workspace_id: String,
    /// Review workspace source class.
    pub review_workspace_source_class: String,
    /// Provider authority class.
    pub provider_authority_class: String,
    /// Workspace freshness class.
    pub freshness_class: String,
    /// Durable comment anchor count.
    pub durable_comment_anchor_count: usize,
    /// Check freshness row count.
    pub check_freshness_count: usize,
    /// Object lineage row count.
    pub object_lineage_count: usize,
    /// True when typed reversible browser handoff is present.
    pub typed_reversible_browser_handoff_present: bool,
    /// True when support/export can reopen the context.
    pub support_export_reopenable: bool,
    /// True when stale checks block operator-truth claims.
    pub stale_check_blocks_operator_truth: bool,
    /// Consumer surfaces wired through the support export.
    pub consumer_surfaces: Vec<String>,
    /// Redaction class used by the support export.
    pub redaction_class: String,
}

impl ReviewWorkspaceInspectorRow {
    fn from_projection(
        source_ref: &'static str,
        projection: ReviewWorkspaceBetaProjection,
    ) -> Self {
        Self {
            source_ref,
            packet_id: projection.packet_id,
            review_workspace_id: projection.review_workspace_id,
            review_workspace_source_class: projection.review_workspace_source_class,
            provider_authority_class: projection.provider_authority_class,
            freshness_class: projection.freshness_class,
            durable_comment_anchor_count: projection.durable_comment_anchor_count,
            check_freshness_count: projection.check_freshness_count,
            object_lineage_count: projection.object_lineage_count,
            typed_reversible_browser_handoff_present: projection
                .typed_reversible_browser_handoff_present,
            support_export_reopenable: projection.support_export_reopenable,
            stale_check_blocks_operator_truth: projection.stale_check_blocks_operator_truth,
            consumer_surfaces: projection.consumer_surfaces,
            redaction_class: projection.redaction_class,
        }
    }
}

/// Error returned when the inspector cannot project a review-workspace row.
#[derive(Debug)]
pub struct ReviewWorkspaceInspectorError {
    source_ref: &'static str,
    message: String,
}

impl ReviewWorkspaceInspectorError {
    /// Returns the artifact path that failed to project.
    pub const fn source_ref(&self) -> &'static str {
        self.source_ref
    }

    /// Returns the parse or validation failure.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ReviewWorkspaceInspectorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.source_ref, self.message)
    }
}

impl std::error::Error for ReviewWorkspaceInspectorError {}

/// Builds review-workspace inspector rows from checked-in beta fixtures.
///
/// # Errors
///
/// Returns [`ReviewWorkspaceInspectorError`] when any fixture fails to
/// parse or validate.
pub fn build_beta_review_workspace_rows(
) -> Result<Vec<ReviewWorkspaceInspectorRow>, ReviewWorkspaceInspectorError> {
    let mut rows = Vec::with_capacity(BETA_REVIEW_WORKSPACE_ROWS.len());
    for (source_ref, payload, seed_payload) in BETA_REVIEW_WORKSPACE_ROWS {
        let projection = project_fixture(source_ref, payload, seed_payload)?;
        rows.push(ReviewWorkspaceInspectorRow::from_projection(
            source_ref, projection,
        ));
    }
    Ok(rows)
}

/// Renders the beta review-workspace inspector projection as deterministic
/// plaintext for CLI/headless/docs/support consumers.
///
/// # Errors
///
/// Returns [`ReviewWorkspaceInspectorError`] when a fixture cannot be
/// projected.
pub fn render_beta_review_workspace_plaintext() -> Result<String, ReviewWorkspaceInspectorError> {
    let rows = build_beta_review_workspace_rows()?;
    let mut lines = vec![
        "Review workspace inspector beta".to_string(),
        "packet_id | workspace | source | authority | freshness | anchors | checks | lineage | browser_handoff | support_reopen | stale_blocks"
            .to_string(),
    ];
    for row in rows {
        lines.push(format!(
            "{} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {}",
            row.packet_id,
            row.review_workspace_id,
            row.review_workspace_source_class,
            row.provider_authority_class,
            row.freshness_class,
            row.durable_comment_anchor_count,
            row.check_freshness_count,
            row.object_lineage_count,
            row.typed_reversible_browser_handoff_present,
            row.support_export_reopenable,
            row.stale_check_blocks_operator_truth,
        ));
    }
    lines.push(String::new());
    Ok(lines.join("\n"))
}

fn project_fixture(
    source_ref: &'static str,
    payload: &str,
    seed_payload: &str,
) -> Result<ReviewWorkspaceBetaProjection, ReviewWorkspaceInspectorError> {
    let fixture: ReviewWorkspaceBetaFixture =
        serde_json::from_str(payload).map_err(|err| projection_error(source_ref, err))?;
    let seed_packet = seed_packet_for(source_ref, seed_payload)?;
    let packet = ReviewWorkspaceBetaPacket::from_seed_packet(fixture.beta_input, &seed_packet)
        .map_err(|err| validation_error(source_ref, err))?;
    let materialized =
        serde_json::to_string(&packet).map_err(|err| projection_error(source_ref, err))?;
    project_review_workspace_beta_packet(&materialized).map_err(|err| beta_error(source_ref, err))
}

fn seed_packet_for(
    source_ref: &'static str,
    seed_payload: &str,
) -> Result<ReviewWorkspaceSeedPacket, ReviewWorkspaceInspectorError> {
    let fixture: ReviewWorkspaceSeedFixture =
        serde_yaml::from_str(seed_payload).map_err(|err| projection_error(source_ref, err))?;
    let open_target = DiffOpenTarget::from_change_list_row_parts(
        &fixture.diff.workspace_ref,
        &fixture.diff.truth_source_ref,
        &fixture.change_list_row.row_ref,
        &fixture.diff.group_token,
        fixture.diff.path.clone(),
        fixture.diff.original_path.clone(),
        &fixture.diff.status_code,
        &fixture.change_list_row.file_state_token,
    );
    let diff_packet = DiffViewSurfacePacket::from_file_input(open_target, fixture.diff);
    Ok(ReviewWorkspaceSeedPacket::from_diff_packet(
        fixture.workspace_seed,
        &diff_packet,
    ))
}

fn projection_error<E: fmt::Display>(
    source_ref: &'static str,
    err: E,
) -> ReviewWorkspaceInspectorError {
    ReviewWorkspaceInspectorError {
        source_ref,
        message: err.to_string(),
    }
}

fn validation_error(
    source_ref: &'static str,
    err: ReviewWorkspaceBetaValidationError,
) -> ReviewWorkspaceInspectorError {
    ReviewWorkspaceInspectorError {
        source_ref,
        message: err.to_string(),
    }
}

fn beta_error(
    source_ref: &'static str,
    err: ReviewWorkspaceBetaError,
) -> ReviewWorkspaceInspectorError {
    ReviewWorkspaceInspectorError {
        source_ref,
        message: err.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beta_review_workspace_rows_project() {
        let rows = build_beta_review_workspace_rows().expect("beta review-workspace rows project");
        assert_eq!(rows.len(), 2);
        assert!(rows
            .iter()
            .any(|row| row.typed_reversible_browser_handoff_present));
        assert!(rows.iter().any(|row| row.stale_check_blocks_operator_truth));

        for row in &rows {
            assert!(row.support_export_reopenable);
            assert!(
                row.consumer_surfaces
                    .iter()
                    .any(|surface| surface == "support_export"),
                "support export consumer must be wired",
            );
            assert!(
                row.consumer_surfaces
                    .iter()
                    .any(|surface| surface == "cli_headless_entry"),
                "CLI/headless consumer must be wired",
            );
            assert_eq!(row.redaction_class, "metadata_safe_default");
        }
    }

    #[test]
    fn beta_review_workspace_plaintext_is_deterministic() {
        let first = render_beta_review_workspace_plaintext().expect("plaintext renders");
        let second = render_beta_review_workspace_plaintext().expect("plaintext renders");
        assert_eq!(first, second);
        assert!(first.contains("Review workspace inspector beta"));
        assert!(first.contains("review.workspace.beta.fixture.local_handoff"));
        assert!(first.contains("review.workspace.beta.fixture.stale_check"));
        assert!(first.contains("true"));
    }

    #[test]
    fn presentation_labels_are_quotable() {
        assert!(REVIEW_WORKSPACE_INSPECTOR_PRESENTATION_LABEL
            .to_lowercase()
            .contains("review workspace"));
        assert!(REVIEW_WORKSPACE_INSPECTOR_PRESENTATION_SUBTITLE
            .to_lowercase()
            .contains("browser handoff"));
    }
}
