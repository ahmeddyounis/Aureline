//! Cache and storage-class governance lineage: the governed,
//! export-safe projection that finalizes how Aureline manages caches,
//! derived stores, and durable workspace state — and proves that the
//! eviction rules and cleanup surfaces never lose user state silently.
//!
//! Where the portable-state lineage proves how restored state preserves
//! provenance and the reactive-state lineage proves how materialized
//! views stay aligned with their authority epoch, this projection
//! proves the *storage layer* underneath both: which storage classes
//! exist, which user-state class each one carries, which eviction
//! policy each one declares, which cleanup surfaces the user can reach
//! to clear them, and which inspection / repair hooks must fire before
//! any destructive cleanup commits.
//!
//! The projection ingests a live [`CacheStorageClassInputs`] envelope
//! verbatim (one [`CacheStorageObservation`] per storage class plus the
//! controlled inspection-hook table) and produces a lineage record
//! that proves the contract claims the stable line is anchored on:
//!
//! - **Storage-class coverage truth.** Every governed storage class
//!   ships a row bound to one closed [`StorageClassKind`] (ephemeral
//!   memory cache, local disk cache, derived index, durable workspace
//!   state, recovery checkpoint, local history, prebuild artifact
//!   cache, support-export staging). The corpus seeds one row per
//!   required class so the user never lands on a release where a
//!   user-state-bearing class slipped past governance.
//! - **Eviction-policy truth.** Each row declares one closed
//!   [`EvictionPolicyClass`]; the projection re-derives the worst-case
//!   durability tier from the user-state class and the eviction class
//!   so an `lru`/`ttl_age`/`quota_pressure` policy cannot ride on
//!   user-authored or user-derived content.
//! - **No user-state loss honesty.** Every storage class carrying user
//!   state declares an eviction policy from the closed safe set
//!   (`never`, `manual_only`, `manual_after_export`) and explicitly
//!   marks `silent_clear_blocked` plus `preserve_on_quota_pressure`.
//! - **Cleanup-surface coverage truth.** Every required cleanup
//!   surface (settings panel, command palette, support cleanup tool,
//!   Help/About, headless CLI) is reachable across the corpus and
//!   every storage class carrying user state binds at least one
//!   cleanup surface and requires pre-action inspection before the
//!   surface fires.
//! - **Claimed durability tier truth.** Each row declares one
//!   [`ClaimedDurabilityTier`]; the projection re-derives the
//!   worst-case tier from the eviction policy and user-state class so
//!   a `stable_durable_user_state` claim cannot ride on an `lru`
//!   policy.
//! - **Support-export honesty.** Each row's support-export projection
//!   preserves the storage class, user-state class, eviction policy,
//!   claimed tier, cleanup surfaces, and silent-clear-blocked flag
//!   while excluding raw cache content, raw secrets, approval tickets,
//!   delegated credentials, and live authority handles.
//! - **Pre-action inspection-hook honesty.** A controlled set of
//!   pre-action inspection / repair hooks (`inspect_cache`,
//!   `compare_before_clear`, `export_before_clear`, `rollback_clear`,
//!   `export`, `repair`) is reachable so destructive cleanups stay
//!   reviewable.
//! - **Lineage and export honesty.** The record sets
//!   `raw_payload_excluded = true` and carries only opaque refs to the
//!   source corpus, workspace, and producer.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Schema version for [`CacheStorageClassLineageRecord`].
pub const CACHE_STORAGE_CLASS_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the cache / storage-class lineage record.
pub const CACHE_STORAGE_CLASS_LINEAGE_SCHEMA_REF: &str =
    "schemas/workspace/cache_storage_class_lineage.schema.json";

/// Stable record-kind tag for the cache / storage-class lineage record.
pub const CACHE_STORAGE_CLASS_LINEAGE_RECORD_KIND: &str = "cache_storage_class_lineage_record";

// ---------------------------------------------------------------------------
// Closed vocabularies.
// ---------------------------------------------------------------------------

/// Closed vocabulary for the storage classes Aureline governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageClassKind {
    /// In-memory cache that drops on process restart.
    EphemeralMemoryCache,
    /// On-disk cache derived from upstream sources.
    LocalDiskCache,
    /// Derived index over user content (search, symbol, etc.).
    DerivedIndex,
    /// Durable workspace state — layout, panes, sessions.
    DurableWorkspaceState,
    /// Recovery / restore checkpoint snapshot of user state.
    RecoveryCheckpoint,
    /// Local-history record of user edits.
    LocalHistory,
    /// Prebuild / build artifact cache.
    PrebuildArtifactCache,
    /// Staging area used for support exports.
    SupportExportStaging,
}

impl StorageClassKind {
    /// Returns the stable snake_case token for this storage class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EphemeralMemoryCache => "ephemeral_memory_cache",
            Self::LocalDiskCache => "local_disk_cache",
            Self::DerivedIndex => "derived_index",
            Self::DurableWorkspaceState => "durable_workspace_state",
            Self::RecoveryCheckpoint => "recovery_checkpoint",
            Self::LocalHistory => "local_history",
            Self::PrebuildArtifactCache => "prebuild_artifact_cache",
            Self::SupportExportStaging => "support_export_staging",
        }
    }
}

/// Closed list of storage classes every cache / storage governance
/// record must seed.
pub const REQUIRED_STORAGE_CLASSES: [StorageClassKind; 6] = [
    StorageClassKind::EphemeralMemoryCache,
    StorageClassKind::LocalDiskCache,
    StorageClassKind::DerivedIndex,
    StorageClassKind::DurableWorkspaceState,
    StorageClassKind::RecoveryCheckpoint,
    StorageClassKind::LocalHistory,
];

/// Closed user-state vocabulary describing the content a storage class
/// holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStateClass {
    /// No user state; safe to drop without notice.
    NoUserState,
    /// No raw user state, but regenerating the content costs the user
    /// observable time.
    RegenerableWithCost,
    /// User-authored content (drafts, edits, layouts).
    UserAuthored,
    /// User-derived history captured by the IDE (local history, last
    /// session).
    UserDerived,
}

