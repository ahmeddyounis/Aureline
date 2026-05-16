//! Shell parity-harness inspector — alpha consumer of the review-pack
//! parity-harness family.
//!
//! This module reads the checked-in alpha parity-harness fixtures and
//! projects them into the deterministic parity-harness rows the
//! inspector renders **after** a local-lane and CI-lane run of one
//! upstream review-pack DSL record. Every row exposes the four review
//! questions the inspector exists to answer:
//!
//! 1. Which upstream pack was harnessed, and where does it anchor?
//! 2. Which lanes engaged or declined?
//! 3. What did each check do in each lane, and what is the resulting
//!    parity finding?
//! 4. Did drift downgrade the row, and what is the overall verdict?
//!
//! The shared data types are defined in
//! [`aureline_review::review_pack_parity_harness`] so review and support
//! packets re-use the same parity vocabulary without forking it per
//! surface.

use std::fmt;

use aureline_review::{
    project_review_pack_parity_harness, ReviewPackParityHarnessDowngradeProjection,
    ReviewPackParityHarnessError, ReviewPackParityHarnessFindingProjection,
    ReviewPackParityHarnessLaneProjection, ReviewPackParityHarnessProjection,
};

const ALPHA_PARITY_HARNESS_ROWS: &[(&str, &str)] = &[
    (
        "fixtures/review/m3/review_pack_harness/first_party_full_parity_run.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m3/review_pack_harness/first_party_full_parity_run.json"
        )),
    ),
    (
        "fixtures/review/m3/review_pack_harness/team_shared_mixed_parity_documented.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m3/review_pack_harness/team_shared_mixed_parity_documented.json"
        )),
    ),
    (
        "fixtures/review/m3/review_pack_harness/partner_signed_ci_only_documented.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m3/review_pack_harness/partner_signed_ci_only_documented.json"
        )),
    ),
    (
        "fixtures/review/m3/review_pack_harness/uncertified_community_drift_downgrade.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m3/review_pack_harness/uncertified_community_drift_downgrade.json"
        )),
    ),
];

/// Presentation label rendered for the parity-harness inspector surface.
pub const PARITY_HARNESS_INSPECTOR_PRESENTATION_LABEL: &str = "Review-pack parity-harness inspector";

/// Presentation subtitle rendered for the parity-harness inspector surface.
pub const PARITY_HARNESS_INSPECTOR_PRESENTATION_SUBTITLE: &str =
    "Compare local and CI runs of a review pack and surface drift downgrades before any green claim ships.";

/// One parity-harness row rendered in the inspector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParityHarnessInspectorRow {
    pub source_ref: &'static str,
    pub parity_harness_id: String,
    pub review_pack_ref: String,
    pub repo_anchor_ref: String,
    pub pack_authority_class: String,
    pub display_label: String,
    pub summary: String,
    pub operator_caveat: String,
    pub harness_version: u32,
    pub schema_version: u32,
    pub lane_observations: Vec<ReviewPackParityHarnessLaneProjection>,
    pub check_findings: Vec<ReviewPackParityHarnessFindingProjection>,
    pub drift_downgrades: Vec<ReviewPackParityHarnessDowngradeProjection>,
    pub overall_verdict_class: String,
    pub row_downgrade_class: String,
    pub finding_count: usize,
    pub full_parity_count: usize,
    pub local_only_match_count: usize,
    pub ci_only_match_count: usize,
    pub drift_detected_count: usize,
    pub downgrade_count: usize,
    pub consumer_surfaces: Vec<String>,
    pub support_export_refs: Vec<String>,
    pub redaction_class: String,
}

impl ParityHarnessInspectorRow {
    fn from_projection(
        source_ref: &'static str,
        projection: ReviewPackParityHarnessProjection,
    ) -> Self {
        Self {
            source_ref,
            parity_harness_id: projection.parity_harness_id,
            review_pack_ref: projection.review_pack_ref,
            repo_anchor_ref: projection.repo_anchor_ref,
            pack_authority_class: projection.pack_authority_class,
            display_label: projection.display_label,
            summary: projection.summary,
            operator_caveat: projection.operator_caveat,
            harness_version: projection.harness_version,
            schema_version: projection.schema_version,
            lane_observations: projection.lane_observations,
            check_findings: projection.check_findings,
            drift_downgrades: projection.drift_downgrades,
            overall_verdict_class: projection.overall_verdict_class,
            row_downgrade_class: projection.row_downgrade_class,
            finding_count: projection.finding_count,
            full_parity_count: projection.full_parity_count,
            local_only_match_count: projection.local_only_match_count,
            ci_only_match_count: projection.ci_only_match_count,
            drift_detected_count: projection.drift_detected_count,
            downgrade_count: projection.downgrade_count,
            consumer_surfaces: projection.consumer_surfaces,
            support_export_refs: projection.support_export_refs,
            redaction_class: projection.redaction_class,
        }
    }
}

/// Error returned when the inspector cannot project a parity-harness row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParityHarnessInspectorError {
    source_ref: &'static str,
    message: String,
}

impl ParityHarnessInspectorError {
    /// Returns the artifact path that failed to project.
    pub const fn source_ref(&self) -> &'static str {
        self.source_ref
    }

    /// Returns the parse or validation failure.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ParityHarnessInspectorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.source_ref, self.message)
    }
}

