//! Secret-broker alpha rows, continuity results, and redaction-safe export.
//!
//! This module owns the first executable secret-broker alpha contract. It
//! turns the broker vocabulary frozen by the security documents into
//! serializable Rust records that provider, registry, database, and remote
//! lanes can quote without copying raw secret material into normal settings,
//! support bundles, or shell-local state.
//!
//! The alpha deliberately covers the first claimed rows only:
//!
//! - OS credential-store handles for registry auth and managed refresh;
//! - session-only broker memory when a class and policy allow a degraded
//!   in-process fallback;
//! - delegated credentials for remote and tunnel reuse; and
//! - locked, unavailable, and trust-store-changed continuity results that
//!   preserve local work while pausing credentialed capabilities.
//!
//! The reviewer-facing landing page is
//! [`/docs/security/secret_broker_alpha.md`](../../../../docs/security/secret_broker_alpha.md).

use serde::{Deserialize, Serialize};

pub use crate::browser_callback::{IdentityModeAlias, TrustState};
pub use crate::credential_state::{StorageModeClass, StoreSourceClass};

/// Record-kind tag carried on [`SecretBrokerAlphaPacket`] payloads.
pub const SECRET_BROKER_ALPHA_PACKET_RECORD_KIND: &str = "secret_broker_alpha_packet_record";

/// Record-kind tag carried on [`SecretBrokerAlphaRow`] payloads.
pub const SECRET_BROKER_ROW_RECORD_KIND: &str = "secret_broker_alpha_row_record";

/// Record-kind tag carried on [`SecretBrokerSupportExport`] payloads.
pub const SECRET_BROKER_SUPPORT_EXPORT_RECORD_KIND: &str = "secret_broker_support_export_record";

/// Record-kind tag carried on [`SecretBrokerSupportExportRow`] payloads.
pub const SECRET_BROKER_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "secret_broker_support_export_row_record";

/// Schema version for the secret-broker alpha payloads.
///
/// Bump only for breaking payload changes. Additive optional fields do not
/// bump the alpha schema.
pub const SECRET_BROKER_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Secret classes admitted by the broker alpha.
///
/// Each variant maps to the frozen class matrix. A row carries exactly one
/// class so support export can explain what kind of authority exists without
/// exposing secret bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretClass {
    /// AI provider API keys, gateway keys, and BYOK session tokens.
    AiProviderToken,
    /// Code host, review, and artifact-host tokens.
    CodeHostToken,
    /// Package and container registry tokens.
    PackageRegistryToken,
    /// Database passwords, DSN tokens, and warehouse credentials.
    DatabaseCredential,
    /// SSH private keys, deploy keys, and agent-mediated SSH material.
    SshKeyMaterial,
    /// mTLS client certificates and matching private-key bindings.
    ClientCertificate,
    /// Release signing keys, notarization credentials, and SBOM signer keys.
    SigningKeyMaterial,
    /// Refresh tokens and delegated collaboration session tokens.
    ProviderSession,
    /// Device-binding keys, recovery tokens, and passkey credential refs.
    DeviceSecret,
    /// Operation-scoped tokens minted by token-exchange flows.
    EphemeralOperationToken,
}

impl SecretClass {
    /// Stable string token used in fixtures, support export, and shell rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiProviderToken => "ai_provider_token",
            Self::CodeHostToken => "code_host_token",
            Self::PackageRegistryToken => "package_registry_token",
            Self::DatabaseCredential => "database_credential",
            Self::SshKeyMaterial => "ssh_key_material",
            Self::ClientCertificate => "client_certificate",
            Self::SigningKeyMaterial => "signing_key_material",
            Self::ProviderSession => "provider_session",
            Self::DeviceSecret => "device_secret",
            Self::EphemeralOperationToken => "ephemeral_operation_token",
        }
    }
}

/// Reference mode used by a claimed secret-broker alpha row.
///
/// The mode tells a consumer whether it is holding a broker handle, a
/// session-only broker-memory reference, or a delegated credential reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretReferenceMode {
    /// The row references a broker-issued credential handle.
    Handle,
    /// The row references process-local broker memory that expires with the
    /// session.
    SessionOnly,
    /// The row references a scoped delegated credential or installation grant.
    Delegated,
}

impl SecretReferenceMode {
    /// Stable string token used in fixtures, support export, and shell rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Handle => "handle",
            Self::SessionOnly => "session_only",
            Self::Delegated => "delegated",
        }
    }
}

/// Trust-store classes that can back a secret-broker row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustStoreClass {
    /// Platform credential store such as Keychain, Credential Manager, or
    /// Secret Service.
    OsKeychain,
    /// Enterprise vault or broker adapter.
    EnterpriseVaultAdapter,
    /// Platform agent such as SSH agent, smartcard agent, or biometric agent.
    PlatformAgent,
    /// Hardware security module or cloud KMS.
    HsmOrKmsBacked,
    /// Process-local memory cache for visible session-only degraded fallback.
    SessionMemoryCache,
    /// Managed policy injector that materializes narrow ephemeral authority.
    ManagedPolicyInjector,
}

impl TrustStoreClass {
    /// Stable string token used in fixtures, support export, and shell rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsKeychain => "os_keychain",
            Self::EnterpriseVaultAdapter => "enterprise_vault_adapter",
            Self::PlatformAgent => "platform_agent",
            Self::HsmOrKmsBacked => "hsm_or_kms_backed",
            Self::SessionMemoryCache => "session_memory_cache",
            Self::ManagedPolicyInjector => "managed_policy_injector",
        }
    }
}

/// Unlock state reported by a trust-store interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnlockStateClass {
    /// Store is present but locked.
    Locked,
    /// Unlock or step-up is currently in progress.
    Unlocking,
    /// Store is available for admitted projections.
    Unlocked,
    /// Additional authenticator or approval is required for this operation.
    StepUpRequired,
    /// Broker is using visible process-local session memory only.
    DegradedSessionOnly,
    /// Store is unreachable, corrupted, or unsupported on this host.
    Unavailable,
}

impl UnlockStateClass {
    /// Stable string token used in fixtures, support export, and shell rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Locked => "locked",
            Self::Unlocking => "unlocking",
            Self::Unlocked => "unlocked",
            Self::StepUpRequired => "step_up_required",
            Self::DegradedSessionOnly => "degraded_session_only",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Projection mode admitted by the broker alpha.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectionModeClass {
    /// Alias only, with no raw material.
    AliasOnly,
    /// Broker resolves on callback and does not return raw material.
    BrokerCallback,
    /// Broker owns request construction when a protocol requires a header.
    RequestHeaderSigner,
    /// Broker lends an anonymous or short-lived file descriptor.
    EphemeralFd,
    /// Broker mounts a tmpfs-class, operation-bound secret view.
    BoundedMount,
    /// Broker injects into one isolated child process only.
    EnvVarIsolatedChild,
    /// Consumer sends data to sign; private bytes never leave the store.
    SignOnly,
    /// Consumer sends data to decrypt; private bytes never leave the store.
    DecryptOnly,
    /// Broker exchanges a source handle for a narrower operation token.
    TokenExchange,
    /// Managed policy materializes narrow authority per call.
    PolicyMaterialised,
    /// Diagnostics and support inspect metadata only.
    InspectMetadata,
    /// User-initiated, bounded reveal flow.
    RevealOnDemand,
}

impl ProjectionModeClass {
    /// Stable string token used in fixtures, support export, and shell rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AliasOnly => "alias_only",
            Self::BrokerCallback => "broker_callback",
            Self::RequestHeaderSigner => "request_header_signer",
            Self::EphemeralFd => "ephemeral_fd",
            Self::BoundedMount => "bounded_mount",
            Self::EnvVarIsolatedChild => "env_var_isolated_child",
            Self::SignOnly => "sign_only",
            Self::DecryptOnly => "decrypt_only",
            Self::TokenExchange => "token_exchange",
            Self::PolicyMaterialised => "policy_materialised",
            Self::InspectMetadata => "inspect_metadata",
            Self::RevealOnDemand => "reveal_on_demand",
        }
    }
}

