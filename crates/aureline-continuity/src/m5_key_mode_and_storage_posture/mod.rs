//! Key-mode and storage-posture inspectors for claimed managed, self-hosted,
//! and sovereign surfaces.
//!
//! This module is the in-product surfacing lane that turns key and trust-root
//! posture from a hidden deployment detail into explicit, exportable continuity
//! state. It sits on top of the frozen continuity-claim matrix
//! ([`crate::m5_locality_tenant_keymode_and_drill_matrix`]) and reuses the
//! matrix [`KeyModeClass`] vocabulary so there is exactly one key-mode
//! vocabulary across the product. For each claimed row that protects durable
//! state it produces two things a person can read directly in the product and
//! in support evidence:
//!
//! 1. A [`KeyModeDescriptor`] — plain-language key mode, trust-root posture
//!    (OS store, vendor-managed, customer-managed, or offline trust root), the
//!    runtime key-availability state, the local keystore store-lock state, the
//!    freshness of the key/trust evidence, and the typed degraded state when a
//!    key fails.
//! 2. A [`StoragePostureDescriptor`] — plain-language encryption-at-rest posture
//!    naming the specific key mode that protects durable storage, so "encrypted"
//!    is never treated as sufficient product truth on its own.
//!
//! The same descriptors are projected onto every claimed surface (desktop,
//! CLI/headless inspect, service-health, support-center export, About/Help, and
//! docs/public-truth) through a [`KeyPostureSurfaceProjection`], so the exact
//! key and storage vocabulary stays byte-identical everywhere instead of
//! drifting per surface.
//!
//! Two guardrails are load-bearing:
//!
//! - A managed-scope row whose key or trust material is unavailable, lost, or
//!   mismatched **fails closed**: only the protected managed lane narrows and the
//!   claim is withdrawn, while local-core continuity is preserved and the failure
//!   is recorded as a typed degraded state rather than a generic network error.
//! - A self-hosted or sovereign row may not lean on vendor-managed keys or a
//!   vendor-managed trust root; doing so narrows the claim rather than quietly
//!   keeping a managed banner green.
//!
//! The packet is metadata-only. It carries closed-vocabulary tokens, export-safe
//! plain-language labels, and opaque refs. Raw KMS handles, raw trust roots, raw
//! key bytes, and any secret material never cross this boundary.

use serde::{Deserialize, Serialize};

use crate::m5_locality_tenant_keymode_and_drill_matrix::{
    seeded_continuity_claim_matrix_input, ContinuityClaimQualificationClass, ContinuityClaimRow,
    ContinuityLaneClass, ContinuityProfileClass, KeyModeClass,
};

#[cfg(test)]
mod tests;

/// Schema version carried on every record in this module.
pub const KEY_POSTURE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const KEY_POSTURE_SHARED_CONTRACT_REF: &str = "continuity:m5_key_mode_and_storage_posture:v1";

/// Record-kind tag for [`KeyModeStoragePosturePage`] payloads.
pub const KEY_POSTURE_PAGE_RECORD_KIND: &str = "key_mode_storage_posture_page_record";

/// Record-kind tag for [`KeyModeStoragePostureSummary`] payloads.
pub const KEY_POSTURE_SUMMARY_RECORD_KIND: &str = "key_mode_storage_posture_summary_record";

/// Record-kind tag for [`KeyModeDescriptor`] payloads.
pub const KEY_MODE_DESCRIPTOR_RECORD_KIND: &str = "key_mode_descriptor_record";

/// Record-kind tag for [`StoragePostureDescriptor`] payloads.
pub const STORAGE_POSTURE_DESCRIPTOR_RECORD_KIND: &str = "storage_posture_descriptor_record";

/// Record-kind tag for [`KeyPostureSurfaceProjection`] payloads.
pub const KEY_POSTURE_SURFACE_PROJECTION_RECORD_KIND: &str =
    "key_posture_surface_projection_record";

/// Record-kind tag for [`KeyPostureRowOutcome`] payloads.
pub const KEY_POSTURE_ROW_OUTCOME_RECORD_KIND: &str = "key_posture_row_outcome_record";

/// Record-kind tag for [`KeyPostureDefect`] payloads.
pub const KEY_POSTURE_DEFECT_RECORD_KIND: &str = "key_posture_defect_record";

/// Record-kind tag for [`KeyModeStoragePostureSupportExport`] payloads.
pub const KEY_POSTURE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "key_mode_storage_posture_support_export_record";

/// Repo-relative path of the canonical reviewer doc for this lane.
pub const KEY_POSTURE_DOC_REF: &str = "docs/m5/continuity/key-mode-and-storage-posture.md";

/// Repo-relative path of the checked-in artifact for this lane.
pub const KEY_POSTURE_ARTIFACT_REF: &str =
    "artifacts/m5/continuity/key_mode_and_storage_posture.md";

/// Repo-relative path of the canonical JSON schema for this lane.
pub const KEY_POSTURE_SCHEMA_REF: &str = "schemas/continuity/key_mode_descriptor.schema.json";

/// Trust-root posture protecting a claimed row's durable state.
///
/// This is the explicit product-facing answer to "which trust root anchors this
/// row" requested by the key-posture lane: OS store, vendor-managed,
/// customer-managed, or offline trust root.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustRootPostureClass {
    /// Trust anchored in the local OS keystore.
    OsStoreTrustRoot,
    /// A vendor-managed trust root.
    VendorManagedTrustRoot,
    /// A customer-managed trust root in a customer KMS or HSM.
    CustomerManagedTrustRoot,
    /// An offline or air-gapped signed trust root.
    OfflineTrustRoot,
    /// A hybrid trust root mixing customer and vendor anchors.
    HybridTrustRoot,
    /// The trust-root posture is not disclosed; the claim must narrow.
    TrustRootUndisclosed,
    /// A trust root does not apply to this row.
    NotApplicable,
}

impl TrustRootPostureClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsStoreTrustRoot => "os_store_trust_root",
            Self::VendorManagedTrustRoot => "vendor_managed_trust_root",
            Self::CustomerManagedTrustRoot => "customer_managed_trust_root",
            Self::OfflineTrustRoot => "offline_trust_root",
            Self::HybridTrustRoot => "hybrid_trust_root",
            Self::TrustRootUndisclosed => "trust_root_undisclosed",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Plain-language summary of the trust-root posture.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::OsStoreTrustRoot => "OS keystore trust root",
            Self::VendorManagedTrustRoot => "vendor-managed trust root",
            Self::CustomerManagedTrustRoot => "customer-managed trust root",
            Self::OfflineTrustRoot => "offline signed trust root",
            Self::HybridTrustRoot => "hybrid trust root",
            Self::TrustRootUndisclosed => "not disclosed",
            Self::NotApplicable => "not applicable",
        }
    }

    /// True when a managed-scope row has named a real trust root.
    ///
    /// An undisclosed or not-applicable trust root on a managed-scope row is
    /// treated as undeclared and narrows the claim.
    pub const fn is_declared(self) -> bool {
        !matches!(self, Self::TrustRootUndisclosed | Self::NotApplicable)
    }
}

