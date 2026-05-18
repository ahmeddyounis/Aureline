//! Editor-facing code-action preview and tainted-evidence admission records.
//!
//! The language crate owns provider and mutation truth for code actions. This
//! module projects that truth into the editor assist surface so quick fixes can
//! share the same preview, approval, rollback, and tainted-context posture as
//! completion and snippet assistance.

use aureline_language::{
    CodeActionApplyPostureClass, CodeActionBlockingReasonClass, CodeActionMutationCounts,
    CodeActionPreviewRequirementClass, CodeActionRecord, CodeActionValidationPlan, RedactionClass,
};
use serde::{Deserialize, Serialize};

/// Integer schema version for editor code-action preview payloads.
pub type CodeActionPreviewSchemaVersion = u32;

/// Schema version used by editor code-action preview records.
pub const CODE_ACTION_PREVIEW_SCHEMA_VERSION: CodeActionPreviewSchemaVersion = 1;

/// Trust posture for evidence used to propose a quick fix or code action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuickFixEvidenceTrustClass {
    /// Evidence came from current semantic or deterministic provider truth.
    TrustedSemantic,
    /// A trusted parser promoted structured tool output into action evidence.
    TrustedParserPromoted,
    /// The user explicitly promoted the cited evidence slice.
    UserPromoted,
    /// Evidence came from terminal output and remains tainted.
    TaintedTerminalOutput,
    /// Evidence came from build logs and remains tainted.
    TaintedBuildLog,
    /// Evidence came from runtime, test, debug, or notebook logs and remains tainted.
    TaintedRuntimeLog,
    /// Evidence came from an imported diagnostic or support snapshot and remains tainted.
    TaintedImportedDiagnostic,
    /// The producer did not declare a trust posture.
    UnknownUntrusted,
}

impl QuickFixEvidenceTrustClass {
    /// Returns true when the evidence can participate in direct semantic apply.
    pub const fn is_promoted(self) -> bool {
        matches!(
            self,
            Self::TrustedSemantic | Self::TrustedParserPromoted | Self::UserPromoted
        )
    }

    /// Returns true when the evidence remains tainted or ambiguous.
    pub const fn is_tainted(self) -> bool {
        matches!(
            self,
            Self::TaintedTerminalOutput
                | Self::TaintedBuildLog
                | Self::TaintedRuntimeLog
                | Self::TaintedImportedDiagnostic
                | Self::UnknownUntrusted
        )
    }

    /// Returns true when a preview or approval fence is required before mutation.
    pub const fn requires_review(self) -> bool {
        !self.is_promoted()
    }

    /// Returns the stable schema token for this trust class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedSemantic => "trusted_semantic",
            Self::TrustedParserPromoted => "trusted_parser_promoted",
            Self::UserPromoted => "user_promoted",
            Self::TaintedTerminalOutput => "tainted_terminal_output",
            Self::TaintedBuildLog => "tainted_build_log",
            Self::TaintedRuntimeLog => "tainted_runtime_log",
            Self::TaintedImportedDiagnostic => "tainted_imported_diagnostic",
            Self::UnknownUntrusted => "unknown_untrusted",
        }
    }
}

/// Editor admission decision for one quick-fix preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeActionPreviewDecisionClass {
    /// The action may apply inline with a named undo group.
    InlineApplyAllowed,
    /// A structured diff preview must be opened before apply.
    PreviewRequired,
    /// The preview must be approved before any mutation.
    ApprovalRequired,
    /// The action is blocked because its evidence is tainted and unpromoted.
    BlockedTaintedEvidence,
    /// The action is read-only and has no apply path.
    InspectOnly,
}

impl CodeActionPreviewDecisionClass {
    /// Returns true when direct inline apply is denied.
    pub const fn blocks_direct_apply(self) -> bool {
        !matches!(self, Self::InlineApplyAllowed)
    }
}

/// Input required to project a code action into an editor preview.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodeActionPreviewRequest {
    /// Stable preview id.
    pub preview_id: String,
    /// Trust posture for evidence cited by the action.
    pub evidence_trust_class: QuickFixEvidenceTrustClass,
    /// Tainted-context fence ref, when the proposal cites untrusted text.
    pub tainted_context_ref: Option<String>,
    /// Promotion ref when a user or trusted parser promoted the cited slice.
    pub trusted_promotion_ref: Option<String>,
    /// Capture timestamp.
    pub captured_at: String,
}

