//! Canonical hover-card and documentation-peek truth model: transient hovercards,
//! documentation peeks, and pinned / open-in-tab / open-in-split peek promotion
//! bound into one inspectable contextual-inspection contract across the claimed
//! M5 inspection contexts.
//!
//! Where the [completion-row model](crate::m5_completion_rows) freezes the one
//! shared *suggestion row*, the [signature / snippet model](crate::m5_signature_snippet)
//! freezes the two protected *mid-typing* surfaces, and the
//! [editor-assist matrix](crate::m5_editor_assist) freezes the per-surface
//! degraded-state *policy*, this module freezes the contextual-inspection
//! surfaces — the **hover card** and the **documentation peek** — that enrich the
//! current editing moment without stealing focus or losing return context. Before
//! it, hover and peek were scattered: one pane let pointer hover be the only path
//! to a symbol's provenance, another let a peek silently retarget to a different
//! object when a richer provider answered later, a third styled a stale or
//! imported-snapshot doc exactly like a live authoritative one. This module folds
//! both into one governed inspection model that carries, for every context:
//!
//! 1. **Symbol / anchor identity that does not silently retarget** — every card
//!    embeds a [`HoverPeekTargetRef`] that pins the originally-resolved symbol,
//!    its source anchor, the navigation target it opens, and the return anchor it
//!    restores. The identity is locked and the card never retargets just because a
//!    richer provider answered later (the `target_identity_locked_no_silent_retarget`
//!    invariant). The target is referenced by id; this module does **not** define a
//!    second navigation-target model.
//! 2. **Source / provider / freshness provenance** — every card embeds the
//!    canonical [`AssistSourceDescriptor`], so provider identity, support posture,
//!    freshness, locality, and degraded state travel with the surface in both its
//!    transient and its pinned / promoted forms.
//! 3. **Mapping quality** — a [`MappingQualityClass`] states how well the anchor
//!    maps to what the card shows (exact, approximate, heuristic, unresolved), and
//!    anything inexact is disclosed.
//! 4. **Raw-versus-rendered truth** — a [`RawRenderedModeClass`] states whether the
//!    card shows raw source, a rendered preview, or both; when the two forms differ
//!    materially in meaning or safety the card offers a visible open-raw escape so
//!    a rendered preview is never the only readable form.
//! 5. **Inline non-live state** — a [`HoverPeekStateClass`] surfaces stale, partial,
//!    policy-limited, imported-snapshot, wrong-provider-fallback, and suppressed
//!    states *inline* with a non-color differentiator, instead of styling them like
//!    live authoritative docs.
//! 6. **Focus-preserving promotion** — a transient card promotes to a
//!    [`HoverPeekPresentationClass::Pinned`] card or a durable tab / split via a
//!    [`PeekPromotion`]; every promotion preserves the same provenance labels and
//!    the same return anchor, and pointer hover is never the only path (every card
//!    is keyboard-invocable).
//!
//! Each claimed inspection context resolves into a [`HoverPeekSnapshot`] that pins
//! its [`AssistDegradeClass`] posture and a visible label and resolves exactly one
//! representative [`HoverPeekCard`]. The build is static and deterministic:
//! [`hover_peek_model`] assembles the one canonical record, the checked-in fixture
//! plus the replay gate freeze it byte-for-byte, and the model proves its own
//! honesty invariants over its data. It carries no file contents, credential
//! bodies, or raw provider payloads, so support, AI, and migration surfaces can
//! consume it directly.

use serde::{Deserialize, Serialize};

use aureline_language::{
    RouterCompletenessClass, RouterDegradedStateClass, RouterFreshnessClass, RouterLocalityClass,
    RouterScopeClaimClass, RouterSupportClass, ScopeLimitClass,
};

use crate::assist::{AssistSourceDescriptor, AssistSourceFamily, AssistSourceLabelClass};
use crate::m5_editor_assist::{
    AssistDegradeClass, ClassDescriptor, EditorSurfaceClass, HoverPeekModeClass,
};

/// Schema version for the hover-peek model record.
pub const M5_HOVER_PEEK_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the hover-peek model record.
pub const M5_HOVER_PEEK_SCHEMA_REF: &str = "schemas/editor/m5-hover-peek.schema.json";

/// Stable record-kind tag for the hover-peek model record.
pub const M5_HOVER_PEEK_RECORD_KIND: &str = "m5_hover_peek_model";

/// Stable id for the canonical hover-peek model.
pub const M5_HOVER_PEEK_MODEL_ID: &str = "m5-hover-peek:model:0001";

/// Capture stamp for the canonical model. Held as a constant so the projection
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_HOVER_PEEK_AS_OF: &str = "2026-06-22T00:00:00Z";

const HOVER_INVOKE_COMMAND: &str = "command.editor.hover.show";
const HOVER_DISMISS_COMMAND: &str = "command.editor.hover.dismiss";
const RAW_ESCAPE_COMMAND: &str = "command.editor.hover.open_raw_source";
const PEEK_PIN_COMMAND: &str = "command.editor.peek.keep_open";
const PEEK_OPEN_TAB_COMMAND: &str = "command.editor.peek.open_in_tab";
const PEEK_OPEN_SPLIT_COMMAND: &str = "command.editor.peek.open_in_split";
const PEEK_DISMISS_RETURN_COMMAND: &str = "command.editor.peek.dismiss_and_return";

// ---------------------------------------------------------------------------
// Inspection context.
// ---------------------------------------------------------------------------

/// The contextual-inspection contexts where hover cards and peeks appear. The
/// first ten reuse the canonical [`EditorSurfaceClass`] file surfaces verbatim;
/// the last two are the additional read-only inspection contexts hover / peek also
/// serve (diff / review surfaces and graph-linked explainers) that the file-surface
/// catalog does not model. This is a superset of the editor surfaces, not a fork:
/// [`HoverPeekContextClass::base_editor_surface`] maps the shared contexts back to
/// their canonical surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HoverPeekContextClass {
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
    /// A diff / review surface (review threads, change markers).
    DiffReviewSurface,
    /// A graph-linked explainer card (codebase-understanding / relation graph).
    GraphLinkedExplainer,
}

impl HoverPeekContextClass {
    /// All inspection contexts, in matrix order.
    pub const ALL: [Self; 12] = [
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
        Self::DiffReviewSurface,
        Self::GraphLinkedExplainer,
    ];

    /// Returns the stable schema token for this context.
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
            Self::DiffReviewSurface => "diff_review_surface",
            Self::GraphLinkedExplainer => "graph_linked_explainer",
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
            Self::DiffReviewSurface => "Diff / review surface",
            Self::GraphLinkedExplainer => "Graph-linked explainer",
        }
    }

    /// The canonical editor surface this context reuses, when it is one of the
    /// shared file surfaces. The two inspection-only contexts return `None`.
    pub const fn base_editor_surface(self) -> Option<EditorSurfaceClass> {
        match self {
            Self::CodeFile => Some(EditorSurfaceClass::CodeFile),
            Self::ConfigFile => Some(EditorSurfaceClass::ConfigFile),
            Self::NotebookCell => Some(EditorSurfaceClass::NotebookCell),
            Self::RequestEditor => Some(EditorSurfaceClass::RequestEditor),
            Self::SqlEditor => Some(EditorSurfaceClass::SqlEditor),
            Self::DocsCodeBlock => Some(EditorSurfaceClass::DocsCodeBlock),
            Self::GeneratedFile => Some(EditorSurfaceClass::GeneratedFile),
            Self::ProtectedFile => Some(EditorSurfaceClass::ProtectedFile),
            Self::PartialIndexState => Some(EditorSurfaceClass::PartialIndexState),
            Self::LargeFileRestricted => Some(EditorSurfaceClass::LargeFileRestricted),
            Self::DiffReviewSurface | Self::GraphLinkedExplainer => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Raw-versus-rendered mode.
// ---------------------------------------------------------------------------

/// Whether a card shows raw source, a rendered preview, or both. When the two
/// forms differ materially in meaning or safety the card must keep them
/// distinguishable and offer a visible open-raw escape, so a rendered preview is
/// never the only readable form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RawRenderedModeClass {
    /// Only the raw source form is meaningful (e.g. a code signature).
    RawSourceOnly,
    /// Only the rendered preview form is meaningful.
    RenderedPreviewOnly,
    /// Both forms are available and the rendering is cosmetic — they do not differ
    /// materially in meaning or safety.
    RawAndRenderedEquivalent,
    /// Both forms are available and they differ materially in meaning or safety
    /// (e.g. a resolved request variable versus its raw template). The raw form
    /// must stay distinguishable and reachable via an open-raw escape.
    RawAndRenderedDistinct,
}

impl RawRenderedModeClass {
    /// All raw / rendered modes, in catalog order.
    pub const ALL: [Self; 4] = [
        Self::RawSourceOnly,
        Self::RenderedPreviewOnly,
        Self::RawAndRenderedEquivalent,
        Self::RawAndRenderedDistinct,
    ];

    /// Returns the stable schema token for this mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawSourceOnly => "raw_source_only",
            Self::RenderedPreviewOnly => "rendered_preview_only",
            Self::RawAndRenderedEquivalent => "raw_and_rendered_equivalent",
            Self::RawAndRenderedDistinct => "raw_and_rendered_distinct",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RawSourceOnly => "Raw source only",
            Self::RenderedPreviewOnly => "Rendered preview only",
            Self::RawAndRenderedEquivalent => "Raw and rendered (equivalent)",
            Self::RawAndRenderedDistinct => "Raw and rendered (materially different)",
        }
    }

    /// Returns true when both a raw and a rendered form are available.
    pub const fn offers_both(self) -> bool {
        matches!(
            self,
            Self::RawAndRenderedEquivalent | Self::RawAndRenderedDistinct
        )
    }

    /// Returns true when the raw and rendered forms differ materially in meaning or
    /// safety, so they must stay distinguishable.
    pub const fn materially_differs(self) -> bool {
        matches!(self, Self::RawAndRenderedDistinct)
    }

    /// Returns true when the card must expose a visible open-raw escape.
    pub const fn requires_raw_escape(self) -> bool {
        self.materially_differs()
    }
}

