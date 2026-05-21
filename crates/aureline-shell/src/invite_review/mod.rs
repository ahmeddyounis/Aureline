//! Collaboration invite, observer-first join, and temporary authority-escrow
//! review for the shared terminal/debug and presentation preview lanes.
//!
//! This module is the shell-side projection the claimed M3 shared-control lanes
//! read *before* anyone joins a session or takes privileged control. It answers
//! the questions a user must be able to settle before accepting an invite or
//! approving a control request:
//!
//! 1. **Who is inviting me, into what, and what does accepting actually grant?**
//!    An [`InviteSessionManifest`] names the session owner, the shared lane
//!    ([`SessionLaneClass`]: shared terminal, shared debug, presentation
//!    preview), the offered role, the client class the invite arrives on, the
//!    retention/recording posture, the invite expiry, and — critically — the
//!    [`InviteCapabilityClass`] that accepting confers. That capability is
//!    *observer-first by construction*: the strongest thing an invite can confer
//!    on accept is the right to *request* temporary control, never control
//!    itself.
//!
//! 2. **What temporary authority is being requested, with what scope, expiry,
//!    and downgrade path?** An [`AuthorityEscrowTicket`] describes one temporary
//!    grant over [terminal input][AuthorityScopeClass::TerminalInput],
//!    [debug step/control][AuthorityScopeClass::DebugStepControl], or a
//!    [presenter][AuthorityScopeClass::PresenterHandoff] /
//!    [follow][AuthorityScopeClass::FollowHandoff] handoff: its lifecycle
//!    [`GrantStateClass`] (pending → active → expired/revoked/frozen), a bounded
//!    expiry, a [`DowngradePathClass`], and the explicit context changes that
//!    force reapproval. A privileged active grant MUST require reapproval on
//!    reconnect, device handoff, and restart so it can never silently resume.
//!
//! 3. **What survives a reconnect, browser-to-desktop handoff, or restart?** An
//!    [`AuthorityContinuity`] block proves that an interrupting context change
//!    does not silently resume privileged control: a still-active grant after an
//!    interruption requires explicit reapproval, frozen/revoked/expired states
//!    stay visible, and the local shared-lane row remains usable even when the
//!    grant is denied or revoked.
//!
//! 4. **Is the event durable and export-safe?** A [`HistoryAttentionEvent`]
//!    feeds the invite/authority transition into a durable history/attention
//!    surface with a stable id and an export-safe reason code whose vocabulary
//!    matches the grant lifecycle.
//!
//! The four pieces are bundled by [`InviteAuthorityReviewSheet`], whose
//! cross-validator proves they agree: the requested authority scope is one the
//! invite's capability actually permits (observer-first), the scope matches the
//! invited lane, the durable reason code matches the grant state, and the
//! continuity block never silently resumes control.
//!
//! Raw URLs, raw email addresses, raw absolute paths, and raw secret material
//! MUST NOT appear; the records carry opaque refs and bounded reviewable
//! summaries only. The schema boundaries are
//! `schemas/collab/invite_session_manifest.schema.json` and
//! `schemas/collab/authority_escrow_ticket.schema.json`; the contract narrative
//! is `docs/collab/m3/invite_and_authority_escrow.md`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`InviteSessionManifest`].
pub const INVITE_SESSION_MANIFEST_RECORD_KIND: &str = "invite_session_manifest_record";

/// Stable record-kind tag carried by [`AuthorityEscrowTicket`].
pub const AUTHORITY_ESCROW_TICKET_RECORD_KIND: &str = "authority_escrow_ticket_record";

/// Stable record-kind tag carried by [`InviteAuthorityReviewSheet`].
pub const INVITE_AUTHORITY_REVIEW_SHEET_RECORD_KIND: &str = "invite_authority_review_sheet_record";

/// Schema version for the [`InviteSessionManifest`] payload shape.
pub const INVITE_SESSION_MANIFEST_SCHEMA_VERSION: u32 = 1;

/// Schema version for the [`AuthorityEscrowTicket`] payload shape.
pub const AUTHORITY_ESCROW_TICKET_SCHEMA_VERSION: u32 = 1;

/// Schema version for the [`InviteAuthorityReviewSheet`] payload shape.
pub const INVITE_AUTHORITY_REVIEW_SHEET_SCHEMA_VERSION: u32 = 1;

/// Frozen reference to the contract doc every record points at.
pub const INVITE_AND_AUTHORITY_CONTRACT_DOC_REF: &str =
    "docs/collab/m3/invite_and_authority_escrow.md";

/// Closed vocabulary for the claimed M3 shared-control lanes an invite can scope
/// into. Distinct lanes never blur together: a terminal invite is not a debug
/// invite, and neither is a presentation row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionLaneClass {
    SharedTerminal,
    SharedDebug,
    PresentationPreview,
}

impl SessionLaneClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SharedTerminal => "shared_terminal",
            Self::SharedDebug => "shared_debug",
            Self::PresentationPreview => "presentation_preview",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::SharedTerminal => "Shared terminal",
            Self::SharedDebug => "Shared debug",
            Self::PresentationPreview => "Presentation preview",
        }
    }
}

/// Closed capability vocabulary describing what *accepting* an invite confers.
///
/// Observer-first is structural: the strongest capability an invite can confer
/// on accept is the right to *request* temporary control — never control itself.
/// There is deliberately no `control_granted` variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InviteCapabilityClass {
    ObserverOnly,
    ObserverWithFollow,
    ObserverWithControlRequest,
}

