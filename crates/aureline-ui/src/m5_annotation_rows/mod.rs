//! Shared annotation-row contract for build, test, security, and provider
//! findings.
//!
//! An annotation row is the cross-surface object that connects an external
//! finding back to a durable file, symbol, manifest, package, run, change, or
//! artifact anchor. Review panes, project-health surfaces, companion clients,
//! support exports, and release proof ingest this object instead of re-deriving
//! provider/scanner labels or retargeting stale anchors locally.
//!
//! The contract intentionally keeps producer, anchor, severity, confidence,
//! freshness, stale handoff, suppression, remediation, and open-details action
//! in separate fields. A stale, superseded, or partially mapped finding must stay
//! visible with its original anchor and handoff state; it may not silently move to
//! the currently visible file revision or run.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`AnnotationRow`].
pub const M5_ANNOTATION_ROW_RECORD_KIND: &str = "m5_annotation_row";

/// Schema version for the annotation row.
pub const M5_ANNOTATION_ROW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_ANNOTATION_ROW_SCHEMA_REF: &str = "schemas/ui/m5-annotation-row.schema.json";

/// Repo-relative path of the canonical first-consumer fixture.
pub const M5_ANNOTATION_ROW_FIXTURE_REF: &str =
    "fixtures/ui/m5-pipeline-dependency-finding-components/annotation_row.json";

/// Reusable annotation row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnnotationRow {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable row id reused by all consumers.
    pub row_id: String,
    /// Canonical annotation id.
    pub annotation_id: String,
    /// Canonical normalized diagnostic id.
    pub diagnostic_id: String,
    /// Provider/scanner provenance disclosed to users and exports.
    pub source_provider: SourceProvider,
    /// Durable top-level anchor ref.
    pub anchor_ref: String,
    /// Typed anchor details.
    pub anchor: AnnotationAnchor,
    /// Opaque source packet ref.
    pub source_packet_ref: String,
    /// Annotation family.
    pub annotation_kind: String,
    /// User-visible severity.
    pub severity: String,
    /// User-visible confidence.
    pub confidence: String,
    /// User-visible freshness.
    pub freshness_state: String,
    /// Narrowed/degraded state.
    pub degraded_state: String,
    /// Stale, superseded, or partial-anchor handoff state.
    pub stale_handoff: StaleHandoff,
    /// Suppression state that remains visible.
    pub suppression_state: SuppressionState,
    /// Remediation state.
    pub remediation: Remediation,
    /// In-product open-details action.
    pub open_details_action: OpenDetailsAction,
    /// Source documents/schemas.
    pub source_refs: Vec<String>,
    /// Consumers that must render this same row contract.
    pub consumer_surfaces: Vec<String>,
    /// Copy/export payloads.
    pub copy_export: CopyExport,
}

/// Provider/scanner provenance for an annotation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceProvider {
    /// Controlled provider label.
    pub provider_label: String,
    /// Controlled provider kind.
    pub provider_kind: String,
    /// Scanner/tool label.
    pub scanner_label: String,
    /// Opaque run ref that produced the finding.
    pub source_run_ref: String,
    /// Revision observed by the producer.
    pub source_revision_ref: String,
    /// Redacted provider-payload ref, not the raw payload.
    pub provider_payload_ref: String,
    /// Must remain false; raw provider dumps are not export-safe.
    pub raw_provider_dump_included: bool,
}

/// Typed anchor details for an annotation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnnotationAnchor {
    /// File, symbol, manifest, package, run, change, artifact, or unresolved.
    pub anchor_kind: String,
    /// Durable anchor ref.
    pub anchor_ref: String,
    /// Opaque file ref or `file:none`.
    pub file_ref: String,
    /// Opaque symbol ref or `symbol:none`.
    pub symbol_ref: String,
    /// Opaque manifest ref or `manifest:none`.
    pub manifest_ref: String,
    /// Revision the anchor was mapped against.
    pub revision_ref: String,
    /// Line-range ref or `line_range:none`.
    pub line_range_ref: String,
    /// Whether this is a partial anchor that must be labeled as such.
    pub partial_anchor: bool,
}

/// Handoff state when a row can no longer be treated as current.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaleHandoff {
    /// Current, stale, superseded, partial, or unverified.
    pub state: String,
    /// Reason the row did not silently retarget.
    pub reason: String,
    /// Anchor used when the finding was produced.
    pub previous_anchor_ref: String,
    /// Successor candidate, if any, that needs review.
    pub successor_anchor_ref: String,
    /// Whether human/provider review is required before retargeting.
    pub review_required: bool,
    /// Must remain true for stale/superseded/partial handoffs.
    pub silent_retarget_prohibited: bool,
}

/// Suppression state that remains visible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuppressionState {
    /// Controlled suppression state.
    pub state: String,
    /// Reason ref.
    pub reason_ref: String,
    /// Review ref.
    pub review_ref: String,
    /// Must remain true so support exports do not lose suppression truth.
    pub visible_in_export: bool,
}

