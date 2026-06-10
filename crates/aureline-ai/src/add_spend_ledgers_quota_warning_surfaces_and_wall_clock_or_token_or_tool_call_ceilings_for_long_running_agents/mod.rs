//! Spend ledgers, quota-warning surfaces, and wall-clock, token, or tool-call
//! ceilings for long-running agents.
//!
//! This module materializes the long-running-agent budget surface into one
//! export-safe truth packet whose unit of truth is an [`AgentBudgetRow`]: a
//! single long-running agent run binding the running spend ledger that accrues
//! its cost, the quota-warning surface that flags an approaching provider limit
//! before it is hit, and the wall-clock, token, and tool-call ceilings that keep
//! the run bounded so it can never spend, run, or call tools without limit. The
//! packet is the canonical budget-governance source for shell, docs, support
//! export, and release tooling; consumers project it instead of re-deriving
//! spend, quota, or ceiling state by hand.
//!
//! The packet refuses to present a long-running agent greener than its spend,
//! quota, and ceiling posture can back. Every claimed run must configure the
//! wall-clock, token, and tool-call ceilings, each enforced with a bounding stop
//! (a hard stop or a soft user prompt) so a claimed agent can never run unbounded.
//! A ceiling that has been reached under a bounding enforcement must have actually
//! stopped or paused the run, and the run state must agree, so the disclosed stop
//! is the stop that happened. A spend ledger only accumulates: its entries are
//! strictly ordered and its running cost band never decreases, its total matches
//! its last entry, a charged band discloses who is charged rather than leaving the
//! charge unknown, and an estimate-only ledger may not back a Stable claim. A
//! quota warning must be surfaced once the quota family enters its warning state,
//! and a provider quota that is exhausted or paused narrows the claim instead of
//! hiding behind a Stable label. Every run carries a closed set of downgrade rules
//! — including the stale-proof and provider-unavailable triggers — that narrow the
//! claim instead of hiding the route, reusing the qualification, downgrade-trigger,
//! and rollback-posture vocabularies frozen by the M5 AI workflow matrix lane and
//! the mode, quota, and cost-band vocabularies frozen by the routing-policy lane,
//! so no budget row may stay greener than its evidence.
//!
//! Raw provider endpoints, credential bodies, raw provider payloads, exact token
//! counts, and exact spend amounts stay outside the support boundary; the packet
//! carries modes, families, coarse bands, ceiling consumption classes, and
//! review-safe labels only.
//!
//! The boundary schema is
//! [`schemas/ai/add-spend-ledgers-quota-warning-surfaces-and-wall-clock-or-token-or-tool-call-ceilings-for-long-running-agents.schema.json`](../../../../schemas/ai/add-spend-ledgers-quota-warning-surfaces-and-wall-clock-or-token-or-tool-call-ceilings-for-long-running-agents.schema.json).
//! The contract doc is
//! [`docs/ai/m5/add_spend_ledgers_quota_warning_surfaces_and_wall_clock_or_token_or_tool_call_ceilings_for_long_running_agents.md`](../../../../docs/ai/m5/add_spend_ledgers_quota_warning_surfaces_and_wall_clock_or_token_or_tool_call_ceilings_for_long_running_agents.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/add_spend_ledgers_quota_warning_surfaces_and_wall_clock_or_token_or_tool_call_ceilings_for_long_running_agents/`](../../../../fixtures/ai/m5/add_spend_ledgers_quota_warning_surfaces_and_wall_clock_or_token_or_tool_call_ceilings_for_long_running_agents/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::{
    M5AiWorkflowDowngradeTrigger, M5AiWorkflowQualificationClass, M5AiWorkflowRollbackPosture,
    M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
};
use crate::implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains::{
    ChargedDisclosureClass, CostBandClass, CostMeasurementClass, QuotaFamilyClass, QuotaScopeClass,
    QuotaStateClass, RoutePolicyModeClass, ROUTING_POLICY_SCHEMA_REF,
};
use crate::materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors::PROVIDER_MODEL_REGISTRY_SCHEMA_REF;

/// Stable record-kind tag carried by [`AgentBudgetPacket`].
pub const AGENT_BUDGET_RECORD_KIND: &str =
    "add_spend_ledgers_quota_warning_surfaces_and_wall_clock_or_token_or_tool_call_ceilings_for_long_running_agents";