/// Runtime availability of the key or trust material protecting a row.
///
/// The three failure states are the fail-closed triggers for the protected
/// managed lane; they are typed so support and service-health surfaces show a
/// specific key failure instead of a generic network error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyAvailabilityState {
    /// The key mode and trust root are available.
    Available,
    /// A customer-managed key is unavailable (KMS unreachable or access revoked).
    CustomerKeyUnavailable,
    /// The running trust root does not match the declared trust root.
    TrustRootMismatch,
    /// Durable key material is lost and cannot be recovered.
    KeyMaterialLost,
}

impl KeyAvailabilityState {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::CustomerKeyUnavailable => "customer_key_unavailable",
            Self::TrustRootMismatch => "trust_root_mismatch",
            Self::KeyMaterialLost => "key_material_lost",
        }
    }

    /// Plain-language summary of the availability state.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::CustomerKeyUnavailable => "customer-managed key unavailable",
            Self::TrustRootMismatch => "trust root mismatch",
            Self::KeyMaterialLost => "key material lost",
        }
    }

    /// True when this state is a hard failure that fails the managed lane closed.
    pub const fn is_hard_failure(self) -> bool {
        matches!(
            self,
            Self::CustomerKeyUnavailable | Self::TrustRootMismatch | Self::KeyMaterialLost
        )
    }

    /// The narrow reason this availability state contributes, if any.
    const fn narrow_reason(self) -> Option<KeyPostureNarrowReasonClass> {
        match self {
            Self::Available => None,
            Self::CustomerKeyUnavailable => {
                Some(KeyPostureNarrowReasonClass::CustomerKeyUnavailable)
            }
            Self::TrustRootMismatch => Some(KeyPostureNarrowReasonClass::TrustRootMismatch),
            Self::KeyMaterialLost => Some(KeyPostureNarrowReasonClass::KeyMaterialLost),
        }
    }
}

/// Lock state of the local keystore or secret store backing a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StoreLockState {
    /// The store is unlocked and keys are accessible.
    Unlocked,
    /// The store is locked and keys are temporarily inaccessible.
    Locked,
    /// A lockable store does not apply to this row.
    NotApplicable,
}

impl StoreLockState {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unlocked => "unlocked",
            Self::Locked => "locked",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Plain-language summary of the store-lock state.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::Unlocked => "unlocked",
            Self::Locked => "locked",
            Self::NotApplicable => "not applicable",
        }
    }
}

/// Encryption-at-rest posture for a row's durable storage.
///
/// The opaque and undisclosed variants exist so a claim of "encrypted" without a
/// named key mode cannot pass as stable product truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageEncryptionClass {
    /// Encrypted at rest on the local device.
    DeviceLocalEncrypted,
    /// Encrypted at rest with vendor-managed keys.
    VendorKeyEncrypted,
    /// Encrypted at rest with customer-managed keys.
    CustomerKeyEncrypted,
    /// Sealed with an offline trust root.
    OfflineSealedEncrypted,
    /// Encrypted, but the protecting key mode is not named; insufficient truth.
    EncryptedKeyModeOpaque,
    /// The encryption posture is not disclosed; the claim must narrow.
    EncryptionUndisclosed,
    /// Encryption-at-rest does not apply to this row.
    NotApplicable,
}

impl StorageEncryptionClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeviceLocalEncrypted => "device_local_encrypted",
            Self::VendorKeyEncrypted => "vendor_key_encrypted",
            Self::CustomerKeyEncrypted => "customer_key_encrypted",
            Self::OfflineSealedEncrypted => "offline_sealed_encrypted",
            Self::EncryptedKeyModeOpaque => "encrypted_key_mode_opaque",
            Self::EncryptionUndisclosed => "encryption_undisclosed",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Plain-language summary of the encryption posture.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::DeviceLocalEncrypted => "encrypted on this device",
            Self::VendorKeyEncrypted => "encrypted with vendor-managed keys",
            Self::CustomerKeyEncrypted => "encrypted with customer-managed keys",
            Self::OfflineSealedEncrypted => "sealed with an offline trust root",
            Self::EncryptedKeyModeOpaque => "encrypted, key mode not named",
            Self::EncryptionUndisclosed => "not disclosed",
            Self::NotApplicable => "not applicable",
        }
    }

    /// True when the posture is disclosed at all.
    pub const fn is_disclosed(self) -> bool {
        !matches!(self, Self::EncryptionUndisclosed)
    }

    /// True when the posture names the specific key mode protecting storage.
    pub const fn key_mode_visible(self) -> bool {
        !matches!(
            self,
            Self::EncryptedKeyModeOpaque | Self::EncryptionUndisclosed
        )
    }
}

/// Freshness state of a row's key and trust-root evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyEvidenceStateClass {
    /// Evidence is current.
    Current,
    /// Evidence is stale but within an approved grace window.
    StaleWithinGrace,
    /// Evidence is stale enough that a fresh recheck is required.
    StaleNeedsRecheck,
    /// No key/trust evidence is present.
    Missing,
}

impl KeyEvidenceStateClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::StaleWithinGrace => "stale_within_grace",
            Self::StaleNeedsRecheck => "stale_needs_recheck",
            Self::Missing => "missing",
        }
    }

    /// Plain-language summary of the evidence state.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::StaleWithinGrace => "stale within grace",
            Self::StaleNeedsRecheck => "stale, needs recheck",
            Self::Missing => "missing",
        }
    }

    /// True when the evidence is fresh enough to leave the claim stable.
    pub const fn is_acceptable(self) -> bool {
        matches!(self, Self::Current | Self::StaleWithinGrace)
    }
}

/// Typed degraded state a row is in because of its key or store posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradedStateClass {
    /// No degradation; key and store posture are healthy.
    NoneHealthy,
    /// The managed lane failed closed; local-safe work is preserved.
    ManagedLaneFailClosed,
    /// The local store is locked; local-safe work is preserved.
    StoreLockedDegraded,
}

impl DegradedStateClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneHealthy => "none_healthy",
            Self::ManagedLaneFailClosed => "managed_lane_fail_closed",
            Self::StoreLockedDegraded => "store_locked_degraded",
        }
    }

    /// Plain-language summary of the degraded state.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::NoneHealthy => "healthy",
            Self::ManagedLaneFailClosed => "managed lane failed closed; local-safe work preserved",
            Self::StoreLockedDegraded => "local store locked; local-safe work preserved",
        }
    }
}

/// Surface a key-mode and storage-posture descriptor is projected onto.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyPostureSurfaceClass {
    /// The desktop product UI.
    Desktop,
    /// The CLI / headless inspect surface.
    CliHeadless,
    /// The service-health surface.
    ServiceHealth,
    /// A support-center export packet.
    SupportCenter,
    /// The About / Help surfaces.
    AboutHelp,
    /// Docs and public-truth pages.
    DocsPublicTruth,
}

impl KeyPostureSurfaceClass {
    /// Every surface in canonical projection order.
    pub const ALL: [KeyPostureSurfaceClass; 6] = [
        Self::Desktop,
        Self::CliHeadless,
        Self::ServiceHealth,
        Self::SupportCenter,
        Self::AboutHelp,
        Self::DocsPublicTruth,
    ];

