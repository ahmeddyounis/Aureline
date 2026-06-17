//! Registry authentication flows, browser/device-code continuity, OS-store and
//! vault handle use, and mirror/offline/cache-only degradation truth for the M5
//! package-mutation lane.
//!
//! Where
//! [`crate::freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix`]
//! *freezes the registry-source and auth-mode vocabulary* and
//! [`crate::manifest_scope_and_source_review`] *names the source behind one
//! mutation*, this module makes the **authentication flow itself** a first-class
//! product object: which profile is current, which provider or mirror it reaches,
//! whether the credential comes from a browser/device-code sign-in or an OS-store
//! or vault handle, what retry/revoke/switch-account/rebind actions are available,
//! and whether the registry is reachable or degraded into a mirror-stale,
//! offline-snapshot, cache-only, auth-required, or policy-blocked state. One
//! [`RegistryAuthFlowRow`] is the object the desktop package workspace,
//! CLI/headless surface, AI inspect context, review workspace, and support/export
//! packets all reuse, so registry auth stops being an undocumented prerequisite
//! hidden inside a per-ecosystem adapter.
//!
//! Three properties hold by construction and are validated against the frozen
//! matrix:
//!
//! 1. **Secrets stay handle-only.** A [`SecretHandle`] carries an opaque handle
//!    ref, a redacted account label, and a [`HandleState`]; it never carries a
//!    token body, a private registry URL, or a full auth payload, and its
//!    retention is always [`RetentionClass::BrokerResolvedNeverPersisted`]. The
//!    `stores_secret_body` guard is recomputed and must stay `false`.
//! 2. **Degradation never collapses into a generic message.** Each row's
//!    [`DegradationState`] maps to a *specific* [`RegistryStatusMessageClass`];
//!    the mirror-stale, offline-snapshot, cache-only, auth-required, and
//!    policy-blocked states are guarded so they can never render as the forbidden
//!    `generic_no_results` or `generic_connection_failed` messages the closed
//!    vocabulary names only to forbid. An authoritative *no results* is itself a
//!    specific, non-degraded outcome distinct from any connection failure.
//! 3. **Auth flows are keyboard-complete and self-describing.** Every row offers
//!    at least the [`AuthActionKind`]s its credential source, continuity state,
//!    and reachability require — a revocable handle always offers revoke and
//!    switch-account; an unsatisfied auth always offers a matched sign-in or
//!    rebind — and every offered action carries a command id and key hint so the
//!    flow is reachable without a pointer.
//!
//! Because every row binds to the frozen matrix through `references_matrix_id`,
//! its source class resolves to a frozen registry cell, and its bound auth/source
//! labels resolve to frozen state rows, product, CLI, and support/export paths
//! express registry identity, auth posture, and degradation truth mechanically
//! rather than by hand.
//!
//! The packet is checked in at `artifacts/deps/m5/registry-auth-flows.json` and
//! embedded here, so this typed consumer and any CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque,
//! redacted ref. It carries no credential bodies, registry tokens, raw provider
//! payloads, or private registry URLs.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_package_state_manifest_scope_registry_auth_and_lockfile_authority_matrix::{
    current_m5_package_state_matrix, AuthMode, PackageStateLabel, PackageSurface,
    RegistrySourceAuthority, RetentionClass, SurfaceWriteAuthority,
};

/// Supported registry-auth-flows packet schema version.
pub const REGISTRY_AUTH_FLOWS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const REGISTRY_AUTH_FLOWS_RECORD_KIND: &str = "registry_auth_flows";

/// Repo-relative path to the checked-in packet.
pub const REGISTRY_AUTH_FLOWS_PATH: &str = "artifacts/deps/m5/registry-auth-flows.json";

/// Embedded checked-in packet JSON.
pub const REGISTRY_AUTH_FLOWS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/deps/m5/registry-auth-flows.json"
));

/// Where a registry credential comes from and how it is presented.
///
/// This is the browser/device-code-versus-OS-store/vault distinction made
/// explicit, finer-grained than the frozen [`AuthMode`] it maps to: the frozen
/// mode names the *mechanism* a resolver sees (browser/device sign-in, OS store,
/// token, policy, anonymous, unsatisfied); the source class names the *product
/// flow* a user drives, so a browser sign-in is never confused with a device-code
/// continuity flow and a keychain handle is never confused with a vault handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialSourceClass {
    /// Interactive browser sign-in (OAuth/SSO) on this device.
    BrowserInteractive,
    /// Device-code continuity: a code is entered on a second device.
    DeviceCodeContinuity,
    /// A handle into the OS keychain or credential store.
    OsKeychainHandle,
    /// A handle into an external secret vault.
    SecretVaultHandle,
    /// A credential provided by org policy through the secret broker.
    PolicyBrokerHandle,
    /// Anonymous access; no credential is presented.
    AnonymousAccess,
    /// Auth is required but no credential is bound yet.
    AuthUnsatisfied,
}

impl CredentialSourceClass {
    /// Every credential source class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::BrowserInteractive,
        Self::DeviceCodeContinuity,
        Self::OsKeychainHandle,
        Self::SecretVaultHandle,
        Self::PolicyBrokerHandle,
        Self::AnonymousAccess,
        Self::AuthUnsatisfied,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BrowserInteractive => "browser_interactive",
            Self::DeviceCodeContinuity => "device_code_continuity",
            Self::OsKeychainHandle => "os_keychain_handle",
            Self::SecretVaultHandle => "secret_vault_handle",
            Self::PolicyBrokerHandle => "policy_broker_handle",
            Self::AnonymousAccess => "anonymous_access",
            Self::AuthUnsatisfied => "auth_unsatisfied",
        }
    }

    /// The frozen [`AuthMode`] this source maps to for matrix binding.
    pub const fn frozen_auth_mode(self) -> AuthMode {
        match self {
            Self::BrowserInteractive | Self::DeviceCodeContinuity => {
                AuthMode::BrowserOrDeviceSignIn
            }
            Self::OsKeychainHandle => AuthMode::OsStoreCredential,
            Self::SecretVaultHandle => AuthMode::TokenCredential,
            Self::PolicyBrokerHandle => AuthMode::PolicyInheritedCredential,
            Self::AnonymousAccess => AuthMode::Anonymous,
            Self::AuthUnsatisfied => AuthMode::AuthRequiredUnsatisfied,
        }
    }

    /// Whether this source keeps its secret as an opaque, broker-resolved handle.
    ///
    /// Anonymous access and an unsatisfied auth hold no secret at all.
    pub const fn is_handle_backed(self) -> bool {
        matches!(
            self,
            Self::BrowserInteractive
                | Self::DeviceCodeContinuity
                | Self::OsKeychainHandle
                | Self::SecretVaultHandle
                | Self::PolicyBrokerHandle
        )
    }

    /// Whether this source drives a browser/device-code continuity flow.
    pub const fn requires_continuity(self) -> bool {
        matches!(self, Self::BrowserInteractive | Self::DeviceCodeContinuity)
    }

    /// Whether this is specifically the device-code (second-device) flow.
    pub const fn is_device_code(self) -> bool {
        matches!(self, Self::DeviceCodeContinuity)
    }

    /// Whether this is specifically the interactive browser flow.
    pub const fn is_browser(self) -> bool {
        matches!(self, Self::BrowserInteractive)
    }

    /// The sign-in action this source needs when auth is unsatisfied, if any.
    ///
    /// A browser flow needs a browser sign-in, a device-code flow needs a
    /// device-code sign-in, and a handle source needs a rebind; anonymous access
    /// needs nothing.
    pub const fn satisfy_action(self) -> Option<AuthActionKind> {
        match self {
            Self::BrowserInteractive => Some(AuthActionKind::SignInBrowser),
            Self::DeviceCodeContinuity => Some(AuthActionKind::SignInDeviceCode),
            Self::OsKeychainHandle | Self::SecretVaultHandle | Self::PolicyBrokerHandle => {
                Some(AuthActionKind::RebindHandle)
            }
            Self::AnonymousAccess | Self::AuthUnsatisfied => None,
        }
    }
}

