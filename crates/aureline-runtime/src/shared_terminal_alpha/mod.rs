//! Shared-terminal control-channel skeleton with explicit grants and
//! presenter-handoff audit.
//!
//! The collaboration control-grant boundary at
//! [`/schemas/collaboration/control_grant.schema.json`](../../../../schemas/collaboration/control_grant.schema.json)
//! already names the closed grant / revocation / audit vocabulary. This
//! module owns the bounded alpha record family that consumes that
//! boundary on the runtime side: each shared terminal pane that any other
//! actor can observe or drive is projected through one
//! [`SharedTerminalAlphaPage`] so reviewers, support exports, and the
//! session UI read one truth.
//!
//! The alpha promise is narrow and strict:
//!
//! - Every participant on a shared terminal resolves to exactly one
//!   [`SharedTerminalControlState`] in one of the four typed states
//!   ([`SharedTerminalControlStateClass::ViewOnlyObserver`],
//!   [`SharedTerminalControlStateClass::RequestControlPending`],
//!   [`SharedTerminalControlStateClass::ActiveControlGrantee`], or
//!   [`SharedTerminalControlStateClass::ControlRevoked`]). Control is
//!   never inferred from presence or presenter state; an active-control
//!   row that does not cite a `control_grant_ref` is refused at validate
//!   time, mirroring the
//!   `control_grant_not_inferable_from_presence_or_presenter_state`
//!   invariant on the upstream control-grant contract.
//! - Presenter handoff and control state changes mint typed
//!   [`SharedTerminalAuditEvent`] rows on the local audit stream so the
//!   session UI, support export, and admin-collaboration surfaces can
//!   inspect the same event ids. Presenter handoff is recorded as an
//!   explicit [`PresenterHandoffEvent`] with a closed outcome class.
//! - When control ends or degrades (revocation, relay outage, session
//!   end), the bound row records a
//!   [`LocalTerminalContinuityObservation`] using a closed vocabulary so
//!   local terminal continuity is preserved without silent authority
//!   widening.
//!
//! The cross-tool boundary lives at
//! [`/schemas/runtime/shared_terminal_control_alpha.schema.json`](../../../../schemas/runtime/shared_terminal_control_alpha.schema.json).
//! The reviewer-facing landing page lives at
//! [`/docs/runtime/m3/shared_terminal_alpha.md`](../../../../docs/runtime/m3/shared_terminal_alpha.md).
//! The reviewer fixture lives at
//! [`/fixtures/runtime/shared_terminal_alpha/page.json`](../../../../fixtures/runtime/shared_terminal_alpha/page.json).

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Alpha schema version exported with every shared-terminal control
/// record.
pub const SHARED_TERMINAL_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every record in this family.
pub const SHARED_TERMINAL_ALPHA_SHARED_CONTRACT_REF: &str =
    "runtime:shared_terminal_control_alpha:v1";

/// Stable record-kind tag for [`SharedTerminalAlphaPage`] payloads.
pub const SHARED_TERMINAL_ALPHA_PAGE_RECORD_KIND: &str =
    "shared_terminal_control_alpha_page_record";

/// Stable record-kind tag for [`SharedTerminalControlState`] payloads.
pub const SHARED_TERMINAL_ALPHA_CONTROL_STATE_RECORD_KIND: &str =
    "shared_terminal_control_alpha_state_record";

/// Stable record-kind tag for [`PresenterHandoffEvent`] payloads.
pub const SHARED_TERMINAL_ALPHA_PRESENTER_HANDOFF_RECORD_KIND: &str =
    "shared_terminal_control_alpha_presenter_handoff_record";

/// Stable record-kind tag for [`SharedTerminalAuditEvent`] payloads.
pub const SHARED_TERMINAL_ALPHA_AUDIT_EVENT_RECORD_KIND: &str =
    "shared_terminal_control_alpha_audit_event_record";

/// Stable record-kind tag for [`LocalTerminalContinuityObservation`]
/// payloads.
pub const SHARED_TERMINAL_ALPHA_CONTINUITY_OBSERVATION_RECORD_KIND: &str =
    "shared_terminal_control_alpha_continuity_observation_record";

/// Stable record-kind tag for [`SharedTerminalAlphaValidationReport`]
/// payloads.
pub const SHARED_TERMINAL_ALPHA_VALIDATION_REPORT_RECORD_KIND: &str =
    "shared_terminal_control_alpha_validation_report";

/// Stable record-kind tag for the redaction-safe support-export
/// projection.
pub const SHARED_TERMINAL_ALPHA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shared_terminal_control_alpha_support_export";

/// The four typed control states the alpha row distinguishes for one
/// participant on one shared terminal pane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedTerminalControlStateClass {
    /// Participant is observing the transcript only; no input authority.
    ViewOnlyObserver,
    /// Participant has requested control; awaiting grantor admission.
    RequestControlPending,
    /// Participant holds a typed control grant and is actively driving
    /// within the lane and action set the grant admits.
    ActiveControlGrantee,
    /// Control was revoked (owner, approver, policy, admin, session
    /// end, transport drop). Mutation is refused; observation continues.
    ControlRevoked,
}

impl SharedTerminalControlStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ViewOnlyObserver => "view_only_observer",
            Self::RequestControlPending => "request_control_pending",
            Self::ActiveControlGrantee => "active_control_grantee",
            Self::ControlRevoked => "control_revoked",
        }
    }

    /// True when the row must cite a control_grant_ref.
    pub const fn requires_control_grant(self) -> bool {
        matches!(self, Self::ActiveControlGrantee | Self::ControlRevoked)
    }

    /// True when the row must cite a revocation_ref.
    pub const fn requires_revocation(self) -> bool {
        matches!(self, Self::ControlRevoked)
    }

    /// True when the row must cite a pending_request_ref instead of a
    /// grant.
    pub const fn requires_pending_request(self) -> bool {
        matches!(self, Self::RequestControlPending)
    }
}

/// Closed vocabulary for one presenter-handoff outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresenterHandoffOutcomeClass {
    /// Destination actor accepted the presenter role.
    PresenterRoleAccepted,
    /// Destination actor declined the presenter role.
    PresenterRoleDeclined,
    /// Presenter stepped away; the role auto-downgraded to view-only.
    PresenterRoleAutoObserver,
    /// Owner / approver / admin revoked the presenter role.
    PresenterRoleRevoked,
    /// Session ended before the handoff completed.
    PresenterRoleExpiredSessionEnd,
}

impl PresenterHandoffOutcomeClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PresenterRoleAccepted => "presenter_role_accepted",
            Self::PresenterRoleDeclined => "presenter_role_declined",
            Self::PresenterRoleAutoObserver => "presenter_role_auto_observer",
            Self::PresenterRoleRevoked => "presenter_role_revoked",
            Self::PresenterRoleExpiredSessionEnd => "presenter_role_expired_session_end",
        }
    }

    /// True when the handoff must cite a destination_actor_ref.
    pub const fn requires_destination_actor(self) -> bool {
        matches!(self, Self::PresenterRoleAccepted)
    }

    /// True when the handoff must cite a decline_reason_label.
    pub const fn requires_decline_reason(self) -> bool {
        matches!(self, Self::PresenterRoleDeclined)
    }

    /// True when the handoff must cite a revocation_cause_label.
    pub const fn requires_revocation_cause(self) -> bool {
        matches!(self, Self::PresenterRoleRevoked)
    }
}