/// Capability classes that can be paused by a broker continuity result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AffectedCapabilityClass {
    /// Package or container registry authentication.
    RegistryAuth,
    /// Managed sign-in refresh or session renewal.
    ManagedSignInRefresh,
    /// Provider reconnect, provider link refresh, or provider route renewal.
    ProviderReconnect,
    /// Database connection attach or credentialed query setup.
    DatabaseAttach,
    /// Remote tunnel or port-forward reuse.
    TunnelReuse,
}

impl AffectedCapabilityClass {
    /// Stable string token used in fixtures, support export, and shell rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RegistryAuth => "registry_auth",
            Self::ManagedSignInRefresh => "managed_sign_in_refresh",
            Self::ProviderReconnect => "provider_reconnect",
            Self::DatabaseAttach => "database_attach",
            Self::TunnelReuse => "tunnel_reuse",
        }
    }
}

/// Continuity state for a broker row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityStateClass {
    /// The credentialed capability is available through its declared mode.
    Ready,
    /// Session-only broker memory is active and visibly degraded.
    DegradedSessionOnlyVisible,
    /// Credentialed action is paused because the credential store is locked.
    PausedCredentialStoreLocked,
    /// Credentialed action is paused because the credential store is
    /// unavailable.
    PausedCredentialStoreUnavailable,
    /// Credentialed action is paused because trust-store or trust-state
    /// evidence changed after handle issuance.
    PausedTrustStoreChanged,
    /// Policy currently blocks the projection or credential source.
    PolicyBlocked,
    /// Handle, delegated credential, or session reference expired.
    Expired,
    /// Handle, delegated credential, or session reference was revoked.
    Revoked,
}

impl ContinuityStateClass {
    /// Stable string token used in fixtures, support export, and shell rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::DegradedSessionOnlyVisible => "degraded_session_only_visible",
            Self::PausedCredentialStoreLocked => "paused_credential_store_locked",
            Self::PausedCredentialStoreUnavailable => "paused_credential_store_unavailable",
            Self::PausedTrustStoreChanged => "paused_trust_store_changed",
            Self::PolicyBlocked => "policy_blocked",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
        }
    }

    /// True when the row must render recovery copy.
    pub const fn requires_visible_recovery(self) -> bool {
        !matches!(self, Self::Ready)
    }

    /// True when credentialed actions must stop until a safe recovery path
    /// succeeds.
    pub const fn pauses_credentialed_actions(self) -> bool {
        matches!(
            self,
            Self::PausedCredentialStoreLocked
                | Self::PausedCredentialStoreUnavailable
                | Self::PausedTrustStoreChanged
                | Self::PolicyBlocked
                | Self::Expired
                | Self::Revoked
        )
    }
}

/// Typed reason carried when a secret-broker row is denied or paused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBrokerDenialReason {
    /// Store is known but locked.
    TrustStoreLocked,
    /// Store is unreachable, corrupted, unsupported, or unavailable.
    TrustStoreUnavailable,
    /// Trust state or trust-store evidence changed after issuance.
    TrustStateDowngraded,
    /// No live runtime-authority ticket admits the projection.
    ApprovalTicketMissing,
    /// Policy forbids the projection for this class, consumer, or scope.
    PolicyDeniedProjection,
    /// Handle or delegated reference expired.
    SecretHandleExpired,
    /// Handle or delegated reference was revoked.
    SecretHandleRevoked,
    /// Session-only fallback is forbidden for this class.
    SessionFallbackClassForbidden,
    /// Plaintext persistence was requested and denied.
    PlaintextPersistenceDenied,
    /// Silent promotion to in-memory fallback was requested and denied.
    SilentInMemoryPromotionDenied,
    /// Stale approval-ticket or stale handle reuse was requested and denied.
    StaleTicketReuseDenied,
}

impl SecretBrokerDenialReason {
    /// Stable string token used in fixtures, support export, and shell rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustStoreLocked => "trust_store_locked",
            Self::TrustStoreUnavailable => "trust_store_unavailable",
            Self::TrustStateDowngraded => "trust_state_downgraded",
            Self::ApprovalTicketMissing => "approval_ticket_missing",
            Self::PolicyDeniedProjection => "policy_denied_projection",
            Self::SecretHandleExpired => "secret_handle_expired",
            Self::SecretHandleRevoked => "secret_handle_revoked",
            Self::SessionFallbackClassForbidden => "session_fallback_class_forbidden",
            Self::PlaintextPersistenceDenied => "plaintext_persistence_denied",
            Self::SilentInMemoryPromotionDenied => "silent_in_memory_promotion_denied",
            Self::StaleTicketReuseDenied => "stale_ticket_reuse_denied",
        }
    }
}

/// Safe actions that can recover from or continue around a broker result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryActionClass {
    /// Use the admitted handle or delegated/session reference.
    UseReference,
    /// Retry after the user or platform unlocks the credential store.
    RetryAfterCredentialStoreUnlock,
    /// Retry after the credential store recovers.
    RetryAfterCredentialStoreRecovery,
    /// Rebind the handle after trust-store or trust-state change.
    RebindAfterTrustStoreChange,
    /// Reauthenticate through the system browser.
    ReauthenticateInSystemBrowser,
    /// Continue local-only work that does not require this credential.
    ContinueLocalOnly,
    /// Export metadata-only support evidence.
    ExportSupportMetadata,
    /// Reconnect the provider through an admitted broker path.
    ReconnectProvider,
    /// Attach or reuse a tunnel with the delegated credential.
    AttachWithDelegatedCredential,
    /// Clear session-only broker memory and reprompt when needed.
    ClearSessionOnlyCredential,
}

impl RecoveryActionClass {
    /// Stable string token used in fixtures, support export, and shell rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UseReference => "use_reference",
            Self::RetryAfterCredentialStoreUnlock => "retry_after_credential_store_unlock",
            Self::RetryAfterCredentialStoreRecovery => "retry_after_credential_store_recovery",
            Self::RebindAfterTrustStoreChange => "rebind_after_trust_store_change",
            Self::ReauthenticateInSystemBrowser => "reauthenticate_in_system_browser",
            Self::ContinueLocalOnly => "continue_local_only",
            Self::ExportSupportMetadata => "export_support_metadata",
            Self::ReconnectProvider => "reconnect_provider",
            Self::AttachWithDelegatedCredential => "attach_with_delegated_credential",
            Self::ClearSessionOnlyCredential => "clear_session_only_credential",
        }
    }
}

