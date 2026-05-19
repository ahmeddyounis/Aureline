//! Durable AI run history, approval timeline, and rerun-review baseline.
//!
//! This module preserves the durable-history half of the AI replayability
//! contract. One [`AiRunHistoryEntry`] is the canonical row for a material AI
//! run: it carries the canonical run id that AI thread, evidence packet,
//! support packet, and replay view all read; the actor and provider/model
//! identity that issued the run; the execution boundary it crossed; an
//! ordered approval timeline that preserves every approve/deny/revoke/expire
//! event with its scope, policy epoch, and actor/object identity; and an
//! `evidence_completeness_class` that records when provider, connector, or
//! policy drift means the run can no longer be reconstructed in full
//! fidelity. One [`AiRerunReview`] compares the original run against the
//! current workspace, policy, provider, model, and tool state, re-resolves
//! the approvals it would need, and exposes typed `Rerun`, `Cancel`, and
//! `OpenAsRecipe` action offers without ever hiding drift.
//!
//! The records store metadata and opaque refs only. They do not carry raw
//! prompt text, raw provider payloads, raw diff bodies, raw endpoint URLs,
//! raw token counts, raw cost amounts, or credential bodies. Replay, rerun,
//! and history surfaces preserve approval and provider truth verbatim
//! instead of collapsing it into a final-status snapshot.
//!
//! The cross-tool contracts the module projects against are the entry
//! schema [`AI_RUN_HISTORY_ENTRY_SCHEMA_REF`], the rerun-review schema
//! [`AI_RERUN_REVIEW_SCHEMA_REF`], and the parity report at
//! [`AI_RUN_HISTORY_PARITY_ARTIFACT_REF`].

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::routing::RoutingPolicyContext;

/// Stable record-kind tag carried by [`AiRunHistoryEntry`] rows.
pub const AI_RUN_HISTORY_ENTRY_RECORD_KIND: &str = "ai_run_history_entry_record";

/// Stable record-kind tag carried by [`AiRerunReview`] rows.
pub const AI_RERUN_REVIEW_RECORD_KIND: &str = "ai_rerun_review_record";

/// Stable record-kind tag carried by [`AiRunHistoryParityPacket`] payloads.
pub const AI_RUN_HISTORY_PARITY_PACKET_RECORD_KIND: &str = "ai_run_history_parity_packet_record";

/// Stable record-kind tag carried by [`AiRunHistorySurfaceRow`] rows.
pub const AI_RUN_HISTORY_SURFACE_ROW_RECORD_KIND: &str = "ai_run_history_surface_row_record";

/// Stable record-kind tag carried by [`AiRunHistorySupportPacket`] payloads.
pub const AI_RUN_HISTORY_SUPPORT_PACKET_RECORD_KIND: &str =
    "ai_run_history_support_packet_record";

/// Schema version of the run-history entry, rerun review, and parity packet.
pub const AI_RUN_HISTORY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the run-history entry boundary schema.
pub const AI_RUN_HISTORY_ENTRY_SCHEMA_REF: &str = "schemas/ai/ai_run_history_entry.schema.json";

/// Repo-relative path of the rerun-review boundary schema.
pub const AI_RERUN_REVIEW_SCHEMA_REF: &str = "schemas/ai/ai_rerun_review.schema.json";

/// Repo-relative path of the protected run-history and replay fixture corpus.
pub const AI_RUN_HISTORY_FIXTURE_DIR: &str = "fixtures/ai/m3/run_history_and_replay";

/// Repo-relative path of the checked-in run-history parity report.
pub const AI_RUN_HISTORY_PARITY_ARTIFACT_REF: &str =
    "artifacts/ai/m3/ai_run_history_parity_report.md";

/// Actor identity class that issued or owns a recorded run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRunActorClass {
    /// Local user at the AI surface.
    LocalUser,
    /// Admin policy.
    AdminPolicy,
    /// Platform control authority.
    PlatformControl,
    /// Automated policy evaluation.
    AutomatedPolicyGate,
    /// Background branch-agent dispatch.
    BackgroundBranchAgent,
}

impl AiRunActorClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalUser => "local_user",
            Self::AdminPolicy => "admin_policy",
            Self::PlatformControl => "platform_control",
            Self::AutomatedPolicyGate => "automated_policy_gate",
            Self::BackgroundBranchAgent => "background_branch_agent",
        }
    }
}

/// Execution boundary class for a recorded run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRunExecutionBoundaryClass {
    /// In-process tool or evaluation.
    LocalInProcess,
    /// Local subprocess on the same device.
    LocalSubprocessSameDevice,
    /// Local sandboxed container on the same device.
    LocalSandboxedContainerSameDevice,
    /// Local companion service over loopback.
    LocalCompanionServiceLoopback,
    /// Remote vendor-managed service.
    RemoteVendorManagedService,
    /// Remote self-hosted service.
    RemoteSelfHostedService,
    /// Enterprise gateway brokered service.
    EnterpriseGatewayBrokeredService,
    /// Extension-provided locus.
    ExtensionProvidedLocus,
}

impl AiRunExecutionBoundaryClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalInProcess => "local_in_process",
            Self::LocalSubprocessSameDevice => "local_subprocess_same_device",
            Self::LocalSandboxedContainerSameDevice => "local_sandboxed_container_same_device",
            Self::LocalCompanionServiceLoopback => "local_companion_service_loopback",
            Self::RemoteVendorManagedService => "remote_vendor_managed_service",
            Self::RemoteSelfHostedService => "remote_self_hosted_service",
            Self::EnterpriseGatewayBrokeredService => "enterprise_gateway_brokered_service",
            Self::ExtensionProvidedLocus => "extension_provided_locus",
        }
    }

    /// True when the boundary keeps execution on the user's device.
    pub const fn is_local(self) -> bool {
        matches!(
            self,
            Self::LocalInProcess
                | Self::LocalSubprocessSameDevice
                | Self::LocalSandboxedContainerSameDevice
                | Self::LocalCompanionServiceLoopback
        )
    }
}

/// Outcome class recorded on one run-history row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRunOutcomeClass {
    /// Inspect-only succeeded.
    SucceededInspectOnly,
    /// Reversible local edit succeeded.
    SucceededWithLocalReversibleEdit,
    /// Destructive local edit succeeded.
    SucceededWithLocalDestructiveEdit,
    /// Branch-agent dispatch succeeded.
    SucceededWithBranchAgentDispatch,
    /// External publish succeeded.
    SucceededWithExternalPublish,
    /// Run was denied by policy.
    DeniedByPolicy,
    /// Run was denied by workspace trust.
    DeniedByWorkspaceTrust,
    /// Run was denied because an approval ticket was missing.
    DeniedByApprovalMissing,
    /// Run was denied because the quota was exhausted.
    DeniedByQuota,
    /// Run was denied by a tainted-context fence.
    DeniedByTaintedContext,
    /// Provider error.
    ErrorProvider,
    /// Transport error.
    ErrorTransport,
    /// Timeout error.
    ErrorTimeout,
    /// User cancelled the run.
    CancelledByUser,
    /// Support replay reconstructed the run with no real side effect.
    SupportReplayNoSideEffect,
}

impl AiRunOutcomeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SucceededInspectOnly => "succeeded_inspect_only",
            Self::SucceededWithLocalReversibleEdit => "succeeded_with_local_reversible_edit",
            Self::SucceededWithLocalDestructiveEdit => "succeeded_with_local_destructive_edit",
            Self::SucceededWithBranchAgentDispatch => "succeeded_with_branch_agent_dispatch",
            Self::SucceededWithExternalPublish => "succeeded_with_external_publish",
            Self::DeniedByPolicy => "denied_by_policy",
            Self::DeniedByWorkspaceTrust => "denied_by_workspace_trust",
            Self::DeniedByApprovalMissing => "denied_by_approval_missing",
            Self::DeniedByQuota => "denied_by_quota",
            Self::DeniedByTaintedContext => "denied_by_tainted_context",
            Self::ErrorProvider => "error_provider",
            Self::ErrorTransport => "error_transport",
            Self::ErrorTimeout => "error_timeout",
            Self::CancelledByUser => "cancelled_by_user",
            Self::SupportReplayNoSideEffect => "support_replay_no_side_effect",
        }
    }

    /// True when this outcome admits no observable side effect.
    pub const fn is_no_side_effect(self) -> bool {
        matches!(
            self,
            Self::SucceededInspectOnly
                | Self::DeniedByPolicy
                | Self::DeniedByWorkspaceTrust
                | Self::DeniedByApprovalMissing
                | Self::DeniedByQuota
                | Self::DeniedByTaintedContext
                | Self::ErrorProvider
                | Self::ErrorTransport
                | Self::ErrorTimeout
                | Self::CancelledByUser
                | Self::SupportReplayNoSideEffect
        )
    }
}

