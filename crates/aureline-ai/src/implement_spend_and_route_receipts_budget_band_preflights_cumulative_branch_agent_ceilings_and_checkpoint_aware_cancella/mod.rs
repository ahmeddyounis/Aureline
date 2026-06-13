//! Spend and route receipts, budget-band preflights, cumulative branch-agent
//! ceilings, and checkpoint-aware cancellation-export parity across inline
//! assist, patch review, and side-branch agents.
//!
//! This module deepens the long-running-agent budget surface into one
//! export-safe truth packet whose unit of truth is an [`AiRunReceiptRow`]: a
//! single AI run — inline assist, patch review, or a branch/worktree agent —
//! binding the budget-band preflight that prices it before dispatch, the route
//! receipt that records which provider/model/mode actually served it and why a
//! downgrade or fallback fired, the post-run spend receipt that reconciles the
//! measured cost against the preflight, the cumulative ceiling and ordered
//! checkpoints that bound a side-branch run across its whole lifetime, and the
//! checkpoint-aware cancellation export that turns a cancellation into a
//! readable, export-safe receipt instead of an opaque kill. The packet is the
//! canonical receipt source for shell, docs, support export, and release tooling;
//! consumers project it instead of re-deriving spend, route, ceiling, or
//! cancellation state by hand.
//!
//! The packet refuses to present an AI run greener than its receipt posture can
//! back. Every claimed run carries a budget-band preflight, a route receipt
//! whose resolved mode agrees with the run, and a post-run spend receipt; a
//! claimed charged run must have an acknowledged preflight so cost is disclosed
//! before it is spent, and a claimed run must project the wall-clock, token, and
//! tool-call ceilings. A route downgrade or fallback must carry a precise reason
//! rather than collapsing into a generic provider error when a more precise
//! reason exists, and the route receipt's mode must match the run's resolved
//! mode. A post-run spend receipt reconciles against its preflight — a measured
//! cost band above the projected band must be disclosed as an overrun, a charged
//! band must disclose who is charged, and an estimate-only receipt may not back a
//! Stable claim. A branch/worktree agent must carry a cumulative ceiling bounding
//! its whole run and ordered checkpoints whose cumulative cost band only
//! accumulates; its cancellation must name the checkpoint it stopped at, carry an
//! export-safe receipt with support-export parity, and a precise reason — so a
//! side-branch run is never killed opaquely. Every run carries a closed set of
//! downgrade rules — including the stale-proof and provider-unavailable triggers
//! — that narrow the claim instead of hiding the route, reusing the
//! qualification, lane, downgrade-trigger, and rollback-posture vocabularies
//! frozen by the M5 AI workflow matrix lane, the mode, quota, cost-band, and
//! fallback-hop vocabularies frozen by the routing-policy lane, and the ceiling
//! vocabularies frozen by the spend-ledger lane, so no receipt row may stay
//! greener than its evidence.
//!
//! Raw provider endpoints, credential bodies, raw provider payloads, exact token
//! counts, and exact spend amounts stay outside the support boundary; the packet
//! carries modes, lanes, coarse bands, ceiling consumption classes, reconciliation
//! classes, and review-safe labels only.
//!
//! The boundary schema is
//! [`schemas/ai/implement-spend-and-route-receipts-budget-band-preflights-cumulative-branch-agent-ceilings-and-checkpoint-aware-cancella.schema.json`](../../../../schemas/ai/implement-spend-and-route-receipts-budget-band-preflights-cumulative-branch-agent-ceilings-and-checkpoint-aware-cancella.schema.json).
//! The contract doc is
//! [`docs/ai/m5/implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella.md`](../../../../docs/ai/m5/implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella/`](../../../../fixtures/ai/m5/implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_spend_ledgers_quota_warning_surfaces_and_wall_clock_or_token_or_tool_call_ceilings_for_long_running_agents::{
    CeilingConsumptionClass, CeilingEnforcementClass, CeilingKindClass,
    LongRunningAgentRunStateClass, AGENT_BUDGET_SCHEMA_REF,
};
use crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::{
    M5AiWorkflowConsumerSurface, M5AiWorkflowDowngradeTrigger, M5AiWorkflowLane,
    M5AiWorkflowQualificationClass, M5AiWorkflowRollbackPosture, M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
};
use crate::implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains::{
    ChargedDisclosureClass, CostBandClass, CostMeasurementClass, FallbackHopOutcomeClass,
    FallbackHopReasonClass, RoutePolicyModeClass, ROUTING_POLICY_SCHEMA_REF,
};
use crate::materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors::PROVIDER_MODEL_REGISTRY_SCHEMA_REF;

/// Stable record-kind tag carried by [`AiRunReceiptPacket`].
pub const AI_RUN_RECEIPT_RECORD_KIND: &str =
    "implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella";

/// Schema version for AI run-receipt records.
pub const AI_RUN_RECEIPT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const AI_RUN_RECEIPT_SCHEMA_REF: &str =
    "schemas/ai/implement-spend-and-route-receipts-budget-band-preflights-cumulative-branch-agent-ceilings-and-checkpoint-aware-cancella.schema.json";

/// Repo-relative path of the receipt contract doc.
pub const AI_RUN_RECEIPT_DOC_REF: &str =
    "docs/ai/m5/implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella.md";

/// Repo-relative path of the protected fixture directory.
pub const AI_RUN_RECEIPT_FIXTURE_DIR: &str =
    "fixtures/ai/m5/implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella";

/// Repo-relative path of the checked support-export artifact.
pub const AI_RUN_RECEIPT_ARTIFACT_REF: &str =
    "artifacts/ai/m5/implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const AI_RUN_RECEIPT_SUMMARY_REF: &str =
    "artifacts/ai/m5/implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella.md";