/// Local-continuation class attached to a broker continuity result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalContinuationClass {
    /// Local editing, save, undo, search, local Git, and local tasks remain
    /// available.
    LocalOnlyFallbackAvailable,
    /// Cached or read-only local evidence remains available.
    CachedReadOnlyFallbackAvailable,
    /// The credentialed capability has no local substitute.
    NoLocalFallback,
}

impl LocalContinuationClass {
    /// Stable string token used in fixtures, support export, and shell rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyFallbackAvailable => "local_only_fallback_available",
            Self::CachedReadOnlyFallbackAvailable => "cached_read_only_fallback_available",
            Self::NoLocalFallback => "no_local_fallback",
        }
    }
}

/// Consumer identity that is allowed to use a broker row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretConsumerIdentity {
    /// Stable consumer id safe for logs and support export.
    pub consumer_id: String,
    /// Short human label safe for product UI and support export.
    pub consumer_label: String,
    /// Capability hash or manifest ref that binds the consumer.
    pub capability_hash_ref: String,
}

/// Secret reference block carried by a broker row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretReference {
    /// Whether the row uses handle, session-only, or delegated mode.
    pub reference_mode: SecretReferenceMode,
    /// Class of secret authority represented by the row.
    pub secret_class: SecretClass,
    /// User-stable broker alias safe for UI and metadata-only exports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority_alias_ref: Option<String>,
    /// Runtime credential handle ref. Support export elides the raw id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_handle_ref: Option<String>,
    /// Runtime session-only broker-memory ref. Support export elides the raw
    /// id and records only presence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_ref: Option<String>,
    /// Runtime delegated credential ref. Support export records only presence
    /// and never exports the delegated token body.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delegated_credential_ref: Option<String>,
    /// Approval ticket or authority ref that admits the projection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ticket_ref: Option<String>,
    /// Export-safe expiry timestamp for the reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Reviewable copy naming the revoke or clear path.
    pub revocation_path_label: String,
}

/// Storage and projection binding for a broker row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretStorageBinding {
    /// Controlled storage-mode vocabulary shared with credential-state rows.
    pub storage_mode: StorageModeClass,
    /// Store/source class shared with credential-state rows.
    pub store_source: StoreSourceClass,
    /// Broker trust-store class backing the row.
    pub trust_store_class: TrustStoreClass,
    /// Current unlock state for the trust-store interaction.
    pub unlock_state: UnlockStateClass,
    /// Projection mode admitted for this row.
    pub projection_mode: ProjectionModeClass,
    /// Whether plaintext persistence is permitted. Alpha rows must keep this
    /// false.
    pub plaintext_persistence_allowed: bool,
    /// Whether the broker may silently promote to in-memory fallback. Alpha
    /// rows must keep this false.
    pub silent_in_memory_promotion_allowed: bool,
    /// Whether stale approval tickets or handles may be reused. Alpha rows
    /// must keep this false.
    pub stale_ticket_reuse_allowed: bool,
    /// Whether raw secret material is present in the row. Alpha rows must keep
    /// this false.
    pub raw_secret_material_present: bool,
    /// Reviewable storage note that carries no raw material.
    pub storage_note: String,
}

/// First-class continuity result attached to a secret-broker row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretContinuityResult {
    /// Current continuity class for the row.
    pub continuity_state: ContinuityStateClass,
    /// Exact capability classes affected by this continuity result.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub affected_capabilities: Vec<AffectedCapabilityClass>,
    /// Local fallback posture when credentialed action is paused or degraded.
    pub local_continuation: LocalContinuationClass,
    /// Whether local non-credential work continues.
    pub local_work_continues: bool,
    /// Whether credentialed actions are paused by this result.
    pub credentialed_actions_paused: bool,
    /// Typed denial or pause reason when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denial_reason: Option<SecretBrokerDenialReason>,
    /// Safe retry, recover, or continue actions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recovery_actions: Vec<RecoveryActionClass>,
    /// Reviewable recovery copy safe for UI and support export.
    pub recovery_copy_label: String,
    /// Opaque ref to the metadata-only support export row.
    pub support_export_ref: String,
}

impl SecretContinuityResult {
    /// True when the row must render a visible recovery state.
    pub fn visible_recovery_required(&self) -> bool {
        self.continuity_state.requires_visible_recovery()
    }
}

/// Redaction and export posture for a secret-broker row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretExportPosture {
    /// Redaction class applied by support/export surfaces.
    pub redaction_class: String,
    /// Support-export posture, normally `metadata_only`.
    pub support_export_posture: String,
    /// Portable profile export posture, normally alias-only or excluded.
    pub profile_export_posture: String,
    /// Always false for alpha rows.
    pub raw_secret_values_exported: bool,
    /// Always false for support exports; raw handle ids are elided.
    pub raw_handle_ids_exported: bool,
    /// Whether alias refs may travel in metadata-only export.
    pub alias_refs_exportable: bool,
    /// Material classes omitted from export.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub omitted_material_classes: Vec<String>,
}

/// Canonical secret-broker alpha row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretBrokerAlphaRow {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Alpha schema version.
    pub schema_version: u32,
    /// Stable row id safe for UI, logs, and support export.
    pub secret_broker_row_id: String,
    /// Reviewable label safe for UI and support export.
    pub display_label: String,
    /// True when this row is claiming the alpha broker contract.
    pub claimed_alpha_row: bool,
    /// Primary capability class represented by this row.
    pub capability_class: AffectedCapabilityClass,
    /// Identity mode envelope for the row.
    pub identity_mode: IdentityModeAlias,
    /// Workspace trust state envelope for the row.
    pub trust_state: TrustState,
    /// Consumer permitted to resolve or use the reference.
    pub consumer: SecretConsumerIdentity,
    /// Opaque target ref for the provider, registry, database, or remote
    /// target.
    pub target_ref: String,
    /// Opaque workspace or worktree scope ref.
    pub workspace_scope_ref: String,
    /// Secret reference block.
    pub secret_ref: SecretReference,
    /// Storage and projection binding.
    pub storage: SecretStorageBinding,
    /// First-class continuity result.
    pub continuity: SecretContinuityResult,
    /// Redaction and export posture.
    pub export: SecretExportPosture,
    /// Export-safe timestamp for row creation.
    pub minted_at: String,
}

