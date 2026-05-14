//! Alpha prompt-composer and AI context-inspector contract.
//!
//! This module owns the pre-send context snapshot for the bounded AI input
//! lane. It consumes the existing [`crate::composer::ComposerDraft`] and
//! [`crate::routing::AiRoutingPacket`] records, then projects the exact
//! intent mode, typed attachment pills, mention previews, grouped context,
//! budget pressure, route posture, and evidence handoff rows that downstream
//! review and support surfaces read.
//!
//! The snapshot stores metadata, opaque refs, enum tokens, and short labels
//! only. It carries no raw prompt body beyond the user draft text already held
//! by [`crate::composer::ComposerDraft`], no raw file contents, no raw provider
//! payloads, no exact cost amounts, no credential material, and no raw URLs.

use serde::{Deserialize, Serialize};

use crate::composer::{
    AttachmentKind, AttachmentStatusClass, ComposerDraft, MentionKind, MentionResolutionState,
    SelectionReasonClass, SourceClass, TrustPosture,
};
use crate::routing::AiRoutingPacket;

/// Stable record-kind tag carried on serialized [`ComposerContextAlphaSnapshot`] payloads.
pub const COMPOSER_CONTEXT_ALPHA_RECORD_KIND: &str = "ai_composer_context_alpha_snapshot_record";

/// Stable record-kind tag carried on serialized [`AiContextEvidenceHandoff`] payloads.
pub const AI_CONTEXT_EVIDENCE_HANDOFF_RECORD_KIND: &str =
    "ai_context_evidence_handoff_alpha_record";

/// Schema version shared by the alpha context snapshot and evidence handoff.
pub const COMPOSER_CONTEXT_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Intent mode selected by the user before sending an AI turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntentModeClass {
    /// Ask an explanatory or exploratory question.
    Ask,
    /// Explain the selected object or context.
    Explain,
    /// Produce a plan without mutating the workspace.
    Plan,
    /// Draft a patch for review.
    DraftPatch,
    /// Review the current diff or review packet.
    ReviewDiff,
    /// Generate test changes for review.
    GenerateTests,
    /// Invoke a governed tool flow that requires approval.
    RunToolWithApproval,
}

impl IntentModeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ask => "ask",
            Self::Explain => "explain",
            Self::Plan => "plan",
            Self::DraftPatch => "draft_patch",
            Self::ReviewDiff => "review_diff",
            Self::GenerateTests => "generate_tests",
            Self::RunToolWithApproval => "run_tool_with_approval",
        }
    }
}

/// Execution boundary disclosed on the composer header.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionBoundaryClass {
    /// Request stays on the local device.
    LocalOnly,
    /// Request uses a first-party managed hosted route.
    ManagedHosted,
    /// Request is brokered through an enterprise gateway.
    EnterpriseGateway,
    /// Request uses an organization or user connected provider.
    ConnectedProvider,
    /// Request cannot dispatch from the current state.
    DispatchBlocked,
}

impl ExecutionBoundaryClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::ManagedHosted => "managed_hosted",
            Self::EnterpriseGateway => "enterprise_gateway",
            Self::ConnectedProvider => "connected_provider",
            Self::DispatchBlocked => "dispatch_blocked",
        }
    }
}

/// Budget pressure class shown before send.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BudgetPressureClass {
    /// The request is under its disclosed budget.
    WithinBudget,
    /// The request is nearing its disclosed budget.
    Warning,
    /// The request exceeds its disclosed budget and must omit, summarize, or re-route.
    Overflow,
}

impl BudgetPressureClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WithinBudget => "within_budget",
            Self::Warning => "warning",
            Self::Overflow => "overflow",
        }
    }
}

/// Group a context item belongs to in the inspector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextGroupClass {
    /// Open files or selected ranges.
    OpenFiles,
    /// Symbols or graph entities.
    SymbolsGraphEntities,
    /// Diagnostics and tests.
    DiagnosticsTests,
    /// Documentation and knowledge sources.
    DocsKnowledgeSources,
    /// Git diff and history.
    DiffsHistory,
    /// Terminal, task, preview, or runtime artifacts.
    RuntimeArtifacts,
    /// Repo, workspace, policy, or user instruction sources.
    InstructionSources,
    /// External tool or connector results.
    ExternalToolResults,
}

impl ContextGroupClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenFiles => "open_files",
            Self::SymbolsGraphEntities => "symbols_graph_entities",
            Self::DiagnosticsTests => "diagnostics_tests",
            Self::DocsKnowledgeSources => "docs_knowledge_sources",
            Self::DiffsHistory => "diffs_history",
            Self::RuntimeArtifacts => "runtime_artifacts",
            Self::InstructionSources => "instruction_sources",
            Self::ExternalToolResults => "external_tool_results",
        }
    }

    /// Human-readable group label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenFiles => "Open files",
            Self::SymbolsGraphEntities => "Symbols and graph entities",
            Self::DiagnosticsTests => "Diagnostics and tests",
            Self::DocsKnowledgeSources => "Docs and knowledge sources",
            Self::DiffsHistory => "Diffs and history",
            Self::RuntimeArtifacts => "Runtime artifacts",
            Self::InstructionSources => "Instruction sources",
            Self::ExternalToolResults => "External tool results",
        }
    }
}

/// Inclusion state of a context item before send.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextItemStateClass {
    /// Context will be included.
    Included,
    /// User or policy pinned the context.
    Pinned,
    /// Context was intentionally omitted and carries a reason.
    Omitted,
    /// Context is blocked by policy, trust, or scope.
    Blocked,
    /// Context is stale or changed since pin.
    Stale,
    /// Context is tainted and must remain fenced as data.
    Tainted,
    /// Context was summarized rather than included raw.
    Summarized,
    /// Context was not requested.
    NotRequested,
}

