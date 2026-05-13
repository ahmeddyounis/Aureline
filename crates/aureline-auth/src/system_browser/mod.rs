//! System-browser defaulting for claimed identity rows.
//!
//! This module owns the alpha claimed-identity row used when a managed,
//! self-hosted, or provider-linked surface needs to explain how auth will
//! proceed. It builds on the lower-level [`crate::browser_callback`] packet:
//! claimed rows prefer system-browser auth when policy allows it, expose
//! device-code and stay-local alternatives where supported, and keep the
//! preserved-local-work block readable when auth is denied, expired, or
//! optional.

use serde::{Deserialize, Serialize};

pub use crate::browser_callback::{
    AccountBoundaryClass, AuthFlowClass, BrowserLaunchPolicyClass, EmbeddedFallbackPosture,
    IdentityModeAlias, PreservedLocalWork, RetryPathClass, TrustState,
};

/// Record-kind tag carried on serialized [`ClaimedIdentityRow`] payloads.
pub const CLAIMED_IDENTITY_ROW_RECORD_KIND: &str = "claimed_identity_system_browser_row_record";

/// Record-kind tag carried on serialized [`SystemBrowserAlphaPacket`] payloads.
pub const SYSTEM_BROWSER_ALPHA_PACKET_RECORD_KIND: &str =
    "system_browser_claimed_identity_packet_record";

/// Schema version of the claimed-identity row and packet payloads.
pub const SYSTEM_BROWSER_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Current auth state for a claimed identity row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimedIdentityStateClass {
    /// No account is required for this row.
    AccountFreeLocal,
    /// The row has a current scoped identity.
    AuthReady,
    /// A system-browser handoff is waiting for user completion.
    AwaitingSystemBrowser,
    /// A device-code fallback is waiting for user completion.
    AwaitingDeviceCode,
    /// The user or provider denied the requested auth scope.
    AuthDenied,
    /// The pending handoff or credential window expired.
    AuthExpired,
    /// Policy blocks the requested auth scope.
    PolicyBlocked,
    /// Browser launch is blocked while other fallbacks may remain available.
    BrowserLaunchBlocked,
    /// The identity provider or account has not been configured yet.
    NotConfigured,
}

impl ClaimedIdentityStateClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AccountFreeLocal => "account_free_local",
            Self::AuthReady => "auth_ready",
            Self::AwaitingSystemBrowser => "awaiting_system_browser",
            Self::AwaitingDeviceCode => "awaiting_device_code",
            Self::AuthDenied => "auth_denied",
            Self::AuthExpired => "auth_expired",
            Self::PolicyBlocked => "policy_blocked",
            Self::BrowserLaunchBlocked => "browser_launch_blocked",
            Self::NotConfigured => "not_configured",
        }
    }

    /// True when the state represents an auth problem or blocked managed path.
    pub const fn requires_visible_recovery(self) -> bool {
        matches!(
            self,
            Self::AwaitingSystemBrowser
                | Self::AwaitingDeviceCode
                | Self::AuthDenied
                | Self::AuthExpired
                | Self::PolicyBlocked
                | Self::BrowserLaunchBlocked
                | Self::NotConfigured
        )
    }

    /// True when the state is a completed or account-free posture.
    pub const fn is_settled(self) -> bool {
        matches!(self, Self::AccountFreeLocal | Self::AuthReady)
    }
}

/// Default action selected for a claimed identity row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimedIdentityDefaultActionClass {
    /// Launch the system browser to start or renew auth.
    OpenSystemBrowser,
    /// Show the device-code fallback flow.
    UseDeviceCode,
    /// Continue locally without sign-in.
    ContinueLocalWithoutSignIn,
    /// Show the policy or support detail because no auth path can proceed.
    InspectPolicyOrSupport,
    /// No auth action is required.
    NoAuthRequired,
}

impl ClaimedIdentityDefaultActionClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenSystemBrowser => "open_system_browser",
            Self::UseDeviceCode => "use_device_code",
            Self::ContinueLocalWithoutSignIn => "continue_local_without_sign_in",
            Self::InspectPolicyOrSupport => "inspect_policy_or_support",
            Self::NoAuthRequired => "no_auth_required",
        }
    }

    /// Short label suitable for shell rows and support exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenSystemBrowser => "Open system browser",
            Self::UseDeviceCode => "Use device code",
            Self::ContinueLocalWithoutSignIn => "Continue local",
            Self::InspectPolicyOrSupport => "Inspect policy or support",
            Self::NoAuthRequired => "No auth required",
        }
    }
}

/// Alternative action class offered alongside the default path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimedIdentityAlternativeClass {
    /// Retry or start auth in the system browser.
    SystemBrowser,
    /// Use device-code auth without embedded credential collection.
    DeviceCode,
    /// Keep working locally without signing in.
    StayLocal,
    /// Inspect policy or request an admin change.
    AdminPolicyReview,
    /// Export auth diagnostics or support evidence.
    SupportExport,
}

impl ClaimedIdentityAlternativeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemBrowser => "system_browser",
            Self::DeviceCode => "device_code",
            Self::StayLocal => "stay_local",
            Self::AdminPolicyReview => "admin_policy_review",
            Self::SupportExport => "support_export",
        }
    }

    /// Short label suitable for shell rows and support exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SystemBrowser => "System browser",
            Self::DeviceCode => "Device code",
            Self::StayLocal => "Stay local",
            Self::AdminPolicyReview => "Review policy",
            Self::SupportExport => "Export diagnostics",
        }
    }
}

/// Export-safe provider and org scope shown on a claimed identity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimedIdentityProviderScope {
    /// Human-readable provider or identity-provider label.
    pub provider_label: String,
    /// Provider domain label used as the anti-phishing cue.
    pub provider_domain_label: String,
    /// Human-readable org, tenant, workspace, or provider scope label.
    pub provider_scope_label: String,
    /// Opaque workspace ref bound to this auth row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_workspace_ref: Option<String>,
    /// Opaque tenant or org ref bound to this auth row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_tenant_or_org_ref: Option<String>,
    /// Opaque actor subject ref bound to this auth row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_actor_subject_ref: Option<String>,
}

/// Auth policy and fallback availability for a claimed identity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimedIdentityAuthPolicy {
    /// Browser launch policy from the lower-level callback packet.
    pub browser_launch_policy_class: BrowserLaunchPolicyClass,
    /// Embedded auth posture; claimed rows keep this forbidden.
    pub embedded_fallback_posture: EmbeddedFallbackPosture,
    /// Whether system-browser auth is supported for this row.
    pub system_browser_supported: bool,
    /// Whether a device-code or equivalent fallback is supported.
    pub device_code_supported: bool,
    /// Whether stay-local continuation is supported.
    pub stay_local_supported: bool,
}

/// Expiry and timeout disclosure for a claimed identity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimedIdentitySessionWindow {
    /// Issue timestamp for the current auth window, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<String>,
    /// Expiry timestamp for the auth or device-code window, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Plain-language expiry or timeout note rendered by shell rows.
    pub expiry_summary_label: String,
}

/// One offered alternative for a claimed identity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimedIdentityAuthAlternative {
    /// Alternative class.
    pub alternative_class: ClaimedIdentityAlternativeClass,
    /// Stable token for [`Self::alternative_class`].
    pub alternative_token: String,
    /// Human-readable action label.
    pub action_label: String,
    /// Whether the alternative is available right now.
    pub available: bool,
    /// Why this alternative is offered or unavailable.
    pub reason_label: String,
    /// Optional expiry for device-code or browser windows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Optional opaque handoff or support ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handoff_ref: Option<String>,
}

impl ClaimedIdentityAuthAlternative {
    fn new(
        alternative_class: ClaimedIdentityAlternativeClass,
        available: bool,
        reason_label: impl Into<String>,
        expires_at: Option<&str>,
        handoff_ref: Option<&str>,
    ) -> Self {
        Self {
            alternative_class,
            alternative_token: alternative_class.as_str().to_owned(),
            action_label: alternative_class.label().to_owned(),
            available,
            reason_label: reason_label.into(),
            expires_at: expires_at.map(str::to_owned),
            handoff_ref: handoff_ref.map(str::to_owned),
        }
    }
}

/// Local continuation posture attached to a claimed identity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimedIdentityLocalContinuation {
    /// Whether the stay-local path is available from this row.
    pub stay_local_available: bool,
    /// Retry action used for stay-local continuation.
    pub stay_local_action: RetryPathClass,
    /// Preserved-local-work block reused from the callback packet contract.
    pub preserved_local_work: PreservedLocalWork,
    /// Shell-facing local-continuity note.
    pub local_continuity_label: String,
}

impl ClaimedIdentityLocalContinuation {
    /// True when local work remains usable and the stay-local path is offered.
    pub fn local_work_available(&self) -> bool {
        self.stay_local_available && self.preserved_local_work.local_work_usable()
    }
}

/// Cross-surface refs tying the claimed row back to boundary and handoff truth.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ClaimedIdentityHandoffRefs {
    /// Ref to the lower-level auth callback packet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_callback_packet_ref: Option<String>,
    /// Ref to a browser-handoff packet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Ref to the native boundary handoff packet from the shell boundary lane.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native_boundary_handoff_ref: Option<String>,
    /// Ref to an embedded boundary card from the shell boundary lane.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embedded_boundary_card_ref: Option<String>,
    /// Ref to a managed-session state record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_session_state_ref: Option<String>,
}

