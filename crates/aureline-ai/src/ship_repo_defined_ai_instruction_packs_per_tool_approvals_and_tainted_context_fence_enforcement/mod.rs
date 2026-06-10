//! Repo-defined AI instruction packs, per-tool approvals, and tainted-context
//! fence enforcement.
//!
//! This module ships the canonical M5 packet that locks three interlocking AI
//! governance concerns into one export-safe artifact. Repo-defined instruction
//! packs may **narrow** the AI's policy posture but never **widen** it;
//! per-tool approvals gate every tool side effect behind a disclosed approval
//! posture with human review on first use; and tainted-context fences keep
//! untrusted context from widening policy or auto-approving a tool. The three
//! concerns are enforced together: tainted context can never grant a tool
//! approval, and a repo instruction can never widen policy. The packet binds
//! three blocks:
//!
//! - A [`RepoInstructionPackBlock`] presents the repo-defined instruction packs
//!   in effect — each carries a source class, a trust posture, the scope effect
//!   it has on policy, and its precedence rank. Every pack is repo-sourced, and
//!   any pack that attempts to widen policy must be blocked and never applied.
//!   Precedence is disclosed rather than hidden.
//! - A [`PerToolApprovalBlock`] presents the per-tool approval decisions — each
//!   carries the tool's capability class, side-effect class, approval posture,
//!   and approval actor. No tool side effect runs without an approval, a denied
//!   tool is never approved, and any side-effecting tool that is approved must
//!   carry a human approval actor and require human review on first use.
//! - A [`ContextFenceBlock`] presents the tainted-context fences in force — each
//!   carries the tainted source class, the enforcement applied, and the usage
//!   constraint it imposes. Every tainted source is fenced, no fence is bypassed,
//!   and each fence blocks both policy widening and tool auto-approval.
//!
//! The packet references upstream M4/M5 lanes by id rather than embedding their
//! content: it cites the prior canonical
//! [`crate::harden_repo_ai_instructions`] repo-instruction hardening lane, the
//! [`crate::tool_gateway`] per-tool approval lane, and the
//! [`crate::finalize_tainted_context_fences`] tainted-context fence lane, plus
//! the [`crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents`]
//! workflow matrix. It projects against the frozen context-assembly contract for
//! evidence-citation and omitted-context truth.
//!
//! The record is export-safe. It carries refs, state tokens, coarse classes,
//! counts, and review labels only. Raw instruction-pack bodies, raw prompt
//! bodies, raw tool arguments, raw tool output, raw tainted content, raw symbol
//! names, raw file paths, provider payloads, endpoint URLs, credentials, raw
//! token counts, exact prices, and billing-account ids stay outside the support
//! boundary.
//!
//! The boundary schema is
//! [`schemas/ai/ship-repo-defined-ai-instruction-packs-per-tool-approvals-and-tainted-context-fence-enforcement.schema.json`](../../../../schemas/ai/ship-repo-defined-ai-instruction-packs-per-tool-approvals-and-tainted-context-fence-enforcement.schema.json).
//! The contract doc is
//! [`docs/ai/m5/ship_repo_defined_ai_instruction_packs_per_tool_approvals_and_tainted_context_fence_enforcement.md`](../../../../docs/ai/m5/ship_repo_defined_ai_instruction_packs_per_tool_approvals_and_tainted_context_fence_enforcement.md).

#[cfg(test)]
mod tests;

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`RepoAiInstructionToolApprovalFencePacket`].
pub const REPO_AI_GOVERNANCE_RECORD_KIND: &str =
    "ai_instruction_packs_tool_approvals_tainted_fence_enforcement";

/// Schema version for AI instruction-pack/tool-approval/fence records.
pub const REPO_AI_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const REPO_AI_GOVERNANCE_SCHEMA_REF: &str =
    "schemas/ai/ship-repo-defined-ai-instruction-packs-per-tool-approvals-and-tainted-context-fence-enforcement.schema.json";

/// Repo-relative path of the M5 contract doc.
pub const REPO_AI_GOVERNANCE_DOC_REF: &str =
    "docs/ai/m5/ship_repo_defined_ai_instruction_packs_per_tool_approvals_and_tainted_context_fence_enforcement.md";