impl InviteCapabilityClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ObserverOnly => "observer_only",
            Self::ObserverWithFollow => "observer_with_follow",
            Self::ObserverWithControlRequest => "observer_with_control_request",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::ObserverOnly => "Observer only",
            Self::ObserverWithFollow => "Observer with follow",
            Self::ObserverWithControlRequest => "Observer with control request",
        }
    }

    /// True when accepting lets the invitee *request* temporary control.
    pub const fn grants_control_request(self) -> bool {
        matches!(self, Self::ObserverWithControlRequest)
    }

    /// True when accepting lets the invitee follow the presenter.
    pub const fn grants_follow(self) -> bool {
        matches!(
            self,
            Self::ObserverWithFollow | Self::ObserverWithControlRequest
        )
    }
}

/// Closed role vocabulary the invite offers. The "candidate" roles are *not*
/// granted on accept; they require a separate authority-escrow request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfferedRoleClass {
    Observer,
    FollowViewer,
    PresenterCandidate,
    DriverCandidate,
}

impl OfferedRoleClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Observer => "observer",
            Self::FollowViewer => "follow_viewer",
            Self::PresenterCandidate => "presenter_candidate",
            Self::DriverCandidate => "driver_candidate",
        }
    }

    /// Whether the offered role is consistent with the invite capability. A
    /// driver/presenter candidate may only ride a control-request-capable
    /// invite; a follow viewer needs at least follow capability.
    pub fn permitted_by_capability(self, capability: InviteCapabilityClass) -> bool {
        match self {
            Self::Observer => true,
            Self::FollowViewer => capability.grants_follow(),
            Self::PresenterCandidate | Self::DriverCandidate => capability.grants_control_request(),
        }
    }
}

/// Closed client-class vocabulary the invite arrives on. The class pins which
/// retention/recording postures are honest for the channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientClass {
    DesktopNative,
    BrowserTab,
    MobileCompanion,
    UntrustedWeb,
}

impl ClientClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopNative => "desktop_native",
            Self::BrowserTab => "browser_tab",
            Self::MobileCompanion => "mobile_companion",
            Self::UntrustedWeb => "untrusted_web",
        }
    }

    /// True for clients that must not carry session retention/recording.
    pub const fn is_untrusted(self) -> bool {
        matches!(self, Self::UntrustedWeb)
    }
}

/// Closed retention/recording-posture vocabulary disclosed on the invite.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionRecordingPostureClass {
    LiveOnlyNoRetention,
    NoRecordingObserverOnly,
    RedactedSessionArchive,
    RecordingWithConsent,
}

impl RetentionRecordingPostureClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveOnlyNoRetention => "live_only_no_retention",
            Self::NoRecordingObserverOnly => "no_recording_observer_only",
            Self::RedactedSessionArchive => "redacted_session_archive",
            Self::RecordingWithConsent => "recording_with_consent",
        }
    }

    /// True when the posture keeps nothing beyond the live view — the only
    /// postures honest to disclose on an untrusted client channel.
    pub const fn is_live_only(self) -> bool {
        matches!(
            self,
            Self::LiveOnlyNoRetention | Self::NoRecordingObserverOnly
        )
    }
}

/// Closed expiry-posture vocabulary. Everything except [`Self::PerpetualNoExpiry`]
/// is a bounded grant; a perpetual grant is never honest for temporary authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpiryPostureClass {
    ExpiresAtFixedTime,
    ExpiresAtSessionEnd,
    ExpiresOnContextChange,
    PerpetualNoExpiry,
}

impl ExpiryPostureClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpiresAtFixedTime => "expires_at_fixed_time",
            Self::ExpiresAtSessionEnd => "expires_at_session_end",
            Self::ExpiresOnContextChange => "expires_on_context_change",
            Self::PerpetualNoExpiry => "perpetual_no_expiry",
        }
    }

    /// True when the grant has a real upper bound.
    pub const fn is_bounded(self) -> bool {
        !matches!(self, Self::PerpetualNoExpiry)
    }
}

/// Closed authority-scope vocabulary for one temporary-control grant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityScopeClass {
    TerminalInput,
    DebugStepControl,
    PresenterHandoff,
    FollowHandoff,
}

impl AuthorityScopeClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TerminalInput => "terminal_input",
            Self::DebugStepControl => "debug_step_control",
            Self::PresenterHandoff => "presenter_handoff",
            Self::FollowHandoff => "follow_handoff",
        }
    }

    /// The lane this scope must be requested against.
    pub const fn required_lane(self) -> SessionLaneClass {
        match self {
            Self::TerminalInput => SessionLaneClass::SharedTerminal,
            Self::DebugStepControl => SessionLaneClass::SharedDebug,
            Self::PresenterHandoff | Self::FollowHandoff => SessionLaneClass::PresentationPreview,
        }
    }

    /// Whether the invite's capability permits requesting this scope. Input,
    /// step/control, and presenter handoff are true control and need the
    /// control-request capability; a follow handoff needs only follow.
    pub fn permitted_by_capability(self, capability: InviteCapabilityClass) -> bool {
        match self {
            Self::TerminalInput | Self::DebugStepControl | Self::PresenterHandoff => {
                capability.grants_control_request()
            }
            Self::FollowHandoff => capability.grants_follow(),
        }
    }
}

/// Closed grant-state lifecycle for a temporary authority grant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantStateClass {
    Requested,
    GrantedActive,
    Expired,
    Revoked,
    FrozenPendingReapproval,
    Denied,
}

impl GrantStateClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::GrantedActive => "granted_active",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
            Self::FrozenPendingReapproval => "frozen_pending_reapproval",
            Self::Denied => "denied",
        }
    }

    /// True when the grant currently confers privileged control.
    pub const fn is_privileged_active(self) -> bool {
        matches!(self, Self::GrantedActive)
    }

    /// True for states that still occupy the live grant slot and therefore must
    /// carry a bounded expiry and reapproval triggers.
    pub const fn is_live(self) -> bool {
        matches!(
            self,
            Self::Requested | Self::GrantedActive | Self::FrozenPendingReapproval
        )
    }
}

