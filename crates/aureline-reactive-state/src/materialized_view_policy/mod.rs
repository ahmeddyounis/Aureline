//! Canonical materialized-view-class policy.
//!
//! The reactive-governance matrix in [`crate::m5_reactive_governance`]
//! tags every M5 surface with a materialized-view *class*, but it only
//! computes a minimal lifecycle tuple (persistence, read authority,
//! delete semantics) for the per-surface declaration. That left the
//! richer questions implicit: how long a class is retained, whether it
//! is exportable, what a clear-data sweep does to it, whether it can be
//! held for offboarding, and what — if anything — it contributes to a
//! support bundle.
//!
//! This module freezes one typed policy for the four materialized-view
//! classes so those questions have a single canonical answer instead of
//! being inferred per surface from a storage location:
//!
//! - [`ViewClass::EphemeralProjection`] — memory-only, evicted on scope
//!   change, never exported, never in a bundle.
//! - [`ViewClass::DurableLocalMaterialization`] — a local cache/db that
//!   is cleared and rebuilt from authority, exports metadata only, and
//!   is cleared on offboarding.
//! - [`ViewClass::ExportableSnapshot`] — a saved, user-authored artifact
//!   that survives a clear-data sweep, is itself the export, can be held
//!   for offboarding, and is restored from the saved copy.
//! - [`ViewClass::ManagedReplicatedView`] — a service/local-mirror replica
//!   whose local copy is revoked on clear-data and reconciled from the
//!   managed source, governed by managed retention.
//!
//! The packet carries both the per-class policy rows and the full
//! clear-data / export / support-bundle / offboarding / restore
//! [`disposition_matrix`](MaterializedViewClassPolicy::disposition_matrix),
//! so clear-data, support, offboarding, restore, and release tooling can
//! ingest one table rather than reimplementing class behavior. The
//! [`class_semantics`] and [`disposition_for`] functions are the single
//! source the rows and the matrix are computed from, so neither can
//! drift.
//!
//! The packet is mirrored by:
//!
//! - [`/schemas/state/materialized_view_policy.schema.json`](../../../../schemas/state/materialized_view_policy.schema.json)
//! - [`/docs/state/materialized_view_policy.md`](../../../../docs/state/materialized_view_policy.md)
//! - [`/artifacts/state/materialized_view_policy.json`](../../../../artifacts/state/materialized_view_policy.json)
//! - [`/artifacts/state/materialized_view_policy.md`](../../../../artifacts/state/materialized_view_policy.md)
//! - [`/fixtures/state/materialized_view_policy/`](../../../../fixtures/state/materialized_view_policy/)

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped onto the packet and fixtures.
pub const MATERIALIZED_VIEW_POLICY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the packet.
pub const MATERIALIZED_VIEW_POLICY_PACKET_RECORD_KIND: &str = "materialized_view_policy_record";

/// Stable record-kind tag carried by fixtures.
pub const MATERIALIZED_VIEW_POLICY_FIXTURE_RECORD_KIND: &str =
    "materialized_view_policy_fixture_record";

/// Repo-relative boundary schema ref.
pub const MATERIALIZED_VIEW_POLICY_SCHEMA_REF: &str =
    "schemas/state/materialized_view_policy.schema.json";

/// Repo-relative reviewer doc ref.
pub const MATERIALIZED_VIEW_POLICY_DOC_REF: &str = "docs/state/materialized_view_policy.md";

/// Repo-relative machine-readable artifact packet.
pub const MATERIALIZED_VIEW_POLICY_PACKET_REF: &str =
    "artifacts/state/materialized_view_policy.json";

/// Repo-relative reviewer artifact report.
pub const MATERIALIZED_VIEW_POLICY_REPORT_REF: &str = "artifacts/state/materialized_view_policy.md";

/// Repo-relative fixture directory.
pub const MATERIALIZED_VIEW_POLICY_FIXTURE_DIR: &str = "fixtures/state/materialized_view_policy";

/// Repo-relative fixture manifest.
pub const MATERIALIZED_VIEW_POLICY_FIXTURE_MANIFEST_REF: &str =
    "fixtures/state/materialized_view_policy/manifest.yaml";

// ---------------------------------------------------------------------------
// Vocabulary.
//
// `ViewClass` and `ReadAuthority` mirror the subscription-envelope
// vocabulary in `crate::envelope` token-for-token; the
// `view_class_vocabulary_matches_envelope` test asserts that parity so
// the policy cannot silently fork the ADR vocabulary.
// ---------------------------------------------------------------------------

/// The four materialized-view classes the policy governs. Mirrors
/// Appendix DB.3 and [`crate::envelope::ViewClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ViewClass {
    EphemeralProjection,
    DurableLocalMaterialization,
    ExportableSnapshot,
    ManagedReplicatedView,
}

impl ViewClass {
    /// Stable token mirrored by the schema and [`crate::envelope`].
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EphemeralProjection => "ephemeral_projection",
            Self::DurableLocalMaterialization => "durable_local_materialization",
            Self::ExportableSnapshot => "exportable_snapshot",
            Self::ManagedReplicatedView => "managed_replicated_view",
        }
    }

    /// Every view class, in canonical order.
    pub const fn all() -> [ViewClass; 4] {
        [
            Self::EphemeralProjection,
            Self::DurableLocalMaterialization,
            Self::ExportableSnapshot,
            Self::ManagedReplicatedView,
        ]
    }
}

