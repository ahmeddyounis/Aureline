//! Canonical M5 workspace-serialization and restore-fidelity matrix: the single machine-readable
//! contract for everything M5 is allowed to remember, export, compare, restore exactly, restore
//! compatibly, or only reopen as context.
//!
//! M5 keeps adding restorable surfaces — preview routes, notebook sessions, query consoles,
//! profiler captures, docs panes, incident workspaces, companion handoff packets, and
//! portable-state artifacts — and each of them remembers some slice of workspace state. Left
//! implicit, every surface invents its own restore language. This packet replaces that with one
//! controlled matrix:
//!
//! - [`RememberedArtifactClass`] enumerates the six remembered-state artifact classes M5 may
//!   persist (workspace-authority checkpoints, window-topology snapshots, portable-state
//!   packages, restore-provenance records, placeholder cards, and compare/export summaries).
//! - [`RestoreFidelityClass`] defines the four restore-fidelity classes —
//!   [`RestoreFidelityClass::ExactRestore`], [`RestoreFidelityClass::CompatibleRestore`],
//!   [`RestoreFidelityClass::LayoutOnly`], and [`RestoreFidelityClass::ManualReview`] — and the
//!   schema, dependency, topology, and freshness conditions that downgrade between them.
//! - [`OwnershipClass`] binds each artifact class to portable, shared, local, or machine-local
//!   ownership, and [`RedactionExclusion`] records what each class excludes.
//! - [`RestorableSurface`] classifies every M5 restorable surface by what it persists, how
//!   portable it is, and which restore-fidelity classes it supports.
//!
//! The matrix is fail-closed. The achieved restore fidelity of a row is the **weakest ceiling**
//! implied by the row's declared maximum, its schema condition, its dependency condition, its
//! topology condition, and its evidence freshness ([`ArtifactClassRow::achieved_fidelity`]), so a
//! schema drift, a missing dependency, a changed topology, or stale evidence narrows the restore
//! claim automatically rather than leaving a surface claiming exact restore by inertia. A missing
//! dependency never silently deletes layout: a row whose dependency condition is unmet downgrades
//! to a slot-preserving placeholder ([`MissingDependencyBehavior`]), and
//! [`MissingDependencyBehavior::SilentDelete`] exists in the vocabulary only so the gate can
//! reject it outright.
//!
//! Portability is claimed, never assumed. A row that exports into a portable-state package must
//! exclude secrets, live authority, and machine-local anchors ([`RedactionExclusion`]); a
//! machine-local row is never exportable. This is the guardrail the spec demands: portable-state
//! packages never serialize live authority or hidden machine-local secrets, and layout restore,
//! portable-state export, and crash-recovery evidence are kept distinct rather than treated as
//! equivalent because they share artifacts.
//!
//! Every continuity surface that reuses remembered state — crash recovery, browser/companion
//! handoff, import/export, and claim publication — is cross-linked to this matrix
//! ([`ContinuityCrossLink`]) so restore language stays canonical, and every required reviewer
//! surface — shiproom, docs/help, and support export — binds to this one packet
//! ([`MatrixConsumerBinding`]) and narrows with it.
//!
//! The packet is checked in at `artifacts/workspace/m5/m5-serialization-and-restore-matrix.json`
//! and embedded here. It is metadata-only: every field is a typed state, a count, or an opaque
//! ref, and it carries no credential bodies, raw provider payloads, live authority handles, or
//! workspace contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported M5 serialization-and-restore matrix schema version.
pub const M5_SERIALIZATION_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_SERIALIZATION_MATRIX_RECORD_KIND: &str = "m5_serialization_and_restore_matrix";

/// Repo-relative path to the checked-in packet.
pub const M5_SERIALIZATION_MATRIX_PATH: &str =
    "artifacts/workspace/m5/m5-serialization-and-restore-matrix.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_SERIALIZATION_MATRIX_SCHEMA_REF: &str =
    "schemas/workspace/m5-serialization-matrix.schema.json";

/// Repo-relative path to the companion document.
pub const M5_SERIALIZATION_MATRIX_DOC_REF: &str =
    "docs/workspace/m5/m5-serialization-and-restore.md";

/// Repo-relative path to the human-readable reviewer artifact.
pub const M5_SERIALIZATION_MATRIX_ARTIFACT_DOC_REF: &str =
    "artifacts/workspace/m5/m5-serialization-and-restore-matrix.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_SERIALIZATION_MATRIX_FIXTURE_DIR: &str =
    "fixtures/workspace/m5/m5-serialization-and-restore";

/// Embedded checked-in packet JSON.
pub const M5_SERIALIZATION_MATRIX_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/workspace/m5/m5-serialization-and-restore-matrix.json"
));

/// One of the six remembered-state artifact classes M5 is allowed to persist.
///
/// Each class names a distinct kind of remembered state. The matrix keeps them separate so that
/// layout restore, portable-state export, and crash-recovery evidence are never treated as
/// equivalent just because they share some artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RememberedArtifactClass {
    /// A re-resolvable checkpoint of the workspace authority that was granted, never the live
    /// authority itself.
    WorkspaceAuthorityCheckpoint,
    /// A snapshot of the window/pane topology and monitor geometry.
    WindowTopologySnapshot,
    /// A serialized, portable workspace-state package for export and import.
    PortableStatePackage,
    /// A restore-provenance record naming source, producer, schema outcome, and resulting fidelity.
    RestoreProvenanceRecord,
    /// A slot-preserving placeholder card standing in for a surface that could not be restored.
    PlaceholderCard,
    /// A compare or export summary projecting a diff between two remembered states.
    CompareExportSummary,
}

impl RememberedArtifactClass {
    /// Every remembered-state artifact class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WorkspaceAuthorityCheckpoint,
        Self::WindowTopologySnapshot,
        Self::PortableStatePackage,
        Self::RestoreProvenanceRecord,
        Self::PlaceholderCard,
        Self::CompareExportSummary,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceAuthorityCheckpoint => "workspace_authority_checkpoint",
            Self::WindowTopologySnapshot => "window_topology_snapshot",
            Self::PortableStatePackage => "portable_state_package",
            Self::RestoreProvenanceRecord => "restore_provenance_record",
            Self::PlaceholderCard => "placeholder_card",
            Self::CompareExportSummary => "compare_export_summary",
        }
    }
}

/// One of the four restore-fidelity classes a remembered-state artifact can support.
///
/// Ordered best to worst by [`RestoreFidelityClass::rank`]: an exact restore reproduces prior
/// state value-for-value, a compatible restore reproduces it through a forward migration, a
/// layout-only restore reproduces the pane/window slots while contents reopen as context or show
/// a placeholder, and a manual-review restore cannot be applied automatically and is surfaced for
/// a human with the slot preserved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreFidelityClass {
    /// The remembered state is restored value-for-value.
    ExactRestore,
    /// The remembered state is restored through a forward schema migration; semantics preserved.
    CompatibleRestore,
    /// Only the pane/window layout is restored; contents reopen as context or show a placeholder.
    LayoutOnly,
    /// The remembered state cannot be restored automatically; surfaced for review, slot preserved.
    ManualReview,
}

impl RestoreFidelityClass {
    /// Every restore-fidelity class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ExactRestore,
        Self::CompatibleRestore,
        Self::LayoutOnly,
        Self::ManualReview,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactRestore => "exact_restore",
            Self::CompatibleRestore => "compatible_restore",
            Self::LayoutOnly => "layout_only",
            Self::ManualReview => "manual_review",
        }
    }

    /// Monotonic rank; higher means more of the remembered state is restored automatically.
    pub const fn rank(self) -> u8 {
        match self {
            Self::ManualReview => 0,
            Self::LayoutOnly => 1,
            Self::CompatibleRestore => 2,
            Self::ExactRestore => 3,
        }
    }

    /// The weaker (lower-rank) of two restore-fidelity classes.
    pub const fn min(self, other: Self) -> Self {
        if other.rank() < self.rank() {
            other
        } else {
            self
        }
    }
}

