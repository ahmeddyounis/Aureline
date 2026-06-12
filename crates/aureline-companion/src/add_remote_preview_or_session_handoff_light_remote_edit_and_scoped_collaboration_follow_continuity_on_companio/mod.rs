//! Companion remote-preview, session-handoff, light-remote-edit, and scoped
//! collaboration-follow continuity surfaces.
//!
//! This module owns the export-safe truth packet for the companion surfaces that
//! let a browser or mobile companion *preview* and *hand off* an active desktop
//! session, perform a single bounded *light remote edit*, and *follow* a
//! collaborator within a bounded shared scope. It binds three surfaces —
//! remote-preview-handoff, light-remote-edit, and collaboration-follow — to the
//! frozen M5 companion-matrix lanes that qualify them, and gives every item an
//! exact [`CompanionDesktopHandoff`] (reused from
//! [`crate::companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff`])
//! so a companion tap resumes the precise host context.
//!
//! Three invariants make these surfaces safe to ship. First, **bounded read/write
//! scope**: the remote-preview-handoff and collaboration-follow surfaces are
//! strictly read-only; only the light-remote-edit surface may write, and its writes
//! are capped, relayed to the host, and require explicit host approval. Second,
//! **scoped collaboration-follow continuity**: collaboration-follow is bounded to a
//! shared scope that the host can revoke, never an unbounded view of a
//! collaborator's machine. Third, **local-core continuity**: a remote preview is a
//! read-only projection, a session handoff never strands user-owned local work, and
//! the local core stays authoritative throughout. Stale or unknown freshness is
//! always labeled, and a degraded item is never shown as live.
//!
//! The packet reuses the matrix vocabulary from
//! [`crate::freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes`]
//! ([`M5CompanionQualificationClass`], [`M5CompanionRolloutStage`],
//! [`M5CompanionDowngradeTrigger`], [`M5CompanionRollbackPosture`],
//! [`M5CompanionLocalityDisclosure`], [`M5CompanionConsumerSurface`]) instead of
//! inventing parallel terms, and each surface row records the matrix lane it
//! inherits qualification from. It builds directly on the bounded read/write and
//! stale-state-honesty foundation in
//! [`crate::ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty`]
//! rather than re-describing it.
//!
//! [`CompanionContinuitySurfacePacket::apply_companion_continuity_degradation`]
//! narrows surfaces and downgrades freshness, handoff resolution, and continuity
//! from a per-surface observation — when the relay is unavailable, proof is stale,
//! the host session is inactive, trust narrowed, the collaboration scope was
//! revoked, or an upstream matrix lane narrowed — so CI or release tooling degrades
//! the surface honestly rather than show a fresh preview, a handoff that no longer
//! resolves, or a collaboration scope that no longer applies. Degraded state is
//! labeled, never hidden, and local work is never stranded.
//!
//! [`canonical_companion_continuity_surface`] builds the surface and
//! [`current_stable_companion_continuity_surface_export`] reads and validates the
//! checked-in support export, so browser/mobile companions, the desktop panel,
//! diagnostics, support exports, and Help/About ingest the packet rather than
//! cloning status text. Credential bodies, raw provider payloads, and raw session,
//! edit, or collaboration bodies stay outside this boundary.
//!
//! The boundary schema is
//! [`schemas/companion/add-remote-preview-or-session-handoff-light-remote-edit-and-scoped-collaboration-follow-continuity-on-companio.schema.json`](../../../../schemas/companion/add-remote-preview-or-session-handoff-light-remote-edit-and-scoped-collaboration-follow-continuity-on-companio.schema.json).
//! The contract doc is
//! [`docs/companion/m5/add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio.md`](../../../../docs/companion/m5/add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio.md).
//! The protected fixture directory is
//! [`fixtures/companion/m5/add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio/`](../../../../fixtures/companion/m5/add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use aureline_auth::{
    secret_boundary_use_audit_result_for_health, seeded_secret_boundary_active_repair_state,
    seeded_secret_boundary_profile_parity_rows, seeded_secret_boundary_repairable_states,
    SecretBoundaryActingIdentityClass, SecretBoundaryConsumerIdentityClass,
    SecretBoundaryConsumerIdentityReceipt, SecretBoundaryCredentialMode,
    SecretBoundaryCredentialStateRow, SecretBoundaryDeclinePath,
    SecretBoundaryDelegatedCredentialRow, SecretBoundaryDelegatedUseClass,
    SecretBoundaryExportSafetyBanner, SecretBoundaryHealthStateClass,
    SecretBoundaryProjectionControl, SecretBoundaryProjectionControlClass,
    SecretBoundaryProjectionMode, SecretBoundaryProjectionModeAudit,
    SecretBoundaryRepairOwnerClass, SecretBoundarySecretAccessPrompt, SecretBoundarySecretClass,
    SecretBoundaryStorageClass, SecretBoundarySurfaceState, SecretBoundaryWorkflowDependency,
    M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF,
};
use serde::{Deserialize, Serialize};

use crate::companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff::{
    CompanionDesktopHandoff, CompanionHandoffResolution, CompanionHandoffTarget,
};
use crate::freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes::{
    M5CompanionConsumerSurface, M5CompanionDowngradeTrigger, M5CompanionLocalityDisclosure,
    M5CompanionMatrixLane, M5CompanionQualificationClass, M5CompanionRollbackPosture,
    M5CompanionRolloutStage, M5_COMPANION_BOUNDARY_MANIFEST_REF, M5_COMPANION_MATRIX_SCHEMA_REF,
    M5_COMPANION_QUALIFICATION_REF, M5_COMPANION_SURFACE_CONTRACT_REF,
};

/// Stable record-kind tag carried by [`CompanionContinuitySurfacePacket`].
pub const COMPANION_CONTINUITY_RECORD_KIND: &str =
    "add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio";

/// Schema version for companion continuity surface records.
pub const COMPANION_CONTINUITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const COMPANION_CONTINUITY_SCHEMA_REF: &str =
    "schemas/companion/add-remote-preview-or-session-handoff-light-remote-edit-and-scoped-collaboration-follow-continuity-on-companio.schema.json";

/// Repo-relative path of the companion continuity surface contract doc.
pub const COMPANION_CONTINUITY_DOC_REF: &str =
    "docs/companion/m5/add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio.md";

/// Repo-relative path of the protected fixture directory.
pub const COMPANION_CONTINUITY_FIXTURE_DIR: &str =
    "fixtures/companion/m5/add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio";

/// Repo-relative path of the checked support-export artifact.
pub const COMPANION_CONTINUITY_ARTIFACT_REF: &str =
    "artifacts/companion/m5/add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const COMPANION_CONTINUITY_SUMMARY_REF: &str =
    "artifacts/companion/m5/add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio.md";

/// One of the three companion continuity surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionContinuitySurface {
    /// Read-only remote preview of a host session with session-handoff continuity.
    RemotePreviewHandoff,
    /// The single bounded, host-approved light-remote-edit surface.
    LightRemoteEdit,
    /// Read-only, scoped follow of a collaborator within a shared scope.
    CollaborationFollow,
}

impl CompanionContinuitySurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::RemotePreviewHandoff,
        Self::LightRemoteEdit,
        Self::CollaborationFollow,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RemotePreviewHandoff => "remote_preview_handoff",
            Self::LightRemoteEdit => "light_remote_edit",
            Self::CollaborationFollow => "collaboration_follow",
        }
    }

    /// Frozen M5 companion-matrix lane this surface inherits qualification from.
    pub const fn matrix_lane(self) -> M5CompanionMatrixLane {
        match self {
            Self::RemotePreviewHandoff => M5CompanionMatrixLane::CompanionSessionFollow,
            Self::LightRemoteEdit => M5CompanionMatrixLane::CompanionLightEdit,
            Self::CollaborationFollow => M5CompanionMatrixLane::CompanionReview,
        }
    }

    /// Read/write scope this surface is bounded to.
    ///
    /// Remote-preview-handoff and collaboration-follow are strictly read-only;
    /// only the light-remote-edit surface may write, and only via a host-approved
    /// relay.
    pub const fn bounded_scope(self) -> CompanionContinuityScope {
        match self {
            Self::RemotePreviewHandoff | Self::CollaborationFollow => {
                CompanionContinuityScope::ReadOnly
            }
            Self::LightRemoteEdit => CompanionContinuityScope::BoundedWriteRelayedToHost,
        }
    }
}

