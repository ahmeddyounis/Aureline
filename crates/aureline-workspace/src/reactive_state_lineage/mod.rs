//! Reactive-state and materialized-view lineage: the governed, export-safe
//! projection that hardens reactive-state and materialized-view invalidation
//! with truthful stale-view downgrade behavior.
//!
//! The projection ingests a live [`ReactiveStateInputs`] envelope verbatim
//! (one [`MaterializedViewObservation`] per reactive view tracked by the
//! workspace plus the controlled inspection-hook table) and produces a
//! stable-line lineage record that proves the seven claims the
//! reactive-state lane is anchored on:
//!
//! - **View-class coverage truth.** Every materialized view declares one
//!   closed [`MaterializedViewClass`] (`ephemeral_projection`,
//!   `durable_local_materialization`, `exportable_snapshot`,
//!   `managed_replicated_view`), and the corpus seeds at least one row per
//!   class so the export, support, and replication paths are observable.
//! - **Stale-view downgrade truth.** Every non-aligned parity state carries
//!   a non-`none` [`ReactiveDowngradeLabel`] drawn from the closed
//!   vocabulary; aligned views carry `none`. The projection re-derives
//!   parity from subscriber observations rather than trusting the prose
//!   field, so a record cannot claim Aligned while a subscriber lags.
//! - **Epoch-parity honesty.** Every required consumer surface (`shell`,
//!   `search`, `graph`, `ai`, `review`, `support`) has one subscriber
//!   epoch row, no subscriber's observed epoch exceeds the authority
//!   epoch, and the parity state matches the observed subscribers
//!   (aligned requires all-authoritative-at-authority-epoch; drift
//!   requires at least one subscriber lagging the authority epoch;
//!   awaiting_resync requires a resync-required / stale / warming signal;
//!   terminal_unavailable requires at least one unavailable subscriber).
//! - **Open-gap honesty.** Downgraded views must record at least one
//!   [`OpenGapClass`] other than `none`; aligned views must declare none.
//! - **Support-export honesty.** Each view's support-export projection
//!   preserves epoch state (`view_class`, `authority_label`,
//!   `authority_epoch`, `subscriber_epochs`), excludes raw private
//!   material and ambient authority, and preserves user-authored files.
//!   `exportable_snapshot` and `managed_replicated_view` views must
//!   declare a non-`local_only` posture so support bundles can preserve
//!   the epoch state.
//! - **No-rerun honesty under stale views.** A controlled set of
//!   pre-destructive inspection / repair hooks (`inspect_drift`,
//!   `compare_epochs`, `resync_review`, `rollback_checkpoint`, `export`,
//!   `repair`) is reachable so any destructive resync is reviewable
//!   before it fires.
//! - **Lineage and export honesty.** The record sets
//!   `raw_payload_excluded = true` and carries only opaque refs to the
//!   source views, the workspace, the producer, and the corpus.
//!
//! In addition the record carries the producer attribution (producer ref,
//! schema version, integrity hash) so replay and support pipelines can
//! pin the source before applying. When the projection cannot prove a
//! claim on the captured posture it auto-narrows below Stable with a
//! named [`ReactiveStateLineageNarrowReason`].

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Schema version for [`ReactiveStateLineageRecord`].
pub const REACTIVE_STATE_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the reactive-state lineage record.
pub const REACTIVE_STATE_LINEAGE_SCHEMA_REF: &str =
    "schemas/workspace/reactive_state_lineage.schema.json";

/// Stable record-kind tag for the reactive-state lineage record.
pub const REACTIVE_STATE_LINEAGE_RECORD_KIND: &str = "reactive_state_lineage_record";

// ---------------------------------------------------------------------------
// Closed vocabularies.
// ---------------------------------------------------------------------------

/// Closed materialized-view-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaterializedViewClass {
    /// In-memory projection rebuilt on every cold start.
    EphemeralProjection,
    /// Persisted on the local device; survives restarts but is not
    /// exported by default.
    DurableLocalMaterialization,
    /// Persisted locally and bundled into support / export packets.
    ExportableSnapshot,
    /// Replicated by a managed service; the local copy may lag the
    /// remote authority epoch.
    ManagedReplicatedView,
}

impl MaterializedViewClass {
    /// Returns the stable snake_case token for this view class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EphemeralProjection => "ephemeral_projection",
            Self::DurableLocalMaterialization => "durable_local_materialization",
            Self::ExportableSnapshot => "exportable_snapshot",
            Self::ManagedReplicatedView => "managed_replicated_view",
        }
    }
}

/// Closed list of materialized-view classes the corpus must cover.
pub const REQUIRED_VIEW_CLASSES: [MaterializedViewClass; 4] = [
    MaterializedViewClass::EphemeralProjection,
    MaterializedViewClass::DurableLocalMaterialization,
    MaterializedViewClass::ExportableSnapshot,
    MaterializedViewClass::ManagedReplicatedView,
];

/// Closed authority-label vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityLabel {
    WorkspaceVfs,
    BufferEditor,
    DerivedKnowledge,
    Execution,
    PolicyEntitlement,
    ProviderOverlay,
}

impl AuthorityLabel {
    /// Returns the stable snake_case token for this authority label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceVfs => "workspace_vfs",
            Self::BufferEditor => "buffer_editor",
            Self::DerivedKnowledge => "derived_knowledge",
            Self::Execution => "execution",
            Self::PolicyEntitlement => "policy_entitlement",
            Self::ProviderOverlay => "provider_overlay",
        }
    }
}

/// Closed consumer-surface vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurfaceKind {
    Shell,
    Search,
    Graph,
    Ai,
    Review,
    Support,
}

impl ConsumerSurfaceKind {
    /// Returns the stable snake_case token for this surface kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::Search => "search",
            Self::Graph => "graph",
            Self::Ai => "ai",
            Self::Review => "review",
            Self::Support => "support",
        }
    }
}

/// Closed list of consumer surfaces every materialized view must wire.
pub const REQUIRED_CONSUMER_SURFACES: [ConsumerSurfaceKind; 6] = [
    ConsumerSurfaceKind::Shell,
    ConsumerSurfaceKind::Search,
    ConsumerSurfaceKind::Graph,
    ConsumerSurfaceKind::Ai,
    ConsumerSurfaceKind::Review,
    ConsumerSurfaceKind::Support,
];

