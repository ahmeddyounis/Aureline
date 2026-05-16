//! Secret-broker beta page: handle-only projection, vault/keychain binding,
//! and consumer-identity audit across the four beta profiles.
//!
//! This module owns the first executable secret-broker beta contract. The
//! alpha (see [`crate::secrets`]) froze the per-row vocabulary for handles,
//! session-only references, delegated credentials, and the redaction-safe
//! support export. The beta builds on that vocabulary and adds:
//!
//! - One **handle-only projection** record per `(profile, vault adapter,
//!   consumer, target, workspace scope, secret class)` tuple. The row carries
//!   the canonical reference mode (`handle`, `delegated`, or visibly degraded
//!   `session_only`), the admitted projection mode (alias-only, broker
//!   callback, ephemeral fd, bounded mount, env-var isolated child, sign-only,
//!   decrypt-only, token exchange, policy-materialised, inspect-metadata, or
//!   request-header signer), the vault signature posture, and the typed
//!   lifecycle state. Raw secret bytes and raw handle ids never travel on the
//!   row.
//! - A **consumer-identity audit** stream listing every projection request
//!   with consumer id, capability hash ref, target, workspace scope, secret
//!   class, projection mode, and a typed outcome (granted on a specific
//!   reference mode, or one of ten typed denial reasons such as
//!   `denied_by_plaintext_requested`, `denied_by_silent_in_memory_promotion`,
//!   `denied_by_stale_handle_reuse`, `denied_by_public_endpoint_fallback`,
//!   etc.). Support exports preserve this lineage verbatim.
//! - Per-profile validation that closes managed authority on `mirror_only`,
//!   `offline`, and `enterprise_managed` profiles when a vault adapter that
//!   claims managed authority is missing a verified signature posture.
//! - Typed defects (`raw_secret_material_present`,
//!   `plaintext_persistence_allowed`, `silent_in_memory_promotion_allowed`,
//!   `stale_handle_reuse_allowed`, `public_endpoint_fallback_allowed`,
//!   `handle_ref_missing`, `delegated_credential_ref_missing`,
//!   `session_ref_missing`, `consumer_audit_missing`,
//!   `consumer_lineage_drift`, `denied_audit_missing_reason`,
//!   `managed_authority_missing_signature`, `profile_coverage_missing`,
//!   `vault_adapter_token_drift`, `profile_token_drift`,
//!   `projection_mode_token_drift`, `reference_mode_token_drift`,
//!   `secret_class_token_drift`, `lifecycle_state_token_drift`,
//!   `audit_outcome_token_drift`) so admin, support, and reviewer surfaces
//!   share one defect vocabulary with the auditing surface.
//! - A redaction-safe [`SecretBrokerBetaSupportExport`] wrapper that the
//!   shell/headless inspector, admin console, and reviewer fixtures replay
//!   for support and audit packets.
//!
//! Reviewer-facing landing page:
//! [`/docs/security/m3/secret_broker_beta.md`](../../../../docs/security/m3/secret_broker_beta.md).

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

pub use crate::secrets::{
    SecretClass, SecretConsumerIdentity, SecretReferenceMode, TrustStoreClass,
};

/// Beta schema version exported with every secret-broker beta record.
pub const SECRET_BROKER_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every secret-broker beta record.
pub const SECRET_BROKER_BETA_SHARED_CONTRACT_REF: &str = "security:secret_broker_beta:v1";

/// Source matrix ref consumed by this beta projection.
pub const SECRET_BROKER_BETA_SOURCE_MATRIX_REF: &str =
    "artifacts/security/m3/secret_broker/secret_broker_matrix.yaml";

/// Stable record kind for [`SecretBrokerBetaPage`] payloads.
pub const SECRET_BROKER_BETA_PAGE_RECORD_KIND: &str = "security_secret_broker_beta_page_record";

/// Stable record kind for [`SecretBrokerBetaHandleRow`] payloads.
pub const SECRET_BROKER_BETA_HANDLE_ROW_RECORD_KIND: &str =
    "security_secret_broker_beta_handle_row_record";

/// Stable record kind for [`SecretConsumerAuditEvent`] payloads.
pub const SECRET_BROKER_BETA_CONSUMER_AUDIT_RECORD_KIND: &str =
    "security_secret_broker_beta_consumer_audit_record";

/// Stable record kind for [`SecretBrokerBetaSummary`] payloads.
pub const SECRET_BROKER_BETA_SUMMARY_RECORD_KIND: &str =
    "security_secret_broker_beta_summary_record";

/// Stable record kind for [`SecretBrokerBetaDefect`] payloads.
pub const SECRET_BROKER_BETA_DEFECT_RECORD_KIND: &str = "security_secret_broker_beta_defect_record";

/// Stable record kind for [`SecretBrokerBetaSupportExport`] payloads.
pub const SECRET_BROKER_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "security_secret_broker_beta_support_export_record";

/// Profile under which a secret-broker row and its consumer audit is
/// inspected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBrokerBetaProfileClass {
    /// Connected beta profile with live vault/keychain paths available.
    Connected,
    /// Mirror-only profile served from a declared signed mirror or vault
    /// mirror snapshot.
    MirrorOnly,
    /// Offline profile served from an air-gapped or last-known-good vault
    /// snapshot.
    Offline,
    /// Enterprise-managed profile applying signed managed narrowing.
    EnterpriseManaged,
}

impl SecretBrokerBetaProfileClass {
    /// All required beta profiles in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Connected,
        Self::MirrorOnly,
        Self::Offline,
        Self::EnterpriseManaged,
    ];

    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::MirrorOnly => "mirror_only",
            Self::Offline => "offline",
            Self::EnterpriseManaged => "enterprise_managed",
        }
    }
}

/// Vault or keychain adapter class that backs a secret-broker beta row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VaultAdapterClass {
    /// Platform credential store: macOS Keychain, Windows Credential Manager,
    /// or Linux Secret Service.
    OsKeychain,
    /// Live enterprise vault reachable over a managed transport.
    EnterpriseVaultManaged,
    /// Customer self-hosted enterprise vault served from a declared mirror.
    EnterpriseVaultSelfHostedMirror,
    /// Air-gapped enterprise vault snapshot delivered out-of-band.
    EnterpriseVaultAirGappedSnapshot,
    /// Platform agent: SSH agent, smartcard agent, or biometric agent.
    PlatformAgent,
    /// Hardware security module or cloud KMS backing.
    HsmOrKmsBacked,
    /// Process-local memory cache for visible session-only degraded fallback.
    SessionMemoryCache,
    /// Managed policy injector materialising narrow ephemeral authority.
    ManagedPolicyInjector,
}

impl VaultAdapterClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsKeychain => "os_keychain",
            Self::EnterpriseVaultManaged => "enterprise_vault_managed",
            Self::EnterpriseVaultSelfHostedMirror => "enterprise_vault_self_hosted_mirror",
            Self::EnterpriseVaultAirGappedSnapshot => "enterprise_vault_air_gapped_snapshot",
            Self::PlatformAgent => "platform_agent",
            Self::HsmOrKmsBacked => "hsm_or_kms_backed",
            Self::SessionMemoryCache => "session_memory_cache",
            Self::ManagedPolicyInjector => "managed_policy_injector",
        }
    }

    /// True when this adapter can carry managed authority for enterprise
    /// pilots and therefore requires a verified vault signature posture.
    pub const fn is_managed_authority(self) -> bool {
        matches!(
            self,
            Self::EnterpriseVaultManaged
                | Self::EnterpriseVaultSelfHostedMirror
                | Self::EnterpriseVaultAirGappedSnapshot
                | Self::HsmOrKmsBacked
                | Self::ManagedPolicyInjector
        )
    }
}

/// Vault signature posture for a secret-broker beta row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VaultSignatureStateClass {
    /// Verified live vault entry pulled over a managed transport.
    VerifiedLive,
    /// Verified entry served from a signed mirror.
    VerifiedMirror,
    /// Verified entry imported from a manually-delivered signed bundle.
    VerifiedManualImport,
    /// Verified entry from an air-gapped signed transfer.
    VerifiedAirGapped,
    /// Signature is not required because the row is a local-origin entry that
    /// cannot widen managed authority.
    NotRequiredLocalOrigin,
}

