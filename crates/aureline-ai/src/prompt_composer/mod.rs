//! Beta prompt-composer conformance records.
//!
//! This module projects the existing composer draft, context snapshot, command
//! registry, route receipts, spend receipts, and evidence packet refs into one
//! export-safe record for the user-facing prompt composer. It does not assemble
//! raw prompt bodies or dispatch model work. The record answers which mode,
//! scope, mentions, attachments, slash commands, budget decisions, draft
//! retention posture, and evidence lineage were visible before the turn could
//! leave the composer.

use std::error::Error;
use std::fmt;

use aureline_commands::{
    CommandEnablementContext, CommandRegistry, DisabledReasonCode, PreflightDecisionClass,
};
use serde::{Deserialize, Serialize};

use crate::composer::beta::ComposerContextEvidenceBetaPacket;
use crate::composer::{AttachmentKind, AttachmentStatusClass, MentionKind, SelectionReasonClass};
use crate::context_inspector::{
    BudgetPressureClass, ComposerAttachmentPill, ComposerContextAlphaSnapshot, ComposerContextItem,
    ComposerMentionPreview, ContextFreshnessClass, ContextItemStateClass,
    ContextOmissionReasonClass, ExecutionBoundaryClass, IntentModeClass, MentionPreviewStateClass,
};
use crate::{SourceClass, TrustPosture};

/// Stable record-kind tag carried by [`PromptComposerConformancePacket`].
pub const PROMPT_COMPOSER_CONFORMANCE_RECORD_KIND: &str = "prompt_composer_conformance_packet";

/// Schema version for prompt-composer conformance packets.
pub const PROMPT_COMPOSER_CONFORMANCE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the prompt-composer draft schema.
pub const PROMPT_COMPOSER_DRAFT_SCHEMA_REF: &str = "schemas/ai/prompt_composer_draft.schema.json";

/// Repo-relative path of the prompt-context attachment schema.
pub const PROMPT_CONTEXT_ATTACHMENT_SCHEMA_REF: &str =
    "schemas/ai/prompt_context_attachment.schema.json";

/// Repo-relative path of the UX contract for the beta prompt composer.
pub const PROMPT_COMPOSER_BETA_UX_DOC_REF: &str = "docs/ux/m3/prompt_composer_beta.md";

/// Repo-relative path of the AI prompt-composer contract.
pub const PROMPT_COMPOSER_AI_DOC_REF: &str = "docs/ai/prompt_composer_contract.md";

/// Repo-relative path of the protected drill fixture directory.
pub const PROMPT_COMPOSER_DRILL_FIXTURE_DIR: &str = "fixtures/ai/m3/prompt_composer_drills";

/// Repo-relative path of the checked conformance export.
pub const PROMPT_COMPOSER_CONFORMANCE_ARTIFACT_REF: &str =
    "artifacts/ai/m3/prompt_composer_conformance/support_export.json";

/// Repo-relative path of the checked conformance Markdown summary.
pub const PROMPT_COMPOSER_CONFORMANCE_SUMMARY_REF: &str =
    "artifacts/ai/m3/prompt_composer_conformance/summary.md";

/// Mention classes the prompt composer resolves before send.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromptMentionKind {
    /// `@file` or equivalent file-slice object.
    File,
    /// `@symbol` or equivalent graph symbol object.
    Symbol,
    /// `@root` or equivalent workspace root/workset object.
    Root,
    /// `@run` or equivalent task, test, terminal, or runtime object.
    Run,
    /// A stable object reference not covered by the shorthand classes.
    ObjectReference,
}

impl PromptMentionKind {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Symbol => "symbol",
            Self::Root => "root",
            Self::Run => "run",
            Self::ObjectReference => "object_reference",
        }
    }
}

impl From<MentionKind> for PromptMentionKind {
    fn from(value: MentionKind) -> Self {
        match value {
            MentionKind::FileMention => Self::File,
            MentionKind::SymbolMention => Self::Symbol,
            MentionKind::RootMention | MentionKind::WorksetMention => Self::Root,
            MentionKind::RunMention | MentionKind::ExecutionContextMention => Self::Run,
            MentionKind::ObjectReferenceMention
            | MentionKind::SearchResultMention
            | MentionKind::DocsAnchorMention
            | MentionKind::DiagnosticMention => Self::ObjectReference,
        }
    }
}

/// Pre-send resolution class for a typed mention.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromptMentionResolutionClass {
    /// The mention resolves to one exact stable object.
    ResolvedExact,
    /// The mention has multiple candidates and cannot bind silently.
    Ambiguous,
    /// The mention did not resolve.
    Unresolved,
    /// The referenced object changed or disappeared since it was attached.
    Stale,
    /// Policy or scope prevents using the resolved object.
    Blocked,
}

impl PromptMentionResolutionClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolvedExact => "resolved_exact",
            Self::Ambiguous => "ambiguous",
            Self::Unresolved => "unresolved",
            Self::Stale => "stale",
            Self::Blocked => "blocked",
        }
    }
}

/// One resolved, ambiguous, stale, or blocked mention row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptMentionResolution {
    /// Mention id from the composer draft.
    pub mention_id: String,
    /// Mention shorthand or object-reference class.
    pub mention_kind: PromptMentionKind,
    /// User-visible mention label.
    pub display_label: String,
    /// Resolution state shown before send.
    pub resolution_class: PromptMentionResolutionClass,
    /// Exact stable target id when resolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_stable_id: Option<String>,
    /// Preview ref for the exact target displayed to the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_target_preview_ref: Option<String>,
    /// Candidate target refs when the mention is ambiguous.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub candidate_target_refs: Vec<String>,
    /// True when the mention prevents send until the user resolves it.
    pub blocks_send: bool,
    /// True when the mention narrows send to a safer read-only or scoped path.
    pub narrows_send: bool,
}

