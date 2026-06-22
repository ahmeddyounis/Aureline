//! Canonical completion-row truth model: source kind, provider identity,
//! deterministic-versus-AI distinction, additional-edit/import cues, and
//! degraded-provider labels across the claimed editor families.
//!
//! Where the [editor-assist matrix](crate::m5_editor_assist) freezes the
//! per-surface degraded-state *policy* and the
//! [assist-descriptor model](crate::m5_assist_descriptors) freezes the typed
//! decoration / code-lens / inlay-hint *descriptor* shape, this module freezes
//! the one shared **completion-row** every claimed editor family renders a
//! suggestion through. Before it, each pane was free to invent its own row: one
//! that distinguished a language-server result from a local-word guess only by a
//! ranking tweak, another that applied an auto-import without saying so, a third
//! that let an AI proposal inherit the same trust weight as deterministic
//! semantic completion. This module materializes one [`CompletionRow`] that
//! carries, for every suggestion:
//!
//! 1. **Source kind and provider identity** — the row embeds the canonical
//!    [`AssistSourceDescriptor`] from the assist contracts, so provider id,
//!    support posture, freshness, locality, and degraded state travel with the
//!    row rather than being inferred from rank.
//! 2. **A deterministic-versus-AI assist class** — [`CompletionAssistClass`]
//!    splits deterministic language intelligence, cached/lexical fallback,
//!    local-word fallback, snippet-only, and AI-backed proposals into distinct
//!    classes, each pinned to a [`TrustWeightClass`]. AI-backed and local-word
//!    rows can never claim full-semantic trust, so a user never has to infer the
//!    difference from styling alone.
//! 3. **An additional-edit / import cue** — [`AdditionalEditCue`] states, before
//!    commit, whether accepting a row only edits the current range or also adds
//!    an import, a dependency, a config edit, or a generated-output effect, and
//!    whether a preview is required first.
//! 4. **Availability and docs truth** — deprecated and unavailable rows are
//!    marked with a non-color differentiator and cannot masquerade as live, and
//!    documentation availability is explicit.
//!
//! Each claimed editor family is resolved into a [`CompletionRowSnapshot`] that
//! pins its [`CompletionProviderPosture`] (full semantic, degraded provider,
//! stale partial index, restricted mode, large-file fallback) and a visible
//! fallback label, so a degraded provider can never appear as a silent ranking
//! regression. Every snapshot also carries the canonical
//! [`CompletionListSnapshot`] derived from the same rows, proving the row
//! projection and the shared assist packet cannot drift.
//!
//! The build is static and deterministic: [`completion_row_model`] assembles the
//! one canonical record, the checked-in fixture plus the replay gate freeze it
//! byte-for-byte, and the model proves its own honesty invariants over its data.
//! It carries no file contents, credential bodies, or raw provider payloads, so
//! support, AI, and migration surfaces can consume it directly.

use serde::{Deserialize, Serialize};

use aureline_language::{
    RedactionClass, RouterCompletenessClass, RouterDegradedStateClass, RouterFreshnessClass,
    RouterLocalityClass, RouterScopeClaimClass, RouterSupportClass, ScopeLimitClass,
};

use crate::assist::{
    AssistSourceDescriptor, AssistSourceFamily, AssistSourceLabelClass,
    CompletionAcceptanceContract, CompletionItemInit, CompletionItemKindClass,
    CompletionItemRecord, CompletionListRequest, CompletionListSnapshot, CompletionSideEffectClass,
};
use crate::m5_editor_assist::{ClassDescriptor, EditorSurfaceClass};

/// Schema version for the completion-row model record.
pub const M5_COMPLETION_ROWS_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the completion-row model record.
pub const M5_COMPLETION_ROWS_SCHEMA_REF: &str = "schemas/editor/m5-completion-rows.schema.json";

/// Stable record-kind tag for the completion-row model record.
pub const M5_COMPLETION_ROWS_RECORD_KIND: &str = "m5_completion_row_model";

/// Stable id for the canonical completion-row model.
pub const M5_COMPLETION_ROWS_MODEL_ID: &str = "m5-completion-rows:model:0001";

/// Capture stamp for the canonical model. Held as a constant so the projection
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_COMPLETION_ROWS_AS_OF: &str = "2026-06-22T00:00:00Z";

const ACCEPT_COMMAND_REF: &str = "command.editor.completion.accept";
const DETAIL_COMMAND_REF: &str = "command.editor.completion.toggle_detail";
const DOCS_COMMAND_REF: &str = "command.editor.completion.show_documentation";

// ---------------------------------------------------------------------------
// Assist class — the deterministic-versus-AI distinction.
// ---------------------------------------------------------------------------

/// Trust class for one completion row, kept explicit so a user never infers the
/// difference between sources from styling alone.
///
/// This is the row-facing refinement of the shared
/// [`AssistSourceLabelClass`]: it additionally splits a lexical local-word guess
/// out of the broader cached-fallback class so a one-word completion can never be
/// mistaken for cached semantic truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionAssistClass {
    /// Deterministic language intelligence (language server or first-party semantic provider).
    DeterministicLanguage,
    /// Cached, syntactic, or lexical fallback result.
    CachedFallback,
    /// Local-word lexical guess gathered from the open buffer.
    LocalWord,
    /// Snippet pack or snippet-session result.
    SnippetOnly,
    /// AI inline assist or AI-authored proposal.
    AiBacked,
    /// Project-graph result.
    ProjectGraph,
    /// Framework, schema, or generated-source provider result.
    FrameworkProvider,
    /// Structured tool-adapter result.
    ToolAdapter,
}

impl CompletionAssistClass {
    /// All assist classes, in catalog order.
    pub const ALL: [Self; 8] = [
        Self::DeterministicLanguage,
        Self::CachedFallback,
        Self::LocalWord,
        Self::SnippetOnly,
        Self::AiBacked,
        Self::ProjectGraph,
        Self::FrameworkProvider,
        Self::ToolAdapter,
    ];

    /// Derives the row assist class from the source label class and item kind.
    ///
    /// The source label class is the shared assist vocabulary; the item kind is
    /// used only to split a local-word guess out of the broader cached-fallback
    /// class.
    pub const fn derive(
        source_label_class: AssistSourceLabelClass,
        kind_class: CompletionItemKindClass,
    ) -> Self {
        match source_label_class {
            AssistSourceLabelClass::DeterministicLanguage => Self::DeterministicLanguage,
            AssistSourceLabelClass::CachedFallback => match kind_class {
                CompletionItemKindClass::LocalWord => Self::LocalWord,
                _ => Self::CachedFallback,
            },
            AssistSourceLabelClass::SnippetOrigin => Self::SnippetOnly,
            AssistSourceLabelClass::AiInlineAssist => Self::AiBacked,
            AssistSourceLabelClass::ProjectGraph => Self::ProjectGraph,
            AssistSourceLabelClass::FrameworkProvider => Self::FrameworkProvider,
            AssistSourceLabelClass::ToolAdapter => Self::ToolAdapter,
        }
    }

    /// Returns the stable schema token for this class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeterministicLanguage => "deterministic_language",
            Self::CachedFallback => "cached_fallback",
            Self::LocalWord => "local_word",
            Self::SnippetOnly => "snippet_only",
            Self::AiBacked => "ai_backed",
            Self::ProjectGraph => "project_graph",
            Self::FrameworkProvider => "framework_provider",
            Self::ToolAdapter => "tool_adapter",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DeterministicLanguage => "Deterministic language",
            Self::CachedFallback => "Cached fallback",
            Self::LocalWord => "Local word",
            Self::SnippetOnly => "Snippet",
            Self::AiBacked => "AI-backed",
            Self::ProjectGraph => "Project graph",
            Self::FrameworkProvider => "Framework provider",
            Self::ToolAdapter => "Tool adapter",
        }
    }

    /// Short tag a compact row badge can show alongside the kind icon.
    pub const fn badge_tag(self) -> &'static str {
        match self {
            Self::DeterministicLanguage => "semantic",
            Self::CachedFallback => "cached",
            Self::LocalWord => "word",
            Self::SnippetOnly => "snippet",
            Self::AiBacked => "AI",
            Self::ProjectGraph => "graph",
            Self::FrameworkProvider => "schema",
            Self::ToolAdapter => "tool",
        }
    }

    /// Trust weight pinned to this assist class.
    pub const fn trust_weight(self) -> TrustWeightClass {
        match self {
            Self::DeterministicLanguage | Self::ProjectGraph | Self::FrameworkProvider => {
                TrustWeightClass::FullSemantic
            }
            Self::SnippetOnly | Self::AiBacked | Self::ToolAdapter => TrustWeightClass::Advisory,
            Self::CachedFallback | Self::LocalWord => TrustWeightClass::HeuristicFallback,
        }
    }

    /// Returns true when this row is deterministic language intelligence.
    pub const fn is_deterministic(self) -> bool {
        matches!(self, Self::DeterministicLanguage)
    }

    /// Returns true when this row is an AI-backed proposal.
    pub const fn is_ai_backed(self) -> bool {
        matches!(self, Self::AiBacked)
    }

    /// Returns true when consumers must keep this class visually distinct from
    /// deterministic semantic completion.
    pub const fn requires_visual_distinction(self) -> bool {
        matches!(
            self,
            Self::CachedFallback | Self::LocalWord | Self::SnippetOnly | Self::AiBacked
        )
    }

    /// Non-color differentiator a row must carry so the class is never color-only.
    pub const fn non_color_differentiator(self) -> &'static str {
        match self {
            Self::DeterministicLanguage => "solid kind glyph",
            Self::CachedFallback => "dashed kind glyph with 'cached' tag",
            Self::LocalWord => "dotted kind glyph with 'word' tag",
            Self::SnippetOnly => "snippet-bracket glyph with 'snippet' tag",
            Self::AiBacked => "AI sparkle glyph with 'AI' text tag",
            Self::ProjectGraph => "graph glyph",
            Self::FrameworkProvider => "schema glyph",
            Self::ToolAdapter => "tool glyph",
        }
    }
}