impl ContextItemStateClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Included => "included",
            Self::Pinned => "pinned",
            Self::Omitted => "omitted",
            Self::Blocked => "blocked",
            Self::Stale => "stale",
            Self::Tainted => "tainted",
            Self::Summarized => "summarized",
            Self::NotRequested => "not_requested",
        }
    }

    /// True when the item should remain visible in evidence handoff rows.
    pub const fn must_survive_handoff(self) -> bool {
        matches!(
            self,
            Self::Pinned | Self::Omitted | Self::Blocked | Self::Stale | Self::Tainted
        )
    }
}

/// Reason context was omitted, summarized, or blocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextOmissionReasonClass {
    /// The item exceeded the disclosed budget.
    Budget,
    /// Policy or trust settings excluded the item.
    Policy,
    /// The item duplicated higher-priority context.
    Duplicate,
    /// The item was stale.
    Stale,
    /// The item was blocked by a resolver or authority check.
    Blocked,
    /// The item was tainted and could not be promoted.
    Tainted,
    /// The item was outside the current scope or workset.
    ScopeExcluded,
    /// The docs item lacked a usable citation anchor.
    CitationAnchorUnavailable,
}

impl ContextOmissionReasonClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Budget => "budget",
            Self::Policy => "policy",
            Self::Duplicate => "duplicate",
            Self::Stale => "stale",
            Self::Blocked => "blocked",
            Self::Tainted => "tainted",
            Self::ScopeExcluded => "scope_excluded",
            Self::CitationAnchorUnavailable => "citation_anchor_unavailable",
        }
    }
}

/// Freshness class for an item shown in the context inspector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextFreshnessClass {
    /// Source is authoritative and live at review time.
    AuthoritativeLive,
    /// Source is current enough from a warm cache.
    WarmCached,
    /// Source is stale but disclosed.
    Stale,
    /// Source changed after the user pinned it.
    ChangedSincePin,
    /// Source freshness could not be verified.
    Unverified,
}

impl ContextFreshnessClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::Stale => "stale",
            Self::ChangedSincePin => "changed_since_pin",
            Self::Unverified => "unverified",
        }
    }
}

/// Trust class for context before send.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextTrustClass {
    /// First-party workspace or product source.
    TrustedFirstParty,
    /// Trusted docs or policy authority.
    TrustedAuthority,
    /// Derived source that has been reviewed.
    ReviewedDerived,
    /// Derived source that has not been reviewed.
    UnreviewedDerived,
    /// External or untrusted source.
    UntrustedExternal,
    /// Source quarantined by policy.
    PolicyQuarantined,
}

impl ContextTrustClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedFirstParty => "trusted_first_party",
            Self::TrustedAuthority => "trusted_authority",
            Self::ReviewedDerived => "reviewed_derived",
            Self::UnreviewedDerived => "unreviewed_derived",
            Self::UntrustedExternal => "untrusted_external",
            Self::PolicyQuarantined => "policy_quarantined",
        }
    }
}

/// Locality class for where context was obtained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextLocalityClass {
    /// Current local workspace.
    LocalWorkspace,
    /// Local cache.
    LocalCache,
    /// Mirrored docs pack.
    MirroredDocsPack,
    /// Managed recall or hosted index.
    ManagedRecall,
    /// Provider overlay.
    ProviderOverlay,
    /// Remote runtime or task host.
    RemoteRuntime,
    /// Outside the current workset or scope.
    OutsideCurrentScope,
}

impl ContextLocalityClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkspace => "local_workspace",
            Self::LocalCache => "local_cache",
            Self::MirroredDocsPack => "mirrored_docs_pack",
            Self::ManagedRecall => "managed_recall",
            Self::ProviderOverlay => "provider_overlay",
            Self::RemoteRuntime => "remote_runtime",
            Self::OutsideCurrentScope => "outside_current_scope",
        }
    }
}

/// Documentation or knowledge source class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsKnowledgeSourceClass {
    /// Project-local docs, ADRs, or runbooks.
    ProjectDocs,
    /// Mirrored or offline docs pack.
    MirroredDocsPack,
    /// Live vendor documentation.
    LiveVendorDocs,
    /// Generated reference source.
    GeneratedReference,
    /// Glossary or learning pack.
    GlossaryPack,
    /// Inference-backed derived knowledge.
    InferenceBacked,
}

impl DocsKnowledgeSourceClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectDocs => "project_docs",
            Self::MirroredDocsPack => "mirrored_docs_pack",
            Self::LiveVendorDocs => "live_vendor_docs",
            Self::GeneratedReference => "generated_reference",
            Self::GlossaryPack => "glossary_pack",
            Self::InferenceBacked => "inference_backed",
        }
    }
}

/// Citation-anchor availability for a docs or knowledge item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationAnchorAvailabilityClass {
    /// Exact anchor exists and is recorded.
    ExactAnchorAvailable,
    /// The source is citable, but exact anchor is unavailable.
    AnchorUnavailableDisclosed,
    /// The anchor is hidden by policy.
    HiddenByPolicy,
    /// The item is not citation-bearing.
    NotCitationBearing,
}

impl CitationAnchorAvailabilityClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactAnchorAvailable => "exact_anchor_available",
            Self::AnchorUnavailableDisclosed => "anchor_unavailable_disclosed",
            Self::HiddenByPolicy => "hidden_by_policy",
            Self::NotCitationBearing => "not_citation_bearing",
        }
    }

    /// True when the source needs a visible note.
    pub const fn requires_note(self) -> bool {
        matches!(
            self,
            Self::AnchorUnavailableDisclosed | Self::HiddenByPolicy
        )
    }
}

