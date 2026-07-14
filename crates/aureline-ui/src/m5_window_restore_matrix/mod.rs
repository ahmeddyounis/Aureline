//! Frozen M5 workspace-window, shared-authority, skeleton-restore, and no-rerun session-hydration matrix.
//!
//! This module locks Aureline's concrete multi-window ownership and restore-orchestration behavior into one
//! export-safe packet. Every claimed M5 workspace-restore profile — shared workspace authority backing
//! multiple windows, window-local pane topology, skeleton-first / hydrate-second restore, no-rerun
//! session hydration, and display-topology recovery — is named once here and constrained by the same shared
//! window-restore-role taxonomy (workspace_authority, window_topology, pane_role, layout_skeleton,
//! session_hydration, restore_fidelity, display_affinity), the same
//! window-local-selection-and-focus-stay-window-local rule, the same
//! pane-trees-stay-versioned-and-attributable rule, the same
//! rebuild-layout-skeleton-before-hydrating-heavy-dependencies rule, the same
//! session-scoped-tools-never-silently-rerun rule, and the same
//! display-topology-changes-keep-windows-and-dialogs-reachable rule regardless of the surface that renders
//! it.
//!
//! The matrix does not redesign project-entry or remembered-state inspectors — it is the shared reusable
//! restore-engine contract those already-governed surfaces consume, and it binds back to the already-landed
//! multi-window-parity and monitor-geometry-remap packets instead of leaving restore truth split across
//! scattered recovery prose and hand-copied window notes. The controlled vocabularies are frozen in one
//! self-describing [`M5WindowRestoreVocabularySet`] rather than minted per surface. The single controlled
//! window-restore-role vocabulary consumers bind to — workspace_authority, window_topology, pane_role,
//! layout_skeleton, session_hydration, restore_fidelity, and display_affinity — keeps workspace authority
//! and window topology separately inspectable; keeps selections and focus window-local while one authority
//! backs many windows; keeps pane trees versioned and attributable; keeps restore rebuilding layout
//! skeletons before hydrating heavy dependencies; keeps terminals, debug sessions, notebooks, previews,
//! remote shells, and collaboration surfaces from silently rerunning or reacquiring broader authority; and
//! keeps display-topology changes leaving every window and dialog reachable. Raw secret values and private
//! endpoints stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_window_restore_matrix,
    seeded_m5_window_restore_matrix_display_topology_recovery_preview_narrowed,
    seeded_m5_window_restore_matrix_no_rerun_session_hydration_beta_narrowed,
    M5_WINDOW_RESTORE_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5WindowRestoreMatrixPacket`].
pub const M5_WINDOW_RESTORE_MATRIX_RECORD_KIND: &str =
    "freeze_m5_workspace_window_shared_authority_skeleton_restore_and_no_rerun_session_hydration_matrix";

/// Schema version for M5 window-restore matrix records.
pub const M5_WINDOW_RESTORE_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined window-restore matrix schema.
pub const M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF: &str =
    "schemas/shell/m5-window-restore-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_WINDOW_RESTORE_MATRIX_DOC_REF: &str = "docs/recovery/m5_window_restore_contract.md";

/// Repo-relative path of the canonical window-topology domain schema (shared workspace authority,
/// window-local pane topology, and display-topology recovery for the shared-authority, window-local, and
/// display-recovery families).
pub const M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF: &str =
    "schemas/shell/m5-window-topology.schema.json";

/// Repo-relative path of the canonical restore-fidelity domain schema (skeleton-first rebuild, no-rerun
/// hydration, and exact / compatible / layout-only fidelity truth for the skeleton-restore and
/// no-rerun-hydration families).
pub const M5_RESTORE_FIDELITY_SCHEMA_REF: &str = "schemas/shell/m5-restore-fidelity.schema.json";

/// Repo-relative path of the already-landed multi-window-parity schema the matrix binds back to.
pub const M5_MULTI_WINDOW_PARITY_SCHEMA_REF: &str =
    "schemas/shell/m5-multi-window-parity.schema.json";

/// Repo-relative path of the already-landed monitor-geometry-remap-and-restore-bounds schema the
/// window-restore matrix binds back to.
pub const M5_MONITOR_GEOMETRY_REMAP_SCHEMA_REF: &str =
    "schemas/shell/m5-monitor-geometry-remap-and-restore-bounds.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_WINDOW_RESTORE_FIXTURE_DIR: &str = "fixtures/ui/m5-window-restore";

/// Repo-relative path of the checked support-export artifact.
pub const M5_WINDOW_RESTORE_ARTIFACT_REF: &str =
    "artifacts/release/m5-window-restore-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_WINDOW_RESTORE_CSV_REF: &str = "artifacts/release/m5-window-restore-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_WINDOW_RESTORE_REPORT_REF: &str =
    "artifacts/shell/m5-workspace-window-restore-matrix.md";

/// One of the five governed workspace-restore families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowRestoreFamily {
    /// Shared workspace authority: one authority backs multiple windows while selections and focus stay
    /// window-local.
    SharedWorkspaceAuthority,
    /// Window-local topology: versioned, attributable pane trees scoped to their owning window.
    WindowLocalTopology,
    /// Skeleton-first restore: rebuild the layout skeleton before hydrating heavy dependencies.
    SkeletonFirstRestore,
    /// No-rerun session hydration: session-scoped tools never silently rerun or reacquire broader authority.
    NoRerunSessionHydration,
    /// Display-topology recovery: topology changes keep windows and dialogs reachable and preserve intent.
    DisplayTopologyRecovery,
}

impl M5WindowRestoreFamily {
    /// Every governed workspace-restore family, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SharedWorkspaceAuthority,
        Self::WindowLocalTopology,
        Self::SkeletonFirstRestore,
        Self::NoRerunSessionHydration,
        Self::DisplayTopologyRecovery,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SharedWorkspaceAuthority => "shared_workspace_authority",
            Self::WindowLocalTopology => "window_local_topology",
            Self::SkeletonFirstRestore => "skeleton_first_restore",
            Self::NoRerunSessionHydration => "no_rerun_session_hydration",
            Self::DisplayTopologyRecovery => "display_topology_recovery",
        }
    }

    /// The canonical per-domain schema ref a downstream surface points at instead of restating this
    /// family's window-topology or restore-fidelity meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::SharedWorkspaceAuthority
            | Self::WindowLocalTopology
            | Self::DisplayTopologyRecovery => M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF,
            Self::SkeletonFirstRestore | Self::NoRerunSessionHydration => {
                M5_RESTORE_FIDELITY_SCHEMA_REF
            }
        }
    }

    /// `true` when this family must name a controlled shared-workspace-authority role.
    pub const fn declares_shared_workspace_authority_roles(self) -> bool {
        matches!(self, Self::SharedWorkspaceAuthority)
    }

    /// `true` when this family must name a controlled window-local-topology role.
    pub const fn declares_window_local_topology_roles(self) -> bool {
        matches!(self, Self::WindowLocalTopology)
    }

    /// `true` when this family must name a controlled skeleton-first-restore role.
    pub const fn declares_skeleton_first_restore_roles(self) -> bool {
        matches!(self, Self::SkeletonFirstRestore)
    }

    /// `true` when this family must name a controlled no-rerun-session-hydration role.
    pub const fn declares_no_rerun_session_hydration_roles(self) -> bool {
        matches!(self, Self::NoRerunSessionHydration)
    }

    /// `true` when this family must name a controlled display-topology-recovery role.
    pub const fn declares_display_topology_recovery_roles(self) -> bool {
        matches!(self, Self::DisplayTopologyRecovery)
    }
}

/// The single controlled window-restore-role vocabulary every shell, recovery, diagnostics, admin, docs, or
/// support consumer binds to. These are the exact acceptance-criteria tokens that keep `workspace_authority`,
/// `window_topology`, `pane_role`, `layout_skeleton`, `session_hydration`, `restore_fidelity`, and
/// `display_affinity` meaning the same thing everywhere the window-restore grammar ships. No surface invents
/// a parallel word for any of these roles, and the authority / hydration / fidelity / affinity roles may
/// never let a restore clobber a window-local selection, silently rerun session-scoped work, overclaim
/// restore fidelity, or strand a window off-screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowRestoreRole {
    /// Workspace-authority role (the shared authority that may back multiple windows).
    WorkspaceAuthority,
    /// Window-topology role (the window-local pane tree and split layout).
    WindowTopology,
    /// Pane-role role (the versioned, attributable role a pane placeholder carries).
    PaneRole,
    /// Layout-skeleton role (the layout rebuilt first, before hydration).
    LayoutSkeleton,
    /// Session-hydration role (heavy dependency hydration that must never silently rerun).
    SessionHydration,
    /// Restore-fidelity role (exact versus compatible versus layout-only restore truth).
    RestoreFidelity,
    /// Display-affinity role (the monitor-affinity hint that keeps windows visible after remap).
    DisplayAffinity,
}

impl M5WindowRestoreRole {
    /// Every window-restore role token, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::WorkspaceAuthority,
        Self::WindowTopology,
        Self::PaneRole,
        Self::LayoutSkeleton,
        Self::SessionHydration,
        Self::RestoreFidelity,
        Self::DisplayAffinity,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceAuthority => "workspace_authority",
            Self::WindowTopology => "window_topology",
            Self::PaneRole => "pane_role",
            Self::LayoutSkeleton => "layout_skeleton",
            Self::SessionHydration => "session_hydration",
            Self::RestoreFidelity => "restore_fidelity",
            Self::DisplayAffinity => "display_affinity",
        }
    }

    /// Whether this role carries authority, hydration, fidelity, or affinity truth whose per-family behavior
    /// must never clobber a window-local selection or focus under shared authority, silently rerun or
    /// reattach session-scoped work, overclaim restore fidelity, or strand a window off-screen after a
    /// display-topology change (`workspace_authority`, `session_hydration`, `restore_fidelity`,
    /// `display_affinity`). The descriptive structure roles (`window_topology`, `pane_role`,
    /// `layout_skeleton`) are inspectable descriptors rather than authority-carrying truth and so do not
    /// carry this requirement.
    pub const fn must_preserve_window_local_selection_and_no_rerun_under_shared_authority(
        self,
    ) -> bool {
        matches!(
            self,
            Self::WorkspaceAuthority
                | Self::SessionHydration
                | Self::RestoreFidelity
                | Self::DisplayAffinity
        )
    }
}

/// Controlled shared-workspace-authority role — how a shared workspace authority is named, so a single
/// authority backing multiple windows, window-local selection and focus, versioned attributable pane trees,
/// and explicit authority-to-window binding follow one restore registry rather than merging authority and
/// topology into one opaque blob.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SharedWorkspaceAuthorityRole {
    /// A single authority backs multiple windows.
    SingleAuthorityBacksMultipleWindows,
    /// Selection and focus stay window-local.
    WindowLocalSelectionAndFocus,
    /// Pane trees stay versioned and attributable.
    VersionedAttributablePaneTrees,
    /// Explicit authority-to-window binding.
    ExplicitAuthorityWindowBinding,
    /// A role bound to the single restore registry.
    BoundToWindowRestoreRegistry,
    /// A merged authority-and-topology opaque blob, which is disallowed.
    MergedAuthorityAndTopologyBlobDisallowed,
}

impl M5SharedWorkspaceAuthorityRole {
    /// Every shared-workspace-authority role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SingleAuthorityBacksMultipleWindows,
        Self::WindowLocalSelectionAndFocus,
        Self::VersionedAttributablePaneTrees,
        Self::ExplicitAuthorityWindowBinding,
        Self::BoundToWindowRestoreRegistry,
        Self::MergedAuthorityAndTopologyBlobDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleAuthorityBacksMultipleWindows => "single_authority_backs_multiple_windows",
            Self::WindowLocalSelectionAndFocus => "window_local_selection_and_focus",
            Self::VersionedAttributablePaneTrees => "versioned_attributable_pane_trees",
            Self::ExplicitAuthorityWindowBinding => "explicit_authority_window_binding",
            Self::BoundToWindowRestoreRegistry => "bound_to_window_restore_registry",
            Self::MergedAuthorityAndTopologyBlobDisallowed => {
                "merged_authority_and_topology_blob_disallowed"
            }
        }
    }
}

/// Controlled window-local-topology role — how a window-local pane topology is named, so the window-scoped
/// pane tree, versioned pane topology, attributable pane roles, and pane-role placeholder follow one restore
/// registry rather than leaving an opaque, unattributable topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowLocalTopologyRole {
    /// Window-scoped pane tree.
    WindowScopedPaneTree,
    /// Versioned pane topology.
    VersionedPaneTopology,
    /// Attributable pane roles.
    AttributablePaneRoles,
    /// Pane-role placeholder.
    PaneRolePlaceholder,
    /// A role bound to the single restore registry.
    BoundToWindowRestoreRegistry,
    /// An opaque, unattributable topology, which is disallowed.
    OpaqueUnattributableTopologyDisallowed,
}

impl M5WindowLocalTopologyRole {
    /// Every window-local-topology role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WindowScopedPaneTree,
        Self::VersionedPaneTopology,
        Self::AttributablePaneRoles,
        Self::PaneRolePlaceholder,
        Self::BoundToWindowRestoreRegistry,
        Self::OpaqueUnattributableTopologyDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WindowScopedPaneTree => "window_scoped_pane_tree",
            Self::VersionedPaneTopology => "versioned_pane_topology",
            Self::AttributablePaneRoles => "attributable_pane_roles",
            Self::PaneRolePlaceholder => "pane_role_placeholder",
            Self::BoundToWindowRestoreRegistry => "bound_to_window_restore_registry",
            Self::OpaqueUnattributableTopologyDisallowed => {
                "opaque_unattributable_topology_disallowed"
            }
        }
    }
}

/// Controlled skeleton-first-restore role — how skeleton-first restore is named, so the layout skeleton
/// rebuilt first, heavy dependency hydrated second, pane-role placeholder shown while hydrating, and
/// disclosed restore-fidelity class follow one restore registry rather than silently deleting layout
/// structure on a missing dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SkeletonFirstRestoreRole {
    /// Layout skeleton rebuilt first.
    LayoutSkeletonRebuiltFirst,
    /// Heavy dependency hydrated second.
    HeavyDependencyHydratedSecond,
    /// Pane-role placeholder shown while hydrating.
    PaneRolePlaceholderShownWhileHydrating,
    /// Disclosed restore-fidelity class.
    DisclosedRestoreFidelityClass,
    /// A role bound to the single restore registry.
    BoundToWindowRestoreRegistry,
    /// A silent structure deletion on a missing dependency, which is disallowed.
    SilentStructureDeletionOnMissingDependencyDisallowed,
}

impl M5SkeletonFirstRestoreRole {
    /// Every skeleton-first-restore role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LayoutSkeletonRebuiltFirst,
        Self::HeavyDependencyHydratedSecond,
        Self::PaneRolePlaceholderShownWhileHydrating,
        Self::DisclosedRestoreFidelityClass,
        Self::BoundToWindowRestoreRegistry,
        Self::SilentStructureDeletionOnMissingDependencyDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LayoutSkeletonRebuiltFirst => "layout_skeleton_rebuilt_first",
            Self::HeavyDependencyHydratedSecond => "heavy_dependency_hydrated_second",
            Self::PaneRolePlaceholderShownWhileHydrating => {
                "pane_role_placeholder_shown_while_hydrating"
            }
            Self::DisclosedRestoreFidelityClass => "disclosed_restore_fidelity_class",
            Self::BoundToWindowRestoreRegistry => "bound_to_window_restore_registry",
            Self::SilentStructureDeletionOnMissingDependencyDisallowed => {
                "silent_structure_deletion_on_missing_dependency_disallowed"
            }
        }
    }
}

/// Controlled no-rerun-session-hydration role — how no-rerun session hydration is named, so a session-scoped
/// tool that never silently reruns, a privileged session that is never implicitly reattached, an explicit
/// user action to reacquire authority, and disclosed reopened-versus-rerun context follow one restore
/// registry rather than hiding a rerun or reattach.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NoRerunSessionHydrationRole {
    /// A session-scoped tool never silently reruns.
    SessionScopedToolNeverSilentlyReruns,
    /// A privileged session is never implicitly reattached.
    PrivilegedSessionNeverImplicitlyReattached,
    /// An explicit user action is required to reacquire broader authority.
    ExplicitUserActionToReacquireAuthority,
    /// Disclosed reopened-versus-rerun context.
    DisclosedReopenedVersusRerunContext,
    /// A role bound to the single restore registry.
    BoundToWindowRestoreRegistry,
    /// A hidden rerun or reattach, which is disallowed.
    HiddenRerunOrReattachDisallowed,
}

impl M5NoRerunSessionHydrationRole {
    /// Every no-rerun-session-hydration role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SessionScopedToolNeverSilentlyReruns,
        Self::PrivilegedSessionNeverImplicitlyReattached,
        Self::ExplicitUserActionToReacquireAuthority,
        Self::DisclosedReopenedVersusRerunContext,
        Self::BoundToWindowRestoreRegistry,
        Self::HiddenRerunOrReattachDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionScopedToolNeverSilentlyReruns => {
                "session_scoped_tool_never_silently_reruns"
            }
            Self::PrivilegedSessionNeverImplicitlyReattached => {
                "privileged_session_never_implicitly_reattached"
            }
            Self::ExplicitUserActionToReacquireAuthority => {
                "explicit_user_action_to_reacquire_authority"
            }
            Self::DisclosedReopenedVersusRerunContext => "disclosed_reopened_versus_rerun_context",
            Self::BoundToWindowRestoreRegistry => "bound_to_window_restore_registry",
            Self::HiddenRerunOrReattachDisallowed => "hidden_rerun_or_reattach_disallowed",
        }
    }
}

/// Controlled display-topology-recovery role — how display-topology recovery is named, so the preserved
/// monitor-affinity hint, windows staying visible after remap, dialogs staying reachable after remap, and
/// preserved follow / presentation intent follow one restore registry rather than stranding a window
/// off-screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DisplayTopologyRecoveryRole {
    /// Monitor-affinity hint preserved.
    MonitorAffinityHintPreserved,
    /// Windows stay visible after remap.
    WindowsStayVisibleAfterRemap,
    /// Dialogs stay reachable after remap.
    DialogsStayReachableAfterRemap,
    /// Follow and presentation intent preserved.
    FollowAndPresentationIntentPreserved,
    /// A role bound to the single restore registry.
    BoundToWindowRestoreRegistry,
    /// An off-screen stranded window, which is disallowed.
    OffscreenStrandedWindowDisallowed,
}

impl M5DisplayTopologyRecoveryRole {
    /// Every display-topology-recovery role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::MonitorAffinityHintPreserved,
        Self::WindowsStayVisibleAfterRemap,
        Self::DialogsStayReachableAfterRemap,
        Self::FollowAndPresentationIntentPreserved,
        Self::BoundToWindowRestoreRegistry,
        Self::OffscreenStrandedWindowDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MonitorAffinityHintPreserved => "monitor_affinity_hint_preserved",
            Self::WindowsStayVisibleAfterRemap => "windows_stay_visible_after_remap",
            Self::DialogsStayReachableAfterRemap => "dialogs_stay_reachable_after_remap",
            Self::FollowAndPresentationIntentPreserved => {
                "follow_and_presentation_intent_preserved"
            }
            Self::BoundToWindowRestoreRegistry => "bound_to_window_restore_registry",
            Self::OffscreenStrandedWindowDisallowed => "offscreen_stranded_window_disallowed",
        }
    }
}

/// Claimed M5 surface family that renders / consumes a window-restore family. No family may invent a
/// parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowRestoreSurfaceFamily {
    /// The shell surface.
    Shell,
    /// The recovery surface.
    Recovery,
    /// The diagnostics surface.
    Diagnostics,
    /// The admin surface.
    Admin,
    /// The docs / help surface.
    DocsHelp,
    /// The support export.
    SupportExport,
}

impl M5WindowRestoreSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Shell,
        Self::Recovery,
        Self::Diagnostics,
        Self::Admin,
        Self::DocsHelp,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::Recovery => "recovery",
            Self::Diagnostics => "diagnostics",
            Self::Admin => "admin",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
        }
    }
}

/// Restore context a family must survive with the same truth, so a family's authority, topology, skeleton,
/// hydration, fidelity, or display-affinity meaning never silently narrows or widens between restore shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowRestoreDeploymentLine {
    /// A cold start from a serialized workspace.
    ColdStart,
    /// A warm restore into an already-running shell.
    WarmRestore,
    /// A crash-loop recovery.
    CrashLoopRecovery,
    /// A multi-monitor restore.
    MultiMonitor,
    /// A remote-target reconnect.
    RemoteReconnect,
}

impl M5WindowRestoreDeploymentLine {
    /// Every restore context, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ColdStart,
        Self::WarmRestore,
        Self::CrashLoopRecovery,
        Self::MultiMonitor,
        Self::RemoteReconnect,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ColdStart => "cold_start",
            Self::WarmRestore => "warm_restore",
            Self::CrashLoopRecovery => "crash_loop_recovery",
            Self::MultiMonitor => "multi_monitor",
            Self::RemoteReconnect => "remote_reconnect",
        }
    }
}

/// Subsystem that consumes a family's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowRestoreConsumerSurface {
    /// The restore coordinator.
    RestoreCoordinator,
    /// The shell UI.
    ShellUi,
    /// The workspace service.
    WorkspaceService,
    /// The session service.
    SessionService,
    /// The diagnostics surface.
    Diagnostics,
    /// The docs / help surface.
    DocsHelp,
    /// The CLI / export path.
    CliExport,
    /// The support export.
    SupportExport,
    /// The general product UI.
    ProductUi,
}

impl M5WindowRestoreConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::RestoreCoordinator,
        Self::ShellUi,
        Self::WorkspaceService,
        Self::SessionService,
        Self::Diagnostics,
        Self::DocsHelp,
        Self::CliExport,
        Self::SupportExport,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestoreCoordinator => "restore_coordinator",
            Self::ShellUi => "shell_ui",
            Self::WorkspaceService => "workspace_service",
            Self::SessionService => "session_service",
            Self::Diagnostics => "diagnostics",
            Self::DocsHelp => "docs_help",
            Self::CliExport => "cli_export",
            Self::SupportExport => "support_export",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every family must offer so no window-restore meaning disappears under
/// zoom, high contrast, keyboard-only use, or export. Records the keyboard, screen-reader, high-zoom,
/// high-contrast, CLI/export, and support-packet requirements up front.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowRestoreAccessibilityRoute {
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

impl M5WindowRestoreAccessibilityRoute {
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

/// Reason a window-restore family has degraded below its qualified state. Required on every row so a stale,
/// unresolved, or narrowed fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowRestoreDegradedReason {
    /// Proof has gone stale.
    ProofStale,
    /// The window-topology registry source is unavailable.
    WindowTopologySourceUnavailable,
    /// The restore-fidelity source is unavailable.
    RestoreFidelitySourceUnavailable,
    /// Session-hydration no-rerun evidence is unverified.
    SessionHydrationEvidenceUnverified,
    /// Skeleton-rebuild evidence is unverified.
    SkeletonRebuildEvidenceUnverified,
    /// Display-topology recovery evidence is unavailable.
    DisplayTopologyEvidenceUnavailable,
}

impl M5WindowRestoreDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::WindowTopologySourceUnavailable,
        Self::RestoreFidelitySourceUnavailable,
        Self::SessionHydrationEvidenceUnverified,
        Self::SkeletonRebuildEvidenceUnverified,
        Self::DisplayTopologyEvidenceUnavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::WindowTopologySourceUnavailable => "window_topology_source_unavailable",
            Self::RestoreFidelitySourceUnavailable => "restore_fidelity_source_unavailable",
            Self::SessionHydrationEvidenceUnverified => "session_hydration_evidence_unverified",
            Self::SkeletonRebuildEvidenceUnverified => "skeleton_rebuild_evidence_unverified",
            Self::DisplayTopologyEvidenceUnavailable => "display_topology_evidence_unavailable",
        }
    }
}

/// Mandatory label a claimed window-restore family must be able to show. The first three are hard
/// requirements on every family; the remaining three close the acceptance-criteria ambiguity about the
/// workspace authority, the restore-fidelity class, and the display affinity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowRestoreRequiredLabel {
    /// The family's stable identity.
    Identity,
    /// The family's window-restore role.
    SemanticRole,
    /// The canonical registry reference the family points at.
    RegistryReference,
    /// The workspace authority the family binds.
    WorkspaceAuthority,
    /// The restore-fidelity class the family claims.
    RestoreFidelityClass,
    /// The display affinity the family preserves.
    DisplayAffinity,
}

impl M5WindowRestoreRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::WorkspaceAuthority,
        Self::RestoreFidelityClass,
        Self::DisplayAffinity,
    ];

    /// The three labels every claimed family must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::RegistryReference];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::RegistryReference => "registry_reference",
            Self::WorkspaceAuthority => "workspace_authority",
            Self::RestoreFidelityClass => "restore_fidelity_class",
            Self::DisplayAffinity => "display_affinity",
        }
    }
}

/// Qualification class for an M5 window-restore row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowRestoreQualificationClass {
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

impl M5WindowRestoreQualificationClass {
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

/// Downgrade trigger that narrows a window-restore family below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowRestoreDowngradeTrigger {
    /// Restore reran commands or reattached privileged sessions implicitly.
    ReranCommandsOrReattachedPrivilegedSessionsImplicitlyDuringRestore,
    /// A missing extension or remote target silently deleted layout structure.
    DeletedLayoutStructureSilentlyOnMissingExtensionOrRemoteTarget,
    /// A display-topology remap left windows or dialogs unreachable.
    LeftWindowsOrDialogsUnreachableAfterDisplayTopologyRemap,
    /// Workspace authority and window topology were merged into one opaque blob.
    MergedWorkspaceAuthorityAndWindowTopologyIntoOneOpaqueBlob,
    /// A restore-fidelity claim outpaced exact / compatible / layout-only evidence.
    OverclaimedRestoreFidelityWhenOnlyContextOrEvidenceReopened,
    /// A window-topology boundary drifted by surface instead of following one registry.
    WindowTopologyBoundaryDriftedBySurface,
    /// A family left its workspace authority unstated.
    WorkspaceAuthorityUnstated,
    /// A family left its restore-fidelity class unstated.
    RestoreFidelityClassUnstated,
    /// A family left its display affinity unstated.
    DisplayAffinityUnstated,
    /// A family left its canonical registry reference unstated.
    RegistryReferenceUnstated,
    /// A family left its session-hydration no-rerun rule unstated.
    SessionHydrationRuleUnstated,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5WindowRestoreDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ReranCommandsOrReattachedPrivilegedSessionsImplicitlyDuringRestore,
        Self::DeletedLayoutStructureSilentlyOnMissingExtensionOrRemoteTarget,
        Self::LeftWindowsOrDialogsUnreachableAfterDisplayTopologyRemap,
        Self::MergedWorkspaceAuthorityAndWindowTopologyIntoOneOpaqueBlob,
        Self::OverclaimedRestoreFidelityWhenOnlyContextOrEvidenceReopened,
        Self::WindowTopologyBoundaryDriftedBySurface,
        Self::WorkspaceAuthorityUnstated,
        Self::RestoreFidelityClassUnstated,
        Self::DisplayAffinityUnstated,
        Self::RegistryReferenceUnstated,
        Self::SessionHydrationRuleUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReranCommandsOrReattachedPrivilegedSessionsImplicitlyDuringRestore => {
                "reran_commands_or_reattached_privileged_sessions_implicitly_during_restore"
            }
            Self::DeletedLayoutStructureSilentlyOnMissingExtensionOrRemoteTarget => {
                "deleted_layout_structure_silently_on_missing_extension_or_remote_target"
            }
            Self::LeftWindowsOrDialogsUnreachableAfterDisplayTopologyRemap => {
                "left_windows_or_dialogs_unreachable_after_display_topology_remap"
            }
            Self::MergedWorkspaceAuthorityAndWindowTopologyIntoOneOpaqueBlob => {
                "merged_workspace_authority_and_window_topology_into_one_opaque_blob"
            }
            Self::OverclaimedRestoreFidelityWhenOnlyContextOrEvidenceReopened => {
                "overclaimed_restore_fidelity_when_only_context_or_evidence_reopened"
            }
            Self::WindowTopologyBoundaryDriftedBySurface => {
                "window_topology_boundary_drifted_by_surface"
            }
            Self::WorkspaceAuthorityUnstated => "workspace_authority_unstated",
            Self::RestoreFidelityClassUnstated => "restore_fidelity_class_unstated",
            Self::DisplayAffinityUnstated => "display_affinity_unstated",
            Self::RegistryReferenceUnstated => "registry_reference_unstated",
            Self::SessionHydrationRuleUnstated => "session_hydration_rule_unstated",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed window-restore family bound to the surface-specific truth it must
/// project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WindowRestoreRow {
    /// Governed window-restore family.
    pub window_restore_family: M5WindowRestoreFamily,
    /// Qualification class earned by this family.
    pub qualification: M5WindowRestoreQualificationClass,
    /// Owner role accountable for keeping this family governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this family.
    pub surface_families: Vec<M5WindowRestoreSurfaceFamily>,
    /// Restore contexts this family keeps the same truth across.
    pub deployment_lines: Vec<M5WindowRestoreDeploymentLine>,
    /// Mandatory labels this family must be able to show (must include the three
    /// [`M5WindowRestoreRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5WindowRestoreRequiredLabel>,
    /// Window-restore roles this family can carry (the frozen AC vocabulary; required on every family).
    pub semantic_roles: Vec<M5WindowRestoreRole>,
    /// Shared-workspace-authority roles this family names (shared-authority family only).
    pub shared_workspace_authority_roles: Vec<M5SharedWorkspaceAuthorityRole>,
    /// Window-local-topology roles this family names (window-local family only).
    pub window_local_topology_roles: Vec<M5WindowLocalTopologyRole>,
    /// Skeleton-first-restore roles this family names (skeleton-restore family only).
    pub skeleton_first_restore_roles: Vec<M5SkeletonFirstRestoreRole>,
    /// No-rerun-session-hydration roles this family names (no-rerun family only).
    pub no_rerun_session_hydration_roles: Vec<M5NoRerunSessionHydrationRole>,
    /// Display-topology-recovery roles this family names (display-recovery family only).
    pub display_topology_recovery_roles: Vec<M5DisplayTopologyRecoveryRole>,
    /// Degraded reasons this family can name (required on every family).
    pub degraded_reasons: Vec<M5WindowRestoreDegradedReason>,
    /// Non-visual accessibility routes this family offers.
    pub accessibility_routes: Vec<M5WindowRestoreAccessibilityRoute>,
    /// Subsystems that consume this family's projection.
    pub consumer_surfaces: Vec<M5WindowRestoreConsumerSurface>,
    /// Downgrade triggers that apply to this family.
    pub downgrade_triggers: Vec<M5WindowRestoreDowngradeTrigger>,
    /// Proof packet refs that keep this family current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this family (must include its own canonical domain schema so
    /// downstream surfaces have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this family never reruns commands or reattaches privileged sessions implicitly during
    /// restore. MUST be `false`.
    pub reruns_commands_or_reattaches_privileged_sessions_implicitly_during_restore: bool,
    /// Hard invariant: this family never lets a missing extension or remote target delete layout structure
    /// silently. MUST be `false`.
    pub deletes_layout_structure_silently_on_missing_extension_or_remote_target: bool,
    /// Hard invariant: this family never leaves windows or dialogs unreachable after a display-topology
    /// remap. MUST be `false`.
    pub leaves_windows_or_dialogs_unreachable_after_display_topology_remap: bool,
    /// Hard invariant: this family never merges workspace-authority state and window-topology state into one
    /// opaque blob. MUST be `false`.
    pub merges_workspace_authority_and_window_topology_into_one_opaque_blob: bool,
    /// Hard invariant: this family never overclaims restore fidelity when the system only reopened context or
    /// evidence. MUST be `false`.
    pub overclaims_restore_fidelity_when_only_context_or_evidence_reopened: bool,
}