/// How a run's budget-band preflight was acknowledged before dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreflightAcknowledgementClass {
    /// The user explicitly acknowledged the projected budget band.
    AcknowledgedByUser,
    /// The projected band fell inside a pre-approved budget and auto-approved.
    AutoApprovedWithinBudget,
    /// The preflight was surfaced but is still awaiting acknowledgement.
    PendingAcknowledgement,
    /// No acknowledgement is required for an unmetered on-device run.
    NotRequiredLocalUnmetered,
}

impl PreflightAcknowledgementClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AcknowledgedByUser => "acknowledged_by_user",
            Self::AutoApprovedWithinBudget => "auto_approved_within_budget",
            Self::PendingAcknowledgement => "pending_acknowledgement",
            Self::NotRequiredLocalUnmetered => "not_required_local_unmetered",
        }
    }

    /// Whether the projected cost has been disclosed and accepted before spend.
    pub const fn is_acknowledged(self) -> bool {
        matches!(
            self,
            Self::AcknowledgedByUser
                | Self::AutoApprovedWithinBudget
                | Self::NotRequiredLocalUnmetered
        )
    }
}

/// Whether and how the serving route changed away from the cheapest primary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteChangeClass {
    /// The primary qualifying route served the run with no change.
    NoChange,
    /// A cheaper or higher route was unavailable and a fallback hop served it.
    FallbackDowngrade,
    /// The execution mode changed (for example managed to BYOK) mid-route.
    ModeChanged,
    /// The run was redirected after the primary route's quota was exhausted.
    QuotaRedirect,
    /// The requested model was substituted for an available one.
    ModelSubstituted,
}

impl RouteChangeClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoChange => "no_change",
            Self::FallbackDowngrade => "fallback_downgrade",
            Self::ModeChanged => "mode_changed",
            Self::QuotaRedirect => "quota_redirect",
            Self::ModelSubstituted => "model_substituted",
        }
    }

    /// Whether the route moved off the primary, demanding a precise reason.
    pub const fn is_downgrade(self) -> bool {
        !matches!(self, Self::NoChange)
    }
}

/// Precise reason a route downgrade or fallback fired.
///
/// The closed set keeps a real cause visible; [`Self::GenericProviderError`] is
/// the catch-all that may never stand in for a precise reason when a downgrade
/// or fallback actually occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteDowngradeReasonClass {
    /// No downgrade fired; the primary route served the run.
    NoDowngrade,
    /// The prior route's provider quota was exhausted.
    QuotaExhausted,
    /// The prior route's provider became unavailable.
    ProviderUnavailable,
    /// Policy blocked the prior route.
    PolicyBlockedRoute,
    /// The requested model was deprecated.
    ModelDeprecated,
    /// The prior route exceeded its latency envelope.
    LatencyFallback,
    /// The prior route exceeded its per-session cost band.
    CostBandFallback,
    /// An unclassified provider error with no more precise reason available.
    GenericProviderError,
}

impl RouteDowngradeReasonClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDowngrade => "no_downgrade",
            Self::QuotaExhausted => "quota_exhausted",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::PolicyBlockedRoute => "policy_blocked_route",
            Self::ModelDeprecated => "model_deprecated",
            Self::LatencyFallback => "latency_fallback",
            Self::CostBandFallback => "cost_band_fallback",
            Self::GenericProviderError => "generic_provider_error",
        }
    }

    /// Whether this is the generic catch-all reason.
    pub const fn is_generic(self) -> bool {
        matches!(self, Self::GenericProviderError)
    }
}

/// How a post-run spend receipt reconciles against its budget-band preflight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpendReconciliationClass {
    /// The measured cost landed inside the projected band.
    WithinProjection,
    /// The measured cost landed below the projected band.
    UnderProjection,
    /// The measured cost landed above the projected band and is disclosed.
    OverProjectionDisclosed,
    /// The measured cost has not been reconciled against the preflight.
    NotReconciled,
}

impl SpendReconciliationClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WithinProjection => "within_projection",
            Self::UnderProjection => "under_projection",
            Self::OverProjectionDisclosed => "over_projection_disclosed",
            Self::NotReconciled => "not_reconciled",
        }
    }

    /// Whether the measured cost has been reconciled against the preflight.
    pub const fn is_reconciled(self) -> bool {
        !matches!(self, Self::NotReconciled)
    }
}

/// How a run was cancelled, if at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CancellationClass {
    /// The run was not cancelled.
    NotCancelled,
    /// The user cancelled the run.
    CancelledByUser,
    /// Policy cancelled the run.
    CancelledByPolicy,
    /// A ceiling was reached and cancelled the run.
    CancelledAtCeiling,
    /// The spend budget was exhausted and cancelled the run.
    CancelledAtBudgetExhaustion,
}

impl CancellationClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotCancelled => "not_cancelled",
            Self::CancelledByUser => "cancelled_by_user",
            Self::CancelledByPolicy => "cancelled_by_policy",
            Self::CancelledAtCeiling => "cancelled_at_ceiling",
            Self::CancelledAtBudgetExhaustion => "cancelled_at_budget_exhaustion",
        }
    }

    /// Whether the run was cancelled.
    pub const fn is_cancelled(self) -> bool {
        !matches!(self, Self::NotCancelled)
    }
}

/// Precise reason a cancellation fired.
///
/// [`Self::GenericProviderError`] is the catch-all that may never stand in for a
/// precise reason on a cancelled run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CancellationReasonClass {
    /// The run was not cancelled.
    NotCancelled,
    /// The user requested the cancellation.
    UserRequested,
    /// Policy blocked the run.
    PolicyBlocked,
    /// A configured ceiling was reached.
    CeilingReached,
    /// The spend budget was exhausted.
    BudgetExhausted,
    /// The provider became unavailable.
    ProviderUnavailable,
    /// An unclassified provider error with no more precise reason available.
    GenericProviderError,
}

