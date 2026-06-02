//! Hardened repo-defined AI instructions, policy interaction, and a
//! provider-neutral kill-switch / backout posture.
//!
//! This module stabilizes the repo-instruction governance lane into one
//! export-safe packet that binds, for a single material AI or high-risk command
//! run governed by repo-authored instructions:
//!
//! - the **repo-defined instruction rows** that pin each repo-authored
//!   instruction source (AGENTS.md-style bundle, designated policy file,
//!   workspace-pinned policy, user-profile policy, or an unknown source that
//!   fails closed) to its canonical instruction-following precedence rank, trust
//!   posture, and the closed rule that a repo instruction bundle MAY narrow but
//!   MAY NOT widen permissions, retention, egress, provider route, or trust;
//! - the **policy interaction rows** that resolve every conflict between a repo
//!   claim and a higher-authority policy strictly by precedence — a designated
//!   policy file always wins, repo narrowing is admitted, and a repo widening
//!   attempt is denied with the typed prohibited-case that fired;
//! - the **provider-neutral kill switch** that, when engaged, fails closed and
//!   blocks every provider, model, and external-tool route at once — never a
//!   single provider — and can only be re-armed through an explicit approval; and
//! - the **backout posture** that preserves a rollback checkpoint, stays fully
//!   reversible with no partial writes, and reconstructs from the same evidence
//!   id the run was minted under.
//!
//! It does not re-derive context-assembly, evidence, scoped-apply, or
//! tainted-context truth. The
//! [`crate::finalize_tainted_context_fences::FinalizedTaintedContextPacket`]
//! proves the finalized fences and content boundary, the
//! [`crate::tainted_context::TaintedContextBetaPacket`] proves the live narrowing
//! run, and the
//! [`crate::harden_ai_scoped_apply::AiScopedApplyHardeningPacket`] proves the
//! apply lifecycle and cross-wedge command parity. This packet re-exports the
//! command-surface and run-mode vocabularies verbatim and adds the finalized
//! invariants the stable line needs: that repo instructions cannot widen
//! authority, that policy precedence is closed and honored, that the kill switch
//! is provider-neutral and fails closed, and that a backout reconstructs the run
//! from the same evidence id for admin and support.
//!
//! The frozen contracts this lane projects against are the prompt-injection and
//! taint contract
//! ([`docs/ai/prompt_injection_and_taint_contract.md`](../../../docs/ai/prompt_injection_and_taint_contract.md))
//! and the model-graduation and budget contract
//! ([`docs/ai/model_graduation_and_budget_contract.md`](../../../docs/ai/model_graduation_and_budget_contract.md)),
//! which owns the kill-switch revocation lever.
//!
//! The record is export-safe. It carries refs, state tokens, coarse classes,
//! counts, and review labels only. Raw instruction bodies, raw prompts, endpoint
//! URLs, credentials, signing-key material, and billing-account ids stay outside
//! the support boundary.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::harden_ai_scoped_apply::{CommandSurfaceClass, SurfaceQualificationClass};
use crate::tainted_context::{TaintedContextOriginLocusClass, TaintedContextRunModeClass};

/// Stable record-kind tag carried by [`RepoAiInstructionHardeningPacket`].
pub const HARDEN_REPO_AI_INSTRUCTIONS_RECORD_KIND: &str = "harden_repo_ai_instructions";

/// Schema version for hardened repo-instruction records.
pub const HARDEN_REPO_AI_INSTRUCTIONS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the hardened repo-instruction boundary schema.
pub const HARDEN_REPO_AI_INSTRUCTIONS_SCHEMA_REF: &str =
    "schemas/ai/harden_repo_ai_instructions.schema.json";

/// Repo-relative path of the hardened repo-instruction contract doc.
pub const HARDEN_REPO_AI_INSTRUCTIONS_AI_DOC_REF: &str =
    "docs/ai/m4/harden_repo_ai_instructions.md";

/// Repo-relative path of the frozen prompt-injection and taint contract.
pub const HARDEN_REPO_AI_INSTRUCTIONS_TAINT_CONTRACT_REF: &str =
    "docs/ai/prompt_injection_and_taint_contract.md";

/// Repo-relative path of the frozen model-graduation and budget contract that
/// owns the kill-switch revocation lever.
pub const HARDEN_REPO_AI_INSTRUCTIONS_KILL_SWITCH_CONTRACT_REF: &str =
    "docs/ai/model_graduation_and_budget_contract.md";

/// Repo-relative path of the protected hardened repo-instruction fixture dir.
pub const HARDEN_REPO_AI_INSTRUCTIONS_FIXTURE_DIR: &str =
    "fixtures/ai/m4/harden_repo_ai_instructions";

/// Repo-relative path of the checked hardened repo-instruction export.
pub const HARDEN_REPO_AI_INSTRUCTIONS_ARTIFACT_REF: &str =
    "artifacts/ai/m4/harden_repo_ai_instructions/support_export.json";

/// Repo-relative path of the checked hardened repo-instruction Markdown summary.
pub const HARDEN_REPO_AI_INSTRUCTIONS_SUMMARY_REF: &str =
    "artifacts/ai/m4/harden_repo_ai_instructions/summary.md";

