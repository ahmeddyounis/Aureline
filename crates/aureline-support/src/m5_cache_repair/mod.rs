//! Targeted corrupt-cache / index repair, stale-label propagation, and a
//! no-reset-everything fallback for the heavy artifact families the M5 depth
//! lanes add.
//!
//! When a derived cache or semantic index is detected corrupt or stale, the
//! shell must prefer the *narrowest sufficient repair* — rebuild one index,
//! refetch one pack by digest, re-derive one cache, or repair one workspace's
//! recovery state — over a vague "clear everything" or factory reset. While the
//! repair is outstanding, every surface that reads the affected class must keep
//! showing a stale / rebuild-needed / corrupt label until the repair actually
//! completes. And when a suspect copy still holds user-owned data or forensic
//! value, it is quarantined before any clear, never deleted to make a problem
//! "go away".
//!
//! This module is the canonical, inspectable truth model behind that behaviour.
//! It mints no new storage primitive: the [`StorageClassId`] vocabulary
//! re-exports verbatim from [`crate::storage_inspector`], and the posture labels
//! re-export the runtime `storage_posture_class` vocabulary from
//! [`RUNTIME_STORAGE_CLASSES_REF`]. Only the fault, repair-action, repair-scope,
//! quarantine-disposition, repair-label, repair-state, and fallback labels are
//! introduced here, and they are bounded explanatory tokens. Crucially, the
//! repair-scope and fallback vocabularies carry **no** global / factory-reset
//! value: a plan is targeted by construction, and the fallback is always a
//! narrower-or-equal action, never a delete-everything.
//!
//! ## What this owns
//!
//! - The [`CacheRepairPlan`] record — one detected-corruption/staleness event
//!   under repair, carrying its affected storage class and scope, the detected
//!   fault, the targeted repair action, the quarantine disposition (and a
//!   quarantine ref when a suspect copy is preserved), the repair state, the
//!   propagated repair label and posture, the per-surface stale labels, the
//!   no-reset-everything fallback, and the open-inspector / run-targeted-repair
//!   actions. Mirrors the boundary schema at [`M5_CACHE_REPAIR_SCHEMA_REF`].
//! - The [`AffectedSurfaceLabel`] row — one surface that reads the affected
//!   class, with the stale / rebuild-needed / corrupt label it shows and whether
//!   that label is still active (it stays active until the repair completes).
//! - The [`CacheRepairPlanCorpus`] container — folds every seeded scenario plan
//!   into one validated bundle, checks the cross-record safety contract (scope is
//!   never global, no factory reset, suspect copies are quarantined before clear,
//!   labels persist until repair completes), and projects a metadata-safe
//!   [`CacheRepairSupportExport`] the support-bundle pipeline can quote without
//!   leaking raw payloads, paths, or credentials.
//! - The [`compose_plan`] projection — the first real consumer: it folds the
//!   canonical runtime storage-class profiles plus a [`RepairSignal`] into a plan
//!   that is correct by construction (the scope is a single class, no factory
//!   reset is offered, protected and user-owned classes preserve a quarantine
//!   copy before any clear, and every affected surface label stays active until
//!   the repair state reaches healthy).
//!
//! ## What this does NOT own
//!
//! - Live byte-level index rebuild, refetch, or quarantine I/O. Those belong to
//!   the runtime crates; this module is the shared truth model the storage
//!   inspector, repair affordances, service-health, and support export project. A
//!   plan describes a *disclosed* repair response; scheduling the rebuild and
//!   emitting the cleanup receipt is a sibling lane.
//! - The runtime storage-class vocabulary or the artifact-family matrix, which
//!   stay frozen in their own lanes.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_storage_governance::{
    current_runtime_storage_class_profiles, ClearCacheProtectionClass,
    M5StorageGovernanceLoadError, RuntimeStorageClassProfiles,
};
use crate::storage_inspector::StorageClassId;

#[cfg(test)]
mod tests;

/// Frozen schema version shared by every record in this module.
pub const M5_CACHE_REPAIR_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for a cache-repair plan.
pub const M5_CACHE_REPAIR_PLAN_RECORD_KIND: &str = "m5_cache_repair_plan";

/// Stable record-kind tag for one affected-surface stale label.
pub const M5_CACHE_REPAIR_SURFACE_LABEL_RECORD_KIND: &str = "m5_cache_repair_surface_label";

/// Stable record-kind tag for the support-export envelope.
pub const M5_CACHE_REPAIR_SUPPORT_EXPORT_RECORD_KIND: &str = "m5_cache_repair_support_export";

/// Stable record-kind tag for one support-export row.
pub const M5_CACHE_REPAIR_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "m5_cache_repair_support_export_row";

/// Repository-relative path of the boundary schema for the plan.
pub const M5_CACHE_REPAIR_SCHEMA_REF: &str = "schemas/storage/m5_cache_repair.schema.json";

/// Repository-relative path of the reviewer contract doc every plan quotes.
pub const M5_CACHE_REPAIR_DOC_REF: &str = "docs/storage/m5_cache_repair_contract.md";

/// Repository-relative path of the canonical runtime storage-class contract.
pub const RUNTIME_STORAGE_CLASSES_REF: &str = "artifacts/runtime/storage_classes.yaml";

/// The metadata-safe redaction class every plan and export envelope carries.
pub const METADATA_SAFE_DEFAULT: &str = "metadata_safe_default";

/// The stable action id that opens the storage inspector from a plan.
pub const OPEN_STORAGE_INSPECTOR_ACTION_REF: &str = "action.storage.open_inspector";

/// The stable action id that runs the targeted repair from a plan.
pub const RUN_TARGETED_REPAIR_ACTION_REF: &str = "action.storage.run_targeted_repair";

// --------------------------------------------------------------------------
// Closed vocabularies introduced by this lane.
//
// storage_class_id re-exports from storage_inspector and storage_posture_class
// re-exports the runtime posture vocabulary. fault, repair-action, repair-scope,
// quarantine-disposition, repair-label, repair-state, and fallback are bounded
// explanatory labels resolved against the runtime contract. None of them carry a
// global / factory-reset value: a repair is targeted by construction.
// --------------------------------------------------------------------------

/// The detected corruption or staleness condition that triggers a repair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FaultClass {
    /// A semantic index / search shard is structurally corrupt or unreadable.
    CorruptIndex,
    /// A content-addressed entry failed digest verification.
    ChecksumMismatch,
    /// An interrupted write left a torn / partially written entry.
    PartialWriteTorn,
    /// An entry was produced by an incompatible schema / capsule version.
    SchemaVersionDrift,
    /// A derived cache is stale against its authoritative source.
    StaleAgainstSource,
    /// A referenced backing object is missing.
    MissingBackingObject,
    /// Entries whose owning scope no longer exists.
    OrphanedEntries,
}

impl FaultClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CorruptIndex => "corrupt_index",
            Self::ChecksumMismatch => "checksum_mismatch",
            Self::PartialWriteTorn => "partial_write_torn",
            Self::SchemaVersionDrift => "schema_version_drift",
            Self::StaleAgainstSource => "stale_against_source",
            Self::MissingBackingObject => "missing_backing_object",
            Self::OrphanedEntries => "orphaned_entries",
        }
    }
}

