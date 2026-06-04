//! Stable dense-collection contract packet for M4 collection portability.
//!
//! The packet in this module is the stable cross-surface lane for collection
//! truth. It uses the typed filter AST, saved-view, scope-counter, selection,
//! and batch-review objects from `aureline-search` as the shared substrate, then
//! adds the governed cross-surface vocabulary needed by search, provider-backed
//! review/admin, package/test/diagnostics, and notebook data-grid surfaces.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use aureline_search::{
    BatchActionClass, BatchAftermathSummary, BatchExecutionOriginClass, BatchMemberDisposition,
    BatchReviewMember, BatchReviewSheet, CollectionCountStatus, CollectionFilterAst,
    CollectionFilterClause, CollectionFilterLiteral, CollectionFilterOperator,
    CollectionFilterSourceClass, CollectionScopeCounters, CollectionSelectionState,
    CollectionSortKey, CollectionSurfaceFamily, FilterRoundTripState, SavedCollectionView,
    SavedViewFallbackBehavior, SavedViewOwnerScope, SavedViewPrivacyClass, SelectionScopeClass,
    StableCollectionItemRef,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`CollectionContractPacket`].
pub const COLLECTION_CONTRACT_PACKET_RECORD_KIND: &str = "dense_collection_contract_stable_packet";

/// Stable record-kind tag for [`CollectionContractSupportExport`].
pub const COLLECTION_CONTRACT_SUPPORT_EXPORT_RECORD_KIND: &str =
    "dense_collection_contract_support_export";

/// Integer schema version for stable dense-collection packets.
pub const COLLECTION_CONTRACT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative reviewer doc path.
pub const COLLECTION_CONTRACT_DOC_REF: &str =
    "docs/m4/stabilize-filter-ast-saved-view-scope-pack-column-preset.md";

/// Repo-relative artifact narrative path.
pub const COLLECTION_CONTRACT_ARTIFACT_DOC_REF: &str =
    "artifacts/collections/m4/stabilize-filter-ast-saved-view-scope-pack-column-preset.md";

/// Repo-relative fixture corpus path.
pub const COLLECTION_CONTRACT_FIXTURE_DIR: &str =
    "fixtures/collections/m4/stabilize-filter-ast-saved-view-scope-pack-column-preset";

/// Repo-relative packet artifact path.
pub const COLLECTION_CONTRACT_PACKET_ARTIFACT_REF: &str =
    "artifacts/collections/m4/stabilize-filter-ast-saved-view-scope-pack-column-preset.json";

const REQUIRED_VOCABULARY_TERMS: [ScopeCounterVocabularyTerm; 8] = [
    ScopeCounterVocabularyTerm::Visible,
    ScopeCounterVocabularyTerm::Loaded,
    ScopeCounterVocabularyTerm::Matching,
    ScopeCounterVocabularyTerm::Selected,
    ScopeCounterVocabularyTerm::Approx,
    ScopeCounterVocabularyTerm::Exact,
    ScopeCounterVocabularyTerm::HiddenByPolicy,
    ScopeCounterVocabularyTerm::OutsideCurrentFilter,
];

/// Surface family covered by the stable collection contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DenseCollectionSurfaceFamily {
    /// Search results, references, and global result grids.
    Search,
    /// Provider-backed review inboxes, approval queues, or admin grids.
    ProviderReviewAdmin,
    /// Package, test, diagnostics, inventory, or run-result grids.
    PackageTestDiagnostics,
    /// Notebook tables, data-result grids, and streaming partial datasets.
    NotebookDataGrid,
}

impl DenseCollectionSurfaceFamily {
    /// Stable token used in packets, fixtures, docs, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Search => "search",
            Self::ProviderReviewAdmin => "provider_review_admin",
            Self::PackageTestDiagnostics => "package_test_diagnostics",
            Self::NotebookDataGrid => "notebook_data_grid",
        }
    }

    /// Every surface family required before the stable claim can publish.
    pub const fn required() -> [Self; 4] {
        [
            Self::Search,
            Self::ProviderReviewAdmin,
            Self::PackageTestDiagnostics,
            Self::NotebookDataGrid,
        ]
    }
}

/// Ownership and execution posture for a dense collection surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DenseCollectionSurfaceOwnership {
    /// Data and actions are locally authoritative.
    LocalAuthoritative,
    /// Provider owns data, count, or mutation authority.
    ProviderAuthoritative,
    /// Client owns staging while provider owns execution or final state.
    MixedClientProvider,
}

/// Canonical count and scope vocabulary that must survive every projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeCounterVocabularyTerm {
    /// Rows currently visible in page, viewport, or rendered grid window.
    Visible,
    /// Rows fetched or materialized into the client.
    Loaded,
    /// Rows matching the active filter or authoritative query.
    Matching,
    /// Rows selected by stable identity or an explicit query snapshot.
    Selected,
    /// Count is approximate and must be labeled as such.
    Approx,
    /// Count is exact for the stated scope.
    Exact,
    /// Rows hidden or blocked by policy.
    HiddenByPolicy,
    /// Selected or matching rows outside the current filter.
    OutsideCurrentFilter,
}

impl ScopeCounterVocabularyTerm {
    /// Stable token used in packets, fixtures, CLI output, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visible => "visible",
            Self::Loaded => "loaded",
            Self::Matching => "matching",
            Self::Selected => "selected",
            Self::Approx => "approx.",
            Self::Exact => "exact",
            Self::HiddenByPolicy => "hidden by policy",
            Self::OutsideCurrentFilter => "outside current filter",
        }
    }
}

/// Meaning shown by select-all controls before a selection can expand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectAllMeaning {
    /// Select all currently visible rows.
    VisibleRows,
    /// Select all loaded rows.
    LoadedRows,
    /// Select all matching rows after a deliberate expansion step.
    AllMatchingAfterExplicitExpansion,
    /// Select a stable explicit identity set.
    ExplicitIdentitySet,
    /// Select a provider-side query object after review.
    ProviderSideQueryAfterReview,
}

impl SelectAllMeaning {
    /// Stable token used in packets and review sheets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VisibleRows => "visible_rows",
            Self::LoadedRows => "loaded_rows",
            Self::AllMatchingAfterExplicitExpansion => "all_matching_after_explicit_expansion",
            Self::ExplicitIdentitySet => "explicit_identity_set",
            Self::ProviderSideQueryAfterReview => "provider_side_query_after_review",
        }
    }
}

