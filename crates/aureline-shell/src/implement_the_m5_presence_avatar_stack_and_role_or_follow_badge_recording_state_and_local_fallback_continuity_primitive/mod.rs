//! One reusable M5 presence-avatar-stack / role-or-follow-badge primitive: the
//! participant identity, the collaboration role, the follow / presenter state, the
//! recording-or-retention cue, and the local-fallback continuity posture, projected
//! the same way across every claimed M5 collaboration surface.
//!
//! Aureline's frozen runtime-boundary component matrix
//! ([`crate::freeze_the_m5_terminal_tab_remote_target_pill_environment_status_strip_toolchain_pin_row_presence_avatar_stack_and_repair_action_card_component_matrix`])
//! names the presence avatar stack as one governed component family and freezes its
//! controlled vocabulary — the collaboration roles and the follow states. This module
//! *implements* that contract, plus the reusable role-or-follow badge it needs, as one
//! primitive so a user can always tell who is present, who is presenting, and whether
//! the current view is being followed, and so collaboration state stays legible when a
//! session degrades rather than disappearing into a generic banner.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_presence_stack`] — that takes one session's participants
//!    (each with its opaque identity, collaboration role, follow state, and presence
//!    liveness), the collaboration link state, and the recording-or-retention cue, and
//!    produces one [`M5ResolvedPresenceStack`] carrying the ordered participant stack
//!    (most-salient first), the derived presenter and control-holder identity, whether
//!    the current view is being followed, the derived continuity posture (live versus
//!    degraded versus reconnecting versus local-fallback versus ended-last-known), the
//!    available follow / control / reconnect actions, and the recording-or-retention
//!    cue. The resolver never masks a collaboration role, never leaves the follow state
//!    ambiguous, never drops presence when the link degrades, and never relies on avatar
//!    imagery alone — a textual participant list is always part of the projection.
//! 2. A parity matrix — [`M5PresenceAvatarStackPrimitivePacket`] — that binds one row
//!    per claimed M5 collaboration surface (the collaboration strip, the shared terminal
//!    header, the shared debug pane, the review / session header, the presenter HUD, the
//!    follow-mode banner, the session roster panel, the activity-center presence entry,
//!    and the shared preview header) to the shared avatar-stack anatomy, role-or-follow
//!    badge anatomy, the same roles, follow states, liveness states, link states,
//!    continuity postures, recording cues, actions, export fields, and non-visual
//!    accessibility routes, so who-is-present / who-presents / who-follows / who-had-
//!    control stays identical on every surface and the support / export packet
//!    reconstructs presence from one shared model.
//!
//! The collaboration role ([`M5CollaborationRole`]), follow state ([`M5FollowState`]),
//! non-visual accessibility routes ([`M5RuntimeBoundaryAccessibilityRoute`]),
//! qualification classes ([`M5RuntimeBoundaryQualificationClass`]), and downgrade
//! triggers ([`M5RuntimeBoundaryDowngradeTrigger`]) are reused verbatim from the frozen
//! runtime-boundary matrix; the shell topology — zones, responsive classes, window
//! classes, and consumer surfaces — is reused from the frozen shell-zone matrix. This
//! module mints new vocabulary only for what the frozen matrix left implicit about the
//! avatar stack and the role-or-follow badge themselves: their collaboration surfaces,
//! their anatomy parts, their participant-liveness states, their collaboration link
//! states, their continuity postures, their recording-or-retention cues, their actions,
//! and their export fields. No M5 surface invents a second presence grammar.
//!
//! Raw display names, avatar bytes, email addresses, tokens, and user text bodies stay
//! outside the support boundary; every session title and participant identity is carried
//! only as an opaque, export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ui/m5-presence-avatar-stack.schema.json`](../../../../schemas/ui/m5-presence-avatar-stack.schema.json)
//! and the contract doc is
//! [`docs/components/m5_presence_avatar_stack_primitive_contract.md`](../../../../docs/components/m5_presence_avatar_stack_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-presence-avatar-stack-primitive/`](../../../../fixtures/ui/m5-presence-avatar-stack-primitive/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_presence_avatar_stack_primitive_packet,
    seeded_m5_presence_avatar_stack_primitive_review_session_header_preview_narrowed,
    seeded_m5_presence_avatar_stack_primitive_shared_debug_pane_beta_narrowed,
    M5_PRESENCE_AVATAR_STACK_PRIMITIVE_PACKET_ID,
};

// The collaboration role, follow state, accessibility routes, qualification classes,
// and downgrade triggers are frozen once, in the runtime-boundary component matrix.
// This primitive reuses them verbatim so it never invents a parallel presence
// vocabulary.
pub use crate::freeze_the_m5_terminal_tab_remote_target_pill_environment_status_strip_toolchain_pin_row_presence_avatar_stack_and_repair_action_card_component_matrix::{
    M5CollaborationRole, M5FollowState, M5RuntimeBoundaryAccessibilityRoute,
    M5RuntimeBoundaryDowngradeTrigger, M5RuntimeBoundaryQualificationClass,
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

/// Stable record-kind tag carried by [`M5PresenceAvatarStackPrimitivePacket`].
pub const M5_PRESENCE_AVATAR_STACK_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_presence_avatar_stack_and_role_or_follow_badge_recording_state_and_local_fallback_continuity_primitive";

/// Schema version for M5 presence-avatar-stack primitive records.
pub const M5_PRESENCE_AVATAR_STACK_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the presence-avatar-stack boundary schema (the packet schema).
pub const M5_PRESENCE_AVATAR_STACK_SCHEMA_REF: &str =
    "schemas/ui/m5-presence-avatar-stack.schema.json";

/// Repo-relative path of the companion role-or-follow-badge component schema.
pub const M5_ROLE_FOLLOW_BADGE_SCHEMA_REF: &str = "schemas/ui/m5-role-follow-badge.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_PRESENCE_AVATAR_STACK_DOC_REF: &str =
    "docs/components/m5_presence_avatar_stack_primitive_contract.md";

/// Repo-relative path of the frozen shell-zone schema this primitive binds against.
pub const M5_PRESENCE_AVATAR_STACK_SHELL_ZONE_REF: &str = "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the frozen runtime-boundary component matrix this primitive
/// narrows from.
pub const M5_PRESENCE_AVATAR_STACK_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-runtime-boundary-components.schema.json";

/// Repo-relative path of the follow-and-presenter-state contract this primitive projects
/// role / follow / presenter truth from.
pub const M5_PRESENCE_AVATAR_STACK_FOLLOW_STATE_REF: &str =
    "schemas/collaboration/follow_and_presenter_state.schema.json";

/// Repo-relative path of the session-state contract this primitive projects link /
/// continuity truth from.
pub const M5_PRESENCE_AVATAR_STACK_SESSION_STATE_REF: &str =
    "schemas/collaboration/session_state.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_PRESENCE_AVATAR_STACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-presence-avatar-stack-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PRESENCE_AVATAR_STACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-presence-avatar-stack-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_PRESENCE_AVATAR_STACK_CSV_REF: &str =
    "artifacts/release/m5-presence-avatar-stack-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_PRESENCE_AVATAR_STACK_REPORT_REF: &str =
    "artifacts/components/m5-presence-avatar-stack-primitive.md";

/// One claimed M5 collaboration surface that renders the shared presence avatar stack
/// and role-or-follow badge. These are the surfaces where a user reads who is present,
/// who is presenting, and whether the view is being followed — the collaboration strip,
/// the shared terminal header, the shared debug pane, the review / session header, the
/// presenter HUD, the follow-mode banner, the session roster panel, the activity-center
/// presence entry, and the shared preview header.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PresenceConsumerSurface {
    /// The always-on collaboration strip.
    CollaborationStrip,
    /// The shared terminal header.
    SharedTerminalHeader,
    /// The shared debug pane.
    SharedDebugPane,
    /// The review / session header.
    ReviewSessionHeader,
    /// The presenter heads-up display.
    PresenterHud,
    /// The follow-mode banner.
    FollowModeBanner,
    /// The session roster panel.
    SessionRosterPanel,
    /// The activity-center presence entry.
    ActivityCenterPresence,
    /// The shared preview header.
    SharedPreviewHeader,
}

