//! Frozen M5 shared-terminal/debug-view, control-grant, presenter-token, consent-envelope,
//! retention-review, and session-restore-view matrix.
//!
//! This module locks Aureline's explicit collaboration-control model — the shared terminal / debugger view,
//! the control grant, the presenter / moderator token, the join-time consent envelope, the recording /
//! retention review, and the replay-free session-restore view that a desktop, companion-browser, or support /
//! incident collaboration consumer must treat as first-class, grant-gated, consent-bound objects rather than an
//! emergent side effect of presence, follow mode, browser handoff, or companion resume — into one export-safe
//! packet. Every covered object class is named once here and constrained by the same shared collaboration-control
//! role taxonomy (control_authority_disclosure, active_driver_disclosure, view_first_default_disclosure,
//! consent_scope_disclosure, recording_retention_state_disclosure, paste_secret_guard_disclosure,
//! replay_free_restore_disclosure), the same required visible state (surface label, control authority, active
//! driver, participant roster and roles, session-state summary, consent / retention state, and guard / restore
//! evidence), the same no-acquiring-control-from-presence-or-follow-without-an-explicit-grant rule, the same
//! one-active-driver-per-sensitive-surface rule, the same no-silent-recording-retention-or-guest-scope-widening
//! rule, the same no-replaying-prior-terminal-or-debug-input-on-join-or-restore rule, and the same
//! no-revealing-raw-secrets-command-text-or-clipboard-without-a-guard-and-consent-posture rule regardless of the
//! surface that renders it.
//!
//! The matrix makes an active driver mechanically distinct from a viewer, a commenter, an editor, a navigator, a
//! presenter / moderator, a live-only session, a metadata-audit view, a replayable text / comment timeline, an
//! elevated-support-evidence view, and the control-requested / control-granted / control-expired /
//! recording-active / consent-renewal-required / restore-view-only states (see [`M5CollaborationControlState`])
//! so the shared-terminal / debug view, the join-review sheet, the control-grant prompt, the presenter-handoff
//! sheet, the paste / secret guard, the retention sheet, and support / export packets can key off the session
//! state, control-authority source, and consent / retention gate rather than guessing from a generic presence
//! pill. It does not widen M5 into a full CRDT engine, a relay service, or a general collaboration product — it
//! reuses the already-landed companion session-follow / incident-awareness surfaces, terminal / session
//! restore-no-rerun truth, presence avatar stacks and role / follow badges, embedded / browser auth handoff
//! components, and incident / support export packets — it is the shared reusable collaboration-control contract
//! those consumers read, and it binds back to the already-landed paste / secret guard, stable-proof-index, and
//! migration-task-row packets so collaboration-control truth is not split across surfaces. The controlled
//! vocabularies are frozen in one self-describing [`M5CollaborationControlVocabularySet`] rather than minted per
//! surface. Raw secrets, raw command lines, raw variable bodies, raw clipboard contents, and private endpoints
//! stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_collaboration_control_matrix,
    seeded_m5_collaboration_control_matrix_control_grant_beta_narrowed,
    seeded_m5_collaboration_control_matrix_session_restore_view_preview_narrowed,
    M5_COLLABORATION_CONTROL_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5CollaborationControlMatrixPacket`].
pub const M5_COLLABORATION_CONTROL_MATRIX_RECORD_KIND: &str =
    "freeze_m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_matrix";

/// Schema version for M5 collaboration-control matrix records.
pub const M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined collaboration-control component matrix schema.
pub const M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_REF: &str =
    "schemas/collaboration/m5-collaboration-control-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_COLLABORATION_CONTROL_MATRIX_DOC_REF: &str =
    "docs/collaboration/m5-collaboration-control-ops.md";

/// Repo-relative path of the canonical shared-terminal / debug-view domain schema (the live shared terminal /
/// debugger stream with its active-driver badge and input provenance).
pub const M5_SHARED_TERMINAL_DEBUG_VIEW_DOMAIN_SCHEMA_REF: &str =
    "schemas/ui/m5-shared-terminal-debug-view.schema.json";

/// Repo-relative path of the canonical control-grant domain schema (the explicit grant of terminal / debug
/// write control with its scope, expiry, and single-driver enforcement).
pub const M5_CONTROL_GRANT_DOMAIN_SCHEMA_REF: &str =
    "schemas/collaboration/m5-control-grant.schema.json";

/// Repo-relative path of the canonical presenter-token / presenter-handoff domain schema (the presenter /
/// moderator token, its holder, and its handoff target).
pub const M5_PRESENTER_TOKEN_DOMAIN_SCHEMA_REF: &str =
    "schemas/ui/m5-presenter-handoff-sheet.schema.json";

/// Repo-relative path of the canonical consent-envelope / join-review domain schema (the join-time consent
/// scope disclosing recording, retention, guest scope, and route visibility consequences).
pub const M5_CONSENT_ENVELOPE_DOMAIN_SCHEMA_REF: &str =
    "schemas/ui/m5-collaboration-join-review-sheet.schema.json";

/// Repo-relative path of the canonical retention-review domain schema (the recording / retention / sealed
/// archive review sheet).
pub const M5_RETENTION_REVIEW_DOMAIN_SCHEMA_REF: &str =
    "schemas/ui/m5-collaboration-retention-sheet.schema.json";

/// Repo-relative path of the canonical session-restore-view / session-policy domain schema (the replay-free
/// session-restore view governed by the frozen session policy manifest).
pub const M5_SESSION_RESTORE_VIEW_DOMAIN_SCHEMA_REF: &str =
    "schemas/collaboration/m5-session-policy-manifest.schema.json";

/// Repo-relative path of the paste / secret-guard schema the matrix references for guarded reveal continuity.
pub const M5_PASTE_SECRET_GUARD_LANDED_SCHEMA_REF: &str =
    "schemas/ui/m5-paste-secret-guard.schema.json";

/// Repo-relative path of the already-landed stable-proof-index schema the matrix binds back to.
pub const M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF: &str =
    "schemas/release/stable_proof_index.schema.json";

/// Repo-relative path of the already-landed migration-task-row schema the matrix binds back to.
pub const M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF: &str =
    "schemas/release/m5-migration-task-row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_COLLABORATION_CONTROL_FIXTURE_DIR: &str = "fixtures/collaboration/m5-shared-control";

/// Repo-relative path of the checked support-export artifact.
pub const M5_COLLABORATION_CONTROL_ARTIFACT_REF: &str =
    "artifacts/release/m5-collaboration-control-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_COLLABORATION_CONTROL_CSV_REF: &str =
    "artifacts/release/m5-collaboration-control-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_COLLABORATION_CONTROL_REPORT_REF: &str =
    "artifacts/design/m5-collaboration-control-component-matrix.md";

/// Repo-relative path of the checked collaboration-control-health dashboard.
pub const M5_COLLABORATION_CONTROL_DASHBOARD_REF: &str =
    "dashboards/m5-collaboration-control-health.json";

/// One of the six governed collaboration-control object classes this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlObject {
    /// A shared terminal / debug view: the live shared terminal or debugger stream that begins view-first, names its single active driver, and shows the provenance of every input instead of letting presence imply control.
    SharedTerminalDebugView,
    /// A control grant: the explicit grant of terminal / debug write control that names its authority, enforces a single active driver, and shows its scope, expiry, and revoke / reclaim path.
    ControlGrant,
    /// A presenter token: the presenter / moderator token that names its holder, its handoff target, and its moderation scope, never letting two presenters drive one sensitive surface.
    PresenterToken,
    /// A consent envelope: the join-time consent scope that discloses recording, retention, guest scope, and route visibility consequences before a participant joins, never widening scope silently.
    ConsentEnvelope,
    /// A retention review: the recording / retention / sealed-archive review that names the recording state, retention mode and duration, and replayable-archive scope, never broadening retention silently.
    RetentionReview,
    /// A session-restore view: the replay-free session-restore view that reattaches read-only, replays no prior input, preserves retention scope, and requires a fresh control grant before write control resumes.
    SessionRestoreView,
}

impl M5CollaborationControlObject {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SharedTerminalDebugView,
        Self::ControlGrant,
        Self::PresenterToken,
        Self::ConsentEnvelope,
        Self::RetentionReview,
        Self::SessionRestoreView,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SharedTerminalDebugView => "shared_terminal_debug_view",
            Self::ControlGrant => "control_grant",
            Self::PresenterToken => "presenter_token",
            Self::ConsentEnvelope => "consent_envelope",
            Self::RetentionReview => "retention_review",
            Self::SessionRestoreView => "session_restore_view",
        }
    }
    /// The canonical per-domain schema ref a downstream surface points at instead of restating this
    /// class's shared-view, grant, presenter, consent, retention, or restore meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::SharedTerminalDebugView => M5_SHARED_TERMINAL_DEBUG_VIEW_DOMAIN_SCHEMA_REF,
            Self::ControlGrant => M5_CONTROL_GRANT_DOMAIN_SCHEMA_REF,
            Self::PresenterToken => M5_PRESENTER_TOKEN_DOMAIN_SCHEMA_REF,
            Self::ConsentEnvelope => M5_CONSENT_ENVELOPE_DOMAIN_SCHEMA_REF,
            Self::RetentionReview => M5_RETENTION_REVIEW_DOMAIN_SCHEMA_REF,
            Self::SessionRestoreView => M5_SESSION_RESTORE_VIEW_DOMAIN_SCHEMA_REF,
        }
    }

    /// `true` when this class must name a controlled shared-terminal / debug-view role.
    pub const fn declares_shared_terminal_debug_view_roles(self) -> bool {
        matches!(self, Self::SharedTerminalDebugView)
    }

    /// `true` when this class must name a controlled control-grant role.
    pub const fn declares_control_grant_roles(self) -> bool {
        matches!(self, Self::ControlGrant)
    }

    /// `true` when this class must name a controlled presenter-token role.
    pub const fn declares_presenter_token_roles(self) -> bool {
        matches!(self, Self::PresenterToken)
    }

    /// `true` when this class must name a controlled consent-envelope role.
    pub const fn declares_consent_envelope_roles(self) -> bool {
        matches!(self, Self::ConsentEnvelope)
    }

    /// `true` when this class must name a controlled retention-review role.
    pub const fn declares_retention_review_roles(self) -> bool {
        matches!(self, Self::RetentionReview)
    }

    /// `true` when this class must name a controlled session-restore-view role.
    pub const fn declares_session_restore_view_roles(self) -> bool {
        matches!(self, Self::SessionRestoreView)
    }
}

/// The single controlled collaboration-control role vocabulary every shared-view, grant, presenter, consent, retention, restore, help / docs, or support / export consumer binds to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlRole {
    /// The control authority — who, if anyone, holds terminal / debug write control — disclosed on every claimed surface.
    ControlAuthorityDisclosure,
    /// The single active driver disclosed so presence never reads as control and only one driver holds a sensitive surface.
    ActiveDriverDisclosure,
    /// The view-first default disclosed so every sensitive session begins view-only until an explicit grant is made.
    ViewFirstDefaultDisclosure,
    /// The join-time consent scope disclosed so recording, retention, guest scope, and route visibility consequences are shown before joining.
    ConsentScopeDisclosure,
    /// The recording / retention state disclosed as an explicit posture, never started silently.
    RecordingRetentionStateDisclosure,
    /// The paste / secret guard disclosed so raw secrets, command text, variable bodies, or clipboard contents are never revealed without a guard.
    PasteSecretGuardDisclosure,
    /// The replay-free restore posture disclosed so a restored or reattached session never replays prior input.
    ReplayFreeRestoreDisclosure,
}

impl M5CollaborationControlRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ControlAuthorityDisclosure,
        Self::ActiveDriverDisclosure,
        Self::ViewFirstDefaultDisclosure,
        Self::ConsentScopeDisclosure,
        Self::RecordingRetentionStateDisclosure,
        Self::PasteSecretGuardDisclosure,
        Self::ReplayFreeRestoreDisclosure,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlAuthorityDisclosure => "control_authority_disclosure",
            Self::ActiveDriverDisclosure => "active_driver_disclosure",
            Self::ViewFirstDefaultDisclosure => "view_first_default_disclosure",
            Self::ConsentScopeDisclosure => "consent_scope_disclosure",
            Self::RecordingRetentionStateDisclosure => "recording_retention_state_disclosure",
            Self::PasteSecretGuardDisclosure => "paste_secret_guard_disclosure",
            Self::ReplayFreeRestoreDisclosure => "replay_free_restore_disclosure",
        }
    }
    /// Whether this role is a hard posture requirement that must be present before a class may be
    /// surfaced as a collaboration-control result (`control_authority_disclosure`,
    /// `active_driver_disclosure`, `view_first_default_disclosure`, `consent_scope_disclosure`). The
    /// contextual roles (`recording_retention_state_disclosure`, `paste_secret_guard_disclosure`,
    /// `replay_free_restore_disclosure`) apply where the object class calls for them.
    pub const fn must_be_present_before_surfacing_as_a_collaboration_control_result(self) -> bool {
        matches!(
            self,
            Self::ControlAuthorityDisclosure
                | Self::ActiveDriverDisclosure
                | Self::ViewFirstDefaultDisclosure
                | Self::ConsentScopeDisclosure
        )
    }
}

/// Collaboration-control state that makes an active driver mechanically distinct from a viewer, commenter, editor, navigator, presenter / moderator, live-only session, metadata-audit view, replayable text / comment timeline, elevated-support-evidence view, and the control-requested / control-granted / control-expired / recording-active / consent-renewal-required / restore-view-only states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlState {
    /// Viewer: a read-only participant with no control and no comment authority.
    Viewer,
    /// Commenter: a participant who may comment but holds no terminal / debug write control.
    Commenter,
    /// Editor: a participant with scoped edit authority that is still not terminal / debug control.
    Editor,
    /// Driver: the single participant currently holding live terminal / debug write control.
    Driver,
    /// Navigator: a participant guiding the driver without holding write control.
    Navigator,
    /// Presenter / moderator: the participant holding the presenter token and moderating handoff.
    PresenterModerator,
    /// Live-only: a session whose content is live-only with no recording or retained transcript.
    LiveOnly,
    /// Metadata audit: a view exposing only session metadata for audit, not the content stream.
    MetadataAudit,
    /// Replayable text / comment timeline: a retained, replayable text and comment timeline (not raw input replay).
    ReplayableTextCommentTimeline,
    /// Elevated support evidence: an elevated support / incident evidence view under an explicit consent posture.
    ElevatedSupportEvidence,
    /// Control requested: a participant has requested control and awaits an explicit grant.
    ControlRequested,
    /// Control granted: control has been explicitly granted to a single active driver.
    ControlGranted,
    /// Control expired: a prior grant has expired and control has reverted to view-first.
    ControlExpired,
    /// Recording active: recording / retention is active under a disclosed consent posture.
    RecordingActive,
    /// Consent renewal required: the consent envelope has lapsed and must be renewed before continuing.
    ConsentRenewalRequired,
    /// Restore view-only: a restored or reattached session is view-only until a fresh grant is made.
    RestoreViewOnly,
}

impl M5CollaborationControlState {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 16] = [
        Self::Viewer,
        Self::Commenter,
        Self::Editor,
        Self::Driver,
        Self::Navigator,
        Self::PresenterModerator,
        Self::LiveOnly,
        Self::MetadataAudit,
        Self::ReplayableTextCommentTimeline,
        Self::ElevatedSupportEvidence,
        Self::ControlRequested,
        Self::ControlGranted,
        Self::ControlExpired,
        Self::RecordingActive,
        Self::ConsentRenewalRequired,
        Self::RestoreViewOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Viewer => "viewer",
            Self::Commenter => "commenter",
            Self::Editor => "editor",
            Self::Driver => "driver",
            Self::Navigator => "navigator",
            Self::PresenterModerator => "presenter_moderator",
            Self::LiveOnly => "live_only",
            Self::MetadataAudit => "metadata_audit",
            Self::ReplayableTextCommentTimeline => "replayable_text_comment_timeline",
            Self::ElevatedSupportEvidence => "elevated_support_evidence",
            Self::ControlRequested => "control_requested",
            Self::ControlGranted => "control_granted",
            Self::ControlExpired => "control_expired",
            Self::RecordingActive => "recording_active",
            Self::ConsentRenewalRequired => "consent_renewal_required",
            Self::RestoreViewOnly => "restore_view_only",
        }
    }
    /// `true` only for the active-driver state, so downstream shared-terminal / debug views, the control-grant
    /// prompt, the presenter-handoff sheet, and support / export packets can key off the single active driver
    /// rather than confusing it with a viewer, commenter, editor, navigator, presenter / moderator, or any
    /// live-only / audit / timeline / restore state.
    pub const fn is_active_driver(self) -> bool {
        matches!(self, Self::Driver)
    }
}

/// Named control-authority source (granted by an explicit control grant, delegated by a presenter token, inferred from presence / follow, or an expired / revoked grant) so the four authority kinds are never flattened into one generic control badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlAuthoritySource {
    /// Granted by an explicit control grant: an authoritative, explicit grant of terminal / debug write control.
    GrantedByExplicitControlGrant,
    /// Delegated by a presenter token: control delegated through the presenter / moderator token, not implied.
    DelegatedByPresenterToken,
    /// Inferred from presence / follow: an implied claim to control from presence or follow mode — never sufficient.
    InferredFromPresenceOrFollow,
    /// An expired or revoked grant: a previously granted authority that has expired or been revoked.
    ExpiredOrRevokedGrant,
}

impl M5CollaborationControlAuthoritySource {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::GrantedByExplicitControlGrant,
        Self::DelegatedByPresenterToken,
        Self::InferredFromPresenceOrFollow,
        Self::ExpiredOrRevokedGrant,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GrantedByExplicitControlGrant => "granted_by_explicit_control_grant",
            Self::DelegatedByPresenterToken => "delegated_by_presenter_token",
            Self::InferredFromPresenceOrFollow => "inferred_from_presence_or_follow",
            Self::ExpiredOrRevokedGrant => "expired_or_revoked_grant",
        }
    }
    /// `true` only for an explicit control grant, so a consumer can mechanically refuse to flatten a
    /// presenter-delegated, presence-inferred, or expired / revoked authority into an explicit-grant badge.
    pub const fn is_explicitly_granted(self) -> bool {
        matches!(self, Self::GrantedByExplicitControlGrant)
    }
}

/// Named consent / retention gate (consent current and recording allowed, blocked by missing join consent, blocked by consent renewal required, blocked by retention scope widening, blocked by guest-scope or route expansion) so no claimed surface lacks a named state for a consent or retention block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlRetentionGate {
    /// Consent current, recording allowed: consent is current and recording / retention may proceed under its disclosed scope.
    ConsentCurrentRecordingAllowed,
    /// Blocked by missing join consent: a participant has not accepted the join-time consent scope.
    BlockedByMissingJoinConsent,
    /// Blocked by consent renewal required: the consent envelope has lapsed and must be renewed.
    BlockedByConsentRenewalRequired,
    /// Blocked by retention scope widening: a proposed retention widening is not yet consented to.
    BlockedByRetentionScopeWidening,
    /// Blocked by guest-scope or route expansion: a proposed guest-scope or route-visibility expansion is not yet consented to.
    BlockedByGuestScopeOrRouteExpansion,
}

impl M5CollaborationControlRetentionGate {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ConsentCurrentRecordingAllowed,
        Self::BlockedByMissingJoinConsent,
        Self::BlockedByConsentRenewalRequired,
        Self::BlockedByRetentionScopeWidening,
        Self::BlockedByGuestScopeOrRouteExpansion,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsentCurrentRecordingAllowed => "consent_current_recording_allowed",
            Self::BlockedByMissingJoinConsent => "blocked_by_missing_join_consent",
            Self::BlockedByConsentRenewalRequired => "blocked_by_consent_renewal_required",
            Self::BlockedByRetentionScopeWidening => "blocked_by_retention_scope_widening",
            Self::BlockedByGuestScopeOrRouteExpansion => {
                "blocked_by_guest_scope_or_route_expansion"
            }
        }
    }
    /// `true` for the blocked states (`blocked_by_missing_join_consent`,
    /// `blocked_by_consent_renewal_required`, `blocked_by_retention_scope_widening`,
    /// `blocked_by_guest_scope_or_route_expansion`) so a consumer can mechanically refuse to start or widen
    /// recording / retention while consent is missing, lapsed, or not yet given.
    pub const fn is_blocked_from_recording_or_retention(self) -> bool {
        matches!(
            self,
            Self::BlockedByMissingJoinConsent
                | Self::BlockedByConsentRenewalRequired
                | Self::BlockedByRetentionScopeWidening
                | Self::BlockedByGuestScopeOrRouteExpansion
        )
    }
}

/// Controlled shared-terminal / debug-view role for one live shared terminal or debugger stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SharedTerminalDebugViewRole {
    /// The live stream view shown so participants see the shared terminal / debugger output.
    LiveStreamViewShown,
    /// The single active-driver badge shown so presence never reads as control.
    ActiveDriverBadgeShown,
    /// The view-first-by-default posture shown so a joined participant starts read-only.
    ViewFirstByDefaultShown,
    /// The provenance of every input shown so who typed what is never ambiguous.
    InputProvenanceShown,
    /// A role bound to the single collaboration-control registry.
    BoundToCollaborationControlRegistry,
    /// Replaying prior terminal / debug input on join, which is disallowed.
    InputReplayedOnJoinDisallowed,
}

impl M5SharedTerminalDebugViewRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LiveStreamViewShown,
        Self::ActiveDriverBadgeShown,
        Self::ViewFirstByDefaultShown,
        Self::InputProvenanceShown,
        Self::BoundToCollaborationControlRegistry,
        Self::InputReplayedOnJoinDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveStreamViewShown => "live_stream_view_shown",
            Self::ActiveDriverBadgeShown => "active_driver_badge_shown",
            Self::ViewFirstByDefaultShown => "view_first_by_default_shown",
            Self::InputProvenanceShown => "input_provenance_shown",
            Self::BoundToCollaborationControlRegistry => "bound_to_collaboration_control_registry",
            Self::InputReplayedOnJoinDisallowed => "input_replayed_on_join_disallowed",
        }
    }
}

/// Controlled control-grant role for the explicit grant of terminal / debug write control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ControlGrantRole {
    /// The granted authority shown so a grant names who now holds write control.
    GrantedAuthorityShown,
    /// The single-active-driver rule enforced so only one driver holds a sensitive surface.
    SingleActiveDriverEnforced,
    /// The grant scope and expiry shown so a grant is bounded in reach and time.
    GrantScopeAndExpiryShown,
    /// The revoke and reclaim path shown so control can be taken back at any time.
    RevokeAndReclaimPathShown,
    /// A role bound to the single collaboration-control registry.
    BoundToCollaborationControlRegistry,
    /// Acquiring control from presence or follow alone without an explicit grant, which is disallowed.
    ControlAcquiredFromPresenceAloneDisallowed,
}

impl M5ControlGrantRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::GrantedAuthorityShown,
        Self::SingleActiveDriverEnforced,
        Self::GrantScopeAndExpiryShown,
        Self::RevokeAndReclaimPathShown,
        Self::BoundToCollaborationControlRegistry,
        Self::ControlAcquiredFromPresenceAloneDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GrantedAuthorityShown => "granted_authority_shown",
            Self::SingleActiveDriverEnforced => "single_active_driver_enforced",
            Self::GrantScopeAndExpiryShown => "grant_scope_and_expiry_shown",
            Self::RevokeAndReclaimPathShown => "revoke_and_reclaim_path_shown",
            Self::BoundToCollaborationControlRegistry => "bound_to_collaboration_control_registry",
            Self::ControlAcquiredFromPresenceAloneDisallowed => {
                "control_acquired_from_presence_alone_disallowed"
            }
        }
    }
}

/// Controlled presenter-token role for the presenter / moderator token and its handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PresenterTokenRole {
    /// The presenter holder shown so the surface names who currently presents / moderates.
    PresenterHolderShown,
    /// The handoff target named so a presenter handoff states who receives the token.
    HandoffTargetNamed,
    /// The moderation scope shown so a presenter's authority is bounded and explicit.
    ModerationScopeShown,
    /// The token expiry and reclaim shown so a presenter token is time-bounded and reclaimable.
    TokenExpiryAndReclaimShown,
    /// A role bound to the single collaboration-control registry.
    BoundToCollaborationControlRegistry,
    /// Allowing multiple presenters to drive one sensitive surface at once, which is disallowed.
    MultiplePresentersOnOneSurfaceDisallowed,
}

impl M5PresenterTokenRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PresenterHolderShown,
        Self::HandoffTargetNamed,
        Self::ModerationScopeShown,
        Self::TokenExpiryAndReclaimShown,
        Self::BoundToCollaborationControlRegistry,
        Self::MultiplePresentersOnOneSurfaceDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PresenterHolderShown => "presenter_holder_shown",
            Self::HandoffTargetNamed => "handoff_target_named",
            Self::ModerationScopeShown => "moderation_scope_shown",
            Self::TokenExpiryAndReclaimShown => "token_expiry_and_reclaim_shown",
            Self::BoundToCollaborationControlRegistry => "bound_to_collaboration_control_registry",
            Self::MultiplePresentersOnOneSurfaceDisallowed => {
                "multiple_presenters_on_one_surface_disallowed"
            }
        }
    }
}

/// Controlled consent-envelope role for the join-time consent scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConsentEnvelopeRole {
    /// The join-time consent scope shown so consequences are disclosed before joining.
    JoinTimeConsentScopeShown,
    /// The guest scope and route visibility shown so a guest's reach is explicit at join.
    GuestScopeAndRouteVisibilityShown,
    /// The recording and retention consequences shown so a joiner knows what is retained.
    RecordingAndRetentionConsequencesShown,
    /// The consent renewal requirement shown so a lapsed consent envelope must be renewed.
    ConsentRenewalRequirementShown,
    /// A role bound to the single collaboration-control registry.
    BoundToCollaborationControlRegistry,
    /// Silently widening guest scope or route visibility without renewed consent, which is disallowed.
    SilentGuestScopeOrRouteWideningDisallowed,
}

impl M5ConsentEnvelopeRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::JoinTimeConsentScopeShown,
        Self::GuestScopeAndRouteVisibilityShown,
        Self::RecordingAndRetentionConsequencesShown,
        Self::ConsentRenewalRequirementShown,
        Self::BoundToCollaborationControlRegistry,
        Self::SilentGuestScopeOrRouteWideningDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::JoinTimeConsentScopeShown => "join_time_consent_scope_shown",
            Self::GuestScopeAndRouteVisibilityShown => "guest_scope_and_route_visibility_shown",
            Self::RecordingAndRetentionConsequencesShown => {
                "recording_and_retention_consequences_shown"
            }
            Self::ConsentRenewalRequirementShown => "consent_renewal_requirement_shown",
            Self::BoundToCollaborationControlRegistry => "bound_to_collaboration_control_registry",
            Self::SilentGuestScopeOrRouteWideningDisallowed => {
                "silent_guest_scope_or_route_widening_disallowed"
            }
        }
    }
}

/// Controlled retention-review role for the recording / retention / sealed-archive review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RetentionReviewRole {
    /// The recording state shown so a review names whether recording is live, paused, or off.
    RecordingStateShown,
    /// The retention mode and duration shown so a review names how long content is kept.
    RetentionModeAndDurationShown,
    /// The replayable-archive scope shown so a sealed replayable archive names what it contains.
    ReplayableArchiveScopeShown,
    /// The export and support-evidence scope shown so an elevated evidence export names its reach.
    ExportAndSupportEvidenceScopeShown,
    /// A role bound to the single collaboration-control registry.
    BoundToCollaborationControlRegistry,
    /// Silently starting or widening recording / transcript retention, which is disallowed.
    SilentRecordingOrRetentionWideningDisallowed,
}

impl M5RetentionReviewRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RecordingStateShown,
        Self::RetentionModeAndDurationShown,
        Self::ReplayableArchiveScopeShown,
        Self::ExportAndSupportEvidenceScopeShown,
        Self::BoundToCollaborationControlRegistry,
        Self::SilentRecordingOrRetentionWideningDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecordingStateShown => "recording_state_shown",
            Self::RetentionModeAndDurationShown => "retention_mode_and_duration_shown",
            Self::ReplayableArchiveScopeShown => "replayable_archive_scope_shown",
            Self::ExportAndSupportEvidenceScopeShown => "export_and_support_evidence_scope_shown",
            Self::BoundToCollaborationControlRegistry => "bound_to_collaboration_control_registry",
            Self::SilentRecordingOrRetentionWideningDisallowed => {
                "silent_recording_or_retention_widening_disallowed"
            }
        }
    }
}

/// Controlled session-restore-view role for the replay-free session-restore view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SessionRestoreViewRole {
    /// The restored scrollback shown read-only so a restore never re-runs prior work.
    RestoredScrollbackShownReadOnly,
    /// No input replay on restore shown so a reattached session replays no prior input.
    NoInputReplayOnRestoreShown,
    /// Retention scope preserved on restore shown so a restore never widens retention.
    RetentionScopePreservedOnRestoreShown,
    /// Reattach requires a fresh control grant shown so write control never carries over blindly.
    ReattachRequiresFreshControlGrantShown,
    /// A role bound to the single collaboration-control registry.
    BoundToCollaborationControlRegistry,
    /// Replaying prior terminal / debug input on restore, which is disallowed.
    PriorInputReplayedOnRestoreDisallowed,
}

impl M5SessionRestoreViewRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RestoredScrollbackShownReadOnly,
        Self::NoInputReplayOnRestoreShown,
        Self::RetentionScopePreservedOnRestoreShown,
        Self::ReattachRequiresFreshControlGrantShown,
        Self::BoundToCollaborationControlRegistry,
        Self::PriorInputReplayedOnRestoreDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestoredScrollbackShownReadOnly => "restored_scrollback_shown_read_only",
            Self::NoInputReplayOnRestoreShown => "no_input_replay_on_restore_shown",
            Self::RetentionScopePreservedOnRestoreShown => {
                "retention_scope_preserved_on_restore_shown"
            }
            Self::ReattachRequiresFreshControlGrantShown => {
                "reattach_requires_fresh_control_grant_shown"
            }
            Self::BoundToCollaborationControlRegistry => "bound_to_collaboration_control_registry",
            Self::PriorInputReplayedOnRestoreDisallowed => {
                "prior_input_replayed_on_restore_disallowed"
            }
        }
    }
}

/// Claimed M5 surface family that renders / consumes a collaboration-control object class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlSurfaceFamily {
    /// The desktop collaboration session surface.
    DesktopSessionSurface,
    /// The shared terminal / debugger control surface.
    SharedTerminalDebugSurface,
    /// The companion-browser session-follow surface.
    CompanionBrowserSurface,
    /// The incident / support collaboration surface.
    IncidentSupportSurface,
    /// The support / export surface.
    SupportExport,
    /// The help / docs surface.
    HelpDocs,
}

impl M5CollaborationControlSurfaceFamily {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesktopSessionSurface,
        Self::SharedTerminalDebugSurface,
        Self::CompanionBrowserSurface,
        Self::IncidentSupportSurface,
        Self::SupportExport,
        Self::HelpDocs,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopSessionSurface => "desktop_session_surface",
            Self::SharedTerminalDebugSurface => "shared_terminal_debug_surface",
            Self::CompanionBrowserSurface => "companion_browser_surface",
            Self::IncidentSupportSurface => "incident_support_surface",
            Self::SupportExport => "support_export",
            Self::HelpDocs => "help_docs",
        }
    }
}

/// Classification stage a class passes through from a view-first join to a granted single driver, a reviewed consent / retention posture, and a restored or sealed session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlClassificationStage {
    /// The joined-view-first stage: the participant joins a sensitive session view-only.
    SessionJoinedViewFirst,
    /// The control-requested stage: a participant requests terminal / debug write control.
    ControlRequested,
    /// The control-granted-single-driver stage: control is granted to a single active driver.
    ControlGrantedSingleDriver,
    /// The consent-and-retention-reviewed stage: the consent envelope and retention posture are reviewed.
    ConsentAndRetentionReviewed,
    /// The restored-or-sealed stage: the session is restored replay-free or sealed into a retained archive.
    SessionRestoredOrSealed,
}

impl M5CollaborationControlClassificationStage {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SessionJoinedViewFirst,
        Self::ControlRequested,
        Self::ControlGrantedSingleDriver,
        Self::ConsentAndRetentionReviewed,
        Self::SessionRestoredOrSealed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionJoinedViewFirst => "session_joined_view_first",
            Self::ControlRequested => "control_requested",
            Self::ControlGrantedSingleDriver => "control_granted_single_driver",
            Self::ConsentAndRetentionReviewed => "consent_and_retention_reviewed",
            Self::SessionRestoredOrSealed => "session_restored_or_sealed",
        }
    }
}

/// Shared consumer surface that must agree on a class's collaboration-control truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlConsumerSurface {
    /// The shared terminal / debug view.
    SharedTerminalDebugView,
    /// The collaboration join-review sheet.
    CollaborationJoinReviewSheet,
    /// The control-grant prompt.
    ControlGrantPrompt,
    /// The presenter-handoff sheet.
    PresenterHandoffSheet,
    /// The paste / secret guard.
    PasteSecretGuard,
    /// The collaboration retention sheet.
    CollaborationRetentionSheet,
    /// The session-restore view.
    SessionRestoreView,
    /// The support / export packet.
    SupportExportPacket,
    /// The help / docs surface.
    HelpDocs,
}

impl M5CollaborationControlConsumerSurface {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::SharedTerminalDebugView,
        Self::CollaborationJoinReviewSheet,
        Self::ControlGrantPrompt,
        Self::PresenterHandoffSheet,
        Self::PasteSecretGuard,
        Self::CollaborationRetentionSheet,
        Self::SessionRestoreView,
        Self::SupportExportPacket,
        Self::HelpDocs,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SharedTerminalDebugView => "shared_terminal_debug_view",
            Self::CollaborationJoinReviewSheet => "collaboration_join_review_sheet",
            Self::ControlGrantPrompt => "control_grant_prompt",
            Self::PresenterHandoffSheet => "presenter_handoff_sheet",
            Self::PasteSecretGuard => "paste_secret_guard",
            Self::CollaborationRetentionSheet => "collaboration_retention_sheet",
            Self::SessionRestoreView => "session_restore_view",
            Self::SupportExportPacket => "support_export_packet",
            Self::HelpDocs => "help_docs",
        }
    }
}

/// Non-visual / accessibility route every class must offer so no collaboration-control meaning disappears under zoom, high contrast, keyboard-only use, or export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlAccessibilityRoute {
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

impl M5CollaborationControlAccessibilityRoute {
    /// Every variant, in declaration order.
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

/// Reason a class has degraded below its qualified collaboration-control-handling state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlDegradedReason {
    /// The control authority for the sensitive surface is unresolved.
    ControlAuthorityUnresolved,
    /// The single active driver is unknown.
    ActiveDriverUnknown,
    /// The view-first default cannot be verified.
    ViewFirstDefaultUnverified,
    /// One or more join-time consent-scope consequences are undisclosed.
    ConsentScopeDisclosureIncomplete,
    /// The recording / retention state is unknown.
    RecordingRetentionStateUnknown,
    /// The replay-free restore safety is unknown.
    RestoreReplaySafetyUnknown,
}

impl M5CollaborationControlDegradedReason {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ControlAuthorityUnresolved,
        Self::ActiveDriverUnknown,
        Self::ViewFirstDefaultUnverified,
        Self::ConsentScopeDisclosureIncomplete,
        Self::RecordingRetentionStateUnknown,
        Self::RestoreReplaySafetyUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlAuthorityUnresolved => "control_authority_unresolved",
            Self::ActiveDriverUnknown => "active_driver_unknown",
            Self::ViewFirstDefaultUnverified => "view_first_default_unverified",
            Self::ConsentScopeDisclosureIncomplete => "consent_scope_disclosure_incomplete",
            Self::RecordingRetentionStateUnknown => "recording_retention_state_unknown",
            Self::RestoreReplaySafetyUnknown => "restore_replay_safety_unknown",
        }
    }
}

/// Mandatory label a claimed collaboration-control class must be able to show. The first three are hard requirements; the remaining three make the session state, the control-authority source, and the consent / retention gate mechanically distinct for every covered class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlRequiredLabel {
    /// The class's stable identity.
    Identity,
    /// The class's collaboration-control lifecycle role.
    LifecycleRole,
    /// The canonical per-domain descriptor the class points at.
    CanonicalReference,
    /// The session state the class must show.
    SessionState,
    /// The control-authority source the class must state.
    ControlAuthoritySource,
    /// The consent / retention gate the class must state.
    ConsentRetentionGate,
}

impl M5CollaborationControlRequiredLabel {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::LifecycleRole,
        Self::CanonicalReference,
        Self::SessionState,
        Self::ControlAuthoritySource,
        Self::ConsentRetentionGate,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::LifecycleRole => "lifecycle_role",
            Self::CanonicalReference => "canonical_reference",
            Self::SessionState => "session_state",
            Self::ControlAuthoritySource => "control_authority_source",
            Self::ConsentRetentionGate => "consent_retention_gate",
        }
    }
    /// The three labels every claimed class must be able to show.
    pub const MANDATORY: [Self; 3] = [
        Self::Identity,
        Self::LifecycleRole,
        Self::CanonicalReference,
    ];
}

/// Qualification class for an M5 collaboration-control row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlQualificationClass {
    /// Class collaboration-control handling qualifies for the Stable claim.
    Stable,
    /// Class collaboration-control handling is narrowed to Beta.
    Beta,
    /// Class collaboration-control handling is narrowed to Preview.
    Preview,
    /// Class collaboration-control handling is experimental and not claimed.
    Experimental,
    /// Class collaboration-control handling is unavailable on this build.
    Unavailable,
    /// Class collaboration-control handling is held pending review.
    Held,
}

impl M5CollaborationControlQualificationClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Experimental,
        Self::Unavailable,
        Self::Held,
    ];

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
    /// Whether the class may carry a public Stable collaboration-control-handling claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a collaboration-control class below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollaborationControlDowngradeTrigger {
    /// Terminal / debug control was acquired from presence or follow without an explicit grant.
    ControlAcquiredWithoutExplicitGrant,
    /// More than one active driver was allowed on a sensitive surface.
    MoreThanOneActiveDriverOnASensitiveSurface,
    /// Recording, transcript retention, or guest-scope widening started silently.
    RecordingOrRetentionStartedSilently,
    /// Prior terminal / debug input was replayed on join or restore.
    PriorInputReplayedOnJoinOrRestore,
    /// A raw secret, command text, variable body, or clipboard content was revealed without a guard.
    RawSecretOrClipboardRevealedWithoutGuard,
    /// A class left its control authority unstated.
    ControlAuthorityUnstated,
    /// A class left its single active driver unstated.
    ActiveDriverUnstated,
    /// A class left its view-first default unstated.
    ViewFirstDefaultUnstated,
    /// A class left its join-time consent scope unstated.
    ConsentScopeUnstated,
    /// A class left its recording / retention state unstated.
    RetentionStateUnstated,
    /// A class left its replay-free restore safety unstated.
    RestoreReplaySafetyUnstated,
    /// The collaboration-control matrix packet has gone stale.
    CollaborationControlMatrixStale,
}

impl M5CollaborationControlDowngradeTrigger {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ControlAcquiredWithoutExplicitGrant,
        Self::MoreThanOneActiveDriverOnASensitiveSurface,
        Self::RecordingOrRetentionStartedSilently,
        Self::PriorInputReplayedOnJoinOrRestore,
        Self::RawSecretOrClipboardRevealedWithoutGuard,
        Self::ControlAuthorityUnstated,
        Self::ActiveDriverUnstated,
        Self::ViewFirstDefaultUnstated,
        Self::ConsentScopeUnstated,
        Self::RetentionStateUnstated,
        Self::RestoreReplaySafetyUnstated,
        Self::CollaborationControlMatrixStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlAcquiredWithoutExplicitGrant => "control_acquired_without_explicit_grant",
            Self::MoreThanOneActiveDriverOnASensitiveSurface => {
                "more_than_one_active_driver_on_a_sensitive_surface"
            }
            Self::RecordingOrRetentionStartedSilently => "recording_or_retention_started_silently",
            Self::PriorInputReplayedOnJoinOrRestore => "prior_input_replayed_on_join_or_restore",
            Self::RawSecretOrClipboardRevealedWithoutGuard => {
                "raw_secret_or_clipboard_revealed_without_guard"
            }
            Self::ControlAuthorityUnstated => "control_authority_unstated",
            Self::ActiveDriverUnstated => "active_driver_unstated",
            Self::ViewFirstDefaultUnstated => "view_first_default_unstated",
            Self::ConsentScopeUnstated => "consent_scope_unstated",
            Self::RetentionStateUnstated => "retention_state_unstated",
            Self::RestoreReplaySafetyUnstated => "restore_replay_safety_unstated",
            Self::CollaborationControlMatrixStale => "collaboration_control_matrix_stale",
        }
    }
}

/// Required visible state a class must carry so a collaboration-control result never reads without its control
/// authority, single active driver, participant roster, session state, or consent / retention state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollaborationControlVisibleState {
    /// Class / surface label shown on the surface (shared view, join sheet, control-grant prompt, retention sheet).
    pub surface_label: String,
    /// Control authority — who, if anyone, holds terminal / debug write control.
    pub control_authority: String,
    /// The single active driver, or explicitly none, disclosed before any control claim.
    pub active_driver: String,
    /// Participant roster and per-participant roles (viewer, commenter, editor, navigator, presenter).
    pub participant_roster_and_roles: String,
    /// Session-state summary (viewer / driver / recording-active / restore-view-only, etc.).
    pub session_state_summary: String,
    /// Consent and retention state disclosed at join and while recording / retention is active.
    pub consent_and_retention_state: String,
    /// Paste / secret guard and replay-free restore evidence backing the session.
    pub guard_and_restore_evidence: String,
}

impl M5CollaborationControlVisibleState {
    /// `true` when every required visible-state field is present.
    fn is_complete(&self) -> bool {
        !self.surface_label.trim().is_empty()
            && !self.control_authority.trim().is_empty()
            && !self.active_driver.trim().is_empty()
            && !self.participant_roster_and_roles.trim().is_empty()
            && !self.session_state_summary.trim().is_empty()
            && !self.consent_and_retention_state.trim().is_empty()
            && !self.guard_and_restore_evidence.trim().is_empty()
    }
}

/// One row in the matrix: one governed collaboration-control object class bound to the surface-specific
/// collaboration-control truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollaborationControlRow {
    /// Governed collaboration-control object class.
    pub object_class: M5CollaborationControlObject,
    /// Qualification class earned by this class's collaboration-control handling.
    pub qualification: M5CollaborationControlQualificationClass,
    /// Session state this row governs (distinguishes an active driver from a viewer, a recording-active session, or a restore-view-only session).
    pub session_state: M5CollaborationControlState,
    /// Owner role accountable for keeping this class's collaboration-control state governed.
    pub owner_role: String,
    /// Backup owner role accountable when the primary owner is unavailable.
    pub backup_owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Required visible state that keeps this class's collaboration-control result visibly owned, grant-gated, and consent-honest.
    pub required_visible_state: M5CollaborationControlVisibleState,
    /// Claimed M5 surface families that render / consume this class.
    pub surface_families: Vec<M5CollaborationControlSurfaceFamily>,
    /// Classification stages this class passes through from a view-first join to a restored or sealed session.
    pub classification_stages: Vec<M5CollaborationControlClassificationStage>,
    /// Mandatory labels this class must be able to show (must include the three
    /// [`M5CollaborationControlRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5CollaborationControlRequiredLabel>,
    /// Collaboration-control roles this class can carry (the frozen AC vocabulary; required on every class).
    pub semantic_roles: Vec<M5CollaborationControlRole>,
    /// SharedTerminalDebugView roles this class names (SharedTerminalDebugView only).
    pub shared_terminal_debug_view_roles: Vec<M5SharedTerminalDebugViewRole>,
    /// ControlGrant roles this class names (ControlGrant only).
    pub control_grant_roles: Vec<M5ControlGrantRole>,
    /// PresenterToken roles this class names (PresenterToken only).
    pub presenter_token_roles: Vec<M5PresenterTokenRole>,
    /// ConsentEnvelope roles this class names (ConsentEnvelope only).
    pub consent_envelope_roles: Vec<M5ConsentEnvelopeRole>,
    /// RetentionReview roles this class names (RetentionReview only).
    pub retention_review_roles: Vec<M5RetentionReviewRole>,
    /// SessionRestoreView roles this class names (SessionRestoreView only).
    pub session_restore_view_roles: Vec<M5SessionRestoreViewRole>,
    /// Degraded reasons this class can name (required on every class).
    pub degraded_reasons: Vec<M5CollaborationControlDegradedReason>,
    /// Non-visual accessibility routes this class offers.
    pub accessibility_routes: Vec<M5CollaborationControlAccessibilityRoute>,
    /// First consumer surfaces that consume this class's collaboration-control projection.
    pub consumer_surfaces: Vec<M5CollaborationControlConsumerSurface>,
    /// Downgrade triggers that apply to this class.
    pub downgrade_triggers: Vec<M5CollaborationControlDowngradeTrigger>,
    /// Required closure-artifact refs that keep this class's collaboration-control state provable.
    pub required_closure_artifact_refs: Vec<String>,
    /// Source contract refs consumed by this class (must include its own canonical domain schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this class never lets presence, follow mode, browser handoff, or companion resume acquire terminal / debug control without an explicit grant. MUST be `false`.
    pub acquires_control_from_presence_or_follow_without_an_explicit_grant: bool,
    /// Hard invariant: this class never allows more than one active driver on a sensitive surface. MUST be `false`.
    pub allows_more_than_one_active_driver_on_a_sensitive_surface: bool,
    /// Hard invariant: this class never starts recording, transcript retention, replayable archives, or guest-scope / route widening silently. MUST be `false`.
    pub starts_recording_transcript_retention_or_guest_scope_widening_silently: bool,
    /// Hard invariant: this class never replays prior terminal / debug input on join or restore. MUST be `false`.
    pub replays_prior_terminal_or_debug_input_on_join_or_restore: bool,
    /// Hard invariant: this class never reveals raw secrets, command text, variable bodies, or clipboard contents without a guard and consent posture. MUST be `false`.
    pub reveals_raw_secrets_command_text_or_clipboard_without_a_guard_and_consent_posture: bool,
}

