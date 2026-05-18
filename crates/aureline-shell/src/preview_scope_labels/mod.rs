//! Product-facing projection for qualified preview-surface labels.
//!
//! The generated packet at `artifacts/compat/m3/qualified_preview_rows.json`
//! is the shared source for notebook, voice, browser-companion, and
//! preview-canvas lifecycle labels, client-scope chips, handoff targets, and
//! downgrade reasons. This module keeps shell consumers read-only: Start
//! Center, Help/About, service health, marketplace/help metadata, and support
//! export can quote the packet without inventing surface-local wording.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`PreviewScopeLabelRegister`].
pub const PREVIEW_SCOPE_LABEL_REGISTER_RECORD_KIND: &str = "m3_qualified_preview_row_register";

/// Schema version for [`PreviewScopeLabelRegister`].
pub const PREVIEW_SCOPE_LABEL_REGISTER_SCHEMA_VERSION: u32 = 1;

/// Surface families that must have explicit preview-scope labels.
pub const REQUIRED_PREVIEW_SCOPE_SURFACE_FAMILIES: [&str; 4] =
    ["browser_companion", "notebook", "preview_canvas", "voice"];

/// Product and export consumers that must project each preview-scope row.
pub const REQUIRED_PREVIEW_SCOPE_PROJECTIONS: [&str; 7] = [
    "compatibility_report",
    "docs_help",
    "help_about",
    "marketplace_help_metadata",
    "service_health",
    "start_center",
    "support_export",
];

/// Generated preview-surface qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewScopeLabelRegister {
    /// Discriminator for this packet family.
    pub record_kind: String,
    /// Packet schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Repository-relative packet ref.
    pub packet_ref: String,
    /// Source register ref used by the generator.
    pub source_claimed_surface_register_ref: String,
    /// Release docs ref that explains the row shape.
    pub docs_ref: String,
    /// Fixture directory used by the validator.
    pub fixture_dir_ref: String,
    /// Reviewer-facing as-of date.
    pub as_of: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Summary counts for product and support consumers.
    pub summary: PreviewScopeLabelSummary,
    /// Qualified preview rows.
    pub rows: Vec<PreviewScopeLabelRow>,
    /// Support-export-safe rows paired to [`Self::rows`].
    pub support_export_rows: Vec<PreviewScopeSupportExportRow>,
    /// True when raw private material is excluded from the packet.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority is excluded from the packet.
    pub ambient_authority_excluded: bool,
}