impl M5PresenceConsumerSurface {
    /// Every claimed collaboration surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::CollaborationStrip,
        Self::SharedTerminalHeader,
        Self::SharedDebugPane,
        Self::ReviewSessionHeader,
        Self::PresenterHud,
        Self::FollowModeBanner,
        Self::SessionRosterPanel,
        Self::ActivityCenterPresence,
        Self::SharedPreviewHeader,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CollaborationStrip => "collaboration_strip",
            Self::SharedTerminalHeader => "shared_terminal_header",
            Self::SharedDebugPane => "shared_debug_pane",
            Self::ReviewSessionHeader => "review_session_header",
            Self::PresenterHud => "presenter_hud",
            Self::FollowModeBanner => "follow_mode_banner",
            Self::SessionRosterPanel => "session_roster_panel",
            Self::ActivityCenterPresence => "activity_center_presence",
            Self::SharedPreviewHeader => "shared_preview_header",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CollaborationStrip => "Collaboration Strip",
            Self::SharedTerminalHeader => "Shared Terminal Header",
            Self::SharedDebugPane => "Shared Debug Pane",
            Self::ReviewSessionHeader => "Review / Session Header",
            Self::PresenterHud => "Presenter HUD",
            Self::FollowModeBanner => "Follow-Mode Banner",
            Self::SessionRosterPanel => "Session Roster Panel",
            Self::ActivityCenterPresence => "Activity-Center Presence",
            Self::SharedPreviewHeader => "Shared Preview Header",
        }
    }
}

/// The liveness of one participant within a shared session, so a stack never conflates
/// an active collaborator with one who has left or whose state is only last-known-local.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PresenceParticipantLiveness {
    /// Present and active.
    Active,
    /// Present but idle.
    Idle,
    /// Present but reconnecting.
    Reconnecting,
    /// Has left the session.
    Departed,
    /// Only last-known-local truth is available (collaboration link lost).
    LastKnownLocal,
}

impl M5PresenceParticipantLiveness {
    /// Every participant-liveness state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Active,
        Self::Idle,
        Self::Reconnecting,
        Self::Departed,
        Self::LastKnownLocal,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Idle => "idle",
            Self::Reconnecting => "reconnecting",
            Self::Departed => "departed",
            Self::LastKnownLocal => "last_known_local",
        }
    }

    /// True when this participant currently counts as present (active, idle, or
    /// reconnecting), i.e. not departed and not merely last-known-local.
    pub const fn is_present(self) -> bool {
        matches!(self, Self::Active | Self::Idle | Self::Reconnecting)
    }
}

/// The state of the collaboration link backing a session, so the primitive derives one
/// continuity posture and never lets collaboration loss erase who was present.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationLinkState {
    /// The collaboration link is live.
    Live,
    /// The link is degraded but still connected.
    Degraded,
    /// The link is reconnecting.
    Reconnecting,
    /// The link is offline; only a local fallback is available.
    OfflineLocalFallback,
    /// The shared session has ended.
    SessionEnded,
}

impl M5CollaborationLinkState {
    /// Every collaboration link state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Live,
        Self::Degraded,
        Self::Reconnecting,
        Self::OfflineLocalFallback,
        Self::SessionEnded,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Degraded => "degraded",
            Self::Reconnecting => "reconnecting",
            Self::OfflineLocalFallback => "offline_local_fallback",
            Self::SessionEnded => "session_ended",
        }
    }

    /// The continuity posture this link state resolves to.
    pub const fn continuity_posture(self) -> M5PresenceContinuityPosture {
        match self {
            Self::Live => M5PresenceContinuityPosture::Live,
            Self::Degraded => M5PresenceContinuityPosture::DegradedVisible,
            Self::Reconnecting => M5PresenceContinuityPosture::ReconnectingVisible,
            Self::OfflineLocalFallback => M5PresenceContinuityPosture::LocalFallbackVisible,
            Self::SessionEnded => M5PresenceContinuityPosture::EndedLastKnownVisible,
        }
    }

    /// True when the link is not live.
    pub const fn is_degraded(self) -> bool {
        !matches!(self, Self::Live)
    }

    /// True when a reconnect action still applies (degraded, reconnecting, or offline).
    pub const fn is_reconnectable(self) -> bool {
        matches!(
            self,
            Self::Degraded | Self::Reconnecting | Self::OfflineLocalFallback
        )
    }
}

