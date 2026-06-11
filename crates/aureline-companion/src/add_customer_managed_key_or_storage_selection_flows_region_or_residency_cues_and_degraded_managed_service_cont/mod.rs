//! Customer-managed-key and storage selection flows, region/residency cues, and
//! degraded managed-service continuity, projected as a downgrade-aware truth packet.
//!
//! This module owns the export-safe truth packet for the residency and offboarding
//! depth lane. It projects four sections: the **key custody selection** flow that
//! records which key-custody options (customer-managed-key, provider-managed-key,
//! local-only-no-escrow) are offered and which is active, the **storage selection**
//! flow that records which storage locations are offered and which is active, the
//! **residency cues** that disclose where each managed artifact scope resides and
//! whether the residency pin is verified, and the **managed-service continuity**
//! rows that record, per managed-service capability, what stays local, what is
//! staged, and what requires provider or admin continuity when the managed service
//! degrades. It binds the first three sections to the frozen M5 companion-matrix
//! [`M5CompanionMatrixLane::ResidencyEncryption`] lane and the continuity section to
//! the [`M5CompanionMatrixLane::OffboardingContinuity`] lane, and gives every item an
//! exact [`CompanionDesktopHandoff`] (reused from
//! [`crate::companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff`])
//! so opening an item always resumes the precise host context locally.
//!
//! The surface deliberately reuses the [`KeyCustody`] and [`EncryptionPosture`]
//! vocabulary from
//! [`crate::ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage`]
//! rather than inventing parallel key/encryption terms, and the
//! [`EncryptedArtifactScope`] vocabulary for the artifact scope a residency cue
//! describes.
//!
//! Three invariants make this surface safe to ship. First, **read-only selection,
//! with the local core authoritative**: every section is read-only, the surface
//! *projects* the selection flow but never applies a selection — a key-custody,
//! storage-location, or residency change is applied by the local core, never authored
//! from this surface — and the local-only key and local-first storage options are
//! always available as a fallback so a managed degradation never strands the user.
//! Second, **provable where claimed**: a key-custody option claims a verified
//! encryption posture, and a residency cue claims a verified region pin, only when it
//! is backed by evidence; an unverifiable claim narrows
//! ([`EncryptionPosture::ClaimedUnverified`], [`ResidencyPinState::PinnedUnverified`])
//! and is labeled rather than shown as a proven claim. Third, **local-first
//! continuity**: every managed-service continuity row records a
//! [`ContinuityPosture`] that says what stays local and what requires provider or
//! admin continuity, and asserts `local_work_preserved` so offboarding or a degraded
//! managed service never strands user-owned local work.
//!
//! [`KeyStorageResidencyContinuitySurfacePacket::apply_residency_continuity_degradation`]
//! narrows sections, narrows selection options to their local fallback, and downgrades
//! freshness, residency pins, and encryption claims from a per-observation signal —
//! when the key-management service is unavailable, the storage provider is
//! unavailable, proof is stale, the residency or encryption claim is unverified,
//! managed-tenant admin continuity is unavailable, the managed service is degraded,
//! the host session is inactive, or an upstream matrix lane narrowed — so CI or
//! release tooling degrades the surface honestly rather than show a selection it can
//! no longer apply, a residency pin it can no longer prove, or an encryption claim it
//! can no longer verify. Degraded state is labeled, never hidden, and the local
//! fallback always remains.
//!
//! [`canonical_key_storage_residency_continuity_surface`] builds the surface and
//! [`current_stable_key_storage_residency_continuity_surface_export`] reads and
//! validates the checked-in support export, so the desktop companion panel, the
//! CLI/headless surface, diagnostics, support exports, and Help/About ingest the
//! packet rather than cloning status text. Credential bodies, raw key material, raw
//! provider payloads, and raw storage record contents stay outside this boundary.
//!
//! The boundary schema is
//! [`schemas/companion/add-customer-managed-key-or-storage-selection-flows-region-or-residency-cues-and-degraded-managed-service-cont.schema.json`](../../../../schemas/companion/add-customer-managed-key-or-storage-selection-flows-region-or-residency-cues-and-degraded-managed-service-cont.schema.json).
//! The contract doc is
//! [`docs/companion/m5/add_customer_managed_key_or_storage_selection_flows_region_or_residency_cues_and_degraded_managed_service_cont.md`](../../../../docs/companion/m5/add_customer_managed_key_or_storage_selection_flows_region_or_residency_cues_and_degraded_managed_service_cont.md).
//! The protected fixture directory is
//! [`fixtures/companion/m5/add_customer_managed_key_or_storage_selection_flows_region_or_residency_cues_and_degraded_managed_service_cont/`](../../../../fixtures/companion/m5/add_customer_managed_key_or_storage_selection_flows_region_or_residency_cues_and_degraded_managed_service_cont/).

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
    M5_OFFBOARDING_CONTRACT_REF, M5_REGION_RESIDENCY_REF,
};
use crate::ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage::{
    EncryptedArtifactScope, EncryptionPosture, KeyCustody,
};
use crate::ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty::{
    CompanionFreshnessState, CompanionReadWriteScope,
};

/// Stable record-kind tag carried by [`KeyStorageResidencyContinuitySurfacePacket`].
pub const KEY_STORAGE_RESIDENCY_RECORD_KIND: &str =
    "add_customer_managed_key_or_storage_selection_flows_region_or_residency_cues_and_degraded_managed_service_cont";

/// Schema version for key/storage/residency/continuity surface records.
pub const KEY_STORAGE_RESIDENCY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const KEY_STORAGE_RESIDENCY_SCHEMA_REF: &str =
    "schemas/companion/add-customer-managed-key-or-storage-selection-flows-region-or-residency-cues-and-degraded-managed-service-cont.schema.json";

/// Repo-relative path of the surface contract doc.
pub const KEY_STORAGE_RESIDENCY_DOC_REF: &str =
    "docs/companion/m5/add_customer_managed_key_or_storage_selection_flows_region_or_residency_cues_and_degraded_managed_service_cont.md";

/// Repo-relative path of the protected fixture directory.
pub const KEY_STORAGE_RESIDENCY_FIXTURE_DIR: &str =
    "fixtures/companion/m5/add_customer_managed_key_or_storage_selection_flows_region_or_residency_cues_and_degraded_managed_service_cont";

/// Repo-relative path of the checked support-export artifact.
pub const KEY_STORAGE_RESIDENCY_ARTIFACT_REF: &str =
    "artifacts/companion/m5/add_customer_managed_key_or_storage_selection_flows_region_or_residency_cues_and_degraded_managed_service_cont/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const KEY_STORAGE_RESIDENCY_SUMMARY_REF: &str =
    "artifacts/companion/m5/add_customer_managed_key_or_storage_selection_flows_region_or_residency_cues_and_degraded_managed_service_cont.md";

/// One of the four key/storage/residency/continuity sections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyStorageResidencySection {
    /// The customer-managed-key / key-custody selection flow.
    KeyCustodySelection,
    /// The storage-location selection flow.
    StorageSelection,
    /// The region/residency cues.
    ResidencyCue,
    /// The degraded managed-service continuity rows.
    ManagedServiceContinuity,
}

impl KeyStorageResidencySection {
    /// Every section, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::KeyCustodySelection,
        Self::StorageSelection,
        Self::ResidencyCue,
        Self::ManagedServiceContinuity,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyCustodySelection => "key_custody_selection",
            Self::StorageSelection => "storage_selection",
            Self::ResidencyCue => "residency_cue",
            Self::ManagedServiceContinuity => "managed_service_continuity",
        }
    }

    /// Frozen M5 companion-matrix lane this section inherits qualification from.
    ///
    /// The key-custody-selection, storage-selection, and residency-cue sections
    /// inherit from the [`M5CompanionMatrixLane::ResidencyEncryption`] lane; the
    /// managed-service-continuity section inherits from the
    /// [`M5CompanionMatrixLane::OffboardingContinuity`] lane.
    pub const fn matrix_lane(self) -> M5CompanionMatrixLane {
        match self {
            Self::KeyCustodySelection | Self::StorageSelection | Self::ResidencyCue => {
                M5CompanionMatrixLane::ResidencyEncryption
            }
            Self::ManagedServiceContinuity => M5CompanionMatrixLane::OffboardingContinuity,
        }
    }

    /// Read/write scope this section is bounded to.
    ///
    /// Every section is read-only: the surface projects the selection flow but never
    /// applies a selection. A key-custody, storage-location, or residency change is
    /// applied by the local core, never authored from this surface.
    pub const fn bounded_scope(self) -> CompanionReadWriteScope {
        CompanionReadWriteScope::ReadOnly
    }
}

/// State of a selectable key-custody or storage option in a selection flow.
///
/// The surface only projects the selection flow: an option is shown as currently
/// [`Self::Active`], offered and [`Self::Available`] to select, gated on admin
/// continuity ([`Self::RequiresAdminApproval`]), or [`Self::Unavailable`] in this
/// tier or region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionState {
    /// The option is currently selected and active.
    Active,
    /// The option is offered and may be selected.
    Available,
    /// The option is offered but requires managed-tenant admin approval first.
    RequiresAdminApproval,
    /// The option is not available in this tier or region.
    Unavailable,
}