impl CancellationReasonClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotCancelled => "not_cancelled",
            Self::UserRequested => "user_requested",
            Self::PolicyBlocked => "policy_blocked",
            Self::CeilingReached => "ceiling_reached",
            Self::BudgetExhausted => "budget_exhausted",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::GenericProviderError => "generic_provider_error",
        }
    }

    /// Whether this is the generic catch-all reason.
    pub const fn is_generic(self) -> bool {
        matches!(self, Self::GenericProviderError)
    }
}

/// Budget-band preflight that prices a run before it dispatches.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetBandPreflight {
    /// Coarse cost band projected for the run before dispatch.
    pub projected_cost_band: CostBandClass,
    /// Whether the projection is an estimate or a measured figure.
    pub projected_measurement: CostMeasurementClass,
    /// Who would be charged for the run's projected spend.
    pub charged: ChargedDisclosureClass,
    /// Review-safe label for the projected budget band.
    pub budget_band_label: String,
    /// Review-safe label naming the budget owner (no raw account id).
    pub budget_owner_label: String,
    /// How the preflight was acknowledged before dispatch.
    pub acknowledgement: PreflightAcknowledgementClass,
    /// Ceilings the preflight projects the run will bound.
    pub projected_ceilings: Vec<CeilingKindClass>,
    /// Review-safe explanation of the preflight posture.
    pub explanation_label: String,
}

/// Route receipt recording which provider/model/mode served a run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteReceipt {
    /// Provider/locality mode the run resolved to.
    pub resolved_mode: RoutePolicyModeClass,
    /// Review-safe label for the serving provider (no raw endpoint).
    pub provider_label: String,
    /// Review-safe label for the serving model.
    pub model_label: String,
    /// Reason the selected fallback hop exists in the chain.
    pub selected_hop_reason: FallbackHopReasonClass,
    /// Outcome of the selected fallback hop.
    pub selected_hop_outcome: FallbackHopOutcomeClass,
    /// Whether and how the route changed away from the primary.
    pub route_change: RouteChangeClass,
    /// Precise reason a downgrade or fallback fired.
    pub downgrade_reason: RouteDowngradeReasonClass,
    /// Review-safe explanation of the route posture.
    pub explanation_label: String,
}

/// Post-run spend receipt reconciling measured cost against the preflight.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostRunSpendReceipt {
    /// Coarse measured cost band for the completed run.
    pub final_cost_band: CostBandClass,
    /// Whether the final band is measured or merely estimated.
    pub measurement: CostMeasurementClass,
    /// Who is charged for the run's spend.
    pub charged: ChargedDisclosureClass,
    /// Review-safe label naming the budget owner (no raw account id).
    pub budget_owner_label: String,
    /// How the measured cost reconciles against the preflight projection.
    pub reconciliation: SpendReconciliationClass,
    /// True when the run's configured spend budget was spent.
    pub budget_exhausted: bool,
    /// Review-safe explanation of the spend posture.
    pub explanation_label: String,
}

/// Cumulative ceiling bounding a side-branch agent across its whole run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CumulativeCeiling {
    /// Kind of budget this cumulative ceiling bounds.
    pub kind: CeilingKindClass,
    /// How much of the cumulative budget the run has consumed.
    pub cumulative_consumption: CeilingConsumptionClass,
    /// How the cumulative ceiling is enforced when reached.
    pub enforcement: CeilingEnforcementClass,
    /// Review-safe label for the cumulative limit (no exact raw count).
    pub cumulative_limit_label: String,
    /// Review-safe explanation of the cumulative ceiling posture.
    pub explanation_label: String,
}

/// One checkpoint in a side-branch agent run's lifetime.
///
/// Checkpoints are strictly ordered and their cumulative cost band only
/// accumulates, so a checkpoint-aware cancellation can name exactly where the
/// run stopped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunCheckpoint {
    /// Position in the run, starting at zero and strictly increasing.
    pub sequence: u32,
    /// Review-safe label for the checkpoint.
    pub checkpoint_label: String,
    /// Coarse cumulative cost band at this checkpoint.
    pub cumulative_cost_band: CostBandClass,
    /// Cumulative ceiling consumption at this checkpoint.
    pub cumulative_consumption: CeilingConsumptionClass,
    /// Whether the run can be cancelled cleanly at this checkpoint.
    pub cancellable: bool,
    /// Review-safe note for the checkpoint.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub notes_label: String,
}

/// Checkpoint-aware, export-safe cancellation receipt for a run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CancellationExport {
    /// How the run was cancelled, if at all.
    pub class: CancellationClass,
    /// Precise reason the cancellation fired.
    pub reason: CancellationReasonClass,
    /// Checkpoint sequence the run was cancelled at, when cancelled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancelled_at_checkpoint: Option<u32>,
    /// Whether an export-safe cancellation receipt is available.
    pub export_safe_receipt_available: bool,
    /// Review-safe ref for the export-safe receipt.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub receipt_ref: String,
    /// Consumer surfaces that can read the same cancellation receipt.
    pub surface_parity: Vec<M5AiWorkflowConsumerSurface>,
    /// Review-safe explanation of the cancellation posture.
    pub explanation_label: String,
}

/// One downgrade rule that narrows a run's claim when a trigger fires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRunReceiptDowngradeRule {
    /// Trigger that fires this rule.
    pub trigger: M5AiWorkflowDowngradeTrigger,
    /// Qualification the run narrows to when the trigger fires.
    pub narrowed_to: M5AiWorkflowQualificationClass,
    /// Whether tooling enforces this rule automatically.
    pub auto_enforced: bool,
    /// Review-safe rationale for the narrowing.
    pub rationale: String,
}

