//! Session role, admission, observer, retention, export, and downgrade qualification.
//!
//! This module embeds the canonical collaboration session-envelope packet used
//! by desktop, browser companion, mobile follow, support export, docs, and
//! Help/About projections. It proves that any exposed session, invite, follow,
//! observer, rejoin, or presenter lane either carries explicit role and
//! retention truth or renders below the Stable contract.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported schema version for session role qualification packets.
pub const SESSION_ROLE_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the qualification packet.
pub const SESSION_ROLE_QUALIFICATION_RECORD_KIND: &str =
    "session_role_admission_and_retention_qualification";

/// Repo-relative path to the canonical packet.
pub const SESSION_ROLE_QUALIFICATION_PATH: &str =
    "artifacts/collab/m4/session-role-admission-and-retention-qualification.json";

/// Embedded canonical packet JSON.
pub const SESSION_ROLE_QUALIFICATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/collab/m4/session-role-admission-and-retention-qualification.json"
));

/// Release label rendered for a collaboration lane after qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationLabel {
    /// Stable surface with a current proof packet and complete envelope truth.
    Stable,
    /// Preview surface that is visible but outside the Stable contract.
    Preview,
    /// Labs surface that is experimental and visibly below Preview.
    Labs,
    /// Surface has been withdrawn or hidden for the promoted build.
    Withdrawn,
}

impl QualificationLabel {
    /// Returns whether the label renders at the Stable cutline.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// Returns a widening rank used for claim-ceiling checks.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Withdrawn => 0,
            Self::Labs => 1,
            Self::Preview => 2,
            Self::Stable => 3,
        }
    }
}

/// M4-exposed collaboration-adjacent lane governed by an envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollaborationLaneKind {
    /// Session invitation card or join link.
    Invitation,
    /// Host lobby and admission review row.
    LobbyAdmission,
    /// Observer-only join or inspect path.
    ObserverJoin,
    /// Presenter, co-presenter, viewer follow, or breakaway path.
    PresenterFollow,
    /// Rejoin path after expiry, reconnect, policy change, or client change.
    Rejoin,
    /// Session export, delete, hold, or support packet path.
    SessionExportDelete,
}

/// Distinct admission lifecycle state for a participant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionState {
    /// Invite was issued and has not yet been accepted.
    Invited,
    /// Participant requested entry and waits in the lobby.
    Requested,
    /// Participant was admitted to the session.
    Admitted,
    /// Participant is admitted as observer only.
    Observer,
    /// Participant or host declined the invitation or request.
    Declined,
    /// Invite or admission window expired.
    Expired,
    /// Invite or grant was revoked.
    Revoked,
    /// Fresh review is required before rejoining or resuming.
    RejoinRequired,
}

/// Shared session lifecycle visible on strips, drawers, and support exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionLifecycleState {
    /// Session is being prepared but is not yet shared.
    PublishPending,
    /// Session is active and within its stated envelope.
    SharedActive,
    /// One or more participants narrowed capability while the session continues.
    ParticipantDegraded,
    /// The shared lane degraded for relay, policy, or client-boundary reasons.
    SharedDegraded,
    /// Live participation ended.
    Ended,
    /// Retained artifacts were sealed according to the envelope.
    Archived,
}

/// Role requested, offered, or active in a session envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionRoleRequested {
    /// View-only observer.
    Observer,
    /// Commenting participant without write or control authority.
    Commenter,
    /// Text editor for declared shared objects.
    Editor,
    /// Navigator who can guide without runtime input authority.
    Navigator,
    /// Driver candidate or active driver over a scoped control plane.
    Driver,
    /// Presenter or moderator of follow state.
    Presenter,
    /// Co-presenter with explicit handoff authority.
    CoPresenter,
    /// Approver who may decide admission or retention changes.
    Approver,
    /// Support agent in an elevated support posture.
    SupportAgent,
}

/// Client boundary disclosed before acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientBoundary {
    /// Full desktop client.
    DesktopNative,
    /// Browser companion client.
    BrowserCompanion,
    /// Mobile companion client.
    MobileCompanion,
    /// Managed admin or support console.
    ManagedAdmin,
    /// External guest client with limited authority.
    ExternalGuest,
}

impl ClientBoundary {
    /// Returns whether the client boundary requires observer-first admission unless proven otherwise.
    pub const fn requires_observer_first(self) -> bool {
        matches!(
            self,
            Self::BrowserCompanion | Self::MobileCompanion | Self::ExternalGuest
        )
    }
}

