use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use aureline_language::{
    RedactionClass, RouterCompletenessClass, RouterDecisionRecord, RouterDegradedStateClass,
    RouterFreshnessClass, RouterLocalityClass, RouterProviderKind, RouterScopeClaimClass,
    RouterSupportClass, ScopeLimitClass,
};
use serde::{Deserialize, Serialize};

/// Integer schema version for editor assist payloads.
pub type AssistSchemaVersion = u32;

/// Schema version used by completion, signature-help, and snippet-session records.
pub const ASSIST_SCHEMA_VERSION: AssistSchemaVersion = 1;

/// Error returned when assist records cannot be built from upstream provider state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssistContractError {
    /// The router decision did not contain the selected provider row.
    MissingSelectedProvider {
        /// Selected provider id that could not be resolved.
        selected_provider_id: String,
    },
}

impl fmt::Display for AssistContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSelectedProvider {
                selected_provider_id,
            } => write!(
                f,
                "router decision selected provider {selected_provider_id} but did not include its provider row"
            ),
        }
    }
}

impl Error for AssistContractError {}

/// Source family for one typing-assistance result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistSourceFamily {
    /// Result came from a supervised language server.
    LanguageServer,
    /// Result came from local lexical, syntax, or text fallback.
    FallbackLexical,
    /// Result came from a snippet pack or snippet session.
    Snippet,
    /// Result came from the project graph.
    ProjectGraph,
    /// Result came from a framework pack or schema overlay.
    FrameworkPack,
    /// Result came from an AI-assist provider.
    AiAssist,
    /// Result came from another structured tool adapter.
    ToolAdapter,
}

impl AssistSourceFamily {
    /// Projects a language-router provider kind into the assist source family.
    pub const fn from_provider_kind(provider_kind: RouterProviderKind) -> Self {
        match provider_kind {
            RouterProviderKind::LanguageServer => Self::LanguageServer,
            RouterProviderKind::SyntaxParser => Self::FallbackLexical,
            RouterProviderKind::ProjectGraph => Self::ProjectGraph,
            RouterProviderKind::FrameworkPack
            | RouterProviderKind::NativeAnalyzer
            | RouterProviderKind::GeneratedSourceBridge => Self::FrameworkPack,
            RouterProviderKind::AiAssist => Self::AiAssist,
            RouterProviderKind::DebugAdapter
            | RouterProviderKind::FormatterAdapter
            | RouterProviderKind::LinterAdapter
            | RouterProviderKind::TestAdapter
            | RouterProviderKind::BuildAdapter => Self::ToolAdapter,
        }
    }

    /// Returns true when this result is an explicit fallback.
    pub const fn is_fallback(self) -> bool {
        matches!(self, Self::FallbackLexical)
    }

    /// Returns true when this result is snippet-authored.
    pub const fn is_snippet(self) -> bool {
        matches!(self, Self::Snippet)
    }

    /// Returns true when this result is backed by an LSP provider.
    pub const fn is_language_server(self) -> bool {
        matches!(self, Self::LanguageServer)
    }

    /// Returns the stable schema token for this source family.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LanguageServer => "language_server",
            Self::FallbackLexical => "fallback_lexical",
            Self::Snippet => "snippet",
            Self::ProjectGraph => "project_graph",
            Self::FrameworkPack => "framework_pack",
            Self::AiAssist => "ai_assist",
            Self::ToolAdapter => "tool_adapter",
        }
    }
}

/// Source descriptor shared by completion items, signatures, and snippets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistSourceDescriptor {
    /// Stable descriptor id.
    pub source_descriptor_id: String,
    /// Normalized source family.
    pub source_family: AssistSourceFamily,
    /// Plain-language source label that consumers must keep visible.
    pub source_label: String,
    /// Provider id, when the source came through the language router.
    pub provider_id: Option<String>,
    /// Upstream router decision ref, when available.
    pub router_decision_ref: Option<String>,
    /// Opaque source object ref such as a snippet pack or lexical index.
    pub source_ref: Option<String>,
    /// Authority posture projected from the provider.
    pub support_class: RouterSupportClass,
    /// Freshness projected from the provider or source pack.
    pub freshness_class: RouterFreshnessClass,
    /// Scope claimed by the source.
    pub scope_claim_class: RouterScopeClaimClass,
    /// Completeness for the claimed scope.
    pub completeness_class: RouterCompletenessClass,
    /// Concrete scope limits that explain partial or fallback results.
    pub scope_limit_classes: Vec<ScopeLimitClass>,
    /// Locality where the source ran or was loaded.
    pub locality_class: RouterLocalityClass,
    /// Degraded state inherited from provider arbitration.
    pub degraded_state_class: RouterDegradedStateClass,
    /// Export-safe source summary.
    pub summary: String,
}

