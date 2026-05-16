//! Sandbox profiles, capability envelopes, and approval-ticket issuance beta
//! page.
//!
//! The provider crate landed an alpha approval-ticket packet that binds
//! provider-plane and helper-backed remote mutations to a short-lived ticket or
//! reviewed-scope object. The beta projection landed here extends that
//! vocabulary to the shell/policy plane so every high-risk local, provider,
//! remote, and credential-projection action is admitted through a typed
//! authority object that the shell or policy service has minted, never
//! self-authorized by extensions, AI tool plans, helpers, or CLI scripts.
//!
//! The page carries four reviewable record kinds:
//!
//! - [`SandboxProfileRow`] — one row per claimed `(profile, sandbox
//!   profile class)` pair. The row names the typed sandbox profile, the
//!   capability classes the profile may ever admit, the side-effect classes
//!   it may ever bind, the trust-profile and policy-epoch refs, the default
//!   use-posture, and the max ticket-lifetime budget.
//! - [`CapabilityEnvelopeRow`] — one row per envelope minted from a sandbox
//!   profile. The envelope names the typed action class, the side-effect
//!   class, the target identity, the actor scope, the allowed capability
//!   classes, the sealed-at and expires-at timestamps, and a typed evidence
//!   and rollback ref list.
//! - [`IssuedApprovalTicketRow`] — one row per ticket the shell or policy
//!   plane mints. The ticket names the issuer class (`shell`,
//!   `policy_service`, `supervisor`), the request-origin class, the
//!   capability-envelope ref, the actor scope, the trust-profile and
//!   policy-epoch refs, the issued-at and expires-at timestamps, the typed
//!   use-posture, and the typed authority requirement.
//! - [`SpendAttemptEvent`] — one event per attempt to spend a ticket against
//!   the current authority context. The event captures the current actor
//!   scope, target identity, sandbox-profile ref, capability-envelope ref,
//!   trust-profile ref, policy-epoch ref, the typed evaluation outcome, the
//!   typed native-reapproval route, an export-safe explanation, and audit
//!   refs.
//!
//! The validator surfaces typed defects when raw authority material is
//! claimed on a row, when an extension or AI plan attempts self-authorization,
//! when authority is silently widened beyond the envelope, when a spend
//! attempt is admitted under drift, when a denial route collapses to
//! `not_required` without a remediation, when a denial misses an audit ref,
//! when a ticket lifetime exceeds its sandbox profile budget, or when any of
//! the four claimed beta profiles is missing.
//!
//! [`ApprovalTicketBetaSupportExport`] wraps the page in a redaction-safe
//! envelope that admin, support, and reviewer surfaces replay verbatim. The
//! export preserves authority lineage (sandbox profile, capability envelope,
//! ticket, spend attempt, audit event refs) and proves the
//! no-self-authorization invariant.
//!
//! Reviewer-facing landing page:
//! [`/docs/security/m3/approval_ticket_beta.md`](../../../../docs/security/m3/approval_ticket_beta.md).

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Beta schema version exported with every approval-ticket beta record.
pub const APPROVAL_TICKET_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every approval-ticket beta record.
pub const APPROVAL_TICKET_BETA_SHARED_CONTRACT_REF: &str = "security:approval_ticket_beta:v1";

/// Source matrix ref consumed by this beta projection.
pub const APPROVAL_TICKET_BETA_SOURCE_MATRIX_REF: &str =
    "artifacts/security/m3/approval_ticket/approval_ticket_matrix.yaml";

/// Stable record kind for the page payload.
pub const APPROVAL_TICKET_BETA_PAGE_RECORD_KIND: &str = "security_approval_ticket_beta_page_record";

/// Stable record kind for a sandbox-profile row payload.
pub const APPROVAL_TICKET_BETA_SANDBOX_PROFILE_ROW_RECORD_KIND: &str =
    "security_approval_ticket_beta_sandbox_profile_row_record";

/// Stable record kind for a capability-envelope row payload.
pub const APPROVAL_TICKET_BETA_CAPABILITY_ENVELOPE_ROW_RECORD_KIND: &str =
    "security_approval_ticket_beta_capability_envelope_row_record";

/// Stable record kind for an issued approval-ticket row payload.
pub const APPROVAL_TICKET_BETA_TICKET_ROW_RECORD_KIND: &str =
    "security_approval_ticket_beta_ticket_row_record";

/// Stable record kind for a spend-attempt event payload.
pub const APPROVAL_TICKET_BETA_SPEND_ATTEMPT_EVENT_RECORD_KIND: &str =
    "security_approval_ticket_beta_spend_attempt_event_record";

/// Stable record kind for a summary payload.
pub const APPROVAL_TICKET_BETA_SUMMARY_RECORD_KIND: &str =
    "security_approval_ticket_beta_summary_record";

/// Stable record kind for a defect payload.
pub const APPROVAL_TICKET_BETA_DEFECT_RECORD_KIND: &str =
    "security_approval_ticket_beta_defect_record";

/// Stable record kind for the support-export wrapper.
pub const APPROVAL_TICKET_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "security_approval_ticket_beta_support_export_record";

/// Profile under which an approval-ticket beta row, envelope, ticket, or spend
/// attempt is inspected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalTicketBetaProfileClass {
    /// Connected beta profile with live provider, helper, and policy paths.
    Connected,
    /// Mirror-only profile served exclusively from a signed mirror; no public
    /// fallback admitted.
    MirrorOnly,
    /// Offline profile served from an air-gapped or last-known-good signed
    /// bundle.
    Offline,
    /// Enterprise-managed profile applying signed managed narrowing.
    EnterpriseManaged,
}

impl ApprovalTicketBetaProfileClass {
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

/// Typed sandbox-profile class that constrains the capabilities a ticket may
/// ever admit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxProfileClass {
    /// Local-only authority; no provider or remote effects admitted.
    LocalOnlyAuthority,
    /// External provider mutations (review comment publish, issue update, CI
    /// rerun) admitted under tight scopes.
    ProviderMutationSandbox,
    /// Helper-backed remote mutations admitted under tight scopes.
    RemoteHelperSandbox,
    /// Credential projection from broker handles to a consumer.
    CredentialProjectionSandbox,
}

impl SandboxProfileClass {
    /// All sandbox-profile classes in canonical order.
    pub const ALL: [Self; 4] = [
        Self::LocalOnlyAuthority,
        Self::ProviderMutationSandbox,
        Self::RemoteHelperSandbox,
        Self::CredentialProjectionSandbox,
    ];

    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyAuthority => "local_only_authority",
            Self::ProviderMutationSandbox => "provider_mutation_sandbox",
            Self::RemoteHelperSandbox => "remote_helper_sandbox",
            Self::CredentialProjectionSandbox => "credential_projection_sandbox",
        }
    }
}

/// Typed high-risk action class that an envelope and ticket bind to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HighRiskActionClass {
    /// External provider mutation: review-comment publish, issue update, CI
    /// rerun.
    ExternalProviderMutation,
    /// Helper-backed remote mutation through a tunneled/remote helper.
    HelperBackedRemoteMutation,
    /// Local high-risk action (destructive workspace mutation, restricted
    /// build step) admitted only with shell-minted authority.
    LocalHighRiskAction,
    /// Credential projection from a broker handle to a typed consumer.
    CredentialProjection,
}

impl HighRiskActionClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExternalProviderMutation => "external_provider_mutation",
            Self::HelperBackedRemoteMutation => "helper_backed_remote_mutation",
            Self::LocalHighRiskAction => "local_high_risk_action",
            Self::CredentialProjection => "credential_projection",
        }
    }

    /// Sandbox profile every envelope of this action class MUST be derived
    /// from. Returning a fixed sandbox profile makes envelope drift typed
    /// rather than narrative.
    pub const fn required_sandbox_profile(self) -> SandboxProfileClass {
        match self {
            Self::ExternalProviderMutation => SandboxProfileClass::ProviderMutationSandbox,
            Self::HelperBackedRemoteMutation => SandboxProfileClass::RemoteHelperSandbox,
            Self::LocalHighRiskAction => SandboxProfileClass::LocalOnlyAuthority,
            Self::CredentialProjection => SandboxProfileClass::CredentialProjectionSandbox,
        }
    }
}

/// Typed side-effect class an envelope and ticket bind to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectClass {
    /// Provider review-comment publish.
    ProviderReviewCommentPublish,
    /// Provider issue update.
    ProviderIssueUpdate,
    /// Provider CI rerun.
    ProviderCiRerun,
    /// Helper-backed remote mutation.
    RemoteHelperMutation,
    /// Credential projection to a typed consumer.
    CredentialProjectionToConsumer,
    /// Local destructive workspace action.
    LocalDestructiveAction,
}

impl SideEffectClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderReviewCommentPublish => "provider_review_comment_publish",
            Self::ProviderIssueUpdate => "provider_issue_update",
            Self::ProviderCiRerun => "provider_ci_rerun",
            Self::RemoteHelperMutation => "remote_helper_mutation",
            Self::CredentialProjectionToConsumer => "credential_projection_to_consumer",
            Self::LocalDestructiveAction => "local_destructive_action",
        }
    }

    /// The high-risk action class this side effect maps to.
    pub const fn action_class(self) -> HighRiskActionClass {
        match self {
            Self::ProviderReviewCommentPublish
            | Self::ProviderIssueUpdate
            | Self::ProviderCiRerun => HighRiskActionClass::ExternalProviderMutation,
            Self::RemoteHelperMutation => HighRiskActionClass::HelperBackedRemoteMutation,
            Self::CredentialProjectionToConsumer => HighRiskActionClass::CredentialProjection,
            Self::LocalDestructiveAction => HighRiskActionClass::LocalHighRiskAction,
        }
    }
}

/// Capability class that an envelope and ticket may admit. The set is a strict
/// subset of the sandbox profile's allowed capability classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityClass {
    /// Publish a provider review comment.
    WriteProviderReviewComment,
    /// Update a provider issue (open, close, comment).
    UpdateProviderIssue,
    /// Rerun a provider CI job.
    RerunProviderCi,
    /// Mutate a helper-backed remote target.
    MutateRemoteHelperTarget,
    /// Project a credential to a typed consumer.
    ProjectCredentialToConsumer,
    /// Execute a local high-risk action.
    ExecuteLocalHighRiskAction,
}

impl CapabilityClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WriteProviderReviewComment => "write_provider_review_comment",
            Self::UpdateProviderIssue => "update_provider_issue",
            Self::RerunProviderCi => "rerun_provider_ci",
            Self::MutateRemoteHelperTarget => "mutate_remote_helper_target",
            Self::ProjectCredentialToConsumer => "project_credential_to_consumer",
            Self::ExecuteLocalHighRiskAction => "execute_local_high_risk_action",
        }
    }
}

/// Issuer class authorised to mint approval tickets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuerClass {
    /// Shell-minted ticket from an interactive user prompt.
    Shell,
    /// Policy-service-minted ticket from a typed policy decision.
    PolicyService,
    /// Supervisor-minted ticket on a managed control path.
    Supervisor,
}

impl IssuerClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::PolicyService => "policy_service",
            Self::Supervisor => "supervisor",
        }
    }
}

/// Request-origin class for a ticket request. Extensions, AI tool plans, CLI
/// scripts, helpers, browser companions, and automation schedulers MUST go
/// through one of the issuer surfaces — they never self-authorize.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestOriginClass {
    /// User-confirmed shell prompt.
    UserShellPrompt,
    /// Policy decision (signed bundle or live policy service).
    PolicyDecision,
    /// Supervisor control path.
    SupervisorControlPath,
    /// AI tool plan asking the shell or policy plane to authorize an action.
    AiToolPlan,
    /// Extension instance requesting authority through the shell or policy
    /// plane.
    ExtensionRequest,
    /// CLI script requesting authority through the shell or policy plane.
    CliScriptRequest,
    /// Browser-companion session requesting authority through the shell.
    BrowserCompanionRequest,
    /// Remote helper requesting authority through the supervisor or policy
    /// plane.
    RemoteHelperRequest,
    /// Automation scheduler requesting authority through the supervisor or
    /// policy plane.
    AutomationSchedulerRequest,
}

impl RequestOriginClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserShellPrompt => "user_shell_prompt",
            Self::PolicyDecision => "policy_decision",
            Self::SupervisorControlPath => "supervisor_control_path",
            Self::AiToolPlan => "ai_tool_plan",
            Self::ExtensionRequest => "extension_request",
            Self::CliScriptRequest => "cli_script_request",
            Self::BrowserCompanionRequest => "browser_companion_request",
            Self::RemoteHelperRequest => "remote_helper_request",
            Self::AutomationSchedulerRequest => "automation_scheduler_request",
        }
    }

    /// True when this request origin is its own issuer surface. Other request
    /// origins MUST carry a `requesting_surface_ref` and be admitted through a
    /// typed issuer class.
    pub const fn is_intrinsic_issuer(self) -> bool {
        matches!(
            self,
            Self::UserShellPrompt | Self::PolicyDecision | Self::SupervisorControlPath
        )
    }
}

/// Actor class for the actor scope on a ticket or envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorClass {
    /// Human account.
    HumanAccount,
    /// Installation or app grant.
    InstallationOrAppGrant,
    /// Delegated credential.
    DelegatedCredential,
    /// Project-scoped grant.
    ProjectScopedGrant,
    /// Policy-injected service identity.
    PolicyInjectedServiceIdentity,
    /// Local-only authority.
    LocalOnlyAuthority,
}

impl ActorClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HumanAccount => "human_account",
            Self::InstallationOrAppGrant => "installation_or_app_grant",
            Self::DelegatedCredential => "delegated_credential",
            Self::ProjectScopedGrant => "project_scoped_grant",
            Self::PolicyInjectedServiceIdentity => "policy_injected_service_identity",
            Self::LocalOnlyAuthority => "local_only_authority",
        }
    }
}

/// Auth-source class for the actor scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthSourceClass {
    /// Human session.
    HumanSession,
    /// Installation grant.
    InstallationGrant,
    /// Delegated credential.
    DelegatedCredential,
    /// Project-scoped grant.
    ProjectScopedGrant,
    /// Policy-injected service.
    PolicyInjectedService,
    /// Local-only authority.
    LocalOnly,
}

impl AuthSourceClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HumanSession => "human_session",
            Self::InstallationGrant => "installation_grant",
            Self::DelegatedCredential => "delegated_credential",
            Self::ProjectScopedGrant => "project_scoped_grant",
            Self::PolicyInjectedService => "policy_injected_service",
            Self::LocalOnly => "local_only",
        }
    }
}

/// Target-class for a target identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetClass {
    /// External provider object.
    ProviderObject,
    /// Helper-backed remote target.
    RemoteHelperTarget,
    /// Local-only workspace target.
    LocalOnlyTarget,
    /// Credential consumer.
    CredentialConsumerTarget,
}

impl TargetClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderObject => "provider_object",
            Self::RemoteHelperTarget => "remote_helper_target",
            Self::LocalOnlyTarget => "local_only_target",
            Self::CredentialConsumerTarget => "credential_consumer_target",
        }
    }
}