impl M5WindowRestoreRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5WindowRestoreRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5WindowRestoreRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.reruns_commands_or_reattaches_privileged_sessions_implicitly_during_restore
            && !self.deletes_layout_structure_silently_on_missing_extension_or_remote_target
            && !self.leaves_windows_or_dialogs_unreachable_after_display_topology_remap
            && !self.merges_workspace_authority_and_window_topology_into_one_opaque_blob
            && !self.overclaims_restore_fidelity_when_only_context_or_evidence_reopened
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WindowRestoreVocabularySet {
    /// Window-restore-family tokens.
    pub window_restore_families: Vec<String>,
    /// Window-restore-role tokens.
    pub semantic_roles: Vec<String>,
    /// Shared-workspace-authority-role tokens.
    pub shared_workspace_authority_roles: Vec<String>,
    /// Window-local-topology-role tokens.
    pub window_local_topology_roles: Vec<String>,
    /// Skeleton-first-restore-role tokens.
    pub skeleton_first_restore_roles: Vec<String>,
    /// No-rerun-session-hydration-role tokens.
    pub no_rerun_session_hydration_roles: Vec<String>,
    /// Display-topology-recovery-role tokens.
    pub display_topology_recovery_roles: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Restore-context tokens.
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

impl M5WindowRestoreVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            window_restore_families: tokens(&M5WindowRestoreFamily::ALL, |v| v.as_str()),
            semantic_roles: tokens(&M5WindowRestoreRole::ALL, |v| v.as_str()),
            shared_workspace_authority_roles: tokens(&M5SharedWorkspaceAuthorityRole::ALL, |v| {
                v.as_str()
            }),
            window_local_topology_roles: tokens(&M5WindowLocalTopologyRole::ALL, |v| v.as_str()),
            skeleton_first_restore_roles: tokens(&M5SkeletonFirstRestoreRole::ALL, |v| v.as_str()),
            no_rerun_session_hydration_roles: tokens(&M5NoRerunSessionHydrationRole::ALL, |v| {
                v.as_str()
            }),
            display_topology_recovery_roles: tokens(&M5DisplayTopologyRecoveryRole::ALL, |v| {
                v.as_str()
            }),
            surface_families: tokens(&M5WindowRestoreSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5WindowRestoreDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5WindowRestoreConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5WindowRestoreAccessibilityRoute::ALL, |v| v.as_str()),
            degraded_reasons: tokens(&M5WindowRestoreDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5WindowRestoreRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5WindowRestoreDowngradeTrigger::ALL, |v| v.as_str()),
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
pub struct M5WindowRestoreGovernanceReview {
    /// Workspace authority and window topology stay separately inspectable.
    pub workspace_authority_and_window_topology_stay_separately_inspectable: bool,
    /// Session-scoped tools never silently rerun or reattach.
    pub session_scoped_tools_never_silently_rerun_or_reattach: bool,
    /// Shared authority never clobbers a window-local selection or focus.
    pub shared_authority_never_clobbers_window_local_selection_or_focus: bool,
    /// Restore rebuilds the layout skeleton before hydrating heavy dependencies.
    pub restore_rebuilds_layout_skeleton_before_hydrating_heavy_dependencies: bool,
    /// Missing extensions or remote targets never delete layout structure silently.
    pub missing_extensions_or_remote_targets_never_delete_layout_structure_silently: bool,
    /// Display-topology changes keep windows and dialogs reachable.
    pub display_topology_changes_keep_windows_and_dialogs_reachable: bool,
    /// Pane trees stay versioned and attributable.
    pub pane_trees_stay_versioned_and_attributable: bool,
    /// Reacquiring broader authority requires an explicit user action.
    pub reacquiring_broader_authority_requires_explicit_user_action: bool,
    /// Restore-fidelity claims never outpace exact / compatible / layout-only evidence.
    pub restore_fidelity_claims_never_outpace_exact_compatible_or_layout_only_evidence: bool,
    /// Every family keeps the same truth across every restore context.
    pub every_family_declares_restore_contexts: bool,
    /// Every family declares a non-visual accessibility route.
    pub every_family_declares_accessibility_route: bool,
    /// Support / export reads a single canonical window-restore source.
    pub support_export_reads_single_window_restore_source: bool,
    /// Shell, recovery, diagnostics, and admin bind to a single canonical window-restore source.
    pub shell_recovery_diagnostics_admin_bind_to_single_window_restore_source: bool,
    /// Later M5 rows cannot invent parallel window-restore vocabulary.
    pub later_rows_cannot_invent_parallel_window_restore_vocabulary: bool,
    /// Restore truth survives zoom and high contrast.
    pub restore_truth_survives_zoom_and_high_contrast: bool,
    /// Claims narrow automatically when the registry is missing, stale, or not yet qualified.
    pub claims_narrow_automatically_when_registry_missing_or_stale: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WindowRestoreConsumerProjection {
    /// Shell and recovery consume the shared window-restore truth.
    pub shell_and_recovery_consume_shared_window_restore_truth: bool,
    /// Diagnostics and admin consume the shared restore-fidelity boundaries.
    pub diagnostics_and_admin_consume_shared_restore_fidelity_boundaries: bool,
    /// Session and workspace services consume the shared window topology.
    pub session_and_workspace_services_consume_shared_window_topology: bool,
    /// Docs, help, and screenshots read a single window-restore source.
    pub docs_help_and_screenshots_read_single_window_restore_source: bool,
    /// Terminal, debug, notebook, and collaboration surfaces bind to the shared no-rerun rule.
    pub terminal_debug_notebook_collab_bind_to_shared_no_rerun_rule: bool,
    /// Support / export reads a single canonical window-restore source.
    pub support_export_reads_single_window_restore_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WindowRestoreProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the family.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the window-restore lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WindowRestoreReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting window-restore audit for the lane.
    pub window_restore_audit_ref: String,
    /// True when support/export parity is required for every family.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every family.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5WindowRestoreMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5WindowRestoreMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Window-restore rows.
    pub window_restore_rows: Vec<M5WindowRestoreRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5WindowRestoreVocabularySet,
    /// Governance-review block.
    pub governance_review: M5WindowRestoreGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5WindowRestoreConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5WindowRestoreProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5WindowRestoreReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 window-restore matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WindowRestoreMatrixPacket {
    /// Record kind; must equal [`M5_WINDOW_RESTORE_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_WINDOW_RESTORE_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Window-restore rows.
    pub window_restore_rows: Vec<M5WindowRestoreRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5WindowRestoreVocabularySet,
    /// Governance-review block.
    pub governance_review: M5WindowRestoreGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5WindowRestoreConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5WindowRestoreProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5WindowRestoreReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5WindowRestoreMatrixPacket {
    /// Builds an M5 window-restore matrix packet from stable-lane input.
    pub fn new(input: M5WindowRestoreMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_WINDOW_RESTORE_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_WINDOW_RESTORE_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            window_restore_rows: input.window_restore_rows,
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

    /// Validates the M5 window-restore matrix invariants.
    pub fn validate(&self) -> Vec<M5WindowRestoreMatrixViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_WINDOW_RESTORE_MATRIX_RECORD_KIND {
            violations.push(M5WindowRestoreMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_WINDOW_RESTORE_MATRIX_SCHEMA_VERSION {
            violations.push(M5WindowRestoreMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5WindowRestoreMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_window_restore_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 window-restore matrix serializes"),
        ) {
            violations.push(M5WindowRestoreMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 window-restore matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "window_restore_family,qualification,owner,canonical_schema,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.window_restore_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.window_restore_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.window_restore_family.canonical_domain_schema_ref(),
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
            .window_restore_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Workspace-Window, Shared-Authority, Skeleton-Restore, and No-Rerun Session-Hydration Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Window-restore families: {} ({} stable)\n",
            self.window_restore_rows.len(),
            stable_families
        ));
        out.push_str(&format!(
            "- Window-restore roles: {}\n",
            self.vocabulary_set.semantic_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Shared-workspace-authority roles: {}\n",
            self.vocabulary_set
                .shared_workspace_authority_roles
                .join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Window-restore families\n\n");
        for row in &self.window_restore_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.window_restore_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.window_restore_family.canonical_domain_schema_ref()
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

/// Errors emitted when reading the checked-in M5 window-restore matrix export.
#[derive(Debug)]
pub enum M5WindowRestoreMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5WindowRestoreMatrixViolation>),
}

impl fmt::Display for M5WindowRestoreMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 window-restore matrix export parse failed: {error}"
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
                    "m5 window-restore matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5WindowRestoreMatrixArtifactError {}

/// Validation failures emitted by [`M5WindowRestoreMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5WindowRestoreMatrixViolation {
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
    /// A required governed window-restore family is missing from the matrix.
    RequiredFamilyMissing,
    /// A window-restore row is incomplete.
    WindowRestoreRowIncomplete,
    /// A window-restore row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A window-restore row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A family declares no window-restore roles.
    SemanticRoleMissing,
    /// The shared-authority family declares no shared-workspace-authority roles.
    SharedWorkspaceAuthorityRoleMissing,
    /// The window-local family declares no window-local-topology roles.
    WindowLocalTopologyRoleMissing,
    /// The skeleton-restore family declares no skeleton-first-restore roles.
    SkeletonFirstRestoreRoleMissing,
    /// The no-rerun family declares no no-rerun-session-hydration roles.
    NoRerunSessionHydrationRoleMissing,
    /// The display-recovery family declares no display-topology-recovery roles.
    DisplayTopologyRecoveryRoleMissing,
    /// A family declares no degraded reasons.
    DegradedReasonMissing,
    /// A family declares no surface families.
    SurfaceFamilyMissing,
    /// A family declares no restore contexts.
    DeploymentLineMissing,
    /// A family declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A family declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A family declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A family claiming Stable is missing required proof packet refs.
    StableFamilyMissingProof,
    /// A family violates a hard invariant (rerunning commands or reattaching privileged sessions implicitly
    /// during restore, deleting layout structure silently on a missing extension or remote target, leaving
    /// windows or dialogs unreachable after a display-topology remap, merging workspace-authority and
    /// window-topology state into one opaque blob, or overclaiming restore fidelity when only context or
    /// evidence was reopened).
    WindowRestoreInvariantViolated,
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

impl M5WindowRestoreMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredFamilyMissing => "required_family_missing",
            Self::WindowRestoreRowIncomplete => "window_restore_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::SharedWorkspaceAuthorityRoleMissing => "shared_workspace_authority_role_missing",
            Self::WindowLocalTopologyRoleMissing => "window_local_topology_role_missing",
            Self::SkeletonFirstRestoreRoleMissing => "skeleton_first_restore_role_missing",
            Self::NoRerunSessionHydrationRoleMissing => "no_rerun_session_hydration_role_missing",
            Self::DisplayTopologyRecoveryRoleMissing => "display_topology_recovery_role_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableFamilyMissingProof => "stable_family_missing_proof",
            Self::WindowRestoreInvariantViolated => "window_restore_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 window-restore matrix export.
pub fn current_stable_m5_window_restore_matrix_export(
) -> Result<M5WindowRestoreMatrixPacket, M5WindowRestoreMatrixArtifactError> {
    let packet: M5WindowRestoreMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-window-restore-proof/support_export.json"
    )))
    .map_err(M5WindowRestoreMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5WindowRestoreMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5WindowRestoreMatrixPacket,
    violations: &mut Vec<M5WindowRestoreMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF,
        M5_WINDOW_RESTORE_MATRIX_DOC_REF,
        M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF,
        M5_RESTORE_FIDELITY_SCHEMA_REF,
        M5_MULTI_WINDOW_PARITY_SCHEMA_REF,
        M5_MONITOR_GEOMETRY_REMAP_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5WindowRestoreMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5WindowRestoreMatrixPacket,
    violations: &mut Vec<M5WindowRestoreMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5WindowRestoreMatrixViolation::VocabularySetDrift);
    }
}

fn validate_window_restore_rows(
    packet: &M5WindowRestoreMatrixPacket,
    violations: &mut Vec<M5WindowRestoreMatrixViolation>,
) {
    let present: BTreeSet<M5WindowRestoreFamily> = packet
        .window_restore_rows
        .iter()
        .map(|row| row.window_restore_family)
        .collect();
    for required in M5WindowRestoreFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5WindowRestoreMatrixViolation::RequiredFamilyMissing);
            return;
        }
    }

    for row in &packet.window_restore_rows {
        let family = row.window_restore_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5WindowRestoreMatrixViolation::WindowRestoreRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5WindowRestoreMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == family.canonical_domain_schema_ref())
        {
            violations.push(M5WindowRestoreMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5WindowRestoreMatrixViolation::SemanticRoleMissing);
        }
        if family.declares_shared_workspace_authority_roles()
            && row.shared_workspace_authority_roles.is_empty()
        {
            violations.push(M5WindowRestoreMatrixViolation::SharedWorkspaceAuthorityRoleMissing);
        }
        if family.declares_window_local_topology_roles()
            && row.window_local_topology_roles.is_empty()
        {
            violations.push(M5WindowRestoreMatrixViolation::WindowLocalTopologyRoleMissing);
        }
        if family.declares_skeleton_first_restore_roles()
            && row.skeleton_first_restore_roles.is_empty()
        {
            violations.push(M5WindowRestoreMatrixViolation::SkeletonFirstRestoreRoleMissing);
        }
        if family.declares_no_rerun_session_hydration_roles()
            && row.no_rerun_session_hydration_roles.is_empty()
        {
            violations.push(M5WindowRestoreMatrixViolation::NoRerunSessionHydrationRoleMissing);
        }
        if family.declares_display_topology_recovery_roles()
            && row.display_topology_recovery_roles.is_empty()
        {
            violations.push(M5WindowRestoreMatrixViolation::DisplayTopologyRecoveryRoleMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5WindowRestoreMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5WindowRestoreMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5WindowRestoreMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5WindowRestoreMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5WindowRestoreMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5WindowRestoreMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5WindowRestoreMatrixViolation::StableFamilyMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5WindowRestoreMatrixViolation::WindowRestoreInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5WindowRestoreMatrixPacket,
    violations: &mut Vec<M5WindowRestoreMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.workspace_authority_and_window_topology_stay_separately_inspectable,
        review.session_scoped_tools_never_silently_rerun_or_reattach,
        review.shared_authority_never_clobbers_window_local_selection_or_focus,
        review.restore_rebuilds_layout_skeleton_before_hydrating_heavy_dependencies,
        review.missing_extensions_or_remote_targets_never_delete_layout_structure_silently,
        review.display_topology_changes_keep_windows_and_dialogs_reachable,
        review.pane_trees_stay_versioned_and_attributable,
        review.reacquiring_broader_authority_requires_explicit_user_action,
        review.restore_fidelity_claims_never_outpace_exact_compatible_or_layout_only_evidence,
        review.every_family_declares_restore_contexts,
        review.every_family_declares_accessibility_route,
        review.support_export_reads_single_window_restore_source,
        review.shell_recovery_diagnostics_admin_bind_to_single_window_restore_source,
        review.later_rows_cannot_invent_parallel_window_restore_vocabulary,
        review.restore_truth_survives_zoom_and_high_contrast,
        review.claims_narrow_automatically_when_registry_missing_or_stale,
    ] {
        if !ok {
            violations.push(M5WindowRestoreMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5WindowRestoreMatrixPacket,
    violations: &mut Vec<M5WindowRestoreMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_and_recovery_consume_shared_window_restore_truth,
        projection.diagnostics_and_admin_consume_shared_restore_fidelity_boundaries,
        projection.session_and_workspace_services_consume_shared_window_topology,
        projection.docs_help_and_screenshots_read_single_window_restore_source,
        projection.terminal_debug_notebook_collab_bind_to_shared_no_rerun_rule,
        projection.support_export_reads_single_window_restore_source,
    ] {
        if !ok {
            violations.push(M5WindowRestoreMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5WindowRestoreMatrixPacket,
    violations: &mut Vec<M5WindowRestoreMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5WindowRestoreMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5WindowRestoreMatrixPacket,
    violations: &mut Vec<M5WindowRestoreMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.window_restore_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5WindowRestoreMatrixViolation::ReleasePostureIncomplete);
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
/// deliberately uses workspace / window / restore / session / topology words; what is rejected is a raw
/// secret *value* shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
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