/// What authority a *read* of a materialized view yields. Every class is
/// a derived projection: a read of a materialized view never presents
/// the owning authority's exact current truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadAuthority {
    /// A read yields a derived projection, never authoritative truth.
    DerivedProjection,
}

impl ReadAuthority {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DerivedProjection => "derived_projection",
        }
    }
}

/// Where a materialized-view class lives. Mirrors the persistence
/// vocabulary in [`crate::m5_reactive_governance`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersistenceClass {
    MemoryOnly,
    LocalCacheOrDb,
    SavedArtifact,
    ServiceOrLocalMirror,
}

impl PersistenceClass {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MemoryOnly => "memory_only",
            Self::LocalCacheOrDb => "local_cache_or_db",
            Self::SavedArtifact => "saved_artifact",
            Self::ServiceOrLocalMirror => "service_or_local_mirror",
        }
    }
}

/// How long a materialized-view class is retained before it is dropped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    /// Held only for the lifetime of the subscription scope.
    UntilScopeChange,
    /// Held in a local cache/db until evicted under pressure or cleared.
    UntilCacheEvictionOrClear,
    /// Held as a saved artifact until the user deletes the artifact.
    UntilArtifactDeleted,
    /// Held per the managed replication lease and its retention policy.
    UntilReplicationLeaseEnds,
}

impl RetentionClass {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UntilScopeChange => "until_scope_change",
            Self::UntilCacheEvictionOrClear => "until_cache_eviction_or_clear",
            Self::UntilArtifactDeleted => "until_artifact_deleted",
            Self::UntilReplicationLeaseEnds => "until_replication_lease_ends",
        }
    }
}

/// Whether and how a materialized-view class may leave the device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportClass {
    /// Never exported; it never leaves memory.
    NotExportable,
    /// Only class/epoch metadata may be exported, never raw payload.
    MetadataOnlyExport,
    /// The saved snapshot artifact is itself the export.
    ExportableSnapshotArtifact,
    /// Only replica metadata is exported; the source stays with the provider.
    ReplicaMetadataExport,
}

impl ExportClass {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotExportable => "not_exportable",
            Self::MetadataOnlyExport => "metadata_only_export",
            Self::ExportableSnapshotArtifact => "exportable_snapshot_artifact",
            Self::ReplicaMetadataExport => "replica_metadata_export",
        }
    }
}

/// What a clear-data sweep does to a materialized-view class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClearDataSemantics {
    /// Evicted from memory on scope change; nothing persisted to clear.
    EvictOnScopeChange,
    /// Cleared, then rebuilt from authoritative inputs on next read.
    ClearOrRebuild,
    /// Preserved: the saved artifact is user-authored and not swept.
    PreserveSavedArtifact,
    /// The local replica is revoked; it reconciles from the managed source.
    RevokeReplicaReconcileOnReconnect,
}

impl ClearDataSemantics {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvictOnScopeChange => "evict_on_scope_change",
            Self::ClearOrRebuild => "clear_or_rebuild",
            Self::PreserveSavedArtifact => "preserve_saved_artifact",
            Self::RevokeReplicaReconcileOnReconnect => "revoke_replica_reconcile_on_reconnect",
        }
    }
}

/// How a materialized-view class behaves under hold / offboarding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldOffboardingSemantics {
    /// Nothing persisted; there is no state to hold.
    NoPersistedStateToHold,
    /// The local cache is cleared on offboarding.
    ClearedOnOffboarding,
    /// The saved artifact can be retained under a legal/export hold.
    RetainableUnderHold,
    /// Hold and offboarding follow the managed retention policy.
    GovernedByManagedRetention,
}

impl HoldOffboardingSemantics {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPersistedStateToHold => "no_persisted_state_to_hold",
            Self::ClearedOnOffboarding => "cleared_on_offboarding",
            Self::RetainableUnderHold => "retainable_under_hold",
            Self::GovernedByManagedRetention => "governed_by_managed_retention",
        }
    }
}

/// What, if anything, a materialized-view class contributes to a support
/// bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportBundleSemantics {
    /// Never included in a support bundle.
    ExcludedFromBundle,
    /// Class/epoch metadata only; never raw payload.
    MetadataSafeInBundle,
    /// The snapshot artifact may be attached with explicit consent.
    SnapshotEligibleWithConsent,
    /// Replica metadata and posture only; never the replicated payload.
    ReplicaMetadataInBundle,
}

impl SupportBundleSemantics {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExcludedFromBundle => "excluded_from_bundle",
            Self::MetadataSafeInBundle => "metadata_safe_in_bundle",
            Self::SnapshotEligibleWithConsent => "snapshot_eligible_with_consent",
            Self::ReplicaMetadataInBundle => "replica_metadata_in_bundle",
        }
    }
}

/// One lifecycle operation a flow can apply to a materialized view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleOperation {
    ClearData,
    Export,
    SupportBundle,
    Offboarding,
    Restore,
}

impl LifecycleOperation {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClearData => "clear_data",
            Self::Export => "export",
            Self::SupportBundle => "support_bundle",
            Self::Offboarding => "offboarding",
            Self::Restore => "restore",
        }
    }

    /// Every lifecycle operation, in canonical order.
    pub const fn all() -> [LifecycleOperation; 5] {
        [
            Self::ClearData,
            Self::Export,
            Self::SupportBundle,
            Self::Offboarding,
            Self::Restore,
        ]
    }
}