impl std::error::Error for ParityHarnessInspectorError {}

/// Builds parity-harness inspector rows from the checked-in alpha
/// parity-harness fixtures.
///
/// # Errors
///
/// Returns [`ParityHarnessInspectorError`] when any fixture fails to
/// parse or validate against the alpha contract.
pub fn build_alpha_parity_harness_rows(
) -> Result<Vec<ParityHarnessInspectorRow>, ParityHarnessInspectorError> {
    let mut rows = Vec::with_capacity(ALPHA_PARITY_HARNESS_ROWS.len());
    for (source_ref, payload) in ALPHA_PARITY_HARNESS_ROWS {
        let projection = project_review_pack_parity_harness(payload)
            .map_err(|err| projection_error(source_ref, err))?;
        rows.push(ParityHarnessInspectorRow::from_projection(source_ref, projection));
    }
    Ok(rows)
}

/// Renders the alpha parity-harness inspector projection as deterministic
/// plaintext for CLI / headless / docs / support consumers.
///
/// # Errors
///
/// Returns [`ParityHarnessInspectorError`] when a fixture cannot be
/// projected.
pub fn render_alpha_parity_harness_plaintext(
) -> Result<String, ParityHarnessInspectorError> {
    let rows = build_alpha_parity_harness_rows()?;
    let mut lines = vec![
        "Review-pack parity-harness inspector alpha".to_string(),
        "parity_harness_id | review_pack_ref | authority | verdict | row_downgrade | findings | full | local_only | ci_only | drift | downgrades"
            .to_string(),
    ];
    for row in rows {
        lines.push(format!(
            "{} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {}",
            row.parity_harness_id,
            row.review_pack_ref,
            row.pack_authority_class,
            row.overall_verdict_class,
            row.row_downgrade_class,
            row.finding_count,
            row.full_parity_count,
            row.local_only_match_count,
            row.ci_only_match_count,
            row.drift_detected_count,
            row.downgrade_count,
        ));
    }
    lines.push(String::new());
    Ok(lines.join("\n"))
}

fn projection_error(
    source_ref: &'static str,
    err: ReviewPackParityHarnessError,
) -> ParityHarnessInspectorError {
    ParityHarnessInspectorError {
        source_ref,
        message: err.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_parity_harness_rows_project() {
        let rows = build_alpha_parity_harness_rows()
            .expect("alpha parity-harness rows must project");
        assert_eq!(rows.len(), 4);
        let mut authorities: Vec<&str> = rows
            .iter()
            .map(|row| row.pack_authority_class.as_str())
            .collect();
        authorities.sort();
        authorities.dedup();
        assert!(authorities.contains(&"repo_first_party"));
        assert!(authorities.contains(&"repo_team_shared"));
        assert!(authorities.contains(&"repo_partner_signed"));
        assert!(authorities.contains(&"repo_uncertified_community"));

        let mut verdicts: Vec<&str> = rows
            .iter()
            .map(|row| row.overall_verdict_class.as_str())
            .collect();
        verdicts.sort();
        verdicts.dedup();
        assert!(verdicts.contains(&"full_parity"));
        assert!(verdicts.contains(&"drift_downgraded"));

        for row in &rows {
            assert!(
                row.consumer_surfaces
                    .iter()
                    .any(|surface| surface == "parity_harness_inspector"),
                "every row must keep the parity_harness_inspector consumer wired",
            );
            assert!(
                row.parity_harness_id
                    .starts_with("review_pack_parity_harness_alpha:"),
                "every row must mint a review_pack_parity_harness_alpha id",
            );
            assert!(
                row.review_pack_ref.starts_with("review_pack_alpha:"),
                "every row must reference a review_pack_alpha id",
            );
            assert_eq!(row.harness_version, 1);
            assert_eq!(row.schema_version, 1);
        }
    }

    #[test]
    fn alpha_parity_harness_plaintext_is_deterministic() {
        let first = render_alpha_parity_harness_plaintext().expect("plaintext renders");
        let second = render_alpha_parity_harness_plaintext().expect("plaintext renders");
        assert_eq!(first, second);
        assert!(first.contains("Review-pack parity-harness inspector alpha"));
        assert!(first.contains("repo_first_party"));
        assert!(first.contains("repo_team_shared"));
        assert!(first.contains("repo_partner_signed"));
        assert!(first.contains("repo_uncertified_community"));
        assert!(first.contains("full_parity"));
        assert!(first.contains("drift_downgraded"));
        assert!(first.contains("downgraded_to_review_required"));
    }

    #[test]
    fn drift_downgrade_row_is_never_clean() {
        let rows = build_alpha_parity_harness_rows().expect("rows must project");
        let drift_row = rows
            .iter()
            .find(|row| row.overall_verdict_class == "drift_downgraded")
            .expect("at least one row must be drift_downgraded");
        assert_ne!(drift_row.row_downgrade_class, "no_downgrade");
        assert!(drift_row.downgrade_count >= 1);
    }

    #[test]
    fn presentation_labels_are_quotable() {
        assert!(PARITY_HARNESS_INSPECTOR_PRESENTATION_LABEL
            .to_lowercase()
            .contains("parity"));
        assert!(PARITY_HARNESS_INSPECTOR_PRESENTATION_SUBTITLE
            .to_lowercase()
            .contains("drift"));
    }
}
