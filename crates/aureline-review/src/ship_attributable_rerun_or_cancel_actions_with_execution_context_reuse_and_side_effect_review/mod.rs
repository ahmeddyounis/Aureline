//! Attributable rerun / cancel actions with execution-context reuse and side-effect review.
//!
//! This module implements the canonical M5 truth packet for the in-product
//! rerun / cancel surface: the lane that lets a reviewer rerun or cancel a CI /
//! pipeline / deployment run without ever firing an upstream effect that is
//! unattributable, that silently reuses a stale execution context, or that
//! mutates external state before its side effects have been reviewed. It binds
//! three pillars into one export-safe record:
//!
//! - **Attributable rerun / cancel actions** — each [`RerunCancelActionRow`]
//!   carries the run it acts on, its durable review anchor, the rerun/cancel
//!   control class, the target scope, the provider-mode mutation mode, the
//!   blocked class, the actor attribution, the audit row that will land, and the
//!   reviewable effect summary, so every rerun or cancel stays attributable and
//!   no action overstates the scope it will touch.
//! - **Execution-context reuse** — each action records the execution context it
//!   would reuse, the reuse decision, and the context freshness, so a degraded
//!   or stale reused context is flagged and reviewed rather than silently
//!   replayed, and an `unknown` provider-owned context is never flattened into a
//!   live one.
//! - **Side-effect review** — each [`SideEffectReviewRow`] records the typed
//!   side-effect class, its acknowledgment requirement, and its disclosure label
//!   for an action, so a non-inert side effect (a deploy, a release, an external
//!   write, a notification, a cost) requires explicit acknowledgment before the
//!   control can fire and never creates hidden write scope.
//!
//! The packet references upstream run-control-review, pipeline-run-row,
//! execution-context, and trust-class contracts by id rather than embedding
//! their content. Raw run / log / artifact bodies, raw provider payloads, raw
//! URLs, raw absolute paths, raw author email addresses, credentials, and live
//! provider responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/review/ship-attributable-rerun-or-cancel-actions-with-execution-context-reuse-and-side-effect-review.schema.json`](../../../../schemas/review/ship-attributable-rerun-or-cancel-actions-with-execution-context-reuse-and-side-effect-review.schema.json).
//! The contract doc is
//! [`docs/review/m5/ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review.md`](../../../../docs/review/m5/ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review.md).
//! The protected fixture directory is
//! [`fixtures/review/m5/ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review/`](../../../../fixtures/review/m5/ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`RerunCancelReviewPacket`].
pub const RERUN_CANCEL_RECORD_KIND: &str =
    "ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review";

/// Schema version for rerun/cancel review records.
pub const RERUN_CANCEL_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const RERUN_CANCEL_SCHEMA_REF: &str =
    "schemas/review/ship-attributable-rerun-or-cancel-actions-with-execution-context-reuse-and-side-effect-review.schema.json";

/// Repo-relative path of the rerun/cancel review contract doc.
pub const RERUN_CANCEL_DOC_REF: &str =
    "docs/review/m5/ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review.md";

/// Repo-relative path of the run-control-review contract this lane builds on.
pub const RERUN_CANCEL_RUN_CONTROL_CONTRACT_REF: &str = "schemas/ci/run_control_review.schema.json";

/// Repo-relative path of the pipeline-run-row contract this lane acts on.
pub const RERUN_CANCEL_PIPELINE_RUN_CONTRACT_REF: &str = "schemas/ci/pipeline_run_row.schema.json";

/// Repo-relative path of the execution-context contract reused for context reuse.
pub const RERUN_CANCEL_EXECUTION_CONTEXT_CONTRACT_REF: &str =
    "schemas/runtime/execution_context.schema.json";

/// Repo-relative path of the trust-class vocabulary this lane reuses.
pub const RERUN_CANCEL_TRUST_CLASS_CONTRACT_REF: &str = "schemas/security/trust_class.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const RERUN_CANCEL_FIXTURE_DIR: &str =
    "fixtures/review/m5/ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review";

/// Repo-relative path of the checked support-export artifact.
pub const RERUN_CANCEL_ARTIFACT_REF: &str =
    "artifacts/review/m5/ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const RERUN_CANCEL_SUMMARY_REF: &str =
    "artifacts/review/m5/ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review.md";