/// Closed vocabulary for one audit event on the shared-terminal control
/// stream. Mirrors the upstream `control_grant_audit_event_id` shape and
/// adds presenter-handoff events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedTerminalAuditEventClass {
    /// Participant requested control.
    ControlRequested,
    /// Owner / approver admitted the request.
    ControlRequestAdmitted,
    /// Owner / approver denied the request.
    ControlRequestDenied,
    /// Active control session started (after admission).
    ControlActiveStarted,
    /// Active control session ended (single-shot spent or hand-back).
    ControlActiveEnded,
    /// Owner / approver / policy / admin revoked control.
    ControlRevoked,
    /// Active control expired with session end (hard cap).
    ControlExpiredSessionEnd,
    /// Presenter-handoff invitation minted.
    PresenterHandoffInitiated,
    /// Presenter-handoff invitation resolved (accepted / declined / etc.).
    PresenterHandoffResolved,
    /// A denial was emitted (invalid request, replay after revoke).
    AuditDenialEmitted,
}

impl SharedTerminalAuditEventClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlRequested => "control_requested",
            Self::ControlRequestAdmitted => "control_request_admitted",
            Self::ControlRequestDenied => "control_request_denied",
            Self::ControlActiveStarted => "control_active_started",
            Self::ControlActiveEnded => "control_active_ended",
            Self::ControlRevoked => "control_revoked",
            Self::ControlExpiredSessionEnd => "control_expired_session_end",
            Self::PresenterHandoffInitiated => "presenter_handoff_initiated",
            Self::PresenterHandoffResolved => "presenter_handoff_resolved",
            Self::AuditDenialEmitted => "audit_denial_emitted",
        }
    }

    /// True when the audit event must cite a denial_reason_label.
    pub const fn requires_denial_reason(self) -> bool {
        matches!(self, Self::ControlRequestDenied | Self::AuditDenialEmitted)
    }

    /// True when the event must cite a presenter_handoff_ref.
    pub const fn requires_handoff_ref(self) -> bool {
        matches!(
            self,
            Self::PresenterHandoffInitiated | Self::PresenterHandoffResolved
        )
    }

    /// True when the event must cite a control_state_ref.
    pub const fn requires_state_ref(self) -> bool {
        matches!(
            self,
            Self::ControlRequested
                | Self::ControlRequestAdmitted
                | Self::ControlRequestDenied
                | Self::ControlActiveStarted
                | Self::ControlActiveEnded
                | Self::ControlRevoked
                | Self::ControlExpiredSessionEnd
        )
    }
}

/// Closed vocabulary for one local-terminal continuity observation. Names
/// how local terminal authority is preserved when shared control ends or
/// degrades.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalContinuityClass {
    /// Owner's local input survived a grantee revocation; the grantee
    /// loses input but the owner can keep driving.
    OwnerInputPreservedAfterGranteeRevoked,
    /// Grantee was demoted to observer; their local prompt continued
    /// without input authority (no silent input injection).
    GranteeDemotedObserverNoInputInjection,
    /// Relay outage forced a non-replayable hand-back; the owner's local
    /// shell continues from the last observed line.
    LocalShellResumedAfterRelayOutage,
    /// Session ended; the bound local pane returns to single-user
    /// authority and continues observation-only on transcript.
    LocalAuthorityRestoredAfterSessionEnd,
    /// Control window expired (single-shot spent or duration window
    /// closed); local authority returned to the owner without resending
    /// in-flight bytes.
    LocalAuthorityRestoredAfterExpiry,
}

impl LocalContinuityClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerInputPreservedAfterGranteeRevoked => {
                "owner_input_preserved_after_grantee_revoked"
            }
            Self::GranteeDemotedObserverNoInputInjection => {
                "grantee_demoted_observer_no_input_injection"
            }
            Self::LocalShellResumedAfterRelayOutage => "local_shell_resumed_after_relay_outage",
            Self::LocalAuthorityRestoredAfterSessionEnd => {
                "local_authority_restored_after_session_end"
            }
            Self::LocalAuthorityRestoredAfterExpiry => "local_authority_restored_after_expiry",
        }
    }
}

/// Closed vocabulary for the participant role on the bound shared
/// terminal pane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParticipantRoleClass {
    /// Session owner / host.
    SessionOwner,
    /// Named participant (invited or accepted).
    Participant,
    /// Approver who can admit grants but does not drive.
    Approver,
    /// Admin acting under admin-signed admission.
    Admin,
}

impl ParticipantRoleClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionOwner => "session_owner",
            Self::Participant => "participant",
            Self::Approver => "approver",
            Self::Admin => "admin",
        }
    }
}

/// Closed vocabulary for the cause of a control revocation. Mirrors the
/// upstream `control_grant_revocation_cause_class` so the runtime row
/// reads the same answer as the collaboration audit stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlRevocationCauseClass {
    /// Session owner revoked the grant.
    OwnerRevoked,
    /// Approver revoked the grant.
    ApproverRevoked,
    /// Policy system revoked the grant.
    PolicyRevoked,
    /// Admin-signed revocation.
    AdminSignedRevocation,
    /// Session ended; auto-revocation.
    SessionEndedAutoRevocation,
    /// Approval ticket expired.
    ApprovalTicketExpired,
    /// Workspace trust narrowed below the grant's required posture.
    WorkspaceTrustNarrowed,
    /// Relay outage; non-replayable.
    RelayOutageNonReplayable,
    /// Grantee released voluntarily.
    GranteeReleasedVoluntary,
    /// Session transport dropped; non-replayable.
    SessionTransportDroppedNonReplayable,
}

impl ControlRevocationCauseClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerRevoked => "owner_revoked",
            Self::ApproverRevoked => "approver_revoked",
            Self::PolicyRevoked => "policy_revoked",
            Self::AdminSignedRevocation => "admin_signed_revocation",
            Self::SessionEndedAutoRevocation => "session_ended_auto_revocation",
            Self::ApprovalTicketExpired => "approval_ticket_expired",
            Self::WorkspaceTrustNarrowed => "workspace_trust_narrowed",
            Self::RelayOutageNonReplayable => "relay_outage_non_replayable",
            Self::GranteeReleasedVoluntary => "grantee_released_voluntary",
            Self::SessionTransportDroppedNonReplayable => {
                "session_transport_dropped_non_replayable"
            }
        }
    }
}

/// Redaction-safe origin metadata for one shared terminal pane. Names
/// the session, pane, host identity, and execution context the pane is
/// bound to without leaking raw cwd, raw shell path, or raw env.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedTerminalBinding {
    /// Opaque ref to the collaboration session this terminal is hosted
    /// inside (upstream `collaboration_session_record`).
    pub session_ref: String,
    /// Opaque ref to the shared-terminal-control-metadata shared-object
    /// row this pane is anchored to (upstream
    /// `shared_terminal_control_metadata`).
    pub shared_object_ref: String,
    /// Opaque ref to the local terminal pane id.
    pub terminal_pane_ref: String,
    /// Opaque ref to the execution-context the pane is launched under.
    pub execution_context_ref: String,
    /// Opaque ref to the host identity the pane runs against.
    pub host_identity_ref: String,
}

/// References to the upstream schemas this alpha page composes with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedTerminalAlphaContractRefs {
    /// Reference to the control-grant boundary schema.
    pub control_grant_schema_ref: String,
    /// Reference to the shared-object boundary schema.
    pub shared_object_schema_ref: String,
    /// Reference to the session-state boundary schema.
    pub session_state_schema_ref: String,
    /// Reference to the follow-and-presenter-state boundary schema.
    pub follow_and_presenter_state_schema_ref: String,
    /// Reference to the session-policy-manifest boundary schema.
    pub session_policy_manifest_schema_ref: String,
}

impl SharedTerminalAlphaContractRefs {
    fn all_refs(&self) -> [&str; 5] {
        [
            &self.control_grant_schema_ref,
            &self.shared_object_schema_ref,
            &self.session_state_schema_ref,
            &self.follow_and_presenter_state_schema_ref,
            &self.session_policy_manifest_schema_ref,
        ]
    }
}