impl UserStateClass {
    /// Returns the stable snake_case token for this user-state class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoUserState => "no_user_state",
            Self::RegenerableWithCost => "regenerable_with_cost",
            Self::UserAuthored => "user_authored",
            Self::UserDerived => "user_derived",
        }
    }

    /// True when the class carries user state that must not be lost.
    pub const fn carries_user_state(self) -> bool {
        matches!(self, Self::UserAuthored | Self::UserDerived)
    }
}

/// Closed eviction-policy vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvictionPolicyClass {
    /// Content is durable; never evicted by the runtime.
    Never,
    /// Eviction only by explicit user action.
    ManualOnly,
    /// Eviction only after the user confirms an export.
    ManualAfterExport,
    /// Least-recently-used automatic eviction.
    Lru,
    /// Eviction once content reaches an age threshold.
    TtlAge,
    /// Eviction under quota pressure.
    QuotaPressure,
    /// Content drops on process restart.
    RestartDrop,
}

impl EvictionPolicyClass {
    /// Returns the stable snake_case token for this eviction policy.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Never => "never",
            Self::ManualOnly => "manual_only",
            Self::ManualAfterExport => "manual_after_export",
            Self::Lru => "lru",
            Self::TtlAge => "ttl_age",
            Self::QuotaPressure => "quota_pressure",
            Self::RestartDrop => "restart_drop",
        }
    }

    /// True when the policy is safe for user-state-bearing storage —
    /// content is only evicted with explicit user action.
    pub const fn is_user_state_safe(self) -> bool {
        matches!(
            self,
            Self::Never | Self::ManualOnly | Self::ManualAfterExport
        )
    }

    /// True when the policy lets the runtime drop content without
    /// explicit user action.
    pub const fn allows_silent_drop(self) -> bool {
        matches!(
            self,
            Self::Lru | Self::TtlAge | Self::QuotaPressure | Self::RestartDrop
        )
    }
}

/// Closed cleanup-surface vocabulary — the UI / CLI surfaces that can
/// clear a storage class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CleanupSurfaceKind {
    /// Settings panel storage / cache section.
    SettingsPanel,
    /// Command palette cleanup actions.
    CommandPalette,
    /// Support cleanup tool (bundled with support export flow).
    SupportCleanupTool,
    /// Help / About storage entry.
    HelpAbout,
    /// Headless CLI cleanup subcommand.
    HeadlessCli,
    /// Crash-recovery panel exposed after an unclean shutdown.
    CrashRecoveryPanel,
}

impl CleanupSurfaceKind {
    /// Returns the stable snake_case token for this cleanup surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingsPanel => "settings_panel",
            Self::CommandPalette => "command_palette",
            Self::SupportCleanupTool => "support_cleanup_tool",
            Self::HelpAbout => "help_about",
            Self::HeadlessCli => "headless_cli",
            Self::CrashRecoveryPanel => "crash_recovery_panel",
        }
    }
}

/// Closed list of cleanup surfaces that must be reachable across the
/// corpus.
pub const REQUIRED_CLEANUP_SURFACES: [CleanupSurfaceKind; 5] = [
    CleanupSurfaceKind::SettingsPanel,
    CleanupSurfaceKind::CommandPalette,
    CleanupSurfaceKind::SupportCleanupTool,
    CleanupSurfaceKind::HelpAbout,
    CleanupSurfaceKind::HeadlessCli,
];

/// Closed durability-tier vocabulary for a storage row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimedDurabilityTier {
    /// Durable storage of user state; eviction is gated.
    StableDurableUserState,
    /// Regenerable cache; eviction is automatic but recoverable.
    StableRegenerableCache,
    /// Ephemeral cache; eviction is silent and content is non-critical.
    StableEphemeralCache,
    /// Explicitly narrowed below Stable.
    NarrowedBelowStable,
}

impl ClaimedDurabilityTier {
    /// Returns the stable snake_case token for this tier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableDurableUserState => "stable_durable_user_state",
            Self::StableRegenerableCache => "stable_regenerable_cache",
            Self::StableEphemeralCache => "stable_ephemeral_cache",
            Self::NarrowedBelowStable => "narrowed_below_stable",
        }
    }
}

/// Closed support-export-posture vocabulary (mirrors the workspace
/// state-package vocabulary so support bundles can share posture
/// classifications across lineages).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CacheSupportExportPosture {
    /// Storage row stays local-only; the support packet redacts the
    /// row entirely.
    LocalOnly,
    /// Storage row ships a metadata-safe projection in the support
    /// packet.
    MetadataSafeExport,
    /// Storage row withholds its state from the support packet until a
    /// manual export reviews it.
    HeldRecord,
}

impl CacheSupportExportPosture {
    /// Returns the stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::MetadataSafeExport => "metadata_safe_export",
            Self::HeldRecord => "held_record",
        }
    }
}

/// Class of pre-action inspection / repair hook offered before any
/// destructive cache / storage cleanup commits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CacheStorageInspectionHookClass {
    /// Open the cache inspector for the storage class.
    InspectCache,
    /// Compare the storage row against its baseline before clearing.
    CompareBeforeClear,
    /// Export the storage row before clearing (when user state is
    /// present).
    ExportBeforeClear,
    /// Capture a one-step rollback before committing a cleanup.
    RollbackClear,
    /// Export the cache / storage lineage record (support-safe).
    Export,
    /// Open the repair sheet for a degraded storage class.
    Repair,
}

impl CacheStorageInspectionHookClass {
    /// Returns the stable snake_case token for this hook class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectCache => "inspect_cache",
            Self::CompareBeforeClear => "compare_before_clear",
            Self::ExportBeforeClear => "export_before_clear",
            Self::RollbackClear => "rollback_clear",
            Self::Export => "export",
            Self::Repair => "repair",
        }
    }
}

/// One pre-action inspection / repair hook offered before a cache /
/// storage cleanup commits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheStorageInspectionHook {
    /// Hook class.
    pub hook_class: CacheStorageInspectionHookClass,
    /// Stable action id.
    pub action_id: String,
    /// UI label.
    pub label: String,
    /// Whether the hook is reachable on this posture.
    pub available: bool,
    /// Disclosure shown when the hook is offered.
    pub disclosure: String,
}