impl VaultSignatureStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifiedLive => "verified_live",
            Self::VerifiedMirror => "verified_mirror",
            Self::VerifiedManualImport => "verified_manual_import",
            Self::VerifiedAirGapped => "verified_air_gapped",
            Self::NotRequiredLocalOrigin => "not_required_local_origin",
        }
    }

    /// True when the posture is verified.
    pub const fn is_verified(self) -> bool {
        matches!(
            self,
            Self::VerifiedLive
                | Self::VerifiedMirror
                | Self::VerifiedManualImport
                | Self::VerifiedAirGapped
        )
    }
}

/// Projection mode admitted by the beta broker.
///
/// The variants mirror the alpha vocabulary but the beta enforces that one
/// projection mode is named per row (no implicit promotion) and that the row
/// declares whether the consumer ever receives raw bytes (always false).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandleProjectionModeClass {
    /// Alias only; the broker resolves on its own callback.
    AliasOnly,
    /// Broker resolves on callback and does not return raw material.
    BrokerCallback,
    /// Broker owns request construction when a protocol requires a header.
    RequestHeaderSigner,
    /// Broker lends an anonymous, short-lived file descriptor.
    EphemeralFd,
    /// Broker mounts an operation-bound, tmpfs-class secret view.
    BoundedMount,
    /// Broker injects into one isolated child process only.
    EnvVarIsolatedChild,
    /// Consumer sends data to sign; private bytes never leave the store.
    SignOnly,
    /// Consumer sends data to decrypt; private bytes never leave the store.
    DecryptOnly,
    /// Broker exchanges a source handle for a narrower operation token.
    TokenExchange,
    /// Managed policy materialises narrow authority per call.
    PolicyMaterialised,
    /// Diagnostics inspects metadata only.
    InspectMetadata,
}

impl HandleProjectionModeClass {
    /// Stable token recorded on records.
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
        }
    }
}

/// Lifecycle state observed on the secret-broker row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandleLifecycleStateClass {
    /// Handle is live and within freshness window.
    Live,
    /// Handle rotation is in progress; new requests use the successor.
    PendingRotation,
    /// Handle is approaching its `expires_at`; clients should rotate.
    Expiring,
    /// Handle expired; managed authority is closed until rotation.
    Expired,
    /// Handle was explicitly revoked.
    Revoked,
    /// Trust state or vault evidence changed after issuance; row is paused.
    PausedTrustChanged,
    /// Backing vault is locked; row is paused.
    PausedVaultLocked,
    /// Backing vault is unreachable; row is paused.
    PausedVaultUnavailable,
}

impl HandleLifecycleStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::PendingRotation => "pending_rotation",
            Self::Expiring => "expiring",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
            Self::PausedTrustChanged => "paused_trust_changed",
            Self::PausedVaultLocked => "paused_vault_locked",
            Self::PausedVaultUnavailable => "paused_vault_unavailable",
        }
    }

    /// True when the lifecycle state holds managed authority closed.
    pub const fn fails_closed(self) -> bool {
        !matches!(self, Self::Live | Self::PendingRotation | Self::Expiring)
    }
}

/// Typed outcome of one consumer-identity audit event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerAuditOutcomeClass {
    /// Granted on a handle reference.
    GrantedHandle,
    /// Granted on a delegated credential reference.
    GrantedDelegated,
    /// Granted on a visibly-degraded session-only reference.
    GrantedSessionOnly,
    /// Denied because policy forbids the projection.
    DeniedByPolicy,
    /// Denied because trust state downgraded.
    DeniedByTrustState,
    /// Denied because the row's lifecycle state holds authority closed.
    DeniedByLifecycleState,
    /// Denied because a required approval ticket was missing.
    DeniedByMissingApproval,
    /// Denied because the caller asked for a plaintext serialisation.
    DeniedByPlaintextRequested,
    /// Denied because the caller asked the broker to silently promote into an
    /// in-memory fallback without consent.
    DeniedBySilentInMemoryPromotion,
    /// Denied because the caller asked to reuse a stale handle.
    DeniedByStaleHandleReuse,
    /// Denied because the caller's request implied a public-endpoint
    /// fallback.
    DeniedByPublicEndpointFallback,
    /// Denied because the underlying handle expired.
    DeniedByExpiry,
    /// Denied because the underlying handle was revoked.
    DeniedByRevocation,
}

impl ConsumerAuditOutcomeClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GrantedHandle => "granted_handle",
            Self::GrantedDelegated => "granted_delegated",
            Self::GrantedSessionOnly => "granted_session_only",
            Self::DeniedByPolicy => "denied_by_policy",
            Self::DeniedByTrustState => "denied_by_trust_state",
            Self::DeniedByLifecycleState => "denied_by_lifecycle_state",
            Self::DeniedByMissingApproval => "denied_by_missing_approval",
            Self::DeniedByPlaintextRequested => "denied_by_plaintext_requested",
            Self::DeniedBySilentInMemoryPromotion => "denied_by_silent_in_memory_promotion",
            Self::DeniedByStaleHandleReuse => "denied_by_stale_handle_reuse",
            Self::DeniedByPublicEndpointFallback => "denied_by_public_endpoint_fallback",
            Self::DeniedByExpiry => "denied_by_expiry",
            Self::DeniedByRevocation => "denied_by_revocation",
        }
    }

    /// True when the outcome is a denial.
    pub const fn is_denial(self) -> bool {
        !matches!(
            self,
            Self::GrantedHandle | Self::GrantedDelegated | Self::GrantedSessionOnly
        )
    }

    /// True when the outcome granted a reference (any reference mode).
    pub const fn is_grant(self) -> bool {
        !self.is_denial()
    }

    /// Reference mode implied by a grant outcome.
    pub const fn implied_reference_mode(self) -> Option<SecretReferenceMode> {
        match self {
            Self::GrantedHandle => Some(SecretReferenceMode::Handle),
            Self::GrantedDelegated => Some(SecretReferenceMode::Delegated),
            Self::GrantedSessionOnly => Some(SecretReferenceMode::SessionOnly),
            _ => None,
        }
    }
}

/// Reviewable vault-binding block attached to a secret-broker beta row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VaultBinding {
    /// Vault adapter backing the row.
    pub vault_adapter: VaultAdapterClass,
    /// Stable token for [`Self::vault_adapter`].
    pub vault_adapter_token: String,
    /// Reviewable adapter label safe for support export.
    pub adapter_label: String,
    /// Stable opaque vault entry ref (alias, not raw id).
    pub vault_entry_ref: String,
    /// Signature posture observed for the vault entry on this profile.
    pub signature_state: VaultSignatureStateClass,
    /// Stable token for [`Self::signature_state`].
    pub signature_state_token: String,
    /// Opaque signer id recorded on the vault bundle.
    pub signer_id: String,
    /// Signed-at timestamp recorded on the bundle.
    pub signed_at: String,
    /// Fetched-at timestamp on this profile.
    pub fetched_at: String,
    /// Valid-until timestamp on the bundle.
    pub valid_until: String,
    /// Stable ref to the preserved signature blob in the artifact store.
    pub signature_blob_ref: String,
}

/// One consumer-identity audit event.
///
/// One row is appended per projection request, naming which consumer asked
/// for which target/scope/secret class, through which projection mode, and
/// what the typed outcome was.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretConsumerAuditEvent {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable event id.
    pub audit_event_id: String,
    /// Source beta row id.
    pub secret_broker_row_ref: String,
    /// Profile under which this request was inspected.
    pub profile: SecretBrokerBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Consumer that made the request.
    pub consumer: SecretConsumerIdentity,
    /// Opaque target ref for the provider, registry, database, or remote
    /// target.
    pub target_ref: String,
    /// Opaque workspace or worktree scope ref.
    pub workspace_scope_ref: String,
    /// Secret class requested.
    pub secret_class: SecretClass,
    /// Stable token for [`Self::secret_class`].
    pub secret_class_token: String,
    /// Projection mode requested by the consumer.
    pub projection_mode: HandleProjectionModeClass,
    /// Stable token for [`Self::projection_mode`].
    pub projection_mode_token: String,
    /// Typed outcome.
    pub outcome: ConsumerAuditOutcomeClass,
    /// Stable token for [`Self::outcome`].
    pub outcome_token: String,
    /// Reference mode returned when the outcome is a grant. None for denials.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granted_reference_mode: Option<SecretReferenceMode>,
    /// Stable token for [`Self::granted_reference_mode`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granted_reference_mode_token: Option<String>,
    /// Optional denial reason note safe for support export.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denial_note: Option<String>,
    /// Request-at timestamp.
    pub requested_at: String,
    /// True when no raw secret material is present on the audit record.
    pub raw_secret_material_present: bool,
    /// True when no raw runtime handle id is exposed on the audit record.
    pub raw_handle_id_exposed: bool,
    /// True when this request did not imply a public-endpoint fallback.
    pub no_public_endpoint_fallback: bool,
}