/// How portable a remembered-state artifact class is.
///
/// Ordered most to least portable by [`OwnershipClass::portability_rank`]. Only
/// [`OwnershipClass::Portable`] and [`OwnershipClass::Shared`] state may be serialized into a
/// portable-state package; [`OwnershipClass::Local`] and [`OwnershipClass::MachineLocal`] state
/// never leaves the machine it was remembered on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnershipClass {
    /// Portable across machines and users; safe to serialize into a portable-state package.
    Portable,
    /// Shared within a team/sharing scope; portable inside that scope.
    Shared,
    /// Local to this machine/install; restorable across restarts but never exported as portable.
    Local,
    /// Bound to this machine/install — paths, monitor geometry, OS handles, machine trust anchors;
    /// never serialized into a portable package.
    MachineLocal,
}

impl OwnershipClass {
    /// Every ownership class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Portable,
        Self::Shared,
        Self::Local,
        Self::MachineLocal,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Portable => "portable",
            Self::Shared => "shared",
            Self::Local => "local",
            Self::MachineLocal => "machine_local",
        }
    }

    /// Monotonic portability rank; higher means it can travel further.
    pub const fn portability_rank(self) -> u8 {
        match self {
            Self::MachineLocal => 0,
            Self::Local => 1,
            Self::Shared => 2,
            Self::Portable => 3,
        }
    }

    /// Whether state with this ownership may be serialized into a portable-state package.
    pub const fn exportable_into_portable_package(self) -> bool {
        matches!(self, Self::Portable | Self::Shared)
    }
}

/// A redaction or exclusion guarantee a remembered-state artifact class makes.
///
/// Every class excludes secrets, live authority, and raw provider payloads; an exportable class
/// additionally excludes machine-local anchors so a portable-state package never carries
/// machine-unique state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionExclusion {
    /// No secret or credential body is serialized.
    ExcludesSecrets,
    /// No live authority handle or session token is serialized.
    ExcludesLiveAuthority,
    /// No machine-local anchor (path, monitor geometry, OS handle, machine trust anchor) is
    /// serialized into a portable package.
    ExcludesMachineLocalAnchors,
    /// No raw provider payload or workspace content is serialized.
    ExcludesRawProviderPayloads,
}

impl RedactionExclusion {
    /// Every exclusion, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ExcludesSecrets,
        Self::ExcludesLiveAuthority,
        Self::ExcludesMachineLocalAnchors,
        Self::ExcludesRawProviderPayloads,
    ];

    /// Exclusions every artifact class must guarantee regardless of portability.
    pub const BASELINE: [Self; 3] = [
        Self::ExcludesSecrets,
        Self::ExcludesLiveAuthority,
        Self::ExcludesRawProviderPayloads,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExcludesSecrets => "excludes_secrets",
            Self::ExcludesLiveAuthority => "excludes_live_authority",
            Self::ExcludesMachineLocalAnchors => "excludes_machine_local_anchors",
            Self::ExcludesRawProviderPayloads => "excludes_raw_provider_payloads",
        }
    }
}

/// The schema condition observed when a remembered-state artifact is restored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaCondition {
    /// The stored schema version matches the running build.
    SchemaMatch,
    /// The stored schema can be forward-migrated into the running build.
    SchemaForwardMigratable,
    /// The stored schema cannot be migrated; restore needs review.
    SchemaUnmigratable,
}

impl SchemaCondition {
    /// Every schema condition, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::SchemaMatch,
        Self::SchemaForwardMigratable,
        Self::SchemaUnmigratable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SchemaMatch => "schema_match",
            Self::SchemaForwardMigratable => "schema_forward_migratable",
            Self::SchemaUnmigratable => "schema_unmigratable",
        }
    }

    /// Highest restore fidelity this schema condition permits.
    pub const fn fidelity_ceiling(self) -> RestoreFidelityClass {
        match self {
            Self::SchemaMatch => RestoreFidelityClass::ExactRestore,
            Self::SchemaForwardMigratable => RestoreFidelityClass::CompatibleRestore,
            Self::SchemaUnmigratable => RestoreFidelityClass::ManualReview,
        }
    }

    /// Whether this condition narrows the restore (anything but a clean schema match).
    pub const fn is_drift(self) -> bool {
        !matches!(self, Self::SchemaMatch)
    }
}

/// The dependency condition observed when a remembered-state artifact is restored.
///
/// A missing dependency never silently deletes layout; it caps the restore at a slot-preserving
/// placeholder ([`RestoreFidelityClass::LayoutOnly`]) or, when the dependency root itself is gone,
/// surfaces the row for manual review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyCondition {
    /// Every dependency the remembered state references resolves.
    DependenciesPresent,
    /// Some dependencies are missing; the affected slots fall back to placeholders.
    DependenciesPartialMissing,
    /// The dependency root itself is missing; restore needs review.
    DependencyRootMissing,
}

impl DependencyCondition {
    /// Every dependency condition, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::DependenciesPresent,
        Self::DependenciesPartialMissing,
        Self::DependencyRootMissing,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DependenciesPresent => "dependencies_present",
            Self::DependenciesPartialMissing => "dependencies_partial_missing",
            Self::DependencyRootMissing => "dependency_root_missing",
        }
    }

    /// Highest restore fidelity this dependency condition permits.
    pub const fn fidelity_ceiling(self) -> RestoreFidelityClass {
        match self {
            Self::DependenciesPresent => RestoreFidelityClass::ExactRestore,
            Self::DependenciesPartialMissing => RestoreFidelityClass::LayoutOnly,
            Self::DependencyRootMissing => RestoreFidelityClass::ManualReview,
        }
    }

    /// Whether a dependency is missing (anything but fully present).
    pub const fn is_missing(self) -> bool {
        !matches!(self, Self::DependenciesPresent)
    }
}

/// The topology condition observed when a remembered-state artifact is restored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyCondition {
    /// The window/monitor topology is identical to the captured one.
    TopologyIdentical,
    /// The topology differs but the layout adapts to it (e.g. a monitor changed).
    TopologyAdapted,
    /// The topology is incompatible; only the pane layout can be restored.
    TopologyIncompatible,
}

impl TopologyCondition {
    /// Every topology condition, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::TopologyIdentical,
        Self::TopologyAdapted,
        Self::TopologyIncompatible,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyIdentical => "topology_identical",
            Self::TopologyAdapted => "topology_adapted",
            Self::TopologyIncompatible => "topology_incompatible",
        }
    }

    /// Highest restore fidelity this topology condition permits.
    pub const fn fidelity_ceiling(self) -> RestoreFidelityClass {
        match self {
            Self::TopologyIdentical => RestoreFidelityClass::ExactRestore,
            Self::TopologyAdapted => RestoreFidelityClass::CompatibleRestore,
            Self::TopologyIncompatible => RestoreFidelityClass::LayoutOnly,
        }
    }

    /// Whether the topology changed at all (anything but identical).
    pub const fn is_changed(self) -> bool {
        !matches!(self, Self::TopologyIdentical)
    }
}

/// How fresh the evidence backing a remembered-state row is.
///
/// Stale evidence narrows what the matrix is willing to trust: it caps the achieved restore
/// fidelity just as a schema, dependency, or topology condition does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshness {
    /// The evidence is current.
    Current,
    /// The evidence is aging but in tolerance; caps at a compatible restore.
    Aging,
    /// The evidence is expired; caps at a layout-only restore.
    Expired,
    /// The evidence is missing; caps at manual review.
    Missing,
}

