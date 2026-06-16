//! Auth-callback and deep-link review intake, spoof-resistant origin
//! disclosure, and confirm/reject parity for OS-initiated returns.
//!
//! Aureline's local-first promise has to survive the moment a flow *returns
//! from outside the product*: the system browser hands back an auth callback, a
//! protocol handler delivers a deep link, a review or collaboration link
//! reopens the app, or a companion resumes a managed action. Each of those is a
//! moment where a wrong origin, an expired link, or a silently widened
//! authority could ride in unreviewed. This module makes that decision explicit
//! and reviewable. Every callback or deep-link entry path is projected as one
//! typed [`CallbackReviewDescriptor`] that:
//!
//! - discloses **who asked** — the source class that delivered the return and a
//!   spoof-resistant **origin assurance** class that records how the origin was
//!   verified (a strict origin match, a pinned loopback port, a pinned deep-link
//!   scheme, a matched device-code poll, or a first-party signed link) so a
//!   wrong or unverified origin can never masquerade as a trusted return;
//! - names **what scope they requested** — the requested action class, the
//!   workspace and tenant scope refs, and the [`AuthorityScopeClass`] the return
//!   would reach, plus whether that authority is broader than a plain local
//!   open;
//! - declares **why the user is asked to confirm or reject** — any authority
//!   wider than a plain local open requires an explicit confirm/reject sheet
//!   before it commits, so a callback can never silently join a collaboration,
//!   resume a managed action, or open a remote mutation; and
//! - preserves **local-only continuity on failure** — a denied, wrong-origin,
//!   expired, or stale return lands on a truthful placeholder or recovery sheet
//!   with bounded recovery actions rather than an empty shell that discards the
//!   original intent.
//!
//! The resulting [`CallbackReviewReport`] is the canonical truth object for the
//! callback/deep-link review lane. It is consumed by:
//!
//! - the live shell entry interstitials and the auth-recovery surface, which
//!   render the same origin / scope / confirm-reject disclosure the CLI prints;
//! - the headless inspector (`aureline_auth_m5_callback_and_deep_link_review`),
//!   the only mint-from-truth path for the JSON fixtures checked in under
//!   `fixtures/platform/m5-callback-and-deep-link/`;
//! - the support-export wrapper and per-incident exports, so a reviewer can
//!   reproduce a denied, wrong-origin, expired, or stale callback from typed
//!   diagnostics instead of screenshots; and
//! - the markdown artifact under
//!   `artifacts/platform/m5-auth-callback-and-deep-link.md` and the companion
//!   doc under `docs/m5/auth-callback-and-protocol-handlers.md`.
//!
//! The review layer extends the existing browser-handoff, embedded-boundary,
//! provider-origin, and auth-recovery rows: the report cross-links the
//! browser-callback packet, the embedded-boundary surface, the provider-origin
//! disclosure, the auth-and-recovery packet, the system-entry intake matrix,
//! and the entry interstitials so trust vocabulary cannot drift independently.
//!
//! Acceptance invariants enforced by the validator:
//!
//! 1. Every required entry kind is present — auth-provider callback, protocol
//!    deep link, review handoff link, collaboration join link, managed resume
//!    link, and remote mutation link — and each entry carries a disclosed
//!    origin, a target identity, a pending correlation alias, an expiry, an
//!    active-profile owner, a trust checkpoint, and the canonical in-product
//!    command its confirm action routes to.
//! 2. Any authority class wider than a plain local open requires an explicit
//!    confirm/reject sheet; an auto-admit that widens authority without one is a
//!    [`SilentAuthorityWiden`] blocker, and one that opens a remote mutation
//!    without one is a distinct [`SilentRemoteMutation`] blocker — the two never
//!    collapse into a single finding.
//! 3. An admitted return whose origin could not be verified is an
//!    [`OriginVerificationBypassed`] blocker, so spoof-resistance can never be
//!    skipped for convenience.
//! 4. A denied return carries at least one recovery action, and each denial
//!    class stays a distinct failure: a wrong-origin denial with no recovery is
//!    a [`WrongOriginLooksLikeAuthFailure`] blocker, an expired one is an
//!    [`ExpiredSilentNoOp`] blocker, a stale one is a [`StaleStateUnsurfaced`]
//!    blocker, and a policy denial is a [`PolicyDenialDeadEnd`] blocker.
//! 5. A return that puts local intent at risk is a [`LocalContinuityLost`]
//!    blocker, and an entry that leaks a raw URL or token body is a
//!    [`RawTargetLeak`] blocker, so packets stay redaction-safe.
//! 6. Stale evidence on a marketed entry is a blocker so release tooling can
//!    narrow the surface instead of shipping it as implicitly stable.
//!
//! All identifiers, refs, and label strings are deterministic so the
//! checked-in fixtures under `fixtures/platform/m5-callback-and-deep-link/` are
//! bit-for-bit equal to the seeded report returned by
//! [`seeded_callback_review_report`].
//!
//! [`SilentAuthorityWiden`]: CallbackFailureMode::SilentAuthorityWiden
//! [`SilentRemoteMutation`]: CallbackFailureMode::SilentRemoteMutation
//! [`OriginVerificationBypassed`]: CallbackFailureMode::OriginVerificationBypassed
//! [`WrongOriginLooksLikeAuthFailure`]: CallbackFailureMode::WrongOriginLooksLikeAuthFailure
//! [`ExpiredSilentNoOp`]: CallbackFailureMode::ExpiredSilentNoOp
//! [`StaleStateUnsurfaced`]: CallbackFailureMode::StaleStateUnsurfaced
//! [`PolicyDenialDeadEnd`]: CallbackFailureMode::PolicyDenialDeadEnd
//! [`LocalContinuityLost`]: CallbackFailureMode::LocalContinuityLost
//! [`RawTargetLeak`]: CallbackFailureMode::RawTargetLeak

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version exported with every callback-review record.
pub const CALLBACK_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every callback-review surface.
pub const CALLBACK_REVIEW_SHARED_CONTRACT_REF: &str = "auth:m5_callback_and_deep_link_review:v1";

/// Stable record kind for [`CallbackReviewReport`] payloads.
pub const CALLBACK_REVIEW_REPORT_RECORD_KIND: &str =
    "auth_m5_callback_and_deep_link_review_report_record";

/// Stable record kind for [`CallbackReviewRow`] payloads.
pub const CALLBACK_REVIEW_ROW_RECORD_KIND: &str =
    "auth_m5_callback_and_deep_link_review_entry_record";

/// Stable record kind for [`CallbackReviewSupportExport`] payloads.
pub const CALLBACK_REVIEW_SUPPORT_EXPORT_RECORD_KIND: &str =
    "auth_m5_callback_and_deep_link_review_support_export_record";

/// Stable record kind for [`CallbackReviewCaseExport`] payloads.
pub const CALLBACK_REVIEW_CASE_EXPORT_RECORD_KIND: &str =
    "auth_m5_callback_and_deep_link_review_case_export_record";

/// Stable report id quoted across surfaces.
pub const CALLBACK_REVIEW_REPORT_ID: &str = "auth:m5_callback_and_deep_link_review:report:v1";

/// Stable support-export id quoted in the published wrapper.
pub const CALLBACK_REVIEW_SUPPORT_EXPORT_ID: &str = "support-export:m5-callback-and-deep-link:001";

/// Source schema ref for the canonical callback-review contract.
pub const CALLBACK_REVIEW_SOURCE_SCHEMA_REF: &str =
    "schemas/platform/m5-deep-link-review.schema.json";

/// Path of the published markdown artifact.
pub const CALLBACK_REVIEW_PUBLISHED_REPORT_REF: &str =
    "artifacts/platform/m5-auth-callback-and-deep-link.md";

/// Path of the published companion doc.
pub const CALLBACK_REVIEW_PUBLISHED_DOC_REF: &str =
    "docs/m5/auth-callback-and-protocol-handlers.md";

/// Generation timestamp captured in every seeded record.
const GENERATED_AT: &str = "2026-06-16T00:00:00Z";

/// One callback or deep-link entry kind the review layer governs.
///
/// These are the six return classes the lane requires the review object to
/// disclose and gate through a single typed path, regardless of which OS
/// affordance delivered the return.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallbackEntryKind {
    /// A browser auth callback returning to a pending sign-in.
    AuthProviderCallback,
    /// A protocol / deep-link scheme open of an existing local context.
    ProtocolDeepLink,
    /// A review or work-item deep link routed to the review surface.
    ReviewHandoffLink,
    /// A collaboration join link that joins presence in a workspace.
    CollaborationJoinLink,
    /// A managed-action resume link routed through a trusted companion.
    ManagedResumeLink,
    /// A provider link that would open a remote mutation.
    RemoteMutationLink,
}

impl CallbackEntryKind {
    /// Returns the stable schema token for this entry kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthProviderCallback => "auth_provider_callback",
            Self::ProtocolDeepLink => "protocol_deep_link",
            Self::ReviewHandoffLink => "review_handoff_link",
            Self::CollaborationJoinLink => "collaboration_join_link",
            Self::ManagedResumeLink => "managed_resume_link",
            Self::RemoteMutationLink => "remote_mutation_link",
        }
    }

    /// Returns the reviewer-facing label for this entry kind.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::AuthProviderCallback => "Auth-provider callback",
            Self::ProtocolDeepLink => "Protocol deep link",
            Self::ReviewHandoffLink => "Review handoff link",
            Self::CollaborationJoinLink => "Collaboration join link",
            Self::ManagedResumeLink => "Managed resume link",
            Self::RemoteMutationLink => "Remote mutation link",
        }
    }

    /// Returns the six required entry kinds in canonical order.
    pub const fn required_kinds() -> [Self; 6] {
        [
            Self::AuthProviderCallback,
            Self::ProtocolDeepLink,
            Self::ReviewHandoffLink,
            Self::CollaborationJoinLink,
            Self::ManagedResumeLink,
            Self::RemoteMutationLink,
        ]
    }
}

/// The source that delivered a return — the spoof-resistant *who asked* axis.
///
/// The entry kind says *what* returned; the source class says *which* origin
/// handed it over. The two are tracked separately so a wrong-origin incident on
/// an external-provider return is a different diagnostic from the same kind
/// arriving through the system default browser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallbackSourceClass {
    /// The system default browser returned to the app.
    SystemDefaultBrowserReturn,
    /// A registered protocol / deep-link scheme handler.
    RegisteredProtocolHandler,
    /// A first-party web surface returned to the app.
    FirstPartyWebReturn,
    /// A trusted companion app handed off the return.
    TrustedCompanionApp,
    /// An external identity / collaboration provider.
    ExternalProvider,
    /// A collaboration service delivered the join.
    CollaborationService,
    /// The origin could not be attributed to a trusted source.
    UnknownUntrusted,
}

impl CallbackSourceClass {
    /// Returns the stable schema token for this source class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemDefaultBrowserReturn => "system_default_browser_return",
            Self::RegisteredProtocolHandler => "registered_protocol_handler",
            Self::FirstPartyWebReturn => "first_party_web_return",
            Self::TrustedCompanionApp => "trusted_companion_app",
            Self::ExternalProvider => "external_provider",
            Self::CollaborationService => "collaboration_service",
            Self::UnknownUntrusted => "unknown_untrusted",
        }
    }

    /// Returns the reviewer-facing label for this source class.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::SystemDefaultBrowserReturn => "System default browser",
            Self::RegisteredProtocolHandler => "Registered protocol handler",
            Self::FirstPartyWebReturn => "First-party web return",
            Self::TrustedCompanionApp => "Trusted companion app",
            Self::ExternalProvider => "External provider",
            Self::CollaborationService => "Collaboration service",
            Self::UnknownUntrusted => "Unknown / untrusted",
        }
    }
}