/// Repo-authored instruction source whose authority this packet pins.
///
/// The closed instruction-following precedence (highest to lowest) is the spine
/// of repo-instruction safety: a designated policy file carries signed admin
/// authority above a repo instruction bundle, and an unknown source fails closed
/// to a data-only lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepoInstructionSourceClass {
    /// Signed admin policy file under `.aureline/ai/policy/*`.
    DesignatedPolicyFile,
    /// AGENTS.md-style repo-authored instruction bundle (first-party authored).
    RepoInstructionBundle,
    /// Workspace-owner-pinned policy below the repo instruction bundle.
    TrustedWorkspacePinnedPolicy,
    /// User-profile policy below the workspace-pinned policy.
    TrustedUserProfilePolicy,
    /// Unknown or unclassified instruction source that must fail closed.
    UnknownRepoInstructionFailClosed,
}

impl RepoInstructionSourceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesignatedPolicyFile => "designated_policy_file",
            Self::RepoInstructionBundle => "repo_instruction_bundle",
            Self::TrustedWorkspacePinnedPolicy => "trusted_workspace_pinned_policy",
            Self::TrustedUserProfilePolicy => "trusted_user_profile_policy",
            Self::UnknownRepoInstructionFailClosed => "unknown_repo_instruction_fail_closed",
        }
    }

    /// Canonical instruction-following precedence rank (lower binds higher),
    /// mirroring the frozen taint-contract ordering.
    pub const fn canonical_precedence_rank(self) -> u32 {
        match self {
            Self::DesignatedPolicyFile => 3,
            Self::RepoInstructionBundle => 5,
            Self::TrustedWorkspacePinnedPolicy => 6,
            Self::TrustedUserProfilePolicy => 7,
            Self::UnknownRepoInstructionFailClosed => 99,
        }
    }

    /// True when this source may legitimately claim authority that widens
    /// permissions, retention, egress, provider route, or trust.
    ///
    /// Only a signed designated policy file may widen; every repo-authored
    /// instruction bundle and pinned policy may narrow only.
    pub const fn may_claim_widening_authority(self) -> bool {
        matches!(self, Self::DesignatedPolicyFile)
    }

    /// True when this source requires signing evidence to carry its authority.
    pub const fn requires_signing_evidence(self) -> bool {
        matches!(self, Self::DesignatedPolicyFile)
    }

    /// True when this source must fail closed to a data-only lane.
    pub const fn must_fail_closed(self) -> bool {
        matches!(self, Self::UnknownRepoInstructionFailClosed)
    }

    /// Repo-instruction sources the packet must cover to claim the stable line.
    pub const fn required_coverage() -> [Self; 2] {
        [Self::DesignatedPolicyFile, Self::RepoInstructionBundle]
    }
}

/// Trust posture assigned to a repo-defined instruction source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstructionTrustPostureClass {
    /// Signed admin authority resolved to an admitted signer.
    SignedAdminAuthority,
    /// First-party repo-authored content the user owns.
    TrustedFirstPartyAuthored,
    /// Workspace-owner-pinned content.
    WorkspacePinned,
    /// User-profile content.
    UserProfile,
    /// Untrusted content that must be fenced as data only.
    UntrustedMustFence,
}

impl InstructionTrustPostureClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedAdminAuthority => "signed_admin_authority",
            Self::TrustedFirstPartyAuthored => "trusted_first_party_authored",
            Self::WorkspacePinned => "workspace_pinned",
            Self::UserProfile => "user_profile",
            Self::UntrustedMustFence => "untrusted_must_fence",
        }
    }
}

/// Precedence outcome when a repo claim meets a higher-authority policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyInteractionOutcomeClass {
    /// A higher-authority policy overrode the repo claim by closed precedence.
    PolicyOverridesRepo,
    /// The repo claim narrowed authority and was admitted.
    RepoNarrowingAdmitted,
    /// The repo claim attempted to widen authority and was denied.
    RepoWideningDenied,
    /// A designated policy file was required but missing; the run failed closed.
    DesignatedPolicyMissingFailClosed,
}

impl PolicyInteractionOutcomeClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyOverridesRepo => "policy_overrides_repo",
            Self::RepoNarrowingAdmitted => "repo_narrowing_admitted",
            Self::RepoWideningDenied => "repo_widening_denied",
            Self::DesignatedPolicyMissingFailClosed => "designated_policy_missing_fail_closed",
        }
    }

    /// True when this outcome must name the typed prohibited case that fired.
    pub const fn requires_prohibited_case(self) -> bool {
        matches!(self, Self::RepoWideningDenied)
    }

    /// Outcomes the packet must cover to prove the policy interaction is real.
    pub const fn required_coverage() -> [Self; 3] {
        [
            Self::PolicyOverridesRepo,
            Self::RepoNarrowingAdmitted,
            Self::RepoWideningDenied,
        ]
    }
}