/// Reopen posture for saved views and shared links.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenScopePosture {
    /// Captured and current scope still match.
    CapturedScopeStillCurrent,
    /// Current roots, providers, permissions, or policy changed and require rebind.
    CurrentScopeChangedRebindRequired,
    /// Selection or batch scope is based on a prior query snapshot.
    BasedOnPriorQuerySnapshot,
    /// Provider or schema compatibility changed and migration is required.
    IncompatibleMigrationRequired,
}

/// Batch-review posture derived from action and selection risk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchProtectionPosture {
    /// No review sheet is required for a routine non-mutating action.
    RoutineNoReviewRequired,
    /// Review sheet is required and present.
    ReviewRequiredPresent,
    /// Review sheet is required but absent.
    ReviewRequiredMissing,
}

/// Object kind covered by the stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableDenseCollectionObjectKind {
    /// Typed filter AST.
    FilterAst,
    /// Saved view.
    SavedView,
    /// Query-history entry.
    QueryHistory,
    /// Scope pack.
    ScopePack,
    /// Column preset.
    ColumnPreset,
    /// Scope counters.
    ScopeCounters,
    /// Selection state.
    SelectionState,
    /// Batch-review sheet.
    BatchReviewSheet,
}

/// Query-history object shared by UI, CLI, export, and support lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionContractQueryHistory {
    /// Stable history id.
    pub query_history_id: String,
    /// Owner or source class that created the history row.
    pub owner_source_class: String,
    /// Privacy posture for query material.
    pub privacy_class: String,
    /// Schema version of the history object.
    pub schema_version: u32,
    /// Execution origin that produced the entry.
    pub execution_origin: String,
    /// True when raw query text is excluded from portable state.
    pub raw_query_material_excluded: bool,
    /// Retention label for local, synced, support, or provider state.
    pub retention_label: String,
}

/// Scope pack object preserving captured-versus-current scope truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionContractScopePack {
    /// Stable scope-pack id.
    pub scope_pack_id: String,
    /// Captured workspace, workset, provider, or dataset identity.
    pub captured_scope_identity: String,
    /// Current scope identity at reopen time.
    pub current_scope_identity: String,
    /// Scope reopen posture.
    pub reopen_posture: ReopenScopePosture,
    /// Include rules in portable reviewable form.
    pub include_rules: Vec<String>,
    /// Exclude rules in portable reviewable form.
    pub exclude_rules: Vec<String>,
    /// Reasons a captured scope member could not be resolved.
    pub missing_scope_reasons: Vec<String>,
}

/// Column preset object with protected identity/provenance columns.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionContractColumnPreset {
    /// Stable preset id.
    pub column_preset_id: String,
    /// Ordered visible columns.
    pub visible_column_ids: Vec<String>,
    /// Pinned columns that may not be silently removed.
    pub pinned_column_ids: Vec<String>,
    /// Required identity or provenance columns.
    pub required_column_ids: Vec<String>,
    /// Density mode token.
    pub density_mode: String,
    /// Migration or reset path shown when columns cannot replay.
    pub fallback_behavior: String,
}

impl CollectionContractColumnPreset {
    fn missing_required_columns(&self) -> Vec<String> {
        self.required_column_ids
            .iter()
            .filter(|id| !self.visible_column_ids.contains(id))
            .cloned()
            .collect()
    }
}

/// Consumer projection proving a lane preserves the packet vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionContractConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: CollectionContractConsumerSurface,
    /// Stable projection id.
    pub projection_id: String,
    /// Packet id consumed by the projection.
    pub packet_id_ref: String,
    /// True when the filter AST is preserved.
    pub preserves_filter_ast: bool,
    /// True when saved-view/query-history/scope-pack state is preserved.
    pub preserves_saved_view_scope_pack_history: bool,
    /// True when column-preset state is preserved.
    pub preserves_column_preset: bool,
    /// True when all scope vocabulary terms are preserved.
    pub preserves_scope_vocabulary: bool,
    /// True when select-all wording stays explicit.
    pub preserves_select_all_meaning: bool,
    /// True when batch-review and mixed-outcome truth are preserved.
    pub preserves_batch_review: bool,
    /// True when raw secrets, cursors, and query material are excluded.
    pub redaction_safe: bool,
}

impl CollectionContractConsumerProjection {
    fn preserves_packet(&self, packet_id: &str) -> bool {
        self.packet_id_ref == packet_id
            && self.preserves_filter_ast
            && self.preserves_saved_view_scope_pack_history
            && self.preserves_column_preset
            && self.preserves_scope_vocabulary
            && self.preserves_select_all_meaning
            && self.preserves_batch_review
            && self.redaction_safe
            && !self.projection_id.trim().is_empty()
    }
}

/// Required consumer surface for dense collection truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionContractConsumerSurface {
    /// Desktop UI chrome and tables.
    DesktopUi,
    /// CLI and headless output.
    CliHeadless,
    /// Export packet.
    ExportPacket,
    /// Support capture.
    SupportCapture,
    /// Keyboard and screen-reader projection.
    AccessibilityTree,
}

impl CollectionContractConsumerSurface {
    /// Required consumer surfaces for stable publication.
    pub const fn required() -> [Self; 5] {
        [
            Self::DesktopUi,
            Self::CliHeadless,
            Self::ExportPacket,
            Self::SupportCapture,
            Self::AccessibilityTree,
        ]
    }

    /// Stable token used in packet fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopUi => "desktop_ui",
            Self::CliHeadless => "cli_headless",
            Self::ExportPacket => "export_packet",
            Self::SupportCapture => "support_capture",
            Self::AccessibilityTree => "accessibility_tree",
        }
    }
}

