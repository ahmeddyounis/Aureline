//! Support-export projection for portable change bundles and shelf entries.
//!
//! The support pipeline consumes the same [`aureline_change_objects`]
//! portable-bundle projection as shell and review surfaces. This keeps offline
//! review, browser companion handoff, incident follow-up, shelf resume, and
//! support export on one vocabulary: target identity, stale-validation labels,
//! reopen posture, redaction class, and authority exclusion all travel as
//! explicit metadata rather than support-runbook prose.

use std::error::Error;
use std::fmt;

use aureline_change_objects::{
    current_portable_bundle_fixture_projections, PortableBundleError, PortableBundleProjection,
    PORTABLE_BUNDLE_DOC_REF, PORTABLE_BUNDLE_SCHEMA_REF, PORTABLE_BUNDLE_SUPPORT_ARTIFACT_REF,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for each portable-bundle support row.
pub const PORTABLE_BUNDLE_SUPPORT_ROW_RECORD_KIND: &str = "portable_bundle_support_export_row";

/// Stable record-kind tag for the support-export envelope.
pub const PORTABLE_BUNDLE_SUPPORT_ENVELOPE_RECORD_KIND: &str =
    "portable_bundle_support_export_envelope";

/// One support-export row derived from a portable bundle projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableBundleSupportExportRow {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable bundle identity.
    pub bundle_id: String,
    /// Portable object class.
    pub object_class: String,
    /// Handoff purpose class.
    pub handoff_purpose_class: String,
    /// Bundle state class.
    pub bundle_state_class: String,
    /// Opaque worktree identity.
    pub worktree_ref: String,
    /// Opaque target identity.
    pub target_ref: String,
    /// Review-pack version label.
    pub review_pack_version: String,
    /// Review-pack parity class.
    pub review_pack_parity_class: String,
    /// Number of diff refs preserved.
    pub diff_ref_count: usize,
    /// Number of evidence refs preserved.
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
    /// True when live provider authority is absent.
    pub no_live_provider_authority: bool,
    /// Redaction class.
    pub redaction_class: String,
    /// Support lineage class.
    pub support_lineage_class: String,
    /// Support-export identity ref.
    pub support_export_ref: String,
}

impl PortableBundleSupportExportRow {
    fn from_projection(projection: PortableBundleProjection) -> Self {
        Self {
            record_kind: PORTABLE_BUNDLE_SUPPORT_ROW_RECORD_KIND.to_owned(),
            bundle_id: projection.bundle_id,
            object_class: projection.object_class,
            handoff_purpose_class: projection.handoff_purpose_class,
            bundle_state_class: projection.bundle_state_class,
            worktree_ref: projection.worktree_ref,
            target_ref: projection.target_ref,
            review_pack_version: projection.review_pack_version,
            review_pack_parity_class: projection.review_pack_parity_class,
            diff_ref_count: projection.diff_ref_count,
            evidence_ref_count: projection.evidence_ref_count,
            validation_freshness_class: projection.validation_freshness_class,
            staleness_labels: projection.staleness_labels,
            compare_only_reopen_available: projection.compare_only_reopen_available,
            desktop_resume_available: projection.desktop_resume_available,
            browser_companion_read_only_available: projection.browser_companion_read_only_available,
            authority_class: projection.authority_class,
            no_live_provider_authority: projection.no_live_provider_authority,
            redaction_class: projection.redaction_class,
            support_lineage_class: projection.support_lineage_class,
            support_export_ref: projection.support_export_ref,
        }
    }
}

/// Support-export envelope for the portable bundle fixture corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableBundleSupportExportEnvelope {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable envelope identity.
    pub envelope_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// UX contract ref.
    pub doc_ref: String,
    /// Support-review artifact ref.
    pub artifact_ref: String,
    /// True when raw paths are absent from the envelope.
    pub raw_paths_excluded: bool,
    /// True when raw remote URLs are absent from the envelope.
    pub raw_remote_urls_excluded: bool,
    /// True when raw credentials are absent from the envelope.
    pub raw_credentials_excluded: bool,
    /// True when live provider authority is absent from the envelope.
    pub live_provider_authority_excluded: bool,
    /// Rows folded from the portable bundle corpus.
    pub rows: Vec<PortableBundleSupportExportRow>,
}

impl PortableBundleSupportExportEnvelope {
    /// Returns true when the envelope is metadata-safe for support export.
    pub fn is_export_safe(&self) -> bool {
        self.schema_ref == PORTABLE_BUNDLE_SCHEMA_REF
            && self.doc_ref == PORTABLE_BUNDLE_DOC_REF
            && self.artifact_ref == PORTABLE_BUNDLE_SUPPORT_ARTIFACT_REF
            && self.raw_paths_excluded
            && self.raw_remote_urls_excluded
            && self.raw_credentials_excluded
            && self.live_provider_authority_excluded
            && !self.rows.is_empty()
            && self.rows.iter().all(|row| row.no_live_provider_authority)
    }
}

/// Error returned when the portable bundle support envelope cannot compile.
#[derive(Debug)]
pub struct PortableBundleSupportExportError {
    source: PortableBundleError,
}

impl fmt::Display for PortableBundleSupportExportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "portable bundle support export projection failed: {}",
            self.source
        )
    }
}

impl Error for PortableBundleSupportExportError {}

impl From<PortableBundleError> for PortableBundleSupportExportError {
    fn from(source: PortableBundleError) -> Self {
        Self { source }
    }
}

/// Compiles the portable bundle support-export envelope.
///
/// # Errors
///
/// Returns [`PortableBundleSupportExportError`] when any checked-in fixture
/// fails to parse or validate.
pub fn compile_portable_bundle_support_export_envelope(
    envelope_id: impl Into<String>,
    captured_at: impl Into<String>,
) -> Result<PortableBundleSupportExportEnvelope, PortableBundleSupportExportError> {
    let mut rows: Vec<_> = current_portable_bundle_fixture_projections()?
        .into_iter()
        .map(PortableBundleSupportExportRow::from_projection)
        .collect();
    rows.sort_by(|left, right| left.bundle_id.cmp(&right.bundle_id));
    Ok(PortableBundleSupportExportEnvelope {
        record_kind: PORTABLE_BUNDLE_SUPPORT_ENVELOPE_RECORD_KIND.to_owned(),
        envelope_id: envelope_id.into(),
        captured_at: captured_at.into(),
        schema_ref: PORTABLE_BUNDLE_SCHEMA_REF.to_owned(),
        doc_ref: PORTABLE_BUNDLE_DOC_REF.to_owned(),
        artifact_ref: PORTABLE_BUNDLE_SUPPORT_ARTIFACT_REF.to_owned(),
        raw_paths_excluded: true,
        raw_remote_urls_excluded: true,
        raw_credentials_excluded: true,
        live_provider_authority_excluded: true,
        rows,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn support_export_envelope_is_metadata_safe() {
        let envelope = compile_portable_bundle_support_export_envelope(
            "support_export:portable_bundle:test",
            "2026-05-18T17:00:00Z",
        )
        .expect("envelope compiles");
        assert!(envelope.is_export_safe());
        assert!(envelope.rows.iter().any(|row| row.desktop_resume_available));
        assert!(envelope
            .rows
            .iter()
            .any(|row| row.compare_only_reopen_available));
    }
}
