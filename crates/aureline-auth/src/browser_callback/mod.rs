//! System-browser auth callback seed and local-versus-managed shell vocabulary.
//!
//! This module owns the [`BrowserCallbackPacket`] record that the auth lane
//! mints before any outbound system-browser handoff and validates against on
//! return, plus the [`ShellAuthVocabulary`] projection a shell consumer renders
//! as a local-vs-managed chip without forking identity truth.
//!
//! ## Why one inspectable packet
//!
//! Without a single seed object, every later surface is free to invent a
//! local `is_signed_in` boolean, accept a returning browser tab without
//! verifying the bound workspace / tenant, or silently fall back to an
//! embedded credential collector when a system-browser launch is blocked.
//! The packet closes those gaps before the first real OAuth / OIDC integration
//! lands. Live provider adapters remain out of scope here; this seed freezes
//! the vocabulary they will land on.
//!
//! ## Shared vocabulary
//!
//! The packet re-uses [`aureline_runtime::IdentityMode`] so the auth lane and
//! the execution-context lane stay on the same identity-mode tokens, and re-
//! exports [`aureline_workspace::TrustState`] for the trust posture stamped on
//! every callback. Surfaces that already read the execution-context object can
//! consume the packet's `policy_and_trust` block without translating
//! vocabularies.
//!
//! ## Failure-drill posture
//!
//! [`BrowserCallbackHandoff::redeem`] fails closed with a typed
//! [`PendingSessionDeniedReason`] when:
//!
//! - the returning state token alias does not match the pending correlation
//!   (`callback_replay_or_state_mismatch`),
//! - the returning origin does not match the pinned validation class
//!   (`callback_origin_mismatch`),
//! - the returning tenant / workspace binding does not match the rule on the
//!   return route (`callback_tenant_or_workspace_mismatch`), or
//! - the launch posture would silently fall back to an embedded webview
//!   (`callback_embedded_fallback_attempted`).
//!
//! In every denied case the packet preserves its
//! [`PreservedLocalWork`] block so a no-account local path keeps working
//! truthfully — the fixture
//! `/fixtures/auth/browser_callback_cases/failure_drill_app_partially_unavailable.json`
//! exercises the named failure drill end to end.

use serde::{Deserialize, Serialize};

pub use aureline_runtime::IdentityMode as IdentityModeAlias;
pub use aureline_workspace::TrustState;

/// Record-kind tag carried on serialized [`BrowserCallbackPacket`] payloads.
pub const BROWSER_CALLBACK_PACKET_RECORD_KIND: &str = "browser_callback_packet_seed_record";

/// Schema version of the seed [`BrowserCallbackPacket`] payload.
///
/// Bumped on breaking payload changes; additive-optional fields do not bump
/// this version. The frozen cross-tool boundary vocabulary in
/// `/schemas/auth/auth_callback_state.schema.json` follows the same versioning
/// rule.
pub const BROWSER_CALLBACK_PACKET_SCHEMA_VERSION: u32 = 1;

/// Auth-flow class. Closed seed vocabulary mirrored from
/// `/docs/auth/system_browser_callback_packet.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthFlowClass {
    /// Outbound auth handoff over the system browser. The default and
    /// required class for any supported sign-in row.
    SystemBrowser,
    /// Device-code poll return. Admissible only when the system browser is
    /// blocked by policy or the platform.
    DeviceCode,
    /// Platform-native authenticator (passkey / WebAuthn) on a host-native
    /// surface; never an embedded auth page.
    PlatformAuthenticatorNative,
    /// No outbound flow required; the install is running in account-free
    /// local mode.
    NotApplicable,
}

impl AuthFlowClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemBrowser => "system_browser",
            Self::DeviceCode => "device_code",
            Self::PlatformAuthenticatorNative => "platform_authenticator_native",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Browser-launch policy class. The closed vocabulary frozen in the packet
/// contract; system-default-browser use is the default and required posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserLaunchPolicyClass {
    SystemDefaultBrowserRequired,
    ManagedApprovedBrowserAllowed,
    SeparatelyApprovedBoundaryContract,
    BrowserLaunchPolicyBlocked,
}

impl BrowserLaunchPolicyClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemDefaultBrowserRequired => "system_default_browser_required",
            Self::ManagedApprovedBrowserAllowed => "managed_approved_browser_allowed",
            Self::SeparatelyApprovedBoundaryContract => "separately_approved_boundary_contract",
            Self::BrowserLaunchPolicyBlocked => "browser_launch_policy_blocked",
        }
    }

    /// True when the surface MUST surface a visible-recovery typed retry path
    /// (the policy denies the system-browser launch outright).
    pub const fn requires_visible_recovery(self) -> bool {
        matches!(self, Self::BrowserLaunchPolicyBlocked)
    }
}

/// Embedded-fallback posture. Silent fallback to an embedded webview is
/// never admissible; the packet declares `EmbeddedFallbackForbidden` for every
/// `system_browser` row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddedFallbackPosture {
    EmbeddedFallbackForbidden,
}

impl EmbeddedFallbackPosture {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmbeddedFallbackForbidden => "embedded_fallback_forbidden",
        }
    }
}

/// Pending-session lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PendingSessionState {
    /// Outbound handoff has not yet started; the packet is staged.
    #[serde(rename = "staged")]
    Staged,
    /// Outbound handoff is open and awaiting browser return.
    #[serde(rename = "awaiting_browser_return")]
    AwaitingBrowserReturn,
    /// Browser return arrived and validated; sign-in is complete.
    #[serde(rename = "completed")]
    Completed,
    /// Browser return arrived but failed origin / tenant / workspace / replay
    /// validation. Carries a typed [`PendingSessionDeniedReason`].
    #[serde(rename = "return_denied")]
    Denied,
    /// A newer outbound handoff replaced this packet; explicit supersedence
    /// rather than silent deletion.
    #[serde(rename = "session_superseded")]
    Superseded,
    /// Pending session expired before a browser return was observed.
    #[serde(rename = "session_expired")]
    Expired,
}

impl PendingSessionState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Staged => "staged",
            Self::AwaitingBrowserReturn => "awaiting_browser_return",
            Self::Completed => "completed",
            Self::Denied => "return_denied",
            Self::Superseded => "session_superseded",
            Self::Expired => "session_expired",
        }
    }
}

/// Typed denial reason recorded when a callback fails closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PendingSessionDeniedReason {
    CallbackOriginMismatch,
    CallbackReplayOrStateMismatch,
    CallbackTenantOrWorkspaceMismatch,
    CallbackEmbeddedFallbackAttempted,
    CallbackPolicyBlocked,
    CallbackPendingSessionExpired,
}