/// One dense collection row proving the shared contract on a surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionContractRow {
    /// Stable row id.
    pub row_id: String,
    /// Covered surface family.
    pub surface_family: DenseCollectionSurfaceFamily,
    /// Local, provider, or mixed ownership posture.
    pub surface_ownership: DenseCollectionSurfaceOwnership,
    /// Collection object kinds carried by the row.
    pub object_kinds: Vec<StableDenseCollectionObjectKind>,
    /// Typed filter AST.
    pub filter_ast: CollectionFilterAst,
    /// Saved view bound to the filter and columns.
    pub saved_view: SavedCollectionView,
    /// Query-history object.
    pub query_history: CollectionContractQueryHistory,
    /// Scope pack object.
    pub scope_pack: CollectionContractScopePack,
    /// Column preset.
    pub column_preset: CollectionContractColumnPreset,
    /// Stable scope counters from the shared collection substrate.
    pub scope_counters: CollectionScopeCounters,
    /// Canonical vocabulary terms rendered by this row.
    pub scope_vocabulary_terms: Vec<ScopeCounterVocabularyTerm>,
    /// Selection state that survives sorting, filtering, pagination, and streaming.
    pub selection_state: CollectionSelectionState,
    /// Meaning shown by select-all controls.
    pub select_all_meaning: SelectAllMeaning,
    /// Batch-review posture.
    pub batch_protection_posture: BatchProtectionPosture,
    /// Batch-review sheet for protected actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch_review: Option<BatchReviewSheet>,
    /// True when export preserves exact packet truth instead of recomputing counts.
    pub export_preserves_scope_truth: bool,
    /// True when support capture preserves the same packet truth.
    pub support_capture_preserves_scope_truth: bool,
    /// True when keyboard and screen-reader users can inspect scope and batch state.
    pub accessibility_inspection_available: bool,
    /// True when raw query text, provider cursors, secrets, and transient selection are excluded.
    pub unsafe_transient_state_excluded: bool,
}

impl CollectionContractRow {
    fn requires_review_sheet(&self) -> bool {
        self.batch_review
            .as_ref()
            .is_some_and(|sheet| sheet.action_class.requires_review_sheet())
            || matches!(
                self.surface_ownership,
                DenseCollectionSurfaceOwnership::ProviderAuthoritative
                    | DenseCollectionSurfaceOwnership::MixedClientProvider
            )
            || matches!(
                self.select_all_meaning,
                SelectAllMeaning::AllMatchingAfterExplicitExpansion
                    | SelectAllMeaning::ProviderSideQueryAfterReview
            )
    }
}

/// Promotion state for the stable dense-collection packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionContractPromotionState {
    /// Packet certifies stable publication.
    Stable,
    /// Packet has warnings and must stay narrowed.
    NarrowedBelowStable,
    /// Packet has blocker findings.
    BlocksStable,
}

/// Severity for one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionContractFindingSeverity {
    /// Informational finding.
    Info,
    /// Warning that narrows stable confidence.
    Warning,
    /// Blocker that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for the stable contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionContractFindingKind {
    /// Packet has wrong record kind.
    WrongRecordKind,
    /// Packet has wrong schema version.
    WrongSchemaVersion,
    /// Required identity is missing.
    MissingIdentity,
    /// A required surface family is missing.
    MissingRequiredSurface,
    /// A row has invalid filter AST or saved view state.
    InvalidFilterOrSavedView,
    /// A row is missing query-history or scope-pack truth.
    MissingQueryHistoryOrScopePack,
    /// Column preset drops a required identity or provenance column.
    ColumnPresetDropsRequiredColumn,
    /// Scope vocabulary drops a required term.
    ScopeVocabularyDropped,
    /// Select-all meaning is ambiguous.
    SelectAllMeaningAmbiguous,
    /// Protected action is missing a batch-review sheet.
    BatchReviewRequiredButMissing,
    /// Batch review lacks mixed outcome or member truth.
    BatchReviewOutcomeCollapsed,
    /// Stale or prior snapshot selection lacks disclosure.
    StaleSelectionUndisclosed,
    /// Export/support/a11y projection drops collection truth.
    ProjectionDropsTruth,
    /// Unsafe transient or secret-bearing state is present.
    UnsafeTransientStatePresent,
    /// Promotion state disagrees with validation.
    PromotionStateMismatch,
}

impl CollectionContractFindingKind {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingRequiredSurface => "missing_required_surface",
            Self::InvalidFilterOrSavedView => "invalid_filter_or_saved_view",
            Self::MissingQueryHistoryOrScopePack => "missing_query_history_or_scope_pack",
            Self::ColumnPresetDropsRequiredColumn => "column_preset_drops_required_column",
            Self::ScopeVocabularyDropped => "scope_vocabulary_dropped",
            Self::SelectAllMeaningAmbiguous => "select_all_meaning_ambiguous",
            Self::BatchReviewRequiredButMissing => "batch_review_required_but_missing",
            Self::BatchReviewOutcomeCollapsed => "batch_review_outcome_collapsed",
            Self::StaleSelectionUndisclosed => "stale_selection_undisclosed",
            Self::ProjectionDropsTruth => "projection_drops_truth",
            Self::UnsafeTransientStatePresent => "unsafe_transient_state_present",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the stable contract validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionContractValidationFinding {
    /// Finding kind.
    pub finding_kind: CollectionContractFindingKind,
    /// Finding severity.
    pub severity: CollectionContractFindingSeverity,
    /// Reviewable finding message.
    pub message: String,
}

impl CollectionContractValidationFinding {
    fn new(
        finding_kind: CollectionContractFindingKind,
        severity: CollectionContractFindingSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            message: message.into(),
        }
    }
}

/// Stable dense-collection contract packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionContractPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Packet generation timestamp.
    pub generated_at: String,
    /// Source docs, artifacts, and fixture refs.
    pub source_contract_refs: Vec<String>,
    /// Rows proving the contract across dense surfaces.
    pub rows: Vec<CollectionContractRow>,
    /// Consumer projections preserving the contract.
    pub consumer_projections: Vec<CollectionContractConsumerProjection>,
    /// Derived promotion state.
    pub promotion_state: CollectionContractPromotionState,
    /// Derived validation findings.
    pub validation_findings: Vec<CollectionContractValidationFinding>,
}