/// Bounded read/write scope a companion continuity surface or item is allowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionContinuityScope {
    /// Strictly read-only: the companion never mutates host state.
    ReadOnly,
    /// Bounded write relayed to the host for preview and explicit approval.
    BoundedWriteRelayedToHost,
}

impl CompanionContinuityScope {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::BoundedWriteRelayedToHost => "bounded_write_relayed_to_host",
        }
    }
}

/// Freshness state of a preview, edit, or collaboration-follow item.
///
/// Stale-state honesty requires that [`Self::Stale`] and [`Self::Unknown`] items
/// are always labeled, and that a degraded item is never re-shown as
/// [`Self::Live`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionContinuityFreshness {
    /// Streaming live from the local core via the relay.
    Live,
    /// Last-known cached value within its freshness window.
    Cached,
    /// Stale beyond its freshness window.
    Stale,
    /// Freshness could not be determined.
    Unknown,
}

impl CompanionContinuityFreshness {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::Unknown => "unknown",
        }
    }

    /// True when this state must carry an explicit stale/unknown label.
    pub const fn requires_label(self) -> bool {
        matches!(self, Self::Stale | Self::Unknown)
    }

    /// Forces a live or cached state to [`Self::Stale`]; honest states are kept.
    pub const fn forced_stale(self) -> Self {
        match self {
            Self::Live | Self::Cached => Self::Stale,
            Self::Stale | Self::Unknown => self,
        }
    }
}

/// Kind of host session a companion is remotely previewing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemotePreviewKind {
    /// A live editor preview.
    EditorPreview,
    /// A running agent session preview.
    AgentRunPreview,
    /// A terminal output stream preview.
    TerminalPreview,
    /// A diff/review preview.
    DiffPreview,
    /// A build/CI status preview.
    BuildPreview,
}

impl RemotePreviewKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorPreview => "editor_preview",
            Self::AgentRunPreview => "agent_run_preview",
            Self::TerminalPreview => "terminal_preview",
            Self::DiffPreview => "diff_preview",
            Self::BuildPreview => "build_preview",
        }
    }
}

/// Session-handoff continuity state of a remote-preview item.
///
/// The local core stays authoritative: a handoff stages and resumes the host
/// context, but [`Self::HandoffUnavailable`] never strands user-owned local work —
/// it falls back to the local-authoritative state rather than dropping it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionHandoffContinuity {
    /// No handoff in flight; the local core is authoritative.
    LocalAuthoritative,
    /// A handoff has been staged to another device.
    HandoffStaged,
    /// The session was resumed from the preview via an exact handoff.
    HandoffResumed,
    /// The handoff cannot complete right now; local work is preserved.
    HandoffUnavailable,
}

impl SessionHandoffContinuity {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalAuthoritative => "local_authoritative",
            Self::HandoffStaged => "handoff_staged",
            Self::HandoffResumed => "handoff_resumed",
            Self::HandoffUnavailable => "handoff_unavailable",
        }
    }

    /// Downgrades a staged/resumed handoff to [`Self::HandoffUnavailable`]; the
    /// local-authoritative and already-unavailable states are kept.
    pub const fn forced_unavailable(self) -> Self {
        match self {
            Self::HandoffStaged | Self::HandoffResumed => Self::HandoffUnavailable,
            Self::LocalAuthoritative | Self::HandoffUnavailable => self,
        }
    }
}

/// Kind of bounded light remote edit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteEditKind {
    /// A bounded text touch-up.
    TextTouchUp,
    /// A reply to a comment thread.
    CommentReply,
    /// A rename suggestion.
    RenameSuggestion,
    /// Resolution of a tracked TODO.
    TodoResolution,
}

impl RemoteEditKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TextTouchUp => "text_touch_up",
            Self::CommentReply => "comment_reply",
            Self::RenameSuggestion => "rename_suggestion",
            Self::TodoResolution => "todo_resolution",
        }
    }
}

/// Lifecycle state of a bounded light remote edit.
///
/// Every state passes through the host: a companion drafts an edit and relays it,
/// but the host previews, approves, and applies it. There is deliberately no
/// "applied by companion" state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemoteEditState {
    /// Drafted on the companion, not yet relayed.
    Drafted,
    /// Relayed to the host for preview.
    RelayedForPreview,
    /// Awaiting explicit host approval.
    AwaitingHostApproval,
    /// Applied by the host after approval.
    AppliedByHost,
    /// Rejected by the host.
    RejectedByHost,
}

impl RemoteEditState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Drafted => "drafted",
            Self::RelayedForPreview => "relayed_for_preview",
            Self::AwaitingHostApproval => "awaiting_host_approval",
            Self::AppliedByHost => "applied_by_host",
            Self::RejectedByHost => "rejected_by_host",
        }
    }
}

/// Role a collaborator holds in a scoped collaboration-follow item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationRole {
    /// The collaborator actively driving the shared session.
    Driver,
    /// The collaborator navigating alongside the driver.
    Navigator,
    /// A passive observer of the shared scope.
    Observer,
    /// A reviewer following a shared review scope.
    Reviewer,
}

impl CollaborationRole {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Driver => "driver",
            Self::Navigator => "navigator",
            Self::Observer => "observer",
            Self::Reviewer => "reviewer",
        }
    }
}

/// Bounded scope a collaboration-follow item is confined to.
///
/// Collaboration-follow is never an unbounded view of a collaborator's machine; it
/// is confined to a shared scope. [`Self::ScopeRevoked`] records that the host
/// withdrew the shared scope, narrowing the follow rather than continuing it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationFollowScope {
    /// Bounded to a shared set of files.
    SharedFileScope,
    /// Bounded to a shared review thread.
    SharedReviewScope,
    /// Bounded to a shared session view.
    SharedSessionScope,
    /// The shared scope was revoked by the host.
    ScopeRevoked,
}

impl CollaborationFollowScope {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SharedFileScope => "shared_file_scope",
            Self::SharedReviewScope => "shared_review_scope",
            Self::SharedSessionScope => "shared_session_scope",
            Self::ScopeRevoked => "scope_revoked",
        }
    }
}

/// Reason a companion continuity surface has degraded below its qualified state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionContinuityDegradedReason {
    /// The companion relay is unavailable.
    RelayUnavailable,
    /// Proof has gone stale.
    ProofStale,
    /// No active desktop host session.
    HostSessionInactive,
    /// Workspace or device trust narrowed.
    TrustNarrowed,
    /// The shared collaboration scope was revoked.
    CollaborationScopeRevoked,
    /// An upstream frozen matrix lane narrowed.
    UpstreamMatrixNarrowed,
    /// One or more handoff targets could not resolve exactly.
    HandoffTargetUnresolved,
    /// One or more item freshness states were downgraded to stale.
    FreshnessDowngradedToStale,
}

impl CompanionContinuityDegradedReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RelayUnavailable => "relay_unavailable",
            Self::ProofStale => "proof_stale",
            Self::HostSessionInactive => "host_session_inactive",
            Self::TrustNarrowed => "trust_narrowed",
            Self::CollaborationScopeRevoked => "collaboration_scope_revoked",
            Self::UpstreamMatrixNarrowed => "upstream_matrix_narrowed",
            Self::HandoffTargetUnresolved => "handoff_target_unresolved",
            Self::FreshnessDowngradedToStale => "freshness_downgraded_to_stale",
        }
    }
}

/// A read-only remote-preview item with session-handoff continuity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemotePreviewItem {
    /// Stable item id.
    pub item_id: String,
    /// Kind of host session being previewed.
    pub preview_kind: RemotePreviewKind,
    /// Session-handoff continuity state.
    pub handoff_continuity: SessionHandoffContinuity,
    /// Freshness of the previewed state.
    pub freshness: CompanionContinuityFreshness,
    /// Read/write scope. Always [`CompanionContinuityScope::ReadOnly`].
    pub read_write_scope: CompanionContinuityScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Always true: a handoff never strands user-owned local work.
    pub local_work_preserved: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the originating local-core session. Carries no payload body.
    pub source_session_ref: String,
    /// Exact desktop handoff back to the host session.
    pub handoff: CompanionDesktopHandoff,
}

/// A bounded, host-approved light-remote-edit item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightRemoteEditItem {
    /// Stable item id.
    pub item_id: String,
    /// Kind of light remote edit.
    pub edit_kind: RemoteEditKind,
    /// Lifecycle state of the edit.
    pub edit_state: RemoteEditState,
    /// Human-readable description of the bound this edit may not exceed.
    pub write_bound_summary: String,
    /// Freshness of the underlying target.
    pub freshness: CompanionContinuityFreshness,
    /// Read/write scope. Always
    /// [`CompanionContinuityScope::BoundedWriteRelayedToHost`].
    pub read_write_scope: CompanionContinuityScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Always true: the host must approve before the edit applies.
    pub requires_host_approval: bool,
    /// Always true: a rejected or unrelayed edit never strands local work.
    pub local_work_preserved: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the edit target. Carries no payload body.
    pub target_ref: String,
    /// Exact desktop handoff to the host preview/approval location.
    pub handoff: CompanionDesktopHandoff,
}