/// Use-posture for an issued ticket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UsePosture {
    /// Single-use ticket (one admitted spend).
    SingleUse,
    /// Bounded reuse ticket (limited admitted spends).
    BoundedReuse,
}

impl UsePosture {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleUse => "single_use",
            Self::BoundedReuse => "bounded_reuse",
        }
    }
}

/// Authority requirement that a ticket-bound mutation declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityRequirement {
    /// An approval ticket is required.
    ApprovalTicketRequired,
    /// A reviewed scope is required.
    ReviewedScopeRequired,
    /// Either an approval ticket or a reviewed scope is admitted.
    TicketOrReviewedScope,
}

impl AuthorityRequirement {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ApprovalTicketRequired => "approval_ticket_required",
            Self::ReviewedScopeRequired => "reviewed_scope_required",
            Self::TicketOrReviewedScope => "ticket_or_reviewed_scope",
        }
    }
}

/// Typed evaluation outcome of a spend attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvaluationOutcome {
    /// The ticket admitted the spend; current authority context matched the
    /// ticket on every axis.
    Admitted,
    /// No ticket or reviewed scope was presented.
    DeniedMissingAuthority,
    /// The presented ticket is past its `expires_at`.
    DeniedExpired,
    /// The current target identity does not match the ticket target.
    DeniedTargetDrift,
    /// The current trust profile does not match the ticket trust profile.
    DeniedTrustProfileDrift,
    /// The current sandbox profile does not match the ticket sandbox profile.
    DeniedSandboxProfileDrift,
    /// The current policy epoch does not match the ticket policy epoch.
    DeniedPolicyEpochDrift,
    /// The current actor scope does not match the ticket actor scope.
    DeniedActorScopeMismatch,
    /// The current capability envelope does not match the ticket envelope, or
    /// the requested capability is not in the envelope's allowed set.
    DeniedCapabilityEnvelopeDrift,
    /// The request originated from a non-issuer surface and attempted to
    /// self-authorize (no shell, policy, or supervisor surface backs the
    /// presented ticket).
    DeniedSelfAuthorizationAttempted,
}

impl EvaluationOutcome {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::DeniedMissingAuthority => "denied_missing_authority",
            Self::DeniedExpired => "denied_expired",
            Self::DeniedTargetDrift => "denied_target_drift",
            Self::DeniedTrustProfileDrift => "denied_trust_profile_drift",
            Self::DeniedSandboxProfileDrift => "denied_sandbox_profile_drift",
            Self::DeniedPolicyEpochDrift => "denied_policy_epoch_drift",
            Self::DeniedActorScopeMismatch => "denied_actor_scope_mismatch",
            Self::DeniedCapabilityEnvelopeDrift => "denied_capability_envelope_drift",
            Self::DeniedSelfAuthorizationAttempted => "denied_self_authorization_attempted",
        }
    }

    /// True when the outcome admits the spend.
    pub const fn is_admitted(self) -> bool {
        matches!(self, Self::Admitted)
    }

    /// True when the outcome denies the spend.
    pub const fn is_denied(self) -> bool {
        !self.is_admitted()
    }
}

/// Typed native-reapproval route surfaced on a spend attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeReapprovalRoute {
    /// No reapproval is required (admitted spend).
    NotRequired,
    /// Surface the native reapproval sheet so the user mints a fresh ticket.
    NativeReapprovalSheet,
    /// Refresh the target (fingerprint or version drift) and reapprove.
    RefreshTargetThenReapprove,
    /// Re-authenticate the actor and reapprove.
    ReauthThenReapprove,
    /// Rescope the actor (narrow or rotate scope) and reapprove.
    RescopeThenReapprove,
    /// Inspect-only path; the action is permanently denied without operator
    /// override.
    InspectOnlyDenied,
}

impl NativeReapprovalRoute {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::NativeReapprovalSheet => "native_reapproval_sheet",
            Self::RefreshTargetThenReapprove => "refresh_target_then_reapprove",
            Self::ReauthThenReapprove => "reauth_then_reapprove",
            Self::RescopeThenReapprove => "rescope_then_reapprove",
            Self::InspectOnlyDenied => "inspect_only_denied",
        }
    }
}

/// Actor scope shared by sandbox profiles, capability envelopes, tickets, and
/// spend attempts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorScope {
    /// Typed actor class.
    pub actor_class: ActorClass,
    /// Stable token for [`Self::actor_class`].
    pub actor_class_token: String,
    /// Opaque actor subject ref.
    pub actor_subject_ref: String,
    /// Opaque list of granted scope refs.
    pub granted_scope_refs: Vec<String>,
    /// Typed auth-source class.
    pub auth_source_class: AuthSourceClass,
    /// Stable token for [`Self::auth_source_class`].
    pub auth_source_class_token: String,
}

/// Target identity shared by envelopes, tickets, and spend attempts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetIdentity {
    /// Typed target class.
    pub target_class: TargetClass,
    /// Stable token for [`Self::target_class`].
    pub target_class_token: String,
    /// Opaque target ref safe for support export.
    pub target_ref: String,
    /// Reviewable target label safe for UI/support.
    pub target_label: String,
    /// Opaque target fingerprint ref (drift signal).
    pub target_fingerprint_ref: String,
    /// Opaque target version ref (drift signal).
    pub target_version_ref: String,
}

/// Beta guardrails carried on every record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaGuardrails {
    /// Beta guardrail: raw authority material (ticket bodies, delegated-token
    /// payloads, signed evidence bodies) is not present on the record.
    pub raw_authority_material_present: bool,
    /// Beta guardrail: the record does not record a self-authorization
    /// attempt that bypassed the shell/policy/supervisor issuance lane.
    pub self_authorization_attempted: bool,
    /// Beta guardrail: authority was not silently widened beyond the envelope.
    pub silent_widening_attempted: bool,
    /// Beta guardrail: plaintext secret bytes are not present.
    pub plaintext_secret_present: bool,
    /// Beta guardrail: undeclared public-endpoint fallback was not offered.
    pub public_endpoint_fallback_offered: bool,
    /// Beta guardrail: local editing is preserved through this failure mode.
    pub local_editing_preserved: bool,
}

impl BetaGuardrails {
    fn clean() -> Self {
        Self {
            raw_authority_material_present: false,
            self_authorization_attempted: false,
            silent_widening_attempted: false,
            plaintext_secret_present: false,
            public_endpoint_fallback_offered: false,
            local_editing_preserved: true,
        }
    }
}

/// One claimed sandbox-profile row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxProfileRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id safe for UI, logs, and support export.
    pub sandbox_profile_row_id: String,
    /// Reviewable label safe for UI and support export.
    pub display_label: String,
    /// Beta profile under which the row is inspected.
    pub profile: ApprovalTicketBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Typed sandbox-profile class.
    pub sandbox_profile_class: SandboxProfileClass,
    /// Stable token for [`Self::sandbox_profile_class`].
    pub sandbox_profile_class_token: String,
    /// Stable opaque sandbox-profile ref referenced by envelopes and tickets.
    pub sandbox_profile_ref: String,
    /// Opaque trust-profile ref binding the sandbox to the active trust
    /// posture.
    pub trust_profile_ref: String,
    /// Opaque policy-epoch ref binding the sandbox to the active policy epoch.
    pub policy_epoch_ref: String,
    /// Capability classes this sandbox profile may ever admit.
    pub allowed_capability_classes: Vec<CapabilityClass>,
    /// Stable tokens for [`Self::allowed_capability_classes`].
    pub allowed_capability_class_tokens: Vec<String>,
    /// Side-effect classes this sandbox profile may ever bind.
    pub allowed_side_effect_classes: Vec<SideEffectClass>,
    /// Stable tokens for [`Self::allowed_side_effect_classes`].
    pub allowed_side_effect_class_tokens: Vec<String>,
    /// Default use-posture for tickets minted under this sandbox profile.
    pub default_use_posture: UsePosture,
    /// Stable token for [`Self::default_use_posture`].
    pub default_use_posture_token: String,
    /// Maximum admitted ticket lifetime, in seconds, under this sandbox
    /// profile. Tickets that exceed this budget surface a typed defect.
    pub max_ticket_lifetime_seconds: u64,
    /// Beta guardrails.
    pub guardrails: BetaGuardrails,
}

/// One capability-envelope row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityEnvelopeRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id safe for UI, logs, and support export.
    pub capability_envelope_row_id: String,
    /// Reviewable label safe for UI and support export.
    pub display_label: String,
    /// Beta profile under which the envelope is inspected.
    pub profile: ApprovalTicketBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Sandbox-profile ref this envelope was derived from.
    pub sandbox_profile_ref: String,
    /// Stable opaque envelope id referenced by tickets.
    pub envelope_id: String,
    /// Opaque envelope fingerprint ref (drift signal).
    pub envelope_fingerprint_ref: String,
    /// Typed action class.
    pub action_class: HighRiskActionClass,
    /// Stable token for [`Self::action_class`].
    pub action_class_token: String,
    /// Typed side-effect class.
    pub side_effect_class: SideEffectClass,
    /// Stable token for [`Self::side_effect_class`].
    pub side_effect_class_token: String,
    /// Bound target identity.
    pub target_identity: TargetIdentity,
    /// Capability classes admitted by this envelope.
    pub allowed_capability_classes: Vec<CapabilityClass>,
    /// Stable tokens for [`Self::allowed_capability_classes`].
    pub allowed_capability_class_tokens: Vec<String>,
    /// Actor scope this envelope was sealed for.
    pub actor_scope: ActorScope,
    /// Timestamp at which the envelope was sealed.
    pub sealed_at: String,
    /// Timestamp at which the envelope expires.
    pub expires_at: String,
    /// Opaque evidence refs paired with this envelope.
    pub evidence_refs: Vec<String>,
    /// Opaque rollback refs paired with this envelope.
    pub rollback_refs: Vec<String>,
    /// Beta guardrails.
    pub guardrails: BetaGuardrails,
}

/// One issued approval-ticket row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IssuedApprovalTicketRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque approval-ticket id.
    pub approval_ticket_id: String,
    /// Reviewable label safe for UI and support export.
    pub display_label: String,
    /// Beta profile under which the ticket is inspected.
    pub profile: ApprovalTicketBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Issuer class (which surface minted the ticket).
    pub issuer_class: IssuerClass,
    /// Stable token for [`Self::issuer_class`].
    pub issuer_class_token: String,
    /// Opaque issuing-surface ref.
    pub issuing_surface_ref: String,
    /// Request-origin class.
    pub request_origin_class: RequestOriginClass,
    /// Stable token for [`Self::request_origin_class`].
    pub request_origin_class_token: String,
    /// Optional opaque requesting-surface ref. Required when
    /// [`Self::request_origin_class`] is not an intrinsic issuer surface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requesting_surface_ref: Option<String>,
    /// Sandbox-profile ref this ticket was minted under.
    pub sandbox_profile_ref: String,
    /// Capability-envelope ref this ticket binds to.
    pub capability_envelope_ref: String,
    /// Bound target identity.
    pub target_identity: TargetIdentity,
    /// Typed action class.
    pub action_class: HighRiskActionClass,
    /// Stable token for [`Self::action_class`].
    pub action_class_token: String,
    /// Typed side-effect class.
    pub side_effect_class: SideEffectClass,
    /// Stable token for [`Self::side_effect_class`].
    pub side_effect_class_token: String,
    /// Actor scope this ticket was minted for.
    pub actor_scope: ActorScope,
    /// Opaque trust-profile ref binding the ticket to the active trust
    /// posture.
    pub trust_profile_ref: String,
    /// Opaque policy-epoch ref binding the ticket to the active policy epoch.
    pub policy_epoch_ref: String,
    /// Timestamp at which the ticket was issued.
    pub issued_at: String,
    /// Timestamp at which the ticket expires.
    pub expires_at: String,
    /// Ticket lifetime, in seconds, derived from `issued_at` and `expires_at`.
    pub lifetime_seconds: u64,
    /// Use-posture.
    pub use_posture: UsePosture,
    /// Stable token for [`Self::use_posture`].
    pub use_posture_token: String,
    /// Authority requirement.
    pub authority_requirement: AuthorityRequirement,
    /// Stable token for [`Self::authority_requirement`].
    pub authority_requirement_token: String,
    /// Opaque evidence refs paired with this ticket.
    pub evidence_refs: Vec<String>,
    /// Opaque rollback refs paired with this ticket.
    pub rollback_refs: Vec<String>,
    /// Opaque ref to the upstream runtime approval-ticket record (
    /// `schemas/runtime/approval_ticket.schema.json`).
    pub runtime_approval_ticket_ref: String,
    /// Optional opaque ref to the provider-plane approval-ticket alpha
    /// projection (`schemas/security/approval_ticket_alpha.schema.json`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_plane_approval_ticket_ref: Option<String>,
    /// Beta guardrails.
    pub guardrails: BetaGuardrails,
}

/// One spend-attempt event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpendAttemptEvent {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque spend-attempt id.
    pub spend_attempt_id: String,
    /// Beta profile under which the spend is inspected.
    pub profile: ApprovalTicketBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Opaque mutation ref the spend attempts to authorize.
    pub mutation_ref: String,
    /// Optional ref to the presented ticket. When absent the outcome MUST be
    /// `denied_missing_authority`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presented_approval_ticket_ref: Option<String>,
    /// Current actor scope at evaluation time.
    pub current_actor_scope: ActorScope,
    /// Current target identity at evaluation time.
    pub current_target_identity: TargetIdentity,
    /// Current sandbox-profile ref.
    pub current_sandbox_profile_ref: String,
    /// Current capability-envelope ref.
    pub current_capability_envelope_ref: String,
    /// Current trust-profile ref.
    pub current_trust_profile_ref: String,
    /// Current policy-epoch ref.
    pub current_policy_epoch_ref: String,
    /// Timestamp at which the spend was evaluated.
    pub evaluated_at: String,
    /// Typed evaluation outcome.
    pub evaluation_outcome: EvaluationOutcome,
    /// Stable token for [`Self::evaluation_outcome`].
    pub evaluation_outcome_token: String,
    /// Typed native-reapproval route.
    pub native_reapproval_route: NativeReapprovalRoute,
    /// Stable token for [`Self::native_reapproval_route`].
    pub native_reapproval_route_token: String,
    /// Export-safe explanation of the outcome.
    pub explanation: String,
    /// Opaque audit-event refs paired with this attempt.
    pub audit_event_refs: Vec<String>,
}

