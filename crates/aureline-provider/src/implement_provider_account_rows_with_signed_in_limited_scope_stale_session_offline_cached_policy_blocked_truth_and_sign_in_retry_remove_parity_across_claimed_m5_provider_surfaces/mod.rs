//! One reusable M5 provider primitive — the provider-account row — so a user can tell,
//! from the row alone, whether Aureline can currently read, write, or only inspect cached
//! provider state before any live mutation is attempted.
//!
//! Aureline's frozen provider-account / mapping / offline-capture component matrix
//! ([`crate::freeze_the_m5_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_component_matrix`])
//! names the provider-account row as one governed component family and freezes its
//! controlled vocabulary — the provider identity classes, the exact account connection
//! states (`not_configured`, `signed_in`, `limited_scope`, `stale_session`,
//! `offline_cached_read`, `policy_blocked`), the tenant scopes, and the effective write
//! scopes, plus the surface families, the deployment lines, the consumer surfaces, the
//! accessibility routes, the qualification classes, and the downgrade triggers. This
//! module *implements* that contract as one reusable resolver so a user can tell — from
//! the account row alone — which provider identity the row represents, its current
//! connection state, its tenant/org scope, its effective write scope, its token/session
//! freshness, and, above all, exactly whether Aureline can read live, write, or only
//! inspect cached provider state, without ever collapsing the six connection states into
//! one generic "connected" chip or letting a stale/offline cached read read as a live
//! write-capable session.
//!
//! The module has one resolver:
//!
//! 1. [`resolve_provider_account_row`] — takes one account's provider identity class,
//!    connection state, tenant scope, effective write scope, session freshness, its
//!    local-draft flag, opaque account label, and opaque stable account identity, and
//!    produces one [`M5ResolvedProviderAccountRow`] carrying the derived row posture (a
//!    not-configured, signed-in, limited-scope, stale-session, offline-cached-read, or
//!    policy-blocked row), the derived access capability (read-and-write, limited
//!    read/write, read-only-live, cached-inspect-only, or no-access), whether Aureline can
//!    read live / write / only inspect cached state, and the bounded reveal-scope /
//!    sign-in / retry-auth / remove-account / export actions. It never masks the
//!    connection state or write scope, never collapses the states into a generic connected
//!    label, never lets a cached-only read present as a live read/write, and never forces
//!    blind credential re-entry — retry, re-auth, and remove always preserve local drafts
//!    and support/export continuity.
//!
//! A single parity matrix — [`M5ProviderAccountRowPacket`] — binds one row per claimed M5
//! provider surface consumer (the account-settings panel, the provider status bar, the
//! connection picker, the headless/CLI accounts surface, and the support account export)
//! to the shared account-row anatomy, the same identity classes, connection states, tenant
//! scopes, write scopes, session-freshness states, row postures, access capabilities,
//! bounded actions, export fields, and non-visual accessibility routes, so the connection
//! / scope / freshness / access vocabulary stays identical across desktop, headless/export,
//! and support consumers.
//!
//! The provider identity class ([`M5ProviderIdentityClass`]), account connection state
//! ([`M5AccountConnectionState`]), tenant scope ([`M5TenantScopeClass`]), effective write
//! scope ([`M5ProviderWriteScope`]), surface family ([`M5ProviderSurfaceFamily`]),
//! deployment line ([`M5ProviderDeploymentLine`]), consumer surface
//! ([`M5ProviderConsumerSurface`]), accessibility route
//! ([`M5ProviderAccessibilityRoute`]), qualification class
//! ([`M5ProviderQualificationClass`]), and downgrade trigger
//! ([`M5ProviderDowngradeTrigger`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the account
//! row itself: the session-freshness states, its provider-surface consumers, its anatomy
//! parts, its derived row posture, its access capability, its bounded actions, and its
//! export fields. No M5 provider surface invents a second account-row grammar.
//!
//! Raw comment bodies, pasted paths, credentials, and private endpoints stay outside the
//! export boundary; every account label and account identity is carried only as an opaque,
//! export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_provider_account_row_connection_picker_preview_narrowed,
    seeded_m5_provider_account_row_headless_cli_accounts_beta_narrowed,
    seeded_m5_provider_account_row_packet, M5_PROVIDER_ACCOUNT_ROW_PACKET_ID,
};

