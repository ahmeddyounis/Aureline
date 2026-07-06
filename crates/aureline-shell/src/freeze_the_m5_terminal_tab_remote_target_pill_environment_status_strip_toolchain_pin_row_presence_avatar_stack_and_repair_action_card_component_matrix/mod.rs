//! Frozen M5 terminal-tab, remote-target-pill, environment-status-strip,
//! toolchain-pin-row, presence-avatar-stack, and repair-action-card component
//! matrix.
//!
//! This module locks Aureline's reusable runtime-boundary and repair components
//! into one export-safe packet. Every component family M5 claims that still
//! drifts too easily by feature lane — the terminal tab/header, the remote target
//! pill, the environment status strip, the toolchain pin row, the presence avatar
//! stack, and the repair action card — is named once here, bound to a canonical
//! shell zone, responsive class, and window class, and constrained by the same
//! host-boundary, session-liveness, resolved-source, collaboration-role, and
//! reversibility rules regardless of the surface family that renders it.
//!
//! The shell topology this matrix binds against — the eight canonical shell
//! zones, the compact/standard/expanded responsive classes, the window classes,
//! the consumer surfaces, and the ten claimed M5 surface families — is the one
//! already frozen by
//! [`crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix`];
//! this matrix re-exports that vocabulary rather than minting parallel terms.
//! What this matrix adds is the stable vocabulary for the *components* themselves:
//! the component families, the shell-integration qualities and session-liveness
//! states, the host-boundary classes and remote connection states, the runtime
//! and toolchain source classes and pin states, the collaboration roles and
//! follow states, the repair blast radii and reversibility classes, the non-visual
//! accessibility routes, and the mandatory labels every component must be able to
//! show.
//!
//! The matrix is the single source of truth for whether a claimed M5
//! runtime-boundary or repair component may publish a terminal, remote, environment,
//! toolchain, presence, or repair claim. Terminal/session surfaces, remote and
//! environment surfaces, collaboration surfaces, and repair surfaces all consume
//! this packet so one terminal-tab model carries session title, host boundary, and
//! shell-integration quality with live-versus-restored honesty, one remote-target
//! pill names the host boundary and connection state, one environment strip names
//! the winning runtime source, one toolchain pin row explains why a toolchain won,
//! one presence stack shows collaboration role and follow state, and one repair
//! card shows blast radius and reversibility before approval. No M5 lane invents a
//! second status grammar, masks a host/runtime boundary, conflates a live session
//! with a restored one, or overstates reversibility.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5RuntimeBoundaryVocabularySet`] rather than minted per surface. Raw URLs,
//! raw local paths, raw usernames, raw hostnames, tokens, raw diagnostics, private
//! endpoints, credentials, and user text bodies stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-runtime-boundary-components.schema.json`](../../../../schemas/ui/m5-runtime-boundary-components.schema.json)
//! and the contract doc is
//! [`docs/components/m5_runtime_boundary_components_contract.md`](../../../../docs/components/m5_runtime_boundary_components_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-runtime-boundary-components/`](../../../../fixtures/ui/m5-runtime-boundary-components/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_runtime_boundary_component_matrix,
    seeded_m5_runtime_boundary_component_matrix_presence_avatar_stack_beta_narrowed,
    seeded_m5_runtime_boundary_component_matrix_repair_action_card_preview_narrowed,
    M5_RUNTIME_BOUNDARY_MATRIX_PACKET_ID,
};