/// The concrete disposition of a materialized-view class under one
/// lifecycle operation. This is the closed vocabulary clear-data,
/// support, offboarding, restore, and export flows quote.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Disposition {
    // clear-data outcomes
    EvictedFromMemory,
    ClearedRebuildableFromAuthority,
    SavedArtifactPreserved,
    LocalReplicaRevokedReconcileLater,
    // export outcomes
    ExcludedNoPersistedState,
    MetadataOnlyExported,
    SnapshotArtifactExported,
    ReplicaMetadataExported,
    // support-bundle outcomes
    ExcludedFromBundle,
    MetadataSafeInBundle,
    SnapshotEligibleWithConsent,
    ReplicaMetadataInBundle,
    // offboarding outcomes
    NothingToHold,
    LocalCacheClearedOnOffboarding,
    RetainedUnderHold,
    GovernedByManagedRetention,
    // restore outcomes
    RebuiltFromAuthority,
    RestoredFromSavedArtifact,
    ReconciledFromManagedSource,
}

impl Disposition {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvictedFromMemory => "evicted_from_memory",
            Self::ClearedRebuildableFromAuthority => "cleared_rebuildable_from_authority",
            Self::SavedArtifactPreserved => "saved_artifact_preserved",
            Self::LocalReplicaRevokedReconcileLater => "local_replica_revoked_reconcile_later",
            Self::ExcludedNoPersistedState => "excluded_no_persisted_state",
            Self::MetadataOnlyExported => "metadata_only_exported",
            Self::SnapshotArtifactExported => "snapshot_artifact_exported",
            Self::ReplicaMetadataExported => "replica_metadata_exported",
            Self::ExcludedFromBundle => "excluded_from_bundle",
            Self::MetadataSafeInBundle => "metadata_safe_in_bundle",
            Self::SnapshotEligibleWithConsent => "snapshot_eligible_with_consent",
            Self::ReplicaMetadataInBundle => "replica_metadata_in_bundle",
            Self::NothingToHold => "nothing_to_hold",
            Self::LocalCacheClearedOnOffboarding => "local_cache_cleared_on_offboarding",
            Self::RetainedUnderHold => "retained_under_hold",
            Self::GovernedByManagedRetention => "governed_by_managed_retention",
            Self::RebuiltFromAuthority => "rebuilt_from_authority",
            Self::RestoredFromSavedArtifact => "restored_from_saved_artifact",
            Self::ReconciledFromManagedSource => "reconciled_from_managed_source",
        }
    }
}

// ---------------------------------------------------------------------------
// Canonical semantics — the single source rows and the matrix derive from.
// ---------------------------------------------------------------------------

/// The frozen lifecycle semantics for a materialized-view class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClassSemantics {
    /// Authority a read yields.
    pub authority_on_read: ReadAuthority,
    /// Where the class lives.
    pub persistence: PersistenceClass,
    /// How long the class is retained.
    pub retention: RetentionClass,
    /// Whether / how the class may be exported.
    pub export: ExportClass,
    /// What clear-data does to the class.
    pub delete_semantics: ClearDataSemantics,
    /// How the class behaves under hold / offboarding.
    pub hold_offboarding: HoldOffboardingSemantics,
    /// What the class contributes to a support bundle.
    pub support_bundle: SupportBundleSemantics,
    /// Whether the class can be rebuilt from authoritative inputs.
    pub rebuildable_from_authority: bool,
    /// Whether persisted state survives a clear-data sweep.
    pub survives_clear_data: bool,
}

/// Returns the canonical lifecycle semantics for a materialized-view
/// class. This is the single source the policy rows are computed from,
/// so the rows can never drift from one another.
///
/// The guardrail is structural: the exportable-snapshot and
/// managed-replicated classes deliberately use distinct retention,
/// delete, hold, export, and support-bundle tokens from the ephemeral
/// class, so a managed or exportable view can never silently inherit
/// ephemeral eviction or retention behavior.
pub const fn class_semantics(view_class: ViewClass) -> ClassSemantics {
    match view_class {
        ViewClass::EphemeralProjection => ClassSemantics {
            authority_on_read: ReadAuthority::DerivedProjection,
            persistence: PersistenceClass::MemoryOnly,
            retention: RetentionClass::UntilScopeChange,
            export: ExportClass::NotExportable,
            delete_semantics: ClearDataSemantics::EvictOnScopeChange,
            hold_offboarding: HoldOffboardingSemantics::NoPersistedStateToHold,
            support_bundle: SupportBundleSemantics::ExcludedFromBundle,
            rebuildable_from_authority: true,
            survives_clear_data: false,
        },
        ViewClass::DurableLocalMaterialization => ClassSemantics {
            authority_on_read: ReadAuthority::DerivedProjection,
            persistence: PersistenceClass::LocalCacheOrDb,
            retention: RetentionClass::UntilCacheEvictionOrClear,
            export: ExportClass::MetadataOnlyExport,
            delete_semantics: ClearDataSemantics::ClearOrRebuild,
            hold_offboarding: HoldOffboardingSemantics::ClearedOnOffboarding,
            support_bundle: SupportBundleSemantics::MetadataSafeInBundle,
            rebuildable_from_authority: true,
            survives_clear_data: false,
        },
        ViewClass::ExportableSnapshot => ClassSemantics {
            authority_on_read: ReadAuthority::DerivedProjection,
            persistence: PersistenceClass::SavedArtifact,
            retention: RetentionClass::UntilArtifactDeleted,
            export: ExportClass::ExportableSnapshotArtifact,
            delete_semantics: ClearDataSemantics::PreserveSavedArtifact,
            hold_offboarding: HoldOffboardingSemantics::RetainableUnderHold,
            support_bundle: SupportBundleSemantics::SnapshotEligibleWithConsent,
            rebuildable_from_authority: false,
            survives_clear_data: true,
        },
        ViewClass::ManagedReplicatedView => ClassSemantics {
            authority_on_read: ReadAuthority::DerivedProjection,
            persistence: PersistenceClass::ServiceOrLocalMirror,
            retention: RetentionClass::UntilReplicationLeaseEnds,
            export: ExportClass::ReplicaMetadataExport,
            delete_semantics: ClearDataSemantics::RevokeReplicaReconcileOnReconnect,
            hold_offboarding: HoldOffboardingSemantics::GovernedByManagedRetention,
            support_bundle: SupportBundleSemantics::ReplicaMetadataInBundle,
            rebuildable_from_authority: true,
            survives_clear_data: false,
        },
    }
}