/// How the origin of a return was verified — the spoof-resistance axis.
///
/// An admitted return MUST carry a verified assurance class; an admitted return
/// on [`OriginUnverified`](Self::OriginUnverified) is an
/// [`OriginVerificationBypassed`](CallbackFailureMode::OriginVerificationBypassed)
/// blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginAssuranceClass {
    /// The return matched the bound origin, state, and PKCE exactly.
    StrictOriginMatched,
    /// A loopback callback arrived on the pinned port the handoff bound.
    LoopbackPortPinned,
    /// A deep-link arrived on the pinned scheme with a matched signed state.
    DeepLinkSchemePinned,
    /// A device-code poll matched the pending device code.
    DeviceCodePollMatched,
    /// A first-party signed link verified against the first-party signer.
    FirstPartySignedLink,
    /// The origin could not be verified. A spoof-suspect return.
    OriginUnverified,
}

impl OriginAssuranceClass {
    /// Returns the stable schema token for this assurance class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StrictOriginMatched => "strict_origin_matched",
            Self::LoopbackPortPinned => "loopback_port_pinned",
            Self::DeepLinkSchemePinned => "deep_link_scheme_pinned",
            Self::DeviceCodePollMatched => "device_code_poll_matched",
            Self::FirstPartySignedLink => "first_party_signed_link",
            Self::OriginUnverified => "origin_unverified",
        }
    }

    /// Returns the reviewer-facing label for this assurance class.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::StrictOriginMatched => "Strict origin matched",
            Self::LoopbackPortPinned => "Loopback port pinned",
            Self::DeepLinkSchemePinned => "Deep-link scheme pinned",
            Self::DeviceCodePollMatched => "Device-code poll matched",
            Self::FirstPartySignedLink => "First-party signed link",
            Self::OriginUnverified => "Origin unverified",
        }
    }

    /// `true` when the origin was provably verified.
    pub const fn is_verified(self) -> bool {
        !matches!(self, Self::OriginUnverified)
    }
}

/// The action a returning callback or deep link requests — *what they want*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestedActionClass {
    /// Open an existing, already-trusted local context. The fast path.
    OpenExistingLocalContext,
    /// Inspect a review or work item, read-only, across a boundary.
    InspectReviewItem,
    /// Resume a pending sign-in returning from the system browser.
    ResumePendingSignIn,
    /// Join a collaboration / presence in a workspace.
    JoinCollaboration,
    /// Resume a managed action or session.
    ResumeManagedAction,
    /// Open a remote, mutating provider flow.
    OpenRemoteMutation,
    /// Explicitly widen authority or scope.
    WidenAuthority,
}

impl RequestedActionClass {
    /// Returns the stable schema token for this action class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenExistingLocalContext => "open_existing_local_context",
            Self::InspectReviewItem => "inspect_review_item",
            Self::ResumePendingSignIn => "resume_pending_sign_in",
            Self::JoinCollaboration => "join_collaboration",
            Self::ResumeManagedAction => "resume_managed_action",
            Self::OpenRemoteMutation => "open_remote_mutation",
            Self::WidenAuthority => "widen_authority",
        }
    }

    /// Returns the reviewer-facing label for this action class.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::OpenExistingLocalContext => "Open existing local context",
            Self::InspectReviewItem => "Inspect review item",
            Self::ResumePendingSignIn => "Resume pending sign-in",
            Self::JoinCollaboration => "Join collaboration",
            Self::ResumeManagedAction => "Resume managed action",
            Self::OpenRemoteMutation => "Open remote mutation",
            Self::WidenAuthority => "Widen authority",
        }
    }

    /// `true` when this action widens authority beyond a plain local open and
    /// therefore must be gated behind a confirm/reject sheet.
    pub const fn widens_authority(self) -> bool {
        !matches!(self, Self::OpenExistingLocalContext)
    }

    /// `true` when the requested action is consistent with the authority scope
    /// the entry declares. A mismatch means the disclosed *what* and *how much*
    /// disagree, which would let a widening action hide behind a narrow scope.
    pub fn is_consistent_with(self, scope: AuthorityScopeClass) -> bool {
        match self {
            Self::OpenExistingLocalContext => scope == AuthorityScopeClass::PlainLocalOpen,
            Self::InspectReviewItem => scope == AuthorityScopeClass::CrossesBoundaryReadOnly,
            Self::JoinCollaboration => scope == AuthorityScopeClass::WorkspaceCollaborationJoin,
            Self::ResumePendingSignIn | Self::ResumeManagedAction => {
                scope == AuthorityScopeClass::WidensToManagedAuthority
            }
            Self::OpenRemoteMutation => scope == AuthorityScopeClass::WidensToProviderMutation,
            Self::WidenAuthority => matches!(
                scope,
                AuthorityScopeClass::WidensToManagedAuthority
                    | AuthorityScopeClass::WidensToProviderMutation
            ),
        }
    }
}

/// The authority a return would reach once it commits.
///
/// The lane invariant is that nothing wider than a plain local open may commit
/// without an explicit confirm/reject sheet. Every class other than
/// [`PlainLocalOpen`](Self::PlainLocalOpen) requires one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityScopeClass {
    /// An exact, local, already-trusted open. The fast path; no confirm/reject.
    PlainLocalOpen,
    /// Crosses a network, review, or tenant boundary to inspect a remote target
    /// (still read-only).
    CrossesBoundaryReadOnly,
    /// Joins workspace collaboration / presence.
    WorkspaceCollaborationJoin,
    /// Widens to managed authority (managed sign-in or managed-action resume).
    WidensToManagedAuthority,
    /// Would trigger a mutating provider-side flow.
    WidensToProviderMutation,
}

impl AuthorityScopeClass {
    /// Returns the stable schema token for this authority scope.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlainLocalOpen => "plain_local_open",
            Self::CrossesBoundaryReadOnly => "crosses_boundary_read_only",
            Self::WorkspaceCollaborationJoin => "workspace_collaboration_join",
            Self::WidensToManagedAuthority => "widens_to_managed_authority",
            Self::WidensToProviderMutation => "widens_to_provider_mutation",
        }
    }

    /// Returns the reviewer-facing label for this authority scope.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::PlainLocalOpen => "Plain local open",
            Self::CrossesBoundaryReadOnly => "Crosses boundary (read-only)",
            Self::WorkspaceCollaborationJoin => "Workspace collaboration join",
            Self::WidensToManagedAuthority => "Widens to managed authority",
            Self::WidensToProviderMutation => "Widens to provider mutation",
        }
    }

    /// `true` when committing this scope MUST be gated behind an explicit
    /// confirm/reject sheet rather than auto-admitted.
    pub const fn requires_confirm_reject(self) -> bool {
        !matches!(self, Self::PlainLocalOpen)
    }
}

/// The disposition of a return at review time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallbackOutcomeClass {
    /// The return was confirmed (or auto-admitted for a plain local open).
    Admitted,
    /// The return's origin was wrong or could not be verified.
    DeniedWrongOrigin,
    /// The return's expiry passed before it was reviewed.
    DeniedExpired,
    /// The pending session the return targets was superseded or is stale.
    DeniedStale,
    /// The return was blocked by policy.
    DeniedByPolicy,
}

impl CallbackOutcomeClass {
    /// Returns the stable schema token for this outcome.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::DeniedWrongOrigin => "denied_wrong_origin",
            Self::DeniedExpired => "denied_expired",
            Self::DeniedStale => "denied_stale",
            Self::DeniedByPolicy => "denied_by_policy",
        }
    }

    /// Returns the reviewer-facing label for this outcome.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::Admitted => "Admitted",
            Self::DeniedWrongOrigin => "Denied: wrong origin",
            Self::DeniedExpired => "Denied: expired",
            Self::DeniedStale => "Denied: stale",
            Self::DeniedByPolicy => "Denied: policy",
        }
    }

    /// `true` when this outcome is a denial and therefore requires at least one
    /// recovery action.
    pub const fn requires_recovery(self) -> bool {
        !matches!(self, Self::Admitted)
    }

    /// The distinct failure mode a missing recovery action raises for this
    /// outcome. The four denial failure classes are never collapsed.
    pub const fn missing_recovery_failure_mode(self) -> Option<CallbackFailureMode> {
        match self {
            Self::Admitted => None,
            Self::DeniedWrongOrigin => Some(CallbackFailureMode::WrongOriginLooksLikeAuthFailure),
            Self::DeniedExpired => Some(CallbackFailureMode::ExpiredSilentNoOp),
            Self::DeniedStale => Some(CallbackFailureMode::StaleStateUnsurfaced),
            Self::DeniedByPolicy => Some(CallbackFailureMode::PolicyDenialDeadEnd),
        }
    }
}

/// A bounded recovery action a denied or degraded return offers instead of
/// dead-ending.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallbackRecoveryAction {
    /// Retry the flow in the system browser.
    RetryInSystemBrowser,
    /// Keep working locally without finishing the callback.
    ContinueLocalWithoutCallback,
    /// Return to the exact pending sign-in to resume.
    ReturnToPendingSignIn,
    /// Show why the origin did not match.
    ShowOriginMismatchDetail,
    /// Return to the review surface that owns the link.
    ReturnToReviewSurface,
    /// Request a fresh link to replace the expired one.
    RequestFreshLink,
    /// Show the policy block detail and the contact path.
    ShowPolicyBlockDetail,
    /// Keep local work and dismiss the failed return.
    KeepLocalWorkAndDismiss,
}

impl CallbackRecoveryAction {
    /// Returns the stable schema token for this recovery action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetryInSystemBrowser => "retry_in_system_browser",
            Self::ContinueLocalWithoutCallback => "continue_local_without_callback",
            Self::ReturnToPendingSignIn => "return_to_pending_sign_in",
            Self::ShowOriginMismatchDetail => "show_origin_mismatch_detail",
            Self::ReturnToReviewSurface => "return_to_review_surface",
            Self::RequestFreshLink => "request_fresh_link",
            Self::ShowPolicyBlockDetail => "show_policy_block_detail",
            Self::KeepLocalWorkAndDismiss => "keep_local_work_and_dismiss",
        }
    }
}

/// A distinct callback/deep-link failure class.
///
/// Each class names a materially different way a return can betray the user's
/// intent. They are never collapsed: a silent authority widen, a silent remote
/// mutation, a bypassed origin verification, a wrong-origin denial that looks
/// like an arbitrary auth failure, an expired silent no-op, an unsurfaced stale
/// state, a policy dead-end, a lost local continuity, and a raw-target leak are
/// separate findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallbackFailureMode {
    /// An auto-admit widened authority beyond a plain local open with no
    /// confirm/reject sheet.
    SilentAuthorityWiden,
    /// An auto-admit opened a mutating provider flow with no confirm/reject
    /// sheet.
    SilentRemoteMutation,
    /// An admitted return skipped origin verification (spoof-resistance
    /// bypassed).
    OriginVerificationBypassed,
    /// A wrong-origin denial offered no recovery and looked like an arbitrary
    /// auth failure.
    WrongOriginLooksLikeAuthFailure,
    /// An expired return became a silent no-op instead of offering recovery.
    ExpiredSilentNoOp,
    /// A stale or superseded pending session was not surfaced.
    StaleStateUnsurfaced,
    /// A policy denial dead-ended with no recovery or contact path.
    PolicyDenialDeadEnd,
    /// A failed return discarded the original intent / left an empty shell.
    LocalContinuityLost,
    /// The entry leaked a raw URL or token body to an end-user surface.
    RawTargetLeak,
}