/// Request used to mint a [`ClaimedIdentityRow`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageClaimedIdentityRowRequest<'a> {
    /// Stable row id.
    pub row_id: &'a str,
    /// Current auth state for the row.
    pub state_class: ClaimedIdentityStateClass,
    /// Account boundary class.
    pub account_boundary_class: AccountBoundaryClass,
    /// Identity mode from the execution-context lane.
    pub identity_mode: IdentityModeAlias,
    /// Workspace trust state.
    pub trust_state: TrustState,
    /// Provider or identity-provider label.
    pub provider_label: &'a str,
    /// Provider domain label.
    pub provider_domain_label: &'a str,
    /// Provider, tenant, org, or workspace scope label.
    pub provider_scope_label: &'a str,
    /// Bound workspace ref.
    pub bound_workspace_ref: Option<&'a str>,
    /// Bound tenant or org ref.
    pub bound_tenant_or_org_ref: Option<&'a str>,
    /// Bound actor subject ref.
    pub bound_actor_subject_ref: Option<&'a str>,
    /// Browser launch policy.
    pub browser_launch_policy_class: BrowserLaunchPolicyClass,
    /// Whether system-browser auth is supported.
    pub system_browser_supported: bool,
    /// Whether device-code fallback is supported.
    pub device_code_supported: bool,
    /// Whether stay-local fallback is supported.
    pub stay_local_supported: bool,
    /// Issued-at timestamp for the active auth window.
    pub issued_at: Option<&'a str>,
    /// Expiry timestamp for the active auth window.
    pub expires_at: Option<&'a str>,
    /// Expiry/timeout copy.
    pub expiry_summary_label: &'a str,
    /// Device-code expiry timestamp.
    pub device_code_expires_at: Option<&'a str>,
    /// Device-code flow ref.
    pub device_code_ref: Option<&'a str>,
    /// Local-continuity copy.
    pub local_continuity_label: &'a str,
    /// Preserved-local-work block.
    pub preserved_local_work: PreservedLocalWork,
    /// Callback packet ref.
    pub auth_callback_packet_ref: Option<&'a str>,
    /// Browser handoff packet ref.
    pub browser_handoff_packet_ref: Option<&'a str>,
    /// Native boundary handoff ref.
    pub native_boundary_handoff_ref: Option<&'a str>,
    /// Embedded boundary card ref.
    pub embedded_boundary_card_ref: Option<&'a str>,
    /// Managed session state ref.
    pub managed_session_state_ref: Option<&'a str>,
    /// Recovery copy.
    pub recovery_copy_label: &'a str,
    /// Primary recovery action override.
    pub primary_recovery_action: Option<RetryPathClass>,
    /// Support-export ref used when recovery is visible.
    pub support_export_ref: Option<&'a str>,
    /// Execution context ref.
    pub execution_context_ref: Option<&'a str>,
    /// Mint timestamp.
    pub minted_at: &'a str,
}

/// Errors raised while minting claimed identity rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClaimedIdentityRowError {
    /// The request did not provide a stable row id.
    EmptyRowId,
    /// A non-local row has no provider/domain/scope label.
    MissingProviderScope,
    /// A visible auth window omitted expiry or timeout copy.
    MissingExpiryDisclosure,
    /// The request would create an auth row with no local or device fallback.
    MissingContinuationFallback,
    /// An auth failure row did not preserve a usable stay-local path.
    MissingLocalContinuationAfterAuthFailure,
    /// The account-free-local row attempted to require browser auth.
    LocalOnlyCannotRequireAuth,
}

impl std::fmt::Display for ClaimedIdentityRowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyRowId => f.write_str("claimed identity row id cannot be empty"),
            Self::MissingProviderScope => {
                f.write_str("claimed identity row must name provider, domain, and scope")
            }
            Self::MissingExpiryDisclosure => {
                f.write_str("claimed identity row must disclose timeout or expiry")
            }
            Self::MissingContinuationFallback => {
                f.write_str("claimed identity row must expose device-code or stay-local fallback")
            }
            Self::MissingLocalContinuationAfterAuthFailure => {
                f.write_str("auth failure row must preserve a usable stay-local continuation")
            }
            Self::LocalOnlyCannotRequireAuth => {
                f.write_str("account-free-local row cannot require browser auth")
            }
        }
    }
}

impl std::error::Error for ClaimedIdentityRowError {}

/// Canonical claimed identity row for system-browser defaulting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimedIdentityRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Current auth state.
    pub state_class: ClaimedIdentityStateClass,
    /// Stable token for [`Self::state_class`].
    pub state_class_token: String,
    /// Account boundary class.
    pub account_boundary_class: AccountBoundaryClass,
    /// Stable token for [`Self::account_boundary_class`].
    pub account_boundary_class_token: String,
    /// Identity mode.
    pub identity_mode: IdentityModeAlias,
    /// Trust state.
    pub trust_state: TrustState,
    /// Provider and scope disclosure.
    pub provider_scope: ClaimedIdentityProviderScope,
    /// Auth policy and fallback support.
    pub auth_policy: ClaimedIdentityAuthPolicy,
    /// Expiry and timeout disclosure.
    pub session_window: ClaimedIdentitySessionWindow,
    /// Default action chosen by the resolver.
    pub default_action: ClaimedIdentityDefaultActionClass,
    /// Stable token for [`Self::default_action`].
    pub default_action_token: String,
    /// Human-readable default action label.
    pub default_action_label: String,
    /// Offered fallback alternatives.
    pub alternatives: Vec<ClaimedIdentityAuthAlternative>,
    /// Local continuation posture.
    pub local_continuation: ClaimedIdentityLocalContinuation,
    /// Refs to lower-level auth, boundary, and session records.
    pub handoff_refs: ClaimedIdentityHandoffRefs,
    /// Recovery copy rendered by shell rows.
    pub recovery_copy_label: String,
    /// Primary recovery action.
    pub primary_recovery_action: RetryPathClass,
    /// Stable token for [`Self::primary_recovery_action`].
    pub primary_recovery_action_token: String,
    /// Whether a visible recovery row is required.
    pub visible_recovery_required: bool,
    /// Optional execution-context ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_ref: Option<String>,
    /// Mint timestamp.
    pub minted_at: String,
}

