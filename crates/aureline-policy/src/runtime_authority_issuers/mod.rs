//! Runtime authority issuer boundaries and remembered-decision narrowing.
//!
//! This module is a higher-level enforcement layer on top of the
//! [`authority`](crate::authority) ticket model. It defines the closed set of
//! issuers that may mint or refresh authority objects, the closed set of
//! requesting surfaces that may only request authority through one of those
//! issuers, and the structured rejection reasons that surface on every UI,
//! CLI, support export, and audit trail when a non-issuer surface tries to
//! self-authorize, infer ambient privilege, or silently broaden a remembered
//! decision.
//!
//! Remembered decisions are projected as narrow reusable rules that compile to
//! renewable short-lived tickets bound to an explicit target, actor,
//! authority source, policy epoch, sandbox profile, scope, and expiry. A
//! remembered rule may never act like an unlimited bearer credential.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::authority::{
    AuthorityActorClass, AuthorityIssuerClass, AuthoritySandboxBinding, AuthorityTargetClass,
    AuthorityTargetIdentity, AuthorityTicketClass,
};

/// Schema version exported by runtime authority issuer beta records.
pub const RUNTIME_AUTHORITY_ISSUER_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by shell, policy, supervisor, AI, extensions,
/// CLI, companions, remote helpers, support, and fixtures.
pub const RUNTIME_AUTHORITY_ISSUER_SHARED_CONTRACT_REF: &str =
    "security:runtime_authority_issuer_beta:v1";

/// Source matrix ref consumed by this projection.
pub const RUNTIME_AUTHORITY_ISSUER_SOURCE_MATRIX_REF: &str =
    "artifacts/security/m3/runtime_authority_lineage_packet.md";

/// Stable record kind for [`RuntimeAuthorityIssuerPage`] payloads.
pub const RUNTIME_AUTHORITY_ISSUER_PAGE_RECORD_KIND: &str =
    "security_runtime_authority_issuer_page_record";

/// Stable record kind for [`RuntimeAuthorityIssuerRecord`] payloads.
pub const RUNTIME_AUTHORITY_ISSUER_RECORD_KIND: &str = "security_runtime_authority_issuer_record";

/// Stable record kind for [`RequestingSurfaceRecord`] payloads.
pub const REQUESTING_SURFACE_RECORD_KIND: &str = "security_runtime_requesting_surface_record";

/// Stable record kind for [`RememberedDecisionRule`] payloads.
pub const REMEMBERED_DECISION_RULE_RECORD_KIND: &str = "security_remembered_decision_rule_record";

/// Stable record kind for [`IssuerBoundaryRequest`] payloads.
pub const ISSUER_BOUNDARY_REQUEST_RECORD_KIND: &str = "security_issuer_boundary_request_record";

/// Stable record kind for [`IssuerBoundaryDecision`] payloads.
pub const ISSUER_BOUNDARY_DECISION_RECORD_KIND: &str = "security_issuer_boundary_decision_record";

/// Stable record kind for [`RuntimeAuthorityIssuerDefect`] payloads.
pub const RUNTIME_AUTHORITY_ISSUER_DEFECT_RECORD_KIND: &str =
    "security_runtime_authority_issuer_defect_record";

/// Stable record kind for [`RuntimeAuthorityIssuerSummary`] payloads.
pub const RUNTIME_AUTHORITY_ISSUER_SUMMARY_RECORD_KIND: &str =
    "security_runtime_authority_issuer_summary_record";

/// Stable record kind for [`RuntimeAuthorityLineagePacket`] payloads.
pub const RUNTIME_AUTHORITY_LINEAGE_PACKET_RECORD_KIND: &str =
    "security_runtime_authority_lineage_packet_record";

/// Closed set of surfaces that may request authority but may never mint or
/// refresh authority on their own. Authority always flows through an
/// [`AuthorityIssuerClass`] seat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestingSurfaceClass {
    /// Admin console requested authority through an issuer.
    AdminConsole,
    /// Local administrator tool requested authority through an issuer.
    LocalAdminTool,
    /// AI tool plan requested authority through an issuer.
    AiTool,
    /// Workspace extension requested authority through an issuer.
    Extension,
    /// Recipe runner requested authority through an issuer.
    RecipeRunner,
    /// CLI script requested authority through an issuer.
    CliScript,
    /// Browser companion requested authority through an issuer.
    BrowserCompanion,
    /// Remote helper requested authority through an issuer.
    RemoteHelper,
    /// Automation scheduler requested authority through an issuer.
    AutomationScheduler,
}

impl RequestingSurfaceClass {
    /// All requesting surfaces in canonical order.
    pub const ALL: [Self; 9] = [
        Self::AdminConsole,
        Self::LocalAdminTool,
        Self::AiTool,
        Self::Extension,
        Self::RecipeRunner,
        Self::CliScript,
        Self::BrowserCompanion,
        Self::RemoteHelper,
        Self::AutomationScheduler,
    ];

    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdminConsole => "admin_console",
            Self::LocalAdminTool => "local_admin_tool",
            Self::AiTool => "ai_tool",
            Self::Extension => "extension",
            Self::RecipeRunner => "recipe_runner",
            Self::CliScript => "cli_script",
            Self::BrowserCompanion => "browser_companion",
            Self::RemoteHelper => "remote_helper",
            Self::AutomationScheduler => "automation_scheduler",
        }
    }
}

/// Closed authority-source projection that distinguishes who Aureline is
/// acting as on a privileged flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthoritySourceClass {
    /// A human user account, signed in to the desktop shell.
    HumanAccount,
    /// An installation or application grant.
    InstallationGrant,
    /// A delegated credential issued by an upstream identity.
    DelegatedCredential,
    /// A local-only authority that cannot reach provider-managed targets.
    LocalOnlyAuthority,
}

impl AuthoritySourceClass {
    /// All authority sources in canonical order.
    pub const ALL: [Self; 4] = [
        Self::HumanAccount,
        Self::InstallationGrant,
        Self::DelegatedCredential,
        Self::LocalOnlyAuthority,
    ];

    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HumanAccount => "human_account",
            Self::InstallationGrant => "installation_grant",
            Self::DelegatedCredential => "delegated_credential",
            Self::LocalOnlyAuthority => "local_only_authority",
        }
    }

    /// True when this source can reach external provider mutation or
    /// credential projection against a provider consumer.
    pub const fn can_reach_provider_targets(self) -> bool {
        !matches!(self, Self::LocalOnlyAuthority)
    }

    /// Maps the closed [`AuthorityActorClass`] vocabulary to the
    /// authority-source projection. Admin actor classes resolve to
    /// `HumanAccount` because admin step-up still rides on a human account.
    pub const fn from_actor_class(actor: AuthorityActorClass) -> Self {
        match actor {
            AuthorityActorClass::HumanAccount
            | AuthorityActorClass::LocalAdmin
            | AuthorityActorClass::OrganizationAdmin => Self::HumanAccount,
            AuthorityActorClass::InstallationOrAppGrant
            | AuthorityActorClass::PolicyInjectedServiceIdentity => Self::InstallationGrant,
            AuthorityActorClass::DelegatedCredential => Self::DelegatedCredential,
            AuthorityActorClass::LocalOnlyAuthority => Self::LocalOnlyAuthority,
        }
    }
}

/// Decision class for an [`IssuerBoundaryDecision`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuerBoundaryDecisionClass {
    /// The issuer agreed to mint or refresh authority for the requesting surface.
    Granted,
    /// The issuer agreed to refresh a remembered-decision rule into a fresh
    /// short-lived ticket.
    RememberedDecisionNarrowed,
    /// The issuer refused the request.
    Refused,
}

impl IssuerBoundaryDecisionClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Granted => "granted",
            Self::RememberedDecisionNarrowed => "remembered_decision_narrowed",
            Self::Refused => "refused",
        }
    }

    /// True when this decision admits the request.
    pub const fn is_admitted(self) -> bool {
        matches!(self, Self::Granted | Self::RememberedDecisionNarrowed)
    }
}

/// Closed reason vocabulary for refusals. Each variant remains visible on
/// the UI, CLI, support export, and audit trail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuerBoundaryRejectionReason {
    /// A non-issuer surface tried to mint authority on its own.
    SelfAuthorizationByNonIssuer,
    /// The request inferred ambient privilege from the invoking surface
    /// without an explicit issuer chain.
    AmbientPrivilegeInferred,
    /// The request did not name a recognized issuer.
    MissingIssuerBinding,
    /// The named issuer is not allowed to mint authority for this requesting
    /// surface.
    IssuerNotAllowedForSurface,
    /// The remembered-decision rule was missing or stale for a refresh request.
    RememberedDecisionMissing,
    /// The remembered rule's scope, target, or actor was too broad to admit a
    /// short-lived ticket.
    RememberedDecisionTooBroad,
    /// The remembered rule renewal lifetime exceeded the ticket-class budget.
    RememberedDecisionLifetimeExceedsBudget,
    /// The remembered rule was attached to a ticket class that must reprompt.
    RememberedDecisionForbiddenClass,
    /// Requested target identity drifted from the remembered rule.
    RememberedDecisionTargetDrift,
    /// Requested actor identity drifted from the remembered rule.
    RememberedDecisionActorDrift,
    /// Authority source class drifted between the actor and the remembered rule.
    AuthoritySourceMismatch,
    /// Authority source projection cannot reach the requested target class.
    AuthoritySourceUnreachableTarget,
    /// Policy epoch drifted from the remembered rule.
    PolicyEpochDrift,
    /// Sandbox profile or capability envelope drifted from the remembered rule.
    SandboxBindingDrift,
    /// The signed root-authority proof was missing or unverified for an
    /// admin/root change.
    RootAuthorityProofMissing,
}

impl IssuerBoundaryRejectionReason {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelfAuthorizationByNonIssuer => "self_authorization_by_non_issuer",
            Self::AmbientPrivilegeInferred => "ambient_privilege_inferred",
            Self::MissingIssuerBinding => "missing_issuer_binding",
            Self::IssuerNotAllowedForSurface => "issuer_not_allowed_for_surface",
            Self::RememberedDecisionMissing => "remembered_decision_missing",
            Self::RememberedDecisionTooBroad => "remembered_decision_too_broad",
            Self::RememberedDecisionLifetimeExceedsBudget => {
                "remembered_decision_lifetime_exceeds_budget"
            }
            Self::RememberedDecisionForbiddenClass => "remembered_decision_forbidden_class",
            Self::RememberedDecisionTargetDrift => "remembered_decision_target_drift",
            Self::RememberedDecisionActorDrift => "remembered_decision_actor_drift",
            Self::AuthoritySourceMismatch => "authority_source_mismatch",
            Self::AuthoritySourceUnreachableTarget => "authority_source_unreachable_target",
            Self::PolicyEpochDrift => "policy_epoch_drift",
            Self::SandboxBindingDrift => "sandbox_binding_drift",
            Self::RootAuthorityProofMissing => "root_authority_proof_missing",
        }
    }
}

/// One allowed runtime-authority issuer record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeAuthorityIssuerRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable issuer id.
    pub issuer_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Issuer class.
    pub issuer_class: AuthorityIssuerClass,
    /// Stable token for [`Self::issuer_class`].
    pub issuer_class_token: String,
    /// Opaque issuing surface ref.
    pub issuing_surface_ref: String,
    /// Ticket classes this issuer may mint or refresh.
    pub mintable_ticket_classes: Vec<AuthorityTicketClass>,
    /// Stable tokens for [`Self::mintable_ticket_classes`].
    pub mintable_ticket_class_tokens: Vec<String>,
    /// Requesting-surface classes whose requests this issuer may route.
    pub allowed_requesting_surfaces: Vec<RequestingSurfaceClass>,
    /// Stable tokens for [`Self::allowed_requesting_surfaces`].
    pub allowed_requesting_surface_tokens: Vec<String>,
    /// Authority-source classes this issuer may attest.
    pub attestable_authority_sources: Vec<AuthoritySourceClass>,
    /// Stable tokens for [`Self::attestable_authority_sources`].
    pub attestable_authority_source_tokens: Vec<String>,
    /// True when this issuer can mint root-authority changes.
    pub may_mint_root_authority_changes: bool,
    /// Opaque audit-event refs that record the issuer registration.
    pub audit_event_refs: Vec<String>,
}

