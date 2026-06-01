//! Stable, route-explicit AI scoped-apply hardening records.
//!
//! This module hardens the AI scoped-apply lane into one export-safe packet
//! that binds the preview → approval → apply → revert lifecycle, scoped-apply
//! honesty, multi-file patch honesty, cross-wedge command parity, route/spend
//! authority truth, and the exportable evidence/rollback lineage into a single
//! attributable artifact.
//!
//! It does not re-derive mutation, run-history, or patch-review truth. The
//! [`crate::evidence::AiMutationEvidencePacket`] mutation wedge, the
//! [`crate::run_history`] run-history / rerun-review lane, the
//! [`crate::finalize_ai_evidence_packets`] finalization lane, and the frozen
//! multi-file patch-review-summary boundary
//! ([`schemas/ai/patch_review_summary.schema.json`](../../../schemas/ai/patch_review_summary.schema.json))
//! remain canonical for their own slices. This packet references those lineages
//! by id and adds the apply-time invariants a reviewer, operator, support
//! engineer, or automation surface needs to prove that no write-capable path
//! widened authority, skipped preview/approval, hid a touched file, or escaped
//! its declared scope.
//!
//! The frozen contracts this lane projects against are the alpha
//! preview/apply/revert enforcement contract
//! ([`docs/commands/alpha_preview_apply_revert.md`](../../../docs/commands/alpha_preview_apply_revert.md))
//! and the command invocation-result and cross-surface parity contract
//! ([`docs/commands/invocation_result_and_parity_contract.md`](../../../docs/commands/invocation_result_and_parity_contract.md)).
//! The forced wedge parity (palette, menu, keybinding, CLI/headless, deep-link,
//! automation recipe, and AI assistant) shares one command descriptor, preview,
//! approval, result, and rollback model so a second entry point can never skip
//! the reviewed path.
//!
//! The record is export-safe. It carries refs, state tokens, coarse classes,
//! counts, digests, and review labels only. Raw patch bodies, raw diff text,
//! raw prompt bodies, source file bodies, provider payloads, endpoint URLs,
//! credentials, raw token counts, exact prices, and billing-account ids stay
//! outside the support boundary.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`AiScopedApplyHardeningPacket`].
pub const AI_SCOPED_APPLY_HARDENING_RECORD_KIND: &str = "ai_scoped_apply_hardening";

/// Schema version for AI scoped-apply hardening records.
pub const AI_SCOPED_APPLY_HARDENING_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the scoped-apply hardening boundary schema.
pub const AI_SCOPED_APPLY_HARDENING_SCHEMA_REF: &str =
    "schemas/ai/ai_scoped_apply_hardening.schema.json";

/// Repo-relative path of the scoped-apply hardening contract doc.
pub const AI_SCOPED_APPLY_HARDENING_AI_DOC_REF: &str = "docs/ai/m4/harden_ai_scoped_apply.md";

/// Repo-relative path of the frozen preview/apply/revert enforcement contract.
pub const AI_SCOPED_APPLY_HARDENING_PREVIEW_APPLY_REVERT_CONTRACT_REF: &str =
    "docs/commands/alpha_preview_apply_revert.md";

/// Repo-relative path of the frozen invocation-result and parity contract.
pub const AI_SCOPED_APPLY_HARDENING_PARITY_CONTRACT_REF: &str =
    "docs/commands/invocation_result_and_parity_contract.md";

/// Repo-relative path of the protected scoped-apply hardening fixture directory.
pub const AI_SCOPED_APPLY_HARDENING_FIXTURE_DIR: &str = "fixtures/ai/m4/harden_ai_scoped_apply";

/// Repo-relative path of the checked scoped-apply hardening export.
pub const AI_SCOPED_APPLY_HARDENING_ARTIFACT_REF: &str =
    "artifacts/ai/m4/harden_ai_scoped_apply/support_export.json";

/// Repo-relative path of the checked scoped-apply hardening Markdown summary.
pub const AI_SCOPED_APPLY_HARDENING_SUMMARY_REF: &str =
    "artifacts/ai/m4/harden_ai_scoped_apply/summary.md";

