//! Shared manifest-diff card contract for package, review, health, companion,
//! support, and release surfaces.
//!
//! A manifest-diff card is the cross-surface object that tells users what a
//! package mutation would change in manifests and lockfiles, which scripts or
//! hooks are affected, which peer/runtime constraints move, whether a checkpoint
//! exists, how rollback works, and what apply boundary is in force. Package
//! manager, review pane, project-health, companion, support, and release
//! consumers render this same card rather than collapsing the change into a
//! generic dependency update.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`ManifestDiffCard`].
pub const M5_MANIFEST_DIFF_CARD_RECORD_KIND: &str = "m5_manifest_diff_card";

/// Schema version for the manifest diff card.
pub const M5_MANIFEST_DIFF_CARD_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_MANIFEST_DIFF_CARD_SCHEMA_REF: &str = "schemas/ui/m5-manifest-diff-card.schema.json";

/// Repo-relative path of the canonical first-consumer fixture.
pub const M5_MANIFEST_DIFF_CARD_FIXTURE_REF: &str =
    "fixtures/ui/m5-pipeline-dependency-finding-components/manifest_diff_card.json";

/// Reusable manifest diff card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestDiffCard {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable card id reused by all consumers.
    pub card_id: String,
    /// Canonical manifest-diff id.
    pub manifest_diff_id: String,
    /// Package operation ref.
    pub operation_ref: String,
    /// Manifest, scope, ecosystem, and workset identity.
    pub manifest_identity: ManifestIdentity,
    /// Change counts.
    pub change_summary: ChangeSummary,
    /// Script and hook preview rows.
    pub scripts_hooks_preview: Vec<ScriptHookChange>,
    /// Peer/runtime/engine/toolchain constraint changes.
    pub constraint_changes: Vec<ConstraintChange>,
    /// Checkpoint state.
    pub checkpoint_state: CheckpointState,
    /// Rollback state.
    pub rollback_state: RollbackState,
    /// Apply boundary.
    pub apply_boundary: ApplyBoundary,
    /// Freshness state.
    pub freshness_state: String,
    /// Narrowed/degraded state.
    pub degraded_state: String,
    /// Source documents/schemas.
    pub source_refs: Vec<String>,
    /// Consumers that must render this same card contract.
    pub consumer_surfaces: Vec<String>,
    /// Copy/export payloads.
    pub copy_export: CopyExport,
}

/// Manifest and ecosystem identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestIdentity {
    /// Manifest ref.
    pub manifest_ref: String,
    /// Whole workspace, selected manifest, workset slice, workspace member, or path/VCS target.
    pub scope_kind: String,
    /// Ecosystem token.
    pub ecosystem: String,
    /// Owning workspace or workset ref.
    pub workspace_or_workset_ref: String,
}

/// Change counts for the diff card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeSummary {
    /// Dependency change count.
    pub dependency_change_count: u32,
    /// Lockfile change count.
    pub lockfile_change_count: u32,
    /// Scripts/hooks change count.
    pub scripts_hooks_change_count: u32,
    /// Constraint change count.
    pub constraint_change_count: u32,
    /// Metadata change count.
    pub metadata_change_count: u32,
}

/// Script or hook preview row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptHookChange {
    /// Script or hook name.
    pub name: String,
    /// Added, removed, changed, or unchanged.
    pub change_type: String,
    /// Allowed, review-required, blocked, or policy-hidden.
    pub policy_label: String,
    /// Preview ref.
    pub preview_ref: String,
}

/// Constraint change row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstraintChange {
    /// Peer, runtime, engine, or toolchain.
    pub constraint_kind: String,
    /// Package/runtime ref.
    pub package_ref: String,
    /// Previous representation.
    pub from_repr: String,
    /// New representation.
    pub to_repr: String,
    /// Compatibility posture.
    pub compatibility_posture: String,
}

/// Checkpoint state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointState {
    /// Available, missing, not-applicable, or policy-hidden.
    pub state: String,
    /// Checkpoint ref.
    pub checkpoint_ref: String,
    /// Whether the checkpoint was created before apply.
    pub created_before_apply: bool,
}

/// Rollback state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackState {
    /// Available, compensating-only, unavailable, not-applicable, or policy-hidden.
    pub state: String,
    /// Rollback ref.
    pub rollback_ref: String,
    /// Rollback scope.
    pub rollback_scope: String,
}

/// Apply boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplyBoundary {
    /// Mutates, stages, inspect-only, or redacted-export.
    pub write_authority: String,
    /// Direct apply, stage-for-review, inspect-only, blocked, or policy-hidden.
    pub mutation_posture: String,
    /// Whether review is required.
    pub review_required: bool,
    /// Policy boundary ref.
    pub policy_boundary_ref: String,
    /// Disabled reason, or `not_applicable`.
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

