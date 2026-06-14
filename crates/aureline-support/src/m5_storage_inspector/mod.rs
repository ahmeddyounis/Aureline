//! Storage inspectors and workspace-storage detail for the heavy artifact
//! families the M5 depth lanes add.
//!
//! M5 lands notebook outputs, profiler traces, replay bundles, docs/model/
//! template packs, generated previews, extension downloads, prebuild layers,
//! support artifacts, and review/incident evidence — plus it touches the
//! user-owned recovery state earlier milestones own. Those lanes are not
//! complete until the shell can explain, without manual filesystem inspection,
//! how much disk each class uses per workspace, which consumers dominate, what
//! a rebuild would cost, which bytes are pinned, how sensitive they are, and
//! which are disposable versus correctness-relevant versus durable evidence
//! versus user-owned recovery state.
//!
//! This module is the canonical, inspectable truth model behind those
//! surfaces. It folds the checked-in inspector-card, storage-class breakdown,
//! workspace-storage detail, and rebuild-cost hint fixtures into typed records
//! and validates every record against the cross-record safety contract the
//! source documents freeze. It mints no new storage primitive: the
//! storage-class, authority, rebuild-cost, GC-policy, clear-protection, and
//! pin-source vocabularies re-export verbatim from the established
//! [`crate::storage_inspector`] and [`crate::m5_storage_governance`] models.
//!
//! ## What this owns
//!
//! - The [`StorageInspectorCard`] record — total used bytes, quota/policy
//!   source, per-class breakdown row refs, authority-aware largest consumers,
//!   scan posture, and protected-class visibility for one inspector scope.
//!   Mirrors the boundary schema at [`STORAGE_INSPECTOR_CARD_SCHEMA_REF`].
//! - The [`StorageClassBreakdownRow`] record — one storage class on one scope,
//!   carrying used/reclaimable/protected/pinned bytes, posture, rebuild cost,
//!   the pinned-consumer breakdown, and the per-class largest consumers.
//!   Mirrors [`STORAGE_CLASS_BREAKDOWN_SCHEMA_REF`].
//! - The [`WorkspaceStorageDetailRow`] record — one entry-level disclosure with
//!   size, last-used, the embedded [`RebuildCostHint`], pin state, sensitivity,
//!   freshness/corruption/policy-protection state, and the typed clear/export
//!   actions a surface offers verbatim. Mirrors
//!   [`WORKSPACE_STORAGE_DETAIL_SCHEMA_REF`].
//! - The [`StorageInspectorCorpus`] container — folds every seeded card,
//!   breakdown row, and detail row into one validated bundle, checks the
//!   cross-record safety invariants, and projects a metadata-safe
//!   [`StorageInspectorSupportExport`] the support-bundle pipeline can quote
//!   without leaking raw payloads, raw paths, or raw credential bodies.
//!
//! ## What this does NOT own
//!
//! - Live byte-level scanning, garbage collection, or quota enforcement. Those
//!   belong to the runtime crates; this module is the shared truth model the
//!   inspector, workspace detail, low-disk banner, clear-data review, pin
//!   manager, cleanup-history lane, admin console, and support export project.
//! - The clear-data review sheet, low-disk banner, pin/retention manager, or
//!   cleanup-history lane records. Those are owned by sibling storage lanes;
//!   this module only references them by opaque id.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub use crate::m5_storage_governance::{
    AuthorityClass, ClearCacheProtectionClass, GcPolicyClass, PinSourceClass, RebuildCostClass,
};
pub use crate::storage_inspector::StorageClassId;

#[cfg(test)]
mod tests;

// --------------------------------------------------------------------------
// Frozen schema versions, record kinds, and boundary refs.
// --------------------------------------------------------------------------

/// Frozen schema version shared by the inspector-card record.
pub const STORAGE_INSPECTOR_CARD_SCHEMA_VERSION: u32 = 1;

/// Frozen schema version shared by the storage-class breakdown row.
pub const STORAGE_CLASS_BREAKDOWN_ROW_SCHEMA_VERSION: u32 = 1;

/// Frozen schema version shared by the workspace-storage detail row.
pub const WORKSPACE_STORAGE_DETAIL_ROW_SCHEMA_VERSION: u32 = 1;

/// Frozen schema version shared by the rebuild-cost hint record.
pub const REBUILD_COST_HINT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the inspector-card record.
pub const STORAGE_INSPECTOR_CARD_RECORD_KIND: &str = "storage_inspector_card_record";

/// Stable record-kind tag for the storage-class breakdown row.
pub const STORAGE_CLASS_BREAKDOWN_ROW_RECORD_KIND: &str = "storage_class_breakdown_row_record";

/// Stable record-kind tag for the workspace-storage detail row.
pub const WORKSPACE_STORAGE_DETAIL_ROW_RECORD_KIND: &str = "workspace_storage_detail_row_record";

/// Stable record-kind tag for the rebuild-cost hint record.
pub const REBUILD_COST_HINT_RECORD_KIND: &str = "rebuild_cost_hint_record";

/// Stable record-kind tag for one support-export card row.
pub const STORAGE_INSPECTOR_SUPPORT_EXPORT_CARD_RECORD_KIND: &str =
    "storage_inspector_support_export_card";

/// Stable record-kind tag for the support-export envelope.
pub const STORAGE_INSPECTOR_SUPPORT_EXPORT_RECORD_KIND: &str = "storage_inspector_support_export";

/// Repository-relative path of the inspector-card boundary schema.
pub const STORAGE_INSPECTOR_CARD_SCHEMA_REF: &str =
    "schemas/storage/storage_inspector_card.schema.json";

/// Repository-relative path of the storage-class breakdown boundary schema.
pub const STORAGE_CLASS_BREAKDOWN_SCHEMA_REF: &str =
    "schemas/storage/storage_class_breakdown.schema.json";

/// Repository-relative path of the workspace-storage detail boundary schema.
pub const WORKSPACE_STORAGE_DETAIL_SCHEMA_REF: &str =
    "schemas/storage/workspace_storage_detail.schema.json";

/// Repository-relative path of the rebuild-cost hint boundary schema.
pub const REBUILD_COST_HINT_SCHEMA_REF: &str = "schemas/storage/rebuild_cost_hint.schema.json";

/// Repository-relative path of the inspector / breakdown contract doc.
pub const STORAGE_INSPECTOR_CONTRACT_DOC_REF: &str = "docs/storage/storage_inspector_contract.md";

/// Repository-relative path of the workspace-detail / rebuild-cost contract doc.
pub const WORKSPACE_STORAGE_DETAIL_CONTRACT_DOC_REF: &str =
    "docs/storage/workspace_storage_detail_contract.md";

/// Repository-relative directory holding the inspector-card / breakdown corpus.
pub const STORAGE_INSPECTOR_CASES_DIR: &str = "fixtures/storage/storage_inspector_cases";

/// Repository-relative directory holding the workspace-detail corpus.
pub const WORKSPACE_STORAGE_DETAIL_CASES_DIR: &str =
    "fixtures/storage/workspace_storage_detail_cases";

// --------------------------------------------------------------------------
// Closed vocabularies introduced by the inspector / detail contracts.
//
// storage_class_id re-exports from `storage_inspector`. authority_class,
// rebuild_cost_class, gc_policy_class, clear_cache_protection_class, and
// pin_source_class re-export from `m5_storage_governance`. Everything below is
// the inspector / detail surface vocabulary frozen by the source contracts.
// --------------------------------------------------------------------------

/// Redaction posture carried by every inspectable storage record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    MetadataSafeDefault,
    OperatorOnlyRestricted,
    InternalSupportRestricted,
    SigningEvidenceOnly,
}

impl RedactionClass {
    /// Returns true when the posture is admissible on an export-safe record.
    pub const fn is_export_safe(self) -> bool {
        matches!(
            self,
            Self::MetadataSafeDefault | Self::OperatorOnlyRestricted
        )
    }
}

/// Current storage posture of a class or scope.
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

/// Quota basis the inspector renders for a scope or class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuotaBasisClass {
    PerWorkspaceQuota,
    GlobalDeviceQuota,
    PerClassCeiling,
    PerTenantQuota,
    PolicyBoundEvidenceQuota,
    RetentionPolicyOnly,
    DigestStorePlusClassCeiling,
}

/// Authority that authored the quota or policy bound.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuotaAuthorityClass {
    UserLocalAuthority,
    AdminPolicyAuthority,
    TenantPolicyAuthority,
    DeviceGovernorAuthority,
    NotApplicable,
}

/// Surface a breakdown row declares itself inspectable on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectableSurfaceClass {
    StorageInspector,
    WorkspaceStorageDetail,
    ClearDataReview,
    LowDiskBanner,
    PinManager,
    CleanupHistoryLane,
}

/// Surface a card declares itself a consumer of.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurfaceClass {
    StorageInspector,
    WorkspaceStorageDetail,
    ClearDataReview,
    LowDiskBanner,
    PinManager,
    CleanupHistoryLane,
    AdminStorageConsole,
    SupportPacketExport,
    AboutPanelStorageExcerpt,
    CliTextFormatter,
}

/// Protected-class visibility token a broad-scope card must carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtectedClassVisibility {
    EvidenceSupportCacheVisible,
    UserOwnedRecoveryStateVisible,
}

/// Scope the inspector card or detail row covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorScopeClass {
    DeviceTotal,
    WorkspaceOnly,
    WorksetOnly,
    ProfileOnly,
    TenantOnly,
    SliceOnly,
}

impl InspectorScopeClass {
    /// Broad scopes that must list both protected-class visibility tokens.
    const fn is_broad(self) -> bool {
        matches!(
            self,
            Self::DeviceTotal | Self::WorkspaceOnly | Self::TenantOnly
        )
    }
}

/// Freshness of the most recent storage scan behind a card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScanFreshnessClass {
    ScanFreshWithinWindow,
    ScanWithinExtendedWindow,
    ScanPastExtendedWindow,
    ScanInFlight,
    ScanUnknownNoScanYet,
}

/// Why a scan covered only part of the scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartialScanReasonClass {
    NotPartialFullScanComplete,
    PartialScopeFilterApplied,
    PartialScanInProgress,
    PartialDueToLowDisk,
    PartialDueToQuotaOrThrottle,
}

/// Why a scan ran at reduced privilege.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LowPrivilegeScanReasonClass {
    NotLowPrivilegeFullInspection,
    LowPrivilegeUserOnlyScan,
    LowPrivilegeManagedTenantRestriction,
    LowPrivilegeAdminConsentRequired,
    LowPrivilegeOfflineScopeOnly,
}

/// A cache consumer registered on a card or breakdown row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerClass {
    WorkspaceIndexCorpus,
    WorkspaceHistoryLane,
    WorkspaceRecoveryJournal,
    WorksetSessionRestoreState,
    ExtensionPack,
    DocsPackCorpus,
    ModelPackBlob,
    UpdatePackBlob,
    PolicyBundleBlob,
    PrebuildLayerBlob,
    TemplateOrArchetypePack,
    EvidencePacketBlob,
    SupportExportAssemblyInFlight,
    TerminalRestoreMetadataStore,
    MirrorSnapshotSegment,
    OfflineBundleSegment,
    AggregatedRemainderOther,
}