// ---------------------------------------------------------------------------
// Mapping quality.
// ---------------------------------------------------------------------------

/// How well the source anchor maps to the symbol / target the card shows. Anything
/// inexact is disclosed so an approximate or heuristic mapping never reads as an
/// exact one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MappingQualityClass {
    /// The anchor maps exactly to a single resolved symbol / target.
    Exact,
    /// The anchor maps to an approximate or best-effort location.
    Approximate,
    /// The anchor was resolved heuristically (lexical / detected-language).
    Heuristic,
    /// The anchor could not be resolved to a target.
    Unresolved,
}

impl MappingQualityClass {
    /// All mapping-quality classes, in catalog order.
    pub const ALL: [Self; 4] = [
        Self::Exact,
        Self::Approximate,
        Self::Heuristic,
        Self::Unresolved,
    ];

    /// Returns the stable schema token for this class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::Heuristic => "heuristic",
            Self::Unresolved => "unresolved",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Exact => "Exact",
            Self::Approximate => "Approximate",
            Self::Heuristic => "Heuristic",
            Self::Unresolved => "Unresolved",
        }
    }

    /// Returns true when this mapping must be disclosed (anything but exact).
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Exact)
    }
}

// ---------------------------------------------------------------------------
// Inline state.
// ---------------------------------------------------------------------------

/// The inline state a hover card or peek surfaces, so a stale, partial,
/// policy-limited, imported-snapshot, or wrong-provider result is never styled like
/// a live authoritative doc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HoverPeekStateClass {
    /// Live, authoritative content from a current provider.
    Live,
    /// A previous result shown while a refresh is pending.
    Stale,
    /// Partial content while the semantic index is still building.
    Partial,
    /// Content narrowed by policy (protected / restricted path).
    PolicyLimited,
    /// An imported snapshot rather than a live read (e.g. generated output).
    ImportedSnapshot,
    /// A labeled fallback from a different provider than the authoritative one.
    WrongProviderFallback,
    /// Suppressed because the file is in large-file / restricted mode.
    Suppressed,
}

impl HoverPeekStateClass {
    /// All inline states, in catalog order.
    pub const ALL: [Self; 7] = [
        Self::Live,
        Self::Stale,
        Self::Partial,
        Self::PolicyLimited,
        Self::ImportedSnapshot,
        Self::WrongProviderFallback,
        Self::Suppressed,
    ];

    /// Returns the stable schema token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Stale => "stale",
            Self::Partial => "partial",
            Self::PolicyLimited => "policy_limited",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::WrongProviderFallback => "wrong_provider_fallback",
            Self::Suppressed => "suppressed",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Live => "Live",
            Self::Stale => "Stale — refresh pending",
            Self::Partial => "Partial — index still building",
            Self::PolicyLimited => "Policy-limited",
            Self::ImportedSnapshot => "Imported snapshot",
            Self::WrongProviderFallback => "Fallback provider",
            Self::Suppressed => "Suppressed — large-file mode",
        }
    }

    /// Returns true when the state is live and authoritative.
    pub const fn is_authoritative_live(self) -> bool {
        matches!(self, Self::Live)
    }

    /// Returns true when the state must be disclosed inline with a labeled cue.
    pub const fn requires_inline_disclosure(self) -> bool {
        !self.is_authoritative_live()
    }

    /// Returns true when the state offers readable card content (everything but a
    /// suppressed card).
    pub const fn offers_content(self) -> bool {
        !matches!(self, Self::Suppressed)
    }
}

// ---------------------------------------------------------------------------
// Promotion path + presentation.
// ---------------------------------------------------------------------------

/// A path that promotes a transient card into a more durable surface, or dismisses
/// it back to the return anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PeekPromotionPathClass {
    /// Keep the card open / pinned in place.
    KeepOpenPinned,
    /// Open the target in a durable editor tab.
    OpenInTab,
    /// Open the target in a durable editor split.
    OpenInSplit,
    /// Dismiss the card and return to the originating anchor.
    DismissReturn,
}

impl PeekPromotionPathClass {
    /// All promotion paths, in catalog order.
    pub const ALL: [Self; 4] = [
        Self::KeepOpenPinned,
        Self::OpenInTab,
        Self::OpenInSplit,
        Self::DismissReturn,
    ];

    /// Returns the stable schema token for this path.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeepOpenPinned => "keep_open_pinned",
            Self::OpenInTab => "open_in_tab",
            Self::OpenInSplit => "open_in_split",
            Self::DismissReturn => "dismiss_return",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::KeepOpenPinned => "Keep open / pin",
            Self::OpenInTab => "Open in tab",
            Self::OpenInSplit => "Open in split",
            Self::DismissReturn => "Dismiss and return",
        }
    }

    /// Canonical command id for this path.
    pub const fn command_id(self) -> &'static str {
        match self {
            Self::KeepOpenPinned => PEEK_PIN_COMMAND,
            Self::OpenInTab => PEEK_OPEN_TAB_COMMAND,
            Self::OpenInSplit => PEEK_OPEN_SPLIT_COMMAND,
            Self::DismissReturn => PEEK_DISMISS_RETURN_COMMAND,
        }
    }

    /// Returns true when this path opens a durable tab / split surface.
    pub const fn is_durable(self) -> bool {
        matches!(self, Self::OpenInTab | Self::OpenInSplit)
    }
}

/// The presentation form a card is currently resolved in. A pinned or promoted card
/// preserves the same provenance labels and return anchor as its transient form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HoverPeekPresentationClass {
    /// A transient hover / peek card.
    Transient,
    /// A pinned card kept open in place.
    Pinned,
    /// Promoted into a durable editor tab.
    PromotedTab,
    /// Promoted into a durable editor split.
    PromotedSplit,
}

impl HoverPeekPresentationClass {
    /// All presentation classes, in catalog order.
    pub const ALL: [Self; 4] = [
        Self::Transient,
        Self::Pinned,
        Self::PromotedTab,
        Self::PromotedSplit,
    ];

    /// Returns the stable schema token for this class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Transient => "transient",
            Self::Pinned => "pinned",
            Self::PromotedTab => "promoted_tab",
            Self::PromotedSplit => "promoted_split",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Transient => "Transient",
            Self::Pinned => "Pinned",
            Self::PromotedTab => "Promoted to tab",
            Self::PromotedSplit => "Promoted to split",
        }
    }

    /// Returns true when the card is promoted into a durable tab / split surface.
    pub const fn is_durable(self) -> bool {
        matches!(self, Self::PromotedTab | Self::PromotedSplit)
    }

    /// Returns true when the card persists (pinned or promoted) rather than being
    /// transient.
    pub const fn is_persisted(self) -> bool {
        !matches!(self, Self::Transient)
    }
}