/// Closed freshness label for a subscriber observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriberFreshness {
    /// Subscriber observed the authoritative frame at the current
    /// authority epoch.
    Authoritative,
    /// Subscriber has accepted an earlier frame and is awaiting refresh.
    Cached,
    /// Subscriber knows it is behind the authority epoch.
    Stale,
    /// Subscriber projected an imported / replayed bundle that is not
    /// authoritative for the live workspace.
    Imported,
    /// Subscriber has warmed up but never observed an authoritative frame.
    Warming,
    /// Subscriber cannot serve the projection right now.
    Unavailable,
}

impl SubscriberFreshness {
    /// Returns the stable snake_case token for this freshness label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::Imported => "imported",
            Self::Warming => "warming",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Closed invalidation-cause vocabulary recording why a subscriber
/// observed its latest frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvalidationCauseClass {
    AuthorityWrite,
    DerivedRecompute,
    PolicyChange,
    ProviderOverlayChange,
    ExternalChange,
    ImportedBundleSwap,
    ResyncRequired,
}

impl InvalidationCauseClass {
    /// Returns the stable snake_case token for this cause class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthorityWrite => "authority_write",
            Self::DerivedRecompute => "derived_recompute",
            Self::PolicyChange => "policy_change",
            Self::ProviderOverlayChange => "provider_overlay_change",
            Self::ExternalChange => "external_change",
            Self::ImportedBundleSwap => "imported_bundle_swap",
            Self::ResyncRequired => "resync_required",
        }
    }
}

/// Closed epoch-parity-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EpochParityState {
    /// All required subscribers observe the current authority epoch
    /// authoritatively.
    Aligned,
    /// At least one subscriber lags the authority epoch or holds a
    /// non-authoritative frame.
    DriftDetected,
    /// At least one subscriber is waiting for a resync after an
    /// invalidation it could not absorb in place.
    AwaitingResync,
    /// At least one subscriber cannot serve the projection right now.
    TerminalUnavailable,
}

impl EpochParityState {
    /// Returns the stable snake_case token for this parity state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Aligned => "aligned",
            Self::DriftDetected => "drift_detected",
            Self::AwaitingResync => "awaiting_resync",
            Self::TerminalUnavailable => "terminal_unavailable",
        }
    }

    /// Returns true when the parity state is aligned.
    pub const fn is_aligned(self) -> bool {
        matches!(self, Self::Aligned)
    }
}

/// Closed support-export-posture vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportExportPosture {
    LocalOnly,
    MetadataSafeExport,
    HeldRecord,
}

impl SupportExportPosture {
    /// Returns the stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::MetadataSafeExport => "metadata_safe_export",
            Self::HeldRecord => "held_record",
        }
    }
}

/// Closed downgrade-label vocabulary for reactive-state stale views.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReactiveDowngradeLabel {
    /// No downgrade applied; the row passes outright.
    None,
    /// Red — the stable row is blocked until drift is resolved.
    RedBlocksStableRow,
    /// Yellow — at least one surface lags but the safety invariants
    /// still hold.
    YellowSurfacePartial,
    /// Yellow — authority skew detected; the surfaces project a
    /// consistent older frame.
    YellowAuthoritySkew,
    /// View degrades to the authority-only path until replication
    /// parity ships.
    DegradedToAuthorityOnly,
    /// The protected corpus is stale; the release candidate cannot
    /// promote until it is restored.
    StaleCorpusBlocksReleaseCandidate,
}

impl ReactiveDowngradeLabel {
    /// Returns the stable snake_case token for this downgrade label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::RedBlocksStableRow => "red_blocks_stable_row",
            Self::YellowSurfacePartial => "yellow_surface_partial",
            Self::YellowAuthoritySkew => "yellow_authority_skew",
            Self::DegradedToAuthorityOnly => "degraded_to_authority_only",
            Self::StaleCorpusBlocksReleaseCandidate => "stale_corpus_blocks_release_candidate",
        }
    }

    /// Returns true when the label is `none` (no downgrade applied).
    pub const fn is_healthy(self) -> bool {
        matches!(self, Self::None)
    }
}

/// Closed open-gap class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenGapClass {
    None,
    SubscriberPending,
    ReplicationPending,
    SupportExportPending,
    DriftRecoveryManual,
}

impl OpenGapClass {
    /// Returns the stable snake_case token for this gap class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::SubscriberPending => "subscriber_pending",
            Self::ReplicationPending => "replication_pending",
            Self::SupportExportPending => "support_export_pending",
            Self::DriftRecoveryManual => "drift_recovery_manual",
        }
    }
}

// ---------------------------------------------------------------------------
// Inspection hooks.
// ---------------------------------------------------------------------------

/// Class of pre-destructive inspection / repair hook available before a
/// reactive view is resynced, exported, or repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReactiveStateInspectionHookClass {
    /// Open the drift inspector for the reactive view.
    InspectDrift,
    /// Compare authority and subscriber epochs side by side.
    CompareEpochs,
    /// Open the resync review sheet before any destructive resync.
    ResyncReview,
    /// Capture a one-step rollback checkpoint before resyncing.
    RollbackCheckpoint,
    /// Export the lineage record (support-safe, no raw payload bytes).
    Export,
    /// Open the repair sheet for a stuck or unavailable view.
    Repair,
}

impl ReactiveStateInspectionHookClass {
    /// Returns the stable snake_case token for this hook class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectDrift => "inspect_drift",
            Self::CompareEpochs => "compare_epochs",
            Self::ResyncReview => "resync_review",
            Self::RollbackCheckpoint => "rollback_checkpoint",
            Self::Export => "export",
            Self::Repair => "repair",
        }
    }
}

/// One pre-destructive inspection / repair hook offered before the
/// reactive view commits to a destructive action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveStateInspectionHook {
    /// Hook class.
    pub hook_class: ReactiveStateInspectionHookClass,
    /// Stable action id.
    pub action_id: String,
    /// UI label.
    pub label: String,
    /// Whether the hook is reachable for this posture.
    pub available: bool,
    /// Disclosure shown when the hook is offered.
    pub disclosure: String,
}