/// Coarse cost band for a recorded run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRunCostBandClass {
    /// No cost incurred.
    NoCost,
    /// Negligible cost band.
    NegligibleCost,
    /// Small cost band.
    SmallCost,
    /// Moderate cost band.
    ModerateCost,
    /// Large cost band.
    LargeCost,
    /// Cost is unknown and MUST be disclosed.
    CostUnknownMustDisclose,
}

impl AiRunCostBandClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoCost => "no_cost",
            Self::NegligibleCost => "negligible_cost",
            Self::SmallCost => "small_cost",
            Self::ModerateCost => "moderate_cost",
            Self::LargeCost => "large_cost",
            Self::CostUnknownMustDisclose => "cost_unknown_must_disclose",
        }
    }
}

/// Coarse quota band for a recorded run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRunQuotaBandClass {
    /// Quota is unconstrained.
    QuotaUnconstrained,
    /// Quota is healthy.
    QuotaHealthy,
    /// Quota is pressured.
    QuotaPressured,
    /// Quota has been exhausted.
    QuotaExhausted,
    /// Quota was revoked.
    QuotaRevoked,
    /// Quota is unknown and MUST be disclosed.
    QuotaUnknownMustDisclose,
}

impl AiRunQuotaBandClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuotaUnconstrained => "quota_unconstrained",
            Self::QuotaHealthy => "quota_healthy",
            Self::QuotaPressured => "quota_pressured",
            Self::QuotaExhausted => "quota_exhausted",
            Self::QuotaRevoked => "quota_revoked",
            Self::QuotaUnknownMustDisclose => "quota_unknown_must_disclose",
        }
    }
}

/// Decision class recorded on one approval-timeline event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalEventDecisionClass {
    /// Approval is waiting for a decision.
    PendingUserReview,
    /// Approval was granted.
    Granted,
    /// Approval was rejected.
    Rejected,
    /// Approval was revoked after being granted.
    Revoked,
    /// Approval expired.
    Expired,
    /// Approval was blocked by policy.
    BlockedByPolicy,
}

impl ApprovalEventDecisionClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PendingUserReview => "pending_user_review",
            Self::Granted => "granted",
            Self::Rejected => "rejected",
            Self::Revoked => "revoked",
            Self::Expired => "expired",
            Self::BlockedByPolicy => "blocked_by_policy",
        }
    }

    /// True when the event is one that admits a material run.
    pub const fn admits_run(self) -> bool {
        matches!(self, Self::Granted)
    }
}

/// Scope class recorded on one approval-timeline event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalScopeClass {
    /// One-time approval for a single invocation.
    OneTimePerInvocation,
    /// Approval is scoped to one session.
    PerSession,
    /// Approval is scoped to one workspace.
    PerWorkspace,
    /// Approval is scoped to one policy epoch.
    PerPolicyEpoch,
    /// Approval is admin-ticketed.
    AdminTicket,
}

impl ApprovalScopeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OneTimePerInvocation => "one_time_per_invocation",
            Self::PerSession => "per_session",
            Self::PerWorkspace => "per_workspace",
            Self::PerPolicyEpoch => "per_policy_epoch",
            Self::AdminTicket => "admin_ticket",
        }
    }
}

/// Actor class that decided an approval event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalEventActorClass {
    /// Local user.
    LocalUser,
    /// Admin policy.
    AdminPolicy,
    /// Platform control authority.
    PlatformControl,
    /// Automated policy gate.
    AutomatedPolicyGate,
}

impl ApprovalEventActorClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalUser => "local_user",
            Self::AdminPolicy => "admin_policy",
            Self::PlatformControl => "platform_control",
            Self::AutomatedPolicyGate => "automated_policy_gate",
        }
    }
}

/// Object class an approval event covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalObjectClass {
    /// Approval covers an AI run.
    AiRun,
    /// Approval covers a tool invocation.
    ToolInvocation,
    /// Approval covers a branch-agent dispatch.
    BranchAgentDispatch,
    /// Approval covers an external publish.
    ExternalPublish,
    /// Approval covers a policy override.
    PolicyOverride,
    /// Approval covers a route override.
    RouteOverride,
    /// Approval covers a first-use admission.
    FirstUseAdmission,
}

impl ApprovalObjectClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiRun => "ai_run",
            Self::ToolInvocation => "tool_invocation",
            Self::BranchAgentDispatch => "branch_agent_dispatch",
            Self::ExternalPublish => "external_publish",
            Self::PolicyOverride => "policy_override",
            Self::RouteOverride => "route_override",
            Self::FirstUseAdmission => "first_use_admission",
        }
    }
}

/// Lifecycle state class of one run-history row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRunHistoryStateClass {
    /// Run is in review and has not landed an outcome yet.
    ActiveInReview,
    /// Run applied a material change.
    Applied,
    /// Run was rejected.
    Rejected,
    /// Run was cancelled.
    Cancelled,
    /// Run expired in review without an outcome.
    ExpiredInReview,
}

impl AiRunHistoryStateClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveInReview => "active_in_review",
            Self::Applied => "applied",
            Self::Rejected => "rejected",
            Self::Cancelled => "cancelled",
            Self::ExpiredInReview => "expired_in_review",
        }
    }

    /// True when the row is terminal and MUST carry a completed_at timestamp.
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Applied | Self::Rejected | Self::Cancelled)
    }
}

/// Evidence completeness class for a run-history row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceCompletenessClass {
    /// Evidence is reconstructible in full fidelity.
    ReconstructibleFull,
    /// Evidence is incomplete; replay is degraded.
    EvidenceIncompleteDegradedReplay,
    /// Evidence is incomplete; replay is blocked.
    EvidenceIncompleteReplayBlocked,
}

impl EvidenceCompletenessClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructibleFull => "reconstructible_full",
            Self::EvidenceIncompleteDegradedReplay => "evidence_incomplete_degraded_replay",
            Self::EvidenceIncompleteReplayBlocked => "evidence_incomplete_replay_blocked",
        }
    }

    /// True when the row records incompleteness and MUST carry a typed reason.
    pub const fn requires_reason(self) -> bool {
        !matches!(self, Self::ReconstructibleFull)
    }
}

/// Typed reason a run-history row records incomplete evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceIncompletenessReasonClass {
    /// Provider was withdrawn from the registry.
    ProviderWithdrawn,
    /// Provider is currently unreachable.
    ProviderUnreachable,
    /// Model revision is no longer available.
    ModelRevisionNoLongerAvailable,
    /// External tool descriptor was withdrawn.
    ExternalToolDescriptorWithdrawn,
    /// Connector credential was revoked.
    ConnectorCredentialRevoked,
    /// Policy epoch changed since the run.
    PolicyEpochChanged,
    /// Workspace revision is no longer available.
    WorkspaceRevisionUnavailable,
    /// Evidence export was redacted in replay.
    EvidenceExportRedactedInReplay,
    /// User deleted run history.
    UserDeletedHistory,
}

impl EvidenceIncompletenessReasonClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderWithdrawn => "provider_withdrawn",
            Self::ProviderUnreachable => "provider_unreachable",
            Self::ModelRevisionNoLongerAvailable => "model_revision_no_longer_available",
            Self::ExternalToolDescriptorWithdrawn => "external_tool_descriptor_withdrawn",
            Self::ConnectorCredentialRevoked => "connector_credential_revoked",
            Self::PolicyEpochChanged => "policy_epoch_changed",
            Self::WorkspaceRevisionUnavailable => "workspace_revision_unavailable",
            Self::EvidenceExportRedactedInReplay => "evidence_export_redacted_in_replay",
            Self::UserDeletedHistory => "user_deleted_history",
        }
    }
}

