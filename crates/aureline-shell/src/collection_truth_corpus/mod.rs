//! Collection-truth conformance corpus.
//!
//! Builds on the M3 collection-truth beta primitives in
//! [`crate::collection_truth`] and turns them into a reusable evidence
//! corpus that every claimed beta collection surface (search/result
//! grids, review inboxes, log timelines, package inventories, work-item
//! boards, admin grids) MUST pass to retain its beta claim. The corpus
//! catches the demo-only failure modes dense collections are prone to:
//! hidden narrowing, wrong bulk scope, stale saved views, unstable
//! anchors, and provider-implied totals that are not actually current.
//!
//! ## Honesty contract
//!
//! 1. The seeded packet covers every surface family from
//!    [`CollectionTruthSurfaceFamily`] AND every required
//!    [`CollectionTruthEdgeCaseClass`] so a regression that hides a
//!    visible/loaded/matching/total ambiguity, blocked row, hidden
//!    selected row, or stale provider cursor cannot pass.
//! 2. Saved-view migration cases must cover schema-version upgrades,
//!    unsupported column presets, stale provider-owned scopes, and
//!    policy-narrowed collections. Restore must downgrade or reject —
//!    never silently misinterpret — using one
//!    [`SavedViewFallbackBehavior`].
//! 3. Accessibility drills cover anchor-based range selection,
//!    hidden-selected count inspection, batch-review opening, and
//!    saved-view switching under virtualization. Keyboard and
//!    screen-reader assertions are quoted verbatim so design QA and
//!    support exports share one wording.
//! 4. The support-export projection captures count class, active
//!    narrowing sources, selected-id classes, blocked/skipped reasoning,
//!    and resulting action scope — and explicitly carries no raw row
//!    payload, no secret-bearing literal, and no provider cursor.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::collection_truth::{
    seeded_collection_truth_beta_packet, validate_collection_truth_beta_packet,
    BatchActionConsequenceClass, BatchReviewBlockedReason, BatchReviewSheetRecord,
    BatchReviewSummary, CollectionScopeCounterRecord, CollectionScopeCounterRow,
    CollectionTruthBetaPacket, CollectionTruthCase, CollectionTruthSurfaceFamily,
    CountSummaryClass, FilterBarChipRecord, FilterBarStateRecord, NarrowingSourceClass,
    RecoveryGuidanceClass, SavedCollectionViewRecord, SavedViewColumnPreset, SavedViewDriftState,
    SavedViewFallbackBehavior, SavedViewPinnedCountAxis, SavedViewScopeClass, ScopeCounterClass,
    ScopeCounterStatus, SelectAllEscalationClass, COLLECTION_TRUTH_BETA_SHARED_CONTRACT_REF,
};

/// Schema version exported with every corpus record.
pub const COLLECTION_TRUTH_CORPUS_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by fixtures, docs, and support exports.
pub const COLLECTION_TRUTH_CORPUS_SHARED_CONTRACT_REF: &str = "shell:collection_truth_corpus:v1";

/// Stable record kind for [`CollectionTruthCorpusPacket`].
pub const COLLECTION_TRUTH_CORPUS_PACKET_RECORD_KIND: &str =
    "shell_collection_truth_corpus_packet_record";

/// Stable record kind for [`CollectionTruthCorpusCase`].
pub const COLLECTION_TRUTH_CORPUS_CASE_RECORD_KIND: &str =
    "shell_collection_truth_corpus_case_record";

/// Stable record kind for [`SavedViewMigrationCase`].
pub const SAVED_VIEW_MIGRATION_CASE_RECORD_KIND: &str =
    "shell_collection_truth_corpus_saved_view_migration_record";

/// Stable record kind for [`CollectionTruthAccessibilityDrill`].
pub const COLLECTION_TRUTH_DRILL_RECORD_KIND: &str = "shell_collection_truth_corpus_drill_record";

/// Stable record kind for [`CollectionTruthSupportExport`].
pub const COLLECTION_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_collection_truth_corpus_support_export_record";

/// Stable packet id used by every consumer.
pub const COLLECTION_TRUTH_CORPUS_PACKET_ID: &str = "shell:collection_truth_corpus:packet:default";

/// Deterministic packet timestamp.
pub const COLLECTION_TRUTH_CORPUS_GENERATED_AT: &str = "2026-05-18T00:00:00Z";

/// Closed class of edge-case shape that the corpus exercises across
/// every surface family. The validator refuses a corpus packet that
/// does not exercise every variant somewhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionTruthEdgeCaseClass {
    /// Visible equals loaded equals matching equals total, all exact.
    VisibleEqualsLoadedExactTotal,
    /// Virtualized window where visible < loaded < matching and total is
    /// partial because indexing has not finished.
    VirtualizedWindowPartialMatching,
    /// Provider capped or sampled matching — total is unknown or
    /// approximate and counter must say so.
    ProviderCappedApproximateMatching,
    /// Provider retention window cuts off pre-retention totals.
    ProviderRetentionUnknownTotal,
    /// Selection includes blocked rows distinguishable from
    /// skipped/already-compliant rows.
    BlockedRowsPresent,
    /// Selection includes hidden rows that are outside the current
    /// view and must remain inspectable.
    HiddenSelectedRowsPresent,
    /// Last known provider cursor is stale; saved view refuses to
    /// rebind silently and surfaces a rebind / recreate flow.
    StaleProviderCursorDetected,
}

impl CollectionTruthEdgeCaseClass {
    /// Stable token used in fixtures, packets, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VisibleEqualsLoadedExactTotal => "visible_equals_loaded_exact_total",
            Self::VirtualizedWindowPartialMatching => "virtualized_window_partial_matching",
            Self::ProviderCappedApproximateMatching => "provider_capped_approximate_matching",
            Self::ProviderRetentionUnknownTotal => "provider_retention_unknown_total",
            Self::BlockedRowsPresent => "blocked_rows_present",
            Self::HiddenSelectedRowsPresent => "hidden_selected_rows_present",
            Self::StaleProviderCursorDetected => "stale_provider_cursor_detected",
        }
    }

    /// Returns every required edge-case class the validator enforces.
    pub const fn all() -> [Self; 7] {
        [
            Self::VisibleEqualsLoadedExactTotal,
            Self::VirtualizedWindowPartialMatching,
            Self::ProviderCappedApproximateMatching,
            Self::ProviderRetentionUnknownTotal,
            Self::BlockedRowsPresent,
            Self::HiddenSelectedRowsPresent,
            Self::StaleProviderCursorDetected,
        ]
    }
}

/// Closed class of saved-view migration shape the corpus exercises.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedViewMigrationCaseClass {
    /// Older schema upgraded exactly — every captured field still
    /// round-trips and no disclosure is required.
    OlderSchemaVersionUpgradedExact,
    /// Older schema upgraded with labeled drift — the captured filter
    /// or column set partially binds and disclosure labels are kept.
    OlderSchemaVersionUpgradedDegraded,
    /// Captured column preset is unsupported on the current surface —
    /// it is dropped from the loadable subset and labeled.
    UnsupportedColumnPresetDroppedLabeled,
    /// Captured provider cursor is stale; the restore refuses the
    /// cursor and offers a rebind path.
    StaleProviderCursorRefusedAndOffered,
    /// Captured provider-owned scope no longer resolves; offer to
    /// recreate the view from current state.
    StaleProviderOwnedScopeOfferRecreate,
    /// Policy narrowing changed since capture; the view is rebound
    /// against the new policy epoch with disclosure.
    PolicyNarrowedCollectionRebound,
}

impl SavedViewMigrationCaseClass {
    /// Stable token used in fixtures, packets, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OlderSchemaVersionUpgradedExact => "older_schema_version_upgraded_exact",
            Self::OlderSchemaVersionUpgradedDegraded => "older_schema_version_upgraded_degraded",
            Self::UnsupportedColumnPresetDroppedLabeled => {
                "unsupported_column_preset_dropped_labeled"
            }
            Self::StaleProviderCursorRefusedAndOffered => {
                "stale_provider_cursor_refused_and_offered"
            }
            Self::StaleProviderOwnedScopeOfferRecreate => {
                "stale_provider_owned_scope_offer_recreate"
            }
            Self::PolicyNarrowedCollectionRebound => "policy_narrowed_collection_rebound",
        }
    }

    /// Returns every required migration class the validator enforces.
    pub const fn all() -> [Self; 6] {
        [
            Self::OlderSchemaVersionUpgradedExact,
            Self::OlderSchemaVersionUpgradedDegraded,
            Self::UnsupportedColumnPresetDroppedLabeled,
            Self::StaleProviderCursorRefusedAndOffered,
            Self::StaleProviderOwnedScopeOfferRecreate,
            Self::PolicyNarrowedCollectionRebound,
        ]
    }
}

/// Closed class of keyboard / screen-reader drill the corpus exercises.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionTruthAccessibilityDrillClass {
    /// Anchor-based range selection across a virtualized window.
    KeyboardAnchorRangeSelection,
    /// Hidden-selected count inspection via screen-reader narration.
    ScreenReaderHiddenSelectedInspection,
    /// Batch-review sheet opening via keyboard before continuing a
    /// destructive, export-bearing, or provider-backed action.
    KeyboardBatchReviewOpen,
    /// Saved-view switching under virtualization (no anchor drift).
    SavedViewSwitcherUnderVirtualization,
}

impl CollectionTruthAccessibilityDrillClass {
    /// Stable token used in fixtures, packets, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardAnchorRangeSelection => "keyboard_anchor_range_selection",
            Self::ScreenReaderHiddenSelectedInspection => {
                "screen_reader_hidden_selected_inspection"
            }
            Self::KeyboardBatchReviewOpen => "keyboard_batch_review_open",
            Self::SavedViewSwitcherUnderVirtualization => {
                "saved_view_switcher_under_virtualization"
            }
        }
    }

    /// Returns every required drill class the validator enforces.
    pub const fn all() -> [Self; 4] {
        [
            Self::KeyboardAnchorRangeSelection,
            Self::ScreenReaderHiddenSelectedInspection,
            Self::KeyboardBatchReviewOpen,
            Self::SavedViewSwitcherUnderVirtualization,
        ]
    }
}

/// One worked corpus case: extends a base [`CollectionTruthCase`] with
/// edge-case classification, anchor-based selection record, and
/// inspectability invariants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionTruthCorpusCase {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable case id.
    pub case_id: String,
    /// Surface family this case represents.
    pub surface_family: CollectionTruthSurfaceFamily,
    /// Short reviewable label for the case.
    pub case_label: String,
    /// Edge-case classes this case exercises.
    pub edge_case_classes: Vec<CollectionTruthEdgeCaseClass>,
    /// Filter bar state record.
    pub filter_bar: FilterBarStateRecord,
    /// Saved view restored for this case.
    pub saved_view: SavedCollectionViewRecord,
    /// Scope counter record for this case.
    pub scope_counter: CollectionScopeCounterRecord,
    /// Batch-review sheet for the proposed consequential action.
    pub batch_review: BatchReviewSheetRecord,
    /// Anchor row id used by anchor-based range selection drills.
    pub anchor_row_id: String,
    /// Stable, redacted selected row ids (no payload literals).
    pub selected_row_ids: Vec<String>,
    /// Stable, redacted hidden selected row ids (rows selected but
    /// outside the current viewport / view).
    pub hidden_selected_row_ids: Vec<String>,
    /// Stable, redacted blocked row ids.
    pub blocked_row_ids: Vec<String>,
}