/// Retention mode disclosed by the session envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionMode {
    /// Live presence only with minimal telemetry.
    LiveOnly,
    /// Metadata audit without retained payloads.
    MetadataAudit,
    /// Replayable text/comment timeline with bounded retained object classes.
    ReplayableTextCommentTimeline,
    /// Elevated support or regulated evidence posture.
    SupportOrRegulatedEvidence,
}

/// Guest scope disclosed by invite and admission surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuestScope {
    /// Organization members only.
    OrgMembersOnly,
    /// Named guests admitted by invite only.
    NamedGuestByInvite,
    /// Approved guest admitted with an approval ticket.
    ApprovedGuestWithTicket,
    /// Public link is read-only and visibly watermarked.
    PublicReadOnlyWatermarked,
    /// Guest admission is blocked by policy.
    GuestForbidden,
}

impl GuestScope {
    /// Returns whether the envelope has an external guest boundary.
    pub const fn is_external(self) -> bool {
        matches!(
            self,
            Self::NamedGuestByInvite
                | Self::ApprovedGuestWithTicket
                | Self::PublicReadOnlyWatermarked
        )
    }
}

/// Export/delete right disclosed separately for local and managed copies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportDeleteRight {
    /// Participant may export their own contributions.
    ExportOwnContributions,
    /// Session owner may export a redacted bundle.
    OwnerRedactedBundle,
    /// Admin may export an audit bundle.
    AdminAuditExport,
    /// Participant may delete a local-only copy.
    DeleteLocalCopy,
    /// Deletion request is available for managed copies.
    RequestManagedDeletion,
    /// Legal hold blocks content deletion while allowing hold summary export.
    LegalHoldBlocksDelete,
    /// Export is unavailable because no durable copy exists.
    NoDurableExport,
}

/// Presenter role disclosed for presentation and guided-follow behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresenterRole {
    /// No presenter role applies to this lane.
    NotApplicable,
    /// Current presenter.
    Presenter,
    /// Explicit co-presenter.
    CoPresenter,
    /// Viewer following presenter.
    Viewer,
    /// Moderator who may manage presenter or audience follow state.
    Moderator,
}

/// Audience follow state disclosed for guided-follow and breakaway behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FollowState {
    /// Follow state does not apply to this lane.
    NotApplicable,
    /// Viewer follows the presenter.
    FollowingPresenter,
    /// Viewer has intentionally browsed independently.
    Breakaway,
    /// Viewer can return to presenter from a visible affordance.
    ReturnAvailable,
    /// Follow is degraded or unavailable on the current client.
    FollowDegraded,
}

/// Trigger that requires a visible consent or review event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsentTrigger {
    /// No fresh consent trigger is pending.
    None,
    /// Retention broadened during the session.
    RetentionBroadened,
    /// External guest was admitted or guest scope widened.
    GuestAdmitted,
    /// Participant role or authority widened.
    RoleWidened,
    /// Route visibility widened.
    RouteVisibilityWidened,
    /// Support or regulated evidence posture was enabled.
    SupportEvidenceEnabled,
}

impl ConsentTrigger {
    /// Returns whether a fresh visible review event is mandatory.
    pub const fn requires_visible_event(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Class of downgrade or narrowing that the envelope must explain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeClass {
    /// No downgrade applies.
    None,
    /// Relay degraded and shared awareness narrowed.
    RelayDegraded,
    /// Policy changed and role or retention must narrow.
    PolicyNarrowed,
    /// Client lacks capability for the requested authority.
    ClientScopeNarrowed,
    /// Guest boundary requires observer-first behavior.
    GuestBoundaryNarrowed,
    /// Invite expired and fresh admission is required.
    InviteExpired,
    /// Legal hold changes delete semantics.
    LegalHoldApplied,
}

/// Local continuity posture when a session narrows or relay degrades.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalContinuityPosture {
    /// Local continuity is not relevant for this row.
    NotApplicable,
    /// Local unsent work remains local-only.
    PreserveUnsentLocalOnly,
    /// Local unsent work is offered as a reviewable patch.
    PreserveAsReviewPatch,
    /// Local notes or navigation history remain available.
    PreserveLocalNotes,
    /// No local mutation occurred, so there is nothing to preserve.
    NoLocalMutation,
}

/// Current, captured proof packet backing a stable envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StableProofPacket {
    /// Stable proof identifier.
    pub packet_id: String,
    /// Repo-relative packet reference.
    pub packet_ref: String,
    /// UTC capture date.
    pub captured_at: String,
    /// Evidence references backing this proof.
    pub evidence_refs: Vec<String>,
    /// Whether the proof is current for the release packet.
    pub current: bool,
}

