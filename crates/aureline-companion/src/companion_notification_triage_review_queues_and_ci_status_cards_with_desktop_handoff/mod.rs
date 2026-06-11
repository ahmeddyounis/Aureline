//! Browser and mobile companion notification triage, review queues, and CI-status
//! cards with exact desktop handoff.
//!
//! This module owns the export-safe truth packet for the read-only companion
//! triage surface that browser and mobile companions project. It binds three
//! sections — notification triage, review queues, and CI-status cards — to the
//! frozen M5 companion lanes that qualify them, and gives every item an
//! [`CompanionDesktopHandoff`] that resolves to an *exact* desktop location so a
//! companion tap resumes the precise host context instead of an approximate one.
//!
//! The surface stays deliberately narrow. Companions never author edits: every
//! notification is read-only, every review-queue item carries an approve/defer or
//! escalate-only authority, and the desktop host stays authoritative. The packet
//! reuses the matrix vocabulary from
//! [`crate::freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes`]
//! ([`M5CompanionQualificationClass`], [`M5CompanionRolloutStage`],
//! [`M5CompanionDowngradeTrigger`], [`M5CompanionRollbackPosture`],
//! [`M5CompanionLocalityDisclosure`], [`M5CompanionConsumerSurface`]) instead of
//! inventing parallel terms, and each section row records the matrix lane it
//! inherits qualification from.
//!
//! [`CompanionTriageSurfacePacket::apply_companion_degradation`] narrows sections
//! and downgrades handoff resolution from per-surface observations — when the
//! companion relay is unavailable, proof is stale, the host session is inactive,
//! workspace trust narrowed, or an upstream matrix lane narrowed — so CI or
//! release tooling can degrade the surface honestly rather than show an exact
//! handoff that no longer resolves. Degraded state is labeled, never hidden.
//!
//! [`canonical_companion_triage_surface`] builds the surface and
//! [`current_stable_companion_triage_surface_export`] reads and validates the
//! checked-in support export, so browser/mobile companions, the desktop panel,
//! diagnostics, support exports, and Help/About ingest the packet rather than
//! cloning status text. Credential bodies, raw provider payloads, and raw event
//! bodies stay outside this boundary.
//!
//! The boundary schema is
//! [`schemas/companion/companion-notification-triage-review-queues-and-ci-status-cards-with-desktop-handoff.schema.json`](../../../../schemas/companion/companion-notification-triage-review-queues-and-ci-status-cards-with-desktop-handoff.schema.json).
//! The contract doc is
//! [`docs/companion/m5/companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff.md`](../../../../docs/companion/m5/companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff.md).
//! The protected fixture directory is
//! [`fixtures/companion/m5/companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff/`](../../../../fixtures/companion/m5/companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes::{
    M5CompanionConsumerSurface, M5CompanionDowngradeTrigger, M5CompanionLocalityDisclosure,
    M5CompanionMatrixLane, M5CompanionQualificationClass, M5CompanionRollbackPosture,
    M5CompanionRolloutStage, M5_COMPANION_BOUNDARY_MANIFEST_REF, M5_COMPANION_MATRIX_SCHEMA_REF,
    M5_COMPANION_QUALIFICATION_REF, M5_COMPANION_SURFACE_CONTRACT_REF,
};

/// Stable record-kind tag carried by [`CompanionTriageSurfacePacket`].
pub const COMPANION_TRIAGE_RECORD_KIND: &str =
    "companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff";

/// Schema version for companion triage surface records.
pub const COMPANION_TRIAGE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const COMPANION_TRIAGE_SCHEMA_REF: &str =
    "schemas/companion/companion-notification-triage-review-queues-and-ci-status-cards-with-desktop-handoff.schema.json";

/// Repo-relative path of the companion triage surface contract doc.
pub const COMPANION_TRIAGE_DOC_REF: &str =
    "docs/companion/m5/companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff.md";

/// Repo-relative path of the protected fixture directory.
pub const COMPANION_TRIAGE_FIXTURE_DIR: &str =
    "fixtures/companion/m5/companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff";

/// Repo-relative path of the checked support-export artifact.
pub const COMPANION_TRIAGE_ARTIFACT_REF: &str =
    "artifacts/companion/m5/companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const COMPANION_TRIAGE_SUMMARY_REF: &str =
    "artifacts/companion/m5/companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff.md";

/// One of the three companion triage surface sections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionTriageSection {
    /// Browser/mobile notification triage list.
    NotificationTriage,
    /// Browser/mobile review queue.
    ReviewQueue,
    /// Browser/mobile CI-status cards.
    CiStatusCards,
}

impl CompanionTriageSection {
    /// Every section, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::NotificationTriage,
        Self::ReviewQueue,
        Self::CiStatusCards,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotificationTriage => "notification_triage",
            Self::ReviewQueue => "review_queue",
            Self::CiStatusCards => "ci_status_cards",
        }
    }

    /// Frozen M5 companion-matrix lane this section inherits qualification from.
    pub const fn matrix_lane(self) -> M5CompanionMatrixLane {
        match self {
            Self::NotificationTriage => M5CompanionMatrixLane::CompanionNotification,
            Self::ReviewQueue => M5CompanionMatrixLane::CompanionReview,
            Self::CiStatusCards => M5CompanionMatrixLane::CompanionSessionFollow,
        }
    }
}