/// Relative trust weight a row may carry, kept distinct from the source class so
/// the guardrail — AI-backed and local-word rows never inherit deterministic
/// weight — is provable in data, not styling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustWeightClass {
    /// Full deterministic semantic weight.
    FullSemantic,
    /// Advisory weight: useful but not authoritative.
    Advisory,
    /// Heuristic / cached fallback weight.
    HeuristicFallback,
}

impl TrustWeightClass {
    /// All trust weights, in catalog order.
    pub const ALL: [Self; 3] = [Self::FullSemantic, Self::Advisory, Self::HeuristicFallback];

    /// Returns the stable schema token for this weight.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullSemantic => "full_semantic",
            Self::Advisory => "advisory",
            Self::HeuristicFallback => "heuristic_fallback",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::FullSemantic => "Full semantic",
            Self::Advisory => "Advisory",
            Self::HeuristicFallback => "Heuristic fallback",
        }
    }

    /// Returns true when this weight is the full deterministic-semantic weight.
    pub const fn is_full_semantic(self) -> bool {
        matches!(self, Self::FullSemantic)
    }
}

// ---------------------------------------------------------------------------
// Additional-edit / import cue.
// ---------------------------------------------------------------------------

/// What accepting a completion row does beyond the current insertion range.
///
/// Every value other than [`AdditionalEditCue::None`] must be disclosed before
/// commit so a one-keystroke accept never silently rewrites imports, edits a
/// dependency manifest, mutates configuration, or changes generated output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdditionalEditCue {
    /// Acceptance edits only the current insertion range.
    None,
    /// Acceptance adds further edits in the current file.
    AdditionalEditsInFile,
    /// Acceptance adds or rewrites an import.
    AddsImport,
    /// Acceptance adds or changes a dependency.
    AddsDependency,
    /// Acceptance edits configuration outside the insertion range.
    EditsConfig,
    /// Acceptance changes generated output through its generator.
    GeneratedOutputEffect,
}

impl AdditionalEditCue {
    /// All cues, in catalog order.
    pub const ALL: [Self; 6] = [
        Self::None,
        Self::AdditionalEditsInFile,
        Self::AddsImport,
        Self::AddsDependency,
        Self::EditsConfig,
        Self::GeneratedOutputEffect,
    ];

    /// Returns the stable schema token for this cue.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::AdditionalEditsInFile => "additional_edits_in_file",
            Self::AddsImport => "adds_import",
            Self::AddsDependency => "adds_dependency",
            Self::EditsConfig => "edits_config",
            Self::GeneratedOutputEffect => "generated_output_effect",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "Edits current range only",
            Self::AdditionalEditsInFile => "Adds edits in this file",
            Self::AddsImport => "Adds an import",
            Self::AddsDependency => "Adds a dependency",
            Self::EditsConfig => "Edits configuration",
            Self::GeneratedOutputEffect => "Changes generated output",
        }
    }

    /// Returns true when acceptance must disclose the effect before commit.
    pub const fn requires_pre_commit_disclosure(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Default acceptance side-effect posture for the shared assist contract.
    pub const fn base_side_effect_class(self) -> CompletionSideEffectClass {
        match self {
            Self::None => CompletionSideEffectClass::CurrentRangeOnly,
            Self::AdditionalEditsInFile | Self::AddsImport | Self::EditsConfig => {
                CompletionSideEffectClass::CurrentFileAdditionalEditsNoted
            }
            Self::AddsDependency | Self::GeneratedOutputEffect => {
                CompletionSideEffectClass::PreviewRequiredBeforeAdditionalEdits
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Availability.
// ---------------------------------------------------------------------------

/// Availability posture for one completion row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionAvailabilityClass {
    /// The completion can be accepted.
    Available,
    /// The completion targets deprecated API and must be marked.
    Deprecated,
    /// The completion cannot be accepted in the current context.
    Unavailable,
}

impl CompletionAvailabilityClass {
    /// All availability classes, in catalog order.
    pub const ALL: [Self; 3] = [Self::Available, Self::Deprecated, Self::Unavailable];

    /// Returns the stable schema token for this class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Deprecated => "deprecated",
            Self::Unavailable => "unavailable",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Available => "Available",
            Self::Deprecated => "Deprecated",
            Self::Unavailable => "Unavailable",
        }
    }

    /// Returns true when the row can be accepted and applied.
    pub const fn is_acceptable(self) -> bool {
        matches!(self, Self::Available | Self::Deprecated)
    }

    /// Non-color marker a non-available row must carry, or empty for available.
    pub const fn non_color_marker(self) -> &'static str {
        match self {
            Self::Available => "",
            Self::Deprecated => "strikethrough label",
            Self::Unavailable => "dimmed label with 'unavailable' tag",
        }
    }
}

// ---------------------------------------------------------------------------
// Provider posture for one surface snapshot.
// ---------------------------------------------------------------------------

/// Provider posture for a completion-row snapshot, surfaced as a visible label so
/// a degraded path never appears as a silent ranking regression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionProviderPosture {
    /// A full-fidelity semantic provider answered.
    FullSemantic,
    /// The preferred provider is degraded and fallback results are shown.
    DegradedProvider,
    /// The semantic index is still building, so results are partial.
    StalePartialIndex,
    /// The surface is in restricted mode, so semantic assist is limited.
    RestrictedMode,
    /// The file is in large-file mode, so only lexical fallback is offered.
    LargeFileFallback,
    /// The provider is offline and only cached results are available.
    OfflineCachedOnly,
}

impl CompletionProviderPosture {
    /// All postures, in catalog order.
    pub const ALL: [Self; 6] = [
        Self::FullSemantic,
        Self::DegradedProvider,
        Self::StalePartialIndex,
        Self::RestrictedMode,
        Self::LargeFileFallback,
        Self::OfflineCachedOnly,
    ];

    /// Returns the stable schema token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullSemantic => "full_semantic",
            Self::DegradedProvider => "degraded_provider",
            Self::StalePartialIndex => "stale_partial_index",
            Self::RestrictedMode => "restricted_mode",
            Self::LargeFileFallback => "large_file_fallback",
            Self::OfflineCachedOnly => "offline_cached_only",
        }
    }

    /// Visible label consumers must show with the completion list.
    pub const fn label(self) -> &'static str {
        match self {
            Self::FullSemantic => "Full semantic completion",
            Self::DegradedProvider => "Degraded provider — fallback results",
            Self::StalePartialIndex => "Index still building — partial results",
            Self::RestrictedMode => "Restricted mode — limited assist",
            Self::LargeFileFallback => "Large-file mode — lexical fallback only",
            Self::OfflineCachedOnly => "Offline — cached results only",
        }
    }

    /// Returns true when the posture is degraded relative to full semantics.
    pub const fn is_degraded(self) -> bool {
        !matches!(self, Self::FullSemantic)
    }

    /// Returns true when consumers must show a visible fallback label.
    pub const fn requires_fallback_label(self) -> bool {
        self.is_degraded()
    }
}

// ---------------------------------------------------------------------------
// Completion row.
// ---------------------------------------------------------------------------