// ---------------------------------------------------------------------------
// Target reference + promotion record.
// ---------------------------------------------------------------------------

/// The symbol / anchor identity a card inspects, referenced by id. This does
/// **not** define a second navigation-target model: it carries the canonical
/// navigation-target and anchor refs by id and locks the identity so the card never
/// silently retargets to a different object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoverPeekTargetRef {
    /// Stable ref to the symbol the card inspects.
    pub symbol_ref: String,
    /// Stable ref to the source-side anchor the card was invoked from.
    pub source_anchor_ref: String,
    /// Stable ref to the canonical navigation target the card opens / peeks.
    pub navigation_target_ref: String,
    /// Stable ref to the return anchor restored when the card closes.
    pub return_anchor_ref: String,
    /// Whether the resolved identity is locked so a later, richer provider answer
    /// cannot silently retarget the card.
    pub identity_locked: bool,
}

impl HoverPeekTargetRef {
    /// Returns true when every ref is present and the identity is locked.
    pub fn is_coherent(&self) -> bool {
        self.identity_locked
            && !self.symbol_ref.trim().is_empty()
            && !self.source_anchor_ref.trim().is_empty()
            && !self.navigation_target_ref.trim().is_empty()
            && !self.return_anchor_ref.trim().is_empty()
    }
}

/// One promotion path resolved for a card, preserving its provenance and return
/// anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeekPromotion {
    /// Promotion path class.
    pub path_class: PeekPromotionPathClass,
    /// Canonical command id that triggers the promotion.
    pub command_id_ref: String,
    /// Human-readable label.
    pub label: String,
    /// Whether the path opens a durable tab / split surface.
    pub is_durable: bool,
    /// Whether the promotion preserves the same provider / source / freshness labels
    /// the transient card showed.
    pub preserves_source_labels: bool,
    /// The source descriptor id whose labels the promotion preserves.
    pub source_descriptor_id_ref: String,
    /// Whether the promotion preserves the return anchor / navigation continuity.
    pub preserves_return_anchor: bool,
    /// The return anchor ref preserved across the promotion.
    pub return_anchor_ref: String,
}

impl PeekPromotion {
    fn build(
        path_class: PeekPromotionPathClass,
        source_descriptor_id: &str,
        return_anchor_ref: &str,
    ) -> Self {
        Self {
            path_class,
            command_id_ref: path_class.command_id().to_owned(),
            label: path_class.label().to_owned(),
            is_durable: path_class.is_durable(),
            preserves_source_labels: true,
            source_descriptor_id_ref: source_descriptor_id.to_owned(),
            preserves_return_anchor: true,
            return_anchor_ref: return_anchor_ref.to_owned(),
        }
    }

    /// Returns true when the promotion preserves provenance: it keeps the same
    /// source labels and names the descriptor it preserves.
    pub fn preserves_provenance(&self) -> bool {
        self.preserves_source_labels && !self.source_descriptor_id_ref.trim().is_empty()
    }

    /// Returns true when the promotion preserves the return anchor.
    pub fn preserves_continuity(&self) -> bool {
        self.preserves_return_anchor && !self.return_anchor_ref.trim().is_empty()
    }
}

// ---------------------------------------------------------------------------
// Hover / peek card.
// ---------------------------------------------------------------------------

/// One hover card or documentation peek bound for an inspection context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoverPeekCard {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable card id.
    pub card_id: String,
    /// Hover / peek mode (reuses the shared hover/peek mode vocabulary).
    pub mode_class: HoverPeekModeClass,
    /// Inspection context this card belongs to.
    pub context_class: HoverPeekContextClass,
    /// Symbol / anchor identity, referenced by id.
    pub target: HoverPeekTargetRef,
    /// Source and provenance descriptor (provider id, support, freshness, locality).
    pub source: AssistSourceDescriptor,
    /// Inline state surfaced by the card.
    pub state_class: HoverPeekStateClass,
    /// Mapping quality from anchor to target.
    pub mapping_quality: MappingQualityClass,
    /// Whether an inexact mapping is disclosed.
    pub mapping_disclosed: bool,
    /// Raw-versus-rendered mode.
    pub raw_rendered_mode: RawRenderedModeClass,
    /// Command id for the open-raw escape, when the raw and rendered forms differ.
    pub raw_escape_command_id_ref: Option<String>,
    /// Plain-language summary of the rendered form, when offered.
    pub rendered_form_summary: Option<String>,
    /// Plain-language summary of the raw form, when offered.
    pub raw_form_summary: Option<String>,
    /// Presentation form the card is currently resolved in.
    pub presentation_class: HoverPeekPresentationClass,
    /// Promotion paths offered by the card.
    pub promotions: Vec<PeekPromotion>,
    /// Whether the card silently retargets when a richer provider answers later
    /// (must be false).
    pub retarget_on_later_provider: bool,
    /// Whether the card is invocable by keyboard (so pointer hover is never the only
    /// path).
    pub keyboard_invocable: bool,
    /// Command id that invokes the card from the keyboard.
    pub keyboard_command_id_ref: String,
    /// Command id that dismisses the card and returns to the anchor.
    pub dismiss_command_id_ref: String,
    /// Whether provider / source / freshness provenance is visible on the card.
    pub provenance_visible: bool,
    /// Whether a non-live state is disclosed inline.
    pub inline_state_disclosed: bool,
    /// Non-color differentiator for non-live / inexact / fallback states.
    pub non_color_differentiator: String,
    /// Accessible summary for screen readers.
    pub accessibility_label: String,
}

/// Initialization data for a [`HoverPeekCard`].
#[derive(Debug, Clone, PartialEq, Eq)]
struct HoverPeekCardInit {
    context: HoverPeekContextClass,
    card_id: String,
    mode_class: HoverPeekModeClass,
    source: AssistSourceDescriptor,
    state_class: HoverPeekStateClass,
    mapping_quality: MappingQualityClass,
    raw_rendered_mode: RawRenderedModeClass,
    presentation_class: HoverPeekPresentationClass,
    rendered_form_summary: Option<String>,
    raw_form_summary: Option<String>,
}

impl HoverPeekCard {
    /// Stable record-kind tag for hover / peek cards.
    pub const RECORD_KIND: &'static str = "m5_hover_peek_card";

    fn new(init: HoverPeekCardInit) -> Self {
        let context = init.context;
        let symbol_ref = format!("symbol:{}:inspected", context.as_str());
        let source_anchor_ref = format!("anchor:{}:invocation", context.as_str());
        let navigation_target_ref = format!("nav-target:{}:resolved", context.as_str());
        let return_anchor_ref = format!("return-anchor:{}", context.as_str());
        let target = HoverPeekTargetRef {
            symbol_ref,
            source_anchor_ref,
            navigation_target_ref,
            return_anchor_ref: return_anchor_ref.clone(),
            identity_locked: true,
        };

        let mapping_disclosed = init.mapping_quality.requires_disclosure();
        let inline_state_disclosed = init.state_class.requires_inline_disclosure();
        let raw_escape_command_id_ref = if init.raw_rendered_mode.requires_raw_escape() {
            Some(RAW_ESCAPE_COMMAND.to_owned())
        } else {
            None
        };

        let promotions = if init.state_class.offers_content() {
            PeekPromotionPathClass::ALL
                .iter()
                .map(|path| {
                    PeekPromotion::build(
                        *path,
                        &init.source.source_descriptor_id,
                        &return_anchor_ref,
                    )
                })
                .collect()
        } else {
            // A suppressed card has no content to promote; it still returns to its
            // anchor via the dismiss path.
            vec![PeekPromotion::build(
                PeekPromotionPathClass::DismissReturn,
                &init.source.source_descriptor_id,
                &return_anchor_ref,
            )]
        };

        let non_color_differentiator = match init.state_class {
            HoverPeekStateClass::Live if init.mapping_quality == MappingQualityClass::Exact => {
                "source label text".to_owned()
            }
            HoverPeekStateClass::Live => {
                format!("mapping badge + \"{}\" text", init.mapping_quality.label())
            }
            other => format!("state badge + \"{}\" text", other.label()),
        };

        let accessibility_label = build_accessibility_label(&init);

        Self {
            record_kind: Self::RECORD_KIND.to_owned(),
            card_id: format!("hover-peek:{}", context.as_str()),
            mode_class: init.mode_class,
            context_class: context,
            target,
            source: init.source,
            state_class: init.state_class,
            mapping_quality: init.mapping_quality,
            mapping_disclosed,
            raw_rendered_mode: init.raw_rendered_mode,
            raw_escape_command_id_ref,
            rendered_form_summary: init.rendered_form_summary,
            raw_form_summary: init.raw_form_summary,
            presentation_class: init.presentation_class,
            promotions,
            retarget_on_later_provider: false,
            keyboard_invocable: true,
            keyboard_command_id_ref: HOVER_INVOKE_COMMAND.to_owned(),
            dismiss_command_id_ref: HOVER_DISMISS_COMMAND.to_owned(),
            provenance_visible: true,
            inline_state_disclosed,
            non_color_differentiator,
            accessibility_label,
        }
    }