impl PromptMentionResolution {
    /// Projects a prompt-composer mention row from the pre-send context preview.
    pub fn from_preview(preview: &ComposerMentionPreview) -> Self {
        let resolution_class = match preview.preview_state {
            MentionPreviewStateClass::ResolvedExact => PromptMentionResolutionClass::ResolvedExact,
            MentionPreviewStateClass::Ambiguous => PromptMentionResolutionClass::Ambiguous,
            MentionPreviewStateClass::Unresolved => PromptMentionResolutionClass::Unresolved,
            MentionPreviewStateClass::Stale => PromptMentionResolutionClass::Stale,
            MentionPreviewStateClass::Blocked => PromptMentionResolutionClass::Blocked,
        };
        let blocks_send = !matches!(
            resolution_class,
            PromptMentionResolutionClass::ResolvedExact
        );
        let narrows_send = matches!(resolution_class, PromptMentionResolutionClass::Blocked);
        Self {
            mention_id: preview.mention_id.clone(),
            mention_kind: PromptMentionKind::from(preview.kind),
            display_label: preview.display_label.clone(),
            resolution_class,
            target_stable_id: preview.target_stable_id.clone(),
            exact_target_preview_ref: preview
                .target_stable_id
                .as_ref()
                .map(|target| format!("preview:mention-target:{target}")),
            candidate_target_refs: preview.candidate_target_refs.clone(),
            blocks_send,
            narrows_send,
        }
    }
}

/// One typed attachment pill with identity, trust, freshness, and actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptContextAttachment {
    /// Attachment id from the composer draft.
    pub attachment_id: String,
    /// Attachment kind.
    pub kind: AttachmentKind,
    /// Source class for the attached object.
    pub source_class: SourceClass,
    /// Stable object identity that survives display-label changes.
    pub stable_object_ref: String,
    /// Freshness label visible on the pill.
    pub freshness_class: ContextFreshnessClass,
    /// Trust label visible on the pill.
    pub trust_posture: TrustPosture,
    /// Why this attachment is present.
    pub selection_reason: SelectionReasonClass,
    /// Attachment status from the draft.
    pub status: AttachmentStatusClass,
    /// Context state projected by the pre-send inspector.
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
    /// True when policy allows removing the attachment before send.
    pub removable: bool,
}

impl PromptContextAttachment {
    /// Builds a typed prompt-context attachment from a composer attachment pill.
    pub fn from_pill(
        pill: &ComposerAttachmentPill,
        stable_object_ref: impl Into<String>,
        freshness_class: ContextFreshnessClass,
    ) -> Self {
        let attachment_id = pill.attachment_id.clone();
        Self {
            attachment_id: attachment_id.clone(),
            kind: pill.kind,
            source_class: pill.source_class,
            stable_object_ref: stable_object_ref.into(),
            freshness_class,
            trust_posture: pill.trust_posture,
            selection_reason: pill.selection_reason,
            status: pill.status,
            context_state: pill.context_state,
            display_label: pill.display_label.clone(),
            estimated_byte_size: pill.estimated_byte_size,
            preview_action_ref: format!(
                "action:prompt-composer.preview-attachment:{attachment_id}"
            ),
            open_action_ref: format!("action:prompt-composer.open-attachment:{attachment_id}"),
            remove_action_ref: format!("action:prompt-composer.remove-attachment:{attachment_id}"),
            keyboard_reachable: true,
            removable: pill.removable,
        }
    }
}

/// One slash-command binding resolved through the canonical command registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptSlashCommandBinding {
    /// Invocation id from the composer surface.
    pub invocation_id: String,
    /// Stable command id.
    pub command_id: String,
    /// User-visible command label.
    pub display_label: String,
    /// Canonical machine verb shared with palette, CLI, and automation.
    pub canonical_verb: String,
    /// Capability class from the command descriptor.
    pub capability_scope_class: String,
    /// Approval posture from the command descriptor.
    pub approval_posture_class: String,
    /// Preflight decision token.
    pub preflight_decision_token: String,
    /// Disabled reason token when preflight blocks or disables invocation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason_token: Option<String>,
    /// Help anchor ref from the command descriptor.
    pub help_path_ref: String,
    /// Invocation schema ref shared with non-AI command surfaces.
    pub invocation_schema_ref: String,
    /// Result schema ref shared with non-AI command surfaces.
    pub result_schema_ref: String,
}

impl PromptSlashCommandBinding {
    /// Resolves a slash command through the canonical command registry.
    pub fn from_registry_preflight(
        invocation_id: impl Into<String>,
        requested_command_id: impl Into<String>,
        display_label: impl Into<String>,
        registry: &CommandRegistry,
        context: &CommandEnablementContext,
    ) -> Self {
        let invocation_id = invocation_id.into();
        let requested_command_id = requested_command_id.into();
        let display_label = display_label.into();
        let Some(entry) = registry.get(&requested_command_id) else {
            return Self {
                invocation_id,
                command_id: requested_command_id,
                display_label,
                canonical_verb: String::new(),
                capability_scope_class: String::new(),
                approval_posture_class: String::new(),
                preflight_decision_token: "unresolved_no_match".to_owned(),
                disabled_reason_token: Some("command_id_not_found".to_owned()),
                help_path_ref: String::new(),
                invocation_schema_ref: String::new(),
                result_schema_ref: String::new(),
            };
        };

        let contract = entry.public_contract();
        let preflight = entry.preflight(context);
        let docs = &entry.descriptor.docs_help_anchor_ref;
        Self {
            invocation_id,
            command_id: contract.command_id,
            display_label,
            canonical_verb: contract.canonical_verb,
            capability_scope_class: contract.capability_scope_class,
            approval_posture_class: entry.descriptor.approval_posture_class.clone(),
            preflight_decision_token: preflight_decision_token(preflight.decision_class).to_owned(),
            disabled_reason_token: preflight
                .enablement_snapshot
                .disabled_reason_code
                .map(disabled_reason_token),
            help_path_ref: format!(
                "docs-help:{}:{}:{}",
                docs.pack_id, docs.anchor_id, docs.anchor_kind
            ),
            invocation_schema_ref: contract.invocation_schema_ref,
            result_schema_ref: contract.result_schema_ref,
        }
    }
}

