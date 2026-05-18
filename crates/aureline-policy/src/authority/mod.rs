//! Runtime authority tickets for credential projection, privileged attach,
//! and root-authority changes.
//!
//! The module provides a beta contract for authority tickets that must survive
//! support export and policy replay without carrying raw credentials, raw
//! policy payloads, or ambient grants. It validates ticket lineage against the
//! current actor, target, sandbox, policy epoch, and authority source before a
//! high-risk action is admitted.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Schema version exported with authority-ticket beta records.
pub const AUTHORITY_TICKET_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by authority-ticket records and fixtures.
pub const AUTHORITY_TICKET_SHARED_CONTRACT_REF: &str = "security:authority_ticket_beta:v1";

/// Source matrix ref consumed by this projection.
pub const AUTHORITY_TICKET_SOURCE_MATRIX_REF: &str =
    "artifacts/security/m3/authority_ticket/authority_ticket_matrix.yaml";

/// Stable record kind for [`AuthorityTicketPage`] payloads.
pub const AUTHORITY_TICKET_PAGE_RECORD_KIND: &str = "security_authority_ticket_page_record";

/// Stable record kind for [`AuthorityTicketRecord`] payloads.
pub const AUTHORITY_TICKET_RECORD_KIND: &str = "security_authority_ticket_record";

/// Stable record kind for [`CredentialProjectionRecord`] payloads.
pub const CREDENTIAL_PROJECTION_RECORD_KIND: &str = "security_credential_projection_record";

/// Stable record kind for [`RootAuthorityChangeRecord`] payloads.
pub const ROOT_AUTHORITY_CHANGE_RECORD_KIND: &str = "security_root_authority_change_record";

/// Stable record kind for [`AuthorityTicketSpendAttempt`] payloads.
pub const AUTHORITY_TICKET_SPEND_ATTEMPT_RECORD_KIND: &str =
    "security_authority_ticket_spend_attempt_record";

/// Stable record kind for [`AuthorityTicketDefect`] payloads.
pub const AUTHORITY_TICKET_DEFECT_RECORD_KIND: &str = "security_authority_ticket_defect_record";

/// Stable record kind for [`AuthorityTicketSummary`] payloads.
pub const AUTHORITY_TICKET_SUMMARY_RECORD_KIND: &str = "security_authority_ticket_summary_record";

/// Stable record kind for [`AuthorityTicketSupportExport`] payloads.
pub const AUTHORITY_TICKET_SUPPORT_EXPORT_RECORD_KIND: &str =
    "security_authority_ticket_support_export_record";

/// Ticket class bound to one authority family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityTicketClass {
    /// Local workspace mutation or destructive local repair.
    LocalMutation,
    /// External provider mutation or remote provider side effect.
    ExternalProviderMutation,
    /// Credential projection or secret use through a broker.
    CredentialProjection,
    /// Debug attach, privileged inspection, deep capture, or profiler attach.
    PrivilegedDebugAttach,
    /// Policy, trust-store, admin, or governance mutation.
    PolicyTrustAdminChange,
}

impl AuthorityTicketClass {
    /// All required beta ticket classes in canonical order.
    pub const ALL: [Self; 5] = [
        Self::LocalMutation,
        Self::ExternalProviderMutation,
        Self::CredentialProjection,
        Self::PrivilegedDebugAttach,
        Self::PolicyTrustAdminChange,
    ];

    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalMutation => "local_mutation",
            Self::ExternalProviderMutation => "external_provider_mutation",
            Self::CredentialProjection => "credential_projection",
            Self::PrivilegedDebugAttach => "privileged_debug_attach",
            Self::PolicyTrustAdminChange => "policy_trust_admin_change",
        }
    }

    /// Maximum seeded ticket lifetime in seconds.
    pub const fn max_lifetime_seconds(self) -> u64 {
        match self {
            Self::LocalMutation => 900,
            Self::ExternalProviderMutation => 600,
            Self::CredentialProjection => 180,
            Self::PrivilegedDebugAttach => 300,
            Self::PolicyTrustAdminChange => 300,
        }
    }

    /// True when a stale or denied spend must be explicitly reapproved before
    /// replay.
    pub const fn requires_reapproval_on_drift(self) -> bool {
        matches!(
            self,
            Self::ExternalProviderMutation
                | Self::CredentialProjection
                | Self::PrivilegedDebugAttach
                | Self::PolicyTrustAdminChange
        )
    }
}

/// Side-effect class bound by a ticket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthoritySideEffectClass {
    /// Local workspace write or repair.
    LocalWorkspaceWrite,
    /// External provider write.
    ExternalProviderWrite,
    /// Credential projection to a named consumer.
    CredentialProjectionToConsumer,
    /// Privileged debug or inspection attach.
    PrivilegedInspectionAttach,
    /// Policy, trust, or admin mutation.
    PolicyTrustMutation,
}

impl AuthoritySideEffectClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkspaceWrite => "local_workspace_write",
            Self::ExternalProviderWrite => "external_provider_write",
            Self::CredentialProjectionToConsumer => "credential_projection_to_consumer",
            Self::PrivilegedInspectionAttach => "privileged_inspection_attach",
            Self::PolicyTrustMutation => "policy_trust_mutation",
        }
    }

    /// Ticket class required for this side-effect class.
    pub const fn required_ticket_class(self) -> AuthorityTicketClass {
        match self {
            Self::LocalWorkspaceWrite => AuthorityTicketClass::LocalMutation,
            Self::ExternalProviderWrite => AuthorityTicketClass::ExternalProviderMutation,
            Self::CredentialProjectionToConsumer => AuthorityTicketClass::CredentialProjection,
            Self::PrivilegedInspectionAttach => AuthorityTicketClass::PrivilegedDebugAttach,
            Self::PolicyTrustMutation => AuthorityTicketClass::PolicyTrustAdminChange,
        }
    }
}

/// Issuer class allowed to mint a ticket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityIssuerClass {
    /// Desktop shell or native approval surface.
    Shell,
    /// Policy service or policy-pack verifier.
    PolicyService,
    /// Supervisor control path.
    Supervisor,
}

impl AuthorityIssuerClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::PolicyService => "policy_service",
            Self::Supervisor => "supervisor",
        }
    }
}

/// Request origin that asked for authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityRequestOriginClass {
    /// User-approved shell prompt.
    UserShellPrompt,
    /// Policy decision or signed policy evaluation.
    PolicyDecision,
    /// Supervisor control path.
    SupervisorControlPath,
    /// Admin console requested the ticket through a policy or supervisor lane.
    AdminConsoleRequest,
    /// Local administrator requested the ticket through a shell or supervisor lane.
    LocalAdminRequest,
    /// AI tool plan requested authority through an issuer.
    AiToolPlan,
    /// Extension requested authority through an issuer.
    ExtensionRequest,
    /// CLI script requested authority through an issuer.
    CliScriptRequest,
    /// Browser companion requested authority through an issuer.
    BrowserCompanionRequest,
    /// Remote helper requested authority through an issuer.
    RemoteHelperRequest,
    /// Automation scheduler requested authority through an issuer.
    AutomationSchedulerRequest,
}

impl AuthorityRequestOriginClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserShellPrompt => "user_shell_prompt",
            Self::PolicyDecision => "policy_decision",
            Self::SupervisorControlPath => "supervisor_control_path",
            Self::AdminConsoleRequest => "admin_console_request",
            Self::LocalAdminRequest => "local_admin_request",
            Self::AiToolPlan => "ai_tool_plan",
            Self::ExtensionRequest => "extension_request",
            Self::CliScriptRequest => "cli_script_request",
            Self::BrowserCompanionRequest => "browser_companion_request",
            Self::RemoteHelperRequest => "remote_helper_request",
            Self::AutomationSchedulerRequest => "automation_scheduler_request",
        }
    }

    /// True when the request origin is one of the issuer seats.
    pub const fn is_intrinsic_issuer(self) -> bool {
        matches!(
            self,
            Self::UserShellPrompt | Self::PolicyDecision | Self::SupervisorControlPath
        )
    }
}

/// Actor class bound to a ticket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityActorClass {
    /// Human user account.
    HumanAccount,
    /// Local administrator.
    LocalAdmin,
    /// Organization administrator.
    OrganizationAdmin,
    /// Installation or application grant.
    InstallationOrAppGrant,
    /// Delegated credential.
    DelegatedCredential,
    /// Policy-injected service identity.
    PolicyInjectedServiceIdentity,
    /// Local-only authority.
    LocalOnlyAuthority,
}

impl AuthorityActorClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HumanAccount => "human_account",
            Self::LocalAdmin => "local_admin",
            Self::OrganizationAdmin => "organization_admin",
            Self::InstallationOrAppGrant => "installation_or_app_grant",
            Self::DelegatedCredential => "delegated_credential",
            Self::PolicyInjectedServiceIdentity => "policy_injected_service_identity",
            Self::LocalOnlyAuthority => "local_only_authority",
        }
    }
}

/// Target class bound to a ticket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityTargetClass {
    /// Local workspace object.
    LocalWorkspace,
    /// External provider object.
    ProviderObject,
    /// Credential consumer.
    CredentialConsumer,
    /// Debug attach or privileged-inspection target.
    DebugAttachTarget,
    /// Trust store or signing root.
    TrustStore,
    /// Policy or admin control object.
    PolicyAdminObject,
}

impl AuthorityTargetClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkspace => "local_workspace",
            Self::ProviderObject => "provider_object",
            Self::CredentialConsumer => "credential_consumer",
            Self::DebugAttachTarget => "debug_attach_target",
            Self::TrustStore => "trust_store",
            Self::PolicyAdminObject => "policy_admin_object",
        }
    }
}

/// Credential projection mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialProjectionMode {
    /// Broker-only handle reference.
    HandleOnly,
    /// Delegated handle scoped to one consumer.
    DelegatedHandle,
    /// Session-only secret reference.
    SessionOnlySecret,
    /// Broker signs request headers without exposing raw material.
    RequestHeaderSigner,
    /// Broker signs challenges or exchanges tokens.
    SignOnly,
    /// Broker exchanges a source handle for a narrower operation token.
    TokenExchange,
    /// Broker lends an ephemeral file descriptor.
    EphemeralFileDescriptor,
    /// Consumer may inspect metadata only.
    MetadataOnly,
}

impl CredentialProjectionMode {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HandleOnly => "handle_only",
            Self::DelegatedHandle => "delegated_handle",
            Self::SessionOnlySecret => "session_only_secret",
            Self::RequestHeaderSigner => "request_header_signer",
            Self::SignOnly => "sign_only",
            Self::TokenExchange => "token_exchange",
            Self::EphemeralFileDescriptor => "ephemeral_file_descriptor",
            Self::MetadataOnly => "metadata_only",
        }
    }
}

/// Credential reference class projected by the broker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialReferenceClass {
    /// Broker handle.
    BrokerHandle,
    /// Delegated credential.
    DelegatedCredential,
    /// Session-only secret.
    SessionOnlySecret,
    /// Projected credential token.
    ProjectedCredential,
}

impl CredentialReferenceClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BrokerHandle => "broker_handle",
            Self::DelegatedCredential => "delegated_credential",
            Self::SessionOnlySecret => "session_only_secret",
            Self::ProjectedCredential => "projected_credential",
        }
    }
}

/// Root-authority change class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RootAuthorityChangeClass {
    /// Policy-admin grant, revoke, or scope change.
    PolicyAdminChange,
    /// Trust-store import, pin, or removal.
    TrustStoreChange,
    /// Signing-root rotation or supersede.
    SigningRootRotation,
    /// Elevated governance or capability-surface mutation.
    GovernanceElevation,
}

impl RootAuthorityChangeClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyAdminChange => "policy_admin_change",
            Self::TrustStoreChange => "trust_store_change",
            Self::SigningRootRotation => "signing_root_rotation",
            Self::GovernanceElevation => "governance_elevation",
        }
    }
}

/// Source proof class for root-authority changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthoritySourceProofClass {
    /// Signed policy bundle.
    SignedPolicyBundle,
    /// Signed trust-root rotation packet.
    SignedTrustRootRotation,
    /// Signed admin command packet.
    SignedAdminCommand,
    /// Local root authority accepted through platform policy.
    LocalRootAuthority,
    /// Hardware-backed admin step-up.
    HardwareBackedAdminStepUp,
    /// Missing or unverifiable proof.
    MissingOrUnverified,
}