/// Returns the default pre-destructive inspection / repair hook table.
pub fn default_reactive_state_inspection_hooks() -> Vec<ReactiveStateInspectionHook> {
    vec![
        ReactiveStateInspectionHook {
            hook_class: ReactiveStateInspectionHookClass::InspectDrift,
            action_id: "reactive_state.inspect_drift".to_owned(),
            label: "Inspect drift".to_owned(),
            available: true,
            disclosure:
                "Opens the drift inspector with each subscriber surface, its observed epoch, freshness, and last invalidation cause."
                    .to_owned(),
        },
        ReactiveStateInspectionHook {
            hook_class: ReactiveStateInspectionHookClass::CompareEpochs,
            action_id: "reactive_state.compare_epochs".to_owned(),
            label: "Compare authority and subscriber epochs".to_owned(),
            available: true,
            disclosure:
                "Produces a reviewable diff between the current authority epoch and each subscriber's observed epoch before any resync."
                    .to_owned(),
        },
        ReactiveStateInspectionHook {
            hook_class: ReactiveStateInspectionHookClass::ResyncReview,
            action_id: "reactive_state.resync_review".to_owned(),
            label: "Review resync plan".to_owned(),
            available: true,
            disclosure:
                "Opens the resync review sheet so any destructive resync can be inspected before it fires."
                    .to_owned(),
        },
        ReactiveStateInspectionHook {
            hook_class: ReactiveStateInspectionHookClass::RollbackCheckpoint,
            action_id: "reactive_state.rollback_checkpoint".to_owned(),
            label: "Create rollback checkpoint".to_owned(),
            available: true,
            disclosure:
                "Captures a one-step rollback checkpoint before resyncing the materialized view."
                    .to_owned(),
        },
        ReactiveStateInspectionHook {
            hook_class: ReactiveStateInspectionHookClass::Export,
            action_id: "reactive_state.export".to_owned(),
            label: "Export reactive-state lineage".to_owned(),
            available: true,
            disclosure:
                "Exports this reactive-state lineage record for support without raw payload bytes."
                    .to_owned(),
        },
        ReactiveStateInspectionHook {
            hook_class: ReactiveStateInspectionHookClass::Repair,
            action_id: "reactive_state.repair".to_owned(),
            label: "Open repair sheet".to_owned(),
            available: true,
            disclosure:
                "Opens the repair sheet for a stuck or unavailable view; surfaces the manual remediation steps."
                    .to_owned(),
        },
    ]
}

// ---------------------------------------------------------------------------
// Input envelope.
// ---------------------------------------------------------------------------

/// One subscriber-surface observation row carried by a materialized view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriberEpochObservation {
    pub surface_kind: ConsumerSurfaceKind,
    pub observed_epoch: u64,
    pub observed_freshness: SubscriberFreshness,
    pub last_invalidation_cause: InvalidationCauseClass,
    pub observed_at: String,
}

/// Metadata-safe support-export projection input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportInputs {
    pub posture: SupportExportPosture,
    pub includes_view_class: bool,
    pub includes_authority_label: bool,
    pub includes_authority_epoch: bool,
    pub includes_subscriber_epochs: bool,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub preserves_user_authored_files: bool,
}

impl SupportExportInputs {
    /// Returns the metadata-safe baseline for a given posture.
    pub const fn metadata_safe_baseline(posture: SupportExportPosture) -> Self {
        Self {
            posture,
            includes_view_class: true,
            includes_authority_label: true,
            includes_authority_epoch: true,
            includes_subscriber_epochs: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            preserves_user_authored_files: true,
        }
    }
}

/// One open-gap row attached to a materialized view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenGapEntry {
    pub gap_class: OpenGapClass,
    pub summary: String,
}

/// One observation of a materialized view at a captured moment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializedViewObservation {
    pub view_id: String,
    pub title: String,
    pub view_class: MaterializedViewClass,
    pub authority_label: AuthorityLabel,
    pub authority_epoch: u64,
    pub subscriber_epochs: Vec<SubscriberEpochObservation>,
    pub declared_parity_state: EpochParityState,
    pub support_export: SupportExportInputs,
    pub declared_downgrade_label: ReactiveDowngradeLabel,
    #[serde(default)]
    pub open_gaps: Vec<OpenGapEntry>,
    pub captured_at: String,
}

/// Input envelope ingested by the projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveStateInputs {
    pub workspace_ref: String,
    pub producer_ref: String,
    pub corpus_ref: String,
    pub captured_at: String,
    pub views: Vec<MaterializedViewObservation>,
}

// ---------------------------------------------------------------------------
// Narrow reasons + qualification.
// ---------------------------------------------------------------------------

/// Named reason a reactive-state lineage record narrows below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReactiveStateLineageNarrowReason {
    /// The captured input had no materialized views.
    CorpusEmpty,
    /// A required materialized-view class is missing from the corpus.
    RequiredViewClassMissing,
    /// A required consumer surface is missing on at least one view.
    RequiredConsumerSurfaceMissing,
    /// A subscriber observed_epoch exceeds the authority_epoch.
    SubscriberEpochExceedsAuthority,
    /// A view declared `aligned` but at least one subscriber lags or is
    /// non-authoritative.
    AlignedParityNotProven,
    /// A view declared `drift_detected` but no subscriber lags the
    /// authority epoch.
    DriftWithoutEpochLag,
    /// A view declared `awaiting_resync` but no subscriber carries a
    /// resync-required / stale / warming signal.
    AwaitingResyncWithoutSignal,
    /// A view declared `terminal_unavailable` but no subscriber is
    /// unavailable.
    TerminalUnavailableWithoutSignal,
    /// A non-aligned parity state declared no downgrade.
    NonAlignedMissingDowngrade,
    /// An aligned parity state declared a non-`none` downgrade.
    AlignedCarriesDowngrade,
    /// A downgraded view recorded no open-gap row.
    DowngradedWithoutOpenGap,
    /// An aligned view declared a non-`none` open-gap row.
    AlignedCarriesOpenGap,
    /// A support-export projection drops a required epoch field.
    SupportExportEpochFieldsDropped,
    /// An exportable / replicated view declared a `local_only` posture.
    SupportExportPostureUnsafe,
    /// Raw private material or ambient authority slipped into the
    /// support-export projection.
    SupportExportRedactionUnsafe,
    /// A required pre-destructive inspection hook is unavailable.
    InspectionHookUnavailable,
    /// Producer attribution (producer ref / workspace ref / corpus ref)
    /// is incomplete.
    ProducerAttributionIncomplete,
    /// Workspace ref is empty (would break support export).
    LineageExportUnsafe,
}