/// Budget handling for one source under prompt pressure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromptBudgetActionClass {
    /// The source is included as raw context.
    IncludeRaw,
    /// The source is pinned and cannot be silently dropped.
    PinKeep,
    /// The source is omitted with a typed reason.
    Omit,
    /// The source is summarized with a typed reason.
    Summarize,
    /// The source is trimmed to a bounded representation.
    Trim,
    /// The route changed to preserve budget or policy truth.
    RouteSwitch,
    /// The source or route blocks send.
    Block,
}

impl PromptBudgetActionClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncludeRaw => "include_raw",
            Self::PinKeep => "pin_keep",
            Self::Omit => "omit",
            Self::Summarize => "summarize",
            Self::Trim => "trim",
            Self::RouteSwitch => "route_switch",
            Self::Block => "block",
        }
    }
}

/// One budget decision row shown before send.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptBudgetDecisionRow {
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
}

/// Non-AI continuation path when route, budget, or policy closes AI dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromptComposerSafeFallbackClass {
    /// Manual editing and ordinary search remain available.
    ManualEditAndSearch,
    /// Local-only or local-model reasoning remains available.
    LocalOnlyReview,
    /// CLI or headless non-AI command path remains available.
    CliHeadlessPath,
    /// Deferred review/export path remains available.
    DeferredReviewExport,
    /// Invalid state: the user is left with no non-AI continuation.
    AiOnlyDeadEnd,
}

impl PromptComposerSafeFallbackClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManualEditAndSearch => "manual_edit_and_search",
            Self::LocalOnlyReview => "local_only_review",
            Self::CliHeadlessPath => "cli_headless_path",
            Self::DeferredReviewExport => "deferred_review_export",
            Self::AiOnlyDeadEnd => "ai_only_dead_end",
        }
    }
}

/// Budget strip shown before send.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptBudgetStrip {
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
    pub decision_rows: Vec<PromptBudgetDecisionRow>,
    /// Safe non-AI fallback path.
    pub safe_fallback_class: PromptComposerSafeFallbackClass,
    /// Review-safe explanation label.
    pub explanation_label: String,
}

/// Draft retention and sync visibility shown in the composer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DraftRetentionScopeClass {
    /// Draft is local to the current workspace/device.
    LocalOnly,
    /// Draft is locally encrypted or protected by the device profile.
    LocalProtected,
    /// Managed policy mirrors or extends retention.
    ManagedPolicyReplicated,
    /// Collaboration mode shares the draft or metadata with participants.
    CollaborationShared,
    /// Policy prevents durable draft retention.
    DurableRetentionDisabled,
}

impl DraftRetentionScopeClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::LocalProtected => "local_protected",
            Self::ManagedPolicyReplicated => "managed_policy_replicated",
            Self::CollaborationShared => "collaboration_shared",
            Self::DurableRetentionDisabled => "durable_retention_disabled",
        }
    }
}

/// Draft persistence row visible before send.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptDraftPersistence {
    /// Retention/sync scope.
    pub retention_scope_class: DraftRetentionScopeClass,
    /// True when draft state is saved locally first.
    pub local_first: bool,
    /// Policy epoch that shaped retention.
    pub policy_epoch_ref: String,
    /// Collaboration session ref when sharing changes retention.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collaboration_session_ref: Option<String>,
    /// Visible note explaining retention or sync behavior.
    pub visibility_note: String,
    /// Clear/delete action ref.
    pub clear_action_ref: String,
}

/// Text-entry and send semantics that protect multiline and IME editing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptInputSemantics {
    /// True when multiline entry is preserved.
    pub multiline_entry_preserved: bool,
    /// True when IME composition suppresses accidental send.
    pub ime_composition_blocks_send: bool,
    /// True when send requires explicit action outside composition.
    pub send_requires_explicit_action: bool,
    /// True when the draft survives route, policy, and offline changes.
    pub draft_survives_degraded_routes: bool,
}

/// Failure or degraded state the composer must handle without losing the draft.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromptComposerEdgeCaseClass {
    /// A referenced attachment is stale.
    StaleAttachment,
    /// A mention is unresolved or ambiguous.
    UnresolvedMention,
    /// The composition exceeds the disclosed budget.
    OverBudgetComposition,
    /// The selected provider, tool, or route is policy-blocked.
    PolicyBlockedRoute,
    /// Network/provider degradation leaves only local-safe paths.
    OfflineLocalOnlyDegradation,
}

impl PromptComposerEdgeCaseClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaleAttachment => "stale_attachment",
            Self::UnresolvedMention => "unresolved_mention",
            Self::OverBudgetComposition => "over_budget_composition",
            Self::PolicyBlockedRoute => "policy_blocked_route",
            Self::OfflineLocalOnlyDegradation => "offline_local_only_degradation",
        }
    }
}