// The provider identity class, account connection state, tenant scope, effective write
// scope, surface family, deployment line, consumer surface, accessibility route,
// qualification class, and downgrade triggers are frozen once, in the provider-account /
// offline-capture component matrix. This primitive reuses them verbatim so it never
// invents a parallel provider-account vocabulary.
pub use crate::freeze_the_m5_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_component_matrix::{
    M5AccountConnectionState, M5ProviderAccessibilityRoute, M5ProviderConsumerSurface,
    M5ProviderDeploymentLine, M5ProviderDowngradeTrigger, M5ProviderIdentityClass,
    M5ProviderQualificationClass, M5ProviderSurfaceFamily, M5ProviderWriteScope,
    M5TenantScopeClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ProviderAccountRowPacket`].
pub const M5_PROVIDER_ACCOUNT_ROW_RECORD_KIND: &str =
    "implement_m5_provider_account_rows_with_signed_in_limited_scope_stale_session_offline_cached_policy_blocked_truth_and_sign_in_retry_remove_parity_across_claimed_m5_provider_surfaces";

/// Schema version for M5 provider-account-row records.
pub const M5_PROVIDER_ACCOUNT_ROW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the account-row boundary schema.
pub const M5_PROVIDER_ACCOUNT_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-provider-account-row.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_PROVIDER_ACCOUNT_ROW_DOC_REF: &str =
    "docs/providers/m5_provider_account_row_primitive.md";

/// Repo-relative path of the frozen provider-account / offline-capture component matrix
/// this primitive narrows from.
pub const M5_PROVIDER_ACCOUNT_ROW_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-provider-account-offline-capture-component-matrix.schema.json";

/// Repo-relative path of the connected-account-record contract this primitive binds its
/// identity / connection truth against.
pub const M5_PROVIDER_ACCOUNT_ROW_CONNECTED_ACCOUNT_REF: &str =
    "schemas/providers/connected_account_record.schema.json";

/// Repo-relative path of the provider-account-scope contract this primitive binds its
/// tenant / effective-write-scope truth against.
pub const M5_PROVIDER_ACCOUNT_ROW_ACCOUNT_SCOPE_REF: &str =
    "schemas/providers/provider_account_scope.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_PROVIDER_ACCOUNT_ROW_FIXTURE_DIR: &str =
    "fixtures/ui/m5-provider-account-row-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PROVIDER_ACCOUNT_ROW_ARTIFACT_REF: &str =
    "artifacts/release/m5-provider-account-row-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_PROVIDER_ACCOUNT_ROW_CSV_REF: &str =
    "artifacts/release/m5-provider-account-row-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_PROVIDER_ACCOUNT_ROW_REPORT_REF: &str =
    "artifacts/design/m5-provider-account-row-primitive.md";

/// Controlled token/session freshness — how current the account's credential is, so a row
/// never leaves session freshness implicit and a stale, expired, or revoked session is
/// never presented as a fresh one. This is the freshness dimension the implementation
/// requirements call out alongside connection state and write scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderAccountSessionFreshness {
    /// The session / token is current and healthy.
    FreshSession,
    /// The session / token is valid but near expiry.
    NearExpiry,
    /// The session / token has expired and needs re-authentication.
    ExpiredSession,
    /// The token was revoked upstream.
    RevokedToken,
    /// The account has never been authenticated.
    NeverAuthenticated,
    /// The session freshness could not be determined.
    UnknownFreshness,
}

impl M5ProviderAccountSessionFreshness {
    /// Every session-freshness state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FreshSession,
        Self::NearExpiry,
        Self::ExpiredSession,
        Self::RevokedToken,
        Self::NeverAuthenticated,
        Self::UnknownFreshness,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshSession => "fresh_session",
            Self::NearExpiry => "near_expiry",
            Self::ExpiredSession => "expired_session",
            Self::RevokedToken => "revoked_token",
            Self::NeverAuthenticated => "never_authenticated",
            Self::UnknownFreshness => "unknown_freshness",
        }
    }

    /// True when the session is current and healthy.
    pub const fn is_fresh(self) -> bool {
        matches!(self, Self::FreshSession)
    }

    /// True when the session should be refreshed / re-authenticated (near-expiry, expired,
    /// revoked, or unknown), so a retry / re-auth affordance is offered before a live write.
    pub const fn needs_refresh(self) -> bool {
        matches!(
            self,
            Self::NearExpiry | Self::ExpiredSession | Self::RevokedToken | Self::UnknownFreshness
        )
    }
}

/// One claimed M5 provider-surface consumer that renders the shared provider-account row.
/// These are the consumers the acceptance criteria name — the account-settings panel, the
/// provider status bar, the connection picker, the headless/CLI accounts surface, and the
/// support account export — so the same account-row grammar works across every claimed
/// provider surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderAccountConsumerSurface {
    /// The account-settings panel surface.
    AccountSettingsPanel,
    /// The provider status-bar surface.
    ProviderStatusBar,
    /// The connection-picker surface.
    ConnectionPicker,
    /// The headless / CLI accounts surface.
    HeadlessCliAccounts,
    /// The support account-export surface.
    SupportAccountExport,
}

impl M5ProviderAccountConsumerSurface {
    /// Every claimed provider-surface consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::AccountSettingsPanel,
        Self::ProviderStatusBar,
        Self::ConnectionPicker,
        Self::HeadlessCliAccounts,
        Self::SupportAccountExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AccountSettingsPanel => "account_settings_panel",
            Self::ProviderStatusBar => "provider_status_bar",
            Self::ConnectionPicker => "connection_picker",
            Self::HeadlessCliAccounts => "headless_cli_accounts",
            Self::SupportAccountExport => "support_account_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AccountSettingsPanel => "Account Settings Panel",
            Self::ProviderStatusBar => "Provider Status Bar",
            Self::ConnectionPicker => "Connection Picker",
            Self::HeadlessCliAccounts => "Headless / CLI Accounts",
            Self::SupportAccountExport => "Support Account Export",
        }
    }
}

/// The derived posture of a provider-account row — the resolver's verdict about the
/// account's current connection standing. Derived one-to-one from the frozen account
/// connection state, so the six governed states are never collapsed into one generic
/// "connected" chip and a signed-in, healthy account never reads the same as a stale,
/// offline, or policy-blocked one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderAccountRowPosture {
    /// No provider account is configured; nothing can be read or written yet.
    NotConfiguredRow,
    /// The account is signed in with full scope — the highest-trust row.
    SignedInRow,
    /// The account is signed in but scope is limited.
    LimitedScopeRow,
    /// The session is stale and needs re-authentication before a live write.
    StaleSessionRow,
    /// Only an offline cached read is available.
    OfflineCachedReadRow,
    /// The account is blocked by policy.
    PolicyBlockedRow,
}

impl M5ProviderAccountRowPosture {
    /// Every row posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotConfiguredRow,
        Self::SignedInRow,
        Self::LimitedScopeRow,
        Self::StaleSessionRow,
        Self::OfflineCachedReadRow,
        Self::PolicyBlockedRow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotConfiguredRow => "not_configured_row",
            Self::SignedInRow => "signed_in_row",
            Self::LimitedScopeRow => "limited_scope_row",
            Self::StaleSessionRow => "stale_session_row",
            Self::OfflineCachedReadRow => "offline_cached_read_row",
            Self::PolicyBlockedRow => "policy_blocked_row",
        }
    }

    /// True only for a fully signed-in row — the one posture that may present a healthy,
    /// full-trust connection. Every other posture deliberately signals a caveat.
    pub const fn shows_full_connection(self) -> bool {
        matches!(self, Self::SignedInRow)
    }

    /// True when the row needs operator attention before it is trusted as write-ready.
    pub const fn needs_attention(self) -> bool {
        !matches!(self, Self::SignedInRow)
    }

    /// True when the row's standing is recoverable by re-authentication (stale session),
    /// so the account is not lost, only in need of refresh.
    pub const fn needs_reauth(self) -> bool {
        matches!(self, Self::StaleSessionRow)
    }
}

/// Controlled access capability — exactly what Aureline can do with the provider right now
/// from this account, so the acceptance-criterion promise that a user can tell whether
/// Aureline can read, write, or only inspect cached state is explicit and never inferred.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderAccountAccessCapability {
    /// Live read and full write.
    CanReadAndWrite,
    /// Live read and a limited (comment/status) write.
    CanReadWriteLimited,
    /// Live read only, no write.
    CanReadOnlyLive,
    /// Only a cached read can be inspected; no live read or write.
    CachedInspectOnly,
    /// No access at all — nothing can be read, written, or inspected.
    NoAccess,
}

impl M5ProviderAccountAccessCapability {
    /// Every access capability, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CanReadAndWrite,
        Self::CanReadWriteLimited,
        Self::CanReadOnlyLive,
        Self::CachedInspectOnly,
        Self::NoAccess,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanReadAndWrite => "can_read_and_write",
            Self::CanReadWriteLimited => "can_read_write_limited",
            Self::CanReadOnlyLive => "can_read_only_live",
            Self::CachedInspectOnly => "cached_inspect_only",
            Self::NoAccess => "no_access",
        }
    }

    /// True when Aureline can read live provider state from this account.
    pub const fn can_read_live(self) -> bool {
        matches!(
            self,
            Self::CanReadAndWrite | Self::CanReadWriteLimited | Self::CanReadOnlyLive
        )
    }

    /// True when Aureline can write to the provider from this account (full or limited).
    pub const fn can_write(self) -> bool {
        matches!(self, Self::CanReadAndWrite | Self::CanReadWriteLimited)
    }

    /// True when only a cached read can be inspected — no live read or write is possible.
    pub const fn only_inspect_cached(self) -> bool {
        matches!(self, Self::CachedInspectOnly)
    }
}

/// One bounded action a provider-account row offers, so a row never hides its reveal-scope
/// / sign-in / retry-auth / remove-account / export affordances, and a user can recover a
/// connection without leaving the row or blindly re-entering credentials.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderAccountRowAction {
    /// Reveal the account's identity, connection state, tenant scope, write scope, and
    /// session freshness.
    RevealScope,
    /// Sign in a not-yet-configured account.
    SignInAccount,
    /// Retry / re-authenticate a configured-but-degraded account.
    RetryAuth,
    /// Remove the connected account.
    RemoveAccount,
    /// Export the account row as provider evidence.
    ExportRow,
}

impl M5ProviderAccountRowAction {
    /// Every account-row action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RevealScope,
        Self::SignInAccount,
        Self::RetryAuth,
        Self::RemoveAccount,
        Self::ExportRow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealScope => "reveal_scope",
            Self::SignInAccount => "sign_in_account",
            Self::RetryAuth => "retry_auth",
            Self::RemoveAccount => "remove_account",
            Self::ExportRow => "export_row",
        }
    }
}

/// Controlled provider-account-row anatomy part the shared row surfaces. The parts in
/// [`M5ProviderAccountRowAnatomyPart::MANDATORY`] are required on every row so the provider
/// identity, connection state, effective write scope, access capability, and the account
/// action cue are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderAccountRowAnatomyPart {
    /// The provider identity + stable account cue.
    ProviderIdentityCue,
    /// The current connection-state cue.
    ConnectionStateCue,
    /// The tenant / org scope cue.
    TenantScopeCue,
    /// The effective write-scope cue.
    WriteScopeCue,
    /// The token / session freshness cue.
    SessionFreshnessCue,
    /// The read / write / inspect access-capability cue.
    AccessCapabilityCue,
    /// The sign-in / retry / remove action cue.
    AccountActionCue,
    /// The non-visual keyboard-route cue.
    KeyboardRouteCue,
}

impl M5ProviderAccountRowAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ProviderIdentityCue,
        Self::ConnectionStateCue,
        Self::TenantScopeCue,
        Self::WriteScopeCue,
        Self::SessionFreshnessCue,
        Self::AccessCapabilityCue,
        Self::AccountActionCue,
        Self::KeyboardRouteCue,
    ];

    /// The anatomy parts every row must render.
    pub const MANDATORY: [Self; 5] = [
        Self::ProviderIdentityCue,
        Self::ConnectionStateCue,
        Self::WriteScopeCue,
        Self::AccessCapabilityCue,
        Self::AccountActionCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderIdentityCue => "provider_identity_cue",
            Self::ConnectionStateCue => "connection_state_cue",
            Self::TenantScopeCue => "tenant_scope_cue",
            Self::WriteScopeCue => "write_scope_cue",
            Self::SessionFreshnessCue => "session_freshness_cue",
            Self::AccessCapabilityCue => "access_capability_cue",
            Self::AccountActionCue => "account_action_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the row export carries so provider-account-row truth is reconstructable. The
/// fields in [`M5ProviderAccountRowExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderAccountRowExportField {
    /// The provider identity class.
    ProviderIdentity,
    /// The account connection state.
    ConnectionState,
    /// The tenant scope.
    TenantScope,
    /// The effective write scope.
    WriteScope,
    /// The token / session freshness.
    SessionFreshness,
    /// The derived access capability.
    AccessCapability,
    /// The derived row posture.
    RowPosture,
    /// The bounded available actions.
    AvailableActions,
}

impl M5ProviderAccountRowExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ProviderIdentity,
        Self::ConnectionState,
        Self::TenantScope,
        Self::WriteScope,
        Self::SessionFreshness,
        Self::AccessCapability,
        Self::RowPosture,
        Self::AvailableActions,
    ];

    /// The export fields every row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::ProviderIdentity,
        Self::ConnectionState,
        Self::WriteScope,
        Self::AccessCapability,
        Self::RowPosture,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderIdentity => "provider_identity",
            Self::ConnectionState => "connection_state",
            Self::TenantScope => "tenant_scope",
            Self::WriteScope => "write_scope",
            Self::SessionFreshness => "session_freshness",
            Self::AccessCapability => "access_capability",
            Self::RowPosture => "row_posture",
            Self::AvailableActions => "available_actions",
        }
    }
}

