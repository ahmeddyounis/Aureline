//! Low-disk and managed-quota pressure banners, visible eviction ordering,
//! paused-work disclosure, and no-authoritative-state-loss guards for the heavy
//! artifact families the M5 depth lanes add.
//!
//! A storage-pressure banner is the operator-facing object the shell shows when
//! disk or quota pressure narrows a surface. It replaces a silent trim with an
//! honest, inspectable disclosure: which pressure class fired, which background
//! work paused, the frozen eviction order that applies, which classes stay
//! protected, and the action that opens the storage inspector. It covers
//! low-disk floors and managed quota ceilings, and it never lets pressure
//! silently delete authoritative recovery or referenced evidence state.
//!
//! This module is the canonical, inspectable truth model behind that banner. It
//! mints no new storage primitive: the storage-class, low-disk-ladder,
//! artifact-family, and authority vocabularies re-export verbatim from
//! [`crate::storage_inspector`] and [`crate::m5_storage_governance`]. Only the
//! pressure-class, pressure-source, paused-work, eviction-step disposition,
//! state-loss-guard, and escalation labels are introduced here, and they are
//! bounded explanatory tokens that resolve back to the runtime contracts at
//! [`RUNTIME_STORAGE_CLASSES_REF`] and [`RUNTIME_LOW_DISK_DRILLS_REF`].
//!
//! ## What this owns
//!
//! - The [`StoragePressureBanner`] record — one pressure event under
//!   disclosure, carrying its pressure class and source, the deepest ladder
//!   step reached, the paused background work, the full eviction order, the
//!   protected classes left untrimmed, the per-class no-state-loss guards, the
//!   escalation posture, and the open-inspector / open-review actions. Mirrors
//!   the boundary schema at [`M5_STORAGE_PRESSURE_SCHEMA_REF`].
//! - The [`EvictionOrderStep`] row — one step in the frozen low-disk ladder
//!   with its 1-based order, disposition, target class, whether it was applied
//!   at this pressure, and whether it is protected and requires reviewed
//!   escalation.
//! - The [`StateLossGuard`] row — one per-class guarantee that pressure freed
//!   only disposable, rebuildable, or unpinned-past-retention bytes and never
//!   authoritative recovery or referenced evidence state.
//! - The [`StoragePressureBannerCorpus`] container — folds every seeded
//!   scenario banner into one validated bundle, checks the cross-record safety
//!   contract (frozen eviction order, protected-class invariants, no
//!   authoritative state loss), and projects a metadata-safe
//!   [`StoragePressureBannerSupportExport`] the support-bundle pipeline can
//!   quote without leaking raw payloads, paths, or credentials.
//! - The [`compose_banner`] projection — the first real consumer: it folds the
//!   frozen [`M5ArtifactFamilyStorageMatrix`] plus a [`PressureSignal`] into a
//!   banner that is correct by construction (eviction order follows the frozen
//!   sequence, user-owned recovery state is never auto-trimmed, pinned and
//!   in-window evidence is retained, and no path reports authoritative loss).
//!
//! ## What this does NOT own
//!
//! - Live byte-level eviction, prefetch suspension, or quota enforcement. Those
//!   belong to the runtime crates; this module is the shared truth model the
//!   low-disk banner, storage inspector, clear-data review, and support export
//!   project. A banner describes a *disclosed* pressure response; scheduling the
//!   trim and emitting the cleanup receipt is a sibling lane.
//! - The runtime storage-class vocabulary, the artifact-family matrix, or the
//!   clear-data review sheet, which stay frozen in their own lanes.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_storage_governance::{LowDiskLadderStep, M5ArtifactFamilyStorageMatrix};
use crate::storage_inspector::StorageClassId;

#[cfg(test)]
mod tests;

/// Frozen schema version shared by every record in this module.
pub const M5_STORAGE_PRESSURE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for a storage-pressure banner.
pub const M5_STORAGE_PRESSURE_BANNER_RECORD_KIND: &str = "m5_storage_pressure_banner";

/// Stable record-kind tag for one eviction-order step.
pub const M5_STORAGE_PRESSURE_EVICTION_STEP_RECORD_KIND: &str = "m5_storage_pressure_eviction_step";

/// Stable record-kind tag for one no-state-loss guard.
pub const M5_STORAGE_PRESSURE_GUARD_RECORD_KIND: &str = "m5_storage_pressure_state_loss_guard";

/// Stable record-kind tag for the support-export envelope.
pub const M5_STORAGE_PRESSURE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_storage_pressure_support_export";

/// Stable record-kind tag for one support-export row.
pub const M5_STORAGE_PRESSURE_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "m5_storage_pressure_support_export_row";

/// Repository-relative path of the boundary schema for the banner.
pub const M5_STORAGE_PRESSURE_SCHEMA_REF: &str = "schemas/storage/m5_storage_pressure.schema.json";

/// Repository-relative path of the reviewer contract doc every banner quotes.
pub const M5_STORAGE_PRESSURE_DOC_REF: &str = "docs/storage/m5_storage_pressure_contract.md";

/// Repository-relative path of the canonical runtime storage-class contract.
pub const RUNTIME_STORAGE_CLASSES_REF: &str = "artifacts/runtime/storage_classes.yaml";

/// Repository-relative path of the frozen low-disk drill sequence.
pub const RUNTIME_LOW_DISK_DRILLS_REF: &str = "artifacts/runtime/low_disk_drills.yaml";

/// The metadata-safe redaction class every banner and export envelope carries.
pub const METADATA_SAFE_DEFAULT: &str = "metadata_safe_default";

/// The stable action id that opens the storage inspector from a banner.
pub const OPEN_STORAGE_INSPECTOR_ACTION_REF: &str = "action.storage.open_inspector";

/// The stable action id that opens the class-selective clear-data review.
pub const OPEN_CLEAR_DATA_REVIEW_ACTION_REF: &str = "action.storage.open_clear_data_review";

// --------------------------------------------------------------------------
// Closed vocabularies introduced by this lane.
//
// pressure_class re-exports the runtime governor health-state tiers that map
// to the low-disk floors. pressure_source, paused_work, eviction-step
// disposition, state-loss-guard, and escalation are bounded explanatory
// labels resolved against the runtime contracts.
// --------------------------------------------------------------------------

/// Severity tier of the pressure, re-exported from the runtime governor
/// health-state tiers that drive the low-disk floors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PressureClass {
    /// First low-disk / soft-quota floor: pause speculative work, trim
    /// disposable hot caches.
    Constrained,
    /// Deeper floor: trim rebuildable knowledge caches and unpinned artifact /
    /// prebuild caches.
    Degraded,
    /// Lowest floor: expire unpinned evidence past retention; authoritative
    /// recovery state is still never auto-trimmed.
    ProtectCore,
}

impl PressureClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Constrained => "constrained",
            Self::Degraded => "degraded",
            Self::ProtectCore => "protect_core",
        }
    }

    /// 1-based ladder order of the deepest step this tier may auto-apply. No
    /// tier ever reaches the user-owned recovery step (order 8) automatically.
    pub const fn max_auto_ladder_order(self) -> u32 {
        match self {
            Self::Constrained => 3,
            Self::Degraded => 6,
            Self::ProtectCore => 7,
        }
    }
}

