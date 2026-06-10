//! Branch or worktree agent lifecycle, side-branch review packs, and merge-back
//! handoff.
//!
//! This module ships the canonical M5 packet that implements an AI agent run
//! isolated to a side branch or worktree, the reviewable pack it produces, and
//! the human-gated handoff that lands the work. It carries three bound blocks:
//!
//! - An [`AgentLifecycleBlock`] binds the agent run to one execution locus and a
//!   stage timeline (launch review → planning → isolated editing → validation →
//!   approval → review-ready → merge-back handoff → completion) where every
//!   mutating stage carries preview, approval, and checkpoint evidence and never
//!   writes outside its admitted isolation.
//! - A [`SideBranchReviewPackBlock`] binds the produced change to the upstream
//!   evidence-rich patch review lane by id (diff packet, validation receipt, and
//!   rollback handle refs), enumerates disclosed findings, and stays produced in
//!   isolation and reviewable before any merge.
//! - A [`MergeBackHandoffBlock`] binds the landing to a human: the agent may
//!   never self-merge or self-push to a protected destination, the merge
//!   requires human approval, and reviewable artifacts survive cleanup.
//!
//! The packet references upstream M4/M5 lanes by id rather than embedding their
//! content: it reuses the [`crate::qualify_background_branch_agent_lifecycle`]
//! execution-locus, operator-action, and cleanup vocabulary, and it cites the
//! [`crate::ship_evidence_rich_patch_review_with_diff_packets_validation_receipts_and_rollback_handles_across_apply_flows`]
//! evidence lane and the
//! [`crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents`]
//! workflow matrix.
//!
//! The record is export-safe. It carries refs, state tokens, coarse classes,
//! counts, and review labels only. Raw branch names, raw worktree paths, raw
//! diff bodies, raw patch text, raw logs, raw prompt bodies, source file bodies,
//! provider payloads, endpoint URLs, credentials, raw token counts, exact
//! prices, and billing-account ids stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ai/implement-branch-or-worktree-agent-lifecycle-side-branch-review-packs-and-merge-back-handoff.schema.json`](../../../../schemas/ai/implement-branch-or-worktree-agent-lifecycle-side-branch-review-packs-and-merge-back-handoff.schema.json).
//! The contract doc is
//! [`docs/ai/m5/implement_branch_or_worktree_agent_lifecycle_side_branch_review_packs_and_merge_back_handoff.md`](../../../../docs/ai/m5/implement_branch_or_worktree_agent_lifecycle_side_branch_review_packs_and_merge_back_handoff.md).
//! The frozen branch-agent base contract is
//! [`docs/ai/background_branch_agent_lifecycle.md`](../../../../docs/ai/background_branch_agent_lifecycle.md).

#[cfg(test)]
mod tests;

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::qualify_background_branch_agent_lifecycle::{
    BranchAgentCleanupDisposition, BranchAgentExecutionLocus, BranchAgentOperatorAction,
};

/// Stable record-kind tag carried by [`BranchWorktreeAgentLifecyclePacket`].
pub const BRANCH_WORKTREE_AGENT_LIFECYCLE_RECORD_KIND: &str =
    "branch_worktree_agent_lifecycle_implementation";

/// Schema version for branch or worktree agent lifecycle records.
pub const BRANCH_WORKTREE_AGENT_LIFECYCLE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const BRANCH_WORKTREE_AGENT_LIFECYCLE_SCHEMA_REF: &str =
    "schemas/ai/implement-branch-or-worktree-agent-lifecycle-side-branch-review-packs-and-merge-back-handoff.schema.json";

/// Repo-relative path of the M5 contract doc.
pub const BRANCH_WORKTREE_AGENT_LIFECYCLE_DOC_REF: &str =
    "docs/ai/m5/implement_branch_or_worktree_agent_lifecycle_side_branch_review_packs_and_merge_back_handoff.md";