// ---- provider-account-row resolver --------------------------------------

/// The full input to the provider-account-row resolver for one account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderAccountRowResolutionInput {
    /// The provider identity class.
    pub identity_class: M5ProviderIdentityClass,
    /// The account connection state.
    pub connection_state: M5AccountConnectionState,
    /// The tenant / org scope.
    pub tenant_scope: M5TenantScopeClass,
    /// The effective write scope.
    pub write_scope: M5ProviderWriteScope,
    /// The token / session freshness.
    pub session_freshness: M5ProviderAccountSessionFreshness,
    /// True when local drafts / queued work exist behind this account, so retry / remove
    /// must preserve them.
    pub has_local_drafts: bool,
    /// The opaque user-facing account label (must be non-empty).
    pub account_label: String,
    /// The opaque stable account identity (must be non-empty).
    pub account_identity_ref: String,
}

/// The resolved provider-account-row truth for one account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedProviderAccountRow {
    /// The provider identity class.
    pub identity_class: M5ProviderIdentityClass,
    /// The account connection state.
    pub connection_state: M5AccountConnectionState,
    /// The tenant / org scope.
    pub tenant_scope: M5TenantScopeClass,
    /// The effective write scope.
    pub write_scope: M5ProviderWriteScope,
    /// The token / session freshness.
    pub session_freshness: M5ProviderAccountSessionFreshness,
    /// The opaque account label, preserved exactly from the input.
    pub account_label: String,
    /// The opaque stable account identity, preserved exactly from the input.
    pub account_identity_ref: String,
    /// The derived row posture.
    pub row_posture: M5ProviderAccountRowPosture,
    /// The derived access capability.
    pub access_capability: M5ProviderAccountAccessCapability,
    /// The bounded actions this row offers.
    pub available_actions: Vec<M5ProviderAccountRowAction>,
    /// True when Aureline can read live provider state from this account.
    pub can_read_live: bool,
    /// True when Aureline can write to the provider from this account.
    pub can_write: bool,
    /// True when only a cached read can be inspected — no live read or write.
    pub only_inspect_cached: bool,
    /// True when local drafts / queued work exist behind this account, preserved from the
    /// input.
    pub has_local_drafts: bool,
    /// True when the row needs operator attention before it is trusted as write-ready.
    pub needs_attention: bool,
    /// True when the account's standing is recoverable by re-authentication.
    pub needs_reauth: bool,
    /// Retry / re-auth / remove flows preserve local drafts. ALWAYS `true`.
    pub preserves_local_drafts: bool,
    /// Retry / re-auth / remove flows preserve support / export continuity. ALWAYS `true`.
    pub preserves_support_export_continuity: bool,
    /// The row never forces blind credential re-entry. ALWAYS `false`.
    pub requires_blind_credential_reentry: bool,
}