impl PreviewScopeLabelRegister {
    /// Loads a preview-scope packet from bytes.
    ///
    /// # Errors
    ///
    /// Returns [`PreviewScopeLabelLoadError`] when JSON parsing fails or the
    /// packet record-kind/schema-version does not match this module.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, PreviewScopeLabelLoadError> {
        let register: Self = serde_json::from_slice(bytes)
            .map_err(|source| PreviewScopeLabelLoadError::Json { source })?;
        if register.record_kind != PREVIEW_SCOPE_LABEL_REGISTER_RECORD_KIND {
            return Err(PreviewScopeLabelLoadError::SchemaMismatch {
                expected_record_kind: PREVIEW_SCOPE_LABEL_REGISTER_RECORD_KIND,
                actual_record_kind: register.record_kind,
            });
        }
        if register.schema_version != PREVIEW_SCOPE_LABEL_REGISTER_SCHEMA_VERSION {
            return Err(PreviewScopeLabelLoadError::SchemaVersionMismatch {
                expected: PREVIEW_SCOPE_LABEL_REGISTER_SCHEMA_VERSION,
                actual: register.schema_version,
            });
        }
        Ok(register)
    }

    /// Loads a preview-scope packet from a JSON file.
    ///
    /// # Errors
    ///
    /// Returns [`PreviewScopeLabelLoadError`] when reading or parsing fails.
    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, PreviewScopeLabelLoadError> {
        let path = path.as_ref();
        let bytes = std::fs::read(path).map_err(|source| PreviewScopeLabelLoadError::Io {
            path: path.display().to_string(),
            source,
        })?;
        Self::from_bytes(&bytes)
    }

    /// Validates product-facing row parity and handoff honesty.
    ///
    /// # Errors
    ///
    /// Returns one or more [`PreviewScopeLabelFinding`] values when rows do
    /// not cover required families, omit projections, drift from support
    /// export, or imply native-depth browser/voice capability.
    pub fn validate(&self) -> Result<(), Vec<PreviewScopeLabelFinding>> {
        let mut findings = Vec::new();

        if self.summary.row_count as usize != self.rows.len() {
            findings.push(finding(
                None,
                "packet.summary.row_count",
                "summary row count must match preview rows",
            ));
        }
        if self.summary.support_export_row_count as usize != self.support_export_rows.len() {
            findings.push(finding(
                None,
                "packet.summary.support_export_row_count",
                "summary support-export row count must match support-export rows",
            ));
        }
        if self.summary.handoff_required_row_count as usize
            != self.rows.iter().filter(|row| row.handoff.required).count()
        {
            findings.push(finding(
                None,
                "packet.summary.handoff_required_row_count",
                "summary handoff count must match preview rows",
            ));
        }
        if self.summary.preview_lifecycle_row_count as usize
            != self
                .rows
                .iter()
                .filter(|row| row.effective_lifecycle_label == "preview")
                .count()
        {
            findings.push(finding(
                None,
                "packet.summary.preview_lifecycle_row_count",
                "summary preview lifecycle count must match preview rows",
            ));
        }
        if self.summary.beta_lifecycle_row_count as usize
            != self
                .rows
                .iter()
                .filter(|row| row.effective_lifecycle_label == "beta")
                .count()
        {
            findings.push(finding(
                None,
                "packet.summary.beta_lifecycle_row_count",
                "summary beta lifecycle count must match preview rows",
            ));
        }
        if !self.raw_private_material_excluded {
            findings.push(finding(
                None,
                "packet.raw_private_material",
                "raw private material must be excluded",
            ));
        }
        if !self.ambient_authority_excluded {
            findings.push(finding(
                None,
                "packet.ambient_authority",
                "ambient authority must be excluded",
            ));
        }

        let families: BTreeSet<_> = self
            .rows
            .iter()
            .map(|row| row.surface_family.as_str())
            .collect();
        for required in REQUIRED_PREVIEW_SCOPE_SURFACE_FAMILIES {
            if !families.contains(required) {
                findings.push(finding(
                    None,
                    "packet.surface_family.missing",
                    format!("missing required preview surface family {required}"),
                ));
            }
        }
        let summary_families: BTreeSet<_> = self
            .summary
            .covered_surface_families
            .iter()
            .map(String::as_str)
            .collect();
        let summary_required_families: BTreeSet<_> = self
            .summary
            .required_surface_families
            .iter()
            .map(String::as_str)
            .collect();
        if summary_families != families {
            findings.push(finding(
                None,
                "packet.summary.covered_surface_families",
                "summary surface-family coverage must match preview rows",
            ));
        }
        if summary_required_families
            != REQUIRED_PREVIEW_SCOPE_SURFACE_FAMILIES
                .into_iter()
                .collect::<BTreeSet<_>>()
        {
            findings.push(finding(
                None,
                "packet.summary.required_surface_families",
                "summary required surface families must match the shell contract",
            ));
        }

        let support_by_ref: BTreeMap<_, _> = self
            .support_export_rows
            .iter()
            .map(|row| (row.surface_ref.as_str(), row))
            .collect();
        if support_by_ref.len() != self.rows.len() {
            findings.push(finding(
                None,
                "packet.support_export.count_drift",
                "support export row count must match preview row count",
            ));
        }

        for row in &self.rows {
            validate_row(
                row,
                support_by_ref.get(row.surface_id.as_str()).copied(),
                &mut findings,
            );
        }

        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }

    /// Returns rows that project to a named product or export surface.
    pub fn rows_for_consumer(&self, surface: &str) -> Vec<&PreviewScopeLabelRow> {
        self.rows
            .iter()
            .filter(|row| {
                row.consumer_projections
                    .iter()
                    .any(|projection| projection.surface == surface)
            })
            .collect()
    }

    /// Renders an export-safe plaintext summary.
    pub fn render_plaintext(&self) -> String {
        let mut text = String::new();
        text.push_str(&format!("Preview surface labels: {}\n", self.packet_id));
        text.push_str(&format!("As of: {}\n", self.as_of));
        for row in &self.rows {
            text.push_str(&format!(
                "- {}: {} / {} / {}; handoff={} ({})\n",
                row.public_label,
                row.effective_lifecycle_label_display,
                row.effective_support_label,
                row.client_scope_label,
                row.handoff.target_label,
                row.handoff.limitation_statement,
            ));
        }
        text
    }
}