/// Lifecycle state of a stored credential handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandleState {
    /// The handle resolves to a live credential.
    Active,
    /// The handle has been revoked and no longer resolves.
    Revoked,
    /// The handle has expired and must be refreshed or rebound.
    Expired,
}

impl HandleState {
    /// Every handle state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Active, Self::Revoked, Self::Expired];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Revoked => "revoked",
            Self::Expired => "expired",
        }
    }

    /// Whether a handle in this state blocks trust until rebound.
    pub const fn blocks_trust(self) -> bool {
        matches!(self, Self::Revoked | Self::Expired)
    }
}

/// Lifecycle of a browser/device-code continuity flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityState {
    /// No continuity flow applies to this credential source.
    NotApplicable,
    /// A browser was handed off; awaiting the callback to return.
    AwaitingBrowserReturn,
    /// A device code was issued; awaiting entry on a second device.
    AwaitingDeviceCode,
    /// The continuity flow completed and a handle is bound.
    Established,
    /// The continuity window expired before completion.
    Expired,
    /// The continuity flow failed.
    Failed,
}

impl ContinuityState {
    /// Every continuity state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotApplicable,
        Self::AwaitingBrowserReturn,
        Self::AwaitingDeviceCode,
        Self::Established,
        Self::Expired,
        Self::Failed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::AwaitingBrowserReturn => "awaiting_browser_return",
            Self::AwaitingDeviceCode => "awaiting_device_code",
            Self::Established => "established",
            Self::Expired => "expired",
            Self::Failed => "failed",
        }
    }

    /// Whether the flow is mid-continuity, awaiting a browser return or device code.
    pub const fn is_awaiting(self) -> bool {
        matches!(self, Self::AwaitingBrowserReturn | Self::AwaitingDeviceCode)
    }

    /// Whether the flow has completed and bound a handle.
    pub const fn is_established(self) -> bool {
        matches!(self, Self::Established)
    }

    /// Whether the flow blocks resolution until it is resumed or restarted.
    pub const fn blocks_until_resolved(self) -> bool {
        matches!(
            self,
            Self::AwaitingBrowserReturn | Self::AwaitingDeviceCode | Self::Expired | Self::Failed
        )
    }
}

/// Reachability of a registry or mirror, and the truth a degraded path must
/// disclose instead of a generic failure.
///
/// This is the no-results/auth-required/mirror-stale/cache-only/offline/
/// policy-blocked distinction. An authoritative *no results* is a specific,
/// non-degraded outcome; the degraded variants each disclose their own state so
/// none collapses into "no results" or "connection failed".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradationState {
    /// The registry is reachable, authenticated, and fresh.
    ReachableFresh,
    /// Reachable, authenticated, fresh, and genuinely zero matches.
    NoResultsAuthoritative,
    /// Auth is required and not satisfied.
    AuthRequired,
    /// A mirror is reachable but its metadata is known to be stale.
    MirrorStale,
    /// Only an offline snapshot of the registry metadata is available.
    OfflineSnapshotOnly,
    /// Only the local package cache is available.
    CacheOnly,
    /// A policy blocks this registry or source.
    PolicyBlocked,
}

impl DegradationState {
    /// Every degradation state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ReachableFresh,
        Self::NoResultsAuthoritative,
        Self::AuthRequired,
        Self::MirrorStale,
        Self::OfflineSnapshotOnly,
        Self::CacheOnly,
        Self::PolicyBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableFresh => "reachable_fresh",
            Self::NoResultsAuthoritative => "no_results_authoritative",
            Self::AuthRequired => "auth_required",
            Self::MirrorStale => "mirror_stale",
            Self::OfflineSnapshotOnly => "offline_snapshot_only",
            Self::CacheOnly => "cache_only",
            Self::PolicyBlocked => "policy_blocked",
        }
    }

    /// The specific status message class this state renders to.
    pub const fn canonical_message_class(self) -> RegistryStatusMessageClass {
        match self {
            Self::ReachableFresh => RegistryStatusMessageClass::ReachableFreshSource,
            Self::NoResultsAuthoritative => RegistryStatusMessageClass::NoResultsAuthoritative,
            Self::AuthRequired => RegistryStatusMessageClass::AuthRequiredDisclosure,
            Self::MirrorStale => RegistryStatusMessageClass::MirrorStaleDisclosure,
            Self::OfflineSnapshotOnly => RegistryStatusMessageClass::OfflineSnapshotDisclosure,
            Self::CacheOnly => RegistryStatusMessageClass::CacheOnlyDisclosure,
            Self::PolicyBlocked => RegistryStatusMessageClass::PolicyBlockedDisclosure,
        }
    }

    /// Whether this state is a degraded path that must disclose itself.
    pub const fn is_degraded(self) -> bool {
        !matches!(self, Self::ReachableFresh | Self::NoResultsAuthoritative)
    }

    /// Whether this state must render a specific disclosure rather than read as a
    /// clean, fresh resolution.
    pub const fn must_disclose(self) -> bool {
        !matches!(self, Self::ReachableFresh)
    }

    /// Whether this state blocks trust outright (resolution cannot proceed).
    pub const fn blocks_trust(self) -> bool {
        matches!(self, Self::AuthRequired | Self::PolicyBlocked)
    }

    /// Whether a mutation (write) may proceed against the registry in this state.
    ///
    /// A mutation needs a reachable, fresh, authenticated registry; a stale,
    /// offline, cache-only, auth-required, or policy-blocked path may still serve
    /// a disclosed read but never an install or update.
    pub const fn permits_mutation(self) -> bool {
        matches!(self, Self::ReachableFresh)
    }

    /// The extra action this state requires beyond the credential's sign-in path.
    pub const fn recovery_action(self) -> Option<AuthActionKind> {
        match self {
            Self::OfflineSnapshotOnly | Self::CacheOnly => Some(AuthActionKind::UseOfflineSnapshot),
            Self::PolicyBlocked => Some(AuthActionKind::RequestPolicyException),
            _ => None,
        }
    }

    /// The frozen registry source this state binds to, if it names one.
    pub const fn bound_source(self) -> Option<RegistrySourceAuthority> {
        match self {
            Self::OfflineSnapshotOnly => Some(RegistrySourceAuthority::OfflineSnapshot),
            Self::CacheOnly => Some(RegistrySourceAuthority::LocalCache),
            Self::MirrorStale => Some(RegistrySourceAuthority::EnterpriseMirror),
            _ => None,
        }
    }

    /// The frozen package-state label this state binds to, if it names one.
    pub const fn bound_label(self) -> Option<PackageStateLabel> {
        match self {
            Self::AuthRequired => Some(PackageStateLabel::AuthRequired),
            Self::OfflineSnapshotOnly => Some(PackageStateLabel::OfflineSnapshotOnly),
            Self::MirrorStale => Some(PackageStateLabel::UnknownOrStale),
            _ => None,
        }
    }
}

