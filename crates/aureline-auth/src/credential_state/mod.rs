//! Credential-state row, locked / unavailable store handling, and provider /
//! account registry seed.
//!
//! This module is the M1 seed for the credential-state lane. It owns:
//!
//! - one inspectable [`CredentialStateRow`] record that names the storage mode,
//!   scope, expiry, revoke action, source label, and locked / unavailable
//!   states for credentials or delegated handles used by the initial managed /
//!   provider lanes;
//! - one [`ProviderAccountRecord`] seed for connected accounts and providers;
//! - one [`ProviderAccountRegistry`] join object so later browser handoff and
//!   publish-later work can reuse one registry shape; and
//! - one [`CredentialStateChip`] projection a shell consumer renders next to
//!   the existing [`crate::ShellAuthChip`] without forking authority truth.
//!
//! The seed deliberately covers a subset of the full credential-state contract
//! frozen in
//! [`/docs/auth/credential_state_and_secret_prompt_contract.md`](../../../../docs/auth/credential_state_and_secret_prompt_contract.md)
//! and
//! [`/schemas/auth/credential_state.schema.json`](../../../../schemas/auth/credential_state.schema.json).
//! It freezes the user / admin-facing vocabulary the M1 protected row needs —
//! storage mode, scope, expiry, revoke action, locked / unavailable state — and
//! grows additively without forking truth.
//!
//! ## Why one inspectable seed
//!
//! Without a single seed object every later credential surface (terminal pane,
//! activity center, status bar, support / export flows) is free to invent a
//! local "Connected" / "Signed in" badge, hide the locked or unavailable store
//! posture behind a generic warning, or silently fall back to a plaintext file
//! credential. The seed closes those gaps before the first real OAuth / OIDC /
//! agent integration lands.
//!
//! ## Failure-drill posture
//!
//! [`ProviderAccountRegistry::lock_store`] and
//! [`ProviderAccountRegistry::mark_store_unavailable`] flip every row backed by
//! the named store source onto the typed locked / unavailable state class,
//! preserve the alias / handle reference for later recovery, surface the
//! revoke action verbatim, and never silently downgrade to a plaintext-file
//! fallback. The seed surface MUST stay readable so the user can tell that
//! the saved provider session exists but cannot be resolved while the store
//! is locked or unreachable.
//!
//! The fixture
//! `/fixtures/auth/credential_state_cases/seed_locked_keychain_on_launch.json`
//! exercises the named failure drill end to end.

use serde::{Deserialize, Serialize};

pub use crate::browser_callback::{
    AccountBoundaryClass, IdentityModeAlias, RetryPathClass, TrustState,
};

/// Record-kind tag carried on serialized [`CredentialStateRow`] payloads.
pub const CREDENTIAL_STATE_ROW_RECORD_KIND: &str = "credential_state_row_seed_record";

/// Record-kind tag carried on serialized [`ProviderAccountRecord`] payloads.
pub const PROVIDER_ACCOUNT_RECORD_KIND: &str = "provider_account_seed_record";

/// Record-kind tag carried on serialized [`ProviderAccountRegistry`] payloads.
pub const PROVIDER_ACCOUNT_REGISTRY_RECORD_KIND: &str = "provider_account_registry_seed_record";

/// Schema version of the credential-state seed payloads.
///
/// Bumped on breaking payload changes; additive-optional fields do not bump
/// this version. The frozen cross-tool boundary contract in
/// `/schemas/auth/credential_state.schema.json` follows the same versioning
/// rule.
pub const CREDENTIAL_STATE_SEED_SCHEMA_VERSION: u32 = 1;

/// Closed credential-state vocabulary mirrored from
/// `/docs/auth/credential_state_and_secret_prompt_contract.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialStateClass {
    /// No alias, handle, delegated credential, or approved source exists for
    /// the requested scope. Local non-credential work continues.
    Absent,
    /// A broker alias or handle is present and consumers receive the handle,
    /// not raw material.
    HandleOnly,
    /// The credential source is available for the declared scope.
    Available,
    /// A known store exists but is locked or waiting for user / platform
    /// unlock.
    Locked,
    /// A handle, delegated credential, session token, or callback window
    /// passed its expiry.
    Expired,
    /// The issuer, broker, admin policy, or user revoked the authority.
    Revoked,
    /// The source credential changed and old handles are no longer
    /// authoritative.
    Rotated,
    /// The expected secure store is unreachable, corrupted, unsupported, or
    /// unavailable on this host.
    StoreUnavailable,
    /// Current org, workspace, trust, or admin policy forbids the credential
    /// source, projection, target, or action.
    PolicyBlocked,
}