/// Edge-case handling row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptComposerEdgeCaseRow {
    /// Edge case class.
    pub edge_case_class: PromptComposerEdgeCaseClass,
    /// Source mention, attachment, route, or budget ref.
    pub source_ref: String,
    /// True when the current draft remains intact.
    pub preserves_current_draft: bool,
    /// Fallback path shown to the user.
    pub safe_fallback_class: PromptComposerSafeFallbackClass,
    /// Review-safe explanation.
    pub explanation_label: String,
}

/// Evidence packet class preserved by the conformance packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromptEvidencePacketClass {
    /// Inline evidence stub in the current client.
    InlineStub,
    /// Operator handoff packet.
    OperatorPacket,
    /// Support packet with redaction manifest.
    SupportPacket,
    /// Compliance or audit packet.
    ComplianceAuditPacket,
}

impl PromptEvidencePacketClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlineStub => "inline_stub",
            Self::OperatorPacket => "operator_packet",
            Self::SupportPacket => "support_packet",
            Self::ComplianceAuditPacket => "compliance_audit_packet",
        }
    }
}

/// Evidence, route, spend, redaction, and replay lineage for one composed turn.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptEvidenceLineage {
    /// Stable evidence id.
    pub evidence_id: String,
    /// Composer session ref.
    pub composer_session_ref: String,
    /// Turn draft ref.
    pub turn_draft_ref: String,
    /// Context snapshot ref.
    pub composer_context_snapshot_ref: String,
    /// Preserved packet classes.
    pub packet_classes: Vec<PromptEvidencePacketClass>,
    /// Route receipt ref.
    pub route_receipt_ref: String,
    /// Spend receipt ref.
    pub spend_receipt_ref: String,
    /// Redaction manifest ref.
    pub redaction_manifest_ref: String,
    /// Replay lineage ref.
    pub replay_lineage_ref: String,
    /// Operator packet ref.
    pub operator_packet_ref: String,
    /// Support packet ref.
    pub support_packet_ref: String,
    /// Compliance packet ref.
    pub compliance_packet_ref: String,
}

/// Preview-only branch or worktree row that must not widen into apply behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewBranchComposerRow {
    /// Stable preview row ref.
    pub preview_row_ref: String,
    /// Branch or worktree ref.
    pub branch_or_worktree_ref: String,
    /// True when the row is preview-only.
    pub preview_only: bool,
    /// Must be false for preview rows.
    pub autonomous_apply_enabled: bool,
    /// Cumulative budget posture ref for the preview job.
    pub cumulative_budget_posture_ref: String,
    /// Route receipts shown on the preview row.
    pub route_receipt_refs: Vec<String>,
}

/// Intent mode row shown at the top of the composer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptIntentRow {
    /// Explicit intent mode.
    pub mode_class: IntentModeClass,
    /// Current scope label.
    pub current_scope_label: String,
    /// Current execution boundary.
    pub execution_boundary_class: ExecutionBoundaryClass,
    /// Optional action or command identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_identity_ref: Option<String>,
}

