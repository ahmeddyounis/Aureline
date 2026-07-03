//! Shared dependency-row contract for package, review, health, companion, and
//! support surfaces.
//!
//! A dependency row is the cross-surface object that tells users which package
//! is in play, what would change, where the manifest or lockfile impact lives,
//! and what risk context already exists. Package-manager views, review panes,
//! project-health centers, framework-pack health bundles, companion clients,
//! support exports, and release proof ingest this object instead of inventing
//! their own row layout or field vocabulary.
//!
//! The contract intentionally keeps package identity, ecosystem, dependency
//! relation, version delta, manifest scope, lockfile impact, advisory counts,
//! changelog action, license action, freshness, degraded state, and update state
//! in separate fields. Limited, blocked, stale, and policy-constrained updates
//! must remain visible as rows; they may not collapse into a missing row or a
//! disabled action with no state.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`DependencyRow`].
pub const M5_DEPENDENCY_ROW_RECORD_KIND: &str = "m5_dependency_row";

/// Schema version for the dependency row.
pub const M5_DEPENDENCY_ROW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_DEPENDENCY_ROW_SCHEMA_REF: &str = "schemas/ui/m5-dependency-row.schema.json";

/// Repo-relative path of the canonical first-consumer fixture.
pub const M5_DEPENDENCY_ROW_FIXTURE_REF: &str =
    "fixtures/ui/m5-pipeline-dependency-finding-components/dependency_row.json";

/// Reusable dependency row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyRow {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable row id reused by all consumers.
    pub row_id: String,
    /// Display name shown by row consumers.
    pub package_name: String,
    /// Canonical package coordinate ref.
    pub package_identity_ref: String,
    /// Exact resolved identity ref.
    pub resolved_identity_ref: String,
    /// Manifest, scope, ecosystem, and workset identity.
    pub manifest_identity: ManifestIdentity,
    /// Direct, transitive, workspace-local, path, or VCS relation.
    pub dependency_relation: String,
    /// Current-to-target version delta.
    pub version_delta: VersionDelta,
    /// Manifest/lockfile impact summary.
    pub lockfile_impact: LockfileImpact,
    /// Advisory counts and freshness.
    pub advisory_summary: AdvisorySummary,
    /// License action, separate from changelog action.
    pub license_action: RowAction,
    /// Changelog action, separate from license action.
    pub changelog_action: RowAction,
    /// Whether update action is available, limited, blocked, or policy constrained.
    pub update_state: String,
    /// Registry/advisory/package freshness state.
    pub freshness_state: String,
    /// Narrowed/degraded state.
    pub degraded_state: String,
    /// Source documents/schemas.
    pub source_refs: Vec<String>,
    /// Consumers that must render this same row contract.
    pub consumer_surfaces: Vec<String>,
    /// Copy/export payloads.
    pub copy_export: CopyExport,
}

/// Manifest and ecosystem identity for a dependency row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestIdentity {
    /// Opaque manifest ref.
    pub manifest_ref: String,
    /// Whole workspace, selected manifest, workset slice, workspace member, path/VCS target.
    pub scope_kind: String,
    /// Ecosystem token.
    pub ecosystem: String,
    /// Owning workspace or workset ref.
    pub workspace_or_workset_ref: String,
}

/// Current-to-target version delta for a dependency row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionDelta {
    /// Current or resolved version display.
    pub current_version_repr: String,
    /// Target or latest version display.
    pub target_version_repr: String,
    /// Requested manifest range display.
    pub requested_range_repr: String,
    /// None, patch, minor, major, security_patch, lockfile_only, workspace_convergence, unknown.
    pub delta_class: String,
    /// Lockfile authority state.
    pub lockfile_authority: String,
}

/// Manifest/lockfile impact summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockfileImpact {
    /// None, manifest_only, lockfile_only, manifest_and_lockfile, blocked_by_policy, unknown.
    pub impact_state: String,
    /// Opaque lockfile ref or `lockfile:none`.
    pub lockfile_ref: String,
    /// Number of affected lockfile entries.
    pub affected_entries: u32,
    /// Whether review is required before the change can apply.
    pub review_required: bool,
    /// Short export-safe summary.
    pub summary: String,
}