// The canonical shell topology — zones, responsive classes, window classes,
// consumer surfaces, and the ten claimed M5 surface families — is frozen once, in
// the shell-zone matrix. This matrix reuses it verbatim so no runtime-boundary or
// repair component invents a parallel slot, layout class, window class, or surface
// family.
pub use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix::{
    M5ResponsiveClass, M5ShellConsumerSurface, M5ShellSurfaceFamily, M5ShellZoneSlot, M5WindowClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5RuntimeBoundaryMatrixPacket`].
pub const M5_RUNTIME_BOUNDARY_MATRIX_RECORD_KIND: &str =
    "freeze_m5_terminal_tab_remote_target_pill_environment_status_strip_toolchain_pin_row_presence_avatar_stack_and_repair_action_card_component_matrix";

/// Schema version for M5 runtime-boundary-component-matrix records.
pub const M5_RUNTIME_BOUNDARY_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the runtime-boundary-components boundary schema.
pub const M5_RUNTIME_BOUNDARY_SCHEMA_REF: &str =
    "schemas/ui/m5-runtime-boundary-components.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_RUNTIME_BOUNDARY_DOC_REF: &str =
    "docs/components/m5_runtime_boundary_components_contract.md";

/// Repo-relative path of the frozen shell-zone schema this matrix binds against.
pub const M5_RUNTIME_BOUNDARY_SHELL_ZONE_REF: &str = "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the terminal/session source contract this matrix binds
/// against.
pub const M5_RUNTIME_BOUNDARY_TERMINAL_CONTRACT_REF: &str =
    "schemas/terminal/session_restore_metadata.schema.json";

/// Repo-relative path of the repair-transaction source contract this matrix binds
/// against.
pub const M5_RUNTIME_BOUNDARY_REPAIR_CONTRACT_REF: &str =
    "schemas/doctor/project-doctor-repair-transaction-receipts.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_RUNTIME_BOUNDARY_FIXTURE_DIR: &str = "fixtures/ui/m5-runtime-boundary-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_RUNTIME_BOUNDARY_ARTIFACT_REF: &str =
    "artifacts/release/m5-runtime-boundary-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_RUNTIME_BOUNDARY_CSV_REF: &str =
    "artifacts/release/m5-runtime-boundary-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_RUNTIME_BOUNDARY_REPORT_REF: &str =
    "artifacts/components/m5-runtime-boundary-components.md";

/// One of the six governed runtime-boundary component families this matrix
/// freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RuntimeBoundaryComponentFamily {
    /// A terminal tab / header strip carrying session title, host boundary, and
    /// shell-integration quality.
    TerminalTab,
    /// A remote target pill naming the host boundary and connection state.
    RemoteTargetPill,
    /// An environment status strip naming the winning runtime source.
    EnvironmentStatusStrip,
    /// A toolchain pin row explaining why a toolchain won.
    ToolchainPinRow,
    /// A presence avatar stack showing collaboration role and follow state.
    PresenceAvatarStack,
    /// A repair action card showing blast radius and reversibility before
    /// approval.
    RepairActionCard,
}

impl M5RuntimeBoundaryComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TerminalTab,
        Self::RemoteTargetPill,
        Self::EnvironmentStatusStrip,
        Self::ToolchainPinRow,
        Self::PresenceAvatarStack,
        Self::RepairActionCard,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TerminalTab => "terminal_tab",
            Self::RemoteTargetPill => "remote_target_pill",
            Self::EnvironmentStatusStrip => "environment_status_strip",
            Self::ToolchainPinRow => "toolchain_pin_row",
            Self::PresenceAvatarStack => "presence_avatar_stack",
            Self::RepairActionCard => "repair_action_card",
        }
    }

    /// `true` when this family is a terminal tab and must therefore declare its
    /// shell-integration qualities and session-liveness states.
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::TerminalTab)
    }

    /// `true` when this family is a remote target pill and must therefore declare
    /// its host-boundary classes and connection states.
    pub const fn is_remote_target(self) -> bool {
        matches!(self, Self::RemoteTargetPill)
    }

    /// `true` when this family is an environment status strip and must therefore
    /// declare its runtime source classes.
    pub const fn is_environment(self) -> bool {
        matches!(self, Self::EnvironmentStatusStrip)
    }

    /// `true` when this family is a toolchain pin row and must therefore declare
    /// its toolchain source classes and pin states.
    pub const fn is_toolchain(self) -> bool {
        matches!(self, Self::ToolchainPinRow)
    }

    /// `true` when this family is a presence avatar stack and must therefore
    /// declare its collaboration roles and follow states.
    pub const fn is_presence(self) -> bool {
        matches!(self, Self::PresenceAvatarStack)
    }

    /// `true` when this family is a repair action card and must therefore declare
    /// its blast radii and reversibility classes.
    pub const fn is_repair(self) -> bool {
        matches!(self, Self::RepairActionCard)
    }
}

/// Controlled shell-integration quality — how much of the shell integration a
/// terminal tab actually has, so a tab never implies richer integration than the
/// live session provides.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellIntegrationQuality {
    /// Full shell integration: cwd, command marks, and exit codes are reported.
    FullyIntegrated,
    /// Command marks are reported but cwd is not.
    CommandMarksOnly,
    /// The working directory is reported but command marks are not.
    CwdReportingOnly,
    /// A basic PTY with no shell integration.
    BasicPtyNoIntegration,
    /// Integration was negotiated but is currently degraded.
    IntegrationDegraded,
}

impl M5ShellIntegrationQuality {
    /// Every shell-integration quality, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullyIntegrated,
        Self::CommandMarksOnly,
        Self::CwdReportingOnly,
        Self::BasicPtyNoIntegration,
        Self::IntegrationDegraded,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyIntegrated => "fully_integrated",
            Self::CommandMarksOnly => "command_marks_only",
            Self::CwdReportingOnly => "cwd_reporting_only",
            Self::BasicPtyNoIntegration => "basic_pty_no_integration",
            Self::IntegrationDegraded => "integration_degraded",
        }
    }
}

/// Controlled terminal session liveness — whether the tab is a live session or a
/// restored transcript, so live and restored are never conflated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TerminalSessionLiveness {
    /// A live session attached to the foreground.
    LiveAttached,
    /// A live session running detached in the background.
    LiveDetachedRunning,
    /// A read-only session restored from a saved transcript.
    RestoredFromTranscript,
    /// A session that dropped and is reconnecting.
    Reconnecting,
    /// A session that has exited / closed.
    ClosedExited,
}

impl M5TerminalSessionLiveness {
    /// Every session-liveness state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LiveAttached,
        Self::LiveDetachedRunning,
        Self::RestoredFromTranscript,
        Self::Reconnecting,
        Self::ClosedExited,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveAttached => "live_attached",
            Self::LiveDetachedRunning => "live_detached_running",
            Self::RestoredFromTranscript => "restored_from_transcript",
            Self::Reconnecting => "reconnecting",
            Self::ClosedExited => "closed_exited",
        }
    }
}

/// Controlled host-boundary class — where the shell / process actually runs, so a
/// remote or container boundary is never masked as local.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HostBoundaryClass {
    /// The local machine.
    LocalHost,
    /// A remote host reached over a secure shell connection.
    RemoteSshHost,
    /// A dev container / container host.
    ContainerHost,
    /// A managed remote workspace host.
    ManagedWorkspaceHost,
    /// A virtual machine host.
    VirtualMachineHost,
    /// A sandboxed WebAssembly host.
    WasmSandboxHost,
}

impl M5HostBoundaryClass {
    /// Every host-boundary class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalHost,
        Self::RemoteSshHost,
        Self::ContainerHost,
        Self::ManagedWorkspaceHost,
        Self::VirtualMachineHost,
        Self::WasmSandboxHost,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalHost => "local_host",
            Self::RemoteSshHost => "remote_ssh_host",
            Self::ContainerHost => "container_host",
            Self::ManagedWorkspaceHost => "managed_workspace_host",
            Self::VirtualMachineHost => "virtual_machine_host",
            Self::WasmSandboxHost => "wasm_sandbox_host",
        }
    }
}

/// Controlled remote connection state — the live connection posture of a remote
/// target, so a stale or offline connection is never shown as connected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RemoteConnectionState {
    /// Connected and healthy.
    Connected,
    /// Establishing the connection.
    Connecting,
    /// Dropped and reconnecting.
    Reconnecting,
    /// Disconnected.
    Disconnected,
    /// Serving from an offline / mirrored cache.
    OfflineCached,
}

impl M5RemoteConnectionState {
    /// Every connection state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Connected,
        Self::Connecting,
        Self::Reconnecting,
        Self::Disconnected,
        Self::OfflineCached,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::Connecting => "connecting",
            Self::Reconnecting => "reconnecting",
            Self::Disconnected => "disconnected",
            Self::OfflineCached => "offline_cached",
        }
    }
}

/// Controlled runtime source class — which source won the resolved runtime shown
/// in the environment status strip, so the winning source is always explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RuntimeSourceClass {
    /// Pinned by the project.
    ProjectPinned,
    /// Configured at the workspace level.
    WorkspaceConfigured,
    /// Resolved by a tool / version manager.
    ToolManagerResolved,
    /// The system default.
    SystemDefault,
    /// Provided by the container image.
    ContainerProvided,
    /// A session-scoped override is active.
    SessionOverride,
}

impl M5RuntimeSourceClass {
    /// Every runtime source class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProjectPinned,
        Self::WorkspaceConfigured,
        Self::ToolManagerResolved,
        Self::SystemDefault,
        Self::ContainerProvided,
        Self::SessionOverride,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectPinned => "project_pinned",
            Self::WorkspaceConfigured => "workspace_configured",
            Self::ToolManagerResolved => "tool_manager_resolved",
            Self::SystemDefault => "system_default",
            Self::ContainerProvided => "container_provided",
            Self::SessionOverride => "session_override",
        }
    }
}

/// Controlled toolchain source class — why a specific toolchain won, so a pin row
/// never hides the origin of the resolved toolchain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ToolchainSourceClass {
    /// A checked-in pin file selected the toolchain.
    PinFile,
    /// A workspace setting selected the toolchain.
    WorkspaceSetting,
    /// A version manager selected the toolchain.
    VersionManager,
    /// A system-installed toolchain was used.
    SystemInstalled,
    /// The container image provided the toolchain.
    ContainerImage,
    /// A session override selected the toolchain.
    SessionOverride,
}

impl M5ToolchainSourceClass {
    /// Every toolchain source class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PinFile,
        Self::WorkspaceSetting,
        Self::VersionManager,
        Self::SystemInstalled,
        Self::ContainerImage,
        Self::SessionOverride,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PinFile => "pin_file",
            Self::WorkspaceSetting => "workspace_setting",
            Self::VersionManager => "version_manager",
            Self::SystemInstalled => "system_installed",
            Self::ContainerImage => "container_image",
            Self::SessionOverride => "session_override",
        }
    }
}

/// Controlled toolchain pin state — the pin/resolution posture of a toolchain pin
/// row, so a missing, conflicting, or overridden pin is never shown as cleanly
/// resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ToolchainPinState {
    /// The pinned toolchain resolved cleanly.
    PinnedResolved,
    /// The pinned toolchain is missing and a fallback is in use, disclosed.
    PinnedMissingFallback,
    /// No pin is set and a default is in use.
    Unpinned,
    /// Multiple pins disagree and the conflict is disclosed.
    PinConflict,
    /// An override supersedes the pin, disclosed.
    PinOverridden,
}

impl M5ToolchainPinState {
    /// Every toolchain pin state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PinnedResolved,
        Self::PinnedMissingFallback,
        Self::Unpinned,
        Self::PinConflict,
        Self::PinOverridden,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PinnedResolved => "pinned_resolved",
            Self::PinnedMissingFallback => "pinned_missing_fallback",
            Self::Unpinned => "unpinned",
            Self::PinConflict => "pin_conflict",
            Self::PinOverridden => "pin_overridden",
        }
    }
}

/// Controlled collaboration role — the role a participant holds in a shared
/// session, so a presence stack never conflates an observer with a controller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationRole {
    /// The session host / owner.
    SessionHost,
    /// A collaborator with edit capability.
    Collaborator,
    /// The active presenter.
    Presenter,
    /// A read-only observer.
    Observer,
    /// The holder of the shared control token.
    ControlHolder,
}

impl M5CollaborationRole {
    /// Every collaboration role, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SessionHost,
        Self::Collaborator,
        Self::Presenter,
        Self::Observer,
        Self::ControlHolder,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionHost => "session_host",
            Self::Collaborator => "collaborator",
            Self::Presenter => "presenter",
            Self::Observer => "observer",
            Self::ControlHolder => "control_holder",
        }
    }
}

/// Controlled follow state — the follow / presentation posture of a participant,
/// so a presence stack always makes who-follows-whom explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FollowState {
    /// Following the active presenter.
    FollowingPresenter,
    /// Being followed by others.
    BeingFollowed,
    /// Not following anyone.
    NotFollowing,
    /// Presenting to others.
    PresentingToOthers,
    /// Following is paused.
    FollowPaused,
}

impl M5FollowState {
    /// Every follow state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FollowingPresenter,
        Self::BeingFollowed,
        Self::NotFollowing,
        Self::PresentingToOthers,
        Self::FollowPaused,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FollowingPresenter => "following_presenter",
            Self::BeingFollowed => "being_followed",
            Self::NotFollowing => "not_following",
            Self::PresentingToOthers => "presenting_to_others",
            Self::FollowPaused => "follow_paused",
        }
    }
}

/// Controlled repair blast radius — how far a repair action reaches, so a repair
/// card never understates what a repair will change before approval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepairBlastRadius {
    /// No writes: a preview / diagnostic only.
    NoWritesPreview,
    /// Writes scoped to the workspace.
    WorkspaceScoped,
    /// Writes that change the toolchain.
    ToolchainScoped,
    /// Writes that change the host environment.
    HostEnvironmentScoped,
    /// Writes that reach multiple targets (local plus remote / container).
    MultiTargetScoped,
}

impl M5RepairBlastRadius {
    /// Every repair blast radius, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NoWritesPreview,
        Self::WorkspaceScoped,
        Self::ToolchainScoped,
        Self::HostEnvironmentScoped,
        Self::MultiTargetScoped,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoWritesPreview => "no_writes_preview",
            Self::WorkspaceScoped => "workspace_scoped",
            Self::ToolchainScoped => "toolchain_scoped",
            Self::HostEnvironmentScoped => "host_environment_scoped",
            Self::MultiTargetScoped => "multi_target_scoped",
        }
    }
}

/// Controlled reversibility class — the reversal posture of a repair action, so a
/// repair card never overstates how reversible a repair is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReversibilityClass {
    /// Fully reversible via a checkpoint and rollback.
    FullyReversibleCheckpoint,
    /// Reversible via a taken backup and restore path.
    ReversibleWithBackup,
    /// Partially reversible, with the irreversible steps disclosed.
    PartiallyReversible,
    /// Irreversible, requiring explicit confirmation.
    IrreversibleConfirmed,
    /// Reversal requires manual steps, disclosed.
    ReversalRequiresManualSteps,
}

impl M5ReversibilityClass {
    /// Every reversibility class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullyReversibleCheckpoint,
        Self::ReversibleWithBackup,
        Self::PartiallyReversible,
        Self::IrreversibleConfirmed,
        Self::ReversalRequiresManualSteps,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyReversibleCheckpoint => "fully_reversible_checkpoint",
            Self::ReversibleWithBackup => "reversible_with_backup",
            Self::PartiallyReversible => "partially_reversible",
            Self::IrreversibleConfirmed => "irreversible_confirmed",
            Self::ReversalRequiresManualSteps => "reversal_requires_manual_steps",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no truth is
/// hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RuntimeBoundaryAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5RuntimeBoundaryAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed runtime-boundary component must be able to show. The
/// first three are hard requirements on every component per the guardrails.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RuntimeBoundaryRequiredLabel {
    /// The component's stable identity / session title / what it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The host / runtime boundary the component reports.
    Boundary,
    /// The winning resolved source / toolchain / scope.
    ResolvedSource,
    /// The reversibility / audit-reopen path for the component's action.
    Reversibility,
}

impl M5RuntimeBoundaryRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::Boundary,
        Self::ResolvedSource,
        Self::Reversibility,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::Boundary => "boundary",
            Self::ResolvedSource => "resolved_source",
            Self::Reversibility => "reversibility",
        }
    }
}

/// Qualification class for an M5 runtime-boundary-component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RuntimeBoundaryQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5RuntimeBoundaryQualificationClass {
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

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a runtime-boundary component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RuntimeBoundaryDowngradeTrigger {
    /// A terminal tab hid its shell-integration quality.
    ShellIntegrationQualityHidden,
    /// A terminal tab left live-versus-restored ambiguous.
    SessionLivenessAmbiguous,
    /// A remote pill masked the host boundary as local.
    HostBoundaryMasked,
    /// A remote pill showed a stale connection state.
    ConnectionStateStale,
    /// An environment strip left the winning runtime source unexplained.
    RuntimeSourceUnexplained,
    /// A toolchain row hid a pin conflict.
    ToolchainPinConflictHidden,
    /// A presence stack masked a collaboration role.
    CollaborationRoleMasked,
    /// A presence stack left the follow state ambiguous.
    FollowStateAmbiguous,
    /// A repair card understated its blast radius.
    RepairBlastRadiusUnderstated,
    /// A repair card overstated its reversibility.
    ReversibilityOverstated,
    /// Audit / support truth was lost off the primary surface.
    AuditTruthLostOffPrimarySurface,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5RuntimeBoundaryDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ShellIntegrationQualityHidden,
        Self::SessionLivenessAmbiguous,
        Self::HostBoundaryMasked,
        Self::ConnectionStateStale,
        Self::RuntimeSourceUnexplained,
        Self::ToolchainPinConflictHidden,
        Self::CollaborationRoleMasked,
        Self::FollowStateAmbiguous,
        Self::RepairBlastRadiusUnderstated,
        Self::ReversibilityOverstated,
        Self::AuditTruthLostOffPrimarySurface,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellIntegrationQualityHidden => "shell_integration_quality_hidden",
            Self::SessionLivenessAmbiguous => "session_liveness_ambiguous",
            Self::HostBoundaryMasked => "host_boundary_masked",
            Self::ConnectionStateStale => "connection_state_stale",
            Self::RuntimeSourceUnexplained => "runtime_source_unexplained",
            Self::ToolchainPinConflictHidden => "toolchain_pin_conflict_hidden",
            Self::CollaborationRoleMasked => "collaboration_role_masked",
            Self::FollowStateAmbiguous => "follow_state_ambiguous",
            Self::RepairBlastRadiusUnderstated => "repair_blast_radius_understated",
            Self::ReversibilityOverstated => "reversibility_overstated",
            Self::AuditTruthLostOffPrimarySurface => "audit_truth_lost_off_primary_surface",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed runtime-boundary component family bound to
/// its shell zone, layout classes, and the surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RuntimeBoundaryComponentRow {
    /// Governed component family.
    pub component_family: M5RuntimeBoundaryComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5RuntimeBoundaryQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Canonical shell zone this component attaches to.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Responsive classes this component must survive.
    pub responsive_classes: Vec<M5ResponsiveClass>,
    /// Window classes this component keeps continuity across.
    pub window_classes: Vec<M5WindowClass>,
    /// Claimed M5 surface families that render / consume this component.
    pub surface_families: Vec<M5ShellSurfaceFamily>,
    /// Mandatory labels this component must be able to show (must include the
    /// three [`M5RuntimeBoundaryRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5RuntimeBoundaryRequiredLabel>,
    /// Shell-integration qualities this component projects (terminal only).
    pub shell_integration_qualities: Vec<M5ShellIntegrationQuality>,
    /// Session-liveness states this component distinguishes (terminal only).
    pub session_liveness_states: Vec<M5TerminalSessionLiveness>,
    /// Host-boundary classes this component names (remote-target only).
    pub host_boundary_classes: Vec<M5HostBoundaryClass>,
    /// Remote connection states this component shows (remote-target only).
    pub connection_states: Vec<M5RemoteConnectionState>,
    /// Runtime source classes this component names (environment only).
    pub runtime_source_classes: Vec<M5RuntimeSourceClass>,
    /// Toolchain source classes this component names (toolchain only).
    pub toolchain_source_classes: Vec<M5ToolchainSourceClass>,
    /// Toolchain pin states this component distinguishes (toolchain only).
    pub toolchain_pin_states: Vec<M5ToolchainPinState>,
    /// Collaboration roles this component shows (presence only).
    pub collaboration_roles: Vec<M5CollaborationRole>,
    /// Follow states this component shows (presence only).
    pub follow_states: Vec<M5FollowState>,
    /// Repair blast radii this component discloses (repair only).
    pub repair_blast_radii: Vec<M5RepairBlastRadius>,
    /// Reversibility classes this component discloses (repair only).
    pub reversibility_classes: Vec<M5ReversibilityClass>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5RuntimeBoundaryAccessibilityRoute>,
    /// Shell subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5RuntimeBoundaryDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never masks a host / runtime boundary. MUST
    /// be `false`.
    pub masks_host_or_runtime_boundary: bool,
    /// Hard invariant: this component never conflates a live session with a
    /// restored one. MUST be `false`.
    pub conflates_live_and_restored_session: bool,
    /// Hard invariant: this component never invents a private status grammar. MUST
    /// be `false`.
    pub invents_private_status_grammar: bool,
    /// Hard invariant: this component never overstates reversibility or drops
    /// audit / support truth. MUST be `false`.
    pub overstates_reversibility_or_drops_audit_truth: bool,
}