/// Returns the concrete disposition of a materialized-view class under
/// one lifecycle operation. This is the single source the disposition
/// matrix and every fixture are computed from.
pub const fn disposition_for(view_class: ViewClass, operation: LifecycleOperation) -> Disposition {
    match (view_class, operation) {
        // clear-data
        (ViewClass::EphemeralProjection, LifecycleOperation::ClearData) => {
            Disposition::EvictedFromMemory
        }
        (ViewClass::DurableLocalMaterialization, LifecycleOperation::ClearData) => {
            Disposition::ClearedRebuildableFromAuthority
        }
        (ViewClass::ExportableSnapshot, LifecycleOperation::ClearData) => {
            Disposition::SavedArtifactPreserved
        }
        (ViewClass::ManagedReplicatedView, LifecycleOperation::ClearData) => {
            Disposition::LocalReplicaRevokedReconcileLater
        }
        // export
        (ViewClass::EphemeralProjection, LifecycleOperation::Export) => {
            Disposition::ExcludedNoPersistedState
        }
        (ViewClass::DurableLocalMaterialization, LifecycleOperation::Export) => {
            Disposition::MetadataOnlyExported
        }
        (ViewClass::ExportableSnapshot, LifecycleOperation::Export) => {
            Disposition::SnapshotArtifactExported
        }
        (ViewClass::ManagedReplicatedView, LifecycleOperation::Export) => {
            Disposition::ReplicaMetadataExported
        }
        // support bundle
        (ViewClass::EphemeralProjection, LifecycleOperation::SupportBundle) => {
            Disposition::ExcludedFromBundle
        }
        (ViewClass::DurableLocalMaterialization, LifecycleOperation::SupportBundle) => {
            Disposition::MetadataSafeInBundle
        }
        (ViewClass::ExportableSnapshot, LifecycleOperation::SupportBundle) => {
            Disposition::SnapshotEligibleWithConsent
        }
        (ViewClass::ManagedReplicatedView, LifecycleOperation::SupportBundle) => {
            Disposition::ReplicaMetadataInBundle
        }
        // offboarding
        (ViewClass::EphemeralProjection, LifecycleOperation::Offboarding) => {
            Disposition::NothingToHold
        }
        (ViewClass::DurableLocalMaterialization, LifecycleOperation::Offboarding) => {
            Disposition::LocalCacheClearedOnOffboarding
        }
        (ViewClass::ExportableSnapshot, LifecycleOperation::Offboarding) => {
            Disposition::RetainedUnderHold
        }
        (ViewClass::ManagedReplicatedView, LifecycleOperation::Offboarding) => {
            Disposition::GovernedByManagedRetention
        }
        // restore
        (ViewClass::EphemeralProjection, LifecycleOperation::Restore) => {
            Disposition::RebuiltFromAuthority
        }
        (ViewClass::DurableLocalMaterialization, LifecycleOperation::Restore) => {
            Disposition::RebuiltFromAuthority
        }
        (ViewClass::ExportableSnapshot, LifecycleOperation::Restore) => {
            Disposition::RestoredFromSavedArtifact
        }
        (ViewClass::ManagedReplicatedView, LifecycleOperation::Restore) => {
            Disposition::ReconciledFromManagedSource
        }
    }
}

// ---------------------------------------------------------------------------
// Packet structures.
// ---------------------------------------------------------------------------

/// One materialized-view class governed by the policy, with every
/// lifecycle binding made explicit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializedViewClassRow {
    /// The materialized-view class.
    pub view_class: ViewClass,
    /// One-sentence reviewer summary of the class.
    pub summary: String,
    /// Authority a read yields (always a derived projection).
    pub authority_on_read: ReadAuthority,
    /// Where the class lives.
    pub persistence: PersistenceClass,
    /// How long the class is retained.
    pub retention: RetentionClass,
    /// Whether / how the class may be exported.
    pub export: ExportClass,
    /// What clear-data does to the class.
    pub delete_semantics: ClearDataSemantics,
    /// How the class behaves under hold / offboarding.
    pub hold_offboarding: HoldOffboardingSemantics,
    /// What the class contributes to a support bundle.
    pub support_bundle: SupportBundleSemantics,
    /// Whether the class can be rebuilt from authoritative inputs.
    pub rebuildable_from_authority: bool,
    /// Whether persisted state survives a clear-data sweep.
    pub survives_clear_data: bool,
    /// Representative reactive surfaces backed by this class.
    pub example_surfaces: Vec<String>,
    /// Real consumer surfaces that ingest this policy row.
    pub consumer_refs: Vec<String>,
    /// Short reviewer note.
    pub notes: String,
}