/// Advisory counts and freshness for a dependency row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvisorySummary {
    /// Total advisories affecting the row.
    pub advisory_count: u32,
    /// Highest severity token.
    pub highest_severity: String,
    /// Advisory feed freshness token.
    pub advisory_freshness: String,
    /// Suppression counts that remain visible.
    pub suppression_counts: SuppressionCounts,
}

/// Suppression counts preserved on dependency rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuppressionCounts {
    /// Unsuppressed advisory count.
    pub unsuppressed: u32,
    /// Suppressed-until-review advisory count.
    pub suppressed_until_review: u32,
    /// Suppressed-by-policy advisory count.
    pub suppressed_by_policy: u32,
    /// Expired exception count.
    pub exception_expired: u32,
}

/// Row action reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RowAction {
    /// Available, not_applicable, blocked, policy_hidden, or unavailable.
    pub action_state: String,
    /// Stable action target ref.
    pub action_ref: String,
    /// Whether the action is currently invokable.
    pub available: bool,
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

/// Minimal projection consumed by package, review, health, companion, and
/// support surfaces. Every projection is derived from the same [`DependencyRow`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencySurfaceProjection {
    /// Consumer surface name.
    pub consumer_surface: String,
    /// Stable row id.
    pub row_id: String,
    /// Package display name.
    pub package_name: String,
    /// Ecosystem token.
    pub ecosystem: String,
    /// Dependency relation token.
    pub dependency_relation: String,
    /// Manifest-scope token.
    pub manifest_scope: String,
    /// Current version display.
    pub current_version: String,
    /// Target/latest version display.
    pub target_version: String,
    /// Version delta token.
    pub delta_class: String,
    /// Lockfile impact token.
    pub lockfile_impact: String,
    /// Advisory count.
    pub advisory_count: u32,
    /// License action state.
    pub license_action_state: String,
    /// Changelog action state.
    pub changelog_action_state: String,
    /// Update state token.
    pub update_state: String,
    /// Freshness state.
    pub freshness_state: String,
    /// Degraded state.
    pub degraded_state: String,
}