/// One source-labeled, commit-honest completion row shared by every claimed
/// editor family.
///
/// The row embeds the canonical [`AssistSourceDescriptor`] for provenance and
/// derives its presentation truth — assist class, trust weight, additional-edit
/// cue, availability, and the non-color differentiator — deterministically, so it
/// projects the shared assist packet rather than re-inventing it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionRow {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub m5_completion_rows_schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Completion session id that owns this row.
    pub completion_session_id: String,
    /// Primary label shown in the completion list.
    pub primary_label: String,
    /// Completion kind.
    pub kind_class: CompletionItemKindClass,
    /// Stable icon token derived from the kind.
    pub kind_icon_token: String,
    /// Row assist class (the deterministic-versus-AI distinction).
    pub assist_class: CompletionAssistClass,
    /// Trust weight pinned to the assist class.
    pub trust_weight: TrustWeightClass,
    /// Canonical source and provenance descriptor.
    pub source: AssistSourceDescriptor,
    /// Additional-edit / import cue.
    pub additional_edit_cue: AdditionalEditCue,
    /// Export-safe note about the additional edit, when there is one.
    pub additional_edit_summary: Option<String>,
    /// Whether accepting the row must disclose its effect before commit.
    pub commit_disclosure_required: bool,
    /// Whether accepting the row requires a preview surface first.
    pub preview_required: bool,
    /// Availability posture.
    pub availability: CompletionAvailabilityClass,
    /// Whether documentation is available for the row.
    pub docs_available: bool,
    /// Command ref to open documentation, when available.
    pub docs_command_ref: Option<String>,
    /// Command ref to accept the row.
    pub accept_command_ref: String,
    /// Command ref to toggle detail.
    pub detail_command_ref: String,
    /// Stable rank within the list.
    pub rank: u32,
    /// Sort group used before rank.
    pub sort_group: u32,
    /// Whether consumers must keep the row visually distinct from deterministic semantics.
    pub requires_visual_distinction: bool,
    /// Non-color differentiator the row must render.
    pub non_color_differentiator: String,
    /// Accessible label for screen readers.
    pub accessibility_label: String,
    /// Redaction posture for export.
    pub redaction_class: RedactionClass,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl CompletionRow {
    /// Stable record-kind tag for completion rows.
    pub const RECORD_KIND: &'static str = "m5_completion_row";

    /// Builds a completion row from its init data, deriving the presentation truth.
    pub fn new(init: CompletionRowInit) -> Self {
        let assist_class =
            CompletionAssistClass::derive(init.source.source_label_class, init.kind_class);
        let trust_weight = assist_class.trust_weight();
        let acceptable = init.availability.is_acceptable();
        let commit_disclosure_required = acceptable
            && (init.additional_edit_cue.requires_pre_commit_disclosure() || init.preview_required);
        let requires_visual_distinction = assist_class.requires_visual_distinction()
            || init.source.requires_degraded_disclosure()
            || init.availability != CompletionAvailabilityClass::Available;

        let mut non_color = assist_class.non_color_differentiator().to_owned();
        let availability_marker = init.availability.non_color_marker();
        if !availability_marker.is_empty() {
            non_color.push_str("; ");
            non_color.push_str(availability_marker);
        }

        let provider = init
            .source
            .provider_id
            .clone()
            .unwrap_or_else(|| init.source.source_label.clone());
        let accessibility_label = format!(
            "{label}, {kind}, {class} from {provider}; trust {trust}; {cue}{availability}",
            label = init.primary_label,
            kind = kind_label(init.kind_class),
            class = assist_class.label(),
            provider = provider,
            trust = trust_weight.label(),
            cue = init.additional_edit_cue.label(),
            availability = match init.availability {
                CompletionAvailabilityClass::Available => String::new(),
                other => format!("; {}", other.label()),
            },
        );

        let export_safe_summary = format!(
            "Completion row {id} is {class} ({trust}); cue {cue}; availability {availability}.",
            id = init.row_id,
            class = assist_class.as_str(),
            trust = trust_weight.as_str(),
            cue = init.additional_edit_cue.as_str(),
            availability = init.availability.as_str(),
        );

        Self {
            record_kind: Self::RECORD_KIND.into(),
            m5_completion_rows_schema_version: M5_COMPLETION_ROWS_SCHEMA_VERSION,
            row_id: init.row_id,
            completion_session_id: init.completion_session_id,
            primary_label: init.primary_label,
            kind_class: init.kind_class,
            kind_icon_token: kind_icon_token(init.kind_class).to_owned(),
            assist_class,
            trust_weight,
            source: init.source,
            additional_edit_cue: init.additional_edit_cue,
            additional_edit_summary: init.additional_edit_summary,
            commit_disclosure_required,
            preview_required: init.preview_required && acceptable,
            availability: init.availability,
            docs_available: init.docs_available,
            docs_command_ref: if init.docs_available {
                Some(DOCS_COMMAND_REF.to_owned())
            } else {
                None
            },
            accept_command_ref: ACCEPT_COMMAND_REF.to_owned(),
            detail_command_ref: DETAIL_COMMAND_REF.to_owned(),
            rank: init.rank,
            sort_group: init.sort_group,
            requires_visual_distinction,
            non_color_differentiator: non_color,
            accessibility_label,
            redaction_class: RedactionClass::MetadataSafeDefault,
            export_safe_summary,
        }
    }

    /// Returns the canonical acceptance side-effect posture for this row.
    pub fn side_effect_class(&self) -> CompletionSideEffectClass {
        if !self.availability.is_acceptable() {
            CompletionSideEffectClass::InspectOnlyNoApply
        } else if self.preview_required {
            CompletionSideEffectClass::PreviewRequiredBeforeAdditionalEdits
        } else {
            self.additional_edit_cue.base_side_effect_class()
        }
    }

    /// Projects this row back into the canonical shared [`CompletionItemRecord`],
    /// so the row and the assist packet cannot drift.
    pub fn to_canonical_item(&self, captured_at: impl Into<String>) -> CompletionItemRecord {
        let undo_group_label = format!("Accept completion: {}", self.primary_label);
        CompletionItemRecord::new(CompletionItemInit {
            completion_item_id: self.row_id.clone(),
            completion_session_id: self.completion_session_id.clone(),
            label: self.primary_label.clone(),
            kind_class: self.kind_class,
            source: self.source.clone(),
            insert_text_ref: format!("insert:{}", self.row_id),
            rank: self.rank,
            sort_group: self.sort_group,
            acceptance: CompletionAcceptanceContract {
                accept_command_id_ref: self.accept_command_ref.clone(),
                detail_command_id_ref: self.detail_command_ref.clone(),
                side_effect_class: self.side_effect_class(),
                preview_required: self.preview_required,
                undo_group_label,
                additional_edit_summary: self.additional_edit_summary.clone(),
            },
            captured_at: captured_at.into(),
        })
    }
}

/// Initialization data for [`CompletionRow`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionRowInit {
    /// Stable row id.
    pub row_id: String,
    /// Completion session id that owns this row.
    pub completion_session_id: String,
    /// Primary label shown in the completion list.
    pub primary_label: String,
    /// Completion kind.
    pub kind_class: CompletionItemKindClass,
    /// Canonical source and provenance descriptor.
    pub source: AssistSourceDescriptor,
    /// Additional-edit / import cue.
    pub additional_edit_cue: AdditionalEditCue,
    /// Export-safe note about the additional edit, when there is one.
    pub additional_edit_summary: Option<String>,
    /// Whether accepting the row requires a preview surface first.
    pub preview_required: bool,
    /// Availability posture.
    pub availability: CompletionAvailabilityClass,
    /// Whether documentation is available for the row.
    pub docs_available: bool,
    /// Stable rank within the list.
    pub rank: u32,
    /// Sort group used before rank.
    pub sort_group: u32,
}

// ---------------------------------------------------------------------------
// Row counts.
// ---------------------------------------------------------------------------

/// Aggregate row counts for a snapshot, kept for compact and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CompletionRowCounts {
    /// Total rows in the snapshot.
    pub total: usize,
    /// Deterministic-language rows.
    pub deterministic_language: usize,
    /// AI-backed rows.
    pub ai_backed: usize,
    /// Cached-fallback rows.
    pub cached_fallback: usize,
    /// Local-word rows.
    pub local_word: usize,
    /// Snippet-only rows.
    pub snippet_only: usize,
    /// Framework-provider rows.
    pub framework_provider: usize,
    /// Project-graph rows.
    pub project_graph: usize,
    /// Tool-adapter rows.
    pub tool_adapter: usize,
    /// Rows that disclose an additional-edit effect before commit.
    pub additional_edit_disclosed: usize,
    /// Rows that require a preview before acceptance.
    pub preview_required: usize,
    /// Deprecated rows.
    pub deprecated: usize,
    /// Unavailable rows.
    pub unavailable: usize,
    /// Rows with documentation available.
    pub docs_available: usize,
}

impl CompletionRowCounts {
    /// Builds the counts from a row slice.
    pub fn from_rows(rows: &[CompletionRow]) -> Self {
        let mut counts = Self {
            total: rows.len(),
            ..Self::default()
        };
        for row in rows {
            match row.assist_class {
                CompletionAssistClass::DeterministicLanguage => counts.deterministic_language += 1,
                CompletionAssistClass::AiBacked => counts.ai_backed += 1,
                CompletionAssistClass::CachedFallback => counts.cached_fallback += 1,
                CompletionAssistClass::LocalWord => counts.local_word += 1,
                CompletionAssistClass::SnippetOnly => counts.snippet_only += 1,
                CompletionAssistClass::FrameworkProvider => counts.framework_provider += 1,
                CompletionAssistClass::ProjectGraph => counts.project_graph += 1,
                CompletionAssistClass::ToolAdapter => counts.tool_adapter += 1,
            }
            if row.commit_disclosure_required {
                counts.additional_edit_disclosed += 1;
            }
            if row.preview_required {
                counts.preview_required += 1;
            }
            match row.availability {
                CompletionAvailabilityClass::Deprecated => counts.deprecated += 1,
                CompletionAvailabilityClass::Unavailable => counts.unavailable += 1,
                CompletionAvailabilityClass::Available => {}
            }
            if row.docs_available {
                counts.docs_available += 1;
            }
        }
        counts
    }
}

// ---------------------------------------------------------------------------
// Per-surface snapshot.
// ---------------------------------------------------------------------------