/// One claimed secret-broker beta handle row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretBrokerBetaHandleRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id safe for UI, logs, and support export.
    pub secret_broker_row_id: String,
    /// Reviewable label safe for UI and support export.
    pub display_label: String,
    /// Profile under which this row is inspected.
    pub profile: SecretBrokerBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Consumer permitted to resolve or use the reference.
    pub consumer: SecretConsumerIdentity,
    /// Opaque target ref.
    pub target_ref: String,
    /// Opaque workspace scope ref.
    pub workspace_scope_ref: String,
    /// Secret class represented by the row.
    pub secret_class: SecretClass,
    /// Stable token for [`Self::secret_class`].
    pub secret_class_token: String,
    /// Reference mode the row currently issues.
    pub reference_mode: SecretReferenceMode,
    /// Stable token for [`Self::reference_mode`].
    pub reference_mode_token: String,
    /// Projection mode admitted for the row.
    pub projection_mode: HandleProjectionModeClass,
    /// Stable token for [`Self::projection_mode`].
    pub projection_mode_token: String,
    /// Vault/keychain binding.
    pub vault_binding: VaultBinding,
    /// Lifecycle state.
    pub lifecycle_state: HandleLifecycleStateClass,
    /// Stable token for [`Self::lifecycle_state`].
    pub lifecycle_state_token: String,
    /// User-stable broker alias safe for UI and metadata-only exports.
    pub authority_alias_ref: String,
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
    /// Approval ticket ref that admits the projection.
    pub approval_ticket_ref: String,
    /// Reviewable copy naming the revoke or clear path.
    pub revocation_path_label: String,
    /// Mint-at timestamp.
    pub minted_at: String,
    /// Beta guardrail: raw secret material is not present on the row.
    pub raw_secret_material_present: bool,
    /// Beta guardrail: plaintext persistence is not admitted on the row.
    pub plaintext_persistence_allowed: bool,
    /// Beta guardrail: silent in-memory promotion is not admitted on the row.
    pub silent_in_memory_promotion_allowed: bool,
    /// Beta guardrail: stale handle reuse is not admitted on the row.
    pub stale_handle_reuse_allowed: bool,
    /// Beta guardrail: undeclared public-endpoint fallback is not admitted.
    pub public_endpoint_fallback_allowed: bool,
}

/// Defect-kind vocabulary surfaced by the beta validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBrokerBetaDefectKind {
    /// Row claims raw secret material is present.
    RawSecretMaterialPresent,
    /// Row admits plaintext persistence.
    PlaintextPersistenceAllowed,
    /// Row admits silent in-memory promotion.
    SilentInMemoryPromotionAllowed,
    /// Row admits stale handle reuse.
    StaleHandleReuseAllowed,
    /// Row admits undeclared public-endpoint fallback.
    PublicEndpointFallbackAllowed,
    /// Row is handle-mode without a credential handle ref.
    HandleRefMissing,
    /// Row is session-only without a session ref.
    SessionRefMissing,
    /// Row is delegated without a delegated credential ref.
    DelegatedCredentialRefMissing,
    /// Row claims a managed-authority vault adapter but no verified signature
    /// posture.
    ManagedAuthorityMissingSignature,
    /// Row has no consumer audit events.
    ConsumerAuditMissing,
    /// Audit event names a consumer that does not match the row.
    ConsumerLineageDrift,
    /// Audit event was a denial without a denial note.
    DeniedAuditMissingReason,
    /// Audit grant outcome implied a reference mode that the row does not
    /// issue.
    GrantedReferenceModeMismatch,
    /// One of the four required beta profiles has no claimed row.
    ProfileCoverageMissing,
    /// `profile_token` did not match `profile`.
    ProfileTokenDrift,
    /// `vault_adapter_token` did not match `vault_adapter`.
    VaultAdapterTokenDrift,
    /// `signature_state_token` did not match `signature_state`.
    SignatureStateTokenDrift,
    /// `projection_mode_token` did not match `projection_mode`.
    ProjectionModeTokenDrift,
    /// `reference_mode_token` did not match `reference_mode`.
    ReferenceModeTokenDrift,
    /// `secret_class_token` did not match `secret_class`.
    SecretClassTokenDrift,
    /// `lifecycle_state_token` did not match `lifecycle_state`.
    LifecycleStateTokenDrift,
    /// Audit `outcome_token` did not match `outcome`.
    AuditOutcomeTokenDrift,
}

impl SecretBrokerBetaDefectKind {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawSecretMaterialPresent => "raw_secret_material_present",
            Self::PlaintextPersistenceAllowed => "plaintext_persistence_allowed",
            Self::SilentInMemoryPromotionAllowed => "silent_in_memory_promotion_allowed",
            Self::StaleHandleReuseAllowed => "stale_handle_reuse_allowed",
            Self::PublicEndpointFallbackAllowed => "public_endpoint_fallback_allowed",
            Self::HandleRefMissing => "handle_ref_missing",
            Self::SessionRefMissing => "session_ref_missing",
            Self::DelegatedCredentialRefMissing => "delegated_credential_ref_missing",
            Self::ManagedAuthorityMissingSignature => "managed_authority_missing_signature",
            Self::ConsumerAuditMissing => "consumer_audit_missing",
            Self::ConsumerLineageDrift => "consumer_lineage_drift",
            Self::DeniedAuditMissingReason => "denied_audit_missing_reason",
            Self::GrantedReferenceModeMismatch => "granted_reference_mode_mismatch",
            Self::ProfileCoverageMissing => "profile_coverage_missing",
            Self::ProfileTokenDrift => "profile_token_drift",
            Self::VaultAdapterTokenDrift => "vault_adapter_token_drift",
            Self::SignatureStateTokenDrift => "signature_state_token_drift",
            Self::ProjectionModeTokenDrift => "projection_mode_token_drift",
            Self::ReferenceModeTokenDrift => "reference_mode_token_drift",
            Self::SecretClassTokenDrift => "secret_class_token_drift",
            Self::LifecycleStateTokenDrift => "lifecycle_state_token_drift",
            Self::AuditOutcomeTokenDrift => "audit_outcome_token_drift",
        }
    }
}

/// Typed validation defect for the secret-broker beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretBrokerBetaDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Defect kind.
    pub defect_kind: SecretBrokerBetaDefectKind,
    /// Stable token for [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Subject id (row id, audit event id, or `"page"`).
    pub subject_id: String,
    /// Field that failed validation.
    pub field: String,
    /// Export-safe explanation.
    pub note: String,
}