/// The targeted remedy a plan applies. There is intentionally no
/// delete-everything / factory-reset action: every variant repairs one class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairActionClass {
    /// Rebuild a corrupt index / shard from authoritative source.
    RebuildFromSource,
    /// Refetch a content-addressed pack by its digest.
    RefetchByDigest,
    /// Revalidate stale entries against source; correct in place.
    RevalidateAgainstSource,
    /// Drop torn / orphaned disposable entries; re-derive on demand.
    RederiveOnDemand,
    /// Quarantine the suspect copy, then rebuild from source.
    QuarantineThenRebuild,
    /// Quarantine the suspect copy and route to a class-specific review;
    /// authoritative evidence that cannot be auto-rebuilt is never cleared.
    QuarantineThenManualReview,
    /// Repair user-owned recovery state in place from its own checkpoint, with
    /// the suspect copy preserved.
    RepairInPlaceFromCheckpoint,
}

impl RepairActionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RebuildFromSource => "rebuild_from_source",
            Self::RefetchByDigest => "refetch_by_digest",
            Self::RevalidateAgainstSource => "revalidate_against_source",
            Self::RederiveOnDemand => "rederive_on_demand",
            Self::QuarantineThenRebuild => "quarantine_then_rebuild",
            Self::QuarantineThenManualReview => "quarantine_then_manual_review",
            Self::RepairInPlaceFromCheckpoint => "repair_in_place_from_checkpoint",
        }
    }

    /// Every repair action is targeted to one class; none resets everything.
    pub const fn is_targeted(self) -> bool {
        true
    }

    /// True when the action discards the suspect entry before rebuilding. Such
    /// an action MUST preserve a quarantine copy first when the suspect data
    /// still holds user-owned data or forensic value.
    pub const fn clears_suspect_data(self) -> bool {
        match self {
            Self::RebuildFromSource
            | Self::RefetchByDigest
            | Self::RederiveOnDemand
            | Self::QuarantineThenRebuild => true,
            Self::RevalidateAgainstSource
            | Self::QuarantineThenManualReview
            | Self::RepairInPlaceFromCheckpoint => false,
        }
    }
}

/// The granularity of a repair. Both variants target a single storage class;
/// there is intentionally no all-classes / factory-reset scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairScopeClass {
    /// One storage class within one workspace.
    SingleClassSingleWorkspace,
    /// One storage class across every workspace that shares it.
    SingleClassAllWorkspaces,
}

impl RepairScopeClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleClassSingleWorkspace => "single_class_single_workspace",
            Self::SingleClassAllWorkspaces => "single_class_all_workspaces",
        }
    }

    /// Always false: a repair scope is never global / factory-wide.
    pub const fn is_global(self) -> bool {
        false
    }

    /// True when the scope is bound to a single named workspace.
    pub const fn requires_workspace(self) -> bool {
        matches!(self, Self::SingleClassSingleWorkspace)
    }
}

/// What happens to the suspect copy before any clear.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineDispositionClass {
    /// Suspect copy quarantined because it still holds user-owned data.
    QuarantinedUserOwnedDataPreserved,
    /// Suspect copy quarantined because it still holds forensic value.
    QuarantinedForensicsValuePreserved,
    /// Suspect copy quarantined pending an export-before-delete review.
    QuarantinedPendingExport,
    /// Pure disposable cache with no user-owned data and no forensic value; no
    /// quarantine copy is required before re-derivation.
    NoQuarantineDisposableOnly,
}

impl QuarantineDispositionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuarantinedUserOwnedDataPreserved => "quarantined_user_owned_data_preserved",
            Self::QuarantinedForensicsValuePreserved => "quarantined_forensics_value_preserved",
            Self::QuarantinedPendingExport => "quarantined_pending_export",
            Self::NoQuarantineDisposableOnly => "no_quarantine_disposable_only",
        }
    }

    /// True when a suspect copy is preserved before any clear.
    pub const fn preserves_suspect_copy(self) -> bool {
        !matches!(self, Self::NoQuarantineDisposableOnly)
    }
}

/// The propagated label a surface shows while a class is corrupt / stale / under
/// repair. It stays present until the repair actually completes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairLabelClass {
    /// The class is corrupt and not yet repaired.
    Corrupt,
    /// The class is stale against its source and not yet refreshed.
    Stale,
    /// A rebuildable cache must be rebuilt before it is trustworthy.
    RebuildNeeded,
    /// A semantic index must be reindexed before it is trustworthy.
    ReindexNeeded,
    /// A repair is in progress; the class is not yet trustworthy.
    RepairInProgress,
    /// The suspect copy is quarantined; the authoritative copy is preserved.
    Quarantined,
    /// The class is repaired and healthy.
    Healthy,
}

impl RepairLabelClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Corrupt => "corrupt",
            Self::Stale => "stale",
            Self::RebuildNeeded => "rebuild_needed",
            Self::ReindexNeeded => "reindex_needed",
            Self::RepairInProgress => "repair_in_progress",
            Self::Quarantined => "quarantined",
            Self::Healthy => "healthy",
        }
    }

    /// True only for the terminal healthy label.
    pub const fn is_healthy(self) -> bool {
        matches!(self, Self::Healthy)
    }

    /// The canonical runtime posture this label projects onto an inspectable
    /// surface.
    pub const fn posture(self) -> StoragePostureClass {
        match self {
            Self::Corrupt
            | Self::Stale
            | Self::RebuildNeeded
            | Self::ReindexNeeded
            | Self::RepairInProgress => StoragePostureClass::RebuildPending,
            Self::Quarantined => StoragePostureClass::RetainedForEvidence,
            Self::Healthy => StoragePostureClass::Healthy,
        }
    }
}

/// Runtime storage-posture vocabulary, re-exported verbatim from the runtime
/// storage-class contract so inspectors and this lane share one posture set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StoragePostureClass {
    Healthy,
    RebuildPending,
    PressureTrimmed,
    ResetCandidate,
    RetainedForEvidence,
    Missing,
}

impl StoragePostureClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::RebuildPending => "rebuild_pending",
            Self::PressureTrimmed => "pressure_trimmed",
            Self::ResetCandidate => "reset_candidate",
            Self::RetainedForEvidence => "retained_for_evidence",
            Self::Missing => "missing",
        }
    }
}

/// The lifecycle state of a repair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairStateClass {
    /// The fault was detected; no repair has started.
    Detected,
    /// The suspect copy was quarantined before any clear.
    QuarantinePreserved,
    /// The targeted repair is in progress.
    RepairInProgress,
    /// The repair completed and the class is healthy.
    RepairCompleteHealthy,
    /// The targeted repair failed; a narrower-or-equal fallback is offered.
    RepairFailedFallbackOffered,
}

impl RepairStateClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Detected => "detected",
            Self::QuarantinePreserved => "quarantine_preserved",
            Self::RepairInProgress => "repair_in_progress",
            Self::RepairCompleteHealthy => "repair_complete_healthy",
            Self::RepairFailedFallbackOffered => "repair_failed_fallback_offered",
        }
    }

    /// True only when the repair completed and the class is healthy.
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::RepairCompleteHealthy)
    }

    /// True when the repair failed and a fallback is offered.
    pub const fn is_failed(self) -> bool {
        matches!(self, Self::RepairFailedFallbackOffered)
    }
}

/// The fallback offered when a targeted repair fails. Every variant is a
/// narrower-or-equal action; there is intentionally no reset-everything value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackActionClass {
    /// No fallback is needed; the repair has not failed.
    NoFallbackNeeded,
    /// Retry the same targeted repair.
    RetryTargetedRepair,
    /// Widen, under review, to the same class across the workspace — still one
    /// class, never every class.
    WidenToWorkspaceScopeReview,
    /// Continue working with the affected cache excluded; rebuild lazily.
    OpenWithoutAffectedCache,
    /// Route to a class-specific review (protected / user-owned classes).
    ManualClassSpecificReviewRequired,
}