/// Locale/source-language fallback state for docs and knowledge attachments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceLanguageFallbackClass {
    /// The active text is already source-language text.
    SourceLanguageOriginal,
    /// The requested locale was available and citation-preserving.
    RequestedLocaleAvailable,
    /// The product fell back to source language.
    FallbackToSourceLanguage,
    /// No source-language escape hatch is available.
    SourceLanguageUnavailable,
}

impl SourceLanguageFallbackClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceLanguageOriginal => "source_language_original",
            Self::RequestedLocaleAvailable => "requested_locale_available",
            Self::FallbackToSourceLanguage => "fallback_to_source_language",
            Self::SourceLanguageUnavailable => "source_language_unavailable",
        }
    }
}

/// Mention preview state before send.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MentionPreviewStateClass {
    /// Mention resolves to exactly one stable object.
    ResolvedExact,
    /// Mention has multiple candidates and requires user resolution.
    Ambiguous,
    /// Mention did not resolve.
    Unresolved,
    /// Mention is stale.
    Stale,
    /// Mention is blocked by policy or scope.
    Blocked,
}

impl MentionPreviewStateClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolvedExact => "resolved_exact",
            Self::Ambiguous => "ambiguous",
            Self::Unresolved => "unresolved",
            Self::Stale => "stale",
            Self::Blocked => "blocked",
        }
    }

    /// True when the mention prevents send.
    pub const fn blocks_send(self) -> bool {
        !matches!(self, Self::ResolvedExact)
    }
}

/// Review lock class for no-silent-widening behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewLockClass {
    /// Review has not started.
    NotStarted,
    /// Review has started and context/route are frozen until explicit review.
    FrozenForReview,
    /// A route, scope, or source changed and requires another review.
    ReReviewRequired,
}

impl ReviewLockClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::FrozenForReview => "frozen_for_review",
            Self::ReReviewRequired => "re_review_required",
        }
    }
}

/// Send-readiness state of a pre-dispatch snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComposerContextReviewState {
    /// The snapshot is ready for send.
    ReadyToSend,
    /// User or resolver action is required before send.
    BlockedPendingResolution,
    /// Budget pressure requires omission, summarization, or route review.
    BudgetReviewRequired,
    /// Context or route changed after review began.
    ReReviewRequired,
}

impl ComposerContextReviewState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyToSend => "ready_to_send",
            Self::BlockedPendingResolution => "blocked_pending_resolution",
            Self::BudgetReviewRequired => "budget_review_required",
            Self::ReReviewRequired => "re_review_required",
        }
    }
}

/// Documentation and knowledge identity preserved from composer to evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsKnowledgeIdentity {
    /// Stable docs or knowledge node ref.
    pub docs_node_ref: String,
    /// Docs source class.
    pub source_class: DocsKnowledgeSourceClass,
    /// Version, revision, or snapshot ref for the docs node.
    pub version_or_revision_ref: String,
    /// Docs, glossary, or knowledge pack ref when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_pack_ref: Option<String>,
    /// Exact citation anchor ref when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_anchor_ref: Option<String>,
    /// Citation-anchor availability.
    pub citation_availability_class: CitationAnchorAvailabilityClass,
    /// Visible note for hidden or unavailable anchors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation_note: Option<String>,
    /// Source language of the canonical docs item.
    pub source_language: String,
    /// Active locale shown to the user.
    pub active_language: String,
    /// Source-language fallback state.
    pub source_language_fallback_class: SourceLanguageFallbackClass,
}

impl DocsKnowledgeIdentity {
    /// Validates docs identity fields without inspecting any document body.
    pub fn validate(&self) -> Vec<ComposerContextAlphaViolation> {
        let mut violations = Vec::new();
        if self.docs_node_ref.trim().is_empty()
            || self.version_or_revision_ref.trim().is_empty()
            || self.source_language.trim().is_empty()
            || self.active_language.trim().is_empty()
        {
            violations.push(ComposerContextAlphaViolation::DocsIdentityMissing);
        }
        if self.citation_availability_class == CitationAnchorAvailabilityClass::ExactAnchorAvailable
            && self
                .exact_anchor_ref
                .as_deref()
                .map_or(true, |value| value.trim().is_empty())
        {
            violations.push(ComposerContextAlphaViolation::DocsCitationAnchorMissing);
        }
        if self.citation_availability_class.requires_note()
            && self
                .citation_note
                .as_deref()
                .map_or(true, |value| value.trim().is_empty())
        {
            violations.push(ComposerContextAlphaViolation::DocsCitationNoteMissing);
        }
        if self.source_language_fallback_class
            == SourceLanguageFallbackClass::SourceLanguageUnavailable
        {
            violations.push(ComposerContextAlphaViolation::SourceLanguageFallbackMissing);
        }
        violations
    }
}

/// Mention preview row shown before send.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerMentionPreview {
    /// Mention id from the composer draft.
    pub mention_id: String,
    /// Mention kind.
    pub kind: MentionKind,
    /// Preview state.
    pub preview_state: MentionPreviewStateClass,
    /// Stable target id when resolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_stable_id: Option<String>,
    /// Candidate target refs when the mention is ambiguous.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub candidate_target_refs: Vec<String>,
    /// User-visible preview label.
    pub display_label: String,
    /// Docs identity when the mention resolves to a docs or knowledge node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_identity: Option<DocsKnowledgeIdentity>,
}

