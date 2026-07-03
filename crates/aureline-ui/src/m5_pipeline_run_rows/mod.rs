//! Shared pipeline-run row contract for review, pipeline, health, companion,
//! support, and release surfaces.
//!
//! A pipeline-run row is the cross-surface object that tells users which
//! provider run is in play, what triggered it, which branch/change it belongs
//! to, how many artifacts are available, how fresh the imported provider truth
//! is, and which rerun/cancel controls are actually authorized. Review panes,
//! pipeline viewers, project-health centers, companion clients, support
//! exports, and release proof ingest this object instead of translating
//! provider-specific status text locally.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`PipelineRunRow`].
pub const M5_PIPELINE_RUN_ROW_RECORD_KIND: &str = "m5_pipeline_run_row";

/// Schema version for the pipeline-run row.
pub const M5_PIPELINE_RUN_ROW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_PIPELINE_RUN_ROW_SCHEMA_REF: &str = "schemas/ui/m5-pipeline-run-row.schema.json";

/// Repo-relative path of the canonical first-consumer fixture.
pub const M5_PIPELINE_RUN_ROW_FIXTURE_REF: &str =
    "fixtures/ui/m5-pipeline-dependency-finding-components/pipeline_run_row.json";

/// Reusable pipeline-run row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PipelineRunRow {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable row id reused by all consumers.
    pub row_id: String,
    /// Canonical normalized pipeline run id.
    pub pipeline_run_id: String,
    /// Opaque provider run ref.
    pub provider_run_ref: String,
    /// Controlled provider label.
    pub provider_label: String,
    /// Workflow or job name.
    pub workflow_or_job_name: String,
    /// Durable review/change anchor attached to the run.
    pub review_anchor_ref: String,
    /// Trigger identity and actor class.
    pub trigger: PipelineTrigger,
    /// Duration disclosure.
    pub duration: PipelineDuration,
    /// Branch, commit, and change relation.
    pub branch_change_relation: BranchChangeRelation,
    /// Normalized status.
    pub normalized_status: String,
    /// Artifact/log summary.
    pub artifact_summary: ArtifactSummary,
    /// Freshness state.
    pub freshness_state: String,
    /// Freshness note required for stale/partial/superseded rows.
    pub freshness_note: Option<String>,
    /// Narrowed/degraded state.
    pub degraded_state: String,
    /// Rerun/cancel authority.
    pub run_control_authority: RunControlAuthority,
    /// In-product open-details action.
    pub open_details_action: OpenDetailsAction,
    /// Provider handoff bar.
    pub provider_handoff: ProviderHandoff,
    /// Source documents/schemas.
    pub source_refs: Vec<String>,
    /// Consumers that must render this same row contract.
    pub consumer_surfaces: Vec<String>,
    /// Copy/export payloads.
    pub copy_export: CopyExport,
}

/// Trigger identity and actor class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PipelineTrigger {
    /// Trigger type.
    pub trigger_type: String,
    /// Opaque actor ref.
    pub actor_ref: String,
    /// Controlled actor class.
    pub actor_class: String,
    /// Trigger event ref.
    pub event_ref: String,
    /// Trigger timestamp or timestamp ref.
    pub triggered_at: String,
}

/// Duration disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PipelineDuration {
    /// Exact, approximate, or unknown.
    pub state: String,
    /// Milliseconds when known.
    pub milliseconds: Option<u64>,
    /// Display label.
    pub display_label: String,
}

/// Branch, commit, and change relation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchChangeRelation {
    /// Branch ref, when available.
    pub branch_ref: Option<String>,
    /// Commit ref, when available.
    pub commit_ref: Option<String>,
    /// Change object ref, when available.
    pub change_object_ref: Option<String>,
    /// Base/change relation.
    pub base_relation: String,
    /// Whether the base is stale.
    pub stale_base: bool,
}

/// Artifact/log summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactSummary {
    /// Number of artifacts.
    pub artifact_count: u32,
    /// Number of logs.
    pub log_count: u32,
    /// Number of unavailable artifacts/logs.
    pub unavailable_count: u32,
    /// Artifact browser ref.
    pub artifact_browser_ref: String,
    /// Retention label.
    pub retention_label: String,
}