fn validate_row(
    row: &PreviewScopeLabelRow,
    support: Option<&PreviewScopeSupportExportRow>,
    findings: &mut Vec<PreviewScopeLabelFinding>,
) {
    if !matches!(row.effective_lifecycle_label.as_str(), "preview" | "beta") {
        findings.push(finding(
            Some(&row.surface_id),
            "row.lifecycle.unknown",
            "preview rows must carry preview or beta lifecycle labels",
        ));
    }
    if row.client_scope.trim().is_empty() || row.client_scope_label.trim().is_empty() {
        findings.push(finding(
            Some(&row.surface_id),
            "row.client_scope.missing",
            "client-scope chip is required",
        ));
    }
    let projection_surfaces: BTreeSet<_> = row
        .consumer_projections
        .iter()
        .map(|projection| projection.surface.as_str())
        .collect();
    for required in REQUIRED_PREVIEW_SCOPE_PROJECTIONS {
        if !projection_surfaces.contains(required) {
            findings.push(finding(
                Some(&row.surface_id),
                "row.consumer_projection.missing",
                format!("missing consumer projection {required}"),
            ));
        }
    }
    if matches!(row.surface_family.as_str(), "browser_companion" | "voice")
        && row.native_depth_capability_claimed
    {
        findings.push(finding(
            Some(&row.surface_id),
            "row.native_depth_overclaim",
            "browser companion and voice rows must not claim native-depth capability",
        ));
    }
    if !row.handoff.required
        || row.handoff.target.trim().is_empty()
        || row.handoff.limitation_statement.trim().is_empty()
    {
        findings.push(finding(
            Some(&row.surface_id),
            "row.handoff.missing",
            "handoff target and limitation statement are required",
        ));
    }

    let Some(support) = support else {
        findings.push(finding(
            Some(&row.surface_id),
            "row.support_export.missing",
            "support export row is missing",
        ));
        return;
    };
    if &row.support_export != support {
        findings.push(finding(
            Some(&row.surface_id),
            "row.support_export.inline_drift",
            "inline support export row drifted from top-level support export row",
        ));
    }
    let parity = [
        (
            "surface_family",
            support.surface_family.as_str(),
            row.surface_family.as_str(),
        ),
        (
            "lifecycle_label",
            support.lifecycle_label.as_str(),
            row.effective_lifecycle_label.as_str(),
        ),
        (
            "support_class",
            support.support_class.as_str(),
            row.effective_support_class.as_str(),
        ),
        (
            "client_scope",
            support.client_scope.as_str(),
            row.client_scope.as_str(),
        ),
        (
            "freshness_state",
            support.freshness_state.as_str(),
            row.evidence.freshness_state.as_str(),
        ),
        (
            "handoff_target",
            support.handoff_target.as_str(),
            row.handoff.target.as_str(),
        ),
    ];
    for (field, support_value, row_value) in parity {
        if support_value != row_value {
            findings.push(finding(
                Some(&row.surface_id),
                format!("row.support_export.{field}.drift"),
                format!("support export {field} drifted from preview row"),
            ));
        }
    }
    if support.handoff_required != row.handoff.required {
        findings.push(finding(
            Some(&row.surface_id),
            "row.support_export.handoff_required.drift",
            "support export handoff flag drifted from preview row",
        ));
    }
    if !support.raw_private_material_excluded || !support.ambient_authority_excluded {
        findings.push(finding(
            Some(&row.surface_id),
            "row.support_export.private_or_authority",
            "support export rows must exclude raw private material and ambient authority",
        ));
    }
}

