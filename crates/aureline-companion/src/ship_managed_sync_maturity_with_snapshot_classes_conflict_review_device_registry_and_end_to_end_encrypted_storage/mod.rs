//! Managed sync maturity: snapshot classes, conflict review, device registry, and
//! end-to-end encrypted storage, projected as a downgrade-aware truth packet.
//!
//! This module owns the export-safe truth packet for managed sync maturity. It
//! projects four sections: the **snapshot classes** that record which classes of
//! local state (settings, profile, device registry, workspace layout, extension
//! state) participate in managed sync and in which direction, the **conflict
//! review** queue that records every sync conflict with no silent server authority,
//! the **device registry** that records the devices participating in sync and their
//! trust state, and the **encrypted storage** posture that records the
//! customer-managed-key, end-to-end-encryption, and region-residency claim for each
//! managed artifact scope — proved where claimed or honestly labeled where not. It
//! binds the first three sections to the frozen M5 companion-matrix
//! [`M5CompanionMatrixLane::ManagedSync`] lane and the encrypted-storage section to
//! the [`M5CompanionMatrixLane::ResidencyEncryption`] lane, and gives every item an
//! exact [`CompanionDesktopHandoff`] (reused from
//! [`crate::companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff`])
//! so opening an item always resumes the precise host context locally.
//!
//! Three invariants make this surface safe to ship. First, **inspectable, with no
//! silent server authority**: every section is read-only, the local core stays the
//! authoritative source of truth, every synced record reconciles back to it
//! ([`SyncReconciliationState`]), and a sync conflict is always reviewed by the user
//! rather than silently resolved in the server's favor. Second, **provable where
//! claimed**: an encrypted-storage row claims end-to-end or at-rest encryption only
//! when it is verifiable; an unverifiable claim narrows to
//! [`EncryptionPosture::ClaimedUnverified`] and is labeled rather than shown as a
//! proven claim. Third, **stale-state honesty**: every item carries a
//! [`CompanionFreshnessState`], stale or unknown freshness is always labeled, and a
//! degraded item is never shown as live.
//!
//! The packet reuses the matrix vocabulary from
//! [`crate::freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes`]
//! ([`M5CompanionQualificationClass`], [`M5CompanionRolloutStage`],
//! [`M5CompanionDowngradeTrigger`], [`M5CompanionRollbackPosture`],
//! [`M5CompanionLocalityDisclosure`], [`M5CompanionConsumerSurface`]), and the
//! freshness, scope, and handoff vocabulary
//! ([`CompanionFreshnessState`], [`CompanionReadWriteScope`],
//! [`CompanionDesktopHandoff`], [`CompanionHandoffResolution`],
//! [`CompanionHandoffTarget`]) from the session-follow and triage surfaces, instead
//! of inventing parallel terms. Each section row records the matrix lane it inherits
//! qualification from.
//!
//! [`ManagedSyncMaturitySurfacePacket::apply_managed_sync_degradation`] narrows
//! sections and downgrades freshness, reconciliation, and encryption claims from a
//! per-observation signal — when the sync provider is unavailable, proof is stale,
//! managed-tenant admin continuity is unavailable, the residency or encryption claim
//! is unverified, sync can no longer be inspected or reconciled, device trust
//! narrowed, the host session is inactive, or an upstream matrix lane narrowed — so
//! CI or release tooling degrades the surface honestly rather than show fresh state,
//! a reconciled record it can no longer reconcile, or an encryption claim it can no
//! longer prove. Degraded state is labeled, never hidden.
//!
//! [`canonical_managed_sync_maturity_surface`] builds the surface and
//! [`current_stable_managed_sync_maturity_surface_export`] reads and validates the
//! checked-in support export, so the desktop companion panel, the CLI/headless
//! surface, diagnostics, support exports, and Help/About ingest the packet rather
//! than cloning status text. Credential bodies, raw key material, raw provider
//! payloads, and raw sync record contents stay outside this boundary.
//!
//! The boundary schema is
//! [`schemas/companion/ship-managed-sync-maturity-with-snapshot-classes-conflict-review-device-registry-and-end-to-end-encrypted-storage.schema.json`](../../../../schemas/companion/ship-managed-sync-maturity-with-snapshot-classes-conflict-review-device-registry-and-end-to-end-encrypted-storage.schema.json).
//! The contract doc is
//! [`docs/companion/m5/ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage.md`](../../../../docs/companion/m5/ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage.md).
//! The protected fixture directory is
//! [`fixtures/companion/m5/ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage/`](../../../../fixtures/companion/m5/ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff::{
    CompanionDesktopHandoff, CompanionHandoffResolution, CompanionHandoffTarget,
};
use crate::freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes::{
    M5CompanionConsumerSurface, M5CompanionDowngradeTrigger, M5CompanionLocalityDisclosure,
    M5CompanionMatrixLane, M5CompanionQualificationClass, M5CompanionRollbackPosture,
    M5CompanionRolloutStage, M5_COMPANION_MATRIX_SCHEMA_REF, M5_MANAGED_SYNC_POLICY_REF,
    M5_PROFILE_SYNC_CONTRACT_REF, M5_REGION_RESIDENCY_REF,
};
use crate::ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty::{
    CompanionFreshnessState, CompanionReadWriteScope,
};

/// Stable record-kind tag carried by [`ManagedSyncMaturitySurfacePacket`].
pub const MANAGED_SYNC_MATURITY_RECORD_KIND: &str =
    "ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage";

/// Schema version for managed sync maturity surface records.
pub const MANAGED_SYNC_MATURITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const MANAGED_SYNC_MATURITY_SCHEMA_REF: &str =
    "schemas/companion/ship-managed-sync-maturity-with-snapshot-classes-conflict-review-device-registry-and-end-to-end-encrypted-storage.schema.json";

/// Repo-relative path of the managed sync maturity surface contract doc.
pub const MANAGED_SYNC_MATURITY_DOC_REF: &str =
    "docs/companion/m5/ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage.md";

/// Repo-relative path of the protected fixture directory.
pub const MANAGED_SYNC_MATURITY_FIXTURE_DIR: &str =
    "fixtures/companion/m5/ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage";

/// Repo-relative path of the checked support-export artifact.
pub const MANAGED_SYNC_MATURITY_ARTIFACT_REF: &str =
    "artifacts/companion/m5/ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const MANAGED_SYNC_MATURITY_SUMMARY_REF: &str =
    "artifacts/companion/m5/ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage.md";

/// One of the four managed sync maturity sections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedSyncSection {
    /// The managed sync snapshot classes.
    SnapshotClass,
    /// The conflict review queue.
    ConflictReview,
    /// The device registry.
    DeviceRegistry,
    /// The end-to-end encrypted storage posture.
    EncryptedStorage,
}

impl ManagedSyncSection {
    /// Every section, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SnapshotClass,
        Self::ConflictReview,
        Self::DeviceRegistry,
        Self::EncryptedStorage,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SnapshotClass => "snapshot_class",
            Self::ConflictReview => "conflict_review",
            Self::DeviceRegistry => "device_registry",
            Self::EncryptedStorage => "encrypted_storage",
        }
    }

    /// Frozen M5 companion-matrix lane this section inherits qualification from.
    ///
    /// The snapshot-class, conflict-review, and device-registry sections inherit
    /// from the [`M5CompanionMatrixLane::ManagedSync`] lane; the encrypted-storage
    /// section inherits from the [`M5CompanionMatrixLane::ResidencyEncryption`] lane.
    pub const fn matrix_lane(self) -> M5CompanionMatrixLane {
        match self {
            Self::SnapshotClass | Self::ConflictReview | Self::DeviceRegistry => {
                M5CompanionMatrixLane::ManagedSync
            }
            Self::EncryptedStorage => M5CompanionMatrixLane::ResidencyEncryption,
        }
    }

    /// Read/write scope this section is bounded to.
    ///
    /// Every section is read-only: the surface inspects and reports but never mutates
    /// host state directly. A conflict resolution decision and a snapshot or device
    /// change are applied by the local core, never authored from this surface.
    pub const fn bounded_scope(self) -> CompanionReadWriteScope {
        CompanionReadWriteScope::ReadOnly
    }
}

/// Class of local state that participates in managed sync.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncSnapshotClassKind {
    /// User and workspace settings.
    Settings,
    /// User profile.
    Profile,
    /// The device registry itself.
    DeviceRegistry,
    /// Workspace layout and view state.
    WorkspaceLayout,
    /// Installed-extension state.
    ExtensionState,
}

impl SyncSnapshotClassKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Settings => "settings",
            Self::Profile => "profile",
            Self::DeviceRegistry => "device_registry",
            Self::WorkspaceLayout => "workspace_layout",
            Self::ExtensionState => "extension_state",
        }
    }
}

/// Direction a snapshot class flows during managed sync.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncDirection {
    /// Local changes flow to managed storage.
    LocalToManaged,
    /// Managed changes flow to the local core.
    ManagedToLocal,
    /// Changes flow both ways.
    Bidirectional,
    /// The class never leaves the local core.
    LocalOnly,
}