impl DependencyRow {
    /// Returns all validation violations for this row.
    #[must_use]
    pub fn validate(&self) -> Vec<DependencyRowViolation> {
        use DependencyRowViolation as V;

        let mut violations = Vec::new();
        if self.record_kind != M5_DEPENDENCY_ROW_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_DEPENDENCY_ROW_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if any_blank([
            &self.row_id,
            &self.package_name,
            &self.package_identity_ref,
            &self.resolved_identity_ref,
            &self.dependency_relation,
            &self.update_state,
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
            &self.dependency_relation,
            ["direct", "transitive", "workspace_local", "path", "vcs"],
        ) {
            violations.push(V::UnknownDependencyRelation);
        }
        if !allowed(
            &self.manifest_identity.ecosystem,
            ["cargo", "node_pnpm", "python_pip", "other"],
        ) {
            violations.push(V::UnknownEcosystem);
        }
        if any_blank([
            &self.version_delta.current_version_repr,
            &self.version_delta.target_version_repr,
            &self.version_delta.requested_range_repr,
            &self.version_delta.delta_class,
            &self.version_delta.lockfile_authority,
        ]) {
            violations.push(V::MissingVersionDelta);
        }
        if self.version_delta.current_version_repr == self.version_delta.target_version_repr
            && self.version_delta.delta_class != "none"
        {
            violations.push(V::VersionDeltaContradiction);
        }
        if any_blank([
            &self.lockfile_impact.impact_state,
            &self.lockfile_impact.lockfile_ref,
            &self.lockfile_impact.summary,
        ]) || !allowed(
            &self.lockfile_impact.impact_state,
            [
                "none",
                "manifest_only",
                "lockfile_only",
                "manifest_and_lockfile",
                "blocked_by_policy",
                "unknown",
            ],
        ) {
            violations.push(V::MissingLockfileImpact);
        }
        if self.lockfile_impact.impact_state != "none"
            && self.lockfile_impact.lockfile_ref == "lockfile:none"
        {
            violations.push(V::MissingLockfileImpact);
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
            ],
        ) {
            violations.push(V::CoreConsumerSurfaceMissing);
        }
        if !has_all(
            &self.consumer_surfaces,
            ["framework_pack_health", "support_export", "release_proof"],
        ) {
            violations.push(V::ProofConsumerSurfaceMissing);
        }
        if !allowed(
            &self.update_state,
            [
                "available",
                "limited",
                "blocked",
                "policy_constrained",
                "inspect_only",
                "not_applicable",
            ],
        ) {
            violations.push(V::UnknownUpdateState);
        }
        if self.update_state != "available"
            && self.update_state != "not_applicable"
            && self.degraded_state == "none"
        {
            violations.push(V::ConstrainedUpdateStateHidden);
        }
        self.validate_action(
            &self.license_action,
            V::LicenseActionMissing,
            &mut violations,
        );
        self.validate_action(
            &self.changelog_action,
            V::ChangelogActionMissing,
            &mut violations,
        );
        if self.advisory_summary.suppression_counts.total() > self.advisory_summary.advisory_count {
            violations.push(V::AdvisorySuppressionMismatch);
        }
        if !has_all(&self.copy_export.formats, ["text", "json", "markdown"])
            || !self.copy_export.screenshot_only_prohibited
        {
            violations.push(V::CopyExportIncomplete);
        }
        if !has_all(
            &self.copy_export.export_fields,
            [
                "package_name",
                "manifest_identity",
                "dependency_relation",
                "version_delta",
                "lockfile_impact",
                "advisory_summary",
                "license_action",
                "changelog_action",
                "update_state",
            ],
        ) {
            violations.push(V::CopyExportDropsDependencyTruth);
        }
        let copy_blob = format!(
            "{}\n{}\n{}",
            self.copy_export.text, self.copy_export.json, self.copy_export.markdown
        );
        for required in [
            &self.package_name,
            &self.manifest_identity.ecosystem,
            &self.manifest_identity.scope_kind,
            &self.dependency_relation,
            &self.version_delta.current_version_repr,
            &self.version_delta.target_version_repr,
            &self.lockfile_impact.impact_state,
            &self.advisory_summary.advisory_count.to_string(),
            &self.license_action.action_state,
            &self.changelog_action.action_state,
            &self.update_state,
        ] {
            if !copy_blob.contains(required) {
                violations.push(V::CopyExportDropsDependencyTruth);
                break;
            }
        }
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("dependency row serializes"),
        ) {
            violations.push(V::RawBoundaryMaterialInExport);
        }

        violations
    }

    fn validate_action(
        &self,
        action: &RowAction,
        missing: DependencyRowViolation,
        violations: &mut Vec<DependencyRowViolation>,
    ) {
        if any_blank([
            &action.action_state,
            &action.action_ref,
            &action.disabled_reason,
        ]) {
            violations.push(missing);
            return;
        }
        match action.action_state.as_str() {
            "available" if !action.available || action.disabled_reason != "not_applicable" => {
                violations.push(missing);
            }
            "blocked" | "policy_hidden" | "unavailable" if action.available => {
                violations.push(missing);
            }
            "not_applicable" | "available" | "blocked" | "policy_hidden" | "unavailable" => {}
            _ => violations.push(missing),
        }
    }

    /// Projects this row to a named consumer surface using the same field values.
    #[must_use]
    pub fn projection_for(&self, consumer_surface: &str) -> Option<DependencySurfaceProjection> {
        if !self
            .consumer_surfaces
            .iter()
            .any(|surface| surface == consumer_surface)
        {
            return None;
        }
        Some(DependencySurfaceProjection {
            consumer_surface: consumer_surface.to_owned(),
            row_id: self.row_id.clone(),
            package_name: self.package_name.clone(),
            ecosystem: self.manifest_identity.ecosystem.clone(),
            dependency_relation: self.dependency_relation.clone(),
            manifest_scope: self.manifest_identity.scope_kind.clone(),
            current_version: self.version_delta.current_version_repr.clone(),
            target_version: self.version_delta.target_version_repr.clone(),
            delta_class: self.version_delta.delta_class.clone(),
            lockfile_impact: self.lockfile_impact.impact_state.clone(),
            advisory_count: self.advisory_summary.advisory_count,
            license_action_state: self.license_action.action_state.clone(),
            changelog_action_state: self.changelog_action.action_state.clone(),
            update_state: self.update_state.clone(),
            freshness_state: self.freshness_state.clone(),
            degraded_state: self.degraded_state.clone(),
        })
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only row fails.
    #[must_use]
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("dependency row serializes")
    }
}

