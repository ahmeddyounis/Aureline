//! Evidence-rich patch review with diff packets, validation receipts, and
//! rollback handles across apply flows.
//!
//! This module ships the canonical M5 packet that binds structured diff packets,
//! validation receipts, and rollback handles into one export-safe artifact for
//! the three AI apply flows — inline assist, patch review, and branch or
//! worktree agents.
//!
//! A [`DiffPacketBlock`] carries content-addressed hunk-level diff inventory
//! with per-file and per-hunk disclosure, approval, and scope honesty. A
//! [`ValidationReceiptBlock`] carries attributable validation evidence
//! (lint, type-check, test, security-scan, format, build) bound to the exact
//! patch digest. A [`RollbackHandleBlock`] carries recoverable checkpoint
//! lineage and revert availability scoped to the apply flow.
//!
//! The packet references upstream M4/M5 lanes by id rather than embedding
//! their content: the [`crate::harden_ai_scoped_apply::AiScopedApplyHardeningPacket`]
//! apply-time invariants, the [`crate::ai_review_assist::AiReviewAssistTruthPacket`]
//! review finding lineage, the [`crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::M5AiWorkflowMatrixPacket`]
//! workflow qualification matrix, and the
//! [`crate::finalize_ai_evidence_packets::AiEvidencePacketFinalization`]
//! evidence finalization lane.
//!
//! The record is export-safe. It carries refs, state tokens, coarse classes,
//! counts, digests, and review labels only. Raw diff bodies, raw patch text,
//! raw prompt bodies, source file bodies, provider payloads, endpoint URLs,
//! credentials, raw token counts, exact prices, and billing-account ids stay
//! outside the support boundary.
//!
//! The frozen contracts this lane projects against are the multi-file patch
//! review sequence contract
//! ([`artifacts/ai/multifile_patch_review_sequence.md`](../../../../artifacts/ai/multifile_patch_review_sequence.md))
//! and the M5 AI workflow matrix contract
//! ([`docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md`](../../../../docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md)).

#[cfg(test)]
mod tests;

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`EvidenceRichPatchReviewPacket`].
pub const EVIDENCE_RICH_PATCH_REVIEW_RECORD_KIND: &str = "evidence_rich_patch_review";

/// Schema version for evidence-rich patch review records.
pub const EVIDENCE_RICH_PATCH_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const EVIDENCE_RICH_PATCH_REVIEW_SCHEMA_REF: &str =
    "schemas/ai/ship-evidence-rich-patch-review-with-diff-packets-validation-receipts-and-rollback-handles-across-apply-flows.schema.json";

/// Repo-relative path of the evidence-rich patch review contract doc.
pub const EVIDENCE_RICH_PATCH_REVIEW_DOC_REF: &str =
    "docs/ai/m5/ship_evidence_rich_patch_review_with_diff_packets_validation_receipts_and_rollback_handles_across_apply_flows.md";

/// Repo-relative path of the frozen multi-file patch review sequence contract.
pub const EVIDENCE_RICH_PATCH_REVIEW_PATCH_SEQUENCE_REF: &str =
    "artifacts/ai/multifile_patch_review_sequence.md";

/// Repo-relative path of the frozen M5 AI workflow matrix contract.
pub const EVIDENCE_RICH_PATCH_REVIEW_M5_MATRIX_CONTRACT_REF: &str =
    "docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md";

/// Repo-relative path of the frozen preview/apply/revert enforcement contract.
pub const EVIDENCE_RICH_PATCH_REVIEW_PREVIEW_APPLY_REVERT_CONTRACT_REF: &str =
    "docs/commands/alpha_preview_apply_revert.md";

/// Repo-relative path of the protected fixture directory.
pub const EVIDENCE_RICH_PATCH_REVIEW_FIXTURE_DIR: &str =
    "fixtures/ai/m5/ship_evidence_rich_patch_review_with_diff_packets_validation_receipts_and_rollback_handles_across_apply_flows";