/// Category of a companion notification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionNotificationCategory {
    /// Build or compile event.
    Build,
    /// Review or approval event.
    Review,
    /// Agent run event.
    Agent,
    /// Incident or crash event.
    Incident,
    /// Managed-sync event.
    Sync,
}

impl CompanionNotificationCategory {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Build => "build",
            Self::Review => "review",
            Self::Agent => "agent",
            Self::Incident => "incident",
            Self::Sync => "sync",
        }
    }
}

/// Triage priority of a companion notification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionTriagePriority {
    /// Critical — surfaced at the top of triage.
    Critical,
    /// High priority.
    High,
    /// Normal priority.
    Normal,
    /// Low priority.
    Low,
}

impl CompanionTriagePriority {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::High => "high",
            Self::Normal => "normal",
            Self::Low => "low",
        }
    }
}

/// Triage state of a companion notification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionTriageState {
    /// Not yet triaged.
    Unread,
    /// Reviewed and acknowledged.
    Triaged,
    /// Snoozed for later.
    Snoozed,
    /// Dismissed.
    Dismissed,
    /// Escalated to the desktop host for action.
    EscalatedToDesktop,
}

impl CompanionTriageState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unread => "unread",
            Self::Triaged => "triaged",
            Self::Snoozed => "snoozed",
            Self::Dismissed => "dismissed",
            Self::EscalatedToDesktop => "escalated_to_desktop",
        }
    }
}

/// Kind of work queued for companion review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionReviewKind {
    /// Pending agent change set.
    AgentChange,
    /// Diff awaiting review.
    DiffReview,
    /// Comment thread awaiting response.
    CommentThread,
    /// Explicit approval request.
    ApprovalRequest,
}

impl CompanionReviewKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AgentChange => "agent_change",
            Self::DiffReview => "diff_review",
            Self::CommentThread => "comment_thread",
            Self::ApprovalRequest => "approval_request",
        }
    }
}

/// State of a companion review-queue item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionReviewQueueState {
    /// Awaiting a decision.
    Pending,
    /// Approved from the companion.
    Approved,
    /// Deferred to the desktop host.
    Deferred,
    /// Escalated to the desktop host.
    EscalatedToDesktop,
}

impl CompanionReviewQueueState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Approved => "approved",
            Self::Deferred => "deferred",
            Self::EscalatedToDesktop => "escalated_to_desktop",
        }
    }
}

/// Decision authority a companion holds over a review-queue item.
///
/// The companion is read-only with respect to authoring: it may approve or defer
/// a pre-staged action, or escalate it to the desktop host, but never author the
/// underlying edit. There is deliberately no authoring variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionDecisionAuthority {
    /// May approve or defer a pre-staged action without authoring edits.
    ApproveOrDefer,
    /// May only escalate the item to the desktop host.
    EscalateOnly,
}

impl CompanionDecisionAuthority {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ApproveOrDefer => "approve_or_defer",
            Self::EscalateOnly => "escalate_only",
        }
    }
}

/// Status shown on a companion CI-status card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionCiStatus {
    /// Pipeline passed.
    Passed,
    /// Pipeline failed.
    Failed,
    /// Pipeline is running.
    Running,
    /// Pipeline is queued.
    Queued,
    /// Pipeline was canceled.
    Canceled,
    /// Status is stale and could not be refreshed.
    Stale,
}

impl CompanionCiStatus {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Running => "running",
            Self::Queued => "queued",
            Self::Canceled => "canceled",
            Self::Stale => "stale",
        }
    }
}

/// Freshness of a companion CI-status card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionCardFreshness {
    /// Streaming live from the local core via the relay.
    Live,
    /// Last-known cached value.
    Cached,
    /// Stale beyond its freshness window.
    Stale,
}

impl CompanionCardFreshness {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Cached => "cached",
            Self::Stale => "stale",
        }
    }
}

/// Target a companion handoff resumes on the desktop host.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionHandoffTarget {
    /// An exact file location (path plus position).
    FileLocation,
    /// The review panel for a specific item.
    ReviewPanel,
    /// A CI pipeline run view.
    CiPipeline,
    /// An incident workspace.
    IncidentWorkspace,
    /// A running agent session.
    AgentSession,
}

impl CompanionHandoffTarget {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileLocation => "file_location",
            Self::ReviewPanel => "review_panel",
            Self::CiPipeline => "ci_pipeline",
            Self::IncidentWorkspace => "incident_workspace",
            Self::AgentSession => "agent_session",
        }
    }
}

/// How precisely a companion handoff resolves on the desktop host.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionHandoffResolution {
    /// Resolves to the exact desktop location — the qualified state.
    Exact,
    /// Resolves only to an approximate location.
    Approximate,
    /// Cannot currently resolve.
    Unresolved,
}

impl CompanionHandoffResolution {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::Unresolved => "unresolved",
        }
    }

    /// Narrows the resolution one step toward [`Self::Unresolved`].
    pub const fn narrowed_one_step(self) -> Self {
        match self {
            Self::Exact => Self::Approximate,
            Self::Approximate | Self::Unresolved => Self::Unresolved,
        }
    }
}

