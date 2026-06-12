//! Governed managed-auth-and-recovery record for M5 surfaces.
//!
//! This module is the auth-boundary contract that brings the M5 managed
//! identity events — sign-in, step-up, re-auth, session revocation,
//! deprovision, and account recovery — into one inspectable, local-first
//! record instead of an opaque "signed-in / signed-out" toggle. It does not
//! perform any authentication; it defines the canonical record those flows emit
//! so the desktop shell, CLI inspect, docs/help, and support exports all answer
//! the same questions for an M5 auth event: who owns it (provider/org), how the
//! managed flow hands off (system browser, passkey posture, typed handoff
//! reason, keyboard-complete fallback), which managed capabilities a degraded
//! condition pauses, what local work stays usable through it, where refresh
//! credentials and delegated handles live, and how the row behaves under the
//! passkey-unavailable, browser-handoff-failure, offline-identity,
//! policy-forced-sign-out, and deprovision-on-active-local-work drills.
//!
//! The gate is fail-closed. Local durable work is always preserved: an event or
//! drill can never claim that loss of managed identity threatens unsaved local
//! work, local files, or local-only workflows unless a separately governed
//! policy row is named. A degraded condition can only pause managed
//! capabilities, never local ones, and must keep a keyboard-complete fallback.
//! On a claimed stable profile a recovery flow can never require embedded
//! password-first or CAPTCHA-only collection; where passkeys are unavailable the
//! fallback posture must be explicit. Refresh credentials and delegated handles
//! must live in an OS-backed or approved enterprise store and stay excluded from
//! portable-state, sync, and support export. Every event must disclose its
//! provider/owner. All of these are build-time invariants, so a record that
//! hides a local-work threat, an embedded-credential fallback, a leaked refresh
//! token, or an undisclosed managed owner behind a calm sign-in chip cannot be
//! constructed.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for M5 managed-auth-and-recovery records.
pub const M5_AUTH_AND_RECOVERY_RECORD_KIND: &str = "m5_auth_and_recovery_record";

/// Schema version for [`M5AuthAndRecovery`] records.
pub const M5_AUTH_AND_RECOVERY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by shell, CLI, docs/help, and support.
pub const M5_AUTH_AND_RECOVERY_SHARED_CONTRACT_REF: &str = "auth:m5_auth_and_recovery:v1";

const MAX_REF_CHARS: usize = 240;
const CANONICAL_OBJECT_SCHEME: &str = "aureline://";

/// Returns true when `reference` is a non-empty canonical object ref.
pub fn is_canonical_object_ref(reference: &str) -> bool {
    let reference = reference.trim();
    if reference.is_empty() || reference.len() > MAX_REF_CHARS {
        return false;
    }
    let Some(rest) = reference.strip_prefix(CANONICAL_OBJECT_SCHEME) else {
        return false;
    };
    let Some((class, ident)) = rest.split_once('/') else {
        return false;
    };
    !class.is_empty() && !ident.is_empty()
}

/// A managed identity event the contract requires the record to disclose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthEventKind {
    /// Initial managed sign-in via the system browser.
    ManagedSignIn,
    /// Step-up challenge for a higher-assurance managed action.
    StepUp,
    /// Silent or interactive re-authentication of an existing session.
    #[serde(rename = "reauth")]
    ReAuth,
    /// Server- or policy-driven revocation of the active session.
    SessionRevocation,
    /// Org deprovisioning that removes managed seat/entitlements.
    Deprovision,
    /// Account-recovery flow when the primary factor is unavailable.
    AccountRecovery,
}

impl AuthEventKind {
    /// Returns the canonical token for this event kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedSignIn => "managed_sign_in",
            Self::StepUp => "step_up",
            Self::ReAuth => "reauth",
            Self::SessionRevocation => "session_revocation",
            Self::Deprovision => "deprovision",
            Self::AccountRecovery => "account_recovery",
        }
    }

    /// Every managed event the record is required to disclose.
    pub const REQUIRED: [Self; 6] = [
        Self::ManagedSignIn,
        Self::StepUp,
        Self::ReAuth,
        Self::SessionRevocation,
        Self::Deprovision,
        Self::AccountRecovery,
    ];
}

/// The M5 surface an auth event is rendered on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthSurface {
    /// The primary desktop shell.
    Desktop,
    /// A paired companion device surface.
    Companion,
    /// A browser-adjacent path (system-browser handoff / web return).
    BrowserAdjacent,
}

impl AuthSurface {
    /// Returns the canonical token for this surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Companion => "companion",
            Self::BrowserAdjacent => "browser_adjacent",
        }
    }

    /// Every surface the record must represent across its events.
    pub const REQUIRED: [Self; 3] = [Self::Desktop, Self::Companion, Self::BrowserAdjacent];
}

/// How a managed flow hands off to obtain or refresh identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffMethod {
    /// System browser with a platform passkey (preferred).
    SystemBrowserPasskey,
    /// System browser with a hardware security key.
    SystemBrowserSecurityKey,
    /// System browser device-code entry (keyboard-complete).
    SystemBrowserDeviceCode,
    /// System browser federated password page (non-embedded).
    SystemBrowserFederatedPassword,
}

impl HandoffMethod {
    /// Returns the canonical token for this handoff method.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemBrowserPasskey => "system_browser_passkey",
            Self::SystemBrowserSecurityKey => "system_browser_security_key",
            Self::SystemBrowserDeviceCode => "system_browser_device_code",
            Self::SystemBrowserFederatedPassword => "system_browser_federated_password",
        }
    }
}

/// Typed reason a managed flow launches a browser handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffReason {
    /// Interactive initial sign-in.
    InteractiveSignIn,
    /// Higher-assurance step-up challenge.
    StepUpChallenge,
    /// Policy- or expiry-driven re-authentication.
    #[serde(rename = "policy_reauth")]
    PolicyReAuth,
    /// Account-recovery flow.
    RecoveryFlow,
}