/// Constructor input for [`PromptComposerConformancePacket::from_context_snapshot`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptComposerConformanceInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Display label.
    pub display_label: String,
    /// Draft persistence row.
    pub draft_persistence: PromptDraftPersistence,
    /// Input semantics row.
    pub input_semantics: PromptInputSemantics,
    /// Slash-command rows already resolved through the command registry.
    pub slash_command_rows: Vec<PromptSlashCommandBinding>,
    /// Extra budget decisions not derivable from context rows.
    pub budget_decisions: Vec<PromptBudgetDecisionRow>,
    /// Edge-case rows required by conformance drills.
    pub edge_case_rows: Vec<PromptComposerEdgeCaseRow>,
    /// Evidence lineage.
    pub evidence_lineage: PromptEvidenceLineage,
    /// Preview branch/worktree rows.
    pub preview_branch_rows: Vec<PreviewBranchComposerRow>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// JSON export ref.
    pub json_export_ref: String,
    /// Markdown summary ref.
    pub markdown_summary_ref: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe prompt-composer conformance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptComposerConformancePacket {
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
    /// Composer draft id.
    pub composer_draft_id: String,
    /// Composer session id.
    pub composer_session_id: String,
    /// Request workspace ref.
    pub request_workspace_ref: String,
    /// Context snapshot ref.
    pub composer_context_snapshot_ref: String,
    /// Intent row shown before send.
    pub intent_row: PromptIntentRow,
    /// Mention resolution rows.
    pub mention_rows: Vec<PromptMentionResolution>,
    /// Typed attachment rows.
    pub attachment_rows: Vec<PromptContextAttachment>,
    /// Slash-command rows resolved through the canonical command registry.
    pub slash_command_rows: Vec<PromptSlashCommandBinding>,
    /// Budget strip and decisions.
    pub budget_strip: PromptBudgetStrip,
    /// Draft persistence row.
    pub draft_persistence: PromptDraftPersistence,
    /// Text-entry semantics row.
    pub input_semantics: PromptInputSemantics,
    /// Edge-case drill rows.
    pub edge_case_rows: Vec<PromptComposerEdgeCaseRow>,
    /// Evidence/route/spend/redaction lineage.
    pub evidence_lineage: PromptEvidenceLineage,
    /// Preview-only branch/worktree rows.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preview_branch_rows: Vec<PreviewBranchComposerRow>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// JSON export ref.
    pub json_export_ref: String,
    /// Markdown summary ref.
    pub markdown_summary_ref: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl PromptComposerConformancePacket {
    /// Builds a conformance packet from the canonical pre-send snapshot.
    pub fn from_context_snapshot(
        snapshot: &ComposerContextAlphaSnapshot,
        beta_evidence: &ComposerContextEvidenceBetaPacket,
        input: PromptComposerConformanceInput,
    ) -> Self {
        let mention_rows = snapshot
            .mention_previews
            .iter()
            .map(PromptMentionResolution::from_preview)
            .collect();
        let attachment_rows = snapshot
            .attachment_pills
            .iter()
            .map(|pill| {
                let context_item = context_item_for_attachment(snapshot, &pill.attachment_id);
                let stable_object_ref = context_item
                    .map(|item| item.stable_identity_ref.clone())
                    .unwrap_or_else(|| format!("composer-attachment:{}", pill.attachment_id));
                let freshness_class = context_item
                    .map(|item| item.freshness_class)
                    .unwrap_or_else(|| freshness_from_status(pill.status));
                PromptContextAttachment::from_pill(pill, stable_object_ref, freshness_class)
            })
            .collect();

        let budget_strip = PromptBudgetStrip {
            aggregate_byte_estimate: snapshot.budget_strip.aggregate_byte_estimate,
            budget_byte_ceiling: snapshot.budget_strip.budget_byte_ceiling,
            pressure_class: snapshot.budget_strip.pressure_class,
            included_context_group_tokens: snapshot
                .budget_strip
                .included_context_group_tokens
                .clone(),
            omitted_or_trimmed_group_tokens: snapshot
                .budget_strip
                .omitted_or_trimmed_group_tokens
                .clone(),
            decision_rows: budget_decisions(snapshot, beta_evidence, input.budget_decisions),
            safe_fallback_class: PromptComposerSafeFallbackClass::ManualEditAndSearch,
            explanation_label: budget_explanation(snapshot.budget_strip.pressure_class),
        };

        Self {
            record_kind: PROMPT_COMPOSER_CONFORMANCE_RECORD_KIND.to_owned(),
            schema_version: PROMPT_COMPOSER_CONFORMANCE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            display_label: input.display_label,
            composer_draft_id: snapshot.composer_draft_id.clone(),
            composer_session_id: snapshot.composer_session_id.clone(),
            request_workspace_ref: snapshot.request_workspace_id.clone(),
            composer_context_snapshot_ref: snapshot.review_lock.context_snapshot_ref.clone(),
            intent_row: PromptIntentRow {
                mode_class: snapshot.intent_mode,
                current_scope_label: snapshot.scope_label.clone(),
                execution_boundary_class: snapshot.execution_boundary_class,
                action_identity_ref: snapshot.action_identity_ref.clone(),
            },
            mention_rows,
            attachment_rows,
            slash_command_rows: input.slash_command_rows,
            budget_strip,
            draft_persistence: input.draft_persistence,
            input_semantics: input.input_semantics,
            edge_case_rows: input.edge_case_rows,
            evidence_lineage: input.evidence_lineage,
            preview_branch_rows: input.preview_branch_rows,
            source_contract_refs: input.source_contract_refs,
            json_export_ref: input.json_export_ref,
            markdown_summary_ref: input.markdown_summary_ref,
            minted_at: input.minted_at,
        }
    }

    /// Validates the prompt-composer conformance packet.
    pub fn validate(&self) -> Vec<PromptComposerConformanceViolation> {
        let mut violations = Vec::new();
        if self.record_kind != PROMPT_COMPOSER_CONFORMANCE_RECORD_KIND {
            violations.push(PromptComposerConformanceViolation::WrongRecordKind);
        }
        if self.schema_version != PROMPT_COMPOSER_CONFORMANCE_SCHEMA_VERSION {
            violations.push(PromptComposerConformanceViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.composer_draft_id.trim().is_empty()
            || self.composer_session_id.trim().is_empty()
            || self.request_workspace_ref.trim().is_empty()
            || self.composer_context_snapshot_ref.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(PromptComposerConformanceViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_intent(self, &mut violations);
        validate_mentions(self, &mut violations);
        validate_attachments(self, &mut violations);
        validate_slash_commands(self, &mut violations);
        validate_budget(self, &mut violations);
        validate_draft_persistence(self, &mut violations);
        validate_input_semantics(self, &mut violations);
        validate_edge_cases(self, &mut violations);
        validate_evidence_lineage(self, &mut violations);
        validate_preview_branch_rows(self, &mut violations);
        if self.json_export_ref.trim().is_empty() || self.markdown_summary_ref.trim().is_empty() {
            violations.push(PromptComposerConformanceViolation::ExportRefsMissing);
        }
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("prompt composer conformance packet serializes"),
        ) {
            violations.push(PromptComposerConformanceViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("prompt composer conformance packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Prompt Composer Conformance\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Draft: `{}` / session `{}`\n",
            self.composer_draft_id, self.composer_session_id
        ));
        out.push_str(&format!(
            "- Mode/scope: `{}` / `{}`\n",
            self.intent_row.mode_class.as_str(),
            self.intent_row.current_scope_label
        ));
        out.push_str(&format!(
            "- Mentions / attachments / slash commands: {} / {} / {}\n",
            self.mention_rows.len(),
            self.attachment_rows.len(),
            self.slash_command_rows.len()
        ));
        out.push_str(&format!(
            "- Budget: `{}` ({} decisions)\n",
            self.budget_strip.pressure_class.as_str(),
            self.budget_strip.decision_rows.len()
        ));
        out.push_str(&format!(
            "- Evidence: `{}` / route `{}` / spend `{}`\n",
            self.evidence_lineage.evidence_id,
            self.evidence_lineage.route_receipt_ref,
            self.evidence_lineage.spend_receipt_ref
        ));
        out.push_str(&format!(
            "- Edge cases covered: {}\n",
            self.edge_case_rows.len()
        ));
        out
    }
}

/// Errors emitted when reading the checked-in prompt-composer export.
#[derive(Debug)]
pub enum PromptComposerConformanceArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PromptComposerConformanceViolation>),
}

impl fmt::Display for PromptComposerConformanceArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "prompt composer export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "prompt composer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for PromptComposerConformanceArtifactError {}

/// Validation failures emitted by [`PromptComposerConformancePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PromptComposerConformanceViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// Intent mode, scope, or boundary row is incomplete.
    IntentRowIncomplete,
    /// Mention row could silently bind or lacks required candidates.
    MentionResolutionUnsafe,
    /// Attachment row lacks stable identity, source, freshness, or trust disclosure.
    AttachmentDisclosureIncomplete,
    /// Attachment row lacks required preview, open, remove, or keyboard action.
    AttachmentActionMissing,
    /// Slash-command row does not preserve command-graph identity.
    SlashCommandParityMissing,
    /// Disabled slash-command row lacks a disabled reason.
    DisabledReasonMissing,
    /// Budget overflow lacks omission, summary, or route-switch explanation.
    BudgetOverflowWithoutExplanation,
    /// Draft retention/sync behavior is not visible.
    DraftRetentionNotVisible,
    /// Multiline, IME, or send semantics are unsafe.
    InputSemanticsUnsafe,
    /// Required degraded/edge-case class is not represented.
    EdgeCaseCoverageMissing,
    /// Edge-case handling would lose the current draft.
    EdgeCaseDropsDraft,
    /// Evidence lineage lacks core refs.
    EvidenceLineageIncomplete,
    /// Required evidence packet class is missing.
    EvidencePacketClassMissing,
    /// Preview-only branch/worktree row widened into autonomous apply behavior.
    PreviewBranchWidened,
    /// Export refs are missing.
    ExportRefsMissing,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl PromptComposerConformanceViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::IntentRowIncomplete => "intent_row_incomplete",
            Self::MentionResolutionUnsafe => "mention_resolution_unsafe",
            Self::AttachmentDisclosureIncomplete => "attachment_disclosure_incomplete",
            Self::AttachmentActionMissing => "attachment_action_missing",
            Self::SlashCommandParityMissing => "slash_command_parity_missing",
            Self::DisabledReasonMissing => "disabled_reason_missing",
            Self::BudgetOverflowWithoutExplanation => "budget_overflow_without_explanation",
            Self::DraftRetentionNotVisible => "draft_retention_not_visible",
            Self::InputSemanticsUnsafe => "input_semantics_unsafe",
            Self::EdgeCaseCoverageMissing => "edge_case_coverage_missing",
            Self::EdgeCaseDropsDraft => "edge_case_drops_draft",
            Self::EvidenceLineageIncomplete => "evidence_lineage_incomplete",
            Self::EvidencePacketClassMissing => "evidence_packet_class_missing",
            Self::PreviewBranchWidened => "preview_branch_widened",
            Self::ExportRefsMissing => "export_refs_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Returns the checked-in prompt-composer conformance export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or validate.
pub fn current_beta_prompt_composer_conformance_export(
) -> Result<PromptComposerConformancePacket, PromptComposerConformanceArtifactError> {
    let packet: PromptComposerConformancePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m3/prompt_composer_conformance/support_export.json"
    )))
    .map_err(PromptComposerConformanceArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PromptComposerConformanceArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &PromptComposerConformancePacket,
    violations: &mut Vec<PromptComposerConformanceViolation>,
) {
    for required in [
        PROMPT_COMPOSER_AI_DOC_REF,
        PROMPT_COMPOSER_BETA_UX_DOC_REF,
        PROMPT_COMPOSER_DRAFT_SCHEMA_REF,
        PROMPT_CONTEXT_ATTACHMENT_SCHEMA_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(PromptComposerConformanceViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_intent(
    packet: &PromptComposerConformancePacket,
    violations: &mut Vec<PromptComposerConformanceViolation>,
) {
    if packet.intent_row.current_scope_label.trim().is_empty() {
        violations.push(PromptComposerConformanceViolation::IntentRowIncomplete);
    }
}

fn validate_mentions(
    packet: &PromptComposerConformancePacket,
    violations: &mut Vec<PromptComposerConformanceViolation>,
) {
    for mention in &packet.mention_rows {
        match mention.resolution_class {
            PromptMentionResolutionClass::ResolvedExact => {
                if mention
                    .target_stable_id
                    .as_deref()
                    .map_or(true, str::is_empty)
                    || mention
                        .exact_target_preview_ref
                        .as_deref()
                        .map_or(true, str::is_empty)
                    || mention.blocks_send
                {
                    violations.push(PromptComposerConformanceViolation::MentionResolutionUnsafe);
                    break;
                }
            }
            PromptMentionResolutionClass::Ambiguous => {
                if mention.candidate_target_refs.len() < 2 || !mention.blocks_send {
                    violations.push(PromptComposerConformanceViolation::MentionResolutionUnsafe);
                    break;
                }
            }
            PromptMentionResolutionClass::Unresolved
            | PromptMentionResolutionClass::Stale
            | PromptMentionResolutionClass::Blocked => {
                if !mention.blocks_send && !mention.narrows_send {
                    violations.push(PromptComposerConformanceViolation::MentionResolutionUnsafe);
                    break;
                }
            }
        }
    }
}

fn validate_attachments(
    packet: &PromptComposerConformancePacket,
    violations: &mut Vec<PromptComposerConformanceViolation>,
) {
    for attachment in &packet.attachment_rows {
        if attachment.attachment_id.trim().is_empty()
            || attachment.stable_object_ref.trim().is_empty()
            || attachment.display_label.trim().is_empty()
        {
            violations.push(PromptComposerConformanceViolation::AttachmentDisclosureIncomplete);
            break;
        }
        if attachment.preview_action_ref.trim().is_empty()
            || attachment.open_action_ref.trim().is_empty()
            || attachment.remove_action_ref.trim().is_empty()
            || !attachment.keyboard_reachable
            || !attachment.removable
        {
            violations.push(PromptComposerConformanceViolation::AttachmentActionMissing);
            break;
        }
    }
}

fn validate_slash_commands(
    packet: &PromptComposerConformancePacket,
    violations: &mut Vec<PromptComposerConformanceViolation>,
) {
    for command in &packet.slash_command_rows {
        if command.command_id.trim().is_empty()
            || command.canonical_verb.trim().is_empty()
            || command.capability_scope_class.trim().is_empty()
            || command.approval_posture_class.trim().is_empty()
            || command.help_path_ref.trim().is_empty()
            || command.invocation_schema_ref.trim().is_empty()
            || command.result_schema_ref.trim().is_empty()
        {
            violations.push(PromptComposerConformanceViolation::SlashCommandParityMissing);
            break;
        }
        if command.preflight_decision_token != "allowed"
            && command
                .disabled_reason_token
                .as_deref()
                .unwrap_or("")
                .is_empty()
        {
            violations.push(PromptComposerConformanceViolation::DisabledReasonMissing);
            break;
        }
    }
}

fn validate_budget(
    packet: &PromptComposerConformancePacket,
    violations: &mut Vec<PromptComposerConformanceViolation>,
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
        violations.push(PromptComposerConformanceViolation::BudgetOverflowWithoutExplanation);
    }
}

fn validate_draft_persistence(
    packet: &PromptComposerConformancePacket,
    violations: &mut Vec<PromptComposerConformanceViolation>,
) {
    let row = &packet.draft_persistence;
    let managed_or_shared = matches!(
        row.retention_scope_class,
        DraftRetentionScopeClass::ManagedPolicyReplicated
            | DraftRetentionScopeClass::CollaborationShared
    );
    if !row.local_first
        || row.policy_epoch_ref.trim().is_empty()
        || row.clear_action_ref.trim().is_empty()
        || row.visibility_note.trim().is_empty()
        || (managed_or_shared && row.visibility_note.len() < 12)
    {
        violations.push(PromptComposerConformanceViolation::DraftRetentionNotVisible);
    }
}

fn validate_input_semantics(
    packet: &PromptComposerConformancePacket,
    violations: &mut Vec<PromptComposerConformanceViolation>,
) {
    let row = &packet.input_semantics;
    if !row.multiline_entry_preserved
        || !row.ime_composition_blocks_send
        || !row.send_requires_explicit_action
        || !row.draft_survives_degraded_routes
    {
        violations.push(PromptComposerConformanceViolation::InputSemanticsUnsafe);
    }
}

fn validate_edge_cases(
    packet: &PromptComposerConformancePacket,
    violations: &mut Vec<PromptComposerConformanceViolation>,
) {
    for required in required_edge_cases() {
        if !packet
            .edge_case_rows
            .iter()
            .any(|row| row.edge_case_class == required)
        {
            violations.push(PromptComposerConformanceViolation::EdgeCaseCoverageMissing);
            return;
        }
    }
    for row in &packet.edge_case_rows {
        if !row.preserves_current_draft
            || row.safe_fallback_class == PromptComposerSafeFallbackClass::AiOnlyDeadEnd
            || row.explanation_label.trim().is_empty()
        {
            violations.push(PromptComposerConformanceViolation::EdgeCaseDropsDraft);
            return;
        }
    }
}

fn validate_evidence_lineage(
    packet: &PromptComposerConformancePacket,
    violations: &mut Vec<PromptComposerConformanceViolation>,
) {
    let lineage = &packet.evidence_lineage;
    if lineage.evidence_id.trim().is_empty()
        || lineage.composer_session_ref != packet.composer_session_id
        || lineage.turn_draft_ref != packet.composer_draft_id
        || lineage.composer_context_snapshot_ref != packet.composer_context_snapshot_ref
        || lineage.route_receipt_ref.trim().is_empty()
        || lineage.spend_receipt_ref.trim().is_empty()
        || lineage.redaction_manifest_ref.trim().is_empty()
        || lineage.replay_lineage_ref.trim().is_empty()
        || lineage.operator_packet_ref.trim().is_empty()
        || lineage.support_packet_ref.trim().is_empty()
        || lineage.compliance_packet_ref.trim().is_empty()
    {
        violations.push(PromptComposerConformanceViolation::EvidenceLineageIncomplete);
        return;
    }
    for required in [
        PromptEvidencePacketClass::InlineStub,
        PromptEvidencePacketClass::OperatorPacket,
        PromptEvidencePacketClass::SupportPacket,
        PromptEvidencePacketClass::ComplianceAuditPacket,
    ] {
        if !lineage.packet_classes.contains(&required) {
            violations.push(PromptComposerConformanceViolation::EvidencePacketClassMissing);
            return;
        }
    }
}

fn validate_preview_branch_rows(
    packet: &PromptComposerConformancePacket,
    violations: &mut Vec<PromptComposerConformanceViolation>,
) {
    for row in &packet.preview_branch_rows {
        if row.preview_row_ref.trim().is_empty()
            || row.branch_or_worktree_ref.trim().is_empty()
            || !row.preview_only
            || row.autonomous_apply_enabled
            || row.cumulative_budget_posture_ref.trim().is_empty()
            || row.route_receipt_refs.is_empty()
            || row
                .route_receipt_refs
                .iter()
                .any(|reference| reference.trim().is_empty())
        {
            violations.push(PromptComposerConformanceViolation::PreviewBranchWidened);
            return;
        }
    }
}

fn context_item_for_attachment<'a>(
    snapshot: &'a ComposerContextAlphaSnapshot,
    attachment_id: &str,
) -> Option<&'a ComposerContextItem> {
    snapshot
        .context_items
        .iter()
        .find(|item| item.source_attachment_ref.as_deref() == Some(attachment_id))
}