/// Repo-relative path of the frozen context-assembly contract.
pub const REPO_AI_GOVERNANCE_CONTEXT_ASSEMBLY_CONTRACT_REF: &str =
    "docs/ai/context_assembly_contract.md";

/// Repo-relative path of the prior canonical repo-instruction hardening lane.
pub const REPO_AI_GOVERNANCE_REPO_INSTRUCTION_CONTRACT_REF: &str =
    "docs/ai/m4/harden_repo_ai_instructions.md";

/// Repo-relative path of the prompt-injection and taint contract.
pub const REPO_AI_GOVERNANCE_TAINT_CONTRACT_REF: &str =
    "docs/ai/prompt_injection_and_taint_contract.md";

/// Repo-relative path of the prior canonical tainted-context fence lane.
pub const REPO_AI_GOVERNANCE_FENCE_CONTRACT_REF: &str =
    "docs/ai/m4/finalize_tainted_context_fences.md";

/// Repo-relative path of the provider/model/external-tool registry contract.
pub const REPO_AI_GOVERNANCE_TOOL_REGISTRY_CONTRACT_REF: &str =
    "docs/ai/provider-model-tool-registry.md";

/// Repo-relative path of the frozen M5 AI workflow matrix contract.
pub const REPO_AI_GOVERNANCE_M5_MATRIX_CONTRACT_REF: &str =
    "docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md";

/// Repo-relative path of the protected fixture directory.
pub const REPO_AI_GOVERNANCE_FIXTURE_DIR: &str =
    "fixtures/ai/m5/ship_repo_defined_ai_instruction_packs_per_tool_approvals_and_tainted_context_fence_enforcement";

/// Repo-relative path of the checked support-export artifact.
pub const REPO_AI_GOVERNANCE_ARTIFACT_REF: &str =
    "artifacts/ai/m5/ship_repo_defined_ai_instruction_packs_per_tool_approvals_and_tainted_context_fence_enforcement/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const REPO_AI_GOVERNANCE_SUMMARY_REF: &str =
    "artifacts/ai/m5/ship_repo_defined_ai_instruction_packs_per_tool_approvals_and_tainted_context_fence_enforcement.md";

/// Source class of a repo-defined AI instruction pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstructionPackSourceClass {
    /// Committed to the repository and shared with every collaborator.
    RepoCommitted,
    /// Present in the working tree but not committed.
    RepoLocalUncommitted,
    /// Shared at the workspace level above a single repo.
    WorkspaceShared,
    /// Pinned by an organization policy above the workspace.
    OrganizationPinned,
}

/// Trust posture assigned to a repo-defined instruction pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstructionPackTrustClass {
    /// Trusted repo-authored content.
    TrustedRepo,
    /// Imported from an untrusted source and treated as tainted.
    UntrustedImported,
    /// Trusted but applied under a restricted posture.
    Restricted,
}

/// Effect a repo-defined instruction pack has on the AI's policy scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstructionPackScopeEffectClass {
    /// Narrows the policy posture (allowed).
    NarrowsPolicy,
    /// Neither narrows nor widens the policy posture.
    Neutral,
    /// Attempts to widen the policy posture (must be blocked).
    AttemptsWiden,
}

impl InstructionPackScopeEffectClass {
    /// Returns whether this scope effect attempts to widen policy.
    #[must_use]
    pub fn attempts_widen(self) -> bool {
        matches!(self, Self::AttemptsWiden)
    }
}

/// Capability class of a tool the per-tool approval lane gates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PerToolCapabilityClass {
    /// Reads workspace or context without mutation.
    ReadOnly,
    /// Writes to the workspace.
    WorkspaceWrite,
    /// Performs outbound network egress.
    NetworkEgress,
    /// Executes a process.
    ProcessExec,
    /// Calls an external service.
    ExternalService,
}

/// Side-effect class of a tool the per-tool approval lane gates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PerToolSideEffectClass {
    /// No side effect beyond reading.
    None,
    /// Mutates local workspace state.
    LocalMutation,
    /// Makes an outbound network call.
    NetworkCall,
    /// Performs an irreversible action.
    Irreversible,
}

