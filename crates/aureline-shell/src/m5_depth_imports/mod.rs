//! Migration-center coverage, importer diffs, and compatibility truth for the
//! M5 depth-import artifact families.
//!
//! The stable v1 migration center taught switching users to trust three things
//! before they commit: a before/after **diff** shown before apply, a **restore
//! checkpoint** that protects every mutated domain, and an explicit
//! **outcome + compatibility note** for every row rather than a generic
//! best-effort success banner. This module carries that same contract forward
//! into the new M5 depth lanes — notebook handoff artifacts, request and schema
//! bundles, database query/session exports, profiler or trace captures, signed
//! template or scaffold manifests, and companion/export packets — so that
//! adopting an adjacent tool's artifacts becomes a diff-first, checkpointed,
//! supportable story instead of a guess.
//!
//! Each artifact family is projected as one [`DepthImportRow`] that reuses the
//! canonical six-state importer vocabulary
//! ([`InteropOutcome`]: `imported` / `mapped` / `skipped` / `manual_review` /
//! `bridge_required` / `unsupported`) and the canonical
//! [`ImportMappingClassification`] fidelity ladder
//! (`exact` / `translated` / `partial` / `shimmed` / `unsupported`) from
//! [`crate::import::diff_review`], rather than inventing a per-feature import
//! dialog. Every row also pins:
//!
//! - a disclosed **continuity scope** ([`ContinuityScope`]) so a row that only
//!   supports inspect-only, partial, bridge-based, or export-bundle continuity
//!   can never be marketed as native parity;
//! - a **restore checkpoint** and **restore path** whenever the apply mutates
//!   durable state, so a partial or lossy apply is always reversible;
//! - a **compatibility note** and known deviations for any non-native scope, so
//!   docs, help, and support exports ingest the same truth instead of rephrasing
//!   it; and
//! - the bridge requirement details for any row that depends on a bridge for
//!   continuity.
//!
//! The records are inspectable, serde-serializable truth packets that carry no
//! credential bodies, raw provider payloads, file paths, or artifact content.
//! They are consumed by the live migration center, the headless inspector
//! (`aureline_shell_m5_depth_imports`), the support-export wrapper, the docs
//! page under `docs/m5/migration-depth-lanes.md`, the compatibility report under
//! `artifacts/compat/m5/migration-reports/`, and the boundary schema
//! `schemas/migration/m5-depth-import.schema.json`. The seeded projection is
//! deterministic so the checked-in fixtures under `fixtures/migration/m5_depth/`
//! are bit-for-bit equal to the output of [`seeded_m5_depth_import_report`].

use serde::{Deserialize, Serialize};

use crate::import::diff_review::ImportMappingClassification;

#[cfg(test)]
mod tests;

/// Schema version exported with every record.
pub const M5_DEPTH_IMPORT_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by migration center, CLI, docs, and support.
pub const M5_DEPTH_IMPORT_SHARED_CONTRACT_REF: &str = "migration:m5_depth_import:v1";

/// Stable record kind for [`DepthImportReport`] payloads.
pub const M5_DEPTH_IMPORT_REPORT_RECORD_KIND: &str = "m5_depth_import_report_record";

/// Stable record kind for [`DepthImportRow`] payloads.
pub const M5_DEPTH_IMPORT_ROW_RECORD_KIND: &str = "m5_depth_import_row_record";

/// Stable record kind for [`DepthImportSupportExport`] payloads.
pub const M5_DEPTH_IMPORT_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_depth_import_support_export_record";

/// Stable report id used to pivot across surfaces.
pub const M5_DEPTH_IMPORT_REPORT_ID: &str = "migration:m5_depth_import:v1:default";

/// Deterministic generated-at value carried by the seeded report.
const GENERATED_AT: &str = "2026-06-11T00:00:00Z";

/// One M5-adjacent artifact family the migration center can preview, map, and
/// — where supported — import into Aureline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DepthArtifactClass {
    /// Notebook handoff artifacts (cells, outputs, attachments).
    NotebookHandoff,
    /// Request and schema bundles (saved requests, collections, API schemas).
    RequestSchemaBundle,
    /// Database query and session exports.
    DatabaseQuerySessionExport,
    /// Profiler or trace captures.
    ProfilerTraceCapture,
    /// Signed template or scaffold manifests.
    TemplateScaffoldManifest,
    /// Companion or export packets users treat as switching aids.
    CompanionExportPacket,
}

impl DepthArtifactClass {
    /// Returns the stable schema token for this artifact class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookHandoff => "notebook_handoff",
            Self::RequestSchemaBundle => "request_schema_bundle",
            Self::DatabaseQuerySessionExport => "database_query_session_export",
            Self::ProfilerTraceCapture => "profiler_trace_capture",
            Self::TemplateScaffoldManifest => "template_scaffold_manifest",
            Self::CompanionExportPacket => "companion_export_packet",
        }
    }

    /// Returns the reviewer-facing label for this artifact class.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::NotebookHandoff => "Notebook handoff",
            Self::RequestSchemaBundle => "Request / schema bundle",
            Self::DatabaseQuerySessionExport => "Database query / session export",
            Self::ProfilerTraceCapture => "Profiler / trace capture",
            Self::TemplateScaffoldManifest => "Template / scaffold manifest",
            Self::CompanionExportPacket => "Companion / export packet",
        }
    }

    /// Returns every required artifact class in canonical order.
    pub const fn required_classes() -> [Self; 6] {
        [
            Self::NotebookHandoff,
            Self::RequestSchemaBundle,
            Self::DatabaseQuerySessionExport,
            Self::ProfilerTraceCapture,
            Self::TemplateScaffoldManifest,
            Self::CompanionExportPacket,
        ]
    }
}