/// Repo-relative path of the frozen branch-agent lifecycle base contract.
pub const BRANCH_WORKTREE_AGENT_LIFECYCLE_BASE_CONTRACT_REF: &str =
    "docs/ai/background_branch_agent_lifecycle.md";

/// Repo-relative path of the frozen evidence-rich patch review contract.
pub const BRANCH_WORKTREE_AGENT_LIFECYCLE_EVIDENCE_CONTRACT_REF: &str =
    "docs/ai/m5/ship_evidence_rich_patch_review_with_diff_packets_validation_receipts_and_rollback_handles_across_apply_flows.md";

/// Repo-relative path of the frozen M5 AI workflow matrix contract.
pub const BRANCH_WORKTREE_AGENT_LIFECYCLE_M5_MATRIX_CONTRACT_REF: &str =
    "docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md";

/// Repo-relative path of the protected fixture directory.
pub const BRANCH_WORKTREE_AGENT_LIFECYCLE_FIXTURE_DIR: &str =
    "fixtures/ai/m5/implement_branch_or_worktree_agent_lifecycle_side_branch_review_packs_and_merge_back_handoff";

/// Repo-relative path of the checked support-export artifact.
pub const BRANCH_WORKTREE_AGENT_LIFECYCLE_ARTIFACT_REF: &str =
    "artifacts/ai/m5/implement_branch_or_worktree_agent_lifecycle_side_branch_review_packs_and_merge_back_handoff/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const BRANCH_WORKTREE_AGENT_LIFECYCLE_SUMMARY_REF: &str =
    "artifacts/ai/m5/implement_branch_or_worktree_agent_lifecycle_side_branch_review_packs_and_merge_back_handoff.md";

/// Stage in the branch or worktree agent run lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentLifecycleStage {
    /// Pre-dispatch launch review shown before execution begins.
    LaunchReview,
    /// The agent is preparing a plan.
    Planning,
    /// The agent is editing inside its admitted isolated target.
    IsolatedEditing,
    /// The agent is validating the produced change.
    Validating,
    /// The agent is stopped at an approval gate.
    AwaitingApproval,
    /// The run produced a reviewable pack and is ready for human review.
    ReviewReady,
    /// The run is handed off to a human for merge-back.
    MergeBackHandoff,
    /// The run completed after a human landed or discarded the work.
    Completed,
    /// Drift requires fresh review or operator takeover before more writes.
    ReReviewRequired,
    /// The run was cancelled and preserved its checkpoint lineage.
    Cancelled,
    /// The run failed and preserved its review artifacts.
    Failed,
}

impl AgentLifecycleStage {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaunchReview => "launch_review",
            Self::Planning => "planning",
            Self::IsolatedEditing => "isolated_editing",
            Self::Validating => "validating",
            Self::AwaitingApproval => "awaiting_approval",
            Self::ReviewReady => "review_ready",
            Self::MergeBackHandoff => "merge_back_handoff",
            Self::Completed => "completed",
            Self::ReReviewRequired => "re_review_required",
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
        }
    }

    /// Whether the stage performs mutating writes inside the isolated target.
    const fn is_mutating(self) -> bool {
        matches!(self, Self::IsolatedEditing)
    }
}

/// Severity class for a side-branch review-pack finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewFindingSeverity {
    /// Must be resolved before merge.
    Blocker,
    /// High-priority finding.
    Major,
    /// Low-priority finding.
    Minor,
    /// Informational only.
    Info,
}

impl ReviewFindingSeverity {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocker => "blocker",
            Self::Major => "major",
            Self::Minor => "minor",
            Self::Info => "info",
        }
    }
}

/// State of the merge-back handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeBackState {
    /// The pack is awaiting human review.
    PendingReview,
    /// Review completed; the change is ready for a human-driven merge.
    ReadyForHumanMerge,
    /// A human landed the change. The agent never lands it.
    MergedByHuman,
    /// The change was discarded after preserving evidence.
    Discarded,
    /// The handoff is blocked by policy, trust, or drift.
    Blocked,
}

