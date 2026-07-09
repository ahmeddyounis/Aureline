//! Frozen M5 credential-state-row, secret-access-prompt-sheet, vault-or-keychain-picker,
//! credential-store-capability-row, browser-device-code-handoff-card,
//! delegated-credential-row, rotation-revoke-event-row, and export-safety-banner
//! component matrix.
//!
//! This module locks Aureline's reusable credential / auth-boundary components into one
//! export-safe packet. Every credential-bearing subcomponent M5 claims that still drifts
//! too easily by sign-in, provider, registry, request, remote, package, release, help, or
//! support surface — the credential-state row, the secret-access-prompt sheet, the
//! vault-or-keychain picker, the credential-store-capability row, the
//! browser/device-code handoff card, the delegated-credential row, the
//! rotation/revoke-event row, and the export-safety banner — is named once here and
//! constrained by the same storage mode, credential class, handle-only-versus-raw-reveal
//! posture, auth-handoff class, local-versus-forwarded/delegated identity, credential
//! lifecycle (expiry / refresh / rotation / revoke) state, store capability, degraded
//! state, and raw-secret-excluded export-safety boundary regardless of the surface family
//! that renders it.
//!
//! What this matrix freezes is the stable vocabulary for the *components* themselves: the
//! component families, the controlled storage modes (`os_keychain`, `encrypted_vault`,
//! `secret_broker_handle`, `session_memory_only`, `external_reference`,
//! `no_secret_stored`), credential classes, reveal postures, auth-handoff classes,
//! delegated-identity states, credential lifecycle states, store capabilities, degraded
//! states, export-safety classes, the deployment lines every component must survive, the
//! non-visual accessibility routes, and the mandatory labels every component must be able
//! to show. It does not re-architect the credential-state, secret-access-prompt,
//! secret-handle, credential-picker, system-browser return, or export-redaction contracts
//! that already own those records — it is the shared credential-component contract layered
//! on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 sign-in, provider,
//! registry, request, remote, package, release, help, or support surface may publish a
//! storage, reveal, delegation, lifecycle, or export-safety claim. Credential, secret,
//! vault, delegated-identity, lifecycle, and export consumers all read this packet so one
//! credential-state row names where a secret is stored and whether a handle-only path
//! exists, one secret-access-prompt sheet names its reveal posture and auth-handoff class,
//! one vault-or-keychain picker names the store it will write to, one
//! credential-store-capability row names what the store can and cannot do, one
//! browser/device-code handoff card names the auth handoff in flight, one
//! delegated-credential row names which identity is being forwarded or delegated, one
//! rotation/revoke-event row names what rotation or revoke will impact, and one
//! export-safety banner names what an export will and will not reveal. No M5 lane invents
//! a second credential grammar or an alternate label for a stored secret, a delegated
//! identity, a revoked credential, or a raw-secret-excluded export.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5CredentialComponentVocabularySet`] rather than minted per surface. Raw secret
//! values, pasted tokens, private endpoints, and passphrases stay outside the export
//! boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_credential_component_matrix,
    seeded_m5_credential_component_matrix_browser_device_code_handoff_card_beta_narrowed,
    seeded_m5_credential_component_matrix_export_safety_banner_preview_narrowed,
    M5_CREDENTIAL_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5CredentialComponentMatrixPacket`].
pub const M5_CREDENTIAL_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_credential_state_row_secret_access_prompt_sheet_vault_or_keychain_picker_credential_store_capability_row_browser_device_code_handoff_card_delegated_credential_row_rotation_revoke_event_row_and_export_safety_banner_component_matrix";

/// Schema version for M5 credential component-matrix records.
pub const M5_CREDENTIAL_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined credential component-matrix boundary schema.
pub const M5_CREDENTIAL_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-credential-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_CREDENTIAL_COMPONENT_DOC_REF: &str =
    "docs/security/m5_credential_component_matrix.md";

/// Repo-relative path of the credential-state-row canonical component schema.
pub const M5_CREDENTIAL_STATE_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-credential-state-row.schema.json";

/// Repo-relative path of the secret-access-prompt-sheet canonical component schema.
pub const M5_SECRET_ACCESS_PROMPT_SHEET_SCHEMA_REF: &str =
    "schemas/ui/m5-secret-access-prompt-sheet.schema.json";

/// Repo-relative path of the vault-or-keychain-picker canonical component schema.
pub const M5_VAULT_KEYCHAIN_PICKER_SCHEMA_REF: &str =
    "schemas/ui/m5-vault-keychain-picker.schema.json";

/// Repo-relative path of the credential-store-capability-row canonical component schema.
pub const M5_CREDENTIAL_STORE_CAPABILITY_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-credential-store-capability-row.schema.json";

/// Repo-relative path of the browser-device-code-handoff-card canonical component schema.
pub const M5_BROWSER_DEVICE_CODE_HANDOFF_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-browser-device-code-handoff-card.schema.json";

/// Repo-relative path of the delegated-credential-row canonical component schema.
pub const M5_DELEGATED_CREDENTIAL_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-delegated-credential-row.schema.json";

/// Repo-relative path of the rotation-revoke-event-row canonical component schema.
pub const M5_ROTATION_REVOKE_EVENT_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-rotation-revoke-event-row.schema.json";

/// Repo-relative path of the export-safety-banner canonical component schema.
pub const M5_EXPORT_SAFETY_BANNER_SCHEMA_REF: &str =
    "schemas/ui/m5-export-safety-banner.schema.json";

/// Repo-relative path of the credential-state foundation contract the credential-state row
/// binds against.
pub const M5_CREDENTIAL_COMPONENT_FOUNDATION_CREDENTIAL_STATE_REF: &str =
    "schemas/auth/credential_state.schema.json";

/// Repo-relative path of the secret-access-prompt foundation contract the prompt sheet
/// binds against.
pub const M5_CREDENTIAL_COMPONENT_FOUNDATION_SECRET_ACCESS_PROMPT_REF: &str =
    "schemas/auth/secret_access_prompt.schema.json";

/// Repo-relative path of the credential-picker foundation contract the vault/keychain
/// picker binds against.
pub const M5_CREDENTIAL_COMPONENT_FOUNDATION_CREDENTIAL_PICKER_REF: &str =
    "schemas/auth/credential_picker_state.schema.json";

/// Repo-relative path of the secret-handle foundation contract the store-capability and
/// rotation/revoke rows bind against.
pub const M5_CREDENTIAL_COMPONENT_FOUNDATION_SECRET_HANDLE_REF: &str =
    "schemas/security/secret_handle.schema.json";

/// Repo-relative path of the system-browser-return foundation contract the browser/device-
/// code handoff card binds against.
pub const M5_CREDENTIAL_COMPONENT_FOUNDATION_SYSTEM_BROWSER_REF: &str =
    "schemas/auth/system_browser_return_paths_beta.schema.json";

/// Repo-relative path of the credential-projection foundation contract the
/// delegated-credential row binds against.
pub const M5_CREDENTIAL_COMPONENT_FOUNDATION_CREDENTIAL_PROJECTION_REF: &str =
    "schemas/security/credential_projection.schema.json";

/// Repo-relative path of the export-redaction foundation contract the export-safety banner
/// binds against.
pub const M5_CREDENTIAL_COMPONENT_FOUNDATION_EXPORT_REDACTION_REF: &str =
    "schemas/support/export_redaction_profile.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_CREDENTIAL_COMPONENT_FIXTURE_DIR: &str = "fixtures/ui/m5-credential-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CREDENTIAL_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-credential-component-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_CREDENTIAL_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-credential-component-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_CREDENTIAL_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-credential-component-matrix.md";

/// One of the eight governed credential / auth-boundary component families this matrix
/// freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialComponentFamily {
    /// A credential-state row carrying its storage mode, credential class, reveal posture,
    /// and lifecycle state.
    CredentialStateRow,
    /// A secret-access-prompt sheet carrying its credential class, reveal posture, and
    /// auth-handoff class.
    SecretAccessPromptSheet,
    /// A vault-or-keychain picker carrying the storage mode and store capability it will
    /// write to.
    VaultOrKeychainPicker,
    /// A credential-store-capability row carrying the store's capabilities and degraded
    /// state.
    CredentialStoreCapabilityRow,
    /// A browser/device-code handoff card carrying the auth-handoff class in flight.
    BrowserDeviceCodeHandoffCard,
    /// A delegated-credential row carrying the forwarded / delegated identity state.
    DelegatedCredentialRow,
    /// A rotation/revoke-event row carrying the credential lifecycle state.
    RotationRevokeEventRow,
    /// An export-safety banner carrying the reveal posture and export-safety boundary.
    ExportSafetyBanner,
}

impl M5CredentialComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::CredentialStateRow,
        Self::SecretAccessPromptSheet,
        Self::VaultOrKeychainPicker,
        Self::CredentialStoreCapabilityRow,
        Self::BrowserDeviceCodeHandoffCard,
        Self::DelegatedCredentialRow,
        Self::RotationRevokeEventRow,
        Self::ExportSafetyBanner,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CredentialStateRow => "credential_state_row",
            Self::SecretAccessPromptSheet => "secret_access_prompt_sheet",
            Self::VaultOrKeychainPicker => "vault_or_keychain_picker",
            Self::CredentialStoreCapabilityRow => "credential_store_capability_row",
            Self::BrowserDeviceCodeHandoffCard => "browser_device_code_handoff_card",
            Self::DelegatedCredentialRow => "delegated_credential_row",
            Self::RotationRevokeEventRow => "rotation_revoke_event_row",
            Self::ExportSafetyBanner => "export_safety_banner",
        }
    }

    /// The canonical per-component schema ref a downstream row points at instead of
    /// restating this component's credential truth by hand.
    pub const fn canonical_component_schema_ref(self) -> &'static str {
        match self {
            Self::CredentialStateRow => M5_CREDENTIAL_STATE_ROW_SCHEMA_REF,
            Self::SecretAccessPromptSheet => M5_SECRET_ACCESS_PROMPT_SHEET_SCHEMA_REF,
            Self::VaultOrKeychainPicker => M5_VAULT_KEYCHAIN_PICKER_SCHEMA_REF,
            Self::CredentialStoreCapabilityRow => M5_CREDENTIAL_STORE_CAPABILITY_ROW_SCHEMA_REF,
            Self::BrowserDeviceCodeHandoffCard => M5_BROWSER_DEVICE_CODE_HANDOFF_CARD_SCHEMA_REF,
            Self::DelegatedCredentialRow => M5_DELEGATED_CREDENTIAL_ROW_SCHEMA_REF,
            Self::RotationRevokeEventRow => M5_ROTATION_REVOKE_EVENT_ROW_SCHEMA_REF,
            Self::ExportSafetyBanner => M5_EXPORT_SAFETY_BANNER_SCHEMA_REF,
        }
    }

    /// `true` when this family must declare a controlled storage mode.
    pub const fn declares_storage_mode(self) -> bool {
        matches!(
            self,
            Self::CredentialStateRow
                | Self::VaultOrKeychainPicker
                | Self::CredentialStoreCapabilityRow
        )
    }

    /// `true` when this family must declare a controlled credential class.
    pub const fn declares_credential_class(self) -> bool {
        matches!(
            self,
            Self::CredentialStateRow | Self::SecretAccessPromptSheet | Self::DelegatedCredentialRow
        )
    }

    /// `true` when this family must declare a controlled reveal posture.
    pub const fn declares_reveal_posture(self) -> bool {
        matches!(
            self,
            Self::CredentialStateRow | Self::SecretAccessPromptSheet | Self::ExportSafetyBanner
        )
    }

    /// `true` when this family must declare a controlled auth-handoff class.
    pub const fn declares_auth_handoff_class(self) -> bool {
        matches!(
            self,
            Self::SecretAccessPromptSheet | Self::BrowserDeviceCodeHandoffCard
        )
    }

    /// `true` when this family must declare a controlled delegated-identity state.
    pub const fn declares_delegated_identity_state(self) -> bool {
        matches!(self, Self::DelegatedCredentialRow)
    }

    /// `true` when this family must declare a controlled credential lifecycle state.
    pub const fn declares_lifecycle_state(self) -> bool {
        matches!(
            self,
            Self::CredentialStateRow | Self::RotationRevokeEventRow
        )
    }

    /// `true` when this family must declare a controlled store capability.
    pub const fn declares_store_capability(self) -> bool {
        matches!(
            self,
            Self::VaultOrKeychainPicker | Self::CredentialStoreCapabilityRow
        )
    }

    /// `true` when this family must declare a controlled export-safety class.
    pub const fn declares_export_safety_class(self) -> bool {
        matches!(self, Self::ExportSafetyBanner)
    }
}

/// Controlled storage mode — where a secret actually lives, so a credential component never
/// leaves storage implicit or invents a parallel taxonomy. These are the exact
/// acceptance-criteria storage-mode terms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialStorageMode {
    /// Stored in the OS keychain / keystore.
    OsKeychain,
    /// Stored in an encrypted vault.
    EncryptedVault,
    /// Held only as a broker handle, never the raw secret.
    SecretBrokerHandle,
    /// Held in session memory only, gone at exit.
    SessionMemoryOnly,
    /// An external reference to a secret held elsewhere.
    ExternalReference,
    /// No secret stored at all.
    NoSecretStored,
}

impl M5CredentialStorageMode {
    /// Every storage mode, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OsKeychain,
        Self::EncryptedVault,
        Self::SecretBrokerHandle,
        Self::SessionMemoryOnly,
        Self::ExternalReference,
        Self::NoSecretStored,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsKeychain => "os_keychain",
            Self::EncryptedVault => "encrypted_vault",
            Self::SecretBrokerHandle => "secret_broker_handle",
            Self::SessionMemoryOnly => "session_memory_only",
            Self::ExternalReference => "external_reference",
            Self::NoSecretStored => "no_secret_stored",
        }
    }
}

/// Controlled credential class — the kind of credential a component represents, so a
/// component never leaves the credential kind implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialClass {
    /// An OAuth / OIDC access token.
    OauthToken,
    /// An API key.
    ApiKey,
    /// A personal access token.
    PersonalAccessToken,
    /// An SSH or signing key.
    SshOrSigningKey,
    /// A client certificate.
    ClientCertificate,
    /// A device-code grant.
    DeviceCodeGrant,
}

impl M5CredentialClass {
    /// Every credential class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OauthToken,
        Self::ApiKey,
        Self::PersonalAccessToken,
        Self::SshOrSigningKey,
        Self::ClientCertificate,
        Self::DeviceCodeGrant,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OauthToken => "oauth_token",
            Self::ApiKey => "api_key",
            Self::PersonalAccessToken => "personal_access_token",
            Self::SshOrSigningKey => "ssh_or_signing_key",
            Self::ClientCertificate => "client_certificate",
            Self::DeviceCodeGrant => "device_code_grant",
        }
    }
}

/// Controlled reveal posture — whether a component ever exposes the raw secret or keeps a
/// handle-only path, so a user never has to infer whether a raw reveal is possible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialRevealPosture {
    /// Handle only — the raw secret is never shown.
    HandleOnly,
    /// Only the last few characters are shown.
    MaskedLastFour,
    /// The raw secret can be revealed on explicit demand.
    RevealOnDemand,
    /// A scoped copy-to-clipboard path exists but no on-screen reveal.
    ClipboardScoped,
    /// The secret is never revealed by any path.
    NeverRevealed,
    /// A raw reveal is blocked by policy.
    PolicyBlockedReveal,
}

impl M5CredentialRevealPosture {
    /// Every reveal posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HandleOnly,
        Self::MaskedLastFour,
        Self::RevealOnDemand,
        Self::ClipboardScoped,
        Self::NeverRevealed,
        Self::PolicyBlockedReveal,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HandleOnly => "handle_only",
            Self::MaskedLastFour => "masked_last_four",
            Self::RevealOnDemand => "reveal_on_demand",
            Self::ClipboardScoped => "clipboard_scoped",
            Self::NeverRevealed => "never_revealed",
            Self::PolicyBlockedReveal => "policy_blocked_reveal",
        }
    }
}

/// Controlled auth-handoff class — how a secret-access-prompt or handoff card completes an
/// authentication, so no surface invents an alternate label for a browser or device-code
/// handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthHandoffClass {
    /// A system-browser redirect handoff.
    SystemBrowserRedirect,
    /// A device-code poll handoff.
    DeviceCodePoll,
    /// An embedded in-app prompt.
    EmbeddedPrompt,
    /// A passkey step-up.
    PasskeyStepUp,
    /// A delegated forward to another principal.
    DelegatedForward,
    /// Handoff deferred until back online.
    OfflineDeferred,
}

impl M5AuthHandoffClass {
    /// Every auth-handoff class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SystemBrowserRedirect,
        Self::DeviceCodePoll,
        Self::EmbeddedPrompt,
        Self::PasskeyStepUp,
        Self::DelegatedForward,
        Self::OfflineDeferred,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemBrowserRedirect => "system_browser_redirect",
            Self::DeviceCodePoll => "device_code_poll",
            Self::EmbeddedPrompt => "embedded_prompt",
            Self::PasskeyStepUp => "passkey_step_up",
            Self::DelegatedForward => "delegated_forward",
            Self::OfflineDeferred => "offline_deferred",
        }
    }
}

/// Controlled delegated-identity state — which identity a delegated-credential row is
/// acting as, so a user never has to infer whether an identity is local, forwarded, or
/// delegated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DelegatedIdentityState {
    /// The local identity is acting directly.
    LocalIdentity,
    /// The identity is forwarded from another principal.
    ForwardedIdentity,
    /// Acting on behalf of another principal by delegation.
    DelegatedOnBehalf,
    /// Impersonation scoped to selected resources.
    ImpersonationScoped,
    /// A service account is acting.
    ServiceAccountActing,
    /// The delegation has been revoked.
    DelegationRevoked,
}

impl M5DelegatedIdentityState {
    /// Every delegated-identity state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalIdentity,
        Self::ForwardedIdentity,
        Self::DelegatedOnBehalf,
        Self::ImpersonationScoped,
        Self::ServiceAccountActing,
        Self::DelegationRevoked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalIdentity => "local_identity",
            Self::ForwardedIdentity => "forwarded_identity",
            Self::DelegatedOnBehalf => "delegated_on_behalf",
            Self::ImpersonationScoped => "impersonation_scoped",
            Self::ServiceAccountActing => "service_account_acting",
            Self::DelegationRevoked => "delegation_revoked",
        }
    }
}

/// Controlled credential lifecycle state — the expiry / refresh / rotation / revoke state a
/// credential-state or rotation/revoke-event row names, so a user always sees what rotation
/// or revoke will impact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialLifecycleState {
    /// Active and current.
    ActiveCurrent,
    /// A refresh is needed.
    RefreshNeeded,
    /// A rotation is due.
    RotationDue,
    /// The credential has been revoked.
    Revoked,
    /// The credential has expired.
    Expired,
    /// Superseded by a newer credential.
    Superseded,
}

impl M5CredentialLifecycleState {
    /// Every lifecycle state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ActiveCurrent,
        Self::RefreshNeeded,
        Self::RotationDue,
        Self::Revoked,
        Self::Expired,
        Self::Superseded,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveCurrent => "active_current",
            Self::RefreshNeeded => "refresh_needed",
            Self::RotationDue => "rotation_due",
            Self::Revoked => "revoked",
            Self::Expired => "expired",
            Self::Superseded => "superseded",
        }
    }
}

/// Controlled store capability — what a credential store can and cannot do, so a
/// vault/keychain picker or store-capability row never leaves a store's guarantees
/// implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialStoreCapability {
    /// Persists across restart.
    PersistAcrossRestart,
    /// Locked at rest by the OS.
    OsLockedAtRest,
    /// Syncs across devices.
    SyncAcrossDevices,
    /// Hardware backed.
    HardwareBacked,
    /// Export is blocked by the store itself.
    StoreExportBlocked,
    /// Session only, never persisted.
    SessionOnly,
}

impl M5CredentialStoreCapability {
    /// Every store capability, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PersistAcrossRestart,
        Self::OsLockedAtRest,
        Self::SyncAcrossDevices,
        Self::HardwareBacked,
        Self::StoreExportBlocked,
        Self::SessionOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PersistAcrossRestart => "persist_across_restart",
            Self::OsLockedAtRest => "os_locked_at_rest",
            Self::SyncAcrossDevices => "sync_across_devices",
            Self::HardwareBacked => "hardware_backed",
            Self::StoreExportBlocked => "store_export_blocked",
            Self::SessionOnly => "session_only",
        }
    }
}

/// Controlled export-safety class — what an export-safety banner discloses an export will
/// and will not reveal, so no surface implies a raw secret is export-safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialExportSafetyClass {
    /// Raw secret values are excluded from export.
    RawSecretExcluded,
    /// Metadata only is exported.
    MetadataOnly,
    /// Only a handle reference is exported.
    HandleReferenceOnly,
    /// A redacted share is exported.
    RedactedShare,
    /// Endpoints are masked in export.
    EndpointsMasked,
    /// Export is blocked entirely.
    ExportBlocked,
}

impl M5CredentialExportSafetyClass {
    /// Every export-safety class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RawSecretExcluded,
        Self::MetadataOnly,
        Self::HandleReferenceOnly,
        Self::RedactedShare,
        Self::EndpointsMasked,
        Self::ExportBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawSecretExcluded => "raw_secret_excluded",
            Self::MetadataOnly => "metadata_only",
            Self::HandleReferenceOnly => "handle_reference_only",
            Self::RedactedShare => "redacted_share",
            Self::EndpointsMasked => "endpoints_masked",
            Self::ExportBlocked => "export_blocked",
        }
    }
}

/// Controlled degraded state — the fallback state every credential component must be able
/// to name, so a session-only fallback or policy-blocked state is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialDegradedState {
    /// Fully available.
    FullyAvailable,
    /// Limited capability.
    LimitedCapability,
    /// Stale, needs reauthentication.
    StaleNeedsReauth,
    /// Offline cached.
    OfflineCached,
    /// Blocked by policy.
    PolicyBlocked,
    /// Unavailable on this build.
    Unavailable,
}

impl M5CredentialDegradedState {
    /// Every degraded state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullyAvailable,
        Self::LimitedCapability,
        Self::StaleNeedsReauth,
        Self::OfflineCached,
        Self::PolicyBlocked,
        Self::Unavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyAvailable => "fully_available",
            Self::LimitedCapability => "limited_capability",
            Self::StaleNeedsReauth => "stale_needs_reauth",
            Self::OfflineCached => "offline_cached",
            Self::PolicyBlocked => "policy_blocked",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Claimed M5 surface family that renders / consumes a credential component. No component
/// may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialSurfaceFamily {
    /// The sign-in / authentication surface.
    SignInAndAuth,
    /// The provider / registry authorization surface.
    ProviderAndRegistry,
    /// The remote-target / database attach surface.
    RemoteAndDatabase,
    /// The package / release publish surface.
    PackageAndRelease,
    /// The recovery / audit surface.
    RecoveryAndAudit,
    /// The CLI / headless surface.
    CliAndHeadless,
}

impl M5CredentialSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SignInAndAuth,
        Self::ProviderAndRegistry,
        Self::RemoteAndDatabase,
        Self::PackageAndRelease,
        Self::RecoveryAndAudit,
        Self::CliAndHeadless,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignInAndAuth => "sign_in_and_auth",
            Self::ProviderAndRegistry => "provider_and_registry",
            Self::RemoteAndDatabase => "remote_and_database",
            Self::PackageAndRelease => "package_and_release",
            Self::RecoveryAndAudit => "recovery_and_audit",
            Self::CliAndHeadless => "cli_and_headless",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's storage,
/// reveal, delegation, lifecycle, or export truth never silently narrows or widens between
/// deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5CredentialDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialConsumerSurface {
    /// The credential-settings UI.
    CredentialSettingsUi,
    /// The secret-prompt UI.
    SecretPromptUi,
    /// The vault-picker UI.
    VaultPickerUi,
    /// The device-code / browser handoff UI.
    DeviceCodeHandoffUi,
    /// The delegated-identity UI.
    DelegatedIdentityUi,
    /// The support export.
    SupportExport,
    /// The CLI inspect / headless surface.
    CliInspect,
    /// The status-bar UI.
    StatusBarUi,
    /// The general product UI.
    ProductUi,
}

impl M5CredentialConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::CredentialSettingsUi,
        Self::SecretPromptUi,
        Self::VaultPickerUi,
        Self::DeviceCodeHandoffUi,
        Self::DelegatedIdentityUi,
        Self::SupportExport,
        Self::CliInspect,
        Self::StatusBarUi,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CredentialSettingsUi => "credential_settings_ui",
            Self::SecretPromptUi => "secret_prompt_ui",
            Self::VaultPickerUi => "vault_picker_ui",
            Self::DeviceCodeHandoffUi => "device_code_handoff_ui",
            Self::DelegatedIdentityUi => "delegated_identity_ui",
            Self::SupportExport => "support_export",
            Self::CliInspect => "cli_inspect",
            Self::StatusBarUi => "status_bar_ui",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no credential truth is
/// hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5CredentialAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed credential component must be able to show. The first three are
/// hard requirements on every component; the remaining three close the acceptance-criteria
/// ambiguity about storage/reveal posture, identity/delegation, and expiry / export
/// boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialRequiredLabel {
    /// The component's stable identity / what credential object it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The storage mode and handle-only-versus-raw-reveal posture behind the component.
    StorageAndRevealPosture,
    /// The local-versus-forwarded/delegated identity behind the component.
    IdentityAndDelegation,
    /// The expiry / lifecycle state and raw-secret-excluded export boundary behind the
    /// component.
    ExpiryAndExportBoundary,
}

impl M5CredentialRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::StorageAndRevealPosture,
        Self::IdentityAndDelegation,
        Self::ExpiryAndExportBoundary,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::StorageAndRevealPosture => "storage_and_reveal_posture",
            Self::IdentityAndDelegation => "identity_and_delegation",
            Self::ExpiryAndExportBoundary => "expiry_and_export_boundary",
        }
    }
}

/// Qualification class for an M5 credential component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5CredentialQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a credential component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialDowngradeTrigger {
    /// A component left its storage mode unstated.
    StorageModeUnstated,
    /// A component left its reveal posture unstated.
    RevealPostureUnstated,
    /// A component left its credential class unstated.
    CredentialClassUnstated,
    /// A component left its auth-handoff class unstated.
    AuthHandoffClassUnstated,
    /// A component left its delegated-identity state unstated.
    DelegatedIdentityUnstated,
    /// A component hid its lifecycle (expiry / rotation / revoke) state.
    LifecycleStateHidden,
    /// A component hid its store capability.
    StoreCapabilityUnstated,
    /// A component hid its export-safety boundary.
    ExportSafetyBoundaryHidden,
    /// A surface invented an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// Friendly "connected" / "signed in" wording concealed storage, delegation, or reveal
    /// truth.
    FriendlyConnectedWordingUsed,
    /// A session-only fallback state was hidden before send / run / publish.
    SessionOnlyFallbackHidden,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5CredentialDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::StorageModeUnstated,
        Self::RevealPostureUnstated,
        Self::CredentialClassUnstated,
        Self::AuthHandoffClassUnstated,
        Self::DelegatedIdentityUnstated,
        Self::LifecycleStateHidden,
        Self::StoreCapabilityUnstated,
        Self::ExportSafetyBoundaryHidden,
        Self::AlternateStateLabelInvented,
        Self::FriendlyConnectedWordingUsed,
        Self::SessionOnlyFallbackHidden,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StorageModeUnstated => "storage_mode_unstated",
            Self::RevealPostureUnstated => "reveal_posture_unstated",
            Self::CredentialClassUnstated => "credential_class_unstated",
            Self::AuthHandoffClassUnstated => "auth_handoff_class_unstated",
            Self::DelegatedIdentityUnstated => "delegated_identity_unstated",
            Self::LifecycleStateHidden => "lifecycle_state_hidden",
            Self::StoreCapabilityUnstated => "store_capability_unstated",
            Self::ExportSafetyBoundaryHidden => "export_safety_boundary_hidden",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::FriendlyConnectedWordingUsed => "friendly_connected_wording_used",
            Self::SessionOnlyFallbackHidden => "session_only_fallback_hidden",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed credential component family bound to the
/// surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialComponentRow {
    /// Governed component family.
    pub component_family: M5CredentialComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5CredentialQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this component.
    pub surface_families: Vec<M5CredentialSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5CredentialDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5CredentialRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5CredentialRequiredLabel>,
    /// Storage modes this component names (storage-declaring families only).
    pub storage_modes: Vec<M5CredentialStorageMode>,
    /// Credential classes this component names (class-declaring families only).
    pub credential_classes: Vec<M5CredentialClass>,
    /// Reveal postures this component names (reveal-declaring families only).
    pub reveal_postures: Vec<M5CredentialRevealPosture>,
    /// Auth-handoff classes this component names (handoff-declaring families only).
    pub auth_handoff_classes: Vec<M5AuthHandoffClass>,
    /// Delegated-identity states this component names (delegated-credential-row only).
    pub delegated_identity_states: Vec<M5DelegatedIdentityState>,
    /// Credential lifecycle states this component names (lifecycle-declaring families
    /// only).
    pub lifecycle_states: Vec<M5CredentialLifecycleState>,
    /// Store capabilities this component names (store-declaring families only).
    pub store_capabilities: Vec<M5CredentialStoreCapability>,
    /// Export-safety classes this component names (export-safety-banner only).
    pub export_safety_classes: Vec<M5CredentialExportSafetyClass>,
    /// Degraded states this component can name (required on every component).
    pub degraded_states: Vec<M5CredentialDegradedState>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5CredentialAccessibilityRoute>,
    /// Subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5CredentialConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5CredentialDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component (must include its own canonical
    /// component schema so downstream rows have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never masks its storage mode or reveal posture. MUST
    /// be `false`.
    pub masks_storage_or_reveal_posture: bool,
    /// Hard invariant: this component never hides a forwarded / delegated identity. MUST be
    /// `false`.
    pub hides_identity_delegation: bool,
    /// Hard invariant: this component never invents an alternate label for a governed
    /// state. MUST be `false`.
    pub invents_alternate_state_label: bool,
    /// Hard invariant: this component never implies a raw secret is export-safe. MUST be
    /// `false`.
    pub implies_raw_secret_exportable: bool,
}

impl M5CredentialComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5CredentialRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5CredentialRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_storage_or_reveal_posture
            && !self.hides_identity_delegation
            && !self.invents_alternate_state_label
            && !self.implies_raw_secret_exportable
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialComponentVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Storage-mode tokens.
    pub storage_modes: Vec<String>,
    /// Credential-class tokens.
    pub credential_classes: Vec<String>,
    /// Reveal-posture tokens.
    pub reveal_postures: Vec<String>,
    /// Auth-handoff-class tokens.
    pub auth_handoff_classes: Vec<String>,
    /// Delegated-identity-state tokens.
    pub delegated_identity_states: Vec<String>,
    /// Credential-lifecycle-state tokens.
    pub lifecycle_states: Vec<String>,
    /// Store-capability tokens.
    pub store_capabilities: Vec<String>,
    /// Export-safety-class tokens.
    pub export_safety_classes: Vec<String>,
    /// Degraded-state tokens.
    pub degraded_states: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5CredentialComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5CredentialComponentFamily::ALL, |v| v.as_str()),
            storage_modes: tokens(&M5CredentialStorageMode::ALL, |v| v.as_str()),
            credential_classes: tokens(&M5CredentialClass::ALL, |v| v.as_str()),
            reveal_postures: tokens(&M5CredentialRevealPosture::ALL, |v| v.as_str()),
            auth_handoff_classes: tokens(&M5AuthHandoffClass::ALL, |v| v.as_str()),
            delegated_identity_states: tokens(&M5DelegatedIdentityState::ALL, |v| v.as_str()),
            lifecycle_states: tokens(&M5CredentialLifecycleState::ALL, |v| v.as_str()),
            store_capabilities: tokens(&M5CredentialStoreCapability::ALL, |v| v.as_str()),
            export_safety_classes: tokens(&M5CredentialExportSafetyClass::ALL, |v| v.as_str()),
            degraded_states: tokens(&M5CredentialDegradedState::ALL, |v| v.as_str()),
            surface_families: tokens(&M5CredentialSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5CredentialDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5CredentialConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5CredentialAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5CredentialRequiredLabel::ALL, |v| v.as_str()),
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
pub struct M5CredentialComponentGovernanceReview {
    /// The credential-state row shows its storage mode and reveal posture.
    pub credential_state_row_shows_storage_and_reveal: bool,
    /// The secret-access-prompt sheet shows its credential class and auth-handoff class.
    pub secret_access_prompt_shows_class_and_handoff: bool,
    /// The vault/keychain picker shows the storage mode and store capability it writes to.
    pub vault_picker_shows_storage_and_capability: bool,
    /// The credential-store-capability row shows its capability and degraded state.
    pub store_capability_row_shows_capability_and_degraded: bool,
    /// The browser/device-code handoff card shows its auth-handoff class.
    pub handoff_card_shows_auth_handoff_class: bool,
    /// The delegated-credential row shows its delegated-identity state.
    pub delegated_row_shows_identity_state: bool,
    /// The rotation/revoke-event row shows its lifecycle state.
    pub rotation_revoke_row_shows_lifecycle_state: bool,
    /// The export-safety banner shows its export-safety boundary.
    pub export_safety_banner_shows_export_boundary: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The storage-mode vocabulary is named once.
    pub storage_mode_vocabulary_named_once: bool,
    /// The reveal posture and raw-secret-excluded export safety are named once.
    pub reveal_and_export_safety_named_once: bool,
    /// The forwarded / delegated identity is always explicit.
    pub delegated_identity_always_explicit: bool,
    /// The session-only fallback state is always explicit before send / run / publish.
    pub session_only_fallback_always_explicit: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel credential vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialComponentConsumerProjection {
    /// Credential surfaces consume the shared storage-mode vocabulary.
    pub credential_surfaces_consume_storage_vocabulary: bool,
    /// Prompt / handoff surfaces consume the auth-handoff vocabulary.
    pub prompt_surfaces_consume_handoff_vocabulary: bool,
    /// Delegated surfaces consume the delegated-identity vocabulary.
    pub delegated_surfaces_consume_identity_vocabulary: bool,
    /// Lifecycle surfaces consume the rotation / revoke vocabulary.
    pub lifecycle_surfaces_consume_rotation_vocabulary: bool,
    /// Export surfaces consume the export-safety vocabulary.
    pub export_surfaces_consume_safety_vocabulary: bool,
    /// Support / export reads a single canonical credential source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the credential component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialComponentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting credential audit for the lane.
    pub credential_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5CredentialComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CredentialComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5CredentialComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CredentialComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CredentialComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CredentialComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CredentialComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CredentialComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 credential component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialComponentMatrixPacket {
    /// Record kind; must equal [`M5_CREDENTIAL_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CREDENTIAL_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5CredentialComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CredentialComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CredentialComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CredentialComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CredentialComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CredentialComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CredentialComponentMatrixPacket {
    /// Builds an M5 credential component matrix packet from stable-lane input.
    pub fn new(input: M5CredentialComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_CREDENTIAL_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_CREDENTIAL_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
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

    /// Validates the M5 credential component matrix invariants.
    pub fn validate(&self) -> Vec<M5CredentialComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_CREDENTIAL_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5CredentialComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CREDENTIAL_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5CredentialComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CredentialComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 credential component matrix packet serializes"),
        ) {
            violations.push(M5CredentialComponentMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 credential component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,canonical_schema,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.component_family.canonical_component_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Credential-State-Row, Secret-Access-Prompt-Sheet, Vault-or-Keychain-Picker, Credential-Store-Capability-Row, Browser-Device-Code-Handoff-Card, Delegated-Credential-Row, Rotation-Revoke-Event-Row, and Export-Safety-Banner Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Storage modes: {}\n",
            self.vocabulary_set.storage_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Export-safety classes: {}\n",
            self.vocabulary_set.export_safety_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.component_family.canonical_component_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 credential matrix export.
#[derive(Debug)]
pub enum M5CredentialComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CredentialComponentMatrixViolation>),
}

impl fmt::Display for M5CredentialComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 credential component matrix export parse failed: {error}"
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
                    "m5 credential component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CredentialComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5CredentialComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CredentialComponentMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A component row does not point at its own canonical component schema.
    ComponentSchemaRefMissing,
    /// A storage-declaring component declares no storage modes.
    StorageModeMissing,
    /// A class-declaring component declares no credential classes.
    CredentialClassMissing,
    /// A reveal-declaring component declares no reveal postures.
    RevealPostureMissing,
    /// A handoff-declaring component declares no auth-handoff classes.
    AuthHandoffClassMissing,
    /// A delegated-credential-row component declares no delegated-identity states.
    DelegatedIdentityStateMissing,
    /// A lifecycle-declaring component declares no lifecycle states.
    LifecycleStateMissing,
    /// A store-declaring component declares no store capabilities.
    StoreCapabilityMissing,
    /// An export-safety-banner component declares no export-safety classes.
    ExportSafetyClassMissing,
    /// A component declares no degraded states.
    DegradedStateMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no deployment lines.
    DeploymentLineMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (masked storage/reveal, hidden delegation,
    /// invented alternate state label, or implied raw-secret export).
    ComponentInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5CredentialComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::StorageModeMissing => "storage_mode_missing",
            Self::CredentialClassMissing => "credential_class_missing",
            Self::RevealPostureMissing => "reveal_posture_missing",
            Self::AuthHandoffClassMissing => "auth_handoff_class_missing",
            Self::DelegatedIdentityStateMissing => "delegated_identity_state_missing",
            Self::LifecycleStateMissing => "lifecycle_state_missing",
            Self::StoreCapabilityMissing => "store_capability_missing",
            Self::ExportSafetyClassMissing => "export_safety_class_missing",
            Self::DegradedStateMissing => "degraded_state_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 credential matrix export.
pub fn current_stable_m5_credential_component_matrix_export(
) -> Result<M5CredentialComponentMatrixPacket, M5CredentialComponentMatrixArtifactError> {
    let packet: M5CredentialComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-credential-component-proof/support_export.json"
    )))
    .map_err(M5CredentialComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CredentialComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5CredentialComponentMatrixPacket,
    violations: &mut Vec<M5CredentialComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_DOC_REF,
        M5_CREDENTIAL_STATE_ROW_SCHEMA_REF,
        M5_SECRET_ACCESS_PROMPT_SHEET_SCHEMA_REF,
        M5_VAULT_KEYCHAIN_PICKER_SCHEMA_REF,
        M5_CREDENTIAL_STORE_CAPABILITY_ROW_SCHEMA_REF,
        M5_BROWSER_DEVICE_CODE_HANDOFF_CARD_SCHEMA_REF,
        M5_DELEGATED_CREDENTIAL_ROW_SCHEMA_REF,
        M5_ROTATION_REVOKE_EVENT_ROW_SCHEMA_REF,
        M5_EXPORT_SAFETY_BANNER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5CredentialComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5CredentialComponentMatrixPacket,
    violations: &mut Vec<M5CredentialComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5CredentialComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5CredentialComponentMatrixPacket,
    violations: &mut Vec<M5CredentialComponentMatrixViolation>,
) {
    let present: BTreeSet<M5CredentialComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5CredentialComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5CredentialComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5CredentialComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5CredentialComponentMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == family.canonical_component_schema_ref())
        {
            violations.push(M5CredentialComponentMatrixViolation::ComponentSchemaRefMissing);
        }
        if family.declares_storage_mode() && row.storage_modes.is_empty() {
            violations.push(M5CredentialComponentMatrixViolation::StorageModeMissing);
        }
        if family.declares_credential_class() && row.credential_classes.is_empty() {
            violations.push(M5CredentialComponentMatrixViolation::CredentialClassMissing);
        }
        if family.declares_reveal_posture() && row.reveal_postures.is_empty() {
            violations.push(M5CredentialComponentMatrixViolation::RevealPostureMissing);
        }
        if family.declares_auth_handoff_class() && row.auth_handoff_classes.is_empty() {
            violations.push(M5CredentialComponentMatrixViolation::AuthHandoffClassMissing);
        }
        if family.declares_delegated_identity_state() && row.delegated_identity_states.is_empty() {
            violations.push(M5CredentialComponentMatrixViolation::DelegatedIdentityStateMissing);
        }
        if family.declares_lifecycle_state() && row.lifecycle_states.is_empty() {
            violations.push(M5CredentialComponentMatrixViolation::LifecycleStateMissing);
        }
        if family.declares_store_capability() && row.store_capabilities.is_empty() {
            violations.push(M5CredentialComponentMatrixViolation::StoreCapabilityMissing);
        }
        if family.declares_export_safety_class() && row.export_safety_classes.is_empty() {
            violations.push(M5CredentialComponentMatrixViolation::ExportSafetyClassMissing);
        }
        if row.degraded_states.is_empty() {
            violations.push(M5CredentialComponentMatrixViolation::DegradedStateMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5CredentialComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5CredentialComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5CredentialComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5CredentialComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5CredentialComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5CredentialComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5CredentialComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5CredentialComponentMatrixPacket,
    violations: &mut Vec<M5CredentialComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.credential_state_row_shows_storage_and_reveal,
        review.secret_access_prompt_shows_class_and_handoff,
        review.vault_picker_shows_storage_and_capability,
        review.store_capability_row_shows_capability_and_degraded,
        review.handoff_card_shows_auth_handoff_class,
        review.delegated_row_shows_identity_state,
        review.rotation_revoke_row_shows_lifecycle_state,
        review.export_safety_banner_shows_export_boundary,
        review.no_surface_invents_alternate_state_label,
        review.storage_mode_vocabulary_named_once,
        review.reveal_and_export_safety_named_once,
        review.delegated_identity_always_explicit,
        review.session_only_fallback_always_explicit,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5CredentialComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5CredentialComponentMatrixPacket,
    violations: &mut Vec<M5CredentialComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.credential_surfaces_consume_storage_vocabulary,
        projection.prompt_surfaces_consume_handoff_vocabulary,
        projection.delegated_surfaces_consume_identity_vocabulary,
        projection.lifecycle_surfaces_consume_rotation_vocabulary,
        projection.export_surfaces_consume_safety_vocabulary,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5CredentialComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5CredentialComponentMatrixPacket,
    violations: &mut Vec<M5CredentialComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5CredentialComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5CredentialComponentMatrixPacket,
    violations: &mut Vec<M5CredentialComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.credential_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5CredentialComponentMatrixViolation::ReleasePostureIncomplete);
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

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The
/// controlled vocabulary deliberately uses the words `secret` and `credential`, so those
/// bare identifier tokens are allowed; what is rejected is a raw secret *value* shape — a
/// pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