/// One cell of the clear-data / export / support-bundle / offboarding /
/// restore disposition matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DispositionRow {
    /// The materialized-view class.
    pub view_class: ViewClass,
    /// The lifecycle operation applied.
    pub operation: LifecycleOperation,
    /// The concrete disposition.
    pub disposition: Disposition,
    /// One-sentence rationale for the disposition.
    pub rationale: String,
}

/// Shared source references for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceContractRefs {
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Packet ref.
    pub packet_ref: String,
    /// Report ref.
    pub report_ref: String,
    /// Fixture manifest ref.
    pub fixture_manifest_ref: String,
}

/// Top-level packet freezing the materialized-view-class policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializedViewClassPolicy {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer title.
    pub title: String,
    /// Shared refs.
    pub source_contract_refs: SourceContractRefs,
    /// One policy row per materialized-view class.
    pub classes: Vec<MaterializedViewClassRow>,
    /// Full per-class disposition matrix over the lifecycle operations.
    pub disposition_matrix: Vec<DispositionRow>,
    /// Short invariant summary.
    pub invariants: Vec<String>,
}

/// One fixture binding a class and a lifecycle operation to the expected
/// disposition, proving the canonical policy behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializedViewClassPolicyFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Class under test.
    pub view_class: ViewClass,
    /// Lifecycle operation under test.
    pub operation: LifecycleOperation,
    /// Expected disposition.
    pub expected_disposition: Disposition,
    /// One consumer that quotes this policy.
    pub consumer_ref: String,
    /// Short reviewer note.
    pub notes: String,
}

/// One validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationViolation {
    /// Stable check id.
    pub check_id: &'static str,
    /// Human-readable explanation.
    pub message: String,
}

/// Validation report for the packet or fixtures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    /// All detected violations.
    pub violations: Vec<ValidationViolation>,
}

impl ValidationReport {
    fn push(&mut self, check_id: &'static str, message: impl Into<String>) {
        self.violations.push(ValidationViolation {
            check_id,
            message: message.into(),
        });
    }