/// Remediation state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Remediation {
    /// Inspect, apply_fix, open_review, suppress_until_review,
    /// policy_exception_review, or no_fix_yet.
    pub action_label: String,
    /// Whether an automated or guided fix exists.
    pub fix_available: bool,
    /// Owner ref.
    pub owner_ref: String,
    /// Due or review ref.
    pub due_or_review_ref: String,
    /// Blocked reason, or `none`.
    pub blocked_reason: String,
}

/// Open-details action for the row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenDetailsAction {
    /// Stable action id.
    pub action_id: String,
    /// Controlled label, currently `open_details`.
    pub label: String,
    /// Details target ref.
    pub target_ref: String,
    /// Whether this action leaves the local support-safe detail packet.
    pub requires_provider_handoff: bool,
    /// Whether the details action is currently enabled.
    pub enabled: bool,
    /// Disabled reason, or `none`.
    pub disabled_reason: String,
}

/// Export-safe text, JSON, and Markdown copies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyExport {
    /// Available formats.
    pub formats: Vec<String>,
    /// Fields intentionally preserved in export.
    pub export_fields: Vec<String>,
    /// Plain text copy.
    pub text: String,
    /// JSON copy.
    pub json: String,
    /// Markdown copy.
    pub markdown: String,
    /// Must remain true; screenshot-only reconstruction is forbidden.
    pub screenshot_only_prohibited: bool,
}

/// Minimal projection consumed by review, health, companion, and support
/// surfaces. Every projection is derived from the same [`AnnotationRow`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotationSurfaceProjection {
    /// Consumer surface name.
    pub consumer_surface: String,
    /// Stable row id.
    pub row_id: String,
    /// Provider label.
    pub provider_label: String,
    /// Scanner label.
    pub scanner_label: String,
    /// Anchor kind.
    pub anchor_kind: String,
    /// Anchor ref.
    pub anchor_ref: String,
    /// Severity.
    pub severity: String,
    /// Confidence.
    pub confidence: String,
    /// Freshness state.
    pub freshness_state: String,
    /// Stale handoff state.
    pub stale_handoff_state: String,
    /// Stale handoff reason.
    pub stale_handoff_reason: String,
    /// Suppression state.
    pub suppression_state: String,
    /// Open-details action id.
    pub open_details_action_id: String,
}

impl AnnotationRow {
    /// Returns all validation violations for this row.
    #[must_use]
    pub fn validate(&self) -> Vec<AnnotationRowViolation> {
        use AnnotationRowViolation as V;

        let mut violations = Vec::new();
        if self.record_kind != M5_ANNOTATION_ROW_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_ANNOTATION_ROW_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if any_blank([
            &self.row_id,
            &self.annotation_id,
            &self.diagnostic_id,
            &self.anchor_ref,
            &self.source_packet_ref,
        ]) {
            violations.push(V::MissingIdentity);
        }
        if self.source_refs.is_empty() || self.consumer_surfaces.is_empty() {
            violations.push(V::MissingSourceOrConsumer);
        }
        if self.source_provider.raw_provider_dump_included
            || any_blank([
                &self.source_provider.provider_label,
                &self.source_provider.provider_kind,
                &self.source_provider.scanner_label,
                &self.source_provider.source_run_ref,
                &self.source_provider.source_revision_ref,
                &self.source_provider.provider_payload_ref,
            ])
        {
            violations.push(V::MissingProviderDisclosure);
        }
        if self.anchor.anchor_ref != self.anchor_ref
            || any_blank([
                &self.anchor.anchor_kind,
                &self.anchor.anchor_ref,
                &self.anchor.revision_ref,
            ])
        {
            violations.push(V::AnchorMismatch);
        }
        if self.anchor.partial_anchor && self.stale_handoff.state != "partial" {
            violations.push(V::PartialAnchorNotHandoff);
        }
        if self.requires_stale_handoff()
            && (self.stale_handoff.reason == "not_applicable"
                || !self.stale_handoff.review_required
                || !self.stale_handoff.silent_retarget_prohibited
                || self.stale_handoff.previous_anchor_ref != self.anchor_ref)
        {
            violations.push(V::StaleHandoffIncomplete);
        }
        if !self.suppression_state.visible_in_export {
            violations.push(V::SuppressionHiddenFromExport);
        }
        if self.open_details_action.label != "open_details"
            || !self.open_details_action.enabled
            || any_blank([
                &self.open_details_action.action_id,
                &self.open_details_action.target_ref,
            ])
        {
            violations.push(V::OpenDetailsActionMissing);
        }
        if !has_all(
            &self.consumer_surfaces,
            [
                "code_surface",
                "review_pane",
                "project_health_center",
                "support_export",
            ],
        ) {
            violations.push(V::CoreConsumerSurfaceMissing);
        }
        if !has_all(&self.copy_export.formats, ["text", "json", "markdown"])
            || !self.copy_export.screenshot_only_prohibited
        {
            violations.push(V::CopyExportIncomplete);
        }
        if !has_all(
            &self.copy_export.export_fields,
            [
                "source_provider",
                "anchor",
                "stale_handoff",
                "open_details_action",
            ],
        ) {
            violations.push(V::CopyExportDropsAnchorTruth);
        }
        let copy_blob = format!(
            "{}\n{}\n{}",
            self.copy_export.text, self.copy_export.json, self.copy_export.markdown
        );
        for required in [
            &self.source_provider.provider_label,
            &self.source_provider.scanner_label,
            &self.anchor.anchor_kind,
            &self.anchor_ref,
            &self.stale_handoff.state,
            &self.stale_handoff.reason,
        ] {
            if !copy_blob.contains(required) {
                violations.push(V::CopyExportDropsAnchorTruth);
                break;
            }
        }
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("annotation row serializes"),
        ) {
            violations.push(V::RawProviderMaterialInExport);
        }