/// Redaction class attached to one run-history row or rerun review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRunHistoryRedactionClass {
    /// Default metadata-safe export.
    MetadataSafeDefault,
    /// Operator-only restricted export.
    OperatorOnlyRestricted,
    /// Internal support restricted export.
    InternalSupportRestricted,
    /// Signing or provenance evidence only.
    SigningEvidenceOnly,
}

impl AiRunHistoryRedactionClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::OperatorOnlyRestricted => "operator_only_restricted",
            Self::InternalSupportRestricted => "internal_support_restricted",
            Self::SigningEvidenceOnly => "signing_evidence_only",
        }
    }
}

/// Validation outcome rollup attached to a run-history row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRunValidationOutcomeClass {
    /// Validation was not run.
    ValidationNotRun,
    /// Validation passed cleanly.
    ValidationPassed,
    /// Validation passed with warnings.
    ValidationPassedWithWarnings,
    /// Validation failed.
    ValidationFailed,
    /// Validation completed only partially and is not reproducible.
    ValidationPartialUnreproducible,
}

impl AiRunValidationOutcomeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ValidationNotRun => "validation_not_run",
            Self::ValidationPassed => "validation_passed",
            Self::ValidationPassedWithWarnings => "validation_passed_with_warnings",
            Self::ValidationFailed => "validation_failed",
            Self::ValidationPartialUnreproducible => "validation_partial_unreproducible",
        }
    }
}

/// One approval/deny event preserved on the run history row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalTimelineEvent {
    /// Stable event id.
    pub event_id: String,
    /// Opaque approval ticket ref.
    pub approval_ticket_ref: String,
    /// Approval decision.
    pub decision_class: ApprovalEventDecisionClass,
    /// Approval scope.
    pub scope_class: ApprovalScopeClass,
    /// Actor that decided this event.
    pub actor_class: ApprovalEventActorClass,
    /// Opaque actor identity ref.
    pub actor_identity_ref: String,
    /// Object class the approval covers.
    pub object_class: ApprovalObjectClass,
    /// Opaque object identity ref.
    pub object_identity_ref: String,
    /// Policy epoch the decision resolved against.
    pub policy_epoch_ref: String,
    /// Expiry timestamp when one is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Reviewer-visible revocation or expiry note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revocation_note: Option<String>,
    /// Decision timestamp or pending timestamp.
    pub decided_at: String,
    /// Reviewer-visible summary label.
    pub summary_label: String,
}

impl ApprovalTimelineEvent {
    /// True when this event admits the run that owns it (after no later
    /// event revokes, expires, or rejects the approval).
    pub fn admits_run(&self) -> bool {
        self.decision_class.admits_run()
    }
}

/// Evidence lineage refs preserved on the run-history row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRunEvidenceLineage {
    /// Evidence packet ref.
    pub evidence_packet_ref: String,
    /// Routing packet ref.
    pub routing_packet_ref: String,
    /// Spend receipt ref.
    pub spend_receipt_ref: String,
    /// Provider-route receipt ref.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub route_receipt_ref: String,
    /// Evidence replay packet ref.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub replay_packet_ref: String,
    /// Mutation journal ref when the run applied a change.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub mutation_journal_ref: String,
    /// Rollback checkpoint ref when the run applied a change.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub rollback_checkpoint_ref: String,
    /// Produced artifact refs.
    #[serde(default)]
    pub produced_artifact_refs: Vec<String>,
    /// Diff artifact refs.
    #[serde(default)]
    pub diff_artifact_refs: Vec<String>,
    /// Validation summary refs.
    #[serde(default)]
    pub validation_summary_refs: Vec<String>,
    /// Validation outcome rollup.
    pub validation_outcome_class: AiRunValidationOutcomeClass,
    /// Running build identity ref.
    pub running_build_identity_ref: String,
}

/// Composer thread lineage refs preserved on the run-history row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRunThreadLineage {
    /// Composer session ref.
    pub composer_session_ref: String,
    /// Turn draft ref.
    pub turn_draft_ref: String,
    /// Request workspace ref.
    pub request_workspace_ref: String,
    /// Context assembly ref when one exists.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub assembly_ref: String,
}

/// Action refs the row offers to the reviewer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRunHistoryActions {
    /// Open details action ref.
    pub open_details_action_ref: String,
    /// Open evidence packet action ref.
    pub open_evidence_packet_action_ref: String,
    /// Open AI thread action ref.
    pub open_thread_action_ref: String,
    /// Open replay action ref.
    pub open_replay_action_ref: String,
    /// Open support export action ref.
    pub open_support_export_action_ref: String,
    /// Open rerun review action ref when applicable.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub open_rerun_review_action_ref: String,
    /// Open-as-recipe action ref when applicable.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub open_as_recipe_action_ref: String,
    /// Share action ref when applicable.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub share_action_ref: String,
    /// Export action ref when applicable.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub export_action_ref: String,
}

/// One canonical run-history row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRunHistoryEntry {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Canonical run id. Stable across AI thread, evidence packet,
    /// support packet, and replay view.
    pub canonical_run_id: String,
    /// Reviewer-visible task label.
    pub task_label: String,
    /// Actor class that issued the run.
    pub actor_class: AiRunActorClass,
    /// Opaque actor identity ref.
    pub actor_identity_ref: String,
    /// Provider registry entry ref.
    pub provider_entry_ref: String,
    /// Model registry entry ref.
    pub model_entry_ref: String,
    /// Provider label.
    pub provider_label: String,
    /// Model label.
    pub model_label: String,
    /// Execution boundary class observed at run.
    pub execution_boundary_class: AiRunExecutionBoundaryClass,
    /// Reviewer-visible boundary label.
    pub boundary_label: String,
    /// Outcome class observed at run.
    pub outcome_class: AiRunOutcomeClass,
    /// Reviewer-visible outcome summary label.
    pub outcome_summary_label: String,
    /// Cost band.
    pub cost_band_class: AiRunCostBandClass,
    /// Quota band.
    pub quota_band_class: AiRunQuotaBandClass,
    /// Ordered approval timeline events.
    pub approval_timeline: Vec<ApprovalTimelineEvent>,
    /// Evidence lineage refs.
    pub evidence_lineage: AiRunEvidenceLineage,
    /// Composer thread lineage refs.
    pub thread_lineage: AiRunThreadLineage,
    /// Run lifecycle state.
    pub history_state_class: AiRunHistoryStateClass,
    /// Evidence completeness class.
    pub evidence_completeness_class: EvidenceCompletenessClass,
    /// Typed incompleteness reason when completeness is degraded or blocked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_incompleteness_reason_class: Option<EvidenceIncompletenessReasonClass>,
    /// Reviewer-visible incompleteness note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_incompleteness_note: Option<String>,
    /// Tool-call lineage refs.
    #[serde(default)]
    pub tool_call_lineage_refs: Vec<String>,
    /// Rerun review refs minted for this run.
    #[serde(default)]
    pub rerun_review_refs: Vec<String>,
    /// Reviewer-visible action refs.
    pub actions: AiRunHistoryActions,
    /// Policy context.
    pub policy_context: RoutingPolicyContext,
    /// Redaction class.
    pub redaction_class: AiRunHistoryRedactionClass,
    /// Mint timestamp.
    pub minted_at: String,
    /// Terminal timestamp for applied/rejected/cancelled rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

impl AiRunHistoryEntry {
    /// True when this row preserves a granted approval that admits the run.
    pub fn has_granted_approval(&self) -> bool {
        self.approval_timeline.iter().any(|event| event.admits_run())
    }

    /// True when this row records an approval that was revoked or expired.
    pub fn has_revoked_or_expired_approval(&self) -> bool {
        self.approval_timeline.iter().any(|event| {
            matches!(
                event.decision_class,
                ApprovalEventDecisionClass::Revoked | ApprovalEventDecisionClass::Expired
            )
        })
    }
}