/// Minimal projection consumed by package, review, health, companion, support,
/// and release surfaces. Every projection is derived from one
/// [`ManifestDiffCard`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestDiffSurfaceProjection {
    /// Consumer surface name.
    pub consumer_surface: String,
    /// Stable card id.
    pub card_id: String,
    /// Manifest diff id.
    pub manifest_diff_id: String,
    /// Manifest scope.
    pub manifest_scope: String,
    /// Ecosystem token.
    pub ecosystem: String,
    /// Scripts/hooks change count.
    pub scripts_hooks_change_count: u32,
    /// Constraint change count.
    pub constraint_change_count: u32,
    /// Checkpoint state.
    pub checkpoint_state: String,
    /// Rollback state.
    pub rollback_state: String,
    /// Write authority.
    pub write_authority: String,
    /// Mutation posture.
    pub mutation_posture: String,
    /// Limited-action note. `not_applicable` means no narrowing occurred.
    pub limited_action_note: String,
    /// Freshness state.
    pub freshness_state: String,
    /// Degraded state.
    pub degraded_state: String,
}

impl ManifestDiffCard {
    /// Returns all validation violations for this card.
    #[must_use]
    pub fn validate(&self) -> Vec<ManifestDiffCardViolation> {
        use ManifestDiffCardViolation as V;

        let mut violations = Vec::new();
        if self.record_kind != M5_MANIFEST_DIFF_CARD_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_MANIFEST_DIFF_CARD_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if any_blank([
            &self.card_id,
            &self.manifest_diff_id,
            &self.operation_ref,
            &self.freshness_state,
            &self.degraded_state,
        ]) {
            violations.push(V::MissingIdentity);
        }
        if any_blank([
            &self.manifest_identity.manifest_ref,
            &self.manifest_identity.scope_kind,
            &self.manifest_identity.ecosystem,
            &self.manifest_identity.workspace_or_workset_ref,
        ]) {
            violations.push(V::MissingManifestScope);
        }
        if !allowed(
            &self.manifest_identity.scope_kind,
            [
                "whole_workspace",
                "selected_manifest",
                "workset_slice",
                "workspace_member",
                "path_or_vcs_target",
            ],
        ) {
            violations.push(V::MissingManifestScope);
        }
        if self.change_summary.scripts_hooks_change_count as usize
            != self.scripts_hooks_preview.len()
            || self.change_summary.constraint_change_count as usize != self.constraint_changes.len()
        {
            violations.push(V::ChangeSummaryMismatch);
        }
        if self.scripts_hooks_preview.iter().any(|row| {
            any_blank([
                &row.name,
                &row.change_type,
                &row.policy_label,
                &row.preview_ref,
            ]) || !allowed(
                &row.policy_label,
                ["allowed", "review_required", "blocked", "policy_hidden"],
            )
        }) {
            violations.push(V::ScriptsHooksPreviewMissing);
        }
        if self.constraint_changes.iter().any(|row| {
            any_blank([
                &row.constraint_kind,
                &row.package_ref,
                &row.from_repr,
                &row.to_repr,
                &row.compatibility_posture,
            ]) || !allowed(
                &row.compatibility_posture,
                [
                    "compatible",
                    "review_required",
                    "breaking",
                    "unknown",
                    "policy_hidden",
                ],
            )
        }) {
            violations.push(V::ConstraintChangesMissing);
        }
        if any_blank([
            &self.checkpoint_state.state,
            &self.checkpoint_state.checkpoint_ref,
            &self.rollback_state.state,
            &self.rollback_state.rollback_ref,
            &self.rollback_state.rollback_scope,
        ]) {
            violations.push(V::CheckpointOrRollbackMissing);
        }
        if self.apply_boundary.write_authority == "mutates"
            && !self.checkpoint_state.created_before_apply
        {
            violations.push(V::ApplyBoundaryUnsafe);
        }
        if self.apply_boundary.write_authority != "mutates"
            && self.apply_boundary.mutation_posture == "direct_apply"
        {
            violations.push(V::ApplyBoundaryUnsafe);
        }
        if any_blank([
            &self.apply_boundary.write_authority,
            &self.apply_boundary.mutation_posture,
            &self.apply_boundary.policy_boundary_ref,
            &self.apply_boundary.disabled_reason,
        ]) {
            violations.push(V::ApplyBoundaryMissing);
        }
        if self.degraded_state == "rollback_unavailable" && self.rollback_state.state == "available"
        {
            violations.push(V::RollbackStateContradiction);
        }
        if self.source_refs.is_empty() || self.consumer_surfaces.is_empty() {
            violations.push(V::MissingSourceOrConsumer);
        }
        if !has_all(
            &self.consumer_surfaces,
            [
                "package_manager",
                "review_pane",
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
                "manifest_diff_id",
                "operation_ref",
                "manifest_identity",
                "change_summary",
                "scripts_hooks_preview",
                "constraint_changes",
                "checkpoint_state",
                "rollback_state",
                "apply_boundary",
            ],
        ) {
            violations.push(V::CopyExportDropsDiffTruth);
        }
        let copy_blob = format!(
            "{}\n{}\n{}",
            self.copy_export.text, self.copy_export.json, self.copy_export.markdown
        );
        for required in [
            &self.manifest_diff_id,
            &self.operation_ref,
            &self.manifest_identity.manifest_ref,
            &self.checkpoint_state.state,
            &self.rollback_state.state,
            &self.apply_boundary.write_authority,
        ] {
            if !copy_blob.contains(required) {
                violations.push(V::CopyExportDropsDiffTruth);
                break;
            }
        }
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("manifest diff card serializes"),
        ) {
            violations.push(V::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Projects this card to a named consumer surface using the same field values.
    #[must_use]
    pub fn projection_for(&self, consumer_surface: &str) -> Option<ManifestDiffSurfaceProjection> {
        if !self
            .consumer_surfaces
            .iter()
            .any(|surface| surface == consumer_surface)
        {
            return None;
        }
        let limited_action_note = if matches!(
            self.apply_boundary.write_authority.as_str(),
            "mutates" | "stages"
        ) {
            self.apply_boundary.disabled_reason.clone()
        } else {
            self.apply_boundary.mutation_posture.clone()
        };
        Some(ManifestDiffSurfaceProjection {
            consumer_surface: consumer_surface.to_owned(),
            card_id: self.card_id.clone(),
            manifest_diff_id: self.manifest_diff_id.clone(),
            manifest_scope: self.manifest_identity.scope_kind.clone(),
            ecosystem: self.manifest_identity.ecosystem.clone(),
            scripts_hooks_change_count: self.change_summary.scripts_hooks_change_count,
            constraint_change_count: self.change_summary.constraint_change_count,
            checkpoint_state: self.checkpoint_state.state.clone(),
            rollback_state: self.rollback_state.state.clone(),
            write_authority: self.apply_boundary.write_authority.clone(),
            mutation_posture: self.apply_boundary.mutation_posture.clone(),
            limited_action_note,
            freshness_state: self.freshness_state.clone(),
            degraded_state: self.degraded_state.clone(),
        })
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only card fails.
    #[must_use]
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("manifest diff card serializes")
    }
}

/// Error returned when the checked-in manifest diff fixture fails to load.
#[derive(Debug)]
pub enum ManifestDiffCardArtifactError {
    /// The fixture could not be parsed.
    Fixture(serde_json::Error),
    /// The parsed card failed validation.
    Validation(Vec<ManifestDiffCardViolation>),
}

impl fmt::Display for ManifestDiffCardArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fixture(err) => write!(f, "manifest diff fixture parse error: {err}"),
            Self::Validation(violations) => {
                write!(f, "manifest diff fixture failed validation: {violations:?}")
            }
        }
    }
}