impl FallbackActionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoFallbackNeeded => "no_fallback_needed",
            Self::RetryTargetedRepair => "retry_targeted_repair",
            Self::WidenToWorkspaceScopeReview => "widen_to_workspace_scope_review",
            Self::OpenWithoutAffectedCache => "open_without_affected_cache",
            Self::ManualClassSpecificReviewRequired => "manual_class_specific_review_required",
        }
    }

    /// Always false: a fallback never resets everything or factory-resets.
    pub const fn is_reset_everything(self) -> bool {
        false
    }
}

// --------------------------------------------------------------------------
// Class helpers — protected / authoritative classes that must preserve a
// quarantine copy and can never be auto-rebuilt from a derived source.
// --------------------------------------------------------------------------

/// True for the protected classes — evidence and user-owned recovery — whose
/// suspect copies must always be quarantined before any clear.
const fn is_protected_class(class_id: StorageClassId) -> bool {
    matches!(
        class_id,
        StorageClassId::EvidenceSupportCache | StorageClassId::UserOwnedRecoveryState
    )
}

// --------------------------------------------------------------------------
// Plan records.
// --------------------------------------------------------------------------

/// One surface that reads the affected class and shows a propagated label until
/// the repair completes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffectedSurfaceLabel {
    pub record_kind: String,
    pub surface_ref: String,
    pub surface_label: String,
    pub repair_label: RepairLabelClass,
    pub posture: StoragePostureClass,
    /// True until the repair completes; a surface keeps the stale label until
    /// the class is actually trustworthy again.
    pub label_active: bool,
    /// Always true: the label is designed to clear once the repair completes.
    pub clears_on_repair_complete: bool,
    pub detail: String,
}

/// A cache-repair plan: the disclosed, targeted response to one detected
/// corruption or staleness event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheRepairPlan {
    pub record_kind: String,
    pub schema_version: u32,
    pub plan_id: String,
    pub emitted_at: String,
    pub headline: String,
    pub storage_class_id: StorageClassId,
    pub scope_class: RepairScopeClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_label: Option<String>,
    pub fault_class: FaultClass,
    pub repair_action: RepairActionClass,
    pub quarantine_disposition: QuarantineDispositionClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quarantine_ref: Option<String>,
    pub repair_state: RepairStateClass,
    pub repair_label: RepairLabelClass,
    pub posture: StoragePostureClass,
    pub affected_surface_labels: Vec<AffectedSurfaceLabel>,
    pub fallback_action: FallbackActionClass,
    /// True when the suspect copy holds user-owned data that must be preserved.
    pub preserves_user_owned_data: bool,
    /// True when the suspect copy holds forensic value that must be preserved.
    pub preserves_forensics_value: bool,
    /// Always false: a single-class repair never offers a factory reset.
    pub factory_reset_offered: bool,
    /// Always true: this plan repairs one class instead of resetting everything.
    pub reset_everything_avoided: bool,
    /// Always true: the proposed action is the narrowest sufficient scope.
    pub narrowest_sufficient_scope: bool,
    pub open_inspector_action_ref: String,
    pub run_targeted_repair_action_ref: String,
    #[serde(default)]
    pub guardrail_notices: Vec<String>,
    pub schema_ref: String,
    pub doc_ref: String,
}

impl CacheRepairPlan {
    /// Returns the surface label for the given surface ref, if present.
    pub fn surface_label(&self, surface_ref: &str) -> Option<&AffectedSurfaceLabel> {
        self.affected_surface_labels
            .iter()
            .find(|label| label.surface_ref == surface_ref)
    }

    /// Count of surface labels still actively showing a stale state.
    pub fn active_label_count(&self) -> u32 {
        self.affected_surface_labels
            .iter()
            .filter(|label| label.label_active)
            .count() as u32
    }

    /// True when the plan is export-safe: no factory reset, reset-everything is
    /// avoided, the scope is targeted, and the suspect-copy / label invariants
    /// hold.
    pub fn is_export_safe(&self) -> bool {
        let mut violations = Vec::new();
        self.validate_into(&mut violations, "plan");
        violations.is_empty()
    }

    /// Validates this plan against the targeted-repair, quarantine-before-clear,
    /// and stale-label-propagation contract, attributing each violation to
    /// `target_ref`.
    pub fn validate_into(&self, violations: &mut Vec<CacheRepairViolation>, target_ref: &str) {
        let target = target_ref;

        if self.schema_version != M5_CACHE_REPAIR_SCHEMA_VERSION {
            push(
                violations,
                "plan.schema_version",
                target,
                "schema_version must be 1",
            );
        }
        if self.record_kind != M5_CACHE_REPAIR_PLAN_RECORD_KIND {
            push(
                violations,
                "plan.record_kind",
                target,
                "record_kind must be m5_cache_repair_plan",
            );
        }
        if self.schema_ref != M5_CACHE_REPAIR_SCHEMA_REF {
            push(
                violations,
                "plan.schema_ref",
                target,
                "schema_ref must pin the boundary schema",
            );
        }
        if self.doc_ref != M5_CACHE_REPAIR_DOC_REF {
            push(
                violations,
                "plan.doc_ref",
                target,
                "doc_ref must pin the contract doc",
            );
        }
        if self.plan_id.trim().is_empty() {
            push(
                violations,
                "plan.plan_id",
                target,
                "plan_id must be non-empty",
            );
        }
        if self.headline.trim().is_empty() {
            push(
                violations,
                "plan.headline",
                target,
                "headline must be non-empty",
            );
        }
        if self.open_inspector_action_ref != OPEN_STORAGE_INSPECTOR_ACTION_REF {
            push(
                violations,
                "plan.open_inspector_action_ref",
                target,
                "open_inspector_action_ref must offer the inspector action",
            );
        }
        if self.run_targeted_repair_action_ref != RUN_TARGETED_REPAIR_ACTION_REF {
            push(
                violations,
                "plan.run_targeted_repair_action_ref",
                target,
                "run_targeted_repair_action_ref must offer the targeted-repair action",
            );
        }

        self.validate_no_reset_everything(violations, target);
        self.validate_scope(violations, target);
        self.validate_quarantine(violations, target);
        self.validate_labels(violations, target);
        self.validate_fallback(violations, target);
    }

    /// A plan never offers a factory reset, always avoids reset-everything, and
    /// always proposes the narrowest sufficient scope.
    fn validate_no_reset_everything(
        &self,
        violations: &mut Vec<CacheRepairViolation>,
        target: &str,
    ) {
        if self.factory_reset_offered {
            push(
                violations,
                "plan.factory_reset_offered",
                target,
                "factory_reset_offered must be false",
            );
        }
        if !self.reset_everything_avoided {
            push(
                violations,
                "plan.reset_everything_avoided",
                target,
                "reset_everything_avoided must be true",
            );
        }
        if !self.narrowest_sufficient_scope {
            push(
                violations,
                "plan.narrowest_sufficient_scope",
                target,
                "narrowest_sufficient_scope must be true",
            );
        }
        if self.scope_class.is_global() {
            push(
                violations,
                "plan.scope_global",
                target,
                "scope_class must not be global",
            );
        }
        if self.fallback_action.is_reset_everything() {
            push(
                violations,
                "plan.fallback_reset_everything",
                target,
                "fallback_action must never reset everything",
            );
        }
        if !self.repair_action.is_targeted() {
            push(
                violations,
                "plan.repair_action_targeted",
                target,
                "repair_action must be targeted to one class",
            );
        }
    }