/// Reason a companion triage surface has degraded below its qualified state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionDegradedReason {
    /// The companion relay is unavailable.
    RelayUnavailable,
    /// Proof has gone stale.
    ProofStale,
    /// No active desktop host session.
    HostSessionInactive,
    /// Workspace or device trust narrowed.
    TrustNarrowed,
    /// An upstream frozen matrix lane narrowed.
    UpstreamMatrixNarrowed,
    /// One or more handoff targets could not resolve exactly.
    HandoffTargetUnresolved,
}

impl CompanionDegradedReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RelayUnavailable => "relay_unavailable",
            Self::ProofStale => "proof_stale",
            Self::HostSessionInactive => "host_session_inactive",
            Self::TrustNarrowed => "trust_narrowed",
            Self::UpstreamMatrixNarrowed => "upstream_matrix_narrowed",
            Self::HandoffTargetUnresolved => "handoff_target_unresolved",
        }
    }
}

/// Exact desktop handoff attached to a companion item.
///
/// The handoff carries an opaque, resolvable deep-link ref — never a payload body
/// — and records whether it resolves to the exact desktop location and whether an
/// active host session is required to resume it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionDesktopHandoff {
    /// Target the handoff resumes on the desktop host.
    pub target: CompanionHandoffTarget,
    /// Opaque, resolvable deep-link ref. Carries no payload body.
    pub deep_link_ref: String,
    /// How precisely the handoff resolves on the host.
    pub resolution: CompanionHandoffResolution,
    /// True when an active desktop host session is required to resume.
    pub requires_active_host: bool,
}

/// A companion notification triage item (read-only).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionNotificationItem {
    /// Stable item id.
    pub item_id: String,
    /// Notification category.
    pub category: CompanionNotificationCategory,
    /// Triage priority.
    pub priority: CompanionTriagePriority,
    /// Triage state.
    pub triage_state: CompanionTriageState,
    /// Redacted headline. Carries no payload body.
    pub headline: String,
    /// Ref to the originating local-core event. Carries no payload body.
    pub source_event_ref: String,
    /// Always true: the companion never authors from a notification.
    pub read_only: bool,
    /// Exact desktop handoff.
    pub handoff: CompanionDesktopHandoff,
}

/// A companion review-queue item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionReviewQueueItem {
    /// Stable item id.
    pub item_id: String,
    /// Kind of work queued.
    pub review_kind: CompanionReviewKind,
    /// Queue state.
    pub queue_state: CompanionReviewQueueState,
    /// Decision authority the companion holds (never authoring).
    pub decision_authority: CompanionDecisionAuthority,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the originating evidence. Carries no payload body.
    pub evidence_ref: String,
    /// Exact desktop handoff.
    pub handoff: CompanionDesktopHandoff,
}

/// A companion CI-status card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionCiStatusCard {
    /// Stable card id.
    pub card_id: String,
    /// Human-readable pipeline label.
    pub pipeline_label: String,
    /// Pipeline status.
    pub status: CompanionCiStatus,
    /// Card freshness.
    pub freshness: CompanionCardFreshness,
    /// Count of failing checks.
    pub failing_check_count: u32,
    /// Exact desktop handoff.
    pub handoff: CompanionDesktopHandoff,
}

/// Per-section qualification inherited from the frozen M5 matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionSectionQualification {
    /// Section the row applies to.
    pub section: CompanionTriageSection,
    /// Qualification class earned by this section.
    pub qualification: M5CompanionQualificationClass,
    /// Staged rollout stage.
    pub rollout_stage: M5CompanionRolloutStage,
    /// Token of the frozen matrix lane this section inherits qualification from.
    pub matrix_lane_ref: String,
    /// Downgrade triggers that apply to this section.
    pub downgrade_triggers: Vec<M5CompanionDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5CompanionRollbackPosture,
}

/// Handoff contract for the whole surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionHandoffContract {
    /// Each item must resolve to an exact desktop location when qualified.
    pub exact_target_required: bool,
    /// The companion is read-only and never authors edits.
    pub read_only_companion: bool,
    /// The desktop host stays authoritative.
    pub host_authoritative: bool,
    /// No payload bodies cross the export boundary.
    pub no_payload_bodies: bool,
}

/// Security and privacy review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionTriageSecurityReview {
    /// The companion surface is read-only.
    pub companion_read_only: bool,
    /// No editor authority is exposed on the companion.
    pub no_editor_authority_on_companion: bool,
    /// The desktop host stays authoritative.
    pub host_stays_authoritative: bool,
    /// Exact desktop handoff is preserved or honestly degraded.
    pub exact_desktop_handoff_preserved: bool,
    /// No payload bodies cross the export boundary.
    pub no_payload_bodies_in_export: bool,
    /// Degraded state is labeled rather than hidden.
    pub degraded_state_labeled: bool,
    /// Downgrade narrows the claim rather than hiding the surface.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Every section discloses local, staged, and provider/admin continuity.
    pub locality_disclosed: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionTriageConsumerProjection {
    /// Browser companion projects the triage surface.
    pub browser_companion_shows_triage: bool,
    /// Mobile companion projects the triage surface.
    pub mobile_companion_shows_triage: bool,
    /// Desktop panel shows the handoff targets.
    pub desktop_panel_shows_handoff_target: bool,
    /// Support export shows section and item state.
    pub support_export_shows_state: bool,
    /// Diagnostics shows degraded labels.
    pub diagnostics_shows_degraded_labels: bool,
    /// Preview / Labs sections are visibly labeled when not qualified Stable.
    pub preview_labs_label_for_unqualified_sections: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionTriageProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the surface.
    pub auto_narrow_on_stale: bool,
}