/// A specific status message class a degradation state renders to.
///
/// The two `Generic*` variants are named only so the lane can forbid them: no
/// row may render a generic class, which is how the packet proves offline,
/// mirror-stale, and cache-only paths never collapse into "no results" or
/// "connection failed".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryStatusMessageClass {
    /// "Reachable, fresh" source message.
    ReachableFreshSource,
    /// "No matching packages" authoritative-empty message.
    NoResultsAuthoritative,
    /// "Sign-in required" disclosure message.
    AuthRequiredDisclosure,
    /// "Mirror metadata is stale" disclosure message.
    MirrorStaleDisclosure,
    /// "Offline snapshot only" disclosure message.
    OfflineSnapshotDisclosure,
    /// "Local cache only" disclosure message.
    CacheOnlyDisclosure,
    /// "Blocked by policy" disclosure message.
    PolicyBlockedDisclosure,
    /// Forbidden generic "no results" message.
    GenericNoResults,
    /// Forbidden generic "connection failed" message.
    GenericConnectionFailed,
}

impl RegistryStatusMessageClass {
    /// Every status message class, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ReachableFreshSource,
        Self::NoResultsAuthoritative,
        Self::AuthRequiredDisclosure,
        Self::MirrorStaleDisclosure,
        Self::OfflineSnapshotDisclosure,
        Self::CacheOnlyDisclosure,
        Self::PolicyBlockedDisclosure,
        Self::GenericNoResults,
        Self::GenericConnectionFailed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableFreshSource => "reachable_fresh_source",
            Self::NoResultsAuthoritative => "no_results_authoritative",
            Self::AuthRequiredDisclosure => "auth_required_disclosure",
            Self::MirrorStaleDisclosure => "mirror_stale_disclosure",
            Self::OfflineSnapshotDisclosure => "offline_snapshot_disclosure",
            Self::CacheOnlyDisclosure => "cache_only_disclosure",
            Self::PolicyBlockedDisclosure => "policy_blocked_disclosure",
            Self::GenericNoResults => "generic_no_results",
            Self::GenericConnectionFailed => "generic_connection_failed",
        }
    }

    /// Whether this class is one of the forbidden generic collapse messages.
    pub const fn is_generic_collapse(self) -> bool {
        matches!(self, Self::GenericNoResults | Self::GenericConnectionFailed)
    }

    /// Whether this class is a specific, non-collapsing message.
    pub const fn is_specific(self) -> bool {
        !self.is_generic_collapse()
    }
}

/// A keyboard-driven action available on a registry auth flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthActionKind {
    /// Retry the current resolution.
    Retry,
    /// Start an interactive browser sign-in.
    SignInBrowser,
    /// Start a device-code sign-in for a second device.
    SignInDeviceCode,
    /// Revoke the current credential handle.
    Revoke,
    /// Switch to a different profile or account.
    SwitchAccount,
    /// Rebind a handle whose credential was revoked or expired.
    RebindHandle,
    /// Proceed with the available offline snapshot or cache.
    UseOfflineSnapshot,
    /// Request a policy exception for a blocked source.
    RequestPolicyException,
}

impl AuthActionKind {
    /// Every action kind, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Retry,
        Self::SignInBrowser,
        Self::SignInDeviceCode,
        Self::Revoke,
        Self::SwitchAccount,
        Self::RebindHandle,
        Self::UseOfflineSnapshot,
        Self::RequestPolicyException,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Retry => "retry",
            Self::SignInBrowser => "sign_in_browser",
            Self::SignInDeviceCode => "sign_in_device_code",
            Self::Revoke => "revoke",
            Self::SwitchAccount => "switch_account",
            Self::RebindHandle => "rebind_handle",
            Self::UseOfflineSnapshot => "use_offline_snapshot",
            Self::RequestPolicyException => "request_policy_exception",
        }
    }
}

/// The current identity profile a registry auth flow runs under.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegistryProfile {
    /// Durable profile identity, stable across reopen and account switch.
    pub profile_id: String,
    /// Human-readable profile label; redacted, never a raw URL or token.
    pub display_label: String,
    /// Whether this is the current/active profile for its registry source.
    pub is_current: bool,
}

impl RegistryProfile {
    /// Whether the profile fields are present.
    pub fn is_consistent(&self) -> bool {
        !self.profile_id.trim().is_empty() && !self.display_label.trim().is_empty()
    }
}

/// An opaque, broker-resolved credential handle that never carries a secret body.
///
/// The handle is the only thing this lane persists for a credential. It names an
/// opaque reference the secret broker resolves on demand, a redacted account
/// label, and a [`HandleState`]; it never carries a token, a private registry
/// URL, or a full auth payload, and `stores_secret_body` is a recomputed guard
/// that must stay `false`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretHandle {
    /// Opaque handle reference the broker resolves; never a token or URL.
    pub handle_ref: String,
    /// Lifecycle state of the handle.
    pub state: HandleState,
    /// Retention class; must be [`RetentionClass::BrokerResolvedNeverPersisted`].
    pub retention: RetentionClass,
    /// Redacted account label safe for support exports; never a token or URL.
    pub redacted_account_label: String,
    /// Whether a secret body is stored; the guard must keep this `false`.
    pub stores_secret_body: bool,
}

impl SecretHandle {
    /// The retention class every credential handle must carry.
    pub const CANONICAL_RETENTION: RetentionClass = RetentionClass::BrokerResolvedNeverPersisted;

    /// Whether this handle currently blocks trust (revoked or expired).
    pub const fn blocks_trust(&self) -> bool {
        self.state.blocks_trust()
    }

    /// Whether the handle is redaction- and retention-safe: it stores no body,
    /// retains only a broker-resolved handle, and leaks no raw URL.
    pub fn is_export_safe(&self) -> bool {
        !self.stores_secret_body
            && self.retention == Self::CANONICAL_RETENTION
            && !leaks_raw_url(&self.handle_ref)
            && !leaks_raw_url(&self.redacted_account_label)
    }

    /// Whether the handle is internally consistent.
    pub fn is_consistent(&self) -> bool {
        self.is_export_safe()
            && !self.handle_ref.trim().is_empty()
            && !self.redacted_account_label.trim().is_empty()
    }
}

/// A single keyboard-driven action offered on a registry auth flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthActionRow {
    /// The action kind.
    pub kind: AuthActionKind,
    /// Stable command id that invokes the action.
    pub command_id: String,
    /// Keyboard hint for the action; never empty so the flow stays reachable.
    pub key_hint: String,
    /// Redacted action label safe for support exports.
    pub redacted_label: String,
}

impl AuthActionRow {
    /// Whether the action is keyboard-complete: it carries a command id and a key
    /// hint, so it can be driven without a pointer.
    pub fn is_keyboard_complete(&self) -> bool {
        !self.command_id.trim().is_empty() && !self.key_hint.trim().is_empty()
    }

    /// Whether the action is export-safe (no raw URL leaks in its labels).
    pub fn is_export_safe(&self) -> bool {
        !leaks_raw_url(&self.command_id)
            && !leaks_raw_url(&self.redacted_label)
            && !self.redacted_label.trim().is_empty()
    }
}

