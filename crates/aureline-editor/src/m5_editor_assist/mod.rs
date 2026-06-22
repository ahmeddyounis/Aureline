//! Canonical editor-assist micro-surface matrix: the editor's single, frozen,
//! export-safe truth for decorations, code lenses, inlay hints, completion,
//! signature help, snippet sessions, and hover/peek across the editor surfaces
//! the current product claims.
//!
//! Every editor surface — code files, config files, notebook cells, request and
//! SQL editors, docs-code panes, generated files, protected files, partial-index
//! states, and large-file / restricted mode — used to be free to invent its own
//! assist-source labels, precedence order, and degraded-state copy. This module
//! freezes one matrix so they cannot. It is consumed directly by the editor
//! shell, the headless CLI emitter, Help/About, support export, and AI evidence
//! surfaces; none of them re-derive per-pane micro-behavior.
//!
//! The matrix pins five things at once:
//!
//! 1. **Precedence** — a single ordered ladder ([`PrecedenceLayer`]) in which
//!    editing truth (the current debug frame, diagnostics, merge conflicts,
//!    review change markers, breakpoints, search matches, selection occurrences)
//!    outranks convenience metadata (code lenses, inlay hints, the inline
//!    completion ghost, hover cards, parameter hints) everywhere. The matrix
//!    proves no convenience layer ever outranks an editing-truth layer.
//! 2. **Class catalogs** — closed, stable vocabularies for decoration classes,
//!    code-lens classes, inlay-hint classes, completion source kinds (reusing
//!    [`AssistSourceLabelClass`]), signature-help states, snippet-session states
//!    (reusing [`SnippetSessionStateClass`]), and hover/peek modes.
//! 3. **The surface matrix** — for every claimed editor surface, exactly one
//!    [`SurfaceAssistCell`] per assist channel, carrying the degraded-state class
//!    the surface narrows that channel to and whether the channel stays
//!    keyboard-reachable.
//! 4. **Identity & lifecycle** — stable id prefixes and required lifecycle fields
//!    ([`IdentityContract`]) for completion sessions, hint descriptors, hover/peek
//!    cards, snippet sessions, and degraded assist states, defined once for all
//!    consumers.
//! 5. **Support / export minimums** — the fields each micro-surface record must
//!    carry into a support export ([`SupportExportMinimum`]), with no credential
//!    bodies or raw provider payloads.
//!
//! The matrix is static and deterministic: [`editor_assist_matrix`] builds the
//! one canonical record, and the checked-in fixture plus the replay gate freeze
//! it so the in-code matrix cannot drift from the published artifact. The matrix
//! carries the [`MatrixInvariant`]s it must satisfy and evaluates them over its
//! own data, so a structural regression flips an invariant to `holds = false`
//! rather than silently shipping.

use serde::{Deserialize, Serialize};

use crate::assist::{AssistSourceLabelClass, SnippetSessionStateClass};

/// Schema version for the editor-assist matrix record.
pub const M5_EDITOR_ASSIST_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the editor-assist matrix record.
pub const M5_EDITOR_ASSIST_SCHEMA_REF: &str = "schemas/editor/m5-editor-assist.schema.json";

/// Stable record-kind tag for the editor-assist matrix record.
pub const M5_EDITOR_ASSIST_RECORD_KIND: &str = "m5_editor_assist_matrix";

/// Stable id for the canonical editor-assist matrix.
pub const M5_EDITOR_ASSIST_MATRIX_ID: &str = "m5-editor-assist:matrix:0001";

/// Capture stamp for the canonical matrix. Held as a constant so the projection
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_EDITOR_ASSIST_AS_OF: &str = "2026-06-22T00:00:00Z";

// ---------------------------------------------------------------------------
// Truth tier + precedence ladder.
// ---------------------------------------------------------------------------

/// Whether a draw layer is protected editing truth or suppressible convenience
/// metadata. Editing truth always outranks convenience metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TruthTier {
    /// Diagnostics, debug/current-frame, conflict, review, search, and selection
    /// state. Never suppressed to make room for convenience chrome.
    EditingTruth,
    /// Code lenses, inlay hints, the inline completion ghost, hover cards, and
    /// parameter hints. Subordinate to editing truth and suppressible under
    /// constrained surfaces.
    ConvenienceMetadata,
}

impl TruthTier {
    /// Returns the stable schema token for this truth tier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditingTruth => "editing_truth",
            Self::ConvenienceMetadata => "convenience_metadata",
        }
    }
}

/// One layer in the editor draw / precedence ladder.
///
/// `rank` is the precedence index: lower ranks win overlap and are drawn closest
/// to editing truth. Every [`TruthTier::EditingTruth`] layer ranks above every
/// [`TruthTier::ConvenienceMetadata`] layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditorLayerClass {
    /// The current debug execution frame / instruction pointer.
    CurrentDebugFrame,
    /// Error diagnostics.
    ErrorDiagnostic,
    /// Merge / rebase conflict regions.
    MergeConflictRegion,
    /// Review change markers and inline review threads.
    ReviewChangeMarker,
    /// Warning diagnostics.
    WarningDiagnostic,
    /// Breakpoint gutter markers.
    BreakpointMarker,
    /// Info / hint diagnostics.
    InfoHintDiagnostic,
    /// Active search / find matches.
    SearchMatch,
    /// Selection and matching-occurrence highlights.
    SelectionOccurrence,
    /// Code-lens action rows.
    CodeLens,
    /// Inlay hints.
    InlayHint,
    /// The inline completion ghost / preview.
    InlineCompletionGhost,
    /// Hover / quick-info cards.
    HoverCard,
    /// Signature / parameter hint popups.
    ParameterHint,
}

impl EditorLayerClass {
    /// Precedence-ordered list of all layers, highest precedence first.
    pub const ALL: [Self; 14] = [
        Self::CurrentDebugFrame,
        Self::ErrorDiagnostic,
        Self::MergeConflictRegion,
        Self::ReviewChangeMarker,
        Self::WarningDiagnostic,
        Self::BreakpointMarker,
        Self::InfoHintDiagnostic,
        Self::SearchMatch,
        Self::SelectionOccurrence,
        Self::CodeLens,
        Self::InlayHint,
        Self::InlineCompletionGhost,
        Self::HoverCard,
        Self::ParameterHint,
    ];

    /// Returns the stable schema token for this layer.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentDebugFrame => "current_debug_frame",
            Self::ErrorDiagnostic => "error_diagnostic",
            Self::MergeConflictRegion => "merge_conflict_region",
            Self::ReviewChangeMarker => "review_change_marker",
            Self::WarningDiagnostic => "warning_diagnostic",
            Self::BreakpointMarker => "breakpoint_marker",
            Self::InfoHintDiagnostic => "info_hint_diagnostic",
            Self::SearchMatch => "search_match",
            Self::SelectionOccurrence => "selection_occurrence",
            Self::CodeLens => "code_lens",
            Self::InlayHint => "inlay_hint",
            Self::InlineCompletionGhost => "inline_completion_ghost",
            Self::HoverCard => "hover_card",
            Self::ParameterHint => "parameter_hint",
        }
    }

    /// Human-readable label for this layer.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CurrentDebugFrame => "Current debug frame",
            Self::ErrorDiagnostic => "Error diagnostics",
            Self::MergeConflictRegion => "Merge-conflict regions",
            Self::ReviewChangeMarker => "Review change markers",
            Self::WarningDiagnostic => "Warning diagnostics",
            Self::BreakpointMarker => "Breakpoint markers",
            Self::InfoHintDiagnostic => "Info / hint diagnostics",
            Self::SearchMatch => "Search matches",
            Self::SelectionOccurrence => "Selection and matching occurrences",
            Self::CodeLens => "Code lenses",
            Self::InlayHint => "Inlay hints",
            Self::InlineCompletionGhost => "Inline completion ghost",
            Self::HoverCard => "Hover / quick-info cards",
            Self::ParameterHint => "Signature / parameter hints",
        }
    }

    /// Truth tier this layer belongs to.
    pub const fn truth_tier(self) -> TruthTier {
        match self {
            Self::CurrentDebugFrame
            | Self::ErrorDiagnostic
            | Self::MergeConflictRegion
            | Self::ReviewChangeMarker
            | Self::WarningDiagnostic
            | Self::BreakpointMarker
            | Self::InfoHintDiagnostic
            | Self::SearchMatch
            | Self::SelectionOccurrence => TruthTier::EditingTruth,
            Self::CodeLens
            | Self::InlayHint
            | Self::InlineCompletionGhost
            | Self::HoverCard
            | Self::ParameterHint => TruthTier::ConvenienceMetadata,
        }
    }

    /// Precedence rank: the index of this layer in [`EditorLayerClass::ALL`].
    pub fn rank(self) -> u8 {
        Self::ALL
            .iter()
            .position(|layer| *layer == self)
            .expect("every layer is present in ALL") as u8
    }
}