impl SecretBrokerBetaDefect {
    fn new(
        defect_kind: SecretBrokerBetaDefectKind,
        subject_id: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: SECRET_BROKER_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: SECRET_BROKER_BETA_SCHEMA_VERSION,
            shared_contract_ref: SECRET_BROKER_BETA_SHARED_CONTRACT_REF.to_owned(),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            subject_id: subject_id.into(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Aggregate summary for the secret-broker beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretBrokerBetaSummary {
    /// Stable record kind of the parent page.
    pub page_record_kind: String,
    /// Stable record kind of the summary.
    pub record_kind: String,
    /// Number of handle rows.
    pub handle_row_count: usize,
    /// Number of consumer-audit events.
    pub consumer_audit_count: usize,
    /// Profile tokens present across the page.
    pub profiles_present: Vec<String>,
    /// Vault adapter tokens present across the page.
    pub vault_adapters_present: Vec<String>,
    /// Reference-mode tokens present across the page.
    pub reference_modes_present: Vec<String>,
    /// Projection-mode tokens present across the page.
    pub projection_modes_present: Vec<String>,
    /// Secret-class tokens present across the page.
    pub secret_classes_present: Vec<String>,
    /// Lifecycle-state tokens present across the page.
    pub lifecycle_states_present: Vec<String>,
    /// Audit-outcome tokens present across the page.
    pub audit_outcomes_present: Vec<String>,
    /// Counts of audit grants by reference mode token.
    pub audit_grants_by_reference_mode: BTreeMap<String, usize>,
    /// Counts of audit denials by outcome token.
    pub audit_denials_by_outcome: BTreeMap<String, usize>,
    /// Number of defects.
    pub defect_count: usize,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
}

impl SecretBrokerBetaSummary {
    /// Builds the summary from rows, audit events, and defects.
    pub fn from_records(
        handle_rows: &[SecretBrokerBetaHandleRow],
        consumer_audit: &[SecretConsumerAuditEvent],
        defects: &[SecretBrokerBetaDefect],
    ) -> Self {
        let mut profiles_present: BTreeSet<String> = BTreeSet::new();
        let mut vault_adapters_present: BTreeSet<String> = BTreeSet::new();
        let mut reference_modes_present: BTreeSet<String> = BTreeSet::new();
        let mut projection_modes_present: BTreeSet<String> = BTreeSet::new();
        let mut secret_classes_present: BTreeSet<String> = BTreeSet::new();
        let mut lifecycle_states_present: BTreeSet<String> = BTreeSet::new();
        let mut audit_outcomes_present: BTreeSet<String> = BTreeSet::new();

        for row in handle_rows {
            profiles_present.insert(row.profile_token.clone());
            vault_adapters_present.insert(row.vault_binding.vault_adapter_token.clone());
            reference_modes_present.insert(row.reference_mode_token.clone());
            projection_modes_present.insert(row.projection_mode_token.clone());
            secret_classes_present.insert(row.secret_class_token.clone());
            lifecycle_states_present.insert(row.lifecycle_state_token.clone());
        }
        for event in consumer_audit {
            profiles_present.insert(event.profile_token.clone());
            projection_modes_present.insert(event.projection_mode_token.clone());
            secret_classes_present.insert(event.secret_class_token.clone());
            audit_outcomes_present.insert(event.outcome_token.clone());
            if let Some(token) = &event.granted_reference_mode_token {
                reference_modes_present.insert(token.clone());
            }
        }

        let mut audit_grants_by_reference_mode: BTreeMap<String, usize> = BTreeMap::new();
        let mut audit_denials_by_outcome: BTreeMap<String, usize> = BTreeMap::new();
        for event in consumer_audit {
            if event.outcome.is_grant() {
                if let Some(token) = &event.granted_reference_mode_token {
                    *audit_grants_by_reference_mode
                        .entry(token.clone())
                        .or_insert(0) += 1;
                }
            } else {
                *audit_denials_by_outcome
                    .entry(event.outcome_token.clone())
                    .or_insert(0) += 1;
            }
        }

        let mut defect_counts_by_kind: BTreeMap<String, usize> = BTreeMap::new();
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind_token.clone())
                .or_insert(0) += 1;
        }

        Self {
            page_record_kind: SECRET_BROKER_BETA_PAGE_RECORD_KIND.to_owned(),
            record_kind: SECRET_BROKER_BETA_SUMMARY_RECORD_KIND.to_owned(),
            handle_row_count: handle_rows.len(),
            consumer_audit_count: consumer_audit.len(),
            profiles_present: profiles_present.into_iter().collect(),
            vault_adapters_present: vault_adapters_present.into_iter().collect(),
            reference_modes_present: reference_modes_present.into_iter().collect(),
            projection_modes_present: projection_modes_present.into_iter().collect(),
            secret_classes_present: secret_classes_present.into_iter().collect(),
            lifecycle_states_present: lifecycle_states_present.into_iter().collect(),
            audit_outcomes_present: audit_outcomes_present.into_iter().collect(),
            audit_grants_by_reference_mode,
            audit_denials_by_outcome,
            defect_count: defects.len(),
            defect_counts_by_kind,
        }
    }
}

/// Top-level beta page consumed by admin, support, shell, and reviewer
/// fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretBrokerBetaPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Source matrix ref.
    pub source_matrix_ref: String,
    /// Claimed secret-broker handle rows.
    pub handle_rows: Vec<SecretBrokerBetaHandleRow>,
    /// Consumer-identity audit events.
    pub consumer_audit: Vec<SecretConsumerAuditEvent>,
    /// Typed validation defects.
    pub defects: Vec<SecretBrokerBetaDefect>,
    /// Aggregate summary.
    pub summary: SecretBrokerBetaSummary,
}

/// Support-export wrapper for the secret-broker beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretBrokerBetaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Exported page.
    pub page: SecretBrokerBetaPage,
    /// Defect-kind tokens present in the page.
    pub defect_kinds_present: Vec<String>,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True when raw secret values are excluded from the export.
    pub raw_secret_values_excluded: bool,
    /// True when raw runtime handle ids are excluded from the export.
    pub raw_handle_ids_excluded: bool,
    /// True when consumer lineage (consumer id, target, scope, projection
    /// mode, outcome) is preserved verbatim.
    pub consumer_lineage_preserved: bool,
    /// Reviewable summary of the redaction posture.
    pub redaction_summary: String,
}

impl SecretBrokerBetaSupportExport {
    /// Builds a support-export wrapper from a beta page.
    ///
    /// The wrapped page is redaction-cleaned: runtime handle ids, session
    /// refs, and delegated credential refs are dropped from the embedded
    /// handle rows. Consumer lineage (consumer id, target ref, workspace
    /// scope ref, projection mode, typed outcome) is preserved verbatim
    /// because none of those carry raw secret bytes.
    pub fn from_page(
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
        page: SecretBrokerBetaPage,
    ) -> Self {
        let mut redacted_page = page;
        for row in &mut redacted_page.handle_rows {
            row.credential_handle_ref = None;
            row.session_ref = None;
            row.delegated_credential_ref = None;
        }

        let defect_counts_by_kind = redacted_page.summary.defect_counts_by_kind.clone();
        let defect_kinds_present = defect_counts_by_kind.keys().cloned().collect();
        Self {
            record_kind: SECRET_BROKER_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SECRET_BROKER_BETA_SCHEMA_VERSION,
            shared_contract_ref: SECRET_BROKER_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            page: redacted_page,
            defect_kinds_present,
            defect_counts_by_kind,
            raw_secret_values_excluded: true,
            raw_handle_ids_excluded: true,
            consumer_lineage_preserved: true,
            redaction_summary:
                "Metadata-only secret-broker beta export: vault adapter, signature posture, \
                 projection mode, consumer lineage, target/scope refs, and typed audit \
                 outcomes are preserved; raw secret values and raw runtime handle ids are \
                 excluded."
                    .to_owned(),
        }
    }
}