    /// The scope is a single class; a workspace-scoped plan names its workspace,
    /// and an all-workspaces plan names none.
    fn validate_scope(&self, violations: &mut Vec<CacheRepairViolation>, target: &str) {
        match self.scope_class {
            RepairScopeClass::SingleClassSingleWorkspace => {
                if self
                    .workspace_ref
                    .as_ref()
                    .map(|reference| reference.trim().is_empty())
                    .unwrap_or(true)
                {
                    push(
                        violations,
                        "plan.scope.workspace_required",
                        target,
                        "single_class_single_workspace must name a workspace_ref",
                    );
                }
            }
            RepairScopeClass::SingleClassAllWorkspaces => {
                if self.workspace_ref.is_some() {
                    push(
                        violations,
                        "plan.scope.workspace_unexpected",
                        target,
                        "single_class_all_workspaces must not name a workspace_ref",
                    );
                }
            }
        }
    }

    /// A suspect copy that still holds user-owned data or forensic value is
    /// quarantined before any clear; a disposable-only class needs no copy.
    fn validate_quarantine(&self, violations: &mut Vec<CacheRepairViolation>, target: &str) {
        let preserves_copy = self.quarantine_disposition.preserves_suspect_copy();
        let needs_copy = self.preserves_user_owned_data || self.preserves_forensics_value;

        if preserves_copy != needs_copy {
            push(
                violations,
                "plan.quarantine.disposition_mismatch",
                target,
                "a quarantine copy is preserved exactly when user-owned or forensic value is at risk",
            );
        }
        if preserves_copy
            != self
                .quarantine_ref
                .as_ref()
                .map(|reference| !reference.trim().is_empty())
                .unwrap_or(false)
        {
            push(
                violations,
                "plan.quarantine.ref_mismatch",
                target,
                "quarantine_ref is present exactly when a suspect copy is preserved",
            );
        }

        // Protected classes always preserve a quarantine copy — they are never
        // disposable-only and never cleared without preservation.
        if is_protected_class(self.storage_class_id) && !preserves_copy {
            push(
                violations,
                "plan.quarantine.protected_requires_copy",
                target,
                "evidence and user-owned recovery classes must quarantine the suspect copy",
            );
        }
        if self.storage_class_id == StorageClassId::UserOwnedRecoveryState
            && !self.preserves_user_owned_data
        {
            push(
                violations,
                "plan.quarantine.recovery_user_owned",
                target,
                "user-owned recovery repair must preserve user-owned data",
            );
        }
        if self.storage_class_id == StorageClassId::EvidenceSupportCache
            && !self.preserves_forensics_value
        {
            push(
                violations,
                "plan.quarantine.evidence_forensics",
                target,
                "evidence repair must preserve forensic value",
            );
        }

        // Never clear suspect data before preserving a required quarantine copy.
        if self.repair_action.clears_suspect_data()
            && needs_copy
            && self
                .quarantine_ref
                .as_ref()
                .map(|reference| reference.trim().is_empty())
                .unwrap_or(true)
        {
            push(
                violations,
                "plan.quarantine.clear_before_preserve",
                target,
                "suspect data must not be cleared before a required quarantine copy exists",
            );
        }
    }

    /// Every affected surface shows the propagated label until the repair
    /// completes; on completion every label clears and the class is healthy.
    fn validate_labels(&self, violations: &mut Vec<CacheRepairViolation>, target: &str) {
        if self.posture != self.repair_label.posture() {
            push(
                violations,
                "plan.posture_mismatch",
                target,
                "posture must equal the repair label's posture",
            );
        }
        // complete <=> healthy.
        if self.repair_state.is_complete() != self.repair_label.is_healthy() {
            push(
                violations,
                "plan.state_label_coherence",
                target,
                "repair_label is healthy exactly when repair_state is complete",
            );
        }
        if self.affected_surface_labels.is_empty() {
            push(
                violations,
                "plan.labels.empty",
                target,
                "a repair must name at least one affected surface",
            );
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for label in &self.affected_surface_labels {
            if label.record_kind != M5_CACHE_REPAIR_SURFACE_LABEL_RECORD_KIND {
                push(
                    violations,
                    "plan.label.record_kind",
                    target,
                    "surface label record_kind must be m5_cache_repair_surface_label",
                );
            }
            if !seen.insert(label.surface_ref.as_str()) {
                push(
                    violations,
                    "plan.label.duplicate_surface",
                    target,
                    "each affected surface may appear at most once",
                );
            }
            if label.surface_ref.trim().is_empty() || label.surface_label.trim().is_empty() {
                push(
                    violations,
                    "plan.label.empty_surface",
                    target,
                    "surface_ref and surface_label must be non-empty",
                );
            }
            if label.detail.trim().is_empty() {
                push(
                    violations,
                    "plan.label.empty_detail",
                    target,
                    "surface label detail must be non-empty",
                );
            }
            if !label.clears_on_repair_complete {
                push(
                    violations,
                    "plan.label.must_clear",
                    target,
                    "a propagated label must be cleared on repair completion",
                );
            }
            // Surfaces carry the plan's label and posture.
            if label.repair_label != self.repair_label {
                push(
                    violations,
                    "plan.label.label_mismatch",
                    target,
                    "each surface must show the plan's repair label until repair completes",
                );
            }
            if label.posture != label.repair_label.posture() {
                push(
                    violations,
                    "plan.label.posture_mismatch",
                    target,
                    "surface posture must equal its repair label's posture",
                );
            }
            // The label stays active until the repair completes.
            let expected_active = !self.repair_state.is_complete();
            if label.label_active != expected_active {
                push(
                    violations,
                    "plan.label.active_mismatch",
                    target,
                    "a surface label stays active until the repair completes",
                );
            }
        }
    }

    /// A failed repair always offers a (narrower-or-equal) fallback; a
    /// non-failed repair offers none.
    fn validate_fallback(&self, violations: &mut Vec<CacheRepairViolation>, target: &str) {
        let has_fallback = self.fallback_action != FallbackActionClass::NoFallbackNeeded;
        if self.repair_state.is_failed() != has_fallback {
            push(
                violations,
                "plan.fallback.state_coherence",
                target,
                "a fallback is offered exactly when the repair failed",
            );
        }
    }
}

// --------------------------------------------------------------------------
// Matrix-backed composer — the first real consumer.
// --------------------------------------------------------------------------

/// One affected surface the caller hands the composer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffectedSurfaceInput {
    pub surface_ref: String,
    pub surface_label: String,
    pub detail: String,
}

/// The repair signal [`compose_plan`] folds into a plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairSignal {
    pub plan_id: String,
    pub emitted_at: String,
    pub storage_class_id: StorageClassId,
    pub scope_class: RepairScopeClass,
    #[serde(default)]
    pub workspace_ref: Option<String>,
    #[serde(default)]
    pub workspace_label: Option<String>,
    pub fault_class: FaultClass,
    pub repair_state: RepairStateClass,
    /// True when the suspect copy carries user-owned data beyond the class's
    /// inherent authority (rare for derived caches; always implied for the
    /// user-owned recovery class).
    #[serde(default)]
    pub holds_user_owned_data: bool,
    /// True when the suspect copy carries forensic value beyond the class's
    /// inherent authority (always implied for the evidence class).
    #[serde(default)]
    pub holds_forensics_value: bool,
    /// Optional explicit quarantine ref; the composer derives a deterministic
    /// one when a copy is required and none is supplied.
    #[serde(default)]
    pub quarantine_ref: Option<String>,
    pub affected_surfaces: Vec<AffectedSurfaceInput>,
}