/// Controlled importer outcome vocabulary, identical to the migration-center
/// `interop_result_state` set so docs, support exports, and issue templates can
/// describe the same truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteropOutcome {
    /// The source artifact was imported into a native target object.
    Imported,
    /// The source artifact was mapped onto a declared target object or command.
    Mapped,
    /// The user or policy declined the import; existing state was retained.
    Skipped,
    /// The row needs explicit human review before it can apply.
    ManualReview,
    /// Continuity depends on a bridge, shim, or compatibility layer.
    BridgeRequired,
    /// No safe target exists for this source artifact.
    Unsupported,
}

impl InteropOutcome {
    /// Returns the stable schema token for this outcome.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Imported => "imported",
            Self::Mapped => "mapped",
            Self::Skipped => "skipped",
            Self::ManualReview => "manual_review",
            Self::BridgeRequired => "bridge_required",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Disclosed continuity scope for an artifact family. This is the explicit
/// support posture that keeps a non-native lane from being marketed as parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityScope {
    /// Native parity: the artifact applies into a first-class Aureline object.
    Native,
    /// Partial mapping: a subset applies, with visible caveats retained.
    PartialMapping,
    /// Inspect-only: the artifact can be opened and reviewed but not applied.
    InspectOnly,
    /// Bridge-based: continuity depends on a bridge or compatibility layer.
    Bridge,
    /// Export-bundle: continuity is preserved as a re-exportable bundle, not a
    /// native import.
    ExportBundle,
    /// Unsupported: Aureline does not claim continuity for this artifact family.
    Unsupported,
}

impl ContinuityScope {
    /// Returns the stable schema token for this continuity scope.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::PartialMapping => "partial_mapping",
            Self::InspectOnly => "inspect_only",
            Self::Bridge => "bridge",
            Self::ExportBundle => "export_bundle",
            Self::Unsupported => "unsupported",
        }
    }

    /// Returns `true` when this scope claims native parity.
    pub const fn claims_native_parity(self) -> bool {
        matches!(self, Self::Native)
    }
}

/// Bridge family a row depends on when it cannot claim native parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BridgeSurfaceClass {
    /// A kernel, runtime, or execution bridge.
    RuntimeBridge,
    /// A request/transport translation bridge.
    RequestTranslator,
    /// A schema or contract translation bridge.
    SchemaTranslator,
    /// A capability-layer compatibility bridge.
    CapabilityLayer,
    /// Some other declared bridge family.
    Other,
}

impl BridgeSurfaceClass {
    /// Returns the stable schema token for this bridge family.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeBridge => "runtime_bridge",
            Self::RequestTranslator => "request_translator",
            Self::SchemaTranslator => "schema_translator",
            Self::CapabilityLayer => "capability_layer",
            Self::Other => "other",
        }
    }
}

/// Whether the required bridge is present, still needed, blocked, or absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BridgeInstallationPosture {
    /// The bridge is already present.
    AlreadyPresent,
    /// The bridge is required before the artifact can be used.
    RequiredBeforeUse,
    /// The bridge is recommended but not required.
    Recommended,
    /// Policy blocks installing the bridge.
    PolicyBlocked,
    /// No bridge is available for this artifact family.
    Unavailable,
}

impl BridgeInstallationPosture {
    /// Returns the stable schema token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AlreadyPresent => "already_present",
            Self::RequiredBeforeUse => "required_before_use",
            Self::Recommended => "recommended",
            Self::PolicyBlocked => "policy_blocked",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Bridge requirement details for a row that cannot claim native parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeRequirement {
    /// Stable bridge ref (no credential body, no raw payload).
    pub bridge_ref: String,
    /// Bridge family the row depends on.
    pub bridge_surface_class: BridgeSurfaceClass,
    /// Whether the bridge is present, required, blocked, or unavailable.
    pub installation_posture: BridgeInstallationPosture,
    /// Reviewer-facing note explaining why native parity is not claimed.
    pub note: String,
}

/// A disclosed deviation from native parity for an artifact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnownDeviation {
    /// Stable deviation id quoted across surfaces.
    pub deviation_id: String,
    /// Reviewer-facing summary of what does not come across natively.
    pub summary: String,
    /// True when the deviation is recoverable by a documented follow-up.
    pub recoverable: bool,
}

/// The restore checkpoint and restore path that protect a mutating apply.
///
/// A row that mutates durable state must carry one of these so that a partial,
/// lossy, or blocked apply is always reversible rather than disappearing into a
/// generic success banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreCheckpoint {
    /// Stable checkpoint id minted before apply.
    pub checkpoint_id: String,
    /// Reviewer-facing description of the restore path.
    pub restore_path: String,
    /// Domains protected by the checkpoint.
    pub protected_domains: Vec<String>,
}