/// Entry-level class on a workspace-storage detail row. Re-exports
/// [`ConsumerClass`] plus the `interactive_hot_cache_shard` entry value the
/// card aggregates under [`ConsumerClass::WorkspaceIndexCorpus`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryClass {
    WorkspaceIndexCorpus,
    WorkspaceHistoryLane,
    WorkspaceRecoveryJournal,
    WorksetSessionRestoreState,
    ExtensionPack,
    DocsPackCorpus,
    ModelPackBlob,
    UpdatePackBlob,
    PolicyBundleBlob,
    PrebuildLayerBlob,
    TemplateOrArchetypePack,
    EvidencePacketBlob,
    SupportExportAssemblyInFlight,
    TerminalRestoreMetadataStore,
    MirrorSnapshotSegment,
    OfflineBundleSegment,
    InteractiveHotCacheShard,
    AggregatedRemainderOther,
}

/// Origin posture distinguishing local, mirrored, imported, and policy state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorOrImportOriginClass {
    LocalAuthoritative,
    LocalDisposableCache,
    LocalEvidenceCapture,
    MirroredCopy,
    OfflineBundleLocal,
    VendorSignedOfflineLocal,
    CustomerSignedMirrorLocal,
    PolicyProtectedAdminArtifact,
    NotApplicable,
}

impl MirrorOrImportOriginClass {
    /// True for mirror / offline-bundle origins that must cite a deployment
    /// summary mirror row and forbid a generic always-allowed clear.
    pub const fn is_mirror_or_offline(self) -> bool {
        matches!(
            self,
            Self::MirroredCopy
                | Self::OfflineBundleLocal
                | Self::VendorSignedOfflineLocal
                | Self::CustomerSignedMirrorLocal
        )
    }
}

/// Per-consumer pin summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PinSummaryClass {
    NoPinInScope,
    PinnedOneSource,
    PinnedMultipleSources,
    NotApplicable,
}

/// Sensitivity tier of an entry. `t3` is admissible only on evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SensitivityClass {
    T0MetadataOnly,
    T1LowRiskDerived,
    T2CodeBearingBounded,
    T3SecretAdjacentNotReusableCache,
}

/// Detail-authority posture the workspace-detail surface renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetailAuthorityPostureClass {
    DisposableDerivedState,
    CorrectnessRelevantDerivedState,
    ImportedDurableArtifact,
    PolicyHeldEvidenceState,
    AuthoritativeUserOwnedState,
}

/// Freshness state rendered verbatim instead of being hidden under last-used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessStateClass {
    FreshWithinWindow,
    StaleWithinExtendedWindow,
    StalePastExtendedWindow,
    UnknownNoScanYet,
    NotApplicableNoFreshnessSignal,
}

impl FreshnessStateClass {
    /// Freshness states that carry a null `last_used_at`.
    const fn has_no_timestamp(self) -> bool {
        matches!(
            self,
            Self::UnknownNoScanYet | Self::NotApplicableNoFreshnessSignal
        )
    }
}

/// Corruption state of an entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorruptionStateClass {
    NotCorrupt,
    SuspectedCorruptPendingRevalidation,
    ConfirmedCorruptQuarantined,
    ConfirmedCorruptPendingRebuildOrReplace,
    UnknownLowPrivilegeScan,
}

impl CorruptionStateClass {
    /// Confirmed-corrupt states that must link a corruption-rescue path.
    const fn is_confirmed_corrupt(self) -> bool {
        matches!(
            self,
            Self::ConfirmedCorruptQuarantined | Self::ConfirmedCorruptPendingRebuildOrReplace
        )
    }
}

/// Policy-protection state of an entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyProtectionStateClass {
    NotPolicyProtected,
    ProtectedAdminPolicyPin,
    ProtectedTenantPolicyPin,
    ProtectedRetentionWindow,
    ProtectedEvidenceCaseOrReview,
    ProtectedUserOwnedAuthoritative,
}

/// Pin state of an entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PinStateClass {
    UnpinnedNoSources,
    PinnedUserOnly,
    PinnedAdminOrTenantPolicyOnly,
    PinnedReleaseOrOfflineBundleOnly,
    PinnedEvidenceCaseOrReviewOnly,
    PinnedRetentionWindowOnly,
    PinnedMultipleClasses,
    NotApplicableAuthoritativeState,
}

impl PinStateClass {
    /// Pin states that require a non-empty pin-source breakdown.
    const fn requires_breakdown(self) -> bool {
        matches!(
            self,
            Self::PinnedUserOnly
                | Self::PinnedAdminOrTenantPolicyOnly
                | Self::PinnedReleaseOrOfflineBundleOnly
                | Self::PinnedEvidenceCaseOrReviewOnly
                | Self::PinnedRetentionWindowOnly
                | Self::PinnedMultipleClasses
        )
    }
}

/// Clear action a workspace-detail row offers verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClearActionClass {
    ClearAdmissibleGeneric,
    ClearAdmissibleAfterPinRelease,
    ClearRequiresClassSpecificReview,
    ClearRefusedAuthoritativeUserOwned,
    ClearRefusedPolicyHeld,
}

/// Export action a workspace-detail row offers verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportActionClass {
    ExportOfferedMetadataSafe,
    ExportOfferedOperatorOnly,
    ExportRequiredBeforeClear,
    ExportUnsupportedAlreadyLocalOnlyDisposable,
    ExportUnsupportedClass,
}

/// What a rebuild would have to reach if an entry were removed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfflineRebuildRiskClass {
    SafeToRemoveOffline,
    RebuildRequiresNetworkResync,
    RebuildRequiresMirrorOrOfflineBundle,
    RebuildRequiresAdminOrPolicySignedPack,
    NotRebuildableAfterRemoval,
}

/// Startup impact the user sees if an entry is removed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StartupImpactClass {
    NoUserVisibleImpact,
    SlowerFirstOpenUntilWarm,
    SlowerFirstQueryUntilReindexed,
    SlowerFirstBuildUntilPrebuilt,
    FeatureUnavailableUntilRebuilt,
    NotApplicableAuthoritativeState,
}

/// Whether a rebuild preserves the entry's provenance trail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceContinuityClass {
    ProvenancePreservedRebuildFromLocalTruth,
    ProvenancePreservedRebuildFromSignedSource,
    ProvenanceBreaksUntilResignedOrReImported,
    AuthoritativeProvenanceIrreplaceable,
    NotApplicableDisposableNoProvenance,
}

/// One input a rebuild needs to consume.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RebuildInputClass {
    LocalWorkspaceSource,
    LocalHistoryJournal,
    NetworkProviderOrIndex,
    CustomerOperatedMirror,
    VendorPublishedMirror,
    OfflineBundleLocal,
    VendorSignedOfflineBundle,
    CustomerSignedMirror,
    PolicySignedPack,
    NoInputAuthoritativeState,
}

/// Four-class rebuild-safety summary rendered verbatim by every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RebuildSafetySummaryClass {
    CheapToRebuildSafeToRemove,
    ExpensiveToRebuildButSafe,
    ImpossibleToRebuildOffline,
    DangerousToDeleteAuthoritative,
}

/// Non-negative byte ceiling, or the `not_applicable` sentinel when the quota
/// basis carries no enforceable ceiling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QuotaCeilingBytes {
    /// An enforceable non-negative byte ceiling.
    Bytes(u64),
    /// The basis carries no enforceable ceiling.
    NotApplicable,
}

impl Serialize for QuotaCeilingBytes {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Bytes(value) => serializer.serialize_u64(*value),
            Self::NotApplicable => serializer.serialize_str("not_applicable"),
        }
    }
}

impl<'de> Deserialize<'de> for QuotaCeilingBytes {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct QuotaCeilingVisitor;

        impl Visitor<'_> for QuotaCeilingVisitor {
            type Value = QuotaCeilingBytes;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a non-negative integer or the string \"not_applicable\"")
            }

            fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
                Ok(QuotaCeilingBytes::Bytes(value))
            }

            fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
                u64::try_from(value)
                    .map(QuotaCeilingBytes::Bytes)
                    .map_err(|_| E::custom("quota_ceiling_bytes must be non-negative"))
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
                if value == "not_applicable" {
                    Ok(QuotaCeilingBytes::NotApplicable)
                } else {
                    Err(E::custom(format!(
                        "unexpected quota_ceiling_bytes sentinel: {value}"
                    )))
                }
            }
        }

        deserializer.deserialize_any(QuotaCeilingVisitor)
    }
}

// --------------------------------------------------------------------------
// Shared sub-records.
// --------------------------------------------------------------------------

/// Scope reference attached to a card, breakdown row, or detail row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorScope {
    pub scope_class: InspectorScopeClass,
    #[serde(default)]
    pub scope_ref: Option<String>,
    pub scope_label: String,
}

/// Quota / policy source on an inspector card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuotaOrPolicySource {
    pub quota_basis_class: QuotaBasisClass,
    pub quota_ceiling_bytes: QuotaCeilingBytes,
    pub quota_authority_class: QuotaAuthorityClass,
    #[serde(default)]
    pub policy_source_ref: Option<String>,
}

/// Scan posture attached to an inspector card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScanPosture {
    pub scan_freshness_class: ScanFreshnessClass,
    #[serde(default)]
    pub last_full_scan_at: Option<String>,
    pub partial_scan_reason_class: PartialScanReasonClass,
    pub low_privilege_scan_reason_class: LowPrivilegeScanReasonClass,
    #[serde(default)]
    pub unscannable_class_ids: Vec<StorageClassId>,
}

/// Inspect-only open action attached to consumers and detail rows. The fields
/// are constrained to the safe-default inspect-only posture; the validator
/// enforces the closed token set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectOnlyOpenAction {
    pub action_id: String,
    pub label: String,
    pub scope_class: String,
    pub authority_class: String,
    pub consent_class: String,
    pub side_effects: Vec<String>,
    pub preserves_evidence_context: bool,
    pub modal_prohibited: bool,
    pub revalidation_on_open: String,
}

impl InspectOnlyOpenAction {
    /// True when the action carries the frozen safe-default inspect-only
    /// posture: local scope, user-local authority, no consent, a single
    /// no-side-effect marker, evidence-preserving, and modal-prohibited.
    pub fn is_safe_default(&self) -> bool {
        self.scope_class == "scope_local_only"
            && self.authority_class == "user_local_authority"
            && self.consent_class == "no_consent_required_safe_default"
            && self.side_effects == ["no_side_effect_inspect_only"]
            && self.preserves_evidence_context
            && self.modal_prohibited
            && matches!(
                self.revalidation_on_open.as_str(),
                "none_already_fresh" | "snapshot_open_read_only"
            )
    }
}

/// Open-details action on an inspector card. Carries a target route ref in
/// addition to the inspect-only posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenDetailsAction {
    pub action_id: String,
    pub label: String,
    pub target_route_ref: String,
    pub scope_class: String,
    pub authority_class: String,
    pub consent_class: String,
    pub side_effects: Vec<String>,
    pub preserves_evidence_context: bool,
    pub revalidation_on_open: String,
    pub modal_prohibited: bool,
}

impl OpenDetailsAction {
    /// True when the action carries the frozen safe-default inspect-only posture.
    pub fn is_safe_default(&self) -> bool {
        !self.target_route_ref.is_empty()
            && self.scope_class == "scope_local_only"
            && self.authority_class == "user_local_authority"
            && self.consent_class == "no_consent_required_safe_default"
            && self.side_effects == ["no_side_effect_inspect_only"]
            && self.preserves_evidence_context
            && self.modal_prohibited
            && matches!(
                self.revalidation_on_open.as_str(),
                "none_already_fresh" | "snapshot_open_read_only"
            )
    }
}