/// Validates the secret-broker beta page and returns typed defects on
/// failure.
pub fn validate_secret_broker_beta_page(
    page: &SecretBrokerBetaPage,
) -> Result<(), Vec<SecretBrokerBetaDefect>> {
    let defects = audit_secret_broker_beta_page(&page.handle_rows, &page.consumer_audit);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Recomputes defects for a secret-broker beta page.
pub fn audit_secret_broker_beta_page(
    handle_rows: &[SecretBrokerBetaHandleRow],
    consumer_audit: &[SecretConsumerAuditEvent],
) -> Vec<SecretBrokerBetaDefect> {
    let mut defects = Vec::new();

    let row_ids: BTreeSet<&str> = handle_rows
        .iter()
        .map(|row| row.secret_broker_row_id.as_str())
        .collect();

    for row in handle_rows {
        if row.profile_token != row.profile.as_str() {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::ProfileTokenDrift,
                row.secret_broker_row_id.clone(),
                "profile_token",
                "profile_token must match profile",
            ));
        }
        if row.vault_binding.vault_adapter_token != row.vault_binding.vault_adapter.as_str() {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::VaultAdapterTokenDrift,
                row.secret_broker_row_id.clone(),
                "vault_binding.vault_adapter_token",
                "vault_adapter_token must match vault_adapter",
            ));
        }
        if row.vault_binding.signature_state_token != row.vault_binding.signature_state.as_str() {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::SignatureStateTokenDrift,
                row.secret_broker_row_id.clone(),
                "vault_binding.signature_state_token",
                "signature_state_token must match signature_state",
            ));
        }
        if row.projection_mode_token != row.projection_mode.as_str() {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::ProjectionModeTokenDrift,
                row.secret_broker_row_id.clone(),
                "projection_mode_token",
                "projection_mode_token must match projection_mode",
            ));
        }
        if row.reference_mode_token != row.reference_mode.as_str() {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::ReferenceModeTokenDrift,
                row.secret_broker_row_id.clone(),
                "reference_mode_token",
                "reference_mode_token must match reference_mode",
            ));
        }
        if row.secret_class_token != row.secret_class.as_str() {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::SecretClassTokenDrift,
                row.secret_broker_row_id.clone(),
                "secret_class_token",
                "secret_class_token must match secret_class",
            ));
        }
        if row.lifecycle_state_token != row.lifecycle_state.as_str() {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::LifecycleStateTokenDrift,
                row.secret_broker_row_id.clone(),
                "lifecycle_state_token",
                "lifecycle_state_token must match lifecycle_state",
            ));
        }

        if row.raw_secret_material_present {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::RawSecretMaterialPresent,
                row.secret_broker_row_id.clone(),
                "raw_secret_material_present",
                "claimed beta row must not carry raw secret material",
            ));
        }
        if row.plaintext_persistence_allowed {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::PlaintextPersistenceAllowed,
                row.secret_broker_row_id.clone(),
                "plaintext_persistence_allowed",
                "claimed beta row must not admit plaintext persistence",
            ));
        }
        if row.silent_in_memory_promotion_allowed {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::SilentInMemoryPromotionAllowed,
                row.secret_broker_row_id.clone(),
                "silent_in_memory_promotion_allowed",
                "claimed beta row must not admit silent in-memory promotion",
            ));
        }
        if row.stale_handle_reuse_allowed {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::StaleHandleReuseAllowed,
                row.secret_broker_row_id.clone(),
                "stale_handle_reuse_allowed",
                "claimed beta row must not admit stale handle reuse",
            ));
        }
        if row.public_endpoint_fallback_allowed {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::PublicEndpointFallbackAllowed,
                row.secret_broker_row_id.clone(),
                "public_endpoint_fallback_allowed",
                "claimed beta row must not admit undeclared public-endpoint fallback",
            ));
        }

        match row.reference_mode {
            SecretReferenceMode::Handle if row.credential_handle_ref.is_none() => {
                defects.push(SecretBrokerBetaDefect::new(
                    SecretBrokerBetaDefectKind::HandleRefMissing,
                    row.secret_broker_row_id.clone(),
                    "credential_handle_ref",
                    "handle-mode row must declare a credential handle ref",
                ));
            }
            SecretReferenceMode::SessionOnly if row.session_ref.is_none() => {
                defects.push(SecretBrokerBetaDefect::new(
                    SecretBrokerBetaDefectKind::SessionRefMissing,
                    row.secret_broker_row_id.clone(),
                    "session_ref",
                    "session-only row must declare a session ref",
                ));
            }
            SecretReferenceMode::Delegated if row.delegated_credential_ref.is_none() => {
                defects.push(SecretBrokerBetaDefect::new(
                    SecretBrokerBetaDefectKind::DelegatedCredentialRefMissing,
                    row.secret_broker_row_id.clone(),
                    "delegated_credential_ref",
                    "delegated row must declare a delegated credential ref",
                ));
            }
            _ => {}
        }

        if row.vault_binding.vault_adapter.is_managed_authority()
            && !row.vault_binding.signature_state.is_verified()
        {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::ManagedAuthorityMissingSignature,
                row.secret_broker_row_id.clone(),
                "vault_binding.signature_state",
                "managed-authority vault adapter must carry a verified signature posture",
            ));
        }
        if row.vault_binding.vault_adapter.is_managed_authority()
            && row.vault_binding.signature_blob_ref.is_empty()
        {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::ManagedAuthorityMissingSignature,
                row.secret_broker_row_id.clone(),
                "vault_binding.signature_blob_ref",
                "managed-authority vault adapter must preserve signature blob ref",
            ));
        }

        let has_audit = consumer_audit
            .iter()
            .any(|event| event.secret_broker_row_ref == row.secret_broker_row_id);
        if !has_audit {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::ConsumerAuditMissing,
                row.secret_broker_row_id.clone(),
                "consumer_audit",
                "row has no consumer-identity audit events",
            ));
        }
    }

    for event in consumer_audit {
        if event.profile_token != event.profile.as_str() {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::ProfileTokenDrift,
                event.audit_event_id.clone(),
                "profile_token",
                "audit profile_token must match profile",
            ));
        }
        if event.projection_mode_token != event.projection_mode.as_str() {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::ProjectionModeTokenDrift,
                event.audit_event_id.clone(),
                "projection_mode_token",
                "audit projection_mode_token must match projection_mode",
            ));
        }
        if event.secret_class_token != event.secret_class.as_str() {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::SecretClassTokenDrift,
                event.audit_event_id.clone(),
                "secret_class_token",
                "audit secret_class_token must match secret_class",
            ));
        }
        if event.outcome_token != event.outcome.as_str() {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::AuditOutcomeTokenDrift,
                event.audit_event_id.clone(),
                "outcome_token",
                "audit outcome_token must match outcome",
            ));
        }
        if event.raw_secret_material_present {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::RawSecretMaterialPresent,
                event.audit_event_id.clone(),
                "raw_secret_material_present",
                "audit event must not carry raw secret material",
            ));
        }
        if event.raw_handle_id_exposed {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::RawSecretMaterialPresent,
                event.audit_event_id.clone(),
                "raw_handle_id_exposed",
                "audit event must not expose raw runtime handle ids",
            ));
        }
        if !event.no_public_endpoint_fallback {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::PublicEndpointFallbackAllowed,
                event.audit_event_id.clone(),
                "no_public_endpoint_fallback",
                "audit event must refuse undeclared public-endpoint fallback",
            ));
        }

        if !row_ids.contains(event.secret_broker_row_ref.as_str()) {
            defects.push(SecretBrokerBetaDefect::new(
                SecretBrokerBetaDefectKind::ConsumerLineageDrift,
                event.audit_event_id.clone(),
                "secret_broker_row_ref",
                "audit event names a row id that is not present on the page",
            ));
        } else if let Some(row) = handle_rows
            .iter()
            .find(|row| row.secret_broker_row_id == event.secret_broker_row_ref)
        {
            if event.consumer.consumer_id != row.consumer.consumer_id
                || event.consumer.capability_hash_ref != row.consumer.capability_hash_ref
                || event.target_ref != row.target_ref
                || event.workspace_scope_ref != row.workspace_scope_ref
                || event.secret_class != row.secret_class
                || event.profile != row.profile
            {
                defects.push(SecretBrokerBetaDefect::new(
                    SecretBrokerBetaDefectKind::ConsumerLineageDrift,
                    event.audit_event_id.clone(),
                    "consumer/target/scope",
                    "audit lineage must match the source row's consumer, target, scope, class, and profile",
                ));
            }
            if event.outcome.is_grant() {
                if let Some(implied) = event.outcome.implied_reference_mode() {
                    if implied != row.reference_mode {
                        defects.push(SecretBrokerBetaDefect::new(
                            SecretBrokerBetaDefectKind::GrantedReferenceModeMismatch,
                            event.audit_event_id.clone(),
                            "outcome",
                            "audit grant implies a reference mode the row does not issue",
                        ));
                    }
                }
            }
        }

        match (event.outcome.is_grant(), &event.granted_reference_mode) {
            (true, None) => {
                defects.push(SecretBrokerBetaDefect::new(
                    SecretBrokerBetaDefectKind::GrantedReferenceModeMismatch,
                    event.audit_event_id.clone(),
                    "granted_reference_mode",
                    "grant audit must declare a granted_reference_mode",
                ));
            }
            (true, Some(mode)) => {
                let expected_token = mode.as_str();
                let token_ok = event
                    .granted_reference_mode_token
                    .as_deref()
                    .map(|token| token == expected_token)
                    .unwrap_or(false);
                if !token_ok {
                    defects.push(SecretBrokerBetaDefect::new(
                        SecretBrokerBetaDefectKind::ReferenceModeTokenDrift,
                        event.audit_event_id.clone(),
                        "granted_reference_mode_token",
                        "granted_reference_mode_token must match granted_reference_mode",
                    ));
                }
            }
            (false, _) => {
                if event.denial_note.is_none() {
                    defects.push(SecretBrokerBetaDefect::new(
                        SecretBrokerBetaDefectKind::DeniedAuditMissingReason,
                        event.audit_event_id.clone(),
                        "denial_note",
                        "denied audit event must carry a denial note",
                    ));
                }
            }
        }
    }

    let observed_profiles: BTreeSet<&str> = handle_rows
        .iter()
        .map(|row| row.profile_token.as_str())
        .collect();
    let required_profiles: BTreeSet<&str> = SecretBrokerBetaProfileClass::ALL
        .iter()
        .map(|profile| profile.as_str())
        .collect();
    for missing in required_profiles.difference(&observed_profiles) {
        defects.push(SecretBrokerBetaDefect::new(
            SecretBrokerBetaDefectKind::ProfileCoverageMissing,
            "page",
            "profiles",
            format!("missing {} profile coverage", missing),
        ));
    }

    defects
}