impl StableProofPacket {
    fn is_green(&self) -> bool {
        self.current && !self.packet_id.trim().is_empty() && !self.evidence_refs.is_empty()
    }
}

/// Local and managed retention/export/delete truth carried separately.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetainedCopyTruth {
    /// Whether a local-only copy exists.
    pub local_copy_present: bool,
    /// Local retention summary shown to the participant.
    pub local_retention: String,
    /// Rights that apply to local copies.
    pub local_rights: Vec<ExportDeleteRight>,
    /// Whether a managed copy exists.
    pub managed_copy_present: bool,
    /// Managed retention summary shown to the participant.
    pub managed_retention: String,
    /// Rights that apply to managed copies.
    pub managed_rights: Vec<ExportDeleteRight>,
    /// Whether legal hold affects deletion.
    pub legal_hold: bool,
}

impl RetainedCopyTruth {
    fn complete(&self) -> bool {
        !self.local_retention.trim().is_empty()
            && !self.managed_retention.trim().is_empty()
            && (!self.local_copy_present || !self.local_rights.is_empty())
            && (!self.managed_copy_present || !self.managed_rights.is_empty())
            && (!self.legal_hold
                || self
                    .managed_rights
                    .contains(&ExportDeleteRight::LegalHoldBlocksDelete)
                || self
                    .local_rights
                    .contains(&ExportDeleteRight::LegalHoldBlocksDelete))
    }
}

/// Role, retention, admission, and downgrade envelope for one visible lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionEnvelopeRecord {
    /// Stable envelope identifier.
    pub envelope_id: String,
    /// Human-readable title.
    pub title: String,
    /// Lane governed by this envelope.
    pub lane_kind: CollaborationLaneKind,
    /// Whether the promoted build exposes the lane.
    pub promoted_build_surface: bool,
    /// Claimed label before qualification.
    pub claim_label: QualificationLabel,
    /// Label after qualification or downgrade.
    pub displayed_label: QualificationLabel,
    /// Current proof packet for stable lanes.
    #[serde(default)]
    pub proof_packet: Option<StableProofPacket>,
    /// Workspace identity visible before acceptance.
    pub workspace_identity: String,
    /// Inviter or approver identity visible before acceptance.
    pub inviter_or_approver: String,
    /// Role requested, offered, or admitted.
    pub requested_role: SessionRoleRequested,
    /// Absolute expiry or rejoin deadline label.
    pub expiry: String,
    /// Client boundary disclosed before acceptance.
    pub client_boundary: ClientBoundary,
    /// Admission lifecycle state.
    pub admission_state: AdmissionState,
    /// Session lifecycle state.
    pub session_state: SessionLifecycleState,
    /// Retention mode disclosed before acceptance.
    pub retention_mode: RetentionMode,
    /// Guest scope disclosed before acceptance.
    pub guest_scope: GuestScope,
    /// Export/delete/hold truth for local and managed copies.
    pub retained_copy_truth: RetainedCopyTruth,
    /// Presenter role for presentation and guided-follow lanes.
    pub presenter_role: PresenterRole,
    /// Audience follow or breakaway state.
    pub follow_state: FollowState,
    /// Consent trigger that caused or would cause a fresh review event.
    pub consent_trigger: ConsentTrigger,
    /// Whether the trigger has a visible consent or review event.
    pub visible_review_event: bool,
    /// Downgrade or narrowing class.
    pub downgrade_class: DowngradeClass,
    /// Explanation of how authority narrows.
    pub downgrade_behavior: String,
    /// Local continuity posture when the session narrows.
    pub local_continuity: LocalContinuityPosture,
    /// Explicit safe actions shown to the participant.
    pub participant_actions: Vec<String>,
    /// Destinations that ingest this envelope rather than cloned text.
    pub projection_destinations: Vec<String>,
    /// Support/export evidence references.
    pub support_export_refs: Vec<String>,
    /// Reviewable rationale.
    pub rationale: String,
}

impl SessionEnvelopeRecord {
    fn proof_is_green(&self) -> bool {
        self.proof_packet
            .as_ref()
            .is_some_and(StableProofPacket::is_green)
    }