impl ComposerMentionPreview {
    fn from_draft_mention(mention: &crate::composer::ComposerMention) -> Self {
        let preview_state = match mention.resolution_state {
            MentionResolutionState::Resolved => MentionPreviewStateClass::ResolvedExact,
            MentionResolutionState::UnresolvedNotFound => MentionPreviewStateClass::Unresolved,
            MentionResolutionState::UnresolvedScopeExcluded => MentionPreviewStateClass::Blocked,
            MentionResolutionState::UnresolvedStale => MentionPreviewStateClass::Stale,
        };
        Self {
            mention_id: mention.mention_id.clone(),
            kind: mention.kind,
            preview_state,
            target_stable_id: mention.target_stable_id.clone(),
            candidate_target_refs: Vec::new(),
            display_label: mention.display_label.clone(),
            docs_identity: None,
        }
    }
}

/// Typed attachment pill shown in the composer before send.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerAttachmentPill {
    /// Attachment id from the composer draft.
    pub attachment_id: String,
    /// Attachment kind.
    pub kind: AttachmentKind,
    /// Source class from the draft.
    pub source_class: SourceClass,
    /// Trust posture from the draft.
    pub trust_posture: TrustPosture,
    /// Why the attachment was selected.
    pub selection_reason: SelectionReasonClass,
    /// Attachment status from the draft.
    pub status: AttachmentStatusClass,
    /// Inclusion state projected for the pre-send inspector.
    pub context_state: ContextItemStateClass,
    /// Display label.
    pub display_label: String,
    /// Rough byte estimate.
    pub estimated_byte_size: u64,
    /// True when the user can remove the attachment before send.
    pub removable: bool,
    /// Docs identity when this is a docs or knowledge attachment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_identity: Option<DocsKnowledgeIdentity>,
}

impl ComposerAttachmentPill {
    fn from_draft_attachment(attachment: &crate::composer::ComposerAttachment) -> Self {
        let context_state = match attachment.status {
            AttachmentStatusClass::Live => {
                if attachment.trust_posture.requires_fence() {
                    ContextItemStateClass::Tainted
                } else {
                    ContextItemStateClass::Included
                }
            }
            AttachmentStatusClass::Stale => ContextItemStateClass::Stale,
            AttachmentStatusClass::TaintedOutsideFencedSection => ContextItemStateClass::Tainted,
            AttachmentStatusClass::OverBudget => ContextItemStateClass::Omitted,
            AttachmentStatusClass::PolicyBlocked | AttachmentStatusClass::OutOfScope => {
                ContextItemStateClass::Blocked
            }
        };
        Self {
            attachment_id: attachment.attachment_id.clone(),
            kind: attachment.kind,
            source_class: attachment.source_class,
            trust_posture: attachment.trust_posture,
            selection_reason: attachment.selection_reason,
            status: attachment.status,
            context_state,
            display_label: attachment.display_label.clone(),
            estimated_byte_size: attachment.estimated_byte_size,
            removable: true,
            docs_identity: None,
        }
    }
}

/// One context row grouped in the inspector before send.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerContextItem {
    /// Stable context item id.
    pub context_item_id: String,
    /// Group this item appears in.
    pub group_class: ContextGroupClass,
    /// Inclusion state.
    pub state_class: ContextItemStateClass,
    /// Source class token from the broader AI context contract.
    pub source_class: SourceClass,
    /// Stable object identity ref.
    pub stable_identity_ref: String,
    /// User-visible label.
    pub display_label: String,
    /// Freshness class.
    pub freshness_class: ContextFreshnessClass,
    /// Trust class.
    pub trust_class: ContextTrustClass,
    /// Locality class.
    pub locality_class: ContextLocalityClass,
    /// Rough byte estimate.
    pub estimated_byte_size: u64,
    /// Reason when the row is omitted, summarized, blocked, stale, or tainted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub omission_reason_class: Option<ContextOmissionReasonClass>,
    /// Attachment id this context row came from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_attachment_ref: Option<String>,
    /// Mention id this context row came from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_mention_ref: Option<String>,
    /// Docs identity when this context item is a docs or knowledge row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_identity: Option<DocsKnowledgeIdentity>,
}

impl ComposerContextItem {
    /// True when the item is a docs or knowledge row.
    pub fn is_docs_or_knowledge(&self) -> bool {
        self.group_class == ContextGroupClass::DocsKnowledgeSources
            || matches!(
                self.source_class,
                SourceClass::DocsPackExcerpt | SourceClass::CitationAnchorQuote
            )
    }
}

/// Budget strip shown before send.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerBudgetStrip {
    /// Aggregate byte estimate from attachment pills and context rows.
    pub aggregate_byte_estimate: u64,
    /// Budget ceiling used for the pre-send review.
    pub budget_byte_ceiling: u64,
    /// Budget pressure class.
    pub pressure_class: BudgetPressureClass,
    /// Included context group tokens.
    pub included_context_group_tokens: Vec<String>,
    /// Omitted or trimmed context group tokens.
    pub omitted_or_trimmed_group_tokens: Vec<String>,
    /// Selected provider label from the routing packet.
    pub selected_provider_label: String,
    /// Selected model label from the routing packet.
    pub selected_model_label: String,
    /// Quota state token from the routing packet.
    pub quota_state_token: String,
    /// Cost envelope token from the routing packet.
    pub cost_envelope_token: String,
}

/// Review lock fields that prevent silent widening after inspection begins.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerContextReviewLock {
    /// Review lock class.
    pub lock_class: ReviewLockClass,
    /// Frozen context snapshot ref.
    pub context_snapshot_ref: String,
    /// Frozen route snapshot ref.
    pub route_snapshot_ref: String,
    /// Review-start timestamp when review has started.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_started_at: Option<String>,
}