impl CredentialStateClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Absent => "absent",
            Self::HandleOnly => "handle_only",
            Self::Available => "available",
            Self::Locked => "locked",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
            Self::Rotated => "rotated",
            Self::StoreUnavailable => "store_unavailable",
            Self::PolicyBlocked => "policy_blocked",
        }
    }

    /// True when the named state class blocks credential resolution and the
    /// surface MUST render the row as unavailable rather than a generic
    /// "Connected" / "Signed in" badge.
    pub const fn is_unavailable_class(self) -> bool {
        matches!(
            self,
            Self::Absent
                | Self::Locked
                | Self::Expired
                | Self::Revoked
                | Self::Rotated
                | Self::StoreUnavailable
                | Self::PolicyBlocked
        )
    }

    /// True when the surface MUST emit a visible recovery row alongside the
    /// credential-state row. `Available` and `HandleOnly` rows do not need a
    /// recovery row by default.
    pub const fn requires_visible_recovery(self) -> bool {
        self.is_unavailable_class()
    }
}

/// Closed storage-mode vocabulary the row carries in product copy, docs,
/// telemetry, support exports, and admin explanations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageModeClass {
    SystemCredentialStore,
    EnterpriseSecretStore,
    SessionOnly,
    HandleOnly,
    Delegated,
    NotConfigured,
}

impl StorageModeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemCredentialStore => "system_credential_store",
            Self::EnterpriseSecretStore => "enterprise_secret_store",
            Self::SessionOnly => "session_only",
            Self::HandleOnly => "handle_only",
            Self::Delegated => "delegated",
            Self::NotConfigured => "not_configured",
        }
    }
}

/// Closed store-source vocabulary the row carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StoreSourceClass {
    OsKeychain,
    EnterpriseVaultAdapter,
    AgentSocket,
    FileBackedSecretRef,
    RemoteSessionScopedHandle,
    HardwareBackedOrPasskeyAdjacent,
    ManagedPolicyInjector,
    BrowserDeviceCodeHandoff,
    NoSecureStore,
}

impl StoreSourceClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsKeychain => "os_keychain",
            Self::EnterpriseVaultAdapter => "enterprise_vault_adapter",
            Self::AgentSocket => "agent_socket",
            Self::FileBackedSecretRef => "file_backed_secret_ref",
            Self::RemoteSessionScopedHandle => "remote_session_scoped_handle",
            Self::HardwareBackedOrPasskeyAdjacent => "hardware_backed_or_passkey_adjacent",
            Self::ManagedPolicyInjector => "managed_policy_injector",
            Self::BrowserDeviceCodeHandoff => "browser_device_code_handoff",
            Self::NoSecureStore => "no_secure_store",
        }
    }
}

/// Closed lifetime vocabulary used by the seed row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifetimeClass {
    OperationScoped,
    SessionOnly,
    TimeBounded,
    PersistentUntilRevoked,
    RotatedSuccessorRequired,
    Unavailable,
}

impl LifetimeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OperationScoped => "operation_scoped",
            Self::SessionOnly => "session_only",
            Self::TimeBounded => "time_bounded",
            Self::PersistentUntilRevoked => "persistent_until_revoked",
            Self::RotatedSuccessorRequired => "rotated_successor_required",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Closed revoke-action vocabulary the row exposes verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevokeActionClass {
    /// Remove a saved local provider session from the secure store. Local
    /// editing, save, undo, search, local Git, and local tasks remain
    /// available.
    RemoveSavedProviderSession,
    /// Sign out of the managed (vendor-hosted convenience) workspace. Local
    /// work stays on the device.
    SignOutOfManagedSession,
    /// Sign out of the customer-run (self-hosted) IdP. Local work stays on
    /// the device.
    SignOutOfSelfHostedSession,
    /// Disconnect a connected provider account. Local non-credential work
    /// continues.
    DisconnectProviderAccount,
    /// Rotate the credential and rebind any handles to the successor.
    RotateAndRebindHandle,
    /// Purge the session-only credential held in broker memory. Reprompt is
    /// required after restart.
    PurgeSessionOnlyCredential,
    /// No durable revoke action exists from this row; the row points at
    /// admin / policy escalation.
    NoRevokeActionAvailable,
}

impl RevokeActionClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RemoveSavedProviderSession => "remove_saved_provider_session",
            Self::SignOutOfManagedSession => "sign_out_of_managed_session",
            Self::SignOutOfSelfHostedSession => "sign_out_of_self_hosted_session",
            Self::DisconnectProviderAccount => "disconnect_provider_account",
            Self::RotateAndRebindHandle => "rotate_and_rebind_handle",
            Self::PurgeSessionOnlyCredential => "purge_session_only_credential",
            Self::NoRevokeActionAvailable => "no_revoke_action_available",
        }
    }
}