    /// Surfaces a local-core row must still reach (support-center is optional).
    pub const LOCAL_CORE: [KeyPostureSurfaceClass; 5] = [
        Self::Desktop,
        Self::CliHeadless,
        Self::ServiceHealth,
        Self::AboutHelp,
        Self::DocsPublicTruth,
    ];

    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::CliHeadless => "cli_headless",
            Self::ServiceHealth => "service_health",
            Self::SupportCenter => "support_center",
            Self::AboutHelp => "about_help",
            Self::DocsPublicTruth => "docs_public_truth",
        }
    }
}

/// Typed reason a key-mode/storage-posture claim narrowed below stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyPostureNarrowReasonClass {
    /// No narrowing is active.
    NotNarrowed,
    /// A managed-scope row does not name an explicit key mode.
    KeyModeUndisclosed,
    /// A managed-scope row does not declare its trust-root posture.
    TrustRootPostureUndisclosed,
    /// Storage claims "encrypted" without naming the protecting key mode.
    EncryptionPostureOpaque,
    /// The storage encryption posture is not disclosed.
    EncryptionPostureUndisclosed,
    /// A customer-managed key is unavailable; the managed lane fails closed.
    CustomerKeyUnavailable,
    /// The running trust root mismatches the declared one; the lane fails closed.
    TrustRootMismatch,
    /// Durable key material is lost; the managed lane fails closed.
    KeyMaterialLost,
    /// The local store is locked on a managed-scope row.
    StoreLockedOnManagedLane,
    /// The key/trust evidence is stale and a fresh recheck is required.
    KeyEvidenceStale,
    /// A managed-scope row has no key/trust posture evidence.
    KeyPostureEvidenceMissing,
    /// The claimed profile is inconsistent with its key mode or trust root.
    ProfileKeyModeMismatch,
    /// A row is not projected onto every required surface.
    SurfaceReuseIncomplete,
    /// A surface renders different key/storage vocabulary than the descriptor.
    KeyStorageVocabularyDrift,
    /// A local-only row claims a managed key/trust scope without a dependency.
    LocalOnlyKeyOverclaimed,
}

impl KeyPostureNarrowReasonClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::KeyModeUndisclosed => "key_mode_undisclosed",
            Self::TrustRootPostureUndisclosed => "trust_root_posture_undisclosed",
            Self::EncryptionPostureOpaque => "encryption_posture_opaque",
            Self::EncryptionPostureUndisclosed => "encryption_posture_undisclosed",
            Self::CustomerKeyUnavailable => "customer_key_unavailable",
            Self::TrustRootMismatch => "trust_root_mismatch",
            Self::KeyMaterialLost => "key_material_lost",
            Self::StoreLockedOnManagedLane => "store_locked_on_managed_lane",
            Self::KeyEvidenceStale => "key_evidence_stale",
            Self::KeyPostureEvidenceMissing => "key_posture_evidence_missing",
            Self::ProfileKeyModeMismatch => "profile_key_mode_mismatch",
            Self::SurfaceReuseIncomplete => "surface_reuse_incomplete",
            Self::KeyStorageVocabularyDrift => "key_storage_vocabulary_drift",
            Self::LocalOnlyKeyOverclaimed => "local_only_key_overclaimed",
        }
    }

    /// True when this reason withdraws the claim immediately (fails closed).
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::CustomerKeyUnavailable | Self::TrustRootMismatch | Self::KeyMaterialLost
        )
    }

    /// True when this reason holds the claim at preview.
    pub const fn is_preview_reason(self) -> bool {
        matches!(
            self,
            Self::KeyModeUndisclosed
                | Self::TrustRootPostureUndisclosed
                | Self::StoreLockedOnManagedLane
                | Self::KeyEvidenceStale
                | Self::ProfileKeyModeMismatch
                | Self::KeyStorageVocabularyDrift
                | Self::LocalOnlyKeyOverclaimed
        )
    }
}

/// Derives a qualification from the key-posture narrow reasons present.
fn qualification_from_reasons<'a>(
    reasons: impl IntoIterator<Item = &'a KeyPostureNarrowReasonClass>,
) -> ContinuityClaimQualificationClass {
    let mut saw_any = false;
    let mut saw_preview = false;
    for reason in reasons {
        saw_any = true;
        if reason.is_withdrawal_reason() {
            return ContinuityClaimQualificationClass::Withdrawn;
        }
        if reason.is_preview_reason() {
            saw_preview = true;
        }
    }
    if saw_preview {
        ContinuityClaimQualificationClass::Preview
    } else if saw_any {
        ContinuityClaimQualificationClass::Beta
    } else {
        ContinuityClaimQualificationClass::Stable
    }
}

/// One claimed surface decorated with the facts needed to build its key-mode and
/// storage-posture descriptors.
///
/// The reused profile, lane, and key-mode fields are sourced from the frozen
/// continuity-claim matrix so the key-mode vocabulary stays identical; the
/// trust-root, availability, store-lock, encryption, and evidence fields are
/// this lane's additive truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyModeStorageEntry {
    /// Opaque row identifier shared with the continuity-claim matrix.
    pub row_id: String,
    /// Reviewable label naming the claimed surface.
    pub surface_label: String,
    /// Claimed deployment profile.
    pub profile_class: ContinuityProfileClass,
    /// Stable token for [`Self::profile_class`].
    pub profile_class_token: String,
    /// True when this row depends on a claimed managed or self-hosted lane.
    pub has_claimed_managed_dependency: bool,
    /// Continuity lane this row belongs to.
    pub continuity_lane: ContinuityLaneClass,
    /// Stable token for [`Self::continuity_lane`].
    pub continuity_lane_token: String,
    /// True when this row actually protects durable state with keys or trust roots.
    pub protects_durable_state: bool,
    /// Key-mode posture protecting durable state.
    pub key_mode: KeyModeClass,
    /// Stable token for [`Self::key_mode`].
    pub key_mode_token: String,
    /// Trust-root posture anchoring the row.
    pub trust_root_posture: TrustRootPostureClass,
    /// Stable token for [`Self::trust_root_posture`].
    pub trust_root_posture_token: String,
    /// Runtime availability of the key or trust material.
    pub key_availability: KeyAvailabilityState,
    /// Stable token for [`Self::key_availability`].
    pub key_availability_token: String,
    /// Lock state of the local keystore or secret store.
    pub store_lock: StoreLockState,
    /// Stable token for [`Self::store_lock`].
    pub store_lock_token: String,
    /// Encryption-at-rest posture for durable storage.
    pub storage_encryption: StorageEncryptionClass,
    /// Stable token for [`Self::storage_encryption`].
    pub storage_encryption_token: String,
    /// Freshness of the key and trust-root evidence.
    pub key_evidence_state: KeyEvidenceStateClass,
    /// Stable token for [`Self::key_evidence_state`].
    pub key_evidence_state_token: String,
    /// Opaque ref to the key/trust posture evidence; never a secret.
    pub key_posture_evidence_ref: String,
    /// Surfaces this row is projected onto.
    pub projected_surfaces: Vec<KeyPostureSurfaceClass>,
}

impl KeyModeStorageEntry {
    /// True when this row sits inside managed continuity scope.
    ///
    /// Managed, self-hosted, and sovereign profiles are always in scope, as is
    /// any row on the managed continuity lane or carrying a claimed managed
    /// dependency. A pure local-only row with no managed dependency is out of
    /// scope and is not held to managed-lane key-posture requirements.
    pub fn in_managed_scope(&self) -> bool {
        self.profile_class != ContinuityProfileClass::LocalOnly
            || self.continuity_lane == ContinuityLaneClass::ManagedLane
            || self.has_claimed_managed_dependency
    }