impl AuthoritySourceProofClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedPolicyBundle => "signed_policy_bundle",
            Self::SignedTrustRootRotation => "signed_trust_root_rotation",
            Self::SignedAdminCommand => "signed_admin_command",
            Self::LocalRootAuthority => "local_root_authority",
            Self::HardwareBackedAdminStepUp => "hardware_backed_admin_step_up",
            Self::MissingOrUnverified => "missing_or_unverified",
        }
    }

    /// True when this proof can authorize a root-authority change.
    pub const fn is_authoritative(self) -> bool {
        !matches!(self, Self::MissingOrUnverified)
    }

    const fn requires_signature(self) -> bool {
        matches!(
            self,
            Self::SignedPolicyBundle | Self::SignedTrustRootRotation | Self::SignedAdminCommand
        )
    }

    const fn requires_local_authority_ref(self) -> bool {
        matches!(
            self,
            Self::LocalRootAuthority | Self::HardwareBackedAdminStepUp
        )
    }
}

/// Use posture for an issued ticket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityUsePosture {
    /// Single-use ticket.
    SingleUse,
    /// Bounded reuse ticket with an external counter.
    BoundedReuse,
}

impl AuthorityUsePosture {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleUse => "single_use",
            Self::BoundedReuse => "bounded_reuse",
        }
    }
}

/// Revocation state for a ticket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityRevocationState {
    /// Ticket is live and unrevoked.
    Live,
    /// Ticket expired.
    Expired,
    /// Ticket was revoked by user, admin, or system policy.
    Revoked,
    /// Ticket was invalidated by drift or lineage break.
    Invalidated,
}

impl AuthorityRevocationState {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
            Self::Invalidated => "invalidated",
        }
    }

    /// True when a ticket in this state may be spent.
    pub const fn is_spendable(self) -> bool {
        matches!(self, Self::Live)
    }
}

/// Spend evaluation outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityEvaluationOutcome {
    /// Ticket admitted the spend.
    Admitted,
    /// No matching ticket was presented.
    DeniedMissingTicket,
    /// Ticket was expired.
    DeniedExpired,
    /// Ticket was revoked or invalidated.
    DeniedRevoked,
    /// Target identity drifted.
    DeniedTargetDrift,
    /// Policy epoch drifted.
    DeniedPolicyEpochDrift,
    /// Sandbox or capability envelope drifted.
    DeniedSandboxDrift,
    /// Authority source mismatched the ticket lineage.
    DeniedAuthoritySourceMismatch,
    /// Credential projection record was missing or unsafe.
    DeniedCredentialProjectionMissing,
    /// Root proof was missing or not authoritative.
    DeniedRootProofMissing,
    /// Ticket lineage was missing or mismatched.
    DeniedLineageMismatch,
}

impl AuthorityEvaluationOutcome {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::DeniedMissingTicket => "denied_missing_ticket",
            Self::DeniedExpired => "denied_expired",
            Self::DeniedRevoked => "denied_revoked",
            Self::DeniedTargetDrift => "denied_target_drift",
            Self::DeniedPolicyEpochDrift => "denied_policy_epoch_drift",
            Self::DeniedSandboxDrift => "denied_sandbox_drift",
            Self::DeniedAuthoritySourceMismatch => "denied_authority_source_mismatch",
            Self::DeniedCredentialProjectionMissing => "denied_credential_projection_missing",
            Self::DeniedRootProofMissing => "denied_root_proof_missing",
            Self::DeniedLineageMismatch => "denied_lineage_mismatch",
        }
    }

    /// True when this outcome admits the spend.
    pub const fn is_admitted(self) -> bool {
        matches!(self, Self::Admitted)
    }
}

/// Actor binding shared by tickets and spend attempts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityActorBinding {
    /// Actor class.
    pub actor_class: AuthorityActorClass,
    /// Stable token for [`Self::actor_class`].
    pub actor_class_token: String,
    /// Opaque actor subject ref.
    pub actor_subject_ref: String,
    /// Opaque source of authority for this actor.
    pub authority_source_ref: String,
    /// Opaque granted scope refs.
    pub granted_scope_refs: Vec<String>,
}

/// Target identity bound by a ticket.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityTargetIdentity {
    /// Target class.
    pub target_class: AuthorityTargetClass,
    /// Stable token for [`Self::target_class`].
    pub target_class_token: String,
    /// Opaque target ref.
    pub target_ref: String,
    /// Reviewable target label.
    pub target_label: String,
    /// Opaque target fingerprint ref.
    pub target_fingerprint_ref: String,
}

/// Sandbox and policy binding for a ticket.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoritySandboxBinding {
    /// Opaque sandbox-profile ref.
    pub sandbox_profile_ref: String,
    /// Opaque sandbox-profile fingerprint ref.
    pub sandbox_profile_fingerprint_ref: String,
    /// Opaque capability-envelope ref.
    pub capability_envelope_ref: String,
    /// Opaque policy-epoch ref.
    pub policy_epoch_ref: String,
}

/// Revocation hooks carried by a ticket.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityRevocationHook {
    /// Revocation state.
    pub revocation_state: AuthorityRevocationState,
    /// Stable token for [`Self::revocation_state`].
    pub revocation_state_token: String,
    /// Opaque revocation epoch ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoke_epoch_ref: Option<String>,
    /// Opaque revoke event ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoke_event_ref: Option<String>,
    /// Opaque revoke path ref safe for support export.
    pub revoke_path_ref: String,
}

/// Narrow remembered rule that can mint fresh short-lived tickets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RememberedAuthorityRule {
    /// Stable reusable rule id.
    pub reusable_rule_id: String,
    /// Narrow rule scope.
    pub scope_ref: String,
    /// Maximum renewed ticket lifetime in seconds.
    pub renewable_ticket_lifetime_seconds: u64,
    /// Opaque owner or policy source ref.
    pub owner_policy_source_ref: String,
    /// Opaque revoke path ref.
    pub revoke_path_ref: String,
}

/// Ticket lineage block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityLineage {
    /// Predecessor ticket refs.
    pub parent_ticket_refs: Vec<String>,
    /// Authority source ref used at issue.
    pub authority_source_ref: String,
    /// Opaque lineage fingerprint ref.
    pub lineage_fingerprint_ref: String,
}

/// Guardrails carried by every authority record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityGuardrails {
    /// True when raw authority material was present.
    pub raw_authority_material_present: bool,
    /// True when plaintext secret material was present.
    pub plaintext_secret_present: bool,
    /// True when an ambient grant was accepted.
    pub ambient_authority_grant_present: bool,
    /// True when authority was silently widened.
    pub silent_authority_widening_present: bool,
    /// True when local editing was preserved after denial.
    pub local_editing_preserved: bool,
}

impl AuthorityGuardrails {
    fn clean() -> Self {
        Self {
            raw_authority_material_present: false,
            plaintext_secret_present: false,
            ambient_authority_grant_present: false,
            silent_authority_widening_present: false,
            local_editing_preserved: true,
        }
    }
}

/// Consumer identity for credential projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialConsumerIdentity {
    /// Opaque consumer id.
    pub consumer_id: String,
    /// Reviewable consumer label.
    pub consumer_label: String,
    /// Opaque consumer capability hash ref.
    pub consumer_capability_hash_ref: String,
}

/// Credential projection record bound to a ticket.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialProjectionRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable projection id.
    pub projection_id: String,
    /// Ticket ref that admitted this projection.
    pub ticket_ref: String,
    /// Projection mode.
    pub projection_mode: CredentialProjectionMode,
    /// Stable token for [`Self::projection_mode`].
    pub projection_mode_token: String,
    /// Credential reference class.
    pub credential_reference_class: CredentialReferenceClass,
    /// Stable token for [`Self::credential_reference_class`].
    pub credential_reference_class_token: String,
    /// Consumer identity receiving the projection.
    pub consumer_identity: CredentialConsumerIdentity,
    /// Opaque secret class ref.
    pub secret_class_ref: String,
    /// Opaque projected credential ref.
    pub projected_credential_ref: String,
    /// Opaque target identity ref.
    pub target_identity_ref: String,
    /// Projection expiry timestamp.
    pub expires_at: String,
    /// Opaque revocation path ref.
    pub revocation_path_ref: String,
    /// Guardrails for the projection row.
    pub guardrails: AuthorityGuardrails,
}

/// Source proof for root-authority changes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoritySourceProof {
    /// Proof class.
    pub proof_class: AuthoritySourceProofClass,
    /// Stable token for [`Self::proof_class`].
    pub proof_class_token: String,
    /// Opaque source ref.
    pub source_ref: String,
    /// Opaque signer ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signer_ref: Option<String>,
    /// Opaque signature blob ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_blob_ref: Option<String>,
    /// Opaque local-authority ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_authority_ref: Option<String>,
    /// Opaque proof fingerprint ref.
    pub proof_fingerprint_ref: String,
    /// Verification timestamp.
    pub verified_at: String,
}

/// Root-authority change record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RootAuthorityChangeRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable change id.
    pub change_id: String,
    /// Ticket ref that admitted this change.
    pub ticket_ref: String,
    /// Root-authority change class.
    pub change_class: RootAuthorityChangeClass,
    /// Stable token for [`Self::change_class`].
    pub change_class_token: String,
    /// Source proof that makes the change authoritative.
    pub source_proof: AuthoritySourceProof,
    /// Admin actor responsible for the change.
    pub admin_actor: AuthorityActorBinding,
    /// Target identity for the root-authority change.
    pub target_identity: AuthorityTargetIdentity,
    /// Policy epoch under which the change applies.
    pub policy_epoch_ref: String,
    /// Timestamp at which the change becomes effective.
    pub effective_at: String,
    /// Opaque rollback ref.
    pub rollback_ref: String,
    /// Opaque audit-event refs.
    pub audit_event_refs: Vec<String>,
    /// True when support/admin export can reconstruct this change.
    pub exportable: bool,
}

/// One issued authority ticket.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityTicketRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable ticket id.
    pub ticket_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Ticket class.
    pub ticket_class: AuthorityTicketClass,
    /// Stable token for [`Self::ticket_class`].
    pub ticket_class_token: String,
    /// Side-effect class.
    pub side_effect_class: AuthoritySideEffectClass,
    /// Stable token for [`Self::side_effect_class`].
    pub side_effect_class_token: String,
    /// Issuer class.
    pub issuer_class: AuthorityIssuerClass,
    /// Stable token for [`Self::issuer_class`].
    pub issuer_class_token: String,
    /// Opaque issuing-surface ref.
    pub issuing_surface_ref: String,
    /// Request-origin class.
    pub request_origin_class: AuthorityRequestOriginClass,
    /// Stable token for [`Self::request_origin_class`].
    pub request_origin_class_token: String,
    /// Optional requesting-surface ref for non-issuer origins.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requesting_surface_ref: Option<String>,
    /// Actor binding.
    pub actor_binding: AuthorityActorBinding,
    /// Target identity.
    pub target_identity: AuthorityTargetIdentity,
    /// Sandbox and policy binding.
    pub sandbox_binding: AuthoritySandboxBinding,
    /// Timestamp at which the ticket was issued.
    pub issued_at: String,
    /// Timestamp at which the ticket expires.
    pub expires_at: String,
    /// Ticket lifetime in seconds.
    pub lifetime_seconds: u64,
    /// Use posture.
    pub use_posture: AuthorityUsePosture,
    /// Stable token for [`Self::use_posture`].
    pub use_posture_token: String,
    /// Revocation hook.
    pub revocation_hook: AuthorityRevocationHook,
    /// Optional credential projection ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_projection_ref: Option<String>,
    /// Optional root-authority change ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_authority_change_ref: Option<String>,
    /// Optional source proof for root-authority tickets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_proof: Option<AuthoritySourceProof>,
    /// Optional remembered-rule scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remembered_rule: Option<RememberedAuthorityRule>,
    /// Lineage and authority-source binding.
    pub lineage: AuthorityLineage,
    /// Opaque audit-event refs.
    pub audit_event_refs: Vec<String>,
    /// Guardrails for the ticket.
    pub guardrails: AuthorityGuardrails,
}