impl CallbackFailureMode {
    /// Returns the stable schema token for this failure mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SilentAuthorityWiden => "silent_authority_widen",
            Self::SilentRemoteMutation => "silent_remote_mutation",
            Self::OriginVerificationBypassed => "origin_verification_bypassed",
            Self::WrongOriginLooksLikeAuthFailure => "wrong_origin_looks_like_auth_failure",
            Self::ExpiredSilentNoOp => "expired_silent_no_op",
            Self::StaleStateUnsurfaced => "stale_state_unsurfaced",
            Self::PolicyDenialDeadEnd => "policy_denial_dead_end",
            Self::LocalContinuityLost => "local_continuity_lost",
            Self::RawTargetLeak => "raw_target_leak",
        }
    }
}

/// Freshness of the captured callback-review evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallbackEvidenceFreshness {
    /// The evidence is current.
    Fresh,
    /// The evidence is stale. A blocker on a marketed entry.
    Stale,
}

impl CallbackEvidenceFreshness {
    /// Returns the stable schema token for this freshness.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
        }
    }
}

/// A desktop platform the entry is claimed on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CallbackReviewPlatform {
    /// macOS desktop platform.
    Macos,
    /// Windows desktop platform.
    Windows,
    /// Linux desktop platform.
    Linux,
}

impl CallbackReviewPlatform {
    /// Returns the stable schema token for this platform.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Macos => "macos",
            Self::Windows => "windows",
            Self::Linux => "linux",
        }
    }

    /// Returns the three platforms in canonical order.
    pub const fn all() -> [Self; 3] {
        [Self::Macos, Self::Windows, Self::Linux]
    }
}

/// How local work and intent survive a return — the continuity guarantee.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalContinuityPosture {
    /// The original intent is preserved through a truthful placeholder or
    /// recovery sheet.
    LocalIntentPreserved,
    /// Local work is intact; only managed capability is narrowed.
    LocalWorkIntactManagedNarrowed,
    /// Local continuity is at risk. A blocker.
    LocalContinuityAtRisk,
}

impl LocalContinuityPosture {
    /// Returns the stable schema token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalIntentPreserved => "local_intent_preserved",
            Self::LocalWorkIntactManagedNarrowed => "local_work_intact_managed_narrowed",
            Self::LocalContinuityAtRisk => "local_continuity_at_risk",
        }
    }

    /// Returns the reviewer-facing label for this posture.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::LocalIntentPreserved => "Local intent preserved",
            Self::LocalWorkIntactManagedNarrowed => "Local work intact, managed narrowed",
            Self::LocalContinuityAtRisk => "Local continuity at risk",
        }
    }

    /// `true` when the original intent provably survives.
    pub const fn preserves_intent(self) -> bool {
        !matches!(self, Self::LocalContinuityAtRisk)
    }
}

/// Cross-links to the canonical upstream packets the review layer extends so
/// trust vocabulary cannot drift independently.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallbackReviewCrossLinks {
    /// Browser-handoff / system-browser callback packet.
    pub browser_handoff_ref: String,
    /// Embedded-boundary surface owner / origin chrome.
    pub embedded_boundary_ref: String,
    /// Provider-origin disclosure contract.
    pub provider_origin_ref: String,
    /// Auth-and-recovery packet denied / stale returns route to.
    pub auth_recovery_ref: String,
    /// System-entry intake matrix the OS-entry layer owns.
    pub system_entry_ref: String,
    /// Entry-interstitial gate the confirm/reject paths route through.
    pub entry_interstitial_ref: String,
}

impl CallbackReviewCrossLinks {
    /// Returns the cross-link fields as `(label, ref)` pairs in canonical
    /// order.
    pub fn as_pairs(&self) -> [(&'static str, &str); 6] {
        [
            ("browser_handoff_ref", &self.browser_handoff_ref),
            ("embedded_boundary_ref", &self.embedded_boundary_ref),
            ("provider_origin_ref", &self.provider_origin_ref),
            ("auth_recovery_ref", &self.auth_recovery_ref),
            ("system_entry_ref", &self.system_entry_ref),
            ("entry_interstitial_ref", &self.entry_interstitial_ref),
        ]
    }

    /// The canonical cross-link set every report carries.
    pub fn canonical() -> Self {
        Self {
            browser_handoff_ref: "docs/auth/system_browser_callback_packet.md".to_owned(),
            embedded_boundary_ref: "shell:embedded_boundary:v1".to_owned(),
            provider_origin_ref: "docs/m5/embedded-boundaries-and-auth.md".to_owned(),
            auth_recovery_ref: "artifacts/auth/m5_auth_and_recovery.md".to_owned(),
            system_entry_ref: "artifacts/platform/m5-system-open-and-file-association.md"
                .to_owned(),
            entry_interstitial_ref: "shell:entry_interstitials:v1".to_owned(),
        }
    }
}

/// Canonical descriptor for one callback / deep-link review entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallbackReviewDescriptor {
    /// Stable entry id (e.g. `callback:auth_provider.system_browser`).
    pub entry_id: String,
    /// Entry kind the return belongs to.
    pub entry_kind: CallbackEntryKind,
    /// Source that delivered the return.
    pub source_class: CallbackSourceClass,
    /// How the origin was verified (spoof resistance).
    pub origin_assurance: OriginAssuranceClass,
    /// Descriptor revision the report was produced against. MUST be non-empty.
    pub descriptor_revision_ref: String,
    /// Canonical primary label ref.
    pub primary_label_ref: String,
    /// Export-safe ref naming the disclosed origin (who asked). MUST be
    /// non-empty. Never a raw URL or secret body.
    pub disclosed_origin_ref: String,
    /// Action the return requests.
    pub requested_action: RequestedActionClass,
    /// Export-safe ref for the target identity the return points at. MUST be
    /// non-empty.
    pub target_identity_ref: String,
    /// Workspace scope ref, when the return is workspace-scoped.
    pub workspace_scope_ref: Option<String>,
    /// Tenant scope ref, when the return is tenant-scoped.
    pub tenant_scope_ref: Option<String>,
    /// Authority the return would reach once it commits.
    pub authority_scope: AuthorityScopeClass,
    /// `true` when the authority is broader than a plain local open.
    pub widens_authority: bool,
    /// `true` when the commit must be gated behind a confirm/reject sheet.
    pub requires_confirm_reject: bool,
    /// Confirm/reject sheet ref (required when
    /// [`Self::requires_confirm_reject`] is `true`).
    pub confirm_reject_sheet_ref: Option<String>,
    /// Export-safe alias for the pending state / nonce / PKCE correlation. MUST
    /// be non-empty. Never a raw token.
    pub pending_correlation_ref: String,
    /// Timestamp the return expires. MUST be non-empty.
    pub expiry_at: String,
    /// Active profile owner the return routes through. MUST be non-empty.
    pub active_profile_owner_ref: String,
    /// Trust / profile / tenant / policy checkpoint the return routes through.
    /// MUST be non-empty.
    pub trust_checkpoint_ref: String,
    /// Canonical in-product command the confirm action routes to (in-product
    /// parity). MUST be non-empty.
    pub canonical_command_ref: String,
    /// Disposition of the return at review time.
    pub outcome: CallbackOutcomeClass,
    /// Recovery actions offered when the return is denied.
    pub recovery_actions: Vec<CallbackRecoveryAction>,
    /// How local work and intent survive the return.
    pub local_continuity: LocalContinuityPosture,
    /// Continuity note retained on the descriptor. MUST be non-empty.
    pub continuity_note: String,
    /// Exact degraded-state vocabulary user-visible surfaces MUST use. MUST be
    /// non-empty.
    pub degraded_state_vocabulary: Vec<String>,
    /// Claimed platforms. MUST be non-empty.
    pub claimed_platforms: Vec<CallbackReviewPlatform>,
    /// Freshness of the captured evidence.
    pub evidence_freshness: CallbackEvidenceFreshness,
    /// Timestamp the evidence was captured.
    pub evidence_captured_at: String,
    /// Rule user-visible surfaces follow when evidence goes stale. MUST be
    /// non-empty.
    pub downgrade_rule_ref: String,
    /// `true` once the entry carries only redaction-safe refs (no raw URL or
    /// token body). MUST be `true`.
    pub redaction_safe: bool,
    /// `true` when the entry is marketed and must pass the report or narrow.
    pub marketed: bool,
    /// `true` once the entry rides the governed callback-review harness. MUST be
    /// `true`.
    pub registered_on_callback_review_harness: bool,
}

/// Outcome of evaluating an entry's confirm/reject discipline against the
/// in-product authority path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfirmRejectOutcome {
    /// Authority scope the entry declared.
    pub authority_scope: AuthorityScopeClass,
    /// `true` when a confirm/reject sheet is required for this scope.
    pub confirm_reject_required: bool,
    /// `true` when the required confirm/reject gate is fully enforced (a sheet
    /// is named) or correctly absent for a plain local open.
    pub confirm_reject_enforced: bool,
    /// `true` when the confirm action routes to a canonical in-product command.
    pub routes_to_canonical_command: bool,
    /// `true` when the requested action is consistent with the authority scope.
    pub action_scope_consistent: bool,
    /// `true` when the entry provably reuses the in-product authority path:
    /// the gate is enforced, the command is canonical, and the action and scope
    /// agree.
    pub reuses_in_product_authority_path: bool,
    /// Stable note explaining a divergence, when not matched.
    pub divergence_note: Option<String>,
}

/// Blocking finding class the validator emits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum CallbackReviewBlockingFinding {
    /// An auto-admit widened authority beyond a plain local open with no
    /// confirm/reject sheet.
    SilentAuthorityWiden {
        /// Entry that exposes the gap.
        entry_id: String,
        /// Authority scope the entry would have reached.
        authority_scope: AuthorityScopeClass,
    },
    /// An auto-admit opened a mutating provider flow with no confirm/reject
    /// sheet.
    SilentRemoteMutation {
        /// Entry that exposes the gap.
        entry_id: String,
    },
    /// An admitted return skipped origin verification.
    OriginVerificationBypassed {
        /// Entry that exposes the gap.
        entry_id: String,
    },
    /// A wrong-origin denial offered no recovery.
    WrongOriginLooksLikeAuthFailure {
        /// Entry that exposes the gap.
        entry_id: String,
        /// Outcome that required recovery.
        outcome: CallbackOutcomeClass,
    },
    /// An expired return became a silent no-op.
    ExpiredSilentNoOp {
        /// Entry that exposes the gap.
        entry_id: String,
    },
    /// A stale or superseded pending session was not surfaced.
    StaleStateUnsurfaced {
        /// Entry that exposes the gap.
        entry_id: String,
    },
    /// A policy denial dead-ended with no recovery.
    PolicyDenialDeadEnd {
        /// Entry that exposes the gap.
        entry_id: String,
    },
    /// A failed return put local intent at risk.
    LocalContinuityLost {
        /// Entry that exposes the gap.
        entry_id: String,
    },
    /// The entry leaked a raw URL or token body.
    RawTargetLeak {
        /// Entry that exposes the gap.
        entry_id: String,
    },
    /// The entry disclosed no origin.
    MissingDisclosedOrigin {
        /// Entry that exposes the gap.
        entry_id: String,
    },
    /// The entry named no target identity.
    MissingTargetIdentity {
        /// Entry that exposes the gap.
        entry_id: String,
    },
    /// The entry carried no pending correlation alias.
    MissingPendingCorrelation {
        /// Entry that exposes the gap.
        entry_id: String,
    },
    /// The entry carried no expiry.
    MissingExpiry {
        /// Entry that exposes the gap.
        entry_id: String,
    },
    /// The entry carried no active-profile owner.
    MissingActiveProfileOwner {
        /// Entry that exposes the gap.
        entry_id: String,
    },
    /// The return bypassed trust / policy evaluation (no trust checkpoint).
    TrustEvaluationBypassed {
        /// Entry that exposes the gap.
        entry_id: String,
    },
    /// The entry reused no canonical in-product command.
    MissingCanonicalCommand {
        /// Entry that exposes the gap.
        entry_id: String,
    },
    /// A scope that requires a confirm/reject sheet named none.
    MissingConfirmRejectSheet {
        /// Entry that exposes the gap.
        entry_id: String,
    },
    /// The entry carried no continuity note.
    MissingContinuityNote {
        /// Entry that exposes the gap.
        entry_id: String,
    },
    /// The entry carried no degraded-state vocabulary.
    MissingDegradedStateVocabulary {
        /// Entry that exposes the gap.
        entry_id: String,
    },
    /// The entry claimed no platform.
    MissingClaimedPlatforms {
        /// Entry that exposes the gap.
        entry_id: String,
    },
    /// The entry carried no downgrade rule.
    MissingDowngradeRule {
        /// Entry that exposes the gap.
        entry_id: String,
    },
    /// A marketed entry carries stale evidence.
    StaleEvidenceOnMarketedEntry {
        /// Entry that exposes the gap.
        entry_id: String,
    },
    /// The entry drives its own return path off the governed harness.
    EntryNotOnHarness {
        /// Entry that exposes the gap.
        entry_id: String,
    },
}

