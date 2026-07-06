//! One reusable M5 terminal-tab / header-strip primitive: session title, host
//! boundary, shell-integration quality, cwd-or-transcript state, and
//! shared-control truth, projected the same way across every M5 execution shell.
//!
//! Aureline's frozen runtime-boundary component matrix
//! ([`crate::freeze_the_m5_terminal_tab_remote_target_pill_environment_status_strip_toolchain_pin_row_presence_avatar_stack_and_repair_action_card_component_matrix`])
//! names the terminal tab / header strip as one governed component family and
//! freezes its controlled vocabulary — the shell-integration qualities, the
//! session-liveness states, the host-boundary classes, the collaboration roles,
//! and the follow states. This module *implements* that terminal-tab contract as
//! one reusable primitive so a user can orient before typing and can tell whether
//! a session is local, remote, containerized, managed, shared, live, restored, or
//! inspect-only — instead of that truth drifting by feature lane.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_terminal_tab`] — that takes one terminal session's
//!    title, host boundary, shell-integration quality, liveness, connection state,
//!    live-or-last-known cwd, and collaboration role / follow / reauthorization
//!    state, and produces one [`M5ResolvedTerminalTab`] carrying the derived input
//!    posture (write-capable-live versus read-only-restored versus
//!    inspect-only-observer versus reauthorization-blocked versus closed), the
//!    cwd-or-transcript display state, and the shared-control posture. The resolver
//!    never confuses a restored transcript with a live write-capable shell and
//!    never leaves shared-control or reauthorization state to be inferred from
//!    background collaboration metadata.
//! 2. A parity matrix — [`M5TerminalTabPrimitivePacket`] — that binds one row per
//!    claimed M5 terminal-console consumer (the terminal panel, the notebook
//!    console, the request console, the preview dev-server console, and the
//!    incident shell) to the shared tab anatomy, the same input postures, cwd
//!    states, and shared-control postures, the same export fields, and the same
//!    non-visual accessibility routes, so the boundary and integration cues remain
//!    visible on every consumer and the support / export packet reconstructs
//!    boundary and liveness truth from one shared model.
//!
//! The shell-integration quality ([`M5ShellIntegrationQuality`]), session-liveness
//! state ([`M5TerminalSessionLiveness`]), host-boundary class
//! ([`M5HostBoundaryClass`]), remote connection state ([`M5RemoteConnectionState`]),
//! collaboration role ([`M5CollaborationRole`]), follow state ([`M5FollowState`]),
//! non-visual accessibility routes ([`M5RuntimeBoundaryAccessibilityRoute`]),
//! qualification classes ([`M5RuntimeBoundaryQualificationClass`]), and downgrade
//! triggers ([`M5RuntimeBoundaryDowngradeTrigger`]) are reused verbatim from the
//! frozen runtime-boundary matrix; the shell topology — zones, responsive classes,
//! window classes, and consumer surfaces — is reused from the frozen shell-zone
//! matrix. This module mints new vocabulary only for what the frozen matrix left
//! implicit about the terminal tab itself: its terminal-console consumer families,
//! its anatomy parts, its derived input postures, its cwd display states, its
//! shared-control postures, and its export fields. No M5 shell invents a second
//! terminal grammar.
//!
//! Raw URLs, raw local paths, raw usernames, raw hostnames, tokens, credentials,
//! and user text bodies stay outside the support boundary; every session title and
//! cwd is carried only as an opaque, export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ui/m5-terminal-tab.schema.json`](../../../../schemas/ui/m5-terminal-tab.schema.json)
//! and the contract doc is
//! [`docs/components/m5_terminal_tab_primitive_contract.md`](../../../../docs/components/m5_terminal_tab_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-terminal-tab-primitive/`](../../../../fixtures/ui/m5-terminal-tab-primitive/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_terminal_tab_primitive_incident_shell_beta_narrowed,
    seeded_m5_terminal_tab_primitive_packet,
    seeded_m5_terminal_tab_primitive_preview_dev_server_preview_narrowed,
    M5_TERMINAL_TAB_PRIMITIVE_PACKET_ID,
};

// The shell-integration quality, session-liveness state, host-boundary class,
// connection state, collaboration role, follow state, accessibility routes,
// qualification classes, and downgrade triggers are frozen once, in the
// runtime-boundary component matrix. This primitive reuses them verbatim so it
// never invents a parallel terminal vocabulary.
pub use crate::freeze_the_m5_terminal_tab_remote_target_pill_environment_status_strip_toolchain_pin_row_presence_avatar_stack_and_repair_action_card_component_matrix::{
    M5CollaborationRole, M5FollowState, M5HostBoundaryClass, M5RemoteConnectionState,
    M5RuntimeBoundaryAccessibilityRoute, M5RuntimeBoundaryDowngradeTrigger,
    M5RuntimeBoundaryQualificationClass, M5ShellIntegrationQuality, M5TerminalSessionLiveness,
};