impl AssistSourceDescriptor {
    /// Builds a descriptor for the selected provider in a router decision.
    ///
    /// # Errors
    ///
    /// Returns [`AssistContractError::MissingSelectedProvider`] when the decision
    /// references a provider id that is absent from its provider stack.
    pub fn from_router_decision(
        decision: &RouterDecisionRecord,
    ) -> Result<Self, AssistContractError> {
        let selected_provider_id = &decision.decision_outcome.selected_provider_id;
        let row = decision
            .provider_stack_rows
            .iter()
            .find(|row| &row.provider_id == selected_provider_id)
            .ok_or_else(|| AssistContractError::MissingSelectedProvider {
                selected_provider_id: selected_provider_id.clone(),
            })?;

        Ok(Self {
            source_descriptor_id: format!(
                "assist-source:{}",
                sanitize_id(&decision.router_decision_id)
            ),
            source_family: AssistSourceFamily::from_provider_kind(row.provider_kind),
            source_label: row.provider_display_label.clone(),
            provider_id: Some(row.provider_id.clone()),
            router_decision_ref: Some(decision.router_decision_id.clone()),
            source_ref: None,
            support_class: row.support_class,
            freshness_class: row.freshness_class,
            scope_claim_class: decision.request_context.requested_scope_claim_class,
            completeness_class: completeness_for_selected_source(row.provider_kind, decision),
            scope_limit_classes: scope_limits_for_selected_source(row.provider_kind, decision),
            locality_class: row.locality_class,
            degraded_state_class: decision.decision_outcome.degraded_state_class,
            summary: decision.surface_report.export_safe_explanation.clone(),
        })
    }

    /// Builds a descriptor for a snippet source.
    pub fn snippet(
        source_descriptor_id: impl Into<String>,
        source_label: impl Into<String>,
        source_ref: impl Into<String>,
        scope_claim_class: RouterScopeClaimClass,
        locality_class: RouterLocalityClass,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            source_descriptor_id: source_descriptor_id.into(),
            source_family: AssistSourceFamily::Snippet,
            source_label: source_label.into(),
            provider_id: None,
            router_decision_ref: None,
            source_ref: Some(source_ref.into()),
            support_class: RouterSupportClass::Authoritative,
            freshness_class: RouterFreshnessClass::AuthoritativeLive,
            scope_claim_class,
            completeness_class: RouterCompletenessClass::CompleteForClaimedScope,
            scope_limit_classes: Vec::new(),
            locality_class,
            degraded_state_class: RouterDegradedStateClass::None,
            summary: summary.into(),
        }
    }

    /// Returns true when consumers must render a visible source label.
    pub fn requires_source_label(&self) -> bool {
        !self.source_label.trim().is_empty()
    }

    /// Returns true when fallback, freshness, or partiality must be disclosed.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.source_family.is_fallback()
            || self.degraded_state_class != RouterDegradedStateClass::None
            || !matches!(self.support_class, RouterSupportClass::Authoritative)
            || self.freshness_class != RouterFreshnessClass::AuthoritativeLive
            || self.completeness_class != RouterCompletenessClass::CompleteForClaimedScope
            || !self.scope_limit_classes.is_empty()
    }
}

/// Completion item role used by editor assist consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionItemKindClass {
    /// Function or callable completion.
    Function,
    /// Method completion.
    Method,
    /// Type, class, or interface completion.
    Type,
    /// Variable, constant, or field completion.
    Value,
    /// Keyword completion.
    Keyword,
    /// Path, import, or module completion.
    Path,
    /// Snippet completion.
    Snippet,
    /// Local-word lexical completion.
    LocalWord,
}

/// Side-effect posture for accepting one completion item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionSideEffectClass {
    /// Acceptance mutates only the current insertion range.
    CurrentRangeOnly,
    /// Acceptance may add edits in the current file and must say so.
    CurrentFileAdditionalEditsNoted,
    /// Acceptance requires preview before broader edits apply.
    PreviewRequiredBeforeAdditionalEdits,
    /// Acceptance is inspect-only and cannot mutate.
    InspectOnlyNoApply,
}

impl CompletionSideEffectClass {
    /// Returns true when accepting the completion requires a preview surface.
    pub const fn requires_preview(self) -> bool {
        matches!(self, Self::PreviewRequiredBeforeAdditionalEdits)
    }
}

/// Command and mutation contract for accepting one completion item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionAcceptanceContract {
    /// Stable command id for accepting the item.
    pub accept_command_id_ref: String,
    /// Stable command id for opening detail or documentation.
    pub detail_command_id_ref: String,
    /// Side-effect posture for acceptance.
    pub side_effect_class: CompletionSideEffectClass,
    /// Preview route required before broad or unsafe edits.
    pub preview_required: bool,
    /// Named undo group to use if the item is accepted.
    pub undo_group_label: String,
    /// Export-safe note about additional edits.
    pub additional_edit_summary: Option<String>,
}

impl CompletionAcceptanceContract {
    /// Returns true when this acceptance path may be applied directly.
    pub const fn can_apply_directly(&self) -> bool {
        !self.preview_required
            && matches!(
                self.side_effect_class,
                CompletionSideEffectClass::CurrentRangeOnly
                    | CompletionSideEffectClass::CurrentFileAdditionalEditsNoted
            )
    }
}