/// Rerun/cancel authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunControlAuthority {
    /// Whether rerun is available on this surface.
    pub rerun_available: bool,
    /// Whether cancel is available on this surface.
    pub cancel_available: bool,
    /// Controlled authority label.
    pub authority_label: String,
    /// Acting identity ref.
    pub acting_identity_ref: String,
    /// Whether side-effect review is required.
    pub side_effect_review_required: bool,
    /// Disabled reason or `not_applicable`.
    pub disabled_reason: String,
}

/// Open-details action for the run row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenDetailsAction {
    /// Stable action ref.
    pub action_ref: String,
    /// Action label.
    pub action_label: String,
    /// Details target ref.
    pub details_target_ref: String,
}

/// Provider-native handoff disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderHandoff {
    /// Whether provider-native context is required.
    pub provider_native_required: bool,
    /// Handoff target ref.
    pub handoff_target_ref: String,
    /// Handoff reason.
    pub handoff_reason: String,
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

/// Minimal projection consumed by review, health, companion, support, and
/// release surfaces. Every projection is derived from one [`PipelineRunRow`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineRunSurfaceProjection {
    /// Consumer surface name.
    pub consumer_surface: String,
    /// Stable row id.
    pub row_id: String,
    /// Pipeline run id.
    pub pipeline_run_id: String,
    /// Provider label.
    pub provider_label: String,
    /// Workflow or job name.
    pub workflow_or_job_name: String,
    /// Trigger actor class.
    pub trigger_actor_class: String,
    /// Branch/change relation.
    pub base_relation: String,
    /// Normalized status.
    pub normalized_status: String,
    /// Artifact count.
    pub artifact_count: u32,
    /// Unavailable artifact/log count.
    pub unavailable_count: u32,
    /// Freshness state.
    pub freshness_state: String,
    /// Degraded state.
    pub degraded_state: String,
    /// Authority label.
    pub authority_label: String,
    /// Limited-action note. `not_applicable` means no narrowing occurred.
    pub limited_action_note: String,
    /// Provider handoff target.
    pub provider_handoff_target: String,
}