/// Repo-relative path of the checked support-export artifact.
pub const EVIDENCE_RICH_PATCH_REVIEW_ARTIFACT_REF: &str =
    "artifacts/ai/m5/ship_evidence_rich_patch_review_with_diff_packets_validation_receipts_and_rollback_handles_across_apply_flows/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const EVIDENCE_RICH_PATCH_REVIEW_SUMMARY_REF: &str =
    "artifacts/ai/m5/ship_evidence_rich_patch_review_with_diff_packets_validation_receipts_and_rollback_handles_across_apply_flows.md";

/// One of the three M5 AI apply flows governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplyFlowClass {
    /// Inline composer assist and quick-edit workflows.
    InlineAssist,
    /// AI-assisted patch review, finding publication, and resolution.
    PatchReview,
    /// Background branch-agent or worktree-isolated long-running tasks.
    BranchOrWorktreeAgent,
}

impl ApplyFlowClass {
    /// Every flow, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::InlineAssist,
        Self::PatchReview,
        Self::BranchOrWorktreeAgent,
    ];

    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlineAssist => "inline_assist",
            Self::PatchReview => "patch_review",
            Self::BranchOrWorktreeAgent => "branch_or_worktree_agent",
        }
    }
}

/// Lifecycle state of a diff packet across propose, review, validate, approve,
/// apply, and revert.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffPacketStateClass {
    /// Proposed and awaiting review.
    Proposed,
    /// Under active review.
    UnderReview,
    /// Review completed and approved for apply.
    Approved,
    /// Approved and applied to the live tree.
    Applied,
    /// Applied and then reverted.
    Reverted,
    /// Rejected or blocked; never applied.
    Rejected,
}

impl DiffPacketStateClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proposed => "proposed",
            Self::UnderReview => "under_review",
            Self::Approved => "approved",
            Self::Applied => "applied",
            Self::Reverted => "reverted",
            Self::Rejected => "rejected",
        }
    }

    /// States that mutated the live tree.
    const fn reached_live_tree(self) -> bool {
        matches!(self, Self::Applied | Self::Reverted)
    }

    /// Terminal states where the patch was never applied.
    const fn is_terminal_no_apply(self) -> bool {
        matches!(self, Self::Rejected)
    }
}

/// Kind of validation run against a diff packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationKindClass {
    /// Static lint or style check.
    Lint,
    /// Type-system check.
    TypeCheck,
    /// Test execution.
    Test,
    /// Security or dependency scan.
    SecurityScan,
    /// Format check.
    FormatCheck,
    /// Build check.
    BuildCheck,
}

impl ValidationKindClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lint => "lint",
            Self::TypeCheck => "type_check",
            Self::Test => "test",
            Self::SecurityScan => "security_scan",
            Self::FormatCheck => "format_check",
            Self::BuildCheck => "build_check",
        }
    }
}

/// Outcome of a single validation run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationOutcomeClass {
    /// All checks passed.
    Passed,
    /// One or more checks failed.
    Failed,
    /// Partial pass with acceptable failures.
    Partial,
    /// Validation was skipped.
    Skipped,
    /// Validation blocked by policy or trust state.
    Blocked,
}

impl ValidationOutcomeClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Partial => "partial",
            Self::Skipped => "skipped",
            Self::Blocked => "blocked",
        }
    }

    /// Whether the validation outcome permits apply.
    pub const fn permits_apply(self) -> bool {
        matches!(self, Self::Passed | Self::Partial)
    }
}

/// Scope a rollback handle can recover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackScopeClass {
    /// A single workspace file.
    SingleFile,
    /// A bounded set of workspace files.
    MultiFileBounded,
    /// A workspace-wide edit.
    WorkspaceWide,
}

impl RollbackScopeClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleFile => "single_file",
            Self::MultiFileBounded => "multi_file_bounded",
            Self::WorkspaceWide => "workspace_wide",
        }
    }
}