/// A single registry authentication flow: the current profile, the provider or
/// mirror it reaches, the credential source and handle behind it, the continuity
/// state of any browser/device-code sign-in, the reachability/degradation truth,
/// and the keyboard-complete actions available.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegistryAuthFlowRow {
    /// Stable row id.
    pub row_id: String,
    /// The current identity profile.
    pub profile: RegistryProfile,
    /// Registry or mirror source class.
    pub source_class: RegistrySourceAuthority,
    /// Redacted mirror or private-registry owner; present only for a private
    /// registry or enterprise mirror, never a raw URL or token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_owner: Option<String>,
    /// Redacted source label safe for support exports; never a URL or token.
    pub redacted_source_label: String,
    /// Where the credential comes from and how it is presented.
    pub credential_source: CredentialSourceClass,
    /// Frozen auth mode; must equal [`CredentialSourceClass::frozen_auth_mode`].
    pub auth_mode: AuthMode,
    /// The credential handle, present only for a handle-backed source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handle: Option<SecretHandle>,
    /// Lifecycle of any browser/device-code continuity flow.
    pub continuity: ContinuityState,
    /// Reachability and degradation truth of the registry.
    pub reachability: DegradationState,
    /// Keyboard-complete actions available on this flow.
    #[serde(default)]
    pub actions: Vec<AuthActionRow>,
    /// Reviewer-facing note.
    pub note: String,
}

impl RegistryAuthFlowRow {
    /// Whether a source class names an owner (a private registry or mirror).
    pub const fn class_has_owner(source: RegistrySourceAuthority) -> bool {
        matches!(
            source,
            RegistrySourceAuthority::PrivateRegistry | RegistrySourceAuthority::EnterpriseMirror
        )
    }

    /// The specific status message class this flow renders for its reachability.
    pub const fn message_class(&self) -> RegistryStatusMessageClass {
        self.reachability.canonical_message_class()
    }

    /// Whether the flow's credential is held as an opaque handle.
    pub const fn is_handle_backed(&self) -> bool {
        self.credential_source.is_handle_backed()
    }

    /// Whether the flow uses a browser or device-code sign-in.
    pub const fn is_browser_or_device(&self) -> bool {
        self.credential_source.requires_continuity()
    }

    /// Whether the flow specifically uses the device-code (second-device) path.
    pub const fn is_device_code(&self) -> bool {
        self.credential_source.is_device_code()
    }

    /// Whether trust in this flow is blocked right now.
    ///
    /// A blocking reachability (auth-required or policy-blocked), a revoked or
    /// expired handle, or a stalled continuity flow each block trust.
    pub fn trust_blocked(&self) -> bool {
        self.reachability.blocks_trust()
            || self.handle.as_ref().is_some_and(SecretHandle::blocks_trust)
            || self.continuity.blocks_until_resolved()
    }

    /// Whether a mutation may proceed against this registry from a mutating
    /// surface: trust is not blocked and the reachability permits a write.
    pub fn mutation_ready(&self) -> bool {
        !self.trust_blocked() && self.reachability.permits_mutation()
    }

    /// The set of action kinds this flow must offer.
    ///
    /// A revocable handle must always offer revoke and switch-account so a leaked
    /// or wrong account is recoverable; an unsatisfied auth or stalled continuity
    /// must offer a matched sign-in or rebind plus a retry; a degraded path must
    /// offer its recovery action.
    pub fn required_action_kinds(&self) -> BTreeSet<AuthActionKind> {
        let mut req = BTreeSet::new();
        if self.handle.is_some() {
            req.insert(AuthActionKind::Revoke);
            req.insert(AuthActionKind::SwitchAccount);
        }
        if self.reachability == DegradationState::AuthRequired
            || self.continuity.blocks_until_resolved()
            || self.handle.as_ref().is_some_and(SecretHandle::blocks_trust)
        {
            if let Some(action) = self.credential_source.satisfy_action() {
                req.insert(action);
            }
            req.insert(AuthActionKind::Retry);
        }
        if let Some(action) = self.reachability.recovery_action() {
            req.insert(action);
        }
        req
    }

    /// The action kinds this flow actually offers.
    pub fn offered_action_kinds(&self) -> BTreeSet<AuthActionKind> {
        self.actions.iter().map(|a| a.kind).collect()
    }

    /// Whether every required action is offered.
    pub fn actions_complete(&self) -> bool {
        self.required_action_kinds()
            .is_subset(&self.offered_action_kinds())
    }

    /// Whether every offered action is keyboard-complete.
    pub fn all_actions_keyboard_complete(&self) -> bool {
        self.actions.iter().all(AuthActionRow::is_keyboard_complete)
    }

    /// Whether the handle's presence matches the credential source.
    ///
    /// Only a handle-backed source may carry a handle; anonymous access and an
    /// unsatisfied auth never do.
    pub fn handle_presence_consistent(&self) -> bool {
        if self.handle.is_some() {
            self.credential_source.is_handle_backed()
        } else {
            true
        }
    }

    /// Whether the continuity state matches the credential source.
    ///
    /// A non-continuity source must be `not_applicable`; a continuity source must
    /// be in one of the continuity states.
    pub fn continuity_consistent(&self) -> bool {
        if self.credential_source.requires_continuity() {
            self.continuity != ContinuityState::NotApplicable
        } else {
            self.continuity == ContinuityState::NotApplicable
        }
    }

    /// Whether the reachability is causally consistent with the credential state.
    ///
    /// A reachable-fresh or authoritative-no-results outcome cannot coexist with a
    /// revoked/expired handle or a stalled continuity; an auth-required outcome
    /// must have a cause — an unsatisfied/blocked credential, a revoked/expired
    /// handle, or a stalled continuity.
    pub fn reachability_consistent(&self) -> bool {
        let credential_blocks = self.handle.as_ref().is_some_and(SecretHandle::blocks_trust)
            || self.continuity.blocks_until_resolved()
            || self.credential_source == CredentialSourceClass::AuthUnsatisfied;
        match self.reachability {
            DegradationState::ReachableFresh | DegradationState::NoResultsAuthoritative => {
                !credential_blocks
            }
            DegradationState::AuthRequired => credential_blocks,
            _ => true,
        }
    }

    /// The frozen registry source this flow binds for matrix lookup.
    pub const fn frozen_source(&self) -> RegistrySourceAuthority {
        self.source_class
    }

    /// Every frozen package-state label this flow surfaces (the bound degradation
    /// label, if any).
    pub fn applicable_labels(&self) -> BTreeSet<PackageStateLabel> {
        let mut labels = BTreeSet::new();
        if let Some(label) = self.reachability.bound_label() {
            labels.insert(label);
        }
        labels
    }

    /// Whether the flow is redaction- and export-safe end to end.
    pub fn is_export_safe(&self) -> bool {
        let no_url = !leaks_raw_url(&self.redacted_source_label)
            && self
                .mirror_owner
                .as_ref()
                .map_or(true, |owner| !leaks_raw_url(owner))
            && !leaks_raw_url(&self.profile.display_label);
        no_url
            && self
                .handle
                .as_ref()
                .map_or(true, SecretHandle::is_export_safe)
            && self.actions.iter().all(AuthActionRow::is_export_safe)
    }

    /// Whether the row is internally consistent against the contract.
    pub fn is_consistent(&self) -> bool {
        self.profile.is_consistent()
            && self.auth_mode == self.credential_source.frozen_auth_mode()
            && self.handle_presence_consistent()
            && self
                .handle
                .as_ref()
                .map_or(true, SecretHandle::is_consistent)
            && self.continuity_consistent()
            && self.reachability_consistent()
            && self.mirror_owner.is_some() == Self::class_has_owner(self.source_class)
            && self.message_class().is_specific()
            && self.actions_complete()
            && self.all_actions_keyboard_complete()
            && self.is_export_safe()
    }