impl ReactiveStateLineageNarrowReason {
    /// Returns the stable snake_case token for this narrow reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CorpusEmpty => "corpus_empty",
            Self::RequiredViewClassMissing => "required_view_class_missing",
            Self::RequiredConsumerSurfaceMissing => "required_consumer_surface_missing",
            Self::SubscriberEpochExceedsAuthority => "subscriber_epoch_exceeds_authority",
            Self::AlignedParityNotProven => "aligned_parity_not_proven",
            Self::DriftWithoutEpochLag => "drift_without_epoch_lag",
            Self::AwaitingResyncWithoutSignal => "awaiting_resync_without_signal",
            Self::TerminalUnavailableWithoutSignal => "terminal_unavailable_without_signal",
            Self::NonAlignedMissingDowngrade => "non_aligned_missing_downgrade",
            Self::AlignedCarriesDowngrade => "aligned_carries_downgrade",
            Self::DowngradedWithoutOpenGap => "downgraded_without_open_gap",
            Self::AlignedCarriesOpenGap => "aligned_carries_open_gap",
            Self::SupportExportEpochFieldsDropped => "support_export_epoch_fields_dropped",
            Self::SupportExportPostureUnsafe => "support_export_posture_unsafe",
            Self::SupportExportRedactionUnsafe => "support_export_redaction_unsafe",
            Self::InspectionHookUnavailable => "inspection_hook_unavailable",
            Self::ProducerAttributionIncomplete => "producer_attribution_incomplete",
            Self::LineageExportUnsafe => "lineage_export_unsafe",
        }
    }
}

/// Stable-qualification posture for a reactive-state lineage record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveStateLineageQualification {
    /// Whether the record proves the contract on the claimed posture.
    pub qualified: bool,
    /// Stable lifecycle label: `stable` or `narrowed_below_stable`.
    pub level: String,
    /// Named reasons the record narrowed below Stable, when not qualified.
    pub narrow_reasons: Vec<ReactiveStateLineageNarrowReason>,
}

// ---------------------------------------------------------------------------
// Pillar projections.
// ---------------------------------------------------------------------------

/// One view-class row carried in the lineage projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveViewLineageRow {
    /// Stable view id.
    pub view_id: String,
    /// View title.
    pub title: String,
    /// Materialized-view class.
    pub view_class: MaterializedViewClass,
    /// Authority label.
    pub authority_label: AuthorityLabel,
    /// Authority epoch the view claims.
    pub authority_epoch: u64,
    /// Re-derived parity state.
    pub derived_parity_state: EpochParityState,
    /// Parity state declared by the input (may diverge if the input is
    /// dishonest; the projection narrows in that case).
    pub declared_parity_state: EpochParityState,
    /// True when the declared parity state matches the re-derived parity
    /// state.
    pub parity_state_matches: bool,
    /// Minimum subscriber observed epoch.
    pub min_subscriber_epoch: u64,
    /// Maximum subscriber observed epoch.
    pub max_subscriber_epoch: u64,
    /// Number of subscribers recorded.
    pub subscriber_count: usize,
    /// Number of subscribers lagging the authority epoch.
    pub subscribers_lagging_count: usize,
    /// True when at least one subscriber is unavailable.
    pub any_unavailable: bool,
    /// True when at least one subscriber carries the resync-required
    /// signal.
    pub any_resync_required: bool,
    /// Downgrade label declared by the input.
    pub downgrade_label: ReactiveDowngradeLabel,
    /// Open-gap classes recorded on this view.
    pub open_gap_classes: Vec<OpenGapClass>,
    /// Support-export posture for this view.
    pub support_export_posture: SupportExportPosture,
}

/// View-class coverage posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewClassCoverageSummary {
    /// All materialized-view rows carried by the corpus.
    pub view_rows: Vec<ReactiveViewLineageRow>,
    /// True when every required materialized-view class is present.
    pub all_required_view_classes_present: bool,
    /// True when every view declares an entry for every required
    /// consumer surface.
    pub all_required_consumer_surfaces_present: bool,
    /// True when no subscriber observed epoch exceeds the authority
    /// epoch on any view.
    pub no_subscriber_epoch_exceeds_authority: bool,
}

/// Stale-view downgrade truth posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaleViewDowngradeSummary {
    /// Number of aligned views.
    pub aligned_count: usize,
    /// Number of drift-detected views.
    pub drift_detected_count: usize,
    /// Number of awaiting-resync views.
    pub awaiting_resync_count: usize,
    /// Number of terminal-unavailable views.
    pub terminal_unavailable_count: usize,
    /// True when every non-aligned view declares a non-`none` downgrade
    /// label.
    pub all_non_aligned_views_carry_downgrade: bool,
    /// True when every aligned view declares the `none` downgrade label.
    pub all_aligned_views_carry_none: bool,
    /// True when every downgraded view records at least one open-gap row.
    pub all_downgraded_views_record_open_gap: bool,
    /// True when every aligned view records no open-gap row.
    pub all_aligned_views_record_no_open_gap: bool,
}

/// Epoch-parity honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochParityHonestySummary {
    /// True when every view's declared parity state matches the
    /// projection's re-derived parity state.
    pub all_views_declared_parity_matches_observation: bool,
    /// True when every drift-detected view has at least one subscriber
    /// lagging the authority epoch.
    pub all_drift_views_have_epoch_lag: bool,
    /// True when every awaiting-resync view has a resync-required /
    /// stale / warming signal.
    pub all_awaiting_resync_views_have_signal: bool,
    /// True when every terminal-unavailable view has at least one
    /// unavailable subscriber.
    pub all_terminal_unavailable_views_have_signal: bool,
}

/// Support-export honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveSupportExportSummary {
    /// True when every view's support-export projection preserves
    /// view_class, authority_label, authority_epoch, and subscriber
    /// epochs.
    pub all_views_preserve_epoch_state: bool,
    /// True when every view declares
    /// `raw_private_material_excluded = true`.
    pub all_views_redact_raw_private_material: bool,
    /// True when every view declares `ambient_authority_excluded = true`.
    pub all_views_exclude_ambient_authority: bool,
    /// True when every view declares
    /// `preserves_user_authored_files = true`.
    pub all_views_preserve_user_authored_files: bool,
    /// True when every exportable / replicated view declares a
    /// non-`local_only` posture.
    pub all_exportable_or_replicated_views_have_safe_posture: bool,
}

/// Producer-attribution posture for replay safety.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveProducerAttributionSummary {
    /// Opaque producer build / instance ref.
    pub producer_ref: String,
    /// Schema version pinned by the input.
    pub schema_version: u32,
    /// Opaque integrity hash derived from the input view identities.
    pub integrity_hash: String,
    /// Input capture timestamp.
    pub captured_at: String,
    /// True when producer attribution fields are non-empty.
    pub producer_attribution_complete: bool,
}

// ---------------------------------------------------------------------------
// Top-level record.
// ---------------------------------------------------------------------------

