//! Approval-ticket alpha records for provider and helper-backed mutations.
//!
//! This module consumes the connected-provider registry and the existing
//! runtime/integration approval-ticket schemas by reference. It validates that
//! high-risk external mutations cite a short-lived ticket or reviewed-scope
//! object, that the authority object is bound to the current actor, target,
//! sandbox, trust profile, and policy epoch, and that denied spend attempts
//! route back to native reapproval rather than replaying stale authority.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::registry::{FindingSeverity, ProviderFixtureMetadata, RedactionClass, TargetRef};

/// Schema version for the approval-ticket alpha packet.
pub const APPROVAL_TICKET_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ApprovalTicketAlphaPacket`].
pub const APPROVAL_TICKET_ALPHA_PACKET_RECORD_KIND: &str = "approval_ticket_alpha_packet";

/// Stable record-kind tag for [`ApprovalTicketAlphaRecord`].
pub const APPROVAL_TICKET_ALPHA_RECORD_KIND: &str = "approval_ticket_alpha_record";

/// Stable record-kind tag for [`ReviewedScopeAlphaRecord`].
pub const REVIEWED_SCOPE_ALPHA_RECORD_KIND: &str = "reviewed_scope_alpha_record";

/// Stable record-kind tag for [`MutationAuthorityBinding`].
pub const MUTATION_AUTHORITY_BINDING_RECORD_KIND: &str = "mutation_authority_binding_record";

/// Stable record-kind tag for [`ApprovalTicketSpendAttempt`].
pub const APPROVAL_TICKET_SPEND_ATTEMPT_RECORD_KIND: &str = "approval_ticket_spend_attempt_record";

/// Stable record-kind tag for [`ApprovalTicketAlphaValidationReport`].
pub const APPROVAL_TICKET_ALPHA_VALIDATION_REPORT_RECORD_KIND: &str =
    "approval_ticket_alpha_validation_report";

/// Stable record-kind tag for [`ApprovalTicketSupportAdminPacket`].
pub const APPROVAL_TICKET_SUPPORT_ADMIN_PACKET_RECORD_KIND: &str =
    "approval_ticket_support_admin_packet";

/// Packet containing approval tickets, reviewed scopes, mutation bindings, and spend attempts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalTicketAlphaPacket {
    /// Optional fixture metadata for protected validation paths.
    #[serde(default, rename = "__fixture__")]
    pub fixture_metadata: Option<ProviderFixtureMetadata>,
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this packet.
    pub approval_ticket_alpha_schema_version: u32,
    /// Opaque packet id.
    pub packet_id: String,
    /// Upstream contracts this packet consumes by reference.
    pub contract_refs: ApprovalTicketContractRefs,
    /// Ticket records minted for high-risk provider mutations.
    #[serde(default)]
    pub approval_tickets: Vec<ApprovalTicketAlphaRecord>,
    /// Reviewed-scope objects that can admit bounded helper-backed mutations.
    #[serde(default)]
    pub reviewed_scopes: Vec<ReviewedScopeAlphaRecord>,
    /// High-risk mutation requests bound to a ticket or reviewed scope.
    #[serde(default)]
    pub mutation_bindings: Vec<MutationAuthorityBinding>,
    /// Spend attempts that prove admitted and fail-closed outcomes.
    #[serde(default)]
    pub spend_attempts: Vec<ApprovalTicketSpendAttempt>,
}

impl ApprovalTicketAlphaPacket {
    /// Validates the approval-ticket alpha packet and returns a redaction-safe report.
    pub fn validate(&self) -> ApprovalTicketAlphaValidationReport {
        let mut validator = ApprovalTicketAlphaValidator::new(self);
        validator.validate();
        validator.finish()
    }

    /// Builds the support/admin packet that reconstructs authority lineage without secrets.
    pub fn support_admin_projection(&self) -> ApprovalTicketSupportAdminPacket {
        let mut lineage_summaries: Vec<ApprovalTicketLineageSummary> = self
            .approval_tickets
            .iter()
            .map(|ticket| ApprovalTicketLineageSummary {
                authority_ref: ticket.approval_ticket_id.clone(),
                authority_kind: ApprovalAuthorityKind::ApprovalTicket,
                actor_class: ticket.actor_scope.actor_class,
                actor_subject_ref: ticket.actor_scope.actor_subject_ref.clone(),
                auth_source_class: ticket.actor_scope.auth_source_class,
                target_identity: ticket.target_identity.clone(),
                side_effect_class: ticket.side_effect_class,
                sandbox_profile_ref: ticket.sandbox_profile_ref.clone(),
                trust_profile_ref: ticket.trust_profile_ref.clone(),
                policy_epoch_ref: ticket.policy_epoch_ref.clone(),
                expires_at: ticket.expires_at.clone(),
                evidence_refs: ticket.evidence_refs.clone(),
                rollback_refs: ticket.rollback_refs.clone(),
                actor_lineage: ticket.actor_lineage.clone(),
                redaction_class: ticket.redaction_class,
            })
            .collect();

        lineage_summaries.extend(self.reviewed_scopes.iter().map(|scope| {
            ApprovalTicketLineageSummary {
                authority_ref: scope.reviewed_scope_id.clone(),
                authority_kind: ApprovalAuthorityKind::ReviewedScope,
                actor_class: scope.actor_scope.actor_class,
                actor_subject_ref: scope.actor_scope.actor_subject_ref.clone(),
                auth_source_class: scope.actor_scope.auth_source_class,
                target_identity: scope.target_identity.clone(),
                side_effect_class: scope.side_effect_class,
                sandbox_profile_ref: scope.sandbox_profile_ref.clone(),
                trust_profile_ref: scope.trust_profile_ref.clone(),
                policy_epoch_ref: scope.policy_epoch_ref.clone(),
                expires_at: scope.expires_at.clone(),
                evidence_refs: scope.evidence_refs.clone(),
                rollback_refs: scope.rollback_refs.clone(),
                actor_lineage: scope.actor_lineage.clone(),
                redaction_class: scope.redaction_class,
            }
        }));

        let mutation_summaries = self
            .mutation_bindings
            .iter()
            .map(|binding| {
                let (authority_ref, authority_kind) =
                    if let Some(ticket_ref) = &binding.approval_ticket_ref {
                        (
                            Some(ticket_ref.clone()),
                            Some(ApprovalAuthorityKind::ApprovalTicket),
                        )
                    } else if let Some(scope_ref) = &binding.reviewed_scope_ref {
                        (
                            Some(scope_ref.clone()),
                            Some(ApprovalAuthorityKind::ReviewedScope),
                        )
                    } else {
                        (None, None)
                    };

                MutationAuthoritySummary {
                    mutation_id: binding.mutation_id.clone(),
                    high_risk_action_class: binding.high_risk_action_class,
                    authority_requirement: binding.authority_requirement,
                    authority_ref,
                    authority_kind,
                    actor_class: binding.actor_scope.actor_class,
                    auth_source_class: binding.actor_scope.auth_source_class,
                    target_identity: binding.target_identity.clone(),
                    side_effect_class: binding.side_effect_class,
                    support_summary: binding.support_summary.clone(),
                }
            })
            .collect();

        let spend_summaries = self
            .spend_attempts
            .iter()
            .map(|attempt| ApprovalTicketSpendSummary {
                spend_attempt_id: attempt.spend_attempt_id.clone(),
                mutation_ref: attempt.mutation_ref.clone(),
                presented_approval_ticket_ref: attempt.presented_approval_ticket_ref.clone(),
                presented_reviewed_scope_ref: attempt.presented_reviewed_scope_ref.clone(),
                evaluation_outcome: attempt.evaluation_outcome,
                native_reapproval_route: attempt.native_reapproval_route,
                explanation: attempt.explanation.clone(),
                audit_event_refs: attempt.audit_event_refs.clone(),
            })
            .collect();

        ApprovalTicketSupportAdminPacket {
            record_kind: APPROVAL_TICKET_SUPPORT_ADMIN_PACKET_RECORD_KIND.to_string(),
            schema_version: APPROVAL_TICKET_ALPHA_SCHEMA_VERSION,
            packet_id: self.packet_id.clone(),
            lineage_summaries,
            mutation_summaries,
            spend_summaries,
            redaction_class: RedactionClass::MetadataSafeDefault,
        }
    }
}

/// References to upstream schemas and packets consumed by the approval-ticket alpha packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalTicketContractRefs {
    /// Connected-provider registry schema reference.
    pub connected_provider_registry_schema_ref: String,
    /// Runtime approval-ticket schema reference.
    pub runtime_approval_ticket_schema_ref: String,
    /// Provider-plane approval-ticket schema reference.
    pub provider_plane_approval_ticket_schema_ref: String,
    /// Security audit-stream schema reference.
    pub audit_stream_schema_ref: String,
    /// Connected-provider packet this approval packet binds to.
    pub provider_registry_packet_ref: String,
}

impl ApprovalTicketContractRefs {
    fn all_refs(&self) -> [&str; 5] {
        [
            &self.connected_provider_registry_schema_ref,
            &self.runtime_approval_ticket_schema_ref,
            &self.provider_plane_approval_ticket_schema_ref,
            &self.audit_stream_schema_ref,
            &self.provider_registry_packet_ref,
        ]
    }
}

/// Ticket record that binds actor, target, side effect, sandbox, policy epoch, and expiry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalTicketAlphaRecord {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this ticket.
    pub approval_ticket_alpha_schema_version: u32,
    /// Opaque approval-ticket id.
    pub approval_ticket_id: String,
    /// Issuer allowed to mint this ticket.
    pub issuer_class: ApprovalIssuerClass,
    /// Request origin that asked for approval.
    pub request_origin_class: ApprovalRequestOriginClass,
    /// Requesting surface ref when the origin is not the issuer itself.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requesting_surface_ref: Option<String>,
    /// Descriptor this ticket is associated with when the mutation is provider-backed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_descriptor_ref: Option<String>,
    /// Connected provider record this ticket is associated with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connected_provider_record_ref: Option<String>,
    /// Actor and authority scope the ticket is spent as.
    pub actor_scope: ApprovalActorScope,
    /// Target identity the ticket is bound to.
    pub target_identity: ApprovalTargetIdentity,
    /// Side-effect class admitted by the ticket.
    pub side_effect_class: ApprovalSideEffectClass,
    /// Sandbox or capability profile the mutation must run under.
    pub sandbox_profile_ref: String,
    /// Trust profile at issue time.
    pub trust_profile_ref: String,
    /// Policy epoch at issue time.
    pub policy_epoch_ref: String,
    /// Time when the ticket was issued.
    pub issued_at: String,
    /// Time after which the ticket is not spendable.
    pub expires_at: String,
    /// Use posture for the ticket.
    pub use_posture: ApprovalTicketUsePosture,
    /// Runtime approval-ticket ref consumed by this alpha record.
    pub runtime_approval_ticket_ref: String,
    /// Provider-plane approval-ticket ref consumed by this alpha record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_plane_approval_ticket_ref: Option<String>,
    /// Export-safe actor lineage for support/admin reconstruction.
    pub actor_lineage: Vec<ApprovalActorLineageEntry>,
    /// Evidence refs that justify or explain the ticket.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Rollback refs associated with the admitted side effect.
    #[serde(default)]
    pub rollback_refs: Vec<String>,
    /// True only when raw provider payload refs appear on the record.
    pub raw_payload_refs_present: bool,
    /// Redaction posture for the ticket.
    pub redaction_class: RedactionClass,
}

/// Reviewed-scope object that can admit a bounded high-risk mutation without replaying ambient authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewedScopeAlphaRecord {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this reviewed scope.
    pub approval_ticket_alpha_schema_version: u32,
    /// Opaque reviewed-scope id.
    pub reviewed_scope_id: String,
    /// Actor and authority scope reviewed for this object.
    pub actor_scope: ApprovalActorScope,
    /// Target identity reviewed for this object.
    pub target_identity: ApprovalTargetIdentity,
    /// Side-effect class admitted by the reviewed scope.
    pub side_effect_class: ApprovalSideEffectClass,
    /// Sandbox or capability profile the mutation must run under.
    pub sandbox_profile_ref: String,
    /// Trust profile at review time.
    pub trust_profile_ref: String,
    /// Policy epoch at review time.
    pub policy_epoch_ref: String,
    /// Time when the scope was reviewed.
    pub reviewed_at: String,
    /// Time after which the reviewed scope is not spendable.
    pub expires_at: String,
    /// Export-safe actor lineage for support/admin reconstruction.
    pub actor_lineage: Vec<ApprovalActorLineageEntry>,
    /// Evidence refs that justify or explain the reviewed scope.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Rollback refs associated with the reviewed side effect.
    #[serde(default)]
    pub rollback_refs: Vec<String>,
    /// True only when raw provider payload refs appear on the record.
    pub raw_payload_refs_present: bool,
    /// Redaction posture for the reviewed scope.
    pub redaction_class: RedactionClass,
}

/// Binding from a high-risk mutation request to its required authority object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationAuthorityBinding {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this binding.
    pub approval_ticket_alpha_schema_version: u32,
    /// Opaque mutation id.
    pub mutation_id: String,
    /// Surface or command that requested the mutation.
    pub mutation_surface_ref: String,
    /// Descriptor this mutation belongs to when provider-backed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_descriptor_ref: Option<String>,
    /// High-risk action class.
    pub high_risk_action_class: HighRiskActionClass,
    /// Side-effect class requested by the mutation.
    pub side_effect_class: ApprovalSideEffectClass,
    /// Actor and authority scope requested by the mutation.
    pub actor_scope: ApprovalActorScope,
    /// Target identity requested by the mutation.
    pub target_identity: ApprovalTargetIdentity,
    /// Sandbox or capability profile requested by the mutation.
    pub sandbox_profile_ref: String,
    /// Trust profile required by the mutation.
    pub trust_profile_ref: String,
    /// Policy epoch required by the mutation.
    pub policy_epoch_ref: String,
    /// Authority object requirement for this mutation.
    pub authority_requirement: MutationAuthorityRequirement,
    /// Approval ticket admitted for this mutation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ticket_ref: Option<String>,
    /// Reviewed scope admitted for this mutation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewed_scope_ref: Option<String>,
    /// Audit event refs emitted or expected for this binding.
    pub audit_event_refs: Vec<String>,
    /// Redaction-safe summary shown in support/admin packets.
    pub support_summary: String,
}

/// Spend-attempt record that evaluates current context against a ticket or reviewed scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalTicketSpendAttempt {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this spend attempt.
    pub approval_ticket_alpha_schema_version: u32,
    /// Opaque spend-attempt id.
    pub spend_attempt_id: String,
    /// Mutation binding this spend attempt evaluates.
    pub mutation_ref: String,
    /// Approval ticket presented at spend time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presented_approval_ticket_ref: Option<String>,
    /// Reviewed scope presented at spend time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presented_reviewed_scope_ref: Option<String>,
    /// Actor and authority scope observed at spend time.
    pub current_actor_scope: ApprovalActorScope,
    /// Target identity observed at spend time.
    pub current_target_identity: ApprovalTargetIdentity,
    /// Sandbox or capability profile observed at spend time.
    pub current_sandbox_profile_ref: String,
    /// Trust profile observed at spend time.
    pub current_trust_profile_ref: String,
    /// Policy epoch observed at spend time.
    pub current_policy_epoch_ref: String,
    /// Time when the spend attempt was evaluated.
    pub evaluated_at: String,
    /// Outcome claimed by the spend attempt.
    pub evaluation_outcome: TicketEvaluationOutcome,
    /// Native reapproval route used after a denied or drifted spend.
    pub native_reapproval_route: NativeReapprovalRoute,
    /// Clear redaction-safe explanation for the outcome.
    pub explanation: String,
    /// Audit event refs emitted for the spend attempt.
    pub audit_event_refs: Vec<String>,
}

/// Actor scope admitted by approval-ticket alpha records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalActorScope {
    /// Actor class Aureline acts as for the mutation.
    pub actor_class: ApprovalActorClass,
    /// Opaque actor subject reference.
    pub actor_subject_ref: String,
    /// Scope refs granted to the actor.
    #[serde(default)]
    pub granted_scope_refs: Vec<String>,
    /// Auth source behind the actor class.
    pub auth_source_class: ApprovalAuthSourceClass,
}

/// Export-safe actor-lineage entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalActorLineageEntry {
    /// Opaque lineage ref.
    pub lineage_ref: String,
    /// Actor class represented by this lineage hop.
    pub actor_class: ApprovalActorClass,
    /// Opaque actor subject reference.
    pub actor_subject_ref: String,
    /// Auth source represented by this lineage hop.
    pub auth_source_class: ApprovalAuthSourceClass,
    /// Opaque authority source reference.
    pub authority_source_ref: String,
}

/// Target identity captured by an authority object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalTargetIdentity {
    /// Target class for comparison and support grouping.
    pub target_class: ApprovalTargetClass,
    /// Provider, helper, or local target ref.
    pub target_ref: TargetRef,
    /// Opaque target fingerprint at approval time.
    pub target_fingerprint_ref: String,
    /// Opaque target version or revision at approval time.
    pub target_version_ref: String,
}

/// Actor classes visible in approval-ticket lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalActorClass {
    /// Human provider account.
    HumanAccount,
    /// Installation, app, or bot grant.
    InstallationOrAppGrant,
    /// Delegated user credential.
    DelegatedCredential,
    /// Project-scoped provider grant.
    ProjectScopedGrant,
    /// Policy-injected service identity.
    PolicyInjectedServiceIdentity,
    /// Local-only authority that never crosses a provider boundary.
    LocalOnlyAuthority,
    /// Unknown actor class requiring repair.
    UnknownActorClass,
}

/// Auth sources visible in approval-ticket lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalAuthSourceClass {
    /// Signed-in human session.
    HumanSession,
    /// Provider installation grant.
    InstallationGrant,
    /// Delegated credential.
    DelegatedCredential,
    /// Project-scoped grant.
    ProjectScopedGrant,
    /// Policy-injected service identity.
    PolicyInjectedService,
    /// Local-only authority source.
    LocalOnly,
    /// Browser-only authentication source.
    BrowserOnly,
    /// Unknown auth source requiring repair.
    UnknownAuthSource,
}

/// Issuer classes allowed to mint approval tickets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalIssuerClass {
    /// Product shell issuer.
    Shell,
    /// Policy service issuer.
    PolicyService,
    /// Supervisor issuer.
    Supervisor,
}

/// Request-origin classes allowed to request approval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalRequestOriginClass {
    /// Approval was requested by the user-facing shell prompt.
    UserShellPrompt,
    /// Approval was requested by a policy decision.
    PolicyDecision,
    /// Approval was requested by a supervisor control path.
    SupervisorControlPath,
    /// Approval was requested by an AI tool plan.
    AiToolPlan,
    /// Approval was requested by an extension.
    ExtensionRequest,
    /// Approval was requested by a CLI script.
    CliScriptRequest,
    /// Approval was requested by a browser companion.
    BrowserCompanionRequest,
    /// Approval was requested by a remote helper.
    RemoteHelperRequest,
    /// Approval was requested by an automation scheduler.
    AutomationSchedulerRequest,
}

/// Target classes carried by authority objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalTargetClass {
    /// Provider-side object such as a pull request, issue, or CI run.
    ProviderObject,
    /// Remote helper-backed target.
    RemoteHelperTarget,
    /// Local-only target.
    LocalOnlyTarget,
}

/// Side effects admitted by approval-ticket alpha records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalSideEffectClass {
    /// Publish or update a provider review comment.
    ProviderReviewCommentPublish,
    /// Update an issue or work item through a provider.
    ProviderIssueUpdate,
    /// Rerun or retry a provider CI/check run.
    ProviderCiRerun,
    /// Mutate a bounded remote-helper target.
    RemoteHelperMutation,
}

/// Use posture for short-lived authority objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalTicketUsePosture {
    /// Ticket may be spent once.
    SingleUse,
    /// Ticket may be spent within a bounded counter and window.
    BoundedReuse,
}

/// High-risk mutation classes that must cite authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HighRiskActionClass {
    /// Provider-backed mutation.
    ExternalProviderMutation,
    /// Helper-backed remote mutation.
    HelperBackedRemoteMutation,
}

/// Authority requirement for a high-risk mutation binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationAuthorityRequirement {
    /// Mutation can proceed with either an approval ticket or a reviewed scope.
    TicketOrReviewedScope,
    /// Mutation requires an approval ticket.
    ApprovalTicketRequired,
    /// Mutation requires a reviewed scope.
    ReviewedScopeRequired,
}

/// Authority kind used by coverage and support/admin summaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalAuthorityKind {
    /// A short-lived approval-ticket record.
    ApprovalTicket,
    /// A bounded reviewed-scope object.
    ReviewedScope,
}

/// Evaluation outcomes for an authority spend attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TicketEvaluationOutcome {
    /// Spend attempt was admitted.
    Admitted,
    /// No ticket or reviewed scope was presented.
    DeniedMissingAuthority,
    /// Ticket or reviewed scope expired.
    DeniedExpired,
    /// Target identity drifted.
    DeniedTargetDrift,
    /// Trust profile changed.
    DeniedTrustProfileDrift,
    /// Sandbox profile changed.
    DeniedSandboxProfileDrift,
    /// Policy epoch changed.
    DeniedPolicyEpochDrift,
    /// Actor scope no longer matches the authority object.
    DeniedActorScopeMismatch,
}

/// Native reapproval route for denied spend attempts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeReapprovalRoute {
    /// No reapproval is needed for admitted spends.
    NotRequired,
    /// Open the native approval sheet.
    NativeReapprovalSheet,
    /// Refresh the target and then open native reapproval.
    RefreshTargetThenReapprove,
    /// Reauthenticate and then open native reapproval.
    ReauthThenReapprove,
    /// Rescope and then open native reapproval.
    RescopeThenReapprove,
    /// Keep the surface inspect-only.
    InspectOnlyDenied,
}

/// Validation report emitted by the first approval-ticket alpha consumer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalTicketAlphaValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version validated by this report.
    pub schema_version: u32,
    /// Packet id under validation.
    pub packet_id: String,
    /// Whether all checks passed.
    pub passed: bool,
    /// Coverage observed while validating the packet.
    pub coverage: ApprovalTicketAlphaCoverage,
    /// Findings emitted by failed checks.
    pub findings: Vec<ApprovalTicketAlphaValidationFinding>,
}

/// Coverage observed during approval-ticket alpha validation.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ApprovalTicketAlphaCoverage {
    /// Actor classes covered by tickets, scopes, bindings, and spend attempts.
    pub actor_classes: BTreeSet<ApprovalActorClass>,
    /// Auth source classes covered by tickets, scopes, bindings, and spend attempts.
    pub auth_source_classes: BTreeSet<ApprovalAuthSourceClass>,
    /// Side-effect classes covered by authority objects.
    pub side_effect_classes: BTreeSet<ApprovalSideEffectClass>,
    /// Authority kinds covered by bindings and spend attempts.
    pub authority_kinds: BTreeSet<ApprovalAuthorityKind>,
    /// High-risk action classes covered by bindings.
    pub high_risk_action_classes: BTreeSet<HighRiskActionClass>,
    /// Evaluation outcomes covered by spend attempts.
    pub evaluation_outcomes: BTreeSet<TicketEvaluationOutcome>,
}

/// Validation finding emitted by the first approval-ticket alpha consumer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalTicketAlphaValidationFinding {
    /// Severity of the finding.
    pub severity: FindingSeverity,
    /// Stable check id.
    pub check_id: String,
    /// Redaction-safe finding message.
    pub message: String,
}

/// Support/admin packet that reconstructs approval-ticket lineage without raw payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalTicketSupportAdminPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for the projection.
    pub schema_version: u32,
    /// Packet id projected into support/admin export.
    pub packet_id: String,
    /// Authority-lineage summaries safe for support and admin packets.
    pub lineage_summaries: Vec<ApprovalTicketLineageSummary>,
    /// Mutation bindings safe for support and admin packets.
    pub mutation_summaries: Vec<MutationAuthoritySummary>,
    /// Spend-attempt summaries safe for support and admin packets.
    pub spend_summaries: Vec<ApprovalTicketSpendSummary>,
    /// Redaction posture for the projection.
    pub redaction_class: RedactionClass,
}

/// Export-safe authority-lineage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalTicketLineageSummary {
    /// Opaque authority reference.
    pub authority_ref: String,
    /// Authority object kind.
    pub authority_kind: ApprovalAuthorityKind,
    /// Actor class Aureline acted as.
    pub actor_class: ApprovalActorClass,
    /// Opaque actor subject reference.
    pub actor_subject_ref: String,
    /// Auth source Aureline used.
    pub auth_source_class: ApprovalAuthSourceClass,
    /// Target identity the authority object was bound to.
    pub target_identity: ApprovalTargetIdentity,
    /// Side-effect class admitted by the authority object.
    pub side_effect_class: ApprovalSideEffectClass,
    /// Sandbox profile admitted by the authority object.
    pub sandbox_profile_ref: String,
    /// Trust profile admitted by the authority object.
    pub trust_profile_ref: String,
    /// Policy epoch admitted by the authority object.
    pub policy_epoch_ref: String,
    /// Expiry of the authority object.
    pub expires_at: String,
    /// Evidence refs safe for support/admin export.
    pub evidence_refs: Vec<String>,
    /// Rollback refs safe for support/admin export.
    pub rollback_refs: Vec<String>,
    /// Actor lineage safe for support/admin export.
    pub actor_lineage: Vec<ApprovalActorLineageEntry>,
    /// Redaction posture for the source authority object.
    pub redaction_class: RedactionClass,
}

/// Export-safe mutation binding summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationAuthoritySummary {
    /// Opaque mutation id.
    pub mutation_id: String,
    /// High-risk action class.
    pub high_risk_action_class: HighRiskActionClass,
    /// Authority requirement.
    pub authority_requirement: MutationAuthorityRequirement,
    /// Authority ref used by the mutation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority_ref: Option<String>,
    /// Authority kind used by the mutation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority_kind: Option<ApprovalAuthorityKind>,
    /// Actor class Aureline acts as.
    pub actor_class: ApprovalActorClass,
    /// Auth source Aureline uses.
    pub auth_source_class: ApprovalAuthSourceClass,
    /// Target identity the mutation is bound to.
    pub target_identity: ApprovalTargetIdentity,
    /// Side-effect class requested by the mutation.
    pub side_effect_class: ApprovalSideEffectClass,
    /// Redaction-safe support summary.
    pub support_summary: String,
}

/// Export-safe spend-attempt summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalTicketSpendSummary {
    /// Opaque spend-attempt id.
    pub spend_attempt_id: String,
    /// Mutation binding this spend attempt evaluates.
    pub mutation_ref: String,
    /// Approval ticket presented at spend time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presented_approval_ticket_ref: Option<String>,
    /// Reviewed scope presented at spend time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presented_reviewed_scope_ref: Option<String>,
    /// Evaluation outcome.
    pub evaluation_outcome: TicketEvaluationOutcome,
    /// Native reapproval route.
    pub native_reapproval_route: NativeReapprovalRoute,
    /// Clear redaction-safe explanation for the outcome.
    pub explanation: String,
    /// Audit event refs safe for support/admin export.
    pub audit_event_refs: Vec<String>,
}

struct ApprovalTicketAlphaValidator<'a> {
    packet: &'a ApprovalTicketAlphaPacket,
    ticket_ids: BTreeSet<&'a str>,
    reviewed_scope_ids: BTreeSet<&'a str>,
    mutation_ids: BTreeSet<&'a str>,
    spend_attempt_ids: BTreeSet<&'a str>,
    tickets_by_id: BTreeMap<&'a str, &'a ApprovalTicketAlphaRecord>,
    reviewed_scopes_by_id: BTreeMap<&'a str, &'a ReviewedScopeAlphaRecord>,
    mutations_by_id: BTreeMap<&'a str, &'a MutationAuthorityBinding>,
    findings: Vec<ApprovalTicketAlphaValidationFinding>,
    coverage: ApprovalTicketAlphaCoverage,
}

impl<'a> ApprovalTicketAlphaValidator<'a> {
    fn new(packet: &'a ApprovalTicketAlphaPacket) -> Self {
        Self {
            packet,
            ticket_ids: BTreeSet::new(),
            reviewed_scope_ids: BTreeSet::new(),
            mutation_ids: BTreeSet::new(),
            spend_attempt_ids: BTreeSet::new(),
            tickets_by_id: BTreeMap::new(),
            reviewed_scopes_by_id: BTreeMap::new(),
            mutations_by_id: BTreeMap::new(),
            findings: Vec::new(),
            coverage: ApprovalTicketAlphaCoverage::default(),
        }
    }

    fn validate(&mut self) {
        self.validate_packet_header();
        self.validate_tickets();
        self.validate_reviewed_scopes();
        self.validate_mutation_bindings();
        self.validate_spend_attempts();
        self.validate_required_coverage();
    }

    fn finish(self) -> ApprovalTicketAlphaValidationReport {
        ApprovalTicketAlphaValidationReport {
            record_kind: APPROVAL_TICKET_ALPHA_VALIDATION_REPORT_RECORD_KIND.to_string(),
            schema_version: APPROVAL_TICKET_ALPHA_SCHEMA_VERSION,
            packet_id: self.packet.packet_id.clone(),
            passed: self.findings.is_empty(),
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn validate_packet_header(&mut self) {
        self.expect(
            self.packet.record_kind == APPROVAL_TICKET_ALPHA_PACKET_RECORD_KIND,
            "approval_ticket_alpha.packet_record_kind",
            "packet record_kind must be approval_ticket_alpha_packet",
        );
        self.expect(
            self.packet.approval_ticket_alpha_schema_version
                == APPROVAL_TICKET_ALPHA_SCHEMA_VERSION,
            "approval_ticket_alpha.packet_schema_version",
            "packet schema version must match the crate constant",
        );
        for contract_ref in self.packet.contract_refs.all_refs() {
            self.expect(
                non_empty(contract_ref),
                "approval_ticket_alpha.contract_ref_missing",
                "every consumed upstream contract ref must be non-empty",
            );
        }
    }

    fn validate_tickets(&mut self) {
        self.expect(
            !self.packet.approval_tickets.is_empty(),
            "approval_ticket_alpha.tickets_missing",
            "at least one approval ticket is required",
        );

        for ticket in &self.packet.approval_tickets {
            self.expect(
                ticket.record_kind == APPROVAL_TICKET_ALPHA_RECORD_KIND,
                "approval_ticket_alpha.ticket_record_kind",
                "approval ticket record_kind is wrong",
            );
            self.expect(
                ticket.approval_ticket_alpha_schema_version == APPROVAL_TICKET_ALPHA_SCHEMA_VERSION,
                "approval_ticket_alpha.ticket_schema_version",
                "approval ticket schema version is wrong",
            );
            let ticket_id_is_unique = self.ticket_ids.insert(&ticket.approval_ticket_id);
            self.expect(
                ticket_id_is_unique,
                "approval_ticket_alpha.ticket_duplicate",
                "approval ticket ids must be unique",
            );
            self.tickets_by_id
                .insert(&ticket.approval_ticket_id, ticket);
            self.cover_authority_common(
                &ticket.actor_scope,
                &ticket.target_identity,
                ticket.side_effect_class,
                ApprovalAuthorityKind::ApprovalTicket,
            );
            self.expect_actor_is_resolved(&ticket.actor_scope, "ticket");
            self.expect_target_is_bound(&ticket.target_identity, "ticket");
            self.expect(
                timestamp_before(&ticket.issued_at, &ticket.expires_at),
                "approval_ticket_alpha.ticket_expiry_not_after_issue",
                "approval ticket expiry must be after issue time",
            );
            self.expect(
                non_empty(&ticket.runtime_approval_ticket_ref),
                "approval_ticket_alpha.runtime_ticket_ref_missing",
                "approval ticket must cite the runtime approval-ticket record",
            );
            self.expect(
                !ticket.actor_lineage.is_empty(),
                "approval_ticket_alpha.ticket_lineage_missing",
                "approval ticket must preserve actor lineage",
            );
            self.expect(
                has_evidence_or_rollback(&ticket.evidence_refs, &ticket.rollback_refs),
                "approval_ticket_alpha.ticket_evidence_or_rollback_missing",
                "approval ticket must cite evidence or rollback refs",
            );
            self.expect(
                !ticket.raw_payload_refs_present,
                "approval_ticket_alpha.ticket_raw_payload_ref_present",
                "approval tickets cannot carry raw provider payload refs",
            );
        }
    }

    fn validate_reviewed_scopes(&mut self) {
        self.expect(
            !self.packet.reviewed_scopes.is_empty(),
            "approval_ticket_alpha.reviewed_scopes_missing",
            "at least one reviewed scope is required",
        );

        for scope in &self.packet.reviewed_scopes {
            self.expect(
                scope.record_kind == REVIEWED_SCOPE_ALPHA_RECORD_KIND,
                "approval_ticket_alpha.reviewed_scope_record_kind",
                "reviewed scope record_kind is wrong",
            );
            self.expect(
                scope.approval_ticket_alpha_schema_version == APPROVAL_TICKET_ALPHA_SCHEMA_VERSION,
                "approval_ticket_alpha.reviewed_scope_schema_version",
                "reviewed scope schema version is wrong",
            );
            let scope_id_is_unique = self.reviewed_scope_ids.insert(&scope.reviewed_scope_id);
            self.expect(
                scope_id_is_unique,
                "approval_ticket_alpha.reviewed_scope_duplicate",
                "reviewed scope ids must be unique",
            );
            self.reviewed_scopes_by_id
                .insert(&scope.reviewed_scope_id, scope);
            self.cover_authority_common(
                &scope.actor_scope,
                &scope.target_identity,
                scope.side_effect_class,
                ApprovalAuthorityKind::ReviewedScope,
            );
            self.expect_actor_is_resolved(&scope.actor_scope, "reviewed scope");
            self.expect_target_is_bound(&scope.target_identity, "reviewed scope");
            self.expect(
                timestamp_before(&scope.reviewed_at, &scope.expires_at),
                "approval_ticket_alpha.reviewed_scope_expiry_not_after_review",
                "reviewed scope expiry must be after review time",
            );
            self.expect(
                !scope.actor_lineage.is_empty(),
                "approval_ticket_alpha.reviewed_scope_lineage_missing",
                "reviewed scope must preserve actor lineage",
            );
            self.expect(
                has_evidence_or_rollback(&scope.evidence_refs, &scope.rollback_refs),
                "approval_ticket_alpha.reviewed_scope_evidence_or_rollback_missing",
                "reviewed scope must cite evidence or rollback refs",
            );
            self.expect(
                !scope.raw_payload_refs_present,
                "approval_ticket_alpha.reviewed_scope_raw_payload_ref_present",
                "reviewed scopes cannot carry raw provider payload refs",
            );
        }
    }

    fn validate_mutation_bindings(&mut self) {
        self.expect(
            !self.packet.mutation_bindings.is_empty(),
            "approval_ticket_alpha.mutation_bindings_missing",
            "at least one high-risk mutation binding is required",
        );

        for binding in &self.packet.mutation_bindings {
            self.expect(
                binding.record_kind == MUTATION_AUTHORITY_BINDING_RECORD_KIND,
                "approval_ticket_alpha.mutation_binding_record_kind",
                "mutation authority binding record_kind is wrong",
            );
            self.expect(
                binding.approval_ticket_alpha_schema_version
                    == APPROVAL_TICKET_ALPHA_SCHEMA_VERSION,
                "approval_ticket_alpha.mutation_binding_schema_version",
                "mutation authority binding schema version is wrong",
            );
            let mutation_id_is_unique = self.mutation_ids.insert(&binding.mutation_id);
            self.expect(
                mutation_id_is_unique,
                "approval_ticket_alpha.mutation_duplicate",
                "mutation binding ids must be unique",
            );
            self.mutations_by_id.insert(&binding.mutation_id, binding);
            self.coverage
                .actor_classes
                .insert(binding.actor_scope.actor_class);
            self.coverage
                .auth_source_classes
                .insert(binding.actor_scope.auth_source_class);
            self.coverage
                .high_risk_action_classes
                .insert(binding.high_risk_action_class);
            self.expect_actor_is_resolved(&binding.actor_scope, "mutation binding");
            self.expect_target_is_bound(&binding.target_identity, "mutation binding");
            self.expect(
                !binding.audit_event_refs.is_empty(),
                "approval_ticket_alpha.mutation_audit_missing",
                "mutation binding must cite audit event refs",
            );
            self.expect(
                non_empty(&binding.support_summary),
                "approval_ticket_alpha.mutation_summary_missing",
                "mutation binding must include a support-safe summary",
            );
            self.validate_authority_requirement(binding);
            self.validate_binding_authority_matches(binding);
        }
    }

    fn validate_authority_requirement(&mut self, binding: &MutationAuthorityBinding) {
        let has_ticket = binding
            .approval_ticket_ref
            .as_deref()
            .is_some_and(non_empty);
        let has_scope = binding.reviewed_scope_ref.as_deref().is_some_and(non_empty);
        self.expect(
            has_ticket ^ has_scope,
            "approval_ticket_alpha.mutation_authority_cardinality_invalid",
            "high-risk mutations must cite exactly one ticket or reviewed scope",
        );
        self.expect(
            has_ticket || has_scope,
            "approval_ticket_alpha.mutation_authority_missing",
            "high-risk mutations require a ticket or reviewed scope object",
        );
        match binding.authority_requirement {
            MutationAuthorityRequirement::TicketOrReviewedScope => {}
            MutationAuthorityRequirement::ApprovalTicketRequired => self.expect(
                has_ticket,
                "approval_ticket_alpha.mutation_ticket_missing",
                "this mutation requires an approval ticket",
            ),
            MutationAuthorityRequirement::ReviewedScopeRequired => self.expect(
                has_scope,
                "approval_ticket_alpha.mutation_reviewed_scope_missing",
                "this mutation requires a reviewed scope object",
            ),
        }
        if let Some(ticket_ref) = &binding.approval_ticket_ref {
            self.coverage
                .authority_kinds
                .insert(ApprovalAuthorityKind::ApprovalTicket);
            self.expect(
                self.ticket_ids.contains(ticket_ref.as_str()),
                "approval_ticket_alpha.mutation_ticket_unknown",
                "mutation binding cites an unknown approval ticket",
            );
        }
        if let Some(scope_ref) = &binding.reviewed_scope_ref {
            self.coverage
                .authority_kinds
                .insert(ApprovalAuthorityKind::ReviewedScope);
            self.expect(
                self.reviewed_scope_ids.contains(scope_ref.as_str()),
                "approval_ticket_alpha.mutation_reviewed_scope_unknown",
                "mutation binding cites an unknown reviewed scope",
            );
        }
    }

    fn validate_binding_authority_matches(&mut self, binding: &MutationAuthorityBinding) {
        if let Some(ticket_ref) = &binding.approval_ticket_ref {
            if let Some(ticket) = self.tickets_by_id.get(ticket_ref.as_str()) {
                let (side_effect_matches, target_matches, actor_matches, context_matches) = {
                    (
                        binding.side_effect_class == ticket.side_effect_class,
                        target_identity_matches(&binding.target_identity, &ticket.target_identity),
                        actor_scope_matches(&binding.actor_scope, &ticket.actor_scope),
                        binding.sandbox_profile_ref == ticket.sandbox_profile_ref
                            && binding.trust_profile_ref == ticket.trust_profile_ref
                            && binding.policy_epoch_ref == ticket.policy_epoch_ref,
                    )
                };
                self.expect(
                    side_effect_matches,
                    "approval_ticket_alpha.mutation_ticket_side_effect_mismatch",
                    "mutation side effect must match the cited ticket",
                );
                self.expect(
                    target_matches,
                    "approval_ticket_alpha.mutation_ticket_target_mismatch",
                    "mutation target identity must match the cited ticket",
                );
                self.expect(
                    actor_matches,
                    "approval_ticket_alpha.mutation_ticket_actor_mismatch",
                    "mutation actor scope must match the cited ticket",
                );
                self.expect(
                    context_matches,
                    "approval_ticket_alpha.mutation_ticket_context_mismatch",
                    "mutation sandbox, trust profile, and policy epoch must match the cited ticket",
                );
            }
        }
        if let Some(scope_ref) = &binding.reviewed_scope_ref {
            if let Some(scope) = self.reviewed_scopes_by_id.get(scope_ref.as_str()) {
                let (side_effect_matches, target_matches, actor_matches, context_matches) = {
                    (
                        binding.side_effect_class == scope.side_effect_class,
                        target_identity_matches(&binding.target_identity, &scope.target_identity),
                        actor_scope_matches(&binding.actor_scope, &scope.actor_scope),
                        binding.sandbox_profile_ref == scope.sandbox_profile_ref
                            && binding.trust_profile_ref == scope.trust_profile_ref
                            && binding.policy_epoch_ref == scope.policy_epoch_ref,
                    )
                };
                self.expect(
                    side_effect_matches,
                    "approval_ticket_alpha.mutation_scope_side_effect_mismatch",
                    "mutation side effect must match the cited reviewed scope",
                );
                self.expect(
                    target_matches,
                    "approval_ticket_alpha.mutation_scope_target_mismatch",
                    "mutation target identity must match the cited reviewed scope",
                );
                self.expect(
                    actor_matches,
                    "approval_ticket_alpha.mutation_scope_actor_mismatch",
                    "mutation actor scope must match the cited reviewed scope",
                );
                self.expect(
                    context_matches,
                    "approval_ticket_alpha.mutation_scope_context_mismatch",
                    "mutation sandbox, trust profile, and policy epoch must match the cited reviewed scope",
                );
            }
        }
    }

    fn validate_spend_attempts(&mut self) {
        self.expect(
            !self.packet.spend_attempts.is_empty(),
            "approval_ticket_alpha.spend_attempts_missing",
            "at least one spend attempt is required",
        );

        for attempt in &self.packet.spend_attempts {
            self.expect(
                attempt.record_kind == APPROVAL_TICKET_SPEND_ATTEMPT_RECORD_KIND,
                "approval_ticket_alpha.spend_attempt_record_kind",
                "spend attempt record_kind is wrong",
            );
            self.expect(
                attempt.approval_ticket_alpha_schema_version
                    == APPROVAL_TICKET_ALPHA_SCHEMA_VERSION,
                "approval_ticket_alpha.spend_attempt_schema_version",
                "spend attempt schema version is wrong",
            );
            let spend_id_is_unique = self.spend_attempt_ids.insert(&attempt.spend_attempt_id);
            self.expect(
                spend_id_is_unique,
                "approval_ticket_alpha.spend_attempt_duplicate",
                "spend attempt ids must be unique",
            );
            self.coverage
                .actor_classes
                .insert(attempt.current_actor_scope.actor_class);
            self.coverage
                .auth_source_classes
                .insert(attempt.current_actor_scope.auth_source_class);
            self.coverage
                .evaluation_outcomes
                .insert(attempt.evaluation_outcome);
            self.expect(
                self.mutation_ids.contains(attempt.mutation_ref.as_str()),
                "approval_ticket_alpha.spend_mutation_unknown",
                "spend attempt must cite a known mutation binding",
            );
            self.expect(
                !attempt.audit_event_refs.is_empty(),
                "approval_ticket_alpha.spend_audit_missing",
                "spend attempt must cite audit event refs",
            );
            self.expect(
                non_empty(&attempt.explanation),
                "approval_ticket_alpha.spend_explanation_missing",
                "spend attempt must include a clear explanation",
            );
            let expected_outcome = self.expected_outcome_for_attempt(attempt);
            self.expect(
                attempt.evaluation_outcome == expected_outcome,
                "approval_ticket_alpha.spend_expected_outcome_mismatch",
                "spend attempt outcome must match current target, actor, sandbox, trust, policy, and expiry state",
            );
            self.validate_reapproval_route(attempt, expected_outcome);
        }
    }

    fn expected_outcome_for_attempt(
        &self,
        attempt: &ApprovalTicketSpendAttempt,
    ) -> TicketEvaluationOutcome {
        let has_ticket = attempt
            .presented_approval_ticket_ref
            .as_deref()
            .is_some_and(non_empty);
        let has_scope = attempt
            .presented_reviewed_scope_ref
            .as_deref()
            .is_some_and(non_empty);
        if has_ticket == has_scope {
            return TicketEvaluationOutcome::DeniedMissingAuthority;
        }
        if let Some(ticket_ref) = &attempt.presented_approval_ticket_ref {
            if let Some(ticket) = self.tickets_by_id.get(ticket_ref.as_str()) {
                return expected_outcome_against_authority(
                    &attempt.current_actor_scope,
                    &attempt.current_target_identity,
                    &attempt.current_sandbox_profile_ref,
                    &attempt.current_trust_profile_ref,
                    &attempt.current_policy_epoch_ref,
                    &attempt.evaluated_at,
                    AuthorityContext {
                        actor_scope: &ticket.actor_scope,
                        target_identity: &ticket.target_identity,
                        sandbox_profile_ref: &ticket.sandbox_profile_ref,
                        trust_profile_ref: &ticket.trust_profile_ref,
                        policy_epoch_ref: &ticket.policy_epoch_ref,
                        expires_at: &ticket.expires_at,
                    },
                );
            }
            return TicketEvaluationOutcome::DeniedMissingAuthority;
        }
        if let Some(scope_ref) = &attempt.presented_reviewed_scope_ref {
            if let Some(scope) = self.reviewed_scopes_by_id.get(scope_ref.as_str()) {
                return expected_outcome_against_authority(
                    &attempt.current_actor_scope,
                    &attempt.current_target_identity,
                    &attempt.current_sandbox_profile_ref,
                    &attempt.current_trust_profile_ref,
                    &attempt.current_policy_epoch_ref,
                    &attempt.evaluated_at,
                    AuthorityContext {
                        actor_scope: &scope.actor_scope,
                        target_identity: &scope.target_identity,
                        sandbox_profile_ref: &scope.sandbox_profile_ref,
                        trust_profile_ref: &scope.trust_profile_ref,
                        policy_epoch_ref: &scope.policy_epoch_ref,
                        expires_at: &scope.expires_at,
                    },
                );
            }
        }
        TicketEvaluationOutcome::DeniedMissingAuthority
    }

    fn validate_reapproval_route(
        &mut self,
        attempt: &ApprovalTicketSpendAttempt,
        expected_outcome: TicketEvaluationOutcome,
    ) {
        if expected_outcome == TicketEvaluationOutcome::Admitted {
            self.expect(
                attempt.native_reapproval_route == NativeReapprovalRoute::NotRequired,
                "approval_ticket_alpha.admitted_reapproval_route_invalid",
                "admitted spend attempts must not route to reapproval",
            );
        } else {
            self.expect(
                attempt.native_reapproval_route != NativeReapprovalRoute::NotRequired,
                "approval_ticket_alpha.denied_reapproval_route_missing",
                "denied spend attempts must route to native reapproval or inspect-only denial",
            );
            if matches!(
                expected_outcome,
                TicketEvaluationOutcome::DeniedExpired
                    | TicketEvaluationOutcome::DeniedTargetDrift
                    | TicketEvaluationOutcome::DeniedTrustProfileDrift
                    | TicketEvaluationOutcome::DeniedSandboxProfileDrift
                    | TicketEvaluationOutcome::DeniedPolicyEpochDrift
                    | TicketEvaluationOutcome::DeniedActorScopeMismatch
            ) {
                self.expect(
                    matches!(
                        attempt.native_reapproval_route,
                        NativeReapprovalRoute::NativeReapprovalSheet
                            | NativeReapprovalRoute::RefreshTargetThenReapprove
                            | NativeReapprovalRoute::ReauthThenReapprove
                            | NativeReapprovalRoute::RescopeThenReapprove
                    ),
                    "approval_ticket_alpha.drift_reapproval_route_invalid",
                    "expired or drifted authority must route to native reapproval rather than silent replay",
                );
            }
        }
    }

    fn validate_required_coverage(&mut self) {
        for actor_class in [
            ApprovalActorClass::HumanAccount,
            ApprovalActorClass::InstallationOrAppGrant,
            ApprovalActorClass::DelegatedCredential,
            ApprovalActorClass::LocalOnlyAuthority,
        ] {
            self.expect(
                self.coverage.actor_classes.contains(&actor_class),
                "approval_ticket_alpha.actor_class_coverage_missing",
                "approval-ticket coverage must distinguish human, installation, delegated, and local-only authority",
            );
        }

        for authority_kind in [
            ApprovalAuthorityKind::ApprovalTicket,
            ApprovalAuthorityKind::ReviewedScope,
        ] {
            self.expect(
                self.coverage.authority_kinds.contains(&authority_kind),
                "approval_ticket_alpha.authority_kind_coverage_missing",
                "approval-ticket coverage must include tickets and reviewed scopes",
            );
        }

        for action_class in [
            HighRiskActionClass::ExternalProviderMutation,
            HighRiskActionClass::HelperBackedRemoteMutation,
        ] {
            self.expect(
                self.coverage
                    .high_risk_action_classes
                    .contains(&action_class),
                "approval_ticket_alpha.high_risk_action_coverage_missing",
                "coverage must include provider and helper-backed high-risk mutations",
            );
        }

        for outcome in [
            TicketEvaluationOutcome::Admitted,
            TicketEvaluationOutcome::DeniedExpired,
            TicketEvaluationOutcome::DeniedTargetDrift,
            TicketEvaluationOutcome::DeniedTrustProfileDrift,
            TicketEvaluationOutcome::DeniedSandboxProfileDrift,
            TicketEvaluationOutcome::DeniedPolicyEpochDrift,
            TicketEvaluationOutcome::DeniedActorScopeMismatch,
        ] {
            self.expect(
                self.coverage.evaluation_outcomes.contains(&outcome),
                "approval_ticket_alpha.evaluation_outcome_coverage_missing",
                "coverage must prove admitted plus expired and drifted fail-closed outcomes",
            );
        }
    }

    fn cover_authority_common(
        &mut self,
        actor_scope: &ApprovalActorScope,
        target_identity: &ApprovalTargetIdentity,
        side_effect_class: ApprovalSideEffectClass,
        authority_kind: ApprovalAuthorityKind,
    ) {
        self.coverage.actor_classes.insert(actor_scope.actor_class);
        self.coverage
            .auth_source_classes
            .insert(actor_scope.auth_source_class);
        self.coverage.side_effect_classes.insert(side_effect_class);
        self.coverage.authority_kinds.insert(authority_kind);
        self.expect_target_is_bound(target_identity, "authority object");
    }

    fn expect_actor_is_resolved(&mut self, actor_scope: &ApprovalActorScope, owner: &str) {
        self.expect(
            actor_scope.actor_class != ApprovalActorClass::UnknownActorClass,
            "approval_ticket_alpha.actor_unknown",
            &format!("{owner} cannot act under unknown actor scope"),
        );
        self.expect(
            actor_scope.auth_source_class != ApprovalAuthSourceClass::UnknownAuthSource,
            "approval_ticket_alpha.auth_source_unknown",
            &format!("{owner} cannot act under unknown auth source"),
        );
        self.expect(
            non_empty(&actor_scope.actor_subject_ref),
            "approval_ticket_alpha.actor_subject_missing",
            &format!("{owner} must cite an actor subject ref"),
        );
    }

    fn expect_target_is_bound(&mut self, target_identity: &ApprovalTargetIdentity, owner: &str) {
        self.expect(
            non_empty(&target_identity.target_ref.target_ref),
            "approval_ticket_alpha.target_ref_missing",
            &format!("{owner} must cite a target ref"),
        );
        self.expect(
            non_empty(&target_identity.target_fingerprint_ref),
            "approval_ticket_alpha.target_fingerprint_missing",
            &format!("{owner} must cite a target fingerprint ref"),
        );
        self.expect(
            non_empty(&target_identity.target_version_ref),
            "approval_ticket_alpha.target_version_missing",
            &format!("{owner} must cite a target version ref"),
        );
    }

    fn expect(&mut self, passed: bool, check_id: &str, message: &str) {
        if !passed {
            self.findings.push(ApprovalTicketAlphaValidationFinding {
                severity: FindingSeverity::Error,
                check_id: check_id.to_string(),
                message: message.to_string(),
            });
        }
    }
}

struct AuthorityContext<'a> {
    actor_scope: &'a ApprovalActorScope,
    target_identity: &'a ApprovalTargetIdentity,
    sandbox_profile_ref: &'a str,
    trust_profile_ref: &'a str,
    policy_epoch_ref: &'a str,
    expires_at: &'a str,
}

fn expected_outcome_against_authority(
    current_actor_scope: &ApprovalActorScope,
    current_target_identity: &ApprovalTargetIdentity,
    current_sandbox_profile_ref: &str,
    current_trust_profile_ref: &str,
    current_policy_epoch_ref: &str,
    evaluated_at: &str,
    authority: AuthorityContext<'_>,
) -> TicketEvaluationOutcome {
    if timestamp_at_or_after(evaluated_at, authority.expires_at) {
        return TicketEvaluationOutcome::DeniedExpired;
    }
    if !target_identity_matches(current_target_identity, authority.target_identity) {
        return TicketEvaluationOutcome::DeniedTargetDrift;
    }
    if current_trust_profile_ref != authority.trust_profile_ref {
        return TicketEvaluationOutcome::DeniedTrustProfileDrift;
    }
    if current_sandbox_profile_ref != authority.sandbox_profile_ref {
        return TicketEvaluationOutcome::DeniedSandboxProfileDrift;
    }
    if current_policy_epoch_ref != authority.policy_epoch_ref {
        return TicketEvaluationOutcome::DeniedPolicyEpochDrift;
    }
    if !actor_scope_matches(current_actor_scope, authority.actor_scope) {
        return TicketEvaluationOutcome::DeniedActorScopeMismatch;
    }
    TicketEvaluationOutcome::Admitted
}

fn target_identity_matches(
    current: &ApprovalTargetIdentity,
    expected: &ApprovalTargetIdentity,
) -> bool {
    current.target_class == expected.target_class
        && current.target_ref.target_ref_class == expected.target_ref.target_ref_class
        && current.target_ref.target_ref == expected.target_ref.target_ref
        && current.target_fingerprint_ref == expected.target_fingerprint_ref
        && current.target_version_ref == expected.target_version_ref
}

fn actor_scope_matches(current: &ApprovalActorScope, expected: &ApprovalActorScope) -> bool {
    if current.actor_class != expected.actor_class
        || current.actor_subject_ref != expected.actor_subject_ref
        || current.auth_source_class != expected.auth_source_class
    {
        return false;
    }
    let current_scopes: BTreeSet<&str> = current
        .granted_scope_refs
        .iter()
        .map(String::as_str)
        .collect();
    let expected_scopes: BTreeSet<&str> = expected
        .granted_scope_refs
        .iter()
        .map(String::as_str)
        .collect();
    current_scopes == expected_scopes
}

fn has_evidence_or_rollback(evidence_refs: &[String], rollback_refs: &[String]) -> bool {
    evidence_refs.iter().any(|value| non_empty(value))
        || rollback_refs.iter().any(|value| non_empty(value))
}

fn timestamp_before(left: &str, right: &str) -> bool {
    non_empty(left) && non_empty(right) && left < right
}

fn timestamp_at_or_after(left: &str, right: &str) -> bool {
    non_empty(left) && non_empty(right) && left >= right
}

fn non_empty(value: &str) -> bool {
    !value.trim().is_empty()
}