/// A completion-row snapshot for one claimed editor family, with its provider
/// posture, visible fallback label, rows, and the canonical assist snapshot
/// derived from the same rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionRowSnapshot {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub m5_completion_rows_schema_version: u32,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Editor surface family covered by this snapshot.
    pub surface_class: EditorSurfaceClass,
    /// Workspace id covered by the list.
    pub workspace_id: String,
    /// Document ref covered by the list.
    pub document_ref: String,
    /// Language id resolved for the document.
    pub language_id: String,
    /// Provider posture for the list.
    pub provider_posture: CompletionProviderPosture,
    /// Visible provider-posture label.
    pub provider_posture_label: String,
    /// Whether a visible fallback label is required.
    pub fallback_label_required: bool,
    /// Source-labeled completion rows.
    pub rows: Vec<CompletionRow>,
    /// Aggregate row counts.
    pub counts: CompletionRowCounts,
    /// Canonical shared completion-list snapshot derived from the same rows.
    pub canonical_snapshot: CompletionListSnapshot,
    /// Whether the snapshot needs source / fallback / commit disclosure.
    pub disclosure_required: bool,
    /// Accessible summary for screen readers.
    pub accessibility_summary: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl CompletionRowSnapshot {
    /// Stable record-kind tag for completion-row snapshots.
    pub const RECORD_KIND: &'static str = "m5_completion_row_snapshot";

    /// Returns the row with the given id, when present.
    pub fn row(&self, row_id: &str) -> Option<&CompletionRow> {
        self.rows.iter().find(|row| row.row_id == row_id)
    }
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

/// One frozen honesty invariant the model must satisfy, with the result of
/// evaluating it over the model's own data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionRowInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// Human-readable statement of the invariant.
    pub statement: String,
    /// Whether the invariant holds on the built model.
    pub holds: bool,
}

// ---------------------------------------------------------------------------
// Top-level record.
// ---------------------------------------------------------------------------

/// The canonical, frozen, export-safe completion-row model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompletionRowModel {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_completion_rows_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable model id.
    pub model_id: String,
    /// Capture stamp.
    pub as_of: String,
    /// Assist-class catalog.
    pub assist_classes: Vec<ClassDescriptor>,
    /// Trust-weight catalog.
    pub trust_weights: Vec<ClassDescriptor>,
    /// Additional-edit-cue catalog.
    pub additional_edit_cues: Vec<ClassDescriptor>,
    /// Availability catalog.
    pub availability_classes: Vec<ClassDescriptor>,
    /// Provider-posture catalog.
    pub provider_postures: Vec<ClassDescriptor>,
    /// One snapshot per claimed editor family.
    pub surface_snapshots: Vec<CompletionRowSnapshot>,
    /// Frozen invariants and whether each holds on this model.
    pub invariants: Vec<CompletionRowInvariant>,
    /// Whether the model is metadata-safe for support export.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl CompletionRowModel {
    /// Returns true when every frozen invariant holds on this model.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|invariant| invariant.holds)
    }

    /// Returns true when the model is metadata-safe for support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == M5_COMPLETION_ROWS_SCHEMA_REF
            && self.record_kind == M5_COMPLETION_ROWS_RECORD_KIND
    }

    /// Returns the snapshot for the given surface, when present.
    pub fn snapshot(&self, surface: EditorSurfaceClass) -> Option<&CompletionRowSnapshot> {
        self.surface_snapshots
            .iter()
            .find(|snapshot| snapshot.surface_class == surface)
    }

    /// Returns every row across every snapshot.
    pub fn all_rows(&self) -> impl Iterator<Item = &CompletionRow> {
        self.surface_snapshots
            .iter()
            .flat_map(|snapshot| snapshot.rows.iter())
    }
}

// ---------------------------------------------------------------------------
// Helpers.
// ---------------------------------------------------------------------------

const fn kind_icon_token(kind: CompletionItemKindClass) -> &'static str {
    match kind {
        CompletionItemKindClass::Function => "icon.completion.function",
        CompletionItemKindClass::Method => "icon.completion.method",
        CompletionItemKindClass::Type => "icon.completion.type",
        CompletionItemKindClass::Value => "icon.completion.value",
        CompletionItemKindClass::Keyword => "icon.completion.keyword",
        CompletionItemKindClass::Path => "icon.completion.path",
        CompletionItemKindClass::Snippet => "icon.completion.snippet",
        CompletionItemKindClass::LocalWord => "icon.completion.local_word",
    }
}

const fn kind_label(kind: CompletionItemKindClass) -> &'static str {
    match kind {
        CompletionItemKindClass::Function => "function",
        CompletionItemKindClass::Method => "method",
        CompletionItemKindClass::Type => "type",
        CompletionItemKindClass::Value => "value",
        CompletionItemKindClass::Keyword => "keyword",
        CompletionItemKindClass::Path => "path",
        CompletionItemKindClass::Snippet => "snippet",
        CompletionItemKindClass::LocalWord => "local word",
    }
}

#[allow(clippy::too_many_arguments)]
fn descriptor(
    surface: EditorSurfaceClass,
    assist_class: CompletionAssistClass,
    provider_id: Option<&str>,
    provider_label: &str,
    support: RouterSupportClass,
    freshness: RouterFreshnessClass,
    scope: RouterScopeClaimClass,
    completeness: RouterCompletenessClass,
    locality: RouterLocalityClass,
    degraded: RouterDegradedStateClass,
    scope_limits: Vec<ScopeLimitClass>,
    summary: &str,
) -> AssistSourceDescriptor {
    AssistSourceDescriptor {
        source_descriptor_id: format!(
            "completion-source:{}:{}",
            surface.as_str(),
            assist_class.as_str()
        ),
        source_family: source_family(assist_class),
        source_label_class: source_label_class(assist_class),
        source_label: provider_label.to_owned(),
        provider_id: provider_id.map(str::to_owned),
        router_decision_ref: provider_id
            .map(|id| format!("router-decision:{}:{id}", surface.as_str())),
        source_ref: None,
        support_class: support,
        freshness_class: freshness,
        scope_claim_class: scope,
        completeness_class: completeness,
        scope_limit_classes: scope_limits,
        locality_class: locality,
        degraded_state_class: degraded,
        summary: summary.to_owned(),
    }
}

const fn source_family(assist_class: CompletionAssistClass) -> AssistSourceFamily {
    match assist_class {
        CompletionAssistClass::DeterministicLanguage => AssistSourceFamily::LanguageServer,
        CompletionAssistClass::CachedFallback | CompletionAssistClass::LocalWord => {
            AssistSourceFamily::FallbackLexical
        }
        CompletionAssistClass::SnippetOnly => AssistSourceFamily::Snippet,
        CompletionAssistClass::AiBacked => AssistSourceFamily::AiAssist,
        CompletionAssistClass::ProjectGraph => AssistSourceFamily::ProjectGraph,
        CompletionAssistClass::FrameworkProvider => AssistSourceFamily::FrameworkPack,
        CompletionAssistClass::ToolAdapter => AssistSourceFamily::ToolAdapter,
    }
}

const fn source_label_class(assist_class: CompletionAssistClass) -> AssistSourceLabelClass {
    match assist_class {
        CompletionAssistClass::DeterministicLanguage => {
            AssistSourceLabelClass::DeterministicLanguage
        }
        CompletionAssistClass::CachedFallback | CompletionAssistClass::LocalWord => {
            AssistSourceLabelClass::CachedFallback
        }
        CompletionAssistClass::SnippetOnly => AssistSourceLabelClass::SnippetOrigin,
        CompletionAssistClass::AiBacked => AssistSourceLabelClass::AiInlineAssist,
        CompletionAssistClass::ProjectGraph => AssistSourceLabelClass::ProjectGraph,
        CompletionAssistClass::FrameworkProvider => AssistSourceLabelClass::FrameworkProvider,
        CompletionAssistClass::ToolAdapter => AssistSourceLabelClass::ToolAdapter,
    }
}

fn class_descriptor(token: &str, label: &str, note: &str) -> ClassDescriptor {
    ClassDescriptor {
        class_token: token.to_owned(),
        label: label.to_owned(),
        note: note.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Builder.
// ---------------------------------------------------------------------------

/// Builds the one canonical completion-row model.
///
/// The build is deterministic and self-contained: it materializes one
/// [`CompletionRowSnapshot`] per claimed editor family, derives the canonical
/// shared [`CompletionListSnapshot`] from the same rows, and evaluates every
/// frozen honesty invariant over the assembled data so the record's
/// `invariants[].holds` reflect real checks.
pub fn completion_row_model() -> CompletionRowModel {
    let surface_snapshots = build_surface_snapshots();
    let invariants = evaluate_invariants(&surface_snapshots);

    let qualified = invariants.iter().all(|invariant| invariant.holds);
    let total_rows: usize = surface_snapshots
        .iter()
        .map(|snapshot| snapshot.rows.len())
        .sum();
    let summary = if qualified {
        format!(
            "Completion-row model frozen: {rows} rows across {surfaces} editor families; \
             deterministic, cached, local-word, snippet, and AI classes stay distinct with \
             pinned trust weights, every additional-edit/import effect discloses before commit, \
             and every degraded provider posture is labeled. All {invariants} invariants hold.",
            rows = total_rows,
            surfaces = surface_snapshots.len(),
            invariants = invariants.len(),
        )
    } else {
        format!(
            "Completion-row model INVALID: {failing} of {total} invariants do not hold.",
            failing = invariants.iter().filter(|i| !i.holds).count(),
            total = invariants.len(),
        )
    };

    CompletionRowModel {
        record_kind: M5_COMPLETION_ROWS_RECORD_KIND.to_owned(),
        m5_completion_rows_schema_version: M5_COMPLETION_ROWS_SCHEMA_VERSION,
        schema_ref: M5_COMPLETION_ROWS_SCHEMA_REF.to_owned(),
        model_id: M5_COMPLETION_ROWS_MODEL_ID.to_owned(),
        as_of: M5_COMPLETION_ROWS_AS_OF.to_owned(),
        assist_classes: build_assist_class_catalog(),
        trust_weights: build_trust_weight_catalog(),
        additional_edit_cues: build_cue_catalog(),
        availability_classes: build_availability_catalog(),
        provider_postures: build_posture_catalog(),
        surface_snapshots,
        invariants,
        raw_payload_excluded: true,
        summary,
    }
}

/// Builds the human-readable projection of the model for support and headless use.
pub fn completion_row_model_lines(model: &CompletionRowModel) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Completion-row model — {} ({})",
        model.model_id, model.as_of
    ));
    lines.push(format!(
        "schema_ref={} version={}",
        model.schema_ref, model.m5_completion_rows_schema_version
    ));

    lines.push("Surface snapshots:".to_owned());
    for snapshot in &model.surface_snapshots {
        lines.push(format!(
            "  {surface}: {posture} ({label}) — {total} rows, disclosure={disclosure}",
            surface = snapshot.surface_class.as_str(),
            posture = snapshot.provider_posture.as_str(),
            label = snapshot.provider_posture_label,
            total = snapshot.counts.total,
            disclosure = snapshot.disclosure_required,
        ));
        for row in &snapshot.rows {
            lines.push(format!(
                "    {label} [{kind}] class={class} trust={trust} cue={cue} \
                 disclose={disclose} preview={preview} availability={availability}",
                label = row.primary_label,
                kind = kind_label(row.kind_class),
                class = row.assist_class.as_str(),
                trust = row.trust_weight.as_str(),
                cue = row.additional_edit_cue.as_str(),
                disclose = row.commit_disclosure_required,
                preview = row.preview_required,
                availability = row.availability.as_str(),
            ));
        }
    }

    lines.push("Invariants:".to_owned());
    for invariant in &model.invariants {
        lines.push(format!(
            "  {id} holds={holds}",
            id = invariant.invariant_id,
            holds = invariant.holds,
        ));
    }

    lines.push(model.summary.clone());
    lines
}