/// One typed per-participant control-state row on a shared terminal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedTerminalControlState {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque state id.
    pub state_id: String,
    /// Reviewable label safe for support export.
    pub display_label: String,
    /// Binding to session, terminal pane, execution context, host.
    pub binding: SharedTerminalBinding,
    /// Opaque ref to the participant actor.
    pub participant_actor_ref: String,
    /// Participant role.
    pub participant_role: ParticipantRoleClass,
    /// Current control state.
    pub control_state: SharedTerminalControlStateClass,
    /// Opaque ref to the upstream `control_grant_record`. Required for
    /// active and revoked states.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub control_grant_ref: Option<String>,
    /// Opaque ref to the upstream `control_grant_revocation_record`.
    /// Required when state is `control_revoked`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revocation_ref: Option<String>,
    /// Typed cause of revocation. Required when state is
    /// `control_revoked`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revocation_cause: Option<ControlRevocationCauseClass>,
    /// Opaque ref to the pending request row. Required when state is
    /// `request_control_pending`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_request_ref: Option<String>,
    /// Opaque refs to audit events minted for this row.
    #[serde(default)]
    pub audit_event_refs: Vec<String>,
    /// Optional continuity observation refs for this row.
    #[serde(default)]
    pub continuity_observation_refs: Vec<String>,
    /// Reviewable rationale.
    pub rationale_summary: String,
    /// Guardrail: row does not carry raw terminal bytes.
    pub raw_terminal_bytes_present: bool,
    /// Guardrail: row does not carry raw input payload.
    pub raw_input_payload_present: bool,
    /// Guardrail: row did not silently widen mutation authority.
    pub silent_authority_widening_taken: bool,
    /// Guardrail: local terminal continuity preserved for the
    /// session-owner local lane regardless of this row's state.
    pub local_terminal_continuity_preserved: bool,
    /// Timestamp at which the state was observed.
    pub observed_at: String,
}

/// One presenter-handoff event on a shared terminal pane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresenterHandoffEvent {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque handoff id.
    pub handoff_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Opaque ref to the bound session.
    pub session_ref: String,
    /// Opaque ref to the shared-terminal-control-metadata shared object.
    pub shared_object_ref: String,
    /// Opaque ref to the actor initiating the handoff.
    pub initiating_actor_ref: String,
    /// Opaque ref to the destination actor. Required when outcome is
    /// `presenter_role_accepted`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_actor_ref: Option<String>,
    /// Typed outcome.
    pub outcome: PresenterHandoffOutcomeClass,
    /// Reviewable decline-reason label. Required when outcome is
    /// `presenter_role_declined`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decline_reason_label: Option<String>,
    /// Reviewable revocation-cause label. Required when outcome is
    /// `presenter_role_revoked`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revocation_cause_label: Option<String>,
    /// Opaque ref to the presenter_state_record this handoff binds to.
    pub presenter_state_ref: String,
    /// Opaque audit-event refs minted for this handoff.
    #[serde(default)]
    pub audit_event_refs: Vec<String>,
    /// Reviewable summary safe for support export.
    pub support_export_summary: String,
    /// Guardrail: handoff did not silently widen mutation authority.
    pub silent_authority_widening_taken: bool,
    /// Timestamp at which the handoff was minted.
    pub minted_at: String,
    /// Timestamp at which the handoff resolved (accepted / declined /
    /// expired). Optional for pending handoffs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
}

/// One audit-event row on the shared-terminal control stream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedTerminalAuditEvent {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque audit-event id.
    pub audit_event_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Opaque ref to the bound session.
    pub session_ref: String,
    /// Opaque ref to the shared-terminal-control-metadata shared object.
    pub shared_object_ref: String,
    /// Typed audit-event class.
    pub event_class: SharedTerminalAuditEventClass,
    /// Optional ref to the bound control-state row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub control_state_ref: Option<String>,
    /// Optional ref to the bound presenter-handoff row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presenter_handoff_ref: Option<String>,
    /// Optional opaque ref to the upstream control-grant audit event id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upstream_audit_event_ref: Option<String>,
    /// Reviewable denial reason label. Required when event_class is
    /// `audit_denial_emitted` or `control_request_denied`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denial_reason_label: Option<String>,
    /// Timestamp at which the audit event was minted.
    pub minted_at: String,
}

/// One local-terminal continuity observation. Names how local terminal
/// authority is preserved when shared control ends or degrades.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalTerminalContinuityObservation {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque observation id.
    pub observation_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Opaque ref to the bound shared-terminal control-state row.
    pub bound_state_ref: String,
    /// Typed continuity class.
    pub continuity_class: LocalContinuityClass,
    /// Reviewable rationale.
    pub rationale_summary: String,
    /// Guardrail: observation did not silently widen mutation authority.
    pub silent_authority_widening_taken: bool,
    /// Guardrail: local terminal continuity preserved for the local
    /// pane.
    pub local_terminal_continuity_preserved: bool,
    /// Guardrail: no in-flight input was replayed against the bound
    /// shared-terminal pane after the degradation.
    pub in_flight_input_replayed: bool,
    /// Timestamp at which the observation was made.
    pub observed_at: String,
}

/// Optional fixture metadata used by protected cases.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedTerminalAlphaFixtureMetadata {
    /// Short fixture name.
    pub name: String,
    /// Reviewer-safe scenario summary.
    pub scenario: String,
}

/// One alpha page: control states + presenter handoff events + audit
/// events + continuity observations under one session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedTerminalAlphaPage {
    /// Optional fixture metadata.
    #[serde(
        default,
        rename = "__fixture__",
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture_metadata: Option<SharedTerminalAlphaFixtureMetadata>,
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for the page.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque page id.
    pub page_id: String,
    /// Upstream contracts this page composes with by reference.
    pub contract_refs: SharedTerminalAlphaContractRefs,
    /// Per-participant control-state rows.
    pub control_states: Vec<SharedTerminalControlState>,
    /// Presenter-handoff events.
    #[serde(default)]
    pub presenter_handoffs: Vec<PresenterHandoffEvent>,
    /// Audit-event rows.
    #[serde(default)]
    pub audit_events: Vec<SharedTerminalAuditEvent>,
    /// Local-terminal continuity observations.
    #[serde(default)]
    pub continuity_observations: Vec<LocalTerminalContinuityObservation>,
    /// Reviewable summary safe for support export.
    pub support_export_summary: String,
}

impl SharedTerminalAlphaPage {
    /// Validate the page against alpha invariants. Returns a structured
    /// report; the page is valid when `report.passed` is true.
    pub fn validate(&self) -> SharedTerminalAlphaValidationReport {
        let mut validator = Validator::new(self);
        validator.run();
        validator.finish()
    }

    /// Build a redaction-safe support-export projection.
    pub fn support_export_projection(&self) -> SharedTerminalAlphaSupportExport {
        let state_summaries = self
            .control_states
            .iter()
            .map(|state| SharedTerminalControlStateSummary {
                state_id: state.state_id.clone(),
                display_label: state.display_label.clone(),
                session_ref: state.binding.session_ref.clone(),
                shared_object_ref: state.binding.shared_object_ref.clone(),
                terminal_pane_ref: state.binding.terminal_pane_ref.clone(),
                participant_actor_ref: state.participant_actor_ref.clone(),
                participant_role: state.participant_role,
                control_state: state.control_state,
                revocation_cause: state.revocation_cause,
                rationale_summary: state.rationale_summary.clone(),
            })
            .collect();
        let presenter_summaries = self
            .presenter_handoffs
            .iter()
            .map(|handoff| PresenterHandoffSummary {
                handoff_id: handoff.handoff_id.clone(),
                display_label: handoff.display_label.clone(),
                session_ref: handoff.session_ref.clone(),
                shared_object_ref: handoff.shared_object_ref.clone(),
                outcome: handoff.outcome,
                decline_reason_label: handoff.decline_reason_label.clone(),
                revocation_cause_label: handoff.revocation_cause_label.clone(),
                support_export_summary: handoff.support_export_summary.clone(),
            })
            .collect();
        let audit_summaries = self
            .audit_events
            .iter()
            .map(|event| SharedTerminalAuditEventSummary {
                audit_event_id: event.audit_event_id.clone(),
                display_label: event.display_label.clone(),
                session_ref: event.session_ref.clone(),
                event_class: event.event_class,
                denial_reason_label: event.denial_reason_label.clone(),
            })
            .collect();
        let continuity_summaries = self
            .continuity_observations
            .iter()
            .map(|observation| LocalTerminalContinuityObservationSummary {
                observation_id: observation.observation_id.clone(),
                display_label: observation.display_label.clone(),
                bound_state_ref: observation.bound_state_ref.clone(),
                continuity_class: observation.continuity_class,
                rationale_summary: observation.rationale_summary.clone(),
            })
            .collect();
        SharedTerminalAlphaSupportExport {
            record_kind: SHARED_TERMINAL_ALPHA_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: SHARED_TERMINAL_ALPHA_SCHEMA_VERSION,
            page_id: self.page_id.clone(),
            state_summaries,
            presenter_summaries,
            audit_summaries,
            continuity_summaries,
        }
    }
}