/// One M5-adjacent artifact family projected as an importer-diff row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepthImportRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable row id quoted across surfaces.
    pub row_id: String,
    /// Artifact family the row covers.
    pub artifact_class: DepthArtifactClass,
    /// Reviewer-facing title for the row.
    pub title: String,
    /// Opaque source artifact ref (no path, no content).
    pub source_artifact_ref: String,
    /// Resolved target object or recommended destination, when one exists.
    pub target_object_ref: Option<String>,
    /// Controlled importer outcome.
    pub outcome: InteropOutcome,
    /// Per-row fidelity classification on the canonical ladder.
    pub fidelity: ImportMappingClassification,
    /// Disclosed continuity scope; never widened beyond what is proven.
    pub continuity_scope: ContinuityScope,
    /// True when applying the row mutates durable state.
    pub mutates_durable_state: bool,
    /// Restore checkpoint protecting a mutating apply. Required whenever
    /// `mutates_durable_state` is true.
    pub restore_checkpoint: Option<RestoreCheckpoint>,
    /// Bridge requirement details. Required when `outcome` is
    /// [`InteropOutcome::BridgeRequired`].
    pub bridge_requirement: Option<BridgeRequirement>,
    /// Reviewer-facing compatibility note. Required for any non-native scope.
    pub compatibility_note: Option<String>,
    /// Known deviations from native parity disclosed before apply.
    pub known_deviations: Vec<KnownDeviation>,
    /// Docs/help refs that publish the row.
    pub docs_help_refs: Vec<String>,
    /// Reviewer-facing narrative summary.
    pub narrative: String,
}

impl DepthImportRow {
    /// Returns `true` when the row is shown with a reversible apply: either it
    /// does not mutate durable state, or it carries a restore checkpoint.
    pub fn apply_is_reversible(&self) -> bool {
        !self.mutates_durable_state || self.restore_checkpoint.is_some()
    }

    /// Returns deterministic compact rows for text review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("{} [{}]", self.title, self.artifact_class.as_str()),
            format!(
                "  outcome={} fidelity={} scope={}",
                self.outcome.as_str(),
                self.fidelity.as_str(),
                self.continuity_scope.as_str()
            ),
            format!(
                "  mutates_durable_state={} reversible={}",
                self.mutates_durable_state,
                self.apply_is_reversible()
            ),
        ];
        if let Some(checkpoint) = &self.restore_checkpoint {
            lines.push(format!("  restore: {}", checkpoint.restore_path));
        }
        if let Some(note) = &self.compatibility_note {
            lines.push(format!("  compatibility_note: {note}"));
        }
        for deviation in &self.known_deviations {
            lines.push(format!("  deviation: {}", deviation.summary));
        }
        lines
    }
}

/// Explicit grouped counts for the six controlled outcome states.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutcomeSummary {
    /// Number of `imported` rows.
    pub imported: usize,
    /// Number of `mapped` rows.
    pub mapped: usize,
    /// Number of `skipped` rows.
    pub skipped: usize,
    /// Number of `manual_review` rows.
    pub manual_review: usize,
    /// Number of `bridge_required` rows.
    pub bridge_required: usize,
    /// Number of `unsupported` rows.
    pub unsupported: usize,
    /// Total number of rows.
    pub total_rows: usize,
}

impl OutcomeSummary {
    fn from_rows(rows: &[DepthImportRow]) -> Self {
        let mut summary = Self {
            imported: 0,
            mapped: 0,
            skipped: 0,
            manual_review: 0,
            bridge_required: 0,
            unsupported: 0,
            total_rows: rows.len(),
        };
        for row in rows {
            match row.outcome {
                InteropOutcome::Imported => summary.imported += 1,
                InteropOutcome::Mapped => summary.mapped += 1,
                InteropOutcome::Skipped => summary.skipped += 1,
                InteropOutcome::ManualReview => summary.manual_review += 1,
                InteropOutcome::BridgeRequired => summary.bridge_required += 1,
                InteropOutcome::Unsupported => summary.unsupported += 1,
            }
        }
        summary
    }
}

/// Artifact-class coverage summary across the report's rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassCoverageSummary {
    /// Artifact classes covered by the report, in canonical order.
    pub covered_classes: Vec<DepthArtifactClass>,
    /// Total number of required artifact classes.
    pub total_required_classes: usize,
    /// Number of covered classes that claim native parity.
    pub native_parity_classes: usize,
}

impl ClassCoverageSummary {
    fn from_rows(rows: &[DepthImportRow]) -> Self {
        let mut covered_classes = Vec::new();
        let mut native_parity_classes = 0;
        for class in DepthArtifactClass::required_classes() {
            if let Some(row) = rows.iter().find(|row| row.artifact_class == class) {
                covered_classes.push(class);
                if row.continuity_scope.claims_native_parity() {
                    native_parity_classes += 1;
                }
            }
        }
        Self {
            covered_classes,
            total_required_classes: DepthArtifactClass::required_classes().len(),
            native_parity_classes,
        }
    }