    fn has_join_truth(&self) -> bool {
        !self.workspace_identity.trim().is_empty()
            && !self.inviter_or_approver.trim().is_empty()
            && !self.expiry.trim().is_empty()
            && self.retained_copy_truth.complete()
            && !self.participant_actions.is_empty()
    }

    fn has_projection_truth(&self) -> bool {
        let projections: BTreeSet<&str> = self
            .projection_destinations
            .iter()
            .map(String::as_str)
            .collect();
        projections.contains("desktop")
            && projections.contains("browser_companion")
            && projections.contains("mobile_follow")
            && projections.contains("docs_help")
            && projections.contains("support_export")
    }

    fn has_presenter_truth(&self) -> bool {
        self.lane_kind != CollaborationLaneKind::PresenterFollow
            || (self.presenter_role != PresenterRole::NotApplicable
                && self.follow_state != FollowState::NotApplicable
                && self.participant_actions.iter().any(|action| {
                    action.contains("return_to_presenter") || action.contains("stay_independent")
                }))
    }

    fn is_observer_first_or_narrowed(&self) -> bool {
        !self.client_boundary.requires_observer_first()
            || matches!(
                self.requested_role,
                SessionRoleRequested::Observer | SessionRoleRequested::Commenter
            )
            || !self.displayed_label.is_stable()
    }

    fn has_safe_downgrade(&self) -> bool {
        self.downgrade_class == DowngradeClass::None
            || (!self.downgrade_behavior.trim().is_empty()
                && self.local_continuity != LocalContinuityPosture::NotApplicable)
    }
}

/// Summary counts for the canonical qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionRoleQualificationSummary {
    /// Total exposed rows.
    pub exposed_lane_count: usize,
    /// Rows rendering at Stable.
    pub stable_lane_count: usize,
    /// Rows narrowed below Stable.
    pub downgraded_lane_count: usize,
    /// Rows with current proof packets.
    pub green_packet_count: usize,
    /// Rows with fresh visible consent or review events.
    pub fresh_review_event_count: usize,
}

/// Canonical packet for session role/admission/retention qualification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionRoleAdmissionAndRetentionQualification {
    /// Packet schema version.
    pub schema_version: u32,
    /// Packet record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Human-readable artifact reference.
    pub artifact_ref: String,
    /// User-facing documentation reference.
    pub docs_ref: String,
    /// JSON Schema reference.
    pub schema_ref: String,
    /// Fixture directory reference.
    pub fixture_ref: String,
    /// Session-envelope rows.
    pub session_envelopes: Vec<SessionEnvelopeRecord>,
    /// Summary counts.
    pub summary: SessionRoleQualificationSummary,
}

impl SessionRoleAdmissionAndRetentionQualification {
    /// Recomputes the summary from envelope state.
    pub fn computed_summary(&self) -> SessionRoleQualificationSummary {
        let exposed: Vec<&SessionEnvelopeRecord> = self
            .session_envelopes
            .iter()
            .filter(|envelope| envelope.promoted_build_surface)
            .collect();

        SessionRoleQualificationSummary {
            exposed_lane_count: exposed.len(),
            stable_lane_count: exposed
                .iter()
                .filter(|envelope| envelope.displayed_label.is_stable())
                .count(),
            downgraded_lane_count: exposed
                .iter()
                .filter(|envelope| !envelope.displayed_label.is_stable())
                .count(),
            green_packet_count: exposed
                .iter()
                .filter(|envelope| envelope.proof_is_green())
                .count(),
            fresh_review_event_count: exposed
                .iter()
                .filter(|envelope| envelope.visible_review_event)
                .count(),
        }
    }