/// Defect-kind vocabulary surfaced by the beta validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalTicketBetaDefectKind {
    /// Record claims raw authority material is present.
    RawAuthorityMaterialPresent,
    /// Record claims a self-authorization attempt occurred.
    SelfAuthorizationAttempted,
    /// Record claims silent widening was attempted.
    SilentWideningAttempted,
    /// Record claims plaintext secret is present.
    PlaintextSecretPresent,
    /// Record claims undeclared public-endpoint fallback was offered.
    PublicEndpointFallbackOffered,
    /// Record does not preserve local editing.
    LocalEditingNotPreserved,
    /// A non-intrinsic request origin (extension, AI, CLI, helper, scheduler)
    /// did not supply a `requesting_surface_ref`.
    MissingRequestingSurfaceRef,
    /// `profile_token` did not match `profile`.
    ProfileTokenDrift,
    /// `sandbox_profile_class_token` did not match `sandbox_profile_class`.
    SandboxProfileClassTokenDrift,
    /// `action_class_token` did not match `action_class`.
    ActionClassTokenDrift,
    /// `side_effect_class_token` did not match `side_effect_class`.
    SideEffectClassTokenDrift,
    /// `issuer_class_token` did not match `issuer_class`.
    IssuerClassTokenDrift,
    /// `request_origin_class_token` did not match `request_origin_class`.
    RequestOriginClassTokenDrift,
    /// `use_posture_token` did not match `use_posture`.
    UsePostureTokenDrift,
    /// `authority_requirement_token` did not match `authority_requirement`.
    AuthorityRequirementTokenDrift,
    /// `evaluation_outcome_token` did not match `evaluation_outcome`.
    EvaluationOutcomeTokenDrift,
    /// `native_reapproval_route_token` did not match
    /// `native_reapproval_route`.
    NativeReapprovalRouteTokenDrift,
    /// `target_class_token` did not match `target_class`.
    TargetClassTokenDrift,
    /// `actor_class_token` did not match `actor_class`.
    ActorClassTokenDrift,
    /// `auth_source_class_token` did not match `auth_source_class`.
    AuthSourceClassTokenDrift,
    /// `allowed_capability_class_tokens` did not match
    /// `allowed_capability_classes`.
    CapabilityClassTokensDrift,
    /// `allowed_side_effect_class_tokens` did not match
    /// `allowed_side_effect_classes`.
    SideEffectTokensDrift,
    /// A capability envelope was derived from a sandbox profile class that
    /// does not admit its action class.
    EnvelopeSandboxClassMismatch,
    /// A capability envelope admits a capability class outside its sandbox
    /// profile's allowed set.
    EnvelopeCapabilityOutsideSandbox,
    /// A capability envelope admits a side-effect class outside its sandbox
    /// profile's allowed set.
    EnvelopeSideEffectOutsideSandbox,
    /// A capability envelope's sandbox-profile ref is not present on the page.
    EnvelopeSandboxRefUnknown,
    /// A ticket's sandbox-profile ref is not present on the page.
    TicketSandboxRefUnknown,
    /// A ticket's capability-envelope ref is not present on the page.
    TicketEnvelopeRefUnknown,
    /// A ticket's lifetime exceeds the sandbox profile's
    /// `max_ticket_lifetime_seconds` budget.
    TicketLifetimeExceedsSandboxBudget,
    /// A ticket's `lifetime_seconds` did not match the difference between
    /// `issued_at` and `expires_at`.
    TicketLifetimeMismatch,
    /// A ticket's expires_at is not after issued_at.
    TicketExpiryNotAfterIssuance,
    /// A spend attempt references an approval ticket id that is not present.
    SpendTicketRefUnknown,
    /// A spend attempt admitted under sandbox-profile drift, target drift,
    /// trust-profile drift, policy-epoch drift, actor-scope drift, or
    /// envelope drift.
    SpendAdmittedUnderDrift,
    /// A denial outcome on a spend attempt missed an audit-event ref.
    SpendDenialMissingAuditRef,
    /// A denial outcome collapsed `native_reapproval_route` to
    /// `not_required`.
    SpendDenialReapprovalRouteCollapsed,
    /// An admitted outcome declared a `native_reapproval_route` other than
    /// `not_required`.
    SpendAdmittedReapprovalRouteUnexpected,
    /// A spend attempt typed `denied_self_authorization_attempted` was minted
    /// by an intrinsic issuer surface.
    SelfAuthorizationDenialOnIntrinsicIssuer,
    /// A spend attempt typed `denied_missing_authority` carried a
    /// presented_approval_ticket_ref.
    MissingAuthorityCarriedTicketRef,
    /// A spend attempt typed `denied_expired` did not have
    /// `evaluated_at > ticket.expires_at`.
    ExpiredSpendNotAfterExpiry,
    /// A spend attempt typed `denied_target_drift` did not change the
    /// current target ref vs the ticket target ref.
    TargetDriftMatchesTicketTarget,
    /// A spend attempt typed `denied_sandbox_profile_drift` did not change
    /// the current sandbox-profile ref vs the ticket sandbox-profile ref.
    SandboxDriftMatchesTicketSandbox,
    /// A spend attempt typed `denied_policy_epoch_drift` did not change the
    /// current policy-epoch ref vs the ticket policy-epoch ref.
    PolicyEpochDriftMatchesTicketEpoch,
    /// A spend attempt typed `denied_trust_profile_drift` did not change the
    /// current trust-profile ref vs the ticket trust-profile ref.
    TrustProfileDriftMatchesTicketTrust,
    /// A spend attempt typed `denied_capability_envelope_drift` did not
    /// change the current capability-envelope ref vs the ticket envelope ref.
    EnvelopeDriftMatchesTicketEnvelope,
    /// One of the four required beta profiles has no claimed row.
    ProfileCoverageMissing,
    /// One of the four sandbox-profile classes has no claimed row.
    SandboxProfileCoverageMissing,
}

impl ApprovalTicketBetaDefectKind {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawAuthorityMaterialPresent => "raw_authority_material_present",
            Self::SelfAuthorizationAttempted => "self_authorization_attempted",
            Self::SilentWideningAttempted => "silent_widening_attempted",
            Self::PlaintextSecretPresent => "plaintext_secret_present",
            Self::PublicEndpointFallbackOffered => "public_endpoint_fallback_offered",
            Self::LocalEditingNotPreserved => "local_editing_not_preserved",
            Self::MissingRequestingSurfaceRef => "missing_requesting_surface_ref",
            Self::ProfileTokenDrift => "profile_token_drift",
            Self::SandboxProfileClassTokenDrift => "sandbox_profile_class_token_drift",
            Self::ActionClassTokenDrift => "action_class_token_drift",
            Self::SideEffectClassTokenDrift => "side_effect_class_token_drift",
            Self::IssuerClassTokenDrift => "issuer_class_token_drift",
            Self::RequestOriginClassTokenDrift => "request_origin_class_token_drift",
            Self::UsePostureTokenDrift => "use_posture_token_drift",
            Self::AuthorityRequirementTokenDrift => "authority_requirement_token_drift",
            Self::EvaluationOutcomeTokenDrift => "evaluation_outcome_token_drift",
            Self::NativeReapprovalRouteTokenDrift => "native_reapproval_route_token_drift",
            Self::TargetClassTokenDrift => "target_class_token_drift",
            Self::ActorClassTokenDrift => "actor_class_token_drift",
            Self::AuthSourceClassTokenDrift => "auth_source_class_token_drift",
            Self::CapabilityClassTokensDrift => "capability_class_tokens_drift",
            Self::SideEffectTokensDrift => "side_effect_tokens_drift",
            Self::EnvelopeSandboxClassMismatch => "envelope_sandbox_class_mismatch",
            Self::EnvelopeCapabilityOutsideSandbox => "envelope_capability_outside_sandbox",
            Self::EnvelopeSideEffectOutsideSandbox => "envelope_side_effect_outside_sandbox",
            Self::EnvelopeSandboxRefUnknown => "envelope_sandbox_ref_unknown",
            Self::TicketSandboxRefUnknown => "ticket_sandbox_ref_unknown",
            Self::TicketEnvelopeRefUnknown => "ticket_envelope_ref_unknown",
            Self::TicketLifetimeExceedsSandboxBudget => "ticket_lifetime_exceeds_sandbox_budget",
            Self::TicketLifetimeMismatch => "ticket_lifetime_mismatch",
            Self::TicketExpiryNotAfterIssuance => "ticket_expiry_not_after_issuance",
            Self::SpendTicketRefUnknown => "spend_ticket_ref_unknown",
            Self::SpendAdmittedUnderDrift => "spend_admitted_under_drift",
            Self::SpendDenialMissingAuditRef => "spend_denial_missing_audit_ref",
            Self::SpendDenialReapprovalRouteCollapsed => "spend_denial_reapproval_route_collapsed",
            Self::SpendAdmittedReapprovalRouteUnexpected => {
                "spend_admitted_reapproval_route_unexpected"
            }
            Self::SelfAuthorizationDenialOnIntrinsicIssuer => {
                "self_authorization_denial_on_intrinsic_issuer"
            }
            Self::MissingAuthorityCarriedTicketRef => "missing_authority_carried_ticket_ref",
            Self::ExpiredSpendNotAfterExpiry => "expired_spend_not_after_expiry",
            Self::TargetDriftMatchesTicketTarget => "target_drift_matches_ticket_target",
            Self::SandboxDriftMatchesTicketSandbox => "sandbox_drift_matches_ticket_sandbox",
            Self::PolicyEpochDriftMatchesTicketEpoch => "policy_epoch_drift_matches_ticket_epoch",
            Self::TrustProfileDriftMatchesTicketTrust => {
                "trust_profile_drift_matches_ticket_trust"
            }
            Self::EnvelopeDriftMatchesTicketEnvelope => "envelope_drift_matches_ticket_envelope",
            Self::ProfileCoverageMissing => "profile_coverage_missing",
            Self::SandboxProfileCoverageMissing => "sandbox_profile_coverage_missing",
        }
    }
}

/// Typed validation defect for the approval-ticket beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalTicketBetaDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Defect kind.
    pub defect_kind: ApprovalTicketBetaDefectKind,
    /// Stable token for [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Subject id (row id, ticket id, spend id, or `"page"`).
    pub subject_id: String,
    /// Field that failed validation.
    pub field: String,
    /// Export-safe explanation.
    pub note: String,
}

impl ApprovalTicketBetaDefect {
    fn new(
        defect_kind: ApprovalTicketBetaDefectKind,
        subject_id: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: APPROVAL_TICKET_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: APPROVAL_TICKET_BETA_SCHEMA_VERSION,
            shared_contract_ref: APPROVAL_TICKET_BETA_SHARED_CONTRACT_REF.to_owned(),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            subject_id: subject_id.into(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Aggregate summary for the approval-ticket beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalTicketBetaSummary {
    /// Stable record kind of the parent page.
    pub page_record_kind: String,
    /// Stable record kind of the summary.
    pub record_kind: String,
    /// Number of sandbox-profile rows.
    pub sandbox_profile_row_count: usize,
    /// Number of capability-envelope rows.
    pub capability_envelope_row_count: usize,
    /// Number of issued approval-ticket rows.
    pub ticket_row_count: usize,
    /// Number of spend-attempt events.
    pub spend_attempt_event_count: usize,
    /// Profile tokens present across the page.
    pub profiles_present: Vec<String>,
    /// Sandbox-profile class tokens present across the page.
    pub sandbox_profile_classes_present: Vec<String>,
    /// Action class tokens present across the page.
    pub action_classes_present: Vec<String>,
    /// Side-effect class tokens present across the page.
    pub side_effect_classes_present: Vec<String>,
    /// Issuer class tokens present across the page.
    pub issuer_classes_present: Vec<String>,
    /// Request-origin class tokens present across the page.
    pub request_origin_classes_present: Vec<String>,
    /// Counts of spend attempts by evaluation-outcome token.
    pub spend_attempts_by_outcome: BTreeMap<String, usize>,
    /// Counts of spend attempts by native-reapproval-route token.
    pub spend_attempts_by_reapproval_route: BTreeMap<String, usize>,
    /// Number of defects.
    pub defect_count: usize,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
}

impl ApprovalTicketBetaSummary {
    /// Builds the summary from rows, envelopes, tickets, spend events, and
    /// defects.
    pub fn from_records(
        sandbox_profile_rows: &[SandboxProfileRow],
        capability_envelope_rows: &[CapabilityEnvelopeRow],
        ticket_rows: &[IssuedApprovalTicketRow],
        spend_attempt_events: &[SpendAttemptEvent],
        defects: &[ApprovalTicketBetaDefect],
    ) -> Self {
        let mut profiles_present: BTreeSet<String> = BTreeSet::new();
        let mut sandbox_profile_classes_present: BTreeSet<String> = BTreeSet::new();
        let mut action_classes_present: BTreeSet<String> = BTreeSet::new();
        let mut side_effect_classes_present: BTreeSet<String> = BTreeSet::new();
        let mut issuer_classes_present: BTreeSet<String> = BTreeSet::new();
        let mut request_origin_classes_present: BTreeSet<String> = BTreeSet::new();

        for row in sandbox_profile_rows {
            profiles_present.insert(row.profile_token.clone());
            sandbox_profile_classes_present.insert(row.sandbox_profile_class_token.clone());
            for token in &row.allowed_side_effect_class_tokens {
                side_effect_classes_present.insert(token.clone());
            }
        }
        for row in capability_envelope_rows {
            profiles_present.insert(row.profile_token.clone());
            action_classes_present.insert(row.action_class_token.clone());
            side_effect_classes_present.insert(row.side_effect_class_token.clone());
        }
        for ticket in ticket_rows {
            profiles_present.insert(ticket.profile_token.clone());
            issuer_classes_present.insert(ticket.issuer_class_token.clone());
            request_origin_classes_present.insert(ticket.request_origin_class_token.clone());
            action_classes_present.insert(ticket.action_class_token.clone());
            side_effect_classes_present.insert(ticket.side_effect_class_token.clone());
        }
        for event in spend_attempt_events {
            profiles_present.insert(event.profile_token.clone());
        }

        let mut spend_attempts_by_outcome: BTreeMap<String, usize> = BTreeMap::new();
        let mut spend_attempts_by_reapproval_route: BTreeMap<String, usize> = BTreeMap::new();
        for event in spend_attempt_events {
            *spend_attempts_by_outcome
                .entry(event.evaluation_outcome_token.clone())
                .or_insert(0) += 1;
            *spend_attempts_by_reapproval_route
                .entry(event.native_reapproval_route_token.clone())
                .or_insert(0) += 1;
        }

        let mut defect_counts_by_kind: BTreeMap<String, usize> = BTreeMap::new();
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind_token.clone())
                .or_insert(0) += 1;
        }

        Self {
            page_record_kind: APPROVAL_TICKET_BETA_PAGE_RECORD_KIND.to_owned(),
            record_kind: APPROVAL_TICKET_BETA_SUMMARY_RECORD_KIND.to_owned(),
            sandbox_profile_row_count: sandbox_profile_rows.len(),
            capability_envelope_row_count: capability_envelope_rows.len(),
            ticket_row_count: ticket_rows.len(),
            spend_attempt_event_count: spend_attempt_events.len(),
            profiles_present: profiles_present.into_iter().collect(),
            sandbox_profile_classes_present: sandbox_profile_classes_present.into_iter().collect(),
            action_classes_present: action_classes_present.into_iter().collect(),
            side_effect_classes_present: side_effect_classes_present.into_iter().collect(),
            issuer_classes_present: issuer_classes_present.into_iter().collect(),
            request_origin_classes_present: request_origin_classes_present.into_iter().collect(),
            spend_attempts_by_outcome,
            spend_attempts_by_reapproval_route,
            defect_count: defects.len(),
            defect_counts_by_kind,
        }
    }
}

/// Top-level approval-ticket beta page consumed by admin, support, shell, and
/// reviewer fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalTicketBetaPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Source matrix ref.
    pub source_matrix_ref: String,
    /// Sandbox-profile rows.
    pub sandbox_profile_rows: Vec<SandboxProfileRow>,
    /// Capability-envelope rows.
    pub capability_envelope_rows: Vec<CapabilityEnvelopeRow>,
    /// Issued approval-ticket rows.
    pub ticket_rows: Vec<IssuedApprovalTicketRow>,
    /// Spend-attempt events.
    pub spend_attempt_events: Vec<SpendAttemptEvent>,
    /// Typed validation defects.
    pub defects: Vec<ApprovalTicketBetaDefect>,
    /// Aggregate summary.
    pub summary: ApprovalTicketBetaSummary,
}