impl ClaimedIdentityRow {
    /// Mint a claimed identity row and resolve its default/fallback posture.
    ///
    /// # Errors
    ///
    /// Returns [`ClaimedIdentityRowError`] when required provider/scope,
    /// expiry, or fallback fields are missing.
    pub fn stage(
        request: StageClaimedIdentityRowRequest<'_>,
    ) -> Result<Self, ClaimedIdentityRowError> {
        if request.row_id.trim().is_empty() {
            return Err(ClaimedIdentityRowError::EmptyRowId);
        }
        let local_only = request.account_boundary_class == AccountBoundaryClass::LocalOnly
            || request.state_class == ClaimedIdentityStateClass::AccountFreeLocal;
        if local_only
            && (request.system_browser_supported || request.device_code_supported)
            && request.state_class != ClaimedIdentityStateClass::AccountFreeLocal
        {
            return Err(ClaimedIdentityRowError::LocalOnlyCannotRequireAuth);
        }
        if !local_only
            && (request.provider_label.trim().is_empty()
                || request.provider_domain_label.trim().is_empty()
                || request.provider_scope_label.trim().is_empty())
        {
            return Err(ClaimedIdentityRowError::MissingProviderScope);
        }
        if !request.state_class.is_settled() && request.expiry_summary_label.trim().is_empty() {
            return Err(ClaimedIdentityRowError::MissingExpiryDisclosure);
        }

        let default_action = resolve_default_action(&request);
        let visible_recovery_required = request.state_class.requires_visible_recovery()
            || request
                .browser_launch_policy_class
                .requires_visible_recovery()
            || matches!(
                default_action,
                ClaimedIdentityDefaultActionClass::UseDeviceCode
                    | ClaimedIdentityDefaultActionClass::ContinueLocalWithoutSignIn
                    | ClaimedIdentityDefaultActionClass::InspectPolicyOrSupport
            );
        let alternatives = build_alternatives(&request, visible_recovery_required);
        let has_device_or_local = alternatives.iter().any(|alternative| {
            alternative.available
                && matches!(
                    alternative.alternative_class,
                    ClaimedIdentityAlternativeClass::DeviceCode
                        | ClaimedIdentityAlternativeClass::StayLocal
                )
        });
        if !local_only && !has_device_or_local {
            return Err(ClaimedIdentityRowError::MissingContinuationFallback);
        }
        let local_continuation_available =
            request.stay_local_supported && request.preserved_local_work.local_work_usable();
        if !local_only
            && matches!(
                request.state_class,
                ClaimedIdentityStateClass::AuthDenied
                    | ClaimedIdentityStateClass::AuthExpired
                    | ClaimedIdentityStateClass::PolicyBlocked
            )
            && !local_continuation_available
        {
            return Err(ClaimedIdentityRowError::MissingLocalContinuationAfterAuthFailure);
        }
        let primary_recovery_action = request.primary_recovery_action.unwrap_or_else(|| {
            primary_recovery_action(default_action, request.device_code_supported)
        });

        Ok(Self {
            record_kind: CLAIMED_IDENTITY_ROW_RECORD_KIND.to_owned(),
            schema_version: SYSTEM_BROWSER_ALPHA_SCHEMA_VERSION,
            row_id: request.row_id.to_owned(),
            state_class: request.state_class,
            state_class_token: request.state_class.as_str().to_owned(),
            account_boundary_class: request.account_boundary_class,
            account_boundary_class_token: request.account_boundary_class.as_str().to_owned(),
            identity_mode: request.identity_mode,
            trust_state: request.trust_state,
            provider_scope: ClaimedIdentityProviderScope {
                provider_label: request.provider_label.to_owned(),
                provider_domain_label: request.provider_domain_label.to_owned(),
                provider_scope_label: request.provider_scope_label.to_owned(),
                bound_workspace_ref: request.bound_workspace_ref.map(str::to_owned),
                bound_tenant_or_org_ref: request.bound_tenant_or_org_ref.map(str::to_owned),
                bound_actor_subject_ref: request.bound_actor_subject_ref.map(str::to_owned),
            },
            auth_policy: ClaimedIdentityAuthPolicy {
                browser_launch_policy_class: request.browser_launch_policy_class,
                embedded_fallback_posture: EmbeddedFallbackPosture::EmbeddedFallbackForbidden,
                system_browser_supported: request.system_browser_supported,
                device_code_supported: request.device_code_supported,
                stay_local_supported: request.stay_local_supported,
            },
            session_window: ClaimedIdentitySessionWindow {
                issued_at: request.issued_at.map(str::to_owned),
                expires_at: request.expires_at.map(str::to_owned),
                expiry_summary_label: request.expiry_summary_label.to_owned(),
            },
            default_action,
            default_action_token: default_action.as_str().to_owned(),
            default_action_label: default_action.label().to_owned(),
            alternatives,
            local_continuation: ClaimedIdentityLocalContinuation {
                stay_local_available: request.stay_local_supported,
                stay_local_action: RetryPathClass::ContinueLocalWithoutSignIn,
                preserved_local_work: request.preserved_local_work,
                local_continuity_label: request.local_continuity_label.to_owned(),
            },
            handoff_refs: ClaimedIdentityHandoffRefs {
                auth_callback_packet_ref: request.auth_callback_packet_ref.map(str::to_owned),
                browser_handoff_packet_ref: request.browser_handoff_packet_ref.map(str::to_owned),
                native_boundary_handoff_ref: request.native_boundary_handoff_ref.map(str::to_owned),
                embedded_boundary_card_ref: request.embedded_boundary_card_ref.map(str::to_owned),
                managed_session_state_ref: request.managed_session_state_ref.map(str::to_owned),
            },
            recovery_copy_label: request.recovery_copy_label.to_owned(),
            primary_recovery_action,
            primary_recovery_action_token: primary_recovery_action.as_str().to_owned(),
            visible_recovery_required,
            execution_context_ref: request.execution_context_ref.map(str::to_owned),
            minted_at: request.minted_at.to_owned(),
        })
    }