/// Schema version for long-running-agent budget records.
pub const AGENT_BUDGET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const AGENT_BUDGET_SCHEMA_REF: &str =
    "schemas/ai/add-spend-ledgers-quota-warning-surfaces-and-wall-clock-or-token-or-tool-call-ceilings-for-long-running-agents.schema.json";

/// Repo-relative path of the budget-governance contract doc.
pub const AGENT_BUDGET_DOC_REF: &str =
    "docs/ai/m5/add_spend_ledgers_quota_warning_surfaces_and_wall_clock_or_token_or_tool_call_ceilings_for_long_running_agents.md";

/// Repo-relative path of the protected fixture directory.
pub const AGENT_BUDGET_FIXTURE_DIR: &str =
    "fixtures/ai/m5/add_spend_ledgers_quota_warning_surfaces_and_wall_clock_or_token_or_tool_call_ceilings_for_long_running_agents";

/// Repo-relative path of the checked support-export artifact.
pub const AGENT_BUDGET_ARTIFACT_REF: &str =
    "artifacts/ai/m5/add_spend_ledgers_quota_warning_surfaces_and_wall_clock_or_token_or_tool_call_ceilings_for_long_running_agents/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const AGENT_BUDGET_SUMMARY_REF: &str =
    "artifacts/ai/m5/add_spend_ledgers_quota_warning_surfaces_and_wall_clock_or_token_or_tool_call_ceilings_for_long_running_agents.md";

/// Kind of ceiling that bounds a long-running agent run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CeilingKindClass {
    /// Elapsed wall-clock time bound.
    WallClock,
    /// Token-budget bound across the run.
    TokenBudget,
    /// Count of tool calls the run may make.
    ToolCallCount,
    /// Coarse per-run cost-budget bound.
    CostBudget,
    /// Count of agent steps or turns the run may take.
    AgentStepCount,
}

impl CeilingKindClass {
    /// Ceilings every claimed long-running agent must configure and bound.
    pub const REQUIRED: [Self; 3] = [Self::WallClock, Self::TokenBudget, Self::ToolCallCount];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WallClock => "wall_clock",
            Self::TokenBudget => "token_budget",
            Self::ToolCallCount => "tool_call_count",
            Self::CostBudget => "cost_budget",
            Self::AgentStepCount => "agent_step_count",
        }
    }

    /// Whether a claimed run must carry and bound this ceiling.
    pub const fn is_required(self) -> bool {
        matches!(
            self,
            Self::WallClock | Self::TokenBudget | Self::ToolCallCount
        )
    }
}

/// How much of a ceiling's budget a run has consumed at mint time.
///
/// Consumption is a coarse, review-safe class; the packet never carries an exact
/// elapsed time, token count, or call count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CeilingConsumptionClass {
    /// Comfortably below the warning threshold.
    WellWithinLimit,
    /// Climbing toward the warning threshold.
    ApproachingWarning,
    /// At or past the warning threshold; the warning has been raised.
    WarningRaised,
    /// The ceiling has been reached.
    CeilingReached,
}

impl CeilingConsumptionClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WellWithinLimit => "well_within_limit",
            Self::ApproachingWarning => "approaching_warning",
            Self::WarningRaised => "warning_raised",
            Self::CeilingReached => "ceiling_reached",
        }
    }

    /// Whether consumption has reached or passed the warning threshold.
    pub const fn at_or_past_warning(self) -> bool {
        matches!(self, Self::WarningRaised | Self::CeilingReached)
    }

    /// Whether the ceiling has been reached.
    pub const fn is_reached(self) -> bool {
        matches!(self, Self::CeilingReached)
    }
}

/// How a ceiling is enforced when its budget is reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CeilingEnforcementClass {
    /// The run halts automatically the moment the ceiling is reached.
    HardStopOnReach,
    /// The run pauses and asks the user before continuing past the ceiling.
    SoftPromptOnReach,
    /// The ceiling only raises a notice; the run keeps going.
    AdvisoryWarnOnly,
    /// The ceiling is configured but not enforced.
    Unenforced,
}

impl CeilingEnforcementClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HardStopOnReach => "hard_stop_on_reach",
            Self::SoftPromptOnReach => "soft_prompt_on_reach",
            Self::AdvisoryWarnOnly => "advisory_warn_only",
            Self::Unenforced => "unenforced",
        }
    }

    /// Whether this enforcement bounds the run by stopping or pausing it.
    ///
    /// Advisory and unenforced ceilings do not bound the run, so they may not
    /// stand in for a required ceiling on a claimed surface.
    pub const fn is_bounding(self) -> bool {
        matches!(self, Self::HardStopOnReach | Self::SoftPromptOnReach)
    }
}