/// Governed, export-safe reactive-state lineage record per posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveStateLineageRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub reactive_state_lineage_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable posture id.
    pub posture_id: String,
    /// Workspace ref the corpus describes.
    pub workspace_ref: String,
    /// Opaque ref to the corpus the projection ingested.
    pub corpus_ref: String,
    /// Producer attribution pillar.
    pub producer_attribution: ReactiveProducerAttributionSummary,
    /// View-class coverage pillar.
    pub view_class_coverage: ViewClassCoverageSummary,
    /// Stale-view downgrade truth pillar.
    pub stale_view_downgrade: StaleViewDowngradeSummary,
    /// Epoch-parity honesty pillar.
    pub epoch_parity_honesty: EpochParityHonestySummary,
    /// Support-export honesty pillar.
    pub support_export_honesty: ReactiveSupportExportSummary,
    /// Pre-destructive inspection / repair hooks.
    pub inspection_hooks: Vec<ReactiveStateInspectionHook>,
    /// Stable-qualification posture with named narrow reasons.
    pub stable_qualification: ReactiveStateLineageQualification,
    /// Whether the record is metadata-safe for support export.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl ReactiveStateLineageRecord {
    /// Returns true when the record is metadata-safe for support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == REACTIVE_STATE_LINEAGE_SCHEMA_REF
            && self.record_kind == REACTIVE_STATE_LINEAGE_RECORD_KIND
            && !self.workspace_ref.trim().is_empty()
            && !self.corpus_ref.trim().is_empty()
    }

    /// Returns true when the record proves the contract on the claimed
    /// posture.
    pub fn is_stable_qualified(&self) -> bool {
        self.stable_qualification.qualified
    }

    /// Returns the inspection hook of the given class, when present.
    pub fn inspection_hook(
        &self,
        class: ReactiveStateInspectionHookClass,
    ) -> Option<&ReactiveStateInspectionHook> {
        self.inspection_hooks
            .iter()
            .find(|hook| hook.hook_class == class)
    }
}

// ---------------------------------------------------------------------------
// Projection.
// ---------------------------------------------------------------------------

/// Projects a governed reactive-state lineage record from a live
/// [`ReactiveStateInputs`] envelope using the default inspection-hook set.
pub fn project_reactive_state_lineage(
    posture_id: impl Into<String>,
    inputs: &ReactiveStateInputs,
) -> ReactiveStateLineageRecord {
    project_reactive_state_lineage_with_hooks(
        posture_id,
        inputs,
        default_reactive_state_inspection_hooks(),
    )
}

/// Like [`project_reactive_state_lineage`] but with an explicit
/// inspection-hook set (for testing degraded-hook postures).
pub fn project_reactive_state_lineage_with_hooks(
    posture_id: impl Into<String>,
    inputs: &ReactiveStateInputs,
    inspection_hooks: Vec<ReactiveStateInspectionHook>,
) -> ReactiveStateLineageRecord {
    let posture_id: String = posture_id.into();

    let view_class_coverage = project_view_class_coverage(inputs);
    let stale_view_downgrade = project_stale_view_downgrade(&view_class_coverage);
    let epoch_parity_honesty = project_epoch_parity_honesty(&view_class_coverage);
    let support_export_honesty = project_support_export_honesty(inputs);
    let producer_attribution = project_producer_attribution(inputs);

    let mut narrow_reasons = Vec::new();

    if inputs.views.is_empty() {
        narrow_reasons.push(ReactiveStateLineageNarrowReason::CorpusEmpty);
    }
    if !view_class_coverage.all_required_view_classes_present {
        narrow_reasons.push(ReactiveStateLineageNarrowReason::RequiredViewClassMissing);
    }
    if !view_class_coverage.all_required_consumer_surfaces_present {
        narrow_reasons.push(ReactiveStateLineageNarrowReason::RequiredConsumerSurfaceMissing);
    }
    if !view_class_coverage.no_subscriber_epoch_exceeds_authority {
        narrow_reasons.push(ReactiveStateLineageNarrowReason::SubscriberEpochExceedsAuthority);
    }

    collect_parity_narrows(&view_class_coverage, &mut narrow_reasons);
    collect_downgrade_narrows(&stale_view_downgrade, &mut narrow_reasons);
    collect_support_export_narrows(&support_export_honesty, &mut narrow_reasons);

    let required_hooks = [
        ReactiveStateInspectionHookClass::InspectDrift,
        ReactiveStateInspectionHookClass::CompareEpochs,
        ReactiveStateInspectionHookClass::ResyncReview,
        ReactiveStateInspectionHookClass::RollbackCheckpoint,
        ReactiveStateInspectionHookClass::Export,
        ReactiveStateInspectionHookClass::Repair,
    ];
    if !required_hooks
        .iter()
        .all(|required| hook_available(&inspection_hooks, *required))
    {
        narrow_reasons.push(ReactiveStateLineageNarrowReason::InspectionHookUnavailable);
    }

    if !producer_attribution.producer_attribution_complete {
        narrow_reasons.push(ReactiveStateLineageNarrowReason::ProducerAttributionIncomplete);
    }

    if inputs.workspace_ref.trim().is_empty() || inputs.corpus_ref.trim().is_empty() {
        narrow_reasons.push(ReactiveStateLineageNarrowReason::LineageExportUnsafe);
    }

    let qualified = narrow_reasons.is_empty();
    let stable_qualification = ReactiveStateLineageQualification {
        qualified,
        level: if qualified {
            "stable".to_owned()
        } else {
            "narrowed_below_stable".to_owned()
        },
        narrow_reasons,
    };

    let summary = build_summary(
        &view_class_coverage,
        &stale_view_downgrade,
        &epoch_parity_honesty,
        &stable_qualification,
    );

    ReactiveStateLineageRecord {
        record_kind: REACTIVE_STATE_LINEAGE_RECORD_KIND.to_owned(),
        reactive_state_lineage_schema_version: REACTIVE_STATE_LINEAGE_SCHEMA_VERSION,
        schema_ref: REACTIVE_STATE_LINEAGE_SCHEMA_REF.to_owned(),
        posture_id,
        workspace_ref: inputs.workspace_ref.clone(),
        corpus_ref: inputs.corpus_ref.clone(),
        producer_attribution,
        view_class_coverage,
        stale_view_downgrade,
        epoch_parity_honesty,
        support_export_honesty,
        inspection_hooks,
        stable_qualification,
        raw_payload_excluded: true,
        summary,
    }
}

// ---------------------------------------------------------------------------
// Pillar builders.
// ---------------------------------------------------------------------------