fn freshness_from_status(status: AttachmentStatusClass) -> ContextFreshnessClass {
    match status {
        AttachmentStatusClass::Live | AttachmentStatusClass::TaintedOutsideFencedSection => {
            ContextFreshnessClass::AuthoritativeLive
        }
        AttachmentStatusClass::Stale => ContextFreshnessClass::Stale,
        AttachmentStatusClass::OverBudget
        | AttachmentStatusClass::PolicyBlocked
        | AttachmentStatusClass::OutOfScope => ContextFreshnessClass::Unverified,
    }
}

fn budget_decisions(
    snapshot: &ComposerContextAlphaSnapshot,
    beta_evidence: &ComposerContextEvidenceBetaPacket,
    mut extra: Vec<PromptBudgetDecisionRow>,
) -> Vec<PromptBudgetDecisionRow> {
    let mut decisions = Vec::new();
    for item in &snapshot.context_items {
        let action_class = budget_action_for_context_state(item.state_class);
        if action_class == PromptBudgetActionClass::IncludeRaw {
            continue;
        }
        decisions.push(PromptBudgetDecisionRow {
            decision_id: format!("budget-decision:{}", item.context_item_id),
            source_ref: item.stable_identity_ref.clone(),
            source_class: item.source_class,
            context_state: item.state_class,
            action_class,
            reason_token: item
                .omission_reason_class
                .or_else(|| default_reason_for_state(item.state_class))
                .map(|reason| reason.as_str().to_owned()),
            estimated_byte_size: item.estimated_byte_size,
            route_receipt_ref: None,
        });
    }
    if snapshot.budget_strip.pressure_class == BudgetPressureClass::Overflow
        && !decisions
            .iter()
            .any(|row| row.action_class == PromptBudgetActionClass::RouteSwitch)
    {
        decisions.push(PromptBudgetDecisionRow {
            decision_id: "budget-decision:route-pressure".to_owned(),
            source_ref: beta_evidence.routing_packet_ref.clone(),
            source_class: SourceClass::WorkspaceSearchResult,
            context_state: ContextItemStateClass::Omitted,
            action_class: PromptBudgetActionClass::RouteSwitch,
            reason_token: Some("budget_or_route_pressure".to_owned()),
            estimated_byte_size: 0,
            route_receipt_ref: Some(beta_evidence.route_receipt_ref.clone()),
        });
    }
    decisions.append(&mut extra);
    decisions
}