impl PendingSessionDeniedReason {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CallbackOriginMismatch => "callback_origin_mismatch",
            Self::CallbackReplayOrStateMismatch => "callback_replay_or_state_mismatch",
            Self::CallbackTenantOrWorkspaceMismatch => "callback_tenant_or_workspace_mismatch",
            Self::CallbackEmbeddedFallbackAttempted => "callback_embedded_fallback_attempted",
            Self::CallbackPolicyBlocked => "callback_policy_blocked",
            Self::CallbackPendingSessionExpired => "callback_pending_session_expired",
        }
    }
}

/// Account-boundary class. Closed seed vocabulary that distinguishes the
/// no-account local path from any signed-in or restricted posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountBoundaryClass {
    LocalOnly,
    SelfHosted,
    Managed,
    RestrictedManagedOnly,
    GraceDegradedManaged,
    UnknownBoundary,
}

impl AccountBoundaryClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::RestrictedManagedOnly => "restricted_managed_only",
            Self::GraceDegradedManaged => "grace_degraded_managed",
            Self::UnknownBoundary => "unknown_boundary",
        }
    }

    /// True when the surface MUST render the local-vs-managed boundary chip
    /// distinct from a `Connected` badge. `LocalOnly` and `UnknownBoundary`
    /// also need a visible chip — the first to disclose the no-account path,
    /// the second because it's a fail-closed state.
    pub const fn boundary_chip_required(self) -> bool {
        true
    }
}

/// Return-mode class. Names how the browser hands back to the app.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReturnModeClass {
    LoopbackHttpReturn,
    PlatformDeepLinkReturn,
    DeviceCodePollReturn,
    ManualReturnResume,
    NotApplicable,
}

impl ReturnModeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LoopbackHttpReturn => "loopback_http_return",
            Self::PlatformDeepLinkReturn => "platform_deep_link_return",
            Self::DeviceCodePollReturn => "device_code_poll_return",
            Self::ManualReturnResume => "manual_return_resume",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Origin-validation class for the returning browser state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReturnOriginValidationClass {
    StrictOriginMatchRequired,
    LoopbackPortPinned,
    DeepLinkSchemePinned,
    DeviceCodePollOnly,
    ManualResumeOnly,
}

impl ReturnOriginValidationClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StrictOriginMatchRequired => "strict_origin_match_required",
            Self::LoopbackPortPinned => "loopback_port_pinned",
            Self::DeepLinkSchemePinned => "deep_link_scheme_pinned",
            Self::DeviceCodePollOnly => "device_code_poll_only",
            Self::ManualResumeOnly => "manual_resume_only",
        }
    }
}

/// Tenant / workspace-match rule for the returning state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReturnTenantOrWorkspaceMatchRule {
    MustMatchBoundWorkspaceAndTenant,
    MustMatchBoundTenant,
    MustMatchBoundWorkspace,
    /// Admissible only for pre-workspace `account_free_local` sign-in.
    NoTenantOrWorkspaceBinding,
}

impl ReturnTenantOrWorkspaceMatchRule {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MustMatchBoundWorkspaceAndTenant => "must_match_bound_workspace_and_tenant",
            Self::MustMatchBoundTenant => "must_match_bound_tenant",
            Self::MustMatchBoundWorkspace => "must_match_bound_workspace",
            Self::NoTenantOrWorkspaceBinding => "no_tenant_or_workspace_binding",
        }
    }
}

/// Closed retry-path vocabulary for the recovery row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetryPathClass {
    RetryInSystemBrowser,
    SwitchToDeviceCode,
    ResumeAfterStepUp,
    ResumeAfterCredentialStoreUnlock,
    RequestAdminPolicyChange,
    ContinueLocalWithoutSignIn,
    ImportSignedSessionSnapshot,
    ReturnToAccountFreeLocal,
    ContactSupportWithExport,
    NoRecoveryWithoutSupersedingAction,
}

impl RetryPathClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetryInSystemBrowser => "retry_in_system_browser",
            Self::SwitchToDeviceCode => "switch_to_device_code",
            Self::ResumeAfterStepUp => "resume_after_step_up",
            Self::ResumeAfterCredentialStoreUnlock => "resume_after_credential_store_unlock",
            Self::RequestAdminPolicyChange => "request_admin_policy_change",
            Self::ContinueLocalWithoutSignIn => "continue_local_without_sign_in",
            Self::ImportSignedSessionSnapshot => "import_signed_session_snapshot",
            Self::ReturnToAccountFreeLocal => "return_to_account_free_local",
            Self::ContactSupportWithExport => "contact_support_with_export",
            Self::NoRecoveryWithoutSupersedingAction => "no_recovery_without_superseding_action",
        }
    }
}

/// Posture class for the preserved-local-work block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreservedLocalWorkPostureClass {
    LocalWorkIntact,
    LocalWorkIntactWithManagedNarrowed,
    LocalWorkIntactWithSelfHostedNarrowed,
    LocalWorkNarrowedByWorkspaceTrust,
    LocalWorkBlockedByPolicy,
}

impl PreservedLocalWorkPostureClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkIntact => "local_work_intact",
            Self::LocalWorkIntactWithManagedNarrowed => "local_work_intact_with_managed_narrowed",
            Self::LocalWorkIntactWithSelfHostedNarrowed => {
                "local_work_intact_with_self_hosted_narrowed"
            }
            Self::LocalWorkNarrowedByWorkspaceTrust => "local_work_narrowed_by_workspace_trust",
            Self::LocalWorkBlockedByPolicy => "local_work_blocked_by_policy",
        }
    }

    /// True when the no-account local path stays usable under this posture.
    pub const fn local_work_usable(self) -> bool {
        !matches!(self, Self::LocalWorkBlockedByPolicy)
    }
}

/// Preserved-local-work block. Surfaces quote this by reference rather than
/// re-describing what still works locally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreservedLocalWork {
    pub posture_class: PreservedLocalWorkPostureClass,
    pub note: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub retained_capabilities: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocked_capabilities: Vec<String>,
}

impl PreservedLocalWork {
    /// True when local editing, save, undo, search, local Git, local tasks,
    /// and BYOK AI keep working under this posture.
    pub fn local_work_usable(&self) -> bool {
        self.posture_class.local_work_usable()
    }
}

/// Recovery-path block. Carries the typed retry-path vocabulary the surface
/// renders verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryPath {
    pub primary_recovery_action: RetryPathClass,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fallback_recovery_actions: Vec<RetryPathClass>,
    pub visible_recovery_required: bool,
    pub recovery_copy_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repair_hook_ref: Option<String>,
}