/// One drift comparison row on a rerun review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunDriftRow {
    /// Axis class the row compares.
    pub axis_class: RerunDriftAxisClass,
    /// Drift class observed on this axis.
    pub drift_class: RerunDriftClass,
    /// Opaque ref to the original identity on this axis.
    pub original_identity_ref: String,
    /// Opaque ref to the current identity on this axis.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub current_identity_ref: String,
    /// Original-state label.
    pub original_label: String,
    /// Current-state label.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub current_label: String,
    /// Summary label shown verbatim.
    pub summary_label: String,
}

/// Axis class for a rerun drift comparison row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunDriftAxisClass {
    /// Workspace revision drift.
    WorkspaceRevision,
    /// Policy epoch drift.
    PolicyEpoch,
    /// Provider lifecycle drift.
    ProviderLifecycle,
    /// Model lifecycle drift.
    ModelLifecycle,
    /// Tool availability drift.
    ToolAvailability,
    /// Approval validity drift.
    ApprovalValidity,
    /// Connector authority drift.
    ConnectorAuthority,
    /// Execution boundary drift.
    ExecutionBoundary,
    /// Running build identity drift.
    RunningBuildIdentity,
}

impl RerunDriftAxisClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceRevision => "workspace_revision",
            Self::PolicyEpoch => "policy_epoch",
            Self::ProviderLifecycle => "provider_lifecycle",
            Self::ModelLifecycle => "model_lifecycle",
            Self::ToolAvailability => "tool_availability",
            Self::ApprovalValidity => "approval_validity",
            Self::ConnectorAuthority => "connector_authority",
            Self::ExecutionBoundary => "execution_boundary",
            Self::RunningBuildIdentity => "running_build_identity",
        }
    }
}

/// Drift class observed on one axis of a rerun review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunDriftClass {
    /// No drift on this axis.
    NoDrift,
    /// Minor drift.
    MinorDrift,
    /// Material drift that affects rerun fidelity.
    MaterialDrift,
    /// Original identity was removed or withdrawn.
    RemovedOrWithdrawn,
}

impl RerunDriftClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDrift => "no_drift",
            Self::MinorDrift => "minor_drift",
            Self::MaterialDrift => "material_drift",
            Self::RemovedOrWithdrawn => "removed_or_withdrawn",
        }
    }

    /// True when this drift class blocks Rerun by itself.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::MaterialDrift | Self::RemovedOrWithdrawn)
    }
}

/// Re-resolved approval requirement summary on a rerun review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunApprovalResolution {
    /// Resolution class summary.
    pub resolution_class: RerunApprovalResolutionClass,
    /// Approval refs the rerun would need.
    pub required_approval_refs: Vec<String>,
    /// Approval refs freshly resolved against the current policy epoch.
    pub freshly_resolved_approval_refs: Vec<String>,
    /// Approval refs that are missing.
    #[serde(default)]
    pub missing_approval_refs: Vec<String>,
    /// Approval refs that have expired.
    #[serde(default)]
    pub expired_approval_refs: Vec<String>,
    /// Reviewer-visible summary label.
    pub summary_label: String,
}

/// Resolution class for approvals on a rerun review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunApprovalResolutionClass {
    /// All required approvals are freshly resolved.
    AllRequiredFreshlyResolved,
    /// A required approval is missing.
    MissingRequiredApproval,
    /// A required approval has expired.
    ExpiredRequiredApproval,
    /// Required approval is blocked by policy.
    BlockedByPolicy,
    /// No approvals are required for the rerun.
    NotApplicableNoApprovalsRequired,
}

impl RerunApprovalResolutionClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllRequiredFreshlyResolved => "all_required_freshly_resolved",
            Self::MissingRequiredApproval => "missing_required_approval",
            Self::ExpiredRequiredApproval => "expired_required_approval",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::NotApplicableNoApprovalsRequired => "not_applicable_no_approvals_required",
        }
    }

    /// True when this resolution class admits Rerun.
    pub const fn admits_rerun(self) -> bool {
        matches!(
            self,
            Self::AllRequiredFreshlyResolved | Self::NotApplicableNoApprovalsRequired
        )
    }
}

/// Action offer class on a rerun review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunActionOffer {
    /// Rerun the original run.
    Rerun,
    /// Cancel without rerunning.
    Cancel,
    /// Open the original as a recipe instead of rerunning.
    OpenAsRecipe,
    /// Open the original evidence packet.
    OpenOriginalEvidence,
    /// Open the original AI thread.
    OpenOriginalThread,
    /// Request approval renewal before rerunning.
    RequestApprovalRenewal,
}

impl RerunActionOffer {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rerun => "rerun",
            Self::Cancel => "cancel",
            Self::OpenAsRecipe => "open_as_recipe",
            Self::OpenOriginalEvidence => "open_original_evidence",
            Self::OpenOriginalThread => "open_original_thread",
            Self::RequestApprovalRenewal => "request_approval_renewal",
        }
    }
}

/// Admission class for the Rerun action on a rerun review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunAdmissionClass {
    /// Rerun is admitted.
    AdmitRerun,
    /// Rerun denied due to blocking drift.
    DenyRerunDriftBlocking,
    /// Rerun denied because approval is unresolved.
    DenyRerunApprovalUnresolved,
    /// Rerun denied because the provider is unavailable.
    DenyRerunProviderUnavailable,
    /// Rerun denied because the tool is unavailable.
    DenyRerunToolUnavailable,
    /// Rerun denied by policy.
    DenyRerunPolicyBlocked,
    /// Rerun denied because the evidence is incomplete.
    DenyRerunEvidenceIncomplete,
}

impl RerunAdmissionClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdmitRerun => "admit_rerun",
            Self::DenyRerunDriftBlocking => "deny_rerun_drift_blocking",
            Self::DenyRerunApprovalUnresolved => "deny_rerun_approval_unresolved",
            Self::DenyRerunProviderUnavailable => "deny_rerun_provider_unavailable",
            Self::DenyRerunToolUnavailable => "deny_rerun_tool_unavailable",
            Self::DenyRerunPolicyBlocked => "deny_rerun_policy_blocked",
            Self::DenyRerunEvidenceIncomplete => "deny_rerun_evidence_incomplete",
        }
    }

    /// True when this admission class allows Rerun.
    pub const fn admits_rerun(self) -> bool {
        matches!(self, Self::AdmitRerun)
    }
}

/// Typed reason class for a Rerun denial.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunDeniedReasonClass {
    /// Material drift was present on one or more axes.
    MaterialDriftPresent,
    /// A required approval is missing.
    ApprovalMissing,
    /// A required approval has expired.
    ApprovalExpired,
    /// The provider was withdrawn from the registry.
    ProviderWithdrawn,
    /// The tool descriptor was withdrawn.
    ToolWithdrawn,
    /// Rerun is blocked by policy.
    PolicyBlocked,
    /// The evidence is incomplete.
    EvidenceIncomplete,
    /// The workspace revision is no longer available.
    WorkspaceRevisionUnavailable,
}

impl RerunDeniedReasonClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MaterialDriftPresent => "material_drift_present",
            Self::ApprovalMissing => "approval_missing",
            Self::ApprovalExpired => "approval_expired",
            Self::ProviderWithdrawn => "provider_withdrawn",
            Self::ToolWithdrawn => "tool_withdrawn",
            Self::PolicyBlocked => "policy_blocked",
            Self::EvidenceIncomplete => "evidence_incomplete",
            Self::WorkspaceRevisionUnavailable => "workspace_revision_unavailable",
        }
    }
}