fn finding(
    row_id: Option<&str>,
    check_id: impl Into<String>,
    message: impl Into<String>,
) -> PreviewScopeLabelFinding {
    PreviewScopeLabelFinding {
        row_id: row_id.map(str::to_owned),
        check_id: check_id.into(),
        message: message.into(),
    }
}

/// Summary counts for qualified preview rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewScopeLabelSummary {
    /// Number of qualified preview rows.
    pub row_count: u32,
    /// Required surface families the packet must cover.
    pub required_surface_families: Vec<String>,
    /// Surface families present in the packet.
    pub covered_surface_families: Vec<String>,
    /// Number of rows requiring explicit handoff.
    pub handoff_required_row_count: u32,
    /// Number of rows whose effective lifecycle is preview.
    pub preview_lifecycle_row_count: u32,
    /// Number of rows whose effective lifecycle is beta.
    pub beta_lifecycle_row_count: u32,
    /// Number of support-export rows.
    pub support_export_row_count: u32,
}

/// One qualified preview-surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewScopeLabelRow {
    /// Stable surface id.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Required family: notebook, voice, browser companion, or preview canvas.
    pub surface_family: String,
    /// Short product label.
    pub public_label: String,
    /// Lifecycle declared by the source register.
    pub declared_lifecycle_label: String,
    /// Display label for [`Self::declared_lifecycle_label`].
    pub declared_lifecycle_label_display: String,
    /// Lifecycle after freshness and gate downgrades.
    pub effective_lifecycle_label: String,
    /// Display label for [`Self::effective_lifecycle_label`].
    pub effective_lifecycle_label_display: String,
    /// Support class declared by the source register.
    pub declared_support_class: String,
    /// Display label for [`Self::declared_support_class`].
    pub declared_support_label: String,
    /// Support class after freshness and gate downgrades.
    pub effective_support_class: String,
    /// Display label for [`Self::effective_support_class`].
    pub effective_support_label: String,
    /// Machine-readable client-scope token.
    pub client_scope: String,
    /// Product-facing client-scope chip label.
    pub client_scope_label: String,
    /// True only if the row claims native-depth behavior in its current client.
    pub native_depth_capability_claimed: bool,
    /// Evidence freshness and references.
    pub evidence: PreviewScopeEvidence,
    /// Qualification gates that derive downgrades.
    pub qualification_gates: Vec<PreviewScopeQualificationGate>,
    /// Required handoff target and limitation text.
    pub handoff: PreviewScopeHandoff,
    /// Downgrade reason tokens and prose.
    pub downgrade: PreviewScopeDowngrade,
    /// Consumer projections that must quote this row.
    pub consumer_projections: Vec<PreviewScopeConsumerProjection>,
    /// Paired support-export row.
    pub support_export: PreviewScopeSupportExportRow,
    /// One-line export-safe summary.
    pub display_summary: String,
}

/// Evidence freshness fields for a preview-scope row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewScopeEvidence {
    /// Evidence as-of date when available.
    pub as_of: Option<String>,
    /// Review window in days.
    pub review_window_days: u32,
    /// Effective freshness state.
    pub freshness_state: String,
    /// Evidence age in days when an as-of date exists.
    pub evidence_age_days: Option<i32>,
    /// Evidence refs backing the row.
    pub evidence_refs: Vec<String>,
}

