//! Richer M5 prompt composer with intent modes, typed attachments, context
//! pinning, and omitted-context truth.
//!
//! This module unifies the M3 beta conformance and M4 stable stabilization
//! lanes into one richer M5 prompt-composer packet. It adds mode-specific
//! behavior constraints, attachment semantic roles and provenance chains,
//! pinning policies with auto-refresh and stale-detection, and omitted-context
//! restoration paths that make exclusions fully replayable and reversible.
//!
//! The record is export-safe. It carries refs, state tokens, coarse classes,
//! and review labels only; raw prompt bodies, source file bodies, provider
//! payloads, endpoint URLs, credentials, raw token counts, exact prices, and
//! billing account ids stay outside the support boundary.

#[cfg(test)]
mod tests;

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::context_inspector::{
    BudgetPressureClass, ContextFreshnessClass, ContextItemStateClass, ContextOmissionReasonClass,
    ExecutionBoundaryClass, IntentModeClass,
};
use crate::prompt_composer::{
    PromptBudgetActionClass, PromptComposerConformancePacket, PromptComposerSafeFallbackClass,
    PromptEvidenceLineage, PromptEvidencePacketClass,
};
use crate::stabilize_prompt_composer::{
    CompareAnswerRow, ComposerSurfaceClass, ContextDriftBanner, ForkedThreadLineage,
    PinnedFreshnessStateClass, StableAttachmentSourceClass,
};
use crate::{SourceClass, TrustPosture};

/// Stable record-kind tag carried by [`RicherPromptComposerPacket`].
pub const RICHER_PROMPT_COMPOSER_RECORD_KIND: &str = "richer_prompt_composer_packet";

/// Schema version for richer prompt-composer records.
pub const RICHER_PROMPT_COMPOSER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const RICHER_PROMPT_COMPOSER_SCHEMA_REF: &str =
    "schemas/ai/implement-a-richer-prompt-composer-with-intent-modes-typed-attachments-context-pinning-and-omitted-context-tru.schema.json";

/// Repo-relative path of the richer prompt-composer contract doc.
pub const RICHER_PROMPT_COMPOSER_DOC_REF: &str =
    "docs/ai/m5/implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru.md";

/// Repo-relative path of the frozen AI prompt-composer contract.
pub const RICHER_PROMPT_COMPOSER_BASE_CONTRACT_REF: &str = "docs/ai/prompt_composer_contract.md";

/// Repo-relative path of the beta conformance artifact this lane extends.
pub const RICHER_PROMPT_COMPOSER_BETA_ARTIFACT_REF: &str =
    "artifacts/ai/m3/prompt_composer_conformance/support_export.json";

/// Repo-relative path of the stable stabilization artifact this lane extends.
pub const RICHER_PROMPT_COMPOSER_STABLE_ARTIFACT_REF: &str =
    "artifacts/ai/m4/prompt_composer_stabilization/support_export.json";

/// Repo-relative path of the protected fixture directory.
pub const RICHER_PROMPT_COMPOSER_FIXTURE_DIR: &str =
    "fixtures/ai/m5/implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru";

/// Repo-relative path of the checked support-export artifact.
pub const RICHER_PROMPT_COMPOSER_ARTIFACT_REF: &str =
    "artifacts/ai/m5/implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const RICHER_PROMPT_COMPOSER_SUMMARY_REF: &str =
    "artifacts/ai/m5/implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru.md";

/// Behavior constraint applied to an intent mode before send.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntentModeBehaviorConstraint {
    /// The mode requires review before any apply.
    ReviewBeforeApply,
    /// The mode requires explicit operator approval for tool invocation.
    ExplicitToolApproval,
    /// The mode narrows to read-only context and never mutates.
    ReadOnlyContext,
    /// The mode may generate tests but does not count them as coverage proof.
    GeneratedTestsNotCoverageProof,
    /// The mode may draft patches but only in a preview or worktree context.
    DraftOnlyNoInPlaceApply,
    /// The mode requires a scoped-apply hardening packet before dispatch.
    RequiresScopedApplyHardening,
    /// The mode requires an evidence packet before the turn leaves the composer.
    RequiresEvidencePacket,
}

