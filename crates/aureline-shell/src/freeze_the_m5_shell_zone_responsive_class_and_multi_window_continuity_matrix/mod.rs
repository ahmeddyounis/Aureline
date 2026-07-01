//! Frozen M5 shell-zone, responsive-class, and multi-window continuity matrix.
//!
//! This module locks the canonical M5 live-shell object model into one
//! export-safe packet. Each [`M5ShellSurfaceRow`] names one claimed M5 surface
//! family — the notebook, data grid, profiler, pipeline, docs, preview, review,
//! incident, companion, and operator surfaces — and binds it to its canonical
//! shell slot, its fallback slot, the dependency-missing placeholder behavior,
//! the responsive classes it must survive, the window classes it may live in,
//! the allowed occupant transitions (side-by-side, tabbed, sheeted, overflowed),
//! the responsive collapse ladder, the owning-window routing expectations, the
//! workspace-global continuity truths every window preserves, evidence
//! requirements, downgrade triggers, rollback posture, source contracts, and
//! consumer-surface parity.
//!
//! The matrix is the single source of truth for whether a claimed M5 surface may
//! assert desktop/shell maturity. Shell, windowing, layout, and status frames,
//! plus docs/help and release-proof packets, consume this one packet rather than
//! re-inventing local slot, collapse, or multi-window prose: new surfaces attach
//! only to declared shell slots; responsive collapse never changes task identity
//! or hides critical state; every window preserves workspace-global trust,
//! remote, profile, and recovery truth while keeping layout local; and dialogs,
//! notifications, and approvals route back to the owning window and object
//! without focus theft or orphaning.
//!
//! The controlled vocabularies mirror the shell-zone/responsive-fallback tokens
//! already owned by the frozen `stabilize_shell_zoning_and_responsive_fallback`
//! contract, the window-topology snapshot, the attention-routing packet, the
//! notification envelope, and the session-restore fidelity contract; the matrix
//! freezes them in one self-describing [`M5ShellVocabularySet`] rather than
//! minting parallel tokens. It references the upstream contracts by id. Raw URLs,
//! raw local paths, raw usernames, raw hostnames, tokens, and credentials stay
//! outside the export boundary.
//!
//! The boundary schema is
//! [`schemas/shell/m5-shell-zone.schema.json`](../../../../schemas/shell/m5-shell-zone.schema.json)
//! with the responsive-class companion schema
//! [`schemas/shell/m5-responsive-class.schema.json`](../../../../schemas/shell/m5-responsive-class.schema.json).
//! The contract doc is
//! [`docs/shell/m5_shell_zone_matrix_contract.md`](../../../../docs/shell/m5_shell_zone_matrix_contract.md).
//! The release-proof directory is
//! [`artifacts/release/m5-shell-continuity-proof/`](../../../../artifacts/release/m5-shell-continuity-proof/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_shell_zone_matrix, seeded_m5_shell_zone_matrix_companion_overlay_narrowed,
    seeded_m5_shell_zone_matrix_profiler_remote_held, M5_SHELL_ZONE_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ShellZoneMatrixPacket`].
pub const M5_SHELL_ZONE_MATRIX_RECORD_KIND: &str =
    "freeze_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix";

/// Schema version for M5 shell-zone matrix records.
pub const M5_SHELL_ZONE_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the shell-zone matrix boundary schema.
pub const M5_SHELL_ZONE_MATRIX_SCHEMA_REF: &str = "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the responsive-class companion schema.
pub const M5_SHELL_RESPONSIVE_CLASS_SCHEMA_REF: &str =
    "schemas/shell/m5-responsive-class.schema.json";

/// Repo-relative path of the shell-zone matrix contract doc.
pub const M5_SHELL_ZONE_MATRIX_DOC_REF: &str = "docs/shell/m5_shell_zone_matrix_contract.md";

/// Frozen shell-zoning / responsive-fallback contract this matrix builds on.
pub const M5_SHELL_ZONING_CONTRACT_REF: &str =
    "shell:stabilize_shell_zoning_and_responsive_fallback:v1";

/// Window-topology snapshot contract this matrix mirrors for multi-window truth.
pub const M5_SHELL_WINDOW_TOPOLOGY_CONTRACT_REF: &str =
    "schemas/workspace/window_topology_snapshot.schema.json";

/// Attention-routing contract this matrix mirrors for owning-window routing.
pub const M5_SHELL_ATTENTION_ROUTING_CONTRACT_REF: &str =
    "schemas/activity/m5-attention-routing.schema.json";

/// Notification-envelope contract this matrix mirrors for routed-action truth.
pub const M5_SHELL_NOTIFICATION_ENVELOPE_CONTRACT_REF: &str =
    "schemas/ux/notification_envelope.schema.json";

/// Session-restore fidelity contract this matrix mirrors for recovery truth.
pub const M5_SHELL_SESSION_RESTORE_CONTRACT_REF: &str =
    "schemas/recovery/session-restore-fidelity.schema.json";

/// Reference-layout contract this matrix mirrors for design-token/slot fidelity.
pub const M5_SHELL_REFERENCE_LAYOUT_CONTRACT_REF: &str =
    "schemas/design-system/m5-reference-layout.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_SHELL_ZONE_MATRIX_FIXTURE_DIR: &str = "fixtures/ui/m5-shell-layouts";

/// Repo-relative path of the checked release-proof support-export artifact.
pub const M5_SHELL_ZONE_MATRIX_ARTIFACT_REF: &str =
    "artifacts/release/m5-shell-continuity-proof/support_export.json";

/// Repo-relative path of the checked Markdown governance summary.
pub const M5_SHELL_ZONE_MATRIX_GOVERNANCE_REF: &str =
    "artifacts/release/m5-shell-continuity-proof/governance.md";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_SHELL_ZONE_MATRIX_CSV_REF: &str =
    "artifacts/release/m5-shell-continuity-proof/matrix.csv";

/// Repo-relative path of the checked human-readable matrix Markdown.
pub const M5_SHELL_ZONE_MATRIX_MARKDOWN_REF: &str = "artifacts/shell/m5-shell-zone-matrix.md";

/// One of the ten claimed M5 shell surface families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellSurfaceFamily {
    /// Notebook editor / cell surface.
    Notebook,
    /// Tabular data grid surface.
    DataGrid,
    /// Profiler / performance analysis surface.
    Profiler,
    /// Pipeline / workflow graph surface.
    Pipeline,
    /// Documentation reader surface.
    Docs,
    /// Preview surface (render, diff, media).
    Preview,
    /// Review / change-request surface.
    Review,
    /// Incident / operations-response surface.
    Incident,
    /// Companion assistant surface.
    Companion,
    /// Operator / control-plane surface.
    Operator,
}

impl M5ShellSurfaceFamily {
    /// Every governed family, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::Notebook,
        Self::DataGrid,
        Self::Profiler,
        Self::Pipeline,
        Self::Docs,
        Self::Preview,
        Self::Review,
        Self::Incident,
        Self::Companion,
        Self::Operator,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::DataGrid => "data_grid",
            Self::Profiler => "profiler",
            Self::Pipeline => "pipeline",
            Self::Docs => "docs",
            Self::Preview => "preview",
            Self::Review => "review",
            Self::Incident => "incident",
            Self::Companion => "companion",
            Self::Operator => "operator",
        }
    }

    /// Controlled state vocabularies every shell surface MUST declare.
    ///
    /// Every claimed M5 shell family lives under all six controlled shell
    /// vocabularies; a family cannot opt out of declaring how it behaves under
    /// resize, detach, multi-window, or routed-action pressure.
    pub fn required_state_vocabularies(self) -> &'static [M5ShellStateVocabulary] {
        use M5ShellStateVocabulary as V;
        &[
            V::ResponsiveClass,
            V::WindowClass,
            V::OccupantPersistence,
            V::FallbackPlacement,
            V::OwningWindowRouting,
            V::ContinuityTruth,
        ]
    }
}

/// Qualification class for an M5 shell surface family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellQualificationClass {
    /// Family qualifies for the Stable shell claim.
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

impl M5ShellQualificationClass {
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

    /// Whether the family may carry a public Stable shell claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Canonical shell zone / slot a surface may attach to.
///
/// Mirrors the eight stable shell zones frozen by the
/// `stabilize_shell_zoning_and_responsive_fallback` contract so a new surface can
/// never invent its own slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellZoneSlot {
    /// Title / context bar: workspace, trust, target, profile, and route identity.
    TitleContextBar,
    /// Activity rail: durable top-level route rail.
    ActivityRail,
    /// Left sidebar: structural navigation and query collections.
    LeftSidebar,
    /// Main workspace: editor groups, review surfaces, and primary working sets.
    MainWorkspace,
    /// Right inspector: contextual detail and inspectable evidence.
    RightInspector,
    /// Bottom panel: execution, output, problems, terminal, and timeline panels.
    BottomPanel,
    /// Status bar: persistent instrumentation and compact recovery/status truth.
    StatusBar,
    /// Transient overlay: window-local palettes, dialogs, sheets, and overlays.
    TransientOverlay,
}

impl M5ShellZoneSlot {
    /// Every zone, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::TitleContextBar,
        Self::ActivityRail,
        Self::LeftSidebar,
        Self::MainWorkspace,
        Self::RightInspector,
        Self::BottomPanel,
        Self::StatusBar,
        Self::TransientOverlay,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TitleContextBar => "title_context_bar",
            Self::ActivityRail => "activity_rail",
            Self::LeftSidebar => "left_sidebar",
            Self::MainWorkspace => "main_workspace",
            Self::RightInspector => "right_inspector",
            Self::BottomPanel => "bottom_panel",
            Self::StatusBar => "status_bar",
            Self::TransientOverlay => "transient_overlay",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::TitleContextBar => "Title / Context Bar",
            Self::ActivityRail => "Activity Rail",
            Self::LeftSidebar => "Left Sidebar",
            Self::MainWorkspace => "Main Workspace",
            Self::RightInspector => "Right Inspector",
            Self::BottomPanel => "Bottom Panel",
            Self::StatusBar => "Status Bar",
            Self::TransientOverlay => "Transient Overlay",
        }
    }
}

/// Responsive class resolved before applying the collapse ladder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResponsiveClass {
    /// Compact desktop: narrow width, zoom, or secondary compact display.
    CompactDesktop,
    /// Standard desktop: default working width.
    StandardDesktop,
    /// Expanded desktop: wide primary display.
    ExpandedDesktop,
}

impl M5ResponsiveClass {
    /// Every responsive class, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::CompactDesktop,
        Self::StandardDesktop,
        Self::ExpandedDesktop,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompactDesktop => "compact_desktop",
            Self::StandardDesktop => "standard_desktop",
            Self::ExpandedDesktop => "expanded_desktop",
        }
    }
}

/// Window class a surface family may live in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowClass {
    /// Primary workspace window that owns workspace-global truth.
    PrimaryWorkspaceWindow,
    /// Secondary detached window carrying full workspace-global truth.
    SecondaryDetachedWindow,
    /// Floating utility window scoped to a single object or tool.
    FloatingUtilityWindow,
    /// Companion overlay window attached to an owning window.
    CompanionOverlayWindow,
}

impl M5WindowClass {
    /// Every window class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PrimaryWorkspaceWindow,
        Self::SecondaryDetachedWindow,
        Self::FloatingUtilityWindow,
        Self::CompanionOverlayWindow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryWorkspaceWindow => "primary_workspace_window",
            Self::SecondaryDetachedWindow => "secondary_detached_window",
            Self::FloatingUtilityWindow => "floating_utility_window",
            Self::CompanionOverlayWindow => "companion_overlay_window",
        }
    }
}

/// Allowed occupant transition for a surface family within a zone or window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OccupantPersistence {
    /// May persist side-by-side with a peer surface.
    SideBySide,
    /// May persist as a tab in a shared group.
    Tabbed,
    /// May persist as a sheet attached to its zone.
    Sheeted,
    /// May persist behind a keyboard-reachable overflow route.
    Overflowed,
    /// May persist as a solo docked occupant only.
    SoloDocked,
}

impl M5OccupantPersistence {
    /// Every occupant transition, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SideBySide,
        Self::Tabbed,
        Self::Sheeted,
        Self::Overflowed,
        Self::SoloDocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SideBySide => "side_by_side",
            Self::Tabbed => "tabbed",
            Self::Sheeted => "sheeted",
            Self::Overflowed => "overflowed",
            Self::SoloDocked => "solo_docked",
        }
    }
}

/// Responsive collapse placement in a family's fallback ladder.
///
/// The ladder is ordered and MUST terminate in [`Self::Placeholder`] so identity
/// and the reopen path are always preserved when the surface can no longer dock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FallbackPlacement {
    /// Surface remains docked in its declared zone.
    Docked,
    /// Surface opens as a sheet attached to its declared zone.
    Sheet,
    /// Surface is reachable through a keyboard-accessible overflow route.
    Overflow,
    /// Surface becomes an in-slot placeholder preserving identity and reopen path.
    Placeholder,
}

impl M5FallbackPlacement {
    /// Every fallback placement, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Docked,
        Self::Sheet,
        Self::Overflow,
        Self::Placeholder,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Docked => "docked",
            Self::Sheet => "sheet",
            Self::Overflow => "overflow",
            Self::Placeholder => "placeholder",
        }
    }
}

/// Owning-window routing expectation for dialogs, notifications, and approvals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OwningWindowRouting {
    /// Routed actions return to the owning window and object.
    RouteToOwningWindowObject,
    /// The exact object anchor is preserved on return.
    PreserveObjectAnchorOnReturn,
    /// Routing never steals focus from an unrelated window.
    NoFocusTheft,
    /// Detach or close never orphans a routed action.
    NoOrphanOnDetach,
}

impl M5OwningWindowRouting {
    /// Every routing expectation, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RouteToOwningWindowObject,
        Self::PreserveObjectAnchorOnReturn,
        Self::NoFocusTheft,
        Self::NoOrphanOnDetach,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RouteToOwningWindowObject => "route_to_owning_window_object",
            Self::PreserveObjectAnchorOnReturn => "preserve_object_anchor_on_return",
            Self::NoFocusTheft => "no_focus_theft",
            Self::NoOrphanOnDetach => "no_orphan_on_detach",
        }
    }
}

/// Workspace-global continuity truth every window preserves while layout stays
/// local.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContinuityTruth {
    /// Workspace-global trust class.
    WorkspaceGlobalTrust,
    /// Active remote target.
    RemoteTarget,
    /// Active deployment profile.
    DeploymentProfile,
    /// Recovery / restore state.
    RecoveryState,
}

impl M5ContinuityTruth {
    /// Every continuity truth, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::WorkspaceGlobalTrust,
        Self::RemoteTarget,
        Self::DeploymentProfile,
        Self::RecoveryState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceGlobalTrust => "workspace_global_trust",
            Self::RemoteTarget => "remote_target",
            Self::DeploymentProfile => "deployment_profile",
            Self::RecoveryState => "recovery_state",
        }
    }
}

/// Dependency-missing placeholder behavior for a surface family's canonical slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlaceholderBehavior {
    /// In-slot placeholder preserves the surface identity and reopen path.
    InSlotIdentityPreserved,
    /// Placeholder prompts to reconnect a remote or reauthorize a provider.
    ReconnectRemoteOrProvider,
    /// Placeholder prompts to install or enable a missing dependency.
    InstallOrEnableDependency,
    /// Placeholder recenters after a window/display topology drift.
    RecenteredOnTopologyDrift,
}

impl M5PlaceholderBehavior {
    /// Every placeholder behavior, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::InSlotIdentityPreserved,
        Self::ReconnectRemoteOrProvider,
        Self::InstallOrEnableDependency,
        Self::RecenteredOnTopologyDrift,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InSlotIdentityPreserved => "in_slot_identity_preserved",
            Self::ReconnectRemoteOrProvider => "reconnect_remote_or_provider",
            Self::InstallOrEnableDependency => "install_or_enable_dependency",
            Self::RecenteredOnTopologyDrift => "recentered_on_topology_drift",
        }
    }
}

/// Names one of the controlled state vocabularies a shell surface carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellStateVocabulary {
    /// Responsive class.
    ResponsiveClass,
    /// Window class.
    WindowClass,
    /// Occupant transition.
    OccupantPersistence,
    /// Responsive collapse placement.
    FallbackPlacement,
    /// Owning-window routing expectation.
    OwningWindowRouting,
    /// Workspace-global continuity truth.
    ContinuityTruth,
}

impl M5ShellStateVocabulary {
    /// Every vocabulary, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ResponsiveClass,
        Self::WindowClass,
        Self::OccupantPersistence,
        Self::FallbackPlacement,
        Self::OwningWindowRouting,
        Self::ContinuityTruth,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResponsiveClass => "responsive_class",
            Self::WindowClass => "window_class",
            Self::OccupantPersistence => "occupant_persistence",
            Self::FallbackPlacement => "fallback_placement",
            Self::OwningWindowRouting => "owning_window_routing",
            Self::ContinuityTruth => "continuity_truth",
        }
    }
}

/// Evidence requirement level for a surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellEvidenceRequirement {
    /// At least one proof packet is required.
    Required,
    /// Proof is recommended but not blocking.
    Recommended,
    /// Proof is optional.
    Optional,
    /// Not applicable for this family's current qualification.
    NotApplicable,
}

impl M5ShellEvidenceRequirement {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Recommended => "recommended",
            Self::Optional => "optional",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Downgrade trigger that can narrow a family below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellDowngradeTrigger {
    /// A surface attached outside a declared shell slot.
    SlotUndeclared,
    /// Responsive collapse changed the task identity.
    CollapseChangedTaskIdentity,
    /// Responsive collapse hid critical state instead of overflowing it.
    CriticalStateHiddenOnCollapse,
    /// A routed action lost its owning-window routing (focus theft or orphaning).
    OwningWindowRoutingLost,
    /// Workspace-global truth diverged across windows.
    WorkspaceTruthDivergedAcrossWindows,
    /// A placeholder lost the surface identity or its reopen path.
    PlaceholderLostIdentityOrReopen,
    /// A secondary display / window topology drift was not recentered.
    SecondaryDisplayTopologyDrift,
    /// A policy or legal block applies.
    PolicyBlocked,
    /// The proof packet has gone stale.
    ProofStale,
    /// An upstream dependency contract narrowed.
    UpstreamDependencyNarrowed,
}

impl M5ShellDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::SlotUndeclared,
        Self::CollapseChangedTaskIdentity,
        Self::CriticalStateHiddenOnCollapse,
        Self::OwningWindowRoutingLost,
        Self::WorkspaceTruthDivergedAcrossWindows,
        Self::PlaceholderLostIdentityOrReopen,
        Self::SecondaryDisplayTopologyDrift,
        Self::PolicyBlocked,
        Self::ProofStale,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SlotUndeclared => "slot_undeclared",
            Self::CollapseChangedTaskIdentity => "collapse_changed_task_identity",
            Self::CriticalStateHiddenOnCollapse => "critical_state_hidden_on_collapse",
            Self::OwningWindowRoutingLost => "owning_window_routing_lost",
            Self::WorkspaceTruthDivergedAcrossWindows => "workspace_truth_diverged_across_windows",
            Self::PlaceholderLostIdentityOrReopen => "placeholder_lost_identity_or_reopen",
            Self::SecondaryDisplayTopologyDrift => "secondary_display_topology_drift",
            Self::PolicyBlocked => "policy_blocked",
            Self::ProofStale => "proof_stale",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Rollback posture for a surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellRollbackPosture {
    /// The surface attaches only to its declared shell slot.
    AttachesOnlyToDeclaredSlot,
    /// Responsive collapse preserves the task identity.
    CollapsePreservesTaskIdentity,
    /// Critical state stays visible or moves to a keyboard-reachable overflow.
    CriticalStateStaysVisibleOrOverflowed,
    /// Routed actions return to the owning window and object.
    RoutesToOwningWindowObject,
    /// Every window preserves workspace-global trust, remote, profile, recovery.
    WindowPreservesWorkspaceGlobalTruth,
    /// The placeholder preserves the surface identity and its reopen path.
    PlaceholderPreservesIdentityAndReopen,
    /// Not applicable for the family's current qualification.
    NotApplicable,
}

impl M5ShellRollbackPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AttachesOnlyToDeclaredSlot => "attaches_only_to_declared_slot",
            Self::CollapsePreservesTaskIdentity => "collapse_preserves_task_identity",
            Self::CriticalStateStaysVisibleOrOverflowed => {
                "critical_state_stays_visible_or_overflowed"
            }
            Self::RoutesToOwningWindowObject => "routes_to_owning_window_object",
            Self::WindowPreservesWorkspaceGlobalTruth => "window_preserves_workspace_global_truth",
            Self::PlaceholderPreservesIdentityAndReopen => {
                "placeholder_preserves_identity_and_reopen"
            }
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Consumer surface that must project a shell family's slot metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellConsumerSurface {
    /// Shell frame / app chrome.
    ShellFrame,
    /// Windowing subsystem.
    Windowing,
    /// Layout subsystem.
    Layout,
    /// Status bar.
    StatusBar,
    /// Attention router.
    AttentionRouter,
    /// Notification envelope.
    NotificationEnvelope,
    /// Docs / help surface.
    DocsHelp,
    /// Release-proof packet.
    ReleaseProof,
    /// Support / export packet.
    SupportExport,
    /// Product UI surface.
    ProductUi,
}

impl M5ShellConsumerSurface {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellFrame => "shell_frame",
            Self::Windowing => "windowing",
            Self::Layout => "layout",
            Self::StatusBar => "status_bar",
            Self::AttentionRouter => "attention_router",
            Self::NotificationEnvelope => "notification_envelope",
            Self::DocsHelp => "docs_help",
            Self::ReleaseProof => "release_proof",
            Self::SupportExport => "support_export",
            Self::ProductUi => "product_ui",
        }
    }
}

/// One row in the M5 shell-zone / responsive-class / multi-window matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellSurfaceRow {
    /// Governed surface family.
    pub family: M5ShellSurfaceFamily,
    /// Qualification class earned by this family.
    pub qualification: M5ShellQualificationClass,
    /// Owner role accountable for keeping this family governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Required fields the surface descriptor must carry.
    pub required_fields: Vec<String>,
    /// Canonical shell slot this family attaches to.
    pub canonical_slot: M5ShellZoneSlot,
    /// Fallback shell slot when the canonical slot cannot be shown.
    pub fallback_slot: M5ShellZoneSlot,
    /// Dependency-missing placeholder behavior for the canonical slot.
    pub placeholder_behavior: M5PlaceholderBehavior,
    /// Controlled state vocabularies this family carries.
    pub state_vocabularies: Vec<M5ShellStateVocabulary>,
    /// Responsive classes this family must survive.
    pub responsive_classes: Vec<M5ResponsiveClass>,
    /// Window classes this family may live in.
    pub window_classes: Vec<M5WindowClass>,
    /// Allowed occupant transitions.
    pub occupant_persistence: Vec<M5OccupantPersistence>,
    /// Ordered responsive collapse ladder; terminates in `placeholder`.
    pub fallback_placements: Vec<M5FallbackPlacement>,
    /// Owning-window routing expectations.
    pub owning_window_routing: Vec<M5OwningWindowRouting>,
    /// Workspace-global continuity truths preserved by every window.
    pub continuity_truths: Vec<M5ContinuityTruth>,
    /// Evidence requirement level.
    pub evidence_requirement: M5ShellEvidenceRequirement,
    /// Proof packet refs that keep this family current.
    pub required_proof_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this family.
    pub downgrade_triggers: Vec<M5ShellDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5ShellRollbackPosture,
    /// Source contract refs consumed by this family.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this family's slot metadata.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
}

impl M5ShellSurfaceRow {
    /// Returns true when the row declares the given vocabulary.
    fn declares(&self, vocab: M5ShellStateVocabulary) -> bool {
        self.state_vocabularies.contains(&vocab)
    }

    /// Returns true when the token vec for `vocab` is non-empty.
    fn vocab_tokens_present(&self, vocab: M5ShellStateVocabulary) -> bool {
        use M5ShellStateVocabulary as V;
        match vocab {
            V::ResponsiveClass => !self.responsive_classes.is_empty(),
            V::WindowClass => !self.window_classes.is_empty(),
            V::OccupantPersistence => !self.occupant_persistence.is_empty(),
            V::FallbackPlacement => !self.fallback_placements.is_empty(),
            V::OwningWindowRouting => !self.owning_window_routing.is_empty(),
            V::ContinuityTruth => !self.continuity_truths.is_empty(),
        }
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
///
/// Each field lists every canonical token for one controlled vocabulary, in
/// declaration order. The matrix validates each list against the typed `ALL`
/// arrays so the frozen vocabulary cannot silently drift.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellVocabularySet {
    /// Shell-zone slot tokens.
    pub shell_zone_slots: Vec<String>,
    /// Responsive-class tokens.
    pub responsive_classes: Vec<String>,
    /// Window-class tokens.
    pub window_classes: Vec<String>,
    /// Occupant-transition tokens.
    pub occupant_persistence: Vec<String>,
    /// Fallback-placement tokens.
    pub fallback_placements: Vec<String>,
    /// Owning-window-routing tokens.
    pub owning_window_routing: Vec<String>,
    /// Continuity-truth tokens.
    pub continuity_truths: Vec<String>,
    /// Placeholder-behavior tokens.
    pub placeholder_behaviors: Vec<String>,
}

impl M5ShellVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            shell_zone_slots: tokens(&M5ShellZoneSlot::ALL, |v| v.as_str()),
            responsive_classes: tokens(&M5ResponsiveClass::ALL, |v| v.as_str()),
            window_classes: tokens(&M5WindowClass::ALL, |v| v.as_str()),
            occupant_persistence: tokens(&M5OccupantPersistence::ALL, |v| v.as_str()),
            fallback_placements: tokens(&M5FallbackPlacement::ALL, |v| v.as_str()),
            owning_window_routing: tokens(&M5OwningWindowRouting::ALL, |v| v.as_str()),
            continuity_truths: tokens(&M5ContinuityTruth::ALL, |v| v.as_str()),
            placeholder_behaviors: tokens(&M5PlaceholderBehavior::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Shell-continuity review block.
///
/// Every flag is a hard invariant; all must hold for the matrix to validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellContinuityReview {
    /// New surfaces attach only to declared shell slots.
    pub new_surfaces_attach_only_to_declared_slots: bool,
    /// Responsive collapse never changes task identity.
    pub responsive_collapse_never_changes_task_identity: bool,
    /// Responsive collapse never hides critical state.
    pub responsive_collapse_never_hides_critical_state: bool,
    /// Every window preserves workspace-global trust, remote, profile, recovery.
    pub every_window_preserves_workspace_global_trust_remote_profile_recovery: bool,
    /// Layout stays local while workspace-global truth stays global.
    pub layout_stays_local_while_truth_stays_global: bool,
    /// Dialogs, notifications, approvals route to the owning window and object.
    pub dialogs_notifications_approvals_route_to_owning_window_object: bool,
    /// Routed actions never steal focus or orphan.
    pub no_focus_theft_or_orphaning: bool,
    /// Secondary display and zoom preserve surface identity.
    pub secondary_display_and_zoom_preserve_identity: bool,
    /// One shell-zone matrix is consumed rather than local layout prose.
    pub one_shell_zone_matrix_not_local_layout_prose: bool,
    /// No surface invents its own slot or collapse behavior.
    pub no_surface_invents_its_own_slot_or_collapse: bool,
    /// Downgrade narrows the claim rather than hiding the surface.
    pub downgrade_narrows_instead_of_hides: bool,
    /// An unmapped surface blocks a shell-maturity claim.
    pub unmapped_surface_blocks_shell_maturity_claim: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellConsumerProjection {
    /// The shell frame consumes the slot matrix.
    pub shell_frame_consumes_slot_matrix: bool,
    /// Windowing consumes the window classes.
    pub windowing_consumes_window_classes: bool,
    /// Layout consumes the responsive classes.
    pub layout_consumes_responsive_classes: bool,
    /// Status bar consumes its declared status-bar slot.
    pub status_bar_consumes_status_slot: bool,
    /// The attention router routes to the owning window and object.
    pub attention_router_routes_to_owning_window: bool,
    /// The notification envelope uses owning-window routing.
    pub notification_envelope_uses_owning_window_routing: bool,
    /// Docs / help consume the shared slot metadata.
    pub docs_help_consume_slot_metadata: bool,
    /// Release proof consumes the shared slot metadata.
    pub release_proof_consumes_slot_metadata: bool,
    /// Support export shows the shell-zone matrix.
    pub support_export_shows_shell_zone_matrix: bool,
    /// Preview / Labs surfaces are visibly labeled when not mapped by this packet.
    pub preview_labs_label_for_unmapped_surfaces: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the family.
    pub auto_narrow_on_stale: bool,
}

/// Release and multi-window parity posture for the shell-continuity lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellReleasePosture {
    /// Ref of the supporting release packet for the lane.
    pub release_packet_ref: String,
    /// Ref of the supporting multi-window continuity proof packet for the lane.
    pub multi_window_proof_packet_ref: String,
    /// True when support/export parity is required for every family.
    pub support_export_parity_required: bool,
    /// True when multi-window parity is required for every family.
    pub multi_window_parity_required: bool,
}

/// Constructor input for [`M5ShellZoneMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ShellZoneMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5ShellSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ShellVocabularySet,
    /// Shell-continuity review block.
    pub continuity_review: M5ShellContinuityReview,
    /// Consumer projection block.
    pub consumer_projection: M5ShellConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ShellProofFreshness,
    /// Release and multi-window parity posture.
    pub release_posture: M5ShellReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 shell-zone / responsive-class / multi-window matrix
/// packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellZoneMatrixPacket {
    /// Record kind; must equal [`M5_SHELL_ZONE_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SHELL_ZONE_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5ShellSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ShellVocabularySet,
    /// Shell-continuity review block.
    pub continuity_review: M5ShellContinuityReview,
    /// Consumer projection block.
    pub consumer_projection: M5ShellConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ShellProofFreshness,
    /// Release and multi-window parity posture.
    pub release_posture: M5ShellReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ShellZoneMatrixPacket {
    /// Builds an M5 shell-zone matrix packet from stable-lane input.
    pub fn new(input: M5ShellZoneMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_SHELL_ZONE_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_SHELL_ZONE_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            surface_rows: input.surface_rows,
            vocabulary_set: input.vocabulary_set,
            continuity_review: input.continuity_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 shell-zone matrix invariants.
    pub fn validate(&self) -> Vec<M5ShellZoneMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SHELL_ZONE_MATRIX_RECORD_KIND {
            violations.push(M5ShellZoneMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SHELL_ZONE_MATRIX_SCHEMA_VERSION {
            violations.push(M5ShellZoneMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ShellZoneMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_continuity_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 shell-zone matrix packet serializes"),
        ) {
            violations.push(M5ShellZoneMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 shell-zone matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family,
    /// naming its qualification, owner, canonical/fallback slot, placeholder
    /// behavior, evidence, downgrade triggers, rollback posture, and consumers.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "family,qualification,owner,canonical_slot,fallback_slot,placeholder_behavior,window_classes,occupant_persistence,fallback_placements,evidence_requirement,downgrade_triggers,rollback_posture,consumer_surfaces\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.canonical_slot.as_str(),
                row.fallback_slot.as_str(),
                row.placeholder_behavior.as_str(),
                join_tokens(&row.window_classes, |v| v.as_str()),
                join_tokens(&row.occupant_persistence, |v| v.as_str()),
                join_tokens(&row.fallback_placements, |v| v.as_str()),
                row.evidence_requirement.as_str(),
                join_tokens(&row.downgrade_triggers, |t| t.as_str()),
                row.rollback_posture.as_str(),
                join_tokens(&row.consumer_surfaces, |s| s.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown governance summary for support, docs, or review
    /// handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_families = self
            .surface_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Shell-Zone, Responsive-Class, and Multi-Window Continuity Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Families: {} ({} stable)\n",
            self.surface_rows.len(),
            stable_families
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Surface families\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Canonical slot: {} (`{}`)\n",
                row.canonical_slot.label(),
                row.canonical_slot.as_str()
            ));
            out.push_str(&format!(
                "  - Fallback slot: {} (`{}`)\n",
                row.fallback_slot.label(),
                row.fallback_slot.as_str()
            ));
            out.push_str(&format!(
                "  - Dependency-missing placeholder: `{}`\n",
                row.placeholder_behavior.as_str()
            ));
            out.push_str(&format!(
                "  - Windows: {}\n",
                row.window_classes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Collapse ladder: {}\n",
                row.fallback_placements
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(" → ")
            ));
            out.push_str(&format!("  - Rollback: {}\n", row.rollback_posture.as_str()));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 shell-zone matrix export.
#[derive(Debug)]
pub enum M5ShellZoneMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ShellZoneMatrixViolation>),
}

impl fmt::Display for M5ShellZoneMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "m5 shell-zone matrix export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 shell-zone matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ShellZoneMatrixArtifactError {}

/// Validation failures emitted by [`M5ShellZoneMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ShellZoneMatrixViolation {
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
    /// A required governed family is missing from the matrix.
    RequiredFamilyMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row omits a vocabulary its family requires.
    RequiredVocabularyMissing,
    /// A declared vocabulary has no concrete tokens.
    DeclaredVocabularyHasNoTokens,
    /// A token vec is populated for a vocabulary the row does not declare.
    UndeclaredVocabularyHasTokens,
    /// A responsive collapse ladder does not terminate in `placeholder`.
    FallbackLadderNotTerminatedByPlaceholder,
    /// A family does not admit the primary workspace window.
    PrimaryWindowMissing,
    /// A family does not survive every responsive class.
    ResponsiveClassCoverageIncomplete,
    /// A family does not declare every owning-window routing expectation.
    OwningWindowRoutingIncomplete,
    /// A family does not preserve every workspace-global continuity truth.
    ContinuityTruthIncomplete,
    /// A family claiming Stable is missing required proof packet refs.
    StableFamilyMissingProof,
    /// A family has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A family has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Continuity review does not satisfy required invariants.
    ContinuityReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/multi-window parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5ShellZoneMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredFamilyMissing => "required_family_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::RequiredVocabularyMissing => "required_vocabulary_missing",
            Self::DeclaredVocabularyHasNoTokens => "declared_vocabulary_has_no_tokens",
            Self::UndeclaredVocabularyHasTokens => "undeclared_vocabulary_has_tokens",
            Self::FallbackLadderNotTerminatedByPlaceholder => {
                "fallback_ladder_not_terminated_by_placeholder"
            }
            Self::PrimaryWindowMissing => "primary_window_missing",
            Self::ResponsiveClassCoverageIncomplete => "responsive_class_coverage_incomplete",
            Self::OwningWindowRoutingIncomplete => "owning_window_routing_incomplete",
            Self::ContinuityTruthIncomplete => "continuity_truth_incomplete",
            Self::StableFamilyMissingProof => "stable_family_missing_proof",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ContinuityReviewIncomplete => "continuity_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 shell-zone matrix export.
pub fn current_stable_m5_shell_zone_matrix_export(
) -> Result<M5ShellZoneMatrixPacket, M5ShellZoneMatrixArtifactError> {
    let packet: M5ShellZoneMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-shell-continuity-proof/support_export.json"
    )))
    .map_err(M5ShellZoneMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ShellZoneMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ShellZoneMatrixPacket,
    violations: &mut Vec<M5ShellZoneMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SHELL_ZONE_MATRIX_SCHEMA_REF,
        M5_SHELL_RESPONSIVE_CLASS_SCHEMA_REF,
        M5_SHELL_ZONE_MATRIX_DOC_REF,
        M5_SHELL_ZONING_CONTRACT_REF,
        M5_SHELL_WINDOW_TOPOLOGY_CONTRACT_REF,
        M5_SHELL_ATTENTION_ROUTING_CONTRACT_REF,
        M5_SHELL_NOTIFICATION_ENVELOPE_CONTRACT_REF,
        M5_SHELL_SESSION_RESTORE_CONTRACT_REF,
        M5_SHELL_REFERENCE_LAYOUT_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ShellZoneMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ShellZoneMatrixPacket,
    violations: &mut Vec<M5ShellZoneMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ShellZoneMatrixViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5ShellZoneMatrixPacket,
    violations: &mut Vec<M5ShellZoneMatrixViolation>,
) {
    let present: BTreeSet<M5ShellSurfaceFamily> =
        packet.surface_rows.iter().map(|row| row.family).collect();
    for required in M5ShellSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5ShellZoneMatrixViolation::RequiredFamilyMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.required_fields.is_empty()
            || row.state_vocabularies.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5ShellZoneMatrixViolation::SurfaceRowIncomplete);
        }

        for required_vocab in row.family.required_state_vocabularies() {
            if !row.declares(*required_vocab) {
                violations.push(M5ShellZoneMatrixViolation::RequiredVocabularyMissing);
            }
        }

        for vocab in M5ShellStateVocabulary::ALL {
            let declared = row.declares(vocab);
            let has_tokens = row.vocab_tokens_present(vocab);
            if declared && !has_tokens {
                violations.push(M5ShellZoneMatrixViolation::DeclaredVocabularyHasNoTokens);
            }
            if !declared && has_tokens {
                violations.push(M5ShellZoneMatrixViolation::UndeclaredVocabularyHasTokens);
            }
        }

        if row.fallback_placements.last() != Some(&M5FallbackPlacement::Placeholder) {
            violations
                .push(M5ShellZoneMatrixViolation::FallbackLadderNotTerminatedByPlaceholder);
        }
        if !row
            .window_classes
            .contains(&M5WindowClass::PrimaryWorkspaceWindow)
        {
            violations.push(M5ShellZoneMatrixViolation::PrimaryWindowMissing);
        }
        for class in M5ResponsiveClass::ALL {
            if !row.responsive_classes.contains(&class) {
                violations.push(M5ShellZoneMatrixViolation::ResponsiveClassCoverageIncomplete);
                break;
            }
        }
        for expectation in M5OwningWindowRouting::ALL {
            if !row.owning_window_routing.contains(&expectation) {
                violations.push(M5ShellZoneMatrixViolation::OwningWindowRoutingIncomplete);
                break;
            }
        }
        for truth in M5ContinuityTruth::ALL {
            if !row.continuity_truths.contains(&truth) {
                violations.push(M5ShellZoneMatrixViolation::ContinuityTruthIncomplete);
                break;
            }
        }

        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ShellZoneMatrixViolation::StableFamilyMissingProof);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ShellZoneMatrixViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ShellZoneMatrixViolation::ConsumerSurfacesMissing);
        }
    }
}