/// Per-surface observation fed to
/// [`CompanionTriageSurfacePacket::apply_companion_degradation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompanionSurfaceObservation {
    /// True when the companion relay is available.
    pub relay_available: bool,
    /// True when proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when an active desktop host session exists.
    pub host_session_active: bool,
    /// True when workspace and device trust are intact.
    pub trust_intact: bool,
    /// True when an upstream frozen matrix lane narrowed.
    pub upstream_matrix_narrowed: bool,
}

/// Constructor input for [`CompanionTriageSurfacePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompanionTriageSurfacePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer surfaces that project this packet.
    pub projected_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Per-section qualification rows.
    pub section_qualifications: Vec<CompanionSectionQualification>,
    /// Notification triage items.
    pub notification_triage: Vec<CompanionNotificationItem>,
    /// Review-queue items.
    pub review_queue: Vec<CompanionReviewQueueItem>,
    /// CI-status cards.
    pub ci_status_cards: Vec<CompanionCiStatusCard>,
    /// Handoff contract.
    pub handoff_contract: CompanionHandoffContract,
    /// Locality disclosure.
    pub locality_disclosure: M5CompanionLocalityDisclosure,
    /// Security review block.
    pub security_review: CompanionTriageSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: CompanionTriageConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CompanionTriageProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe companion notification triage, review queue, and CI-status surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionTriageSurfacePacket {
    /// Record kind; must equal [`COMPANION_TRIAGE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`COMPANION_TRIAGE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer surfaces that project this packet.
    pub projected_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Per-section qualification rows.
    pub section_qualifications: Vec<CompanionSectionQualification>,
    /// Notification triage items.
    pub notification_triage: Vec<CompanionNotificationItem>,
    /// Review-queue items.
    pub review_queue: Vec<CompanionReviewQueueItem>,
    /// CI-status cards.
    pub ci_status_cards: Vec<CompanionCiStatusCard>,
    /// Handoff contract.
    pub handoff_contract: CompanionHandoffContract,
    /// Locality disclosure.
    pub locality_disclosure: M5CompanionLocalityDisclosure,
    /// Security review block.
    pub security_review: CompanionTriageSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: CompanionTriageConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CompanionTriageProofFreshness,
    /// Degraded-state labels currently applied (empty when fully qualified).
    pub degraded_labels: Vec<CompanionDegradedReason>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl CompanionTriageSurfacePacket {
    /// Builds a companion triage surface packet from stable-lane input.
    pub fn new(input: CompanionTriageSurfacePacketInput) -> Self {
        Self {
            record_kind: COMPANION_TRIAGE_RECORD_KIND.to_owned(),
            schema_version: COMPANION_TRIAGE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            projected_surfaces: input.projected_surfaces,
            section_qualifications: input.section_qualifications,
            notification_triage: input.notification_triage,
            review_queue: input.review_queue,
            ci_status_cards: input.ci_status_cards,
            handoff_contract: input.handoff_contract,
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

    /// Narrows sections and downgrades handoff resolution from a per-surface
    /// observation, recording the reasons in [`Self::degraded_labels`].
    ///
    /// An unavailable relay, stale proof, or narrowed upstream matrix lane narrows
    /// every section's qualification and rollout stage one step and forces stale
    /// CI cards. A narrowed trust additionally narrows the review queue. An
    /// inactive host session downgrades the resolution of every handoff that
    /// requires an active host, so an exact handoff is never claimed when it can no
    /// longer resolve. Degraded state is labeled, never hidden.
    pub fn apply_companion_degradation(&mut self, observation: &CompanionSurfaceObservation) {
        let mut labels: BTreeSet<CompanionDegradedReason> =
            self.degraded_labels.iter().copied().collect();

        let surface_adverse = !observation.relay_available
            || !observation.proof_fresh
            || observation.upstream_matrix_narrowed;

        if !observation.relay_available {
            labels.insert(CompanionDegradedReason::RelayUnavailable);
            for card in &mut self.ci_status_cards {
                card.freshness = CompanionCardFreshness::Stale;
            }
        }
        if !observation.proof_fresh {
            labels.insert(CompanionDegradedReason::ProofStale);
        }
        if observation.upstream_matrix_narrowed {
            labels.insert(CompanionDegradedReason::UpstreamMatrixNarrowed);
        }
        if !observation.trust_intact {
            labels.insert(CompanionDegradedReason::TrustNarrowed);
        }

        for row in &mut self.section_qualifications {
            let adverse = surface_adverse
                || (!observation.trust_intact
                    && row.section == CompanionTriageSection::ReviewQueue);
            if adverse {
                row.qualification = row.qualification.narrowed_one_step();
                row.rollout_stage = row.rollout_stage.narrowed_one_step();
            }
        }

        if !observation.host_session_active {
            labels.insert(CompanionDegradedReason::HostSessionInactive);
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
                labels.insert(CompanionDegradedReason::HandoffTargetUnresolved);
            }
        }

        self.degraded_labels = labels.into_iter().collect();
    }

    /// Validates the companion triage surface invariants.
    pub fn validate(&self) -> Vec<CompanionTriageViolation> {
        let mut violations = Vec::new();

        if self.record_kind != COMPANION_TRIAGE_RECORD_KIND {
            violations.push(CompanionTriageViolation::WrongRecordKind);
        }
        if self.schema_version != COMPANION_TRIAGE_SCHEMA_VERSION {
            violations.push(CompanionTriageViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(CompanionTriageViolation::MissingIdentity);
        }
        if self.projected_surfaces.is_empty() {
            violations.push(CompanionTriageViolation::ProjectedSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_section_qualifications(self, &mut violations);
        validate_items(self, &mut violations);
        validate_handoff_contract(self, &mut violations);
        validate_locality(self, &mut violations);
        validate_security_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("companion triage packet serializes"),
        ) {
            violations.push(CompanionTriageViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("companion triage packet serializes")
    }

    /// Sections currently publishable (Stable, Beta, or Preview) and not withheld.
    pub fn publishable_sections(&self) -> impl Iterator<Item = &CompanionSectionQualification> {
        self.section_qualifications.iter().filter(|row| {
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

    /// Iterates every handoff across all three sections, in section order.
    pub fn handoffs(&self) -> impl Iterator<Item = &CompanionDesktopHandoff> {
        self.notification_triage
            .iter()
            .map(|item| &item.handoff)
            .chain(self.review_queue.iter().map(|item| &item.handoff))
            .chain(self.ci_status_cards.iter().map(|card| &card.handoff))
    }

    fn handoffs_mut(&mut self) -> impl Iterator<Item = &mut CompanionDesktopHandoff> {
        self.notification_triage
            .iter_mut()
            .map(|item| &mut item.handoff)
            .chain(self.review_queue.iter_mut().map(|item| &mut item.handoff))
            .chain(
                self.ci_status_cards
                    .iter_mut()
                    .map(|card| &mut card.handoff),
            )
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Companion Notification Triage, Review Queues, and CI-Status Cards\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Sections: {} | Notifications: {} | Review queue: {} | CI cards: {}\n",
            self.section_qualifications.len(),
            self.notification_triage.len(),
            self.review_queue.len(),
            self.ci_status_cards.len(),
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

        out.push_str("\n## Sections\n\n");
        for row in &self.section_qualifications {
            out.push_str(&format!(
                "- **{}**: `{}` / `{}` (matrix lane `{}`)\n",
                row.section.as_str(),
                row.qualification.as_str(),
                row.rollout_stage.as_str(),
                row.matrix_lane_ref,
            ));
        }

        out.push_str("\n## Notification triage\n\n");
        for item in &self.notification_triage {
            out.push_str(&format!(
                "- `{}` [{}/{}] {} — {} → `{}` ({})\n",
                item.item_id,
                item.category.as_str(),
                item.priority.as_str(),
                item.triage_state.as_str(),
                item.headline,
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Review queue\n\n");
        for item in &self.review_queue {
            out.push_str(&format!(
                "- `{}` [{}] {} ({}) — {} → `{}` ({})\n",
                item.item_id,
                item.review_kind.as_str(),
                item.queue_state.as_str(),
                item.decision_authority.as_str(),
                item.summary,
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## CI-status cards\n\n");
        for card in &self.ci_status_cards {
            out.push_str(&format!(
                "- `{}` {} — `{}` ({}, {} failing) → `{}` ({})\n",
                card.card_id,
                card.pipeline_label,
                card.status.as_str(),
                card.freshness.as_str(),
                card.failing_check_count,
                card.handoff.target.as_str(),
                card.handoff.resolution.as_str(),
            ));
        }

        out
    }
}

/// Errors emitted when reading the checked-in companion triage export.
#[derive(Debug)]
pub enum CompanionTriageArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CompanionTriageViolation>),
}

impl fmt::Display for CompanionTriageArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "companion triage export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "companion triage export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for CompanionTriageArtifactError {}

/// Validation failures emitted by [`CompanionTriageSurfacePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompanionTriageViolation {
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
    /// A required section qualification row is missing.
    RequiredSectionMissing,
    /// A section row's matrix lane ref does not match its section.
    SectionLaneMismatch,
    /// A section row is incomplete.
    SectionRowIncomplete,
    /// A section has no content items.
    SectionContentMissing,
    /// A notification item is not read-only.
    NotificationNotReadOnly,
    /// An item is missing identity or a redacted body, or has a payload-like body.
    ItemIncomplete,
    /// An item's handoff is missing its deep-link ref.
    HandoffRefMissing,
    /// The handoff contract is not fully satisfied.
    HandoffContractIncomplete,
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

impl CompanionTriageViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::ProjectedSurfacesMissing => "projected_surfaces_missing",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSectionMissing => "required_section_missing",
            Self::SectionLaneMismatch => "section_lane_mismatch",
            Self::SectionRowIncomplete => "section_row_incomplete",
            Self::SectionContentMissing => "section_content_missing",
            Self::NotificationNotReadOnly => "notification_not_read_only",
            Self::ItemIncomplete => "item_incomplete",
            Self::HandoffRefMissing => "handoff_ref_missing",
            Self::HandoffContractIncomplete => "handoff_contract_incomplete",
            Self::LocalityDisclosureIncomplete => "locality_disclosure_incomplete",
            Self::SecurityReviewIncomplete => "security_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable companion triage surface export.
///
/// This is the canonical reader: a browser/mobile companion, the desktop panel,
/// diagnostics, support-export, or Help/About surface calls it to ingest the
/// triage packet rather than cloning status text.
///
/// # Errors
///
/// Returns [`CompanionTriageArtifactError`] when the checked-in support export
/// fails to parse or fails validation.
pub fn current_stable_companion_triage_surface_export(
) -> Result<CompanionTriageSurfacePacket, CompanionTriageArtifactError> {
    let packet: CompanionTriageSurfacePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/companion/m5/companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff/support_export.json"
    )))
    .map_err(CompanionTriageArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CompanionTriageArtifactError::Validation(violations))
    }
}

/// Canonical source contract refs that every triage export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        COMPANION_TRIAGE_SCHEMA_REF.to_owned(),
        COMPANION_TRIAGE_DOC_REF.to_owned(),
        M5_COMPANION_SURFACE_CONTRACT_REF.to_owned(),
        M5_COMPANION_QUALIFICATION_REF.to_owned(),
        M5_COMPANION_BOUNDARY_MANIFEST_REF.to_owned(),
        M5_COMPANION_MATRIX_SCHEMA_REF.to_owned(),
    ]
}

/// Canonical handoff contract with every guarantee satisfied.
pub fn canonical_handoff_contract() -> CompanionHandoffContract {
    CompanionHandoffContract {
        exact_target_required: true,
        read_only_companion: true,
        host_authoritative: true,
        no_payload_bodies: true,
    }
}

/// Canonical security review block with every invariant satisfied.
pub fn canonical_security_review() -> CompanionTriageSecurityReview {
    CompanionTriageSecurityReview {
        companion_read_only: true,
        no_editor_authority_on_companion: true,
        host_stays_authoritative: true,
        exact_desktop_handoff_preserved: true,
        no_payload_bodies_in_export: true,
        degraded_state_labeled: true,
        downgrade_narrows_instead_of_hides: true,
        locality_disclosed: true,
    }
}

/// Canonical consumer projection block with every surface projecting truth.
pub fn canonical_consumer_projection() -> CompanionTriageConsumerProjection {
    CompanionTriageConsumerProjection {
        browser_companion_shows_triage: true,
        mobile_companion_shows_triage: true,
        desktop_panel_shows_handoff_target: true,
        support_export_shows_state: true,
        diagnostics_shows_degraded_labels: true,
        preview_labs_label_for_unqualified_sections: true,
    }
}

/// Canonical per-section qualification rows, inherited from the frozen matrix.
pub fn canonical_section_qualifications() -> Vec<CompanionSectionQualification> {
    use M5CompanionDowngradeTrigger as Trigger;
    use M5CompanionQualificationClass as Qual;
    use M5CompanionRollbackPosture as Rollback;
    use M5CompanionRolloutStage as Stage;

    vec![
        CompanionSectionQualification {
            section: CompanionTriageSection::NotificationTriage,
            qualification: Qual::Stable,
            rollout_stage: Stage::GeneralAvailability,
            matrix_lane_ref: CompanionTriageSection::NotificationTriage
                .matrix_lane()
                .as_str()
                .to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::CompanionScopeExpansionUnqualified,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::CompanionReadOnlyNarrowScope,
        },
        CompanionSectionQualification {
            section: CompanionTriageSection::ReviewQueue,
            qualification: Qual::Stable,
            rollout_stage: Stage::GeneralAvailability,
            matrix_lane_ref: CompanionTriageSection::ReviewQueue
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
        CompanionSectionQualification {
            section: CompanionTriageSection::CiStatusCards,
            qualification: Qual::Beta,
            rollout_stage: Stage::StagedRollout,
            matrix_lane_ref: CompanionTriageSection::CiStatusCards
                .matrix_lane()
                .as_str()
                .to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::CompanionReadOnlyNarrowScope,
        },
    ]
}

/// Canonical locality disclosure for the triage surface.
pub fn canonical_locality_disclosure() -> M5CompanionLocalityDisclosure {
    M5CompanionLocalityDisclosure {
        stays_local:
            "Notification, review, and CI source events are computed and owned by the local core and stay inspectable offline."
                .to_owned(),
        staged:
            "Companion fan-out of triage, review queues, and CI cards rolls out per cohort and capability gate."
                .to_owned(),
        requires_provider_or_admin_continuity:
            "Live delivery and exact desktop handoff require the companion relay and an active host session; the local core never depends on them to function."
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

/// Canonical notification triage items.
pub fn canonical_notification_triage() -> Vec<CompanionNotificationItem> {
    use CompanionHandoffTarget as Target;
    use CompanionNotificationCategory as Category;
    use CompanionTriagePriority as Priority;
    use CompanionTriageState as State;

    vec![
        CompanionNotificationItem {
            item_id: "notif:build:0001".to_owned(),
            category: Category::Build,
            priority: Priority::High,
            triage_state: State::Unread,
            headline: "Build failed on the active workspace".to_owned(),
            source_event_ref: "event:build:0001".to_owned(),
            read_only: true,
            handoff: handoff(Target::CiPipeline, "handoff:ci-pipeline:0001", false),
        },
        CompanionNotificationItem {
            item_id: "notif:review:0002".to_owned(),
            category: Category::Review,
            priority: Priority::Normal,
            triage_state: State::Triaged,
            headline: "Review requested on a pending change".to_owned(),
            source_event_ref: "event:review:0002".to_owned(),
            read_only: true,
            handoff: handoff(Target::ReviewPanel, "handoff:review-panel:0002", true),
        },
        CompanionNotificationItem {
            item_id: "notif:agent:0003".to_owned(),
            category: Category::Agent,
            priority: Priority::Normal,
            triage_state: State::Unread,
            headline: "Agent run finished and is awaiting review".to_owned(),
            source_event_ref: "event:agent:0003".to_owned(),
            read_only: true,
            handoff: handoff(Target::AgentSession, "handoff:agent-session:0003", true),
        },
        CompanionNotificationItem {
            item_id: "notif:incident:0004".to_owned(),
            category: Category::Incident,
            priority: Priority::Critical,
            triage_state: State::EscalatedToDesktop,
            headline: "Incident raised from a crash trail".to_owned(),
            source_event_ref: "event:incident:0004".to_owned(),
            read_only: true,
            handoff: handoff(
                Target::IncidentWorkspace,
                "handoff:incident-workspace:0004",
                false,
            ),
        },
    ]
}

/// Canonical review-queue items.
pub fn canonical_review_queue() -> Vec<CompanionReviewQueueItem> {
    use CompanionDecisionAuthority as Authority;
    use CompanionHandoffTarget as Target;
    use CompanionReviewKind as Kind;
    use CompanionReviewQueueState as QueueState;

    vec![
        CompanionReviewQueueItem {
            item_id: "review:0001".to_owned(),
            review_kind: Kind::AgentChange,
            queue_state: QueueState::Pending,
            decision_authority: Authority::ApproveOrDefer,
            summary: "Agent change set staged for approval".to_owned(),
            evidence_ref: "evidence:agent-change:0001".to_owned(),
            handoff: handoff(Target::FileLocation, "handoff:file-location:0001", true),
        },
        CompanionReviewQueueItem {
            item_id: "review:0002".to_owned(),
            review_kind: Kind::DiffReview,
            queue_state: QueueState::Pending,
            decision_authority: Authority::ApproveOrDefer,
            summary: "Diff awaiting review".to_owned(),
            evidence_ref: "evidence:diff:0002".to_owned(),
            handoff: handoff(Target::ReviewPanel, "handoff:review-panel:r0002", true),
        },
        CompanionReviewQueueItem {
            item_id: "review:0003".to_owned(),
            review_kind: Kind::ApprovalRequest,
            queue_state: QueueState::Deferred,
            decision_authority: Authority::EscalateOnly,
            summary: "Approval request deferred to the desktop host".to_owned(),
            evidence_ref: "evidence:approval:0003".to_owned(),
            handoff: handoff(Target::ReviewPanel, "handoff:review-panel:r0003", true),
        },
    ]
}

/// Canonical CI-status cards.
pub fn canonical_ci_status_cards() -> Vec<CompanionCiStatusCard> {
    use CompanionCardFreshness as Freshness;
    use CompanionCiStatus as Status;
    use CompanionHandoffTarget as Target;

    vec![
        CompanionCiStatusCard {
            card_id: "ci:main:0001".to_owned(),
            pipeline_label: "main".to_owned(),
            status: Status::Passed,
            freshness: Freshness::Live,
            failing_check_count: 0,
            handoff: handoff(Target::CiPipeline, "handoff:ci-pipeline:main", false),
        },
        CompanionCiStatusCard {
            card_id: "ci:pr:0002".to_owned(),
            pipeline_label: "pull-request".to_owned(),
            status: Status::Failed,
            freshness: Freshness::Live,
            failing_check_count: 2,
            handoff: handoff(Target::CiPipeline, "handoff:ci-pipeline:pr", false),
        },
        CompanionCiStatusCard {
            card_id: "ci:nightly:0003".to_owned(),
            pipeline_label: "nightly".to_owned(),
            status: Status::Running,
            freshness: Freshness::Cached,
            failing_check_count: 0,
            handoff: handoff(Target::CiPipeline, "handoff:ci-pipeline:nightly", false),
        },
    ]
}

/// Builds the canonical companion triage surface packet.
///
/// This is the first consumer: it mints the surface the checked-in support export
/// and Markdown summary are generated from, so the artifact never drifts from the
/// typed section, item, and handoff definitions.
pub fn canonical_companion_triage_surface(
    packet_id: String,
    surface_label: String,
    minted_at: String,
    proof_freshness: CompanionTriageProofFreshness,
) -> CompanionTriageSurfacePacket {
    CompanionTriageSurfacePacket::new(CompanionTriageSurfacePacketInput {
        packet_id,
        surface_label,
        projected_surfaces: vec![
            M5CompanionConsumerSurface::BrowserCompanion,
            M5CompanionConsumerSurface::MobileCompanion,
            M5CompanionConsumerSurface::DesktopCompanionPanel,
            M5CompanionConsumerSurface::SupportExport,
            M5CompanionConsumerSurface::Diagnostics,
        ],
        section_qualifications: canonical_section_qualifications(),
        notification_triage: canonical_notification_triage(),
        review_queue: canonical_review_queue(),
        ci_status_cards: canonical_ci_status_cards(),
        handoff_contract: canonical_handoff_contract(),
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
    packet: &CompanionTriageSurfacePacket,
    violations: &mut Vec<CompanionTriageViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        COMPANION_TRIAGE_SCHEMA_REF,
        COMPANION_TRIAGE_DOC_REF,
        M5_COMPANION_SURFACE_CONTRACT_REF,
        M5_COMPANION_QUALIFICATION_REF,
        M5_COMPANION_MATRIX_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(CompanionTriageViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_section_qualifications(
    packet: &CompanionTriageSurfacePacket,
    violations: &mut Vec<CompanionTriageViolation>,
) {
    let present: BTreeSet<CompanionTriageSection> = packet
        .section_qualifications
        .iter()
        .map(|row| row.section)
        .collect();
    for required in CompanionTriageSection::ALL {
        if !present.contains(&required) {
            violations.push(CompanionTriageViolation::RequiredSectionMissing);
            return;
        }
    }

    for row in &packet.section_qualifications {
        if row.matrix_lane_ref != row.section.matrix_lane().as_str() {
            violations.push(CompanionTriageViolation::SectionLaneMismatch);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(CompanionTriageViolation::SectionRowIncomplete);
        }
    }
}

fn validate_items(
    packet: &CompanionTriageSurfacePacket,
    violations: &mut Vec<CompanionTriageViolation>,
) {
    if packet.notification_triage.is_empty()
        || packet.review_queue.is_empty()
        || packet.ci_status_cards.is_empty()
    {
        violations.push(CompanionTriageViolation::SectionContentMissing);
    }

    for item in &packet.notification_triage {
        if !item.read_only {
            violations.push(CompanionTriageViolation::NotificationNotReadOnly);
        }
        if item.item_id.trim().is_empty()
            || item.headline.trim().is_empty()
            || item.source_event_ref.trim().is_empty()
        {
            violations.push(CompanionTriageViolation::ItemIncomplete);
        }
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.review_queue {
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.evidence_ref.trim().is_empty()
        {
            violations.push(CompanionTriageViolation::ItemIncomplete);
        }
        validate_handoff(&item.handoff, violations);
    }

    for card in &packet.ci_status_cards {
        if card.card_id.trim().is_empty() || card.pipeline_label.trim().is_empty() {
            violations.push(CompanionTriageViolation::ItemIncomplete);
        }
        validate_handoff(&card.handoff, violations);
    }
}

fn validate_handoff(
    handoff: &CompanionDesktopHandoff,
    violations: &mut Vec<CompanionTriageViolation>,
) {
    if handoff.deep_link_ref.trim().is_empty() {
        violations.push(CompanionTriageViolation::HandoffRefMissing);
    }
}

fn validate_handoff_contract(
    packet: &CompanionTriageSurfacePacket,
    violations: &mut Vec<CompanionTriageViolation>,
) {
    let contract = &packet.handoff_contract;
    for ok in [
        contract.exact_target_required,
        contract.read_only_companion,
        contract.host_authoritative,
        contract.no_payload_bodies,
    ] {
        if !ok {
            violations.push(CompanionTriageViolation::HandoffContractIncomplete);
            return;
        }
    }
}

fn validate_locality(
    packet: &CompanionTriageSurfacePacket,
    violations: &mut Vec<CompanionTriageViolation>,
) {
    let locality = &packet.locality_disclosure;
    if locality.stays_local.trim().is_empty()
        || locality.staged.trim().is_empty()
        || locality
            .requires_provider_or_admin_continuity
            .trim()
            .is_empty()
    {
        violations.push(CompanionTriageViolation::LocalityDisclosureIncomplete);
    }
}

fn validate_security_review(
    packet: &CompanionTriageSurfacePacket,
    violations: &mut Vec<CompanionTriageViolation>,
) {
    let review = &packet.security_review;
    for ok in [
        review.companion_read_only,
        review.no_editor_authority_on_companion,
        review.host_stays_authoritative,
        review.exact_desktop_handoff_preserved,
        review.no_payload_bodies_in_export,
        review.degraded_state_labeled,
        review.downgrade_narrows_instead_of_hides,
        review.locality_disclosed,
    ] {
        if !ok {
            violations.push(CompanionTriageViolation::SecurityReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &CompanionTriageSurfacePacket,
    violations: &mut Vec<CompanionTriageViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.browser_companion_shows_triage,
        projection.mobile_companion_shows_triage,
        projection.desktop_panel_shows_handoff_target,
        projection.support_export_shows_state,
        projection.diagnostics_shows_degraded_labels,
        projection.preview_labs_label_for_unqualified_sections,
    ] {
        if !ok {
            violations.push(CompanionTriageViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &CompanionTriageSurfacePacket,
    violations: &mut Vec<CompanionTriageViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(CompanionTriageViolation::ProofFreshnessIncomplete);
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