impl SelectionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Available => "available",
            Self::RequiresAdminApproval => "requires_admin_approval",
            Self::Unavailable => "unavailable",
        }
    }

    /// Narrows an active or available selection to require admin approval; an option
    /// already gated or unavailable is kept.
    ///
    /// Used when a managed dependency (key management, storage provider, admin
    /// continuity) is lost: a non-local option can no longer be applied, so it narrows
    /// to require admin approval rather than continuing to look applicable.
    pub const fn narrowed_to_admin_gated(self) -> Self {
        match self {
            Self::Active | Self::Available => Self::RequiresAdminApproval,
            Self::RequiresAdminApproval | Self::Unavailable => self,
        }
    }
}

/// A storage-location option offered by the storage selection flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageLocationKind {
    /// Storage stays entirely on the local device.
    LocalOnly,
    /// A customer-managed storage bucket.
    CustomerManagedBucket,
    /// A provider-managed region.
    ProviderManagedRegion,
    /// Local-first storage with a managed mirror.
    HybridLocalFirst,
}

impl StorageLocationKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::CustomerManagedBucket => "customer_managed_bucket",
            Self::ProviderManagedRegion => "provider_managed_region",
            Self::HybridLocalFirst => "hybrid_local_first",
        }
    }

    /// True when this option keeps working with no managed-service continuity.
    ///
    /// The local-only and hybrid-local-first options remain a fallback when the
    /// managed storage provider is unavailable, so a storage degradation never strands
    /// the user.
    pub const fn is_local_fallback(self) -> bool {
        matches!(self, Self::LocalOnly | Self::HybridLocalFirst)
    }
}

/// Verification state of a region-residency pin.
///
/// A residency cue claims a verified pin only when it is backed by evidence; an
/// unverifiable claim narrows to [`Self::PinnedUnverified`] and is labeled rather than
/// shown as proven.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResidencyPinState {
    /// Pinned to a region and the pin is verified by evidence.
    PinnedVerified,
    /// Pinned to a region but the pin could not be verified.
    PinnedUnverified,
    /// Not pinned to a specific region; stated honestly.
    Unpinned,
}

impl ResidencyPinState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PinnedVerified => "pinned_verified",
            Self::PinnedUnverified => "pinned_unverified",
            Self::Unpinned => "unpinned",
        }
    }

    /// True when the pin is a verified residency claim.
    pub const fn is_verified_claim(self) -> bool {
        matches!(self, Self::PinnedVerified)
    }

    /// Downgrades a verified pin to [`Self::PinnedUnverified`]; other states are kept.
    pub const fn downgraded_to_unverified(self) -> Self {
        match self {
            Self::PinnedVerified => Self::PinnedUnverified,
            Self::PinnedUnverified | Self::Unpinned => self,
        }
    }
}

/// A managed-service capability whose degraded continuity is recorded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedServiceCapability {
    /// Managed sync of local state.
    ManagedSync,
    /// Customer-managed-key key management.
    KeyManagement,
    /// Region-residency pinning.
    ResidencyPinning,
    /// Managed-tenant device approval.
    DeviceApproval,
    /// The managed audit log.
    ManagedAuditLog,
}

impl ManagedServiceCapability {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedSync => "managed_sync",
            Self::KeyManagement => "key_management",
            Self::ResidencyPinning => "residency_pinning",
            Self::DeviceApproval => "device_approval",
            Self::ManagedAuditLog => "managed_audit_log",
        }
    }
}

/// Continuity posture of a managed-service capability when the managed service
/// degrades.
///
/// This is the explicit locality disclosure required of every managed surface: it
/// says what stays local and what requires provider or admin continuity, so no
/// managed surface can imply a hidden control plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityPosture {
    /// The capability is fully local; managed-service degradation has no effect.
    LocalCoreContinuesUnaffected,
    /// The managed feature degrades to a local fallback that keeps the user working.
    DegradedLocalFallback,
    /// The capability is suspended until the provider returns; local work preserved.
    RequiresProviderContinuity,
    /// The capability requires managed-tenant admin continuity; local work preserved.
    RequiresAdminContinuity,
}

impl ContinuityPosture {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalCoreContinuesUnaffected => "local_core_continues_unaffected",
            Self::DegradedLocalFallback => "degraded_local_fallback",
            Self::RequiresProviderContinuity => "requires_provider_continuity",
            Self::RequiresAdminContinuity => "requires_admin_continuity",
        }
    }

    /// True when a managed-service outage degrades this capability.
    ///
    /// A fully local capability is unaffected; every other posture is marked degraded
    /// when the managed service goes down, but local work is always preserved.
    pub const fn affected_by_managed_outage(self) -> bool {
        !matches!(self, Self::LocalCoreContinuesUnaffected)
    }

    /// True when this capability requires managed-tenant admin continuity.
    pub const fn requires_admin_continuity(self) -> bool {
        matches!(self, Self::RequiresAdminContinuity)
    }
}

/// A key-custody selection-flow row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyCustodySelectionItem {
    /// Stable item id.
    pub item_id: String,
    /// Key-custody option this row offers.
    pub offered_custody: KeyCustody,
    /// Selection state of the option.
    pub selection_state: SelectionState,
    /// Encryption posture this custody option yields, proved where claimed.
    pub encryption_posture: EncryptionPosture,
    /// True when the encryption claim is verified by evidence.
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
    /// Ref to the key-custody evidence. Carries no payload body or key material.
    pub evidence_ref: String,
    /// Exact desktop handoff to the key-custody selection.
    pub handoff: CompanionDesktopHandoff,
}

/// A storage-location selection-flow row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageSelectionItem {
    /// Stable item id.
    pub item_id: String,
    /// Storage-location option this row offers.
    pub offered_location: StorageLocationKind,
    /// Selection state of the option.
    pub selection_state: SelectionState,
    /// Opaque region-residency ref of the option. Carries no payload body.
    pub residency_region_ref: String,
    /// True when the residency claim of the option is verified by evidence.
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
    /// Ref to the storage-option record. Carries no payload body.
    pub record_ref: String,
    /// Exact desktop handoff to the storage selection.
    pub handoff: CompanionDesktopHandoff,
}

/// A region/residency cue row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidencyCueItem {
    /// Stable item id.
    pub item_id: String,
    /// Managed artifact scope this cue describes.
    pub artifact_scope: EncryptedArtifactScope,
    /// Opaque region-residency pin ref. Carries no payload body.
    pub residency_region_ref: String,
    /// Verification state of the residency pin, proved where claimed.
    pub pin_state: ResidencyPinState,
    /// True when the residency pin is verified by evidence.
    pub claim_verified: bool,
    /// True when an unverified pin carries a visible "claimed, not verified" label.
    pub proof_label_shown: bool,
    /// Freshness of the row.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the residency evidence. Carries no payload body.
    pub evidence_ref: String,
    /// Exact desktop handoff to the residency cue.
    pub handoff: CompanionDesktopHandoff,
}

/// A degraded managed-service continuity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedServiceContinuityItem {
    /// Stable item id.
    pub item_id: String,
    /// Managed-service capability this row describes.
    pub capability: ManagedServiceCapability,
    /// Continuity posture: what stays local and what requires provider/admin continuity.
    pub continuity_posture: ContinuityPosture,
    /// Always true: offboarding or degradation never strands user-owned local work.
    pub local_work_preserved: bool,
    /// True when the capability is currently degraded.
    pub degraded: bool,
    /// Freshness of the row.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the continuity record. Carries no payload body.
    pub record_ref: String,
    /// Exact desktop handoff to the continuity row.
    pub handoff: CompanionDesktopHandoff,
}

/// Per-section qualification inherited from the frozen M5 matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyStorageResidencySectionQualification {
    /// Section the row applies to.
    pub section: KeyStorageResidencySection,
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
pub struct ResidencyScopeContract {
    /// The key-custody-selection section is read-only.
    pub key_custody_selection_read_only: bool,
    /// The storage-selection section is read-only.
    pub storage_selection_read_only: bool,
    /// The residency-cue section is read-only.
    pub residency_cue_read_only: bool,
    /// The continuity section is read-only.
    pub continuity_read_only: bool,
    /// The local core stays the authoritative source of truth.
    pub local_core_authoritative: bool,
    /// A selection is applied by the local core, never authored from this surface.
    pub selection_applied_by_local_core_not_surface: bool,
    /// The surface never holds an unbounded write authority.
    pub no_unbounded_workspace_write: bool,
    /// Offboarding or degradation never strands user-owned local work.
    pub offboarding_never_strands_local_work: bool,
    /// No payload bodies cross the export boundary.
    pub no_payload_bodies: bool,
}