/// Rerun / cancel control class an action exercises.
///
/// A focused subset of the frozen run-control vocabulary, scoped to rerun and
/// cancel. `unknown_control_provider_owned` must never be flattened into a known
/// control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionControlClass {
    /// Rerun an entire workflow run.
    RerunWorkflow,
    /// Rerun only the failed jobs of a run.
    RerunFailedJobs,
    /// Rerun a single job.
    RerunSingleJob,
    /// Rerun a single step.
    RerunSingleStep,
    /// Cancel an entire workflow run.
    CancelWorkflow,
    /// Cancel a single job.
    CancelSingleJob,
    /// Provider returned a control the contract does not recognise yet.
    UnknownControlProviderOwned,
}

impl ActionControlClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RerunWorkflow => "rerun_workflow",
            Self::RerunFailedJobs => "rerun_failed_jobs",
            Self::RerunSingleJob => "rerun_single_job",
            Self::RerunSingleStep => "rerun_single_step",
            Self::CancelWorkflow => "cancel_workflow",
            Self::CancelSingleJob => "cancel_single_job",
            Self::UnknownControlProviderOwned => "unknown_control_provider_owned",
        }
    }

    /// The target scope a known control must resolve to, if the control fixes one.
    pub const fn required_scope(self) -> Option<ActionTargetScope> {
        match self {
            Self::RerunWorkflow | Self::CancelWorkflow => {
                Some(ActionTargetScope::EntireWorkflowRun)
            }
            Self::RerunFailedJobs => Some(ActionTargetScope::FailedJobsOnly),
            Self::RerunSingleJob | Self::CancelSingleJob => Some(ActionTargetScope::SingleJobOnly),
            Self::RerunSingleStep => Some(ActionTargetScope::SingleStepOnly),
            Self::UnknownControlProviderOwned => None,
        }
    }

    /// Whether this control needs at least one explicit attention reason.
    ///
    /// A provider-owned unknown control must carry a reason so the surface never
    /// presents it as a benign known action.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(self, Self::UnknownControlProviderOwned)
    }
}

/// Target scope a rerun / cancel action touches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionTargetScope {
    /// A single step only.
    SingleStepOnly,
    /// A single job only.
    SingleJobOnly,
    /// The failed jobs only.
    FailedJobsOnly,
    /// The entire workflow run.
    EntireWorkflowRun,
    /// The entire check run.
    EntireCheckRun,
    /// The entire deployment run.
    EntireDeploymentRun,
    /// The entire release run.
    EntireReleaseRun,
}

impl ActionTargetScope {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleStepOnly => "single_step_only",
            Self::SingleJobOnly => "single_job_only",
            Self::FailedJobsOnly => "failed_jobs_only",
            Self::EntireWorkflowRun => "entire_workflow_run",
            Self::EntireCheckRun => "entire_check_run",
            Self::EntireDeploymentRun => "entire_deployment_run",
            Self::EntireReleaseRun => "entire_release_run",
        }
    }
}

/// Provider-mode mutation mode a rerun / cancel action fires under.
///
/// Rerun and cancel controls always reach upstream provider state, so the
/// local-only `local_draft` mode is intentionally absent. Each mode cites the
/// grant it depends on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionMutationMode {
    /// Fires the upstream effect now; must cite an approval ticket.
    PublishNow,
    /// Hands off to the provider in the browser; must cite a browser-handoff packet.
    OpenInProvider,
    /// Queues the effect for a later drain; must cite a publish-later queue item.
    DeferredPublish,
}

impl ActionMutationMode {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublishNow => "publish_now",
            Self::OpenInProvider => "open_in_provider",
            Self::DeferredPublish => "deferred_publish",
        }
    }

    /// Whether this mode must cite an approval ticket ref.
    pub const fn requires_approval_ref(self) -> bool {
        matches!(self, Self::PublishNow)
    }

    /// Whether this mode must cite a browser-handoff packet ref.
    pub const fn requires_browser_handoff_ref(self) -> bool {
        matches!(self, Self::OpenInProvider)
    }

    /// Whether this mode must cite a publish-later queue item ref.
    pub const fn requires_deferred_queue_ref(self) -> bool {
        matches!(self, Self::DeferredPublish)
    }
}

/// Decision about how a rerun reuses (or forks) an execution context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionContextReuseDecision {
    /// Reuse the identical recorded execution context.
    ReuseIdenticalContext,
    /// Reuse the context with pinned inputs.
    ReuseWithPinnedInputs,
    /// Reuse the context but refresh short-lived secrets.
    ReuseWithRefreshedSecrets,
    /// Fork a fresh execution context.
    ForkNewContext,
    /// Defer to the provider's default context.
    ProviderDefaultContext,
    /// Provider returned a context decision the contract does not recognise yet.
    UnknownContextProviderOwned,
}