/// Outcome of a ceiling at mint time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CeilingStopOutcomeClass {
    /// The ceiling has not fired.
    NotTriggered,
    /// The run was hard-stopped at this ceiling.
    HardStopped,
    /// The run is paused awaiting a user decision at this ceiling.
    PausedAwaitingUser,
    /// The user approved continuing past this ceiling and the run resumed.
    ResumedAfterApproval,
    /// An advisory notice fired but the run continued.
    AdvisoryNoticeOnly,
}

impl CeilingStopOutcomeClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotTriggered => "not_triggered",
            Self::HardStopped => "hard_stopped",
            Self::PausedAwaitingUser => "paused_awaiting_user",
            Self::ResumedAfterApproval => "resumed_after_approval",
            Self::AdvisoryNoticeOnly => "advisory_notice_only",
        }
    }

    /// Whether this outcome halted or paused the run.
    pub const fn halted_or_paused(self) -> bool {
        matches!(self, Self::HardStopped | Self::PausedAwaitingUser)
    }
}

/// One ceiling bounding a long-running agent run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentCeiling {
    /// Kind of budget this ceiling bounds.
    pub kind: CeilingKindClass,
    /// How much of the budget the run has consumed.
    pub consumption: CeilingConsumptionClass,
    /// How the ceiling is enforced when reached.
    pub enforcement: CeilingEnforcementClass,
    /// Outcome of the ceiling at mint time.
    pub stop_outcome: CeilingStopOutcomeClass,
    /// Review-safe label for the configured limit (no exact raw count).
    pub limit_label: String,
    /// Review-safe label for the warning threshold surfaced before the limit.
    pub warning_threshold_label: String,
    /// Review-safe explanation of the ceiling posture.
    pub explanation_label: String,
}

/// One entry in a long-running agent run's spend ledger.
///
/// Entries are strictly ordered and their running cost band only accumulates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpendLedgerEntry {
    /// Position in the ledger, starting at zero and strictly increasing.
    pub sequence: u32,
    /// Review-safe label for the run phase that accrued this entry.
    pub phase_label: String,
    /// Coarse cost band this phase added on its own.
    pub delta_cost_band: CostBandClass,
    /// Coarse running cost band after this phase.
    pub running_cost_band: CostBandClass,
    /// Review-safe note for the entry.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub notes_label: String,
}

/// Running spend ledger for a long-running agent run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpendLedger {
    /// Coarse running cost band for the whole run.
    pub running_cost_band: CostBandClass,
    /// Whether the running cost band is measured or merely estimated.
    pub measurement: CostMeasurementClass,
    /// Who is charged for the run's spend.
    pub charged: ChargedDisclosureClass,
    /// Review-safe label naming the budget owner (no raw account id).
    pub budget_owner_label: String,
    /// True when the run's configured spend budget has been spent.
    pub budget_exhausted: bool,
    /// Ordered ledger entries that accumulate to the running cost band.
    pub entries: Vec<SpendLedgerEntry>,
    /// Review-safe explanation of the spend posture.
    pub explanation_label: String,
}

/// Quota-warning surface for a long-running agent run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuotaWarningSurface {
    /// Quota family governing the run.
    pub family: QuotaFamilyClass,
    /// State of the quota at mint time.
    pub state: QuotaStateClass,
    /// Scope the quota is accounted against.
    pub scope: QuotaScopeClass,
    /// Whether the warning has actually been surfaced to the user.
    pub warning_surfaced: bool,
    /// Review-safe label for the warning threshold.
    pub warning_threshold_label: String,
    /// Review-safe label naming who owns the budget (no raw account id).
    pub budget_owner_label: String,
    /// Review-safe explanation of the quota posture.
    pub explanation_label: String,
}

/// Run state of a long-running agent at mint time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LongRunningAgentRunStateClass {
    /// Actively running within budget.
    Running,
    /// Completed within every ceiling and budget.
    CompletedWithinBudget,
    /// Stopped because a ceiling was reached.
    StoppedAtCeiling,
    /// Paused awaiting a user decision at a soft-prompt ceiling.
    PausedAwaitingUser,
    /// Stopped because the spend budget was exhausted.
    BudgetExhaustedStopped,
    /// Cancelled by the user.
    CancelledByUser,
    /// Cancelled by policy.
    CancelledByPolicy,
}

impl LongRunningAgentRunStateClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::CompletedWithinBudget => "completed_within_budget",
            Self::StoppedAtCeiling => "stopped_at_ceiling",
            Self::PausedAwaitingUser => "paused_awaiting_user",
            Self::BudgetExhaustedStopped => "budget_exhausted_stopped",
            Self::CancelledByUser => "cancelled_by_user",
            Self::CancelledByPolicy => "cancelled_by_policy",
        }
    }

    /// Whether the run is paused awaiting a user decision.
    pub const fn is_paused(self) -> bool {
        matches!(self, Self::PausedAwaitingUser)
    }

    /// Whether the run reached a terminal stop.
    pub const fn is_terminal_stop(self) -> bool {
        matches!(
            self,
            Self::StoppedAtCeiling
                | Self::BudgetExhaustedStopped
                | Self::CancelledByUser
                | Self::CancelledByPolicy
        )
    }
}

/// One downgrade rule that narrows a run's claim when a trigger fires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentBudgetDowngradeRule {
    /// Trigger that fires this rule.
    pub trigger: M5AiWorkflowDowngradeTrigger,
    /// Qualification the run narrows to when the trigger fires.
    pub narrowed_to: M5AiWorkflowQualificationClass,
    /// Whether tooling enforces this rule automatically.
    pub auto_enforced: bool,
    /// Review-safe rationale for the narrowing.
    pub rationale: String,
}

/// One budget row binding spend, quota-warning, and ceiling truth for a
/// long-running agent run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentBudgetRow {
    /// Stable budget id.
    pub budget_id: String,
    /// Stable id of the long-running agent run.
    pub agent_run_id: String,
    /// Human-readable agent label.
    pub agent_label: String,
    /// Provider/locality mode the run resolves to.
    pub resolved_mode: RoutePolicyModeClass,
    /// Qualification class claimed for this run.
    pub claimed_qualification: M5AiWorkflowQualificationClass,
    /// Run state at mint time.
    pub run_state: LongRunningAgentRunStateClass,
    /// Running spend ledger.
    pub spend_ledger: SpendLedger,
    /// Quota-warning surface.
    pub quota_warning: QuotaWarningSurface,
    /// Ceilings bounding the run; must include the wall-clock, token, and
    /// tool-call ceilings.
    pub ceilings: Vec<AgentCeiling>,
    /// Downgrade rules that narrow the claim.
    pub downgrade_rules: Vec<AgentBudgetDowngradeRule>,
    /// Rollback posture for a budget-policy change on this run.
    pub rollback_posture: M5AiWorkflowRollbackPosture,
    /// True when the rollback path has been drilled and verified.
    pub rollback_verified: bool,
    /// Required evidence packet refs backing the claim.
    pub evidence_packet_refs: Vec<String>,
}

impl AgentBudgetRow {
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

    /// The configured ceiling of `kind`, if present.
    pub fn ceiling(&self, kind: CeilingKindClass) -> Option<&AgentCeiling> {
        self.ceilings.iter().find(|ceiling| ceiling.kind == kind)
    }

    /// Whether the run configures every required ceiling.
    pub fn has_required_ceilings(&self) -> bool {
        CeilingKindClass::REQUIRED
            .iter()
            .all(|kind| self.ceiling(*kind).is_some())
    }

    /// Whether the run's provider quota currently blocks new dispatch.
    pub fn quota_blocks_dispatch(&self) -> bool {
        self.quota_warning.state.blocks_dispatch()
    }

    /// Whether the run has stopped or paused at a ceiling or budget.
    pub fn is_stopped(&self) -> bool {
        self.run_state.is_terminal_stop() || self.run_state.is_paused()
    }

    /// Whether any ceiling is at or past its warning threshold.
    pub fn has_ceiling_warning(&self) -> bool {
        self.ceilings
            .iter()
            .any(|ceiling| ceiling.consumption.at_or_past_warning())
    }