/// Support-export wrapper for the approval-ticket beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalTicketBetaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Exported page.
    pub page: ApprovalTicketBetaPage,
    /// Defect-kind tokens present in the page.
    pub defect_kinds_present: Vec<String>,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True when raw authority bodies are excluded from the export.
    pub raw_authority_material_excluded: bool,
    /// True when authority lineage (sandbox profile, capability envelope,
    /// ticket, spend attempt, audit-event refs) is preserved verbatim.
    pub authority_lineage_preserved: bool,
    /// True when the export proves the no-self-authorization invariant.
    pub no_self_authorization_invariant: bool,
    /// Reviewable summary of the redaction posture.
    pub redaction_summary: String,
}

impl ApprovalTicketBetaSupportExport {
    /// Builds a support-export wrapper from a beta page.
    pub fn from_page(
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
        page: ApprovalTicketBetaPage,
    ) -> Self {
        let defect_counts_by_kind = page.summary.defect_counts_by_kind.clone();
        let defect_kinds_present = defect_counts_by_kind.keys().cloned().collect();
        Self {
            record_kind: APPROVAL_TICKET_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: APPROVAL_TICKET_BETA_SCHEMA_VERSION,
            shared_contract_ref: APPROVAL_TICKET_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            page,
            defect_kinds_present,
            defect_counts_by_kind,
            raw_authority_material_excluded: true,
            authority_lineage_preserved: true,
            no_self_authorization_invariant: true,
            redaction_summary:
                "Metadata-only approval-ticket beta export: sandbox-profile rows, capability \
                 envelopes, issued tickets, spend attempts, and audit-event refs are preserved; \
                 raw authority bodies, raw evidence bodies, raw delegated-token bodies, and \
                 plaintext secret material are excluded because the beta projection never \
                 carries them."
                    .to_owned(),
        }
    }
}

fn check_guardrails(
    defects: &mut Vec<ApprovalTicketBetaDefect>,
    subject_id: &str,
    guardrails: &BetaGuardrails,
) {
    if guardrails.raw_authority_material_present {
        defects.push(ApprovalTicketBetaDefect::new(
            ApprovalTicketBetaDefectKind::RawAuthorityMaterialPresent,
            subject_id,
            "guardrails.raw_authority_material_present",
            "claimed beta record must not carry raw authority material",
        ));
    }
    if guardrails.self_authorization_attempted {
        defects.push(ApprovalTicketBetaDefect::new(
            ApprovalTicketBetaDefectKind::SelfAuthorizationAttempted,
            subject_id,
            "guardrails.self_authorization_attempted",
            "claimed beta record must not record a self-authorization attempt",
        ));
    }
    if guardrails.silent_widening_attempted {
        defects.push(ApprovalTicketBetaDefect::new(
            ApprovalTicketBetaDefectKind::SilentWideningAttempted,
            subject_id,
            "guardrails.silent_widening_attempted",
            "claimed beta record must not record a silent authority widening",
        ));
    }
    if guardrails.plaintext_secret_present {
        defects.push(ApprovalTicketBetaDefect::new(
            ApprovalTicketBetaDefectKind::PlaintextSecretPresent,
            subject_id,
            "guardrails.plaintext_secret_present",
            "claimed beta record must not carry plaintext secret material",
        ));
    }
    if guardrails.public_endpoint_fallback_offered {
        defects.push(ApprovalTicketBetaDefect::new(
            ApprovalTicketBetaDefectKind::PublicEndpointFallbackOffered,
            subject_id,
            "guardrails.public_endpoint_fallback_offered",
            "claimed beta record must not offer an undeclared public-endpoint fallback",
        ));
    }
    if !guardrails.local_editing_preserved {
        defects.push(ApprovalTicketBetaDefect::new(
            ApprovalTicketBetaDefectKind::LocalEditingNotPreserved,
            subject_id,
            "guardrails.local_editing_preserved",
            "claimed beta record must preserve local editing",
        ));
    }
}

fn capability_tokens_match(
    classes: &[CapabilityClass],
    tokens: &[String],
) -> bool {
    if classes.len() != tokens.len() {
        return false;
    }
    classes
        .iter()
        .zip(tokens.iter())
        .all(|(class, token)| class.as_str() == token.as_str())
}

fn side_effect_tokens_match(
    classes: &[SideEffectClass],
    tokens: &[String],
) -> bool {
    if classes.len() != tokens.len() {
        return false;
    }
    classes
        .iter()
        .zip(tokens.iter())
        .all(|(class, token)| class.as_str() == token.as_str())
}

fn check_actor_scope(
    defects: &mut Vec<ApprovalTicketBetaDefect>,
    subject_id: &str,
    scope: &ActorScope,
) {
    if scope.actor_class_token != scope.actor_class.as_str() {
        defects.push(ApprovalTicketBetaDefect::new(
            ApprovalTicketBetaDefectKind::ActorClassTokenDrift,
            subject_id,
            "actor_scope.actor_class_token",
            "actor_class_token must match actor_class",
        ));
    }
    if scope.auth_source_class_token != scope.auth_source_class.as_str() {
        defects.push(ApprovalTicketBetaDefect::new(
            ApprovalTicketBetaDefectKind::AuthSourceClassTokenDrift,
            subject_id,
            "actor_scope.auth_source_class_token",
            "auth_source_class_token must match auth_source_class",
        ));
    }
}

fn check_target_identity(
    defects: &mut Vec<ApprovalTicketBetaDefect>,
    subject_id: &str,
    target: &TargetIdentity,
) {
    if target.target_class_token != target.target_class.as_str() {
        defects.push(ApprovalTicketBetaDefect::new(
            ApprovalTicketBetaDefectKind::TargetClassTokenDrift,
            subject_id,
            "target_identity.target_class_token",
            "target_class_token must match target_class",
        ));
    }
}