impl HandoffReason {
    /// Returns the canonical token for this handoff reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InteractiveSignIn => "interactive_sign_in",
            Self::StepUpChallenge => "step_up_challenge",
            Self::PolicyReAuth => "policy_reauth",
            Self::RecoveryFlow => "recovery_flow",
        }
    }
}

/// Posture of platform passkeys for an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PasskeyPosture {
    /// Passkeys are preferred and offered first.
    Preferred,
    /// Passkeys are available as one option.
    Available,
    /// Passkeys are unavailable; an explicit fallback posture is named.
    UnavailableFallbackExplicit,
    /// Passkeys are unavailable and no explicit fallback is named (rejected).
    UnavailableFallbackImplicit,
}

impl PasskeyPosture {
    /// Returns the canonical token for this passkey posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preferred => "preferred",
            Self::Available => "available",
            Self::UnavailableFallbackExplicit => "unavailable_fallback_explicit",
            Self::UnavailableFallbackImplicit => "unavailable_fallback_implicit",
        }
    }

    /// Returns true when passkeys are unavailable for this posture.
    pub const fn is_unavailable(self) -> bool {
        matches!(
            self,
            Self::UnavailableFallbackExplicit | Self::UnavailableFallbackImplicit
        )
    }

    /// Returns true when the fallback posture is explicitly named.
    pub const fn fallback_is_explicit(self) -> bool {
        !matches!(self, Self::UnavailableFallbackImplicit)
    }
}

/// The fallback collection method used when a passkey or handoff is unavailable.
///
/// [`Self::EmbeddedPasswordFirst`] and [`Self::CaptchaOnly`] are disallowed on
/// claimed stable profiles: a recovery flow there can never require them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackPosture {
    /// Re-authenticate with a platform passkey in the system browser.
    SystemBrowserPasskey,
    /// Fall back to a hardware security key in the system browser.
    SystemBrowserSecurityKey,
    /// Fall back to keyboard-complete device-code entry.
    SystemBrowserDeviceCode,
    /// Fall back to a federated password page in the system browser.
    SystemBrowserFederatedPassword,
    /// Fall back to a one-time recovery code.
    RecoveryCode,
    /// Embedded password-first collection inside the app (disallowed on stable).
    EmbeddedPasswordFirst,
    /// CAPTCHA-only recovery with no other factor (disallowed on stable).
    CaptchaOnly,
}

impl FallbackPosture {
    /// Returns the canonical token for this fallback posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemBrowserPasskey => "system_browser_passkey",
            Self::SystemBrowserSecurityKey => "system_browser_security_key",
            Self::SystemBrowserDeviceCode => "system_browser_device_code",
            Self::SystemBrowserFederatedPassword => "system_browser_federated_password",
            Self::RecoveryCode => "recovery_code",
            Self::EmbeddedPasswordFirst => "embedded_password_first",
            Self::CaptchaOnly => "captcha_only",
        }
    }

    /// Returns true when this posture is forbidden on a claimed stable profile.
    pub const fn is_embedded_or_captcha(self) -> bool {
        matches!(self, Self::EmbeddedPasswordFirst | Self::CaptchaOnly)
    }
}

/// The install channel a record describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileChannel {
    /// A claimed stable install.
    Stable,
    /// A preview / insiders install.
    Preview,
    /// A portable install.
    Portable,
    /// A centrally managed / enrolled install.
    Managed,
}

impl ProfileChannel {
    /// Returns the canonical token for this profile channel.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Preview => "preview",
            Self::Portable => "portable",
            Self::Managed => "managed",
        }
    }

    /// Returns true when the no-embedded-recovery guarantee applies.
    pub const fn forbids_embedded_recovery(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// A managed capability a degraded condition can pause.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedCapabilityClass {
    /// Hosted AI / managed inference.
    HostedAi,
    /// Managed settings/device sync.
    ManagedSync,
    /// Marketplace publication.
    MarketplacePublish,
    /// Companion device control.
    CompanionControl,
    /// Org policy distribution / managed entitlements.
    PolicyDistribution,
    /// Org collaboration sessions.
    OrgCollab,
}

impl ManagedCapabilityClass {
    /// Returns the canonical token for this managed capability.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostedAi => "hosted_ai",
            Self::ManagedSync => "managed_sync",
            Self::MarketplacePublish => "marketplace_publish",
            Self::CompanionControl => "companion_control",
            Self::PolicyDistribution => "policy_distribution",
            Self::OrgCollab => "org_collab",
        }
    }
}

/// A local capability that must remain usable through a managed-identity event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCapabilityClass {
    /// Editing open buffers, including unsaved work.
    LocalEditing,
    /// Reading and writing local files on disk.
    LocalFiles,
    /// Local edit/run history.
    LocalHistory,
    /// Fully local-only workflows (no managed dependency).
    LocalOnlyWorkflows,
    /// Bring-your-own-key provider lanes.
    ByokProviders,
}

impl LocalCapabilityClass {
    /// Returns the canonical token for this local capability.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalEditing => "local_editing",
            Self::LocalFiles => "local_files",
            Self::LocalHistory => "local_history",
            Self::LocalOnlyWorkflows => "local_only_workflows",
            Self::ByokProviders => "byok_providers",
        }
    }
}

/// A degraded condition the record drills, and that an event can carry live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillKind {
    /// No passkey is available on this device/profile.
    PasskeyUnavailable,
    /// The system-browser handoff failed to launch or return.
    BrowserHandoffFailure,
    /// Identity verification is offline / unreachable.
    OfflineIdentity,
    /// Policy forced the active session to sign out.
    PolicyForcedSignOut,
    /// The seat was deprovisioned while local work was active.
    DeprovisionOnActiveLocalWork,
}