impl PerToolSideEffectClass {
    /// Returns whether this tool has a side effect beyond reading.
    #[must_use]
    pub fn has_side_effect(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Approval posture recorded for a per-tool decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PerToolApprovalPostureClass {
    /// The tool is denied and never runs.
    Denied,
    /// The tool requires human review on first use.
    FirstUseReview,
    /// The tool requires approval on every call.
    PerCall,
    /// The tool is approved for the current session after review.
    SessionScoped,
    /// The tool is pre-approved because it is read-only with no side effect.
    PreApprovedReadOnly,
}

impl PerToolApprovalPostureClass {
    /// Returns whether this posture denies the tool.
    #[must_use]
    pub fn is_denied(self) -> bool {
        matches!(self, Self::Denied)
    }
}

/// Actor that granted a per-tool approval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PerToolApprovalActorClass {
    /// A human operator approved the tool.
    HumanOperator,
    /// A repo policy pre-approved a read-only, no-side-effect tool.
    RepoPolicy,
    /// No actor approved the tool (it is denied or unapproved).
    Unattributed,
}

impl PerToolApprovalActorClass {
    /// Returns whether a human operator granted the approval.
    #[must_use]
    pub fn is_human(self) -> bool {
        matches!(self, Self::HumanOperator)
    }
}

/// Source class of a tainted context input the fence lane guards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextFenceSourceClass {
    /// Imported from an external workspace or file.
    ImportedExternal,
    /// Produced by a tool call.
    ToolOutput,
    /// Fetched from the web.
    WebFetch,
    /// An untrusted document in the repo.
    UntrustedRepoDoc,
    /// Content pasted into the composer.
    PastedContent,
}

/// Enforcement applied to a tainted context input by a fence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextFenceEnforcementClass {
    /// The tainted input is blocked outright.
    Blocked,
    /// The tainted input is downgraded to advisory-only.
    DowngradedToAdvisory,
    /// The tainted input is quarantined behind a boundary.
    Quarantined,
    /// The tainted input is allowed only after explicit human review.
    AllowedAfterReview,
}

/// Usage constraint a fence imposes on a tainted context input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextFenceUsageConstraintClass {
    /// The tainted input may never widen policy.
    NoPolicyWidening,
    /// The tainted input may never auto-approve a tool.
    NoToolAutoApproval,
    /// The tainted input may never carry apply authority.
    NoApplyAuthority,
    /// The tainted input may be displayed only.
    DisplayOnly,
}

/// Consumer surface that must project this governance lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceConsumerSurface {
    /// The desktop composer surface.
    DesktopComposer,
    /// The desktop AI settings surface.
    DesktopSettings,
    /// The CLI/headless surface.
    CliHeadless,
    /// The browser companion surface.
    BrowserCompanion,
    /// The support-export surface.
    SupportExport,
    /// The diagnostics surface.
    Diagnostics,
}

impl GovernanceConsumerSurface {
    /// All consumer surfaces that must project this lane.
    pub const ALL: [GovernanceConsumerSurface; 6] = [
        GovernanceConsumerSurface::DesktopComposer,
        GovernanceConsumerSurface::DesktopSettings,
        GovernanceConsumerSurface::CliHeadless,
        GovernanceConsumerSurface::BrowserCompanion,
        GovernanceConsumerSurface::SupportExport,
        GovernanceConsumerSurface::Diagnostics,
    ];
}

/// Qualification class for a consumer-surface projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceSurfaceQualificationClass {
    /// Stable and fully reachable.
    Stable,
    /// Beta-qualified.
    Beta,
    /// Preview-qualified.
    Preview,
    /// Experimental.
    Experimental,
    /// Not available on this surface.
    Unavailable,
}