/// Closed unavailable-reason vocabulary the row carries when the credential
/// cannot be resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialUnavailableReason {
    /// The expected secure store is locked or waiting for user / platform
    /// unlock.
    StoreLocked,
    /// The expected secure store is unreachable, corrupted, unsupported, or
    /// unavailable on this host.
    StoreUnavailable,
    /// No secure store is configured on this host. Session-only fallback may
    /// be admissible where class and policy allow.
    NoSecureStoreConfigured,
    /// Current org, workspace, trust, or admin policy forbids the credential
    /// source, projection, target, or action.
    PolicyBlocked,
    /// No alias, handle, delegated credential, or approved source exists for
    /// the requested scope.
    CredentialMissing,
    /// A handle, delegated credential, session token, or callback window
    /// passed its expiry.
    CredentialExpired,
}

impl CredentialUnavailableReason {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StoreLocked => "store_locked",
            Self::StoreUnavailable => "store_unavailable",
            Self::NoSecureStoreConfigured => "no_secure_store_configured",
            Self::PolicyBlocked => "policy_blocked",
            Self::CredentialMissing => "credential_missing",
            Self::CredentialExpired => "credential_expired",
        }
    }
}

/// Scope the credential authority applies to. Carries no raw secret material
/// and no raw URLs / tenant names; only export-safe labels and opaque refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialScope {
    pub scope_label: String,
    pub audience_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_workspace_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_tenant_or_org_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_actor_subject_ref: Option<String>,
}

/// Lifetime block: when the authority was issued, when it expires, and which
/// revoke action returns control to the user. The seed never exposes the raw
/// token expiry payload — only export-safe ISO-8601 timestamps and a typed
/// revoke action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialLifetime {
    pub lifetime_class: LifetimeClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    pub revocation_path_label: String,
    pub revoke_action: RevokeActionClass,
}

/// Storage posture the row exposes verbatim. The seed contract forbids
/// long-lived plaintext fallback and never carries raw secret material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoragePosture {
    pub storage_mode: StorageModeClass,
    pub store_source: StoreSourceClass,
    pub session_only_downgrade_visible: bool,
    /// The seed contract forbids long-lived plaintext fallback. The field is
    /// kept on the wire so any downstream surface that reads the row asserts
    /// the rule explicitly rather than inferring it.
    pub plaintext_fallback_allowed: bool,
    /// Raw secret material never appears in seed records. Raw material
    /// observed in a workspace file is represented by a denial / audit row,
    /// not by setting this field true.
    pub raw_secret_material_present: bool,
    pub storage_note: String,
}

/// Canonical seed [`CredentialStateRow`] record.
///
/// Surfaces (terminal pane, activity center, status bar, support / export
/// flows) consume this object and quote its fields verbatim. They do not
/// re-derive a generic `Connected` / `Signed in` badge, never collapse a
/// locked or unavailable store posture into a generic "warning" chip, and
/// never silently fall back to plaintext-file credential storage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialStateRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub credential_state_id: String,
    pub state_class: CredentialStateClass,
    pub display_label: String,
    /// Opaque ref into the [`ProviderAccountRecord`] this row authorities
    /// against. The registry uses this to join account records to their
    /// credential-state rows without inlining the full record.
    pub provider_account_ref: String,
    pub source_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority_alias_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority_handle_ref: Option<String>,
    pub scope: CredentialScope,
    pub storage: StoragePosture,
    pub lifetime: CredentialLifetime,
    pub identity_mode: IdentityModeAlias,
    pub trust_state: TrustState,
    pub local_work_continues: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<CredentialUnavailableReason>,
    pub recovery_copy_label: String,
    pub primary_recovery_action: RetryPathClass,
    /// Optional ref into the execution-context lane so the credential-state
    /// row and the canonical [`aureline_runtime::ExecutionContext`] stay
    /// joined for a support export.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_ref: Option<String>,
    pub minted_at: String,
}

impl CredentialStateRow {
    /// True when the row sits in any unavailable state class.
    pub fn is_unavailable(&self) -> bool {
        self.state_class.is_unavailable_class()
    }

    /// True when the row's local-work continuity hint is still set; the seed
    /// contract requires the no-account local path to remain readable on
    /// every protected row.
    pub fn local_work_continues(&self) -> bool {
        self.local_work_continues
    }

    /// True when the surface MUST emit a visible recovery row alongside this
    /// credential-state row.
    pub fn requires_visible_recovery(&self) -> bool {
        self.state_class.requires_visible_recovery()
    }
}

/// Provider / account registry seed: one connected account or provider
/// record. Reusable by later browser handoff and publish-later work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderAccountRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub provider_account_id: String,
    pub provider_domain_label: String,
    pub destination_class_label: String,
    pub account_boundary_class: AccountBoundaryClass,
    pub identity_mode: IdentityModeAlias,
    pub trust_state: TrustState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_workspace_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_tenant_or_org_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_actor_subject_ref: Option<String>,
    /// Refs into [`CredentialStateRow::credential_state_id`] entries owned by
    /// the same registry.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub credential_state_row_refs: Vec<String>,
    pub minted_at: String,
}