impl DrillKind {
    /// Returns the canonical token for this drill kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PasskeyUnavailable => "passkey_unavailable",
            Self::BrowserHandoffFailure => "browser_handoff_failure",
            Self::OfflineIdentity => "offline_identity",
            Self::PolicyForcedSignOut => "policy_forced_sign_out",
            Self::DeprovisionOnActiveLocalWork => "deprovision_on_active_local_work",
        }
    }

    /// Every drill the record is required to demonstrate.
    pub const REQUIRED: [Self; 5] = [
        Self::PasskeyUnavailable,
        Self::BrowserHandoffFailure,
        Self::OfflineIdentity,
        Self::PolicyForcedSignOut,
        Self::DeprovisionOnActiveLocalWork,
    ];
}

/// The review lens a drill exercises.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillCategory {
    /// Security review lens.
    Security,
    /// Accessibility (keyboard-complete) review lens.
    Accessibility,
    /// Recovery / continuity review lens.
    Recovery,
}

impl DrillCategory {
    /// Returns the canonical token for this drill category.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Security => "security",
            Self::Accessibility => "accessibility",
            Self::Recovery => "recovery",
        }
    }

    /// Every category the drill set must collectively cover.
    pub const REQUIRED: [Self; 3] = [Self::Security, Self::Accessibility, Self::Recovery];
}

/// How a live degraded condition resolves for an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConditionDisposition {
    /// Managed capability paused; local work preserved and the row labeled.
    ManagedPausedLocalPreserved,
    /// A keyboard-complete recovery path is offered for the condition.
    RecoveryOffered,
    /// Managed seat removed; local work and exports preserved.
    ManagedExitLocalPreserved,
}

impl ConditionDisposition {
    /// Returns the canonical token for this disposition.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedPausedLocalPreserved => "managed_paused_local_preserved",
            Self::RecoveryOffered => "recovery_offered",
            Self::ManagedExitLocalPreserved => "managed_exit_local_preserved",
        }
    }
}

/// A credential the record accounts for and excludes from export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialClass {
    /// A refresh token.
    RefreshToken,
    /// A delegated capability handle.
    DelegatedHandle,
    /// A session-only broker reference.
    SessionBroker,
}

impl CredentialClass {
    /// Returns the canonical token for this credential class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefreshToken => "refresh_token",
            Self::DelegatedHandle => "delegated_handle",
            Self::SessionBroker => "session_broker",
        }
    }

    /// Returns true when this credential must never leave the device.
    pub const fn must_be_export_excluded(self) -> bool {
        matches!(
            self,
            Self::RefreshToken | Self::DelegatedHandle | Self::SessionBroker
        )
    }
}

/// Where a credential is held.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialStoreClass {
    /// An OS-backed keychain / credential store.
    OsKeychain,
    /// An approved enterprise secret store.
    EnterpriseStore,
    /// Session-only in-memory broker (never persisted).
    SessionBrokerMemory,
}

impl CredentialStoreClass {
    /// Returns the canonical token for this store class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsKeychain => "os_keychain",
            Self::EnterpriseStore => "enterprise_store",
            Self::SessionBrokerMemory => "session_broker_memory",
        }
    }

    /// Returns true when this store is OS-backed or an approved enterprise store.
    pub const fn is_protected_store(self) -> bool {
        matches!(
            self,
            Self::OsKeychain | Self::EnterpriseStore | Self::SessionBrokerMemory
        )
    }
}

/// Source surface that must render the same auth-and-recovery truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// Desktop shell account / auth surface.
    DesktopShell,
    /// CLI or headless inspect command.
    CliInspect,
    /// Docs and inline help.
    DocsHelp,
    /// Support bundle / support-center export.
    SupportExport,
}

impl SurfaceClass {
    /// Required surface set for parity.
    pub const REQUIRED: [Self; 4] = [
        Self::DesktopShell,
        Self::CliInspect,
        Self::DocsHelp,
        Self::SupportExport,
    ];
}

/// The local work that stays usable through a managed-identity event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalContinuityBlock {
    /// Editing open buffers (including unsaved work) stays available.
    pub local_editing_preserved: bool,
    /// Local file read/write stays available.
    pub local_files_preserved: bool,
    /// Local history stays available.
    pub local_history_preserved: bool,
    /// Fully local-only workflows stay available.
    pub local_only_workflows_preserved: bool,
    /// Human-readable statement of what stays local.
    pub statement: String,
}

impl LocalContinuityBlock {
    /// Returns true when every local capability is preserved.
    pub fn fully_preserved(&self) -> bool {
        self.local_editing_preserved
            && self.local_files_preserved
            && self.local_history_preserved
            && self.local_only_workflows_preserved
    }
}

/// The browser-handoff envelope for an auth event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserHandoff {
    /// Preferred handoff method.
    pub method: HandoffMethod,
    /// Typed reason the handoff launches.
    pub reason: HandoffReason,
    /// Canonical ref to the return route.
    pub return_route_ref: String,
    /// Whether a keyboard-complete fallback path exists (accessibility).
    pub keyboard_complete_fallback: bool,
}

/// A live degraded condition currently affecting an auth event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthCondition {
    /// The degraded condition.
    pub drill: DrillKind,
    /// Managed capabilities this condition pauses (never local ones).
    pub paused_capabilities: Vec<ManagedCapabilityClass>,
    /// Local capabilities that remain usable through the condition.
    pub local_capabilities_remaining: Vec<LocalCapabilityClass>,
    /// Explicit fallback posture for the condition.
    pub fallback_posture: FallbackPosture,
    /// Whether the fallback path is keyboard-complete.
    pub keyboard_complete_fallback: bool,
    /// Whether local durable work is threatened (must be false unless governed).
    pub local_work_threatened: bool,
    /// Canonical ref to the governed policy row that authorizes any local impact.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub governed_policy_exception_ref: Option<String>,
    /// How the condition resolves.
    pub disposition: ConditionDisposition,
    /// Human-readable explanation of the paused/preserved split.
    pub detail: String,
}