    /// Validates structural invariants that do not depend on wall-clock arithmetic.
    pub fn validate(&self) -> Vec<SessionRoleQualificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != SESSION_ROLE_QUALIFICATION_SCHEMA_VERSION {
            violations.push(SessionRoleQualificationViolation::SchemaVersion {
                expected: SESSION_ROLE_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }

        if self.record_kind != SESSION_ROLE_QUALIFICATION_RECORD_KIND {
            violations.push(SessionRoleQualificationViolation::RecordKind {
                expected: SESSION_ROLE_QUALIFICATION_RECORD_KIND.to_string(),
                actual: self.record_kind.clone(),
            });
        }

        let mut envelope_ids = BTreeSet::new();
        for envelope in &self.session_envelopes {
            if !envelope_ids.insert(envelope.envelope_id.clone()) {
                violations.push(SessionRoleQualificationViolation::DuplicateEnvelopeId {
                    envelope_id: envelope.envelope_id.clone(),
                });
            }

            if envelope.displayed_label.rank() > envelope.claim_label.rank() {
                violations.push(SessionRoleQualificationViolation::DisplayedWiderThanClaim {
                    envelope_id: envelope.envelope_id.clone(),
                });
            }

            if envelope.promoted_build_surface
                && envelope.displayed_label.is_stable()
                && !envelope.proof_is_green()
            {
                violations.push(
                    SessionRoleQualificationViolation::StableLaneWithoutGreenPacket {
                        envelope_id: envelope.envelope_id.clone(),
                    },
                );
            }

            if !envelope.has_join_truth() {
                violations.push(
                    SessionRoleQualificationViolation::MissingJoinDisclosureTruth {
                        envelope_id: envelope.envelope_id.clone(),
                    },
                );
            }

            if envelope.consent_trigger.requires_visible_event() && !envelope.visible_review_event {
                violations.push(
                    SessionRoleQualificationViolation::MissingFreshConsentEvent {
                        envelope_id: envelope.envelope_id.clone(),
                    },
                );
            }

            if !envelope.is_observer_first_or_narrowed() {
                violations.push(SessionRoleQualificationViolation::ObserverFirstRequired {
                    envelope_id: envelope.envelope_id.clone(),
                });
            }

            if !envelope.has_presenter_truth() {
                violations.push(
                    SessionRoleQualificationViolation::MissingPresenterFollowTruth {
                        envelope_id: envelope.envelope_id.clone(),
                    },
                );
            }

            if !envelope.has_safe_downgrade() {
                violations.push(SessionRoleQualificationViolation::UnsafeDowngradeBehavior {
                    envelope_id: envelope.envelope_id.clone(),
                });
            }

            if envelope.displayed_label.is_stable()
                && (!envelope.has_projection_truth() || envelope.support_export_refs.is_empty())
            {
                violations.push(SessionRoleQualificationViolation::MissingProjectionTruth {
                    envelope_id: envelope.envelope_id.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(SessionRoleQualificationViolation::SummaryMismatch {
                expected: self.computed_summary(),
                actual: self.summary.clone(),
            });
        }

        violations
    }
}

/// Validation error for session role qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionRoleQualificationViolation {
    /// Packet schema version differs from the supported version.
    SchemaVersion { expected: u32, actual: u32 },
    /// Packet record kind differs from the supported kind.
    RecordKind { expected: String, actual: String },
    /// Envelope id appears more than once.
    DuplicateEnvelopeId { envelope_id: String },
    /// Displayed label is wider than the claim label.
    DisplayedWiderThanClaim { envelope_id: String },
    /// Stable lane lacks a current proof packet.
    StableLaneWithoutGreenPacket { envelope_id: String },
    /// Join, lobby, or observer disclosures omit required truth.
    MissingJoinDisclosureTruth { envelope_id: String },
    /// Scope-widening trigger lacks a visible consent event.
    MissingFreshConsentEvent { envelope_id: String },
    /// Client or guest boundary needs observer-first posture or downgrade.
    ObserverFirstRequired { envelope_id: String },
    /// Presenter/follow lane omits presenter, breakaway, or return truth.
    MissingPresenterFollowTruth { envelope_id: String },
    /// Downgrade path does not preserve local continuity or explain narrowing.
    UnsafeDowngradeBehavior { envelope_id: String },
    /// Stable lane is not projected into all required consuming surfaces.
    MissingProjectionTruth { envelope_id: String },
    /// Summary block drifted from envelope state.
    SummaryMismatch {
        expected: SessionRoleQualificationSummary,
        actual: SessionRoleQualificationSummary,
    },
}

impl fmt::Display for SessionRoleQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for SessionRoleQualificationViolation {}

/// Parses the checked-in session role/admission/retention qualification packet.
pub fn current_session_role_admission_and_retention_qualification(
) -> Result<SessionRoleAdmissionAndRetentionQualification, Box<dyn Error + Send + Sync>> {
    Ok(serde_json::from_str(SESSION_ROLE_QUALIFICATION_JSON)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_packet_is_valid() {
        let packet = current_session_role_admission_and_retention_qualification()
            .expect("canonical session role qualification packet parses");

        assert_eq!(packet.validate(), Vec::new());
    }
}
