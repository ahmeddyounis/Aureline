//! Companion session-follow and incident-awareness surfaces with bounded
//! read/write scope and stale-state honesty.
//!
//! This module owns the export-safe truth packet for the companion surfaces that
//! let a browser or mobile companion *follow* an active desktop session and stay
//! *aware* of incidents, plus the single bounded light-edit surface that may write
//! at all. It binds three surfaces — session-follow, incident-awareness, and
//! bounded light-edit — to the frozen M5 companion-matrix lanes that qualify them,
//! and gives every item an exact [`CompanionDesktopHandoff`] (reused from
//! [`crate::companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff`])
//! so a companion tap resumes the precise host context.
//!
//! Two invariants make these surfaces safe to ship. First, **bounded read/write
//! scope**: the session-follow and incident-awareness surfaces are strictly
//! read-only; only the bounded light-edit surface may write, and its writes are
//! capped, relayed to the host, and require explicit host approval. There is no
//! unbounded companion authoring. Second, **stale-state honesty**: every item
//! carries a [`CompanionFreshnessState`], stale or unknown freshness is always
//! labeled, and a degraded item is never shown as live.
//!
//! The packet reuses the matrix vocabulary from
//! [`crate::freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes`]
//! ([`M5CompanionQualificationClass`], [`M5CompanionRolloutStage`],
//! [`M5CompanionDowngradeTrigger`], [`M5CompanionRollbackPosture`],
//! [`M5CompanionLocalityDisclosure`], [`M5CompanionConsumerSurface`]) instead of
//! inventing parallel terms, and each surface row records the matrix lane it
//! inherits qualification from.
//!
//! [`CompanionScopeSurfacePacket::apply_companion_scope_degradation`] narrows
//! surfaces and downgrades freshness and handoff resolution from a per-surface
//! observation — when the relay is unavailable, proof is stale, the host session
//! is inactive, trust narrowed, incident attribution was lost, or an upstream
//! matrix lane narrowed — so CI or release tooling degrades the surface honestly
//! rather than show fresh follow state or an exact handoff that no longer resolves.
//! Degraded state is labeled, never hidden.
//!
//! [`canonical_companion_scope_surface`] builds the surface and
//! [`current_stable_companion_scope_surface_export`] reads and validates the
//! checked-in support export, so browser/mobile companions, the incident
//! workspace, the desktop panel, diagnostics, support exports, and Help/About
//! ingest the packet rather than cloning status text. Credential bodies, raw
//! provider payloads, and raw session or incident bodies stay outside this
//! boundary.
//!
//! The boundary schema is
//! [`schemas/companion/ship-session-follow-and-incident-awareness-surfaces-with-bounded-read-write-scope-and-stale-state-honesty.schema.json`](../../../../schemas/companion/ship-session-follow-and-incident-awareness-surfaces-with-bounded-read-write-scope-and-stale-state-honesty.schema.json).
//! The contract doc is
//! [`docs/companion/m5/ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty.md`](../../../../docs/companion/m5/ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty.md).
//! The protected fixture directory is
//! [`fixtures/companion/m5/ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty/`](../../../../fixtures/companion/m5/ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff::{
    CompanionDesktopHandoff, CompanionHandoffResolution, CompanionHandoffTarget,
};
use crate::freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes::{
    M5CompanionConsumerSurface, M5CompanionDowngradeTrigger, M5CompanionLocalityDisclosure,
    M5CompanionMatrixLane, M5CompanionQualificationClass, M5CompanionRollbackPosture,
    M5CompanionRolloutStage, M5_COMPANION_BOUNDARY_MANIFEST_REF, M5_COMPANION_MATRIX_SCHEMA_REF,
    M5_COMPANION_QUALIFICATION_REF, M5_COMPANION_SURFACE_CONTRACT_REF,
    M5_INCIDENT_WORKSPACE_CONTRACT_REF,
};

/// Stable record-kind tag carried by [`CompanionScopeSurfacePacket`].
pub const COMPANION_SCOPE_RECORD_KIND: &str =
    "ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty";

/// Schema version for companion scope surface records.
pub const COMPANION_SCOPE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const COMPANION_SCOPE_SCHEMA_REF: &str =
    "schemas/companion/ship-session-follow-and-incident-awareness-surfaces-with-bounded-read-write-scope-and-stale-state-honesty.schema.json";

/// Repo-relative path of the companion scope surface contract doc.
pub const COMPANION_SCOPE_DOC_REF: &str =
    "docs/companion/m5/ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty.md";

/// Repo-relative path of the protected fixture directory.
pub const COMPANION_SCOPE_FIXTURE_DIR: &str =
    "fixtures/companion/m5/ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty";

/// Repo-relative path of the checked support-export artifact.
pub const COMPANION_SCOPE_ARTIFACT_REF: &str =
    "artifacts/companion/m5/ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const COMPANION_SCOPE_SUMMARY_REF: &str =
    "artifacts/companion/m5/ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty.md";

/// One of the three companion scope surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionScopeSurface {
    /// Read-only follow of an active desktop session.
    SessionFollow,
    /// Read-only awareness of incidents raised on the host.
    IncidentAwareness,
    /// The single bounded, host-approved light-edit surface.
    BoundedLightEdit,
}