/// One managed identity event disclosure row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthEventRow {
    /// Stable event id.
    pub event_id: String,
    /// Managed event kind.
    pub kind: AuthEventKind,
    /// Surface the event renders on.
    pub surface: AuthSurface,
    /// Provider/owner label (required, non-empty).
    pub provider_label: String,
    /// Owning org label, when one applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_label: Option<String>,
    /// Canonical ref to the issuer/identity authority.
    pub issuer_ref: String,
    /// Browser-handoff envelope.
    pub handoff: BrowserHandoff,
    /// Passkey posture for the event.
    pub passkey_posture: PasskeyPosture,
    /// Local-work continuity for the event (always present).
    pub local_continuity: LocalContinuityBlock,
    /// Whether the managed identity is healthy (no live degraded condition).
    pub managed_healthy: bool,
    /// A live degraded condition affecting this event, when one applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_condition: Option<AuthCondition>,
    /// Human-readable explanation of the event row.
    pub detail: String,
}

impl AuthEventRow {
    /// Derives the continuity posture for this event from its inputs.
    pub fn continuity(&self) -> ContinuityCeiling {
        match &self.active_condition {
            None => ContinuityCeiling::LocalFirstFull,
            Some(condition) => {
                if condition.local_work_threatened
                    && condition.governed_policy_exception_ref.is_none()
                {
                    ContinuityCeiling::LocalContinuityAtRisk
                } else {
                    ContinuityCeiling::ManagedNarrowedLocalIntact
                }
            }
        }
    }
}

/// A credential-storage row proving where a credential lives and that it never
/// leaves the device.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialStorageRow {
    /// Stable store row id.
    pub store_id: String,
    /// Credential class.
    pub credential_class: CredentialClass,
    /// Store class the credential is held in.
    pub store_class: CredentialStoreClass,
    /// Whether the credential is excluded from portable-state export.
    pub excluded_from_portable_state: bool,
    /// Whether the credential is excluded from sync.
    pub excluded_from_sync: bool,
    /// Whether the credential is excluded from support export.
    pub excluded_from_support_export: bool,
    /// Human-readable statement of where the credential lives.
    pub detail: String,
}

impl CredentialStorageRow {
    /// Returns true when this row excludes the credential from every export lane.
    pub fn fully_export_excluded(&self) -> bool {
        self.excluded_from_portable_state
            && self.excluded_from_sync
            && self.excluded_from_support_export
    }
}

/// A degraded-state drill kept honest about local-work continuity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthDrill {
    /// Drill kind.
    pub kind: DrillKind,
    /// Review lenses the drill exercises.
    pub categories: Vec<DrillCategory>,
    /// Managed capabilities the drill pauses.
    pub paused_capabilities: Vec<ManagedCapabilityClass>,
    /// Whether local durable work stays preserved (always true).
    pub local_preserved: bool,
    /// Whether the degraded state is visibly labeled (always true).
    pub local_labeled: bool,
    /// Whether the recovery path is keyboard-complete (always true).
    pub keyboard_complete: bool,
    /// The observable signal the drill expects.
    pub expected_signal: String,
    /// The recovery path back to a healthy managed session.
    pub recovery_path: String,
}

/// Source surface parity row for auth-and-recovery truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceTruthRow {
    /// Surface class.
    pub surface_class: SurfaceClass,
    /// Whether the surface consumes this shared record.
    pub consumes_shared_record: bool,
    /// Whether the surface shows the provider/owner disclosure.
    pub shows_provider_disclosure: bool,
    /// Whether the surface shows the paused managed capabilities.
    pub shows_paused_capabilities: bool,
    /// Whether the surface shows the local-work continuity.
    pub shows_local_continuity: bool,
    /// Whether the surface shows the fallback posture.
    pub shows_fallback_posture: bool,
    /// Whether the surface shows the degraded-state drills.
    pub shows_drills: bool,
}

/// Derived continuity ceiling for one event or the whole record.
///
/// Ordered from [`Self::LocalFirstFull`] (best) to
/// [`Self::LocalContinuityAtRisk`] (worst). The record publishes the weakest
/// event continuity as its effective ceiling; a valid record can never reach
/// [`Self::LocalContinuityAtRisk`] because the build gate rejects a local-work
/// threat that is not governed by a named policy row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityCeiling {
    /// Managed identity healthy; full managed and local capability available.
    LocalFirstFull,
    /// A managed capability is paused, but all local work remains usable.
    ManagedNarrowedLocalIntact,
    /// Local work would be threatened (unconstructable for a valid record).
    LocalContinuityAtRisk,
}

impl ContinuityCeiling {
    /// Returns the canonical token for this continuity ceiling.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFirstFull => "local_first_full",
            Self::ManagedNarrowedLocalIntact => "managed_narrowed_local_intact",
            Self::LocalContinuityAtRisk => "local_continuity_at_risk",
        }
    }

    /// Continuity rank where `0` is best and higher values are weaker.
    pub const fn rank(self) -> u8 {
        match self {
            Self::LocalFirstFull => 0,
            Self::ManagedNarrowedLocalIntact => 1,
            Self::LocalContinuityAtRisk => 2,
        }
    }

    /// Returns true when managed identity is fully healthy.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::LocalFirstFull)
    }
}

/// Derived pillar verdicts for the auth-and-recovery contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthRecoveryPillars {
    /// Every required managed event kind is disclosed.
    pub events_complete: bool,
    /// Desktop, companion, and browser-adjacent surfaces are all represented.
    pub surfaces_represented: bool,
    /// Every event discloses its provider/owner.
    pub provider_disclosed: bool,
    /// Every event and drill preserves local durable work.
    pub local_first_preserved: bool,
    /// Fallback posture is explicit and keyboard-complete; no embedded/CAPTCHA on stable.
    pub fallback_posture_explicit: bool,
    /// Refresh credentials and delegated handles are export-excluded and OS/enterprise-backed.
    pub credentials_export_excluded: bool,
    /// Every drill keeps local preserved, labeled, and keyboard-complete; all kinds and categories present.
    pub drills_local_preserving: bool,
    /// All required surfaces render the same record.
    pub surface_truth_complete: bool,
}