impl GovernanceSurfaceQualificationClass {
    /// Returns whether this qualification class is stable.
    #[must_use]
    pub fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that can narrow this lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDowngradeTrigger {
    /// The proof packet is stale.
    ProofStale,
    /// Policy blocked the lane.
    PolicyBlocked,
    /// The provider is unavailable.
    ProviderUnavailable,
    /// The workspace trust posture narrowed.
    TrustNarrowing,
    /// A scope expansion was claimed without qualification.
    ScopeExpansionUnqualified,
    /// An upstream dependency narrowed.
    UpstreamDependencyNarrowed,
    /// A repo instruction widened policy.
    RepoInstructionWidenedPolicy,
    /// A tool side effect ran without approval.
    ToolSideEffectUnapproved,
    /// A tainted context input bypassed its fence.
    TaintedContextBypassedFence,
    /// A tainted context input granted a tool approval.
    ToolApprovalGrantedByTaintedContext,
}

/// A repo-defined AI instruction pack in effect.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoInstructionPackRow {
    /// Stable id of the instruction pack.
    pub pack_id: String,
    /// Source class of the pack.
    pub source_class: InstructionPackSourceClass,
    /// Trust posture assigned to the pack.
    pub trust_posture: InstructionPackTrustClass,
    /// Effect the pack has on policy scope.
    pub scope_effect: InstructionPackScopeEffectClass,
    /// Precedence rank of the pack (lower is higher precedence).
    pub precedence_rank: u32,
    /// Whether the pack was applied.
    pub applied: bool,
    /// Whether any widening attempt by this pack was blocked.
    pub widen_blocked: bool,
    /// Whether the pack is disclosed rather than hidden.
    pub disclosed: bool,
}

/// Block of repo-defined instruction packs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoInstructionPackBlock {
    /// Stable id of the instruction-pack set.
    pub pack_set_id: String,
    /// Whether every pack is repo-sourced rather than a provider default.
    pub repo_only: bool,
    /// Whether repo packs are barred from widening policy.
    pub no_repo_policy_widening: bool,
    /// Whether pack precedence is disclosed.
    pub precedence_disclosed: bool,
    /// The instruction packs in effect.
    pub pack_rows: Vec<RepoInstructionPackRow>,
}

/// A per-tool approval decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerToolApprovalRow {
    /// Stable id of the tool.
    pub tool_id: String,
    /// Capability class of the tool.
    pub capability_class: PerToolCapabilityClass,
    /// Side-effect class of the tool.
    pub side_effect_class: PerToolSideEffectClass,
    /// Approval posture recorded for the tool.
    pub approval_posture: PerToolApprovalPostureClass,
    /// Whether the tool is approved to run.
    pub approved: bool,
    /// Actor that granted the approval.
    pub approval_actor_class: PerToolApprovalActorClass,
    /// Whether the tool requires human review on first use.
    pub requires_human_first_use: bool,
    /// Whether the decision is disclosed rather than hidden.
    pub disclosed: bool,
}

/// Block of per-tool approval decisions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerToolApprovalBlock {
    /// Stable id of the approval set.
    pub approval_set_id: String,
    /// Whether no tool side effect runs without an approval.
    pub no_side_effect_without_approval: bool,
    /// Whether side-effecting tools require human review on first use.
    pub first_use_review_required: bool,
    /// The per-tool approval decisions.
    pub approval_rows: Vec<PerToolApprovalRow>,
}

/// A tainted-context fence in force.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextFenceRow {
    /// Stable id of the fence.
    pub fence_id: String,
    /// Source class of the tainted input.
    pub source_class: ContextFenceSourceClass,
    /// Enforcement applied to the tainted input.
    pub enforcement: ContextFenceEnforcementClass,
    /// Usage constraint the fence imposes.
    pub usage_constraint: ContextFenceUsageConstraintClass,
    /// Whether the fence blocks the tainted input from widening policy.
    pub widening_blocked: bool,
    /// Whether the fence blocks the tainted input from auto-approving a tool.
    pub auto_approval_blocked: bool,
    /// Whether the fence is disclosed rather than hidden.
    pub disclosed: bool,
}

/// Block of tainted-context fences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextFenceBlock {
    /// Stable id of the fence set.
    pub fence_set_id: String,
    /// Whether every tainted source is fenced.
    pub all_tainted_fenced: bool,
    /// Whether no fence is bypassed.
    pub no_fence_bypass: bool,
    /// The tainted-context fences in force.
    pub fence_rows: Vec<ContextFenceRow>,
}

