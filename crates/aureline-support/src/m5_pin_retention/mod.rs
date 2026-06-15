//! Pin / retention managers and cleanup-history lanes for the heavy artifact
//! families the M5 depth lanes add.
//!
//! A pin / retention manager is the operator-facing object the shell shows when
//! the user asks *why is this still on disk?* For every pinned or retained
//! artifact — crash evidence, review packs, docs / model / template packs,
//! certified templates, checkpoints, and incident bundles — it states the pin
//! source, who pinned it, the expiry / policy window, the object that references
//! it, the path to unpin it, and the export path offered before any delete. The
//! sibling cleanup-history lane keeps every past eviction attributable after the
//! fact: who ran it, which class and family it touched, how many bytes it
//! reclaimed, which pins blocked it, and the stale / reindex-needed state it
//! left behind — without ever capturing a raw payload.
//!
//! This module is the canonical, inspectable truth model behind both surfaces.
//! It mints no new storage primitive: the storage-class, artifact-family, and
//! pin-source vocabularies re-export verbatim from [`crate::storage_inspector`]
//! and [`crate::m5_storage_governance`]. Only the pin-actor, retention-state,
//! unpin-path, export-path, referenced-object, and cleanup labels are
//! introduced here, and they are bounded explanatory tokens that resolve back to
//! the runtime contract at [`RUNTIME_STORAGE_CLASSES_REF`] and the frozen
//! artifact-family matrix at [`M5_ARTIFACT_FAMILY_MATRIX_REF`].
//!
//! ## What this owns
//!
//! - The [`PinRetentionManager`] record — one workspace / scope view binding its
//!   pins and its recent cleanup history to the `pin_manager` and
//!   `cleanup_history_lane` surfaces, plus the open-inspector / open-review
//!   actions. Mirrors the boundary schema at [`M5_PIN_RETENTION_SCHEMA_REF`].
//! - The [`PinRetentionEntry`] row — one pinned or retained artifact, carrying
//!   its pin source, pin actor, retention state and expiry, referenced object,
//!   unpin path, and export path.
//! - The [`CleanupHistoryEntry`] row — one past cleanup / eviction event,
//!   carrying its actor, trigger, class and family, reclaimed bytes, blocked
//!   pins, and resulting stale / reindex-needed state. It never captures raw
//!   payloads and never reports authoritative-state loss.
//! - The [`PinRetentionManagerCorpus`] container — folds every seeded scenario
//!   manager into one validated bundle, checks the cross-record safety contract
//!   (derived fields track the pin source, protected classes are never silently
//!   deleted, cleanup history stays attributable and payload-free), and projects
//!   a metadata-safe [`PinRetentionSupportExport`] the support-bundle pipeline
//!   can quote without leaking raw payloads, paths, or credentials.
//! - The [`compose_manager`] projection — the first real consumer: it folds the
//!   frozen [`M5ArtifactFamilyStorageMatrix`] plus a [`ManagerSignal`] into a
//!   manager that is correct by construction (storage class, protection, unpin
//!   path, export path, and pin actor all derive from the matrix and the pin
//!   source; no cleanup auto-deletes user-owned recovery state; no path reports
//!   authoritative loss or captures a raw payload).
//!
//! ## What this does NOT own
//!
//! - Live pinning, unpinning, byte-level eviction, or retention scheduling.
//!   Those belong to the runtime crates; this module is the shared truth model
//!   the pin manager, cleanup-history lane, storage inspector, clear-data
//!   review, and support export project.
//! - The runtime storage-class vocabulary or the artifact-family matrix, which
//!   stay frozen in their own lanes.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_storage_governance::{
    ArtifactFamilyId, DefaultRetentionClass, M5ArtifactFamilyRow, M5ArtifactFamilyStorageMatrix,
    PinSourceClass,
};
use crate::storage_inspector::StorageClassId;

#[cfg(test)]
mod tests;

/// Frozen schema version shared by every record in this module.
pub const M5_PIN_RETENTION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for a pin / retention manager.
pub const M5_PIN_RETENTION_MANAGER_RECORD_KIND: &str = "m5_pin_retention_manager";

/// Stable record-kind tag for one pin / retention entry.
pub const M5_PIN_RETENTION_PIN_RECORD_KIND: &str = "m5_pin_retention_pin";

/// Stable record-kind tag for one cleanup-history event.
pub const M5_PIN_RETENTION_CLEANUP_RECORD_KIND: &str = "m5_pin_retention_cleanup_event";

/// Stable record-kind tag for the support-export envelope.
pub const M5_PIN_RETENTION_SUPPORT_EXPORT_RECORD_KIND: &str = "m5_pin_retention_support_export";

/// Stable record-kind tag for one support-export row.
pub const M5_PIN_RETENTION_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "m5_pin_retention_support_export_row";

/// Repository-relative path of the boundary schema for the manager.
pub const M5_PIN_RETENTION_SCHEMA_REF: &str = "schemas/storage/m5_pin_retention.schema.json";

/// Repository-relative path of the reviewer contract doc every manager quotes.
pub const M5_PIN_RETENTION_DOC_REF: &str = "docs/storage/m5_pin_retention_contract.md";

/// Repository-relative path of the canonical runtime storage-class contract.
pub const RUNTIME_STORAGE_CLASSES_REF: &str = "artifacts/runtime/storage_classes.yaml";

/// Repository-relative path of the frozen artifact-family matrix the composer folds.
pub const M5_ARTIFACT_FAMILY_MATRIX_REF: &str =
    "artifacts/storage/m5_artifact_family_storage_matrix.yaml";

/// The metadata-safe redaction class every manager and export envelope carries.
pub const METADATA_SAFE_DEFAULT: &str = "metadata_safe_default";

/// The stable action id that opens the storage inspector from a manager.
pub const OPEN_STORAGE_INSPECTOR_ACTION_REF: &str = "action.storage.open_inspector";

/// The stable action id that opens the class-selective clear-data review.
pub const OPEN_CLEAR_DATA_REVIEW_ACTION_REF: &str = "action.storage.open_clear_data_review";

// --------------------------------------------------------------------------
// Closed vocabularies introduced by this lane.
//
// All of pin-actor, retention-state, unpin-path, export-path, referenced-object,
// and the cleanup labels are bounded explanatory tokens. They resolve against
// the runtime storage-class contract and the frozen pin-source vocabulary
// re-exported from m5_storage_governance.
// --------------------------------------------------------------------------

/// The actor class that established a pin — *who pinned it* — derived from the
/// pin source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PinActorClass {
    /// An explicit user pin.
    User,
    /// An explicit admin / policy pin or a last-known-good policy bundle.
    AdminPolicy,
    /// A release artifact-graph reference.
    ReleaseProcess,
    /// A support case or incident reference.
    CaseOrIncident,
    /// A review-packet reference.
    ReviewProcess,
    /// An offline-entitlement-bundle reference.
    OfflineBundle,
    /// A certified template / archetype reference.
    CertifiedTemplateSource,
    /// A support-export assembly in flight.
    SupportExportProcess,
    /// A retention-policy window.
    RetentionPolicy,
}

impl PinActorClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::AdminPolicy => "admin_policy",
            Self::ReleaseProcess => "release_process",
            Self::CaseOrIncident => "case_or_incident",
            Self::ReviewProcess => "review_process",
            Self::OfflineBundle => "offline_bundle",
            Self::CertifiedTemplateSource => "certified_template_source",
            Self::SupportExportProcess => "support_export_process",
            Self::RetentionPolicy => "retention_policy",
        }
    }
}

/// Why an entry is still retained on disk — the expiry / policy window posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionStateClass {
    /// Held by a finite retention window that has not yet elapsed; carries an
    /// `expires_at`.
    InRetentionWindow,
    /// Held indefinitely while a release / case / review / offline / support
    /// reference still names it; no expiry while the reference holds.
    PinnedIndefiniteWhileReferenced,
    /// User-owned recovery state kept until the user explicitly resets it.
    RetainedUntilExplicitReset,
    /// An explicit user pin with no automatic expiry.
    PinnedByExplicitUserChoice,
    /// An admin / policy-managed retention window with no user-set expiry.
    PolicyWindowManaged,
}

impl RetentionStateClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InRetentionWindow => "in_retention_window",
            Self::PinnedIndefiniteWhileReferenced => "pinned_indefinite_while_referenced",
            Self::RetainedUntilExplicitReset => "retained_until_explicit_reset",
            Self::PinnedByExplicitUserChoice => "pinned_by_explicit_user_choice",
            Self::PolicyWindowManaged => "policy_window_managed",
        }
    }
}

/// How a pin is released — the unpin path — derived from the pin source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnpinPathClass {
    /// The user can unpin it directly.
    UserUnpinsDirectly,
    /// Unpinning requires an admin / policy change.
    AdminPolicyChangeRequired,
    /// Unpinning means releasing the object that references it; the pin clears
    /// when the last reference is gone.
    ReleaseReferencingObject,
    /// The pin clears automatically when its retention window elapses.
    AutoUnpinsAtRetentionExpiry,
}