/// Validation report emitted by the alpha validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedTerminalAlphaValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version validated.
    pub schema_version: u32,
    /// Page id under validation.
    pub page_id: String,
    /// True when no error-severity checks failed.
    pub passed: bool,
    /// Coverage observed while validating the page.
    pub coverage: SharedTerminalAlphaCoverage,
    /// Findings emitted by failed checks.
    pub findings: Vec<SharedTerminalAlphaFinding>,
}

/// Coverage observed during alpha validation.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SharedTerminalAlphaCoverage {
    /// Control states covered by per-participant rows.
    pub control_states: BTreeSet<SharedTerminalControlStateClass>,
    /// Participant roles covered.
    pub participant_roles: BTreeSet<ParticipantRoleClass>,
    /// Presenter-handoff outcomes covered.
    pub presenter_handoff_outcomes: BTreeSet<PresenterHandoffOutcomeClass>,
    /// Audit-event classes covered.
    pub audit_event_classes: BTreeSet<SharedTerminalAuditEventClass>,
    /// Continuity-observation classes covered.
    pub continuity_classes: BTreeSet<LocalContinuityClass>,
    /// Revocation causes covered by revoked state rows.
    pub revocation_causes: BTreeSet<ControlRevocationCauseClass>,
}

/// One validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedTerminalAlphaFinding {
    /// Severity.
    pub severity: SharedTerminalAlphaFindingSeverity,
    /// Stable check id.
    pub check_id: String,
    /// Redaction-safe message.
    pub message: String,
}

/// Validation finding severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedTerminalAlphaFindingSeverity {
    /// Error that blocks the page.
    Error,
    /// Warning that keeps the page reviewable but visibly degraded.
    Warning,
}

/// Redaction-safe support-export projection of one alpha page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedTerminalAlphaSupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Page id.
    pub page_id: String,
    /// Per-participant control-state summaries.
    pub state_summaries: Vec<SharedTerminalControlStateSummary>,
    /// Presenter-handoff summaries.
    pub presenter_summaries: Vec<PresenterHandoffSummary>,
    /// Audit-event summaries.
    pub audit_summaries: Vec<SharedTerminalAuditEventSummary>,
    /// Continuity-observation summaries.
    pub continuity_summaries: Vec<LocalTerminalContinuityObservationSummary>,
}

/// Redaction-safe summary of one per-participant control-state row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedTerminalControlStateSummary {
    /// State id.
    pub state_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Session ref.
    pub session_ref: String,
    /// Shared-object ref.
    pub shared_object_ref: String,
    /// Terminal pane ref.
    pub terminal_pane_ref: String,
    /// Participant actor ref.
    pub participant_actor_ref: String,
    /// Participant role.
    pub participant_role: ParticipantRoleClass,
    /// Control state.
    pub control_state: SharedTerminalControlStateClass,
    /// Optional revocation cause.
    pub revocation_cause: Option<ControlRevocationCauseClass>,
    /// Reviewable rationale.
    pub rationale_summary: String,
}

/// Redaction-safe summary of one presenter-handoff event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresenterHandoffSummary {
    /// Handoff id.
    pub handoff_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Session ref.
    pub session_ref: String,
    /// Shared-object ref.
    pub shared_object_ref: String,
    /// Outcome class.
    pub outcome: PresenterHandoffOutcomeClass,
    /// Optional decline reason label.
    pub decline_reason_label: Option<String>,
    /// Optional revocation cause label.
    pub revocation_cause_label: Option<String>,
    /// Reviewable support-export summary.
    pub support_export_summary: String,
}

/// Redaction-safe summary of one audit-event row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedTerminalAuditEventSummary {
    /// Audit event id.
    pub audit_event_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Session ref.
    pub session_ref: String,
    /// Event class.
    pub event_class: SharedTerminalAuditEventClass,
    /// Optional denial reason label.
    pub denial_reason_label: Option<String>,
}

/// Redaction-safe summary of one continuity observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalTerminalContinuityObservationSummary {
    /// Observation id.
    pub observation_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Bound state ref.
    pub bound_state_ref: String,
    /// Continuity class.
    pub continuity_class: LocalContinuityClass,
    /// Reviewable rationale.
    pub rationale_summary: String,
}

struct Validator<'a> {
    page: &'a SharedTerminalAlphaPage,
    state_ids: BTreeSet<&'a str>,
    handoff_ids: BTreeSet<&'a str>,
    audit_event_ids: BTreeSet<&'a str>,
    observation_ids: BTreeSet<&'a str>,
    coverage: SharedTerminalAlphaCoverage,
    findings: Vec<SharedTerminalAlphaFinding>,
}

impl<'a> Validator<'a> {
    fn new(page: &'a SharedTerminalAlphaPage) -> Self {
        Self {
            page,
            state_ids: BTreeSet::new(),
            handoff_ids: BTreeSet::new(),
            audit_event_ids: BTreeSet::new(),
            observation_ids: BTreeSet::new(),
            coverage: SharedTerminalAlphaCoverage::default(),
            findings: Vec::new(),
        }
    }

    fn run(&mut self) {
        self.validate_page_header();
        self.validate_control_states();
        self.validate_presenter_handoffs();
        self.validate_audit_events();
        self.validate_continuity_observations();
        self.validate_required_coverage();
    }