impl EvidenceFreshness {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Current, Self::Aging, Self::Expired, Self::Missing];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Aging => "aging",
            Self::Expired => "expired",
            Self::Missing => "missing",
        }
    }

    /// Highest restore fidelity this freshness state permits.
    pub const fn fidelity_ceiling(self) -> RestoreFidelityClass {
        match self {
            Self::Current => RestoreFidelityClass::ExactRestore,
            Self::Aging => RestoreFidelityClass::CompatibleRestore,
            Self::Expired => RestoreFidelityClass::LayoutOnly,
            Self::Missing => RestoreFidelityClass::ManualReview,
        }
    }

    /// Whether this state is stale (anything but current).
    pub const fn is_stale(self) -> bool {
        !matches!(self, Self::Current)
    }
}

/// What a remembered-state row does when a dependency it references is missing.
///
/// The matrix forbids [`MissingDependencyBehavior::SilentDelete`]: a missing dependency must
/// preserve the slot as a placeholder or reopen the target as context — it never deletes layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissingDependencyBehavior {
    /// The slot is preserved as a placeholder card naming what is missing.
    PlaceholderSlotPreserved,
    /// The target is reopened as context without claiming a full restore.
    ReopenAsContext,
    /// The slot is silently deleted. **Forbidden** — present only so the gate can reject it.
    SilentDelete,
}

impl MissingDependencyBehavior {
    /// Every behavior, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::PlaceholderSlotPreserved,
        Self::ReopenAsContext,
        Self::SilentDelete,
    ];

    /// The behaviors the matrix permits; [`Self::SilentDelete`] is never one of them.
    pub const ALLOWED: [Self; 2] = [Self::PlaceholderSlotPreserved, Self::ReopenAsContext];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlaceholderSlotPreserved => "placeholder_slot_preserved",
            Self::ReopenAsContext => "reopen_as_context",
            Self::SilentDelete => "silent_delete",
        }
    }

    /// Whether the behavior preserves the slot rather than deleting layout.
    pub const fn preserves_slot(self) -> bool {
        !matches!(self, Self::SilentDelete)
    }
}

/// A headline reason the matrix narrows a row's restore fidelity below an exact restore.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeReason {
    /// The stored schema drifted from the running build.
    SchemaDrift,
    /// A dependency the remembered state references is missing.
    DependencyMissing,
    /// The window/monitor topology changed.
    TopologyChanged,
    /// The evidence backing the row is aging, expired, or missing.
    EvidenceStale,
}

impl DowngradeReason {
    /// Every downgrade reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SchemaDrift,
        Self::DependencyMissing,
        Self::TopologyChanged,
        Self::EvidenceStale,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SchemaDrift => "schema_drift",
            Self::DependencyMissing => "dependency_missing",
            Self::TopologyChanged => "topology_changed",
            Self::EvidenceStale => "evidence_stale",
        }
    }
}

/// The exact recovery path surfaced when a row's restore fidelity is narrowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryPath {
    /// Restore through the available forward schema migration.
    RestoreCompatibly,
    /// Locate the missing dependency or root, then restore more.
    RelocateDependency,
    /// Reopen the target as context; the layout is preserved.
    ReopenAsContext,
    /// Refresh the aging, expired, or missing evidence.
    RefreshEvidence,
    /// The restore cannot be applied automatically; review it.
    ManualReview,
    /// No recovery is needed; the row restores exactly.
    #[serde(rename = "none")]
    NoneNeeded,
}

impl RecoveryPath {
    /// Every recovery path, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RestoreCompatibly,
        Self::RelocateDependency,
        Self::ReopenAsContext,
        Self::RefreshEvidence,
        Self::ManualReview,
        Self::NoneNeeded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestoreCompatibly => "restore_compatibly",
            Self::RelocateDependency => "relocate_dependency",
            Self::ReopenAsContext => "reopen_as_context",
            Self::RefreshEvidence => "refresh_evidence",
            Self::ManualReview => "manual_review",
            Self::NoneNeeded => "none",
        }
    }

    /// Whether this is a real recovery path the owner can take.
    pub const fn is_offered(self) -> bool {
        !matches!(self, Self::NoneNeeded)
    }
}

/// A continuity surface that reuses remembered state and must reuse this matrix's vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuitySurface {
    /// Crash recovery and unsaved-state restore.
    CrashRecovery,
    /// Browser and mobile companion handoff.
    BrowserCompanionHandoff,
    /// Portable-state import and export.
    ImportExport,
    /// Release claim publication and shiproom evidence.
    ClaimPublication,
}

impl ContinuitySurface {
    /// Every continuity surface, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CrashRecovery,
        Self::BrowserCompanionHandoff,
        Self::ImportExport,
        Self::ClaimPublication,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrashRecovery => "crash_recovery",
            Self::BrowserCompanionHandoff => "browser_companion_handoff",
            Self::ImportExport => "import_export",
            Self::ClaimPublication => "claim_publication",
        }
    }
}

/// A restorable M5 surface classified by what it persists and how it restores.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestorableSurface {
    /// A preview route or rendered output pane.
    PreviewRoute,
    /// A notebook session.
    NotebookSession,
    /// A database query console.
    QueryConsole,
    /// A profiler trace capture.
    ProfilerCapture,
    /// A documentation pane.
    DocsPane,
    /// An incident workspace.
    IncidentWorkspace,
    /// A browser/mobile companion handoff packet.
    CompanionHandoffPacket,
    /// A portable-state artifact.
    PortableStateArtifact,
}

impl RestorableSurface {
    /// Every restorable surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::PreviewRoute,
        Self::NotebookSession,
        Self::QueryConsole,
        Self::ProfilerCapture,
        Self::DocsPane,
        Self::IncidentWorkspace,
        Self::CompanionHandoffPacket,
        Self::PortableStateArtifact,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewRoute => "preview_route",
            Self::NotebookSession => "notebook_session",
            Self::QueryConsole => "query_console",
            Self::ProfilerCapture => "profiler_capture",
            Self::DocsPane => "docs_pane",
            Self::IncidentWorkspace => "incident_workspace",
            Self::CompanionHandoffPacket => "companion_handoff_packet",
            Self::PortableStateArtifact => "portable_state_artifact",
        }
    }
}

/// A downstream reviewer surface that must ingest this matrix and narrow with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatrixConsumerSurface {
    /// Shiproom review and release-evidence surface.
    Shiproom,
    /// Docs and help surface.
    DocsHelp,
    /// Support-export bundle.
    SupportExport,
}

impl MatrixConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 3] = [Self::Shiproom, Self::DocsHelp, Self::SupportExport];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shiproom => "shiproom",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
        }
    }
}