fn budget_action_for_context_state(state: ContextItemStateClass) -> PromptBudgetActionClass {
    match state {
        ContextItemStateClass::Included => PromptBudgetActionClass::IncludeRaw,
        ContextItemStateClass::Pinned => PromptBudgetActionClass::PinKeep,
        ContextItemStateClass::Omitted => PromptBudgetActionClass::Omit,
        ContextItemStateClass::Blocked => PromptBudgetActionClass::Block,
        ContextItemStateClass::Stale => PromptBudgetActionClass::Block,
        ContextItemStateClass::Tainted => PromptBudgetActionClass::Trim,
        ContextItemStateClass::Summarized => PromptBudgetActionClass::Summarize,
        ContextItemStateClass::Trimmed => PromptBudgetActionClass::Trim,
        ContextItemStateClass::NotRequested => PromptBudgetActionClass::Omit,
    }
}

fn default_reason_for_state(state: ContextItemStateClass) -> Option<ContextOmissionReasonClass> {
    match state {
        ContextItemStateClass::Blocked => Some(ContextOmissionReasonClass::Blocked),
        ContextItemStateClass::Stale => Some(ContextOmissionReasonClass::Stale),
        ContextItemStateClass::Tainted => Some(ContextOmissionReasonClass::Tainted),
        ContextItemStateClass::Summarized | ContextItemStateClass::Trimmed => {
            Some(ContextOmissionReasonClass::Budget)
        }
        _ => None,
    }
}