/// Completion item with source labeling and side-effect disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionItemRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub assist_schema_version: AssistSchemaVersion,
    /// Stable completion item id.
    pub completion_item_id: String,
    /// Completion session id that owns this item.
    pub completion_session_id: String,
    /// Display label shown in the completion list.
    pub label: String,
    /// Completion kind.
    pub kind_class: CompletionItemKindClass,
    /// Source and provenance descriptor.
    pub source: AssistSourceDescriptor,
    /// Opaque insertion payload reference.
    pub insert_text_ref: String,
    /// Stable rank within the current list.
    pub rank: u32,
    /// Sort group used before rank.
    pub sort_group: u32,
    /// Acceptance and side-effect contract.
    pub acceptance: CompletionAcceptanceContract,
    /// Redaction posture for export.
    pub redaction_class: RedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl CompletionItemRecord {
    /// Stable record-kind tag for completion items.
    pub const RECORD_KIND: &'static str = "assist_completion_item_record";

    /// Builds a completion item record.
    pub fn new(init: CompletionItemInit) -> Self {
        let source_label = init.source.source_label.clone();
        Self {
            record_kind: Self::RECORD_KIND.into(),
            assist_schema_version: ASSIST_SCHEMA_VERSION,
            completion_item_id: init.completion_item_id,
            completion_session_id: init.completion_session_id,
            label: init.label,
            kind_class: init.kind_class,
            source: init.source,
            insert_text_ref: init.insert_text_ref,
            rank: init.rank,
            sort_group: init.sort_group,
            acceptance: init.acceptance,
            redaction_class: RedactionClass::MetadataSafeDefault,
            captured_at: init.captured_at,
            export_safe_summary: format!("Completion item is source-labeled as {source_label}."),
        }
    }

    /// Returns true when accepting the item requires a preview first.
    pub const fn requires_preview(&self) -> bool {
        self.acceptance.preview_required
    }

    /// Returns true when fallback, source, or side-effect disclosure is needed.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.source.requires_degraded_disclosure() || self.requires_preview()
    }
}

/// Initialization data for [`CompletionItemRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionItemInit {
    /// Stable completion item id.
    pub completion_item_id: String,
    /// Completion session id that owns this item.
    pub completion_session_id: String,
    /// Display label shown in the completion list.
    pub label: String,
    /// Completion kind.
    pub kind_class: CompletionItemKindClass,
    /// Source and provenance descriptor.
    pub source: AssistSourceDescriptor,
    /// Opaque insertion payload reference.
    pub insert_text_ref: String,
    /// Stable rank within the current list.
    pub rank: u32,
    /// Sort group used before rank.
    pub sort_group: u32,
    /// Acceptance and side-effect contract.
    pub acceptance: CompletionAcceptanceContract,
    /// Capture timestamp.
    pub captured_at: String,
}

/// Request for a completion-list snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionListRequest {
    /// Stable completion session id.
    pub completion_session_id: String,
    /// Workspace id covered by the list.
    pub workspace_id: String,
    /// Document ref covered by the list.
    pub document_ref: String,
    /// Language id resolved for the document.
    pub language_id: String,
    /// Anchor ref for the requested caret or range.
    pub request_anchor_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
}

/// Aggregate source counts visible to assist consumers.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct AssistSourceCounts {
    /// Completion items in the list.
    pub completion_total_count: usize,
    /// Completion items sourced from language servers.
    pub lsp_completion_count: usize,
    /// Completion items sourced from lexical or syntax fallback.
    pub fallback_completion_count: usize,
    /// Completion items sourced from snippets.
    pub snippet_completion_count: usize,
    /// Completion items requiring preview before acceptance.
    pub preview_required_count: usize,
    /// Number of active snippet sessions.
    pub active_snippet_session_count: usize,
    /// Number of signature-help cards visible in the snapshot.
    pub signature_help_count: usize,
    /// Number of source labels present across the snapshot.
    pub source_label_count: usize,
    /// Source families present in deterministic order.
    pub source_families: Vec<AssistSourceFamily>,
}

impl AssistSourceCounts {
    /// Builds counts from completion items, signature help, and snippet state.
    pub fn from_parts(
        completion_items: &[CompletionItemRecord],
        signature_help: Option<&SignatureHelpRecord>,
        snippet_session: Option<&SnippetSessionRecord>,
    ) -> Self {
        let mut families = BTreeSet::new();
        let mut counts = Self {
            completion_total_count: completion_items.len(),
            ..Self::default()
        };

        for item in completion_items {
            families.insert(item.source.source_family);
            if item.source.requires_source_label() {
                counts.source_label_count += 1;
            }
            if item.source.source_family.is_language_server() {
                counts.lsp_completion_count += 1;
            }
            if item.source.source_family.is_fallback() {
                counts.fallback_completion_count += 1;
            }
            if item.source.source_family.is_snippet() {
                counts.snippet_completion_count += 1;
            }
            if item.requires_preview() {
                counts.preview_required_count += 1;
            }
        }

        if let Some(signature_help) = signature_help {
            counts.signature_help_count = 1;
            families.insert(signature_help.source.source_family);
            if signature_help.source.requires_source_label() {
                counts.source_label_count += 1;
            }
        }

        if let Some(snippet_session) = snippet_session {
            families.insert(snippet_session.source.source_family);
            if snippet_session.source.requires_source_label() {
                counts.source_label_count += 1;
            }
            if snippet_session.is_active() {
                counts.active_snippet_session_count = 1;
            }
        }

        counts.source_families = families.into_iter().collect();
        counts
    }
}