/// One matrix row for a remembered-state artifact class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactClassRow {
    /// Stable row id.
    pub row_id: String,
    /// Remembered-state artifact class this row governs.
    pub artifact_class: RememberedArtifactClass,
    /// Owner accountable for the row.
    pub owner: String,
    /// What this class persists, in plain words.
    pub persisted_scope: String,
    /// Portability/ownership class.
    pub ownership: OwnershipClass,
    /// Whether this class may be serialized into a portable-state package.
    pub exportable: bool,
    /// Redaction/exclusion guarantees this class makes.
    #[serde(default)]
    pub redaction_exclusions: Vec<RedactionExclusion>,
    /// Restore-fidelity classes this artifact class can support.
    #[serde(default)]
    pub supported_fidelity_classes: Vec<RestoreFidelityClass>,
    /// Best restore fidelity this class claims, before the gate.
    pub declared_max_fidelity: RestoreFidelityClass,
    /// Observed schema condition.
    pub schema_condition: SchemaCondition,
    /// Observed dependency condition.
    pub dependency_condition: DependencyCondition,
    /// Observed topology condition.
    pub topology_condition: TopologyCondition,
    /// How fresh the row's evidence is.
    pub evidence_freshness: EvidenceFreshness,
    /// What the row does when a dependency is missing; never a silent delete.
    pub missing_dependency_behavior: MissingDependencyBehavior,
    /// Restore fidelity actually achieved after the gate; must equal
    /// [`ArtifactClassRow::achieved_fidelity`].
    pub published_fidelity: RestoreFidelityClass,
    /// Headline downgrade reasons; must equal the recomputed set.
    #[serde(default)]
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Recovery path surfaced when the fidelity is narrowed; must equal the recomputed path.
    pub recovery_path: RecoveryPath,
    /// Continuity surfaces that reuse this artifact class.
    #[serde(default)]
    pub continuity_surfaces: Vec<ContinuitySurface>,
    /// Caveats attached to the published fidelity.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Fields whose evidence is stale, missing, or narrowing the row.
    #[serde(default)]
    pub stale_or_missing_fields: Vec<String>,
    /// Ref to the schema governing this artifact class.
    pub schema_ref: String,
    /// Ref to the row's supporting evidence.
    pub evidence_ref: String,
    /// Active scope snapshot the row answered, stamped for replay.
    pub scope_snapshot_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl ArtifactClassRow {
    /// The restore fidelity the gate permits this row to achieve.
    ///
    /// The weakest ceiling implied by the declared maximum, the schema condition, the dependency
    /// condition, the topology condition, and the evidence freshness, so a schema drift, a missing
    /// dependency, a changed topology, or stale evidence can never publish an exact restore.
    pub fn achieved_fidelity(&self) -> RestoreFidelityClass {
        self.declared_max_fidelity
            .min(self.schema_condition.fidelity_ceiling())
            .min(self.dependency_condition.fidelity_ceiling())
            .min(self.topology_condition.fidelity_ceiling())
            .min(self.evidence_freshness.fidelity_ceiling())
    }

    /// The headline downgrade reasons recomputed from the row's observed conditions.
    pub fn computed_downgrade_reasons(&self) -> Vec<DowngradeReason> {
        let mut reasons = Vec::new();
        if self.schema_condition.is_drift() {
            reasons.push(DowngradeReason::SchemaDrift);
        }
        if self.dependency_condition.is_missing() {
            reasons.push(DowngradeReason::DependencyMissing);
        }
        if self.topology_condition.is_changed() {
            reasons.push(DowngradeReason::TopologyChanged);
        }
        if self.evidence_freshness.is_stale() {
            reasons.push(DowngradeReason::EvidenceStale);
        }
        reasons
    }

    /// The recovery path the gate must record, derived from the row's observed conditions.
    ///
    /// Ordered by severity: a manual-review restore points at review, a missing dependency points
    /// at relocating it, a migratable schema points at a compatible restore, a changed topology
    /// points at reopening as context, stale evidence points at a refresh, and a clean row needs
    /// nothing.
    pub fn computed_recovery_path(&self) -> RecoveryPath {
        if self.achieved_fidelity() == RestoreFidelityClass::ManualReview {
            RecoveryPath::ManualReview
        } else if self.dependency_condition.is_missing() {
            RecoveryPath::RelocateDependency
        } else if self.schema_condition.is_drift() {
            RecoveryPath::RestoreCompatibly
        } else if self.topology_condition.is_changed() {
            RecoveryPath::ReopenAsContext
        } else if self.evidence_freshness.is_stale() {
            RecoveryPath::RefreshEvidence
        } else {
            RecoveryPath::NoneNeeded
        }
    }

    /// Whether the row achieves a clean exact restore.
    pub fn is_exact(&self) -> bool {
        self.achieved_fidelity() == RestoreFidelityClass::ExactRestore
    }

    /// Whether the gate narrowed the achieved fidelity below the declared maximum.
    pub fn is_downgraded(&self) -> bool {
        self.achieved_fidelity().rank() < self.declared_max_fidelity.rank()
    }

    /// Exclusions the row must guarantee given its portability.
    pub fn required_exclusions(&self) -> Vec<RedactionExclusion> {
        if self.exportable {
            RedactionExclusion::ALL.to_vec()
        } else {
            RedactionExclusion::BASELINE.to_vec()
        }
    }

    /// Whether the row guarantees every exclusion its portability requires.
    pub fn has_required_exclusions(&self) -> bool {
        let present: BTreeSet<RedactionExclusion> =
            self.redaction_exclusions.iter().copied().collect();
        self.required_exclusions()
            .iter()
            .all(|e| present.contains(e))
    }

    /// Whether the row carries its own non-empty refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.schema_ref.trim().is_empty()
            && !self.evidence_ref.trim().is_empty()
            && !self.scope_snapshot_ref.trim().is_empty()
    }

    /// Whether the stored published fidelity, reasons, and path all agree with the recomputed gate.
    pub fn gate_consistent(&self) -> bool {
        self.published_fidelity == self.achieved_fidelity()
            && self.downgrade_reasons == self.computed_downgrade_reasons()
            && self.recovery_path == self.computed_recovery_path()
    }
}

/// One classification row for a restorable M5 surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceRow {
    /// Stable row id.
    pub row_id: String,
    /// Restorable surface this row classifies.
    pub surface: RestorableSurface,
    /// Owner accountable for the row.
    pub owner: String,
    /// Remembered-state artifact classes this surface persists.
    #[serde(default)]
    pub persisted_artifact_classes: Vec<RememberedArtifactClass>,
    /// Best restore fidelity this surface supports.
    pub max_supported_fidelity: RestoreFidelityClass,
    /// How portable this surface's remembered state is.
    pub portability: OwnershipClass,
    /// Continuity surfaces this surface participates in.
    #[serde(default)]
    pub continuity_surfaces: Vec<ContinuitySurface>,
    /// Reviewer-facing note.
    pub note: String,
}

impl SurfaceRow {
    /// Whether the surface persists at least one artifact class and references no duplicates.
    pub fn persists_distinct_classes(&self) -> bool {
        if self.persisted_artifact_classes.is_empty() {
            return false;
        }
        let mut seen = BTreeSet::new();
        self.persisted_artifact_classes
            .iter()
            .all(|c| seen.insert(*c))
    }
}

/// A matrix-level cross-link binding a continuity surface to its canonical packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuityCrossLink {
    /// Continuity surface this cross-link binds.
    pub continuity_surface: ContinuitySurface,
    /// Stable cross-link ref.
    pub crosslink_ref: String,
    /// Canonical packet this surface reuses for its restore language.
    pub canonical_packet_ref: String,
    /// True when the surface reuses this matrix's vocabulary rather than inventing its own.
    pub reuses_matrix_vocabulary: bool,
}

impl ContinuityCrossLink {
    fn is_canonical(&self) -> bool {
        self.reuses_matrix_vocabulary
            && !self.crosslink_ref.trim().is_empty()
            && !self.canonical_packet_ref.trim().is_empty()
    }
}

/// One binding wiring a downstream reviewer surface to this matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatrixConsumerBinding {
    /// Consumer surface this binding wires.
    pub consumer_surface: MatrixConsumerSurface,
    /// Stable binding ref.
    pub binding_ref: String,
    /// Matrix packet id this surface ingests.
    pub matrix_packet_id_ref: String,
    /// Active scope snapshot stamped on the binding for replay.
    pub scope_snapshot_ref: String,
    /// True when the surface ingests this matrix rather than a parallel sheet.
    pub ingests_matrix: bool,
    /// True when the surface preserves the published fidelity labels verbatim.
    pub preserves_fidelity_labels: bool,
    /// True when the surface preserves the ownership/portability labels verbatim.
    pub preserves_ownership_labels: bool,
    /// True when the surface narrows automatically as rows are downgraded.
    pub narrows_on_downgrade: bool,
    /// True when raw private material is excluded from the binding.
    pub raw_private_material_excluded: bool,
}