    /// True when the row chooses system-browser auth as the default action.
    pub fn defaults_to_system_browser(&self) -> bool {
        self.default_action == ClaimedIdentityDefaultActionClass::OpenSystemBrowser
    }

    /// True when the row exposes an available device-code alternative.
    pub fn has_device_code_alternative(&self) -> bool {
        self.alternatives.iter().any(|alternative| {
            alternative.available
                && alternative.alternative_class == ClaimedIdentityAlternativeClass::DeviceCode
        })
    }

    /// True when the row exposes an available stay-local alternative.
    pub fn has_stay_local_alternative(&self) -> bool {
        self.alternatives.iter().any(|alternative| {
            alternative.available
                && alternative.alternative_class == ClaimedIdentityAlternativeClass::StayLocal
        })
    }

    /// True when local work remains usable from this row.
    pub fn local_work_available(&self) -> bool {
        self.local_continuation.local_work_available()
    }

    /// True when the row would strand the user without a device-code or
    /// stay-local path. Auth failure rows require stay-local continuation
    /// because a retry-only path is not enough to preserve local work.
    pub fn dead_end_without_local_continuation(&self) -> bool {
        if matches!(
            self.state_class,
            ClaimedIdentityStateClass::AuthDenied
                | ClaimedIdentityStateClass::AuthExpired
                | ClaimedIdentityStateClass::PolicyBlocked
        ) {
            return !self.local_work_available();
        }
        !self.local_work_available()
            && !self.has_device_code_alternative()
            && !self.has_stay_local_alternative()
    }
}

/// Shell/support projection for a claimed identity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimedIdentitySurfaceRow {
    /// Stable row id.
    pub row_id: String,
    /// Current auth state token.
    pub state_class_token: String,
    /// Account boundary token.
    pub account_boundary_class_token: String,
    /// Provider label.
    pub provider_label: String,
    /// Provider domain label.
    pub provider_domain_label: String,
    /// Provider/org/workspace scope label.
    pub provider_scope_label: String,
    /// Expiry timestamp for the active auth window, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Plain-language expiry or timeout note.
    pub expiry_summary_label: String,
    /// Default action token.
    pub default_action_token: String,
    /// Default action label.
    pub default_action_label: String,
    /// Available alternative tokens.
    pub available_alternative_tokens: Vec<String>,
    /// Whether the row has a visible device-code fallback.
    pub device_code_available: bool,
    /// Whether the row has a visible stay-local fallback.
    pub stay_local_available: bool,
    /// Whether local work remains usable.
    pub local_work_available: bool,
    /// Local-continuity note.
    pub local_continuity_label: String,
    /// Retained local capabilities.
    pub retained_capabilities: Vec<String>,
    /// Managed/provider capabilities currently blocked or narrowed.
    pub blocked_capabilities: Vec<String>,
    /// Whether a visible recovery row is required.
    pub visible_recovery_required: bool,
    /// Recovery copy rendered by shell rows.
    pub recovery_copy_label: String,
    /// Primary recovery action token.
    pub primary_recovery_action_token: String,
    /// Whether the row would strand the user without local continuation.
    pub dead_end_without_local_continuation: bool,
    /// Ref to the lower-level auth callback packet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_callback_packet_ref: Option<String>,
    /// Ref to the native boundary handoff packet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native_boundary_handoff_ref: Option<String>,
    /// Ref to the managed session state packet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_session_state_ref: Option<String>,
    /// Optional execution-context ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_ref: Option<String>,
}