impl CompanionScopeSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::SessionFollow,
        Self::IncidentAwareness,
        Self::BoundedLightEdit,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionFollow => "session_follow",
            Self::IncidentAwareness => "incident_awareness",
            Self::BoundedLightEdit => "bounded_light_edit",
        }
    }

    /// Frozen M5 companion-matrix lane this surface inherits qualification from.
    pub const fn matrix_lane(self) -> M5CompanionMatrixLane {
        match self {
            Self::SessionFollow => M5CompanionMatrixLane::CompanionSessionFollow,
            Self::IncidentAwareness => M5CompanionMatrixLane::IncidentWorkspace,
            Self::BoundedLightEdit => M5CompanionMatrixLane::CompanionLightEdit,
        }
    }

    /// Read/write scope this surface is bounded to.
    ///
    /// Session-follow and incident-awareness are strictly read-only; only the
    /// bounded light-edit surface may write, and only via a host-approved relay.
    pub const fn bounded_scope(self) -> CompanionReadWriteScope {
        match self {
            Self::SessionFollow | Self::IncidentAwareness => CompanionReadWriteScope::ReadOnly,
            Self::BoundedLightEdit => CompanionReadWriteScope::BoundedWriteRelayedToHost,
        }
    }
}

/// Bounded read/write scope a companion surface or item is allowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionReadWriteScope {
    /// Strictly read-only: the companion never mutates host state.
    ReadOnly,
    /// Bounded write relayed to the host for preview and explicit approval.
    BoundedWriteRelayedToHost,
}

impl CompanionReadWriteScope {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::BoundedWriteRelayedToHost => "bounded_write_relayed_to_host",
        }
    }
}

/// Freshness state of a followed-session, incident, or light-edit item.
///
/// Stale-state honesty requires that [`Self::Stale`] and [`Self::Unknown`] items
/// are always labeled, and that a degraded item is never re-shown as
/// [`Self::Live`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionFreshnessState {
    /// Streaming live from the local core via the relay.
    Live,
    /// Last-known cached value within its freshness window.
    Cached,
    /// Stale beyond its freshness window.
    Stale,
    /// Freshness could not be determined.
    Unknown,
}

impl CompanionFreshnessState {
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

/// Kind of desktop session a companion is following.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionFollowKind {
    /// An active editor session.
    ActiveEditor,
    /// A running agent session.
    AgentRun,
    /// A terminal output stream.
    TerminalStream,
    /// A debug session.
    DebugSession,
    /// A review session.
    ReviewSession,
}

impl SessionFollowKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveEditor => "active_editor",
            Self::AgentRun => "agent_run",
            Self::TerminalStream => "terminal_stream",
            Self::DebugSession => "debug_session",
            Self::ReviewSession => "review_session",
        }
    }
}

/// Follow state of a session-follow item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionFollowState {
    /// Actively following the live host session.
    Following,
    /// Following is paused by the user.
    Paused,
    /// Detached from the host session.
    Detached,
    /// The host session is inactive and cannot be followed.
    HostInactive,
}

impl SessionFollowState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Following => "following",
            Self::Paused => "paused",
            Self::Detached => "detached",
            Self::HostInactive => "host_inactive",
        }
    }
}

/// Severity of an incident-awareness item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentSeverity {
    /// Critical severity.
    Critical,
    /// High severity.
    High,
    /// Medium severity.
    Medium,
    /// Low severity.
    Low,
}

impl IncidentSeverity {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

/// Awareness state of an incident-awareness item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentAwarenessState {
    /// Open and unacknowledged.
    Open,
    /// Acknowledged from the companion.
    Acknowledged,
    /// Mitigation is in progress on the host.
    Mitigating,
    /// Resolved.
    Resolved,
}

impl IncidentAwarenessState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Acknowledged => "acknowledged",
            Self::Mitigating => "mitigating",
            Self::Resolved => "resolved",
        }
    }
}

/// Attribution state of an incident-awareness item.
///
/// Incident packets must stay attributable to evidence and build identity. When
/// attribution is lost the item narrows to [`Self::Unattributed`] rather than
/// claiming a provenance it can no longer prove.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentAttributionState {
    /// Fully attributed to evidence and build identity.
    Attributed,
    /// Partially attributed; some attribution is missing.
    PartiallyAttributed,
    /// Attribution could not be established.
    Unattributed,
}

impl IncidentAttributionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Attributed => "attributed",
            Self::PartiallyAttributed => "partially_attributed",
            Self::Unattributed => "unattributed",
        }
    }
}

/// Kind of bounded light-edit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LightEditKind {
    /// A bounded text touch-up.
    TextTouchUp,
    /// A reply to a comment thread.
    CommentReply,
    /// A rename suggestion.
    RenameSuggestion,
    /// Resolution of a tracked TODO.
    TodoResolution,
}

impl LightEditKind {
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

/// Lifecycle state of a bounded light-edit.
///
/// Every state passes through the host: a companion drafts an edit and relays it,
/// but the host previews, approves, and applies it. There is deliberately no
/// "applied by companion" state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LightEditState {
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

impl LightEditState {
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

/// Reason a companion scope surface has degraded below its qualified state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionScopeDegradedReason {
    /// The companion relay is unavailable.
    RelayUnavailable,
    /// Proof has gone stale.
    ProofStale,
    /// No active desktop host session.
    HostSessionInactive,
    /// Workspace or device trust narrowed.
    TrustNarrowed,
    /// Incident attribution to evidence or build identity was lost.
    IncidentAttributionLost,
    /// An upstream frozen matrix lane narrowed.
    UpstreamMatrixNarrowed,
    /// One or more handoff targets could not resolve exactly.
    HandoffTargetUnresolved,
    /// One or more item freshness states were downgraded to stale.
    FreshnessDowngradedToStale,
}

impl CompanionScopeDegradedReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RelayUnavailable => "relay_unavailable",
            Self::ProofStale => "proof_stale",
            Self::HostSessionInactive => "host_session_inactive",
            Self::TrustNarrowed => "trust_narrowed",
            Self::IncidentAttributionLost => "incident_attribution_lost",
            Self::UpstreamMatrixNarrowed => "upstream_matrix_narrowed",
            Self::HandoffTargetUnresolved => "handoff_target_unresolved",
            Self::FreshnessDowngradedToStale => "freshness_downgraded_to_stale",
        }
    }
}