/// One requesting-surface record. A requesting surface can only request
/// authority through an issuer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestingSurfaceRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable requesting-surface id.
    pub surface_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Requesting-surface class.
    pub surface_class: RequestingSurfaceClass,
    /// Stable token for [`Self::surface_class`].
    pub surface_class_token: String,
    /// Opaque requesting-surface ref carried on every request.
    pub surface_ref: String,
    /// Allowed issuer ids for this surface.
    pub allowed_issuer_ids: Vec<String>,
    /// Note describing how this surface must request authority.
    pub usage_note: String,
}

/// Narrow remembered-decision rule. A rule compiles a remembered decision to
/// a renewable short-lived ticket bound to one target, one actor, one
/// authority source, one policy epoch, one sandbox profile, one scope, and
/// an explicit expiry. Forbidden ticket classes must always reprompt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RememberedDecisionRule {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable rule id.
    pub rule_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Issuer id that owns the rule.
    pub owning_issuer_id: String,
    /// Ticket class this rule may renew.
    pub ticket_class: AuthorityTicketClass,
    /// Stable token for [`Self::ticket_class`].
    pub ticket_class_token: String,
    /// Target identity the rule narrows to.
    pub target_identity: AuthorityTargetIdentity,
    /// Actor class the rule narrows to.
    pub actor_class: AuthorityActorClass,
    /// Stable token for [`Self::actor_class`].
    pub actor_class_token: String,
    /// Opaque actor-subject ref the rule narrows to.
    pub actor_subject_ref: String,
    /// Authority-source class for the rule.
    pub authority_source_class: AuthoritySourceClass,
    /// Stable token for [`Self::authority_source_class`].
    pub authority_source_class_token: String,
    /// Opaque authority-source ref the rule narrows to.
    pub authority_source_ref: String,
    /// Opaque scope ref the rule narrows to.
    pub scope_ref: String,
    /// Sandbox and policy binding the rule narrows to.
    pub sandbox_binding: AuthoritySandboxBinding,
    /// Renewable ticket lifetime in seconds.
    pub renewable_ticket_lifetime_seconds: u64,
    /// Timestamp at which the rule itself expires.
    pub rule_expires_at: String,
    /// Opaque revoke-path ref usable from UI, CLI, support, and admin audit.
    pub revoke_path_ref: String,
    /// Opaque audit-event refs that record the rule registration.
    pub audit_event_refs: Vec<String>,
}

/// A request from a requesting surface to mint or refresh authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IssuerBoundaryRequest {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable request id.
    pub request_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Requesting surface id.
    pub requesting_surface_id: String,
    /// Requesting-surface class.
    pub requesting_surface_class: RequestingSurfaceClass,
    /// Stable token for [`Self::requesting_surface_class`].
    pub requesting_surface_class_token: String,
    /// Opaque requesting-surface ref.
    pub requesting_surface_ref: String,
    /// Issuer id that the surface routed this request through; empty when the
    /// request attempted to self-authorize.
    pub routed_issuer_id: String,
    /// Ticket class requested.
    pub requested_ticket_class: AuthorityTicketClass,
    /// Stable token for [`Self::requested_ticket_class`].
    pub requested_ticket_class_token: String,
    /// Optional remembered-rule id when the request asks to renew.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remembered_rule_id: Option<String>,
    /// Actor class accompanying the request.
    pub actor_class: AuthorityActorClass,
    /// Stable token for [`Self::actor_class`].
    pub actor_class_token: String,
    /// Opaque actor subject ref.
    pub actor_subject_ref: String,
    /// Authority-source class accompanying the request.
    pub authority_source_class: AuthoritySourceClass,
    /// Stable token for [`Self::authority_source_class`].
    pub authority_source_class_token: String,
    /// Opaque authority-source ref.
    pub authority_source_ref: String,
    /// Target identity requested.
    pub requested_target_identity: AuthorityTargetIdentity,
    /// Sandbox and policy binding requested.
    pub requested_sandbox_binding: AuthoritySandboxBinding,
    /// True when the surface attempted to bypass the issuer boundary.
    pub claims_self_authorization: bool,
    /// True when the surface inferred ambient privilege from its invoking surface.
    pub claims_ambient_privilege: bool,
    /// True when an authoritative root-authority proof was attached.
    pub root_authority_proof_present: bool,
    /// Timestamp at which the request was submitted.
    pub requested_at: String,
    /// Opaque audit-event refs that record the request.
    pub audit_event_refs: Vec<String>,
}

/// Decision the issuer-boundary evaluator returned for a request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IssuerBoundaryDecision {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable decision id.
    pub decision_id: String,
    /// Stable request id this decision answered.
    pub request_id: String,
    /// Decision class.
    pub decision_class: IssuerBoundaryDecisionClass,
    /// Stable token for [`Self::decision_class`].
    pub decision_class_token: String,
    /// Optional minted authority ticket ref when admitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minted_authority_ticket_ref: Option<String>,
    /// Optional remembered-rule ref when admitted from a remembered rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renewed_from_rule_id: Option<String>,
    /// Closed rejection reasons when refused. Empty when admitted.
    pub rejection_reasons: Vec<IssuerBoundaryRejectionReason>,
    /// Stable tokens for [`Self::rejection_reasons`].
    pub rejection_reason_tokens: Vec<String>,
    /// Actor class surfaced to UI, CLI, support, and audit.
    pub actor_class: AuthorityActorClass,
    /// Stable token for [`Self::actor_class`].
    pub actor_class_token: String,
    /// Authority-source class surfaced to UI, CLI, support, and audit.
    pub authority_source_class: AuthoritySourceClass,
    /// Stable token for [`Self::authority_source_class`].
    pub authority_source_class_token: String,
    /// Issuer that mediated this decision.
    pub issuer_class: AuthorityIssuerClass,
    /// Stable token for [`Self::issuer_class`].
    pub issuer_class_token: String,
    /// Export-safe explanation that remains visible across surfaces.
    pub explanation: String,
    /// True when local editing is preserved after a refusal.
    pub local_editing_preserved: bool,
    /// True when the surface must reprompt before retry.
    pub reprompt_required: bool,
    /// Timestamp at which the decision was emitted.
    pub decided_at: String,
    /// Opaque audit-event refs.
    pub audit_event_refs: Vec<String>,
}

/// Defect-kind vocabulary surfaced by the validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeAuthorityIssuerDefectKind {
    /// A record kind or schema-version field drifted.
    RecordShapeDrift,
    /// A stable token did not match its enum field.
    TokenDrift,
    /// An issuer class was not one of shell/policy_service/supervisor.
    IssuerNotAllowed,
    /// An issuer claimed it can mint a ticket class outside its allowed scope.
    IssuerOverreach,
    /// An issuer claimed it can mint root-authority changes without supervisor or policy seat.
    UnauthorizedRootAuthorityClaim,
    /// A requesting-surface record routes through no issuer.
    RequestingSurfaceMissingIssuer,
    /// A requesting-surface record's allowed issuer is not registered.
    RequestingSurfaceIssuerNotFound,
    /// A remembered rule lacks a narrow target or scope binding.
    RememberedRuleNotNarrow,
    /// A remembered rule's renewable lifetime exceeds the class budget.
    RememberedRuleLifetimeExceedsBudget,
    /// A remembered rule was attached to a forbidden ticket class.
    RememberedRuleForbiddenClass,
    /// A request claimed self-authorization but was admitted.
    AdmittedSelfAuthorization,
    /// A request claimed ambient privilege but was admitted.
    AdmittedAmbientPrivilege,
    /// A request routed through an issuer not allowed for its surface class.
    RequestRoutedThroughForbiddenIssuer,
    /// A decision admitted a request without a matching authority chain.
    DecisionAdmittedWithoutChain,
    /// A decision refused a request without at least one rejection reason.
    RefusedDecisionMissingReason,
    /// A decision admitted across an authority-source mismatch with the actor.
    DecisionAdmittedOnSourceMismatch,
    /// A decision admitted a remembered renewal beyond the rule's expiry.
    DecisionAdmittedBeyondRuleExpiry,
    /// A decision dropped local editing or reprompt on a non-issuer refusal.
    DecisionDroppedRecoveryGuidance,
    /// One required requesting-surface class was not represented.
    RequestingSurfaceCoverageMissing,
    /// One required rejection reason was not represented across the page.
    RejectionReasonCoverageMissing,
    /// The lineage packet failed to preserve a recognised lineage element.
    LineagePacketDriftedFromPage,
}

impl RuntimeAuthorityIssuerDefectKind {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecordShapeDrift => "record_shape_drift",
            Self::TokenDrift => "token_drift",
            Self::IssuerNotAllowed => "issuer_not_allowed",
            Self::IssuerOverreach => "issuer_overreach",
            Self::UnauthorizedRootAuthorityClaim => "unauthorized_root_authority_claim",
            Self::RequestingSurfaceMissingIssuer => "requesting_surface_missing_issuer",
            Self::RequestingSurfaceIssuerNotFound => "requesting_surface_issuer_not_found",
            Self::RememberedRuleNotNarrow => "remembered_rule_not_narrow",
            Self::RememberedRuleLifetimeExceedsBudget => "remembered_rule_lifetime_exceeds_budget",
            Self::RememberedRuleForbiddenClass => "remembered_rule_forbidden_class",
            Self::AdmittedSelfAuthorization => "admitted_self_authorization",
            Self::AdmittedAmbientPrivilege => "admitted_ambient_privilege",
            Self::RequestRoutedThroughForbiddenIssuer => "request_routed_through_forbidden_issuer",
            Self::DecisionAdmittedWithoutChain => "decision_admitted_without_chain",
            Self::RefusedDecisionMissingReason => "refused_decision_missing_reason",
            Self::DecisionAdmittedOnSourceMismatch => "decision_admitted_on_source_mismatch",
            Self::DecisionAdmittedBeyondRuleExpiry => "decision_admitted_beyond_rule_expiry",
            Self::DecisionDroppedRecoveryGuidance => "decision_dropped_recovery_guidance",
            Self::RequestingSurfaceCoverageMissing => "requesting_surface_coverage_missing",
            Self::RejectionReasonCoverageMissing => "rejection_reason_coverage_missing",
            Self::LineagePacketDriftedFromPage => "lineage_packet_drifted_from_page",
        }
    }
}

/// Typed validation defect for the runtime-authority-issuer page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeAuthorityIssuerDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Defect kind.
    pub defect_kind: RuntimeAuthorityIssuerDefectKind,
    /// Stable token for [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Subject id.
    pub subject_id: String,
    /// Field that failed validation.
    pub field: String,
    /// Export-safe note.
    pub note: String,
}