/// Constructor input for [`ComposerContextAlphaSnapshot::project`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerContextAlphaInput {
    /// Intent mode selected by the user.
    pub intent_mode: IntentModeClass,
    /// Current scope label.
    pub scope_label: String,
    /// Execution boundary label.
    pub execution_boundary_class: ExecutionBoundaryClass,
    /// Stable slash-command or action identity when one is active.
    pub action_identity_ref: Option<String>,
    /// Optional mention preview overrides. Missing draft mentions are projected.
    pub mention_previews: Vec<ComposerMentionPreview>,
    /// Optional attachment pill overrides. Missing draft attachments are projected.
    pub attachment_pills: Vec<ComposerAttachmentPill>,
    /// Grouped context rows.
    pub context_items: Vec<ComposerContextItem>,
    /// Review lock fields.
    pub review_lock: ComposerContextReviewLock,
}

/// Alpha pre-send snapshot for prompt composition and context inspection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerContextAlphaSnapshot {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Composer draft id.
    pub composer_draft_id: String,
    /// Composer session id.
    pub composer_session_id: String,
    /// Request workspace id.
    pub request_workspace_id: String,
    /// Intent text from the draft.
    pub intent_text: String,
    /// Intent mode.
    pub intent_mode: IntentModeClass,
    /// Current scope label.
    pub scope_label: String,
    /// Execution boundary class.
    pub execution_boundary_class: ExecutionBoundaryClass,
    /// Action identity ref when a slash command or governed action is active.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_identity_ref: Option<String>,
    /// Mention resolution previews.
    pub mention_previews: Vec<ComposerMentionPreview>,
    /// Typed attachment pills.
    pub attachment_pills: Vec<ComposerAttachmentPill>,
    /// Grouped context rows.
    pub context_items: Vec<ComposerContextItem>,
    /// Budget and route strip.
    pub budget_strip: ComposerBudgetStrip,
    /// Review lock.
    pub review_lock: ComposerContextReviewLock,
    /// Derived review state.
    pub review_state: ComposerContextReviewState,
}

impl ComposerContextAlphaSnapshot {
    /// Project a pre-send snapshot from a draft, routing packet, and explicit context rows.
    pub fn project(
        draft: &ComposerDraft,
        routing_packet: &AiRoutingPacket,
        input: ComposerContextAlphaInput,
    ) -> Self {
        let mention_previews = merge_mention_previews(draft, input.mention_previews);
        let attachment_pills = merge_attachment_pills(draft, input.attachment_pills);
        let budget_strip = project_budget_strip(
            draft,
            routing_packet,
            &attachment_pills,
            &input.context_items,
        );
        let review_state = derive_review_state(
            draft,
            &mention_previews,
            &attachment_pills,
            &input.context_items,
            budget_strip.pressure_class,
            input.review_lock.lock_class,
        );

        Self {
            record_kind: COMPOSER_CONTEXT_ALPHA_RECORD_KIND.to_owned(),
            schema_version: COMPOSER_CONTEXT_ALPHA_SCHEMA_VERSION,
            composer_draft_id: draft.composer_draft_id.clone(),
            composer_session_id: draft.composer_session_id.clone(),
            request_workspace_id: draft.request_workspace_id.clone(),
            intent_text: draft.intent_text.clone(),
            intent_mode: input.intent_mode,
            scope_label: input.scope_label,
            execution_boundary_class: input.execution_boundary_class,
            action_identity_ref: input.action_identity_ref,
            mention_previews,
            attachment_pills,
            context_items: input.context_items,
            budget_strip,
            review_lock: input.review_lock,
            review_state,
        }
    }

    /// Builds the canonical context handoff row set for evidence packets.
    pub fn evidence_handoff(&self, handoff_id: impl Into<String>) -> AiContextEvidenceHandoff {
        AiContextEvidenceHandoff {
            record_kind: AI_CONTEXT_EVIDENCE_HANDOFF_RECORD_KIND.to_owned(),
            schema_version: COMPOSER_CONTEXT_ALPHA_SCHEMA_VERSION,
            handoff_id: handoff_id.into(),
            composer_context_snapshot_ref: self.review_lock.context_snapshot_ref.clone(),
            composer_session_ref: self.composer_session_id.clone(),
            turn_draft_ref: self.composer_draft_id.clone(),
            request_workspace_ref: self.request_workspace_id.clone(),
            context_rows: self
                .context_items
                .iter()
                .filter(|item| {
                    item.state_class.must_survive_handoff()
                        || item.docs_identity.is_some()
                        || item.source_attachment_ref.is_some()
                        || item.source_mention_ref.is_some()
                })
                .map(AiContextEvidenceHandoffRow::from_context_item)
                .collect(),
        }
    }