/// Completion-list snapshot consumed by editor, support, and CLI surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionListSnapshot {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub assist_schema_version: AssistSchemaVersion,
    /// Completion session id.
    pub completion_session_id: String,
    /// Workspace id covered by the list.
    pub workspace_id: String,
    /// Document ref covered by the list.
    pub document_ref: String,
    /// Language id resolved for the document.
    pub language_id: String,
    /// Anchor ref for the requested caret or range.
    pub request_anchor_ref: String,
    /// Ordered completion items.
    pub items: Vec<CompletionItemRecord>,
    /// Source counts for compact surfaces.
    pub source_counts: AssistSourceCounts,
    /// Upstream router decision refs cited by the list.
    pub router_decision_refs: Vec<String>,
    /// Whether fallback, freshness, partiality, or preview disclosure is needed.
    pub disclosure_required: bool,
    /// Redaction posture for export.
    pub redaction_class: RedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Accessible summary for screen readers.
    pub accessibility_summary: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl CompletionListSnapshot {
    /// Stable record-kind tag for completion-list snapshots.
    pub const RECORD_KIND: &'static str = "assist_completion_list_snapshot";

    /// Builds a deterministic completion-list snapshot.
    pub fn from_items(
        request: CompletionListRequest,
        mut items: Vec<CompletionItemRecord>,
    ) -> Self {
        items.sort_by(|left, right| {
            (left.sort_group, left.rank, &left.completion_item_id).cmp(&(
                right.sort_group,
                right.rank,
                &right.completion_item_id,
            ))
        });
        let source_counts = AssistSourceCounts::from_parts(&items, None, None);
        let disclosure_required = items
            .iter()
            .any(CompletionItemRecord::requires_degraded_disclosure);
        let router_decision_refs = router_decision_refs_from_items(&items);
        let item_count = items.len();

        Self {
            record_kind: Self::RECORD_KIND.into(),
            assist_schema_version: ASSIST_SCHEMA_VERSION,
            completion_session_id: request.completion_session_id,
            workspace_id: request.workspace_id,
            document_ref: request.document_ref,
            language_id: request.language_id,
            request_anchor_ref: request.request_anchor_ref,
            items,
            source_counts,
            router_decision_refs,
            disclosure_required,
            redaction_class: RedactionClass::MetadataSafeDefault,
            captured_at: request.captured_at,
            accessibility_summary: format!(
                "{item_count} completion items available with source labels."
            ),
            export_safe_summary: format!(
                "Completion list contains {item_count} source-labeled items."
            ),
        }
    }
}

/// Placement posture for a signature-help card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignaturePlacementClass {
    /// Card stays near the caret without taking focus.
    NonBlockingNearCaret,
    /// Card moves to a side or detail surface because inline placement is unsafe.
    MovedToDetail,
    /// Card is hidden because provider state cannot support it.
    HiddenUnavailable,
}

/// Signature-help card record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignatureHelpRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub assist_schema_version: AssistSchemaVersion,
    /// Stable signature-help id.
    pub signature_help_id: String,
    /// Assistance session id.
    pub assist_session_id: String,
    /// Document ref covered by the card.
    pub document_ref: String,
    /// Language id resolved for the document.
    pub language_id: String,
    /// Anchor ref for the active call site.
    pub invocation_anchor_ref: String,
    /// Source and provenance descriptor.
    pub source: AssistSourceDescriptor,
    /// One-based active signature index.
    pub active_signature_index: u32,
    /// Total signature overload count.
    pub signature_count: u32,
    /// One-based active parameter index.
    pub active_parameter_index: u32,
    /// Total parameter count in the active signature.
    pub parameter_count: u32,
    /// Placement posture for the card.
    pub placement_class: SignaturePlacementClass,
    /// Whether the card avoids focus capture.
    pub non_blocking: bool,
    /// Whether the card remains valid during IME composition.
    pub ime_composition_safe: bool,
    /// Stable dismiss command id.
    pub dismiss_command_id_ref: String,
    /// Redaction posture for export.
    pub redaction_class: RedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Accessible summary for screen readers.
    pub accessibility_label: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl SignatureHelpRecord {
    /// Stable record-kind tag for signature-help records.
    pub const RECORD_KIND: &'static str = "assist_signature_help_record";

    /// Builds a signature-help record.
    pub fn new(init: SignatureHelpInit) -> Self {
        let source_label = init.source.source_label.clone();
        Self {
            record_kind: Self::RECORD_KIND.into(),
            assist_schema_version: ASSIST_SCHEMA_VERSION,
            signature_help_id: init.signature_help_id,
            assist_session_id: init.assist_session_id,
            document_ref: init.document_ref,
            language_id: init.language_id,
            invocation_anchor_ref: init.invocation_anchor_ref,
            source: init.source,
            active_signature_index: init.active_signature_index,
            signature_count: init.signature_count,
            active_parameter_index: init.active_parameter_index,
            parameter_count: init.parameter_count,
            placement_class: init.placement_class,
            non_blocking: init.non_blocking,
            ime_composition_safe: init.ime_composition_safe,
            dismiss_command_id_ref: init.dismiss_command_id_ref,
            redaction_class: RedactionClass::MetadataSafeDefault,
            captured_at: init.captured_at,
            accessibility_label: format!(
                "Signature help from {source_label}; parameter {} of {}.",
                init.active_parameter_index, init.parameter_count
            ),
            export_safe_summary: format!("Signature help is source-labeled as {source_label}."),
        }
    }

    /// Returns true when the card satisfies non-blocking typing-loop rules.
    pub const fn is_typing_loop_safe(&self) -> bool {
        self.non_blocking && self.ime_composition_safe
    }

    /// Returns true when fallback, freshness, or partiality disclosure is needed.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.source.requires_degraded_disclosure()
            || matches!(
                self.placement_class,
                SignaturePlacementClass::HiddenUnavailable
            )
    }
}