impl PipelineRunRow {
    /// Returns all validation violations for this row.
    #[must_use]
    pub fn validate(&self) -> Vec<PipelineRunRowViolation> {
        use PipelineRunRowViolation as V;

        let mut violations = Vec::new();
        if self.record_kind != M5_PIPELINE_RUN_ROW_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_PIPELINE_RUN_ROW_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if any_blank([
            &self.row_id,
            &self.pipeline_run_id,
            &self.provider_run_ref,
            &self.provider_label,
            &self.workflow_or_job_name,
            &self.review_anchor_ref,
            &self.normalized_status,
            &self.freshness_state,
            &self.degraded_state,
        ]) {
            violations.push(V::MissingIdentity);
        }
        if any_blank([
            &self.trigger.trigger_type,
            &self.trigger.actor_ref,
            &self.trigger.actor_class,
            &self.trigger.event_ref,
            &self.trigger.triggered_at,
        ]) {
            violations.push(V::MissingTrigger);
        }
        if !allowed(
            &self.normalized_status,
            [
                "queued",
                "running",
                "succeeded",
                "failed",
                "cancelled",
                "timed_out",
                "action_required",
                "blocked",
                "unknown",
            ],
        ) {
            violations.push(V::UnknownStatus);
        }
        if self.duration.state != "unknown" && self.duration.milliseconds.is_none() {
            violations.push(V::MissingDuration);
        }
        if self.duration.state == "unknown" && self.duration.milliseconds.is_some() {
            violations.push(V::MissingDuration);
        }
        if !self.branch_change_relation.has_any_ref()
            || (self.branch_change_relation.base_relation == "stale_base"
                && !self.branch_change_relation.stale_base)
        {
            violations.push(V::MissingBranchChangeRelation);
        }
        if any_blank([
            &self.artifact_summary.artifact_browser_ref,
            &self.artifact_summary.retention_label,
        ]) {
            violations.push(V::MissingArtifactSummary);
        }
        if self.artifact_summary.unavailable_count
            > self.artifact_summary.artifact_count + self.artifact_summary.log_count
        {
            violations.push(V::ArtifactSummaryContradiction);
        }
        if self.requires_freshness_note()
            && self
                .freshness_note
                .as_deref()
                .map(str::trim)
                .map(str::is_empty)
                .unwrap_or(true)
        {
            violations.push(V::FreshnessNoteMissing);
        }
        if self.run_control_authority.authority_label != "allowed"
            && self.run_control_authority.disabled_reason == "not_applicable"
        {
            violations.push(V::LimitedAuthorityHidden);
        }
        if (self.run_control_authority.rerun_available
            || self.run_control_authority.cancel_available)
            && self.run_control_authority.authority_label != "allowed"
        {
            violations.push(V::AuthorityContradiction);
        }
        if any_blank([
            &self.run_control_authority.authority_label,
            &self.run_control_authority.acting_identity_ref,
            &self.run_control_authority.disabled_reason,
            &self.open_details_action.action_ref,
            &self.open_details_action.action_label,
            &self.open_details_action.details_target_ref,
            &self.provider_handoff.handoff_target_ref,
            &self.provider_handoff.handoff_reason,
        ]) {
            violations.push(V::MissingActionOrHandoff);
        }
        if self.source_refs.is_empty() || self.consumer_surfaces.is_empty() {
            violations.push(V::MissingSourceOrConsumer);
        }
        if !has_all(
            &self.consumer_surfaces,
            [
                "review_pane",
                "pipeline_viewer",
                "project_health_center",
                "companion_client",
                "support_export",
                "release_proof",
            ],
        ) {
            violations.push(V::ConsumerSurfaceMissing);
        }
        if !has_all(&self.copy_export.formats, ["text", "json", "markdown"])
            || !self.copy_export.screenshot_only_prohibited
        {
            violations.push(V::CopyExportIncomplete);
        }
        if !has_all(
            &self.copy_export.export_fields,
            [
                "pipeline_run_id",
                "provider_run_ref",
                "provider_label",
                "workflow_or_job_name",
                "trigger",
                "duration",
                "branch_change_relation",
                "artifact_summary",
                "run_control_authority",
                "provider_handoff",
            ],
        ) {
            violations.push(V::CopyExportDropsRunTruth);
        }
        let copy_blob = format!(
            "{}\n{}\n{}",
            self.copy_export.text, self.copy_export.json, self.copy_export.markdown
        );
        for required in [
            &self.pipeline_run_id,
            &self.provider_label,
            &self.workflow_or_job_name,
            &self.trigger.actor_class,
            &self.branch_change_relation.base_relation,
            &self.normalized_status,
            &self.freshness_state,
            &self.run_control_authority.authority_label,
            &self.provider_handoff.handoff_target_ref,
        ] {
            if !copy_blob.contains(required) {
                violations.push(V::CopyExportDropsRunTruth);
                break;
            }
        }
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("pipeline row serializes"),
        ) {
            violations.push(V::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Whether this row needs an explicit freshness note.
    #[must_use]
    pub fn requires_freshness_note(&self) -> bool {
        self.degraded_state != "none"
            || matches!(
                self.freshness_state.as_str(),
                "stale" | "superseded" | "partial" | "blocked" | "policy_hidden" | "expired"
            )
    }

    /// Projects this row to a named consumer surface using the same field values.
    #[must_use]
    pub fn projection_for(&self, consumer_surface: &str) -> Option<PipelineRunSurfaceProjection> {
        if !self
            .consumer_surfaces
            .iter()
            .any(|surface| surface == consumer_surface)
        {
            return None;
        }
        Some(PipelineRunSurfaceProjection {
            consumer_surface: consumer_surface.to_owned(),
            row_id: self.row_id.clone(),
            pipeline_run_id: self.pipeline_run_id.clone(),
            provider_label: self.provider_label.clone(),
            workflow_or_job_name: self.workflow_or_job_name.clone(),
            trigger_actor_class: self.trigger.actor_class.clone(),
            base_relation: self.branch_change_relation.base_relation.clone(),
            normalized_status: self.normalized_status.clone(),
            artifact_count: self.artifact_summary.artifact_count,
            unavailable_count: self.artifact_summary.unavailable_count,
            freshness_state: self.freshness_state.clone(),
            degraded_state: self.degraded_state.clone(),
            authority_label: self.run_control_authority.authority_label.clone(),
            limited_action_note: if self.run_control_authority.authority_label == "allowed" {
                "not_applicable".to_owned()
            } else {
                self.run_control_authority.disabled_reason.clone()
            },
            provider_handoff_target: self.provider_handoff.handoff_target_ref.clone(),
        })
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only row fails.
    #[must_use]
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("pipeline row serializes")
    }
}

impl BranchChangeRelation {
    fn has_any_ref(&self) -> bool {
        [&self.branch_ref, &self.commit_ref, &self.change_object_ref]
            .iter()
            .any(|value| {
                value
                    .as_deref()
                    .map(str::trim)
                    .is_some_and(|s| !s.is_empty())
            })
    }
}

/// Error returned when the checked-in pipeline fixture fails to load.
#[derive(Debug)]
pub enum PipelineRunRowArtifactError {
    /// The fixture could not be parsed.
    Fixture(serde_json::Error),
    /// The parsed row failed validation.
    Validation(Vec<PipelineRunRowViolation>),
}

impl fmt::Display for PipelineRunRowArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fixture(err) => write!(f, "pipeline fixture parse error: {err}"),
            Self::Validation(violations) => {
                write!(f, "pipeline fixture failed validation: {violations:?}")
            }
        }
    }
}

