//! Shell review-pack inspector — alpha consumer of the review-pack DSL
//! family.
//!
//! This module reads the checked-in alpha review-pack DSL fixtures and
//! projects them into the deterministic review-pack rows the inspector
//! renders ahead of any local or CI harness run. Every row exposes the
//! five review questions the inspector exists to answer:
//!
//! 1. Which pack is this, and where does it live in the repo?
//! 2. Who authored or signed it (first-party, team-shared,
//!    partner-signed, uncertified-community)?
//! 3. What checks would run, with what severity, parity, and execution
//!    class?
//! 4. Who owns the affected scopes?
//! 5. What is local-only, CI-only, or unsupported on the current DSL
//!    version?
//!
//! The shared data types are defined in
//! [`aureline_review::review_pack_dsl`] so review and support packets
//! re-use the same review-pack vocabulary without forking it per
//! surface.

use std::fmt;

use aureline_review::{
    project_review_pack, ReviewPackError, ReviewPackOwnershipProjection,
    ReviewPackParityObservation, ReviewPackProjection, ReviewPackUnsupportedField,
};

const ALPHA_REVIEW_PACK_ROWS: &[(&str, &str)] = &[
    (
        "fixtures/review/m3/review_pack_dsl/first_party_local_and_ci_parity.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m3/review_pack_dsl/first_party_local_and_ci_parity.json"
        )),
    ),
    (
        "fixtures/review/m3/review_pack_dsl/team_shared_mixed_parity.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m3/review_pack_dsl/team_shared_mixed_parity.json"
        )),
    ),
    (
        "fixtures/review/m3/review_pack_dsl/partner_signed_ci_only_lane.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m3/review_pack_dsl/partner_signed_ci_only_lane.json"
        )),
    ),
    (
        "fixtures/review/m3/review_pack_dsl/uncertified_community_local_only_lane.json",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m3/review_pack_dsl/uncertified_community_local_only_lane.json"
        )),
    ),
];

/// Presentation label rendered for the review-pack inspector surface.
pub const REVIEW_PACK_INSPECTOR_PRESENTATION_LABEL: &str = "Review-pack inspector";

/// Presentation subtitle rendered for the review-pack inspector surface.
pub const REVIEW_PACK_INSPECTOR_PRESENTATION_SUBTITLE: &str =
    "Inspect repo-defined review packs and their local/CI parity before running the harness.";

/// One review-pack row rendered in the inspector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewPackInspectorRow {
    pub source_ref: &'static str,
    pub review_pack_id: String,
    pub repo_anchor_ref: String,
    pub display_label: String,
    pub summary: String,
    pub pack_authority_class: String,
    pub operator_caveat: String,
    pub dsl_version: u32,
    pub schema_version: u32,
    pub check_count: usize,
    pub blocking_check_count: usize,
    pub local_only_check_count: usize,
    pub ci_only_check_count: usize,
    pub local_ci_parity_classes: Vec<String>,
    pub ownership_hints: Vec<ReviewPackOwnershipProjection>,
    pub parity_observations: Vec<ReviewPackParityObservation>,
    pub unsupported_fields: Vec<ReviewPackUnsupportedField>,
    pub consumer_surfaces: Vec<String>,
    pub support_export_refs: Vec<String>,
    pub redaction_class: String,
}

impl ReviewPackInspectorRow {
    fn from_projection(source_ref: &'static str, projection: ReviewPackProjection) -> Self {
        let check_count = projection.checks.len();
        Self {
            source_ref,
            review_pack_id: projection.review_pack_id,
            repo_anchor_ref: projection.repo_anchor_ref,
            display_label: projection.display_label,
            summary: projection.summary,
            pack_authority_class: projection.pack_authority_class,
            operator_caveat: projection.operator_caveat,
            dsl_version: projection.dsl_version,
            schema_version: projection.schema_version,
            check_count,
            blocking_check_count: projection.blocking_check_count,
            local_only_check_count: projection.local_only_check_count,
            ci_only_check_count: projection.ci_only_check_count,
            local_ci_parity_classes: projection.local_ci_parity_classes,
            ownership_hints: projection.ownership_hints,
            parity_observations: projection.parity_observations,
            unsupported_fields: projection.unsupported_fields,
            consumer_surfaces: projection.consumer_surfaces,
            support_export_refs: projection.support_export_refs,
            redaction_class: projection.redaction_class,
        }
    }
}

/// Error returned when the inspector cannot project a review-pack row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewPackInspectorError {
    source_ref: &'static str,
    message: String,
}