/// What is exerting the pressure, resolved against the runtime quota-basis and
/// low-disk-floor vocabularies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PressureSourceClass {
    /// Device free-space floor breach (`global_device_quota` / low-disk floor).
    LowDiskFloor,
    /// Tenant- or admin-managed storage quota ceiling.
    ManagedTenantQuota,
    /// Per-workspace storage quota ceiling.
    PerWorkspaceQuota,
    /// A per-class storage ceiling for one derived-data class.
    PerClassCeiling,
}

impl PressureSourceClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LowDiskFloor => "low_disk_floor",
            Self::ManagedTenantQuota => "managed_tenant_quota",
            Self::PerWorkspaceQuota => "per_workspace_quota",
            Self::PerClassCeiling => "per_class_ceiling",
        }
    }

    /// True for managed (admin / tenant) pressure, which must never satisfy a
    /// ceiling by silently deleting local user-owned state.
    pub const fn is_managed(self) -> bool {
        matches!(self, Self::ManagedTenantQuota)
    }
}

/// Background work paused under pressure before any deletion. Pausing is
/// always preferred to trimming; these lanes are foreground-safe to defer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PausedWorkClass {
    /// Speculative fetch and prefetch (ladder step 1).
    SpeculativeFetchAndPrefetch,
    /// Managed replication and pack refresh (ladder step 2).
    ManagedReplicationAndPackRefresh,
    /// Provider model / index refresh.
    ProviderRefresh,
    /// AI context-window expansion and warmup.
    AiContextExpansion,
    /// Extension background timers.
    ExtensionTimer,
    /// Telemetry and diagnostics forwarding.
    TelemetryForward,
}

impl PausedWorkClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SpeculativeFetchAndPrefetch => "speculative_fetch_and_prefetch",
            Self::ManagedReplicationAndPackRefresh => "managed_replication_and_pack_refresh",
            Self::ProviderRefresh => "provider_refresh",
            Self::AiContextExpansion => "ai_context_expansion",
            Self::ExtensionTimer => "extension_timer",
            Self::TelemetryForward => "telemetry_forward",
        }
    }
}

/// The default set of background lanes paused under any pressure tier, in a
/// stable, inspectable order.
const DEFAULT_PAUSED_WORK: &[PausedWorkClass] = &[
    PausedWorkClass::SpeculativeFetchAndPrefetch,
    PausedWorkClass::ManagedReplicationAndPackRefresh,
    PausedWorkClass::ProviderRefresh,
    PausedWorkClass::AiContextExpansion,
    PausedWorkClass::ExtensionTimer,
    PausedWorkClass::TelemetryForward,
];

/// What one eviction step does to its target class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvictionStepDispositionClass {
    /// Pause background work; delete nothing.
    PauseBackgroundWorkNoDeletion,
    /// Trim a disposable cache that rebuilds within one interaction.
    TrimDisposableCache,
    /// Trim a rebuildable derived cache; the class is left rebuild-pending.
    TrimRebuildableCache,
    /// Trim only unpinned entries; pinned entries are retained.
    TrimUnpinnedArtifact,
    /// Expire only unpinned evidence past its retention window; pinned and
    /// in-window evidence is retained.
    ExpireUnpinnedEvidencePastRetentionOnly,
    /// Protected class that is never trimmed automatically; removal requires a
    /// reviewed, class-specific escalation.
    ProtectedNeverAutoRequiresReviewedEscalation,
}

impl EvictionStepDispositionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PauseBackgroundWorkNoDeletion => "pause_background_work_no_deletion",
            Self::TrimDisposableCache => "trim_disposable_cache",
            Self::TrimRebuildableCache => "trim_rebuildable_cache",
            Self::TrimUnpinnedArtifact => "trim_unpinned_artifact",
            Self::ExpireUnpinnedEvidencePastRetentionOnly => {
                "expire_unpinned_evidence_past_retention_only"
            }
            Self::ProtectedNeverAutoRequiresReviewedEscalation => {
                "protected_never_auto_requires_reviewed_escalation"
            }
        }
    }

    /// True when this disposition deletes nothing authoritative — disposable,
    /// rebuildable, unpinned, or unpinned-past-retention only.
    pub const fn is_non_authoritative(self) -> bool {
        !matches!(self, Self::ProtectedNeverAutoRequiresReviewedEscalation)
    }
}

/// The per-class no-authoritative-state-loss guarantee a banner carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateLossGuardClass {
    /// Disposable cache trimmed; no authoritative loss, rebuilds on demand.
    DisposableTrimmedNoAuthoritativeLoss,
    /// Rebuildable cache trimmed; class left rebuild-pending.
    RebuildableTrimmedRebuildPending,
    /// Unpinned artifact / prebuild trimmed; pinned entries retained.
    UnpinnedArtifactTrimmedPinnedRetained,
    /// Unpinned evidence past retention expired; pinned and in-window evidence
    /// retained.
    UnpinnedEvidenceExpiredPinnedAndInWindowRetained,
    /// Protected evidence fully retained; no eviction touched it.
    ProtectedEvidenceFullyRetained,
    /// User-owned recovery state never auto-trimmed under pressure.
    UserOwnedRecoveryStateNeverAutoTrimmed,
    /// Pressure could not be satisfied without protected state; a reviewed
    /// escalation is required and nothing was auto-deleted.
    EscalationRequiredNotAutoApplied,
}

impl StateLossGuardClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisposableTrimmedNoAuthoritativeLoss => {
                "disposable_trimmed_no_authoritative_loss"
            }
            Self::RebuildableTrimmedRebuildPending => "rebuildable_trimmed_rebuild_pending",
            Self::UnpinnedArtifactTrimmedPinnedRetained => {
                "unpinned_artifact_trimmed_pinned_retained"
            }
            Self::UnpinnedEvidenceExpiredPinnedAndInWindowRetained => {
                "unpinned_evidence_expired_pinned_and_in_window_retained"
            }
            Self::ProtectedEvidenceFullyRetained => "protected_evidence_fully_retained",
            Self::UserOwnedRecoveryStateNeverAutoTrimmed => {
                "user_owned_recovery_state_never_auto_trimmed"
            }
            Self::EscalationRequiredNotAutoApplied => "escalation_required_not_auto_applied",
        }
    }

    /// True when the guard guarantees authoritative state was retained.
    pub const fn protects_authoritative_state(self) -> bool {
        matches!(
            self,
            Self::UnpinnedEvidenceExpiredPinnedAndInWindowRetained
                | Self::ProtectedEvidenceFullyRetained
                | Self::UserOwnedRecoveryStateNeverAutoTrimmed
                | Self::EscalationRequiredNotAutoApplied
        )
    }
}

/// Whether a reviewed escalation is needed to free protected state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EscalationStateClass {
    /// Pressure was absorbed by disposable / rebuildable / unpinned bytes.
    NoEscalationNeeded,
    /// Only protected state remains over the ceiling; a class-specific review
    /// is required and has not yet been approved. Nothing is auto-deleted.
    ReviewedEscalationRequiredNotYetApproved,
    /// A class-specific review has been approved for the named protected state.
    ReviewedEscalationApproved,
}