    /// Validate the pre-send snapshot against the alpha context invariants.
    pub fn validate(&self) -> Vec<ComposerContextAlphaViolation> {
        let mut violations = Vec::new();
        if self.record_kind != COMPOSER_CONTEXT_ALPHA_RECORD_KIND {
            violations.push(ComposerContextAlphaViolation::WrongRecordKind);
        }
        if self.schema_version != COMPOSER_CONTEXT_ALPHA_SCHEMA_VERSION {
            violations.push(ComposerContextAlphaViolation::WrongSchemaVersion);
        }
        if self.composer_draft_id.trim().is_empty()
            || self.composer_session_id.trim().is_empty()
            || self.request_workspace_id.trim().is_empty()
        {
            violations.push(ComposerContextAlphaViolation::MissingComposerIdentity);
        }
        if self.review_lock.context_snapshot_ref.trim().is_empty()
            || self.review_lock.route_snapshot_ref.trim().is_empty()
        {
            violations.push(ComposerContextAlphaViolation::ReviewLockMissingSnapshotRef);
        }
        if self.review_lock.lock_class == ReviewLockClass::ReReviewRequired
            && self.review_state != ComposerContextReviewState::ReReviewRequired
        {
            violations.push(ComposerContextAlphaViolation::ReviewStartedButScopeChanged);
        }
        if self.budget_strip.pressure_class == BudgetPressureClass::Overflow
            && self.review_state != ComposerContextReviewState::BudgetReviewRequired
            && self.review_state != ComposerContextReviewState::BlockedPendingResolution
        {
            violations.push(ComposerContextAlphaViolation::BudgetOverflowNotSurfaced);
        }

        for mention in &self.mention_previews {
            if mention.preview_state == MentionPreviewStateClass::Ambiguous
                && mention.candidate_target_refs.len() < 2
            {
                violations.push(ComposerContextAlphaViolation::MentionAmbiguityWithoutCandidates);
            }
            if let Some(docs_identity) = &mention.docs_identity {
                violations.extend(docs_identity.validate());
            }
        }

        for pill in &self.attachment_pills {
            if let Some(docs_identity) = &pill.docs_identity {
                violations.extend(docs_identity.validate());
            }
        }

        for item in &self.context_items {
            if item.stable_identity_ref.trim().is_empty() {
                violations.push(ComposerContextAlphaViolation::ContextItemIdentityMissing);
            }
            if matches!(
                item.state_class,
                ContextItemStateClass::Omitted
                    | ContextItemStateClass::Blocked
                    | ContextItemStateClass::Stale
                    | ContextItemStateClass::Tainted
                    | ContextItemStateClass::Summarized
            ) && item.omission_reason_class.is_none()
            {
                violations.push(ComposerContextAlphaViolation::OmittedContextReasonMissing);
            }
            if item.is_docs_or_knowledge() && item.docs_identity.is_none() {
                violations.push(ComposerContextAlphaViolation::DocsIdentityMissing);
            }
            if let Some(docs_identity) = &item.docs_identity {
                violations.extend(docs_identity.validate());
            }
        }
        violations
    }

    /// Deterministic Markdown summary for review or support handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# AI Composer Context Alpha\n\n");
        out.push_str(&format!("- Draft: `{}`\n", self.composer_draft_id));
        out.push_str(&format!("- Session: `{}`\n", self.composer_session_id));
        out.push_str(&format!("- Mode: `{}`\n", self.intent_mode.as_str()));
        out.push_str(&format!("- Scope: `{}`\n", self.scope_label));
        out.push_str(&format!(
            "- Boundary: `{}`\n",
            self.execution_boundary_class.as_str()
        ));
        out.push_str(&format!(
            "- Review state: `{}`\n",
            self.review_state.as_str()
        ));
        out.push_str(&format!(
            "- Budget: `{}` ({} / {} bytes)\n",
            self.budget_strip.pressure_class.as_str(),
            self.budget_strip.aggregate_byte_estimate,
            self.budget_strip.budget_byte_ceiling
        ));
        out.push_str(&format!(
            "- Route: {} / {} / `{}`\n",
            self.budget_strip.selected_provider_label,
            self.budget_strip.selected_model_label,
            self.budget_strip.quota_state_token
        ));
        out.push_str("\n## Context Groups\n\n");
        for group in all_context_groups() {
            let rows: Vec<_> = self
                .context_items
                .iter()
                .filter(|item| item.group_class == group)
                .collect();
            if rows.is_empty() {
                continue;
            }
            out.push_str(&format!("### {}\n\n", group.label()));
            for item in rows {
                out.push_str(&format!(
                    "- `{}`: `{}` / `{}` / `{}` / `{}`\n",
                    item.context_item_id,
                    item.state_class.as_str(),
                    item.source_class.as_str(),
                    item.freshness_class.as_str(),
                    item.trust_class.as_str()
                ));
            }
            out.push('\n');
        }
        out
    }
}

/// Evidence handoff that downstream packets ingest without re-deriving context truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiContextEvidenceHandoff {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable handoff id.
    pub handoff_id: String,
    /// Source composer context snapshot ref.
    pub composer_context_snapshot_ref: String,
    /// Composer session ref.
    pub composer_session_ref: String,
    /// Turn draft ref.
    pub turn_draft_ref: String,
    /// Request workspace ref.
    pub request_workspace_ref: String,
    /// Context rows that must survive into downstream evidence.
    pub context_rows: Vec<AiContextEvidenceHandoffRow>,
}

impl AiContextEvidenceHandoff {
    /// Validate the handoff without resolving bodies.
    pub fn validate(&self) -> Vec<ComposerContextAlphaViolation> {
        let mut violations = Vec::new();
        if self.record_kind != AI_CONTEXT_EVIDENCE_HANDOFF_RECORD_KIND {
            violations.push(ComposerContextAlphaViolation::WrongRecordKind);
        }
        if self.schema_version != COMPOSER_CONTEXT_ALPHA_SCHEMA_VERSION {
            violations.push(ComposerContextAlphaViolation::WrongSchemaVersion);
        }
        if self.handoff_id.trim().is_empty()
            || self.composer_context_snapshot_ref.trim().is_empty()
            || self.composer_session_ref.trim().is_empty()
            || self.turn_draft_ref.trim().is_empty()
            || self.request_workspace_ref.trim().is_empty()
        {
            violations.push(ComposerContextAlphaViolation::MissingComposerIdentity);
        }
        for row in &self.context_rows {
            if row.context_item_id.trim().is_empty() || row.stable_identity_ref.trim().is_empty() {
                violations.push(ComposerContextAlphaViolation::ContextItemIdentityMissing);
            }
            if matches!(
                row.state_token.as_str(),
                "omitted" | "blocked" | "stale" | "tainted" | "summarized"
            ) && row.omission_reason_token.is_none()
            {
                violations.push(ComposerContextAlphaViolation::OmittedContextReasonMissing);
            }
            if matches!(row.group_token.as_str(), "docs_knowledge_sources")
                && row.docs_node_ref.is_none()
            {
                violations.push(ComposerContextAlphaViolation::DocsIdentityMissing);
            }
            if row.citation_availability_token.as_deref() == Some("exact_anchor_available")
                && row.exact_anchor_ref.is_none()
            {
                violations.push(ComposerContextAlphaViolation::DocsCitationAnchorMissing);
            }
            if row.source_language_fallback_token.as_deref() == Some("source_language_unavailable")
            {
                violations.push(ComposerContextAlphaViolation::SourceLanguageFallbackMissing);
            }
        }
        violations
    }
}

