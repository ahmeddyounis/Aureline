//! Family qualification packet for notebook and data-rich promoted surfaces.
//!
//! This module owns the release packet that prevents notebooks, rich outputs,
//! data tables, result grids, chart summaries, and experiment handoff cards from
//! inheriting a Stable label from adjacent editor, runtime, debug, or task
//! qualification rows. Each row keeps document trust, kernel/runtime trust, and
//! output trust separate, then binds that posture to replay/export, review,
//! accessibility, downgrade-label, support-export, and handoff evidence.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{OwnerSignoff, StableClaimLevel};

/// Supported schema version for the checked-in qualification packet.
pub const NOTEBOOK_DATA_RICH_SURFACE_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const NOTEBOOK_DATA_RICH_SURFACE_QUALIFICATION_RECORD_KIND: &str =
    "notebook_and_data_rich_surface_qualification";

/// Repo-relative path to the checked-in packet.
pub const NOTEBOOK_DATA_RICH_SURFACE_QUALIFICATION_PATH: &str =
    "artifacts/release/m4/notebook-and-data-rich-surface-qualification.json";

/// Embedded checked-in packet JSON.
pub const NOTEBOOK_DATA_RICH_SURFACE_QUALIFICATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m4/notebook-and-data-rich-surface-qualification.json"
));

/// Notebook/data-rich surface family covered by one row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookDataSurfaceKind {
    /// Notebook document chrome, header, kernel bar, cells, and output panes.
    NotebookDocument,
    /// Notebook review, diff, clean-output, and paired-export flow.
    NotebookReviewExport,
    /// Variable explorer or dataframe-like notebook inspection surface.
    VariableExplorer,
    /// Data table or generic tabular output preview.
    DataTable,
    /// Database or query result grid.
    ResultGrid,
    /// Chart or plot summary surface.
    ChartSummary,
    /// Experiment run summary or handoff card.
    ExperimentHandoff,
    /// API or database response viewer.
    ApiDatabaseResponseViewer,
}

impl NotebookDataSurfaceKind {
    /// Every surface kind, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::NotebookDocument,
        Self::NotebookReviewExport,
        Self::VariableExplorer,
        Self::DataTable,
        Self::ResultGrid,
        Self::ChartSummary,
        Self::ExperimentHandoff,
        Self::ApiDatabaseResponseViewer,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookDocument => "notebook_document",
            Self::NotebookReviewExport => "notebook_review_export",
            Self::VariableExplorer => "variable_explorer",
            Self::DataTable => "data_table",
            Self::ResultGrid => "result_grid",
            Self::ChartSummary => "chart_summary",
            Self::ExperimentHandoff => "experiment_handoff",
            Self::ApiDatabaseResponseViewer => "api_database_response_viewer",
        }
    }

    /// True for rows that must not imply database/result-grid depth by adjacency.
    pub const fn database_depth_sensitive(self) -> bool {
        matches!(self, Self::ResultGrid | Self::ApiDatabaseResponseViewer)
    }
}

/// Trust posture recorded for each independent trust axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustPosture {
    /// Source is trusted under workspace or document policy.
    Trusted,
    /// Source is explicitly untrusted.
    Untrusted,
    /// Source combines trusted and untrusted material.
    Mixed,
    /// Source was imported from another tool or prior run.
    Imported,
    /// Source is captured evidence only.
    Captured,
    /// Source exists but is stale.
    Stale,
    /// No runtime-backed trust axis is present for this document-only row.
    NotApplicable,
}

impl TrustPosture {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::Untrusted => "untrusted",
            Self::Mixed => "mixed",
            Self::Imported => "imported",
            Self::Captured => "captured",
            Self::Stale => "stale",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when this trust posture requires visible downgrade or review state.
    pub const fn requires_visible_state(self) -> bool {
        matches!(
            self,
            Self::Untrusted | Self::Mixed | Self::Imported | Self::Captured | Self::Stale
        )
    }
}