impl IntentModeBehaviorConstraint {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewBeforeApply => "review_before_apply",
            Self::ExplicitToolApproval => "explicit_tool_approval",
            Self::ReadOnlyContext => "read_only_context",
            Self::GeneratedTestsNotCoverageProof => "generated_tests_not_coverage_proof",
            Self::DraftOnlyNoInPlaceApply => "draft_only_no_in_place_apply",
            Self::RequiresScopedApplyHardening => "requires_scoped_apply_hardening",
            Self::RequiresEvidencePacket => "requires_evidence_packet",
        }
    }

    /// Constraints required for each intent mode before the lane claims M5 rich.
    pub const fn required_for_mode(mode: IntentModeClass) -> &'static [Self] {
        match mode {
            IntentModeClass::Ask => &[Self::ReadOnlyContext, Self::RequiresEvidencePacket],
            IntentModeClass::Explain => &[Self::ReadOnlyContext, Self::RequiresEvidencePacket],
            IntentModeClass::Plan => &[Self::DraftOnlyNoInPlaceApply, Self::RequiresEvidencePacket],
            IntentModeClass::DraftPatch => &[
                Self::ReviewBeforeApply,
                Self::DraftOnlyNoInPlaceApply,
                Self::RequiresScopedApplyHardening,
                Self::RequiresEvidencePacket,
            ],
            IntentModeClass::ReviewDiff => &[Self::ReadOnlyContext, Self::RequiresEvidencePacket],
            IntentModeClass::GenerateTests => &[
                Self::GeneratedTestsNotCoverageProof,
                Self::RequiresEvidencePacket,
            ],
            IntentModeClass::RunToolWithApproval => {
                &[Self::ExplicitToolApproval, Self::RequiresEvidencePacket]
            }
        }
    }
}

/// Semantic role of an attachment within the composed context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachmentSemanticRoleClass {
    /// Primary context the turn is about.
    PrimaryContext,
    /// Reference material consulted but not modified.
    ReferenceMaterial,
    /// Instruction or policy source shaping the turn.
    InstructionSource,
    /// Test fixture or expected output.
    TestFixture,
    /// Diagnostic or error output.
    DiagnosticOutput,
    /// Generated or derived artifact.
    GeneratedArtifact,
    /// External content of unknown provenance.
    ExternalReference,
}

impl AttachmentSemanticRoleClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryContext => "primary_context",
            Self::ReferenceMaterial => "reference_material",
            Self::InstructionSource => "instruction_source",
            Self::TestFixture => "test_fixture",
            Self::DiagnosticOutput => "diagnostic_output",
            Self::GeneratedArtifact => "generated_artifact",
            Self::ExternalReference => "external_reference",
        }
    }
}

/// Provenance class for an attached object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachmentProvenanceClass {
    /// Directly from the operator's workspace.
    DirectWorkspace,
    /// Retrieved via a search or graph query.
    RetrievedQuery,
    /// Imported from an external source.
    ExternalImport,
    /// Generated by a prior AI turn or tool.
    AiGenerated,
    /// From a docs or knowledge pack.
    DocsKnowledgePack,
    /// From a terminal or runtime capture.
    RuntimeCapture,
}

impl AttachmentProvenanceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectWorkspace => "direct_workspace",
            Self::RetrievedQuery => "retrieved_query",
            Self::ExternalImport => "external_import",
            Self::AiGenerated => "ai_generated",
            Self::DocsKnowledgePack => "docs_knowledge_pack",
            Self::RuntimeCapture => "runtime_capture",
        }
    }
}

/// Policy governing how a pinned context object behaves over time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PinPolicyClass {
    /// Operator must manually refresh or remove.
    ManualOnly,
    /// Auto-refresh when the underlying object changes.
    AutoRefreshOnChange,
    /// Auto-refresh on a fixed interval.
    AutoRefreshOnInterval,
    /// Pin becomes stale after a duration and blocks send.
    StaleAfterDuration,
}

impl PinPolicyClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManualOnly => "manual_only",
            Self::AutoRefreshOnChange => "auto_refresh_on_change",
            Self::AutoRefreshOnInterval => "auto_refresh_on_interval",
            Self::StaleAfterDuration => "stale_after_duration",
        }
    }
}

/// Auto-refresh behavior for a pinned context object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PinAutoRefreshClass {
    /// Refresh happens immediately on change.
    Immediate,
    /// Refresh is deferred to a background task.
    Deferred,
    /// No auto-refresh; operator must act.
    None,
}

impl PinAutoRefreshClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Immediate => "immediate",
            Self::Deferred => "deferred",
            Self::None => "none",
        }
    }
}

/// How an omitted context object can be restored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OmittedContextRestorationClass {
    /// One-click restore without re-review.
    OneClickRestore,
    /// Restore requires the operator to re-review the context.
    RequiresReReview,
    /// Permanently excluded by policy.
    PermanentlyExcluded,
    /// Excluded for this session only.
    SessionOnlyExcluded,
}

impl OmittedContextRestorationClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OneClickRestore => "one_click_restore",
            Self::RequiresReReview => "requires_re_review",
            Self::PermanentlyExcluded => "permanently_excluded",
            Self::SessionOnlyExcluded => "session_only_excluded",
        }
    }
}

/// Freshness of the omission reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExclusionFreshnessClass {
    /// The reason the context was omitted is still valid.
    ReasonStillValid,
    /// The reason has expired (e.g., budget recovered).
    ReasonExpired,
    /// The underlying condition changed.
    ConditionChanged,
}

impl ExclusionFreshnessClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReasonStillValid => "reason_still_valid",
            Self::ReasonExpired => "reason_expired",
            Self::ConditionChanged => "condition_changed",
        }
    }
}