impl CollectionTruthCorpusCase {
    /// True when the case admits at least one ambiguous-total axis.
    pub fn has_ambiguous_total(&self) -> bool {
        self.scope_counter.rows.iter().any(|row| {
            row.counter_class == ScopeCounterClass::Total && row.status != ScopeCounterStatus::Exact
        })
    }
}

/// One saved-view migration case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedViewMigrationCase {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable migration case id.
    pub migration_case_id: String,
    /// Surface family this migration case applies to.
    pub surface_family: CollectionTruthSurfaceFamily,
    /// Case class.
    pub case_class: SavedViewMigrationCaseClass,
    /// Schema version of the captured view.
    pub captured_schema_version: u32,
    /// Schema version of the restored view.
    pub restored_schema_version: u32,
    /// Captured view (as it would be deserialized from older state).
    pub captured_view: SavedCollectionViewRecord,
    /// Restored view (after migration to current schema).
    pub restored_view: SavedCollectionViewRecord,
    /// Notes the restore surface displays alongside the migrated view.
    pub migration_notes: Vec<String>,
    /// Portable-subset findings (what was dropped, refused, or relabeled).
    pub portability_findings: Vec<String>,
}

/// One keyboard / screen-reader drill record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionTruthAccessibilityDrill {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable drill id.
    pub drill_id: String,
    /// Surface family this drill targets.
    pub surface_family: CollectionTruthSurfaceFamily,
    /// Drill class.
    pub drill_class: CollectionTruthAccessibilityDrillClass,
    /// Short label for the drill.
    pub label: String,
    /// Keyboard or screen-reader steps, in order.
    pub steps: Vec<String>,
    /// Assertions the drill MUST verify before passing.
    pub expected_assertions: Vec<String>,
    /// Virtualization invariants that MUST hold during the drill.
    pub virtualization_invariants: Vec<String>,
    /// Screen-reader narration summary verified verbatim by support.
    pub accessibility_narration_summary: String,
}

/// Support-export projection — fully redacted, schema-bound.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Sourced corpus packet id.
    pub packet_ref: String,
    /// Per-case count classes (no values).
    pub count_summary_classes: Vec<SupportExportCountSummaryRow>,
    /// Per-case active narrowing sources.
    pub active_narrowing_sources: Vec<SupportExportNarrowingRow>,
    /// Per-case selected id classes (redaction-safe).
    pub selected_id_classes: Vec<SupportExportSelectedIdRow>,
    /// Per-case blocked / skipped reasoning (label-only).
    pub blocked_or_skipped_reasoning: Vec<SupportExportBlockedRow>,
    /// Per-case resulting action scope (counts + escalation class).
    pub resulting_action_scope: Vec<SupportExportActionScopeRow>,
    /// Constant invariant carried verbatim in every export.
    pub no_sensitive_payload: bool,
    /// Redaction rules carried verbatim alongside the export.
    pub redaction_rules: Vec<String>,
}

/// One count-summary row in the support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportCountSummaryRow {
    /// Case id.
    pub case_id: String,
    /// Surface family.
    pub surface_family: CollectionTruthSurfaceFamily,
    /// Count summary class.
    pub count_summary_class: CountSummaryClass,
    /// Per-axis counter classes.
    pub counter_classes: Vec<ScopeCounterClass>,
    /// Per-axis counter statuses.
    pub counter_statuses: Vec<ScopeCounterStatus>,
    /// True when at least one axis is non-exact.
    pub has_non_exact_axis: bool,
}

/// One active-narrowing row in the support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportNarrowingRow {
    /// Case id.
    pub case_id: String,
    /// Surface family.
    pub surface_family: CollectionTruthSurfaceFamily,
    /// Active narrowing source classes.
    pub source_classes: Vec<NarrowingSourceClass>,
    /// True when at least one source is hidden narrowing.
    pub has_hidden_narrowing: bool,
}

/// One selected-id row in the support export — redaction-safe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportSelectedIdRow {
    /// Case id.
    pub case_id: String,
    /// Surface family.
    pub surface_family: CollectionTruthSurfaceFamily,
    /// Stable, redacted selected ids.
    pub selected_row_ids: Vec<String>,
    /// Stable, redacted hidden selected ids.
    pub hidden_selected_row_ids: Vec<String>,
}

/// One blocked / skipped reasoning row in the support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportBlockedRow {
    /// Case id.
    pub case_id: String,
    /// Surface family.
    pub surface_family: CollectionTruthSurfaceFamily,
    /// Stable, redacted blocked row ids.
    pub blocked_row_ids: Vec<String>,
    /// Blocked reason labels (no payload).
    pub blocked_reason_labels: Vec<String>,
}

/// One resulting-action-scope row in the support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportActionScopeRow {
    /// Case id.
    pub case_id: String,
    /// Surface family.
    pub surface_family: CollectionTruthSurfaceFamily,
    /// Action id.
    pub action_id: String,
    /// Action consequence class.
    pub consequence_class: BatchActionConsequenceClass,
    /// Select-all escalation class.
    pub select_all_escalation_class: SelectAllEscalationClass,
    /// Included / excluded / blocked / hidden counts.
    pub included_count: u64,
    /// See [`Self::included_count`].
    pub excluded_count: u64,
    /// See [`Self::included_count`].
    pub blocked_count: u64,
    /// See [`Self::included_count`].
    pub hidden_count: u64,
    /// True when continue was enabled.
    pub continue_enabled: bool,
}

/// One matrix row — projection consumed by the matrix.json artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionTruthMatrixRow {
    /// Surface family.
    pub surface_family: CollectionTruthSurfaceFamily,
    /// Edge-case classes exercised by this surface family.
    pub edge_case_classes_exercised: Vec<CollectionTruthEdgeCaseClass>,
    /// Saved-view migration classes exercised by this surface family.
    pub saved_view_migration_classes_exercised: Vec<SavedViewMigrationCaseClass>,
    /// Drill classes exercised by this surface family.
    pub drill_classes_exercised: Vec<CollectionTruthAccessibilityDrillClass>,
    /// Has at least one ambiguous-total axis somewhere in the surface.
    pub has_ambiguous_total: bool,
    /// Has at least one consequential batch action with hidden selected
    /// rows that the user must inspect before continuing.
    pub has_hidden_selected_rows: bool,
}

/// Coverage matrix written to artifacts/qe/m3/collection_truth_matrix.json.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionTruthMatrix {
    /// Stable matrix id.
    pub matrix_id: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Rows in canonical surface-family order.
    pub rows: Vec<CollectionTruthMatrixRow>,
    /// Edge-case class counts across the whole corpus.
    pub edge_case_class_counts: BTreeMap<String, usize>,
    /// Migration class counts across the whole corpus.
    pub migration_class_counts: BTreeMap<String, usize>,
    /// Drill class counts across the whole corpus.
    pub drill_class_counts: BTreeMap<String, usize>,
}

/// Coverage summary for the corpus packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionTruthCorpusCoverageSummary {
    /// Distinct surface families present.
    pub surface_families_present: Vec<CollectionTruthSurfaceFamily>,
    /// Distinct edge case classes present.
    pub edge_case_classes_present: Vec<CollectionTruthEdgeCaseClass>,
    /// Distinct migration classes present.
    pub saved_view_migration_classes_present: Vec<SavedViewMigrationCaseClass>,
    /// Distinct drill classes present.
    pub drill_classes_present: Vec<CollectionTruthAccessibilityDrillClass>,
}

impl CollectionTruthCorpusCoverageSummary {
    fn from_parts(
        cases: &[CollectionTruthCorpusCase],
        migrations: &[SavedViewMigrationCase],
        drills: &[CollectionTruthAccessibilityDrill],
    ) -> Self {
        let mut surface_families: BTreeSet<CollectionTruthSurfaceFamily> = BTreeSet::new();
        let mut edge_cases: BTreeSet<CollectionTruthEdgeCaseClass> = BTreeSet::new();
        let mut migration_classes: BTreeSet<SavedViewMigrationCaseClass> = BTreeSet::new();
        let mut drill_classes: BTreeSet<CollectionTruthAccessibilityDrillClass> = BTreeSet::new();
        for case in cases {
            surface_families.insert(case.surface_family);
            for class in &case.edge_case_classes {
                edge_cases.insert(*class);
            }
        }
        for migration in migrations {
            surface_families.insert(migration.surface_family);
            migration_classes.insert(migration.case_class);
        }
        for drill in drills {
            surface_families.insert(drill.surface_family);
            drill_classes.insert(drill.drill_class);
        }
        Self {
            surface_families_present: surface_families.into_iter().collect(),
            edge_case_classes_present: edge_cases.into_iter().collect(),
            saved_view_migration_classes_present: migration_classes.into_iter().collect(),
            drill_classes_present: drill_classes.into_iter().collect(),
        }
    }
}

/// Top-level corpus packet record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionTruthCorpusPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Base packet emitted by [`crate::collection_truth`] (re-quoted in
    /// full so the corpus packet is self-contained for support reviews).
    pub base_packet: CollectionTruthBetaPacket,
    /// Base packet's shared contract ref — re-asserted for review.
    pub base_packet_shared_contract_ref: String,
    /// Corpus cases (one or more per surface family).
    pub corpus_cases: Vec<CollectionTruthCorpusCase>,
    /// Saved-view migration cases.
    pub saved_view_migrations: Vec<SavedViewMigrationCase>,
    /// Keyboard / screen-reader drills.
    pub accessibility_drills: Vec<CollectionTruthAccessibilityDrill>,
    /// Support-export projection.
    pub support_export: CollectionTruthSupportExport,
    /// Matrix projection.
    pub matrix: CollectionTruthMatrix,
    /// Coverage summary.
    pub coverage_summary: CollectionTruthCorpusCoverageSummary,
}

/// Validation errors raised against the corpus packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum CollectionTruthCorpusValidationError {
    /// Packet metadata is wrong.
    PacketMetadataWrong { reason: String },
    /// Base packet failed M3-206 validation.
    BasePacketInvalid { reason: String },
    /// A required surface family is missing.
    SurfaceFamilyMissing { missing: String },
    /// A required edge-case class is missing.
    EdgeCaseClassMissing { missing: String },
    /// A required saved-view migration class is missing.
    SavedViewMigrationClassMissing { missing: String },
    /// A required accessibility drill class is missing.
    AccessibilityDrillClassMissing { missing: String },
    /// A migration case rebuilt under the same schema but did not
    /// downgrade or disclose drift.
    MigrationLeavesUndisclosedDrift { migration_case_id: String },
    /// The support export claimed to carry sensitive payload.
    SupportExportClaimsSensitivePayload,
    /// A case quoted a hidden selected row but the batch review hidden
    /// count is zero (lost row).
    HiddenSelectedRowsNotReflectedInBatchSheet { case_id: String },
    /// A case quoted a blocked row but the batch sheet does not say so.
    BlockedRowsNotReflectedInBatchSheet { case_id: String },
}