    /// Returns true when the card shows live authoritative content.
    pub fn is_live(&self) -> bool {
        self.state_class.is_authoritative_live()
    }

    /// Returns true when the card offers readable content (not suppressed).
    pub fn offers_content(&self) -> bool {
        self.state_class.offers_content()
    }

    /// Returns true when the card keeps its target identity locked and never
    /// silently retargets.
    pub fn target_identity_locked(&self) -> bool {
        self.target.is_coherent() && !self.retarget_on_later_provider
    }

    /// Returns true when provenance is visible and source-labeled.
    pub fn provenance_labeled(&self) -> bool {
        self.provenance_visible && !self.source.source_label.trim().is_empty()
    }

    /// Returns true when a non-live state is disclosed inline with a non-color cue.
    pub fn non_live_state_disclosed(&self) -> bool {
        if self.state_class.is_authoritative_live() {
            return true;
        }
        self.inline_state_disclosed && !self.non_color_differentiator.trim().is_empty()
    }

    /// Returns true when an inexact mapping is disclosed.
    pub fn mapping_disclosed_when_inexact(&self) -> bool {
        !self.mapping_quality.requires_disclosure() || self.mapping_disclosed
    }

    /// Returns true when a materially different raw form has a visible open-raw
    /// escape.
    pub fn raw_escape_when_distinct(&self) -> bool {
        if !self.raw_rendered_mode.materially_differs() {
            return true;
        }
        self.raw_escape_command_id_ref
            .as_ref()
            .is_some_and(|command| !command.trim().is_empty())
    }

    /// Returns true when every offered promotion preserves provenance and return
    /// continuity.
    pub fn promotions_preserve_provenance_and_continuity(&self) -> bool {
        self.promotions
            .iter()
            .all(|promotion| promotion.preserves_provenance() && promotion.preserves_continuity())
    }

    /// Returns true when the card offers every promotion path (only required when it
    /// has content to promote).
    pub fn offers_all_promotion_paths(&self) -> bool {
        if !self.offers_content() {
            return true;
        }
        PeekPromotionPathClass::ALL.iter().all(|path| {
            self.promotions
                .iter()
                .any(|promotion| promotion.path_class == *path)
        })
    }
}

fn build_accessibility_label(init: &HoverPeekCardInit) -> String {
    let source = &init.source.source_label;
    let mode = init.mode_class.label();
    if !init.state_class.offers_content() {
        return format!(
            "{mode} unavailable here ({state}); source {source}. Press the inspect key to retry.",
            state = init.state_class.label(),
        );
    }
    let state = if init.state_class.is_authoritative_live() {
        String::new()
    } else {
        format!(" {}.", init.state_class.label())
    };
    let mapping = if init.mapping_quality.requires_disclosure() {
        format!(" Mapping: {}.", init.mapping_quality.label())
    } else {
        String::new()
    };
    let raw = if init.raw_rendered_mode.materially_differs() {
        " Rendered preview shown; open raw source for the exact text.".to_owned()
    } else {
        String::new()
    };
    format!("{mode} from {source}.{state}{mapping}{raw}")
}

// ---------------------------------------------------------------------------
// Context snapshot.
// ---------------------------------------------------------------------------

/// One claimed inspection context resolved into its representative card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoverPeekSnapshot {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub m5_hover_peek_schema_version: u32,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Inspection context covered by this snapshot.
    pub context_class: HoverPeekContextClass,
    /// The canonical editor surface this context reuses, when applicable.
    pub base_editor_surface: Option<EditorSurfaceClass>,
    /// Workspace id covered by the snapshot.
    pub workspace_id: String,
    /// Document ref covered by the snapshot.
    pub document_ref: String,
    /// Language id resolved for the document.
    pub language_id: String,
    /// Degraded-state posture for the context.
    pub degrade_class: AssistDegradeClass,
    /// Visible degrade label.
    pub degrade_label: String,
    /// The representative hover / peek card for this context.
    pub card: HoverPeekCard,
    /// Whether the snapshot needs source / state / mapping disclosure.
    pub disclosure_required: bool,
    /// Accessible summary for screen readers.
    pub accessibility_summary: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl HoverPeekSnapshot {
    /// Stable record-kind tag for hover-peek snapshots.
    pub const RECORD_KIND: &'static str = "m5_hover_peek_snapshot";
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

/// One frozen honesty invariant the model must satisfy, with the result of
/// evaluating it over the model's own data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoverPeekInvariant {
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

/// The canonical, frozen, export-safe hover-card and documentation-peek model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HoverPeekModel {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_hover_peek_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable model id.
    pub model_id: String,
    /// Capture stamp.
    pub as_of: String,
    /// Hover / peek mode catalog (reuses the shared hover/peek mode vocabulary).
    pub mode_classes: Vec<ClassDescriptor>,
    /// Inspection-context catalog.
    pub context_classes: Vec<ClassDescriptor>,
    /// Inline-state catalog.
    pub state_classes: Vec<ClassDescriptor>,
    /// Mapping-quality catalog.
    pub mapping_quality_classes: Vec<ClassDescriptor>,
    /// Raw / rendered catalog.
    pub raw_rendered_classes: Vec<ClassDescriptor>,
    /// Promotion-path catalog.
    pub promotion_path_classes: Vec<ClassDescriptor>,
    /// Presentation-class catalog.
    pub presentation_classes: Vec<ClassDescriptor>,
    /// One snapshot per claimed inspection context.
    pub context_snapshots: Vec<HoverPeekSnapshot>,
    /// Frozen invariants and whether each holds on this model.
    pub invariants: Vec<HoverPeekInvariant>,
    /// Whether the model is metadata-safe for support export.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl HoverPeekModel {
    /// Returns true when every frozen invariant holds on this model.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|invariant| invariant.holds)
    }

    /// Returns true when the model is metadata-safe for support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == M5_HOVER_PEEK_SCHEMA_REF
            && self.record_kind == M5_HOVER_PEEK_RECORD_KIND
    }

    /// Returns the snapshot for the given context, when present.
    pub fn snapshot(&self, context: HoverPeekContextClass) -> Option<&HoverPeekSnapshot> {
        self.context_snapshots
            .iter()
            .find(|snapshot| snapshot.context_class == context)
    }

    /// Returns every card across every snapshot.
    pub fn all_cards(&self) -> impl Iterator<Item = &HoverPeekCard> {
        self.context_snapshots.iter().map(|snapshot| &snapshot.card)
    }
}

// ---------------------------------------------------------------------------
// Helpers.
// ---------------------------------------------------------------------------

fn document_ref_for(context: HoverPeekContextClass) -> String {
    let path = match context {
        HoverPeekContextClass::CodeFile => "src/render.rs",
        HoverPeekContextClass::ConfigFile => "Cargo.toml",
        HoverPeekContextClass::NotebookCell => "analysis.ipynb#cell-3",
        HoverPeekContextClass::RequestEditor => "requests/list_users.http",
        HoverPeekContextClass::SqlEditor => "queries/active_users.sql",
        HoverPeekContextClass::DocsCodeBlock => "docs/guide.md#example-2",
        HoverPeekContextClass::GeneratedFile => "target/generated/schema.rs",
        HoverPeekContextClass::ProtectedFile => "infra/policy.toml",
        HoverPeekContextClass::PartialIndexState => "src/pipeline.rs",
        HoverPeekContextClass::LargeFileRestricted => "logs/trace.log",
        HoverPeekContextClass::DiffReviewSurface => "review/change-42.diff",
        HoverPeekContextClass::GraphLinkedExplainer => "graph/explainer/render_path",
    };
    format!("doc:{path}")
}