/// Initialization data for [`SignatureHelpRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureHelpInit {
    /// Stable signature-help id.
    pub signature_help_id: String,
    /// Assistance session id.
    pub assist_session_id: String,
    /// Document ref covered by the card.
    pub document_ref: String,
    /// Language id resolved for the document.
    pub language_id: String,
    /// Anchor ref for the active call site.
    pub invocation_anchor_ref: String,
    /// Source and provenance descriptor.
    pub source: AssistSourceDescriptor,
    /// One-based active signature index.
    pub active_signature_index: u32,
    /// Total signature overload count.
    pub signature_count: u32,
    /// One-based active parameter index.
    pub active_parameter_index: u32,
    /// Total parameter count in the active signature.
    pub parameter_count: u32,
    /// Placement posture for the card.
    pub placement_class: SignaturePlacementClass,
    /// Whether the card avoids focus capture.
    pub non_blocking: bool,
    /// Whether the card remains valid during IME composition.
    pub ime_composition_safe: bool,
    /// Stable dismiss command id.
    pub dismiss_command_id_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
}

/// Lifecycle state for a snippet session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnippetSessionStateClass {
    /// No snippet session is active.
    Inactive,
    /// Snippet session is active and owns placeholder traversal.
    Active,
    /// Snippet session ended after normal traversal.
    Exited,
    /// Snippet session was cancelled by the user.
    Cancelled,
}

/// Tab-key behavior for a snippet session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnippetTabBehaviorClass {
    /// Tab traverses placeholders only while the session is active.
    TraversePlaceholdersWhileActive,
    /// Tab exits the session when invoked on the final placeholder.
    ExitOnFinalPlaceholder,
    /// Tab passes through because no active snippet session owns it.
    PassThroughOutsideSession,
}

/// Policy for non-snippet keys while a snippet session is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnippetUnrelatedKeyPolicyClass {
    /// Unrelated text input and editor commands pass through.
    PassThroughUnrelatedKeys,
    /// Only explicit snippet traversal and escape commands are intercepted.
    CaptureSnippetNavigationOnly,
}

/// Key intent class routed through snippet-session handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnippetKeyIntentClass {
    /// Tab key.
    Tab,
    /// Shift+Tab key.
    ShiftTab,
    /// Escape key.
    Escape,
    /// Ordinary text input.
    TextInput,
    /// Arrow or navigation key.
    Navigation,
    /// Non-snippet command shortcut.
    CommandShortcut,
    /// Other key intent.
    Other,
}

/// Result class from snippet-session key handling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnippetKeyOutcomeClass {
    /// The key moved to the next placeholder.
    MoveToNextPlaceholder,
    /// The key moved to the previous placeholder.
    MoveToPreviousPlaceholder,
    /// The key exited the snippet session.
    ExitSession,
    /// The key cancelled the snippet session.
    CancelSession,
    /// The key was passed to normal editor handling.
    PassThrough,
}