/// Returns the default pre-action inspection / repair hook table.
pub fn default_cache_storage_inspection_hooks() -> Vec<CacheStorageInspectionHook> {
    vec![
        CacheStorageInspectionHook {
            hook_class: CacheStorageInspectionHookClass::InspectCache,
            action_id: "cache_storage.inspect_cache".to_owned(),
            label: "Inspect cache contents".to_owned(),
            available: true,
            disclosure:
                "Opens the cache inspector with the storage class, user-state class, eviction policy, and the captured cleanup surfaces before any cleanup commits."
                    .to_owned(),
        },
        CacheStorageInspectionHook {
            hook_class: CacheStorageInspectionHookClass::CompareBeforeClear,
            action_id: "cache_storage.compare_before_clear".to_owned(),
            label: "Compare before clear".to_owned(),
            available: true,
            disclosure:
                "Renders the diff between the live storage row and the baseline so the user can review what cleanup will drop before it fires."
                    .to_owned(),
        },
        CacheStorageInspectionHook {
            hook_class: CacheStorageInspectionHookClass::ExportBeforeClear,
            action_id: "cache_storage.export_before_clear".to_owned(),
            label: "Export before clear".to_owned(),
            available: true,
            disclosure:
                "Exports the storage row's user-visible state into a support-safe artifact before any cleanup commits, so user state can be restored if cleanup misfires."
                    .to_owned(),
        },
        CacheStorageInspectionHook {
            hook_class: CacheStorageInspectionHookClass::RollbackClear,
            action_id: "cache_storage.rollback_clear".to_owned(),
            label: "Rollback last clear".to_owned(),
            available: true,
            disclosure:
                "Captures a one-step rollback checkpoint so the user can revert a cleanup if a downstream surface relied on the cleared row."
                    .to_owned(),
        },
        CacheStorageInspectionHook {
            hook_class: CacheStorageInspectionHookClass::Export,
            action_id: "cache_storage.export".to_owned(),
            label: "Export cache / storage lineage".to_owned(),
            available: true,
            disclosure:
                "Exports this cache / storage lineage record for support without raw cache content, secrets, approval tickets, or delegated credentials."
                    .to_owned(),
        },
        CacheStorageInspectionHook {
            hook_class: CacheStorageInspectionHookClass::Repair,
            action_id: "cache_storage.repair".to_owned(),
            label: "Open repair sheet".to_owned(),
            available: true,
            disclosure:
                "Opens the repair sheet for a degraded storage class and surfaces the manual remediation steps rather than clearing as a shortcut."
                    .to_owned(),
        },
    ]
}

// ---------------------------------------------------------------------------
// Input envelope.
// ---------------------------------------------------------------------------

/// Metadata-safe support-export projection input for a storage row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheSupportExportInputs {
    pub posture: CacheSupportExportPosture,
    pub includes_storage_class: bool,
    pub includes_user_state_class: bool,
    pub includes_eviction_policy: bool,
    pub includes_claimed_tier: bool,
    pub includes_cleanup_surfaces: bool,
    pub includes_silent_clear_blocked: bool,
    pub raw_cache_content_excluded: bool,
    pub raw_secrets_excluded: bool,
    pub approval_tickets_excluded: bool,
    pub delegated_credentials_excluded: bool,
    pub live_authority_handles_excluded: bool,
}

impl CacheSupportExportInputs {
    /// Returns the metadata-safe baseline for a given posture.
    pub const fn metadata_safe_baseline(posture: CacheSupportExportPosture) -> Self {
        Self {
            posture,
            includes_storage_class: true,
            includes_user_state_class: true,
            includes_eviction_policy: true,
            includes_claimed_tier: true,
            includes_cleanup_surfaces: true,
            includes_silent_clear_blocked: true,
            raw_cache_content_excluded: true,
            raw_secrets_excluded: true,
            approval_tickets_excluded: true,
            delegated_credentials_excluded: true,
            live_authority_handles_excluded: true,
        }
    }
}

/// One observation of a governed storage class at a captured moment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheStorageObservation {
    /// Stable storage id (route-style, e.g. `local_history.workspace`).
    pub storage_id: String,
    /// Human-readable title.
    pub title: String,
    /// Closed storage class kind.
    pub storage_class: StorageClassKind,
    /// User-state class carried by this storage row.
    pub user_state_class: UserStateClass,
    /// Declared eviction policy.
    pub eviction_policy: EvictionPolicyClass,
    /// Claimed durability tier.
    pub claimed_durability_tier: ClaimedDurabilityTier,
    /// Cleanup surfaces that can clear this storage row.
    pub cleanup_surfaces: Vec<CleanupSurfaceKind>,
    /// Whether destructive cleanup is blocked from firing silently
    /// (must be true for user-state-bearing rows).
    pub silent_clear_blocked: bool,
    /// Whether a pre-action inspection hook must fire before cleanup
    /// (must be true for user-state-bearing rows).
    pub requires_pre_action_inspection: bool,
    /// Whether the row is preserved under quota pressure (must be true
    /// for user-state-bearing rows).
    pub preserve_on_quota_pressure: bool,
    /// Support-export projection.
    pub support_export: CacheSupportExportInputs,
    /// Capture timestamp.
    pub captured_at: String,
}

/// Input envelope ingested by the projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheStorageClassInputs {
    /// Opaque workspace ref the corpus describes.
    pub workspace_ref: String,
    /// Opaque producer ref.
    pub producer_ref: String,
    /// Opaque corpus ref.
    pub corpus_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Captured storage observations.
    pub storages: Vec<CacheStorageObservation>,
}

// ---------------------------------------------------------------------------
// Narrow reasons + qualification.
// ---------------------------------------------------------------------------