/// One spend attempt against current authority context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityTicketSpendAttempt {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable spend-attempt id.
    pub spend_attempt_id: String,
    /// Attempted ticket class.
    pub attempted_ticket_class: AuthorityTicketClass,
    /// Stable token for [`Self::attempted_ticket_class`].
    pub attempted_ticket_class_token: String,
    /// Optional presented ticket ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presented_ticket_ref: Option<String>,
    /// Current actor binding.
    pub current_actor_binding: AuthorityActorBinding,
    /// Current target identity.
    pub current_target_identity: AuthorityTargetIdentity,
    /// Current sandbox and policy binding.
    pub current_sandbox_binding: AuthoritySandboxBinding,
    /// Current authority source ref.
    pub current_authority_source_ref: String,
    /// Timestamp at which the spend was evaluated.
    pub evaluated_at: String,
    /// Evaluation outcome.
    pub evaluation_outcome: AuthorityEvaluationOutcome,
    /// Stable token for [`Self::evaluation_outcome`].
    pub evaluation_outcome_token: String,
    /// True when user/admin reapproval is required before replay.
    pub reapproval_required: bool,
    /// True when the failed action can be safely replayed without reapproval.
    pub safely_replayable: bool,
    /// Export-safe explanation.
    pub explanation: String,
    /// Opaque audit-event refs.
    pub audit_event_refs: Vec<String>,
}

/// Defect-kind vocabulary surfaced by the validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityTicketDefectKind {
    /// A record kind or schema-version field drifted.
    RecordShapeDrift,
    /// A stable token did not match its enum field.
    TokenDrift,
    /// Side-effect class does not match the ticket class.
    SideEffectClassMismatch,
    /// A non-issuer origin omitted a requesting-surface ref.
    MissingRequestingSurfaceRef,
    /// Ticket lifetime does not match issued/expires timestamps.
    TicketLifetimeMismatch,
    /// Ticket lifetime exceeds the class budget.
    TicketLifetimeExceedsBudget,
    /// Credential projection ref is required but missing.
    CredentialProjectionMissing,
    /// Credential projection carries raw or plaintext secret material.
    RawSecretMaterialPresent,
    /// Credential projection consumer identity is incomplete.
    CredentialProjectionConsumerMissing,
    /// Root-authority change ref is required but missing.
    RootAuthorityChangeMissing,
    /// Root-authority proof is missing or not authoritative.
    RootAuthorityProofMissing,
    /// Root-authority proof lacks required signature or local proof refs.
    RootAuthorityProofIncomplete,
    /// Root-authority change is not exportable or lacks audit refs.
    RootAuthorityChangeNotExportable,
    /// Remembered rule is too broad or too long-lived.
    RememberedRuleTooBroad,
    /// Remembered rule is attached to a forbidden authority class.
    RememberedRuleForbidden,
    /// Spend event admitted without a current matching ticket.
    SpendAdmittedWithoutTicket,
    /// Spend event admitted under target, policy, sandbox, actor, or source drift.
    SpendAdmittedUnderDrift,
    /// Spend denial lacks an audit ref.
    SpendDenialMissingAuditRef,
    /// Spend denial for an unsafe replay does not require reapproval.
    SpendDenialMissingReapproval,
    /// Denial outcome did not match the changed binding dimension.
    SpendDenialOutcomeMismatch,
    /// One required ticket class is not represented.
    TicketClassCoverageMissing,
    /// One required credential projection mode family is not represented.
    CredentialProjectionCoverageMissing,
    /// One required privileged/admin spend outcome is not represented.
    SpendOutcomeCoverageMissing,
    /// Guardrails claim raw authority, ambient grants, or silent widening.
    GuardrailViolation,
}

impl AuthorityTicketDefectKind {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecordShapeDrift => "record_shape_drift",
            Self::TokenDrift => "token_drift",
            Self::SideEffectClassMismatch => "side_effect_class_mismatch",
            Self::MissingRequestingSurfaceRef => "missing_requesting_surface_ref",
            Self::TicketLifetimeMismatch => "ticket_lifetime_mismatch",
            Self::TicketLifetimeExceedsBudget => "ticket_lifetime_exceeds_budget",
            Self::CredentialProjectionMissing => "credential_projection_missing",
            Self::RawSecretMaterialPresent => "raw_secret_material_present",
            Self::CredentialProjectionConsumerMissing => "credential_projection_consumer_missing",
            Self::RootAuthorityChangeMissing => "root_authority_change_missing",
            Self::RootAuthorityProofMissing => "root_authority_proof_missing",
            Self::RootAuthorityProofIncomplete => "root_authority_proof_incomplete",
            Self::RootAuthorityChangeNotExportable => "root_authority_change_not_exportable",
            Self::RememberedRuleTooBroad => "remembered_rule_too_broad",
            Self::RememberedRuleForbidden => "remembered_rule_forbidden",
            Self::SpendAdmittedWithoutTicket => "spend_admitted_without_ticket",
            Self::SpendAdmittedUnderDrift => "spend_admitted_under_drift",
            Self::SpendDenialMissingAuditRef => "spend_denial_missing_audit_ref",
            Self::SpendDenialMissingReapproval => "spend_denial_missing_reapproval",
            Self::SpendDenialOutcomeMismatch => "spend_denial_outcome_mismatch",
            Self::TicketClassCoverageMissing => "ticket_class_coverage_missing",
            Self::CredentialProjectionCoverageMissing => "credential_projection_coverage_missing",
            Self::SpendOutcomeCoverageMissing => "spend_outcome_coverage_missing",
            Self::GuardrailViolation => "guardrail_violation",
        }
    }
}

/// Typed validation defect for the authority-ticket page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityTicketDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Defect kind.
    pub defect_kind: AuthorityTicketDefectKind,
    /// Stable token for [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Subject id.
    pub subject_id: String,
    /// Field that failed validation.
    pub field: String,
    /// Export-safe note.
    pub note: String,
}