/// Runtime or execution origin class attached to a surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionOriginClass {
    /// Opened as a document with no runtime execution claim.
    DocumentOnly,
    /// Output came from a user-run local or selected kernel/runtime.
    UserRunKernel,
    /// Output came from a managed or remote kernel.
    ManagedRuntime,
    /// Output was imported from another tool or file.
    ImportedOutput,
    /// Output was restored from captured evidence.
    CapturedEvidence,
    /// Output crosses runtime or trust boundaries.
    MixedBoundary,
}

/// Freshness class attached to output, table, chart, or handoff evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputFreshnessClass {
    /// Current live runtime output.
    Live,
    /// No kernel is selected.
    NoKernel,
    /// Kernel/runtime is disconnected.
    DisconnectedKernel,
    /// Output is stale relative to document, kernel, or target identity.
    StaleOutput,
    /// Output was imported.
    ImportedOutput,
    /// Output was captured and is evidence only.
    CapturedOutput,
}

impl OutputFreshnessClass {
    /// True when the row must visibly avoid live/rerunnable claims.
    pub const fn requires_visible_state(self) -> bool {
        !matches!(self, Self::Live)
    }
}

/// Replay and export posture proven by a surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayExportPosture {
    /// Re-running is allowed only after explicit review of kernel/runtime identity.
    RerunnableAfterReview,
    /// Export is a snapshot only and must not imply a rerun.
    SnapshotOnly,
    /// Clean-output preview is derived and reversible.
    CleanOutputPreview,
    /// Paired text/script export preserves canonical-source and derived-state truth.
    PairedExportDescriptor,
    /// Support export is redacted and metadata-only.
    SupportExportRedacted,
    /// Handoff export preserves destination, scope, freshness, and redaction.
    ScopedHandoffExport,
}

/// Review mode proven by a surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewMode {
    /// Cell-aware diff is the default when the notebook parses.
    CellAwareDiffDefault,
    /// Metadata-focused review with output awareness.
    MetadataOutputAware,
    /// Raw JSON fallback with explicit reason.
    RawJsonFallback,
    /// Snapshot or golden review for rendered output.
    SnapshotGoldenReview,
}

/// Required state label on degraded notebook/data-rich surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisibleStateLabel {
    /// No selected kernel.
    NoKernel,
    /// Kernel disconnected.
    DisconnectedKernel,
    /// Stale output.
    StaleOutput,
    /// Imported output.
    ImportedOutput,
    /// Captured output.
    CapturedOutput,
    /// Mixed-trust or cross-runtime boundary.
    MixedTrust,
    /// Safe preview only.
    SafePreview,
    /// Truncated or virtualized large output.
    LargeOutputTruncated,
    /// Not part of the Stable contract.
    PreviewNotStable,
}

/// Notebook object contracts required by stable notebook rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NotebookObjectContracts {
    /// Kernel capability descriptor is present.
    pub kernel_capability_descriptor: bool,
    /// Cell execution record is present.
    pub cell_execution_record: bool,
    /// Cell-to-debug-frame link is present or explicitly unavailable.
    pub cell_to_frame_link: bool,
    /// Variable-explorer snapshot is present.
    pub variable_explorer_snapshot: bool,
    /// Kernel restart consequence record is present.
    pub kernel_restart_consequence_record: bool,
    /// Rich outputs carry trust or sandbox state.
    pub rich_output_trust_or_sandbox_state: bool,
    /// Cell-aware diff is the default when the notebook parses.
    pub cell_aware_diff_default: bool,
    /// Fallback reasons are explicit for parse or payload mismatches.
    #[serde(default)]
    pub fallback_reasons: Vec<String>,
}

impl NotebookObjectContracts {
    fn complete(&self) -> bool {
        self.kernel_capability_descriptor
            && self.cell_execution_record
            && self.cell_to_frame_link
            && self.variable_explorer_snapshot
            && self.kernel_restart_consequence_record
            && self.rich_output_trust_or_sandbox_state
            && self.cell_aware_diff_default
            && !self.fallback_reasons.is_empty()
    }
}