/// Availability state of a rollback handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackStateClass {
    /// Rollback is available and has not been consumed.
    Available,
    /// Rollback was consumed and the handle is no longer valid.
    Consumed,
    /// Rollback expired without being consumed.
    Expired,
    /// Rollback is unavailable for this apply flow.
    Unavailable,
}

impl RollbackStateClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Consumed => "consumed",
            Self::Expired => "expired",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Consumer surface that must project this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurfaceClass {
    /// Desktop composer or inline assist UI.
    DesktopComposer,
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
        Self::DesktopComposer,
        Self::DesktopReviewWorkspace,
        Self::CliHeadless,
        Self::BrowserCompanion,
        Self::SupportExport,
        Self::Diagnostics,
    ];

    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopComposer => "desktop_composer",
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

/// One file disclosed in a diff packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffFileRow {
    /// Opaque file ref (never a raw absolute path).
    pub file_ref: String,
    /// Change kind for this file.
    pub change_kind: crate::harden_ai_scoped_apply::PatchChangeKind,
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
    /// Source file ref for a rename.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rename_from_ref: Option<String>,
}

/// One hunk disclosed in a diff packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffHunkRow {
    /// Opaque hunk id.
    pub hunk_id: String,
    /// Opaque file ref this hunk belongs to.
    pub file_ref: String,
    /// Content-addressed hunk digest.
    pub hunk_digest: String,
    /// True when this hunk was disclosed in the preview.
    pub disclosed_in_preview: bool,
    /// True when this hunk was approved for apply.
    pub approved_for_apply: bool,
    /// True when this hunk reached the live tree.
    pub reached_live_tree: bool,
}

/// Structured diff packet block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffPacketBlock {
    /// Content-addressed patch digest ref binding the diff packet to validation
    /// and rollback lineage.
    pub patch_digest_ref: String,
    /// Declared file count.
    pub declared_file_count: u32,
    /// Disclosed file count.
    pub disclosed_file_count: u32,
    /// Per-file diff inventory.
    pub files: Vec<DiffFileRow>,
    /// Per-hunk diff inventory.
    pub hunks: Vec<DiffHunkRow>,
    /// Opaque ref to the patch artifact (diff body) stored in the evidence
    /// cache. Never a raw diff body.
    pub patch_artifact_ref: String,
}

/// One validation receipt row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationReceiptRow {
    /// Validation kind.
    pub kind: ValidationKindClass,
    /// Validation outcome.
    pub outcome: ValidationOutcomeClass,
    /// Validator ref (opaque id of the runner).
    pub validator_ref: String,
    /// Validation run timestamp.
    pub run_at: String,
    /// Patch digest ref this validation is bound to.
    pub bound_patch_digest_ref: String,
    /// True when the validation result is disclosed before apply.
    pub disclosed_before_apply: bool,
    /// True when the validation blocked apply.
    pub blocked_apply: bool,
}

/// Validation receipt block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationReceiptBlock {
    /// Receipt id.
    pub receipt_id: String,
    /// Validation receipt rows.
    pub validations: Vec<ValidationReceiptRow>,
    /// Overall validation outcome (most severe of the rows).
    pub overall_outcome: ValidationOutcomeClass,
    /// True when at least one validation is required before apply.
    pub validation_required_before_apply: bool,
    /// True when all required validations passed.
    pub all_required_passed: bool,
}

/// Rollback handle block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackHandleBlock {
    /// Rollback handle id.
    pub handle_id: String,
    /// Rollback scope class.
    pub scope: RollbackScopeClass,
    /// Rollback state.
    pub state: RollbackStateClass,
    /// Checkpoint ref captured before apply.
    pub checkpoint_ref: String,
    /// Mutation journal ref when the apply mutated the live tree.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation_journal_ref: Option<String>,
    /// True when a revert is available after apply.
    pub revert_available: bool,
    /// Revert handle ref when a revert is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revert_handle_ref: Option<String>,
    /// Expiry timestamp for the rollback handle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// One apply-flow binding row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplyFlowBindingRow {
    /// Apply flow this row binds.
    pub flow: ApplyFlowClass,
    /// Diff packet state for this flow.
    pub diff_state: DiffPacketStateClass,
    /// True when this flow carries a diff packet.
    pub has_diff_packet: bool,
    /// True when this flow carries a validation receipt.
    pub has_validation_receipt: bool,
    /// True when this flow carries a rollback handle.
    pub has_rollback_handle: bool,
    /// True when preview was shown before apply on this flow.
    pub preview_shown: bool,
    /// True when approval was required on this flow.
    pub approval_required: bool,
    /// True when approval was granted on this flow.
    pub approval_granted: bool,
    /// True when the apply stayed bounded to declared scope on this flow.
    pub apply_bounded_to_scope: bool,
}