impl Error for ManifestDiffCardArtifactError {}

/// A validation invariant a manifest diff card can violate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestDiffCardViolation {
    /// The record-kind tag is wrong.
    WrongRecordKind,
    /// The schema version is wrong.
    WrongSchemaVersion,
    /// Card identity is incomplete.
    MissingIdentity,
    /// Manifest identity is incomplete or unknown.
    MissingManifestScope,
    /// Change counts disagree with preview rows.
    ChangeSummaryMismatch,
    /// Script/hook preview is incomplete.
    ScriptsHooksPreviewMissing,
    /// Constraint changes are incomplete.
    ConstraintChangesMissing,
    /// Checkpoint or rollback state is incomplete.
    CheckpointOrRollbackMissing,
    /// Apply boundary is incomplete.
    ApplyBoundaryMissing,
    /// Apply boundary would permit an unsafe direct mutation.
    ApplyBoundaryUnsafe,
    /// Degraded rollback state contradicts rollback availability.
    RollbackStateContradiction,
    /// Source refs or consumer surfaces are missing.
    MissingSourceOrConsumer,
    /// Required consumer surface is missing.
    ConsumerSurfaceMissing,
    /// Copy/export formats are incomplete.
    CopyExportIncomplete,
    /// Copy/export drops hooks, constraints, checkpoint, rollback, or apply boundary truth.
    CopyExportDropsDiffTruth,
    /// Raw manifest/provider/path/credential material crossed the export boundary.
    RawBoundaryMaterialInExport,
}

/// Loads and validates the checked-in canonical manifest diff fixture.
///
/// # Errors
///
/// Returns an error if the checked-in fixture cannot be parsed or fails
/// validation.
pub fn current_m5_manifest_diff_card() -> Result<ManifestDiffCard, ManifestDiffCardArtifactError> {
    let card: ManifestDiffCard = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-pipeline-dependency-finding-components/manifest_diff_card.json"
    )))
    .map_err(ManifestDiffCardArtifactError::Fixture)?;
    let violations = card.validate();
    if violations.is_empty() {
        Ok(card)
    } else {
        Err(ManifestDiffCardArtifactError::Validation(violations))
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
                || lower.contains("raw manifest")
                || lower.contains("raw provider")
                || lower.contains("/users/")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