impl SecretBrokerAlphaRow {
    /// Validate the row against the alpha guardrails.
    pub fn validate(&self) -> Result<(), SecretBrokerRowError> {
        if self.record_kind != SECRET_BROKER_ROW_RECORD_KIND {
            return Err(SecretBrokerRowError::InvalidRecordKind {
                row_id: self.secret_broker_row_id.clone(),
                actual: self.record_kind.clone(),
            });
        }
        if self.schema_version != SECRET_BROKER_ALPHA_SCHEMA_VERSION {
            return Err(SecretBrokerRowError::InvalidSchemaVersion {
                row_id: self.secret_broker_row_id.clone(),
                actual: self.schema_version,
            });
        }
        if !self.claimed_alpha_row {
            return Err(SecretBrokerRowError::UnclaimedAlphaRow {
                row_id: self.secret_broker_row_id.clone(),
            });
        }
        if self.storage.raw_secret_material_present || self.export.raw_secret_values_exported {
            return Err(SecretBrokerRowError::RawSecretMaterialPresent {
                row_id: self.secret_broker_row_id.clone(),
            });
        }
        if self.storage.plaintext_persistence_allowed {
            return Err(SecretBrokerRowError::PlaintextPersistenceAllowed {
                row_id: self.secret_broker_row_id.clone(),
            });
        }
        if self.storage.silent_in_memory_promotion_allowed {
            return Err(SecretBrokerRowError::SilentInMemoryPromotionAllowed {
                row_id: self.secret_broker_row_id.clone(),
            });
        }
        if self.storage.stale_ticket_reuse_allowed {
            return Err(SecretBrokerRowError::StaleTicketReuseAllowed {
                row_id: self.secret_broker_row_id.clone(),
            });
        }
        match self.secret_ref.reference_mode {
            SecretReferenceMode::Handle if self.secret_ref.credential_handle_ref.is_none() => {
                return Err(SecretBrokerRowError::MissingHandleRef {
                    row_id: self.secret_broker_row_id.clone(),
                });
            }
            SecretReferenceMode::SessionOnly if self.secret_ref.session_ref.is_none() => {
                return Err(SecretBrokerRowError::MissingSessionRef {
                    row_id: self.secret_broker_row_id.clone(),
                });
            }
            SecretReferenceMode::SessionOnly
                if self.storage.storage_mode != StorageModeClass::SessionOnly
                    || self.storage.trust_store_class != TrustStoreClass::SessionMemoryCache =>
            {
                return Err(SecretBrokerRowError::SessionOnlyStorageMismatch {
                    row_id: self.secret_broker_row_id.clone(),
                });
            }
            SecretReferenceMode::Delegated
                if self.secret_ref.delegated_credential_ref.is_none() =>
            {
                return Err(SecretBrokerRowError::MissingDelegatedCredentialRef {
                    row_id: self.secret_broker_row_id.clone(),
                });
            }
            SecretReferenceMode::Delegated
                if self.storage.storage_mode != StorageModeClass::Delegated =>
            {
                return Err(SecretBrokerRowError::DelegatedStorageMismatch {
                    row_id: self.secret_broker_row_id.clone(),
                });
            }
            _ => {}
        }
        if !self
            .continuity
            .affected_capabilities
            .contains(&self.capability_class)
        {
            return Err(SecretBrokerRowError::MissingAffectedCapability {
                row_id: self.secret_broker_row_id.clone(),
            });
        }
        if !self.continuity.local_work_continues {
            return Err(SecretBrokerRowError::LocalContinuationMissing {
                row_id: self.secret_broker_row_id.clone(),
            });
        }
        if self.continuity.visible_recovery_required()
            && self.continuity.recovery_actions.is_empty()
        {
            return Err(SecretBrokerRowError::MissingRecoveryAction {
                row_id: self.secret_broker_row_id.clone(),
            });
        }
        if self
            .continuity
            .continuity_state
            .pauses_credentialed_actions()
            && self.continuity.denial_reason.is_none()
        {
            return Err(SecretBrokerRowError::MissingDenialReason {
                row_id: self.secret_broker_row_id.clone(),
            });
        }
        match self.continuity.continuity_state {
            ContinuityStateClass::PausedCredentialStoreLocked
                if self.storage.unlock_state != UnlockStateClass::Locked =>
            {
                return Err(SecretBrokerRowError::LockedStateMismatch {
                    row_id: self.secret_broker_row_id.clone(),
                });
            }
            ContinuityStateClass::PausedCredentialStoreUnavailable
                if self.storage.unlock_state != UnlockStateClass::Unavailable =>
            {
                return Err(SecretBrokerRowError::UnavailableStateMismatch {
                    row_id: self.secret_broker_row_id.clone(),
                });
            }
            ContinuityStateClass::PausedTrustStoreChanged
                if self.continuity.denial_reason
                    != Some(SecretBrokerDenialReason::TrustStateDowngraded) =>
            {
                return Err(SecretBrokerRowError::TrustStoreChangedWithoutRebind {
                    row_id: self.secret_broker_row_id.clone(),
                });
            }
            _ => {}
        }
        Ok(())
    }

    /// Project the row to the surface shape that UI, CLI, or status rows can
    /// render.
    pub fn surface_row(&self) -> SecretBrokerSurfaceRow {
        SecretBrokerSurfaceRow::from_row(self)
    }

    /// Project the row to metadata-only support-export evidence.
    pub fn support_export_row(&self) -> SecretBrokerSupportExportRow {
        SecretBrokerSupportExportRow::from_row(self)
    }
}

/// Projection that a UI, CLI, or status surface can render without deriving
/// broker truth locally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretBrokerSurfaceRow {
    /// Stable row id safe for UI, logs, and support export.
    pub secret_broker_row_id: String,
    /// Reviewable label safe for UI and support export.
    pub display_label: String,
    /// Capability class represented by this row.
    pub capability_class: AffectedCapabilityClass,
    /// Stable capability token.
    pub capability_class_token: String,
    /// Reference mode represented by this row.
    pub reference_mode: SecretReferenceMode,
    /// Stable reference-mode token.
    pub reference_mode_token: String,
    /// Secret class represented by this row.
    pub secret_class: SecretClass,
    /// Stable secret-class token.
    pub secret_class_token: String,
    /// Consumer label safe for UI.
    pub consumer_label: String,
    /// Opaque target ref.
    pub target_ref: String,
    /// Opaque workspace scope ref.
    pub workspace_scope_ref: String,
    /// Storage-mode token shared with credential-state rows.
    pub storage_mode_token: String,
    /// Trust-store token backing the row.
    pub trust_store_class_token: String,
    /// Unlock-state token backing the row.
    pub unlock_state_token: String,
    /// Projection-mode token backing the row.
    pub projection_mode_token: String,
    /// Continuity-state token backing the row.
    pub continuity_state_token: String,
    /// Exact affected capability tokens.
    pub affected_capability_tokens: Vec<String>,
    /// Whether local non-credential work continues.
    pub local_work_continues: bool,
    /// Whether the row needs recovery copy.
    pub visible_recovery_required: bool,
    /// Whether credentialed actions are paused.
    pub credentialed_actions_paused: bool,
    /// Safe recovery action tokens.
    pub recovery_action_tokens: Vec<String>,
    /// Reviewable recovery copy.
    pub recovery_copy_label: String,
    /// Whether plaintext persistence is allowed.
    pub plaintext_persistence_allowed: bool,
    /// Whether silent in-memory promotion is allowed.
    pub silent_in_memory_promotion_allowed: bool,
    /// Whether stale ticket reuse is allowed.
    pub stale_ticket_reuse_allowed: bool,
    /// Whether raw secret material appears in the row.
    pub raw_secret_material_present: bool,
    /// Opaque support-export row ref.
    pub support_export_ref: String,
}

