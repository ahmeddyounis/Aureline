//! Stable saved-query / filter-AST / scope-pack / column-preset portability
//! and collection-truth packet for dense result surfaces.
//!
//! This module is the search-owned contract for the M4 stable lane that ships
//! one export-safe truth packet per saved-search workflow. The packet binds
//! the typed filter AST, the saved view + column-preset, the scope-pack
//! binding, the query-history entry, the scope counters, and the
//! batch-review sheet to a single identity space so the desktop and companion
//! shells, CLI/headless inspector, support export, docs/help surface, AI
//! context inspector, and release proof index all read the same packet
//! instead of reconstructing portability truth from raw saved-view material.
//!
//! The packet is intentionally metadata-only — it carries no raw source
//! bodies, no secrets, and no ambient credentials — and pins the closed
//! vocabularies that dense surfaces depend on:
//!
//! - [`CollectionPortabilityRow`] joins a [`SavedQuery`], [`QueryHistoryEntry`],
//!   [`ScopePackBinding`], [`SavedCollectionView`] (which carries the
//!   [`CollectionFilterAst`] and column-preset bindings), a
//!   [`CollectionScopeCounters`] record, an optional [`BatchReviewSheet`],
//!   and the closed reopen-state vocabulary.
//! - [`CollectionPortabilityConsumerProjection`] proves that a dense lane
//!   (desktop search shell, companion shell, CLI/headless inspector, AI
//!   context inspector, docs/help, support export, release proof index)
//!   preserves the packet verbatim with the full filter-AST, saved-view,
//!   column-preset, scope-counter, scope-pack, and batch-review vocabulary.
//! - The packet glues those rows together, derives a closed promotion state
//!   from validation findings, and refuses to certify when any required
//!   lane drops the closed vocabulary or admits raw boundary material.

use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::collections::{
    BatchReviewSheet, CollectionCountTerm, CollectionScopeCounters, CollectionSelectionState,
    SavedCollectionView, SelectionScopeClass,
};
use crate::query_artifacts::{
    QueryHistoryEntry, SavedQuery, ScopePackBinding, SearchArtifactMigrationState,
    SearchResultSemantics, SearchScopeHonestyState,
};

/// Stable record-kind tag for [`CollectionPortabilityTruthPacket`].
pub const COLLECTION_PORTABILITY_TRUTH_PACKET_RECORD_KIND: &str =
    "collection_portability_truth_stable_packet";

/// Stable record-kind tag for [`CollectionPortabilityTruthSupportExport`].
pub const COLLECTION_PORTABILITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "collection_portability_truth_support_export";

/// Integer schema version for the stable portability packet.
pub const COLLECTION_PORTABILITY_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const COLLECTION_PORTABILITY_TRUTH_SCHEMA_REF: &str =
    "schemas/search/collection_portability_truth_packet.schema.json";

/// Repo-relative path of the reviewer doc.
pub const COLLECTION_PORTABILITY_TRUTH_DOC_REF: &str =
    "docs/search/m4/collection_portability_truth_packet.md";

/// Repo-relative path of the human-readable artifact narrative.
pub const COLLECTION_PORTABILITY_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/search/m4/collection_portability_truth_packet.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const COLLECTION_PORTABILITY_TRUTH_FIXTURE_DIR: &str =
    "fixtures/search/m4/collection_portability_truth_packet";

/// Repo-relative path of the checked-in stable truth packet.
pub const COLLECTION_PORTABILITY_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/search/m4/collection_portability_truth_packet.json";

/// Closed promotion state for [`CollectionPortabilityTruthPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionPortabilityPromotionState {
    /// Packet certifies a stable claim.
    Stable,
    /// Packet must remain narrowed below stable until a recorded gap closes.
    NarrowedBelowStable,
    /// Packet has a blocker finding and cannot publish on stable surfaces.
    BlocksStable,
}