fn project_view_class_coverage(inputs: &ReactiveStateInputs) -> ViewClassCoverageSummary {
    let view_rows: Vec<ReactiveViewLineageRow> =
        inputs.views.iter().map(project_view_row).collect();

    let observed_classes: BTreeSet<_> = view_rows.iter().map(|row| row.view_class).collect();
    let all_required_view_classes_present = REQUIRED_VIEW_CLASSES
        .iter()
        .all(|required| observed_classes.contains(required));

    let all_required_consumer_surfaces_present = inputs.views.iter().all(|view| {
        let observed: BTreeSet<_> = view
            .subscriber_epochs
            .iter()
            .map(|sub| sub.surface_kind)
            .collect();
        REQUIRED_CONSUMER_SURFACES
            .iter()
            .all(|required| observed.contains(required))
    });

    let no_subscriber_epoch_exceeds_authority = inputs.views.iter().all(|view| {
        view.subscriber_epochs
            .iter()
            .all(|sub| sub.observed_epoch <= view.authority_epoch)
    });

    ViewClassCoverageSummary {
        view_rows,
        all_required_view_classes_present,
        all_required_consumer_surfaces_present,
        no_subscriber_epoch_exceeds_authority,
    }
}

fn project_view_row(view: &MaterializedViewObservation) -> ReactiveViewLineageRow {
    let (min_epoch, max_epoch) = subscriber_epoch_bounds(&view.subscriber_epochs)
        .unwrap_or((view.authority_epoch, view.authority_epoch));
    let subscribers_lagging_count = view
        .subscriber_epochs
        .iter()
        .filter(|sub| sub.observed_epoch < view.authority_epoch)
        .count();
    let any_unavailable = view
        .subscriber_epochs
        .iter()
        .any(|sub| sub.observed_freshness == SubscriberFreshness::Unavailable);
    let any_resync_required = view
        .subscriber_epochs
        .iter()
        .any(|sub| sub.last_invalidation_cause == InvalidationCauseClass::ResyncRequired);
    let derived = derive_parity_state(view);
    let parity_state_matches = derived == view.declared_parity_state;
    let open_gap_classes: Vec<OpenGapClass> =
        view.open_gaps.iter().map(|gap| gap.gap_class).collect();

    ReactiveViewLineageRow {
        view_id: view.view_id.clone(),
        title: view.title.clone(),
        view_class: view.view_class,
        authority_label: view.authority_label,
        authority_epoch: view.authority_epoch,
        derived_parity_state: derived,
        declared_parity_state: view.declared_parity_state,
        parity_state_matches,
        min_subscriber_epoch: min_epoch,
        max_subscriber_epoch: max_epoch,
        subscriber_count: view.subscriber_epochs.len(),
        subscribers_lagging_count,
        any_unavailable,
        any_resync_required,
        downgrade_label: view.declared_downgrade_label,
        open_gap_classes,
        support_export_posture: view.support_export.posture,
    }
}

fn derive_parity_state(view: &MaterializedViewObservation) -> EpochParityState {
    let any_unavailable = view
        .subscriber_epochs
        .iter()
        .any(|sub| sub.observed_freshness == SubscriberFreshness::Unavailable);
    if any_unavailable {
        return EpochParityState::TerminalUnavailable;
    }
    let any_resync_required = view
        .subscriber_epochs
        .iter()
        .any(|sub| sub.last_invalidation_cause == InvalidationCauseClass::ResyncRequired);
    let any_stale_or_warming = view.subscriber_epochs.iter().any(|sub| {
        matches!(
            sub.observed_freshness,
            SubscriberFreshness::Stale | SubscriberFreshness::Warming
        )
    });
    if any_resync_required || any_stale_or_warming {
        return EpochParityState::AwaitingResync;
    }
    let any_lag = view
        .subscriber_epochs
        .iter()
        .any(|sub| sub.observed_epoch < view.authority_epoch);
    if any_lag {
        return EpochParityState::DriftDetected;
    }
    let all_at_authority_authoritative = !view.subscriber_epochs.is_empty()
        && view.subscriber_epochs.iter().all(|sub| {
            sub.observed_epoch == view.authority_epoch
                && sub.observed_freshness == SubscriberFreshness::Authoritative
        });
    if all_at_authority_authoritative {
        EpochParityState::Aligned
    } else {
        EpochParityState::DriftDetected
    }
}

fn project_stale_view_downgrade(coverage: &ViewClassCoverageSummary) -> StaleViewDowngradeSummary {
    let mut aligned_count = 0usize;
    let mut drift_detected_count = 0usize;
    let mut awaiting_resync_count = 0usize;
    let mut terminal_unavailable_count = 0usize;
    let mut all_non_aligned_views_carry_downgrade = true;
    let mut all_aligned_views_carry_none = true;
    let mut all_downgraded_views_record_open_gap = true;
    let mut all_aligned_views_record_no_open_gap = true;

    for row in &coverage.view_rows {
        match row.declared_parity_state {
            EpochParityState::Aligned => aligned_count += 1,
            EpochParityState::DriftDetected => drift_detected_count += 1,
            EpochParityState::AwaitingResync => awaiting_resync_count += 1,
            EpochParityState::TerminalUnavailable => terminal_unavailable_count += 1,
        }

        let healthy = row.downgrade_label.is_healthy();
        let has_open_gap = row
            .open_gap_classes
            .iter()
            .any(|class| *class != OpenGapClass::None);

        if row.declared_parity_state.is_aligned() {
            if !healthy {
                all_aligned_views_carry_none = false;
            }
            if has_open_gap {
                all_aligned_views_record_no_open_gap = false;
            }
        } else {
            if healthy {
                all_non_aligned_views_carry_downgrade = false;
            }
            if !has_open_gap {
                all_downgraded_views_record_open_gap = false;
            }
        }
    }

    StaleViewDowngradeSummary {
        aligned_count,
        drift_detected_count,
        awaiting_resync_count,
        terminal_unavailable_count,
        all_non_aligned_views_carry_downgrade,
        all_aligned_views_carry_none,
        all_downgraded_views_record_open_gap,
        all_aligned_views_record_no_open_gap,
    }
}