/// Snippet-session state shown in the editor strip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnippetSessionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub assist_schema_version: AssistSchemaVersion,
    /// Stable snippet-session id.
    pub snippet_session_id: String,
    /// Document ref covered by the session.
    pub document_ref: String,
    /// Language id resolved for the document.
    pub language_id: String,
    /// Source and provenance descriptor.
    pub source: AssistSourceDescriptor,
    /// Current lifecycle state.
    pub state_class: SnippetSessionStateClass,
    /// One-based active placeholder index.
    pub active_placeholder_index: Option<u32>,
    /// Placeholder count in the session.
    pub placeholder_count: u32,
    /// Number of carets participating in the insertion.
    pub selection_count: u32,
    /// Whether multi-cursor traversal is supported.
    pub multi_cursor_compatible: bool,
    /// Tab-key behavior.
    pub tab_behavior_class: SnippetTabBehaviorClass,
    /// Policy for unrelated keys.
    pub unrelated_key_policy_class: SnippetUnrelatedKeyPolicyClass,
    /// Stable command id for next-placeholder traversal.
    pub next_placeholder_command_id_ref: String,
    /// Stable command id for previous-placeholder traversal.
    pub previous_placeholder_command_id_ref: String,
    /// Stable command id for exiting the session.
    pub exit_command_id_ref: String,
    /// Stable command id for cancelling the session.
    pub escape_command_id_ref: String,
    /// Whether the snippet strip must be visible.
    pub visible_strip_required: bool,
    /// Redaction posture for export.
    pub redaction_class: RedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Accessible summary for screen readers.
    pub accessibility_label: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl SnippetSessionRecord {
    /// Stable record-kind tag for snippet-session records.
    pub const RECORD_KIND: &'static str = "assist_snippet_session_record";

    /// Builds a snippet-session record.
    pub fn new(init: SnippetSessionInit) -> Self {
        let source_label = init.source.source_label.clone();
        Self {
            record_kind: Self::RECORD_KIND.into(),
            assist_schema_version: ASSIST_SCHEMA_VERSION,
            snippet_session_id: init.snippet_session_id,
            document_ref: init.document_ref,
            language_id: init.language_id,
            source: init.source,
            state_class: init.state_class,
            active_placeholder_index: init.active_placeholder_index,
            placeholder_count: init.placeholder_count,
            selection_count: init.selection_count,
            multi_cursor_compatible: init.multi_cursor_compatible,
            tab_behavior_class: init.tab_behavior_class,
            unrelated_key_policy_class:
                SnippetUnrelatedKeyPolicyClass::CaptureSnippetNavigationOnly,
            next_placeholder_command_id_ref: init.next_placeholder_command_id_ref,
            previous_placeholder_command_id_ref: init.previous_placeholder_command_id_ref,
            exit_command_id_ref: init.exit_command_id_ref,
            escape_command_id_ref: init.escape_command_id_ref,
            visible_strip_required: init.visible_strip_required,
            redaction_class: RedactionClass::MetadataSafeDefault,
            captured_at: init.captured_at,
            accessibility_label: format!(
                "Snippet session from {source_label}; placeholder {} of {}.",
                init.active_placeholder_index.unwrap_or(0),
                init.placeholder_count
            ),
            export_safe_summary: format!("Snippet session is source-labeled as {source_label}."),
        }
    }

    /// Returns true when placeholder traversal is active.
    pub const fn is_active(&self) -> bool {
        matches!(self.state_class, SnippetSessionStateClass::Active)
    }

    /// Returns true when the snippet strip should be visible.
    pub const fn is_visible(&self) -> bool {
        self.visible_strip_required && self.is_active()
    }

    /// Returns true when escape can leave the session.
    pub fn can_escape(&self) -> bool {
        self.is_active() && !self.escape_command_id_ref.trim().is_empty()
    }

    /// Returns true when Tab is owned by snippet traversal.
    pub const fn captures_tab(&self) -> bool {
        self.is_active() && self.placeholder_count > 0
    }
}

/// Initialization data for [`SnippetSessionRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnippetSessionInit {
    /// Stable snippet-session id.
    pub snippet_session_id: String,
    /// Document ref covered by the session.
    pub document_ref: String,
    /// Language id resolved for the document.
    pub language_id: String,
    /// Source and provenance descriptor.
    pub source: AssistSourceDescriptor,
    /// Current lifecycle state.
    pub state_class: SnippetSessionStateClass,
    /// One-based active placeholder index.
    pub active_placeholder_index: Option<u32>,
    /// Placeholder count in the session.
    pub placeholder_count: u32,
    /// Number of carets participating in the insertion.
    pub selection_count: u32,
    /// Whether multi-cursor traversal is supported.
    pub multi_cursor_compatible: bool,
    /// Tab-key behavior.
    pub tab_behavior_class: SnippetTabBehaviorClass,
    /// Stable command id for next-placeholder traversal.
    pub next_placeholder_command_id_ref: String,
    /// Stable command id for previous-placeholder traversal.
    pub previous_placeholder_command_id_ref: String,
    /// Stable command id for exiting the session.
    pub exit_command_id_ref: String,
    /// Stable command id for cancelling the session.
    pub escape_command_id_ref: String,
    /// Whether the snippet strip must be visible.
    pub visible_strip_required: bool,
    /// Capture timestamp.
    pub captured_at: String,
}

/// Outcome record emitted when snippet-session key handling runs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnippetKeyOutcomeRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub assist_schema_version: AssistSchemaVersion,
    /// Snippet session id that handled or passed through the key.
    pub snippet_session_id: String,
    /// Key intent that was evaluated.
    pub key_intent_class: SnippetKeyIntentClass,
    /// Outcome of key handling.
    pub outcome_class: SnippetKeyOutcomeClass,
    /// Resulting snippet-session state.
    pub resulting_state_class: SnippetSessionStateClass,
    /// Resulting one-based placeholder index.
    pub resulting_placeholder_index: Option<u32>,
    /// Command that handled the key, when snippet handling consumed it.
    pub command_id_ref: Option<String>,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl SnippetKeyOutcomeRecord {
    /// Stable record-kind tag for snippet key outcome records.
    pub const RECORD_KIND: &'static str = "assist_snippet_key_outcome_record";

    fn new(
        snippet_session_id: String,
        key_intent_class: SnippetKeyIntentClass,
        outcome_class: SnippetKeyOutcomeClass,
        session: &SnippetSessionRecord,
        command_id_ref: Option<String>,
    ) -> Self {
        Self {
            record_kind: Self::RECORD_KIND.into(),
            assist_schema_version: ASSIST_SCHEMA_VERSION,
            snippet_session_id,
            key_intent_class,
            outcome_class,
            resulting_state_class: session.state_class,
            resulting_placeholder_index: session.active_placeholder_index,
            command_id_ref,
            export_safe_summary: format!(
                "Snippet key intent {:?} resolved as {:?}.",
                key_intent_class, outcome_class
            ),
        }
    }
}