impl MergeBackState {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PendingReview => "pending_review",
            Self::ReadyForHumanMerge => "ready_for_human_merge",
            Self::MergedByHuman => "merged_by_human",
            Self::Discarded => "discarded",
            Self::Blocked => "blocked",
        }
    }

    /// Whether the change reached its merge-back destination.
    const fn reached_destination(self) -> bool {
        matches!(self, Self::MergedByHuman)
    }
}

/// Operator action offered on the merge-back handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeBackAction {
    /// Compare the produced change to its base.
    CompareToBase,
    /// Cherry-pick the change through a user-driven command.
    CherryPick,
    /// Open a pull request against the destination.
    OpenPullRequest,
    /// Request a human review before landing.
    RequestHumanReview,
    /// Rerun declared validation.
    RerunValidation,
    /// Discard or delete the side branch/worktree after review.
    DiscardBranch,
}

impl MergeBackAction {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompareToBase => "compare_to_base",
            Self::CherryPick => "cherry_pick",
            Self::OpenPullRequest => "open_pull_request",
            Self::RequestHumanReview => "request_human_review",
            Self::RerunValidation => "rerun_validation",
            Self::DiscardBranch => "discard_branch",
        }
    }
}

/// Consumer surface that must project this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurfaceClass {
    /// Desktop branch-agent workspace.
    DesktopAgentWorkspace,
    /// Desktop review workspace.
    DesktopReviewWorkspace,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Browser or companion follow-up.
    BrowserCompanion,
    /// Support/export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
}

impl ConsumerSurfaceClass {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesktopAgentWorkspace,
        Self::DesktopReviewWorkspace,
        Self::CliHeadless,
        Self::BrowserCompanion,
        Self::SupportExport,
        Self::Diagnostics,
    ];

    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopAgentWorkspace => "desktop_agent_workspace",
            Self::DesktopReviewWorkspace => "desktop_review_workspace",
            Self::CliHeadless => "cli_headless",
            Self::BrowserCompanion => "browser_companion",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
        }
    }
}

/// Qualification class for a consumer surface projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceQualificationClass {
    /// Surface qualifies for the Stable claim.
    Stable,
    /// Surface is narrowed to Beta.
    Beta,
    /// Surface is narrowed to Preview.
    Preview,
    /// Surface is experimental.
    Experimental,
    /// Surface is unavailable on this row.
    Unavailable,
}

impl SurfaceQualificationClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
        }
    }

    const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that can narrow this lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Required provider or model is unavailable.
    ProviderUnavailable,
    /// Workspace trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond the qualified boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
    /// Isolation could not be held; the agent wrote outside its target.
    IsolationBreach,
    /// The side-branch review pack is missing required evidence refs.
    ReviewPackMissing,
    /// Merge-back proceeded without human approval.
    MergeBackUnapproved,
}

impl DowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ProviderUnavailable,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
        Self::IsolationBreach,
        Self::ReviewPackMissing,
        Self::MergeBackUnapproved,
    ];

    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
            Self::IsolationBreach => "isolation_breach",
            Self::ReviewPackMissing => "review_pack_missing",
            Self::MergeBackUnapproved => "merge_back_unapproved",
        }
    }
}

/// One stage row in the agent lifecycle timeline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentLifecycleStageRow {
    /// Lifecycle stage this row covers.
    pub stage: AgentLifecycleStage,
    /// Checkpoint ref captured at this stage.
    pub checkpoint_ref: String,
    /// True when a preview was shown before any mutating write at this stage.
    pub preview_shown: bool,
    /// True when approval was required to leave this stage.
    pub approval_required: bool,
    /// True when approval was granted at this stage.
    pub approval_granted: bool,
    /// True when already-produced review artifacts are retained at this stage.
    pub review_artifacts_preserved: bool,
    /// True when this stage mutated state outside its admitted isolation. Must
    /// be false on a healthy run.
    pub mutated_outside_isolation: bool,
    /// Operator actions available at this stage.
    pub operator_actions: Vec<BranchAgentOperatorAction>,
}