impl UnpinPathClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserUnpinsDirectly => "user_unpins_directly",
            Self::AdminPolicyChangeRequired => "admin_policy_change_required",
            Self::ReleaseReferencingObject => "release_referencing_object",
            Self::AutoUnpinsAtRetentionExpiry => "auto_unpins_at_retention_expiry",
        }
    }
}

/// The export-before-delete path offered for an entry, derived from the matrix
/// row and the pin source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportPathClass {
    /// Export is required before this entry may be deleted.
    ExportRequiredBeforeDelete,
    /// Export is offered before this entry may be deleted.
    ExportOfferedBeforeDelete,
    /// The entry is already captured in a support-export assembly.
    ExportAlreadyInAssembly,
}

impl ExportPathClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExportRequiredBeforeDelete => "export_required_before_delete",
            Self::ExportOfferedBeforeDelete => "export_offered_before_delete",
            Self::ExportAlreadyInAssembly => "export_already_in_assembly",
        }
    }
}

/// The class of object that references / holds a pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferencedObjectClass {
    /// A workspace artifact the user pinned directly.
    WorkspaceArtifact,
    /// A release artifact graph.
    ReleaseArtifactGraph,
    /// A support case or incident.
    SupportCaseOrIncident,
    /// A review packet.
    ReviewPacket,
    /// An offline-entitlement bundle.
    OfflineEntitlementBundle,
    /// A certified template or archetype.
    CertifiedTemplateOrArchetype,
    /// A support-export assembly in flight.
    SupportExportAssembly,
    /// A retention-policy window.
    RetentionPolicyWindow,
    /// Local history / a rollback checkpoint.
    LocalCheckpointOrHistory,
    /// An admin / policy binding.
    AdminPolicyBinding,
}

impl ReferencedObjectClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceArtifact => "workspace_artifact",
            Self::ReleaseArtifactGraph => "release_artifact_graph",
            Self::SupportCaseOrIncident => "support_case_or_incident",
            Self::ReviewPacket => "review_packet",
            Self::OfflineEntitlementBundle => "offline_entitlement_bundle",
            Self::CertifiedTemplateOrArchetype => "certified_template_or_archetype",
            Self::SupportExportAssembly => "support_export_assembly",
            Self::RetentionPolicyWindow => "retention_policy_window",
            Self::LocalCheckpointOrHistory => "local_checkpoint_or_history",
            Self::AdminPolicyBinding => "admin_policy_binding",
        }
    }
}

/// Which inspectable surface this manager binds to. Re-exports the runtime
/// `inspectable_surface_class_vocabulary` tokens this lane projects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectableSurfaceClass {
    /// The pin / retention manager surface.
    PinManager,
    /// The cleanup-history lane surface.
    CleanupHistoryLane,
}

impl InspectableSurfaceClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PinManager => "pin_manager",
            Self::CleanupHistoryLane => "cleanup_history_lane",
        }
    }
}

/// The two surfaces every manager binds, in a stable order.
const REQUIRED_SURFACES: &[InspectableSurfaceClass] = &[
    InspectableSurfaceClass::PinManager,
    InspectableSurfaceClass::CleanupHistoryLane,
];

/// Who or what ran a cleanup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CleanupActorClass {
    /// An explicit user action.
    User,
    /// An admin / policy action.
    AdminPolicy,
    /// The runtime resource governor under disk / quota pressure.
    SystemPressureGovernor,
    /// The retention scheduler expiring a window.
    RetentionScheduler,
    /// The offboarding / reset flow.
    OffboardingFlow,
}

impl CleanupActorClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::AdminPolicy => "admin_policy",
            Self::SystemPressureGovernor => "system_pressure_governor",
            Self::RetentionScheduler => "retention_scheduler",
            Self::OffboardingFlow => "offboarding_flow",
        }
    }

    /// True for the automatic (non-user) actors that may never delete
    /// user-owned recovery state.
    pub const fn is_automatic(self) -> bool {
        matches!(
            self,
            Self::SystemPressureGovernor | Self::RetentionScheduler
        )
    }
}

/// What triggered a cleanup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CleanupTriggerClass {
    /// A device low-disk floor.
    LowDiskPressure,
    /// A managed / workspace storage-quota ceiling.
    ManagedQuotaPressure,
    /// An explicit class-selective clear-data action.
    ExplicitUserClearData,
    /// An offboarding or reset flow.
    OffboardingOrReset,
    /// A retention window elapsing.
    RetentionWindowExpiry,
    /// A support case closing.
    CaseClose,
}

impl CleanupTriggerClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LowDiskPressure => "low_disk_pressure",
            Self::ManagedQuotaPressure => "managed_quota_pressure",
            Self::ExplicitUserClearData => "explicit_user_clear_data",
            Self::OffboardingOrReset => "offboarding_or_reset",
            Self::RetentionWindowExpiry => "retention_window_expiry",
            Self::CaseClose => "case_close",
        }
    }

    /// True for the automatic pressure triggers that may never delete
    /// user-owned recovery state.
    pub const fn is_pressure(self) -> bool {
        matches!(self, Self::LowDiskPressure | Self::ManagedQuotaPressure)
    }
}

/// What one cleanup did to its target class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CleanupDispositionClass {
    /// Trimmed a disposable cache that rebuilds on demand.
    TrimmedDisposableCache,
    /// Trimmed a rebuildable derived cache; the class is left rebuild-pending.
    TrimmedRebuildableCache,
    /// Trimmed only unpinned artifact / prebuild entries.
    TrimmedUnpinnedArtifact,
    /// Expired only unpinned evidence past its retention window.
    ExpiredUnpinnedEvidencePastRetention,
    /// Exported the entry, then deleted it under explicit review.
    ExportedThenDeleted,
    /// No-op: every targeted entry was pin-protected and nothing was removed.
    BlockedNoOpPinProtected,
}

impl CleanupDispositionClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrimmedDisposableCache => "trimmed_disposable_cache",
            Self::TrimmedRebuildableCache => "trimmed_rebuildable_cache",
            Self::TrimmedUnpinnedArtifact => "trimmed_unpinned_artifact",
            Self::ExpiredUnpinnedEvidencePastRetention => {
                "expired_unpinned_evidence_past_retention"
            }
            Self::ExportedThenDeleted => "exported_then_deleted",
            Self::BlockedNoOpPinProtected => "blocked_no_op_pin_protected",
        }
    }
}

/// The state a cleanup left its target class in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultingStateClass {
    /// Fully reclaimed; no residual.
    FullyReclaimedNoResidual,
    /// Left rebuild-pending; the cache will rebuild on demand.
    RebuildPending,
    /// A semantic index now needs a reindex.
    ReindexNeeded,
    /// Some entries were removed; pinned entries were retained.
    PartialRetainedPins,
    /// Authoritative state was untouched.
    AuthoritativeStateUntouched,
}

impl ResultingStateClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyReclaimedNoResidual => "fully_reclaimed_no_residual",
            Self::RebuildPending => "rebuild_pending",
            Self::ReindexNeeded => "reindex_needed",
            Self::PartialRetainedPins => "partial_retained_pins",
            Self::AuthoritativeStateUntouched => "authoritative_state_untouched",
        }
    }
}

// --------------------------------------------------------------------------
// Derivation helpers — the frozen mapping from pin source to actor, unpin path,
// and retention defaults. These resolve the frozen pin-source vocabulary; they
// mint nothing new.
// --------------------------------------------------------------------------

/// The pin actor for a pin source — *who pinned it*.
const fn pin_actor_for(source: PinSourceClass) -> PinActorClass {
    match source {
        PinSourceClass::ExplicitUserPin => PinActorClass::User,
        PinSourceClass::ExplicitAdminPolicyPin | PinSourceClass::PolicyBundleLastKnownGoodRef => {
            PinActorClass::AdminPolicy
        }
        PinSourceClass::ReleaseArtifactGraphRef => PinActorClass::ReleaseProcess,
        PinSourceClass::CaseReferenceRef => PinActorClass::CaseOrIncident,
        PinSourceClass::ReviewPackRef => PinActorClass::ReviewProcess,
        PinSourceClass::OfflineBundleRef => PinActorClass::OfflineBundle,
        PinSourceClass::CertifiedArchetypeOrTemplateRef => PinActorClass::CertifiedTemplateSource,
        PinSourceClass::SupportExportAssemblyRef => PinActorClass::SupportExportProcess,
        PinSourceClass::RetentionWindowRef => PinActorClass::RetentionPolicy,
    }
}

/// The unpin path for a pin source.
const fn unpin_path_for(source: PinSourceClass) -> UnpinPathClass {
    match source {
        PinSourceClass::ExplicitUserPin => UnpinPathClass::UserUnpinsDirectly,
        PinSourceClass::ExplicitAdminPolicyPin | PinSourceClass::PolicyBundleLastKnownGoodRef => {
            UnpinPathClass::AdminPolicyChangeRequired
        }
        PinSourceClass::RetentionWindowRef => UnpinPathClass::AutoUnpinsAtRetentionExpiry,
        PinSourceClass::ReleaseArtifactGraphRef
        | PinSourceClass::CaseReferenceRef
        | PinSourceClass::ReviewPackRef
        | PinSourceClass::OfflineBundleRef
        | PinSourceClass::CertifiedArchetypeOrTemplateRef
        | PinSourceClass::SupportExportAssemblyRef => UnpinPathClass::ReleaseReferencingObject,
    }
}