/// Provider / account registry seed: one inspectable join object that pairs
/// connected-account / provider records with their credential-state rows.
///
/// The registry is small and deliberately additive: later browser handoff,
/// device-code, and publish-later work consume the same shape rather than
/// minting a private "provider list" cache.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderAccountRegistry {
    pub record_kind: String,
    pub schema_version: u32,
    pub registry_id: String,
    pub accounts: Vec<ProviderAccountRecord>,
    pub credential_states: Vec<CredentialStateRow>,
    pub minted_at: String,
}

impl ProviderAccountRegistry {
    /// Mint an empty registry.
    pub fn new(registry_id: impl Into<String>, minted_at: impl Into<String>) -> Self {
        Self {
            record_kind: PROVIDER_ACCOUNT_REGISTRY_RECORD_KIND.to_owned(),
            schema_version: CREDENTIAL_STATE_SEED_SCHEMA_VERSION,
            registry_id: registry_id.into(),
            accounts: Vec::new(),
            credential_states: Vec::new(),
            minted_at: minted_at.into(),
        }
    }

    /// Register a provider / account record. Existing entries with the same
    /// `provider_account_id` are replaced; the seed registry is small enough
    /// that linear search is honest.
    pub fn upsert_account(&mut self, record: ProviderAccountRecord) {
        if let Some(slot) = self
            .accounts
            .iter_mut()
            .find(|entry| entry.provider_account_id == record.provider_account_id)
        {
            *slot = record;
        } else {
            self.accounts.push(record);
        }
    }

    /// Register a credential-state row.
    pub fn upsert_credential_state(&mut self, row: CredentialStateRow) {
        if let Some(slot) = self
            .credential_states
            .iter_mut()
            .find(|entry| entry.credential_state_id == row.credential_state_id)
        {
            *slot = row;
        } else {
            self.credential_states.push(row);
        }
    }

    /// Iterate credential-state rows authorities-bound to the named account.
    pub fn rows_for_account<'a>(
        &'a self,
        provider_account_id: &'a str,
    ) -> impl Iterator<Item = &'a CredentialStateRow> + 'a {
        self.credential_states
            .iter()
            .filter(move |row| row.provider_account_ref == provider_account_id)
    }

    /// Find a credential-state row by id.
    pub fn find_row(&self, credential_state_id: &str) -> Option<&CredentialStateRow> {
        self.credential_states
            .iter()
            .find(|row| row.credential_state_id == credential_state_id)
    }

    /// Lock every credential-state row backed by the named store source.
    /// Failure-drill helper: the saved alias survives, the typed
    /// [`CredentialStateClass::Locked`] state class fires, the unavailable
    /// reason flips to [`CredentialUnavailableReason::StoreLocked`], and the
    /// recovery copy + action switch to the unlock path. No raw material is
    /// touched.
    pub fn lock_store(&mut self, store_source: StoreSourceClass) -> usize {
        let mut affected = 0;
        for row in self.credential_states.iter_mut() {
            if row.storage.store_source == store_source {
                row.state_class = CredentialStateClass::Locked;
                row.unavailable_reason = Some(CredentialUnavailableReason::StoreLocked);
                row.primary_recovery_action = RetryPathClass::ResumeAfterCredentialStoreUnlock;
                row.recovery_copy_label = format!(
                    "{} is locked. Unlock the store to resolve the saved credential. \
                     Local work keeps saving to this device.",
                    store_source_display(store_source),
                );
                row.lifetime.lifetime_class = LifetimeClass::Unavailable;
                affected += 1;
            }
        }
        affected
    }

    /// Mark every credential-state row backed by the named store source as
    /// store-unavailable. Failure-drill helper: the saved alias survives, the
    /// typed [`CredentialStateClass::StoreUnavailable`] state class fires,
    /// the unavailable reason flips to
    /// [`CredentialUnavailableReason::StoreUnavailable`], and the recovery
    /// copy + action switch to the store-recovery path. The seed contract
    /// forbids a silent plaintext-file fallback.
    pub fn mark_store_unavailable(&mut self, store_source: StoreSourceClass) -> usize {
        let mut affected = 0;
        for row in self.credential_states.iter_mut() {
            if row.storage.store_source == store_source {
                row.state_class = CredentialStateClass::StoreUnavailable;
                row.unavailable_reason = Some(CredentialUnavailableReason::StoreUnavailable);
                row.primary_recovery_action = RetryPathClass::ContactSupportWithExport;
                row.recovery_copy_label = format!(
                    "{} is unavailable on this host. The saved credential cannot be \
                     resolved. Local work keeps saving to this device.",
                    store_source_display(store_source),
                );
                row.lifetime.lifetime_class = LifetimeClass::Unavailable;
                row.storage.session_only_downgrade_visible = false;
                row.storage.plaintext_fallback_allowed = false;
                affected += 1;
            }
        }
        affected
    }
}