/// Cross-surface consumer-parity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceSurfaceParityRow {
    /// Consumer surface this row describes.
    pub surface: GovernanceConsumerSurface,
    /// Whether the surface projects instruction packs.
    pub shows_instruction_packs: bool,
    /// Whether the surface projects tool approvals.
    pub shows_tool_approvals: bool,
    /// Whether the surface projects tainted-context fences.
    pub shows_fences: bool,
    /// Whether the surface is reachable.
    pub reachable: bool,
    /// Qualification class of the projection.
    pub qualification: GovernanceSurfaceQualificationClass,
    /// Whether the surface claims stable.
    pub claimed_stable: bool,
}

/// Stable-lane input used to mint a [`RepoAiInstructionToolApprovalFencePacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoAiInstructionToolApprovalFencePacketInput {
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical governance-run id shared across surfaces and evidence.
    pub governance_run_id: String,
    /// Display label.
    pub display_label: String,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this packet was evaluated under.
    pub policy_epoch_ref: String,
    /// Whether the three concerns are enforced together as one interlock.
    pub fence_enforcement_interlocked: bool,
    /// Instruction-pack block.
    pub instruction_packs: RepoInstructionPackBlock,
    /// Per-tool approval block.
    pub tool_approvals: PerToolApprovalBlock,
    /// Tainted-context fence block.
    pub context_fences: ContextFenceBlock,
    /// Cross-surface consumer-parity rows.
    pub consumer_surface_parity: Vec<GovernanceSurfaceParityRow>,
    /// Downgrade triggers that apply to this packet.
    pub downgrade_triggers: Vec<GovernanceDowngradeTrigger>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe AI instruction-pack/tool-approval/fence record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoAiInstructionToolApprovalFencePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical governance-run id shared across surfaces and evidence.
    pub governance_run_id: String,
    /// Display label.
    pub display_label: String,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this packet was evaluated under.
    pub policy_epoch_ref: String,
    /// Whether the three concerns are enforced together as one interlock.
    pub fence_enforcement_interlocked: bool,
    /// Instruction-pack block.
    pub instruction_packs: RepoInstructionPackBlock,
    /// Per-tool approval block.
    pub tool_approvals: PerToolApprovalBlock,
    /// Tainted-context fence block.
    pub context_fences: ContextFenceBlock,
    /// Cross-surface consumer-parity rows.
    pub consumer_surface_parity: Vec<GovernanceSurfaceParityRow>,
    /// Downgrade triggers that apply to this packet.
    pub downgrade_triggers: Vec<GovernanceDowngradeTrigger>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl RepoAiInstructionToolApprovalFencePacket {
    /// Builds the packet from the stable-lane input.
    #[must_use]
    pub fn new(input: RepoAiInstructionToolApprovalFencePacketInput) -> Self {
        Self {
            record_kind: REPO_AI_GOVERNANCE_RECORD_KIND.to_owned(),
            schema_version: REPO_AI_GOVERNANCE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            governance_run_id: input.governance_run_id,
            display_label: input.display_label,
            trust_state_token: input.trust_state_token,
            policy_epoch_ref: input.policy_epoch_ref,
            fence_enforcement_interlocked: input.fence_enforcement_interlocked,
            instruction_packs: input.instruction_packs,
            tool_approvals: input.tool_approvals,
            context_fences: input.context_fences,
            consumer_surface_parity: input.consumer_surface_parity,
            downgrade_triggers: input.downgrade_triggers,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the packet's stable-line invariants.
    #[must_use]
    pub fn validate(&self) -> Vec<RepoAiInstructionToolApprovalFenceViolation> {
        let mut violations = Vec::new();
        if self.record_kind != REPO_AI_GOVERNANCE_RECORD_KIND {
            violations.push(RepoAiInstructionToolApprovalFenceViolation::WrongRecordKind);
        }
        if self.schema_version != REPO_AI_GOVERNANCE_SCHEMA_VERSION {
            violations.push(RepoAiInstructionToolApprovalFenceViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.governance_run_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.trust_state_token.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(RepoAiInstructionToolApprovalFenceViolation::MissingIdentity);
        }
        if !self.fence_enforcement_interlocked {
            violations.push(RepoAiInstructionToolApprovalFenceViolation::InterlockNotEnforced);
        }
        validate_source_contracts(self, &mut violations);
        validate_instruction_packs(self, &mut violations);
        validate_tool_approvals(self, &mut violations);
        validate_context_fences(self, &mut violations);
        validate_consumer_surface_parity(self, &mut violations);
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("governance packet serializes"),
        ) {
            violations
                .push(RepoAiInstructionToolApprovalFenceViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    #[must_use]
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("governance packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    #[must_use]
    pub fn render_markdown_summary(&self) -> String {
        let stable_surfaces = self
            .consumer_surface_parity
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let widen_attempts = self
            .instruction_packs
            .pack_rows
            .iter()
            .filter(|row| row.scope_effect.attempts_widen())
            .count();
        let denied_tools = self
            .tool_approvals
            .approval_rows
            .iter()
            .filter(|row| row.approval_posture.is_denied())
            .count();
        let mut out = String::new();
        out.push_str(
            "# Repo-Defined AI Instruction Packs, Per-Tool Approvals, and Tainted-Context Fences\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Governance run: `{}`\n", self.governance_run_id));
        out.push_str(&format!(
            "- Interlock enforced: {}\n",
            self.fence_enforcement_interlocked
        ));
        out.push_str(&format!(
            "- Instruction packs: `{}` ({} packs, {} widen-attempts blocked, repo-only: {}, no-widening: {})\n",
            self.instruction_packs.pack_set_id,
            self.instruction_packs.pack_rows.len(),
            widen_attempts,
            self.instruction_packs.repo_only,
            self.instruction_packs.no_repo_policy_widening
        ));
        out.push_str(&format!(
            "- Tool approvals: `{}` ({} tools, {} denied, no-unapproved-side-effect: {}, first-use review: {})\n",
            self.tool_approvals.approval_set_id,
            self.tool_approvals.approval_rows.len(),
            denied_tools,
            self.tool_approvals.no_side_effect_without_approval,
            self.tool_approvals.first_use_review_required
        ));
        out.push_str(&format!(
            "- Context fences: `{}` ({} fences, all-fenced: {}, no-bypass: {})\n",
            self.context_fences.fence_set_id,
            self.context_fences.fence_rows.len(),
            self.context_fences.all_tainted_fenced,
            self.context_fences.no_fence_bypass
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

/// Validation failures emitted by
/// [`RepoAiInstructionToolApprovalFencePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RepoAiInstructionToolApprovalFenceViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The three concerns are not enforced together as one interlock.
    InterlockNotEnforced,
    /// The instruction-pack set has no packs.
    InstructionPackSetEmpty,
    /// An instruction pack is missing required identity.
    InstructionPackIncomplete,
    /// An instruction pack is disclosed without being marked disclosed.
    HiddenInstructionPack,
    /// An instruction pack is not repo-sourced while the block claims repo-only.
    NonRepoInstructionSource,
    /// Instruction-pack precedence is not disclosed.
    InstructionPrecedenceNotDisclosed,
    /// A repo instruction widened policy.
    RepoInstructionWidenedPolicy,
    /// The per-tool approval set has no tools.
    ToolApprovalSetEmpty,
    /// A per-tool approval is missing required identity.
    ToolApprovalIncomplete,
    /// A per-tool approval is disclosed without being marked disclosed.
    HiddenToolApproval,
    /// A denied tool is marked approved.
    DeniedToolApproved,
    /// A tool side effect is allowed without approval.
    ToolSideEffectUnapproved,
    /// A side-effecting tool is approved without a human approval actor.
    SideEffectToolWithoutHumanApproval,
    /// Side-effecting tools do not require human review on first use.
    FirstUseReviewNotRequired,
    /// The fence set has no fences.
    FenceSetEmpty,
    /// A fence is missing required identity.
    FenceIncomplete,
    /// A fence is disclosed without being marked disclosed.
    HiddenFence,
    /// A tainted source is not fenced.
    TaintedContextUnfenced,
    /// A fence is bypassed.
    FenceBypassed,
    /// A tainted context input is allowed to widen policy.
    TaintedContextWidenedPolicy,
    /// A tainted context input is allowed to auto-approve a tool.
    TaintedContextGrantedToolApproval,
    /// A required consumer surface is missing from the parity rows.
    ConsumerSurfaceCoverageMissing,
    /// A surface claims stable while not reachable.
    StableClaimNotQualified,
    /// The export carries raw boundary material.
    RawBoundaryMaterialInExport,
}

/// Error returned when the checked-in export fails to load or validate.
#[derive(Debug)]
pub enum RepoAiInstructionToolApprovalFenceArtifactError {
    /// The checked-in support export failed to parse.
    SupportExport(serde_json::Error),
    /// The checked-in support export failed validation.
    Validation(Vec<RepoAiInstructionToolApprovalFenceViolation>),
}

impl fmt::Display for RepoAiInstructionToolApprovalFenceArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "governance support export failed to parse: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| format!("{violation:?}"))
                    .collect::<Vec<_>>()
                    .join(",");
                write!(formatter, "governance export failed validation: {tokens}")
            }
        }
    }
}

impl Error for RepoAiInstructionToolApprovalFenceArtifactError {}

/// Returns the checked-in governance export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or
/// validate.
pub fn current_stable_repo_ai_instruction_tool_approval_fence_export(
) -> Result<RepoAiInstructionToolApprovalFencePacket, RepoAiInstructionToolApprovalFenceArtifactError>
{
    let packet: RepoAiInstructionToolApprovalFencePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/ship_repo_defined_ai_instruction_packs_per_tool_approvals_and_tainted_context_fence_enforcement/support_export.json"
    )))
    .map_err(RepoAiInstructionToolApprovalFenceArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RepoAiInstructionToolApprovalFenceArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &RepoAiInstructionToolApprovalFencePacket,
    violations: &mut Vec<RepoAiInstructionToolApprovalFenceViolation>,
) {
    for required in [
        REPO_AI_GOVERNANCE_DOC_REF,
        REPO_AI_GOVERNANCE_SCHEMA_REF,
        REPO_AI_GOVERNANCE_CONTEXT_ASSEMBLY_CONTRACT_REF,
        REPO_AI_GOVERNANCE_REPO_INSTRUCTION_CONTRACT_REF,
        REPO_AI_GOVERNANCE_TAINT_CONTRACT_REF,
        REPO_AI_GOVERNANCE_FENCE_CONTRACT_REF,
        REPO_AI_GOVERNANCE_TOOL_REGISTRY_CONTRACT_REF,
        REPO_AI_GOVERNANCE_M5_MATRIX_CONTRACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(RepoAiInstructionToolApprovalFenceViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_instruction_packs(
    packet: &RepoAiInstructionToolApprovalFencePacket,
    violations: &mut Vec<RepoAiInstructionToolApprovalFenceViolation>,
) {
    let packs = &packet.instruction_packs;
    if packs.pack_set_id.trim().is_empty() || packs.pack_rows.is_empty() {
        violations.push(RepoAiInstructionToolApprovalFenceViolation::InstructionPackSetEmpty);
        return;
    }
    if !packs.repo_only {
        violations.push(RepoAiInstructionToolApprovalFenceViolation::NonRepoInstructionSource);
    }
    if !packs.precedence_disclosed {
        violations
            .push(RepoAiInstructionToolApprovalFenceViolation::InstructionPrecedenceNotDisclosed);
    }
    if !packs.no_repo_policy_widening {
        violations.push(RepoAiInstructionToolApprovalFenceViolation::RepoInstructionWidenedPolicy);
    }
    for pack in &packs.pack_rows {
        if pack.pack_id.trim().is_empty() {
            violations.push(RepoAiInstructionToolApprovalFenceViolation::InstructionPackIncomplete);
        }
        if !pack.disclosed {
            violations.push(RepoAiInstructionToolApprovalFenceViolation::HiddenInstructionPack);
        }
        if pack.scope_effect.attempts_widen() && (!pack.widen_blocked || pack.applied) {
            violations
                .push(RepoAiInstructionToolApprovalFenceViolation::RepoInstructionWidenedPolicy);
        }
    }
}

fn validate_tool_approvals(
    packet: &RepoAiInstructionToolApprovalFencePacket,
    violations: &mut Vec<RepoAiInstructionToolApprovalFenceViolation>,
) {
    let approvals = &packet.tool_approvals;
    if approvals.approval_set_id.trim().is_empty() || approvals.approval_rows.is_empty() {
        violations.push(RepoAiInstructionToolApprovalFenceViolation::ToolApprovalSetEmpty);
        return;
    }
    if !approvals.no_side_effect_without_approval {
        violations.push(RepoAiInstructionToolApprovalFenceViolation::ToolSideEffectUnapproved);
    }
    if !approvals.first_use_review_required {
        violations.push(RepoAiInstructionToolApprovalFenceViolation::FirstUseReviewNotRequired);
    }
    for tool in &approvals.approval_rows {
        if tool.tool_id.trim().is_empty() {
            violations.push(RepoAiInstructionToolApprovalFenceViolation::ToolApprovalIncomplete);
        }
        if !tool.disclosed {
            violations.push(RepoAiInstructionToolApprovalFenceViolation::HiddenToolApproval);
        }
        if tool.approval_posture.is_denied() && tool.approved {
            violations.push(RepoAiInstructionToolApprovalFenceViolation::DeniedToolApproved);
        }
        if tool.side_effect_class.has_side_effect() && tool.approved {
            if !tool.approval_actor_class.is_human() {
                violations.push(
                    RepoAiInstructionToolApprovalFenceViolation::SideEffectToolWithoutHumanApproval,
                );
            }
            if !tool.requires_human_first_use {
                violations
                    .push(RepoAiInstructionToolApprovalFenceViolation::ToolSideEffectUnapproved);
            }
        }
    }
}

fn validate_context_fences(
    packet: &RepoAiInstructionToolApprovalFencePacket,
    violations: &mut Vec<RepoAiInstructionToolApprovalFenceViolation>,
) {
    let fences = &packet.context_fences;
    if fences.fence_set_id.trim().is_empty() || fences.fence_rows.is_empty() {
        violations.push(RepoAiInstructionToolApprovalFenceViolation::FenceSetEmpty);
        return;
    }
    if !fences.all_tainted_fenced {
        violations.push(RepoAiInstructionToolApprovalFenceViolation::TaintedContextUnfenced);
    }
    if !fences.no_fence_bypass {
        violations.push(RepoAiInstructionToolApprovalFenceViolation::FenceBypassed);
    }
    for fence in &fences.fence_rows {
        if fence.fence_id.trim().is_empty() {
            violations.push(RepoAiInstructionToolApprovalFenceViolation::FenceIncomplete);
        }
        if !fence.disclosed {
            violations.push(RepoAiInstructionToolApprovalFenceViolation::HiddenFence);
        }
        if !fence.widening_blocked {
            violations
                .push(RepoAiInstructionToolApprovalFenceViolation::TaintedContextWidenedPolicy);
        }
        if !fence.auto_approval_blocked {
            violations.push(
                RepoAiInstructionToolApprovalFenceViolation::TaintedContextGrantedToolApproval,
            );
        }
    }
}

fn validate_consumer_surface_parity(
    packet: &RepoAiInstructionToolApprovalFencePacket,
    violations: &mut Vec<RepoAiInstructionToolApprovalFenceViolation>,
) {
    let mut seen = std::collections::HashSet::new();
    for row in &packet.consumer_surface_parity {
        seen.insert(row.surface);
        if row.claimed_stable && !row.reachable {
            violations.push(RepoAiInstructionToolApprovalFenceViolation::StableClaimNotQualified);
        }
    }
    for required in GovernanceConsumerSurface::ALL {
        if !seen.contains(&required) {
            violations
                .push(RepoAiInstructionToolApprovalFenceViolation::ConsumerSurfaceCoverageMissing);
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
        || lower.contains('@')
        || lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("oauth_token")
        || lower.contains("bearer ")
        || lower.contains("billing-account")
        || lower.contains("raw_prompt")
        || lower.contains("/users/")
}