/// The export path for a pin source against the family's matrix row.
fn export_path_for(source: PinSourceClass, row: &M5ArtifactFamilyRow) -> ExportPathClass {
    if source == PinSourceClass::SupportExportAssemblyRef {
        ExportPathClass::ExportAlreadyInAssembly
    } else if row.export_before_delete_required {
        ExportPathClass::ExportRequiredBeforeDelete
    } else {
        ExportPathClass::ExportOfferedBeforeDelete
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
// Manager records.
// --------------------------------------------------------------------------

/// One pinned or retained artifact in the pin / retention manager.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinRetentionEntry {
    pub record_kind: String,
    pub pin_id: String,
    pub label: String,
    pub family_id: ArtifactFamilyId,
    pub storage_class_id: StorageClassId,
    /// The frozen pin source — *what* pins it.
    pub pin_source: PinSourceClass,
    /// The actor class — *who* pinned it — derived from the pin source.
    pub pin_actor: PinActorClass,
    /// An opaque, metadata-safe reference to the actor (role token, never PII).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_by_ref: Option<String>,
    /// The object that references / holds the pin.
    pub referenced_object_class: ReferencedObjectClass,
    /// An opaque, metadata-safe reference to the referencing object.
    pub referenced_object_ref: String,
    /// The default-retention posture re-exported from the matrix row.
    pub default_retention_class: DefaultRetentionClass,
    /// Why the entry is still retained — the expiry / policy window posture.
    pub retention_state: RetentionStateClass,
    /// When a finite retention window elapses, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// How the pin is released.
    pub unpin_path: UnpinPathClass,
    /// The export-before-delete path offered before any delete.
    pub export_path: ExportPathClass,
    /// Bytes the entry holds on disk.
    pub on_disk_bytes: u64,
    /// True for evidence and user-owned recovery classes.
    pub protected_continuity: bool,
}

impl PinRetentionEntry {
    /// Validates this entry against the derived-field and protected-class
    /// contract, attributing each violation to `target_ref`.
    fn validate_into(&self, violations: &mut Vec<PinRetentionViolation>, target_ref: &str) {
        let target = target_ref;

        if self.record_kind != M5_PIN_RETENTION_PIN_RECORD_KIND {
            push(
                violations,
                "pin.record_kind",
                target,
                "record_kind must be m5_pin_retention_pin",
            );
        }
        if self.pin_id.trim().is_empty() {
            push(violations, "pin.pin_id", target, "pin_id must be non-empty");
        }
        if self.label.trim().is_empty() {
            push(violations, "pin.label", target, "label must be non-empty");
        }
        if self.referenced_object_ref.trim().is_empty() {
            push(
                violations,
                "pin.referenced_object_ref",
                target,
                "referenced_object_ref must be non-empty",
            );
        }

        // Pin actor and unpin path are derived from the pin source.
        if self.pin_actor != pin_actor_for(self.pin_source) {
            push(
                violations,
                "pin.pin_actor",
                target,
                "pin_actor must be derived from the pin source",
            );
        }
        if self.unpin_path != unpin_path_for(self.pin_source) {
            push(
                violations,
                "pin.unpin_path",
                target,
                "unpin_path must be derived from the pin source",
            );
        }

        // Protected continuity tracks the protected storage classes exactly.
        if self.protected_continuity != is_protected_class(self.storage_class_id) {
            push(
                violations,
                "pin.protected_continuity",
                target,
                "protected_continuity must be true exactly for evidence and user-owned recovery classes",
            );
        }

        self.validate_export_path(violations, target);
        self.validate_retention_state(violations, target);
        self.validate_referenced_object(violations, target);
    }

    /// A protected entry must require export before delete (unless it is already
    /// captured in an assembly); a disposable / rebuildable entry offers it.
    fn validate_export_path(&self, violations: &mut Vec<PinRetentionViolation>, target: &str) {
        let expected = if self.pin_source == PinSourceClass::SupportExportAssemblyRef {
            ExportPathClass::ExportAlreadyInAssembly
        } else if self.protected_continuity {
            ExportPathClass::ExportRequiredBeforeDelete
        } else {
            ExportPathClass::ExportOfferedBeforeDelete
        };
        if self.export_path != expected {
            push(
                violations,
                "pin.export_path",
                target,
                "export_path must follow the protection posture and pin source",
            );
        }
    }

    /// Retention state must be consistent with the pin source and the presence
    /// of a finite expiry.
    fn validate_retention_state(&self, violations: &mut Vec<PinRetentionViolation>, target: &str) {
        // `in_retention_window` is the only state that carries an expiry.
        let has_expiry = self.expires_at.is_some();
        let in_window = self.retention_state == RetentionStateClass::InRetentionWindow;
        if in_window != has_expiry {
            push(
                violations,
                "pin.expires_at",
                target,
                "expires_at is present exactly when retention_state is in_retention_window",
            );
        }

        let admissible = match self.pin_source {
            PinSourceClass::RetentionWindowRef => {
                matches!(self.retention_state, RetentionStateClass::InRetentionWindow)
            }
            PinSourceClass::ExplicitUserPin => {
                // A user pin on user-owned recovery state reads as
                // retained-until-reset; anywhere else as an explicit user choice.
                if self.storage_class_id == StorageClassId::UserOwnedRecoveryState {
                    matches!(
                        self.retention_state,
                        RetentionStateClass::RetainedUntilExplicitReset
                            | RetentionStateClass::PinnedByExplicitUserChoice
                    )
                } else {
                    matches!(
                        self.retention_state,
                        RetentionStateClass::PinnedByExplicitUserChoice
                    )
                }
            }
            PinSourceClass::ExplicitAdminPolicyPin
            | PinSourceClass::PolicyBundleLastKnownGoodRef => {
                matches!(
                    self.retention_state,
                    RetentionStateClass::PolicyWindowManaged
                )
            }
            PinSourceClass::ReleaseArtifactGraphRef
            | PinSourceClass::CaseReferenceRef
            | PinSourceClass::ReviewPackRef
            | PinSourceClass::OfflineBundleRef
            | PinSourceClass::CertifiedArchetypeOrTemplateRef
            | PinSourceClass::SupportExportAssemblyRef => matches!(
                self.retention_state,
                RetentionStateClass::PinnedIndefiniteWhileReferenced
            ),
        };
        if !admissible {
            push(
                violations,
                "pin.retention_state",
                target,
                "retention_state must be consistent with the pin source",
            );
        }
    }

    /// The referenced object class must be consistent with the pin source.
    fn validate_referenced_object(
        &self,
        violations: &mut Vec<PinRetentionViolation>,
        target: &str,
    ) {
        let admissible = match self.pin_source {
            PinSourceClass::ExplicitUserPin => matches!(
                self.referenced_object_class,
                ReferencedObjectClass::WorkspaceArtifact
                    | ReferencedObjectClass::LocalCheckpointOrHistory
            ),
            PinSourceClass::ExplicitAdminPolicyPin
            | PinSourceClass::PolicyBundleLastKnownGoodRef => matches!(
                self.referenced_object_class,
                ReferencedObjectClass::AdminPolicyBinding
            ),
            PinSourceClass::ReleaseArtifactGraphRef => matches!(
                self.referenced_object_class,
                ReferencedObjectClass::ReleaseArtifactGraph
            ),
            PinSourceClass::CaseReferenceRef => matches!(
                self.referenced_object_class,
                ReferencedObjectClass::SupportCaseOrIncident
            ),
            PinSourceClass::ReviewPackRef => {
                matches!(
                    self.referenced_object_class,
                    ReferencedObjectClass::ReviewPacket
                )
            }
            PinSourceClass::OfflineBundleRef => matches!(
                self.referenced_object_class,
                ReferencedObjectClass::OfflineEntitlementBundle
            ),
            PinSourceClass::CertifiedArchetypeOrTemplateRef => matches!(
                self.referenced_object_class,
                ReferencedObjectClass::CertifiedTemplateOrArchetype
            ),
            PinSourceClass::SupportExportAssemblyRef => matches!(
                self.referenced_object_class,
                ReferencedObjectClass::SupportExportAssembly
            ),
            PinSourceClass::RetentionWindowRef => matches!(
                self.referenced_object_class,
                ReferencedObjectClass::RetentionPolicyWindow
            ),
        };
        if !admissible {
            push(
                violations,
                "pin.referenced_object_class",
                target,
                "referenced_object_class must be consistent with the pin source",
            );
        }
    }
}

/// One past cleanup / eviction event in the cleanup-history lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanupHistoryEntry {
    pub record_kind: String,
    pub event_id: String,
    pub occurred_at: String,
    pub actor_class: CleanupActorClass,
    /// An opaque, metadata-safe reference to the actor (role token, never PII).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_ref: Option<String>,
    pub trigger_class: CleanupTriggerClass,
    pub family_id: ArtifactFamilyId,
    pub storage_class_id: StorageClassId,
    pub disposition: CleanupDispositionClass,
    /// Bytes this cleanup reclaimed.
    pub reclaimed_bytes: u64,
    /// How many pinned entries blocked this cleanup.
    pub blocked_pin_count: u32,
    /// The pin sources that blocked the cleanup, in a stable order.
    #[serde(default)]
    pub blocked_pin_sources: Vec<PinSourceClass>,
    /// The state the cleanup left its class in.
    pub resulting_state: ResultingStateClass,
    /// True when the class now needs a reindex.
    pub reindex_needed: bool,
    /// Always false: cleanup never touches authoritative state.
    pub authoritative_state_touched: bool,
    /// Always false: cleanup history never captures a raw payload.
    pub raw_payload_captured: bool,
}