/// Folds the canonical runtime storage-class profiles plus a repair signal into
/// a plan that is correct by construction: the scope targets one class, no
/// factory reset is offered, protected and user-owned classes preserve a
/// quarantine copy before any clear, and every affected surface label stays
/// active until the repair reaches a healthy state.
///
/// The `profiles` argument anchors the plan to the same per-class clear-cache
/// protection and export-before-delete contract the storage-governance lane
/// validates; the composer reads no private protection posture of its own.
pub fn compose_plan(
    profiles: &RuntimeStorageClassProfiles,
    signal: &RepairSignal,
) -> CacheRepairPlan {
    let class_id = signal.storage_class_id;
    let is_user_owned = class_id == StorageClassId::UserOwnedRecoveryState;
    let is_evidence = class_id == StorageClassId::EvidenceSupportCache;

    // Per-class protection / export posture is read from the canonical runtime
    // profile, not invented here.
    let export_before_delete = profiles
        .get(class_id)
        .map(|profile| profile.export_before_delete_required)
        .unwrap_or(false);
    let profile_protected = profiles
        .get(class_id)
        .map(|profile| {
            matches!(
                profile.clear_cache_protection_class,
                ClearCacheProtectionClass::ProtectedRequiresClassSpecificReview
                    | ClearCacheProtectionClass::ProtectedNeverGenericClear
            )
        })
        .unwrap_or(false);

    let preserves_user = is_user_owned || signal.holds_user_owned_data;
    let preserves_forensics =
        (is_evidence || profile_protected || signal.holds_forensics_value) && !preserves_user;
    let needs_copy = preserves_user || preserves_forensics;

    let quarantine_disposition = if preserves_user {
        QuarantineDispositionClass::QuarantinedUserOwnedDataPreserved
    } else if preserves_forensics {
        if export_before_delete {
            QuarantineDispositionClass::QuarantinedPendingExport
        } else {
            QuarantineDispositionClass::QuarantinedForensicsValuePreserved
        }
    } else {
        QuarantineDispositionClass::NoQuarantineDisposableOnly
    };

    let quarantine_ref = if needs_copy {
        Some(
            signal
                .quarantine_ref
                .clone()
                .unwrap_or_else(|| format!("quarantine.{}.{}", class_id.as_str(), signal.plan_id)),
        )
    } else {
        None
    };

    let repair_action = derive_action(class_id, signal.fault_class, needs_copy);
    let repair_label = label_for_state(signal.repair_state, class_id, signal.fault_class);
    let posture = repair_label.posture();

    let affected_surface_labels = signal
        .affected_surfaces
        .iter()
        .map(|input| AffectedSurfaceLabel {
            record_kind: M5_CACHE_REPAIR_SURFACE_LABEL_RECORD_KIND.to_owned(),
            surface_ref: input.surface_ref.clone(),
            surface_label: input.surface_label.clone(),
            repair_label,
            posture,
            label_active: !signal.repair_state.is_complete(),
            clears_on_repair_complete: true,
            detail: input.detail.clone(),
        })
        .collect();

    let fallback_action = derive_fallback(signal.repair_state, class_id);

    let mut guardrail_notices = vec![
        "Aureline repairs this storage class on its own; it never resets everything or factory-resets to fix one corrupt class.".to_owned(),
    ];
    if preserves_user {
        guardrail_notices.push(
            "The suspect copy holds user-owned data; it is quarantined before any repair clears it and is never deleted to fix a cache.".to_owned(),
        );
    }
    if preserves_forensics {
        guardrail_notices.push(
            "The suspect copy holds forensic value; it is quarantined for review before any repair clears it.".to_owned(),
        );
    }

    CacheRepairPlan {
        record_kind: M5_CACHE_REPAIR_PLAN_RECORD_KIND.to_owned(),
        schema_version: M5_CACHE_REPAIR_SCHEMA_VERSION,
        plan_id: signal.plan_id.clone(),
        emitted_at: signal.emitted_at.clone(),
        headline: compose_headline(class_id, signal.fault_class, signal.repair_state),
        storage_class_id: class_id,
        scope_class: signal.scope_class,
        workspace_ref: signal.workspace_ref.clone(),
        workspace_label: signal.workspace_label.clone(),
        fault_class: signal.fault_class,
        repair_action,
        quarantine_disposition,
        quarantine_ref,
        repair_state: signal.repair_state,
        repair_label,
        posture,
        affected_surface_labels,
        fallback_action,
        preserves_user_owned_data: preserves_user,
        preserves_forensics_value: preserves_forensics,
        factory_reset_offered: false,
        reset_everything_avoided: true,
        narrowest_sufficient_scope: true,
        open_inspector_action_ref: OPEN_STORAGE_INSPECTOR_ACTION_REF.to_owned(),
        run_targeted_repair_action_ref: RUN_TARGETED_REPAIR_ACTION_REF.to_owned(),
        guardrail_notices,
        schema_ref: M5_CACHE_REPAIR_SCHEMA_REF.to_owned(),
        doc_ref: M5_CACHE_REPAIR_DOC_REF.to_owned(),
    }
}

/// Derives the targeted repair action for a class / fault.
fn derive_action(
    class_id: StorageClassId,
    fault: FaultClass,
    needs_copy: bool,
) -> RepairActionClass {
    match class_id {
        StorageClassId::UserOwnedRecoveryState => RepairActionClass::RepairInPlaceFromCheckpoint,
        StorageClassId::EvidenceSupportCache => RepairActionClass::QuarantineThenManualReview,
        StorageClassId::KnowledgeCache => {
            if needs_copy {
                RepairActionClass::QuarantineThenRebuild
            } else if fault == FaultClass::StaleAgainstSource {
                RepairActionClass::RevalidateAgainstSource
            } else {
                RepairActionClass::RebuildFromSource
            }
        }
        StorageClassId::ArtifactCache | StorageClassId::PrebuildEnvironmentCache => {
            if needs_copy {
                RepairActionClass::QuarantineThenRebuild
            } else {
                match fault {
                    FaultClass::ChecksumMismatch
                    | FaultClass::MissingBackingObject
                    | FaultClass::CorruptIndex
                    | FaultClass::PartialWriteTorn => RepairActionClass::RefetchByDigest,
                    FaultClass::StaleAgainstSource => RepairActionClass::RevalidateAgainstSource,
                    FaultClass::SchemaVersionDrift | FaultClass::OrphanedEntries => {
                        RepairActionClass::RederiveOnDemand
                    }
                }
            }
        }
        StorageClassId::InteractiveHotCache => {
            if needs_copy {
                RepairActionClass::QuarantineThenRebuild
            } else if fault == FaultClass::StaleAgainstSource {
                RepairActionClass::RevalidateAgainstSource
            } else {
                RepairActionClass::RederiveOnDemand
            }
        }
    }
}