    /// Surfaces this row is required to reach.
    pub fn required_surfaces(&self) -> &'static [KeyPostureSurfaceClass] {
        if self.in_managed_scope() {
            &KeyPostureSurfaceClass::ALL
        } else {
            &KeyPostureSurfaceClass::LOCAL_CORE
        }
    }

    /// True when the managed lane must fail closed for this row.
    ///
    /// A managed-scope row whose key or trust material is unavailable, lost, or
    /// mismatched fails closed; local-core work never enters this rule.
    pub fn fail_closed_on_managed_lane(&self) -> bool {
        self.in_managed_scope() && self.key_availability.is_hard_failure()
    }

    /// The typed degraded state this row is in.
    pub fn degraded_state(&self) -> DegradedStateClass {
        if self.fail_closed_on_managed_lane() {
            DegradedStateClass::ManagedLaneFailClosed
        } else if self.in_managed_scope() && self.store_lock == StoreLockState::Locked {
            DegradedStateClass::StoreLockedDegraded
        } else {
            DegradedStateClass::NoneHealthy
        }
    }

    /// True when local-core continuity survives this row's key posture.
    ///
    /// Local-safe work always continues: a managed key failure narrows only the
    /// protected managed lane, never the local autosave and version-control core.
    pub const fn local_core_preserved(&self) -> bool {
        true
    }

    /// True when the profile is inconsistent with its key mode or trust root.
    ///
    /// A self-hosted or sovereign row may not lean on vendor-managed keys or a
    /// vendor-managed trust root.
    pub fn profile_key_mode_mismatch(&self) -> bool {
        self.profile_class.is_self_governed()
            && (self.key_mode == KeyModeClass::VendorManagedKeys
                || self.trust_root_posture == TrustRootPostureClass::VendorManagedTrustRoot)
    }

    /// Builds an entry from a frozen continuity-claim row plus this lane's facts.
    #[allow(clippy::too_many_arguments)]
    pub fn from_claim_row(
        row: &ContinuityClaimRow,
        protects_durable_state: bool,
        trust_root_posture: TrustRootPostureClass,
        key_availability: KeyAvailabilityState,
        store_lock: StoreLockState,
        storage_encryption: StorageEncryptionClass,
        key_evidence_state: KeyEvidenceStateClass,
        key_posture_evidence_ref: impl Into<String>,
        projected_surfaces: Vec<KeyPostureSurfaceClass>,
    ) -> Self {
        Self {
            row_id: row.row_id.clone(),
            surface_label: row.surface_label.clone(),
            profile_class: row.profile_class,
            profile_class_token: row.profile_class.as_str().to_owned(),
            has_claimed_managed_dependency: row.has_claimed_managed_dependency,
            continuity_lane: row.continuity_lane,
            continuity_lane_token: row.continuity_lane.as_str().to_owned(),
            protects_durable_state,
            key_mode: row.key_mode,
            key_mode_token: row.key_mode.as_str().to_owned(),
            trust_root_posture,
            trust_root_posture_token: trust_root_posture.as_str().to_owned(),
            key_availability,
            key_availability_token: key_availability.as_str().to_owned(),
            store_lock,
            store_lock_token: store_lock.as_str().to_owned(),
            storage_encryption,
            storage_encryption_token: storage_encryption.as_str().to_owned(),
            key_evidence_state,
            key_evidence_state_token: key_evidence_state.as_str().to_owned(),
            key_posture_evidence_ref: key_posture_evidence_ref.into(),
            projected_surfaces,
        }
    }
}

/// Plain-language key mode, trust root, availability, and degraded state for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyModeDescriptor {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque descriptor identifier.
    pub descriptor_id: String,
    /// Row this descriptor describes.
    pub row_id: String,
    /// Reviewable label naming the claimed surface.
    pub surface_label: String,
    /// Stable token for the claimed profile.
    pub profile_class_token: String,
    /// Plain-language profile summary.
    pub profile_plain: String,
    /// Stable token for the continuity lane.
    pub continuity_lane_token: String,
    /// Plain-language continuity-lane summary.
    pub continuity_lane_plain: String,
    /// Stable token for the key mode.
    pub key_mode_token: String,
    /// Plain-language key mode.
    pub key_mode_plain: String,
    /// Stable token for the trust-root posture.
    pub trust_root_posture_token: String,
    /// Plain-language trust-root posture.
    pub trust_root_posture_plain: String,
    /// Stable token for the key-availability state.
    pub key_availability_token: String,
    /// Plain-language key-availability state.
    pub key_availability_plain: String,
    /// Stable token for the store-lock state.
    pub store_lock_token: String,
    /// Plain-language store-lock state.
    pub store_lock_plain: String,
    /// Stable token for the key-evidence state.
    pub key_evidence_state_token: String,
    /// Plain-language key-evidence state.
    pub key_evidence_state_plain: String,
    /// Stable token for the typed degraded state.
    pub degraded_state_token: String,
    /// Plain-language typed degraded state.
    pub degraded_state_plain: String,
    /// Canonical one-line key summary reused by every surface projection.
    pub key_summary_line: String,
    /// True when the managed lane fails closed for this row.
    pub fail_closed_on_managed_lane: bool,
    /// True when local-core continuity survives this row's key posture.
    pub local_core_preserved: bool,
    /// True when this row actually protects durable state with keys or trust roots.
    pub protects_durable_state: bool,
}

impl KeyModeDescriptor {
    /// Builds a key-mode descriptor from a decorated entry.
    pub fn from_entry(entry: &KeyModeStorageEntry) -> Self {
        Self {
            record_kind: KEY_MODE_DESCRIPTOR_RECORD_KIND.to_owned(),
            schema_version: KEY_POSTURE_SCHEMA_VERSION,
            shared_contract_ref: KEY_POSTURE_SHARED_CONTRACT_REF.to_owned(),
            descriptor_id: format!("continuity:key-mode-descriptor:{}", entry.row_id),
            row_id: entry.row_id.clone(),
            surface_label: entry.surface_label.clone(),
            profile_class_token: entry.profile_class_token.clone(),
            profile_plain: profile_plain(entry.profile_class).to_owned(),
            continuity_lane_token: entry.continuity_lane_token.clone(),
            continuity_lane_plain: lane_plain(entry.continuity_lane).to_owned(),
            key_mode_token: entry.key_mode_token.clone(),
            key_mode_plain: key_mode_plain(entry.key_mode).to_owned(),
            trust_root_posture_token: entry.trust_root_posture_token.clone(),
            trust_root_posture_plain: entry.trust_root_posture.plain().to_owned(),
            key_availability_token: entry.key_availability_token.clone(),
            key_availability_plain: entry.key_availability.plain().to_owned(),
            store_lock_token: entry.store_lock_token.clone(),
            store_lock_plain: entry.store_lock.plain().to_owned(),
            key_evidence_state_token: entry.key_evidence_state_token.clone(),
            key_evidence_state_plain: entry.key_evidence_state.plain().to_owned(),
            degraded_state_token: entry.degraded_state().as_str().to_owned(),
            degraded_state_plain: entry.degraded_state().plain().to_owned(),
            key_summary_line: key_summary_line(entry),
            fail_closed_on_managed_lane: entry.fail_closed_on_managed_lane(),
            local_core_preserved: entry.local_core_preserved(),
            protects_durable_state: entry.protects_durable_state,
        }
    }
}