/// Typed prohibited case fired when a repo claim is denied, mirroring the closed
/// `prohibited_case_class` vocabulary in the taint contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepoProhibitedCaseClass {
    /// Repo instruction-bundle text proposes to widen permissions/retention/egress.
    RepoTextWideningAttempted,
    /// Repo text proposes to change the provider route.
    RepoTextProviderRouteChange,
    /// Repo text proposes to widen workspace-trust state.
    RepoTextTrustStateWidening,
    /// A model response or tainted segment proposes to modify a policy file.
    PolicyFileSelfModificationAttempted,
    /// Repo or model prose claims to authorize a privileged follow-on.
    ModelSelfAuthorizationAttempted,
    /// A surface attempts a privileged action without spending an approval ticket.
    ApprovalPathBypassAttempted,
}

impl RepoProhibitedCaseClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepoTextWideningAttempted => "repo_text_widening_attempted",
            Self::RepoTextProviderRouteChange => "repo_text_attempts_provider_route_change",
            Self::RepoTextTrustStateWidening => "repo_text_attempts_trust_state_widening",
            Self::PolicyFileSelfModificationAttempted => "policy_file_self_modification_attempted",
            Self::ModelSelfAuthorizationAttempted => "model_self_authorization_attempted",
            Self::ApprovalPathBypassAttempted => "approval_path_bypass_attempted",
        }
    }
}

/// Provider-neutral scope a kill switch covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KillSwitchScopeClass {
    /// Every provider, model, and external-tool route at once (provider-neutral).
    AllProvidersAndTools,
    /// Hosted provider routes only (not provider-neutral).
    HostedProvidersOnly,
    /// A single provider only (not provider-neutral).
    SingleProvider,
    /// A single model only (not provider-neutral).
    SingleModel,
}

impl KillSwitchScopeClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllProvidersAndTools => "all_providers_and_tools",
            Self::HostedProvidersOnly => "hosted_providers_only",
            Self::SingleProvider => "single_provider",
            Self::SingleModel => "single_model",
        }
    }

    /// True when this scope blocks every route regardless of provider identity.
    pub const fn is_provider_neutral(self) -> bool {
        matches!(self, Self::AllProvidersAndTools)
    }
}

/// State of the provider-neutral kill switch at mint time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KillSwitchStateClass {
    /// Disengaged: normal routing is allowed under policy.
    DisengagedNormalRouting,
    /// Engaged and failing closed: every AI route is blocked.
    EngagedFailClosed,
    /// Engaged on a manual hold pending an explicit re-arm approval.
    EngagedManualHold,
}

impl KillSwitchStateClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisengagedNormalRouting => "disengaged_normal_routing",
            Self::EngagedFailClosed => "engaged_fail_closed",
            Self::EngagedManualHold => "engaged_manual_hold",
        }
    }

    /// True when this state blocks all AI routes.
    pub const fn is_engaged(self) -> bool {
        matches!(self, Self::EngagedFailClosed | Self::EngagedManualHold)
    }
}

/// Completeness of the backout posture for this run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackoutCompletenessClass {
    /// A full backout to the checkpoint with no partial writes left behind.
    FullBackoutNoPartialWrites,
    /// A partial backout that needs review before the run can be retried.
    PartialBackoutNeedsReview,
    /// The backout failed and was escalated; the run cannot claim Stable.
    BackoutFailedEscalated,
}

impl BackoutCompletenessClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullBackoutNoPartialWrites => "full_backout_no_partial_writes",
            Self::PartialBackoutNeedsReview => "partial_backout_needs_review",
            Self::BackoutFailedEscalated => "backout_failed_escalated",
        }
    }

    /// True when this completeness qualifies a run for the Stable lane.
    pub const fn is_stable_qualified(self) -> bool {
        matches!(self, Self::FullBackoutNoPartialWrites)
    }
}

/// One repo-defined instruction source row pinned to its precedence and trust.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoInstructionRow {
    /// Stable instruction-source ref.
    pub instruction_ref: String,
    /// Review-safe label of the source path (never the raw body).
    pub source_path_label: String,
    /// Repo-instruction source class.
    pub source_class: RepoInstructionSourceClass,
    /// Trust posture assigned to the source.
    pub trust_posture_class: InstructionTrustPostureClass,
    /// Origin locus disclosed for the source.
    pub origin_locus_class: TaintedContextOriginLocusClass,
    /// Declared instruction-following precedence rank (lower binds higher).
    pub precedence_rank: u32,
    /// True when this source claims authority that widens scope.
    ///
    /// Must be `false` for every source except a signed designated policy file.
    pub claims_widening_authority: bool,
    /// True when this source may narrow scope (always allowed).
    pub may_narrow_authority: bool,
    /// True when this source was vetted against the active policy epoch.
    pub policy_vetted: bool,
    /// True when this source's instruction authority was fenced to data only.
    pub fenced_to_data_only: bool,
    /// Signing evidence ref (required for a designated policy file).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signing_evidence_ref: Option<String>,
    /// True when raw bodies are forbidden for this source.
    pub raw_body_forbidden: bool,
    /// Review-safe explanation shown to users and support.
    pub user_visible_label: String,
}