impl RuntimeAuthorityIssuerDefect {
    fn new(
        defect_kind: RuntimeAuthorityIssuerDefectKind,
        subject_id: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: RUNTIME_AUTHORITY_ISSUER_DEFECT_RECORD_KIND.to_owned(),
            schema_version: RUNTIME_AUTHORITY_ISSUER_SCHEMA_VERSION,
            shared_contract_ref: RUNTIME_AUTHORITY_ISSUER_SHARED_CONTRACT_REF.to_owned(),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            subject_id: subject_id.into(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Aggregate summary for the runtime-authority-issuer page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeAuthorityIssuerSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Number of registered issuers.
    pub issuer_count: usize,
    /// Number of registered requesting surfaces.
    pub requesting_surface_count: usize,
    /// Number of remembered-decision rules.
    pub remembered_rule_count: usize,
    /// Number of boundary requests.
    pub request_count: usize,
    /// Number of boundary decisions.
    pub decision_count: usize,
    /// Requesting-surface class tokens present.
    pub requesting_surface_classes_present: Vec<String>,
    /// Authority-source class tokens present on requests.
    pub authority_source_classes_present: Vec<String>,
    /// Decisions by decision-class token.
    pub decisions_by_class: BTreeMap<String, usize>,
    /// Rejection reason tokens present across the page.
    pub rejection_reasons_present: Vec<String>,
    /// Defect count.
    pub defect_count: usize,
    /// Defect counts by kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
}

impl RuntimeAuthorityIssuerSummary {
    /// Builds a summary from records.
    pub fn from_records(
        issuers: &[RuntimeAuthorityIssuerRecord],
        requesting_surfaces: &[RequestingSurfaceRecord],
        remembered_rules: &[RememberedDecisionRule],
        requests: &[IssuerBoundaryRequest],
        decisions: &[IssuerBoundaryDecision],
        defects: &[RuntimeAuthorityIssuerDefect],
    ) -> Self {
        let requesting_surface_classes_present = requesting_surfaces
            .iter()
            .map(|surface| surface.surface_class_token.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let authority_source_classes_present = requests
            .iter()
            .map(|request| request.authority_source_class_token.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let mut decisions_by_class = BTreeMap::new();
        for decision in decisions {
            *decisions_by_class
                .entry(decision.decision_class_token.clone())
                .or_insert(0) += 1;
        }
        let rejection_reasons_present = decisions
            .iter()
            .flat_map(|decision| decision.rejection_reason_tokens.iter().cloned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let mut defect_counts_by_kind = BTreeMap::new();
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind_token.clone())
                .or_insert(0) += 1;
        }
        Self {
            record_kind: RUNTIME_AUTHORITY_ISSUER_SUMMARY_RECORD_KIND.to_owned(),
            issuer_count: issuers.len(),
            requesting_surface_count: requesting_surfaces.len(),
            remembered_rule_count: remembered_rules.len(),
            request_count: requests.len(),
            decision_count: decisions.len(),
            requesting_surface_classes_present,
            authority_source_classes_present,
            decisions_by_class,
            rejection_reasons_present,
            defect_count: defects.len(),
            defect_counts_by_kind,
        }
    }
}

/// Top-level runtime-authority-issuer page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeAuthorityIssuerPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Source matrix ref.
    pub source_matrix_ref: String,
    /// Registered issuers.
    pub issuers: Vec<RuntimeAuthorityIssuerRecord>,
    /// Registered requesting surfaces.
    pub requesting_surfaces: Vec<RequestingSurfaceRecord>,
    /// Remembered-decision rules.
    pub remembered_rules: Vec<RememberedDecisionRule>,
    /// Boundary requests submitted by non-issuer surfaces.
    pub requests: Vec<IssuerBoundaryRequest>,
    /// Decisions returned by the boundary evaluator.
    pub decisions: Vec<IssuerBoundaryDecision>,
    /// Typed validation defects.
    pub defects: Vec<RuntimeAuthorityIssuerDefect>,
    /// Aggregate summary.
    pub summary: RuntimeAuthorityIssuerSummary,
}

/// Support-safe privileged-action lineage packet derived from a page. The
/// packet is the artifact published in release evidence and consumed by
/// admin audits and support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeAuthorityLineagePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Packet emission timestamp.
    pub generated_at: String,
    /// Lineage rows, one per boundary decision.
    pub lineage_rows: Vec<RuntimeAuthorityLineageRow>,
    /// Counts of rejection reasons preserved verbatim from the page.
    pub rejection_reason_counts: BTreeMap<String, usize>,
    /// True when raw credentials and raw authority payloads are excluded.
    pub raw_credentials_excluded: bool,
    /// True when issuer, requesting-surface, actor, and authority-source
    /// fields remain explicit (no conflation between local-only and
    /// provider-linked actions).
    pub provider_versus_local_distinguished: bool,
    /// Reviewable redaction summary.
    pub redaction_summary: String,
}

/// One row in the privileged-action lineage packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeAuthorityLineageRow {
    /// Decision id this row was projected from.
    pub decision_id: String,
    /// Request id the decision answered.
    pub request_id: String,
    /// Requesting-surface class token.
    pub requesting_surface_class_token: String,
    /// Opaque requesting-surface ref.
    pub requesting_surface_ref: String,
    /// Issuer class token that mediated the decision.
    pub issuer_class_token: String,
    /// Ticket class token requested.
    pub requested_ticket_class_token: String,
    /// Actor class token.
    pub actor_class_token: String,
    /// Authority-source class token.
    pub authority_source_class_token: String,
    /// Decision class token.
    pub decision_class_token: String,
    /// Closed rejection reason tokens, empty when admitted.
    pub rejection_reason_tokens: Vec<String>,
    /// Optional minted authority ticket ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minted_authority_ticket_ref: Option<String>,
    /// Optional renewed-from rule id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renewed_from_rule_id: Option<String>,
    /// Export-safe explanation.
    pub explanation: String,
    /// Decision timestamp.
    pub decided_at: String,
    /// Audit-event refs preserved verbatim.
    pub audit_event_refs: Vec<String>,
}

impl RuntimeAuthorityLineagePacket {
    /// Builds a lineage packet from a validated page.
    pub fn from_page(
        packet_id: impl Into<String>,
        display_label: impl Into<String>,
        generated_at: impl Into<String>,
        page: &RuntimeAuthorityIssuerPage,
    ) -> Self {
        let mut rejection_reason_counts: BTreeMap<String, usize> = BTreeMap::new();
        let mut lineage_rows = Vec::with_capacity(page.decisions.len());
        for decision in &page.decisions {
            for token in &decision.rejection_reason_tokens {
                *rejection_reason_counts.entry(token.clone()).or_insert(0) += 1;
            }
            let request = page
                .requests
                .iter()
                .find(|request| request.request_id == decision.request_id);
            lineage_rows.push(RuntimeAuthorityLineageRow {
                decision_id: decision.decision_id.clone(),
                request_id: decision.request_id.clone(),
                requesting_surface_class_token: request
                    .map(|request| request.requesting_surface_class_token.clone())
                    .unwrap_or_default(),
                requesting_surface_ref: request
                    .map(|request| request.requesting_surface_ref.clone())
                    .unwrap_or_default(),
                issuer_class_token: decision.issuer_class_token.clone(),
                requested_ticket_class_token: request
                    .map(|request| request.requested_ticket_class_token.clone())
                    .unwrap_or_default(),
                actor_class_token: decision.actor_class_token.clone(),
                authority_source_class_token: decision.authority_source_class_token.clone(),
                decision_class_token: decision.decision_class_token.clone(),
                rejection_reason_tokens: decision.rejection_reason_tokens.clone(),
                minted_authority_ticket_ref: decision.minted_authority_ticket_ref.clone(),
                renewed_from_rule_id: decision.renewed_from_rule_id.clone(),
                explanation: decision.explanation.clone(),
                decided_at: decision.decided_at.clone(),
                audit_event_refs: decision.audit_event_refs.clone(),
            });
        }
        Self {
            record_kind: RUNTIME_AUTHORITY_LINEAGE_PACKET_RECORD_KIND.to_owned(),
            schema_version: RUNTIME_AUTHORITY_ISSUER_SCHEMA_VERSION,
            shared_contract_ref: RUNTIME_AUTHORITY_ISSUER_SHARED_CONTRACT_REF.to_owned(),
            packet_id: packet_id.into(),
            display_label: display_label.into(),
            generated_at: generated_at.into(),
            lineage_rows,
            rejection_reason_counts,
            raw_credentials_excluded: true,
            provider_versus_local_distinguished: true,
            redaction_summary:
                "Metadata-only runtime-authority lineage packet: issuer class, requesting-surface \
                 class, requesting-surface ref, ticket class, actor class, authority-source \
                 class, decision class, rejection reasons, minted ticket ref, renewed-from rule \
                 id, audit refs, and decision timestamps are preserved verbatim. Raw \
                 credentials, raw authority bodies, raw policy payloads, raw evidence bodies, \
                 and plaintext secret material are excluded."
                    .to_owned(),
        }
    }
}

/// Validates a runtime-authority-issuer page.
pub fn validate_runtime_authority_issuer_page(
    page: &RuntimeAuthorityIssuerPage,
) -> Result<(), Vec<RuntimeAuthorityIssuerDefect>> {
    let defects = audit_runtime_authority_issuer_page(
        &page.issuers,
        &page.requesting_surfaces,
        &page.remembered_rules,
        &page.requests,
        &page.decisions,
    );
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Recomputes runtime-authority-issuer defects from records.
pub fn audit_runtime_authority_issuer_page(
    issuers: &[RuntimeAuthorityIssuerRecord],
    requesting_surfaces: &[RequestingSurfaceRecord],
    remembered_rules: &[RememberedDecisionRule],
    requests: &[IssuerBoundaryRequest],
    decisions: &[IssuerBoundaryDecision],
) -> Vec<RuntimeAuthorityIssuerDefect> {
    let mut defects = Vec::new();
    let issuers_by_id: BTreeMap<&str, &RuntimeAuthorityIssuerRecord> = issuers
        .iter()
        .map(|issuer| (issuer.issuer_id.as_str(), issuer))
        .collect();
    let surfaces_by_id: BTreeMap<&str, &RequestingSurfaceRecord> = requesting_surfaces
        .iter()
        .map(|surface| (surface.surface_id.as_str(), surface))
        .collect();
    let rules_by_id: BTreeMap<&str, &RememberedDecisionRule> = remembered_rules
        .iter()
        .map(|rule| (rule.rule_id.as_str(), rule))
        .collect();
    let requests_by_id: BTreeMap<&str, &IssuerBoundaryRequest> = requests
        .iter()
        .map(|request| (request.request_id.as_str(), request))
        .collect();

    for issuer in issuers {
        check_issuer(&mut defects, issuer);
    }
    for surface in requesting_surfaces {
        check_requesting_surface(&mut defects, surface, &issuers_by_id);
    }
    for rule in remembered_rules {
        check_remembered_rule(&mut defects, rule);
    }
    for request in requests {
        check_request(&mut defects, request, &surfaces_by_id, &issuers_by_id);
    }
    for decision in decisions {
        check_decision(&mut defects, decision, &requests_by_id, &rules_by_id);
    }

    let observed_surface_classes = requesting_surfaces
        .iter()
        .map(|surface| surface.surface_class_token.as_str())
        .collect::<BTreeSet<_>>();
    for required in RequestingSurfaceClass::ALL {
        if !observed_surface_classes.contains(required.as_str()) {
            defects.push(RuntimeAuthorityIssuerDefect::new(
                RuntimeAuthorityIssuerDefectKind::RequestingSurfaceCoverageMissing,
                "page",
                "requesting_surfaces",
                format!("missing {} requesting surface", required.as_str()),
            ));
        }
    }

    let observed_rejection_reasons = decisions
        .iter()
        .flat_map(|decision| decision.rejection_reason_tokens.iter().map(String::as_str))
        .collect::<BTreeSet<_>>();
    for required in [
        IssuerBoundaryRejectionReason::SelfAuthorizationByNonIssuer,
        IssuerBoundaryRejectionReason::AmbientPrivilegeInferred,
        IssuerBoundaryRejectionReason::RememberedDecisionTooBroad,
        IssuerBoundaryRejectionReason::AuthoritySourceMismatch,
    ] {
        if !observed_rejection_reasons.contains(required.as_str()) {
            defects.push(RuntimeAuthorityIssuerDefect::new(
                RuntimeAuthorityIssuerDefectKind::RejectionReasonCoverageMissing,
                "page",
                "decisions",
                format!("missing {} rejection reason coverage", required.as_str()),
            ));
        }
    }

    defects
}

fn check_issuer(
    defects: &mut Vec<RuntimeAuthorityIssuerDefect>,
    issuer: &RuntimeAuthorityIssuerRecord,
) {
    if issuer.record_kind != RUNTIME_AUTHORITY_ISSUER_RECORD_KIND
        || issuer.schema_version != RUNTIME_AUTHORITY_ISSUER_SCHEMA_VERSION
        || issuer.shared_contract_ref != RUNTIME_AUTHORITY_ISSUER_SHARED_CONTRACT_REF
    {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::RecordShapeDrift,
            issuer.issuer_id.clone(),
            "record_kind/schema_version/shared_contract_ref",
            "issuer record shape must match the runtime-authority-issuer contract",
        ));
    }
    if issuer.issuer_class_token != issuer.issuer_class.as_str() {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::TokenDrift,
            issuer.issuer_id.clone(),
            "issuer_class_token",
            "issuer_class_token must match issuer_class",
        ));
    }
    if issuer.mintable_ticket_classes.len() != issuer.mintable_ticket_class_tokens.len()
        || issuer.allowed_requesting_surfaces.len()
            != issuer.allowed_requesting_surface_tokens.len()
        || issuer.attestable_authority_sources.len()
            != issuer.attestable_authority_source_tokens.len()
    {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::TokenDrift,
            issuer.issuer_id.clone(),
            "mintable_ticket_class_tokens/allowed_requesting_surface_tokens/attestable_authority_source_tokens",
            "issuer record token vectors must mirror their enum vectors",
        ));
    } else {
        for (class, token) in issuer
            .mintable_ticket_classes
            .iter()
            .zip(issuer.mintable_ticket_class_tokens.iter())
        {
            if token != class.as_str() {
                defects.push(RuntimeAuthorityIssuerDefect::new(
                    RuntimeAuthorityIssuerDefectKind::TokenDrift,
                    issuer.issuer_id.clone(),
                    "mintable_ticket_class_tokens",
                    "mintable ticket class token must match its enum value",
                ));
                break;
            }
        }
        for (class, token) in issuer
            .allowed_requesting_surfaces
            .iter()
            .zip(issuer.allowed_requesting_surface_tokens.iter())
        {
            if token != class.as_str() {
                defects.push(RuntimeAuthorityIssuerDefect::new(
                    RuntimeAuthorityIssuerDefectKind::TokenDrift,
                    issuer.issuer_id.clone(),
                    "allowed_requesting_surface_tokens",
                    "allowed requesting surface token must match its enum value",
                ));
                break;
            }
        }
        for (class, token) in issuer
            .attestable_authority_sources
            .iter()
            .zip(issuer.attestable_authority_source_tokens.iter())
        {
            if token != class.as_str() {
                defects.push(RuntimeAuthorityIssuerDefect::new(
                    RuntimeAuthorityIssuerDefectKind::TokenDrift,
                    issuer.issuer_id.clone(),
                    "attestable_authority_source_tokens",
                    "attestable authority source token must match its enum value",
                ));
                break;
            }
        }
    }
    // Issuer overreach: only supervisor or policy_service may attest root
    // authority. Shell may only request user-level mints.
    if issuer.may_mint_root_authority_changes
        && !matches!(
            issuer.issuer_class,
            AuthorityIssuerClass::Supervisor | AuthorityIssuerClass::PolicyService
        )
    {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::UnauthorizedRootAuthorityClaim,
            issuer.issuer_id.clone(),
            "may_mint_root_authority_changes",
            "only supervisor or policy_service may mint root-authority changes",
        ));
    }
    if !issuer.may_mint_root_authority_changes
        && issuer
            .mintable_ticket_classes
            .iter()
            .any(|class| matches!(class, AuthorityTicketClass::PolicyTrustAdminChange))
    {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::IssuerOverreach,
            issuer.issuer_id.clone(),
            "mintable_ticket_classes",
            "policy_trust_admin_change requires may_mint_root_authority_changes",
        ));
    }
    if issuer.issuer_class == AuthorityIssuerClass::Shell
        && issuer
            .mintable_ticket_classes
            .iter()
            .any(|class| matches!(class, AuthorityTicketClass::PolicyTrustAdminChange))
    {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::IssuerOverreach,
            issuer.issuer_id.clone(),
            "mintable_ticket_classes",
            "shell issuer cannot mint policy_trust_admin_change tickets",
        ));
    }
}