impl EscalationStateClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoEscalationNeeded => "no_escalation_needed",
            Self::ReviewedEscalationRequiredNotYetApproved => {
                "reviewed_escalation_required_not_yet_approved"
            }
            Self::ReviewedEscalationApproved => "reviewed_escalation_approved",
        }
    }
}

// --------------------------------------------------------------------------
// Ladder helpers — the frozen mapping from each step to its target class and
// disposition. These resolve the runtime low_disk_ladder_step vocabulary; they
// mint nothing new.
// --------------------------------------------------------------------------

/// The frozen low-disk ladder, early → late, exactly as ordered by the runtime
/// contract.
const FROZEN_LADDER: &[LowDiskLadderStep] = &[
    LowDiskLadderStep::StopSpeculativeFetchAndPrefetch,
    LowDiskLadderStep::PauseManagedReplicationAndPackRefresh,
    LowDiskLadderStep::TrimInteractiveHotCache,
    LowDiskLadderStep::TrimKnowledgeCacheRebuildable,
    LowDiskLadderStep::TrimArtifactCacheUnpinned,
    LowDiskLadderStep::TrimPrebuildEnvironmentUnpinned,
    LowDiskLadderStep::ExpireUnpinnedEvidencePastRetention,
    LowDiskLadderStep::UserOwnedRecoveryStateOnlyUnderExplicitReview,
];

/// The storage class one ladder step targets, or `None` for the pause steps.
const fn step_target_class(step: LowDiskLadderStep) -> Option<StorageClassId> {
    match step {
        LowDiskLadderStep::StopSpeculativeFetchAndPrefetch
        | LowDiskLadderStep::PauseManagedReplicationAndPackRefresh => None,
        LowDiskLadderStep::TrimInteractiveHotCache => Some(StorageClassId::InteractiveHotCache),
        LowDiskLadderStep::TrimKnowledgeCacheRebuildable => Some(StorageClassId::KnowledgeCache),
        LowDiskLadderStep::TrimArtifactCacheUnpinned => Some(StorageClassId::ArtifactCache),
        LowDiskLadderStep::TrimPrebuildEnvironmentUnpinned => {
            Some(StorageClassId::PrebuildEnvironmentCache)
        }
        LowDiskLadderStep::ExpireUnpinnedEvidencePastRetention => {
            Some(StorageClassId::EvidenceSupportCache)
        }
        LowDiskLadderStep::UserOwnedRecoveryStateOnlyUnderExplicitReview => {
            Some(StorageClassId::UserOwnedRecoveryState)
        }
    }
}

/// The disposition of one ladder step.
const fn step_disposition(step: LowDiskLadderStep) -> EvictionStepDispositionClass {
    match step {
        LowDiskLadderStep::StopSpeculativeFetchAndPrefetch
        | LowDiskLadderStep::PauseManagedReplicationAndPackRefresh => {
            EvictionStepDispositionClass::PauseBackgroundWorkNoDeletion
        }
        LowDiskLadderStep::TrimInteractiveHotCache => {
            EvictionStepDispositionClass::TrimDisposableCache
        }
        LowDiskLadderStep::TrimKnowledgeCacheRebuildable => {
            EvictionStepDispositionClass::TrimRebuildableCache
        }
        LowDiskLadderStep::TrimArtifactCacheUnpinned
        | LowDiskLadderStep::TrimPrebuildEnvironmentUnpinned => {
            EvictionStepDispositionClass::TrimUnpinnedArtifact
        }
        LowDiskLadderStep::ExpireUnpinnedEvidencePastRetention => {
            EvictionStepDispositionClass::ExpireUnpinnedEvidencePastRetentionOnly
        }
        LowDiskLadderStep::UserOwnedRecoveryStateOnlyUnderExplicitReview => {
            EvictionStepDispositionClass::ProtectedNeverAutoRequiresReviewedEscalation
        }
    }
}

/// A stable, human-readable label for one ladder step.
const fn step_label(step: LowDiskLadderStep) -> &'static str {
    match step {
        LowDiskLadderStep::StopSpeculativeFetchAndPrefetch => {
            "Pause speculative fetch and prefetch"
        }
        LowDiskLadderStep::PauseManagedReplicationAndPackRefresh => {
            "Pause managed replication and pack refresh"
        }
        LowDiskLadderStep::TrimInteractiveHotCache => "Trim interactive hot cache",
        LowDiskLadderStep::TrimKnowledgeCacheRebuildable => "Trim rebuildable knowledge cache",
        LowDiskLadderStep::TrimArtifactCacheUnpinned => "Trim unpinned artifact cache",
        LowDiskLadderStep::TrimPrebuildEnvironmentUnpinned => "Trim unpinned prebuild environment",
        LowDiskLadderStep::ExpireUnpinnedEvidencePastRetention => {
            "Expire unpinned evidence past retention"
        }
        LowDiskLadderStep::UserOwnedRecoveryStateOnlyUnderExplicitReview => {
            "User-owned recovery state — explicit review only"
        }
    }
}

/// True for the protected storage classes — evidence and user-owned recovery.
const fn is_protected_class(class_id: StorageClassId) -> bool {
    matches!(
        class_id,
        StorageClassId::EvidenceSupportCache | StorageClassId::UserOwnedRecoveryState
    )
}

// --------------------------------------------------------------------------
// Banner records.
// --------------------------------------------------------------------------

/// One step in the disclosed eviction order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvictionOrderStep {
    pub record_kind: String,
    pub ladder_step: LowDiskLadderStep,
    pub ladder_order: u32,
    pub disposition: EvictionStepDispositionClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_class_id: Option<StorageClassId>,
    /// True when this step was applied automatically at the banner's pressure.
    pub applied: bool,
    /// True when this step targets a protected class (evidence / recovery).
    pub protected: bool,
    /// True when removing this step's bytes requires a reviewed escalation.
    pub requires_reviewed_escalation: bool,
    pub label: String,
}

/// One per-class no-authoritative-state-loss guard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateLossGuard {
    pub record_kind: String,
    pub class_id: StorageClassId,
    pub guard_class: StateLossGuardClass,
    /// Bytes freed under this guard — disposable / rebuildable / unpinned only.
    pub reclaimed_bytes: u64,
    /// Bytes retained under this guard — pinned, in-window, or authoritative.
    pub retained_bytes: u64,
    /// True when the guard holds: no authoritative state was lost.
    pub holds: bool,
    pub detail: String,
}

impl StateLossGuard {
    /// True when this guard covers a protected class — evidence or user-owned
    /// recovery state.
    pub const fn is_protected_class(&self) -> bool {
        is_protected_class(self.class_id)
    }
}

/// One named workspace or scope the banner applies to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeRef {
    pub scope_ref: String,
    pub label: String,
}