/// Plain-language encryption-at-rest posture naming the protecting key mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoragePostureDescriptor {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque descriptor identifier.
    pub descriptor_id: String,
    /// Row this descriptor describes.
    pub row_id: String,
    /// Reviewable label naming the claimed surface.
    pub surface_label: String,
    /// Stable token for the storage encryption posture.
    pub storage_encryption_token: String,
    /// Plain-language storage encryption posture.
    pub storage_encryption_plain: String,
    /// Stable token for the key mode protecting storage.
    pub key_mode_token: String,
    /// Plain-language key mode protecting storage.
    pub key_mode_plain: String,
    /// Stable token for the trust-root posture.
    pub trust_root_posture_token: String,
    /// Plain-language trust-root posture.
    pub trust_root_posture_plain: String,
    /// Canonical one-line storage summary reused by every surface projection.
    pub storage_summary_line: String,
    /// True when the posture names the specific key mode protecting storage.
    pub key_mode_visible: bool,
}

impl StoragePostureDescriptor {
    /// Builds a storage-posture descriptor from a decorated entry.
    pub fn from_entry(entry: &KeyModeStorageEntry) -> Self {
        Self {
            record_kind: STORAGE_POSTURE_DESCRIPTOR_RECORD_KIND.to_owned(),
            schema_version: KEY_POSTURE_SCHEMA_VERSION,
            shared_contract_ref: KEY_POSTURE_SHARED_CONTRACT_REF.to_owned(),
            descriptor_id: format!("continuity:storage-posture-descriptor:{}", entry.row_id),
            row_id: entry.row_id.clone(),
            surface_label: entry.surface_label.clone(),
            storage_encryption_token: entry.storage_encryption_token.clone(),
            storage_encryption_plain: entry.storage_encryption.plain().to_owned(),
            key_mode_token: entry.key_mode_token.clone(),
            key_mode_plain: key_mode_plain(entry.key_mode).to_owned(),
            trust_root_posture_token: entry.trust_root_posture_token.clone(),
            trust_root_posture_plain: entry.trust_root_posture.plain().to_owned(),
            storage_summary_line: storage_summary_line(entry),
            key_mode_visible: entry.storage_encryption.key_mode_visible(),
        }
    }
}

/// One surface rendering of a row's key-mode and storage-posture descriptors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyPostureSurfaceProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Surface this projection renders on.
    pub surface: KeyPostureSurfaceClass,
    /// Stable token for [`Self::surface`].
    pub surface_token: String,
    /// Row this projection describes.
    pub row_id: String,
    /// Key-mode descriptor id rendered on this surface.
    pub key_descriptor_id: String,
    /// Storage-posture descriptor id rendered on this surface.
    pub storage_descriptor_id: String,
    /// Key summary line rendered on this surface.
    pub key_summary_line: String,
    /// Storage summary line rendered on this surface.
    pub storage_summary_line: String,
}

/// Per-row verdict joining a row to its computed qualification and reasons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyPostureRowOutcome {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Row this outcome describes.
    pub row_id: String,
    /// Stable token for the claimed profile.
    pub profile_class_token: String,
    /// True when the row is in managed continuity scope.
    pub in_managed_scope: bool,
    /// Computed qualification token for the row.
    pub qualification_token: String,
    /// True when the row narrowed below stable.
    pub narrowed: bool,
    /// True when the row's claim is withheld entirely.
    pub claim_withheld: bool,
    /// True when the managed lane failed closed for this row.
    pub fail_closed: bool,
    /// True when local-core continuity survives this row's key posture.
    pub local_core_preserved: bool,
    /// Stable token for the typed degraded state.
    pub degraded_state_token: String,
    /// Stable narrow-reason tokens that applied to the row.
    pub narrow_reason_tokens: Vec<String>,
}

/// Typed defect emitted by the key-posture audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyPostureDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque defect identifier.
    pub defect_id: String,
    /// Typed narrow reason.
    pub narrow_reason: KeyPostureNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Opaque source row id that triggered the defect.
    pub source: String,
    /// Export-safe explanation of the defect.
    pub note: String,
}

impl KeyPostureDefect {
    fn new(
        narrow_reason: KeyPostureNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source = source.into();
        Self {
            record_kind: KEY_POSTURE_DEFECT_RECORD_KIND.to_owned(),
            schema_version: KEY_POSTURE_SCHEMA_VERSION,
            shared_contract_ref: KEY_POSTURE_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "continuity:defect:key-posture:{}:{}",
                narrow_reason.as_str(),
                source
            ),
            narrow_reason,
            narrow_reason_token: narrow_reason.as_str().to_owned(),
            source,
            note: note.into(),
        }
    }
}

/// Aggregate summary for a key-mode/storage-posture page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyModeStoragePostureSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Overall qualification for the page.
    pub overall_qualification_token: String,
    /// Number of claimed entries.
    pub entry_count: usize,
    /// Number of entries in managed continuity scope.
    pub managed_scope_entry_count: usize,
    /// Number of entries on the local-core continuity lane.
    pub local_core_entry_count: usize,
    /// Number of entries protected by a customer-controlled key.
    pub customer_controlled_key_entry_count: usize,
    /// Number of entries anchored by an offline trust root.
    pub offline_trust_root_entry_count: usize,
    /// Number of entries whose managed lane failed closed.
    pub fail_closed_entry_count: usize,
    /// Number of entries whose local store is locked.
    pub store_locked_entry_count: usize,
    /// Number of entries that narrowed below stable.
    pub narrowed_entry_count: usize,
    /// Number of entries whose claim is withheld.
    pub withdrawn_entry_count: usize,
    /// Number of surface projections emitted.
    pub surface_projection_count: usize,
    /// True when every surface renders the same key/storage vocabulary.
    pub vocabulary_consistent: bool,
    /// True when every entry preserves local-core continuity.
    pub all_local_core_preserved: bool,
    /// True when no raw key material is carried anywhere in the packet.
    pub raw_key_material_excluded: bool,
    /// Number of defects recorded for the page.
    pub defect_count: usize,
}

/// Full auditable input for the key-mode/storage-posture page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyModeStoragePostureInput {
    /// Reviewable label for the page.
    pub input_label: String,
    /// Claimed entries.
    pub entries: Vec<KeyModeStorageEntry>,
}

/// Canonical proof packet for key-mode and storage-posture inspectors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyModeStoragePosturePage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page identifier.
    pub page_id: String,
    /// Reviewable page label.
    pub page_label: String,
    /// UTC timestamp when the packet was generated.
    pub generated_at: String,
    /// Aggregate summary derived from the embedded input and defects.
    pub summary: KeyModeStoragePostureSummary,
    /// Typed defects for the packet.
    pub defects: Vec<KeyPostureDefect>,
    /// Key-mode descriptors, one per entry.
    pub key_descriptors: Vec<KeyModeDescriptor>,
    /// Storage-posture descriptors, one per entry.
    pub storage_descriptors: Vec<StoragePostureDescriptor>,
    /// Per-surface projections proving identical vocabulary across surfaces.
    pub surface_projections: Vec<KeyPostureSurfaceProjection>,
    /// Per-row verdicts joining each row to its computed qualification.
    pub row_outcomes: Vec<KeyPostureRowOutcome>,
    /// The audited input embedded as evidence.
    pub input: KeyModeStoragePostureInput,
}