/// Stateful snippet-session key handler.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnippetSessionController {
    session: SnippetSessionRecord,
}

impl SnippetSessionController {
    /// Builds a controller from a snippet-session record.
    pub fn new(session: SnippetSessionRecord) -> Self {
        Self { session }
    }

    /// Returns the current snippet-session record.
    pub const fn session(&self) -> &SnippetSessionRecord {
        &self.session
    }

    /// Consumes the controller and returns the current snippet-session record.
    pub fn into_session(self) -> SnippetSessionRecord {
        self.session
    }

    /// Applies one key intent to the snippet session.
    pub fn handle_key(&mut self, intent: SnippetKeyIntentClass) -> SnippetKeyOutcomeRecord {
        let session_id = self.session.snippet_session_id.clone();
        if !self.session.is_active() {
            return SnippetKeyOutcomeRecord::new(
                session_id,
                intent,
                SnippetKeyOutcomeClass::PassThrough,
                &self.session,
                None,
            );
        }

        match intent {
            SnippetKeyIntentClass::Tab if self.session.captures_tab() => {
                let current = self.session.active_placeholder_index.unwrap_or(1);
                if current < self.session.placeholder_count {
                    self.session.active_placeholder_index = Some(current + 1);
                    SnippetKeyOutcomeRecord::new(
                        session_id,
                        intent,
                        SnippetKeyOutcomeClass::MoveToNextPlaceholder,
                        &self.session,
                        Some(self.session.next_placeholder_command_id_ref.clone()),
                    )
                } else {
                    self.session.state_class = SnippetSessionStateClass::Exited;
                    self.session.active_placeholder_index = None;
                    SnippetKeyOutcomeRecord::new(
                        session_id,
                        intent,
                        SnippetKeyOutcomeClass::ExitSession,
                        &self.session,
                        Some(self.session.exit_command_id_ref.clone()),
                    )
                }
            }
            SnippetKeyIntentClass::ShiftTab if self.session.captures_tab() => {
                let current = self.session.active_placeholder_index.unwrap_or(1);
                if current > 1 {
                    self.session.active_placeholder_index = Some(current - 1);
                    SnippetKeyOutcomeRecord::new(
                        session_id,
                        intent,
                        SnippetKeyOutcomeClass::MoveToPreviousPlaceholder,
                        &self.session,
                        Some(self.session.previous_placeholder_command_id_ref.clone()),
                    )
                } else {
                    SnippetKeyOutcomeRecord::new(
                        session_id,
                        intent,
                        SnippetKeyOutcomeClass::PassThrough,
                        &self.session,
                        None,
                    )
                }
            }
            SnippetKeyIntentClass::Escape if self.session.can_escape() => {
                self.session.state_class = SnippetSessionStateClass::Cancelled;
                self.session.active_placeholder_index = None;
                SnippetKeyOutcomeRecord::new(
                    session_id,
                    intent,
                    SnippetKeyOutcomeClass::CancelSession,
                    &self.session,
                    Some(self.session.escape_command_id_ref.clone()),
                )
            }
            SnippetKeyIntentClass::TextInput
            | SnippetKeyIntentClass::Navigation
            | SnippetKeyIntentClass::CommandShortcut
            | SnippetKeyIntentClass::Other
            | SnippetKeyIntentClass::Tab
            | SnippetKeyIntentClass::ShiftTab
            | SnippetKeyIntentClass::Escape => SnippetKeyOutcomeRecord::new(
                session_id,
                intent,
                SnippetKeyOutcomeClass::PassThrough,
                &self.session,
                None,
            ),
        }
    }
}

/// Overall state class for the editor assist surface snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistSurfaceStateClass {
    /// Assist surface is available with source labels.
    Available,
    /// Assist surface is available but contains fallback or degraded rows.
    AvailableDegraded,
    /// Assist surface has no current assistance to show.
    Empty,
}

/// Request for a combined assist surface snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssistSurfaceSnapshotRequest {
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Workspace id covered by the snapshot.
    pub workspace_id: String,
    /// Document ref covered by the snapshot.
    pub document_ref: String,
    /// Language id resolved for the document.
    pub language_id: String,
    /// Completion session id to project.
    pub completion_session_id: String,
    /// Anchor ref for completion projection.
    pub request_anchor_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
}

/// Combined editor assist surface snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistSurfaceSnapshot {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub assist_schema_version: AssistSchemaVersion,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Workspace id covered by the snapshot.
    pub workspace_id: String,
    /// Document ref covered by the snapshot.
    pub document_ref: String,
    /// Language id resolved for the document.
    pub language_id: String,
    /// Overall surface state.
    pub state_class: AssistSurfaceStateClass,
    /// Completion-list projection.
    pub completion_list: CompletionListSnapshot,
    /// Signature-help card, when visible.
    pub signature_help: Option<SignatureHelpRecord>,
    /// Snippet-session strip, when visible.
    pub snippet_session: Option<SnippetSessionRecord>,
    /// Aggregate source counts for compact consumers.
    pub source_counts: AssistSourceCounts,
    /// Whether degraded or preview disclosure is required.
    pub disclosure_required: bool,
    /// Redaction posture for export.
    pub redaction_class: RedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Accessible summary for screen readers.
    pub accessibility_summary: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl AssistSurfaceSnapshot {
    /// Stable record-kind tag for combined assist surface snapshots.
    pub const RECORD_KIND: &'static str = "assist_surface_snapshot_record";
}