/// A read-only, scoped collaboration-follow item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationFollowItem {
    /// Stable item id.
    pub item_id: String,
    /// Role the collaborator holds in the shared scope.
    pub collaboration_role: CollaborationRole,
    /// Bounded scope this follow is confined to.
    pub follow_scope: CollaborationFollowScope,
    /// Freshness of the followed collaboration state.
    pub freshness: CompanionContinuityFreshness,
    /// Read/write scope. Always [`CompanionContinuityScope::ReadOnly`].
    pub read_write_scope: CompanionContinuityScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Always true: the follow is bounded to a host-revocable shared scope.
    pub scope_bounded: bool,
    /// Always true: leaving or losing the scope never strands local work.
    pub local_work_preserved: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the collaborator handle. Carries no payload body.
    pub collaborator_ref: String,
    /// Exact desktop handoff into the shared review/session scope.
    pub handoff: CompanionDesktopHandoff,
}

/// Per-surface qualification inherited from the frozen M5 matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionContinuitySurfaceQualification {
    /// Surface the row applies to.
    pub surface: CompanionContinuitySurface,
    /// Qualification class earned by this surface.
    pub qualification: M5CompanionQualificationClass,
    /// Staged rollout stage.
    pub rollout_stage: M5CompanionRolloutStage,
    /// Read/write scope this surface is bounded to.
    pub read_write_scope: CompanionContinuityScope,
    /// Token of the frozen matrix lane this surface inherits qualification from.
    pub matrix_lane_ref: String,
    /// Downgrade triggers that apply to this surface.
    pub downgrade_triggers: Vec<M5CompanionDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5CompanionRollbackPosture,
}

/// Bounded read/write scope contract for the whole surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionContinuityScopeContract {
    /// The remote-preview-handoff surface is read-only.
    pub remote_preview_read_only: bool,
    /// The collaboration-follow surface is read-only.
    pub collaboration_follow_read_only: bool,
    /// Light remote edit is bounded and requires explicit host approval.
    pub light_remote_edit_bounded_and_host_approved: bool,
    /// The companion never holds an unbounded write authority.
    pub no_unbounded_companion_write: bool,
    /// Collaboration-follow is confined to a host-revocable shared scope.
    pub collaboration_follow_scoped: bool,
    /// The desktop host stays authoritative.
    pub host_authoritative: bool,
    /// No payload bodies cross the export boundary.
    pub no_payload_bodies: bool,
}

/// Stale-state honesty contract for the whole surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionContinuityStaleHonesty {
    /// Every stale item is labeled.
    pub stale_items_labeled: bool,
    /// Every unknown-freshness item is labeled.
    pub unknown_freshness_labeled: bool,
    /// A stale item is never shown as live.
    pub never_show_stale_as_live: bool,
    /// A freshness floor is enforced before an item is shown.
    pub freshness_floor_enforced: bool,
}

/// Local-core continuity guarantee for the whole surface.
///
/// This is the lane's distinct value: a remote preview is a read-only projection of
/// the authoritative local core, a session handoff never strands local work, and a
/// revoked collaboration scope narrows the follow rather than orphaning anything.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionContinuityLocalCoreGuarantee {
    /// The local core stays authoritative throughout.
    pub local_core_authoritative: bool,
    /// A session handoff never strands user-owned local work.
    pub handoff_never_strands_local_work: bool,
    /// A remote preview is a read-only projection, never an authoring surface.
    pub preview_is_read_only_projection: bool,
    /// A revoked collaboration scope narrows the follow honestly.
    pub collaboration_scope_revocable: bool,
}

/// Security and privacy review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionContinuitySecurityReview {
    /// The remote-preview-handoff surface is read-only.
    pub remote_preview_read_only: bool,
    /// The collaboration-follow surface is read-only.
    pub collaboration_follow_read_only: bool,
    /// Light remote edit is bounded and host-approved.
    pub light_remote_edit_bounded_and_host_approved: bool,
    /// No unbounded companion write authority is exposed.
    pub no_unbounded_companion_write: bool,
    /// Collaboration-follow is confined to a host-revocable shared scope.
    pub collaboration_follow_scoped: bool,
    /// The desktop host stays authoritative.
    pub host_stays_authoritative: bool,
    /// User-owned local work is never stranded by a handoff or scope loss.
    pub local_work_never_stranded: bool,
    /// Stale state is labeled rather than hidden.
    pub stale_state_labeled_never_hidden: bool,
    /// Exact desktop handoff is preserved or honestly degraded.
    pub exact_desktop_handoff_preserved: bool,
    /// No payload bodies cross the export boundary.
    pub no_payload_bodies_in_export: bool,
    /// Downgrade narrows the claim rather than hiding the surface.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Every surface discloses local, staged, and provider/admin continuity.
    pub locality_disclosed: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionContinuityConsumerProjection {
    /// Browser companion projects the remote-preview-handoff surface.
    pub browser_companion_shows_remote_preview: bool,
    /// Mobile companion projects the remote-preview-handoff surface.
    pub mobile_companion_shows_remote_preview: bool,
    /// Desktop panel shows the handoff targets.
    pub desktop_panel_shows_handoff_target: bool,
    /// Desktop panel projects the scoped collaboration-follow surface.
    pub desktop_panel_shows_collaboration_follow: bool,
    /// Support export shows scope and freshness state.
    pub support_export_shows_scope_and_freshness: bool,
    /// Diagnostics shows stale and degraded labels.
    pub diagnostics_shows_stale_labels: bool,
    /// Preview / Labs surfaces are visibly labeled when not qualified Stable.
    pub preview_labs_label_for_unqualified_surfaces: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionContinuityProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the surface.
    pub auto_narrow_on_stale: bool,
}

/// Per-surface observation fed to
/// [`CompanionContinuitySurfacePacket::apply_companion_continuity_degradation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompanionContinuityObservation {
    /// True when the companion relay is available.
    pub relay_available: bool,
    /// True when proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when an active desktop host session exists.
    pub host_session_active: bool,
    /// True when workspace and device trust are intact.
    pub trust_intact: bool,
    /// True when the shared collaboration scope is still granted.
    pub collaboration_scope_intact: bool,
    /// True when an upstream frozen matrix lane narrowed.
    pub upstream_matrix_narrowed: bool,
}

/// Constructor input for [`CompanionContinuitySurfacePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompanionContinuitySurfacePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer surfaces that project this packet.
    pub projected_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Per-surface qualification rows.
    pub surface_qualifications: Vec<CompanionContinuitySurfaceQualification>,
    /// Remote-preview-handoff items.
    pub remote_preview: Vec<RemotePreviewItem>,
    /// Light-remote-edit items.
    pub light_remote_edit: Vec<LightRemoteEditItem>,
    /// Collaboration-follow items.
    pub collaboration_follow: Vec<CollaborationFollowItem>,
    /// Bounded read/write scope contract.
    pub scope_contract: CompanionContinuityScopeContract,
    /// Stale-state honesty contract.
    pub stale_state_honesty: CompanionContinuityStaleHonesty,
    /// Local-core continuity guarantee.
    pub continuity_guarantee: CompanionContinuityLocalCoreGuarantee,
    /// Locality disclosure.
    pub locality_disclosure: M5CompanionLocalityDisclosure,
    /// Security review block.
    pub security_review: CompanionContinuitySecurityReview,
    /// Consumer projection block.
    pub consumer_projection: CompanionContinuityConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CompanionContinuityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe companion remote-preview, handoff, light-edit, and