/// Agent lifecycle block binding the run to one execution locus and a stage
/// timeline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentLifecycleBlock {
    /// Execution locus the run is admitted to operate in.
    pub execution_locus: BranchAgentExecutionLocus,
    /// Current lifecycle stage.
    pub current_stage: AgentLifecycleStage,
    /// Base branch or base commit ref the run started from.
    pub base_ref: String,
    /// Side branch identity ref.
    pub branch_identity_ref: String,
    /// Worktree identity ref; absent when the locus carries no worktree.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worktree_identity_ref: Option<String>,
    /// True when the launch review was disclosed before execution began.
    pub launch_review_disclosed: bool,
    /// True when isolation held for every mutating stage.
    pub isolation_verified: bool,
    /// True when automation stayed bounded by declared stop conditions.
    pub automation_bounded_by_stop_conditions: bool,
    /// Lifecycle stage rows in execution order.
    pub stage_rows: Vec<AgentLifecycleStageRow>,
}

/// One disclosed finding row in a side-branch review pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackFindingRow {
    /// Opaque finding id.
    pub finding_id: String,
    /// Finding severity.
    pub severity: ReviewFindingSeverity,
    /// Opaque file ref the finding applies to.
    pub file_ref: String,
    /// True when the finding is disclosed in the pack.
    pub disclosed_in_pack: bool,
    /// True when the finding has been resolved.
    pub resolved: bool,
}

/// Side-branch review pack block binding the produced change to upstream
/// evidence by id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SideBranchReviewPackBlock {
    /// Stable review-pack id.
    pub pack_id: String,
    /// Base branch or base commit ref.
    pub base_ref: String,
    /// Side branch head ref.
    pub head_ref: String,
    /// Diff packet ref into the evidence-rich patch review lane.
    pub diff_packet_ref: String,
    /// Validation receipt ref into the evidence-rich patch review lane.
    pub validation_receipt_ref: String,
    /// Rollback handle ref into the evidence-rich patch review lane.
    pub rollback_handle_ref: String,
    /// Evidence packet ref binding the pack to finalized evidence lineage.
    pub evidence_packet_ref: String,
    /// Disclosed finding rows.
    pub finding_rows: Vec<ReviewPackFindingRow>,
    /// True when a compare-to-base action is available on the pack.
    pub compare_to_base_available: bool,
    /// True when the pack was produced inside the run's isolation.
    pub produced_in_isolation: bool,
    /// True when human review is required before merge.
    pub review_required_before_merge: bool,
}

/// Merge-back handoff block binding the landing to a human.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeBackHandoffBlock {
    /// Stable handoff id.
    pub handoff_id: String,
    /// Merge-back state.
    pub state: MergeBackState,
    /// Opaque destination branch ref.
    pub destination_ref: String,
    /// True when the destination is a protected branch.
    pub destination_protected: bool,
    /// True when merge-back requires human approval. Must be true.
    pub requires_human_approval: bool,
    /// True when a human granted merge-back approval.
    pub human_approval_granted: bool,
    /// True when the agent is forbidden from self-merging. Must be true.
    pub self_merge_forbidden: bool,
    /// True when the agent is forbidden from self-pushing to a protected
    /// destination. Must be true.
    pub protected_destination_self_push_forbidden: bool,
    /// Operator actions available on the handoff.
    pub available_actions: Vec<MergeBackAction>,
    /// Cleanup disposition for the side branch/worktree.
    pub cleanup_disposition: BranchAgentCleanupDisposition,
    /// True when reviewable artifacts survive cleanup.
    pub review_artifacts_survive_cleanup: bool,
}