impl ExecutionContextReuseDecision {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReuseIdenticalContext => "reuse_identical_context",
            Self::ReuseWithPinnedInputs => "reuse_with_pinned_inputs",
            Self::ReuseWithRefreshedSecrets => "reuse_with_refreshed_secrets",
            Self::ForkNewContext => "fork_new_context",
            Self::ProviderDefaultContext => "provider_default_context",
            Self::UnknownContextProviderOwned => "unknown_context_provider_owned",
        }
    }

    /// Whether this decision replays an existing recorded context.
    ///
    /// Forking a fresh context or deferring to a provider default does not replay
    /// recorded context state, so a degraded freshness on the recorded context
    /// cannot mislead the user about what will run.
    pub const fn reuses_existing_context(self) -> bool {
        matches!(
            self,
            Self::ReuseIdenticalContext
                | Self::ReuseWithPinnedInputs
                | Self::ReuseWithRefreshedSecrets
        )
    }

    /// Whether this decision needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(self, Self::UnknownContextProviderOwned)
    }
}

/// Freshness class shown for a reused execution context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextFreshness {
    /// Live provider truth.
    AuthoritativeLive,
    /// Warm cached truth.
    WarmCached,
    /// Degraded cached truth.
    DegradedCached,
    /// Stale truth.
    Stale,
    /// Truth could not be verified.
    Unverified,
}

impl ContextFreshness {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::DegradedCached => "degraded_cached",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
        }
    }

    /// Whether this class is degraded enough that reuse must be flagged and reviewed.
    pub const fn narrows_reuse(self) -> bool {
        matches!(self, Self::DegradedCached | Self::Stale | Self::Unverified)
    }
}

/// Why a rerun / cancel action is blocked, if it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionBlockedClass {
    /// The action is admissible.
    NotBlocked,
    /// No auth grant for the target scope.
    BlockedNoAuthForTargetScope,
    /// Policy forbids the control.
    BlockedPolicyForbidsControl,
    /// The provider does not support the control.
    BlockedProviderDoesNotSupportControl,
    /// The run state is not eligible for the control.
    BlockedRunStateNotEligible,
    /// Run freshness is stale and a review is required first.
    BlockedFreshnessStaleReviewRequired,
    /// The reused execution context is stale and a review is required first.
    BlockedContextReuseStaleReviewRequired,
    /// A side effect is unacknowledged and a review is required first.
    BlockedSideEffectUnacknowledged,
    /// The surface is offline or disconnected.
    BlockedOfflineOrDisconnected,
    /// Provider returned a block reason the contract does not recognise yet.
    BlockedUnknownReasonProviderOwned,
}

impl ActionBlockedClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotBlocked => "not_blocked",
            Self::BlockedNoAuthForTargetScope => "blocked_no_auth_for_target_scope",
            Self::BlockedPolicyForbidsControl => "blocked_policy_forbids_control",
            Self::BlockedProviderDoesNotSupportControl => {
                "blocked_provider_does_not_support_control"
            }
            Self::BlockedRunStateNotEligible => "blocked_run_state_not_eligible",
            Self::BlockedFreshnessStaleReviewRequired => "blocked_freshness_stale_review_required",
            Self::BlockedContextReuseStaleReviewRequired => {
                "blocked_context_reuse_stale_review_required"
            }
            Self::BlockedSideEffectUnacknowledged => "blocked_side_effect_unacknowledged",
            Self::BlockedOfflineOrDisconnected => "blocked_offline_or_disconnected",
            Self::BlockedUnknownReasonProviderOwned => "blocked_unknown_reason_provider_owned",
        }
    }

    /// Whether the action is blocked.
    pub const fn is_blocked(self) -> bool {
        !matches!(self, Self::NotBlocked)
    }

    /// Whether this block class needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        self.is_blocked()
    }
}

/// Typed side-effect class an action would fire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectClass {
    /// No external side effect; the action only re-evaluates in place.
    NoExternalSideEffect,
    /// Triggers a deployment.
    TriggersDeployment,
    /// Triggers a release.
    TriggersRelease,
    /// Sends notifications.
    SendsNotifications,
    /// Writes an external artifact.
    WritesExternalArtifact,
    /// Mutates provider run state (for example, cancels an in-flight job).
    MutatesProviderRunState,
    /// Consumes quota or incurs cost.
    ConsumesQuotaOrCost,
    /// Provider returned a side-effect class the contract does not recognise yet.
    UnknownSideEffectProviderOwned,
}