fn check_requesting_surface(
    defects: &mut Vec<RuntimeAuthorityIssuerDefect>,
    surface: &RequestingSurfaceRecord,
    issuers_by_id: &BTreeMap<&str, &RuntimeAuthorityIssuerRecord>,
) {
    if surface.record_kind != REQUESTING_SURFACE_RECORD_KIND
        || surface.schema_version != RUNTIME_AUTHORITY_ISSUER_SCHEMA_VERSION
        || surface.shared_contract_ref != RUNTIME_AUTHORITY_ISSUER_SHARED_CONTRACT_REF
    {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::RecordShapeDrift,
            surface.surface_id.clone(),
            "record_kind/schema_version/shared_contract_ref",
            "requesting-surface record shape must match the contract",
        ));
    }
    if surface.surface_class_token != surface.surface_class.as_str() {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::TokenDrift,
            surface.surface_id.clone(),
            "surface_class_token",
            "surface_class_token must match surface_class",
        ));
    }
    if surface.allowed_issuer_ids.is_empty() {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::RequestingSurfaceMissingIssuer,
            surface.surface_id.clone(),
            "allowed_issuer_ids",
            "requesting surface must route through at least one issuer",
        ));
    } else {
        for issuer_id in &surface.allowed_issuer_ids {
            match issuers_by_id.get(issuer_id.as_str()) {
                Some(issuer) => {
                    if !issuer
                        .allowed_requesting_surfaces
                        .contains(&surface.surface_class)
                    {
                        defects.push(RuntimeAuthorityIssuerDefect::new(
                            RuntimeAuthorityIssuerDefectKind::RequestingSurfaceIssuerNotFound,
                            surface.surface_id.clone(),
                            "allowed_issuer_ids",
                            "named issuer does not list this surface class as allowed",
                        ));
                    }
                }
                None => defects.push(RuntimeAuthorityIssuerDefect::new(
                    RuntimeAuthorityIssuerDefectKind::RequestingSurfaceIssuerNotFound,
                    surface.surface_id.clone(),
                    "allowed_issuer_ids",
                    "requesting surface references an unregistered issuer",
                )),
            }
        }
    }
}

fn check_remembered_rule(
    defects: &mut Vec<RuntimeAuthorityIssuerDefect>,
    rule: &RememberedDecisionRule,
) {
    if rule.record_kind != REMEMBERED_DECISION_RULE_RECORD_KIND
        || rule.schema_version != RUNTIME_AUTHORITY_ISSUER_SCHEMA_VERSION
        || rule.shared_contract_ref != RUNTIME_AUTHORITY_ISSUER_SHARED_CONTRACT_REF
    {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::RecordShapeDrift,
            rule.rule_id.clone(),
            "record_kind/schema_version/shared_contract_ref",
            "remembered-rule record shape must match the contract",
        ));
    }
    if rule.ticket_class_token != rule.ticket_class.as_str()
        || rule.actor_class_token != rule.actor_class.as_str()
        || rule.authority_source_class_token != rule.authority_source_class.as_str()
        || rule.target_identity.target_class_token != rule.target_identity.target_class.as_str()
    {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::TokenDrift,
            rule.rule_id.clone(),
            "ticket_class_token/actor_class_token/authority_source_class_token/target_identity.target_class_token",
            "remembered-rule tokens must match their enum values",
        ));
    }
    if matches!(
        rule.ticket_class,
        AuthorityTicketClass::CredentialProjection
            | AuthorityTicketClass::PrivilegedDebugAttach
            | AuthorityTicketClass::PolicyTrustAdminChange
    ) {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::RememberedRuleForbiddenClass,
            rule.rule_id.clone(),
            "ticket_class",
            "credential projection, privileged attach, and policy/trust/admin tickets must reprompt and cannot be remembered",
        ));
    }
    if rule.scope_ref.is_empty()
        || rule.actor_subject_ref.is_empty()
        || rule.authority_source_ref.is_empty()
        || rule.target_identity.target_ref.is_empty()
        || rule.sandbox_binding.policy_epoch_ref.is_empty()
        || rule.sandbox_binding.sandbox_profile_ref.is_empty()
    {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::RememberedRuleNotNarrow,
            rule.rule_id.clone(),
            "scope_ref/actor_subject_ref/authority_source_ref/target_identity/sandbox_binding",
            "remembered rule must bind explicit target, actor, scope, authority source, sandbox profile, and policy epoch",
        ));
    }
    if rule.renewable_ticket_lifetime_seconds == 0
        || rule.renewable_ticket_lifetime_seconds > rule.ticket_class.max_lifetime_seconds()
    {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::RememberedRuleLifetimeExceedsBudget,
            rule.rule_id.clone(),
            "renewable_ticket_lifetime_seconds",
            "remembered-rule renewal lifetime must not exceed the ticket class budget",
        ));
    }
    if AuthoritySourceClass::from_actor_class(rule.actor_class) != rule.authority_source_class {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::TokenDrift,
            rule.rule_id.clone(),
            "authority_source_class",
            "authority_source_class must match the projection of actor_class",
        ));
    }
}

fn check_request(
    defects: &mut Vec<RuntimeAuthorityIssuerDefect>,
    request: &IssuerBoundaryRequest,
    surfaces_by_id: &BTreeMap<&str, &RequestingSurfaceRecord>,
    issuers_by_id: &BTreeMap<&str, &RuntimeAuthorityIssuerRecord>,
) {
    if request.record_kind != ISSUER_BOUNDARY_REQUEST_RECORD_KIND
        || request.schema_version != RUNTIME_AUTHORITY_ISSUER_SCHEMA_VERSION
        || request.shared_contract_ref != RUNTIME_AUTHORITY_ISSUER_SHARED_CONTRACT_REF
    {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::RecordShapeDrift,
            request.request_id.clone(),
            "record_kind/schema_version/shared_contract_ref",
            "boundary-request record shape must match the contract",
        ));
    }
    if request.requesting_surface_class_token != request.requesting_surface_class.as_str()
        || request.requested_ticket_class_token != request.requested_ticket_class.as_str()
        || request.actor_class_token != request.actor_class.as_str()
        || request.authority_source_class_token != request.authority_source_class.as_str()
    {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::TokenDrift,
            request.request_id.clone(),
            "requesting_surface_class_token/requested_ticket_class_token/actor_class_token/authority_source_class_token",
            "request tokens must match their enum values",
        ));
    }
    // Only mark a routed-through-forbidden-issuer defect when the surface is
    // registered and the routed issuer is wrong. Self-authorization with an
    // empty routed_issuer_id is handled on the decision side.
    if !request.routed_issuer_id.is_empty() {
        match issuers_by_id.get(request.routed_issuer_id.as_str()) {
            Some(issuer) => {
                if !issuer
                    .allowed_requesting_surfaces
                    .contains(&request.requesting_surface_class)
                {
                    defects.push(RuntimeAuthorityIssuerDefect::new(
                        RuntimeAuthorityIssuerDefectKind::RequestRoutedThroughForbiddenIssuer,
                        request.request_id.clone(),
                        "routed_issuer_id",
                        "request routed through an issuer that does not list this surface class",
                    ));
                }
                if !issuer
                    .mintable_ticket_classes
                    .contains(&request.requested_ticket_class)
                {
                    defects.push(RuntimeAuthorityIssuerDefect::new(
                        RuntimeAuthorityIssuerDefectKind::RequestRoutedThroughForbiddenIssuer,
                        request.request_id.clone(),
                        "routed_issuer_id",
                        "issuer cannot mint the requested ticket class",
                    ));
                }
            }
            None => defects.push(RuntimeAuthorityIssuerDefect::new(
                RuntimeAuthorityIssuerDefectKind::RequestingSurfaceIssuerNotFound,
                request.request_id.clone(),
                "routed_issuer_id",
                "request references an unregistered issuer",
            )),
        }
    }
    if surfaces_by_id
        .get(request.requesting_surface_id.as_str())
        .is_none()
    {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::RequestingSurfaceMissingIssuer,
            request.request_id.clone(),
            "requesting_surface_id",
            "request references an unregistered requesting surface",
        ));
    }
    if AuthoritySourceClass::from_actor_class(request.actor_class) != request.authority_source_class
    {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::TokenDrift,
            request.request_id.clone(),
            "authority_source_class",
            "request authority_source_class must match the projection of actor_class",
        ));
    }
}

