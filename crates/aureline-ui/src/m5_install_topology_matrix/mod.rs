//! Frozen M5 install-topology, mutable-state-boundary, portable-update, and fleet-rollout execution
//! matrix.
//!
//! This module locks Aureline's concrete delivery-topology behavior into one export-safe packet. Every
//! claimed M5 delivery profile — per-user managed install, per-machine managed install, side-by-side
//! stable-plus-preview, portable mode, and offline / air-gap bundles — is named once here and constrained
//! by the same shared install-topology-role taxonomy (install_mode, updater_owner, binary_root,
//! writable_state_roots, policy_roots, rollback_target, rollout_ring), the same
//! binary-placement-and-updater-ownership-stays-inspectable rule, the same
//! portable-mode-never-spills-machine-global-durable-state rule, the same
//! stable-and-preview-channels-never-corrupt-one-another rule, the same
//! rollback-targets-the-full-artifact-graph rule, and the same
//! rollout-rings-keep-promotion-and-rollback-evidence rule regardless of the surface that renders it.
//!
//! The matrix does not revisit release-center UI or marketplace install-review semantics — it is the
//! shared reusable delivery-topology contract those packaging and rollout lanes consume, and it binds back
//! to the already-landed coexistence / fleet-rollout and native-desktop-integration packets instead of
//! leaving install truth split across scattered packaging prose and hand-copied installer notes. The
//! controlled vocabularies are frozen in one self-describing [`M5InstallTopologyVocabularySet`] rather than
//! minted per surface. The single controlled install-topology-role vocabulary consumers bind to —
//! install_mode, updater_owner, binary_root, writable_state_roots, policy_roots, rollback_target, and
//! rollout_ring — keeps binary placement, updater ownership, writable state roots, policy roots, and
//! rollback targets inspectable; keeps stable and preview channels from corrupting one another; keeps
//! portable mode from spilling durable settings, secrets, or services into hidden machine-global paths;
//! keeps silent and managed flows preserving diagnostics and repair / verify truth; and keeps rollout
//! rings holding promotion and rollback evidence per ring. Raw secret values and private endpoints stay
//! outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_install_topology_matrix,
    seeded_m5_install_topology_matrix_offline_airgap_bundle_preview_narrowed,
    seeded_m5_install_topology_matrix_side_by_side_channel_beta_narrowed,
    M5_INSTALL_TOPOLOGY_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5InstallTopologyMatrixPacket`].
pub const M5_INSTALL_TOPOLOGY_MATRIX_RECORD_KIND: &str =
    "freeze_m5_install_topology_mutable_state_boundary_portable_update_and_fleet_rollout_matrix";

/// Schema version for M5 install-topology matrix records.
pub const M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined install-topology matrix schema.
pub const M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF: &str =
    "schemas/install/m5-install-topology-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_INSTALL_TOPOLOGY_MATRIX_DOC_REF: &str = "docs/install/m5_install_topology_contract.md";

/// Repo-relative path of the canonical install-topology domain schema (install mode, updater owner, and
/// binary-root truth for the per-user, per-machine, and side-by-side families).
pub const M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF: &str =
    "schemas/install/m5-install-topology.schema.json";

/// Repo-relative path of the canonical state-root-boundaries domain schema (writable state roots, policy
/// roots, and isolation truth for the portable and offline / air-gap families).
pub const M5_STATE_ROOT_BOUNDARIES_SCHEMA_REF: &str =
    "schemas/install/m5-state-root-boundaries.schema.json";

/// Repo-relative path of the already-landed coexistence / fleet-rollout schema the matrix binds back to.
pub const M5_COEXISTENCE_AND_FLEET_ROLLOUT_SCHEMA_REF: &str =
    "schemas/install/m5-coexistence-and-fleet-rollout.schema.json";

/// Repo-relative path of the already-landed native-desktop matrix schema the install-topology matrix binds
/// back to.
pub const M5_NATIVE_DESKTOP_MATRIX_SCHEMA_REF: &str =
    "schemas/platform/m5-native-desktop-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_INSTALL_TOPOLOGY_FIXTURE_DIR: &str = "fixtures/install/m5-delivery-topologies";

/// Repo-relative path of the checked support-export artifact.
pub const M5_INSTALL_TOPOLOGY_ARTIFACT_REF: &str =
    "artifacts/release/m5-install-topology-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_INSTALL_TOPOLOGY_CSV_REF: &str =
    "artifacts/release/m5-install-topology-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_INSTALL_TOPOLOGY_REPORT_REF: &str = "artifacts/install/m5-install-topology-matrix.md";

/// One of the five governed delivery-topology families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallTopologyFamily {
    /// Per-user managed install: binary and state scoped to the user profile, per-user updater ownership.
    PerUserManaged,
    /// Per-machine managed install: shared binary root, admin / system updater ownership, machine policy.
    PerMachineManaged,
    /// Side-by-side stable-plus-preview channels living on the same machine without corrupting each other.
    SideBySideStablePreview,
    /// Portable mode: self-contained, colocated state, no hidden machine-global durable spill.
    PortableMode,
    /// Offline / air-gap bundle: bundled artifacts, offline updater ownership, no undisclosed network need.
    OfflineAirgapBundle,
}

impl M5InstallTopologyFamily {
    /// Every governed delivery-topology family, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PerUserManaged,
        Self::PerMachineManaged,
        Self::SideBySideStablePreview,
        Self::PortableMode,
        Self::OfflineAirgapBundle,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PerUserManaged => "per_user_managed",
            Self::PerMachineManaged => "per_machine_managed",
            Self::SideBySideStablePreview => "side_by_side_stable_preview",
            Self::PortableMode => "portable_mode",
            Self::OfflineAirgapBundle => "offline_airgap_bundle",
        }
    }

    /// The canonical per-domain schema ref a downstream surface points at instead of restating this
    /// family's install-mode, updater-ownership, binary-root, or state-root meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::PerUserManaged | Self::PerMachineManaged | Self::SideBySideStablePreview => {
                M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF
            }
            Self::PortableMode | Self::OfflineAirgapBundle => M5_STATE_ROOT_BOUNDARIES_SCHEMA_REF,
        }
    }

    /// `true` when this family must name a controlled per-user-managed-install role.
    pub const fn declares_per_user_managed_install_roles(self) -> bool {
        matches!(self, Self::PerUserManaged)
    }

    /// `true` when this family must name a controlled per-machine-managed-install role.
    pub const fn declares_per_machine_managed_install_roles(self) -> bool {
        matches!(self, Self::PerMachineManaged)
    }

    /// `true` when this family must name a controlled side-by-side-channel role.
    pub const fn declares_side_by_side_channel_roles(self) -> bool {
        matches!(self, Self::SideBySideStablePreview)
    }

    /// `true` when this family must name a controlled portable-mode role.
    pub const fn declares_portable_mode_roles(self) -> bool {
        matches!(self, Self::PortableMode)
    }

    /// `true` when this family must name a controlled offline / air-gap-bundle role.
    pub const fn declares_offline_airgap_bundle_roles(self) -> bool {
        matches!(self, Self::OfflineAirgapBundle)
    }
}

/// The single controlled install-topology-role vocabulary every About, update, diagnostics, admin, docs, or
/// support consumer binds to. These are the exact acceptance-criteria tokens that keep `install_mode`,
/// `updater_owner`, `binary_root`, `writable_state_roots`, `policy_roots`, `rollback_target`, and
/// `rollout_ring` meaning the same thing everywhere the install-topology grammar ships. No surface invents a
/// parallel word for any of these roles, and the ownership-and-isolation roles may never let a topology
/// change hide who owns the updater, spill state into hidden machine-global paths, or narrow rollback below
/// the full artifact graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallTopologyRole {
    /// Install-mode role (per-user / per-machine / side-by-side / portable / offline).
    InstallMode,
    /// Updater-ownership role (who owns and controls the updater).
    UpdaterOwner,
    /// Binary-root role (where the executable and sidecars are placed).
    BinaryRoot,
    /// Writable-state-roots role (settings, caches, services, durable state).
    WritableStateRoots,
    /// Policy-roots role (where bootstrap and managed policy is read from).
    PolicyRoots,
    /// Rollback-target role (the full artifact graph a rollback restores).
    RollbackTarget,
    /// Rollout-ring role (the ring identity that carries promotion and rollback evidence).
    RolloutRing,
}

impl M5InstallTopologyRole {
    /// Every install-topology role token, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::InstallMode,
        Self::UpdaterOwner,
        Self::BinaryRoot,
        Self::WritableStateRoots,
        Self::PolicyRoots,
        Self::RollbackTarget,
        Self::RolloutRing,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstallMode => "install_mode",
            Self::UpdaterOwner => "updater_owner",
            Self::BinaryRoot => "binary_root",
            Self::WritableStateRoots => "writable_state_roots",
            Self::PolicyRoots => "policy_roots",
            Self::RollbackTarget => "rollback_target",
            Self::RolloutRing => "rollout_ring",
        }
    }

    /// Whether this role carries ownership or state-isolation truth whose per-topology behavior must never
    /// hide updater ownership, spill durable state into hidden machine-global paths, corrupt a coexisting
    /// channel, or narrow rollback below the full artifact graph (`updater_owner`, `writable_state_roots`,
    /// `policy_roots`, `rollback_target`). The descriptive placement / identity roles (`install_mode`,
    /// `binary_root`, `rollout_ring`) are inspectable descriptors rather than ownership-carrying isolation
    /// and so do not carry this requirement.
    pub const fn must_preserve_state_isolation_and_ownership_under_coexistence(self) -> bool {
        matches!(
            self,
            Self::UpdaterOwner
                | Self::WritableStateRoots
                | Self::PolicyRoots
                | Self::RollbackTarget
        )
    }
}

/// Controlled per-user-managed-install role — how a per-user managed install is named, so the user-scoped
/// binary root, per-user updater ownership, and user-writable state root follow one topology registry
/// rather than spilling durable state into hidden machine-global paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PerUserManagedInstallRole {
    /// User-scoped binary root.
    UserScopedBinaryRoot,
    /// Per-user updater ownership.
    PerUserUpdaterOwnership,
    /// User-writable state root.
    UserWritableStateRoot,
    /// User-scoped policy root.
    UserScopedPolicyRoot,
    /// A role bound to the single topology registry.
    BoundToTopologyRegistry,
    /// A machine-global durable-state spill, which is disallowed.
    MachineGlobalStateSpillDisallowed,
}

impl M5PerUserManagedInstallRole {
    /// Every per-user-managed-install role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::UserScopedBinaryRoot,
        Self::PerUserUpdaterOwnership,
        Self::UserWritableStateRoot,
        Self::UserScopedPolicyRoot,
        Self::BoundToTopologyRegistry,
        Self::MachineGlobalStateSpillDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserScopedBinaryRoot => "user_scoped_binary_root",
            Self::PerUserUpdaterOwnership => "per_user_updater_ownership",
            Self::UserWritableStateRoot => "user_writable_state_root",
            Self::UserScopedPolicyRoot => "user_scoped_policy_root",
            Self::BoundToTopologyRegistry => "bound_to_topology_registry",
            Self::MachineGlobalStateSpillDisallowed => "machine_global_state_spill_disallowed",
        }
    }
}

/// Controlled per-machine-managed-install role — how a per-machine managed install is named, so the
/// machine-scoped binary root, admin-owned updater, shared machine state root, and machine policy root
/// follow one topology registry rather than hiding updater ownership or admin control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PerMachineManagedInstallRole {
    /// Machine-scoped binary root.
    MachineScopedBinaryRoot,
    /// Admin-owned updater.
    AdminOwnedUpdater,
    /// Shared machine state root.
    SharedMachineStateRoot,
    /// Machine policy root.
    MachinePolicyRoot,
    /// A role bound to the single topology registry.
    BoundToTopologyRegistry,
    /// A hidden updater ownership or admin control, which is disallowed.
    HiddenUpdaterOwnershipDisallowed,
}

impl M5PerMachineManagedInstallRole {
    /// Every per-machine-managed-install role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::MachineScopedBinaryRoot,
        Self::AdminOwnedUpdater,
        Self::SharedMachineStateRoot,
        Self::MachinePolicyRoot,
        Self::BoundToTopologyRegistry,
        Self::HiddenUpdaterOwnershipDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MachineScopedBinaryRoot => "machine_scoped_binary_root",
            Self::AdminOwnedUpdater => "admin_owned_updater",
            Self::SharedMachineStateRoot => "shared_machine_state_root",
            Self::MachinePolicyRoot => "machine_policy_root",
            Self::BoundToTopologyRegistry => "bound_to_topology_registry",
            Self::HiddenUpdaterOwnershipDisallowed => "hidden_updater_ownership_disallowed",
        }
    }
}

/// Controlled side-by-side-channel role — how coexisting stable and preview channels are named, so the
/// isolated channel binary root, isolated channel state namespace, explicit cross-channel handoff, and
/// per-channel rollback target follow one topology registry rather than sharing a state namespace without a
/// handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SideBySideChannelRole {
    /// Isolated channel binary root.
    IsolatedChannelBinaryRoot,
    /// Isolated channel state namespace.
    IsolatedChannelStateNamespace,
    /// Explicit cross-channel import / handoff.
    ExplicitCrossChannelHandoff,
    /// Per-channel rollback target.
    PerChannelRollbackTarget,
    /// A role bound to the single topology registry.
    BoundToTopologyRegistry,
    /// A shared state namespace without an explicit handoff, which is disallowed.
    SharedStateNamespaceWithoutHandoffDisallowed,
}

impl M5SideBySideChannelRole {
    /// Every side-by-side-channel role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::IsolatedChannelBinaryRoot,
        Self::IsolatedChannelStateNamespace,
        Self::ExplicitCrossChannelHandoff,
        Self::PerChannelRollbackTarget,
        Self::BoundToTopologyRegistry,
        Self::SharedStateNamespaceWithoutHandoffDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IsolatedChannelBinaryRoot => "isolated_channel_binary_root",
            Self::IsolatedChannelStateNamespace => "isolated_channel_state_namespace",
            Self::ExplicitCrossChannelHandoff => "explicit_cross_channel_handoff",
            Self::PerChannelRollbackTarget => "per_channel_rollback_target",
            Self::BoundToTopologyRegistry => "bound_to_topology_registry",
            Self::SharedStateNamespaceWithoutHandoffDisallowed => {
                "shared_state_namespace_without_handoff_disallowed"
            }
        }
    }
}

/// Controlled portable-mode role — how portable mode is named, so the self-contained binary root, colocated
/// writable state root, and disclosed portable limitations follow one topology registry rather than
/// spilling hidden machine-global durable state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PortableModeRole {
    /// Self-contained binary root.
    SelfContainedBinaryRoot,
    /// Colocated writable state root.
    ColocatedWritableStateRoot,
    /// No machine-global spill of durable settings, secrets, or services.
    NoMachineGlobalSpill,
    /// Disclosed portable-mode limitations.
    DisclosedPortableLimitations,
    /// A role bound to the single topology registry.
    BoundToTopologyRegistry,
    /// A hidden machine-global durable-state write, which is disallowed.
    HiddenMachineGlobalDurableStateDisallowed,
}

impl M5PortableModeRole {
    /// Every portable-mode role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SelfContainedBinaryRoot,
        Self::ColocatedWritableStateRoot,
        Self::NoMachineGlobalSpill,
        Self::DisclosedPortableLimitations,
        Self::BoundToTopologyRegistry,
        Self::HiddenMachineGlobalDurableStateDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelfContainedBinaryRoot => "self_contained_binary_root",
            Self::ColocatedWritableStateRoot => "colocated_writable_state_root",
            Self::NoMachineGlobalSpill => "no_machine_global_spill",
            Self::DisclosedPortableLimitations => "disclosed_portable_limitations",
            Self::BoundToTopologyRegistry => "bound_to_topology_registry",
            Self::HiddenMachineGlobalDurableStateDisallowed => {
                "hidden_machine_global_durable_state_disallowed"
            }
        }
    }
}

/// Controlled offline / air-gap-bundle role — how an offline or air-gap bundle is named, so the bundled
/// artifact root, offline updater ownership, bundled policy root, and complete rollback-target set follow
/// one topology registry rather than hiding an undisclosed network dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OfflineAirgapBundleRole {
    /// Bundled artifact root.
    BundledArtifactRoot,
    /// Offline updater ownership.
    OfflineUpdaterOwnership,
    /// Bundled policy root.
    BundledPolicyRoot,
    /// Complete rollback-target set (full artifact graph, not just the primary executable).
    CompleteRollbackTargetSet,
    /// A role bound to the single topology registry.
    BoundToTopologyRegistry,
    /// An undisclosed network dependency, which is disallowed.
    UndisclosedNetworkDependencyDisallowed,
}

impl M5OfflineAirgapBundleRole {
    /// Every offline / air-gap-bundle role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BundledArtifactRoot,
        Self::OfflineUpdaterOwnership,
        Self::BundledPolicyRoot,
        Self::CompleteRollbackTargetSet,
        Self::BoundToTopologyRegistry,
        Self::UndisclosedNetworkDependencyDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BundledArtifactRoot => "bundled_artifact_root",
            Self::OfflineUpdaterOwnership => "offline_updater_ownership",
            Self::BundledPolicyRoot => "bundled_policy_root",
            Self::CompleteRollbackTargetSet => "complete_rollback_target_set",
            Self::BoundToTopologyRegistry => "bound_to_topology_registry",
            Self::UndisclosedNetworkDependencyDisallowed => {
                "undisclosed_network_dependency_disallowed"
            }
        }
    }
}

/// Claimed M5 surface family that renders / consumes an install-topology family. No family may invent a
/// parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallTopologySurfaceFamily {
    /// The About surface.
    About,
    /// The update surface.
    Update,
    /// The diagnostics surface.
    Diagnostics,
    /// The admin surface.
    Admin,
    /// The docs / help surface.
    DocsHelp,
    /// The support export.
    SupportExport,
}

impl M5InstallTopologySurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::About,
        Self::Update,
        Self::Diagnostics,
        Self::Admin,
        Self::DocsHelp,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::About => "about",
            Self::Update => "update",
            Self::Diagnostics => "diagnostics",
            Self::Admin => "admin",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
        }
    }
}

/// Deployment line a family must survive with the same truth, so a family's install-mode, updater-ownership,
/// binary-root, state-root, policy-root, or rollback meaning never silently narrows or widens between
/// deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallTopologyDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5InstallTopologyDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Subsystem that consumes a family's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallTopologyConsumerSurface {
    /// The updater service.
    UpdaterService,
    /// The shell / About UI.
    ShellUi,
    /// The diagnostics surface.
    Diagnostics,
    /// The admin surface.
    Admin,
    /// The installer.
    Installer,
    /// The docs / help surface.
    DocsHelp,
    /// The CLI / export path.
    CliExport,
    /// The support export.
    SupportExport,
    /// The general product UI.
    ProductUi,
}

impl M5InstallTopologyConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::UpdaterService,
        Self::ShellUi,
        Self::Diagnostics,
        Self::Admin,
        Self::Installer,
        Self::DocsHelp,
        Self::CliExport,
        Self::SupportExport,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UpdaterService => "updater_service",
            Self::ShellUi => "shell_ui",
            Self::Diagnostics => "diagnostics",
            Self::Admin => "admin",
            Self::Installer => "installer",
            Self::DocsHelp => "docs_help",
            Self::CliExport => "cli_export",
            Self::SupportExport => "support_export",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every family must offer so no install-topology meaning disappears under
/// zoom, high contrast, keyboard-only use, or export. Records the keyboard, screen-reader, high-zoom,
/// high-contrast, CLI/export, and support-packet requirements up front.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallTopologyAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader (via a non-visual cue / label).
    ScreenReaderAnnounced,
    /// Reflows legibly at high zoom.
    HighZoomReflow,
    /// Preserves truth under high-contrast and forced-colors modes.
    HighContrastSafe,
    /// Reachable and inspectable through the CLI / export path.
    CliExportable,
    /// Present in the support / export packet, never renderer-only.
    SupportPacketPresent,
}

impl M5InstallTopologyAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::HighZoomReflow,
        Self::HighContrastSafe,
        Self::CliExportable,
        Self::SupportPacketPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::CliExportable => "cli_exportable",
            Self::SupportPacketPresent => "support_packet_present",
        }
    }
}

/// Reason an install-topology family has degraded below its qualified state. Required on every row so a
/// stale, unresolved, or narrowed fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallTopologyDegradedReason {
    /// Proof has gone stale.
    ProofStale,
    /// The install-topology registry source is unavailable.
    TopologyRegistrySourceUnavailable,
    /// The state-root-boundary source is unavailable.
    StateRootBoundarySourceUnavailable,
    /// Updater ownership is unverified.
    UpdaterOwnershipUnverified,
    /// Rollback completeness is unverified.
    RollbackCompletenessUnverified,
    /// Rollout-ring evidence is unavailable.
    RolloutRingEvidenceUnavailable,
}

impl M5InstallTopologyDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::TopologyRegistrySourceUnavailable,
        Self::StateRootBoundarySourceUnavailable,
        Self::UpdaterOwnershipUnverified,
        Self::RollbackCompletenessUnverified,
        Self::RolloutRingEvidenceUnavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::TopologyRegistrySourceUnavailable => "topology_registry_source_unavailable",
            Self::StateRootBoundarySourceUnavailable => "state_root_boundary_source_unavailable",
            Self::UpdaterOwnershipUnverified => "updater_ownership_unverified",
            Self::RollbackCompletenessUnverified => "rollback_completeness_unverified",
            Self::RolloutRingEvidenceUnavailable => "rollout_ring_evidence_unavailable",
        }
    }
}

/// Mandatory label a claimed install-topology family must be able to show. The first three are hard
/// requirements on every family; the remaining three close the acceptance-criteria ambiguity about the
/// install mode, the writable state root, and the rollback target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallTopologyRequiredLabel {
    /// The family's stable identity.
    Identity,
    /// The family's install-topology role.
    SemanticRole,
    /// The canonical registry reference the family points at.
    RegistryReference,
    /// The install mode the family delivers.
    InstallMode,
    /// The writable state root the family owns.
    StateRoot,
    /// The rollback target the family restores.
    RollbackTarget,
}

impl M5InstallTopologyRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::InstallMode,
        Self::StateRoot,
        Self::RollbackTarget,
    ];

    /// The three labels every claimed family must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::RegistryReference];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::RegistryReference => "registry_reference",
            Self::InstallMode => "install_mode",
            Self::StateRoot => "state_root",
            Self::RollbackTarget => "rollback_target",
        }
    }
}

/// Qualification class for an M5 install-topology row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallTopologyQualificationClass {
    /// Family qualifies for the Stable claim.
    Stable,
    /// Family is narrowed to Beta.
    Beta,
    /// Family is narrowed to Preview.
    Preview,
    /// Family is experimental and not claimed.
    Experimental,
    /// Family is unavailable on this build.
    Unavailable,
    /// Family is held pending upstream resolution.
    Held,
}

impl M5InstallTopologyQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the family may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows an install-topology family below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallTopologyDowngradeTrigger {
    /// Portable mode wrote hidden machine-global durable state.
    PortableModeWroteHiddenMachineGlobalDurableState,
    /// A preview channel reused a stable state namespace without an explicit import / handoff.
    PreviewChannelReusedStableStateNamespaceWithoutHandoff,
    /// A rollback targeted only the primary executable while sidecars or metadata drifted.
    RollbackTargetedPrimaryExecutableWhileSidecarsDrifted,
    /// Updater ownership or admin control was hidden in a managed flow.
    UpdaterOwnershipOrAdminControlHiddenInManagedFlow,
    /// A deployment claim outpaced ring or repair / verify evidence.
    DeploymentClaimOutpacedRingOrRepairVerifyEvidence,
    /// A state-root boundary drifted by topology instead of following one registry.
    StateRootBoundaryDriftedByTopology,
    /// A family left its install mode unstated.
    InstallModeUnstated,
    /// A family left its writable state root unstated.
    StateRootUnstated,
    /// A family left its rollback target unstated.
    RollbackTargetUnstated,
    /// A family left its canonical registry reference unstated.
    RegistryReferenceUnstated,
    /// A family left its updater owner unstated.
    UpdaterOwnerUnstated,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5InstallTopologyDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::PortableModeWroteHiddenMachineGlobalDurableState,
        Self::PreviewChannelReusedStableStateNamespaceWithoutHandoff,
        Self::RollbackTargetedPrimaryExecutableWhileSidecarsDrifted,
        Self::UpdaterOwnershipOrAdminControlHiddenInManagedFlow,
        Self::DeploymentClaimOutpacedRingOrRepairVerifyEvidence,
        Self::StateRootBoundaryDriftedByTopology,
        Self::InstallModeUnstated,
        Self::StateRootUnstated,
        Self::RollbackTargetUnstated,
        Self::RegistryReferenceUnstated,
        Self::UpdaterOwnerUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PortableModeWroteHiddenMachineGlobalDurableState => {
                "portable_mode_wrote_hidden_machine_global_durable_state"
            }
            Self::PreviewChannelReusedStableStateNamespaceWithoutHandoff => {
                "preview_channel_reused_stable_state_namespace_without_handoff"
            }
            Self::RollbackTargetedPrimaryExecutableWhileSidecarsDrifted => {
                "rollback_targeted_primary_executable_while_sidecars_drifted"
            }
            Self::UpdaterOwnershipOrAdminControlHiddenInManagedFlow => {
                "updater_ownership_or_admin_control_hidden_in_managed_flow"
            }
            Self::DeploymentClaimOutpacedRingOrRepairVerifyEvidence => {
                "deployment_claim_outpaced_ring_or_repair_verify_evidence"
            }
            Self::StateRootBoundaryDriftedByTopology => "state_root_boundary_drifted_by_topology",
            Self::InstallModeUnstated => "install_mode_unstated",
            Self::StateRootUnstated => "state_root_unstated",
            Self::RollbackTargetUnstated => "rollback_target_unstated",
            Self::RegistryReferenceUnstated => "registry_reference_unstated",
            Self::UpdaterOwnerUnstated => "updater_owner_unstated",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed install-topology family bound to the surface-specific truth it must
/// project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallTopologyRow {
    /// Governed install-topology family.
    pub install_topology_family: M5InstallTopologyFamily,
    /// Qualification class earned by this family.
    pub qualification: M5InstallTopologyQualificationClass,
    /// Owner role accountable for keeping this family governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this family.
    pub surface_families: Vec<M5InstallTopologySurfaceFamily>,
    /// Deployment lines this family keeps the same truth across.
    pub deployment_lines: Vec<M5InstallTopologyDeploymentLine>,
    /// Mandatory labels this family must be able to show (must include the three
    /// [`M5InstallTopologyRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5InstallTopologyRequiredLabel>,
    /// Install-topology roles this family can carry (the frozen AC vocabulary; required on every family).
    pub semantic_roles: Vec<M5InstallTopologyRole>,
    /// Per-user-managed-install roles this family names (per-user-managed family only).
    pub per_user_managed_install_roles: Vec<M5PerUserManagedInstallRole>,
    /// Per-machine-managed-install roles this family names (per-machine-managed family only).
    pub per_machine_managed_install_roles: Vec<M5PerMachineManagedInstallRole>,
    /// Side-by-side-channel roles this family names (side-by-side family only).
    pub side_by_side_channel_roles: Vec<M5SideBySideChannelRole>,
    /// Portable-mode roles this family names (portable-mode family only).
    pub portable_mode_roles: Vec<M5PortableModeRole>,
    /// Offline / air-gap-bundle roles this family names (offline / air-gap family only).
    pub offline_airgap_bundle_roles: Vec<M5OfflineAirgapBundleRole>,
    /// Degraded reasons this family can name (required on every family).
    pub degraded_reasons: Vec<M5InstallTopologyDegradedReason>,
    /// Non-visual accessibility routes this family offers.
    pub accessibility_routes: Vec<M5InstallTopologyAccessibilityRoute>,
    /// Subsystems that consume this family's projection.
    pub consumer_surfaces: Vec<M5InstallTopologyConsumerSurface>,
    /// Downgrade triggers that apply to this family.
    pub downgrade_triggers: Vec<M5InstallTopologyDowngradeTrigger>,
    /// Proof packet refs that keep this family current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this family (must include its own canonical domain schema so
    /// downstream surfaces have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this family never lets portable mode write hidden machine-global durable state. MUST
    /// be `false`.
    pub portable_mode_writes_hidden_machine_global_durable_state: bool,
    /// Hard invariant: this family never lets a preview channel reuse a stable state namespace without an
    /// explicit import / handoff. MUST be `false`.
    pub preview_channel_reuses_stable_state_namespace_without_handoff: bool,
    /// Hard invariant: this family never rolls back only the primary executable while sidecars or metadata
    /// drift. MUST be `false`.
    pub rollback_targets_primary_executable_while_sidecars_drift: bool,
    /// Hard invariant: this family never hides updater ownership or admin control in a managed flow. MUST be
    /// `false`.
    pub hides_updater_ownership_or_admin_control_in_managed_flow: bool,
    /// Hard invariant: this family never publishes a deployment claim that outpaces ring or repair / verify
    /// evidence. MUST be `false`.
    pub publishes_deployment_claim_outpacing_ring_or_repair_verify_evidence: bool,
}