/// One largest-consumer row on a card or breakdown row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LargestConsumerRow {
    pub consumer_class: ConsumerClass,
    pub consumer_ref: String,
    pub consumer_label: String,
    pub consumer_used_bytes: u64,
    pub authority_class: AuthorityClass,
    pub rebuild_cost_class: RebuildCostClass,
    pub mirror_or_import_origin_class: MirrorOrImportOriginClass,
    pub pin_summary_class: PinSummaryClass,
    pub inspect_only_open_action: InspectOnlyOpenAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_pin_ref_summary_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// One pinned-consumer breakdown row on a breakdown row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinnedConsumerBreakdownRow {
    pub pin_source_class: PinSourceClass,
    pub pinned_bytes: u64,
    pub pin_ref_summary_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// One pin-source breakdown row on a workspace-detail row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinSourceBreakdownRow {
    pub pin_source_class: PinSourceClass,
    pub pin_ref_summary_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pin_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Rebuild-cost hint embedded in a workspace-storage detail row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RebuildCostHint {
    pub record_kind: String,
    pub rebuild_cost_hint_schema_version: u32,
    pub rebuild_cost_class: RebuildCostClass,
    pub offline_rebuild_risk_class: OfflineRebuildRiskClass,
    pub startup_impact_class: StartupImpactClass,
    pub provenance_continuity_class: ProvenanceContinuityClass,
    pub rebuild_inputs_required: Vec<RebuildInputClass>,
    pub rebuild_safety_summary_class: RebuildSafetySummaryClass,
    pub rebuild_explanation: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl RebuildCostHint {
    /// True when the hint declares the authoritative, no-rebuild posture.
    pub fn is_authoritative(&self) -> bool {
        matches!(
            self.rebuild_safety_summary_class,
            RebuildSafetySummaryClass::DangerousToDeleteAuthoritative
        )
    }
}

// --------------------------------------------------------------------------
// Top records.
// --------------------------------------------------------------------------

/// Storage-inspector card record for one inspector scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageInspectorCard {
    pub record_kind: String,
    pub storage_inspector_card_schema_version: u32,
    pub card_id: String,
    pub emitted_at: String,
    pub inspector_scope: InspectorScope,
    pub total_used_bytes: u64,
    pub quota_or_policy_source: QuotaOrPolicySource,
    pub class_breakdown_row_refs: Vec<String>,
    #[serde(default)]
    pub largest_consumers: Vec<LargestConsumerRow>,
    pub scan_posture: ScanPosture,
    #[serde(default)]
    pub protected_class_visibility: Vec<ProtectedClassVisibility>,
    pub open_details_action: OpenDetailsAction,
    pub consumer_surfaces: Vec<ConsumerSurfaceClass>,
    pub redaction_class: RedactionClass,
    pub export_safe: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_clear_cache_preview_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_low_disk_drill_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_deployment_summary_card_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_continuity_packet_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl StorageInspectorCard {
    /// Returns the card's largest consumers re-sorted by raw bytes, descending,
    /// breaking ties by consumer ref. The persisted [`Self::largest_consumers`]
    /// keeps the authority-aware order surfaces render; this helper answers the
    /// "which entries hold the most bytes" question for diagnostics.
    pub fn top_consumers_by_bytes(&self) -> Vec<&LargestConsumerRow> {
        let mut rows: Vec<&LargestConsumerRow> = self.largest_consumers.iter().collect();
        rows.sort_by(|a, b| {
            b.consumer_used_bytes
                .cmp(&a.consumer_used_bytes)
                .then_with(|| a.consumer_ref.cmp(&b.consumer_ref))
        });
        rows
    }
}

/// Storage-class breakdown row for one storage class on one scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageClassBreakdownRow {
    pub record_kind: String,
    pub storage_class_breakdown_row_schema_version: u32,
    pub row_id: String,
    pub card_id_ref: String,
    pub storage_class_id: StorageClassId,
    pub class_scope: InspectorScope,
    pub class_used_bytes: u64,
    pub reclaimable_bytes_estimate: u64,
    pub protected_bytes: u64,
    pub pinned_bytes: u64,
    pub authority_class: AuthorityClass,
    pub rebuild_cost_class: RebuildCostClass,
    pub gc_policy_class: GcPolicyClass,
    pub clear_cache_protection_class: ClearCacheProtectionClass,
    pub quota_basis_class: QuotaBasisClass,
    pub quota_ceiling_bytes: QuotaCeilingBytes,
    pub mirror_or_import_origin_class: MirrorOrImportOriginClass,
    #[serde(default)]
    pub mirror_offline_artifact_row_ref: Option<String>,
    #[serde(default)]
    pub pinned_consumer_breakdown: Vec<PinnedConsumerBreakdownRow>,
    #[serde(default)]
    pub largest_consumers: Vec<LargestConsumerRow>,
    pub posture: StoragePostureClass,
    #[serde(default)]
    pub last_class_scan_at: Option<String>,
    pub inspectable_on_surfaces: Vec<InspectableSurfaceClass>,
    pub redaction_class: RedactionClass,
    pub export_safe: bool,
    pub note: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_clear_cache_preview_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_low_disk_drill_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_deployment_summary_card_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_pin_manager_route_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_class_specific_review_ref: Option<String>,
}

impl StorageClassBreakdownRow {
    /// True when the class is one of the two protected storage classes.
    pub fn is_protected_class(&self) -> bool {
        matches!(
            self.storage_class_id,
            StorageClassId::EvidenceSupportCache | StorageClassId::UserOwnedRecoveryState
        )
    }
}

/// Workspace-storage detail row for one entry-level disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceStorageDetailRow {
    pub record_kind: String,
    pub workspace_storage_detail_row_schema_version: u32,
    pub row_id: String,
    pub card_id_ref: String,
    pub breakdown_row_id_ref: String,
    pub detail_scope: InspectorScope,
    pub storage_class_id: StorageClassId,
    pub entry_class: EntryClass,
    pub entry_ref: String,
    pub entry_label: String,
    pub size_bytes: u64,
    #[serde(default)]
    pub last_used_at: Option<String>,
    pub rebuild_cost_hint: RebuildCostHint,
    pub authority_class: AuthorityClass,
    pub detail_authority_posture_class: DetailAuthorityPostureClass,
    pub mirror_or_import_origin_class: MirrorOrImportOriginClass,
    #[serde(default)]
    pub mirror_offline_artifact_row_ref: Option<String>,
    pub sensitivity_class: SensitivityClass,
    pub freshness_state: FreshnessStateClass,
    pub corruption_state: CorruptionStateClass,
    pub policy_protection_state: PolicyProtectionStateClass,
    pub pin_state: PinStateClass,
    #[serde(default)]
    pub pin_source_breakdown: Vec<PinSourceBreakdownRow>,
    pub clear_cache_protection_class: ClearCacheProtectionClass,
    pub clear_action: ClearActionClass,
    pub export_action: ExportActionClass,
    pub inspect_only_open_action: InspectOnlyOpenAction,
    pub redaction_class: RedactionClass,
    pub export_safe: bool,
    pub note: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_clear_cache_preview_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_class_specific_review_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_pin_manager_route_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_low_disk_drill_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_deployment_summary_card_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_corruption_rescue_ref: Option<String>,
}

// --------------------------------------------------------------------------
// Corpus container, entries, and loaders.
// --------------------------------------------------------------------------

/// One inspector-card fixture paired with its repository-relative path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageInspectorCardEntry {
    pub fixture_ref: String,
    pub card: StorageInspectorCard,
}

/// One breakdown-row fixture paired with its repository-relative path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageClassBreakdownEntry {
    pub fixture_ref: String,
    pub row: StorageClassBreakdownRow,
}

/// One workspace-detail fixture paired with its repository-relative path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceStorageDetailEntry {
    pub fixture_ref: String,
    pub row: WorkspaceStorageDetailRow,
}

/// Storage-inspector corpus loaded from the checked-in fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageInspectorCorpus {
    pub cards: Vec<StorageInspectorCardEntry>,
    pub breakdown_rows: Vec<StorageClassBreakdownEntry>,
    pub detail_rows: Vec<WorkspaceStorageDetailEntry>,
}

const CARD_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/storage/storage_inspector_cases/mirror_heavy_offline_install_card.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/storage_inspector_cases/mirror_heavy_offline_install_card.yaml"
        )),
    ),
    (
        "fixtures/storage/storage_inspector_cases/policy_limited_scan_card.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/storage_inspector_cases/policy_limited_scan_card.yaml"
        )),
    ),
    (
        "fixtures/storage/storage_inspector_cases/shared_profile_multi_workspace_card.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/storage_inspector_cases/shared_profile_multi_workspace_card.yaml"
        )),
    ),
    (
        "fixtures/storage/storage_inspector_cases/single_workspace_local_profile_card.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/storage_inspector_cases/single_workspace_local_profile_card.yaml"
        )),
    ),
];

const BREAKDOWN_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/storage/storage_inspector_cases/mirror_heavy_offline_install_row_artifact_cache_mirrored.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/storage_inspector_cases/mirror_heavy_offline_install_row_artifact_cache_mirrored.yaml"
        )),
    ),
    (
        "fixtures/storage/storage_inspector_cases/mirror_heavy_offline_install_row_prebuild_environment_offline_bundle.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/storage_inspector_cases/mirror_heavy_offline_install_row_prebuild_environment_offline_bundle.yaml"
        )),
    ),
    (
        "fixtures/storage/storage_inspector_cases/policy_limited_scan_row_evidence_support_cache_unscannable.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/storage_inspector_cases/policy_limited_scan_row_evidence_support_cache_unscannable.yaml"
        )),
    ),
    (
        "fixtures/storage/storage_inspector_cases/shared_profile_multi_workspace_row_evidence_support_cache.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/storage_inspector_cases/shared_profile_multi_workspace_row_evidence_support_cache.yaml"
        )),
    ),
    (
        "fixtures/storage/storage_inspector_cases/single_workspace_local_profile_row_knowledge_cache.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/storage_inspector_cases/single_workspace_local_profile_row_knowledge_cache.yaml"
        )),
    ),
    (
        "fixtures/storage/storage_inspector_cases/single_workspace_local_profile_row_user_owned_recovery_state.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/storage_inspector_cases/single_workspace_local_profile_row_user_owned_recovery_state.yaml"
        )),
    ),
];

const DETAIL_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/storage/workspace_storage_detail_cases/imported_docs_pack_mirror_segment_row.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/workspace_storage_detail_cases/imported_docs_pack_mirror_segment_row.yaml"
        )),
    ),
    (
        "fixtures/storage/workspace_storage_detail_cases/interactive_hot_cache_shard_row.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/workspace_storage_detail_cases/interactive_hot_cache_shard_row.yaml"
        )),
    ),
    (
        "fixtures/storage/workspace_storage_detail_cases/knowledge_cache_search_index_row.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/workspace_storage_detail_cases/knowledge_cache_search_index_row.yaml"
        )),
    ),
    (
        "fixtures/storage/workspace_storage_detail_cases/policy_held_evidence_packet_row.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/workspace_storage_detail_cases/policy_held_evidence_packet_row.yaml"
        )),
    ),
    (
        "fixtures/storage/workspace_storage_detail_cases/user_owned_recovery_journal_row.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/storage/workspace_storage_detail_cases/user_owned_recovery_journal_row.yaml"
        )),
    ),
];

/// Strongly typed error returned by the corpus loaders.
#[derive(Debug)]
pub enum StorageInspectorLoadError {
    Yaml {
        fixture_ref: String,
        source: serde_yaml::Error,
    },
}

impl fmt::Display for StorageInspectorLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yaml {
                fixture_ref,
                source,
            } => {
                write!(
                    f,
                    "storage-inspector yaml parse error in {fixture_ref}: {source}"
                )
            }
        }
    }
}

impl Error for StorageInspectorLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Yaml { source, .. } => Some(source),
        }
    }
}