// The canonical shell topology — zones, responsive classes, window classes, and
// consumer surfaces — is frozen once, in the shell-zone matrix.
pub use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix::{
    M5ResponsiveClass, M5ShellConsumerSurface, M5ShellZoneSlot, M5WindowClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5TerminalTabPrimitivePacket`].
pub const M5_TERMINAL_TAB_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_terminal_tab_and_header_strip_boundary_liveness_and_shared_control_primitive";

/// Schema version for M5 terminal-tab-primitive records.
pub const M5_TERMINAL_TAB_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the terminal-tab-primitive boundary schema.
pub const M5_TERMINAL_TAB_SCHEMA_REF: &str = "schemas/ui/m5-terminal-tab.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_TERMINAL_TAB_DOC_REF: &str = "docs/components/m5_terminal_tab_primitive_contract.md";

/// Repo-relative path of the frozen shell-zone schema this primitive binds
/// against.
pub const M5_TERMINAL_TAB_SHELL_ZONE_REF: &str = "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the frozen runtime-boundary component matrix this
/// primitive narrows from.
pub const M5_TERMINAL_TAB_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-runtime-boundary-components.schema.json";

/// Repo-relative path of the session-restore metadata contract this primitive
/// projects live-versus-restored truth from.
pub const M5_TERMINAL_TAB_SESSION_RESTORE_REF: &str =
    "schemas/terminal/session_restore_metadata.schema.json";

/// Repo-relative path of the collaboration control-grant contract this primitive
/// projects shared-control truth from.
pub const M5_TERMINAL_TAB_CONTROL_GRANT_REF: &str =
    "schemas/collaboration/control_grant.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_TERMINAL_TAB_FIXTURE_DIR: &str = "fixtures/ui/m5-terminal-tab-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_TERMINAL_TAB_ARTIFACT_REF: &str =
    "artifacts/release/m5-terminal-tab-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_TERMINAL_TAB_CSV_REF: &str = "artifacts/release/m5-terminal-tab-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_TERMINAL_TAB_REPORT_REF: &str = "artifacts/components/m5-terminal-tab-primitive.md";

/// One claimed M5 terminal-console consumer that renders the shared terminal tab.
/// These are the consumers the acceptance criteria name — the terminal panel, the
/// notebook console, the request console, the preview dev-server console, and the
/// incident shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TerminalConsoleSurface {
    /// The primary terminal panel / integrated terminal.
    TerminalPanel,
    /// The notebook cell / kernel console.
    NotebookConsole,
    /// The request / REPL console.
    RequestConsole,
    /// The preview dev-server console.
    PreviewDevServer,
    /// The incident / break-glass shell.
    IncidentShell,
}

impl M5TerminalConsoleSurface {
    /// Every claimed terminal-console consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TerminalPanel,
        Self::NotebookConsole,
        Self::RequestConsole,
        Self::PreviewDevServer,
        Self::IncidentShell,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TerminalPanel => "terminal_panel",
            Self::NotebookConsole => "notebook_console",
            Self::RequestConsole => "request_console",
            Self::PreviewDevServer => "preview_dev_server",
            Self::IncidentShell => "incident_shell",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::TerminalPanel => "Terminal Panel",
            Self::NotebookConsole => "Notebook Console",
            Self::RequestConsole => "Request Console",
            Self::PreviewDevServer => "Preview Dev-Server Console",
            Self::IncidentShell => "Incident Shell",
        }
    }
}

/// One anatomy part the shared terminal tab / header strip surfaces. The first
/// three in [`M5TerminalTabAnatomyPart::MANDATORY`] are required on every tab so a
/// user can orient before typing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TerminalTabAnatomyPart {
    /// The session / terminal title.
    SessionTitle,
    /// The local-or-remote-or-container-or-managed host boundary badge.
    HostBoundaryBadge,
    /// The typed session-liveness state (live versus restored).
    LivenessState,
    /// The shell-integration quality cue.
    ShellIntegrationCue,
    /// The cwd or last-known-cwd / transcript state.
    CwdOrTranscriptState,
    /// The shared-control / presenter-follow cue.
    SharedControlCue,
    /// The reauthorization-required cue.
    ReauthorizationCue,
    /// The reconnect / reattach action.
    ReconnectAction,
}

impl M5TerminalTabAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SessionTitle,
        Self::HostBoundaryBadge,
        Self::LivenessState,
        Self::ShellIntegrationCue,
        Self::CwdOrTranscriptState,
        Self::SharedControlCue,
        Self::ReauthorizationCue,
        Self::ReconnectAction,
    ];

    /// The anatomy parts every terminal tab must render before input.
    pub const MANDATORY: [Self; 3] = [
        Self::SessionTitle,
        Self::HostBoundaryBadge,
        Self::LivenessState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionTitle => "session_title",
            Self::HostBoundaryBadge => "host_boundary_badge",
            Self::LivenessState => "liveness_state",
            Self::ShellIntegrationCue => "shell_integration_cue",
            Self::CwdOrTranscriptState => "cwd_or_transcript_state",
            Self::SharedControlCue => "shared_control_cue",
            Self::ReauthorizationCue => "reauthorization_cue",
            Self::ReconnectAction => "reconnect_action",
        }
    }
}

/// The derived input posture of a terminal tab — whether the user may type into a
/// live write-capable shell, or whether the tab is read-only / inspect-only /
/// closed. This is the resolver's headline verdict: a restored transcript is never
/// write-capable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TerminalInputPosture {
    /// A live PTY the user may type into.
    WriteCapableLive,
    /// A read-only session restored from a transcript.
    ReadOnlyRestored,
    /// A read-only session that dropped and is reconnecting.
    ReadOnlyReconnecting,
    /// A shared session the participant may only observe.
    InspectOnlyObserver,
    /// A live session whose input is blocked pending reauthorization.
    ReauthorizationBlocked,
    /// A closed / exited session that accepts no input.
    ClosedNoInput,
}

impl M5TerminalInputPosture {
    /// Every input posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WriteCapableLive,
        Self::ReadOnlyRestored,
        Self::ReadOnlyReconnecting,
        Self::InspectOnlyObserver,
        Self::ReauthorizationBlocked,
        Self::ClosedNoInput,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WriteCapableLive => "write_capable_live",
            Self::ReadOnlyRestored => "read_only_restored",
            Self::ReadOnlyReconnecting => "read_only_reconnecting",
            Self::InspectOnlyObserver => "inspect_only_observer",
            Self::ReauthorizationBlocked => "reauthorization_blocked",
            Self::ClosedNoInput => "closed_no_input",
        }
    }

    /// Whether this posture permits typed input into a live write-capable shell.
    pub const fn is_write_capable(self) -> bool {
        matches!(self, Self::WriteCapableLive)
    }
}

/// The cwd-or-transcript display state of a terminal tab, so a live cwd, a
/// last-known cwd, an unavailable cwd, and a shell that never reports cwd are never
/// conflated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CwdDisplayState {
    /// The live cwd reported by shell integration.
    LiveCwdReported,
    /// The last-known cwd shown for a restored / reconnecting / closed session.
    LastKnownCwdShown,
    /// The cwd is unavailable even though the shell can report it.
    CwdUnavailable,
    /// The shell integration does not report cwd at all.
    CwdNotReportedByShell,
}

impl M5CwdDisplayState {
    /// Every cwd display state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LiveCwdReported,
        Self::LastKnownCwdShown,
        Self::CwdUnavailable,
        Self::CwdNotReportedByShell,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveCwdReported => "live_cwd_reported",
            Self::LastKnownCwdShown => "last_known_cwd_shown",
            Self::CwdUnavailable => "cwd_unavailable",
            Self::CwdNotReportedByShell => "cwd_not_reported_by_shell",
        }
    }
}

/// The shared-control posture of a terminal tab, surfaced in the tab chrome rather
/// than buried in a collaboration side panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SharedControlPosture {
    /// A solo session with no other participants.
    SoloSession,
    /// A shared session where this participant holds control.
    SharedControlHeld,
    /// A shared session where this participant may only observe.
    SharedObserverOnly,
    /// A shared session where this participant is following the presenter.
    SharedFollowingPresenter,
    /// A shared session whose control is blocked pending reauthorization.
    ReauthorizationRequired,
}

impl M5SharedControlPosture {
    /// Every shared-control posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SoloSession,
        Self::SharedControlHeld,
        Self::SharedObserverOnly,
        Self::SharedFollowingPresenter,
        Self::ReauthorizationRequired,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SoloSession => "solo_session",
            Self::SharedControlHeld => "shared_control_held",
            Self::SharedObserverOnly => "shared_observer_only",
            Self::SharedFollowingPresenter => "shared_following_presenter",
            Self::ReauthorizationRequired => "reauthorization_required",
        }
    }
}

/// A field the support / export packet carries so boundary and liveness truth is
/// reconstructable from the shared tab model. The first four in
/// [`M5TerminalTabExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TerminalTabExportField {
    /// The opaque session-title representation.
    SessionTitle,
    /// The host-boundary class.
    HostBoundary,
    /// The shell-integration quality.
    ShellIntegrationQuality,
    /// The session-liveness state.
    Liveness,
    /// The derived input posture.
    InputPosture,
    /// The cwd display state.
    CwdDisplayState,
    /// The shared-control posture.
    SharedControlPosture,
}

impl M5TerminalTabExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SessionTitle,
        Self::HostBoundary,
        Self::ShellIntegrationQuality,
        Self::Liveness,
        Self::InputPosture,
        Self::CwdDisplayState,
        Self::SharedControlPosture,
    ];

    /// The export fields every terminal-tab export must carry.
    pub const MANDATORY: [Self; 4] = [
        Self::SessionTitle,
        Self::HostBoundary,
        Self::Liveness,
        Self::InputPosture,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionTitle => "session_title",
            Self::HostBoundary => "host_boundary",
            Self::ShellIntegrationQuality => "shell_integration_quality",
            Self::Liveness => "liveness",
            Self::InputPosture => "input_posture",
            Self::CwdDisplayState => "cwd_display_state",
            Self::SharedControlPosture => "shared_control_posture",
        }
    }
}