/// A storage-pressure banner: the disclosed response to one low-disk or quota
/// pressure event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoragePressureBanner {
    pub record_kind: String,
    pub schema_version: u32,
    pub banner_id: String,
    pub emitted_at: String,
    pub headline: String,
    pub pressure_class: PressureClass,
    pub pressure_source: PressureSourceClass,
    pub scope_ref: String,
    pub scope_label: String,
    /// The deepest ladder step applied at this pressure.
    pub current_ladder_step: LowDiskLadderStep,
    pub current_ladder_order: u32,
    /// Background work paused before any deletion.
    pub paused_work: Vec<PausedWorkClass>,
    /// The full frozen eviction order, early → late.
    pub eviction_order: Vec<EvictionOrderStep>,
    /// Classes left untrimmed under this pressure.
    pub protected_class_ids_not_trimmed: Vec<StorageClassId>,
    /// Per-class no-authoritative-state-loss guards.
    pub state_loss_guards: Vec<StateLossGuard>,
    pub escalation_state: EscalationStateClass,
    /// Always false: pressure never causes authoritative state loss.
    pub authoritative_state_loss: bool,
    pub open_inspector_action_ref: String,
    pub open_clear_data_review_action_ref: String,
    #[serde(default)]
    pub guardrail_notices: Vec<String>,
    pub schema_ref: String,
    pub doc_ref: String,
}

impl StoragePressureBanner {
    /// Returns the eviction step for the given ladder step, if present.
    pub fn step(&self, ladder_step: LowDiskLadderStep) -> Option<&EvictionOrderStep> {
        self.eviction_order
            .iter()
            .find(|step| step.ladder_step == ladder_step)
    }

    /// Returns the guard for the given storage class, if present.
    pub fn guard(&self, class_id: StorageClassId) -> Option<&StateLossGuard> {
        self.state_loss_guards
            .iter()
            .find(|guard| guard.class_id == class_id)
    }

    /// Total bytes reclaimed across every guard.
    pub fn total_reclaimed_bytes(&self) -> u64 {
        self.state_loss_guards
            .iter()
            .fold(0u64, |acc, guard| acc.saturating_add(guard.reclaimed_bytes))
    }

    /// True when the banner is export-safe and every guard holds with no
    /// authoritative state loss.
    pub fn is_export_safe(&self) -> bool {
        !self.authoritative_state_loss && self.state_loss_guards.iter().all(|guard| guard.holds)
    }

    /// Validates this banner against the frozen-ordering and protected-class
    /// safety contract, attributing each violation to `target_ref`.
    pub fn validate_into(&self, violations: &mut Vec<StoragePressureViolation>, target_ref: &str) {
        let target = target_ref;

        if self.schema_version != M5_STORAGE_PRESSURE_SCHEMA_VERSION {
            push(
                violations,
                "banner.schema_version",
                target,
                "schema_version must be 1",
            );
        }
        if self.record_kind != M5_STORAGE_PRESSURE_BANNER_RECORD_KIND {
            push(
                violations,
                "banner.record_kind",
                target,
                "record_kind must be m5_storage_pressure_banner",
            );
        }
        if self.schema_ref != M5_STORAGE_PRESSURE_SCHEMA_REF {
            push(
                violations,
                "banner.schema_ref",
                target,
                "schema_ref must pin the boundary schema",
            );
        }
        if self.doc_ref != M5_STORAGE_PRESSURE_DOC_REF {
            push(
                violations,
                "banner.doc_ref",
                target,
                "doc_ref must pin the contract doc",
            );
        }
        if self.banner_id.trim().is_empty() {
            push(
                violations,
                "banner.banner_id",
                target,
                "banner_id must be non-empty",
            );
        }
        if self.headline.trim().is_empty() {
            push(
                violations,
                "banner.headline",
                target,
                "headline must be non-empty",
            );
        }
        if self.open_inspector_action_ref != OPEN_STORAGE_INSPECTOR_ACTION_REF {
            push(
                violations,
                "banner.open_inspector_action_ref",
                target,
                "open_inspector_action_ref must offer the inspector action",
            );
        }
        if self.open_clear_data_review_action_ref != OPEN_CLEAR_DATA_REVIEW_ACTION_REF {
            push(
                violations,
                "banner.open_clear_data_review_action_ref",
                target,
                "open_clear_data_review_action_ref must offer the review action",
            );
        }

        // Authoritative state loss is never admissible.
        if self.authoritative_state_loss {
            push(
                violations,
                "banner.authoritative_state_loss",
                target,
                "authoritative_state_loss must be false",
            );
        }

        self.validate_eviction_order(violations, target);
        self.validate_paused_work(violations, target);
        self.validate_protected_classes(violations, target);
        self.validate_guards(violations, target);
    }

    /// The eviction order MUST be the full frozen ladder, in order, with the
    /// derived disposition / target / protection for each step.
    fn validate_eviction_order(
        &self,
        violations: &mut Vec<StoragePressureViolation>,
        target: &str,
    ) {
        if self.eviction_order.len() != FROZEN_LADDER.len() {
            push(
                violations,
                "banner.eviction_order.length",
                target,
                "eviction_order must list every frozen ladder step exactly once",
            );
            return;
        }

        let mut deepest_applied: Option<u32> = None;
        for (index, step) in self.eviction_order.iter().enumerate() {
            let expected = FROZEN_LADDER[index];
            if step.ladder_step != expected {
                push(
                    violations,
                    "banner.eviction_order.sequence",
                    target,
                    "eviction_order must follow the frozen low-disk sequence",
                );
            }
            if step.record_kind != M5_STORAGE_PRESSURE_EVICTION_STEP_RECORD_KIND {
                push(
                    violations,
                    "banner.eviction_order.record_kind",
                    target,
                    "step record_kind must be m5_storage_pressure_eviction_step",
                );
            }
            if step.ladder_order != step.ladder_step.ladder_order() {
                push(
                    violations,
                    "banner.eviction_order.ladder_order",
                    target,
                    "step ladder_order must equal its frozen position",
                );
            }
            if step.disposition != step_disposition(step.ladder_step) {
                push(
                    violations,
                    "banner.eviction_order.disposition",
                    target,
                    "step disposition must match the frozen ladder disposition",
                );
            }
            if step.target_class_id != step_target_class(step.ladder_step) {
                push(
                    violations,
                    "banner.eviction_order.target_class",
                    target,
                    "step target_class_id must match the frozen ladder target",
                );
            }
            let expected_protected = step
                .target_class_id
                .map(is_protected_class)
                .unwrap_or(false);
            if step.protected != expected_protected {
                push(
                    violations,
                    "banner.eviction_order.protected_flag",
                    target,
                    "step protected flag must track its target class",
                );
            }
            // The user-owned recovery step is the only step that requires a
            // reviewed escalation, and it is NEVER applied automatically.
            let is_recovery_step = step.ladder_step
                == LowDiskLadderStep::UserOwnedRecoveryStateOnlyUnderExplicitReview;
            if step.requires_reviewed_escalation != is_recovery_step {
                push(
                    violations,
                    "banner.eviction_order.escalation_flag",
                    target,
                    "only the user-owned recovery step may require reviewed escalation",
                );
            }
            if is_recovery_step && step.applied {
                push(
                    violations,
                    "banner.eviction_order.recovery_auto_applied",
                    target,
                    "user-owned recovery state must never be auto-applied under pressure",
                );
            }
            // Applied steps form a contiguous prefix bounded by the pressure
            // tier; nothing past the tier's auto ceiling may be applied.
            if step.applied {
                if step.ladder_order > self.pressure_class.max_auto_ladder_order() {
                    push(
                        violations,
                        "banner.eviction_order.applied_past_tier",
                        target,
                        "no step past the pressure tier ceiling may be applied",
                    );
                }
                deepest_applied = Some(match deepest_applied {
                    Some(prev) => prev.max(step.ladder_order),
                    None => step.ladder_order,
                });
            }
        }

        // current_ladder_step must equal the deepest applied step.
        if let Some(order) = deepest_applied {
            if self.current_ladder_order != order {
                push(
                    violations,
                    "banner.current_ladder_step",
                    target,
                    "current_ladder_step must equal the deepest applied step",
                );
            }
        }
        if self.current_ladder_order != self.current_ladder_step.ladder_order() {
            push(
                violations,
                "banner.current_ladder_order",
                target,
                "current_ladder_order must equal current_ladder_step order",
            );
        }
    }