fn validate_continuity_review(
    packet: &M5ShellZoneMatrixPacket,
    violations: &mut Vec<M5ShellZoneMatrixViolation>,
) {
    let review = &packet.continuity_review;
    for ok in [
        review.new_surfaces_attach_only_to_declared_slots,
        review.responsive_collapse_never_changes_task_identity,
        review.responsive_collapse_never_hides_critical_state,
        review.every_window_preserves_workspace_global_trust_remote_profile_recovery,
        review.layout_stays_local_while_truth_stays_global,
        review.dialogs_notifications_approvals_route_to_owning_window_object,
        review.no_focus_theft_or_orphaning,
        review.secondary_display_and_zoom_preserve_identity,
        review.one_shell_zone_matrix_not_local_layout_prose,
        review.no_surface_invents_its_own_slot_or_collapse,
        review.downgrade_narrows_instead_of_hides,
        review.unmapped_surface_blocks_shell_maturity_claim,
    ] {
        if !ok {
            violations.push(M5ShellZoneMatrixViolation::ContinuityReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ShellZoneMatrixPacket,
    violations: &mut Vec<M5ShellZoneMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_frame_consumes_slot_matrix,
        projection.windowing_consumes_window_classes,
        projection.layout_consumes_responsive_classes,
        projection.status_bar_consumes_status_slot,
        projection.attention_router_routes_to_owning_window,
        projection.notification_envelope_uses_owning_window_routing,
        projection.docs_help_consume_slot_metadata,
        projection.release_proof_consumes_slot_metadata,
        projection.support_export_shows_shell_zone_matrix,
        projection.preview_labs_label_for_unmapped_surfaces,
    ] {
        if !ok {
            violations.push(M5ShellZoneMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ShellZoneMatrixPacket,
    violations: &mut Vec<M5ShellZoneMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ShellZoneMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ShellZoneMatrixPacket,
    violations: &mut Vec<M5ShellZoneMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.multi_window_proof_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.multi_window_parity_required
    {
        violations.push(M5ShellZoneMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Collects the `as_str` tokens of an `ALL` array into an owned token vec.
fn tokens<T, F>(all: &[T], to_token: F) -> Vec<String>
where
    F: Fn(&T) -> &'static str,
{
    all.iter().map(|item| to_token(item).to_owned()).collect()
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items
        .iter()
        .map(|item| to_token(item))
        .collect::<Vec<_>>()
        .join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