fn parse_fixtures<T: for<'de> Deserialize<'de>>(
    fixtures: &[(&str, &str)],
) -> Result<Vec<(String, T)>, StorageInspectorLoadError> {
    fixtures
        .iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str::<T>(yaml)
                .map(|record| ((*fixture_ref).to_owned(), record))
                .map_err(|source| StorageInspectorLoadError::Yaml {
                    fixture_ref: (*fixture_ref).to_owned(),
                    source,
                })
        })
        .collect()
}

/// Loads the checked-in storage-inspector corpus (cards, breakdown rows, and
/// workspace-storage detail rows).
pub fn current_storage_inspector_corpus(
) -> Result<StorageInspectorCorpus, StorageInspectorLoadError> {
    let cards = parse_fixtures::<StorageInspectorCard>(CARD_FIXTURES)?
        .into_iter()
        .map(|(fixture_ref, card)| StorageInspectorCardEntry { fixture_ref, card })
        .collect();
    let breakdown_rows = parse_fixtures::<StorageClassBreakdownRow>(BREAKDOWN_FIXTURES)?
        .into_iter()
        .map(|(fixture_ref, row)| StorageClassBreakdownEntry { fixture_ref, row })
        .collect();
    let detail_rows = parse_fixtures::<WorkspaceStorageDetailRow>(DETAIL_FIXTURES)?
        .into_iter()
        .map(|(fixture_ref, row)| WorkspaceStorageDetailEntry { fixture_ref, row })
        .collect();
    Ok(StorageInspectorCorpus {
        cards,
        breakdown_rows,
        detail_rows,
    })
}

// --------------------------------------------------------------------------
// Validation.
// --------------------------------------------------------------------------

/// A validation violation surfaced by the corpus harness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageInspectorViolation {
    pub check_id: String,
    pub target_ref: String,
    pub message: String,
}

impl fmt::Display for StorageInspectorViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}: {}",
            self.check_id, self.target_ref, self.message
        )
    }
}

fn push(
    violations: &mut Vec<StorageInspectorViolation>,
    check_id: impl Into<String>,
    target_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(StorageInspectorViolation {
        check_id: check_id.into(),
        target_ref: target_ref.into(),
        message: message.into(),
    });
}

impl StorageInspectorCorpus {
    /// Returns the loaded card with the given id, if present.
    pub fn card(&self, card_id: &str) -> Option<&StorageInspectorCard> {
        self.cards
            .iter()
            .map(|entry| &entry.card)
            .find(|card| card.card_id == card_id)
    }

    /// Returns the loaded breakdown row with the given id, if present.
    pub fn breakdown_row(&self, row_id: &str) -> Option<&StorageClassBreakdownRow> {
        self.breakdown_rows
            .iter()
            .map(|entry| &entry.row)
            .find(|row| row.row_id == row_id)
    }

    /// Returns the loaded breakdown rows that belong to the given card, sorted
    /// by storage class for a stable, inspectable order.
    pub fn class_breakdown_for(&self, card_id: &str) -> Vec<&StorageClassBreakdownRow> {
        let mut rows: Vec<&StorageClassBreakdownRow> = self
            .breakdown_rows
            .iter()
            .map(|entry| &entry.row)
            .filter(|row| row.card_id_ref == card_id)
            .collect();
        rows.sort_by(|a, b| {
            a.storage_class_id
                .cmp(&b.storage_class_id)
                .then_with(|| a.row_id.cmp(&b.row_id))
        });
        rows
    }

    /// Returns the loaded workspace-detail rows that belong to the given card.
    pub fn detail_rows_for(&self, card_id: &str) -> Vec<&WorkspaceStorageDetailRow> {
        let mut rows: Vec<&WorkspaceStorageDetailRow> = self
            .detail_rows
            .iter()
            .map(|entry| &entry.row)
            .filter(|row| row.card_id_ref == card_id)
            .collect();
        rows.sort_by(|a, b| a.row_id.cmp(&b.row_id));
        rows
    }

    /// Validates the corpus against the storage-inspector safety contract.
    pub fn validate(&self) -> Vec<StorageInspectorViolation> {
        let mut violations = Vec::new();

        if self.cards.is_empty() {
            push(
                &mut violations,
                "corpus.no_cards",
                STORAGE_INSPECTOR_CASES_DIR,
                "corpus must load at least one inspector card",
            );
        }
        if self.detail_rows.is_empty() {
            push(
                &mut violations,
                "corpus.no_detail_rows",
                WORKSPACE_STORAGE_DETAIL_CASES_DIR,
                "corpus must load at least one workspace-storage detail row",
            );
        }

        let mut card_ids: BTreeSet<&str> = BTreeSet::new();
        for entry in &self.cards {
            if !card_ids.insert(entry.card.card_id.as_str()) {
                push(
                    &mut violations,
                    "card.duplicate_id",
                    &entry.card.card_id,
                    "card_id must be unique within the corpus",
                );
            }
            validate_card(&mut violations, &entry.card);
        }

        let mut row_ids: BTreeSet<&str> = BTreeSet::new();
        for entry in &self.breakdown_rows {
            if !row_ids.insert(entry.row.row_id.as_str()) {
                push(
                    &mut violations,
                    "breakdown.duplicate_id",
                    &entry.row.row_id,
                    "row_id must be unique within the corpus",
                );
            }
            validate_breakdown_row(&mut violations, &entry.row, self);
        }

        let mut detail_ids: BTreeSet<&str> = BTreeSet::new();
        for entry in &self.detail_rows {
            if !detail_ids.insert(entry.row.row_id.as_str()) {
                push(
                    &mut violations,
                    "detail.duplicate_id",
                    &entry.row.row_id,
                    "row_id must be unique within the corpus",
                );
            }
            validate_detail_row(&mut violations, &entry.row, self);
        }

        violations
    }
}

fn validate_card(violations: &mut Vec<StorageInspectorViolation>, card: &StorageInspectorCard) {
    let target = card.card_id.as_str();

    if card.record_kind != STORAGE_INSPECTOR_CARD_RECORD_KIND {
        push(
            violations,
            "card.record_kind",
            target,
            "record_kind must be storage_inspector_card_record",
        );
    }
    if card.storage_inspector_card_schema_version != STORAGE_INSPECTOR_CARD_SCHEMA_VERSION {
        push(
            violations,
            "card.schema_version",
            target,
            "schema_version must be 1",
        );
    }
    if card.card_id.trim().is_empty() {
        push(
            violations,
            "card.card_id",
            target,
            "card_id must be non-empty",
        );
    }

    validate_scope(
        violations,
        target,
        "card.inspector_scope",
        &card.inspector_scope,
    );

    if card.class_breakdown_row_refs.is_empty() {
        push(
            violations,
            "card.class_breakdown_row_refs.empty",
            target,
            "class_breakdown_row_refs must name at least one class row",
        );
    }
    let mut seen_refs: BTreeSet<&str> = BTreeSet::new();
    for row_ref in &card.class_breakdown_row_refs {
        if !seen_refs.insert(row_ref.as_str()) {
            push(
                violations,
                "card.class_breakdown_row_refs.duplicate",
                target,
                format!("class_breakdown_row_refs has duplicate ref {row_ref}"),
            );
        }
    }

    if card.consumer_surfaces.is_empty() {
        push(
            violations,
            "card.consumer_surfaces.empty",
            target,
            "consumer_surfaces must name at least one surface",
        );
    }
    let mut seen_surfaces: BTreeSet<ConsumerSurfaceClass> = BTreeSet::new();
    for surface in &card.consumer_surfaces {
        if !seen_surfaces.insert(*surface) {
            push(
                violations,
                "card.consumer_surfaces.duplicate",
                target,
                "consumer_surfaces must be unique",
            );
        }
    }

    // Scan-posture honesty.
    let scan = &card.scan_posture;
    match scan.scan_freshness_class {
        ScanFreshnessClass::ScanUnknownNoScanYet => {
            if card.total_used_bytes != 0 {
                push(
                    violations,
                    "card.unknown_scan_nonzero_total",
                    target,
                    "scan_unknown_no_scan_yet forces total_used_bytes to 0",
                );
            }
            if scan.last_full_scan_at.is_some() {
                push(
                    violations,
                    "card.unknown_scan_has_timestamp",
                    target,
                    "scan_unknown_no_scan_yet carries a null last_full_scan_at",
                );
            }
        }
        ScanFreshnessClass::ScanInFlight => {
            if scan.partial_scan_reason_class != PartialScanReasonClass::PartialScanInProgress {
                push(
                    violations,
                    "card.scan_in_flight_partial_reason",
                    target,
                    "scan_in_flight requires partial_scan_reason_class=partial_scan_in_progress",
                );
            }
            if scan.last_full_scan_at.is_none() {
                push(
                    violations,
                    "card.scan_in_flight_missing_timestamp",
                    target,
                    "scan_in_flight carries a non-null last_full_scan_at",
                );
            }
        }
        _ => {
            if scan.last_full_scan_at.is_none() {
                push(
                    violations,
                    "card.scan_missing_timestamp",
                    target,
                    "a completed scan carries a non-null last_full_scan_at",
                );
            }
        }
    }
    if matches!(
        scan.scan_freshness_class,
        ScanFreshnessClass::ScanPastExtendedWindow
    ) && card
        .consumer_surfaces
        .contains(&ConsumerSurfaceClass::LowDiskBanner)
    {
        push(
            violations,
            "card.stale_scan_drives_low_disk",
            target,
            "a past-extended-window scan must not list the low-disk banner as a consumer",
        );
    }
    if matches!(
        scan.low_privilege_scan_reason_class,
        LowPrivilegeScanReasonClass::LowPrivilegeAdminConsentRequired
    ) && !card
        .consumer_surfaces
        .contains(&ConsumerSurfaceClass::AdminStorageConsole)
    {
        push(
            violations,
            "card.admin_consent_missing_console",
            target,
            "admin-consent-required scans must list the admin storage console",
        );
    }
    if matches!(
        scan.low_privilege_scan_reason_class,
        LowPrivilegeScanReasonClass::LowPrivilegeManagedTenantRestriction
    ) && card.linked_deployment_summary_card_ref.is_none()
    {
        push(
            violations,
            "card.managed_tenant_missing_deployment_link",
            target,
            "managed-tenant restriction must link the deployment-summary card",
        );
    }
    if matches!(
        scan.partial_scan_reason_class,
        PartialScanReasonClass::PartialDueToLowDisk
    ) && card.linked_low_disk_drill_ref.is_none()
    {
        push(
            violations,
            "card.low_disk_partial_missing_drill",
            target,
            "a low-disk partial scan must link the low-disk drill that bounded it",
        );
    }

    if card.total_used_bytes > 0 && card.largest_consumers.is_empty() {
        push(
            violations,
            "card.nonzero_total_no_consumers",
            target,
            "a non-zero total requires at least one largest-consumer row",
        );
    }

    // Protected-class visibility on broad scopes.
    if card.inspector_scope.scope_class.is_broad() {
        let has_evidence = card
            .protected_class_visibility
            .contains(&ProtectedClassVisibility::EvidenceSupportCacheVisible);
        let has_recovery = card
            .protected_class_visibility
            .contains(&ProtectedClassVisibility::UserOwnedRecoveryStateVisible);
        if !has_evidence || !has_recovery {
            push(
                violations,
                "card.protected_visibility_missing",
                target,
                "device/workspace/tenant scopes must list both protected-class visibility tokens",
            );
        }
    }

    if card.export_safe && !card.redaction_class.is_export_safe() {
        push(
            violations,
            "card.export_safe_redaction",
            target,
            "export_safe cards must keep redaction at metadata_safe_default or operator_only_restricted",
        );
    }

    if !card.open_details_action.is_safe_default() {
        push(
            violations,
            "card.open_details_action_unsafe",
            target,
            "open_details_action must carry the safe-default inspect-only posture with a target route",
        );
    }

    for consumer in &card.largest_consumers {
        validate_consumer_row(violations, target, consumer);
    }
}