impl M5CollaborationControlRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5CollaborationControlRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5CollaborationControlRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.acquires_control_from_presence_or_follow_without_an_explicit_grant
            && !self.allows_more_than_one_active_driver_on_a_sensitive_surface
            && !self.starts_recording_transcript_retention_or_guest_scope_widening_silently
            && !self.replays_prior_terminal_or_debug_input_on_join_or_restore
            && !self
                .reveals_raw_secrets_command_text_or_clipboard_without_a_guard_and_consent_posture
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollaborationControlVocabularySet {
    /// Object classes tokens.
    pub object_classes: Vec<String>,
    /// Session states tokens.
    pub session_states: Vec<String>,
    /// Control-authority sources tokens.
    pub control_authority_sources: Vec<String>,
    /// Consent / retention gates tokens.
    pub retention_gates: Vec<String>,
    /// Semantic roles tokens.
    pub semantic_roles: Vec<String>,
    /// Shared-terminal / debug-view roles tokens.
    pub shared_terminal_debug_view_roles: Vec<String>,
    /// Control-grant roles tokens.
    pub control_grant_roles: Vec<String>,
    /// Presenter-token roles tokens.
    pub presenter_token_roles: Vec<String>,
    /// Consent-envelope roles tokens.
    pub consent_envelope_roles: Vec<String>,
    /// Retention-review roles tokens.
    pub retention_review_roles: Vec<String>,
    /// Session-restore-view roles tokens.
    pub session_restore_view_roles: Vec<String>,
    /// Surface families tokens.
    pub surface_families: Vec<String>,
    /// Classification stages tokens.
    pub classification_stages: Vec<String>,
    /// Consumer surfaces tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility routes tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded reasons tokens.
    pub degraded_reasons: Vec<String>,
    /// Required labels tokens.
    pub required_labels: Vec<String>,
    /// Downgrade triggers tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5CollaborationControlVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            object_classes: tokens(&M5CollaborationControlObject::ALL, |v| v.as_str()),
            session_states: tokens(&M5CollaborationControlState::ALL, |v| v.as_str()),
            control_authority_sources: tokens(&M5CollaborationControlAuthoritySource::ALL, |v| {
                v.as_str()
            }),
            retention_gates: tokens(&M5CollaborationControlRetentionGate::ALL, |v| v.as_str()),
            semantic_roles: tokens(&M5CollaborationControlRole::ALL, |v| v.as_str()),
            shared_terminal_debug_view_roles: tokens(&M5SharedTerminalDebugViewRole::ALL, |v| {
                v.as_str()
            }),
            control_grant_roles: tokens(&M5ControlGrantRole::ALL, |v| v.as_str()),
            presenter_token_roles: tokens(&M5PresenterTokenRole::ALL, |v| v.as_str()),
            consent_envelope_roles: tokens(&M5ConsentEnvelopeRole::ALL, |v| v.as_str()),
            retention_review_roles: tokens(&M5RetentionReviewRole::ALL, |v| v.as_str()),
            session_restore_view_roles: tokens(&M5SessionRestoreViewRole::ALL, |v| v.as_str()),
            surface_families: tokens(&M5CollaborationControlSurfaceFamily::ALL, |v| v.as_str()),
            classification_stages: tokens(&M5CollaborationControlClassificationStage::ALL, |v| {
                v.as_str()
            }),
            consumer_surfaces: tokens(&M5CollaborationControlConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5CollaborationControlAccessibilityRoute::ALL, |v| {
                v.as_str()
            }),
            degraded_reasons: tokens(&M5CollaborationControlDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5CollaborationControlRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5CollaborationControlDowngradeTrigger::ALL, |v| {
                v.as_str()
            }),
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
pub struct M5CollaborationControlGovernanceReview {
    /// No presence, follow mode, browser handoff, or companion resume implies terminal / debug control.
    pub no_presence_or_follow_implies_terminal_or_debug_control: bool,
    /// Every covered object class names owner backup owner and first consumer.
    pub every_covered_object_class_names_owner_backup_owner_and_first_consumer: bool,
    /// Active driver state is mechanically distinct from a viewer.
    pub active_driver_state_is_mechanically_distinct_from_viewer: bool,
    /// Every sensitive session begins view-first.
    pub every_sensitive_session_begins_view_first: bool,
    /// Every control grant names a single active driver.
    pub every_control_grant_names_a_single_active_driver: bool,
    /// Every join discloses recording, retention, and guest scope.
    pub every_join_discloses_recording_retention_and_guest_scope: bool,
    /// No recording or retention starts silently.
    pub no_recording_or_retention_starts_silently: bool,
    /// Every presenter handoff names its holder and target.
    pub every_presenter_handoff_names_holder_and_target: bool,
    /// No prior terminal / debug input is replayed on join or restore.
    pub no_input_replay_on_join_or_restore: bool,
    /// Every object declares classification stages.
    pub every_object_declares_classification_stages: bool,
    /// Every object declares accessibility route.
    pub every_object_declares_accessibility_route: bool,
    /// Support export reads single collaboration-control source.
    pub support_export_reads_single_collaboration_control_source: bool,
    /// Desktop, terminal, companion, incident, and support bind to a single source.
    pub desktop_terminal_companion_incident_and_support_bind_to_single_source: bool,
    /// Later rows cannot invent parallel collaboration-control vocabulary.
    pub later_rows_cannot_invent_parallel_collaboration_control_vocabulary: bool,
    /// Collaboration-control truth survives zoom and high contrast.
    pub collaboration_control_truth_survives_zoom_and_high_contrast: bool,
    /// Claims narrow automatically when matrix row missing or stale.
    pub claims_narrow_automatically_when_matrix_row_missing_or_stale: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollaborationControlConsumerProjection {
    /// Shared terminal / debug view and join review consume shared collaboration-control truth.
    pub shared_terminal_debug_view_and_join_review_consume_shared_collaboration_control_truth: bool,
    /// Presenter handoff and control grant consume shared authority truth.
    pub presenter_handoff_and_control_grant_consume_shared_authority_truth: bool,
    /// Help and support export consume shared consent and retention truth.
    pub help_and_support_export_consume_shared_consent_and_retention_truth: bool,
    /// Docs help and screenshots read single collaboration-control source.
    pub docs_help_and_screenshots_read_single_collaboration_control_source: bool,
    /// Companion and incident surfaces bind to shared session-state source.
    pub companion_and_incident_surfaces_bind_to_shared_session_state_source: bool,
    /// Support export reads single collaboration-control source.
    pub support_export_reads_single_collaboration_control_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollaborationControlProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof / audit refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the class.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the collaboration-control lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollaborationControlReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting collaboration-control audit for the lane.
    pub collaboration_control_audit_ref: String,
    /// True when support/export parity is required for every class.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every class.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5CollaborationControlMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CollaborationControlMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Collaboration-control rows.
    pub collaboration_control_rows: Vec<M5CollaborationControlRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CollaborationControlVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CollaborationControlGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CollaborationControlConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CollaborationControlProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CollaborationControlReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 collaboration-control matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollaborationControlMatrixPacket {
    /// Record kind; must equal [`M5_COLLABORATION_CONTROL_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Collaboration-control rows.
    pub collaboration_control_rows: Vec<M5CollaborationControlRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CollaborationControlVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CollaborationControlGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CollaborationControlConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CollaborationControlProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CollaborationControlReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CollaborationControlMatrixPacket {
    /// Builds an M5 collaboration-control matrix packet from input.
    pub fn new(input: M5CollaborationControlMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_COLLABORATION_CONTROL_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            collaboration_control_rows: input.collaboration_control_rows,
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

    /// Validates the M5 collaboration-control matrix invariants.
    pub fn validate(&self) -> Vec<M5CollaborationControlMatrixViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_COLLABORATION_CONTROL_MATRIX_RECORD_KIND {
            violations.push(M5CollaborationControlMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_VERSION {
            violations.push(M5CollaborationControlMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CollaborationControlMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_collaboration_control_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 collaboration-control matrix serializes"),
        ) {
            violations.push(M5CollaborationControlMatrixViolation::RawMaterialInExport);
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
            .expect("m5 collaboration-control matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed collaboration-control class.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "object_class,qualification,session_state,owner,backup_owner,canonical_schema,surface_families,classification_stages,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.collaboration_control_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.object_class.as_str(),
                row.qualification.as_str(),
                row.session_state.as_str(),
                csv_field(&row.owner_role),
                csv_field(&row.backup_owner_role),
                row.object_class.canonical_domain_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.classification_stages, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic collaboration-control-health dashboard JSON that session and support surfaces render from one
    /// canonical matrix instead of hand-authoring readiness chrome.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn render_dashboard_json(&self) -> String {
        let objects: Vec<serde_json::Value> = self
            .collaboration_control_rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "object_class": row.object_class.as_str(),
                    "qualification": row.qualification.as_str(),
                    "session_state": row.session_state.as_str(),
                    "canonical_schema": row.object_class.canonical_domain_schema_ref(),
                    "classification_stages": row
                        .classification_stages
                        .iter()
                        .map(|v| v.as_str())
                        .collect::<Vec<_>>(),
                    "consumer_surfaces": row
                        .consumer_surfaces
                        .iter()
                        .map(|v| v.as_str())
                        .collect::<Vec<_>>(),
                })
            })
            .collect();
        let dashboard = serde_json::json!({
            "record_kind": "m5_collaboration_control_health",
            "packet_id": self.packet_id,
            "matrix_label": self.matrix_label,
            "matrix_schema_ref": M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_REF,
            "support_export_ref": M5_COLLABORATION_CONTROL_ARTIFACT_REF,
            "classification_stages": self.vocabulary_set.classification_stages,
            "downgrade_triggers": self.vocabulary_set.downgrade_triggers,
            "objects": objects,
        });
        serde_json::to_string_pretty(&dashboard)
            .expect("m5 collaboration-control-health dashboard serializes")
    }

    /// Deterministic Markdown report for support, docs, or collaboration handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_objects = self
            .collaboration_control_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Shared-Terminal/Debug-View, Control-Grant, Presenter-Token, Consent-Envelope, Retention-Review, and Session-Restore-View Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Object classes: {} ({} stable)\n",
            self.collaboration_control_rows.len(),
            stable_objects
        ));
        out.push_str(&format!(
            "- Collaboration-control roles: {}\n",
            self.vocabulary_set.semantic_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Classification stages: {}\n",
            self.vocabulary_set.classification_stages.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last audit: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Object classes\n\n");
        for row in &self.collaboration_control_rows {
            out.push_str(&format!(
                "- **{}**: `{}` (session_state: `{}`)\n",
                row.object_class.as_str(),
                row.qualification.as_str(),
                row.session_state.as_str()
            ));
            out.push_str(&format!(
                "  - Owner: {} (backup: {})\n",
                row.owner_role, row.backup_owner_role
            ));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.object_class.canonical_domain_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Active driver: {}\n",
                row.required_visible_state.active_driver
            ));
            out.push_str(&format!(
                "  - Consent / retention state: {}\n",
                row.required_visible_state.consent_and_retention_state
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

/// Errors emitted when reading the checked-in M5 collaboration-control matrix export.
#[derive(Debug)]
pub enum M5CollaborationControlMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CollaborationControlMatrixViolation>),
}

impl fmt::Display for M5CollaborationControlMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 collaboration-control matrix export parse failed: {error}"
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
                    "m5 collaboration-control matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CollaborationControlMatrixArtifactError {}

/// Validation failures emitted by [`M5CollaborationControlMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CollaborationControlMatrixViolation {
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
    /// A required governed object class is missing from the matrix.
    RequiredObjectMissing,
    /// A collaboration-control row is incomplete.
    CollaborationControlRowIncomplete,
    /// A collaboration-control row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A collaboration-control row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A class declares no collaboration-control roles.
    SemanticRoleMissing,
    /// The SharedTerminalDebugView class declares no SharedTerminalDebugView roles.
    SharedTerminalDebugViewRoleMissing,
    /// The ControlGrant class declares no ControlGrant roles.
    ControlGrantRoleMissing,
    /// The PresenterToken class declares no PresenterToken roles.
    PresenterTokenRoleMissing,
    /// The ConsentEnvelope class declares no ConsentEnvelope roles.
    ConsentEnvelopeRoleMissing,
    /// The RetentionReview class declares no RetentionReview roles.
    RetentionReviewRoleMissing,
    /// The SessionRestoreView class declares no SessionRestoreView roles.
    SessionRestoreViewRoleMissing,
    /// A class omits required visible-state fields.
    VisibleStateIncomplete,
    /// A class declares no degraded reasons.
    DegradedReasonMissing,
    /// A class declares no surface families.
    SurfaceFamilyMissing,
    /// A class declares no classification stages.
    ClassificationStageMissing,
    /// A class declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A class declares no first consumer surfaces.
    ConsumerSurfacesMissing,
    /// A class declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A class claiming Stable is missing required closure-artifact refs.
    StableObjectMissingClosureArtifact,
    /// A class violates a hard collaboration-control invariant.
    CollaborationControlInvariantViolated,
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

impl M5CollaborationControlMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredObjectMissing => "required_object_missing",
            Self::CollaborationControlRowIncomplete => "collaboration_control_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::SharedTerminalDebugViewRoleMissing => "shared_terminal_debug_view_role_missing",
            Self::ControlGrantRoleMissing => "control_grant_role_missing",
            Self::PresenterTokenRoleMissing => "presenter_token_role_missing",
            Self::ConsentEnvelopeRoleMissing => "consent_envelope_role_missing",
            Self::RetentionReviewRoleMissing => "retention_review_role_missing",
            Self::SessionRestoreViewRoleMissing => "session_restore_view_role_missing",
            Self::VisibleStateIncomplete => "visible_state_incomplete",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::ClassificationStageMissing => "classification_stage_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableObjectMissingClosureArtifact => "stable_object_missing_closure_artifact",
            Self::CollaborationControlInvariantViolated => {
                "collaboration_control_invariant_violated"
            }
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 collaboration-control matrix export.
pub fn current_stable_m5_collaboration_control_matrix_export(
) -> Result<M5CollaborationControlMatrixPacket, M5CollaborationControlMatrixArtifactError> {
    let packet: M5CollaborationControlMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-collaboration-control-proof/support_export.json"
    )))
    .map_err(M5CollaborationControlMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CollaborationControlMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5CollaborationControlMatrixPacket,
    violations: &mut Vec<M5CollaborationControlMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_COLLABORATION_CONTROL_MATRIX_SCHEMA_REF,
        M5_COLLABORATION_CONTROL_MATRIX_DOC_REF,
        M5_SHARED_TERMINAL_DEBUG_VIEW_DOMAIN_SCHEMA_REF,
        M5_CONTROL_GRANT_DOMAIN_SCHEMA_REF,
        M5_PRESENTER_TOKEN_DOMAIN_SCHEMA_REF,
        M5_CONSENT_ENVELOPE_DOMAIN_SCHEMA_REF,
        M5_RETENTION_REVIEW_DOMAIN_SCHEMA_REF,
        M5_SESSION_RESTORE_VIEW_DOMAIN_SCHEMA_REF,
        M5_PASTE_SECRET_GUARD_LANDED_SCHEMA_REF,
        M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5CollaborationControlMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5CollaborationControlMatrixPacket,
    violations: &mut Vec<M5CollaborationControlMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5CollaborationControlMatrixViolation::VocabularySetDrift);
    }
}

fn validate_collaboration_control_rows(
    packet: &M5CollaborationControlMatrixPacket,
    violations: &mut Vec<M5CollaborationControlMatrixViolation>,
) {
    let present: BTreeSet<M5CollaborationControlObject> = packet
        .collaboration_control_rows
        .iter()
        .map(|row| row.object_class)
        .collect();
    for required in M5CollaborationControlObject::ALL {
        if !present.contains(&required) {
            violations.push(M5CollaborationControlMatrixViolation::RequiredObjectMissing);
            return;
        }
    }

    for row in &packet.collaboration_control_rows {
        let class = row.object_class;
        if row.owner_role.trim().is_empty()
            || row.backup_owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations
                .push(M5CollaborationControlMatrixViolation::CollaborationControlRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5CollaborationControlMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == class.canonical_domain_schema_ref())
        {
            violations.push(M5CollaborationControlMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5CollaborationControlMatrixViolation::SemanticRoleMissing);
        }
        if class.declares_shared_terminal_debug_view_roles()
            && row.shared_terminal_debug_view_roles.is_empty()
        {
            violations
                .push(M5CollaborationControlMatrixViolation::SharedTerminalDebugViewRoleMissing);
        }
        if class.declares_control_grant_roles() && row.control_grant_roles.is_empty() {
            violations.push(M5CollaborationControlMatrixViolation::ControlGrantRoleMissing);
        }
        if class.declares_presenter_token_roles() && row.presenter_token_roles.is_empty() {
            violations.push(M5CollaborationControlMatrixViolation::PresenterTokenRoleMissing);
        }
        if class.declares_consent_envelope_roles() && row.consent_envelope_roles.is_empty() {
            violations.push(M5CollaborationControlMatrixViolation::ConsentEnvelopeRoleMissing);
        }
        if class.declares_retention_review_roles() && row.retention_review_roles.is_empty() {
            violations.push(M5CollaborationControlMatrixViolation::RetentionReviewRoleMissing);
        }
        if class.declares_session_restore_view_roles() && row.session_restore_view_roles.is_empty()
        {
            violations.push(M5CollaborationControlMatrixViolation::SessionRestoreViewRoleMissing);
        }
        if !row.required_visible_state.is_complete() {
            violations.push(M5CollaborationControlMatrixViolation::VisibleStateIncomplete);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5CollaborationControlMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5CollaborationControlMatrixViolation::SurfaceFamilyMissing);
        }
        if row.classification_stages.is_empty() {
            violations.push(M5CollaborationControlMatrixViolation::ClassificationStageMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5CollaborationControlMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5CollaborationControlMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5CollaborationControlMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_closure_artifact_refs.is_empty() {
            violations
                .push(M5CollaborationControlMatrixViolation::StableObjectMissingClosureArtifact);
        }
        if !row.honours_invariants() {
            violations
                .push(M5CollaborationControlMatrixViolation::CollaborationControlInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5CollaborationControlMatrixPacket,
    violations: &mut Vec<M5CollaborationControlMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.no_presence_or_follow_implies_terminal_or_debug_control,
        review.every_covered_object_class_names_owner_backup_owner_and_first_consumer,
        review.active_driver_state_is_mechanically_distinct_from_viewer,
        review.every_sensitive_session_begins_view_first,
        review.every_control_grant_names_a_single_active_driver,
        review.every_join_discloses_recording_retention_and_guest_scope,
        review.no_recording_or_retention_starts_silently,
        review.every_presenter_handoff_names_holder_and_target,
        review.no_input_replay_on_join_or_restore,
        review.every_object_declares_classification_stages,
        review.every_object_declares_accessibility_route,
        review.support_export_reads_single_collaboration_control_source,
        review.desktop_terminal_companion_incident_and_support_bind_to_single_source,
        review.later_rows_cannot_invent_parallel_collaboration_control_vocabulary,
        review.collaboration_control_truth_survives_zoom_and_high_contrast,
        review.claims_narrow_automatically_when_matrix_row_missing_or_stale,
    ] {
        if !ok {
            violations.push(M5CollaborationControlMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5CollaborationControlMatrixPacket,
    violations: &mut Vec<M5CollaborationControlMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection
            .shared_terminal_debug_view_and_join_review_consume_shared_collaboration_control_truth,
        projection.presenter_handoff_and_control_grant_consume_shared_authority_truth,
        projection.help_and_support_export_consume_shared_consent_and_retention_truth,
        projection.docs_help_and_screenshots_read_single_collaboration_control_source,
        projection.companion_and_incident_surfaces_bind_to_shared_session_state_source,
        projection.support_export_reads_single_collaboration_control_source,
    ] {
        if !ok {
            violations.push(M5CollaborationControlMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5CollaborationControlMatrixPacket,
    violations: &mut Vec<M5CollaborationControlMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5CollaborationControlMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5CollaborationControlMatrixPacket,
    violations: &mut Vec<M5CollaborationControlMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.collaboration_control_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5CollaborationControlMatrixViolation::ReleasePostureIncomplete);
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
/// deliberately uses control / consent / recording / retention / secret words; what is rejected is a raw secret
/// *value* shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
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