impl SecretBrokerSurfaceRow {
    /// Project a surface row from a [`SecretBrokerAlphaRow`].
    pub fn from_row(row: &SecretBrokerAlphaRow) -> Self {
        Self {
            secret_broker_row_id: row.secret_broker_row_id.clone(),
            display_label: row.display_label.clone(),
            capability_class: row.capability_class,
            capability_class_token: row.capability_class.as_str().to_owned(),
            reference_mode: row.secret_ref.reference_mode,
            reference_mode_token: row.secret_ref.reference_mode.as_str().to_owned(),
            secret_class: row.secret_ref.secret_class,
            secret_class_token: row.secret_ref.secret_class.as_str().to_owned(),
            consumer_label: row.consumer.consumer_label.clone(),
            target_ref: row.target_ref.clone(),
            workspace_scope_ref: row.workspace_scope_ref.clone(),
            storage_mode_token: row.storage.storage_mode.as_str().to_owned(),
            trust_store_class_token: row.storage.trust_store_class.as_str().to_owned(),
            unlock_state_token: row.storage.unlock_state.as_str().to_owned(),
            projection_mode_token: row.storage.projection_mode.as_str().to_owned(),
            continuity_state_token: row.continuity.continuity_state.as_str().to_owned(),
            affected_capability_tokens: row
                .continuity
                .affected_capabilities
                .iter()
                .map(|class| class.as_str().to_owned())
                .collect(),
            local_work_continues: row.continuity.local_work_continues,
            visible_recovery_required: row.continuity.visible_recovery_required(),
            credentialed_actions_paused: row.continuity.credentialed_actions_paused,
            recovery_action_tokens: row
                .continuity
                .recovery_actions
                .iter()
                .map(|action| action.as_str().to_owned())
                .collect(),
            recovery_copy_label: row.continuity.recovery_copy_label.clone(),
            plaintext_persistence_allowed: row.storage.plaintext_persistence_allowed,
            silent_in_memory_promotion_allowed: row.storage.silent_in_memory_promotion_allowed,
            stale_ticket_reuse_allowed: row.storage.stale_ticket_reuse_allowed,
            raw_secret_material_present: row.storage.raw_secret_material_present,
            support_export_ref: row.continuity.support_export_ref.clone(),
        }
    }
}

/// Metadata-only support-export row for one broker alpha row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretBrokerSupportExportRow {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Alpha schema version.
    pub schema_version: u32,
    /// Stable export row id.
    pub export_row_id: String,
    /// Source broker row id.
    pub secret_broker_row_ref: String,
    /// Reviewable label safe for support export.
    pub display_label: String,
    /// Capability class represented by this row.
    pub capability_class: AffectedCapabilityClass,
    /// Stable capability token.
    pub capability_class_token: String,
    /// Reference mode represented by this row.
    pub reference_mode: SecretReferenceMode,
    /// Stable reference-mode token.
    pub reference_mode_token: String,
    /// Secret class represented by this row.
    pub secret_class: SecretClass,
    /// Stable secret-class token.
    pub secret_class_token: String,
    /// Consumer id safe for support export.
    pub consumer_id: String,
    /// Opaque target ref safe for support export.
    pub target_ref: String,
    /// Opaque workspace scope ref safe for support export.
    pub workspace_scope_ref: String,
    /// Alias ref when policy allows aliases in export.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority_alias_ref: Option<String>,
    /// True when the source row had a runtime handle ref.
    pub credential_handle_ref_present: bool,
    /// True when the source row had a runtime session ref.
    pub session_ref_present: bool,
    /// True when the source row had a runtime delegated credential ref.
    pub delegated_credential_ref_present: bool,
    /// True when an approval ticket ref exists.
    pub approval_ticket_ref_present: bool,
    /// Storage-mode token shared with credential-state rows.
    pub storage_mode_token: String,
    /// Store-source token shared with credential-state rows.
    pub store_source_token: String,
    /// Trust-store token backing the row.
    pub trust_store_class_token: String,
    /// Unlock-state token backing the row.
    pub unlock_state_token: String,
    /// Projection-mode token backing the row.
    pub projection_mode_token: String,
    /// Continuity-state token backing the row.
    pub continuity_state_token: String,
    /// Exact affected capability tokens.
    pub affected_capability_tokens: Vec<String>,
    /// Typed denial token when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denial_reason_token: Option<String>,
    /// Local-continuation token.
    pub local_continuation_token: String,
    /// Whether local non-credential work continues.
    pub local_work_continues: bool,
    /// Whether credentialed actions are paused.
    pub credentialed_actions_paused: bool,
    /// Safe recovery action tokens.
    pub recovery_action_tokens: Vec<String>,
    /// Reviewable recovery copy safe for support export.
    pub recovery_copy_label: String,
    /// Redaction class applied by support/export.
    pub redaction_class: String,
    /// Always false: raw secret values are not exported.
    pub raw_secret_values_exported: bool,
    /// Always false: runtime handle ids are not exported.
    pub raw_handle_ids_exported: bool,
    /// Material classes omitted from export.
    pub omitted_material_classes: Vec<String>,
}

impl SecretBrokerSupportExportRow {
    /// Project metadata-only support evidence from a broker row.
    pub fn from_row(row: &SecretBrokerAlphaRow) -> Self {
        Self {
            record_kind: SECRET_BROKER_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            schema_version: SECRET_BROKER_ALPHA_SCHEMA_VERSION,
            export_row_id: row.continuity.support_export_ref.clone(),
            secret_broker_row_ref: row.secret_broker_row_id.clone(),
            display_label: row.display_label.clone(),
            capability_class: row.capability_class,
            capability_class_token: row.capability_class.as_str().to_owned(),
            reference_mode: row.secret_ref.reference_mode,
            reference_mode_token: row.secret_ref.reference_mode.as_str().to_owned(),
            secret_class: row.secret_ref.secret_class,
            secret_class_token: row.secret_ref.secret_class.as_str().to_owned(),
            consumer_id: row.consumer.consumer_id.clone(),
            target_ref: row.target_ref.clone(),
            workspace_scope_ref: row.workspace_scope_ref.clone(),
            authority_alias_ref: if row.export.alias_refs_exportable {
                row.secret_ref.authority_alias_ref.clone()
            } else {
                None
            },
            credential_handle_ref_present: row.secret_ref.credential_handle_ref.is_some(),
            session_ref_present: row.secret_ref.session_ref.is_some(),
            delegated_credential_ref_present: row.secret_ref.delegated_credential_ref.is_some(),
            approval_ticket_ref_present: row.secret_ref.approval_ticket_ref.is_some(),
            storage_mode_token: row.storage.storage_mode.as_str().to_owned(),
            store_source_token: row.storage.store_source.as_str().to_owned(),
            trust_store_class_token: row.storage.trust_store_class.as_str().to_owned(),
            unlock_state_token: row.storage.unlock_state.as_str().to_owned(),
            projection_mode_token: row.storage.projection_mode.as_str().to_owned(),
            continuity_state_token: row.continuity.continuity_state.as_str().to_owned(),
            affected_capability_tokens: row
                .continuity
                .affected_capabilities
                .iter()
                .map(|class| class.as_str().to_owned())
                .collect(),
            denial_reason_token: row
                .continuity
                .denial_reason
                .map(|reason| reason.as_str().to_owned()),
            local_continuation_token: row.continuity.local_continuation.as_str().to_owned(),
            local_work_continues: row.continuity.local_work_continues,
            credentialed_actions_paused: row.continuity.credentialed_actions_paused,
            recovery_action_tokens: row
                .continuity
                .recovery_actions
                .iter()
                .map(|action| action.as_str().to_owned())
                .collect(),
            recovery_copy_label: row.continuity.recovery_copy_label.clone(),
            redaction_class: row.export.redaction_class.clone(),
            raw_secret_values_exported: false,
            raw_handle_ids_exported: false,
            omitted_material_classes: row.export.omitted_material_classes.clone(),
        }
    }
}