fn validate_scope(
    violations: &mut Vec<StorageInspectorViolation>,
    target: &str,
    check_prefix: &str,
    scope: &InspectorScope,
) {
    let device_total = matches!(scope.scope_class, InspectorScopeClass::DeviceTotal);
    if device_total && scope.scope_ref.is_some() {
        push(
            violations,
            format!("{check_prefix}.device_total_ref"),
            target,
            "device_total scope carries a null scope_ref",
        );
    }
    if !device_total && scope.scope_ref.is_none() {
        push(
            violations,
            format!("{check_prefix}.scope_ref_required"),
            target,
            "non-device scopes carry a non-null scope_ref",
        );
    }
    if scope.scope_label.trim().is_empty() {
        push(
            violations,
            format!("{check_prefix}.scope_label"),
            target,
            "scope_label must be non-empty",
        );
    }
}

fn validate_consumer_row(
    violations: &mut Vec<StorageInspectorViolation>,
    target: &str,
    consumer: &LargestConsumerRow,
) {
    if matches!(
        consumer.authority_class,
        AuthorityClass::UserAuthoredDurableTruth
    ) {
        push(
            violations,
            "consumer.authored_durable_truth",
            target,
            "user_authored_durable_truth authority is never a cache consumer",
        );
    }
    if matches!(
        consumer.consumer_class,
        ConsumerClass::AggregatedRemainderOther
    ) {
        if !matches!(
            consumer.authority_class,
            AuthorityClass::DisposableDerivedCache
        ) {
            push(
                violations,
                "consumer.remainder_authority",
                target,
                "aggregated_remainder_other rows carry disposable_derived_cache authority",
            );
        }
        if !matches!(
            consumer.mirror_or_import_origin_class,
            MirrorOrImportOriginClass::NotApplicable
        ) {
            push(
                violations,
                "consumer.remainder_origin",
                target,
                "aggregated_remainder_other rows carry a not_applicable origin",
            );
        }
        if !matches!(consumer.pin_summary_class, PinSummaryClass::NotApplicable) {
            push(
                violations,
                "consumer.remainder_pin_summary",
                target,
                "aggregated_remainder_other rows carry a not_applicable pin summary",
            );
        }
    }
    if !consumer.inspect_only_open_action.is_safe_default() {
        push(
            violations,
            "consumer.open_action_unsafe",
            target,
            "consumer inspect_only_open_action must carry the safe-default inspect-only posture",
        );
    }
}

fn validate_breakdown_row(
    violations: &mut Vec<StorageInspectorViolation>,
    row: &StorageClassBreakdownRow,
    corpus: &StorageInspectorCorpus,
) {
    let target = row.row_id.as_str();

    if row.record_kind != STORAGE_CLASS_BREAKDOWN_ROW_RECORD_KIND {
        push(
            violations,
            "breakdown.record_kind",
            target,
            "record_kind must be storage_class_breakdown_row_record",
        );
    }
    if row.storage_class_breakdown_row_schema_version != STORAGE_CLASS_BREAKDOWN_ROW_SCHEMA_VERSION
    {
        push(
            violations,
            "breakdown.schema_version",
            target,
            "schema_version must be 1",
        );
    }

    // The card the row claims must exist, and must list this row id.
    match corpus.card(&row.card_id_ref) {
        None => push(
            violations,
            "breakdown.card_ref_unresolved",
            target,
            format!("card_id_ref {} has no loaded card", row.card_id_ref),
        ),
        Some(card) => {
            if !card
                .class_breakdown_row_refs
                .iter()
                .any(|r| r == &row.row_id)
            {
                push(
                    violations,
                    "breakdown.not_listed_on_card",
                    target,
                    "row_id is not listed in the parent card's class_breakdown_row_refs",
                );
            }
        }
    }

    validate_scope(
        violations,
        target,
        "breakdown.class_scope",
        &row.class_scope,
    );

    if matches!(
        row.authority_class,
        AuthorityClass::UserAuthoredDurableTruth
    ) {
        push(
            violations,
            "breakdown.authored_durable_truth",
            target,
            "user_authored_durable_truth authority never registers as a breakdown row",
        );
    }

    validate_class_posture(violations, row);

    if matches!(row.posture, StoragePostureClass::RetainedForEvidence) && !row.is_protected_class()
    {
        push(
            violations,
            "breakdown.retained_for_evidence_class",
            target,
            "retained_for_evidence posture is limited to evidence / user-owned classes",
        );
    }
    if matches!(row.posture, StoragePostureClass::Missing) && row.class_used_bytes != 0 {
        push(
            violations,
            "breakdown.missing_posture_nonzero",
            target,
            "missing posture forces class_used_bytes to 0",
        );
    }
    if row.is_protected_class() && row.reclaimable_bytes_estimate != 0 {
        push(
            violations,
            "breakdown.protected_reclaimable_nonzero",
            target,
            "protected classes carry a zero reclaimable_bytes_estimate",
        );
    }

    validate_mirror_origin(
        violations,
        target,
        "breakdown",
        row.storage_class_id,
        row.mirror_or_import_origin_class,
        row.mirror_offline_artifact_row_ref.as_deref(),
        Some(row.clear_cache_protection_class),
    );

    if matches!(
        row.mirror_or_import_origin_class,
        MirrorOrImportOriginClass::NotApplicable
    ) {
        let all_zero = row.class_used_bytes == 0
            && row.reclaimable_bytes_estimate == 0
            && row.protected_bytes == 0
            && row.pinned_bytes == 0;
        if !all_zero
            || !row.largest_consumers.is_empty()
            || !row.pinned_consumer_breakdown.is_empty()
        {
            push(
                violations,
                "breakdown.not_applicable_nonzero",
                target,
                "not_applicable origin forces zero bytes and empty consumer / pin breakdowns",
            );
        }
    }

    // Pinned-bytes and consumer-row coupling.
    if row.pinned_bytes > 0 && row.pinned_consumer_breakdown.is_empty() {
        push(
            violations,
            "breakdown.pinned_without_breakdown",
            target,
            "a non-zero pinned_bytes total requires a non-empty pinned_consumer_breakdown",
        );
    }
    if row.pinned_bytes == 0 && !row.pinned_consumer_breakdown.is_empty() {
        push(
            violations,
            "breakdown.unpinned_with_breakdown",
            target,
            "a zero pinned_bytes total carries no pinned_consumer_breakdown rows",
        );
    }
    if row.class_used_bytes > 0 && row.largest_consumers.is_empty() {
        push(
            violations,
            "breakdown.nonzero_without_consumers",
            target,
            "a non-zero class_used_bytes requires at least one largest-consumer row",
        );
    }
    if row.class_used_bytes == 0 && !row.largest_consumers.is_empty() {
        push(
            violations,
            "breakdown.zero_with_consumers",
            target,
            "a zero class_used_bytes carries no largest-consumer rows",
        );
    }

    if row.inspectable_on_surfaces.is_empty() {
        push(
            violations,
            "breakdown.inspectable_surfaces_empty",
            target,
            "inspectable_on_surfaces must name at least one surface",
        );
    }
    let mut seen_surfaces: BTreeSet<InspectableSurfaceClass> = BTreeSet::new();
    for surface in &row.inspectable_on_surfaces {
        if !seen_surfaces.insert(*surface) {
            push(
                violations,
                "breakdown.inspectable_surfaces_duplicate",
                target,
                "inspectable_on_surfaces must be unique",
            );
        }
    }

    if row.export_safe && !row.redaction_class.is_export_safe() {
        push(
            violations,
            "breakdown.export_safe_redaction",
            target,
            "export_safe rows keep redaction at metadata_safe_default or operator_only_restricted",
        );
    }

    for consumer in &row.largest_consumers {
        validate_consumer_row(violations, target, consumer);
    }
}