impl CollectionPortabilityPromotionState {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity for one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionPortabilityFindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the packet below stable.
    Warning,
    /// Blocker that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for the packet validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionPortabilityFindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// A row's embedded saved query fails its own validation.
    InvalidSavedQuery,
    /// A row's embedded query-history entry fails its own validation.
    InvalidQueryHistory,
    /// A row's embedded scope-pack binding fails its own validation.
    InvalidScopePackBinding,
    /// A row's embedded saved view fails its own portability validation.
    InvalidSavedView,
    /// A row's embedded batch-review sheet fails its own validation.
    InvalidBatchReview,
    /// A row's saved view cites a different filter-AST id than the row's filter AST.
    FilterAstIdMismatch,
    /// A row's saved query, history entry, and scope binding disagree on the scope binding id.
    ScopePackBindingMismatch,
    /// A row's saved query and history entry disagree on the captured stable scope id.
    StableScopeIdMismatch,
    /// A row's saved view drops a required column-preset id.
    MissingColumnPresetRef,
    /// A row's column preset omits a column the saved view pins as visible/required.
    ColumnPresetDropsRequiredColumn,
    /// A row's scope counters omit one of the required count terms.
    ScopeCounterVocabularyDropped,
    /// A row's batch-review sheet is required by the action class but missing.
    BatchReviewRequiredButMissing,
    /// A row's batch-review sheet drops execution-origin truth.
    BatchReviewExecutionOriginDropped,
    /// A row's batch-review sheet drops rollback / recovery guidance.
    BatchReviewRollbackGuidanceMissing,
    /// A row claims `current_live_results` for a captured/reopen artifact (it must rerun).
    CapturedArtifactClaimsLiveResults,
    /// A row's reopen state collapses captured-vs-current scope honesty into a green badge.
    ScopeHonestyStateCollapsed,
    /// A row's reopen drops the migration vocabulary required for portable replay.
    MigrationStateDropped,
    /// Packet `covered_reopen_states` drops a reopen state carried by a row.
    ReopenStateCoverageDropped,
    /// Packet `covered_reopen_states` declares a reopen state no row carries.
    ReopenStateCoverageOverDeclared,
    /// Packet `covered_surfaces` drops a surface family carried by a row.
    SurfaceFamilyCoverageDropped,
    /// Packet `covered_surfaces` declares a surface family no row carries.
    SurfaceFamilyCoverageOverDeclared,
    /// A required consumer projection is missing (desktop / companion / cli / export / etc.).
    MissingConsumerProjection,
    /// A consumer projection drops or reminted truth (packet id, filter ast, saved view, etc.).
    ConsumerProjectionDrift,
    /// A consumer projection drops the filter-AST vocabulary.
    ProjectionFilterAstDropped,
    /// A consumer projection drops the saved-view vocabulary.
    ProjectionSavedViewDropped,
    /// A consumer projection drops the scope-pack binding vocabulary.
    ProjectionScopePackDropped,
    /// A consumer projection drops the column-preset vocabulary.
    ProjectionColumnPresetDropped,
    /// A consumer projection drops the scope-counter vocabulary.
    ProjectionScopeCountersDropped,
    /// A consumer projection drops the batch-review vocabulary on review-required rows.
    ProjectionBatchReviewDropped,
    /// A consumer projection drops the query-history vocabulary.
    ProjectionQueryHistoryDropped,
    /// A consumer projection drops the scope-honesty / reopen vocabulary.
    ProjectionScopeHonestyDropped,
    /// A row or projection admits raw source bodies, secrets, or ambient credentials.
    RawBoundaryMaterialPresent,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl CollectionPortabilityFindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::InvalidSavedQuery => "invalid_saved_query",
            Self::InvalidQueryHistory => "invalid_query_history",
            Self::InvalidScopePackBinding => "invalid_scope_pack_binding",
            Self::InvalidSavedView => "invalid_saved_view",
            Self::InvalidBatchReview => "invalid_batch_review",
            Self::FilterAstIdMismatch => "filter_ast_id_mismatch",
            Self::ScopePackBindingMismatch => "scope_pack_binding_mismatch",
            Self::StableScopeIdMismatch => "stable_scope_id_mismatch",
            Self::MissingColumnPresetRef => "missing_column_preset_ref",
            Self::ColumnPresetDropsRequiredColumn => "column_preset_drops_required_column",
            Self::ScopeCounterVocabularyDropped => "scope_counter_vocabulary_dropped",
            Self::BatchReviewRequiredButMissing => "batch_review_required_but_missing",
            Self::BatchReviewExecutionOriginDropped => "batch_review_execution_origin_dropped",
            Self::BatchReviewRollbackGuidanceMissing => "batch_review_rollback_guidance_missing",
            Self::CapturedArtifactClaimsLiveResults => "captured_artifact_claims_live_results",
            Self::ScopeHonestyStateCollapsed => "scope_honesty_state_collapsed",
            Self::MigrationStateDropped => "migration_state_dropped",
            Self::ReopenStateCoverageDropped => "reopen_state_coverage_dropped",
            Self::ReopenStateCoverageOverDeclared => "reopen_state_coverage_over_declared",
            Self::SurfaceFamilyCoverageDropped => "surface_family_coverage_dropped",
            Self::SurfaceFamilyCoverageOverDeclared => "surface_family_coverage_over_declared",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::ProjectionFilterAstDropped => "projection_filter_ast_dropped",
            Self::ProjectionSavedViewDropped => "projection_saved_view_dropped",
            Self::ProjectionScopePackDropped => "projection_scope_pack_dropped",
            Self::ProjectionColumnPresetDropped => "projection_column_preset_dropped",
            Self::ProjectionScopeCountersDropped => "projection_scope_counters_dropped",
            Self::ProjectionBatchReviewDropped => "projection_batch_review_dropped",
            Self::ProjectionQueryHistoryDropped => "projection_query_history_dropped",
            Self::ProjectionScopeHonestyDropped => "projection_scope_honesty_dropped",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Closed reopen-posture vocabulary for portability rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionPortabilityReopenState {
    /// Captured scope still matches current scope; the saved view replays exactly.
    CapturedScopeStillCurrent,
    /// Recipient must re-resolve under their own current scope and permissions.
    RecipientMustReResolve,
    /// Current scope is wider; the reopen narrows back to the captured scope.
    CurrentScopeWiderNarrowedToCaptured,
    /// Current scope is narrower; the reopen discloses the reduction.
    CurrentScopeNarrowerDisclosed,
    /// Current scope changed laterally; the user must rebind before replay.
    CurrentScopeChangedRebindRequired,
    /// An incompatible saved view or filter-AST must migrate or reset before replay.
    IncompatibleArtifactMigrationRequired,
}

impl CollectionPortabilityReopenState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapturedScopeStillCurrent => "captured_scope_still_current",
            Self::RecipientMustReResolve => "recipient_must_re_resolve",
            Self::CurrentScopeWiderNarrowedToCaptured => "current_scope_wider_narrowed_to_captured",
            Self::CurrentScopeNarrowerDisclosed => "current_scope_narrower_disclosed",
            Self::CurrentScopeChangedRebindRequired => "current_scope_changed_rebind_required",
            Self::IncompatibleArtifactMigrationRequired => {
                "incompatible_artifact_migration_required"
            }
        }
    }

    /// Projects the durable scope-honesty state from the saved-query layer
    /// into this packet's reopen-state vocabulary. Migration-required rows
    /// are still marked separately via [`CollectionPortabilityRow::reopen_state`].
    pub const fn from_scope_honesty(state: SearchScopeHonestyState) -> Self {
        match state {
            SearchScopeHonestyState::CapturedScopeStillCurrent => Self::CapturedScopeStillCurrent,
            SearchScopeHonestyState::RecipientMustReResolve => Self::RecipientMustReResolve,
            SearchScopeHonestyState::CurrentScopeWiderNarrowedToCaptured => {
                Self::CurrentScopeWiderNarrowedToCaptured
            }
            SearchScopeHonestyState::CurrentScopeNarrowerDisclosed => {
                Self::CurrentScopeNarrowerDisclosed
            }
            SearchScopeHonestyState::CurrentScopeChangedRebindRequired => {
                Self::CurrentScopeChangedRebindRequired
            }
        }
    }
}