/// One row of the precedence ladder, frozen into the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrecedenceLayer {
    /// Precedence rank; lower wins overlap.
    pub rank: u8,
    /// Stable layer token.
    pub layer: EditorLayerClass,
    /// Human-readable layer label.
    pub label: String,
    /// Truth tier this layer belongs to.
    pub truth_tier: TruthTier,
    /// Whether the layer may be suppressed under a constrained surface. Editing
    /// truth is never suppressed; convenience metadata may be.
    pub suppressible_under_constraint: bool,
    /// What the layer carries and why it sits where it does.
    pub disclosure: String,
}

// ---------------------------------------------------------------------------
// Class catalogs.
// ---------------------------------------------------------------------------

/// Decoration classes drawn in the gutter or inline. Each maps to an
/// editing-truth precedence layer; decorations are never convenience metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecorationClass {
    /// Inline diagnostic underline / squiggle.
    DiagnosticUnderline,
    /// Gutter diagnostic icon.
    DiagnosticGutterIcon,
    /// Current debug line highlight.
    DebugCurrentLine,
    /// Breakpoint gutter glyph.
    BreakpointGutter,
    /// Merge-conflict region band.
    MergeConflictRegion,
    /// Review change gutter band.
    ReviewChangeGutter,
    /// Search match highlight.
    SearchMatchHighlight,
    /// Selection / matching-occurrence highlight.
    SelectionOccurrenceHighlight,
    /// Bracket match highlight.
    BracketMatch,
    /// Inline diff change marker.
    InlineDiffMarker,
}

impl DecorationClass {
    /// All decoration classes, in catalog order.
    pub const ALL: [Self; 10] = [
        Self::DiagnosticUnderline,
        Self::DiagnosticGutterIcon,
        Self::DebugCurrentLine,
        Self::BreakpointGutter,
        Self::MergeConflictRegion,
        Self::ReviewChangeGutter,
        Self::SearchMatchHighlight,
        Self::SelectionOccurrenceHighlight,
        Self::BracketMatch,
        Self::InlineDiffMarker,
    ];

    /// Returns the stable schema token for this decoration class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiagnosticUnderline => "diagnostic_underline",
            Self::DiagnosticGutterIcon => "diagnostic_gutter_icon",
            Self::DebugCurrentLine => "debug_current_line",
            Self::BreakpointGutter => "breakpoint_gutter",
            Self::MergeConflictRegion => "merge_conflict_region",
            Self::ReviewChangeGutter => "review_change_gutter",
            Self::SearchMatchHighlight => "search_match_highlight",
            Self::SelectionOccurrenceHighlight => "selection_occurrence_highlight",
            Self::BracketMatch => "bracket_match",
            Self::InlineDiffMarker => "inline_diff_marker",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DiagnosticUnderline => "Diagnostic underline",
            Self::DiagnosticGutterIcon => "Diagnostic gutter icon",
            Self::DebugCurrentLine => "Current debug line",
            Self::BreakpointGutter => "Breakpoint gutter glyph",
            Self::MergeConflictRegion => "Merge-conflict region",
            Self::ReviewChangeGutter => "Review change gutter",
            Self::SearchMatchHighlight => "Search match highlight",
            Self::SelectionOccurrenceHighlight => "Selection / occurrence highlight",
            Self::BracketMatch => "Bracket match",
            Self::InlineDiffMarker => "Inline diff marker",
        }
    }

    /// The precedence layer that owns this decoration.
    pub const fn owning_layer(self) -> EditorLayerClass {
        match self {
            Self::DiagnosticUnderline | Self::DiagnosticGutterIcon => {
                EditorLayerClass::ErrorDiagnostic
            }
            Self::DebugCurrentLine => EditorLayerClass::CurrentDebugFrame,
            Self::BreakpointGutter => EditorLayerClass::BreakpointMarker,
            Self::MergeConflictRegion => EditorLayerClass::MergeConflictRegion,
            Self::ReviewChangeGutter | Self::InlineDiffMarker => {
                EditorLayerClass::ReviewChangeMarker
            }
            Self::SearchMatchHighlight => EditorLayerClass::SearchMatch,
            Self::SelectionOccurrenceHighlight | Self::BracketMatch => {
                EditorLayerClass::SelectionOccurrence
            }
        }
    }
}

/// Code-lens classes. The inline reference / action rows above declarations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CodeLensClass {
    /// Reference count for a symbol.
    ReferenceCount,
    /// Implementation count for a trait / interface.
    ImplementationCount,
    /// Run or debug a test or entry point.
    RunOrDebugAction,
    /// Test pass / fail status.
    TestStatus,
    /// Version-control authorship / blame.
    VcsAuthorship,
    /// AI explain / action entry point.
    AiExplainAction,
    /// Generated-source origin and regenerate route.
    GeneratedSourceOrigin,
}

impl CodeLensClass {
    /// All code-lens classes, in catalog order.
    pub const ALL: [Self; 7] = [
        Self::ReferenceCount,
        Self::ImplementationCount,
        Self::RunOrDebugAction,
        Self::TestStatus,
        Self::VcsAuthorship,
        Self::AiExplainAction,
        Self::GeneratedSourceOrigin,
    ];

    /// Returns the stable schema token for this code-lens class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReferenceCount => "reference_count",
            Self::ImplementationCount => "implementation_count",
            Self::RunOrDebugAction => "run_or_debug_action",
            Self::TestStatus => "test_status",
            Self::VcsAuthorship => "vcs_authorship",
            Self::AiExplainAction => "ai_explain_action",
            Self::GeneratedSourceOrigin => "generated_source_origin",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ReferenceCount => "Reference count",
            Self::ImplementationCount => "Implementation count",
            Self::RunOrDebugAction => "Run / debug action",
            Self::TestStatus => "Test status",
            Self::VcsAuthorship => "VCS authorship",
            Self::AiExplainAction => "AI explain action",
            Self::GeneratedSourceOrigin => "Generated-source origin",
        }
    }

    /// Whether this lens must carry an explicit AI source label when shown.
    pub const fn requires_ai_label(self) -> bool {
        matches!(self, Self::AiExplainAction)
    }
}

/// Inlay-hint classes. Inline non-editable annotations rendered between tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InlayHintClass {
    /// Parameter-name hints at call sites.
    ParameterName,
    /// Inferred-type hints for bindings.
    InferredType,
    /// Chained-call type hints.
    ChainedCallType,
    /// Enum-member discriminant value.
    EnumMemberValue,
    /// Implicit conversion / coercion hint.
    ImplicitConversion,
    /// AI-inferred annotation, always labeled as AI.
    AiInferred,
}

impl InlayHintClass {
    /// All inlay-hint classes, in catalog order.
    pub const ALL: [Self; 6] = [
        Self::ParameterName,
        Self::InferredType,
        Self::ChainedCallType,
        Self::EnumMemberValue,
        Self::ImplicitConversion,
        Self::AiInferred,
    ];