impl std::fmt::Display for CollectionTruthCorpusValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PacketMetadataWrong { reason } => write!(f, "packet metadata invalid: {reason}"),
            Self::BasePacketInvalid { reason } => write!(f, "base packet invalid: {reason}"),
            Self::SurfaceFamilyMissing { missing } => {
                write!(f, "surface family missing from corpus: {missing}")
            }
            Self::EdgeCaseClassMissing { missing } => {
                write!(f, "edge case class missing from corpus: {missing}")
            }
            Self::SavedViewMigrationClassMissing { missing } => {
                write!(
                    f,
                    "saved view migration class missing from corpus: {missing}"
                )
            }
            Self::AccessibilityDrillClassMissing { missing } => {
                write!(
                    f,
                    "accessibility drill class missing from corpus: {missing}"
                )
            }
            Self::MigrationLeavesUndisclosedDrift { migration_case_id } => {
                write!(
                    f,
                    "migration case {migration_case_id} does not downgrade, refuse, or disclose drift"
                )
            }
            Self::SupportExportClaimsSensitivePayload => {
                write!(f, "support export must declare no_sensitive_payload = true")
            }
            Self::HiddenSelectedRowsNotReflectedInBatchSheet { case_id } => {
                write!(
                    f,
                    "case {case_id} quotes hidden selected rows but the batch sheet reports zero hidden_count"
                )
            }
            Self::BlockedRowsNotReflectedInBatchSheet { case_id } => {
                write!(
                    f,
                    "case {case_id} quotes blocked rows but the batch sheet reports zero blocked_count"
                )
            }
        }
    }
}

impl std::error::Error for CollectionTruthCorpusValidationError {}