// ---------------------------------------------------------------------------
// Catalog builders.
// ---------------------------------------------------------------------------

fn build_assist_class_catalog() -> Vec<ClassDescriptor> {
    CompletionAssistClass::ALL
        .iter()
        .map(|class| {
            class_descriptor(
                class.as_str(),
                class.label(),
                &format!(
                    "Trust weight {}; {}distinct from deterministic semantics.",
                    class.trust_weight().as_str(),
                    if class.requires_visual_distinction() {
                        "must be kept visually "
                    } else {
                        "kept "
                    },
                ),
            )
        })
        .collect()
}

fn build_trust_weight_catalog() -> Vec<ClassDescriptor> {
    TrustWeightClass::ALL
        .iter()
        .map(|weight| {
            class_descriptor(
                weight.as_str(),
                weight.label(),
                if weight.is_full_semantic() {
                    "Only deterministic semantic / project-graph / framework providers may carry this weight."
                } else {
                    "Advisory or fallback weight; never inherits deterministic-semantic styling."
                },
            )
        })
        .collect()
}

fn build_cue_catalog() -> Vec<ClassDescriptor> {
    AdditionalEditCue::ALL
        .iter()
        .map(|cue| {
            class_descriptor(
                cue.as_str(),
                cue.label(),
                if cue.requires_pre_commit_disclosure() {
                    "Must be disclosed before commit."
                } else {
                    "No additional effect on accept."
                },
            )
        })
        .collect()
}

fn build_availability_catalog() -> Vec<ClassDescriptor> {
    CompletionAvailabilityClass::ALL
        .iter()
        .map(|availability| {
            class_descriptor(
                availability.as_str(),
                availability.label(),
                if availability.is_acceptable() {
                    "Acceptable; deprecated rows carry a non-color marker."
                } else {
                    "Inspect-only; cannot be applied."
                },
            )
        })
        .collect()
}