/// One rerun review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRerunReview {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable rerun review id.
    pub rerun_review_id: String,
    /// Canonical run id of the original run.
    pub canonical_run_id: String,
    /// Run-history entry ref the rerun review compares against.
    pub original_run_entry_ref: String,
    /// Typed drift rows.
    pub drift_rows: Vec<RerunDriftRow>,
    /// Re-resolved approval requirement summary.
    pub approval_resolution: RerunApprovalResolution,
    /// Ordered set of action offers shown to the reviewer.
    pub action_offers: Vec<RerunActionOffer>,
    /// Rerun admission class.
    pub rerun_admission_class: RerunAdmissionClass,
    /// Typed denial reason when Rerun is denied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerun_denied_reason_class: Option<RerunDeniedReasonClass>,
    /// Reviewer-visible denial note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerun_denied_reason_note: Option<String>,
    /// Policy context.
    pub policy_context: RoutingPolicyContext,
    /// Redaction class.
    pub redaction_class: AiRunHistoryRedactionClass,
    /// Mint timestamp.
    pub minted_at: String,
}

impl AiRerunReview {
    /// True when any drift row has a blocking drift class.
    pub fn has_blocking_drift(&self) -> bool {
        self.drift_rows
            .iter()
            .any(|row| row.drift_class.is_blocking())
    }

    /// True when the rerun review admits the Rerun action.
    pub fn admits_rerun(&self) -> bool {
        self.rerun_admission_class.admits_rerun()
    }
}

/// Surface that must read the same run-history truth as UI, evidence, support, or replay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRunHistorySurfaceClass {
    /// AI thread or composer history list.
    AiThread,
    /// Evidence panel / review workspace.
    EvidencePanel,
    /// Support export packet.
    SupportPacket,
    /// Replay view.
    ReplayView,
    /// CLI or headless audit.
    Cli,
}

impl AiRunHistorySurfaceClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiThread => "ai_thread",
            Self::EvidencePanel => "evidence_panel",
            Self::SupportPacket => "support_packet",
            Self::ReplayView => "replay_view",
            Self::Cli => "cli",
        }
    }
}

/// Required surfaces that MUST share canonical run ids and rerun-review refs.
const REQUIRED_SURFACE_CLASSES: &[AiRunHistorySurfaceClass] = &[
    AiRunHistorySurfaceClass::AiThread,
    AiRunHistorySurfaceClass::EvidencePanel,
    AiRunHistorySurfaceClass::SupportPacket,
    AiRunHistorySurfaceClass::ReplayView,
];

/// One surface's parity proof that it reads the same canonical run ids and rerun reviews.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRunHistorySurfaceRow {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Surface class.
    pub surface_class: AiRunHistorySurfaceClass,
    /// Stable projection ref for this surface.
    pub projection_ref: String,
    /// Canonical run ids the surface renders.
    pub canonical_run_ids: Vec<String>,
    /// Rerun-review refs the surface renders.
    pub rerun_review_refs: Vec<String>,
    /// True when the surface preserves canonical run id.
    pub preserves_canonical_run_id: bool,
    /// True when the surface preserves the full approval timeline.
    pub preserves_approval_timeline: bool,
    /// True when the surface preserves evidence-incomplete truth verbatim.
    pub preserves_evidence_incompleteness: bool,
    /// True when the surface excludes raw private material.
    pub raw_private_material_excluded: bool,
    /// True when the surface supports a deterministic JSON export.
    pub supports_json_export: bool,
}

impl AiRunHistorySurfaceRow {
    /// Builds a surface row preserving the required parity invariants.
    pub fn new(
        surface_class: AiRunHistorySurfaceClass,
        projection_ref: impl Into<String>,
        canonical_run_ids: Vec<String>,
        rerun_review_refs: Vec<String>,
    ) -> Self {
        Self {
            record_kind: AI_RUN_HISTORY_SURFACE_ROW_RECORD_KIND.to_owned(),
            schema_version: AI_RUN_HISTORY_SCHEMA_VERSION,
            surface_class,
            projection_ref: projection_ref.into(),
            canonical_run_ids,
            rerun_review_refs,
            preserves_canonical_run_id: true,
            preserves_approval_timeline: true,
            preserves_evidence_incompleteness: true,
            raw_private_material_excluded: true,
            supports_json_export: true,
        }
    }
}

/// Inputs accepted by [`AiRunHistoryParityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiRunHistoryParityPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer-visible label.
    pub display_label: String,
    /// Run-history entries covered by the packet.
    pub entries: Vec<AiRunHistoryEntry>,
    /// Rerun-review records covered by the packet.
    pub rerun_reviews: Vec<AiRerunReview>,
    /// Surface parity rows covered by the packet.
    pub surface_rows: Vec<AiRunHistorySurfaceRow>,
    /// Source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Policy context.
    pub policy_context: RoutingPolicyContext,
    /// Mint timestamp.
    pub minted_at: String,
}

/// Packet that aggregates run-history entries, rerun reviews, and surface parity rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRunHistoryParityPacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer-visible label.
    pub display_label: String,
    /// Run-history entries.
    pub entries: Vec<AiRunHistoryEntry>,
    /// Rerun reviews.
    pub rerun_reviews: Vec<AiRerunReview>,
    /// Surface parity rows.
    pub surface_rows: Vec<AiRunHistorySurfaceRow>,
    /// Source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Policy context.
    pub policy_context: RoutingPolicyContext,
    /// Mint timestamp.
    pub minted_at: String,
}

impl AiRunHistoryParityPacket {
    /// Builds a parity packet from canonical inputs.
    pub fn new(input: AiRunHistoryParityPacketInput) -> Self {
        Self {
            record_kind: AI_RUN_HISTORY_PARITY_PACKET_RECORD_KIND.to_owned(),
            schema_version: AI_RUN_HISTORY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            display_label: input.display_label,
            entries: input.entries,
            rerun_reviews: input.rerun_reviews,
            surface_rows: input.surface_rows,
            source_contract_refs: input.source_contract_refs,
            policy_context: input.policy_context,
            minted_at: input.minted_at,
        }
    }

