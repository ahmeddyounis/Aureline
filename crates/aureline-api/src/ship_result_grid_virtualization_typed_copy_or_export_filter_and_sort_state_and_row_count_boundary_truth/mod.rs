//! Result-grid virtualization, typed copy or export, filter and sort state,
//! and row-count boundary truth qualification records.
//!
//! This module owns the typed records that keep result-grid surfaces
//! inspectable and attributable without depending on hidden shell shortcuts or
//! ad hoc scripts. The boundary schema is
//! [`/schemas/data/ship-result-grid-virtualization-typed-copy-or-export-filter-and-sort-state-and-row-count-boundary-truth.schema.json`](../../../schemas/data/ship-result-grid-virtualization-typed-copy-or-export-filter-and-sort-state-and-row-count-boundary-truth.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/ship-result-grid-virtualization-typed-copy-or-export-filter-and-sort-state-and-row-count-boundary-truth.json`](../../../artifacts/data/m5/ship-result-grid-virtualization-typed-copy-or-export-filter-and-sort-state-and-row-count-boundary-truth.json).
//!
//! Raw row bodies, raw cell values, raw column-comment bodies, raw
//! fully-qualified object names, and raw connection-string fragments do not
//! belong in these records. They carry stable IDs, closed posture vocabularies,
//! and reviewable summaries that UI, CLI, export, support, and public-proof
//! surfaces can ingest safely.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported schema version for result-grid qualification packets.
pub const RESULT_GRID_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ResultGridQualificationPacket`].
pub const RESULT_GRID_QUALIFICATION_RECORD_KIND: &str =
    "ship_result_grid_virtualization_typed_copy_or_export_filter_and_sort_state_and_row_count_boundary_truth";

/// Repo-relative path to the checked-in result-grid qualification packet.
pub const RESULT_GRID_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/data/m5/ship-result-grid-virtualization-typed-copy-or-export-filter-and-sort-state-and-row-count-boundary-truth.json";

/// Embedded checked-in packet JSON.
pub const RESULT_GRID_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/data/m5/ship-result-grid-virtualization-typed-copy-or-export-filter-and-sort-state-and-row-count-boundary-truth.json"
));

/// Qualification label shown on promoted result-grid surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultGridQualificationLabel {
    /// Surface has current proof and may be called stable for its declared scope.
    Stable,
    /// Surface is visible but below stable.
    Preview,
    /// Surface is an experiment or internal lab.
    Labs,
    /// Surface may inspect metadata but must not execute or export live data.
    InspectOnly,
    /// Surface may import or view captured files only.
    ImportOnly,
}

impl ResultGridQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Result-grid surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultGridSurfaceKind {
    /// Main result-grid viewer surface.
    ResultGridViewer,
    /// Typed copy-to-clipboard action.
    TypedCopyAction,
    /// Typed export-to-file action.
    TypedExportAction,
    /// Filter and sort state panel.
    FilterSortStatePanel,
    /// Row-count boundary truth chip.
    RowCountBoundaryChip,
}

/// Virtualization posture applied to a result grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VirtualizationPostureClass {
    /// Small result rendered inline without virtualization.
    InlineNoVirtualizationSmallResult,
    /// Rows are virtualized; columns are inline.
    RowVirtualizedColumnsInline,
    /// Both rows and columns are virtualized.
    RowAndColumnVirtualized,
    /// Large cells open in a detail pane.
    OpenInDetailForLargeCell,
    /// Active content in cells is blocked.
    BlockedActiveContentInCell,
    /// Textual fallback when grid rendering is unavailable.
    TextualFallbackNoGrid,
    /// Virtualization posture is unknown and requires review.
    VirtualizationPostureUnknownRequiresReview,
}

/// Truncation state of a result set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TruncationStateClass {
    /// Full result set with no truncation.
    NoTruncationFullResultSet,
    /// Row truncation; user admit required to continue.
    RowTruncatedUserAdmitRequiredToContinue,
    /// Row truncation due to engine cap.
    RowTruncatedEngineCapped,
    /// Row truncation due to provider cap.
    RowTruncatedProviderCapped,
    /// Row truncation due to size budget cap.
    RowTruncatedSizeBudgetCapped,
    /// Row truncation due to time budget cap.
    RowTruncatedTimeBudgetCapped,
    /// Cell truncated; open in detail pane.
    CellTruncatedOpenInDetail,
    /// Result set is paged; more pages available.
    ResultSetPagedMorePagesAvailable,
    /// Result set is streaming; open buffer.
    ResultSetStreamingOpenBuffer,
    /// Result set is streaming; oldest buffer dropped.
    ResultSetStreamingBufferDroppedOldest,
    /// Truncation state is unknown and requires review.
    TruncationStateUnknownRequiresReview,
}

/// Typed reason for truncation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TruncationReasonClass {
    /// No truncation; full result.
    NoTruncationReasonFullResult,
    /// User-applied row limit.
    UserRowLimitApplied,
    /// Engine max rows or max size reached.
    EngineMaxRowsOrMaxSizeReached,
    /// Provider quota or billing cap reached.
    ProviderQuotaOrBillingCapReached,
    /// Session memory budget reached.
    SessionMemoryBudgetReached,
    /// Session time budget reached.
    SessionTimeBudgetReached,
    /// Viewer inline size budget reached.
    ViewerInlineSizeBudgetReached,
    /// Blob or LOB size budget reached.
    BlobOrLobSizeBudgetReached,
    /// Active content blocked; no render.
    ActiveContentBlockedNoRender,
    /// Schema drift produced a partial typed result.
    SchemaDriftPartialResultTyped,
    /// Truncation reason is unknown and requires review.
    TruncationReasonUnknownRequiresReview,
}

/// Row-count truth class disclosed on the surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowCountTruthClass {
    /// Exact total row count is known.
    RowCountExactTotalKnown,
    /// Exact count of returned rows only; total unknown.
    RowCountExactReturnedOnlyTotalUnknown,
    /// Estimate provided by the engine.
    RowCountEstimateEngineProvided,
    /// Estimate provided by the planner.
    RowCountEstimatePlannerProvided,
    /// Row count unknown while streaming is in flight.
    RowCountUnknownStreamingInFlight,
    /// Row-count truth class is unknown and requires review.
    RowCountTruthClassUnknownRequiresReview,
}

/// Export posture for result-grid copy or export actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportPostureClass {
    /// Full result typed export is admissible.
    ExportAdmissibleFullResultTyped,
    /// Visible rows only typed export is admissible.
    ExportAdmissibleVisibleRowsOnlyTyped,
    /// Visible rows only textual fallback export is admissible.
    ExportAdmissibleVisibleRowsOnlyTextualFallback,
    /// Export blocked pending user consent.
    ExportBlockedPendingConsent,
    /// Export blocked pending policy.
    ExportBlockedPendingPolicy,
    /// Export blocked because redaction class is too high.
    ExportBlockedRedactionClassTooHigh,
    /// Export blocked because active content is present.
    ExportBlockedActiveContentPresent,
    /// Export blocked because provider does not permit export.
    ExportBlockedProviderDoesNotPermitExport,
    /// Export posture is unknown and requires review.
    ExportPostureUnknownRequiresReview,
}

/// Export format for typed or textual fallback exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportFormatClass {
    /// CSV with typed header.
    CsvWithTypedHeader,
    /// TSV with typed header.
    TsvWithTypedHeader,
    /// JSON Lines typed.
    JsonLinesTyped,
    /// JSON Array typed.
    JsonArrayTyped,
    /// Parquet typed.
    ParquetTyped,
    /// Arrow IPC typed.
    ArrowIpcTyped,
    /// SQL INSERT script typed.
    SqlInsertScriptTyped,
    /// Markdown table textual fallback.
    MarkdownTableTextualFallback,
    /// HTML table textual fallback.
    HtmlTableTextualFallback,
    /// Clipboard textual fallback.
    ClipboardTextualFallback,
    /// Notebook handoff typed.
    NotebookHandoffTyped,
    /// Export format is unknown and requires review.
    ExportFormatUnknownRequiresReview,
}

/// Type-coercion state for exports that may lose precision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeCoercionStateClass {
    /// No coercion; engine types are preserved.
    NoCoercionEngineTypedPreserved,
    /// Lossless coercion with documentation.
    LosslessCoercionDocumented,
    /// Lossy coercion with explicit user choice.
    LossyCoercionExplicitUserChoice,
    /// Lossy coercion limited to textual fallback only.
    LossyCoercionTextualFallbackOnly,
    /// Coercion blocked for high redaction class.
    CoercionBlockedForHighRedactionClass,
    /// Type-coercion state is unknown and requires review.
    TypeCoercionStateUnknownRequiresReview,
}

/// Notebook-handoff state for result-grid to notebook kernel transfers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookHandoffStateClass {
    /// No notebook handoff.
    NoNotebookHandoff,
    /// Notebook handoff proposed pending user admit.
    NotebookHandoffProposedPendingUserAdmit,
    /// Typed dataframe handoff admitted.
    NotebookHandoffAdmittedDataframeTyped,
    /// Textual fallback handoff admitted.
    NotebookHandoffAdmittedTextualFallbackOnly,
    /// Notebook handoff blocked pending policy.
    NotebookHandoffBlockedPendingPolicy,
    /// Notebook handoff blocked because redaction class is too high.
    NotebookHandoffBlockedRedactionClassTooHigh,
    /// Notebook handoff state is unknown and requires review.
    NotebookHandoffUnknownRequiresReview,
}

/// Closed column-type class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColumnTypeClass {
    /// Boolean logical.
    BooleanLogical,
    /// Signed integer.
    IntegerSigned,
    /// Unsigned integer.
    IntegerUnsigned,
    /// Decimal or numeric.
    DecimalOrNumeric,
    /// Floating point.
    FloatingPoint,
    /// Text string.
    StringText,
    /// Bounded varchar.
    StringBoundedVarchar,
    /// Binary bytes.
    BinaryBytes,
    /// UUID or GUID.
    UuidOrGuid,
    /// JSON or JSONB document.
    JsonOrJsonbDocument,
    /// XML document.
    XmlDocument,
    /// Date only.
    DateOnly,
    /// Time only.
    TimeOnly,
    /// Timestamp without timezone.
    TimestampNoTimezone,
    /// Timestamp with timezone.
    TimestampWithTimezone,
    /// Interval or duration.
    IntervalOrDuration,
    /// Geometry or geography.
    GeometryOrGeography,
    /// Array of typed values.
    ArrayOfTypedValues,
    /// User-defined or struct record.
    UserDefinedOrStructRecord,
    /// Enum named value.
    EnumNamedValue,
    /// Vector embedding.
    VectorEmbedding,
    /// BLOB or LOB handle.
    BlobOrLobHandle,
    /// Column type unknown and requires review.
    ColumnTypeUnknownRequiresReview,
}

/// Column provenance class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColumnProvenanceClass {
    /// From base table or view.
    FromBaseTableOrView,
    /// From subquery or CTE.
    FromSubqueryOrCte,
    /// Computed expression.
    ComputedExpression,
    /// Aggregate or window function.
    AggregateOrWindowFunction,
    /// Literal constant.
    LiteralConstant,
    /// Engine pseudo column.
    EnginePseudoColumn,
    /// Column provenance unknown and requires review.
    ColumnProvenanceUnknownRequiresReview,
}

/// Where filter evaluation runs relative to the result grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterEvaluationLocus {
    /// Filter pushed down to the engine.
    EngineSideFilterPushedDown,
    /// Filter runs client-side over returned rows only.
    ClientSideFilterOverReturnedRowsOnly,
    /// Mixed locus; user review required.
    MixedFilterLocusUserReviewRequired,
    /// Filter evaluation locus unknown and requires review.
    FilterEvaluationLocusUnknownRequiresReview,
}

/// Proof packet metadata attached to a stable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultGridQualificationProof {
    /// Stable proof packet id.
    pub packet_id: String,
    /// Repo-relative proof artifact reference.
    pub packet_ref: String,
    /// Proof-index reference.
    pub proof_index_ref: String,
    /// UTC capture date.
    pub captured_at: String,
    /// Evidence artifact references.
    pub evidence_refs: Vec<String>,
}

/// Boolean guard set that keeps stable surfaces from inheriting generic table truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultGridSurfaceGuardSet {
    /// Virtualization posture is visible.
    pub virtualization_posture_visible: bool,
    /// Truncation state is visible.
    pub truncation_state_visible: bool,
    /// Typed copy action is visible and reviewable.
    pub typed_copy_visible: bool,
    /// Typed export action is visible and reviewable.
    pub typed_export_visible: bool,
    /// Filter and sort state is visible.
    pub filter_sort_state_visible: bool,
    /// Row-count truth is visible.
    pub row_count_truth_visible: bool,
    /// Export format and coercion state is visible.
    pub export_format_visible: bool,
    /// Notebook handoff state is visible.
    pub notebook_handoff_visible: bool,
}

impl ResultGridSurfaceGuardSet {
    /// Returns true when every required visible guard is present.
    pub const fn all_visible(&self) -> bool {
        self.virtualization_posture_visible
            && self.truncation_state_visible
            && self.typed_copy_visible
            && self.typed_export_visible
            && self.filter_sort_state_visible
            && self.row_count_truth_visible
            && self.export_format_visible
            && self.notebook_handoff_visible
    }
}

/// One governed surface row in the qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultGridSurfaceQualificationRow {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Surface family.
    pub surface_kind: ResultGridSurfaceKind,
    /// Whether this surface is included in the promoted build.
    pub promoted_build_surface: bool,
    /// Claimed label from upstream release planning.
    pub claim_label: ResultGridQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: ResultGridQualificationLabel,
    /// Proof packet when the surface is stable.
    pub qualification_packet: Option<ResultGridQualificationProof>,
    /// Visible guard set.
    pub guards: ResultGridSurfaceGuardSet,
    /// True when missing proof narrows below stable instead of inheriting a label.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// One result-grid viewer row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultGridViewerRow {
    /// Stable viewer id.
    pub viewer_id: String,
    /// Supported virtualization postures.
    pub virtualization_postures: Vec<VirtualizationPostureClass>,
    /// Supported truncation state classes.
    pub truncation_states: Vec<TruncationStateClass>,
    /// Supported row-count truth classes.
    pub row_count_truth_classes: Vec<RowCountTruthClass>,
    /// Supported export posture classes.
    pub export_postures: Vec<ExportPostureClass>,
    /// Supported export format classes.
    pub export_formats: Vec<ExportFormatClass>,
    /// Supported type-coercion states.
    pub type_coercion_states: Vec<TypeCoercionStateClass>,
    /// Supported notebook-handoff states.
    pub notebook_handoff_states: Vec<NotebookHandoffStateClass>,
    /// Maximum rows held in memory before virtualization.
    pub max_rows_before_virtualization: u64,
    /// Maximum cell bytes rendered inline.
    pub max_inline_cell_bytes: u64,
    /// Whether active content is blocked in cells.
    pub active_content_blocked: bool,
    /// Whether the viewer is visible in UI.
    pub visible_in_ui: bool,
}

/// One typed copy action row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TypedCopyActionRow {
    /// Stable action id.
    pub action_id: String,
    /// Supported export formats for copy.
    pub copy_formats: Vec<ExportFormatClass>,
    /// Whether typed columns are preserved.
    pub preserves_typed_columns: bool,
    /// Whether truncation state is disclosed on copy.
    pub preserves_truncation_disclosure: bool,
    /// Whether provenance is preserved on copy.
    pub preserves_provenance_chip: bool,
    /// Type-coercion state applied.
    pub type_coercion_state: TypeCoercionStateClass,
    /// Whether the action is visible in UI.
    pub visible_in_ui: bool,
    /// Whether the action requires explicit user review.
    pub requires_review: bool,
}

/// One typed export action row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TypedExportActionRow {
    /// Stable action id.
    pub action_id: String,
    /// Supported export formats.
    pub export_formats: Vec<ExportFormatClass>,
    /// Whether typed columns are preserved.
    pub preserves_typed_columns: bool,
    /// Whether truncation state is disclosed on export.
    pub preserves_truncation_disclosure: bool,
    /// Whether provenance is preserved on export.
    pub preserves_provenance_chip: bool,
    /// Type-coercion state applied.
    pub type_coercion_state: TypeCoercionStateClass,
    /// Whether the action is visible in UI.
    pub visible_in_ui: bool,
    /// Whether the action requires explicit user review.
    pub requires_review: bool,
}

/// One filter and sort state panel row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FilterSortStatePanelRow {
    /// Stable panel id.
    pub panel_id: String,
    /// Supported filter evaluation loci.
    pub filter_evaluation_loci: Vec<FilterEvaluationLocus>,
    /// Whether the panel shows filter predicate count.
    pub shows_filter_predicate_count: bool,
    /// Whether the panel shows sort key count.
    pub shows_sort_key_count: bool,
    /// Whether the panel is visible in UI.
    pub visible_in_ui: bool,
    /// Whether client-side-only filter locus is disclosed.
    pub discloses_client_side_only_locus: bool,
}

/// One row-count boundary chip row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RowCountBoundaryChipRow {
    /// Stable chip id.
    pub chip_id: String,
    /// Supported row-count truth classes.
    pub row_count_truth_classes: Vec<RowCountTruthClass>,
    /// Whether the chip shows returned row count.
    pub shows_returned_count: bool,
    /// Whether the chip shows total exact count when known.
    pub shows_total_exact_when_known: bool,
    /// Whether the chip shows total estimate when known.
    pub shows_total_estimate_when_known: bool,
    /// Whether streaming state is disclosed.
    pub discloses_streaming_state: bool,
    /// Whether the chip is visible in UI.
    pub visible_in_ui: bool,
}

/// Summary counts for a result-grid qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultGridQualificationSummary {
    /// Number of promoted surfaces.
    pub promoted_surface_count: usize,
    /// Number of stable surfaces.
    pub stable_surface_count: usize,
    /// Number of narrowed promoted surfaces.
    pub narrowed_surface_count: usize,
    /// Number of result-grid viewer rows.
    pub result_grid_viewer_count: usize,
    /// Number of typed copy action rows.
    pub typed_copy_action_count: usize,
    /// Number of typed export action rows.
    pub typed_export_action_count: usize,
    /// Number of filter/sort state panel rows.
    pub filter_sort_state_panel_count: usize,
    /// Number of row-count boundary chip rows.
    pub row_count_boundary_chip_count: usize,
}

/// Canonical result-grid qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultGridQualificationPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Release document reference.
    pub release_doc_ref: String,
    /// Help document reference.
    pub help_doc_ref: String,
    /// JSON Schema path.
    pub schema_ref: String,
    /// Surface rows.
    pub surfaces: Vec<ResultGridSurfaceQualificationRow>,
    /// Result-grid viewer rows.
    pub result_grid_viewers: Vec<ResultGridViewerRow>,
    /// Typed copy action rows.
    pub typed_copy_actions: Vec<TypedCopyActionRow>,
    /// Typed export action rows.
    pub typed_export_actions: Vec<TypedExportActionRow>,
    /// Filter/sort state panel rows.
    pub filter_sort_state_panels: Vec<FilterSortStatePanelRow>,
    /// Row-count boundary chip rows.
    pub row_count_boundary_chips: Vec<RowCountBoundaryChipRow>,
    /// Summary counts.
    pub summary: ResultGridQualificationSummary,
}

impl ResultGridQualificationPacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> ResultGridQualificationSummary {
        let promoted_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.promoted_build_surface)
            .count();
        let stable_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.displayed_label.is_stable())
            .count();
        ResultGridQualificationSummary {
            promoted_surface_count,
            stable_surface_count,
            narrowed_surface_count: promoted_surface_count.saturating_sub(stable_surface_count),
            result_grid_viewer_count: self.result_grid_viewers.len(),
            typed_copy_action_count: self.typed_copy_actions.len(),
            typed_export_action_count: self.typed_export_actions.len(),
            filter_sort_state_panel_count: self.filter_sort_state_panels.len(),
            row_count_boundary_chip_count: self.row_count_boundary_chips.len(),
        }
    }

    /// Validates packet invariants for UI, CLI, support, and release consumers.
    pub fn validate(&self) -> Vec<ResultGridQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != RESULT_GRID_QUALIFICATION_SCHEMA_VERSION {
            violations.push(ResultGridQualificationViolation::SchemaVersion {
                expected: RESULT_GRID_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != RESULT_GRID_QUALIFICATION_RECORD_KIND {
            violations.push(ResultGridQualificationViolation::RecordKind {
                expected: RESULT_GRID_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        collect_ids(
            self.surfaces
                .iter()
                .map(|surface| surface.surface_id.as_str()),
            &mut violations,
            ResultGridQualificationViolationKind::Surface,
        );
        collect_ids(
            self.result_grid_viewers
                .iter()
                .map(|row| row.viewer_id.as_str()),
            &mut violations,
            ResultGridQualificationViolationKind::ResultGridViewer,
        );
        collect_ids(
            self.typed_copy_actions
                .iter()
                .map(|row| row.action_id.as_str()),
            &mut violations,
            ResultGridQualificationViolationKind::TypedCopyAction,
        );
        collect_ids(
            self.typed_export_actions
                .iter()
                .map(|row| row.action_id.as_str()),
            &mut violations,
            ResultGridQualificationViolationKind::TypedExportAction,
        );
        collect_ids(
            self.filter_sort_state_panels
                .iter()
                .map(|row| row.panel_id.as_str()),
            &mut violations,
            ResultGridQualificationViolationKind::FilterSortStatePanel,
        );
        collect_ids(
            self.row_count_boundary_chips
                .iter()
                .map(|row| row.chip_id.as_str()),
            &mut violations,
            ResultGridQualificationViolationKind::RowCountBoundaryChip,
        );

        for surface in &self.surfaces {
            if surface.displayed_label.is_stable() {
                if surface.qualification_packet.is_none() {
                    violations.push(
                        ResultGridQualificationViolation::StableSurfaceMissingProof {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
                if !surface.guards.all_visible() {
                    violations.push(
                        ResultGridQualificationViolation::StableSurfaceMissingGuard {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
            }
            if !surface.displayed_label.is_stable()
                && surface.claim_label.is_stable()
                && !surface.downgrade_if_missing
            {
                violations.push(
                    ResultGridQualificationViolation::NarrowedSurfaceLacksDowngradeRule {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        let virtualization_postures: BTreeSet<_> = self
            .result_grid_viewers
            .iter()
            .flat_map(|row| row.virtualization_postures.iter().copied())
            .collect();
        for required_posture in [
            VirtualizationPostureClass::RowVirtualizedColumnsInline,
            VirtualizationPostureClass::RowAndColumnVirtualized,
            VirtualizationPostureClass::TextualFallbackNoGrid,
        ] {
            if !virtualization_postures.contains(&required_posture) {
                violations.push(
                    ResultGridQualificationViolation::MissingVirtualizationPosture {
                        posture: required_posture,
                    },
                );
            }
        }

        for row in &self.result_grid_viewers {
            if row.max_rows_before_virtualization == 0 {
                violations.push(
                    ResultGridQualificationViolation::ResultGridViewerHasNoVirtualizationLimit {
                        viewer_id: row.viewer_id.clone(),
                    },
                );
            }
            if row.max_inline_cell_bytes == 0 {
                violations.push(
                    ResultGridQualificationViolation::ResultGridViewerHasNoCellSizeLimit {
                        viewer_id: row.viewer_id.clone(),
                    },
                );
            }
            if !row.visible_in_ui {
                violations.push(
                    ResultGridQualificationViolation::ResultGridViewerNotVisibleInUi {
                        viewer_id: row.viewer_id.clone(),
                    },
                );
            }
        }

        let copy_formats: BTreeSet<_> = self
            .typed_copy_actions
            .iter()
            .flat_map(|row| row.copy_formats.iter().copied())
            .collect();
        for required_format in [ExportFormatClass::ClipboardTextualFallback] {
            if !copy_formats.contains(&required_format) {
                violations.push(ResultGridQualificationViolation::MissingCopyExportFormat {
                    format: required_format,
                });
            }
        }

        for row in &self.typed_copy_actions {
            if !row.preserves_truncation_disclosure {
                violations.push(
                    ResultGridQualificationViolation::TypedCopyMissingTruncationDisclosure {
                        action_id: row.action_id.clone(),
                    },
                );
            }
            if !row.visible_in_ui {
                violations.push(
                    ResultGridQualificationViolation::TypedCopyActionNotVisibleInUi {
                        action_id: row.action_id.clone(),
                    },
                );
            }
        }

        let export_formats: BTreeSet<_> = self
            .typed_export_actions
            .iter()
            .flat_map(|row| row.export_formats.iter().copied())
            .collect();
        for required_format in [
            ExportFormatClass::CsvWithTypedHeader,
            ExportFormatClass::JsonLinesTyped,
            ExportFormatClass::ParquetTyped,
        ] {
            if !export_formats.contains(&required_format) {
                violations.push(ResultGridQualificationViolation::MissingExportFormat {
                    format: required_format,
                });
            }
        }

        for row in &self.typed_export_actions {
            if !row.preserves_truncation_disclosure {
                violations.push(
                    ResultGridQualificationViolation::TypedExportMissingTruncationDisclosure {
                        action_id: row.action_id.clone(),
                    },
                );
            }
            if !row.visible_in_ui {
                violations.push(
                    ResultGridQualificationViolation::TypedExportActionNotVisibleInUi {
                        action_id: row.action_id.clone(),
                    },
                );
            }
        }

        let filter_loci: BTreeSet<_> = self
            .filter_sort_state_panels
            .iter()
            .flat_map(|row| row.filter_evaluation_loci.iter().copied())
            .collect();
        for required_locus in [
            FilterEvaluationLocus::EngineSideFilterPushedDown,
            FilterEvaluationLocus::ClientSideFilterOverReturnedRowsOnly,
        ] {
            if !filter_loci.contains(&required_locus) {
                violations.push(
                    ResultGridQualificationViolation::MissingFilterEvaluationLocus {
                        locus: required_locus,
                    },
                );
            }
        }

        for row in &self.filter_sort_state_panels {
            if !row.visible_in_ui {
                violations.push(
                    ResultGridQualificationViolation::FilterSortStatePanelNotVisibleInUi {
                        panel_id: row.panel_id.clone(),
                    },
                );
            }
            if !row.discloses_client_side_only_locus {
                violations.push(
                    ResultGridQualificationViolation::FilterSortStatePanelHidesClientSideLocus {
                        panel_id: row.panel_id.clone(),
                    },
                );
            }
        }

        let row_count_truths: BTreeSet<_> = self
            .row_count_boundary_chips
            .iter()
            .flat_map(|row| row.row_count_truth_classes.iter().copied())
            .collect();
        for required_truth in [
            RowCountTruthClass::RowCountExactTotalKnown,
            RowCountTruthClass::RowCountExactReturnedOnlyTotalUnknown,
            RowCountTruthClass::RowCountUnknownStreamingInFlight,
        ] {
            if !row_count_truths.contains(&required_truth) {
                violations.push(
                    ResultGridQualificationViolation::MissingRowCountTruthClass {
                        truth: required_truth,
                    },
                );
            }
        }

        for row in &self.row_count_boundary_chips {
            if !row.visible_in_ui {
                violations.push(
                    ResultGridQualificationViolation::RowCountBoundaryChipNotVisibleInUi {
                        chip_id: row.chip_id.clone(),
                    },
                );
            }
            if !row.shows_returned_count {
                violations.push(
                    ResultGridQualificationViolation::RowCountBoundaryChipHidesReturnedCount {
                        chip_id: row.chip_id.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ResultGridQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in result-grid qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_result_grid_qualification(
) -> Result<ResultGridQualificationPacket, serde_json::Error> {
    serde_json::from_str(RESULT_GRID_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultGridQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Result-grid viewer rows.
    ResultGridViewer,
    /// Typed copy action rows.
    TypedCopyAction,
    /// Typed export action rows.
    TypedExportAction,
    /// Filter/sort state panel rows.
    FilterSortStatePanel,
    /// Row-count boundary chip rows.
    RowCountBoundaryChip,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<ResultGridQualificationViolation>,
    kind: ResultGridQualificationViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(ResultGridQualificationViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for result-grid qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResultGridQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: ResultGridQualificationViolationKind,
        id: String,
    },
    /// Stable row has no proof packet.
    StableSurfaceMissingProof { surface_id: String },
    /// Stable row is missing one or more visible guards.
    StableSurfaceMissingGuard { surface_id: String },
    /// Narrowed stable claim lacks an explicit downgrade rule.
    NarrowedSurfaceLacksDowngradeRule { surface_id: String },
    /// Required virtualization posture is missing.
    MissingVirtualizationPosture { posture: VirtualizationPostureClass },
    /// Result-grid viewer has no virtualization limit.
    ResultGridViewerHasNoVirtualizationLimit { viewer_id: String },
    /// Result-grid viewer has no inline cell size limit.
    ResultGridViewerHasNoCellSizeLimit { viewer_id: String },
    /// Result-grid viewer is not visible in UI.
    ResultGridViewerNotVisibleInUi { viewer_id: String },
    /// Required copy export format is missing.
    MissingCopyExportFormat { format: ExportFormatClass },
    /// Typed copy action does not disclose truncation state.
    TypedCopyMissingTruncationDisclosure { action_id: String },
    /// Typed copy action is not visible in UI.
    TypedCopyActionNotVisibleInUi { action_id: String },
    /// Required export format is missing.
    MissingExportFormat { format: ExportFormatClass },
    /// Typed export action does not disclose truncation state.
    TypedExportMissingTruncationDisclosure { action_id: String },
    /// Typed export action is not visible in UI.
    TypedExportActionNotVisibleInUi { action_id: String },
    /// Required filter evaluation locus is missing.
    MissingFilterEvaluationLocus { locus: FilterEvaluationLocus },
    /// Filter/sort state panel is not visible in UI.
    FilterSortStatePanelNotVisibleInUi { panel_id: String },
    /// Filter/sort state panel hides client-side-only filter locus.
    FilterSortStatePanelHidesClientSideLocus { panel_id: String },
    /// Required row-count truth class is missing.
    MissingRowCountTruthClass { truth: RowCountTruthClass },
    /// Row-count boundary chip is not visible in UI.
    RowCountBoundaryChipNotVisibleInUi { chip_id: String },
    /// Row-count boundary chip hides returned row count.
    RowCountBoundaryChipHidesReturnedCount { chip_id: String },
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for ResultGridQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema_version expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record_kind expected {expected}, got {actual}")
            }
            Self::DuplicateId { kind, id } => write!(f, "{kind:?} id {id} is duplicated"),
            Self::StableSurfaceMissingProof { surface_id } => {
                write!(f, "{surface_id} is stable without a proof packet")
            }
            Self::StableSurfaceMissingGuard { surface_id } => {
                write!(f, "{surface_id} is stable without complete guard truth")
            }
            Self::NarrowedSurfaceLacksDowngradeRule { surface_id } => {
                write!(f, "{surface_id} is narrowed without a downgrade rule")
            }
            Self::MissingVirtualizationPosture { posture } => {
                write!(f, "virtualization posture {posture:?} is not covered")
            }
            Self::ResultGridViewerHasNoVirtualizationLimit { viewer_id } => {
                write!(f, "{viewer_id} has no virtualization row limit")
            }
            Self::ResultGridViewerHasNoCellSizeLimit { viewer_id } => {
                write!(f, "{viewer_id} has no inline cell size limit")
            }
            Self::ResultGridViewerNotVisibleInUi { viewer_id } => {
                write!(f, "{viewer_id} is not visible in UI")
            }
            Self::MissingCopyExportFormat { format } => {
                write!(f, "copy export format {format:?} is not covered")
            }
            Self::TypedCopyMissingTruncationDisclosure { action_id } => {
                write!(f, "{action_id} does not disclose truncation state on copy")
            }
            Self::TypedCopyActionNotVisibleInUi { action_id } => {
                write!(f, "{action_id} is not visible in UI")
            }
            Self::MissingExportFormat { format } => {
                write!(f, "export format {format:?} is not covered")
            }
            Self::TypedExportMissingTruncationDisclosure { action_id } => {
                write!(
                    f,
                    "{action_id} does not disclose truncation state on export"
                )
            }
            Self::TypedExportActionNotVisibleInUi { action_id } => {
                write!(f, "{action_id} is not visible in UI")
            }
            Self::MissingFilterEvaluationLocus { locus } => {
                write!(f, "filter evaluation locus {locus:?} is not covered")
            }
            Self::FilterSortStatePanelNotVisibleInUi { panel_id } => {
                write!(f, "{panel_id} is not visible in UI")
            }
            Self::FilterSortStatePanelHidesClientSideLocus { panel_id } => {
                write!(f, "{panel_id} hides client-side-only filter locus")
            }
            Self::MissingRowCountTruthClass { truth } => {
                write!(f, "row-count truth class {truth:?} is not covered")
            }
            Self::RowCountBoundaryChipNotVisibleInUi { chip_id } => {
                write!(f, "{chip_id} is not visible in UI")
            }
            Self::RowCountBoundaryChipHidesReturnedCount { chip_id } => {
                write!(f, "{chip_id} hides returned row count")
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for ResultGridQualificationViolation {}