impl SideEffectClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoExternalSideEffect => "no_external_side_effect",
            Self::TriggersDeployment => "triggers_deployment",
            Self::TriggersRelease => "triggers_release",
            Self::SendsNotifications => "sends_notifications",
            Self::WritesExternalArtifact => "writes_external_artifact",
            Self::MutatesProviderRunState => "mutates_provider_run_state",
            Self::ConsumesQuotaOrCost => "consumes_quota_or_cost",
            Self::UnknownSideEffectProviderOwned => "unknown_side_effect_provider_owned",
        }
    }

    /// Whether this side effect is inert (no external write scope).
    pub const fn is_inert(self) -> bool {
        matches!(self, Self::NoExternalSideEffect)
    }

    /// Whether this side effect must carry an explicit acknowledgment requirement.
    ///
    /// Any non-inert side effect — including an unknown provider-owned one — must
    /// be acknowledged before the control can fire, so the action never creates
    /// hidden write scope.
    pub const fn requires_acknowledgment(self) -> bool {
        !self.is_inert()
    }
}

/// Acknowledgment a side effect requires before its action can fire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectAckRequirement {
    /// No acknowledgment required; admissible only for inert side effects.
    NoAckRequired,
    /// The user must explicitly confirm the effect.
    RequiresExplicitConfirmation,
    /// The effect requires a spent approval ticket.
    RequiresApprovalTicket,
    /// The effect requires a returned browser handoff grant.
    RequiresBrowserHandoff,
    /// The effect is queued for a later drain.
    RequiresDeferredQueue,
    /// No safe action is offered for the effect.
    DeniedNoSafeAction,
}

impl SideEffectAckRequirement {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAckRequired => "no_ack_required",
            Self::RequiresExplicitConfirmation => "requires_explicit_confirmation",
            Self::RequiresApprovalTicket => "requires_approval_ticket",
            Self::RequiresBrowserHandoff => "requires_browser_handoff",
            Self::RequiresDeferredQueue => "requires_deferred_queue",
            Self::DeniedNoSafeAction => "denied_no_safe_action",
        }
    }

    /// Whether this requirement is stronger than no acknowledgment.
    pub const fn is_explicit(self) -> bool {
        !matches!(self, Self::NoAckRequired)
    }
}

/// Downgrade trigger that can narrow this lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunCancelDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// An action was surfaced without attribution.
    ActionAttributionMissing,
    /// A reused execution context was stale.
    ContextReuseStale,
    /// A side effect was surfaced without review.
    SideEffectUnreviewed,
    /// A rerun/cancel authority was revoked.
    RunControlAuthorityRevoked,
    /// Workspace trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond the qualified rerun/cancel boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl RerunCancelDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ActionAttributionMissing,
        Self::ContextReuseStale,
        Self::SideEffectUnreviewed,
        Self::RunControlAuthorityRevoked,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ActionAttributionMissing => "action_attribution_missing",
            Self::ContextReuseStale => "context_reuse_stale",
            Self::SideEffectUnreviewed => "side_effect_unreviewed",
            Self::RunControlAuthorityRevoked => "run_control_authority_revoked",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must project this lane's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunCancelConsumerSurface {
    /// Runs panel.
    RunsPanel,
    /// Run-control menu.
    RunControlMenu,
    /// Side-effect review sheet.
    SideEffectReviewSheet,
    /// Review workspace header.
    ReviewWorkspaceHeader,
    /// Command palette.
    CommandPalette,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
}

impl RerunCancelConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::RunsPanel,
        Self::RunControlMenu,
        Self::SideEffectReviewSheet,
        Self::ReviewWorkspaceHeader,
        Self::CommandPalette,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunsPanel => "runs_panel",
            Self::RunControlMenu => "run_control_menu",
            Self::SideEffectReviewSheet => "side_effect_review_sheet",
            Self::ReviewWorkspaceHeader => "review_workspace_header",
            Self::CommandPalette => "command_palette",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Reviewable effect summary projected for an action.
///
/// Four short reviewable strings: what upstream effect will fire, where it lands,
/// under whose authority, and what audit row will land. Every consumer surface
/// must project the same summary for the same action id within a session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionEffectSummary {
    /// What upstream effect will fire.
    pub what_will_fire_label: String,
    /// Where the effect lands (the target scope).
    pub where_label: String,
    /// Under whose authority the effect fires.
    pub under_whose_authority_label: String,
    /// What audit row will land when the action is invoked.
    pub audit_row_label: String,
}