/// Metadata-only support/export projection for a broker packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretBrokerSupportExport {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Alpha schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export-safe generation timestamp.
    pub generated_at: String,
    /// Metadata-only broker rows.
    pub rows: Vec<SecretBrokerSupportExportRow>,
    /// Reviewable summary of the redaction decision.
    pub redaction_summary: String,
    /// Always false: raw secret values are not exported.
    pub raw_secret_values_exported: bool,
    /// Always false: runtime handle ids are not exported.
    pub raw_handle_ids_exported: bool,
}

impl SecretBrokerSupportExport {
    /// True when the export confirms that no raw secret values or raw handle
    /// ids are present.
    pub fn redaction_safe(&self) -> bool {
        !self.raw_secret_values_exported
            && !self.raw_handle_ids_exported
            && self
                .rows
                .iter()
                .all(|row| !row.raw_secret_values_exported && !row.raw_handle_ids_exported)
    }
}

/// Packet containing claimed secret-broker alpha rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretBrokerAlphaPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Alpha schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed secret-broker alpha rows.
    pub rows: Vec<SecretBrokerAlphaRow>,
    /// Export-safe packet creation timestamp.
    pub minted_at: String,
}

impl SecretBrokerAlphaPacket {
    /// Mint an empty packet.
    pub fn new(packet_id: impl Into<String>, minted_at: impl Into<String>) -> Self {
        Self {
            record_kind: SECRET_BROKER_ALPHA_PACKET_RECORD_KIND.to_owned(),
            schema_version: SECRET_BROKER_ALPHA_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            rows: Vec::new(),
            minted_at: minted_at.into(),
        }
    }

    /// Append a broker row.
    pub fn add_row(&mut self, row: SecretBrokerAlphaRow) {
        self.rows.push(row);
    }