/// Richer intent mode row shown at the top of the composer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RicherIntentModeRow {
    /// Explicit intent mode.
    pub mode_class: IntentModeClass,
    /// Current scope label.
    pub current_scope_label: String,
    /// Current execution boundary.
    pub execution_boundary_class: ExecutionBoundaryClass,
    /// Optional action or command identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_identity_ref: Option<String>,
    /// Behavior constraints applied to this mode.
    pub behavior_constraints: Vec<IntentModeBehaviorConstraint>,
    /// Required tool-pack refs for this mode.
    pub required_tool_pack_refs: Vec<String>,
    /// Approval posture class for this mode.
    pub approval_posture_class: String,
}

/// One richer typed attachment with semantic role and provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RicherAttachmentRow {
    /// Attachment id shared with the conformance packet.
    pub attachment_id: String,
    /// Stable object identity that survives display-label changes.
    pub stable_object_ref: String,
    /// Review-safe origin label.
    pub origin_label: String,
    /// Typed source class.
    pub source_class: StableAttachmentSourceClass,
    /// Trust posture.
    pub trust_posture: TrustPosture,
    /// Freshness class shown on the pill.
    pub freshness_class: ContextFreshnessClass,
    /// Semantic role within the composed context.
    pub semantic_role: AttachmentSemanticRoleClass,
    /// Provenance class.
    pub provenance_class: AttachmentProvenanceClass,
    /// Intent modes for which this attachment is relevant.
    pub mode_relevance: Vec<IntentModeClass>,
    /// Current inclusion posture before send.
    pub context_state: ContextItemStateClass,
    /// Display label safe for review and export.
    pub display_label: String,
    /// Rough byte estimate used for the budget strip.
    pub estimated_byte_size: u64,
    /// Preview action ref.
    pub preview_action_ref: String,
    /// Open-source action ref.
    pub open_action_ref: String,
    /// Individual remove action ref.
    pub remove_action_ref: String,
    /// True when keyboard users can focus and remove the pill.
    pub keyboard_reachable: bool,
    /// Screen-reader narration label.
    pub screen_reader_label: String,
}

/// One richer pinned context row with policy and auto-refresh.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RicherPinnedContextRow {
    /// Stable pin id.
    pub pin_id: String,
    /// Stable object identity of the pinned object.
    pub stable_object_ref: String,
    /// Review-safe display label.
    pub display_label: String,
    /// Pinned freshness state.
    pub freshness_state: PinnedFreshnessStateClass,
    /// What changed underneath the pin, when stale.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drift_source: Option<crate::stabilize_prompt_composer::DriftSourceClass>,
    /// Pin policy.
    pub pin_policy: PinPolicyClass,
    /// Auto-refresh behavior.
    pub auto_refresh: PinAutoRefreshClass,
    /// Stale-after duration in seconds when policy is StaleAfterDuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stale_after_duration_seconds: Option<u64>,
    /// Refresh action ref.
    pub refresh_action_ref: String,
    /// Remove action ref.
    pub remove_action_ref: String,
    /// True when a stale pin blocks send until refreshed or removed.
    pub blocks_send_until_resolved: bool,
    /// True when keyboard users can focus, refresh, and remove the pin.
    pub keyboard_reachable: bool,
}

/// One richer omitted source that stays inspectable and restorable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RicherOmittedContextRow {
    /// Stable object identity of the omitted source.
    pub source_ref: String,
    /// Source class token.
    pub source_class: SourceClass,
    /// Why the source was omitted.
    pub omission_reason_class: ContextOmissionReasonClass,
    /// How the source can be restored.
    pub restoration_class: OmittedContextRestorationClass,
    /// Freshness of the exclusion reason.
    pub exclusion_freshness: ExclusionFreshnessClass,
    /// True when the omitted source remains inspectable after send.
    pub inspectable_after_send: bool,
    /// Inspect action ref.
    pub inspect_action_ref: String,
    /// Restoration action ref.
    pub restoration_action_ref: String,
    /// True when replay, support, and audit flows can explain the exclusion.
    pub replay_explains_exclusion: bool,
    /// True when keyboard users can reach the omitted-context review row.
    pub keyboard_reachable: bool,
}

/// One richer budget decision row shown before send.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RicherBudgetDecisionRow {
    /// Stable decision row id.
    pub decision_id: String,
    /// Source ref being included, omitted, summarized, or route-switched.
    pub source_ref: String,
    /// Source class token.
    pub source_class: SourceClass,
    /// Resulting context state token.
    pub context_state: ContextItemStateClass,
    /// Budget action class.
    pub action_class: PromptBudgetActionClass,
    /// Omission, trim, summary, or route-change reason token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_token: Option<String>,
    /// Rough byte estimate.
    pub estimated_byte_size: u64,
    /// Route receipt ref when this row describes a route switch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route_receipt_ref: Option<String>,
    /// True when this decision was driven by a pin policy rather than budget.
    pub driven_by_pin_policy: bool,
}