fn validate_detail_row(
    violations: &mut Vec<StorageInspectorViolation>,
    row: &WorkspaceStorageDetailRow,
    corpus: &StorageInspectorCorpus,
) {
    let target = row.row_id.as_str();

    if row.record_kind != WORKSPACE_STORAGE_DETAIL_ROW_RECORD_KIND {
        push(
            violations,
            "detail.record_kind",
            target,
            "record_kind must be workspace_storage_detail_row_record",
        );
    }
    if row.workspace_storage_detail_row_schema_version
        != WORKSPACE_STORAGE_DETAIL_ROW_SCHEMA_VERSION
    {
        push(
            violations,
            "detail.schema_version",
            target,
            "schema_version must be 1",
        );
    }

    if corpus.card(&row.card_id_ref).is_none() {
        push(
            violations,
            "detail.card_ref_unresolved",
            target,
            format!("card_id_ref {} has no loaded card", row.card_id_ref),
        );
    }
    // A breakdown ref that resolves must agree on the storage class. Seeded
    // detail rows may reference a breakdown row not checked in; that is allowed
    // (the corpus is representative, not exhaustive), so unresolved refs are
    // not a violation.
    if let Some(breakdown) = corpus.breakdown_row(&row.breakdown_row_id_ref) {
        if breakdown.storage_class_id != row.storage_class_id {
            push(
                violations,
                "detail.breakdown_class_mismatch",
                target,
                "breakdown_row_id_ref resolves to a different storage class",
            );
        }
    }

    validate_scope(violations, target, "detail.detail_scope", &row.detail_scope);

    if matches!(
        row.authority_class,
        AuthorityClass::UserAuthoredDurableTruth
    ) {
        push(
            violations,
            "detail.authored_durable_truth",
            target,
            "user_authored_durable_truth authority never registers as a workspace-detail row",
        );
    }

    validate_rebuild_cost_hint(violations, target, &row.rebuild_cost_hint);

    // last_used_at follows the freshness state.
    if row.freshness_state.has_no_timestamp() {
        if row.last_used_at.is_some() {
            push(
                violations,
                "detail.last_used_should_be_null",
                target,
                "unknown / not-applicable freshness carries a null last_used_at",
            );
        }
    } else if row.last_used_at.is_none() {
        push(
            violations,
            "detail.last_used_required",
            target,
            "a freshness state with a signal carries a non-null last_used_at",
        );
    }

    // Storage-class anchored postures and actions.
    match row.storage_class_id {
        StorageClassId::UserOwnedRecoveryState => {
            if !matches!(row.authority_class, AuthorityClass::UserOwnedRecoveryState) {
                push(
                    violations,
                    "detail.user_owned_authority",
                    target,
                    "user_owned_recovery_state binds user_owned_recovery_state authority",
                );
            }
            if !matches!(
                row.detail_authority_posture_class,
                DetailAuthorityPostureClass::AuthoritativeUserOwnedState
            ) {
                push(
                    violations,
                    "detail.user_owned_posture",
                    target,
                    "user_owned_recovery_state binds authoritative_user_owned_state posture",
                );
            }
            if !matches!(
                row.clear_cache_protection_class,
                ClearCacheProtectionClass::ProtectedNeverGenericClear
            ) {
                push(
                    violations,
                    "detail.user_owned_protection",
                    target,
                    "user_owned_recovery_state binds protected_never_generic_clear",
                );
            }
            if !matches!(
                row.policy_protection_state,
                PolicyProtectionStateClass::ProtectedUserOwnedAuthoritative
            ) {
                push(
                    violations,
                    "detail.user_owned_policy_state",
                    target,
                    "user_owned_recovery_state binds protected_user_owned_authoritative",
                );
            }
            if !matches!(
                row.pin_state,
                PinStateClass::NotApplicableAuthoritativeState
            ) {
                push(
                    violations,
                    "detail.user_owned_pin_state",
                    target,
                    "user_owned_recovery_state binds not_applicable_authoritative_state pin state",
                );
            }
            if !matches!(
                row.clear_action,
                ClearActionClass::ClearRefusedAuthoritativeUserOwned
            ) {
                push(
                    violations,
                    "detail.user_owned_clear_action",
                    target,
                    "user_owned_recovery_state refuses the generic clear",
                );
            }
            if !matches!(
                row.export_action,
                ExportActionClass::ExportRequiredBeforeClear
                    | ExportActionClass::ExportOfferedMetadataSafe
                    | ExportActionClass::ExportOfferedOperatorOnly
            ) {
                push(
                    violations,
                    "detail.user_owned_export_action",
                    target,
                    "user_owned_recovery_state offers export before any clear",
                );
            }
            if !matches!(
                row.mirror_or_import_origin_class,
                MirrorOrImportOriginClass::LocalAuthoritative
            ) {
                push(
                    violations,
                    "detail.user_owned_origin",
                    target,
                    "user_owned_recovery_state binds the local_authoritative origin",
                );
            }
            if row.linked_class_specific_review_ref.is_none() {
                push(
                    violations,
                    "detail.user_owned_review_link",
                    target,
                    "user_owned_recovery_state must link the class-specific review",
                );
            }
            if !matches!(
                row.rebuild_cost_hint.rebuild_safety_summary_class,
                RebuildSafetySummaryClass::DangerousToDeleteAuthoritative
            ) {
                push(
                    violations,
                    "detail.user_owned_rebuild_summary",
                    target,
                    "user_owned_recovery_state forces the dangerous-to-delete rebuild summary",
                );
            }
        }
        StorageClassId::EvidenceSupportCache => {
            if !matches!(
                row.detail_authority_posture_class,
                DetailAuthorityPostureClass::PolicyHeldEvidenceState
            ) {
                push(
                    violations,
                    "detail.evidence_posture",
                    target,
                    "evidence_support_cache binds policy_held_evidence_state posture",
                );
            }
            if !matches!(
                row.clear_cache_protection_class,
                ClearCacheProtectionClass::ProtectedRequiresClassSpecificReview
            ) {
                push(
                    violations,
                    "detail.evidence_protection",
                    target,
                    "evidence_support_cache binds protected_requires_class_specific_review",
                );
            }
            if !matches!(
                row.clear_action,
                ClearActionClass::ClearRequiresClassSpecificReview
            ) {
                push(
                    violations,
                    "detail.evidence_clear_action",
                    target,
                    "evidence_support_cache requires the class-specific review clear action",
                );
            }
            if !matches!(
                row.policy_protection_state,
                PolicyProtectionStateClass::ProtectedEvidenceCaseOrReview
                    | PolicyProtectionStateClass::ProtectedRetentionWindow
                    | PolicyProtectionStateClass::ProtectedAdminPolicyPin
                    | PolicyProtectionStateClass::ProtectedTenantPolicyPin
            ) {
                push(
                    violations,
                    "detail.evidence_policy_state",
                    target,
                    "evidence_support_cache binds a policy / case / retention protection state",
                );
            }
            if !matches!(
                row.authority_class,
                AuthorityClass::AdminOrControlArtifact | AuthorityClass::UserOwnedRecoveryState
            ) {
                push(
                    violations,
                    "detail.evidence_authority",
                    target,
                    "evidence_support_cache binds admin/control or user-owned authority",
                );
            }
            if row.linked_class_specific_review_ref.is_none() {
                push(
                    violations,
                    "detail.evidence_review_link",
                    target,
                    "evidence_support_cache must link the class-specific review",
                );
            }
        }
        _ => {}
    }

    // Posture-anchored constraints.
    match row.detail_authority_posture_class {
        DetailAuthorityPostureClass::AuthoritativeUserOwnedState => {
            if !matches!(row.storage_class_id, StorageClassId::UserOwnedRecoveryState) {
                push(
                    violations,
                    "detail.posture_user_owned_class",
                    target,
                    "authoritative_user_owned_state binds the user_owned_recovery_state class",
                );
            }
        }
        DetailAuthorityPostureClass::PolicyHeldEvidenceState => {
            if !matches!(row.storage_class_id, StorageClassId::EvidenceSupportCache) {
                push(
                    violations,
                    "detail.posture_evidence_class",
                    target,
                    "policy_held_evidence_state binds the evidence_support_cache class",
                );
            }
        }
        DetailAuthorityPostureClass::ImportedDurableArtifact => {
            let origin_ok = matches!(
                row.mirror_or_import_origin_class,
                MirrorOrImportOriginClass::MirroredCopy
                    | MirrorOrImportOriginClass::OfflineBundleLocal
                    | MirrorOrImportOriginClass::VendorSignedOfflineLocal
                    | MirrorOrImportOriginClass::CustomerSignedMirrorLocal
                    | MirrorOrImportOriginClass::PolicyProtectedAdminArtifact
            );
            if !origin_ok {
                push(
                    violations,
                    "detail.posture_imported_origin",
                    target,
                    "imported_durable_artifact binds a mirror / offline / policy-protected origin",
                );
            }
            if matches!(
                row.clear_cache_protection_class,
                ClearCacheProtectionClass::GenericClearAlwaysAllowed
            ) {
                push(
                    violations,
                    "detail.posture_imported_generic_clear",
                    target,
                    "imported_durable_artifact forbids generic_clear_always_allowed",
                );
            }
        }
        DetailAuthorityPostureClass::DisposableDerivedState
        | DetailAuthorityPostureClass::CorrectnessRelevantDerivedState => {
            if !matches!(row.authority_class, AuthorityClass::DisposableDerivedCache) {
                push(violations, "detail.posture_disposable_authority", target, "disposable / correctness-relevant postures bind disposable_derived_cache authority");
            }
            if !matches!(
                row.mirror_or_import_origin_class,
                MirrorOrImportOriginClass::LocalDisposableCache
            ) {
                push(violations, "detail.posture_disposable_origin", target, "disposable / correctness-relevant postures bind the local_disposable_cache origin");
            }
        }
    }

    validate_mirror_origin(
        violations,
        target,
        "detail",
        row.storage_class_id,
        row.mirror_or_import_origin_class,
        row.mirror_offline_artifact_row_ref.as_deref(),
        Some(row.clear_cache_protection_class),
    );

    // Pin-state coupling.
    if row.pin_state.requires_breakdown() && row.pin_source_breakdown.is_empty() {
        push(
            violations,
            "detail.pinned_without_breakdown",
            target,
            "a pinned state requires a non-empty pin_source_breakdown",
        );
    }
    if matches!(
        row.pin_state,
        PinStateClass::UnpinnedNoSources | PinStateClass::NotApplicableAuthoritativeState
    ) && !row.pin_source_breakdown.is_empty()
    {
        push(
            violations,
            "detail.unpinned_with_breakdown",
            target,
            "unpinned / not-applicable pin states carry an empty pin_source_breakdown",
        );
    }

    // Clear-action coupling.
    match row.clear_action {
        ClearActionClass::ClearAdmissibleGeneric => {
            if !matches!(
                row.clear_cache_protection_class,
                ClearCacheProtectionClass::GenericClearAlwaysAllowed
            ) || !matches!(row.pin_state, PinStateClass::UnpinnedNoSources)
                || !matches!(row.authority_class, AuthorityClass::DisposableDerivedCache)
            {
                push(violations, "detail.clear_generic_pairing", target, "clear_admissible_generic requires generic_clear_always_allowed + unpinned + disposable authority");
            }
        }
        ClearActionClass::ClearAdmissibleAfterPinRelease => {
            if !matches!(
                row.clear_cache_protection_class,
                ClearCacheProtectionClass::GenericClearWithPinExclusions
            ) || !row.pin_state.requires_breakdown()
            {
                push(violations, "detail.clear_pin_release_pairing", target, "clear_admissible_after_pin_release requires generic_clear_with_pin_exclusions + a pinned state");
            }
        }
        ClearActionClass::ClearRequiresClassSpecificReview => {
            if !matches!(row.storage_class_id, StorageClassId::EvidenceSupportCache) {
                push(
                    violations,
                    "detail.clear_review_class",
                    target,
                    "clear_requires_class_specific_review binds the evidence_support_cache class",
                );
            }
            if row.linked_class_specific_review_ref.is_none() {
                push(
                    violations,
                    "detail.clear_review_link",
                    target,
                    "clear_requires_class_specific_review must link the class-specific review",
                );
            }
        }
        ClearActionClass::ClearRefusedAuthoritativeUserOwned => {
            if !matches!(row.storage_class_id, StorageClassId::UserOwnedRecoveryState) {
                push(violations, "detail.clear_refused_class", target, "clear_refused_authoritative_user_owned binds the user_owned_recovery_state class");
            }
            if row.linked_class_specific_review_ref.is_none() {
                push(
                    violations,
                    "detail.clear_refused_link",
                    target,
                    "clear_refused_authoritative_user_owned must link the class-specific review",
                );
            }
        }
        ClearActionClass::ClearRefusedPolicyHeld => {
            if !matches!(
                row.policy_protection_state,
                PolicyProtectionStateClass::ProtectedAdminPolicyPin
                    | PolicyProtectionStateClass::ProtectedTenantPolicyPin
                    | PolicyProtectionStateClass::ProtectedRetentionWindow
            ) {
                push(
                    violations,
                    "detail.clear_policy_held_state",
                    target,
                    "clear_refused_policy_held requires a policy-held protection state",
                );
            }
        }
    }

    // Export-action coupling.
    if matches!(
        row.export_action,
        ExportActionClass::ExportRequiredBeforeClear
    ) && !matches!(
        row.rebuild_cost_hint.rebuild_cost_class,
        RebuildCostClass::AuthoritativeNoRebuild
    ) {
        push(
            violations,
            "detail.export_required_rebuild_cost",
            target,
            "export_required_before_clear requires authoritative_no_rebuild rebuild cost",
        );
    }
    if matches!(
        row.export_action,
        ExportActionClass::ExportUnsupportedAlreadyLocalOnlyDisposable
    ) && !matches!(row.authority_class, AuthorityClass::DisposableDerivedCache)
    {
        push(
            violations,
            "detail.export_unsupported_disposable",
            target,
            "export_unsupported_already_local_only_disposable requires disposable_derived_cache authority",
        );
    }

    if matches!(
        row.sensitivity_class,
        SensitivityClass::T3SecretAdjacentNotReusableCache
    ) && !matches!(row.storage_class_id, StorageClassId::EvidenceSupportCache)
    {
        push(
            violations,
            "detail.t3_non_evidence",
            target,
            "sensitivity t3 is admissible only on evidence_support_cache rows",
        );
    }

    if row.corruption_state.is_confirmed_corrupt() && row.linked_corruption_rescue_ref.is_none() {
        push(
            violations,
            "detail.corrupt_missing_rescue",
            target,
            "confirmed-corrupt rows must link the corruption-rescue path",
        );
    }

    if row.export_safe && !row.redaction_class.is_export_safe() {
        push(
            violations,
            "detail.export_safe_redaction",
            target,
            "export_safe rows keep redaction at metadata_safe_default or operator_only_restricted",
        );
    }

    if !row.inspect_only_open_action.is_safe_default() {
        push(
            violations,
            "detail.open_action_unsafe",
            target,
            "inspect_only_open_action must carry the safe-default inspect-only posture",
        );
    }
}