    /// Validate the parity invariants without resolving raw boundary material.
    pub fn validate(&self) -> Vec<AiRunHistoryViolation> {
        let mut violations: Vec<AiRunHistoryViolation> = Vec::new();

        if self.record_kind != AI_RUN_HISTORY_PARITY_PACKET_RECORD_KIND {
            violations.push(AiRunHistoryViolation::WrongRecordKind);
        }
        if self.schema_version != AI_RUN_HISTORY_SCHEMA_VERSION {
            violations.push(AiRunHistoryViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.minted_at.trim().is_empty()
            || self.policy_context.policy_epoch_ref.trim().is_empty()
        {
            violations.push(AiRunHistoryViolation::MissingIdentity);
        }

        for required in [
            AI_RUN_HISTORY_ENTRY_SCHEMA_REF,
            AI_RERUN_REVIEW_SCHEMA_REF,
            AI_RUN_HISTORY_PARITY_ARTIFACT_REF,
        ] {
            if !self
                .source_contract_refs
                .iter()
                .any(|reference| reference == required)
            {
                violations.push(AiRunHistoryViolation::MissingSourceContracts);
                break;
            }
        }

        if self.entries.is_empty() {
            violations.push(AiRunHistoryViolation::MissingEntries);
        }
        if self.rerun_reviews.is_empty() {
            violations.push(AiRunHistoryViolation::MissingRerunReviews);
        }

        validate_entries(self, &mut violations);
        validate_rerun_reviews(self, &mut violations);
        validate_surface_rows(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("run history packet serializes"),
        ) {
            violations.push(AiRunHistoryViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON for the parity packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("run history packet serializes")
    }

    /// Project an export-safe support packet from the parity packet.
    pub fn support_packet(&self) -> AiRunHistorySupportPacket {
        AiRunHistorySupportPacket {
            record_kind: AI_RUN_HISTORY_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: AI_RUN_HISTORY_SCHEMA_VERSION,
            support_packet_id: format!("support-export:ai-run-history:{}", self.packet_id),
            parity_packet_ref: self.packet_id.clone(),
            entry_rows: self
                .entries
                .iter()
                .map(AiRunHistorySupportEntryRow::from_entry)
                .collect(),
            rerun_review_rows: self
                .rerun_reviews
                .iter()
                .map(AiRunHistorySupportRerunRow::from_rerun)
                .collect(),
            surface_projection_refs: self
                .surface_rows
                .iter()
                .map(|row| row.projection_ref.clone())
                .collect(),
            source_contract_refs: self.source_contract_refs.clone(),
            policy_epoch_ref: self.policy_context.policy_epoch_ref.clone(),
            minted_at: self.minted_at.clone(),
        }
    }

    /// Deterministic Markdown summary used as the parity report body.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# AI Run History Parity Report\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Run history entries: {}\n", self.entries.len()));
        out.push_str(&format!("- Rerun reviews: {}\n", self.rerun_reviews.len()));
        out.push_str(&format!(
            "- Surface parity rows: {}\n",
            self.surface_rows.len()
        ));

        out.push_str("\n## Run outcome coverage\n");
        let mut outcomes: BTreeSet<&str> = BTreeSet::new();
        for entry in &self.entries {
            outcomes.insert(entry.outcome_class.as_str());
        }
        for outcome in &outcomes {
            out.push_str(&format!("- `{outcome}`\n"));
        }

        out.push_str("\n## Approval decision coverage\n");
        let mut decisions: BTreeSet<&str> = BTreeSet::new();
        for entry in &self.entries {
            for event in &entry.approval_timeline {
                decisions.insert(event.decision_class.as_str());
            }
        }
        for decision in &decisions {
            out.push_str(&format!("- `{decision}`\n"));
        }

        out.push_str("\n## Evidence completeness coverage\n");
        let mut completeness: BTreeSet<&str> = BTreeSet::new();
        for entry in &self.entries {
            completeness.insert(entry.evidence_completeness_class.as_str());
        }
        for class in &completeness {
            out.push_str(&format!("- `{class}`\n"));
        }

        out.push_str("\n## Rerun admission coverage\n");
        let mut admissions: BTreeSet<&str> = BTreeSet::new();
        for rerun in &self.rerun_reviews {
            admissions.insert(rerun.rerun_admission_class.as_str());
        }
        for admission in &admissions {
            out.push_str(&format!("- `{admission}`\n"));
        }

        out.push_str("\n## Surface parity coverage\n");
        let mut surfaces: BTreeSet<&str> = BTreeSet::new();
        for row in &self.surface_rows {
            surfaces.insert(row.surface_class.as_str());
        }
        for surface in &surfaces {
            out.push_str(&format!("- `{surface}`\n"));
        }

        out
    }

    /// Canonical run ids present on the packet.
    pub fn canonical_run_ids(&self) -> Vec<&str> {
        self.entries
            .iter()
            .map(|entry| entry.canonical_run_id.as_str())
            .collect()
    }

    /// Rerun review ids present on the packet.
    pub fn rerun_review_ids(&self) -> Vec<&str> {
        self.rerun_reviews
            .iter()
            .map(|rerun| rerun.rerun_review_id.as_str())
            .collect()
    }
}

/// Export-safe support packet projection of [`AiRunHistoryParityPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRunHistorySupportPacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable support packet id.
    pub support_packet_id: String,
    /// Parity packet ref.
    pub parity_packet_ref: String,
    /// Support rows projected from run-history entries.
    pub entry_rows: Vec<AiRunHistorySupportEntryRow>,
    /// Support rows projected from rerun reviews.
    pub rerun_review_rows: Vec<AiRunHistorySupportRerunRow>,
    /// Surface projection refs preserved on the support packet.
    pub surface_projection_refs: Vec<String>,
    /// Source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Policy epoch ref.
    pub policy_epoch_ref: String,
    /// Mint timestamp.
    pub minted_at: String,
}

impl AiRunHistorySupportPacket {
    /// Deterministic export-safe JSON for the support packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only support packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("run history support packet serializes")
    }
}

/// One support-row projection of a run-history entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRunHistorySupportEntryRow {
    /// Canonical run id.
    pub canonical_run_id: String,
    /// Outcome token.
    pub outcome_token: String,
    /// History-state token.
    pub history_state_token: String,
    /// Provider entry ref.
    pub provider_entry_ref: String,
    /// Model entry ref.
    pub model_entry_ref: String,
    /// Execution boundary token.
    pub execution_boundary_token: String,
    /// Cost band token.
    pub cost_band_token: String,
    /// Quota band token.
    pub quota_band_token: String,
    /// Evidence packet ref.
    pub evidence_packet_ref: String,
    /// Routing packet ref.
    pub routing_packet_ref: String,
    /// Spend receipt ref.
    pub spend_receipt_ref: String,
    /// Mutation journal ref when one exists.
    pub mutation_journal_ref: String,
    /// Approval decision tokens preserved verbatim.
    pub approval_decision_tokens: Vec<String>,
    /// Approval ticket refs preserved verbatim.
    pub approval_ticket_refs: Vec<String>,
    /// Evidence completeness token.
    pub evidence_completeness_token: String,
    /// Evidence incompleteness reason token when set.
    pub evidence_incompleteness_reason_token: String,
    /// Rerun review refs.
    pub rerun_review_refs: Vec<String>,
    /// Redaction class token.
    pub redaction_class_token: String,
    /// Mint timestamp.
    pub minted_at: String,
    /// Completed at timestamp, when set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

impl AiRunHistorySupportEntryRow {
    /// Project a support row from one canonical entry.
    pub fn from_entry(entry: &AiRunHistoryEntry) -> Self {
        Self {
            canonical_run_id: entry.canonical_run_id.clone(),
            outcome_token: entry.outcome_class.as_str().to_owned(),
            history_state_token: entry.history_state_class.as_str().to_owned(),
            provider_entry_ref: entry.provider_entry_ref.clone(),
            model_entry_ref: entry.model_entry_ref.clone(),
            execution_boundary_token: entry.execution_boundary_class.as_str().to_owned(),
            cost_band_token: entry.cost_band_class.as_str().to_owned(),
            quota_band_token: entry.quota_band_class.as_str().to_owned(),
            evidence_packet_ref: entry.evidence_lineage.evidence_packet_ref.clone(),
            routing_packet_ref: entry.evidence_lineage.routing_packet_ref.clone(),
            spend_receipt_ref: entry.evidence_lineage.spend_receipt_ref.clone(),
            mutation_journal_ref: entry.evidence_lineage.mutation_journal_ref.clone(),
            approval_decision_tokens: entry
                .approval_timeline
                .iter()
                .map(|event| event.decision_class.as_str().to_owned())
                .collect(),
            approval_ticket_refs: entry
                .approval_timeline
                .iter()
                .map(|event| event.approval_ticket_ref.clone())
                .collect(),
            evidence_completeness_token: entry.evidence_completeness_class.as_str().to_owned(),
            evidence_incompleteness_reason_token: entry
                .evidence_incompleteness_reason_class
                .map(|reason| reason.as_str().to_owned())
                .unwrap_or_default(),
            rerun_review_refs: entry.rerun_review_refs.clone(),
            redaction_class_token: entry.redaction_class.as_str().to_owned(),
            minted_at: entry.minted_at.clone(),
            completed_at: entry.completed_at.clone(),
        }
    }
}

/// One support-row projection of a rerun review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRunHistorySupportRerunRow {
    /// Rerun review id.
    pub rerun_review_id: String,
    /// Canonical run id of the original run.
    pub canonical_run_id: String,
    /// Run-history entry ref the rerun review compares against.
    pub original_run_entry_ref: String,
    /// Rerun admission token.
    pub rerun_admission_token: String,
    /// Rerun denied reason token when set.
    pub rerun_denied_reason_token: String,
    /// Drift axis -> drift class tokens.
    pub drift_summary_tokens: Vec<String>,
    /// Approval resolution token.
    pub approval_resolution_token: String,
    /// Action offer tokens preserved verbatim.
    pub action_offer_tokens: Vec<String>,
    /// Mint timestamp.
    pub minted_at: String,
}