/// Errors returned by [`resolve_provider_account_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ProviderAccountRowResolutionError {
    /// The account label was empty.
    EmptyAccountLabel,
    /// The account identity ref was empty.
    EmptyAccountIdentity,
    /// A row descriptor carried forbidden material.
    ForbiddenAccountMaterial,
}

impl M5ProviderAccountRowResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyAccountLabel => "empty_account_label",
            Self::EmptyAccountIdentity => "empty_account_identity",
            Self::ForbiddenAccountMaterial => "forbidden_account_material",
        }
    }
}

impl fmt::Display for M5ProviderAccountRowResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "provider account row resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ProviderAccountRowResolutionError {}

/// Resolves one provider-account row from its declared account state.
///
/// The derived row posture is taken one-to-one from the frozen account connection state, so
/// the six governed states never collapse into one generic "connected" chip. The access
/// capability is derived from the connection state and the effective write scope so a user
/// can tell — from the row alone — whether Aureline can read live, write, or only inspect a
/// cached read: a not-configured or policy-blocked account has no access, a stale or
/// offline-cached account can only inspect a cached read, a signed-in account reads and
/// writes to the extent of its write scope, and a limited-scope account reads and writes
/// only within its reduced scope. The row always offers reveal-scope and export; it offers
/// sign-in when nothing is configured, retry / re-auth when a configured account is degraded
/// or its session needs refresh, and remove whenever an account is configured — and every
/// one of those flows preserves local drafts and support / export continuity rather than
/// forcing blind credential re-entry.
pub fn resolve_provider_account_row(
    input: &M5ProviderAccountRowResolutionInput,
) -> Result<M5ResolvedProviderAccountRow, M5ProviderAccountRowResolutionError> {
    if input.account_label.trim().is_empty() {
        return Err(M5ProviderAccountRowResolutionError::EmptyAccountLabel);
    }
    if input.account_identity_ref.trim().is_empty() {
        return Err(M5ProviderAccountRowResolutionError::EmptyAccountIdentity);
    }
    if value_repr_is_forbidden(&input.account_label)
        || value_repr_is_forbidden(&input.account_identity_ref)
    {
        return Err(M5ProviderAccountRowResolutionError::ForbiddenAccountMaterial);
    }

    let row_posture = derive_row_posture(input.connection_state);
    let access_capability = derive_access_capability(input.connection_state, input.write_scope);
    let available_actions = derive_account_actions(input.connection_state, input.session_freshness);

    Ok(M5ResolvedProviderAccountRow {
        identity_class: input.identity_class,
        connection_state: input.connection_state,
        tenant_scope: input.tenant_scope,
        write_scope: input.write_scope,
        session_freshness: input.session_freshness,
        account_label: input.account_label.clone(),
        account_identity_ref: input.account_identity_ref.clone(),
        row_posture,
        access_capability,
        available_actions,
        can_read_live: access_capability.can_read_live(),
        can_write: access_capability.can_write(),
        only_inspect_cached: access_capability.only_inspect_cached(),
        has_local_drafts: input.has_local_drafts,
        needs_attention: row_posture.needs_attention(),
        needs_reauth: row_posture.needs_reauth(),
        // The acceptance criterion: retry / re-auth / remove preserve local drafts and
        // support/export continuity and never force blind credential re-entry.
        preserves_local_drafts: true,
        preserves_support_export_continuity: true,
        requires_blind_credential_reentry: false,
    })
}