fn check_decision(
    defects: &mut Vec<RuntimeAuthorityIssuerDefect>,
    decision: &IssuerBoundaryDecision,
    requests_by_id: &BTreeMap<&str, &IssuerBoundaryRequest>,
    rules_by_id: &BTreeMap<&str, &RememberedDecisionRule>,
) {
    if decision.record_kind != ISSUER_BOUNDARY_DECISION_RECORD_KIND
        || decision.schema_version != RUNTIME_AUTHORITY_ISSUER_SCHEMA_VERSION
        || decision.shared_contract_ref != RUNTIME_AUTHORITY_ISSUER_SHARED_CONTRACT_REF
    {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::RecordShapeDrift,
            decision.decision_id.clone(),
            "record_kind/schema_version/shared_contract_ref",
            "boundary-decision record shape must match the contract",
        ));
    }
    if decision.decision_class_token != decision.decision_class.as_str()
        || decision.actor_class_token != decision.actor_class.as_str()
        || decision.authority_source_class_token != decision.authority_source_class.as_str()
        || decision.issuer_class_token != decision.issuer_class.as_str()
    {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::TokenDrift,
            decision.decision_id.clone(),
            "decision_class_token/actor_class_token/authority_source_class_token/issuer_class_token",
            "decision tokens must match their enum values",
        ));
    }
    if decision.rejection_reasons.len() != decision.rejection_reason_tokens.len() {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::TokenDrift,
            decision.decision_id.clone(),
            "rejection_reason_tokens",
            "rejection_reason_tokens must mirror rejection_reasons",
        ));
    } else {
        for (reason, token) in decision
            .rejection_reasons
            .iter()
            .zip(decision.rejection_reason_tokens.iter())
        {
            if token != reason.as_str() {
                defects.push(RuntimeAuthorityIssuerDefect::new(
                    RuntimeAuthorityIssuerDefectKind::TokenDrift,
                    decision.decision_id.clone(),
                    "rejection_reason_tokens",
                    "rejection reason token must match its enum value",
                ));
                break;
            }
        }
    }

    let request = requests_by_id.get(decision.request_id.as_str()).copied();

    match decision.decision_class {
        IssuerBoundaryDecisionClass::Granted
        | IssuerBoundaryDecisionClass::RememberedDecisionNarrowed => {
            if let Some(request) = request {
                if request.claims_self_authorization {
                    defects.push(RuntimeAuthorityIssuerDefect::new(
                        RuntimeAuthorityIssuerDefectKind::AdmittedSelfAuthorization,
                        decision.decision_id.clone(),
                        "decision_class",
                        "self-authorization attempts must be refused",
                    ));
                }
                if request.claims_ambient_privilege {
                    defects.push(RuntimeAuthorityIssuerDefect::new(
                        RuntimeAuthorityIssuerDefectKind::AdmittedAmbientPrivilege,
                        decision.decision_id.clone(),
                        "decision_class",
                        "ambient-privilege inferences must be refused",
                    ));
                }
                if request.routed_issuer_id.is_empty() {
                    defects.push(RuntimeAuthorityIssuerDefect::new(
                        RuntimeAuthorityIssuerDefectKind::DecisionAdmittedWithoutChain,
                        decision.decision_id.clone(),
                        "request.routed_issuer_id",
                        "admitted decisions must point at a routed issuer",
                    ));
                }
                if AuthoritySourceClass::from_actor_class(request.actor_class)
                    != decision.authority_source_class
                {
                    defects.push(RuntimeAuthorityIssuerDefect::new(
                        RuntimeAuthorityIssuerDefectKind::DecisionAdmittedOnSourceMismatch,
                        decision.decision_id.clone(),
                        "authority_source_class",
                        "admitted decisions must keep authority-source class consistent with actor class",
                    ));
                }
                if request.requested_target_identity.target_class
                    == AuthorityTargetClass::ProviderObject
                    && !decision.authority_source_class.can_reach_provider_targets()
                {
                    defects.push(RuntimeAuthorityIssuerDefect::new(
                        RuntimeAuthorityIssuerDefectKind::DecisionAdmittedOnSourceMismatch,
                        decision.decision_id.clone(),
                        "authority_source_class",
                        "local-only authority cannot admit provider-object targets",
                    ));
                }
                if decision.decision_class
                    == IssuerBoundaryDecisionClass::RememberedDecisionNarrowed
                {
                    match decision
                        .renewed_from_rule_id
                        .as_deref()
                        .and_then(|id| rules_by_id.get(id).copied())
                    {
                        Some(rule) => {
                            if rule.target_identity.target_ref
                                != request.requested_target_identity.target_ref
                                || rule.actor_subject_ref != request.actor_subject_ref
                                || rule.sandbox_binding.policy_epoch_ref
                                    != request.requested_sandbox_binding.policy_epoch_ref
                                || rule.sandbox_binding.sandbox_profile_ref
                                    != request.requested_sandbox_binding.sandbox_profile_ref
                                || rule.ticket_class != request.requested_ticket_class
                                || rule.authority_source_class != request.authority_source_class
                            {
                                defects.push(RuntimeAuthorityIssuerDefect::new(
                                    RuntimeAuthorityIssuerDefectKind::DecisionAdmittedOnSourceMismatch,
                                    decision.decision_id.clone(),
                                    "renewed_from_rule_id",
                                    "remembered-rule renewal must narrow to the same target, actor, ticket class, authority source, policy epoch, and sandbox profile",
                                ));
                            }
                            if let (Some(rule_expiry), Some(decided_at)) = (
                                parse_timestamp(&rule.rule_expires_at),
                                parse_timestamp(&decision.decided_at),
                            ) {
                                if decided_at > rule_expiry {
                                    defects.push(RuntimeAuthorityIssuerDefect::new(
                                        RuntimeAuthorityIssuerDefectKind::DecisionAdmittedBeyondRuleExpiry,
                                        decision.decision_id.clone(),
                                        "decided_at",
                                        "remembered-rule renewal admitted past the rule's expiry",
                                    ));
                                }
                            }
                        }
                        None => defects.push(RuntimeAuthorityIssuerDefect::new(
                            RuntimeAuthorityIssuerDefectKind::DecisionAdmittedWithoutChain,
                            decision.decision_id.clone(),
                            "renewed_from_rule_id",
                            "remembered renewal must resolve to a recorded remembered rule",
                        )),
                    }
                }
                if request.requested_ticket_class == AuthorityTicketClass::PolicyTrustAdminChange
                    && !request.root_authority_proof_present
                {
                    defects.push(RuntimeAuthorityIssuerDefect::new(
                        RuntimeAuthorityIssuerDefectKind::DecisionAdmittedWithoutChain,
                        decision.decision_id.clone(),
                        "request.root_authority_proof_present",
                        "policy/trust/admin admit requires a recorded root-authority proof",
                    ));
                }
            } else {
                defects.push(RuntimeAuthorityIssuerDefect::new(
                    RuntimeAuthorityIssuerDefectKind::DecisionAdmittedWithoutChain,
                    decision.decision_id.clone(),
                    "request_id",
                    "decision must reference a recorded request",
                ));
            }
        }
        IssuerBoundaryDecisionClass::Refused => {
            if decision.rejection_reasons.is_empty() {
                defects.push(RuntimeAuthorityIssuerDefect::new(
                    RuntimeAuthorityIssuerDefectKind::RefusedDecisionMissingReason,
                    decision.decision_id.clone(),
                    "rejection_reasons",
                    "refused decisions must carry at least one closed rejection reason",
                ));
            }
            if !decision.local_editing_preserved || !decision.reprompt_required {
                defects.push(RuntimeAuthorityIssuerDefect::new(
                    RuntimeAuthorityIssuerDefectKind::DecisionDroppedRecoveryGuidance,
                    decision.decision_id.clone(),
                    "local_editing_preserved/reprompt_required",
                    "refused decisions must preserve local editing and require reprompt before retry",
                ));
            }
        }
    }
    if decision.audit_event_refs.is_empty() {
        defects.push(RuntimeAuthorityIssuerDefect::new(
            RuntimeAuthorityIssuerDefectKind::RecordShapeDrift,
            decision.decision_id.clone(),
            "audit_event_refs",
            "every decision must preserve at least one audit event ref",
        ));
    }
}

/// Minimal strict UTC timestamp parser for `YYYY-MM-DDTHH:MM:SSZ`. Mirrors the
/// parser in [`crate::authority`] without re-exporting private state.
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

/// Builds the seeded runtime-authority-issuer page.
pub fn seeded_runtime_authority_issuer_page() -> RuntimeAuthorityIssuerPage {
    let issuers = seed_issuers();
    let requesting_surfaces = seed_requesting_surfaces();
    let remembered_rules = seed_remembered_rules();
    let requests = seed_requests();
    let decisions = seed_decisions();
    let defects = audit_runtime_authority_issuer_page(
        &issuers,
        &requesting_surfaces,
        &remembered_rules,
        &requests,
        &decisions,
    );
    let summary = RuntimeAuthorityIssuerSummary::from_records(
        &issuers,
        &requesting_surfaces,
        &remembered_rules,
        &requests,
        &decisions,
        &defects,
    );
    RuntimeAuthorityIssuerPage {
        record_kind: RUNTIME_AUTHORITY_ISSUER_PAGE_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_AUTHORITY_ISSUER_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_AUTHORITY_ISSUER_SHARED_CONTRACT_REF.to_owned(),
        source_matrix_ref: RUNTIME_AUTHORITY_ISSUER_SOURCE_MATRIX_REF.to_owned(),
        issuers,
        requesting_surfaces,
        remembered_rules,
        requests,
        decisions,
        defects,
        summary,
    }
}

#[allow(clippy::too_many_arguments)]
fn issuer(
    issuer_id: &str,
    display_label: &str,
    issuer_class: AuthorityIssuerClass,
    issuing_surface_ref: &str,
    mintable_ticket_classes: Vec<AuthorityTicketClass>,
    allowed_requesting_surfaces: Vec<RequestingSurfaceClass>,
    attestable_authority_sources: Vec<AuthoritySourceClass>,
    may_mint_root_authority_changes: bool,
    audit_event_refs: Vec<&str>,
) -> RuntimeAuthorityIssuerRecord {
    let mintable_ticket_class_tokens = mintable_ticket_classes
        .iter()
        .map(|class| class.as_str().to_owned())
        .collect();
    let allowed_requesting_surface_tokens = allowed_requesting_surfaces
        .iter()
        .map(|class| class.as_str().to_owned())
        .collect();
    let attestable_authority_source_tokens = attestable_authority_sources
        .iter()
        .map(|class| class.as_str().to_owned())
        .collect();
    RuntimeAuthorityIssuerRecord {
        record_kind: RUNTIME_AUTHORITY_ISSUER_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_AUTHORITY_ISSUER_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_AUTHORITY_ISSUER_SHARED_CONTRACT_REF.to_owned(),
        issuer_id: issuer_id.to_owned(),
        display_label: display_label.to_owned(),
        issuer_class,
        issuer_class_token: issuer_class.as_str().to_owned(),
        issuing_surface_ref: issuing_surface_ref.to_owned(),
        mintable_ticket_classes,
        mintable_ticket_class_tokens,
        allowed_requesting_surfaces,
        allowed_requesting_surface_tokens,
        attestable_authority_sources,
        attestable_authority_source_tokens,
        may_mint_root_authority_changes,
        audit_event_refs: audit_event_refs.into_iter().map(String::from).collect(),
    }
}

fn seed_issuers() -> Vec<RuntimeAuthorityIssuerRecord> {
    vec![
        issuer(
            "issuer:shell:desktop",
            "Desktop shell approval surface",
            AuthorityIssuerClass::Shell,
            "shell:desktop:approval-surface",
            vec![
                AuthorityTicketClass::LocalMutation,
                AuthorityTicketClass::ExternalProviderMutation,
                AuthorityTicketClass::CredentialProjection,
                AuthorityTicketClass::PrivilegedDebugAttach,
            ],
            vec![
                RequestingSurfaceClass::AdminConsole,
                RequestingSurfaceClass::LocalAdminTool,
                RequestingSurfaceClass::AiTool,
                RequestingSurfaceClass::Extension,
                RequestingSurfaceClass::RecipeRunner,
                RequestingSurfaceClass::CliScript,
                RequestingSurfaceClass::BrowserCompanion,
                RequestingSurfaceClass::RemoteHelper,
                RequestingSurfaceClass::AutomationScheduler,
            ],
            vec![
                AuthoritySourceClass::HumanAccount,
                AuthoritySourceClass::DelegatedCredential,
                AuthoritySourceClass::LocalOnlyAuthority,
            ],
            false,
            vec!["audit:runtime-authority-issuer:shell:registered"],
        ),
        issuer(
            "issuer:policy-service:primary",
            "Policy service issuer",
            AuthorityIssuerClass::PolicyService,
            "policy-service:primary:issuer",
            vec![
                AuthorityTicketClass::LocalMutation,
                AuthorityTicketClass::ExternalProviderMutation,
                AuthorityTicketClass::CredentialProjection,
                AuthorityTicketClass::PrivilegedDebugAttach,
                AuthorityTicketClass::PolicyTrustAdminChange,
            ],
            vec![
                RequestingSurfaceClass::AdminConsole,
                RequestingSurfaceClass::AutomationScheduler,
                RequestingSurfaceClass::CliScript,
                RequestingSurfaceClass::Extension,
            ],
            vec![
                AuthoritySourceClass::HumanAccount,
                AuthoritySourceClass::InstallationGrant,
                AuthoritySourceClass::DelegatedCredential,
                AuthoritySourceClass::LocalOnlyAuthority,
            ],
            true,
            vec!["audit:runtime-authority-issuer:policy-service:registered"],
        ),
        issuer(
            "issuer:supervisor:control-plane",
            "Supervisor control-plane issuer",
            AuthorityIssuerClass::Supervisor,
            "supervisor:control-plane:issuer",
            vec![
                AuthorityTicketClass::ExternalProviderMutation,
                AuthorityTicketClass::CredentialProjection,
                AuthorityTicketClass::PolicyTrustAdminChange,
            ],
            vec![
                RequestingSurfaceClass::AdminConsole,
                RequestingSurfaceClass::AutomationScheduler,
                RequestingSurfaceClass::RemoteHelper,
            ],
            vec![
                AuthoritySourceClass::HumanAccount,
                AuthoritySourceClass::InstallationGrant,
            ],
            true,
            vec!["audit:runtime-authority-issuer:supervisor:registered"],
        ),
    ]
}