impl ActionEffectSummary {
    /// Whether every reviewable string is present.
    pub fn is_complete(&self) -> bool {
        !self.what_will_fire_label.trim().is_empty()
            && !self.where_label.trim().is_empty()
            && !self.under_whose_authority_label.trim().is_empty()
            && !self.audit_row_label.trim().is_empty()
    }
}

/// One attributable rerun / cancel action row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunCancelActionRow {
    /// Stable action id.
    pub action_id: String,
    /// Run id this action acts on.
    pub run_id: String,
    /// Durable review anchor id bound to this action.
    pub durable_anchor_id: String,
    /// Human-readable target identity (what the run is for).
    pub target_identity_label: String,
    /// Rerun / cancel control class.
    pub control_class: ActionControlClass,
    /// Target scope the action touches.
    pub target_scope: ActionTargetScope,
    /// Provider-mode mutation mode the action fires under.
    pub mutation_mode: ActionMutationMode,
    /// Execution context id the action would reuse or fork from.
    pub execution_context_id: String,
    /// Execution-context reuse decision.
    pub context_reuse_decision: ExecutionContextReuseDecision,
    /// Freshness class of the reused execution context.
    pub context_freshness: ContextFreshness,
    /// Staleness label; required and non-empty when a reused context is degraded.
    pub context_staleness_label: String,
    /// Why the action is blocked, if it is.
    pub blocked_class: ActionBlockedClass,
    /// Human-readable actor attribution (under whose authority the action fires).
    pub actor_attribution_label: String,
    /// Opaque ref to the audit row that lands when the action is invoked.
    pub audit_row_ref: String,
    /// Reviewable effect summary.
    pub effect_summary: ActionEffectSummary,
    /// Attention reasons; required and non-empty when the action needs attention.
    pub attention_reasons: Vec<String>,
    /// Human-readable review summary.
    pub review_summary: String,
    /// Approval ticket ref; required when the mutation mode is `publish_now`.
    pub approval_ticket_ref: Option<String>,
    /// Browser-handoff packet ref; required when the mutation mode is `open_in_provider`.
    pub browser_handoff_ref: Option<String>,
    /// Publish-later queue item ref; required when the mutation mode is `deferred_publish`.
    pub deferred_queue_ref: Option<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl RerunCancelActionRow {
    /// Whether this action needs at least one explicit attention reason.
    pub const fn requires_attention_reason(&self) -> bool {
        self.control_class.requires_attention_reason()
            || self.context_reuse_decision.requires_attention_reason()
            || self.blocked_class.requires_attention_reason()
    }
}