/// Builds the seeded secret-broker beta page covering connected, mirror,
/// offline, and enterprise-managed profiles with handle, delegated, and
/// session-only rows plus consumer-identity audit events.
pub fn seeded_secret_broker_beta_page() -> SecretBrokerBetaPage {
    let handle_rows = seed_handle_rows();
    let consumer_audit = seed_consumer_audit(&handle_rows);

    let defects = audit_secret_broker_beta_page(&handle_rows, &consumer_audit);
    let summary = SecretBrokerBetaSummary::from_records(&handle_rows, &consumer_audit, &defects);

    SecretBrokerBetaPage {
        record_kind: SECRET_BROKER_BETA_PAGE_RECORD_KIND.to_owned(),
        schema_version: SECRET_BROKER_BETA_SCHEMA_VERSION,
        shared_contract_ref: SECRET_BROKER_BETA_SHARED_CONTRACT_REF.to_owned(),
        source_matrix_ref: SECRET_BROKER_BETA_SOURCE_MATRIX_REF.to_owned(),
        handle_rows,
        consumer_audit,
        defects,
        summary,
    }
}

fn seed_handle_rows() -> Vec<SecretBrokerBetaHandleRow> {
    vec![
        handle_row(HandleRowSeed {
            row_id: "secret-broker-beta:row:connected:registry-auth",
            display_label: "Registry auth credential-store handle",
            profile: SecretBrokerBetaProfileClass::Connected,
            consumer_id: "consumer:package-registry:npm",
            consumer_label: "Package registry client",
            capability_hash_ref: "capability-hash:package-registry-client:beta",
            target_ref: "registry:npm:public",
            workspace_scope_ref: "workspace:local:payments",
            secret_class: SecretClass::PackageRegistryToken,
            reference_mode: SecretReferenceMode::Handle,
            projection_mode: HandleProjectionModeClass::BrokerCallback,
            vault_adapter: VaultAdapterClass::OsKeychain,
            signature_state: VaultSignatureStateClass::NotRequiredLocalOrigin,
            adapter_label: "OS credential store",
            vault_entry_ref: "vault-entry:os-keychain:registry:npm:payments",
            signer_id: "",
            signed_at: "",
            fetched_at: "2026-05-16T01:00:00Z",
            valid_until: "",
            signature_blob_ref: "",
            authority_alias_ref: "secret-alias:registry:npm:payments",
            credential_handle_ref: Some("credential-handle:registry:npm:payments:0001"),
            session_ref: None,
            delegated_credential_ref: None,
            approval_ticket_ref: "approval-ticket:registry-auth:payments:0001",
            revocation_path_label: "Remove saved registry credential",
            lifecycle_state: HandleLifecycleStateClass::Live,
            minted_at: "2026-05-16T01:00:05Z",
        }),
        handle_row(HandleRowSeed {
            row_id: "secret-broker-beta:row:mirror_only:provider-token",
            display_label: "Provider API key from signed vault mirror",
            profile: SecretBrokerBetaProfileClass::MirrorOnly,
            consumer_id: "consumer:ai-provider:byok",
            consumer_label: "BYOK AI provider client",
            capability_hash_ref: "capability-hash:ai-provider:byok:beta",
            target_ref: "provider:byok-ai:tenant-001",
            workspace_scope_ref: "workspace:remote:payments",
            secret_class: SecretClass::AiProviderToken,
            reference_mode: SecretReferenceMode::Handle,
            projection_mode: HandleProjectionModeClass::RequestHeaderSigner,
            vault_adapter: VaultAdapterClass::EnterpriseVaultSelfHostedMirror,
            signature_state: VaultSignatureStateClass::VerifiedMirror,
            adapter_label: "Self-hosted enterprise vault (signed mirror)",
            vault_entry_ref: "vault-entry:enterprise-vault:mirror:provider:byok-ai:tenant-001",
            signer_id: "vault-signer:tenant-managed",
            signed_at: "2026-05-15T22:00:00Z",
            fetched_at: "2026-05-16T01:05:00Z",
            valid_until: "2026-05-22T22:00:00Z",
            signature_blob_ref:
                "artifacts/security/m3/secret_broker/vault_mirror_provider_byok_ai.sig",
            authority_alias_ref: "secret-alias:provider:byok-ai:tenant-001",
            credential_handle_ref: Some("credential-handle:provider:byok-ai:tenant-001:0001"),
            session_ref: None,
            delegated_credential_ref: None,
            approval_ticket_ref: "approval-ticket:provider:byok-ai:tenant-001",
            revocation_path_label: "Revoke vault mirror entry",
            lifecycle_state: HandleLifecycleStateClass::Live,
            minted_at: "2026-05-16T01:05:05Z",
        }),
        handle_row(HandleRowSeed {
            row_id: "secret-broker-beta:row:offline:ssh-deploy-key",
            display_label: "SSH deploy key from air-gapped vault snapshot",
            profile: SecretBrokerBetaProfileClass::Offline,
            consumer_id: "consumer:git:deploy",
            consumer_label: "Git deploy client",
            capability_hash_ref: "capability-hash:git:deploy:beta",
            target_ref: "remote:git:internal-fleet",
            workspace_scope_ref: "workspace:local:fleet",
            secret_class: SecretClass::SshKeyMaterial,
            reference_mode: SecretReferenceMode::Handle,
            projection_mode: HandleProjectionModeClass::SignOnly,
            vault_adapter: VaultAdapterClass::EnterpriseVaultAirGappedSnapshot,
            signature_state: VaultSignatureStateClass::VerifiedAirGapped,
            adapter_label: "Air-gapped enterprise vault snapshot",
            vault_entry_ref: "vault-entry:enterprise-vault:air-gapped:ssh:deploy:fleet:0001",
            signer_id: "vault-signer:tenant-managed",
            signed_at: "2026-05-14T22:00:00Z",
            fetched_at: "2026-05-15T03:00:00Z",
            valid_until: "2026-05-21T22:00:00Z",
            signature_blob_ref:
                "artifacts/security/m3/secret_broker/vault_air_gapped_ssh_deploy.sig",
            authority_alias_ref: "secret-alias:ssh:deploy:fleet:0001",
            credential_handle_ref: Some("credential-handle:ssh:deploy:fleet:0001"),
            session_ref: None,
            delegated_credential_ref: None,
            approval_ticket_ref: "approval-ticket:ssh:deploy:fleet:0001",
            revocation_path_label: "Revoke air-gapped vault snapshot entry",
            lifecycle_state: HandleLifecycleStateClass::Live,
            minted_at: "2026-05-15T03:05:00Z",
        }),
        handle_row(HandleRowSeed {
            row_id: "secret-broker-beta:row:enterprise_managed:tunnel-delegate",
            display_label: "Tunnel reuse via managed delegated credential",
            profile: SecretBrokerBetaProfileClass::EnterpriseManaged,
            consumer_id: "consumer:tunnel:remote",
            consumer_label: "Remote tunnel client",
            capability_hash_ref: "capability-hash:tunnel:remote:beta",
            target_ref: "tunnel:remote:fleet:0001",
            workspace_scope_ref: "workspace:remote:fleet",
            secret_class: SecretClass::EphemeralOperationToken,
            reference_mode: SecretReferenceMode::Delegated,
            projection_mode: HandleProjectionModeClass::TokenExchange,
            vault_adapter: VaultAdapterClass::ManagedPolicyInjector,
            signature_state: VaultSignatureStateClass::VerifiedManualImport,
            adapter_label: "Managed policy injector (signed file import)",
            vault_entry_ref: "vault-entry:policy-injector:tunnel:fleet:0001",
            signer_id: "policy-signer:aureline-baseline",
            signed_at: "2026-05-15T00:00:00Z",
            fetched_at: "2026-05-16T01:30:00Z",
            valid_until: "2026-05-22T00:00:00Z",
            signature_blob_ref:
                "artifacts/security/m3/secret_broker/policy_injector_tunnel_fleet.sig",
            authority_alias_ref: "secret-alias:tunnel:fleet:0001",
            credential_handle_ref: None,
            session_ref: None,
            delegated_credential_ref: Some("delegated-credential:tunnel:fleet:0001"),
            approval_ticket_ref: "approval-ticket:tunnel:fleet:0001",
            revocation_path_label: "Revoke delegated tunnel credential",
            lifecycle_state: HandleLifecycleStateClass::Live,
            minted_at: "2026-05-16T01:30:05Z",
        }),
        handle_row(HandleRowSeed {
            row_id: "secret-broker-beta:row:connected:provider-session-only",
            display_label: "Local-only AI provider session credential",
            profile: SecretBrokerBetaProfileClass::Connected,
            consumer_id: "consumer:ai-provider:local",
            consumer_label: "Local AI provider client",
            capability_hash_ref: "capability-hash:ai-provider:local:beta",
            target_ref: "provider:byok-ai:local",
            workspace_scope_ref: "workspace:local:scratch",
            secret_class: SecretClass::AiProviderToken,
            reference_mode: SecretReferenceMode::SessionOnly,
            projection_mode: HandleProjectionModeClass::BrokerCallback,
            vault_adapter: VaultAdapterClass::SessionMemoryCache,
            signature_state: VaultSignatureStateClass::NotRequiredLocalOrigin,
            adapter_label: "Session memory cache (visible session-only)",
            vault_entry_ref: "vault-entry:session-memory:provider:byok-ai:local",
            signer_id: "",
            signed_at: "",
            fetched_at: "2026-05-16T01:10:00Z",
            valid_until: "",
            signature_blob_ref: "",
            authority_alias_ref: "secret-alias:provider:byok-ai:local",
            credential_handle_ref: None,
            session_ref: Some("session-secret:provider:byok-ai:local-only:0001"),
            delegated_credential_ref: None,
            approval_ticket_ref: "approval-ticket:provider:byok-ai:local:0001",
            revocation_path_label: "Clear session-only provider credential",
            lifecycle_state: HandleLifecycleStateClass::Live,
            minted_at: "2026-05-16T01:10:05Z",
        }),
    ]
}