/// Selection and provability contract for the whole surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidencyProvabilityContract {
    /// Every key-custody option offered is disclosed.
    pub key_custody_options_disclosed: bool,
    /// A local-only key custody option is always offered as a fallback.
    pub local_only_key_fallback_offered: bool,
    /// A local-first storage option is always offered as a fallback.
    pub local_first_storage_fallback_offered: bool,
    /// The region residency is disclosed for every managed artifact scope.
    pub residency_region_disclosed: bool,
    /// The residency pin is provable or labeled as unverified.
    pub residency_claim_provable_or_labeled: bool,
    /// The encryption posture is provable or labeled as unverified.
    pub encryption_claim_provable_or_labeled: bool,
    /// No selection, residency, or encryption claim is made without backing evidence.
    pub no_claim_without_evidence: bool,
}

/// Stale-state honesty contract for the whole surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidencyStaleStateHonesty {
    /// Every stale item is labeled.
    pub stale_items_labeled: bool,
    /// Every unknown-freshness item is labeled.
    pub unknown_freshness_labeled: bool,
    /// A stale item is never shown as live.
    pub never_show_stale_as_live: bool,
    /// A freshness floor is enforced before an item is shown.
    pub freshness_floor_enforced: bool,
}

/// Continuity contract for the whole surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManagedServiceContinuityContract {
    /// The local core continues when the managed service degrades.
    pub local_core_continues_when_managed_degrades: bool,
    /// A degraded capability is labeled, not hidden.
    pub degraded_capability_labeled_not_hidden: bool,
    /// User-owned local work is never stranded.
    pub local_work_never_stranded: bool,
    /// Provider and admin continuity requirements are disclosed.
    pub provider_and_admin_continuity_disclosed: bool,
}

/// Security and privacy review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidencySecurityReview {
    /// The key-custody-selection section is read-only.
    pub key_custody_selection_read_only: bool,
    /// The storage-selection section is read-only.
    pub storage_selection_read_only: bool,
    /// The residency-cue section is read-only.
    pub residency_cue_read_only: bool,
    /// The continuity section is read-only.
    pub continuity_read_only: bool,
    /// The local core stays the authoritative source of truth.
    pub local_core_authoritative: bool,
    /// A selection is applied by the local core, never authored from this surface.
    pub selection_applied_by_local_core_not_surface: bool,
    /// Customer-managed-key custody is supported and recorded.
    pub customer_managed_key_supported: bool,
    /// A local-only key fallback is always offered.
    pub local_only_key_fallback_offered: bool,
    /// Encryption claims are provable or labeled as unverified.
    pub encryption_claim_provable_or_labeled: bool,
    /// Region residency is disclosed for every managed artifact scope.
    pub residency_region_disclosed: bool,
    /// Residency pins are provable or labeled as unverified.
    pub residency_claim_provable_or_labeled: bool,
    /// The managed service degrading never strands user-owned local work.
    pub local_work_never_stranded: bool,
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
pub struct ResidencyConsumerProjection {
    /// Desktop panel projects the key-custody selection flow.
    pub desktop_panel_shows_key_custody_selection: bool,
    /// Desktop panel projects the storage selection flow.
    pub desktop_panel_shows_storage_selection: bool,
    /// Desktop panel projects the residency cues.
    pub desktop_panel_shows_residency_cues: bool,
    /// Diagnostics shows the managed-service continuity posture.
    pub diagnostics_shows_continuity_posture: bool,
    /// CLI / headless shows the active selection and residency cues.
    pub cli_headless_shows_active_selection: bool,
    /// Support export shows the selection, residency, and continuity state.
    pub support_export_shows_selection_and_continuity: bool,
    /// Help / About shows the encryption and residency claim.
    pub help_about_shows_encryption_and_residency_claim: bool,
    /// Preview / Labs sections are visibly labeled when not qualified Stable.
    pub preview_labs_label_for_unqualified_sections: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResidencyProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the section.
    pub auto_narrow_on_stale: bool,
}

/// Per-observation signal fed to
/// [`KeyStorageResidencyContinuitySurfacePacket::apply_residency_continuity_degradation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResidencyContinuityObservation {
    /// True when the customer-managed-key key-management service is available.
    pub key_management_available: bool,
    /// True when the managed storage provider is available.
    pub storage_provider_available: bool,
    /// True when proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when the region-residency claims are verified.
    pub residency_verified: bool,
    /// True when the encryption claims are verified.
    pub encryption_verified: bool,
    /// True when managed-tenant admin continuity is available.
    pub admin_continuity_available: bool,
    /// True when the managed service is available (not degraded).
    pub managed_service_available: bool,
    /// True when an active desktop host session exists.
    pub host_session_active: bool,
    /// True when an upstream frozen matrix lane narrowed.
    pub upstream_matrix_narrowed: bool,
}

/// Reason a section has degraded below its qualified state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResidencyDegradedReason {
    /// The customer-managed-key key-management service is unavailable.
    KeyManagementUnavailable,
    /// The managed storage provider is unavailable.
    StorageProviderUnavailable,
    /// Proof has gone stale.
    ProofStale,
    /// The residency claim could not be verified.
    ResidencyUnverified,
    /// The encryption claim could not be verified.
    EncryptionUnverified,
    /// Managed-tenant admin continuity is unavailable.
    AdminContinuityUnavailable,
    /// The managed service is degraded.
    ManagedServiceDegraded,
    /// No active desktop host session.
    HostSessionInactive,
    /// An upstream frozen matrix lane narrowed.
    UpstreamMatrixNarrowed,
    /// One or more desktop handoff targets could not resolve exactly.
    HandoffTargetUnresolved,
    /// One or more item freshness states were downgraded to stale.
    FreshnessDowngradedToStale,
    /// One or more selection options narrowed to their local fallback.
    SelectionNarrowedToLocalFallback,
    /// One or more residency pins were downgraded to claimed-but-unverified.
    ResidencyClaimDowngraded,
    /// One or more encryption claims were downgraded to claimed-but-unverified.
    EncryptionClaimDowngraded,
}

impl ResidencyDegradedReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyManagementUnavailable => "key_management_unavailable",
            Self::StorageProviderUnavailable => "storage_provider_unavailable",
            Self::ProofStale => "proof_stale",
            Self::ResidencyUnverified => "residency_unverified",
            Self::EncryptionUnverified => "encryption_unverified",
            Self::AdminContinuityUnavailable => "admin_continuity_unavailable",
            Self::ManagedServiceDegraded => "managed_service_degraded",
            Self::HostSessionInactive => "host_session_inactive",
            Self::UpstreamMatrixNarrowed => "upstream_matrix_narrowed",
            Self::HandoffTargetUnresolved => "handoff_target_unresolved",
            Self::FreshnessDowngradedToStale => "freshness_downgraded_to_stale",
            Self::SelectionNarrowedToLocalFallback => "selection_narrowed_to_local_fallback",
            Self::ResidencyClaimDowngraded => "residency_claim_downgraded",
            Self::EncryptionClaimDowngraded => "encryption_claim_downgraded",
        }
    }
}