/// One side-effect review row bound to an action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SideEffectReviewRow {
    /// Action id this side effect belongs to.
    pub action_id: String,
    /// Stable side-effect id.
    pub side_effect_id: String,
    /// Human-readable, redaction-aware side-effect label.
    pub side_effect_label: String,
    /// Typed side-effect class.
    pub side_effect_class: SideEffectClass,
    /// Acknowledgment the side effect requires before the action can fire.
    pub acknowledgment_requirement: SideEffectAckRequirement,
    /// Human-readable disclosure label shown for the side effect.
    pub disclosure_label: String,
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunCancelTrustReview {
    /// Control class is explicit, never implied.
    pub action_control_class_explicit: bool,
    /// Target scope is explicit and never overstated.
    pub target_scope_explicit: bool,
    /// Every mutating action is attributable to an actor.
    pub every_mutating_action_attributable: bool,
    /// An audit row is recorded for every action.
    pub audit_row_recorded_for_every_action: bool,
    /// Execution-context reuse is explicit, never implied.
    pub execution_context_reuse_explicit: bool,
    /// A stale reused context is flagged and reviewed, never silently replayed.
    pub stale_context_reuse_flagged_not_hidden: bool,
    /// Side effects are reviewed before the control can be invoked.
    pub side_effect_reviewed_before_invocation: bool,
    /// A non-inert side effect requires an explicit acknowledgment.
    pub non_inert_side_effect_requires_acknowledgment: bool,
    /// No rerun / cancel action creates hidden write scope.
    pub no_hidden_write_scope: bool,
    /// The mutation mode cites the grant it depends on.
    pub mutation_mode_cites_required_grant: bool,
    /// Downgrade narrows the claim rather than hiding the lane.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl RerunCancelTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.action_control_class_explicit
            && self.target_scope_explicit
            && self.every_mutating_action_attributable
            && self.audit_row_recorded_for_every_action
            && self.execution_context_reuse_explicit
            && self.stale_context_reuse_flagged_not_hidden
            && self.side_effect_reviewed_before_invocation
            && self.non_inert_side_effect_requires_acknowledgment
            && self.no_hidden_write_scope
            && self.mutation_mode_cites_required_grant
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunCancelConsumerProjection {
    /// Runs panel shows the control class.
    pub runs_panel_shows_control_class: bool,
    /// Run-control menu shows the target scope.
    pub run_control_menu_shows_target_scope: bool,
    /// Run-control menu shows the actor attribution.
    pub run_control_menu_shows_attribution: bool,
    /// Side-effect sheet shows the effect summary.
    pub side_effect_sheet_shows_effect_summary: bool,
    /// Side-effect sheet shows the acknowledgment requirement.
    pub side_effect_sheet_shows_acknowledgment: bool,
    /// Review workspace header shows the context reuse decision.
    pub review_workspace_header_shows_context_reuse: bool,
    /// Command palette shows the effect summary.
    pub command_palette_shows_effect_summary: bool,
    /// CLI / headless shows qualification truth.
    pub cli_headless_shows_truth: bool,
    /// Support export shows qualification truth.
    pub support_export_shows_truth: bool,
    /// Diagnostics shows qualification truth.
    pub diagnostics_shows_truth: bool,
    /// Help / About shows qualification truth.
    pub help_about_shows_truth: bool,
    /// Preview / Labs lanes are labeled when not covered by this packet.
    pub preview_labs_label_for_unqualified: bool,
}

impl RerunCancelConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.runs_panel_shows_control_class
            && self.run_control_menu_shows_target_scope
            && self.run_control_menu_shows_attribution
            && self.side_effect_sheet_shows_effect_summary
            && self.side_effect_sheet_shows_acknowledgment
            && self.review_workspace_header_shows_context_reuse
            && self.command_palette_shows_effect_summary
            && self.cli_headless_shows_truth
            && self.support_export_shows_truth
            && self.diagnostics_shows_truth
            && self.help_about_shows_truth
            && self.preview_labs_label_for_unqualified
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunCancelProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`RerunCancelReviewPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RerunCancelReviewPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Attributable rerun / cancel action rows.
    pub action_rows: Vec<RerunCancelActionRow>,
    /// Side-effect review rows.
    pub side_effect_rows: Vec<SideEffectReviewRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<RerunCancelDowngradeTrigger>,
    /// Consumer surfaces that must project this lane's truth.
    pub consumer_surfaces: Vec<RerunCancelConsumerSurface>,
    /// Trust review block.
    pub trust_review: RerunCancelTrustReview,
    /// Consumer projection block.
    pub consumer_projection: RerunCancelConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RerunCancelProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe attributable rerun / cancel actions, execution-context reuse, and side-effect review packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunCancelReviewPacket {
    /// Record kind; must equal [`RERUN_CANCEL_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`RERUN_CANCEL_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Attributable rerun / cancel action rows.
    pub action_rows: Vec<RerunCancelActionRow>,
    /// Side-effect review rows.
    pub side_effect_rows: Vec<SideEffectReviewRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<RerunCancelDowngradeTrigger>,
    /// Consumer surfaces that must project this lane's truth.
    pub consumer_surfaces: Vec<RerunCancelConsumerSurface>,
    /// Trust review block.
    pub trust_review: RerunCancelTrustReview,
    /// Consumer projection block.
    pub consumer_projection: RerunCancelConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RerunCancelProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl RerunCancelReviewPacket {
    /// Builds a rerun / cancel review packet from stable-lane input.
    pub fn new(input: RerunCancelReviewPacketInput) -> Self {
        Self {
            record_kind: RERUN_CANCEL_RECORD_KIND.to_owned(),
            schema_version: RERUN_CANCEL_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            action_rows: input.action_rows,
            side_effect_rows: input.side_effect_rows,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the rerun / cancel review invariants.
    pub fn validate(&self) -> Vec<RerunCancelViolation> {
        let mut violations = Vec::new();

        if self.record_kind != RERUN_CANCEL_RECORD_KIND {
            violations.push(RerunCancelViolation::WrongRecordKind);
        }
        if self.schema_version != RERUN_CANCEL_SCHEMA_VERSION {
            violations.push(RerunCancelViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(RerunCancelViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(RerunCancelViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(RerunCancelViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_action_rows(self, &mut violations);
        validate_side_effect_rows(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(RerunCancelViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(RerunCancelViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(RerunCancelViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("rerun/cancel review packet serializes"),
        ) {
            violations.push(RerunCancelViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("rerun/cancel review packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let rerun_actions = self
            .action_rows
            .iter()
            .filter(|row| {
                matches!(
                    row.control_class,
                    ActionControlClass::RerunWorkflow
                        | ActionControlClass::RerunFailedJobs
                        | ActionControlClass::RerunSingleJob
                        | ActionControlClass::RerunSingleStep
                )
            })
            .count();
        let cancel_actions = self
            .action_rows
            .iter()
            .filter(|row| {
                matches!(
                    row.control_class,
                    ActionControlClass::CancelWorkflow | ActionControlClass::CancelSingleJob
                )
            })
            .count();
        let blocked_actions = self
            .action_rows
            .iter()
            .filter(|row| row.blocked_class.is_blocked())
            .count();
        let acknowledged_effects = self
            .side_effect_rows
            .iter()
            .filter(|row| row.acknowledgment_requirement.is_explicit())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Attributable Rerun / Cancel Actions, Execution-Context Reuse, and Side-Effect Review\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Actions: {} ({} rerun, {} cancel, {} blocked)\n",
            self.action_rows.len(),
            rerun_actions,
            cancel_actions,
            blocked_actions
        ));
        out.push_str(&format!(
            "- Side effects: {} ({} requiring acknowledgment)\n",
            self.side_effect_rows.len(),
            acknowledged_effects
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Actions\n\n");
        for row in &self.action_rows {
            out.push_str(&format!(
                "- **{}** ({}) → anchor `{}`: scope `{}`, mode `{}`, context `{}`/`{}`, blocked `{}`, authority `{}`\n",
                row.target_identity_label,
                row.control_class.as_str(),
                row.durable_anchor_id,
                row.target_scope.as_str(),
                row.mutation_mode.as_str(),
                row.context_reuse_decision.as_str(),
                row.context_freshness.as_str(),
                row.blocked_class.as_str(),
                row.actor_attribution_label
            ));
        }

        out.push_str("\n## Side-effect review\n\n");
        for row in &self.side_effect_rows {
            out.push_str(&format!(
                "- `{}` on `{}`: class `{}`, ack `{}`\n",
                row.side_effect_id,
                row.action_id,
                row.side_effect_class.as_str(),
                row.acknowledgment_requirement.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in rerun / cancel review export.
#[derive(Debug)]
pub enum RerunCancelArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RerunCancelViolation>),
}

impl fmt::Display for RerunCancelArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "rerun/cancel review export parse failed: {error}"
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
                    "rerun/cancel review export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for RerunCancelArtifactError {}

/// Validation failures emitted by [`RerunCancelReviewPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RerunCancelViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No action rows are present.
    ActionRowsMissing,
    /// An action row is incomplete.
    ActionRowIncomplete,
    /// An action's effect summary is incomplete.
    EffectSummaryIncomplete,
    /// An action's control class and target scope disagree.
    ControlScopeMismatch,
    /// A mutating action is missing its actor attribution or audit row.
    AttributionMissing,
    /// A mutation mode is missing the grant ref it requires.
    MutationGrantRefMissing,
    /// A degraded reused context is missing its staleness label.
    ContextReuseStaleUnflagged,
    /// An action needing attention is missing its attention reasons.
    AttentionReasonMissing,
    /// An action has no side-effect review row.
    ActionMissingSideEffectReview,
    /// A side-effect row references an action id with no action row.
    OrphanRowReference,
    /// No side-effect review rows are present.
    SideEffectRowsMissing,
    /// A side-effect review row is incomplete.
    SideEffectRowIncomplete,
    /// A non-inert side effect is missing an explicit acknowledgment requirement.
    SideEffectUnacknowledged,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl RerunCancelViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ActionRowsMissing => "action_rows_missing",
            Self::ActionRowIncomplete => "action_row_incomplete",
            Self::EffectSummaryIncomplete => "effect_summary_incomplete",
            Self::ControlScopeMismatch => "control_scope_mismatch",
            Self::AttributionMissing => "attribution_missing",
            Self::MutationGrantRefMissing => "mutation_grant_ref_missing",
            Self::ContextReuseStaleUnflagged => "context_reuse_stale_unflagged",
            Self::AttentionReasonMissing => "attention_reason_missing",
            Self::ActionMissingSideEffectReview => "action_missing_side_effect_review",
            Self::OrphanRowReference => "orphan_row_reference",
            Self::SideEffectRowsMissing => "side_effect_rows_missing",
            Self::SideEffectRowIncomplete => "side_effect_row_incomplete",
            Self::SideEffectUnacknowledged => "side_effect_unacknowledged",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable rerun / cancel review export.
pub fn current_rerun_cancel_review_export(
) -> Result<RerunCancelReviewPacket, RerunCancelArtifactError> {
    let packet: RerunCancelReviewPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review/support_export.json"
    )))
    .map_err(RerunCancelArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RerunCancelArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &RerunCancelReviewPacket,
    violations: &mut Vec<RerunCancelViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        RERUN_CANCEL_SCHEMA_REF,
        RERUN_CANCEL_DOC_REF,
        RERUN_CANCEL_RUN_CONTROL_CONTRACT_REF,
        RERUN_CANCEL_PIPELINE_RUN_CONTRACT_REF,
        RERUN_CANCEL_EXECUTION_CONTEXT_CONTRACT_REF,
        RERUN_CANCEL_TRUST_CLASS_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(RerunCancelViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_action_rows(
    packet: &RerunCancelReviewPacket,
    violations: &mut Vec<RerunCancelViolation>,
) {
    if packet.action_rows.is_empty() {
        violations.push(RerunCancelViolation::ActionRowsMissing);
        return;
    }

    let reviewed_actions: BTreeSet<&str> = packet
        .side_effect_rows
        .iter()
        .map(|row| row.action_id.as_str())
        .collect();

    for row in &packet.action_rows {
        if row.action_id.trim().is_empty()
            || row.run_id.trim().is_empty()
            || row.durable_anchor_id.trim().is_empty()
            || row.target_identity_label.trim().is_empty()
            || row.execution_context_id.trim().is_empty()
            || row.review_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(RerunCancelViolation::ActionRowIncomplete);
        }
        if !row.effect_summary.is_complete() {
            violations.push(RerunCancelViolation::EffectSummaryIncomplete);
        }
        if let Some(required) = row.control_class.required_scope() {
            if required != row.target_scope {
                violations.push(RerunCancelViolation::ControlScopeMismatch);
            }
        }
        if row.actor_attribution_label.trim().is_empty() || row.audit_row_ref.trim().is_empty() {
            violations.push(RerunCancelViolation::AttributionMissing);
        }
        validate_mutation_grant(row, violations);
        if row.context_reuse_decision.reuses_existing_context()
            && row.context_freshness.narrows_reuse()
            && row.context_staleness_label.trim().is_empty()
        {
            violations.push(RerunCancelViolation::ContextReuseStaleUnflagged);
        }
        if row.requires_attention_reason() && row.attention_reasons.is_empty() {
            violations.push(RerunCancelViolation::AttentionReasonMissing);
        }
        if !row.action_id.trim().is_empty() && !reviewed_actions.contains(row.action_id.as_str()) {
            violations.push(RerunCancelViolation::ActionMissingSideEffectReview);
        }
    }
}

fn validate_mutation_grant(row: &RerunCancelActionRow, violations: &mut Vec<RerunCancelViolation>) {
    let approval_ok = !row.mutation_mode.requires_approval_ref()
        || row
            .approval_ticket_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty());
    let handoff_ok = !row.mutation_mode.requires_browser_handoff_ref()
        || row
            .browser_handoff_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty());
    let deferred_ok = !row.mutation_mode.requires_deferred_queue_ref()
        || row
            .deferred_queue_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty());
    if !(approval_ok && handoff_ok && deferred_ok) {
        violations.push(RerunCancelViolation::MutationGrantRefMissing);
    }
}

fn validate_side_effect_rows(
    packet: &RerunCancelReviewPacket,
    violations: &mut Vec<RerunCancelViolation>,
) {
    if packet.side_effect_rows.is_empty() {
        violations.push(RerunCancelViolation::SideEffectRowsMissing);
        return;
    }

    let action_ids: BTreeSet<&str> = packet
        .action_rows
        .iter()
        .map(|row| row.action_id.as_str())
        .collect();

    for row in &packet.side_effect_rows {
        if row.action_id.trim().is_empty()
            || row.side_effect_id.trim().is_empty()
            || row.side_effect_label.trim().is_empty()
            || row.disclosure_label.trim().is_empty()
        {
            violations.push(RerunCancelViolation::SideEffectRowIncomplete);
        }
        if !row.action_id.trim().is_empty() && !action_ids.contains(row.action_id.as_str()) {
            violations.push(RerunCancelViolation::OrphanRowReference);
        }
        if row.side_effect_class.requires_acknowledgment()
            && !row.acknowledgment_requirement.is_explicit()
        {
            violations.push(RerunCancelViolation::SideEffectUnacknowledged);
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret ")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