impl AiRunHistorySupportRerunRow {
    /// Project a support row from one rerun review.
    pub fn from_rerun(rerun: &AiRerunReview) -> Self {
        Self {
            rerun_review_id: rerun.rerun_review_id.clone(),
            canonical_run_id: rerun.canonical_run_id.clone(),
            original_run_entry_ref: rerun.original_run_entry_ref.clone(),
            rerun_admission_token: rerun.rerun_admission_class.as_str().to_owned(),
            rerun_denied_reason_token: rerun
                .rerun_denied_reason_class
                .map(|reason| reason.as_str().to_owned())
                .unwrap_or_default(),
            drift_summary_tokens: rerun
                .drift_rows
                .iter()
                .map(|row| format!("{}:{}", row.axis_class.as_str(), row.drift_class.as_str()))
                .collect(),
            approval_resolution_token: rerun.approval_resolution.resolution_class.as_str().to_owned(),
            action_offer_tokens: rerun
                .action_offers
                .iter()
                .map(|offer| offer.as_str().to_owned())
                .collect(),
            minted_at: rerun.minted_at.clone(),
        }
    }
}

/// Validation failures emitted by [`AiRunHistoryParityPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AiRunHistoryViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity fields are missing.
    MissingIdentity,
    /// Source contract refs are missing.
    MissingSourceContracts,
    /// Run-history entries are missing.
    MissingEntries,
    /// Rerun reviews are missing.
    MissingRerunReviews,
    /// Run-history entry identity fields are missing.
    EntryMissingIdentity,
    /// Run-history entry has no approval timeline events.
    EntryMissingApprovalTimeline,
    /// Approval timeline event identity fields are missing.
    ApprovalEventMissingIdentity,
    /// Approval timeline event records a revocation but no reviewer note.
    ApprovalEventRevokedWithoutNote,
    /// Run-history entry records incompleteness but no typed reason or note.
    EntryIncompleteWithoutReason,
    /// Terminal entry is missing a completed_at timestamp.
    EntryTerminalWithoutCompletedAt,
    /// Active entry records a completed_at timestamp.
    EntryActiveWithCompletedAt,
    /// Run-history entry must carry a granted approval event for outcome rows that
    /// landed a material side effect, but no granted event is present.
    EntryAppliedWithoutGrantedApproval,
    /// Rerun review identity fields are missing.
    RerunMissingIdentity,
    /// Rerun review references an entry that does not exist on the packet.
    RerunReferencesUnknownEntry,
    /// Rerun review and original entry disagree on canonical_run_id.
    RerunMismatchedCanonicalRunId,
    /// Rerun review denial is missing a typed reason or reviewer note.
    RerunDenialWithoutReason,
    /// Rerun review admits Rerun while a drift row is blocking.
    RerunAdmitWithBlockingDrift,
    /// Rerun review admits Rerun while approval is unresolved.
    RerunAdmitWithUnresolvedApproval,
    /// Rerun review missing an open-as-recipe offer.
    RerunMissingOpenAsRecipeOffer,
    /// Rerun review missing one of the required drift axes.
    RerunMissingRequiredDriftAxis,
    /// Required surface parity row is missing.
    MissingSurfaceProjection,
    /// Surface projection drifts from canonical refs.
    SurfaceProjectionDrift,
    /// Surface projection drops the canonical run id of an entry.
    SurfaceMissingCanonicalRunId,
    /// Surface projection drops a rerun review ref present on the packet.
    SurfaceMissingRerunReviewRef,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl AiRunHistoryViolation {
    /// Stable token for tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::MissingEntries => "missing_entries",
            Self::MissingRerunReviews => "missing_rerun_reviews",
            Self::EntryMissingIdentity => "entry_missing_identity",
            Self::EntryMissingApprovalTimeline => "entry_missing_approval_timeline",
            Self::ApprovalEventMissingIdentity => "approval_event_missing_identity",
            Self::ApprovalEventRevokedWithoutNote => "approval_event_revoked_without_note",
            Self::EntryIncompleteWithoutReason => "entry_incomplete_without_reason",
            Self::EntryTerminalWithoutCompletedAt => "entry_terminal_without_completed_at",
            Self::EntryActiveWithCompletedAt => "entry_active_with_completed_at",
            Self::EntryAppliedWithoutGrantedApproval => "entry_applied_without_granted_approval",
            Self::RerunMissingIdentity => "rerun_missing_identity",
            Self::RerunReferencesUnknownEntry => "rerun_references_unknown_entry",
            Self::RerunMismatchedCanonicalRunId => "rerun_mismatched_canonical_run_id",
            Self::RerunDenialWithoutReason => "rerun_denial_without_reason",
            Self::RerunAdmitWithBlockingDrift => "rerun_admit_with_blocking_drift",
            Self::RerunAdmitWithUnresolvedApproval => "rerun_admit_with_unresolved_approval",
            Self::RerunMissingOpenAsRecipeOffer => "rerun_missing_open_as_recipe_offer",
            Self::RerunMissingRequiredDriftAxis => "rerun_missing_required_drift_axis",
            Self::MissingSurfaceProjection => "missing_surface_projection",
            Self::SurfaceProjectionDrift => "surface_projection_drift",
            Self::SurfaceMissingCanonicalRunId => "surface_missing_canonical_run_id",
            Self::SurfaceMissingRerunReviewRef => "surface_missing_rerun_review_ref",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Errors emitted when reading the checked-in run-history parity packet.
#[derive(Debug)]
pub enum AiRunHistoryArtifactError {
    /// Parity packet failed to parse.
    Packet(serde_json::Error),
    /// Parity packet failed validation.
    Validation(Vec<AiRunHistoryViolation>),
}

impl fmt::Display for AiRunHistoryArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(formatter, "ai run history packet parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "ai run history packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for AiRunHistoryArtifactError {}

/// Returns the checked-in run-history parity packet.
///
/// # Errors
///
/// Returns [`AiRunHistoryArtifactError`] when the checked-in fixture cannot be
/// parsed or fails validation.
pub fn current_beta_ai_run_history_parity_packet(
) -> Result<AiRunHistoryParityPacket, AiRunHistoryArtifactError> {
    let packet: AiRunHistoryParityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m3/run_history_and_replay/ai_run_history_parity_packet.json"
    )))
    .map_err(AiRunHistoryArtifactError::Packet)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AiRunHistoryArtifactError::Validation(violations))
    }
}

const REQUIRED_DRIFT_AXES: &[RerunDriftAxisClass] = &[
    RerunDriftAxisClass::WorkspaceRevision,
    RerunDriftAxisClass::PolicyEpoch,
    RerunDriftAxisClass::ProviderLifecycle,
    RerunDriftAxisClass::ModelLifecycle,
    RerunDriftAxisClass::ToolAvailability,
];