/// The label a detected fault shows before any repair.
const fn detected_label(fault: FaultClass) -> RepairLabelClass {
    match fault {
        FaultClass::CorruptIndex
        | FaultClass::ChecksumMismatch
        | FaultClass::PartialWriteTorn
        | FaultClass::MissingBackingObject => RepairLabelClass::Corrupt,
        FaultClass::StaleAgainstSource
        | FaultClass::SchemaVersionDrift
        | FaultClass::OrphanedEntries => RepairLabelClass::Stale,
    }
}

/// The label a class shows while a repair is outstanding.
const fn pending_label(class_id: StorageClassId, fault: FaultClass) -> RepairLabelClass {
    if is_protected_class(class_id) {
        // Authoritative classes are repaired in place / quarantined, never
        // "rebuilt"; their suspect copy is quarantined while preserved.
        RepairLabelClass::Quarantined
    } else if matches!(class_id, StorageClassId::KnowledgeCache)
        && matches!(
            fault,
            FaultClass::CorruptIndex
                | FaultClass::SchemaVersionDrift
                | FaultClass::StaleAgainstSource
                | FaultClass::OrphanedEntries
        )
    {
        RepairLabelClass::ReindexNeeded
    } else {
        RepairLabelClass::RebuildNeeded
    }
}

/// The propagated label for a repair state.
const fn label_for_state(
    state: RepairStateClass,
    class_id: StorageClassId,
    fault: FaultClass,
) -> RepairLabelClass {
    match state {
        RepairStateClass::Detected => detected_label(fault),
        RepairStateClass::QuarantinePreserved => RepairLabelClass::Quarantined,
        RepairStateClass::RepairInProgress => pending_label(class_id, fault),
        RepairStateClass::RepairCompleteHealthy => RepairLabelClass::Healthy,
        RepairStateClass::RepairFailedFallbackOffered => pending_label(class_id, fault),
    }
}

/// Derives the no-reset-everything fallback for a failed repair.
const fn derive_fallback(state: RepairStateClass, class_id: StorageClassId) -> FallbackActionClass {
    if !state.is_failed() {
        FallbackActionClass::NoFallbackNeeded
    } else if is_protected_class(class_id) {
        FallbackActionClass::ManualClassSpecificReviewRequired
    } else {
        FallbackActionClass::RetryTargetedRepair
    }
}

/// Composes the stable plan headline for a class / fault / state.
fn compose_headline(
    class_id: StorageClassId,
    fault: FaultClass,
    state: RepairStateClass,
) -> String {
    let what = match class_id {
        StorageClassId::InteractiveHotCache => "An interactive cache",
        StorageClassId::KnowledgeCache => "A search / graph index",
        StorageClassId::ArtifactCache => "An artifact pack cache",
        StorageClassId::PrebuildEnvironmentCache => "A prebuild environment cache",
        StorageClassId::EvidenceSupportCache => "An evidence artifact",
        StorageClassId::UserOwnedRecoveryState => "Recovery state",
    };
    let condition = match fault {
        FaultClass::CorruptIndex => "is corrupt",
        FaultClass::ChecksumMismatch => "failed digest verification",
        FaultClass::PartialWriteTorn => "has a torn write",
        FaultClass::SchemaVersionDrift => "drifted from its schema version",
        FaultClass::StaleAgainstSource => "is stale against its source",
        FaultClass::MissingBackingObject => "is missing its backing object",
        FaultClass::OrphanedEntries => "has orphaned entries",
    };
    let action = match state {
        RepairStateClass::Detected => "Aureline will repair just this class.",
        RepairStateClass::QuarantinePreserved => {
            "Aureline quarantined the suspect copy and will repair just this class."
        }
        RepairStateClass::RepairInProgress => "Aureline is repairing just this class.",
        RepairStateClass::RepairCompleteHealthy => "Aureline repaired just this class.",
        RepairStateClass::RepairFailedFallbackOffered => {
            "The targeted repair did not finish; Aureline offers a narrower fallback, never a reset."
        }
    };
    format!("{what} {condition}. {action}")
}

// --------------------------------------------------------------------------
// Corpus container, entries, and loaders.
// --------------------------------------------------------------------------

/// One plan fixture paired with its repository-relative path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheRepairPlanEntry {
    pub fixture_ref: String,
    pub plan: CacheRepairPlan,
}

/// Cache-repair plan corpus loaded from the checked-in fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheRepairPlanCorpus {
    pub plans: Vec<CacheRepairPlanEntry>,
}

const PLAN_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/storage/m5_cache_repair_cases/knowledge_cache_corrupt_index_reindex.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_cache_repair_cases/knowledge_cache_corrupt_index_reindex.yaml"
        )),
    ),
    (
        "fixtures/storage/m5_cache_repair_cases/artifact_pack_checksum_mismatch_refetch.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_cache_repair_cases/artifact_pack_checksum_mismatch_refetch.yaml"
        )),
    ),
    (
        "fixtures/storage/m5_cache_repair_cases/generated_preview_torn_rederive.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_cache_repair_cases/generated_preview_torn_rederive.yaml"
        )),
    ),
    (
        "fixtures/storage/m5_cache_repair_cases/evidence_trace_corrupt_quarantined_for_review.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_cache_repair_cases/evidence_trace_corrupt_quarantined_for_review.yaml"
        )),
    ),
    (
        "fixtures/storage/m5_cache_repair_cases/recovery_state_torn_repair_in_place.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_cache_repair_cases/recovery_state_torn_repair_in_place.yaml"
        )),
    ),
    (
        "fixtures/storage/m5_cache_repair_cases/prebuild_missing_backing_repair_failed_fallback.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_cache_repair_cases/prebuild_missing_backing_repair_failed_fallback.yaml"
        )),
    ),
];

/// Strongly typed error returned by the corpus loader.
#[derive(Debug)]
pub enum CacheRepairLoadError {
    Yaml {
        fixture_ref: String,
        source: serde_yaml::Error,
    },
    Profiles(M5StorageGovernanceLoadError),
}

impl fmt::Display for CacheRepairLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yaml {
                fixture_ref,
                source,
            } => {
                write!(
                    f,
                    "cache-repair yaml parse error in {fixture_ref}: {source}"
                )
            }
            Self::Profiles(source) => {
                write!(f, "cache-repair runtime profile load error: {source}")
            }
        }
    }
}

impl Error for CacheRepairLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Yaml { source, .. } => Some(source),
            Self::Profiles(source) => Some(source),
        }
    }
}

impl From<M5StorageGovernanceLoadError> for CacheRepairLoadError {
    fn from(value: M5StorageGovernanceLoadError) -> Self {
        Self::Profiles(value)
    }
}

/// Loads the checked-in cache-repair plan corpus.
pub fn current_cache_repair_plan_corpus() -> Result<CacheRepairPlanCorpus, CacheRepairLoadError> {
    let mut plans = Vec::with_capacity(PLAN_FIXTURES.len());
    for (fixture_ref, yaml) in PLAN_FIXTURES {
        let plan = serde_yaml::from_str::<CacheRepairPlan>(yaml).map_err(|source| {
            CacheRepairLoadError::Yaml {
                fixture_ref: (*fixture_ref).to_owned(),
                source,
            }
        })?;
        plans.push(CacheRepairPlanEntry {
            fixture_ref: (*fixture_ref).to_owned(),
            plan,
        });
    }
    Ok(CacheRepairPlanCorpus { plans })
}

/// Loads the canonical runtime storage-class profiles the composer folds.
pub fn current_runtime_profiles() -> Result<RuntimeStorageClassProfiles, CacheRepairLoadError> {
    current_runtime_storage_class_profiles().map_err(CacheRepairLoadError::from)
}