/// Data-rich object contracts required by stable table/result/chart rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataRichObjectContracts {
    /// Stable row identity is present where rows exist.
    pub row_identity: bool,
    /// Virtualization or truncation truth is visible.
    pub virtualization_truncation_truth: bool,
    /// Typed copy/export review is present.
    pub typed_copy_export_review: bool,
    /// Chart summaries expose text or table access.
    pub chart_summary_access: bool,
    /// Mixed-trust downgrade is present for crossed trust/runtime boundaries.
    pub mixed_trust_downgrade: bool,
}

impl DataRichObjectContracts {
    fn complete(&self) -> bool {
        self.row_identity
            && self.virtualization_truncation_truth
            && self.typed_copy_export_review
            && self.chart_summary_access
            && self.mixed_trust_downgrade
    }
}

/// Publication and support destinations that must ingest the row label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QualificationProjection {
    /// Docs and Help pages ingest the qualification label.
    pub docs_help: bool,
    /// About and service-health surfaces ingest the qualification label.
    pub about_service_health: bool,
    /// Marketplace or catalog text ingests the qualification label.
    pub marketplace_catalog: bool,
    /// CLI/headless inspection exposes the qualification label.
    pub cli_headless_inspect: bool,
    /// Support export carries the qualification label and evidence refs.
    pub support_export: bool,
    /// Migration/replay surfaces ingest replay and output-origin posture.
    pub migration_replay: bool,
}

impl QualificationProjection {
    fn complete(&self) -> bool {
        self.docs_help
            && self.about_service_health
            && self.marketplace_catalog
            && self.cli_headless_inspect
            && self.support_export
            && self.migration_replay
    }
}

/// Cross-surface handoff truth preserved by data-rich rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandoffTruth {
    /// Destination class is preserved.
    pub destination_class: bool,
    /// Row/column, cell, artifact, or chart scope is preserved.
    pub scope: bool,
    /// Freshness is preserved.
    pub freshness: bool,
    /// Redaction mode is preserved.
    pub redaction_mode: bool,
    /// Lineage/source context is preserved.
    pub lineage: bool,
}

impl HandoffTruth {
    fn complete(&self) -> bool {
        self.destination_class
            && self.scope
            && self.freshness
            && self.redaction_mode
            && self.lineage
    }
}

/// One notebook/data-rich qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NotebookDataSurfaceRow {
    /// Stable row id.
    pub surface_id: String,
    /// Human-readable title.
    pub title: String,
    /// Surface family.
    pub surface_kind: NotebookDataSurfaceKind,
    /// Whether the promoted build exposes this surface.
    pub promoted_build_surface: bool,
    /// Claimed lifecycle label before family qualification.
    pub claim_label: StableClaimLevel,
    /// Label rendered after qualification or narrowing.
    pub displayed_label: StableClaimLevel,
    /// Stable proof packet, absent for preview-only rows.
    #[serde(default)]
    pub qualification_packet: Option<ProofPacket>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Document trust posture.
    pub document_trust: TrustPosture,
    /// Kernel/runtime trust posture.
    pub kernel_runtime_trust: TrustPosture,
    /// Output trust posture.
    pub output_trust: TrustPosture,
    /// Execution origin class.
    pub execution_origin: ExecutionOriginClass,
    /// Output freshness class.
    pub freshness: OutputFreshnessClass,
    /// Replay/export postures proven by this row.
    #[serde(default)]
    pub replay_export_posture: Vec<ReplayExportPosture>,
    /// Review modes proven by this row.
    #[serde(default)]
    pub review_modes: Vec<ReviewMode>,
    /// Visible state labels available on the surface.
    #[serde(default)]
    pub visible_state_labels: Vec<VisibleStateLabel>,
    /// Notebook contracts for notebook-bearing rows.
    #[serde(default)]
    pub notebook_contracts: Option<NotebookObjectContracts>,
    /// Data-rich contracts for table/result/chart-bearing rows.
    #[serde(default)]
    pub data_rich_contracts: Option<DataRichObjectContracts>,
    /// Publication/support projections.
    pub projection: QualificationProjection,
    /// Cross-surface handoff truth.
    pub handoff_truth: HandoffTruth,
    /// Accessibility evidence refs.
    #[serde(default)]
    pub accessibility_refs: Vec<String>,
    /// Snapshot/golden evidence refs.
    #[serde(default)]
    pub snapshot_golden_refs: Vec<String>,
    /// Support/export packet refs.
    #[serde(default)]
    pub support_export_refs: Vec<String>,
    /// Reviewable reason this row carries its posture.
    pub rationale: String,
}