impl CleanupHistoryEntry {
    /// Validates this event against the attributability and no-authoritative-loss
    /// contract, attributing each violation to `target_ref`.
    fn validate_into(&self, violations: &mut Vec<PinRetentionViolation>, target_ref: &str) {
        let target = target_ref;

        if self.record_kind != M5_PIN_RETENTION_CLEANUP_RECORD_KIND {
            push(
                violations,
                "cleanup.record_kind",
                target,
                "record_kind must be m5_pin_retention_cleanup_event",
            );
        }
        if self.event_id.trim().is_empty() {
            push(
                violations,
                "cleanup.event_id",
                target,
                "event_id must be non-empty",
            );
        }
        if self.occurred_at.trim().is_empty() {
            push(
                violations,
                "cleanup.occurred_at",
                target,
                "occurred_at must be non-empty",
            );
        }

        // Cleanup never touches authoritative state and never captures payloads.
        if self.authoritative_state_touched {
            push(
                violations,
                "cleanup.authoritative_state_touched",
                target,
                "authoritative_state_touched must be false",
            );
        }
        if self.raw_payload_captured {
            push(
                violations,
                "cleanup.raw_payload_captured",
                target,
                "raw_payload_captured must be false",
            );
        }

        // Blocked-pin count and sources must agree.
        if self.blocked_pin_count < self.blocked_pin_sources.len() as u32 {
            push(
                violations,
                "cleanup.blocked_pin_count",
                target,
                "blocked_pin_count must be at least the number of blocked pin sources",
            );
        }
        if (self.blocked_pin_count > 0) == self.blocked_pin_sources.is_empty() {
            push(
                violations,
                "cleanup.blocked_pin_sources",
                target,
                "blocked_pin_sources is non-empty exactly when a pin blocked the cleanup",
            );
        }

        // reindex_needed tracks the resulting state.
        if self.reindex_needed != (self.resulting_state == ResultingStateClass::ReindexNeeded) {
            push(
                violations,
                "cleanup.reindex_needed",
                target,
                "reindex_needed is true exactly when resulting_state is reindex_needed",
            );
        }

        self.validate_disposition(violations, target);
        self.validate_user_owned_recovery_guard(violations, target);
    }

    /// Each disposition constrains the class, trigger, reclaimed bytes, and
    /// resulting state.
    fn validate_disposition(&self, violations: &mut Vec<PinRetentionViolation>, target: &str) {
        match self.disposition {
            CleanupDispositionClass::BlockedNoOpPinProtected => {
                if self.reclaimed_bytes != 0 {
                    push(
                        violations,
                        "cleanup.blocked.bytes",
                        target,
                        "a blocked no-op must reclaim zero bytes",
                    );
                }
                if self.blocked_pin_count == 0 {
                    push(
                        violations,
                        "cleanup.blocked.pins",
                        target,
                        "a blocked no-op must record at least one blocking pin",
                    );
                }
                if !matches!(
                    self.resulting_state,
                    ResultingStateClass::PartialRetainedPins
                        | ResultingStateClass::AuthoritativeStateUntouched
                ) {
                    push(
                        violations,
                        "cleanup.blocked.state",
                        target,
                        "a blocked no-op leaves partial_retained_pins or authoritative_state_untouched",
                    );
                }
            }
            CleanupDispositionClass::ExpiredUnpinnedEvidencePastRetention => {
                if self.storage_class_id != StorageClassId::EvidenceSupportCache {
                    push(
                        violations,
                        "cleanup.evidence.class",
                        target,
                        "evidence expiry only targets the evidence support cache",
                    );
                }
                if !matches!(
                    self.trigger_class,
                    CleanupTriggerClass::RetentionWindowExpiry
                        | CleanupTriggerClass::LowDiskPressure
                        | CleanupTriggerClass::ManagedQuotaPressure
                ) {
                    push(
                        violations,
                        "cleanup.evidence.trigger",
                        target,
                        "evidence expiry only fires on retention expiry or storage pressure",
                    );
                }
            }
            CleanupDispositionClass::ExportedThenDeleted => {
                if !matches!(
                    self.trigger_class,
                    CleanupTriggerClass::ExplicitUserClearData
                        | CleanupTriggerClass::OffboardingOrReset
                        | CleanupTriggerClass::CaseClose
                ) {
                    push(
                        violations,
                        "cleanup.export_delete.trigger",
                        target,
                        "exported-then-deleted only fires under an explicit reviewed action",
                    );
                }
            }
            CleanupDispositionClass::TrimmedRebuildableCache => {
                if !matches!(
                    self.resulting_state,
                    ResultingStateClass::RebuildPending | ResultingStateClass::ReindexNeeded
                ) {
                    push(
                        violations,
                        "cleanup.rebuildable.state",
                        target,
                        "a rebuildable trim leaves rebuild_pending or reindex_needed",
                    );
                }
            }
            CleanupDispositionClass::TrimmedDisposableCache
            | CleanupDispositionClass::TrimmedUnpinnedArtifact => {}
        }
    }

    /// User-owned recovery state may only lose bytes under an explicit,
    /// export-first user action — never silently under pressure or scheduling.
    fn validate_user_owned_recovery_guard(
        &self,
        violations: &mut Vec<PinRetentionViolation>,
        target: &str,
    ) {
        if self.storage_class_id != StorageClassId::UserOwnedRecoveryState {
            // Automatic pressure must never reclaim authoritative recovery bytes,
            // and the only recovery-targeting cleanup that may reclaim bytes is
            // an explicit, exported-then-deleted user action — both checked below
            // for the recovery class. Nothing to enforce for other classes here.
            return;
        }
        let reclaims = self.reclaimed_bytes > 0;
        let explicit_export_delete = self.actor_class == CleanupActorClass::User
            && matches!(
                self.trigger_class,
                CleanupTriggerClass::ExplicitUserClearData
                    | CleanupTriggerClass::OffboardingOrReset
            )
            && self.disposition == CleanupDispositionClass::ExportedThenDeleted;
        if reclaims && !explicit_export_delete {
            push(
                violations,
                "cleanup.recovery.silent_delete",
                target,
                "user-owned recovery state may only be deleted by an explicit, exported user action",
            );
        }
        if self.trigger_class.is_pressure() && reclaims {
            push(
                violations,
                "cleanup.recovery.pressure_delete",
                target,
                "storage pressure must never reclaim user-owned recovery bytes",
            );
        }
    }
}

/// A pin / retention manager: the pins and recent cleanup history for one scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinRetentionManager {
    pub record_kind: String,
    pub schema_version: u32,
    pub manager_id: String,
    pub emitted_at: String,
    pub scope_ref: String,
    pub scope_label: String,
    /// The inspectable surfaces this manager binds.
    pub surfaces: Vec<InspectableSurfaceClass>,
    /// The pinned or retained artifacts.
    pub pins: Vec<PinRetentionEntry>,
    /// The recent cleanup history, newest-attributable last.
    pub cleanup_history: Vec<CleanupHistoryEntry>,
    pub open_inspector_action_ref: String,
    pub open_clear_data_review_action_ref: String,
    pub schema_ref: String,
    pub doc_ref: String,
}

impl PinRetentionManager {
    /// Returns the pin with the given id, if present.
    pub fn pin(&self, pin_id: &str) -> Option<&PinRetentionEntry> {
        self.pins.iter().find(|pin| pin.pin_id == pin_id)
    }

    /// Returns the cleanup event with the given id, if present.
    pub fn cleanup_event(&self, event_id: &str) -> Option<&CleanupHistoryEntry> {
        self.cleanup_history
            .iter()
            .find(|event| event.event_id == event_id)
    }

    /// Total bytes reclaimed across every cleanup event.
    pub fn total_reclaimed_bytes(&self) -> u64 {
        self.cleanup_history
            .iter()
            .fold(0u64, |acc, event| acc.saturating_add(event.reclaimed_bytes))
    }

    /// Number of pins that protect evidence or user-owned recovery state.
    pub fn protected_pin_count(&self) -> u32 {
        self.pins
            .iter()
            .filter(|pin| pin.protected_continuity)
            .count() as u32
    }

    /// True when no cleanup touched authoritative state or captured a payload.
    pub fn is_export_safe(&self) -> bool {
        self.cleanup_history
            .iter()
            .all(|event| !event.authoritative_state_touched && !event.raw_payload_captured)
    }