/// Editor-facing preview record for one quick fix or code action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeActionPreviewRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub code_action_preview_schema_version: CodeActionPreviewSchemaVersion,
    /// Stable preview id.
    pub preview_id: String,
    /// Source language-platform action id.
    pub code_action_id: String,
    /// Plain-language action label.
    pub action_label: String,
    /// Provider label shown in editor and support surfaces.
    pub provider_label: String,
    /// Trust posture for evidence cited by the action.
    pub evidence_trust_class: QuickFixEvidenceTrustClass,
    /// Tainted-context fence ref, when present.
    pub tainted_context_ref: Option<String>,
    /// Promotion ref when the cited evidence was explicitly promoted.
    pub trusted_promotion_ref: Option<String>,
    /// Preview requirement inherited from the language action.
    pub preview_requirement_class: CodeActionPreviewRequirementClass,
    /// Current apply posture inherited from the language action.
    pub apply_posture_class: CodeActionApplyPostureClass,
    /// Editor decision for direct apply, preview, approval, or inspect-only.
    pub decision_class: CodeActionPreviewDecisionClass,
    /// Reasons direct apply is blocked.
    pub direct_apply_block_reason_refs: Vec<String>,
    /// True when a structured preview must be opened before mutation.
    pub preview_required: bool,
    /// True when explicit approval is required after preview.
    pub approval_required: bool,
    /// True when the action must have a rollback or grouped undo route.
    pub rollback_required: bool,
    /// True when grouped undo is required for the mutation.
    pub grouped_undo_required: bool,
    /// True when the action has an attributable rollback or undo route.
    pub rollback_route_available: bool,
    /// Mutation counts carried without raw paths or patches.
    pub mutation_counts: CodeActionMutationCounts,
    /// Blocking reasons inherited from the language action.
    pub blocking_reason_classes: Vec<CodeActionBlockingReasonClass>,
    /// Undo group reference, when present.
    pub undo_group_ref: Option<String>,
    /// Checkpoint reference, when present.
    pub checkpoint_ref: Option<String>,
    /// Review packet reference, when present.
    pub review_packet_ref: Option<String>,
    /// Validation and replay plan for the action.
    pub validation_plan: CodeActionValidationPlan,
    /// Redaction posture.
    pub redaction_class: RedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Accessible summary for screen readers.
    pub accessibility_summary: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl CodeActionPreviewRecord {
    /// Stable record-kind tag for editor code-action previews.
    pub const RECORD_KIND: &'static str = "editor_code_action_preview_record";

    /// Projects one language code action into the editor preview contract.
    pub fn from_code_action(action: &CodeActionRecord, request: CodeActionPreviewRequest) -> Self {
        let tainted_unpromoted =
            request.evidence_trust_class.is_tainted() && request.trusted_promotion_ref.is_none();
        let preview_required =
            action.preview_required() || request.evidence_trust_class.requires_review();
        let approval_required = tainted_unpromoted
            || action.apply_posture_class.is_blocked()
            || (preview_required && !action.silent_apply_allowed());
        let grouped_undo_required = action.is_mutation_bearing()
            && (action.is_multi_file_or_configuration_changing()
                || action.has_generated_or_protected_impact()
                || action.mutation_counts.affected_file_count > 1);
        let rollback_required = action.is_mutation_bearing()
            && (preview_required
                || grouped_undo_required
                || action.has_generated_or_protected_impact());
        let rollback_route_available = !action.is_mutation_bearing()
            || action.has_named_undo_group()
            || action.checkpoint_ref.is_some();
        let decision_class = if matches!(
            action.apply_posture_class,
            CodeActionApplyPostureClass::NotApplicableReadOnly
        ) {
            CodeActionPreviewDecisionClass::InspectOnly
        } else if tainted_unpromoted {
            CodeActionPreviewDecisionClass::BlockedTaintedEvidence
        } else if approval_required {
            CodeActionPreviewDecisionClass::ApprovalRequired
        } else if preview_required {
            CodeActionPreviewDecisionClass::PreviewRequired
        } else {
            CodeActionPreviewDecisionClass::InlineApplyAllowed
        };
        let direct_apply_block_reason_refs = direct_apply_block_reason_refs(
            action,
            request.evidence_trust_class,
            tainted_unpromoted,
        );
        let undo_group_ref = action
            .undo_group
            .as_ref()
            .map(|group| group.undo_group_id.clone());
        let accessibility_summary = format!(
            "{} from {}; decision {:?}; evidence {}.",
            action.action_label,
            action.acting_provider.provider_display_label,
            decision_class,
            request.evidence_trust_class.as_str()
        );

        Self {
            record_kind: Self::RECORD_KIND.into(),
            code_action_preview_schema_version: CODE_ACTION_PREVIEW_SCHEMA_VERSION,
            preview_id: request.preview_id,
            code_action_id: action.code_action_id.clone(),
            action_label: action.action_label.clone(),
            provider_label: action.acting_provider.provider_display_label.clone(),
            evidence_trust_class: request.evidence_trust_class,
            tainted_context_ref: request.tainted_context_ref,
            trusted_promotion_ref: request.trusted_promotion_ref,
            preview_requirement_class: action.preview_requirement_class,
            apply_posture_class: action.apply_posture_class,
            decision_class,
            direct_apply_block_reason_refs,
            preview_required,
            approval_required,
            rollback_required,
            grouped_undo_required,
            rollback_route_available,
            mutation_counts: action.mutation_counts.clone(),
            blocking_reason_classes: action.blocking_reason_classes.clone(),
            undo_group_ref,
            checkpoint_ref: action.checkpoint_ref.clone(),
            review_packet_ref: action.review_packet_ref.clone(),
            validation_plan: action.validation_plan.clone(),
            redaction_class: RedactionClass::MetadataSafeDefault,
            captured_at: request.captured_at,
            accessibility_summary,
            export_safe_summary: format!(
                "{} editor preview blocks direct apply: {}.",
                action.code_action_id,
                decision_class.blocks_direct_apply()
            ),
        }
    }

    /// Returns true when a broad mutation has the required preview and rollback path.
    pub const fn broad_change_is_previewable_and_rollback_ready(&self) -> bool {
        self.preview_required && self.rollback_required && self.rollback_route_available
    }

    /// Returns true when unpromoted tainted evidence blocks direct apply.
    pub const fn tainted_evidence_blocks_direct_apply(&self) -> bool {
        matches!(
            self.decision_class,
            CodeActionPreviewDecisionClass::BlockedTaintedEvidence
        )
    }
}

fn direct_apply_block_reason_refs(
    action: &CodeActionRecord,
    evidence_trust_class: QuickFixEvidenceTrustClass,
    tainted_unpromoted: bool,
) -> Vec<String> {
    let mut refs = action.admission().refused_silent_apply_reason_refs;
    if evidence_trust_class.requires_review() {
        refs.push(format!("evidence_trust:{}", evidence_trust_class.as_str()));
    }
    if tainted_unpromoted {
        refs.push("tainted_context:promotion_required".into());
    }
    refs.sort();
    refs.dedup();
    refs
}