/// Lifecycle state of a scoped-apply across preview, approval, apply, revert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplyLifecycleStateClass {
    /// Preview was produced and the apply is waiting on approval.
    PreviewPendingApproval,
    /// Approval was granted but the patch has not reached the live tree.
    ApprovedNotYetApplied,
    /// The patch reached the live tree and is being kept.
    AppliedKept,
    /// The patch reached the live tree and was then reverted.
    AppliedThenReverted,
    /// The operator rejected the patch; nothing reached the live tree.
    RejectedNoApply,
    /// A policy or fence blocked the patch; nothing reached the live tree.
    BlockedNoApply,
}

impl ApplyLifecycleStateClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewPendingApproval => "preview_pending_approval",
            Self::ApprovedNotYetApplied => "approved_not_yet_applied",
            Self::AppliedKept => "applied_kept",
            Self::AppliedThenReverted => "applied_then_reverted",
            Self::RejectedNoApply => "rejected_no_apply",
            Self::BlockedNoApply => "blocked_no_apply",
        }
    }

    /// Lifecycle states that mutated the live tree.
    const fn reached_live_tree(self) -> bool {
        matches!(self, Self::AppliedKept | Self::AppliedThenReverted)
    }

    /// Terminal states where the patch was never applied.
    const fn is_terminal_no_apply(self) -> bool {
        matches!(self, Self::RejectedNoApply | Self::BlockedNoApply)
    }
}

/// Coarse write-scope class an AI scoped-apply can declare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplyWriteScopeClass {
    /// A single workspace file.
    SingleFile,
    /// A bounded set of workspace files.
    MultiFileBounded,
    /// A workspace-wide edit.
    WorkspaceWide,
    /// A change with an external effect beyond the workspace.
    ExternalEffect,
}

impl ApplyWriteScopeClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleFile => "single_file",
            Self::MultiFileBounded => "multi_file_bounded",
            Self::WorkspaceWide => "workspace_wide",
            Self::ExternalEffect => "external_effect",
        }
    }
}

/// Per-file change kind disclosed in a multi-file patch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatchChangeKind {
    /// A new file is created.
    CreateFile,
    /// An existing file is modified.
    ModifyFile,
    /// An existing file is deleted.
    DeleteFile,
    /// An existing file is renamed.
    RenameFile,
}

impl PatchChangeKind {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CreateFile => "create_file",
            Self::ModifyFile => "modify_file",
            Self::DeleteFile => "delete_file",
            Self::RenameFile => "rename_file",
        }
    }

    const fn is_rename(self) -> bool {
        matches!(self, Self::RenameFile)
    }

    const fn requires_hunks(self) -> bool {
        matches!(self, Self::ModifyFile)
    }
}

/// Launch wedge a scoped-apply command can be invoked from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandSurfaceClass {
    /// Command palette.
    PaletteCommand,
    /// Application menu item.
    MenuItem,
    /// Keybinding.
    Keybinding,
    /// CLI / headless invocation.
    CliHeadless,
    /// Deep link.
    DeepLink,
    /// Automation recipe / macro.
    AutomationRecipe,
    /// AI assistant invocation.
    AiAssistant,
}

impl CommandSurfaceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PaletteCommand => "palette_command",
            Self::MenuItem => "menu_item",
            Self::Keybinding => "keybinding",
            Self::CliHeadless => "cli_headless",
            Self::DeepLink => "deep_link",
            Self::AutomationRecipe => "automation_recipe",
            Self::AiAssistant => "ai_assistant",
        }
    }

    /// Launch wedges the packet must cover to claim cross-surface parity.
    pub const fn required_coverage() -> [Self; 7] {
        [
            Self::PaletteCommand,
            Self::MenuItem,
            Self::Keybinding,
            Self::CliHeadless,
            Self::DeepLink,
            Self::AutomationRecipe,
            Self::AiAssistant,
        ]
    }
}

/// Stable-qualification posture of a single command surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceQualificationClass {
    /// Surface qualifies for the Stable lane.
    Stable,
    /// Surface is narrowed to Beta.
    Beta,
    /// Surface is narrowed to Preview.
    Preview,
    /// Surface is experimental.
    Experimental,
    /// Surface is not available on this row.
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