    /// Returns the stable schema token for this inlay-hint class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParameterName => "parameter_name",
            Self::InferredType => "inferred_type",
            Self::ChainedCallType => "chained_call_type",
            Self::EnumMemberValue => "enum_member_value",
            Self::ImplicitConversion => "implicit_conversion",
            Self::AiInferred => "ai_inferred",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ParameterName => "Parameter name",
            Self::InferredType => "Inferred type",
            Self::ChainedCallType => "Chained-call type",
            Self::EnumMemberValue => "Enum member value",
            Self::ImplicitConversion => "Implicit conversion",
            Self::AiInferred => "AI-inferred annotation",
        }
    }

    /// Whether this hint must carry an explicit AI source label when shown.
    pub const fn requires_ai_label(self) -> bool {
        matches!(self, Self::AiInferred)
    }
}

/// Signature-help lifecycle states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignatureHelpStateClass {
    /// No signature help is shown.
    Hidden,
    /// A single signature is shown.
    VisibleSingle,
    /// An overloaded signature set is shown with an active-overload index.
    VisibleOverloaded,
    /// A previous signature is shown while a refresh is pending.
    StalePendingRefresh,
    /// Signature help is unavailable on this surface.
    Unavailable,
}

impl SignatureHelpStateClass {
    /// All signature-help states, in catalog order.
    pub const ALL: [Self; 5] = [
        Self::Hidden,
        Self::VisibleSingle,
        Self::VisibleOverloaded,
        Self::StalePendingRefresh,
        Self::Unavailable,
    ];

    /// Returns the stable schema token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Hidden => "hidden",
            Self::VisibleSingle => "visible_single",
            Self::VisibleOverloaded => "visible_overloaded",
            Self::StalePendingRefresh => "stale_pending_refresh",
            Self::Unavailable => "unavailable",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Hidden => "Hidden",
            Self::VisibleSingle => "Visible (single)",
            Self::VisibleOverloaded => "Visible (overloaded)",
            Self::StalePendingRefresh => "Stale, refresh pending",
            Self::Unavailable => "Unavailable",
        }
    }
}

/// Hover and peek modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HoverPeekModeClass {
    /// Transient hover / quick-info card.
    HoverQuickInfo,
    /// Pinned hover card.
    HoverPinned,
    /// Inline peek of a definition.
    PeekDefinition,
    /// Inline peek of references.
    PeekReferences,
    /// Inline peek of implementations.
    PeekImplementations,
    /// Inline peek of a type definition.
    PeekTypeDefinition,
    /// Inline peek of a call hierarchy.
    PeekCallHierarchy,
}

impl HoverPeekModeClass {
    /// All hover/peek modes, in catalog order.
    pub const ALL: [Self; 7] = [
        Self::HoverQuickInfo,
        Self::HoverPinned,
        Self::PeekDefinition,
        Self::PeekReferences,
        Self::PeekImplementations,
        Self::PeekTypeDefinition,
        Self::PeekCallHierarchy,
    ];

    /// Returns the stable schema token for this mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoverQuickInfo => "hover_quick_info",
            Self::HoverPinned => "hover_pinned",
            Self::PeekDefinition => "peek_definition",
            Self::PeekReferences => "peek_references",
            Self::PeekImplementations => "peek_implementations",
            Self::PeekTypeDefinition => "peek_type_definition",
            Self::PeekCallHierarchy => "peek_call_hierarchy",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::HoverQuickInfo => "Hover quick-info",
            Self::HoverPinned => "Pinned hover",
            Self::PeekDefinition => "Peek definition",
            Self::PeekReferences => "Peek references",
            Self::PeekImplementations => "Peek implementations",
            Self::PeekTypeDefinition => "Peek type definition",
            Self::PeekCallHierarchy => "Peek call hierarchy",
        }
    }

    /// Whether this mode is an inline peek (versus a hover overlay).
    pub const fn is_peek(self) -> bool {
        matches!(
            self,
            Self::PeekDefinition
                | Self::PeekReferences
                | Self::PeekImplementations
                | Self::PeekTypeDefinition
                | Self::PeekCallHierarchy
        )
    }
}

// ---------------------------------------------------------------------------
// The surface matrix.
// ---------------------------------------------------------------------------

/// The editor surfaces the current product claims. These are the rows of the
/// matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditorSurfaceClass {
    /// A normal source-code file.
    CodeFile,
    /// A configuration file with schema-backed assist.
    ConfigFile,
    /// A notebook cell.
    NotebookCell,
    /// An HTTP / REST request editor.
    RequestEditor,
    /// A SQL editor.
    SqlEditor,
    /// A fenced code block in a docs / markdown pane.
    DocsCodeBlock,
    /// A generated file whose edits route through its generator.
    GeneratedFile,
    /// A protected-path file whose writes require review.
    ProtectedFile,
    /// A file whose semantic index is still building.
    PartialIndexState,
    /// A file open in large-file / restricted mode.
    LargeFileRestricted,
}

impl EditorSurfaceClass {
    /// All editor surfaces, in matrix-row order.
    pub const ALL: [Self; 10] = [
        Self::CodeFile,
        Self::ConfigFile,
        Self::NotebookCell,
        Self::RequestEditor,
        Self::SqlEditor,
        Self::DocsCodeBlock,
        Self::GeneratedFile,
        Self::ProtectedFile,
        Self::PartialIndexState,
        Self::LargeFileRestricted,
    ];

    /// Returns the stable schema token for this surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeFile => "code_file",
            Self::ConfigFile => "config_file",
            Self::NotebookCell => "notebook_cell",
            Self::RequestEditor => "request_editor",
            Self::SqlEditor => "sql_editor",
            Self::DocsCodeBlock => "docs_code_block",
            Self::GeneratedFile => "generated_file",
            Self::ProtectedFile => "protected_file",
            Self::PartialIndexState => "partial_index_state",
            Self::LargeFileRestricted => "large_file_restricted",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CodeFile => "Code file",
            Self::ConfigFile => "Config file",
            Self::NotebookCell => "Notebook cell",
            Self::RequestEditor => "Request editor",
            Self::SqlEditor => "SQL editor",
            Self::DocsCodeBlock => "Docs-code block",
            Self::GeneratedFile => "Generated file",
            Self::ProtectedFile => "Protected file",
            Self::PartialIndexState => "Partial-index state",
            Self::LargeFileRestricted => "Large-file / restricted mode",
        }
    }

    /// Whether this surface constrains assist relative to a normal code file.
    pub const fn is_constrained(self) -> bool {
        !matches!(self, Self::CodeFile | Self::ConfigFile)
    }

    /// Whether direct assist apply is blocked because writes route elsewhere
    /// (generated files regenerate; protected paths require review).
    pub const fn blocks_direct_apply(self) -> bool {
        matches!(self, Self::GeneratedFile | Self::ProtectedFile)
    }
}

/// The assist channels the matrix tracks. These are the columns of the matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistChannelClass {
    /// The completion list.
    Completion,
    /// Signature / parameter help.
    SignatureHelp,
    /// Snippet sessions.
    SnippetSession,
    /// Code lenses.
    CodeLens,
    /// Inlay hints.
    InlayHint,
    /// Hover / quick-info cards.
    Hover,
    /// Inline peek surfaces.
    Peek,
    /// Inline AI assist proposals.
    InlineAiAssist,
    /// Diagnostic / editing-truth decorations.
    Decoration,
}

impl AssistChannelClass {
    /// All assist channels, in matrix-column order.
    pub const ALL: [Self; 9] = [
        Self::Completion,
        Self::SignatureHelp,
        Self::SnippetSession,
        Self::CodeLens,
        Self::InlayHint,
        Self::Hover,
        Self::Peek,
        Self::InlineAiAssist,
        Self::Decoration,
    ];