/// collaboration-follow continuity surface packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionContinuitySurfacePacket {
    /// Record kind; must equal [`COMPANION_CONTINUITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`COMPANION_CONTINUITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer surfaces that project this packet.
    pub projected_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Per-surface qualification rows.
    pub surface_qualifications: Vec<CompanionContinuitySurfaceQualification>,
    /// Remote-preview-handoff items.
    pub remote_preview: Vec<RemotePreviewItem>,
    /// Light-remote-edit items.
    pub light_remote_edit: Vec<LightRemoteEditItem>,
    /// Collaboration-follow items.
    pub collaboration_follow: Vec<CollaborationFollowItem>,
    /// Bounded read/write scope contract.
    pub scope_contract: CompanionContinuityScopeContract,
    /// Stale-state honesty contract.
    pub stale_state_honesty: CompanionContinuityStaleHonesty,
    /// Local-core continuity guarantee.
    pub continuity_guarantee: CompanionContinuityLocalCoreGuarantee,
    /// Locality disclosure.
    pub locality_disclosure: M5CompanionLocalityDisclosure,
    /// Security review block.
    pub security_review: CompanionContinuitySecurityReview,
    /// Consumer projection block.
    pub consumer_projection: CompanionContinuityConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CompanionContinuityProofFreshness,
    /// Degraded-state labels currently applied (empty when fully qualified).
    pub degraded_labels: Vec<CompanionContinuityDegradedReason>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl CompanionContinuitySurfacePacket {
    /// Builds a companion continuity surface packet from stable-lane input.
    pub fn new(input: CompanionContinuitySurfacePacketInput) -> Self {
        Self {
            record_kind: COMPANION_CONTINUITY_RECORD_KIND.to_owned(),
            schema_version: COMPANION_CONTINUITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            projected_surfaces: input.projected_surfaces,
            surface_qualifications: input.surface_qualifications,
            remote_preview: input.remote_preview,
            light_remote_edit: input.light_remote_edit,
            collaboration_follow: input.collaboration_follow,
            scope_contract: input.scope_contract,
            stale_state_honesty: input.stale_state_honesty,
            continuity_guarantee: input.continuity_guarantee,
            locality_disclosure: input.locality_disclosure,
            security_review: input.security_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            degraded_labels: Vec::new(),
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Narrows surfaces and downgrades freshness, handoff resolution, and
    /// continuity from a per-surface observation, recording the reasons in
    /// [`Self::degraded_labels`].
    ///
    /// An unavailable relay, stale proof, or narrowed upstream matrix lane narrows
    /// every surface's qualification and rollout stage one step, and an unavailable
    /// relay additionally forces every live or cached item to stale and labels it.
    /// Narrowed trust additionally narrows the light-remote-edit surface; a revoked
    /// collaboration scope narrows the collaboration-follow surface and marks its
    /// items scope-revoked. An inactive host session downgrades the resolution of
    /// every handoff that requires an active host, marks the affected remote-preview
    /// items handoff-unavailable, and narrows the light-remote-edit surface, since a
    /// bounded write can no longer be relayed and applied. Degraded state is
    /// labeled, never hidden, and local work is never stranded.
    pub fn apply_companion_continuity_degradation(
        &mut self,
        observation: &CompanionContinuityObservation,
    ) {
        let mut labels: BTreeSet<CompanionContinuityDegradedReason> =
            self.degraded_labels.iter().copied().collect();

        let surface_adverse = !observation.relay_available
            || !observation.proof_fresh
            || observation.upstream_matrix_narrowed;

        if !observation.relay_available {
            labels.insert(CompanionContinuityDegradedReason::RelayUnavailable);
            if self.force_all_freshness_stale() {
                labels.insert(CompanionContinuityDegradedReason::FreshnessDowngradedToStale);
            }
        }
        if !observation.proof_fresh {
            labels.insert(CompanionContinuityDegradedReason::ProofStale);
        }
        if observation.upstream_matrix_narrowed {
            labels.insert(CompanionContinuityDegradedReason::UpstreamMatrixNarrowed);
        }
        if !observation.trust_intact {
            labels.insert(CompanionContinuityDegradedReason::TrustNarrowed);
        }
        if !observation.collaboration_scope_intact {
            labels.insert(CompanionContinuityDegradedReason::CollaborationScopeRevoked);
            for item in &mut self.collaboration_follow {
                item.follow_scope = CollaborationFollowScope::ScopeRevoked;
            }
        }

        for row in &mut self.surface_qualifications {
            let adverse = surface_adverse
                || (!observation.trust_intact
                    && row.surface == CompanionContinuitySurface::LightRemoteEdit)
                || (!observation.host_session_active
                    && row.surface == CompanionContinuitySurface::LightRemoteEdit)
                || (!observation.collaboration_scope_intact
                    && row.surface == CompanionContinuitySurface::CollaborationFollow);
            if adverse {
                row.qualification = row.qualification.narrowed_one_step();
                row.rollout_stage = row.rollout_stage.narrowed_one_step();
            }
        }

        if !observation.host_session_active {
            labels.insert(CompanionContinuityDegradedReason::HostSessionInactive);
            let mut any_unresolved = false;
            for item in &mut self.remote_preview {
                if item.handoff.requires_active_host
                    && item.handoff.resolution == CompanionHandoffResolution::Exact
                {
                    item.handoff.resolution = CompanionHandoffResolution::Unresolved;
                    item.handoff_continuity = item.handoff_continuity.forced_unavailable();
                    any_unresolved = true;
                }
            }
            for handoff in self
                .light_remote_edit
                .iter_mut()
                .map(|item| &mut item.handoff)
                .chain(
                    self.collaboration_follow
                        .iter_mut()
                        .map(|item| &mut item.handoff),
                )
            {
                if handoff.requires_active_host
                    && handoff.resolution == CompanionHandoffResolution::Exact
                {
                    handoff.resolution = CompanionHandoffResolution::Unresolved;
                    any_unresolved = true;
                }
            }
            if any_unresolved {
                labels.insert(CompanionContinuityDegradedReason::HandoffTargetUnresolved);
            }
        }

        self.degraded_labels = labels.into_iter().collect();
    }

    /// Forces every live/cached item freshness to stale and labels it. Returns
    /// true when at least one item was downgraded.
    fn force_all_freshness_stale(&mut self) -> bool {
        let mut downgraded = false;
        for item in &mut self.remote_preview {
            if item.freshness != item.freshness.forced_stale() {
                item.freshness = item.freshness.forced_stale();
                item.stale_label_shown = true;
                downgraded = true;
            }
        }
        for item in &mut self.light_remote_edit {
            if item.freshness != item.freshness.forced_stale() {
                item.freshness = item.freshness.forced_stale();
                item.stale_label_shown = true;
                downgraded = true;
            }
        }
        for item in &mut self.collaboration_follow {
            if item.freshness != item.freshness.forced_stale() {
                item.freshness = item.freshness.forced_stale();
                item.stale_label_shown = true;
                downgraded = true;
            }
        }
        downgraded
    }

    /// Validates the companion continuity surface invariants.
    pub fn validate(&self) -> Vec<CompanionContinuityViolation> {
        let mut violations = Vec::new();

        if self.record_kind != COMPANION_CONTINUITY_RECORD_KIND {
            violations.push(CompanionContinuityViolation::WrongRecordKind);
        }
        if self.schema_version != COMPANION_CONTINUITY_SCHEMA_VERSION {
            violations.push(CompanionContinuityViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(CompanionContinuityViolation::MissingIdentity);
        }
        if self.projected_surfaces.is_empty() {
            violations.push(CompanionContinuityViolation::ProjectedSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_surface_qualifications(self, &mut violations);
        validate_items(self, &mut violations);
        validate_scope_contract(self, &mut violations);
        validate_stale_state_honesty(self, &mut violations);
        validate_continuity_guarantee(self, &mut violations);
        validate_locality(self, &mut violations);
        validate_security_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("companion continuity packet serializes"),
        ) {
            violations.push(CompanionContinuityViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("companion continuity packet serializes")
    }

    /// Projects the shared M5 secret-boundary state for the companion
    /// session-handoff lane.
    pub fn secret_boundary_states(&self) -> Vec<SecretBoundarySurfaceState> {
        let Some(item) = self.remote_preview.first() else {
            return Vec::new();
        };

        let matrix_row_id = "m5.secret.companion.session_handoff";
        let target_label = format!("Companion handoff {}", item.source_session_ref);
        let health_state = companion_secret_health_state(item.freshness);
        let projection_controls = companion_projection_controls(matrix_row_id);
        let audit_result = secret_boundary_use_audit_result_for_health(health_state);
        let workflows = vec![
            companion_secret_workflow("workflow:companion.follow", "Resume companion follow"),
            companion_secret_workflow("workflow:companion.handoff", "Return to desktop session"),
        ];
        let decline_path = SecretBoundaryDeclinePath {
            decline_label: "Stay read-only on companion".to_owned(),
            still_works_summary:
                "Declining keeps read-only follow state and exact desktop handoff instructions available."
                    .to_owned(),
        };

        vec![SecretBoundarySurfaceState {
            matrix_row_id: matrix_row_id.to_owned(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            secret_access_prompt: SecretBoundarySecretAccessPrompt {
                matrix_row_id: matrix_row_id.to_owned(),
                vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
                requester_label: "Companion session handoff".to_owned(),
                secret_class: SecretBoundarySecretClass::CloudDelegatedIdentity,
                target_workflow_label: target_label.clone(),
                storage_class: SecretBoundaryStorageClass::SessionOnly,
                credential_mode: SecretBoundaryCredentialMode::BrowserHandoff,
                projection_mode: SecretBoundaryProjectionMode::BrowserHandoff,
                lifetime_label: "Companion handoff session".to_owned(),
                expires_at: None,
                dependent_workflows: workflows.clone(),
                decline_path: decline_path.clone(),
            },
            credential_state_row: SecretBoundaryCredentialStateRow {
                matrix_row_id: matrix_row_id.to_owned(),
                vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
                display_label: "Companion handoff credential state".to_owned(),
                secret_class: SecretBoundarySecretClass::CloudDelegatedIdentity,
                source_class: SecretBoundaryCredentialMode::BrowserHandoff,
                target_boundary_label: target_label.clone(),
                storage_class: SecretBoundaryStorageClass::SessionOnly,
                projection_mode: SecretBoundaryProjectionMode::BrowserHandoff,
                health_state,
                expires_at: None,
                rotate_action_label: "Refresh companion handoff".to_owned(),
                revoke_action_label: "Revoke companion handoff".to_owned(),
                test_action_label: "Test companion handoff".to_owned(),
                dependent_workflows: workflows,
                decline_path,
            },
            vault_picker: None,
            delegated_credential_row: Some(SecretBoundaryDelegatedCredentialRow {
                matrix_row_id: matrix_row_id.to_owned(),
                vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
                delegated_use_class: SecretBoundaryDelegatedUseClass::ForwardedLocalCredential,
                target_host_or_workspace_label: target_label.clone(),
                expires_at: None,
                policy_owner_label: "Desktop companion session".to_owned(),
                projection_controls: projection_controls.clone(),
            }),
            consumer_identity_receipt: SecretBoundaryConsumerIdentityReceipt::new(
                format!("{matrix_row_id}:consumer-receipt"),
                matrix_row_id,
                SecretBoundaryActingIdentityClass::DelegatedCredential,
                SecretBoundaryConsumerIdentityClass::CompanionHandoff,
                "Desktop companion session",
                target_label.clone(),
                SecretBoundaryCredentialMode::BrowserHandoff,
                SecretBoundaryProjectionMode::BrowserHandoff,
                SecretBoundaryStorageClass::SessionOnly,
                audit_result,
            ),
            projection_mode_audit: SecretBoundaryProjectionModeAudit::new(
                format!("{matrix_row_id}:projection-audit"),
                matrix_row_id,
                SecretBoundaryActingIdentityClass::DelegatedCredential,
                SecretBoundaryConsumerIdentityClass::CompanionHandoff,
                "Desktop companion session",
                target_label,
                SecretBoundaryProjectionMode::BrowserHandoff,
                audit_result,
                SecretBoundaryRepairOwnerClass::User,
                projection_controls
                    .iter()
                    .map(|control| control.control_class)
                    .collect(),
            ),
            repairable_states: seeded_secret_boundary_repairable_states(matrix_row_id),
            active_repair_state: seeded_secret_boundary_active_repair_state(matrix_row_id, health_state),
            profile_parity_rows: seeded_secret_boundary_profile_parity_rows(matrix_row_id),
            export_safety_banner: SecretBoundaryExportSafetyBanner::standard(
                matrix_row_id,
                "Raw companion handoff tokens, callback payloads, and relay secrets stay excluded from support bundles and handoff descriptors.",
            ),
        }]
    }

    /// Surfaces currently publishable (Stable, Beta, or Preview) and not withheld.
    pub fn publishable_surfaces(
        &self,
    ) -> impl Iterator<Item = &CompanionContinuitySurfaceQualification> {
        self.surface_qualifications.iter().filter(|row| {
            matches!(
                row.qualification,
                M5CompanionQualificationClass::Stable
                    | M5CompanionQualificationClass::Beta
                    | M5CompanionQualificationClass::Preview
            ) && row.rollout_stage != M5CompanionRolloutStage::Withheld
        })
    }

    /// True when every item's handoff resolves to the exact desktop location.
    pub fn all_handoffs_exact(&self) -> bool {
        self.handoffs()
            .all(|handoff| handoff.resolution == CompanionHandoffResolution::Exact)
    }

    /// True when every stale or unknown-freshness item carries a visible label.
    pub fn stale_state_honestly_labeled(&self) -> bool {
        self.remote_preview
            .iter()
            .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .light_remote_edit
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .collaboration_follow
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
    }

    /// True when every item declares its local work preserved.
    pub fn local_work_never_stranded(&self) -> bool {
        self.remote_preview
            .iter()
            .all(|item| item.local_work_preserved)
            && self
                .light_remote_edit
                .iter()
                .all(|item| item.local_work_preserved)
            && self
                .collaboration_follow
                .iter()
                .all(|item| item.local_work_preserved)
    }

    /// Iterates every handoff across all three surfaces, in surface order.
    pub fn handoffs(&self) -> impl Iterator<Item = &CompanionDesktopHandoff> {
        self.remote_preview
            .iter()
            .map(|item| &item.handoff)
            .chain(self.light_remote_edit.iter().map(|item| &item.handoff))
            .chain(self.collaboration_follow.iter().map(|item| &item.handoff))
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# Companion Remote-Preview, Session-Handoff, Light-Remote-Edit, and Collaboration-Follow Continuity\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Surfaces: {} | Remote-preview: {} | Light-remote-edit: {} | Collaboration-follow: {}\n",
            self.surface_qualifications.len(),
            self.remote_preview.len(),
            self.light_remote_edit.len(),
            self.collaboration_follow.len(),
        ));
        out.push_str(&format!(
            "- Exact handoff for every item: {}\n",
            if self.all_handoffs_exact() {
                "yes"
            } else {
                "no"
            }
        ));
        out.push_str(&format!(
            "- Stale state honestly labeled: {}\n",
            if self.stale_state_honestly_labeled() {
                "yes"
            } else {
                "no"
            }
        ));
        out.push_str(&format!(
            "- Local work never stranded: {}\n",
            if self.local_work_never_stranded() {
                "yes"
            } else {
                "no"
            }
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        if self.degraded_labels.is_empty() {
            out.push_str("- Degraded: none\n");
        } else {
            let labels = self
                .degraded_labels
                .iter()
                .map(|reason| reason.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!("- Degraded: {labels}\n"));
        }

        out.push_str("\n## Surfaces\n\n");
        for row in &self.surface_qualifications {
            out.push_str(&format!(
                "- **{}**: `{}` / `{}` [{}] (matrix lane `{}`)\n",
                row.surface.as_str(),
                row.qualification.as_str(),
                row.rollout_stage.as_str(),
                row.read_write_scope.as_str(),
                row.matrix_lane_ref,
            ));
        }

        out.push_str("\n## Remote-preview-handoff\n\n");
        for item in &self.remote_preview {
            out.push_str(&format!(
                "- `{}` [{}] {} — {} ({}) → `{}` ({})\n",
                item.item_id,
                item.preview_kind.as_str(),
                item.handoff_continuity.as_str(),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Light-remote-edit\n\n");
        for item in &self.light_remote_edit {
            out.push_str(&format!(
                "- `{}` [{}] {} ({}) — {} → `{}` ({})\n",
                item.item_id,
                item.edit_kind.as_str(),
                item.edit_state.as_str(),
                item.read_write_scope.as_str(),
                item.summary,
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Collaboration-follow\n\n");
        for item in &self.collaboration_follow {
            out.push_str(&format!(
                "- `{}` [{}/{}] {} ({}) → `{}` ({})\n",
                item.item_id,
                item.collaboration_role.as_str(),
                item.follow_scope.as_str(),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out
    }
}

fn companion_secret_workflow(
    workflow_ref: impl Into<String>,
    workflow_label: impl Into<String>,
) -> SecretBoundaryWorkflowDependency {
    SecretBoundaryWorkflowDependency {
        workflow_ref: workflow_ref.into(),
        workflow_label: workflow_label.into(),
    }
}

fn companion_secret_health_state(
    freshness: CompanionContinuityFreshness,
) -> SecretBoundaryHealthStateClass {
    match freshness {
        CompanionContinuityFreshness::Live | CompanionContinuityFreshness::Cached => {
            SecretBoundaryHealthStateClass::Healthy
        }
        CompanionContinuityFreshness::Stale => SecretBoundaryHealthStateClass::Expired,
        CompanionContinuityFreshness::Unknown => SecretBoundaryHealthStateClass::Missing,
    }
}

fn companion_projection_controls(matrix_row_id: &str) -> Vec<SecretBoundaryProjectionControl> {
    let local_safe_note =
        "Read-only follow state and exact desktop handoff instructions remain available.";
    vec![
        SecretBoundaryProjectionControl::new(
            matrix_row_id,
            SecretBoundaryProjectionControlClass::PauseForwarding,
            "Pause companion credential forwarding",
            local_safe_note,
        ),
        SecretBoundaryProjectionControl::new(
            matrix_row_id,
            SecretBoundaryProjectionControlClass::StopUsingSecret,
            "Stop using companion handoff secret",
            local_safe_note,
        ),
        SecretBoundaryProjectionControl::new(
            matrix_row_id,
            SecretBoundaryProjectionControlClass::DropDelegatedIdentity,
            "Drop companion delegated identity",
            local_safe_note,
        ),
    ]
}

/// Errors emitted when reading the checked-in companion continuity export.
#[derive(Debug)]
pub enum CompanionContinuityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CompanionContinuityViolation>),
}

impl fmt::Display for CompanionContinuityArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "companion continuity export parse failed: {error}"
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
                    "companion continuity export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for CompanionContinuityArtifactError {}

/// Validation failures emitted by [`CompanionContinuitySurfacePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompanionContinuityViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Projected surfaces list is empty.
    ProjectedSurfacesMissing,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required surface qualification row is missing.
    RequiredSurfaceMissing,
    /// A surface row's matrix lane ref does not match its surface.
    SurfaceLaneMismatch,
    /// A surface row's read/write scope does not match its bounded scope.
    SurfaceScopeMismatch,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface has no content items.
    SurfaceContentMissing,
    /// A read-only surface item is not marked read-only.
    ReadOnlyScopeViolated,
    /// A light-edit item is not bounded or does not require host approval.
    LightEditUnbounded,
    /// A collaboration-follow item is not marked scope-bounded.
    CollaborationFollowUnscoped,
    /// An item does not preserve user-owned local work.
    LocalWorkStranded,
    /// An item is missing identity or a redacted body, or has a payload-like body.
    ItemIncomplete,
    /// A stale or unknown-freshness item is not labeled.
    StaleStateNotLabeled,
    /// An item's handoff is missing its deep-link ref.
    HandoffRefMissing,
    /// The bounded read/write scope contract is not fully satisfied.
    ScopeContractIncomplete,
    /// The stale-state honesty contract is not fully satisfied.
    StaleStateHonestyIncomplete,
    /// The local-core continuity guarantee is not fully satisfied.
    ContinuityGuaranteeIncomplete,
    /// The locality disclosure is incomplete.
    LocalityDisclosureIncomplete,
    /// Security review does not satisfy required invariants.
    SecurityReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl CompanionContinuityViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::ProjectedSurfacesMissing => "projected_surfaces_missing",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceLaneMismatch => "surface_lane_mismatch",
            Self::SurfaceScopeMismatch => "surface_scope_mismatch",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::SurfaceContentMissing => "surface_content_missing",
            Self::ReadOnlyScopeViolated => "read_only_scope_violated",
            Self::LightEditUnbounded => "light_edit_unbounded",
            Self::CollaborationFollowUnscoped => "collaboration_follow_unscoped",
            Self::LocalWorkStranded => "local_work_stranded",
            Self::ItemIncomplete => "item_incomplete",
            Self::StaleStateNotLabeled => "stale_state_not_labeled",
            Self::HandoffRefMissing => "handoff_ref_missing",
            Self::ScopeContractIncomplete => "scope_contract_incomplete",
            Self::StaleStateHonestyIncomplete => "stale_state_honesty_incomplete",
            Self::ContinuityGuaranteeIncomplete => "continuity_guarantee_incomplete",
            Self::LocalityDisclosureIncomplete => "locality_disclosure_incomplete",
            Self::SecurityReviewIncomplete => "security_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable companion continuity surface export.
///
/// This is the canonical reader: a browser/mobile companion, the desktop panel,
/// diagnostics, support-export, or Help/About surface calls it to ingest the packet
/// rather than cloning status text.
///
/// # Errors
///
/// Returns [`CompanionContinuityArtifactError`] when the checked-in support export
/// fails to parse or fails validation.
pub fn current_stable_companion_continuity_surface_export(
) -> Result<CompanionContinuitySurfacePacket, CompanionContinuityArtifactError> {
    let packet: CompanionContinuitySurfacePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/companion/m5/add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio/support_export.json"
    )))
    .map_err(CompanionContinuityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CompanionContinuityArtifactError::Validation(violations))
    }
}

/// Canonical source contract refs that every continuity export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        COMPANION_CONTINUITY_SCHEMA_REF.to_owned(),
        COMPANION_CONTINUITY_DOC_REF.to_owned(),
        M5_COMPANION_SURFACE_CONTRACT_REF.to_owned(),
        M5_COMPANION_QUALIFICATION_REF.to_owned(),
        M5_COMPANION_BOUNDARY_MANIFEST_REF.to_owned(),
        M5_COMPANION_MATRIX_SCHEMA_REF.to_owned(),
    ]
}

/// Canonical bounded read/write scope contract with every guarantee satisfied.
pub fn canonical_scope_contract() -> CompanionContinuityScopeContract {
    CompanionContinuityScopeContract {
        remote_preview_read_only: true,
        collaboration_follow_read_only: true,
        light_remote_edit_bounded_and_host_approved: true,
        no_unbounded_companion_write: true,
        collaboration_follow_scoped: true,
        host_authoritative: true,
        no_payload_bodies: true,
    }
}

/// Canonical stale-state honesty contract with every guarantee satisfied.
pub fn canonical_stale_state_honesty() -> CompanionContinuityStaleHonesty {
    CompanionContinuityStaleHonesty {
        stale_items_labeled: true,
        unknown_freshness_labeled: true,
        never_show_stale_as_live: true,
        freshness_floor_enforced: true,
    }
}

/// Canonical local-core continuity guarantee with every guarantee satisfied.
pub fn canonical_continuity_guarantee() -> CompanionContinuityLocalCoreGuarantee {
    CompanionContinuityLocalCoreGuarantee {
        local_core_authoritative: true,
        handoff_never_strands_local_work: true,
        preview_is_read_only_projection: true,
        collaboration_scope_revocable: true,
    }
}

/// Canonical security review block with every invariant satisfied.
pub fn canonical_security_review() -> CompanionContinuitySecurityReview {
    CompanionContinuitySecurityReview {
        remote_preview_read_only: true,
        collaboration_follow_read_only: true,
        light_remote_edit_bounded_and_host_approved: true,
        no_unbounded_companion_write: true,
        collaboration_follow_scoped: true,
        host_stays_authoritative: true,
        local_work_never_stranded: true,
        stale_state_labeled_never_hidden: true,
        exact_desktop_handoff_preserved: true,
        no_payload_bodies_in_export: true,
        downgrade_narrows_instead_of_hides: true,
        locality_disclosed: true,
    }
}

/// Canonical consumer projection block with every surface projecting truth.
pub fn canonical_consumer_projection() -> CompanionContinuityConsumerProjection {
    CompanionContinuityConsumerProjection {
        browser_companion_shows_remote_preview: true,
        mobile_companion_shows_remote_preview: true,
        desktop_panel_shows_handoff_target: true,
        desktop_panel_shows_collaboration_follow: true,
        support_export_shows_scope_and_freshness: true,
        diagnostics_shows_stale_labels: true,
        preview_labs_label_for_unqualified_surfaces: true,
    }
}

/// Canonical per-surface qualification rows, inherited from the frozen matrix.
pub fn canonical_surface_qualifications() -> Vec<CompanionContinuitySurfaceQualification> {
    use M5CompanionDowngradeTrigger as Trigger;
    use M5CompanionQualificationClass as Qual;
    use M5CompanionRollbackPosture as Rollback;
    use M5CompanionRolloutStage as Stage;

    vec![
        CompanionContinuitySurfaceQualification {
            surface: CompanionContinuitySurface::RemotePreviewHandoff,
            qualification: Qual::Beta,
            rollout_stage: Stage::StagedRollout,
            read_write_scope: CompanionContinuitySurface::RemotePreviewHandoff.bounded_scope(),
            matrix_lane_ref: CompanionContinuitySurface::RemotePreviewHandoff
                .matrix_lane()
                .as_str()
                .to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::TrustNarrowing,
                Trigger::CompanionScopeExpansionUnqualified,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::CompanionReadOnlyNarrowScope,
        },
        CompanionContinuitySurfaceQualification {
            surface: CompanionContinuitySurface::LightRemoteEdit,
            qualification: Qual::Preview,
            rollout_stage: Stage::EarlyAccess,
            read_write_scope: CompanionContinuitySurface::LightRemoteEdit.bounded_scope(),
            matrix_lane_ref: CompanionContinuitySurface::LightRemoteEdit
                .matrix_lane()
                .as_str()
                .to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::TrustNarrowing,
                Trigger::CompanionScopeExpansionUnqualified,
                Trigger::PolicyBlocked,
            ],
            rollback_posture: Rollback::StagedReversibleViaRollout,
        },
        CompanionContinuitySurfaceQualification {
            surface: CompanionContinuitySurface::CollaborationFollow,
            qualification: Qual::Stable,
            rollout_stage: Stage::GeneralAvailability,
            read_write_scope: CompanionContinuitySurface::CollaborationFollow.bounded_scope(),
            matrix_lane_ref: CompanionContinuitySurface::CollaborationFollow
                .matrix_lane()
                .as_str()
                .to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::TrustNarrowing,
                Trigger::CompanionScopeExpansionUnqualified,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::CompanionReadOnlyNarrowScope,
        },
    ]
}

/// Canonical locality disclosure for the continuity surface.
pub fn canonical_locality_disclosure() -> M5CompanionLocalityDisclosure {
    M5CompanionLocalityDisclosure {
        stays_local:
            "Session state, edit targets, and collaboration scopes are owned by the local core and stay inspectable offline; a remote preview never holds authoritative state."
                .to_owned(),
        staged:
            "Remote preview streaming, session handoff, bounded light remote edit, and scoped collaboration-follow roll out per cohort and capability gate."
                .to_owned(),
        requires_provider_or_admin_continuity:
            "Live preview, exact handoff, relaying a bounded edit for host approval, and a shared collaboration scope require the companion relay and an active host session; the local core never depends on them to function and never strands local work."
                .to_owned(),
    }
}

fn handoff(
    target: CompanionHandoffTarget,
    deep_link_ref: &str,
    requires_active_host: bool,
) -> CompanionDesktopHandoff {
    CompanionDesktopHandoff {
        target,
        deep_link_ref: deep_link_ref.to_owned(),
        resolution: CompanionHandoffResolution::Exact,
        requires_active_host,
    }
}

/// Canonical read-only remote-preview-handoff items.
pub fn canonical_remote_preview() -> Vec<RemotePreviewItem> {
    use CompanionContinuityFreshness as Fresh;
    use CompanionHandoffTarget as Target;
    use RemotePreviewKind as Kind;
    use SessionHandoffContinuity as Continuity;

    let scope = CompanionContinuityScope::ReadOnly;
    vec![
        RemotePreviewItem {
            item_id: "preview:editor:0001".to_owned(),
            preview_kind: Kind::EditorPreview,
            handoff_continuity: Continuity::LocalAuthoritative,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            local_work_preserved: true,
            summary: "Remote preview of the active editor on the host workspace".to_owned(),
            source_session_ref: "session:editor:0001".to_owned(),
            handoff: handoff(
                Target::FileLocation,
                "handoff:file-location:preview-0001",
                true,
            ),
        },
        RemotePreviewItem {
            item_id: "preview:agent:0002".to_owned(),
            preview_kind: Kind::AgentRunPreview,
            handoff_continuity: Continuity::HandoffStaged,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            local_work_preserved: true,
            summary: "Remote preview of a running agent session staged for handoff".to_owned(),
            source_session_ref: "session:agent:0002".to_owned(),
            handoff: handoff(
                Target::AgentSession,
                "handoff:agent-session:preview-0002",
                true,
            ),
        },
        RemotePreviewItem {
            item_id: "preview:terminal:0003".to_owned(),
            preview_kind: Kind::TerminalPreview,
            handoff_continuity: Continuity::HandoffResumed,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            local_work_preserved: true,
            summary: "Resumed terminal preview from a cached handoff".to_owned(),
            source_session_ref: "session:terminal:0003".to_owned(),
            handoff: handoff(
                Target::FileLocation,
                "handoff:file-location:preview-0003",
                true,
            ),
        },
    ]
}

/// Canonical bounded, host-approved light-remote-edit items.
pub fn canonical_light_remote_edit() -> Vec<LightRemoteEditItem> {
    use CompanionContinuityFreshness as Fresh;
    use CompanionHandoffTarget as Target;
    use RemoteEditKind as Kind;
    use RemoteEditState as EditState;

    let scope = CompanionContinuityScope::BoundedWriteRelayedToHost;
    vec![
        LightRemoteEditItem {
            item_id: "edit:0001".to_owned(),
            edit_kind: Kind::TextTouchUp,
            edit_state: EditState::AwaitingHostApproval,
            write_bound_summary:
                "Single-line text touch-up within the open file; no structural edits".to_owned(),
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            requires_host_approval: true,
            local_work_preserved: true,
            summary: "Bounded text touch-up relayed for host approval".to_owned(),
            target_ref: "target:file-location:edit-0001".to_owned(),
            handoff: handoff(
                Target::FileLocation,
                "handoff:file-location:edit-0001",
                true,
            ),
        },
        LightRemoteEditItem {
            item_id: "edit:0002".to_owned(),
            edit_kind: Kind::CommentReply,
            edit_state: EditState::RelayedForPreview,
            write_bound_summary: "Reply text only on an existing comment thread".to_owned(),
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            requires_host_approval: true,
            local_work_preserved: true,
            summary: "Comment reply relayed to the host for preview".to_owned(),
            target_ref: "target:review-panel:edit-0002".to_owned(),
            handoff: handoff(Target::ReviewPanel, "handoff:review-panel:edit-0002", true),
        },
    ]
}

/// Canonical read-only, scoped collaboration-follow items.
pub fn canonical_collaboration_follow() -> Vec<CollaborationFollowItem> {
    use CollaborationFollowScope as FollowScope;
    use CollaborationRole as Role;
    use CompanionContinuityFreshness as Fresh;
    use CompanionHandoffTarget as Target;

    let scope = CompanionContinuityScope::ReadOnly;
    vec![
        CollaborationFollowItem {
            item_id: "collab:0001".to_owned(),
            collaboration_role: Role::Driver,
            follow_scope: FollowScope::SharedSessionScope,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            scope_bounded: true,
            local_work_preserved: true,
            summary: "Following the driver within a shared session scope".to_owned(),
            collaborator_ref: "collaborator:0001".to_owned(),
            handoff: handoff(
                Target::ReviewPanel,
                "handoff:review-panel:collab-0001",
                false,
            ),
        },
        CollaborationFollowItem {
            item_id: "collab:0002".to_owned(),
            collaboration_role: Role::Reviewer,
            follow_scope: FollowScope::SharedReviewScope,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            scope_bounded: true,
            local_work_preserved: true,
            summary: "Following a reviewer within a shared review scope".to_owned(),
            collaborator_ref: "collaborator:0002".to_owned(),
            handoff: handoff(
                Target::ReviewPanel,
                "handoff:review-panel:collab-0002",
                false,
            ),
        },
        CollaborationFollowItem {
            item_id: "collab:0003".to_owned(),
            collaboration_role: Role::Observer,
            follow_scope: FollowScope::SharedFileScope,
            freshness: Fresh::Stale,
            read_write_scope: scope,
            stale_label_shown: true,
            scope_bounded: true,
            local_work_preserved: true,
            summary: "Observer follow with a stale shared-file scope".to_owned(),
            collaborator_ref: "collaborator:0003".to_owned(),
            handoff: handoff(
                Target::FileLocation,
                "handoff:file-location:collab-0003",
                false,
            ),
        },
    ]
}

/// Builds the canonical companion continuity surface packet.
///
/// This is the first consumer: it mints the surface the checked-in support export
/// and Markdown summary are generated from, so the artifact never drifts from the
/// typed surface, item, scope, and freshness definitions.
pub fn canonical_companion_continuity_surface(
    packet_id: String,
    surface_label: String,
    minted_at: String,
    proof_freshness: CompanionContinuityProofFreshness,
) -> CompanionContinuitySurfacePacket {
    CompanionContinuitySurfacePacket::new(CompanionContinuitySurfacePacketInput {
        packet_id,
        surface_label,
        projected_surfaces: vec![
            M5CompanionConsumerSurface::BrowserCompanion,
            M5CompanionConsumerSurface::MobileCompanion,
            M5CompanionConsumerSurface::DesktopCompanionPanel,
            M5CompanionConsumerSurface::SupportExport,
            M5CompanionConsumerSurface::Diagnostics,
            M5CompanionConsumerSurface::HelpAbout,
        ],
        surface_qualifications: canonical_surface_qualifications(),
        remote_preview: canonical_remote_preview(),
        light_remote_edit: canonical_light_remote_edit(),
        collaboration_follow: canonical_collaboration_follow(),
        scope_contract: canonical_scope_contract(),
        stale_state_honesty: canonical_stale_state_honesty(),
        continuity_guarantee: canonical_continuity_guarantee(),
        locality_disclosure: canonical_locality_disclosure(),
        security_review: canonical_security_review(),
        consumer_projection: canonical_consumer_projection(),
        proof_freshness,
        source_contract_refs: canonical_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at,
    })
}

fn validate_source_contracts(
    packet: &CompanionContinuitySurfacePacket,
    violations: &mut Vec<CompanionContinuityViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        COMPANION_CONTINUITY_SCHEMA_REF,
        COMPANION_CONTINUITY_DOC_REF,
        M5_COMPANION_SURFACE_CONTRACT_REF,
        M5_COMPANION_MATRIX_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(CompanionContinuityViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_surface_qualifications(
    packet: &CompanionContinuitySurfacePacket,
    violations: &mut Vec<CompanionContinuityViolation>,
) {
    let present: BTreeSet<CompanionContinuitySurface> = packet
        .surface_qualifications
        .iter()
        .map(|row| row.surface)
        .collect();
    for required in CompanionContinuitySurface::ALL {
        if !present.contains(&required) {
            violations.push(CompanionContinuityViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_qualifications {
        if row.matrix_lane_ref != row.surface.matrix_lane().as_str() {
            violations.push(CompanionContinuityViolation::SurfaceLaneMismatch);
        }
        if row.read_write_scope != row.surface.bounded_scope() {
            violations.push(CompanionContinuityViolation::SurfaceScopeMismatch);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(CompanionContinuityViolation::SurfaceRowIncomplete);
        }
    }
}

fn validate_items(
    packet: &CompanionContinuitySurfacePacket,
    violations: &mut Vec<CompanionContinuityViolation>,
) {
    if packet.remote_preview.is_empty()
        || packet.light_remote_edit.is_empty()
        || packet.collaboration_follow.is_empty()
    {
        violations.push(CompanionContinuityViolation::SurfaceContentMissing);
    }

    for item in &packet.remote_preview {
        if item.read_write_scope != CompanionContinuityScope::ReadOnly {
            violations.push(CompanionContinuityViolation::ReadOnlyScopeViolated);
        }
        if !item.local_work_preserved {
            violations.push(CompanionContinuityViolation::LocalWorkStranded);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.source_session_ref.trim().is_empty()
        {
            violations.push(CompanionContinuityViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.light_remote_edit {
        if item.read_write_scope != CompanionContinuityScope::BoundedWriteRelayedToHost
            || !item.requires_host_approval
            || item.write_bound_summary.trim().is_empty()
        {
            violations.push(CompanionContinuityViolation::LightEditUnbounded);
        }
        if !item.local_work_preserved {
            violations.push(CompanionContinuityViolation::LocalWorkStranded);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.target_ref.trim().is_empty()
        {
            violations.push(CompanionContinuityViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.collaboration_follow {
        if item.read_write_scope != CompanionContinuityScope::ReadOnly {
            violations.push(CompanionContinuityViolation::ReadOnlyScopeViolated);
        }
        if !item.scope_bounded {
            violations.push(CompanionContinuityViolation::CollaborationFollowUnscoped);
        }
        if !item.local_work_preserved {
            violations.push(CompanionContinuityViolation::LocalWorkStranded);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.collaborator_ref.trim().is_empty()
        {
            violations.push(CompanionContinuityViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }
}

fn validate_freshness_label(
    freshness: CompanionContinuityFreshness,
    stale_label_shown: bool,
    violations: &mut Vec<CompanionContinuityViolation>,
) {
    if freshness.requires_label() && !stale_label_shown {
        violations.push(CompanionContinuityViolation::StaleStateNotLabeled);
    }
}

fn validate_handoff(
    handoff: &CompanionDesktopHandoff,
    violations: &mut Vec<CompanionContinuityViolation>,
) {
    if handoff.deep_link_ref.trim().is_empty() {
        violations.push(CompanionContinuityViolation::HandoffRefMissing);
    }
}

fn validate_scope_contract(
    packet: &CompanionContinuitySurfacePacket,
    violations: &mut Vec<CompanionContinuityViolation>,
) {
    let contract = &packet.scope_contract;
    for ok in [
        contract.remote_preview_read_only,
        contract.collaboration_follow_read_only,
        contract.light_remote_edit_bounded_and_host_approved,
        contract.no_unbounded_companion_write,
        contract.collaboration_follow_scoped,
        contract.host_authoritative,
        contract.no_payload_bodies,
    ] {
        if !ok {
            violations.push(CompanionContinuityViolation::ScopeContractIncomplete);
            return;
        }
    }
}

fn validate_stale_state_honesty(
    packet: &CompanionContinuitySurfacePacket,
    violations: &mut Vec<CompanionContinuityViolation>,
) {
    let honesty = &packet.stale_state_honesty;
    for ok in [
        honesty.stale_items_labeled,
        honesty.unknown_freshness_labeled,
        honesty.never_show_stale_as_live,
        honesty.freshness_floor_enforced,
    ] {
        if !ok {
            violations.push(CompanionContinuityViolation::StaleStateHonestyIncomplete);
            return;
        }
    }
}

fn validate_continuity_guarantee(
    packet: &CompanionContinuitySurfacePacket,
    violations: &mut Vec<CompanionContinuityViolation>,
) {
    let guarantee = &packet.continuity_guarantee;
    for ok in [
        guarantee.local_core_authoritative,
        guarantee.handoff_never_strands_local_work,
        guarantee.preview_is_read_only_projection,
        guarantee.collaboration_scope_revocable,
    ] {
        if !ok {
            violations.push(CompanionContinuityViolation::ContinuityGuaranteeIncomplete);
            return;
        }
    }
}

fn validate_locality(
    packet: &CompanionContinuitySurfacePacket,
    violations: &mut Vec<CompanionContinuityViolation>,
) {
    let locality = &packet.locality_disclosure;
    if locality.stays_local.trim().is_empty()
        || locality.staged.trim().is_empty()
        || locality
            .requires_provider_or_admin_continuity
            .trim()
            .is_empty()
    {
        violations.push(CompanionContinuityViolation::LocalityDisclosureIncomplete);
    }
}

fn validate_security_review(
    packet: &CompanionContinuitySurfacePacket,
    violations: &mut Vec<CompanionContinuityViolation>,
) {
    let review = &packet.security_review;
    for ok in [
        review.remote_preview_read_only,
        review.collaboration_follow_read_only,
        review.light_remote_edit_bounded_and_host_approved,
        review.no_unbounded_companion_write,
        review.collaboration_follow_scoped,
        review.host_stays_authoritative,
        review.local_work_never_stranded,
        review.stale_state_labeled_never_hidden,
        review.exact_desktop_handoff_preserved,
        review.no_payload_bodies_in_export,
        review.downgrade_narrows_instead_of_hides,
        review.locality_disclosed,
    ] {
        if !ok {
            violations.push(CompanionContinuityViolation::SecurityReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &CompanionContinuitySurfacePacket,
    violations: &mut Vec<CompanionContinuityViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.browser_companion_shows_remote_preview,
        projection.mobile_companion_shows_remote_preview,
        projection.desktop_panel_shows_handoff_target,
        projection.desktop_panel_shows_collaboration_follow,
        projection.support_export_shows_scope_and_freshness,
        projection.diagnostics_shows_stale_labels,
        projection.preview_labs_label_for_unqualified_surfaces,
    ] {
        if !ok {
            violations.push(CompanionContinuityViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &CompanionContinuitySurfacePacket,
    violations: &mut Vec<CompanionContinuityViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(CompanionContinuityViolation::ProofFreshnessIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