/// Derives the row posture one-to-one from the frozen account connection state, so no
/// surface collapses the six governed states into a generic "connected" chip.
fn derive_row_posture(connection_state: M5AccountConnectionState) -> M5ProviderAccountRowPosture {
    use M5AccountConnectionState as State;
    use M5ProviderAccountRowPosture as Posture;
    match connection_state {
        State::NotConfigured => Posture::NotConfiguredRow,
        State::SignedIn => Posture::SignedInRow,
        State::LimitedScope => Posture::LimitedScopeRow,
        State::StaleSession => Posture::StaleSessionRow,
        State::OfflineCachedRead => Posture::OfflineCachedReadRow,
        State::PolicyBlocked => Posture::PolicyBlockedRow,
    }
}

/// Derives the access capability from the connection state and the effective write scope,
/// so a user can tell whether Aureline can read live, write, or only inspect cached state.
fn derive_access_capability(
    connection_state: M5AccountConnectionState,
    write_scope: M5ProviderWriteScope,
) -> M5ProviderAccountAccessCapability {
    use M5AccountConnectionState as State;
    use M5ProviderAccountAccessCapability as Cap;
    use M5ProviderWriteScope as Scope;
    match connection_state {
        // Nothing configured or a hard policy block: no access at all.
        State::NotConfigured | State::PolicyBlocked => Cap::NoAccess,
        // A stale session or an explicit offline cached read can only inspect the cache; a
        // live read or write requires a fresh session / reachability.
        State::StaleSession | State::OfflineCachedRead => Cap::CachedInspectOnly,
        // A fully signed-in account reads and writes to the extent of its write scope.
        State::SignedIn => match write_scope {
            Scope::FullWrite => Cap::CanReadAndWrite,
            Scope::CommentOnly | Scope::StatusOnly => Cap::CanReadWriteLimited,
            Scope::ReadOnly | Scope::NoWrite | Scope::ScopeUnknown => Cap::CanReadOnlyLive,
        },
        // A limited-scope account reads and writes only within its reduced scope; a full
        // write scope is capped to limited because the account itself is scope-limited.
        State::LimitedScope => match write_scope {
            Scope::FullWrite | Scope::CommentOnly | Scope::StatusOnly => Cap::CanReadWriteLimited,
            Scope::ReadOnly | Scope::NoWrite | Scope::ScopeUnknown => Cap::CanReadOnlyLive,
        },
    }
}

/// Derives the bounded action set from the connection state and session freshness.
///
/// Reveal-scope and export are always offered; sign-in is offered only when nothing is
/// configured; retry / re-auth is offered when a configured account is degraded or its
/// session needs refresh; remove is offered whenever an account is configured.
fn derive_account_actions(
    connection_state: M5AccountConnectionState,
    session_freshness: M5ProviderAccountSessionFreshness,
) -> Vec<M5ProviderAccountRowAction> {
    use M5ProviderAccountRowAction as Action;
    let posture = derive_row_posture(connection_state);
    let mut actions = vec![Action::RevealScope];
    if matches!(connection_state, M5AccountConnectionState::NotConfigured) {
        actions.push(Action::SignInAccount);
    } else {
        if posture.needs_attention() || session_freshness.needs_refresh() {
            actions.push(Action::RetryAuth);
        }
        actions.push(Action::RemoveAccount);
    }
    actions.push(Action::ExportRow);
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked provider-account-row resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderAccountRowResolutionCase {
    /// The resolver input.
    pub input: M5ProviderAccountRowResolutionInput,
    /// The resolved truth. Must equal `resolve_provider_account_row(&input)`.
    pub resolved: M5ResolvedProviderAccountRow,
}

impl M5ProviderAccountRowResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5ProviderAccountRowResolutionInput) -> Self {
        let resolved =
            resolve_provider_account_row(&input).expect("seed account row case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_provider_account_row(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved account identity preserves the input identity exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.account_identity_ref == self.input.account_identity_ref
            && self.resolved.account_label == self.input.account_label
    }

    /// True when the resolved case preserves local drafts and support/export continuity and
    /// never forces blind credential re-entry.
    pub fn preserves_draft_continuity(&self) -> bool {
        self.resolved.preserves_local_drafts
            && self.resolved.preserves_support_export_continuity
            && !self.resolved.requires_blind_credential_reentry
    }
}