    /// Validate the packet and all contained rows.
    pub fn validate(&self) -> Result<(), SecretBrokerPacketError> {
        if self.record_kind != SECRET_BROKER_ALPHA_PACKET_RECORD_KIND {
            return Err(SecretBrokerPacketError::InvalidRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        if self.schema_version != SECRET_BROKER_ALPHA_SCHEMA_VERSION {
            return Err(SecretBrokerPacketError::InvalidSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.rows.is_empty() {
            return Err(SecretBrokerPacketError::EmptyPacket);
        }
        for row in &self.rows {
            row.validate().map_err(SecretBrokerPacketError::Row)?;
        }
        Ok(())
    }

    /// Project UI/CLI/status surface rows.
    pub fn surface_rows(&self) -> Vec<SecretBrokerSurfaceRow> {
        self.rows
            .iter()
            .map(SecretBrokerSurfaceRow::from_row)
            .collect()
    }

    /// Project a metadata-only support/export packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> SecretBrokerSupportExport {
        SecretBrokerSupportExport {
            record_kind: SECRET_BROKER_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SECRET_BROKER_ALPHA_SCHEMA_VERSION,
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            rows: self
                .rows
                .iter()
                .map(SecretBrokerSupportExportRow::from_row)
                .collect(),
            redaction_summary: "Metadata-only secret-broker export: class, alias posture, store \
                                state, continuity result, and recovery actions are included; raw \
                                secret values and raw handle ids are excluded."
                .to_owned(),
            raw_secret_values_exported: false,
            raw_handle_ids_exported: false,
        }
    }

    /// Pause matching rows because a credential store locked.
    pub fn mark_credential_store_locked(
        &mut self,
        trust_store_class: TrustStoreClass,
        capability_filter: &[AffectedCapabilityClass],
    ) -> usize {
        let mut affected = 0;
        for row in &mut self.rows {
            if row.storage.trust_store_class == trust_store_class
                && capability_matches(row.capability_class, capability_filter)
            {
                row.storage.unlock_state = UnlockStateClass::Locked;
                row.storage.plaintext_persistence_allowed = false;
                row.storage.silent_in_memory_promotion_allowed = false;
                row.storage.stale_ticket_reuse_allowed = false;
                row.storage.raw_secret_material_present = false;
                row.continuity.continuity_state = ContinuityStateClass::PausedCredentialStoreLocked;
                row.continuity.affected_capabilities = vec![row.capability_class];
                row.continuity.local_continuation =
                    LocalContinuationClass::LocalOnlyFallbackAvailable;
                row.continuity.local_work_continues = true;
                row.continuity.credentialed_actions_paused = true;
                row.continuity.denial_reason = Some(SecretBrokerDenialReason::TrustStoreLocked);
                row.continuity.recovery_actions = vec![
                    RecoveryActionClass::RetryAfterCredentialStoreUnlock,
                    RecoveryActionClass::ContinueLocalOnly,
                    RecoveryActionClass::ExportSupportMetadata,
                ];
                row.continuity.recovery_copy_label = format!(
                    "{} is paused because {} is locked. Unlock the store to retry; local work \
                     continues without credentialed actions.",
                    row.display_label,
                    trust_store_display(trust_store_class),
                );
                affected += 1;
            }
        }
        affected
    }

    /// Pause matching rows because a credential store became unavailable.
    pub fn mark_credential_store_unavailable(
        &mut self,
        trust_store_class: TrustStoreClass,
        capability_filter: &[AffectedCapabilityClass],
    ) -> usize {
        let mut affected = 0;
        for row in &mut self.rows {
            if row.storage.trust_store_class == trust_store_class
                && capability_matches(row.capability_class, capability_filter)
            {
                row.storage.unlock_state = UnlockStateClass::Unavailable;
                row.storage.plaintext_persistence_allowed = false;
                row.storage.silent_in_memory_promotion_allowed = false;
                row.storage.stale_ticket_reuse_allowed = false;
                row.storage.raw_secret_material_present = false;
                row.continuity.continuity_state =
                    ContinuityStateClass::PausedCredentialStoreUnavailable;
                row.continuity.affected_capabilities = vec![row.capability_class];
                row.continuity.local_continuation =
                    LocalContinuationClass::LocalOnlyFallbackAvailable;
                row.continuity.local_work_continues = true;
                row.continuity.credentialed_actions_paused = true;
                row.continuity.denial_reason =
                    Some(SecretBrokerDenialReason::TrustStoreUnavailable);
                row.continuity.recovery_actions = vec![
                    RecoveryActionClass::RetryAfterCredentialStoreRecovery,
                    RecoveryActionClass::ContinueLocalOnly,
                    RecoveryActionClass::ExportSupportMetadata,
                ];
                row.continuity.recovery_copy_label = format!(
                    "{} is paused because {} is unavailable. Recover the store or reconfigure \
                     the source; local work continues without credentialed actions.",
                    row.display_label,
                    trust_store_display(trust_store_class),
                );
                affected += 1;
            }
        }
        affected
    }

    /// Pause matching rows because trust-store or trust-state evidence changed
    /// after issuance.
    pub fn mark_trust_store_changed(
        &mut self,
        trust_store_class: TrustStoreClass,
        capability_filter: &[AffectedCapabilityClass],
    ) -> usize {
        let mut affected = 0;
        for row in &mut self.rows {
            if row.storage.trust_store_class == trust_store_class
                && capability_matches(row.capability_class, capability_filter)
            {
                row.trust_state = TrustState::Restricted;
                row.storage.unlock_state = UnlockStateClass::StepUpRequired;
                row.storage.plaintext_persistence_allowed = false;
                row.storage.silent_in_memory_promotion_allowed = false;
                row.storage.stale_ticket_reuse_allowed = false;
                row.storage.raw_secret_material_present = false;
                row.continuity.continuity_state = ContinuityStateClass::PausedTrustStoreChanged;
                row.continuity.affected_capabilities = vec![row.capability_class];
                row.continuity.local_continuation =
                    LocalContinuationClass::CachedReadOnlyFallbackAvailable;
                row.continuity.local_work_continues = true;
                row.continuity.credentialed_actions_paused = true;
                row.continuity.denial_reason = Some(SecretBrokerDenialReason::TrustStateDowngraded);
                row.continuity.recovery_actions = vec![
                    RecoveryActionClass::RebindAfterTrustStoreChange,
                    RecoveryActionClass::ContinueLocalOnly,
                    RecoveryActionClass::ExportSupportMetadata,
                ];
                row.continuity.recovery_copy_label = format!(
                    "{} is paused because {} trust evidence changed. Rebind the handle before \
                     reuse; local cached work remains available.",
                    row.display_label,
                    trust_store_display(trust_store_class),
                );
                affected += 1;
            }
        }
        affected
    }

    /// True when any row pauses or degrades a credentialed capability.
    pub fn has_visible_continuity_result(&self) -> bool {
        self.rows
            .iter()
            .any(|row| row.continuity.visible_recovery_required())
    }
}

fn capability_matches(
    capability_class: AffectedCapabilityClass,
    filter: &[AffectedCapabilityClass],
) -> bool {
    filter.is_empty() || filter.contains(&capability_class)
}

const fn trust_store_display(class: TrustStoreClass) -> &'static str {
    match class {
        TrustStoreClass::OsKeychain => "OS credential store",
        TrustStoreClass::EnterpriseVaultAdapter => "enterprise vault",
        TrustStoreClass::PlatformAgent => "platform agent",
        TrustStoreClass::HsmOrKmsBacked => "HSM or KMS",
        TrustStoreClass::SessionMemoryCache => "session memory cache",
        TrustStoreClass::ManagedPolicyInjector => "managed policy injector",
    }
}

/// Validation error for one secret-broker alpha row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecretBrokerRowError {
    /// The row carried an unexpected record kind.
    InvalidRecordKind { row_id: String, actual: String },
    /// The row carried an unexpected schema version.
    InvalidSchemaVersion { row_id: String, actual: u32 },
    /// The row did not claim the alpha contract.
    UnclaimedAlphaRow { row_id: String },
    /// A handle-mode row omitted the runtime handle ref.
    MissingHandleRef { row_id: String },
    /// A session-only row omitted the runtime session ref.
    MissingSessionRef { row_id: String },
    /// A delegated row omitted the delegated credential ref.
    MissingDelegatedCredentialRef { row_id: String },
    /// A session-only row did not use session-only storage.
    SessionOnlyStorageMismatch { row_id: String },
    /// A delegated row did not use delegated storage.
    DelegatedStorageMismatch { row_id: String },
    /// The row or export claimed raw material was present.
    RawSecretMaterialPresent { row_id: String },
    /// The row allowed plaintext persistence.
    PlaintextPersistenceAllowed { row_id: String },
    /// The row allowed silent in-memory promotion.
    SilentInMemoryPromotionAllowed { row_id: String },
    /// The row allowed stale ticket or stale handle reuse.
    StaleTicketReuseAllowed { row_id: String },
    /// The continuity result omitted the row's capability class.
    MissingAffectedCapability { row_id: String },
    /// The continuity result omitted local work continuation.
    LocalContinuationMissing { row_id: String },
    /// A recovery-requiring result omitted recovery actions.
    MissingRecoveryAction { row_id: String },
    /// A paused credentialed result omitted a typed denial reason.
    MissingDenialReason { row_id: String },
    /// A locked continuity result did not carry locked unlock state.
    LockedStateMismatch { row_id: String },
    /// An unavailable continuity result did not carry unavailable state.
    UnavailableStateMismatch { row_id: String },
    /// A trust-store-changed result did not carry rebind denial posture.
    TrustStoreChangedWithoutRebind { row_id: String },
}

impl std::fmt::Display for SecretBrokerRowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRecordKind { row_id, actual } => {
                write!(f, "secret-broker row {row_id} has invalid record kind {actual}")
            }
            Self::InvalidSchemaVersion { row_id, actual } => {
                write!(f, "secret-broker row {row_id} has invalid schema version {actual}")
            }
            Self::UnclaimedAlphaRow { row_id } => {
                write!(f, "secret-broker row {row_id} did not claim the alpha contract")
            }
            Self::MissingHandleRef { row_id } => {
                write!(f, "secret-broker row {row_id} is handle-mode without a handle ref")
            }
            Self::MissingSessionRef { row_id } => {
                write!(f, "secret-broker row {row_id} is session-only without a session ref")
            }
            Self::MissingDelegatedCredentialRef { row_id } => write!(
                f,
                "secret-broker row {row_id} is delegated without a delegated credential ref"
            ),
            Self::SessionOnlyStorageMismatch { row_id } => write!(
                f,
                "secret-broker row {row_id} uses session-only mode without session-memory storage"
            ),
            Self::DelegatedStorageMismatch { row_id } => write!(
                f,
                "secret-broker row {row_id} uses delegated mode without delegated storage"
            ),
            Self::RawSecretMaterialPresent { row_id } => {
                write!(f, "secret-broker row {row_id} carried raw secret material")
            }
            Self::PlaintextPersistenceAllowed { row_id } => {
                write!(f, "secret-broker row {row_id} allowed plaintext persistence")
            }
            Self::SilentInMemoryPromotionAllowed { row_id } => write!(
                f,
                "secret-broker row {row_id} allowed silent in-memory promotion"
            ),
            Self::StaleTicketReuseAllowed { row_id } => {
                write!(f, "secret-broker row {row_id} allowed stale ticket reuse")
            }
            Self::MissingAffectedCapability { row_id } => write!(
                f,
                "secret-broker row {row_id} omitted its affected capability class"
            ),
            Self::LocalContinuationMissing { row_id } => {
                write!(f, "secret-broker row {row_id} does not preserve local work")
            }
            Self::MissingRecoveryAction { row_id } => {
                write!(f, "secret-broker row {row_id} omitted recovery actions")
            }
            Self::MissingDenialReason { row_id } => {
                write!(f, "secret-broker row {row_id} omitted a typed denial reason")
            }
            Self::LockedStateMismatch { row_id } => {
                write!(f, "secret-broker row {row_id} has locked continuity without locked state")
            }
            Self::UnavailableStateMismatch { row_id } => write!(
                f,
                "secret-broker row {row_id} has unavailable continuity without unavailable state"
            ),
            Self::TrustStoreChangedWithoutRebind { row_id } => write!(
                f,
                "secret-broker row {row_id} has trust-store-changed continuity without rebind posture"
            ),
        }
    }
}