    /// Whether this run is surfacing a quota or ceiling warning.
    pub fn has_active_warning(&self) -> bool {
        self.has_ceiling_warning() || self.quota_warning.state == QuotaStateClass::Warning
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
        out.push_str(&format!("### Budget `{}`\n", self.budget_id));
        out.push_str(&format!(
            "- Agent: `{}` ({})\n",
            self.agent_run_id, self.agent_label
        ));
        out.push_str(&format!(
            "- Qualification: `{}`\n",
            self.claimed_qualification.as_str()
        ));
        out.push_str(&format!("- Mode: `{}`\n", self.resolved_mode.as_str()));
        out.push_str(&format!("- Run state: `{}`\n", self.run_state.as_str()));
        out.push_str(&format!(
            "- Spend ledger: `{}` / `{}` / `{}` (budget exhausted: {}) ({})\n",
            self.spend_ledger.running_cost_band.as_str(),
            self.spend_ledger.measurement.as_str(),
            self.spend_ledger.charged.as_str(),
            self.spend_ledger.budget_exhausted,
            self.spend_ledger.explanation_label
        ));
        out.push_str(&format!(
            "- Quota: `{}` / `{}` / `{}` (warning surfaced: {}) ({})\n",
            self.quota_warning.family.as_str(),
            self.quota_warning.state.as_str(),
            self.quota_warning.scope.as_str(),
            self.quota_warning.warning_surfaced,
            self.quota_warning.budget_owner_label
        ));
        out.push_str("- Ceilings:\n");
        for ceiling in &self.ceilings {
            out.push_str(&format!(
                "  - `{}` / `{}` / `{}` / `{}` ({})\n",
                ceiling.kind.as_str(),
                ceiling.consumption.as_str(),
                ceiling.enforcement.as_str(),
                ceiling.stop_outcome.as_str(),
                ceiling.limit_label
            ));
        }
        out
    }
}