const fn store_source_display(class: StoreSourceClass) -> &'static str {
    match class {
        StoreSourceClass::OsKeychain => "OS keychain",
        StoreSourceClass::EnterpriseVaultAdapter => "Enterprise vault",
        StoreSourceClass::AgentSocket => "Agent socket",
        StoreSourceClass::FileBackedSecretRef => "File-backed secret reference",
        StoreSourceClass::RemoteSessionScopedHandle => "Remote session-scoped handle",
        StoreSourceClass::HardwareBackedOrPasskeyAdjacent => {
            "Hardware-backed or passkey-adjacent credential"
        }
        StoreSourceClass::ManagedPolicyInjector => "Managed policy injector",
        StoreSourceClass::BrowserDeviceCodeHandoff => "Browser or device-code handoff",
        StoreSourceClass::NoSecureStore => "No secure store",
    }
}

/// Stable credential-state chip a consumer renders on a protected row.
///
/// The chip carries the credential-state class token, the storage-mode and
/// store-source tokens, the scope and lifetime summary, the typed
/// unavailable reason (when present), and the revoke-action and primary
/// recovery action tokens so a support export can round-trip the same truth
/// a terminal-pane row, an activity-center row, and a status mirror render.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialStateChip {
    pub credential_state_id: String,
    pub provider_account_ref: String,
    pub state_class: CredentialStateClass,
    pub state_class_token: String,
    pub display_label: String,
    pub storage_mode: StorageModeClass,
    pub storage_mode_token: String,
    pub store_source: StoreSourceClass,
    pub store_source_token: String,
    pub scope_label: String,
    pub audience_label: String,
    pub lifetime_class: LifetimeClass,
    pub lifetime_class_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    pub revocation_path_label: String,
    pub revoke_action: RevokeActionClass,
    pub revoke_action_token: String,
    pub identity_mode: IdentityModeAlias,
    pub trust_state: TrustState,
    pub local_work_continues: bool,
    pub visible_recovery_required: bool,
    pub recovery_copy_label: String,
    pub primary_recovery_action: RetryPathClass,
    pub primary_recovery_action_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<CredentialUnavailableReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason_token: Option<String>,
    pub session_only_downgrade_visible: bool,
    pub plaintext_fallback_allowed: bool,
    pub raw_secret_material_present: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_ref: Option<String>,
}