    /// At least the two pause steps must be disclosed as paused work, and the
    /// list must be free of duplicates.
    fn validate_paused_work(&self, violations: &mut Vec<StoragePressureViolation>, target: &str) {
        let mut seen: BTreeSet<PausedWorkClass> = BTreeSet::new();
        for lane in &self.paused_work {
            if !seen.insert(*lane) {
                push(
                    violations,
                    "banner.paused_work.duplicate",
                    target,
                    "paused_work must not repeat a lane",
                );
            }
        }
        if !seen.contains(&PausedWorkClass::SpeculativeFetchAndPrefetch)
            || !seen.contains(&PausedWorkClass::ManagedReplicationAndPackRefresh)
        {
            push(
                violations,
                "banner.paused_work.pause_steps",
                target,
                "paused_work must disclose the two ladder pause steps",
            );
        }
    }

    /// User-owned recovery state must always appear among the untrimmed
    /// protected classes, and the list must be free of duplicates.
    fn validate_protected_classes(
        &self,
        violations: &mut Vec<StoragePressureViolation>,
        target: &str,
    ) {
        let mut seen: BTreeSet<StorageClassId> = BTreeSet::new();
        for class_id in &self.protected_class_ids_not_trimmed {
            if !seen.insert(*class_id) {
                push(
                    violations,
                    "banner.protected_classes.duplicate",
                    target,
                    "protected_class_ids_not_trimmed must not repeat a class",
                );
            }
            if !is_protected_class(*class_id) {
                push(
                    violations,
                    "banner.protected_classes.non_protected",
                    target,
                    "only protected classes may be listed as not trimmed",
                );
            }
        }
        if !seen.contains(&StorageClassId::UserOwnedRecoveryState) {
            push(
                violations,
                "banner.protected_classes.recovery_missing",
                target,
                "user-owned recovery state must always be listed as not trimmed",
            );
        }
    }

    /// Every storage class must carry exactly one guard, the recovery-state
    /// guard must reclaim zero bytes, and every guard must hold.
    fn validate_guards(&self, violations: &mut Vec<StoragePressureViolation>, target: &str) {
        let mut seen: BTreeSet<StorageClassId> = BTreeSet::new();
        for guard in &self.state_loss_guards {
            if guard.record_kind != M5_STORAGE_PRESSURE_GUARD_RECORD_KIND {
                push(
                    violations,
                    "banner.guard.record_kind",
                    target,
                    "guard record_kind must be m5_storage_pressure_state_loss_guard",
                );
            }
            if !seen.insert(guard.class_id) {
                push(
                    violations,
                    "banner.guard.duplicate",
                    target,
                    "each storage class may carry at most one guard",
                );
            }
            if !guard.holds {
                push(
                    violations,
                    "banner.guard.does_not_hold",
                    target,
                    "every state-loss guard must hold",
                );
            }
            // User-owned recovery state is never reduced by pressure — full stop.
            if guard.class_id == StorageClassId::UserOwnedRecoveryState
                && guard.reclaimed_bytes != 0
            {
                push(
                    violations,
                    "banner.guard.recovery_reclaim",
                    target,
                    "user-owned recovery state must reclaim zero bytes under pressure",
                );
            }
            if guard.class_id == StorageClassId::UserOwnedRecoveryState
                && !matches!(
                    guard.guard_class,
                    StateLossGuardClass::UserOwnedRecoveryStateNeverAutoTrimmed
                        | StateLossGuardClass::EscalationRequiredNotAutoApplied
                )
            {
                push(
                    violations,
                    "banner.guard.recovery_class",
                    target,
                    "recovery-state guard must be a never-auto-trim or escalation guard",
                );
            }
            // Evidence retains pinned / in-window bytes regardless of tier.
            if guard.class_id == StorageClassId::EvidenceSupportCache
                && self.pressure_class != PressureClass::ProtectCore
                && guard.reclaimed_bytes != 0
            {
                push(
                    violations,
                    "banner.guard.evidence_reclaim",
                    target,
                    "evidence is only expired past retention at protect_core",
                );
            }
        }
        // Every storage class must be represented.
        for class_id in ALL_STORAGE_CLASSES {
            if !seen.contains(class_id) {
                push(
                    violations,
                    "banner.guard.class_missing",
                    target,
                    "every storage class must carry a state-loss guard",
                );
            }
        }

        // An unapproved escalation must never coexist with reclaimed protected
        // bytes; an escalation requirement implies nothing was auto-removed.
        if self.escalation_state == EscalationStateClass::ReviewedEscalationRequiredNotYetApproved {
            let protected_reclaimed = self
                .state_loss_guards
                .iter()
                .filter(|guard| guard.is_protected_class())
                .any(|guard| guard.reclaimed_bytes != 0);
            if protected_reclaimed {
                push(
                    violations,
                    "banner.escalation.protected_reclaimed",
                    target,
                    "a pending escalation must not have reclaimed protected bytes",
                );
            }
        }
    }
}

/// The closed set of storage classes every banner's guards must cover.
const ALL_STORAGE_CLASSES: &[StorageClassId] = &[
    StorageClassId::InteractiveHotCache,
    StorageClassId::KnowledgeCache,
    StorageClassId::ArtifactCache,
    StorageClassId::PrebuildEnvironmentCache,
    StorageClassId::EvidenceSupportCache,
    StorageClassId::UserOwnedRecoveryState,
];

// --------------------------------------------------------------------------
// Matrix-backed composer — the first real consumer.
// --------------------------------------------------------------------------

/// One class's observed byte facts the inspector measured for the composer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassObservation {
    pub class_id: StorageClassId,
    /// Reclaimable unpinned bytes (disposable / rebuildable / unpinned).
    #[serde(default)]
    pub reclaimable_unpinned_bytes: u64,
    /// Pinned or in-window bytes that must be retained.
    #[serde(default)]
    pub pinned_or_in_window_bytes: u64,
    /// For evidence only: unpinned bytes already past their retention window.
    #[serde(default)]
    pub unpinned_past_retention_bytes: u64,
}