/// One qualification gate feeding a preview row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewScopeQualificationGate {
    /// Stable gate id.
    pub gate_id: String,
    /// Gate state.
    pub state: String,
    /// Whether the gate is required for beta publication.
    pub required_for_beta: bool,
    /// Support class to narrow to when the gate is incomplete.
    pub downgrade_to: String,
    /// Reason token emitted when the gate narrows the row.
    pub downgrade_reason_token: String,
}

/// Handoff target and limitation text for a preview row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewScopeHandoff {
    /// Whether this row requires an explicit handoff path.
    pub required: bool,
    /// Machine-readable handoff target token.
    pub target: String,
    /// Product-facing handoff target label.
    pub target_label: String,
    /// Stable route ref for the handoff.
    pub route_ref: String,
    /// Product-facing limitation statement.
    pub limitation_statement: String,
    /// Whether the handoff preserves current object/context identity.
    pub preserves_context: bool,
}

/// Downgrade reasons and fallback classes for a preview row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewScopeDowngrade {
    /// Machine-readable reason tokens emitted by the generator.
    pub downgrade_reason_tokens: Vec<String>,
    /// Export-safe downgrade reason prose.
    pub downgrade_reasons: Vec<String>,
    /// Support class used when evidence is stale.
    pub stale_evidence_downgrade_to: String,
    /// Support class used when evidence is missing.
    pub missing_evidence_downgrade_to: String,
}

/// Consumer projection metadata for one preview row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewScopeConsumerProjection {
    /// Consumer surface token.
    pub surface: String,
    /// Ref that the consumer quotes for this row.
    pub projection_ref: String,
}

/// Support-export-safe projection paired to a preview row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewScopeSupportExportRow {
    /// Stable support row id.
    pub support_row_id: String,
    /// Source preview-surface row id.
    pub surface_ref: String,
    /// Surface family token.
    pub surface_family: String,
    /// Effective lifecycle label token.
    pub lifecycle_label: String,
    /// Effective support class token.
    pub support_class: String,
    /// Client-scope token.
    pub client_scope: String,
    /// Freshness state token.
    pub freshness_state: String,
    /// Whether an explicit handoff is required.
    pub handoff_required: bool,
    /// Handoff target token.
    pub handoff_target: String,
    /// Downgrade reason tokens.
    pub downgrade_reason_tokens: Vec<String>,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority is excluded.
    pub ambient_authority_excluded: bool,
}

/// Validation finding for preview-scope packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreviewScopeLabelFinding {
    /// Row id associated with the finding, when applicable.
    pub row_id: Option<String>,
    /// Stable check id.
    pub check_id: String,
    /// Human-readable finding message.
    pub message: String,
}

/// Error returned while loading a preview-scope packet.
#[derive(Debug)]
pub enum PreviewScopeLabelLoadError {
    /// File read failed.
    Io {
        /// Path that failed to load.
        path: String,
        /// Source I/O error.
        source: std::io::Error,
    },
    /// JSON parsing failed.
    Json {
        /// Source JSON error.
        source: serde_json::Error,
    },
    /// Record kind did not match this module.
    SchemaMismatch {
        /// Expected record kind.
        expected_record_kind: &'static str,
        /// Actual record kind.
        actual_record_kind: String,
    },
    /// Schema version did not match this module.
    SchemaVersionMismatch {
        /// Expected schema version.
        expected: u32,
        /// Actual schema version.
        actual: u32,
    },
}

impl fmt::Display for PreviewScopeLabelLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => write!(f, "failed to read {path}: {source}"),
            Self::Json { source } => write!(f, "failed to parse preview scope packet: {source}"),
            Self::SchemaMismatch {
                expected_record_kind,
                actual_record_kind,
            } => write!(
                f,
                "expected preview scope record kind {expected_record_kind}, got {actual_record_kind}"
            ),
            Self::SchemaVersionMismatch { expected, actual } => {
                write!(f, "expected preview scope schema version {expected}, got {actual}")
            }
        }
    }
}

impl std::error::Error for PreviewScopeLabelLoadError {}