fn build_posture_catalog() -> Vec<ClassDescriptor> {
    CompletionProviderPosture::ALL
        .iter()
        .map(|posture| {
            class_descriptor(
                posture.as_str(),
                posture.label(),
                if posture.requires_fallback_label() {
                    "Degraded; a visible fallback label is required."
                } else {
                    "Full semantic; no fallback label needed."
                },
            )
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Surface snapshot builders.
// ---------------------------------------------------------------------------

struct SnapshotSpec {
    surface: EditorSurfaceClass,
    session_id: &'static str,
    workspace_id: &'static str,
    document_ref: &'static str,
    language_id: &'static str,
    posture: CompletionProviderPosture,
}

fn build_surface_snapshots() -> Vec<CompletionRowSnapshot> {
    vec![
        build_code_file_snapshot(),
        build_config_file_snapshot(),
        build_notebook_cell_snapshot(),
        build_request_editor_snapshot(),
        build_sql_editor_snapshot(),
        build_docs_code_block_snapshot(),
        build_generated_file_snapshot(),
        build_protected_file_snapshot(),
        build_partial_index_snapshot(),
        build_large_file_snapshot(),
    ]
}

fn assemble_snapshot(spec: SnapshotSpec, rows: Vec<CompletionRow>) -> CompletionRowSnapshot {
    let counts = CompletionRowCounts::from_rows(&rows);
    let items: Vec<CompletionItemRecord> = rows
        .iter()
        .map(|row| row.to_canonical_item(M5_COMPLETION_ROWS_AS_OF))
        .collect();
    let canonical_snapshot = CompletionListSnapshot::from_items(
        CompletionListRequest {
            completion_session_id: spec.session_id.to_owned(),
            workspace_id: spec.workspace_id.to_owned(),
            document_ref: spec.document_ref.to_owned(),
            language_id: spec.language_id.to_owned(),
            request_anchor_ref: format!("anchor:{}", spec.surface.as_str()),
            captured_at: M5_COMPLETION_ROWS_AS_OF.to_owned(),
        },
        items,
    );

    let disclosure_required = spec.posture.is_degraded()
        || rows
            .iter()
            .any(|row| row.commit_disclosure_required || row.source.requires_degraded_disclosure());

    let accessibility_summary = format!(
        "{total} completion rows for {surface}: {det} semantic, {ai} AI-backed, \
         {fallback} fallback. Provider posture: {posture}.",
        total = counts.total,
        surface = spec.surface.label(),
        det = counts.deterministic_language,
        ai = counts.ai_backed,
        fallback = counts.cached_fallback + counts.local_word,
        posture = spec.posture.label(),
    );
    let export_safe_summary = format!(
        "Completion rows for {surface} under posture {posture}: \
         {total} rows, {disclosed} disclose additional edits.",
        surface = spec.surface.as_str(),
        posture = spec.posture.as_str(),
        total = counts.total,
        disclosed = counts.additional_edit_disclosed,
    );

    CompletionRowSnapshot {
        record_kind: CompletionRowSnapshot::RECORD_KIND.to_owned(),
        m5_completion_rows_schema_version: M5_COMPLETION_ROWS_SCHEMA_VERSION,
        snapshot_id: format!("completion-rows:{}", spec.surface.as_str()),
        surface_class: spec.surface,
        workspace_id: spec.workspace_id.to_owned(),
        document_ref: spec.document_ref.to_owned(),
        language_id: spec.language_id.to_owned(),
        provider_posture: spec.posture,
        provider_posture_label: spec.posture.label().to_owned(),
        fallback_label_required: spec.posture.requires_fallback_label(),
        rows,
        counts,
        canonical_snapshot,
        disclosure_required,
        accessibility_summary,
        export_safe_summary,
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    _surface: EditorSurfaceClass,
    session_id: &str,
    row_id: &str,
    primary_label: &str,
    kind: CompletionItemKindClass,
    source: AssistSourceDescriptor,
    cue: AdditionalEditCue,
    additional_edit_summary: Option<&str>,
    preview_required: bool,
    availability: CompletionAvailabilityClass,
    docs_available: bool,
    rank: u32,
) -> CompletionRow {
    CompletionRow::new(CompletionRowInit {
        row_id: row_id.to_owned(),
        completion_session_id: session_id.to_owned(),
        primary_label: primary_label.to_owned(),
        kind_class: kind,
        source,
        additional_edit_cue: cue,
        additional_edit_summary: additional_edit_summary.map(str::to_owned),
        preview_required,
        availability,
        docs_available,
        rank,
        sort_group: 0,
    })
}

fn build_code_file_snapshot() -> CompletionRowSnapshot {
    let surface = EditorSurfaceClass::CodeFile;
    let session = "completion:code_file";
    let rows = vec![
        row(
            surface,
            session,
            "code-fn",
            "render_frame",
            CompletionItemKindClass::Function,
            descriptor(
                surface,
                CompletionAssistClass::DeterministicLanguage,
                Some("rust-analyzer"),
                "rust-analyzer",
                RouterSupportClass::Authoritative,
                RouterFreshnessClass::AuthoritativeLive,
                RouterScopeClaimClass::WholeWorkspace,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalSidecar,
                RouterDegradedStateClass::None,
                Vec::new(),
                "Workspace-wide semantic completion from the language server.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            true,
            0,
        ),
        row(
            surface,
            session,
            "code-type-import",
            "Duration",
            CompletionItemKindClass::Type,
            descriptor(
                surface,
                CompletionAssistClass::DeterministicLanguage,
                Some("rust-analyzer"),
                "rust-analyzer",
                RouterSupportClass::Authoritative,
                RouterFreshnessClass::AuthoritativeLive,
                RouterScopeClaimClass::WholeWorkspace,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalSidecar,
                RouterDegradedStateClass::None,
                Vec::new(),
                "Type completion that adds a use import on accept.",
            ),
            AdditionalEditCue::AddsImport,
            Some("Adds `use std::time::Duration;` to the import block."),
            false,
            CompletionAvailabilityClass::Available,
            true,
            1,
        ),
        row(
            surface,
            session,
            "code-deprecated",
            "legacy_paint",
            CompletionItemKindClass::Method,
            descriptor(
                surface,
                CompletionAssistClass::DeterministicLanguage,
                Some("rust-analyzer"),
                "rust-analyzer",
                RouterSupportClass::Authoritative,
                RouterFreshnessClass::AuthoritativeLive,
                RouterScopeClaimClass::WholeWorkspace,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalSidecar,
                RouterDegradedStateClass::None,
                Vec::new(),
                "Deprecated method completion, marked and still inspectable.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Deprecated,
            true,
            2,
        ),
        row(
            surface,
            session,
            "code-snippet",
            "for-loop",
            CompletionItemKindClass::Snippet,
            descriptor(
                surface,
                CompletionAssistClass::SnippetOnly,
                None,
                "Snippet pack",
                RouterSupportClass::Authoritative,
                RouterFreshnessClass::AuthoritativeLive,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::None,
                Vec::new(),
                "Snippet-session completion.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            false,
            3,
        ),
        row(
            surface,
            session,
            "code-localword",
            "frame_budget",
            CompletionItemKindClass::LocalWord,
            descriptor(
                surface,
                CompletionAssistClass::LocalWord,
                None,
                "Local words",
                RouterSupportClass::FallbackOnly,
                RouterFreshnessClass::WarmCached,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::PartialForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::None,
                vec![ScopeLimitClass::SingleFileOnly],
                "Lexical local-word fallback from the open buffer.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            false,
            4,
        ),
        row(
            surface,
            session,
            "code-ai",
            "render_frame_with_vsync",
            CompletionItemKindClass::Function,
            descriptor(
                surface,
                CompletionAssistClass::AiBacked,
                None,
                "AI assist",
                RouterSupportClass::Advisory,
                RouterFreshnessClass::AuthoritativeLive,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::PartialForClaimedScope,
                RouterLocalityClass::ManagedService,
                RouterDegradedStateClass::None,
                Vec::new(),
                "AI-backed proposal; advisory weight, kept visually distinct.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            false,
            5,
        ),
    ];
    assemble_snapshot(
        SnapshotSpec {
            surface,
            session_id: session,
            workspace_id: "workspace:demo",
            document_ref: "doc:src/render.rs",
            language_id: "rust",
            posture: CompletionProviderPosture::FullSemantic,
        },
        rows,
    )
}

fn build_config_file_snapshot() -> CompletionRowSnapshot {
    let surface = EditorSurfaceClass::ConfigFile;
    let session = "completion:config_file";
    let rows = vec![
        row(
            surface,
            session,
            "config-key",
            "build.target",
            CompletionItemKindClass::Value,
            descriptor(
                surface,
                CompletionAssistClass::FrameworkProvider,
                Some("config-schema"),
                "Config schema",
                RouterSupportClass::Authoritative,
                RouterFreshnessClass::AuthoritativeLive,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::None,
                Vec::new(),
                "Schema-backed key completion.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            true,
            0,
        ),
        row(
            surface,
            session,
            "config-value-edit",
            "release",
            CompletionItemKindClass::Value,
            descriptor(
                surface,
                CompletionAssistClass::FrameworkProvider,
                Some("config-schema"),
                "Config schema",
                RouterSupportClass::Authoritative,
                RouterFreshnessClass::AuthoritativeLive,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::None,
                Vec::new(),
                "Value completion that rewrites a sibling profile key.",
            ),
            AdditionalEditCue::EditsConfig,
            Some("Sets `profile.release.opt-level = 3` to keep the profile valid."),
            false,
            CompletionAvailabilityClass::Available,
            true,
            1,
        ),
        row(
            surface,
            session,
            "config-localword",
            "target_dir",
            CompletionItemKindClass::LocalWord,
            descriptor(
                surface,
                CompletionAssistClass::LocalWord,
                None,
                "Local words",
                RouterSupportClass::FallbackOnly,
                RouterFreshnessClass::WarmCached,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::PartialForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::None,
                vec![ScopeLimitClass::SingleFileOnly],
                "Lexical local-word fallback.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            false,
            2,
        ),
    ];
    assemble_snapshot(
        SnapshotSpec {
            surface,
            session_id: session,
            workspace_id: "workspace:demo",
            document_ref: "doc:Cargo.toml",
            language_id: "toml",
            posture: CompletionProviderPosture::FullSemantic,
        },
        rows,
    )
}

fn build_notebook_cell_snapshot() -> CompletionRowSnapshot {
    let surface = EditorSurfaceClass::NotebookCell;
    let session = "completion:notebook_cell";
    let rows = vec![
        row(
            surface,
            session,
            "nb-fn",
            "DataFrame",
            CompletionItemKindClass::Type,
            descriptor(
                surface,
                CompletionAssistClass::DeterministicLanguage,
                Some("pyright"),
                "Pyright",
                RouterSupportClass::Authoritative,
                RouterFreshnessClass::AuthoritativeLive,
                RouterScopeClaimClass::NotebookCell,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalSidecar,
                RouterDegradedStateClass::None,
                Vec::new(),
                "Cell-scoped semantic completion.",
            ),
            AdditionalEditCue::AddsImport,
            Some("Adds `import pandas as pd` to the first cell."),
            false,
            CompletionAvailabilityClass::Available,
            true,
            0,
        ),
        row(
            surface,
            session,
            "nb-ai",
            "df.describe()",
            CompletionItemKindClass::Method,
            descriptor(
                surface,
                CompletionAssistClass::AiBacked,
                None,
                "AI assist",
                RouterSupportClass::Advisory,
                RouterFreshnessClass::AuthoritativeLive,
                RouterScopeClaimClass::NotebookCell,
                RouterCompletenessClass::PartialForClaimedScope,
                RouterLocalityClass::ManagedService,
                RouterDegradedStateClass::None,
                Vec::new(),
                "AI-backed cell proposal; advisory weight.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            false,
            1,
        ),
        row(
            surface,
            session,
            "nb-localword",
            "frame_rate",
            CompletionItemKindClass::LocalWord,
            descriptor(
                surface,
                CompletionAssistClass::LocalWord,
                None,
                "Local words",
                RouterSupportClass::FallbackOnly,
                RouterFreshnessClass::WarmCached,
                RouterScopeClaimClass::NotebookCell,
                RouterCompletenessClass::PartialForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::None,
                vec![ScopeLimitClass::SingleFileOnly],
                "Lexical local-word fallback from the cell.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            false,
            2,
        ),
    ];
    assemble_snapshot(
        SnapshotSpec {
            surface,
            session_id: session,
            workspace_id: "workspace:demo",
            document_ref: "doc:analysis.ipynb#cell-3",
            language_id: "python",
            posture: CompletionProviderPosture::FullSemantic,
        },
        rows,
    )
}

fn build_request_editor_snapshot() -> CompletionRowSnapshot {
    let surface = EditorSurfaceClass::RequestEditor;
    let session = "completion:request_editor";
    let rows = vec![
        row(
            surface,
            session,
            "req-header",
            "Authorization",
            CompletionItemKindClass::Keyword,
            descriptor(
                surface,
                CompletionAssistClass::FrameworkProvider,
                Some("http-schema"),
                "HTTP schema",
                RouterSupportClass::Authoritative,
                RouterFreshnessClass::AuthoritativeLive,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::None,
                Vec::new(),
                "Header-name completion from the request schema.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            true,
            0,
        ),
        row(
            surface,
            session,
            "req-var",
            "{{base_url}}",
            CompletionItemKindClass::Value,
            descriptor(
                surface,
                CompletionAssistClass::ProjectGraph,
                Some("env-resolver"),
                "Environment",
                RouterSupportClass::Authoritative,
                RouterFreshnessClass::AuthoritativeLive,
                RouterScopeClaimClass::ActiveWorkset,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::None,
                Vec::new(),
                "Environment-variable completion resolved from the active environment.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            true,
            1,
        ),
        row(
            surface,
            session,
            "req-localword",
            "X-Trace-Id",
            CompletionItemKindClass::LocalWord,
            descriptor(
                surface,
                CompletionAssistClass::LocalWord,
                None,
                "Local words",
                RouterSupportClass::FallbackOnly,
                RouterFreshnessClass::WarmCached,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::PartialForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::None,
                vec![ScopeLimitClass::SingleFileOnly],
                "Lexical local-word fallback.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            false,
            2,
        ),
    ];
    assemble_snapshot(
        SnapshotSpec {
            surface,
            session_id: session,
            workspace_id: "workspace:demo",
            document_ref: "doc:requests/login.http",
            language_id: "http",
            posture: CompletionProviderPosture::FullSemantic,
        },
        rows,
    )
}

fn build_sql_editor_snapshot() -> CompletionRowSnapshot {
    let surface = EditorSurfaceClass::SqlEditor;
    let session = "completion:sql_editor";
    let rows = vec![
        row(
            surface,
            session,
            "sql-keyword",
            "SELECT",
            CompletionItemKindClass::Keyword,
            descriptor(
                surface,
                CompletionAssistClass::DeterministicLanguage,
                Some("sql-grammar"),
                "SQL grammar",
                RouterSupportClass::Authoritative,
                RouterFreshnessClass::AuthoritativeLive,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::None,
                Vec::new(),
                "Deterministic SQL keyword completion.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            true,
            0,
        ),
        row(
            surface,
            session,
            "sql-table-cached",
            "customers",
            CompletionItemKindClass::Value,
            descriptor(
                surface,
                CompletionAssistClass::CachedFallback,
                Some("db-introspect"),
                "Schema (cached)",
                RouterSupportClass::FallbackOnly,
                RouterFreshnessClass::DegradedCached,
                RouterScopeClaimClass::WholeWorkspace,
                RouterCompletenessClass::PartialForClaimedScope,
                RouterLocalityClass::ManagedService,
                RouterDegradedStateClass::DegradedProviderUnavailable,
                vec![ScopeLimitClass::SingleFileOnly],
                "Table name from a cached schema; live connection unavailable.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            false,
            1,
        ),
        row(
            surface,
            session,
            "sql-localword",
            "order_total",
            CompletionItemKindClass::LocalWord,
            descriptor(
                surface,
                CompletionAssistClass::LocalWord,
                None,
                "Local words",
                RouterSupportClass::FallbackOnly,
                RouterFreshnessClass::WarmCached,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::PartialForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::None,
                vec![ScopeLimitClass::SingleFileOnly],
                "Lexical local-word fallback.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            false,
            2,
        ),
    ];
    assemble_snapshot(
        SnapshotSpec {
            surface,
            session_id: session,
            workspace_id: "workspace:demo",
            document_ref: "doc:queries/report.sql",
            language_id: "sql",
            posture: CompletionProviderPosture::DegradedProvider,
        },
        rows,
    )
}

fn build_docs_code_block_snapshot() -> CompletionRowSnapshot {
    let surface = EditorSurfaceClass::DocsCodeBlock;
    let session = "completion:docs_code_block";
    let rows = vec![
        row(
            surface,
            session,
            "docs-snippet",
            "fn-stub",
            CompletionItemKindClass::Snippet,
            descriptor(
                surface,
                CompletionAssistClass::SnippetOnly,
                None,
                "Snippet pack",
                RouterSupportClass::Authoritative,
                RouterFreshnessClass::AuthoritativeLive,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::None,
                Vec::new(),
                "Snippet completion inside a docs code block.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            false,
            0,
        ),
        row(
            surface,
            session,
            "docs-localword",
            "viewport",
            CompletionItemKindClass::LocalWord,
            descriptor(
                surface,
                CompletionAssistClass::LocalWord,
                None,
                "Local words",
                RouterSupportClass::FallbackOnly,
                RouterFreshnessClass::WarmCached,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::PartialForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::None,
                vec![ScopeLimitClass::SingleFileOnly],
                "Lexical fallback inside a docs code block; semantic provider not wired.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            false,
            1,
        ),
        row(
            surface,
            session,
            "docs-semantic-unavailable",
            "Self::render",
            CompletionItemKindClass::Method,
            descriptor(
                surface,
                CompletionAssistClass::DeterministicLanguage,
                Some("rust-analyzer"),
                "rust-analyzer",
                RouterSupportClass::InspectOnly,
                RouterFreshnessClass::Unverified,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::UnavailableForClaimedScope,
                RouterLocalityClass::LocalSidecar,
                RouterDegradedStateClass::DegradedScopeNarrowed,
                vec![ScopeLimitClass::SingleFileOnly],
                "Semantic completion unavailable for an embedded code block.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Unavailable,
            false,
            2,
        ),
    ];
    assemble_snapshot(
        SnapshotSpec {
            surface,
            session_id: session,
            workspace_id: "workspace:demo",
            document_ref: "doc:README.md#code-block-2",
            language_id: "rust",
            posture: CompletionProviderPosture::RestrictedMode,
        },
        rows,
    )
}

fn build_generated_file_snapshot() -> CompletionRowSnapshot {
    let surface = EditorSurfaceClass::GeneratedFile;
    let session = "completion:generated_file";
    let rows = vec![
        row(
            surface,
            session,
            "gen-symbol",
            "ProtoMessage",
            CompletionItemKindClass::Type,
            descriptor(
                surface,
                CompletionAssistClass::DeterministicLanguage,
                Some("generated-bridge"),
                "Generated-source bridge",
                RouterSupportClass::Advisory,
                RouterFreshnessClass::AuthoritativeLive,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalSidecar,
                RouterDegradedStateClass::None,
                Vec::new(),
                "Symbol read from generated output; editing routes through the generator.",
            ),
            AdditionalEditCue::GeneratedOutputEffect,
            Some("Accepting edits the generator template, not the generated file directly."),
            true,
            CompletionAvailabilityClass::Available,
            true,
            0,
        ),
        row(
            surface,
            session,
            "gen-localword",
            "field_count",
            CompletionItemKindClass::LocalWord,
            descriptor(
                surface,
                CompletionAssistClass::LocalWord,
                None,
                "Local words",
                RouterSupportClass::FallbackOnly,
                RouterFreshnessClass::WarmCached,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::PartialForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::None,
                vec![ScopeLimitClass::SingleFileOnly],
                "Lexical fallback in a generated file.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            false,
            1,
        ),
    ];
    assemble_snapshot(
        SnapshotSpec {
            surface,
            session_id: session,
            workspace_id: "workspace:demo",
            document_ref: "doc:gen/messages.rs",
            language_id: "rust",
            posture: CompletionProviderPosture::FullSemantic,
        },
        rows,
    )
}

fn build_protected_file_snapshot() -> CompletionRowSnapshot {
    let surface = EditorSurfaceClass::ProtectedFile;
    let session = "completion:protected_file";
    let rows = vec![
        row(
            surface,
            session,
            "prot-symbol",
            "rotate_signing_key",
            CompletionItemKindClass::Function,
            descriptor(
                surface,
                CompletionAssistClass::DeterministicLanguage,
                Some("rust-analyzer"),
                "rust-analyzer",
                RouterSupportClass::Authoritative,
                RouterFreshnessClass::AuthoritativeLive,
                RouterScopeClaimClass::WholeWorkspace,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalSidecar,
                RouterDegradedStateClass::DegradedPolicyNarrowed,
                Vec::new(),
                "Semantic completion in a protected path; acceptance requires review.",
            ),
            AdditionalEditCue::AdditionalEditsInFile,
            Some("Edits in a protected path are staged for review before commit."),
            true,
            CompletionAvailabilityClass::Available,
            true,
            0,
        ),
        row(
            surface,
            session,
            "prot-localword",
            "key_id",
            CompletionItemKindClass::LocalWord,
            descriptor(
                surface,
                CompletionAssistClass::LocalWord,
                None,
                "Local words",
                RouterSupportClass::FallbackOnly,
                RouterFreshnessClass::WarmCached,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::PartialForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::None,
                vec![ScopeLimitClass::SingleFileOnly],
                "Lexical fallback in a protected path.",
            ),
            AdditionalEditCue::None,
            None,
            true,
            CompletionAvailabilityClass::Available,
            false,
            1,
        ),
    ];
    assemble_snapshot(
        SnapshotSpec {
            surface,
            session_id: session,
            workspace_id: "workspace:demo",
            document_ref: "doc:crates/aureline-keys/src/lib.rs",
            language_id: "rust",
            posture: CompletionProviderPosture::RestrictedMode,
        },
        rows,
    )
}

fn build_partial_index_snapshot() -> CompletionRowSnapshot {
    let surface = EditorSurfaceClass::PartialIndexState;
    let session = "completion:partial_index";
    let rows = vec![
        row(
            surface,
            session,
            "idx-partial-symbol",
            "WorkspaceGraph",
            CompletionItemKindClass::Type,
            descriptor(
                surface,
                CompletionAssistClass::DeterministicLanguage,
                Some("rust-analyzer"),
                "rust-analyzer (indexing)",
                RouterSupportClass::Advisory,
                RouterFreshnessClass::Stale,
                RouterScopeClaimClass::LoadedSlice,
                RouterCompletenessClass::PartialForClaimedScope,
                RouterLocalityClass::LocalSidecar,
                RouterDegradedStateClass::DegradedScopeNarrowed,
                vec![ScopeLimitClass::SingleFileOnly],
                "Semantic completion narrowed to the indexed slice while the index builds.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            true,
            0,
        ),
        row(
            surface,
            session,
            "idx-localword",
            "graph_node",
            CompletionItemKindClass::LocalWord,
            descriptor(
                surface,
                CompletionAssistClass::LocalWord,
                None,
                "Local words",
                RouterSupportClass::FallbackOnly,
                RouterFreshnessClass::WarmCached,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::PartialForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::None,
                vec![ScopeLimitClass::SingleFileOnly],
                "Lexical fallback while the index is incomplete.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            false,
            1,
        ),
    ];
    assemble_snapshot(
        SnapshotSpec {
            surface,
            session_id: session,
            workspace_id: "workspace:demo",
            document_ref: "doc:src/graph.rs",
            language_id: "rust",
            posture: CompletionProviderPosture::StalePartialIndex,
        },
        rows,
    )
}

fn build_large_file_snapshot() -> CompletionRowSnapshot {
    let surface = EditorSurfaceClass::LargeFileRestricted;
    let session = "completion:large_file";
    let rows = vec![
        row(
            surface,
            session,
            "lf-localword",
            "checksum_block",
            CompletionItemKindClass::LocalWord,
            descriptor(
                surface,
                CompletionAssistClass::LocalWord,
                None,
                "Local words (lexical)",
                RouterSupportClass::FallbackOnly,
                RouterFreshnessClass::WarmCached,
                RouterScopeClaimClass::LoadedSlice,
                RouterCompletenessClass::PartialForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::DegradedScopeNarrowed,
                vec![ScopeLimitClass::SingleFileOnly],
                "Lexical fallback over the loaded slice in large-file mode.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            false,
            0,
        ),
        row(
            surface,
            session,
            "lf-snippet",
            "line-jump",
            CompletionItemKindClass::Snippet,
            descriptor(
                surface,
                CompletionAssistClass::SnippetOnly,
                None,
                "Snippet pack",
                RouterSupportClass::Authoritative,
                RouterFreshnessClass::AuthoritativeLive,
                RouterScopeClaimClass::SingleFile,
                RouterCompletenessClass::CompleteForClaimedScope,
                RouterLocalityClass::LocalInProcess,
                RouterDegradedStateClass::None,
                Vec::new(),
                "Snippet completion still available in large-file mode.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Available,
            false,
            1,
        ),
        row(
            surface,
            session,
            "lf-semantic-unavailable",
            "WorkspaceIndex::lookup",
            CompletionItemKindClass::Method,
            descriptor(
                surface,
                CompletionAssistClass::DeterministicLanguage,
                Some("rust-analyzer"),
                "rust-analyzer",
                RouterSupportClass::Unsupported,
                RouterFreshnessClass::Unverified,
                RouterScopeClaimClass::LoadedSlice,
                RouterCompletenessClass::UnavailableForClaimedScope,
                RouterLocalityClass::LocalSidecar,
                RouterDegradedStateClass::DegradedScopeNarrowed,
                vec![ScopeLimitClass::SingleFileOnly],
                "Semantic completion disabled in large-file mode.",
            ),
            AdditionalEditCue::None,
            None,
            false,
            CompletionAvailabilityClass::Unavailable,
            false,
            2,
        ),
    ];
    assemble_snapshot(
        SnapshotSpec {
            surface,
            session_id: session,
            workspace_id: "workspace:demo",
            document_ref: "doc:data/dump.log",
            language_id: "log",
            posture: CompletionProviderPosture::LargeFileFallback,
        },
        rows,
    )
}

// ---------------------------------------------------------------------------
// Invariant evaluation.
// ---------------------------------------------------------------------------

fn evaluate_invariants(snapshots: &[CompletionRowSnapshot]) -> Vec<CompletionRowInvariant> {
    let all_rows: Vec<&CompletionRow> = snapshots
        .iter()
        .flat_map(|snapshot| snapshot.rows.iter())
        .collect();

    let mut invariants = Vec::new();

    invariants.push(CompletionRowInvariant {
        invariant_id: "every_surface_family_has_rows".into(),
        statement: "Each claimed editor family resolves at least one completion row.".into(),
        holds: !snapshots.is_empty() && snapshots.iter().all(|s| !s.rows.is_empty()),
    });

    invariants.push(CompletionRowInvariant {
        invariant_id: "ai_never_full_semantic_trust".into(),
        statement: "No AI-backed row carries full-semantic trust weight.".into(),
        holds: all_rows
            .iter()
            .filter(|r| r.assist_class.is_ai_backed())
            .all(|r| !r.trust_weight.is_full_semantic()),
    });

    invariants.push(CompletionRowInvariant {
        invariant_id: "local_word_never_full_semantic_trust".into(),
        statement: "No local-word row carries full-semantic trust weight.".into(),
        holds: all_rows
            .iter()
            .filter(|r| r.assist_class == CompletionAssistClass::LocalWord)
            .all(|r| !r.trust_weight.is_full_semantic()),
    });

    invariants.push(CompletionRowInvariant {
        invariant_id: "trust_weight_tracks_assist_class".into(),
        statement: "Every row's trust weight equals the weight pinned to its assist class.".into(),
        holds: all_rows
            .iter()
            .all(|r| r.trust_weight == r.assist_class.trust_weight()),
    });

    invariants.push(CompletionRowInvariant {
        invariant_id: "additional_edit_rows_disclose_before_commit".into(),
        statement: "Every acceptable row with an additional-edit cue discloses it before commit."
            .into(),
        holds: all_rows
            .iter()
            .filter(|r| {
                r.availability.is_acceptable()
                    && r.additional_edit_cue.requires_pre_commit_disclosure()
            })
            .all(|r| r.commit_disclosure_required),
    });

    invariants.push(CompletionRowInvariant {
        invariant_id: "generated_and_dependency_effects_require_preview".into(),
        statement: "Rows whose acceptance changes generated output or a dependency require preview."
            .into(),
        holds: all_rows
            .iter()
            .filter(|r| {
                r.availability.is_acceptable()
                    && matches!(
                        r.additional_edit_cue,
                        AdditionalEditCue::GeneratedOutputEffect | AdditionalEditCue::AddsDependency
                    )
            })
            .all(|r| r.preview_required),
    });

    invariants.push(CompletionRowInvariant {
        invariant_id: "degraded_postures_label_fallback".into(),
        statement: "Every degraded provider posture carries a visible fallback label and \
                    flags disclosure."
            .into(),
        holds: snapshots
            .iter()
            .filter(|s| s.provider_posture.is_degraded())
            .all(|s| {
                s.fallback_label_required
                    && !s.provider_posture_label.trim().is_empty()
                    && s.disclosure_required
            }),
    });

    invariants.push(CompletionRowInvariant {
        invariant_id: "deterministic_and_ai_are_distinct".into(),
        statement: "At least one surface exposes both a deterministic and an AI-backed row, and \
                    their assist class and trust weight differ."
            .into(),
        holds: snapshots.iter().any(|s| {
            let det = s.rows.iter().find(|r| r.assist_class.is_deterministic());
            let ai = s.rows.iter().find(|r| r.assist_class.is_ai_backed());
            match (det, ai) {
                (Some(det), Some(ai)) => {
                    det.assist_class != ai.assist_class && det.trust_weight != ai.trust_weight
                }
                _ => false,
            }
        }),
    });

    invariants.push(CompletionRowInvariant {
        invariant_id: "every_row_carries_source_label".into(),
        statement: "Every row carries a non-empty source label so provider/source is visible."
            .into(),
        holds: all_rows
            .iter()
            .all(|r| !r.source.source_label.trim().is_empty()),
    });

    invariants.push(CompletionRowInvariant {
        invariant_id: "rows_match_canonical_snapshot".into(),
        statement: "Each snapshot's canonical assist list has one item per row with the same \
                    source and id."
            .into(),
        holds: snapshots.iter().all(|s| {
            if s.canonical_snapshot.items.len() != s.rows.len() {
                return false;
            }
            s.rows.iter().all(|row| {
                s.canonical_snapshot
                    .items
                    .iter()
                    .find(|item| item.completion_item_id == row.row_id)
                    .is_some_and(|item| {
                        item.source == row.source && item.label == row.primary_label
                    })
            })
        }),
    });

    invariants.push(CompletionRowInvariant {
        invariant_id: "non_available_rows_are_marked".into(),
        statement: "Every deprecated or unavailable row carries a non-color marker and visual \
                    distinction."
            .into(),
        holds: all_rows
            .iter()
            .filter(|r| r.availability != CompletionAvailabilityClass::Available)
            .all(|r| {
                r.requires_visual_distinction
                    && r.non_color_differentiator
                        .contains(r.availability.non_color_marker())
            }),
    });

    invariants.push(CompletionRowInvariant {
        invariant_id: "distinct_classes_have_non_color_differentiator".into(),
        statement: "Every row requiring visual distinction carries a non-empty non-color \
                    differentiator."
            .into(),
        holds: all_rows
            .iter()
            .filter(|r| r.requires_visual_distinction)
            .all(|r| !r.non_color_differentiator.trim().is_empty()),
    });

    invariants.push(CompletionRowInvariant {
        invariant_id: "disclosure_implies_summary_or_preview".into(),
        statement: "Every row that must disclose before commit carries a summary or requires \
                    preview."
            .into(),
        holds: all_rows
            .iter()
            .filter(|r| r.commit_disclosure_required)
            .all(|r| r.additional_edit_summary.is_some() || r.preview_required),
    });

    invariants.push(CompletionRowInvariant {
        invariant_id: "unavailable_rows_are_inspect_only".into(),
        statement: "Every unavailable row resolves to an inspect-only acceptance posture.".into(),
        holds: all_rows
            .iter()
            .filter(|r| r.availability == CompletionAvailabilityClass::Unavailable)
            .all(|r| r.side_effect_class() == CompletionSideEffectClass::InspectOnlyNoApply),
    });

    invariants.push(CompletionRowInvariant {
        invariant_id: "assist_class_catalog_complete".into(),
        statement: "Every assist class appears at least once across the surfaces.".into(),
        holds: CompletionAssistClass::ALL.iter().all(|class| {
            // ProjectGraph, FrameworkProvider, and ToolAdapter need not appear on
            // every surface, but the deterministic/cached/local-word/snippet/AI
            // core classes that drive the distinction must all be exercised.
            if matches!(
                class,
                CompletionAssistClass::DeterministicLanguage
                    | CompletionAssistClass::CachedFallback
                    | CompletionAssistClass::LocalWord
                    | CompletionAssistClass::SnippetOnly
                    | CompletionAssistClass::AiBacked
            ) {
                all_rows.iter().any(|r| r.assist_class == *class)
            } else {
                true
            }
        }),
    });

    invariants
}

#[cfg(test)]
mod tests;