    /// Returns the stable schema token for this channel.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Completion => "completion",
            Self::SignatureHelp => "signature_help",
            Self::SnippetSession => "snippet_session",
            Self::CodeLens => "code_lens",
            Self::InlayHint => "inlay_hint",
            Self::Hover => "hover",
            Self::Peek => "peek",
            Self::InlineAiAssist => "inline_ai_assist",
            Self::Decoration => "decoration",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Completion => "Completion",
            Self::SignatureHelp => "Signature help",
            Self::SnippetSession => "Snippet session",
            Self::CodeLens => "Code lens",
            Self::InlayHint => "Inlay hint",
            Self::Hover => "Hover",
            Self::Peek => "Peek",
            Self::InlineAiAssist => "Inline AI assist",
            Self::Decoration => "Decoration",
        }
    }

    /// Whether applying this channel mutates the buffer (so it must be blocked on
    /// surfaces that route writes elsewhere).
    pub const fn is_apply_capable(self) -> bool {
        matches!(
            self,
            Self::Completion | Self::SnippetSession | Self::InlineAiAssist
        )
    }

    /// Whether this channel resolves semantics from the project index (so it must
    /// narrow on partial-index and large-file surfaces).
    pub const fn is_semantic(self) -> bool {
        matches!(
            self,
            Self::Completion
                | Self::SignatureHelp
                | Self::CodeLens
                | Self::InlayHint
                | Self::Hover
                | Self::Peek
        )
    }
}

/// The degraded-state classes a surface can narrow a channel to. Defined once so
/// no pane invents its own degraded-state copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistDegradeClass {
    /// Full-fidelity assist with all sources.
    FullFidelity,
    /// Available but limited to a labeled fallback source (lexical, snippet,
    /// schema, or best-effort by detected language).
    SourceLabeledFallback,
    /// Shown for reading, but apply is blocked and disclosed because writes route
    /// elsewhere.
    ReadOnlyNoApply,
    /// Suppressed because the file is in large-file / restricted mode.
    SuppressedLargeFile,
    /// Narrowed to a labeled pending state while the semantic index builds.
    PendingPartialIndex,
    /// Not offered on this surface at all.
    BlockedUnavailable,
}

impl AssistDegradeClass {
    /// All degraded-state classes, in catalog order.
    pub const ALL: [Self; 6] = [
        Self::FullFidelity,
        Self::SourceLabeledFallback,
        Self::ReadOnlyNoApply,
        Self::SuppressedLargeFile,
        Self::PendingPartialIndex,
        Self::BlockedUnavailable,
    ];

    /// Returns the stable schema token for this degraded-state class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullFidelity => "full_fidelity",
            Self::SourceLabeledFallback => "source_labeled_fallback",
            Self::ReadOnlyNoApply => "read_only_no_apply",
            Self::SuppressedLargeFile => "suppressed_large_file",
            Self::PendingPartialIndex => "pending_partial_index",
            Self::BlockedUnavailable => "blocked_unavailable",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::FullFidelity => "Full fidelity",
            Self::SourceLabeledFallback => "Source-labeled fallback",
            Self::ReadOnlyNoApply => "Read-only (apply blocked)",
            Self::SuppressedLargeFile => "Suppressed (large file)",
            Self::PendingPartialIndex => "Pending (partial index)",
            Self::BlockedUnavailable => "Blocked / unavailable",
        }
    }

    /// Whether the channel offers something the user can act on or read.
    pub const fn is_offered(self) -> bool {
        !matches!(self, Self::BlockedUnavailable)
    }

    /// Whether the channel is at full fidelity.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::FullFidelity)
    }
}

/// One cell of the surface matrix: how a surface narrows a single assist channel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceAssistCell {
    /// Assist channel this cell describes.
    pub channel: AssistChannelClass,
    /// Degraded-state class the surface narrows the channel to.
    pub degrade_state: AssistDegradeClass,
    /// Whether the channel stays reachable by keyboard when offered.
    pub keyboard_reachable: bool,
    /// Disclosure copy shown to the user / support for this cell.
    pub disclosure: String,
}

/// One row of the surface matrix: a surface and its per-channel cells.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceAssistProfile {
    /// Surface this profile describes.
    pub surface: EditorSurfaceClass,
    /// Human-readable surface label.
    pub label: String,
    /// Whether the surface constrains assist relative to a code file.
    pub is_constrained: bool,
    /// Why and how the surface constrains assist.
    pub constraint_note: String,
    /// Exactly one cell per [`AssistChannelClass`], in channel order.
    pub cells: Vec<SurfaceAssistCell>,
}

impl SurfaceAssistProfile {
    /// Returns the cell for the given channel, when present.
    pub fn cell(&self, channel: AssistChannelClass) -> Option<&SurfaceAssistCell> {
        self.cells.iter().find(|cell| cell.channel == channel)
    }
}

// ---------------------------------------------------------------------------
// Identity + lifecycle contracts.
// ---------------------------------------------------------------------------

/// The micro-surface record kinds whose identity and lifecycle this matrix
/// freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MicroSurfaceKind {
    /// A completion session.
    CompletionSession,
    /// An inlay-hint or code-lens descriptor.
    HintDescriptor,
    /// A hover or peek card.
    HoverPeekCard,
    /// A snippet session.
    SnippetSession,
    /// A degraded assist-state record.
    DegradedAssistState,
}

impl MicroSurfaceKind {
    /// All micro-surface kinds, in catalog order.
    pub const ALL: [Self; 5] = [
        Self::CompletionSession,
        Self::HintDescriptor,
        Self::HoverPeekCard,
        Self::SnippetSession,
        Self::DegradedAssistState,
    ];

    /// Returns the stable schema token for this kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompletionSession => "completion_session",
            Self::HintDescriptor => "hint_descriptor",
            Self::HoverPeekCard => "hover_peek_card",
            Self::SnippetSession => "snippet_session",
            Self::DegradedAssistState => "degraded_assist_state",
        }
    }

    /// Stable id prefix every record of this kind uses.
    pub const fn id_prefix(self) -> &'static str {
        match self {
            Self::CompletionSession => "completion-session:",
            Self::HintDescriptor => "hint:",
            Self::HoverPeekCard => "hover-peek:",
            Self::SnippetSession => "snippet-session:",
            Self::DegradedAssistState => "assist-degrade:",
        }
    }

    /// Stable support-export record-kind tag for this kind.
    pub const fn export_record_kind(self) -> &'static str {
        match self {
            Self::CompletionSession => "completion_session_record",
            Self::HintDescriptor => "hint_descriptor_record",
            Self::HoverPeekCard => "hover_peek_card_record",
            Self::SnippetSession => "snippet_session_record",
            Self::DegradedAssistState => "degraded_assist_state_record",
        }
    }
}

/// Identity and lifecycle contract for one micro-surface kind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityContract {
    /// Micro-surface kind.
    pub kind: MicroSurfaceKind,
    /// Stable id prefix every record of this kind uses.
    pub id_prefix: String,
    /// Required lifecycle fields, in declaration order.
    pub required_lifecycle_fields: Vec<String>,
    /// What the kind identifies and why these fields are required.
    pub note: String,
}

/// Support-export minimum for one micro-surface record kind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportMinimum {
    /// Export record-kind tag.
    pub record_kind: String,
    /// Fields that must appear in a support export of this record kind.
    pub required_fields: Vec<String>,
    /// Whether the record excludes raw payloads / credential bodies.
    pub raw_payload_excluded: bool,
    /// What the export carries and what it must never carry.
    pub note: String,
}

// ---------------------------------------------------------------------------
// Catalog descriptor + invariants.
// ---------------------------------------------------------------------------

/// A uniform catalog entry for the flat class catalogs (decorations, code
/// lenses, inlay hints, completion source kinds, signature states, snippet
/// states, hover/peek modes).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassDescriptor {
    /// Stable class token.
    pub class_token: String,
    /// Human-readable label.
    pub label: String,
    /// What the class is and any honesty requirement attached to it.
    pub note: String,
}

/// One frozen invariant the matrix must satisfy, with the result of evaluating it
/// over the matrix's own data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// Human-readable statement of the invariant.
    pub statement: String,
    /// Whether the invariant holds on the built matrix.
    pub holds: bool,
}

// ---------------------------------------------------------------------------
// Top-level record.
// ---------------------------------------------------------------------------