impl RecoveryPath {
    /// True when `continue_local_without_sign_in` is reachable from this row,
    /// either as the primary or as a typed fallback action.
    pub fn local_continuity_offered(&self) -> bool {
        if self.primary_recovery_action == RetryPathClass::ContinueLocalWithoutSignIn {
            return true;
        }
        self.fallback_recovery_actions
            .iter()
            .any(|action| *action == RetryPathClass::ContinueLocalWithoutSignIn)
    }
}

/// Callback-correlation envelope. Aliases (not raw tokens) cross this
/// boundary; raw tokens, raw URLs, raw cookies, raw provider codes, raw PKCE
/// verifiers, and raw nonces never appear here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallbackCorrelation {
    pub correlation_id: String,
    pub pending_session_id: String,
    pub state_token_alias: String,
    pub nonce_alias: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pkce_challenge_alias: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_workspace_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_tenant_or_org_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_actor_subject_ref: Option<String>,
    pub issued_at: String,
    pub expires_at: String,
}

/// Return-route record. Names the validation class, anchor, target label,
/// tenant / workspace match rule, and pinned origin validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReturnRoute {
    pub return_mode_class: ReturnModeClass,
    pub return_anchor_ref: String,
    pub return_target_label: String,
    pub return_origin_validation_class: ReturnOriginValidationClass,
    pub return_tenant_or_workspace_match_rule: ReturnTenantOrWorkspaceMatchRule,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub return_policy_check_refs: Vec<String>,
}

/// Canonical seed [`BrowserCallbackPacket`] record.
///
/// Surfaces (terminal pane, activity center, status bar, support / export
/// flows) consume this object and quote its fields verbatim. They do not
/// re-derive `is_signed_in`, never collapse `restricted_managed_only` into
/// `managed`, and never present an embedded credential collector when
/// `browser_launch_policy_class` would forbid it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserCallbackPacket {
    pub record_kind: String,
    pub schema_version: u32,
    pub packet_id: String,
    pub auth_flow_class: AuthFlowClass,
    pub browser_launch_policy_class: BrowserLaunchPolicyClass,
    pub embedded_fallback_posture: EmbeddedFallbackPosture,
    pub pending_session_state: PendingSessionState,
    pub account_boundary_class: AccountBoundaryClass,
    pub identity_mode: IdentityModeAlias,
    pub trust_state: TrustState,
    pub provider_domain_label: String,
    pub destination_class_label: String,
    pub callback_correlation: CallbackCorrelation,
    pub return_route: ReturnRoute,
    pub preserved_local_work: PreservedLocalWork,
    pub recovery_path: RecoveryPath,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_session_denied_reason: Option<PendingSessionDeniedReason>,
    /// Optional ref into the execution-context lane so the auth packet and the
    /// canonical [`aureline_runtime::ExecutionContext`] stay joined for a
    /// support export.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_ref: Option<String>,
    pub minted_at: String,
}

impl BrowserCallbackPacket {
    /// Stable seed-packet id.
    pub fn packet_id(&self) -> &str {
        &self.packet_id
    }

    /// True when the packet declares an outbound system-browser handoff (as
    /// opposed to the no-account-required local path).
    pub fn is_system_browser_handoff(&self) -> bool {
        matches!(self.auth_flow_class, AuthFlowClass::SystemBrowser)
    }

    /// True when the packet's account-boundary posture is the no-account
    /// local path. Local editing, save, undo, search, local Git, local tasks,
    /// and BYOK AI MUST stay representable.
    pub fn is_local_only_path(&self) -> bool {
        matches!(self.account_boundary_class, AccountBoundaryClass::LocalOnly)
    }

    /// True when the surface MUST render a typed visible-recovery row alongside
    /// this packet (denied callback, expired session, browser-launch policy
    /// blocked, or any non-completed pending state on a managed posture).
    pub fn requires_visible_recovery(&self) -> bool {
        self.recovery_path.visible_recovery_required
            || self.browser_launch_policy_class.requires_visible_recovery()
            || matches!(
                self.pending_session_state,
                PendingSessionState::Denied
                    | PendingSessionState::Expired
                    | PendingSessionState::Superseded
            )
    }

    /// True when the packet's preserved-local-work block guarantees the
    /// no-account local path keeps working.
    pub fn preserves_local_work(&self) -> bool {
        self.preserved_local_work.local_work_usable()
    }
}

/// Inputs for staging an outbound system-browser handoff.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageSystemBrowserHandoffRequest<'a> {
    pub packet_id: &'a str,
    pub identity_mode: IdentityModeAlias,
    pub account_boundary_class: AccountBoundaryClass,
    pub trust_state: TrustState,
    pub provider_domain_label: &'a str,
    pub destination_class_label: &'a str,
    pub return_target_label: &'a str,
    pub return_anchor_ref: &'a str,
    pub return_mode_class: ReturnModeClass,
    pub return_origin_validation_class: ReturnOriginValidationClass,
    pub return_tenant_or_workspace_match_rule: ReturnTenantOrWorkspaceMatchRule,
    pub return_policy_check_refs: &'a [&'a str],
    pub bound_workspace_ref: Option<&'a str>,
    pub bound_tenant_or_org_ref: Option<&'a str>,
    pub bound_actor_subject_ref: Option<&'a str>,
    pub correlation_id: &'a str,
    pub pending_session_id: &'a str,
    pub state_token_alias: &'a str,
    pub nonce_alias: &'a str,
    pub pkce_challenge_alias: Option<&'a str>,
    pub issued_at: &'a str,
    pub expires_at: &'a str,
    pub recovery_copy_label: &'a str,
    pub primary_recovery_action: RetryPathClass,
    pub fallback_recovery_actions: &'a [RetryPathClass],
    pub repair_hook_ref: Option<&'a str>,
    pub preserved_local_work: PreservedLocalWork,
    pub execution_context_ref: Option<&'a str>,
}

/// Inputs for staging the no-account local path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageAccountFreeLocalRequest<'a> {
    pub packet_id: &'a str,
    pub correlation_id: &'a str,
    pub pending_session_id: &'a str,
    pub provider_domain_label: &'a str,
    pub destination_class_label: &'a str,
    pub return_anchor_ref: &'a str,
    pub return_target_label: &'a str,
    pub minted_at: &'a str,
    pub recovery_copy_label: &'a str,
    pub execution_context_ref: Option<&'a str>,
}