impl CollectionContractPacket {
    /// Materializes a stable contract packet and records validation findings.
    pub fn materialize(
        packet_id: impl Into<String>,
        generated_at: impl Into<String>,
        rows: Vec<CollectionContractRow>,
        consumer_projections: Vec<CollectionContractConsumerProjection>,
    ) -> Self {
        let mut packet = Self {
            record_kind: COLLECTION_CONTRACT_PACKET_RECORD_KIND.to_owned(),
            schema_version: COLLECTION_CONTRACT_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            generated_at: generated_at.into(),
            source_contract_refs: vec![
                COLLECTION_CONTRACT_DOC_REF.to_owned(),
                COLLECTION_CONTRACT_ARTIFACT_DOC_REF.to_owned(),
                COLLECTION_CONTRACT_FIXTURE_DIR.to_owned(),
            ],
            rows,
            consumer_projections,
            promotion_state: CollectionContractPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates packet invariants, including serialized metadata fields.
    pub fn validate(&self) -> Vec<CollectionContractValidationFinding> {
        self.derived_findings(true)
    }

    /// Builds a support export that preserves the exact product packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> CollectionContractSupportExport {
        CollectionContractSupportExport {
            record_kind: COLLECTION_CONTRACT_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: COLLECTION_CONTRACT_SCHEMA_VERSION,
            export_id: export_id.into(),
            packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            packet: self.clone(),
        }
    }

    fn derived_findings(
        &self,
        include_record_fields: bool,
    ) -> Vec<CollectionContractValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != COLLECTION_CONTRACT_PACKET_RECORD_KIND {
            findings.push(CollectionContractValidationFinding::new(
                CollectionContractFindingKind::WrongRecordKind,
                CollectionContractFindingSeverity::Blocker,
                "packet record kind does not match the stable contract",
            ));
        }
        if include_record_fields && self.schema_version != COLLECTION_CONTRACT_SCHEMA_VERSION {
            findings.push(CollectionContractValidationFinding::new(
                CollectionContractFindingKind::WrongSchemaVersion,
                CollectionContractFindingSeverity::Blocker,
                "packet schema version does not match the stable contract",
            ));
        }
        if self.packet_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            findings.push(CollectionContractValidationFinding::new(
                CollectionContractFindingKind::MissingIdentity,
                CollectionContractFindingSeverity::Blocker,
                "packet id and generated timestamp are required",
            ));
        }

        let present_surfaces: BTreeSet<_> =
            self.rows.iter().map(|row| row.surface_family).collect();
        for required in DenseCollectionSurfaceFamily::required() {
            if !present_surfaces.contains(&required) {
                findings.push(CollectionContractValidationFinding::new(
                    CollectionContractFindingKind::MissingRequiredSurface,
                    CollectionContractFindingSeverity::Blocker,
                    format!("missing required surface {}", required.as_str()),
                ));
            }
        }

        for row in &self.rows {
            validate_row(row, &mut findings);
        }

        for required in CollectionContractConsumerSurface::required() {
            if !self.consumer_projections.iter().any(|projection| {
                projection.consumer_surface == required
                    && projection.preserves_packet(&self.packet_id)
            }) {
                findings.push(CollectionContractValidationFinding::new(
                    CollectionContractFindingKind::ProjectionDropsTruth,
                    CollectionContractFindingSeverity::Blocker,
                    format!("missing preserved {} projection", required.as_str()),
                ));
            }
        }

        for projection in &self.consumer_projections {
            if !projection.preserves_packet(&self.packet_id) {
                findings.push(CollectionContractValidationFinding::new(
                    CollectionContractFindingKind::ProjectionDropsTruth,
                    CollectionContractFindingSeverity::Blocker,
                    format!(
                        "projection {} drops stable collection truth",
                        projection.projection_id
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion.retain(|finding| {
                finding.finding_kind != CollectionContractFindingKind::PromotionStateMismatch
            });
            let derived = promotion_state_for(&without_promotion);
            if self.promotion_state != derived {
                findings.push(CollectionContractValidationFinding::new(
                    CollectionContractFindingKind::PromotionStateMismatch,
                    CollectionContractFindingSeverity::Blocker,
                    "stored promotion state does not match validator output",
                ));
            }
        }

        findings
    }
}

fn validate_row(
    row: &CollectionContractRow,
    findings: &mut Vec<CollectionContractValidationFinding>,
) {
    if row.row_id.trim().is_empty() {
        findings.push(CollectionContractValidationFinding::new(
            CollectionContractFindingKind::MissingIdentity,
            CollectionContractFindingSeverity::Blocker,
            "row id is required",
        ));
    }

    let saved_view_findings = row.saved_view.validate_portability();
    if !row.filter_ast.validate().is_empty() || !saved_view_findings.is_empty() {
        findings.push(CollectionContractValidationFinding::new(
            CollectionContractFindingKind::InvalidFilterOrSavedView,
            CollectionContractFindingSeverity::Blocker,
            format!("row {} has invalid filter AST or saved view", row.row_id),
        ));
    }

    if row.query_history.query_history_id.trim().is_empty()
        || row.scope_pack.scope_pack_id.trim().is_empty()
        || row.query_history.schema_version == 0
        || !row.query_history.raw_query_material_excluded
    {
        findings.push(CollectionContractValidationFinding::new(
            CollectionContractFindingKind::MissingQueryHistoryOrScopePack,
            CollectionContractFindingSeverity::Blocker,
            format!("row {} drops query-history or scope-pack truth", row.row_id),
        ));
    }

    let missing_columns = row.column_preset.missing_required_columns();
    if !missing_columns.is_empty() {
        findings.push(CollectionContractValidationFinding::new(
            CollectionContractFindingKind::ColumnPresetDropsRequiredColumn,
            CollectionContractFindingSeverity::Blocker,
            format!(
                "row {} drops required columns: {}",
                row.row_id,
                missing_columns.join(",")
            ),
        ));
    }

    let vocabulary: BTreeSet<_> = row.scope_vocabulary_terms.iter().copied().collect();
    for required in REQUIRED_VOCABULARY_TERMS {
        if !vocabulary.contains(&required) {
            findings.push(CollectionContractValidationFinding::new(
                CollectionContractFindingKind::ScopeVocabularyDropped,
                CollectionContractFindingSeverity::Blocker,
                format!("row {} drops {}", row.row_id, required.as_str()),
            ));
        }
    }

    if matches!(
        row.select_all_meaning,
        SelectAllMeaning::AllMatchingAfterExplicitExpansion
    ) && row.selection_state.query_snapshot_id_ref.is_none()
    {
        findings.push(CollectionContractValidationFinding::new(
            CollectionContractFindingKind::SelectAllMeaningAmbiguous,
            CollectionContractFindingSeverity::Blocker,
            format!(
                "row {} all-matching selection lacks a query snapshot",
                row.row_id
            ),
        ));
    }

    if row.requires_review_sheet() && row.batch_review.is_none() {
        findings.push(CollectionContractValidationFinding::new(
            CollectionContractFindingKind::BatchReviewRequiredButMissing,
            CollectionContractFindingSeverity::Blocker,
            format!(
                "row {} protected action lacks a batch-review sheet",
                row.row_id
            ),
        ));
    }

    if let Some(sheet) = &row.batch_review {
        if !sheet.validate().is_empty()
            || (sheet.action_class.requires_review_sheet() && sheet.aftermath_summary.is_none())
            || (sheet.action_class.requires_review_sheet()
                && sheet.included_item_id_refs.is_empty()
                && sheet.blocked_item_id_refs.is_empty()
                && sheet.hidden_item_id_refs.is_empty()
                && sheet.excluded_item_id_refs.is_empty()
                && sheet.stale_item_id_refs.is_empty())
        {
            findings.push(CollectionContractValidationFinding::new(
                CollectionContractFindingKind::BatchReviewOutcomeCollapsed,
                CollectionContractFindingSeverity::Blocker,
                format!(
                    "row {} batch review lacks member or mixed-outcome truth",
                    row.row_id
                ),
            ));
        }
    }

    let stale_scope = matches!(
        row.scope_pack.reopen_posture,
        ReopenScopePosture::BasedOnPriorQuerySnapshot
            | ReopenScopePosture::CurrentScopeChangedRebindRequired
            | ReopenScopePosture::IncompatibleMigrationRequired
    );
    if stale_scope && !row.selection_state.basis_is_stale {
        findings.push(CollectionContractValidationFinding::new(
            CollectionContractFindingKind::StaleSelectionUndisclosed,
            CollectionContractFindingSeverity::Blocker,
            format!(
                "row {} changed scope without stale selection disclosure",
                row.row_id
            ),
        ));
    }

    if !row.export_preserves_scope_truth
        || !row.support_capture_preserves_scope_truth
        || !row.accessibility_inspection_available
    {
        findings.push(CollectionContractValidationFinding::new(
            CollectionContractFindingKind::ProjectionDropsTruth,
            CollectionContractFindingSeverity::Blocker,
            format!(
                "row {} drops export, support, or accessibility truth",
                row.row_id
            ),
        ));
    }

    if !row.unsafe_transient_state_excluded
        || row.saved_view.captures_selection
        || row.saved_view.captures_provider_cursor
    {
        findings.push(CollectionContractValidationFinding::new(
            CollectionContractFindingKind::UnsafeTransientStatePresent,
            CollectionContractFindingSeverity::Blocker,
            format!("row {} admits unsafe transient state", row.row_id),
        ));
    }
}

fn promotion_state_for(
    findings: &[CollectionContractValidationFinding],
) -> CollectionContractPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == CollectionContractFindingSeverity::Blocker)
    {
        CollectionContractPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == CollectionContractFindingSeverity::Warning)
    {
        CollectionContractPromotionState::NarrowedBelowStable
    } else {
        CollectionContractPromotionState::Stable
    }
}

/// Support-export wrapper preserving the exact stable product packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionContractSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Exact product packet.
    pub packet: CollectionContractPacket,
}