impl SuppressionCounts {
    fn total(&self) -> u32 {
        self.unsuppressed
            + self.suppressed_until_review
            + self.suppressed_by_policy
            + self.exception_expired
    }
}

/// Error returned when the checked-in dependency fixture fails to load.
#[derive(Debug)]
pub enum DependencyRowArtifactError {
    /// The fixture could not be parsed.
    Fixture(serde_json::Error),
    /// The parsed row failed validation.
    Validation(Vec<DependencyRowViolation>),
}

impl fmt::Display for DependencyRowArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fixture(err) => write!(f, "dependency fixture parse error: {err}"),
            Self::Validation(violations) => {
                write!(f, "dependency fixture failed validation: {violations:?}")
            }
        }
    }
}

impl Error for DependencyRowArtifactError {}

/// A validation invariant a dependency row can violate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyRowViolation {
    /// The record-kind tag is wrong.
    WrongRecordKind,
    /// The schema version is wrong.
    WrongSchemaVersion,
    /// Row identity is incomplete.
    MissingIdentity,
    /// Manifest scope or ecosystem is incomplete.
    MissingManifestScope,
    /// Dependency relation is outside the controlled vocabulary.
    UnknownDependencyRelation,
    /// Ecosystem is outside the controlled vocabulary.
    UnknownEcosystem,
    /// Version delta is incomplete.
    MissingVersionDelta,
    /// Version delta says changed while current and target are equal.
    VersionDeltaContradiction,
    /// Lockfile impact is incomplete or contradictory.
    MissingLockfileImpact,
    /// Source refs or consumer surfaces are missing.
    MissingSourceOrConsumer,
    /// Package, review, health, or companion consumer is missing.
    CoreConsumerSurfaceMissing,
    /// Framework-pack, support, or release proof consumer is missing.
    ProofConsumerSurfaceMissing,
    /// Update state is outside the controlled vocabulary.
    UnknownUpdateState,
    /// Limited, blocked, policy-constrained, or inspect-only state was hidden.
    ConstrainedUpdateStateHidden,
    /// License action is missing or contradictory.
    LicenseActionMissing,
    /// Changelog action is missing or contradictory.
    ChangelogActionMissing,
    /// Suppression counts exceed total advisory count.
    AdvisorySuppressionMismatch,
    /// Copy/export formats are incomplete.
    CopyExportIncomplete,
    /// Copy/export drops dependency, manifest, action, or update-state truth.
    CopyExportDropsDependencyTruth,
    /// Raw path, credential, provider body, or advisory material crossed the export boundary.
    RawBoundaryMaterialInExport,
}

/// Loads and validates the checked-in canonical dependency fixture.
///
/// # Errors
///
/// Returns an error if the checked-in fixture cannot be parsed or fails
/// validation.
pub fn current_m5_dependency_row() -> Result<DependencyRow, DependencyRowArtifactError> {
    let row: DependencyRow = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-pipeline-dependency-finding-components/dependency_row.json"
    )))
    .map_err(DependencyRowArtifactError::Fixture)?;
    let violations = row.validate();
    if violations.is_empty() {
        Ok(row)
    } else {
        Err(DependencyRowArtifactError::Validation(violations))
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
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("raw advisory")
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