/// Inputs the validator inspects when a browser callback returns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnedCallbackInputs<'a> {
    pub returning_state_token_alias: &'a str,
    pub returning_origin_validation_class: ReturnOriginValidationClass,
    pub returning_tenant_or_org_ref: Option<&'a str>,
    pub returning_workspace_ref: Option<&'a str>,
    pub embedded_fallback_attempted: bool,
    pub policy_blocked: bool,
    pub observed_at: &'a str,
}

/// Validation errors emitted by [`BrowserCallbackHandoff`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrowserCallbackValidationError {
    EmbeddedFallbackForbidden,
    BrowserLaunchPolicyBlocked,
    SystemBrowserRequiredForFlow { attempted_flow: AuthFlowClass },
    AccountFreeLocalCannotStageOutboundHandoff,
}

impl std::fmt::Display for BrowserCallbackValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmbeddedFallbackForbidden => {
                f.write_str("embedded credential collection is forbidden by the seed contract")
            }
            Self::BrowserLaunchPolicyBlocked => f.write_str(
                "browser launch is policy-blocked; route to a typed visible-recovery action",
            ),
            Self::SystemBrowserRequiredForFlow { attempted_flow } => write!(
                f,
                "auth flow class {} cannot stage a system-browser handoff",
                attempted_flow.as_str(),
            ),
            Self::AccountFreeLocalCannotStageOutboundHandoff => f.write_str(
                "account-free-local boundary cannot stage an outbound system-browser handoff",
            ),
        }
    }
}

impl std::error::Error for BrowserCallbackValidationError {}

/// Stateful seed handoff. Owns one [`BrowserCallbackPacket`] from staging
/// through redemption, and exposes typed validators that fail closed with the
/// shared denial vocabulary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserCallbackHandoff {
    packet: BrowserCallbackPacket,
}

impl BrowserCallbackHandoff {
    /// Stage an outbound system-browser handoff.
    ///
    /// Fails closed if the request would silently allow an embedded fallback,
    /// if the policy class is `BrowserLaunchPolicyBlocked`, or if the request
    /// targets the no-account local path.
    pub fn stage_system_browser_handoff(
        request: StageSystemBrowserHandoffRequest<'_>,
    ) -> Result<Self, BrowserCallbackValidationError> {
        if request.account_boundary_class == AccountBoundaryClass::LocalOnly {
            return Err(BrowserCallbackValidationError::AccountFreeLocalCannotStageOutboundHandoff);
        }
        let packet = BrowserCallbackPacket {
            record_kind: BROWSER_CALLBACK_PACKET_RECORD_KIND.to_owned(),
            schema_version: BROWSER_CALLBACK_PACKET_SCHEMA_VERSION,
            packet_id: request.packet_id.to_owned(),
            auth_flow_class: AuthFlowClass::SystemBrowser,
            browser_launch_policy_class: BrowserLaunchPolicyClass::SystemDefaultBrowserRequired,
            embedded_fallback_posture: EmbeddedFallbackPosture::EmbeddedFallbackForbidden,
            pending_session_state: PendingSessionState::AwaitingBrowserReturn,
            account_boundary_class: request.account_boundary_class,
            identity_mode: request.identity_mode,
            trust_state: request.trust_state,
            provider_domain_label: request.provider_domain_label.to_owned(),
            destination_class_label: request.destination_class_label.to_owned(),
            callback_correlation: CallbackCorrelation {
                correlation_id: request.correlation_id.to_owned(),
                pending_session_id: request.pending_session_id.to_owned(),
                state_token_alias: request.state_token_alias.to_owned(),
                nonce_alias: request.nonce_alias.to_owned(),
                pkce_challenge_alias: request.pkce_challenge_alias.map(str::to_owned),
                bound_workspace_ref: request.bound_workspace_ref.map(str::to_owned),
                bound_tenant_or_org_ref: request.bound_tenant_or_org_ref.map(str::to_owned),
                bound_actor_subject_ref: request.bound_actor_subject_ref.map(str::to_owned),
                issued_at: request.issued_at.to_owned(),
                expires_at: request.expires_at.to_owned(),
            },
            return_route: ReturnRoute {
                return_mode_class: request.return_mode_class,
                return_anchor_ref: request.return_anchor_ref.to_owned(),
                return_target_label: request.return_target_label.to_owned(),
                return_origin_validation_class: request.return_origin_validation_class,
                return_tenant_or_workspace_match_rule: request
                    .return_tenant_or_workspace_match_rule,
                return_policy_check_refs: request
                    .return_policy_check_refs
                    .iter()
                    .map(|s| (*s).to_owned())
                    .collect(),
            },
            preserved_local_work: request.preserved_local_work,
            recovery_path: RecoveryPath {
                primary_recovery_action: request.primary_recovery_action,
                fallback_recovery_actions: request.fallback_recovery_actions.to_vec(),
                visible_recovery_required: true,
                recovery_copy_label: request.recovery_copy_label.to_owned(),
                repair_hook_ref: request.repair_hook_ref.map(str::to_owned),
            },
            pending_session_denied_reason: None,
            execution_context_ref: request.execution_context_ref.map(str::to_owned),
            minted_at: request.issued_at.to_owned(),
        };
        Ok(Self { packet })
    }