/// Closed downgrade-path vocabulary: where authority falls back when the grant
/// ends or is interrupted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradePathClass {
    RevertToObserver,
    RevertToFollow,
    RevertToLocalOnly,
    FreezePendingReapproval,
}

impl DowngradePathClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevertToObserver => "revert_to_observer",
            Self::RevertToFollow => "revert_to_follow",
            Self::RevertToLocalOnly => "revert_to_local_only",
            Self::FreezePendingReapproval => "freeze_pending_reapproval",
        }
    }
}

/// Closed vocabulary of context changes that force reapproval of a live grant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReapprovalTriggerClass {
    OnReconnect,
    OnDeviceHandoff,
    OnSessionRestart,
    OnOwnerChange,
    OnScopeChange,
}

impl ReapprovalTriggerClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnReconnect => "on_reconnect",
            Self::OnDeviceHandoff => "on_device_handoff",
            Self::OnSessionRestart => "on_session_restart",
            Self::OnOwnerChange => "on_owner_change",
            Self::OnScopeChange => "on_scope_change",
        }
    }
}

/// Closed vocabulary of context changes the continuity block can report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextChangeClass {
    NoneSteadyState,
    Reconnected,
    BrowserToDesktopHandoff,
    DesktopToBrowserHandoff,
    AppRestart,
    SessionContextChanged,
}

impl ContextChangeClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneSteadyState => "none_steady_state",
            Self::Reconnected => "reconnected",
            Self::BrowserToDesktopHandoff => "browser_to_desktop_handoff",
            Self::DesktopToBrowserHandoff => "desktop_to_browser_handoff",
            Self::AppRestart => "app_restart",
            Self::SessionContextChanged => "session_context_changed",
        }
    }

    /// True for changes that break the live session context and therefore must
    /// not silently carry privileged control across them.
    pub const fn is_interrupting(self) -> bool {
        !matches!(self, Self::NoneSteadyState)
    }
}

/// Closed action vocabulary offered by the continuity block after a context
/// change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityActionClass {
    ReapproveControl,
    ContinueObserverOnly,
    ContinueLocalOnly,
    RevokeGrant,
    ExportAuthorityRecord,
}

impl ContinuityActionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReapproveControl => "reapprove_control",
            Self::ContinueObserverOnly => "continue_observer_only",
            Self::ContinueLocalOnly => "continue_local_only",
            Self::RevokeGrant => "revoke_grant",
            Self::ExportAuthorityRecord => "export_authority_record",
        }
    }
}

/// Closed durable-attention-surface vocabulary the invite/authority event feeds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionSurfaceClass {
    ActivityCenter,
    DurableAttention,
    NotificationInbox,
    SessionHistory,
}

impl AttentionSurfaceClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActivityCenter => "activity_center",
            Self::DurableAttention => "durable_attention",
            Self::NotificationInbox => "notification_inbox",
            Self::SessionHistory => "session_history",
        }
    }
}

/// Closed export-safe reason-code vocabulary for the durable history event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasonCodeClass {
    InviteOfferedObserverFirst,
    ControlRequestedPending,
    ControlGrantedTemporary,
    ControlExpired,
    ControlRevoked,
    GrantFrozenPendingReapproval,
    ReapprovalRequiredOnContextChange,
    LocalLanePreserved,
}

impl ReasonCodeClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InviteOfferedObserverFirst => "invite_offered_observer_first",
            Self::ControlRequestedPending => "control_requested_pending",
            Self::ControlGrantedTemporary => "control_granted_temporary",
            Self::ControlExpired => "control_expired",
            Self::ControlRevoked => "control_revoked",
            Self::GrantFrozenPendingReapproval => "grant_frozen_pending_reapproval",
            Self::ReapprovalRequiredOnContextChange => "reapproval_required_on_context_change",
            Self::LocalLanePreserved => "local_lane_preserved",
        }
    }

    /// The reason code a durable event must carry for the given grant state, so
    /// the history surface never disagrees with the live grant.
    pub const fn expected_for_state(state: GrantStateClass) -> Self {
        match state {
            GrantStateClass::Requested => Self::ControlRequestedPending,
            GrantStateClass::GrantedActive => Self::ControlGrantedTemporary,
            GrantStateClass::Expired => Self::ControlExpired,
            GrantStateClass::Revoked | GrantStateClass::Denied => Self::ControlRevoked,
            GrantStateClass::FrozenPendingReapproval => Self::GrantFrozenPendingReapproval,
        }
    }
}

/// A bounded expiry window stated on an invite or a grant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpiryWindow {
    pub posture_class: ExpiryPostureClass,
    #[serde(default)]
    pub expires_at_ref: Option<String>,
    pub expiry_label: String,
}

impl ExpiryWindow {
    fn validate(
        &self,
        record_id: &str,
        field: &'static str,
    ) -> Result<(), InviteAuthorityValidationError> {
        if non_empty(&self.expiry_label).is_none() {
            return Err(InviteAuthorityValidationError::EmptyRequiredField {
                record_id: record_id.to_owned(),
                field,
            });
        }
        if let Some(ref_value) = self.expires_at_ref.as_deref() {
            if !ref_is_opaque(ref_value) {
                return Err(InviteAuthorityValidationError::RawRefLeak {
                    record_id: record_id.to_owned(),
                    field,
                });
            }
        }
        // A fixed-time expiry must name the deadline it claims to honor.
        if matches!(self.posture_class, ExpiryPostureClass::ExpiresAtFixedTime)
            && self.expires_at_ref.is_none()
        {
            return Err(InviteAuthorityValidationError::FixedExpiryMissingDeadline {
                record_id: record_id.to_owned(),
            });
        }
        Ok(())
    }
}