        violations
    }

    /// Whether this row must preserve stale/superseded/partial handoff details.
    #[must_use]
    pub fn requires_stale_handoff(&self) -> bool {
        matches!(
            self.freshness_state.as_str(),
            "stale" | "superseded" | "partial"
        ) || matches!(
            self.degraded_state.as_str(),
            "stale" | "superseded" | "partial"
        )
    }

    /// Projects this row to a named consumer surface using the same field values.
    #[must_use]
    pub fn projection_for(&self, consumer_surface: &str) -> Option<AnnotationSurfaceProjection> {
        if !self
            .consumer_surfaces
            .iter()
            .any(|surface| surface == consumer_surface)
        {
            return None;
        }
        Some(AnnotationSurfaceProjection {
            consumer_surface: consumer_surface.to_owned(),
            row_id: self.row_id.clone(),
            provider_label: self.source_provider.provider_label.clone(),
            scanner_label: self.source_provider.scanner_label.clone(),
            anchor_kind: self.anchor.anchor_kind.clone(),
            anchor_ref: self.anchor_ref.clone(),
            severity: self.severity.clone(),
            confidence: self.confidence.clone(),
            freshness_state: self.freshness_state.clone(),
            stale_handoff_state: self.stale_handoff.state.clone(),
            stale_handoff_reason: self.stale_handoff.reason.clone(),
            suppression_state: self.suppression_state.state.clone(),
            open_details_action_id: self.open_details_action.action_id.clone(),
        })
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only row fails.
    #[must_use]
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("annotation row serializes")
    }
}

/// Error returned when the checked-in annotation fixture fails to load.
#[derive(Debug)]
pub enum AnnotationRowArtifactError {
    /// The fixture could not be parsed.
    Fixture(serde_json::Error),
    /// The parsed row failed validation.
    Validation(Vec<AnnotationRowViolation>),
}

impl fmt::Display for AnnotationRowArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fixture(err) => write!(f, "annotation fixture parse error: {err}"),
            Self::Validation(violations) => {
                write!(f, "annotation fixture failed validation: {violations:?}")
            }
        }
    }
}

impl Error for AnnotationRowArtifactError {}

/// A validation invariant an annotation row can violate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnotationRowViolation {
    /// The record-kind tag is wrong.
    WrongRecordKind,
    /// The schema version is wrong.
    WrongSchemaVersion,
    /// Row identity is incomplete.
    MissingIdentity,
    /// Source refs or consumer surfaces are missing.
    MissingSourceOrConsumer,
    /// Provider/scanner disclosure is incomplete or includes raw provider dumps.
    MissingProviderDisclosure,
    /// The top-level anchor and typed anchor disagree.
    AnchorMismatch,
    /// A partial anchor is not reflected in the handoff state.
    PartialAnchorNotHandoff,
    /// Stale/superseded/partial state did not preserve handoff truth.
    StaleHandoffIncomplete,
    /// Suppression is hidden from export.
    SuppressionHiddenFromExport,
    /// Open-details action is missing or inert.
    OpenDetailsActionMissing,
    /// Code, review, project-health, or support export consumer is missing.
    CoreConsumerSurfaceMissing,
    /// Copy/export formats are incomplete.
    CopyExportIncomplete,
    /// Copy/export drops provenance, anchor, stale handoff, or details action.
    CopyExportDropsAnchorTruth,
    /// Raw provider/body/credential material crossed the export boundary.
    RawProviderMaterialInExport,
}

/// Loads and validates the checked-in canonical annotation fixture.
///
/// # Errors
///
/// Returns an error if the checked-in fixture cannot be parsed or fails
/// validation.
pub fn current_m5_annotation_row() -> Result<AnnotationRow, AnnotationRowArtifactError> {
    let row: AnnotationRow = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-pipeline-dependency-finding-components/annotation_row.json"
    )))
    .map_err(AnnotationRowArtifactError::Fixture)?;
    let violations = row.validate();
    if violations.is_empty() {
        Ok(row)
    } else {
        Err(AnnotationRowArtifactError::Validation(violations))
    }
}

fn any_blank<'a>(values: impl IntoIterator<Item = &'a String>) -> bool {
    values.into_iter().any(|value| value.trim().is_empty())
}

fn has_all<'a>(actual: &[String], required: impl IntoIterator<Item = &'a str>) -> bool {
    let actual: BTreeSet<&str> = actual.iter().map(String::as_str).collect();
    required.into_iter().all(|value| actual.contains(value))
}

fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("raw provider")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