fn seed_consumer_audit(handle_rows: &[SecretBrokerBetaHandleRow]) -> Vec<SecretConsumerAuditEvent> {
    let mut events = Vec::new();

    // For each seeded handle row, append at least one grant audit event and a
    // typed denial audit event to prove lineage and the denial vocabulary.
    let grants: &[(&str, ConsumerAuditOutcomeClass)] = &[
        (
            "secret-broker-beta:row:connected:registry-auth",
            ConsumerAuditOutcomeClass::GrantedHandle,
        ),
        (
            "secret-broker-beta:row:mirror_only:provider-token",
            ConsumerAuditOutcomeClass::GrantedHandle,
        ),
        (
            "secret-broker-beta:row:offline:ssh-deploy-key",
            ConsumerAuditOutcomeClass::GrantedHandle,
        ),
        (
            "secret-broker-beta:row:enterprise_managed:tunnel-delegate",
            ConsumerAuditOutcomeClass::GrantedDelegated,
        ),
        (
            "secret-broker-beta:row:connected:provider-session-only",
            ConsumerAuditOutcomeClass::GrantedSessionOnly,
        ),
    ];

    for (idx, (row_id, outcome)) in grants.iter().enumerate() {
        let row = handle_rows
            .iter()
            .find(|row| row.secret_broker_row_id == *row_id)
            .expect("seeded grant row exists");
        events.push(consumer_audit_event(AuditEventSeed {
            audit_event_id: &format!("secret-broker-beta:audit:grant:{:03}", idx + 1),
            row,
            outcome: *outcome,
            denial_note: None,
            requested_at: &format!("2026-05-16T02:{:02}:00Z", idx),
        }));
    }

    let denials: &[(&str, ConsumerAuditOutcomeClass, &str)] = &[
        (
            "secret-broker-beta:row:connected:registry-auth",
            ConsumerAuditOutcomeClass::DeniedByPlaintextRequested,
            "Caller requested a plaintext credential body; only handle projection is admitted.",
        ),
        (
            "secret-broker-beta:row:mirror_only:provider-token",
            ConsumerAuditOutcomeClass::DeniedByPublicEndpointFallback,
            "Caller asked the broker to fall back to a public endpoint after mirror loss; \
             refused on the mirror-only profile.",
        ),
        (
            "secret-broker-beta:row:offline:ssh-deploy-key",
            ConsumerAuditOutcomeClass::DeniedByStaleHandleReuse,
            "Caller attempted to reuse a stale handle past the air-gapped snapshot's valid-until.",
        ),
        (
            "secret-broker-beta:row:enterprise_managed:tunnel-delegate",
            ConsumerAuditOutcomeClass::DeniedBySilentInMemoryPromotion,
            "Caller asked the broker to silently promote into an in-memory fallback; refused on \
             the enterprise-managed profile.",
        ),
        (
            "secret-broker-beta:row:connected:provider-session-only",
            ConsumerAuditOutcomeClass::DeniedByPolicy,
            "Provider session-only projection denied by policy for cross-workspace scope reuse.",
        ),
    ];

    for (idx, (row_id, outcome, note)) in denials.iter().enumerate() {
        let row = handle_rows
            .iter()
            .find(|row| row.secret_broker_row_id == *row_id)
            .expect("seeded denial row exists");
        events.push(consumer_audit_event(AuditEventSeed {
            audit_event_id: &format!("secret-broker-beta:audit:denial:{:03}", idx + 1),
            row,
            outcome: *outcome,
            denial_note: Some((*note).to_owned()),
            requested_at: &format!("2026-05-16T03:{:02}:00Z", idx),
        }));
    }

    events
}

struct HandleRowSeed {
    row_id: &'static str,
    display_label: &'static str,
    profile: SecretBrokerBetaProfileClass,
    consumer_id: &'static str,
    consumer_label: &'static str,
    capability_hash_ref: &'static str,
    target_ref: &'static str,
    workspace_scope_ref: &'static str,
    secret_class: SecretClass,
    reference_mode: SecretReferenceMode,
    projection_mode: HandleProjectionModeClass,
    vault_adapter: VaultAdapterClass,
    signature_state: VaultSignatureStateClass,
    adapter_label: &'static str,
    vault_entry_ref: &'static str,
    signer_id: &'static str,
    signed_at: &'static str,
    fetched_at: &'static str,
    valid_until: &'static str,
    signature_blob_ref: &'static str,
    authority_alias_ref: &'static str,
    credential_handle_ref: Option<&'static str>,
    session_ref: Option<&'static str>,
    delegated_credential_ref: Option<&'static str>,
    approval_ticket_ref: &'static str,
    revocation_path_label: &'static str,
    lifecycle_state: HandleLifecycleStateClass,
    minted_at: &'static str,
}

fn handle_row(seed: HandleRowSeed) -> SecretBrokerBetaHandleRow {
    SecretBrokerBetaHandleRow {
        record_kind: SECRET_BROKER_BETA_HANDLE_ROW_RECORD_KIND.to_owned(),
        schema_version: SECRET_BROKER_BETA_SCHEMA_VERSION,
        shared_contract_ref: SECRET_BROKER_BETA_SHARED_CONTRACT_REF.to_owned(),
        secret_broker_row_id: seed.row_id.to_owned(),
        display_label: seed.display_label.to_owned(),
        profile: seed.profile,
        profile_token: seed.profile.as_str().to_owned(),
        consumer: SecretConsumerIdentity {
            consumer_id: seed.consumer_id.to_owned(),
            consumer_label: seed.consumer_label.to_owned(),
            capability_hash_ref: seed.capability_hash_ref.to_owned(),
        },
        target_ref: seed.target_ref.to_owned(),
        workspace_scope_ref: seed.workspace_scope_ref.to_owned(),
        secret_class: seed.secret_class,
        secret_class_token: seed.secret_class.as_str().to_owned(),
        reference_mode: seed.reference_mode,
        reference_mode_token: seed.reference_mode.as_str().to_owned(),
        projection_mode: seed.projection_mode,
        projection_mode_token: seed.projection_mode.as_str().to_owned(),
        vault_binding: VaultBinding {
            vault_adapter: seed.vault_adapter,
            vault_adapter_token: seed.vault_adapter.as_str().to_owned(),
            adapter_label: seed.adapter_label.to_owned(),
            vault_entry_ref: seed.vault_entry_ref.to_owned(),
            signature_state: seed.signature_state,
            signature_state_token: seed.signature_state.as_str().to_owned(),
            signer_id: seed.signer_id.to_owned(),
            signed_at: seed.signed_at.to_owned(),
            fetched_at: seed.fetched_at.to_owned(),
            valid_until: seed.valid_until.to_owned(),
            signature_blob_ref: seed.signature_blob_ref.to_owned(),
        },
        lifecycle_state: seed.lifecycle_state,
        lifecycle_state_token: seed.lifecycle_state.as_str().to_owned(),
        authority_alias_ref: seed.authority_alias_ref.to_owned(),
        credential_handle_ref: seed.credential_handle_ref.map(String::from),
        session_ref: seed.session_ref.map(String::from),
        delegated_credential_ref: seed.delegated_credential_ref.map(String::from),
        approval_ticket_ref: seed.approval_ticket_ref.to_owned(),
        revocation_path_label: seed.revocation_path_label.to_owned(),
        minted_at: seed.minted_at.to_owned(),
        raw_secret_material_present: false,
        plaintext_persistence_allowed: false,
        silent_in_memory_promotion_allowed: false,
        stale_handle_reuse_allowed: false,
        public_endpoint_fallback_allowed: false,
    }
}