/// Reason a record is narrowed below a calm, fully-healthy claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// One or more required managed event kinds are missing.
    EventsIncomplete,
    /// A required surface is not represented across the events.
    SurfacesIncomplete,
    /// An event does not disclose its provider/owner.
    ProviderUndisclosed,
    /// A local-work threat is present without a governed policy row.
    LocalFirstThreatened,
    /// A fallback posture is implicit or embedded/CAPTCHA on a stable profile.
    FallbackPostureImplicit,
    /// A protected credential is not export-excluded or not OS/enterprise-backed.
    CredentialExportLeak,
    /// A drill loses local authority, label, or keyboard completeness.
    DrillNotLocalPreserving,
    /// At least one event currently has a live degraded condition.
    ManagedCapabilityPaused,
    /// One or more surfaces omit required auth-and-recovery truth.
    SurfaceTruthIncomplete,
}

/// Public claim class derived from the auth-and-recovery evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthRecoveryClaim {
    /// Managed identity is calm: every event healthy and every pillar holds.
    LocalFirstManagedSafe,
    /// Resolution is sound but a managed capability is paused on at least one event.
    NarrowedManagedDegraded,
    /// A structural pillar failed; the record is not safely usable as-is.
    Unsupported,
}

/// Derived trust verdict for the whole record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthRecoveryQualification {
    /// Derived claim class.
    pub claim_class: AuthRecoveryClaim,
    /// Weakest event continuity across all events.
    pub effective_continuity_ceiling: ContinuityCeiling,
    /// Whether the record qualifies as calm, local-first, managed-safe.
    pub qualifies_local_first_safe: bool,
    /// Named narrowing reasons.
    pub narrowing_reasons: Vec<NarrowingReason>,
}

/// Input used to build an [`M5AuthAndRecovery`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AuthAndRecoveryInput {
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp for the record.
    pub as_of: String,
    /// Human-readable summary.
    pub summary: String,
    /// The install channel this record describes.
    pub profile_channel: ProfileChannel,
    /// Managed identity event disclosures.
    pub events: Vec<AuthEventRow>,
    /// Credential-storage rows.
    pub credential_stores: Vec<CredentialStorageRow>,
    /// Degraded-state drills.
    pub drills: Vec<AuthDrill>,
    /// Surface truth rows.
    pub surface_truth: Vec<SurfaceTruthRow>,
}

/// Canonical managed-auth-and-recovery record for M5 surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthAndRecovery {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp for the record.
    pub as_of: String,
    /// Human-readable summary.
    pub summary: String,
    /// The install channel this record describes.
    pub profile_channel: ProfileChannel,
    /// Managed identity event disclosures.
    pub events: Vec<AuthEventRow>,
    /// Credential-storage rows.
    pub credential_stores: Vec<CredentialStorageRow>,
    /// Degraded-state drills.
    pub drills: Vec<AuthDrill>,
    /// Surface truth rows.
    pub surface_truth: Vec<SurfaceTruthRow>,
    /// Event kinds disclosed by the record.
    pub event_kind_coverage: Vec<AuthEventKind>,
    /// Surfaces represented across the events.
    pub surface_coverage: Vec<AuthSurface>,
    /// Managed capabilities surfaced as paused by conditions or drills.
    pub paused_capability_coverage: Vec<ManagedCapabilityClass>,
    /// Credential classes accounted for.
    pub credential_class_coverage: Vec<CredentialClass>,
    /// Drill kinds demonstrated by the record.
    pub drill_coverage: Vec<DrillKind>,
    /// Drill categories collectively exercised.
    pub drill_category_coverage: Vec<DrillCategory>,
    /// Derived pillar verdicts.
    pub pillars: AuthRecoveryPillars,
    /// Derived trust qualification.
    pub trust_qualification: AuthRecoveryQualification,
}