/// The pressure signal [`compose_banner`] folds into a banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PressureSignal {
    pub banner_id: String,
    pub emitted_at: String,
    pub pressure_class: PressureClass,
    pub pressure_source: PressureSourceClass,
    pub scope_ref: String,
    pub scope_label: String,
    /// Per-class observed byte facts; classes omitted default to zero.
    #[serde(default)]
    pub observations: Vec<ClassObservation>,
    /// True when only protected state remains over the ceiling: the banner
    /// requests a reviewed escalation rather than auto-deleting anything.
    #[serde(default)]
    pub only_protected_over_ceiling: bool,
}

impl PressureSignal {
    fn observation(&self, class_id: StorageClassId) -> ClassObservation {
        self.observations
            .iter()
            .copied()
            .find(|obs| obs.class_id == class_id)
            .unwrap_or(ClassObservation {
                class_id,
                reclaimable_unpinned_bytes: 0,
                pinned_or_in_window_bytes: 0,
                unpinned_past_retention_bytes: 0,
            })
    }
}

/// Folds the frozen artifact-family matrix plus a pressure signal into a banner
/// that is correct by construction: the eviction order follows the frozen
/// sequence, user-owned recovery state is never auto-trimmed, pinned and
/// in-window evidence is retained, and no path reports authoritative loss.
///
/// The `matrix` argument anchors the banner to the same frozen low-disk ladder
/// the storage-governance lane validates; the composer reads no private
/// ordering of its own.
pub fn compose_banner(
    matrix: &M5ArtifactFamilyStorageMatrix,
    signal: &PressureSignal,
) -> StoragePressureBanner {
    let max_auto = signal.pressure_class.max_auto_ladder_order();

    // The protected classes a banner leaves untrimmed are read from the frozen
    // matrix's own protected-continuity rows, projected onto the matrix's
    // low-disk eviction order — the banner mints no private protection set.
    let matrix_protected: BTreeSet<StorageClassId> = matrix
        .low_disk_eviction_order()
        .iter()
        .filter(|row| row.protected_continuity)
        .map(|row| row.storage_class_id)
        .collect();

    let mut eviction_order = Vec::with_capacity(FROZEN_LADDER.len());
    let mut deepest_applied = FROZEN_LADDER[0];
    for &step in FROZEN_LADDER {
        let order = step.ladder_order();
        let target = step_target_class(step);
        let protected = target
            .map(|class| matrix_protected.contains(&class))
            .unwrap_or(false);
        let is_recovery_step =
            step == LowDiskLadderStep::UserOwnedRecoveryStateOnlyUnderExplicitReview;
        // A step applies when it sits within the tier ceiling and is not the
        // user-owned recovery step, which only ever moves under explicit review.
        let applied = order <= max_auto && !is_recovery_step;
        if applied {
            deepest_applied = step;
        }
        eviction_order.push(EvictionOrderStep {
            record_kind: M5_STORAGE_PRESSURE_EVICTION_STEP_RECORD_KIND.to_owned(),
            ladder_step: step,
            ladder_order: order,
            disposition: step_disposition(step),
            target_class_id: target,
            applied,
            protected,
            requires_reviewed_escalation: is_recovery_step,
            label: step_label(step).to_owned(),
        });
    }

    // Protected classes left untrimmed: every matrix-protected class except the
    // evidence class once the tier reaches the evidence-expiry step (which only
    // expires unpinned-past-retention entries). User-owned recovery is always
    // retained because its step never auto-applies.
    let evidence_expiry_reached =
        max_auto >= LowDiskLadderStep::ExpireUnpinnedEvidencePastRetention.ladder_order();
    let mut protected_not_trimmed: Vec<StorageClassId> = matrix_protected
        .iter()
        .copied()
        .filter(|class| {
            !(evidence_expiry_reached && *class == StorageClassId::EvidenceSupportCache)
        })
        .collect();
    protected_not_trimmed.sort_by(|a, b| a.as_str().cmp(b.as_str()));

    let escalation_state = if signal.only_protected_over_ceiling {
        EscalationStateClass::ReviewedEscalationRequiredNotYetApproved
    } else {
        EscalationStateClass::NoEscalationNeeded
    };

    let state_loss_guards = ALL_STORAGE_CLASSES
        .iter()
        .map(|&class_id| compose_guard(class_id, signal, max_auto))
        .collect();

    let mut guardrail_notices = Vec::new();
    if signal.pressure_source.is_managed() {
        guardrail_notices.push(
            "Managed quota pressure never deletes local user-owned recovery state; it stays until you explicitly review it.".to_owned(),
        );
    }
    if signal.only_protected_over_ceiling {
        guardrail_notices.push(
            "Only protected state remains over the ceiling. Aureline paused and is asking for a reviewed class-specific decision instead of deleting it.".to_owned(),
        );
    }

    StoragePressureBanner {
        record_kind: M5_STORAGE_PRESSURE_BANNER_RECORD_KIND.to_owned(),
        schema_version: M5_STORAGE_PRESSURE_SCHEMA_VERSION,
        banner_id: signal.banner_id.clone(),
        emitted_at: signal.emitted_at.clone(),
        headline: compose_headline(signal.pressure_class, signal.pressure_source),
        pressure_class: signal.pressure_class,
        pressure_source: signal.pressure_source,
        scope_ref: signal.scope_ref.clone(),
        scope_label: signal.scope_label.clone(),
        current_ladder_step: deepest_applied,
        current_ladder_order: deepest_applied.ladder_order(),
        paused_work: DEFAULT_PAUSED_WORK.to_vec(),
        eviction_order,
        protected_class_ids_not_trimmed: protected_not_trimmed,
        state_loss_guards,
        escalation_state,
        authoritative_state_loss: false,
        open_inspector_action_ref: OPEN_STORAGE_INSPECTOR_ACTION_REF.to_owned(),
        open_clear_data_review_action_ref: OPEN_CLEAR_DATA_REVIEW_ACTION_REF.to_owned(),
        guardrail_notices,
        schema_ref: M5_STORAGE_PRESSURE_SCHEMA_REF.to_owned(),
        doc_ref: M5_STORAGE_PRESSURE_DOC_REF.to_owned(),
    }
}