/// One cross-surface consumer-parity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerSurfaceParityRow {
    /// Consumer surface this row covers.
    pub surface: ConsumerSurfaceClass,
    /// True when this surface shows diff packet truth.
    pub shows_diff_packet: bool,
    /// True when this surface shows validation receipt truth.
    pub shows_validation_receipt: bool,
    /// True when this surface shows rollback handle truth.
    pub shows_rollback_handle: bool,
    /// True when this surface shows apply-flow binding truth.
    pub shows_apply_flow_binding: bool,
    /// True when this surface is reachable for this packet.
    pub reachable: bool,
    /// Qualification class for this surface projection.
    pub qualification: SurfaceQualificationClass,
    /// True when this surface claims the Stable lane.
    pub claimed_stable: bool,
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
    /// Scope expanded beyond qualified boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
    /// Validation receipt is missing or failed.
    ValidationFailedOrMissing,
    /// Rollback handle is unavailable after apply.
    RollbackUnavailable,
}

impl DowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ProviderUnavailable,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
        Self::ValidationFailedOrMissing,
        Self::RollbackUnavailable,
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
            Self::ValidationFailedOrMissing => "validation_failed_or_missing",
            Self::RollbackUnavailable => "rollback_unavailable",
        }
    }
}

/// Constructor input for [`EvidenceRichPatchReviewPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceRichPatchReviewPacketInput {
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical apply id shared across surfaces and rollback.
    pub apply_id: String,
    /// Display label.
    pub display_label: String,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this packet was evaluated under.
    pub policy_epoch_ref: String,
    /// Diff packet block.
    pub diff_packet: DiffPacketBlock,
    /// Validation receipt block.
    pub validation_receipt: ValidationReceiptBlock,
    /// Rollback handle block.
    pub rollback_handle: RollbackHandleBlock,
    /// Apply-flow binding rows.
    pub apply_flow_bindings: Vec<ApplyFlowBindingRow>,
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

/// Export-safe evidence-rich patch review record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceRichPatchReviewPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical apply id shared across surfaces and rollback.
    pub apply_id: String,
    /// Display label.
    pub display_label: String,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this packet was evaluated under.
    pub policy_epoch_ref: String,
    /// Diff packet block.
    pub diff_packet: DiffPacketBlock,
    /// Validation receipt block.
    pub validation_receipt: ValidationReceiptBlock,
    /// Rollback handle block.
    pub rollback_handle: RollbackHandleBlock,
    /// Apply-flow binding rows.
    pub apply_flow_bindings: Vec<ApplyFlowBindingRow>,
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