/// One row in the primitive matrix: one provider-surface consumer bound to the shared
/// account-row anatomy, identity classes, connection states, tenant scopes, write scopes,
/// session-freshness states, row postures, access capabilities, bounded actions, export
/// fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderAccountConsumerRow {
    /// Provider-surface consumer family.
    pub consumer_surface: M5ProviderAccountConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5ProviderQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 provider surface families that render / consume this row.
    pub surface_families: Vec<M5ProviderSurfaceFamily>,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5ProviderDeploymentLine>,
    /// Anatomy parts this row renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5ProviderAccountRowAnatomyPart>,
    /// Provider identity classes this consumer distinguishes.
    pub identity_classes: Vec<M5ProviderIdentityClass>,
    /// Account connection states this consumer distinguishes.
    pub connection_states: Vec<M5AccountConnectionState>,
    /// Tenant scopes this consumer distinguishes.
    pub tenant_scopes: Vec<M5TenantScopeClass>,
    /// Write scopes this consumer distinguishes.
    pub write_scopes: Vec<M5ProviderWriteScope>,
    /// Session-freshness states this consumer distinguishes.
    pub session_freshness_states: Vec<M5ProviderAccountSessionFreshness>,
    /// Row postures this consumer distinguishes.
    pub row_postures: Vec<M5ProviderAccountRowPosture>,
    /// Access capabilities this consumer distinguishes.
    pub access_capabilities: Vec<M5ProviderAccountAccessCapability>,
    /// Bounded account-row actions this consumer offers.
    pub row_actions: Vec<M5ProviderAccountRowAction>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5ProviderAccountRowExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5ProviderAccessibilityRoute>,
    /// Provider subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5ProviderConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5ProviderDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked account-row resolutions proving the resolver on this consumer.
    pub account_examples: Vec<M5ProviderAccountRowResolutionCase>,
    /// Hard invariant: this consumer never masks its connection state or write scope. MUST
    /// be `false`.
    pub masks_connection_or_scope: bool,
    /// Hard invariant: this consumer never collapses the six connection states into one
    /// generic connected label. MUST be `false`.
    pub collapses_states_into_generic_connected: bool,
    /// Hard invariant: this consumer never renders a cached-only read with live certainty.
    /// MUST be `false`.
    pub overstates_cached_as_live: bool,
    /// Hard invariant: this consumer never forces blind credential re-entry. MUST be
    /// `false`.
    pub forces_blind_credential_reentry: bool,
}