fn budget_explanation(pressure: BudgetPressureClass) -> String {
    match pressure {
        BudgetPressureClass::WithinBudget => {
            "All selected context fits the disclosed route budget.".to_owned()
        }
        BudgetPressureClass::Warning => {
            "The turn is near budget; pinned context remains visible before send.".to_owned()
        }
        BudgetPressureClass::Overflow => {
            "Budget pressure is visible through omitted, trimmed, summarized, and route rows."
                .to_owned()
        }
    }
}

fn preflight_decision_token(value: PreflightDecisionClass) -> &'static str {
    match value {
        PreflightDecisionClass::Allowed => "allowed",
        PreflightDecisionClass::BlockedByPolicy => "blocked_by_policy",
        PreflightDecisionClass::DisabledWithReason => "disabled_with_reason",
        PreflightDecisionClass::PreviewRequired => "preview_required",
        PreflightDecisionClass::ApprovalRequired => "approval_required",
    }
}

fn disabled_reason_token(value: DisabledReasonCode) -> String {
    value.as_str().to_owned()
}

fn required_edge_cases() -> [PromptComposerEdgeCaseClass; 5] {
    [
        PromptComposerEdgeCaseClass::StaleAttachment,
        PromptComposerEdgeCaseClass::UnresolvedMention,
        PromptComposerEdgeCaseClass::OverBudgetComposition,
        PromptComposerEdgeCaseClass::PolicyBlockedRoute,
        PromptComposerEdgeCaseClass::OfflineLocalOnlyDegradation,
    ]
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