impl MatrixConsumerBinding {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.matrix_packet_id_ref == packet_id
            && self.ingests_matrix
            && self.preserves_fidelity_labels
            && self.preserves_ownership_labels
            && self.narrows_on_downgrade
            && self.raw_private_material_excluded
            && !self.binding_ref.trim().is_empty()
            && !self.scope_snapshot_ref.trim().is_empty()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5SerializationMatrixSummary {
    /// Total artifact-class rows.
    pub artifact_class_rows: usize,
    /// Total surface rows.
    pub surface_rows: usize,
    /// Rows achieving an exact restore.
    pub exact_restore_rows: usize,
    /// Rows achieving a compatible restore.
    pub compatible_restore_rows: usize,
    /// Rows achieving a layout-only restore.
    pub layout_only_rows: usize,
    /// Rows held for manual review.
    pub manual_review_rows: usize,
    /// Rows the gate narrowed below their declared maximum.
    pub downgraded_rows: usize,
    /// Rows exportable into a portable-state package.
    pub exportable_rows: usize,
    /// Rows whose evidence is aging, expired, or missing.
    pub stale_evidence_rows: usize,
}

/// A redaction-safe export row projected from an artifact-class row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SerializationMatrixExportRow {
    /// Row id.
    pub row_id: String,
    /// Artifact-class token.
    pub artifact_class: String,
    /// Owner accountable for the row.
    pub owner: String,
    /// Ownership/portability token.
    pub ownership: String,
    /// Whether the class is exportable into a portable-state package.
    pub exportable: bool,
    /// Declared-max-fidelity token.
    pub declared_max_fidelity: String,
    /// Published-fidelity token.
    pub published_fidelity: String,
    /// Downgrade-reason tokens.
    pub downgrade_reasons: Vec<String>,
    /// Recovery-path token.
    pub recovery_path: String,
    /// Missing-dependency-behavior token.
    pub missing_dependency_behavior: String,
    /// Continuity-surface tokens this class reuses.
    pub continuity_surfaces: Vec<String>,
    /// Whether the row achieves a clean exact restore.
    pub exact: bool,
    /// Whether the published fidelity was downgraded below the declared maximum.
    pub downgraded: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the matrix — the canonical index downstream surfaces
/// render instead of restating each row's restore class by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SerializationMatrixExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected artifact-class rows.
    pub rows: Vec<M5SerializationMatrixExportRow>,
    /// Whether every row's published fidelity and path agree with the gate.
    pub all_rows_gate_consistent: bool,
    /// Rows achieving an exact restore.
    pub exact_count: usize,
    /// Rows the gate narrowed below an exact restore.
    pub narrowed_count: usize,
    /// Rows held for manual review.
    pub manual_review_count: usize,
}

/// The typed M5 serialization-and-restore matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5SerializationMatrix {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed remembered-state artifact-class vocabulary.
    pub artifact_classes: Vec<RememberedArtifactClass>,
    /// Closed restore-fidelity-class vocabulary.
    pub restore_fidelity_classes: Vec<RestoreFidelityClass>,
    /// Closed ownership-class vocabulary.
    pub ownership_classes: Vec<OwnershipClass>,
    /// Closed redaction-exclusion vocabulary.
    pub redaction_exclusions: Vec<RedactionExclusion>,
    /// Closed schema-condition vocabulary.
    pub schema_conditions: Vec<SchemaCondition>,
    /// Closed dependency-condition vocabulary.
    pub dependency_conditions: Vec<DependencyCondition>,
    /// Closed topology-condition vocabulary.
    pub topology_conditions: Vec<TopologyCondition>,
    /// Closed evidence-freshness vocabulary.
    pub evidence_freshness_states: Vec<EvidenceFreshness>,
    /// Closed missing-dependency-behavior vocabulary.
    pub missing_dependency_behaviors: Vec<MissingDependencyBehavior>,
    /// Closed downgrade-reason vocabulary.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Closed recovery-path vocabulary.
    pub recovery_paths: Vec<RecoveryPath>,
    /// Closed continuity-surface vocabulary.
    pub continuity_surfaces: Vec<ContinuitySurface>,
    /// Closed restorable-surface vocabulary.
    pub restorable_surfaces: Vec<RestorableSurface>,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<MatrixConsumerSurface>,
    /// Artifact-class rows, one per remembered-state artifact class.
    #[serde(default)]
    pub rows: Vec<ArtifactClassRow>,
    /// Surface rows, one per restorable surface.
    #[serde(default)]
    pub surface_rows: Vec<SurfaceRow>,
    /// Continuity cross-links, one per continuity surface.
    #[serde(default)]
    pub continuity_crosslinks: Vec<ContinuityCrossLink>,
    /// Consumer bindings, one per required reviewer surface.
    #[serde(default)]
    pub consumer_bindings: Vec<MatrixConsumerBinding>,
    /// Summary counts.
    pub summary: M5SerializationMatrixSummary,
}

impl M5SerializationMatrix {
    /// Returns the row for a remembered-state artifact class.
    pub fn row(&self, class: RememberedArtifactClass) -> Option<&ArtifactClassRow> {
        self.rows.iter().find(|r| r.artifact_class == class)
    }

    /// Returns the classification row for a restorable surface.
    pub fn surface_row(&self, surface: RestorableSurface) -> Option<&SurfaceRow> {
        self.surface_rows.iter().find(|r| r.surface == surface)
    }

    /// The strongest declared maximum fidelity across the artifact classes a surface persists.
    fn best_persisted_declared_fidelity(
        &self,
        surface: &SurfaceRow,
    ) -> Option<RestoreFidelityClass> {
        surface
            .persisted_artifact_classes
            .iter()
            .filter_map(|c| self.row(*c))
            .map(|r| r.declared_max_fidelity)
            .max_by_key(|f| f.rank())
    }

    /// The most portable ownership across the artifact classes a surface persists.
    fn best_persisted_portability(&self, surface: &SurfaceRow) -> Option<OwnershipClass> {
        surface
            .persisted_artifact_classes
            .iter()
            .filter_map(|c| self.row(*c))
            .map(|r| r.ownership)
            .max_by_key(|o| o.portability_rank())
    }

    /// Whether every artifact-class and surface row agrees with the recomputed gate.
    pub fn all_rows_gate_consistent(&self) -> bool {
        self.rows.iter().all(|r| r.gate_consistent())
    }

    /// Whether a consumer binding preserves this matrix for the given surface.
    pub fn has_binding_for(&self, surface: MatrixConsumerSurface) -> bool {
        self.consumer_bindings
            .iter()
            .any(|b| b.consumer_surface == surface && b.preserves_truth_for(&self.packet_id))
    }

