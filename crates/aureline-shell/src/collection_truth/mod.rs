//! Beta shell-side collection-truth primitives.
//!
//! This module owns the launch-critical shell projection for dense
//! collections: filter bars, saved views, result-scope counters, and
//! batch-review sheets across search results, problems, review queues,
//! logs, admin boards, and provider-backed tables. It composes with
//! [`aureline_search::collections`] (alpha grammar) and adds the
//! shell-facing record shapes that the M3 beta surfaces consume
//! verbatim. Surfaces never mint parallel chip, scope, count, or batch
//! vocabulary — those rows are non-conforming.
//!
//! ## Honesty contract
//!
//! 1. Every filter chip carries its [`NarrowingSourceClass`]. Hidden
//!    narrowing from policy, workset, provider, client, or partial
//!    data MUST remain inspectable in UI, accessibility narration,
//!    saved-view serialization, and support exports.
//! 2. Saved views distinguish portable state (filters, sort, group,
//!    visible columns) from refused state (transient selection,
//!    provider cursors, secret-bearing literals). Restore drift
//!    resolves to one [`SavedViewDriftState`] and one
//!    [`SavedViewFallbackBehavior`].
//! 3. Result-scope counters carry one [`ScopeCounterClass`] per row
//!    (visible, loaded, matching, total, partial, provider-owned).
//!    Collapsing two axes into one number is non-conforming.
//! 4. Batch-review sheets MUST be emitted before destructive, export-
//!    bearing, or provider-backed actions and MUST split included,
//!    excluded, blocked, and hidden counts before continuing.
//! 5. `Select all` starts at [`SelectAllEscalationClass::VisibleOrLoaded`]
//!    and only escalates to [`SelectAllEscalationClass::AllMatchingSafe`]
//!    when the surface can honestly carry that scope.

pub mod batch_review;
pub mod filter_bar;
pub mod saved_view;
pub mod scope_counter;

use serde::{Deserialize, Serialize};

pub use batch_review::{
    BatchActionConsequenceClass, BatchReviewBlockedReason, BatchReviewBlockedReasonClass,
    BatchReviewSheetRecord, BatchReviewSummary, BatchScopeAmbiguityFinding, RecoveryGuidanceClass,
    SelectAllEscalationClass, BATCH_REVIEW_SHEET_RECORD_KIND,
};
pub use filter_bar::{
    CountSummaryClass, FilterBarChipRecord, FilterBarStateRecord, NarrowingSourceClass,
    FILTER_BAR_STATE_RECORD_KIND,
};
pub use saved_view::{
    SavedCollectionViewRecord, SavedViewColumnPreset, SavedViewDriftState,
    SavedViewFallbackBehavior, SavedViewPinnedCountAxis, SavedViewScopeClass,
    SAVED_COLLECTION_VIEW_RECORD_KIND,
};
pub use scope_counter::{
    CollectionScopeCounterRecord, CollectionScopeCounterRow, ScopeCounterClass, ScopeCounterStatus,
    COLLECTION_SCOPE_COUNTER_RECORD_KIND,
};

/// Schema version exported by every collection-truth beta record.
pub const COLLECTION_TRUTH_BETA_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by fixtures, docs, and support exports.
pub const COLLECTION_TRUTH_BETA_SHARED_CONTRACT_REF: &str = "shell:collection_truth_beta:v1";

/// Stable record kind for [`CollectionTruthBetaPacket`].
pub const COLLECTION_TRUTH_BETA_PACKET_RECORD_KIND: &str =
    "shell_collection_truth_beta_packet_record";

/// Surface family that consumes the collection-truth beta record set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionTruthSurfaceFamily {
    /// Search results, problems, and result-grid surfaces.
    SearchOrResultGrid,
    /// Review inboxes, AI-evidence rollups, and batched diff queues.
    ReviewInbox,
    /// Logs, runs, and incident-timeline surfaces.
    LogOrEventCollection,
    /// Package, extension, and dependency inventories.
    PackageOrInventoryGrid,
    /// Issue boards, task lists, and triage queues.
    WorkItemBoard,
    /// Admin, settings, audit-event, and managed-workspace grids.
    AdminOrSettingsGrid,
}