/// The derived continuity posture of a presence stack, so a degraded or lost link keeps
/// collaboration state visible instead of collapsing into a generic session banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PresenceContinuityPosture {
    /// The link is live and presence is fully current.
    Live,
    /// The link is degraded; presence stays visible with a degraded cue.
    DegradedVisible,
    /// The link is reconnecting; presence stays visible with a reconnecting cue.
    ReconnectingVisible,
    /// The link is offline; presence stays visible as last-known via local fallback.
    LocalFallbackVisible,
    /// The session ended; the last-known roster stays visible.
    EndedLastKnownVisible,
}

impl M5PresenceContinuityPosture {
    /// Every continuity posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Live,
        Self::DegradedVisible,
        Self::ReconnectingVisible,
        Self::LocalFallbackVisible,
        Self::EndedLastKnownVisible,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::DegradedVisible => "degraded_visible",
            Self::ReconnectingVisible => "reconnecting_visible",
            Self::LocalFallbackVisible => "local_fallback_visible",
            Self::EndedLastKnownVisible => "ended_last_known_visible",
        }
    }

    /// True when this posture is anything other than live.
    pub const fn is_degraded(self) -> bool {
        !matches!(self, Self::Live)
    }

    /// True when this posture is a local-fallback / ended posture whose who-was-present
    /// and who-had-control truth must be preserved from local state.
    pub const fn is_local_fallback(self) -> bool {
        matches!(
            self,
            Self::LocalFallbackVisible | Self::EndedLastKnownVisible
        )
    }
}

/// The recording-or-retention cue a presence stack shows where applicable, so a session
/// never records or retains silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RecordingRetentionCue {
    /// Recording / retention does not apply to this session.
    NotApplicable,
    /// The session is being recorded.
    Recording,
    /// The session output is retained after it ends.
    Retained,
    /// Retention is pending a decision.
    RetentionPending,
    /// The session is explicitly not recorded.
    NotRecorded,
}

impl M5RecordingRetentionCue {
    /// Every recording-or-retention cue, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NotApplicable,
        Self::Recording,
        Self::Retained,
        Self::RetentionPending,
        Self::NotRecorded,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::Recording => "recording",
            Self::Retained => "retained",
            Self::RetentionPending => "retention_pending",
            Self::NotRecorded => "not_recorded",
        }
    }
}

/// One anatomy part the shared presence avatar stack surfaces. The parts in
/// [`M5PresenceAvatarStackPart::MANDATORY`] are required on every stack so a user can
/// read who is present, their role, and their follow state without relying on avatar
/// imagery alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PresenceAvatarStackPart {
    /// The stacked avatars.
    AvatarStack,
    /// The participant identity labels.
    ParticipantIdentity,
    /// The role badge.
    RoleBadge,
    /// The follow-state badge.
    FollowStateBadge,
    /// The recording-or-retention cue.
    RecordingRetentionCue,
    /// The overflow count for hidden participants.
    OverflowCount,
    /// The textual participant list (non-avatar reachable).
    TextParticipantList,
}

impl M5PresenceAvatarStackPart {
    /// Every avatar-stack part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::AvatarStack,
        Self::ParticipantIdentity,
        Self::RoleBadge,
        Self::FollowStateBadge,
        Self::RecordingRetentionCue,
        Self::OverflowCount,
        Self::TextParticipantList,
    ];

    /// The avatar-stack parts every presence stack must render. The textual participant
    /// list is mandatory so no truth is avatar-only.
    pub const MANDATORY: [Self; 4] = [
        Self::ParticipantIdentity,
        Self::RoleBadge,
        Self::FollowStateBadge,
        Self::TextParticipantList,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AvatarStack => "avatar_stack",
            Self::ParticipantIdentity => "participant_identity",
            Self::RoleBadge => "role_badge",
            Self::FollowStateBadge => "follow_state_badge",
            Self::RecordingRetentionCue => "recording_retention_cue",
            Self::OverflowCount => "overflow_count",
            Self::TextParticipantList => "text_participant_list",
        }
    }
}

/// One anatomy part the shared role-or-follow badge surfaces. The parts in
/// [`M5RoleFollowBadgePart::MANDATORY`] are required so a badge always states the role
/// and the follow state in text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RoleFollowBadgePart {
    /// The role label.
    RoleLabel,
    /// The follow-state label.
    FollowLabel,
    /// The presenter marker.
    PresenterMarker,
    /// The control-holder marker.
    ControlHolderMarker,
    /// The self marker.
    SelfMarker,
}

impl M5RoleFollowBadgePart {
    /// Every role-or-follow badge part, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RoleLabel,
        Self::FollowLabel,
        Self::PresenterMarker,
        Self::ControlHolderMarker,
        Self::SelfMarker,
    ];

    /// The badge parts every role-or-follow badge must render.
    pub const MANDATORY: [Self; 2] = [Self::RoleLabel, Self::FollowLabel];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RoleLabel => "role_label",
            Self::FollowLabel => "follow_label",
            Self::PresenterMarker => "presenter_marker",
            Self::ControlHolderMarker => "control_holder_marker",
            Self::SelfMarker => "self_marker",
        }
    }
}

/// One action a presence stack can offer, so following, control, and reconnect are never
/// dead-ends and a degraded link always keeps a reconnect path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PresenceAction {
    /// Open the full textual participant list.
    ViewParticipantList,
    /// Follow the active presenter.
    FollowPresenter,
    /// Stop following the presenter.
    StopFollowing,
    /// Reconnect the degraded collaboration link.
    ReconnectCollaboration,
}

impl M5PresenceAction {
    /// Every presence action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ViewParticipantList,
        Self::FollowPresenter,
        Self::StopFollowing,
        Self::ReconnectCollaboration,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ViewParticipantList => "view_participant_list",
            Self::FollowPresenter => "follow_presenter",
            Self::StopFollowing => "stop_following",
            Self::ReconnectCollaboration => "reconnect_collaboration",
        }
    }
}

/// A field the support / export packet carries so presence is reconstructable from the
/// shared model. The fields in [`M5PresenceExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PresenceExportField {
    /// The opaque session identity.
    SessionIdentity,
    /// The opaque participant identity.
    ParticipantIdentity,
    /// The participant collaboration role.
    ParticipantRole,
    /// The participant follow state.
    FollowState,
    /// The opaque presenter identity.
    PresenterIdentity,
    /// The opaque control-holder identity.
    ControlHolderIdentity,
    /// The derived continuity posture.
    ContinuityPosture,
    /// The recording-or-retention cue.
    RecordingRetentionCue,
    /// The participant liveness.
    ParticipantLiveness,
    /// The present-participant count.
    PresentCount,
    /// The available actions.
    AvailableActions,
}