/// Reasons a managed-auth-and-recovery record cannot be built.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// No event rows were supplied.
    NoEvents,
    /// A required managed event kind has no row.
    MissingEventKind {
        /// The event kind with no row.
        kind: AuthEventKind,
    },
    /// An event id is used by more than one row.
    DuplicateEvent {
        /// The duplicated event id.
        event_id: String,
    },
    /// A required surface is not represented across the events.
    MissingSurface {
        /// The unrepresented surface.
        surface: AuthSurface,
    },
    /// An event does not disclose its provider/owner.
    ProviderUndisclosed {
        /// The event id with no provider label.
        event_id: String,
    },
    /// A canonical ref field is invalid.
    NonCanonicalRef {
        /// The field carrying the invalid ref.
        field: &'static str,
        /// The offending value.
        value: String,
    },
    /// An event would threaten local durable work without a governed policy row.
    LocalWorkThreatened {
        /// The event id with the ungoverned local-work threat.
        event_id: String,
    },
    /// An event with an unavailable passkey names no explicit fallback posture.
    FallbackPostureImplicit {
        /// The event id with the implicit fallback.
        event_id: String,
    },
    /// A stable-profile event requires embedded password-first or CAPTCHA-only recovery.
    EmbeddedRecoveryRequiredOnStable {
        /// The event id requiring the forbidden recovery.
        event_id: String,
    },
    /// A degraded condition lacks a keyboard-complete fallback.
    ConditionNotKeyboardComplete {
        /// The event id with the inaccessible condition.
        event_id: String,
    },
    /// A degraded condition pauses no managed capability (an opaque condition).
    ConditionPausesNothing {
        /// The event id with the empty condition.
        event_id: String,
    },
    /// A protected credential is not excluded from every export lane.
    CredentialExportLeak {
        /// The store id leaking the credential.
        store_id: String,
    },
    /// A protected credential is not in an OS-backed or approved enterprise store.
    CredentialStoreNotProtected {
        /// The store id with the unprotected store.
        store_id: String,
    },
    /// A required credential class is missing from the record.
    MissingCredentialClass {
        /// The missing credential class.
        class: CredentialClass,
    },
    /// A drill loses local authority.
    DrillNotLocalPreserving {
        /// The offending drill kind.
        kind: DrillKind,
    },
    /// A drill is not keyboard-complete.
    DrillNotKeyboardComplete {
        /// The offending drill kind.
        kind: DrillKind,
    },
    /// A drill names no review category.
    DrillWithoutCategory {
        /// The offending drill kind.
        kind: DrillKind,
    },
    /// A required drill is missing from the record.
    MissingDrill {
        /// The missing drill kind.
        kind: DrillKind,
    },
    /// A required drill category is not covered by any drill.
    MissingDrillCategory {
        /// The missing drill category.
        category: DrillCategory,
    },
    /// A required surface row is missing.
    MissingSurfaceRow {
        /// The missing surface.
        surface: SurfaceClass,
    },
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoEvents => write!(f, "at least one auth event row is required"),
            Self::MissingEventKind { kind } => {
                write!(f, "missing managed event kind `{}`", kind.as_str())
            }
            Self::DuplicateEvent { event_id } => write!(f, "duplicated event id `{event_id}`"),
            Self::MissingSurface { surface } => {
                write!(f, "no event represents surface `{}`", surface.as_str())
            }
            Self::ProviderUndisclosed { event_id } => {
                write!(f, "event `{event_id}` must disclose its provider/owner")
            }
            Self::NonCanonicalRef { field, value } => {
                write!(f, "field `{field}` must be a canonical ref, got {value:?}")
            }
            Self::LocalWorkThreatened { event_id } => write!(
                f,
                "event `{event_id}` threatens local durable work without a governed policy row"
            ),
            Self::FallbackPostureImplicit { event_id } => write!(
                f,
                "event `{event_id}` has an unavailable passkey but no explicit fallback posture"
            ),
            Self::EmbeddedRecoveryRequiredOnStable { event_id } => write!(
                f,
                "event `{event_id}` requires embedded password-first or CAPTCHA-only recovery on a stable profile"
            ),
            Self::ConditionNotKeyboardComplete { event_id } => write!(
                f,
                "event `{event_id}` has a degraded condition without a keyboard-complete fallback"
            ),
            Self::ConditionPausesNothing { event_id } => write!(
                f,
                "event `{event_id}` has a degraded condition that pauses no managed capability"
            ),
            Self::CredentialExportLeak { store_id } => write!(
                f,
                "credential store `{store_id}` must be excluded from portable-state, sync, and support export"
            ),
            Self::CredentialStoreNotProtected { store_id } => write!(
                f,
                "credential store `{store_id}` must be OS-backed or an approved enterprise store"
            ),
            Self::MissingCredentialClass { class } => {
                write!(f, "missing credential class `{}`", class.as_str())
            }
            Self::DrillNotLocalPreserving { kind } => write!(
                f,
                "drill `{}` must keep local durable work preserved and labeled",
                kind.as_str()
            ),
            Self::DrillNotKeyboardComplete { kind } => write!(
                f,
                "drill `{}` must keep a keyboard-complete recovery path",
                kind.as_str()
            ),
            Self::DrillWithoutCategory { kind } => {
                write!(f, "drill `{}` must name at least one review category", kind.as_str())
            }
            Self::MissingDrill { kind } => write!(f, "missing drill `{}`", kind.as_str()),
            Self::MissingDrillCategory { category } => {
                write!(f, "missing drill category `{}`", category.as_str())
            }
            Self::MissingSurfaceRow { surface } => write!(f, "missing surface `{surface:?}`"),
        }
    }
}

impl std::error::Error for BuildError {}

fn require_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_canonical_object_ref(value) {
        Ok(())
    } else {
        Err(BuildError::NonCanonicalRef {
            field,
            value: value.to_owned(),
        })
    }
}