impl NotebookDataSurfaceRow {
    /// True when this row renders at or above the Stable cutline.
    pub fn renders_stable(&self) -> bool {
        self.displayed_label.is_at_or_above_cutline()
    }

    /// True when the row carries a captured, current proof packet.
    pub fn has_green_packet(&self) -> bool {
        self.qualification_packet.as_ref().is_some_and(|packet| {
            packet.has_capture() && packet.slo_state == FreshnessSloState::Current
        })
    }

    /// True when the row has a visible downgrade label for stale/imported/captured states.
    pub fn has_required_visible_state(&self) -> bool {
        if self.document_trust.requires_visible_state()
            || self.kernel_runtime_trust.requires_visible_state()
            || self.output_trust.requires_visible_state()
            || self.freshness.requires_visible_state()
        {
            !self.visible_state_labels.is_empty()
        } else {
            true
        }
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NotebookDataQualificationSummary {
    /// Total promoted-build rows.
    pub promoted_surface_count: usize,
    /// Rows rendering at Stable.
    pub stable_surface_count: usize,
    /// Rows narrowed below Stable.
    pub narrowed_surface_count: usize,
    /// Stable rows with green packets.
    pub green_packet_count: usize,
    /// Rows carrying preview/labs labels.
    pub preview_or_labs_count: usize,
}

/// Canonical family qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NotebookDataRichSurfaceQualification {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Human-readable release document.
    pub release_doc_ref: String,
    /// User-facing help projection.
    pub help_doc_ref: String,
    /// JSON Schema path.
    pub schema_ref: String,
    /// Surface rows.
    pub surfaces: Vec<NotebookDataSurfaceRow>,
    /// Summary counts.
    pub summary: NotebookDataQualificationSummary,
}

impl NotebookDataRichSurfaceQualification {
    /// Returns rows rendered at Stable.
    pub fn stable_surfaces(&self) -> Vec<&NotebookDataSurfaceRow> {
        self.surfaces
            .iter()
            .filter(|surface| surface.renders_stable())
            .collect()
    }

    /// Returns rows narrowed below Stable.
    pub fn narrowed_surfaces(&self) -> Vec<&NotebookDataSurfaceRow> {
        self.surfaces
            .iter()
            .filter(|surface| !surface.renders_stable())
            .collect()
    }

    /// Recomputes summary counts from row state.
    pub fn computed_summary(&self) -> NotebookDataQualificationSummary {
        let promoted: Vec<&NotebookDataSurfaceRow> = self
            .surfaces
            .iter()
            .filter(|surface| surface.promoted_build_surface)
            .collect();
        NotebookDataQualificationSummary {
            promoted_surface_count: promoted.len(),
            stable_surface_count: promoted
                .iter()
                .filter(|surface| surface.renders_stable())
                .count(),
            narrowed_surface_count: promoted
                .iter()
                .filter(|surface| !surface.renders_stable())
                .count(),
            green_packet_count: promoted
                .iter()
                .filter(|surface| surface.renders_stable() && surface.has_green_packet())
                .count(),
            preview_or_labs_count: promoted
                .iter()
                .filter(|surface| !surface.renders_stable())
                .count(),
        }
    }