struct AuditEventSeed<'a> {
    audit_event_id: &'a str,
    row: &'a SecretBrokerBetaHandleRow,
    outcome: ConsumerAuditOutcomeClass,
    denial_note: Option<String>,
    requested_at: &'a str,
}

fn consumer_audit_event(seed: AuditEventSeed<'_>) -> SecretConsumerAuditEvent {
    let granted_reference_mode = seed.outcome.implied_reference_mode();
    SecretConsumerAuditEvent {
        record_kind: SECRET_BROKER_BETA_CONSUMER_AUDIT_RECORD_KIND.to_owned(),
        schema_version: SECRET_BROKER_BETA_SCHEMA_VERSION,
        shared_contract_ref: SECRET_BROKER_BETA_SHARED_CONTRACT_REF.to_owned(),
        audit_event_id: seed.audit_event_id.to_owned(),
        secret_broker_row_ref: seed.row.secret_broker_row_id.clone(),
        profile: seed.row.profile,
        profile_token: seed.row.profile.as_str().to_owned(),
        consumer: seed.row.consumer.clone(),
        target_ref: seed.row.target_ref.clone(),
        workspace_scope_ref: seed.row.workspace_scope_ref.clone(),
        secret_class: seed.row.secret_class,
        secret_class_token: seed.row.secret_class.as_str().to_owned(),
        projection_mode: seed.row.projection_mode,
        projection_mode_token: seed.row.projection_mode.as_str().to_owned(),
        outcome: seed.outcome,
        outcome_token: seed.outcome.as_str().to_owned(),
        granted_reference_mode,
        granted_reference_mode_token: granted_reference_mode.map(|mode| mode.as_str().to_owned()),
        denial_note: seed.denial_note,
        requested_at: seed.requested_at.to_owned(),
        raw_secret_material_present: false,
        raw_handle_id_exposed: false,
        no_public_endpoint_fallback: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_with_zero_defects() {
        let page = seeded_secret_broker_beta_page();
        validate_secret_broker_beta_page(&page).expect("seeded page validates");
        assert!(page.defects.is_empty());
        assert!(page.handle_rows.len() >= 4);
        for profile in SecretBrokerBetaProfileClass::ALL {
            assert!(page
                .summary
                .profiles_present
                .iter()
                .any(|token| token == profile.as_str()));
        }
    }

    #[test]
    fn seeded_page_covers_handle_delegated_and_session_modes() {
        let page = seeded_secret_broker_beta_page();
        let modes: BTreeSet<&str> = page
            .handle_rows
            .iter()
            .map(|row| row.reference_mode_token.as_str())
            .collect();
        assert!(modes.contains("handle"));
        assert!(modes.contains("delegated"));
        assert!(modes.contains("session_only"));
    }

    #[test]
    fn seeded_page_records_consumer_audit_for_every_row() {
        let page = seeded_secret_broker_beta_page();
        for row in &page.handle_rows {
            let count = page
                .consumer_audit
                .iter()
                .filter(|event| event.secret_broker_row_ref == row.secret_broker_row_id)
                .count();
            assert!(
                count >= 1,
                "row {} missing consumer audit events",
                row.secret_broker_row_id
            );
        }
        assert!(page
            .summary
            .audit_outcomes_present
            .iter()
            .any(|token| token == "denied_by_plaintext_requested"));
        assert!(page
            .summary
            .audit_outcomes_present
            .iter()
            .any(|token| token == "denied_by_public_endpoint_fallback"));
    }

    #[test]
    fn validator_flags_raw_secret_material() {
        let mut page = seeded_secret_broker_beta_page();
        page.handle_rows[0].raw_secret_material_present = true;
        let defects = audit_secret_broker_beta_page(&page.handle_rows, &page.consumer_audit);
        assert!(defects.iter().any(
            |defect| defect.defect_kind == SecretBrokerBetaDefectKind::RawSecretMaterialPresent
        ));
    }

    #[test]
    fn validator_flags_managed_authority_missing_signature() {
        let mut page = seeded_secret_broker_beta_page();
        let row = page
            .handle_rows
            .iter_mut()
            .find(|row| row.vault_binding.vault_adapter.is_managed_authority())
            .expect("seeded managed-authority row");
        row.vault_binding.signature_blob_ref.clear();
        row.vault_binding.signature_state = VaultSignatureStateClass::NotRequiredLocalOrigin;
        row.vault_binding.signature_state_token =
            row.vault_binding.signature_state.as_str().to_owned();
        let defects = audit_secret_broker_beta_page(&page.handle_rows, &page.consumer_audit);
        assert!(defects.iter().any(|defect| defect.defect_kind
            == SecretBrokerBetaDefectKind::ManagedAuthorityMissingSignature));
    }

    #[test]
    fn validator_flags_consumer_audit_missing() {
        let mut page = seeded_secret_broker_beta_page();
        let removed_row_id = page.handle_rows[0].secret_broker_row_id.clone();
        page.consumer_audit
            .retain(|event| event.secret_broker_row_ref != removed_row_id);
        let defects = audit_secret_broker_beta_page(&page.handle_rows, &page.consumer_audit);
        assert!(defects.iter().any(|defect| defect.defect_kind
            == SecretBrokerBetaDefectKind::ConsumerAuditMissing
            && defect.subject_id == removed_row_id));
    }

    #[test]
    fn validator_flags_consumer_lineage_drift() {
        let mut page = seeded_secret_broker_beta_page();
        page.consumer_audit[0].consumer.consumer_id = "consumer:unrelated".to_owned();
        let defects = audit_secret_broker_beta_page(&page.handle_rows, &page.consumer_audit);
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind == SecretBrokerBetaDefectKind::ConsumerLineageDrift));
    }

    #[test]
    fn validator_flags_denied_audit_missing_reason() {
        let mut page = seeded_secret_broker_beta_page();
        let event = page
            .consumer_audit
            .iter_mut()
            .find(|event| event.outcome.is_denial())
            .expect("seeded denial event");
        event.denial_note = None;
        let defects = audit_secret_broker_beta_page(&page.handle_rows, &page.consumer_audit);
        assert!(defects.iter().any(
            |defect| defect.defect_kind == SecretBrokerBetaDefectKind::DeniedAuditMissingReason
        ));
    }

    #[test]
    fn validator_flags_profile_coverage_missing() {
        let mut page = seeded_secret_broker_beta_page();
        page.handle_rows
            .retain(|row| row.profile != SecretBrokerBetaProfileClass::Offline);
        page.consumer_audit
            .retain(|event| event.profile != SecretBrokerBetaProfileClass::Offline);
        let defects = audit_secret_broker_beta_page(&page.handle_rows, &page.consumer_audit);
        assert!(defects.iter().any(|defect| defect.defect_kind
            == SecretBrokerBetaDefectKind::ProfileCoverageMissing
            && defect.note.contains("offline")));
    }

    #[test]
    fn support_export_round_trip_is_metadata_safe() {
        let page = seeded_secret_broker_beta_page();
        let export = SecretBrokerBetaSupportExport::from_page(
            "secret-broker-beta:support-export:001",
            "2026-05-16T05:00:00Z",
            page,
        );
        assert!(export.raw_secret_values_excluded);
        assert!(export.raw_handle_ids_excluded);
        assert!(export.consumer_lineage_preserved);
        assert!(export.defect_kinds_present.is_empty());
    }

    #[test]
    fn summary_counts_match_records() {
        let page = seeded_secret_broker_beta_page();
        assert_eq!(page.summary.handle_row_count, page.handle_rows.len());
        assert_eq!(page.summary.consumer_audit_count, page.consumer_audit.len());
        assert_eq!(page.summary.defect_count, 0);
        let grants: usize = page.summary.audit_grants_by_reference_mode.values().sum();
        let denials: usize = page.summary.audit_denials_by_outcome.values().sum();
        assert_eq!(grants + denials, page.consumer_audit.len());
    }
}