impl CallbackReviewBlockingFinding {
    /// Returns the stable schema token for the finding class.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::SilentAuthorityWiden { .. } => "silent_authority_widen",
            Self::SilentRemoteMutation { .. } => "silent_remote_mutation",
            Self::OriginVerificationBypassed { .. } => "origin_verification_bypassed",
            Self::WrongOriginLooksLikeAuthFailure { .. } => "wrong_origin_looks_like_auth_failure",
            Self::ExpiredSilentNoOp { .. } => "expired_silent_no_op",
            Self::StaleStateUnsurfaced { .. } => "stale_state_unsurfaced",
            Self::PolicyDenialDeadEnd { .. } => "policy_denial_dead_end",
            Self::LocalContinuityLost { .. } => "local_continuity_lost",
            Self::RawTargetLeak { .. } => "raw_target_leak",
            Self::MissingDisclosedOrigin { .. } => "missing_disclosed_origin",
            Self::MissingTargetIdentity { .. } => "missing_target_identity",
            Self::MissingPendingCorrelation { .. } => "missing_pending_correlation",
            Self::MissingExpiry { .. } => "missing_expiry",
            Self::MissingActiveProfileOwner { .. } => "missing_active_profile_owner",
            Self::TrustEvaluationBypassed { .. } => "trust_evaluation_bypassed",
            Self::MissingCanonicalCommand { .. } => "missing_canonical_command",
            Self::MissingConfirmRejectSheet { .. } => "missing_confirm_reject_sheet",
            Self::MissingContinuityNote { .. } => "missing_continuity_note",
            Self::MissingDegradedStateVocabulary { .. } => "missing_degraded_state_vocabulary",
            Self::MissingClaimedPlatforms { .. } => "missing_claimed_platforms",
            Self::MissingDowngradeRule { .. } => "missing_downgrade_rule",
            Self::StaleEvidenceOnMarketedEntry { .. } => "stale_evidence_on_marketed_entry",
            Self::EntryNotOnHarness { .. } => "entry_not_on_harness",
        }
    }

    /// Returns the entry id this finding is attached to.
    pub fn entry_id(&self) -> &str {
        match self {
            Self::SilentAuthorityWiden { entry_id, .. }
            | Self::SilentRemoteMutation { entry_id }
            | Self::OriginVerificationBypassed { entry_id }
            | Self::WrongOriginLooksLikeAuthFailure { entry_id, .. }
            | Self::ExpiredSilentNoOp { entry_id }
            | Self::StaleStateUnsurfaced { entry_id }
            | Self::PolicyDenialDeadEnd { entry_id }
            | Self::LocalContinuityLost { entry_id }
            | Self::RawTargetLeak { entry_id }
            | Self::MissingDisclosedOrigin { entry_id }
            | Self::MissingTargetIdentity { entry_id }
            | Self::MissingPendingCorrelation { entry_id }
            | Self::MissingExpiry { entry_id }
            | Self::MissingActiveProfileOwner { entry_id }
            | Self::TrustEvaluationBypassed { entry_id }
            | Self::MissingCanonicalCommand { entry_id }
            | Self::MissingConfirmRejectSheet { entry_id }
            | Self::MissingContinuityNote { entry_id }
            | Self::MissingDegradedStateVocabulary { entry_id }
            | Self::MissingClaimedPlatforms { entry_id }
            | Self::MissingDowngradeRule { entry_id }
            | Self::StaleEvidenceOnMarketedEntry { entry_id }
            | Self::EntryNotOnHarness { entry_id } => entry_id,
        }
    }

    /// Returns the distinct failure mode this finding maps to, when it maps to a
    /// contract-honesty failure class (rather than a missing-field gap).
    pub fn failure_mode(&self) -> Option<CallbackFailureMode> {
        match self {
            Self::SilentAuthorityWiden { .. } => Some(CallbackFailureMode::SilentAuthorityWiden),
            Self::SilentRemoteMutation { .. } => Some(CallbackFailureMode::SilentRemoteMutation),
            Self::OriginVerificationBypassed { .. } => {
                Some(CallbackFailureMode::OriginVerificationBypassed)
            }
            Self::WrongOriginLooksLikeAuthFailure { .. } => {
                Some(CallbackFailureMode::WrongOriginLooksLikeAuthFailure)
            }
            Self::ExpiredSilentNoOp { .. } => Some(CallbackFailureMode::ExpiredSilentNoOp),
            Self::StaleStateUnsurfaced { .. } => Some(CallbackFailureMode::StaleStateUnsurfaced),
            Self::PolicyDenialDeadEnd { .. } => Some(CallbackFailureMode::PolicyDenialDeadEnd),
            Self::LocalContinuityLost { .. } => Some(CallbackFailureMode::LocalContinuityLost),
            Self::RawTargetLeak { .. } => Some(CallbackFailureMode::RawTargetLeak),
            _ => None,
        }
    }
}

/// One per-entry callback-review row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallbackReviewRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Canonical descriptor for the entry.
    pub descriptor: CallbackReviewDescriptor,
    /// Confirm/reject outcome computed against the in-product authority path.
    pub confirm_reject_outcome: ConfirmRejectOutcome,
    /// Blocking findings emitted against this row.
    pub blocking_findings: Vec<CallbackReviewBlockingFinding>,
    /// `true` when the entry is marketed.
    pub marketed: bool,
}

/// One `(class, count)` blocking-finding tally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallbackReviewFindingCount {
    /// Finding class token.
    pub class: String,
    /// Number of findings in this class.
    pub count: usize,
}

/// Per-class blocking-finding summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallbackReviewFindingSummary {
    /// Total blocking findings across the report.
    pub total_blocking_findings: usize,
    /// Per-class tallies, sorted by class token.
    pub by_class: Vec<CallbackReviewFindingCount>,
}

/// Per-entry-kind presence summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallbackReviewKindCoverage {
    /// Entry kind this summary covers.
    pub entry_kind: CallbackEntryKind,
    /// Number of registered entries of this kind.
    pub entry_count: usize,
}

/// Per-authority-scope coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallbackReviewScopeCoverage {
    /// Authority scope this summary covers.
    pub authority_scope: AuthorityScopeClass,
    /// Number of entries that reach this scope.
    pub entry_count: usize,
    /// Number of those entries gated behind a confirm/reject sheet.
    pub gated_behind_confirm_reject: usize,
}

/// A single disposition-index entry so platform QA, docs, and support surfaces
/// can quote who asked, what scope, and the review outcome for each return.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallbackReviewDispositionEntry {
    /// Entry id the disposition covers.
    pub entry_id: String,
    /// Entry kind the disposition covers.
    pub entry_kind: CallbackEntryKind,
    /// Disclosed origin ref (who asked).
    pub disclosed_origin_ref: String,
    /// Requested action (what they want).
    pub requested_action: RequestedActionClass,
    /// Authority scope the return would reach.
    pub authority_scope: AuthorityScopeClass,
    /// Disposition at review time.
    pub outcome: CallbackOutcomeClass,
    /// `true` when the return is gated behind a confirm/reject sheet.
    pub requires_confirm_reject: bool,
}

/// One marketed entry release tooling should narrow because a control failed or
/// its evidence is stale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallbackReviewNarrowableEntry {
    /// Entry id that must narrow.
    pub entry_id: String,
    /// Failure mode that drives the narrowing, when control-scoped.
    pub failure_mode: Option<CallbackFailureMode>,
    /// Stable reason the entry is narrowable.
    pub reason: String,
}

/// Auth-callback and deep-link review report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallbackReviewReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable report id quoted across surfaces.
    pub report_id: String,
    /// Source schema ref for the canonical contract.
    pub source_schema_ref: String,
    /// Required entry kinds, in canonical order.
    pub required_entry_kinds: Vec<CallbackEntryKind>,
    /// Union of claimed platforms across all entries, sorted.
    pub claimed_platforms: Vec<CallbackReviewPlatform>,
    /// Cross-links to upstream packets.
    pub cross_links: CallbackReviewCrossLinks,
    /// Per-entry rows, sorted by `descriptor.entry_id`.
    pub entries: Vec<CallbackReviewRow>,
    /// Per-entry-kind presence summary, in canonical kind order.
    pub entry_kind_coverage: Vec<CallbackReviewKindCoverage>,
    /// Per-authority-scope coverage summary, in canonical scope order.
    pub authority_scope_coverage: Vec<CallbackReviewScopeCoverage>,
    /// Per-class blocking-finding summary.
    pub findings_summary: CallbackReviewFindingSummary,
    /// Canonical disposition index, sorted by entry id.
    pub disposition_index: Vec<CallbackReviewDispositionEntry>,
    /// Number of registered entries present.
    pub registered_entry_count: usize,
    /// Number of entries marketed.
    pub marketed_entry_count: usize,
    /// Number of entries that provably reuse the in-product authority path.
    pub confirm_reject_parity_count: usize,
    /// Marketed entries release tooling should narrow.
    pub narrowable_marketed_entries: Vec<CallbackReviewNarrowableEntry>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Markdown publication ref this report is rendered to.
    pub published_report_ref: String,
    /// Companion doc publication ref.
    pub published_doc_ref: String,
    /// Docs/help refs the report can be reopened from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the report can be reopened from.
    pub support_export_refs: Vec<String>,
    /// Timestamp captured when the report was generated.
    pub generated_at: String,
}

impl CallbackReviewReport {
    /// Returns `true` when every required entry kind has at least one
    /// registered entry.
    pub fn every_kind_present(&self) -> bool {
        CallbackEntryKind::required_kinds().into_iter().all(|kind| {
            self.entries
                .iter()
                .any(|entry| entry.descriptor.entry_kind == kind)
        })
    }