/// Constructor input for [`KeyStorageResidencyContinuitySurfacePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyStorageResidencyContinuitySurfacePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer surfaces that project this packet.
    pub projected_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Per-section qualification rows.
    pub section_qualifications: Vec<KeyStorageResidencySectionQualification>,
    /// Key-custody selection items.
    pub key_custody_selections: Vec<KeyCustodySelectionItem>,
    /// Storage selection items.
    pub storage_selections: Vec<StorageSelectionItem>,
    /// Residency cue items.
    pub residency_cues: Vec<ResidencyCueItem>,
    /// Managed-service continuity items.
    pub continuity_rows: Vec<ManagedServiceContinuityItem>,
    /// Read/write scope and authority contract.
    pub scope_contract: ResidencyScopeContract,
    /// Selection and provability contract.
    pub provability_contract: ResidencyProvabilityContract,
    /// Stale-state honesty contract.
    pub stale_state_honesty: ResidencyStaleStateHonesty,
    /// Continuity contract.
    pub continuity_contract: ManagedServiceContinuityContract,
    /// Locality disclosure.
    pub locality_disclosure: M5CompanionLocalityDisclosure,
    /// Security review block.
    pub security_review: ResidencySecurityReview,
    /// Consumer projection block.
    pub consumer_projection: ResidencyConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ResidencyProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe key/storage/residency/continuity surface packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyStorageResidencyContinuitySurfacePacket {
    /// Record kind; must equal [`KEY_STORAGE_RESIDENCY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`KEY_STORAGE_RESIDENCY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer surfaces that project this packet.
    pub projected_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Per-section qualification rows.
    pub section_qualifications: Vec<KeyStorageResidencySectionQualification>,
    /// Key-custody selection items.
    pub key_custody_selections: Vec<KeyCustodySelectionItem>,
    /// Storage selection items.
    pub storage_selections: Vec<StorageSelectionItem>,
    /// Residency cue items.
    pub residency_cues: Vec<ResidencyCueItem>,
    /// Managed-service continuity items.
    pub continuity_rows: Vec<ManagedServiceContinuityItem>,
    /// Read/write scope and authority contract.
    pub scope_contract: ResidencyScopeContract,
    /// Selection and provability contract.
    pub provability_contract: ResidencyProvabilityContract,
    /// Stale-state honesty contract.
    pub stale_state_honesty: ResidencyStaleStateHonesty,
    /// Continuity contract.
    pub continuity_contract: ManagedServiceContinuityContract,
    /// Locality disclosure.
    pub locality_disclosure: M5CompanionLocalityDisclosure,
    /// Security review block.
    pub security_review: ResidencySecurityReview,
    /// Consumer projection block.
    pub consumer_projection: ResidencyConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ResidencyProofFreshness,
    /// Degraded-state labels currently applied (empty when fully qualified).
    pub degraded_labels: Vec<ResidencyDegradedReason>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl KeyStorageResidencyContinuitySurfacePacket {
    /// Builds a key/storage/residency/continuity surface packet from stable-lane input.
    pub fn new(input: KeyStorageResidencyContinuitySurfacePacketInput) -> Self {
        Self {
            record_kind: KEY_STORAGE_RESIDENCY_RECORD_KIND.to_owned(),
            schema_version: KEY_STORAGE_RESIDENCY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            projected_surfaces: input.projected_surfaces,
            section_qualifications: input.section_qualifications,
            key_custody_selections: input.key_custody_selections,
            storage_selections: input.storage_selections,
            residency_cues: input.residency_cues,
            continuity_rows: input.continuity_rows,
            scope_contract: input.scope_contract,
            provability_contract: input.provability_contract,
            stale_state_honesty: input.stale_state_honesty,
            continuity_contract: input.continuity_contract,
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

    /// Narrows sections, narrows selection options to their local fallback, and
    /// downgrades freshness, residency pins, and encryption claims from a
    /// per-observation signal, recording the reasons in [`Self::degraded_labels`].
    ///
    /// A degraded managed service, stale proof, or narrowed upstream matrix lane
    /// narrows every section one step, and a degraded managed service additionally
    /// forces every live or cached item to stale and labels it and marks every
    /// non-local continuity capability degraded. When the key-management service is
    /// unavailable, every non-local key-custody option narrows to require admin
    /// approval. When the storage provider is unavailable, every non-local storage
    /// option narrows likewise. When residency is unverified, every verified residency
    /// pin downgrades to [`ResidencyPinState::PinnedUnverified`] and is labeled. When
    /// encryption is unverified, every verified encryption claim downgrades to
    /// [`EncryptionPosture::ClaimedUnverified`] and is labeled. Missing admin
    /// continuity narrows the key-custody, residency-cue, and continuity sections. An
    /// inactive host session downgrades the resolution of every host-dependent desktop
    /// handoff. The local-only key and local-first storage options always remain, and
    /// local work is always preserved. Degraded state is labeled, never hidden.
    pub fn apply_residency_continuity_degradation(
        &mut self,
        observation: &ResidencyContinuityObservation,
    ) {
        let mut labels: BTreeSet<ResidencyDegradedReason> =
            self.degraded_labels.iter().copied().collect();

        let section_adverse = !observation.managed_service_available
            || !observation.proof_fresh
            || observation.upstream_matrix_narrowed;

        if !observation.managed_service_available {
            labels.insert(ResidencyDegradedReason::ManagedServiceDegraded);
            if self.force_all_freshness_stale() {
                labels.insert(ResidencyDegradedReason::FreshnessDowngradedToStale);
            }
            for row in &mut self.continuity_rows {
                if row.continuity_posture.affected_by_managed_outage() {
                    row.degraded = true;
                }
            }
        }
        if !observation.proof_fresh {
            labels.insert(ResidencyDegradedReason::ProofStale);
        }
        if observation.upstream_matrix_narrowed {
            labels.insert(ResidencyDegradedReason::UpstreamMatrixNarrowed);
        }
        if !observation.key_management_available {
            labels.insert(ResidencyDegradedReason::KeyManagementUnavailable);
            let mut narrowed = false;
            for item in &mut self.key_custody_selections {
                if item.offered_custody != KeyCustody::LocalOnlyNoKeyEscrow {
                    let next = item.selection_state.narrowed_to_admin_gated();
                    if next != item.selection_state {
                        item.selection_state = next;
                        narrowed = true;
                    }
                }
            }
            if narrowed {
                labels.insert(ResidencyDegradedReason::SelectionNarrowedToLocalFallback);
            }
        }
        if !observation.storage_provider_available {
            labels.insert(ResidencyDegradedReason::StorageProviderUnavailable);
            let mut narrowed = false;
            for item in &mut self.storage_selections {
                if !item.offered_location.is_local_fallback() {
                    let next = item.selection_state.narrowed_to_admin_gated();
                    if next != item.selection_state {
                        item.selection_state = next;
                        narrowed = true;
                    }
                }
            }
            if narrowed {
                labels.insert(ResidencyDegradedReason::SelectionNarrowedToLocalFallback);
            }
        }
        if !observation.residency_verified {
            labels.insert(ResidencyDegradedReason::ResidencyUnverified);
            if self.force_all_residency_unverified() {
                labels.insert(ResidencyDegradedReason::ResidencyClaimDowngraded);
            }
        }
        if !observation.encryption_verified {
            labels.insert(ResidencyDegradedReason::EncryptionUnverified);
            let mut any_downgraded = false;
            for item in &mut self.key_custody_selections {
                if item.encryption_posture.is_verified_claim() {
                    item.encryption_posture = item.encryption_posture.downgraded_to_unverified();
                    item.claim_verified = false;
                    item.proof_label_shown = true;
                    any_downgraded = true;
                }
            }
            if any_downgraded {
                labels.insert(ResidencyDegradedReason::EncryptionClaimDowngraded);
            }
        }
        if !observation.admin_continuity_available {
            labels.insert(ResidencyDegradedReason::AdminContinuityUnavailable);
        }

        for row in &mut self.section_qualifications {
            let adverse = section_adverse
                || (!observation.key_management_available
                    && row.section == KeyStorageResidencySection::KeyCustodySelection)
                || (!observation.encryption_verified
                    && row.section == KeyStorageResidencySection::KeyCustodySelection)
                || (!observation.storage_provider_available
                    && row.section == KeyStorageResidencySection::StorageSelection)
                || (!observation.residency_verified
                    && matches!(
                        row.section,
                        KeyStorageResidencySection::StorageSelection
                            | KeyStorageResidencySection::ResidencyCue
                    ))
                || (!observation.admin_continuity_available
                    && matches!(
                        row.section,
                        KeyStorageResidencySection::KeyCustodySelection
                            | KeyStorageResidencySection::ResidencyCue
                            | KeyStorageResidencySection::ManagedServiceContinuity
                    ));
            if adverse {
                row.qualification = row.qualification.narrowed_one_step();
                row.rollout_stage = row.rollout_stage.narrowed_one_step();
            }
        }

        if !observation.host_session_active {
            labels.insert(ResidencyDegradedReason::HostSessionInactive);
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
                labels.insert(ResidencyDegradedReason::HandoffTargetUnresolved);
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

    /// Downgrades every verified residency pin (in cues and storage options) to
    /// unverified and labels it. Returns true when at least one pin was downgraded.
    fn force_all_residency_unverified(&mut self) -> bool {
        let mut downgraded = false;
        for item in &mut self.residency_cues {
            if item.pin_state.is_verified_claim() {
                item.pin_state = item.pin_state.downgraded_to_unverified();
                item.claim_verified = false;
                item.proof_label_shown = true;
                downgraded = true;
            }
        }
        for item in &mut self.storage_selections {
            if item.claim_verified {
                item.claim_verified = false;
                item.proof_label_shown = true;
                downgraded = true;
            }
        }
        downgraded
    }

    /// Mutable access to every item's freshness state and stale-label flag.
    fn freshness_states_mut(
        &mut self,
    ) -> impl Iterator<Item = (&mut CompanionFreshnessState, &mut bool)> {
        self.key_custody_selections
            .iter_mut()
            .map(|item| (&mut item.freshness, &mut item.stale_label_shown))
            .chain(
                self.storage_selections
                    .iter_mut()
                    .map(|item| (&mut item.freshness, &mut item.stale_label_shown)),
            )
            .chain(
                self.residency_cues
                    .iter_mut()
                    .map(|item| (&mut item.freshness, &mut item.stale_label_shown)),
            )
            .chain(
                self.continuity_rows
                    .iter_mut()
                    .map(|item| (&mut item.freshness, &mut item.stale_label_shown)),
            )
    }

    /// Validates the surface invariants.
    pub fn validate(&self) -> Vec<ResidencyViolation> {
        let mut violations = Vec::new();

        if self.record_kind != KEY_STORAGE_RESIDENCY_RECORD_KIND {
            violations.push(ResidencyViolation::WrongRecordKind);
        }
        if self.schema_version != KEY_STORAGE_RESIDENCY_SCHEMA_VERSION {
            violations.push(ResidencyViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ResidencyViolation::MissingIdentity);
        }
        if self.projected_surfaces.is_empty() {
            violations.push(ResidencyViolation::ProjectedSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_section_qualifications(self, &mut violations);
        validate_items(self, &mut violations);
        validate_scope_contract(self, &mut violations);
        validate_provability_contract(self, &mut violations);
        validate_stale_state_honesty(self, &mut violations);
        validate_continuity_contract(self, &mut violations);
        validate_locality(self, &mut violations);
        validate_security_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("residency continuity packet serializes"),
        ) {
            violations.push(ResidencyViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("residency continuity packet serializes")
    }

    /// Sections currently publishable (Stable, Beta, or Preview) and not withheld.
    pub fn publishable_sections(
        &self,
    ) -> impl Iterator<Item = &KeyStorageResidencySectionQualification> {
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

    /// True when a local-only key custody option is offered as a fallback.
    pub fn local_only_key_fallback_offered(&self) -> bool {
        self.key_custody_selections
            .iter()
            .any(|item| item.offered_custody == KeyCustody::LocalOnlyNoKeyEscrow)
    }

    /// True when a local-first storage option is offered as a fallback.
    pub fn local_first_storage_fallback_offered(&self) -> bool {
        self.storage_selections
            .iter()
            .any(|item| item.offered_location.is_local_fallback())
    }

    /// True when a customer-managed-key custody option is represented.
    pub fn customer_managed_key_represented(&self) -> bool {
        self.key_custody_selections
            .iter()
            .any(|item| item.offered_custody == KeyCustody::CustomerManagedKey)
    }

    /// True when every encryption claim is verified or labeled as unverified.
    pub fn encryption_claims_honestly_qualified(&self) -> bool {
        self.key_custody_selections.iter().all(|item| {
            if item.encryption_posture.is_verified_claim() {
                item.claim_verified
            } else if item.encryption_posture == EncryptionPosture::ClaimedUnverified {
                item.proof_label_shown
            } else {
                true
            }
        })
    }

    /// True when every residency pin is verified or labeled as unverified.
    pub fn residency_claims_honestly_qualified(&self) -> bool {
        self.residency_cues.iter().all(|item| {
            if item.pin_state.is_verified_claim() {
                item.claim_verified
            } else if item.pin_state == ResidencyPinState::PinnedUnverified {
                item.proof_label_shown
            } else {
                true
            }
        })
    }

    /// True when every continuity row preserves user-owned local work.
    pub fn local_work_never_stranded(&self) -> bool {
        self.continuity_rows
            .iter()
            .all(|item| item.local_work_preserved)
    }

    /// True when every stale or unknown-freshness item carries a visible label.
    pub fn stale_state_honestly_labeled(&self) -> bool {
        self.key_custody_selections
            .iter()
            .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .storage_selections
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .residency_cues
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .continuity_rows
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
    }

    /// Iterates every desktop handoff across all four sections, in section order.
    pub fn handoffs(&self) -> impl Iterator<Item = &CompanionDesktopHandoff> {
        self.key_custody_selections
            .iter()
            .map(|item| &item.handoff)
            .chain(self.storage_selections.iter().map(|item| &item.handoff))
            .chain(self.residency_cues.iter().map(|item| &item.handoff))
            .chain(self.continuity_rows.iter().map(|item| &item.handoff))
    }

    fn handoffs_mut(&mut self) -> impl Iterator<Item = &mut CompanionDesktopHandoff> {
        self.key_custody_selections
            .iter_mut()
            .map(|item| &mut item.handoff)
            .chain(
                self.storage_selections
                    .iter_mut()
                    .map(|item| &mut item.handoff),
            )
            .chain(self.residency_cues.iter_mut().map(|item| &mut item.handoff))
            .chain(
                self.continuity_rows
                    .iter_mut()
                    .map(|item| &mut item.handoff),
            )
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Customer-Managed-Key and Storage Selection Flows, Region/Residency Cues, and Degraded Managed-Service Continuity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Sections: {} | Key-custody options: {} | Storage options: {} | Residency cues: {} | Continuity rows: {}\n",
            self.section_qualifications.len(),
            self.key_custody_selections.len(),
            self.storage_selections.len(),
            self.residency_cues.len(),
            self.continuity_rows.len(),
        ));
        out.push_str(&format!(
            "- Exact desktop handoff for every item: {}\n",
            yes_no(self.all_handoffs_exact())
        ));
        out.push_str(&format!(
            "- Local-only key fallback offered: {}\n",
            yes_no(self.local_only_key_fallback_offered())
        ));
        out.push_str(&format!(
            "- Local-first storage fallback offered: {}\n",
            yes_no(self.local_first_storage_fallback_offered())
        ));
        out.push_str(&format!(
            "- Encryption claims honestly qualified: {}\n",
            yes_no(self.encryption_claims_honestly_qualified())
        ));
        out.push_str(&format!(
            "- Residency claims honestly qualified: {}\n",
            yes_no(self.residency_claims_honestly_qualified())
        ));
        out.push_str(&format!(
            "- Local work never stranded: {}\n",
            yes_no(self.local_work_never_stranded())
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

        out.push_str("\n## Key custody selection\n\n");
        for item in &self.key_custody_selections {
            out.push_str(&format!(
                "- `{}` [{}/{}/{}] (verified: {}) {} ({}) → `{}` ({})\n",
                item.item_id,
                item.offered_custody.as_str(),
                item.selection_state.as_str(),
                item.encryption_posture.as_str(),
                yes_no(item.claim_verified),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Storage selection\n\n");
        for item in &self.storage_selections {
            out.push_str(&format!(
                "- `{}` [{}/{}] residency `{}` (verified: {}) {} ({}) → `{}` ({})\n",
                item.item_id,
                item.offered_location.as_str(),
                item.selection_state.as_str(),
                item.residency_region_ref,
                yes_no(item.claim_verified),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Residency cues\n\n");
        for item in &self.residency_cues {
            out.push_str(&format!(
                "- `{}` [{}/{}] residency `{}` (verified: {}) {} ({}) → `{}` ({})\n",
                item.item_id,
                item.artifact_scope.as_str(),
                item.pin_state.as_str(),
                item.residency_region_ref,
                yes_no(item.claim_verified),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Managed-service continuity\n\n");
        for item in &self.continuity_rows {
            out.push_str(&format!(
                "- `{}` [{}/{}] local_work_preserved `{}` degraded `{}` {} ({}) → `{}` ({})\n",
                item.item_id,
                item.capability.as_str(),
                item.continuity_posture.as_str(),
                yes_no(item.local_work_preserved),
                yes_no(item.degraded),
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

/// Errors emitted when reading the checked-in surface export.
#[derive(Debug)]
pub enum ResidencyArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ResidencyViolation>),
}

impl fmt::Display for ResidencyArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "key/storage/residency/continuity export parse failed: {error}"
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
                    "key/storage/residency/continuity export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ResidencyArtifactError {}

/// Validation failures emitted by
/// [`KeyStorageResidencyContinuitySurfacePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResidencyViolation {
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
    /// No local-only key custody fallback option is offered.
    LocalKeyFallbackMissing,
    /// No local-first storage fallback option is offered.
    LocalStorageFallbackMissing,
    /// An encryption claim is verified-marked without backing verification.
    EncryptionClaimedButUnverified,
    /// An unverified encryption claim is not labeled.
    EncryptionClaimNotLabeled,
    /// A residency pin is verified-marked without backing verification.
    ResidencyClaimedButUnverified,
    /// An unverified residency pin is not labeled.
    ResidencyClaimNotLabeled,
    /// A row is missing its residency region ref.
    ResidencyRegionMissing,
    /// A continuity row does not preserve local work.
    LocalWorkStranded,
    /// An item is missing identity or a redacted body.
    ItemIncomplete,
    /// A stale or unknown-freshness item is not labeled.
    StaleStateNotLabeled,
    /// An item's desktop handoff is missing its deep-link ref.
    HandoffRefMissing,
    /// The read/write scope contract is not fully satisfied.
    ScopeContractIncomplete,
    /// The provability contract is not fully satisfied.
    ProvabilityContractIncomplete,
    /// The stale-state honesty contract is not fully satisfied.
    StaleStateHonestyIncomplete,
    /// The continuity contract is not fully satisfied.
    ContinuityContractIncomplete,
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

impl ResidencyViolation {
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
            Self::LocalKeyFallbackMissing => "local_key_fallback_missing",
            Self::LocalStorageFallbackMissing => "local_storage_fallback_missing",
            Self::EncryptionClaimedButUnverified => "encryption_claimed_but_unverified",
            Self::EncryptionClaimNotLabeled => "encryption_claim_not_labeled",
            Self::ResidencyClaimedButUnverified => "residency_claimed_but_unverified",
            Self::ResidencyClaimNotLabeled => "residency_claim_not_labeled",
            Self::ResidencyRegionMissing => "residency_region_missing",
            Self::LocalWorkStranded => "local_work_stranded",
            Self::ItemIncomplete => "item_incomplete",
            Self::StaleStateNotLabeled => "stale_state_not_labeled",
            Self::HandoffRefMissing => "handoff_ref_missing",
            Self::ScopeContractIncomplete => "scope_contract_incomplete",
            Self::ProvabilityContractIncomplete => "provability_contract_incomplete",
            Self::StaleStateHonestyIncomplete => "stale_state_honesty_incomplete",
            Self::ContinuityContractIncomplete => "continuity_contract_incomplete",
            Self::LocalityDisclosureIncomplete => "locality_disclosure_incomplete",
            Self::SecurityReviewIncomplete => "security_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable surface export.
///
/// This is the canonical reader: the desktop companion panel, the CLI/headless
/// surface, diagnostics, support-export, or Help/About surface calls it to ingest
/// the packet rather than cloning status text.
///
/// # Errors
///
/// Returns [`ResidencyArtifactError`] when the checked-in support export fails to
/// parse or fails validation.
pub fn current_stable_key_storage_residency_continuity_surface_export(
) -> Result<KeyStorageResidencyContinuitySurfacePacket, ResidencyArtifactError> {
    let packet: KeyStorageResidencyContinuitySurfacePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/companion/m5/add_customer_managed_key_or_storage_selection_flows_region_or_residency_cues_and_degraded_managed_service_cont/support_export.json"
    )))
    .map_err(ResidencyArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ResidencyArtifactError::Validation(violations))
    }
}

/// Canonical source contract refs that every export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        KEY_STORAGE_RESIDENCY_SCHEMA_REF.to_owned(),
        KEY_STORAGE_RESIDENCY_DOC_REF.to_owned(),
        M5_REGION_RESIDENCY_REF.to_owned(),
        M5_OFFBOARDING_CONTRACT_REF.to_owned(),
        M5_MANAGED_SYNC_POLICY_REF.to_owned(),
        M5_COMPANION_MATRIX_SCHEMA_REF.to_owned(),
    ]
}

/// Canonical read/write scope and authority contract with every guarantee met.
pub fn canonical_scope_contract() -> ResidencyScopeContract {
    ResidencyScopeContract {
        key_custody_selection_read_only: true,
        storage_selection_read_only: true,
        residency_cue_read_only: true,
        continuity_read_only: true,
        local_core_authoritative: true,
        selection_applied_by_local_core_not_surface: true,
        no_unbounded_workspace_write: true,
        offboarding_never_strands_local_work: true,
        no_payload_bodies: true,
    }
}

/// Canonical selection and provability contract with every guarantee satisfied.
pub fn canonical_provability_contract() -> ResidencyProvabilityContract {
    ResidencyProvabilityContract {
        key_custody_options_disclosed: true,
        local_only_key_fallback_offered: true,
        local_first_storage_fallback_offered: true,
        residency_region_disclosed: true,
        residency_claim_provable_or_labeled: true,
        encryption_claim_provable_or_labeled: true,
        no_claim_without_evidence: true,
    }
}

/// Canonical stale-state honesty contract with every guarantee satisfied.
pub fn canonical_stale_state_honesty() -> ResidencyStaleStateHonesty {
    ResidencyStaleStateHonesty {
        stale_items_labeled: true,
        unknown_freshness_labeled: true,
        never_show_stale_as_live: true,
        freshness_floor_enforced: true,
    }
}

/// Canonical continuity contract with every guarantee satisfied.
pub fn canonical_continuity_contract() -> ManagedServiceContinuityContract {
    ManagedServiceContinuityContract {
        local_core_continues_when_managed_degrades: true,
        degraded_capability_labeled_not_hidden: true,
        local_work_never_stranded: true,
        provider_and_admin_continuity_disclosed: true,
    }
}

/// Canonical security review block with every invariant satisfied.
pub fn canonical_security_review() -> ResidencySecurityReview {
    ResidencySecurityReview {
        key_custody_selection_read_only: true,
        storage_selection_read_only: true,
        residency_cue_read_only: true,
        continuity_read_only: true,
        local_core_authoritative: true,
        selection_applied_by_local_core_not_surface: true,
        customer_managed_key_supported: true,
        local_only_key_fallback_offered: true,
        encryption_claim_provable_or_labeled: true,
        residency_region_disclosed: true,
        residency_claim_provable_or_labeled: true,
        local_work_never_stranded: true,
        stale_state_labeled_never_hidden: true,
        exact_desktop_handoff_preserved: true,
        no_credential_or_key_bodies_in_export: true,
        no_payload_bodies_in_export: true,
        downgrade_narrows_instead_of_hides: true,
        locality_disclosed: true,
    }
}

/// Canonical consumer projection block with every section projecting truth.
pub fn canonical_consumer_projection() -> ResidencyConsumerProjection {
    ResidencyConsumerProjection {
        desktop_panel_shows_key_custody_selection: true,
        desktop_panel_shows_storage_selection: true,
        desktop_panel_shows_residency_cues: true,
        diagnostics_shows_continuity_posture: true,
        cli_headless_shows_active_selection: true,
        support_export_shows_selection_and_continuity: true,
        help_about_shows_encryption_and_residency_claim: true,
        preview_labs_label_for_unqualified_sections: true,
    }
}

/// Canonical per-section qualification rows, inherited from the frozen matrix.
///
/// The key-custody-selection, storage-selection, and residency-cue sections inherit
/// the residency-encryption lane's Preview/early-access qualification; the
/// managed-service-continuity section inherits the offboarding-continuity lane's
/// Beta/staged-rollout qualification, because local-first continuity is a stronger
/// guarantee than an unverified managed selection claim.
pub fn canonical_section_qualifications() -> Vec<KeyStorageResidencySectionQualification> {
    use KeyStorageResidencySection as Section;
    use M5CompanionDowngradeTrigger as Trigger;
    use M5CompanionQualificationClass as Qual;
    use M5CompanionRollbackPosture as Rollback;
    use M5CompanionRolloutStage as Stage;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        KeyStorageResidencySectionQualification {
            section: Section::KeyCustodySelection,
            qualification: Qual::Preview,
            rollout_stage: Stage::EarlyAccess,
            read_write_scope: scope,
            matrix_lane_ref: Section::KeyCustodySelection
                .matrix_lane()
                .as_str()
                .to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ResidencyOrEncryptionUnverified,
                Trigger::ProviderUnavailable,
                Trigger::AdminContinuityRequired,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::LocalCoreContinuesNoRemoteState,
        },
        KeyStorageResidencySectionQualification {
            section: Section::StorageSelection,
            qualification: Qual::Preview,
            rollout_stage: Stage::EarlyAccess,
            read_write_scope: scope,
            matrix_lane_ref: Section::StorageSelection.matrix_lane().as_str().to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ResidencyOrEncryptionUnverified,
                Trigger::ProviderUnavailable,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::LocalCoreContinuesNoRemoteState,
        },
        KeyStorageResidencySectionQualification {
            section: Section::ResidencyCue,
            qualification: Qual::Preview,
            rollout_stage: Stage::EarlyAccess,
            read_write_scope: scope,
            matrix_lane_ref: Section::ResidencyCue.matrix_lane().as_str().to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ResidencyOrEncryptionUnverified,
                Trigger::AdminContinuityRequired,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::EvidencePreservedNoRevert,
        },
        KeyStorageResidencySectionQualification {
            section: Section::ManagedServiceContinuity,
            qualification: Qual::Beta,
            rollout_stage: Stage::StagedRollout,
            read_write_scope: scope,
            matrix_lane_ref: Section::ManagedServiceContinuity
                .matrix_lane()
                .as_str()
                .to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::AdminContinuityRequired,
                Trigger::OffboardingStrandsLocalWork,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::OffboardingExportPreservesLocalWork,
        },
    ]
}

/// Canonical locality disclosure for the surface.
pub fn canonical_locality_disclosure() -> M5CompanionLocalityDisclosure {
    M5CompanionLocalityDisclosure {
        stays_local:
            "A local-only key custody option and a local-first storage option are always offered and always work; the local core is the authoritative source of truth, every selection flow, residency cue, and continuity row stays inspectable offline, and user-owned local work is never stranded."
                .to_owned(),
        staged:
            "Customer-managed-key custody, customer-managed and provider-managed storage locations, and region pinning roll out per cohort and managed tenant and are visibly labeled until qualified."
                .to_owned(),
        requires_provider_or_admin_continuity:
            "Applying a customer-managed key, a managed storage location, or a region pin requires the key-management service, the managed storage provider, and, for managed tenants, admin continuity; these claims are shown as proven only when verifiable. When the managed service degrades the local fallback keeps the user working and the degraded capability is labeled, never hidden."
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

/// Canonical key-custody selection items.
pub fn canonical_key_custody_selections() -> Vec<KeyCustodySelectionItem> {
    use CompanionFreshnessState as Fresh;
    use EncryptionPosture as Posture;
    use KeyCustody as Custody;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        KeyCustodySelectionItem {
            item_id: "key:0001".to_owned(),
            offered_custody: Custody::CustomerManagedKey,
            selection_state: SelectionState::Active,
            encryption_posture: Posture::EndToEndEncryptedVerified,
            claim_verified: true,
            proof_label_shown: false,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary:
                "Customer-managed key active; managed artifacts end-to-end encrypted, claim verified"
                    .to_owned(),
            evidence_ref: "evidence:key-custody-proof:cmk".to_owned(),
            handoff: desktop_handoff("handoff:key:0001", false),
        },
        KeyCustodySelectionItem {
            item_id: "key:0002".to_owned(),
            offered_custody: Custody::ProviderManagedKey,
            selection_state: SelectionState::Available,
            encryption_posture: Posture::EncryptedAtRestVerified,
            claim_verified: true,
            proof_label_shown: false,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Provider-managed key available; encrypted at rest, claim verified".to_owned(),
            evidence_ref: "evidence:key-custody-proof:provider".to_owned(),
            handoff: desktop_handoff("handoff:key:0002", false),
        },
        KeyCustodySelectionItem {
            item_id: "key:0003".to_owned(),
            offered_custody: Custody::LocalOnlyNoKeyEscrow,
            selection_state: SelectionState::Available,
            encryption_posture: Posture::EndToEndEncryptedVerified,
            claim_verified: true,
            proof_label_shown: false,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary:
                "Local-only key, never escrowed, always available as a fallback; claim verified"
                    .to_owned(),
            evidence_ref: "evidence:key-custody-proof:local".to_owned(),
            handoff: desktop_handoff("handoff:key:0003", false),
        },
    ]
}

/// Canonical storage selection items.
pub fn canonical_storage_selections() -> Vec<StorageSelectionItem> {
    use CompanionFreshnessState as Fresh;
    use StorageLocationKind as Location;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        StorageSelectionItem {
            item_id: "storage:0001".to_owned(),
            offered_location: Location::HybridLocalFirst,
            selection_state: SelectionState::Active,
            residency_region_ref: "region:eu-west".to_owned(),
            claim_verified: true,
            proof_label_shown: false,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Local-first storage with an EU-west managed mirror; residency verified"
                .to_owned(),
            record_ref: "storage:option:hybrid".to_owned(),
            handoff: desktop_handoff("handoff:storage:0001", false),
        },
        StorageSelectionItem {
            item_id: "storage:0002".to_owned(),
            offered_location: Location::CustomerManagedBucket,
            selection_state: SelectionState::Available,
            residency_region_ref: "region:customer-bucket".to_owned(),
            claim_verified: true,
            proof_label_shown: false,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Customer-managed storage bucket available; residency verified".to_owned(),
            record_ref: "storage:option:customer-bucket".to_owned(),
            handoff: desktop_handoff("handoff:storage:0002", false),
        },
        StorageSelectionItem {
            item_id: "storage:0003".to_owned(),
            offered_location: Location::ProviderManagedRegion,
            selection_state: SelectionState::RequiresAdminApproval,
            residency_region_ref: "region:unverified".to_owned(),
            claim_verified: false,
            proof_label_shown: true,
            freshness: Fresh::Unknown,
            read_write_scope: scope,
            stale_label_shown: true,
            summary:
                "Provider-managed region offered but residency not yet verified; labeled, admin-gated"
                    .to_owned(),
            record_ref: "storage:option:provider-region".to_owned(),
            handoff: desktop_handoff("handoff:storage:0003", false),
        },
        StorageSelectionItem {
            item_id: "storage:0004".to_owned(),
            offered_location: Location::LocalOnly,
            selection_state: SelectionState::Available,
            residency_region_ref: "region:local-device".to_owned(),
            claim_verified: true,
            proof_label_shown: false,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Local-only storage always available as a fallback; residency verified"
                .to_owned(),
            record_ref: "storage:option:local".to_owned(),
            handoff: desktop_handoff("handoff:storage:0004", false),
        },
    ]
}

/// Canonical residency cue items.
pub fn canonical_residency_cues() -> Vec<ResidencyCueItem> {
    use CompanionFreshnessState as Fresh;
    use EncryptedArtifactScope as Scope;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        ResidencyCueItem {
            item_id: "residency:0001".to_owned(),
            artifact_scope: Scope::ManagedSnapshotStore,
            residency_region_ref: "region:eu-west".to_owned(),
            pin_state: ResidencyPinState::PinnedVerified,
            claim_verified: true,
            proof_label_shown: false,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Managed snapshot store pinned to EU-west; pin verified".to_owned(),
            evidence_ref: "evidence:residency-pin:eu-west".to_owned(),
            handoff: desktop_handoff("handoff:residency:0001", false),
        },
        ResidencyCueItem {
            item_id: "residency:0002".to_owned(),
            artifact_scope: Scope::SyncTransport,
            residency_region_ref: "region:eu-west".to_owned(),
            pin_state: ResidencyPinState::PinnedVerified,
            claim_verified: true,
            proof_label_shown: false,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Sync transport pinned to EU-west; pin verified".to_owned(),
            evidence_ref: "evidence:residency-pin:eu-west".to_owned(),
            handoff: desktop_handoff("handoff:residency:0002", false),
        },
        ResidencyCueItem {
            item_id: "residency:0003".to_owned(),
            artifact_scope: Scope::ConflictHistory,
            residency_region_ref: "region:unverified".to_owned(),
            pin_state: ResidencyPinState::PinnedUnverified,
            claim_verified: false,
            proof_label_shown: true,
            freshness: Fresh::Unknown,
            read_write_scope: scope,
            stale_label_shown: true,
            summary: "Conflict-history residency claimed but not yet verified; labeled".to_owned(),
            evidence_ref: "evidence:residency-pin:pending".to_owned(),
            handoff: desktop_handoff("handoff:residency:0003", false),
        },
    ]
}

/// Canonical managed-service continuity items.
pub fn canonical_continuity_rows() -> Vec<ManagedServiceContinuityItem> {
    use CompanionFreshnessState as Fresh;
    use ContinuityPosture as Posture;
    use ManagedServiceCapability as Capability;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        ManagedServiceContinuityItem {
            item_id: "continuity:0001".to_owned(),
            capability: Capability::ManagedSync,
            continuity_posture: Posture::DegradedLocalFallback,
            local_work_preserved: true,
            degraded: false,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary:
                "Managed sync degrades to local-first; edits keep flowing to the local core"
                    .to_owned(),
            record_ref: "continuity:managed-sync".to_owned(),
            handoff: desktop_handoff("handoff:continuity:0001", false),
        },
        ManagedServiceContinuityItem {
            item_id: "continuity:0002".to_owned(),
            capability: Capability::KeyManagement,
            continuity_posture: Posture::RequiresProviderContinuity,
            local_work_preserved: true,
            degraded: false,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary:
                "Customer-managed-key rotation requires the key-management provider; local-only key keeps working"
                    .to_owned(),
            record_ref: "continuity:key-management".to_owned(),
            handoff: desktop_handoff("handoff:continuity:0002", false),
        },
        ManagedServiceContinuityItem {
            item_id: "continuity:0003".to_owned(),
            capability: Capability::DeviceApproval,
            continuity_posture: Posture::RequiresAdminContinuity,
            local_work_preserved: true,
            degraded: false,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary:
                "Managed-tenant device approval requires admin continuity; the local core never depends on it"
                    .to_owned(),
            record_ref: "continuity:device-approval".to_owned(),
            handoff: desktop_handoff("handoff:continuity:0003", false),
        },
        ManagedServiceContinuityItem {
            item_id: "continuity:0004".to_owned(),
            capability: Capability::ManagedAuditLog,
            continuity_posture: Posture::LocalCoreContinuesUnaffected,
            local_work_preserved: true,
            degraded: false,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary:
                "Local activity log is fully local and unaffected by managed-service degradation"
                    .to_owned(),
            record_ref: "continuity:audit-log".to_owned(),
            handoff: desktop_handoff("handoff:continuity:0004", false),
        },
    ]
}

/// Builds the canonical surface packet.
///
/// This is the first consumer: it mints the surface the checked-in support export
/// and Markdown summary are generated from, so the artifact never drifts from the
/// typed section, item, scope, provability, continuity, and freshness definitions.
pub fn canonical_key_storage_residency_continuity_surface(
    packet_id: String,
    surface_label: String,
    minted_at: String,
    proof_freshness: ResidencyProofFreshness,
) -> KeyStorageResidencyContinuitySurfacePacket {
    KeyStorageResidencyContinuitySurfacePacket::new(
        KeyStorageResidencyContinuitySurfacePacketInput {
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
            key_custody_selections: canonical_key_custody_selections(),
            storage_selections: canonical_storage_selections(),
            residency_cues: canonical_residency_cues(),
            continuity_rows: canonical_continuity_rows(),
            scope_contract: canonical_scope_contract(),
            provability_contract: canonical_provability_contract(),
            stale_state_honesty: canonical_stale_state_honesty(),
            continuity_contract: canonical_continuity_contract(),
            locality_disclosure: canonical_locality_disclosure(),
            security_review: canonical_security_review(),
            consumer_projection: canonical_consumer_projection(),
            proof_freshness,
            source_contract_refs: canonical_source_contract_refs(),
            redaction_class_token: "metadata_safe_default".to_owned(),
            minted_at,
        },
    )
}

fn validate_source_contracts(
    packet: &KeyStorageResidencyContinuitySurfacePacket,
    violations: &mut Vec<ResidencyViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        KEY_STORAGE_RESIDENCY_SCHEMA_REF,
        KEY_STORAGE_RESIDENCY_DOC_REF,
        M5_REGION_RESIDENCY_REF,
        M5_OFFBOARDING_CONTRACT_REF,
        M5_MANAGED_SYNC_POLICY_REF,
        M5_COMPANION_MATRIX_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ResidencyViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_section_qualifications(
    packet: &KeyStorageResidencyContinuitySurfacePacket,
    violations: &mut Vec<ResidencyViolation>,
) {
    let present: BTreeSet<KeyStorageResidencySection> = packet
        .section_qualifications
        .iter()
        .map(|row| row.section)
        .collect();
    for required in KeyStorageResidencySection::ALL {
        if !present.contains(&required) {
            violations.push(ResidencyViolation::RequiredSectionMissing);
            return;
        }
    }

    for row in &packet.section_qualifications {
        if row.matrix_lane_ref != row.section.matrix_lane().as_str() {
            violations.push(ResidencyViolation::SectionLaneMismatch);
        }
        if row.read_write_scope != row.section.bounded_scope() {
            violations.push(ResidencyViolation::SectionScopeMismatch);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(ResidencyViolation::SectionRowIncomplete);
        }
    }
}

fn validate_items(
    packet: &KeyStorageResidencyContinuitySurfacePacket,
    violations: &mut Vec<ResidencyViolation>,
) {
    if packet.key_custody_selections.is_empty()
        || packet.storage_selections.is_empty()
        || packet.residency_cues.is_empty()
        || packet.continuity_rows.is_empty()
    {
        violations.push(ResidencyViolation::SectionContentMissing);
    }

    if !packet.local_only_key_fallback_offered() {
        violations.push(ResidencyViolation::LocalKeyFallbackMissing);
    }
    if !packet.local_first_storage_fallback_offered() {
        violations.push(ResidencyViolation::LocalStorageFallbackMissing);
    }

    for item in &packet.key_custody_selections {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(ResidencyViolation::ReadOnlyScopeViolated);
        }
        if item.encryption_posture.is_verified_claim() && !item.claim_verified {
            violations.push(ResidencyViolation::EncryptionClaimedButUnverified);
        }
        if item.encryption_posture == EncryptionPosture::ClaimedUnverified
            && !item.proof_label_shown
        {
            violations.push(ResidencyViolation::EncryptionClaimNotLabeled);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.evidence_ref.trim().is_empty()
        {
            violations.push(ResidencyViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.storage_selections {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(ResidencyViolation::ReadOnlyScopeViolated);
        }
        if item.residency_region_ref.trim().is_empty() {
            violations.push(ResidencyViolation::ResidencyRegionMissing);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.record_ref.trim().is_empty()
        {
            violations.push(ResidencyViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.residency_cues {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(ResidencyViolation::ReadOnlyScopeViolated);
        }
        if item.pin_state.is_verified_claim() && !item.claim_verified {
            violations.push(ResidencyViolation::ResidencyClaimedButUnverified);
        }
        if item.pin_state == ResidencyPinState::PinnedUnverified && !item.proof_label_shown {
            violations.push(ResidencyViolation::ResidencyClaimNotLabeled);
        }
        if item.residency_region_ref.trim().is_empty() {
            violations.push(ResidencyViolation::ResidencyRegionMissing);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.evidence_ref.trim().is_empty()
        {
            violations.push(ResidencyViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.continuity_rows {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(ResidencyViolation::ReadOnlyScopeViolated);
        }
        if !item.local_work_preserved {
            violations.push(ResidencyViolation::LocalWorkStranded);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.record_ref.trim().is_empty()
        {
            violations.push(ResidencyViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }
}

fn validate_freshness_label(
    freshness: CompanionFreshnessState,
    stale_label_shown: bool,
    violations: &mut Vec<ResidencyViolation>,
) {
    if freshness.requires_label() && !stale_label_shown {
        violations.push(ResidencyViolation::StaleStateNotLabeled);
    }
}

fn validate_handoff(handoff: &CompanionDesktopHandoff, violations: &mut Vec<ResidencyViolation>) {
    if handoff.deep_link_ref.trim().is_empty() {
        violations.push(ResidencyViolation::HandoffRefMissing);
    }
}

fn validate_scope_contract(
    packet: &KeyStorageResidencyContinuitySurfacePacket,
    violations: &mut Vec<ResidencyViolation>,
) {
    let contract = &packet.scope_contract;
    for ok in [
        contract.key_custody_selection_read_only,
        contract.storage_selection_read_only,
        contract.residency_cue_read_only,
        contract.continuity_read_only,
        contract.local_core_authoritative,
        contract.selection_applied_by_local_core_not_surface,
        contract.no_unbounded_workspace_write,
        contract.offboarding_never_strands_local_work,
        contract.no_payload_bodies,
    ] {
        if !ok {
            violations.push(ResidencyViolation::ScopeContractIncomplete);
            return;
        }
    }
}

fn validate_provability_contract(
    packet: &KeyStorageResidencyContinuitySurfacePacket,
    violations: &mut Vec<ResidencyViolation>,
) {
    let contract = &packet.provability_contract;
    for ok in [
        contract.key_custody_options_disclosed,
        contract.local_only_key_fallback_offered,
        contract.local_first_storage_fallback_offered,
        contract.residency_region_disclosed,
        contract.residency_claim_provable_or_labeled,
        contract.encryption_claim_provable_or_labeled,
        contract.no_claim_without_evidence,
    ] {
        if !ok {
            violations.push(ResidencyViolation::ProvabilityContractIncomplete);
            return;
        }
    }
}

fn validate_stale_state_honesty(
    packet: &KeyStorageResidencyContinuitySurfacePacket,
    violations: &mut Vec<ResidencyViolation>,
) {
    let honesty = &packet.stale_state_honesty;
    for ok in [
        honesty.stale_items_labeled,
        honesty.unknown_freshness_labeled,
        honesty.never_show_stale_as_live,
        honesty.freshness_floor_enforced,
    ] {
        if !ok {
            violations.push(ResidencyViolation::StaleStateHonestyIncomplete);
            return;
        }
    }
}

fn validate_continuity_contract(
    packet: &KeyStorageResidencyContinuitySurfacePacket,
    violations: &mut Vec<ResidencyViolation>,
) {
    let contract = &packet.continuity_contract;
    for ok in [
        contract.local_core_continues_when_managed_degrades,
        contract.degraded_capability_labeled_not_hidden,
        contract.local_work_never_stranded,
        contract.provider_and_admin_continuity_disclosed,
    ] {
        if !ok {
            violations.push(ResidencyViolation::ContinuityContractIncomplete);
            return;
        }
    }
}

fn validate_locality(
    packet: &KeyStorageResidencyContinuitySurfacePacket,
    violations: &mut Vec<ResidencyViolation>,
) {
    let locality = &packet.locality_disclosure;
    if locality.stays_local.trim().is_empty()
        || locality.staged.trim().is_empty()
        || locality
            .requires_provider_or_admin_continuity
            .trim()
            .is_empty()
    {
        violations.push(ResidencyViolation::LocalityDisclosureIncomplete);
    }
}

fn validate_security_review(
    packet: &KeyStorageResidencyContinuitySurfacePacket,
    violations: &mut Vec<ResidencyViolation>,
) {
    let review = &packet.security_review;
    for ok in [
        review.key_custody_selection_read_only,
        review.storage_selection_read_only,
        review.residency_cue_read_only,
        review.continuity_read_only,
        review.local_core_authoritative,
        review.selection_applied_by_local_core_not_surface,
        review.customer_managed_key_supported,
        review.local_only_key_fallback_offered,
        review.encryption_claim_provable_or_labeled,
        review.residency_region_disclosed,
        review.residency_claim_provable_or_labeled,
        review.local_work_never_stranded,
        review.stale_state_labeled_never_hidden,
        review.exact_desktop_handoff_preserved,
        review.no_credential_or_key_bodies_in_export,
        review.no_payload_bodies_in_export,
        review.downgrade_narrows_instead_of_hides,
        review.locality_disclosed,
    ] {
        if !ok {
            violations.push(ResidencyViolation::SecurityReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &KeyStorageResidencyContinuitySurfacePacket,
    violations: &mut Vec<ResidencyViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.desktop_panel_shows_key_custody_selection,
        projection.desktop_panel_shows_storage_selection,
        projection.desktop_panel_shows_residency_cues,
        projection.diagnostics_shows_continuity_posture,
        projection.cli_headless_shows_active_selection,
        projection.support_export_shows_selection_and_continuity,
        projection.help_about_shows_encryption_and_residency_claim,
        projection.preview_labs_label_for_unqualified_sections,
    ] {
        if !ok {
            violations.push(ResidencyViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &KeyStorageResidencyContinuitySurfacePacket,
    violations: &mut Vec<ResidencyViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(ResidencyViolation::ProofFreshnessIncomplete);
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