impl KeyModeStoragePosturePage {
    /// Builds a key-mode/storage-posture page from the supplied input.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        input: KeyModeStoragePostureInput,
    ) -> Self {
        let key_descriptors: Vec<KeyModeDescriptor> = input
            .entries
            .iter()
            .map(KeyModeDescriptor::from_entry)
            .collect();
        let storage_descriptors: Vec<StoragePostureDescriptor> = input
            .entries
            .iter()
            .map(StoragePostureDescriptor::from_entry)
            .collect();
        let surface_projections = build_surface_projections(&input.entries);
        let defects = audit(&input, &surface_projections);
        let row_outcomes = build_row_outcomes(&input, &defects);
        let summary = build_summary(&input, &surface_projections, &row_outcomes, &defects);
        Self {
            record_kind: KEY_POSTURE_PAGE_RECORD_KIND.to_owned(),
            schema_version: KEY_POSTURE_SCHEMA_VERSION,
            shared_contract_ref: KEY_POSTURE_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            defects,
            key_descriptors,
            storage_descriptors,
            surface_projections,
            row_outcomes,
            input,
        }
    }

    /// True when the page qualifies stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == ContinuityClaimQualificationClass::Stable.as_str()
    }

    /// True when every surface renders identical key/storage vocabulary.
    pub fn surfaces_share_vocabulary(&self) -> bool {
        self.summary.vocabulary_consistent
    }

    /// Returns the key-mode descriptor for a row id, if present.
    pub fn key_descriptor(&self, row_id: &str) -> Option<&KeyModeDescriptor> {
        self.key_descriptors.iter().find(|d| d.row_id == row_id)
    }

    /// Returns the storage-posture descriptor for a row id, if present.
    pub fn storage_descriptor(&self, row_id: &str) -> Option<&StoragePostureDescriptor> {
        self.storage_descriptors.iter().find(|d| d.row_id == row_id)
    }

    /// Returns the computed outcome for a row id, if present.
    pub fn row_outcome(&self, row_id: &str) -> Option<&KeyPostureRowOutcome> {
        self.row_outcomes.iter().find(|o| o.row_id == row_id)
    }

    /// Returns the key-mode descriptors whose managed lane failed closed.
    pub fn fail_closed_descriptors(&self) -> Vec<&KeyModeDescriptor> {
        self.key_descriptors
            .iter()
            .filter(|d| d.fail_closed_on_managed_lane)
            .collect()
    }
}

/// Support-export wrapper for the key-mode/storage-posture page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyModeStoragePostureSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export identifier.
    pub export_id: String,
    /// UTC timestamp when the export was produced.
    pub generated_at: String,
    /// The key-mode/storage-posture page embedded as evidence.
    pub page: KeyModeStoragePosturePage,
    /// Typed narrow reasons present in the embedded packet.
    pub narrow_reasons_present: Vec<KeyPostureNarrowReasonClass>,
    /// True when raw key material is excluded from this export.
    pub raw_key_material_excluded: bool,
}

impl KeyModeStoragePostureSupportExport {
    /// Wraps a key-mode/storage-posture page inside a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: KeyModeStoragePosturePage,
    ) -> Self {
        let mut reasons: Vec<KeyPostureNarrowReasonClass> = Vec::new();
        for defect in &page.defects {
            if !reasons.contains(&defect.narrow_reason) {
                reasons.push(defect.narrow_reason);
            }
        }
        reasons.sort();
        Self {
            record_kind: KEY_POSTURE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: KEY_POSTURE_SCHEMA_VERSION,
            shared_contract_ref: KEY_POSTURE_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            raw_key_material_excluded: true,
        }
    }
}

/// Re-runs the key-posture audit over a page, including its stored projections.
///
/// Unlike [`KeyModeStoragePosturePage::new`], this validates the page's stored
/// surface projections against freshly derived canonical lines, so a tampered
/// projection (one that renders different vocabulary than its descriptor) is
/// caught on re-validation.
pub fn audit_key_mode_storage_posture_page(
    page: &KeyModeStoragePosturePage,
) -> Vec<KeyPostureDefect> {
    audit(&page.input, &page.surface_projections)
}