    /// Stage the no-account local path. The packet declares
    /// `auth_flow_class = not_applicable`, `account_boundary_class = local_only`,
    /// `pending_session_state = completed`, and a `ContinueLocalWithoutSignIn`
    /// recovery row.
    pub fn stage_account_free_local(request: StageAccountFreeLocalRequest<'_>) -> Self {
        let packet = BrowserCallbackPacket {
            record_kind: BROWSER_CALLBACK_PACKET_RECORD_KIND.to_owned(),
            schema_version: BROWSER_CALLBACK_PACKET_SCHEMA_VERSION,
            packet_id: request.packet_id.to_owned(),
            auth_flow_class: AuthFlowClass::NotApplicable,
            browser_launch_policy_class: BrowserLaunchPolicyClass::SystemDefaultBrowserRequired,
            embedded_fallback_posture: EmbeddedFallbackPosture::EmbeddedFallbackForbidden,
            pending_session_state: PendingSessionState::Completed,
            account_boundary_class: AccountBoundaryClass::LocalOnly,
            identity_mode: IdentityModeAlias::AccountFreeLocal,
            trust_state: TrustState::Trusted,
            provider_domain_label: request.provider_domain_label.to_owned(),
            destination_class_label: request.destination_class_label.to_owned(),
            callback_correlation: CallbackCorrelation {
                correlation_id: request.correlation_id.to_owned(),
                pending_session_id: request.pending_session_id.to_owned(),
                state_token_alias: format!("state_alias.{}.none", request.packet_id),
                nonce_alias: format!("nonce_alias.{}.none", request.packet_id),
                pkce_challenge_alias: None,
                bound_workspace_ref: None,
                bound_tenant_or_org_ref: None,
                bound_actor_subject_ref: None,
                issued_at: request.minted_at.to_owned(),
                expires_at: request.minted_at.to_owned(),
            },
            return_route: ReturnRoute {
                return_mode_class: ReturnModeClass::NotApplicable,
                return_anchor_ref: request.return_anchor_ref.to_owned(),
                return_target_label: request.return_target_label.to_owned(),
                return_origin_validation_class: ReturnOriginValidationClass::ManualResumeOnly,
                return_tenant_or_workspace_match_rule:
                    ReturnTenantOrWorkspaceMatchRule::NoTenantOrWorkspaceBinding,
                return_policy_check_refs: Vec::new(),
            },
            preserved_local_work: PreservedLocalWork {
                posture_class: PreservedLocalWorkPostureClass::LocalWorkIntact,
                note: "Local editing, save, undo, search, local Git, local tasks, and BYOK \
                       AI remain available without any sign-in."
                    .to_owned(),
                retained_capabilities: vec![
                    "Open, edit, save, and undo local workspaces.".to_owned(),
                    "Search over the local workspace and navigate symbols.".to_owned(),
                    "Run local Git commands and commit locally.".to_owned(),
                    "Run local tasks and debuggers in trusted workspaces.".to_owned(),
                    "Use BYOK AI providers and local model execution.".to_owned(),
                ],
                blocked_capabilities: Vec::new(),
            },
            recovery_path: RecoveryPath {
                primary_recovery_action: RetryPathClass::ContinueLocalWithoutSignIn,
                fallback_recovery_actions: Vec::new(),
                visible_recovery_required: false,
                recovery_copy_label: request.recovery_copy_label.to_owned(),
                repair_hook_ref: None,
            },
            pending_session_denied_reason: None,
            execution_context_ref: request.execution_context_ref.map(str::to_owned),
            minted_at: request.minted_at.to_owned(),
        };
        Self { packet }
    }

    /// Returns the underlying packet.
    pub fn packet(&self) -> &BrowserCallbackPacket {
        &self.packet
    }

    /// Consume into the underlying packet.
    pub fn into_packet(self) -> BrowserCallbackPacket {
        self.packet
    }

    /// Validate a returning browser state and either complete the pending
    /// session or fail closed with a typed denial reason. The packet's
    /// preserved-local-work block survives every denial.
    pub fn redeem(&mut self, returning: ReturnedCallbackInputs<'_>) -> &BrowserCallbackPacket {
        if returning.embedded_fallback_attempted {
            self.deny(PendingSessionDeniedReason::CallbackEmbeddedFallbackAttempted);
            return &self.packet;
        }
        if returning.policy_blocked {
            self.deny(PendingSessionDeniedReason::CallbackPolicyBlocked);
            return &self.packet;
        }
        if returning.returning_state_token_alias
            != self.packet.callback_correlation.state_token_alias
        {
            self.deny(PendingSessionDeniedReason::CallbackReplayOrStateMismatch);
            return &self.packet;
        }
        if returning.returning_origin_validation_class
            != self.packet.return_route.return_origin_validation_class
        {
            self.deny(PendingSessionDeniedReason::CallbackOriginMismatch);
            return &self.packet;
        }
        if returning.observed_at >= self.packet.callback_correlation.expires_at.as_str() {
            self.deny(PendingSessionDeniedReason::CallbackPendingSessionExpired);
            return &self.packet;
        }
        match self
            .packet
            .return_route
            .return_tenant_or_workspace_match_rule
        {
            ReturnTenantOrWorkspaceMatchRule::MustMatchBoundWorkspaceAndTenant => {
                let workspace_ok = returning.returning_workspace_ref
                    == self
                        .packet
                        .callback_correlation
                        .bound_workspace_ref
                        .as_deref();
                let tenant_ok = returning.returning_tenant_or_org_ref
                    == self
                        .packet
                        .callback_correlation
                        .bound_tenant_or_org_ref
                        .as_deref();
                if !workspace_ok || !tenant_ok {
                    self.deny(PendingSessionDeniedReason::CallbackTenantOrWorkspaceMismatch);
                    return &self.packet;
                }
            }
            ReturnTenantOrWorkspaceMatchRule::MustMatchBoundTenant => {
                if returning.returning_tenant_or_org_ref
                    != self
                        .packet
                        .callback_correlation
                        .bound_tenant_or_org_ref
                        .as_deref()
                {
                    self.deny(PendingSessionDeniedReason::CallbackTenantOrWorkspaceMismatch);
                    return &self.packet;
                }
            }
            ReturnTenantOrWorkspaceMatchRule::MustMatchBoundWorkspace => {
                if returning.returning_workspace_ref
                    != self
                        .packet
                        .callback_correlation
                        .bound_workspace_ref
                        .as_deref()
                {
                    self.deny(PendingSessionDeniedReason::CallbackTenantOrWorkspaceMismatch);
                    return &self.packet;
                }
            }
            ReturnTenantOrWorkspaceMatchRule::NoTenantOrWorkspaceBinding => {
                // Admissible only for pre-workspace account_free_local sign-in;
                // the no-account local path stages through
                // [`Self::stage_account_free_local`], not this validator.
            }
        }

        self.packet.pending_session_state = PendingSessionState::Completed;
        self.packet.pending_session_denied_reason = None;
        self.packet.recovery_path.visible_recovery_required = false;
        &self.packet
    }

    /// Mark the pending session as superseded by a newer outbound handoff.
    /// Supersedence is explicit; no silent deletion.
    pub fn supersede(&mut self) -> &BrowserCallbackPacket {
        self.packet.pending_session_state = PendingSessionState::Superseded;
        self.packet.pending_session_denied_reason = None;
        self.packet.recovery_path.visible_recovery_required = true;
        &self.packet
    }

    fn deny(&mut self, reason: PendingSessionDeniedReason) {
        self.packet.pending_session_state = PendingSessionState::Denied;
        self.packet.pending_session_denied_reason = Some(reason);
        self.packet.recovery_path.visible_recovery_required = true;
    }
}