/// The canonical, frozen, export-safe editor-assist micro-surface matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorAssistMatrix {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_editor_assist_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable matrix id.
    pub matrix_id: String,
    /// Capture stamp.
    pub as_of: String,
    /// The precedence ladder, highest precedence first.
    pub precedence_ladder: Vec<PrecedenceLayer>,
    /// Decoration class catalog.
    pub decoration_classes: Vec<ClassDescriptor>,
    /// Code-lens class catalog.
    pub code_lens_classes: Vec<ClassDescriptor>,
    /// Inlay-hint class catalog.
    pub inlay_hint_classes: Vec<ClassDescriptor>,
    /// Completion source-kind catalog (reuses the shared source-label classes).
    pub completion_source_kinds: Vec<ClassDescriptor>,
    /// Signature-help state catalog.
    pub signature_help_states: Vec<ClassDescriptor>,
    /// Snippet-session state catalog (reuses the shared snippet-state classes).
    pub snippet_session_states: Vec<ClassDescriptor>,
    /// Hover/peek mode catalog.
    pub hover_peek_modes: Vec<ClassDescriptor>,
    /// Degraded-state class catalog.
    pub degrade_states: Vec<ClassDescriptor>,
    /// Identity / lifecycle contracts per micro-surface kind.
    pub identity_contracts: Vec<IdentityContract>,
    /// The surface matrix, one profile per surface.
    pub surface_profiles: Vec<SurfaceAssistProfile>,
    /// Support / export minimums per micro-surface record kind.
    pub support_export_minimums: Vec<SupportExportMinimum>,
    /// Frozen invariants and whether each holds on this matrix.
    pub invariants: Vec<MatrixInvariant>,
    /// Whether the matrix and its catalogs are metadata-safe for support export.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl EditorAssistMatrix {
    /// Returns true when every frozen invariant holds on this matrix.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|invariant| invariant.holds)
    }

    /// Returns true when the matrix is metadata-safe for support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == M5_EDITOR_ASSIST_SCHEMA_REF
            && self.record_kind == M5_EDITOR_ASSIST_RECORD_KIND
            && self
                .support_export_minimums
                .iter()
                .all(|minimum| minimum.raw_payload_excluded)
    }

    /// Returns the surface profile for the given surface, when present.
    pub fn surface_profile(&self, surface: EditorSurfaceClass) -> Option<&SurfaceAssistProfile> {
        self.surface_profiles
            .iter()
            .find(|profile| profile.surface == surface)
    }

    /// Returns the matrix cell for a surface / channel pair, when present.
    pub fn cell(
        &self,
        surface: EditorSurfaceClass,
        channel: AssistChannelClass,
    ) -> Option<&SurfaceAssistCell> {
        self.surface_profile(surface)
            .and_then(|profile| profile.cell(channel))
    }
}

// ---------------------------------------------------------------------------
// Builder.
// ---------------------------------------------------------------------------

/// Builds the one canonical editor-assist matrix.
///
/// The build is deterministic and self-contained: it never opens a file, reads
/// the environment, or consults the clock. It assembles the precedence ladder,
/// the class catalogs, the identity contracts, the surface matrix, and the
/// support-export minimums, then evaluates every frozen invariant over the
/// assembled data so the record's `invariants[].holds` reflect real checks.
pub fn editor_assist_matrix() -> EditorAssistMatrix {
    let precedence_ladder = build_precedence_ladder();
    let decoration_classes = build_decoration_catalog();
    let code_lens_classes = build_code_lens_catalog();
    let inlay_hint_classes = build_inlay_hint_catalog();
    let completion_source_kinds = build_completion_source_catalog();
    let signature_help_states = build_signature_state_catalog();
    let snippet_session_states = build_snippet_state_catalog();
    let hover_peek_modes = build_hover_peek_catalog();
    let degrade_states = build_degrade_catalog();
    let identity_contracts = build_identity_contracts();
    let surface_profiles = build_surface_profiles();
    let support_export_minimums = build_support_export_minimums();

    let invariants =
        evaluate_invariants(&precedence_ladder, &surface_profiles, &identity_contracts);

    let qualified = invariants.iter().all(|invariant| invariant.holds);
    let summary = if qualified {
        format!(
            "Editor-assist matrix frozen: {surfaces} surfaces × {channels} channels, \
             {layers}-layer precedence ladder, all {invariants} invariants hold.",
            surfaces = surface_profiles.len(),
            channels = AssistChannelClass::ALL.len(),
            layers = precedence_ladder.len(),
            invariants = invariants.len(),
        )
    } else {
        let failed: Vec<&str> = invariants
            .iter()
            .filter(|invariant| !invariant.holds)
            .map(|invariant| invariant.invariant_id.as_str())
            .collect();
        format!(
            "Editor-assist matrix is inconsistent: failing invariants {}.",
            failed.join(", ")
        )
    };

    EditorAssistMatrix {
        record_kind: M5_EDITOR_ASSIST_RECORD_KIND.to_owned(),
        m5_editor_assist_schema_version: M5_EDITOR_ASSIST_SCHEMA_VERSION,
        schema_ref: M5_EDITOR_ASSIST_SCHEMA_REF.to_owned(),
        matrix_id: M5_EDITOR_ASSIST_MATRIX_ID.to_owned(),
        as_of: M5_EDITOR_ASSIST_AS_OF.to_owned(),
        precedence_ladder,
        decoration_classes,
        code_lens_classes,
        inlay_hint_classes,
        completion_source_kinds,
        signature_help_states,
        snippet_session_states,
        hover_peek_modes,
        degrade_states,
        identity_contracts,
        surface_profiles,
        support_export_minimums,
        invariants,
        raw_payload_excluded: true,
        summary,
    }
}

fn build_precedence_ladder() -> Vec<PrecedenceLayer> {
    EditorLayerClass::ALL
        .iter()
        .map(|layer| {
            let truth_tier = layer.truth_tier();
            PrecedenceLayer {
                rank: layer.rank(),
                layer: *layer,
                label: layer.label().to_owned(),
                truth_tier,
                suppressible_under_constraint: matches!(truth_tier, TruthTier::ConvenienceMetadata),
                disclosure: precedence_disclosure(*layer),
            }
        })
        .collect()
}

fn precedence_disclosure(layer: EditorLayerClass) -> String {
    match layer.truth_tier() {
        TruthTier::EditingTruth => format!(
            "{} is editing truth; it is never suppressed or outranked by convenience metadata.",
            layer.label()
        ),
        TruthTier::ConvenienceMetadata => format!(
            "{} is convenience metadata; it sits below editing truth and may be suppressed on constrained surfaces.",
            layer.label()
        ),
    }
}

fn build_decoration_catalog() -> Vec<ClassDescriptor> {
    DecorationClass::ALL
        .iter()
        .map(|class| ClassDescriptor {
            class_token: class.as_str().to_owned(),
            label: class.label().to_owned(),
            note: format!(
                "Editing-truth decoration owned by the {} precedence layer.",
                class.owning_layer().as_str()
            ),
        })
        .collect()
}

fn build_code_lens_catalog() -> Vec<ClassDescriptor> {
    CodeLensClass::ALL
        .iter()
        .map(|class| ClassDescriptor {
            class_token: class.as_str().to_owned(),
            label: class.label().to_owned(),
            note: if class.requires_ai_label() {
                "Convenience code lens; must carry an explicit AI source label.".to_owned()
            } else {
                "Convenience code lens; subordinate to editing truth.".to_owned()
            },
        })
        .collect()
}

fn build_inlay_hint_catalog() -> Vec<ClassDescriptor> {
    InlayHintClass::ALL
        .iter()
        .map(|class| ClassDescriptor {
            class_token: class.as_str().to_owned(),
            label: class.label().to_owned(),
            note: if class.requires_ai_label() {
                "Convenience inlay hint; must carry an explicit AI source label.".to_owned()
            } else {
                "Convenience inlay hint; non-editable annotation.".to_owned()
            },
        })
        .collect()
}

/// Completion source kinds reuse the shared [`AssistSourceLabelClass`] vocabulary
/// so no pane forks its own source-label terms.
fn build_completion_source_catalog() -> Vec<ClassDescriptor> {
    const SOURCE_KINDS: [AssistSourceLabelClass; 7] = [
        AssistSourceLabelClass::DeterministicLanguage,
        AssistSourceLabelClass::CachedFallback,
        AssistSourceLabelClass::SnippetOrigin,
        AssistSourceLabelClass::AiInlineAssist,
        AssistSourceLabelClass::ProjectGraph,
        AssistSourceLabelClass::FrameworkProvider,
        AssistSourceLabelClass::ToolAdapter,
    ];
    SOURCE_KINDS
        .iter()
        .map(|class| ClassDescriptor {
            class_token: class.as_str().to_owned(),
            label: completion_source_label(*class).to_owned(),
            note: if class.requires_visual_distinction() {
                "Completion source; must stay visually distinct from deterministic language."
                    .to_owned()
            } else {
                "Completion source kind shared with the assist source-label model.".to_owned()
            },
        })
        .collect()
}