/// Named reason a cache / storage-class lineage record narrows below
/// Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CacheStorageClassLineageNarrowReason {
    /// The captured input had no storage observations.
    CorpusEmpty,
    /// A required storage class is missing from the corpus.
    RequiredStorageClassMissing,
    /// A required cleanup surface is missing from the corpus
    /// (cleanup-surface union does not cover the required set).
    RequiredCleanupSurfaceMissing,
    /// A storage row carrying user state declared an eviction policy
    /// that allows silent drop.
    UserStateWithUnsafeEviction,
    /// A storage row carrying user state did not declare
    /// `silent_clear_blocked = true`.
    UserStateCleanupNotBlocked,
    /// A storage row carrying user state did not declare
    /// `requires_pre_action_inspection = true`.
    UserStateInspectionNotRequired,
    /// A storage row carrying user state did not declare
    /// `preserve_on_quota_pressure = true`.
    UserStateQuotaPressureUnsafe,
    /// A storage row carrying user state did not bind any cleanup
    /// surface (the user can never clear it).
    UserStateCleanupSurfaceMissing,
    /// A storage row declared a tier that does not match the worst-case
    /// derived tier given the user-state class and eviction policy.
    DurabilityTierMismatchDerived,
    /// A required pre-action inspection / repair hook is unavailable.
    InspectionHookUnavailable,
    /// A support-export projection drops a required field.
    SupportExportFieldsDropped,
    /// Raw cache content, secrets, approval tickets, delegated
    /// credentials, or live authority handles slipped into a
    /// support-export projection.
    SupportExportRedactionUnsafe,
    /// A user-state-bearing storage row declared `local_only` support
    /// export (would hide a state loss from the support bundle).
    SupportExportPostureUnsafe,
    /// Producer attribution is incomplete (producer ref / captured-at).
    ProducerAttributionIncomplete,
    /// Workspace ref or corpus ref is empty (would break support
    /// export).
    LineageExportUnsafe,
}

impl CacheStorageClassLineageNarrowReason {
    /// Returns the stable snake_case token for this narrow reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CorpusEmpty => "corpus_empty",
            Self::RequiredStorageClassMissing => "required_storage_class_missing",
            Self::RequiredCleanupSurfaceMissing => "required_cleanup_surface_missing",
            Self::UserStateWithUnsafeEviction => "user_state_with_unsafe_eviction",
            Self::UserStateCleanupNotBlocked => "user_state_cleanup_not_blocked",
            Self::UserStateInspectionNotRequired => "user_state_inspection_not_required",
            Self::UserStateQuotaPressureUnsafe => "user_state_quota_pressure_unsafe",
            Self::UserStateCleanupSurfaceMissing => "user_state_cleanup_surface_missing",
            Self::DurabilityTierMismatchDerived => "durability_tier_mismatch_derived",
            Self::InspectionHookUnavailable => "inspection_hook_unavailable",
            Self::SupportExportFieldsDropped => "support_export_fields_dropped",
            Self::SupportExportRedactionUnsafe => "support_export_redaction_unsafe",
            Self::SupportExportPostureUnsafe => "support_export_posture_unsafe",
            Self::ProducerAttributionIncomplete => "producer_attribution_incomplete",
            Self::LineageExportUnsafe => "lineage_export_unsafe",
        }
    }
}

/// Stable-qualification posture for a cache / storage-class lineage
/// record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheStorageClassLineageQualification {
    /// Whether the record proves the contract on the claimed posture.
    pub qualified: bool,
    /// Stable lifecycle label: `stable` or `narrowed_below_stable`.
    pub level: String,
    /// Named reasons the record narrowed below Stable, when not
    /// qualified.
    pub narrow_reasons: Vec<CacheStorageClassLineageNarrowReason>,
}

// ---------------------------------------------------------------------------
// Pillar projections.
// ---------------------------------------------------------------------------

/// One storage row carried in the lineage projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheStorageRow {
    /// Stable storage id.
    pub storage_id: String,
    /// Storage title.
    pub title: String,
    /// Storage class kind.
    pub storage_class: StorageClassKind,
    /// User-state class.
    pub user_state_class: UserStateClass,
    /// Eviction policy.
    pub eviction_policy: EvictionPolicyClass,
    /// Declared durability tier.
    pub declared_durability_tier: ClaimedDurabilityTier,
    /// Re-derived worst-case tier given the user-state and eviction
    /// classes.
    pub derived_durability_tier: ClaimedDurabilityTier,
    /// True when the declared tier matches the derived tier.
    pub tier_matches: bool,
    /// Cleanup surfaces bound to this storage row.
    pub cleanup_surfaces: Vec<CleanupSurfaceKind>,
    /// True when destructive cleanup is blocked from firing silently.
    pub silent_clear_blocked: bool,
    /// True when a pre-action inspection hook must fire before cleanup.
    pub requires_pre_action_inspection: bool,
    /// True when this row is preserved under quota pressure.
    pub preserve_on_quota_pressure: bool,
    /// True when this row carries user state that must not be lost.
    pub carries_user_state: bool,
    /// Support-export posture.
    pub support_export_posture: CacheSupportExportPosture,
}

/// Storage-class coverage posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageClassCoverageSummary {
    /// All storage rows carried by the corpus.
    pub storage_rows: Vec<CacheStorageRow>,
    /// True when every required storage class is present.
    pub all_required_storage_classes_present: bool,
}

/// Eviction-policy truth posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvictionPolicyTruthSummary {
    /// True when every declared durability tier matches the derived
    /// tier.
    pub all_tiers_match_derived: bool,
    /// Count of unique eviction policies observed across rows.
    pub distinct_eviction_policies: usize,
}

/// User-state governance posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserStateGovernanceSummary {
    /// Count of storage rows that carry user state.
    pub user_state_row_count: usize,
    /// True when every user-state-bearing row declares an eviction
    /// policy from the safe set.
    pub all_user_state_rows_have_safe_eviction: bool,
    /// True when every user-state-bearing row declares
    /// `silent_clear_blocked = true`.
    pub all_user_state_rows_block_silent_clear: bool,
    /// True when every user-state-bearing row declares
    /// `requires_pre_action_inspection = true`.
    pub all_user_state_rows_require_inspection: bool,
    /// True when every user-state-bearing row declares
    /// `preserve_on_quota_pressure = true`.
    pub all_user_state_rows_preserve_on_quota_pressure: bool,
    /// True when every user-state-bearing row binds at least one
    /// cleanup surface.
    pub all_user_state_rows_have_cleanup_surface: bool,
}

/// Cleanup-surface coverage posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanupSurfaceCoverageSummary {
    /// Union of cleanup surfaces observed across the corpus.
    pub observed_cleanup_surfaces: Vec<CleanupSurfaceKind>,
    /// True when every required cleanup surface is reachable.
    pub all_required_cleanup_surfaces_present: bool,
}