/// One cross-surface consumer-parity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerSurfaceParityRow {
    /// Consumer surface this row covers.
    pub surface: ConsumerSurfaceClass,
    /// True when this surface shows agent lifecycle truth.
    pub shows_lifecycle: bool,
    /// True when this surface shows the side-branch review pack.
    pub shows_review_pack: bool,
    /// True when this surface shows merge-back handoff truth.
    pub shows_merge_back_handoff: bool,
    /// True when this surface is reachable for this packet.
    pub reachable: bool,
    /// Qualification class for this surface projection.
    pub qualification: SurfaceQualificationClass,
    /// True when this surface claims the Stable lane.
    pub claimed_stable: bool,
}

/// Constructor input for [`BranchWorktreeAgentLifecyclePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchWorktreeAgentLifecyclePacketInput {
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical agent run id shared across surfaces, evidence, and handoff.
    pub agent_run_id: String,
    /// Display label.
    pub display_label: String,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this packet was evaluated under.
    pub policy_epoch_ref: String,
    /// Agent lifecycle block.
    pub lifecycle: AgentLifecycleBlock,
    /// Side-branch review pack block.
    pub review_pack: SideBranchReviewPackBlock,
    /// Merge-back handoff block.
    pub merge_back: MergeBackHandoffBlock,
    /// Cross-surface consumer-parity rows.
    pub consumer_surface_parity: Vec<ConsumerSurfaceParityRow>,
    /// Downgrade triggers that apply to this packet.
    pub downgrade_triggers: Vec<DowngradeTrigger>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe branch or worktree agent lifecycle record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchWorktreeAgentLifecyclePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical agent run id shared across surfaces, evidence, and handoff.
    pub agent_run_id: String,
    /// Display label.
    pub display_label: String,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this packet was evaluated under.
    pub policy_epoch_ref: String,
    /// Agent lifecycle block.
    pub lifecycle: AgentLifecycleBlock,
    /// Side-branch review pack block.
    pub review_pack: SideBranchReviewPackBlock,
    /// Merge-back handoff block.
    pub merge_back: MergeBackHandoffBlock,
    /// Cross-surface consumer-parity rows.
    pub consumer_surface_parity: Vec<ConsumerSurfaceParityRow>,
    /// Downgrade triggers that apply to this packet.
    pub downgrade_triggers: Vec<DowngradeTrigger>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl BranchWorktreeAgentLifecyclePacket {
    /// Builds a branch or worktree agent lifecycle packet from the stable-lane
    /// input.
    pub fn new(input: BranchWorktreeAgentLifecyclePacketInput) -> Self {
        Self {
            record_kind: BRANCH_WORKTREE_AGENT_LIFECYCLE_RECORD_KIND.to_owned(),
            schema_version: BRANCH_WORKTREE_AGENT_LIFECYCLE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            agent_run_id: input.agent_run_id,
            display_label: input.display_label,
            trust_state_token: input.trust_state_token,
            policy_epoch_ref: input.policy_epoch_ref,
            lifecycle: input.lifecycle,
            review_pack: input.review_pack,
            merge_back: input.merge_back,
            consumer_surface_parity: input.consumer_surface_parity,
            downgrade_triggers: input.downgrade_triggers,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the branch or worktree agent lifecycle packet's stable-line
    /// invariants.
    pub fn validate(&self) -> Vec<BranchWorktreeAgentLifecycleViolation> {
        let mut violations = Vec::new();
        if self.record_kind != BRANCH_WORKTREE_AGENT_LIFECYCLE_RECORD_KIND {
            violations.push(BranchWorktreeAgentLifecycleViolation::WrongRecordKind);
        }
        if self.schema_version != BRANCH_WORKTREE_AGENT_LIFECYCLE_SCHEMA_VERSION {
            violations.push(BranchWorktreeAgentLifecycleViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.agent_run_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.trust_state_token.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(BranchWorktreeAgentLifecycleViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_lifecycle(self, &mut violations);
        validate_review_pack(self, &mut violations);
        validate_merge_back(self, &mut violations);
        validate_consumer_surface_parity(self, &mut violations);
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("branch or worktree agent lifecycle packet serializes"),
        ) {
            violations.push(BranchWorktreeAgentLifecycleViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("branch or worktree agent lifecycle packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let blocker_count = self
            .review_pack
            .finding_rows
            .iter()
            .filter(|row| row.severity == ReviewFindingSeverity::Blocker)
            .count();
        let stable_surfaces = self
            .consumer_surface_parity
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# Branch or Worktree Agent Lifecycle\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Agent run: `{}`\n", self.agent_run_id));
        out.push_str(&format!(
            "- Lifecycle: `{}` (locus: `{}`, stages: {})\n",
            self.lifecycle.current_stage.as_str(),
            self.lifecycle.execution_locus.as_str(),
            self.lifecycle.stage_rows.len()
        ));
        out.push_str(&format!(
            "- Review pack: `{}` ({} findings, {} blocker)\n",
            self.review_pack.pack_id,
            self.review_pack.finding_rows.len(),
            blocker_count
        ));
        out.push_str(&format!(
            "- Merge-back: `{}` (human approval required: {} / granted: {})\n",
            self.merge_back.state.as_str(),
            self.merge_back.requires_human_approval,
            self.merge_back.human_approval_granted
        ));
        out.push_str(&format!(
            "- Surface parity: {} surfaces ({} stable)\n",
            self.consumer_surface_parity.len(),
            stable_surfaces
        ));
        out.push_str(&format!(
            "- Downgrade triggers: {}\n",
            self.downgrade_triggers.len()
        ));
        out
    }
}

/// Validation failures emitted by [`BranchWorktreeAgentLifecyclePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BranchWorktreeAgentLifecycleViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// Lifecycle block is incomplete.
    LifecycleIncomplete,
    /// Launch review was not disclosed before execution.
    LaunchReviewMissing,
    /// A mutating stage wrote outside its admitted isolation.
    IsolationBreached,
    /// A mutating stage applied without preview or approval.
    StageAppliedWithoutPreviewOrApproval,
    /// A stage row does not preserve review artifacts.
    StageDoesNotPreserveArtifacts,
    /// The review pack is missing required evidence refs.
    ReviewPackIncomplete,
    /// The review pack is not produced inside the run's isolation.
    ReviewPackNotIsolated,
    /// A finding reached the pack without being disclosed.
    HiddenFinding,
    /// Merge-back does not require human approval.
    MergeBackApprovalNotRequired,
    /// Merge-back reached its destination without human approval.
    MergeBackWithoutApproval,
    /// The packet permits agent self-merge or protected destination self-push.
    UnsafeLandingPosture,
    /// Cleanup does not preserve reviewable artifacts.
    CleanupLosesReviewArtifacts,
    /// Merge-back is missing required operator actions.
    MergeBackActionsIncomplete,
    /// A consumer surface is not covered by the parity rows.
    ConsumerSurfaceCoverageMissing,
    /// A surface claims Stable without qualifying for it.
    StableClaimNotQualified,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl BranchWorktreeAgentLifecycleViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::LifecycleIncomplete => "lifecycle_incomplete",
            Self::LaunchReviewMissing => "launch_review_missing",
            Self::IsolationBreached => "isolation_breached",
            Self::StageAppliedWithoutPreviewOrApproval => {
                "stage_applied_without_preview_or_approval"
            }
            Self::StageDoesNotPreserveArtifacts => "stage_does_not_preserve_artifacts",
            Self::ReviewPackIncomplete => "review_pack_incomplete",
            Self::ReviewPackNotIsolated => "review_pack_not_isolated",
            Self::HiddenFinding => "hidden_finding",
            Self::MergeBackApprovalNotRequired => "merge_back_approval_not_required",
            Self::MergeBackWithoutApproval => "merge_back_without_approval",
            Self::UnsafeLandingPosture => "unsafe_landing_posture",
            Self::CleanupLosesReviewArtifacts => "cleanup_loses_review_artifacts",
            Self::MergeBackActionsIncomplete => "merge_back_actions_incomplete",
            Self::ConsumerSurfaceCoverageMissing => "consumer_surface_coverage_missing",
            Self::StableClaimNotQualified => "stable_claim_not_qualified",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

impl fmt::Display for BranchWorktreeAgentLifecycleViolation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl Error for BranchWorktreeAgentLifecycleViolation {}

/// Errors emitted when reading the checked-in lifecycle export.
#[derive(Debug)]
pub enum BranchWorktreeAgentLifecycleArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<BranchWorktreeAgentLifecycleViolation>),
}

impl fmt::Display for BranchWorktreeAgentLifecycleArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "branch or worktree agent lifecycle export parse failed: {error}"
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
                    "branch or worktree agent lifecycle export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for BranchWorktreeAgentLifecycleArtifactError {}

/// Returns the checked-in branch or worktree agent lifecycle export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or
/// validate.
pub fn current_stable_branch_worktree_agent_lifecycle_export(
) -> Result<BranchWorktreeAgentLifecyclePacket, BranchWorktreeAgentLifecycleArtifactError> {
    let packet: BranchWorktreeAgentLifecyclePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/implement_branch_or_worktree_agent_lifecycle_side_branch_review_packs_and_merge_back_handoff/support_export.json"
    )))
    .map_err(BranchWorktreeAgentLifecycleArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(BranchWorktreeAgentLifecycleArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &BranchWorktreeAgentLifecyclePacket,
    violations: &mut Vec<BranchWorktreeAgentLifecycleViolation>,
) {
    for required in [
        BRANCH_WORKTREE_AGENT_LIFECYCLE_DOC_REF,
        BRANCH_WORKTREE_AGENT_LIFECYCLE_SCHEMA_REF,
        BRANCH_WORKTREE_AGENT_LIFECYCLE_BASE_CONTRACT_REF,
        BRANCH_WORKTREE_AGENT_LIFECYCLE_EVIDENCE_CONTRACT_REF,
        BRANCH_WORKTREE_AGENT_LIFECYCLE_M5_MATRIX_CONTRACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(BranchWorktreeAgentLifecycleViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_lifecycle(
    packet: &BranchWorktreeAgentLifecyclePacket,
    violations: &mut Vec<BranchWorktreeAgentLifecycleViolation>,
) {
    let lifecycle = &packet.lifecycle;
    if lifecycle.base_ref.trim().is_empty()
        || lifecycle.branch_identity_ref.trim().is_empty()
        || lifecycle.stage_rows.is_empty()
    {
        violations.push(BranchWorktreeAgentLifecycleViolation::LifecycleIncomplete);
        return;
    }
    if !lifecycle.launch_review_disclosed
        || !lifecycle
            .stage_rows
            .iter()
            .any(|row| row.stage == AgentLifecycleStage::LaunchReview)
    {
        violations.push(BranchWorktreeAgentLifecycleViolation::LaunchReviewMissing);
    }
    if !lifecycle.isolation_verified {
        violations.push(BranchWorktreeAgentLifecycleViolation::IsolationBreached);
    }
    for row in &lifecycle.stage_rows {
        if row.mutated_outside_isolation {
            violations.push(BranchWorktreeAgentLifecycleViolation::IsolationBreached);
        }
        if row.stage.is_mutating() && (!row.preview_shown || !row.approval_granted) {
            violations
                .push(BranchWorktreeAgentLifecycleViolation::StageAppliedWithoutPreviewOrApproval);
        }
        if !row.review_artifacts_preserved {
            violations.push(BranchWorktreeAgentLifecycleViolation::StageDoesNotPreserveArtifacts);
        }
    }
}

fn validate_review_pack(
    packet: &BranchWorktreeAgentLifecyclePacket,
    violations: &mut Vec<BranchWorktreeAgentLifecycleViolation>,
) {
    let pack = &packet.review_pack;
    if pack.pack_id.trim().is_empty()
        || pack.base_ref.trim().is_empty()
        || pack.head_ref.trim().is_empty()
        || pack.diff_packet_ref.trim().is_empty()
        || pack.validation_receipt_ref.trim().is_empty()
        || pack.rollback_handle_ref.trim().is_empty()
        || pack.evidence_packet_ref.trim().is_empty()
        || !pack.review_required_before_merge
    {
        violations.push(BranchWorktreeAgentLifecycleViolation::ReviewPackIncomplete);
    }
    if !pack.produced_in_isolation {
        violations.push(BranchWorktreeAgentLifecycleViolation::ReviewPackNotIsolated);
    }
    for finding in &pack.finding_rows {
        if !finding.disclosed_in_pack {
            violations.push(BranchWorktreeAgentLifecycleViolation::HiddenFinding);
            break;
        }
    }
}

fn validate_merge_back(
    packet: &BranchWorktreeAgentLifecyclePacket,
    violations: &mut Vec<BranchWorktreeAgentLifecycleViolation>,
) {
    let merge = &packet.merge_back;
    if merge.handoff_id.trim().is_empty() || merge.destination_ref.trim().is_empty() {
        violations.push(BranchWorktreeAgentLifecycleViolation::ReviewPackIncomplete);
    }
    if !merge.requires_human_approval {
        violations.push(BranchWorktreeAgentLifecycleViolation::MergeBackApprovalNotRequired);
    }
    if merge.state.reached_destination() && !merge.human_approval_granted {
        violations.push(BranchWorktreeAgentLifecycleViolation::MergeBackWithoutApproval);
    }
    if !(merge.self_merge_forbidden && merge.protected_destination_self_push_forbidden) {
        violations.push(BranchWorktreeAgentLifecycleViolation::UnsafeLandingPosture);
    }
    if !merge.review_artifacts_survive_cleanup {
        violations.push(BranchWorktreeAgentLifecycleViolation::CleanupLosesReviewArtifacts);
    }
    for required in [
        MergeBackAction::CompareToBase,
        MergeBackAction::RequestHumanReview,
        MergeBackAction::DiscardBranch,
    ] {
        if !merge.available_actions.contains(&required) {
            violations.push(BranchWorktreeAgentLifecycleViolation::MergeBackActionsIncomplete);
            break;
        }
    }
}

fn validate_consumer_surface_parity(
    packet: &BranchWorktreeAgentLifecyclePacket,
    violations: &mut Vec<BranchWorktreeAgentLifecycleViolation>,
) {
    let mut seen = std::collections::HashSet::new();
    for row in &packet.consumer_surface_parity {
        seen.insert(row.surface);
        if row.claimed_stable && !row.reachable {
            violations.push(BranchWorktreeAgentLifecycleViolation::StableClaimNotQualified);
        }
    }
    for required in ConsumerSurfaceClass::ALL {
        if !seen.contains(&required) {
            violations.push(BranchWorktreeAgentLifecycleViolation::ConsumerSurfaceCoverageMissing);
            break;
        }
    }
}

fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_boundary_material(text),
        serde_json::Value::Array(values) => {
            values.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(values) => values
            .values()
            .any(json_contains_forbidden_boundary_material),
        _ => false,
    }
}

fn contains_forbidden_boundary_material(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("oauth_token")
        || lower.contains("bearer ")
        || lower.contains("billing-account")
        || lower.contains("raw_prompt")
        || lower.contains("/users/")
}