    /// Returns `true` when every required artifact class is covered.
    pub fn covers_every_class(&self) -> bool {
        self.covered_classes.len() == self.total_required_classes
    }
}

/// Migration-center compatibility report for the M5 depth-import lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepthImportReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the report.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable report id used to pivot across surfaces.
    pub report_id: String,
    /// Reviewer-facing summary line printed above the rows.
    pub headline: String,
    /// True when the migration center previews every row's diff before apply.
    pub preview_before_apply: bool,
    /// Importer-diff rows in canonical artifact-class order.
    pub rows: Vec<DepthImportRow>,
    /// Grouped outcome counts.
    pub outcome_summary: OutcomeSummary,
    /// Artifact-class coverage summary.
    pub class_coverage: ClassCoverageSummary,
    /// True when no row mutates durable state without a restore checkpoint.
    pub every_apply_reversible: bool,
    /// True when no non-native row is marketed as native parity.
    pub no_overclaimed_parity: bool,
    /// True when no record carries raw artifact content or credential bodies.
    pub no_raw_artifact_content: bool,
    /// Compatibility-report refs published downstream.
    pub compatibility_report_refs: Vec<String>,
    /// Readiness review refs that consume the report.
    pub readiness_review_refs: Vec<String>,
    /// Docs/help refs the report reopens from.
    pub docs_help_refs: Vec<String>,
    /// Stable export refs carrying the report into support and offline review.
    pub export_refs: Vec<String>,
    /// Support packet refs that preserve the report.
    pub support_packet_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl DepthImportReport {
    /// Returns the row count for the report.
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns `true` when every required artifact class is covered.
    pub fn covers_every_class(&self) -> bool {
        self.class_coverage.covers_every_class()
    }

    /// Returns compact text lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "report: id={}, rows={}, classes={}/{}",
            self.report_id,
            self.rows.len(),
            self.class_coverage.covered_classes.len(),
            self.class_coverage.total_required_classes,
        ));
        lines.push(format!(
            "preview_before_apply={} every_apply_reversible={} no_overclaimed_parity={}",
            self.preview_before_apply, self.every_apply_reversible, self.no_overclaimed_parity
        ));
        for row in &self.rows {
            lines.extend(row.compact_lines());
        }
        lines
    }

    /// Renders the markdown compatibility report for the lane.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 depth-import migration & compatibility report\n\n");
        out.push_str(
            "Generated from the seeded report in\n\
             [`crate::m5_depth_imports`](../../../../crates/aureline-shell/src/m5_depth_imports/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_depth_imports -- markdown > \\\n  artifacts/compat/m5/migration-reports/m5_depth_import_report.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!("- Rows: {}\n", self.rows.len()));
        out.push_str(&format!(
            "- Artifact classes covered: {}/{}\n",
            self.class_coverage.covered_classes.len(),
            self.class_coverage.total_required_classes
        ));
        out.push_str(&format!(
            "- Preview before apply: {}\n",
            self.preview_before_apply
        ));
        out.push_str(&format!(
            "- Every apply reversible: {}\n",
            self.every_apply_reversible
        ));
        out.push_str(&format!(
            "- No overclaimed parity: {}\n",
            self.no_overclaimed_parity
        ));
        out.push_str(&format!(
            "- No raw artifact content: {}\n",
            self.no_raw_artifact_content
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Outcome summary\n\n");
        out.push_str("| Outcome | Count |\n|---|---:|\n");
        out.push_str(&format!(
            "| imported | {} |\n",
            self.outcome_summary.imported
        ));
        out.push_str(&format!("| mapped | {} |\n", self.outcome_summary.mapped));
        out.push_str(&format!("| skipped | {} |\n", self.outcome_summary.skipped));
        out.push_str(&format!(
            "| manual_review | {} |\n",
            self.outcome_summary.manual_review
        ));
        out.push_str(&format!(
            "| bridge_required | {} |\n",
            self.outcome_summary.bridge_required
        ));
        out.push_str(&format!(
            "| unsupported | {} |\n",
            self.outcome_summary.unsupported
        ));
        out.push_str(&format!(
            "| **total** | **{}** |\n\n",
            self.outcome_summary.total_rows
        ));

        out.push_str("## Artifact-class coverage\n\n");
        out.push_str("| Artifact class | Outcome | Fidelity | Continuity scope | Reversible |\n");
        out.push_str("|---|---|---|---|:---:|\n");
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | {} |\n",
                row.artifact_class.display_label(),
                row.outcome.as_str(),
                row.fidelity.as_str(),
                row.continuity_scope.as_str(),
                if row.apply_is_reversible() {
                    "yes"
                } else {
                    "NO"
                },
            ));
        }
        out.push('\n');

        for row in &self.rows {
            out.push_str(&format!(
                "## {} (`{}`)\n\n",
                row.title,
                row.artifact_class.as_str()
            ));
            out.push_str(&format!("{}\n\n", row.narrative));
            out.push_str(&format!("- Outcome: `{}`\n", row.outcome.as_str()));
            out.push_str(&format!(
                "- Continuity scope: `{}`\n",
                row.continuity_scope.as_str()
            ));
            if let Some(note) = &row.compatibility_note {
                out.push_str(&format!("- Compatibility note: {note}\n"));
            }
            if let Some(checkpoint) = &row.restore_checkpoint {
                out.push_str(&format!("- Restore path: {}\n", checkpoint.restore_path));
            }
            if let Some(bridge) = &row.bridge_requirement {
                out.push_str(&format!(
                    "- Bridge: `{}` ({}) — {}\n",
                    bridge.bridge_ref,
                    bridge.installation_posture.as_str(),
                    bridge.note
                ));
            }
            if !row.known_deviations.is_empty() {
                out.push_str("- Known deviations:\n");
                for deviation in &row.known_deviations {
                    out.push_str(&format!(
                        "  - `{}` — {} (recoverable: {})\n",
                        deviation.deviation_id, deviation.summary, deviation.recoverable
                    ));
                }
            }
            out.push('\n');
        }

        out
    }
}