/// Composes one per-class guard from the signal observation and pressure tier.
fn compose_guard(
    class_id: StorageClassId,
    signal: &PressureSignal,
    max_auto: u32,
) -> StateLossGuard {
    let obs = signal.observation(class_id);
    let order = |step: LowDiskLadderStep| step.ladder_order();
    match class_id {
        StorageClassId::InteractiveHotCache => {
            let applied = max_auto >= order(LowDiskLadderStep::TrimInteractiveHotCache);
            let reclaimed = if applied {
                obs.reclaimable_unpinned_bytes
            } else {
                0
            };
            StateLossGuard {
                record_kind: M5_STORAGE_PRESSURE_GUARD_RECORD_KIND.to_owned(),
                class_id,
                guard_class: StateLossGuardClass::DisposableTrimmedNoAuthoritativeLoss,
                reclaimed_bytes: reclaimed,
                retained_bytes: obs.pinned_or_in_window_bytes,
                holds: true,
                detail: "Disposable hot cache; rebuilds on demand. No authoritative loss."
                    .to_owned(),
            }
        }
        StorageClassId::KnowledgeCache => {
            let applied = max_auto >= order(LowDiskLadderStep::TrimKnowledgeCacheRebuildable);
            let reclaimed = if applied {
                obs.reclaimable_unpinned_bytes
            } else {
                0
            };
            StateLossGuard {
                record_kind: M5_STORAGE_PRESSURE_GUARD_RECORD_KIND.to_owned(),
                class_id,
                guard_class: StateLossGuardClass::RebuildableTrimmedRebuildPending,
                reclaimed_bytes: reclaimed,
                retained_bytes: obs.pinned_or_in_window_bytes,
                holds: true,
                detail: "Rebuildable index; left rebuild-pending. Pinned corpora retained."
                    .to_owned(),
            }
        }
        StorageClassId::ArtifactCache => {
            let applied = max_auto >= order(LowDiskLadderStep::TrimArtifactCacheUnpinned);
            let reclaimed = if applied {
                obs.reclaimable_unpinned_bytes
            } else {
                0
            };
            StateLossGuard {
                record_kind: M5_STORAGE_PRESSURE_GUARD_RECORD_KIND.to_owned(),
                class_id,
                guard_class: StateLossGuardClass::UnpinnedArtifactTrimmedPinnedRetained,
                reclaimed_bytes: reclaimed,
                retained_bytes: obs.pinned_or_in_window_bytes,
                holds: true,
                detail:
                    "Unpinned artifacts trimmed; pinned / mirrored / release-ref entries retained."
                        .to_owned(),
            }
        }
        StorageClassId::PrebuildEnvironmentCache => {
            let applied = max_auto >= order(LowDiskLadderStep::TrimPrebuildEnvironmentUnpinned);
            let reclaimed = if applied {
                obs.reclaimable_unpinned_bytes
            } else {
                0
            };
            StateLossGuard {
                record_kind: M5_STORAGE_PRESSURE_GUARD_RECORD_KIND.to_owned(),
                class_id,
                guard_class: StateLossGuardClass::UnpinnedArtifactTrimmedPinnedRetained,
                reclaimed_bytes: reclaimed,
                retained_bytes: obs.pinned_or_in_window_bytes,
                holds: true,
                detail: "Unpinned prebuilds trimmed; offline / certified packs retained. Startup may widen.".to_owned(),
            }
        }
        StorageClassId::EvidenceSupportCache => {
            let expired = max_auto >= order(LowDiskLadderStep::ExpireUnpinnedEvidencePastRetention);
            if expired {
                StateLossGuard {
                    record_kind: M5_STORAGE_PRESSURE_GUARD_RECORD_KIND.to_owned(),
                    class_id,
                    guard_class:
                        StateLossGuardClass::UnpinnedEvidenceExpiredPinnedAndInWindowRetained,
                    reclaimed_bytes: obs.unpinned_past_retention_bytes,
                    retained_bytes: obs.pinned_or_in_window_bytes,
                    holds: true,
                    detail: "Only unpinned evidence past retention expired; pinned and in-window evidence retained.".to_owned(),
                }
            } else {
                StateLossGuard {
                    record_kind: M5_STORAGE_PRESSURE_GUARD_RECORD_KIND.to_owned(),
                    class_id,
                    guard_class: StateLossGuardClass::ProtectedEvidenceFullyRetained,
                    reclaimed_bytes: 0,
                    retained_bytes: obs
                        .pinned_or_in_window_bytes
                        .saturating_add(obs.unpinned_past_retention_bytes),
                    holds: true,
                    detail: "Evidence cache fully retained; not reached by this pressure tier."
                        .to_owned(),
                }
            }
        }
        StorageClassId::UserOwnedRecoveryState => {
            let guard_class = if signal.only_protected_over_ceiling {
                StateLossGuardClass::EscalationRequiredNotAutoApplied
            } else {
                StateLossGuardClass::UserOwnedRecoveryStateNeverAutoTrimmed
            };
            StateLossGuard {
                record_kind: M5_STORAGE_PRESSURE_GUARD_RECORD_KIND.to_owned(),
                class_id,
                guard_class,
                reclaimed_bytes: 0,
                retained_bytes: obs
                    .pinned_or_in_window_bytes
                    .saturating_add(obs.reclaimable_unpinned_bytes),
                holds: true,
                detail: "User-owned recovery state is never auto-trimmed; removal requires an explicit class-specific review.".to_owned(),
            }
        }
    }
}

/// Composes the stable banner headline for a pressure class and source.
fn compose_headline(pressure_class: PressureClass, source: PressureSourceClass) -> String {
    let where_clause = match source {
        PressureSourceClass::LowDiskFloor => "Low disk space",
        PressureSourceClass::ManagedTenantQuota => "Managed storage quota reached",
        PressureSourceClass::PerWorkspaceQuota => "Workspace storage quota reached",
        PressureSourceClass::PerClassCeiling => "A storage class reached its ceiling",
    };
    let action = match pressure_class {
        PressureClass::Constrained => {
            "Aureline paused background fetches and began trimming disposable caches."
        }
        PressureClass::Degraded => {
            "Aureline paused background work and trimmed rebuildable and unpinned caches."
        }
        PressureClass::ProtectCore => {
            "Aureline trimmed every disposable cache and expired only unpinned evidence past retention. Recovery state is untouched."
        }
    };
    format!("{where_clause}. {action}")
}

// --------------------------------------------------------------------------
// Corpus container, entries, and loaders.
// --------------------------------------------------------------------------

/// One banner fixture paired with its repository-relative path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoragePressureBannerEntry {
    pub fixture_ref: String,
    pub banner: StoragePressureBanner,
}

/// Storage-pressure banner corpus loaded from the checked-in fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoragePressureBannerCorpus {
    pub banners: Vec<StoragePressureBannerEntry>,
}

const BANNER_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/storage/m5_storage_pressure_cases/low_disk_constrained_pauses_then_trims_disposable.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_storage_pressure_cases/low_disk_constrained_pauses_then_trims_disposable.yaml"
        )),
    ),
    (
        "fixtures/storage/m5_storage_pressure_cases/low_disk_degraded_trims_rebuildable_unpinned.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_storage_pressure_cases/low_disk_degraded_trims_rebuildable_unpinned.yaml"
        )),
    ),
    (
        "fixtures/storage/m5_storage_pressure_cases/low_disk_protect_core_expires_unpinned_evidence_only.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_storage_pressure_cases/low_disk_protect_core_expires_unpinned_evidence_only.yaml"
        )),
    ),
    (
        "fixtures/storage/m5_storage_pressure_cases/managed_quota_ceiling_narrows_surface.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_storage_pressure_cases/managed_quota_ceiling_narrows_surface.yaml"
        )),
    ),
    (
        "fixtures/storage/m5_storage_pressure_cases/quota_pressure_refuses_user_owned_state.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_storage_pressure_cases/quota_pressure_refuses_user_owned_state.yaml"
        )),
    ),
];

/// Strongly typed error returned by the corpus loader.
#[derive(Debug)]
pub enum StoragePressureLoadError {
    Yaml {
        fixture_ref: String,
        source: serde_yaml::Error,
    },
}