fn completion_source_label(class: AssistSourceLabelClass) -> &'static str {
    match class {
        AssistSourceLabelClass::DeterministicLanguage => "Deterministic language",
        AssistSourceLabelClass::CachedFallback => "Cached / lexical fallback",
        AssistSourceLabelClass::SnippetOrigin => "Snippet origin",
        AssistSourceLabelClass::AiInlineAssist => "AI inline assist",
        AssistSourceLabelClass::ProjectGraph => "Project graph",
        AssistSourceLabelClass::FrameworkProvider => "Framework provider",
        AssistSourceLabelClass::ToolAdapter => "Tool adapter",
    }
}

fn build_signature_state_catalog() -> Vec<ClassDescriptor> {
    SignatureHelpStateClass::ALL
        .iter()
        .map(|class| ClassDescriptor {
            class_token: class.as_str().to_owned(),
            label: class.label().to_owned(),
            note: "Signature-help lifecycle state.".to_owned(),
        })
        .collect()
}

/// Snippet-session states reuse the shared [`SnippetSessionStateClass`] vocabulary.
fn build_snippet_state_catalog() -> Vec<ClassDescriptor> {
    const STATES: [SnippetSessionStateClass; 4] = [
        SnippetSessionStateClass::Inactive,
        SnippetSessionStateClass::Active,
        SnippetSessionStateClass::Exited,
        SnippetSessionStateClass::Cancelled,
    ];
    STATES
        .iter()
        .map(|class| ClassDescriptor {
            class_token: snippet_state_token(*class).to_owned(),
            label: snippet_state_label(*class).to_owned(),
            note: "Snippet-session lifecycle state shared with the assist snippet model."
                .to_owned(),
        })
        .collect()
}

fn snippet_state_token(class: SnippetSessionStateClass) -> &'static str {
    match class {
        SnippetSessionStateClass::Inactive => "inactive",
        SnippetSessionStateClass::Active => "active",
        SnippetSessionStateClass::Exited => "exited",
        SnippetSessionStateClass::Cancelled => "cancelled",
    }
}

fn snippet_state_label(class: SnippetSessionStateClass) -> &'static str {
    match class {
        SnippetSessionStateClass::Inactive => "Inactive",
        SnippetSessionStateClass::Active => "Active",
        SnippetSessionStateClass::Exited => "Exited",
        SnippetSessionStateClass::Cancelled => "Cancelled",
    }
}

fn build_hover_peek_catalog() -> Vec<ClassDescriptor> {
    HoverPeekModeClass::ALL
        .iter()
        .map(|class| ClassDescriptor {
            class_token: class.as_str().to_owned(),
            label: class.label().to_owned(),
            note: if class.is_peek() {
                "Inline peek; preserves source, provider, freshness, and raw-versus-rendered truth."
                    .to_owned()
            } else {
                "Hover card; preserves source, provider, freshness, and raw-versus-rendered truth."
                    .to_owned()
            },
        })
        .collect()
}

fn build_degrade_catalog() -> Vec<ClassDescriptor> {
    AssistDegradeClass::ALL
        .iter()
        .map(|class| ClassDescriptor {
            class_token: class.as_str().to_owned(),
            label: class.label().to_owned(),
            note: degrade_note(*class).to_owned(),
        })
        .collect()
}

fn degrade_note(class: AssistDegradeClass) -> &'static str {
    match class {
        AssistDegradeClass::FullFidelity => "Full assist with all sources available.",
        AssistDegradeClass::SourceLabeledFallback => {
            "Available but limited to a labeled fallback source."
        }
        AssistDegradeClass::ReadOnlyNoApply => "Shown for reading; apply is blocked and disclosed.",
        AssistDegradeClass::SuppressedLargeFile => "Suppressed in large-file / restricted mode.",
        AssistDegradeClass::PendingPartialIndex => {
            "Labeled pending while the semantic index builds."
        }
        AssistDegradeClass::BlockedUnavailable => "Not offered on this surface.",
    }
}

fn build_identity_contracts() -> Vec<IdentityContract> {
    MicroSurfaceKind::ALL
        .iter()
        .map(|kind| IdentityContract {
            kind: *kind,
            id_prefix: kind.id_prefix().to_owned(),
            required_lifecycle_fields: identity_fields(*kind),
            note: identity_note(*kind).to_owned(),
        })
        .collect()
}

fn identity_fields(kind: MicroSurfaceKind) -> Vec<String> {
    let fields: &[&str] = match kind {
        MicroSurfaceKind::CompletionSession => &[
            "session_id",
            "surface_class",
            "anchor_ref",
            "source_label_class",
            "degrade_state",
            "opened_at",
            "closed_at",
        ],
        MicroSurfaceKind::HintDescriptor => &[
            "hint_id",
            "hint_family",
            "owning_layer",
            "source_label_class",
            "resolved",
            "dismissed",
        ],
        MicroSurfaceKind::HoverPeekCard => &[
            "card_id",
            "mode_class",
            "source_label_class",
            "provider_id",
            "freshness",
            "raw_versus_rendered",
            "dismissed_at",
        ],
        MicroSurfaceKind::SnippetSession => &[
            "session_id",
            "state_class",
            "placeholder_index",
            "placeholder_count",
            "exit_route",
            "opened_at",
            "ended_at",
        ],
        MicroSurfaceKind::DegradedAssistState => &[
            "degrade_id",
            "surface_class",
            "channel_class",
            "degrade_state",
            "disclosure",
            "observed_at",
        ],
    };
    fields.iter().map(|field| (*field).to_owned()).collect()
}

fn identity_note(kind: MicroSurfaceKind) -> &'static str {
    match kind {
        MicroSurfaceKind::CompletionSession => {
            "A completion session; carries its source label and degraded state for its whole life."
        }
        MicroSurfaceKind::HintDescriptor => {
            "An inlay-hint or code-lens descriptor; names its owning precedence layer and source."
        }
        MicroSurfaceKind::HoverPeekCard => {
            "A hover or peek card; preserves source, provider, freshness, and raw-versus-rendered."
        }
        MicroSurfaceKind::SnippetSession => {
            "A snippet session; exposes placeholder progress and an explicit exit route."
        }
        MicroSurfaceKind::DegradedAssistState => {
            "A degraded assist-state record; names the surface, channel, and degraded-state class."
        }
    }
}

fn build_support_export_minimums() -> Vec<SupportExportMinimum> {
    let mut minimums: Vec<SupportExportMinimum> = MicroSurfaceKind::ALL
        .iter()
        .map(|kind| SupportExportMinimum {
            record_kind: kind.export_record_kind().to_owned(),
            required_fields: identity_fields(*kind),
            raw_payload_excluded: true,
            note: "Exports identity, source label, and degraded state only; no credential bodies \
                   or raw provider payloads."
                .to_owned(),
        })
        .collect();
    minimums.push(SupportExportMinimum {
        record_kind: M5_EDITOR_ASSIST_RECORD_KIND.to_owned(),
        required_fields: vec![
            "matrix_id".to_owned(),
            "as_of".to_owned(),
            "schema_ref".to_owned(),
            "summary".to_owned(),
        ],
        raw_payload_excluded: true,
        note: "The matrix itself is export-safe and carries no file contents.".to_owned(),
    });
    minimums
}

// ---------------------------------------------------------------------------
// Surface matrix construction.
// ---------------------------------------------------------------------------

fn build_surface_profiles() -> Vec<SurfaceAssistProfile> {
    EditorSurfaceClass::ALL
        .iter()
        .map(|surface| build_surface_profile(*surface))
        .collect()
}