    /// Whether a continuity cross-link binds the given surface.
    pub fn has_crosslink_for(&self, surface: ContinuitySurface) -> bool {
        self.continuity_crosslinks
            .iter()
            .any(|c| c.continuity_surface == surface && c.is_canonical())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5SerializationMatrixSummary {
        let count_fidelity = |class: RestoreFidelityClass| {
            self.rows
                .iter()
                .filter(|r| r.published_fidelity == class)
                .count()
        };
        M5SerializationMatrixSummary {
            artifact_class_rows: self.rows.len(),
            surface_rows: self.surface_rows.len(),
            exact_restore_rows: count_fidelity(RestoreFidelityClass::ExactRestore),
            compatible_restore_rows: count_fidelity(RestoreFidelityClass::CompatibleRestore),
            layout_only_rows: count_fidelity(RestoreFidelityClass::LayoutOnly),
            manual_review_rows: count_fidelity(RestoreFidelityClass::ManualReview),
            downgraded_rows: self.rows.iter().filter(|r| r.is_downgraded()).count(),
            exportable_rows: self.rows.iter().filter(|r| r.exportable).count(),
            stale_evidence_rows: self
                .rows
                .iter()
                .filter(|r| r.evidence_freshness.is_stale())
                .count(),
        }
    }

    /// Produces the restore index downstream surfaces — shiproom, docs/help, and support exports —
    /// render instead of restating each row's restore class by hand.
    pub fn export_projection(&self) -> M5SerializationMatrixExportProjection {
        let rows = self
            .rows
            .iter()
            .map(|r| M5SerializationMatrixExportRow {
                row_id: r.row_id.clone(),
                artifact_class: r.artifact_class.as_str().to_owned(),
                owner: r.owner.clone(),
                ownership: r.ownership.as_str().to_owned(),
                exportable: r.exportable,
                declared_max_fidelity: r.declared_max_fidelity.as_str().to_owned(),
                published_fidelity: r.published_fidelity.as_str().to_owned(),
                downgrade_reasons: r
                    .downgrade_reasons
                    .iter()
                    .map(|x| x.as_str().to_owned())
                    .collect(),
                recovery_path: r.recovery_path.as_str().to_owned(),
                missing_dependency_behavior: r.missing_dependency_behavior.as_str().to_owned(),
                continuity_surfaces: r
                    .continuity_surfaces
                    .iter()
                    .map(|x| x.as_str().to_owned())
                    .collect(),
                exact: r.is_exact(),
                downgraded: r.is_downgraded(),
                summary: format!(
                    "{}: {} ownership, declared {}, published {}, recovery {}",
                    r.artifact_class.as_str(),
                    r.ownership.as_str(),
                    r.declared_max_fidelity.as_str(),
                    r.published_fidelity.as_str(),
                    r.recovery_path.as_str()
                ),
            })
            .collect();
        M5SerializationMatrixExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_rows_gate_consistent: self.all_rows_gate_consistent(),
            exact_count: self.rows.iter().filter(|r| r.is_exact()).count(),
            narrowed_count: self.rows.iter().filter(|r| !r.is_exact()).count(),
            manual_review_count: self
                .rows
                .iter()
                .filter(|r| r.published_fidelity == RestoreFidelityClass::ManualReview)
                .count(),
        }
    }