const fn language_id_for(context: HoverPeekContextClass) -> &'static str {
    match context {
        HoverPeekContextClass::CodeFile
        | HoverPeekContextClass::GeneratedFile
        | HoverPeekContextClass::PartialIndexState
        | HoverPeekContextClass::DiffReviewSurface => "rust",
        HoverPeekContextClass::ConfigFile | HoverPeekContextClass::ProtectedFile => "toml",
        HoverPeekContextClass::NotebookCell => "python",
        HoverPeekContextClass::RequestEditor => "http",
        HoverPeekContextClass::SqlEditor => "sql",
        HoverPeekContextClass::DocsCodeBlock => "markdown",
        HoverPeekContextClass::LargeFileRestricted => "log",
        HoverPeekContextClass::GraphLinkedExplainer => "graph",
    }
}

#[allow(clippy::too_many_arguments)]
fn source(
    context: HoverPeekContextClass,
    family: AssistSourceFamily,
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
        source_descriptor_id: format!("hover-peek-source:{}:{}", context.as_str(), family.as_str()),
        source_family: family,
        source_label_class: AssistSourceLabelClass::from_source_family(family),
        source_label: provider_label.to_owned(),
        provider_id: provider_id.map(str::to_owned),
        router_decision_ref: provider_id
            .map(|id| format!("router-decision:{}:{id}", context.as_str())),
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

/// Builds the one canonical hover-card and documentation-peek model.
///
/// The build is deterministic and self-contained: it materializes one
/// [`HoverPeekSnapshot`] per claimed inspection context, each resolving a
/// representative [`HoverPeekCard`] that embeds the canonical
/// [`AssistSourceDescriptor`] for provenance and pins its symbol / anchor identity
/// by ref, and evaluates every frozen honesty invariant over the assembled data so
/// the record's `invariants[].holds` reflect real checks.
pub fn hover_peek_model() -> HoverPeekModel {
    let context_snapshots = build_context_snapshots();
    let invariants = evaluate_invariants(&context_snapshots);

    let qualified = invariants.iter().all(|invariant| invariant.holds);
    let summary = if qualified {
        format!(
            "Hover-peek model frozen: {contexts} inspection contexts each resolve a hover / peek \
             card. Every card is keyboard-invocable and provenance-labeled, never silently \
             retargets, surfaces stale / partial / policy-limited / imported-snapshot / \
             wrong-provider states inline, keeps raw and rendered forms distinguishable when they \
             differ materially, and preserves the same provider / source / freshness labels and \
             return anchor when pinned or promoted into a tab or split. All {invariants} \
             invariants hold.",
            contexts = context_snapshots.len(),
            invariants = invariants.len(),
        )
    } else {
        format!(
            "Hover-peek model INVALID: {failing} of {total} invariants do not hold.",
            failing = invariants.iter().filter(|i| !i.holds).count(),
            total = invariants.len(),
        )
    };

    HoverPeekModel {
        record_kind: M5_HOVER_PEEK_RECORD_KIND.to_owned(),
        m5_hover_peek_schema_version: M5_HOVER_PEEK_SCHEMA_VERSION,
        schema_ref: M5_HOVER_PEEK_SCHEMA_REF.to_owned(),
        model_id: M5_HOVER_PEEK_MODEL_ID.to_owned(),
        as_of: M5_HOVER_PEEK_AS_OF.to_owned(),
        mode_classes: build_mode_catalog(),
        context_classes: build_context_catalog(),
        state_classes: build_state_catalog(),
        mapping_quality_classes: build_mapping_quality_catalog(),
        raw_rendered_classes: build_raw_rendered_catalog(),
        promotion_path_classes: build_promotion_path_catalog(),
        presentation_classes: build_presentation_catalog(),
        context_snapshots,
        invariants,
        raw_payload_excluded: true,
        summary,
    }
}

/// Builds the human-readable projection of the model for support and headless use.
pub fn hover_peek_model_lines(model: &HoverPeekModel) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Hover-peek model — {} ({})",
        model.model_id, model.as_of
    ));
    lines.push(format!(
        "schema_ref={} version={}",
        model.schema_ref, model.m5_hover_peek_schema_version
    ));

    lines.push("Context snapshots:".to_owned());
    for snapshot in &model.context_snapshots {
        lines.push(format!(
            "  {context}: {degrade} ({label}) — disclosure={disclosure}",
            context = snapshot.context_class.as_str(),
            degrade = snapshot.degrade_class.as_str(),
            label = snapshot.degrade_label,
            disclosure = snapshot.disclosure_required,
        ));
        let card = &snapshot.card;
        lines.push(format!(
            "    card: mode={mode} state={state} mapping={mapping} raw_rendered={raw} \
             presentation={presentation} source={source} promotions={promotions}",
            mode = card.mode_class.as_str(),
            state = card.state_class.as_str(),
            mapping = card.mapping_quality.as_str(),
            raw = card.raw_rendered_mode.as_str(),
            presentation = card.presentation_class.as_str(),
            source = card.source.source_label,
            promotions = card.promotions.len(),
        ));
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

fn build_mode_catalog() -> Vec<ClassDescriptor> {
    HoverPeekModeClass::ALL
        .iter()
        .map(|mode| {
            let note = if mode.is_peek() {
                "Inline peek; preserves source, provider, freshness, and raw-versus-rendered truth."
            } else {
                "Hover card; preserves source, provider, freshness, and raw-versus-rendered truth."
            };
            class_descriptor(mode.as_str(), mode.label(), note)
        })
        .collect()
}

fn build_context_catalog() -> Vec<ClassDescriptor> {
    HoverPeekContextClass::ALL
        .iter()
        .map(|context| {
            let note = match context.base_editor_surface() {
                Some(surface) => {
                    format!("Reuses the canonical {} editor surface.", surface.as_str())
                }
                None => "Inspection-only context not modeled by the editor file-surface catalog."
                    .to_owned(),
            };
            class_descriptor(context.as_str(), context.label(), &note)
        })
        .collect()
}

fn build_state_catalog() -> Vec<ClassDescriptor> {
    HoverPeekStateClass::ALL
        .iter()
        .map(|state| {
            let note = if state.is_authoritative_live() {
                "Live authoritative content."
            } else if state.offers_content() {
                "Non-live; disclosed inline with a labeled cue, never styled like live docs."
            } else {
                "Suppressed; the card stays keyboard reachable and discloses its reason."
            };
            class_descriptor(state.as_str(), state.label(), note)
        })
        .collect()
}

fn build_mapping_quality_catalog() -> Vec<ClassDescriptor> {
    MappingQualityClass::ALL
        .iter()
        .map(|mapping| {
            let note = if mapping.requires_disclosure() {
                "Inexact mapping; disclosed so it never reads as exact."
            } else {
                "Exact mapping from anchor to a single resolved target."
            };
            class_descriptor(mapping.as_str(), mapping.label(), note)
        })
        .collect()
}

fn build_raw_rendered_catalog() -> Vec<ClassDescriptor> {
    RawRenderedModeClass::ALL
        .iter()
        .map(|mode| {
            let note = if mode.materially_differs() {
                "Raw and rendered differ materially; the card offers a visible open-raw escape."
            } else if mode.offers_both() {
                "Raw and rendered both available; rendering is cosmetic."
            } else {
                "Single readable form."
            };
            class_descriptor(mode.as_str(), mode.label(), note)
        })
        .collect()
}

fn build_promotion_path_catalog() -> Vec<ClassDescriptor> {
    PeekPromotionPathClass::ALL
        .iter()
        .map(|path| {
            let note = if path.is_durable() {
                "Promotes into a durable tab / split, preserving provenance and return anchor."
            } else {
                "Keeps or returns the card, preserving provenance and return anchor."
            };
            class_descriptor(path.as_str(), path.label(), note)
        })
        .collect()
}

fn build_presentation_catalog() -> Vec<ClassDescriptor> {
    HoverPeekPresentationClass::ALL
        .iter()
        .map(|presentation| {
            let note = if presentation.is_persisted() {
                "Persisted form; preserves the same provenance labels and return anchor."
            } else {
                "Transient form."
            };
            class_descriptor(presentation.as_str(), presentation.label(), note)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Snapshot assembly.
// ---------------------------------------------------------------------------

struct SnapshotSpec {
    context: HoverPeekContextClass,
    workspace_id: &'static str,
    degrade_class: AssistDegradeClass,
    degrade_label: &'static str,
    card: HoverPeekCard,
}

fn assemble_snapshot(spec: SnapshotSpec) -> HoverPeekSnapshot {
    let context = spec.context;
    let card = spec.card;
    let disclosure_required = spec.degrade_class != AssistDegradeClass::FullFidelity
        || card.inline_state_disclosed
        || card.mapping_disclosed
        || card.raw_rendered_mode.materially_differs();

    let accessibility_summary = format!(
        "{context}: {mode} from {source} ({state}).",
        context = context.label(),
        mode = card.mode_class.label(),
        source = card.source.source_label,
        state = card.state_class.label(),
    );
    let export_safe_summary = format!(
        "{context} resolves a {mode} card; state {state}, mapping {mapping}, posture {posture}.",
        context = context.as_str(),
        mode = card.mode_class.as_str(),
        state = card.state_class.as_str(),
        mapping = card.mapping_quality.as_str(),
        posture = spec.degrade_class.as_str(),
    );

    HoverPeekSnapshot {
        record_kind: HoverPeekSnapshot::RECORD_KIND.to_owned(),
        m5_hover_peek_schema_version: M5_HOVER_PEEK_SCHEMA_VERSION,
        snapshot_id: format!("hover-peek:{}", context.as_str()),
        context_class: context,
        base_editor_surface: context.base_editor_surface(),
        workspace_id: spec.workspace_id.to_owned(),
        document_ref: document_ref_for(context),
        language_id: language_id_for(context).to_owned(),
        degrade_class: spec.degrade_class,
        degrade_label: spec.degrade_label.to_owned(),
        card,
        disclosure_required,
        accessibility_summary,
        export_safe_summary,
    }
}

fn build_context_snapshots() -> Vec<HoverPeekSnapshot> {
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
        build_diff_review_snapshot(),
        build_graph_explainer_snapshot(),
    ]
}

fn build_code_file_snapshot() -> HoverPeekSnapshot {
    let context = HoverPeekContextClass::CodeFile;
    let card = HoverPeekCard::new(HoverPeekCardInit {
        context,
        card_id: String::new(),
        mode_class: HoverPeekModeClass::HoverQuickInfo,
        source: source(
            context,
            AssistSourceFamily::LanguageServer,
            Some("rust-analyzer"),
            "rust-analyzer",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::AuthoritativeLive,
            RouterScopeClaimClass::WholeWorkspace,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalSidecar,
            RouterDegradedStateClass::None,
            Vec::new(),
            "Live quick-info hover for a resolved symbol from the language server.",
        ),
        state_class: HoverPeekStateClass::Live,
        mapping_quality: MappingQualityClass::Exact,
        raw_rendered_mode: RawRenderedModeClass::RawSourceOnly,
        presentation_class: HoverPeekPresentationClass::Transient,
        rendered_form_summary: None,
        raw_form_summary: Some("Signature and doc comment for the hovered symbol.".to_owned()),
    });
    assemble_snapshot(SnapshotSpec {
        context,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::FullFidelity,
        degrade_label: "Full fidelity",
        card,
    })
}

fn build_config_file_snapshot() -> HoverPeekSnapshot {
    let context = HoverPeekContextClass::ConfigFile;
    let card = HoverPeekCard::new(HoverPeekCardInit {
        context,
        card_id: String::new(),
        mode_class: HoverPeekModeClass::HoverQuickInfo,
        source: source(
            context,
            AssistSourceFamily::FrameworkPack,
            Some("schema-pack:cargo"),
            "Schema pack",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::AuthoritativeLive,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalInProcess,
            RouterDegradedStateClass::None,
            Vec::new(),
            "Schema-backed hover for a config key; raw key and rendered description agree.",
        ),
        state_class: HoverPeekStateClass::Live,
        mapping_quality: MappingQualityClass::Exact,
        raw_rendered_mode: RawRenderedModeClass::RawAndRenderedEquivalent,
        presentation_class: HoverPeekPresentationClass::Transient,
        rendered_form_summary: Some("Rendered schema description of the key.".to_owned()),
        raw_form_summary: Some("Raw schema entry for the key.".to_owned()),
    });
    assemble_snapshot(SnapshotSpec {
        context,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::FullFidelity,
        degrade_label: "Full fidelity",
        card,
    })
}

fn build_notebook_cell_snapshot() -> HoverPeekSnapshot {
    let context = HoverPeekContextClass::NotebookCell;
    // A peek-definition promoted into a durable split: provenance and return anchor
    // must survive the promotion.
    let card = HoverPeekCard::new(HoverPeekCardInit {
        context,
        card_id: String::new(),
        mode_class: HoverPeekModeClass::PeekDefinition,
        source: source(
            context,
            AssistSourceFamily::LanguageServer,
            Some("pyright"),
            "Pyright",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::AuthoritativeLive,
            RouterScopeClaimClass::NotebookCell,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalInProcess,
            RouterDegradedStateClass::None,
            Vec::new(),
            "Cell-scoped peek definition promoted into a split while keeping its provenance.",
        ),
        state_class: HoverPeekStateClass::Live,
        mapping_quality: MappingQualityClass::Exact,
        raw_rendered_mode: RawRenderedModeClass::RawSourceOnly,
        presentation_class: HoverPeekPresentationClass::PromotedSplit,
        rendered_form_summary: None,
        raw_form_summary: Some("Definition body for the cell symbol.".to_owned()),
    });
    assemble_snapshot(SnapshotSpec {
        context,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::FullFidelity,
        degrade_label: "Full fidelity",
        card,
    })
}

fn build_request_editor_snapshot() -> HoverPeekSnapshot {
    let context = HoverPeekContextClass::RequestEditor;
    // Raw template versus resolved variable differ materially: the card must keep
    // both distinguishable and offer an open-raw escape.
    let card = HoverPeekCard::new(HoverPeekCardInit {
        context,
        card_id: String::new(),
        mode_class: HoverPeekModeClass::HoverQuickInfo,
        source: source(
            context,
            AssistSourceFamily::FrameworkPack,
            Some("http-template"),
            "Request template helpers",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::AuthoritativeLive,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalInProcess,
            RouterDegradedStateClass::None,
            Vec::new(),
            "Hover over a request variable; the resolved value differs from its raw template.",
        ),
        state_class: HoverPeekStateClass::Live,
        mapping_quality: MappingQualityClass::Exact,
        raw_rendered_mode: RawRenderedModeClass::RawAndRenderedDistinct,
        presentation_class: HoverPeekPresentationClass::Transient,
        rendered_form_summary: Some("Resolved value of the request variable.".to_owned()),
        raw_form_summary: Some("Raw `{{template}}` expression as written.".to_owned()),
    });
    assemble_snapshot(SnapshotSpec {
        context,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::FullFidelity,
        degrade_label: "Full fidelity",
        card,
    })
}

fn build_sql_editor_snapshot() -> HoverPeekSnapshot {
    let context = HoverPeekContextClass::SqlEditor;
    // No live database connection: a dialect fallback answers, so the card is a
    // wrong-provider fallback rather than the authoritative schema provider.
    let card = HoverPeekCard::new(HoverPeekCardInit {
        context,
        card_id: String::new(),
        mode_class: HoverPeekModeClass::HoverQuickInfo,
        source: source(
            context,
            AssistSourceFamily::FallbackLexical,
            None,
            "SQL dialect fallback",
            RouterSupportClass::FallbackOnly,
            RouterFreshnessClass::WarmCached,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::PartialForClaimedScope,
            RouterLocalityClass::LocalInProcess,
            RouterDegradedStateClass::DegradedProviderUnavailable,
            vec![ScopeLimitClass::SingleFileOnly],
            "No live database connection; hover answered by a dialect fallback, not live schema.",
        ),
        state_class: HoverPeekStateClass::WrongProviderFallback,
        mapping_quality: MappingQualityClass::Heuristic,
        raw_rendered_mode: RawRenderedModeClass::RawSourceOnly,
        presentation_class: HoverPeekPresentationClass::Transient,
        rendered_form_summary: None,
        raw_form_summary: Some("Dialect-derived description for the identifier.".to_owned()),
    });
    assemble_snapshot(SnapshotSpec {
        context,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::SourceLabeledFallback,
        degrade_label: "Source-labeled fallback — no live connection",
        card,
    })
}

fn build_docs_code_block_snapshot() -> HoverPeekSnapshot {
    let context = HoverPeekContextClass::DocsCodeBlock;
    // A pinned docs hover whose rendered markdown differs from its raw source, shown
    // while a refresh is pending. Pinned form must keep the same provenance labels.
    let card = HoverPeekCard::new(HoverPeekCardInit {
        context,
        card_id: String::new(),
        mode_class: HoverPeekModeClass::HoverPinned,
        source: source(
            context,
            AssistSourceFamily::FallbackLexical,
            None,
            "Detected-language best effort",
            RouterSupportClass::FallbackOnly,
            RouterFreshnessClass::Stale,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::PartialForClaimedScope,
            RouterLocalityClass::LocalInProcess,
            RouterDegradedStateClass::DegradedScopeNarrowed,
            vec![ScopeLimitClass::SingleFileOnly],
            "Pinned best-effort docs hover; rendered markdown differs from raw source.",
        ),
        state_class: HoverPeekStateClass::Stale,
        mapping_quality: MappingQualityClass::Approximate,
        raw_rendered_mode: RawRenderedModeClass::RawAndRenderedDistinct,
        presentation_class: HoverPeekPresentationClass::Pinned,
        rendered_form_summary: Some("Rendered markdown documentation.".to_owned()),
        raw_form_summary: Some("Raw markdown source of the doc comment.".to_owned()),
    });
    assemble_snapshot(SnapshotSpec {
        context,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::SourceLabeledFallback,
        degrade_label: "Source-labeled fallback — best effort by detected language",
        card,
    })
}

fn build_generated_file_snapshot() -> HoverPeekSnapshot {
    let context = HoverPeekContextClass::GeneratedFile;
    // A peek into generated output is an imported snapshot, not a live read.
    let card = HoverPeekCard::new(HoverPeekCardInit {
        context,
        card_id: String::new(),
        mode_class: HoverPeekModeClass::PeekDefinition,
        source: source(
            context,
            AssistSourceFamily::FrameworkPack,
            Some("generated-source-bridge"),
            "Generated-source bridge",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::WarmCached,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalSidecar,
            RouterDegradedStateClass::None,
            Vec::new(),
            "Peek shows an imported snapshot of generated output; edits route through the generator.",
        ),
        state_class: HoverPeekStateClass::ImportedSnapshot,
        mapping_quality: MappingQualityClass::Exact,
        raw_rendered_mode: RawRenderedModeClass::RawSourceOnly,
        presentation_class: HoverPeekPresentationClass::Transient,
        rendered_form_summary: None,
        raw_form_summary: Some("Imported snapshot of the generated definition.".to_owned()),
    });
    assemble_snapshot(SnapshotSpec {
        context,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::ReadOnlyNoApply,
        degrade_label: "Read-only — generated output, regenerate via the generator",
        card,
    })
}

fn build_protected_file_snapshot() -> HoverPeekSnapshot {
    let context = HoverPeekContextClass::ProtectedFile;
    let card = HoverPeekCard::new(HoverPeekCardInit {
        context,
        card_id: String::new(),
        mode_class: HoverPeekModeClass::HoverQuickInfo,
        source: source(
            context,
            AssistSourceFamily::FrameworkPack,
            Some("schema-pack:policy"),
            "Policy schema pack",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::AuthoritativeLive,
            RouterScopeClaimClass::WholeWorkspace,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalSidecar,
            RouterDegradedStateClass::DegradedPolicyNarrowed,
            Vec::new(),
            "Hover on a protected policy key; content narrowed by policy, writes require review.",
        ),
        state_class: HoverPeekStateClass::PolicyLimited,
        mapping_quality: MappingQualityClass::Exact,
        raw_rendered_mode: RawRenderedModeClass::RawSourceOnly,
        presentation_class: HoverPeekPresentationClass::Transient,
        rendered_form_summary: None,
        raw_form_summary: Some("Policy key description, narrowed by policy.".to_owned()),
    });
    assemble_snapshot(SnapshotSpec {
        context,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::ReadOnlyNoApply,
        degrade_label: "Read-only — writes require staged review",
        card,
    })
}

fn build_partial_index_snapshot() -> HoverPeekSnapshot {
    let context = HoverPeekContextClass::PartialIndexState;
    let card = HoverPeekCard::new(HoverPeekCardInit {
        context,
        card_id: String::new(),
        mode_class: HoverPeekModeClass::PeekReferences,
        source: source(
            context,
            AssistSourceFamily::LanguageServer,
            Some("rust-analyzer"),
            "rust-analyzer (indexing)",
            RouterSupportClass::Advisory,
            RouterFreshnessClass::Unverified,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::PartialForClaimedScope,
            RouterLocalityClass::LocalSidecar,
            RouterDegradedStateClass::DegradedScopeNarrowed,
            vec![ScopeLimitClass::SingleFileOnly],
            "Peek references while the index builds; results are partial and labeled.",
        ),
        state_class: HoverPeekStateClass::Partial,
        mapping_quality: MappingQualityClass::Approximate,
        raw_rendered_mode: RawRenderedModeClass::RawSourceOnly,
        presentation_class: HoverPeekPresentationClass::Transient,
        rendered_form_summary: None,
        raw_form_summary: Some("Partial reference list while indexing completes.".to_owned()),
    });
    assemble_snapshot(SnapshotSpec {
        context,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::PendingPartialIndex,
        degrade_label: "Pending — index still building",
        card,
    })
}

fn build_large_file_snapshot() -> HoverPeekSnapshot {
    let context = HoverPeekContextClass::LargeFileRestricted;
    let card = HoverPeekCard::new(HoverPeekCardInit {
        context,
        card_id: String::new(),
        mode_class: HoverPeekModeClass::HoverQuickInfo,
        source: source(
            context,
            AssistSourceFamily::FallbackLexical,
            None,
            "Large-file mode",
            RouterSupportClass::Unsupported,
            RouterFreshnessClass::Unverified,
            RouterScopeClaimClass::SingleFile,
            RouterCompletenessClass::UnavailableForClaimedScope,
            RouterLocalityClass::LocalInProcess,
            RouterDegradedStateClass::DegradedScopeNarrowed,
            vec![ScopeLimitClass::SingleFileOnly],
            "Hover and peek are suppressed in large-file / restricted mode.",
        ),
        state_class: HoverPeekStateClass::Suppressed,
        mapping_quality: MappingQualityClass::Unresolved,
        raw_rendered_mode: RawRenderedModeClass::RawSourceOnly,
        presentation_class: HoverPeekPresentationClass::Transient,
        rendered_form_summary: None,
        raw_form_summary: None,
    });
    assemble_snapshot(SnapshotSpec {
        context,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::SuppressedLargeFile,
        degrade_label: "Suppressed — large-file mode",
        card,
    })
}

fn build_diff_review_snapshot() -> HoverPeekSnapshot {
    let context = HoverPeekContextClass::DiffReviewSurface;
    // Promoted from a review thread into a durable tab: provenance and return anchor
    // must survive the promotion.
    let card = HoverPeekCard::new(HoverPeekCardInit {
        context,
        card_id: String::new(),
        mode_class: HoverPeekModeClass::HoverQuickInfo,
        source: source(
            context,
            AssistSourceFamily::LanguageServer,
            Some("rust-analyzer"),
            "rust-analyzer (review base)",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::AuthoritativeLive,
            RouterScopeClaimClass::WholeWorkspace,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalSidecar,
            RouterDegradedStateClass::None,
            Vec::new(),
            "Hover on a changed symbol in a review surface, promoted into a durable tab.",
        ),
        state_class: HoverPeekStateClass::Live,
        mapping_quality: MappingQualityClass::Exact,
        raw_rendered_mode: RawRenderedModeClass::RawSourceOnly,
        presentation_class: HoverPeekPresentationClass::PromotedTab,
        rendered_form_summary: None,
        raw_form_summary: Some("Symbol info for the reviewed change.".to_owned()),
    });
    assemble_snapshot(SnapshotSpec {
        context,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::FullFidelity,
        degrade_label: "Full fidelity",
        card,
    })
}

fn build_graph_explainer_snapshot() -> HoverPeekSnapshot {
    let context = HoverPeekContextClass::GraphLinkedExplainer;
    // A pinned graph-linked explainer peek: provenance must stay visible in the
    // pinned form.
    let card = HoverPeekCard::new(HoverPeekCardInit {
        context,
        card_id: String::new(),
        mode_class: HoverPeekModeClass::PeekCallHierarchy,
        source: source(
            context,
            AssistSourceFamily::ProjectGraph,
            Some("project-graph"),
            "Project graph",
            RouterSupportClass::Authoritative,
            RouterFreshnessClass::AuthoritativeLive,
            RouterScopeClaimClass::WholeWorkspace,
            RouterCompletenessClass::CompleteForClaimedScope,
            RouterLocalityClass::LocalSidecar,
            RouterDegradedStateClass::None,
            Vec::new(),
            "Pinned call-hierarchy explainer linked to a graph node; provenance stays visible.",
        ),
        state_class: HoverPeekStateClass::Live,
        mapping_quality: MappingQualityClass::Exact,
        raw_rendered_mode: RawRenderedModeClass::RawAndRenderedEquivalent,
        presentation_class: HoverPeekPresentationClass::Pinned,
        rendered_form_summary: Some("Rendered call-hierarchy explainer.".to_owned()),
        raw_form_summary: Some("Underlying graph edges for the node.".to_owned()),
    });
    assemble_snapshot(SnapshotSpec {
        context,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::FullFidelity,
        degrade_label: "Full fidelity",
        card,
    })
}

// ---------------------------------------------------------------------------
// Invariant evaluation.
// ---------------------------------------------------------------------------

fn evaluate_invariants(snapshots: &[HoverPeekSnapshot]) -> Vec<HoverPeekInvariant> {
    let cards: Vec<&HoverPeekCard> = snapshots.iter().map(|snapshot| &snapshot.card).collect();

    let mut invariants = Vec::new();

    invariants.push(HoverPeekInvariant {
        invariant_id: "every_context_resolves_a_card".into(),
        statement: "Each claimed inspection context resolves exactly one hover / peek card.".into(),
        holds: !snapshots.is_empty()
            && HoverPeekContextClass::ALL.iter().all(|context| {
                snapshots
                    .iter()
                    .filter(|s| s.context_class == *context)
                    .count()
                    == 1
            }),
    });

    invariants.push(HoverPeekInvariant {
        invariant_id: "every_card_keyboard_invocable".into(),
        statement: "Every card is keyboard-invocable, so pointer hover is never the only path to \
                    its content or provenance."
            .into(),
        holds: cards
            .iter()
            .all(|card| card.keyboard_invocable && !card.keyboard_command_id_ref.trim().is_empty()),
    });

    invariants.push(HoverPeekInvariant {
        invariant_id: "every_card_provenance_labeled".into(),
        statement: "Every card carries visible provider / source provenance.".into(),
        holds: cards.iter().all(|card| card.provenance_labeled()),
    });

    invariants.push(HoverPeekInvariant {
        invariant_id: "non_live_states_disclosed_inline".into(),
        statement: "Every non-live card discloses its state inline with a non-color cue, never \
                    styled like live authoritative docs."
            .into(),
        holds: cards.iter().all(|card| card.non_live_state_disclosed()),
    });

    invariants.push(HoverPeekInvariant {
        invariant_id: "wrong_provider_not_styled_live".into(),
        statement: "Every wrong-provider fallback card is non-live and discloses the fallback \
                    inline."
            .into(),
        holds: cards
            .iter()
            .filter(|card| matches!(card.state_class, HoverPeekStateClass::WrongProviderFallback))
            .all(|card| !card.is_live() && card.non_live_state_disclosed()),
    });

    invariants.push(HoverPeekInvariant {
        invariant_id: "mapping_quality_disclosed_when_inexact".into(),
        statement: "Every card with an inexact mapping discloses the mapping quality.".into(),
        holds: cards
            .iter()
            .all(|card| card.mapping_disclosed_when_inexact()),
    });

    invariants.push(HoverPeekInvariant {
        invariant_id: "raw_rendered_distinct_offers_escape".into(),
        statement: "Every card whose raw and rendered forms differ materially offers a visible \
                    open-raw escape."
            .into(),
        holds: cards.iter().all(|card| card.raw_escape_when_distinct()),
    });

    invariants.push(HoverPeekInvariant {
        invariant_id: "target_identity_locked_no_silent_retarget".into(),
        statement: "Every card pins a locked symbol / anchor identity and never silently \
                    retargets when a later provider answers."
            .into(),
        holds: cards.iter().all(|card| card.target_identity_locked()),
    });

    invariants.push(HoverPeekInvariant {
        invariant_id: "promotions_preserve_provenance_and_continuity".into(),
        statement: "Every promotion preserves the same provider / source labels and the return \
                    anchor."
            .into(),
        holds: cards
            .iter()
            .all(|card| card.promotions_preserve_provenance_and_continuity()),
    });

    invariants.push(HoverPeekInvariant {
        invariant_id: "content_cards_offer_all_promotion_paths".into(),
        statement: "Every content card offers keep-open / pin, open-in-tab, open-in-split, and \
                    dismiss-return."
            .into(),
        holds: cards.iter().all(|card| card.offers_all_promotion_paths()),
    });

    invariants.push(HoverPeekInvariant {
        invariant_id: "persisted_forms_preserve_labels".into(),
        statement: "Every pinned or promoted card preserves visible provenance and a freshness \
                    label, just like its transient form."
            .into(),
        holds: cards
            .iter()
            .filter(|card| card.presentation_class.is_persisted())
            .all(|card| card.provenance_labeled() && !card.source.source_label.trim().is_empty()),
    });

    invariants.push(HoverPeekInvariant {
        invariant_id: "every_card_screen_reader_meaningful".into(),
        statement: "Every card carries a non-empty screen-reader label.".into(),
        holds: cards
            .iter()
            .all(|card| !card.accessibility_label.trim().is_empty()),
    });

    invariants.push(HoverPeekInvariant {
        invariant_id: "diff_review_and_graph_contexts_present".into(),
        statement: "The diff / review and graph-linked explainer contexts each resolve a card, so \
                    hover / peek is consistent beyond plain file surfaces."
            .into(),
        holds: [
            HoverPeekContextClass::DiffReviewSurface,
            HoverPeekContextClass::GraphLinkedExplainer,
        ]
        .iter()
        .all(|context| snapshots.iter().any(|s| s.context_class == *context)),
    });

    invariants.push(HoverPeekInvariant {
        invariant_id: "suppressed_card_still_reachable_and_disclosed".into(),
        statement: "Every suppressed card stays keyboard-invocable, screen-reader labeled, and \
                    discloses its suppression."
            .into(),
        holds: cards
            .iter()
            .filter(|card| !card.offers_content())
            .all(|card| {
                card.keyboard_invocable
                    && card.inline_state_disclosed
                    && !card.accessibility_label.trim().is_empty()
            }),
    });

    invariants.push(HoverPeekInvariant {
        invariant_id: "degraded_contexts_label_and_disclose".into(),
        statement: "Every context that is not full fidelity carries a visible degrade label and \
                    flags disclosure."
            .into(),
        holds: snapshots
            .iter()
            .filter(|s| s.degrade_class != AssistDegradeClass::FullFidelity)
            .all(|s| !s.degrade_label.trim().is_empty() && s.disclosure_required),
    });

    invariants.push(HoverPeekInvariant {
        invariant_id: "shared_contexts_reuse_editor_surface_vocab".into(),
        statement: "Every shared file context maps to a distinct canonical editor surface, so the \
                    surface vocabulary is reused, not forked."
            .into(),
        holds: {
            let mut surfaces: Vec<EditorSurfaceClass> = snapshots
                .iter()
                .filter_map(|s| s.base_editor_surface)
                .collect();
            let total = surfaces.len();
            surfaces.sort_unstable();
            surfaces.dedup();
            surfaces.len() == total && total == EditorSurfaceClass::ALL.len()
        },
    });

    invariants
}

#[cfg(test)]
mod tests;