/// The per-channel degraded-state policy for one surface. Keeping the policy in
/// one place is the whole point of the matrix: each surface narrows the same
/// channels through the same closed degraded-state vocabulary.
fn surface_channel_degrade(
    surface: EditorSurfaceClass,
    channel: AssistChannelClass,
) -> AssistDegradeClass {
    use AssistChannelClass as Ch;
    use AssistDegradeClass as D;
    use EditorSurfaceClass as S;

    // Decorations are editing truth: never suppressed except where the large-file
    // viewer cannot render the full decoration set, where the surviving error
    // decorations are explicitly labeled as reduced.
    if channel == Ch::Decoration {
        return match surface {
            S::LargeFileRestricted => D::SourceLabeledFallback,
            _ => D::FullFidelity,
        };
    }

    match surface {
        S::CodeFile => D::FullFidelity,
        S::ConfigFile => match channel {
            // Config files are schema-backed text: no run/reference lenses and no
            // cross-file peek, but full completion, signature, snippet, and hover.
            Ch::CodeLens | Ch::Peek => D::SourceLabeledFallback,
            _ => D::FullFidelity,
        },
        S::NotebookCell => match channel {
            // Per-cell language scope: cross-cell symbol resolution is best-effort.
            Ch::CodeLens | Ch::Peek => D::SourceLabeledFallback,
            _ => D::FullFidelity,
        },
        S::RequestEditor => match channel {
            // Variable / header assist from the request schema, not a full LSP.
            Ch::Completion | Ch::SignatureHelp | Ch::CodeLens | Ch::InlayHint | Ch::Hover => {
                D::SourceLabeledFallback
            }
            Ch::Peek => D::BlockedUnavailable,
            Ch::SnippetSession | Ch::InlineAiAssist => D::FullFidelity,
            Ch::Decoration => D::FullFidelity,
        },
        S::SqlEditor => match channel {
            // Dialect + introspected-schema backed; degraded, not full LSP.
            Ch::Completion
            | Ch::SignatureHelp
            | Ch::CodeLens
            | Ch::InlayHint
            | Ch::Hover
            | Ch::Peek => D::SourceLabeledFallback,
            Ch::SnippetSession | Ch::InlineAiAssist => D::FullFidelity,
            Ch::Decoration => D::FullFidelity,
        },
        S::DocsCodeBlock => match channel {
            // Best-effort by detected language with no project context.
            Ch::Completion | Ch::SignatureHelp | Ch::Hover => D::SourceLabeledFallback,
            Ch::CodeLens | Ch::InlayHint | Ch::Peek => D::BlockedUnavailable,
            Ch::SnippetSession | Ch::InlineAiAssist => D::FullFidelity,
            Ch::Decoration => D::FullFidelity,
        },
        S::GeneratedFile | S::ProtectedFile => match channel {
            // Apply-capable channels are read-only; reading channels stay full.
            Ch::Completion | Ch::SnippetSession | Ch::InlineAiAssist => D::ReadOnlyNoApply,
            Ch::SignatureHelp | Ch::CodeLens | Ch::InlayHint | Ch::Hover | Ch::Peek => {
                D::FullFidelity
            }
            Ch::Decoration => D::FullFidelity,
        },
        S::PartialIndexState => match channel {
            // Semantic channels are labeled pending; lexical/snippet/AI still run.
            Ch::Completion
            | Ch::SignatureHelp
            | Ch::CodeLens
            | Ch::InlayHint
            | Ch::Hover
            | Ch::Peek => D::PendingPartialIndex,
            Ch::InlineAiAssist => D::SourceLabeledFallback,
            Ch::SnippetSession => D::FullFidelity,
            Ch::Decoration => D::FullFidelity,
        },
        S::LargeFileRestricted => match channel {
            // All convenience assist is suppressed in large-file / restricted mode.
            Ch::Completion
            | Ch::SignatureHelp
            | Ch::SnippetSession
            | Ch::CodeLens
            | Ch::InlayHint
            | Ch::Hover
            | Ch::Peek
            | Ch::InlineAiAssist => D::SuppressedLargeFile,
            Ch::Decoration => D::SourceLabeledFallback,
        },
    }
}

fn build_surface_profile(surface: EditorSurfaceClass) -> SurfaceAssistProfile {
    let cells = AssistChannelClass::ALL
        .iter()
        .map(|channel| {
            let degrade_state = surface_channel_degrade(surface, *channel);
            SurfaceAssistCell {
                channel: *channel,
                degrade_state,
                keyboard_reachable: degrade_state.is_offered(),
                disclosure: cell_disclosure(surface, *channel, degrade_state),
            }
        })
        .collect();
    SurfaceAssistProfile {
        surface,
        label: surface.label().to_owned(),
        is_constrained: surface.is_constrained(),
        constraint_note: surface_constraint_note(surface).to_owned(),
        cells,
    }
}

fn surface_constraint_note(surface: EditorSurfaceClass) -> &'static str {
    match surface {
        EditorSurfaceClass::CodeFile => "Full-fidelity assist across every channel.",
        EditorSurfaceClass::ConfigFile => {
            "Schema-backed text; no run/reference lens and no cross-file peek."
        }
        EditorSurfaceClass::NotebookCell => {
            "Per-cell language scope; cross-cell symbol resolution is best-effort fallback."
        }
        EditorSurfaceClass::RequestEditor => {
            "Variable / header assist from the request schema; no symbol peek and no full LSP."
        }
        EditorSurfaceClass::SqlEditor => {
            "Dialect and introspected-schema backed; degraded relative to a full language server."
        }
        EditorSurfaceClass::DocsCodeBlock => {
            "Best-effort by detected language with no project context; lenses, inlay hints, and peek are unavailable."
        }
        EditorSurfaceClass::GeneratedFile => {
            "Edits route through the generator; assist apply is blocked and disclosed, reading stays full."
        }
        EditorSurfaceClass::ProtectedFile => {
            "Writes require protected-path review; assist apply is blocked and disclosed, reading stays full."
        }
        EditorSurfaceClass::PartialIndexState => {
            "Semantic results are pending until indexing completes; lexical, snippet, and AI fallback are labeled."
        }
        EditorSurfaceClass::LargeFileRestricted => {
            "Convenience assist is suppressed; only reduced editing-truth decorations remain."
        }
    }
}

fn cell_disclosure(
    surface: EditorSurfaceClass,
    channel: AssistChannelClass,
    degrade: AssistDegradeClass,
) -> String {
    match degrade {
        AssistDegradeClass::FullFidelity => {
            format!("{} is available at full fidelity here.", channel.label())
        }
        AssistDegradeClass::SourceLabeledFallback => format!(
            "{} runs on a labeled fallback source on the {}.",
            channel.label(),
            surface.label()
        ),
        AssistDegradeClass::ReadOnlyNoApply => format!(
            "{} is shown for reading on the {}; apply is blocked and disclosed.",
            channel.label(),
            surface.label()
        ),
        AssistDegradeClass::SuppressedLargeFile => format!(
            "{} is suppressed in large-file / restricted mode.",
            channel.label()
        ),
        AssistDegradeClass::PendingPartialIndex => format!(
            "{} is labeled pending until the semantic index finishes building.",
            channel.label()
        ),
        AssistDegradeClass::BlockedUnavailable => {
            format!(
                "{} is not offered on the {}.",
                channel.label(),
                surface.label()
            )
        }
    }
}

// ---------------------------------------------------------------------------
// Invariant evaluation.
// ---------------------------------------------------------------------------