    /// Validates structural invariants that do not depend on wall-clock arithmetic.
    pub fn validate(&self) -> Vec<NotebookDataQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != NOTEBOOK_DATA_RICH_SURFACE_QUALIFICATION_SCHEMA_VERSION {
            violations.push(NotebookDataQualificationViolation::SchemaVersion {
                expected: NOTEBOOK_DATA_RICH_SURFACE_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != NOTEBOOK_DATA_RICH_SURFACE_QUALIFICATION_RECORD_KIND {
            violations.push(NotebookDataQualificationViolation::RecordKind {
                expected: NOTEBOOK_DATA_RICH_SURFACE_QUALIFICATION_RECORD_KIND.to_string(),
                actual: self.record_kind.clone(),
            });
        }

        let mut ids = BTreeSet::new();
        for surface in &self.surfaces {
            if !ids.insert(surface.surface_id.clone()) {
                violations.push(NotebookDataQualificationViolation::DuplicateSurfaceId {
                    surface_id: surface.surface_id.clone(),
                });
            }
            if surface.displayed_label.rank() > surface.claim_label.rank() {
                violations.push(
                    NotebookDataQualificationViolation::DisplayedWiderThanClaim {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.promoted_build_surface
                && surface.renders_stable()
                && !surface.has_green_packet()
            {
                violations.push(
                    NotebookDataQualificationViolation::StableSurfaceWithoutGreenPacket {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.renders_stable() && !surface.owner_signoff.signed_off {
                violations.push(
                    NotebookDataQualificationViolation::StableSurfaceMissingOwnerSignoff {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.renders_stable() && !surface.has_required_visible_state() {
                violations.push(
                    NotebookDataQualificationViolation::MissingVisibleDowngradeState {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.renders_stable() && !surface.projection.complete() {
                violations.push(NotebookDataQualificationViolation::IncompleteProjection {
                    surface_id: surface.surface_id.clone(),
                });
            }
            if surface.renders_stable() && !surface.handoff_truth.complete() {
                violations.push(NotebookDataQualificationViolation::IncompleteHandoffTruth {
                    surface_id: surface.surface_id.clone(),
                });
            }
            if surface.renders_stable()
                && (surface.replay_export_posture.is_empty()
                    || surface.snapshot_golden_refs.is_empty()
                    || surface.support_export_refs.is_empty())
            {
                violations.push(
                    NotebookDataQualificationViolation::IncompleteReplayExportEvidence {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.renders_stable() && surface.accessibility_refs.is_empty() {
                violations.push(
                    NotebookDataQualificationViolation::MissingAccessibilityEvidence {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.renders_stable()
                && matches!(
                    surface.surface_kind,
                    NotebookDataSurfaceKind::NotebookDocument
                        | NotebookDataSurfaceKind::NotebookReviewExport
                        | NotebookDataSurfaceKind::VariableExplorer
                )
                && !surface
                    .notebook_contracts
                    .as_ref()
                    .is_some_and(NotebookObjectContracts::complete)
            {
                violations.push(
                    NotebookDataQualificationViolation::IncompleteNotebookContracts {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.renders_stable()
                && matches!(
                    surface.surface_kind,
                    NotebookDataSurfaceKind::DataTable
                        | NotebookDataSurfaceKind::ResultGrid
                        | NotebookDataSurfaceKind::ChartSummary
                        | NotebookDataSurfaceKind::ExperimentHandoff
                        | NotebookDataSurfaceKind::ApiDatabaseResponseViewer
                )
                && !surface
                    .data_rich_contracts
                    .as_ref()
                    .is_some_and(DataRichObjectContracts::complete)
            {
                violations.push(
                    NotebookDataQualificationViolation::IncompleteDataRichContracts {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
            if surface.promoted_build_surface
                && surface.surface_kind.database_depth_sensitive()
                && surface.renders_stable()
                && !surface.qualification_packet.as_ref().is_some_and(|packet| {
                    packet
                        .evidence_refs
                        .iter()
                        .any(|evidence| evidence.contains("database_result_grid_family_packet"))
                })
            {
                violations.push(NotebookDataQualificationViolation::DatabaseDepthOverclaim {
                    surface_id: surface.surface_id.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(NotebookDataQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in notebook/data-rich qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_notebook_data_rich_surface_qualification(
) -> Result<NotebookDataRichSurfaceQualification, serde_json::Error> {
    serde_json::from_str(NOTEBOOK_DATA_RICH_SURFACE_QUALIFICATION_JSON)
}

/// Validation failure for the notebook/data-rich qualification packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotebookDataQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// Surface ids must be unique.
    DuplicateSurfaceId { surface_id: String },
    /// Displayed lifecycle label is wider than the row claim.
    DisplayedWiderThanClaim { surface_id: String },
    /// A Stable promoted surface lacks a current captured proof packet.
    StableSurfaceWithoutGreenPacket { surface_id: String },
    /// A Stable promoted surface lacks owner sign-off.
    StableSurfaceMissingOwnerSignoff { surface_id: String },
    /// A degraded trust/freshness posture lacks visible state labels.
    MissingVisibleDowngradeState { surface_id: String },
    /// Docs, Help, About, catalog, CLI, support, or migration projection is incomplete.
    IncompleteProjection { surface_id: String },
    /// Handoff lineage, scope, freshness, or redaction truth is incomplete.
    IncompleteHandoffTruth { surface_id: String },
    /// Replay/export, snapshot/golden, or support-export evidence is incomplete.
    IncompleteReplayExportEvidence { surface_id: String },
    /// Accessibility evidence is missing.
    MissingAccessibilityEvidence { surface_id: String },
    /// Notebook object contracts are incomplete on a Stable notebook row.
    IncompleteNotebookContracts { surface_id: String },
    /// Data-rich contracts are incomplete on a Stable data-rich row.
    IncompleteDataRichContracts { surface_id: String },
    /// A database/result-grid-sensitive row widened without its own family packet.
    DatabaseDepthOverclaim { surface_id: String },
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for NotebookDataQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema_version expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record_kind expected {expected}, got {actual}")
            }
            Self::DuplicateSurfaceId { surface_id } => {
                write!(f, "{surface_id} is duplicated")
            }
            Self::DisplayedWiderThanClaim { surface_id } => {
                write!(f, "{surface_id} displays wider than its claim")
            }
            Self::StableSurfaceWithoutGreenPacket { surface_id } => {
                write!(f, "{surface_id} is Stable without a green packet")
            }
            Self::StableSurfaceMissingOwnerSignoff { surface_id } => {
                write!(f, "{surface_id} is Stable without owner sign-off")
            }
            Self::MissingVisibleDowngradeState { surface_id } => {
                write!(f, "{surface_id} lacks visible downgrade state")
            }
            Self::IncompleteProjection { surface_id } => {
                write!(f, "{surface_id} lacks full projection coverage")
            }
            Self::IncompleteHandoffTruth { surface_id } => {
                write!(f, "{surface_id} lacks handoff truth")
            }
            Self::IncompleteReplayExportEvidence { surface_id } => {
                write!(f, "{surface_id} lacks replay/export evidence")
            }
            Self::MissingAccessibilityEvidence { surface_id } => {
                write!(f, "{surface_id} lacks accessibility evidence")
            }
            Self::IncompleteNotebookContracts { surface_id } => {
                write!(f, "{surface_id} lacks notebook object contracts")
            }
            Self::IncompleteDataRichContracts { surface_id } => {
                write!(f, "{surface_id} lacks data-rich object contracts")
            }
            Self::DatabaseDepthOverclaim { surface_id } => {
                write!(f, "{surface_id} overclaims database/result-grid depth")
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for NotebookDataQualificationViolation {}