/// Support-export honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheSupportExportHonestySummary {
    /// True when every storage row's support-export projection
    /// preserves the required cache / storage fields.
    pub all_rows_preserve_fields: bool,
    /// True when every storage row excludes raw cache content.
    pub all_rows_exclude_raw_cache_content: bool,
    /// True when every storage row redacts raw secrets.
    pub all_rows_redact_raw_secrets: bool,
    /// True when every storage row excludes approval tickets.
    pub all_rows_exclude_approval_tickets: bool,
    /// True when every storage row excludes delegated credentials.
    pub all_rows_exclude_delegated_credentials: bool,
    /// True when every storage row excludes live authority handles.
    pub all_rows_exclude_live_authority_handles: bool,
    /// True when every user-state-bearing storage row declares a
    /// non-`local_only` support-export posture.
    pub all_user_state_rows_have_safe_posture: bool,
}

/// Producer-attribution posture for replay safety.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheStorageProducerAttributionSummary {
    /// Opaque producer build / instance ref.
    pub producer_ref: String,
    /// Schema version pinned by the input.
    pub schema_version: u32,
    /// Opaque integrity hash derived from the input identities.
    pub integrity_hash: String,
    /// Input capture timestamp.
    pub captured_at: String,
    /// True when producer attribution fields are non-empty.
    pub producer_attribution_complete: bool,
}

// ---------------------------------------------------------------------------
// Top-level record.
// ---------------------------------------------------------------------------

/// Governed, export-safe cache / storage-class lineage record per
/// posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheStorageClassLineageRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub cache_storage_class_lineage_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable posture id.
    pub posture_id: String,
    /// Workspace ref the corpus describes.
    pub workspace_ref: String,
    /// Opaque ref to the corpus the projection ingested.
    pub corpus_ref: String,
    /// Producer attribution pillar.
    pub producer_attribution: CacheStorageProducerAttributionSummary,
    /// Storage-class coverage pillar.
    pub storage_class_coverage: StorageClassCoverageSummary,
    /// Eviction-policy truth pillar.
    pub eviction_policy_truth: EvictionPolicyTruthSummary,
    /// User-state governance pillar.
    pub user_state_governance: UserStateGovernanceSummary,
    /// Cleanup-surface coverage pillar.
    pub cleanup_surface_coverage: CleanupSurfaceCoverageSummary,
    /// Support-export honesty pillar.
    pub support_export_honesty: CacheSupportExportHonestySummary,
    /// Pre-action inspection / repair hooks.
    pub inspection_hooks: Vec<CacheStorageInspectionHook>,
    /// Stable-qualification posture with named narrow reasons.
    pub stable_qualification: CacheStorageClassLineageQualification,
    /// Whether the record is metadata-safe for support export.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl CacheStorageClassLineageRecord {
    /// Returns true when the record is metadata-safe for support
    /// export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == CACHE_STORAGE_CLASS_LINEAGE_SCHEMA_REF
            && self.record_kind == CACHE_STORAGE_CLASS_LINEAGE_RECORD_KIND
            && !self.workspace_ref.trim().is_empty()
            && !self.corpus_ref.trim().is_empty()
    }

    /// Returns true when the record proves the contract on the
    /// claimed posture.
    pub fn is_stable_qualified(&self) -> bool {
        self.stable_qualification.qualified
    }

    /// Returns the inspection hook of the given class, when present.
    pub fn inspection_hook(
        &self,
        class: CacheStorageInspectionHookClass,
    ) -> Option<&CacheStorageInspectionHook> {
        self.inspection_hooks
            .iter()
            .find(|hook| hook.hook_class == class)
    }
}

// ---------------------------------------------------------------------------
// Projection.
// ---------------------------------------------------------------------------

/// Projects a governed cache / storage-class lineage record from a
/// live [`CacheStorageClassInputs`] envelope using the default
/// inspection-hook set.
pub fn project_cache_storage_class_lineage(
    posture_id: impl Into<String>,
    inputs: &CacheStorageClassInputs,
) -> CacheStorageClassLineageRecord {
    project_cache_storage_class_lineage_with_hooks(
        posture_id,
        inputs,
        default_cache_storage_inspection_hooks(),
    )
}