impl M5PresenceExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::SessionIdentity,
        Self::ParticipantIdentity,
        Self::ParticipantRole,
        Self::FollowState,
        Self::PresenterIdentity,
        Self::ControlHolderIdentity,
        Self::ContinuityPosture,
        Self::RecordingRetentionCue,
        Self::ParticipantLiveness,
        Self::PresentCount,
        Self::AvailableActions,
    ];

    /// The export fields every presence export must carry.
    pub const MANDATORY: [Self; 6] = [
        Self::SessionIdentity,
        Self::ParticipantIdentity,
        Self::ParticipantRole,
        Self::FollowState,
        Self::PresenterIdentity,
        Self::ContinuityPosture,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionIdentity => "session_identity",
            Self::ParticipantIdentity => "participant_identity",
            Self::ParticipantRole => "participant_role",
            Self::FollowState => "follow_state",
            Self::PresenterIdentity => "presenter_identity",
            Self::ControlHolderIdentity => "control_holder_identity",
            Self::ContinuityPosture => "continuity_posture",
            Self::RecordingRetentionCue => "recording_retention_cue",
            Self::ParticipantLiveness => "participant_liveness",
            Self::PresentCount => "present_count",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// The role-salience rank used to order the presence stack; a lower rank sorts first so
/// the presenter and control holder are never buried below observers.
const fn role_salience(role: M5CollaborationRole) -> u8 {
    match role {
        M5CollaborationRole::Presenter => 0,
        M5CollaborationRole::ControlHolder => 1,
        M5CollaborationRole::SessionHost => 2,
        M5CollaborationRole::Collaborator => 3,
        M5CollaborationRole::Observer => 4,
    }
}

/// One participant in a shared session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PresenceParticipant {
    /// The opaque, export-safe participant identity.
    pub participant_repr: String,
    /// The participant's collaboration role.
    pub role: M5CollaborationRole,
    /// The participant's follow state.
    pub follow_state: M5FollowState,
    /// The participant's liveness.
    pub liveness: M5PresenceParticipantLiveness,
    /// Whether this participant is the local user.
    pub is_self: bool,
}

/// One ranked participant in the resolved presence stack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RankedParticipant {
    /// The opaque participant identity.
    pub participant_repr: String,
    /// The participant's collaboration role.
    pub role: M5CollaborationRole,
    /// The participant's follow state.
    pub follow_state: M5FollowState,
    /// The participant's liveness.
    pub liveness: M5PresenceParticipantLiveness,
    /// Whether this participant is the local user.
    pub is_self: bool,
    /// Whether this participant is the derived presenter.
    pub is_presenter: bool,
    /// Whether this participant is the derived control holder.
    pub is_control_holder: bool,
    /// Whether this participant currently counts as present.
    pub is_present: bool,
    /// The salience rank used to order the stack (lower sorts first).
    pub salience_rank: u8,
}

/// The full input to the presence-stack resolver for one collaboration surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PresenceStackResolutionInput {
    /// The opaque, export-safe session-title representation.
    pub session_title: String,
    /// The participants, in any order.
    pub participants: Vec<M5PresenceParticipant>,
    /// The collaboration link state.
    pub link_state: M5CollaborationLinkState,
    /// The recording-or-retention cue.
    pub recording_cue: M5RecordingRetentionCue,
}

/// The resolved presence stack for one collaboration surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPresenceStack {
    /// The opaque session-title representation.
    pub session_title: String,
    /// The ordered participant stack (most-salient first).
    pub ordered_participants: Vec<M5RankedParticipant>,
    /// The opaque presenter identity, when one is present.
    pub presenter_repr: Option<String>,
    /// The opaque control-holder identity, when one is present.
    pub control_holder_repr: Option<String>,
    /// True when the local user's view is being followed by others (self is being
    /// followed or presenting).
    pub current_view_being_followed: bool,
    /// True when the local user is following the presenter.
    pub self_is_following_presenter: bool,
    /// The collaboration link state.
    pub link_state: M5CollaborationLinkState,
    /// The derived continuity posture.
    pub continuity_posture: M5PresenceContinuityPosture,
    /// True when collaboration state stays visible through this posture. Always `true`.
    pub collaboration_remains_visible: bool,
    /// The recording-or-retention cue.
    pub recording_cue: M5RecordingRetentionCue,
    /// The number of participants that count as present.
    pub present_count: usize,
    /// The number of participants in the roster (present or last-known).
    pub roster_count: usize,
    /// The actions this surface exposes.
    pub available_actions: Vec<M5PresenceAction>,
    /// True when presence never relies on avatar imagery alone (a textual list is always
    /// projected). Always `true`.
    pub non_avatar_reachable: bool,
}

/// Errors returned by [`resolve_presence_stack`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5PresenceStackResolutionError {
    /// The session title was empty.
    EmptySessionTitle,
    /// No participants were supplied.
    EmptyParticipants,
    /// A participant had an empty identity.
    EmptyParticipantRepr,
    /// Two participants shared the same identity.
    DuplicateParticipant,
    /// More than one participant was flagged as the local user.
    DuplicateSelfParticipant,
    /// A representation carried forbidden material.
    ForbiddenPresenceMaterial,
}

impl M5PresenceStackResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptySessionTitle => "empty_session_title",
            Self::EmptyParticipants => "empty_participants",
            Self::EmptyParticipantRepr => "empty_participant_repr",
            Self::DuplicateParticipant => "duplicate_participant",
            Self::DuplicateSelfParticipant => "duplicate_self_participant",
            Self::ForbiddenPresenceMaterial => "forbidden_presence_material",
        }
    }
}

impl fmt::Display for M5PresenceStackResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "presence-stack resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5PresenceStackResolutionError {}