impl CacheRepairPlanCorpus {
    /// Returns the plan with the given id, if present.
    pub fn plan(&self, plan_id: &str) -> Option<&CacheRepairPlan> {
        self.plans
            .iter()
            .find(|entry| entry.plan.plan_id == plan_id)
            .map(|entry| &entry.plan)
    }

    /// Validates every seeded plan against the safety contract, attributing each
    /// violation to its originating fixture.
    pub fn validate(&self) -> Vec<CacheRepairViolation> {
        let mut violations = Vec::new();
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for entry in &self.plans {
            if !seen.insert(entry.plan.plan_id.as_str()) {
                push(
                    &mut violations,
                    "corpus.duplicate_plan_id",
                    &entry.fixture_ref,
                    "plan_id must be unique across the corpus",
                );
            }
            entry
                .plan
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
    ) -> CacheRepairSupportExport {
        let mut plans: Vec<CacheRepairSupportExportRow> = self
            .plans
            .iter()
            .map(|entry| CacheRepairSupportExportRow::from_plan(&entry.plan))
            .collect();
        plans.sort_by(|a, b| a.plan_id.cmp(&b.plan_id));
        let plan_count = plans.len() as u32;
        let in_progress_count = self
            .plans
            .iter()
            .filter(|entry| entry.plan.repair_state == RepairStateClass::RepairInProgress)
            .count() as u32;
        let failed_count = self
            .plans
            .iter()
            .filter(|entry| entry.plan.repair_state.is_failed())
            .count() as u32;
        let quarantine_preserved_count = self
            .plans
            .iter()
            .filter(|entry| entry.plan.quarantine_disposition.preserves_suspect_copy())
            .count() as u32;
        let factory_reset_offered_count = self
            .plans
            .iter()
            .filter(|entry| entry.plan.factory_reset_offered)
            .count() as u32;
        CacheRepairSupportExport {
            record_kind: M5_CACHE_REPAIR_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_CACHE_REPAIR_SCHEMA_VERSION,
            envelope_id: envelope_id.into(),
            captured_at: captured_at.into(),
            schema_ref: M5_CACHE_REPAIR_SCHEMA_REF.to_owned(),
            doc_ref: M5_CACHE_REPAIR_DOC_REF.to_owned(),
            runtime_storage_classes_ref: RUNTIME_STORAGE_CLASSES_REF.to_owned(),
            plan_count,
            in_progress_count,
            failed_count,
            quarantine_preserved_count,
            factory_reset_offered_count,
            raw_content_exported: false,
            redaction_class: METADATA_SAFE_DEFAULT.to_owned(),
            plans,
        }
    }
}

// --------------------------------------------------------------------------
// Support-export projection.
// --------------------------------------------------------------------------

/// One metadata-safe summary row in the support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheRepairSupportExportRow {
    pub record_kind: String,
    pub plan_id: String,
    pub storage_class_id: StorageClassId,
    pub scope_class: RepairScopeClass,
    pub fault_class: FaultClass,
    pub repair_action: RepairActionClass,
    pub quarantine_disposition: QuarantineDispositionClass,
    pub repair_state: RepairStateClass,
    pub repair_label: RepairLabelClass,
    pub posture: StoragePostureClass,
    pub affected_surface_count: u32,
    pub active_label_count: u32,
    pub fallback_action: FallbackActionClass,
    pub preserves_user_owned_data: bool,
    pub preserves_forensics_value: bool,
    pub factory_reset_offered: bool,
    pub reset_everything_avoided: bool,
}

impl CacheRepairSupportExportRow {
    fn from_plan(plan: &CacheRepairPlan) -> Self {
        Self {
            record_kind: M5_CACHE_REPAIR_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            plan_id: plan.plan_id.clone(),
            storage_class_id: plan.storage_class_id,
            scope_class: plan.scope_class,
            fault_class: plan.fault_class,
            repair_action: plan.repair_action,
            quarantine_disposition: plan.quarantine_disposition,
            repair_state: plan.repair_state,
            repair_label: plan.repair_label,
            posture: plan.posture,
            affected_surface_count: plan.affected_surface_labels.len() as u32,
            active_label_count: plan.active_label_count(),
            fallback_action: plan.fallback_action,
            preserves_user_owned_data: plan.preserves_user_owned_data,
            preserves_forensics_value: plan.preserves_forensics_value,
            factory_reset_offered: plan.factory_reset_offered,
            reset_everything_avoided: plan.reset_everything_avoided,
        }
    }
}

/// The metadata-safe support-export envelope folded from the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheRepairSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub envelope_id: String,
    pub captured_at: String,
    pub schema_ref: String,
    pub doc_ref: String,
    pub runtime_storage_classes_ref: String,
    pub plan_count: u32,
    pub in_progress_count: u32,
    pub failed_count: u32,
    pub quarantine_preserved_count: u32,
    pub factory_reset_offered_count: u32,
    pub raw_content_exported: bool,
    pub redaction_class: String,
    pub plans: Vec<CacheRepairSupportExportRow>,
}

impl CacheRepairSupportExport {
    /// True when the envelope is metadata-safe, plan-complete, and offers no
    /// factory reset.
    pub fn is_export_safe(&self) -> bool {
        !self.raw_content_exported
            && self.redaction_class == METADATA_SAFE_DEFAULT
            && self.plans.len() as u32 == self.plan_count
            && self.factory_reset_offered_count == 0
    }
}

// --------------------------------------------------------------------------
// Validation.
// --------------------------------------------------------------------------

/// A validation violation surfaced by the plan harness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheRepairViolation {
    pub check_id: String,
    pub target_ref: String,
    pub message: String,
}

impl fmt::Display for CacheRepairViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}: {}",
            self.check_id, self.target_ref, self.message
        )
    }
}

fn push(
    violations: &mut Vec<CacheRepairViolation>,
    check_id: impl Into<String>,
    target_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(CacheRepairViolation {
        check_id: check_id.into(),
        target_ref: target_ref.into(),
        message: message.into(),
    });
}

// --------------------------------------------------------------------------
// Seeded signals — the inputs the dump example folds into the fixture corpus.
// --------------------------------------------------------------------------