fn project_epoch_parity_honesty(coverage: &ViewClassCoverageSummary) -> EpochParityHonestySummary {
    let mut all_views_declared_parity_matches_observation = true;
    let mut all_drift_views_have_epoch_lag = true;
    let mut all_awaiting_resync_views_have_signal = true;
    let mut all_terminal_unavailable_views_have_signal = true;

    for row in &coverage.view_rows {
        if !row.parity_state_matches {
            all_views_declared_parity_matches_observation = false;
        }
        match row.declared_parity_state {
            EpochParityState::DriftDetected => {
                if row.subscribers_lagging_count == 0 {
                    all_drift_views_have_epoch_lag = false;
                }
            }
            EpochParityState::AwaitingResync => {
                if !row.any_resync_required && row.subscribers_lagging_count == 0 {
                    all_awaiting_resync_views_have_signal = false;
                }
            }
            EpochParityState::TerminalUnavailable => {
                if !row.any_unavailable {
                    all_terminal_unavailable_views_have_signal = false;
                }
            }
            EpochParityState::Aligned => {}
        }
    }

    EpochParityHonestySummary {
        all_views_declared_parity_matches_observation,
        all_drift_views_have_epoch_lag,
        all_awaiting_resync_views_have_signal,
        all_terminal_unavailable_views_have_signal,
    }
}

fn project_support_export_honesty(inputs: &ReactiveStateInputs) -> ReactiveSupportExportSummary {
    let mut all_views_preserve_epoch_state = true;
    let mut all_views_redact_raw_private_material = true;
    let mut all_views_exclude_ambient_authority = true;
    let mut all_views_preserve_user_authored_files = true;
    let mut all_exportable_or_replicated_views_have_safe_posture = true;

    for view in &inputs.views {
        let support = view.support_export;
        if !(support.includes_view_class
            && support.includes_authority_label
            && support.includes_authority_epoch
            && support.includes_subscriber_epochs)
        {
            all_views_preserve_epoch_state = false;
        }
        if !support.raw_private_material_excluded {
            all_views_redact_raw_private_material = false;
        }
        if !support.ambient_authority_excluded {
            all_views_exclude_ambient_authority = false;
        }
        if !support.preserves_user_authored_files {
            all_views_preserve_user_authored_files = false;
        }
        if matches!(
            view.view_class,
            MaterializedViewClass::ExportableSnapshot
                | MaterializedViewClass::ManagedReplicatedView,
        ) && support.posture == SupportExportPosture::LocalOnly
        {
            all_exportable_or_replicated_views_have_safe_posture = false;
        }
    }

    ReactiveSupportExportSummary {
        all_views_preserve_epoch_state,
        all_views_redact_raw_private_material,
        all_views_exclude_ambient_authority,
        all_views_preserve_user_authored_files,
        all_exportable_or_replicated_views_have_safe_posture,
    }
}

fn project_producer_attribution(
    inputs: &ReactiveStateInputs,
) -> ReactiveProducerAttributionSummary {
    let integrity_hash = compute_integrity_hash(inputs);
    let producer_attribution_complete =
        !inputs.producer_ref.trim().is_empty() && !inputs.captured_at.trim().is_empty();
    ReactiveProducerAttributionSummary {
        producer_ref: inputs.producer_ref.clone(),
        schema_version: REACTIVE_STATE_LINEAGE_SCHEMA_VERSION,
        integrity_hash,
        captured_at: inputs.captured_at.clone(),
        producer_attribution_complete,
    }
}

fn collect_parity_narrows(
    coverage: &ViewClassCoverageSummary,
    narrow_reasons: &mut Vec<ReactiveStateLineageNarrowReason>,
) {
    let mut aligned_violation = false;
    let mut drift_violation = false;
    let mut awaiting_violation = false;
    let mut terminal_violation = false;

    for row in &coverage.view_rows {
        match row.declared_parity_state {
            EpochParityState::Aligned => {
                if !row.parity_state_matches {
                    aligned_violation = true;
                }
            }
            EpochParityState::DriftDetected => {
                if row.subscribers_lagging_count == 0 {
                    drift_violation = true;
                }
            }
            EpochParityState::AwaitingResync => {
                if !row.any_resync_required && row.subscribers_lagging_count == 0 {
                    awaiting_violation = true;
                }
            }
            EpochParityState::TerminalUnavailable => {
                if !row.any_unavailable {
                    terminal_violation = true;
                }
            }
        }
    }

    if aligned_violation {
        narrow_reasons.push(ReactiveStateLineageNarrowReason::AlignedParityNotProven);
    }
    if drift_violation {
        narrow_reasons.push(ReactiveStateLineageNarrowReason::DriftWithoutEpochLag);
    }
    if awaiting_violation {
        narrow_reasons.push(ReactiveStateLineageNarrowReason::AwaitingResyncWithoutSignal);
    }
    if terminal_violation {
        narrow_reasons.push(ReactiveStateLineageNarrowReason::TerminalUnavailableWithoutSignal);
    }
}

fn collect_downgrade_narrows(
    summary: &StaleViewDowngradeSummary,
    narrow_reasons: &mut Vec<ReactiveStateLineageNarrowReason>,
) {
    if !summary.all_non_aligned_views_carry_downgrade {
        narrow_reasons.push(ReactiveStateLineageNarrowReason::NonAlignedMissingDowngrade);
    }
    if !summary.all_aligned_views_carry_none {
        narrow_reasons.push(ReactiveStateLineageNarrowReason::AlignedCarriesDowngrade);
    }
    if !summary.all_downgraded_views_record_open_gap {
        narrow_reasons.push(ReactiveStateLineageNarrowReason::DowngradedWithoutOpenGap);
    }
    if !summary.all_aligned_views_record_no_open_gap {
        narrow_reasons.push(ReactiveStateLineageNarrowReason::AlignedCarriesOpenGap);
    }
}

fn collect_support_export_narrows(
    summary: &ReactiveSupportExportSummary,
    narrow_reasons: &mut Vec<ReactiveStateLineageNarrowReason>,
) {
    if !summary.all_views_preserve_epoch_state {
        narrow_reasons.push(ReactiveStateLineageNarrowReason::SupportExportEpochFieldsDropped);
    }
    if !summary.all_exportable_or_replicated_views_have_safe_posture {
        narrow_reasons.push(ReactiveStateLineageNarrowReason::SupportExportPostureUnsafe);
    }
    if !(summary.all_views_redact_raw_private_material
        && summary.all_views_exclude_ambient_authority
        && summary.all_views_preserve_user_authored_files)
    {
        narrow_reasons.push(ReactiveStateLineageNarrowReason::SupportExportRedactionUnsafe);
    }
}

fn subscriber_epoch_bounds(subscribers: &[SubscriberEpochObservation]) -> Option<(u64, u64)> {
    let mut iter = subscribers.iter().map(|s| s.observed_epoch);
    let first = iter.next()?;
    let mut min = first;
    let mut max = first;
    for value in iter {
        if value < min {
            min = value;
        }
        if value > max {
            max = value;
        }
    }
    Some((min, max))
}