    /// Builds an export-safe support packet preserving the exact matrix.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> M5SerializationMatrixSupportExport {
        M5SerializationMatrixSupportExport {
            record_kind: M5_SERIALIZATION_MATRIX_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_SERIALIZATION_MATRIX_SCHEMA_VERSION,
            export_id: export_id.into(),
            matrix_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            matrix: self.clone(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5SerializationMatrixViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen_ids = BTreeSet::new();
        let mut seen_classes = BTreeSet::new();
        for row in &self.rows {
            if !seen_ids.insert(row.row_id.clone()) {
                violations.push(M5SerializationMatrixViolation::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }
            if !seen_classes.insert(row.artifact_class) {
                violations.push(M5SerializationMatrixViolation::DuplicateArtifactClassRow {
                    class: row.artifact_class.as_str(),
                });
            }
            self.validate_artifact_row(row, &mut violations);
        }
        for &class in &RememberedArtifactClass::ALL {
            if !seen_classes.contains(&class) {
                violations.push(M5SerializationMatrixViolation::MissingArtifactClassRow {
                    class: class.as_str(),
                });
            }
        }

        let mut seen_surface_ids = BTreeSet::new();
        let mut seen_surfaces = BTreeSet::new();
        for row in &self.surface_rows {
            if !seen_surface_ids.insert(row.row_id.clone()) {
                violations.push(M5SerializationMatrixViolation::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }
            if !seen_surfaces.insert(row.surface) {
                violations.push(M5SerializationMatrixViolation::DuplicateSurfaceRow {
                    surface: row.surface.as_str(),
                });
            }
            self.validate_surface_row(row, &mut violations);
        }
        for &surface in &RestorableSurface::ALL {
            if !seen_surfaces.contains(&surface) {
                violations.push(M5SerializationMatrixViolation::MissingSurfaceRow {
                    surface: surface.as_str(),
                });
            }
        }

        for surface in ContinuitySurface::ALL {
            if !self.has_crosslink_for(surface) {
                violations.push(M5SerializationMatrixViolation::MissingContinuityCrossLink {
                    surface: surface.as_str(),
                });
            }
        }
        for surface in MatrixConsumerSurface::REQUIRED {
            if !self.has_binding_for(surface) {
                violations.push(M5SerializationMatrixViolation::MissingConsumerBinding {
                    surface: surface.as_str(),
                });
            }
        }
        for binding in &self.consumer_bindings {
            if !binding.preserves_truth_for(&self.packet_id) {
                violations.push(M5SerializationMatrixViolation::ConsumerBindingDrift {
                    binding_ref: binding.binding_ref.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5SerializationMatrixViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5SerializationMatrixViolation>) {
        if self.schema_version != M5_SERIALIZATION_MATRIX_SCHEMA_VERSION {
            violations.push(M5SerializationMatrixViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_SERIALIZATION_MATRIX_RECORD_KIND {
            violations.push(M5SerializationMatrixViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5SerializationMatrixViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "artifact_classes",
                self.artifact_classes == RememberedArtifactClass::ALL.to_vec(),
            ),
            (
                "restore_fidelity_classes",
                self.restore_fidelity_classes == RestoreFidelityClass::ALL.to_vec(),
            ),
            (
                "ownership_classes",
                self.ownership_classes == OwnershipClass::ALL.to_vec(),
            ),
            (
                "redaction_exclusions",
                self.redaction_exclusions == RedactionExclusion::ALL.to_vec(),
            ),
            (
                "schema_conditions",
                self.schema_conditions == SchemaCondition::ALL.to_vec(),
            ),
            (
                "dependency_conditions",
                self.dependency_conditions == DependencyCondition::ALL.to_vec(),
            ),
            (
                "topology_conditions",
                self.topology_conditions == TopologyCondition::ALL.to_vec(),
            ),
            (
                "evidence_freshness_states",
                self.evidence_freshness_states == EvidenceFreshness::ALL.to_vec(),
            ),
            (
                "missing_dependency_behaviors",
                self.missing_dependency_behaviors == MissingDependencyBehavior::ALL.to_vec(),
            ),
            (
                "downgrade_reasons",
                self.downgrade_reasons == DowngradeReason::ALL.to_vec(),
            ),
            (
                "recovery_paths",
                self.recovery_paths == RecoveryPath::ALL.to_vec(),
            ),
            (
                "continuity_surfaces",
                self.continuity_surfaces == ContinuitySurface::ALL.to_vec(),
            ),
            (
                "restorable_surfaces",
                self.restorable_surfaces == RestorableSurface::ALL.to_vec(),
            ),
            (
                "consumer_surfaces",
                self.consumer_surfaces == MatrixConsumerSurface::REQUIRED.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5SerializationMatrixViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_artifact_row(
        &self,
        row: &ArtifactClassRow,
        violations: &mut Vec<M5SerializationMatrixViolation>,
    ) {
        for (field, value) in [
            ("row_id", &row.row_id),
            ("owner", &row.owner),
            ("persisted_scope", &row.persisted_scope),
            ("schema_ref", &row.schema_ref),
            ("evidence_ref", &row.evidence_ref),
            ("scope_snapshot_ref", &row.scope_snapshot_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5SerializationMatrixViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: field,
                });
            }
        }

        // The achieved fidelity must equal the gate's recomputed ceiling, so a schema drift, a
        // missing dependency, a changed topology, or stale evidence can never publish exact.
        let achieved = row.achieved_fidelity();
        if row.published_fidelity != achieved {
            violations.push(M5SerializationMatrixViolation::OverstatedFidelity {
                row_id: row.row_id.clone(),
                published: row.published_fidelity.as_str(),
                computed: achieved.as_str(),
            });
        }

        // The published fidelity may never exceed the declared maximum.
        if row.published_fidelity.rank() > row.declared_max_fidelity.rank() {
            violations.push(M5SerializationMatrixViolation::ExceedsDeclaredFidelity {
                row_id: row.row_id.clone(),
                published: row.published_fidelity.as_str(),
                declared: row.declared_max_fidelity.as_str(),
            });
        }

        // The published fidelity must be one the artifact class declares it can support.
        if !row
            .supported_fidelity_classes
            .contains(&row.published_fidelity)
        {
            violations.push(M5SerializationMatrixViolation::UnsupportedFidelity {
                row_id: row.row_id.clone(),
                fidelity: row.published_fidelity.as_str(),
            });
        }

        let computed = row.computed_downgrade_reasons();
        if row.downgrade_reasons != computed {
            violations.push(M5SerializationMatrixViolation::DowngradeReasonsMismatch {
                row_id: row.row_id.clone(),
            });
        }
        let computed_path = row.computed_recovery_path();
        if row.recovery_path != computed_path {
            violations.push(M5SerializationMatrixViolation::RecoveryPathMismatch {
                row_id: row.row_id.clone(),
                declared: row.recovery_path.as_str(),
                required: computed_path.as_str(),
            });
        }

        // A missing dependency never silently deletes layout: the behavior must preserve the slot.
        if !row.missing_dependency_behavior.preserves_slot() {
            violations.push(M5SerializationMatrixViolation::SilentLayoutDelete {
                row_id: row.row_id.clone(),
            });
        }

        // Portability is claimed, never assumed: a machine-local row is never exportable, and an
        // exportable row must exclude secrets, live authority, and machine-local anchors.
        if row.exportable && !row.ownership.exportable_into_portable_package() {
            violations.push(M5SerializationMatrixViolation::NonPortableExport {
                row_id: row.row_id.clone(),
                ownership: row.ownership.as_str(),
            });
        }
        if !row.has_required_exclusions() {
            violations.push(M5SerializationMatrixViolation::MissingRedactionExclusion {
                row_id: row.row_id.clone(),
            });
        }

        if !row.has_required_evidence() {
            violations.push(M5SerializationMatrixViolation::EmptyField {
                id: row.row_id.clone(),
                field_name: "evidence_refs",
            });
        }
        if row.supported_fidelity_classes.is_empty() {
            violations.push(M5SerializationMatrixViolation::EmptyField {
                id: row.row_id.clone(),
                field_name: "supported_fidelity_classes",
            });
        }
        if row.continuity_surfaces.is_empty() {
            violations.push(M5SerializationMatrixViolation::EmptyField {
                id: row.row_id.clone(),
                field_name: "continuity_surfaces",
            });
        }

        // A narrowed row must offer a real recovery path, name a caveat, and name what is stale.
        if achieved != RestoreFidelityClass::ExactRestore {
            if !row.recovery_path.is_offered() {
                violations.push(M5SerializationMatrixViolation::MissingRecoveryPath {
                    row_id: row.row_id.clone(),
                });
            }
            if row.caveats.is_empty() {
                violations.push(M5SerializationMatrixViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: "caveats",
                });
            }
            if row.stale_or_missing_fields.is_empty() {
                violations.push(M5SerializationMatrixViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: "stale_or_missing_fields",
                });
            }
        }

        // An exact-restore row must be genuinely clean: pristine conditions, no downgrade reason,
        // no recovery path. This is the guardrail against an inherited "remembers everything"
        // badge over a surface that actually drifted, lost a dependency, or went stale.
        if achieved == RestoreFidelityClass::ExactRestore
            && (row.schema_condition != SchemaCondition::SchemaMatch
                || row.dependency_condition != DependencyCondition::DependenciesPresent
                || row.topology_condition != TopologyCondition::TopologyIdentical
                || row.evidence_freshness != EvidenceFreshness::Current
                || !row.downgrade_reasons.is_empty()
                || !row.caveats.is_empty()
                || !row.stale_or_missing_fields.is_empty()
                || row.recovery_path.is_offered())
        {
            violations.push(M5SerializationMatrixViolation::ExactRowNotClean {
                row_id: row.row_id.clone(),
            });
        }
    }

    fn validate_surface_row(
        &self,
        row: &SurfaceRow,
        violations: &mut Vec<M5SerializationMatrixViolation>,
    ) {
        for (field, value) in [
            ("row_id", &row.row_id),
            ("owner", &row.owner),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5SerializationMatrixViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: field,
                });
            }
        }

        if !row.persists_distinct_classes() {
            violations.push(M5SerializationMatrixViolation::SurfacePersistsNothing {
                row_id: row.row_id.clone(),
            });
        }
        // Every persisted class must reference a real artifact-class row.
        for class in &row.persisted_artifact_classes {
            if self.row(*class).is_none() {
                violations.push(
                    M5SerializationMatrixViolation::SurfaceUnknownArtifactClass {
                        row_id: row.row_id.clone(),
                        class: class.as_str(),
                    },
                );
            }
        }
        if row.continuity_surfaces.is_empty() {
            violations.push(M5SerializationMatrixViolation::EmptyField {
                id: row.row_id.clone(),
                field_name: "continuity_surfaces",
            });
        }

        // A surface cannot claim a restore fidelity better than any artifact class it persists.
        if let Some(best) = self.best_persisted_declared_fidelity(row) {
            if row.max_supported_fidelity.rank() > best.rank() {
                violations.push(
                    M5SerializationMatrixViolation::SurfaceFidelityExceedsClasses {
                        row_id: row.row_id.clone(),
                        surface_fidelity: row.max_supported_fidelity.as_str(),
                        class_fidelity: best.as_str(),
                    },
                );
            }
        }
        // A surface cannot be more portable than any artifact class it persists, and a portable or
        // shared surface must persist at least one exportable class.
        if let Some(best) = self.best_persisted_portability(row) {
            if row.portability.portability_rank() > best.portability_rank() {
                violations.push(
                    M5SerializationMatrixViolation::SurfacePortabilityExceedsClasses {
                        row_id: row.row_id.clone(),
                        surface_ownership: row.portability.as_str(),
                        class_ownership: best.as_str(),
                    },
                );
            }
        }
        if row.portability.exportable_into_portable_package() {
            let any_exportable = row
                .persisted_artifact_classes
                .iter()
                .filter_map(|c| self.row(*c))
                .any(|r| r.exportable);
            if !any_exportable {
                violations.push(M5SerializationMatrixViolation::SurfacePortabilityUnbacked {
                    row_id: row.row_id.clone(),
                });
            }
        }
    }
}

/// A validation violation for the M5 serialization-and-restore matrix packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5SerializationMatrixViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row id appears more than once.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
    },
    /// An artifact class carries more than one row.
    DuplicateArtifactClassRow {
        /// Artifact-class token.
        class: &'static str,
    },
    /// An artifact class has no row.
    MissingArtifactClassRow {
        /// Artifact-class token.
        class: &'static str,
    },
    /// A restorable surface carries more than one row.
    DuplicateSurfaceRow {
        /// Surface token.
        surface: &'static str,
    },
    /// A restorable surface has no row.
    MissingSurfaceRow {
        /// Surface token.
        surface: &'static str,
    },
    /// A row publishes a fidelity beyond what the gate computes.
    OverstatedFidelity {
        /// Row id.
        row_id: String,
        /// Published fidelity token.
        published: &'static str,
        /// Computed fidelity token.
        computed: &'static str,
    },
    /// A row publishes a fidelity above its declared maximum.
    ExceedsDeclaredFidelity {
        /// Row id.
        row_id: String,
        /// Published fidelity token.
        published: &'static str,
        /// Declared maximum token.
        declared: &'static str,
    },
    /// A row publishes a fidelity its artifact class does not list as supported.
    UnsupportedFidelity {
        /// Row id.
        row_id: String,
        /// Published fidelity token.
        fidelity: &'static str,
    },
    /// A row's downgrade reasons disagree with the recomputed reasons.
    DowngradeReasonsMismatch {
        /// Row id.
        row_id: String,
    },
    /// A row's recovery path disagrees with the recomputed path.
    RecoveryPathMismatch {
        /// Row id.
        row_id: String,
        /// Declared path token.
        declared: &'static str,
        /// Required path token.
        required: &'static str,
    },
    /// A row would silently delete layout when a dependency is missing.
    SilentLayoutDelete {
        /// Row id.
        row_id: String,
    },
    /// A row is exportable but its ownership cannot leave the machine.
    NonPortableExport {
        /// Row id.
        row_id: String,
        /// Ownership token.
        ownership: &'static str,
    },
    /// A row does not guarantee every redaction exclusion its portability requires.
    MissingRedactionExclusion {
        /// Row id.
        row_id: String,
    },
    /// A narrowed row offers no recovery path.
    MissingRecoveryPath {
        /// Row id.
        row_id: String,
    },
    /// An exact-restore row still narrows a condition or carries a downgrade reason.
    ExactRowNotClean {
        /// Row id.
        row_id: String,
    },
    /// A surface row persists no artifact class.
    SurfacePersistsNothing {
        /// Row id.
        row_id: String,
    },
    /// A surface row persists an artifact class with no matrix row.
    SurfaceUnknownArtifactClass {
        /// Row id.
        row_id: String,
        /// Artifact-class token.
        class: &'static str,
    },
    /// A surface claims a restore fidelity better than any class it persists.
    SurfaceFidelityExceedsClasses {
        /// Row id.
        row_id: String,
        /// Surface fidelity token.
        surface_fidelity: &'static str,
        /// Best class fidelity token.
        class_fidelity: &'static str,
    },
    /// A surface claims more portability than any class it persists.
    SurfacePortabilityExceedsClasses {
        /// Row id.
        row_id: String,
        /// Surface ownership token.
        surface_ownership: &'static str,
        /// Best class ownership token.
        class_ownership: &'static str,
    },
    /// A portable or shared surface persists no exportable class.
    SurfacePortabilityUnbacked {
        /// Row id.
        row_id: String,
    },
    /// A continuity surface has no canonical cross-link.
    MissingContinuityCrossLink {
        /// Surface token.
        surface: &'static str,
    },
    /// A required consumer surface has no binding.
    MissingConsumerBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A consumer binding drops or remints matrix truth.
    ConsumerBindingDrift {
        /// Binding ref.
        binding_ref: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5SerializationMatrixViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical vocabulary")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateRowId { row_id } => write!(f, "duplicate row id {row_id}"),
            Self::DuplicateArtifactClassRow { class } => {
                write!(f, "duplicate row for artifact class {class}")
            }
            Self::MissingArtifactClassRow { class } => {
                write!(f, "missing row for artifact class {class}")
            }
            Self::DuplicateSurfaceRow { surface } => {
                write!(f, "duplicate row for surface {surface}")
            }
            Self::MissingSurfaceRow { surface } => {
                write!(f, "missing row for restorable surface {surface}")
            }
            Self::OverstatedFidelity {
                row_id,
                published,
                computed,
            } => write!(
                f,
                "row {row_id} publishes fidelity {published} but the gate computes {computed}"
            ),
            Self::ExceedsDeclaredFidelity {
                row_id,
                published,
                declared,
            } => write!(
                f,
                "row {row_id} publishes fidelity {published} above declared maximum {declared}"
            ),
            Self::UnsupportedFidelity { row_id, fidelity } => write!(
                f,
                "row {row_id} publishes fidelity {fidelity} its class does not support"
            ),
            Self::DowngradeReasonsMismatch { row_id } => {
                write!(f, "row {row_id} downgrade reasons disagree with the gate")
            }
            Self::RecoveryPathMismatch {
                row_id,
                declared,
                required,
            } => write!(
                f,
                "row {row_id} records recovery {declared} but the gate requires {required}"
            ),
            Self::SilentLayoutDelete { row_id } => {
                write!(f, "row {row_id} would silently delete layout on a missing dependency")
            }
            Self::NonPortableExport { row_id, ownership } => write!(
                f,
                "row {row_id} is exportable but {ownership} state cannot leave the machine"
            ),
            Self::MissingRedactionExclusion { row_id } => {
                write!(f, "row {row_id} does not guarantee its required redaction exclusions")
            }
            Self::MissingRecoveryPath { row_id } => {
                write!(f, "row {row_id} is narrowed but offers no recovery path")
            }
            Self::ExactRowNotClean { row_id } => write!(
                f,
                "row {row_id} restores exact but narrows a condition or carries a downgrade reason"
            ),
            Self::SurfacePersistsNothing { row_id } => {
                write!(f, "surface row {row_id} persists no artifact class")
            }
            Self::SurfaceUnknownArtifactClass { row_id, class } => write!(
                f,
                "surface row {row_id} persists artifact class {class} with no matrix row"
            ),
            Self::SurfaceFidelityExceedsClasses {
                row_id,
                surface_fidelity,
                class_fidelity,
            } => write!(
                f,
                "surface row {row_id} claims fidelity {surface_fidelity} above its best class {class_fidelity}"
            ),
            Self::SurfacePortabilityExceedsClasses {
                row_id,
                surface_ownership,
                class_ownership,
            } => write!(
                f,
                "surface row {row_id} claims portability {surface_ownership} above its best class {class_ownership}"
            ),
            Self::SurfacePortabilityUnbacked { row_id } => write!(
                f,
                "surface row {row_id} is portable/shared but persists no exportable class"
            ),
            Self::MissingContinuityCrossLink { surface } => {
                write!(f, "missing canonical cross-link for continuity surface {surface}")
            }
            Self::MissingConsumerBinding { surface } => {
                write!(f, "missing consumer binding for surface {surface}")
            }
            Self::ConsumerBindingDrift { binding_ref } => {
                write!(f, "binding {binding_ref} does not preserve matrix truth")
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the rows"),
        }
    }
}

impl Error for M5SerializationMatrixViolation {}

/// Stable record-kind tag for [`M5SerializationMatrixSupportExport`].
pub const M5_SERIALIZATION_MATRIX_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_serialization_and_restore_matrix_support_export";

/// Support-export wrapper preserving the matrix verbatim for support and evidence packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SerializationMatrixSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub matrix_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Exact matrix preserved by the export.
    pub matrix: M5SerializationMatrix,
}

impl M5SerializationMatrixSupportExport {
    /// Whether the export preserves the same packet id and a clean matrix.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == M5_SERIALIZATION_MATRIX_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == M5_SERIALIZATION_MATRIX_SCHEMA_VERSION
            && self.matrix_packet_id_ref == self.matrix.packet_id
            && self.raw_private_material_excluded
            && self.matrix.validate().is_empty()
    }
}

/// Loads the embedded M5 serialization-and-restore matrix packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5SerializationMatrix`].
pub fn current_m5_serialization_matrix() -> Result<M5SerializationMatrix, serde_json::Error> {
    serde_json::from_str(M5_SERIALIZATION_MATRIX_JSON)
}

#[cfg(test)]
mod tests;