/// Local-versus-managed shell vocabulary projected from a
/// [`BrowserCallbackPacket`]. The shell consumer renders one chip per posture
/// without inventing local truth; the chip never blocks the no-account local
/// path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellAuthVocabulary {
    /// No account; local-only path. Editing, save, undo, search, local Git,
    /// local tasks, and BYOK AI all keep working with no sign-in.
    AccountFreeLocal,
    /// Signed in to a managed (vendor-hosted convenience) workspace via the
    /// system browser.
    SignedInManaged,
    /// Signed in to a customer-run (self-hosted) IdP via the system browser.
    SignedInSelfHosted,
    /// Managed posture narrowed to managed-sign-in-required for this row.
    /// Local-only work stays truthful.
    RestrictedManagedOnly,
    /// Bounded grace posture: managed services unreachable; local and
    /// self-hosted capability stays truthful.
    GraceDegradedManaged,
    /// Sign-in is required again (session expiry, password reset, policy-
    /// epoch roll, trust downgrade). Local work keeps saving.
    ReauthRequired,
    /// Auth lane is reachable but no provider / IdP is configured yet.
    /// Surfaces render an honest "not yet configured" chip rather than a
    /// "Connected" badge.
    NotConfigured,
    /// Fail-closed posture: the boundary is unknown. Surfaces route to a
    /// typed repair rather than rendering an ambiguous `Connected` badge.
    UnknownBoundary,
}

impl ShellAuthVocabulary {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AccountFreeLocal => "shell_identity_account_free_local",
            Self::SignedInManaged => "shell_identity_signed_in_managed",
            Self::SignedInSelfHosted => "shell_identity_signed_in_self_hosted",
            Self::RestrictedManagedOnly => "shell_identity_restricted_managed_only",
            Self::GraceDegradedManaged => "shell_identity_grace_degraded_managed",
            Self::ReauthRequired => "shell_identity_reauth_required",
            Self::NotConfigured => "shell_identity_not_configured",
            Self::UnknownBoundary => "shell_identity_unknown_boundary",
        }
    }

    /// Short export-safe chip label rendered next to the terminal pane and
    /// quoted verbatim by support exports.
    pub const fn chip_label(self) -> &'static str {
        match self {
            Self::AccountFreeLocal => "Local only",
            Self::SignedInManaged => "Managed",
            Self::SignedInSelfHosted => "Self-hosted",
            Self::RestrictedManagedOnly => "Managed (restricted)",
            Self::GraceDegradedManaged => "Managed (degraded)",
            Self::ReauthRequired => "Sign in again",
            Self::NotConfigured => "Sign-in not configured",
            Self::UnknownBoundary => "Identity unknown",
        }
    }

    /// True when the chip MUST always stay readable on a no-account local
    /// flow. The seed contract requires the no-account path to remain visible
    /// even when sign-in is incomplete.
    pub const fn local_path_always_available(self) -> bool {
        true
    }

    /// Project a shell vocabulary chip from a [`BrowserCallbackPacket`].
    ///
    /// The projection is lossy by design: it collapses the packet's typed
    /// vocabulary onto one chip the bottom-panel chrome renders. Surfaces
    /// that need the full record continue to read it directly.
    pub fn from_packet(packet: &BrowserCallbackPacket) -> Self {
        match (packet.account_boundary_class, packet.pending_session_state) {
            (AccountBoundaryClass::LocalOnly, _) => Self::AccountFreeLocal,
            (AccountBoundaryClass::UnknownBoundary, _) => Self::UnknownBoundary,
            (AccountBoundaryClass::RestrictedManagedOnly, _) => Self::RestrictedManagedOnly,
            (AccountBoundaryClass::GraceDegradedManaged, _) => Self::GraceDegradedManaged,
            (
                _,
                PendingSessionState::Denied
                | PendingSessionState::Expired
                | PendingSessionState::Superseded
                | PendingSessionState::AwaitingBrowserReturn,
            ) => match packet.account_boundary_class {
                AccountBoundaryClass::Managed => Self::ReauthRequired,
                AccountBoundaryClass::SelfHosted => Self::ReauthRequired,
                _ => Self::ReauthRequired,
            },
            (AccountBoundaryClass::Managed, PendingSessionState::Completed) => {
                Self::SignedInManaged
            }
            (AccountBoundaryClass::SelfHosted, PendingSessionState::Completed) => {
                Self::SignedInSelfHosted
            }
            (AccountBoundaryClass::Managed | AccountBoundaryClass::SelfHosted, _) => {
                Self::NotConfigured
            }
        }
    }
}

/// Stable shell-auth chip projection a consumer renders on a protected row.
///
/// The chip carries the projected vocabulary, the chip label, the boundary
/// class token, the recovery copy, the `local_path_available` boolean, and
/// the seed packet id so a support export can round-trip the same truth a
/// terminal-pane row, an activity-center row, and a status mirror render.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellAuthChip {
    pub packet_id: String,
    pub vocabulary: ShellAuthVocabulary,
    pub vocabulary_token: String,
    pub chip_label: String,
    pub account_boundary_class: AccountBoundaryClass,
    pub account_boundary_class_token: String,
    pub identity_mode: IdentityModeAlias,
    pub trust_state: TrustState,
    pub local_path_available: bool,
    pub visible_recovery_required: bool,
    pub recovery_copy_label: String,
    pub primary_recovery_action: RetryPathClass,
    pub primary_recovery_action_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_ref: Option<String>,
}