    /// Projects the canonical per-row view reused by desktop, review, and AI
    /// inspect surfaces.
    pub fn view(&self) -> RegistryAuthFlowView {
        RegistryAuthFlowView {
            row_id: self.row_id.clone(),
            profile_id: self.profile.profile_id.clone(),
            profile_label: self.profile.display_label.clone(),
            is_current: self.profile.is_current,
            source_class: self.source_class.as_str().to_owned(),
            mirror_owner: self.mirror_owner.clone(),
            redacted_source_label: self.redacted_source_label.clone(),
            credential_source: self.credential_source.as_str().to_owned(),
            auth_mode: self.auth_mode.as_str().to_owned(),
            is_handle_backed: self.is_handle_backed(),
            is_browser_or_device: self.is_browser_or_device(),
            is_device_code: self.is_device_code(),
            handle_state: self.handle.as_ref().map(|h| h.state.as_str().to_owned()),
            redacted_account_label: self
                .handle
                .as_ref()
                .map(|h| h.redacted_account_label.clone()),
            retention: self
                .handle
                .as_ref()
                .map(|h| h.retention.as_str().to_owned()),
            continuity: self.continuity.as_str().to_owned(),
            reachability: self.reachability.as_str().to_owned(),
            message_class: self.message_class().as_str().to_owned(),
            is_degraded: self.reachability.is_degraded(),
            must_disclose: self.reachability.must_disclose(),
            trust_blocked: self.trust_blocked(),
            mutation_ready: self.mutation_ready(),
            actions: self
                .actions
                .iter()
                .map(|a| AuthActionView {
                    kind: a.kind.as_str().to_owned(),
                    command_id: a.command_id.clone(),
                    key_hint: a.key_hint.clone(),
                    redacted_label: a.redacted_label.clone(),
                })
                .collect(),
            applicable_labels: self
                .applicable_labels()
                .iter()
                .map(|l| l.as_str().to_owned())
                .collect(),
        }
    }

    /// Projects a redaction-safe export row reused by support/export packets and
    /// the CLI inspect surface.
    pub fn export_row(&self) -> RegistryAuthFlowExportRow {
        RegistryAuthFlowExportRow {
            row_id: self.row_id.clone(),
            profile_id: self.profile.profile_id.clone(),
            profile_label: self.profile.display_label.clone(),
            is_current: self.profile.is_current,
            source_class: self.source_class.as_str().to_owned(),
            mirror_owner: self.mirror_owner.clone(),
            redacted_source_label: self.redacted_source_label.clone(),
            credential_source: self.credential_source.as_str().to_owned(),
            auth_mode: self.auth_mode.as_str().to_owned(),
            is_handle_backed: self.is_handle_backed(),
            is_device_code: self.is_device_code(),
            handle_state: self.handle.as_ref().map(|h| h.state.as_str().to_owned()),
            redacted_account_label: self
                .handle
                .as_ref()
                .map(|h| h.redacted_account_label.clone()),
            retention: self
                .handle
                .as_ref()
                .map(|h| h.retention.as_str().to_owned()),
            stores_secret_body: self.handle.as_ref().is_some_and(|h| h.stores_secret_body),
            continuity: self.continuity.as_str().to_owned(),
            reachability: self.reachability.as_str().to_owned(),
            message_class: self.message_class().as_str().to_owned(),
            trust_blocked: self.trust_blocked(),
            mutation_ready: self.mutation_ready(),
            offered_actions: self
                .offered_action_kinds()
                .iter()
                .map(|k| k.as_str().to_owned())
                .collect(),
        }
    }

    /// Projects the row onto a specific marketed surface, pinning the write
    /// authority that surface may carry from the frozen matrix.
    pub fn surface_projection(&self, surface: PackageSurface) -> RegistryAuthFlowSurfaceProjection {
        let authority = surface.canonical_write_authority();
        RegistryAuthFlowSurfaceProjection {
            surface: surface.as_str().to_owned(),
            write_authority: authority.as_str().to_owned(),
            // A mutation only proceeds where the surface can mutate AND the flow
            // is mutation-ready; inspect/review/export surfaces never mutate.
            can_mutate_here: authority.can_mutate() && self.mutation_ready(),
            redacted: matches!(authority, SurfaceWriteAuthority::RedactedExport),
            view: self.view(),
        }
    }
}

/// The canonical per-row view reused by desktop, review, and AI inspect surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryAuthFlowView {
    /// Row id.
    pub row_id: String,
    /// Profile id.
    pub profile_id: String,
    /// Profile label.
    pub profile_label: String,
    /// Whether this is the current profile.
    pub is_current: bool,
    /// Source class token.
    pub source_class: String,
    /// Redacted mirror or private-registry owner, if any.
    pub mirror_owner: Option<String>,
    /// Redacted source label.
    pub redacted_source_label: String,
    /// Credential source token.
    pub credential_source: String,
    /// Frozen auth mode token.
    pub auth_mode: String,
    /// Whether the credential is held as a handle.
    pub is_handle_backed: bool,
    /// Whether a browser or device-code sign-in is used.
    pub is_browser_or_device: bool,
    /// Whether the device-code path is used specifically.
    pub is_device_code: bool,
    /// Handle state token, if a handle is present.
    pub handle_state: Option<String>,
    /// Redacted account label, if a handle is present.
    pub redacted_account_label: Option<String>,
    /// Retention class token, if a handle is present.
    pub retention: Option<String>,
    /// Continuity state token.
    pub continuity: String,
    /// Reachability/degradation token.
    pub reachability: String,
    /// Specific status message-class token.
    pub message_class: String,
    /// Whether the path is degraded.
    pub is_degraded: bool,
    /// Whether the path must disclose itself.
    pub must_disclose: bool,
    /// Whether trust is blocked right now.
    pub trust_blocked: bool,
    /// Whether a mutation may proceed.
    pub mutation_ready: bool,
    /// Keyboard-complete action views.
    pub actions: Vec<AuthActionView>,
    /// Every applicable frozen package-state label token.
    pub applicable_labels: Vec<String>,
}

/// A redaction-safe view of one keyboard-driven action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthActionView {
    /// Action kind token.
    pub kind: String,
    /// Command id.
    pub command_id: String,
    /// Key hint.
    pub key_hint: String,
    /// Redacted label.
    pub redacted_label: String,
}

/// A redaction-safe export row reused by support/export packets and CLI inspect.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryAuthFlowExportRow {
    /// Row id.
    pub row_id: String,
    /// Profile id.
    pub profile_id: String,
    /// Profile label.
    pub profile_label: String,
    /// Whether this is the current profile.
    pub is_current: bool,
    /// Source class token.
    pub source_class: String,
    /// Redacted mirror or private-registry owner, if any.
    pub mirror_owner: Option<String>,
    /// Redacted source label.
    pub redacted_source_label: String,
    /// Credential source token.
    pub credential_source: String,
    /// Frozen auth mode token.
    pub auth_mode: String,
    /// Whether the credential is held as a handle.
    pub is_handle_backed: bool,
    /// Whether the device-code path is used specifically.
    pub is_device_code: bool,
    /// Handle state token, if a handle is present.
    pub handle_state: Option<String>,
    /// Redacted account label, if a handle is present.
    pub redacted_account_label: Option<String>,
    /// Retention class token, if a handle is present.
    pub retention: Option<String>,
    /// Whether a secret body is stored; always false.
    pub stores_secret_body: bool,
    /// Continuity state token.
    pub continuity: String,
    /// Reachability/degradation token.
    pub reachability: String,
    /// Specific status message-class token.
    pub message_class: String,
    /// Whether trust is blocked right now.
    pub trust_blocked: bool,
    /// Whether a mutation may proceed.
    pub mutation_ready: bool,
    /// Offered action-kind tokens.
    pub offered_actions: Vec<String>,
}