/// Support-export wrapper that quotes the report plus every stable id reviewers
/// need to pivot across surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepthImportSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the wrapper.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Report quoted in full.
    pub report: DepthImportReport,
    /// Stable report id, row ids, and restore-checkpoint ids in deterministic
    /// order.
    pub case_ids: Vec<String>,
}

impl DepthImportSupportExport {
    /// Builds the support-export wrapper for a report.
    pub fn from_report(support_export_id: impl Into<String>, report: DepthImportReport) -> Self {
        let mut case_ids = Vec::new();
        case_ids.push(report.report_id.clone());
        for row in &report.rows {
            case_ids.push(row.row_id.clone());
        }
        for row in &report.rows {
            if let Some(checkpoint) = &row.restore_checkpoint {
                case_ids.push(checkpoint.checkpoint_id.clone());
            }
        }
        Self {
            record_kind: M5_DEPTH_IMPORT_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_DEPTH_IMPORT_SCHEMA_VERSION,
            shared_contract_ref: M5_DEPTH_IMPORT_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Validation error produced by [`validate_m5_depth_import_report`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum DepthImportValidationError {
    /// A required artifact class has no row in the report.
    MissingArtifactClass {
        /// Artifact class with the missing row.
        artifact_class: String,
    },
    /// The report does not preview the diff before apply.
    PreviewBeforeApplyMissing,
    /// A row mutates durable state without a restore checkpoint.
    MutatingApplyWithoutCheckpoint {
        /// Row that violated the invariant.
        row_id: String,
    },
    /// A non-native row was marketed as native parity, or a native row carries
    /// an outcome/fidelity that does not support a parity claim.
    NativeParityOverclaimed {
        /// Row that violated the invariant.
        row_id: String,
    },
    /// A bridge_required row carries no bridge requirement.
    BridgeRequirementMissing {
        /// Row that violated the invariant.
        row_id: String,
    },
    /// A non-native row carries no compatibility note.
    CompatibilityNoteMissing {
        /// Row that violated the invariant.
        row_id: String,
    },
    /// A row carries no docs/help ref.
    DocsHelpRefMissing {
        /// Row that violated the invariant.
        row_id: String,
    },
    /// A report containing bridge_required or unsupported rows omits export or
    /// support refs.
    EscalationRefsMissing,
    /// The outcome summary does not match the rows.
    OutcomeSummaryStale,
    /// The class coverage summary does not match the rows.
    ClassCoverageStale,
    /// The report does not declare a compatibility-report ref.
    CompatibilityReportRefMissing,
    /// The report does not declare a readiness review ref.
    ReadinessReviewMissing,
}

/// Validates a report against the M5 depth-import acceptance invariants.
///
/// The checks encode the migration guardrails: every mutating apply is
/// checkpointed, no non-native lane is marketed as parity, every non-native row
/// discloses a compatibility note, and any report with bridge or unsupported
/// rows carries the export and support refs that escalation surfaces consume.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_m5_depth_import_report(
    report: &DepthImportReport,
) -> Result<(), Vec<DepthImportValidationError>> {
    let mut errors = Vec::new();

    let outcome_summary = OutcomeSummary::from_rows(&report.rows);
    if outcome_summary != report.outcome_summary {
        errors.push(DepthImportValidationError::OutcomeSummaryStale);
    }

    let class_coverage = ClassCoverageSummary::from_rows(&report.rows);
    if class_coverage != report.class_coverage {
        errors.push(DepthImportValidationError::ClassCoverageStale);
    }

    for class in DepthArtifactClass::required_classes() {
        if !report.rows.iter().any(|row| row.artifact_class == class) {
            errors.push(DepthImportValidationError::MissingArtifactClass {
                artifact_class: class.as_str().to_owned(),
            });
        }
    }

    if !report.preview_before_apply {
        errors.push(DepthImportValidationError::PreviewBeforeApplyMissing);
    }

    let mut has_escalation_row = false;
    for row in &report.rows {
        if row.mutates_durable_state && row.restore_checkpoint.is_none() {
            errors.push(DepthImportValidationError::MutatingApplyWithoutCheckpoint {
                row_id: row.row_id.clone(),
            });
        }

        // A native-parity claim must be backed by an imported/mapped outcome on
        // the exact/translated fidelity rungs. Any weaker outcome or fidelity,
        // or a bridge/unsupported outcome, may never present as native parity.
        if row.continuity_scope.claims_native_parity() {
            let outcome_supports_parity = matches!(
                row.outcome,
                InteropOutcome::Imported | InteropOutcome::Mapped
            );
            let fidelity_supports_parity = matches!(
                row.fidelity,
                ImportMappingClassification::Exact | ImportMappingClassification::Translated
            );
            if !(outcome_supports_parity && fidelity_supports_parity) {
                errors.push(DepthImportValidationError::NativeParityOverclaimed {
                    row_id: row.row_id.clone(),
                });
            }
        } else if row.compatibility_note.is_none() {
            errors.push(DepthImportValidationError::CompatibilityNoteMissing {
                row_id: row.row_id.clone(),
            });
        }

        if matches!(row.outcome, InteropOutcome::BridgeRequired) {
            if row.bridge_requirement.is_none() {
                errors.push(DepthImportValidationError::BridgeRequirementMissing {
                    row_id: row.row_id.clone(),
                });
            }
            has_escalation_row = true;
        }
        if matches!(row.outcome, InteropOutcome::Unsupported) {
            has_escalation_row = true;
        }

        if row.docs_help_refs.is_empty() {
            errors.push(DepthImportValidationError::DocsHelpRefMissing {
                row_id: row.row_id.clone(),
            });
        }
    }

    if has_escalation_row
        && (report.export_refs.is_empty() || report.support_packet_refs.is_empty())
    {
        errors.push(DepthImportValidationError::EscalationRefsMissing);
    }

    if report.compatibility_report_refs.is_empty() {
        errors.push(DepthImportValidationError::CompatibilityReportRefMissing);
    }
    if report.readiness_review_refs.is_empty() {
        errors.push(DepthImportValidationError::ReadinessReviewMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Builds the seeded M5 depth-import compatibility report.
pub fn seeded_m5_depth_import_report() -> DepthImportReport {
    let rows = seeded_rows();
    let outcome_summary = OutcomeSummary::from_rows(&rows);
    let class_coverage = ClassCoverageSummary::from_rows(&rows);
    let every_apply_reversible = rows.iter().all(DepthImportRow::apply_is_reversible);
    let no_overclaimed_parity = rows.iter().all(|row| {
        !row.continuity_scope.claims_native_parity()
            || (matches!(
                row.outcome,
                InteropOutcome::Imported | InteropOutcome::Mapped
            ) && matches!(
                row.fidelity,
                ImportMappingClassification::Exact | ImportMappingClassification::Translated
            ))
    });

    DepthImportReport {
        record_kind: M5_DEPTH_IMPORT_REPORT_RECORD_KIND.to_owned(),
        schema_version: M5_DEPTH_IMPORT_SCHEMA_VERSION,
        shared_contract_ref: M5_DEPTH_IMPORT_SHARED_CONTRACT_REF.to_owned(),
        report_id: M5_DEPTH_IMPORT_REPORT_ID.to_owned(),
        headline: "Diff-first, checkpointed migration coverage and compatibility truth for the M5 \
             depth-import artifact families."
            .to_owned(),
        preview_before_apply: true,
        rows,
        outcome_summary,
        class_coverage,
        every_apply_reversible,
        no_overclaimed_parity,
        no_raw_artifact_content: true,
        compatibility_report_refs: vec![
            "artifacts/compat/m5/migration-reports/m5_depth_import_report.md".to_owned(),
            "docs/release/compatibility_report_template.md#migration.m5_depth_imports".to_owned(),
        ],
        readiness_review_refs: vec![
            "readiness-review:m5:depth_import_compatibility".to_owned(),
            "readiness-review:m5:switching_continuity".to_owned(),
        ],
        docs_help_refs: vec![
            "docs/m5/migration-depth-lanes.md".to_owned(),
            "docs/migration/migration_center_object_model.md".to_owned(),
        ],
        export_refs: vec![
            "support_bundle.m5_depth_import_report".to_owned(),
            "migration_center.machine_readable_report".to_owned(),
        ],
        support_packet_refs: vec!["support_packet.m5_depth_import_review.default".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

struct RowSeed {
    row_id: &'static str,
    artifact_class: DepthArtifactClass,
    title: &'static str,
    source_artifact_ref: &'static str,
    target_object_ref: Option<&'static str>,
    outcome: InteropOutcome,
    fidelity: ImportMappingClassification,
    continuity_scope: ContinuityScope,
    mutates_durable_state: bool,
    restore_checkpoint: Option<(&'static str, &'static str, &'static [&'static str])>,
    bridge_requirement: Option<(
        &'static str,
        BridgeSurfaceClass,
        BridgeInstallationPosture,
        &'static str,
    )>,
    compatibility_note: Option<&'static str>,
    known_deviations: &'static [(&'static str, &'static str, bool)],
    docs_help_refs: &'static [&'static str],
    narrative: &'static str,
}

fn build_row(seed: &RowSeed) -> DepthImportRow {
    DepthImportRow {
        record_kind: M5_DEPTH_IMPORT_ROW_RECORD_KIND.to_owned(),
        schema_version: M5_DEPTH_IMPORT_SCHEMA_VERSION,
        shared_contract_ref: M5_DEPTH_IMPORT_SHARED_CONTRACT_REF.to_owned(),
        row_id: seed.row_id.to_owned(),
        artifact_class: seed.artifact_class,
        title: seed.title.to_owned(),
        source_artifact_ref: seed.source_artifact_ref.to_owned(),
        target_object_ref: seed.target_object_ref.map(str::to_owned),
        outcome: seed.outcome,
        fidelity: seed.fidelity,
        continuity_scope: seed.continuity_scope,
        mutates_durable_state: seed.mutates_durable_state,
        restore_checkpoint: seed
            .restore_checkpoint
            .map(|(id, path, domains)| RestoreCheckpoint {
                checkpoint_id: id.to_owned(),
                restore_path: path.to_owned(),
                protected_domains: domains.iter().map(|d| (*d).to_owned()).collect(),
            }),
        bridge_requirement: seed.bridge_requirement.map(|(r, surface, posture, note)| {
            BridgeRequirement {
                bridge_ref: r.to_owned(),
                bridge_surface_class: surface,
                installation_posture: posture,
                note: note.to_owned(),
            }
        }),
        compatibility_note: seed.compatibility_note.map(str::to_owned),
        known_deviations: seed
            .known_deviations
            .iter()
            .map(|(id, summary, recoverable)| KnownDeviation {
                deviation_id: (*id).to_owned(),
                summary: (*summary).to_owned(),
                recoverable: *recoverable,
            })
            .collect(),
        docs_help_refs: seed
            .docs_help_refs
            .iter()
            .map(|r| (*r).to_owned())
            .collect(),
        narrative: seed.narrative.to_owned(),
    }
}

fn seeded_rows() -> Vec<DepthImportRow> {
    const ROW_SEEDS: &[RowSeed] = &[
        RowSeed {
            row_id: "m5-depth-import-row:notebook-handoff",
            artifact_class: DepthArtifactClass::NotebookHandoff,
            title: "Notebook handoff import",
            source_artifact_ref: "source:notebook_handoff_bundle",
            target_object_ref: Some("aureline:notebook:imported_document"),
            outcome: InteropOutcome::Imported,
            fidelity: ImportMappingClassification::Translated,
            continuity_scope: ContinuityScope::Native,
            mutates_durable_state: true,
            restore_checkpoint: Some((
                "checkpoint:notebook-handoff",
                "Restore from the pre-apply notebook checkpoint to revert the imported document.",
                &["notebook_documents"],
            )),
            bridge_requirement: None,
            compatibility_note: None,
            known_deviations: &[(
                "deviation:notebook.kernel_state",
                "Live kernel state and execution counts are not part of the handoff; the imported \
                 document opens without a running kernel.",
                true,
            )],
            docs_help_refs: &[
                "docs/m5/migration-depth-lanes.md#notebook-handoff",
                "docs/migration/migration_center_object_model.md#importer-outcome-row",
            ],
            narrative:
                "Notebook cells, outputs, and attachment refs translate into a native imported \
                 document. The apply is checkpointed and the imported document can be restored if \
                 the result is not what the user expected.",
        },
        RowSeed {
            row_id: "m5-depth-import-row:request-schema-bundle",
            artifact_class: DepthArtifactClass::RequestSchemaBundle,
            title: "Request and schema bundle import",
            source_artifact_ref: "source:request_schema_bundle",
            target_object_ref: Some("aureline:request_workspace:imported_collection"),
            outcome: InteropOutcome::Mapped,
            fidelity: ImportMappingClassification::Partial,
            continuity_scope: ContinuityScope::PartialMapping,
            mutates_durable_state: true,
            restore_checkpoint: Some((
                "checkpoint:request-schema-bundle",
                "Restore from the pre-apply request-workspace checkpoint to remove the mapped \
                 collection.",
                &["request_workspace", "schemas"],
            )),
            bridge_requirement: None,
            compatibility_note: Some(
                "Saved requests and the API schema map into a request workspace; environment \
                 secrets and pre-request scripts are not imported and must be re-entered.",
            ),
            known_deviations: &[(
                "deviation:request.secret_scripts",
                "Environment secrets and pre-request scripting do not cross the boundary.",
                false,
            )],
            docs_help_refs: &[
                "docs/m5/migration-depth-lanes.md#request-schema-bundle",
                "docs/migration/migration_center_object_model.md#importer-outcome-row",
            ],
            narrative:
                "Requests and schemas map onto a native request workspace, but the mapping is \
                 partial: scripts and secrets are intentionally excluded and disclosed as known \
                 deviations rather than implied to come across.",
        },
        RowSeed {
            row_id: "m5-depth-import-row:database-query-session",
            artifact_class: DepthArtifactClass::DatabaseQuerySessionExport,
            title: "Database query and session export",
            source_artifact_ref: "source:database_query_session_export",
            target_object_ref: Some("aureline:data_workspace:imported_query_library"),
            outcome: InteropOutcome::Mapped,
            fidelity: ImportMappingClassification::Partial,
            continuity_scope: ContinuityScope::PartialMapping,
            mutates_durable_state: true,
            restore_checkpoint: Some((
                "checkpoint:database-query-session",
                "Restore from the pre-apply data-workspace checkpoint to remove the imported query \
                 library.",
                &["data_workspace"],
            )),
            bridge_requirement: None,
            compatibility_note: Some(
                "Saved queries and session metadata map into a query library; live connection \
                 credentials are never imported and must be reconnected explicitly.",
            ),
            known_deviations: &[(
                "deviation:database.live_connection",
                "Live connection credentials and open result-set state do not cross the boundary.",
                true,
            )],
            docs_help_refs: &[
                "docs/m5/migration-depth-lanes.md#database-query-session-export",
                "docs/migration/migration_center_object_model.md#importer-outcome-row",
            ],
            narrative:
                "Query text and session metadata map into a native query library. Credentials are \
                 excluded by design, so the row stays a disclosed partial mapping rather than a \
                 parity claim.",
        },
        RowSeed {
            row_id: "m5-depth-import-row:profiler-trace-capture",
            artifact_class: DepthArtifactClass::ProfilerTraceCapture,
            title: "Profiler or trace capture import",
            source_artifact_ref: "source:profiler_trace_capture",
            target_object_ref: Some("aureline:profiler:inspected_capture"),
            outcome: InteropOutcome::ManualReview,
            fidelity: ImportMappingClassification::Partial,
            continuity_scope: ContinuityScope::InspectOnly,
            mutates_durable_state: false,
            restore_checkpoint: None,
            bridge_requirement: None,
            compatibility_note: Some(
                "Captured traces open in an inspect-only viewer; Aureline does not re-run or \
                 re-symbolicate the capture, so the import does not mutate durable state.",
            ),
            known_deviations: &[(
                "deviation:profiler.resymbolication",
                "Re-symbolication and re-capture are not performed; symbols already present in the \
                 capture are shown as-is.",
                false,
            )],
            docs_help_refs: &[
                "docs/m5/migration-depth-lanes.md#profiler-trace-capture",
                "docs/migration/migration_center_object_model.md#importer-outcome-row",
            ],
            narrative:
                "A captured trace is opened inspect-only for review. Because nothing durable is \
                 written, there is no checkpoint to restore; the row is flagged for manual review \
                 so the user decides whether to keep the inspected capture.",
        },
        RowSeed {
            row_id: "m5-depth-import-row:template-scaffold-manifest",
            artifact_class: DepthArtifactClass::TemplateScaffoldManifest,
            title: "Signed template or scaffold manifest import",
            source_artifact_ref: "source:template_scaffold_manifest",
            target_object_ref: Some("aureline:templates:bridged_scaffold"),
            outcome: InteropOutcome::BridgeRequired,
            fidelity: ImportMappingClassification::Shimmed,
            continuity_scope: ContinuityScope::Bridge,
            mutates_durable_state: false,
            restore_checkpoint: None,
            bridge_requirement: Some((
                "bridge:scaffold-manifest-runner",
                BridgeSurfaceClass::CapabilityLayer,
                BridgeInstallationPosture::RequiredBeforeUse,
                "Scaffold execution runs through the compatibility bridge; the manifest signature \
                 is verified before any scaffold action is offered.",
            )),
            compatibility_note: Some(
                "Signed scaffold manifests run through a compatibility bridge rather than as a \
                 native scaffold; the bridge must be present before the scaffold can execute.",
            ),
            known_deviations: &[(
                "deviation:template.native_scaffold",
                "There is no native scaffold engine for this manifest format; continuity depends \
                 on the bridge remaining available.",
                true,
            )],
            docs_help_refs: &[
                "docs/m5/migration-depth-lanes.md#template-scaffold-manifest",
                "docs/migration/migration_center_object_model.md#bridge-required",
            ],
            narrative:
                "A signed scaffold manifest is honored through a verified compatibility bridge. \
                 The bridge posture is disclosed so the row is never presented as native scaffold \
                 parity.",
        },
        RowSeed {
            row_id: "m5-depth-import-row:companion-export-packet",
            artifact_class: DepthArtifactClass::CompanionExportPacket,
            title: "Companion or export packet import",
            source_artifact_ref: "source:companion_export_packet",
            target_object_ref: None,
            outcome: InteropOutcome::Unsupported,
            fidelity: ImportMappingClassification::Unsupported,
            continuity_scope: ContinuityScope::ExportBundle,
            mutates_durable_state: false,
            restore_checkpoint: None,
            bridge_requirement: None,
            compatibility_note: Some(
                "Companion export packets are preserved as a re-exportable bundle for offline \
                 review; Aureline does not import them as a live companion session, so the bundle \
                 is the supported continuity path.",
            ),
            known_deviations: &[(
                "deviation:companion.live_session",
                "Live companion device sessions are not reconstructed; only the export bundle is \
                 retained.",
                false,
            )],
            docs_help_refs: &[
                "docs/m5/migration-depth-lanes.md#companion-export-packet",
                "docs/migration/migration_center_object_model.md#unsupported",
            ],
            narrative:
                "Companion export packets have no native import target, so the migration center \
                 keeps them as an export bundle and says so plainly instead of implying a live \
                 companion import.",
        },
    ];

    ROW_SEEDS.iter().map(build_row).collect()
}