    fn is_empty(&self) -> bool {
        self.violations.is_empty()
    }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "materialized view policy validation failed")?;
        for violation in &self.violations {
            writeln!(f, "- {}: {}", violation.check_id, violation.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationReport {}

// ---------------------------------------------------------------------------
// Seeded packet.
// ---------------------------------------------------------------------------

/// Returns the checked-in materialized-view-class policy this lane
/// freezes.
pub fn seeded_materialized_view_policy() -> MaterializedViewClassPolicy {
    let classes = ViewClass::all().into_iter().map(class_row).collect();
    let disposition_matrix = disposition_matrix();

    MaterializedViewClassPolicy {
        record_kind: MATERIALIZED_VIEW_POLICY_PACKET_RECORD_KIND.to_owned(),
        schema_version: MATERIALIZED_VIEW_POLICY_SCHEMA_VERSION,
        packet_id: "state.materialized_view_policy.v1".to_owned(),
        title: "Canonical materialized-view-class persistence, export, and delete policy".to_owned(),
        source_contract_refs: SourceContractRefs {
            doc_ref: MATERIALIZED_VIEW_POLICY_DOC_REF.to_owned(),
            schema_ref: MATERIALIZED_VIEW_POLICY_SCHEMA_REF.to_owned(),
            packet_ref: MATERIALIZED_VIEW_POLICY_PACKET_REF.to_owned(),
            report_ref: MATERIALIZED_VIEW_POLICY_REPORT_REF.to_owned(),
            fixture_manifest_ref: MATERIALIZED_VIEW_POLICY_FIXTURE_MANIFEST_REF.to_owned(),
        },
        classes,
        disposition_matrix,
        invariants: vec![
            "Every materialized view declares one of four classes (ephemeral projection, durable local materialization, exportable snapshot, managed replicated view); persistence, retention, export, delete, hold/offboarding, and support-bundle behavior follow from the class, never from a storage location alone.".to_owned(),
            "No materialized-view class presents exact current truth on read; every read is a derived projection of an authority.".to_owned(),
            "Clear-data, export, support-bundle, offboarding, and restore behavior is fixed per class in one disposition matrix, so each flow ingests the same table instead of reimplementing class behavior.".to_owned(),
            "Managed-replicated and exportable-snapshot classes carry distinct retention, delete, hold, export, and support-bundle semantics from the ephemeral class; neither can silently inherit ephemeral eviction or retention.".to_owned(),
            "Exportable snapshots are user-authored artifacts: a clear-data sweep preserves them and they are restored from the saved copy, never rebuilt from authority.".to_owned(),
        ],
    }
}

/// Returns the checked-in fixture rows this lane freezes — a curated
/// subset of the disposition matrix that covers every class, every
/// operation, and the no-ephemeral-inheritance guardrail.
pub fn seeded_materialized_view_policy_fixtures() -> Vec<MaterializedViewClassPolicyFixture> {
    vec![
        fixture(
            "fixture:mv_policy:ephemeral_clear_data",
            ViewClass::EphemeralProjection,
            LifecycleOperation::ClearData,
            "crates/aureline-support/src/m5_clear_data_review/mod.rs",
            "Clearing data on an ephemeral projection evicts it from memory; there is no persisted copy to sweep.",
        ),
        fixture(
            "fixture:mv_policy:ephemeral_offboarding",
            ViewClass::EphemeralProjection,
            LifecycleOperation::Offboarding,
            "crates/aureline-support/src/m5_offboarding_continuity/mod.rs",
            "Offboarding an ephemeral projection holds nothing because no state is persisted.",
        ),
        fixture(
            "fixture:mv_policy:durable_clear_data",
            ViewClass::DurableLocalMaterialization,
            LifecycleOperation::ClearData,
            "crates/aureline-support/src/m5_clear_data_review/mod.rs",
            "Clearing data on a durable local materialization clears the cache; it is rebuildable from authority on next read.",
        ),
        fixture(
            "fixture:mv_policy:durable_support_bundle",
            ViewClass::DurableLocalMaterialization,
            LifecycleOperation::SupportBundle,
            "crates/aureline-support/src/materialized_view_policy/mod.rs",
            "A durable local materialization contributes metadata-safe class/epoch state to a support bundle, never raw payload.",
        ),
        fixture(
            "fixture:mv_policy:durable_restore",
            ViewClass::DurableLocalMaterialization,
            LifecycleOperation::Restore,
            "crates/aureline-support/src/records_export_delete_governance/mod.rs",
            "A durable local materialization is rebuilt from authority on restore rather than byte-restored from a saved copy.",
        ),
        fixture(
            "fixture:mv_policy:exportable_export",
            ViewClass::ExportableSnapshot,
            LifecycleOperation::Export,
            "crates/aureline-support/src/m5_subscription_export/mod.rs",
            "An exportable snapshot is itself the export artifact.",
        ),
        fixture(
            "fixture:mv_policy:exportable_clear_data",
            ViewClass::ExportableSnapshot,
            LifecycleOperation::ClearData,
            "crates/aureline-support/src/m5_clear_data_review/mod.rs",
            "A clear-data sweep preserves an exportable snapshot: the saved artifact is user-authored and not evicted like an ephemeral cache.",
        ),
        fixture(
            "fixture:mv_policy:exportable_offboarding",
            ViewClass::ExportableSnapshot,
            LifecycleOperation::Offboarding,
            "crates/aureline-support/src/m5_offboarding_continuity/mod.rs",
            "An exportable snapshot can be retained under a legal/export hold during offboarding.",
        ),
        fixture(
            "fixture:mv_policy:exportable_restore",
            ViewClass::ExportableSnapshot,
            LifecycleOperation::Restore,
            "crates/aureline-support/src/records_export_delete_governance/mod.rs",
            "An exportable snapshot is restored from its saved artifact, never rebuilt from authority.",
        ),
        fixture(
            "fixture:mv_policy:managed_clear_data",
            ViewClass::ManagedReplicatedView,
            LifecycleOperation::ClearData,
            "crates/aureline-support/src/m5_clear_data_review/mod.rs",
            "Clearing data on a managed replicated view revokes the local replica; it reconciles from the managed source on reconnect.",
        ),
        fixture(
            "fixture:mv_policy:managed_offboarding",
            ViewClass::ManagedReplicatedView,
            LifecycleOperation::Offboarding,
            "crates/aureline-support/src/m5_offboarding_continuity/mod.rs",
            "Offboarding a managed replicated view follows the managed retention policy, not local cache rules.",
        ),
        fixture(
            "fixture:mv_policy:managed_support_bundle",
            ViewClass::ManagedReplicatedView,
            LifecycleOperation::SupportBundle,
            "crates/aureline-support/src/materialized_view_policy/mod.rs",
            "A managed replicated view contributes replica metadata and posture to a support bundle, never the replicated payload.",
        ),
    ]
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

/// Validates the seeded packet or an on-disk copy of it.
pub fn validate_materialized_view_policy(
    packet: &MaterializedViewClassPolicy,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != MATERIALIZED_VIEW_POLICY_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            format!(
                "record_kind must be {MATERIALIZED_VIEW_POLICY_PACKET_RECORD_KIND}, got {}",
                packet.record_kind
            ),
        );
    }
    if packet.schema_version != MATERIALIZED_VIEW_POLICY_SCHEMA_VERSION {
        report.push(
            "packet.schema_version",
            format!(
                "schema_version must be {}, got {}",
                MATERIALIZED_VIEW_POLICY_SCHEMA_VERSION, packet.schema_version
            ),
        );
    }
    if packet.source_contract_refs.doc_ref != MATERIALIZED_VIEW_POLICY_DOC_REF {
        report.push("packet.doc_ref", "doc_ref drifted");
    }
    if packet.source_contract_refs.schema_ref != MATERIALIZED_VIEW_POLICY_SCHEMA_REF {
        report.push("packet.schema_ref", "schema_ref drifted");
    }
    if packet.source_contract_refs.packet_ref != MATERIALIZED_VIEW_POLICY_PACKET_REF {
        report.push("packet.packet_ref", "packet_ref drifted");
    }
    if packet.source_contract_refs.report_ref != MATERIALIZED_VIEW_POLICY_REPORT_REF {
        report.push("packet.report_ref", "report_ref drifted");
    }
    if packet.source_contract_refs.fixture_manifest_ref
        != MATERIALIZED_VIEW_POLICY_FIXTURE_MANIFEST_REF
    {
        report.push(
            "packet.fixture_manifest_ref",
            "fixture_manifest_ref drifted",
        );
    }

    validate_classes(packet, &mut report);
    validate_disposition_matrix(packet, &mut report);

    if packet.invariants.iter().all(|inv| inv.trim().is_empty()) {
        report.push("packet.invariants", "invariants must be non-empty");
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

fn validate_classes(packet: &MaterializedViewClassPolicy, report: &mut ValidationReport) {
    // Exactly one row per class, in canonical order, computed from the
    // single source so it cannot drift.
    let expected: Vec<MaterializedViewClassRow> =
        ViewClass::all().into_iter().map(class_row).collect();
    if packet.classes != expected {
        report.push(
            "packet.classes",
            "class rows drifted from the canonical class semantics",
        );
    }

    let mut seen = BTreeSet::new();
    for row in &packet.classes {
        if !seen.insert(row.view_class) {
            report.push(
                "class.duplicate",
                format!("duplicate class row {}", row.view_class.as_str()),
            );
        }
        if row.example_surfaces.is_empty() {
            report.push(
                "class.example_surfaces",
                format!(
                    "class {} must list a representative surface",
                    row.view_class.as_str()
                ),
            );
        }
        if row.consumer_refs.is_empty() {
            report.push(
                "class.consumer_refs",
                format!(
                    "class {} must carry a consumer ref",
                    row.view_class.as_str()
                ),
            );
        }
        if row.authority_on_read != ReadAuthority::DerivedProjection {
            report.push(
                "class.authority_on_read",
                format!(
                    "class {} read must be a derived projection",
                    row.view_class.as_str()
                ),
            );
        }
    }

    for required in ViewClass::all() {
        if !seen.contains(&required) {
            report.push(
                "packet.class_missing",
                format!("policy must cover class {}", required.as_str()),
            );
        }
    }

    // Guardrail: managed-replicated and exportable-snapshot classes must
    // not inherit ephemeral retention / delete / hold / export /
    // support-bundle tokens.
    let ephemeral = class_semantics(ViewClass::EphemeralProjection);
    for guarded in [
        ViewClass::ExportableSnapshot,
        ViewClass::ManagedReplicatedView,
    ] {
        let sem = class_semantics(guarded);
        if sem.retention == ephemeral.retention
            || sem.delete_semantics == ephemeral.delete_semantics
            || sem.hold_offboarding == ephemeral.hold_offboarding
            || sem.export == ephemeral.export
            || sem.support_bundle == ephemeral.support_bundle
        {
            report.push(
                "class.ephemeral_inheritance",
                format!(
                    "class {} must not inherit ephemeral retention/delete/hold/export/support semantics",
                    guarded.as_str()
                ),
            );
        }
    }
}

fn validate_disposition_matrix(
    packet: &MaterializedViewClassPolicy,
    report: &mut ValidationReport,
) {
    let expected = disposition_matrix();
    if packet.disposition_matrix != expected {
        report.push(
            "packet.disposition_matrix",
            "disposition matrix drifted from the canonical disposition_for mapping",
        );
    }

    let mut seen = BTreeSet::new();
    for row in &packet.disposition_matrix {
        if !seen.insert((row.view_class, row.operation)) {
            report.push(
                "disposition.duplicate",
                format!(
                    "duplicate disposition for {} / {}",
                    row.view_class.as_str(),
                    row.operation.as_str()
                ),
            );
        }
        let expected_disposition = disposition_for(row.view_class, row.operation);
        if row.disposition != expected_disposition {
            report.push(
                "disposition.mismatch",
                format!(
                    "disposition for {} / {} must be {}, got {}",
                    row.view_class.as_str(),
                    row.operation.as_str(),
                    expected_disposition.as_str(),
                    row.disposition.as_str()
                ),
            );
        }
    }

    // Clear-data must distinguish every class: no two classes share a
    // clear-data disposition, so a flow cannot infer behavior from
    // storage location alone.
    let clear_data: Vec<Disposition> = ViewClass::all()
        .into_iter()
        .map(|vc| disposition_for(vc, LifecycleOperation::ClearData))
        .collect();
    let distinct: BTreeSet<_> = clear_data.iter().copied().collect();
    if distinct.len() != clear_data.len() {
        report.push(
            "disposition.clear_data_collision",
            "every class must have a distinct clear-data disposition",
        );
    }
}

/// Validates one fixture against the packet.
pub fn validate_materialized_view_policy_fixture(
    packet: &MaterializedViewClassPolicy,
    fixture: &MaterializedViewClassPolicyFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };
    if fixture.record_kind != MATERIALIZED_VIEW_POLICY_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            format!(
                "fixture {} record_kind must be {}",
                fixture.fixture_id, MATERIALIZED_VIEW_POLICY_FIXTURE_RECORD_KIND
            ),
        );
    }
    if fixture.schema_version != MATERIALIZED_VIEW_POLICY_SCHEMA_VERSION {
        report.push(
            "fixture.schema_version",
            format!(
                "fixture {} schema_version must be {}",
                fixture.fixture_id, MATERIALIZED_VIEW_POLICY_SCHEMA_VERSION
            ),
        );
    }

    let expected = disposition_for(fixture.view_class, fixture.operation);
    if fixture.expected_disposition != expected {
        report.push(
            "fixture.expected_disposition",
            format!(
                "fixture {} expected {} but policy yields {}",
                fixture.fixture_id,
                fixture.expected_disposition.as_str(),
                expected.as_str()
            ),
        );
    }

    let Some(row) = packet
        .classes
        .iter()
        .find(|row| row.view_class == fixture.view_class)
    else {
        report.push(
            "fixture.class_missing",
            format!(
                "fixture {} points to class {} missing from the policy",
                fixture.fixture_id,
                fixture.view_class.as_str()
            ),
        );
        return Err(report);
    };

    if !row.consumer_refs.iter().any(|c| c == &fixture.consumer_ref) {
        report.push(
            "fixture.consumer_ref",
            format!(
                "fixture {} consumer_ref {} must be declared by class {}",
                fixture.fixture_id,
                fixture.consumer_ref,
                row.view_class.as_str()
            ),
        );
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

// ---------------------------------------------------------------------------
// Builders / helpers (all deterministic, all computed from one source).
// ---------------------------------------------------------------------------

fn class_row(view_class: ViewClass) -> MaterializedViewClassRow {
    let sem = class_semantics(view_class);
    let (summary, example_surfaces, consumer_refs, notes) = class_descriptors(view_class);
    MaterializedViewClassRow {
        view_class,
        summary: summary.to_owned(),
        authority_on_read: sem.authority_on_read,
        persistence: sem.persistence,
        retention: sem.retention,
        export: sem.export,
        delete_semantics: sem.delete_semantics,
        hold_offboarding: sem.hold_offboarding,
        support_bundle: sem.support_bundle,
        rebuildable_from_authority: sem.rebuildable_from_authority,
        survives_clear_data: sem.survives_clear_data,
        example_surfaces: example_surfaces.iter().map(|s| (*s).to_owned()).collect(),
        consumer_refs: consumer_refs.iter().map(|s| (*s).to_owned()).collect(),
        notes: notes.to_owned(),
    }
}

fn class_descriptors(
    view_class: ViewClass,
) -> (
    &'static str,
    &'static [&'static str],
    &'static [&'static str],
    &'static str,
) {
    match view_class {
        ViewClass::EphemeralProjection => (
            "A memory-only derived projection rebuilt on demand and evicted when its scope changes.",
            &["editor_buffer_outline", "graph_neighborhood", "ai_context_panel"],
            &[
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
                "crates/aureline-support/src/materialized_view_policy/mod.rs",
                "crates/aureline-support/src/m5_clear_data_review/mod.rs",
                "crates/aureline-support/src/m5_offboarding_continuity/mod.rs",
            ],
            "Holds nothing across a clear-data sweep; never exported and never placed in a support bundle.",
        ),
        ViewClass::DurableLocalMaterialization => (
            "A local cache/db materialization cleared and rebuilt from authoritative inputs.",
            &["shell_workspace_tree", "search_results", "docs_browser"],
            &[
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
                "crates/aureline-support/src/materialized_view_policy/mod.rs",
                "crates/aureline-support/src/m5_clear_data_review/mod.rs",
                "crates/aureline-support/src/records_export_delete_governance/mod.rs",
            ],
            "Clear-data drops the cache; the view rebuilds from authority and exports metadata only.",
        ),
        ViewClass::ExportableSnapshot => (
            "A saved, user-authored snapshot artifact captured at a point in time.",
            &["preview_output", "support_export_view"],
            &[
                "crates/aureline-support/src/materialized_view_policy/mod.rs",
                "crates/aureline-support/src/m5_clear_data_review/mod.rs",
                "crates/aureline-support/src/m5_subscription_export/mod.rs",
                "crates/aureline-support/src/records_export_delete_governance/mod.rs",
                "crates/aureline-support/src/m5_offboarding_continuity/mod.rs",
            ],
            "Survives a clear-data sweep, can be held for offboarding, and is restored from the saved copy, not rebuilt from authority.",
        ),
        ViewClass::ManagedReplicatedView => (
            "A service-backed or locally mirrored replica reconciled from a managed source.",
            &["review_workspace", "companion_panel"],
            &[
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
                "crates/aureline-support/src/materialized_view_policy/mod.rs",
                "crates/aureline-support/src/m5_clear_data_review/mod.rs",
                "crates/aureline-support/src/m5_offboarding_continuity/mod.rs",
            ],
            "Clear-data revokes the local replica; hold and offboarding follow the managed retention policy.",
        ),
    }
}