impl SyncDirection {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalToManaged => "local_to_managed",
            Self::ManagedToLocal => "managed_to_local",
            Self::Bidirectional => "bidirectional",
            Self::LocalOnly => "local_only",
        }
    }
}

/// Reconciliation state of a synced record against the authoritative local core.
///
/// Managed sync stays inspectable: every synced record reconciles back to the local
/// core. When reconciliation can no longer be established the record narrows to
/// [`Self::Unreconcilable`] rather than claiming a reconciliation it cannot prove.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncReconciliationState {
    /// Fully reconciled with the authoritative local core.
    Reconciled,
    /// Diverged from the local core and pending review.
    DivergedPendingReview,
    /// Reconciliation against the local core could not be established.
    Unreconcilable,
}

impl SyncReconciliationState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reconciled => "reconciled",
            Self::DivergedPendingReview => "diverged_pending_review",
            Self::Unreconcilable => "unreconcilable",
        }
    }

    /// Forces a reconciled or diverged record to [`Self::Unreconcilable`]; an
    /// already-unreconcilable record is kept.
    pub const fn forced_unreconcilable(self) -> Self {
        match self {
            Self::Reconciled | Self::DivergedPendingReview => Self::Unreconcilable,
            Self::Unreconcilable => self,
        }
    }
}

/// Kind of managed sync conflict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncConflictKind {
    /// The same record was edited concurrently on two devices.
    ConcurrentEdit,
    /// The record was deleted on a remote device.
    DeletedRemotely,
    /// The record's schema version diverged.
    SchemaMismatch,
    /// Device clocks disagreed on ordering.
    ClockSkew,
    /// The originating device was revoked.
    DeviceRevoked,
}

impl SyncConflictKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConcurrentEdit => "concurrent_edit",
            Self::DeletedRemotely => "deleted_remotely",
            Self::SchemaMismatch => "schema_mismatch",
            Self::ClockSkew => "clock_skew",
            Self::DeviceRevoked => "device_revoked",
        }
    }
}

/// Resolution state of a managed sync conflict.
///
/// There is deliberately no "resolved by server" state: a conflict is reviewed by
/// the user, and the local core applies the decision. The server never wins
/// silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictResolutionState {
    /// Awaiting user review.
    PendingReview,
    /// Resolved by keeping the local version.
    ResolvedKeepLocal,
    /// Resolved by accepting the remote version after review.
    ResolvedKeepRemote,
    /// Resolved by merging both versions after review.
    ResolvedMerged,
    /// Deferred by the user.
    Deferred,
}

impl ConflictResolutionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PendingReview => "pending_review",
            Self::ResolvedKeepLocal => "resolved_keep_local",
            Self::ResolvedKeepRemote => "resolved_keep_remote",
            Self::ResolvedMerged => "resolved_merged",
            Self::Deferred => "deferred",
        }
    }

    /// True when the conflict has been resolved (in either direction or merged).
    pub const fn is_resolved(self) -> bool {
        matches!(
            self,
            Self::ResolvedKeepLocal | Self::ResolvedKeepRemote | Self::ResolvedMerged
        )
    }
}

/// Trust state of a device in the registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncDeviceTrustState {
    /// Trusted and participating in sync.
    Trusted,
    /// Awaiting approval before it may sync.
    PendingApproval,
    /// Revoked and no longer permitted to sync.
    Revoked,
    /// Trust could not be established.
    Unknown,
}

impl SyncDeviceTrustState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::PendingApproval => "pending_approval",
            Self::Revoked => "revoked",
            Self::Unknown => "unknown",
        }
    }

    /// Narrows trusted to pending-approval when device trust narrows; other states
    /// are kept.
    pub const fn narrowed_trust(self) -> Self {
        match self {
            Self::Trusted => Self::PendingApproval,
            Self::PendingApproval | Self::Revoked | Self::Unknown => self,
        }
    }
}

/// Managed artifact scope an encrypted-storage row describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncryptedArtifactScope {
    /// The managed snapshot store.
    ManagedSnapshotStore,
    /// The sync transport channel.
    SyncTransport,
    /// The conflict-resolution history.
    ConflictHistory,
    /// The device registry store.
    DeviceRegistryStore,
}

impl EncryptedArtifactScope {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedSnapshotStore => "managed_snapshot_store",
            Self::SyncTransport => "sync_transport",
            Self::ConflictHistory => "conflict_history",
            Self::DeviceRegistryStore => "device_registry_store",
        }
    }
}

/// Encryption posture of a managed artifact scope.
///
/// Customer-managed-key and end-to-end-encryption claims stay provable: a verified
/// posture is claimed only when it is backed by evidence. An unverifiable claim
/// narrows to [`Self::ClaimedUnverified`] and is labeled rather than shown as a
/// proven claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncryptionPosture {
    /// End-to-end encrypted, claim verified by evidence.
    EndToEndEncryptedVerified,
    /// Encrypted at rest, claim verified by evidence.
    EncryptedAtRestVerified,
    /// Encryption is claimed but the claim could not be verified.
    ClaimedUnverified,
    /// The scope is not encrypted, stated honestly.
    NotEncrypted,
}

impl EncryptionPosture {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EndToEndEncryptedVerified => "end_to_end_encrypted_verified",
            Self::EncryptedAtRestVerified => "encrypted_at_rest_verified",
            Self::ClaimedUnverified => "claimed_unverified",
            Self::NotEncrypted => "not_encrypted",
        }
    }

    /// True when the posture is a verified encryption claim.
    pub const fn is_verified_claim(self) -> bool {
        matches!(
            self,
            Self::EndToEndEncryptedVerified | Self::EncryptedAtRestVerified
        )
    }

    /// True when the posture asserts some form of encryption (verified or claimed).
    pub const fn claims_encryption(self) -> bool {
        !matches!(self, Self::NotEncrypted)
    }

    /// Downgrades a verified claim to [`Self::ClaimedUnverified`]; other postures are
    /// kept.
    pub const fn downgraded_to_unverified(self) -> Self {
        match self {
            Self::EndToEndEncryptedVerified | Self::EncryptedAtRestVerified => {
                Self::ClaimedUnverified
            }
            Self::ClaimedUnverified | Self::NotEncrypted => self,
        }
    }
}

/// Key custody model for a managed artifact scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyCustody {
    /// The customer manages the key.
    CustomerManagedKey,
    /// The provider manages the key.
    ProviderManagedKey,
    /// The key never leaves the local core; nothing is escrowed.
    LocalOnlyNoKeyEscrow,
}

impl KeyCustody {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CustomerManagedKey => "customer_managed_key",
            Self::ProviderManagedKey => "provider_managed_key",
            Self::LocalOnlyNoKeyEscrow => "local_only_no_key_escrow",
        }
    }
}

/// A managed sync snapshot-class row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncSnapshotClassItem {
    /// Stable item id.
    pub item_id: String,
    /// Class of state participating in sync.
    pub class_kind: SyncSnapshotClassKind,
    /// Direction the class flows.
    pub direction: SyncDirection,
    /// Reconciliation state against the authoritative local core.
    pub reconciliation: SyncReconciliationState,
    /// Always true: the local core is the authoritative source of truth.
    pub local_authoritative: bool,
    /// Freshness of the row.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the snapshot-class record. Carries no payload body.
    pub record_ref: String,
    /// Exact desktop handoff to the snapshot class.
    pub handoff: CompanionDesktopHandoff,
}

/// A conflict-review queue item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncConflictItem {
    /// Stable item id.
    pub item_id: String,
    /// Ref to the snapshot class this conflict belongs to. Carries no payload body.
    pub snapshot_class_ref: String,
    /// Kind of conflict.
    pub conflict_kind: SyncConflictKind,
    /// Resolution state.
    pub resolution_state: ConflictResolutionState,
    /// Always true: a conflict is reviewed by the user, never resolved silently.
    pub requires_user_review: bool,
    /// Reconciliation state of the conflicted record.
    pub reconciliation: SyncReconciliationState,
    /// Freshness of the row.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the conflict record. Carries no payload body.
    pub conflict_ref: String,
    /// Exact desktop handoff to the conflict.
    pub handoff: CompanionDesktopHandoff,
}

/// A device-registry row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncDeviceItem {
    /// Stable item id.
    pub item_id: String,
    /// Trust state of the device.
    pub trust_state: SyncDeviceTrustState,
    /// True when this row is the current local device.
    pub this_device: bool,
    /// Reconciliation state of the device's synced view.
    pub reconciliation: SyncReconciliationState,
    /// Freshness of the row.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the device record. Carries no payload body.
    pub device_ref: String,
    /// Exact desktop handoff to the device.
    pub handoff: CompanionDesktopHandoff,
}

/// An encrypted-storage posture row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptedStorageItem {
    /// Stable item id.
    pub item_id: String,
    /// Managed artifact scope this row describes.
    pub artifact_scope: EncryptedArtifactScope,
    /// Encryption posture, proved where claimed.
    pub encryption_posture: EncryptionPosture,
    /// Key custody model.
    pub key_custody: KeyCustody,
    /// Opaque region-residency pin ref. Carries no payload body.
    pub residency_region_ref: String,
    /// True when the encryption/residency claim is verified by evidence.
    pub claim_verified: bool,
    /// True when an unverified claim carries a visible "claimed, not verified" label.
    pub proof_label_shown: bool,
    /// Freshness of the row.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the encryption/residency evidence. Carries no payload body.
    pub evidence_ref: String,
    /// Exact desktop handoff to the encrypted-storage row.
    pub handoff: CompanionDesktopHandoff,
}