impl ClaimedIdentitySurfaceRow {
    /// Project a shell/support row from a [`ClaimedIdentityRow`].
    pub fn from_row(row: &ClaimedIdentityRow) -> Self {
        Self {
            row_id: row.row_id.clone(),
            state_class_token: row.state_class_token.clone(),
            account_boundary_class_token: row.account_boundary_class_token.clone(),
            provider_label: row.provider_scope.provider_label.clone(),
            provider_domain_label: row.provider_scope.provider_domain_label.clone(),
            provider_scope_label: row.provider_scope.provider_scope_label.clone(),
            expires_at: row.session_window.expires_at.clone(),
            expiry_summary_label: row.session_window.expiry_summary_label.clone(),
            default_action_token: row.default_action_token.clone(),
            default_action_label: row.default_action_label.clone(),
            available_alternative_tokens: row
                .alternatives
                .iter()
                .filter(|alternative| alternative.available)
                .map(|alternative| alternative.alternative_token.clone())
                .collect(),
            device_code_available: row.has_device_code_alternative(),
            stay_local_available: row.has_stay_local_alternative(),
            local_work_available: row.local_work_available(),
            local_continuity_label: row.local_continuation.local_continuity_label.clone(),
            retained_capabilities: row
                .local_continuation
                .preserved_local_work
                .retained_capabilities
                .clone(),
            blocked_capabilities: row
                .local_continuation
                .preserved_local_work
                .blocked_capabilities
                .clone(),
            visible_recovery_required: row.visible_recovery_required,
            recovery_copy_label: row.recovery_copy_label.clone(),
            primary_recovery_action_token: row.primary_recovery_action_token.clone(),
            dead_end_without_local_continuation: row.dead_end_without_local_continuation(),
            auth_callback_packet_ref: row.handoff_refs.auth_callback_packet_ref.clone(),
            native_boundary_handoff_ref: row.handoff_refs.native_boundary_handoff_ref.clone(),
            managed_session_state_ref: row.handoff_refs.managed_session_state_ref.clone(),
            execution_context_ref: row.execution_context_ref.clone(),
        }
    }
}

/// Packet grouping claimed identity rows for shell/support consumption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemBrowserAlphaPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed identity rows in stable render order.
    pub claimed_identity_rows: Vec<ClaimedIdentityRow>,
    /// Mint timestamp.
    pub minted_at: String,
}

impl SystemBrowserAlphaPacket {
    /// Build a packet from claimed identity rows.
    pub fn new(
        packet_id: impl Into<String>,
        claimed_identity_rows: Vec<ClaimedIdentityRow>,
        minted_at: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: SYSTEM_BROWSER_ALPHA_PACKET_RECORD_KIND.to_owned(),
            schema_version: SYSTEM_BROWSER_ALPHA_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            claimed_identity_rows,
            minted_at: minted_at.into(),
        }
    }

    /// Project all claimed identity rows into shell/support surface rows.
    pub fn surface_rows(&self) -> Vec<ClaimedIdentitySurfaceRow> {
        self.claimed_identity_rows
            .iter()
            .map(ClaimedIdentitySurfaceRow::from_row)
            .collect()
    }

    /// True when every claimed row exposes a device-code or stay-local path.
    pub fn all_claimed_rows_have_fallback(&self) -> bool {
        self.claimed_identity_rows.iter().all(|row| {
            row.account_boundary_class == AccountBoundaryClass::LocalOnly
                || row.has_device_code_alternative()
                || row.has_stay_local_alternative()
        })
    }

    /// True when no claimed row can strand the user without local continuation.
    pub fn prevents_dead_end_auth_failure(&self) -> bool {
        self.claimed_identity_rows
            .iter()
            .all(|row| !row.dead_end_without_local_continuation())
    }
}

fn resolve_default_action(
    request: &StageClaimedIdentityRowRequest<'_>,
) -> ClaimedIdentityDefaultActionClass {
    if request.account_boundary_class == AccountBoundaryClass::LocalOnly
        || request.state_class == ClaimedIdentityStateClass::AccountFreeLocal
        || request.state_class == ClaimedIdentityStateClass::AuthReady
    {
        return ClaimedIdentityDefaultActionClass::NoAuthRequired;
    }
    if matches!(
        request.state_class,
        ClaimedIdentityStateClass::AuthDenied | ClaimedIdentityStateClass::AuthExpired
    ) && request.stay_local_supported
    {
        return ClaimedIdentityDefaultActionClass::ContinueLocalWithoutSignIn;
    }
    if request
        .browser_launch_policy_class
        .requires_visible_recovery()
        || request.state_class == ClaimedIdentityStateClass::BrowserLaunchBlocked
        || !request.system_browser_supported
    {
        if request.device_code_supported {
            return ClaimedIdentityDefaultActionClass::UseDeviceCode;
        }
        if request.stay_local_supported {
            return ClaimedIdentityDefaultActionClass::ContinueLocalWithoutSignIn;
        }
        return ClaimedIdentityDefaultActionClass::InspectPolicyOrSupport;
    }
    if request.system_browser_supported {
        return ClaimedIdentityDefaultActionClass::OpenSystemBrowser;
    }
    if request.device_code_supported {
        return ClaimedIdentityDefaultActionClass::UseDeviceCode;
    }
    if request.stay_local_supported {
        return ClaimedIdentityDefaultActionClass::ContinueLocalWithoutSignIn;
    }
    ClaimedIdentityDefaultActionClass::InspectPolicyOrSupport
}