/// Context row handed from the composer/context inspector into evidence packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiContextEvidenceHandoffRow {
    /// Context item id.
    pub context_item_id: String,
    /// Context group token.
    pub group_token: String,
    /// Context state token.
    pub state_token: String,
    /// Source class token.
    pub source_class_token: String,
    /// Stable identity ref.
    pub stable_identity_ref: String,
    /// Freshness token.
    pub freshness_token: String,
    /// Trust token.
    pub trust_token: String,
    /// Locality token.
    pub locality_token: String,
    /// Omission reason token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub omission_reason_token: Option<String>,
    /// Source attachment ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_attachment_ref: Option<String>,
    /// Source mention ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_mention_ref: Option<String>,
    /// Docs or knowledge node ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_node_ref: Option<String>,
    /// Docs source class token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_source_class_token: Option<String>,
    /// Version or revision ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_or_revision_ref: Option<String>,
    /// Exact anchor ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_anchor_ref: Option<String>,
    /// Citation availability token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation_availability_token: Option<String>,
    /// Source-language fallback token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_language_fallback_token: Option<String>,
}

impl AiContextEvidenceHandoffRow {
    fn from_context_item(item: &ComposerContextItem) -> Self {
        let docs = item.docs_identity.as_ref();
        Self {
            context_item_id: item.context_item_id.clone(),
            group_token: item.group_class.as_str().to_owned(),
            state_token: item.state_class.as_str().to_owned(),
            source_class_token: item.source_class.as_str().to_owned(),
            stable_identity_ref: item.stable_identity_ref.clone(),
            freshness_token: item.freshness_class.as_str().to_owned(),
            trust_token: item.trust_class.as_str().to_owned(),
            locality_token: item.locality_class.as_str().to_owned(),
            omission_reason_token: item
                .omission_reason_class
                .map(|reason| reason.as_str().to_owned()),
            source_attachment_ref: item.source_attachment_ref.clone(),
            source_mention_ref: item.source_mention_ref.clone(),
            docs_node_ref: docs.map(|identity| identity.docs_node_ref.clone()),
            docs_source_class_token: docs.map(|identity| identity.source_class.as_str().to_owned()),
            version_or_revision_ref: docs.map(|identity| identity.version_or_revision_ref.clone()),
            exact_anchor_ref: docs.and_then(|identity| identity.exact_anchor_ref.clone()),
            citation_availability_token: docs
                .map(|identity| identity.citation_availability_class.as_str().to_owned()),
            source_language_fallback_token: docs
                .map(|identity| identity.source_language_fallback_class.as_str().to_owned()),
        }
    }
}

/// Validation failures emitted by [`ComposerContextAlphaSnapshot::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComposerContextAlphaViolation {
    /// Record kind is wrong.
    WrongRecordKind,
    /// Schema version is wrong.
    WrongSchemaVersion,
    /// Composer, session, or request-workspace identity is missing.
    MissingComposerIdentity,
    /// Context item identity is missing.
    ContextItemIdentityMissing,
    /// Omitted, blocked, stale, tainted, or summarized context lacks a reason.
    OmittedContextReasonMissing,
    /// Docs or knowledge identity is missing.
    DocsIdentityMissing,
    /// Exact citation anchor was claimed but not recorded.
    DocsCitationAnchorMissing,
    /// Hidden or unavailable citation lacks a visible note.
    DocsCitationNoteMissing,
    /// Source-language fallback is unavailable or missing.
    SourceLanguageFallbackMissing,
    /// Mention ambiguity does not list enough candidates.
    MentionAmbiguityWithoutCandidates,
    /// Budget overflow was not surfaced in the review state.
    BudgetOverflowNotSurfaced,
    /// Review lock is missing context or route snapshot refs.
    ReviewLockMissingSnapshotRef,
    /// Context, route, or scope changed after review began.
    ReviewStartedButScopeChanged,
}

impl ComposerContextAlphaViolation {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingComposerIdentity => "missing_composer_identity",
            Self::ContextItemIdentityMissing => "context_item_identity_missing",
            Self::OmittedContextReasonMissing => "omitted_context_reason_missing",
            Self::DocsIdentityMissing => "docs_identity_missing",
            Self::DocsCitationAnchorMissing => "docs_citation_anchor_missing",
            Self::DocsCitationNoteMissing => "docs_citation_note_missing",
            Self::SourceLanguageFallbackMissing => "source_language_fallback_missing",
            Self::MentionAmbiguityWithoutCandidates => "mention_ambiguity_without_candidates",
            Self::BudgetOverflowNotSurfaced => "budget_overflow_not_surfaced",
            Self::ReviewLockMissingSnapshotRef => "review_lock_missing_snapshot_ref",
            Self::ReviewStartedButScopeChanged => "review_started_but_scope_changed",
        }
    }
}