/// Resolves one collaboration surface's presence avatar stack and role-or-follow badges
/// from its participants, collaboration link state, and recording-or-retention cue.
///
/// The presenter is the participant presenting to others (or, failing that, the one
/// whose role is presenter); the control holder is the participant holding the shared
/// control token. The stack is ordered by role salience so the presenter and control
/// holder are never buried. The continuity posture is derived from the link state so a
/// degraded, reconnecting, offline, or ended link keeps presence visible instead of
/// collapsing into a generic banner, and a reconnect action stays attached while the
/// link is recoverable. A textual participant list is always part of the projection, so
/// no truth is avatar-only.
pub fn resolve_presence_stack(
    input: &M5PresenceStackResolutionInput,
) -> Result<M5ResolvedPresenceStack, M5PresenceStackResolutionError> {
    if input.session_title.trim().is_empty() {
        return Err(M5PresenceStackResolutionError::EmptySessionTitle);
    }
    if value_repr_is_forbidden(&input.session_title) {
        return Err(M5PresenceStackResolutionError::ForbiddenPresenceMaterial);
    }
    if input.participants.is_empty() {
        return Err(M5PresenceStackResolutionError::EmptyParticipants);
    }

    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut self_count = 0usize;
    for participant in &input.participants {
        if participant.participant_repr.trim().is_empty() {
            return Err(M5PresenceStackResolutionError::EmptyParticipantRepr);
        }
        if value_repr_is_forbidden(&participant.participant_repr) {
            return Err(M5PresenceStackResolutionError::ForbiddenPresenceMaterial);
        }
        if !seen.insert(participant.participant_repr.as_str()) {
            return Err(M5PresenceStackResolutionError::DuplicateParticipant);
        }
        if participant.is_self {
            self_count += 1;
        }
    }
    if self_count > 1 {
        return Err(M5PresenceStackResolutionError::DuplicateSelfParticipant);
    }

    // The presenter is whoever is presenting to others; failing that, whoever holds the
    // presenter role.
    let presenter = input
        .participants
        .iter()
        .find(|p| p.follow_state == M5FollowState::PresentingToOthers)
        .or_else(|| {
            input
                .participants
                .iter()
                .find(|p| p.role == M5CollaborationRole::Presenter)
        });
    let presenter_repr = presenter.map(|p| p.participant_repr.clone());

    // The control holder is whoever holds the shared control token.
    let control_holder = input
        .participants
        .iter()
        .find(|p| p.role == M5CollaborationRole::ControlHolder);
    let control_holder_repr = control_holder.map(|p| p.participant_repr.clone());

    let self_participant = input.participants.iter().find(|p| p.is_self);
    let self_is_following_presenter = self_participant
        .map(|p| p.follow_state == M5FollowState::FollowingPresenter)
        .unwrap_or(false);
    let current_view_being_followed = self_participant
        .map(|p| {
            matches!(
                p.follow_state,
                M5FollowState::BeingFollowed | M5FollowState::PresentingToOthers
            )
        })
        .unwrap_or(false);
    let self_is_presenter = match (self_participant, presenter) {
        (Some(me), Some(pres)) => me.participant_repr == pres.participant_repr,
        _ => false,
    };

    // The ordered stack: sorted by role salience, self first within a tie, then declared
    // order, so the presenter and control holder float to the top and stay stable.
    let mut ordered: Vec<(usize, &M5PresenceParticipant)> =
        input.participants.iter().enumerate().collect();
    ordered.sort_by_key(|(index, participant)| {
        (
            role_salience(participant.role),
            if participant.is_self { 0u8 } else { 1u8 },
            *index,
        )
    });
    let ordered_participants: Vec<M5RankedParticipant> = ordered
        .iter()
        .map(|(_, participant)| {
            let is_presenter = presenter
                .map(|p| p.participant_repr == participant.participant_repr)
                .unwrap_or(false);
            let is_control_holder = control_holder
                .map(|p| p.participant_repr == participant.participant_repr)
                .unwrap_or(false);
            M5RankedParticipant {
                participant_repr: participant.participant_repr.clone(),
                role: participant.role,
                follow_state: participant.follow_state,
                liveness: participant.liveness,
                is_self: participant.is_self,
                is_presenter,
                is_control_holder,
                is_present: participant.liveness.is_present(),
                salience_rank: role_salience(participant.role),
            }
        })
        .collect();

    let present_count = input
        .participants
        .iter()
        .filter(|p| p.liveness.is_present())
        .count();

    let mut available_actions = vec![M5PresenceAction::ViewParticipantList];
    if presenter_repr.is_some()
        && self_participant.is_some()
        && !self_is_following_presenter
        && !self_is_presenter
    {
        available_actions.push(M5PresenceAction::FollowPresenter);
    }
    if self_is_following_presenter {
        available_actions.push(M5PresenceAction::StopFollowing);
    }
    if input.link_state.is_reconnectable() {
        available_actions.push(M5PresenceAction::ReconnectCollaboration);
    }

    Ok(M5ResolvedPresenceStack {
        session_title: input.session_title.clone(),
        ordered_participants,
        presenter_repr,
        control_holder_repr,
        current_view_being_followed,
        self_is_following_presenter,
        link_state: input.link_state,
        continuity_posture: input.link_state.continuity_posture(),
        collaboration_remains_visible: true,
        recording_cue: input.recording_cue,
        present_count,
        roster_count: input.participants.len(),
        available_actions,
        non_avatar_reachable: true,
    })
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs presence from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PresenceStackResolutionCase {
    /// The resolver input.
    pub input: M5PresenceStackResolutionInput,
    /// The resolved truth. Must equal `resolve_presence_stack(&input)`.
    pub resolved: M5ResolvedPresenceStack,
}

impl M5PresenceStackResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5PresenceStackResolutionInput) -> Self {
        let resolved = resolve_presence_stack(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_presence_stack(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one collaboration surface bound to the shared avatar
/// stack, role-or-follow badge anatomy, roles, follow states, liveness states, link
/// states, continuity postures, recording cues, actions, export fields, and
/// accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PresenceConsumerRow {
    /// Collaboration surface family.
    pub consumer_surface: M5PresenceConsumerSurface,
    /// Qualification class earned by this surface.
    pub qualification: M5RuntimeBoundaryQualificationClass,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Canonical shell zone this stack / badge attaches to.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Responsive classes this component must survive.
    pub responsive_classes: Vec<M5ResponsiveClass>,
    /// Window classes this component keeps continuity across.
    pub window_classes: Vec<M5WindowClass>,
    /// Avatar-stack parts this surface renders (must include the mandatory parts).
    pub stack_parts: Vec<M5PresenceAvatarStackPart>,
    /// Role-or-follow badge parts this surface renders (must include the mandatory
    /// parts).
    pub badge_parts: Vec<M5RoleFollowBadgePart>,
    /// Collaboration roles this surface distinguishes.
    pub collaboration_roles: Vec<M5CollaborationRole>,
    /// Follow states this surface distinguishes.
    pub follow_states: Vec<M5FollowState>,
    /// Participant-liveness states this surface distinguishes.
    pub participant_liveness_states: Vec<M5PresenceParticipantLiveness>,
    /// Collaboration link states this surface distinguishes.
    pub link_states: Vec<M5CollaborationLinkState>,
    /// Continuity postures this surface distinguishes.
    pub continuity_postures: Vec<M5PresenceContinuityPosture>,
    /// Recording-or-retention cues this surface distinguishes.
    pub recording_cues: Vec<M5RecordingRetentionCue>,
    /// Presence actions this surface offers.
    pub presence_actions: Vec<M5PresenceAction>,
    /// Export fields this surface carries (must include the mandatory fields).
    pub export_fields: Vec<M5PresenceExportField>,
    /// Non-visual accessibility routes this surface offers.
    pub accessibility_routes: Vec<M5RuntimeBoundaryAccessibilityRoute>,
    /// Shell subsystems that consume this surface's projection.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this surface.
    pub downgrade_triggers: Vec<M5RuntimeBoundaryDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface.
    pub example_resolutions: Vec<M5PresenceStackResolutionCase>,
    /// Hard invariant: this surface never masks a collaboration role. MUST be `false`.
    pub masks_collaboration_role: bool,
    /// Hard invariant: this surface never leaves the follow state ambiguous. MUST be
    /// `false`.
    pub leaves_follow_state_ambiguous: bool,
    /// Hard invariant: this surface never relies on avatar imagery alone. MUST be
    /// `false`.
    pub relies_on_avatar_imagery_alone: bool,
    /// Hard invariant: this surface never drops presence when the link degrades. MUST be
    /// `false`.
    pub drops_presence_when_degraded: bool,
}

impl M5PresenceConsumerRow {
    /// True when the row declares every mandatory avatar-stack part.
    fn declares_mandatory_stack_parts(&self) -> bool {
        let present: BTreeSet<M5PresenceAvatarStackPart> =
            self.stack_parts.iter().copied().collect();
        M5PresenceAvatarStackPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory badge part.
    fn declares_mandatory_badge_parts(&self) -> bool {
        let present: BTreeSet<M5RoleFollowBadgePart> = self.badge_parts.iter().copied().collect();
        M5RoleFollowBadgePart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5PresenceExportField> = self.export_fields.iter().copied().collect();
        M5PresenceExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_collaboration_role
            && !self.leaves_follow_state_ambiguous
            && !self.relies_on_avatar_imagery_alone
            && !self.drops_presence_when_degraded
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PresenceVocabularySet {
    /// Collaboration-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Avatar-stack-part tokens.
    pub stack_parts: Vec<String>,
    /// Role-or-follow badge-part tokens.
    pub badge_parts: Vec<String>,
    /// Participant-liveness tokens.
    pub participant_liveness_states: Vec<String>,
    /// Collaboration-link-state tokens.
    pub link_states: Vec<String>,
    /// Continuity-posture tokens.
    pub continuity_postures: Vec<String>,
    /// Recording-or-retention-cue tokens.
    pub recording_cues: Vec<String>,
    /// Presence-action tokens.
    pub presence_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Collaboration-role tokens (reused from the frozen matrix).
    pub collaboration_roles: Vec<String>,
    /// Follow-state tokens (reused from the frozen matrix).
    pub follow_states: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5PresenceVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5PresenceConsumerSurface::ALL, |v| v.as_str()),
            stack_parts: tokens(&M5PresenceAvatarStackPart::ALL, |v| v.as_str()),
            badge_parts: tokens(&M5RoleFollowBadgePart::ALL, |v| v.as_str()),
            participant_liveness_states: tokens(&M5PresenceParticipantLiveness::ALL, |v| {
                v.as_str()
            }),
            link_states: tokens(&M5CollaborationLinkState::ALL, |v| v.as_str()),
            continuity_postures: tokens(&M5PresenceContinuityPosture::ALL, |v| v.as_str()),
            recording_cues: tokens(&M5RecordingRetentionCue::ALL, |v| v.as_str()),
            presence_actions: tokens(&M5PresenceAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5PresenceExportField::ALL, |v| v.as_str()),
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
pub struct M5PresenceGovernanceReview {
    /// One primitive carries presence, role, and follow truth on every surface.
    pub one_primitive_carries_presence_role_and_follow: bool,
    /// Participant identity, role, follow state, and presenter identity are always shown.
    pub identity_role_follow_and_presenter_always_shown: bool,
    /// A collaboration role is never masked.
    pub collaboration_role_never_masked: bool,
    /// The follow state is never left ambiguous.
    pub follow_state_never_ambiguous: bool,
    /// Collaboration state stays visible through degraded / reconnecting flows.
    pub collaboration_visible_through_degraded_flows: bool,
    /// Local-fallback continuity preserves who was present and who had control.
    pub local_fallback_preserves_who_was_present_and_in_control: bool,
    /// Presence never relies on avatar imagery alone.
    pub presence_never_relies_on_avatar_imagery_alone: bool,
    /// The support / export packet reconstructs presence truth.
    pub support_export_reconstructs_presence: bool,
    /// No surface invents a second presence grammar.
    pub no_surface_invents_second_presence_grammar: bool,
    /// Every row is bound to a canonical shell zone.
    pub every_row_bound_to_shell_zone: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel presence vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PresenceConsumerProjection {
    /// Strip, terminal, debug, review, HUD, banner, roster, activity, and preview
    /// surfaces all consume the shared primitive.
    pub collaboration_surfaces_consume_shared_primitive: bool,
    /// The presence resolver reads a single canonical participant source.
    pub presence_resolver_reads_single_participant_source: bool,
    /// The role-or-follow badges read a single canonical state source.
    pub role_follow_badges_read_single_state_source: bool,
    /// Continuity reads a single canonical link-state source.
    pub continuity_reads_single_link_state_source: bool,
    /// Support / export reads a single canonical presence source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PresenceProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PresenceReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting presence audit.
    pub presence_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5PresenceAvatarStackPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PresenceAvatarStackPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5PresenceConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5PresenceVocabularySet,
    /// Governance-review block.
    pub governance_review: M5PresenceGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5PresenceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PresenceProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5PresenceReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 presence-avatar-stack primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PresenceAvatarStackPrimitivePacket {
    /// Record kind; must equal [`M5_PRESENCE_AVATAR_STACK_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PRESENCE_AVATAR_STACK_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5PresenceConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5PresenceVocabularySet,
    /// Governance-review block.
    pub governance_review: M5PresenceGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5PresenceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PresenceProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5PresenceReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5PresenceAvatarStackPrimitivePacket {
    /// Builds an M5 presence-avatar-stack primitive packet from stable-lane input.
    pub fn new(input: M5PresenceAvatarStackPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_PRESENCE_AVATAR_STACK_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_PRESENCE_AVATAR_STACK_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            consumer_rows: input.consumer_rows,
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

    /// Validates the M5 presence-avatar-stack primitive invariants.
    pub fn validate(&self) -> Vec<M5PresenceAvatarStackPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PRESENCE_AVATAR_STACK_PRIMITIVE_RECORD_KIND {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PRESENCE_AVATAR_STACK_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_consumer_rows(self, &mut violations);
        validate_presenter_visibility_covered(self, &mut violations);
        validate_degraded_continuity_covered(self, &mut violations);
        validate_local_fallback_continuity_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 presence-avatar-stack primitive packet serializes"),
        ) {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 presence-avatar-stack primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per collaboration surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,shell_zone_slot,stack_parts,badge_parts,collaboration_roles,follow_states,participant_liveness_states,link_states,continuity_postures,recording_cues,presence_actions,export_fields,example_count\n",
        );
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shell_zone_slot.as_str(),
                join_tokens(&row.stack_parts, |v| v.as_str()),
                join_tokens(&row.badge_parts, |v| v.as_str()),
                join_tokens(&row.collaboration_roles, |v| v.as_str()),
                join_tokens(&row.follow_states, |v| v.as_str()),
                join_tokens(&row.participant_liveness_states, |v| v.as_str()),
                join_tokens(&row.link_states, |v| v.as_str()),
                join_tokens(&row.continuity_postures, |v| v.as_str()),
                join_tokens(&row.recording_cues, |v| v.as_str()),
                join_tokens(&row.presence_actions, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_resolutions.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .consumer_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Presence Avatar Stack and Role-or-Follow Badge Primitive: Names, Roles, Recording State, and Local-Fallback Continuity\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Collaboration surfaces: {} ({} stable)\n",
            self.consumer_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Collaboration roles: {}\n",
            self.vocabulary_set.collaboration_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Follow states: {}\n",
            self.vocabulary_set.follow_states.join(", ")
        ));
        out.push_str(&format!(
            "- Continuity postures: {}\n",
            self.vocabulary_set.continuity_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Recording cues: {}\n",
            self.vocabulary_set.recording_cues.join(", ")
        ));
        out.push_str(&format!(
            "- Presence actions: {}\n",
            self.vocabulary_set.presence_actions.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Collaboration surfaces\n\n");
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
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
                let presenter = case
                    .resolved
                    .presenter_repr
                    .as_deref()
                    .unwrap_or("no presenter");
                out.push_str(&format!(
                    "    - `{}` → {} present of {}, presenter `{}`, continuity `{}`, recording `{}`{}\n",
                    case.resolved.session_title,
                    case.resolved.present_count,
                    case.resolved.roster_count,
                    presenter,
                    case.resolved.continuity_posture.as_str(),
                    case.resolved.recording_cue.as_str(),
                    if case.resolved.current_view_being_followed {
                        ", view followed"
                    } else {
                        ""
                    },
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 presence-avatar-stack export.
#[derive(Debug)]
pub enum M5PresenceAvatarStackPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5PresenceAvatarStackPrimitiveViolation>),
}

impl fmt::Display for M5PresenceAvatarStackPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 presence-avatar-stack primitive export parse failed: {error}"
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
                    "m5 presence-avatar-stack primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5PresenceAvatarStackPrimitiveArtifactError {}

/// Validation failures emitted by [`M5PresenceAvatarStackPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5PresenceAvatarStackPrimitiveViolation {
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
    /// A required collaboration-surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A consumer row is incomplete.
    ConsumerRowIncomplete,
    /// A consumer row omits one of the mandatory avatar-stack parts.
    MandatoryStackPartMissing,
    /// A consumer row omits one of the mandatory role-or-follow badge parts.
    MandatoryBadgePartMissing,
    /// A consumer row declares no collaboration roles.
    CollaborationRoleMissing,
    /// A consumer row declares no follow states.
    FollowStateMissing,
    /// A consumer row declares no participant-liveness states.
    ParticipantLivenessMissing,
    /// A consumer row declares no link states.
    LinkStateMissing,
    /// A consumer row declares no continuity postures.
    ContinuityPostureMissing,
    /// A consumer row declares no recording cues.
    RecordingCueMissing,
    /// A consumer row declares no presence actions.
    PresenceActionMissing,
    /// A consumer row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A consumer row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A consumer row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A consumer row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A consumer row declares no worked resolution cases.
    ExampleResolutionMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A surface claiming Stable is missing required proof packet refs.
    StableSurfaceMissingProof,
    /// No worked resolution proves the presenter and follow state visible from the
    /// component.
    PresenterVisibilityUnproven,
    /// No worked resolution proves collaboration state staying visible through a degraded
    /// / reconnecting link.
    DegradedContinuityUnproven,
    /// No worked resolution proves local-fallback continuity preserving who was present
    /// and who had control.
    LocalFallbackContinuityUnproven,
    /// A consumer row violates a hard invariant.
    ConsumerInvariantViolated,
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

impl M5PresenceAvatarStackPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::ConsumerRowIncomplete => "consumer_row_incomplete",
            Self::MandatoryStackPartMissing => "mandatory_stack_part_missing",
            Self::MandatoryBadgePartMissing => "mandatory_badge_part_missing",
            Self::CollaborationRoleMissing => "collaboration_role_missing",
            Self::FollowStateMissing => "follow_state_missing",
            Self::ParticipantLivenessMissing => "participant_liveness_missing",
            Self::LinkStateMissing => "link_state_missing",
            Self::ContinuityPostureMissing => "continuity_posture_missing",
            Self::RecordingCueMissing => "recording_cue_missing",
            Self::PresenceActionMissing => "presence_action_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleResolutionMissing => "example_resolution_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableSurfaceMissingProof => "stable_surface_missing_proof",
            Self::PresenterVisibilityUnproven => "presenter_visibility_unproven",
            Self::DegradedContinuityUnproven => "degraded_continuity_unproven",
            Self::LocalFallbackContinuityUnproven => "local_fallback_continuity_unproven",
            Self::ConsumerInvariantViolated => "consumer_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 presence-avatar-stack export.
pub fn current_stable_m5_presence_avatar_stack_primitive_export(
) -> Result<M5PresenceAvatarStackPrimitivePacket, M5PresenceAvatarStackPrimitiveArtifactError> {
    let packet: M5PresenceAvatarStackPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-presence-avatar-stack-proof/support_export.json"
    )))
    .map_err(M5PresenceAvatarStackPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5PresenceAvatarStackPrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5PresenceAvatarStackPrimitivePacket,
    violations: &mut Vec<M5PresenceAvatarStackPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PRESENCE_AVATAR_STACK_SCHEMA_REF,
        M5_ROLE_FOLLOW_BADGE_SCHEMA_REF,
        M5_PRESENCE_AVATAR_STACK_DOC_REF,
        M5_PRESENCE_AVATAR_STACK_SHELL_ZONE_REF,
        M5_PRESENCE_AVATAR_STACK_COMPONENT_MATRIX_REF,
        M5_PRESENCE_AVATAR_STACK_FOLLOW_STATE_REF,
        M5_PRESENCE_AVATAR_STACK_SESSION_STATE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5PresenceAvatarStackPrimitivePacket,
    violations: &mut Vec<M5PresenceAvatarStackPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5PresenceAvatarStackPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_consumer_rows(
    packet: &M5PresenceAvatarStackPrimitivePacket,
    violations: &mut Vec<M5PresenceAvatarStackPrimitiveViolation>,
) {
    let present: BTreeSet<M5PresenceConsumerSurface> = packet
        .consumer_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in M5PresenceConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.consumer_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.stack_parts.is_empty()
            || row.badge_parts.is_empty()
        {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::ConsumerRowIncomplete);
        }
        if !row.declares_mandatory_stack_parts() {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::MandatoryStackPartMissing);
        }
        if !row.declares_mandatory_badge_parts() {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::MandatoryBadgePartMissing);
        }
        if row.collaboration_roles.is_empty() {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::CollaborationRoleMissing);
        }
        if row.follow_states.is_empty() {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::FollowStateMissing);
        }
        if row.participant_liveness_states.is_empty() {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::ParticipantLivenessMissing);
        }
        if row.link_states.is_empty() {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::LinkStateMissing);
        }
        if row.continuity_postures.is_empty() {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::ContinuityPostureMissing);
        }
        if row.recording_cues.is_empty() {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::RecordingCueMissing);
        }
        if row.presence_actions.is_empty() {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::PresenceActionMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5RuntimeBoundaryAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_resolutions.is_empty() {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::ExampleResolutionMissing);
        }
        if row
            .example_resolutions
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::StableSurfaceMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::ConsumerInvariantViolated);
        }
    }
}