/// A row projected onto a specific marketed surface with its pinned write
/// authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryAuthFlowSurfaceProjection {
    /// Package surface token.
    pub surface: String,
    /// Write authority token pinned by the frozen matrix.
    pub write_authority: String,
    /// Whether a mutation may proceed from this surface.
    pub can_mutate_here: bool,
    /// Whether the surface produces a redacted export.
    pub redacted: bool,
    /// The canonical per-row view.
    pub view: RegistryAuthFlowView,
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegistryAuthFlowsSummary {
    /// Total rows.
    pub total_rows: usize,
    /// Rows whose profile is the current one.
    pub current_profile_rows: usize,
    /// Rows whose credential is held as a handle.
    pub handle_backed_rows: usize,
    /// Rows that use a browser or device-code sign-in.
    pub browser_or_device_rows: usize,
    /// Rows that use the device-code path specifically.
    pub device_code_rows: usize,
    /// Rows whose registry is in a degraded state.
    pub degraded_rows: usize,
    /// Rows whose reachability is auth-required.
    pub auth_required_rows: usize,
    /// Rows whose trust is blocked.
    pub trust_blocked_rows: usize,
    /// Rows ready for a mutation.
    pub mutation_ready_rows: usize,
    /// Rows that offer a revoke action.
    pub revocable_rows: usize,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryAuthFlowsExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Matrix id every row binds to.
    pub references_matrix_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected export rows.
    pub rows: Vec<RegistryAuthFlowExportRow>,
    /// Whether every row is consistent with the contract.
    pub all_consistent: bool,
    /// Whether no row renders a generic collapse message.
    pub no_generic_collapse: bool,
    /// Whether no row stores a secret body.
    pub no_secret_bodies: bool,
    /// Whether every offered action is keyboard-complete.
    pub all_keyboard_complete: bool,
    /// Whether every row binds to the frozen matrix.
    pub all_bind_matrix: bool,
}

/// The typed registry-auth-flows packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegistryAuthFlows {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// The frozen matrix packet id every row binds to.
    pub references_matrix_id: String,
    /// Closed credential-source vocabulary represented by this packet.
    pub credential_source_classes: Vec<CredentialSourceClass>,
    /// Closed handle-state vocabulary represented by this packet.
    pub handle_states: Vec<HandleState>,
    /// Closed continuity-state vocabulary represented by this packet.
    pub continuity_states: Vec<ContinuityState>,
    /// Closed degradation-state vocabulary represented by this packet.
    pub degradation_states: Vec<DegradationState>,
    /// Closed status-message-class vocabulary represented by this packet.
    pub status_message_classes: Vec<RegistryStatusMessageClass>,
    /// Closed auth-action-kind vocabulary represented by this packet.
    pub auth_action_kinds: Vec<AuthActionKind>,
    /// The registry auth flow rows.
    #[serde(default)]
    pub rows: Vec<RegistryAuthFlowRow>,
    /// Summary counts.
    pub summary: RegistryAuthFlowsSummary,
}

impl RegistryAuthFlows {
    /// Returns the row with the given id.
    pub fn row(&self, row_id: &str) -> Option<&RegistryAuthFlowRow> {
        self.rows.iter().find(|r| r.row_id == row_id)
    }

    /// Whether every row is consistent with the contract.
    pub fn all_consistent(&self) -> bool {
        self.rows.iter().all(RegistryAuthFlowRow::is_consistent)
    }

    /// Whether no row renders a generic collapse message.
    pub fn no_generic_collapse(&self) -> bool {
        self.rows.iter().all(|r| r.message_class().is_specific())
    }

    /// Whether no row stores a secret body anywhere.
    pub fn no_secret_bodies(&self) -> bool {
        self.rows
            .iter()
            .all(|r| r.handle.as_ref().map_or(true, |h| !h.stores_secret_body))
    }

    /// Whether every offered action is keyboard-complete.
    pub fn all_keyboard_complete(&self) -> bool {
        self.rows
            .iter()
            .all(RegistryAuthFlowRow::all_actions_keyboard_complete)
    }