impl CollectionTruthSurfaceFamily {
    /// Stable token used in fixtures, packets, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchOrResultGrid => "search_or_result_grid",
            Self::ReviewInbox => "review_inbox",
            Self::LogOrEventCollection => "log_or_event_collection",
            Self::PackageOrInventoryGrid => "package_or_inventory_grid",
            Self::WorkItemBoard => "work_item_board",
            Self::AdminOrSettingsGrid => "admin_or_settings_grid",
        }
    }

    /// Returns every surface family the beta packet must cover.
    pub const fn all() -> [Self; 6] {
        [
            Self::SearchOrResultGrid,
            Self::ReviewInbox,
            Self::LogOrEventCollection,
            Self::PackageOrInventoryGrid,
            Self::WorkItemBoard,
            Self::AdminOrSettingsGrid,
        ]
    }
}

/// One worked surface case bundling filter, view, counter, and batch records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionTruthCase {
    /// Stable case id.
    pub case_id: String,
    /// Surface family this case represents.
    pub surface_family: CollectionTruthSurfaceFamily,
    /// Short reviewable label for the case.
    pub case_label: String,
    /// Filter bar state record.
    pub filter_bar: FilterBarStateRecord,
    /// Saved view restored for this case.
    pub saved_view: SavedCollectionViewRecord,
    /// Scope counter record for this case.
    pub scope_counter: CollectionScopeCounterRecord,
    /// Batch-review sheet for the proposed consequential action.
    pub batch_review: BatchReviewSheetRecord,
}

/// Coverage summary computed from the seeded beta packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionTruthCoverageSummary {
    /// Distinct surface families covered.
    pub surface_families_present: Vec<CollectionTruthSurfaceFamily>,
    /// Distinct narrowing source classes present.
    pub narrowing_sources_present: Vec<NarrowingSourceClass>,
    /// Distinct counter classes present.
    pub counter_classes_present: Vec<ScopeCounterClass>,
    /// Distinct batch consequence classes present.
    pub batch_consequence_classes_present: Vec<BatchActionConsequenceClass>,
    /// Distinct saved-view drift states present.
    pub saved_view_drift_states_present: Vec<SavedViewDriftState>,
    /// Distinct select-all escalation classes present.
    pub select_all_escalation_classes_present: Vec<SelectAllEscalationClass>,
}

impl CollectionTruthCoverageSummary {
    fn from_cases(cases: &[CollectionTruthCase]) -> Self {
        use std::collections::BTreeSet;
        let mut surface_families: BTreeSet<CollectionTruthSurfaceFamily> = BTreeSet::new();
        let mut narrowing_sources: BTreeSet<NarrowingSourceClass> = BTreeSet::new();
        let mut counter_classes: BTreeSet<ScopeCounterClass> = BTreeSet::new();
        let mut batch_consequences: BTreeSet<BatchActionConsequenceClass> = BTreeSet::new();
        let mut drift_states: BTreeSet<SavedViewDriftState> = BTreeSet::new();
        let mut escalations: BTreeSet<SelectAllEscalationClass> = BTreeSet::new();
        for case in cases {
            surface_families.insert(case.surface_family);
            for chip in &case.filter_bar.chips {
                narrowing_sources.insert(chip.source_class);
            }
            for row in &case.scope_counter.rows {
                counter_classes.insert(row.counter_class);
            }
            batch_consequences.insert(case.batch_review.consequence_class);
            drift_states.insert(case.saved_view.drift_state);
            escalations.insert(case.batch_review.select_all_escalation_class);
        }
        Self {
            surface_families_present: surface_families.into_iter().collect(),
            narrowing_sources_present: narrowing_sources.into_iter().collect(),
            counter_classes_present: counter_classes.into_iter().collect(),
            batch_consequence_classes_present: batch_consequences.into_iter().collect(),
            saved_view_drift_states_present: drift_states.into_iter().collect(),
            select_all_escalation_classes_present: escalations.into_iter().collect(),
        }
    }
}

/// Beta packet exported by the seeded collection-truth corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionTruthBetaPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version for the packet payload.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Coverage summary computed from the cases.
    pub summary: CollectionTruthCoverageSummary,
    /// Worked surface cases.
    pub cases: Vec<CollectionTruthCase>,
}

/// Validation errors raised against the beta packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectionTruthValidationError {
    /// Packet metadata is wrong.
    PacketMetadataWrong { reason: String },
    /// Required surface family is missing from the packet.
    SurfaceFamilyMissing {
        missing: CollectionTruthSurfaceFamily,
    },
    /// One case failed an invariant check.
    CaseInvariantFailed { case_id: String, reason: String },
}