impl ReviewPackInspectorError {
    /// Returns the artifact path that failed to project.
    pub const fn source_ref(&self) -> &'static str {
        self.source_ref
    }

    /// Returns the parse or validation failure.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ReviewPackInspectorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.source_ref, self.message)
    }
}

impl std::error::Error for ReviewPackInspectorError {}

/// Builds review-pack inspector rows from the checked-in alpha
/// review-pack DSL fixtures.
///
/// # Errors
///
/// Returns [`ReviewPackInspectorError`] when any fixture fails to parse
/// or validate against the alpha contract.
pub fn build_alpha_review_pack_rows(
) -> Result<Vec<ReviewPackInspectorRow>, ReviewPackInspectorError> {
    let mut rows = Vec::with_capacity(ALPHA_REVIEW_PACK_ROWS.len());
    for (source_ref, payload) in ALPHA_REVIEW_PACK_ROWS {
        let projection =
            project_review_pack(payload).map_err(|err| projection_error(source_ref, err))?;
        rows.push(ReviewPackInspectorRow::from_projection(
            source_ref, projection,
        ));
    }
    Ok(rows)
}

/// Renders the alpha review-pack inspector projection as deterministic
/// plaintext for CLI / headless / docs / support consumers.
///
/// # Errors
///
/// Returns [`ReviewPackInspectorError`] when a fixture cannot be
/// projected.
pub fn render_alpha_review_pack_plaintext() -> Result<String, ReviewPackInspectorError> {
    let rows = build_alpha_review_pack_rows()?;
    let mut lines = vec![
        "Review-pack DSL inspector alpha".to_string(),
        "review_pack_id | authority | dsl_version | check_count | blocking | local_only | ci_only | parity_classes | unsupported_fields"
            .to_string(),
    ];
    for row in rows {
        let parity_classes = if row.local_ci_parity_classes.is_empty() {
            "none".to_string()
        } else {
            row.local_ci_parity_classes.join(",")
        };
        let unsupported = if row.unsupported_fields.is_empty() {
            "none".to_string()
        } else {
            row.unsupported_fields
                .iter()
                .map(|f| f.unsupported_class.as_str())
                .collect::<Vec<_>>()
                .join(",")
        };
        lines.push(format!(
            "{} | {} | {} | {} | {} | {} | {} | {} | {}",
            row.review_pack_id,
            row.pack_authority_class,
            row.dsl_version,
            row.check_count,
            row.blocking_check_count,
            row.local_only_check_count,
            row.ci_only_check_count,
            parity_classes,
            unsupported,
        ));
    }
    lines.push(String::new());
    Ok(lines.join("\n"))
}

fn projection_error(source_ref: &'static str, err: ReviewPackError) -> ReviewPackInspectorError {
    ReviewPackInspectorError {
        source_ref,
        message: err.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_review_pack_rows_project() {
        let rows = build_alpha_review_pack_rows().expect("alpha review-pack rows must project");
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

        for row in &rows {
            assert!(
                row.consumer_surfaces
                    .iter()
                    .any(|surface| surface == "review_pack_inspector"),
                "every row must keep the review_pack_inspector consumer wired",
            );
            assert!(
                row.review_pack_id.starts_with("review_pack_alpha:"),
                "every row must mint a review_pack_alpha id",
            );
            assert_eq!(row.dsl_version, 1);
            assert_eq!(row.schema_version, 1);
        }
    }

    #[test]
    fn alpha_review_pack_plaintext_is_deterministic() {
        let first = render_alpha_review_pack_plaintext().expect("plaintext renders");
        let second = render_alpha_review_pack_plaintext().expect("plaintext renders");
        assert_eq!(first, second);
        assert!(first.contains("Review-pack DSL inspector alpha"));
        assert!(first.contains("repo_first_party"));
        assert!(first.contains("repo_team_shared"));
        assert!(first.contains("repo_partner_signed"));
        assert!(first.contains("repo_uncertified_community"));
        assert!(first.contains("local_and_ci_parity"));
        assert!(first.contains("local_only_documented"));
        assert!(first.contains("ci_only_documented"));
        assert!(first.contains("parity_unknown_requires_review"));
    }

    #[test]
    fn presentation_labels_are_quotable() {
        assert!(REVIEW_PACK_INSPECTOR_PRESENTATION_LABEL
            .to_lowercase()
            .contains("review-pack"));
        assert!(REVIEW_PACK_INSPECTOR_PRESENTATION_SUBTITLE
            .to_lowercase()
            .contains("local/ci"));
    }
}