impl EvidenceRichPatchReviewPacket {
    /// Builds an evidence-rich patch review packet from the stable-lane input.
    pub fn new(input: EvidenceRichPatchReviewPacketInput) -> Self {
        Self {
            record_kind: EVIDENCE_RICH_PATCH_REVIEW_RECORD_KIND.to_owned(),
            schema_version: EVIDENCE_RICH_PATCH_REVIEW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            apply_id: input.apply_id,
            display_label: input.display_label,
            trust_state_token: input.trust_state_token,
            policy_epoch_ref: input.policy_epoch_ref,
            diff_packet: input.diff_packet,
            validation_receipt: input.validation_receipt,
            rollback_handle: input.rollback_handle,
            apply_flow_bindings: input.apply_flow_bindings,
            consumer_surface_parity: input.consumer_surface_parity,
            downgrade_triggers: input.downgrade_triggers,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the evidence-rich patch review packet's stable-line invariants.
    pub fn validate(&self) -> Vec<EvidenceRichPatchReviewViolation> {
        let mut violations = Vec::new();
        if self.record_kind != EVIDENCE_RICH_PATCH_REVIEW_RECORD_KIND {
            violations.push(EvidenceRichPatchReviewViolation::WrongRecordKind);
        }
        if self.schema_version != EVIDENCE_RICH_PATCH_REVIEW_SCHEMA_VERSION {
            violations.push(EvidenceRichPatchReviewViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.apply_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.trust_state_token.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(EvidenceRichPatchReviewViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_diff_packet(self, &mut violations);
        validate_validation_receipt(self, &mut violations);
        validate_rollback_handle(self, &mut violations);
        validate_apply_flow_bindings(self, &mut violations);
        validate_consumer_surface_parity(self, &mut violations);
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("evidence-rich patch review packet serializes"),
        ) {
            violations.push(EvidenceRichPatchReviewViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("evidence-rich patch review packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let applied_files = self
            .diff_packet
            .files
            .iter()
            .filter(|file| file.reached_live_tree)
            .count();
        let stable_surfaces = self
            .consumer_surface_parity
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# Evidence-Rich Patch Review\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Apply id: `{}`\n", self.apply_id));
        out.push_str(&format!(
            "- Diff state: `{}` (files: {} declared / {} disclosed / {} reached tree)\n",
            self.overall_diff_state().as_str(),
            self.diff_packet.declared_file_count,
            self.diff_packet.disclosed_file_count,
            applied_files
        ));
        out.push_str(&format!(
            "- Validation: `{}` (required: {} / all passed: {})\n",
            self.validation_receipt.overall_outcome.as_str(),
            self.validation_receipt.validation_required_before_apply,
            self.validation_receipt.all_required_passed
        ));
        out.push_str(&format!(
            "- Rollback: `{}` (available: {})\n",
            self.rollback_handle.state.as_str(),
            self.rollback_handle.revert_available
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

    /// Computes the overall diff state from apply-flow bindings.
    fn overall_diff_state(&self) -> DiffPacketStateClass {
        // Prefer the most advanced state among bindings.
        self.apply_flow_bindings
            .iter()
            .map(|binding| binding.diff_state)
            .max_by_key(|state| match state {
                DiffPacketStateClass::Proposed => 0,
                DiffPacketStateClass::UnderReview => 1,
                DiffPacketStateClass::Approved => 2,
                DiffPacketStateClass::Applied => 3,
                DiffPacketStateClass::Reverted => 4,
                DiffPacketStateClass::Rejected => 5,
            })
            .unwrap_or(DiffPacketStateClass::Proposed)
    }
}

/// Errors emitted when reading the checked-in evidence-rich patch review export.
#[derive(Debug)]
pub enum EvidenceRichPatchReviewArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<EvidenceRichPatchReviewViolation>),
}

impl fmt::Display for EvidenceRichPatchReviewArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "evidence-rich patch review export parse failed: {error}"
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
                    "evidence-rich patch review export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for EvidenceRichPatchReviewArtifactError {}

/// Validation failures emitted by [`EvidenceRichPatchReviewPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvidenceRichPatchReviewViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// Diff packet block is incomplete.
    DiffPacketIncomplete,
    /// A file reached the live tree without being disclosed in the preview.
    HiddenPatchFile,
    /// A file reached the live tree without being approved for apply.
    UnapprovedFileApplied,
    /// Declared and disclosed file counts do not match the rows.
    DiffPacketCountMismatch,
    /// Validation receipt block is incomplete.
    ValidationReceiptIncomplete,
    /// Validation receipt is missing when required.
    ValidationReceiptMissing,
    /// Validation failed but apply was still permitted.
    ValidationFailedButApplied,
    /// Rollback handle block is incomplete.
    RollbackHandleIncomplete,
    /// Rollback is unavailable after an apply mutated the live tree.
    RollbackUnavailableAfterApply,
    /// Apply-flow bindings are missing a required flow.
    ApplyFlowBindingMissing,
    /// Apply-flow binding claims apply without preview or approval.
    AppliedWithoutPreviewOrApproval,
    /// A consumer surface is not covered by the parity rows.
    ConsumerSurfaceCoverageMissing,
    /// A surface claims Stable without qualifying for it.
    StableClaimNotQualified,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl EvidenceRichPatchReviewViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::DiffPacketIncomplete => "diff_packet_incomplete",
            Self::HiddenPatchFile => "hidden_patch_file",
            Self::UnapprovedFileApplied => "unapproved_file_applied",
            Self::DiffPacketCountMismatch => "diff_packet_count_mismatch",
            Self::ValidationReceiptIncomplete => "validation_receipt_incomplete",
            Self::ValidationReceiptMissing => "validation_receipt_missing",
            Self::ValidationFailedButApplied => "validation_failed_but_applied",
            Self::RollbackHandleIncomplete => "rollback_handle_incomplete",
            Self::RollbackUnavailableAfterApply => "rollback_unavailable_after_apply",
            Self::ApplyFlowBindingMissing => "apply_flow_binding_missing",
            Self::AppliedWithoutPreviewOrApproval => "applied_without_preview_or_approval",
            Self::ConsumerSurfaceCoverageMissing => "consumer_surface_coverage_missing",
            Self::StableClaimNotQualified => "stable_claim_not_qualified",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Returns the checked-in evidence-rich patch review export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or validate.
pub fn current_stable_evidence_rich_patch_review_export(
) -> Result<EvidenceRichPatchReviewPacket, EvidenceRichPatchReviewArtifactError> {
    let packet: EvidenceRichPatchReviewPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/ship_evidence_rich_patch_review_with_diff_packets_validation_receipts_and_rollback_handles_across_apply_flows/support_export.json"
    )))
    .map_err(EvidenceRichPatchReviewArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(EvidenceRichPatchReviewArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &EvidenceRichPatchReviewPacket,
    violations: &mut Vec<EvidenceRichPatchReviewViolation>,
) {
    for required in [
        EVIDENCE_RICH_PATCH_REVIEW_DOC_REF,
        EVIDENCE_RICH_PATCH_REVIEW_SCHEMA_REF,
        EVIDENCE_RICH_PATCH_REVIEW_PATCH_SEQUENCE_REF,
        EVIDENCE_RICH_PATCH_REVIEW_M5_MATRIX_CONTRACT_REF,
        EVIDENCE_RICH_PATCH_REVIEW_PREVIEW_APPLY_REVERT_CONTRACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(EvidenceRichPatchReviewViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_diff_packet(
    packet: &EvidenceRichPatchReviewPacket,
    violations: &mut Vec<EvidenceRichPatchReviewViolation>,
) {
    let diff = &packet.diff_packet;
    if diff.patch_digest_ref.trim().is_empty()
        || diff.patch_artifact_ref.trim().is_empty()
        || diff.declared_file_count == 0
    {
        violations.push(EvidenceRichPatchReviewViolation::DiffPacketIncomplete);
        return;
    }
    let disclosed = diff
        .files
        .iter()
        .filter(|file| file.disclosed_in_preview)
        .count() as u32;
    if disclosed != diff.disclosed_file_count {
        violations.push(EvidenceRichPatchReviewViolation::DiffPacketCountMismatch);
    }
    if diff.files.len() as u32 != diff.declared_file_count {
        violations.push(EvidenceRichPatchReviewViolation::DiffPacketCountMismatch);
    }
    for file in &diff.files {
        if file.reached_live_tree && !file.disclosed_in_preview {
            violations.push(EvidenceRichPatchReviewViolation::HiddenPatchFile);
        }
        if file.reached_live_tree && !file.approved_for_apply {
            violations.push(EvidenceRichPatchReviewViolation::UnapprovedFileApplied);
        }
    }
    for hunk in &diff.hunks {
        if hunk.reached_live_tree && !hunk.disclosed_in_preview {
            violations.push(EvidenceRichPatchReviewViolation::HiddenPatchFile);
        }
        if hunk.reached_live_tree && !hunk.approved_for_apply {
            violations.push(EvidenceRichPatchReviewViolation::UnapprovedFileApplied);
        }
    }
}

fn validate_validation_receipt(
    packet: &EvidenceRichPatchReviewPacket,
    violations: &mut Vec<EvidenceRichPatchReviewViolation>,
) {
    let receipt = &packet.validation_receipt;
    if receipt.receipt_id.trim().is_empty() {
        violations.push(EvidenceRichPatchReviewViolation::ValidationReceiptIncomplete);
        return;
    }
    if receipt.validation_required_before_apply && receipt.validations.is_empty() {
        violations.push(EvidenceRichPatchReviewViolation::ValidationReceiptMissing);
    }
    for validation in &receipt.validations {
        if validation.validator_ref.trim().is_empty()
            || validation.run_at.trim().is_empty()
            || validation.bound_patch_digest_ref.trim().is_empty()
        {
            violations.push(EvidenceRichPatchReviewViolation::ValidationReceiptIncomplete);
            break;
        }
        if validation.blocked_apply && !validation.outcome.permits_apply() {
            continue;
        }
    }
    // If validation is required and not all required passed, apply should not
    // have reached the live tree.
    if receipt.validation_required_before_apply
        && !receipt.all_required_passed
        && packet
            .apply_flow_bindings
            .iter()
            .any(|binding| binding.diff_state.reached_live_tree())
    {
        violations.push(EvidenceRichPatchReviewViolation::ValidationFailedButApplied);
    }
}

fn validate_rollback_handle(
    packet: &EvidenceRichPatchReviewPacket,
    violations: &mut Vec<EvidenceRichPatchReviewViolation>,
) {
    let rollback = &packet.rollback_handle;
    if rollback.handle_id.trim().is_empty() || rollback.checkpoint_ref.trim().is_empty() {
        violations.push(EvidenceRichPatchReviewViolation::RollbackHandleIncomplete);
        return;
    }
    let any_applied = packet
        .apply_flow_bindings
        .iter()
        .any(|binding| binding.diff_state.reached_live_tree());
    if any_applied && !rollback.revert_available {
        violations.push(EvidenceRichPatchReviewViolation::RollbackUnavailableAfterApply);
    }
}

fn validate_apply_flow_bindings(
    packet: &EvidenceRichPatchReviewPacket,
    violations: &mut Vec<EvidenceRichPatchReviewViolation>,
) {
    let mut seen = std::collections::HashSet::new();
    for binding in &packet.apply_flow_bindings {
        seen.insert(binding.flow);
        if binding.diff_state.reached_live_tree()
            && (!binding.preview_shown || !binding.approval_granted)
        {
            violations.push(EvidenceRichPatchReviewViolation::AppliedWithoutPreviewOrApproval);
        }
    }
    for required in ApplyFlowClass::ALL {
        if !seen.contains(&required) {
            violations.push(EvidenceRichPatchReviewViolation::ApplyFlowBindingMissing);
            break;
        }
    }
}

fn validate_consumer_surface_parity(
    packet: &EvidenceRichPatchReviewPacket,
    violations: &mut Vec<EvidenceRichPatchReviewViolation>,
) {
    let mut seen = std::collections::HashSet::new();
    for row in &packet.consumer_surface_parity {
        seen.insert(row.surface);
        if row.claimed_stable && !row.reachable {
            violations.push(EvidenceRichPatchReviewViolation::StableClaimNotQualified);
        }
    }
    for required in ConsumerSurfaceClass::ALL {
        if !seen.contains(&required) {
            violations.push(EvidenceRichPatchReviewViolation::ConsumerSurfaceCoverageMissing);
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