impl CollectionContractSupportExport {
    /// True when support export preserves the exact packet safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == COLLECTION_CONTRACT_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == COLLECTION_CONTRACT_SCHEMA_VERSION
            && self.packet_id_ref == self.packet.packet_id
            && self.raw_private_material_excluded
            && self.packet.validate().is_empty()
    }
}

/// Errors emitted while reading the checked-in packet artifact.
#[derive(Debug)]
pub enum CollectionContractArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<CollectionContractValidationFinding>),
}

impl fmt::Display for CollectionContractArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "collection contract packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "collection contract packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for CollectionContractArtifactError {}

/// Returns the checked-in stable dense-collection contract packet.
///
/// # Errors
///
/// Returns an artifact error if the packet cannot parse or fails validation.
pub fn current_stable_dense_collection_contract_packet(
) -> Result<CollectionContractPacket, CollectionContractArtifactError> {
    let packet: CollectionContractPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/collections/m4/stabilize-filter-ast-saved-view-scope-pack-column-preset.json"
    )))
    .map_err(CollectionContractArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(CollectionContractArtifactError::Validation(findings))
    }
}

/// Builds the seeded stable dense-collection contract packet.
pub fn seeded_dense_collection_contract_packet() -> CollectionContractPacket {
    CollectionContractPacket::materialize(
        "collections:m4:dense-contract:stable",
        "2026-06-04T00:00:00Z",
        vec![
            search_row(),
            provider_review_admin_row(),
            package_test_diagnostics_row(),
            notebook_data_grid_row(),
        ],
        CollectionContractConsumerSurface::required()
            .into_iter()
            .map(|surface| CollectionContractConsumerProjection {
                consumer_surface: surface,
                projection_id: format!(
                    "collections:m4:dense-contract:projection:{}",
                    surface.as_str()
                ),
                packet_id_ref: "collections:m4:dense-contract:stable".to_owned(),
                preserves_filter_ast: true,
                preserves_saved_view_scope_pack_history: true,
                preserves_column_preset: true,
                preserves_scope_vocabulary: true,
                preserves_select_all_meaning: true,
                preserves_batch_review: true,
                redaction_safe: true,
            })
            .collect(),
    )
}

fn search_row() -> CollectionContractRow {
    let family = DenseCollectionSurfaceFamily::Search;
    let filter_ast = filter_ast(
        "filter:search:service-errors",
        "workspace current workset",
        vec![
            clause(
                "query",
                "query",
                "Query",
                CollectionFilterOperator::FreeText,
                "service error",
                CollectionFilterSourceClass::User,
            ),
            clause(
                "workset",
                "workset",
                "Workset",
                CollectionFilterOperator::Equals,
                "current slice",
                CollectionFilterSourceClass::Workset,
            ),
            clause(
                "policy",
                "visibility",
                "Visibility",
                CollectionFilterOperator::Equals,
                "policy redacted",
                CollectionFilterSourceClass::Policy,
            ),
        ],
    );
    let selection = CollectionSelectionState::explicit_identity_set(
        "selection:search:service-errors",
        "view:search:service-errors",
        vec!["search:item:1".to_owned(), "search:item:2".to_owned()],
        Some("search:item:1".to_owned()),
        1,
        1,
        0,
    );
    let counters = CollectionScopeCounters::from_known_values(
        50,
        200,
        1_420,
        2,
        1,
        1_221,
        12,
        1,
        CollectionCountStatus::Approximate,
    );
    let members = vec![
        member(
            "search:item:1",
            CollectionSurfaceFamily::SearchCollection,
            BatchMemberDisposition::Included,
            "Included in selected export.",
        ),
        member(
            "search:item:2",
            CollectionSurfaceFamily::SearchCollection,
            BatchMemberDisposition::Blocked,
            "Hidden by policy.",
        ),
        member(
            "search:item:hidden",
            CollectionSurfaceFamily::SearchCollection,
            BatchMemberDisposition::Hidden,
            "Outside current filter.",
        ),
    ];
    CollectionContractRow {
        row_id: "row:search:service-errors".to_owned(),
        surface_family: family,
        surface_ownership: DenseCollectionSurfaceOwnership::LocalAuthoritative,
        object_kinds: all_object_kinds(),
        saved_view: saved_view(
            "view:search:service-errors",
            "Service errors",
            filter_ast.clone(),
            vec!["path", "kind", "owner", "matches"],
        ),
        filter_ast,
        query_history: query_history(
            "history:search:service-errors",
            "local_user",
            "workspace_redacted",
            "client_local_execution",
        ),
        scope_pack: scope_pack(
            "scope:search:current-workset",
            ReopenScopePosture::CapturedScopeStillCurrent,
        ),
        column_preset: column_preset(
            "columns:search:identity",
            vec!["path", "kind", "owner", "matches"],
            vec!["path", "owner"],
        ),
        scope_counters: counters.clone(),
        scope_vocabulary_terms: REQUIRED_VOCABULARY_TERMS.to_vec(),
        selection_state: selection,
        select_all_meaning: SelectAllMeaning::ExplicitIdentitySet,
        batch_protection_posture: BatchProtectionPosture::ReviewRequiredPresent,
        batch_review: Some(batch_review(
            "batch:search:export",
            "view:search:service-errors",
            "search.export_selected",
            "Export selected matches",
            BatchActionClass::ExportOrShare,
            SelectionScopeClass::ExplicitCustomSet,
            BatchExecutionOriginClass::ClientLocalExecution,
            counters,
            members,
        )),
        export_preserves_scope_truth: true,
        support_capture_preserves_scope_truth: true,
        accessibility_inspection_available: true,
        unsafe_transient_state_excluded: true,
    }
}