fn primary_recovery_action(
    default_action: ClaimedIdentityDefaultActionClass,
    device_code_supported: bool,
) -> RetryPathClass {
    match default_action {
        ClaimedIdentityDefaultActionClass::OpenSystemBrowser => {
            RetryPathClass::RetryInSystemBrowser
        }
        ClaimedIdentityDefaultActionClass::UseDeviceCode => RetryPathClass::SwitchToDeviceCode,
        ClaimedIdentityDefaultActionClass::ContinueLocalWithoutSignIn => {
            RetryPathClass::ContinueLocalWithoutSignIn
        }
        ClaimedIdentityDefaultActionClass::InspectPolicyOrSupport => {
            if device_code_supported {
                RetryPathClass::SwitchToDeviceCode
            } else {
                RetryPathClass::ContactSupportWithExport
            }
        }
        ClaimedIdentityDefaultActionClass::NoAuthRequired => {
            RetryPathClass::ContinueLocalWithoutSignIn
        }
    }
}

fn build_alternatives(
    request: &StageClaimedIdentityRowRequest<'_>,
    visible_recovery_required: bool,
) -> Vec<ClaimedIdentityAuthAlternative> {
    let mut alternatives = Vec::new();
    if request.system_browser_supported
        && request.account_boundary_class != AccountBoundaryClass::LocalOnly
    {
        alternatives.push(ClaimedIdentityAuthAlternative::new(
            ClaimedIdentityAlternativeClass::SystemBrowser,
            !request
                .browser_launch_policy_class
                .requires_visible_recovery(),
            "Use the platform-safe system-browser auth path.",
            request.expires_at,
            request.browser_handoff_packet_ref,
        ));
    }
    if request.device_code_supported {
        alternatives.push(ClaimedIdentityAuthAlternative::new(
            ClaimedIdentityAlternativeClass::DeviceCode,
            true,
            "Use device-code auth without embedded credential collection.",
            request.device_code_expires_at.or(request.expires_at),
            request.device_code_ref,
        ));
    }
    if request.stay_local_supported {
        alternatives.push(ClaimedIdentityAuthAlternative::new(
            ClaimedIdentityAlternativeClass::StayLocal,
            true,
            "Keep local files, unsaved edits, search, local Git, and local tasks available.",
            None,
            None,
        ));
    }
    if request
        .browser_launch_policy_class
        .requires_visible_recovery()
        || request.state_class == ClaimedIdentityStateClass::PolicyBlocked
    {
        alternatives.push(ClaimedIdentityAuthAlternative::new(
            ClaimedIdentityAlternativeClass::AdminPolicyReview,
            true,
            "Review the policy source that blocked this auth path.",
            None,
            request.native_boundary_handoff_ref,
        ));
    }
    if visible_recovery_required {
        alternatives.push(ClaimedIdentityAuthAlternative::new(
            ClaimedIdentityAlternativeClass::SupportExport,
            true,
            "Export metadata-safe auth diagnostics without raw secrets.",
            None,
            request.support_export_ref,
        ));
    }
    alternatives
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::browser_callback::PreservedLocalWorkPostureClass;

    fn preserved_local_work() -> PreservedLocalWork {
        PreservedLocalWork {
            posture_class: PreservedLocalWorkPostureClass::LocalWorkIntactWithManagedNarrowed,
            note: "Local work remains available while managed auth is incomplete.".to_owned(),
            retained_capabilities: vec![
                "Edit local files.".to_owned(),
                "Save local files.".to_owned(),
                "Use local Git.".to_owned(),
            ],
            blocked_capabilities: vec!["Managed settings sync waits for sign-in.".to_owned()],
        }
    }

    fn managed_request() -> StageClaimedIdentityRowRequest<'static> {
        StageClaimedIdentityRowRequest {
            row_id: "claimed-identity:managed:payments-prod",
            state_class: ClaimedIdentityStateClass::AwaitingSystemBrowser,
            account_boundary_class: AccountBoundaryClass::Managed,
            identity_mode: IdentityModeAlias::ManagedConvenience,
            trust_state: TrustState::Trusted,
            provider_label: "Acme identity",
            provider_domain_label: "login.acme.example",
            provider_scope_label: "payments-prod tenant",
            bound_workspace_ref: Some("workspace:payments-prod"),
            bound_tenant_or_org_ref: Some("tenant:acme-prod"),
            bound_actor_subject_ref: Some("actor-subject:sam.acme"),
            browser_launch_policy_class: BrowserLaunchPolicyClass::SystemDefaultBrowserRequired,
            system_browser_supported: true,
            device_code_supported: true,
            stay_local_supported: true,
            issued_at: Some("2026-05-13T08:00:00Z"),
            expires_at: Some("2026-05-13T08:10:00Z"),
            expiry_summary_label: "System-browser handoff expires in 10 minutes.",
            device_code_expires_at: Some("2026-05-13T08:15:00Z"),
            device_code_ref: Some("device-code:managed:payments-prod"),
            local_continuity_label: "Local files and unsaved edits remain available.",
            preserved_local_work: preserved_local_work(),
            auth_callback_packet_ref: Some("auth-callback:managed:payments-prod"),
            browser_handoff_packet_ref: Some("browser-handoff:managed:payments-prod"),
            native_boundary_handoff_ref: Some("native-handoff:auth-callback:payments-prod"),
            embedded_boundary_card_ref: None,
            managed_session_state_ref: Some("managed-session:payments-prod"),
            recovery_copy_label: "Continue sign-in in your browser or stay local.",
            primary_recovery_action: None,
            support_export_ref: Some("support-export:auth:payments-prod"),
            execution_context_ref: Some("execution-context:auth:payments-prod"),
            minted_at: "2026-05-13T08:00:01Z",
        }
    }

    #[test]
    fn managed_claim_prefers_system_browser_and_exposes_fallbacks() {
        let row = ClaimedIdentityRow::stage(managed_request()).expect("row stages");
        assert!(row.defaults_to_system_browser());
        assert!(row.has_device_code_alternative());
        assert!(row.has_stay_local_alternative());
        assert!(row.local_work_available());
        assert!(!row.dead_end_without_local_continuation());

        let surface = ClaimedIdentitySurfaceRow::from_row(&row);
        assert_eq!(surface.default_action_token, "open_system_browser");
        assert_eq!(surface.provider_domain_label, "login.acme.example");
        assert_eq!(surface.provider_scope_label, "payments-prod tenant");
        assert_eq!(surface.expires_at.as_deref(), Some("2026-05-13T08:10:00Z"));
        assert!(surface.device_code_available);
        assert!(surface.stay_local_available);
        assert!(surface.local_work_available);
    }

    #[test]
    fn browser_launch_block_prefers_device_code_then_stay_local() {
        let mut request = managed_request();
        request.state_class = ClaimedIdentityStateClass::BrowserLaunchBlocked;
        request.browser_launch_policy_class = BrowserLaunchPolicyClass::BrowserLaunchPolicyBlocked;
        let row = ClaimedIdentityRow::stage(request).expect("row stages");
        assert_eq!(
            row.default_action,
            ClaimedIdentityDefaultActionClass::UseDeviceCode
        );
        assert!(row.has_device_code_alternative());
        assert!(row.has_stay_local_alternative());
        assert!(row.visible_recovery_required);
        assert!(!row.dead_end_without_local_continuation());
    }

    #[test]
    fn denied_auth_defaults_to_local_continuation() {
        let mut request = managed_request();
        request.state_class = ClaimedIdentityStateClass::AuthDenied;
        let row = ClaimedIdentityRow::stage(request).expect("row stages");
        assert_eq!(
            row.default_action,
            ClaimedIdentityDefaultActionClass::ContinueLocalWithoutSignIn
        );
        assert_eq!(
            row.primary_recovery_action,
            RetryPathClass::ContinueLocalWithoutSignIn
        );
        assert!(row.visible_recovery_required);
        assert!(row.local_work_available());
        assert!(!row.dead_end_without_local_continuation());
    }

    #[test]
    fn missing_fallback_is_rejected_for_claimed_rows() {
        let mut request = managed_request();
        request.device_code_supported = false;
        request.stay_local_supported = false;
        let err = ClaimedIdentityRow::stage(request).unwrap_err();
        assert_eq!(err, ClaimedIdentityRowError::MissingContinuationFallback);
    }

    #[test]
    fn auth_failure_without_local_continuation_is_rejected() {
        let mut request = managed_request();
        request.state_class = ClaimedIdentityStateClass::AuthExpired;
        request.stay_local_supported = false;
        let err = ClaimedIdentityRow::stage(request).unwrap_err();
        assert_eq!(
            err,
            ClaimedIdentityRowError::MissingLocalContinuationAfterAuthFailure
        );
    }

    #[test]
    fn packet_projects_surface_rows_and_blocks_dead_ends() {
        let row = ClaimedIdentityRow::stage(managed_request()).expect("row stages");
        let packet = SystemBrowserAlphaPacket::new(
            "system-browser-alpha:claimed-identity",
            vec![row],
            "2026-05-13T08:00:01Z",
        );
        assert!(packet.all_claimed_rows_have_fallback());
        assert!(packet.prevents_dead_end_auth_failure());
        let rows = packet.surface_rows();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].default_action_token, "open_system_browser");
    }
}