/// A read-only session-follow item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionFollowItem {
    /// Stable item id.
    pub item_id: String,
    /// Kind of session being followed.
    pub follow_kind: SessionFollowKind,
    /// Follow state.
    pub follow_state: SessionFollowState,
    /// Freshness of the followed state.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the originating local-core session. Carries no payload body.
    pub source_session_ref: String,
    /// Exact desktop handoff back to the host session.
    pub handoff: CompanionDesktopHandoff,
}

/// A read-only incident-awareness item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentAwarenessItem {
    /// Stable item id.
    pub item_id: String,
    /// Incident severity.
    pub severity: IncidentSeverity,
    /// Awareness state.
    pub awareness_state: IncidentAwarenessState,
    /// Attribution to evidence and build identity.
    pub attribution: IncidentAttributionState,
    /// Freshness of the incident state.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the originating incident evidence. Carries no payload body.
    pub evidence_ref: String,
    /// Exact desktop handoff into the incident workspace.
    pub handoff: CompanionDesktopHandoff,
}

/// A bounded, host-approved light-edit item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundedLightEditItem {
    /// Stable item id.
    pub item_id: String,
    /// Kind of light-edit.
    pub edit_kind: LightEditKind,
    /// Lifecycle state of the edit.
    pub edit_state: LightEditState,
    /// Human-readable description of the bound this edit may not exceed.
    pub write_bound_summary: String,
    /// Freshness of the underlying target.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always
    /// [`CompanionReadWriteScope::BoundedWriteRelayedToHost`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Always true: the host must approve before the edit applies.
    pub requires_host_approval: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the edit target. Carries no payload body.
    pub target_ref: String,
    /// Exact desktop handoff to the host preview/approval location.
    pub handoff: CompanionDesktopHandoff,
}

/// Per-surface qualification inherited from the frozen M5 matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionScopeSurfaceQualification {
    /// Surface the row applies to.
    pub surface: CompanionScopeSurface,
    /// Qualification class earned by this surface.
    pub qualification: M5CompanionQualificationClass,
    /// Staged rollout stage.
    pub rollout_stage: M5CompanionRolloutStage,
    /// Read/write scope this surface is bounded to.
    pub read_write_scope: CompanionReadWriteScope,
    /// Token of the frozen matrix lane this surface inherits qualification from.
    pub matrix_lane_ref: String,
    /// Downgrade triggers that apply to this surface.
    pub downgrade_triggers: Vec<M5CompanionDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5CompanionRollbackPosture,
}

/// Bounded read/write scope contract for the whole surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionScopeContract {
    /// The session-follow surface is read-only.
    pub session_follow_read_only: bool,
    /// The incident-awareness surface is read-only.
    pub incident_awareness_read_only: bool,
    /// Light-edit is bounded and requires explicit host approval.
    pub light_edit_bounded_and_host_approved: bool,
    /// The companion never holds an unbounded write authority.
    pub no_unbounded_companion_write: bool,
    /// The desktop host stays authoritative.
    pub host_authoritative: bool,
    /// No payload bodies cross the export boundary.
    pub no_payload_bodies: bool,
}

/// Stale-state honesty contract for the whole surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionStaleStateHonesty {
    /// Every stale item is labeled.
    pub stale_items_labeled: bool,
    /// Every unknown-freshness item is labeled.
    pub unknown_freshness_labeled: bool,
    /// A stale item is never shown as live.
    pub never_show_stale_as_live: bool,
    /// A freshness floor is enforced before an item is shown.
    pub freshness_floor_enforced: bool,
}

/// Security and privacy review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionScopeSecurityReview {
    /// The session-follow surface is read-only.
    pub session_follow_read_only: bool,
    /// The incident-awareness surface is read-only.
    pub incident_awareness_read_only: bool,
    /// Light-edit is bounded and host-approved.
    pub light_edit_bounded_and_host_approved: bool,
    /// No unbounded companion write authority is exposed.
    pub no_unbounded_companion_write: bool,
    /// The desktop host stays authoritative.
    pub host_stays_authoritative: bool,
    /// Incident attribution is preserved or honestly narrowed.
    pub incident_attribution_preserved: bool,
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
pub struct CompanionScopeConsumerProjection {
    /// Browser companion projects the session-follow surface.
    pub browser_companion_shows_session_follow: bool,
    /// Mobile companion projects the session-follow surface.
    pub mobile_companion_shows_session_follow: bool,
    /// Incident workspace projects the incident-awareness surface.
    pub incident_workspace_shows_awareness: bool,
    /// Desktop panel shows the handoff targets.
    pub desktop_panel_shows_handoff_target: bool,
    /// Support export shows scope and freshness state.
    pub support_export_shows_scope_and_freshness: bool,
    /// Diagnostics shows stale and degraded labels.
    pub diagnostics_shows_stale_labels: bool,
    /// Preview / Labs surfaces are visibly labeled when not qualified Stable.
    pub preview_labs_label_for_unqualified_surfaces: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionScopeProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the surface.
    pub auto_narrow_on_stale: bool,
}

/// Per-surface observation fed to
/// [`CompanionScopeSurfacePacket::apply_companion_scope_degradation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompanionScopeObservation {
    /// True when the companion relay is available.
    pub relay_available: bool,
    /// True when proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when an active desktop host session exists.
    pub host_session_active: bool,
    /// True when workspace and device trust are intact.
    pub trust_intact: bool,
    /// True when incident attribution to evidence and build identity is intact.
    pub incident_attribution_intact: bool,
    /// True when an upstream frozen matrix lane narrowed.
    pub upstream_matrix_narrowed: bool,
}