impl M5RuntimeBoundaryComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5RuntimeBoundaryRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5RuntimeBoundaryRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_host_or_runtime_boundary
            && !self.conflates_live_and_restored_session
            && !self.invents_private_status_grammar
            && !self.overstates_reversibility_or_drops_audit_truth
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RuntimeBoundaryVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Shell-integration-quality tokens.
    pub shell_integration_qualities: Vec<String>,
    /// Session-liveness-state tokens.
    pub session_liveness_states: Vec<String>,
    /// Host-boundary-class tokens.
    pub host_boundary_classes: Vec<String>,
    /// Connection-state tokens.
    pub connection_states: Vec<String>,
    /// Runtime-source-class tokens.
    pub runtime_source_classes: Vec<String>,
    /// Toolchain-source-class tokens.
    pub toolchain_source_classes: Vec<String>,
    /// Toolchain-pin-state tokens.
    pub toolchain_pin_states: Vec<String>,
    /// Collaboration-role tokens.
    pub collaboration_roles: Vec<String>,
    /// Follow-state tokens.
    pub follow_states: Vec<String>,
    /// Repair-blast-radius tokens.
    pub repair_blast_radii: Vec<String>,
    /// Reversibility-class tokens.
    pub reversibility_classes: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5RuntimeBoundaryVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5RuntimeBoundaryComponentFamily::ALL, |v| v.as_str()),
            shell_integration_qualities: tokens(&M5ShellIntegrationQuality::ALL, |v| v.as_str()),
            session_liveness_states: tokens(&M5TerminalSessionLiveness::ALL, |v| v.as_str()),
            host_boundary_classes: tokens(&M5HostBoundaryClass::ALL, |v| v.as_str()),
            connection_states: tokens(&M5RemoteConnectionState::ALL, |v| v.as_str()),
            runtime_source_classes: tokens(&M5RuntimeSourceClass::ALL, |v| v.as_str()),
            toolchain_source_classes: tokens(&M5ToolchainSourceClass::ALL, |v| v.as_str()),
            toolchain_pin_states: tokens(&M5ToolchainPinState::ALL, |v| v.as_str()),
            collaboration_roles: tokens(&M5CollaborationRole::ALL, |v| v.as_str()),
            follow_states: tokens(&M5FollowState::ALL, |v| v.as_str()),
            repair_blast_radii: tokens(&M5RepairBlastRadius::ALL, |v| v.as_str()),
            reversibility_classes: tokens(&M5ReversibilityClass::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5RuntimeBoundaryAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5RuntimeBoundaryRequiredLabel::ALL, |v| v.as_str()),
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
pub struct M5RuntimeBoundaryGovernanceReview {
    /// The terminal tab shows session title, host boundary, and shell-integration
    /// quality.
    pub terminal_tab_shows_boundary_and_shell_integration: bool,
    /// The remote pill shows host boundary and connection state.
    pub remote_pill_shows_host_boundary_and_connection: bool,
    /// The environment strip names the winning runtime source.
    pub environment_strip_names_winning_runtime_source: bool,
    /// The toolchain row explains why the toolchain won.
    pub toolchain_row_explains_why_toolchain_won: bool,
    /// The presence stack shows collaboration role and follow state.
    pub presence_stack_shows_role_and_follow_state: bool,
    /// The repair card shows blast radius and reversibility before approval.
    pub repair_card_shows_blast_radius_and_reversibility: bool,
    /// Live and restored sessions are never conflated.
    pub live_versus_restored_never_conflated: bool,
    /// No component invents a second status grammar.
    pub no_component_invents_second_status_grammar: bool,
    /// Every component is bound to a canonical shell zone.
    pub every_component_bound_to_shell_zone: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel runtime-boundary vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RuntimeBoundaryConsumerProjection {
    /// Terminal / session surfaces consume the shared terminal vocabulary.
    pub terminal_and_session_surfaces_consume_matrix: bool,
    /// Remote and environment surfaces consume the boundary / source vocabulary.
    pub remote_and_environment_surfaces_consume_boundary_vocabulary: bool,
    /// Collaboration surfaces consume the role / follow vocabulary.
    pub collaboration_surfaces_consume_role_follow_vocabulary: bool,
    /// Repair surfaces consume the blast-radius / reversibility vocabulary.
    pub repair_surfaces_consume_reversibility_vocabulary: bool,
    /// Support / export reads a single canonical runtime-boundary source.
    pub support_export_reads_single_source: bool,
    /// The accessibility bridge reads a single canonical runtime-boundary source.
    pub accessibility_bridge_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RuntimeBoundaryProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the runtime-boundary lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RuntimeBoundaryReleasePosture {
    /// Ref of the supporting release packet for the lane.
    pub release_packet_ref: String,
    /// Ref of the supporting runtime-boundary audit for the lane.
    pub runtime_boundary_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5RuntimeBoundaryMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RuntimeBoundaryMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5RuntimeBoundaryComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RuntimeBoundaryVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RuntimeBoundaryGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RuntimeBoundaryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RuntimeBoundaryProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RuntimeBoundaryReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 runtime-boundary-component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RuntimeBoundaryMatrixPacket {
    /// Record kind; must equal [`M5_RUNTIME_BOUNDARY_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RUNTIME_BOUNDARY_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5RuntimeBoundaryComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RuntimeBoundaryVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RuntimeBoundaryGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RuntimeBoundaryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RuntimeBoundaryProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RuntimeBoundaryReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RuntimeBoundaryMatrixPacket {
    /// Builds an M5 runtime-boundary-component matrix packet from stable-lane
    /// input.
    pub fn new(input: M5RuntimeBoundaryMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_RUNTIME_BOUNDARY_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_RUNTIME_BOUNDARY_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
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

    /// Validates the M5 runtime-boundary-component matrix invariants.
    pub fn validate(&self) -> Vec<M5RuntimeBoundaryMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_RUNTIME_BOUNDARY_MATRIX_RECORD_KIND {
            violations.push(M5RuntimeBoundaryMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RUNTIME_BOUNDARY_MATRIX_SCHEMA_VERSION {
            violations.push(M5RuntimeBoundaryMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5RuntimeBoundaryMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 runtime boundary matrix packet serializes"),
        ) {
            violations.push(M5RuntimeBoundaryMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 runtime boundary matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,shell_zone_slot,responsive_classes,window_classes,surface_families,required_labels,consumer_surfaces\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shell_zone_slot.as_str(),
                join_tokens(&row.responsive_classes, |v| v.as_str()),
                join_tokens(&row.window_classes, |v| v.as_str()),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Terminal-Tab, Remote-Target-Pill, Environment-Status-Strip, Toolchain-Pin-Row, Presence-Avatar-Stack, and Repair-Action-Card Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Host-boundary classes: {}\n",
            self.vocabulary_set.host_boundary_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Reversibility classes: {}\n",
            self.vocabulary_set.reversibility_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Shell zone: `{}`\n",
                row.shell_zone_slot.as_str()
            ));
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

/// Errors emitted when reading the checked-in M5 runtime-boundary matrix export.
#[derive(Debug)]
pub enum M5RuntimeBoundaryMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5RuntimeBoundaryMatrixViolation>),
}

impl fmt::Display for M5RuntimeBoundaryMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 runtime boundary matrix export parse failed: {error}"
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
                    "m5 runtime boundary matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5RuntimeBoundaryMatrixArtifactError {}

/// Validation failures emitted by [`M5RuntimeBoundaryMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5RuntimeBoundaryMatrixViolation {
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
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A terminal component declares no shell-integration qualities.
    ShellIntegrationQualityMissing,
    /// A terminal component declares no session-liveness states.
    SessionLivenessStateMissing,
    /// A remote-target component declares no host-boundary classes.
    HostBoundaryClassMissing,
    /// A remote-target component declares no connection states.
    ConnectionStateMissing,
    /// An environment component declares no runtime source classes.
    RuntimeSourceClassMissing,
    /// A toolchain component declares no toolchain source classes.
    ToolchainSourceClassMissing,
    /// A toolchain component declares no toolchain pin states.
    ToolchainPinStateMissing,
    /// A presence component declares no collaboration roles.
    CollaborationRoleMissing,
    /// A presence component declares no follow states.
    FollowStateMissing,
    /// A repair component declares no blast radii.
    RepairBlastRadiusMissing,
    /// A repair component declares no reversibility classes.
    ReversibilityClassMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no responsive classes.
    ResponsiveClassMissing,
    /// A component declares no window classes.
    WindowClassMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (masked boundary, conflated
    /// live/restored session, private status grammar, or overstated reversibility
    /// / dropped audit truth).
    ComponentInvariantViolated,
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

impl M5RuntimeBoundaryMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::ShellIntegrationQualityMissing => "shell_integration_quality_missing",
            Self::SessionLivenessStateMissing => "session_liveness_state_missing",
            Self::HostBoundaryClassMissing => "host_boundary_class_missing",
            Self::ConnectionStateMissing => "connection_state_missing",
            Self::RuntimeSourceClassMissing => "runtime_source_class_missing",
            Self::ToolchainSourceClassMissing => "toolchain_source_class_missing",
            Self::ToolchainPinStateMissing => "toolchain_pin_state_missing",
            Self::CollaborationRoleMissing => "collaboration_role_missing",
            Self::FollowStateMissing => "follow_state_missing",
            Self::RepairBlastRadiusMissing => "repair_blast_radius_missing",
            Self::ReversibilityClassMissing => "reversibility_class_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::ResponsiveClassMissing => "responsive_class_missing",
            Self::WindowClassMissing => "window_class_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 runtime-boundary matrix export.
pub fn current_stable_m5_runtime_boundary_component_matrix_export(
) -> Result<M5RuntimeBoundaryMatrixPacket, M5RuntimeBoundaryMatrixArtifactError> {
    let packet: M5RuntimeBoundaryMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-runtime-boundary-proof/support_export.json"
    )))
    .map_err(M5RuntimeBoundaryMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5RuntimeBoundaryMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5RuntimeBoundaryMatrixPacket,
    violations: &mut Vec<M5RuntimeBoundaryMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_RUNTIME_BOUNDARY_SCHEMA_REF,
        M5_RUNTIME_BOUNDARY_DOC_REF,
        M5_RUNTIME_BOUNDARY_SHELL_ZONE_REF,
        M5_RUNTIME_BOUNDARY_TERMINAL_CONTRACT_REF,
        M5_RUNTIME_BOUNDARY_REPAIR_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5RuntimeBoundaryMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5RuntimeBoundaryMatrixPacket,
    violations: &mut Vec<M5RuntimeBoundaryMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5RuntimeBoundaryMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5RuntimeBoundaryMatrixPacket,
    violations: &mut Vec<M5RuntimeBoundaryMatrixViolation>,
) {
    let present: BTreeSet<M5RuntimeBoundaryComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5RuntimeBoundaryComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5RuntimeBoundaryMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5RuntimeBoundaryMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5RuntimeBoundaryMatrixViolation::MandatoryLabelMissing);
        }
        if family.is_terminal() && row.shell_integration_qualities.is_empty() {
            violations.push(M5RuntimeBoundaryMatrixViolation::ShellIntegrationQualityMissing);
        }
        if family.is_terminal() && row.session_liveness_states.is_empty() {
            violations.push(M5RuntimeBoundaryMatrixViolation::SessionLivenessStateMissing);
        }
        if family.is_remote_target() && row.host_boundary_classes.is_empty() {
            violations.push(M5RuntimeBoundaryMatrixViolation::HostBoundaryClassMissing);
        }
        if family.is_remote_target() && row.connection_states.is_empty() {
            violations.push(M5RuntimeBoundaryMatrixViolation::ConnectionStateMissing);
        }
        if family.is_environment() && row.runtime_source_classes.is_empty() {
            violations.push(M5RuntimeBoundaryMatrixViolation::RuntimeSourceClassMissing);
        }
        if family.is_toolchain() && row.toolchain_source_classes.is_empty() {
            violations.push(M5RuntimeBoundaryMatrixViolation::ToolchainSourceClassMissing);
        }
        if family.is_toolchain() && row.toolchain_pin_states.is_empty() {
            violations.push(M5RuntimeBoundaryMatrixViolation::ToolchainPinStateMissing);
        }
        if family.is_presence() && row.collaboration_roles.is_empty() {
            violations.push(M5RuntimeBoundaryMatrixViolation::CollaborationRoleMissing);
        }
        if family.is_presence() && row.follow_states.is_empty() {
            violations.push(M5RuntimeBoundaryMatrixViolation::FollowStateMissing);
        }
        if family.is_repair() && row.repair_blast_radii.is_empty() {
            violations.push(M5RuntimeBoundaryMatrixViolation::RepairBlastRadiusMissing);
        }
        if family.is_repair() && row.reversibility_classes.is_empty() {
            violations.push(M5RuntimeBoundaryMatrixViolation::ReversibilityClassMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5RuntimeBoundaryMatrixViolation::SurfaceFamilyMissing);
        }
        if row.responsive_classes.is_empty() {
            violations.push(M5RuntimeBoundaryMatrixViolation::ResponsiveClassMissing);
        }
        if row.window_classes.is_empty() {
            violations.push(M5RuntimeBoundaryMatrixViolation::WindowClassMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5RuntimeBoundaryMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5RuntimeBoundaryMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5RuntimeBoundaryMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5RuntimeBoundaryMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5RuntimeBoundaryMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5RuntimeBoundaryMatrixPacket,
    violations: &mut Vec<M5RuntimeBoundaryMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.terminal_tab_shows_boundary_and_shell_integration,
        review.remote_pill_shows_host_boundary_and_connection,
        review.environment_strip_names_winning_runtime_source,
        review.toolchain_row_explains_why_toolchain_won,
        review.presence_stack_shows_role_and_follow_state,
        review.repair_card_shows_blast_radius_and_reversibility,
        review.live_versus_restored_never_conflated,
        review.no_component_invents_second_status_grammar,
        review.every_component_bound_to_shell_zone,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5RuntimeBoundaryMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5RuntimeBoundaryMatrixPacket,
    violations: &mut Vec<M5RuntimeBoundaryMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.terminal_and_session_surfaces_consume_matrix,
        projection.remote_and_environment_surfaces_consume_boundary_vocabulary,
        projection.collaboration_surfaces_consume_role_follow_vocabulary,
        projection.repair_surfaces_consume_reversibility_vocabulary,
        projection.support_export_reads_single_source,
        projection.accessibility_bridge_reads_single_source,
    ] {
        if !ok {
            violations.push(M5RuntimeBoundaryMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5RuntimeBoundaryMatrixPacket,
    violations: &mut Vec<M5RuntimeBoundaryMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5RuntimeBoundaryMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5RuntimeBoundaryMatrixPacket,
    violations: &mut Vec<M5RuntimeBoundaryMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.runtime_boundary_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5RuntimeBoundaryMatrixViolation::ReleasePostureIncomplete);
    }
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