impl std::error::Error for SecretBrokerRowError {}

/// Validation error for a secret-broker alpha packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecretBrokerPacketError {
    /// The packet carried an unexpected record kind.
    InvalidRecordKind { actual: String },
    /// The packet carried an unexpected schema version.
    InvalidSchemaVersion { actual: u32 },
    /// The packet had no rows.
    EmptyPacket,
    /// One row failed validation.
    Row(SecretBrokerRowError),
}

impl std::fmt::Display for SecretBrokerPacketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRecordKind { actual } => {
                write!(f, "secret-broker packet has invalid record kind {actual}")
            }
            Self::InvalidSchemaVersion { actual } => {
                write!(
                    f,
                    "secret-broker packet has invalid schema version {actual}"
                )
            }
            Self::EmptyPacket => write!(f, "secret-broker packet must contain at least one row"),
            Self::Row(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for SecretBrokerPacketError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn export_posture() -> SecretExportPosture {
        SecretExportPosture {
            redaction_class: "metadata_safe_default".to_owned(),
            support_export_posture: "metadata_only".to_owned(),
            profile_export_posture: "alias_only".to_owned(),
            raw_secret_values_exported: false,
            raw_handle_ids_exported: false,
            alias_refs_exportable: true,
            omitted_material_classes: vec![
                "raw_secret_material".to_owned(),
                "raw_handle_id".to_owned(),
                "delegated_token_body".to_owned(),
            ],
        }
    }

    fn handle_row() -> SecretBrokerAlphaRow {
        SecretBrokerAlphaRow {
            record_kind: SECRET_BROKER_ROW_RECORD_KIND.to_owned(),
            schema_version: SECRET_BROKER_ALPHA_SCHEMA_VERSION,
            secret_broker_row_id: "secret_broker.registry_auth.handle.unit".to_owned(),
            display_label: "Registry auth credential-store handle".to_owned(),
            claimed_alpha_row: true,
            capability_class: AffectedCapabilityClass::RegistryAuth,
            identity_mode: IdentityModeAlias::AccountFreeLocal,
            trust_state: TrustState::Trusted,
            consumer: SecretConsumerIdentity {
                consumer_id: "consumer.package.registry".to_owned(),
                consumer_label: "Package registry client".to_owned(),
                capability_hash_ref: "capability-hash:registry-client".to_owned(),
            },
            target_ref: "registry:crates-io".to_owned(),
            workspace_scope_ref: "workspace:local".to_owned(),
            secret_ref: SecretReference {
                reference_mode: SecretReferenceMode::Handle,
                secret_class: SecretClass::PackageRegistryToken,
                authority_alias_ref: Some("secret-alias:registry:crates-io".to_owned()),
                credential_handle_ref: Some("credential-handle:registry:crates-io".to_owned()),
                session_ref: None,
                delegated_credential_ref: None,
                approval_ticket_ref: Some("approval-ticket:registry-auth".to_owned()),
                expires_at: None,
                revocation_path_label: "Remove registry credential".to_owned(),
            },
            storage: SecretStorageBinding {
                storage_mode: StorageModeClass::SystemCredentialStore,
                store_source: StoreSourceClass::OsKeychain,
                trust_store_class: TrustStoreClass::OsKeychain,
                unlock_state: UnlockStateClass::Unlocked,
                projection_mode: ProjectionModeClass::BrokerCallback,
                plaintext_persistence_allowed: false,
                silent_in_memory_promotion_allowed: false,
                stale_ticket_reuse_allowed: false,
                raw_secret_material_present: false,
                storage_note: "OS credential store holds the registry alias.".to_owned(),
            },
            continuity: SecretContinuityResult {
                continuity_state: ContinuityStateClass::Ready,
                affected_capabilities: vec![AffectedCapabilityClass::RegistryAuth],
                local_continuation: LocalContinuationClass::LocalOnlyFallbackAvailable,
                local_work_continues: true,
                credentialed_actions_paused: false,
                denial_reason: None,
                recovery_actions: vec![
                    RecoveryActionClass::UseReference,
                    RecoveryActionClass::ContinueLocalOnly,
                ],
                recovery_copy_label: "Registry auth can resolve through the broker handle."
                    .to_owned(),
                support_export_ref: "secret-export-row:registry-auth".to_owned(),
            },
            export: export_posture(),
            minted_at: "2026-05-13T23:55:00Z".to_owned(),
        }
    }

    #[test]
    fn handle_row_validates_and_support_export_elides_raw_handle_id() {
        let row = handle_row();
        row.validate().expect("row validates");
        let export = row.support_export_row();
        assert!(export.credential_handle_ref_present);
        assert!(!export.raw_secret_values_exported);
        assert!(!export.raw_handle_ids_exported);
        let encoded = serde_json::to_string(&export).expect("serialize support export row");
        assert!(!encoded.contains("credential-handle:registry:crates-io"));
    }

    #[test]
    fn packet_helpers_turn_store_lock_into_paused_continuity_result() {
        let mut packet =
            SecretBrokerAlphaPacket::new("secret_broker_alpha_packet.unit", "2026-05-13T23:55:00Z");
        packet.add_row(handle_row());

        let affected = packet.mark_credential_store_locked(
            TrustStoreClass::OsKeychain,
            &[AffectedCapabilityClass::RegistryAuth],
        );
        assert_eq!(affected, 1);
        packet.validate().expect("locked packet validates");

        let row = &packet.rows[0];
        assert_eq!(
            row.continuity.continuity_state,
            ContinuityStateClass::PausedCredentialStoreLocked
        );
        assert_eq!(row.storage.unlock_state, UnlockStateClass::Locked);
        assert_eq!(
            row.continuity.denial_reason,
            Some(SecretBrokerDenialReason::TrustStoreLocked)
        );
        assert!(row.continuity.local_work_continues);
        assert!(!row.storage.plaintext_persistence_allowed);
        assert!(!row.storage.silent_in_memory_promotion_allowed);
        assert!(!row.storage.stale_ticket_reuse_allowed);
    }

    #[test]
    fn guardrails_reject_plaintext_silent_promotion_and_stale_reuse() {
        let mut row = handle_row();
        row.storage.plaintext_persistence_allowed = true;
        assert!(matches!(
            row.validate(),
            Err(SecretBrokerRowError::PlaintextPersistenceAllowed { .. })
        ));

        let mut row = handle_row();
        row.storage.silent_in_memory_promotion_allowed = true;
        assert!(matches!(
            row.validate(),
            Err(SecretBrokerRowError::SilentInMemoryPromotionAllowed { .. })
        ));

        let mut row = handle_row();
        row.storage.stale_ticket_reuse_allowed = true;
        assert!(matches!(
            row.validate(),
            Err(SecretBrokerRowError::StaleTicketReuseAllowed { .. })
        ));
    }
}