/// Validates the approval-ticket beta page and returns typed defects on
/// failure.
pub fn validate_approval_ticket_beta_page(
    page: &ApprovalTicketBetaPage,
) -> Result<(), Vec<ApprovalTicketBetaDefect>> {
    let defects = audit_approval_ticket_beta_page(
        &page.sandbox_profile_rows,
        &page.capability_envelope_rows,
        &page.ticket_rows,
        &page.spend_attempt_events,
    );
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Recomputes defects for an approval-ticket beta page.
pub fn audit_approval_ticket_beta_page(
    sandbox_profile_rows: &[SandboxProfileRow],
    capability_envelope_rows: &[CapabilityEnvelopeRow],
    ticket_rows: &[IssuedApprovalTicketRow],
    spend_attempt_events: &[SpendAttemptEvent],
) -> Vec<ApprovalTicketBetaDefect> {
    let mut defects = Vec::new();

    let sandbox_by_ref: BTreeMap<&str, &SandboxProfileRow> = sandbox_profile_rows
        .iter()
        .map(|row| (row.sandbox_profile_ref.as_str(), row))
        .collect();
    let envelope_by_id: BTreeMap<&str, &CapabilityEnvelopeRow> = capability_envelope_rows
        .iter()
        .map(|row| (row.envelope_id.as_str(), row))
        .collect();
    let ticket_by_id: BTreeMap<&str, &IssuedApprovalTicketRow> = ticket_rows
        .iter()
        .map(|row| (row.approval_ticket_id.as_str(), row))
        .collect();

    for row in sandbox_profile_rows {
        if row.profile_token != row.profile.as_str() {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::ProfileTokenDrift,
                row.sandbox_profile_row_id.clone(),
                "profile_token",
                "profile_token must match profile",
            ));
        }
        if row.sandbox_profile_class_token != row.sandbox_profile_class.as_str() {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::SandboxProfileClassTokenDrift,
                row.sandbox_profile_row_id.clone(),
                "sandbox_profile_class_token",
                "sandbox_profile_class_token must match sandbox_profile_class",
            ));
        }
        if row.default_use_posture_token != row.default_use_posture.as_str() {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::UsePostureTokenDrift,
                row.sandbox_profile_row_id.clone(),
                "default_use_posture_token",
                "default_use_posture_token must match default_use_posture",
            ));
        }
        if !capability_tokens_match(
            &row.allowed_capability_classes,
            &row.allowed_capability_class_tokens,
        ) {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::CapabilityClassTokensDrift,
                row.sandbox_profile_row_id.clone(),
                "allowed_capability_class_tokens",
                "allowed_capability_class_tokens must match allowed_capability_classes",
            ));
        }
        if !side_effect_tokens_match(
            &row.allowed_side_effect_classes,
            &row.allowed_side_effect_class_tokens,
        ) {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::SideEffectTokensDrift,
                row.sandbox_profile_row_id.clone(),
                "allowed_side_effect_class_tokens",
                "allowed_side_effect_class_tokens must match allowed_side_effect_classes",
            ));
        }
        check_guardrails(
            &mut defects,
            row.sandbox_profile_row_id.as_str(),
            &row.guardrails,
        );
    }

    for envelope in capability_envelope_rows {
        if envelope.profile_token != envelope.profile.as_str() {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::ProfileTokenDrift,
                envelope.capability_envelope_row_id.clone(),
                "profile_token",
                "profile_token must match profile",
            ));
        }
        if envelope.action_class_token != envelope.action_class.as_str() {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::ActionClassTokenDrift,
                envelope.capability_envelope_row_id.clone(),
                "action_class_token",
                "action_class_token must match action_class",
            ));
        }
        if envelope.side_effect_class_token != envelope.side_effect_class.as_str() {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::SideEffectClassTokenDrift,
                envelope.capability_envelope_row_id.clone(),
                "side_effect_class_token",
                "side_effect_class_token must match side_effect_class",
            ));
        }
        if !capability_tokens_match(
            &envelope.allowed_capability_classes,
            &envelope.allowed_capability_class_tokens,
        ) {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::CapabilityClassTokensDrift,
                envelope.capability_envelope_row_id.clone(),
                "allowed_capability_class_tokens",
                "allowed_capability_class_tokens must match allowed_capability_classes",
            ));
        }
        check_actor_scope(
            &mut defects,
            envelope.capability_envelope_row_id.as_str(),
            &envelope.actor_scope,
        );
        check_target_identity(
            &mut defects,
            envelope.capability_envelope_row_id.as_str(),
            &envelope.target_identity,
        );
        check_guardrails(
            &mut defects,
            envelope.capability_envelope_row_id.as_str(),
            &envelope.guardrails,
        );

        let sandbox = sandbox_by_ref.get(envelope.sandbox_profile_ref.as_str());
        match sandbox {
            None => {
                defects.push(ApprovalTicketBetaDefect::new(
                    ApprovalTicketBetaDefectKind::EnvelopeSandboxRefUnknown,
                    envelope.capability_envelope_row_id.clone(),
                    "sandbox_profile_ref",
                    "envelope sandbox_profile_ref must reference a known sandbox profile row",
                ));
            }
            Some(sandbox) => {
                if envelope.action_class.required_sandbox_profile()
                    != sandbox.sandbox_profile_class
                {
                    defects.push(ApprovalTicketBetaDefect::new(
                        ApprovalTicketBetaDefectKind::EnvelopeSandboxClassMismatch,
                        envelope.capability_envelope_row_id.clone(),
                        "sandbox_profile_ref/action_class",
                        "envelope action_class does not match the sandbox profile's class",
                    ));
                }
                let allowed_capabilities: BTreeSet<&str> = sandbox
                    .allowed_capability_classes
                    .iter()
                    .map(|class| class.as_str())
                    .collect();
                for class in &envelope.allowed_capability_classes {
                    if !allowed_capabilities.contains(class.as_str()) {
                        defects.push(ApprovalTicketBetaDefect::new(
                            ApprovalTicketBetaDefectKind::EnvelopeCapabilityOutsideSandbox,
                            envelope.capability_envelope_row_id.clone(),
                            "allowed_capability_classes",
                            "envelope admits a capability class outside the sandbox profile",
                        ));
                        break;
                    }
                }
                let allowed_side_effects: BTreeSet<&str> = sandbox
                    .allowed_side_effect_classes
                    .iter()
                    .map(|class| class.as_str())
                    .collect();
                if !allowed_side_effects.contains(envelope.side_effect_class.as_str()) {
                    defects.push(ApprovalTicketBetaDefect::new(
                        ApprovalTicketBetaDefectKind::EnvelopeSideEffectOutsideSandbox,
                        envelope.capability_envelope_row_id.clone(),
                        "side_effect_class",
                        "envelope side_effect_class outside sandbox profile allowed set",
                    ));
                }
            }
        }
    }

    for ticket in ticket_rows {
        if ticket.profile_token != ticket.profile.as_str() {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::ProfileTokenDrift,
                ticket.approval_ticket_id.clone(),
                "profile_token",
                "profile_token must match profile",
            ));
        }
        if ticket.issuer_class_token != ticket.issuer_class.as_str() {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::IssuerClassTokenDrift,
                ticket.approval_ticket_id.clone(),
                "issuer_class_token",
                "issuer_class_token must match issuer_class",
            ));
        }
        if ticket.request_origin_class_token != ticket.request_origin_class.as_str() {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::RequestOriginClassTokenDrift,
                ticket.approval_ticket_id.clone(),
                "request_origin_class_token",
                "request_origin_class_token must match request_origin_class",
            ));
        }
        if ticket.action_class_token != ticket.action_class.as_str() {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::ActionClassTokenDrift,
                ticket.approval_ticket_id.clone(),
                "action_class_token",
                "action_class_token must match action_class",
            ));
        }
        if ticket.side_effect_class_token != ticket.side_effect_class.as_str() {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::SideEffectClassTokenDrift,
                ticket.approval_ticket_id.clone(),
                "side_effect_class_token",
                "side_effect_class_token must match side_effect_class",
            ));
        }
        if ticket.use_posture_token != ticket.use_posture.as_str() {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::UsePostureTokenDrift,
                ticket.approval_ticket_id.clone(),
                "use_posture_token",
                "use_posture_token must match use_posture",
            ));
        }
        if ticket.authority_requirement_token != ticket.authority_requirement.as_str() {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::AuthorityRequirementTokenDrift,
                ticket.approval_ticket_id.clone(),
                "authority_requirement_token",
                "authority_requirement_token must match authority_requirement",
            ));
        }

        check_actor_scope(
            &mut defects,
            ticket.approval_ticket_id.as_str(),
            &ticket.actor_scope,
        );
        check_target_identity(
            &mut defects,
            ticket.approval_ticket_id.as_str(),
            &ticket.target_identity,
        );
        check_guardrails(
            &mut defects,
            ticket.approval_ticket_id.as_str(),
            &ticket.guardrails,
        );

        if !ticket.request_origin_class.is_intrinsic_issuer()
            && ticket
                .requesting_surface_ref
                .as_deref()
                .map(str::is_empty)
                .unwrap_or(true)
        {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::MissingRequestingSurfaceRef,
                ticket.approval_ticket_id.clone(),
                "requesting_surface_ref",
                "non-intrinsic request origin must carry a requesting_surface_ref",
            ));
        }

        let sandbox = sandbox_by_ref.get(ticket.sandbox_profile_ref.as_str());
        if sandbox.is_none() {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::TicketSandboxRefUnknown,
                ticket.approval_ticket_id.clone(),
                "sandbox_profile_ref",
                "ticket sandbox_profile_ref must reference a known sandbox profile row",
            ));
        }
        let envelope = envelope_by_id.get(ticket.capability_envelope_ref.as_str());
        if envelope.is_none() {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::TicketEnvelopeRefUnknown,
                ticket.approval_ticket_id.clone(),
                "capability_envelope_ref",
                "ticket capability_envelope_ref must reference a known envelope row",
            ));
        }

        match (parse_timestamp(&ticket.issued_at), parse_timestamp(&ticket.expires_at)) {
            (Some(issued), Some(expires)) => {
                if expires <= issued {
                    defects.push(ApprovalTicketBetaDefect::new(
                        ApprovalTicketBetaDefectKind::TicketExpiryNotAfterIssuance,
                        ticket.approval_ticket_id.clone(),
                        "expires_at",
                        "ticket expires_at must be strictly after issued_at",
                    ));
                } else {
                    let actual = (expires - issued) as u64;
                    if actual != ticket.lifetime_seconds {
                        defects.push(ApprovalTicketBetaDefect::new(
                            ApprovalTicketBetaDefectKind::TicketLifetimeMismatch,
                            ticket.approval_ticket_id.clone(),
                            "lifetime_seconds",
                            "lifetime_seconds must equal expires_at - issued_at",
                        ));
                    }
                    if let Some(sandbox) = sandbox {
                        if actual > sandbox.max_ticket_lifetime_seconds {
                            defects.push(ApprovalTicketBetaDefect::new(
                                ApprovalTicketBetaDefectKind::TicketLifetimeExceedsSandboxBudget,
                                ticket.approval_ticket_id.clone(),
                                "expires_at",
                                "ticket lifetime exceeds sandbox max_ticket_lifetime_seconds",
                            ));
                        }
                    }
                }
            }
            _ => {
                // Unparseable timestamps surface a token-drift-style defect via the
                // schema validator; we leave that to the JSON-schema layer.
            }
        }
    }

    for event in spend_attempt_events {
        if event.profile_token != event.profile.as_str() {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::ProfileTokenDrift,
                event.spend_attempt_id.clone(),
                "profile_token",
                "profile_token must match profile",
            ));
        }
        if event.evaluation_outcome_token != event.evaluation_outcome.as_str() {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::EvaluationOutcomeTokenDrift,
                event.spend_attempt_id.clone(),
                "evaluation_outcome_token",
                "evaluation_outcome_token must match evaluation_outcome",
            ));
        }
        if event.native_reapproval_route_token != event.native_reapproval_route.as_str() {
            defects.push(ApprovalTicketBetaDefect::new(
                ApprovalTicketBetaDefectKind::NativeReapprovalRouteTokenDrift,
                event.spend_attempt_id.clone(),
                "native_reapproval_route_token",
                "native_reapproval_route_token must match native_reapproval_route",
            ));
        }

        check_actor_scope(
            &mut defects,
            event.spend_attempt_id.as_str(),
            &event.current_actor_scope,
        );
        check_target_identity(
            &mut defects,
            event.spend_attempt_id.as_str(),
            &event.current_target_identity,
        );

        let presented = event
            .presented_approval_ticket_ref
            .as_deref()
            .and_then(|id| ticket_by_id.get(id).copied());

        if matches!(
            event.evaluation_outcome,
            EvaluationOutcome::DeniedMissingAuthority
        ) {
            if event.presented_approval_ticket_ref.is_some() {
                defects.push(ApprovalTicketBetaDefect::new(
                    ApprovalTicketBetaDefectKind::MissingAuthorityCarriedTicketRef,
                    event.spend_attempt_id.clone(),
                    "presented_approval_ticket_ref",
                    "denied_missing_authority must not carry a presented_approval_ticket_ref",
                ));
            }
        } else if let Some(ref presented_ref) = event.presented_approval_ticket_ref {
            if presented.is_none() {
                defects.push(ApprovalTicketBetaDefect::new(
                    ApprovalTicketBetaDefectKind::SpendTicketRefUnknown,
                    event.spend_attempt_id.clone(),
                    "presented_approval_ticket_ref",
                    format!("presented ticket ref {presented_ref} not present on page"),
                ));
            }
        }

        if event.evaluation_outcome.is_admitted() {
            if !matches!(event.native_reapproval_route, NativeReapprovalRoute::NotRequired) {
                defects.push(ApprovalTicketBetaDefect::new(
                    ApprovalTicketBetaDefectKind::SpendAdmittedReapprovalRouteUnexpected,
                    event.spend_attempt_id.clone(),
                    "native_reapproval_route",
                    "admitted spend must declare native_reapproval_route=not_required",
                ));
            }
            if let Some(ticket) = presented {
                if ticket.target_identity.target_ref != event.current_target_identity.target_ref
                    || ticket.target_identity.target_fingerprint_ref
                        != event.current_target_identity.target_fingerprint_ref
                    || ticket.target_identity.target_version_ref
                        != event.current_target_identity.target_version_ref
                    || ticket.sandbox_profile_ref != event.current_sandbox_profile_ref
                    || ticket.capability_envelope_ref != event.current_capability_envelope_ref
                    || ticket.trust_profile_ref != event.current_trust_profile_ref
                    || ticket.policy_epoch_ref != event.current_policy_epoch_ref
                    || ticket.actor_scope.actor_subject_ref
                        != event.current_actor_scope.actor_subject_ref
                {
                    defects.push(ApprovalTicketBetaDefect::new(
                        ApprovalTicketBetaDefectKind::SpendAdmittedUnderDrift,
                        event.spend_attempt_id.clone(),
                        "evaluation_outcome",
                        "admitted spend must match the ticket on every authority axis",
                    ));
                }
            }
        } else {
            if matches!(event.native_reapproval_route, NativeReapprovalRoute::NotRequired) {
                defects.push(ApprovalTicketBetaDefect::new(
                    ApprovalTicketBetaDefectKind::SpendDenialReapprovalRouteCollapsed,
                    event.spend_attempt_id.clone(),
                    "native_reapproval_route",
                    "denied spend must not collapse native_reapproval_route to not_required",
                ));
            }
            if event.audit_event_refs.is_empty()
                || event
                    .audit_event_refs
                    .iter()
                    .all(|reference| reference.is_empty())
            {
                defects.push(ApprovalTicketBetaDefect::new(
                    ApprovalTicketBetaDefectKind::SpendDenialMissingAuditRef,
                    event.spend_attempt_id.clone(),
                    "audit_event_refs",
                    "denied spend must declare at least one audit_event_ref",
                ));
            }
        }

        if let Some(ticket) = presented {
            match event.evaluation_outcome {
                EvaluationOutcome::DeniedExpired => {
                    if let (Some(evaluated), Some(expires)) = (
                        parse_timestamp(&event.evaluated_at),
                        parse_timestamp(&ticket.expires_at),
                    ) {
                        if evaluated <= expires {
                            defects.push(ApprovalTicketBetaDefect::new(
                                ApprovalTicketBetaDefectKind::ExpiredSpendNotAfterExpiry,
                                event.spend_attempt_id.clone(),
                                "evaluated_at",
                                "denied_expired must have evaluated_at > ticket.expires_at",
                            ));
                        }
                    }
                }
                EvaluationOutcome::DeniedTargetDrift => {
                    if ticket.target_identity.target_ref == event.current_target_identity.target_ref
                        && ticket.target_identity.target_fingerprint_ref
                            == event.current_target_identity.target_fingerprint_ref
                        && ticket.target_identity.target_version_ref
                            == event.current_target_identity.target_version_ref
                    {
                        defects.push(ApprovalTicketBetaDefect::new(
                            ApprovalTicketBetaDefectKind::TargetDriftMatchesTicketTarget,
                            event.spend_attempt_id.clone(),
                            "current_target_identity",
                            "denied_target_drift must change the current target identity",
                        ));
                    }
                }
                EvaluationOutcome::DeniedSandboxProfileDrift => {
                    if ticket.sandbox_profile_ref == event.current_sandbox_profile_ref {
                        defects.push(ApprovalTicketBetaDefect::new(
                            ApprovalTicketBetaDefectKind::SandboxDriftMatchesTicketSandbox,
                            event.spend_attempt_id.clone(),
                            "current_sandbox_profile_ref",
                            "denied_sandbox_profile_drift must change the current sandbox_profile_ref",
                        ));
                    }
                }
                EvaluationOutcome::DeniedPolicyEpochDrift => {
                    if ticket.policy_epoch_ref == event.current_policy_epoch_ref {
                        defects.push(ApprovalTicketBetaDefect::new(
                            ApprovalTicketBetaDefectKind::PolicyEpochDriftMatchesTicketEpoch,
                            event.spend_attempt_id.clone(),
                            "current_policy_epoch_ref",
                            "denied_policy_epoch_drift must change the current policy_epoch_ref",
                        ));
                    }
                }
                EvaluationOutcome::DeniedTrustProfileDrift => {
                    if ticket.trust_profile_ref == event.current_trust_profile_ref {
                        defects.push(ApprovalTicketBetaDefect::new(
                            ApprovalTicketBetaDefectKind::TrustProfileDriftMatchesTicketTrust,
                            event.spend_attempt_id.clone(),
                            "current_trust_profile_ref",
                            "denied_trust_profile_drift must change the current trust_profile_ref",
                        ));
                    }
                }
                EvaluationOutcome::DeniedCapabilityEnvelopeDrift => {
                    if ticket.capability_envelope_ref == event.current_capability_envelope_ref {
                        defects.push(ApprovalTicketBetaDefect::new(
                            ApprovalTicketBetaDefectKind::EnvelopeDriftMatchesTicketEnvelope,
                            event.spend_attempt_id.clone(),
                            "current_capability_envelope_ref",
                            "denied_capability_envelope_drift must change the envelope ref",
                        ));
                    }
                }
                EvaluationOutcome::DeniedSelfAuthorizationAttempted => {
                    if ticket.request_origin_class.is_intrinsic_issuer() {
                        defects.push(ApprovalTicketBetaDefect::new(
                            ApprovalTicketBetaDefectKind::SelfAuthorizationDenialOnIntrinsicIssuer,
                            event.spend_attempt_id.clone(),
                            "evaluation_outcome",
                            "denied_self_authorization_attempted may not target a ticket minted by an intrinsic issuer surface",
                        ));
                    }
                }
                _ => {}
            }
        }
    }

    let mut observed_profiles: BTreeSet<&str> = BTreeSet::new();
    for row in sandbox_profile_rows {
        observed_profiles.insert(row.profile_token.as_str());
    }
    for row in capability_envelope_rows {
        observed_profiles.insert(row.profile_token.as_str());
    }
    for row in ticket_rows {
        observed_profiles.insert(row.profile_token.as_str());
    }
    let required_profiles: BTreeSet<&str> = ApprovalTicketBetaProfileClass::ALL
        .iter()
        .map(|profile| profile.as_str())
        .collect();
    for missing in required_profiles.difference(&observed_profiles) {
        defects.push(ApprovalTicketBetaDefect::new(
            ApprovalTicketBetaDefectKind::ProfileCoverageMissing,
            "page",
            "profiles",
            format!("missing {} profile coverage", missing),
        ));
    }

    let observed_sandbox_classes: BTreeSet<&str> = sandbox_profile_rows
        .iter()
        .map(|row| row.sandbox_profile_class_token.as_str())
        .collect();
    let required_sandbox_classes: BTreeSet<&str> = SandboxProfileClass::ALL
        .iter()
        .map(|class| class.as_str())
        .collect();
    for missing in required_sandbox_classes.difference(&observed_sandbox_classes) {
        defects.push(ApprovalTicketBetaDefect::new(
            ApprovalTicketBetaDefectKind::SandboxProfileCoverageMissing,
            "page",
            "sandbox_profile_classes",
            format!("missing {} sandbox-profile class coverage", missing),
        ));
    }

    defects
}

/// Minimal RFC 3339 parser that returns the timestamp as seconds since the
/// Unix epoch. The parser is deliberately strict: timestamps must use the
/// `YYYY-MM-DDTHH:MM:SSZ` shape (UTC, second precision) so seed fixtures and
/// drift drills stay reviewer-legible without pulling chrono into this crate.
fn parse_timestamp(value: &str) -> Option<i64> {
    let bytes = value.as_bytes();
    if bytes.len() != 20 {
        return None;
    }
    if bytes[4] != b'-'
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
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    if hour > 23 || minute > 59 || second > 60 {
        return None;
    }
    let days_per_month: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let is_leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let mut days_before_year: i64 = 0;
    for y in 1970..year {
        days_before_year += 365;
        if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 {
            days_before_year += 1;
        }
    }
    let mut days_before_month: i64 = 0;
    for (m, &count) in days_per_month.iter().enumerate() {
        if (m as i64) + 1 >= month {
            break;
        }
        days_before_month += count;
        if m == 1 && is_leap {
            days_before_month += 1;
        }
    }
    let day_of_year = days_before_month + (day - 1);
    let total_days = days_before_year + day_of_year;
    Some(total_days * 86_400 + hour * 3600 + minute * 60 + second)
}

/// Builds the seeded approval-ticket beta page covering connected, mirror,
/// offline, and enterprise-managed profiles, every sandbox-profile class, and
/// admitted plus six denial outcomes.
pub fn seeded_approval_ticket_beta_page() -> ApprovalTicketBetaPage {
    let sandbox_profile_rows = seed_sandbox_profile_rows();
    let capability_envelope_rows = seed_capability_envelope_rows();
    let ticket_rows = seed_ticket_rows();
    let spend_attempt_events = seed_spend_attempt_events();

    let defects = audit_approval_ticket_beta_page(
        &sandbox_profile_rows,
        &capability_envelope_rows,
        &ticket_rows,
        &spend_attempt_events,
    );
    let summary = ApprovalTicketBetaSummary::from_records(
        &sandbox_profile_rows,
        &capability_envelope_rows,
        &ticket_rows,
        &spend_attempt_events,
        &defects,
    );

    ApprovalTicketBetaPage {
        record_kind: APPROVAL_TICKET_BETA_PAGE_RECORD_KIND.to_owned(),
        schema_version: APPROVAL_TICKET_BETA_SCHEMA_VERSION,
        shared_contract_ref: APPROVAL_TICKET_BETA_SHARED_CONTRACT_REF.to_owned(),
        source_matrix_ref: APPROVAL_TICKET_BETA_SOURCE_MATRIX_REF.to_owned(),
        sandbox_profile_rows,
        capability_envelope_rows,
        ticket_rows,
        spend_attempt_events,
        defects,
        summary,
    }
}