    /// Validates this manager against the pin / cleanup contract, attributing
    /// each violation to `target_ref`.
    pub fn validate_into(&self, violations: &mut Vec<PinRetentionViolation>, target_ref: &str) {
        let target = target_ref;

        if self.schema_version != M5_PIN_RETENTION_SCHEMA_VERSION {
            push(
                violations,
                "manager.schema_version",
                target,
                "schema_version must be 1",
            );
        }
        if self.record_kind != M5_PIN_RETENTION_MANAGER_RECORD_KIND {
            push(
                violations,
                "manager.record_kind",
                target,
                "record_kind must be m5_pin_retention_manager",
            );
        }
        if self.schema_ref != M5_PIN_RETENTION_SCHEMA_REF {
            push(
                violations,
                "manager.schema_ref",
                target,
                "schema_ref must pin the boundary schema",
            );
        }
        if self.doc_ref != M5_PIN_RETENTION_DOC_REF {
            push(
                violations,
                "manager.doc_ref",
                target,
                "doc_ref must pin the contract doc",
            );
        }
        if self.manager_id.trim().is_empty() {
            push(
                violations,
                "manager.manager_id",
                target,
                "manager_id must be non-empty",
            );
        }
        if self.scope_ref.trim().is_empty() {
            push(
                violations,
                "manager.scope_ref",
                target,
                "scope_ref must be non-empty",
            );
        }
        if self.open_inspector_action_ref != OPEN_STORAGE_INSPECTOR_ACTION_REF {
            push(
                violations,
                "manager.open_inspector_action_ref",
                target,
                "open_inspector_action_ref must offer the inspector action",
            );
        }
        if self.open_clear_data_review_action_ref != OPEN_CLEAR_DATA_REVIEW_ACTION_REF {
            push(
                violations,
                "manager.open_clear_data_review_action_ref",
                target,
                "open_clear_data_review_action_ref must offer the review action",
            );
        }

        // The manager binds both the pin-manager and cleanup-history surfaces.
        for required in REQUIRED_SURFACES {
            if !self.surfaces.contains(required) {
                push(
                    violations,
                    "manager.surfaces",
                    target,
                    "surfaces must bind both pin_manager and cleanup_history_lane",
                );
            }
        }

        // Pin ids and event ids are unique within a manager.
        let mut seen_pins: BTreeSet<&str> = BTreeSet::new();
        for pin in &self.pins {
            if !seen_pins.insert(pin.pin_id.as_str()) {
                push(
                    violations,
                    "manager.duplicate_pin_id",
                    target,
                    "pin_id must be unique within a manager",
                );
            }
            pin.validate_into(violations, target);
        }
        let mut seen_events: BTreeSet<&str> = BTreeSet::new();
        for event in &self.cleanup_history {
            if !seen_events.insert(event.event_id.as_str()) {
                push(
                    violations,
                    "manager.duplicate_event_id",
                    target,
                    "event_id must be unique within a manager",
                );
            }
            event.validate_into(violations, target);
        }
    }

    /// Convenience: true when this manager validates with zero violations.
    pub fn is_valid(&self) -> bool {
        let mut violations = Vec::new();
        self.validate_into(&mut violations, &self.manager_id);
        violations.is_empty()
    }
}

// --------------------------------------------------------------------------
// Matrix-backed composer — the first real consumer.
// --------------------------------------------------------------------------

/// One pin the composer folds into a [`PinRetentionEntry`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinInput {
    pub pin_id: String,
    pub label: String,
    pub family_id: ArtifactFamilyId,
    pub pin_source: PinSourceClass,
    pub referenced_object_class: ReferencedObjectClass,
    pub referenced_object_ref: String,
    pub retention_state: RetentionStateClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_by_ref: Option<String>,
    pub on_disk_bytes: u64,
}

/// One cleanup the composer folds into a [`CleanupHistoryEntry`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanupInput {
    pub event_id: String,
    pub occurred_at: String,
    pub actor_class: CleanupActorClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_ref: Option<String>,
    pub trigger_class: CleanupTriggerClass,
    pub family_id: ArtifactFamilyId,
    pub disposition: CleanupDispositionClass,
    pub reclaimed_bytes: u64,
    #[serde(default)]
    pub blocked_pin_sources: Vec<PinSourceClass>,
    pub resulting_state: ResultingStateClass,
}

/// The manager signal [`compose_manager`] folds into a manager.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagerSignal {
    pub manager_id: String,
    pub emitted_at: String,
    pub scope_ref: String,
    pub scope_label: String,
    #[serde(default)]
    pub pins: Vec<PinInput>,
    #[serde(default)]
    pub cleanups: Vec<CleanupInput>,
}

/// Folds the frozen artifact-family matrix plus a manager signal into a manager
/// that is correct by construction: every entry's storage class, protection,
/// pin actor, unpin path, and export path derive from the matrix and the pin
/// source; no cleanup auto-deletes user-owned recovery state; and no path
/// reports authoritative loss or captures a raw payload.
///
/// The `matrix` argument anchors the manager to the same frozen storage classes
/// and pin sources the storage-governance lane validates; the composer reads no
/// private mapping of its own.
pub fn compose_manager(
    matrix: &M5ArtifactFamilyStorageMatrix,
    signal: &ManagerSignal,
) -> PinRetentionManager {
    let pins = signal
        .pins
        .iter()
        .map(|input| compose_pin(matrix, input))
        .collect();
    let cleanup_history = signal
        .cleanups
        .iter()
        .map(|input| compose_cleanup(matrix, input))
        .collect();

    PinRetentionManager {
        record_kind: M5_PIN_RETENTION_MANAGER_RECORD_KIND.to_owned(),
        schema_version: M5_PIN_RETENTION_SCHEMA_VERSION,
        manager_id: signal.manager_id.clone(),
        emitted_at: signal.emitted_at.clone(),
        scope_ref: signal.scope_ref.clone(),
        scope_label: signal.scope_label.clone(),
        surfaces: REQUIRED_SURFACES.to_vec(),
        pins,
        cleanup_history,
        open_inspector_action_ref: OPEN_STORAGE_INSPECTOR_ACTION_REF.to_owned(),
        open_clear_data_review_action_ref: OPEN_CLEAR_DATA_REVIEW_ACTION_REF.to_owned(),
        schema_ref: M5_PIN_RETENTION_SCHEMA_REF.to_owned(),
        doc_ref: M5_PIN_RETENTION_DOC_REF.to_owned(),
    }
}

/// Composes one pin entry from an input against the family's matrix row.
fn compose_pin(matrix: &M5ArtifactFamilyStorageMatrix, input: &PinInput) -> PinRetentionEntry {
    let row = matrix.family(input.family_id);
    let storage_class_id = row
        .map(|row| row.storage_class_id)
        .unwrap_or_else(|| default_family_storage_class(input.family_id));
    let default_retention_class = row
        .map(|row| row.default_retention_class)
        .unwrap_or(DefaultRetentionClass::EvictUnderPressureIfUnpinned);
    let protected_continuity = is_protected_class(storage_class_id);
    let export_path = match row {
        Some(row) => export_path_for(input.pin_source, row),
        None => {
            if input.pin_source == PinSourceClass::SupportExportAssemblyRef {
                ExportPathClass::ExportAlreadyInAssembly
            } else if protected_continuity {
                ExportPathClass::ExportRequiredBeforeDelete
            } else {
                ExportPathClass::ExportOfferedBeforeDelete
            }
        }
    };

    PinRetentionEntry {
        record_kind: M5_PIN_RETENTION_PIN_RECORD_KIND.to_owned(),
        pin_id: input.pin_id.clone(),
        label: input.label.clone(),
        family_id: input.family_id,
        storage_class_id,
        pin_source: input.pin_source,
        pin_actor: pin_actor_for(input.pin_source),
        pinned_by_ref: input.pinned_by_ref.clone(),
        referenced_object_class: input.referenced_object_class,
        referenced_object_ref: input.referenced_object_ref.clone(),
        default_retention_class,
        retention_state: input.retention_state,
        expires_at: input.expires_at.clone(),
        unpin_path: unpin_path_for(input.pin_source),
        export_path,
        on_disk_bytes: input.on_disk_bytes,
        protected_continuity,
    }
}

/// Composes one cleanup event from an input against the family's matrix row.
fn compose_cleanup(
    matrix: &M5ArtifactFamilyStorageMatrix,
    input: &CleanupInput,
) -> CleanupHistoryEntry {
    let storage_class_id = matrix
        .family(input.family_id)
        .map(|row| row.storage_class_id)
        .unwrap_or_else(|| default_family_storage_class(input.family_id));
    CleanupHistoryEntry {
        record_kind: M5_PIN_RETENTION_CLEANUP_RECORD_KIND.to_owned(),
        event_id: input.event_id.clone(),
        occurred_at: input.occurred_at.clone(),
        actor_class: input.actor_class,
        actor_ref: input.actor_ref.clone(),
        trigger_class: input.trigger_class,
        family_id: input.family_id,
        storage_class_id,
        disposition: input.disposition,
        reclaimed_bytes: input.reclaimed_bytes,
        blocked_pin_count: input.blocked_pin_sources.len() as u32,
        blocked_pin_sources: input.blocked_pin_sources.clone(),
        resulting_state: input.resulting_state,
        reindex_needed: input.resulting_state == ResultingStateClass::ReindexNeeded,
        authoritative_state_touched: false,
        raw_payload_captured: false,
    }
}