/// At least one worked resolution across the matrix must identify a presenter, and at
/// least one must show the local view being followed — the acceptance-criterion example
/// that a user can tell who is presenting and whether the current view is being followed
/// from the component itself.
fn validate_presenter_visibility_covered(
    packet: &M5PresenceAvatarStackPrimitivePacket,
    violations: &mut Vec<M5PresenceAvatarStackPrimitiveViolation>,
) {
    let cases = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter());
    let mut presenter_shown = false;
    let mut view_followed = false;
    for case in cases {
        if case.resolved.presenter_repr.is_some() {
            presenter_shown = true;
        }
        if case.resolved.current_view_being_followed {
            view_followed = true;
        }
    }
    if !(presenter_shown && view_followed) {
        violations.push(M5PresenceAvatarStackPrimitiveViolation::PresenterVisibilityUnproven);
    }
}

/// At least one worked resolution across the matrix must keep collaboration state visible
/// through a degraded or reconnecting link — the acceptance-criterion example that
/// collaboration state does not disappear into a generic session banner.
fn validate_degraded_continuity_covered(
    packet: &M5PresenceAvatarStackPrimitivePacket,
    violations: &mut Vec<M5PresenceAvatarStackPrimitiveViolation>,
) {
    let proven = packet.consumer_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.continuity_posture.is_degraded()
                && case.resolved.collaboration_remains_visible
                && !case.resolved.ordered_participants.is_empty()
        })
    });
    if !proven {
        violations.push(M5PresenceAvatarStackPrimitiveViolation::DegradedContinuityUnproven);
    }
}