/// True when this host boundary is the local machine.
const fn host_is_local(host: M5HostBoundaryClass) -> bool {
    matches!(host, M5HostBoundaryClass::LocalHost)
}

/// True when this liveness is a live (attached or detached-running) session.
const fn liveness_is_live(liveness: M5TerminalSessionLiveness) -> bool {
    matches!(
        liveness,
        M5TerminalSessionLiveness::LiveAttached | M5TerminalSessionLiveness::LiveDetachedRunning
    )
}

/// True when this shell integration reports the working directory at all.
const fn integration_reports_cwd(quality: M5ShellIntegrationQuality) -> bool {
    matches!(
        quality,
        M5ShellIntegrationQuality::FullyIntegrated
            | M5ShellIntegrationQuality::CwdReportingOnly
            | M5ShellIntegrationQuality::IntegrationDegraded
    )
}

/// The full input to the terminal-tab resolver for one session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TerminalTabResolutionInput {
    /// The opaque, export-safe session-title representation.
    pub session_title: String,
    /// The host boundary the session runs on.
    pub host_boundary: M5HostBoundaryClass,
    /// The shell-integration quality negotiated for the session.
    pub shell_integration: M5ShellIntegrationQuality,
    /// The session-liveness state.
    pub liveness: M5TerminalSessionLiveness,
    /// The remote connection state. Required for a non-local host, forbidden for a
    /// local host.
    pub connection_state: Option<M5RemoteConnectionState>,
    /// The live cwd representation, when a live session's shell reports it.
    pub cwd_repr: Option<String>,
    /// The last-known cwd representation, shown for a restored / reconnecting /
    /// closed session.
    pub last_known_cwd_repr: Option<String>,
    /// The participant's collaboration role, when the session is shared.
    pub collaboration_role: Option<M5CollaborationRole>,
    /// The participant's follow state, when the session is shared. Requires a role.
    pub follow_state: Option<M5FollowState>,
    /// True when input / control is blocked pending reauthorization. Only a shared
    /// session may require reauthorization.
    pub reauthorization_required: bool,
}