/// The seeded repair signals the scenario corpus is composed from. Keeping the
/// signals here means the fixtures, the composer, and the tests share one
/// source of truth.
pub fn seeded_repair_signals() -> Vec<RepairSignal> {
    vec![
        // A corrupt search / graph index, reindexed in place. Disposable derived
        // cache: no quarantine copy needed. Search, graph, and symbol surfaces
        // show reindex-needed until the rebuild completes.
        RepairSignal {
            plan_id: "cache_repair.knowledge_cache_corrupt_index.v1".to_owned(),
            emitted_at: "2026-06-14T00:00:00Z".to_owned(),
            storage_class_id: StorageClassId::KnowledgeCache,
            scope_class: RepairScopeClass::SingleClassSingleWorkspace,
            workspace_ref: Some("ws.alpha".to_owned()),
            workspace_label: Some("Project Alpha".to_owned()),
            fault_class: FaultClass::CorruptIndex,
            repair_state: RepairStateClass::RepairInProgress,
            holds_user_owned_data: false,
            holds_forensics_value: false,
            quarantine_ref: None,
            affected_surfaces: vec![
                AffectedSurfaceInput {
                    surface_ref: "surface.search".to_owned(),
                    surface_label: "Search".to_owned(),
                    detail: "Search results are reindexing; partial results may be stale."
                        .to_owned(),
                },
                AffectedSurfaceInput {
                    surface_ref: "surface.code_graph".to_owned(),
                    surface_label: "Code graph".to_owned(),
                    detail: "Graph navigation is reindexing.".to_owned(),
                },
                AffectedSurfaceInput {
                    surface_ref: "surface.symbols".to_owned(),
                    surface_label: "Symbols".to_owned(),
                    detail: "Workspace symbols are reindexing.".to_owned(),
                },
            ],
        },
        // A docs / model pack failed digest verification; refetched by digest
        // across every workspace that shares the artifact cache. Admin-governed
        // but content-addressed and rebuildable: no quarantine copy needed.
        RepairSignal {
            plan_id: "cache_repair.artifact_pack_checksum_mismatch.v1".to_owned(),
            emitted_at: "2026-06-14T00:00:00Z".to_owned(),
            storage_class_id: StorageClassId::ArtifactCache,
            scope_class: RepairScopeClass::SingleClassAllWorkspaces,
            workspace_ref: None,
            workspace_label: None,
            fault_class: FaultClass::ChecksumMismatch,
            repair_state: RepairStateClass::RepairInProgress,
            holds_user_owned_data: false,
            holds_forensics_value: false,
            quarantine_ref: None,
            affected_surfaces: vec![
                AffectedSurfaceInput {
                    surface_ref: "surface.docs_viewer".to_owned(),
                    surface_label: "Docs".to_owned(),
                    detail: "The docs pack is refetching by digest.".to_owned(),
                },
                AffectedSurfaceInput {
                    surface_ref: "surface.model_runtime".to_owned(),
                    surface_label: "Model runtime".to_owned(),
                    detail: "The model pack is refetching by digest.".to_owned(),
                },
            ],
        },
        // A torn generated preview in the interactive hot cache, just detected.
        // Pure disposable: re-derive on demand, no quarantine copy needed.
        RepairSignal {
            plan_id: "cache_repair.generated_preview_torn.v1".to_owned(),
            emitted_at: "2026-06-14T00:00:00Z".to_owned(),
            storage_class_id: StorageClassId::InteractiveHotCache,
            scope_class: RepairScopeClass::SingleClassSingleWorkspace,
            workspace_ref: Some("ws.beta".to_owned()),
            workspace_label: Some("Project Beta".to_owned()),
            fault_class: FaultClass::PartialWriteTorn,
            repair_state: RepairStateClass::Detected,
            holds_user_owned_data: false,
            holds_forensics_value: false,
            quarantine_ref: None,
            affected_surfaces: vec![
                AffectedSurfaceInput {
                    surface_ref: "surface.preview_pane".to_owned(),
                    surface_label: "Preview".to_owned(),
                    detail: "The rendered preview will re-derive on next open.".to_owned(),
                },
                AffectedSurfaceInput {
                    surface_ref: "surface.diagram_render".to_owned(),
                    surface_label: "Diagram render".to_owned(),
                    detail: "The diagram render will re-derive on next open.".to_owned(),
                },
            ],
        },
        // A corrupt profiler / replay trace in the evidence cache. Forensic
        // value: the suspect copy is quarantined pending export and routed to a
        // class-specific review; it is never auto-cleared.
        RepairSignal {
            plan_id: "cache_repair.evidence_trace_corrupt.v1".to_owned(),
            emitted_at: "2026-06-14T00:00:00Z".to_owned(),
            storage_class_id: StorageClassId::EvidenceSupportCache,
            scope_class: RepairScopeClass::SingleClassSingleWorkspace,
            workspace_ref: Some("ws.alpha".to_owned()),
            workspace_label: Some("Project Alpha".to_owned()),
            fault_class: FaultClass::CorruptIndex,
            repair_state: RepairStateClass::QuarantinePreserved,
            holds_user_owned_data: false,
            holds_forensics_value: false,
            quarantine_ref: None,
            affected_surfaces: vec![
                AffectedSurfaceInput {
                    surface_ref: "surface.profiler".to_owned(),
                    surface_label: "Profiler".to_owned(),
                    detail: "The corrupt trace is quarantined for review.".to_owned(),
                },
                AffectedSurfaceInput {
                    surface_ref: "surface.replay_viewer".to_owned(),
                    surface_label: "Replay".to_owned(),
                    detail: "The replay bundle is quarantined for review.".to_owned(),
                },
                AffectedSurfaceInput {
                    surface_ref: "surface.support_center".to_owned(),
                    surface_label: "Support center".to_owned(),
                    detail: "The suspect evidence is preserved and exportable before any delete."
                        .to_owned(),
                },
            ],
        },
        // A torn dirty-buffer journal in user-owned recovery state, repaired in
        // place from its checkpoint. User-owned data: the suspect copy is
        // quarantined and never deleted; recovery is repaired in place.
        RepairSignal {
            plan_id: "cache_repair.recovery_state_torn.v1".to_owned(),
            emitted_at: "2026-06-14T00:00:00Z".to_owned(),
            storage_class_id: StorageClassId::UserOwnedRecoveryState,
            scope_class: RepairScopeClass::SingleClassSingleWorkspace,
            workspace_ref: Some("ws.gamma".to_owned()),
            workspace_label: Some("Project Gamma".to_owned()),
            fault_class: FaultClass::PartialWriteTorn,
            repair_state: RepairStateClass::RepairInProgress,
            holds_user_owned_data: false,
            holds_forensics_value: false,
            quarantine_ref: None,
            affected_surfaces: vec![
                AffectedSurfaceInput {
                    surface_ref: "surface.local_history".to_owned(),
                    surface_label: "Local history".to_owned(),
                    detail:
                        "The torn journal is quarantined; history is repaired from its checkpoint."
                            .to_owned(),
                },
                AffectedSurfaceInput {
                    surface_ref: "surface.session_restore".to_owned(),
                    surface_label: "Session restore".to_owned(),
                    detail: "Session restore is repairing from its last checkpoint.".to_owned(),
                },
            ],
        },
        // A prebuild layer is missing its backing object; the targeted refetch
        // failed, so a narrower fallback (retry) is offered — never a reset.
        RepairSignal {
            plan_id: "cache_repair.prebuild_missing_backing.v1".to_owned(),
            emitted_at: "2026-06-14T00:00:00Z".to_owned(),
            storage_class_id: StorageClassId::PrebuildEnvironmentCache,
            scope_class: RepairScopeClass::SingleClassSingleWorkspace,
            workspace_ref: Some("ws.delta".to_owned()),
            workspace_label: Some("Project Delta".to_owned()),
            fault_class: FaultClass::MissingBackingObject,
            repair_state: RepairStateClass::RepairFailedFallbackOffered,
            holds_user_owned_data: false,
            holds_forensics_value: false,
            quarantine_ref: None,
            affected_surfaces: vec![
                AffectedSurfaceInput {
                    surface_ref: "surface.prebuild_runtime".to_owned(),
                    surface_label: "Prebuild runtime".to_owned(),
                    detail: "The prebuild layer is missing; startup may widen until it is rebuilt."
                        .to_owned(),
                },
                AffectedSurfaceInput {
                    surface_ref: "surface.terminal".to_owned(),
                    surface_label: "Terminal".to_owned(),
                    detail: "The environment capsule is rebuilding.".to_owned(),
                },
            ],
        },
    ]
}