fn provider_review_admin_row() -> CollectionContractRow {
    let filter_ast = filter_ast(
        "filter:admin:approval-queue",
        "provider queue current permissions",
        vec![
            clause(
                "state",
                "state",
                "State",
                CollectionFilterOperator::Equals,
                "needs approval",
                CollectionFilterSourceClass::User,
            ),
            clause(
                "provider",
                "provider_window",
                "Provider window",
                CollectionFilterOperator::Range,
                "last 7 days",
                CollectionFilterSourceClass::ProviderLimit,
            )
            .with_round_trip_state(
                FilterRoundTripState::StaleProviderData,
                "Provider returned stale cursor; rebind required",
            ),
        ],
    );
    let selection = CollectionSelectionState::select_all_scope(
        "selection:admin:approval-queue",
        "view:admin:approval-queue",
        SelectionScopeClass::ProviderSideQuery,
        238,
        19,
        6,
        4,
        Some("snapshot:admin:approval-queue:prior".to_owned()),
    );
    let counters = CollectionScopeCounters::from_known_values(
        40,
        120,
        238,
        238,
        6,
        23,
        6,
        19,
        CollectionCountStatus::ProviderLimited,
    );
    let members = vec![
        member(
            "admin:item:1",
            CollectionSurfaceFamily::AdminOrSettingsGrid,
            BatchMemberDisposition::Included,
            "Provider will apply the approval.",
        ),
        member(
            "admin:item:2",
            CollectionSurfaceFamily::AdminOrSettingsGrid,
            BatchMemberDisposition::Blocked,
            "Policy blocks approval.",
        ),
        member(
            "admin:item:3",
            CollectionSurfaceFamily::AdminOrSettingsGrid,
            BatchMemberDisposition::Stale,
            "Based on prior query snapshot.",
        ),
    ];
    CollectionContractRow {
        row_id: "row:admin:approval-queue".to_owned(),
        surface_family: DenseCollectionSurfaceFamily::ProviderReviewAdmin,
        surface_ownership: DenseCollectionSurfaceOwnership::ProviderAuthoritative,
        object_kinds: all_object_kinds(),
        saved_view: saved_view(
            "view:admin:approval-queue",
            "Provider approvals",
            filter_ast.clone(),
            vec!["identity", "provider", "actor", "state"],
        ),
        filter_ast,
        query_history: query_history(
            "history:admin:approval-queue",
            "provider_owned",
            "policy_governed",
            "provider_authoritative_execution",
        ),
        scope_pack: scope_pack(
            "scope:admin:provider-queue",
            ReopenScopePosture::BasedOnPriorQuerySnapshot,
        ),
        column_preset: column_preset(
            "columns:admin:provenance",
            vec!["identity", "provider", "actor", "state"],
            vec!["identity", "provider", "actor"],
        ),
        scope_counters: counters.clone(),
        scope_vocabulary_terms: REQUIRED_VOCABULARY_TERMS.to_vec(),
        selection_state: selection,
        select_all_meaning: SelectAllMeaning::ProviderSideQueryAfterReview,
        batch_protection_posture: BatchProtectionPosture::ReviewRequiredPresent,
        batch_review: Some(batch_review(
            "batch:admin:approve-provider",
            "view:admin:approval-queue",
            "admin.provider_approve",
            "Approve provider queue",
            BatchActionClass::ProviderOwnedMutation,
            SelectionScopeClass::ProviderSideQuery,
            BatchExecutionOriginClass::ProviderAuthoritativeExecution,
            counters,
            members,
        )),
        export_preserves_scope_truth: true,
        support_capture_preserves_scope_truth: true,
        accessibility_inspection_available: true,
        unsafe_transient_state_excluded: true,
    }
}