fn validate_entries(
    packet: &AiRunHistoryParityPacket,
    violations: &mut Vec<AiRunHistoryViolation>,
) {
    let mut seen_canonical_run_ids: BTreeSet<&str> = BTreeSet::new();
    for entry in &packet.entries {
        if entry.record_kind != AI_RUN_HISTORY_ENTRY_RECORD_KIND
            || entry.schema_version != AI_RUN_HISTORY_SCHEMA_VERSION
            || entry.canonical_run_id.trim().is_empty()
            || entry.task_label.trim().is_empty()
            || entry.actor_identity_ref.trim().is_empty()
            || entry.provider_entry_ref.trim().is_empty()
            || entry.model_entry_ref.trim().is_empty()
            || entry.provider_label.trim().is_empty()
            || entry.model_label.trim().is_empty()
            || entry.boundary_label.trim().is_empty()
            || entry.outcome_summary_label.trim().is_empty()
            || entry.minted_at.trim().is_empty()
            || entry.policy_context.policy_epoch_ref.trim().is_empty()
            || entry.evidence_lineage.evidence_packet_ref.trim().is_empty()
            || entry.evidence_lineage.routing_packet_ref.trim().is_empty()
            || entry.evidence_lineage.spend_receipt_ref.trim().is_empty()
            || entry
                .evidence_lineage
                .running_build_identity_ref
                .trim()
                .is_empty()
            || entry.thread_lineage.composer_session_ref.trim().is_empty()
            || entry.thread_lineage.turn_draft_ref.trim().is_empty()
            || entry.thread_lineage.request_workspace_ref.trim().is_empty()
            || entry.actions.open_details_action_ref.trim().is_empty()
            || entry
                .actions
                .open_evidence_packet_action_ref
                .trim()
                .is_empty()
            || entry.actions.open_thread_action_ref.trim().is_empty()
            || entry.actions.open_replay_action_ref.trim().is_empty()
            || entry
                .actions
                .open_support_export_action_ref
                .trim()
                .is_empty()
        {
            violations.push(AiRunHistoryViolation::EntryMissingIdentity);
            continue;
        }
        if !seen_canonical_run_ids.insert(entry.canonical_run_id.as_str()) {
            violations.push(AiRunHistoryViolation::EntryMissingIdentity);
            continue;
        }

        if entry.approval_timeline.is_empty() {
            violations.push(AiRunHistoryViolation::EntryMissingApprovalTimeline);
        }

        let mut seen_event_ids: BTreeSet<&str> = BTreeSet::new();
        for event in &entry.approval_timeline {
            if event.event_id.trim().is_empty()
                || event.approval_ticket_ref.trim().is_empty()
                || event.actor_identity_ref.trim().is_empty()
                || event.object_identity_ref.trim().is_empty()
                || event.policy_epoch_ref.trim().is_empty()
                || event.decided_at.trim().is_empty()
                || event.summary_label.trim().is_empty()
                || !seen_event_ids.insert(event.event_id.as_str())
            {
                violations.push(AiRunHistoryViolation::ApprovalEventMissingIdentity);
                continue;
            }
            if matches!(event.decision_class, ApprovalEventDecisionClass::Revoked)
                && event
                    .revocation_note
                    .as_deref()
                    .map(|note| note.trim().is_empty())
                    .unwrap_or(true)
            {
                violations.push(AiRunHistoryViolation::ApprovalEventRevokedWithoutNote);
            }
        }

        if entry.evidence_completeness_class.requires_reason() {
            let has_reason = entry.evidence_incompleteness_reason_class.is_some()
                && entry
                    .evidence_incompleteness_note
                    .as_deref()
                    .map(|note| !note.trim().is_empty())
                    .unwrap_or(false);
            if !has_reason {
                violations.push(AiRunHistoryViolation::EntryIncompleteWithoutReason);
            }
        }

        if entry.history_state_class.is_terminal() {
            if entry
                .completed_at
                .as_deref()
                .map(|value| value.trim().is_empty())
                .unwrap_or(true)
            {
                violations.push(AiRunHistoryViolation::EntryTerminalWithoutCompletedAt);
            }
        } else if entry.completed_at.is_some() {
            violations.push(AiRunHistoryViolation::EntryActiveWithCompletedAt);
        }

        if matches!(entry.history_state_class, AiRunHistoryStateClass::Applied)
            && !entry.has_granted_approval()
        {
            violations.push(AiRunHistoryViolation::EntryAppliedWithoutGrantedApproval);
        }
    }
}

fn validate_rerun_reviews(
    packet: &AiRunHistoryParityPacket,
    violations: &mut Vec<AiRunHistoryViolation>,
) {
    let canonical_run_ids: BTreeSet<&str> = packet
        .entries
        .iter()
        .map(|entry| entry.canonical_run_id.as_str())
        .collect();

    let mut seen_rerun_ids: BTreeSet<&str> = BTreeSet::new();
    for rerun in &packet.rerun_reviews {
        if rerun.record_kind != AI_RERUN_REVIEW_RECORD_KIND
            || rerun.schema_version != AI_RUN_HISTORY_SCHEMA_VERSION
            || rerun.rerun_review_id.trim().is_empty()
            || rerun.canonical_run_id.trim().is_empty()
            || rerun.original_run_entry_ref.trim().is_empty()
            || rerun.minted_at.trim().is_empty()
            || rerun.policy_context.policy_epoch_ref.trim().is_empty()
            || rerun.drift_rows.is_empty()
            || rerun.action_offers.is_empty()
            || !seen_rerun_ids.insert(rerun.rerun_review_id.as_str())
        {
            violations.push(AiRunHistoryViolation::RerunMissingIdentity);
            continue;
        }

        if !canonical_run_ids.contains(rerun.canonical_run_id.as_str()) {
            violations.push(AiRunHistoryViolation::RerunReferencesUnknownEntry);
        } else {
            let original = packet
                .entries
                .iter()
                .find(|entry| entry.canonical_run_id == rerun.canonical_run_id);
            if let Some(original) = original {
                if original.canonical_run_id != rerun.canonical_run_id {
                    violations.push(AiRunHistoryViolation::RerunMismatchedCanonicalRunId);
                }
            }
        }

        let present_axes: BTreeSet<RerunDriftAxisClass> = rerun
            .drift_rows
            .iter()
            .map(|row| row.axis_class)
            .collect();
        for required in REQUIRED_DRIFT_AXES {
            if !present_axes.contains(required) {
                violations.push(AiRunHistoryViolation::RerunMissingRequiredDriftAxis);
                break;
            }
        }

        if !rerun
            .action_offers
            .iter()
            .any(|offer| matches!(offer, RerunActionOffer::OpenAsRecipe))
        {
            violations.push(AiRunHistoryViolation::RerunMissingOpenAsRecipeOffer);
        }

        let blocking_drift = rerun.has_blocking_drift();
        let approval_admits = rerun.approval_resolution.resolution_class.admits_rerun();

        if rerun.admits_rerun() {
            if blocking_drift {
                violations.push(AiRunHistoryViolation::RerunAdmitWithBlockingDrift);
            }
            if !approval_admits {
                violations.push(AiRunHistoryViolation::RerunAdmitWithUnresolvedApproval);
            }
        } else {
            let has_reason = rerun.rerun_denied_reason_class.is_some()
                && rerun
                    .rerun_denied_reason_note
                    .as_deref()
                    .map(|note| !note.trim().is_empty())
                    .unwrap_or(false);
            if !has_reason {
                violations.push(AiRunHistoryViolation::RerunDenialWithoutReason);
            }
        }
    }
}

fn validate_surface_rows(
    packet: &AiRunHistoryParityPacket,
    violations: &mut Vec<AiRunHistoryViolation>,
) {
    let canonical_run_ids: BTreeSet<&str> = packet
        .entries
        .iter()
        .map(|entry| entry.canonical_run_id.as_str())
        .collect();
    let rerun_review_ids: BTreeSet<&str> = packet
        .rerun_reviews
        .iter()
        .map(|rerun| rerun.rerun_review_id.as_str())
        .collect();

    for required in REQUIRED_SURFACE_CLASSES {
        if !packet
            .surface_rows
            .iter()
            .any(|row| row.surface_class == *required)
        {
            violations.push(AiRunHistoryViolation::MissingSurfaceProjection);
            break;
        }
    }

    for row in &packet.surface_rows {
        if row.record_kind != AI_RUN_HISTORY_SURFACE_ROW_RECORD_KIND
            || row.schema_version != AI_RUN_HISTORY_SCHEMA_VERSION
            || row.projection_ref.trim().is_empty()
            || row.canonical_run_ids.is_empty()
            || !row.preserves_canonical_run_id
            || !row.preserves_approval_timeline
            || !row.preserves_evidence_incompleteness
            || !row.raw_private_material_excluded
            || !row.supports_json_export
        {
            violations.push(AiRunHistoryViolation::SurfaceProjectionDrift);
            continue;
        }

        for reference in &row.canonical_run_ids {
            if !canonical_run_ids.contains(reference.as_str()) {
                violations.push(AiRunHistoryViolation::SurfaceMissingCanonicalRunId);
                break;
            }
        }
        for reference in &row.rerun_review_refs {
            if !rerun_review_ids.contains(reference.as_str()) {
                violations.push(AiRunHistoryViolation::SurfaceMissingRerunReviewRef);
                break;
            }
        }
    }
}

fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_boundary_material(text),
        serde_json::Value::Array(values) => {
            values.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

fn contains_forbidden_boundary_material(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("api_key=")
        || lower.contains("api-key=")
        || lower.contains("raw_api_key")
        || lower.contains("oauth_token")
        || lower.contains("bearer ")
}

#[cfg(test)]
mod tests;