impl Error for PipelineRunRowArtifactError {}

/// A validation invariant a pipeline-run row can violate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineRunRowViolation {
    /// The record-kind tag is wrong.
    WrongRecordKind,
    /// The schema version is wrong.
    WrongSchemaVersion,
    /// Row identity is incomplete.
    MissingIdentity,
    /// Trigger identity is incomplete.
    MissingTrigger,
    /// Normalized status is outside the controlled vocabulary.
    UnknownStatus,
    /// Duration disclosure is contradictory.
    MissingDuration,
    /// Branch/change relation is missing or contradictory.
    MissingBranchChangeRelation,
    /// Artifact summary is incomplete.
    MissingArtifactSummary,
    /// Artifact unavailable count exceeds known artifact/log total.
    ArtifactSummaryContradiction,
    /// Stale, partial, blocked, or superseded state lacks a freshness note.
    FreshnessNoteMissing,
    /// Limited authority is not explained by a disabled reason.
    LimitedAuthorityHidden,
    /// Rerun/cancel availability contradicts authority.
    AuthorityContradiction,
    /// Open-details action or provider handoff is missing.
    MissingActionOrHandoff,
    /// Source refs or consumer surfaces are missing.
    MissingSourceOrConsumer,
    /// Required consumer surface is missing.
    ConsumerSurfaceMissing,
    /// Copy/export formats are incomplete.
    CopyExportIncomplete,
    /// Copy/export drops run, trigger, artifact, authority, or handoff truth.
    CopyExportDropsRunTruth,
    /// Raw provider/log/path/credential material crossed the export boundary.
    RawBoundaryMaterialInExport,
}

/// Loads and validates the checked-in canonical pipeline-run fixture.
///
/// # Errors
///
/// Returns an error if the checked-in fixture cannot be parsed or fails
/// validation.
pub fn current_m5_pipeline_run_row() -> Result<PipelineRunRow, PipelineRunRowArtifactError> {
    let row: PipelineRunRow = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-pipeline-dependency-finding-components/pipeline_run_row.json"
    )))
    .map_err(PipelineRunRowArtifactError::Fixture)?;
    let violations = row.validate();
    if violations.is_empty() {
        Ok(row)
    } else {
        Err(PipelineRunRowArtifactError::Validation(violations))
    }
}

fn any_blank<'a>(values: impl IntoIterator<Item = &'a String>) -> bool {
    values.into_iter().any(|value| value.trim().is_empty())
}

fn allowed<'a>(value: &str, allowed_values: impl IntoIterator<Item = &'a str>) -> bool {
    allowed_values.into_iter().any(|allowed| allowed == value)
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
                || lower.contains("bearer ")
                || lower.contains("raw provider")
                || lower.contains("raw log")
                || lower.contains("/users/")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