fn requesting_surface(
    surface_id: &str,
    display_label: &str,
    surface_class: RequestingSurfaceClass,
    surface_ref: &str,
    allowed_issuer_ids: Vec<&str>,
    usage_note: &str,
) -> RequestingSurfaceRecord {
    RequestingSurfaceRecord {
        record_kind: REQUESTING_SURFACE_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_AUTHORITY_ISSUER_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_AUTHORITY_ISSUER_SHARED_CONTRACT_REF.to_owned(),
        surface_id: surface_id.to_owned(),
        display_label: display_label.to_owned(),
        surface_class,
        surface_class_token: surface_class.as_str().to_owned(),
        surface_ref: surface_ref.to_owned(),
        allowed_issuer_ids: allowed_issuer_ids.into_iter().map(String::from).collect(),
        usage_note: usage_note.to_owned(),
    }
}

fn seed_requesting_surfaces() -> Vec<RequestingSurfaceRecord> {
    vec![
        requesting_surface(
            "surface:admin-console:primary",
            "Admin console",
            RequestingSurfaceClass::AdminConsole,
            "admin-console:requesting-surface",
            vec![
                "issuer:policy-service:primary",
                "issuer:supervisor:control-plane",
                "issuer:shell:desktop",
            ],
            "Admin console actions request authority through the policy service, supervisor, or desktop shell.",
        ),
        requesting_surface(
            "surface:local-admin:cli",
            "Local administrator CLI",
            RequestingSurfaceClass::LocalAdminTool,
            "local-admin:cli:requesting-surface",
            vec!["issuer:shell:desktop"],
            "Local administrator tooling can only request authority through the desktop shell approval surface.",
        ),
        requesting_surface(
            "surface:ai-tool:plan",
            "AI tool plan",
            RequestingSurfaceClass::AiTool,
            "ai-tool:plan:requesting-surface",
            vec!["issuer:shell:desktop"],
            "AI tool plans can only request authority through the shell prompt; they may never mint or refresh authority.",
        ),
        requesting_surface(
            "surface:extension:workspace",
            "Workspace extension",
            RequestingSurfaceClass::Extension,
            "extension:workspace:requesting-surface",
            vec!["issuer:shell:desktop", "issuer:policy-service:primary"],
            "Extensions request authority through the shell or policy service; ambient privilege inferred from the host surface is rejected.",
        ),
        requesting_surface(
            "surface:recipe:runner",
            "Recipe runner",
            RequestingSurfaceClass::RecipeRunner,
            "recipe:runner:requesting-surface",
            vec!["issuer:shell:desktop"],
            "Recipe steps request authority through the shell. Recipe runners cannot reuse prior remembered decisions outside their narrow class.",
        ),
        requesting_surface(
            "surface:cli:script",
            "CLI script",
            RequestingSurfaceClass::CliScript,
            "cli:script:requesting-surface",
            vec!["issuer:shell:desktop", "issuer:policy-service:primary"],
            "CLI scripts request authority through the shell or policy service. CLI scripts cannot self-authorize even when run from a privileged terminal.",
        ),
        requesting_surface(
            "surface:browser-companion:default",
            "Browser companion",
            RequestingSurfaceClass::BrowserCompanion,
            "browser-companion:requesting-surface",
            vec!["issuer:shell:desktop"],
            "Browser companions request authority through the shell. The companion's own session cookies never grant runtime authority by themselves.",
        ),
        requesting_surface(
            "surface:remote-helper:default",
            "Remote helper",
            RequestingSurfaceClass::RemoteHelper,
            "remote-helper:requesting-surface",
            vec!["issuer:shell:desktop", "issuer:supervisor:control-plane"],
            "Remote helpers request authority through the shell or supervisor control plane. Remote helpers cannot mint authority based on remote tunnel posture alone.",
        ),
        requesting_surface(
            "surface:automation-scheduler:default",
            "Automation scheduler",
            RequestingSurfaceClass::AutomationScheduler,
            "automation-scheduler:requesting-surface",
            vec![
                "issuer:policy-service:primary",
                "issuer:supervisor:control-plane",
                "issuer:shell:desktop",
            ],
            "Automation scheduler jobs request authority through policy, supervisor, or shell. Scheduled runs cannot inherit ambient authority from past runs.",
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn remembered_rule(
    rule_id: &str,
    display_label: &str,
    owning_issuer_id: &str,
    ticket_class: AuthorityTicketClass,
    target_identity: AuthorityTargetIdentity,
    actor_class: AuthorityActorClass,
    actor_subject_ref: &str,
    authority_source_class: AuthoritySourceClass,
    authority_source_ref: &str,
    scope_ref: &str,
    sandbox_binding: AuthoritySandboxBinding,
    renewable_ticket_lifetime_seconds: u64,
    rule_expires_at: &str,
    revoke_path_ref: &str,
    audit_event_refs: Vec<&str>,
) -> RememberedDecisionRule {
    RememberedDecisionRule {
        record_kind: REMEMBERED_DECISION_RULE_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_AUTHORITY_ISSUER_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_AUTHORITY_ISSUER_SHARED_CONTRACT_REF.to_owned(),
        rule_id: rule_id.to_owned(),
        display_label: display_label.to_owned(),
        owning_issuer_id: owning_issuer_id.to_owned(),
        ticket_class,
        ticket_class_token: ticket_class.as_str().to_owned(),
        target_identity,
        actor_class,
        actor_class_token: actor_class.as_str().to_owned(),
        actor_subject_ref: actor_subject_ref.to_owned(),
        authority_source_class,
        authority_source_class_token: authority_source_class.as_str().to_owned(),
        authority_source_ref: authority_source_ref.to_owned(),
        scope_ref: scope_ref.to_owned(),
        sandbox_binding,
        renewable_ticket_lifetime_seconds,
        rule_expires_at: rule_expires_at.to_owned(),
        revoke_path_ref: revoke_path_ref.to_owned(),
        audit_event_refs: audit_event_refs.into_iter().map(String::from).collect(),
    }
}

fn local_target() -> AuthorityTargetIdentity {
    AuthorityTargetIdentity {
        target_class: AuthorityTargetClass::LocalWorkspace,
        target_class_token: AuthorityTargetClass::LocalWorkspace.as_str().to_owned(),
        target_ref: "workspace:aureline:current-repo".to_owned(),
        target_label: "Aureline current repository".to_owned(),
        target_fingerprint_ref: "target-fingerprint:workspace:aureline:v1".to_owned(),
    }
}

fn provider_target() -> AuthorityTargetIdentity {
    AuthorityTargetIdentity {
        target_class: AuthorityTargetClass::ProviderObject,
        target_class_token: AuthorityTargetClass::ProviderObject.as_str().to_owned(),
        target_ref: "provider:github:owner/repo:release-draft:42".to_owned(),
        target_label: "Release draft 42 in owner/repo".to_owned(),
        target_fingerprint_ref: "target-fingerprint:provider:release-draft:42:v1".to_owned(),
    }
}

fn local_sandbox() -> AuthoritySandboxBinding {
    AuthoritySandboxBinding {
        sandbox_profile_ref: "sandbox:local-mutation:format:v1".to_owned(),
        sandbox_profile_fingerprint_ref: "sandbox-fingerprint:local-mutation:format:v1".to_owned(),
        capability_envelope_ref: "capability-envelope:local-format:v1".to_owned(),
        policy_epoch_ref: "policy-epoch:enterprise:2026-05-18".to_owned(),
    }
}

fn external_sandbox() -> AuthoritySandboxBinding {
    AuthoritySandboxBinding {
        sandbox_profile_ref: "sandbox:external-provider:publish:v1".to_owned(),
        sandbox_profile_fingerprint_ref: "sandbox-fingerprint:external-provider:publish:v1"
            .to_owned(),
        capability_envelope_ref: "capability-envelope:provider-publish:v1".to_owned(),
        policy_epoch_ref: "policy-epoch:enterprise:2026-05-18".to_owned(),
    }
}

fn seed_remembered_rules() -> Vec<RememberedDecisionRule> {
    vec![remembered_rule(
        "remembered-rule:local-format:0001",
        "Remembered local formatter rule",
        "issuer:shell:desktop",
        AuthorityTicketClass::LocalMutation,
        local_target(),
        AuthorityActorClass::HumanAccount,
        "actor:user:local-01",
        AuthoritySourceClass::HumanAccount,
        "authority-source:local-user-session:01",
        "scope:workspace:aureline:format",
        local_sandbox(),
        600,
        "2026-05-19T10:00:00Z",
        "revoke-path:remembered-rule:format-current-repo",
        vec!["audit:remembered-rule:local-format:registered"],
    )]
}

#[allow(clippy::too_many_arguments)]
fn request(
    request_id: &str,
    display_label: &str,
    requesting_surface_id: &str,
    requesting_surface_class: RequestingSurfaceClass,
    requesting_surface_ref: &str,
    routed_issuer_id: &str,
    requested_ticket_class: AuthorityTicketClass,
    remembered_rule_id: Option<&str>,
    actor_class: AuthorityActorClass,
    actor_subject_ref: &str,
    authority_source_class: AuthoritySourceClass,
    authority_source_ref: &str,
    requested_target_identity: AuthorityTargetIdentity,
    requested_sandbox_binding: AuthoritySandboxBinding,
    claims_self_authorization: bool,
    claims_ambient_privilege: bool,
    root_authority_proof_present: bool,
    requested_at: &str,
    audit_event_refs: Vec<&str>,
) -> IssuerBoundaryRequest {
    IssuerBoundaryRequest {
        record_kind: ISSUER_BOUNDARY_REQUEST_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_AUTHORITY_ISSUER_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_AUTHORITY_ISSUER_SHARED_CONTRACT_REF.to_owned(),
        request_id: request_id.to_owned(),
        display_label: display_label.to_owned(),
        requesting_surface_id: requesting_surface_id.to_owned(),
        requesting_surface_class,
        requesting_surface_class_token: requesting_surface_class.as_str().to_owned(),
        requesting_surface_ref: requesting_surface_ref.to_owned(),
        routed_issuer_id: routed_issuer_id.to_owned(),
        requested_ticket_class,
        requested_ticket_class_token: requested_ticket_class.as_str().to_owned(),
        remembered_rule_id: remembered_rule_id.map(String::from),
        actor_class,
        actor_class_token: actor_class.as_str().to_owned(),
        actor_subject_ref: actor_subject_ref.to_owned(),
        authority_source_class,
        authority_source_class_token: authority_source_class.as_str().to_owned(),
        authority_source_ref: authority_source_ref.to_owned(),
        requested_target_identity,
        requested_sandbox_binding,
        claims_self_authorization,
        claims_ambient_privilege,
        root_authority_proof_present,
        requested_at: requested_at.to_owned(),
        audit_event_refs: audit_event_refs.into_iter().map(String::from).collect(),
    }
}

fn seed_requests() -> Vec<IssuerBoundaryRequest> {
    vec![
        request(
            "request:ai-tool:provider-publish:0001",
            "AI tool plan asks the shell to mint an external-provider mutation ticket",
            "surface:ai-tool:plan",
            RequestingSurfaceClass::AiTool,
            "ai-tool:plan:requesting-surface",
            "issuer:shell:desktop",
            AuthorityTicketClass::ExternalProviderMutation,
            None,
            AuthorityActorClass::HumanAccount,
            "actor:user:local-01",
            AuthoritySourceClass::HumanAccount,
            "authority-source:provider-session:github:01",
            provider_target(),
            external_sandbox(),
            false,
            false,
            false,
            "2026-05-18T10:00:00Z",
            vec!["audit:issuer-boundary:ai-tool:provider-publish:request"],
        ),
        request(
            "request:cli-script:remembered-format:0002",
            "CLI script asks policy service to renew a remembered local-format rule",
            "surface:cli:script",
            RequestingSurfaceClass::CliScript,
            "cli:script:requesting-surface",
            "issuer:policy-service:primary",
            AuthorityTicketClass::LocalMutation,
            Some("remembered-rule:local-format:0001"),
            AuthorityActorClass::HumanAccount,
            "actor:user:local-01",
            AuthoritySourceClass::HumanAccount,
            "authority-source:local-user-session:01",
            local_target(),
            local_sandbox(),
            false,
            false,
            false,
            "2026-05-18T10:00:30Z",
            vec!["audit:issuer-boundary:cli:remembered-format:request"],
        ),
        request(
            "request:extension:self-authorize:0003",
            "Extension attempted to mint a credential projection without an issuer",
            "surface:extension:workspace",
            RequestingSurfaceClass::Extension,
            "extension:workspace:requesting-surface",
            "",
            AuthorityTicketClass::CredentialProjection,
            None,
            AuthorityActorClass::InstallationOrAppGrant,
            "actor:install:extension-runtime",
            AuthoritySourceClass::InstallationGrant,
            "authority-source:installation-grant:extension:registry",
            AuthorityTargetIdentity {
                target_class: AuthorityTargetClass::CredentialConsumer,
                target_class_token: AuthorityTargetClass::CredentialConsumer.as_str().to_owned(),
                target_ref: "consumer:extension:registry-fetch".to_owned(),
                target_label: "Extension registry fetch consumer".to_owned(),
                target_fingerprint_ref: "target-fingerprint:consumer:extension:registry:v1"
                    .to_owned(),
            },
            AuthoritySandboxBinding {
                sandbox_profile_ref: "sandbox:credential-projection:extension:v1".to_owned(),
                sandbox_profile_fingerprint_ref:
                    "sandbox-fingerprint:credential-projection:extension:v1".to_owned(),
                capability_envelope_ref: "capability-envelope:credential-projection:extension:v1"
                    .to_owned(),
                policy_epoch_ref: "policy-epoch:enterprise:2026-05-18".to_owned(),
            },
            true,
            false,
            false,
            "2026-05-18T10:01:00Z",
            vec!["audit:issuer-boundary:extension:self-authorize:request"],
        ),
        request(
            "request:browser-companion:ambient:0004",
            "Browser companion claimed ambient privilege from host session",
            "surface:browser-companion:default",
            RequestingSurfaceClass::BrowserCompanion,
            "browser-companion:requesting-surface",
            "issuer:shell:desktop",
            AuthorityTicketClass::ExternalProviderMutation,
            None,
            AuthorityActorClass::HumanAccount,
            "actor:user:local-01",
            AuthoritySourceClass::HumanAccount,
            "authority-source:provider-session:github:01",
            provider_target(),
            external_sandbox(),
            false,
            true,
            false,
            "2026-05-18T10:01:30Z",
            vec!["audit:issuer-boundary:companion:ambient:request"],
        ),
        request(
            "request:recipe:broaden-remembered:0005",
            "Recipe asked to broaden remembered local-format rule into provider publish",
            "surface:recipe:runner",
            RequestingSurfaceClass::RecipeRunner,
            "recipe:runner:requesting-surface",
            "issuer:shell:desktop",
            AuthorityTicketClass::LocalMutation,
            Some("remembered-rule:local-format:0001"),
            AuthorityActorClass::HumanAccount,
            "actor:user:local-01",
            AuthoritySourceClass::HumanAccount,
            "authority-source:local-user-session:01",
            AuthorityTargetIdentity {
                target_class: AuthorityTargetClass::LocalWorkspace,
                target_class_token: AuthorityTargetClass::LocalWorkspace.as_str().to_owned(),
                target_ref: "workspace:aureline:other-repo".to_owned(),
                target_label: "Aureline other repository".to_owned(),
                target_fingerprint_ref: "target-fingerprint:workspace:aureline:other:v1".to_owned(),
            },
            local_sandbox(),
            false,
            false,
            false,
            "2026-05-18T10:02:00Z",
            vec!["audit:issuer-boundary:recipe:broaden-remembered:request"],
        ),
        request(
            "request:remote-helper:local-only-provider:0006",
            "Remote helper used a local-only authority to reach a provider target",
            "surface:remote-helper:default",
            RequestingSurfaceClass::RemoteHelper,
            "remote-helper:requesting-surface",
            "issuer:supervisor:control-plane",
            AuthorityTicketClass::ExternalProviderMutation,
            None,
            AuthorityActorClass::LocalOnlyAuthority,
            "actor:local-only:tunnel-bridge",
            AuthoritySourceClass::LocalOnlyAuthority,
            "authority-source:local-only:tunnel-bridge",
            provider_target(),
            external_sandbox(),
            false,
            false,
            false,
            "2026-05-18T10:02:30Z",
            vec!["audit:issuer-boundary:remote-helper:local-only:request"],
        ),
        request(
            "request:admin-console:root-rotation:0007",
            "Admin console asked supervisor to mint a trust-root rotation ticket",
            "surface:admin-console:primary",
            RequestingSurfaceClass::AdminConsole,
            "admin-console:requesting-surface",
            "issuer:supervisor:control-plane",
            AuthorityTicketClass::PolicyTrustAdminChange,
            None,
            AuthorityActorClass::OrganizationAdmin,
            "actor:org-admin:security-lead",
            AuthoritySourceClass::HumanAccount,
            "root-source:signed-trust-root-rotation:2026-05-18",
            AuthorityTargetIdentity {
                target_class: AuthorityTargetClass::TrustStore,
                target_class_token: AuthorityTargetClass::TrustStore.as_str().to_owned(),
                target_ref: "trust-root:2026-primary".to_owned(),
                target_label: "Trust root 2026 primary".to_owned(),
                target_fingerprint_ref: "target-fingerprint:trust-root:2026-primary:v1".to_owned(),
            },
            AuthoritySandboxBinding {
                sandbox_profile_ref: "sandbox:root-authority:supervisor:v1".to_owned(),
                sandbox_profile_fingerprint_ref:
                    "sandbox-fingerprint:root-authority:supervisor:v1".to_owned(),
                capability_envelope_ref: "capability-envelope:root-authority:trust-root-rotation:v1"
                    .to_owned(),
                policy_epoch_ref: "policy-epoch:enterprise:2026-05-18".to_owned(),
            },
            false,
            false,
            true,
            "2026-05-18T10:03:00Z",
            vec!["audit:issuer-boundary:admin:root-rotation:request"],
        ),
        request(
            "request:local-admin:debug-attach:0008",
            "Local admin tool asked the shell to mint a privileged debug-attach ticket",
            "surface:local-admin:cli",
            RequestingSurfaceClass::LocalAdminTool,
            "local-admin:cli:requesting-surface",
            "issuer:shell:desktop",
            AuthorityTicketClass::PrivilegedDebugAttach,
            None,
            AuthorityActorClass::LocalAdmin,
            "actor:local-admin:operator-01",
            AuthoritySourceClass::HumanAccount,
            "authority-source:local-admin-session:01",
            AuthorityTargetIdentity {
                target_class: AuthorityTargetClass::DebugAttachTarget,
                target_class_token: AuthorityTargetClass::DebugAttachTarget.as_str().to_owned(),
                target_ref: "debug-attach:workspace:aureline:editor-process:42".to_owned(),
                target_label: "Editor process 42 in current workspace".to_owned(),
                target_fingerprint_ref:
                    "target-fingerprint:debug-attach:editor-process:42:v1".to_owned(),
            },
            AuthoritySandboxBinding {
                sandbox_profile_ref: "sandbox:privileged-debug-attach:editor-process:v1".to_owned(),
                sandbox_profile_fingerprint_ref:
                    "sandbox-fingerprint:privileged-debug-attach:editor-process:v1".to_owned(),
                capability_envelope_ref: "capability-envelope:privileged-debug-attach:editor-process:v1"
                    .to_owned(),
                policy_epoch_ref: "policy-epoch:enterprise:2026-05-18".to_owned(),
            },
            false,
            false,
            false,
            "2026-05-18T10:04:00Z",
            vec!["audit:issuer-boundary:local-admin:debug-attach:request"],
        ),
        request(
            "request:admin-console:credential-projection:0009",
            "Admin console asked supervisor to project a session-only handle to a registry consumer",
            "surface:admin-console:primary",
            RequestingSurfaceClass::AdminConsole,
            "admin-console:requesting-surface",
            "issuer:supervisor:control-plane",
            AuthorityTicketClass::CredentialProjection,
            None,
            AuthorityActorClass::OrganizationAdmin,
            "actor:org-admin:registry-publisher",
            AuthoritySourceClass::HumanAccount,
            "authority-source:org-admin-session:registry-publisher",
            AuthorityTargetIdentity {
                target_class: AuthorityTargetClass::CredentialConsumer,
                target_class_token: AuthorityTargetClass::CredentialConsumer.as_str().to_owned(),
                target_ref: "consumer:registry:publish-session".to_owned(),
                target_label: "Registry publish-session consumer".to_owned(),
                target_fingerprint_ref:
                    "target-fingerprint:consumer:registry:publish-session:v1".to_owned(),
            },
            AuthoritySandboxBinding {
                sandbox_profile_ref: "sandbox:credential-projection:registry:v1".to_owned(),
                sandbox_profile_fingerprint_ref:
                    "sandbox-fingerprint:credential-projection:registry:v1".to_owned(),
                capability_envelope_ref:
                    "capability-envelope:credential-projection:registry-publish:v1".to_owned(),
                policy_epoch_ref: "policy-epoch:enterprise:2026-05-18".to_owned(),
            },
            false,
            false,
            false,
            "2026-05-18T10:05:00Z",
            vec!["audit:issuer-boundary:admin:credential-projection:request"],
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn decision(
    decision_id: &str,
    request_id: &str,
    decision_class: IssuerBoundaryDecisionClass,
    minted_authority_ticket_ref: Option<&str>,
    renewed_from_rule_id: Option<&str>,
    rejection_reasons: Vec<IssuerBoundaryRejectionReason>,
    actor_class: AuthorityActorClass,
    authority_source_class: AuthoritySourceClass,
    issuer_class: AuthorityIssuerClass,
    explanation: &str,
    local_editing_preserved: bool,
    reprompt_required: bool,
    decided_at: &str,
    audit_event_refs: Vec<&str>,
) -> IssuerBoundaryDecision {
    let rejection_reason_tokens = rejection_reasons
        .iter()
        .map(|reason| reason.as_str().to_owned())
        .collect();
    IssuerBoundaryDecision {
        record_kind: ISSUER_BOUNDARY_DECISION_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_AUTHORITY_ISSUER_SCHEMA_VERSION,
        shared_contract_ref: RUNTIME_AUTHORITY_ISSUER_SHARED_CONTRACT_REF.to_owned(),
        decision_id: decision_id.to_owned(),
        request_id: request_id.to_owned(),
        decision_class,
        decision_class_token: decision_class.as_str().to_owned(),
        minted_authority_ticket_ref: minted_authority_ticket_ref.map(String::from),
        renewed_from_rule_id: renewed_from_rule_id.map(String::from),
        rejection_reasons,
        rejection_reason_tokens,
        actor_class,
        actor_class_token: actor_class.as_str().to_owned(),
        authority_source_class,
        authority_source_class_token: authority_source_class.as_str().to_owned(),
        issuer_class,
        issuer_class_token: issuer_class.as_str().to_owned(),
        explanation: explanation.to_owned(),
        local_editing_preserved,
        reprompt_required,
        decided_at: decided_at.to_owned(),
        audit_event_refs: audit_event_refs.into_iter().map(String::from).collect(),
    }
}

fn seed_decisions() -> Vec<IssuerBoundaryDecision> {
    vec![
        decision(
            "decision:ai-tool:provider-publish:granted:0001",
            "request:ai-tool:provider-publish:0001",
            IssuerBoundaryDecisionClass::Granted,
            Some("authority-ticket:external:provider-publish:0001"),
            None,
            Vec::new(),
            AuthorityActorClass::HumanAccount,
            AuthoritySourceClass::HumanAccount,
            AuthorityIssuerClass::Shell,
            "Shell minted a single-use external-provider mutation ticket after the user approved the AI plan; ticket is bound to actor, target, sandbox, and policy epoch.",
            true,
            false,
            "2026-05-18T10:00:05Z",
            vec!["audit:issuer-boundary:ai-tool:provider-publish:granted"],
        ),
        decision(
            "decision:cli-script:remembered-format:renewed:0002",
            "request:cli-script:remembered-format:0002",
            IssuerBoundaryDecisionClass::RememberedDecisionNarrowed,
            Some("authority-ticket:local:remembered-format:0002"),
            Some("remembered-rule:local-format:0001"),
            Vec::new(),
            AuthorityActorClass::HumanAccount,
            AuthoritySourceClass::HumanAccount,
            AuthorityIssuerClass::PolicyService,
            "Policy service narrowed the remembered local-format rule into a fresh ten-minute ticket scoped to the current repo and policy epoch.",
            true,
            false,
            "2026-05-18T10:00:35Z",
            vec!["audit:issuer-boundary:cli:remembered-format:narrowed"],
        ),
        decision(
            "decision:extension:self-authorize:refused:0003",
            "request:extension:self-authorize:0003",
            IssuerBoundaryDecisionClass::Refused,
            None,
            None,
            vec![
                IssuerBoundaryRejectionReason::SelfAuthorizationByNonIssuer,
                IssuerBoundaryRejectionReason::MissingIssuerBinding,
            ],
            AuthorityActorClass::InstallationOrAppGrant,
            AuthoritySourceClass::InstallationGrant,
            AuthorityIssuerClass::PolicyService,
            "Extension attempted to mint a credential projection on its own; refused and local editing preserved. Reroute the request through the shell or policy service to reprompt.",
            true,
            true,
            "2026-05-18T10:01:05Z",
            vec!["audit:issuer-boundary:extension:self-authorize:refused"],
        ),
        decision(
            "decision:browser-companion:ambient:refused:0004",
            "request:browser-companion:ambient:0004",
            IssuerBoundaryDecisionClass::Refused,
            None,
            None,
            vec![IssuerBoundaryRejectionReason::AmbientPrivilegeInferred],
            AuthorityActorClass::HumanAccount,
            AuthoritySourceClass::HumanAccount,
            AuthorityIssuerClass::Shell,
            "Browser companion inferred ambient privilege from its host session; refused. The shell must reprompt the user before publishing.",
            true,
            true,
            "2026-05-18T10:01:35Z",
            vec!["audit:issuer-boundary:companion:ambient:refused"],
        ),
        decision(
            "decision:recipe:broaden-remembered:refused:0005",
            "request:recipe:broaden-remembered:0005",
            IssuerBoundaryDecisionClass::Refused,
            None,
            None,
            vec![
                IssuerBoundaryRejectionReason::RememberedDecisionTooBroad,
                IssuerBoundaryRejectionReason::RememberedDecisionTargetDrift,
            ],
            AuthorityActorClass::HumanAccount,
            AuthoritySourceClass::HumanAccount,
            AuthorityIssuerClass::Shell,
            "Recipe asked to renew the remembered local-format rule against a different repository; refused because remembered decisions stay narrow to their original target. Reprompt the user to approve the other repository.",
            true,
            true,
            "2026-05-18T10:02:05Z",
            vec!["audit:issuer-boundary:recipe:broaden-remembered:refused"],
        ),
        decision(
            "decision:remote-helper:local-only-provider:refused:0006",
            "request:remote-helper:local-only-provider:0006",
            IssuerBoundaryDecisionClass::Refused,
            None,
            None,
            vec![
                IssuerBoundaryRejectionReason::AuthoritySourceMismatch,
                IssuerBoundaryRejectionReason::AuthoritySourceUnreachableTarget,
            ],
            AuthorityActorClass::LocalOnlyAuthority,
            AuthoritySourceClass::LocalOnlyAuthority,
            AuthorityIssuerClass::Supervisor,
            "Remote helper used a local-only authority to reach a provider object; refused because local-only authority cannot reach provider-managed targets. Reroute through a human-account or installation-grant authority.",
            true,
            true,
            "2026-05-18T10:02:35Z",
            vec!["audit:issuer-boundary:remote-helper:local-only:refused"],
        ),
        decision(
            "decision:admin-console:root-rotation:granted:0007",
            "request:admin-console:root-rotation:0007",
            IssuerBoundaryDecisionClass::Granted,
            Some("authority-ticket:admin:trust-root-rotation:0001"),
            None,
            Vec::new(),
            AuthorityActorClass::OrganizationAdmin,
            AuthoritySourceClass::HumanAccount,
            AuthorityIssuerClass::Supervisor,
            "Supervisor minted a trust-root rotation ticket after verifying the signed root-authority proof and step-up admin actor.",
            true,
            false,
            "2026-05-18T10:03:05Z",
            vec!["audit:issuer-boundary:admin:root-rotation:granted"],
        ),
        decision(
            "decision:local-admin:debug-attach:granted:0008",
            "request:local-admin:debug-attach:0008",
            IssuerBoundaryDecisionClass::Granted,
            Some("authority-ticket:privileged-debug-attach:editor-process:0001"),
            None,
            Vec::new(),
            AuthorityActorClass::LocalAdmin,
            AuthoritySourceClass::HumanAccount,
            AuthorityIssuerClass::Shell,
            "Shell minted a single-use privileged debug-attach ticket after the local admin stepped up at the shell prompt; the ticket is bound to the editor process target, sandbox profile, and policy epoch.",
            true,
            false,
            "2026-05-18T10:04:05Z",
            vec!["audit:issuer-boundary:local-admin:debug-attach:granted"],
        ),
        decision(
            "decision:admin-console:credential-projection:granted:0009",
            "request:admin-console:credential-projection:0009",
            IssuerBoundaryDecisionClass::Granted,
            Some("authority-ticket:credential-projection:registry-publish:0001"),
            None,
            Vec::new(),
            AuthorityActorClass::OrganizationAdmin,
            AuthoritySourceClass::HumanAccount,
            AuthorityIssuerClass::Supervisor,
            "Supervisor minted a session-only credential projection ticket scoped to the registry publish consumer; the broker exchanges a handle and no raw secret material is projected through the requesting surface.",
            true,
            false,
            "2026-05-18T10:05:05Z",
            vec!["audit:issuer-boundary:admin:credential-projection:granted"],
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates() {
        let page = seeded_runtime_authority_issuer_page();
        validate_runtime_authority_issuer_page(&page)
            .expect("seeded runtime-authority-issuer page validates");
        assert!(page.defects.is_empty(), "defects: {:?}", page.defects);
        for surface_class in RequestingSurfaceClass::ALL {
            assert!(page
                .summary
                .requesting_surface_classes_present
                .iter()
                .any(|token| token == surface_class.as_str()));
        }
        for reason in [
            IssuerBoundaryRejectionReason::SelfAuthorizationByNonIssuer,
            IssuerBoundaryRejectionReason::AmbientPrivilegeInferred,
            IssuerBoundaryRejectionReason::RememberedDecisionTooBroad,
            IssuerBoundaryRejectionReason::AuthoritySourceMismatch,
        ] {
            assert!(
                page.summary
                    .rejection_reasons_present
                    .iter()
                    .any(|token| token == reason.as_str()),
                "missing rejection reason coverage: {}",
                reason.as_str()
            );
        }
    }

    #[test]
    fn lineage_packet_preserves_rejection_reasons() {
        let page = seeded_runtime_authority_issuer_page();
        let packet = RuntimeAuthorityLineagePacket::from_page(
            "runtime-authority-lineage-packet:test",
            "Runtime authority lineage packet test",
            "2026-05-18T10:30:00Z",
            &page,
        );
        assert!(packet.raw_credentials_excluded);
        assert!(packet.provider_versus_local_distinguished);
        assert_eq!(packet.lineage_rows.len(), page.decisions.len());
        for token in [
            IssuerBoundaryRejectionReason::SelfAuthorizationByNonIssuer.as_str(),
            IssuerBoundaryRejectionReason::AmbientPrivilegeInferred.as_str(),
            IssuerBoundaryRejectionReason::RememberedDecisionTooBroad.as_str(),
            IssuerBoundaryRejectionReason::AuthoritySourceMismatch.as_str(),
        ] {
            assert!(packet.rejection_reason_counts.contains_key(token));
        }
    }

    #[test]
    fn validator_flags_admitted_self_authorization() {
        let mut page = seeded_runtime_authority_issuer_page();
        let decision = page
            .decisions
            .iter_mut()
            .find(|decision| {
                decision
                    .rejection_reasons
                    .contains(&IssuerBoundaryRejectionReason::SelfAuthorizationByNonIssuer)
            })
            .expect("self-authorization refused decision");
        decision.decision_class = IssuerBoundaryDecisionClass::Granted;
        decision.decision_class_token = IssuerBoundaryDecisionClass::Granted.as_str().to_owned();
        decision.rejection_reasons.clear();
        decision.rejection_reason_tokens.clear();
        decision.minted_authority_ticket_ref =
            Some("authority-ticket:credential:invalid:0099".to_owned());
        let defects = audit_runtime_authority_issuer_page(
            &page.issuers,
            &page.requesting_surfaces,
            &page.remembered_rules,
            &page.requests,
            &page.decisions,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RuntimeAuthorityIssuerDefectKind::AdmittedSelfAuthorization));
    }

    #[test]
    fn validator_flags_broad_remembered_rule() {
        let mut page = seeded_runtime_authority_issuer_page();
        page.remembered_rules[0].scope_ref.clear();
        let defects = audit_runtime_authority_issuer_page(
            &page.issuers,
            &page.requesting_surfaces,
            &page.remembered_rules,
            &page.requests,
            &page.decisions,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RuntimeAuthorityIssuerDefectKind::RememberedRuleNotNarrow));
    }

    #[test]
    fn validator_flags_remembered_rule_for_forbidden_class() {
        let mut page = seeded_runtime_authority_issuer_page();
        page.remembered_rules[0].ticket_class = AuthorityTicketClass::CredentialProjection;
        page.remembered_rules[0].ticket_class_token = AuthorityTicketClass::CredentialProjection
            .as_str()
            .to_owned();
        let defects = audit_runtime_authority_issuer_page(
            &page.issuers,
            &page.requesting_surfaces,
            &page.remembered_rules,
            &page.requests,
            &page.decisions,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RuntimeAuthorityIssuerDefectKind::RememberedRuleForbiddenClass));
    }

    #[test]
    fn validator_flags_shell_root_authority_overreach() {
        let mut page = seeded_runtime_authority_issuer_page();
        let shell = page
            .issuers
            .iter_mut()
            .find(|issuer| issuer.issuer_class == AuthorityIssuerClass::Shell)
            .expect("shell issuer");
        shell.may_mint_root_authority_changes = true;
        shell
            .mintable_ticket_classes
            .push(AuthorityTicketClass::PolicyTrustAdminChange);
        shell.mintable_ticket_class_tokens.push(
            AuthorityTicketClass::PolicyTrustAdminChange
                .as_str()
                .to_owned(),
        );
        let defects = audit_runtime_authority_issuer_page(
            &page.issuers,
            &page.requesting_surfaces,
            &page.remembered_rules,
            &page.requests,
            &page.decisions,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RuntimeAuthorityIssuerDefectKind::UnauthorizedRootAuthorityClaim));
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind == RuntimeAuthorityIssuerDefectKind::IssuerOverreach));
    }

    #[test]
    fn validator_flags_refused_without_reason() {
        let mut page = seeded_runtime_authority_issuer_page();
        let decision = page
            .decisions
            .iter_mut()
            .find(|decision| decision.decision_class == IssuerBoundaryDecisionClass::Refused)
            .expect("refused decision");
        decision.rejection_reasons.clear();
        decision.rejection_reason_tokens.clear();
        let defects = audit_runtime_authority_issuer_page(
            &page.issuers,
            &page.requesting_surfaces,
            &page.remembered_rules,
            &page.requests,
            &page.decisions,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RuntimeAuthorityIssuerDefectKind::RefusedDecisionMissingReason));
    }

    #[test]
    fn validator_flags_local_only_admitted_to_provider() {
        let mut page = seeded_runtime_authority_issuer_page();
        // Flip the local-only refusal to granted; the validator should mark it
        // as an admitted authority-source mismatch.
        let decision = page
            .decisions
            .iter_mut()
            .find(|decision| {
                decision
                    .rejection_reasons
                    .contains(&IssuerBoundaryRejectionReason::AuthoritySourceUnreachableTarget)
            })
            .expect("local-only refusal");
        decision.decision_class = IssuerBoundaryDecisionClass::Granted;
        decision.decision_class_token = IssuerBoundaryDecisionClass::Granted.as_str().to_owned();
        decision.rejection_reasons.clear();
        decision.rejection_reason_tokens.clear();
        decision.minted_authority_ticket_ref =
            Some("authority-ticket:external:invalid:0099".to_owned());
        let defects = audit_runtime_authority_issuer_page(
            &page.issuers,
            &page.requesting_surfaces,
            &page.remembered_rules,
            &page.requests,
            &page.decisions,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == RuntimeAuthorityIssuerDefectKind::DecisionAdmittedOnSourceMismatch));
    }
}