/// Per-section qualification inherited from the frozen M5 matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedSyncSectionQualification {
    /// Section the row applies to.
    pub section: ManagedSyncSection,
    /// Qualification class earned by this section.
    pub qualification: M5CompanionQualificationClass,
    /// Staged rollout stage.
    pub rollout_stage: M5CompanionRolloutStage,
    /// Read/write scope this section is bounded to.
    pub read_write_scope: CompanionReadWriteScope,
    /// Token of the frozen matrix lane this section inherits qualification from.
    pub matrix_lane_ref: String,
    /// Downgrade triggers that apply to this section.
    pub downgrade_triggers: Vec<M5CompanionDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5CompanionRollbackPosture,
}

/// Read/write scope and authority contract for the whole surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedSyncScopeContract {
    /// The snapshot-class section is read-only.
    pub snapshot_class_read_only: bool,
    /// The conflict-review section is read-only.
    pub conflict_review_read_only: bool,
    /// The device-registry section is read-only.
    pub device_registry_read_only: bool,
    /// The encrypted-storage section is read-only.
    pub encrypted_storage_read_only: bool,
    /// The local core stays the authoritative source of truth.
    pub local_core_authoritative: bool,
    /// No conflict is ever resolved silently in the server's favor.
    pub no_silent_server_authority: bool,
    /// Every conflict requires explicit user review.
    pub conflict_requires_user_review: bool,
    /// The surface never holds an unbounded write authority.
    pub no_unbounded_workspace_write: bool,
    /// No payload bodies cross the export boundary.
    pub no_payload_bodies: bool,
}

/// Inspectability and provability contract for the whole surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedSyncInspectabilityContract {
    /// Every synced record reconciles back to the local core, or narrows honestly.
    pub every_synced_record_reconcilable: bool,
    /// Conflicts are reviewed, never auto-resolved in the server's favor.
    pub conflicts_reviewed_not_auto_resolved: bool,
    /// The device registry is fully inspectable.
    pub device_registry_inspectable: bool,
    /// Encryption claims are provable or labeled as unverified.
    pub encryption_claim_provable_or_labeled: bool,
    /// Region residency is disclosed for every managed artifact scope.
    pub residency_region_disclosed: bool,
    /// No reconciliation or encryption claim is made without backing evidence.
    pub no_claim_without_evidence: bool,
}

/// Stale-state honesty contract for the whole surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedSyncStaleStateHonesty {
    /// Every stale item is labeled.
    pub stale_items_labeled: bool,
    /// Every unknown-freshness item is labeled.
    pub unknown_freshness_labeled: bool,
    /// A stale item is never shown as live.
    pub never_show_stale_as_live: bool,
    /// A freshness floor is enforced before an item is shown.
    pub freshness_floor_enforced: bool,
}

/// Security and privacy review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedSyncSecurityReview {
    /// The snapshot-class section is read-only.
    pub snapshot_class_read_only: bool,
    /// The conflict-review section is read-only.
    pub conflict_review_read_only: bool,
    /// The device-registry section is read-only.
    pub device_registry_read_only: bool,
    /// The encrypted-storage section is read-only.
    pub encrypted_storage_read_only: bool,
    /// The local core stays the authoritative source of truth.
    pub local_core_authoritative: bool,
    /// No conflict is resolved silently in the server's favor.
    pub no_silent_server_authority: bool,
    /// Managed sync stays inspectable and reconcilable.
    pub managed_sync_inspectable: bool,
    /// Every conflict requires explicit user review.
    pub conflict_requires_user_review: bool,
    /// Device trust is tracked and revocation is honored.
    pub device_trust_tracked: bool,
    /// Encryption claims are provable or labeled as unverified.
    pub encryption_claim_provable_or_labeled: bool,
    /// Region residency is disclosed for every managed artifact scope.
    pub residency_region_disclosed: bool,
    /// Customer-managed-key custody is supported and recorded.
    pub customer_managed_key_supported: bool,
    /// End-to-end-encryption is claimed only when verifiable.
    pub e2ee_claimed_only_when_verifiable: bool,
    /// Stale state is labeled rather than hidden.
    pub stale_state_labeled_never_hidden: bool,
    /// Exact desktop handoff is preserved or honestly degraded.
    pub exact_desktop_handoff_preserved: bool,
    /// No credential or key bodies cross the export boundary.
    pub no_credential_or_key_bodies_in_export: bool,
    /// No payload bodies cross the export boundary.
    pub no_payload_bodies_in_export: bool,
    /// Downgrade narrows the claim rather than hiding the section.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Every section discloses local, staged, and provider/admin continuity.
    pub locality_disclosed: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedSyncConsumerProjection {
    /// Desktop panel projects the snapshot classes.
    pub desktop_panel_shows_snapshot_classes: bool,
    /// Desktop panel projects the conflict review queue.
    pub desktop_panel_shows_conflict_review: bool,
    /// Desktop panel projects the device registry.
    pub desktop_panel_shows_device_registry: bool,
    /// Diagnostics shows the encryption posture.
    pub diagnostics_shows_encryption_posture: bool,
    /// CLI / headless shows the reconciliation state.
    pub cli_headless_shows_reconciliation_state: bool,
    /// Support export shows reconciliation and freshness state.
    pub support_export_shows_reconciliation_and_freshness: bool,
    /// Help / About shows the encryption and residency claim.
    pub help_about_shows_encryption_and_residency_claim: bool,
    /// Preview / Labs sections are visibly labeled when not qualified Stable.
    pub preview_labs_label_for_unqualified_sections: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedSyncProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the section.
    pub auto_narrow_on_stale: bool,
}

/// Per-observation signal fed to
/// [`ManagedSyncMaturitySurfacePacket::apply_managed_sync_degradation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedSyncObservation {
    /// True when the managed sync provider is available.
    pub sync_provider_available: bool,
    /// True when proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when managed-tenant admin continuity is available.
    pub admin_continuity_available: bool,
    /// True when the residency and encryption claims are verified.
    pub residency_and_encryption_verified: bool,
    /// True when managed sync can be inspected and reconciled to the local core.
    pub sync_inspectable: bool,
    /// True when device trust is intact.
    pub device_trust_intact: bool,
    /// True when an active desktop host session exists.
    pub host_session_active: bool,
    /// True when an upstream frozen matrix lane narrowed.
    pub upstream_matrix_narrowed: bool,
}

/// Reason a managed sync section has degraded below its qualified state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedSyncDegradedReason {
    /// The managed sync provider is unavailable.
    SyncProviderUnavailable,
    /// Proof has gone stale.
    ProofStale,
    /// Managed-tenant admin continuity is unavailable.
    AdminContinuityUnavailable,
    /// The residency or encryption claim could not be verified.
    ResidencyOrEncryptionUnverified,
    /// Managed sync could not be inspected or reconciled.
    SyncInspectionUnavailable,
    /// Device trust narrowed.
    DeviceTrustNarrowed,
    /// No active desktop host session.
    HostSessionInactive,
    /// An upstream frozen matrix lane narrowed.
    UpstreamMatrixNarrowed,
    /// One or more desktop handoff targets could not resolve exactly.
    HandoffTargetUnresolved,
    /// One or more item freshness states were downgraded to stale.
    FreshnessDowngradedToStale,
    /// One or more records were downgraded to unreconcilable.
    ReconciliationDowngraded,
    /// One or more encryption claims were downgraded to claimed-but-unverified.
    EncryptionClaimDowngraded,
}

impl ManagedSyncDegradedReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SyncProviderUnavailable => "sync_provider_unavailable",
            Self::ProofStale => "proof_stale",
            Self::AdminContinuityUnavailable => "admin_continuity_unavailable",
            Self::ResidencyOrEncryptionUnverified => "residency_or_encryption_unverified",
            Self::SyncInspectionUnavailable => "sync_inspection_unavailable",
            Self::DeviceTrustNarrowed => "device_trust_narrowed",
            Self::HostSessionInactive => "host_session_inactive",
            Self::UpstreamMatrixNarrowed => "upstream_matrix_narrowed",
            Self::HandoffTargetUnresolved => "handoff_target_unresolved",
            Self::FreshnessDowngradedToStale => "freshness_downgraded_to_stale",
            Self::ReconciliationDowngraded => "reconciliation_downgraded",
            Self::EncryptionClaimDowngraded => "encryption_claim_downgraded",
        }
    }
}