/// The resolved boundary / liveness / shared-control truth for one terminal tab.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedTerminalTab {
    /// The opaque session-title representation.
    pub session_title: String,
    /// The host boundary the session runs on.
    pub host_boundary: M5HostBoundaryClass,
    /// True when the host boundary is the local machine.
    pub boundary_is_local: bool,
    /// The remote connection state, when non-local.
    pub connection_state: Option<M5RemoteConnectionState>,
    /// The shell-integration quality.
    pub shell_integration: M5ShellIntegrationQuality,
    /// The session-liveness state.
    pub liveness: M5TerminalSessionLiveness,
    /// True when this tab is a read-only restored transcript.
    pub is_restored_transcript: bool,
    /// The derived input posture.
    pub input_posture: M5TerminalInputPosture,
    /// True when the tab is a live write-capable shell.
    pub is_write_capable: bool,
    /// The cwd-or-transcript display state.
    pub cwd_display: M5CwdDisplayState,
    /// The cwd representation actually shown, when any.
    pub cwd_repr: Option<String>,
    /// The participant's collaboration role, when shared.
    pub collaboration_role: Option<M5CollaborationRole>,
    /// The participant's follow state, when shared.
    pub follow_state: Option<M5FollowState>,
    /// The shared-control posture.
    pub shared_control_posture: M5SharedControlPosture,
    /// True when control is blocked pending reauthorization.
    pub requires_reauthorization: bool,
}

/// Errors returned by [`resolve_terminal_tab`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5TerminalTabResolutionError {
    /// The session title was empty.
    EmptySessionTitle,
    /// A non-local host carried no connection state.
    RemoteHostMissingConnectionState,
    /// A local host carried a remote connection state.
    LocalHostWithConnectionState,
    /// A follow state was set without a collaboration role.
    FollowStateWithoutRole,
    /// Reauthorization was required on an unshared (solo) session.
    ReauthorizationWithoutSharedSession,
    /// A session title or cwd representation carried forbidden material.
    ForbiddenSessionMaterial,
}

impl M5TerminalTabResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptySessionTitle => "empty_session_title",
            Self::RemoteHostMissingConnectionState => "remote_host_missing_connection_state",
            Self::LocalHostWithConnectionState => "local_host_with_connection_state",
            Self::FollowStateWithoutRole => "follow_state_without_role",
            Self::ReauthorizationWithoutSharedSession => "reauthorization_without_shared_session",
            Self::ForbiddenSessionMaterial => "forbidden_session_material",
        }
    }
}

impl fmt::Display for M5TerminalTabResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "terminal-tab resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5TerminalTabResolutionError {}