/// One invite/join sheet: who is inviting, into what lane, what role, on which
/// client class, with what retention posture and expiry, and exactly what
/// capability accepting confers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InviteSessionManifest {
    pub invite_session_manifest_schema_version: u32,
    pub record_kind: String,
    pub invite_id: String,
    pub session_owner_ref: String,
    pub session_owner_label: String,
    pub session_lane_class: SessionLaneClass,
    pub offered_role_class: OfferedRoleClass,
    pub invite_capability_class: InviteCapabilityClass,
    pub client_class: ClientClass,
    pub retention_recording_posture_class: RetentionRecordingPostureClass,
    pub invite_expiry: ExpiryWindow,
    pub headline_label: String,
    pub manifest_summary: String,
    pub contract_doc_ref: String,
    #[serde(default)]
    pub notes: Option<String>,
}

impl InviteSessionManifest {
    /// Validate the invite manifest against the contract.
    pub fn validate(&self) -> Result<(), InviteAuthorityValidationError> {
        if self.invite_session_manifest_schema_version != INVITE_SESSION_MANIFEST_SCHEMA_VERSION {
            return Err(InviteAuthorityValidationError::WrongManifestSchemaVersion {
                invite_id: self.invite_id.clone(),
                actual: self.invite_session_manifest_schema_version,
            });
        }
        if self.record_kind != INVITE_SESSION_MANIFEST_RECORD_KIND {
            return Err(InviteAuthorityValidationError::WrongManifestRecordKind {
                invite_id: self.invite_id.clone(),
                actual: self.record_kind.clone(),
            });
        }
        if !self.invite_id.starts_with("collab_invite:") {
            return Err(InviteAuthorityValidationError::MalformedInviteId {
                invite_id: self.invite_id.clone(),
            });
        }
        if self.contract_doc_ref != INVITE_AND_AUTHORITY_CONTRACT_DOC_REF {
            return Err(InviteAuthorityValidationError::WrongContractDocRef {
                record_id: self.invite_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }
        for (field, value) in [
            ("headline_label", &self.headline_label),
            ("manifest_summary", &self.manifest_summary),
            ("session_owner_label", &self.session_owner_label),
        ] {
            if non_empty(value).is_none() {
                return Err(InviteAuthorityValidationError::EmptyRequiredField {
                    record_id: self.invite_id.clone(),
                    field,
                });
            }
        }
        if !ref_is_opaque(&self.session_owner_ref) {
            return Err(InviteAuthorityValidationError::RawRefLeak {
                record_id: self.invite_id.clone(),
                field: "session_owner_ref",
            });
        }
        self.invite_expiry
            .validate(&self.invite_id, "invite_expiry")?;

        // The offered role must be consistent with what accepting confers — a
        // driver/presenter candidate may not ride an observer-only invite.
        if !self
            .offered_role_class
            .permitted_by_capability(self.invite_capability_class)
        {
            return Err(InviteAuthorityValidationError::RoleCapabilityMismatch {
                invite_id: self.invite_id.clone(),
                role: self.offered_role_class,
                capability: self.invite_capability_class,
            });
        }

        // An invite that can lead to temporary control must itself be bounded —
        // a control path is never opened by a perpetual invite.
        if self.invite_capability_class.grants_control_request()
            && !self.invite_expiry.posture_class.is_bounded()
        {
            return Err(InviteAuthorityValidationError::InviteExpiryUnbounded {
                invite_id: self.invite_id.clone(),
            });
        }

        // An untrusted client channel may not carry session retention/recording.
        if self.client_class.is_untrusted()
            && !self.retention_recording_posture_class.is_live_only()
        {
            return Err(InviteAuthorityValidationError::RetentionUnsafeForClient {
                invite_id: self.invite_id.clone(),
                client: self.client_class,
                retention: self.retention_recording_posture_class,
            });
        }

        Ok(())
    }
}

/// One temporary authority-escrow ticket: what scope is requested, its lifecycle
/// state, expiry, downgrade path, and the context changes that force reapproval.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityEscrowTicket {
    pub authority_escrow_ticket_schema_version: u32,
    pub record_kind: String,
    pub ticket_id: String,
    pub invite_ref: String,
    pub authority_scope_class: AuthorityScopeClass,
    pub grant_state_class: GrantStateClass,
    pub requested_by_ref: String,
    pub requested_by_label: String,
    pub grant_expiry: ExpiryWindow,
    pub downgrade_path_class: DowngradePathClass,
    pub reapproval_required_on: Vec<ReapprovalTriggerClass>,
    pub scope_summary: String,
    pub headline_label: String,
    pub contract_doc_ref: String,
    #[serde(default)]
    pub notes: Option<String>,
}

impl AuthorityEscrowTicket {
    /// Validate the escrow ticket against the contract.
    pub fn validate(&self) -> Result<(), InviteAuthorityValidationError> {
        if self.authority_escrow_ticket_schema_version != AUTHORITY_ESCROW_TICKET_SCHEMA_VERSION {
            return Err(InviteAuthorityValidationError::WrongTicketSchemaVersion {
                ticket_id: self.ticket_id.clone(),
                actual: self.authority_escrow_ticket_schema_version,
            });
        }
        if self.record_kind != AUTHORITY_ESCROW_TICKET_RECORD_KIND {
            return Err(InviteAuthorityValidationError::WrongTicketRecordKind {
                ticket_id: self.ticket_id.clone(),
                actual: self.record_kind.clone(),
            });
        }
        if !self.ticket_id.starts_with("authority_escrow:") {
            return Err(InviteAuthorityValidationError::MalformedTicketId {
                ticket_id: self.ticket_id.clone(),
            });
        }
        if self.contract_doc_ref != INVITE_AND_AUTHORITY_CONTRACT_DOC_REF {
            return Err(InviteAuthorityValidationError::WrongContractDocRef {
                record_id: self.ticket_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }
        for (field, value) in [
            ("headline_label", &self.headline_label),
            ("scope_summary", &self.scope_summary),
            ("requested_by_label", &self.requested_by_label),
        ] {
            if non_empty(value).is_none() {
                return Err(InviteAuthorityValidationError::EmptyRequiredField {
                    record_id: self.ticket_id.clone(),
                    field,
                });
            }
        }
        if !ref_is_opaque(&self.invite_ref) {
            return Err(InviteAuthorityValidationError::RawRefLeak {
                record_id: self.ticket_id.clone(),
                field: "invite_ref",
            });
        }
        if !ref_is_opaque(&self.requested_by_ref) {
            return Err(InviteAuthorityValidationError::RawRefLeak {
                record_id: self.ticket_id.clone(),
                field: "requested_by_ref",
            });
        }
        self.grant_expiry
            .validate(&self.ticket_id, "grant_expiry")?;

        // A live grant (pending, active, or frozen) must be time-bounded —
        // temporary authority never becomes perpetual.
        if self.grant_state_class.is_live() && !self.grant_expiry.posture_class.is_bounded() {
            return Err(InviteAuthorityValidationError::GrantExpiryUnbounded {
                ticket_id: self.ticket_id.clone(),
                state: self.grant_state_class,
            });
        }

        // A live grant names at least one reapproval trigger so it cannot quietly
        // persist when context changes.
        if self.grant_state_class.is_live() && self.reapproval_required_on.is_empty() {
            return Err(InviteAuthorityValidationError::NoReapprovalTriggers {
                ticket_id: self.ticket_id.clone(),
                state: self.grant_state_class,
            });
        }

        // An *active* privileged grant must require reapproval on the three
        // silent-resume vectors: reconnect, device handoff, and restart.
        if self.grant_state_class.is_privileged_active() {
            let triggers: BTreeSet<ReapprovalTriggerClass> =
                self.reapproval_required_on.iter().copied().collect();
            let required = [
                ReapprovalTriggerClass::OnReconnect,
                ReapprovalTriggerClass::OnDeviceHandoff,
                ReapprovalTriggerClass::OnSessionRestart,
            ];
            if !required.iter().all(|t| triggers.contains(t)) {
                return Err(InviteAuthorityValidationError::SilentResumeNotGuarded {
                    ticket_id: self.ticket_id.clone(),
                });
            }
        }

        Ok(())
    }
}

/// The continuity block: what survives a reconnect, device handoff, or restart.
/// Proves that an interrupting context change never silently resumes privileged
/// control and that the local shared-lane row stays usable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityContinuity {
    pub context_change_class: ContextChangeClass,
    #[serde(default)]
    pub resumed_privileged_silently: bool,
    pub post_change_grant_state_class: GrantStateClass,
    pub reapproval_satisfied: bool,
    pub local_lane_usable: bool,
    pub available_actions: Vec<ContinuityActionClass>,
    #[serde(default)]
    pub preserved_grant_ref: Option<String>,
    pub continuity_summary: String,
}

impl AuthorityContinuity {
    fn validate(&self, sheet_id: &str) -> Result<(), InviteAuthorityValidationError> {
        if non_empty(&self.continuity_summary).is_none() {
            return Err(InviteAuthorityValidationError::EmptyRequiredField {
                record_id: sheet_id.to_owned(),
                field: "authority_continuity.continuity_summary",
            });
        }
        // Silent privileged resume is never allowed, regardless of context.
        if self.resumed_privileged_silently {
            return Err(InviteAuthorityValidationError::SilentResumeNotAllowed {
                sheet_id: sheet_id.to_owned(),
            });
        }
        // The local shared-lane row must remain usable even when the grant is
        // denied or revoked.
        if !self.local_lane_usable {
            return Err(InviteAuthorityValidationError::LocalLaneNotUsable {
                sheet_id: sheet_id.to_owned(),
            });
        }
        if self.available_actions.is_empty() {
            return Err(InviteAuthorityValidationError::NoContinuityActions {
                sheet_id: sheet_id.to_owned(),
            });
        }
        if let Some(grant_ref) = self.preserved_grant_ref.as_deref() {
            if !ref_is_opaque(grant_ref) {
                return Err(InviteAuthorityValidationError::RawRefLeak {
                    record_id: sheet_id.to_owned(),
                    field: "authority_continuity.preserved_grant_ref",
                });
            }
        }
        Ok(())
    }
}

/// The durable history/attention event the invite/authority transition feeds,
/// with a stable id and an export-safe reason code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryAttentionEvent {
    pub event_id: String,
    pub attention_surface_class: AttentionSurfaceClass,
    pub reason_code_class: ReasonCodeClass,
    pub durable: bool,
    pub export_safe: bool,
    pub event_summary: String,
}

impl HistoryAttentionEvent {
    fn validate(&self, sheet_id: &str) -> Result<(), InviteAuthorityValidationError> {
        if !self.event_id.starts_with("collab_authority_event:") || !ref_is_opaque(&self.event_id) {
            return Err(InviteAuthorityValidationError::MalformedEventId {
                event_id: self.event_id.clone(),
            });
        }
        if non_empty(&self.event_summary).is_none() {
            return Err(InviteAuthorityValidationError::EmptyRequiredField {
                record_id: sheet_id.to_owned(),
                field: "history_event.event_summary",
            });
        }
        if !self.durable {
            return Err(InviteAuthorityValidationError::EventNotDurable {
                sheet_id: sheet_id.to_owned(),
            });
        }
        if !self.export_safe {
            return Err(InviteAuthorityValidationError::EventNotExportSafe {
                sheet_id: sheet_id.to_owned(),
            });
        }
        Ok(())
    }
}

/// One review sheet bundling the invite manifest, the escrow ticket, the
/// continuity block, and the durable history event rendered together before a
/// user accepts an invite or approves a control request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InviteAuthorityReviewSheet {
    pub invite_authority_review_sheet_schema_version: u32,
    pub record_kind: String,
    pub sheet_id: String,
    pub sheet_summary: String,
    pub invite_manifest: InviteSessionManifest,
    pub authority_ticket: AuthorityEscrowTicket,
    pub authority_continuity: AuthorityContinuity,
    pub history_event: HistoryAttentionEvent,
    pub contract_doc_ref: String,
    #[serde(default)]
    pub notes: Option<String>,
}