impl std::fmt::Display for CollectionTruthValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PacketMetadataWrong { reason } => {
                write!(f, "packet metadata invalid: {reason}")
            }
            Self::SurfaceFamilyMissing { missing } => {
                write!(f, "surface family missing: {}", missing.as_str())
            }
            Self::CaseInvariantFailed { case_id, reason } => {
                write!(f, "case {case_id} invariant failed: {reason}")
            }
        }
    }
}

impl std::error::Error for CollectionTruthValidationError {}

/// Validates the collection-truth beta packet.
pub fn validate_collection_truth_beta_packet(
    packet: &CollectionTruthBetaPacket,
) -> Result<(), Vec<CollectionTruthValidationError>> {
    let mut errors = Vec::new();
    if packet.record_kind != COLLECTION_TRUTH_BETA_PACKET_RECORD_KIND {
        errors.push(CollectionTruthValidationError::PacketMetadataWrong {
            reason: "record kind mismatch".to_string(),
        });
    }
    if packet.schema_version != COLLECTION_TRUTH_BETA_SCHEMA_VERSION {
        errors.push(CollectionTruthValidationError::PacketMetadataWrong {
            reason: "schema version mismatch".to_string(),
        });
    }
    if packet.shared_contract_ref != COLLECTION_TRUTH_BETA_SHARED_CONTRACT_REF {
        errors.push(CollectionTruthValidationError::PacketMetadataWrong {
            reason: "shared contract ref mismatch".to_string(),
        });
    }
    for required in CollectionTruthSurfaceFamily::all() {
        if !packet
            .cases
            .iter()
            .any(|case| case.surface_family == required)
        {
            errors.push(CollectionTruthValidationError::SurfaceFamilyMissing { missing: required });
        }
    }
    for case in &packet.cases {
        validate_case(case, &mut errors);
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_case(case: &CollectionTruthCase, errors: &mut Vec<CollectionTruthValidationError>) {
    if case.filter_bar.surface_family != case.surface_family {
        errors.push(CollectionTruthValidationError::CaseInvariantFailed {
            case_id: case.case_id.clone(),
            reason: "filter bar surface family mismatch".to_string(),
        });
    }
    if case.saved_view.surface_family != case.surface_family {
        errors.push(CollectionTruthValidationError::CaseInvariantFailed {
            case_id: case.case_id.clone(),
            reason: "saved view surface family mismatch".to_string(),
        });
    }
    if case.scope_counter.surface_family != case.surface_family {
        errors.push(CollectionTruthValidationError::CaseInvariantFailed {
            case_id: case.case_id.clone(),
            reason: "scope counter surface family mismatch".to_string(),
        });
    }
    if case.batch_review.surface_family != case.surface_family {
        errors.push(CollectionTruthValidationError::CaseInvariantFailed {
            case_id: case.case_id.clone(),
            reason: "batch review surface family mismatch".to_string(),
        });
    }
    // Hidden narrowing must be reachable via accessibility summary.
    let hidden_chip_count = case
        .filter_bar
        .chips
        .iter()
        .filter(|chip| chip.is_hidden_narrowing)
        .count();
    if hidden_chip_count > 0 && case.filter_bar.hidden_narrowing_summary.is_empty() {
        errors.push(CollectionTruthValidationError::CaseInvariantFailed {
            case_id: case.case_id.clone(),
            reason: "hidden narrowing chips lack a summary".to_string(),
        });
    }
    // Saved view drift must pair to a fallback.
    if case.saved_view.drift_state != SavedViewDriftState::BoundCurrentStateMatchesCaptured
        && matches!(
            case.saved_view.fallback_behavior,
            SavedViewFallbackBehavior::PreserveAndLabelDegraded
        )
        && case.saved_view.stale_or_degraded_labels.is_empty()
    {
        errors.push(CollectionTruthValidationError::CaseInvariantFailed {
            case_id: case.case_id.clone(),
            reason: "drifted saved view without disclosure labels".to_string(),
        });
    }
    // Counters must always cover visible, loaded, and matching axes.
    let counter_classes: std::collections::BTreeSet<_> = case
        .scope_counter
        .rows
        .iter()
        .map(|row| row.counter_class)
        .collect();
    for required in [
        ScopeCounterClass::Visible,
        ScopeCounterClass::Loaded,
        ScopeCounterClass::Matching,
    ] {
        if !counter_classes.contains(&required) {
            errors.push(CollectionTruthValidationError::CaseInvariantFailed {
                case_id: case.case_id.clone(),
                reason: format!("scope counter missing {} axis", required.as_str()),
            });
        }
    }
    // Batch review sheet must not allow ambiguous scope.
    if !case.batch_review.ambiguity_findings.is_empty() && case.batch_review.continue_enabled {
        errors.push(CollectionTruthValidationError::CaseInvariantFailed {
            case_id: case.case_id.clone(),
            reason: "ambiguous batch scope cannot enable continue".to_string(),
        });
    }
    // Destructive / export / provider-owned consequence classes require review.
    if case.batch_review.consequence_class.requires_review_sheet()
        && !case.batch_review.review_required
    {
        errors.push(CollectionTruthValidationError::CaseInvariantFailed {
            case_id: case.case_id.clone(),
            reason: "consequential action missing review sheet".to_string(),
        });
    }
}

/// Builds the seeded collection-truth beta packet used by fixtures.
pub fn seeded_collection_truth_beta_packet() -> CollectionTruthBetaPacket {
    let cases = vec![
        seeds::search_or_result_grid_case(),
        seeds::review_inbox_case(),
        seeds::log_or_event_collection_case(),
        seeds::package_or_inventory_grid_case(),
        seeds::work_item_board_case(),
        seeds::admin_or_settings_grid_case(),
    ];
    let summary = CollectionTruthCoverageSummary::from_cases(&cases);
    CollectionTruthBetaPacket {
        record_kind: COLLECTION_TRUTH_BETA_PACKET_RECORD_KIND.to_string(),
        schema_version: COLLECTION_TRUTH_BETA_SCHEMA_VERSION,
        shared_contract_ref: COLLECTION_TRUTH_BETA_SHARED_CONTRACT_REF.to_string(),
        packet_id: "shell:collection-truth:beta:packet:default".to_string(),
        generated_at: "2026-05-18T00:00:00Z".to_string(),
        summary,
        cases,
    }
}

mod seeds {
    use super::*;

    pub(super) fn search_or_result_grid_case() -> CollectionTruthCase {
        let family = CollectionTruthSurfaceFamily::SearchOrResultGrid;
        let chips = vec![
            FilterBarChipRecord::user_text("query", "uppercase reducer"),
            FilterBarChipRecord::user_facet("kind", "function"),
            FilterBarChipRecord::saved_view_pinned("workspace", "Service tier"),
            FilterBarChipRecord::policy_narrowed(
                "visibility",
                "Internal projects only",
                "policy_pack: internal_only",
            ),
            FilterBarChipRecord::provider_limit_disclosed(
                "Provider truncated at 5,000 matches",
                "provider returned approximate count",
            ),
        ];
        let filter_bar = FilterBarStateRecord::new(
            "filter-bar:search:results:default",
            family,
            "Workspace search",
            chips,
            CountSummaryClass::ApproximateProviderLimited,
            "reset filter bar to defaults",
        );
        let saved_view = SavedCollectionViewRecord::new(
            "saved-view:search:service-tier",
            family,
            "Service tier",
            SavedViewScopeClass::Workspace,
            SavedViewDriftState::ProviderStateDriftedDisclosed,
            SavedViewFallbackBehavior::PreserveAndLabelDegraded,
            vec![
                SavedViewColumnPreset::new("path", "Path", true),
                SavedViewColumnPreset::new("kind", "Kind", true),
                SavedViewColumnPreset::new("matches", "Matches", false),
            ],
            vec![("path", false)],
            vec![
                SavedViewPinnedCountAxis::Visible,
                SavedViewPinnedCountAxis::Matching,
            ],
            vec!["Provider returned approximate count".to_string()],
        );
        let scope_counter = CollectionScopeCounterRecord::new(
            "scope-counter:search:results:default",
            family,
            vec![
                CollectionScopeCounterRow::visible(120),
                CollectionScopeCounterRow::loaded(120),
                CollectionScopeCounterRow::matching_approximate(4_872),
                CollectionScopeCounterRow::total_partial(5_000),
                CollectionScopeCounterRow::provider_owned_unknown(),
            ],
            "Visible 120 of approximately 4,872 matches (provider capped at 5,000).",
        );
        let batch_review = BatchReviewSheetRecord::new(
            "batch-review:search:export-selected",
            family,
            "search.export_selected_matches",
            "Export selected matches",
            BatchActionConsequenceClass::ExportOrShare,
            SelectAllEscalationClass::VisibleOrLoaded,
            "client_local_execution",
            BatchReviewSummary {
                included_count: 120,
                excluded_count: 0,
                blocked_count: 8,
                hidden_count: 4_752,
                selected_versus_all_matching_label:
                    "Exports the 120 loaded matches only; 4,752 hidden matches stay out unless escalated.".to_string(),
            },
            vec![
                BatchReviewBlockedReason::policy_narrowed("8 internal-only rows excluded by policy"),
            ],
            RecoveryGuidanceClass::ExportRollbackByRedelivery,
            "Cancel discards the staged export.".to_string(),
        );
        CollectionTruthCase {
            case_id: "case:search:export-selected".to_string(),
            surface_family: family,
            case_label: "Search export — visible scope only".to_string(),
            filter_bar,
            saved_view,
            scope_counter,
            batch_review,
        }
    }

    pub(super) fn review_inbox_case() -> CollectionTruthCase {
        let family = CollectionTruthSurfaceFamily::ReviewInbox;
        let chips = vec![
            FilterBarChipRecord::user_facet("state", "needs review"),
            FilterBarChipRecord::user_facet("assignee", "current user"),
            FilterBarChipRecord::workset_narrowed(
                "workset",
                "release branch",
                "active workset narrows review queue",
            ),
            FilterBarChipRecord::partial_data_disclosed(
                "AI evidence still indexing",
                "evidence index is partial",
            ),
        ];
        let filter_bar = FilterBarStateRecord::new(
            "filter-bar:review:queue:default",
            family,
            "Review queue",
            chips,
            CountSummaryClass::PartialIndexing,
            "reset filter bar to defaults",
        );
        let saved_view = SavedCollectionViewRecord::new(
            "saved-view:review:my-reviews",
            family,
            "My reviews",
            SavedViewScopeClass::User,
            SavedViewDriftState::BoundCurrentStateMatchesCaptured,
            SavedViewFallbackBehavior::PreserveAndLabelDegraded,
            vec![
                SavedViewColumnPreset::new("title", "Title", true),
                SavedViewColumnPreset::new("author", "Author", true),
                SavedViewColumnPreset::new("age", "Age", false),
            ],
            vec![("age", false)],
            vec![
                SavedViewPinnedCountAxis::Visible,
                SavedViewPinnedCountAxis::Loaded,
                SavedViewPinnedCountAxis::Matching,
            ],
            Vec::new(),
        );
        let scope_counter = CollectionScopeCounterRecord::new(
            "scope-counter:review:queue:default",
            family,
            vec![
                CollectionScopeCounterRow::visible(18),
                CollectionScopeCounterRow::loaded(18),
                CollectionScopeCounterRow::matching_exact(18),
                CollectionScopeCounterRow::total_partial(42),
                CollectionScopeCounterRow::partial(2, "2 items still loading evidence"),
            ],
            "Visible 18 of 18 loaded matches; 2 items still loading AI evidence.".to_string(),
        );
        let batch_review = BatchReviewSheetRecord::new(
            "batch-review:review:approve-selected",
            family,
            "review.approve_selected",
            "Approve selected reviews",
            BatchActionConsequenceClass::RemoteMutation,
            SelectAllEscalationClass::VisibleOrLoaded,
            "mixed_client_then_provider",
            BatchReviewSummary {
                included_count: 6,
                excluded_count: 12,
                blocked_count: 0,
                hidden_count: 2,
                selected_versus_all_matching_label:
                    "Approves the 6 selected rows; 2 hidden rows remain pending evidence."
                        .to_string(),
            },
            Vec::new(),
            RecoveryGuidanceClass::CompensatingRevertWithinWindow,
            "Cancel restores selection without sending approvals.".to_string(),
        );
        CollectionTruthCase {
            case_id: "case:review:approve-selected".to_string(),
            surface_family: family,
            case_label: "Review inbox — selected approvals".to_string(),
            filter_bar,
            saved_view,
            scope_counter,
            batch_review,
        }
    }

    pub(super) fn log_or_event_collection_case() -> CollectionTruthCase {
        let family = CollectionTruthSurfaceFamily::LogOrEventCollection;
        let chips = vec![
            FilterBarChipRecord::user_text("contains", "timeout"),
            FilterBarChipRecord::user_facet("severity", "error"),
            FilterBarChipRecord::provider_limit_disclosed(
                "Provider retention 24 h",
                "older events not retained",
            ),
            FilterBarChipRecord::client_limit_disclosed(
                "Client window 10,000 lines",
                "older lines paged out",
            ),
        ];
        let filter_bar = FilterBarStateRecord::new(
            "filter-bar:logs:incident:default",
            family,
            "Incident logs",
            chips,
            CountSummaryClass::ProviderRetentionWindowed,
            "reset filter bar to defaults",
        );
        let saved_view = SavedCollectionViewRecord::new(
            "saved-view:logs:errors-last-day",
            family,
            "Errors — last 24 h",
            SavedViewScopeClass::Shared,
            SavedViewDriftState::ProviderStateDriftedDisclosed,
            SavedViewFallbackBehavior::LoadPortableSubsetWithLabels,
            vec![
                SavedViewColumnPreset::new("timestamp", "Time", true),
                SavedViewColumnPreset::new("source", "Source", true),
                SavedViewColumnPreset::new("message", "Message", false),
            ],
            vec![("timestamp", true)],
            vec![
                SavedViewPinnedCountAxis::Visible,
                SavedViewPinnedCountAxis::Loaded,
            ],
            vec![
                "Provider retained only the last 24 h of error events".to_string(),
                "Older windows reload paged out".to_string(),
            ],
        );
        let scope_counter = CollectionScopeCounterRecord::new(
            "scope-counter:logs:errors-last-day:default",
            family,
            vec![
                CollectionScopeCounterRow::visible(500),
                CollectionScopeCounterRow::loaded(10_000),
                CollectionScopeCounterRow::matching_provider_limited(10_000, 24),
                CollectionScopeCounterRow::total_unknown_due_to_retention(),
                CollectionScopeCounterRow::provider_owned_exact(10_000),
            ],
            "Loaded 10,000 of provider-retained matches in the last 24 h.".to_string(),
        );
        let batch_review = BatchReviewSheetRecord::new(
            "batch-review:logs:export-window",
            family,
            "logs.export_window",
            "Export current log window",
            BatchActionConsequenceClass::ExportOrShare,
            SelectAllEscalationClass::VisibleOrLoaded,
            "client_local_execution",
            BatchReviewSummary {
                included_count: 10_000,
                excluded_count: 0,
                blocked_count: 0,
                hidden_count: 0,
                selected_versus_all_matching_label:
                    "Exports the 10,000 loaded lines; older lines outside provider retention cannot be exported."
                        .to_string(),
            },
            Vec::new(),
            RecoveryGuidanceClass::ExportRollbackByRedelivery,
            "Cancel discards the staged log export.".to_string(),
        );
        CollectionTruthCase {
            case_id: "case:logs:export-window".to_string(),
            surface_family: family,
            case_label: "Log export — visible window".to_string(),
            filter_bar,
            saved_view,
            scope_counter,
            batch_review,
        }
    }

    pub(super) fn package_or_inventory_grid_case() -> CollectionTruthCase {
        let family = CollectionTruthSurfaceFamily::PackageOrInventoryGrid;
        let chips = vec![
            FilterBarChipRecord::user_facet("publisher", "trusted"),
            FilterBarChipRecord::user_facet("status", "outdated"),
            FilterBarChipRecord::policy_narrowed(
                "marketplace",
                "Org allowlist",
                "org policy gates marketplace",
            ),
        ];
        let filter_bar = FilterBarStateRecord::new(
            "filter-bar:packages:inventory:default",
            family,
            "Extension inventory",
            chips,
            CountSummaryClass::ExactLocal,
            "reset filter bar to defaults",
        );
        let saved_view = SavedCollectionViewRecord::new(
            "saved-view:packages:outdated",
            family,
            "Outdated extensions",
            SavedViewScopeClass::Workspace,
            SavedViewDriftState::BoundCurrentStateMatchesCaptured,
            SavedViewFallbackBehavior::PreserveAndLabelDegraded,
            vec![
                SavedViewColumnPreset::new("name", "Name", true),
                SavedViewColumnPreset::new("publisher", "Publisher", true),
                SavedViewColumnPreset::new("version", "Version", false),
            ],
            vec![("name", false)],
            vec![
                SavedViewPinnedCountAxis::Visible,
                SavedViewPinnedCountAxis::Matching,
                SavedViewPinnedCountAxis::Loaded,
            ],
            Vec::new(),
        );
        let scope_counter = CollectionScopeCounterRecord::new(
            "scope-counter:packages:inventory:default",
            family,
            vec![
                CollectionScopeCounterRow::visible(14),
                CollectionScopeCounterRow::loaded(14),
                CollectionScopeCounterRow::matching_exact(14),
                CollectionScopeCounterRow::total_exact(86),
            ],
            "14 of 86 inventory rows match outdated filter.".to_string(),
        );
        let batch_review = BatchReviewSheetRecord::new(
            "batch-review:packages:uninstall-selected",
            family,
            "packages.uninstall_selected",
            "Uninstall selected extensions",
            BatchActionConsequenceClass::DestructiveLocal,
            SelectAllEscalationClass::AllMatchingSafe,
            "client_local_execution",
            BatchReviewSummary {
                included_count: 9,
                excluded_count: 0,
                blocked_count: 5,
                hidden_count: 0,
                selected_versus_all_matching_label:
                    "Uninstalls 9 user-installed extensions; 5 admin-pinned extensions blocked."
                        .to_string(),
            },
            vec![BatchReviewBlockedReason::policy_narrowed(
                "5 admin-pinned extensions blocked by org policy",
            )],
            RecoveryGuidanceClass::ReversibleViaUndoStack,
            "Cancel restores selection without uninstalling.".to_string(),
        );
        CollectionTruthCase {
            case_id: "case:packages:uninstall-selected".to_string(),
            surface_family: family,
            case_label: "Inventory uninstall — admin pinned blocked".to_string(),
            filter_bar,
            saved_view,
            scope_counter,
            batch_review,
        }
    }

    pub(super) fn work_item_board_case() -> CollectionTruthCase {
        let family = CollectionTruthSurfaceFamily::WorkItemBoard;
        let chips = vec![
            FilterBarChipRecord::user_facet("milestone", "release train"),
            FilterBarChipRecord::user_facet("assignee", "current user"),
            FilterBarChipRecord::workset_narrowed(
                "workset",
                "current sprint",
                "current workset narrows board",
            ),
        ];
        let filter_bar = FilterBarStateRecord::new(
            "filter-bar:work-items:my-board:default",
            family,
            "Work items",
            chips,
            CountSummaryClass::ExactWithWorksetNarrowing,
            "reset filter bar to defaults",
        );
        let saved_view = SavedCollectionViewRecord::new(
            "saved-view:work-items:my-board",
            family,
            "My board",
            SavedViewScopeClass::Shared,
            SavedViewDriftState::ColumnSetDriftedDisclosed,
            SavedViewFallbackBehavior::LoadPortableSubsetWithLabels,
            vec![
                SavedViewColumnPreset::new("title", "Title", true),
                SavedViewColumnPreset::new("state", "State", true),
                SavedViewColumnPreset::new("priority", "Priority", false),
            ],
            vec![("priority", true)],
            vec![
                SavedViewPinnedCountAxis::Visible,
                SavedViewPinnedCountAxis::Matching,
            ],
            vec!["One captured column removed by provider".to_string()],
        );
        let scope_counter = CollectionScopeCounterRecord::new(
            "scope-counter:work-items:my-board:default",
            family,
            vec![
                CollectionScopeCounterRow::visible(11),
                CollectionScopeCounterRow::loaded(11),
                CollectionScopeCounterRow::matching_exact(11),
                CollectionScopeCounterRow::total_exact(11),
            ],
            "11 work items match the current workset and assignee filter.".to_string(),
        );
        let batch_review = BatchReviewSheetRecord::new(
            "batch-review:work-items:move-selected",
            family,
            "work_items.move_selected",
            "Move selected work items",
            BatchActionConsequenceClass::RemoteMutation,
            SelectAllEscalationClass::AllMatchingSafe,
            "provider_authoritative_execution",
            BatchReviewSummary {
                included_count: 11,
                excluded_count: 0,
                blocked_count: 0,
                hidden_count: 0,
                selected_versus_all_matching_label:
                    "Moves all 11 matching work items to the next milestone.".to_string(),
            },
            Vec::new(),
            RecoveryGuidanceClass::CompensatingRevertWithinWindow,
            "Cancel restores the prior assignment without sending a move request.".to_string(),
        );
        CollectionTruthCase {
            case_id: "case:work-items:move-selected".to_string(),
            surface_family: family,
            case_label: "Work items — all matching move".to_string(),
            filter_bar,
            saved_view,
            scope_counter,
            batch_review,
        }
    }

    pub(super) fn admin_or_settings_grid_case() -> CollectionTruthCase {
        let family = CollectionTruthSurfaceFamily::AdminOrSettingsGrid;
        let chips = vec![
            FilterBarChipRecord::user_facet("role", "admin"),
            FilterBarChipRecord::policy_narrowed(
                "tenant",
                "Tenant A",
                "policy pinned tenant scope",
            ),
        ];
        let filter_bar = FilterBarStateRecord::new(
            "filter-bar:admin:identities:default",
            family,
            "Admin identities",
            chips,
            CountSummaryClass::ExactWithPolicyPinning,
            "reset filter bar to defaults",
        );
        let saved_view = SavedCollectionViewRecord::new(
            "saved-view:admin:identities:default",
            family,
            "Tenant admins",
            SavedViewScopeClass::PolicyPinned,
            SavedViewDriftState::PolicyNarrowingChangedDisclosed,
            SavedViewFallbackBehavior::RefuseUntilRebound,
            vec![
                SavedViewColumnPreset::new("name", "Name", true),
                SavedViewColumnPreset::new("role", "Role", true),
                SavedViewColumnPreset::new("last_active", "Last active", false),
            ],
            vec![("name", false)],
            vec![
                SavedViewPinnedCountAxis::Visible,
                SavedViewPinnedCountAxis::Matching,
            ],
            vec!["Policy narrowing changed since this view was captured".to_string()],
        );
        let scope_counter = CollectionScopeCounterRecord::new(
            "scope-counter:admin:identities:default",
            family,
            vec![
                CollectionScopeCounterRow::visible(4),
                CollectionScopeCounterRow::loaded(4),
                CollectionScopeCounterRow::matching_exact(4),
                CollectionScopeCounterRow::total_exact(4),
            ],
            "4 admin identities match the policy-pinned tenant scope.".to_string(),
        );
        let batch_review = BatchReviewSheetRecord::new(
            "batch-review:admin:rotate-keys",
            family,
            "admin.rotate_keys",
            "Rotate signing keys for selected admins",
            BatchActionConsequenceClass::DestructiveRemote,
            SelectAllEscalationClass::AllMatchingSafe,
            "provider_authoritative_execution",
            BatchReviewSummary {
                included_count: 2,
                excluded_count: 2,
                blocked_count: 0,
                hidden_count: 0,
                selected_versus_all_matching_label:
                    "Rotates keys for the 2 selected admins; 2 other admins excluded by policy hold."
                        .to_string(),
            },
            Vec::new(),
            RecoveryGuidanceClass::CompensatingRevertWithinWindow,
            "Cancel keeps existing keys; rotation can be re-issued from admin tools.".to_string(),
        );
        CollectionTruthCase {
            case_id: "case:admin:rotate-keys".to_string(),
            surface_family: family,
            case_label: "Admin grid — rotate selected keys".to_string(),
            filter_bar,
            saved_view,
            scope_counter,
            batch_review,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_packet_validates() {
        let packet = seeded_collection_truth_beta_packet();
        validate_collection_truth_beta_packet(&packet).expect("seeded packet must validate");
    }

    #[test]
    fn seeded_packet_covers_every_surface_family() {
        let packet = seeded_collection_truth_beta_packet();
        for required in CollectionTruthSurfaceFamily::all() {
            assert!(
                packet
                    .summary
                    .surface_families_present
                    .iter()
                    .any(|family| *family == required),
                "missing surface family {}",
                required.as_str()
            );
        }
    }

    #[test]
    fn seeded_packet_exercises_provider_limited_counts() {
        let packet = seeded_collection_truth_beta_packet();
        assert!(packet
            .summary
            .counter_classes_present
            .contains(&ScopeCounterClass::ProviderOwned));
    }

    #[test]
    fn seeded_packet_exercises_all_select_all_escalations() {
        let packet = seeded_collection_truth_beta_packet();
        assert!(packet
            .summary
            .select_all_escalation_classes_present
            .contains(&SelectAllEscalationClass::VisibleOrLoaded));
        assert!(packet
            .summary
            .select_all_escalation_classes_present
            .contains(&SelectAllEscalationClass::AllMatchingSafe));
    }
}