/// Proof freshness block for the budget-governance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentBudgetProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows claimed runs.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`AgentBudgetPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentBudgetPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// Budget rows.
    pub budgets: Vec<AgentBudgetRow>,
    /// Proof freshness block.
    pub proof_freshness: AgentBudgetProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe long-running-agent budget-governance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentBudgetPacket {
    /// Record kind; must equal [`AGENT_BUDGET_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`AGENT_BUDGET_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// Budget rows.
    pub budgets: Vec<AgentBudgetRow>,
    /// Proof freshness block.
    pub proof_freshness: AgentBudgetProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl AgentBudgetPacket {
    /// Builds a budget-governance packet from stable-lane input.
    pub fn new(input: AgentBudgetPacketInput) -> Self {
        Self {
            record_kind: AGENT_BUDGET_RECORD_KIND.to_owned(),
            schema_version: AGENT_BUDGET_SCHEMA_VERSION,
            packet_id: input.packet_id,
            catalogue_label: input.catalogue_label,
            budgets: input.budgets,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the budget-governance invariants.
    pub fn validate(&self) -> Vec<AgentBudgetViolation> {
        let mut violations = Vec::new();

        if self.record_kind != AGENT_BUDGET_RECORD_KIND {
            violations.push(AgentBudgetViolation::WrongRecordKind);
        }
        if self.schema_version != AGENT_BUDGET_SCHEMA_VERSION {
            violations.push(AgentBudgetViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.catalogue_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(AgentBudgetViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_budgets_present(self, &mut violations);
        for budget in &self.budgets {
            validate_budget(budget, &mut violations);
        }
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("budget packet serializes"),
        ) {
            violations.push(AgentBudgetViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Count of runs carrying a publicly claimed qualification.
    pub fn claimed_budget_count(&self) -> usize {
        self.budgets.iter().filter(|b| b.is_claimed()).count()
    }

    /// Count of runs that have stopped or paused at a ceiling or budget.
    pub fn stopped_budget_count(&self) -> usize {
        self.budgets.iter().filter(|b| b.is_stopped()).count()
    }

    /// Count of runs surfacing a quota or ceiling warning.
    pub fn warning_budget_count(&self) -> usize {
        self.budgets
            .iter()
            .filter(|b| b.has_active_warning())
            .count()
    }

    /// Returns the budget row for `budget_id`, if present.
    pub fn budget(&self, budget_id: &str) -> Option<&AgentBudgetRow> {
        self.budgets.iter().find(|b| b.budget_id == budget_id)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("budget packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# Spend Ledgers, Quota-Warning Surfaces, And Ceilings For Long-Running Agents\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalogue_label));
        out.push_str(&format!(
            "- Budgets: {} ({} claimed, {} warning, {} stopped or paused)\n",
            self.budgets.len(),
            self.claimed_budget_count(),
            self.warning_budget_count(),
            self.stopped_budget_count()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Budget inspectors\n\n");
        for budget in &self.budgets {
            out.push_str(&budget.render_inspector());
            out.push('\n');
        }
        out
    }
}

/// Errors emitted when reading the checked-in budget-governance export.
#[derive(Debug)]
pub enum AgentBudgetArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<AgentBudgetViolation>),
}

impl fmt::Display for AgentBudgetArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "budget export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(formatter, "budget export failed validation: {tokens}")
            }
        }
    }
}

impl Error for AgentBudgetArtifactError {}

/// Validation failures emitted by [`AgentBudgetPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AgentBudgetViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The packet carries no budgets.
    NoBudgets,
    /// A budget id appears more than once.
    DuplicateBudget,
    /// A budget row is missing a required identity or label field.
    BudgetRowIncomplete,
    /// A spend ledger's entries are not strictly ordered from zero.
    LedgerEntriesNotOrdered,
    /// A spend ledger's running cost band decreases across entries.
    LedgerRunningNotMonotonic,
    /// A spend ledger's running total does not match its last entry.
    LedgerTotalMismatch,
    /// A charged spend ledger omits its charge-disclosure label.
    ChargedLedgerUndisclosed,
    /// A claimed run's spend ledger is only an estimate.
    EstimatedLedgerClaimsStable,
    /// A run is missing a required wall-clock, token, or tool-call ceiling.
    MissingRequiredCeiling,
    /// A ceiling kind appears more than once.
    DuplicateCeiling,
    /// A claimed run's required ceiling is not bounding.
    ClaimedCeilingNotBounding,
    /// A bounding ceiling was reached but the run did not stop or pause.
    CeilingReachedNotStopped,
    /// A ceiling at its warning threshold carries no warning label.
    CeilingWarningNotSurfaced,
    /// A quota in its warning state is not surfaced.
    QuotaWarningNotSurfaced,
    /// An exhausted or paused provider quota still claims Stable.
    ExhaustedQuotaClaimsStable,
    /// A hard-stopped ceiling does not agree with a terminal run state.
    RunStateStopMismatch,
    /// A paused ceiling does not agree with a paused run state.
    RunStatePauseMismatch,
    /// A claimed run is missing required evidence packet refs.
    ClaimedBudgetMissingEvidence,
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

impl AgentBudgetViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NoBudgets => "no_budgets",
            Self::DuplicateBudget => "duplicate_budget",
            Self::BudgetRowIncomplete => "budget_row_incomplete",
            Self::LedgerEntriesNotOrdered => "ledger_entries_not_ordered",
            Self::LedgerRunningNotMonotonic => "ledger_running_not_monotonic",
            Self::LedgerTotalMismatch => "ledger_total_mismatch",
            Self::ChargedLedgerUndisclosed => "charged_ledger_undisclosed",
            Self::EstimatedLedgerClaimsStable => "estimated_ledger_claims_stable",
            Self::MissingRequiredCeiling => "missing_required_ceiling",
            Self::DuplicateCeiling => "duplicate_ceiling",
            Self::ClaimedCeilingNotBounding => "claimed_ceiling_not_bounding",
            Self::CeilingReachedNotStopped => "ceiling_reached_not_stopped",
            Self::CeilingWarningNotSurfaced => "ceiling_warning_not_surfaced",
            Self::QuotaWarningNotSurfaced => "quota_warning_not_surfaced",
            Self::ExhaustedQuotaClaimsStable => "exhausted_quota_claims_stable",
            Self::RunStateStopMismatch => "run_state_stop_mismatch",
            Self::RunStatePauseMismatch => "run_state_pause_mismatch",
            Self::ClaimedBudgetMissingEvidence => "claimed_budget_missing_evidence",
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

/// Reads and validates the checked-in budget-governance export.
pub fn current_agent_budget_export() -> Result<AgentBudgetPacket, AgentBudgetArtifactError> {
    let packet: AgentBudgetPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/add_spend_ledgers_quota_warning_surfaces_and_wall_clock_or_token_or_tool_call_ceilings_for_long_running_agents/support_export.json"
    )))
    .map_err(AgentBudgetArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AgentBudgetArtifactError::Validation(violations))
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

/// Ordinal rank used to assert a spend ledger only accumulates.
///
/// Higher means a more expensive coarse band; an estimate band ranks above every
/// measured band so an unverified estimate never reads as cheaper than a measured
/// figure.
fn cost_band_rank(band: CostBandClass) -> u8 {
    match band {
        CostBandClass::BundledNoIncrementalCost => 0,
        CostBandClass::FreeTierRateLimited => 1,
        CostBandClass::FlatFeeSubscriptionBand => 2,
        CostBandClass::MeteredLowBand => 3,
        CostBandClass::MeteredMediumBand => 4,
        CostBandClass::MeteredHighBand => 5,
        CostBandClass::EstimatedUnverifiedBand => 6,
    }
}

fn validate_source_contracts(
    packet: &AgentBudgetPacket,
    violations: &mut Vec<AgentBudgetViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        AGENT_BUDGET_SCHEMA_REF,
        AGENT_BUDGET_DOC_REF,
        PROVIDER_MODEL_REGISTRY_SCHEMA_REF,
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
        ROUTING_POLICY_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(AgentBudgetViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_budgets_present(
    packet: &AgentBudgetPacket,
    violations: &mut Vec<AgentBudgetViolation>,
) {
    if packet.budgets.is_empty() {
        violations.push(AgentBudgetViolation::NoBudgets);
        return;
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for budget in &packet.budgets {
        if !seen.insert(budget.budget_id.as_str()) {
            violations.push(AgentBudgetViolation::DuplicateBudget);
        }
    }
}

fn validate_budget(budget: &AgentBudgetRow, violations: &mut Vec<AgentBudgetViolation>) {
    if budget.budget_id.trim().is_empty()
        || budget.agent_run_id.trim().is_empty()
        || budget.agent_label.trim().is_empty()
        || budget.spend_ledger.budget_owner_label.trim().is_empty()
        || budget.spend_ledger.explanation_label.trim().is_empty()
        || budget.quota_warning.budget_owner_label.trim().is_empty()
        || budget
            .quota_warning
            .warning_threshold_label
            .trim()
            .is_empty()
        || budget.quota_warning.explanation_label.trim().is_empty()
    {
        violations.push(AgentBudgetViolation::BudgetRowIncomplete);
    }

    validate_ledger(budget, violations);
    validate_ceilings(budget, violations);
    validate_quota_warning(budget, violations);
    validate_run_state(budget, violations);

    if budget.is_claimed() && budget.evidence_packet_refs.is_empty() {
        violations.push(AgentBudgetViolation::ClaimedBudgetMissingEvidence);
    }

    // A claimed run whose budget-policy change can be reversed must have drilled
    // that reversal; a non-applicable posture carries no reversal to verify.
    if budget.is_claimed()
        && budget.rollback_posture != M5AiWorkflowRollbackPosture::NotApplicable
        && !budget.rollback_verified
    {
        violations.push(AgentBudgetViolation::ClaimedRollbackUnverified);
    }

    validate_downgrade_rules(budget, violations);
}

fn validate_ledger(budget: &AgentBudgetRow, violations: &mut Vec<AgentBudgetViolation>) {
    let ledger = &budget.spend_ledger;

    // A run that has begun must carry at least one strictly-ordered ledger entry.
    let mut ordered = !ledger.entries.is_empty();
    for (index, entry) in ledger.entries.iter().enumerate() {
        if entry.sequence as usize != index {
            ordered = false;
            break;
        }
    }
    if !ordered {
        violations.push(AgentBudgetViolation::LedgerEntriesNotOrdered);
    }

    // The running cost band only accumulates, so its rank never decreases.
    let mut monotonic = true;
    let mut prev_rank = 0u8;
    for entry in &ledger.entries {
        let rank = cost_band_rank(entry.running_cost_band);
        if rank < prev_rank {
            monotonic = false;
            break;
        }
        prev_rank = rank;
    }
    if !monotonic {
        violations.push(AgentBudgetViolation::LedgerRunningNotMonotonic);
    }

    // The ledger total must equal the last entry's running band.
    if let Some(last) = ledger.entries.last() {
        if last.running_cost_band != ledger.running_cost_band {
            violations.push(AgentBudgetViolation::LedgerTotalMismatch);
        }
    }

    // A band that charges the user must say who is charged.
    if ledger.running_cost_band.is_charged() && !ledger.charged.is_disclosed() {
        violations.push(AgentBudgetViolation::ChargedLedgerUndisclosed);
    }

    // An estimate-only ledger may not back a Stable claim.
    if budget.claimed_qualification == M5AiWorkflowQualificationClass::Stable
        && (ledger.running_cost_band == CostBandClass::EstimatedUnverifiedBand
            || ledger.measurement == CostMeasurementClass::EstimateBand)
    {
        violations.push(AgentBudgetViolation::EstimatedLedgerClaimsStable);
    }
}

fn validate_ceilings(budget: &AgentBudgetRow, violations: &mut Vec<AgentBudgetViolation>) {
    let mut seen: BTreeSet<CeilingKindClass> = BTreeSet::new();
    for ceiling in &budget.ceilings {
        if !seen.insert(ceiling.kind) {
            violations.push(AgentBudgetViolation::DuplicateCeiling);
        }
    }

    if !budget.has_required_ceilings() {
        violations.push(AgentBudgetViolation::MissingRequiredCeiling);
    }

    for ceiling in &budget.ceilings {
        // A claimed run can never leave a required ceiling unbounded.
        if budget.is_claimed() && ceiling.kind.is_required() && !ceiling.enforcement.is_bounding() {
            violations.push(AgentBudgetViolation::ClaimedCeilingNotBounding);
        }

        // A bounding ceiling that has been reached must have actually stopped or
        // paused the run, so the disclosed stop is the stop that happened.
        if ceiling.consumption.is_reached()
            && ceiling.enforcement.is_bounding()
            && !ceiling.stop_outcome.halted_or_paused()
        {
            violations.push(AgentBudgetViolation::CeilingReachedNotStopped);
        }

        // A ceiling at or past its warning threshold must surface a warning label.
        if ceiling.consumption.at_or_past_warning()
            && ceiling.warning_threshold_label.trim().is_empty()
        {
            violations.push(AgentBudgetViolation::CeilingWarningNotSurfaced);
        }
    }
}

fn validate_quota_warning(budget: &AgentBudgetRow, violations: &mut Vec<AgentBudgetViolation>) {
    // A quota in its warning state must actually surface the warning.
    if budget.quota_warning.state == QuotaStateClass::Warning
        && !budget.quota_warning.warning_surfaced
    {
        violations.push(AgentBudgetViolation::QuotaWarningNotSurfaced);
    }

    // An exhausted or paused provider quota narrows the claim instead of hiding
    // behind a Stable label.
    if budget.quota_blocks_dispatch()
        && budget.claimed_qualification == M5AiWorkflowQualificationClass::Stable
    {
        violations.push(AgentBudgetViolation::ExhaustedQuotaClaimsStable);
    }
}

fn validate_run_state(budget: &AgentBudgetRow, violations: &mut Vec<AgentBudgetViolation>) {
    let hard_stopped = budget
        .ceilings
        .iter()
        .any(|ceiling| ceiling.stop_outcome == CeilingStopOutcomeClass::HardStopped);
    let paused = budget
        .ceilings
        .iter()
        .any(|ceiling| ceiling.stop_outcome == CeilingStopOutcomeClass::PausedAwaitingUser);

    // A hard-stopped ceiling, or an exhausted spend budget, must agree with a
    // terminal run state.
    if (hard_stopped || budget.spend_ledger.budget_exhausted)
        && !budget.run_state.is_terminal_stop()
    {
        violations.push(AgentBudgetViolation::RunStateStopMismatch);
    }

    // A paused ceiling without a hard stop must agree with a paused run state.
    if paused && !hard_stopped && !budget.run_state.is_paused() {
        violations.push(AgentBudgetViolation::RunStatePauseMismatch);
    }
}

fn validate_downgrade_rules(budget: &AgentBudgetRow, violations: &mut Vec<AgentBudgetViolation>) {
    if budget.downgrade_rules.is_empty() {
        violations.push(AgentBudgetViolation::DowngradeRulesMissing);
        return;
    }

    if !budget
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProofStale)
    {
        violations.push(AgentBudgetViolation::DowngradeRuleMissingProofStale);
    }

    // Provider quota exhaustion and provider outages narrow through the
    // provider-unavailable trigger, so every row must carry it.
    if !budget
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProviderUnavailable)
    {
        violations.push(AgentBudgetViolation::DowngradeRuleMissingProviderUnavailable);
    }

    let claimed_rank = qualification_rank(budget.claimed_qualification);
    for rule in &budget.downgrade_rules {
        if qualification_rank(rule.narrowed_to) >= claimed_rank {
            violations.push(AgentBudgetViolation::DowngradeRuleNotNarrowing);
            break;
        }
    }
}

fn validate_proof_freshness(
    packet: &AgentBudgetPacket,
    violations: &mut Vec<AgentBudgetViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(AgentBudgetViolation::ProofFreshnessIncomplete);
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