    /// Returns `true` when at least one entry provably reuses the in-product
    /// authority path.
    pub fn has_confirm_reject_parity(&self) -> bool {
        self.entries.iter().any(|entry| {
            entry
                .confirm_reject_outcome
                .reuses_in_product_authority_path
        })
    }

    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "report: entries={}, marketed={}, parity={}, blocking={}, clean={}",
            self.registered_entry_count,
            self.marketed_entry_count,
            self.confirm_reject_parity_count,
            self.findings_summary.total_blocking_findings,
            self.report_clean,
        ));
        for entry in &self.entries {
            lines.push(format!(
                "{}: kind={}, action={}, scope={}, origin={}, outcome={}, gated={}",
                entry.descriptor.entry_id,
                entry.descriptor.entry_kind.as_str(),
                entry.descriptor.requested_action.as_str(),
                entry.descriptor.authority_scope.as_str(),
                entry.descriptor.origin_assurance.as_str(),
                entry.descriptor.outcome.as_str(),
                entry.descriptor.requires_confirm_reject,
            ));
        }
        for entry in &self.entries {
            for finding in &entry.blocking_findings {
                lines.push(format!(
                    "blocker: {} -- {}",
                    finding.class_token(),
                    finding.entry_id(),
                ));
            }
        }
        for narrowable in &self.narrowable_marketed_entries {
            lines.push(format!(
                "narrowable: {} -- {}",
                narrowable.entry_id, narrowable.reason,
            ));
        }
        lines
    }

    /// Renders the markdown artifact.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 auth-callback and deep-link review\n\n");
        out.push_str(
            "Generated from the seeded report in\n\
             [`crate::m5_callback_and_deep_link_review`](../../crates/aureline-auth/src/m5_callback_and_deep_link_review/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-auth --bin aureline_auth_m5_callback_and_deep_link_review -- report-md > \\\n  artifacts/platform/m5-auth-callback-and-deep-link.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Claimed platforms: {}\n",
            self.claimed_platforms
                .iter()
                .map(|platform| format!("`{}`", platform.as_str()))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Registered entries: `{}`\n",
            self.registered_entry_count
        ));
        out.push_str(&format!(
            "- Marketed entries: `{}`\n",
            self.marketed_entry_count
        ));
        out.push_str(&format!(
            "- Confirm/reject parity entries: `{}`\n",
            self.confirm_reject_parity_count
        ));
        out.push_str(&format!(
            "- Blocking findings: `{}`\n",
            self.findings_summary.total_blocking_findings
        ));
        out.push_str(&format!(
            "- Narrowable marketed entries: `{}`\n",
            self.narrowable_marketed_entries.len()
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Cross-links\n\n");
        out.push_str("| Upstream packet | Ref |\n| --------------- | --- |\n");
        for (label, value) in self.cross_links.as_pairs() {
            out.push_str(&format!("| `{label}` | `{value}` |\n"));
        }
        out.push('\n');

        out.push_str("## Per-entry-kind coverage\n\n");
        out.push_str("| Entry kind | Registered entries |\n| ---------- | -----------------: |\n");
        for coverage in &self.entry_kind_coverage {
            out.push_str(&format!(
                "| {} | {} |\n",
                coverage.entry_kind.display_label(),
                coverage.entry_count,
            ));
        }
        out.push('\n');

        out.push_str("## Per-authority-scope coverage\n\n");
        out.push_str(
            "| Authority scope | Entries | Gated behind confirm/reject |\n\
             | --------------- | ------: | --------------------------: |\n",
        );
        for coverage in &self.authority_scope_coverage {
            out.push_str(&format!(
                "| {} | {} | {} |\n",
                coverage.authority_scope.display_label(),
                coverage.entry_count,
                coverage.gated_behind_confirm_reject,
            ));
        }
        out.push('\n');

        out.push_str("## Disposition index\n\n");
        out.push_str(
            "| Entry | Kind | Action | Authority scope | Outcome | Confirm/reject |\n\
             | ----- | ---- | ------ | --------------- | ------- | -------------- |\n",
        );
        for entry in &self.disposition_index {
            out.push_str(&format!(
                "| `{}` | {} | `{}` | `{}` | `{}` | `{}` |\n",
                entry.entry_id,
                entry.entry_kind.display_label(),
                entry.requested_action.as_str(),
                entry.authority_scope.as_str(),
                entry.outcome.as_str(),
                entry.requires_confirm_reject,
            ));
        }
        out.push('\n');

        out.push_str("## Findings summary\n\n");
        out.push_str("| Class | Count |\n| ----- | ----: |\n");
        for tally in &self.findings_summary.by_class {
            out.push_str(&format!("| `{}` | {} |\n", tally.class, tally.count));
        }
        if self.findings_summary.by_class.is_empty() {
            out.push_str("| _(none)_ | 0 |\n");
        }
        out.push('\n');

        out.push_str("## Per-entry rows\n\n");
        for entry in &self.entries {
            let d = &entry.descriptor;
            out.push_str(&format!(
                "### `{}` ({} via {})\n\n",
                d.entry_id,
                d.entry_kind.as_str(),
                d.source_class.as_str(),
            ));
            out.push_str(&format!(
                "- Descriptor revision: `{}`\n",
                d.descriptor_revision_ref
            ));
            out.push_str(&format!(
                "- Disclosed origin: `{}` (`{}`)\n",
                d.disclosed_origin_ref,
                d.source_class.as_str(),
            ));
            out.push_str(&format!(
                "- Origin assurance: `{}`\n",
                d.origin_assurance.as_str()
            ));
            out.push_str(&format!("- Target identity: `{}`\n", d.target_identity_ref));
            out.push_str(&format!(
                "- Requested action: `{}`\n",
                d.requested_action.as_str()
            ));
            out.push_str(&format!(
                "- Authority scope: `{}` (widens authority: `{}`)\n",
                d.authority_scope.as_str(),
                d.widens_authority,
            ));
            out.push_str(&format!(
                "- Confirm/reject required: `{}` (reuses in-product path: `{}`)\n",
                d.requires_confirm_reject,
                entry
                    .confirm_reject_outcome
                    .reuses_in_product_authority_path,
            ));
            out.push_str(&format!(
                "- Canonical command: `{}`\n",
                d.canonical_command_ref
            ));
            out.push_str(&format!(
                "- Active profile owner: `{}`\n",
                d.active_profile_owner_ref
            ));
            out.push_str(&format!(
                "- Trust checkpoint: `{}`\n",
                d.trust_checkpoint_ref
            ));
            out.push_str(&format!(
                "- Pending correlation: `{}`\n",
                d.pending_correlation_ref
            ));
            out.push_str(&format!("- Expiry: `{}`\n", d.expiry_at));
            out.push_str(&format!("- Outcome: `{}`\n", d.outcome.as_str()));
            if d.recovery_actions.is_empty() {
                out.push_str("- Recovery actions: _(none required)_\n");
            } else {
                out.push_str(&format!(
                    "- Recovery actions: {}\n",
                    d.recovery_actions
                        .iter()
                        .map(|action| format!("`{}`", action.as_str()))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            out.push_str(&format!(
                "- Local continuity: `{}`\n",
                d.local_continuity.as_str()
            ));
            out.push_str(&format!(
                "- Claimed platforms: {}\n",
                d.claimed_platforms
                    .iter()
                    .map(|platform| format!("`{}`", platform.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "- Evidence freshness: `{}` (captured `{}`)\n",
                d.evidence_freshness.as_str(),
                d.evidence_captured_at,
            ));
            out.push_str(&format!("- Downgrade rule: `{}`\n", d.downgrade_rule_ref));
            out.push_str(&format!(
                "- Redaction-safe: `{}`\n",
                if d.redaction_safe { "yes" } else { "no" }
            ));
            out.push_str(&format!(
                "- Marketed: `{}`\n",
                if entry.marketed { "yes" } else { "no" }
            ));
            out.push_str(&format!("- Continuity note: {}\n", d.continuity_note));
            out.push_str("- Degraded-state vocabulary:\n");
            for phrase in &d.degraded_state_vocabulary {
                out.push_str(&format!("  - {phrase}\n"));
            }
            out.push('\n');

            if entry.blocking_findings.is_empty() {
                out.push_str("Findings: none.\n\n");
            } else {
                out.push_str("Findings:\n\n");
                for finding in &entry.blocking_findings {
                    out.push_str(&format!("- `{}`\n", finding.class_token()));
                }
                out.push('\n');
            }
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-auth --bin aureline_auth_m5_callback_and_deep_link_review -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-auth --test m5_callback_and_deep_link_review_fixtures\n",
        );
        out.push_str("python3 tools/ci/m5/callback_and_deep_link_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the full callback-review report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallbackReviewSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Report quoted in full.
    pub report: CallbackReviewReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl CallbackReviewSupportExport {
    /// Builds the support-export wrapper for a report.
    pub fn from_report(support_export_id: impl Into<String>, report: CallbackReviewReport) -> Self {
        let mut case_ids = vec![report.report_id.clone()];
        for entry in &report.entries {
            case_ids.push(entry.descriptor.entry_id.clone());
            case_ids.push(entry.descriptor.descriptor_revision_ref.clone());
        }
        Self {
            record_kind: CALLBACK_REVIEW_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: CALLBACK_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: CALLBACK_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Per-incident support-export packet for a single denied return.
///
/// This is the export a reviewer reproduces a denied, wrong-origin, expired, or
/// stale callback from — the typed diagnostic that replaces a screenshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallbackReviewCaseExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable case-export id.
    pub case_export_id: String,
    /// Stable case label (e.g. `wrong_origin`).
    pub case_label: String,
    /// Outcome that defines the incident class.
    pub outcome: CallbackOutcomeClass,
    /// The entry row in full.
    pub entry: CallbackReviewRow,
    /// Recovery actions the incident offers.
    pub recovery_actions: Vec<CallbackRecoveryAction>,
    /// Stable reproduction note for support.
    pub reproduction_note: String,
}

impl CallbackReviewCaseExport {
    /// Builds a per-incident case export from a denied entry row.
    pub fn from_row(
        case_export_id: impl Into<String>,
        case_label: impl Into<String>,
        reproduction_note: impl Into<String>,
        row: CallbackReviewRow,
    ) -> Self {
        let outcome = row.descriptor.outcome;
        let recovery_actions = row.descriptor.recovery_actions.clone();
        Self {
            record_kind: CALLBACK_REVIEW_CASE_EXPORT_RECORD_KIND.to_owned(),
            schema_version: CALLBACK_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: CALLBACK_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            case_export_id: case_export_id.into(),
            case_label: case_label.into(),
            outcome,
            entry: row,
            recovery_actions,
            reproduction_note: reproduction_note.into(),
        }
    }
}

/// Evaluates an entry's confirm/reject discipline against the in-product
/// authority path.
fn compute_confirm_reject_outcome(descriptor: &CallbackReviewDescriptor) -> ConfirmRejectOutcome {
    let confirm_reject_required = descriptor.authority_scope.requires_confirm_reject();
    let sheet_present = descriptor
        .confirm_reject_sheet_ref
        .as_deref()
        .map(str::trim)
        .map(str::is_empty)
        == Some(false);
    let confirm_reject_enforced = if confirm_reject_required {
        descriptor.requires_confirm_reject && sheet_present
    } else {
        true
    };
    let routes_to_canonical_command = !descriptor.canonical_command_ref.trim().is_empty();
    let action_scope_consistent = descriptor
        .requested_action
        .is_consistent_with(descriptor.authority_scope);
    let reuses_in_product_authority_path =
        confirm_reject_enforced && routes_to_canonical_command && action_scope_consistent;

    let divergence_note = if reuses_in_product_authority_path {
        None
    } else if !action_scope_consistent {
        Some("requested action and authority scope disagree".to_owned())
    } else if !confirm_reject_enforced {
        Some("a widening authority is not gated behind a confirm/reject sheet".to_owned())
    } else {
        Some("the confirm action does not route to a canonical in-product command".to_owned())
    };

    ConfirmRejectOutcome {
        authority_scope: descriptor.authority_scope,
        confirm_reject_required,
        confirm_reject_enforced,
        routes_to_canonical_command,
        action_scope_consistent,
        reuses_in_product_authority_path,
        divergence_note,
    }
}

/// Computes the per-entry blocking findings from a descriptor.
fn compute_entry_findings(
    descriptor: &CallbackReviewDescriptor,
) -> Vec<CallbackReviewBlockingFinding> {
    let mut findings = Vec::new();
    let entry_id = descriptor.entry_id.clone();

    // Disclosure and identity integrity.
    if descriptor.disclosed_origin_ref.trim().is_empty() {
        findings.push(CallbackReviewBlockingFinding::MissingDisclosedOrigin {
            entry_id: entry_id.clone(),
        });
    }
    if descriptor.target_identity_ref.trim().is_empty() {
        findings.push(CallbackReviewBlockingFinding::MissingTargetIdentity {
            entry_id: entry_id.clone(),
        });
    }
    if descriptor.pending_correlation_ref.trim().is_empty() {
        findings.push(CallbackReviewBlockingFinding::MissingPendingCorrelation {
            entry_id: entry_id.clone(),
        });
    }
    if descriptor.expiry_at.trim().is_empty() {
        findings.push(CallbackReviewBlockingFinding::MissingExpiry {
            entry_id: entry_id.clone(),
        });
    }
    if descriptor.active_profile_owner_ref.trim().is_empty() {
        findings.push(CallbackReviewBlockingFinding::MissingActiveProfileOwner {
            entry_id: entry_id.clone(),
        });
    }
    if descriptor.trust_checkpoint_ref.trim().is_empty() {
        findings.push(CallbackReviewBlockingFinding::TrustEvaluationBypassed {
            entry_id: entry_id.clone(),
        });
    }
    if descriptor.canonical_command_ref.trim().is_empty() {
        findings.push(CallbackReviewBlockingFinding::MissingCanonicalCommand {
            entry_id: entry_id.clone(),
        });
    }
    if descriptor.continuity_note.trim().is_empty() {
        findings.push(CallbackReviewBlockingFinding::MissingContinuityNote {
            entry_id: entry_id.clone(),
        });
    }
    if descriptor
        .degraded_state_vocabulary
        .iter()
        .all(|phrase| phrase.trim().is_empty())
    {
        findings.push(
            CallbackReviewBlockingFinding::MissingDegradedStateVocabulary {
                entry_id: entry_id.clone(),
            },
        );
    }
    if descriptor.claimed_platforms.is_empty() {
        findings.push(CallbackReviewBlockingFinding::MissingClaimedPlatforms {
            entry_id: entry_id.clone(),
        });
    }
    if descriptor.downgrade_rule_ref.trim().is_empty() {
        findings.push(CallbackReviewBlockingFinding::MissingDowngradeRule {
            entry_id: entry_id.clone(),
        });
    }
    if !descriptor.registered_on_callback_review_harness {
        findings.push(CallbackReviewBlockingFinding::EntryNotOnHarness {
            entry_id: entry_id.clone(),
        });
    }
    if descriptor.marketed && descriptor.evidence_freshness == CallbackEvidenceFreshness::Stale {
        findings.push(
            CallbackReviewBlockingFinding::StaleEvidenceOnMarketedEntry {
                entry_id: entry_id.clone(),
            },
        );
    }
    if !descriptor.redaction_safe {
        findings.push(CallbackReviewBlockingFinding::RawTargetLeak {
            entry_id: entry_id.clone(),
        });
    }

    // Spoof resistance: an admitted return must have a verified origin.
    if descriptor.outcome == CallbackOutcomeClass::Admitted
        && !descriptor.origin_assurance.is_verified()
    {
        findings.push(CallbackReviewBlockingFinding::OriginVerificationBypassed {
            entry_id: entry_id.clone(),
        });
    }

    // Confirm/reject discipline: anything wider than a plain local open must be
    // gated. A silent authority widen and a silent remote mutation stay
    // distinct failure classes.
    if descriptor.authority_scope.requires_confirm_reject() {
        if !descriptor.requires_confirm_reject {
            if descriptor.authority_scope == AuthorityScopeClass::WidensToProviderMutation {
                findings.push(CallbackReviewBlockingFinding::SilentRemoteMutation {
                    entry_id: entry_id.clone(),
                });
            } else {
                findings.push(CallbackReviewBlockingFinding::SilentAuthorityWiden {
                    entry_id: entry_id.clone(),
                    authority_scope: descriptor.authority_scope,
                });
            }
        } else if descriptor
            .confirm_reject_sheet_ref
            .as_deref()
            .map(str::trim)
            .map(str::is_empty)
            != Some(false)
        {
            findings.push(CallbackReviewBlockingFinding::MissingConfirmRejectSheet {
                entry_id: entry_id.clone(),
            });
        }
    }

    // Local continuity: a failed return must preserve the original intent.
    if descriptor.local_continuity == LocalContinuityPosture::LocalContinuityAtRisk {
        findings.push(CallbackReviewBlockingFinding::LocalContinuityLost {
            entry_id: entry_id.clone(),
        });
    }

    // Recovery: a denial must offer a recovery action, and each denial class
    // stays a distinct failure.
    if descriptor.outcome.requires_recovery() && descriptor.recovery_actions.is_empty() {
        if let Some(mode) = descriptor.outcome.missing_recovery_failure_mode() {
            let finding = match mode {
                CallbackFailureMode::WrongOriginLooksLikeAuthFailure => {
                    CallbackReviewBlockingFinding::WrongOriginLooksLikeAuthFailure {
                        entry_id: entry_id.clone(),
                        outcome: descriptor.outcome,
                    }
                }
                CallbackFailureMode::ExpiredSilentNoOp => {
                    CallbackReviewBlockingFinding::ExpiredSilentNoOp {
                        entry_id: entry_id.clone(),
                    }
                }
                CallbackFailureMode::StaleStateUnsurfaced => {
                    CallbackReviewBlockingFinding::StaleStateUnsurfaced {
                        entry_id: entry_id.clone(),
                    }
                }
                _ => CallbackReviewBlockingFinding::PolicyDenialDeadEnd {
                    entry_id: entry_id.clone(),
                },
            };
            findings.push(finding);
        }
    }

    findings
}

/// Builds a [`CallbackReviewRow`] from a descriptor, computing the confirm/
/// reject outcome and per-entry blocking findings.
pub fn build_callback_review_row(descriptor: CallbackReviewDescriptor) -> CallbackReviewRow {
    let marketed = descriptor.marketed;
    let confirm_reject_outcome = compute_confirm_reject_outcome(&descriptor);
    let blocking_findings = compute_entry_findings(&descriptor);

    CallbackReviewRow {
        record_kind: CALLBACK_REVIEW_ROW_RECORD_KIND.to_owned(),
        schema_version: CALLBACK_REVIEW_SCHEMA_VERSION,
        shared_contract_ref: CALLBACK_REVIEW_SHARED_CONTRACT_REF.to_owned(),
        descriptor,
        confirm_reject_outcome,
        blocking_findings,
        marketed,
    }
}

/// Canonical authority-scope order for coverage summaries.
const SCOPE_ORDER: [AuthorityScopeClass; 5] = [
    AuthorityScopeClass::PlainLocalOpen,
    AuthorityScopeClass::CrossesBoundaryReadOnly,
    AuthorityScopeClass::WorkspaceCollaborationJoin,
    AuthorityScopeClass::WidensToManagedAuthority,
    AuthorityScopeClass::WidensToProviderMutation,
];

/// Computes the per-kind, per-scope, and per-class summaries from finished rows.
fn summarize_report(
    entries: &[CallbackReviewRow],
) -> (
    Vec<CallbackReviewKindCoverage>,
    Vec<CallbackReviewScopeCoverage>,
    CallbackReviewFindingSummary,
) {
    let mut kind_coverage: Vec<CallbackReviewKindCoverage> = CallbackEntryKind::required_kinds()
        .into_iter()
        .map(|entry_kind| CallbackReviewKindCoverage {
            entry_kind,
            entry_count: 0,
        })
        .collect();

    let mut scope_coverage: Vec<CallbackReviewScopeCoverage> = SCOPE_ORDER
        .into_iter()
        .map(|authority_scope| CallbackReviewScopeCoverage {
            authority_scope,
            entry_count: 0,
            gated_behind_confirm_reject: 0,
        })
        .collect();

    let mut class_counts: Vec<CallbackReviewFindingCount> = Vec::new();
    let mut total = 0usize;

    for entry in entries {
        if let Some(kind_row) = kind_coverage
            .iter_mut()
            .find(|row| row.entry_kind == entry.descriptor.entry_kind)
        {
            kind_row.entry_count += 1;
        }
        if let Some(scope_row) = scope_coverage
            .iter_mut()
            .find(|row| row.authority_scope == entry.descriptor.authority_scope)
        {
            scope_row.entry_count += 1;
            if entry.descriptor.requires_confirm_reject {
                scope_row.gated_behind_confirm_reject += 1;
            }
        }
        for finding in &entry.blocking_findings {
            total += 1;
            let class = finding.class_token();
            if let Some(tally) = class_counts.iter_mut().find(|tally| tally.class == class) {
                tally.count += 1;
            } else {
                class_counts.push(CallbackReviewFindingCount {
                    class: class.to_owned(),
                    count: 1,
                });
            }
        }
    }

    class_counts.sort_by(|left, right| left.class.cmp(&right.class));
    (
        kind_coverage,
        scope_coverage,
        CallbackReviewFindingSummary {
            total_blocking_findings: total,
            by_class: class_counts,
        },
    )
}

/// Computes the marketed entries release tooling should narrow because a
/// control failed or their evidence is stale.
fn compute_narrowable_entries(entries: &[CallbackReviewRow]) -> Vec<CallbackReviewNarrowableEntry> {
    let mut narrowable = Vec::new();
    for entry in entries {
        if !entry.marketed {
            continue;
        }
        for finding in &entry.blocking_findings {
            narrowable.push(CallbackReviewNarrowableEntry {
                entry_id: entry.descriptor.entry_id.clone(),
                failure_mode: finding.failure_mode(),
                reason: format!("blocking_finding:{}", finding.class_token()),
            });
        }
    }
    narrowable
}

/// Builds a full [`CallbackReviewReport`] from per-entry rows.
pub fn build_callback_review_report(entries: Vec<CallbackReviewRow>) -> CallbackReviewReport {
    let mut entries = entries;
    entries.sort_by(|left, right| left.descriptor.entry_id.cmp(&right.descriptor.entry_id));

    let registered_entry_count = entries.len();
    let marketed_entry_count = entries.iter().filter(|entry| entry.marketed).count();
    let confirm_reject_parity_count = entries
        .iter()
        .filter(|entry| {
            entry
                .confirm_reject_outcome
                .reuses_in_product_authority_path
        })
        .count();

    let (entry_kind_coverage, authority_scope_coverage, findings_summary) =
        summarize_report(&entries);
    let narrowable_marketed_entries = compute_narrowable_entries(&entries);
    let report_clean = findings_summary.total_blocking_findings == 0;

    let mut platform_set: Vec<CallbackReviewPlatform> = Vec::new();
    for entry in &entries {
        for platform in &entry.descriptor.claimed_platforms {
            if !platform_set.contains(platform) {
                platform_set.push(*platform);
            }
        }
    }
    platform_set.sort();

    let mut disposition_index: Vec<CallbackReviewDispositionEntry> = entries
        .iter()
        .map(|entry| CallbackReviewDispositionEntry {
            entry_id: entry.descriptor.entry_id.clone(),
            entry_kind: entry.descriptor.entry_kind,
            disclosed_origin_ref: entry.descriptor.disclosed_origin_ref.clone(),
            requested_action: entry.descriptor.requested_action,
            authority_scope: entry.descriptor.authority_scope,
            outcome: entry.descriptor.outcome,
            requires_confirm_reject: entry.descriptor.requires_confirm_reject,
        })
        .collect();
    disposition_index.sort_by(|left, right| left.entry_id.cmp(&right.entry_id));

    CallbackReviewReport {
        record_kind: CALLBACK_REVIEW_REPORT_RECORD_KIND.to_owned(),
        schema_version: CALLBACK_REVIEW_SCHEMA_VERSION,
        shared_contract_ref: CALLBACK_REVIEW_SHARED_CONTRACT_REF.to_owned(),
        report_id: CALLBACK_REVIEW_REPORT_ID.to_owned(),
        source_schema_ref: CALLBACK_REVIEW_SOURCE_SCHEMA_REF.to_owned(),
        required_entry_kinds: CallbackEntryKind::required_kinds().to_vec(),
        claimed_platforms: platform_set,
        cross_links: CallbackReviewCrossLinks::canonical(),
        entries,
        entry_kind_coverage,
        authority_scope_coverage,
        findings_summary,
        disposition_index,
        registered_entry_count,
        marketed_entry_count,
        confirm_reject_parity_count,
        narrowable_marketed_entries,
        report_clean,
        published_report_ref: CALLBACK_REVIEW_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: CALLBACK_REVIEW_PUBLISHED_DOC_REF.to_owned(),
        docs_help_refs: vec![
            CALLBACK_REVIEW_PUBLISHED_DOC_REF.to_owned(),
            "docs/help/auth_callback_and_protocol_handlers.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-callback-and-deep-link".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_callback_review_report`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum CallbackReviewValidationError {
    /// The report has no registered entries.
    NoRegisteredEntries,
    /// A required entry kind has no registered entry.
    RequiredEntryKindMissing { entry_kind: String },
    /// No entry provably reuses the in-product authority path.
    NoConfirmRejectParity,
    /// A blocking finding remains on an entry.
    BlockingFindingPresent { entry_id: String, class: String },
    /// A cross-link ref is empty.
    CrossLinkMissing { field: String },
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
    /// An entry's descriptor revision ref is empty.
    MissingDescriptorRevisionRef { entry_id: String },
}

/// Validates a report against the callback-review acceptance invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_callback_review_report(
    report: &CallbackReviewReport,
) -> Result<(), Vec<CallbackReviewValidationError>> {
    let mut errors = Vec::new();

    if report.entries.is_empty() {
        errors.push(CallbackReviewValidationError::NoRegisteredEntries);
    }

    for kind in CallbackEntryKind::required_kinds() {
        let present = report
            .entries
            .iter()
            .any(|entry| entry.descriptor.entry_kind == kind);
        if !present {
            errors.push(CallbackReviewValidationError::RequiredEntryKindMissing {
                entry_kind: kind.as_str().to_owned(),
            });
        }
    }

    if !report.has_confirm_reject_parity() {
        errors.push(CallbackReviewValidationError::NoConfirmRejectParity);
    }

    for entry in &report.entries {
        if entry.descriptor.descriptor_revision_ref.trim().is_empty() {
            errors.push(
                CallbackReviewValidationError::MissingDescriptorRevisionRef {
                    entry_id: entry.descriptor.entry_id.clone(),
                },
            );
        }
        for finding in &entry.blocking_findings {
            errors.push(CallbackReviewValidationError::BlockingFindingPresent {
                entry_id: finding.entry_id().to_owned(),
                class: finding.class_token().to_owned(),
            });
        }
    }

    for (field, value) in report.cross_links.as_pairs() {
        if value.trim().is_empty() {
            errors.push(CallbackReviewValidationError::CrossLinkMissing {
                field: field.to_owned(),
            });
        }
    }

    if report.published_report_ref.trim().is_empty() {
        errors.push(CallbackReviewValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(CallbackReviewValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Seed row used by [`seeded_callback_review_report`].
struct EntrySeed {
    entry_id: &'static str,
    entry_kind: CallbackEntryKind,
    source_class: CallbackSourceClass,
    origin_assurance: OriginAssuranceClass,
    disclosed_origin_ref: &'static str,
    requested_action: RequestedActionClass,
    target_identity_ref: &'static str,
    workspace_scope_ref: Option<&'static str>,
    tenant_scope_ref: Option<&'static str>,
    authority_scope: AuthorityScopeClass,
    confirm_reject_sheet_ref: Option<&'static str>,
    canonical_command_ref: &'static str,
    outcome: CallbackOutcomeClass,
    recovery_actions: &'static [CallbackRecoveryAction],
    local_continuity: LocalContinuityPosture,
    continuity_note: &'static str,
    degraded_state_vocabulary: &'static [&'static str],
}

fn build_entry_from_seed(seed: &EntrySeed) -> CallbackReviewRow {
    let widens_authority = seed.authority_scope.requires_confirm_reject();
    let descriptor = CallbackReviewDescriptor {
        entry_id: seed.entry_id.to_owned(),
        entry_kind: seed.entry_kind,
        source_class: seed.source_class,
        origin_assurance: seed.origin_assurance,
        descriptor_revision_ref: format!("{}:rev:2026.06.01-01", seed.entry_id),
        primary_label_ref: format!("label:{}:primary", seed.entry_id),
        disclosed_origin_ref: seed.disclosed_origin_ref.to_owned(),
        requested_action: seed.requested_action,
        target_identity_ref: seed.target_identity_ref.to_owned(),
        workspace_scope_ref: seed.workspace_scope_ref.map(str::to_owned),
        tenant_scope_ref: seed.tenant_scope_ref.map(str::to_owned),
        authority_scope: seed.authority_scope,
        widens_authority,
        requires_confirm_reject: widens_authority,
        confirm_reject_sheet_ref: seed.confirm_reject_sheet_ref.map(str::to_owned),
        pending_correlation_ref: format!("correlation:{}:state_nonce_pkce", seed.entry_id),
        expiry_at: "2026-06-16T00:10:00Z".to_owned(),
        active_profile_owner_ref: format!("profile-owner:{}", seed.entry_id),
        trust_checkpoint_ref: format!("trust:{}:profile_tenant_policy", seed.entry_id),
        canonical_command_ref: seed.canonical_command_ref.to_owned(),
        outcome: seed.outcome,
        recovery_actions: seed.recovery_actions.to_vec(),
        local_continuity: seed.local_continuity,
        continuity_note: seed.continuity_note.to_owned(),
        degraded_state_vocabulary: seed
            .degraded_state_vocabulary
            .iter()
            .map(|phrase| (*phrase).to_owned())
            .collect(),
        claimed_platforms: CallbackReviewPlatform::all().to_vec(),
        evidence_freshness: CallbackEvidenceFreshness::Fresh,
        evidence_captured_at: GENERATED_AT.to_owned(),
        downgrade_rule_ref: "downgrade:callback_review:narrow_on_stale_evidence".to_owned(),
        redaction_safe: true,
        marketed: true,
        registered_on_callback_review_harness: true,
    };
    build_callback_review_row(descriptor)
}

const ENTRY_SEEDS: &[EntrySeed] = &[
    // ---- Clean entry-kind rows: one per required kind. ----
    // Auth-provider callback: a system-browser sign-in return, gated.
    EntrySeed {
        entry_id: "callback:auth_provider.system_browser",
        entry_kind: CallbackEntryKind::AuthProviderCallback,
        source_class: CallbackSourceClass::SystemDefaultBrowserReturn,
        origin_assurance: OriginAssuranceClass::StrictOriginMatched,
        disclosed_origin_ref: "origin:auth_provider.system_browser:disclosed",
        requested_action: RequestedActionClass::ResumePendingSignIn,
        target_identity_ref: "target:auth_provider.system_browser:pending_sign_in",
        workspace_scope_ref: None,
        tenant_scope_ref: Some("tenant:auth_provider.system_browser:bound"),
        authority_scope: AuthorityScopeClass::WidensToManagedAuthority,
        confirm_reject_sheet_ref: Some(
            "confirm_reject:auth_provider.system_browser:resume_sign_in",
        ),
        canonical_command_ref: "cmd:auth.resume_pending_sign_in",
        outcome: CallbackOutcomeClass::Admitted,
        recovery_actions: &[],
        local_continuity: LocalContinuityPosture::LocalWorkIntactManagedNarrowed,
        continuity_note: "A system-browser auth callback returns to the exact pending sign-in in the originating profile behind a confirm/reject sheet and never silently widens authority past what the sign-in requested.",
        degraded_state_vocabulary: &[
            "Return to Aureline to finish signing in",
            "Confirm this sign-in or stay signed out",
            "Keep working locally without signing in",
        ],
    },
    // Protocol deep link: a plain local open, the fast path.
    EntrySeed {
        entry_id: "callback:protocol_deep_link.open_local",
        entry_kind: CallbackEntryKind::ProtocolDeepLink,
        source_class: CallbackSourceClass::RegisteredProtocolHandler,
        origin_assurance: OriginAssuranceClass::DeepLinkSchemePinned,
        disclosed_origin_ref: "origin:protocol_deep_link.open_local:disclosed",
        requested_action: RequestedActionClass::OpenExistingLocalContext,
        target_identity_ref: "target:protocol_deep_link.open_local:local_context",
        workspace_scope_ref: Some("workspace:protocol_deep_link.open_local:active"),
        tenant_scope_ref: None,
        authority_scope: AuthorityScopeClass::PlainLocalOpen,
        confirm_reject_sheet_ref: None,
        canonical_command_ref: "cmd:workspace.open.target",
        outcome: CallbackOutcomeClass::Admitted,
        recovery_actions: &[],
        local_continuity: LocalContinuityPosture::LocalIntentPreserved,
        continuity_note: "A protocol deep link that resolves to an already-trusted local context opens directly with no authority widening and therefore no confirm/reject sheet.",
        degraded_state_vocabulary: &[
            "Open this item",
            "This item is no longer available",
            "Choose a different item",
        ],
    },
    // Review handoff link: inspect-only, crosses a boundary, gated.
    EntrySeed {
        entry_id: "callback:review_handoff.web_return",
        entry_kind: CallbackEntryKind::ReviewHandoffLink,
        source_class: CallbackSourceClass::FirstPartyWebReturn,
        origin_assurance: OriginAssuranceClass::FirstPartySignedLink,
        disclosed_origin_ref: "origin:review_handoff.web_return:disclosed",
        requested_action: RequestedActionClass::InspectReviewItem,
        target_identity_ref: "target:review_handoff.web_return:review_item",
        workspace_scope_ref: Some("workspace:review_handoff.web_return:scoped"),
        tenant_scope_ref: None,
        authority_scope: AuthorityScopeClass::CrossesBoundaryReadOnly,
        confirm_reject_sheet_ref: Some("confirm_reject:review_handoff.web_return:open_review"),
        canonical_command_ref: "cmd:review.open_handoff",
        outcome: CallbackOutcomeClass::Admitted,
        recovery_actions: &[],
        local_continuity: LocalContinuityPosture::LocalIntentPreserved,
        continuity_note: "A review handoff link opens the review surface inspect-only behind a confirm/reject sheet because it crosses a boundary, and is never coerced into a mutating action.",
        degraded_state_vocabulary: &[
            "Review this item without making changes",
            "This review link points to an item you cannot access",
            "This review link has expired",
        ],
    },
    // Collaboration join link: joins presence, gated.
    EntrySeed {
        entry_id: "callback:collaboration_join.presence",
        entry_kind: CallbackEntryKind::CollaborationJoinLink,
        source_class: CallbackSourceClass::CollaborationService,
        origin_assurance: OriginAssuranceClass::FirstPartySignedLink,
        disclosed_origin_ref: "origin:collaboration_join.presence:disclosed",
        requested_action: RequestedActionClass::JoinCollaboration,
        target_identity_ref: "target:collaboration_join.presence:session",
        workspace_scope_ref: Some("workspace:collaboration_join.presence:shared"),
        tenant_scope_ref: Some("tenant:collaboration_join.presence:bound"),
        authority_scope: AuthorityScopeClass::WorkspaceCollaborationJoin,
        confirm_reject_sheet_ref: Some("confirm_reject:collaboration_join.presence:join"),
        canonical_command_ref: "cmd:collab.join_session",
        outcome: CallbackOutcomeClass::Admitted,
        recovery_actions: &[],
        local_continuity: LocalContinuityPosture::LocalWorkIntactManagedNarrowed,
        continuity_note: "A collaboration join link joins shared presence only after an explicit confirm/reject sheet discloses who is hosting and what is shared, and never auto-joins on open.",
        degraded_state_vocabulary: &[
            "Join this collaboration session",
            "This session is no longer active",
            "Keep working locally without joining",
        ],
    },
    // Managed resume link: resumes a managed action, gated.
    EntrySeed {
        entry_id: "callback:managed_resume.companion",
        entry_kind: CallbackEntryKind::ManagedResumeLink,
        source_class: CallbackSourceClass::TrustedCompanionApp,
        origin_assurance: OriginAssuranceClass::DeepLinkSchemePinned,
        disclosed_origin_ref: "origin:managed_resume.companion:disclosed",
        requested_action: RequestedActionClass::ResumeManagedAction,
        target_identity_ref: "target:managed_resume.companion:managed_action",
        workspace_scope_ref: Some("workspace:managed_resume.companion:scoped"),
        tenant_scope_ref: Some("tenant:managed_resume.companion:bound"),
        authority_scope: AuthorityScopeClass::WidensToManagedAuthority,
        confirm_reject_sheet_ref: Some("confirm_reject:managed_resume.companion:resume_action"),
        canonical_command_ref: "cmd:managed.resume_action",
        outcome: CallbackOutcomeClass::Admitted,
        recovery_actions: &[],
        local_continuity: LocalContinuityPosture::LocalWorkIntactManagedNarrowed,
        continuity_note: "A managed-resume link from a trusted companion resumes a managed action only behind a confirm/reject sheet that names the action and its scope, and never widens authority silently.",
        degraded_state_vocabulary: &[
            "Resume this managed action",
            "This managed action is no longer available",
            "Keep working locally without resuming",
        ],
    },
    // Remote mutation link: opens a remote mutation, gated.
    EntrySeed {
        entry_id: "callback:remote_mutation.provider",
        entry_kind: CallbackEntryKind::RemoteMutationLink,
        source_class: CallbackSourceClass::ExternalProvider,
        origin_assurance: OriginAssuranceClass::StrictOriginMatched,
        disclosed_origin_ref: "origin:remote_mutation.provider:disclosed",
        requested_action: RequestedActionClass::OpenRemoteMutation,
        target_identity_ref: "target:remote_mutation.provider:remote_resource",
        workspace_scope_ref: Some("workspace:remote_mutation.provider:scoped"),
        tenant_scope_ref: Some("tenant:remote_mutation.provider:bound"),
        authority_scope: AuthorityScopeClass::WidensToProviderMutation,
        confirm_reject_sheet_ref: Some("confirm_reject:remote_mutation.provider:confirm_mutation"),
        canonical_command_ref: "cmd:provider.open_remote_mutation",
        outcome: CallbackOutcomeClass::Admitted,
        recovery_actions: &[],
        local_continuity: LocalContinuityPosture::LocalWorkIntactManagedNarrowed,
        continuity_note: "A provider link that would mutate remote state always shows a confirm/reject sheet disclosing the exact mutation before any write, and never auto-commits from an open.",
        degraded_state_vocabulary: &[
            "Confirm this remote change before it is applied",
            "This remote target is unreachable",
            "Keep working locally without the remote change",
        ],
    },
    // ---- Denied case rows: the four required incident fixtures. ----
    // Denied: a review handoff link blocked by managed policy.
    EntrySeed {
        entry_id: "callback:case.denied",
        entry_kind: CallbackEntryKind::ReviewHandoffLink,
        source_class: CallbackSourceClass::FirstPartyWebReturn,
        origin_assurance: OriginAssuranceClass::FirstPartySignedLink,
        disclosed_origin_ref: "origin:case.denied:disclosed",
        requested_action: RequestedActionClass::InspectReviewItem,
        target_identity_ref: "target:case.denied:review_item",
        workspace_scope_ref: Some("workspace:case.denied:scoped"),
        tenant_scope_ref: None,
        authority_scope: AuthorityScopeClass::CrossesBoundaryReadOnly,
        confirm_reject_sheet_ref: Some("confirm_reject:case.denied:policy_block"),
        canonical_command_ref: "cmd:review.open_handoff",
        outcome: CallbackOutcomeClass::DeniedByPolicy,
        recovery_actions: &[
            CallbackRecoveryAction::ShowPolicyBlockDetail,
            CallbackRecoveryAction::ReturnToReviewSurface,
            CallbackRecoveryAction::KeepLocalWorkAndDismiss,
        ],
        local_continuity: LocalContinuityPosture::LocalIntentPreserved,
        continuity_note: "A review link blocked by managed policy degrades to a policy-block detail with a return path and keeps local work, never a silent dead-end or an unscoped retry.",
        degraded_state_vocabulary: &[
            "This link was blocked by policy",
            "See why this was blocked",
            "Return to the review surface",
        ],
    },
    // Stale: a managed-resume link whose pending session was superseded.
    EntrySeed {
        entry_id: "callback:case.stale",
        entry_kind: CallbackEntryKind::ManagedResumeLink,
        source_class: CallbackSourceClass::TrustedCompanionApp,
        origin_assurance: OriginAssuranceClass::DeepLinkSchemePinned,
        disclosed_origin_ref: "origin:case.stale:disclosed",
        requested_action: RequestedActionClass::ResumeManagedAction,
        target_identity_ref: "target:case.stale:managed_action",
        workspace_scope_ref: Some("workspace:case.stale:scoped"),
        tenant_scope_ref: Some("tenant:case.stale:bound"),
        authority_scope: AuthorityScopeClass::WidensToManagedAuthority,
        confirm_reject_sheet_ref: Some("confirm_reject:case.stale:resume_action"),
        canonical_command_ref: "cmd:managed.resume_action",
        outcome: CallbackOutcomeClass::DeniedStale,
        recovery_actions: &[
            CallbackRecoveryAction::ReturnToPendingSignIn,
            CallbackRecoveryAction::RetryInSystemBrowser,
            CallbackRecoveryAction::ContinueLocalWithoutCallback,
        ],
        local_continuity: LocalContinuityPosture::LocalWorkIntactManagedNarrowed,
        continuity_note: "A managed-resume link whose pending session was superseded surfaces the stale state explicitly and offers a fresh resume, never silently replaying an outdated action.",
        degraded_state_vocabulary: &[
            "This session was replaced by a newer one",
            "Resume from the current session",
            "Keep working locally without resuming",
        ],
    },
    // Expired: an auth callback whose link expired before review.
    EntrySeed {
        entry_id: "callback:case.expired",
        entry_kind: CallbackEntryKind::AuthProviderCallback,
        source_class: CallbackSourceClass::SystemDefaultBrowserReturn,
        origin_assurance: OriginAssuranceClass::StrictOriginMatched,
        disclosed_origin_ref: "origin:case.expired:disclosed",
        requested_action: RequestedActionClass::ResumePendingSignIn,
        target_identity_ref: "target:case.expired:pending_sign_in",
        workspace_scope_ref: None,
        tenant_scope_ref: Some("tenant:case.expired:bound"),
        authority_scope: AuthorityScopeClass::WidensToManagedAuthority,
        confirm_reject_sheet_ref: Some("confirm_reject:case.expired:resume_sign_in"),
        canonical_command_ref: "cmd:auth.resume_pending_sign_in",
        outcome: CallbackOutcomeClass::DeniedExpired,
        recovery_actions: &[
            CallbackRecoveryAction::RequestFreshLink,
            CallbackRecoveryAction::RetryInSystemBrowser,
            CallbackRecoveryAction::ContinueLocalWithoutCallback,
        ],
        local_continuity: LocalContinuityPosture::LocalWorkIntactManagedNarrowed,
        continuity_note: "An auth callback that arrives after its expiry says so plainly and offers a fresh sign-in link, never a silent no-op that looks like nothing happened.",
        degraded_state_vocabulary: &[
            "This sign-in link has expired",
            "Request a fresh sign-in link",
            "Keep working locally without signing in",
        ],
    },
    // Wrong-origin: an auth callback from an unverified external origin.
    EntrySeed {
        entry_id: "callback:case.wrong_origin",
        entry_kind: CallbackEntryKind::AuthProviderCallback,
        source_class: CallbackSourceClass::ExternalProvider,
        origin_assurance: OriginAssuranceClass::OriginUnverified,
        disclosed_origin_ref: "origin:case.wrong_origin:disclosed",
        requested_action: RequestedActionClass::ResumePendingSignIn,
        target_identity_ref: "target:case.wrong_origin:pending_sign_in",
        workspace_scope_ref: None,
        tenant_scope_ref: Some("tenant:case.wrong_origin:bound"),
        authority_scope: AuthorityScopeClass::WidensToManagedAuthority,
        confirm_reject_sheet_ref: Some("confirm_reject:case.wrong_origin:origin_mismatch"),
        canonical_command_ref: "cmd:auth.resume_pending_sign_in",
        outcome: CallbackOutcomeClass::DeniedWrongOrigin,
        recovery_actions: &[
            CallbackRecoveryAction::ShowOriginMismatchDetail,
            CallbackRecoveryAction::RetryInSystemBrowser,
            CallbackRecoveryAction::ContinueLocalWithoutCallback,
        ],
        local_continuity: LocalContinuityPosture::LocalWorkIntactManagedNarrowed,
        continuity_note: "An auth callback whose origin does not match the pending handoff is named as an origin mismatch with a detail view, never an arbitrary auth failure, and is denied before any authority widens.",
        degraded_state_vocabulary: &[
            "This sign-in came from an origin we could not verify",
            "See why this origin was rejected",
            "Retry the sign-in in your browser",
        ],
    },
];

/// Seeded report builder used by the headless inspector and the integration
/// test. The seed mirrors the JSON fixtures checked in under
/// `fixtures/platform/m5-callback-and-deep-link/`.
pub fn seeded_callback_review_report() -> CallbackReviewReport {
    let entries = ENTRY_SEEDS.iter().map(build_entry_from_seed).collect();
    build_callback_review_report(entries)
}

/// Stable case-id label for the four required incident fixtures.
pub const CALLBACK_REVIEW_CASE_LABELS: [(&str, &str); 4] = [
    ("callback:case.wrong_origin", "wrong_origin"),
    ("callback:case.expired", "expired"),
    ("callback:case.stale", "stale"),
    ("callback:case.denied", "denied"),
];

/// Builds the four per-incident case exports from the seeded report, in
/// canonical order.
pub fn seeded_callback_review_case_exports() -> Vec<CallbackReviewCaseExport> {
    let report = seeded_callback_review_report();
    CALLBACK_REVIEW_CASE_LABELS
        .iter()
        .filter_map(|(entry_id, label)| {
            let row = report
                .entries
                .iter()
                .find(|entry| entry.descriptor.entry_id == *entry_id)?
                .clone();
            Some(CallbackReviewCaseExport::from_row(
                format!("support-export:m5-callback-and-deep-link:case:{label}"),
                *label,
                format!(
                    "Reproduce the {label} return from this typed entry: the disclosed origin and its assurance class, the requested action and authority scope, the expiry and pending correlation, the review outcome, and the offered recovery actions.",
                ),
                row,
            ))
        })
        .collect()
}