/// Consumer surface that must inherit the packet's truth verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionPortabilityConsumerSurface {
    /// Desktop search shell / dense result pane.
    DesktopSearchShell,
    /// Companion (web/mobile) search shell.
    CompanionSearchShell,
    /// CLI / headless dense-collection inspector.
    CliHeadless,
    /// AI context inspector / picker.
    AiContextInspector,
    /// Docs / help surface explaining saved-view portability.
    DocsHelp,
    /// Support export bundle.
    SupportExport,
    /// Release proof index entry.
    ReleaseProofIndex,
}

impl CollectionPortabilityConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 7] = [
        Self::DesktopSearchShell,
        Self::CompanionSearchShell,
        Self::CliHeadless,
        Self::AiContextInspector,
        Self::DocsHelp,
        Self::SupportExport,
        Self::ReleaseProofIndex,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopSearchShell => "desktop_search_shell",
            Self::CompanionSearchShell => "companion_search_shell",
            Self::CliHeadless => "cli_headless",
            Self::AiContextInspector => "ai_context_inspector",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
        }
    }
}

/// Required count terms a portability row's scope counters must carry.
const REQUIRED_COUNT_TERMS: [CollectionCountTerm; 8] = [
    CollectionCountTerm::Visible,
    CollectionCountTerm::Loaded,
    CollectionCountTerm::AllMatching,
    CollectionCountTerm::Selected,
    CollectionCountTerm::Blocked,
    CollectionCountTerm::Hidden,
    CollectionCountTerm::HiddenByPolicy,
    CollectionCountTerm::HiddenByFilter,
];