impl fmt::Display for StoragePressureLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yaml {
                fixture_ref,
                source,
            } => write!(
                f,
                "storage-pressure yaml parse error in {fixture_ref}: {source}"
            ),
        }
    }
}

impl Error for StoragePressureLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Yaml { source, .. } => Some(source),
        }
    }
}

/// Loads the checked-in storage-pressure banner corpus.
pub fn current_storage_pressure_banner_corpus(
) -> Result<StoragePressureBannerCorpus, StoragePressureLoadError> {
    let mut banners = Vec::with_capacity(BANNER_FIXTURES.len());
    for (fixture_ref, yaml) in BANNER_FIXTURES {
        let banner = serde_yaml::from_str::<StoragePressureBanner>(yaml).map_err(|source| {
            StoragePressureLoadError::Yaml {
                fixture_ref: (*fixture_ref).to_owned(),
                source,
            }
        })?;
        banners.push(StoragePressureBannerEntry {
            fixture_ref: (*fixture_ref).to_owned(),
            banner,
        });
    }
    Ok(StoragePressureBannerCorpus { banners })
}

impl StoragePressureBannerCorpus {
    /// Returns the banner with the given id, if present.
    pub fn banner(&self, banner_id: &str) -> Option<&StoragePressureBanner> {
        self.banners
            .iter()
            .find(|entry| entry.banner.banner_id == banner_id)
            .map(|entry| &entry.banner)
    }

    /// Validates every seeded banner against the safety contract, attributing
    /// each violation to its originating fixture.
    pub fn validate(&self) -> Vec<StoragePressureViolation> {
        let mut violations = Vec::new();
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for entry in &self.banners {
            if !seen.insert(entry.banner.banner_id.as_str()) {
                push(
                    &mut violations,
                    "corpus.duplicate_banner_id",
                    &entry.fixture_ref,
                    "banner_id must be unique across the corpus",
                );
            }
            entry
                .banner
                .validate_into(&mut violations, &entry.fixture_ref);
        }
        violations
    }

    /// Projects the corpus into a metadata-safe support/export envelope the
    /// support-bundle pipeline can quote without leaking raw payloads.
    pub fn support_export(
        &self,
        envelope_id: impl Into<String>,
        captured_at: impl Into<String>,
    ) -> StoragePressureBannerSupportExport {
        let mut banners: Vec<StoragePressureSupportExportRow> = self
            .banners
            .iter()
            .map(|entry| StoragePressureSupportExportRow::from_banner(&entry.banner))
            .collect();
        banners.sort_by(|a, b| a.banner_id.cmp(&b.banner_id));
        let pressure_event_count = banners.len() as u32;
        let escalation_pending_count = self
            .banners
            .iter()
            .filter(|entry| {
                entry.banner.escalation_state
                    == EscalationStateClass::ReviewedEscalationRequiredNotYetApproved
            })
            .count() as u32;
        StoragePressureBannerSupportExport {
            record_kind: M5_STORAGE_PRESSURE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_STORAGE_PRESSURE_SCHEMA_VERSION,
            envelope_id: envelope_id.into(),
            captured_at: captured_at.into(),
            schema_ref: M5_STORAGE_PRESSURE_SCHEMA_REF.to_owned(),
            doc_ref: M5_STORAGE_PRESSURE_DOC_REF.to_owned(),
            runtime_storage_classes_ref: RUNTIME_STORAGE_CLASSES_REF.to_owned(),
            runtime_low_disk_drills_ref: RUNTIME_LOW_DISK_DRILLS_REF.to_owned(),
            pressure_event_count,
            escalation_pending_count,
            authoritative_state_loss_count: self
                .banners
                .iter()
                .filter(|entry| entry.banner.authoritative_state_loss)
                .count() as u32,
            raw_content_exported: false,
            redaction_class: METADATA_SAFE_DEFAULT.to_owned(),
            banners,
        }
    }
}

// --------------------------------------------------------------------------
// Support-export projection.
// --------------------------------------------------------------------------

/// One metadata-safe summary row in the support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoragePressureSupportExportRow {
    pub record_kind: String,
    pub banner_id: String,
    pub pressure_class: PressureClass,
    pub pressure_source: PressureSourceClass,
    pub current_ladder_step: LowDiskLadderStep,
    pub current_ladder_order: u32,
    pub paused_work_count: u32,
    pub applied_step_count: u32,
    pub protected_class_not_trimmed_count: u32,
    pub guard_count: u32,
    pub total_reclaimed_bytes: u64,
    pub escalation_state: EscalationStateClass,
    pub authoritative_state_loss: bool,
}

impl StoragePressureSupportExportRow {
    fn from_banner(banner: &StoragePressureBanner) -> Self {
        Self {
            record_kind: M5_STORAGE_PRESSURE_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            banner_id: banner.banner_id.clone(),
            pressure_class: banner.pressure_class,
            pressure_source: banner.pressure_source,
            current_ladder_step: banner.current_ladder_step,
            current_ladder_order: banner.current_ladder_order,
            paused_work_count: banner.paused_work.len() as u32,
            applied_step_count: banner
                .eviction_order
                .iter()
                .filter(|step| step.applied)
                .count() as u32,
            protected_class_not_trimmed_count: banner.protected_class_ids_not_trimmed.len() as u32,
            guard_count: banner.state_loss_guards.len() as u32,
            total_reclaimed_bytes: banner.total_reclaimed_bytes(),
            escalation_state: banner.escalation_state,
            authoritative_state_loss: banner.authoritative_state_loss,
        }
    }
}

/// The metadata-safe support-export envelope folded from the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoragePressureBannerSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub envelope_id: String,
    pub captured_at: String,
    pub schema_ref: String,
    pub doc_ref: String,
    pub runtime_storage_classes_ref: String,
    pub runtime_low_disk_drills_ref: String,
    pub pressure_event_count: u32,
    pub escalation_pending_count: u32,
    pub authoritative_state_loss_count: u32,
    pub raw_content_exported: bool,
    pub redaction_class: String,
    pub banners: Vec<StoragePressureSupportExportRow>,
}

impl StoragePressureBannerSupportExport {
    /// True when the envelope is metadata-safe, banner-complete, and reports no
    /// authoritative state loss.
    pub fn is_export_safe(&self) -> bool {
        !self.raw_content_exported
            && self.redaction_class == METADATA_SAFE_DEFAULT
            && self.banners.len() as u32 == self.pressure_event_count
            && self.authoritative_state_loss_count == 0
    }
}

// --------------------------------------------------------------------------
// Validation.
// --------------------------------------------------------------------------

/// A validation violation surfaced by the banner harness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoragePressureViolation {
    pub check_id: String,
    pub target_ref: String,
    pub message: String,
}

impl fmt::Display for StoragePressureViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}: {}",
            self.check_id, self.target_ref, self.message
        )
    }
}

fn push(
    violations: &mut Vec<StoragePressureViolation>,
    check_id: impl Into<String>,
    target_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(StoragePressureViolation {
        check_id: check_id.into(),
        target_ref: target_ref.into(),
        message: message.into(),
    });
}