/// Richer budget strip shown before send.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RicherBudgetStrip {
    /// Aggregate byte estimate from the context snapshot.
    pub aggregate_byte_estimate: u64,
    /// Budget ceiling from the composer draft.
    pub budget_byte_ceiling: u64,
    /// Pressure class.
    pub pressure_class: BudgetPressureClass,
    /// Included context group tokens.
    pub included_context_group_tokens: Vec<String>,
    /// Omitted, blocked, stale, tainted, summarized, or trimmed group tokens.
    pub omitted_or_trimmed_group_tokens: Vec<String>,
    /// Budget decisions that explain what changed under pressure.
    pub decision_rows: Vec<RicherBudgetDecisionRow>,
    /// Safe non-AI fallback path.
    pub safe_fallback_class: PromptComposerSafeFallbackClass,
    /// Review-safe explanation label.
    pub explanation_label: String,
}

/// Richer thread header showing scope, route, retention, and memory access.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RicherThreadHeader {
    /// Stable thread id.
    pub thread_id: String,
    /// Current scope label.
    pub current_scope_label: String,
    /// Selected provider label.
    pub provider_label: String,
    /// Selected model label.
    pub model_label: String,
    /// Execution boundary class.
    pub execution_boundary_class: ExecutionBoundaryClass,
    /// Retention mode for the thread.
    pub retention_mode_class: crate::stabilize_prompt_composer::ThreadRetentionModeClass,
    /// Memory class token shared with the AI memory model.
    pub memory_class_token: String,
    /// Save-memory action ref.
    pub save_memory_action_ref: String,
    /// Delete action ref.
    pub delete_action_ref: String,
    /// Export action ref.
    pub export_action_ref: String,
    /// Remember/Save preview.
    pub remember_preview: crate::stabilize_prompt_composer::RememberPreview,
}

/// One richer surface-consistency row proving cross-surface reachability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RicherSurfaceConsistencyRow {
    /// Composer surface class.
    pub surface_class: ComposerSurfaceClass,
    /// True when attachment pills are keyboard reachable on this surface.
    pub attachment_pills_keyboard_reachable: bool,
    /// True when mention rows are screen-reader describable on this surface.
    pub mention_rows_screen_reader_describable: bool,
    /// True when omitted-context review is reachable on this surface.
    pub omitted_context_review_reachable: bool,
    /// True when pinned-context review is reachable on this surface.
    pub pinned_context_review_reachable: bool,
    /// True when context-drift banners are reachable on this surface.
    pub context_drift_banner_reachable: bool,
    /// True when intent-mode constraints are visible on this surface.
    pub intent_mode_constraints_visible: bool,
}

impl RicherSurfaceConsistencyRow {
    fn is_fully_reachable(&self) -> bool {
        self.attachment_pills_keyboard_reachable
            && self.mention_rows_screen_reader_describable
            && self.omitted_context_review_reachable
            && self.pinned_context_review_reachable
            && self.context_drift_banner_reachable
            && self.intent_mode_constraints_visible
    }
}