/// Constructor input for [`CompanionScopeSurfacePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompanionScopeSurfacePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer surfaces that project this packet.
    pub projected_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Per-surface qualification rows.
    pub surface_qualifications: Vec<CompanionScopeSurfaceQualification>,
    /// Session-follow items.
    pub session_follow: Vec<SessionFollowItem>,
    /// Incident-awareness items.
    pub incident_awareness: Vec<IncidentAwarenessItem>,
    /// Bounded light-edit items.
    pub bounded_light_edit: Vec<BoundedLightEditItem>,
    /// Bounded read/write scope contract.
    pub scope_contract: CompanionScopeContract,
    /// Stale-state honesty contract.
    pub stale_state_honesty: CompanionStaleStateHonesty,
    /// Locality disclosure.
    pub locality_disclosure: M5CompanionLocalityDisclosure,
    /// Security review block.
    pub security_review: CompanionScopeSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: CompanionScopeConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CompanionScopeProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe companion session-follow and incident-awareness surface packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionScopeSurfacePacket {
    /// Record kind; must equal [`COMPANION_SCOPE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`COMPANION_SCOPE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer surfaces that project this packet.
    pub projected_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Per-surface qualification rows.
    pub surface_qualifications: Vec<CompanionScopeSurfaceQualification>,
    /// Session-follow items.
    pub session_follow: Vec<SessionFollowItem>,
    /// Incident-awareness items.
    pub incident_awareness: Vec<IncidentAwarenessItem>,
    /// Bounded light-edit items.
    pub bounded_light_edit: Vec<BoundedLightEditItem>,
    /// Bounded read/write scope contract.
    pub scope_contract: CompanionScopeContract,
    /// Stale-state honesty contract.
    pub stale_state_honesty: CompanionStaleStateHonesty,
    /// Locality disclosure.
    pub locality_disclosure: M5CompanionLocalityDisclosure,
    /// Security review block.
    pub security_review: CompanionScopeSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: CompanionScopeConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CompanionScopeProofFreshness,
    /// Degraded-state labels currently applied (empty when fully qualified).
    pub degraded_labels: Vec<CompanionScopeDegradedReason>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl CompanionScopeSurfacePacket {
    /// Builds a companion scope surface packet from stable-lane input.
    pub fn new(input: CompanionScopeSurfacePacketInput) -> Self {
        Self {
            record_kind: COMPANION_SCOPE_RECORD_KIND.to_owned(),
            schema_version: COMPANION_SCOPE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            projected_surfaces: input.projected_surfaces,
            surface_qualifications: input.surface_qualifications,
            session_follow: input.session_follow,
            incident_awareness: input.incident_awareness,
            bounded_light_edit: input.bounded_light_edit,
            scope_contract: input.scope_contract,
            stale_state_honesty: input.stale_state_honesty,
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

    /// Narrows surfaces and downgrades freshness and handoff resolution from a
    /// per-surface observation, recording the reasons in [`Self::degraded_labels`].
    ///
    /// An unavailable relay, stale proof, or narrowed upstream matrix lane narrows
    /// every surface's qualification and rollout stage one step, and an unavailable
    /// relay additionally forces every live or cached item to stale and labels it.
    /// Narrowed trust additionally narrows the bounded light-edit surface; lost
    /// incident attribution narrows the incident-awareness surface and marks its
    /// items unattributed. An inactive host session downgrades the resolution of
    /// every handoff that requires an active host and narrows the bounded light-edit
    /// surface, since a bounded write can no longer be relayed and applied.
    /// Degraded state is labeled, never hidden.
    pub fn apply_companion_scope_degradation(&mut self, observation: &CompanionScopeObservation) {
        let mut labels: BTreeSet<CompanionScopeDegradedReason> =
            self.degraded_labels.iter().copied().collect();

        let surface_adverse = !observation.relay_available
            || !observation.proof_fresh
            || observation.upstream_matrix_narrowed;

        if !observation.relay_available {
            labels.insert(CompanionScopeDegradedReason::RelayUnavailable);
            if self.force_all_freshness_stale() {
                labels.insert(CompanionScopeDegradedReason::FreshnessDowngradedToStale);
            }
        }
        if !observation.proof_fresh {
            labels.insert(CompanionScopeDegradedReason::ProofStale);
        }
        if observation.upstream_matrix_narrowed {
            labels.insert(CompanionScopeDegradedReason::UpstreamMatrixNarrowed);
        }
        if !observation.trust_intact {
            labels.insert(CompanionScopeDegradedReason::TrustNarrowed);
        }
        if !observation.incident_attribution_intact {
            labels.insert(CompanionScopeDegradedReason::IncidentAttributionLost);
            for item in &mut self.incident_awareness {
                item.attribution = IncidentAttributionState::Unattributed;
            }
        }

        for row in &mut self.surface_qualifications {
            let adverse = surface_adverse
                || (!observation.trust_intact
                    && row.surface == CompanionScopeSurface::BoundedLightEdit)
                || (!observation.host_session_active
                    && row.surface == CompanionScopeSurface::BoundedLightEdit)
                || (!observation.incident_attribution_intact
                    && row.surface == CompanionScopeSurface::IncidentAwareness);
            if adverse {
                row.qualification = row.qualification.narrowed_one_step();
                row.rollout_stage = row.rollout_stage.narrowed_one_step();
            }
        }

        if !observation.host_session_active {
            labels.insert(CompanionScopeDegradedReason::HostSessionInactive);
            let mut any_unresolved = false;
            for handoff in self.handoffs_mut() {
                if handoff.requires_active_host
                    && handoff.resolution == CompanionHandoffResolution::Exact
                {
                    handoff.resolution = CompanionHandoffResolution::Unresolved;
                    any_unresolved = true;
                }
            }
            if any_unresolved {
                labels.insert(CompanionScopeDegradedReason::HandoffTargetUnresolved);
            }
        }

        self.degraded_labels = labels.into_iter().collect();
    }

    /// Forces every live/cached item freshness to stale and labels it. Returns
    /// true when at least one item was downgraded.
    fn force_all_freshness_stale(&mut self) -> bool {
        let mut downgraded = false;
        for item in &mut self.session_follow {
            if item.freshness != item.freshness.forced_stale() {
                item.freshness = item.freshness.forced_stale();
                item.stale_label_shown = true;
                downgraded = true;
            }
        }
        for item in &mut self.incident_awareness {
            if item.freshness != item.freshness.forced_stale() {
                item.freshness = item.freshness.forced_stale();
                item.stale_label_shown = true;
                downgraded = true;
            }
        }
        for item in &mut self.bounded_light_edit {
            if item.freshness != item.freshness.forced_stale() {
                item.freshness = item.freshness.forced_stale();
                item.stale_label_shown = true;
                downgraded = true;
            }
        }
        downgraded
    }

    /// Validates the companion scope surface invariants.
    pub fn validate(&self) -> Vec<CompanionScopeViolation> {
        let mut violations = Vec::new();

        if self.record_kind != COMPANION_SCOPE_RECORD_KIND {
            violations.push(CompanionScopeViolation::WrongRecordKind);
        }
        if self.schema_version != COMPANION_SCOPE_SCHEMA_VERSION {
            violations.push(CompanionScopeViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(CompanionScopeViolation::MissingIdentity);
        }
        if self.projected_surfaces.is_empty() {
            violations.push(CompanionScopeViolation::ProjectedSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_surface_qualifications(self, &mut violations);
        validate_items(self, &mut violations);
        validate_scope_contract(self, &mut violations);
        validate_stale_state_honesty(self, &mut violations);
        validate_locality(self, &mut violations);
        validate_security_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("companion scope packet serializes"),
        ) {
            violations.push(CompanionScopeViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("companion scope packet serializes")
    }

    /// Surfaces currently publishable (Stable, Beta, or Preview) and not withheld.
    pub fn publishable_surfaces(
        &self,
    ) -> impl Iterator<Item = &CompanionScopeSurfaceQualification> {
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
        self.session_follow
            .iter()
            .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .incident_awareness
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .bounded_light_edit
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
    }

    /// Iterates every handoff across all three surfaces, in surface order.
    pub fn handoffs(&self) -> impl Iterator<Item = &CompanionDesktopHandoff> {
        self.session_follow
            .iter()
            .map(|item| &item.handoff)
            .chain(self.incident_awareness.iter().map(|item| &item.handoff))
            .chain(self.bounded_light_edit.iter().map(|item| &item.handoff))
    }

    fn handoffs_mut(&mut self) -> impl Iterator<Item = &mut CompanionDesktopHandoff> {
        self.session_follow
            .iter_mut()
            .map(|item| &mut item.handoff)
            .chain(
                self.incident_awareness
                    .iter_mut()
                    .map(|item| &mut item.handoff),
            )
            .chain(
                self.bounded_light_edit
                    .iter_mut()
                    .map(|item| &mut item.handoff),
            )
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Companion Session-Follow and Incident-Awareness Surfaces\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Surfaces: {} | Session-follow: {} | Incident-awareness: {} | Light-edit: {}\n",
            self.surface_qualifications.len(),
            self.session_follow.len(),
            self.incident_awareness.len(),
            self.bounded_light_edit.len(),
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

        out.push_str("\n## Session-follow\n\n");
        for item in &self.session_follow {
            out.push_str(&format!(
                "- `{}` [{}] {} — {} ({}) → `{}` ({})\n",
                item.item_id,
                item.follow_kind.as_str(),
                item.follow_state.as_str(),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Incident-awareness\n\n");
        for item in &self.incident_awareness {
            out.push_str(&format!(
                "- `{}` [{}/{}] {} — {} ({}) → `{}` ({})\n",
                item.item_id,
                item.severity.as_str(),
                item.attribution.as_str(),
                item.awareness_state.as_str(),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Bounded light-edit\n\n");
        for item in &self.bounded_light_edit {
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

        out
    }
}

/// Errors emitted when reading the checked-in companion scope export.
#[derive(Debug)]
pub enum CompanionScopeArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CompanionScopeViolation>),
}

impl fmt::Display for CompanionScopeArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "companion scope export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "companion scope export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for CompanionScopeArtifactError {}

/// Validation failures emitted by [`CompanionScopeSurfacePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompanionScopeViolation {
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

impl CompanionScopeViolation {
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
            Self::ItemIncomplete => "item_incomplete",
            Self::StaleStateNotLabeled => "stale_state_not_labeled",
            Self::HandoffRefMissing => "handoff_ref_missing",
            Self::ScopeContractIncomplete => "scope_contract_incomplete",
            Self::StaleStateHonestyIncomplete => "stale_state_honesty_incomplete",
            Self::LocalityDisclosureIncomplete => "locality_disclosure_incomplete",
            Self::SecurityReviewIncomplete => "security_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable companion scope surface export.
///
/// This is the canonical reader: a browser/mobile companion, the incident
/// workspace, the desktop panel, diagnostics, support-export, or Help/About
/// surface calls it to ingest the packet rather than cloning status text.
///
/// # Errors
///
/// Returns [`CompanionScopeArtifactError`] when the checked-in support export
/// fails to parse or fails validation.
pub fn current_stable_companion_scope_surface_export(
) -> Result<CompanionScopeSurfacePacket, CompanionScopeArtifactError> {
    let packet: CompanionScopeSurfacePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/companion/m5/ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty/support_export.json"
    )))
    .map_err(CompanionScopeArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CompanionScopeArtifactError::Validation(violations))
    }
}

/// Canonical source contract refs that every scope export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        COMPANION_SCOPE_SCHEMA_REF.to_owned(),
        COMPANION_SCOPE_DOC_REF.to_owned(),
        M5_COMPANION_SURFACE_CONTRACT_REF.to_owned(),
        M5_COMPANION_QUALIFICATION_REF.to_owned(),
        M5_COMPANION_BOUNDARY_MANIFEST_REF.to_owned(),
        M5_INCIDENT_WORKSPACE_CONTRACT_REF.to_owned(),
        M5_COMPANION_MATRIX_SCHEMA_REF.to_owned(),
    ]
}

/// Canonical bounded read/write scope contract with every guarantee satisfied.
pub fn canonical_scope_contract() -> CompanionScopeContract {
    CompanionScopeContract {
        session_follow_read_only: true,
        incident_awareness_read_only: true,
        light_edit_bounded_and_host_approved: true,
        no_unbounded_companion_write: true,
        host_authoritative: true,
        no_payload_bodies: true,
    }
}

/// Canonical stale-state honesty contract with every guarantee satisfied.
pub fn canonical_stale_state_honesty() -> CompanionStaleStateHonesty {
    CompanionStaleStateHonesty {
        stale_items_labeled: true,
        unknown_freshness_labeled: true,
        never_show_stale_as_live: true,
        freshness_floor_enforced: true,
    }
}

/// Canonical security review block with every invariant satisfied.
pub fn canonical_security_review() -> CompanionScopeSecurityReview {
    CompanionScopeSecurityReview {
        session_follow_read_only: true,
        incident_awareness_read_only: true,
        light_edit_bounded_and_host_approved: true,
        no_unbounded_companion_write: true,
        host_stays_authoritative: true,
        incident_attribution_preserved: true,
        stale_state_labeled_never_hidden: true,
        exact_desktop_handoff_preserved: true,
        no_payload_bodies_in_export: true,
        downgrade_narrows_instead_of_hides: true,
        locality_disclosed: true,
    }
}

/// Canonical consumer projection block with every surface projecting truth.
pub fn canonical_consumer_projection() -> CompanionScopeConsumerProjection {
    CompanionScopeConsumerProjection {
        browser_companion_shows_session_follow: true,
        mobile_companion_shows_session_follow: true,
        incident_workspace_shows_awareness: true,
        desktop_panel_shows_handoff_target: true,
        support_export_shows_scope_and_freshness: true,
        diagnostics_shows_stale_labels: true,
        preview_labs_label_for_unqualified_surfaces: true,
    }
}

/// Canonical per-surface qualification rows, inherited from the frozen matrix.
pub fn canonical_surface_qualifications() -> Vec<CompanionScopeSurfaceQualification> {
    use M5CompanionDowngradeTrigger as Trigger;
    use M5CompanionQualificationClass as Qual;
    use M5CompanionRollbackPosture as Rollback;
    use M5CompanionRolloutStage as Stage;

    vec![
        CompanionScopeSurfaceQualification {
            surface: CompanionScopeSurface::SessionFollow,
            qualification: Qual::Beta,
            rollout_stage: Stage::StagedRollout,
            read_write_scope: CompanionScopeSurface::SessionFollow.bounded_scope(),
            matrix_lane_ref: CompanionScopeSurface::SessionFollow
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
        CompanionScopeSurfaceQualification {
            surface: CompanionScopeSurface::IncidentAwareness,
            qualification: Qual::Stable,
            rollout_stage: Stage::GeneralAvailability,
            read_write_scope: CompanionScopeSurface::IncidentAwareness.bounded_scope(),
            matrix_lane_ref: CompanionScopeSurface::IncidentAwareness
                .matrix_lane()
                .as_str()
                .to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::IncidentAttributionMissing,
                Trigger::PolicyBlocked,
            ],
            rollback_posture: Rollback::LocalCoreContinuesNoRemoteState,
        },
        CompanionScopeSurfaceQualification {
            surface: CompanionScopeSurface::BoundedLightEdit,
            qualification: Qual::Preview,
            rollout_stage: Stage::EarlyAccess,
            read_write_scope: CompanionScopeSurface::BoundedLightEdit.bounded_scope(),
            matrix_lane_ref: CompanionScopeSurface::BoundedLightEdit
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
    ]
}

/// Canonical locality disclosure for the scope surface.
pub fn canonical_locality_disclosure() -> M5CompanionLocalityDisclosure {
    M5CompanionLocalityDisclosure {
        stays_local:
            "Session state, incident evidence, and edit targets are owned by the local core and stay inspectable offline."
                .to_owned(),
        staged:
            "Companion session-follow streaming and bounded light-edit roll out per cohort and capability gate."
                .to_owned(),
        requires_provider_or_admin_continuity:
            "Live follow, exact handoff, and relaying a bounded edit for host approval require the companion relay and an active host session; the local core never depends on them to function."
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

/// Canonical read-only session-follow items.
pub fn canonical_session_follow() -> Vec<SessionFollowItem> {
    use CompanionFreshnessState as Fresh;
    use CompanionHandoffTarget as Target;
    use SessionFollowKind as Kind;
    use SessionFollowState as FollowState;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        SessionFollowItem {
            item_id: "follow:editor:0001".to_owned(),
            follow_kind: Kind::ActiveEditor,
            follow_state: FollowState::Following,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Following the active editor on the host workspace".to_owned(),
            source_session_ref: "session:editor:0001".to_owned(),
            handoff: handoff(
                Target::FileLocation,
                "handoff:file-location:follow-0001",
                true,
            ),
        },
        SessionFollowItem {
            item_id: "follow:agent:0002".to_owned(),
            follow_kind: Kind::AgentRun,
            follow_state: FollowState::Following,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Following a running agent session".to_owned(),
            source_session_ref: "session:agent:0002".to_owned(),
            handoff: handoff(
                Target::AgentSession,
                "handoff:agent-session:follow-0002",
                true,
            ),
        },
        SessionFollowItem {
            item_id: "follow:terminal:0003".to_owned(),
            follow_kind: Kind::TerminalStream,
            follow_state: FollowState::Paused,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Paused on a cached terminal stream".to_owned(),
            source_session_ref: "session:terminal:0003".to_owned(),
            handoff: handoff(
                Target::FileLocation,
                "handoff:file-location:follow-0003",
                true,
            ),
        },
    ]
}

/// Canonical read-only incident-awareness items.
pub fn canonical_incident_awareness() -> Vec<IncidentAwarenessItem> {
    use CompanionFreshnessState as Fresh;
    use CompanionHandoffTarget as Target;
    use IncidentAttributionState as Attribution;
    use IncidentAwarenessState as AwarenessState;
    use IncidentSeverity as Severity;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        IncidentAwarenessItem {
            item_id: "incident:0001".to_owned(),
            severity: Severity::Critical,
            awareness_state: AwarenessState::Open,
            attribution: Attribution::Attributed,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Critical incident raised from a crash trail".to_owned(),
            evidence_ref: "evidence:incident:0001".to_owned(),
            handoff: handoff(
                Target::IncidentWorkspace,
                "handoff:incident-workspace:0001",
                false,
            ),
        },
        IncidentAwarenessItem {
            item_id: "incident:0002".to_owned(),
            severity: Severity::High,
            awareness_state: AwarenessState::Mitigating,
            attribution: Attribution::Attributed,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "High-severity incident under mitigation on the host".to_owned(),
            evidence_ref: "evidence:incident:0002".to_owned(),
            handoff: handoff(
                Target::IncidentWorkspace,
                "handoff:incident-workspace:0002",
                false,
            ),
        },
        IncidentAwarenessItem {
            item_id: "incident:0003".to_owned(),
            severity: Severity::Medium,
            awareness_state: AwarenessState::Acknowledged,
            attribution: Attribution::PartiallyAttributed,
            freshness: Fresh::Stale,
            read_write_scope: scope,
            stale_label_shown: true,
            summary: "Acknowledged incident with stale awareness state".to_owned(),
            evidence_ref: "evidence:incident:0003".to_owned(),
            handoff: handoff(
                Target::IncidentWorkspace,
                "handoff:incident-workspace:0003",
                false,
            ),
        },
    ]
}

/// Canonical bounded, host-approved light-edit items.
pub fn canonical_bounded_light_edit() -> Vec<BoundedLightEditItem> {
    use CompanionFreshnessState as Fresh;
    use CompanionHandoffTarget as Target;
    use LightEditKind as Kind;
    use LightEditState as EditState;

    let scope = CompanionReadWriteScope::BoundedWriteRelayedToHost;
    vec![
        BoundedLightEditItem {
            item_id: "edit:0001".to_owned(),
            edit_kind: Kind::TextTouchUp,
            edit_state: EditState::AwaitingHostApproval,
            write_bound_summary:
                "Single-line text touch-up within the open file; no structural edits".to_owned(),
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            requires_host_approval: true,
            summary: "Bounded text touch-up relayed for host approval".to_owned(),
            target_ref: "target:file-location:edit-0001".to_owned(),
            handoff: handoff(
                Target::FileLocation,
                "handoff:file-location:edit-0001",
                true,
            ),
        },
        BoundedLightEditItem {
            item_id: "edit:0002".to_owned(),
            edit_kind: Kind::CommentReply,
            edit_state: EditState::RelayedForPreview,
            write_bound_summary: "Reply text only on an existing comment thread".to_owned(),
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            requires_host_approval: true,
            summary: "Comment reply relayed to the host for preview".to_owned(),
            target_ref: "target:review-panel:edit-0002".to_owned(),
            handoff: handoff(Target::ReviewPanel, "handoff:review-panel:edit-0002", true),
        },
    ]
}

/// Builds the canonical companion scope surface packet.
///
/// This is the first consumer: it mints the surface the checked-in support export
/// and Markdown summary are generated from, so the artifact never drifts from the
/// typed surface, item, scope, and freshness definitions.
pub fn canonical_companion_scope_surface(
    packet_id: String,
    surface_label: String,
    minted_at: String,
    proof_freshness: CompanionScopeProofFreshness,
) -> CompanionScopeSurfacePacket {
    CompanionScopeSurfacePacket::new(CompanionScopeSurfacePacketInput {
        packet_id,
        surface_label,
        projected_surfaces: vec![
            M5CompanionConsumerSurface::BrowserCompanion,
            M5CompanionConsumerSurface::MobileCompanion,
            M5CompanionConsumerSurface::DesktopCompanionPanel,
            M5CompanionConsumerSurface::IncidentWorkspace,
            M5CompanionConsumerSurface::SupportExport,
            M5CompanionConsumerSurface::Diagnostics,
        ],
        surface_qualifications: canonical_surface_qualifications(),
        session_follow: canonical_session_follow(),
        incident_awareness: canonical_incident_awareness(),
        bounded_light_edit: canonical_bounded_light_edit(),
        scope_contract: canonical_scope_contract(),
        stale_state_honesty: canonical_stale_state_honesty(),
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
    packet: &CompanionScopeSurfacePacket,
    violations: &mut Vec<CompanionScopeViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        COMPANION_SCOPE_SCHEMA_REF,
        COMPANION_SCOPE_DOC_REF,
        M5_COMPANION_SURFACE_CONTRACT_REF,
        M5_INCIDENT_WORKSPACE_CONTRACT_REF,
        M5_COMPANION_MATRIX_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(CompanionScopeViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_surface_qualifications(
    packet: &CompanionScopeSurfacePacket,
    violations: &mut Vec<CompanionScopeViolation>,
) {
    let present: BTreeSet<CompanionScopeSurface> = packet
        .surface_qualifications
        .iter()
        .map(|row| row.surface)
        .collect();
    for required in CompanionScopeSurface::ALL {
        if !present.contains(&required) {
            violations.push(CompanionScopeViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_qualifications {
        if row.matrix_lane_ref != row.surface.matrix_lane().as_str() {
            violations.push(CompanionScopeViolation::SurfaceLaneMismatch);
        }
        if row.read_write_scope != row.surface.bounded_scope() {
            violations.push(CompanionScopeViolation::SurfaceScopeMismatch);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(CompanionScopeViolation::SurfaceRowIncomplete);
        }
    }
}

fn validate_items(
    packet: &CompanionScopeSurfacePacket,
    violations: &mut Vec<CompanionScopeViolation>,
) {
    if packet.session_follow.is_empty()
        || packet.incident_awareness.is_empty()
        || packet.bounded_light_edit.is_empty()
    {
        violations.push(CompanionScopeViolation::SurfaceContentMissing);
    }

    for item in &packet.session_follow {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(CompanionScopeViolation::ReadOnlyScopeViolated);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.source_session_ref.trim().is_empty()
        {
            violations.push(CompanionScopeViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.incident_awareness {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(CompanionScopeViolation::ReadOnlyScopeViolated);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.evidence_ref.trim().is_empty()
        {
            violations.push(CompanionScopeViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.bounded_light_edit {
        if item.read_write_scope != CompanionReadWriteScope::BoundedWriteRelayedToHost
            || !item.requires_host_approval
            || item.write_bound_summary.trim().is_empty()
        {
            violations.push(CompanionScopeViolation::LightEditUnbounded);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.target_ref.trim().is_empty()
        {
            violations.push(CompanionScopeViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }
}

fn validate_freshness_label(
    freshness: CompanionFreshnessState,
    stale_label_shown: bool,
    violations: &mut Vec<CompanionScopeViolation>,
) {
    if freshness.requires_label() && !stale_label_shown {
        violations.push(CompanionScopeViolation::StaleStateNotLabeled);
    }
}

fn validate_handoff(
    handoff: &CompanionDesktopHandoff,
    violations: &mut Vec<CompanionScopeViolation>,
) {
    if handoff.deep_link_ref.trim().is_empty() {
        violations.push(CompanionScopeViolation::HandoffRefMissing);
    }
}

fn validate_scope_contract(
    packet: &CompanionScopeSurfacePacket,
    violations: &mut Vec<CompanionScopeViolation>,
) {
    let contract = &packet.scope_contract;
    for ok in [
        contract.session_follow_read_only,
        contract.incident_awareness_read_only,
        contract.light_edit_bounded_and_host_approved,
        contract.no_unbounded_companion_write,
        contract.host_authoritative,
        contract.no_payload_bodies,
    ] {
        if !ok {
            violations.push(CompanionScopeViolation::ScopeContractIncomplete);
            return;
        }
    }
}

fn validate_stale_state_honesty(
    packet: &CompanionScopeSurfacePacket,
    violations: &mut Vec<CompanionScopeViolation>,
) {
    let honesty = &packet.stale_state_honesty;
    for ok in [
        honesty.stale_items_labeled,
        honesty.unknown_freshness_labeled,
        honesty.never_show_stale_as_live,
        honesty.freshness_floor_enforced,
    ] {
        if !ok {
            violations.push(CompanionScopeViolation::StaleStateHonestyIncomplete);
            return;
        }
    }
}

fn validate_locality(
    packet: &CompanionScopeSurfacePacket,
    violations: &mut Vec<CompanionScopeViolation>,
) {
    let locality = &packet.locality_disclosure;
    if locality.stays_local.trim().is_empty()
        || locality.staged.trim().is_empty()
        || locality
            .requires_provider_or_admin_continuity
            .trim()
            .is_empty()
    {
        violations.push(CompanionScopeViolation::LocalityDisclosureIncomplete);
    }
}

fn validate_security_review(
    packet: &CompanionScopeSurfacePacket,
    violations: &mut Vec<CompanionScopeViolation>,
) {
    let review = &packet.security_review;
    for ok in [
        review.session_follow_read_only,
        review.incident_awareness_read_only,
        review.light_edit_bounded_and_host_approved,
        review.no_unbounded_companion_write,
        review.host_stays_authoritative,
        review.incident_attribution_preserved,
        review.stale_state_labeled_never_hidden,
        review.exact_desktop_handoff_preserved,
        review.no_payload_bodies_in_export,
        review.downgrade_narrows_instead_of_hides,
        review.locality_disclosed,
    ] {
        if !ok {
            violations.push(CompanionScopeViolation::SecurityReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &CompanionScopeSurfacePacket,
    violations: &mut Vec<CompanionScopeViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.browser_companion_shows_session_follow,
        projection.mobile_companion_shows_session_follow,
        projection.incident_workspace_shows_awareness,
        projection.desktop_panel_shows_handoff_target,
        projection.support_export_shows_scope_and_freshness,
        projection.diagnostics_shows_stale_labels,
        projection.preview_labs_label_for_unqualified_surfaces,
    ] {
        if !ok {
            violations.push(CompanionScopeViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &CompanionScopeSurfacePacket,
    violations: &mut Vec<CompanionScopeViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(CompanionScopeViolation::ProofFreshnessIncomplete);
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