/// Resolves one terminal tab from its session state.
///
/// The derived input posture is the headline verdict: a closed session accepts no
/// input, a restored transcript is read-only, a reconnecting session is read-only,
/// an observer is inspect-only, a live session pending reauthorization is blocked,
/// and only a live, non-observer, authorized session is write-capable. A restored
/// transcript is therefore never confused with a live write-capable shell, and the
/// shared-control posture is derived here rather than inferred from background
/// collaboration metadata.
pub fn resolve_terminal_tab(
    input: &M5TerminalTabResolutionInput,
) -> Result<M5ResolvedTerminalTab, M5TerminalTabResolutionError> {
    if input.session_title.trim().is_empty() {
        return Err(M5TerminalTabResolutionError::EmptySessionTitle);
    }
    if value_repr_is_forbidden(&input.session_title) {
        return Err(M5TerminalTabResolutionError::ForbiddenSessionMaterial);
    }
    for cwd in [&input.cwd_repr, &input.last_known_cwd_repr]
        .into_iter()
        .flatten()
    {
        if value_repr_is_forbidden(cwd) {
            return Err(M5TerminalTabResolutionError::ForbiddenSessionMaterial);
        }
    }

    let boundary_is_local = host_is_local(input.host_boundary);
    match (boundary_is_local, input.connection_state.is_some()) {
        (true, true) => return Err(M5TerminalTabResolutionError::LocalHostWithConnectionState),
        (false, false) => {
            return Err(M5TerminalTabResolutionError::RemoteHostMissingConnectionState)
        }
        _ => {}
    }

    if input.follow_state.is_some() && input.collaboration_role.is_none() {
        return Err(M5TerminalTabResolutionError::FollowStateWithoutRole);
    }
    if input.reauthorization_required && input.collaboration_role.is_none() {
        return Err(M5TerminalTabResolutionError::ReauthorizationWithoutSharedSession);
    }

    let is_restored_transcript = matches!(
        input.liveness,
        M5TerminalSessionLiveness::RestoredFromTranscript
    );

    let input_posture = if matches!(input.liveness, M5TerminalSessionLiveness::ClosedExited) {
        M5TerminalInputPosture::ClosedNoInput
    } else if is_restored_transcript {
        M5TerminalInputPosture::ReadOnlyRestored
    } else if matches!(input.liveness, M5TerminalSessionLiveness::Reconnecting) {
        M5TerminalInputPosture::ReadOnlyReconnecting
    } else if matches!(
        input.collaboration_role,
        Some(M5CollaborationRole::Observer)
    ) {
        M5TerminalInputPosture::InspectOnlyObserver
    } else if input.reauthorization_required {
        M5TerminalInputPosture::ReauthorizationBlocked
    } else {
        M5TerminalInputPosture::WriteCapableLive
    };
    let is_write_capable = input_posture.is_write_capable();

    let (cwd_display, cwd_repr) = if !integration_reports_cwd(input.shell_integration) {
        (M5CwdDisplayState::CwdNotReportedByShell, None)
    } else if liveness_is_live(input.liveness) && input.cwd_repr.is_some() {
        (M5CwdDisplayState::LiveCwdReported, input.cwd_repr.clone())
    } else if input.last_known_cwd_repr.is_some() {
        (
            M5CwdDisplayState::LastKnownCwdShown,
            input.last_known_cwd_repr.clone(),
        )
    } else {
        (M5CwdDisplayState::CwdUnavailable, None)
    };

    let requires_reauthorization = input.reauthorization_required;
    let shared_control_posture = match input.collaboration_role {
        None => M5SharedControlPosture::SoloSession,
        Some(role) => {
            if requires_reauthorization {
                M5SharedControlPosture::ReauthorizationRequired
            } else if matches!(role, M5CollaborationRole::Observer) {
                M5SharedControlPosture::SharedObserverOnly
            } else if matches!(
                role,
                M5CollaborationRole::ControlHolder
                    | M5CollaborationRole::SessionHost
                    | M5CollaborationRole::Presenter
            ) {
                M5SharedControlPosture::SharedControlHeld
            } else if matches!(input.follow_state, Some(M5FollowState::FollowingPresenter)) {
                M5SharedControlPosture::SharedFollowingPresenter
            } else {
                M5SharedControlPosture::SharedControlHeld
            }
        }
    };

    Ok(M5ResolvedTerminalTab {
        session_title: input.session_title.clone(),
        host_boundary: input.host_boundary,
        boundary_is_local,
        connection_state: input.connection_state,
        shell_integration: input.shell_integration,
        liveness: input.liveness,
        is_restored_transcript,
        input_posture,
        is_write_capable,
        cwd_display,
        cwd_repr,
        collaboration_role: input.collaboration_role,
        follow_state: input.follow_state,
        shared_control_posture,
        requires_reauthorization,
    })
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs boundary and liveness truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TerminalTabResolutionCase {
    /// The resolver input.
    pub input: M5TerminalTabResolutionInput,
    /// The resolved truth. Must equal `resolve_terminal_tab(&input)`.
    pub resolved: M5ResolvedTerminalTab,
}

impl M5TerminalTabResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5TerminalTabResolutionInput) -> Self {
        let resolved = resolve_terminal_tab(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_terminal_tab(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one terminal-console consumer bound to the
/// shared tab anatomy, input postures, cwd states, shared-control postures, export
/// fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TerminalConsoleRow {
    /// Terminal-console consumer family.
    pub console_surface: M5TerminalConsoleSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5RuntimeBoundaryQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Canonical shell zone this tab attaches to.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Responsive classes this tab must survive.
    pub responsive_classes: Vec<M5ResponsiveClass>,
    /// Window classes this tab keeps continuity across.
    pub window_classes: Vec<M5WindowClass>,
    /// Anatomy parts this tab renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5TerminalTabAnatomyPart>,
    /// Input postures this tab distinguishes.
    pub input_postures: Vec<M5TerminalInputPosture>,
    /// Cwd display states this tab distinguishes.
    pub cwd_display_states: Vec<M5CwdDisplayState>,
    /// Shared-control postures this tab surfaces in chrome.
    pub shared_control_postures: Vec<M5SharedControlPosture>,
    /// Export fields this tab carries (must include the mandatory fields).
    pub export_fields: Vec<M5TerminalTabExportField>,
    /// Non-visual accessibility routes this tab offers.
    pub accessibility_routes: Vec<M5RuntimeBoundaryAccessibilityRoute>,
    /// Shell subsystems that consume this tab's projection.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this tab.
    pub downgrade_triggers: Vec<M5RuntimeBoundaryDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this consumer.
    pub example_resolutions: Vec<M5TerminalTabResolutionCase>,
    /// Hard invariant: this tab never masks the host / runtime boundary. MUST be
    /// `false`.
    pub masks_host_or_runtime_boundary: bool,
    /// Hard invariant: this tab never conflates a live session with a restored one.
    /// MUST be `false`.
    pub conflates_live_and_restored_session: bool,
    /// Hard invariant: this tab never invents a private terminal grammar. MUST be
    /// `false`.
    pub invents_private_terminal_grammar: bool,
    /// Hard invariant: this tab never infers shared-control state from background
    /// metadata. MUST be `false`.
    pub infers_shared_control_from_background_metadata: bool,
}

impl M5TerminalConsoleRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5TerminalTabAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5TerminalTabAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5TerminalTabExportField> =
            self.export_fields.iter().copied().collect();
        M5TerminalTabExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_host_or_runtime_boundary
            && !self.conflates_live_and_restored_session
            && !self.invents_private_terminal_grammar
            && !self.infers_shared_control_from_background_metadata
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TerminalTabVocabularySet {
    /// Terminal-console consumer tokens.
    pub console_surfaces: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Input-posture tokens.
    pub input_postures: Vec<String>,
    /// Cwd-display-state tokens.
    pub cwd_display_states: Vec<String>,
    /// Shared-control-posture tokens.
    pub shared_control_postures: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Shell-integration-quality tokens (reused from the frozen matrix).
    pub shell_integration_qualities: Vec<String>,
    /// Session-liveness-state tokens (reused from the frozen matrix).
    pub session_liveness_states: Vec<String>,
    /// Host-boundary-class tokens (reused from the frozen matrix).
    pub host_boundary_classes: Vec<String>,
    /// Connection-state tokens (reused from the frozen matrix).
    pub connection_states: Vec<String>,
    /// Collaboration-role tokens (reused from the frozen matrix).
    pub collaboration_roles: Vec<String>,
    /// Follow-state tokens (reused from the frozen matrix).
    pub follow_states: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5TerminalTabVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            console_surfaces: tokens(&M5TerminalConsoleSurface::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5TerminalTabAnatomyPart::ALL, |v| v.as_str()),
            input_postures: tokens(&M5TerminalInputPosture::ALL, |v| v.as_str()),
            cwd_display_states: tokens(&M5CwdDisplayState::ALL, |v| v.as_str()),
            shared_control_postures: tokens(&M5SharedControlPosture::ALL, |v| v.as_str()),
            export_fields: tokens(&M5TerminalTabExportField::ALL, |v| v.as_str()),
            shell_integration_qualities: tokens(&M5ShellIntegrationQuality::ALL, |v| v.as_str()),
            session_liveness_states: tokens(&M5TerminalSessionLiveness::ALL, |v| v.as_str()),
            host_boundary_classes: tokens(&M5HostBoundaryClass::ALL, |v| v.as_str()),
            connection_states: tokens(&M5RemoteConnectionState::ALL, |v| v.as_str()),
            collaboration_roles: tokens(&M5CollaborationRole::ALL, |v| v.as_str()),
            follow_states: tokens(&M5FollowState::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5RuntimeBoundaryAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5TerminalTabGovernanceReview {
    /// One terminal-tab primitive carries boundary and liveness truth on every
    /// consumer.
    pub one_primitive_carries_boundary_and_liveness: bool,
    /// The session title and host boundary are shown before input.
    pub session_title_and_host_boundary_always_shown: bool,
    /// Live and restored sessions are never conflated.
    pub live_versus_restored_never_conflated: bool,
    /// A restored transcript is never presented as write-capable.
    pub restored_transcript_never_write_capable: bool,
    /// The cwd or last-known cwd (or transcript state) is always disclosed.
    pub cwd_or_last_known_cwd_always_disclosed: bool,
    /// Shared-control and reauthorization state is always explicit in chrome.
    pub shared_control_and_reauthorization_always_explicit: bool,
    /// The support / export packet reconstructs boundary and liveness truth.
    pub support_export_reconstructs_boundary_truth: bool,
    /// No consumer invents a second terminal grammar.
    pub no_surface_invents_second_terminal_grammar: bool,
    /// Every row is bound to a canonical shell zone.
    pub every_row_bound_to_shell_zone: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel terminal-tab vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TerminalTabConsumerProjection {
    /// Terminal, notebook, request, preview, and incident consoles all consume the
    /// shared primitive.
    pub terminal_console_surfaces_consume_shared_primitive: bool,
    /// The liveness resolver reads a single canonical source.
    pub liveness_resolver_reads_single_source: bool,
    /// The shared-control cue reads a single canonical collaboration source.
    pub shared_control_reads_single_collaboration_source: bool,
    /// The host-boundary badge reads a single canonical source.
    pub boundary_badge_reads_single_source: bool,
    /// Support / export reads a single canonical terminal-tab source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TerminalTabProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the terminal-tab primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TerminalTabReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting terminal-tab audit.
    pub terminal_tab_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5TerminalTabPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TerminalTabPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Console rows.
    pub console_rows: Vec<M5TerminalConsoleRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TerminalTabVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TerminalTabGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TerminalTabConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TerminalTabProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TerminalTabReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 terminal-tab-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TerminalTabPrimitivePacket {
    /// Record kind; must equal [`M5_TERMINAL_TAB_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_TERMINAL_TAB_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Console rows.
    pub console_rows: Vec<M5TerminalConsoleRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TerminalTabVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TerminalTabGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TerminalTabConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TerminalTabProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TerminalTabReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5TerminalTabPrimitivePacket {
    /// Builds an M5 terminal-tab-primitive packet from stable-lane input.
    pub fn new(input: M5TerminalTabPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_TERMINAL_TAB_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_TERMINAL_TAB_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            console_rows: input.console_rows,
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

    /// Validates the M5 terminal-tab-primitive invariants.
    pub fn validate(&self) -> Vec<M5TerminalTabPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_TERMINAL_TAB_PRIMITIVE_RECORD_KIND {
            violations.push(M5TerminalTabPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_TERMINAL_TAB_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5TerminalTabPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5TerminalTabPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_console_rows(self, &mut violations);
        validate_restored_write_confusion_covered(self, &mut violations);
        validate_shared_control_disclosure_covered(self, &mut violations);
        validate_reauthorization_disclosure_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 terminal-tab primitive packet serializes"),
        ) {
            violations.push(M5TerminalTabPrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 terminal-tab primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per console consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "console_surface,qualification,owner,shell_zone_slot,anatomy_parts,input_postures,cwd_display_states,shared_control_postures,export_fields,example_count\n",
        );
        for row in &self.console_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.console_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shell_zone_slot.as_str(),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.input_postures, |v| v.as_str()),
                join_tokens(&row.cwd_display_states, |v| v.as_str()),
                join_tokens(&row.shared_control_postures, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_resolutions.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .console_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Terminal-Tab and Header-Strip Primitive: Boundary, Liveness, and Shared Control\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Terminal-console consumers: {} ({} stable)\n",
            self.console_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Input postures: {}\n",
            self.vocabulary_set.input_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Cwd display states: {}\n",
            self.vocabulary_set.cwd_display_states.join(", ")
        ));
        out.push_str(&format!(
            "- Shared-control postures: {}\n",
            self.vocabulary_set.shared_control_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Host-boundary classes: {}\n",
            self.vocabulary_set.host_boundary_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Terminal-console consumers\n\n");
        for row in &self.console_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.console_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Shell zone: `{}`\n",
                row.shell_zone_slot.as_str()
            ));
            out.push_str(&format!(
                "  - Worked resolutions: {}\n",
                row.example_resolutions.len()
            ));
            for case in &row.example_resolutions {
                out.push_str(&format!(
                    "    - `{}` on `{}` → `{}` (cwd `{}`, `{}`)\n",
                    case.resolved.session_title,
                    case.resolved.host_boundary.as_str(),
                    case.resolved.input_posture.as_str(),
                    case.resolved.cwd_display.as_str(),
                    case.resolved.shared_control_posture.as_str()
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 terminal-tab-primitive export.
#[derive(Debug)]
pub enum M5TerminalTabPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5TerminalTabPrimitiveViolation>),
}

impl fmt::Display for M5TerminalTabPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 terminal-tab primitive export parse failed: {error}"
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
                    "m5 terminal-tab primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5TerminalTabPrimitiveArtifactError {}

/// Validation failures emitted by [`M5TerminalTabPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5TerminalTabPrimitiveViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required terminal-console consumer family is missing from the matrix.
    RequiredConsoleMissing,
    /// A console row is incomplete.
    ConsoleRowIncomplete,
    /// A console row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A console row declares no input postures.
    InputPostureMissing,
    /// A console row declares no cwd display states.
    CwdDisplayStateMissing,
    /// A console row declares no shared-control postures.
    SharedControlPostureMissing,
    /// A console row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A console row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A console row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A console row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A console row declares no worked resolution cases.
    ExampleResolutionMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A console claiming Stable is missing required proof packet refs.
    StableConsoleMissingProof,
    /// No worked resolution proves a restored transcript resolving read-only and
    /// non-write-capable.
    RestoredWriteConfusionUnproven,
    /// No worked resolution proves a shared session disclosing its shared-control
    /// posture.
    SharedControlDisclosureUnproven,
    /// No worked resolution proves a reauthorization-required session disclosing it.
    ReauthorizationDisclosureUnproven,
    /// A console row violates a hard invariant.
    ConsoleInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5TerminalTabPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsoleMissing => "required_console_missing",
            Self::ConsoleRowIncomplete => "console_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::InputPostureMissing => "input_posture_missing",
            Self::CwdDisplayStateMissing => "cwd_display_state_missing",
            Self::SharedControlPostureMissing => "shared_control_posture_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleResolutionMissing => "example_resolution_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsoleMissingProof => "stable_console_missing_proof",
            Self::RestoredWriteConfusionUnproven => "restored_write_confusion_unproven",
            Self::SharedControlDisclosureUnproven => "shared_control_disclosure_unproven",
            Self::ReauthorizationDisclosureUnproven => "reauthorization_disclosure_unproven",
            Self::ConsoleInvariantViolated => "console_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 terminal-tab-primitive export.
pub fn current_stable_m5_terminal_tab_primitive_export(
) -> Result<M5TerminalTabPrimitivePacket, M5TerminalTabPrimitiveArtifactError> {
    let packet: M5TerminalTabPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-terminal-tab-proof/support_export.json"
    )))
    .map_err(M5TerminalTabPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5TerminalTabPrimitiveArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5TerminalTabPrimitivePacket,
    violations: &mut Vec<M5TerminalTabPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_TERMINAL_TAB_SCHEMA_REF,
        M5_TERMINAL_TAB_DOC_REF,
        M5_TERMINAL_TAB_SHELL_ZONE_REF,
        M5_TERMINAL_TAB_COMPONENT_MATRIX_REF,
        M5_TERMINAL_TAB_SESSION_RESTORE_REF,
        M5_TERMINAL_TAB_CONTROL_GRANT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5TerminalTabPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5TerminalTabPrimitivePacket,
    violations: &mut Vec<M5TerminalTabPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5TerminalTabPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_console_rows(
    packet: &M5TerminalTabPrimitivePacket,
    violations: &mut Vec<M5TerminalTabPrimitiveViolation>,
) {
    let present: BTreeSet<M5TerminalConsoleSurface> = packet
        .console_rows
        .iter()
        .map(|row| row.console_surface)
        .collect();
    for required in M5TerminalConsoleSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5TerminalTabPrimitiveViolation::RequiredConsoleMissing);
            return;
        }
    }

    for row in &packet.console_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
        {
            violations.push(M5TerminalTabPrimitiveViolation::ConsoleRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5TerminalTabPrimitiveViolation::MandatoryAnatomyMissing);
        }
        if row.input_postures.is_empty() {
            violations.push(M5TerminalTabPrimitiveViolation::InputPostureMissing);
        }
        if row.cwd_display_states.is_empty() {
            violations.push(M5TerminalTabPrimitiveViolation::CwdDisplayStateMissing);
        }
        if row.shared_control_postures.is_empty() {
            violations.push(M5TerminalTabPrimitiveViolation::SharedControlPostureMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5TerminalTabPrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5RuntimeBoundaryAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5TerminalTabPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5TerminalTabPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5TerminalTabPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_resolutions.is_empty() {
            violations.push(M5TerminalTabPrimitiveViolation::ExampleResolutionMissing);
        }
        if row
            .example_resolutions
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5TerminalTabPrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5TerminalTabPrimitiveViolation::StableConsoleMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5TerminalTabPrimitiveViolation::ConsoleInvariantViolated);
        }
    }
}

/// At least one worked resolution across the matrix must prove a restored
/// transcript resolving to a read-only, non-write-capable posture — the
/// acceptance-criterion example that a live PTY is distinguishable from a restored
/// transcript before input.
fn validate_restored_write_confusion_covered(
    packet: &M5TerminalTabPrimitivePacket,
    violations: &mut Vec<M5TerminalTabPrimitiveViolation>,
) {
    let proven = packet.console_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.is_restored_transcript
                && case.resolved.input_posture == M5TerminalInputPosture::ReadOnlyRestored
                && !case.resolved.is_write_capable
        })
    });
    if !proven {
        violations.push(M5TerminalTabPrimitiveViolation::RestoredWriteConfusionUnproven);
    }
}

/// At least one worked resolution across the matrix must prove a shared session
/// disclosing a non-solo shared-control posture — the acceptance-criterion example
/// that shared-control state is explicit rather than inferred.
fn validate_shared_control_disclosure_covered(
    packet: &M5TerminalTabPrimitivePacket,
    violations: &mut Vec<M5TerminalTabPrimitiveViolation>,
) {
    let proven = packet.console_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.collaboration_role.is_some()
                && case.resolved.shared_control_posture != M5SharedControlPosture::SoloSession
        })
    });
    if !proven {
        violations.push(M5TerminalTabPrimitiveViolation::SharedControlDisclosureUnproven);
    }
}