impl CredentialStateChip {
    /// Project a chip from a [`CredentialStateRow`].
    pub fn from_row(row: &CredentialStateRow) -> Self {
        Self {
            credential_state_id: row.credential_state_id.clone(),
            provider_account_ref: row.provider_account_ref.clone(),
            state_class: row.state_class,
            state_class_token: row.state_class.as_str().to_owned(),
            display_label: row.display_label.clone(),
            storage_mode: row.storage.storage_mode,
            storage_mode_token: row.storage.storage_mode.as_str().to_owned(),
            store_source: row.storage.store_source,
            store_source_token: row.storage.store_source.as_str().to_owned(),
            scope_label: row.scope.scope_label.clone(),
            audience_label: row.scope.audience_label.clone(),
            lifetime_class: row.lifetime.lifetime_class,
            lifetime_class_token: row.lifetime.lifetime_class.as_str().to_owned(),
            expires_at: row.lifetime.expires_at.clone(),
            revocation_path_label: row.lifetime.revocation_path_label.clone(),
            revoke_action: row.lifetime.revoke_action,
            revoke_action_token: row.lifetime.revoke_action.as_str().to_owned(),
            identity_mode: row.identity_mode,
            trust_state: row.trust_state,
            local_work_continues: row.local_work_continues,
            visible_recovery_required: row.requires_visible_recovery(),
            recovery_copy_label: row.recovery_copy_label.clone(),
            primary_recovery_action: row.primary_recovery_action,
            primary_recovery_action_token: row.primary_recovery_action.as_str().to_owned(),
            unavailable_reason: row.unavailable_reason,
            unavailable_reason_token: row.unavailable_reason.map(|r| r.as_str().to_owned()),
            session_only_downgrade_visible: row.storage.session_only_downgrade_visible,
            plaintext_fallback_allowed: row.storage.plaintext_fallback_allowed,
            raw_secret_material_present: row.storage.raw_secret_material_present,
            execution_context_ref: row.execution_context_ref.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn account_free_local_byok_ai_row() -> CredentialStateRow {
        CredentialStateRow {
            record_kind: CREDENTIAL_STATE_ROW_RECORD_KIND.to_owned(),
            schema_version: CREDENTIAL_STATE_SEED_SCHEMA_VERSION,
            credential_state_id: "credential_state.local.byok_ai.0001".to_owned(),
            state_class: CredentialStateClass::HandleOnly,
            display_label: "Local BYOK AI provider".to_owned(),
            provider_account_ref: "provider_account.local.byok_ai".to_owned(),
            source_label: "OS keychain item".to_owned(),
            authority_alias_ref: Some("credential_alias.byok_ai.default".to_owned()),
            authority_handle_ref: Some("credential_handle.byok_ai.default".to_owned()),
            scope: CredentialScope {
                scope_label: "Current local workspace".to_owned(),
                audience_label: "Local AI provider requests".to_owned(),
                bound_workspace_ref: Some("workspace.local.demo".to_owned()),
                bound_tenant_or_org_ref: None,
                bound_actor_subject_ref: None,
            },
            storage: StoragePosture {
                storage_mode: StorageModeClass::SystemCredentialStore,
                store_source: StoreSourceClass::OsKeychain,
                session_only_downgrade_visible: false,
                plaintext_fallback_allowed: false,
                raw_secret_material_present: false,
                storage_note: "OS keychain holds the BYOK AI alias; the broker resolves the \
                               handle on demand without exposing raw material."
                    .to_owned(),
            },
            lifetime: CredentialLifetime {
                lifetime_class: LifetimeClass::PersistentUntilRevoked,
                issued_at: Some("2026-04-29T09:00:00Z".to_owned()),
                expires_at: None,
                revocation_path_label: "Remove saved BYOK AI key".to_owned(),
                revoke_action: RevokeActionClass::RemoveSavedProviderSession,
            },
            identity_mode: IdentityModeAlias::AccountFreeLocal,
            trust_state: TrustState::Trusted,
            local_work_continues: true,
            unavailable_reason: None,
            recovery_copy_label: "BYOK AI is ready. Local work stays on this device.".to_owned(),
            primary_recovery_action: RetryPathClass::ContinueLocalWithoutSignIn,
            execution_context_ref: Some(
                "execution_context.local_desktop.workspace_root".to_owned(),
            ),
            minted_at: "2026-04-29T09:05:00Z".to_owned(),
        }
    }

    fn managed_provider_session_row() -> CredentialStateRow {
        CredentialStateRow {
            record_kind: CREDENTIAL_STATE_ROW_RECORD_KIND.to_owned(),
            schema_version: CREDENTIAL_STATE_SEED_SCHEMA_VERSION,
            credential_state_id: "credential_state.managed.payments_prod.0001".to_owned(),
            state_class: CredentialStateClass::HandleOnly,
            display_label: "Managed provider session".to_owned(),
            provider_account_ref: "provider_account.managed.payments_prod".to_owned(),
            source_label: "OS keychain item".to_owned(),
            authority_alias_ref: Some("credential_alias.managed.payments_prod".to_owned()),
            authority_handle_ref: Some("credential_handle.managed.payments_prod".to_owned()),
            scope: CredentialScope {
                scope_label: "payments-prod workspace".to_owned(),
                audience_label: "Managed sign-in refresh".to_owned(),
                bound_workspace_ref: Some("workspace.payments_prod".to_owned()),
                bound_tenant_or_org_ref: Some("tenant.acme_prod".to_owned()),
                bound_actor_subject_ref: Some("actor_subject.sam.acme".to_owned()),
            },
            storage: StoragePosture {
                storage_mode: StorageModeClass::SystemCredentialStore,
                store_source: StoreSourceClass::OsKeychain,
                session_only_downgrade_visible: false,
                plaintext_fallback_allowed: false,
                raw_secret_material_present: false,
                storage_note: "OS keychain holds the managed provider-session alias; the \
                               broker resolves the handle on demand."
                    .to_owned(),
            },
            lifetime: CredentialLifetime {
                lifetime_class: LifetimeClass::PersistentUntilRevoked,
                issued_at: Some("2026-04-29T09:00:00Z".to_owned()),
                expires_at: None,
                revocation_path_label: "Remove saved provider session".to_owned(),
                revoke_action: RevokeActionClass::RemoveSavedProviderSession,
            },
            identity_mode: IdentityModeAlias::ManagedConvenience,
            trust_state: TrustState::Trusted,
            local_work_continues: true,
            unavailable_reason: None,
            recovery_copy_label: "Managed sign-in is ready. Local work keeps saving to this \
                                  device."
                .to_owned(),
            primary_recovery_action: RetryPathClass::RetryInSystemBrowser,
            execution_context_ref: Some(
                "execution_context.auth.managed_sign_in.payments_prod".to_owned(),
            ),
            minted_at: "2026-04-29T09:05:00Z".to_owned(),
        }
    }

    fn baseline_registry() -> ProviderAccountRegistry {
        let mut registry = ProviderAccountRegistry::new(
            "provider_account_registry.seed.0001",
            "2026-04-29T09:05:00Z",
        );
        registry.upsert_account(ProviderAccountRecord {
            record_kind: PROVIDER_ACCOUNT_RECORD_KIND.to_owned(),
            schema_version: CREDENTIAL_STATE_SEED_SCHEMA_VERSION,
            provider_account_id: "provider_account.local.byok_ai".to_owned(),
            provider_domain_label: "Local BYOK AI provider".to_owned(),
            destination_class_label: "BYOK AI key (local)".to_owned(),
            account_boundary_class: AccountBoundaryClass::LocalOnly,
            identity_mode: IdentityModeAlias::AccountFreeLocal,
            trust_state: TrustState::Trusted,
            bound_workspace_ref: Some("workspace.local.demo".to_owned()),
            bound_tenant_or_org_ref: None,
            bound_actor_subject_ref: None,
            credential_state_row_refs: vec!["credential_state.local.byok_ai.0001".to_owned()],
            minted_at: "2026-04-29T09:05:00Z".to_owned(),
        });
        registry.upsert_account(ProviderAccountRecord {
            record_kind: PROVIDER_ACCOUNT_RECORD_KIND.to_owned(),
            schema_version: CREDENTIAL_STATE_SEED_SCHEMA_VERSION,
            provider_account_id: "provider_account.managed.payments_prod".to_owned(),
            provider_domain_label: "login.acme.example".to_owned(),
            destination_class_label: "Customer-managed identity provider (system browser)"
                .to_owned(),
            account_boundary_class: AccountBoundaryClass::Managed,
            identity_mode: IdentityModeAlias::ManagedConvenience,
            trust_state: TrustState::Trusted,
            bound_workspace_ref: Some("workspace.payments_prod".to_owned()),
            bound_tenant_or_org_ref: Some("tenant.acme_prod".to_owned()),
            bound_actor_subject_ref: Some("actor_subject.sam.acme".to_owned()),
            credential_state_row_refs: vec![
                "credential_state.managed.payments_prod.0001".to_owned()
            ],
            minted_at: "2026-04-29T09:05:00Z".to_owned(),
        });
        registry.upsert_credential_state(account_free_local_byok_ai_row());
        registry.upsert_credential_state(managed_provider_session_row());
        registry
    }

    #[test]
    fn registry_upsert_replaces_existing_records_by_id() {
        let mut registry = baseline_registry();
        let updated = ProviderAccountRecord {
            record_kind: PROVIDER_ACCOUNT_RECORD_KIND.to_owned(),
            schema_version: CREDENTIAL_STATE_SEED_SCHEMA_VERSION,
            provider_account_id: "provider_account.managed.payments_prod".to_owned(),
            provider_domain_label: "login.acme.example".to_owned(),
            destination_class_label: "Customer-managed identity provider (system browser)"
                .to_owned(),
            account_boundary_class: AccountBoundaryClass::RestrictedManagedOnly,
            identity_mode: IdentityModeAlias::ManagedConvenience,
            trust_state: TrustState::Trusted,
            bound_workspace_ref: Some("workspace.payments_prod".to_owned()),
            bound_tenant_or_org_ref: Some("tenant.acme_prod".to_owned()),
            bound_actor_subject_ref: Some("actor_subject.sam.acme".to_owned()),
            credential_state_row_refs: vec![
                "credential_state.managed.payments_prod.0001".to_owned()
            ],
            minted_at: "2026-04-29T09:10:00Z".to_owned(),
        };
        registry.upsert_account(updated);
        assert_eq!(registry.accounts.len(), 2);
        let found = registry
            .accounts
            .iter()
            .find(|account| account.provider_account_id == "provider_account.managed.payments_prod")
            .expect("upserted record stays addressable");
        assert_eq!(
            found.account_boundary_class,
            AccountBoundaryClass::RestrictedManagedOnly
        );
    }

    #[test]
    fn baseline_chip_quotes_storage_scope_and_revoke_action_verbatim() {
        let registry = baseline_registry();
        let row = registry
            .find_row("credential_state.managed.payments_prod.0001")
            .expect("registry exposes baseline row");
        let chip = CredentialStateChip::from_row(row);
        assert_eq!(chip.state_class, CredentialStateClass::HandleOnly);
        assert_eq!(chip.state_class_token, "handle_only");
        assert_eq!(chip.storage_mode, StorageModeClass::SystemCredentialStore);
        assert_eq!(chip.storage_mode_token, "system_credential_store");
        assert_eq!(chip.store_source, StoreSourceClass::OsKeychain);
        assert_eq!(chip.store_source_token, "os_keychain");
        assert_eq!(chip.scope_label, "payments-prod workspace");
        assert_eq!(chip.audience_label, "Managed sign-in refresh");
        assert_eq!(chip.revocation_path_label, "Remove saved provider session");
        assert_eq!(
            chip.revoke_action,
            RevokeActionClass::RemoveSavedProviderSession
        );
        assert_eq!(chip.revoke_action_token, "remove_saved_provider_session");
        assert!(chip.local_work_continues);
        assert!(!chip.visible_recovery_required);
        assert!(!chip.plaintext_fallback_allowed);
        assert!(!chip.raw_secret_material_present);
    }

    #[test]
    fn lock_store_failure_drill_keeps_alias_and_flips_to_locked_state() {
        // Failure drill: the OS keychain locks while the seed row is staged.
        // The alias and handle refs survive, the state class flips to
        // `locked`, the unavailable reason flips to `store_locked`, and the
        // recovery path switches to `resume_after_credential_store_unlock`.
        // The seed contract forbids a silent plaintext-file fallback.
        let mut registry = baseline_registry();
        let affected = registry.lock_store(StoreSourceClass::OsKeychain);
        assert_eq!(affected, 2);

        let row = registry
            .find_row("credential_state.managed.payments_prod.0001")
            .expect("registry exposes locked row");
        assert_eq!(row.state_class, CredentialStateClass::Locked);
        assert_eq!(
            row.unavailable_reason,
            Some(CredentialUnavailableReason::StoreLocked)
        );
        assert_eq!(
            row.primary_recovery_action,
            RetryPathClass::ResumeAfterCredentialStoreUnlock
        );
        assert!(row.is_unavailable());
        assert!(row.requires_visible_recovery());
        assert!(row.local_work_continues());
        assert_eq!(
            row.authority_alias_ref.as_deref(),
            Some("credential_alias.managed.payments_prod"),
            "alias survives the lock so the unlock prompt can resolve the same handle",
        );
        assert!(!row.storage.plaintext_fallback_allowed);
        assert!(!row.storage.raw_secret_material_present);

        let chip = CredentialStateChip::from_row(row);
        assert_eq!(chip.state_class_token, "locked");
        assert_eq!(
            chip.unavailable_reason,
            Some(CredentialUnavailableReason::StoreLocked)
        );
        assert_eq!(
            chip.unavailable_reason_token.as_deref(),
            Some("store_locked")
        );
        assert!(chip.visible_recovery_required);
        assert!(chip.local_work_continues);
    }

    #[test]
    fn mark_store_unavailable_failure_drill_blocks_plaintext_fallback() {
        // Failure drill: the OS keychain is unreachable on this host. The
        // seed contract forbids a silent plaintext-file fallback, the row
        // surfaces the `store_unavailable` state class with a typed
        // unavailable reason, and the recovery path routes to support
        // export so the user can ask for help without leaking material.
        let mut registry = baseline_registry();
        let affected = registry.mark_store_unavailable(StoreSourceClass::OsKeychain);
        assert_eq!(affected, 2);

        for row in &registry.credential_states {
            assert_eq!(row.state_class, CredentialStateClass::StoreUnavailable);
            assert_eq!(
                row.unavailable_reason,
                Some(CredentialUnavailableReason::StoreUnavailable)
            );
            assert!(row.is_unavailable());
            assert!(row.local_work_continues());
            assert!(!row.storage.plaintext_fallback_allowed);
            assert!(!row.storage.session_only_downgrade_visible);
        }
    }

    #[test]
    fn rows_for_account_filters_by_account_ref() {
        let registry = baseline_registry();
        let local: Vec<&CredentialStateRow> = registry
            .rows_for_account("provider_account.local.byok_ai")
            .collect();
        let managed: Vec<&CredentialStateRow> = registry
            .rows_for_account("provider_account.managed.payments_prod")
            .collect();
        assert_eq!(local.len(), 1);
        assert_eq!(managed.len(), 1);
        assert_eq!(
            local[0].credential_state_id,
            "credential_state.local.byok_ai.0001"
        );
        assert_eq!(
            managed[0].credential_state_id,
            "credential_state.managed.payments_prod.0001"
        );
    }

    #[test]
    fn record_kind_and_schema_version_constants_match_records() {
        assert_eq!(
            CREDENTIAL_STATE_ROW_RECORD_KIND,
            "credential_state_row_seed_record"
        );
        assert_eq!(PROVIDER_ACCOUNT_RECORD_KIND, "provider_account_seed_record");
        assert_eq!(
            PROVIDER_ACCOUNT_REGISTRY_RECORD_KIND,
            "provider_account_registry_seed_record"
        );
        assert_eq!(CREDENTIAL_STATE_SEED_SCHEMA_VERSION, 1);

        let row = account_free_local_byok_ai_row();
        assert_eq!(row.record_kind, CREDENTIAL_STATE_ROW_RECORD_KIND);
        assert_eq!(row.schema_version, CREDENTIAL_STATE_SEED_SCHEMA_VERSION);

        let registry = baseline_registry();
        assert_eq!(registry.record_kind, PROVIDER_ACCOUNT_REGISTRY_RECORD_KIND);
        assert_eq!(
            registry.schema_version,
            CREDENTIAL_STATE_SEED_SCHEMA_VERSION
        );
    }
}