/// Preview → approval → apply → revert lifecycle block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplyLifecycleBlock {
    /// Lifecycle state of this scoped-apply.
    pub lifecycle_state: ApplyLifecycleStateClass,
    /// Preview record ref (the structured diff preview shown before apply).
    pub preview_record_ref: String,
    /// True when a preview was shown before any apply path was reachable.
    pub preview_shown: bool,
    /// True when this apply requires explicit approval.
    pub approval_required: bool,
    /// Approval record ref when approval was required and granted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_record_ref: Option<String>,
    /// True when approval was granted.
    pub approval_granted: bool,
    /// True when a rollback checkpoint was captured before the live mutation.
    pub checkpoint_captured_before_apply: bool,
    /// Rollback checkpoint ref captured before apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint_ref: Option<String>,
    /// Mutation journal ref when the apply mutated the live tree.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation_journal_ref: Option<String>,
    /// Apply-audit ref proving which hunks reached the live tree.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_audit_ref: Option<String>,
    /// True when a revert is available after apply.
    pub revert_available: bool,
    /// Revert handle ref when a revert is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revert_handle_ref: Option<String>,
    /// True when a direct trusted-path apply was attempted (must be denied).
    pub direct_trusted_apply_attempted: bool,
}

/// Scoped-apply scope contract block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeContractBlock {
    /// Review-safe label of the declared apply scope.
    pub declared_scope_label: String,
    /// Coarse write-scope class.
    pub scope_class: ApplyWriteScopeClass,
    /// Requested-scope ref the operator approved.
    pub requested_scope_ref: String,
    /// Declared path-class count (never raw paths).
    pub declared_path_class_count: u32,
    /// True when the produced patch stayed inside the declared scope.
    pub apply_bounded_to_declared_scope: bool,
}

/// One disclosed file in a multi-file patch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatchFileRow {
    /// Opaque file ref (never a raw absolute path).
    pub file_ref: String,
    /// Change kind for this file.
    pub change_kind: PatchChangeKind,
    /// Hunk count touched in this file.
    pub hunk_count: u32,
    /// True when this file is inside the declared apply scope.
    pub within_declared_scope: bool,
    /// True when this file was disclosed in the preview.
    pub disclosed_in_preview: bool,
    /// True when this file's hunks were approved for apply.
    pub approved_for_apply: bool,
    /// True when this file's approved hunks reached the live tree.
    pub reached_live_tree: bool,
    /// Source file ref for a rename (required for a rename change kind).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rename_from_ref: Option<String>,
}

/// Multi-file patch honesty block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatchHonestyBlock {
    /// Content-addressed patch digest ref binding validation to the proposal.
    pub patch_digest_ref: String,
    /// Declared file count (must equal the disclosed file rows).
    pub declared_file_count: u32,
    /// Disclosed file count (must equal the rows flagged disclosed_in_preview).
    pub disclosed_file_count: u32,
    /// Per-file disclosure rows.
    pub files: Vec<PatchFileRow>,
}

/// One cross-surface command-parity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceParityRow {
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

impl SurfaceParityRow {
    fn preserves_full_parity(&self) -> bool {
        self.shares_command_descriptor
            && self.shares_preview_model
            && self.shares_approval_model
            && self.shares_result_model
            && self.shares_rollback_model
            && self.route_disclosed
            && self.policy_checked
            && self.reachable
    }
}

/// Route, spend, and authority truth block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteSpendAuthorityBlock {
    /// Provider label aligned with on-screen review.
    pub provider_label: String,
    /// Model label aligned with on-screen review.
    pub model_label: String,
    /// Route receipt ref.
    pub route_receipt_ref: String,
    /// Spend receipt ref.
    pub spend_receipt_ref: String,
    /// True when egress/spend is disclosed before apply.
    pub egress_disclosed: bool,
    /// True when tainted context participated in this run.
    pub tainted_context_present: bool,
    /// Tainted-context fence ref when tainted context participated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tainted_context_fence_ref: Option<String>,
    /// True when authority widened without disclosure (must be false).
    pub authority_widened_without_disclosure: bool,
}

/// Exportable evidence and rollback lineage block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceExportBlock {
    /// Bound AI mutation/finalization evidence packet ref.
    pub evidence_packet_ref: String,
    /// Bound multi-file patch-review summary ref.
    pub patch_review_summary_ref: String,
    /// Rollback handle ref describing recovery without ambient authority.
    pub rollback_handle_ref: String,
    /// JSON export ref.
    pub json_export_ref: String,
    /// Markdown summary ref.
    pub markdown_summary_ref: String,
    /// Export lineage refs (prior exports this one descends from).
    pub export_lineage_refs: Vec<String>,
}