fn disposition_matrix() -> Vec<DispositionRow> {
    let mut rows = Vec::new();
    for view_class in ViewClass::all() {
        for operation in LifecycleOperation::all() {
            let disposition = disposition_for(view_class, operation);
            rows.push(DispositionRow {
                view_class,
                operation,
                disposition,
                rationale: disposition_rationale(view_class, operation, disposition),
            });
        }
    }
    rows
}

fn disposition_rationale(
    view_class: ViewClass,
    operation: LifecycleOperation,
    disposition: Disposition,
) -> String {
    format!(
        "{} under {} resolves to {} by class policy.",
        view_class.as_str(),
        operation.as_str(),
        disposition.as_str()
    )
}

fn fixture(
    fixture_id: &str,
    view_class: ViewClass,
    operation: LifecycleOperation,
    consumer_ref: &str,
    notes: &str,
) -> MaterializedViewClassPolicyFixture {
    MaterializedViewClassPolicyFixture {
        record_kind: MATERIALIZED_VIEW_POLICY_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: MATERIALIZED_VIEW_POLICY_SCHEMA_VERSION,
        fixture_id: fixture_id.to_owned(),
        view_class,
        operation,
        expected_disposition: disposition_for(view_class, operation),
        consumer_ref: consumer_ref.to_owned(),
        notes: notes.to_owned(),
    }
}

#[cfg(test)]
mod tests;