/// Validates the storage-class ↔ authority / rebuild / protection / GC pairings
/// a breakdown row must satisfy so a class can never wear another class's
/// disposal posture.
fn validate_class_posture(
    violations: &mut Vec<StorageInspectorViolation>,
    row: &StorageClassBreakdownRow,
) {
    let target = row.row_id.as_str();
    let storage_class_id = row.storage_class_id;
    let authority_class = row.authority_class;
    let rebuild_cost_class = row.rebuild_cost_class;
    let protection = row.clear_cache_protection_class;
    let gc_policy = row.gc_policy_class;
    let has_review_link = row.linked_class_specific_review_ref.is_some();

    match storage_class_id {
        StorageClassId::UserOwnedRecoveryState => {
            if !matches!(authority_class, AuthorityClass::UserOwnedRecoveryState) {
                push(
                    violations,
                    "class.user_owned_authority",
                    target,
                    "user_owned_recovery_state binds user_owned_recovery_state authority",
                );
            }
            if !matches!(
                protection,
                ClearCacheProtectionClass::ProtectedNeverGenericClear
            ) {
                push(
                    violations,
                    "class.user_owned_protection",
                    target,
                    "user_owned_recovery_state binds protected_never_generic_clear",
                );
            }
            if !matches!(
                gc_policy,
                GcPolicyClass::NeverGcAuthoritative | GcPolicyClass::GcOnExplicitResetOnly
            ) {
                push(
                    violations,
                    "class.user_owned_gc",
                    target,
                    "user_owned_recovery_state binds a never-gc / explicit-reset GC policy",
                );
            }
            if !matches!(rebuild_cost_class, RebuildCostClass::AuthoritativeNoRebuild) {
                push(
                    violations,
                    "class.user_owned_rebuild",
                    target,
                    "user_owned_recovery_state binds authoritative_no_rebuild",
                );
            }
            if !has_review_link {
                push(
                    violations,
                    "class.user_owned_review_link",
                    target,
                    "user_owned_recovery_state must link the class-specific review",
                );
            }
        }
        StorageClassId::EvidenceSupportCache => {
            if !matches!(
                protection,
                ClearCacheProtectionClass::ProtectedRequiresClassSpecificReview
            ) {
                push(
                    violations,
                    "class.evidence_protection",
                    target,
                    "evidence_support_cache binds protected_requires_class_specific_review",
                );
            }
            if !matches!(
                gc_policy,
                GcPolicyClass::NeverGcAuthoritative
                    | GcPolicyClass::GcOnCaseClose
                    | GcPolicyClass::GcOnExplicitResetOnly
            ) {
                push(violations, "class.evidence_gc", target, "evidence_support_cache binds a never-gc / case-close / explicit-reset GC policy");
            }
            if !matches!(
                authority_class,
                AuthorityClass::AdminOrControlArtifact | AuthorityClass::UserOwnedRecoveryState
            ) {
                push(
                    violations,
                    "class.evidence_authority",
                    target,
                    "evidence_support_cache binds admin/control or user-owned authority",
                );
            }
            if !has_review_link {
                push(
                    violations,
                    "class.evidence_review_link",
                    target,
                    "evidence_support_cache must link the class-specific review",
                );
            }
        }
        StorageClassId::InteractiveHotCache
        | StorageClassId::KnowledgeCache
        | StorageClassId::PrebuildEnvironmentCache => {
            if !matches!(authority_class, AuthorityClass::DisposableDerivedCache) {
                push(
                    violations,
                    "class.disposable_authority",
                    target,
                    "hot / knowledge / prebuild classes bind disposable_derived_cache authority",
                );
            }
        }
        StorageClassId::ArtifactCache => {
            if !matches!(
                authority_class,
                AuthorityClass::DisposableDerivedCache | AuthorityClass::AdminOrControlArtifact
            ) {
                push(
                    violations,
                    "class.artifact_authority",
                    target,
                    "artifact_cache binds disposable_derived_cache or admin/control authority",
                );
            }
        }
    }

    if matches!(rebuild_cost_class, RebuildCostClass::AuthoritativeNoRebuild)
        && !matches!(
            storage_class_id,
            StorageClassId::EvidenceSupportCache | StorageClassId::UserOwnedRecoveryState
        )
    {
        push(
            violations,
            "class.authoritative_rebuild_class",
            target,
            "authoritative_no_rebuild is limited to evidence / user-owned classes",
        );
    }
}

/// Validates the mirror / import origin ↔ class / ref / protection pairings
/// shared by breakdown and detail rows.
fn validate_mirror_origin(
    violations: &mut Vec<StorageInspectorViolation>,
    target: &str,
    surface: &str,
    storage_class_id: StorageClassId,
    origin: MirrorOrImportOriginClass,
    mirror_ref: Option<&str>,
    protection: Option<ClearCacheProtectionClass>,
) {
    if origin.is_mirror_or_offline() {
        if mirror_ref.is_none() {
            push(
                violations,
                format!("{surface}.mirror_ref_required"),
                target,
                "mirror / offline-bundle origins must cite a mirror_offline_artifact_row_ref",
            );
        }
        if let Some(protection) = protection {
            if matches!(
                protection,
                ClearCacheProtectionClass::GenericClearAlwaysAllowed
            ) {
                push(
                    violations,
                    format!("{surface}.mirror_generic_clear"),
                    target,
                    "mirror / offline-bundle origins forbid generic_clear_always_allowed",
                );
            }
        }
        if !matches!(
            storage_class_id,
            StorageClassId::ArtifactCache
                | StorageClassId::PrebuildEnvironmentCache
                | StorageClassId::EvidenceSupportCache
        ) {
            push(
                violations,
                format!("{surface}.mirror_class"),
                target,
                "mirror / offline-bundle origins pair with cache-class storage classes",
            );
        }
    } else if mirror_ref.is_some() {
        push(
            violations,
            format!("{surface}.non_mirror_ref"),
            target,
            "non-mirror origins carry a null mirror_offline_artifact_row_ref",
        );
    }

    match origin {
        MirrorOrImportOriginClass::LocalAuthoritative => {
            if !matches!(storage_class_id, StorageClassId::UserOwnedRecoveryState) {
                push(
                    violations,
                    format!("{surface}.local_authoritative_class"),
                    target,
                    "local_authoritative origin pairs only with user_owned_recovery_state",
                );
            }
        }
        MirrorOrImportOriginClass::LocalEvidenceCapture => {
            if !matches!(storage_class_id, StorageClassId::EvidenceSupportCache) {
                push(
                    violations,
                    format!("{surface}.local_evidence_class"),
                    target,
                    "local_evidence_capture origin pairs only with evidence_support_cache",
                );
            }
        }
        MirrorOrImportOriginClass::PolicyProtectedAdminArtifact => {
            if !matches!(
                storage_class_id,
                StorageClassId::EvidenceSupportCache | StorageClassId::ArtifactCache
            ) {
                push(
                    violations,
                    format!("{surface}.policy_protected_class"),
                    target,
                    "policy_protected_admin_artifact origin pairs with evidence or artifact cache",
                );
            }
        }
        _ => {}
    }
}

fn validate_rebuild_cost_hint(
    violations: &mut Vec<StorageInspectorViolation>,
    target: &str,
    hint: &RebuildCostHint,
) {
    if hint.record_kind != REBUILD_COST_HINT_RECORD_KIND {
        push(
            violations,
            "rebuild.record_kind",
            target,
            "rebuild_cost_hint record_kind must be rebuild_cost_hint_record",
        );
    }
    if hint.rebuild_cost_hint_schema_version != REBUILD_COST_HINT_SCHEMA_VERSION {
        push(
            violations,
            "rebuild.schema_version",
            target,
            "rebuild_cost_hint schema_version must be 1",
        );
    }
    if hint.rebuild_inputs_required.is_empty() {
        push(
            violations,
            "rebuild.inputs_empty",
            target,
            "rebuild_inputs_required must name at least one input",
        );
    }
    let mut seen_inputs: BTreeSet<RebuildInputClass> = BTreeSet::new();
    for input in &hint.rebuild_inputs_required {
        if !seen_inputs.insert(*input) {
            push(
                violations,
                "rebuild.inputs_duplicate",
                target,
                "rebuild_inputs_required must be unique",
            );
        }
    }

    let contains = |input: RebuildInputClass| hint.rebuild_inputs_required.contains(&input);

    match hint.rebuild_safety_summary_class {
        RebuildSafetySummaryClass::DangerousToDeleteAuthoritative => {
            let ok = matches!(
                hint.rebuild_cost_class,
                RebuildCostClass::AuthoritativeNoRebuild
            ) && matches!(
                hint.offline_rebuild_risk_class,
                OfflineRebuildRiskClass::NotRebuildableAfterRemoval
            ) && matches!(
                hint.startup_impact_class,
                StartupImpactClass::NotApplicableAuthoritativeState
            ) && matches!(
                hint.provenance_continuity_class,
                ProvenanceContinuityClass::AuthoritativeProvenanceIrreplaceable
            ) && hint.rebuild_inputs_required
                == [RebuildInputClass::NoInputAuthoritativeState];
            if !ok {
                push(
                    violations,
                    "rebuild.dangerous_pairing",
                    target,
                    "dangerous_to_delete_authoritative requires the authoritative no-rebuild axes",
                );
            }
        }
        RebuildSafetySummaryClass::CheapToRebuildSafeToRemove => {
            let ok = matches!(
                hint.offline_rebuild_risk_class,
                OfflineRebuildRiskClass::SafeToRemoveOffline
            ) && matches!(
                hint.startup_impact_class,
                StartupImpactClass::NoUserVisibleImpact
                    | StartupImpactClass::SlowerFirstOpenUntilWarm
            ) && matches!(
                hint.provenance_continuity_class,
                ProvenanceContinuityClass::ProvenancePreservedRebuildFromLocalTruth
                    | ProvenanceContinuityClass::NotApplicableDisposableNoProvenance
            ) && matches!(
                hint.rebuild_cost_class,
                RebuildCostClass::LowRebuildCost | RebuildCostClass::MediumRebuildCost
            );
            if !ok {
                push(violations, "rebuild.cheap_pairing", target, "cheap_to_rebuild_safe_to_remove requires safe-offline / low-impact / local-truth / low-cost axes");
            }
        }
        RebuildSafetySummaryClass::ExpensiveToRebuildButSafe => {
            let ok = matches!(
                hint.offline_rebuild_risk_class,
                OfflineRebuildRiskClass::SafeToRemoveOffline
                    | OfflineRebuildRiskClass::RebuildRequiresNetworkResync
            ) && matches!(
                hint.startup_impact_class,
                StartupImpactClass::SlowerFirstQueryUntilReindexed
                    | StartupImpactClass::SlowerFirstBuildUntilPrebuilt
                    | StartupImpactClass::SlowerFirstOpenUntilWarm
            ) && matches!(
                hint.provenance_continuity_class,
                ProvenanceContinuityClass::ProvenancePreservedRebuildFromLocalTruth
                    | ProvenanceContinuityClass::ProvenancePreservedRebuildFromSignedSource
            ) && matches!(
                hint.rebuild_cost_class,
                RebuildCostClass::MediumRebuildCost | RebuildCostClass::HighRebuildCost
            );
            if !ok {
                push(violations, "rebuild.expensive_pairing", target, "expensive_to_rebuild_but_safe requires safe-or-network / slower-rebuild / preserved-provenance axes");
            }
        }
        RebuildSafetySummaryClass::ImpossibleToRebuildOffline => {
            let ok = matches!(
                hint.offline_rebuild_risk_class,
                OfflineRebuildRiskClass::RebuildRequiresMirrorOrOfflineBundle
                    | OfflineRebuildRiskClass::RebuildRequiresAdminOrPolicySignedPack
            ) && matches!(
                hint.startup_impact_class,
                StartupImpactClass::SlowerFirstQueryUntilReindexed
                    | StartupImpactClass::SlowerFirstBuildUntilPrebuilt
                    | StartupImpactClass::FeatureUnavailableUntilRebuilt
            ) && matches!(
                hint.provenance_continuity_class,
                ProvenanceContinuityClass::ProvenancePreservedRebuildFromSignedSource
                    | ProvenanceContinuityClass::ProvenanceBreaksUntilResignedOrReImported
            ) && matches!(
                hint.rebuild_cost_class,
                RebuildCostClass::HighRebuildCost | RebuildCostClass::MediumRebuildCost
            );
            if !ok {
                push(violations, "rebuild.impossible_pairing", target, "impossible_to_rebuild_offline requires mirror / signed-pack offline-risk and matching axes");
            }
        }
    }

    if matches!(
        hint.rebuild_cost_class,
        RebuildCostClass::AuthoritativeNoRebuild
    ) && !matches!(
        hint.rebuild_safety_summary_class,
        RebuildSafetySummaryClass::DangerousToDeleteAuthoritative
    ) {
        push(
            violations,
            "rebuild.authoritative_cost_summary",
            target,
            "authoritative_no_rebuild cost forces the dangerous-to-delete summary",
        );
    }

    match hint.offline_rebuild_risk_class {
        OfflineRebuildRiskClass::RebuildRequiresNetworkResync => {
            if !contains(RebuildInputClass::NetworkProviderOrIndex) {
                push(
                    violations,
                    "rebuild.network_input_missing",
                    target,
                    "network-resync risk requires the network_provider_or_index input",
                );
            }
        }
        OfflineRebuildRiskClass::RebuildRequiresMirrorOrOfflineBundle => {
            let any_mirror = contains(RebuildInputClass::CustomerOperatedMirror)
                || contains(RebuildInputClass::VendorPublishedMirror)
                || contains(RebuildInputClass::OfflineBundleLocal)
                || contains(RebuildInputClass::VendorSignedOfflineBundle)
                || contains(RebuildInputClass::CustomerSignedMirror);
            if !any_mirror {
                push(
                    violations,
                    "rebuild.mirror_input_missing",
                    target,
                    "mirror / offline-bundle risk requires a mirror / offline-bundle input",
                );
            }
        }
        OfflineRebuildRiskClass::RebuildRequiresAdminOrPolicySignedPack => {
            if !contains(RebuildInputClass::PolicySignedPack) {
                push(
                    violations,
                    "rebuild.policy_pack_input_missing",
                    target,
                    "admin/policy-signed-pack risk requires the policy_signed_pack input",
                );
            }
        }
        _ => {}
    }

    if contains(RebuildInputClass::NoInputAuthoritativeState) {
        if hint.rebuild_inputs_required != [RebuildInputClass::NoInputAuthoritativeState] {
            push(
                violations,
                "rebuild.authoritative_input_mixed",
                target,
                "no_input_authoritative_state cannot mix with other inputs",
            );
        }
        if !matches!(
            hint.rebuild_cost_class,
            RebuildCostClass::AuthoritativeNoRebuild
        ) {
            push(
                violations,
                "rebuild.authoritative_input_cost",
                target,
                "no_input_authoritative_state requires authoritative_no_rebuild cost",
            );
        }
    }
    if hint.rebuild_explanation.trim().is_empty() {
        push(
            violations,
            "rebuild.explanation_empty",
            target,
            "rebuild_explanation must be non-empty",
        );
    }
}