/// Like [`project_cache_storage_class_lineage`] but with an explicit
/// inspection-hook set (for testing degraded-hook postures).
pub fn project_cache_storage_class_lineage_with_hooks(
    posture_id: impl Into<String>,
    inputs: &CacheStorageClassInputs,
    inspection_hooks: Vec<CacheStorageInspectionHook>,
) -> CacheStorageClassLineageRecord {
    let posture_id: String = posture_id.into();

    let storage_class_coverage = project_storage_class_coverage(inputs);
    let eviction_policy_truth = project_eviction_policy_truth(&storage_class_coverage, inputs);
    let user_state_governance = project_user_state_governance(&storage_class_coverage);
    let cleanup_surface_coverage = project_cleanup_surface_coverage(&storage_class_coverage);
    let support_export_honesty = project_support_export_honesty(inputs);
    let producer_attribution = project_producer_attribution(inputs);

    let mut narrow_reasons = Vec::new();

    if inputs.storages.is_empty() {
        narrow_reasons.push(CacheStorageClassLineageNarrowReason::CorpusEmpty);
    }
    if !storage_class_coverage.all_required_storage_classes_present {
        narrow_reasons.push(CacheStorageClassLineageNarrowReason::RequiredStorageClassMissing);
    }
    if !cleanup_surface_coverage.all_required_cleanup_surfaces_present {
        narrow_reasons.push(CacheStorageClassLineageNarrowReason::RequiredCleanupSurfaceMissing);
    }

    collect_user_state_narrows(&user_state_governance, &mut narrow_reasons);
    collect_eviction_policy_narrows(&eviction_policy_truth, &mut narrow_reasons);
    collect_support_export_narrows(&support_export_honesty, &mut narrow_reasons);

    let required_hooks = [
        CacheStorageInspectionHookClass::InspectCache,
        CacheStorageInspectionHookClass::CompareBeforeClear,
        CacheStorageInspectionHookClass::ExportBeforeClear,
        CacheStorageInspectionHookClass::RollbackClear,
        CacheStorageInspectionHookClass::Export,
        CacheStorageInspectionHookClass::Repair,
    ];
    if !required_hooks
        .iter()
        .all(|required| hook_available(&inspection_hooks, *required))
    {
        narrow_reasons.push(CacheStorageClassLineageNarrowReason::InspectionHookUnavailable);
    }

    if !producer_attribution.producer_attribution_complete {
        narrow_reasons.push(CacheStorageClassLineageNarrowReason::ProducerAttributionIncomplete);
    }

    if inputs.workspace_ref.trim().is_empty() || inputs.corpus_ref.trim().is_empty() {
        narrow_reasons.push(CacheStorageClassLineageNarrowReason::LineageExportUnsafe);
    }

    let qualified = narrow_reasons.is_empty();
    let stable_qualification = CacheStorageClassLineageQualification {
        qualified,
        level: if qualified {
            "stable".to_owned()
        } else {
            "narrowed_below_stable".to_owned()
        },
        narrow_reasons,
    };

    let summary = build_summary(
        &storage_class_coverage,
        &user_state_governance,
        &stable_qualification,
    );

    CacheStorageClassLineageRecord {
        record_kind: CACHE_STORAGE_CLASS_LINEAGE_RECORD_KIND.to_owned(),
        cache_storage_class_lineage_schema_version: CACHE_STORAGE_CLASS_LINEAGE_SCHEMA_VERSION,
        schema_ref: CACHE_STORAGE_CLASS_LINEAGE_SCHEMA_REF.to_owned(),
        posture_id,
        workspace_ref: inputs.workspace_ref.clone(),
        corpus_ref: inputs.corpus_ref.clone(),
        producer_attribution,
        storage_class_coverage,
        eviction_policy_truth,
        user_state_governance,
        cleanup_surface_coverage,
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

fn project_storage_class_coverage(inputs: &CacheStorageClassInputs) -> StorageClassCoverageSummary {
    let storage_rows: Vec<CacheStorageRow> =
        inputs.storages.iter().map(project_storage_row).collect();
    let observed: BTreeSet<_> = storage_rows.iter().map(|row| row.storage_class).collect();
    let all_required_storage_classes_present = REQUIRED_STORAGE_CLASSES
        .iter()
        .all(|required| observed.contains(required));
    StorageClassCoverageSummary {
        storage_rows,
        all_required_storage_classes_present,
    }
}

fn project_storage_row(storage: &CacheStorageObservation) -> CacheStorageRow {
    let derived_durability_tier = derive_durability_tier(storage);
    let tier_matches = storage.claimed_durability_tier == derived_durability_tier;
    let carries_user_state = storage.user_state_class.carries_user_state();
    CacheStorageRow {
        storage_id: storage.storage_id.clone(),
        title: storage.title.clone(),
        storage_class: storage.storage_class,
        user_state_class: storage.user_state_class,
        eviction_policy: storage.eviction_policy,
        declared_durability_tier: storage.claimed_durability_tier,
        derived_durability_tier,
        tier_matches,
        cleanup_surfaces: storage.cleanup_surfaces.clone(),
        silent_clear_blocked: storage.silent_clear_blocked,
        requires_pre_action_inspection: storage.requires_pre_action_inspection,
        preserve_on_quota_pressure: storage.preserve_on_quota_pressure,
        carries_user_state,
        support_export_posture: storage.support_export.posture,
    }
}

fn derive_durability_tier(storage: &CacheStorageObservation) -> ClaimedDurabilityTier {
    match storage.user_state_class {
        UserStateClass::UserAuthored | UserStateClass::UserDerived => {
            if storage.eviction_policy.is_user_state_safe()
                && storage.silent_clear_blocked
                && storage.preserve_on_quota_pressure
            {
                ClaimedDurabilityTier::StableDurableUserState
            } else {
                ClaimedDurabilityTier::NarrowedBelowStable
            }
        }
        UserStateClass::RegenerableWithCost => match storage.eviction_policy {
            EvictionPolicyClass::Never
            | EvictionPolicyClass::ManualOnly
            | EvictionPolicyClass::ManualAfterExport
            | EvictionPolicyClass::Lru
            | EvictionPolicyClass::TtlAge
            | EvictionPolicyClass::QuotaPressure => ClaimedDurabilityTier::StableRegenerableCache,
            EvictionPolicyClass::RestartDrop => ClaimedDurabilityTier::StableEphemeralCache,
        },
        UserStateClass::NoUserState => match storage.eviction_policy {
            EvictionPolicyClass::RestartDrop => ClaimedDurabilityTier::StableEphemeralCache,
            EvictionPolicyClass::Lru
            | EvictionPolicyClass::TtlAge
            | EvictionPolicyClass::QuotaPressure
            | EvictionPolicyClass::Never
            | EvictionPolicyClass::ManualOnly
            | EvictionPolicyClass::ManualAfterExport => {
                ClaimedDurabilityTier::StableRegenerableCache
            }
        },
    }
}

fn project_eviction_policy_truth(
    coverage: &StorageClassCoverageSummary,
    inputs: &CacheStorageClassInputs,
) -> EvictionPolicyTruthSummary {
    let mut all_tiers_match_derived = true;
    for row in &coverage.storage_rows {
        if !row.tier_matches {
            all_tiers_match_derived = false;
        }
    }
    let distinct: BTreeSet<_> = inputs.storages.iter().map(|s| s.eviction_policy).collect();
    EvictionPolicyTruthSummary {
        all_tiers_match_derived,
        distinct_eviction_policies: distinct.len(),
    }
}

fn project_user_state_governance(
    coverage: &StorageClassCoverageSummary,
) -> UserStateGovernanceSummary {
    let mut user_state_row_count = 0usize;
    let mut all_user_state_rows_have_safe_eviction = true;
    let mut all_user_state_rows_block_silent_clear = true;
    let mut all_user_state_rows_require_inspection = true;
    let mut all_user_state_rows_preserve_on_quota_pressure = true;
    let mut all_user_state_rows_have_cleanup_surface = true;

    for row in &coverage.storage_rows {
        if !row.carries_user_state {
            continue;
        }
        user_state_row_count += 1;
        if !row.eviction_policy.is_user_state_safe() {
            all_user_state_rows_have_safe_eviction = false;
        }
        if !row.silent_clear_blocked {
            all_user_state_rows_block_silent_clear = false;
        }
        if !row.requires_pre_action_inspection {
            all_user_state_rows_require_inspection = false;
        }
        if !row.preserve_on_quota_pressure {
            all_user_state_rows_preserve_on_quota_pressure = false;
        }
        if row.cleanup_surfaces.is_empty() {
            all_user_state_rows_have_cleanup_surface = false;
        }
    }

    UserStateGovernanceSummary {
        user_state_row_count,
        all_user_state_rows_have_safe_eviction,
        all_user_state_rows_block_silent_clear,
        all_user_state_rows_require_inspection,
        all_user_state_rows_preserve_on_quota_pressure,
        all_user_state_rows_have_cleanup_surface,
    }
}

fn project_cleanup_surface_coverage(
    coverage: &StorageClassCoverageSummary,
) -> CleanupSurfaceCoverageSummary {
    let mut observed_set: BTreeSet<CleanupSurfaceKind> = BTreeSet::new();
    for row in &coverage.storage_rows {
        for surface in &row.cleanup_surfaces {
            observed_set.insert(*surface);
        }
    }
    let observed_cleanup_surfaces: Vec<CleanupSurfaceKind> = observed_set.iter().copied().collect();
    let all_required_cleanup_surfaces_present = REQUIRED_CLEANUP_SURFACES
        .iter()
        .all(|required| observed_set.contains(required));
    CleanupSurfaceCoverageSummary {
        observed_cleanup_surfaces,
        all_required_cleanup_surfaces_present,
    }
}

fn project_support_export_honesty(
    inputs: &CacheStorageClassInputs,
) -> CacheSupportExportHonestySummary {
    let mut all_rows_preserve_fields = true;
    let mut all_rows_exclude_raw_cache_content = true;
    let mut all_rows_redact_raw_secrets = true;
    let mut all_rows_exclude_approval_tickets = true;
    let mut all_rows_exclude_delegated_credentials = true;
    let mut all_rows_exclude_live_authority_handles = true;
    let mut all_user_state_rows_have_safe_posture = true;

    for storage in &inputs.storages {
        let support = storage.support_export;
        if !(support.includes_storage_class
            && support.includes_user_state_class
            && support.includes_eviction_policy
            && support.includes_claimed_tier
            && support.includes_cleanup_surfaces
            && support.includes_silent_clear_blocked)
        {
            all_rows_preserve_fields = false;
        }
        if !support.raw_cache_content_excluded {
            all_rows_exclude_raw_cache_content = false;
        }
        if !support.raw_secrets_excluded {
            all_rows_redact_raw_secrets = false;
        }
        if !support.approval_tickets_excluded {
            all_rows_exclude_approval_tickets = false;
        }
        if !support.delegated_credentials_excluded {
            all_rows_exclude_delegated_credentials = false;
        }
        if !support.live_authority_handles_excluded {
            all_rows_exclude_live_authority_handles = false;
        }
        if storage.user_state_class.carries_user_state()
            && support.posture == CacheSupportExportPosture::LocalOnly
        {
            all_user_state_rows_have_safe_posture = false;
        }
    }

    CacheSupportExportHonestySummary {
        all_rows_preserve_fields,
        all_rows_exclude_raw_cache_content,
        all_rows_redact_raw_secrets,
        all_rows_exclude_approval_tickets,
        all_rows_exclude_delegated_credentials,
        all_rows_exclude_live_authority_handles,
        all_user_state_rows_have_safe_posture,
    }
}

fn project_producer_attribution(
    inputs: &CacheStorageClassInputs,
) -> CacheStorageProducerAttributionSummary {
    let integrity_hash = compute_integrity_hash(inputs);
    let producer_attribution_complete =
        !inputs.producer_ref.trim().is_empty() && !inputs.captured_at.trim().is_empty();
    CacheStorageProducerAttributionSummary {
        producer_ref: inputs.producer_ref.clone(),
        schema_version: CACHE_STORAGE_CLASS_LINEAGE_SCHEMA_VERSION,
        integrity_hash,
        captured_at: inputs.captured_at.clone(),
        producer_attribution_complete,
    }
}

fn collect_user_state_narrows(
    summary: &UserStateGovernanceSummary,
    narrow_reasons: &mut Vec<CacheStorageClassLineageNarrowReason>,
) {
    if !summary.all_user_state_rows_have_safe_eviction {
        narrow_reasons.push(CacheStorageClassLineageNarrowReason::UserStateWithUnsafeEviction);
    }
    if !summary.all_user_state_rows_block_silent_clear {
        narrow_reasons.push(CacheStorageClassLineageNarrowReason::UserStateCleanupNotBlocked);
    }
    if !summary.all_user_state_rows_require_inspection {
        narrow_reasons.push(CacheStorageClassLineageNarrowReason::UserStateInspectionNotRequired);
    }
    if !summary.all_user_state_rows_preserve_on_quota_pressure {
        narrow_reasons.push(CacheStorageClassLineageNarrowReason::UserStateQuotaPressureUnsafe);
    }
    if !summary.all_user_state_rows_have_cleanup_surface {
        narrow_reasons.push(CacheStorageClassLineageNarrowReason::UserStateCleanupSurfaceMissing);
    }
}

fn collect_eviction_policy_narrows(
    summary: &EvictionPolicyTruthSummary,
    narrow_reasons: &mut Vec<CacheStorageClassLineageNarrowReason>,
) {
    if !summary.all_tiers_match_derived {
        narrow_reasons.push(CacheStorageClassLineageNarrowReason::DurabilityTierMismatchDerived);
    }
}

fn collect_support_export_narrows(
    summary: &CacheSupportExportHonestySummary,
    narrow_reasons: &mut Vec<CacheStorageClassLineageNarrowReason>,
) {
    if !summary.all_rows_preserve_fields {
        narrow_reasons.push(CacheStorageClassLineageNarrowReason::SupportExportFieldsDropped);
    }
    if !summary.all_user_state_rows_have_safe_posture {
        narrow_reasons.push(CacheStorageClassLineageNarrowReason::SupportExportPostureUnsafe);
    }
    if !(summary.all_rows_exclude_raw_cache_content
        && summary.all_rows_redact_raw_secrets
        && summary.all_rows_exclude_approval_tickets
        && summary.all_rows_exclude_delegated_credentials
        && summary.all_rows_exclude_live_authority_handles)
    {
        narrow_reasons.push(CacheStorageClassLineageNarrowReason::SupportExportRedactionUnsafe);
    }
}

fn compute_integrity_hash(inputs: &CacheStorageClassInputs) -> String {
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
    for storage in &inputs.storages {
        for byte in storage.storage_id.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(storage.storage_class.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(storage.user_state_class.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(storage.eviction_policy.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(storage.claimed_durability_tier.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("csl:{hash:016x}")
}

fn hook_available(
    hooks: &[CacheStorageInspectionHook],
    class: CacheStorageInspectionHookClass,
) -> bool {
    hooks
        .iter()
        .find(|hook| hook.hook_class == class)
        .map(|hook| hook.available)
        .unwrap_or(false)
}

fn build_summary(
    coverage: &StorageClassCoverageSummary,
    user_state: &UserStateGovernanceSummary,
    qualification: &CacheStorageClassLineageQualification,
) -> String {
    if qualification.qualified {
        format!(
            "Cache / storage-class lineage proven Stable: storages={total} user_state_rows={user} all_safe_eviction={safe} all_block_silent={block}.",
            total = coverage.storage_rows.len(),
            user = user_state.user_state_row_count,
            safe = user_state.all_user_state_rows_have_safe_eviction,
            block = user_state.all_user_state_rows_block_silent_clear,
        )
    } else {
        let reasons: Vec<&str> = qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        format!(
            "Cache / storage-class lineage narrowed below Stable (storages={total}): {reasons}.",
            total = coverage.storage_rows.len(),
            reasons = reasons.join(", "),
        )
    }
}

// ---------------------------------------------------------------------------
// Human-readable projection (for headless emitter / shell status surface).
// ---------------------------------------------------------------------------

/// Returns the human-readable projection of a cache / storage-class
/// lineage record. The same projection is consumed by the workspace
/// cache / storage status surface, the headless CLI emitter,
/// Help/About, and support export.
pub fn cache_storage_class_lineage_lines(record: &CacheStorageClassLineageRecord) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Cache / storage-class lineage — {} ({})",
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
        "storage_class_coverage: storages={} required_classes_present={}",
        record.storage_class_coverage.storage_rows.len(),
        record
            .storage_class_coverage
            .all_required_storage_classes_present,
    ));
    lines.push("Storage rows:".to_owned());
    for row in &record.storage_class_coverage.storage_rows {
        let cleanup_list: Vec<&str> = row
            .cleanup_surfaces
            .iter()
            .map(|surface| surface.as_str())
            .collect();
        lines.push(format!(
            "  - {kind} {id} user_state={user} eviction={eviction} tier_declared={declared} tier_derived={derived} tier_matches={matches} cleanup=[{cleanup}] silent_clear_blocked={block} requires_inspection={inspect} preserve_on_quota={preserve} carries_user_state={carries} support_export={posture}",
            kind = row.storage_class.as_str(),
            id = row.storage_id,
            user = row.user_state_class.as_str(),
            eviction = row.eviction_policy.as_str(),
            declared = row.declared_durability_tier.as_str(),
            derived = row.derived_durability_tier.as_str(),
            matches = row.tier_matches,
            cleanup = cleanup_list.join(","),
            block = row.silent_clear_blocked,
            inspect = row.requires_pre_action_inspection,
            preserve = row.preserve_on_quota_pressure,
            carries = row.carries_user_state,
            posture = row.support_export_posture.as_str(),
        ));
    }
    lines.push(format!(
        "Eviction policy truth: all_tiers_match_derived={tiers} distinct_policies={distinct}",
        tiers = record.eviction_policy_truth.all_tiers_match_derived,
        distinct = record.eviction_policy_truth.distinct_eviction_policies,
    ));
    lines.push(format!(
        "User-state governance: user_state_rows={count} safe_eviction={eviction} block_silent_clear={block} require_inspection={inspect} preserve_on_quota={preserve} has_cleanup_surface={cleanup}",
        count = record.user_state_governance.user_state_row_count,
        eviction = record
            .user_state_governance
            .all_user_state_rows_have_safe_eviction,
        block = record
            .user_state_governance
            .all_user_state_rows_block_silent_clear,
        inspect = record
            .user_state_governance
            .all_user_state_rows_require_inspection,
        preserve = record
            .user_state_governance
            .all_user_state_rows_preserve_on_quota_pressure,
        cleanup = record
            .user_state_governance
            .all_user_state_rows_have_cleanup_surface,
    ));
    let cleanup_list: Vec<&str> = record
        .cleanup_surface_coverage
        .observed_cleanup_surfaces
        .iter()
        .map(|surface| surface.as_str())
        .collect();
    lines.push(format!(
        "Cleanup surface coverage: required_present={present} observed=[{observed}]",
        present = record
            .cleanup_surface_coverage
            .all_required_cleanup_surfaces_present,
        observed = cleanup_list.join(","),
    ));
    lines.push(format!(
        "Support-export honesty: preserve_fields={fields} exclude_raw_cache={raw} redact_secrets={secrets} exclude_approvals={approvals} exclude_credentials={credentials} exclude_authority={authority} user_state_safe_posture={posture}",
        fields = record.support_export_honesty.all_rows_preserve_fields,
        raw = record
            .support_export_honesty
            .all_rows_exclude_raw_cache_content,
        secrets = record.support_export_honesty.all_rows_redact_raw_secrets,
        approvals = record
            .support_export_honesty
            .all_rows_exclude_approval_tickets,
        credentials = record
            .support_export_honesty
            .all_rows_exclude_delegated_credentials,
        authority = record
            .support_export_honesty
            .all_rows_exclude_live_authority_handles,
        posture = record
            .support_export_honesty
            .all_user_state_rows_have_safe_posture,
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