impl ShellAuthChip {
    /// Project a chip from the seed packet.
    pub fn from_packet(packet: &BrowserCallbackPacket) -> Self {
        let vocabulary = ShellAuthVocabulary::from_packet(packet);
        Self {
            packet_id: packet.packet_id.clone(),
            vocabulary,
            vocabulary_token: vocabulary.as_str().to_owned(),
            chip_label: vocabulary.chip_label().to_owned(),
            account_boundary_class: packet.account_boundary_class,
            account_boundary_class_token: packet.account_boundary_class.as_str().to_owned(),
            identity_mode: packet.identity_mode,
            trust_state: packet.trust_state,
            local_path_available: packet.preserves_local_work(),
            visible_recovery_required: packet.requires_visible_recovery(),
            recovery_copy_label: packet.recovery_path.recovery_copy_label.clone(),
            primary_recovery_action: packet.recovery_path.primary_recovery_action,
            primary_recovery_action_token: packet
                .recovery_path
                .primary_recovery_action
                .as_str()
                .to_owned(),
            execution_context_ref: packet.execution_context_ref.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn baseline_local() -> BrowserCallbackHandoff {
        BrowserCallbackHandoff::stage_account_free_local(StageAccountFreeLocalRequest {
            packet_id: "browser_callback_packet.account_free_local.0001",
            correlation_id: "callback_correlation.account_free_local.0001",
            pending_session_id: "pending_session.account_free_local.0001",
            provider_domain_label: "No sign-in required",
            destination_class_label: "No browser handoff required",
            return_anchor_ref: "return_anchor.account_free_local.desktop",
            return_target_label: "Aureline desktop – local workspace",
            minted_at: "2026-04-23T10:00:00Z",
            recovery_copy_label: "You are using Aureline without a sign-in. \
                                  Local work stays on this device.",
            execution_context_ref: Some("execution_context.local_desktop.workspace_root"),
        })
    }

    fn baseline_managed_outbound() -> BrowserCallbackHandoff {
        BrowserCallbackHandoff::stage_system_browser_handoff(StageSystemBrowserHandoffRequest {
            packet_id: "browser_callback_packet.managed_sign_in.outbound.0001",
            identity_mode: IdentityModeAlias::ManagedConvenience,
            account_boundary_class: AccountBoundaryClass::Managed,
            trust_state: TrustState::Trusted,
            provider_domain_label: "login.acme.example",
            destination_class_label: "Customer-managed identity provider (system browser)",
            return_target_label: "Aureline desktop – payments-prod workspace",
            return_anchor_ref: "return_anchor.managed_sign_in.payments_prod",
            return_mode_class: ReturnModeClass::LoopbackHttpReturn,
            return_origin_validation_class: ReturnOriginValidationClass::LoopbackPortPinned,
            return_tenant_or_workspace_match_rule:
                ReturnTenantOrWorkspaceMatchRule::MustMatchBoundWorkspaceAndTenant,
            return_policy_check_refs: &[
                "policy_check.managed.sign_in.required",
                "policy_check.managed.system_browser.required",
            ],
            bound_workspace_ref: Some("workspace.payments_prod"),
            bound_tenant_or_org_ref: Some("tenant.acme_prod"),
            bound_actor_subject_ref: Some("actor_subject.sam.acme"),
            correlation_id: "callback_correlation.managed_sign_in.0001",
            pending_session_id: "pending_session.managed_sign_in.payments_prod.0001",
            state_token_alias: "state_alias.managed_sign_in.0001",
            nonce_alias: "nonce_alias.managed_sign_in.0001",
            pkce_challenge_alias: Some("pkce_alias.managed_sign_in.0001"),
            issued_at: "2026-04-23T10:10:00Z",
            expires_at: "2026-04-23T10:20:00Z",
            recovery_copy_label: "Continue sign-in in your browser. \
                                  Local work keeps saving to this device.",
            primary_recovery_action: RetryPathClass::RetryInSystemBrowser,
            fallback_recovery_actions: &[
                RetryPathClass::SwitchToDeviceCode,
                RetryPathClass::ContinueLocalWithoutSignIn,
                RetryPathClass::ContactSupportWithExport,
            ],
            repair_hook_ref: Some("repair_hook.managed_sign_in.retry.0001"),
            preserved_local_work: PreservedLocalWork {
                posture_class: PreservedLocalWorkPostureClass::LocalWorkIntactWithManagedNarrowed,
                note: "Local editing, save, undo, search, local Git, local tasks, and BYOK \
                       AI remain available while managed sign-in is incomplete."
                    .to_owned(),
                retained_capabilities: vec![
                    "Open, edit, save, and undo the payments-prod workspace locally.".to_owned(),
                    "Search over the local workspace and local history.".to_owned(),
                    "Run local Git commands and commit locally.".to_owned(),
                    "Use BYOK AI providers and local model execution.".to_owned(),
                ],
                blocked_capabilities: vec![
                    "Fetch managed settings sync while sign-in is incomplete.".to_owned(),
                    "Open managed marketplace or managed AI quota views.".to_owned(),
                ],
            },
            execution_context_ref: Some("execution_context.auth.managed_sign_in.payments_prod"),
        })
        .expect("managed outbound handoff stages cleanly")
    }

    #[test]
    fn account_free_local_packet_preserves_local_work_and_advertises_no_sign_in() {
        let handoff = baseline_local();
        let packet = handoff.packet();
        assert_eq!(packet.record_kind, BROWSER_CALLBACK_PACKET_RECORD_KIND);
        assert_eq!(
            packet.schema_version,
            BROWSER_CALLBACK_PACKET_SCHEMA_VERSION
        );
        assert!(packet.is_local_only_path());
        assert_eq!(packet.auth_flow_class, AuthFlowClass::NotApplicable);
        assert_eq!(packet.identity_mode, IdentityModeAlias::AccountFreeLocal);
        assert_eq!(packet.pending_session_state, PendingSessionState::Completed);
        assert_eq!(
            packet.account_boundary_class,
            AccountBoundaryClass::LocalOnly
        );
        assert!(packet.preserves_local_work());
        assert!(!packet.requires_visible_recovery());
        assert_eq!(
            packet.recovery_path.primary_recovery_action,
            RetryPathClass::ContinueLocalWithoutSignIn
        );
    }

    #[test]
    fn account_free_local_cannot_stage_outbound_handoff() {
        // Acceptance: the no-account local path is honored. Attempting to
        // stage an outbound system-browser handoff against `LocalOnly` fails
        // closed instead of silently widening the boundary into `Managed`.
        let err = BrowserCallbackHandoff::stage_system_browser_handoff(
            StageSystemBrowserHandoffRequest {
                packet_id: "should-not-stage",
                identity_mode: IdentityModeAlias::AccountFreeLocal,
                account_boundary_class: AccountBoundaryClass::LocalOnly,
                trust_state: TrustState::Trusted,
                provider_domain_label: "login.acme.example",
                destination_class_label: "Managed IdP",
                return_target_label: "Aureline desktop",
                return_anchor_ref: "return_anchor.test",
                return_mode_class: ReturnModeClass::LoopbackHttpReturn,
                return_origin_validation_class: ReturnOriginValidationClass::LoopbackPortPinned,
                return_tenant_or_workspace_match_rule:
                    ReturnTenantOrWorkspaceMatchRule::MustMatchBoundWorkspaceAndTenant,
                return_policy_check_refs: &[],
                bound_workspace_ref: Some("ws"),
                bound_tenant_or_org_ref: Some("tenant"),
                bound_actor_subject_ref: None,
                correlation_id: "corr",
                pending_session_id: "ps",
                state_token_alias: "state",
                nonce_alias: "nonce",
                pkce_challenge_alias: None,
                issued_at: "2026-04-23T10:00:00Z",
                expires_at: "2026-04-23T10:10:00Z",
                recovery_copy_label: "Continue sign-in in your browser.",
                primary_recovery_action: RetryPathClass::RetryInSystemBrowser,
                fallback_recovery_actions: &[],
                repair_hook_ref: None,
                preserved_local_work: PreservedLocalWork {
                    posture_class: PreservedLocalWorkPostureClass::LocalWorkIntact,
                    note: "Local work intact.".to_owned(),
                    retained_capabilities: Vec::new(),
                    blocked_capabilities: Vec::new(),
                },
                execution_context_ref: None,
            },
        )
        .unwrap_err();
        assert_eq!(
            err,
            BrowserCallbackValidationError::AccountFreeLocalCannotStageOutboundHandoff
        );
    }

    #[test]
    fn managed_outbound_handoff_uses_system_browser_and_forbids_embedded_fallback() {
        let handoff = baseline_managed_outbound();
        let packet = handoff.packet();
        assert!(packet.is_system_browser_handoff());
        assert_eq!(
            packet.embedded_fallback_posture,
            EmbeddedFallbackPosture::EmbeddedFallbackForbidden
        );
        assert_eq!(
            packet.browser_launch_policy_class,
            BrowserLaunchPolicyClass::SystemDefaultBrowserRequired
        );
        assert!(packet.recovery_path.local_continuity_offered());
        assert!(packet.requires_visible_recovery());
    }

    #[test]
    fn matching_returning_state_completes_pending_session() {
        let mut handoff = baseline_managed_outbound();
        handoff.redeem(ReturnedCallbackInputs {
            returning_state_token_alias: "state_alias.managed_sign_in.0001",
            returning_origin_validation_class: ReturnOriginValidationClass::LoopbackPortPinned,
            returning_tenant_or_org_ref: Some("tenant.acme_prod"),
            returning_workspace_ref: Some("workspace.payments_prod"),
            embedded_fallback_attempted: false,
            policy_blocked: false,
            observed_at: "2026-04-23T10:12:00Z",
        });
        let packet = handoff.packet();
        assert_eq!(packet.pending_session_state, PendingSessionState::Completed);
        assert!(packet.pending_session_denied_reason.is_none());
        assert!(!packet.recovery_path.visible_recovery_required);
    }

    #[test]
    fn embedded_fallback_attempt_fails_closed_without_widening_local_path() {
        // Failure drill: the host attempts a silent fallback to an embedded
        // webview. The packet denies the callback with a typed reason and
        // keeps preserved-local-work readable so the no-account path stays
        // truthful.
        let mut handoff = baseline_managed_outbound();
        let packet = handoff
            .redeem(ReturnedCallbackInputs {
                returning_state_token_alias: "state_alias.managed_sign_in.0001",
                returning_origin_validation_class: ReturnOriginValidationClass::LoopbackPortPinned,
                returning_tenant_or_org_ref: Some("tenant.acme_prod"),
                returning_workspace_ref: Some("workspace.payments_prod"),
                embedded_fallback_attempted: true,
                policy_blocked: false,
                observed_at: "2026-04-23T10:12:00Z",
            })
            .clone();
        assert_eq!(packet.pending_session_state, PendingSessionState::Denied);
        assert_eq!(
            packet.pending_session_denied_reason,
            Some(PendingSessionDeniedReason::CallbackEmbeddedFallbackAttempted)
        );
        assert!(packet.preserves_local_work());
        assert!(packet.recovery_path.local_continuity_offered());
    }

    #[test]
    fn tenant_mismatch_fails_closed_with_typed_denial() {
        let mut handoff = baseline_managed_outbound();
        handoff.redeem(ReturnedCallbackInputs {
            returning_state_token_alias: "state_alias.managed_sign_in.0001",
            returning_origin_validation_class: ReturnOriginValidationClass::LoopbackPortPinned,
            returning_tenant_or_org_ref: Some("tenant.other_org"),
            returning_workspace_ref: Some("workspace.payments_prod"),
            embedded_fallback_attempted: false,
            policy_blocked: false,
            observed_at: "2026-04-23T10:12:00Z",
        });
        let packet = handoff.packet();
        assert_eq!(packet.pending_session_state, PendingSessionState::Denied);
        assert_eq!(
            packet.pending_session_denied_reason,
            Some(PendingSessionDeniedReason::CallbackTenantOrWorkspaceMismatch)
        );
    }

    #[test]
    fn replay_or_state_mismatch_fails_closed() {
        let mut handoff = baseline_managed_outbound();
        handoff.redeem(ReturnedCallbackInputs {
            returning_state_token_alias: "state_alias.managed_sign_in.replayed",
            returning_origin_validation_class: ReturnOriginValidationClass::LoopbackPortPinned,
            returning_tenant_or_org_ref: Some("tenant.acme_prod"),
            returning_workspace_ref: Some("workspace.payments_prod"),
            embedded_fallback_attempted: false,
            policy_blocked: false,
            observed_at: "2026-04-23T10:12:00Z",
        });
        assert_eq!(
            handoff.packet().pending_session_denied_reason,
            Some(PendingSessionDeniedReason::CallbackReplayOrStateMismatch)
        );
    }

    #[test]
    fn shell_vocabulary_renders_local_only_chip_for_no_account_path() {
        let handoff = baseline_local();
        let chip = ShellAuthChip::from_packet(handoff.packet());
        assert_eq!(chip.vocabulary, ShellAuthVocabulary::AccountFreeLocal);
        assert_eq!(chip.chip_label, "Local only");
        assert!(chip.local_path_available);
        assert!(!chip.visible_recovery_required);
        assert_eq!(
            chip.primary_recovery_action,
            RetryPathClass::ContinueLocalWithoutSignIn
        );
    }

    #[test]
    fn shell_vocabulary_renders_managed_chip_after_completed_callback() {
        let mut handoff = baseline_managed_outbound();
        handoff.redeem(ReturnedCallbackInputs {
            returning_state_token_alias: "state_alias.managed_sign_in.0001",
            returning_origin_validation_class: ReturnOriginValidationClass::LoopbackPortPinned,
            returning_tenant_or_org_ref: Some("tenant.acme_prod"),
            returning_workspace_ref: Some("workspace.payments_prod"),
            embedded_fallback_attempted: false,
            policy_blocked: false,
            observed_at: "2026-04-23T10:12:00Z",
        });
        let chip = ShellAuthChip::from_packet(handoff.packet());
        assert_eq!(chip.vocabulary, ShellAuthVocabulary::SignedInManaged);
        assert_eq!(chip.chip_label, "Managed");
        assert!(chip.local_path_available);
        assert!(!chip.visible_recovery_required);
    }

    #[test]
    fn shell_vocabulary_renders_reauth_chip_when_managed_callback_is_pending() {
        let handoff = baseline_managed_outbound();
        let chip = ShellAuthChip::from_packet(handoff.packet());
        assert_eq!(chip.vocabulary, ShellAuthVocabulary::ReauthRequired);
        assert!(chip.visible_recovery_required);
        assert!(
            chip.local_path_available,
            "no-account local path stays usable"
        );
    }
}