fn evaluate_invariants(
    precedence_ladder: &[PrecedenceLayer],
    surface_profiles: &[SurfaceAssistProfile],
    identity_contracts: &[IdentityContract],
) -> Vec<MatrixInvariant> {
    vec![
        MatrixInvariant {
            invariant_id: "precedence_truth_outranks_convenience".to_owned(),
            statement:
                "Every editing-truth layer outranks every convenience-metadata layer in the precedence ladder."
                    .to_owned(),
            holds: precedence_truth_outranks_convenience(precedence_ladder),
        },
        MatrixInvariant {
            invariant_id: "every_surface_covers_every_channel".to_owned(),
            statement: "Each surface profile binds exactly one cell per assist channel.".to_owned(),
            holds: every_surface_covers_every_channel(surface_profiles),
        },
        MatrixInvariant {
            invariant_id: "constrained_surfaces_narrow_visibly".to_owned(),
            statement:
                "Every constrained surface narrows or blocks at least one assist channel through the shared degraded-state vocabulary."
                    .to_owned(),
            holds: constrained_surfaces_narrow_visibly(surface_profiles),
        },
        MatrixInvariant {
            invariant_id: "apply_blocked_where_writes_route_elsewhere".to_owned(),
            statement:
                "Generated and protected surfaces never expose full-fidelity apply on completion, snippet, or inline-AI channels."
                    .to_owned(),
            holds: apply_blocked_where_writes_route_elsewhere(surface_profiles),
        },
        MatrixInvariant {
            invariant_id: "large_file_suppresses_convenience_assist".to_owned(),
            statement:
                "Large-file / restricted mode suppresses every convenience assist channel and keeps only reduced decorations."
                    .to_owned(),
            holds: large_file_suppresses_convenience_assist(surface_profiles),
        },
        MatrixInvariant {
            invariant_id: "partial_index_pends_semantic_channels".to_owned(),
            statement:
                "Partial-index state labels every semantic channel pending rather than presenting it as full."
                    .to_owned(),
            holds: partial_index_pends_semantic_channels(surface_profiles),
        },
        MatrixInvariant {
            invariant_id: "offered_channels_stay_keyboard_reachable".to_owned(),
            statement: "Every offered cell stays keyboard-reachable; only blocked cells are not."
                .to_owned(),
            holds: offered_channels_stay_keyboard_reachable(surface_profiles),
        },
        MatrixInvariant {
            invariant_id: "decorations_are_editing_truth".to_owned(),
            statement: "Every decoration class is owned by an editing-truth precedence layer."
                .to_owned(),
            holds: decorations_are_editing_truth(),
        },
        MatrixInvariant {
            invariant_id: "identity_contracts_cover_every_micro_surface".to_owned(),
            statement:
                "Every micro-surface kind has an identity contract with an id prefix and required lifecycle fields."
                    .to_owned(),
            holds: identity_contracts_cover_every_micro_surface(identity_contracts),
        },
    ]
}

fn precedence_truth_outranks_convenience(ladder: &[PrecedenceLayer]) -> bool {
    let max_truth_rank = ladder
        .iter()
        .filter(|layer| layer.truth_tier == TruthTier::EditingTruth)
        .map(|layer| layer.rank)
        .max();
    let min_convenience_rank = ladder
        .iter()
        .filter(|layer| layer.truth_tier == TruthTier::ConvenienceMetadata)
        .map(|layer| layer.rank)
        .min();
    match (max_truth_rank, min_convenience_rank) {
        (Some(truth), Some(convenience)) => truth < convenience,
        _ => false,
    }
}

fn every_surface_covers_every_channel(profiles: &[SurfaceAssistProfile]) -> bool {
    if profiles.len() != EditorSurfaceClass::ALL.len() {
        return false;
    }
    profiles.iter().all(|profile| {
        AssistChannelClass::ALL
            .iter()
            .all(|channel| profile.cell(*channel).is_some())
            && profile.cells.len() == AssistChannelClass::ALL.len()
    })
}

fn constrained_surfaces_narrow_visibly(profiles: &[SurfaceAssistProfile]) -> bool {
    profiles
        .iter()
        .filter(|profile| profile.surface.is_constrained())
        .all(|profile| {
            profile
                .cells
                .iter()
                .any(|cell| !cell.degrade_state.is_full())
        })
}

fn apply_blocked_where_writes_route_elsewhere(profiles: &[SurfaceAssistProfile]) -> bool {
    profiles
        .iter()
        .filter(|profile| profile.surface.blocks_direct_apply())
        .all(|profile| {
            profile
                .cells
                .iter()
                .filter(|cell| cell.channel.is_apply_capable())
                .all(|cell| !cell.degrade_state.is_full())
        })
}

fn large_file_suppresses_convenience_assist(profiles: &[SurfaceAssistProfile]) -> bool {
    let Some(profile) = profiles
        .iter()
        .find(|profile| profile.surface == EditorSurfaceClass::LargeFileRestricted)
    else {
        return false;
    };
    profile.cells.iter().all(|cell| {
        if cell.channel == AssistChannelClass::Decoration {
            // Decorations are editing truth; they are reduced, not suppressed.
            !cell.degrade_state.is_full()
        } else {
            cell.degrade_state == AssistDegradeClass::SuppressedLargeFile
        }
    })
}

fn partial_index_pends_semantic_channels(profiles: &[SurfaceAssistProfile]) -> bool {
    let Some(profile) = profiles
        .iter()
        .find(|profile| profile.surface == EditorSurfaceClass::PartialIndexState)
    else {
        return false;
    };
    profile
        .cells
        .iter()
        .filter(|cell| cell.channel.is_semantic())
        .all(|cell| cell.degrade_state == AssistDegradeClass::PendingPartialIndex)
}

fn offered_channels_stay_keyboard_reachable(profiles: &[SurfaceAssistProfile]) -> bool {
    profiles.iter().all(|profile| {
        profile
            .cells
            .iter()
            .all(|cell| cell.keyboard_reachable == cell.degrade_state.is_offered())
    })
}

fn decorations_are_editing_truth() -> bool {
    DecorationClass::ALL
        .iter()
        .all(|class| class.owning_layer().truth_tier() == TruthTier::EditingTruth)
}

fn identity_contracts_cover_every_micro_surface(contracts: &[IdentityContract]) -> bool {
    MicroSurfaceKind::ALL.iter().all(|kind| {
        contracts.iter().any(|contract| {
            contract.kind == *kind
                && contract.id_prefix == kind.id_prefix()
                && !contract.required_lifecycle_fields.is_empty()
        })
    })
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the export-safe, human-readable lines for the editor-assist matrix.
///
/// This is the shared projection consumed by Help/About, the headless CLI
/// emitter, and support export, so they never clone matrix text from each other.
pub fn editor_assist_matrix_lines(matrix: &EditorAssistMatrix) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Editor-assist matrix — {} ({})",
        matrix.matrix_id, matrix.as_of
    ));
    lines.push(format!(
        "schema_ref={} version={}",
        matrix.schema_ref, matrix.m5_editor_assist_schema_version
    ));

    lines.push("Precedence ladder (highest first):".to_owned());
    for layer in &matrix.precedence_ladder {
        lines.push(format!(
            "  [{rank:02}] {layer} ({tier}) suppressible={suppressible}",
            rank = layer.rank,
            layer = layer.layer.as_str(),
            tier = layer.truth_tier.as_str(),
            suppressible = layer.suppressible_under_constraint,
        ));
    }

    lines.push("Surface matrix:".to_owned());
    for profile in &matrix.surface_profiles {
        lines.push(format!(
            "  {surface} (constrained={constrained}): {note}",
            surface = profile.surface.as_str(),
            constrained = profile.is_constrained,
            note = profile.constraint_note,
        ));
        for cell in &profile.cells {
            lines.push(format!(
                "    {channel} = {degrade} keyboard_reachable={reachable}",
                channel = cell.channel.as_str(),
                degrade = cell.degrade_state.as_str(),
                reachable = cell.keyboard_reachable,
            ));
        }
    }

    lines.push("Identity contracts:".to_owned());
    for contract in &matrix.identity_contracts {
        lines.push(format!(
            "  {kind} id_prefix={prefix} fields=[{fields}]",
            kind = contract.kind.as_str(),
            prefix = contract.id_prefix,
            fields = contract.required_lifecycle_fields.join(", "),
        ));
    }

    lines.push("Invariants:".to_owned());
    for invariant in &matrix.invariants {
        lines.push(format!(
            "  {id} holds={holds}",
            id = invariant.invariant_id,
            holds = invariant.holds,
        ));
    }

    lines.push(matrix.summary.clone());
    lines
}

#[cfg(test)]
mod tests;