/// Validates a corpus packet against the M03-207 acceptance invariants.
pub fn validate_collection_truth_corpus_packet(
    packet: &CollectionTruthCorpusPacket,
) -> Result<(), Vec<CollectionTruthCorpusValidationError>> {
    let mut errors = Vec::new();
    if packet.record_kind != COLLECTION_TRUTH_CORPUS_PACKET_RECORD_KIND {
        errors.push(CollectionTruthCorpusValidationError::PacketMetadataWrong {
            reason: "record kind mismatch".to_string(),
        });
    }
    if packet.schema_version != COLLECTION_TRUTH_CORPUS_SCHEMA_VERSION {
        errors.push(CollectionTruthCorpusValidationError::PacketMetadataWrong {
            reason: "schema version mismatch".to_string(),
        });
    }
    if packet.shared_contract_ref != COLLECTION_TRUTH_CORPUS_SHARED_CONTRACT_REF {
        errors.push(CollectionTruthCorpusValidationError::PacketMetadataWrong {
            reason: "shared contract ref mismatch".to_string(),
        });
    }
    if packet.base_packet_shared_contract_ref != COLLECTION_TRUTH_BETA_SHARED_CONTRACT_REF {
        errors.push(CollectionTruthCorpusValidationError::PacketMetadataWrong {
            reason: "base packet shared contract ref mismatch".to_string(),
        });
    }
    if let Err(base_errors) = validate_collection_truth_beta_packet(&packet.base_packet) {
        for err in base_errors {
            errors.push(CollectionTruthCorpusValidationError::BasePacketInvalid {
                reason: err.to_string(),
            });
        }
    }

    let mut surface_families_seen: BTreeSet<CollectionTruthSurfaceFamily> = BTreeSet::new();
    let mut edge_cases_seen: BTreeSet<CollectionTruthEdgeCaseClass> = BTreeSet::new();
    let mut migration_classes_seen: BTreeSet<SavedViewMigrationCaseClass> = BTreeSet::new();
    let mut drill_classes_seen: BTreeSet<CollectionTruthAccessibilityDrillClass> = BTreeSet::new();

    for case in &packet.corpus_cases {
        surface_families_seen.insert(case.surface_family);
        for class in &case.edge_case_classes {
            edge_cases_seen.insert(*class);
        }
        if !case.hidden_selected_row_ids.is_empty() && case.batch_review.summary.hidden_count == 0 {
            errors.push(
                CollectionTruthCorpusValidationError::HiddenSelectedRowsNotReflectedInBatchSheet {
                    case_id: case.case_id.clone(),
                },
            );
        }
        if !case.blocked_row_ids.is_empty() && case.batch_review.summary.blocked_count == 0 {
            errors.push(
                CollectionTruthCorpusValidationError::BlockedRowsNotReflectedInBatchSheet {
                    case_id: case.case_id.clone(),
                },
            );
        }
    }
    for migration in &packet.saved_view_migrations {
        migration_classes_seen.insert(migration.case_class);
        let restored = &migration.restored_view;
        let bound_and_clean = restored.drift_state
            == SavedViewDriftState::BoundCurrentStateMatchesCaptured
            && restored.stale_or_degraded_labels.is_empty();
        let migration_is_lossless =
            migration.case_class == SavedViewMigrationCaseClass::OlderSchemaVersionUpgradedExact;
        if !migration_is_lossless && bound_and_clean {
            errors.push(
                CollectionTruthCorpusValidationError::MigrationLeavesUndisclosedDrift {
                    migration_case_id: migration.migration_case_id.clone(),
                },
            );
        }
    }
    for drill in &packet.accessibility_drills {
        drill_classes_seen.insert(drill.drill_class);
    }

    for required in CollectionTruthSurfaceFamily::all() {
        if !surface_families_seen.contains(&required) {
            errors.push(CollectionTruthCorpusValidationError::SurfaceFamilyMissing {
                missing: required.as_str().to_string(),
            });
        }
    }
    for required in CollectionTruthEdgeCaseClass::all() {
        if !edge_cases_seen.contains(&required) {
            errors.push(CollectionTruthCorpusValidationError::EdgeCaseClassMissing {
                missing: required.as_str().to_string(),
            });
        }
    }
    for required in SavedViewMigrationCaseClass::all() {
        if !migration_classes_seen.contains(&required) {
            errors.push(
                CollectionTruthCorpusValidationError::SavedViewMigrationClassMissing {
                    missing: required.as_str().to_string(),
                },
            );
        }
    }
    for required in CollectionTruthAccessibilityDrillClass::all() {
        if !drill_classes_seen.contains(&required) {
            errors.push(
                CollectionTruthCorpusValidationError::AccessibilityDrillClassMissing {
                    missing: required.as_str().to_string(),
                },
            );
        }
    }
    if !packet.support_export.no_sensitive_payload {
        errors.push(CollectionTruthCorpusValidationError::SupportExportClaimsSensitivePayload);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Builds the deterministic seeded corpus packet.
pub fn seeded_collection_truth_corpus_packet() -> CollectionTruthCorpusPacket {
    let base_packet = seeded_collection_truth_beta_packet();
    let corpus_cases = seeds::all_corpus_cases(&base_packet);
    let saved_view_migrations = seeds::all_saved_view_migrations();
    let accessibility_drills = seeds::all_accessibility_drills();
    let support_export = build_support_export(&corpus_cases);
    let matrix = build_matrix(&corpus_cases, &saved_view_migrations, &accessibility_drills);
    let coverage_summary = CollectionTruthCorpusCoverageSummary::from_parts(
        &corpus_cases,
        &saved_view_migrations,
        &accessibility_drills,
    );
    CollectionTruthCorpusPacket {
        record_kind: COLLECTION_TRUTH_CORPUS_PACKET_RECORD_KIND.to_string(),
        schema_version: COLLECTION_TRUTH_CORPUS_SCHEMA_VERSION,
        shared_contract_ref: COLLECTION_TRUTH_CORPUS_SHARED_CONTRACT_REF.to_string(),
        packet_id: COLLECTION_TRUTH_CORPUS_PACKET_ID.to_string(),
        generated_at: COLLECTION_TRUTH_CORPUS_GENERATED_AT.to_string(),
        base_packet_shared_contract_ref: COLLECTION_TRUTH_BETA_SHARED_CONTRACT_REF.to_string(),
        base_packet,
        corpus_cases,
        saved_view_migrations,
        accessibility_drills,
        support_export,
        matrix,
        coverage_summary,
    }
}

fn build_support_export(cases: &[CollectionTruthCorpusCase]) -> CollectionTruthSupportExport {
    let mut count_summary_classes = Vec::new();
    let mut active_narrowing_sources = Vec::new();
    let mut selected_id_classes = Vec::new();
    let mut blocked_or_skipped_reasoning = Vec::new();
    let mut resulting_action_scope = Vec::new();
    for case in cases {
        count_summary_classes.push(SupportExportCountSummaryRow {
            case_id: case.case_id.clone(),
            surface_family: case.surface_family,
            count_summary_class: case.filter_bar.count_summary_class,
            counter_classes: case
                .scope_counter
                .rows
                .iter()
                .map(|row| row.counter_class)
                .collect(),
            counter_statuses: case
                .scope_counter
                .rows
                .iter()
                .map(|row| row.status)
                .collect(),
            has_non_exact_axis: case.scope_counter.has_non_exact_row(),
        });
        let mut source_classes: Vec<NarrowingSourceClass> = case
            .filter_bar
            .chips
            .iter()
            .map(|chip| chip.source_class)
            .collect();
        source_classes.sort();
        source_classes.dedup();
        let has_hidden_narrowing = source_classes
            .iter()
            .any(|class| class.is_hidden_narrowing());
        active_narrowing_sources.push(SupportExportNarrowingRow {
            case_id: case.case_id.clone(),
            surface_family: case.surface_family,
            source_classes,
            has_hidden_narrowing,
        });
        selected_id_classes.push(SupportExportSelectedIdRow {
            case_id: case.case_id.clone(),
            surface_family: case.surface_family,
            selected_row_ids: case.selected_row_ids.clone(),
            hidden_selected_row_ids: case.hidden_selected_row_ids.clone(),
        });
        blocked_or_skipped_reasoning.push(SupportExportBlockedRow {
            case_id: case.case_id.clone(),
            surface_family: case.surface_family,
            blocked_row_ids: case.blocked_row_ids.clone(),
            blocked_reason_labels: case
                .batch_review
                .blocked_reasons
                .iter()
                .map(|reason| reason.label.clone())
                .collect(),
        });
        resulting_action_scope.push(SupportExportActionScopeRow {
            case_id: case.case_id.clone(),
            surface_family: case.surface_family,
            action_id: case.batch_review.action_id.clone(),
            consequence_class: case.batch_review.consequence_class,
            select_all_escalation_class: case.batch_review.select_all_escalation_class,
            included_count: case.batch_review.summary.included_count,
            excluded_count: case.batch_review.summary.excluded_count,
            blocked_count: case.batch_review.summary.blocked_count,
            hidden_count: case.batch_review.summary.hidden_count,
            continue_enabled: case.batch_review.continue_enabled,
        });
    }
    CollectionTruthSupportExport {
        record_kind: COLLECTION_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_string(),
        schema_version: COLLECTION_TRUTH_CORPUS_SCHEMA_VERSION,
        support_export_id: "shell:collection_truth_corpus:support_export:default".to_string(),
        packet_ref: COLLECTION_TRUTH_CORPUS_PACKET_ID.to_string(),
        count_summary_classes,
        active_narrowing_sources,
        selected_id_classes,
        blocked_or_skipped_reasoning,
        resulting_action_scope,
        no_sensitive_payload: true,
        redaction_rules: vec![
            "row payload literals are never exported".to_string(),
            "saved-view secret-bearing values are never exported".to_string(),
            "provider cursors are never exported".to_string(),
            "selected row ids are stable redaction-safe slugs only".to_string(),
        ],
    }
}

fn build_matrix(
    cases: &[CollectionTruthCorpusCase],
    migrations: &[SavedViewMigrationCase],
    drills: &[CollectionTruthAccessibilityDrill],
) -> CollectionTruthMatrix {
    let mut per_family_edge: BTreeMap<
        CollectionTruthSurfaceFamily,
        BTreeSet<CollectionTruthEdgeCaseClass>,
    > = BTreeMap::new();
    let mut per_family_migration: BTreeMap<
        CollectionTruthSurfaceFamily,
        BTreeSet<SavedViewMigrationCaseClass>,
    > = BTreeMap::new();
    let mut per_family_drill: BTreeMap<
        CollectionTruthSurfaceFamily,
        BTreeSet<CollectionTruthAccessibilityDrillClass>,
    > = BTreeMap::new();
    let mut per_family_ambiguous_total: BTreeMap<CollectionTruthSurfaceFamily, bool> =
        BTreeMap::new();
    let mut per_family_hidden_selected: BTreeMap<CollectionTruthSurfaceFamily, bool> =
        BTreeMap::new();

    for case in cases {
        let entry = per_family_edge.entry(case.surface_family).or_default();
        for class in &case.edge_case_classes {
            entry.insert(*class);
        }
        per_family_ambiguous_total
            .entry(case.surface_family)
            .and_modify(|flag| *flag = *flag || case.has_ambiguous_total())
            .or_insert(case.has_ambiguous_total());
        per_family_hidden_selected
            .entry(case.surface_family)
            .and_modify(|flag| *flag = *flag || !case.hidden_selected_row_ids.is_empty())
            .or_insert(!case.hidden_selected_row_ids.is_empty());
    }
    for migration in migrations {
        per_family_migration
            .entry(migration.surface_family)
            .or_default()
            .insert(migration.case_class);
    }
    for drill in drills {
        per_family_drill
            .entry(drill.surface_family)
            .or_default()
            .insert(drill.drill_class);
    }

    let mut rows = Vec::new();
    for family in CollectionTruthSurfaceFamily::all() {
        let edge: Vec<_> = per_family_edge
            .get(&family)
            .map(|set| set.iter().copied().collect())
            .unwrap_or_default();
        let migration: Vec<_> = per_family_migration
            .get(&family)
            .map(|set| set.iter().copied().collect())
            .unwrap_or_default();
        let drill: Vec<_> = per_family_drill
            .get(&family)
            .map(|set| set.iter().copied().collect())
            .unwrap_or_default();
        rows.push(CollectionTruthMatrixRow {
            surface_family: family,
            edge_case_classes_exercised: edge,
            saved_view_migration_classes_exercised: migration,
            drill_classes_exercised: drill,
            has_ambiguous_total: per_family_ambiguous_total
                .get(&family)
                .copied()
                .unwrap_or(false),
            has_hidden_selected_rows: per_family_hidden_selected
                .get(&family)
                .copied()
                .unwrap_or(false),
        });
    }

    let mut edge_case_class_counts: BTreeMap<String, usize> = BTreeMap::new();
    for case in cases {
        for class in &case.edge_case_classes {
            *edge_case_class_counts
                .entry(class.as_str().to_string())
                .or_insert(0) += 1;
        }
    }
    let mut migration_class_counts: BTreeMap<String, usize> = BTreeMap::new();
    for migration in migrations {
        *migration_class_counts
            .entry(migration.case_class.as_str().to_string())
            .or_insert(0) += 1;
    }
    let mut drill_class_counts: BTreeMap<String, usize> = BTreeMap::new();
    for drill in drills {
        *drill_class_counts
            .entry(drill.drill_class.as_str().to_string())
            .or_insert(0) += 1;
    }

    CollectionTruthMatrix {
        matrix_id: "shell:collection_truth_corpus:matrix:default".to_string(),
        schema_version: COLLECTION_TRUTH_CORPUS_SCHEMA_VERSION,
        shared_contract_ref: COLLECTION_TRUTH_CORPUS_SHARED_CONTRACT_REF.to_string(),
        rows,
        edge_case_class_counts,
        migration_class_counts,
        drill_class_counts,
    }
}

/// Renders the markdown report consumed by `artifacts/qe/m3/collection_truth_report.md`.
pub fn render_collection_truth_corpus_report_markdown(
    packet: &CollectionTruthCorpusPacket,
) -> String {
    let mut out = String::new();
    out.push_str("# Collection-truth corpus report\n\n");
    out.push_str(
        "Generated from the seeded corpus in `crates/aureline-shell/src/collection_truth_corpus/mod.rs`.\n",
    );
    out.push_str("Regenerate with:\n\n");
    out.push_str("```sh\n");
    out.push_str(
        "cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- report-md > \\\n  artifacts/qe/m3/collection_truth_report.md\n",
    );
    out.push_str("cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- matrix-json > \\\n  artifacts/qe/m3/collection_truth_matrix.json\n");
    out.push_str("```\n\n");
    out.push_str(&format!("- Packet id: `{}`\n", packet.packet_id));
    out.push_str(&format!(
        "- Shared contract ref: `{}`\n",
        packet.shared_contract_ref
    ));
    out.push_str(&format!(
        "- Base packet shared contract ref: `{}`\n",
        packet.base_packet_shared_contract_ref
    ));
    out.push_str(&format!("- Generated at: `{}`\n", packet.generated_at));
    out.push('\n');

    out.push_str("## Surface family matrix\n\n");
    out.push_str(
        "| Surface | Edge cases exercised | Saved-view migrations | Drills | Ambiguous total | Hidden selected |\n",
    );
    out.push_str(
        "| ------- | -------------------- | --------------------- | ------ | --------------- | --------------- |\n",
    );
    for row in &packet.matrix.rows {
        out.push_str(&format!(
            "| `{family}` | {edge} | {migration} | {drill} | {ambig} | {hidden} |\n",
            family = row.surface_family.as_str(),
            edge = format_class_list(&row.edge_case_classes_exercised, |c| c.as_str()),
            migration =
                format_class_list(&row.saved_view_migration_classes_exercised, |c| c.as_str()),
            drill = format_class_list(&row.drill_classes_exercised, |c| c.as_str()),
            ambig = if row.has_ambiguous_total { "yes" } else { "no" },
            hidden = if row.has_hidden_selected_rows {
                "yes"
            } else {
                "no"
            },
        ));
    }
    out.push('\n');

    out.push_str("## Edge-case class counts\n\n");
    for (class, count) in &packet.matrix.edge_case_class_counts {
        out.push_str(&format!("- `{class}` -- {count}\n"));
    }
    out.push('\n');

    out.push_str("## Saved-view migration class counts\n\n");
    for (class, count) in &packet.matrix.migration_class_counts {
        out.push_str(&format!("- `{class}` -- {count}\n"));
    }
    out.push('\n');

    out.push_str("## Drill class counts\n\n");
    for (class, count) in &packet.matrix.drill_class_counts {
        out.push_str(&format!("- `{class}` -- {count}\n"));
    }
    out.push('\n');

    out.push_str("## Corpus cases\n\n");
    for case in &packet.corpus_cases {
        out.push_str(&format!(
            "### `{case}` -- {label}\n\n",
            case = case.case_id,
            label = case.case_label
        ));
        out.push_str(&format!("- Surface: `{}`\n", case.surface_family.as_str()));
        out.push_str(&format!(
            "- Edge cases: {}\n",
            format_class_list(&case.edge_case_classes, |c| c.as_str())
        ));
        out.push_str(&format!(
            "- Count summary: `{}`\n",
            case.filter_bar.count_summary_class.as_str()
        ));
        out.push_str(&format!(
            "- Hidden narrowing: {}\n",
            if case.filter_bar.hidden_narrowing_summary.is_empty() {
                "(none)".to_string()
            } else {
                format!("`{}`", case.filter_bar.hidden_narrowing_summary)
            }
        ));
        out.push_str(&format!("- Anchor row: `{}`\n", case.anchor_row_id));
        out.push_str(&format!(
            "- Selected ids: [{}]\n",
            case.selected_row_ids
                .iter()
                .map(|s| format!("`{s}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        if !case.hidden_selected_row_ids.is_empty() {
            out.push_str(&format!(
                "- Hidden selected ids: [{}]\n",
                case.hidden_selected_row_ids
                    .iter()
                    .map(|s| format!("`{s}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if !case.blocked_row_ids.is_empty() {
            out.push_str(&format!(
                "- Blocked ids: [{}]\n",
                case.blocked_row_ids
                    .iter()
                    .map(|s| format!("`{s}`"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out.push_str(&format!(
            "- Batch action: `{}` ({}); continue enabled: {}\n",
            case.batch_review.action_id,
            case.batch_review.consequence_class.as_str(),
            case.batch_review.continue_enabled
        ));
        out.push_str(&format!(
            "- Select-all escalation: `{}`\n",
            case.batch_review.select_all_escalation_class.as_str()
        ));
        out.push('\n');
    }

    out.push_str("## Saved-view migration cases\n\n");
    for migration in &packet.saved_view_migrations {
        out.push_str(&format!(
            "### `{migration}` -- `{class}`\n\n",
            migration = migration.migration_case_id,
            class = migration.case_class.as_str()
        ));
        out.push_str(&format!(
            "- Surface: `{}`\n",
            migration.surface_family.as_str()
        ));
        out.push_str(&format!(
            "- Schema {from} -> {to}\n",
            from = migration.captured_schema_version,
            to = migration.restored_schema_version
        ));
        out.push_str(&format!(
            "- Restored drift: `{}`; fallback: `{}`\n",
            migration.restored_view.drift_state.as_str(),
            migration.restored_view.fallback_behavior.as_str()
        ));
        if !migration.migration_notes.is_empty() {
            out.push_str("- Migration notes:\n");
            for note in &migration.migration_notes {
                out.push_str(&format!("  - {note}\n"));
            }
        }
        if !migration.portability_findings.is_empty() {
            out.push_str("- Portability findings:\n");
            for finding in &migration.portability_findings {
                out.push_str(&format!("  - {finding}\n"));
            }
        }
        out.push('\n');
    }

    out.push_str("## Accessibility drills\n\n");
    for drill in &packet.accessibility_drills {
        out.push_str(&format!(
            "### `{drill}` -- `{class}`\n\n",
            drill = drill.drill_id,
            class = drill.drill_class.as_str()
        ));
        out.push_str(&format!("- Surface: `{}`\n", drill.surface_family.as_str()));
        out.push_str(&format!("- Label: {}\n", drill.label));
        out.push_str("- Steps:\n");
        for step in &drill.steps {
            out.push_str(&format!("  - {step}\n"));
        }
        out.push_str("- Expected assertions:\n");
        for assertion in &drill.expected_assertions {
            out.push_str(&format!("  - {assertion}\n"));
        }
        out.push_str("- Virtualization invariants:\n");
        for invariant in &drill.virtualization_invariants {
            out.push_str(&format!("  - {invariant}\n"));
        }
        out.push_str(&format!(
            "- Accessibility narration: {}\n",
            drill.accessibility_narration_summary
        ));
        out.push('\n');
    }

    out.push_str("## Support export invariants\n\n");
    out.push_str(&format!(
        "- Support export id: `{}`\n",
        packet.support_export.support_export_id
    ));
    out.push_str(&format!(
        "- Sourced packet: `{}`\n",
        packet.support_export.packet_ref
    ));
    out.push_str(&format!(
        "- No sensitive payload: {}\n",
        packet.support_export.no_sensitive_payload
    ));
    out.push_str("- Redaction rules:\n");
    for rule in &packet.support_export.redaction_rules {
        out.push_str(&format!("  - {rule}\n"));
    }
    out.push('\n');

    out.push_str("## Verification\n\n");
    out.push_str("```sh\n");
    out.push_str(
        "cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- validate\n",
    );
    out.push_str("cargo test -p aureline-shell --test collection_truth_corpus_fixtures\n");
    out.push_str("```\n");
    out
}

/// Renders the markdown drills doc consumed by `docs/qe/m3/collection_truth_drills.md`.
pub fn render_collection_truth_corpus_drills_markdown(
    packet: &CollectionTruthCorpusPacket,
) -> String {
    let mut out = String::new();
    out.push_str("# Collection-truth drills\n\n");
    out.push_str(
        "Keyboard and screen-reader drills that every claimed beta collection surface MUST pass.\n",
    );
    out.push_str("Sourced from the seeded corpus in\n");
    out.push_str(
        "`crates/aureline-shell/src/collection_truth_corpus/mod.rs`. Regenerate with:\n\n",
    );
    out.push_str("```sh\n");
    out.push_str(
        "cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- drills-md > \\\n  docs/qe/m3/collection_truth_drills.md\n",
    );
    out.push_str("```\n\n");

    out.push_str("## Drill index\n\n");
    out.push_str("| Surface | Drill class | Drill id |\n");
    out.push_str("| ------- | ----------- | -------- |\n");
    for drill in &packet.accessibility_drills {
        out.push_str(&format!(
            "| `{surface}` | `{class}` | `{drill}` |\n",
            surface = drill.surface_family.as_str(),
            class = drill.drill_class.as_str(),
            drill = drill.drill_id,
        ));
    }
    out.push('\n');

    for drill in &packet.accessibility_drills {
        out.push_str(&format!(
            "## `{drill}` -- {label}\n\n",
            drill = drill.drill_id,
            label = drill.label
        ));
        out.push_str(&format!("- Surface: `{}`\n", drill.surface_family.as_str()));
        out.push_str(&format!(
            "- Drill class: `{}`\n\n",
            drill.drill_class.as_str()
        ));
        out.push_str("### Steps\n\n");
        for step in &drill.steps {
            out.push_str(&format!("1. {step}\n"));
        }
        out.push('\n');
        out.push_str("### Expected assertions\n\n");
        for assertion in &drill.expected_assertions {
            out.push_str(&format!("- {assertion}\n"));
        }
        out.push('\n');
        out.push_str("### Virtualization invariants\n\n");
        for invariant in &drill.virtualization_invariants {
            out.push_str(&format!("- {invariant}\n"));
        }
        out.push('\n');
        out.push_str("### Accessibility narration\n\n");
        out.push_str(&format!("> {}\n\n", drill.accessibility_narration_summary));
    }
    out
}

fn format_class_list<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    if items.is_empty() {
        "(none)".to_string()
    } else {
        items
            .iter()
            .map(|item| format!("`{}`", to_token(item)))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

mod seeds {
    use super::*;
    use crate::collection_truth::BatchReviewBlockedReasonClass;

    pub(super) fn all_corpus_cases(
        base_packet: &CollectionTruthBetaPacket,
    ) -> Vec<CollectionTruthCorpusCase> {
        let mut cases = Vec::new();
        for base_case in &base_packet.cases {
            cases.push(promote_base_case(base_case));
        }
        cases.push(virtualized_window_review_case());
        cases.push(stale_provider_cursor_log_case());
        cases.push(hidden_selected_admin_case());
        cases
    }

    pub(super) fn all_saved_view_migrations() -> Vec<SavedViewMigrationCase> {
        vec![
            older_schema_exact_migration(),
            older_schema_degraded_migration(),
            unsupported_column_preset_migration(),
            stale_provider_cursor_migration(),
            stale_provider_owned_scope_migration(),
            policy_narrowed_collection_migration(),
        ]
    }

    pub(super) fn all_accessibility_drills() -> Vec<CollectionTruthAccessibilityDrill> {
        vec![
            anchor_range_selection_drill(),
            hidden_selected_inspection_drill(),
            batch_review_open_drill(),
            saved_view_switcher_drill(),
        ]
    }

    fn promote_base_case(base_case: &CollectionTruthCase) -> CollectionTruthCorpusCase {
        let edge_case_classes = classify_base_case(base_case);
        let (selected, hidden_selected, blocked) = selection_seeds_for(base_case);
        CollectionTruthCorpusCase {
            record_kind: COLLECTION_TRUTH_CORPUS_CASE_RECORD_KIND.to_string(),
            schema_version: COLLECTION_TRUTH_CORPUS_SCHEMA_VERSION,
            case_id: format!("corpus:{}", base_case.case_id),
            surface_family: base_case.surface_family,
            case_label: format!("{} (base)", base_case.case_label),
            edge_case_classes,
            filter_bar: base_case.filter_bar.clone(),
            saved_view: base_case.saved_view.clone(),
            scope_counter: base_case.scope_counter.clone(),
            batch_review: base_case.batch_review.clone(),
            anchor_row_id: format!(
                "anchor:{family}:row:0",
                family = base_case.surface_family.as_str()
            ),
            selected_row_ids: selected,
            hidden_selected_row_ids: hidden_selected,
            blocked_row_ids: blocked,
        }
    }

    fn classify_base_case(base_case: &CollectionTruthCase) -> Vec<CollectionTruthEdgeCaseClass> {
        use CollectionTruthEdgeCaseClass as Class;
        let mut classes: BTreeSet<Class> = BTreeSet::new();
        let counts = &base_case.scope_counter;
        let counter_classes: BTreeSet<_> = counts
            .rows
            .iter()
            .map(|row| (row.counter_class, row.status))
            .collect();
        let visible_eq_loaded = counts
            .rows
            .iter()
            .find(|row| row.counter_class == ScopeCounterClass::Visible)
            .and_then(|row| row.value)
            == counts
                .rows
                .iter()
                .find(|row| row.counter_class == ScopeCounterClass::Loaded)
                .and_then(|row| row.value);
        let total_exact_value = counts
            .rows
            .iter()
            .find(|row| {
                row.counter_class == ScopeCounterClass::Total
                    && row.status == ScopeCounterStatus::Exact
            })
            .and_then(|row| row.value);
        let matching_exact_value = counts
            .rows
            .iter()
            .find(|row| {
                row.counter_class == ScopeCounterClass::Matching
                    && row.status == ScopeCounterStatus::Exact
            })
            .and_then(|row| row.value);
        if visible_eq_loaded
            && total_exact_value.is_some()
            && total_exact_value == matching_exact_value
        {
            classes.insert(Class::VisibleEqualsLoadedExactTotal);
        }
        if counts.rows.iter().any(|row| {
            row.counter_class == ScopeCounterClass::Total
                && row.status == ScopeCounterStatus::Partial
        }) {
            classes.insert(Class::VirtualizedWindowPartialMatching);
        }
        if counter_classes.iter().any(|(class, status)| {
            *class == ScopeCounterClass::Matching
                && (*status == ScopeCounterStatus::Approximate
                    || *status == ScopeCounterStatus::ProviderLimited)
        }) {
            classes.insert(Class::ProviderCappedApproximateMatching);
        }
        if counter_classes.iter().any(|(class, status)| {
            *class == ScopeCounterClass::Total && *status == ScopeCounterStatus::Unknown
        }) {
            classes.insert(Class::ProviderRetentionUnknownTotal);
        }
        if base_case.batch_review.summary.blocked_count > 0 {
            classes.insert(Class::BlockedRowsPresent);
        }
        if base_case.batch_review.summary.hidden_count > 0 {
            classes.insert(Class::HiddenSelectedRowsPresent);
        }
        classes.into_iter().collect()
    }

    fn selection_seeds_for(
        base_case: &CollectionTruthCase,
    ) -> (Vec<String>, Vec<String>, Vec<String>) {
        let family = base_case.surface_family.as_str();
        let mut selected: Vec<String> = (0..base_case.batch_review.summary.included_count.min(3))
            .map(|i| format!("row:{family}:included:{i}"))
            .collect();
        if base_case.batch_review.summary.included_count > 3 {
            selected.push(format!("row:{family}:included:plus-more"));
        }
        let hidden = if base_case.batch_review.summary.hidden_count > 0 {
            (0..base_case.batch_review.summary.hidden_count.min(2))
                .map(|i| format!("row:{family}:hidden-selected:{i}"))
                .collect()
        } else {
            Vec::new()
        };
        let blocked = if base_case.batch_review.summary.blocked_count > 0 {
            (0..base_case.batch_review.summary.blocked_count.min(2))
                .map(|i| format!("row:{family}:blocked:{i}"))
                .collect()
        } else {
            Vec::new()
        };
        (selected, hidden, blocked)
    }

    fn virtualized_window_review_case() -> CollectionTruthCorpusCase {
        let family = CollectionTruthSurfaceFamily::ReviewInbox;
        let chips = vec![
            FilterBarChipRecord::user_facet("state", "needs review"),
            FilterBarChipRecord::workset_narrowed(
                "workset",
                "release branch",
                "active workset narrows queue",
            ),
            FilterBarChipRecord::partial_data_disclosed(
                "Reviews still loading",
                "row materialisation lags virtualization window",
            ),
        ];
        let filter_bar = FilterBarStateRecord::new(
            "filter-bar:review:virtualized-window",
            family,
            "Review queue (virtualized window)",
            chips,
            CountSummaryClass::PartialIndexing,
            "reset filter bar to defaults",
        );
        let saved_view = SavedCollectionViewRecord::new(
            "saved-view:review:virtualized-window",
            family,
            "Virtualized review queue",
            SavedViewScopeClass::User,
            SavedViewDriftState::BoundCurrentStateMatchesCaptured,
            SavedViewFallbackBehavior::PreserveAndLabelDegraded,
            vec![
                SavedViewColumnPreset::new("title", "Title", true),
                SavedViewColumnPreset::new("age", "Age", false),
            ],
            vec![("age", false)],
            vec![
                SavedViewPinnedCountAxis::Visible,
                SavedViewPinnedCountAxis::Loaded,
                SavedViewPinnedCountAxis::Matching,
                SavedViewPinnedCountAxis::Total,
            ],
            Vec::new(),
        );
        let scope_counter = CollectionScopeCounterRecord::new(
            "scope-counter:review:virtualized-window",
            family,
            vec![
                CollectionScopeCounterRow::visible(40),
                CollectionScopeCounterRow::loaded(200),
                CollectionScopeCounterRow::matching_exact(640),
                CollectionScopeCounterRow::total_partial(820),
                CollectionScopeCounterRow::partial(180, "180 rows still materialising"),
            ],
            "Visible 40, loaded 200, 640 matches; total partial at 820.".to_string(),
        );
        let batch_review = BatchReviewSheetRecord::new(
            "batch-review:review:virtualized-window-approve",
            family,
            "review.approve_visible_window",
            "Approve visible window",
            BatchActionConsequenceClass::RemoteMutation,
            SelectAllEscalationClass::VisibleOrLoaded,
            "mixed_client_then_provider",
            BatchReviewSummary {
                included_count: 40,
                excluded_count: 160,
                blocked_count: 2,
                hidden_count: 600,
                selected_versus_all_matching_label:
                    "Approves the 40 visible reviews; 600 matches outside the viewport stay out unless escalated."
                        .to_string(),
            },
            vec![BatchReviewBlockedReason::new(
                BatchReviewBlockedReasonClass::FreshnessRequired,
                "2 rows missing required freshness signal",
            )],
            RecoveryGuidanceClass::CompensatingRevertWithinWindow,
            "Cancel discards staged approvals without sending.".to_string(),
        );
        CollectionTruthCorpusCase {
            record_kind: COLLECTION_TRUTH_CORPUS_CASE_RECORD_KIND.to_string(),
            schema_version: COLLECTION_TRUTH_CORPUS_SCHEMA_VERSION,
            case_id: "corpus:review:virtualized-window".to_string(),
            surface_family: family,
            case_label: "Review inbox -- virtualized window approve".to_string(),
            edge_case_classes: vec![
                CollectionTruthEdgeCaseClass::VirtualizedWindowPartialMatching,
                CollectionTruthEdgeCaseClass::BlockedRowsPresent,
                CollectionTruthEdgeCaseClass::HiddenSelectedRowsPresent,
            ],
            filter_bar,
            saved_view,
            scope_counter,
            batch_review,
            anchor_row_id: "anchor:review:virtualized-window:row:0".to_string(),
            selected_row_ids: vec![
                "row:review:virtualized-window:included:0".to_string(),
                "row:review:virtualized-window:included:1".to_string(),
                "row:review:virtualized-window:included:plus-more".to_string(),
            ],
            hidden_selected_row_ids: vec![
                "row:review:virtualized-window:hidden-selected:0".to_string(),
                "row:review:virtualized-window:hidden-selected:1".to_string(),
            ],
            blocked_row_ids: vec![
                "row:review:virtualized-window:blocked:0".to_string(),
                "row:review:virtualized-window:blocked:1".to_string(),
            ],
        }
    }

    fn stale_provider_cursor_log_case() -> CollectionTruthCorpusCase {
        let family = CollectionTruthSurfaceFamily::LogOrEventCollection;
        let chips = vec![
            FilterBarChipRecord::user_facet("severity", "error"),
            FilterBarChipRecord::provider_limit_disclosed(
                "Provider cursor stale",
                "last known cursor refers to retention window that has rotated",
            ),
            FilterBarChipRecord::client_limit_disclosed(
                "Client window 5,000 lines",
                "older lines paged out",
            ),
        ];
        let filter_bar = FilterBarStateRecord::new(
            "filter-bar:logs:stale-cursor",
            family,
            "Incident logs (stale cursor)",
            chips,
            CountSummaryClass::ProviderRetentionWindowed,
            "reset filter bar to defaults",
        );
        let saved_view = SavedCollectionViewRecord::new(
            "saved-view:logs:stale-cursor",
            family,
            "Stale cursor view",
            SavedViewScopeClass::Shared,
            SavedViewDriftState::ViewUnavailableProviderOfflineDisclosed,
            SavedViewFallbackBehavior::ProviderRebindRequired,
            vec![
                SavedViewColumnPreset::new("timestamp", "Time", true),
                SavedViewColumnPreset::new("source", "Source", true),
            ],
            vec![("timestamp", true)],
            vec![SavedViewPinnedCountAxis::Visible],
            vec![
                "Last provider cursor refers to rotated retention window".to_string(),
                "Restore deferred; rebind required before continuing".to_string(),
            ],
        );
        let scope_counter = CollectionScopeCounterRecord::new(
            "scope-counter:logs:stale-cursor",
            family,
            vec![
                CollectionScopeCounterRow::visible(0),
                CollectionScopeCounterRow::loaded(0),
                CollectionScopeCounterRow::matching_provider_limited(0, 24),
                CollectionScopeCounterRow::total_unknown_due_to_retention(),
                CollectionScopeCounterRow::provider_owned_unknown(),
            ],
            "Cursor stale; provider has not returned an authoritative count.".to_string(),
        );
        let batch_review = BatchReviewSheetRecord::new(
            "batch-review:logs:stale-cursor-rebind",
            family,
            "logs.rebind_cursor",
            "Rebind log cursor",
            BatchActionConsequenceClass::RoutineNonMutating,
            SelectAllEscalationClass::VisibleOrLoaded,
            "client_local_execution",
            BatchReviewSummary {
                included_count: 0,
                excluded_count: 0,
                blocked_count: 0,
                hidden_count: 0,
                selected_versus_all_matching_label:
                    "Rebinds the cursor; nothing is exported until rebind succeeds.".to_string(),
            },
            Vec::new(),
            RecoveryGuidanceClass::EvidenceOnlyNoRerun,
            "Cancel keeps the stale cursor labeled.".to_string(),
        );
        CollectionTruthCorpusCase {
            record_kind: COLLECTION_TRUTH_CORPUS_CASE_RECORD_KIND.to_string(),
            schema_version: COLLECTION_TRUTH_CORPUS_SCHEMA_VERSION,
            case_id: "corpus:logs:stale-cursor".to_string(),
            surface_family: family,
            case_label: "Logs -- stale provider cursor".to_string(),
            edge_case_classes: vec![
                CollectionTruthEdgeCaseClass::ProviderCappedApproximateMatching,
                CollectionTruthEdgeCaseClass::ProviderRetentionUnknownTotal,
                CollectionTruthEdgeCaseClass::StaleProviderCursorDetected,
            ],
            filter_bar,
            saved_view,
            scope_counter,
            batch_review,
            anchor_row_id: "anchor:logs:stale-cursor:row:0".to_string(),
            selected_row_ids: Vec::new(),
            hidden_selected_row_ids: Vec::new(),
            blocked_row_ids: Vec::new(),
        }
    }

    fn hidden_selected_admin_case() -> CollectionTruthCorpusCase {
        let family = CollectionTruthSurfaceFamily::AdminOrSettingsGrid;
        let chips = vec![
            FilterBarChipRecord::user_facet("status", "pending"),
            FilterBarChipRecord::policy_narrowed(
                "tenant",
                "Tenant A",
                "policy pinned tenant scope",
            ),
        ];
        let filter_bar = FilterBarStateRecord::new(
            "filter-bar:admin:hidden-selected",
            family,
            "Admin pending invites",
            chips,
            CountSummaryClass::ExactWithPolicyPinning,
            "reset filter bar to defaults",
        );
        let saved_view = SavedCollectionViewRecord::new(
            "saved-view:admin:hidden-selected",
            family,
            "Pending invites",
            SavedViewScopeClass::PolicyPinned,
            SavedViewDriftState::BoundCurrentStateMatchesCaptured,
            SavedViewFallbackBehavior::PreserveAndLabelDegraded,
            vec![
                SavedViewColumnPreset::new("name", "Name", true),
                SavedViewColumnPreset::new("status", "Status", true),
            ],
            vec![("name", false)],
            vec![
                SavedViewPinnedCountAxis::Visible,
                SavedViewPinnedCountAxis::Matching,
                SavedViewPinnedCountAxis::Total,
            ],
            Vec::new(),
        );
        let scope_counter = CollectionScopeCounterRecord::new(
            "scope-counter:admin:hidden-selected",
            family,
            vec![
                CollectionScopeCounterRow::visible(20),
                CollectionScopeCounterRow::loaded(20),
                CollectionScopeCounterRow::matching_exact(20),
                CollectionScopeCounterRow::total_exact(60),
            ],
            "20 of 60 admin invites match the policy-pinned tenant.".to_string(),
        );
        let batch_review = BatchReviewSheetRecord::new(
            "batch-review:admin:revoke-selected",
            family,
            "admin.revoke_invites_selected",
            "Revoke selected invites",
            BatchActionConsequenceClass::DestructiveRemote,
            SelectAllEscalationClass::VisibleOrLoaded,
            "provider_authoritative_execution",
            BatchReviewSummary {
                included_count: 5,
                excluded_count: 0,
                blocked_count: 2,
                hidden_count: 3,
                selected_versus_all_matching_label:
                    "Revokes 5 selected invites; 3 hidden selected invites and 2 admin-pinned invites stay."
                        .to_string(),
            },
            vec![BatchReviewBlockedReason::policy_narrowed(
                "2 admin-pinned invites blocked by org policy",
            )],
            RecoveryGuidanceClass::CompensatingRevertWithinWindow,
            "Cancel keeps invites in place; re-issuance is possible from admin tools.".to_string(),
        );
        CollectionTruthCorpusCase {
            record_kind: COLLECTION_TRUTH_CORPUS_CASE_RECORD_KIND.to_string(),
            schema_version: COLLECTION_TRUTH_CORPUS_SCHEMA_VERSION,
            case_id: "corpus:admin:hidden-selected-revoke".to_string(),
            surface_family: family,
            case_label: "Admin grid -- hidden selected revoke".to_string(),
            edge_case_classes: vec![
                CollectionTruthEdgeCaseClass::VisibleEqualsLoadedExactTotal,
                CollectionTruthEdgeCaseClass::BlockedRowsPresent,
                CollectionTruthEdgeCaseClass::HiddenSelectedRowsPresent,
            ],
            filter_bar,
            saved_view,
            scope_counter,
            batch_review,
            anchor_row_id: "anchor:admin:hidden-selected:row:0".to_string(),
            selected_row_ids: vec![
                "row:admin:hidden-selected:included:0".to_string(),
                "row:admin:hidden-selected:included:1".to_string(),
                "row:admin:hidden-selected:included:plus-more".to_string(),
            ],
            hidden_selected_row_ids: vec![
                "row:admin:hidden-selected:hidden-selected:0".to_string(),
                "row:admin:hidden-selected:hidden-selected:1".to_string(),
            ],
            blocked_row_ids: vec![
                "row:admin:hidden-selected:blocked:0".to_string(),
                "row:admin:hidden-selected:blocked:1".to_string(),
            ],
        }
    }

    fn older_schema_exact_migration() -> SavedViewMigrationCase {
        let captured = SavedCollectionViewRecord::new(
            "saved-view:packages:outdated:v0",
            CollectionTruthSurfaceFamily::PackageOrInventoryGrid,
            "Outdated extensions",
            SavedViewScopeClass::Workspace,
            SavedViewDriftState::BoundCurrentStateMatchesCaptured,
            SavedViewFallbackBehavior::PreserveAndLabelDegraded,
            vec![
                SavedViewColumnPreset::new("name", "Name", true),
                SavedViewColumnPreset::new("publisher", "Publisher", true),
            ],
            vec![("name", false)],
            vec![SavedViewPinnedCountAxis::Visible],
            Vec::new(),
        );
        let restored = captured.clone();
        SavedViewMigrationCase {
            record_kind: SAVED_VIEW_MIGRATION_CASE_RECORD_KIND.to_string(),
            schema_version: COLLECTION_TRUTH_CORPUS_SCHEMA_VERSION,
            migration_case_id: "migration:packages:outdated:exact".to_string(),
            surface_family: CollectionTruthSurfaceFamily::PackageOrInventoryGrid,
            case_class: SavedViewMigrationCaseClass::OlderSchemaVersionUpgradedExact,
            captured_schema_version: 0,
            restored_schema_version: 1,
            captured_view: captured,
            restored_view: restored,
            migration_notes: vec!["v0 -> v1 upgrade applied without semantic loss".to_string()],
            portability_findings: Vec::new(),
        }
    }

    fn older_schema_degraded_migration() -> SavedViewMigrationCase {
        let captured = SavedCollectionViewRecord::new(
            "saved-view:search:service-tier:v0",
            CollectionTruthSurfaceFamily::SearchOrResultGrid,
            "Service tier",
            SavedViewScopeClass::Workspace,
            SavedViewDriftState::BoundCurrentStateMatchesCaptured,
            SavedViewFallbackBehavior::PreserveAndLabelDegraded,
            vec![
                SavedViewColumnPreset::new("path", "Path", true),
                SavedViewColumnPreset::new("matches", "Matches", false),
                SavedViewColumnPreset::new("legacy_owner", "Owner", false),
            ],
            vec![("matches", true)],
            vec![SavedViewPinnedCountAxis::Visible],
            Vec::new(),
        );
        let restored = SavedCollectionViewRecord::new(
            "saved-view:search:service-tier",
            CollectionTruthSurfaceFamily::SearchOrResultGrid,
            "Service tier",
            SavedViewScopeClass::Workspace,
            SavedViewDriftState::ColumnSetDriftedDisclosed,
            SavedViewFallbackBehavior::LoadPortableSubsetWithLabels,
            vec![
                SavedViewColumnPreset::new("path", "Path", true),
                SavedViewColumnPreset::new("matches", "Matches", false),
            ],
            vec![("matches", true)],
            vec![SavedViewPinnedCountAxis::Visible],
            vec!["v0 column `legacy_owner` no longer exists".to_string()],
        );
        SavedViewMigrationCase {
            record_kind: SAVED_VIEW_MIGRATION_CASE_RECORD_KIND.to_string(),
            schema_version: COLLECTION_TRUTH_CORPUS_SCHEMA_VERSION,
            migration_case_id: "migration:search:service-tier:degraded".to_string(),
            surface_family: CollectionTruthSurfaceFamily::SearchOrResultGrid,
            case_class: SavedViewMigrationCaseClass::OlderSchemaVersionUpgradedDegraded,
            captured_schema_version: 0,
            restored_schema_version: 1,
            captured_view: captured,
            restored_view: restored,
            migration_notes: vec![
                "v0 -> v1 upgrade dropped removed columns and surfaced disclosure label"
                    .to_string(),
            ],
            portability_findings: vec![
                "captured column `legacy_owner` not present in v1 schema".to_string()
            ],
        }
    }

    fn unsupported_column_preset_migration() -> SavedViewMigrationCase {
        let captured = SavedCollectionViewRecord::new(
            "saved-view:work-items:my-board:v1-unsupported",
            CollectionTruthSurfaceFamily::WorkItemBoard,
            "My board",
            SavedViewScopeClass::Shared,
            SavedViewDriftState::BoundCurrentStateMatchesCaptured,
            SavedViewFallbackBehavior::PreserveAndLabelDegraded,
            vec![
                SavedViewColumnPreset::new("title", "Title", true),
                SavedViewColumnPreset::new("state", "State", true),
                SavedViewColumnPreset::new("legacy_swimlane", "Swimlane", false),
            ],
            vec![("state", false)],
            vec![SavedViewPinnedCountAxis::Visible],
            Vec::new(),
        );
        let restored = SavedCollectionViewRecord::new(
            "saved-view:work-items:my-board",
            CollectionTruthSurfaceFamily::WorkItemBoard,
            "My board",
            SavedViewScopeClass::Shared,
            SavedViewDriftState::ColumnSetDriftedDisclosed,
            SavedViewFallbackBehavior::LoadPortableSubsetWithLabels,
            vec![
                SavedViewColumnPreset::new("title", "Title", true),
                SavedViewColumnPreset::new("state", "State", true),
            ],
            vec![("state", false)],
            vec![SavedViewPinnedCountAxis::Visible],
            vec![
                "Unsupported column preset `legacy_swimlane` dropped from portable subset"
                    .to_string(),
            ],
        );
        SavedViewMigrationCase {
            record_kind: SAVED_VIEW_MIGRATION_CASE_RECORD_KIND.to_string(),
            schema_version: COLLECTION_TRUTH_CORPUS_SCHEMA_VERSION,
            migration_case_id: "migration:work-items:my-board:unsupported-preset".to_string(),
            surface_family: CollectionTruthSurfaceFamily::WorkItemBoard,
            case_class: SavedViewMigrationCaseClass::UnsupportedColumnPresetDroppedLabeled,
            captured_schema_version: 1,
            restored_schema_version: 1,
            captured_view: captured,
            restored_view: restored,
            migration_notes: vec![
                "Unsupported column preset dropped and labeled rather than silently misinterpreted"
                    .to_string(),
            ],
            portability_findings: vec![
                "captured column `legacy_swimlane` is not supported on the current surface"
                    .to_string(),
            ],
        }
    }

    fn stale_provider_cursor_migration() -> SavedViewMigrationCase {
        let captured = SavedCollectionViewRecord::new(
            "saved-view:logs:errors-last-day:v0",
            CollectionTruthSurfaceFamily::LogOrEventCollection,
            "Errors - last 24 h",
            SavedViewScopeClass::Shared,
            SavedViewDriftState::BoundCurrentStateMatchesCaptured,
            SavedViewFallbackBehavior::PreserveAndLabelDegraded,
            vec![
                SavedViewColumnPreset::new("timestamp", "Time", true),
                SavedViewColumnPreset::new("source", "Source", true),
            ],
            vec![("timestamp", true)],
            vec![SavedViewPinnedCountAxis::Visible],
            Vec::new(),
        );
        let restored = SavedCollectionViewRecord::new(
            "saved-view:logs:errors-last-day",
            CollectionTruthSurfaceFamily::LogOrEventCollection,
            "Errors - last 24 h",
            SavedViewScopeClass::Shared,
            SavedViewDriftState::ViewUnavailableProviderOfflineDisclosed,
            SavedViewFallbackBehavior::ProviderRebindRequired,
            vec![
                SavedViewColumnPreset::new("timestamp", "Time", true),
                SavedViewColumnPreset::new("source", "Source", true),
            ],
            vec![("timestamp", true)],
            vec![SavedViewPinnedCountAxis::Visible],
            vec![
                "Captured provider cursor refers to a rotated retention window".to_string(),
                "Restore deferred; rebind required before reuse".to_string(),
            ],
        );
        SavedViewMigrationCase {
            record_kind: SAVED_VIEW_MIGRATION_CASE_RECORD_KIND.to_string(),
            schema_version: COLLECTION_TRUTH_CORPUS_SCHEMA_VERSION,
            migration_case_id: "migration:logs:stale-cursor".to_string(),
            surface_family: CollectionTruthSurfaceFamily::LogOrEventCollection,
            case_class: SavedViewMigrationCaseClass::StaleProviderCursorRefusedAndOffered,
            captured_schema_version: 0,
            restored_schema_version: 1,
            captured_view: captured,
            restored_view: restored,
            migration_notes: vec!["Stale provider cursor refused; rebind path offered".to_string()],
            portability_findings: vec![
                "Provider cursors are never restored verbatim; rebind required".to_string(),
            ],
        }
    }

    fn stale_provider_owned_scope_migration() -> SavedViewMigrationCase {
        let captured = SavedCollectionViewRecord::new(
            "saved-view:packages:provider-owned-scope:v0",
            CollectionTruthSurfaceFamily::PackageOrInventoryGrid,
            "Provider catalog scope",
            SavedViewScopeClass::ProviderOwned,
            SavedViewDriftState::BoundCurrentStateMatchesCaptured,
            SavedViewFallbackBehavior::PreserveAndLabelDegraded,
            vec![SavedViewColumnPreset::new("name", "Name", true)],
            vec![("name", false)],
            vec![SavedViewPinnedCountAxis::Visible],
            Vec::new(),
        );
        let restored = SavedCollectionViewRecord::new(
            "saved-view:packages:provider-owned-scope",
            CollectionTruthSurfaceFamily::PackageOrInventoryGrid,
            "Provider catalog scope",
            SavedViewScopeClass::ProviderOwned,
            SavedViewDriftState::ViewUnresolvableOfferedRecreate,
            SavedViewFallbackBehavior::OfferRecreateFromCurrent,
            vec![SavedViewColumnPreset::new("name", "Name", true)],
            vec![("name", false)],
            vec![SavedViewPinnedCountAxis::Visible],
            vec!["Provider-owned scope no longer resolves on current provider catalog".to_string()],
        );
        SavedViewMigrationCase {
            record_kind: SAVED_VIEW_MIGRATION_CASE_RECORD_KIND.to_string(),
            schema_version: COLLECTION_TRUTH_CORPUS_SCHEMA_VERSION,
            migration_case_id: "migration:packages:stale-provider-owned-scope".to_string(),
            surface_family: CollectionTruthSurfaceFamily::PackageOrInventoryGrid,
            case_class: SavedViewMigrationCaseClass::StaleProviderOwnedScopeOfferRecreate,
            captured_schema_version: 0,
            restored_schema_version: 1,
            captured_view: captured,
            restored_view: restored,
            migration_notes: vec![
                "Provider-owned scope refused; recreate-from-current path offered".to_string(),
            ],
            portability_findings: vec![
                "Provider-owned scope is never reused silently when the provider catalog changes"
                    .to_string(),
            ],
        }
    }

    fn policy_narrowed_collection_migration() -> SavedViewMigrationCase {
        let captured = SavedCollectionViewRecord::new(
            "saved-view:admin:identities:v0",
            CollectionTruthSurfaceFamily::AdminOrSettingsGrid,
            "Tenant admins",
            SavedViewScopeClass::PolicyPinned,
            SavedViewDriftState::BoundCurrentStateMatchesCaptured,
            SavedViewFallbackBehavior::PreserveAndLabelDegraded,
            vec![
                SavedViewColumnPreset::new("name", "Name", true),
                SavedViewColumnPreset::new("role", "Role", true),
            ],
            vec![("name", false)],
            vec![SavedViewPinnedCountAxis::Visible],
            Vec::new(),
        );
        let restored = SavedCollectionViewRecord::new(
            "saved-view:admin:identities:default",
            CollectionTruthSurfaceFamily::AdminOrSettingsGrid,
            "Tenant admins",
            SavedViewScopeClass::PolicyPinned,
            SavedViewDriftState::PolicyNarrowingChangedDisclosed,
            SavedViewFallbackBehavior::RefuseUntilRebound,
            vec![
                SavedViewColumnPreset::new("name", "Name", true),
                SavedViewColumnPreset::new("role", "Role", true),
            ],
            vec![("name", false)],
            vec![SavedViewPinnedCountAxis::Visible],
            vec![
                "Policy narrowing changed since this view was captured; rebind required"
                    .to_string(),
            ],
        );
        SavedViewMigrationCase {
            record_kind: SAVED_VIEW_MIGRATION_CASE_RECORD_KIND.to_string(),
            schema_version: COLLECTION_TRUTH_CORPUS_SCHEMA_VERSION,
            migration_case_id: "migration:admin:policy-narrowed-rebound".to_string(),
            surface_family: CollectionTruthSurfaceFamily::AdminOrSettingsGrid,
            case_class: SavedViewMigrationCaseClass::PolicyNarrowedCollectionRebound,
            captured_schema_version: 0,
            restored_schema_version: 1,
            captured_view: captured,
            restored_view: restored,
            migration_notes: vec![
                "Policy narrowing changed since capture; rebind required before reuse".to_string(),
            ],
            portability_findings: vec![
                "Captured policy-pinned scope is no longer authoritative".to_string()
            ],
        }
    }

    fn anchor_range_selection_drill() -> CollectionTruthAccessibilityDrill {
        CollectionTruthAccessibilityDrill {
            record_kind: COLLECTION_TRUTH_DRILL_RECORD_KIND.to_string(),
            schema_version: COLLECTION_TRUTH_CORPUS_SCHEMA_VERSION,
            drill_id: "drill:keyboard-anchor-range-selection".to_string(),
            surface_family: CollectionTruthSurfaceFamily::SearchOrResultGrid,
            drill_class: CollectionTruthAccessibilityDrillClass::KeyboardAnchorRangeSelection,
            label: "Anchor-based range selection across virtualized rows".to_string(),
            steps: vec![
                "Focus the first row to set the anchor".to_string(),
                "Press Shift+ArrowDown to extend the range to the next row".to_string(),
                "Press Shift+PageDown to extend across the virtualization window boundary"
                    .to_string(),
                "Press Shift+End to extend through the last loaded row".to_string(),
            ],
            expected_assertions: vec![
                "Anchor row id stays stable across virtualization window scrolls".to_string(),
                "Range selection never includes blocked or hidden rows silently".to_string(),
                "Selected count narration matches `visible_or_loaded` escalation".to_string(),
            ],
            virtualization_invariants: vec![
                "Anchor row remains in the loaded buffer after window recycling".to_string(),
                "Selection state survives a viewport scroll cycle".to_string(),
                "Loaded count never drops below the anchor row index".to_string(),
            ],
            accessibility_narration_summary:
                "`Selected 5 of 200 loaded rows; 0 hidden, 2 blocked; anchor row 1.`".to_string(),
        }
    }

    fn hidden_selected_inspection_drill() -> CollectionTruthAccessibilityDrill {
        CollectionTruthAccessibilityDrill {
            record_kind: COLLECTION_TRUTH_DRILL_RECORD_KIND.to_string(),
            schema_version: COLLECTION_TRUTH_CORPUS_SCHEMA_VERSION,
            drill_id: "drill:screen-reader-hidden-selected-inspection".to_string(),
            surface_family: CollectionTruthSurfaceFamily::AdminOrSettingsGrid,
            drill_class:
                CollectionTruthAccessibilityDrillClass::ScreenReaderHiddenSelectedInspection,
            label: "Inspect hidden-selected count via screen reader".to_string(),
            steps: vec![
                "Activate the scope counter strip via the keyboard".to_string(),
                "Navigate to the hidden-selected count row".to_string(),
                "Activate the Inspect hidden selected affordance".to_string(),
                "Verify the hidden-selected rows are narrated by their stable ids".to_string(),
            ],
            expected_assertions: vec![
                "Hidden-selected count is non-zero and visible to the screen reader".to_string(),
                "Inspect affordance announces stable row ids without payload literals".to_string(),
                "Batch review summary hidden_count equals the hidden-selected count".to_string(),
            ],
            virtualization_invariants: vec![
                "Hidden-selected rows remain countable when scrolled out of the viewport"
                    .to_string(),
                "Scope counter strip never collapses hidden into visible".to_string(),
            ],
            accessibility_narration_summary:
                "`3 selected rows are hidden by the current view. Inspect to review them before continuing.`"
                    .to_string(),
        }
    }

    fn batch_review_open_drill() -> CollectionTruthAccessibilityDrill {
        CollectionTruthAccessibilityDrill {
            record_kind: COLLECTION_TRUTH_DRILL_RECORD_KIND.to_string(),
            schema_version: COLLECTION_TRUTH_CORPUS_SCHEMA_VERSION,
            drill_id: "drill:keyboard-batch-review-open".to_string(),
            surface_family: CollectionTruthSurfaceFamily::ReviewInbox,
            drill_class: CollectionTruthAccessibilityDrillClass::KeyboardBatchReviewOpen,
            label: "Open the batch-review sheet from a consequential action".to_string(),
            steps: vec![
                "Focus the consequential action affordance".to_string(),
                "Press Enter to request the action".to_string(),
                "Verify the batch-review sheet captures focus".to_string(),
                "Tab through included, excluded, blocked, and hidden count rows".to_string(),
                "Verify continue is disabled when the scope is ambiguous".to_string(),
            ],
            expected_assertions: vec![
                "Sheet appears before destructive, export-bearing, or provider-backed actions"
                    .to_string(),
                "Sheet exposes included, excluded, blocked, and hidden rows distinctly".to_string(),
                "Continue control reflects `continue_enabled` from the record".to_string(),
            ],
            virtualization_invariants: vec![
                "Sheet survives the virtualization window scrolling underneath".to_string(),
                "Selected count rendered on the sheet matches the count strip".to_string(),
            ],
            accessibility_narration_summary:
                "`Batch review: 6 included, 12 excluded, 0 blocked, 2 hidden. Continue available; cancel restores selection.`"
                    .to_string(),
        }
    }

    fn saved_view_switcher_drill() -> CollectionTruthAccessibilityDrill {
        CollectionTruthAccessibilityDrill {
            record_kind: COLLECTION_TRUTH_DRILL_RECORD_KIND.to_string(),
            schema_version: COLLECTION_TRUTH_CORPUS_SCHEMA_VERSION,
            drill_id: "drill:saved-view-switcher-virtualized".to_string(),
            surface_family: CollectionTruthSurfaceFamily::LogOrEventCollection,
            drill_class:
                CollectionTruthAccessibilityDrillClass::SavedViewSwitcherUnderVirtualization,
            label: "Switch saved view under virtualization".to_string(),
            steps: vec![
                "Focus the saved-view switcher".to_string(),
                "Select a drifted saved view".to_string(),
                "Verify the drift disclosure is narrated".to_string(),
                "Switch back and confirm anchor row id is restored".to_string(),
            ],
            expected_assertions: vec![
                "Drift disclosure (provider/policy/columns) is announced before switching".to_string(),
                "Fallback behavior (`preserve`, `subset`, `refuse`, `rebind`, `recreate`) is announced"
                    .to_string(),
                "Anchor row id survives switching back to the previous saved view".to_string(),
            ],
            virtualization_invariants: vec![
                "Loaded buffer does not collapse during switch".to_string(),
                "Scope counter axes refresh without dropping `visible` or `loaded`".to_string(),
            ],
            accessibility_narration_summary:
                "`Saved view changed to Errors -- last 24 h. Captured cursor stale; rebind required before reuse.`"
                    .to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_corpus_packet_validates() {
        let packet = seeded_collection_truth_corpus_packet();
        validate_collection_truth_corpus_packet(&packet)
            .expect("seeded corpus packet must validate");
    }

    #[test]
    fn corpus_covers_every_surface_family() {
        let packet = seeded_collection_truth_corpus_packet();
        for required in CollectionTruthSurfaceFamily::all() {
            assert!(
                packet
                    .coverage_summary
                    .surface_families_present
                    .contains(&required),
                "missing surface family {}",
                required.as_str()
            );
        }
    }

    #[test]
    fn corpus_covers_every_edge_case_class() {
        let packet = seeded_collection_truth_corpus_packet();
        for required in CollectionTruthEdgeCaseClass::all() {
            assert!(
                packet
                    .coverage_summary
                    .edge_case_classes_present
                    .contains(&required),
                "missing edge case class {}",
                required.as_str()
            );
        }
    }

    #[test]
    fn corpus_covers_every_saved_view_migration_class() {
        let packet = seeded_collection_truth_corpus_packet();
        for required in SavedViewMigrationCaseClass::all() {
            assert!(
                packet
                    .coverage_summary
                    .saved_view_migration_classes_present
                    .contains(&required),
                "missing migration class {}",
                required.as_str()
            );
        }
    }

    #[test]
    fn corpus_covers_every_accessibility_drill_class() {
        let packet = seeded_collection_truth_corpus_packet();
        for required in CollectionTruthAccessibilityDrillClass::all() {
            assert!(
                packet
                    .coverage_summary
                    .drill_classes_present
                    .contains(&required),
                "missing drill class {}",
                required.as_str()
            );
        }
    }

    #[test]
    fn support_export_declares_no_sensitive_payload() {
        let packet = seeded_collection_truth_corpus_packet();
        assert!(packet.support_export.no_sensitive_payload);
    }

    #[test]
    fn matrix_includes_every_surface_family_row() {
        let packet = seeded_collection_truth_corpus_packet();
        assert_eq!(
            packet.matrix.rows.len(),
            CollectionTruthSurfaceFamily::all().len()
        );
    }

    #[test]
    fn migration_cases_disclose_drift_unless_lossless() {
        let packet = seeded_collection_truth_corpus_packet();
        for migration in &packet.saved_view_migrations {
            if migration.case_class != SavedViewMigrationCaseClass::OlderSchemaVersionUpgradedExact
            {
                assert!(
                    migration.restored_view.drift_state
                        != SavedViewDriftState::BoundCurrentStateMatchesCaptured
                        || !migration.restored_view.stale_or_degraded_labels.is_empty(),
                    "migration {} must downgrade or disclose drift",
                    migration.migration_case_id
                );
            }
        }
    }

    #[test]
    fn report_markdown_renders_deterministically() {
        let packet = seeded_collection_truth_corpus_packet();
        let first = render_collection_truth_corpus_report_markdown(&packet);
        let second = render_collection_truth_corpus_report_markdown(&packet);
        assert_eq!(first, second);
        assert!(first.starts_with("# Collection-truth corpus report"));
    }

    #[test]
    fn drills_markdown_renders_deterministically() {
        let packet = seeded_collection_truth_corpus_packet();
        let first = render_collection_truth_corpus_drills_markdown(&packet);
        let second = render_collection_truth_corpus_drills_markdown(&packet);
        assert_eq!(first, second);
        assert!(first.starts_with("# Collection-truth drills"));
    }
}