fn package_test_diagnostics_row() -> CollectionContractRow {
    let filter_ast = filter_ast(
        "filter:diagnostics:failing-tests",
        "test run package workspace",
        vec![
            clause(
                "status",
                "status",
                "Status",
                CollectionFilterOperator::Equals,
                "failed",
                CollectionFilterSourceClass::User,
            ),
            clause(
                "client",
                "loaded_window",
                "Loaded window",
                CollectionFilterOperator::LessOrEqual,
                "500 rows",
                CollectionFilterSourceClass::ClientLimit,
            ),
        ],
    );
    let selection = CollectionSelectionState::explicit_identity_set(
        "selection:diagnostics:failing-tests",
        "view:diagnostics:failing-tests",
        vec![
            "test:item:1".to_owned(),
            "test:item:2".to_owned(),
            "test:item:3".to_owned(),
        ],
        Some("test:item:1".to_owned()),
        1,
        0,
        0,
    );
    let counters = CollectionScopeCounters::from_known_values(
        75,
        500,
        500,
        3,
        0,
        425,
        0,
        1,
        CollectionCountStatus::Exact,
    );
    CollectionContractRow {
        row_id: "row:diagnostics:failing-tests".to_owned(),
        surface_family: DenseCollectionSurfaceFamily::PackageTestDiagnostics,
        surface_ownership: DenseCollectionSurfaceOwnership::LocalAuthoritative,
        object_kinds: all_object_kinds(),
        saved_view: saved_view(
            "view:diagnostics:failing-tests",
            "Failing test diagnostics",
            filter_ast.clone(),
            vec!["test_id", "package", "target", "failure"],
        ),
        filter_ast,
        query_history: query_history(
            "history:diagnostics:failing-tests",
            "workspace",
            "workspace_portable",
            "client_local_execution",
        ),
        scope_pack: scope_pack(
            "scope:diagnostics:current-run",
            ReopenScopePosture::CapturedScopeStillCurrent,
        ),
        column_preset: column_preset(
            "columns:diagnostics:identity",
            vec!["test_id", "package", "target", "failure"],
            vec!["test_id", "package", "target"],
        ),
        scope_counters: counters,
        scope_vocabulary_terms: REQUIRED_VOCABULARY_TERMS.to_vec(),
        selection_state: selection,
        select_all_meaning: SelectAllMeaning::ExplicitIdentitySet,
        batch_protection_posture: BatchProtectionPosture::RoutineNoReviewRequired,
        batch_review: None,
        export_preserves_scope_truth: true,
        support_capture_preserves_scope_truth: true,
        accessibility_inspection_available: true,
        unsafe_transient_state_excluded: true,
    }
}

fn notebook_data_grid_row() -> CollectionContractRow {
    let filter_ast = filter_ast(
        "filter:notebook:data-grid",
        "notebook result dataset",
        vec![
            clause(
                "contains",
                "cell_value",
                "Cell value",
                CollectionFilterOperator::Contains,
                "redacted literal",
                CollectionFilterSourceClass::User,
            ),
            clause(
                "stream",
                "streaming_window",
                "Streaming window",
                CollectionFilterOperator::Exists,
                "partial result",
                CollectionFilterSourceClass::PartialData,
            ),
        ],
    );
    let selection = CollectionSelectionState::select_all_scope(
        "selection:notebook:data-grid",
        "view:notebook:data-grid",
        SelectionScopeClass::AllMatchingQuery,
        1_240,
        640,
        0,
        12,
        Some("snapshot:notebook:data-grid:stream-17".to_owned()),
    );
    let counters = CollectionScopeCounters::from_known_values(
        100,
        600,
        1_240,
        1_240,
        0,
        640,
        0,
        640,
        CollectionCountStatus::Partial,
    );
    let members = vec![
        member(
            "grid:item:1",
            CollectionSurfaceFamily::PackageOrInventoryGrid,
            BatchMemberDisposition::Included,
            "Copied from reviewed query snapshot.",
        ),
        member(
            "grid:item:2",
            CollectionSurfaceFamily::PackageOrInventoryGrid,
            BatchMemberDisposition::Hidden,
            "Outside current filter after streaming refresh.",
        ),
        member(
            "grid:item:3",
            CollectionSurfaceFamily::PackageOrInventoryGrid,
            BatchMemberDisposition::Stale,
            "Prior query snapshot row.",
        ),
    ];
    CollectionContractRow {
        row_id: "row:notebook:data-grid".to_owned(),
        surface_family: DenseCollectionSurfaceFamily::NotebookDataGrid,
        surface_ownership: DenseCollectionSurfaceOwnership::MixedClientProvider,
        object_kinds: all_object_kinds(),
        saved_view: saved_view(
            "view:notebook:data-grid",
            "Notebook result export",
            filter_ast.clone(),
            vec!["row_id", "dataset", "source_cell", "value"],
        ),
        filter_ast,
        query_history: query_history(
            "history:notebook:data-grid",
            "local_user",
            "local_redacted",
            "mixed_client_then_provider",
        ),
        scope_pack: scope_pack(
            "scope:notebook:streaming-dataset",
            ReopenScopePosture::BasedOnPriorQuerySnapshot,
        ),
        column_preset: column_preset(
            "columns:notebook:data-grid",
            vec!["row_id", "dataset", "source_cell", "value"],
            vec!["row_id", "dataset", "source_cell"],
        ),
        scope_counters: counters.clone(),
        scope_vocabulary_terms: REQUIRED_VOCABULARY_TERMS.to_vec(),
        selection_state: selection,
        select_all_meaning: SelectAllMeaning::AllMatchingAfterExplicitExpansion,
        batch_protection_posture: BatchProtectionPosture::ReviewRequiredPresent,
        batch_review: Some(batch_review(
            "batch:notebook:copy-export",
            "view:notebook:data-grid",
            "notebook.grid_export",
            "Export matching data rows",
            BatchActionClass::ExportOrShare,
            SelectionScopeClass::AllMatchingQuery,
            BatchExecutionOriginClass::MixedClientThenProvider,
            counters,
            members,
        )),
        export_preserves_scope_truth: true,
        support_capture_preserves_scope_truth: true,
        accessibility_inspection_available: true,
        unsafe_transient_state_excluded: true,
    }
}

fn filter_ast(
    filter_ast_id: &str,
    scope_label: &str,
    clauses: Vec<CollectionFilterClause>,
) -> CollectionFilterAst {
    CollectionFilterAst::from_clauses(
        filter_ast_id,
        scope_label,
        clauses,
        "aureline.collections.stable_contract",
        "2026-06-04T00:00:00Z",
    )
}

fn clause(
    clause_id: &str,
    field_id: &str,
    label: &str,
    operator: CollectionFilterOperator,
    display_value: &str,
    source_class: CollectionFilterSourceClass,
) -> CollectionFilterClause {
    CollectionFilterClause::new(
        clause_id,
        field_id,
        label,
        operator,
        Some(CollectionFilterLiteral::redacted(display_value)),
        source_class,
    )
}

fn saved_view(
    saved_view_id: &str,
    name: &str,
    filter_ast: CollectionFilterAst,
    columns: Vec<&str>,
) -> SavedCollectionView {
    SavedCollectionView::new(
        saved_view_id,
        name,
        SavedViewOwnerScope::Workspace,
        SavedViewPrivacyClass::SharedRedacted,
        SavedViewFallbackBehavior::PreserveAndLabelDegraded,
        filter_ast,
        columns.iter().map(|column| (*column).to_owned()).collect(),
        columns
            .iter()
            .take(2)
            .map(|column| (*column).to_owned())
            .collect(),
        "2026-06-04T00:00:00Z",
    )
    .with_sort_keys(vec![CollectionSortKey {
        field_id: columns[0].to_owned(),
        descending: false,
    }])
}