/// One policy-interaction row resolving a repo claim against higher authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyInteractionRow {
    /// Stable interaction ref.
    pub interaction_ref: String,
    /// Repo instruction-source ref this interaction resolves.
    pub instruction_ref: String,
    /// Review-safe label of the repo claim being resolved.
    pub repo_claim_label: String,
    /// Source class of the higher-authority policy that governed the outcome.
    pub governing_authority_class: RepoInstructionSourceClass,
    /// Precedence outcome of the interaction.
    pub outcome_class: PolicyInteractionOutcomeClass,
    /// Typed prohibited case that fired (required for a widening denial).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prohibited_case_class: Option<RepoProhibitedCaseClass>,
    /// Effective run mode after the interaction resolved.
    pub effective_mode_class: TaintedContextRunModeClass,
    /// True when the outcome is reconstructible from audit/support state.
    pub auditable: bool,
    /// Review-safe explanation shown to users and support.
    pub user_visible_label: String,
}

/// Provider-neutral kill-switch posture for this run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KillSwitchPosture {
    /// Stable kill-switch ref (the revocation lever).
    pub kill_switch_ref: String,
    /// Kill-switch state at mint time.
    pub state_class: KillSwitchStateClass,
    /// Scope the kill switch covers.
    pub scope_class: KillSwitchScopeClass,
    /// True when the kill switch is provider-neutral.
    ///
    /// Must agree with [`KillSwitchScopeClass::is_provider_neutral`].
    pub provider_neutral: bool,
    /// True when engaging the kill switch fails closed (denies all routes).
    pub fails_closed: bool,
    /// True when engaging blocks every hosted provider/model route.
    pub disables_hosted_routing: bool,
    /// True when engaging blocks every local/on-device model route.
    pub disables_local_routing: bool,
    /// True when engaging blocks every external-tool route.
    pub disables_external_tools: bool,
    /// True when re-arming the kill switch requires an explicit approval.
    pub re_arm_requires_approval: bool,
    /// Effective run mode while the kill switch is engaged (must be blocked).
    pub effective_mode_when_engaged: TaintedContextRunModeClass,
    /// Review-safe explanation shown to users and support.
    pub user_visible_label: String,
}

/// Backout posture binding the run's rollback checkpoint and reversibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackoutPosture {
    /// Stable backout ref.
    pub backout_ref: String,
    /// Completeness of the backout for this run.
    pub completeness_class: BackoutCompletenessClass,
    /// Rollback checkpoint ref preserved before any write could land.
    pub rollback_checkpoint_ref: String,
    /// True when the run is fully reversible from the checkpoint.
    pub reversible: bool,
    /// True when the backout is linked to the run's evidence id.
    pub evidence_linked: bool,
    /// True when this backout can be triggered by the kill switch engaging.
    pub triggered_by_kill_switch: bool,
    /// Review-safe explanation shown to users and support.
    pub user_visible_label: String,
}

/// One command-surface parity row proving the surface shares the canonical
/// command descriptor, preview, approval, result, and rollback model and honors
/// the same repo-instruction governance, kill switch, and backout posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandSurfaceParityRow {
    /// Launch wedge this row covers.
    pub surface_class: CommandSurfaceClass,
    /// Command descriptor ref projected on this surface.
    pub descriptor_ref: String,
    /// True when this surface shares the canonical command descriptor.
    pub shares_command_descriptor: bool,
    /// True when this surface shares the same preview model.
    pub shares_preview_model: bool,
    /// True when this surface shares the same approval model.
    pub shares_approval_model: bool,
    /// True when this surface shares the same result model.
    pub shares_result_model: bool,
    /// True when this surface shares the same rollback model.
    pub shares_rollback_model: bool,
    /// True when this surface honors the same repo-instruction precedence.
    pub honors_instruction_precedence: bool,
    /// True when this surface obeys the provider-neutral kill switch.
    pub obeys_kill_switch: bool,
    /// True when this surface backs out through the same posture.
    pub honors_backout_posture: bool,
    /// True when this surface discloses provider/route truth.
    pub route_disclosed: bool,
    /// True when this surface runs the same policy checks.
    pub policy_checked: bool,
    /// True when this surface is reachable for this command.
    pub reachable: bool,
    /// Stable-qualification posture for this surface.
    pub qualification: SurfaceQualificationClass,
    /// True when this surface claims the Stable lane.
    pub claimed_stable: bool,
}

impl CommandSurfaceParityRow {
    fn preserves_full_parity(&self) -> bool {
        self.shares_command_descriptor
            && self.shares_preview_model
            && self.shares_approval_model
            && self.shares_result_model
            && self.shares_rollback_model
            && self.honors_instruction_precedence
            && self.obeys_kill_switch
            && self.honors_backout_posture
            && self.route_disclosed
            && self.policy_checked
            && self.reachable
    }
}

/// Exportable evidence lineage binding the in-product evidence id and rollback
/// lineage admin/support reconstruct the same run from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoInstructionEvidenceExport {
    /// Evidence id shown in-product and reused by admin/support exports.
    pub evidence_id: String,
    /// JSON export ref.
    pub json_export_ref: String,
    /// Markdown summary ref.
    pub markdown_summary_ref: String,
    /// Admin inspector ref that resolves the same evidence id.
    pub admin_inspector_ref: String,
    /// Support export ref that resolves the same evidence id.
    pub support_export_ref: String,
    /// Rollback checkpoint lineage refs preserved for this run.
    pub rollback_lineage_refs: Vec<String>,
    /// Export lineage refs (prior exports this one descends from).
    pub export_lineage_refs: Vec<String>,
}