/// At least one worked resolution across the matrix must prove a
/// reauthorization-required session disclosing it — the acceptance-criterion
/// example that reauthorization state stays explicit.
fn validate_reauthorization_disclosure_covered(
    packet: &M5TerminalTabPrimitivePacket,
    violations: &mut Vec<M5TerminalTabPrimitiveViolation>,
) {
    let proven = packet.console_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.requires_reauthorization
                && case.resolved.shared_control_posture
                    == M5SharedControlPosture::ReauthorizationRequired
                && case.resolved.input_posture == M5TerminalInputPosture::ReauthorizationBlocked
        })
    });
    if !proven {
        violations.push(M5TerminalTabPrimitiveViolation::ReauthorizationDisclosureUnproven);
    }
}

fn validate_governance_review(
    packet: &M5TerminalTabPrimitivePacket,
    violations: &mut Vec<M5TerminalTabPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_boundary_and_liveness,
        review.session_title_and_host_boundary_always_shown,
        review.live_versus_restored_never_conflated,
        review.restored_transcript_never_write_capable,
        review.cwd_or_last_known_cwd_always_disclosed,
        review.shared_control_and_reauthorization_always_explicit,
        review.support_export_reconstructs_boundary_truth,
        review.no_surface_invents_second_terminal_grammar,
        review.every_row_bound_to_shell_zone,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5TerminalTabPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5TerminalTabPrimitivePacket,
    violations: &mut Vec<M5TerminalTabPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.terminal_console_surfaces_consume_shared_primitive,
        projection.liveness_resolver_reads_single_source,
        projection.shared_control_reads_single_collaboration_source,
        projection.boundary_badge_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5TerminalTabPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5TerminalTabPrimitivePacket,
    violations: &mut Vec<M5TerminalTabPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5TerminalTabPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5TerminalTabPrimitivePacket,
    violations: &mut Vec<M5TerminalTabPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.terminal_tab_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5TerminalTabPrimitiveViolation::ReleasePostureIncomplete);
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

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