    /// Whether every row's source resolves to a frozen registry cell and every
    /// label it surfaces resolves to a frozen state row, proving the packet binds
    /// to the shared matrix.
    pub fn all_bind_matrix(&self) -> bool {
        let Ok(matrix) = current_m5_package_state_matrix() else {
            return false;
        };
        if self.references_matrix_id != matrix.packet_id {
            return false;
        }
        self.rows.iter().all(|r| {
            matrix.registry_cell(r.frozen_source()).is_some()
                && r.reachability
                    .bound_source()
                    .map_or(true, |s| matrix.registry_cell(s).is_some())
                && r.applicable_labels()
                    .iter()
                    .all(|label| matrix.state(*label).is_some())
        })
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> RegistryAuthFlowsSummary {
        let count = |pred: &dyn Fn(&RegistryAuthFlowRow) -> bool| {
            self.rows.iter().filter(|r| pred(r)).count()
        };
        RegistryAuthFlowsSummary {
            total_rows: self.rows.len(),
            current_profile_rows: count(&|r| r.profile.is_current),
            handle_backed_rows: count(&|r| r.handle.is_some()),
            browser_or_device_rows: count(&RegistryAuthFlowRow::is_browser_or_device),
            device_code_rows: count(&RegistryAuthFlowRow::is_device_code),
            degraded_rows: count(&|r| r.reachability.is_degraded()),
            auth_required_rows: count(&|r| r.reachability == DegradationState::AuthRequired),
            trust_blocked_rows: count(&RegistryAuthFlowRow::trust_blocked),
            mutation_ready_rows: count(&RegistryAuthFlowRow::mutation_ready),
            revocable_rows: count(&|r| r.offered_action_kinds().contains(&AuthActionKind::Revoke)),
        }
    }

    /// Produces a redaction-safe export projection that downstream surfaces —
    /// support exports, the CLI inspect surface, and release/public-truth — render
    /// instead of restating registry-auth state by hand.
    pub fn export_projection(&self) -> RegistryAuthFlowsExportProjection {
        RegistryAuthFlowsExportProjection {
            packet_id: self.packet_id.clone(),
            references_matrix_id: self.references_matrix_id.clone(),
            as_of: self.as_of.clone(),
            rows: self
                .rows
                .iter()
                .map(RegistryAuthFlowRow::export_row)
                .collect(),
            all_consistent: self.all_consistent(),
            no_generic_collapse: self.no_generic_collapse(),
            no_secret_bodies: self.no_secret_bodies(),
            all_keyboard_complete: self.all_keyboard_complete(),
            all_bind_matrix: self.all_bind_matrix(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<RegistryAuthFlowsViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rows(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(RegistryAuthFlowsViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<RegistryAuthFlowsViolation>) {
        if self.schema_version != REGISTRY_AUTH_FLOWS_SCHEMA_VERSION {
            violations.push(RegistryAuthFlowsViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != REGISTRY_AUTH_FLOWS_RECORD_KIND {
            violations.push(RegistryAuthFlowsViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("references_matrix_id", &self.references_matrix_id),
        ] {
            if value.trim().is_empty() {
                violations.push(RegistryAuthFlowsViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "credential_source_classes",
                self.credential_source_classes == CredentialSourceClass::ALL.to_vec(),
            ),
            (
                "handle_states",
                self.handle_states == HandleState::ALL.to_vec(),
            ),
            (
                "continuity_states",
                self.continuity_states == ContinuityState::ALL.to_vec(),
            ),
            (
                "degradation_states",
                self.degradation_states == DegradationState::ALL.to_vec(),
            ),
            (
                "status_message_classes",
                self.status_message_classes == RegistryStatusMessageClass::ALL.to_vec(),
            ),
            (
                "auth_action_kinds",
                self.auth_action_kinds == AuthActionKind::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(RegistryAuthFlowsViolation::ClosedVocabularyMismatch { field });
            }
        }
        match current_m5_package_state_matrix() {
            Ok(matrix) => {
                if self.references_matrix_id != matrix.packet_id {
                    violations.push(RegistryAuthFlowsViolation::MatrixBindingMismatch {
                        referenced: self.references_matrix_id.clone(),
                        expected: matrix.packet_id,
                    });
                }
            }
            Err(_) => violations.push(RegistryAuthFlowsViolation::MatrixUnavailable),
        }
    }

    fn validate_rows(&self, violations: &mut Vec<RegistryAuthFlowsViolation>) {
        let matrix = current_m5_package_state_matrix().ok();
        let mut seen_ids = BTreeSet::new();
        // Track which source identities already claim a current profile.
        let mut current_sources: BTreeSet<(String, String)> = BTreeSet::new();
        for row in &self.rows {
            let id = row.row_id.clone();
            if !seen_ids.insert(id.clone()) {
                violations.push(RegistryAuthFlowsViolation::DuplicateRowId { row_id: id.clone() });
            }

            for (field, value) in [
                ("row_id", &row.row_id),
                ("profile.profile_id", &row.profile.profile_id),
                ("profile.display_label", &row.profile.display_label),
                ("redacted_source_label", &row.redacted_source_label),
                ("note", &row.note),
            ] {
                if value.trim().is_empty() {
                    violations.push(RegistryAuthFlowsViolation::EmptyField {
                        id: id.clone(),
                        field_name: field,
                    });
                }
            }

            // The auth mode must equal the credential source's frozen mode.
            if row.auth_mode != row.credential_source.frozen_auth_mode() {
                violations.push(RegistryAuthFlowsViolation::AuthModeMismatch {
                    row_id: id.clone(),
                    declared: row.auth_mode.as_str(),
                    required: row.credential_source.frozen_auth_mode().as_str(),
                });
            }

            // A handle may be present only for a handle-backed source.
            if !row.handle_presence_consistent() {
                violations.push(RegistryAuthFlowsViolation::HandlePresenceMismatch {
                    row_id: id.clone(),
                    credential_source: row.credential_source.as_str(),
                });
            }

            // Handle retention must be broker-resolved and store no body.
            if let Some(handle) = &row.handle {
                if handle.retention != SecretHandle::CANONICAL_RETENTION {
                    violations.push(RegistryAuthFlowsViolation::RetentionMismatch {
                        row_id: id.clone(),
                        declared: handle.retention.as_str(),
                    });
                }
                if handle.stores_secret_body {
                    violations
                        .push(RegistryAuthFlowsViolation::SecretBodyStored { row_id: id.clone() });
                }
                for (field, value) in [
                    ("handle.handle_ref", &handle.handle_ref),
                    (
                        "handle.redacted_account_label",
                        &handle.redacted_account_label,
                    ),
                ] {
                    if value.trim().is_empty() {
                        violations.push(RegistryAuthFlowsViolation::EmptyField {
                            id: id.clone(),
                            field_name: field,
                        });
                    }
                }
            }

            // Continuity state must match the credential source.
            if !row.continuity_consistent() {
                violations.push(RegistryAuthFlowsViolation::ContinuityMismatch {
                    row_id: id.clone(),
                    continuity: row.continuity.as_str(),
                    credential_source: row.credential_source.as_str(),
                });
            }

            // Reachability must be causally consistent with the credential state.
            if !row.reachability_consistent() {
                violations.push(RegistryAuthFlowsViolation::ReachabilityContradiction {
                    row_id: id.clone(),
                    reachability: row.reachability.as_str(),
                });
            }

            // The mirror owner must be present exactly for a private/mirror source.
            if row.mirror_owner.is_some() != RegistryAuthFlowRow::class_has_owner(row.source_class)
            {
                violations.push(RegistryAuthFlowsViolation::SourceOwnerMismatch {
                    row_id: id.clone(),
                    source_class: row.source_class.as_str(),
                });
            }

            // The reachability message must never collapse into a generic message.
            if row.message_class().is_generic_collapse() {
                violations.push(RegistryAuthFlowsViolation::GenericCollapseMessage {
                    row_id: id.clone(),
                    message: row.message_class().as_str(),
                });
            }

            // Every required action must be offered.
            let offered = row.offered_action_kinds();
            for required in row.required_action_kinds() {
                if !offered.contains(&required) {
                    violations.push(RegistryAuthFlowsViolation::MissingRequiredAction {
                        row_id: id.clone(),
                        action: required.as_str(),
                    });
                }
            }

            // Every offered action must be keyboard-complete and export-safe, and
            // no action kind may be offered twice.
            let mut seen_actions = BTreeSet::new();
            for action in &row.actions {
                if !seen_actions.insert(action.kind) {
                    violations.push(RegistryAuthFlowsViolation::DuplicateAction {
                        row_id: id.clone(),
                        action: action.kind.as_str(),
                    });
                }
                if !action.is_keyboard_complete() {
                    violations.push(RegistryAuthFlowsViolation::ActionNotKeyboardComplete {
                        row_id: id.clone(),
                        action: action.kind.as_str(),
                    });
                }
                if action.redacted_label.trim().is_empty() {
                    violations.push(RegistryAuthFlowsViolation::EmptyField {
                        id: id.clone(),
                        field_name: "action.redacted_label",
                    });
                }
            }

            // No redacted field may leak a raw URL.
            for (field, value) in [
                ("redacted_source_label", &row.redacted_source_label),
                ("profile.display_label", &row.profile.display_label),
            ] {
                if leaks_raw_url(value) {
                    violations.push(RegistryAuthFlowsViolation::RawUrlLeak {
                        id: id.clone(),
                        field_name: field,
                    });
                }
            }
            if let Some(owner) = &row.mirror_owner {
                if leaks_raw_url(owner) {
                    violations.push(RegistryAuthFlowsViolation::RawUrlLeak {
                        id: id.clone(),
                        field_name: "mirror_owner",
                    });
                }
            }
            if let Some(handle) = &row.handle {
                for (field, value) in [
                    ("handle.handle_ref", &handle.handle_ref),
                    (
                        "handle.redacted_account_label",
                        &handle.redacted_account_label,
                    ),
                ] {
                    if leaks_raw_url(value) {
                        violations.push(RegistryAuthFlowsViolation::RawUrlLeak {
                            id: id.clone(),
                            field_name: field,
                        });
                    }
                }
            }

            // At most one current profile per source identity.
            if row.profile.is_current {
                let key = (
                    row.source_class.as_str().to_owned(),
                    row.redacted_source_label.clone(),
                );
                if !current_sources.insert(key) {
                    violations.push(RegistryAuthFlowsViolation::MultipleCurrentProfiles {
                        source_class: row.source_class.as_str(),
                        redacted_source_label: row.redacted_source_label.clone(),
                    });
                }
            }

            // The frozen source and every surfaced label must bind to the matrix.
            if let Some(matrix) = &matrix {
                if matrix.registry_cell(row.frozen_source()).is_none() {
                    violations.push(RegistryAuthFlowsViolation::UnboundSource {
                        row_id: id.clone(),
                        source_class: row.source_class.as_str(),
                    });
                }
                if let Some(source) = row.reachability.bound_source() {
                    if matrix.registry_cell(source).is_none() {
                        violations.push(RegistryAuthFlowsViolation::UnboundSource {
                            row_id: id.clone(),
                            source_class: source.as_str(),
                        });
                    }
                }
                for label in row.applicable_labels() {
                    if matrix.state(label).is_none() {
                        violations.push(RegistryAuthFlowsViolation::UnboundLabel {
                            row_id: id.clone(),
                            label: label.as_str(),
                        });
                    }
                }
            }
        }
    }
}

/// Whether a string leaks a raw URL or scheme that must be redacted.
fn leaks_raw_url(value: &str) -> bool {
    value.contains("://")
}

/// A validation violation for the registry-auth-flows packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryAuthFlowsViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row id appears more than once.
    DuplicateRowId {
        /// Duplicate id.
        row_id: String,
    },
    /// The frozen matrix could not be loaded for binding validation.
    MatrixUnavailable,
    /// The packet references a matrix id other than the frozen matrix.
    MatrixBindingMismatch {
        /// Referenced matrix id.
        referenced: String,
        /// Expected (frozen) matrix id.
        expected: String,
    },
    /// The auth mode disagrees with the credential source's frozen mode.
    AuthModeMismatch {
        /// Row id.
        row_id: String,
        /// Declared auth mode token.
        declared: &'static str,
        /// Required auth mode token.
        required: &'static str,
    },
    /// A handle is present without a handle-backed source.
    HandlePresenceMismatch {
        /// Row id.
        row_id: String,
        /// Credential source token.
        credential_source: &'static str,
    },
    /// A handle carries a retention class other than broker-resolved.
    RetentionMismatch {
        /// Row id.
        row_id: String,
        /// Declared retention token.
        declared: &'static str,
    },
    /// A handle stores a secret body, breaking the handle-only guarantee.
    SecretBodyStored {
        /// Row id.
        row_id: String,
    },
    /// The continuity state disagrees with the credential source.
    ContinuityMismatch {
        /// Row id.
        row_id: String,
        /// Continuity token.
        continuity: &'static str,
        /// Credential source token.
        credential_source: &'static str,
    },
    /// The reachability contradicts the credential state.
    ReachabilityContradiction {
        /// Row id.
        row_id: String,
        /// Reachability token.
        reachability: &'static str,
    },
    /// The mirror owner is present without a private/mirror source, or absent
    /// with one.
    SourceOwnerMismatch {
        /// Row id.
        row_id: String,
        /// Source class token.
        source_class: &'static str,
    },
    /// A row renders a forbidden generic collapse message.
    GenericCollapseMessage {
        /// Row id.
        row_id: String,
        /// Generic message-class token.
        message: &'static str,
    },
    /// A required action is not offered.
    MissingRequiredAction {
        /// Row id.
        row_id: String,
        /// Required action token.
        action: &'static str,
    },
    /// An offered action lacks a command id or key hint.
    ActionNotKeyboardComplete {
        /// Row id.
        row_id: String,
        /// Action token.
        action: &'static str,
    },
    /// An action kind is offered more than once.
    DuplicateAction {
        /// Row id.
        row_id: String,
        /// Duplicate action token.
        action: &'static str,
    },
    /// More than one current profile claims the same registry source identity.
    MultipleCurrentProfiles {
        /// Source class token.
        source_class: &'static str,
        /// Redacted source label.
        redacted_source_label: String,
    },
    /// A field leaks a raw URL that must be redacted.
    RawUrlLeak {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row's registry source does not bind to a frozen registry cell.
    UnboundSource {
        /// Row id.
        row_id: String,
        /// Source class token.
        source_class: &'static str,
    },
    /// A surfaced label does not bind to a frozen state row.
    UnboundLabel {
        /// Row id.
        row_id: String,
        /// Label token.
        label: &'static str,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for RegistryAuthFlowsViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical vocabulary")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateRowId { row_id } => write!(f, "duplicate row id {row_id}"),
            Self::MatrixUnavailable => {
                write!(f, "the frozen package-state matrix could not be loaded")
            }
            Self::MatrixBindingMismatch {
                referenced,
                expected,
            } => write!(
                f,
                "packet references matrix id {referenced} instead of the frozen {expected}"
            ),
            Self::AuthModeMismatch {
                row_id,
                declared,
                required,
            } => write!(
                f,
                "row {row_id} auth mode {declared} disagrees with required {required}"
            ),
            Self::HandlePresenceMismatch {
                row_id,
                credential_source,
            } => write!(
                f,
                "row {row_id} carries a handle for non-handle source {credential_source}"
            ),
            Self::RetentionMismatch { row_id, declared } => write!(
                f,
                "row {row_id} handle retention {declared} is not broker-resolved"
            ),
            Self::SecretBodyStored { row_id } => {
                write!(f, "row {row_id} handle stores a secret body")
            }
            Self::ContinuityMismatch {
                row_id,
                continuity,
                credential_source,
            } => write!(
                f,
                "row {row_id} continuity {continuity} disagrees with source {credential_source}"
            ),
            Self::ReachabilityContradiction {
                row_id,
                reachability,
            } => write!(
                f,
                "row {row_id} reachability {reachability} contradicts the credential state"
            ),
            Self::SourceOwnerMismatch {
                row_id,
                source_class,
            } => write!(
                f,
                "row {row_id} mirror owner is inconsistent for source {source_class}"
            ),
            Self::GenericCollapseMessage { row_id, message } => write!(
                f,
                "row {row_id} renders forbidden generic message {message}"
            ),
            Self::MissingRequiredAction { row_id, action } => {
                write!(f, "row {row_id} does not offer required action {action}")
            }
            Self::ActionNotKeyboardComplete { row_id, action } => {
                write!(f, "row {row_id} action {action} is not keyboard-complete")
            }
            Self::DuplicateAction { row_id, action } => {
                write!(f, "row {row_id} offers action {action} more than once")
            }
            Self::MultipleCurrentProfiles {
                source_class,
                redacted_source_label,
            } => write!(
                f,
                "source {source_class} ({redacted_source_label}) has more than one current profile"
            ),
            Self::RawUrlLeak { id, field_name } => {
                write!(f, "{id} field {field_name} leaks a raw URL")
            }
            Self::UnboundSource {
                row_id,
                source_class,
            } => write!(
                f,
                "row {row_id} source {source_class} has no frozen registry cell"
            ),
            Self::UnboundLabel { row_id, label } => write!(
                f,
                "row {row_id} surfaces label {label} with no frozen state row"
            ),
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for RegistryAuthFlowsViolation {}

/// Loads the embedded registry-auth-flows packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`RegistryAuthFlows`].
pub fn current_registry_auth_flows() -> Result<RegistryAuthFlows, serde_json::Error> {
    serde_json::from_str(REGISTRY_AUTH_FLOWS_JSON)
}

#[cfg(test)]
mod tests;