/// Validates a key-posture page and returns `Ok(())` when the audit is clean.
pub fn validate_key_mode_storage_posture_page(
    page: &KeyModeStoragePosturePage,
) -> Result<(), Vec<KeyPostureDefect>> {
    let defects = audit_key_mode_storage_posture_page(page);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Returns the seeded stable key-mode/storage-posture page.
pub fn seeded_key_mode_storage_posture_page() -> KeyModeStoragePosturePage {
    KeyModeStoragePosturePage::new(
        "continuity:key-posture:seeded",
        "Key-mode and storage-posture inspectors",
        "2026-06-01T00:00:00Z",
        seeded_key_mode_storage_posture_input(),
    )
}

/// Returns the seeded input used by the canonical key-posture page.
///
/// The entries reuse the frozen continuity-claim matrix rows as their profile,
/// lane, and key-mode source, decorated with this lane's trust-root,
/// availability, store-lock, encryption, and evidence truth. The customer
/// self-hosted row exercises a real customer-managed key lane and the sovereign
/// row exercises a real offline-trust-root lane.
pub fn seeded_key_mode_storage_posture_input() -> KeyModeStoragePostureInput {
    let claim_rows = seeded_continuity_claim_matrix_input().claim_rows;
    let entries = claim_rows.iter().map(decorate_seed_row).collect();
    KeyModeStoragePostureInput {
        input_label: "Claimed managed, self-hosted, and sovereign key and storage posture"
            .to_owned(),
        entries,
    }
}

fn decorate_seed_row(row: &ContinuityClaimRow) -> KeyModeStorageEntry {
    let all = KeyPostureSurfaceClass::ALL.to_vec();
    let local_core = KeyPostureSurfaceClass::LOCAL_CORE.to_vec();
    match row.row_id.as_str() {
        "continuity-row:managed-cloud-sync" => KeyModeStorageEntry::from_claim_row(
            row,
            true,
            TrustRootPostureClass::VendorManagedTrustRoot,
            KeyAvailabilityState::Available,
            StoreLockState::NotApplicable,
            StorageEncryptionClass::VendorKeyEncrypted,
            KeyEvidenceStateClass::Current,
            "key-posture-evidence:managed-cloud:2026-06-01",
            all,
        ),
        "continuity-row:managed-relay-failover" => KeyModeStorageEntry::from_claim_row(
            row,
            true,
            TrustRootPostureClass::VendorManagedTrustRoot,
            KeyAvailabilityState::Available,
            StoreLockState::NotApplicable,
            StorageEncryptionClass::VendorKeyEncrypted,
            KeyEvidenceStateClass::Current,
            "key-posture-evidence:managed-relay:2026-05-20",
            all,
        ),
        "continuity-row:self-hosted-restore" => KeyModeStorageEntry::from_claim_row(
            row,
            true,
            TrustRootPostureClass::CustomerManagedTrustRoot,
            KeyAvailabilityState::Available,
            StoreLockState::NotApplicable,
            StorageEncryptionClass::CustomerKeyEncrypted,
            KeyEvidenceStateClass::Current,
            "key-posture-evidence:self-hosted:2026-05-18",
            all,
        ),
        "continuity-row:sovereign-airgap-snapshot" => KeyModeStorageEntry::from_claim_row(
            row,
            true,
            TrustRootPostureClass::OfflineTrustRoot,
            KeyAvailabilityState::Available,
            StoreLockState::NotApplicable,
            StorageEncryptionClass::OfflineSealedEncrypted,
            KeyEvidenceStateClass::StaleWithinGrace,
            "key-posture-evidence:sovereign:2026-05-15",
            all,
        ),
        _ => KeyModeStorageEntry::from_claim_row(
            row,
            true,
            TrustRootPostureClass::OsStoreTrustRoot,
            KeyAvailabilityState::Available,
            StoreLockState::Unlocked,
            StorageEncryptionClass::DeviceLocalEncrypted,
            KeyEvidenceStateClass::Current,
            "key-posture-evidence:local-core:autosave",
            local_core,
        ),
    }
}

fn audit(
    input: &KeyModeStoragePostureInput,
    projections: &[KeyPostureSurfaceProjection],
) -> Vec<KeyPostureDefect> {
    let mut defects = Vec::new();
    for entry in &input.entries {
        audit_entry(entry, &mut defects);
    }
    audit_vocabulary(input, projections, &mut defects);
    defects
}

fn audit_entry(entry: &KeyModeStorageEntry, defects: &mut Vec<KeyPostureDefect>) {
    // Encryption posture disclosure applies to every claimed row. Guardrail:
    // "encrypted" without a named key mode is not sufficient product truth.
    if !entry.storage_encryption.is_disclosed() {
        defects.push(KeyPostureDefect::new(
            KeyPostureNarrowReasonClass::EncryptionPostureUndisclosed,
            entry.row_id.clone(),
            "every claimed row must disclose its encryption-at-rest posture",
        ));
    } else if !entry.storage_encryption.key_mode_visible() {
        defects.push(KeyPostureDefect::new(
            KeyPostureNarrowReasonClass::EncryptionPostureOpaque,
            entry.row_id.clone(),
            "an 'encrypted' claim must name the specific key mode protecting durable storage",
        ));
    }

    // Surface projection completeness.
    let missing: Vec<&KeyPostureSurfaceClass> = entry
        .required_surfaces()
        .iter()
        .filter(|surface| !entry.projected_surfaces.contains(surface))
        .collect();
    if !missing.is_empty() {
        defects.push(KeyPostureDefect::new(
            KeyPostureNarrowReasonClass::SurfaceReuseIncomplete,
            entry.row_id.clone(),
            "this row's key-mode and storage-posture descriptors must reach every required surface",
        ));
    }

    // Guardrail: a local-only row may not claim managed key/trust scope unless it
    // actually carries a claimed managed dependency.
    if entry.profile_class == ContinuityProfileClass::LocalOnly
        && !entry.has_claimed_managed_dependency
        && entry.continuity_lane == ContinuityLaneClass::ManagedLane
    {
        defects.push(KeyPostureDefect::new(
            KeyPostureNarrowReasonClass::LocalOnlyKeyOverclaimed,
            entry.row_id.clone(),
            "a local-only row may not claim a managed key or trust-root scope without a claimed managed dependency",
        ));
    }

    if !entry.in_managed_scope() {
        // Local-core rows owe disclosure and projection only; the managed-lane
        // key, trust-root, and availability rules below never apply, so they stay
        // accurately labeled rather than being narrowed against managed
        // expectations.
        return;
    }

    // Key mode and trust root must be explicit on the protected managed lane.
    if entry.key_mode == KeyModeClass::NotApplicable {
        defects.push(KeyPostureDefect::new(
            KeyPostureNarrowReasonClass::KeyModeUndisclosed,
            entry.row_id.clone(),
            "managed, self-hosted, and sovereign rows must name an explicit key mode",
        ));
    }
    if !entry.trust_root_posture.is_declared() {
        defects.push(KeyPostureDefect::new(
            KeyPostureNarrowReasonClass::TrustRootPostureUndisclosed,
            entry.row_id.clone(),
            "managed-scope rows must declare an explicit trust-root posture",
        ));
    }

    // Fail closed: a runtime key or trust failure withdraws only the managed lane.
    if let Some(reason) = entry.key_availability.narrow_reason() {
        defects.push(KeyPostureDefect::new(
            reason,
            entry.row_id.clone(),
            "the key or trust material is unavailable; the managed lane fails closed while local-core continuity is preserved",
        ));
    }

    // Store-lock degraded state on the managed lane.
    if entry.store_lock == StoreLockState::Locked {
        defects.push(KeyPostureDefect::new(
            KeyPostureNarrowReasonClass::StoreLockedOnManagedLane,
            entry.row_id.clone(),
            "the local store is locked; the managed lane is degraded until it is unlocked",
        ));
    }

    // Key/trust evidence freshness, including the contradicted-by-runtime case.
    if entry.key_evidence_state == KeyEvidenceStateClass::Missing
        || entry.key_posture_evidence_ref.is_empty()
    {
        defects.push(KeyPostureDefect::new(
            KeyPostureNarrowReasonClass::KeyPostureEvidenceMissing,
            entry.row_id.clone(),
            "managed-scope rows must reference current key/trust posture evidence",
        ));
    } else if !entry.key_evidence_state.is_acceptable() {
        defects.push(KeyPostureDefect::new(
            KeyPostureNarrowReasonClass::KeyEvidenceStale,
            entry.row_id.clone(),
            "key/trust evidence is stale and a fresh recheck is required before the claim stays stable",
        ));
    }

    // Profile-vs-posture mismatch guardrail.
    if entry.profile_key_mode_mismatch() {
        defects.push(KeyPostureDefect::new(
            KeyPostureNarrowReasonClass::ProfileKeyModeMismatch,
            entry.row_id.clone(),
            "a self-hosted or sovereign row cannot rely on vendor-managed keys or a vendor-managed trust root",
        ));
    }
}

fn audit_vocabulary(
    input: &KeyModeStoragePostureInput,
    projections: &[KeyPostureSurfaceProjection],
    defects: &mut Vec<KeyPostureDefect>,
) {
    for entry in &input.entries {
        let canonical_key = key_summary_line(entry);
        let canonical_storage = storage_summary_line(entry);
        let drifted = projections
            .iter()
            .filter(|projection| projection.row_id == entry.row_id)
            .any(|projection| {
                projection.key_summary_line != canonical_key
                    || projection.storage_summary_line != canonical_storage
            });
        if drifted {
            defects.push(KeyPostureDefect::new(
                KeyPostureNarrowReasonClass::KeyStorageVocabularyDrift,
                entry.row_id.clone(),
                "a surface renders different key or storage vocabulary than the descriptor",
            ));
        }
    }
}

fn build_surface_projections(entries: &[KeyModeStorageEntry]) -> Vec<KeyPostureSurfaceProjection> {
    let mut projections = Vec::new();
    for entry in entries {
        let key_summary_line = key_summary_line(entry);
        let storage_summary_line = storage_summary_line(entry);
        let key_descriptor_id = format!("continuity:key-mode-descriptor:{}", entry.row_id);
        let storage_descriptor_id =
            format!("continuity:storage-posture-descriptor:{}", entry.row_id);
        for surface in KeyPostureSurfaceClass::ALL {
            if !entry.projected_surfaces.contains(&surface) {
                continue;
            }
            projections.push(KeyPostureSurfaceProjection {
                record_kind: KEY_POSTURE_SURFACE_PROJECTION_RECORD_KIND.to_owned(),
                schema_version: KEY_POSTURE_SCHEMA_VERSION,
                shared_contract_ref: KEY_POSTURE_SHARED_CONTRACT_REF.to_owned(),
                surface,
                surface_token: surface.as_str().to_owned(),
                row_id: entry.row_id.clone(),
                key_descriptor_id: key_descriptor_id.clone(),
                storage_descriptor_id: storage_descriptor_id.clone(),
                key_summary_line: key_summary_line.clone(),
                storage_summary_line: storage_summary_line.clone(),
            });
        }
    }
    projections
}

fn build_row_outcomes(
    input: &KeyModeStoragePostureInput,
    defects: &[KeyPostureDefect],
) -> Vec<KeyPostureRowOutcome> {
    input
        .entries
        .iter()
        .map(|entry| {
            let reasons: Vec<KeyPostureNarrowReasonClass> = defects
                .iter()
                .filter(|defect| defect.source == entry.row_id)
                .map(|defect| defect.narrow_reason)
                .collect();
            let qualification = qualification_from_reasons(reasons.iter());
            let mut reason_tokens: Vec<String> = reasons
                .iter()
                .map(|reason| reason.as_str().to_owned())
                .collect();
            reason_tokens.sort();
            reason_tokens.dedup();
            KeyPostureRowOutcome {
                record_kind: KEY_POSTURE_ROW_OUTCOME_RECORD_KIND.to_owned(),
                schema_version: KEY_POSTURE_SCHEMA_VERSION,
                shared_contract_ref: KEY_POSTURE_SHARED_CONTRACT_REF.to_owned(),
                row_id: entry.row_id.clone(),
                profile_class_token: entry.profile_class_token.clone(),
                in_managed_scope: entry.in_managed_scope(),
                qualification_token: qualification.as_str().to_owned(),
                narrowed: qualification != ContinuityClaimQualificationClass::Stable,
                claim_withheld: qualification == ContinuityClaimQualificationClass::Withdrawn,
                fail_closed: entry.fail_closed_on_managed_lane(),
                local_core_preserved: entry.local_core_preserved(),
                degraded_state_token: entry.degraded_state().as_str().to_owned(),
                narrow_reason_tokens: reason_tokens,
            }
        })
        .collect()
}

fn build_summary(
    input: &KeyModeStoragePostureInput,
    projections: &[KeyPostureSurfaceProjection],
    row_outcomes: &[KeyPostureRowOutcome],
    defects: &[KeyPostureDefect],
) -> KeyModeStoragePostureSummary {
    let overall = if defects
        .iter()
        .any(|defect| defect.narrow_reason.is_withdrawal_reason())
    {
        ContinuityClaimQualificationClass::Withdrawn
    } else if defects
        .iter()
        .any(|defect| defect.narrow_reason.is_preview_reason())
    {
        ContinuityClaimQualificationClass::Preview
    } else if defects.is_empty() {
        ContinuityClaimQualificationClass::Stable
    } else {
        ContinuityClaimQualificationClass::Beta
    };

    let vocabulary_consistent = !defects.iter().any(|defect| {
        defect.narrow_reason == KeyPostureNarrowReasonClass::KeyStorageVocabularyDrift
    });

    KeyModeStoragePostureSummary {
        record_kind: KEY_POSTURE_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: KEY_POSTURE_SCHEMA_VERSION,
        shared_contract_ref: KEY_POSTURE_SHARED_CONTRACT_REF.to_owned(),
        overall_qualification_token: overall.as_str().to_owned(),
        entry_count: input.entries.len(),
        managed_scope_entry_count: input
            .entries
            .iter()
            .filter(|entry| entry.in_managed_scope())
            .count(),
        local_core_entry_count: input
            .entries
            .iter()
            .filter(|entry| entry.continuity_lane == ContinuityLaneClass::LocalCore)
            .count(),
        customer_controlled_key_entry_count: input
            .entries
            .iter()
            .filter(|entry| {
                matches!(
                    entry.key_mode,
                    KeyModeClass::CustomerManagedKeys | KeyModeClass::CustomerHeldRoot
                )
            })
            .count(),
        offline_trust_root_entry_count: input
            .entries
            .iter()
            .filter(|entry| entry.trust_root_posture == TrustRootPostureClass::OfflineTrustRoot)
            .count(),
        fail_closed_entry_count: input
            .entries
            .iter()
            .filter(|entry| entry.fail_closed_on_managed_lane())
            .count(),
        store_locked_entry_count: input
            .entries
            .iter()
            .filter(|entry| entry.store_lock == StoreLockState::Locked)
            .count(),
        narrowed_entry_count: row_outcomes
            .iter()
            .filter(|outcome| outcome.narrowed)
            .count(),
        withdrawn_entry_count: row_outcomes
            .iter()
            .filter(|outcome| outcome.claim_withheld)
            .count(),
        surface_projection_count: projections.len(),
        vocabulary_consistent,
        all_local_core_preserved: row_outcomes
            .iter()
            .all(|outcome| outcome.local_core_preserved),
        raw_key_material_excluded: true,
        defect_count: defects.len(),
    }
}

fn key_summary_line(entry: &KeyModeStorageEntry) -> String {
    format!(
        "Key mode {}; trust root {}; availability {}; store {}; evidence {}; degraded state {}.",
        key_mode_plain(entry.key_mode),
        entry.trust_root_posture.plain(),
        entry.key_availability.plain(),
        entry.store_lock.plain(),
        entry.key_evidence_state.plain(),
        entry.degraded_state().plain(),
    )
}

fn storage_summary_line(entry: &KeyModeStorageEntry) -> String {
    format!(
        "Storage {}; key mode {}; trust root {}.",
        entry.storage_encryption.plain(),
        key_mode_plain(entry.key_mode),
        entry.trust_root_posture.plain(),
    )
}

fn profile_plain(class: ContinuityProfileClass) -> &'static str {
    match class {
        ContinuityProfileClass::Managed => "vendor-managed cloud",
        ContinuityProfileClass::SelfHosted => "customer self-hosted",
        ContinuityProfileClass::Sovereign => "sovereign or air-gapped",
        ContinuityProfileClass::LocalOnly => "local desktop only",
    }
}

fn lane_plain(class: ContinuityLaneClass) -> &'static str {
    match class {
        ContinuityLaneClass::LocalCore => "local-core continuity",
        ContinuityLaneClass::ManagedLane => "managed continuity lane",
    }
}

fn key_mode_plain(class: KeyModeClass) -> &'static str {
    match class {
        KeyModeClass::LocalOsKeystore => "local OS keystore",
        KeyModeClass::VendorManagedKeys => "vendor-managed keys",
        KeyModeClass::CustomerManagedKeys => "customer-managed keys",
        KeyModeClass::CustomerHeldRoot => "customer-held root key",
        KeyModeClass::HybridEnvelope => "hybrid key envelope",
        KeyModeClass::NotApplicable => "not applicable",
    }
}