impl M5ProviderAccountConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ProviderAccountRowAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ProviderAccountRowAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export(&self) -> bool {
        let present: BTreeSet<M5ProviderAccountRowExportField> =
            self.export_fields.iter().copied().collect();
        M5ProviderAccountRowExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_connection_or_scope
            && !self.collapses_states_into_generic_connected
            && !self.overstates_cached_as_live
            && !self.forces_blind_credential_reentry
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderAccountRowVocabularySet {
    /// Provider-surface-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Row-posture tokens.
    pub row_postures: Vec<String>,
    /// Access-capability tokens.
    pub access_capabilities: Vec<String>,
    /// Row-action tokens.
    pub row_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Session-freshness tokens.
    pub session_freshness_states: Vec<String>,
    /// Provider-identity-class tokens (reused from the frozen matrix).
    pub identity_classes: Vec<String>,
    /// Account-connection-state tokens (reused from the frozen matrix).
    pub connection_states: Vec<String>,
    /// Tenant-scope tokens (reused from the frozen matrix).
    pub tenant_scopes: Vec<String>,
    /// Write-scope tokens (reused from the frozen matrix).
    pub write_scopes: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5ProviderAccountRowVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5ProviderAccountConsumerSurface::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5ProviderAccountRowAnatomyPart::ALL, |v| v.as_str()),
            row_postures: tokens(&M5ProviderAccountRowPosture::ALL, |v| v.as_str()),
            access_capabilities: tokens(&M5ProviderAccountAccessCapability::ALL, |v| v.as_str()),
            row_actions: tokens(&M5ProviderAccountRowAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ProviderAccountRowExportField::ALL, |v| v.as_str()),
            session_freshness_states: tokens(&M5ProviderAccountSessionFreshness::ALL, |v| {
                v.as_str()
            }),
            identity_classes: tokens(&M5ProviderIdentityClass::ALL, |v| v.as_str()),
            connection_states: tokens(&M5AccountConnectionState::ALL, |v| v.as_str()),
            tenant_scopes: tokens(&M5TenantScopeClass::ALL, |v| v.as_str()),
            write_scopes: tokens(&M5ProviderWriteScope::ALL, |v| v.as_str()),
            surface_families: tokens(&M5ProviderSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5ProviderDeploymentLine::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ProviderAccessibilityRoute::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderAccountRowGovernanceReview {
    /// The account row shows its provider identity.
    pub account_row_shows_provider_identity: bool,
    /// The account row shows its connection state.
    pub account_row_shows_connection_state: bool,
    /// The account row shows its tenant / org scope.
    pub account_row_shows_tenant_scope: bool,
    /// The account row shows its effective write scope.
    pub account_row_shows_write_scope: bool,
    /// The account row shows its token / session freshness.
    pub account_row_shows_session_freshness: bool,
    /// The account row shows its read / write / inspect access capability.
    pub account_row_shows_access_capability: bool,
    /// The six connection states never collapse into one generic connected label.
    pub states_never_collapse_into_generic_connected: bool,
    /// A cached-only read never reads as a live read / write.
    pub cached_inspect_never_reads_as_live: bool,
    /// Retry / re-auth / remove preserve local drafts and support / export continuity.
    pub retry_remove_preserve_local_drafts: bool,
    /// Account rows keep the same truth across every deployment line.
    pub account_rows_stable_across_deployment_lines: bool,
    /// Account rows keep the same truth across desktop, headless/export, and support
    /// consumers.
    pub account_rows_stable_across_consumer_surfaces: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The support / export packet reconstructs connection, scope, and access truth.
    pub support_export_reconstructs_account_truth: bool,
    /// Later M5 rows cannot invent parallel account-row vocabulary.
    pub later_rows_cannot_invent_parallel_account_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderAccountRowConsumerProjection {
    /// Provider surfaces consume the shared account-row vocabulary.
    pub provider_surfaces_consume_account_vocabulary: bool,
    /// The row-posture resolver reads a single canonical source.
    pub row_posture_reads_single_source: bool,
    /// The access-capability derivation reads a single canonical source.
    pub access_capability_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
    /// Headless and desktop account rows read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderAccountRowProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the account row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderAccountRowReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting provider-account audit.
    pub provider_account_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ProviderAccountRowPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ProviderAccountRowPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Provider-surface rows.
    pub rows: Vec<M5ProviderAccountConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ProviderAccountRowVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ProviderAccountRowGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ProviderAccountRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ProviderAccountRowProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ProviderAccountRowReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 provider-account-row primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderAccountRowPacket {
    /// Record kind; must equal [`M5_PROVIDER_ACCOUNT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PROVIDER_ACCOUNT_ROW_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Provider-surface rows.
    pub rows: Vec<M5ProviderAccountConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ProviderAccountRowVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ProviderAccountRowGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ProviderAccountRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ProviderAccountRowProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ProviderAccountRowReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ProviderAccountRowPacket {
    /// Builds an M5 account-row-primitive packet from stable-lane input.
    pub fn new(input: M5ProviderAccountRowPacketInput) -> Self {
        Self {
            record_kind: M5_PROVIDER_ACCOUNT_ROW_RECORD_KIND.to_owned(),
            schema_version: M5_PROVIDER_ACCOUNT_ROW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 account-row-primitive invariants.
    pub fn validate(&self) -> Vec<M5ProviderAccountRowViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PROVIDER_ACCOUNT_ROW_RECORD_KIND {
            violations.push(M5ProviderAccountRowViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PROVIDER_ACCOUNT_ROW_SCHEMA_VERSION {
            violations.push(M5ProviderAccountRowViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ProviderAccountRowViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_connection_state_coverage(self, &mut violations);
        validate_access_capability_coverage(self, &mut violations);
        validate_action_coverage(self, &mut violations);
        validate_draft_continuity(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 account row primitive packet serializes"),
        ) {
            violations.push(M5ProviderAccountRowViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 account row primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per provider-surface consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy,connection_states,write_scopes,row_postures,access_capabilities,row_actions,account_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.connection_states, |v| v.as_str()),
                join_tokens(&row.write_scopes, |v| v.as_str()),
                join_tokens(&row.row_postures, |v| v.as_str()),
                join_tokens(&row.access_capabilities, |v| v.as_str()),
                join_tokens(&row.row_actions, |v| v.as_str()),
                row.account_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Provider-Account-Row Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Provider-surface consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Row postures: {}\n",
            self.vocabulary_set.row_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Access capabilities: {}\n",
            self.vocabulary_set.access_capabilities.join(", ")
        ));
        out.push_str(&format!(
            "- Connection states: {}\n",
            self.vocabulary_set.connection_states.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Provider-surface consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked rows: {}\n",
                row.account_examples.len()
            ));
            for case in &row.account_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}` / `{}`) → `{}` (access `{}`, can-write `{}`, cached-only `{}`)\n",
                    case.resolved.account_identity_ref,
                    case.resolved.connection_state.as_str(),
                    case.resolved.write_scope.as_str(),
                    case.resolved.row_posture.as_str(),
                    case.resolved.access_capability.as_str(),
                    case.resolved.can_write,
                    case.resolved.only_inspect_cached,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 account-row-primitive export.
#[derive(Debug)]
pub enum M5ProviderAccountRowArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ProviderAccountRowViolation>),
}

impl fmt::Display for M5ProviderAccountRowArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 account row primitive export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 account row primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ProviderAccountRowArtifactError {}

/// Validation failures emitted by [`M5ProviderAccountRowPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ProviderAccountRowViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required provider-surface consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A provider-surface row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A row omits one of the mandatory export fields.
    MandatoryExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked account resolutions.
    AccountExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The worked resolutions do not exercise every account connection state.
    ConnectionStateCoverageUnproven,
    /// The worked resolutions do not prove a write-capable, a read-only-live, and a
    /// cached-inspect-only row.
    AccessCapabilityCoverageUnproven,
    /// The worked resolutions do not prove the sign-in, retry, and remove actions.
    ActionCoverageUnproven,
    /// A worked resolution does not preserve local drafts and support / export continuity.
    DraftContinuityUnproven,
    /// A worked resolution does not preserve its exact account identity and label.
    IdentityPreservationUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ProviderAccountRowViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportMissing => "mandatory_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::AccountExampleMissing => "account_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::ConnectionStateCoverageUnproven => "connection_state_coverage_unproven",
            Self::AccessCapabilityCoverageUnproven => "access_capability_coverage_unproven",
            Self::ActionCoverageUnproven => "action_coverage_unproven",
            Self::DraftContinuityUnproven => "draft_continuity_unproven",
            Self::IdentityPreservationUnproven => "identity_preservation_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 account-row-primitive export.
pub fn current_stable_m5_provider_account_row_export(
) -> Result<M5ProviderAccountRowPacket, M5ProviderAccountRowArtifactError> {
    let packet: M5ProviderAccountRowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-provider-account-row-primitive-proof/support_export.json"
    )))
    .map_err(M5ProviderAccountRowArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ProviderAccountRowArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ProviderAccountRowPacket,
    violations: &mut Vec<M5ProviderAccountRowViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PROVIDER_ACCOUNT_ROW_SCHEMA_REF,
        M5_PROVIDER_ACCOUNT_ROW_DOC_REF,
        M5_PROVIDER_ACCOUNT_ROW_COMPONENT_MATRIX_REF,
        M5_PROVIDER_ACCOUNT_ROW_CONNECTED_ACCOUNT_REF,
        M5_PROVIDER_ACCOUNT_ROW_ACCOUNT_SCOPE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ProviderAccountRowViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ProviderAccountRowPacket,
    violations: &mut Vec<M5ProviderAccountRowViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ProviderAccountRowViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5ProviderAccountRowPacket,
    violations: &mut Vec<M5ProviderAccountRowViolation>,
) {
    let present: BTreeSet<M5ProviderAccountConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5ProviderAccountConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5ProviderAccountRowViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.identity_classes.is_empty()
            || row.connection_states.is_empty()
            || row.tenant_scopes.is_empty()
            || row.write_scopes.is_empty()
            || row.session_freshness_states.is_empty()
            || row.row_postures.is_empty()
            || row.access_capabilities.is_empty()
            || row.row_actions.is_empty()
            || row.export_fields.is_empty()
        {
            violations.push(M5ProviderAccountRowViolation::RowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ProviderAccountRowViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export() {
            violations.push(M5ProviderAccountRowViolation::MandatoryExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ProviderAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5ProviderAccountRowViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ProviderAccountRowViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ProviderAccountRowViolation::DowngradeTriggersMissing);
        }
        if row.account_examples.is_empty() {
            violations.push(M5ProviderAccountRowViolation::AccountExampleMissing);
        }
        if row
            .account_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5ProviderAccountRowViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ProviderAccountRowViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5ProviderAccountRowViolation::RowInvariantViolated);
        }
    }
}

/// Every account connection state must be exercised by some worked resolution — the
/// implementation requirement that rows distinguish not-configured, signed-in,
/// limited-scope, stale-session, offline-cached-read, and policy-blocked without collapsing
/// them into one generic connected state.
fn validate_connection_state_coverage(
    packet: &M5ProviderAccountRowPacket,
    violations: &mut Vec<M5ProviderAccountRowViolation>,
) {
    let exercised: BTreeSet<M5AccountConnectionState> = packet
        .rows
        .iter()
        .flat_map(|row| row.account_examples.iter())
        .map(|case| case.resolved.connection_state)
        .collect();
    let covered = M5AccountConnectionState::ALL
        .iter()
        .all(|state| exercised.contains(state));
    if !covered {
        violations.push(M5ProviderAccountRowViolation::ConnectionStateCoverageUnproven);
    }
}

/// At least one worked resolution must prove a write-capable row, one a read-only-live row,
/// and one a cached-inspect-only row — the acceptance-criterion example that a user can tell
/// whether Aureline can read, write, or only inspect cached provider state from the row
/// alone.
fn validate_access_capability_coverage(
    packet: &M5ProviderAccountRowPacket,
    violations: &mut Vec<M5ProviderAccountRowViolation>,
) {
    let cases = || {
        packet
            .rows
            .iter()
            .flat_map(|row| row.account_examples.iter())
    };
    let has_write = cases().any(|case| case.resolved.can_write);
    let has_read_only_live =
        cases().any(|case| case.resolved.can_read_live && !case.resolved.can_write);
    let has_cached_inspect = cases().any(|case| case.resolved.only_inspect_cached);
    if !(has_write && has_read_only_live && has_cached_inspect) {
        violations.push(M5ProviderAccountRowViolation::AccessCapabilityCoverageUnproven);
    }
}

/// At least one worked resolution must prove each of the sign-in, retry / re-auth, and
/// remove actions — the implementation requirement that a user can recover a connection
/// without leaving the row.
fn validate_action_coverage(
    packet: &M5ProviderAccountRowPacket,
    violations: &mut Vec<M5ProviderAccountRowViolation>,
) {
    let cases = || {
        packet
            .rows
            .iter()
            .flat_map(|row| row.account_examples.iter())
    };
    let has_sign_in = cases().any(|case| {
        case.resolved
            .available_actions
            .contains(&M5ProviderAccountRowAction::SignInAccount)
    });
    let has_retry = cases().any(|case| {
        case.resolved
            .available_actions
            .contains(&M5ProviderAccountRowAction::RetryAuth)
    });
    let has_remove = cases().any(|case| {
        case.resolved
            .available_actions
            .contains(&M5ProviderAccountRowAction::RemoveAccount)
    });
    if !(has_sign_in && has_retry && has_remove) {
        violations.push(M5ProviderAccountRowViolation::ActionCoverageUnproven);
    }
}

/// Every worked resolution must preserve local drafts and support / export continuity and
/// never force blind credential re-entry — the acceptance criterion that retry / re-auth /
/// remove flows are non-destructive.
fn validate_draft_continuity(
    packet: &M5ProviderAccountRowPacket,
    violations: &mut Vec<M5ProviderAccountRowViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.account_examples.iter())
        .all(|case| case.preserves_draft_continuity());
    if !preserved {
        violations.push(M5ProviderAccountRowViolation::DraftContinuityUnproven);
    }
}

/// Every worked resolution must preserve its exact account identity and label — the
/// invariant that the account row never rewrites the user's provider identity.
fn validate_identity_preservation(
    packet: &M5ProviderAccountRowPacket,
    violations: &mut Vec<M5ProviderAccountRowViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.account_examples.iter())
        .all(|case| case.preserves_identity());
    if !preserved {
        violations.push(M5ProviderAccountRowViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5ProviderAccountRowPacket,
    violations: &mut Vec<M5ProviderAccountRowViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.account_row_shows_provider_identity,
        review.account_row_shows_connection_state,
        review.account_row_shows_tenant_scope,
        review.account_row_shows_write_scope,
        review.account_row_shows_session_freshness,
        review.account_row_shows_access_capability,
        review.states_never_collapse_into_generic_connected,
        review.cached_inspect_never_reads_as_live,
        review.retry_remove_preserve_local_drafts,
        review.account_rows_stable_across_deployment_lines,
        review.account_rows_stable_across_consumer_surfaces,
        review.every_row_declares_accessibility_route,
        review.support_export_reconstructs_account_truth,
        review.later_rows_cannot_invent_parallel_account_vocabulary,
    ] {
        if !ok {
            violations.push(M5ProviderAccountRowViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ProviderAccountRowPacket,
    violations: &mut Vec<M5ProviderAccountRowViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.provider_surfaces_consume_account_vocabulary,
        projection.row_posture_reads_single_source,
        projection.access_capability_reads_single_source,
        projection.support_export_reads_single_source,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5ProviderAccountRowViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ProviderAccountRowPacket,
    violations: &mut Vec<M5ProviderAccountRowViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ProviderAccountRowViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ProviderAccountRowPacket,
    violations: &mut Vec<M5ProviderAccountRowViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.provider_account_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ProviderAccountRowViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