/// One receipt row binding preflight, route, spend, ceiling, and cancellation
/// truth for a single AI run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRunReceiptRow {
    /// Stable receipt id.
    pub receipt_id: String,
    /// Stable id of the AI run.
    pub agent_run_id: String,
    /// Human-readable run label.
    pub run_label: String,
    /// Workflow lane the run belongs to.
    pub lane: M5AiWorkflowLane,
    /// Provider/locality mode the run resolves to.
    pub resolved_mode: RoutePolicyModeClass,
    /// Qualification class claimed for this run.
    pub claimed_qualification: M5AiWorkflowQualificationClass,
    /// Run state at mint time.
    pub run_state: LongRunningAgentRunStateClass,
    /// Budget-band preflight that priced the run before dispatch.
    pub preflight: BudgetBandPreflight,
    /// Route receipt recording which provider/model/mode served the run.
    pub route_receipt: RouteReceipt,
    /// Post-run spend receipt reconciling cost against the preflight.
    pub spend_receipt: PostRunSpendReceipt,
    /// Cumulative ceiling bounding a side-branch run; required for branch agents.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cumulative_ceiling: Option<CumulativeCeiling>,
    /// Ordered checkpoints across a side-branch run; required for branch agents.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub checkpoints: Vec<RunCheckpoint>,
    /// Checkpoint-aware, export-safe cancellation receipt.
    pub cancellation: CancellationExport,
    /// Downgrade rules that narrow the claim.
    pub downgrade_rules: Vec<AiRunReceiptDowngradeRule>,
    /// Rollback posture for a budget-policy change on this run.
    pub rollback_posture: M5AiWorkflowRollbackPosture,
    /// True when the rollback path has been drilled and verified.
    pub rollback_verified: bool,
    /// Required evidence packet refs backing the claim.
    pub evidence_packet_refs: Vec<String>,
}

impl AiRunReceiptRow {
    /// Whether this run carries a publicly claimed qualification.
    ///
    /// Stable, Beta, and Preview are claimed lanes; Experimental, Held, and
    /// Unavailable are not.
    pub fn is_claimed(&self) -> bool {
        matches!(
            self.claimed_qualification,
            M5AiWorkflowQualificationClass::Stable
                | M5AiWorkflowQualificationClass::Beta
                | M5AiWorkflowQualificationClass::Preview
        )
    }

    /// Whether this run is a branch or worktree agent.
    pub fn is_branch_agent(&self) -> bool {
        self.lane == M5AiWorkflowLane::BranchOrWorktreeAgent
    }

    /// Whether the run has reached a terminal state with a final spend receipt.
    pub fn is_terminal(&self) -> bool {
        self.run_state.is_terminal_stop()
            || matches!(
                self.run_state,
                LongRunningAgentRunStateClass::CompletedWithinBudget
            )
    }

    /// Whether the run was cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.cancellation.class.is_cancelled()
    }

    /// The checkpoint with `sequence`, if present.
    pub fn checkpoint(&self, sequence: u32) -> Option<&RunCheckpoint> {
        self.checkpoints.iter().find(|cp| cp.sequence == sequence)
    }

    /// Qualification this run narrows to when `trigger` fires.
    ///
    /// Returns the claimed qualification unchanged when no rule matches; this is
    /// the deterministic downgrade automation consumers and release tooling
    /// project instead of re-deriving narrowing locally.
    pub fn narrowed_qualification(
        &self,
        trigger: M5AiWorkflowDowngradeTrigger,
    ) -> M5AiWorkflowQualificationClass {
        self.downgrade_rules
            .iter()
            .find(|rule| rule.trigger == trigger)
            .map(|rule| rule.narrowed_to)
            .unwrap_or(self.claimed_qualification)
    }

    /// Renders a deterministic, review-safe inspector card for this run.
    pub fn render_inspector(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("### Receipt `{}`\n", self.receipt_id));
        out.push_str(&format!(
            "- Run: `{}` ({}) lane `{}`\n",
            self.agent_run_id,
            self.run_label,
            self.lane.as_str()
        ));
        out.push_str(&format!(
            "- Qualification: `{}`\n",
            self.claimed_qualification.as_str()
        ));
        out.push_str(&format!("- Mode: `{}`\n", self.resolved_mode.as_str()));
        out.push_str(&format!("- Run state: `{}`\n", self.run_state.as_str()));
        out.push_str(&format!(
            "- Preflight: `{}` / `{}` ({})\n",
            self.preflight.projected_cost_band.as_str(),
            self.preflight.acknowledgement.as_str(),
            self.preflight.budget_band_label
        ));
        out.push_str(&format!(
            "- Route: `{}` / `{}` / `{}` / `{}` ({})\n",
            self.route_receipt.resolved_mode.as_str(),
            self.route_receipt.selected_hop_outcome.as_str(),
            self.route_receipt.route_change.as_str(),
            self.route_receipt.downgrade_reason.as_str(),
            self.route_receipt.explanation_label
        ));
        out.push_str(&format!(
            "- Spend receipt: `{}` / `{}` / `{}` (budget exhausted: {}) ({})\n",
            self.spend_receipt.final_cost_band.as_str(),
            self.spend_receipt.reconciliation.as_str(),
            self.spend_receipt.charged.as_str(),
            self.spend_receipt.budget_exhausted,
            self.spend_receipt.explanation_label
        ));
        if let Some(ceiling) = &self.cumulative_ceiling {
            out.push_str(&format!(
                "- Cumulative ceiling: `{}` / `{}` / `{}` ({})\n",
                ceiling.kind.as_str(),
                ceiling.cumulative_consumption.as_str(),
                ceiling.enforcement.as_str(),
                ceiling.cumulative_limit_label
            ));
        }
        if !self.checkpoints.is_empty() {
            out.push_str("- Checkpoints:\n");
            for cp in &self.checkpoints {
                out.push_str(&format!(
                    "  - {} `{}` / `{}` (cancellable: {})\n",
                    cp.sequence,
                    cp.checkpoint_label,
                    cp.cumulative_cost_band.as_str(),
                    cp.cancellable
                ));
            }
        }
        out.push_str(&format!(
            "- Cancellation: `{}` / `{}` (export-safe: {}) ({})\n",
            self.cancellation.class.as_str(),
            self.cancellation.reason.as_str(),
            self.cancellation.export_safe_receipt_available,
            self.cancellation.explanation_label
        ));
        out
    }
}