    fn finish(self) -> SharedTerminalAlphaValidationReport {
        let passed = self
            .findings
            .iter()
            .all(|finding| finding.severity != SharedTerminalAlphaFindingSeverity::Error);
        SharedTerminalAlphaValidationReport {
            record_kind: SHARED_TERMINAL_ALPHA_VALIDATION_REPORT_RECORD_KIND.to_string(),
            schema_version: SHARED_TERMINAL_ALPHA_SCHEMA_VERSION,
            page_id: self.page.page_id.clone(),
            passed,
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn validate_page_header(&mut self) {
        let page = self.page;
        self.expect(
            page.record_kind == SHARED_TERMINAL_ALPHA_PAGE_RECORD_KIND,
            "shared_terminal_alpha.page_record_kind",
            "page.record_kind must be shared_terminal_control_alpha_page_record",
        );
        self.expect(
            page.schema_version == SHARED_TERMINAL_ALPHA_SCHEMA_VERSION,
            "shared_terminal_alpha.page_schema_version",
            "page.schema_version must match the crate constant",
        );
        self.expect(
            page.shared_contract_ref == SHARED_TERMINAL_ALPHA_SHARED_CONTRACT_REF,
            "shared_terminal_alpha.page_shared_contract_ref",
            "page.shared_contract_ref must match the shared contract id",
        );
        self.expect(
            !page.page_id.trim().is_empty(),
            "shared_terminal_alpha.page_id_missing",
            "page.page_id must be non-empty",
        );
        self.expect(
            !page.support_export_summary.trim().is_empty(),
            "shared_terminal_alpha.page_support_summary_missing",
            "page.support_export_summary must be non-empty",
        );
        for contract_ref in page.contract_refs.all_refs() {
            self.expect(
                !contract_ref.trim().is_empty(),
                "shared_terminal_alpha.contract_ref_missing",
                "every consumed upstream contract ref must be non-empty",
            );
        }
        self.expect(
            !page.control_states.is_empty(),
            "shared_terminal_alpha.control_states_missing",
            "page must contain at least one control-state row",
        );
    }

    fn validate_control_states(&mut self) {
        for state in &self.page.control_states {
            self.expect(
                state.record_kind == SHARED_TERMINAL_ALPHA_CONTROL_STATE_RECORD_KIND,
                "shared_terminal_alpha.control_state_record_kind",
                "control_state.record_kind is wrong",
            );
            self.expect(
                state.schema_version == SHARED_TERMINAL_ALPHA_SCHEMA_VERSION,
                "shared_terminal_alpha.control_state_schema_version",
                "control_state.schema_version is wrong",
            );
            self.expect(
                state.shared_contract_ref == SHARED_TERMINAL_ALPHA_SHARED_CONTRACT_REF,
                "shared_terminal_alpha.control_state_shared_contract_ref",
                "control_state.shared_contract_ref must match the shared contract id",
            );
            let unique = self.state_ids.insert(&state.state_id);
            self.expect(
                unique,
                "shared_terminal_alpha.control_state_duplicate",
                "control_state.state_id values must be unique within a page",
            );
            self.expect(
                !state.display_label.trim().is_empty(),
                "shared_terminal_alpha.control_state_display_label_missing",
                "control_state.display_label must be non-empty",
            );
            self.expect(
                !state.binding.session_ref.trim().is_empty()
                    && !state.binding.shared_object_ref.trim().is_empty()
                    && !state.binding.terminal_pane_ref.trim().is_empty()
                    && !state.binding.execution_context_ref.trim().is_empty()
                    && !state.binding.host_identity_ref.trim().is_empty(),
                "shared_terminal_alpha.control_state_binding_incomplete",
                "control_state.binding must name session, shared-object, terminal-pane, \
                 execution-context, and host-identity refs",
            );
            self.expect(
                !state.participant_actor_ref.trim().is_empty(),
                "shared_terminal_alpha.control_state_participant_actor_missing",
                "control_state.participant_actor_ref must be non-empty",
            );
            self.expect(
                !state.rationale_summary.trim().is_empty(),
                "shared_terminal_alpha.control_state_rationale_missing",
                "control_state.rationale_summary must be non-empty",
            );
            self.expect(
                !state.observed_at.trim().is_empty(),
                "shared_terminal_alpha.control_state_observed_at_missing",
                "control_state.observed_at must be non-empty",
            );
            self.expect(
                !state.raw_terminal_bytes_present,
                "shared_terminal_alpha.control_state_raw_terminal_bytes_present",
                "control_state.raw_terminal_bytes_present must be false",
            );
            self.expect(
                !state.raw_input_payload_present,
                "shared_terminal_alpha.control_state_raw_input_payload_present",
                "control_state.raw_input_payload_present must be false",
            );
            self.expect(
                !state.silent_authority_widening_taken,
                "shared_terminal_alpha.control_state_silent_authority_widening",
                "control_state.silent_authority_widening_taken must be false",
            );
            self.expect(
                state.local_terminal_continuity_preserved,
                "shared_terminal_alpha.control_state_continuity_not_preserved",
                "control_state.local_terminal_continuity_preserved must be true; local pane \
                 continuity is a closed invariant on this alpha lane",
            );

            self.validate_control_state_refs(state);

            self.coverage.control_states.insert(state.control_state);
            self.coverage
                .participant_roles
                .insert(state.participant_role);
            if let Some(cause) = state.revocation_cause {
                self.coverage.revocation_causes.insert(cause);
            }
        }
    }

    fn validate_control_state_refs(&mut self, state: &SharedTerminalControlState) {
        let non_empty =
            |opt: &Option<String>| opt.as_deref().is_some_and(|value| !value.trim().is_empty());

        if state.control_state.requires_control_grant() {
            self.expect(
                non_empty(&state.control_grant_ref),
                "shared_terminal_alpha.control_state_grant_ref_missing",
                "active and revoked states must cite a control_grant_ref; control is never \
                 inferable from presence or presenter state",
            );
        } else {
            self.expect(
                state.control_grant_ref.is_none(),
                "shared_terminal_alpha.control_state_grant_ref_unexpected",
                "view-only and request-pending states must not cite a control_grant_ref",
            );
        }

        if state.control_state.requires_revocation() {
            self.expect(
                non_empty(&state.revocation_ref),
                "shared_terminal_alpha.control_state_revocation_ref_missing",
                "control_revoked states must cite a revocation_ref",
            );
            self.expect(
                state.revocation_cause.is_some(),
                "shared_terminal_alpha.control_state_revocation_cause_missing",
                "control_revoked states must cite a revocation_cause",
            );
        } else {
            self.expect(
                state.revocation_ref.is_none(),
                "shared_terminal_alpha.control_state_revocation_ref_unexpected",
                "non-revoked states must not cite a revocation_ref",
            );
            self.expect(
                state.revocation_cause.is_none(),
                "shared_terminal_alpha.control_state_revocation_cause_unexpected",
                "non-revoked states must not cite a revocation_cause",
            );
        }

        if state.control_state.requires_pending_request() {
            self.expect(
                non_empty(&state.pending_request_ref),
                "shared_terminal_alpha.control_state_pending_request_missing",
                "request_control_pending states must cite a pending_request_ref",
            );
        } else {
            self.expect(
                state.pending_request_ref.is_none(),
                "shared_terminal_alpha.control_state_pending_request_unexpected",
                "non-pending states must not cite a pending_request_ref",
            );
        }
    }

    fn validate_presenter_handoffs(&mut self) {
        for handoff in &self.page.presenter_handoffs {
            self.expect(
                handoff.record_kind == SHARED_TERMINAL_ALPHA_PRESENTER_HANDOFF_RECORD_KIND,
                "shared_terminal_alpha.presenter_handoff_record_kind",
                "presenter_handoff.record_kind is wrong",
            );
            self.expect(
                handoff.schema_version == SHARED_TERMINAL_ALPHA_SCHEMA_VERSION,
                "shared_terminal_alpha.presenter_handoff_schema_version",
                "presenter_handoff.schema_version is wrong",
            );
            self.expect(
                handoff.shared_contract_ref == SHARED_TERMINAL_ALPHA_SHARED_CONTRACT_REF,
                "shared_terminal_alpha.presenter_handoff_shared_contract_ref",
                "presenter_handoff.shared_contract_ref must match the shared contract id",
            );
            let unique = self.handoff_ids.insert(&handoff.handoff_id);
            self.expect(
                unique,
                "shared_terminal_alpha.presenter_handoff_duplicate",
                "presenter_handoff.handoff_id values must be unique within a page",
            );
            self.expect(
                !handoff.display_label.trim().is_empty(),
                "shared_terminal_alpha.presenter_handoff_display_label_missing",
                "presenter_handoff.display_label must be non-empty",
            );
            self.expect(
                !handoff.session_ref.trim().is_empty()
                    && !handoff.shared_object_ref.trim().is_empty()
                    && !handoff.initiating_actor_ref.trim().is_empty()
                    && !handoff.presenter_state_ref.trim().is_empty(),
                "shared_terminal_alpha.presenter_handoff_binding_incomplete",
                "presenter_handoff must name session, shared-object, initiating-actor, and \
                 presenter-state refs",
            );
            self.expect(
                !handoff.support_export_summary.trim().is_empty(),
                "shared_terminal_alpha.presenter_handoff_support_summary_missing",
                "presenter_handoff.support_export_summary must be non-empty",
            );
            self.expect(
                !handoff.minted_at.trim().is_empty(),
                "shared_terminal_alpha.presenter_handoff_minted_at_missing",
                "presenter_handoff.minted_at must be non-empty",
            );
            self.expect(
                !handoff.silent_authority_widening_taken,
                "shared_terminal_alpha.presenter_handoff_silent_authority_widening",
                "presenter_handoff.silent_authority_widening_taken must be false",
            );

            let non_empty =
                |opt: &Option<String>| opt.as_deref().is_some_and(|value| !value.trim().is_empty());

            if handoff.outcome.requires_destination_actor() {
                self.expect(
                    non_empty(&handoff.destination_actor_ref),
                    "shared_terminal_alpha.presenter_handoff_destination_actor_missing",
                    "presenter_role_accepted handoffs must cite a destination_actor_ref",
                );
            }
            if handoff.outcome.requires_decline_reason() {
                self.expect(
                    non_empty(&handoff.decline_reason_label),
                    "shared_terminal_alpha.presenter_handoff_decline_reason_missing",
                    "presenter_role_declined handoffs must cite a decline_reason_label",
                );
            }
            if handoff.outcome.requires_revocation_cause() {
                self.expect(
                    non_empty(&handoff.revocation_cause_label),
                    "shared_terminal_alpha.presenter_handoff_revocation_cause_missing",
                    "presenter_role_revoked handoffs must cite a revocation_cause_label",
                );
            }
            let resolved_outcome = !matches!(
                handoff.outcome,
                PresenterHandoffOutcomeClass::PresenterRoleExpiredSessionEnd
            );
            if resolved_outcome {
                self.expect(
                    handoff
                        .resolved_at
                        .as_deref()
                        .is_some_and(|value| !value.trim().is_empty()),
                    "shared_terminal_alpha.presenter_handoff_resolved_at_missing",
                    "resolved presenter-handoff outcomes must cite a resolved_at timestamp",
                );
            }

            self.coverage
                .presenter_handoff_outcomes
                .insert(handoff.outcome);
        }
    }

    fn validate_audit_events(&mut self) {
        for event in &self.page.audit_events {
            self.expect(
                event.record_kind == SHARED_TERMINAL_ALPHA_AUDIT_EVENT_RECORD_KIND,
                "shared_terminal_alpha.audit_event_record_kind",
                "audit_event.record_kind is wrong",
            );
            self.expect(
                event.schema_version == SHARED_TERMINAL_ALPHA_SCHEMA_VERSION,
                "shared_terminal_alpha.audit_event_schema_version",
                "audit_event.schema_version is wrong",
            );
            self.expect(
                event.shared_contract_ref == SHARED_TERMINAL_ALPHA_SHARED_CONTRACT_REF,
                "shared_terminal_alpha.audit_event_shared_contract_ref",
                "audit_event.shared_contract_ref must match the shared contract id",
            );
            let unique = self.audit_event_ids.insert(&event.audit_event_id);
            self.expect(
                unique,
                "shared_terminal_alpha.audit_event_duplicate",
                "audit_event.audit_event_id values must be unique within a page",
            );
            self.expect(
                !event.display_label.trim().is_empty(),
                "shared_terminal_alpha.audit_event_display_label_missing",
                "audit_event.display_label must be non-empty",
            );
            self.expect(
                !event.session_ref.trim().is_empty() && !event.shared_object_ref.trim().is_empty(),
                "shared_terminal_alpha.audit_event_binding_incomplete",
                "audit_event must name session and shared-object refs",
            );
            self.expect(
                !event.minted_at.trim().is_empty(),
                "shared_terminal_alpha.audit_event_minted_at_missing",
                "audit_event.minted_at must be non-empty",
            );

            let non_empty =
                |opt: &Option<String>| opt.as_deref().is_some_and(|value| !value.trim().is_empty());

            if event.event_class.requires_denial_reason() {
                self.expect(
                    non_empty(&event.denial_reason_label),
                    "shared_terminal_alpha.audit_event_denial_reason_missing",
                    "denial audit events must cite a denial_reason_label",
                );
            }

            if event.event_class.requires_handoff_ref() {
                self.expect(
                    non_empty(&event.presenter_handoff_ref),
                    "shared_terminal_alpha.audit_event_handoff_ref_missing",
                    "presenter-handoff audit events must cite a presenter_handoff_ref",
                );
                if let Some(handoff_ref) = event.presenter_handoff_ref.as_deref() {
                    self.expect(
                        self.handoff_ids.contains(handoff_ref),
                        "shared_terminal_alpha.audit_event_handoff_ref_unknown",
                        "audit_event.presenter_handoff_ref must reference a handoff in the page",
                    );
                }
            }

            if event.event_class.requires_state_ref() {
                self.expect(
                    non_empty(&event.control_state_ref),
                    "shared_terminal_alpha.audit_event_state_ref_missing",
                    "control-state audit events must cite a control_state_ref",
                );
                if let Some(state_ref) = event.control_state_ref.as_deref() {
                    self.expect(
                        self.state_ids.contains(state_ref),
                        "shared_terminal_alpha.audit_event_state_ref_unknown",
                        "audit_event.control_state_ref must reference a control-state row \
                         in the page",
                    );
                }
            }

            self.coverage.audit_event_classes.insert(event.event_class);
        }
    }

    fn validate_continuity_observations(&mut self) {
        for observation in &self.page.continuity_observations {
            self.expect(
                observation.record_kind == SHARED_TERMINAL_ALPHA_CONTINUITY_OBSERVATION_RECORD_KIND,
                "shared_terminal_alpha.continuity_observation_record_kind",
                "continuity_observation.record_kind is wrong",
            );
            self.expect(
                observation.schema_version == SHARED_TERMINAL_ALPHA_SCHEMA_VERSION,
                "shared_terminal_alpha.continuity_observation_schema_version",
                "continuity_observation.schema_version is wrong",
            );
            self.expect(
                observation.shared_contract_ref == SHARED_TERMINAL_ALPHA_SHARED_CONTRACT_REF,
                "shared_terminal_alpha.continuity_observation_shared_contract_ref",
                "continuity_observation.shared_contract_ref must match the shared contract id",
            );
            let unique = self.observation_ids.insert(&observation.observation_id);
            self.expect(
                unique,
                "shared_terminal_alpha.continuity_observation_duplicate",
                "continuity_observation.observation_id values must be unique within a page",
            );
            self.expect(
                !observation.display_label.trim().is_empty(),
                "shared_terminal_alpha.continuity_observation_display_label_missing",
                "continuity_observation.display_label must be non-empty",
            );
            self.expect(
                !observation.rationale_summary.trim().is_empty(),
                "shared_terminal_alpha.continuity_observation_rationale_missing",
                "continuity_observation.rationale_summary must be non-empty",
            );
            self.expect(
                !observation.bound_state_ref.trim().is_empty(),
                "shared_terminal_alpha.continuity_observation_state_ref_missing",
                "continuity_observation.bound_state_ref must be non-empty",
            );
            self.expect(
                self.state_ids
                    .contains(observation.bound_state_ref.as_str()),
                "shared_terminal_alpha.continuity_observation_state_ref_unknown",
                "continuity_observation.bound_state_ref must reference a control-state row \
                 in the page",
            );
            self.expect(
                !observation.silent_authority_widening_taken,
                "shared_terminal_alpha.continuity_observation_silent_widen",
                "continuity_observation.silent_authority_widening_taken must be false",
            );
            self.expect(
                observation.local_terminal_continuity_preserved,
                "shared_terminal_alpha.continuity_observation_continuity_not_preserved",
                "continuity_observation.local_terminal_continuity_preserved must be true",
            );
            self.expect(
                !observation.in_flight_input_replayed,
                "shared_terminal_alpha.continuity_observation_in_flight_input_replayed",
                "continuity_observation.in_flight_input_replayed must be false; in-flight input \
                 is never replayed against a revoked or degraded grant",
            );
            self.expect(
                !observation.observed_at.trim().is_empty(),
                "shared_terminal_alpha.continuity_observation_observed_at_missing",
                "continuity_observation.observed_at must be non-empty",
            );

            self.coverage
                .continuity_classes
                .insert(observation.continuity_class);
        }
    }

    fn validate_required_coverage(&mut self) {
        for state in [
            SharedTerminalControlStateClass::ViewOnlyObserver,
            SharedTerminalControlStateClass::RequestControlPending,
            SharedTerminalControlStateClass::ActiveControlGrantee,
            SharedTerminalControlStateClass::ControlRevoked,
        ] {
            self.expect(
                self.coverage.control_states.contains(&state),
                "shared_terminal_alpha.coverage_control_state_missing",
                "page must cover view-only, request-pending, active-control, and revoked states",
            );
        }
        for outcome in [
            PresenterHandoffOutcomeClass::PresenterRoleAccepted,
            PresenterHandoffOutcomeClass::PresenterRoleAutoObserver,
        ] {
            self.expect(
                self.coverage.presenter_handoff_outcomes.contains(&outcome),
                "shared_terminal_alpha.coverage_presenter_outcome_missing",
                "page must cover accepted and auto-observer presenter-handoff outcomes",
            );
        }
        for class in [
            SharedTerminalAuditEventClass::ControlActiveStarted,
            SharedTerminalAuditEventClass::ControlRevoked,
            SharedTerminalAuditEventClass::PresenterHandoffResolved,
        ] {
            self.expect(
                self.coverage.audit_event_classes.contains(&class),
                "shared_terminal_alpha.coverage_audit_event_missing",
                "page must cover active-started, revoked, and presenter-handoff-resolved \
                 audit events",
            );
        }
    }

    fn expect(&mut self, passed: bool, check_id: &str, message: &str) {
        if !passed {
            self.findings.push(SharedTerminalAlphaFinding {
                severity: SharedTerminalAlphaFindingSeverity::Error,
                check_id: check_id.to_string(),
                message: message.to_string(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn binding() -> SharedTerminalBinding {
        SharedTerminalBinding {
            session_ref: "collab.session.alpha".to_string(),
            shared_object_ref: "collab.shared_object.shared_terminal_control.alpha".to_string(),
            terminal_pane_ref: "terminal.pane.alpha".to_string(),
            execution_context_ref: "execution.context.alpha".to_string(),
            host_identity_ref: "host.identity.workspace.primary".to_string(),
        }
    }

    fn state(
        id: &str,
        role: ParticipantRoleClass,
        control: SharedTerminalControlStateClass,
        revocation_cause: Option<ControlRevocationCauseClass>,
    ) -> SharedTerminalControlState {
        SharedTerminalControlState {
            record_kind: SHARED_TERMINAL_ALPHA_CONTROL_STATE_RECORD_KIND.to_string(),
            schema_version: SHARED_TERMINAL_ALPHA_SCHEMA_VERSION,
            shared_contract_ref: SHARED_TERMINAL_ALPHA_SHARED_CONTRACT_REF.to_string(),
            state_id: id.to_string(),
            display_label: format!("State {id}"),
            binding: binding(),
            participant_actor_ref: format!("actor.participant.{id}"),
            participant_role: role,
            control_state: control,
            control_grant_ref: if control.requires_control_grant() {
                Some(format!("collab.control_grant.{id}"))
            } else {
                None
            },
            revocation_ref: if control.requires_revocation() {
                Some(format!("collab.control_grant.revocation.{id}"))
            } else {
                None
            },
            revocation_cause,
            pending_request_ref: if control.requires_pending_request() {
                Some(format!("shared_terminal.request.{id}"))
            } else {
                None
            },
            audit_event_refs: vec![format!("shared_terminal.audit.{id}")],
            continuity_observation_refs: Vec::new(),
            rationale_summary: format!("Rationale for state {id}"),
            raw_terminal_bytes_present: false,
            raw_input_payload_present: false,
            silent_authority_widening_taken: false,
            local_terminal_continuity_preserved: true,
            observed_at: "2026-05-13T18:00:00Z".to_string(),
        }
    }

    fn handoff(id: &str, outcome: PresenterHandoffOutcomeClass) -> PresenterHandoffEvent {
        PresenterHandoffEvent {
            record_kind: SHARED_TERMINAL_ALPHA_PRESENTER_HANDOFF_RECORD_KIND.to_string(),
            schema_version: SHARED_TERMINAL_ALPHA_SCHEMA_VERSION,
            shared_contract_ref: SHARED_TERMINAL_ALPHA_SHARED_CONTRACT_REF.to_string(),
            handoff_id: id.to_string(),
            display_label: format!("Handoff {id}"),
            session_ref: "collab.session.alpha".to_string(),
            shared_object_ref: "collab.shared_object.shared_terminal_control.alpha".to_string(),
            initiating_actor_ref: "actor.owner.alpha".to_string(),
            destination_actor_ref: outcome
                .requires_destination_actor()
                .then(|| format!("actor.destination.{id}")),
            outcome,
            decline_reason_label: outcome
                .requires_decline_reason()
                .then(|| "Destination declined the role.".to_string()),
            revocation_cause_label: outcome
                .requires_revocation_cause()
                .then(|| "Owner revoked the role.".to_string()),
            presenter_state_ref: "collab.presenter_state.alpha".to_string(),
            audit_event_refs: vec![format!("shared_terminal.audit.handoff.{id}")],
            support_export_summary: format!("Support summary for handoff {id}"),
            silent_authority_widening_taken: false,
            minted_at: "2026-05-13T18:00:00Z".to_string(),
            resolved_at: if matches!(
                outcome,
                PresenterHandoffOutcomeClass::PresenterRoleExpiredSessionEnd
            ) {
                None
            } else {
                Some("2026-05-13T18:01:00Z".to_string())
            },
        }
    }

    fn audit(
        id: &str,
        class: SharedTerminalAuditEventClass,
        state_ref: Option<&str>,
        handoff_ref: Option<&str>,
        denial: Option<&str>,
    ) -> SharedTerminalAuditEvent {
        SharedTerminalAuditEvent {
            record_kind: SHARED_TERMINAL_ALPHA_AUDIT_EVENT_RECORD_KIND.to_string(),
            schema_version: SHARED_TERMINAL_ALPHA_SCHEMA_VERSION,
            shared_contract_ref: SHARED_TERMINAL_ALPHA_SHARED_CONTRACT_REF.to_string(),
            audit_event_id: id.to_string(),
            display_label: format!("Audit {id}"),
            session_ref: "collab.session.alpha".to_string(),
            shared_object_ref: "collab.shared_object.shared_terminal_control.alpha".to_string(),
            event_class: class,
            control_state_ref: state_ref.map(str::to_string),
            presenter_handoff_ref: handoff_ref.map(str::to_string),
            upstream_audit_event_ref: None,
            denial_reason_label: denial.map(str::to_string),
            minted_at: "2026-05-13T18:00:00Z".to_string(),
        }
    }

    fn baseline_page() -> SharedTerminalAlphaPage {
        let observer = state(
            "viewer",
            ParticipantRoleClass::Participant,
            SharedTerminalControlStateClass::ViewOnlyObserver,
            None,
        );
        let pending = state(
            "requester",
            ParticipantRoleClass::Participant,
            SharedTerminalControlStateClass::RequestControlPending,
            None,
        );
        let active = state(
            "driver",
            ParticipantRoleClass::Participant,
            SharedTerminalControlStateClass::ActiveControlGrantee,
            None,
        );
        let revoked = state(
            "revoked",
            ParticipantRoleClass::Participant,
            SharedTerminalControlStateClass::ControlRevoked,
            Some(ControlRevocationCauseClass::OwnerRevoked),
        );
        let handoff_accepted = handoff(
            "handoff.accepted",
            PresenterHandoffOutcomeClass::PresenterRoleAccepted,
        );
        let handoff_auto = handoff(
            "handoff.auto",
            PresenterHandoffOutcomeClass::PresenterRoleAutoObserver,
        );
        let audit_started = audit(
            "audit.active.started",
            SharedTerminalAuditEventClass::ControlActiveStarted,
            Some("driver"),
            None,
            None,
        );
        let audit_revoked = audit(
            "audit.revoked",
            SharedTerminalAuditEventClass::ControlRevoked,
            Some("revoked"),
            None,
            None,
        );
        let audit_handoff_resolved = audit(
            "audit.handoff.resolved",
            SharedTerminalAuditEventClass::PresenterHandoffResolved,
            None,
            Some("handoff.accepted"),
            None,
        );
        let continuity = LocalTerminalContinuityObservation {
            record_kind: SHARED_TERMINAL_ALPHA_CONTINUITY_OBSERVATION_RECORD_KIND.to_string(),
            schema_version: SHARED_TERMINAL_ALPHA_SCHEMA_VERSION,
            shared_contract_ref: SHARED_TERMINAL_ALPHA_SHARED_CONTRACT_REF.to_string(),
            observation_id: "continuity.revoked".to_string(),
            display_label: "Owner input preserved.".to_string(),
            bound_state_ref: "revoked".to_string(),
            continuity_class: LocalContinuityClass::OwnerInputPreservedAfterGranteeRevoked,
            rationale_summary: "Owner local pane continued without interruption.".to_string(),
            silent_authority_widening_taken: false,
            local_terminal_continuity_preserved: true,
            in_flight_input_replayed: false,
            observed_at: "2026-05-13T18:00:30Z".to_string(),
        };

        SharedTerminalAlphaPage {
            fixture_metadata: None,
            record_kind: SHARED_TERMINAL_ALPHA_PAGE_RECORD_KIND.to_string(),
            schema_version: SHARED_TERMINAL_ALPHA_SCHEMA_VERSION,
            shared_contract_ref: SHARED_TERMINAL_ALPHA_SHARED_CONTRACT_REF.to_string(),
            page_id: "shared_terminal_alpha.page.unit_test".to_string(),
            contract_refs: SharedTerminalAlphaContractRefs {
                control_grant_schema_ref: "schemas/collaboration/control_grant.schema.json"
                    .to_string(),
                shared_object_schema_ref: "schemas/collaboration/shared_object.schema.json"
                    .to_string(),
                session_state_schema_ref: "schemas/collaboration/session_state.schema.json"
                    .to_string(),
                follow_and_presenter_state_schema_ref:
                    "schemas/collaboration/follow_and_presenter_state.schema.json".to_string(),
                session_policy_manifest_schema_ref:
                    "schemas/collaboration/session_policy_manifest.schema.json".to_string(),
            },
            control_states: vec![observer, pending, active, revoked],
            presenter_handoffs: vec![handoff_accepted, handoff_auto],
            audit_events: vec![audit_started, audit_revoked, audit_handoff_resolved],
            continuity_observations: vec![continuity],
            support_export_summary:
                "Shared-terminal alpha unit-test page covering all four control states, a \
                 presenter handoff, audit events, and a continuity observation."
                    .to_string(),
        }
    }

    #[test]
    fn baseline_page_validates() {
        let page = baseline_page();
        let report = page.validate();
        assert!(report.passed, "baseline must pass: {:#?}", report.findings);
        for state in [
            SharedTerminalControlStateClass::ViewOnlyObserver,
            SharedTerminalControlStateClass::RequestControlPending,
            SharedTerminalControlStateClass::ActiveControlGrantee,
            SharedTerminalControlStateClass::ControlRevoked,
        ] {
            assert!(report.coverage.control_states.contains(&state));
        }
    }

    #[test]
    fn active_state_requires_grant_ref() {
        let mut page = baseline_page();
        page.control_states
            .iter_mut()
            .find(|state| state.state_id == "driver")
            .expect("active state present")
            .control_grant_ref = None;
        let report = page.validate();
        assert!(!report.passed);
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.check_id
                == "shared_terminal_alpha.control_state_grant_ref_missing"));
    }

    #[test]
    fn view_only_state_must_not_cite_grant_ref() {
        let mut page = baseline_page();
        page.control_states
            .iter_mut()
            .find(|state| state.state_id == "viewer")
            .expect("viewer state present")
            .control_grant_ref = Some("collab.control_grant.unexpected".to_string());
        let report = page.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(|finding| finding.check_id
            == "shared_terminal_alpha.control_state_grant_ref_unexpected"));
    }

    #[test]
    fn revoked_state_requires_revocation_cause() {
        let mut page = baseline_page();
        page.control_states
            .iter_mut()
            .find(|state| state.state_id == "revoked")
            .expect("revoked state present")
            .revocation_cause = None;
        let report = page.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(|finding| finding.check_id
            == "shared_terminal_alpha.control_state_revocation_cause_missing"));
    }

    #[test]
    fn audit_event_state_ref_must_reference_known_row() {
        let mut page = baseline_page();
        page.audit_events
            .iter_mut()
            .find(|event| event.audit_event_id == "audit.active.started")
            .expect("audit started present")
            .control_state_ref = Some("unknown".to_string());
        let report = page.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(
            |finding| finding.check_id == "shared_terminal_alpha.audit_event_state_ref_unknown"
        ));
    }

    #[test]
    fn continuity_observation_must_bind_known_state() {
        let mut page = baseline_page();
        page.continuity_observations
            .iter_mut()
            .next()
            .expect("observation present")
            .bound_state_ref = "unknown".to_string();
        let report = page.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(|finding| finding.check_id
            == "shared_terminal_alpha.continuity_observation_state_ref_unknown"));
    }

    #[test]
    fn coverage_requires_all_four_control_states() {
        let mut page = baseline_page();
        page.control_states
            .retain(|state| state.control_state != SharedTerminalControlStateClass::ControlRevoked);
        page.continuity_observations
            .retain(|observation| observation.bound_state_ref != "revoked");
        page.audit_events
            .retain(|event| event.control_state_ref.as_deref() != Some("revoked"));
        let report = page.validate();
        assert!(!report.passed);
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.check_id
                == "shared_terminal_alpha.coverage_control_state_missing"));
    }

    #[test]
    fn continuity_observation_must_not_replay_in_flight_input() {
        let mut page = baseline_page();
        page.continuity_observations
            .iter_mut()
            .next()
            .expect("observation present")
            .in_flight_input_replayed = true;
        let report = page.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(|finding| finding.check_id
            == "shared_terminal_alpha.continuity_observation_in_flight_input_replayed"));
    }

    #[test]
    fn support_export_omits_raw_payload_fields() {
        let page = baseline_page();
        let projection = page.support_export_projection();
        let json = serde_json::to_string(&projection).expect("projection serializes");
        assert_eq!(
            projection.record_kind,
            "shared_terminal_control_alpha_support_export"
        );
        assert!(!json.contains("raw_terminal_bytes"));
        assert!(!json.contains("raw_input_payload"));
        assert!(!json.contains("in_flight_input_replayed"));
        assert!(!json.contains("upstream_audit_event_ref"));
        assert_eq!(projection.state_summaries.len(), page.control_states.len());
        assert_eq!(
            projection.presenter_summaries.len(),
            page.presenter_handoffs.len()
        );
        assert_eq!(projection.audit_summaries.len(), page.audit_events.len());
        assert_eq!(
            projection.continuity_summaries.len(),
            page.continuity_observations.len()
        );
    }
}