/// One validation finding emitted by the packet validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionPortabilityValidationFinding {
    /// Closed finding kind.
    pub finding_kind: CollectionPortabilityFindingKind,
    /// Finding severity.
    pub severity: CollectionPortabilityFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl CollectionPortabilityValidationFinding {
    fn new(
        finding_kind: CollectionPortabilityFindingKind,
        severity: CollectionPortabilityFindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// Column-preset binding pinned by a saved view inside a portability row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionPortabilityColumnPreset {
    /// Stable column-preset id.
    pub column_preset_id: String,
    /// Visible column ids in display order.
    pub visible_column_ids: Vec<String>,
    /// Pinned column ids that cannot be silently hidden.
    pub pinned_column_ids: Vec<String>,
    /// Column ids that the saved view declares required (must remain visible).
    #[serde(default)]
    pub required_column_ids: Vec<String>,
    /// True when the preset travels portably (no local-only material).
    pub portable: bool,
}

/// One row binding a saved query, history entry, scope pack, saved view,
/// column preset, scope counters, and an optional batch-review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionPortabilityRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Embedded saved query.
    pub saved_query: SavedQuery,
    /// Embedded query-history entry.
    pub query_history: QueryHistoryEntry,
    /// Embedded scope-pack binding.
    pub scope_pack_binding: ScopePackBinding,
    /// Embedded saved view (which carries the filter AST).
    pub saved_view: SavedCollectionView,
    /// Embedded column-preset binding.
    pub column_preset: CollectionPortabilityColumnPreset,
    /// Embedded scope counters.
    pub scope_counters: CollectionScopeCounters,
    /// Embedded selection state for the reopen surface.
    pub selection_state: CollectionSelectionState,
    /// Optional embedded batch-review sheet for consequential bulk actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch_review: Option<BatchReviewSheet>,
    /// Closed reopen-posture vocabulary.
    pub reopen_state: CollectionPortabilityReopenState,
    /// True when raw source bodies, secrets, and ambient credentials are excluded.
    pub raw_boundary_material_excluded: bool,
    /// Short display title safe to surface alongside the row.
    pub display_title: String,
    /// Capture timestamp for this row.
    pub captured_at: String,
}

impl CollectionPortabilityRow {
    fn scope_counter_terms(&self) -> HashSet<CollectionCountTerm> {
        self.scope_counters
            .values()
            .iter()
            .map(|value| value.term)
            .collect()
    }