fn merge_mention_previews(
    draft: &ComposerDraft,
    mut overrides: Vec<ComposerMentionPreview>,
) -> Vec<ComposerMentionPreview> {
    for mention in &draft.mentions {
        if !overrides
            .iter()
            .any(|preview| preview.mention_id == mention.mention_id)
        {
            overrides.push(ComposerMentionPreview::from_draft_mention(mention));
        }
    }
    overrides
}

fn merge_attachment_pills(
    draft: &ComposerDraft,
    mut overrides: Vec<ComposerAttachmentPill>,
) -> Vec<ComposerAttachmentPill> {
    for attachment in &draft.attachments {
        if !overrides
            .iter()
            .any(|pill| pill.attachment_id == attachment.attachment_id)
        {
            overrides.push(ComposerAttachmentPill::from_draft_attachment(attachment));
        }
    }
    overrides
}

fn project_budget_strip(
    draft: &ComposerDraft,
    routing_packet: &AiRoutingPacket,
    attachment_pills: &[ComposerAttachmentPill],
    context_items: &[ComposerContextItem],
) -> ComposerBudgetStrip {
    let attachment_bytes = attachment_pills.iter().fold(0_u64, |acc, pill| {
        acc.saturating_add(pill.estimated_byte_size)
    });
    let context_bytes = context_items.iter().fold(0_u64, |acc, item| {
        acc.saturating_add(item.estimated_byte_size)
    });
    let aggregate = attachment_bytes.saturating_add(context_bytes);
    let pressure_class = if aggregate > draft.budget_byte_ceiling {
        BudgetPressureClass::Overflow
    } else if aggregate.saturating_mul(5) >= draft.budget_byte_ceiling.saturating_mul(4) {
        BudgetPressureClass::Warning
    } else {
        BudgetPressureClass::WithinBudget
    };

    let included_context_group_tokens = group_tokens_for_states(
        context_items,
        &[
            ContextItemStateClass::Included,
            ContextItemStateClass::Pinned,
        ],
    );
    let omitted_or_trimmed_group_tokens = group_tokens_for_states(
        context_items,
        &[
            ContextItemStateClass::Omitted,
            ContextItemStateClass::Blocked,
            ContextItemStateClass::Stale,
            ContextItemStateClass::Tainted,
            ContextItemStateClass::Summarized,
        ],
    );

    let selected = routing_packet.selected_route();
    ComposerBudgetStrip {
        aggregate_byte_estimate: aggregate,
        budget_byte_ceiling: draft.budget_byte_ceiling,
        pressure_class,
        included_context_group_tokens,
        omitted_or_trimmed_group_tokens,
        selected_provider_label: selected
            .map(|candidate| candidate.provider_label.clone())
            .unwrap_or_default(),
        selected_model_label: selected
            .map(|candidate| candidate.model_label.clone())
            .unwrap_or_default(),
        quota_state_token: selected
            .map(|candidate| candidate.quota.quota_state_class.as_str().to_owned())
            .unwrap_or_default(),
        cost_envelope_token: selected
            .map(|candidate| candidate.envelope.cost_envelope_class.as_str().to_owned())
            .unwrap_or_default(),
    }
}

fn group_tokens_for_states(
    context_items: &[ComposerContextItem],
    states: &[ContextItemStateClass],
) -> Vec<String> {
    let mut tokens = Vec::new();
    for item in context_items {
        if states.contains(&item.state_class) {
            let token = item.group_class.as_str().to_owned();
            if !tokens.contains(&token) {
                tokens.push(token);
            }
        }
    }
    tokens
}

fn derive_review_state(
    draft: &ComposerDraft,
    mentions: &[ComposerMentionPreview],
    attachments: &[ComposerAttachmentPill],
    context_items: &[ComposerContextItem],
    pressure_class: BudgetPressureClass,
    review_lock_class: ReviewLockClass,
) -> ComposerContextReviewState {
    if review_lock_class == ReviewLockClass::ReReviewRequired {
        return ComposerContextReviewState::ReReviewRequired;
    }
    if pressure_class == BudgetPressureClass::Overflow {
        return ComposerContextReviewState::BudgetReviewRequired;
    }
    let draft_has_actionable_blocks = draft.has_actionable_block_reasons();
    let mentions_block = mentions
        .iter()
        .any(|mention| mention.preview_state.blocks_send());
    let attachments_block = attachments.iter().any(|pill| {
        matches!(
            pill.context_state,
            ContextItemStateClass::Blocked | ContextItemStateClass::Stale
        )
    });
    let context_blocks = context_items.iter().any(|item| {
        matches!(
            item.state_class,
            ContextItemStateClass::Blocked | ContextItemStateClass::Stale
        )
    });

    if draft_has_actionable_blocks || mentions_block || attachments_block || context_blocks {
        ComposerContextReviewState::BlockedPendingResolution
    } else {
        ComposerContextReviewState::ReadyToSend
    }
}

fn all_context_groups() -> [ContextGroupClass; 8] {
    [
        ContextGroupClass::OpenFiles,
        ContextGroupClass::SymbolsGraphEntities,
        ContextGroupClass::DiagnosticsTests,
        ContextGroupClass::DocsKnowledgeSources,
        ContextGroupClass::DiffsHistory,
        ContextGroupClass::RuntimeArtifacts,
        ContextGroupClass::InstructionSources,
        ContextGroupClass::ExternalToolResults,
    ]
}

#[cfg(test)]
mod tests;