impl InviteAuthorityReviewSheet {
    /// Cross-validate the sheet: each constituent record validates, the requested
    /// scope is one the invite capability permits (observer-first), the scope
    /// matches the invited lane, the durable reason code matches the grant state,
    /// the ticket binds to the invite, and the continuity block never silently
    /// resumes privileged control.
    pub fn validate(&self) -> Result<(), InviteAuthorityValidationError> {
        if self.invite_authority_review_sheet_schema_version
            != INVITE_AUTHORITY_REVIEW_SHEET_SCHEMA_VERSION
        {
            return Err(InviteAuthorityValidationError::WrongSheetSchemaVersion {
                sheet_id: self.sheet_id.clone(),
                actual: self.invite_authority_review_sheet_schema_version,
            });
        }
        if self.record_kind != INVITE_AUTHORITY_REVIEW_SHEET_RECORD_KIND {
            return Err(InviteAuthorityValidationError::WrongSheetRecordKind {
                sheet_id: self.sheet_id.clone(),
                actual: self.record_kind.clone(),
            });
        }
        if !self.sheet_id.starts_with("invite_authority_review_sheet:") {
            return Err(InviteAuthorityValidationError::MalformedSheetId {
                sheet_id: self.sheet_id.clone(),
            });
        }
        if self.contract_doc_ref != INVITE_AND_AUTHORITY_CONTRACT_DOC_REF {
            return Err(InviteAuthorityValidationError::WrongContractDocRef {
                record_id: self.sheet_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }
        if non_empty(&self.sheet_summary).is_none() {
            return Err(InviteAuthorityValidationError::EmptyRequiredField {
                record_id: self.sheet_id.clone(),
                field: "sheet_summary",
            });
        }

        self.invite_manifest.validate()?;
        self.authority_ticket.validate()?;
        self.authority_continuity.validate(&self.sheet_id)?;
        self.history_event.validate(&self.sheet_id)?;

        // The ticket binds to the bundled invite by stable id.
        if self.authority_ticket.invite_ref != self.invite_manifest.invite_id {
            return Err(InviteAuthorityValidationError::TicketInviteRefMismatch {
                sheet_id: self.sheet_id.clone(),
                invite_ref: self.authority_ticket.invite_ref.clone(),
                invite_id: self.invite_manifest.invite_id.clone(),
            });
        }

        // Observer-first: the requested authority scope must be one the invite's
        // capability actually permits.
        if !self
            .authority_ticket
            .authority_scope_class
            .permitted_by_capability(self.invite_manifest.invite_capability_class)
        {
            return Err(
                InviteAuthorityValidationError::ControlRequestedWithoutCapability {
                    sheet_id: self.sheet_id.clone(),
                    scope: self.authority_ticket.authority_scope_class,
                    capability: self.invite_manifest.invite_capability_class,
                },
            );
        }

        // The requested scope must match the invited lane — a terminal grant
        // belongs to a terminal invite, not a presentation row.
        if self.authority_ticket.authority_scope_class.required_lane()
            != self.invite_manifest.session_lane_class
        {
            return Err(InviteAuthorityValidationError::ScopeLaneMismatch {
                sheet_id: self.sheet_id.clone(),
                scope: self.authority_ticket.authority_scope_class,
                lane: self.invite_manifest.session_lane_class,
            });
        }

        // The durable reason code must match the grant state so the history
        // surface never disagrees with the live grant.
        let expected = ReasonCodeClass::expected_for_state(self.authority_ticket.grant_state_class);
        if self.history_event.reason_code_class != expected {
            return Err(InviteAuthorityValidationError::ReasonCodeStateMismatch {
                sheet_id: self.sheet_id.clone(),
                reason: self.history_event.reason_code_class,
                expected,
                state: self.authority_ticket.grant_state_class,
            });
        }

        // No silent privileged resume: after an interrupting context change, a
        // grant that is still active requires explicit reapproval.
        if self
            .authority_continuity
            .context_change_class
            .is_interrupting()
            && self
                .authority_continuity
                .post_change_grant_state_class
                .is_privileged_active()
            && !self.authority_continuity.reapproval_satisfied
        {
            return Err(InviteAuthorityValidationError::SilentPrivilegedResume {
                sheet_id: self.sheet_id.clone(),
                context_change: self.authority_continuity.context_change_class,
            });
        }

        // A preserved grant ref, when present, must point at this ticket.
        if let Some(grant_ref) = self.authority_continuity.preserved_grant_ref.as_deref() {
            if grant_ref != self.authority_ticket.ticket_id {
                return Err(InviteAuthorityValidationError::PreservedGrantRefMismatch {
                    sheet_id: self.sheet_id.clone(),
                    preserved: grant_ref.to_owned(),
                    ticket_id: self.authority_ticket.ticket_id.clone(),
                });
            }
        }

        Ok(())
    }

    /// Render a deterministic plaintext block for support exports and
    /// reviewer-facing previews. Stable for the same input snapshot.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Collaboration invite and temporary authority-escrow review\n");
        out.push_str(&format!("Sheet: {}\n", self.sheet_id));
        out.push_str(&format!("Summary: {}\n\n", self.sheet_summary));

        let m = &self.invite_manifest;
        out.push_str("Invite:\n");
        out.push_str(&format!(
            "- [{}] {} — lane={} role={} capability={} client={}\n",
            m.invite_id,
            m.headline_label,
            m.session_lane_class.as_str(),
            m.offered_role_class.as_str(),
            m.invite_capability_class.as_str(),
            m.client_class.as_str(),
        ));
        out.push_str(&format!("    owner: {}\n", m.session_owner_ref));
        out.push_str(&format!(
            "    retention: {} expiry={} ({})\n",
            m.retention_recording_posture_class.as_str(),
            m.invite_expiry.posture_class.as_str(),
            m.invite_expiry.expiry_label,
        ));
        out.push('\n');

        let t = &self.authority_ticket;
        out.push_str("Authority escrow:\n");
        out.push_str(&format!(
            "- [{}] {} — scope={} state={} downgrade={}\n",
            t.ticket_id,
            t.headline_label,
            t.authority_scope_class.as_str(),
            t.grant_state_class.as_str(),
            t.downgrade_path_class.as_str(),
        ));
        out.push_str(&format!(
            "    expiry={} ({})\n",
            t.grant_expiry.posture_class.as_str(),
            t.grant_expiry.expiry_label,
        ));
        let triggers: Vec<&str> = t
            .reapproval_required_on
            .iter()
            .map(|trigger| trigger.as_str())
            .collect();
        out.push_str(&format!("    reapproval on: {}\n", triggers.join(", ")));
        out.push('\n');

        let c = &self.authority_continuity;
        out.push_str("Continuity:\n");
        out.push_str(&format!(
            "- context={} post_state={} reapproval_satisfied={} local_lane_usable={}\n",
            c.context_change_class.as_str(),
            c.post_change_grant_state_class.as_str(),
            c.reapproval_satisfied,
            c.local_lane_usable,
        ));
        let actions: Vec<&str> = c.available_actions.iter().map(|a| a.as_str()).collect();
        out.push_str(&format!("    actions: {}\n", actions.join(", ")));
        out.push('\n');

        let e = &self.history_event;
        out.push_str("Durable event:\n");
        out.push_str(&format!(
            "- [{}] surface={} reason={} durable={} export_safe={}\n",
            e.event_id,
            e.attention_surface_class.as_str(),
            e.reason_code_class.as_str(),
            e.durable,
            e.export_safe,
        ));
        out
    }
}

/// True when a ref is an opaque token rather than a raw URL, email, path, or
/// blank — opaque refs are the only thing safe on live and exported surfaces.
fn ref_is_opaque(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty()
        && trimmed == value
        && !trimmed.contains("://")
        && !trimmed.contains('@')
        && !trimmed.contains(char::is_whitespace)
}

fn non_empty(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

/// Closed validation-error vocabulary for the invite/authority contracts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InviteAuthorityValidationError {
    WrongManifestSchemaVersion {
        invite_id: String,
        actual: u32,
    },
    WrongManifestRecordKind {
        invite_id: String,
        actual: String,
    },
    MalformedInviteId {
        invite_id: String,
    },
    RoleCapabilityMismatch {
        invite_id: String,
        role: OfferedRoleClass,
        capability: InviteCapabilityClass,
    },
    InviteExpiryUnbounded {
        invite_id: String,
    },
    RetentionUnsafeForClient {
        invite_id: String,
        client: ClientClass,
        retention: RetentionRecordingPostureClass,
    },

    WrongTicketSchemaVersion {
        ticket_id: String,
        actual: u32,
    },
    WrongTicketRecordKind {
        ticket_id: String,
        actual: String,
    },
    MalformedTicketId {
        ticket_id: String,
    },
    GrantExpiryUnbounded {
        ticket_id: String,
        state: GrantStateClass,
    },
    NoReapprovalTriggers {
        ticket_id: String,
        state: GrantStateClass,
    },
    SilentResumeNotGuarded {
        ticket_id: String,
    },
    FixedExpiryMissingDeadline {
        record_id: String,
    },

    WrongSheetSchemaVersion {
        sheet_id: String,
        actual: u32,
    },
    WrongSheetRecordKind {
        sheet_id: String,
        actual: String,
    },
    MalformedSheetId {
        sheet_id: String,
    },
    MalformedEventId {
        event_id: String,
    },
    TicketInviteRefMismatch {
        sheet_id: String,
        invite_ref: String,
        invite_id: String,
    },
    ControlRequestedWithoutCapability {
        sheet_id: String,
        scope: AuthorityScopeClass,
        capability: InviteCapabilityClass,
    },
    ScopeLaneMismatch {
        sheet_id: String,
        scope: AuthorityScopeClass,
        lane: SessionLaneClass,
    },
    ReasonCodeStateMismatch {
        sheet_id: String,
        reason: ReasonCodeClass,
        expected: ReasonCodeClass,
        state: GrantStateClass,
    },
    SilentPrivilegedResume {
        sheet_id: String,
        context_change: ContextChangeClass,
    },
    SilentResumeNotAllowed {
        sheet_id: String,
    },
    LocalLaneNotUsable {
        sheet_id: String,
    },
    NoContinuityActions {
        sheet_id: String,
    },
    EventNotDurable {
        sheet_id: String,
    },
    EventNotExportSafe {
        sheet_id: String,
    },
    PreservedGrantRefMismatch {
        sheet_id: String,
        preserved: String,
        ticket_id: String,
    },

    WrongContractDocRef {
        record_id: String,
        actual: String,
    },
    EmptyRequiredField {
        record_id: String,
        field: &'static str,
    },
    RawRefLeak {
        record_id: String,
        field: &'static str,
    },
}

impl fmt::Display for InviteAuthorityValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongManifestSchemaVersion { invite_id, actual } => write!(
                f,
                "invite {invite_id} has unsupported invite_session_manifest_schema_version {actual}"
            ),
            Self::WrongManifestRecordKind { invite_id, actual } => write!(
                f,
                "invite {invite_id} has unsupported record kind {actual}"
            ),
            Self::MalformedInviteId { invite_id } => {
                write!(f, "invite id {invite_id} must start with collab_invite:")
            }
            Self::RoleCapabilityMismatch {
                invite_id,
                role,
                capability,
            } => write!(
                f,
                "invite {invite_id} offers role {} which is not permitted by capability {}",
                role.as_str(),
                capability.as_str()
            ),
            Self::InviteExpiryUnbounded { invite_id } => write!(
                f,
                "invite {invite_id} can request control but carries a perpetual (unbounded) expiry"
            ),
            Self::RetentionUnsafeForClient {
                invite_id,
                client,
                retention,
            } => write!(
                f,
                "invite {invite_id} client {} cannot carry retention posture {}",
                client.as_str(),
                retention.as_str()
            ),
            Self::WrongTicketSchemaVersion { ticket_id, actual } => write!(
                f,
                "ticket {ticket_id} has unsupported authority_escrow_ticket_schema_version {actual}"
            ),
            Self::WrongTicketRecordKind { ticket_id, actual } => write!(
                f,
                "ticket {ticket_id} has unsupported record kind {actual}"
            ),
            Self::MalformedTicketId { ticket_id } => write!(
                f,
                "ticket id {ticket_id} must start with authority_escrow:"
            ),
            Self::GrantExpiryUnbounded { ticket_id, state } => write!(
                f,
                "ticket {ticket_id} in state {} must carry a bounded expiry; temporary authority is never perpetual",
                state.as_str()
            ),
            Self::NoReapprovalTriggers { ticket_id, state } => write!(
                f,
                "ticket {ticket_id} in state {} must name at least one reapproval trigger",
                state.as_str()
            ),
            Self::SilentResumeNotGuarded { ticket_id } => write!(
                f,
                "ticket {ticket_id} is active but does not require reapproval on reconnect, device handoff, and restart"
            ),
            Self::FixedExpiryMissingDeadline { record_id } => write!(
                f,
                "record {record_id} declares a fixed-time expiry without naming the deadline ref"
            ),
            Self::WrongSheetSchemaVersion { sheet_id, actual } => write!(
                f,
                "sheet {sheet_id} has unsupported invite_authority_review_sheet_schema_version {actual}"
            ),
            Self::WrongSheetRecordKind { sheet_id, actual } => write!(
                f,
                "sheet {sheet_id} has unsupported record kind {actual}"
            ),
            Self::MalformedSheetId { sheet_id } => write!(
                f,
                "sheet id {sheet_id} must start with invite_authority_review_sheet:"
            ),
            Self::MalformedEventId { event_id } => write!(
                f,
                "history event id {event_id} must start with collab_authority_event: and be opaque"
            ),
            Self::TicketInviteRefMismatch {
                sheet_id,
                invite_ref,
                invite_id,
            } => write!(
                f,
                "sheet {sheet_id} ticket invite_ref {invite_ref} does not match invite id {invite_id}"
            ),
            Self::ControlRequestedWithoutCapability {
                sheet_id,
                scope,
                capability,
            } => write!(
                f,
                "sheet {sheet_id} requests scope {} which capability {} does not permit (observer-first)",
                scope.as_str(),
                capability.as_str()
            ),
            Self::ScopeLaneMismatch {
                sheet_id,
                scope,
                lane,
            } => write!(
                f,
                "sheet {sheet_id} scope {} does not belong to invited lane {}",
                scope.as_str(),
                lane.as_str()
            ),
            Self::ReasonCodeStateMismatch {
                sheet_id,
                reason,
                expected,
                state,
            } => write!(
                f,
                "sheet {sheet_id} durable reason {} disagrees with grant state {} (expected {})",
                reason.as_str(),
                state.as_str(),
                expected.as_str()
            ),
            Self::SilentPrivilegedResume {
                sheet_id,
                context_change,
            } => write!(
                f,
                "sheet {sheet_id} resumes an active grant after {} without explicit reapproval",
                context_change.as_str()
            ),
            Self::SilentResumeNotAllowed { sheet_id } => write!(
                f,
                "sheet {sheet_id} declares resumed_privileged_silently; privileged control must be reapproved"
            ),
            Self::LocalLaneNotUsable { sheet_id } => write!(
                f,
                "sheet {sheet_id} marks the local shared-lane row unusable; local use must survive a denied or revoked grant"
            ),
            Self::NoContinuityActions { sheet_id } => write!(
                f,
                "sheet {sheet_id} continuity must offer at least one action"
            ),
            Self::EventNotDurable { sheet_id } => write!(
                f,
                "sheet {sheet_id} history event must be durable"
            ),
            Self::EventNotExportSafe { sheet_id } => write!(
                f,
                "sheet {sheet_id} history event must be export-safe"
            ),
            Self::PreservedGrantRefMismatch {
                sheet_id,
                preserved,
                ticket_id,
            } => write!(
                f,
                "sheet {sheet_id} preserved grant ref {preserved} does not match ticket {ticket_id}"
            ),
            Self::WrongContractDocRef { record_id, actual } => write!(
                f,
                "record {record_id} cites wrong contract doc {actual}"
            ),
            Self::EmptyRequiredField { record_id, field } => {
                write!(f, "record {record_id} is missing required field {field}")
            }
            Self::RawRefLeak { record_id, field } => write!(
                f,
                "record {record_id} field {field} contains a raw URL, email, path, or whitespace; opaque refs only"
            ),
        }
    }
}

impl std::error::Error for InviteAuthorityValidationError {}

/// Convenience: validate a slice of sheets and reject duplicate sheet ids.
pub fn validate_sheets(
    sheets: &[InviteAuthorityReviewSheet],
) -> Result<(), InviteAuthorityValidationError> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for sheet in sheets {
        sheet.validate()?;
        if !seen.insert(sheet.sheet_id.as_str()) {
            return Err(InviteAuthorityValidationError::MalformedSheetId {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