    fn missing_required_columns(&self) -> Vec<String> {
        self.column_preset
            .required_column_ids
            .iter()
            .filter(|id| !self.column_preset.visible_column_ids.contains(id))
            .cloned()
            .collect()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionPortabilityConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: CollectionPortabilityConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Truth packet id consumed by the projection.
    pub export_packet_id_ref: String,
    /// Render timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id verbatim.
    pub preserves_same_packet: bool,
    /// True when the surface preserves the filter-AST vocabulary verbatim.
    pub preserves_filter_ast: bool,
    /// True when the surface preserves the saved-view vocabulary verbatim.
    pub preserves_saved_view: bool,
    /// True when the surface preserves the scope-pack binding vocabulary verbatim.
    pub preserves_scope_pack: bool,
    /// True when the surface preserves the column-preset vocabulary verbatim.
    pub preserves_column_preset: bool,
    /// True when the surface preserves the scope-counter vocabulary verbatim.
    pub preserves_scope_counters: bool,
    /// True when the surface preserves batch-review truth for review-required rows.
    pub preserves_batch_review: bool,
    /// True when the surface preserves the query-history vocabulary verbatim.
    pub preserves_query_history: bool,
    /// True when the surface preserves the reopen / scope-honesty vocabulary verbatim.
    pub preserves_scope_honesty: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority / credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl CollectionPortabilityConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.export_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_filter_ast
            && self.preserves_saved_view
            && self.preserves_scope_pack
            && self.preserves_column_preset
            && self.preserves_scope_counters
            && self.preserves_batch_review
            && self.preserves_query_history
            && self.preserves_scope_honesty
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`CollectionPortabilityTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionPortabilityTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Query session that produced the packet.
    pub query_session_id_ref: String,
    /// Capture timestamp for the packet as a whole.
    pub generated_at: String,
    /// Closed reopen states covered by this packet.
    #[serde(default)]
    pub covered_reopen_states: Vec<CollectionPortabilityReopenState>,
    /// Closed surface families covered by this packet (from saved-view surface families).
    #[serde(default)]
    pub covered_surfaces: Vec<crate::collections::CollectionSurfaceFamily>,
    /// Rows joining saved queries, scope packs, saved views, and counters.
    #[serde(default)]
    pub rows: Vec<CollectionPortabilityRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<CollectionPortabilityConsumerProjection>,
    /// Source contract refs (docs / schema / fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Search-owned packet for saved-query / filter-AST / scope-pack /
/// column-preset portability and collection truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionPortabilityTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Query session that produced the packet.
    pub query_session_id_ref: String,
    /// Packet capture timestamp.
    pub generated_at: String,
    /// Closed reopen states covered by this packet.
    #[serde(default)]
    pub covered_reopen_states: Vec<CollectionPortabilityReopenState>,
    /// Closed surface families covered by this packet.
    #[serde(default)]
    pub covered_surfaces: Vec<crate::collections::CollectionSurfaceFamily>,
    /// Rows joining saved queries, scope packs, saved views, and counters.
    #[serde(default)]
    pub rows: Vec<CollectionPortabilityRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<CollectionPortabilityConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: CollectionPortabilityPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<CollectionPortabilityValidationFinding>,
}

impl CollectionPortabilityTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: CollectionPortabilityTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: COLLECTION_PORTABILITY_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: COLLECTION_PORTABILITY_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            query_session_id_ref: input.query_session_id_ref,
            generated_at: input.generated_at,
            covered_reopen_states: input.covered_reopen_states,
            covered_surfaces: input.covered_surfaces,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: CollectionPortabilityPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against stable invariants.
    pub fn validate(&self) -> Vec<CollectionPortabilityValidationFinding> {
        self.derived_findings(true)
    }

    /// True when no blocker finding fires.
    pub fn is_stable(&self) -> bool {
        !self.validate().iter().any(|finding| {
            finding.severity == CollectionPortabilityFindingSeverity::Blocker
        })
    }

    /// True when a consumer projection preserves this packet.
    pub fn has_projection_for(
        &self,
        surface: CollectionPortabilityConsumerSurface,
    ) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique reopen-state tokens carried across rows, sorted.
    pub fn reopen_state_tokens(&self) -> Vec<&'static str> {
        let mut set = HashSet::new();
        for row in &self.rows {
            set.insert(row.reopen_state);
        }
        let mut tokens: Vec<&'static str> = set
            .into_iter()
            .map(CollectionPortabilityReopenState::as_str)
            .collect();
        tokens.sort_unstable();
        tokens
    }

    /// Returns the surface-family tokens carried by this packet's rows, sorted.
    ///
    /// Rows project into the search-collection family by default; consumer
    /// packets may declare additional families via `covered_surfaces`.
    pub fn surface_family_tokens(&self) -> Vec<&'static str> {
        if self.rows.is_empty() {
            return Vec::new();
        }
        let mut tokens = vec![crate::collections::CollectionSurfaceFamily::SearchCollection.as_str()];
        tokens.sort_unstable();
        tokens
    }

    /// Builds a support export wrapping the exact product packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> CollectionPortabilityTruthSupportExport {
        CollectionPortabilityTruthSupportExport {
            record_kind: COLLECTION_PORTABILITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: COLLECTION_PORTABILITY_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            export_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            export_packet: self.clone(),
        }
    }

    fn derived_findings(
        &self,
        include_record_fields: bool,
    ) -> Vec<CollectionPortabilityValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != COLLECTION_PORTABILITY_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(CollectionPortabilityValidationFinding::new(
                CollectionPortabilityFindingKind::WrongRecordKind,
                CollectionPortabilityFindingSeverity::Blocker,
                "collection portability truth packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != COLLECTION_PORTABILITY_TRUTH_SCHEMA_VERSION
        {
            findings.push(CollectionPortabilityValidationFinding::new(
                CollectionPortabilityFindingKind::WrongSchemaVersion,
                CollectionPortabilityFindingSeverity::Blocker,
                "collection portability truth packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.query_session_id_ref.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(CollectionPortabilityValidationFinding::new(
                CollectionPortabilityFindingKind::MissingIdentity,
                CollectionPortabilityFindingSeverity::Blocker,
                "packet, workflow, session, and timestamp refs are required",
            ));
        }

        if self.rows.is_empty() {
            findings.push(CollectionPortabilityValidationFinding::new(
                CollectionPortabilityFindingKind::MissingIdentity,
                CollectionPortabilityFindingSeverity::Blocker,
                "packet must include at least one row",
            ));
        }

        let mut row_reopen_states: HashSet<CollectionPortabilityReopenState> = HashSet::new();
        let mut row_surfaces: HashSet<crate::collections::CollectionSurfaceFamily> = HashSet::new();

        for row in &self.rows {
            if row.row_id.trim().is_empty()
                || row.display_title.trim().is_empty()
                || row.captured_at.trim().is_empty()
            {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::MissingIdentity,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "row {} identity, display title, or capture timestamp is empty",
                        row.row_id
                    ),
                ));
            }
            if !row.raw_boundary_material_excluded {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::RawBoundaryMaterialPresent,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "row {} admits raw source bodies, secrets, or destination contents",
                        row.row_id
                    ),
                ));
            }

            let saved_query_findings = row.saved_query.validate();
            if !saved_query_findings.is_empty() {
                let tokens = saved_query_findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::InvalidSavedQuery,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!("row {} saved query invalid: {tokens}", row.row_id),
                ));
            }

            let history_findings = row.query_history.validate();
            if !history_findings.is_empty() {
                let tokens = history_findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::InvalidQueryHistory,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!("row {} query history invalid: {tokens}", row.row_id),
                ));
            }

            let scope_pack_findings = row.scope_pack_binding.validate();
            if !scope_pack_findings.is_empty() {
                let tokens = scope_pack_findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::InvalidScopePackBinding,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!("row {} scope pack binding invalid: {tokens}", row.row_id),
                ));
            }

            let view_findings = row.saved_view.validate_portability();
            if !view_findings.is_empty() {
                let tokens = view_findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::InvalidSavedView,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!("row {} saved view invalid: {tokens}", row.row_id),
                ));
            }

            if let Some(batch_review) = row.batch_review.as_ref() {
                let batch_findings = batch_review.validate();
                if !batch_findings.is_empty() {
                    let tokens = batch_findings
                        .iter()
                        .map(|finding| finding.finding_kind.as_str())
                        .collect::<Vec<_>>()
                        .join(",");
                    findings.push(CollectionPortabilityValidationFinding::new(
                        CollectionPortabilityFindingKind::InvalidBatchReview,
                        CollectionPortabilityFindingSeverity::Blocker,
                        format!("row {} batch review invalid: {tokens}", row.row_id),
                    ));
                }
                if batch_review.review_required
                    && batch_review.recovery_guidance.trim().is_empty()
                {
                    findings.push(CollectionPortabilityValidationFinding::new(
                        CollectionPortabilityFindingKind::BatchReviewRollbackGuidanceMissing,
                        CollectionPortabilityFindingSeverity::Blocker,
                        format!(
                            "row {} batch review sheet drops rollback / recovery guidance",
                            row.row_id
                        ),
                    ));
                }
                if batch_review.review_required
                    && batch_review.execution_origin_class.as_str().is_empty()
                {
                    findings.push(CollectionPortabilityValidationFinding::new(
                        CollectionPortabilityFindingKind::BatchReviewExecutionOriginDropped,
                        CollectionPortabilityFindingSeverity::Blocker,
                        format!(
                            "row {} batch review sheet drops execution origin truth",
                            row.row_id
                        ),
                    ));
                }
            } else if row.selection_state.scope_class == SelectionScopeClass::AllMatchingQuery
                && row.selection_state.selected_count > 0
            {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::BatchReviewRequiredButMissing,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "row {} selects an all-matching scope without a batch review sheet",
                        row.row_id
                    ),
                ));
            }

            if row.saved_view.filter_ast.filter_ast_id != filter_ast_id_for(row) {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::FilterAstIdMismatch,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "row {} saved view filter ast id drifted from canonical id",
                        row.row_id
                    ),
                ));
            }

            if row.saved_query.scope_binding_id_ref != row.scope_pack_binding.scope_binding_id
                || row.query_history.scope_binding_id_ref
                    != row.scope_pack_binding.scope_binding_id
            {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::ScopePackBindingMismatch,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "row {} saved query / history / scope-pack disagree on scope binding id",
                        row.row_id
                    ),
                ));
            }

            if row.saved_query.stable_scope_id != row.scope_pack_binding.captured_stable_scope_id
                || row.query_history.stable_scope_id
                    != row.scope_pack_binding.captured_stable_scope_id
            {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::StableScopeIdMismatch,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "row {} saved query / history / scope-pack disagree on stable scope id",
                        row.row_id
                    ),
                ));
            }

            if row.column_preset.column_preset_id.trim().is_empty() {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::MissingColumnPresetRef,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "row {} saved view does not bind a column preset id",
                        row.row_id
                    ),
                ));
            }
            let missing_required_columns = row.missing_required_columns();
            if !missing_required_columns.is_empty() {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::ColumnPresetDropsRequiredColumn,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "row {} column preset drops required columns: {}",
                        row.row_id,
                        missing_required_columns.join(",")
                    ),
                ));
            }

            let scope_terms = row.scope_counter_terms();
            for required in REQUIRED_COUNT_TERMS {
                if !scope_terms.contains(&required) {
                    findings.push(CollectionPortabilityValidationFinding::new(
                        CollectionPortabilityFindingKind::ScopeCounterVocabularyDropped,
                        CollectionPortabilityFindingSeverity::Blocker,
                        format!(
                            "row {} scope counters drop {}",
                            row.row_id,
                            required.as_str()
                        ),
                    ));
                }
            }

            if matches!(
                row.saved_query.result_semantics,
                SearchResultSemantics::CurrentLiveResults
            ) || matches!(
                row.query_history.result_semantics,
                SearchResultSemantics::CurrentLiveResults
            ) {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::CapturedArtifactClaimsLiveResults,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "row {} captured artifact claims current_live_results without rerun",
                        row.row_id
                    ),
                ));
            }

            let saved_query_state =
                CollectionPortabilityReopenState::from_scope_honesty(
                    row.saved_query.scope_honesty_state,
                );
            let history_state =
                CollectionPortabilityReopenState::from_scope_honesty(
                    row.query_history.scope_honesty_state,
                );
            let migration_required = !matches!(
                row.saved_query.migration_state,
                SearchArtifactMigrationState::Current
            );
            let row_state_consistent = if migration_required {
                row.reopen_state
                    == CollectionPortabilityReopenState::IncompatibleArtifactMigrationRequired
            } else {
                row.reopen_state == saved_query_state && row.reopen_state == history_state
            };
            if !row_state_consistent {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::ScopeHonestyStateCollapsed,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "row {} reopen state {} disagrees with saved-query/history scope honesty or migration",
                        row.row_id,
                        row.reopen_state.as_str()
                    ),
                ));
            }
            if !matches!(
                row.saved_query.migration_state,
                SearchArtifactMigrationState::Current
                    | SearchArtifactMigrationState::MigratedFromPreviousVersion
                    | SearchArtifactMigrationState::MigrationRequired
            ) {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::MigrationStateDropped,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "row {} saved query drops the migration state vocabulary",
                        row.row_id
                    ),
                ));
            }

            row_reopen_states.insert(row.reopen_state);
            row_surfaces.insert(crate::collections::CollectionSurfaceFamily::SearchCollection);
        }

        let covered_reopen: HashSet<CollectionPortabilityReopenState> =
            self.covered_reopen_states.iter().copied().collect();
        for state in &row_reopen_states {
            if !covered_reopen.contains(state) {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::ReopenStateCoverageDropped,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "row carries reopen state {} but packet covered_reopen_states drops it",
                        state.as_str()
                    ),
                ));
            }
        }
        for state in &covered_reopen {
            if !row_reopen_states.contains(state) {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::ReopenStateCoverageOverDeclared,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "packet declares reopen state {} in coverage but no row carries it",
                        state.as_str()
                    ),
                ));
            }
        }

        let covered_surfaces: HashSet<crate::collections::CollectionSurfaceFamily> =
            self.covered_surfaces.iter().copied().collect();
        for surface in &row_surfaces {
            if !covered_surfaces.contains(surface) {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::SurfaceFamilyCoverageDropped,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "row carries surface family {} but packet covered_surfaces drops it",
                        surface.as_str()
                    ),
                ));
            }
        }
        for surface in &covered_surfaces {
            if !row_surfaces.contains(surface) {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::SurfaceFamilyCoverageOverDeclared,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "packet declares surface family {} in coverage but no row carries it",
                        surface.as_str()
                    ),
                ));
            }
        }

        for required_surface in CollectionPortabilityConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::MissingConsumerProjection,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "packet {} is missing a preserved {} projection",
                        self.packet_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }
        for projection in &self.consumer_projections {
            if !projection.preserves_truth_for(&self.packet_id) {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::ConsumerProjectionDrift,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve collection portability truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_filter_ast {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::ProjectionFilterAstDropped,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the filter AST vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_saved_view {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::ProjectionSavedViewDropped,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the saved view vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_scope_pack {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::ProjectionScopePackDropped,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the scope-pack binding vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_column_preset {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::ProjectionColumnPresetDropped,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the column-preset vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_scope_counters {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::ProjectionScopeCountersDropped,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the scope-counter vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_batch_review {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::ProjectionBatchReviewDropped,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the batch-review vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_query_history {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::ProjectionQueryHistoryDropped,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the query-history vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_scope_honesty {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::ProjectionScopeHonestyDropped,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the scope-honesty / reopen vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.raw_private_material_excluded
                || !projection.ambient_authority_excluded
            {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::RawBoundaryMaterialPresent,
                    CollectionPortabilityFindingSeverity::Blocker,
                    format!(
                        "projection {} admits raw or ambient material",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion.retain(|finding| {
                finding.finding_kind != CollectionPortabilityFindingKind::PromotionStateMismatch
            });
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(CollectionPortabilityValidationFinding::new(
                    CollectionPortabilityFindingKind::PromotionStateMismatch,
                    CollectionPortabilityFindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

fn filter_ast_id_for(row: &CollectionPortabilityRow) -> String {
    row.saved_view.filter_ast.filter_ast_id.clone()
}

fn promotion_state_for_findings(
    findings: &[CollectionPortabilityValidationFinding],
) -> CollectionPortabilityPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == CollectionPortabilityFindingSeverity::Blocker)
    {
        CollectionPortabilityPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == CollectionPortabilityFindingSeverity::Warning)
    {
        CollectionPortabilityPromotionState::NarrowedBelowStable
    } else {
        CollectionPortabilityPromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionPortabilityTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub export_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub export_packet: CollectionPortabilityTruthPacket,
}

impl CollectionPortabilityTruthSupportExport {
    /// True when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == COLLECTION_PORTABILITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == COLLECTION_PORTABILITY_TRUTH_SCHEMA_VERSION
            && self.export_packet_id_ref == self.export_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.export_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable packet.
#[derive(Debug)]
pub enum CollectionPortabilityTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<CollectionPortabilityValidationFinding>),
}

impl fmt::Display for CollectionPortabilityTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(formatter, "collection portability truth packet parse failed: {error}")
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "collection portability truth packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for CollectionPortabilityTruthArtifactError {}

/// Returns the checked-in stable truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_collection_portability_truth_packet(
) -> Result<CollectionPortabilityTruthPacket, CollectionPortabilityTruthArtifactError> {
    let packet: CollectionPortabilityTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/search/m4/collection_portability_truth_packet.json"
    )))
    .map_err(CollectionPortabilityTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(CollectionPortabilityTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(
            CollectionPortabilityConsumerSurface::DesktopSearchShell.as_str(),
            "desktop_search_shell"
        );
        assert_eq!(
            CollectionPortabilityConsumerSurface::CompanionSearchShell.as_str(),
            "companion_search_shell"
        );
        assert_eq!(
            CollectionPortabilityFindingKind::FilterAstIdMismatch.as_str(),
            "filter_ast_id_mismatch"
        );
        assert_eq!(
            CollectionPortabilityFindingKind::ScopeCounterVocabularyDropped.as_str(),
            "scope_counter_vocabulary_dropped"
        );
        assert_eq!(
            CollectionPortabilityFindingKind::CapturedArtifactClaimsLiveResults.as_str(),
            "captured_artifact_claims_live_results"
        );
        assert_eq!(
            CollectionPortabilityReopenState::CurrentScopeChangedRebindRequired.as_str(),
            "current_scope_changed_rebind_required"
        );
        assert_eq!(
            CollectionPortabilityPromotionState::Stable.as_str(),
            "stable"
        );
    }

    #[test]
    fn empty_input_blocks_stable() {
        let packet = CollectionPortabilityTruthPacket::materialize(
            CollectionPortabilityTruthPacketInput {
                packet_id: String::new(),
                workflow_or_surface_id: String::new(),
                query_session_id_ref: String::new(),
                generated_at: String::new(),
                covered_reopen_states: Vec::new(),
                covered_surfaces: Vec::new(),
                rows: Vec::new(),
                consumer_projections: Vec::new(),
                source_contract_refs: Vec::new(),
            },
        );
        assert_eq!(
            packet.promotion_state,
            CollectionPortabilityPromotionState::BlocksStable
        );
        assert!(packet.validation_findings.iter().any(|finding| finding
            .finding_kind
            == CollectionPortabilityFindingKind::MissingIdentity));
    }
}