#[allow(clippy::too_many_arguments)]
fn sandbox_row(
    row_id: &str,
    display_label: &str,
    profile: ApprovalTicketBetaProfileClass,
    sandbox_profile_class: SandboxProfileClass,
    sandbox_profile_ref: &str,
    trust_profile_ref: &str,
    policy_epoch_ref: &str,
    allowed_capability_classes: Vec<CapabilityClass>,
    allowed_side_effect_classes: Vec<SideEffectClass>,
    default_use_posture: UsePosture,
    max_ticket_lifetime_seconds: u64,
) -> SandboxProfileRow {
    SandboxProfileRow {
        record_kind: APPROVAL_TICKET_BETA_SANDBOX_PROFILE_ROW_RECORD_KIND.to_owned(),
        schema_version: APPROVAL_TICKET_BETA_SCHEMA_VERSION,
        shared_contract_ref: APPROVAL_TICKET_BETA_SHARED_CONTRACT_REF.to_owned(),
        sandbox_profile_row_id: row_id.to_owned(),
        display_label: display_label.to_owned(),
        profile,
        profile_token: profile.as_str().to_owned(),
        sandbox_profile_class,
        sandbox_profile_class_token: sandbox_profile_class.as_str().to_owned(),
        sandbox_profile_ref: sandbox_profile_ref.to_owned(),
        trust_profile_ref: trust_profile_ref.to_owned(),
        policy_epoch_ref: policy_epoch_ref.to_owned(),
        allowed_capability_class_tokens: allowed_capability_classes
            .iter()
            .map(|class| class.as_str().to_owned())
            .collect(),
        allowed_capability_classes,
        allowed_side_effect_class_tokens: allowed_side_effect_classes
            .iter()
            .map(|class| class.as_str().to_owned())
            .collect(),
        allowed_side_effect_classes,
        default_use_posture,
        default_use_posture_token: default_use_posture.as_str().to_owned(),
        max_ticket_lifetime_seconds,
        guardrails: BetaGuardrails::clean(),
    }
}