// --------------------------------------------------------------------------
// Support-export projection.
// --------------------------------------------------------------------------

/// One class row in a support-export card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportClassRow {
    pub storage_class_id: StorageClassId,
    pub class_used_bytes: u64,
    pub reclaimable_bytes_estimate: u64,
    pub protected_bytes: u64,
    pub pinned_bytes: u64,
    pub authority_class: AuthorityClass,
    pub rebuild_cost_class: RebuildCostClass,
    pub clear_cache_protection_class: ClearCacheProtectionClass,
    pub posture: StoragePostureClass,
    pub mirror_or_import_origin_class: MirrorOrImportOriginClass,
    pub largest_consumer_count: u32,
}

impl SupportExportClassRow {
    fn from_row(row: &StorageClassBreakdownRow) -> Self {
        Self {
            storage_class_id: row.storage_class_id,
            class_used_bytes: row.class_used_bytes,
            reclaimable_bytes_estimate: row.reclaimable_bytes_estimate,
            protected_bytes: row.protected_bytes,
            pinned_bytes: row.pinned_bytes,
            authority_class: row.authority_class,
            rebuild_cost_class: row.rebuild_cost_class,
            clear_cache_protection_class: row.clear_cache_protection_class,
            posture: row.posture,
            mirror_or_import_origin_class: row.mirror_or_import_origin_class,
            largest_consumer_count: row.largest_consumers.len() as u32,
        }
    }
}

/// One largest-consumer row in a support-export card. Carries no raw refs, only
/// the bounded label plus the authority / rebuild / pin posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportConsumerRow {
    pub consumer_class: ConsumerClass,
    pub consumer_label: String,
    pub consumer_used_bytes: u64,
    pub authority_class: AuthorityClass,
    pub rebuild_cost_class: RebuildCostClass,
    pub pin_summary_class: PinSummaryClass,
}

impl SupportExportConsumerRow {
    fn from_row(row: &LargestConsumerRow) -> Self {
        Self {
            consumer_class: row.consumer_class,
            consumer_label: row.consumer_label.clone(),
            consumer_used_bytes: row.consumer_used_bytes,
            authority_class: row.authority_class,
            rebuild_cost_class: row.rebuild_cost_class,
            pin_summary_class: row.pin_summary_class,
        }
    }
}

/// One card in the support-export envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageInspectorSupportExportCard {
    pub record_kind: String,
    pub card_id: String,
    pub scope_class: InspectorScopeClass,
    pub scope_label: String,
    pub total_used_bytes: u64,
    pub quota_basis_class: QuotaBasisClass,
    pub quota_authority_class: QuotaAuthorityClass,
    pub scan_freshness_class: ScanFreshnessClass,
    pub protected_class_visibility: Vec<ProtectedClassVisibility>,
    pub class_breakdown: Vec<SupportExportClassRow>,
    pub largest_consumers: Vec<SupportExportConsumerRow>,
    pub detail_row_count: u32,
    pub redaction_class: RedactionClass,
    pub export_safe: bool,
}

/// Metadata-safe support-export envelope folded from the inspector corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageInspectorSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub envelope_id: String,
    pub captured_at: String,
    pub card_schema_ref: String,
    pub breakdown_schema_ref: String,
    pub detail_schema_ref: String,
    pub rebuild_cost_schema_ref: String,
    pub inspector_contract_doc_ref: String,
    pub workspace_detail_contract_doc_ref: String,
    pub card_count: u32,
    pub breakdown_row_count: u32,
    pub detail_row_count: u32,
    pub protected_class_row_count: u32,
    pub raw_content_exported: bool,
    pub redaction_class: String,
    pub cards: Vec<StorageInspectorSupportExportCard>,
}

impl StorageInspectorSupportExport {
    /// Returns true when the envelope is metadata-safe and inspector-complete.
    pub fn is_export_safe(&self) -> bool {
        !self.raw_content_exported
            && self.redaction_class == "metadata_safe_default"
            && self.card_count >= 1
            && self.cards.len() as u32 == self.card_count
            && self
                .cards
                .iter()
                .all(|card| card.export_safe && card.redaction_class.is_export_safe())
    }
}

impl StorageInspectorCorpus {
    /// Projects the corpus into a metadata-safe support-export envelope the
    /// support-bundle pipeline can quote without leaking raw payloads, raw
    /// paths, or raw credential bodies. Cards sort by `card_id`; each card's
    /// class breakdown sorts by storage class and its largest consumers sort by
    /// raw bytes, descending.
    pub fn support_export(
        &self,
        envelope_id: impl Into<String>,
        captured_at: impl Into<String>,
    ) -> StorageInspectorSupportExport {
        // Per-card detail counts, computed once and read deterministically.
        let mut detail_counts: BTreeMap<&str, u32> = BTreeMap::new();
        for entry in &self.detail_rows {
            *detail_counts
                .entry(entry.row.card_id_ref.as_str())
                .or_insert(0) += 1;
        }

        let mut cards: Vec<StorageInspectorSupportExportCard> = self
            .cards
            .iter()
            .map(|entry| {
                let card = &entry.card;
                let class_breakdown: Vec<SupportExportClassRow> = self
                    .class_breakdown_for(&card.card_id)
                    .into_iter()
                    .map(SupportExportClassRow::from_row)
                    .collect();
                let largest_consumers: Vec<SupportExportConsumerRow> = card
                    .top_consumers_by_bytes()
                    .into_iter()
                    .map(SupportExportConsumerRow::from_row)
                    .collect();
                StorageInspectorSupportExportCard {
                    record_kind: STORAGE_INSPECTOR_SUPPORT_EXPORT_CARD_RECORD_KIND.to_owned(),
                    card_id: card.card_id.clone(),
                    scope_class: card.inspector_scope.scope_class,
                    scope_label: card.inspector_scope.scope_label.clone(),
                    total_used_bytes: card.total_used_bytes,
                    quota_basis_class: card.quota_or_policy_source.quota_basis_class,
                    quota_authority_class: card.quota_or_policy_source.quota_authority_class,
                    scan_freshness_class: card.scan_posture.scan_freshness_class,
                    protected_class_visibility: card.protected_class_visibility.clone(),
                    class_breakdown,
                    largest_consumers,
                    detail_row_count: detail_counts
                        .get(card.card_id.as_str())
                        .copied()
                        .unwrap_or(0),
                    redaction_class: card.redaction_class,
                    export_safe: card.export_safe,
                }
            })
            .collect();
        cards.sort_by(|a, b| a.card_id.cmp(&b.card_id));

        let protected_class_row_count = self
            .breakdown_rows
            .iter()
            .filter(|entry| entry.row.is_protected_class())
            .count() as u32;

        StorageInspectorSupportExport {
            record_kind: STORAGE_INSPECTOR_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: STORAGE_INSPECTOR_CARD_SCHEMA_VERSION,
            envelope_id: envelope_id.into(),
            captured_at: captured_at.into(),
            card_schema_ref: STORAGE_INSPECTOR_CARD_SCHEMA_REF.to_owned(),
            breakdown_schema_ref: STORAGE_CLASS_BREAKDOWN_SCHEMA_REF.to_owned(),
            detail_schema_ref: WORKSPACE_STORAGE_DETAIL_SCHEMA_REF.to_owned(),
            rebuild_cost_schema_ref: REBUILD_COST_HINT_SCHEMA_REF.to_owned(),
            inspector_contract_doc_ref: STORAGE_INSPECTOR_CONTRACT_DOC_REF.to_owned(),
            workspace_detail_contract_doc_ref: WORKSPACE_STORAGE_DETAIL_CONTRACT_DOC_REF.to_owned(),
            card_count: self.cards.len() as u32,
            breakdown_row_count: self.breakdown_rows.len() as u32,
            detail_row_count: self.detail_rows.len() as u32,
            protected_class_row_count,
            raw_content_exported: false,
            redaction_class: "metadata_safe_default".to_owned(),
            cards,
        }
    }
}