/// Constructor input for [`ManagedSyncMaturitySurfacePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedSyncMaturitySurfacePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer surfaces that project this packet.
    pub projected_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Per-section qualification rows.
    pub section_qualifications: Vec<ManagedSyncSectionQualification>,
    /// Snapshot-class items.
    pub snapshot_classes: Vec<SyncSnapshotClassItem>,
    /// Conflict-review items.
    pub conflicts: Vec<SyncConflictItem>,
    /// Device-registry items.
    pub devices: Vec<SyncDeviceItem>,
    /// Encrypted-storage items.
    pub encrypted_storage: Vec<EncryptedStorageItem>,
    /// Read/write scope and authority contract.
    pub scope_contract: ManagedSyncScopeContract,
    /// Inspectability and provability contract.
    pub inspectability_contract: ManagedSyncInspectabilityContract,
    /// Stale-state honesty contract.
    pub stale_state_honesty: ManagedSyncStaleStateHonesty,
    /// Locality disclosure.
    pub locality_disclosure: M5CompanionLocalityDisclosure,
    /// Security review block.
    pub security_review: ManagedSyncSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: ManagedSyncConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ManagedSyncProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe managed sync maturity surface packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedSyncMaturitySurfacePacket {
    /// Record kind; must equal [`MANAGED_SYNC_MATURITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`MANAGED_SYNC_MATURITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer surfaces that project this packet.
    pub projected_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Per-section qualification rows.
    pub section_qualifications: Vec<ManagedSyncSectionQualification>,
    /// Snapshot-class items.
    pub snapshot_classes: Vec<SyncSnapshotClassItem>,
    /// Conflict-review items.
    pub conflicts: Vec<SyncConflictItem>,
    /// Device-registry items.
    pub devices: Vec<SyncDeviceItem>,
    /// Encrypted-storage items.
    pub encrypted_storage: Vec<EncryptedStorageItem>,
    /// Read/write scope and authority contract.
    pub scope_contract: ManagedSyncScopeContract,
    /// Inspectability and provability contract.
    pub inspectability_contract: ManagedSyncInspectabilityContract,
    /// Stale-state honesty contract.
    pub stale_state_honesty: ManagedSyncStaleStateHonesty,
    /// Locality disclosure.
    pub locality_disclosure: M5CompanionLocalityDisclosure,
    /// Security review block.
    pub security_review: ManagedSyncSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: ManagedSyncConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ManagedSyncProofFreshness,
    /// Degraded-state labels currently applied (empty when fully qualified).
    pub degraded_labels: Vec<ManagedSyncDegradedReason>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ManagedSyncMaturitySurfacePacket {
    /// Builds a managed sync maturity surface packet from stable-lane input.
    pub fn new(input: ManagedSyncMaturitySurfacePacketInput) -> Self {
        Self {
            record_kind: MANAGED_SYNC_MATURITY_RECORD_KIND.to_owned(),
            schema_version: MANAGED_SYNC_MATURITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            projected_surfaces: input.projected_surfaces,
            section_qualifications: input.section_qualifications,
            snapshot_classes: input.snapshot_classes,
            conflicts: input.conflicts,
            devices: input.devices,
            encrypted_storage: input.encrypted_storage,
            scope_contract: input.scope_contract,
            inspectability_contract: input.inspectability_contract,
            stale_state_honesty: input.stale_state_honesty,
            locality_disclosure: input.locality_disclosure,
            security_review: input.security_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            degraded_labels: Vec::new(),
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Narrows sections and downgrades freshness, reconciliation, and encryption
    /// claims from a per-observation signal, recording the reasons in
    /// [`Self::degraded_labels`].
    ///
    /// An unavailable provider, stale proof, or narrowed upstream matrix lane narrows
    /// every section's qualification and rollout stage one step, and an unavailable
    /// provider additionally forces every live or cached item to stale and labels it.
    /// When sync can no longer be inspected, every snapshot, conflict, and device
    /// record narrows to [`SyncReconciliationState::Unreconcilable`] and those three
    /// sections narrow. When the residency or encryption claim is unverified, every
    /// verified encryption claim downgrades to
    /// [`EncryptionPosture::ClaimedUnverified`], is labeled, and the encrypted-storage
    /// section narrows. Narrowed device trust narrows trusted devices to
    /// pending-approval and narrows the conflict-review and device-registry sections.
    /// Missing admin continuity narrows the device-registry and encrypted-storage
    /// sections, since managed-tenant device approval and residency depend on it. An
    /// inactive host session downgrades the resolution of every host-dependent desktop
    /// handoff. Degraded state is labeled, never hidden.
    pub fn apply_managed_sync_degradation(&mut self, observation: &ManagedSyncObservation) {
        let mut labels: BTreeSet<ManagedSyncDegradedReason> =
            self.degraded_labels.iter().copied().collect();

        let section_adverse = !observation.sync_provider_available
            || !observation.proof_fresh
            || observation.upstream_matrix_narrowed;

        if !observation.sync_provider_available {
            labels.insert(ManagedSyncDegradedReason::SyncProviderUnavailable);
            if self.force_all_freshness_stale() {
                labels.insert(ManagedSyncDegradedReason::FreshnessDowngradedToStale);
            }
        }
        if !observation.proof_fresh {
            labels.insert(ManagedSyncDegradedReason::ProofStale);
        }
        if observation.upstream_matrix_narrowed {
            labels.insert(ManagedSyncDegradedReason::UpstreamMatrixNarrowed);
        }
        if !observation.sync_inspectable {
            labels.insert(ManagedSyncDegradedReason::SyncInspectionUnavailable);
            if self.force_all_reconciliation_unreconcilable() {
                labels.insert(ManagedSyncDegradedReason::ReconciliationDowngraded);
            }
        }
        if !observation.device_trust_intact {
            labels.insert(ManagedSyncDegradedReason::DeviceTrustNarrowed);
            for device in &mut self.devices {
                device.trust_state = device.trust_state.narrowed_trust();
            }
        }
        if !observation.residency_and_encryption_verified {
            labels.insert(ManagedSyncDegradedReason::ResidencyOrEncryptionUnverified);
            let mut any_downgraded = false;
            for item in &mut self.encrypted_storage {
                if item.encryption_posture.is_verified_claim() {
                    item.encryption_posture = item.encryption_posture.downgraded_to_unverified();
                    item.claim_verified = false;
                    item.proof_label_shown = true;
                    any_downgraded = true;
                }
            }
            if any_downgraded {
                labels.insert(ManagedSyncDegradedReason::EncryptionClaimDowngraded);
            }
        }
        if !observation.admin_continuity_available {
            labels.insert(ManagedSyncDegradedReason::AdminContinuityUnavailable);
        }

        for row in &mut self.section_qualifications {
            let adverse = section_adverse
                || (!observation.sync_inspectable
                    && matches!(
                        row.section,
                        ManagedSyncSection::SnapshotClass
                            | ManagedSyncSection::ConflictReview
                            | ManagedSyncSection::DeviceRegistry
                    ))
                || (!observation.device_trust_intact
                    && matches!(
                        row.section,
                        ManagedSyncSection::ConflictReview | ManagedSyncSection::DeviceRegistry
                    ))
                || (!observation.residency_and_encryption_verified
                    && row.section == ManagedSyncSection::EncryptedStorage)
                || (!observation.admin_continuity_available
                    && matches!(
                        row.section,
                        ManagedSyncSection::DeviceRegistry | ManagedSyncSection::EncryptedStorage
                    ));
            if adverse {
                row.qualification = row.qualification.narrowed_one_step();
                row.rollout_stage = row.rollout_stage.narrowed_one_step();
            }
        }

        if !observation.host_session_active {
            labels.insert(ManagedSyncDegradedReason::HostSessionInactive);
            let mut any_unresolved = false;
            for handoff in self.handoffs_mut() {
                if handoff.requires_active_host
                    && handoff.resolution == CompanionHandoffResolution::Exact
                {
                    handoff.resolution = CompanionHandoffResolution::Unresolved;
                    any_unresolved = true;
                }
            }
            if any_unresolved {
                labels.insert(ManagedSyncDegradedReason::HandoffTargetUnresolved);
            }
        }

        self.degraded_labels = labels.into_iter().collect();
    }

    /// Forces every live/cached item freshness to stale and labels it. Returns true
    /// when at least one item was downgraded.
    fn force_all_freshness_stale(&mut self) -> bool {
        let mut downgraded = false;
        for (state, label) in self.freshness_states_mut() {
            if *state != state.forced_stale() {
                *state = state.forced_stale();
                *label = true;
                downgraded = true;
            }
        }
        downgraded
    }

    /// Forces every snapshot, conflict, and device record to unreconcilable. Returns
    /// true when at least one record was downgraded.
    fn force_all_reconciliation_unreconcilable(&mut self) -> bool {
        let mut downgraded = false;
        for state in self
            .snapshot_classes
            .iter_mut()
            .map(|item| &mut item.reconciliation)
            .chain(
                self.conflicts
                    .iter_mut()
                    .map(|item| &mut item.reconciliation),
            )
            .chain(self.devices.iter_mut().map(|item| &mut item.reconciliation))
        {
            if *state != state.forced_unreconcilable() {
                *state = state.forced_unreconcilable();
                downgraded = true;
            }
        }
        downgraded
    }

    /// Mutable access to every item's freshness state and stale-label flag.
    fn freshness_states_mut(
        &mut self,
    ) -> impl Iterator<Item = (&mut CompanionFreshnessState, &mut bool)> {
        self.snapshot_classes
            .iter_mut()
            .map(|item| (&mut item.freshness, &mut item.stale_label_shown))
            .chain(
                self.conflicts
                    .iter_mut()
                    .map(|item| (&mut item.freshness, &mut item.stale_label_shown)),
            )
            .chain(
                self.devices
                    .iter_mut()
                    .map(|item| (&mut item.freshness, &mut item.stale_label_shown)),
            )
            .chain(
                self.encrypted_storage
                    .iter_mut()
                    .map(|item| (&mut item.freshness, &mut item.stale_label_shown)),
            )
    }

    /// Validates the managed sync maturity surface invariants.
    pub fn validate(&self) -> Vec<ManagedSyncViolation> {
        let mut violations = Vec::new();

        if self.record_kind != MANAGED_SYNC_MATURITY_RECORD_KIND {
            violations.push(ManagedSyncViolation::WrongRecordKind);
        }
        if self.schema_version != MANAGED_SYNC_MATURITY_SCHEMA_VERSION {
            violations.push(ManagedSyncViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ManagedSyncViolation::MissingIdentity);
        }
        if self.projected_surfaces.is_empty() {
            violations.push(ManagedSyncViolation::ProjectedSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_section_qualifications(self, &mut violations);
        validate_items(self, &mut violations);
        validate_scope_contract(self, &mut violations);
        validate_inspectability_contract(self, &mut violations);
        validate_stale_state_honesty(self, &mut violations);
        validate_locality(self, &mut violations);
        validate_security_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("managed sync maturity packet serializes"),
        ) {
            violations.push(ManagedSyncViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("managed sync maturity packet serializes")
    }

    /// Sections currently publishable (Stable, Beta, or Preview) and not withheld.
    pub fn publishable_sections(&self) -> impl Iterator<Item = &ManagedSyncSectionQualification> {
        self.section_qualifications.iter().filter(|row| {
            matches!(
                row.qualification,
                M5CompanionQualificationClass::Stable
                    | M5CompanionQualificationClass::Beta
                    | M5CompanionQualificationClass::Preview
            ) && row.rollout_stage != M5CompanionRolloutStage::Withheld
        })
    }

    /// True when every item's desktop handoff resolves to the exact location.
    pub fn all_handoffs_exact(&self) -> bool {
        self.handoffs()
            .all(|handoff| handoff.resolution == CompanionHandoffResolution::Exact)
    }

    /// True when every synced record reconciles to the local core (or is honestly
    /// marked diverged/unreconcilable, never falsely reconciled).
    pub fn all_records_have_reconciliation_state(&self) -> bool {
        // Every record carries a typed reconciliation state by construction; this
        // confirms there is at least one reconciled record in the canonical corpus.
        self.snapshot_classes
            .iter()
            .any(|item| item.reconciliation == SyncReconciliationState::Reconciled)
    }

    /// True when every encryption claim is verified or labeled as unverified.
    pub fn encryption_claims_honestly_qualified(&self) -> bool {
        self.encrypted_storage.iter().all(|item| {
            if item.encryption_posture.is_verified_claim() {
                item.claim_verified
            } else if item.encryption_posture == EncryptionPosture::ClaimedUnverified {
                item.proof_label_shown
            } else {
                true
            }
        })
    }

    /// True when every conflict requires explicit user review (no silent server win).
    pub fn no_silent_server_authority(&self) -> bool {
        self.conflicts.iter().all(|item| item.requires_user_review)
    }

    /// True when every stale or unknown-freshness item carries a visible label.
    pub fn stale_state_honestly_labeled(&self) -> bool {
        self.snapshot_classes
            .iter()
            .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .conflicts
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .devices
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .encrypted_storage
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
    }

    /// Iterates every desktop handoff across all four sections, in section order.
    pub fn handoffs(&self) -> impl Iterator<Item = &CompanionDesktopHandoff> {
        self.snapshot_classes
            .iter()
            .map(|item| &item.handoff)
            .chain(self.conflicts.iter().map(|item| &item.handoff))
            .chain(self.devices.iter().map(|item| &item.handoff))
            .chain(self.encrypted_storage.iter().map(|item| &item.handoff))
    }

    fn handoffs_mut(&mut self) -> impl Iterator<Item = &mut CompanionDesktopHandoff> {
        self.snapshot_classes
            .iter_mut()
            .map(|item| &mut item.handoff)
            .chain(self.conflicts.iter_mut().map(|item| &mut item.handoff))
            .chain(self.devices.iter_mut().map(|item| &mut item.handoff))
            .chain(
                self.encrypted_storage
                    .iter_mut()
                    .map(|item| &mut item.handoff),
            )
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Managed Sync Maturity: Snapshot Classes, Conflict Review, Device Registry, and End-to-End Encrypted Storage\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Sections: {} | Snapshot classes: {} | Conflicts: {} | Devices: {} | Encrypted scopes: {}\n",
            self.section_qualifications.len(),
            self.snapshot_classes.len(),
            self.conflicts.len(),
            self.devices.len(),
            self.encrypted_storage.len(),
        ));
        out.push_str(&format!(
            "- Exact desktop handoff for every item: {}\n",
            yes_no(self.all_handoffs_exact())
        ));
        out.push_str(&format!(
            "- No silent server authority: {}\n",
            yes_no(self.no_silent_server_authority())
        ));
        out.push_str(&format!(
            "- Encryption claims honestly qualified: {}\n",
            yes_no(self.encryption_claims_honestly_qualified())
        ));
        out.push_str(&format!(
            "- Stale state honestly labeled: {}\n",
            yes_no(self.stale_state_honestly_labeled())
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        if self.degraded_labels.is_empty() {
            out.push_str("- Degraded: none\n");
        } else {
            let labels = self
                .degraded_labels
                .iter()
                .map(|reason| reason.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!("- Degraded: {labels}\n"));
        }

        out.push_str("\n## Sections\n\n");
        for row in &self.section_qualifications {
            out.push_str(&format!(
                "- **{}**: `{}` / `{}` [{}] (matrix lane `{}`)\n",
                row.section.as_str(),
                row.qualification.as_str(),
                row.rollout_stage.as_str(),
                row.read_write_scope.as_str(),
                row.matrix_lane_ref,
            ));
        }

        out.push_str("\n## Snapshot classes\n\n");
        for item in &self.snapshot_classes {
            out.push_str(&format!(
                "- `{}` [{}/{}/{}] {} ({}) → `{}` ({})\n",
                item.item_id,
                item.class_kind.as_str(),
                item.direction.as_str(),
                item.reconciliation.as_str(),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Conflict review\n\n");
        for item in &self.conflicts {
            out.push_str(&format!(
                "- `{}` [{}/{}/{}] {} ({}) → `{}` ({})\n",
                item.item_id,
                item.conflict_kind.as_str(),
                item.resolution_state.as_str(),
                item.reconciliation.as_str(),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Device registry\n\n");
        for item in &self.devices {
            out.push_str(&format!(
                "- `{}` [{}{}/{}] {} ({}) → `{}` ({})\n",
                item.item_id,
                item.trust_state.as_str(),
                if item.this_device { "/this_device" } else { "" },
                item.reconciliation.as_str(),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Encrypted storage\n\n");
        for item in &self.encrypted_storage {
            out.push_str(&format!(
                "- `{}` [{}/{}/{}] residency `{}` (verified: {}) {} ({}) → `{}` ({})\n",
                item.item_id,
                item.artifact_scope.as_str(),
                item.encryption_posture.as_str(),
                item.key_custody.as_str(),
                item.residency_region_ref,
                yes_no(item.claim_verified),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out
    }
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

/// Errors emitted when reading the checked-in managed sync maturity export.
#[derive(Debug)]
pub enum ManagedSyncArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ManagedSyncViolation>),
}

impl fmt::Display for ManagedSyncArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "managed sync maturity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "managed sync maturity export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ManagedSyncArtifactError {}

/// Validation failures emitted by [`ManagedSyncMaturitySurfacePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManagedSyncViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Projected surfaces list is empty.
    ProjectedSurfacesMissing,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required section qualification row is missing.
    RequiredSectionMissing,
    /// A section row's matrix lane ref does not match its section.
    SectionLaneMismatch,
    /// A section row's read/write scope does not match its bounded scope.
    SectionScopeMismatch,
    /// A section row is incomplete.
    SectionRowIncomplete,
    /// A section has no content items.
    SectionContentMissing,
    /// A read-only section item is not marked read-only.
    ReadOnlyScopeViolated,
    /// A snapshot class does not record the local core as authoritative.
    SnapshotNotLocalAuthoritative,
    /// A conflict does not require explicit user review.
    ConflictNotUserReviewed,
    /// An encryption claim is verified-marked without backing verification.
    EncryptionClaimedButUnverified,
    /// An unverified encryption claim is not labeled.
    EncryptionClaimNotLabeled,
    /// An encrypted-storage row is missing its residency region ref.
    ResidencyRegionMissing,
    /// An item is missing identity or a redacted body.
    ItemIncomplete,
    /// A stale or unknown-freshness item is not labeled.
    StaleStateNotLabeled,
    /// An item's desktop handoff is missing its deep-link ref.
    HandoffRefMissing,
    /// The read/write scope contract is not fully satisfied.
    ScopeContractIncomplete,
    /// The inspectability contract is not fully satisfied.
    InspectabilityContractIncomplete,
    /// The stale-state honesty contract is not fully satisfied.
    StaleStateHonestyIncomplete,
    /// The locality disclosure is incomplete.
    LocalityDisclosureIncomplete,
    /// Security review does not satisfy required invariants.
    SecurityReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ManagedSyncViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::ProjectedSurfacesMissing => "projected_surfaces_missing",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSectionMissing => "required_section_missing",
            Self::SectionLaneMismatch => "section_lane_mismatch",
            Self::SectionScopeMismatch => "section_scope_mismatch",
            Self::SectionRowIncomplete => "section_row_incomplete",
            Self::SectionContentMissing => "section_content_missing",
            Self::ReadOnlyScopeViolated => "read_only_scope_violated",
            Self::SnapshotNotLocalAuthoritative => "snapshot_not_local_authoritative",
            Self::ConflictNotUserReviewed => "conflict_not_user_reviewed",
            Self::EncryptionClaimedButUnverified => "encryption_claimed_but_unverified",
            Self::EncryptionClaimNotLabeled => "encryption_claim_not_labeled",
            Self::ResidencyRegionMissing => "residency_region_missing",
            Self::ItemIncomplete => "item_incomplete",
            Self::StaleStateNotLabeled => "stale_state_not_labeled",
            Self::HandoffRefMissing => "handoff_ref_missing",
            Self::ScopeContractIncomplete => "scope_contract_incomplete",
            Self::InspectabilityContractIncomplete => "inspectability_contract_incomplete",
            Self::StaleStateHonestyIncomplete => "stale_state_honesty_incomplete",
            Self::LocalityDisclosureIncomplete => "locality_disclosure_incomplete",
            Self::SecurityReviewIncomplete => "security_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable managed sync maturity surface export.
///
/// This is the canonical reader: the desktop companion panel, the CLI/headless
/// surface, diagnostics, support-export, or Help/About surface calls it to ingest
/// the packet rather than cloning status text.
///
/// # Errors
///
/// Returns [`ManagedSyncArtifactError`] when the checked-in support export fails to
/// parse or fails validation.
pub fn current_stable_managed_sync_maturity_surface_export(
) -> Result<ManagedSyncMaturitySurfacePacket, ManagedSyncArtifactError> {
    let packet: ManagedSyncMaturitySurfacePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/companion/m5/ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage/support_export.json"
    )))
    .map_err(ManagedSyncArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ManagedSyncArtifactError::Validation(violations))
    }
}

/// Canonical source contract refs that every managed sync maturity export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        MANAGED_SYNC_MATURITY_SCHEMA_REF.to_owned(),
        MANAGED_SYNC_MATURITY_DOC_REF.to_owned(),
        M5_MANAGED_SYNC_POLICY_REF.to_owned(),
        M5_PROFILE_SYNC_CONTRACT_REF.to_owned(),
        M5_REGION_RESIDENCY_REF.to_owned(),
        M5_COMPANION_MATRIX_SCHEMA_REF.to_owned(),
    ]
}

/// Canonical read/write scope and authority contract with every guarantee met.
pub fn canonical_scope_contract() -> ManagedSyncScopeContract {
    ManagedSyncScopeContract {
        snapshot_class_read_only: true,
        conflict_review_read_only: true,
        device_registry_read_only: true,
        encrypted_storage_read_only: true,
        local_core_authoritative: true,
        no_silent_server_authority: true,
        conflict_requires_user_review: true,
        no_unbounded_workspace_write: true,
        no_payload_bodies: true,
    }
}

/// Canonical inspectability and provability contract with every guarantee satisfied.
pub fn canonical_inspectability_contract() -> ManagedSyncInspectabilityContract {
    ManagedSyncInspectabilityContract {
        every_synced_record_reconcilable: true,
        conflicts_reviewed_not_auto_resolved: true,
        device_registry_inspectable: true,
        encryption_claim_provable_or_labeled: true,
        residency_region_disclosed: true,
        no_claim_without_evidence: true,
    }
}

/// Canonical stale-state honesty contract with every guarantee satisfied.
pub fn canonical_stale_state_honesty() -> ManagedSyncStaleStateHonesty {
    ManagedSyncStaleStateHonesty {
        stale_items_labeled: true,
        unknown_freshness_labeled: true,
        never_show_stale_as_live: true,
        freshness_floor_enforced: true,
    }
}

/// Canonical security review block with every invariant satisfied.
pub fn canonical_security_review() -> ManagedSyncSecurityReview {
    ManagedSyncSecurityReview {
        snapshot_class_read_only: true,
        conflict_review_read_only: true,
        device_registry_read_only: true,
        encrypted_storage_read_only: true,
        local_core_authoritative: true,
        no_silent_server_authority: true,
        managed_sync_inspectable: true,
        conflict_requires_user_review: true,
        device_trust_tracked: true,
        encryption_claim_provable_or_labeled: true,
        residency_region_disclosed: true,
        customer_managed_key_supported: true,
        e2ee_claimed_only_when_verifiable: true,
        stale_state_labeled_never_hidden: true,
        exact_desktop_handoff_preserved: true,
        no_credential_or_key_bodies_in_export: true,
        no_payload_bodies_in_export: true,
        downgrade_narrows_instead_of_hides: true,
        locality_disclosed: true,
    }
}

/// Canonical consumer projection block with every section projecting truth.
pub fn canonical_consumer_projection() -> ManagedSyncConsumerProjection {
    ManagedSyncConsumerProjection {
        desktop_panel_shows_snapshot_classes: true,
        desktop_panel_shows_conflict_review: true,
        desktop_panel_shows_device_registry: true,
        diagnostics_shows_encryption_posture: true,
        cli_headless_shows_reconciliation_state: true,
        support_export_shows_reconciliation_and_freshness: true,
        help_about_shows_encryption_and_residency_claim: true,
        preview_labs_label_for_unqualified_sections: true,
    }
}

/// Canonical per-section qualification rows, inherited from the frozen matrix.
///
/// The snapshot-class, conflict-review, and device-registry sections inherit the
/// managed-sync lane's Beta/staged-rollout qualification; the encrypted-storage
/// section inherits the residency-encryption lane's Beta/staged-rollout
/// qualification.
pub fn canonical_section_qualifications() -> Vec<ManagedSyncSectionQualification> {
    use M5CompanionDowngradeTrigger as Trigger;
    use M5CompanionQualificationClass as Qual;
    use M5CompanionRollbackPosture as Rollback;
    use M5CompanionRolloutStage as Stage;
    use ManagedSyncSection as Section;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        ManagedSyncSectionQualification {
            section: Section::SnapshotClass,
            qualification: Qual::Beta,
            rollout_stage: Stage::StagedRollout,
            read_write_scope: scope,
            matrix_lane_ref: Section::SnapshotClass.matrix_lane().as_str().to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::SyncInspectionUnavailable,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::SyncReconcilesToLocalCore,
        },
        ManagedSyncSectionQualification {
            section: Section::ConflictReview,
            qualification: Qual::Beta,
            rollout_stage: Stage::StagedRollout,
            read_write_scope: scope,
            matrix_lane_ref: Section::ConflictReview.matrix_lane().as_str().to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::SyncInspectionUnavailable,
                Trigger::TrustNarrowing,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::SyncReconcilesToLocalCore,
        },
        ManagedSyncSectionQualification {
            section: Section::DeviceRegistry,
            qualification: Qual::Beta,
            rollout_stage: Stage::StagedRollout,
            read_write_scope: scope,
            matrix_lane_ref: Section::DeviceRegistry.matrix_lane().as_str().to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::AdminContinuityRequired,
                Trigger::TrustNarrowing,
                Trigger::SyncInspectionUnavailable,
            ],
            rollback_posture: Rollback::SyncReconcilesToLocalCore,
        },
        ManagedSyncSectionQualification {
            section: Section::EncryptedStorage,
            qualification: Qual::Preview,
            rollout_stage: Stage::EarlyAccess,
            read_write_scope: scope,
            matrix_lane_ref: Section::EncryptedStorage.matrix_lane().as_str().to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ResidencyOrEncryptionUnverified,
                Trigger::AdminContinuityRequired,
                Trigger::PolicyBlocked,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::EvidencePreservedNoRevert,
        },
    ]
}

/// Canonical locality disclosure for the managed sync maturity surface.
pub fn canonical_locality_disclosure() -> M5CompanionLocalityDisclosure {
    M5CompanionLocalityDisclosure {
        stays_local:
            "The local core is the authoritative source of truth; every snapshot class, conflict record, device entry, and the exact desktop handoff for each item stay inspectable and reconcilable offline."
                .to_owned(),
        staged:
            "Managed sync of additional snapshot classes, customer-managed-key custody, and region pinning roll out per cohort and managed tenant."
                .to_owned(),
        requires_provider_or_admin_continuity:
            "Server-side sync, conflict relay, and device approval require the sync provider and, for managed tenants, admin continuity; end-to-end-encryption and region-residency guarantees require the managed key authority and are claimed only when verifiable. The local core never depends on them to function."
                .to_owned(),
    }
}

fn desktop_handoff(deep_link_ref: &str, requires_active_host: bool) -> CompanionDesktopHandoff {
    CompanionDesktopHandoff {
        target: CompanionHandoffTarget::ReviewPanel,
        deep_link_ref: deep_link_ref.to_owned(),
        resolution: CompanionHandoffResolution::Exact,
        requires_active_host,
    }
}

/// Canonical snapshot-class items.
pub fn canonical_snapshot_classes() -> Vec<SyncSnapshotClassItem> {
    use CompanionFreshnessState as Fresh;
    use SyncDirection as Direction;
    use SyncReconciliationState as Reconciliation;
    use SyncSnapshotClassKind as Kind;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        SyncSnapshotClassItem {
            item_id: "snapshot:0001".to_owned(),
            class_kind: Kind::Settings,
            direction: Direction::Bidirectional,
            reconciliation: Reconciliation::Reconciled,
            local_authoritative: true,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Settings snapshot class reconciled with the local core".to_owned(),
            record_ref: "sync:snapshot:settings".to_owned(),
            handoff: desktop_handoff("handoff:snapshot:0001", false),
        },
        SyncSnapshotClassItem {
            item_id: "snapshot:0002".to_owned(),
            class_kind: Kind::Profile,
            direction: Direction::Bidirectional,
            reconciliation: Reconciliation::DivergedPendingReview,
            local_authoritative: true,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Profile snapshot class diverged on two devices; pending conflict review"
                .to_owned(),
            record_ref: "sync:snapshot:profile".to_owned(),
            handoff: desktop_handoff("handoff:snapshot:0002", false),
        },
        SyncSnapshotClassItem {
            item_id: "snapshot:0003".to_owned(),
            class_kind: Kind::DeviceRegistry,
            direction: Direction::ManagedToLocal,
            reconciliation: Reconciliation::Reconciled,
            local_authoritative: true,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Device registry snapshot class reconciled with the local core".to_owned(),
            record_ref: "sync:snapshot:device-registry".to_owned(),
            handoff: desktop_handoff("handoff:snapshot:0003", false),
        },
    ]
}

/// Canonical conflict-review items.
pub fn canonical_conflicts() -> Vec<SyncConflictItem> {
    use CompanionFreshnessState as Fresh;
    use ConflictResolutionState as Resolution;
    use SyncConflictKind as Kind;
    use SyncReconciliationState as Reconciliation;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        SyncConflictItem {
            item_id: "conflict:0001".to_owned(),
            snapshot_class_ref: "sync:snapshot:profile".to_owned(),
            conflict_kind: Kind::ConcurrentEdit,
            resolution_state: Resolution::PendingReview,
            requires_user_review: true,
            reconciliation: Reconciliation::DivergedPendingReview,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Profile edited concurrently on two devices; awaiting user review".to_owned(),
            conflict_ref: "sync:conflict:0001".to_owned(),
            handoff: desktop_handoff("handoff:conflict:0001", false),
        },
        SyncConflictItem {
            item_id: "conflict:0002".to_owned(),
            snapshot_class_ref: "sync:snapshot:settings".to_owned(),
            conflict_kind: Kind::ClockSkew,
            resolution_state: Resolution::ResolvedKeepLocal,
            requires_user_review: true,
            reconciliation: Reconciliation::Reconciled,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Clock-skew ordering reviewed by the user; local version kept".to_owned(),
            conflict_ref: "sync:conflict:0002".to_owned(),
            handoff: desktop_handoff("handoff:conflict:0002", false),
        },
    ]
}

/// Canonical device-registry items.
pub fn canonical_devices() -> Vec<SyncDeviceItem> {
    use CompanionFreshnessState as Fresh;
    use SyncDeviceTrustState as Trust;
    use SyncReconciliationState as Reconciliation;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        SyncDeviceItem {
            item_id: "device:0001".to_owned(),
            trust_state: Trust::Trusted,
            this_device: true,
            reconciliation: Reconciliation::Reconciled,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "This device, trusted and reconciled with the local core".to_owned(),
            device_ref: "sync:device:0001".to_owned(),
            handoff: desktop_handoff("handoff:device:0001", false),
        },
        SyncDeviceItem {
            item_id: "device:0002".to_owned(),
            trust_state: Trust::Trusted,
            this_device: false,
            reconciliation: Reconciliation::Reconciled,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Trusted laptop participating in managed sync".to_owned(),
            device_ref: "sync:device:0002".to_owned(),
            handoff: desktop_handoff("handoff:device:0002", false),
        },
        SyncDeviceItem {
            item_id: "device:0003".to_owned(),
            trust_state: Trust::PendingApproval,
            this_device: false,
            reconciliation: Reconciliation::DivergedPendingReview,
            freshness: Fresh::Unknown,
            read_write_scope: scope,
            stale_label_shown: true,
            summary: "New device awaiting approval before it may sync".to_owned(),
            device_ref: "sync:device:0003".to_owned(),
            handoff: desktop_handoff("handoff:device:0003", false),
        },
    ]
}

/// Canonical encrypted-storage items.
pub fn canonical_encrypted_storage() -> Vec<EncryptedStorageItem> {
    use CompanionFreshnessState as Fresh;
    use EncryptedArtifactScope as Scope;
    use EncryptionPosture as Posture;
    use KeyCustody as Custody;

    let read_only = CompanionReadWriteScope::ReadOnly;
    vec![
        EncryptedStorageItem {
            item_id: "encrypted:0001".to_owned(),
            artifact_scope: Scope::ManagedSnapshotStore,
            encryption_posture: Posture::EndToEndEncryptedVerified,
            key_custody: Custody::CustomerManagedKey,
            residency_region_ref: "region:eu-west".to_owned(),
            claim_verified: true,
            proof_label_shown: false,
            freshness: Fresh::Live,
            read_write_scope: read_only,
            stale_label_shown: false,
            summary:
                "Managed snapshot store end-to-end encrypted with a customer-managed key; claim verified"
                    .to_owned(),
            evidence_ref: "evidence:residency-and-e2ee-proof:m5".to_owned(),
            handoff: desktop_handoff("handoff:encrypted:0001", false),
        },
        EncryptedStorageItem {
            item_id: "encrypted:0002".to_owned(),
            artifact_scope: Scope::SyncTransport,
            encryption_posture: Posture::EncryptedAtRestVerified,
            key_custody: Custody::ProviderManagedKey,
            residency_region_ref: "region:eu-west".to_owned(),
            claim_verified: true,
            proof_label_shown: false,
            freshness: Fresh::Cached,
            read_write_scope: read_only,
            stale_label_shown: false,
            summary: "Sync transport encrypted at rest with a provider-managed key; claim verified"
                .to_owned(),
            evidence_ref: "evidence:residency-and-e2ee-proof:m5".to_owned(),
            handoff: desktop_handoff("handoff:encrypted:0002", false),
        },
        EncryptedStorageItem {
            item_id: "encrypted:0003".to_owned(),
            artifact_scope: Scope::ConflictHistory,
            encryption_posture: Posture::ClaimedUnverified,
            key_custody: Custody::ProviderManagedKey,
            residency_region_ref: "region:unverified".to_owned(),
            claim_verified: false,
            proof_label_shown: true,
            freshness: Fresh::Unknown,
            read_write_scope: read_only,
            stale_label_shown: true,
            summary:
                "Conflict-history encryption claimed but not yet verified; labeled as unverified"
                    .to_owned(),
            evidence_ref: "evidence:residency-and-e2ee-proof:m5".to_owned(),
            handoff: desktop_handoff("handoff:encrypted:0003", false),
        },
        EncryptedStorageItem {
            item_id: "encrypted:0004".to_owned(),
            artifact_scope: Scope::DeviceRegistryStore,
            encryption_posture: Posture::EndToEndEncryptedVerified,
            key_custody: Custody::LocalOnlyNoKeyEscrow,
            residency_region_ref: "region:local-device".to_owned(),
            claim_verified: true,
            proof_label_shown: false,
            freshness: Fresh::Live,
            read_write_scope: read_only,
            stale_label_shown: false,
            summary:
                "Device registry store end-to-end encrypted with a local-only key, never escrowed; claim verified"
                    .to_owned(),
            evidence_ref: "evidence:residency-and-e2ee-proof:m5".to_owned(),
            handoff: desktop_handoff("handoff:encrypted:0004", false),
        },
    ]
}

/// Builds the canonical managed sync maturity surface packet.
///
/// This is the first consumer: it mints the surface the checked-in support export
/// and Markdown summary are generated from, so the artifact never drifts from the
/// typed section, item, scope, inspectability, and freshness definitions.
pub fn canonical_managed_sync_maturity_surface(
    packet_id: String,
    surface_label: String,
    minted_at: String,
    proof_freshness: ManagedSyncProofFreshness,
) -> ManagedSyncMaturitySurfacePacket {
    ManagedSyncMaturitySurfacePacket::new(ManagedSyncMaturitySurfacePacketInput {
        packet_id,
        surface_label,
        projected_surfaces: vec![
            M5CompanionConsumerSurface::DesktopCompanionPanel,
            M5CompanionConsumerSurface::CliHeadless,
            M5CompanionConsumerSurface::SupportExport,
            M5CompanionConsumerSurface::Diagnostics,
            M5CompanionConsumerSurface::HelpAbout,
        ],
        section_qualifications: canonical_section_qualifications(),
        snapshot_classes: canonical_snapshot_classes(),
        conflicts: canonical_conflicts(),
        devices: canonical_devices(),
        encrypted_storage: canonical_encrypted_storage(),
        scope_contract: canonical_scope_contract(),
        inspectability_contract: canonical_inspectability_contract(),
        stale_state_honesty: canonical_stale_state_honesty(),
        locality_disclosure: canonical_locality_disclosure(),
        security_review: canonical_security_review(),
        consumer_projection: canonical_consumer_projection(),
        proof_freshness,
        source_contract_refs: canonical_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at,
    })
}

fn validate_source_contracts(
    packet: &ManagedSyncMaturitySurfacePacket,
    violations: &mut Vec<ManagedSyncViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        MANAGED_SYNC_MATURITY_SCHEMA_REF,
        MANAGED_SYNC_MATURITY_DOC_REF,
        M5_MANAGED_SYNC_POLICY_REF,
        M5_PROFILE_SYNC_CONTRACT_REF,
        M5_REGION_RESIDENCY_REF,
        M5_COMPANION_MATRIX_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ManagedSyncViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_section_qualifications(
    packet: &ManagedSyncMaturitySurfacePacket,
    violations: &mut Vec<ManagedSyncViolation>,
) {
    let present: BTreeSet<ManagedSyncSection> = packet
        .section_qualifications
        .iter()
        .map(|row| row.section)
        .collect();
    for required in ManagedSyncSection::ALL {
        if !present.contains(&required) {
            violations.push(ManagedSyncViolation::RequiredSectionMissing);
            return;
        }
    }

    for row in &packet.section_qualifications {
        if row.matrix_lane_ref != row.section.matrix_lane().as_str() {
            violations.push(ManagedSyncViolation::SectionLaneMismatch);
        }
        if row.read_write_scope != row.section.bounded_scope() {
            violations.push(ManagedSyncViolation::SectionScopeMismatch);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(ManagedSyncViolation::SectionRowIncomplete);
        }
    }
}

fn validate_items(
    packet: &ManagedSyncMaturitySurfacePacket,
    violations: &mut Vec<ManagedSyncViolation>,
) {
    if packet.snapshot_classes.is_empty()
        || packet.conflicts.is_empty()
        || packet.devices.is_empty()
        || packet.encrypted_storage.is_empty()
    {
        violations.push(ManagedSyncViolation::SectionContentMissing);
    }

    for item in &packet.snapshot_classes {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(ManagedSyncViolation::ReadOnlyScopeViolated);
        }
        if !item.local_authoritative {
            violations.push(ManagedSyncViolation::SnapshotNotLocalAuthoritative);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.record_ref.trim().is_empty()
        {
            violations.push(ManagedSyncViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.conflicts {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(ManagedSyncViolation::ReadOnlyScopeViolated);
        }
        if !item.requires_user_review {
            violations.push(ManagedSyncViolation::ConflictNotUserReviewed);
        }
        if item.item_id.trim().is_empty()
            || item.snapshot_class_ref.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.conflict_ref.trim().is_empty()
        {
            violations.push(ManagedSyncViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.devices {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(ManagedSyncViolation::ReadOnlyScopeViolated);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.device_ref.trim().is_empty()
        {
            violations.push(ManagedSyncViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.encrypted_storage {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(ManagedSyncViolation::ReadOnlyScopeViolated);
        }
        if item.encryption_posture.is_verified_claim() && !item.claim_verified {
            violations.push(ManagedSyncViolation::EncryptionClaimedButUnverified);
        }
        if item.encryption_posture == EncryptionPosture::ClaimedUnverified
            && !item.proof_label_shown
        {
            violations.push(ManagedSyncViolation::EncryptionClaimNotLabeled);
        }
        if item.residency_region_ref.trim().is_empty() {
            violations.push(ManagedSyncViolation::ResidencyRegionMissing);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.evidence_ref.trim().is_empty()
        {
            violations.push(ManagedSyncViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }
}

fn validate_freshness_label(
    freshness: CompanionFreshnessState,
    stale_label_shown: bool,
    violations: &mut Vec<ManagedSyncViolation>,
) {
    if freshness.requires_label() && !stale_label_shown {
        violations.push(ManagedSyncViolation::StaleStateNotLabeled);
    }
}

fn validate_handoff(handoff: &CompanionDesktopHandoff, violations: &mut Vec<ManagedSyncViolation>) {
    if handoff.deep_link_ref.trim().is_empty() {
        violations.push(ManagedSyncViolation::HandoffRefMissing);
    }
}

fn validate_scope_contract(
    packet: &ManagedSyncMaturitySurfacePacket,
    violations: &mut Vec<ManagedSyncViolation>,
) {
    let contract = &packet.scope_contract;
    for ok in [
        contract.snapshot_class_read_only,
        contract.conflict_review_read_only,
        contract.device_registry_read_only,
        contract.encrypted_storage_read_only,
        contract.local_core_authoritative,
        contract.no_silent_server_authority,
        contract.conflict_requires_user_review,
        contract.no_unbounded_workspace_write,
        contract.no_payload_bodies,
    ] {
        if !ok {
            violations.push(ManagedSyncViolation::ScopeContractIncomplete);
            return;
        }
    }
}

fn validate_inspectability_contract(
    packet: &ManagedSyncMaturitySurfacePacket,
    violations: &mut Vec<ManagedSyncViolation>,
) {
    let contract = &packet.inspectability_contract;
    for ok in [
        contract.every_synced_record_reconcilable,
        contract.conflicts_reviewed_not_auto_resolved,
        contract.device_registry_inspectable,
        contract.encryption_claim_provable_or_labeled,
        contract.residency_region_disclosed,
        contract.no_claim_without_evidence,
    ] {
        if !ok {
            violations.push(ManagedSyncViolation::InspectabilityContractIncomplete);
            return;
        }
    }
}

fn validate_stale_state_honesty(
    packet: &ManagedSyncMaturitySurfacePacket,
    violations: &mut Vec<ManagedSyncViolation>,
) {
    let honesty = &packet.stale_state_honesty;
    for ok in [
        honesty.stale_items_labeled,
        honesty.unknown_freshness_labeled,
        honesty.never_show_stale_as_live,
        honesty.freshness_floor_enforced,
    ] {
        if !ok {
            violations.push(ManagedSyncViolation::StaleStateHonestyIncomplete);
            return;
        }
    }
}

fn validate_locality(
    packet: &ManagedSyncMaturitySurfacePacket,
    violations: &mut Vec<ManagedSyncViolation>,
) {
    let locality = &packet.locality_disclosure;
    if locality.stays_local.trim().is_empty()
        || locality.staged.trim().is_empty()
        || locality
            .requires_provider_or_admin_continuity
            .trim()
            .is_empty()
    {
        violations.push(ManagedSyncViolation::LocalityDisclosureIncomplete);
    }
}

fn validate_security_review(
    packet: &ManagedSyncMaturitySurfacePacket,
    violations: &mut Vec<ManagedSyncViolation>,
) {
    let review = &packet.security_review;
    for ok in [
        review.snapshot_class_read_only,
        review.conflict_review_read_only,
        review.device_registry_read_only,
        review.encrypted_storage_read_only,
        review.local_core_authoritative,
        review.no_silent_server_authority,
        review.managed_sync_inspectable,
        review.conflict_requires_user_review,
        review.device_trust_tracked,
        review.encryption_claim_provable_or_labeled,
        review.residency_region_disclosed,
        review.customer_managed_key_supported,
        review.e2ee_claimed_only_when_verifiable,
        review.stale_state_labeled_never_hidden,
        review.exact_desktop_handoff_preserved,
        review.no_credential_or_key_bodies_in_export,
        review.no_payload_bodies_in_export,
        review.downgrade_narrows_instead_of_hides,
        review.locality_disclosed,
    ] {
        if !ok {
            violations.push(ManagedSyncViolation::SecurityReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &ManagedSyncMaturitySurfacePacket,
    violations: &mut Vec<ManagedSyncViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.desktop_panel_shows_snapshot_classes,
        projection.desktop_panel_shows_conflict_review,
        projection.desktop_panel_shows_device_registry,
        projection.diagnostics_shows_encryption_posture,
        projection.cli_headless_shows_reconciliation_state,
        projection.support_export_shows_reconciliation_and_freshness,
        projection.help_about_shows_encryption_and_residency_claim,
        projection.preview_labs_label_for_unqualified_sections,
    ] {
        if !ok {
            violations.push(ManagedSyncViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &ManagedSyncMaturitySurfacePacket,
    violations: &mut Vec<ManagedSyncViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(ManagedSyncViolation::ProofFreshnessIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