/// Constructor input for [`RepoAiInstructionHardeningPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoAiInstructionHardeningPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Context snapshot ref.
    pub context_snapshot_ref: String,
    /// Evidence packet ref, when one has been minted.
    pub evidence_packet_ref: String,
    /// Whether this row claims the Stable line.
    pub claimed_stable: bool,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this run was evaluated under.
    pub policy_epoch_ref: String,
    /// Repo-defined instruction rows.
    pub instruction_rows: Vec<RepoInstructionRow>,
    /// Policy-interaction rows.
    pub policy_interaction_rows: Vec<PolicyInteractionRow>,
    /// Provider-neutral kill-switch posture.
    pub kill_switch: KillSwitchPosture,
    /// Backout posture.
    pub backout: BackoutPosture,
    /// Command-surface parity rows.
    pub surface_parity_rows: Vec<CommandSurfaceParityRow>,
    /// Exportable evidence lineage.
    pub evidence_export: RepoInstructionEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe hardened repo-instruction record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoAiInstructionHardeningPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Context snapshot ref.
    pub context_snapshot_ref: String,
    /// Evidence packet ref, when one has been minted.
    pub evidence_packet_ref: String,
    /// Whether this row claims the Stable line.
    pub claimed_stable: bool,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this run was evaluated under.
    pub policy_epoch_ref: String,
    /// Repo-defined instruction rows.
    pub instruction_rows: Vec<RepoInstructionRow>,
    /// Policy-interaction rows.
    pub policy_interaction_rows: Vec<PolicyInteractionRow>,
    /// Provider-neutral kill-switch posture.
    pub kill_switch: KillSwitchPosture,
    /// Backout posture.
    pub backout: BackoutPosture,
    /// Command-surface parity rows.
    pub surface_parity_rows: Vec<CommandSurfaceParityRow>,
    /// Exportable evidence lineage.
    pub evidence_export: RepoInstructionEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl RepoAiInstructionHardeningPacket {
    /// Builds a hardened repo-instruction packet from canonical rows.
    pub fn new(input: RepoAiInstructionHardeningPacketInput) -> Self {
        Self {
            record_kind: HARDEN_REPO_AI_INSTRUCTIONS_RECORD_KIND.to_owned(),
            schema_version: HARDEN_REPO_AI_INSTRUCTIONS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            display_label: input.display_label,
            context_snapshot_ref: input.context_snapshot_ref,
            evidence_packet_ref: input.evidence_packet_ref,
            claimed_stable: input.claimed_stable,
            trust_state_token: input.trust_state_token,
            policy_epoch_ref: input.policy_epoch_ref,
            instruction_rows: input.instruction_rows,
            policy_interaction_rows: input.policy_interaction_rows,
            kill_switch: input.kill_switch,
            backout: input.backout,
            surface_parity_rows: input.surface_parity_rows,
            evidence_export: input.evidence_export,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the hardened repo-instruction packet's stable-line invariants.
    pub fn validate(&self) -> Vec<RepoAiInstructionHardeningViolation> {
        let mut violations = Vec::new();
        if self.record_kind != HARDEN_REPO_AI_INSTRUCTIONS_RECORD_KIND {
            violations.push(RepoAiInstructionHardeningViolation::WrongRecordKind);
        }
        if self.schema_version != HARDEN_REPO_AI_INSTRUCTIONS_SCHEMA_VERSION {
            violations.push(RepoAiInstructionHardeningViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.context_snapshot_ref.trim().is_empty()
            || self.trust_state_token.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(RepoAiInstructionHardeningViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_instructions(self, &mut violations);
        validate_policy_interactions(self, &mut violations);
        validate_kill_switch(self, &mut violations);
        validate_backout(self, &mut violations);
        validate_surface_parity(self, &mut violations);
        validate_evidence_export(self, &mut violations);
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("hardened repo instruction packet serializes"),
        ) {
            violations.push(RepoAiInstructionHardeningViolation::RawMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("hardened repo instruction packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let widening_denials = self
            .policy_interaction_rows
            .iter()
            .filter(|row| row.outcome_class == PolicyInteractionOutcomeClass::RepoWideningDenied)
            .count();
        let narrowed_surfaces = self
            .surface_parity_rows
            .iter()
            .filter(|row| !surface_is_stable(row.qualification))
            .count();
        let mut out = String::new();
        out.push_str("# AI Repo-Instruction Hardening\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Evidence id: `{}`\n",
            self.evidence_export.evidence_id
        ));
        out.push_str(&format!("- Claimed stable: {}\n", self.claimed_stable));
        out.push_str(&format!(
            "- Repo instruction sources: {}\n",
            self.instruction_rows.len()
        ));
        out.push_str(&format!(
            "- Policy interactions: {} ({} widening denials)\n",
            self.policy_interaction_rows.len(),
            widening_denials
        ));
        out.push_str(&format!(
            "- Kill switch: {} / {} (provider-neutral: {})\n",
            self.kill_switch.state_class.as_str(),
            self.kill_switch.scope_class.as_str(),
            self.kill_switch.provider_neutral
        ));
        out.push_str(&format!(
            "- Backout: {} (reversible: {})\n",
            self.backout.completeness_class.as_str(),
            self.backout.reversible
        ));
        out.push_str(&format!(
            "- Command surfaces: {} ({} narrowed below Stable)\n",
            self.surface_parity_rows.len(),
            narrowed_surfaces
        ));
        out.push_str(&format!(
            "- Rollback lineage refs: {}\n",
            self.evidence_export.rollback_lineage_refs.len()
        ));
        out
    }
}

/// Errors emitted when reading the checked-in hardened repo-instruction export.
#[derive(Debug)]
pub enum RepoAiInstructionHardeningArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RepoAiInstructionHardeningViolation>),
}

impl fmt::Display for RepoAiInstructionHardeningArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "hardened repo instruction export parse failed: {error}"
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
                    "hardened repo instruction export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for RepoAiInstructionHardeningArtifactError {}

/// Validation failures emitted by [`RepoAiInstructionHardeningPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RepoAiInstructionHardeningViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No repo-instruction rows were provided.
    MissingInstructionRows,
    /// Required repo-instruction source coverage is missing.
    MissingInstructionCoverage,
    /// An instruction row claims widening authority it may not hold.
    InstructionClaimsWideningAuthority,
    /// An instruction row's declared precedence rank disagrees with its source.
    InstructionPrecedenceInconsistent,
    /// A designated policy file is missing its signing evidence ref.
    DesignatedPolicySigningEvidenceMissing,
    /// An unknown-source row did not fail closed to data only.
    UnknownInstructionNotFailClosed,
    /// An instruction row was neither policy-vetted nor fenced.
    InstructionNeitherVettedNorFenced,
    /// Required policy-interaction outcome coverage is missing.
    MissingPolicyInteractionCoverage,
    /// A policy-interaction row is missing identity refs or labels.
    PolicyInteractionRefsMissing,
    /// A widening-denial outcome is missing its typed prohibited case.
    WideningDenialMissingProhibitedCase,
    /// A widening denial did not block the run's effective mode.
    WideningDenialNotBlocked,
    /// The kill switch is not provider-neutral.
    KillSwitchNotProviderNeutral,
    /// The provider-neutral flag disagrees with the kill-switch scope.
    KillSwitchScopeMismatch,
    /// The kill switch does not fail closed across every route family.
    KillSwitchDoesNotFailClosed,
    /// Re-arming the kill switch does not require an approval.
    KillSwitchReArmNotGated,
    /// An engaged kill switch did not block the effective run mode.
    KillSwitchEngagedNotBlocking,
    /// The backout posture is missing its checkpoint or is not reversible.
    BackoutPostureIncomplete,
    /// A claimed-stable run does not have a full backout posture.
    BackoutNotStableQualified,
    /// Required command-surface coverage is missing.
    CommandSurfaceCoverageMissing,
    /// A claimed-stable command surface does not preserve full parity.
    CommandParityBroken,
    /// A surface that cannot qualify still claims Stable.
    UnqualifiedSurfaceClaimsStable,
    /// Evidence export refs or the shared evidence id are missing.
    EvidenceExportRefsMissing,
    /// Rollback lineage refs are missing from the evidence export.
    RollbackLineageMissing,
    /// Export contains raw instruction, route, or credential material.
    RawMaterialInExport,
}

impl RepoAiInstructionHardeningViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::MissingInstructionRows => "missing_instruction_rows",
            Self::MissingInstructionCoverage => "missing_instruction_coverage",
            Self::InstructionClaimsWideningAuthority => "instruction_claims_widening_authority",
            Self::InstructionPrecedenceInconsistent => "instruction_precedence_inconsistent",
            Self::DesignatedPolicySigningEvidenceMissing => {
                "designated_policy_signing_evidence_missing"
            }
            Self::UnknownInstructionNotFailClosed => "unknown_instruction_not_fail_closed",
            Self::InstructionNeitherVettedNorFenced => "instruction_neither_vetted_nor_fenced",
            Self::MissingPolicyInteractionCoverage => "missing_policy_interaction_coverage",
            Self::PolicyInteractionRefsMissing => "policy_interaction_refs_missing",
            Self::WideningDenialMissingProhibitedCase => "widening_denial_missing_prohibited_case",
            Self::WideningDenialNotBlocked => "widening_denial_not_blocked",
            Self::KillSwitchNotProviderNeutral => "kill_switch_not_provider_neutral",
            Self::KillSwitchScopeMismatch => "kill_switch_scope_mismatch",
            Self::KillSwitchDoesNotFailClosed => "kill_switch_does_not_fail_closed",
            Self::KillSwitchReArmNotGated => "kill_switch_re_arm_not_gated",
            Self::KillSwitchEngagedNotBlocking => "kill_switch_engaged_not_blocking",
            Self::BackoutPostureIncomplete => "backout_posture_incomplete",
            Self::BackoutNotStableQualified => "backout_not_stable_qualified",
            Self::CommandSurfaceCoverageMissing => "command_surface_coverage_missing",
            Self::CommandParityBroken => "command_parity_broken",
            Self::UnqualifiedSurfaceClaimsStable => "unqualified_surface_claims_stable",
            Self::EvidenceExportRefsMissing => "evidence_export_refs_missing",
            Self::RollbackLineageMissing => "rollback_lineage_missing",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Returns the checked-in hardened repo-instruction support export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or validate.
pub fn current_stable_harden_repo_ai_instructions_export(
) -> Result<RepoAiInstructionHardeningPacket, RepoAiInstructionHardeningArtifactError> {
    let packet: RepoAiInstructionHardeningPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m4/harden_repo_ai_instructions/support_export.json"
    )))
    .map_err(RepoAiInstructionHardeningArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RepoAiInstructionHardeningArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &RepoAiInstructionHardeningPacket,
    violations: &mut Vec<RepoAiInstructionHardeningViolation>,
) {
    for required in [
        HARDEN_REPO_AI_INSTRUCTIONS_AI_DOC_REF,
        HARDEN_REPO_AI_INSTRUCTIONS_SCHEMA_REF,
        HARDEN_REPO_AI_INSTRUCTIONS_TAINT_CONTRACT_REF,
        HARDEN_REPO_AI_INSTRUCTIONS_KILL_SWITCH_CONTRACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(RepoAiInstructionHardeningViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_instructions(
    packet: &RepoAiInstructionHardeningPacket,
    violations: &mut Vec<RepoAiInstructionHardeningViolation>,
) {
    if packet.instruction_rows.is_empty() {
        violations.push(RepoAiInstructionHardeningViolation::MissingInstructionRows);
        return;
    }

    for required in RepoInstructionSourceClass::required_coverage() {
        if !packet
            .instruction_rows
            .iter()
            .any(|row| row.source_class == required)
        {
            violations.push(RepoAiInstructionHardeningViolation::MissingInstructionCoverage);
            break;
        }
    }

    for row in &packet.instruction_rows {
        if row.instruction_ref.trim().is_empty()
            || row.source_path_label.trim().is_empty()
            || row.user_visible_label.trim().is_empty()
            || !row.raw_body_forbidden
        {
            violations.push(RepoAiInstructionHardeningViolation::MissingInstructionCoverage);
            break;
        }
        // Only a signed designated policy file may claim widening authority.
        if row.claims_widening_authority && !row.source_class.may_claim_widening_authority() {
            violations
                .push(RepoAiInstructionHardeningViolation::InstructionClaimsWideningAuthority);
            break;
        }
        // The declared precedence rank must match the closed canonical ordering.
        if row.precedence_rank != row.source_class.canonical_precedence_rank() {
            violations.push(RepoAiInstructionHardeningViolation::InstructionPrecedenceInconsistent);
            break;
        }
        // A designated policy file's authority is meaningless without a signer.
        if row.source_class.requires_signing_evidence()
            && row
                .signing_evidence_ref
                .as_deref()
                .map_or(true, |reference| reference.trim().is_empty())
        {
            violations
                .push(RepoAiInstructionHardeningViolation::DesignatedPolicySigningEvidenceMissing);
            break;
        }
        // An unknown source must fail closed to a fenced data-only lane.
        if row.source_class.must_fail_closed() && !row.fenced_to_data_only {
            violations.push(RepoAiInstructionHardeningViolation::UnknownInstructionNotFailClosed);
            break;
        }
        // Every source carries authority only if it was vetted or was fenced.
        if !row.policy_vetted && !row.fenced_to_data_only {
            violations.push(RepoAiInstructionHardeningViolation::InstructionNeitherVettedNorFenced);
            break;
        }
    }
}

fn validate_policy_interactions(
    packet: &RepoAiInstructionHardeningPacket,
    violations: &mut Vec<RepoAiInstructionHardeningViolation>,
) {
    for required in PolicyInteractionOutcomeClass::required_coverage() {
        if !packet
            .policy_interaction_rows
            .iter()
            .any(|row| row.outcome_class == required)
        {
            violations.push(RepoAiInstructionHardeningViolation::MissingPolicyInteractionCoverage);
            break;
        }
    }

    for row in &packet.policy_interaction_rows {
        if row.interaction_ref.trim().is_empty()
            || row.instruction_ref.trim().is_empty()
            || row.repo_claim_label.trim().is_empty()
            || row.user_visible_label.trim().is_empty()
        {
            violations.push(RepoAiInstructionHardeningViolation::PolicyInteractionRefsMissing);
            break;
        }
        // A widening denial must name the typed prohibited case that fired.
        if row.outcome_class.requires_prohibited_case() && row.prohibited_case_class.is_none() {
            violations
                .push(RepoAiInstructionHardeningViolation::WideningDenialMissingProhibitedCase);
            break;
        }
        // A widening denial must leave the run in a non-widened (blocked or
        // narrowed) effective mode, never a full run.
        if row.outcome_class == PolicyInteractionOutcomeClass::RepoWideningDenied
            && row.effective_mode_class == TaintedContextRunModeClass::FullRun
        {
            violations.push(RepoAiInstructionHardeningViolation::WideningDenialNotBlocked);
            break;
        }
    }
}

fn validate_kill_switch(
    packet: &RepoAiInstructionHardeningPacket,
    violations: &mut Vec<RepoAiInstructionHardeningViolation>,
) {
    let kill_switch = &packet.kill_switch;
    // The provider-neutral flag must agree with the declared scope.
    if kill_switch.provider_neutral != kill_switch.scope_class.is_provider_neutral() {
        violations.push(RepoAiInstructionHardeningViolation::KillSwitchScopeMismatch);
    }
    // A stable kill switch must be provider-neutral.
    if !kill_switch.scope_class.is_provider_neutral() {
        violations.push(RepoAiInstructionHardeningViolation::KillSwitchNotProviderNeutral);
    }
    // It must fail closed across every route family.
    if !kill_switch.fails_closed
        || !kill_switch.disables_hosted_routing
        || !kill_switch.disables_local_routing
        || !kill_switch.disables_external_tools
    {
        violations.push(RepoAiInstructionHardeningViolation::KillSwitchDoesNotFailClosed);
    }
    // Re-arming must require an explicit approval.
    if !kill_switch.re_arm_requires_approval {
        violations.push(RepoAiInstructionHardeningViolation::KillSwitchReArmNotGated);
    }
    // An engaged kill switch must leave the effective mode blocked.
    if kill_switch.state_class.is_engaged()
        && kill_switch.effective_mode_when_engaged != TaintedContextRunModeClass::Blocked
    {
        violations.push(RepoAiInstructionHardeningViolation::KillSwitchEngagedNotBlocking);
    }
}

fn validate_backout(
    packet: &RepoAiInstructionHardeningPacket,
    violations: &mut Vec<RepoAiInstructionHardeningViolation>,
) {
    let backout = &packet.backout;
    if backout.backout_ref.trim().is_empty()
        || backout.rollback_checkpoint_ref.trim().is_empty()
        || backout.user_visible_label.trim().is_empty()
        || !backout.reversible
        || !backout.evidence_linked
    {
        violations.push(RepoAiInstructionHardeningViolation::BackoutPostureIncomplete);
    }
    // A claimed-stable run must carry a full backout with no partial writes.
    if packet.claimed_stable && !backout.completeness_class.is_stable_qualified() {
        violations.push(RepoAiInstructionHardeningViolation::BackoutNotStableQualified);
    }
}

fn validate_surface_parity(
    packet: &RepoAiInstructionHardeningPacket,
    violations: &mut Vec<RepoAiInstructionHardeningViolation>,
) {
    for required in CommandSurfaceClass::required_coverage() {
        if !packet
            .surface_parity_rows
            .iter()
            .any(|row| row.surface_class == required)
        {
            violations.push(RepoAiInstructionHardeningViolation::CommandSurfaceCoverageMissing);
            break;
        }
    }

    for row in &packet.surface_parity_rows {
        if row.descriptor_ref.trim().is_empty() {
            violations.push(RepoAiInstructionHardeningViolation::CommandSurfaceCoverageMissing);
            break;
        }
        // A surface that claims Stable must preserve full parity and qualify.
        if row.claimed_stable
            && (!row.preserves_full_parity() || !surface_is_stable(row.qualification))
        {
            violations.push(RepoAiInstructionHardeningViolation::CommandParityBroken);
            break;
        }
        // A surface that cannot qualify must narrow below Stable, not inherit it.
        if !surface_is_stable(row.qualification) && row.claimed_stable {
            violations.push(RepoAiInstructionHardeningViolation::UnqualifiedSurfaceClaimsStable);
            break;
        }
    }
}

fn validate_evidence_export(
    packet: &RepoAiInstructionHardeningPacket,
    violations: &mut Vec<RepoAiInstructionHardeningViolation>,
) {
    let export = &packet.evidence_export;
    if export.evidence_id.trim().is_empty()
        || export.json_export_ref.trim().is_empty()
        || export.markdown_summary_ref.trim().is_empty()
        || export.admin_inspector_ref.trim().is_empty()
        || export.support_export_ref.trim().is_empty()
    {
        violations.push(RepoAiInstructionHardeningViolation::EvidenceExportRefsMissing);
    }
    // The rollback lineage is the join key a backout reconstructs the run from.
    if export.rollback_lineage_refs.is_empty()
        || export
            .rollback_lineage_refs
            .iter()
            .any(|reference| reference.trim().is_empty())
    {
        violations.push(RepoAiInstructionHardeningViolation::RollbackLineageMissing);
    }
}

/// True when a command surface's qualification posture is the Stable lane.
const fn surface_is_stable(qualification: SurfaceQualificationClass) -> bool {
    matches!(qualification, SurfaceQualificationClass::Stable)
}

fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_material(text),
        serde_json::Value::Array(values) => values.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

fn contains_forbidden_material(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("bearer ")
        || lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("oauth_token")
        || lower.contains("private_key")
        || lower.contains("signing_key")
        || lower.contains("raw_prompt")
        || lower.contains("raw_body")
        || lower.contains("billing-account")
}

#[cfg(test)]
mod tests;