/// The frozen storage class for a family, used only as a defensive fallback when
/// the matrix lookup misses (it never does for a complete matrix).
const fn default_family_storage_class(family_id: ArtifactFamilyId) -> StorageClassId {
    match family_id {
        ArtifactFamilyId::GeneratedPreview => StorageClassId::InteractiveHotCache,
        ArtifactFamilyId::NotebookOutput
        | ArtifactFamilyId::DocsPack
        | ArtifactFamilyId::ModelPack
        | ArtifactFamilyId::TemplatePack
        | ArtifactFamilyId::ExtensionDownload => StorageClassId::ArtifactCache,
        ArtifactFamilyId::PrebuildLayer => StorageClassId::PrebuildEnvironmentCache,
        ArtifactFamilyId::ProfilerTrace
        | ArtifactFamilyId::ReplayBundle
        | ArtifactFamilyId::SupportArtifact
        | ArtifactFamilyId::ReviewIncidentEvidence => StorageClassId::EvidenceSupportCache,
        ArtifactFamilyId::UserOwnedRecoveryState => StorageClassId::UserOwnedRecoveryState,
    }
}

// --------------------------------------------------------------------------
// Corpus container, entries, and loaders.
// --------------------------------------------------------------------------

/// One manager fixture paired with its repository-relative path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinRetentionManagerEntry {
    pub fixture_ref: String,
    pub manager: PinRetentionManager,
}

/// Pin / retention manager corpus loaded from the checked-in fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinRetentionManagerCorpus {
    pub managers: Vec<PinRetentionManagerEntry>,
}

const MANAGER_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/storage/m5_pin_retention_cases/evidence_and_checkpoint_pins.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_pin_retention_cases/evidence_and_checkpoint_pins.yaml"
        )),
    ),
    (
        "fixtures/storage/m5_pin_retention_cases/offline_packs_and_certified_templates.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_pin_retention_cases/offline_packs_and_certified_templates.yaml"
        )),
    ),
    (
        "fixtures/storage/m5_pin_retention_cases/cleanup_history_blocked_by_pins.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_pin_retention_cases/cleanup_history_blocked_by_pins.yaml"
        )),
    ),
    (
        "fixtures/storage/m5_pin_retention_cases/managed_quota_preserves_user_owned_state.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/m5_pin_retention_cases/managed_quota_preserves_user_owned_state.yaml"
        )),
    ),
];

/// Strongly typed error returned by the corpus loader.
#[derive(Debug)]
pub enum PinRetentionLoadError {
    Yaml {
        fixture_ref: String,
        source: serde_yaml::Error,
    },
}

impl fmt::Display for PinRetentionLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yaml {
                fixture_ref,
                source,
            } => write!(
                f,
                "pin-retention yaml parse error in {fixture_ref}: {source}"
            ),
        }
    }
}

impl Error for PinRetentionLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Yaml { source, .. } => Some(source),
        }
    }
}

/// Loads the checked-in pin / retention manager corpus.
pub fn current_pin_retention_manager_corpus(
) -> Result<PinRetentionManagerCorpus, PinRetentionLoadError> {
    let mut managers = Vec::with_capacity(MANAGER_FIXTURES.len());
    for (fixture_ref, yaml) in MANAGER_FIXTURES {
        let manager = serde_yaml::from_str::<PinRetentionManager>(yaml).map_err(|source| {
            PinRetentionLoadError::Yaml {
                fixture_ref: (*fixture_ref).to_owned(),
                source,
            }
        })?;
        managers.push(PinRetentionManagerEntry {
            fixture_ref: (*fixture_ref).to_owned(),
            manager,
        });
    }
    Ok(PinRetentionManagerCorpus { managers })
}

/// The canonical scenario signals the seeded corpus is composed from. The
/// dump example and the corpus replay test both fold these through
/// [`compose_manager`] so the checked-in fixtures can never drift from the
/// composer.
pub fn seeded_manager_signals() -> Vec<ManagerSignal> {
    vec![
        evidence_and_checkpoint_signal(),
        offline_packs_signal(),
        cleanup_blocked_signal(),
        managed_quota_signal(),
    ]
}

impl PinRetentionManagerCorpus {
    /// Returns the manager with the given id, if present.
    pub fn manager(&self, manager_id: &str) -> Option<&PinRetentionManager> {
        self.managers
            .iter()
            .find(|entry| entry.manager.manager_id == manager_id)
            .map(|entry| &entry.manager)
    }

    /// Validates every seeded manager against the safety contract, attributing
    /// each violation to its originating fixture.
    pub fn validate(&self) -> Vec<PinRetentionViolation> {
        let mut violations = Vec::new();
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for entry in &self.managers {
            if !seen.insert(entry.manager.manager_id.as_str()) {
                push(
                    &mut violations,
                    "corpus.duplicate_manager_id",
                    &entry.fixture_ref,
                    "manager_id must be unique across the corpus",
                );
            }
            entry
                .manager
                .validate_into(&mut violations, &entry.fixture_ref);
        }
        violations
    }

    /// Projects the corpus into a metadata-safe support / export envelope the
    /// support-bundle pipeline can quote without leaking raw payloads.
    pub fn support_export(
        &self,
        envelope_id: impl Into<String>,
        captured_at: impl Into<String>,
    ) -> PinRetentionSupportExport {
        let mut managers: Vec<PinRetentionSupportExportRow> = self
            .managers
            .iter()
            .map(|entry| PinRetentionSupportExportRow::from_manager(&entry.manager))
            .collect();
        managers.sort_by(|a, b| a.manager_id.cmp(&b.manager_id));

        let pin_count = managers.iter().map(|row| row.pin_count).sum();
        let cleanup_event_count = managers.iter().map(|row| row.cleanup_event_count).sum();
        let blocked_pin_event_count = managers.iter().map(|row| row.blocked_pin_event_count).sum();
        let authoritative_state_loss_count = self
            .managers
            .iter()
            .flat_map(|entry| entry.manager.cleanup_history.iter())
            .filter(|event| event.authoritative_state_touched)
            .count() as u32;
        let raw_payload_capture_count = self
            .managers
            .iter()
            .flat_map(|entry| entry.manager.cleanup_history.iter())
            .filter(|event| event.raw_payload_captured)
            .count() as u32;

        PinRetentionSupportExport {
            record_kind: M5_PIN_RETENTION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_PIN_RETENTION_SCHEMA_VERSION,
            envelope_id: envelope_id.into(),
            captured_at: captured_at.into(),
            schema_ref: M5_PIN_RETENTION_SCHEMA_REF.to_owned(),
            doc_ref: M5_PIN_RETENTION_DOC_REF.to_owned(),
            runtime_storage_classes_ref: RUNTIME_STORAGE_CLASSES_REF.to_owned(),
            matrix_ref: M5_ARTIFACT_FAMILY_MATRIX_REF.to_owned(),
            manager_count: managers.len() as u32,
            pin_count,
            cleanup_event_count,
            blocked_pin_event_count,
            authoritative_state_loss_count,
            raw_payload_capture_count,
            raw_content_exported: false,
            redaction_class: METADATA_SAFE_DEFAULT.to_owned(),
            managers,
        }
    }
}

// --------------------------------------------------------------------------
// Support-export projection.
// --------------------------------------------------------------------------

/// One metadata-safe summary row in the support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinRetentionSupportExportRow {
    pub record_kind: String,
    pub manager_id: String,
    pub scope_ref: String,
    pub pin_count: u32,
    pub protected_pin_count: u32,
    pub cleanup_event_count: u32,
    pub total_reclaimed_bytes: u64,
    pub blocked_pin_event_count: u32,
    pub reindex_needed_event_count: u32,
    pub authoritative_state_touched: bool,
    pub raw_payload_captured: bool,
}

impl PinRetentionSupportExportRow {
    fn from_manager(manager: &PinRetentionManager) -> Self {
        Self {
            record_kind: M5_PIN_RETENTION_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            manager_id: manager.manager_id.clone(),
            scope_ref: manager.scope_ref.clone(),
            pin_count: manager.pins.len() as u32,
            protected_pin_count: manager.protected_pin_count(),
            cleanup_event_count: manager.cleanup_history.len() as u32,
            total_reclaimed_bytes: manager.total_reclaimed_bytes(),
            blocked_pin_event_count: manager
                .cleanup_history
                .iter()
                .filter(|event| event.blocked_pin_count > 0)
                .count() as u32,
            reindex_needed_event_count: manager
                .cleanup_history
                .iter()
                .filter(|event| event.reindex_needed)
                .count() as u32,
            authoritative_state_touched: manager
                .cleanup_history
                .iter()
                .any(|event| event.authoritative_state_touched),
            raw_payload_captured: manager
                .cleanup_history
                .iter()
                .any(|event| event.raw_payload_captured),
        }
    }
}