fn seed_sandbox_profile_rows() -> Vec<SandboxProfileRow> {
    vec![
        sandbox_row(
            "approval-ticket-beta:sandbox:connected:provider-mutation",
            "Provider mutation sandbox (connected profile)",
            ApprovalTicketBetaProfileClass::Connected,
            SandboxProfileClass::ProviderMutationSandbox,
            "sandbox-profile:connected:provider-mutation:v1",
            "trust-profile:connected:default:v1",
            "policy-epoch:connected:2026-05-16",
            vec![
                CapabilityClass::WriteProviderReviewComment,
                CapabilityClass::UpdateProviderIssue,
                CapabilityClass::RerunProviderCi,
            ],
            vec![
                SideEffectClass::ProviderReviewCommentPublish,
                SideEffectClass::ProviderIssueUpdate,
                SideEffectClass::ProviderCiRerun,
            ],
            UsePosture::SingleUse,
            900,
        ),
        sandbox_row(
            "approval-ticket-beta:sandbox:mirror_only:remote-helper",
            "Remote-helper mutation sandbox (mirror-only profile)",
            ApprovalTicketBetaProfileClass::MirrorOnly,
            SandboxProfileClass::RemoteHelperSandbox,
            "sandbox-profile:mirror_only:remote-helper:v1",
            "trust-profile:mirror_only:default:v1",
            "policy-epoch:mirror_only:2026-05-16",
            vec![CapabilityClass::MutateRemoteHelperTarget],
            vec![SideEffectClass::RemoteHelperMutation],
            UsePosture::SingleUse,
            600,
        ),
        sandbox_row(
            "approval-ticket-beta:sandbox:offline:local-only-authority",
            "Local-only authority sandbox (offline profile)",
            ApprovalTicketBetaProfileClass::Offline,
            SandboxProfileClass::LocalOnlyAuthority,
            "sandbox-profile:offline:local-only:v1",
            "trust-profile:offline:default:v1",
            "policy-epoch:offline:2026-05-16",
            vec![CapabilityClass::ExecuteLocalHighRiskAction],
            vec![SideEffectClass::LocalDestructiveAction],
            UsePosture::SingleUse,
            300,
        ),
        sandbox_row(
            "approval-ticket-beta:sandbox:enterprise_managed:credential-projection",
            "Credential projection sandbox (enterprise-managed profile)",
            ApprovalTicketBetaProfileClass::EnterpriseManaged,
            SandboxProfileClass::CredentialProjectionSandbox,
            "sandbox-profile:enterprise_managed:credential-projection:v1",
            "trust-profile:enterprise_managed:default:v1",
            "policy-epoch:enterprise_managed:2026-05-16",
            vec![CapabilityClass::ProjectCredentialToConsumer],
            vec![SideEffectClass::CredentialProjectionToConsumer],
            UsePosture::BoundedReuse,
            120,
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn envelope_row(
    row_id: &str,
    display_label: &str,
    profile: ApprovalTicketBetaProfileClass,
    sandbox_profile_ref: &str,
    envelope_id: &str,
    envelope_fingerprint_ref: &str,
    action_class: HighRiskActionClass,
    side_effect_class: SideEffectClass,
    target_identity: TargetIdentity,
    allowed_capability_classes: Vec<CapabilityClass>,
    actor_scope: ActorScope,
    sealed_at: &str,
    expires_at: &str,
    evidence_refs: Vec<&str>,
    rollback_refs: Vec<&str>,
) -> CapabilityEnvelopeRow {
    CapabilityEnvelopeRow {
        record_kind: APPROVAL_TICKET_BETA_CAPABILITY_ENVELOPE_ROW_RECORD_KIND.to_owned(),
        schema_version: APPROVAL_TICKET_BETA_SCHEMA_VERSION,
        shared_contract_ref: APPROVAL_TICKET_BETA_SHARED_CONTRACT_REF.to_owned(),
        capability_envelope_row_id: row_id.to_owned(),
        display_label: display_label.to_owned(),
        profile,
        profile_token: profile.as_str().to_owned(),
        sandbox_profile_ref: sandbox_profile_ref.to_owned(),
        envelope_id: envelope_id.to_owned(),
        envelope_fingerprint_ref: envelope_fingerprint_ref.to_owned(),
        action_class,
        action_class_token: action_class.as_str().to_owned(),
        side_effect_class,
        side_effect_class_token: side_effect_class.as_str().to_owned(),
        target_identity,
        allowed_capability_class_tokens: allowed_capability_classes
            .iter()
            .map(|class| class.as_str().to_owned())
            .collect(),
        allowed_capability_classes,
        actor_scope,
        sealed_at: sealed_at.to_owned(),
        expires_at: expires_at.to_owned(),
        evidence_refs: evidence_refs.into_iter().map(String::from).collect(),
        rollback_refs: rollback_refs.into_iter().map(String::from).collect(),
        guardrails: BetaGuardrails::clean(),
    }
}

fn target(
    target_class: TargetClass,
    target_ref: &str,
    target_label: &str,
    target_fingerprint_ref: &str,
    target_version_ref: &str,
) -> TargetIdentity {
    TargetIdentity {
        target_class,
        target_class_token: target_class.as_str().to_owned(),
        target_ref: target_ref.to_owned(),
        target_label: target_label.to_owned(),
        target_fingerprint_ref: target_fingerprint_ref.to_owned(),
        target_version_ref: target_version_ref.to_owned(),
    }
}

fn actor(
    actor_class: ActorClass,
    actor_subject_ref: &str,
    granted_scope_refs: Vec<&str>,
    auth_source_class: AuthSourceClass,
) -> ActorScope {
    ActorScope {
        actor_class,
        actor_class_token: actor_class.as_str().to_owned(),
        actor_subject_ref: actor_subject_ref.to_owned(),
        granted_scope_refs: granted_scope_refs.into_iter().map(String::from).collect(),
        auth_source_class,
        auth_source_class_token: auth_source_class.as_str().to_owned(),
    }
}

fn seed_capability_envelope_rows() -> Vec<CapabilityEnvelopeRow> {
    vec![
        envelope_row(
            "approval-ticket-beta:envelope:connected:review-comment",
            "Sealed envelope for publishing one provider review comment",
            ApprovalTicketBetaProfileClass::Connected,
            "sandbox-profile:connected:provider-mutation:v1",
            "envelope:connected:review-comment:0001",
            "envelope-fingerprint:connected:review-comment:0001:v1",
            HighRiskActionClass::ExternalProviderMutation,
            SideEffectClass::ProviderReviewCommentPublish,
            target(
                TargetClass::ProviderObject,
                "provider:github:review:owner/repo#1234",
                "PR #1234 in owner/repo",
                "target-fingerprint:provider:github:review:owner/repo#1234:v7",
                "target-version:provider:github:review:owner/repo#1234:v7",
            ),
            vec![CapabilityClass::WriteProviderReviewComment],
            actor(
                ActorClass::HumanAccount,
                "user:human:42",
                vec!["scope:provider:github:owner/repo:review:comment:publish"],
                AuthSourceClass::HumanSession,
            ),
            "2026-05-16T01:00:00Z",
            "2026-05-16T01:15:00Z",
            vec!["evidence:review-prompt:connected:0001"],
            vec!["rollback:review-comment:delete-on-revoke:connected:0001"],
        ),
        envelope_row(
            "approval-ticket-beta:envelope:mirror_only:remote-helper",
            "Sealed envelope for a single remote-helper mutation",
            ApprovalTicketBetaProfileClass::MirrorOnly,
            "sandbox-profile:mirror_only:remote-helper:v1",
            "envelope:mirror_only:remote-helper:0001",
            "envelope-fingerprint:mirror_only:remote-helper:0001:v1",
            HighRiskActionClass::HelperBackedRemoteMutation,
            SideEffectClass::RemoteHelperMutation,
            target(
                TargetClass::RemoteHelperTarget,
                "remote-helper:tunnel:fleet:0001:resource:configmap-a",
                "Remote helper configmap mutation",
                "target-fingerprint:remote-helper:configmap-a:v3",
                "target-version:remote-helper:configmap-a:v3",
            ),
            vec![CapabilityClass::MutateRemoteHelperTarget],
            actor(
                ActorClass::DelegatedCredential,
                "delegated:remote-helper:fleet:0001",
                vec!["scope:remote-helper:fleet:0001:mutate"],
                AuthSourceClass::DelegatedCredential,
            ),
            "2026-05-16T01:30:00Z",
            "2026-05-16T01:40:00Z",
            vec!["evidence:policy-decision:mirror_only:0001"],
            vec!["rollback:remote-helper:revert:mirror_only:0001"],
        ),
        envelope_row(
            "approval-ticket-beta:envelope:offline:local-destructive",
            "Sealed envelope for one local destructive workspace action",
            ApprovalTicketBetaProfileClass::Offline,
            "sandbox-profile:offline:local-only:v1",
            "envelope:offline:local-destructive:0001",
            "envelope-fingerprint:offline:local-destructive:0001:v1",
            HighRiskActionClass::LocalHighRiskAction,
            SideEffectClass::LocalDestructiveAction,
            target(
                TargetClass::LocalOnlyTarget,
                "local:workspace:payments:branch:delete:legacy/migration",
                "Delete local branch legacy/migration",
                "target-fingerprint:local:branch:legacy/migration:v1",
                "target-version:local:branch:legacy/migration:v1",
            ),
            vec![CapabilityClass::ExecuteLocalHighRiskAction],
            actor(
                ActorClass::HumanAccount,
                "user:human:42",
                vec!["scope:local:workspace:payments:branch:delete"],
                AuthSourceClass::LocalOnly,
            ),
            "2026-05-16T02:00:00Z",
            "2026-05-16T02:05:00Z",
            vec!["evidence:shell-prompt:offline:0001"],
            vec!["rollback:local:branch:restore:legacy/migration:v1"],
        ),
        envelope_row(
            "approval-ticket-beta:envelope:enterprise_managed:credential-projection",
            "Sealed envelope for credential projection to one typed consumer",
            ApprovalTicketBetaProfileClass::EnterpriseManaged,
            "sandbox-profile:enterprise_managed:credential-projection:v1",
            "envelope:enterprise_managed:credential-projection:0001",
            "envelope-fingerprint:enterprise_managed:credential-projection:0001:v1",
            HighRiskActionClass::CredentialProjection,
            SideEffectClass::CredentialProjectionToConsumer,
            target(
                TargetClass::CredentialConsumerTarget,
                "consumer:ai-provider:byok:tenant-001",
                "BYOK AI provider consumer (tenant-001)",
                "target-fingerprint:consumer:ai-provider:byok:tenant-001:v2",
                "target-version:consumer:ai-provider:byok:tenant-001:v2",
            ),
            vec![CapabilityClass::ProjectCredentialToConsumer],
            actor(
                ActorClass::PolicyInjectedServiceIdentity,
                "policy-service:credential-projection:tenant-001",
                vec!["scope:credential-projection:tenant-001:ai-provider:byok"],
                AuthSourceClass::PolicyInjectedService,
            ),
            "2026-05-16T02:30:00Z",
            "2026-05-16T02:32:00Z",
            vec!["evidence:policy-decision:enterprise_managed:0001"],
            vec!["rollback:credential-projection:revoke:tenant-001"],
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn ticket_row(
    approval_ticket_id: &str,
    display_label: &str,
    profile: ApprovalTicketBetaProfileClass,
    issuer_class: IssuerClass,
    issuing_surface_ref: &str,
    request_origin_class: RequestOriginClass,
    requesting_surface_ref: Option<&str>,
    sandbox_profile_ref: &str,
    capability_envelope_ref: &str,
    target_identity: TargetIdentity,
    action_class: HighRiskActionClass,
    side_effect_class: SideEffectClass,
    actor_scope: ActorScope,
    trust_profile_ref: &str,
    policy_epoch_ref: &str,
    issued_at: &str,
    expires_at: &str,
    lifetime_seconds: u64,
    use_posture: UsePosture,
    authority_requirement: AuthorityRequirement,
    evidence_refs: Vec<&str>,
    rollback_refs: Vec<&str>,
    runtime_approval_ticket_ref: &str,
    provider_plane_approval_ticket_ref: Option<&str>,
) -> IssuedApprovalTicketRow {
    IssuedApprovalTicketRow {
        record_kind: APPROVAL_TICKET_BETA_TICKET_ROW_RECORD_KIND.to_owned(),
        schema_version: APPROVAL_TICKET_BETA_SCHEMA_VERSION,
        shared_contract_ref: APPROVAL_TICKET_BETA_SHARED_CONTRACT_REF.to_owned(),
        approval_ticket_id: approval_ticket_id.to_owned(),
        display_label: display_label.to_owned(),
        profile,
        profile_token: profile.as_str().to_owned(),
        issuer_class,
        issuer_class_token: issuer_class.as_str().to_owned(),
        issuing_surface_ref: issuing_surface_ref.to_owned(),
        request_origin_class,
        request_origin_class_token: request_origin_class.as_str().to_owned(),
        requesting_surface_ref: requesting_surface_ref.map(String::from),
        sandbox_profile_ref: sandbox_profile_ref.to_owned(),
        capability_envelope_ref: capability_envelope_ref.to_owned(),
        target_identity,
        action_class,
        action_class_token: action_class.as_str().to_owned(),
        side_effect_class,
        side_effect_class_token: side_effect_class.as_str().to_owned(),
        actor_scope,
        trust_profile_ref: trust_profile_ref.to_owned(),
        policy_epoch_ref: policy_epoch_ref.to_owned(),
        issued_at: issued_at.to_owned(),
        expires_at: expires_at.to_owned(),
        lifetime_seconds,
        use_posture,
        use_posture_token: use_posture.as_str().to_owned(),
        authority_requirement,
        authority_requirement_token: authority_requirement.as_str().to_owned(),
        evidence_refs: evidence_refs.into_iter().map(String::from).collect(),
        rollback_refs: rollback_refs.into_iter().map(String::from).collect(),
        runtime_approval_ticket_ref: runtime_approval_ticket_ref.to_owned(),
        provider_plane_approval_ticket_ref: provider_plane_approval_ticket_ref.map(String::from),
        guardrails: BetaGuardrails::clean(),
    }
}

fn seed_ticket_rows() -> Vec<IssuedApprovalTicketRow> {
    vec![
        ticket_row(
            "approval-ticket-beta:ticket:connected:review-comment:0001",
            "Shell-minted ticket for PR #1234 review comment",
            ApprovalTicketBetaProfileClass::Connected,
            IssuerClass::Shell,
            "shell-prompt:connected:0001",
            RequestOriginClass::AiToolPlan,
            Some("ai:tool-plan:review-summary:0001"),
            "sandbox-profile:connected:provider-mutation:v1",
            "envelope:connected:review-comment:0001",
            target(
                TargetClass::ProviderObject,
                "provider:github:review:owner/repo#1234",
                "PR #1234 in owner/repo",
                "target-fingerprint:provider:github:review:owner/repo#1234:v7",
                "target-version:provider:github:review:owner/repo#1234:v7",
            ),
            HighRiskActionClass::ExternalProviderMutation,
            SideEffectClass::ProviderReviewCommentPublish,
            actor(
                ActorClass::HumanAccount,
                "user:human:42",
                vec!["scope:provider:github:owner/repo:review:comment:publish"],
                AuthSourceClass::HumanSession,
            ),
            "trust-profile:connected:default:v1",
            "policy-epoch:connected:2026-05-16",
            "2026-05-16T01:00:00Z",
            "2026-05-16T01:15:00Z",
            900,
            UsePosture::SingleUse,
            AuthorityRequirement::ApprovalTicketRequired,
            vec!["evidence:review-prompt:connected:0001"],
            vec!["rollback:review-comment:delete-on-revoke:connected:0001"],
            "runtime-approval-ticket:connected:0001",
            Some("provider-plane:approval-ticket:connected:0001"),
        ),
        ticket_row(
            "approval-ticket-beta:ticket:mirror_only:remote-helper:0001",
            "Policy-service ticket for one remote-helper configmap mutation",
            ApprovalTicketBetaProfileClass::MirrorOnly,
            IssuerClass::PolicyService,
            "policy-service:decision:mirror_only:0001",
            RequestOriginClass::ExtensionRequest,
            Some("extension:devops-tools:1.4.0"),
            "sandbox-profile:mirror_only:remote-helper:v1",
            "envelope:mirror_only:remote-helper:0001",
            target(
                TargetClass::RemoteHelperTarget,
                "remote-helper:tunnel:fleet:0001:resource:configmap-a",
                "Remote helper configmap mutation",
                "target-fingerprint:remote-helper:configmap-a:v3",
                "target-version:remote-helper:configmap-a:v3",
            ),
            HighRiskActionClass::HelperBackedRemoteMutation,
            SideEffectClass::RemoteHelperMutation,
            actor(
                ActorClass::DelegatedCredential,
                "delegated:remote-helper:fleet:0001",
                vec!["scope:remote-helper:fleet:0001:mutate"],
                AuthSourceClass::DelegatedCredential,
            ),
            "trust-profile:mirror_only:default:v1",
            "policy-epoch:mirror_only:2026-05-16",
            "2026-05-16T01:30:00Z",
            "2026-05-16T01:40:00Z",
            600,
            UsePosture::SingleUse,
            AuthorityRequirement::TicketOrReviewedScope,
            vec!["evidence:policy-decision:mirror_only:0001"],
            vec!["rollback:remote-helper:revert:mirror_only:0001"],
            "runtime-approval-ticket:mirror_only:0001",
            None,
        ),
        ticket_row(
            "approval-ticket-beta:ticket:offline:local-destructive:0001",
            "Shell-minted ticket for a local branch deletion",
            ApprovalTicketBetaProfileClass::Offline,
            IssuerClass::Shell,
            "shell-prompt:offline:0001",
            RequestOriginClass::UserShellPrompt,
            None,
            "sandbox-profile:offline:local-only:v1",
            "envelope:offline:local-destructive:0001",
            target(
                TargetClass::LocalOnlyTarget,
                "local:workspace:payments:branch:delete:legacy/migration",
                "Delete local branch legacy/migration",
                "target-fingerprint:local:branch:legacy/migration:v1",
                "target-version:local:branch:legacy/migration:v1",
            ),
            HighRiskActionClass::LocalHighRiskAction,
            SideEffectClass::LocalDestructiveAction,
            actor(
                ActorClass::HumanAccount,
                "user:human:42",
                vec!["scope:local:workspace:payments:branch:delete"],
                AuthSourceClass::LocalOnly,
            ),
            "trust-profile:offline:default:v1",
            "policy-epoch:offline:2026-05-16",
            "2026-05-16T02:00:00Z",
            "2026-05-16T02:05:00Z",
            300,
            UsePosture::SingleUse,
            AuthorityRequirement::ApprovalTicketRequired,
            vec!["evidence:shell-prompt:offline:0001"],
            vec!["rollback:local:branch:restore:legacy/migration:v1"],
            "runtime-approval-ticket:offline:0001",
            None,
        ),
        ticket_row(
            "approval-ticket-beta:ticket:enterprise_managed:credential-projection:0001",
            "Supervisor ticket for credential projection to BYOK AI provider",
            ApprovalTicketBetaProfileClass::EnterpriseManaged,
            IssuerClass::Supervisor,
            "supervisor:control-path:enterprise_managed:0001",
            RequestOriginClass::AutomationSchedulerRequest,
            Some("scheduler:nightly:byok-projection"),
            "sandbox-profile:enterprise_managed:credential-projection:v1",
            "envelope:enterprise_managed:credential-projection:0001",
            target(
                TargetClass::CredentialConsumerTarget,
                "consumer:ai-provider:byok:tenant-001",
                "BYOK AI provider consumer (tenant-001)",
                "target-fingerprint:consumer:ai-provider:byok:tenant-001:v2",
                "target-version:consumer:ai-provider:byok:tenant-001:v2",
            ),
            HighRiskActionClass::CredentialProjection,
            SideEffectClass::CredentialProjectionToConsumer,
            actor(
                ActorClass::PolicyInjectedServiceIdentity,
                "policy-service:credential-projection:tenant-001",
                vec!["scope:credential-projection:tenant-001:ai-provider:byok"],
                AuthSourceClass::PolicyInjectedService,
            ),
            "trust-profile:enterprise_managed:default:v1",
            "policy-epoch:enterprise_managed:2026-05-16",
            "2026-05-16T02:30:00Z",
            "2026-05-16T02:32:00Z",
            120,
            UsePosture::BoundedReuse,
            AuthorityRequirement::ApprovalTicketRequired,
            vec!["evidence:policy-decision:enterprise_managed:0001"],
            vec!["rollback:credential-projection:revoke:tenant-001"],
            "runtime-approval-ticket:enterprise_managed:0001",
            None,
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn spend_event(
    spend_attempt_id: &str,
    profile: ApprovalTicketBetaProfileClass,
    mutation_ref: &str,
    presented_approval_ticket_ref: Option<&str>,
    current_actor_scope: ActorScope,
    current_target_identity: TargetIdentity,
    current_sandbox_profile_ref: &str,
    current_capability_envelope_ref: &str,
    current_trust_profile_ref: &str,
    current_policy_epoch_ref: &str,
    evaluated_at: &str,
    evaluation_outcome: EvaluationOutcome,
    native_reapproval_route: NativeReapprovalRoute,
    explanation: &str,
    audit_event_refs: Vec<&str>,
) -> SpendAttemptEvent {
    SpendAttemptEvent {
        record_kind: APPROVAL_TICKET_BETA_SPEND_ATTEMPT_EVENT_RECORD_KIND.to_owned(),
        schema_version: APPROVAL_TICKET_BETA_SCHEMA_VERSION,
        shared_contract_ref: APPROVAL_TICKET_BETA_SHARED_CONTRACT_REF.to_owned(),
        spend_attempt_id: spend_attempt_id.to_owned(),
        profile,
        profile_token: profile.as_str().to_owned(),
        mutation_ref: mutation_ref.to_owned(),
        presented_approval_ticket_ref: presented_approval_ticket_ref.map(String::from),
        current_actor_scope,
        current_target_identity,
        current_sandbox_profile_ref: current_sandbox_profile_ref.to_owned(),
        current_capability_envelope_ref: current_capability_envelope_ref.to_owned(),
        current_trust_profile_ref: current_trust_profile_ref.to_owned(),
        current_policy_epoch_ref: current_policy_epoch_ref.to_owned(),
        evaluated_at: evaluated_at.to_owned(),
        evaluation_outcome,
        evaluation_outcome_token: evaluation_outcome.as_str().to_owned(),
        native_reapproval_route,
        native_reapproval_route_token: native_reapproval_route.as_str().to_owned(),
        explanation: explanation.to_owned(),
        audit_event_refs: audit_event_refs.into_iter().map(String::from).collect(),
    }
}

fn seed_spend_attempt_events() -> Vec<SpendAttemptEvent> {
    let connected_actor = actor(
        ActorClass::HumanAccount,
        "user:human:42",
        vec!["scope:provider:github:owner/repo:review:comment:publish"],
        AuthSourceClass::HumanSession,
    );
    let connected_target = target(
        TargetClass::ProviderObject,
        "provider:github:review:owner/repo#1234",
        "PR #1234 in owner/repo",
        "target-fingerprint:provider:github:review:owner/repo#1234:v7",
        "target-version:provider:github:review:owner/repo#1234:v7",
    );
    let drifted_target = target(
        TargetClass::ProviderObject,
        "provider:github:review:owner/repo#1234",
        "PR #1234 in owner/repo (rebased)",
        "target-fingerprint:provider:github:review:owner/repo#1234:v8",
        "target-version:provider:github:review:owner/repo#1234:v8",
    );
    let mirror_actor = actor(
        ActorClass::DelegatedCredential,
        "delegated:remote-helper:fleet:0001",
        vec!["scope:remote-helper:fleet:0001:mutate"],
        AuthSourceClass::DelegatedCredential,
    );
    let mirror_target = target(
        TargetClass::RemoteHelperTarget,
        "remote-helper:tunnel:fleet:0001:resource:configmap-a",
        "Remote helper configmap mutation",
        "target-fingerprint:remote-helper:configmap-a:v3",
        "target-version:remote-helper:configmap-a:v3",
    );
    let offline_actor = actor(
        ActorClass::HumanAccount,
        "user:human:42",
        vec!["scope:local:workspace:payments:branch:delete"],
        AuthSourceClass::LocalOnly,
    );
    let offline_target = target(
        TargetClass::LocalOnlyTarget,
        "local:workspace:payments:branch:delete:legacy/migration",
        "Delete local branch legacy/migration",
        "target-fingerprint:local:branch:legacy/migration:v1",
        "target-version:local:branch:legacy/migration:v1",
    );
    let enterprise_actor = actor(
        ActorClass::PolicyInjectedServiceIdentity,
        "policy-service:credential-projection:tenant-001",
        vec!["scope:credential-projection:tenant-001:ai-provider:byok"],
        AuthSourceClass::PolicyInjectedService,
    );
    let enterprise_target = target(
        TargetClass::CredentialConsumerTarget,
        "consumer:ai-provider:byok:tenant-001",
        "BYOK AI provider consumer (tenant-001)",
        "target-fingerprint:consumer:ai-provider:byok:tenant-001:v2",
        "target-version:consumer:ai-provider:byok:tenant-001:v2",
    );

    vec![
        spend_event(
            "approval-ticket-beta:spend:connected:admitted:0001",
            ApprovalTicketBetaProfileClass::Connected,
            "mutation:provider:github:review-comment-publish:owner/repo#1234:0001",
            Some("approval-ticket-beta:ticket:connected:review-comment:0001"),
            connected_actor.clone(),
            connected_target.clone(),
            "sandbox-profile:connected:provider-mutation:v1",
            "envelope:connected:review-comment:0001",
            "trust-profile:connected:default:v1",
            "policy-epoch:connected:2026-05-16",
            "2026-05-16T01:10:00Z",
            EvaluationOutcome::Admitted,
            NativeReapprovalRoute::NotRequired,
            "Authority matched the ticket on every axis; the review comment publish admitted.",
            vec!["audit:provider-mutation:connected:0001:admit"],
        ),
        spend_event(
            "approval-ticket-beta:spend:connected:target-drift:0002",
            ApprovalTicketBetaProfileClass::Connected,
            "mutation:provider:github:review-comment-publish:owner/repo#1234:0002",
            Some("approval-ticket-beta:ticket:connected:review-comment:0001"),
            connected_actor.clone(),
            drifted_target.clone(),
            "sandbox-profile:connected:provider-mutation:v1",
            "envelope:connected:review-comment:0001",
            "trust-profile:connected:default:v1",
            "policy-epoch:connected:2026-05-16",
            "2026-05-16T01:12:00Z",
            EvaluationOutcome::DeniedTargetDrift,
            NativeReapprovalRoute::RefreshTargetThenReapprove,
            "Target fingerprint advanced from v7 to v8 after a rebase; the stale ticket cannot be replayed against the new commit.",
            vec!["audit:provider-mutation:connected:0002:deny"],
        ),
        spend_event(
            "approval-ticket-beta:spend:mirror_only:expired:0003",
            ApprovalTicketBetaProfileClass::MirrorOnly,
            "mutation:remote-helper:fleet:0001:configmap-a:0003",
            Some("approval-ticket-beta:ticket:mirror_only:remote-helper:0001"),
            mirror_actor.clone(),
            mirror_target.clone(),
            "sandbox-profile:mirror_only:remote-helper:v1",
            "envelope:mirror_only:remote-helper:0001",
            "trust-profile:mirror_only:default:v1",
            "policy-epoch:mirror_only:2026-05-16",
            "2026-05-16T01:50:00Z",
            EvaluationOutcome::DeniedExpired,
            NativeReapprovalRoute::NativeReapprovalSheet,
            "Ticket expired at 01:40:00Z; the helper-backed mutation must be re-approved through the native reapproval sheet.",
            vec!["audit:remote-helper:mirror_only:0003:deny"],
        ),
        spend_event(
            "approval-ticket-beta:spend:mirror_only:policy-epoch-drift:0004",
            ApprovalTicketBetaProfileClass::MirrorOnly,
            "mutation:remote-helper:fleet:0001:configmap-a:0004",
            Some("approval-ticket-beta:ticket:mirror_only:remote-helper:0001"),
            mirror_actor.clone(),
            mirror_target.clone(),
            "sandbox-profile:mirror_only:remote-helper:v1",
            "envelope:mirror_only:remote-helper:0001",
            "trust-profile:mirror_only:default:v1",
            "policy-epoch:mirror_only:2026-05-16-rev2",
            "2026-05-16T01:35:00Z",
            EvaluationOutcome::DeniedPolicyEpochDrift,
            NativeReapprovalRoute::NativeReapprovalSheet,
            "Policy epoch advanced from 2026-05-16 to 2026-05-16-rev2 between issuance and spend; the stale ticket is invalidated.",
            vec!["audit:remote-helper:mirror_only:0004:deny"],
        ),
        spend_event(
            "approval-ticket-beta:spend:offline:missing-authority:0005",
            ApprovalTicketBetaProfileClass::Offline,
            "mutation:local:branch:delete:legacy/migration:0005",
            None,
            offline_actor.clone(),
            offline_target.clone(),
            "sandbox-profile:offline:local-only:v1",
            "envelope:offline:local-destructive:0001",
            "trust-profile:offline:default:v1",
            "policy-epoch:offline:2026-05-16",
            "2026-05-16T02:01:00Z",
            EvaluationOutcome::DeniedMissingAuthority,
            NativeReapprovalRoute::NativeReapprovalSheet,
            "No approval ticket was presented; the destructive workspace action is fail-closed.",
            vec!["audit:local-destructive:offline:0005:deny"],
        ),
        spend_event(
            "approval-ticket-beta:spend:offline:self-authorization:0006",
            ApprovalTicketBetaProfileClass::Offline,
            "mutation:local:branch:delete:legacy/migration:0006",
            Some("approval-ticket-beta:ticket:mirror_only:remote-helper:0001"),
            offline_actor.clone(),
            offline_target.clone(),
            "sandbox-profile:offline:local-only:v1",
            "envelope:offline:local-destructive:0001",
            "trust-profile:offline:default:v1",
            "policy-epoch:offline:2026-05-16",
            "2026-05-16T02:02:00Z",
            EvaluationOutcome::DeniedSelfAuthorizationAttempted,
            NativeReapprovalRoute::InspectOnlyDenied,
            "Extension presented a ticket minted for a different non-intrinsic origin; self-authorization across origins is refused.",
            vec!["audit:local-destructive:offline:0006:deny"],
        ),
        spend_event(
            "approval-ticket-beta:spend:enterprise_managed:sandbox-drift:0007",
            ApprovalTicketBetaProfileClass::EnterpriseManaged,
            "mutation:credential-projection:tenant-001:ai-provider:byok:0007",
            Some("approval-ticket-beta:ticket:enterprise_managed:credential-projection:0001"),
            enterprise_actor.clone(),
            enterprise_target.clone(),
            "sandbox-profile:enterprise_managed:credential-projection:v2",
            "envelope:enterprise_managed:credential-projection:0001",
            "trust-profile:enterprise_managed:default:v1",
            "policy-epoch:enterprise_managed:2026-05-16",
            "2026-05-16T02:31:00Z",
            EvaluationOutcome::DeniedSandboxProfileDrift,
            NativeReapprovalRoute::NativeReapprovalSheet,
            "Sandbox profile advanced from v1 to v2 (tighter ceiling); the existing ticket cannot be replayed against the new sandbox profile.",
            vec!["audit:credential-projection:enterprise_managed:0007:deny"],
        ),
        spend_event(
            "approval-ticket-beta:spend:enterprise_managed:envelope-drift:0008",
            ApprovalTicketBetaProfileClass::EnterpriseManaged,
            "mutation:credential-projection:tenant-001:ai-provider:byok:0008",
            Some("approval-ticket-beta:ticket:enterprise_managed:credential-projection:0001"),
            enterprise_actor,
            enterprise_target,
            "sandbox-profile:enterprise_managed:credential-projection:v1",
            "envelope:enterprise_managed:credential-projection:0002",
            "trust-profile:enterprise_managed:default:v1",
            "policy-epoch:enterprise_managed:2026-05-16",
            "2026-05-16T02:31:30Z",
            EvaluationOutcome::DeniedCapabilityEnvelopeDrift,
            NativeReapprovalRoute::RescopeThenReapprove,
            "Capability envelope advanced from 0001 to 0002; the ticket bound to envelope 0001 is invalidated rather than replayed.",
            vec!["audit:credential-projection:enterprise_managed:0008:deny"],
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_with_zero_defects() {
        let page = seeded_approval_ticket_beta_page();
        validate_approval_ticket_beta_page(&page).expect("seeded page validates");
        assert!(page.defects.is_empty());
        for profile in ApprovalTicketBetaProfileClass::ALL {
            assert!(page
                .summary
                .profiles_present
                .iter()
                .any(|token| token == profile.as_str()));
        }
        for sandbox_class in SandboxProfileClass::ALL {
            assert!(page
                .summary
                .sandbox_profile_classes_present
                .iter()
                .any(|token| token == sandbox_class.as_str()));
        }
    }

    #[test]
    fn seeded_page_covers_admitted_and_drift_denials() {
        let page = seeded_approval_ticket_beta_page();
        let outcomes: BTreeSet<&str> = page
            .summary
            .spend_attempts_by_outcome
            .keys()
            .map(String::as_str)
            .collect();
        for required in [
            "admitted",
            "denied_target_drift",
            "denied_expired",
            "denied_policy_epoch_drift",
            "denied_missing_authority",
            "denied_self_authorization_attempted",
            "denied_sandbox_profile_drift",
            "denied_capability_envelope_drift",
        ] {
            assert!(outcomes.contains(required), "missing outcome {required}");
        }
    }

    #[test]
    fn validator_flags_raw_authority_material() {
        let mut page = seeded_approval_ticket_beta_page();
        page.ticket_rows[0]
            .guardrails
            .raw_authority_material_present = true;
        let defects = audit_approval_ticket_beta_page(
            &page.sandbox_profile_rows,
            &page.capability_envelope_rows,
            &page.ticket_rows,
            &page.spend_attempt_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == ApprovalTicketBetaDefectKind::RawAuthorityMaterialPresent));
    }

    #[test]
    fn validator_flags_self_authorization_attempt() {
        let mut page = seeded_approval_ticket_beta_page();
        page.capability_envelope_rows[0]
            .guardrails
            .self_authorization_attempted = true;
        let defects = audit_approval_ticket_beta_page(
            &page.sandbox_profile_rows,
            &page.capability_envelope_rows,
            &page.ticket_rows,
            &page.spend_attempt_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == ApprovalTicketBetaDefectKind::SelfAuthorizationAttempted));
    }

    #[test]
    fn validator_flags_silent_widening_attempt() {
        let mut page = seeded_approval_ticket_beta_page();
        page.ticket_rows[0]
            .guardrails
            .silent_widening_attempted = true;
        let defects = audit_approval_ticket_beta_page(
            &page.sandbox_profile_rows,
            &page.capability_envelope_rows,
            &page.ticket_rows,
            &page.spend_attempt_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == ApprovalTicketBetaDefectKind::SilentWideningAttempted));
    }

    #[test]
    fn validator_flags_admitted_spend_under_drift() {
        let mut page = seeded_approval_ticket_beta_page();
        let drift_event = page
            .spend_attempt_events
            .iter_mut()
            .find(|event| event.evaluation_outcome == EvaluationOutcome::DeniedTargetDrift)
            .expect("seeded target-drift event");
        drift_event.evaluation_outcome = EvaluationOutcome::Admitted;
        drift_event.evaluation_outcome_token = EvaluationOutcome::Admitted.as_str().to_owned();
        drift_event.native_reapproval_route = NativeReapprovalRoute::NotRequired;
        drift_event.native_reapproval_route_token =
            NativeReapprovalRoute::NotRequired.as_str().to_owned();
        let defects = audit_approval_ticket_beta_page(
            &page.sandbox_profile_rows,
            &page.capability_envelope_rows,
            &page.ticket_rows,
            &page.spend_attempt_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == ApprovalTicketBetaDefectKind::SpendAdmittedUnderDrift));
    }

    #[test]
    fn validator_flags_denied_spend_collapsing_reapproval_route() {
        let mut page = seeded_approval_ticket_beta_page();
        let drift_event = page
            .spend_attempt_events
            .iter_mut()
            .find(|event| event.evaluation_outcome == EvaluationOutcome::DeniedTargetDrift)
            .expect("seeded target-drift event");
        drift_event.native_reapproval_route = NativeReapprovalRoute::NotRequired;
        drift_event.native_reapproval_route_token =
            NativeReapprovalRoute::NotRequired.as_str().to_owned();
        let defects = audit_approval_ticket_beta_page(
            &page.sandbox_profile_rows,
            &page.capability_envelope_rows,
            &page.ticket_rows,
            &page.spend_attempt_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == ApprovalTicketBetaDefectKind::SpendDenialReapprovalRouteCollapsed));
    }

    #[test]
    fn validator_flags_denied_spend_missing_audit_ref() {
        let mut page = seeded_approval_ticket_beta_page();
        let event = page
            .spend_attempt_events
            .iter_mut()
            .find(|event| event.evaluation_outcome == EvaluationOutcome::DeniedExpired)
            .expect("seeded expired event");
        event.audit_event_refs.clear();
        let defects = audit_approval_ticket_beta_page(
            &page.sandbox_profile_rows,
            &page.capability_envelope_rows,
            &page.ticket_rows,
            &page.spend_attempt_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == ApprovalTicketBetaDefectKind::SpendDenialMissingAuditRef));
    }

    #[test]
    fn validator_flags_envelope_capability_outside_sandbox() {
        let mut page = seeded_approval_ticket_beta_page();
        let envelope = &mut page.capability_envelope_rows[0];
        envelope
            .allowed_capability_classes
            .push(CapabilityClass::MutateRemoteHelperTarget);
        envelope
            .allowed_capability_class_tokens
            .push(CapabilityClass::MutateRemoteHelperTarget.as_str().to_owned());
        let defects = audit_approval_ticket_beta_page(
            &page.sandbox_profile_rows,
            &page.capability_envelope_rows,
            &page.ticket_rows,
            &page.spend_attempt_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == ApprovalTicketBetaDefectKind::EnvelopeCapabilityOutsideSandbox));
    }

    #[test]
    fn validator_flags_ticket_lifetime_exceeds_sandbox_budget() {
        let mut page = seeded_approval_ticket_beta_page();
        let ticket = &mut page.ticket_rows[0];
        ticket.expires_at = "2026-05-16T01:25:00Z".to_owned();
        ticket.lifetime_seconds = 1500;
        let defects = audit_approval_ticket_beta_page(
            &page.sandbox_profile_rows,
            &page.capability_envelope_rows,
            &page.ticket_rows,
            &page.spend_attempt_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == ApprovalTicketBetaDefectKind::TicketLifetimeExceedsSandboxBudget));
    }

    #[test]
    fn validator_flags_missing_requesting_surface_ref() {
        let mut page = seeded_approval_ticket_beta_page();
        let ticket = page
            .ticket_rows
            .iter_mut()
            .find(|ticket| ticket.request_origin_class == RequestOriginClass::AiToolPlan)
            .expect("seeded AI tool plan ticket");
        ticket.requesting_surface_ref = None;
        let defects = audit_approval_ticket_beta_page(
            &page.sandbox_profile_rows,
            &page.capability_envelope_rows,
            &page.ticket_rows,
            &page.spend_attempt_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == ApprovalTicketBetaDefectKind::MissingRequestingSurfaceRef));
    }

    #[test]
    fn validator_flags_profile_coverage_missing() {
        let mut page = seeded_approval_ticket_beta_page();
        page.sandbox_profile_rows
            .retain(|row| row.profile != ApprovalTicketBetaProfileClass::Offline);
        page.capability_envelope_rows
            .retain(|row| row.profile != ApprovalTicketBetaProfileClass::Offline);
        page.ticket_rows
            .retain(|row| row.profile != ApprovalTicketBetaProfileClass::Offline);
        page.spend_attempt_events
            .retain(|event| event.profile != ApprovalTicketBetaProfileClass::Offline);
        let defects = audit_approval_ticket_beta_page(
            &page.sandbox_profile_rows,
            &page.capability_envelope_rows,
            &page.ticket_rows,
            &page.spend_attempt_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == ApprovalTicketBetaDefectKind::ProfileCoverageMissing
            && defect.note.contains("offline")));
    }

    #[test]
    fn validator_flags_target_drift_matches_ticket_target() {
        let mut page = seeded_approval_ticket_beta_page();
        let event = page
            .spend_attempt_events
            .iter_mut()
            .find(|event| event.evaluation_outcome == EvaluationOutcome::DeniedTargetDrift)
            .expect("seeded target-drift event");
        event.current_target_identity.target_fingerprint_ref =
            "target-fingerprint:provider:github:review:owner/repo#1234:v7".to_owned();
        event.current_target_identity.target_version_ref =
            "target-version:provider:github:review:owner/repo#1234:v7".to_owned();
        let defects = audit_approval_ticket_beta_page(
            &page.sandbox_profile_rows,
            &page.capability_envelope_rows,
            &page.ticket_rows,
            &page.spend_attempt_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == ApprovalTicketBetaDefectKind::TargetDriftMatchesTicketTarget));
    }

    #[test]
    fn support_export_round_trip_preserves_lineage() {
        let page = seeded_approval_ticket_beta_page();
        let export = ApprovalTicketBetaSupportExport::from_page(
            "approval-ticket-beta:support-export:001",
            "2026-05-16T05:00:00Z",
            page,
        );
        assert!(export.raw_authority_material_excluded);
        assert!(export.authority_lineage_preserved);
        assert!(export.no_self_authorization_invariant);
        assert!(export.defect_kinds_present.is_empty());
    }

    #[test]
    fn summary_counts_match_records() {
        let page = seeded_approval_ticket_beta_page();
        assert_eq!(
            page.summary.sandbox_profile_row_count,
            page.sandbox_profile_rows.len()
        );
        assert_eq!(
            page.summary.capability_envelope_row_count,
            page.capability_envelope_rows.len()
        );
        assert_eq!(page.summary.ticket_row_count, page.ticket_rows.len());
        assert_eq!(
            page.summary.spend_attempt_event_count,
            page.spend_attempt_events.len()
        );
        let outcome_total: usize = page.summary.spend_attempts_by_outcome.values().sum();
        assert_eq!(outcome_total, page.spend_attempt_events.len());
        let route_total: usize = page
            .summary
            .spend_attempts_by_reapproval_route
            .values()
            .sum();
        assert_eq!(route_total, page.spend_attempt_events.len());
    }

    #[test]
    fn parse_timestamp_round_trip() {
        let a = parse_timestamp("2026-05-16T01:00:00Z").expect("valid timestamp");
        let b = parse_timestamp("2026-05-16T01:15:00Z").expect("valid timestamp");
        assert_eq!(b - a, 900);
        assert!(parse_timestamp("not-a-timestamp").is_none());
    }
}