fn compute_integrity_hash(inputs: &ReactiveStateInputs) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    let header = [
        inputs.workspace_ref.as_str(),
        inputs.producer_ref.as_str(),
        inputs.corpus_ref.as_str(),
        inputs.captured_at.as_str(),
    ];
    for input in header {
        for byte in input.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for view in &inputs.views {
        for byte in view.view_id.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(view.view_class.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= view.authority_epoch;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("rsl:{hash:016x}")
}

fn hook_available(
    hooks: &[ReactiveStateInspectionHook],
    class: ReactiveStateInspectionHookClass,
) -> bool {
    hooks
        .iter()
        .find(|hook| hook.hook_class == class)
        .map(|hook| hook.available)
        .unwrap_or(false)
}

fn build_summary(
    coverage: &ViewClassCoverageSummary,
    downgrade: &StaleViewDowngradeSummary,
    parity: &EpochParityHonestySummary,
    qualification: &ReactiveStateLineageQualification,
) -> String {
    if qualification.qualified {
        format!(
            "Reactive-state lineage proven Stable: views={total} aligned={aligned} drift={drift} awaiting_resync={awaiting} terminal_unavailable={terminal}; parity_state_matches_observation={matches}.",
            total = coverage.view_rows.len(),
            aligned = downgrade.aligned_count,
            drift = downgrade.drift_detected_count,
            awaiting = downgrade.awaiting_resync_count,
            terminal = downgrade.terminal_unavailable_count,
            matches = parity.all_views_declared_parity_matches_observation,
        )
    } else {
        let reasons: Vec<&str> = qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        format!(
            "Reactive-state lineage narrowed below Stable (views={total}): {reasons}.",
            total = coverage.view_rows.len(),
            reasons = reasons.join(", "),
        )
    }
}

// ---------------------------------------------------------------------------
// Human-readable projection (for headless emitter / shell status surface).
// ---------------------------------------------------------------------------

/// Returns the human-readable projection of a reactive-state lineage
/// record. The same projection is consumed by the workspace reactive-state
/// status surface, the headless CLI emitter, Help/About, and support
/// export.
pub fn reactive_state_lineage_lines(record: &ReactiveStateLineageRecord) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Reactive-state lineage — {} ({})",
        record.posture_id, record.stable_qualification.level
    ));
    lines.push(format!(
        "workspace={} corpus={} producer={} integrity_hash={} captured_at={}",
        record.workspace_ref,
        record.corpus_ref,
        record.producer_attribution.producer_ref,
        record.producer_attribution.integrity_hash,
        record.producer_attribution.captured_at,
    ));
    lines.push(format!(
        "view_class_coverage: views={} required_view_classes={} required_consumer_surfaces={} no_subscriber_epoch_exceeds_authority={}",
        record.view_class_coverage.view_rows.len(),
        record.view_class_coverage.all_required_view_classes_present,
        record
            .view_class_coverage
            .all_required_consumer_surfaces_present,
        record
            .view_class_coverage
            .no_subscriber_epoch_exceeds_authority,
    ));
    lines.push("View rows:".to_owned());
    for row in &record.view_class_coverage.view_rows {
        lines.push(format!(
            "  - {kind} {id} authority={authority} epoch={epoch} declared={declared} derived={derived} matches={matches} subscribers={subs} lagging={lag} downgrade={downgrade} support_export={posture} open_gaps={gaps}",
            kind = row.view_class.as_str(),
            id = row.view_id,
            authority = row.authority_label.as_str(),
            epoch = row.authority_epoch,
            declared = row.declared_parity_state.as_str(),
            derived = row.derived_parity_state.as_str(),
            matches = row.parity_state_matches,
            subs = row.subscriber_count,
            lag = row.subscribers_lagging_count,
            downgrade = row.downgrade_label.as_str(),
            posture = row.support_export_posture.as_str(),
            gaps = row.open_gap_classes.len(),
        ));
    }
    lines.push(format!(
        "Stale-view downgrade: aligned={} drift={} awaiting={} terminal={} non_aligned_carry_downgrade={} aligned_carry_none={} downgraded_record_gap={} aligned_record_no_gap={}",
        record.stale_view_downgrade.aligned_count,
        record.stale_view_downgrade.drift_detected_count,
        record.stale_view_downgrade.awaiting_resync_count,
        record.stale_view_downgrade.terminal_unavailable_count,
        record
            .stale_view_downgrade
            .all_non_aligned_views_carry_downgrade,
        record.stale_view_downgrade.all_aligned_views_carry_none,
        record
            .stale_view_downgrade
            .all_downgraded_views_record_open_gap,
        record
            .stale_view_downgrade
            .all_aligned_views_record_no_open_gap,
    ));
    lines.push(format!(
        "Epoch parity honesty: matches_observation={} drift_have_lag={} awaiting_have_signal={} terminal_have_signal={}",
        record
            .epoch_parity_honesty
            .all_views_declared_parity_matches_observation,
        record.epoch_parity_honesty.all_drift_views_have_epoch_lag,
        record
            .epoch_parity_honesty
            .all_awaiting_resync_views_have_signal,
        record
            .epoch_parity_honesty
            .all_terminal_unavailable_views_have_signal,
    ));
    lines.push(format!(
        "Support-export honesty: preserve_epoch_state={} redact_private={} exclude_ambient={} preserve_user_files={} exportable_replicated_safe={}",
        record.support_export_honesty.all_views_preserve_epoch_state,
        record
            .support_export_honesty
            .all_views_redact_raw_private_material,
        record
            .support_export_honesty
            .all_views_exclude_ambient_authority,
        record
            .support_export_honesty
            .all_views_preserve_user_authored_files,
        record
            .support_export_honesty
            .all_exportable_or_replicated_views_have_safe_posture,
    ));
    lines.push("Inspection hooks:".to_owned());
    for hook in &record.inspection_hooks {
        lines.push(format!(
            "  {class} [{id}] available={available} — {label}",
            class = hook.hook_class.as_str(),
            id = hook.action_id,
            available = hook.available,
            label = hook.label,
        ));
    }
    if !record.stable_qualification.qualified {
        let reasons: Vec<&str> = record
            .stable_qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        lines.push(format!("Narrowed below Stable: {}", reasons.join(", ")));
    }
    lines.push(record.summary.clone());
    lines
}

#[cfg(test)]
mod tests;