impl M5InstallTopologyRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5InstallTopologyRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5InstallTopologyRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.portable_mode_writes_hidden_machine_global_durable_state
            && !self.preview_channel_reuses_stable_state_namespace_without_handoff
            && !self.rollback_targets_primary_executable_while_sidecars_drift
            && !self.hides_updater_ownership_or_admin_control_in_managed_flow
            && !self.publishes_deployment_claim_outpacing_ring_or_repair_verify_evidence
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallTopologyVocabularySet {
    /// Install-topology-family tokens.
    pub install_topology_families: Vec<String>,
    /// Install-topology-role tokens.
    pub semantic_roles: Vec<String>,
    /// Per-user-managed-install-role tokens.
    pub per_user_managed_install_roles: Vec<String>,
    /// Per-machine-managed-install-role tokens.
    pub per_machine_managed_install_roles: Vec<String>,
    /// Side-by-side-channel-role tokens.
    pub side_by_side_channel_roles: Vec<String>,
    /// Portable-mode-role tokens.
    pub portable_mode_roles: Vec<String>,
    /// Offline / air-gap-bundle-role tokens.
    pub offline_airgap_bundle_roles: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded-reason tokens.
    pub degraded_reasons: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
    /// Downgrade-trigger tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5InstallTopologyVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            install_topology_families: tokens(&M5InstallTopologyFamily::ALL, |v| v.as_str()),
            semantic_roles: tokens(&M5InstallTopologyRole::ALL, |v| v.as_str()),
            per_user_managed_install_roles: tokens(&M5PerUserManagedInstallRole::ALL, |v| {
                v.as_str()
            }),
            per_machine_managed_install_roles: tokens(&M5PerMachineManagedInstallRole::ALL, |v| {
                v.as_str()
            }),
            side_by_side_channel_roles: tokens(&M5SideBySideChannelRole::ALL, |v| v.as_str()),
            portable_mode_roles: tokens(&M5PortableModeRole::ALL, |v| v.as_str()),
            offline_airgap_bundle_roles: tokens(&M5OfflineAirgapBundleRole::ALL, |v| v.as_str()),
            surface_families: tokens(&M5InstallTopologySurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5InstallTopologyDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5InstallTopologyConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5InstallTopologyAccessibilityRoute::ALL, |v| v.as_str()),
            degraded_reasons: tokens(&M5InstallTopologyDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5InstallTopologyRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5InstallTopologyDowngradeTrigger::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallTopologyGovernanceReview {
    /// Binary placement and updater ownership stay inspectable.
    pub binary_placement_and_updater_ownership_inspectable: bool,
    /// Portable mode never spills durable settings, secrets, or services into hidden machine-global paths.
    pub portable_mode_never_spills_machine_global_durable_state: bool,
    /// Stable and preview channels never corrupt one another.
    pub stable_and_preview_channels_never_corrupt_one_another: bool,
    /// Silent and managed flows preserve diagnostics and repair / verify truth.
    pub silent_and_managed_flows_preserve_diagnostics_and_repair_verify: bool,
    /// Rollback targets the full artifact graph, not just the primary executable.
    pub rollback_targets_full_artifact_graph_not_just_primary_executable: bool,
    /// Rollout rings keep promotion and rollback evidence per ring.
    pub rollout_rings_keep_promotion_and_rollback_evidence: bool,
    /// Updater ownership is never hidden in a managed flow.
    pub updater_ownership_never_hidden_in_managed_flow: bool,
    /// Preview channels require an explicit import or handoff before reusing stable state.
    pub preview_channel_requires_explicit_import_or_handoff: bool,
    /// Deployment claims never outpace ring or repair / verify evidence.
    pub deployment_claims_never_outpace_ring_or_repair_verify_evidence: bool,
    /// Every family keeps the same truth across every deployment line.
    pub every_family_declares_deployment_lines: bool,
    /// Every family declares a non-visual accessibility route.
    pub every_family_declares_accessibility_route: bool,
    /// Support / export reads a single canonical install-topology source.
    pub support_export_reads_single_install_topology_source: bool,
    /// About, update, diagnostics, and admin bind to a single canonical install-topology source.
    pub about_update_diagnostics_admin_bind_to_single_install_topology_source: bool,
    /// Later M5 rows cannot invent parallel install vocabulary.
    pub later_rows_cannot_invent_parallel_install_vocabulary: bool,
    /// Install truth survives zoom and high contrast.
    pub install_truth_survives_zoom_and_high_contrast: bool,
    /// Claims narrow automatically when the registry is missing, stale, or not yet qualified.
    pub claims_narrow_automatically_when_registry_missing_or_stale: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallTopologyConsumerProjection {
    /// About and update consume the shared install-topology truth.
    pub about_and_update_consume_shared_install_topology_truth: bool,
    /// Diagnostics and admin consume the shared state-root boundaries.
    pub diagnostics_and_admin_consume_shared_state_root_boundaries: bool,
    /// Installers consume the shared binary and state roots.
    pub installers_consume_shared_binary_and_state_roots: bool,
    /// Docs, help, and screenshots read a single install-topology source.
    pub docs_help_and_screenshots_read_single_install_topology_source: bool,
    /// Rollout tooling binds to the shared ring evidence.
    pub rollout_tooling_binds_to_shared_ring_evidence: bool,
    /// Support / export reads a single canonical install-topology source.
    pub support_export_reads_single_install_topology_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallTopologyProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the family.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the install-topology lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallTopologyReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting install-topology audit for the lane.
    pub install_topology_audit_ref: String,
    /// True when support/export parity is required for every family.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every family.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5InstallTopologyMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5InstallTopologyMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Install-topology rows.
    pub install_topology_rows: Vec<M5InstallTopologyRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5InstallTopologyVocabularySet,
    /// Governance-review block.
    pub governance_review: M5InstallTopologyGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5InstallTopologyConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5InstallTopologyProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5InstallTopologyReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 install-topology matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallTopologyMatrixPacket {
    /// Record kind; must equal [`M5_INSTALL_TOPOLOGY_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Install-topology rows.
    pub install_topology_rows: Vec<M5InstallTopologyRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5InstallTopologyVocabularySet,
    /// Governance-review block.
    pub governance_review: M5InstallTopologyGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5InstallTopologyConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5InstallTopologyProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5InstallTopologyReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5InstallTopologyMatrixPacket {
    /// Builds an M5 install-topology matrix packet from stable-lane input.
    pub fn new(input: M5InstallTopologyMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_INSTALL_TOPOLOGY_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            install_topology_rows: input.install_topology_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 install-topology matrix invariants.
    pub fn validate(&self) -> Vec<M5InstallTopologyMatrixViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_INSTALL_TOPOLOGY_MATRIX_RECORD_KIND {
            violations.push(M5InstallTopologyMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_VERSION {
            violations.push(M5InstallTopologyMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5InstallTopologyMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_install_topology_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 install-topology matrix serializes"),
        ) {
            violations.push(M5InstallTopologyMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 install-topology matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "install_topology_family,qualification,owner,canonical_schema,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.install_topology_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.install_topology_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.install_topology_family.canonical_domain_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_families = self
            .install_topology_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Install-Topology, Mutable-State-Boundary, Portable-Update, and Fleet-Rollout Execution Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Install-topology families: {} ({} stable)\n",
            self.install_topology_rows.len(),
            stable_families
        ));
        out.push_str(&format!(
            "- Install-topology roles: {}\n",
            self.vocabulary_set.semantic_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Per-user-managed-install roles: {}\n",
            self.vocabulary_set
                .per_user_managed_install_roles
                .join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Install-topology families\n\n");
        for row in &self.install_topology_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.install_topology_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.install_topology_family.canonical_domain_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 install-topology matrix export.
#[derive(Debug)]
pub enum M5InstallTopologyMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5InstallTopologyMatrixViolation>),
}

impl fmt::Display for M5InstallTopologyMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 install-topology matrix export parse failed: {error}"
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
                    "m5 install-topology matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5InstallTopologyMatrixArtifactError {}

/// Validation failures emitted by [`M5InstallTopologyMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5InstallTopologyMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed install-topology family is missing from the matrix.
    RequiredFamilyMissing,
    /// An install-topology row is incomplete.
    InstallTopologyRowIncomplete,
    /// An install-topology row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// An install-topology row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A family declares no install-topology roles.
    SemanticRoleMissing,
    /// The per-user-managed family declares no per-user-managed-install roles.
    PerUserManagedInstallRoleMissing,
    /// The per-machine-managed family declares no per-machine-managed-install roles.
    PerMachineManagedInstallRoleMissing,
    /// The side-by-side family declares no side-by-side-channel roles.
    SideBySideChannelRoleMissing,
    /// The portable-mode family declares no portable-mode roles.
    PortableModeRoleMissing,
    /// The offline / air-gap family declares no offline / air-gap-bundle roles.
    OfflineAirgapBundleRoleMissing,
    /// A family declares no degraded reasons.
    DegradedReasonMissing,
    /// A family declares no surface families.
    SurfaceFamilyMissing,
    /// A family declares no deployment lines.
    DeploymentLineMissing,
    /// A family declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A family declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A family declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A family claiming Stable is missing required proof packet refs.
    StableFamilyMissingProof,
    /// A family violates a hard invariant (portable mode spilling hidden machine-global durable state, a
    /// preview channel reusing a stable state namespace without a handoff, a rollback targeting only the
    /// primary executable while sidecars drift, updater ownership or admin control hidden in a managed flow,
    /// or a deployment claim outpacing ring or repair / verify evidence).
    InstallTopologyInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5InstallTopologyMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredFamilyMissing => "required_family_missing",
            Self::InstallTopologyRowIncomplete => "install_topology_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::PerUserManagedInstallRoleMissing => "per_user_managed_install_role_missing",
            Self::PerMachineManagedInstallRoleMissing => "per_machine_managed_install_role_missing",
            Self::SideBySideChannelRoleMissing => "side_by_side_channel_role_missing",
            Self::PortableModeRoleMissing => "portable_mode_role_missing",
            Self::OfflineAirgapBundleRoleMissing => "offline_airgap_bundle_role_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableFamilyMissingProof => "stable_family_missing_proof",
            Self::InstallTopologyInvariantViolated => "install_topology_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 install-topology matrix export.
pub fn current_stable_m5_install_topology_matrix_export(
) -> Result<M5InstallTopologyMatrixPacket, M5InstallTopologyMatrixArtifactError> {
    let packet: M5InstallTopologyMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-install-topology-proof/support_export.json"
    )))
    .map_err(M5InstallTopologyMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5InstallTopologyMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5InstallTopologyMatrixPacket,
    violations: &mut Vec<M5InstallTopologyMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
        M5_INSTALL_TOPOLOGY_MATRIX_DOC_REF,
        M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF,
        M5_STATE_ROOT_BOUNDARIES_SCHEMA_REF,
        M5_COEXISTENCE_AND_FLEET_ROLLOUT_SCHEMA_REF,
        M5_NATIVE_DESKTOP_MATRIX_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5InstallTopologyMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5InstallTopologyMatrixPacket,
    violations: &mut Vec<M5InstallTopologyMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5InstallTopologyMatrixViolation::VocabularySetDrift);
    }
}

fn validate_install_topology_rows(
    packet: &M5InstallTopologyMatrixPacket,
    violations: &mut Vec<M5InstallTopologyMatrixViolation>,
) {
    let present: BTreeSet<M5InstallTopologyFamily> = packet
        .install_topology_rows
        .iter()
        .map(|row| row.install_topology_family)
        .collect();
    for required in M5InstallTopologyFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5InstallTopologyMatrixViolation::RequiredFamilyMissing);
            return;
        }
    }

    for row in &packet.install_topology_rows {
        let family = row.install_topology_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5InstallTopologyMatrixViolation::InstallTopologyRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5InstallTopologyMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == family.canonical_domain_schema_ref())
        {
            violations.push(M5InstallTopologyMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5InstallTopologyMatrixViolation::SemanticRoleMissing);
        }
        if family.declares_per_user_managed_install_roles()
            && row.per_user_managed_install_roles.is_empty()
        {
            violations.push(M5InstallTopologyMatrixViolation::PerUserManagedInstallRoleMissing);
        }
        if family.declares_per_machine_managed_install_roles()
            && row.per_machine_managed_install_roles.is_empty()
        {
            violations.push(M5InstallTopologyMatrixViolation::PerMachineManagedInstallRoleMissing);
        }
        if family.declares_side_by_side_channel_roles() && row.side_by_side_channel_roles.is_empty()
        {
            violations.push(M5InstallTopologyMatrixViolation::SideBySideChannelRoleMissing);
        }
        if family.declares_portable_mode_roles() && row.portable_mode_roles.is_empty() {
            violations.push(M5InstallTopologyMatrixViolation::PortableModeRoleMissing);
        }
        if family.declares_offline_airgap_bundle_roles()
            && row.offline_airgap_bundle_roles.is_empty()
        {
            violations.push(M5InstallTopologyMatrixViolation::OfflineAirgapBundleRoleMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5InstallTopologyMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5InstallTopologyMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5InstallTopologyMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5InstallTopologyMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5InstallTopologyMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5InstallTopologyMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5InstallTopologyMatrixViolation::StableFamilyMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5InstallTopologyMatrixViolation::InstallTopologyInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5InstallTopologyMatrixPacket,
    violations: &mut Vec<M5InstallTopologyMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.binary_placement_and_updater_ownership_inspectable,
        review.portable_mode_never_spills_machine_global_durable_state,
        review.stable_and_preview_channels_never_corrupt_one_another,
        review.silent_and_managed_flows_preserve_diagnostics_and_repair_verify,
        review.rollback_targets_full_artifact_graph_not_just_primary_executable,
        review.rollout_rings_keep_promotion_and_rollback_evidence,
        review.updater_ownership_never_hidden_in_managed_flow,
        review.preview_channel_requires_explicit_import_or_handoff,
        review.deployment_claims_never_outpace_ring_or_repair_verify_evidence,
        review.every_family_declares_deployment_lines,
        review.every_family_declares_accessibility_route,
        review.support_export_reads_single_install_topology_source,
        review.about_update_diagnostics_admin_bind_to_single_install_topology_source,
        review.later_rows_cannot_invent_parallel_install_vocabulary,
        review.install_truth_survives_zoom_and_high_contrast,
        review.claims_narrow_automatically_when_registry_missing_or_stale,
    ] {
        if !ok {
            violations.push(M5InstallTopologyMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5InstallTopologyMatrixPacket,
    violations: &mut Vec<M5InstallTopologyMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.about_and_update_consume_shared_install_topology_truth,
        projection.diagnostics_and_admin_consume_shared_state_root_boundaries,
        projection.installers_consume_shared_binary_and_state_roots,
        projection.docs_help_and_screenshots_read_single_install_topology_source,
        projection.rollout_tooling_binds_to_shared_ring_evidence,
        projection.support_export_reads_single_install_topology_source,
    ] {
        if !ok {
            violations.push(M5InstallTopologyMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5InstallTopologyMatrixPacket,
    violations: &mut Vec<M5InstallTopologyMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5InstallTopologyMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5InstallTopologyMatrixPacket,
    violations: &mut Vec<M5InstallTopologyMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.install_topology_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5InstallTopologyMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled vocabulary
/// deliberately uses install / topology / updater / state-root / policy-root words; what is rejected is a
/// raw secret *value* shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