impl M5AuthAndRecovery {
    /// Builds a derived auth-and-recovery record from raw events, stores, and drills.
    ///
    /// Returns a [`BuildError`] when a structural invariant or a fail-closed
    /// guardrail is violated, so a record that hides a local-work threat, an
    /// embedded-credential fallback, a leaked refresh token, or an undisclosed
    /// managed owner behind a calm sign-in chip cannot be constructed.
    pub fn build(mut input: M5AuthAndRecoveryInput) -> Result<Self, BuildError> {
        if input.events.is_empty() {
            return Err(BuildError::NoEvents);
        }

        let mut seen_events = BTreeSet::new();
        let mut seen_kinds = BTreeSet::new();
        let mut seen_surfaces = BTreeSet::new();
        for event in &input.events {
            if !seen_events.insert(event.event_id.clone()) {
                return Err(BuildError::DuplicateEvent {
                    event_id: event.event_id.clone(),
                });
            }
            seen_kinds.insert(event.kind);
            seen_surfaces.insert(event.surface);

            if event.provider_label.trim().is_empty() {
                return Err(BuildError::ProviderUndisclosed {
                    event_id: event.event_id.clone(),
                });
            }
            require_ref("events.issuer_ref", &event.issuer_ref)?;
            require_ref(
                "events.handoff.return_route_ref",
                &event.handoff.return_route_ref,
            )?;

            // Where passkeys are unavailable the fallback posture must be explicit.
            if event.passkey_posture.is_unavailable()
                && !event.passkey_posture.fallback_is_explicit()
            {
                return Err(BuildError::FallbackPostureImplicit {
                    event_id: event.event_id.clone(),
                });
            }

            if let Some(condition) = &event.active_condition {
                // A degraded condition can only pause managed capabilities; it
                // must name at least one so the row is never opaque.
                if condition.paused_capabilities.is_empty() {
                    return Err(BuildError::ConditionPausesNothing {
                        event_id: event.event_id.clone(),
                    });
                }
                // Local durable work is never threatened unless a separately
                // governed policy row is named.
                if condition.local_work_threatened
                    && condition.governed_policy_exception_ref.is_none()
                {
                    return Err(BuildError::LocalWorkThreatened {
                        event_id: event.event_id.clone(),
                    });
                }
                if let Some(policy_ref) = &condition.governed_policy_exception_ref {
                    require_ref(
                        "events.active_condition.governed_policy_exception_ref",
                        policy_ref,
                    )?;
                }
                // Every degraded condition keeps a keyboard-complete fallback.
                if !condition.keyboard_complete_fallback {
                    return Err(BuildError::ConditionNotKeyboardComplete {
                        event_id: event.event_id.clone(),
                    });
                }
                // On a stable profile, recovery can never require embedded
                // password-first or CAPTCHA-only collection.
                if input.profile_channel.forbids_embedded_recovery()
                    && condition.fallback_posture.is_embedded_or_captcha()
                {
                    return Err(BuildError::EmbeddedRecoveryRequiredOnStable {
                        event_id: event.event_id.clone(),
                    });
                }
            }
        }

        for kind in AuthEventKind::REQUIRED {
            if !seen_kinds.contains(&kind) {
                return Err(BuildError::MissingEventKind { kind });
            }
        }
        for surface in AuthSurface::REQUIRED {
            if !seen_surfaces.contains(&surface) {
                return Err(BuildError::MissingSurface { surface });
            }
        }

        // Credential stores: protected credentials stay export-excluded and in a
        // protected store; every required credential class is accounted for.
        let mut seen_credential_classes = BTreeSet::new();
        for store in &input.credential_stores {
            if store.credential_class.must_be_export_excluded() && !store.fully_export_excluded() {
                return Err(BuildError::CredentialExportLeak {
                    store_id: store.store_id.clone(),
                });
            }
            if !store.store_class.is_protected_store() {
                return Err(BuildError::CredentialStoreNotProtected {
                    store_id: store.store_id.clone(),
                });
            }
            seen_credential_classes.insert(store.credential_class);
        }
        for class in [
            CredentialClass::RefreshToken,
            CredentialClass::DelegatedHandle,
            CredentialClass::SessionBroker,
        ] {
            if !seen_credential_classes.contains(&class) {
                return Err(BuildError::MissingCredentialClass { class });
            }
        }

        // Drills: each keeps local preserved, labeled, and keyboard-complete and
        // names a category; every required drill kind and category is present.
        let mut seen_drills = BTreeSet::new();
        let mut seen_categories = BTreeSet::new();
        for drill in &input.drills {
            if !drill.local_preserved || !drill.local_labeled {
                return Err(BuildError::DrillNotLocalPreserving { kind: drill.kind });
            }
            if !drill.keyboard_complete {
                return Err(BuildError::DrillNotKeyboardComplete { kind: drill.kind });
            }
            if drill.categories.is_empty() {
                return Err(BuildError::DrillWithoutCategory { kind: drill.kind });
            }
            seen_drills.insert(drill.kind);
            seen_categories.extend(drill.categories.iter().copied());
        }
        for kind in DrillKind::REQUIRED {
            if !seen_drills.contains(&kind) {
                return Err(BuildError::MissingDrill { kind });
            }
        }
        for category in DrillCategory::REQUIRED {
            if !seen_categories.contains(&category) {
                return Err(BuildError::MissingDrillCategory { category });
            }
        }

        let present_surfaces: BTreeSet<SurfaceClass> = input
            .surface_truth
            .iter()
            .map(|row| row.surface_class)
            .collect();
        for surface in SurfaceClass::REQUIRED {
            if !present_surfaces.contains(&surface) {
                return Err(BuildError::MissingSurfaceRow { surface });
            }
        }

        input.events.sort_by(|a, b| a.event_id.cmp(&b.event_id));
        input
            .credential_stores
            .sort_by(|a, b| a.store_id.cmp(&b.store_id));
        input.drills.sort_by_key(|drill| drill.kind);
        input.surface_truth.sort_by_key(|row| row.surface_class);

        let event_kind_coverage = collect_sorted(input.events.iter().map(|e| e.kind));
        let surface_coverage = collect_sorted(input.events.iter().map(|e| e.surface));
        let paused_capability_coverage = collect_sorted(
            input
                .events
                .iter()
                .filter_map(|e| e.active_condition.as_ref())
                .flat_map(|c| c.paused_capabilities.iter().copied())
                .chain(
                    input
                        .drills
                        .iter()
                        .flat_map(|d| d.paused_capabilities.iter().copied()),
                ),
        );
        let credential_class_coverage =
            collect_sorted(input.credential_stores.iter().map(|s| s.credential_class));
        let drill_coverage = collect_sorted(input.drills.iter().map(|d| d.kind));
        let drill_category_coverage = collect_sorted(
            input
                .drills
                .iter()
                .flat_map(|d| d.categories.iter().copied()),
        );

        let events_complete = AuthEventKind::REQUIRED
            .iter()
            .all(|kind| seen_kinds.contains(kind));
        let surfaces_represented = AuthSurface::REQUIRED
            .iter()
            .all(|surface| seen_surfaces.contains(surface));

        let provider_disclosed = input
            .events
            .iter()
            .all(|event| !event.provider_label.trim().is_empty());

        let local_first_preserved = input.events.iter().all(|event| {
            event.local_continuity.fully_preserved()
                && event.active_condition.as_ref().map_or(true, |condition| {
                    !condition.local_work_threatened
                        || condition.governed_policy_exception_ref.is_some()
                })
        }) && input.drills.iter().all(|drill| drill.local_preserved);

        let fallback_posture_explicit = input.events.iter().all(|event| {
            (!event.passkey_posture.is_unavailable()
                || event.passkey_posture.fallback_is_explicit())
                && event.active_condition.as_ref().map_or(true, |condition| {
                    condition.keyboard_complete_fallback
                        && !(input.profile_channel.forbids_embedded_recovery()
                            && condition.fallback_posture.is_embedded_or_captcha())
                })
        });

        let credentials_export_excluded = input.credential_stores.iter().all(|store| {
            (!store.credential_class.must_be_export_excluded() || store.fully_export_excluded())
                && store.store_class.is_protected_store()
        });

        let drills_local_preserving =
            input.drills.iter().all(|drill| {
                drill.local_preserved && drill.local_labeled && drill.keyboard_complete
            }) && DrillKind::REQUIRED.iter().all(|k| seen_drills.contains(k))
                && DrillCategory::REQUIRED
                    .iter()
                    .all(|c| seen_categories.contains(c));

        let surface_truth_complete = input.surface_truth.iter().all(|row| {
            row.consumes_shared_record
                && row.shows_provider_disclosure
                && row.shows_paused_capabilities
                && row.shows_local_continuity
                && row.shows_fallback_posture
                && row.shows_drills
        });

        let effective_continuity_ceiling = input
            .events
            .iter()
            .map(AuthEventRow::continuity)
            .max_by_key(|ceiling| ceiling.rank())
            .unwrap_or(ContinuityCeiling::LocalFirstFull);

        let pillars = AuthRecoveryPillars {
            events_complete,
            surfaces_represented,
            provider_disclosed,
            local_first_preserved,
            fallback_posture_explicit,
            credentials_export_excluded,
            drills_local_preserving,
            surface_truth_complete,
        };

        let mut narrowing_reasons = Vec::new();
        if !pillars.events_complete {
            narrowing_reasons.push(NarrowingReason::EventsIncomplete);
        }
        if !pillars.surfaces_represented {
            narrowing_reasons.push(NarrowingReason::SurfacesIncomplete);
        }
        if !pillars.provider_disclosed {
            narrowing_reasons.push(NarrowingReason::ProviderUndisclosed);
        }
        if !pillars.local_first_preserved {
            narrowing_reasons.push(NarrowingReason::LocalFirstThreatened);
        }
        if !pillars.fallback_posture_explicit {
            narrowing_reasons.push(NarrowingReason::FallbackPostureImplicit);
        }
        if !pillars.credentials_export_excluded {
            narrowing_reasons.push(NarrowingReason::CredentialExportLeak);
        }
        if !pillars.drills_local_preserving {
            narrowing_reasons.push(NarrowingReason::DrillNotLocalPreserving);
        }
        if !effective_continuity_ceiling.is_full() {
            narrowing_reasons.push(NarrowingReason::ManagedCapabilityPaused);
        }
        if !pillars.surface_truth_complete {
            narrowing_reasons.push(NarrowingReason::SurfaceTruthIncomplete);
        }

        let structural_ok = pillars.events_complete
            && pillars.surfaces_represented
            && pillars.provider_disclosed
            && pillars.local_first_preserved
            && pillars.fallback_posture_explicit
            && pillars.credentials_export_excluded
            && pillars.drills_local_preserving
            && pillars.surface_truth_complete;

        let qualifies_local_first_safe = structural_ok && effective_continuity_ceiling.is_full();

        let claim_class = if !structural_ok {
            AuthRecoveryClaim::Unsupported
        } else if qualifies_local_first_safe {
            AuthRecoveryClaim::LocalFirstManagedSafe
        } else {
            AuthRecoveryClaim::NarrowedManagedDegraded
        };

        let trust_qualification = AuthRecoveryQualification {
            claim_class,
            effective_continuity_ceiling,
            qualifies_local_first_safe,
            narrowing_reasons,
        };

        Ok(Self {
            record_kind: M5_AUTH_AND_RECOVERY_RECORD_KIND.to_owned(),
            schema_version: M5_AUTH_AND_RECOVERY_SCHEMA_VERSION,
            shared_contract_ref: M5_AUTH_AND_RECOVERY_SHARED_CONTRACT_REF.to_owned(),
            record_id: input.record_id,
            as_of: input.as_of,
            summary: input.summary,
            profile_channel: input.profile_channel,
            events: input.events,
            credential_stores: input.credential_stores,
            drills: input.drills,
            surface_truth: input.surface_truth,
            event_kind_coverage,
            surface_coverage,
            paused_capability_coverage,
            credential_class_coverage,
            drill_coverage,
            drill_category_coverage,
            pillars,
            trust_qualification,
        })
    }

    /// Renders a compact, export-safe support summary from the shared record.
    ///
    /// The lines carry typed states and counts only — never provider payloads,
    /// tokens, handles, hostnames, or org identifiers beyond the channel.
    pub fn support_export_lines(&self) -> Vec<String> {
        vec![
            format!("record_id: {}", self.record_id),
            format!("channel: {}", self.profile_channel.as_str()),
            format!("claim: {:?}", self.trust_qualification.claim_class),
            format!(
                "continuity_ceiling: {}",
                self.trust_qualification
                    .effective_continuity_ceiling
                    .as_str()
            ),
            format!("events: {}", self.events.len()),
            format!("surfaces: {}", self.surface_coverage.len()),
            format!(
                "paused_capabilities: {}",
                self.paused_capability_coverage.len()
            ),
            format!(
                "credential_classes: {}",
                self.credential_class_coverage.len()
            ),
            format!("drills: {}", self.drill_coverage.len()),
            format!("drill_categories: {}", self.drill_category_coverage.len()),
        ]
    }
}

fn collect_sorted<T: Ord>(values: impl Iterator<Item = T>) -> Vec<T> {
    values.collect::<BTreeSet<_>>().into_iter().collect()
}