/// In-memory store for the current editor assist session.
#[derive(Debug, Clone, Default)]
pub struct AssistSessionStore {
    completion_items: BTreeMap<String, CompletionItemRecord>,
    signature_help: Option<SignatureHelpRecord>,
    snippet_session: Option<SnippetSessionRecord>,
}

impl AssistSessionStore {
    /// Builds an empty assist session store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Publishes or replaces one completion item.
    pub fn publish_completion_item(&mut self, item: CompletionItemRecord) {
        self.completion_items
            .insert(item.completion_item_id.clone(), item);
    }

    /// Sets the current signature-help card.
    pub fn set_signature_help(&mut self, signature_help: SignatureHelpRecord) {
        self.signature_help = Some(signature_help);
    }

    /// Sets the current snippet session.
    pub fn set_snippet_session(&mut self, snippet_session: SnippetSessionRecord) {
        self.snippet_session = Some(snippet_session);
    }

    /// Returns true when the store has no current assist state.
    pub fn is_empty(&self) -> bool {
        self.completion_items.is_empty()
            && self.signature_help.is_none()
            && self.snippet_session.is_none()
    }

    /// Builds a completion-list snapshot.
    pub fn completion_list(&self, request: CompletionListRequest) -> CompletionListSnapshot {
        CompletionListSnapshot::from_items(
            request,
            self.completion_items.values().cloned().collect(),
        )
    }

    /// Builds a combined assist surface snapshot.
    pub fn surface_snapshot(&self, request: AssistSurfaceSnapshotRequest) -> AssistSurfaceSnapshot {
        let completion_list = self.completion_list(CompletionListRequest {
            completion_session_id: request.completion_session_id,
            workspace_id: request.workspace_id.clone(),
            document_ref: request.document_ref.clone(),
            language_id: request.language_id.clone(),
            request_anchor_ref: request.request_anchor_ref,
            captured_at: request.captured_at.clone(),
        });
        let signature_help = self.signature_help.clone();
        let snippet_session = self
            .snippet_session
            .as_ref()
            .filter(|row| row.is_visible())
            .cloned();
        let source_counts = AssistSourceCounts::from_parts(
            &completion_list.items,
            signature_help.as_ref(),
            snippet_session.as_ref(),
        );
        let disclosure_required = completion_list.disclosure_required
            || signature_help
                .as_ref()
                .is_some_and(SignatureHelpRecord::requires_degraded_disclosure);
        let state_class = if source_counts.completion_total_count == 0
            && source_counts.signature_help_count == 0
            && source_counts.active_snippet_session_count == 0
        {
            AssistSurfaceStateClass::Empty
        } else if disclosure_required {
            AssistSurfaceStateClass::AvailableDegraded
        } else {
            AssistSurfaceStateClass::Available
        };
        let accessibility_summary = format!(
            "{} completion items, {} signature cards, and {} active snippet sessions are source-labeled.",
            source_counts.completion_total_count,
            source_counts.signature_help_count,
            source_counts.active_snippet_session_count
        );

        AssistSurfaceSnapshot {
            record_kind: AssistSurfaceSnapshot::RECORD_KIND.into(),
            assist_schema_version: ASSIST_SCHEMA_VERSION,
            snapshot_id: request.snapshot_id,
            workspace_id: request.workspace_id,
            document_ref: request.document_ref,
            language_id: request.language_id,
            state_class,
            completion_list,
            signature_help,
            snippet_session,
            source_counts,
            disclosure_required,
            redaction_class: RedactionClass::MetadataSafeDefault,
            captured_at: request.captured_at,
            accessibility_summary,
            export_safe_summary:
                "Editor assist surface preserves completion, signature, and snippet source labels."
                    .into(),
        }
    }
}

fn completeness_for_selected_source(
    provider_kind: RouterProviderKind,
    decision: &RouterDecisionRecord,
) -> RouterCompletenessClass {
    if provider_kind == RouterProviderKind::SyntaxParser
        || decision.decision_outcome.degraded_state_class != RouterDegradedStateClass::None
    {
        RouterCompletenessClass::PartialForClaimedScope
    } else {
        RouterCompletenessClass::CompleteForClaimedScope
    }
}

fn scope_limits_for_selected_source(
    provider_kind: RouterProviderKind,
    decision: &RouterDecisionRecord,
) -> Vec<ScopeLimitClass> {
    if provider_kind == RouterProviderKind::SyntaxParser {
        vec![ScopeLimitClass::SingleFileOnly]
    } else if decision.decision_outcome.degraded_state_class != RouterDegradedStateClass::None {
        vec![ScopeLimitClass::ActiveWorksetOnly]
    } else {
        Vec::new()
    }
}

fn router_decision_refs_from_items(items: &[CompletionItemRecord]) -> Vec<String> {
    let mut refs = BTreeSet::new();
    for item in items {
        if let Some(router_decision_ref) = &item.source.router_decision_ref {
            refs.insert(router_decision_ref.clone());
        }
    }
    refs.into_iter().collect()
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, ':' | '_' | '-') {
                ch
            } else {
                '-'
            }
        })
        .collect()
}