/// Constructor input for [`AiScopedApplyHardeningPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiScopedApplyHardeningPacketInput {
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical scoped-apply id shared across surfaces and rollback.
    pub apply_id: String,
    /// Display label.
    pub display_label: String,
    /// Canonical command descriptor ref (e.g. the apply-patch command).
    pub command_descriptor_ref: String,
    /// Command revision aligned with on-screen review.
    pub command_revision: String,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this apply was evaluated under.
    pub policy_epoch_ref: String,
    /// Approval-path label aligned with on-screen review.
    pub approval_path_label: String,
    /// Preview/approval/apply/revert lifecycle block.
    pub lifecycle: ApplyLifecycleBlock,
    /// Scoped-apply scope contract block.
    pub scope_contract: ScopeContractBlock,
    /// Multi-file patch honesty block.
    pub patch_honesty: PatchHonestyBlock,
    /// Cross-surface command-parity rows.
    pub surface_parity: Vec<SurfaceParityRow>,
    /// Route/spend/authority truth block.
    pub route_authority: RouteSpendAuthorityBlock,
    /// Exportable evidence and rollback lineage block.
    pub evidence_export: EvidenceExportBlock,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe AI scoped-apply hardening record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiScopedApplyHardeningPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical scoped-apply id shared across surfaces and rollback.
    pub apply_id: String,
    /// Display label.
    pub display_label: String,
    /// Canonical command descriptor ref.
    pub command_descriptor_ref: String,
    /// Command revision aligned with on-screen review.
    pub command_revision: String,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this apply was evaluated under.
    pub policy_epoch_ref: String,
    /// Approval-path label aligned with on-screen review.
    pub approval_path_label: String,
    /// Preview/approval/apply/revert lifecycle block.
    pub lifecycle: ApplyLifecycleBlock,
    /// Scoped-apply scope contract block.
    pub scope_contract: ScopeContractBlock,
    /// Multi-file patch honesty block.
    pub patch_honesty: PatchHonestyBlock,
    /// Cross-surface command-parity rows.
    pub surface_parity: Vec<SurfaceParityRow>,
    /// Route/spend/authority truth block.
    pub route_authority: RouteSpendAuthorityBlock,
    /// Exportable evidence and rollback lineage block.
    pub evidence_export: EvidenceExportBlock,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl AiScopedApplyHardeningPacket {
    /// Builds a scoped-apply hardening packet from the stable-lane input.
    pub fn new(input: AiScopedApplyHardeningPacketInput) -> Self {
        Self {
            record_kind: AI_SCOPED_APPLY_HARDENING_RECORD_KIND.to_owned(),
            schema_version: AI_SCOPED_APPLY_HARDENING_SCHEMA_VERSION,
            packet_id: input.packet_id,
            apply_id: input.apply_id,
            display_label: input.display_label,
            command_descriptor_ref: input.command_descriptor_ref,
            command_revision: input.command_revision,
            trust_state_token: input.trust_state_token,
            policy_epoch_ref: input.policy_epoch_ref,
            approval_path_label: input.approval_path_label,
            lifecycle: input.lifecycle,
            scope_contract: input.scope_contract,
            patch_honesty: input.patch_honesty,
            surface_parity: input.surface_parity,
            route_authority: input.route_authority,
            evidence_export: input.evidence_export,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the scoped-apply hardening packet's stable-line invariants.
    pub fn validate(&self) -> Vec<AiScopedApplyHardeningViolation> {
        let mut violations = Vec::new();
        if self.record_kind != AI_SCOPED_APPLY_HARDENING_RECORD_KIND {
            violations.push(AiScopedApplyHardeningViolation::WrongRecordKind);
        }
        if self.schema_version != AI_SCOPED_APPLY_HARDENING_SCHEMA_VERSION {
            violations.push(AiScopedApplyHardeningViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.apply_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.command_descriptor_ref.trim().is_empty()
            || self.command_revision.trim().is_empty()
            || self.trust_state_token.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.approval_path_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(AiScopedApplyHardeningViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_lifecycle(self, &mut violations);
        validate_scope_and_patch(self, &mut violations);
        validate_surface_parity(self, &mut violations);
        validate_route_authority(self, &mut violations);
        validate_evidence_export(self, &mut violations);
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("ai scoped-apply hardening packet serializes"),
        ) {
            violations.push(AiScopedApplyHardeningViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("ai scoped-apply hardening packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let applied_files = self
            .patch_honesty
            .files
            .iter()
            .filter(|file| file.reached_live_tree)
            .count();
        let stable_surfaces = self
            .surface_parity
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# AI Scoped-Apply Hardening\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Apply id: `{}`\n", self.apply_id));
        out.push_str(&format!(
            "- Command: `{}` (rev `{}`)\n",
            self.command_descriptor_ref, self.command_revision
        ));
        out.push_str(&format!(
            "- Lifecycle: `{}` via `{}`\n",
            self.lifecycle.lifecycle_state.as_str(),
            self.approval_path_label
        ));
        out.push_str(&format!(
            "- Route: `{} / {}` (egress disclosed: {})\n",
            self.route_authority.provider_label,
            self.route_authority.model_label,
            self.route_authority.egress_disclosed
        ));
        out.push_str(&format!(
            "- Scope: `{}` (bounded: {})\n",
            self.scope_contract.scope_class.as_str(),
            self.scope_contract.apply_bounded_to_declared_scope
        ));
        out.push_str(&format!(
            "- Patch files: {} declared / {} disclosed / {} reached tree\n",
            self.patch_honesty.declared_file_count,
            self.patch_honesty.disclosed_file_count,
            applied_files
        ));
        out.push_str(&format!(
            "- Surface parity: {} surfaces ({} stable)\n",
            self.surface_parity.len(),
            stable_surfaces
        ));
        out.push_str(&format!(
            "- Revert available: {}\n",
            self.lifecycle.revert_available
        ));
        out
    }
}

/// Errors emitted when reading the checked-in scoped-apply hardening export.
#[derive(Debug)]
pub enum AiScopedApplyHardeningArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<AiScopedApplyHardeningViolation>),
}

impl fmt::Display for AiScopedApplyHardeningArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "ai scoped-apply hardening export parse failed: {error}"
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
                    "ai scoped-apply hardening export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for AiScopedApplyHardeningArtifactError {}

/// Validation failures emitted by [`AiScopedApplyHardeningPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AiScopedApplyHardeningViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The lifecycle block is incomplete for the declared lifecycle state.
    ApplyLifecycleIncomplete,
    /// A patch reached the live tree without preview or required approval.
    AppliedWithoutPreviewOrApproval,
    /// A direct trusted-path apply bypassed the reviewed path.
    DirectApplyBypassedReview,
    /// A rejected or blocked apply still mutated the live tree.
    RejectedStateApplied,
    /// A revert is not available after an apply mutated the live tree.
    RevertUnavailableAfterApply,
    /// The scope contract is incomplete.
    ScopeContractIncomplete,
    /// The apply widened beyond its declared scope.
    ScopeWidenedBeyondDeclared,
    /// The multi-file patch honesty block is incomplete.
    PatchHonestyIncomplete,
    /// A file reached the live tree without being disclosed in the preview.
    HiddenPatchFile,
    /// A file reached the live tree without being approved for apply.
    UnapprovedFileApplied,
    /// A launch wedge is not covered by the parity rows.
    CommandSurfaceCoverageMissing,
    /// A surface parity row does not preserve the shared command model.
    CommandParityBroken,
    /// A surface claims Stable without qualifying for it.
    StableClaimNotQualified,
    /// Route/spend authority truth is incomplete.
    RouteAuthorityIncomplete,
    /// Tainted context participated without a fence.
    TaintedContextUnfenced,
    /// Authority widened without disclosure.
    AuthorityWidenedWithoutDisclosure,
    /// Evidence/rollback export refs are missing.
    ExportRefsMissing,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl AiScopedApplyHardeningViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ApplyLifecycleIncomplete => "apply_lifecycle_incomplete",
            Self::AppliedWithoutPreviewOrApproval => "applied_without_preview_or_approval",
            Self::DirectApplyBypassedReview => "direct_apply_bypassed_review",
            Self::RejectedStateApplied => "rejected_state_applied",
            Self::RevertUnavailableAfterApply => "revert_unavailable_after_apply",
            Self::ScopeContractIncomplete => "scope_contract_incomplete",
            Self::ScopeWidenedBeyondDeclared => "scope_widened_beyond_declared",
            Self::PatchHonestyIncomplete => "patch_honesty_incomplete",
            Self::HiddenPatchFile => "hidden_patch_file",
            Self::UnapprovedFileApplied => "unapproved_file_applied",
            Self::CommandSurfaceCoverageMissing => "command_surface_coverage_missing",
            Self::CommandParityBroken => "command_parity_broken",
            Self::StableClaimNotQualified => "stable_claim_not_qualified",
            Self::RouteAuthorityIncomplete => "route_authority_incomplete",
            Self::TaintedContextUnfenced => "tainted_context_unfenced",
            Self::AuthorityWidenedWithoutDisclosure => "authority_widened_without_disclosure",
            Self::ExportRefsMissing => "export_refs_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Returns the checked-in AI scoped-apply hardening export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or validate.
pub fn current_stable_ai_scoped_apply_hardening_export(
) -> Result<AiScopedApplyHardeningPacket, AiScopedApplyHardeningArtifactError> {
    let packet: AiScopedApplyHardeningPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m4/harden_ai_scoped_apply/support_export.json"
    )))
    .map_err(AiScopedApplyHardeningArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AiScopedApplyHardeningArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &AiScopedApplyHardeningPacket,
    violations: &mut Vec<AiScopedApplyHardeningViolation>,
) {
    for required in [
        AI_SCOPED_APPLY_HARDENING_AI_DOC_REF,
        AI_SCOPED_APPLY_HARDENING_PREVIEW_APPLY_REVERT_CONTRACT_REF,
        AI_SCOPED_APPLY_HARDENING_PARITY_CONTRACT_REF,
        AI_SCOPED_APPLY_HARDENING_SCHEMA_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(AiScopedApplyHardeningViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_lifecycle(
    packet: &AiScopedApplyHardeningPacket,
    violations: &mut Vec<AiScopedApplyHardeningViolation>,
) {
    let lifecycle = &packet.lifecycle;
    let state = lifecycle.lifecycle_state;

    // The preview record is the gate for any write-capable path. A scoped-apply
    // can never be reachable without a preview that was actually shown.
    if lifecycle.preview_record_ref.trim().is_empty() || !lifecycle.preview_shown {
        violations.push(AiScopedApplyHardeningViolation::AppliedWithoutPreviewOrApproval);
    }

    // A direct trusted-path apply must always be denied; no surface may skip the
    // reviewed path.
    if lifecycle.direct_trusted_apply_attempted {
        violations.push(AiScopedApplyHardeningViolation::DirectApplyBypassedReview);
    }

    let approval_ref_present = lifecycle
        .approval_record_ref
        .as_deref()
        .is_some_and(|reference| !reference.trim().is_empty());
    // When approval is required it must be both granted and referenced before the
    // patch can move past pending.
    if lifecycle.approval_required {
        let approved_states = matches!(
            state,
            ApplyLifecycleStateClass::ApprovedNotYetApplied
                | ApplyLifecycleStateClass::AppliedKept
                | ApplyLifecycleStateClass::AppliedThenReverted
        );
        if approved_states && (!lifecycle.approval_granted || !approval_ref_present) {
            violations.push(AiScopedApplyHardeningViolation::AppliedWithoutPreviewOrApproval);
        }
    }

    if state.reached_live_tree() {
        // Applied state must carry the full reversible lineage: checkpoint before
        // apply, mutation journal, apply audit, and an available revert.
        let checkpoint_present = lifecycle
            .rollback_checkpoint_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty());
        let journal_present = lifecycle
            .mutation_journal_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty());
        let audit_present = lifecycle
            .apply_audit_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty());
        if !lifecycle.checkpoint_captured_before_apply
            || !checkpoint_present
            || !journal_present
            || !audit_present
        {
            violations.push(AiScopedApplyHardeningViolation::ApplyLifecycleIncomplete);
        }
        let revert_ref_present = lifecycle
            .revert_handle_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty());
        if !lifecycle.revert_available || !revert_ref_present {
            violations.push(AiScopedApplyHardeningViolation::RevertUnavailableAfterApply);
        }
    }

    if state.is_terminal_no_apply() {
        // A rejected or blocked apply must not have mutated the live tree.
        let mutated = lifecycle
            .mutation_journal_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty())
            || lifecycle
                .apply_audit_ref
                .as_deref()
                .is_some_and(|reference| !reference.trim().is_empty());
        if mutated {
            violations.push(AiScopedApplyHardeningViolation::RejectedStateApplied);
        }
    }
}

fn validate_scope_and_patch(
    packet: &AiScopedApplyHardeningPacket,
    violations: &mut Vec<AiScopedApplyHardeningViolation>,
) {
    let scope = &packet.scope_contract;
    if scope.declared_scope_label.trim().is_empty() || scope.requested_scope_ref.trim().is_empty() {
        violations.push(AiScopedApplyHardeningViolation::ScopeContractIncomplete);
    }
    if !scope.apply_bounded_to_declared_scope {
        violations.push(AiScopedApplyHardeningViolation::ScopeWidenedBeyondDeclared);
    }

    let patch = &packet.patch_honesty;
    let disclosed = patch
        .files
        .iter()
        .filter(|file| file.disclosed_in_preview)
        .count() as u32;
    // The disclosed-file count and declared-file count must match the rows; a
    // patch can never carry a file the operator was not shown.
    if patch.patch_digest_ref.trim().is_empty()
        || patch.declared_file_count as usize != patch.files.len()
        || patch.disclosed_file_count != disclosed
    {
        violations.push(AiScopedApplyHardeningViolation::PatchHonestyIncomplete);
    }

    for file in &patch.files {
        if file.file_ref.trim().is_empty()
            || (file.change_kind.requires_hunks() && file.hunk_count == 0)
            || (file.change_kind.is_rename()
                && !file
                    .rename_from_ref
                    .as_deref()
                    .is_some_and(|reference| !reference.trim().is_empty()))
        {
            violations.push(AiScopedApplyHardeningViolation::PatchHonestyIncomplete);
            break;
        }
    }

    for file in &patch.files {
        if file.reached_live_tree {
            // Only disclosed, approved, in-scope hunks may reach the live tree.
            if !file.disclosed_in_preview {
                violations.push(AiScopedApplyHardeningViolation::HiddenPatchFile);
            }
            if !file.approved_for_apply {
                violations.push(AiScopedApplyHardeningViolation::UnapprovedFileApplied);
            }
            if !file.within_declared_scope {
                violations.push(AiScopedApplyHardeningViolation::ScopeWidenedBeyondDeclared);
            }
        }
    }
}

fn validate_surface_parity(
    packet: &AiScopedApplyHardeningPacket,
    violations: &mut Vec<AiScopedApplyHardeningViolation>,
) {
    for required in CommandSurfaceClass::required_coverage() {
        if !packet
            .surface_parity
            .iter()
            .any(|row| row.surface_class == required)
        {
            violations.push(AiScopedApplyHardeningViolation::CommandSurfaceCoverageMissing);
            break;
        }
    }
    for row in &packet.surface_parity {
        if row.descriptor_ref.trim().is_empty() || !row.preserves_full_parity() {
            violations.push(AiScopedApplyHardeningViolation::CommandParityBroken);
            break;
        }
    }
    for row in &packet.surface_parity {
        // A surface that claims Stable must actually qualify and preserve the full
        // shared model; otherwise it is automatically narrowed below Stable rather
        // than inheriting an adjacent green row.
        if row.claimed_stable && (!row.qualification.is_stable() || !row.preserves_full_parity()) {
            violations.push(AiScopedApplyHardeningViolation::StableClaimNotQualified);
            break;
        }
    }
}

fn validate_route_authority(
    packet: &AiScopedApplyHardeningPacket,
    violations: &mut Vec<AiScopedApplyHardeningViolation>,
) {
    let route = &packet.route_authority;
    if route.provider_label.trim().is_empty()
        || route.model_label.trim().is_empty()
        || route.route_receipt_ref.trim().is_empty()
        || route.spend_receipt_ref.trim().is_empty()
        || !route.egress_disclosed
    {
        violations.push(AiScopedApplyHardeningViolation::RouteAuthorityIncomplete);
    }
    if route.tainted_context_present
        && !route
            .tainted_context_fence_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty())
    {
        violations.push(AiScopedApplyHardeningViolation::TaintedContextUnfenced);
    }
    if route.authority_widened_without_disclosure {
        violations.push(AiScopedApplyHardeningViolation::AuthorityWidenedWithoutDisclosure);
    }
}

fn validate_evidence_export(
    packet: &AiScopedApplyHardeningPacket,
    violations: &mut Vec<AiScopedApplyHardeningViolation>,
) {
    let export = &packet.evidence_export;
    if export.evidence_packet_ref.trim().is_empty()
        || export.patch_review_summary_ref.trim().is_empty()
        || export.rollback_handle_ref.trim().is_empty()
        || export.json_export_ref.trim().is_empty()
        || export.markdown_summary_ref.trim().is_empty()
    {
        violations.push(AiScopedApplyHardeningViolation::ExportRefsMissing);
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

#[cfg(test)]
mod tests;
