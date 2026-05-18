//! Shell portable-bundle inspector.
//!
//! This module renders deterministic rows for portable change bundles and
//! shelf entries. It uses the shared [`aureline_change_objects`] projection so
//! shell UI, CLI/headless output, browser companion handoff, incident packets,
//! and support export all agree on target identity, stale-validation labels,
//! compare-only reopen, desktop resume, redaction posture, and excluded live
//! authority.

use std::fmt;

use aureline_change_objects::{
    current_portable_bundle_fixture_projections, PortableBundleError, PortableBundleProjection,
};

/// Presentation label rendered for the portable-bundle inspector.
pub const PORTABLE_BUNDLE_INSPECTOR_PRESENTATION_LABEL: &str = "Portable bundle inspector";

/// Presentation subtitle rendered for the portable-bundle inspector.
pub const PORTABLE_BUNDLE_INSPECTOR_PRESENTATION_SUBTITLE: &str =
    "Inspect portable bundles and shelves before offline review, support export, or desktop resume.";

/// One row rendered in the portable-bundle inspector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortableBundleInspectorRow {
    /// Stable bundle identity.
    pub bundle_id: String,
    /// Portable object class.
    pub object_class: String,
    /// Handoff purpose class.
    pub handoff_purpose_class: String,
    /// Bundle state class.
    pub bundle_state_class: String,
    /// Reviewable display label.
    pub display_label: String,
    /// Opaque target identity.
    pub target_ref: String,
    /// Opaque worktree identity.
    pub worktree_ref: String,
    /// Review-pack version label.
    pub review_pack_version: String,
    /// Count of diff refs.
    pub diff_ref_count: usize,
    /// Count of evidence refs.
    pub evidence_ref_count: usize,
    /// Validation freshness class.
    pub validation_freshness_class: String,
    /// Stale-validation labels.
    pub staleness_labels: Vec<String>,
    /// True when compare-only reopen is available.
    pub compare_only_reopen_available: bool,
    /// True when desktop resume is available after revalidation.
    pub desktop_resume_available: bool,
    /// True when browser companion read-only inspection is available.
    pub browser_companion_read_only_available: bool,
    /// Authority class.
    pub authority_class: String,
    /// Redaction class.
    pub redaction_class: String,
    /// Support-export ref.
    pub support_export_ref: String,
}

impl PortableBundleInspectorRow {
    fn from_projection(projection: PortableBundleProjection) -> Self {
        Self {
            bundle_id: projection.bundle_id,
            object_class: projection.object_class,
            handoff_purpose_class: projection.handoff_purpose_class,
            bundle_state_class: projection.bundle_state_class,
            display_label: projection.display_label,
            target_ref: projection.target_ref,
            worktree_ref: projection.worktree_ref,
            review_pack_version: projection.review_pack_version,
            diff_ref_count: projection.diff_ref_count,
            evidence_ref_count: projection.evidence_ref_count,
            validation_freshness_class: projection.validation_freshness_class,
            staleness_labels: projection.staleness_labels,
            compare_only_reopen_available: projection.compare_only_reopen_available,
            desktop_resume_available: projection.desktop_resume_available,
            browser_companion_read_only_available: projection.browser_companion_read_only_available,
            authority_class: projection.authority_class,
            redaction_class: projection.redaction_class,
            support_export_ref: projection.support_export_ref,
        }
    }
}

/// Error returned when the inspector cannot project portable bundle rows.
#[derive(Debug)]
pub struct PortableBundleInspectorError {
    message: String,
}

impl PortableBundleInspectorError {
    /// Returns the parse or validation failure.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for PortableBundleInspectorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "portable bundle inspector error: {}",
            self.message
        )
    }
}

impl std::error::Error for PortableBundleInspectorError {}

/// Builds portable-bundle inspector rows from the checked-in fixtures.
///
/// # Errors
///
/// Returns [`PortableBundleInspectorError`] when any fixture fails to parse or
/// validate.
pub fn build_portable_bundle_rows(
) -> Result<Vec<PortableBundleInspectorRow>, PortableBundleInspectorError> {
    let mut rows: Vec<_> = current_portable_bundle_fixture_projections()
        .map_err(portable_bundle_error)?
        .into_iter()
        .map(PortableBundleInspectorRow::from_projection)
        .collect();
    rows.sort_by(|left, right| left.bundle_id.cmp(&right.bundle_id));
    Ok(rows)
}

/// Renders portable-bundle rows as deterministic plaintext for CLI/headless,
/// docs, and support consumers.
///
/// # Errors
///
/// Returns [`PortableBundleInspectorError`] when any fixture fails to project.
pub fn render_portable_bundle_plaintext() -> Result<String, PortableBundleInspectorError> {
    let rows = build_portable_bundle_rows()?;
    let mut lines = vec![
        "Portable bundle inspector beta".to_string(),
        "bundle_id | object | purpose | state | target_ref | validation | stale_labels | compare_only_reopen | desktop_resume_after_revalidation | browser_companion_read_only | authority | redaction"
            .to_string(),
    ];
    for row in rows {
        let stale_labels = if row.staleness_labels.is_empty() {
            "none".to_string()
        } else {
            row.staleness_labels.join(",")
        };
        lines.push(format!(
            "{} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {}",
            row.bundle_id,
            row.object_class,
            row.handoff_purpose_class,
            row.bundle_state_class,
            row.target_ref,
            row.validation_freshness_class,
            stale_labels,
            row.compare_only_reopen_available,
            row.desktop_resume_available,
            row.browser_companion_read_only_available,
            row.authority_class,
            row.redaction_class,
        ));
    }
    lines.push(String::new());
    Ok(lines.join("\n"))
}

fn portable_bundle_error(error: PortableBundleError) -> PortableBundleInspectorError {
    PortableBundleInspectorError {
        message: error.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portable_bundle_rows_project() {
        let rows = build_portable_bundle_rows().expect("portable bundle rows project");
        assert_eq!(rows.len(), 4);
        assert!(
            rows.iter().any(|row| row.compare_only_reopen_available),
            "compare-only reopen must be represented"
        );
        assert!(
            rows.iter().any(|row| row.desktop_resume_available),
            "desktop resume must be represented"
        );
        assert!(
            rows.iter()
                .any(|row| row.browser_companion_read_only_available),
            "browser companion handoff must be represented"
        );
    }

    #[test]
    fn plaintext_is_deterministic() {
        let first = render_portable_bundle_plaintext().expect("plaintext renders");
        let second = render_portable_bundle_plaintext().expect("plaintext renders");
        assert_eq!(first, second);
        assert!(first.contains("Portable bundle inspector beta"));
        assert!(first.contains("compare_only_reopen"));
        assert!(first.contains("desktop_reauth_required"));
        assert!(first.contains("provider_overlay_unavailable"));
    }
}