/// The metadata-safe support-export envelope folded from the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinRetentionSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub envelope_id: String,
    pub captured_at: String,
    pub schema_ref: String,
    pub doc_ref: String,
    pub runtime_storage_classes_ref: String,
    pub matrix_ref: String,
    pub manager_count: u32,
    pub pin_count: u32,
    pub cleanup_event_count: u32,
    pub blocked_pin_event_count: u32,
    pub authoritative_state_loss_count: u32,
    pub raw_payload_capture_count: u32,
    pub raw_content_exported: bool,
    pub redaction_class: String,
    pub managers: Vec<PinRetentionSupportExportRow>,
}

impl PinRetentionSupportExport {
    /// True when the envelope is metadata-safe, manager-complete, and reports no
    /// authoritative state loss or raw-payload capture.
    pub fn is_export_safe(&self) -> bool {
        !self.raw_content_exported
            && self.redaction_class == METADATA_SAFE_DEFAULT
            && self.managers.len() as u32 == self.manager_count
            && self.authoritative_state_loss_count == 0
            && self.raw_payload_capture_count == 0
    }
}

// --------------------------------------------------------------------------
// Validation.
// --------------------------------------------------------------------------

/// A validation violation surfaced by the manager harness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinRetentionViolation {
    pub check_id: String,
    pub target_ref: String,
    pub message: String,
}

impl fmt::Display for PinRetentionViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}: {}",
            self.check_id, self.target_ref, self.message
        )
    }
}

fn push(
    violations: &mut Vec<PinRetentionViolation>,
    check_id: impl Into<String>,
    target_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(PinRetentionViolation {
        check_id: check_id.into(),
        target_ref: target_ref.into(),
        message: message.into(),
    });
}

// --------------------------------------------------------------------------
// Seeded scenario signals.
//
// These are the canonical inputs the checked-in fixtures are composed from. They
// cover the families named in the row: crash / review / incident evidence, docs
// / model / template packs, certified templates, checkpoints, and the cleanup
// history that keeps eviction attributable.
// --------------------------------------------------------------------------

fn evidence_and_checkpoint_signal() -> ManagerSignal {
    ManagerSignal {
        manager_id: "pin_retention.evidence_and_checkpoints.v1".to_owned(),
        emitted_at: "2026-06-14T00:00:00Z".to_owned(),
        scope_ref: "ws.alpha".to_owned(),
        scope_label: "Project Alpha".to_owned(),
        pins: vec![
            PinInput {
                pin_id: "pin.review_evidence.alpha".to_owned(),
                label: "Incident review packet — auth regression".to_owned(),
                family_id: ArtifactFamilyId::ReviewIncidentEvidence,
                pin_source: PinSourceClass::ReviewPackRef,
                referenced_object_class: ReferencedObjectClass::ReviewPacket,
                referenced_object_ref: "review_pack.auth_regression.v3".to_owned(),
                retention_state: RetentionStateClass::PinnedIndefiniteWhileReferenced,
                expires_at: None,
                pinned_by_ref: Some("role.reviewer".to_owned()),
                on_disk_bytes: 420_000_000,
            },
            PinInput {
                pin_id: "pin.crash_evidence.alpha".to_owned(),
                label: "Crash evidence — profiler trace".to_owned(),
                family_id: ArtifactFamilyId::ProfilerTrace,
                pin_source: PinSourceClass::CaseReferenceRef,
                referenced_object_class: ReferencedObjectClass::SupportCaseOrIncident,
                referenced_object_ref: "case.crash.2026_0612".to_owned(),
                retention_state: RetentionStateClass::PinnedIndefiniteWhileReferenced,
                expires_at: None,
                pinned_by_ref: None,
                on_disk_bytes: 880_000_000,
            },
            PinInput {
                pin_id: "pin.support_bundle.alpha".to_owned(),
                label: "Support export assembly in flight".to_owned(),
                family_id: ArtifactFamilyId::SupportArtifact,
                pin_source: PinSourceClass::SupportExportAssemblyRef,
                referenced_object_class: ReferencedObjectClass::SupportExportAssembly,
                referenced_object_ref: "support_export.assembly.pending".to_owned(),
                retention_state: RetentionStateClass::PinnedIndefiniteWhileReferenced,
                expires_at: None,
                pinned_by_ref: None,
                on_disk_bytes: 130_000_000,
            },
            PinInput {
                pin_id: "pin.replay_retention.alpha".to_owned(),
                label: "Replay bundle under retention window".to_owned(),
                family_id: ArtifactFamilyId::ReplayBundle,
                pin_source: PinSourceClass::RetentionWindowRef,
                referenced_object_class: ReferencedObjectClass::RetentionPolicyWindow,
                referenced_object_ref: "retention.replay.90d".to_owned(),
                retention_state: RetentionStateClass::InRetentionWindow,
                expires_at: Some("2026-09-12T00:00:00Z".to_owned()),
                pinned_by_ref: None,
                on_disk_bytes: 360_000_000,
            },
            PinInput {
                pin_id: "pin.checkpoint.alpha".to_owned(),
                label: "Local checkpoint — pre-refactor".to_owned(),
                family_id: ArtifactFamilyId::UserOwnedRecoveryState,
                pin_source: PinSourceClass::ExplicitUserPin,
                referenced_object_class: ReferencedObjectClass::LocalCheckpointOrHistory,
                referenced_object_ref: "checkpoint.pre_refactor.v1".to_owned(),
                retention_state: RetentionStateClass::RetainedUntilExplicitReset,
                expires_at: None,
                pinned_by_ref: Some("role.author".to_owned()),
                on_disk_bytes: 95_000_000,
            },
        ],
        cleanups: vec![
            CleanupInput {
                event_id: "cleanup.preview_trim.alpha".to_owned(),
                occurred_at: "2026-06-13T22:05:00Z".to_owned(),
                actor_class: CleanupActorClass::SystemPressureGovernor,
                actor_ref: None,
                trigger_class: CleanupTriggerClass::LowDiskPressure,
                family_id: ArtifactFamilyId::GeneratedPreview,
                disposition: CleanupDispositionClass::TrimmedDisposableCache,
                reclaimed_bytes: 1_200_000_000,
                blocked_pin_sources: vec![],
                resulting_state: ResultingStateClass::FullyReclaimedNoResidual,
            },
            CleanupInput {
                event_id: "cleanup.evidence_expiry.alpha".to_owned(),
                occurred_at: "2026-06-13T22:06:00Z".to_owned(),
                actor_class: CleanupActorClass::RetentionScheduler,
                actor_ref: None,
                trigger_class: CleanupTriggerClass::RetentionWindowExpiry,
                family_id: ArtifactFamilyId::ProfilerTrace,
                disposition: CleanupDispositionClass::ExpiredUnpinnedEvidencePastRetention,
                reclaimed_bytes: 240_000_000,
                blocked_pin_sources: vec![PinSourceClass::CaseReferenceRef],
                resulting_state: ResultingStateClass::PartialRetainedPins,
            },
        ],
    }
}