fn query_history(
    query_history_id: &str,
    owner_source_class: &str,
    privacy_class: &str,
    execution_origin: &str,
) -> CollectionContractQueryHistory {
    CollectionContractQueryHistory {
        query_history_id: query_history_id.to_owned(),
        owner_source_class: owner_source_class.to_owned(),
        privacy_class: privacy_class.to_owned(),
        schema_version: COLLECTION_CONTRACT_SCHEMA_VERSION,
        execution_origin: execution_origin.to_owned(),
        raw_query_material_excluded: true,
        retention_label: "portable_metadata_only".to_owned(),
    }
}

fn scope_pack(
    scope_pack_id: &str,
    reopen_posture: ReopenScopePosture,
) -> CollectionContractScopePack {
    CollectionContractScopePack {
        scope_pack_id: scope_pack_id.to_owned(),
        captured_scope_identity: format!("{scope_pack_id}:captured"),
        current_scope_identity: format!("{scope_pack_id}:current"),
        reopen_posture,
        include_rules: vec!["current workspace roots".to_owned()],
        exclude_rules: vec!["policy-hidden rows remain counted".to_owned()],
        missing_scope_reasons: if matches!(
            reopen_posture,
            ReopenScopePosture::CapturedScopeStillCurrent
        ) {
            Vec::new()
        } else {
            vec!["provider, policy, or streaming basis changed since capture".to_owned()]
        },
    }
}

fn column_preset(
    column_preset_id: &str,
    visible: Vec<&str>,
    required: Vec<&str>,
) -> CollectionContractColumnPreset {
    CollectionContractColumnPreset {
        column_preset_id: column_preset_id.to_owned(),
        visible_column_ids: visible.iter().map(|column| (*column).to_owned()).collect(),
        pinned_column_ids: required.iter().map(|column| (*column).to_owned()).collect(),
        required_column_ids: required.iter().map(|column| (*column).to_owned()).collect(),
        density_mode: "compact".to_owned(),
        fallback_behavior: "preserve and label degraded; offer reset or migration".to_owned(),
    }
}

fn member(
    stable_item_id: &str,
    surface_family: CollectionSurfaceFamily,
    disposition: BatchMemberDisposition,
    reason_label: &str,
) -> BatchReviewMember {
    let item = StableCollectionItemRef::new(
        stable_item_id,
        surface_family,
        format!("source:{stable_item_id}"),
        stable_item_id,
    )
    .with_blocked(disposition == BatchMemberDisposition::Blocked)
    .with_hidden(disposition == BatchMemberDisposition::Hidden)
    .with_stale(disposition == BatchMemberDisposition::Stale);
    BatchReviewMember {
        item,
        disposition,
        reason_label: reason_label.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn batch_review(
    batch_review_id: &str,
    collection_view_id_ref: &str,
    action_id: &str,
    action_label: &str,
    action_class: BatchActionClass,
    selection_scope_class: SelectionScopeClass,
    execution_origin_class: BatchExecutionOriginClass,
    counters: CollectionScopeCounters,
    members: Vec<BatchReviewMember>,
) -> BatchReviewSheet {
    BatchReviewSheet::from_members(
        batch_review_id,
        collection_view_id_ref,
        action_id,
        action_label,
        action_class,
        selection_scope_class,
        execution_origin_class,
        counters,
        members,
        "Use post-run results to retry failed members, reopen blocked rows, or re-run from a fresh query snapshot.",
    )
    .with_aftermath_summary(BatchAftermathSummary {
        succeeded_count: 1,
        failed_count: 1,
        skipped_count: 1,
        blocked_count: 1,
        summary_label: "Mixed outcome preserved per item: succeeded, failed, skipped, and blocked members remain reviewable.".to_owned(),
    })
}

fn all_object_kinds() -> Vec<StableDenseCollectionObjectKind> {
    vec![
        StableDenseCollectionObjectKind::FilterAst,
        StableDenseCollectionObjectKind::SavedView,
        StableDenseCollectionObjectKind::QueryHistory,
        StableDenseCollectionObjectKind::ScopePack,
        StableDenseCollectionObjectKind::ColumnPreset,
        StableDenseCollectionObjectKind::ScopeCounters,
        StableDenseCollectionObjectKind::SelectionState,
        StableDenseCollectionObjectKind::BatchReviewSheet,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_packet_is_stable() {
        let packet = seeded_dense_collection_contract_packet();
        assert!(packet.validate().is_empty());
        assert_eq!(
            packet.promotion_state,
            CollectionContractPromotionState::Stable
        );
    }

    #[test]
    fn checked_in_packet_is_valid() {
        let packet = current_stable_dense_collection_contract_packet()
            .expect("checked-in stable packet should validate");
        assert!(packet
            .support_export("export:test", "2026-06-04T00:00:00Z")
            .is_export_safe());
    }

    #[test]
    fn vocabulary_tokens_are_pinned() {
        assert_eq!(ScopeCounterVocabularyTerm::Visible.as_str(), "visible");
        assert_eq!(ScopeCounterVocabularyTerm::Loaded.as_str(), "loaded");
        assert_eq!(ScopeCounterVocabularyTerm::Matching.as_str(), "matching");
        assert_eq!(ScopeCounterVocabularyTerm::Selected.as_str(), "selected");
        assert_eq!(ScopeCounterVocabularyTerm::Approx.as_str(), "approx.");
        assert_eq!(ScopeCounterVocabularyTerm::Exact.as_str(), "exact");
        assert_eq!(
            ScopeCounterVocabularyTerm::HiddenByPolicy.as_str(),
            "hidden by policy"
        );
        assert_eq!(
            ScopeCounterVocabularyTerm::OutsideCurrentFilter.as_str(),
            "outside current filter"
        );
    }

    #[test]
    fn missing_data_grid_blocks_stable() {
        let mut packet = seeded_dense_collection_contract_packet();
        packet
            .rows
            .retain(|row| row.surface_family != DenseCollectionSurfaceFamily::NotebookDataGrid);
        let findings = packet.validate();
        assert!(findings.iter().any(|finding| {
            finding.finding_kind == CollectionContractFindingKind::MissingRequiredSurface
        }));
    }

    #[test]
    fn protected_action_without_review_blocks_stable() {
        let mut packet = seeded_dense_collection_contract_packet();
        packet.rows[1].batch_review = None;
        let findings = packet.validate();
        assert!(findings.iter().any(|finding| {
            finding.finding_kind == CollectionContractFindingKind::BatchReviewRequiredButMissing
        }));
    }
}