/// Proof freshness block for the receipt packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRunReceiptProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows claimed runs.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`AiRunReceiptPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiRunReceiptPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// Receipt rows.
    pub receipts: Vec<AiRunReceiptRow>,
    /// Proof freshness block.
    pub proof_freshness: AiRunReceiptProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe AI spend-and-route receipt packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRunReceiptPacket {
    /// Record kind; must equal [`AI_RUN_RECEIPT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`AI_RUN_RECEIPT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// Receipt rows.
    pub receipts: Vec<AiRunReceiptRow>,
    /// Proof freshness block.
    pub proof_freshness: AiRunReceiptProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl AiRunReceiptPacket {
    /// Builds a receipt packet from stable-lane input.
    pub fn new(input: AiRunReceiptPacketInput) -> Self {
        Self {
            record_kind: AI_RUN_RECEIPT_RECORD_KIND.to_owned(),
            schema_version: AI_RUN_RECEIPT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            catalogue_label: input.catalogue_label,
            receipts: input.receipts,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the receipt invariants.
    pub fn validate(&self) -> Vec<AiRunReceiptViolation> {
        let mut violations = Vec::new();

        if self.record_kind != AI_RUN_RECEIPT_RECORD_KIND {
            violations.push(AiRunReceiptViolation::WrongRecordKind);
        }
        if self.schema_version != AI_RUN_RECEIPT_SCHEMA_VERSION {
            violations.push(AiRunReceiptViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.catalogue_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(AiRunReceiptViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_receipts_present(self, &mut violations);
        validate_lane_coverage(self, &mut violations);
        for receipt in &self.receipts {
            validate_receipt(receipt, &mut violations);
        }
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("receipt packet serializes"),
        ) {
            violations.push(AiRunReceiptViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Count of runs carrying a publicly claimed qualification.
    pub fn claimed_receipt_count(&self) -> usize {
        self.receipts.iter().filter(|r| r.is_claimed()).count()
    }

    /// Count of runs that were cancelled.
    pub fn cancelled_receipt_count(&self) -> usize {
        self.receipts.iter().filter(|r| r.is_cancelled()).count()
    }

    /// Count of branch or worktree agent runs.
    pub fn branch_agent_receipt_count(&self) -> usize {
        self.receipts.iter().filter(|r| r.is_branch_agent()).count()
    }

    /// Returns the receipt row for `receipt_id`, if present.
    pub fn receipt(&self, receipt_id: &str) -> Option<&AiRunReceiptRow> {
        self.receipts.iter().find(|r| r.receipt_id == receipt_id)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("receipt packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Spend And Route Receipts, Budget-Band Preflights, Cumulative Branch-Agent Ceilings, And Checkpoint-Aware Cancellation\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalogue_label));
        out.push_str(&format!(
            "- Receipts: {} ({} claimed, {} branch agents, {} cancelled)\n",
            self.receipts.len(),
            self.claimed_receipt_count(),
            self.branch_agent_receipt_count(),
            self.cancelled_receipt_count()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Receipt inspectors\n\n");
        for receipt in &self.receipts {
            out.push_str(&receipt.render_inspector());
            out.push('\n');
        }
        out
    }
}

/// Errors emitted when reading the checked-in receipt export.
#[derive(Debug)]
pub enum AiRunReceiptArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<AiRunReceiptViolation>),
}

impl fmt::Display for AiRunReceiptArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "receipt export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(formatter, "receipt export failed validation: {tokens}")
            }
        }
    }
}

impl Error for AiRunReceiptArtifactError {}

/// Validation failures emitted by [`AiRunReceiptPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AiRunReceiptViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The packet carries no receipts.
    NoReceipts,
    /// A receipt id appears more than once.
    DuplicateReceipt,
    /// A claimed workflow lane has no receipt covering it.
    LaneCoverageIncomplete,
    /// A receipt row is missing a required identity or label field.
    ReceiptRowIncomplete,
    /// A claimed charged run's preflight was not acknowledged before dispatch.
    PreflightUnacknowledged,
    /// A claimed run's preflight does not project a required ceiling.
    PreflightMissingRequiredCeiling,
    /// The route receipt's resolved mode disagrees with the run's mode.
    RouteReceiptModeMismatch,
    /// A route downgrade or fallback carries no precise reason.
    RouteDowngradeReasonMissing,
    /// A route downgrade collapsed into a generic provider error.
    RouteDowngradeReasonTooGeneric,
    /// A terminal run's spend receipt was never reconciled against its preflight.
    SpendReceiptNotReconciled,
    /// A charged spend receipt omits its charge-disclosure label.
    ChargedReceiptUndisclosed,
    /// A claimed run's spend receipt is only an estimate.
    EstimatedReceiptClaimsStable,
    /// A spend receipt above its projected band is not disclosed as an overrun.
    SpendOverrunNotDisclosed,
    /// A branch/worktree agent run has no cumulative ceiling.
    BranchAgentMissingCumulativeCeiling,
    /// A claimed branch agent's cumulative ceiling is not bounding.
    ClaimedCumulativeCeilingNotBounding,
    /// A branch/worktree agent run has no checkpoints.
    BranchAgentCheckpointsMissing,
    /// Checkpoints are not strictly ordered from zero.
    CheckpointsNotOrdered,
    /// A checkpoint's cumulative cost band decreases across the run.
    CheckpointCumulativeNotMonotonic,
    /// A cancellation class disagrees with the run state.
    CancellationStateMismatch,
    /// A cancelled run carries no export-safe receipt.
    CancellationNotExportSafe,
    /// A cancelled branch run does not name the checkpoint it stopped at.
    CancellationCheckpointMissing,
    /// A cancelled run's reason collapsed into a generic provider error.
    CancellationReasonTooGeneric,
    /// A cancelled run's receipt is not readable on the support-export surface.
    CancellationSurfaceParityIncomplete,
    /// A claimed run is missing required evidence packet refs.
    ClaimedReceiptMissingEvidence,
    /// A claimed run's reversing rollback path is not verified.
    ClaimedRollbackUnverified,
    /// A run has no downgrade rules.
    DowngradeRulesMissing,
    /// A run's downgrade rules omit the proof-stale trigger.
    DowngradeRuleMissingProofStale,
    /// A run's downgrade rules omit the provider-unavailable trigger.
    DowngradeRuleMissingProviderUnavailable,
    /// A downgrade rule does not narrow below the claimed qualification.
    DowngradeRuleNotNarrowing,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl AiRunReceiptViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NoReceipts => "no_receipts",
            Self::DuplicateReceipt => "duplicate_receipt",
            Self::LaneCoverageIncomplete => "lane_coverage_incomplete",
            Self::ReceiptRowIncomplete => "receipt_row_incomplete",
            Self::PreflightUnacknowledged => "preflight_unacknowledged",
            Self::PreflightMissingRequiredCeiling => "preflight_missing_required_ceiling",
            Self::RouteReceiptModeMismatch => "route_receipt_mode_mismatch",
            Self::RouteDowngradeReasonMissing => "route_downgrade_reason_missing",
            Self::RouteDowngradeReasonTooGeneric => "route_downgrade_reason_too_generic",
            Self::SpendReceiptNotReconciled => "spend_receipt_not_reconciled",
            Self::ChargedReceiptUndisclosed => "charged_receipt_undisclosed",
            Self::EstimatedReceiptClaimsStable => "estimated_receipt_claims_stable",
            Self::SpendOverrunNotDisclosed => "spend_overrun_not_disclosed",
            Self::BranchAgentMissingCumulativeCeiling => "branch_agent_missing_cumulative_ceiling",
            Self::ClaimedCumulativeCeilingNotBounding => "claimed_cumulative_ceiling_not_bounding",
            Self::BranchAgentCheckpointsMissing => "branch_agent_checkpoints_missing",
            Self::CheckpointsNotOrdered => "checkpoints_not_ordered",
            Self::CheckpointCumulativeNotMonotonic => "checkpoint_cumulative_not_monotonic",
            Self::CancellationStateMismatch => "cancellation_state_mismatch",
            Self::CancellationNotExportSafe => "cancellation_not_export_safe",
            Self::CancellationCheckpointMissing => "cancellation_checkpoint_missing",
            Self::CancellationReasonTooGeneric => "cancellation_reason_too_generic",
            Self::CancellationSurfaceParityIncomplete => "cancellation_surface_parity_incomplete",
            Self::ClaimedReceiptMissingEvidence => "claimed_receipt_missing_evidence",
            Self::ClaimedRollbackUnverified => "claimed_rollback_unverified",
            Self::DowngradeRulesMissing => "downgrade_rules_missing",
            Self::DowngradeRuleMissingProofStale => "downgrade_rule_missing_proof_stale",
            Self::DowngradeRuleMissingProviderUnavailable => {
                "downgrade_rule_missing_provider_unavailable"
            }
            Self::DowngradeRuleNotNarrowing => "downgrade_rule_not_narrowing",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in receipt export.
pub fn current_ai_run_receipt_export() -> Result<AiRunReceiptPacket, AiRunReceiptArtifactError> {
    let packet: AiRunReceiptPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella/support_export.json"
    )))
    .map_err(AiRunReceiptArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AiRunReceiptArtifactError::Validation(violations))
    }
}

/// Ordinal rank used to compare qualification severity for downgrade rules.
///
/// Higher means a stronger public claim, so a downgrade must move to a strictly
/// lower rank.
fn qualification_rank(class: M5AiWorkflowQualificationClass) -> u8 {
    match class {
        M5AiWorkflowQualificationClass::Unavailable => 0,
        M5AiWorkflowQualificationClass::Held => 1,
        M5AiWorkflowQualificationClass::Experimental => 2,
        M5AiWorkflowQualificationClass::Preview => 3,
        M5AiWorkflowQualificationClass::Beta => 4,
        M5AiWorkflowQualificationClass::Stable => 5,
    }
}

/// Ordinal rank used to compare coarse cost bands for accumulation and overrun.
///
/// Only the measured/charged bands are ordered here; the estimate band is left
/// out of magnitude comparisons so an unverified estimate never reads as an
/// overrun against a measured projection.
fn cost_magnitude_rank(band: CostBandClass) -> Option<u8> {
    match band {
        CostBandClass::BundledNoIncrementalCost => Some(0),
        CostBandClass::FreeTierRateLimited => Some(1),
        CostBandClass::FlatFeeSubscriptionBand => Some(2),
        CostBandClass::MeteredLowBand => Some(3),
        CostBandClass::MeteredMediumBand => Some(4),
        CostBandClass::MeteredHighBand => Some(5),
        CostBandClass::EstimatedUnverifiedBand => None,
    }
}

fn validate_source_contracts(
    packet: &AiRunReceiptPacket,
    violations: &mut Vec<AiRunReceiptViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        AI_RUN_RECEIPT_SCHEMA_REF,
        AI_RUN_RECEIPT_DOC_REF,
        PROVIDER_MODEL_REGISTRY_SCHEMA_REF,
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
        ROUTING_POLICY_SCHEMA_REF,
        AGENT_BUDGET_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(AiRunReceiptViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_receipts_present(
    packet: &AiRunReceiptPacket,
    violations: &mut Vec<AiRunReceiptViolation>,
) {
    if packet.receipts.is_empty() {
        violations.push(AiRunReceiptViolation::NoReceipts);
        return;
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for receipt in &packet.receipts {
        if !seen.insert(receipt.receipt_id.as_str()) {
            violations.push(AiRunReceiptViolation::DuplicateReceipt);
        }
    }
}

fn validate_lane_coverage(
    packet: &AiRunReceiptPacket,
    violations: &mut Vec<AiRunReceiptViolation>,
) {
    // The packet must show preflight, route, and spend receipts across inline
    // assist, patch review, and branch/worktree agents, so every lane needs at
    // least one row.
    for lane in M5AiWorkflowLane::ALL {
        if !packet.receipts.iter().any(|receipt| receipt.lane == lane) {
            violations.push(AiRunReceiptViolation::LaneCoverageIncomplete);
            return;
        }
    }
}

fn validate_receipt(receipt: &AiRunReceiptRow, violations: &mut Vec<AiRunReceiptViolation>) {
    if receipt.receipt_id.trim().is_empty()
        || receipt.agent_run_id.trim().is_empty()
        || receipt.run_label.trim().is_empty()
        || receipt.preflight.budget_band_label.trim().is_empty()
        || receipt.preflight.budget_owner_label.trim().is_empty()
        || receipt.preflight.explanation_label.trim().is_empty()
        || receipt.route_receipt.provider_label.trim().is_empty()
        || receipt.route_receipt.model_label.trim().is_empty()
        || receipt.route_receipt.explanation_label.trim().is_empty()
        || receipt.spend_receipt.budget_owner_label.trim().is_empty()
        || receipt.spend_receipt.explanation_label.trim().is_empty()
        || receipt.cancellation.explanation_label.trim().is_empty()
    {
        violations.push(AiRunReceiptViolation::ReceiptRowIncomplete);
    }

    validate_preflight(receipt, violations);
    validate_route_receipt(receipt, violations);
    validate_spend_receipt(receipt, violations);
    validate_cumulative_ceiling(receipt, violations);
    validate_checkpoints(receipt, violations);
    validate_cancellation(receipt, violations);

    if receipt.is_claimed() && receipt.evidence_packet_refs.is_empty() {
        violations.push(AiRunReceiptViolation::ClaimedReceiptMissingEvidence);
    }

    // A claimed run whose budget-policy change can be reversed must have drilled
    // that reversal; a non-applicable posture carries no reversal to verify.
    if receipt.is_claimed()
        && receipt.rollback_posture != M5AiWorkflowRollbackPosture::NotApplicable
        && !receipt.rollback_verified
    {
        violations.push(AiRunReceiptViolation::ClaimedRollbackUnverified);
    }

    validate_downgrade_rules(receipt, violations);
}

fn validate_preflight(receipt: &AiRunReceiptRow, violations: &mut Vec<AiRunReceiptViolation>) {
    let preflight = &receipt.preflight;

    // A claimed run that would charge the user must disclose and accept the
    // projected cost before it is spent.
    if receipt.is_claimed()
        && preflight.projected_cost_band.is_charged()
        && !preflight.acknowledgement.is_acknowledged()
    {
        violations.push(AiRunReceiptViolation::PreflightUnacknowledged);
    }

    // A claimed run must project the wall-clock, token, and tool-call ceilings so
    // the preflight promises a bounded run.
    if receipt.is_claimed() {
        let projected: BTreeSet<CeilingKindClass> =
            preflight.projected_ceilings.iter().copied().collect();
        if !CeilingKindClass::REQUIRED
            .iter()
            .all(|kind| projected.contains(kind))
        {
            violations.push(AiRunReceiptViolation::PreflightMissingRequiredCeiling);
        }
    }
}

fn validate_route_receipt(receipt: &AiRunReceiptRow, violations: &mut Vec<AiRunReceiptViolation>) {
    let route = &receipt.route_receipt;

    // The route receipt's mode must agree with the run's resolved mode.
    if route.resolved_mode != receipt.resolved_mode {
        violations.push(AiRunReceiptViolation::RouteReceiptModeMismatch);
    }

    // A route that changed off the primary must carry a precise, non-generic
    // reason rather than collapsing into an opaque provider error.
    if route.route_change.is_downgrade() {
        if route.downgrade_reason == RouteDowngradeReasonClass::NoDowngrade {
            violations.push(AiRunReceiptViolation::RouteDowngradeReasonMissing);
        } else if route.downgrade_reason.is_generic() {
            violations.push(AiRunReceiptViolation::RouteDowngradeReasonTooGeneric);
        }
    }
}

fn validate_spend_receipt(receipt: &AiRunReceiptRow, violations: &mut Vec<AiRunReceiptViolation>) {
    let spend = &receipt.spend_receipt;

    // A run that has reached a terminal state must reconcile its measured cost
    // against the preflight rather than leaving the receipt open.
    if receipt.is_terminal() && !spend.reconciliation.is_reconciled() {
        violations.push(AiRunReceiptViolation::SpendReceiptNotReconciled);
    }

    // A band that charges the user must say who is charged.
    if spend.final_cost_band.is_charged() && !spend.charged.is_disclosed() {
        violations.push(AiRunReceiptViolation::ChargedReceiptUndisclosed);
    }

    // An estimate-only post-run receipt may not back a Stable claim.
    if receipt.claimed_qualification == M5AiWorkflowQualificationClass::Stable
        && receipt.is_terminal()
        && (spend.final_cost_band == CostBandClass::EstimatedUnverifiedBand
            || spend.measurement == CostMeasurementClass::EstimateBand)
    {
        violations.push(AiRunReceiptViolation::EstimatedReceiptClaimsStable);
    }

    // A measured final band above the projected band is an overrun and must be
    // disclosed as one.
    if let (Some(final_rank), Some(projected_rank)) = (
        cost_magnitude_rank(spend.final_cost_band),
        cost_magnitude_rank(receipt.preflight.projected_cost_band),
    ) {
        if final_rank > projected_rank
            && spend.reconciliation != SpendReconciliationClass::OverProjectionDisclosed
        {
            violations.push(AiRunReceiptViolation::SpendOverrunNotDisclosed);
        }
    }
}

fn validate_cumulative_ceiling(
    receipt: &AiRunReceiptRow,
    violations: &mut Vec<AiRunReceiptViolation>,
) {
    match &receipt.cumulative_ceiling {
        None => {
            // A side-branch agent must bound its whole run with a cumulative
            // ceiling so it can never run away across checkpoints.
            if receipt.is_branch_agent() {
                violations.push(AiRunReceiptViolation::BranchAgentMissingCumulativeCeiling);
            }
        }
        Some(ceiling) => {
            if ceiling.cumulative_limit_label.trim().is_empty()
                || ceiling.explanation_label.trim().is_empty()
            {
                violations.push(AiRunReceiptViolation::ReceiptRowIncomplete);
            }
            // A claimed branch agent may never leave its cumulative ceiling
            // unbounded.
            if receipt.is_claimed()
                && receipt.is_branch_agent()
                && !ceiling.enforcement.is_bounding()
            {
                violations.push(AiRunReceiptViolation::ClaimedCumulativeCeilingNotBounding);
            }
        }
    }
}

fn validate_checkpoints(receipt: &AiRunReceiptRow, violations: &mut Vec<AiRunReceiptViolation>) {
    // A side-branch agent must carry checkpoints so a cancellation can name where
    // the run stopped.
    if receipt.is_branch_agent() && receipt.checkpoints.is_empty() {
        violations.push(AiRunReceiptViolation::BranchAgentCheckpointsMissing);
    }

    let mut ordered = true;
    for (index, checkpoint) in receipt.checkpoints.iter().enumerate() {
        if checkpoint.sequence as usize != index {
            ordered = false;
            break;
        }
        if checkpoint.checkpoint_label.trim().is_empty() {
            violations.push(AiRunReceiptViolation::ReceiptRowIncomplete);
        }
    }
    if !ordered {
        violations.push(AiRunReceiptViolation::CheckpointsNotOrdered);
    }

    // The cumulative cost band only accumulates, so its magnitude never decreases.
    let mut prev_rank = 0u8;
    for checkpoint in &receipt.checkpoints {
        if let Some(rank) = cost_magnitude_rank(checkpoint.cumulative_cost_band) {
            if rank < prev_rank {
                violations.push(AiRunReceiptViolation::CheckpointCumulativeNotMonotonic);
                break;
            }
            prev_rank = rank;
        }
    }
}

fn validate_cancellation(receipt: &AiRunReceiptRow, violations: &mut Vec<AiRunReceiptViolation>) {
    let cancellation = &receipt.cancellation;

    // A cancellation-driven run state must carry a matching cancellation class,
    // and any cancellation must agree with a terminal run state.
    let state_mismatch = match receipt.run_state {
        LongRunningAgentRunStateClass::CancelledByUser => {
            cancellation.class != CancellationClass::CancelledByUser
        }
        LongRunningAgentRunStateClass::CancelledByPolicy => {
            cancellation.class != CancellationClass::CancelledByPolicy
        }
        _ => cancellation.class.is_cancelled() && !receipt.run_state.is_terminal_stop(),
    };
    if state_mismatch {
        violations.push(AiRunReceiptViolation::CancellationStateMismatch);
    }

    if !cancellation.class.is_cancelled() {
        return;
    }

    // A cancellation must be a readable, export-safe receipt rather than an
    // opaque kill.
    if !cancellation.export_safe_receipt_available || cancellation.receipt_ref.trim().is_empty() {
        violations.push(AiRunReceiptViolation::CancellationNotExportSafe);
    }

    // A cancelled side-branch run must name the checkpoint it stopped at, and
    // that checkpoint must exist.
    if receipt.is_branch_agent() {
        match cancellation.cancelled_at_checkpoint {
            None => violations.push(AiRunReceiptViolation::CancellationCheckpointMissing),
            Some(sequence) => {
                if receipt.checkpoint(sequence).is_none() {
                    violations.push(AiRunReceiptViolation::CancellationCheckpointMissing);
                }
            }
        }
    }

    // A cancellation must carry a precise reason rather than a generic provider
    // error when a more precise reason exists.
    if cancellation.reason.is_generic() {
        violations.push(AiRunReceiptViolation::CancellationReasonTooGeneric);
    }

    // A cancelled run's receipt must at least be readable on the support-export
    // surface so support and release tooling can see the same truth.
    if !cancellation
        .surface_parity
        .contains(&M5AiWorkflowConsumerSurface::SupportExport)
    {
        violations.push(AiRunReceiptViolation::CancellationSurfaceParityIncomplete);
    }
}

fn validate_downgrade_rules(
    receipt: &AiRunReceiptRow,
    violations: &mut Vec<AiRunReceiptViolation>,
) {
    if receipt.downgrade_rules.is_empty() {
        violations.push(AiRunReceiptViolation::DowngradeRulesMissing);
        return;
    }

    if !receipt
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProofStale)
    {
        violations.push(AiRunReceiptViolation::DowngradeRuleMissingProofStale);
    }

    // Provider quota exhaustion and provider outages narrow through the
    // provider-unavailable trigger, so every row must carry it.
    if !receipt
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProviderUnavailable)
    {
        violations.push(AiRunReceiptViolation::DowngradeRuleMissingProviderUnavailable);
    }

    let claimed_rank = qualification_rank(receipt.claimed_qualification);
    for rule in &receipt.downgrade_rules {
        if qualification_rank(rule.narrowed_to) >= claimed_rank {
            violations.push(AiRunReceiptViolation::DowngradeRuleNotNarrowing);
            break;
        }
    }
}

fn validate_proof_freshness(
    packet: &AiRunReceiptPacket,
    violations: &mut Vec<AiRunReceiptViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(AiRunReceiptViolation::ProofFreshnessIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("https://")
                || lower.contains("http://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