fn offline_packs_signal() -> ManagerSignal {
    ManagerSignal {
        manager_id: "pin_retention.offline_packs_and_templates.v1".to_owned(),
        emitted_at: "2026-06-14T00:00:00Z".to_owned(),
        scope_ref: "ws.beta".to_owned(),
        scope_label: "Project Beta".to_owned(),
        pins: vec![
            PinInput {
                pin_id: "pin.docs_pack.beta".to_owned(),
                label: "Docs pack — offline bundle".to_owned(),
                family_id: ArtifactFamilyId::DocsPack,
                pin_source: PinSourceClass::OfflineBundleRef,
                referenced_object_class: ReferencedObjectClass::OfflineEntitlementBundle,
                referenced_object_ref: "offline_bundle.docs.v7".to_owned(),
                retention_state: RetentionStateClass::PinnedIndefiniteWhileReferenced,
                expires_at: None,
                pinned_by_ref: None,
                on_disk_bytes: 540_000_000,
            },
            PinInput {
                pin_id: "pin.model_pack.beta".to_owned(),
                label: "Model pack — release reference".to_owned(),
                family_id: ArtifactFamilyId::ModelPack,
                pin_source: PinSourceClass::ReleaseArtifactGraphRef,
                referenced_object_class: ReferencedObjectClass::ReleaseArtifactGraph,
                referenced_object_ref: "release.model.graph.v12".to_owned(),
                retention_state: RetentionStateClass::PinnedIndefiniteWhileReferenced,
                expires_at: None,
                pinned_by_ref: None,
                on_disk_bytes: 6_400_000_000,
            },
            PinInput {
                pin_id: "pin.template_pack.beta".to_owned(),
                label: "Certified template pack".to_owned(),
                family_id: ArtifactFamilyId::TemplatePack,
                pin_source: PinSourceClass::CertifiedArchetypeOrTemplateRef,
                referenced_object_class: ReferencedObjectClass::CertifiedTemplateOrArchetype,
                referenced_object_ref: "certified.template.service_scaffold.v2".to_owned(),
                retention_state: RetentionStateClass::PinnedIndefiniteWhileReferenced,
                expires_at: None,
                pinned_by_ref: None,
                on_disk_bytes: 210_000_000,
            },
            PinInput {
                pin_id: "pin.notebook_output.beta".to_owned(),
                label: "Notebook outputs — pinned".to_owned(),
                family_id: ArtifactFamilyId::NotebookOutput,
                pin_source: PinSourceClass::ExplicitUserPin,
                referenced_object_class: ReferencedObjectClass::WorkspaceArtifact,
                referenced_object_ref: "notebook.analysis.cell_outputs".to_owned(),
                retention_state: RetentionStateClass::PinnedByExplicitUserChoice,
                expires_at: None,
                pinned_by_ref: Some("role.author".to_owned()),
                on_disk_bytes: 1_100_000_000,
            },
        ],
        cleanups: vec![
            CleanupInput {
                event_id: "cleanup.artifact_trim.beta".to_owned(),
                occurred_at: "2026-06-13T20:00:00Z".to_owned(),
                actor_class: CleanupActorClass::SystemPressureGovernor,
                actor_ref: None,
                trigger_class: CleanupTriggerClass::LowDiskPressure,
                family_id: ArtifactFamilyId::ExtensionDownload,
                disposition: CleanupDispositionClass::TrimmedUnpinnedArtifact,
                reclaimed_bytes: 820_000_000,
                blocked_pin_sources: vec![
                    PinSourceClass::OfflineBundleRef,
                    PinSourceClass::ReleaseArtifactGraphRef,
                ],
                resulting_state: ResultingStateClass::PartialRetainedPins,
            },
            CleanupInput {
                event_id: "cleanup.index_trim.beta".to_owned(),
                occurred_at: "2026-06-13T20:01:00Z".to_owned(),
                actor_class: CleanupActorClass::SystemPressureGovernor,
                actor_ref: None,
                trigger_class: CleanupTriggerClass::LowDiskPressure,
                family_id: ArtifactFamilyId::GeneratedPreview,
                disposition: CleanupDispositionClass::TrimmedRebuildableCache,
                reclaimed_bytes: 300_000_000,
                blocked_pin_sources: vec![],
                resulting_state: ResultingStateClass::ReindexNeeded,
            },
        ],
    }
}

fn cleanup_blocked_signal() -> ManagerSignal {
    ManagerSignal {
        manager_id: "pin_retention.cleanup_blocked_by_pins.v1".to_owned(),
        emitted_at: "2026-06-14T00:00:00Z".to_owned(),
        scope_ref: "ws.gamma".to_owned(),
        scope_label: "Project Gamma".to_owned(),
        pins: vec![
            PinInput {
                pin_id: "pin.policy_evidence.gamma".to_owned(),
                label: "Review evidence — admin policy hold".to_owned(),
                family_id: ArtifactFamilyId::ReviewIncidentEvidence,
                pin_source: PinSourceClass::ExplicitAdminPolicyPin,
                referenced_object_class: ReferencedObjectClass::AdminPolicyBinding,
                referenced_object_ref: "policy.legal_hold.case_4471".to_owned(),
                retention_state: RetentionStateClass::PolicyWindowManaged,
                expires_at: None,
                pinned_by_ref: Some("role.admin".to_owned()),
                on_disk_bytes: 510_000_000,
            },
            PinInput {
                pin_id: "pin.checkpoint.gamma".to_owned(),
                label: "Session restore checkpoint".to_owned(),
                family_id: ArtifactFamilyId::UserOwnedRecoveryState,
                pin_source: PinSourceClass::RetentionWindowRef,
                referenced_object_class: ReferencedObjectClass::RetentionPolicyWindow,
                referenced_object_ref: "retention.checkpoint.30d".to_owned(),
                retention_state: RetentionStateClass::InRetentionWindow,
                expires_at: Some("2026-07-13T00:00:00Z".to_owned()),
                pinned_by_ref: None,
                on_disk_bytes: 72_000_000,
            },
        ],
        cleanups: vec![
            CleanupInput {
                event_id: "cleanup.evidence_blocked.gamma".to_owned(),
                occurred_at: "2026-06-13T18:30:00Z".to_owned(),
                actor_class: CleanupActorClass::SystemPressureGovernor,
                actor_ref: None,
                trigger_class: CleanupTriggerClass::LowDiskPressure,
                family_id: ArtifactFamilyId::ReviewIncidentEvidence,
                disposition: CleanupDispositionClass::BlockedNoOpPinProtected,
                reclaimed_bytes: 0,
                blocked_pin_sources: vec![PinSourceClass::ExplicitAdminPolicyPin],
                resulting_state: ResultingStateClass::AuthoritativeStateUntouched,
            },
            CleanupInput {
                event_id: "cleanup.recovery_blocked.gamma".to_owned(),
                occurred_at: "2026-06-13T18:31:00Z".to_owned(),
                actor_class: CleanupActorClass::SystemPressureGovernor,
                actor_ref: None,
                trigger_class: CleanupTriggerClass::LowDiskPressure,
                family_id: ArtifactFamilyId::UserOwnedRecoveryState,
                disposition: CleanupDispositionClass::BlockedNoOpPinProtected,
                reclaimed_bytes: 0,
                blocked_pin_sources: vec![PinSourceClass::RetentionWindowRef],
                resulting_state: ResultingStateClass::AuthoritativeStateUntouched,
            },
            CleanupInput {
                event_id: "cleanup.checkpoint_export_delete.gamma".to_owned(),
                occurred_at: "2026-06-13T19:00:00Z".to_owned(),
                actor_class: CleanupActorClass::User,
                actor_ref: Some("role.author".to_owned()),
                trigger_class: CleanupTriggerClass::ExplicitUserClearData,
                family_id: ArtifactFamilyId::UserOwnedRecoveryState,
                disposition: CleanupDispositionClass::ExportedThenDeleted,
                reclaimed_bytes: 48_000_000,
                blocked_pin_sources: vec![],
                resulting_state: ResultingStateClass::FullyReclaimedNoResidual,
            },
        ],
    }
}

fn managed_quota_signal() -> ManagerSignal {
    ManagerSignal {
        manager_id: "pin_retention.managed_quota_preserves_user_state.v1".to_owned(),
        emitted_at: "2026-06-14T00:00:00Z".to_owned(),
        scope_ref: "ws.delta".to_owned(),
        scope_label: "Project Delta".to_owned(),
        pins: vec![
            PinInput {
                pin_id: "pin.prebuild.delta".to_owned(),
                label: "Prebuild layer — offline bundle".to_owned(),
                family_id: ArtifactFamilyId::PrebuildLayer,
                pin_source: PinSourceClass::OfflineBundleRef,
                referenced_object_class: ReferencedObjectClass::OfflineEntitlementBundle,
                referenced_object_ref: "offline_bundle.toolchain.v4".to_owned(),
                retention_state: RetentionStateClass::PinnedIndefiniteWhileReferenced,
                expires_at: None,
                pinned_by_ref: None,
                on_disk_bytes: 3_200_000_000,
            },
            PinInput {
                pin_id: "pin.checkpoint.delta".to_owned(),
                label: "User checkpoint — feature branch".to_owned(),
                family_id: ArtifactFamilyId::UserOwnedRecoveryState,
                pin_source: PinSourceClass::ExplicitUserPin,
                referenced_object_class: ReferencedObjectClass::LocalCheckpointOrHistory,
                referenced_object_ref: "checkpoint.feature_branch.v2".to_owned(),
                retention_state: RetentionStateClass::RetainedUntilExplicitReset,
                expires_at: None,
                pinned_by_ref: Some("role.author".to_owned()),
                on_disk_bytes: 140_000_000,
            },
        ],
        cleanups: vec![
            CleanupInput {
                event_id: "cleanup.quota_artifact_trim.delta".to_owned(),
                occurred_at: "2026-06-13T16:00:00Z".to_owned(),
                actor_class: CleanupActorClass::AdminPolicy,
                actor_ref: Some("role.tenant_admin".to_owned()),
                trigger_class: CleanupTriggerClass::ManagedQuotaPressure,
                family_id: ArtifactFamilyId::ModelPack,
                disposition: CleanupDispositionClass::TrimmedUnpinnedArtifact,
                reclaimed_bytes: 5_900_000_000,
                blocked_pin_sources: vec![PinSourceClass::OfflineBundleRef],
                resulting_state: ResultingStateClass::PartialRetainedPins,
            },
            CleanupInput {
                event_id: "cleanup.quota_recovery_refused.delta".to_owned(),
                occurred_at: "2026-06-13T16:01:00Z".to_owned(),
                actor_class: CleanupActorClass::SystemPressureGovernor,
                actor_ref: None,
                trigger_class: CleanupTriggerClass::ManagedQuotaPressure,
                family_id: ArtifactFamilyId::UserOwnedRecoveryState,
                disposition: CleanupDispositionClass::BlockedNoOpPinProtected,
                reclaimed_bytes: 0,
                blocked_pin_sources: vec![PinSourceClass::ExplicitUserPin],
                resulting_state: ResultingStateClass::AuthoritativeStateUntouched,
            },
        ],
    }
}