/// Constructor input for [`RicherPromptComposerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RicherPromptComposerInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Display label.
    pub display_label: String,
    /// M3 conformance packet id this lane extends.
    pub composer_conformance_packet_ref: String,
    /// M4 stabilization packet id this lane extends.
    pub composer_stabilization_packet_ref: String,
    /// Conformance context snapshot ref.
    pub composer_context_snapshot_ref: String,
    /// Conformance session ref.
    pub composer_session_ref: String,
    /// Conformance draft ref.
    pub composer_draft_ref: String,
    /// Richer thread header.
    pub thread_header: RicherThreadHeader,
    /// Richer intent mode row.
    pub intent_row: RicherIntentModeRow,
    /// Richer attachment rows.
    pub attachment_rows: Vec<RicherAttachmentRow>,
    /// Richer pinned context rows.
    pub pinned_context_rows: Vec<RicherPinnedContextRow>,
    /// Richer omitted-context review rows.
    pub omitted_context_rows: Vec<RicherOmittedContextRow>,
    /// Richer budget strip.
    pub budget_strip: RicherBudgetStrip,
    /// Context-drift banners.
    pub context_drift_banners: Vec<ContextDriftBanner>,
    /// Compare-answer rows.
    pub compare_answer_rows: Vec<CompareAnswerRow>,
    /// Forked-thread lineage.
    pub forked_thread_lineage: ForkedThreadLineage,
    /// Surface-consistency rows.
    pub surface_consistency_rows: Vec<RicherSurfaceConsistencyRow>,
    /// Evidence lineage.
    pub evidence_lineage: PromptEvidenceLineage,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// JSON export ref.
    pub json_export_ref: String,
    /// Markdown summary ref.
    pub markdown_summary_ref: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe richer M5 prompt-composer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RicherPromptComposerPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Display label.
    pub display_label: String,
    /// M3 conformance packet id this lane extends.
    pub composer_conformance_packet_ref: String,
    /// M4 stabilization packet id this lane extends.
    pub composer_stabilization_packet_ref: String,
    /// Conformance context snapshot ref.
    pub composer_context_snapshot_ref: String,
    /// Conformance session ref.
    pub composer_session_ref: String,
    /// Conformance draft ref.
    pub composer_draft_ref: String,
    /// Richer thread header.
    pub thread_header: RicherThreadHeader,
    /// Richer intent mode row.
    pub intent_row: RicherIntentModeRow,
    /// Richer attachment rows.
    pub attachment_rows: Vec<RicherAttachmentRow>,
    /// Richer pinned context rows.
    pub pinned_context_rows: Vec<RicherPinnedContextRow>,
    /// Richer omitted-context review rows.
    pub omitted_context_rows: Vec<RicherOmittedContextRow>,
    /// Richer budget strip.
    pub budget_strip: RicherBudgetStrip,
    /// Context-drift banners.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context_drift_banners: Vec<ContextDriftBanner>,
    /// Compare-answer rows.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub compare_answer_rows: Vec<CompareAnswerRow>,
    /// Forked-thread lineage.
    pub forked_thread_lineage: ForkedThreadLineage,
    /// Surface-consistency rows.
    pub surface_consistency_rows: Vec<RicherSurfaceConsistencyRow>,
    /// Evidence/route/spend/redaction lineage.
    pub evidence_lineage: PromptEvidenceLineage,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// JSON export ref.
    pub json_export_ref: String,
    /// Markdown summary ref.
    pub markdown_summary_ref: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl RicherPromptComposerPacket {
    /// Builds a richer prompt-composer packet from the M5 input.
    pub fn new(input: RicherPromptComposerInput) -> Self {
        Self {
            record_kind: RICHER_PROMPT_COMPOSER_RECORD_KIND.to_owned(),
            schema_version: RICHER_PROMPT_COMPOSER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            display_label: input.display_label,
            composer_conformance_packet_ref: input.composer_conformance_packet_ref,
            composer_stabilization_packet_ref: input.composer_stabilization_packet_ref,
            composer_context_snapshot_ref: input.composer_context_snapshot_ref,
            composer_session_ref: input.composer_session_ref,
            composer_draft_ref: input.composer_draft_ref,
            thread_header: input.thread_header,
            intent_row: input.intent_row,
            attachment_rows: input.attachment_rows,
            pinned_context_rows: input.pinned_context_rows,
            omitted_context_rows: input.omitted_context_rows,
            budget_strip: input.budget_strip,
            context_drift_banners: input.context_drift_banners,
            compare_answer_rows: input.compare_answer_rows,
            forked_thread_lineage: input.forked_thread_lineage,
            surface_consistency_rows: input.surface_consistency_rows,
            evidence_lineage: input.evidence_lineage,
            source_contract_refs: input.source_contract_refs,
            json_export_ref: input.json_export_ref,
            markdown_summary_ref: input.markdown_summary_ref,
            minted_at: input.minted_at,
        }
    }

    /// Validates the richer packet and the conformance/stabilization packets it extends.
    pub fn validate(
        &self,
        conformance: &PromptComposerConformancePacket,
        stabilization: &crate::stabilize_prompt_composer::PromptComposerStabilizationPacket,
    ) -> Vec<RicherPromptComposerViolation> {
        let mut violations = self.validate_self();
        if !conformance.validate().is_empty()
            || conformance.packet_id != self.composer_conformance_packet_ref
            || conformance.composer_context_snapshot_ref != self.composer_context_snapshot_ref
            || conformance.composer_session_id != self.composer_session_ref
            || conformance.composer_draft_id != self.composer_draft_ref
        {
            violations.push(RicherPromptComposerViolation::EmbeddedConformanceInvalid);
        }
        if !stabilization.validate_self().is_empty()
            || stabilization.composer_conformance_packet_ref != self.composer_conformance_packet_ref
            || stabilization.composer_context_snapshot_ref != self.composer_context_snapshot_ref
            || stabilization.composer_session_ref != self.composer_session_ref
            || stabilization.composer_draft_ref != self.composer_draft_ref
        {
            violations.push(RicherPromptComposerViolation::EmbeddedStabilizationInvalid);
        }
        violations
    }

    /// Validates only the richer packet's own M5 invariants.
    pub fn validate_self(&self) -> Vec<RicherPromptComposerViolation> {
        let mut violations = Vec::new();
        if self.record_kind != RICHER_PROMPT_COMPOSER_RECORD_KIND {
            violations.push(RicherPromptComposerViolation::WrongRecordKind);
        }
        if self.schema_version != RICHER_PROMPT_COMPOSER_SCHEMA_VERSION {
            violations.push(RicherPromptComposerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.composer_conformance_packet_ref.trim().is_empty()
            || self.composer_stabilization_packet_ref.trim().is_empty()
            || self.composer_context_snapshot_ref.trim().is_empty()
            || self.composer_session_ref.trim().is_empty()
            || self.composer_draft_ref.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(RicherPromptComposerViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_intent_row(self, &mut violations);
        validate_attachments(self, &mut violations);
        validate_pinned_context(self, &mut violations);
        validate_omitted_context(self, &mut violations);
        validate_budget(self, &mut violations);
        validate_thread_header(self, &mut violations);
        validate_surface_consistency(self, &mut violations);
        validate_evidence_lineage(self, &mut violations);
        if self.json_export_ref.trim().is_empty() || self.markdown_summary_ref.trim().is_empty() {
            violations.push(RicherPromptComposerViolation::ExportRefsMissing);
        }
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("richer prompt composer packet serializes"),
        ) {
            violations.push(RicherPromptComposerViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("richer prompt composer packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Richer Prompt Composer (M5)\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Extends conformance: `{}` / stabilization: `{}`\n",
            self.composer_conformance_packet_ref, self.composer_stabilization_packet_ref
        ));
        out.push_str(&format!(
            "- Thread: `{}` / scope `{}` / route `{} / {}`\n",
            self.thread_header.thread_id,
            self.thread_header.current_scope_label,
            self.thread_header.provider_label,
            self.thread_header.model_label
        ));
        out.push_str(&format!(
            "- Intent mode: `{}` with {} behavior constraints\n",
            self.intent_row.mode_class.as_str(),
            self.intent_row.behavior_constraints.len()
        ));
        out.push_str(&format!(
            "- Attachments / pinned / omitted-review: {} / {} / {}\n",
            self.attachment_rows.len(),
            self.pinned_context_rows.len(),
            self.omitted_context_rows.len()
        ));
        out.push_str(&format!(
            "- Budget: `{}` ({} decisions)\n",
            self.budget_strip.pressure_class.as_str(),
            self.budget_strip.decision_rows.len()
        ));
        out.push_str(&format!(
            "- Surfaces proven consistent: {}\n",
            self.surface_consistency_rows.len()
        ));
        out.push_str(&format!(
            "- Drift banners / compare-answer rows: {} / {}\n",
            self.context_drift_banners.len(),
            self.compare_answer_rows.len()
        ));
        out.push_str(&format!(
            "- Evidence: `{}` / route `{}` / spend `{}`\n",
            self.evidence_lineage.evidence_id,
            self.evidence_lineage.route_receipt_ref,
            self.evidence_lineage.spend_receipt_ref
        ));
        out
    }
}

/// Errors emitted when reading the checked-in richer export.
#[derive(Debug)]
pub enum RicherPromptComposerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RicherPromptComposerViolation>),
}

impl fmt::Display for RicherPromptComposerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "richer prompt composer export parse failed: {error}"
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
                    "richer prompt composer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for RicherPromptComposerArtifactError {}

/// Validation failures emitted by [`RicherPromptComposerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RicherPromptComposerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The promoted conformance packet is invalid or mismatched.
    EmbeddedConformanceInvalid,
    /// The promoted stabilization packet is invalid or mismatched.
    EmbeddedStabilizationInvalid,
    /// Intent mode row lacks required behavior constraints.
    IntentModeConstraintsMissing,
    /// An attachment row lacks semantic role or provenance.
    AttachmentRichnessIncomplete,
    /// Required typed source classes are not covered.
    AttachmentSourceClassCoverageMissing,
    /// A pinned row lacks policy or auto-refresh disclosure.
    PinPolicyIncomplete,
    /// A pinned-but-stale row does not surface staleness before reuse.
    PinnedStaleNotSurfaced,
    /// An omitted source is not inspectable or restorable after send.
    OmittedContextNotInspectable,
    /// Budget overflow lacks omission, summary, or route-switch explanation.
    BudgetOverflowWithoutExplanation,
    /// Thread header scope, route, retention, or memory access is incomplete.
    ThreadHeaderIncomplete,
    /// Cross-surface keyboard/screen-reader consistency is not proven.
    SurfaceConsistencyMissing,
    /// Evidence lineage lacks core refs.
    EvidenceLineageIncomplete,
    /// Required evidence packet class is missing.
    EvidencePacketClassMissing,
    /// Export refs are missing.
    ExportRefsMissing,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl RicherPromptComposerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::EmbeddedConformanceInvalid => "embedded_conformance_invalid",
            Self::EmbeddedStabilizationInvalid => "embedded_stabilization_invalid",
            Self::IntentModeConstraintsMissing => "intent_mode_constraints_missing",
            Self::AttachmentRichnessIncomplete => "attachment_richness_incomplete",
            Self::AttachmentSourceClassCoverageMissing => {
                "attachment_source_class_coverage_missing"
            }
            Self::PinPolicyIncomplete => "pin_policy_incomplete",
            Self::PinnedStaleNotSurfaced => "pinned_stale_not_surfaced",
            Self::OmittedContextNotInspectable => "omitted_context_not_inspectable",
            Self::BudgetOverflowWithoutExplanation => "budget_overflow_without_explanation",
            Self::ThreadHeaderIncomplete => "thread_header_incomplete",
            Self::SurfaceConsistencyMissing => "surface_consistency_missing",
            Self::EvidenceLineageIncomplete => "evidence_lineage_incomplete",
            Self::EvidencePacketClassMissing => "evidence_packet_class_missing",
            Self::ExportRefsMissing => "export_refs_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Returns the checked-in richer prompt-composer export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or validate.
pub fn current_richer_prompt_composer_export(
) -> Result<RicherPromptComposerPacket, RicherPromptComposerArtifactError> {
    let packet: RicherPromptComposerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru/support_export.json"
    )))
    .map_err(RicherPromptComposerArtifactError::SupportExport)?;
    let violations = packet.validate_self();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RicherPromptComposerArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &RicherPromptComposerPacket,
    violations: &mut Vec<RicherPromptComposerViolation>,
) {
    for required in [
        RICHER_PROMPT_COMPOSER_DOC_REF,
        RICHER_PROMPT_COMPOSER_BASE_CONTRACT_REF,
        RICHER_PROMPT_COMPOSER_SCHEMA_REF,
        RICHER_PROMPT_COMPOSER_BETA_ARTIFACT_REF,
        RICHER_PROMPT_COMPOSER_STABLE_ARTIFACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(RicherPromptComposerViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_intent_row(
    packet: &RicherPromptComposerPacket,
    violations: &mut Vec<RicherPromptComposerViolation>,
) {
    let row = &packet.intent_row;
    if row.current_scope_label.trim().is_empty() || row.approval_posture_class.trim().is_empty() {
        violations.push(RicherPromptComposerViolation::IntentModeConstraintsMissing);
        return;
    }
    let required = IntentModeBehaviorConstraint::required_for_mode(row.mode_class);
    for constraint in required {
        if !row.behavior_constraints.contains(constraint) {
            violations.push(RicherPromptComposerViolation::IntentModeConstraintsMissing);
            return;
        }
    }
}

fn validate_attachments(
    packet: &RicherPromptComposerPacket,
    violations: &mut Vec<RicherPromptComposerViolation>,
) {
    for row in &packet.attachment_rows {
        if row.attachment_id.trim().is_empty()
            || row.stable_object_ref.trim().is_empty()
            || row.origin_label.trim().is_empty()
            || row.display_label.trim().is_empty()
            || row.screen_reader_label.trim().is_empty()
            || !row.keyboard_reachable
            || row.preview_action_ref.trim().is_empty()
            || row.open_action_ref.trim().is_empty()
            || row.remove_action_ref.trim().is_empty()
        {
            violations.push(RicherPromptComposerViolation::AttachmentRichnessIncomplete);
            break;
        }
    }
    for required in StableAttachmentSourceClass::required_coverage() {
        if !packet
            .attachment_rows
            .iter()
            .any(|row| row.source_class == required)
        {
            violations.push(RicherPromptComposerViolation::AttachmentSourceClassCoverageMissing);
            break;
        }
    }
}

fn validate_pinned_context(
    packet: &RicherPromptComposerPacket,
    violations: &mut Vec<RicherPromptComposerViolation>,
) {
    for row in &packet.pinned_context_rows {
        let stale = row.freshness_state == PinnedFreshnessStateClass::PinnedButStale;
        if row.pin_id.trim().is_empty()
            || row.stable_object_ref.trim().is_empty()
            || row.refresh_action_ref.trim().is_empty()
            || row.remove_action_ref.trim().is_empty()
            || !row.keyboard_reachable
        {
            violations.push(RicherPromptComposerViolation::PinPolicyIncomplete);
            return;
        }
        if row.drift_source.is_some() && !stale {
            violations.push(RicherPromptComposerViolation::PinnedStaleNotSurfaced);
            return;
        }
        if stale && (row.drift_source.is_none() || !row.blocks_send_until_resolved) {
            violations.push(RicherPromptComposerViolation::PinnedStaleNotSurfaced);
            return;
        }
        if row.pin_policy == PinPolicyClass::StaleAfterDuration
            && row.stale_after_duration_seconds.is_none()
        {
            violations.push(RicherPromptComposerViolation::PinPolicyIncomplete);
            return;
        }
    }
}

fn validate_omitted_context(
    packet: &RicherPromptComposerPacket,
    violations: &mut Vec<RicherPromptComposerViolation>,
) {
    for row in &packet.omitted_context_rows {
        if row.source_ref.trim().is_empty()
            || row.inspect_action_ref.trim().is_empty()
            || row.restoration_action_ref.trim().is_empty()
            || !row.inspectable_after_send
            || !row.replay_explains_exclusion
            || !row.keyboard_reachable
        {
            violations.push(RicherPromptComposerViolation::OmittedContextNotInspectable);
            break;
        }
        if row.restoration_class == OmittedContextRestorationClass::OneClickRestore
            && row.exclusion_freshness != ExclusionFreshnessClass::ReasonStillValid
        {
            violations.push(RicherPromptComposerViolation::OmittedContextNotInspectable);
            break;
        }
    }
}

fn validate_budget(
    packet: &RicherPromptComposerPacket,
    violations: &mut Vec<RicherPromptComposerViolation>,
) {
    if packet.budget_strip.pressure_class != BudgetPressureClass::Overflow {
        return;
    }
    let explains_pressure = packet.budget_strip.decision_rows.iter().any(|row| {
        matches!(
            row.action_class,
            PromptBudgetActionClass::Omit
                | PromptBudgetActionClass::Summarize
                | PromptBudgetActionClass::Trim
                | PromptBudgetActionClass::RouteSwitch
                | PromptBudgetActionClass::Block
        ) && row
            .reason_token
            .as_deref()
            .is_some_and(|reason| !reason.trim().is_empty())
    });
    if !explains_pressure || packet.budget_strip.explanation_label.trim().is_empty() {
        violations.push(RicherPromptComposerViolation::BudgetOverflowWithoutExplanation);
    }
}

fn validate_thread_header(
    packet: &RicherPromptComposerPacket,
    violations: &mut Vec<RicherPromptComposerViolation>,
) {
    let header = &packet.thread_header;
    if header.thread_id.trim().is_empty()
        || header.current_scope_label.trim().is_empty()
        || header.provider_label.trim().is_empty()
        || header.model_label.trim().is_empty()
        || header.memory_class_token.trim().is_empty()
        || header.save_memory_action_ref.trim().is_empty()
        || header.delete_action_ref.trim().is_empty()
        || header.export_action_ref.trim().is_empty()
    {
        violations.push(RicherPromptComposerViolation::ThreadHeaderIncomplete);
    }
    let preview = &header.remember_preview;
    let summary_describes_retention = !preview.retained_summary_label.trim().is_empty();
    let audience_matches_mode = match header.retention_mode_class {
        crate::stabilize_prompt_composer::ThreadRetentionModeClass::EphemeralNoRetention => {
            preview.retention_locus_class
                == crate::stabilize_prompt_composer::RetentionLocusClass::LocalDevice
                && preview.reuse_audience_class
                    == crate::stabilize_prompt_composer::ReuseAudienceClass::Nobody
        }
        crate::stabilize_prompt_composer::ThreadRetentionModeClass::LocalOnly => {
            preview.retention_locus_class
                == crate::stabilize_prompt_composer::RetentionLocusClass::LocalDevice
        }
        _ => {
            matches!(
                header.retention_mode_class,
                crate::stabilize_prompt_composer::ThreadRetentionModeClass::RepoShared
                    | crate::stabilize_prompt_composer::ThreadRetentionModeClass::OrgShared
            ) && preview.reuse_audience_class
                != crate::stabilize_prompt_composer::ReuseAudienceClass::Nobody
        }
    };
    if !summary_describes_retention
        || preview.preview_action_ref.trim().is_empty()
        || preview.memory_class_token.trim().is_empty()
        || !audience_matches_mode
    {
        violations.push(RicherPromptComposerViolation::ThreadHeaderIncomplete);
    }
}

fn validate_surface_consistency(
    packet: &RicherPromptComposerPacket,
    violations: &mut Vec<RicherPromptComposerViolation>,
) {
    for required in ComposerSurfaceClass::required_coverage() {
        let covered = packet
            .surface_consistency_rows
            .iter()
            .find(|row| row.surface_class == required);
        match covered {
            Some(row) if row.is_fully_reachable() => {}
            _ => {
                violations.push(RicherPromptComposerViolation::SurfaceConsistencyMissing);
                return;
            }
        }
    }
}

fn validate_evidence_lineage(
    packet: &RicherPromptComposerPacket,
    violations: &mut Vec<RicherPromptComposerViolation>,
) {
    let lineage = &packet.evidence_lineage;
    if lineage.evidence_id.trim().is_empty()
        || lineage.composer_session_ref != packet.composer_session_ref
        || lineage.turn_draft_ref != packet.composer_draft_ref
        || lineage.composer_context_snapshot_ref != packet.composer_context_snapshot_ref
        || lineage.route_receipt_ref.trim().is_empty()
        || lineage.spend_receipt_ref.trim().is_empty()
        || lineage.redaction_manifest_ref.trim().is_empty()
        || lineage.replay_lineage_ref.trim().is_empty()
        || lineage.operator_packet_ref.trim().is_empty()
        || lineage.support_packet_ref.trim().is_empty()
        || lineage.compliance_packet_ref.trim().is_empty()
    {
        violations.push(RicherPromptComposerViolation::EvidenceLineageIncomplete);
        return;
    }
    for required in [
        PromptEvidencePacketClass::InlineStub,
        PromptEvidencePacketClass::OperatorPacket,
        PromptEvidencePacketClass::SupportPacket,
        PromptEvidencePacketClass::ComplianceAuditPacket,
    ] {
        if !lineage.packet_classes.contains(&required) {
            violations.push(RicherPromptComposerViolation::EvidencePacketClassMissing);
            return;
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