/// At least one worked resolution across the matrix must preserve who was present and who
/// had control through a local-fallback or ended posture — the acceptance-criterion
/// example that collaboration loss downgrades rather than erases presence and control.
fn validate_local_fallback_continuity_covered(
    packet: &M5PresenceAvatarStackPrimitivePacket,
    violations: &mut Vec<M5PresenceAvatarStackPrimitiveViolation>,
) {
    let proven = packet.consumer_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.continuity_posture.is_local_fallback()
                && case.resolved.control_holder_repr.is_some()
                && !case.resolved.ordered_participants.is_empty()
        })
    });
    if !proven {
        violations.push(M5PresenceAvatarStackPrimitiveViolation::LocalFallbackContinuityUnproven);
    }
}

fn validate_governance_review(
    packet: &M5PresenceAvatarStackPrimitivePacket,
    violations: &mut Vec<M5PresenceAvatarStackPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_presence_role_and_follow,
        review.identity_role_follow_and_presenter_always_shown,
        review.collaboration_role_never_masked,
        review.follow_state_never_ambiguous,
        review.collaboration_visible_through_degraded_flows,
        review.local_fallback_preserves_who_was_present_and_in_control,
        review.presence_never_relies_on_avatar_imagery_alone,
        review.support_export_reconstructs_presence,
        review.no_surface_invents_second_presence_grammar,
        review.every_row_bound_to_shell_zone,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5PresenceAvatarStackPrimitivePacket,
    violations: &mut Vec<M5PresenceAvatarStackPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.collaboration_surfaces_consume_shared_primitive,
        projection.presence_resolver_reads_single_participant_source,
        projection.role_follow_badges_read_single_state_source,
        projection.continuity_reads_single_link_state_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5PresenceAvatarStackPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5PresenceAvatarStackPrimitivePacket,
    violations: &mut Vec<M5PresenceAvatarStackPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5PresenceAvatarStackPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5PresenceAvatarStackPrimitivePacket,
    violations: &mut Vec<M5PresenceAvatarStackPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.presence_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5PresenceAvatarStackPrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
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