impl AuthorityTicketDefect {
    fn new(
        defect_kind: AuthorityTicketDefectKind,
        subject_id: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: AUTHORITY_TICKET_DEFECT_RECORD_KIND.to_owned(),
            schema_version: AUTHORITY_TICKET_SCHEMA_VERSION,
            shared_contract_ref: AUTHORITY_TICKET_SHARED_CONTRACT_REF.to_owned(),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            subject_id: subject_id.into(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Summary for an authority-ticket page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityTicketSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Ticket count.
    pub ticket_count: usize,
    /// Credential projection count.
    pub credential_projection_count: usize,
    /// Root-authority change count.
    pub root_authority_change_count: usize,
    /// Spend-attempt count.
    pub spend_attempt_count: usize,
    /// Ticket-class tokens present.
    pub ticket_classes_present: Vec<String>,
    /// Credential projection mode tokens present.
    pub credential_projection_modes_present: Vec<String>,
    /// Root-authority change-class tokens present.
    pub root_authority_change_classes_present: Vec<String>,
    /// Spend attempts by outcome token.
    pub spend_attempts_by_outcome: BTreeMap<String, usize>,
    /// Defect count.
    pub defect_count: usize,
    /// Defect counts by kind.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
}

impl AuthorityTicketSummary {
    /// Builds a summary from authority records.
    pub fn from_records(
        tickets: &[AuthorityTicketRecord],
        credential_projections: &[CredentialProjectionRecord],
        root_authority_changes: &[RootAuthorityChangeRecord],
        spend_attempts: &[AuthorityTicketSpendAttempt],
        defects: &[AuthorityTicketDefect],
    ) -> Self {
        let ticket_classes_present = tickets
            .iter()
            .map(|ticket| ticket.ticket_class_token.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let credential_projection_modes_present = credential_projections
            .iter()
            .map(|projection| projection.projection_mode_token.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let root_authority_change_classes_present = root_authority_changes
            .iter()
            .map(|change| change.change_class_token.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let mut spend_attempts_by_outcome = BTreeMap::new();
        for spend in spend_attempts {
            *spend_attempts_by_outcome
                .entry(spend.evaluation_outcome_token.clone())
                .or_insert(0) += 1;
        }
        let mut defect_counts_by_kind = BTreeMap::new();
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind_token.clone())
                .or_insert(0) += 1;
        }
        Self {
            record_kind: AUTHORITY_TICKET_SUMMARY_RECORD_KIND.to_owned(),
            ticket_count: tickets.len(),
            credential_projection_count: credential_projections.len(),
            root_authority_change_count: root_authority_changes.len(),
            spend_attempt_count: spend_attempts.len(),
            ticket_classes_present,
            credential_projection_modes_present,
            root_authority_change_classes_present,
            spend_attempts_by_outcome,
            defect_count: defects.len(),
            defect_counts_by_kind,
        }
    }
}

/// Top-level authority-ticket page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityTicketPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Source matrix ref.
    pub source_matrix_ref: String,
    /// Issued authority tickets.
    pub tickets: Vec<AuthorityTicketRecord>,
    /// Credential projection rows.
    pub credential_projections: Vec<CredentialProjectionRecord>,
    /// Root-authority change rows.
    pub root_authority_changes: Vec<RootAuthorityChangeRecord>,
    /// Spend attempts.
    pub spend_attempts: Vec<AuthorityTicketSpendAttempt>,
    /// Typed validation defects.
    pub defects: Vec<AuthorityTicketDefect>,
    /// Aggregate summary.
    pub summary: AuthorityTicketSummary,
}

/// Support-export wrapper for the authority-ticket page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityTicketSupportExport {
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
    pub page: AuthorityTicketPage,
    /// True when privileged-action lineage is preserved.
    pub privileged_action_lineage_preserved: bool,
    /// True when raw credentials are excluded.
    pub raw_credentials_excluded: bool,
    /// True when root-authority proof refs are included.
    pub root_authority_proof_refs_preserved: bool,
    /// Reviewable redaction summary.
    pub redaction_summary: String,
}

impl AuthorityTicketSupportExport {
    /// Builds a support-export wrapper from a page.
    pub fn from_page(
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
        page: AuthorityTicketPage,
    ) -> Self {
        Self {
            record_kind: AUTHORITY_TICKET_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: AUTHORITY_TICKET_SCHEMA_VERSION,
            shared_contract_ref: AUTHORITY_TICKET_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            page,
            privileged_action_lineage_preserved: true,
            raw_credentials_excluded: true,
            root_authority_proof_refs_preserved: true,
            redaction_summary:
                "Metadata-only authority-ticket export: ticket ids, actor bindings, target \
                 fingerprints, sandbox/policy refs, credential projection modes, consumer \
                 identities, root-authority proof refs, spend outcomes, and audit refs are \
                 preserved; raw credentials, raw authority bodies, raw policy payloads, raw \
                 evidence bodies, and plaintext secret material are excluded."
                    .to_owned(),
        }
    }
}

/// Validates an authority-ticket page.
pub fn validate_authority_ticket_page(
    page: &AuthorityTicketPage,
) -> Result<(), Vec<AuthorityTicketDefect>> {
    let defects = audit_authority_ticket_page(
        &page.tickets,
        &page.credential_projections,
        &page.root_authority_changes,
        &page.spend_attempts,
    );
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Recomputes authority-ticket defects from records.
pub fn audit_authority_ticket_page(
    tickets: &[AuthorityTicketRecord],
    credential_projections: &[CredentialProjectionRecord],
    root_authority_changes: &[RootAuthorityChangeRecord],
    spend_attempts: &[AuthorityTicketSpendAttempt],
) -> Vec<AuthorityTicketDefect> {
    let mut defects = Vec::new();
    let projections_by_id: BTreeMap<&str, &CredentialProjectionRecord> = credential_projections
        .iter()
        .map(|projection| (projection.projection_id.as_str(), projection))
        .collect();
    let root_changes_by_id: BTreeMap<&str, &RootAuthorityChangeRecord> = root_authority_changes
        .iter()
        .map(|change| (change.change_id.as_str(), change))
        .collect();
    let tickets_by_id: BTreeMap<&str, &AuthorityTicketRecord> = tickets
        .iter()
        .map(|ticket| (ticket.ticket_id.as_str(), ticket))
        .collect();

    for projection in credential_projections {
        check_projection(&mut defects, projection);
    }
    for change in root_authority_changes {
        check_root_change(&mut defects, change);
    }
    for ticket in tickets {
        check_ticket(
            &mut defects,
            ticket,
            &projections_by_id,
            &root_changes_by_id,
        );
    }
    for spend in spend_attempts {
        check_spend(&mut defects, spend, &tickets_by_id, &projections_by_id);
    }

    let observed_ticket_classes = tickets
        .iter()
        .map(|ticket| ticket.ticket_class_token.as_str())
        .collect::<BTreeSet<_>>();
    for required in AuthorityTicketClass::ALL {
        if !observed_ticket_classes.contains(required.as_str()) {
            defects.push(AuthorityTicketDefect::new(
                AuthorityTicketDefectKind::TicketClassCoverageMissing,
                "page",
                "tickets",
                format!("missing {} ticket class", required.as_str()),
            ));
        }
    }

    for required_mode in [
        CredentialProjectionMode::DelegatedHandle,
        CredentialProjectionMode::SessionOnlySecret,
        CredentialProjectionMode::SignOnly,
    ] {
        if !credential_projections
            .iter()
            .any(|projection| projection.projection_mode == required_mode)
        {
            defects.push(AuthorityTicketDefect::new(
                AuthorityTicketDefectKind::CredentialProjectionCoverageMissing,
                "page",
                "credential_projections",
                format!(
                    "missing {} credential projection coverage",
                    required_mode.as_str()
                ),
            ));
        }
    }

    for outcome in [
        AuthorityEvaluationOutcome::DeniedMissingTicket,
        AuthorityEvaluationOutcome::DeniedPolicyEpochDrift,
        AuthorityEvaluationOutcome::DeniedAuthoritySourceMismatch,
    ] {
        if !spend_attempts
            .iter()
            .any(|spend| spend.evaluation_outcome == outcome)
        {
            defects.push(AuthorityTicketDefect::new(
                AuthorityTicketDefectKind::SpendOutcomeCoverageMissing,
                "page",
                "spend_attempts",
                format!("missing {} spend outcome coverage", outcome.as_str()),
            ));
        }
    }

    defects
}

fn check_projection(
    defects: &mut Vec<AuthorityTicketDefect>,
    projection: &CredentialProjectionRecord,
) {
    if projection.record_kind != CREDENTIAL_PROJECTION_RECORD_KIND
        || projection.schema_version != AUTHORITY_TICKET_SCHEMA_VERSION
        || projection.shared_contract_ref != AUTHORITY_TICKET_SHARED_CONTRACT_REF
    {
        defects.push(AuthorityTicketDefect::new(
            AuthorityTicketDefectKind::RecordShapeDrift,
            projection.projection_id.clone(),
            "record_kind/schema_version/shared_contract_ref",
            "credential projection record shape must match the authority contract",
        ));
    }
    if projection.projection_mode_token != projection.projection_mode.as_str()
        || projection.credential_reference_class_token
            != projection.credential_reference_class.as_str()
    {
        defects.push(AuthorityTicketDefect::new(
            AuthorityTicketDefectKind::TokenDrift,
            projection.projection_id.clone(),
            "projection_mode_token/credential_reference_class_token",
            "credential projection tokens must match their enum values",
        ));
    }
    check_guardrails(
        defects,
        projection.projection_id.as_str(),
        &projection.guardrails,
    );
    if projection.guardrails.plaintext_secret_present
        || projection.guardrails.raw_authority_material_present
    {
        defects.push(AuthorityTicketDefect::new(
            AuthorityTicketDefectKind::RawSecretMaterialPresent,
            projection.projection_id.clone(),
            "guardrails",
            "credential projection must not expose raw or plaintext secret material",
        ));
    }
    if projection.consumer_identity.consumer_id.is_empty()
        || projection
            .consumer_identity
            .consumer_capability_hash_ref
            .is_empty()
    {
        defects.push(AuthorityTicketDefect::new(
            AuthorityTicketDefectKind::CredentialProjectionConsumerMissing,
            projection.projection_id.clone(),
            "consumer_identity",
            "credential projection must identify the consumer and capability hash",
        ));
    }
}

fn check_root_change(defects: &mut Vec<AuthorityTicketDefect>, change: &RootAuthorityChangeRecord) {
    if change.record_kind != ROOT_AUTHORITY_CHANGE_RECORD_KIND
        || change.schema_version != AUTHORITY_TICKET_SCHEMA_VERSION
        || change.shared_contract_ref != AUTHORITY_TICKET_SHARED_CONTRACT_REF
    {
        defects.push(AuthorityTicketDefect::new(
            AuthorityTicketDefectKind::RecordShapeDrift,
            change.change_id.clone(),
            "record_kind/schema_version/shared_contract_ref",
            "root-authority change record shape must match the authority contract",
        ));
    }
    if change.change_class_token != change.change_class.as_str() {
        defects.push(AuthorityTicketDefect::new(
            AuthorityTicketDefectKind::TokenDrift,
            change.change_id.clone(),
            "change_class_token",
            "change_class_token must match change_class",
        ));
    }
    check_actor_binding(defects, change.change_id.as_str(), &change.admin_actor);
    check_target_identity(defects, change.change_id.as_str(), &change.target_identity);
    check_source_proof(defects, change.change_id.as_str(), &change.source_proof);
    if !change.exportable || change.audit_event_refs.is_empty() {
        defects.push(AuthorityTicketDefect::new(
            AuthorityTicketDefectKind::RootAuthorityChangeNotExportable,
            change.change_id.clone(),
            "exportable/audit_event_refs",
            "root-authority changes must be exportable and carry audit refs",
        ));
    }
}

fn check_ticket(
    defects: &mut Vec<AuthorityTicketDefect>,
    ticket: &AuthorityTicketRecord,
    projections_by_id: &BTreeMap<&str, &CredentialProjectionRecord>,
    root_changes_by_id: &BTreeMap<&str, &RootAuthorityChangeRecord>,
) {
    if ticket.record_kind != AUTHORITY_TICKET_RECORD_KIND
        || ticket.schema_version != AUTHORITY_TICKET_SCHEMA_VERSION
        || ticket.shared_contract_ref != AUTHORITY_TICKET_SHARED_CONTRACT_REF
    {
        defects.push(AuthorityTicketDefect::new(
            AuthorityTicketDefectKind::RecordShapeDrift,
            ticket.ticket_id.clone(),
            "record_kind/schema_version/shared_contract_ref",
            "authority-ticket record shape must match the authority contract",
        ));
    }
    if ticket.ticket_class_token != ticket.ticket_class.as_str()
        || ticket.side_effect_class_token != ticket.side_effect_class.as_str()
        || ticket.issuer_class_token != ticket.issuer_class.as_str()
        || ticket.request_origin_class_token != ticket.request_origin_class.as_str()
        || ticket.use_posture_token != ticket.use_posture.as_str()
        || ticket.revocation_hook.revocation_state_token
            != ticket.revocation_hook.revocation_state.as_str()
    {
        defects.push(AuthorityTicketDefect::new(
            AuthorityTicketDefectKind::TokenDrift,
            ticket.ticket_id.clone(),
            "token_fields",
            "ticket tokens must match their enum values",
        ));
    }
    if ticket.side_effect_class.required_ticket_class() != ticket.ticket_class {
        defects.push(AuthorityTicketDefect::new(
            AuthorityTicketDefectKind::SideEffectClassMismatch,
            ticket.ticket_id.clone(),
            "side_effect_class",
            "side_effect_class must map to ticket_class",
        ));
    }
    if !ticket.request_origin_class.is_intrinsic_issuer()
        && ticket
            .requesting_surface_ref
            .as_deref()
            .map(str::is_empty)
            .unwrap_or(true)
    {
        defects.push(AuthorityTicketDefect::new(
            AuthorityTicketDefectKind::MissingRequestingSurfaceRef,
            ticket.ticket_id.clone(),
            "requesting_surface_ref",
            "non-issuer request origins must carry a requesting_surface_ref",
        ));
    }
    check_actor_binding(defects, ticket.ticket_id.as_str(), &ticket.actor_binding);
    check_target_identity(defects, ticket.ticket_id.as_str(), &ticket.target_identity);
    check_sandbox_binding(defects, ticket.ticket_id.as_str(), &ticket.sandbox_binding);
    check_guardrails(defects, ticket.ticket_id.as_str(), &ticket.guardrails);
    check_ticket_lifetime(defects, ticket);
    check_remembered_rule(defects, ticket);

    match ticket.ticket_class {
        AuthorityTicketClass::CredentialProjection => {
            match ticket.credential_projection_ref.as_deref() {
                Some(id) => match projections_by_id.get(id) {
                    Some(projection) => {
                        if projection.ticket_ref != ticket.ticket_id {
                            defects.push(AuthorityTicketDefect::new(
                                AuthorityTicketDefectKind::CredentialProjectionMissing,
                                ticket.ticket_id.clone(),
                                "credential_projection_ref",
                                "credential projection must point back to the admitting ticket",
                            ));
                        }
                        if projection.target_identity_ref != ticket.target_identity.target_ref
                            || projection_expires_after_ticket(projection, ticket)
                        {
                            defects.push(AuthorityTicketDefect::new(
                                AuthorityTicketDefectKind::CredentialProjectionMissing,
                                ticket.ticket_id.clone(),
                                "credential_projection_ref",
                                "credential projection must match the ticket target and expire no later than the admitting ticket",
                            ));
                        }
                    }
                    None => defects.push(AuthorityTicketDefect::new(
                        AuthorityTicketDefectKind::CredentialProjectionMissing,
                        ticket.ticket_id.clone(),
                        "credential_projection_ref",
                        "credential projection ref must resolve on the page",
                    )),
                },
                None => defects.push(AuthorityTicketDefect::new(
                    AuthorityTicketDefectKind::CredentialProjectionMissing,
                    ticket.ticket_id.clone(),
                    "credential_projection_ref",
                    "credential projection tickets must name a projection record",
                )),
            }
        }
        AuthorityTicketClass::PolicyTrustAdminChange => {
            match ticket.root_authority_change_ref.as_deref() {
                Some(id) => match root_changes_by_id.get(id) {
                    Some(change) => {
                        if change.ticket_ref != ticket.ticket_id {
                            defects.push(AuthorityTicketDefect::new(
                                AuthorityTicketDefectKind::RootAuthorityChangeMissing,
                                ticket.ticket_id.clone(),
                                "root_authority_change_ref",
                                "root-authority change must point back to the admitting ticket",
                            ));
                        }
                        if !root_change_matches_ticket(change, ticket) {
                            defects.push(AuthorityTicketDefect::new(
                                AuthorityTicketDefectKind::RootAuthorityChangeMissing,
                                ticket.ticket_id.clone(),
                                "root_authority_change_ref",
                                "root-authority change must match the ticket admin actor, target, policy epoch, and source proof",
                            ));
                        }
                    }
                    None => defects.push(AuthorityTicketDefect::new(
                        AuthorityTicketDefectKind::RootAuthorityChangeMissing,
                        ticket.ticket_id.clone(),
                        "root_authority_change_ref",
                        "root-authority change ref must resolve on the page",
                    )),
                },
                None => defects.push(AuthorityTicketDefect::new(
                    AuthorityTicketDefectKind::RootAuthorityChangeMissing,
                    ticket.ticket_id.clone(),
                    "root_authority_change_ref",
                    "policy/trust/admin tickets must name a root-authority change record",
                )),
            }
            match &ticket.source_proof {
                Some(proof) => check_source_proof(defects, ticket.ticket_id.as_str(), proof),
                None => defects.push(AuthorityTicketDefect::new(
                    AuthorityTicketDefectKind::RootAuthorityProofMissing,
                    ticket.ticket_id.clone(),
                    "source_proof",
                    "policy/trust/admin tickets require signed or local source proof",
                )),
            }
        }
        _ => {}
    }
}

fn check_spend(
    defects: &mut Vec<AuthorityTicketDefect>,
    spend: &AuthorityTicketSpendAttempt,
    tickets_by_id: &BTreeMap<&str, &AuthorityTicketRecord>,
    projections_by_id: &BTreeMap<&str, &CredentialProjectionRecord>,
) {
    if spend.record_kind != AUTHORITY_TICKET_SPEND_ATTEMPT_RECORD_KIND
        || spend.schema_version != AUTHORITY_TICKET_SCHEMA_VERSION
        || spend.shared_contract_ref != AUTHORITY_TICKET_SHARED_CONTRACT_REF
    {
        defects.push(AuthorityTicketDefect::new(
            AuthorityTicketDefectKind::RecordShapeDrift,
            spend.spend_attempt_id.clone(),
            "record_kind/schema_version/shared_contract_ref",
            "spend-attempt record shape must match the authority contract",
        ));
    }
    if spend.attempted_ticket_class_token != spend.attempted_ticket_class.as_str()
        || spend.evaluation_outcome_token != spend.evaluation_outcome.as_str()
    {
        defects.push(AuthorityTicketDefect::new(
            AuthorityTicketDefectKind::TokenDrift,
            spend.spend_attempt_id.clone(),
            "attempted_ticket_class_token/evaluation_outcome_token",
            "spend-attempt tokens must match their enum values",
        ));
    }
    check_actor_binding(
        defects,
        spend.spend_attempt_id.as_str(),
        &spend.current_actor_binding,
    );
    check_target_identity(
        defects,
        spend.spend_attempt_id.as_str(),
        &spend.current_target_identity,
    );
    check_sandbox_binding(
        defects,
        spend.spend_attempt_id.as_str(),
        &spend.current_sandbox_binding,
    );

    let ticket = spend
        .presented_ticket_ref
        .as_deref()
        .and_then(|id| tickets_by_id.get(id).copied());

    if spend.evaluation_outcome.is_admitted() {
        match ticket {
            Some(ticket) => {
                if ticket.ticket_class != spend.attempted_ticket_class
                    || !ticket.revocation_hook.revocation_state.is_spendable()
                    || !actor_binding_matches(&ticket.actor_binding, &spend.current_actor_binding)
                    || !target_identity_matches(
                        &ticket.target_identity,
                        &spend.current_target_identity,
                    )
                    || ticket.sandbox_binding != spend.current_sandbox_binding
                    || ticket.lineage.authority_source_ref != spend.current_authority_source_ref
                    || evaluated_after_expires(spend, ticket)
                    || admitted_credential_projection_missing(ticket, projections_by_id)
                    || admitted_root_proof_missing(ticket)
                {
                    defects.push(AuthorityTicketDefect::new(
                        AuthorityTicketDefectKind::SpendAdmittedUnderDrift,
                        spend.spend_attempt_id.clone(),
                        "evaluation_outcome",
                        "admitted spend must match ticket class, actor, target, sandbox, policy epoch, authority source, expiry, and privileged lineage",
                    ));
                }
            }
            None => defects.push(AuthorityTicketDefect::new(
                AuthorityTicketDefectKind::SpendAdmittedWithoutTicket,
                spend.spend_attempt_id.clone(),
                "presented_ticket_ref",
                "admitted spend must carry a current matching ticket",
            )),
        }
    } else {
        if spend.audit_event_refs.is_empty() {
            defects.push(AuthorityTicketDefect::new(
                AuthorityTicketDefectKind::SpendDenialMissingAuditRef,
                spend.spend_attempt_id.clone(),
                "audit_event_refs",
                "denied spend must preserve an audit ref",
            ));
        }
        if spend.attempted_ticket_class.requires_reapproval_on_drift()
            && !spend.safely_replayable
            && !spend.reapproval_required
        {
            defects.push(AuthorityTicketDefect::new(
                AuthorityTicketDefectKind::SpendDenialMissingReapproval,
                spend.spend_attempt_id.clone(),
                "reapproval_required",
                "denied unsafe privileged actions must force reapproval",
            ));
        }
        if let Some(ticket) = ticket {
            match spend.evaluation_outcome {
                AuthorityEvaluationOutcome::DeniedTargetDrift => {
                    if ticket.target_identity.target_ref == spend.current_target_identity.target_ref
                        && ticket.target_identity.target_fingerprint_ref
                            == spend.current_target_identity.target_fingerprint_ref
                    {
                        defects.push(spend_outcome_mismatch(
                            spend,
                            "current_target_identity",
                            "denied_target_drift must change target identity",
                        ));
                    }
                }
                AuthorityEvaluationOutcome::DeniedPolicyEpochDrift => {
                    if ticket.sandbox_binding.policy_epoch_ref
                        == spend.current_sandbox_binding.policy_epoch_ref
                    {
                        defects.push(spend_outcome_mismatch(
                            spend,
                            "current_sandbox_binding.policy_epoch_ref",
                            "denied_policy_epoch_drift must change policy epoch",
                        ));
                    }
                }
                AuthorityEvaluationOutcome::DeniedSandboxDrift => {
                    if ticket.sandbox_binding.sandbox_profile_ref
                        == spend.current_sandbox_binding.sandbox_profile_ref
                        && ticket.sandbox_binding.sandbox_profile_fingerprint_ref
                            == spend
                                .current_sandbox_binding
                                .sandbox_profile_fingerprint_ref
                    {
                        defects.push(spend_outcome_mismatch(
                            spend,
                            "current_sandbox_binding",
                            "denied_sandbox_drift must change sandbox binding",
                        ));
                    }
                }
                AuthorityEvaluationOutcome::DeniedAuthoritySourceMismatch => {
                    if ticket.lineage.authority_source_ref == spend.current_authority_source_ref {
                        defects.push(spend_outcome_mismatch(
                            spend,
                            "current_authority_source_ref",
                            "denied_authority_source_mismatch must change authority source",
                        ));
                    }
                }
                AuthorityEvaluationOutcome::DeniedMissingTicket => {
                    defects.push(spend_outcome_mismatch(
                        spend,
                        "presented_ticket_ref",
                        "denied_missing_ticket must not carry a resolving ticket",
                    ));
                }
                AuthorityEvaluationOutcome::DeniedCredentialProjectionMissing => {
                    if !admitted_credential_projection_missing(ticket, projections_by_id) {
                        defects.push(spend_outcome_mismatch(
                            spend,
                            "credential_projection_ref",
                            "denied_credential_projection_missing must have an unresolved or unsafe projection",
                        ));
                    }
                }
                AuthorityEvaluationOutcome::DeniedRootProofMissing => {
                    if !admitted_root_proof_missing(ticket) {
                        defects.push(spend_outcome_mismatch(
                            spend,
                            "source_proof",
                            "denied_root_proof_missing must have missing or unauthoritative proof",
                        ));
                    }
                }
                _ => {}
            }
        } else if !matches!(
            spend.evaluation_outcome,
            AuthorityEvaluationOutcome::DeniedMissingTicket
        ) {
            defects.push(spend_outcome_mismatch(
                spend,
                "evaluation_outcome",
                "spend without a presented ticket must be denied_missing_ticket",
            ));
        }
    }
}

fn spend_outcome_mismatch(
    spend: &AuthorityTicketSpendAttempt,
    field: &str,
    note: &str,
) -> AuthorityTicketDefect {
    AuthorityTicketDefect::new(
        AuthorityTicketDefectKind::SpendDenialOutcomeMismatch,
        spend.spend_attempt_id.clone(),
        field,
        note,
    )
}

fn admitted_credential_projection_missing(
    ticket: &AuthorityTicketRecord,
    projections_by_id: &BTreeMap<&str, &CredentialProjectionRecord>,
) -> bool {
    if ticket.ticket_class != AuthorityTicketClass::CredentialProjection {
        return false;
    }
    ticket
        .credential_projection_ref
        .as_deref()
        .and_then(|id| projections_by_id.get(id).copied())
        .map(|projection| {
            projection.guardrails.raw_authority_material_present
                || projection.guardrails.plaintext_secret_present
                || projection.consumer_identity.consumer_id.is_empty()
                || projection
                    .consumer_identity
                    .consumer_capability_hash_ref
                    .is_empty()
        })
        .unwrap_or(true)
}

fn projection_expires_after_ticket(
    projection: &CredentialProjectionRecord,
    ticket: &AuthorityTicketRecord,
) -> bool {
    match (
        parse_timestamp(&projection.expires_at),
        parse_timestamp(&ticket.expires_at),
    ) {
        (Some(projection_expires), Some(ticket_expires)) => projection_expires > ticket_expires,
        _ => false,
    }
}

fn admitted_root_proof_missing(ticket: &AuthorityTicketRecord) -> bool {
    if ticket.ticket_class != AuthorityTicketClass::PolicyTrustAdminChange {
        return false;
    }
    ticket
        .source_proof
        .as_ref()
        .map(|proof| !proof.proof_class.is_authoritative() || source_proof_incomplete(proof))
        .unwrap_or(true)
}

fn root_change_matches_ticket(
    change: &RootAuthorityChangeRecord,
    ticket: &AuthorityTicketRecord,
) -> bool {
    actor_binding_matches(&change.admin_actor, &ticket.actor_binding)
        && target_identity_matches(&change.target_identity, &ticket.target_identity)
        && change.policy_epoch_ref == ticket.sandbox_binding.policy_epoch_ref
        && ticket
            .source_proof
            .as_ref()
            .map(|proof| proof.proof_fingerprint_ref.as_str())
            == Some(change.source_proof.proof_fingerprint_ref.as_str())
}

fn actor_binding_matches(left: &AuthorityActorBinding, right: &AuthorityActorBinding) -> bool {
    left.actor_class == right.actor_class
        && left.actor_subject_ref == right.actor_subject_ref
        && left.authority_source_ref == right.authority_source_ref
        && left.granted_scope_refs == right.granted_scope_refs
}

fn target_identity_matches(
    left: &AuthorityTargetIdentity,
    right: &AuthorityTargetIdentity,
) -> bool {
    left.target_class == right.target_class
        && left.target_ref == right.target_ref
        && left.target_fingerprint_ref == right.target_fingerprint_ref
}

fn check_actor_binding(
    defects: &mut Vec<AuthorityTicketDefect>,
    subject_id: &str,
    actor: &AuthorityActorBinding,
) {
    if actor.actor_class_token != actor.actor_class.as_str() {
        defects.push(AuthorityTicketDefect::new(
            AuthorityTicketDefectKind::TokenDrift,
            subject_id,
            "actor_binding.actor_class_token",
            "actor_class_token must match actor_class",
        ));
    }
}

fn check_target_identity(
    defects: &mut Vec<AuthorityTicketDefect>,
    subject_id: &str,
    target: &AuthorityTargetIdentity,
) {
    if target.target_class_token != target.target_class.as_str() {
        defects.push(AuthorityTicketDefect::new(
            AuthorityTicketDefectKind::TokenDrift,
            subject_id,
            "target_identity.target_class_token",
            "target_class_token must match target_class",
        ));
    }
}

fn check_sandbox_binding(
    defects: &mut Vec<AuthorityTicketDefect>,
    subject_id: &str,
    sandbox: &AuthoritySandboxBinding,
) {
    for (field, value) in [
        ("sandbox_profile_ref", sandbox.sandbox_profile_ref.as_str()),
        (
            "sandbox_profile_fingerprint_ref",
            sandbox.sandbox_profile_fingerprint_ref.as_str(),
        ),
        (
            "capability_envelope_ref",
            sandbox.capability_envelope_ref.as_str(),
        ),
        ("policy_epoch_ref", sandbox.policy_epoch_ref.as_str()),
    ] {
        if value.is_empty() {
            defects.push(AuthorityTicketDefect::new(
                AuthorityTicketDefectKind::RecordShapeDrift,
                subject_id,
                format!("sandbox_binding.{field}"),
                "sandbox binding fields must not be empty",
            ));
        }
    }
}

fn check_guardrails(
    defects: &mut Vec<AuthorityTicketDefect>,
    subject_id: &str,
    guardrails: &AuthorityGuardrails,
) {
    if guardrails.raw_authority_material_present
        || guardrails.ambient_authority_grant_present
        || guardrails.silent_authority_widening_present
        || !guardrails.local_editing_preserved
    {
        defects.push(AuthorityTicketDefect::new(
            AuthorityTicketDefectKind::GuardrailViolation,
            subject_id,
            "guardrails",
            "authority records must exclude raw authority, ambient grants, silent widening, and preserve local editing",
        ));
    }
}

fn check_source_proof(
    defects: &mut Vec<AuthorityTicketDefect>,
    subject_id: &str,
    proof: &AuthoritySourceProof,
) {
    if proof.proof_class_token != proof.proof_class.as_str() {
        defects.push(AuthorityTicketDefect::new(
            AuthorityTicketDefectKind::TokenDrift,
            subject_id,
            "source_proof.proof_class_token",
            "proof_class_token must match proof_class",
        ));
    }
    if !proof.proof_class.is_authoritative() {
        defects.push(AuthorityTicketDefect::new(
            AuthorityTicketDefectKind::RootAuthorityProofMissing,
            subject_id,
            "source_proof.proof_class",
            "root-authority changes require signed or local source proof",
        ));
    }
    if source_proof_incomplete(proof) {
        defects.push(AuthorityTicketDefect::new(
            AuthorityTicketDefectKind::RootAuthorityProofIncomplete,
            subject_id,
            "source_proof",
            "source proof must carry signature refs or a local-authority ref required by its proof class",
        ));
    }
}

fn source_proof_incomplete(proof: &AuthoritySourceProof) -> bool {
    (proof.proof_class.requires_signature()
        && (proof
            .signer_ref
            .as_deref()
            .map(str::is_empty)
            .unwrap_or(true)
            || proof
                .signature_blob_ref
                .as_deref()
                .map(str::is_empty)
                .unwrap_or(true)))
        || (proof.proof_class.requires_local_authority_ref()
            && proof
                .local_authority_ref
                .as_deref()
                .map(str::is_empty)
                .unwrap_or(true))
}

fn check_ticket_lifetime(defects: &mut Vec<AuthorityTicketDefect>, ticket: &AuthorityTicketRecord) {
    match (
        parse_timestamp(&ticket.issued_at),
        parse_timestamp(&ticket.expires_at),
    ) {
        (Some(issued), Some(expires)) if expires > issued => {
            let actual = (expires - issued) as u64;
            if actual != ticket.lifetime_seconds {
                defects.push(AuthorityTicketDefect::new(
                    AuthorityTicketDefectKind::TicketLifetimeMismatch,
                    ticket.ticket_id.clone(),
                    "lifetime_seconds",
                    "lifetime_seconds must equal expires_at - issued_at",
                ));
            }
            if actual > ticket.ticket_class.max_lifetime_seconds() {
                defects.push(AuthorityTicketDefect::new(
                    AuthorityTicketDefectKind::TicketLifetimeExceedsBudget,
                    ticket.ticket_id.clone(),
                    "expires_at",
                    "ticket lifetime exceeds the class-specific budget",
                ));
            }
        }
        (Some(_), Some(_)) => defects.push(AuthorityTicketDefect::new(
            AuthorityTicketDefectKind::TicketLifetimeMismatch,
            ticket.ticket_id.clone(),
            "expires_at",
            "expires_at must be after issued_at",
        )),
        _ => {}
    }
}

fn check_remembered_rule(defects: &mut Vec<AuthorityTicketDefect>, ticket: &AuthorityTicketRecord) {
    if let Some(rule) = &ticket.remembered_rule {
        if matches!(
            ticket.ticket_class,
            AuthorityTicketClass::CredentialProjection
                | AuthorityTicketClass::PrivilegedDebugAttach
                | AuthorityTicketClass::PolicyTrustAdminChange
        ) {
            defects.push(AuthorityTicketDefect::new(
                AuthorityTicketDefectKind::RememberedRuleForbidden,
                ticket.ticket_id.clone(),
                "remembered_rule",
                "credential projection, privileged attach, and admin/root changes must reprompt rather than remember",
            ));
        }
        if rule.scope_ref.is_empty()
            || rule.reusable_rule_id.is_empty()
            || rule.renewable_ticket_lifetime_seconds == 0
            || rule.renewable_ticket_lifetime_seconds > ticket.ticket_class.max_lifetime_seconds()
        {
            defects.push(AuthorityTicketDefect::new(
                AuthorityTicketDefectKind::RememberedRuleTooBroad,
                ticket.ticket_id.clone(),
                "remembered_rule",
                "remembered decisions must narrow to a scope and renew only short-lived tickets",
            ));
        }
    }
}

fn evaluated_after_expires(
    spend: &AuthorityTicketSpendAttempt,
    ticket: &AuthorityTicketRecord,
) -> bool {
    match (
        parse_timestamp(&spend.evaluated_at),
        parse_timestamp(&ticket.expires_at),
    ) {
        (Some(evaluated), Some(expires)) => evaluated > expires,
        _ => false,
    }
}

/// Minimal strict UTC timestamp parser for `YYYY-MM-DDTHH:MM:SSZ`.
fn parse_timestamp(value: &str) -> Option<i64> {
    let bytes = value.as_bytes();
    if bytes.len() != 20
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes[19] != b'Z'
    {
        return None;
    }
    let year: i64 = std::str::from_utf8(&bytes[0..4]).ok()?.parse().ok()?;
    let month: i64 = std::str::from_utf8(&bytes[5..7]).ok()?.parse().ok()?;
    let day: i64 = std::str::from_utf8(&bytes[8..10]).ok()?.parse().ok()?;
    let hour: i64 = std::str::from_utf8(&bytes[11..13]).ok()?.parse().ok()?;
    let minute: i64 = std::str::from_utf8(&bytes[14..16]).ok()?.parse().ok()?;
    let second: i64 = std::str::from_utf8(&bytes[17..19]).ok()?.parse().ok()?;
    if !(1..=12).contains(&month)
        || !(1..=31).contains(&day)
        || hour > 23
        || minute > 59
        || second > 60
    {
        return None;
    }
    let days_per_month: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut days_before_year = 0;
    for y in 1970..year {
        days_before_year += 365;
        if is_leap(y) {
            days_before_year += 1;
        }
    }
    let mut days_before_month = 0;
    for (m, &count) in days_per_month.iter().enumerate() {
        if (m as i64) + 1 >= month {
            break;
        }
        days_before_month += count;
        if m == 1 && is_leap(year) {
            days_before_month += 1;
        }
    }
    Some(
        (days_before_year + days_before_month + day - 1) * 86_400
            + hour * 3600
            + minute * 60
            + second,
    )
}

const fn is_leap(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// Builds the seeded authority-ticket page.
pub fn seeded_authority_ticket_page() -> AuthorityTicketPage {
    let credential_projections = seed_credential_projections();
    let root_authority_changes = seed_root_authority_changes();
    let tickets = seed_tickets();
    let spend_attempts = seed_spend_attempts();
    let defects = audit_authority_ticket_page(
        &tickets,
        &credential_projections,
        &root_authority_changes,
        &spend_attempts,
    );
    let summary = AuthorityTicketSummary::from_records(
        &tickets,
        &credential_projections,
        &root_authority_changes,
        &spend_attempts,
        &defects,
    );
    AuthorityTicketPage {
        record_kind: AUTHORITY_TICKET_PAGE_RECORD_KIND.to_owned(),
        schema_version: AUTHORITY_TICKET_SCHEMA_VERSION,
        shared_contract_ref: AUTHORITY_TICKET_SHARED_CONTRACT_REF.to_owned(),
        source_matrix_ref: AUTHORITY_TICKET_SOURCE_MATRIX_REF.to_owned(),
        tickets,
        credential_projections,
        root_authority_changes,
        spend_attempts,
        defects,
        summary,
    }
}

fn actor(
    actor_class: AuthorityActorClass,
    actor_subject_ref: &str,
    authority_source_ref: &str,
    granted_scope_refs: Vec<&str>,
) -> AuthorityActorBinding {
    AuthorityActorBinding {
        actor_class,
        actor_class_token: actor_class.as_str().to_owned(),
        actor_subject_ref: actor_subject_ref.to_owned(),
        authority_source_ref: authority_source_ref.to_owned(),
        granted_scope_refs: granted_scope_refs.into_iter().map(String::from).collect(),
    }
}

fn target(
    target_class: AuthorityTargetClass,
    target_ref: &str,
    target_label: &str,
    target_fingerprint_ref: &str,
) -> AuthorityTargetIdentity {
    AuthorityTargetIdentity {
        target_class,
        target_class_token: target_class.as_str().to_owned(),
        target_ref: target_ref.to_owned(),
        target_label: target_label.to_owned(),
        target_fingerprint_ref: target_fingerprint_ref.to_owned(),
    }
}

fn sandbox(
    sandbox_profile_ref: &str,
    sandbox_profile_fingerprint_ref: &str,
    capability_envelope_ref: &str,
    policy_epoch_ref: &str,
) -> AuthoritySandboxBinding {
    AuthoritySandboxBinding {
        sandbox_profile_ref: sandbox_profile_ref.to_owned(),
        sandbox_profile_fingerprint_ref: sandbox_profile_fingerprint_ref.to_owned(),
        capability_envelope_ref: capability_envelope_ref.to_owned(),
        policy_epoch_ref: policy_epoch_ref.to_owned(),
    }
}

fn revocation_live() -> AuthorityRevocationHook {
    AuthorityRevocationHook {
        revocation_state: AuthorityRevocationState::Live,
        revocation_state_token: AuthorityRevocationState::Live.as_str().to_owned(),
        revoke_epoch_ref: None,
        revoke_event_ref: None,
        revoke_path_ref: "revoke-path:authority-ticket:standard".to_owned(),
    }
}

fn lineage(authority_source_ref: &str, fingerprint_ref: &str) -> AuthorityLineage {
    AuthorityLineage {
        parent_ticket_refs: Vec::new(),
        authority_source_ref: authority_source_ref.to_owned(),
        lineage_fingerprint_ref: fingerprint_ref.to_owned(),
    }
}

fn signed_root_proof() -> AuthoritySourceProof {
    AuthoritySourceProof {
        proof_class: AuthoritySourceProofClass::SignedTrustRootRotation,
        proof_class_token: AuthoritySourceProofClass::SignedTrustRootRotation
            .as_str()
            .to_owned(),
        source_ref: "root-source:signed-trust-root-rotation:2026-05-18".to_owned(),
        signer_ref: Some("signer:security-root-quorum:2026".to_owned()),
        signature_blob_ref: Some("signature:trust-root-rotation:2026-05-18".to_owned()),
        local_authority_ref: None,
        proof_fingerprint_ref: "proof-fingerprint:trust-root-rotation:2026-05-18".to_owned(),
        verified_at: "2026-05-18T09:55:00Z".to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn ticket(
    ticket_id: &str,
    display_label: &str,
    ticket_class: AuthorityTicketClass,
    side_effect_class: AuthoritySideEffectClass,
    issuer_class: AuthorityIssuerClass,
    issuing_surface_ref: &str,
    request_origin_class: AuthorityRequestOriginClass,
    requesting_surface_ref: Option<&str>,
    actor_binding: AuthorityActorBinding,
    target_identity: AuthorityTargetIdentity,
    sandbox_binding: AuthoritySandboxBinding,
    issued_at: &str,
    expires_at: &str,
    lifetime_seconds: u64,
    use_posture: AuthorityUsePosture,
    credential_projection_ref: Option<&str>,
    root_authority_change_ref: Option<&str>,
    source_proof: Option<AuthoritySourceProof>,
    remembered_rule: Option<RememberedAuthorityRule>,
    authority_source_ref: &str,
    lineage_fingerprint_ref: &str,
    audit_event_refs: Vec<&str>,
) -> AuthorityTicketRecord {
    AuthorityTicketRecord {
        record_kind: AUTHORITY_TICKET_RECORD_KIND.to_owned(),
        schema_version: AUTHORITY_TICKET_SCHEMA_VERSION,
        shared_contract_ref: AUTHORITY_TICKET_SHARED_CONTRACT_REF.to_owned(),
        ticket_id: ticket_id.to_owned(),
        display_label: display_label.to_owned(),
        ticket_class,
        ticket_class_token: ticket_class.as_str().to_owned(),
        side_effect_class,
        side_effect_class_token: side_effect_class.as_str().to_owned(),
        issuer_class,
        issuer_class_token: issuer_class.as_str().to_owned(),
        issuing_surface_ref: issuing_surface_ref.to_owned(),
        request_origin_class,
        request_origin_class_token: request_origin_class.as_str().to_owned(),
        requesting_surface_ref: requesting_surface_ref.map(String::from),
        actor_binding,
        target_identity,
        sandbox_binding,
        issued_at: issued_at.to_owned(),
        expires_at: expires_at.to_owned(),
        lifetime_seconds,
        use_posture,
        use_posture_token: use_posture.as_str().to_owned(),
        revocation_hook: revocation_live(),
        credential_projection_ref: credential_projection_ref.map(String::from),
        root_authority_change_ref: root_authority_change_ref.map(String::from),
        source_proof,
        remembered_rule,
        lineage: lineage(authority_source_ref, lineage_fingerprint_ref),
        audit_event_refs: audit_event_refs.into_iter().map(String::from).collect(),
        guardrails: AuthorityGuardrails::clean(),
    }
}

fn seed_credential_projections() -> Vec<CredentialProjectionRecord> {
    vec![
        credential_projection(
            "credential-projection:delegated-handle:registry:0001",
            "authority-ticket:credential:delegated-handle:0001",
            CredentialProjectionMode::DelegatedHandle,
            CredentialReferenceClass::DelegatedCredential,
            "consumer:build-task:private-registry-fetch",
            "Private registry fetch build task",
            "capability-hash:consumer:build-task:registry:v1",
            "secret-class:registry-token",
            "projected-credential:delegated:registry:0001",
            "2026-05-18T10:03:00Z",
        ),
        credential_projection(
            "credential-projection:session-only:terminal:0002",
            "authority-ticket:credential:session-only:0002",
            CredentialProjectionMode::SessionOnlySecret,
            CredentialReferenceClass::SessionOnlySecret,
            "consumer:terminal:deploy-shell",
            "Session-only deploy shell",
            "capability-hash:consumer:terminal:deploy:v1",
            "secret-class:ssh-agent-session",
            "projected-credential:session-only:ssh:0002",
            "2026-05-18T10:02:00Z",
        ),
        credential_projection(
            "credential-projection:sign-only:provider:0003",
            "authority-ticket:credential:sign-only:0003",
            CredentialProjectionMode::SignOnly,
            CredentialReferenceClass::ProjectedCredential,
            "consumer:provider:release-signer",
            "Provider release signer",
            "capability-hash:consumer:provider:release-signer:v1",
            "secret-class:signing-key-material",
            "projected-credential:sign-only:release:0003",
            "2026-05-18T10:02:00Z",
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn credential_projection(
    projection_id: &str,
    ticket_ref: &str,
    projection_mode: CredentialProjectionMode,
    credential_reference_class: CredentialReferenceClass,
    consumer_id: &str,
    consumer_label: &str,
    consumer_capability_hash_ref: &str,
    secret_class_ref: &str,
    projected_credential_ref: &str,
    expires_at: &str,
) -> CredentialProjectionRecord {
    CredentialProjectionRecord {
        record_kind: CREDENTIAL_PROJECTION_RECORD_KIND.to_owned(),
        schema_version: AUTHORITY_TICKET_SCHEMA_VERSION,
        shared_contract_ref: AUTHORITY_TICKET_SHARED_CONTRACT_REF.to_owned(),
        projection_id: projection_id.to_owned(),
        ticket_ref: ticket_ref.to_owned(),
        projection_mode,
        projection_mode_token: projection_mode.as_str().to_owned(),
        credential_reference_class,
        credential_reference_class_token: credential_reference_class.as_str().to_owned(),
        consumer_identity: CredentialConsumerIdentity {
            consumer_id: consumer_id.to_owned(),
            consumer_label: consumer_label.to_owned(),
            consumer_capability_hash_ref: consumer_capability_hash_ref.to_owned(),
        },
        secret_class_ref: secret_class_ref.to_owned(),
        projected_credential_ref: projected_credential_ref.to_owned(),
        target_identity_ref: consumer_id.to_owned(),
        expires_at: expires_at.to_owned(),
        revocation_path_ref: "revoke-path:credential-projection:broker".to_owned(),
        guardrails: AuthorityGuardrails::clean(),
    }
}

fn seed_root_authority_changes() -> Vec<RootAuthorityChangeRecord> {
    vec![RootAuthorityChangeRecord {
        record_kind: ROOT_AUTHORITY_CHANGE_RECORD_KIND.to_owned(),
        schema_version: AUTHORITY_TICKET_SCHEMA_VERSION,
        shared_contract_ref: AUTHORITY_TICKET_SHARED_CONTRACT_REF.to_owned(),
        change_id: "root-authority-change:trust-root-rotation:0001".to_owned(),
        ticket_ref: "authority-ticket:admin:trust-root-rotation:0001".to_owned(),
        change_class: RootAuthorityChangeClass::SigningRootRotation,
        change_class_token: RootAuthorityChangeClass::SigningRootRotation
            .as_str()
            .to_owned(),
        source_proof: signed_root_proof(),
        admin_actor: actor(
            AuthorityActorClass::OrganizationAdmin,
            "actor:org-admin:security-lead",
            "root-source:signed-trust-root-rotation:2026-05-18",
            vec!["scope:admin:trust-root:rotate"],
        ),
        target_identity: target(
            AuthorityTargetClass::TrustStore,
            "trust-root:2026-primary",
            "Trust root 2026 primary",
            "target-fingerprint:trust-root:2026-primary:v1",
        ),
        policy_epoch_ref: "policy-epoch:enterprise:2026-05-18".to_owned(),
        effective_at: "2026-05-18T10:05:00Z".to_owned(),
        rollback_ref: "rollback:trust-root:restore-2025-primary".to_owned(),
        audit_event_refs: vec![
            "audit:root-authority:trust-root-rotation:issued".to_owned(),
            "audit:root-authority:trust-root-rotation:verified".to_owned(),
        ],
        exportable: true,
    }]
}

fn seed_tickets() -> Vec<AuthorityTicketRecord> {
    let policy_epoch = "policy-epoch:enterprise:2026-05-18";
    vec![
        ticket(
            "authority-ticket:local:remembered-format:0001",
            "Remembered local formatter rule minted a short-lived ticket",
            AuthorityTicketClass::LocalMutation,
            AuthoritySideEffectClass::LocalWorkspaceWrite,
            AuthorityIssuerClass::PolicyService,
            "policy-service:remembered-rule:format-current-repo",
            AuthorityRequestOriginClass::PolicyDecision,
            None,
            actor(
                AuthorityActorClass::HumanAccount,
                "actor:user:local-01",
                "authority-source:local-user-session:01",
                vec!["scope:workspace:aureline:format"],
            ),
            target(
                AuthorityTargetClass::LocalWorkspace,
                "workspace:aureline:current-repo",
                "Aureline current repository",
                "target-fingerprint:workspace:aureline:v1",
            ),
            sandbox(
                "sandbox:local-mutation:format:v1",
                "sandbox-fingerprint:local-mutation:format:v1",
                "capability-envelope:local-format:v1",
                policy_epoch,
            ),
            "2026-05-18T10:00:00Z",
            "2026-05-18T10:10:00Z",
            600,
            AuthorityUsePosture::BoundedReuse,
            None,
            None,
            None,
            Some(RememberedAuthorityRule {
                reusable_rule_id: "remembered-rule:format-current-repo".to_owned(),
                scope_ref: "scope:workspace:aureline:format".to_owned(),
                renewable_ticket_lifetime_seconds: 600,
                owner_policy_source_ref: "authority-source:local-user-session:01".to_owned(),
                revoke_path_ref: "revoke-path:remembered-rule:format-current-repo".to_owned(),
            }),
            "authority-source:local-user-session:01",
            "lineage-fingerprint:local-format:0001",
            vec!["audit:authority-ticket:local-format:issued"],
        ),
        ticket(
            "authority-ticket:external:provider-publish:0001",
            "Shell ticket for one provider publish",
            AuthorityTicketClass::ExternalProviderMutation,
            AuthoritySideEffectClass::ExternalProviderWrite,
            AuthorityIssuerClass::Shell,
            "shell-prompt:provider-publish:0001",
            AuthorityRequestOriginClass::AiToolPlan,
            Some("ai-tool-plan:release-note-publish:0001"),
            actor(
                AuthorityActorClass::HumanAccount,
                "actor:user:local-01",
                "authority-source:provider-session:github:01",
                vec!["scope:provider:github:owner/repo:publish"],
            ),
            target(
                AuthorityTargetClass::ProviderObject,
                "provider:github:owner/repo:release-draft:42",
                "Release draft 42 in owner/repo",
                "target-fingerprint:provider:release-draft:42:v1",
            ),
            sandbox(
                "sandbox:external-provider:publish:v1",
                "sandbox-fingerprint:external-provider:publish:v1",
                "capability-envelope:provider-publish:v1",
                policy_epoch,
            ),
            "2026-05-18T10:00:00Z",
            "2026-05-18T10:05:00Z",
            300,
            AuthorityUsePosture::SingleUse,
            None,
            None,
            None,
            None,
            "authority-source:provider-session:github:01",
            "lineage-fingerprint:provider-publish:0001",
            vec!["audit:authority-ticket:provider-publish:issued"],
        ),
        ticket(
            "authority-ticket:credential:delegated-handle:0001",
            "Policy ticket for delegated registry credential projection",
            AuthorityTicketClass::CredentialProjection,
            AuthoritySideEffectClass::CredentialProjectionToConsumer,
            AuthorityIssuerClass::PolicyService,
            "policy-service:credential-projection:0001",
            AuthorityRequestOriginClass::PolicyDecision,
            None,
            actor(
                AuthorityActorClass::InstallationOrAppGrant,
                "actor:install:build-agent",
                "authority-source:secret-broker:vault:registry:01",
                vec!["scope:secret:registry:read"],
            ),
            target(
                AuthorityTargetClass::CredentialConsumer,
                "consumer:build-task:private-registry-fetch",
                "Private registry fetch build task",
                "target-fingerprint:consumer:build-task:registry:v1",
            ),
            sandbox(
                "sandbox:credential-projection:build:v1",
                "sandbox-fingerprint:credential-projection:build:v1",
                "capability-envelope:credential-projection:registry:v1",
                policy_epoch,
            ),
            "2026-05-18T10:00:00Z",
            "2026-05-18T10:03:00Z",
            180,
            AuthorityUsePosture::BoundedReuse,
            Some("credential-projection:delegated-handle:registry:0001"),
            None,
            None,
            None,
            "authority-source:secret-broker:vault:registry:01",
            "lineage-fingerprint:credential:delegated-handle:0001",
            vec!["audit:authority-ticket:credential-delegated:issued"],
        ),
        ticket(
            "authority-ticket:credential:session-only:0002",
            "Shell ticket for session-only deploy credential projection",
            AuthorityTicketClass::CredentialProjection,
            AuthoritySideEffectClass::CredentialProjectionToConsumer,
            AuthorityIssuerClass::Shell,
            "shell-prompt:credential-session-only:0002",
            AuthorityRequestOriginClass::CliScriptRequest,
            Some("cli-invocation:deploy-script:0002"),
            actor(
                AuthorityActorClass::HumanAccount,
                "actor:user:local-01",
                "authority-source:os-keychain:ssh-agent:01",
                vec!["scope:secret:ssh-agent:session-only"],
            ),
            target(
                AuthorityTargetClass::CredentialConsumer,
                "consumer:terminal:deploy-shell",
                "Session-only deploy shell",
                "target-fingerprint:consumer:terminal:deploy:v1",
            ),
            sandbox(
                "sandbox:credential-projection:terminal:v1",
                "sandbox-fingerprint:credential-projection:terminal:v1",
                "capability-envelope:credential-projection:ssh:v1",
                policy_epoch,
            ),
            "2026-05-18T10:00:00Z",
            "2026-05-18T10:02:00Z",
            120,
            AuthorityUsePosture::SingleUse,
            Some("credential-projection:session-only:terminal:0002"),
            None,
            None,
            None,
            "authority-source:os-keychain:ssh-agent:01",
            "lineage-fingerprint:credential:session-only:0002",
            vec!["audit:authority-ticket:credential-session-only:issued"],
        ),
        ticket(
            "authority-ticket:credential:sign-only:0003",
            "Supervisor ticket for sign-only provider release credential",
            AuthorityTicketClass::CredentialProjection,
            AuthoritySideEffectClass::CredentialProjectionToConsumer,
            AuthorityIssuerClass::Supervisor,
            "supervisor:release-signing:0003",
            AuthorityRequestOriginClass::AutomationSchedulerRequest,
            Some("scheduler:release-signing:0003"),
            actor(
                AuthorityActorClass::PolicyInjectedServiceIdentity,
                "actor:service:release-signer",
                "authority-source:hsm:release-signing:01",
                vec!["scope:secret:release-signing:sign-only"],
            ),
            target(
                AuthorityTargetClass::CredentialConsumer,
                "consumer:provider:release-signer",
                "Provider release signer",
                "target-fingerprint:consumer:provider:release-signer:v1",
            ),
            sandbox(
                "sandbox:credential-projection:signing:v1",
                "sandbox-fingerprint:credential-projection:signing:v1",
                "capability-envelope:credential-projection:signing:v1",
                policy_epoch,
            ),
            "2026-05-18T10:00:00Z",
            "2026-05-18T10:02:00Z",
            120,
            AuthorityUsePosture::SingleUse,
            Some("credential-projection:sign-only:provider:0003"),
            None,
            None,
            None,
            "authority-source:hsm:release-signing:01",
            "lineage-fingerprint:credential:sign-only:0003",
            vec!["audit:authority-ticket:credential-sign-only:issued"],
        ),
        ticket(
            "authority-ticket:debug:privileged-attach:0001",
            "Shell ticket for privileged debug attach",
            AuthorityTicketClass::PrivilegedDebugAttach,
            AuthoritySideEffectClass::PrivilegedInspectionAttach,
            AuthorityIssuerClass::Shell,
            "shell-prompt:debug-attach:0001",
            AuthorityRequestOriginClass::UserShellPrompt,
            None,
            actor(
                AuthorityActorClass::HumanAccount,
                "actor:user:local-01",
                "authority-source:local-user-session:01",
                vec!["scope:debug:attach:language-server"],
            ),
            target(
                AuthorityTargetClass::DebugAttachTarget,
                "debug-target:process:language-server:7421",
                "Language server process 7421",
                "target-fingerprint:debug-target:language-server:7421:v1",
            ),
            sandbox(
                "sandbox:debug-attach:local:v1",
                "sandbox-fingerprint:debug-attach:local:v1",
                "capability-envelope:debug-attach:language-server:v1",
                policy_epoch,
            ),
            "2026-05-18T10:00:00Z",
            "2026-05-18T10:04:00Z",
            240,
            AuthorityUsePosture::SingleUse,
            None,
            None,
            None,
            None,
            "authority-source:local-user-session:01",
            "lineage-fingerprint:debug-attach:0001",
            vec!["audit:authority-ticket:debug-attach:issued"],
        ),
        ticket(
            "authority-ticket:admin:trust-root-rotation:0001",
            "Supervisor ticket for signed trust-root rotation",
            AuthorityTicketClass::PolicyTrustAdminChange,
            AuthoritySideEffectClass::PolicyTrustMutation,
            AuthorityIssuerClass::Supervisor,
            "supervisor:trust-root-rotation:0001",
            AuthorityRequestOriginClass::SupervisorControlPath,
            None,
            actor(
                AuthorityActorClass::OrganizationAdmin,
                "actor:org-admin:security-lead",
                "root-source:signed-trust-root-rotation:2026-05-18",
                vec!["scope:admin:trust-root:rotate"],
            ),
            target(
                AuthorityTargetClass::TrustStore,
                "trust-root:2026-primary",
                "Trust root 2026 primary",
                "target-fingerprint:trust-root:2026-primary:v1",
            ),
            sandbox(
                "sandbox:root-authority:supervisor:v1",
                "sandbox-fingerprint:root-authority:supervisor:v1",
                "capability-envelope:root-authority:trust-root-rotation:v1",
                policy_epoch,
            ),
            "2026-05-18T10:00:00Z",
            "2026-05-18T10:05:00Z",
            300,
            AuthorityUsePosture::SingleUse,
            None,
            Some("root-authority-change:trust-root-rotation:0001"),
            Some(signed_root_proof()),
            None,
            "root-source:signed-trust-root-rotation:2026-05-18",
            "lineage-fingerprint:root-authority:trust-root-rotation:0001",
            vec!["audit:authority-ticket:root-authority:issued"],
        ),
    ]
}

fn seed_spend_attempts() -> Vec<AuthorityTicketSpendAttempt> {
    let policy_epoch = "policy-epoch:enterprise:2026-05-18";
    let credential_actor = actor(
        AuthorityActorClass::InstallationOrAppGrant,
        "actor:install:build-agent",
        "authority-source:secret-broker:vault:registry:01",
        vec!["scope:secret:registry:read"],
    );
    let credential_target = target(
        AuthorityTargetClass::CredentialConsumer,
        "consumer:build-task:private-registry-fetch",
        "Private registry fetch build task",
        "target-fingerprint:consumer:build-task:registry:v1",
    );
    let credential_sandbox = sandbox(
        "sandbox:credential-projection:build:v1",
        "sandbox-fingerprint:credential-projection:build:v1",
        "capability-envelope:credential-projection:registry:v1",
        policy_epoch,
    );
    let debug_actor = actor(
        AuthorityActorClass::HumanAccount,
        "actor:user:local-01",
        "authority-source:local-user-session:01",
        vec!["scope:debug:attach:language-server"],
    );
    let debug_target = target(
        AuthorityTargetClass::DebugAttachTarget,
        "debug-target:process:language-server:7421",
        "Language server process 7421",
        "target-fingerprint:debug-target:language-server:7421:v1",
    );
    let debug_sandbox = sandbox(
        "sandbox:debug-attach:local:v1",
        "sandbox-fingerprint:debug-attach:local:v1",
        "capability-envelope:debug-attach:language-server:v1",
        policy_epoch,
    );
    let root_actor = actor(
        AuthorityActorClass::OrganizationAdmin,
        "actor:org-admin:security-lead",
        "root-source:signed-trust-root-rotation:2026-05-18",
        vec!["scope:admin:trust-root:rotate"],
    );
    let root_target = target(
        AuthorityTargetClass::TrustStore,
        "trust-root:2026-primary",
        "Trust root 2026 primary",
        "target-fingerprint:trust-root:2026-primary:v1",
    );
    let root_sandbox = sandbox(
        "sandbox:root-authority:supervisor:v1",
        "sandbox-fingerprint:root-authority:supervisor:v1",
        "capability-envelope:root-authority:trust-root-rotation:v1",
        policy_epoch,
    );
    vec![
        spend(
            "authority-spend:credential:admitted:0001",
            AuthorityTicketClass::CredentialProjection,
            Some("authority-ticket:credential:delegated-handle:0001"),
            credential_actor.clone(),
            credential_target.clone(),
            credential_sandbox.clone(),
            "authority-source:secret-broker:vault:registry:01",
            AuthorityEvaluationOutcome::Admitted,
            false,
            false,
            "Credential projection matched consumer, target, sandbox, policy epoch, and broker authority source.",
            vec!["audit:authority-spend:credential:admitted"],
        ),
        spend(
            "authority-spend:credential:policy-epoch-drift:0002",
            AuthorityTicketClass::CredentialProjection,
            Some("authority-ticket:credential:delegated-handle:0001"),
            credential_actor.clone(),
            credential_target.clone(),
            sandbox(
                "sandbox:credential-projection:build:v1",
                "sandbox-fingerprint:credential-projection:build:v1",
                "capability-envelope:credential-projection:registry:v1",
                "policy-epoch:enterprise:2026-05-18-rev2",
            ),
            "authority-source:secret-broker:vault:registry:01",
            AuthorityEvaluationOutcome::DeniedPolicyEpochDrift,
            true,
            false,
            "Policy epoch advanced before spend; credential projection must be reapproved.",
            vec!["audit:authority-spend:credential:policy-epoch-drift:deny"],
        ),
        spend(
            "authority-spend:debug:admitted:0003",
            AuthorityTicketClass::PrivilegedDebugAttach,
            Some("authority-ticket:debug:privileged-attach:0001"),
            debug_actor.clone(),
            debug_target.clone(),
            debug_sandbox.clone(),
            "authority-source:local-user-session:01",
            AuthorityEvaluationOutcome::Admitted,
            false,
            false,
            "Privileged attach matched the short-lived ticket and platform sandbox binding.",
            vec!["audit:authority-spend:debug:admitted"],
        ),
        spend(
            "authority-spend:debug:missing-ticket:0004",
            AuthorityTicketClass::PrivilegedDebugAttach,
            None,
            debug_actor,
            debug_target,
            debug_sandbox,
            "authority-source:local-user-session:01",
            AuthorityEvaluationOutcome::DeniedMissingTicket,
            true,
            false,
            "Privileged attach failed closed because no current ticket was presented.",
            vec!["audit:authority-spend:debug:missing-ticket:deny"],
        ),
        spend(
            "authority-spend:root:admitted:0005",
            AuthorityTicketClass::PolicyTrustAdminChange,
            Some("authority-ticket:admin:trust-root-rotation:0001"),
            root_actor.clone(),
            root_target.clone(),
            root_sandbox.clone(),
            "root-source:signed-trust-root-rotation:2026-05-18",
            AuthorityEvaluationOutcome::Admitted,
            false,
            false,
            "Trust-root rotation matched the signed root-authority source proof and current policy epoch.",
            vec!["audit:authority-spend:root:admitted"],
        ),
        spend(
            "authority-spend:root:authority-source-mismatch:0006",
            AuthorityTicketClass::PolicyTrustAdminChange,
            Some("authority-ticket:admin:trust-root-rotation:0001"),
            root_actor,
            root_target,
            root_sandbox,
            "root-source:unsigned-admin-request:2026-05-18",
            AuthorityEvaluationOutcome::DeniedAuthoritySourceMismatch,
            true,
            false,
            "Root-authority source changed from signed rotation proof to an unsigned admin request; reapproval is required.",
            vec!["audit:authority-spend:root:source-mismatch:deny"],
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn spend(
    spend_attempt_id: &str,
    attempted_ticket_class: AuthorityTicketClass,
    presented_ticket_ref: Option<&str>,
    current_actor_binding: AuthorityActorBinding,
    current_target_identity: AuthorityTargetIdentity,
    current_sandbox_binding: AuthoritySandboxBinding,
    current_authority_source_ref: &str,
    evaluation_outcome: AuthorityEvaluationOutcome,
    reapproval_required: bool,
    safely_replayable: bool,
    explanation: &str,
    audit_event_refs: Vec<&str>,
) -> AuthorityTicketSpendAttempt {
    AuthorityTicketSpendAttempt {
        record_kind: AUTHORITY_TICKET_SPEND_ATTEMPT_RECORD_KIND.to_owned(),
        schema_version: AUTHORITY_TICKET_SCHEMA_VERSION,
        shared_contract_ref: AUTHORITY_TICKET_SHARED_CONTRACT_REF.to_owned(),
        spend_attempt_id: spend_attempt_id.to_owned(),
        attempted_ticket_class,
        attempted_ticket_class_token: attempted_ticket_class.as_str().to_owned(),
        presented_ticket_ref: presented_ticket_ref.map(String::from),
        current_actor_binding,
        current_target_identity,
        current_sandbox_binding,
        current_authority_source_ref: current_authority_source_ref.to_owned(),
        evaluated_at: "2026-05-18T10:01:00Z".to_owned(),
        evaluation_outcome,
        evaluation_outcome_token: evaluation_outcome.as_str().to_owned(),
        reapproval_required,
        safely_replayable,
        explanation: explanation.to_owned(),
        audit_event_refs: audit_event_refs.into_iter().map(String::from).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates() {
        let page = seeded_authority_ticket_page();
        validate_authority_ticket_page(&page).expect("seeded authority page validates");
        assert!(page.defects.is_empty());
        for ticket_class in AuthorityTicketClass::ALL {
            assert!(page
                .summary
                .ticket_classes_present
                .iter()
                .any(|token| token == ticket_class.as_str()));
        }
    }

    #[test]
    fn validator_flags_admitted_privileged_attach_without_ticket() {
        let mut page = seeded_authority_ticket_page();
        let spend = page
            .spend_attempts
            .iter_mut()
            .find(|spend| {
                spend.evaluation_outcome == AuthorityEvaluationOutcome::DeniedMissingTicket
            })
            .expect("missing-ticket spend");
        spend.evaluation_outcome = AuthorityEvaluationOutcome::Admitted;
        spend.evaluation_outcome_token = AuthorityEvaluationOutcome::Admitted.as_str().to_owned();
        let defects = audit_authority_ticket_page(
            &page.tickets,
            &page.credential_projections,
            &page.root_authority_changes,
            &page.spend_attempts,
        );
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind
                == AuthorityTicketDefectKind::SpendAdmittedWithoutTicket));
    }

    #[test]
    fn validator_flags_raw_secret_projection() {
        let mut page = seeded_authority_ticket_page();
        page.credential_projections[0]
            .guardrails
            .plaintext_secret_present = true;
        let defects = audit_authority_ticket_page(
            &page.tickets,
            &page.credential_projections,
            &page.root_authority_changes,
            &page.spend_attempts,
        );
        assert!(defects.iter().any(
            |defect| defect.defect_kind == AuthorityTicketDefectKind::RawSecretMaterialPresent
        ));
    }

    #[test]
    fn validator_flags_unsigned_root_change() {
        let mut page = seeded_authority_ticket_page();
        let proof = &mut page.root_authority_changes[0].source_proof;
        proof.proof_class = AuthoritySourceProofClass::MissingOrUnverified;
        proof.proof_class_token = AuthoritySourceProofClass::MissingOrUnverified
            .as_str()
            .to_owned();
        let defects = audit_authority_ticket_page(
            &page.tickets,
            &page.credential_projections,
            &page.root_authority_changes,
            &page.spend_attempts,
        );
        assert!(defects.iter().any(
            |defect| defect.defect_kind == AuthorityTicketDefectKind::RootAuthorityProofMissing
        ));
    }

    #[test]
    fn validator_flags_denial_without_reapproval() {
        let mut page = seeded_authority_ticket_page();
        let spend = page
            .spend_attempts
            .iter_mut()
            .find(|spend| {
                spend.evaluation_outcome
                    == AuthorityEvaluationOutcome::DeniedAuthoritySourceMismatch
            })
            .expect("source-mismatch spend");
        spend.reapproval_required = false;
        let defects = audit_authority_ticket_page(
            &page.tickets,
            &page.credential_projections,
            &page.root_authority_changes,
            &page.spend_attempts,
        );
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind
                == AuthorityTicketDefectKind::SpendDenialMissingReapproval));
    }

    #[test]
    fn validator_flags_projection_expiry_beyond_ticket() {
        let mut page = seeded_authority_ticket_page();
        page.credential_projections[1].expires_at = "2026-05-18T10:03:00Z".to_owned();
        let defects = audit_authority_ticket_page(
            &page.tickets,
            &page.credential_projections,
            &page.root_authority_changes,
            &page.spend_attempts,
        );
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind
                == AuthorityTicketDefectKind::CredentialProjectionMissing));
    }
}
