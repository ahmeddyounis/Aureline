//! Canonical decoration, code-lens, and inlay-hint descriptor model with
//! precedence, provenance/confidence, density, and large-file suppression.
//!
//! Where the [editor-assist matrix](crate::m5_editor_assist) freezes the
//! *vocabulary* — the precedence ladder, the class catalogs, and the per-surface
//! degraded-state policy — this module freezes the *typed descriptor model* every
//! claimed editor surface renders inline metadata through. Before it, each pane
//! was free to invent its own decoration / lens / hint object with its own
//! precedence handling and its own ad hoc reasons for hiding a hint. This module
//! materializes one shared [`AssistDescriptor`] for all three families and one
//! deterministic resolver that turns a descriptor plus a [`RenderContext`] into a
//! [`ResolvedDescriptor`] with an explicit visibility verdict, an explicit
//! suppression reason, and its keyboard / screen-reader / non-color accessibility
//! truth.
//!
//! The model pins five things at once:
//!
//! 1. **One descriptor shape** — [`AssistDescriptor`] carries class, family,
//!    owning precedence layer, source/provider, freshness/confidence, target
//!    anchor/span, placement, actionability, accessibility, and the density /
//!    zoom / layout-shift policy flags that drive suppression. Decorations, code
//!    lenses, and inlay hints all use it; none fork a second object.
//! 2. **Precedence in the resolver** — every descriptor inherits the frozen rank
//!    of its [`EditorLayerClass`]. Editing truth (diagnostics, debug frame,
//!    conflict, review) always outranks advisory metadata, the resolver never
//!    suppresses editing truth for a convenience reason, and an overlapping
//!    convenience descriptor yields to the editing-truth descriptor it collides
//!    with ([`PrecedenceConflictCase`]).
//! 3. **Explicit suppression reasons** — every time a hint is hidden, reduced, or
//!    held the resolver records *why* via [`SuppressionReason`]: density
//!    compaction, high-zoom horizontal budget, typing budget, low confidence,
//!    large-file / restricted mode, partial-index pending, a source fallback, an
//!    unavailable surface, or an editing-truth overlap. The shell can downgrade a
//!    class automatically without losing why it disappeared.
//! 4. **Accessibility for every actionable / severity-bearing class** — every
//!    decoration that conveys severity or carries an action declares a keyboard
//!    path, a screen-reader label, and a non-color differentiator, so nothing is
//!    color-only or mouse-only.
//! 5. **Self-proof** — the model carries the [`ModelInvariant`]s it must satisfy
//!    and evaluates them over its own catalog, scenarios, and conflict cases, so
//!    a structural regression flips an invariant to `holds = false` rather than
//!    silently shipping.
//!
//! The build is static and deterministic: [`assist_descriptor_model`] assembles
//! the one canonical record from the frozen matrix, and the checked-in fixture
//! plus the replay gate freeze it byte-for-byte. It carries no file contents,
//! credential bodies, or raw provider payloads, so support, AI, and migration
//! surfaces can consume it directly.

use serde::{Deserialize, Serialize};

use crate::assist::AssistSourceLabelClass;
use crate::m5_editor_assist::{
    editor_assist_matrix, AssistChannelClass, AssistDegradeClass, ClassDescriptor, CodeLensClass,
    DecorationClass, EditorAssistMatrix, EditorLayerClass, EditorSurfaceClass, InlayHintClass,
    MicroSurfaceKind, TruthTier,
};

/// Schema version for the assist-descriptor model record.
pub const M5_ASSIST_DESCRIPTORS_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the assist-descriptor model record.
pub const M5_ASSIST_DESCRIPTORS_SCHEMA_REF: &str =
    "schemas/editor/m5-assist-descriptors.schema.json";

/// Stable record-kind tag for the assist-descriptor model record.
pub const M5_ASSIST_DESCRIPTORS_RECORD_KIND: &str = "m5_assist_descriptor_model";

/// Stable id for the canonical assist-descriptor model.
pub const M5_ASSIST_DESCRIPTORS_MODEL_ID: &str = "m5-assist-descriptors:model:0001";

/// Capture stamp for the canonical model. Held as a constant so the projection
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_ASSIST_DESCRIPTORS_AS_OF: &str = "2026-06-22T00:00:00Z";

// ---------------------------------------------------------------------------
// Class catalogs new to the descriptor model.
// ---------------------------------------------------------------------------

/// Which of the three micro-surface families a descriptor belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistDescriptorFamily {
    /// A gutter or inline decoration (editing truth).
    Decoration,
    /// An above-line code lens (convenience metadata).
    CodeLens,
    /// An inline inlay hint (convenience metadata).
    InlayHint,
}

impl AssistDescriptorFamily {
    /// All families, in catalog order.
    pub const ALL: [Self; 3] = [Self::Decoration, Self::CodeLens, Self::InlayHint];

    /// Returns the stable schema token for this family.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Decoration => "decoration",
            Self::CodeLens => "code_lens",
            Self::InlayHint => "inlay_hint",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Decoration => "Decoration",
            Self::CodeLens => "Code lens",
            Self::InlayHint => "Inlay hint",
        }
    }
}

/// Where a descriptor is drawn relative to the text it annotates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlacementClass {
    /// A glyph drawn in the gutter.
    GutterGlyph,
    /// An inline underline / squiggle under the span.
    InlineUnderline,
    /// An inline range box / highlight over the span.
    InlineRange,
    /// A full-line background highlight.
    LineHighlight,
    /// A row drawn above the annotated line.
    AboveLine,
    /// Inline text inserted before the token.
    InlineBefore,
    /// Inline text inserted after the token.
    InlineAfter,
}

impl PlacementClass {
    /// All placements, in catalog order.
    pub const ALL: [Self; 7] = [
        Self::GutterGlyph,
        Self::InlineUnderline,
        Self::InlineRange,
        Self::LineHighlight,
        Self::AboveLine,
        Self::InlineBefore,
        Self::InlineAfter,
    ];

    /// Returns the stable schema token for this placement.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GutterGlyph => "gutter_glyph",
            Self::InlineUnderline => "inline_underline",
            Self::InlineRange => "inline_range",
            Self::LineHighlight => "line_highlight",
            Self::AboveLine => "above_line",
            Self::InlineBefore => "inline_before",
            Self::InlineAfter => "inline_after",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::GutterGlyph => "Gutter glyph",
            Self::InlineUnderline => "Inline underline",
            Self::InlineRange => "Inline range",
            Self::LineHighlight => "Line highlight",
            Self::AboveLine => "Above line",
            Self::InlineBefore => "Inline before",
            Self::InlineAfter => "Inline after",
        }
    }
}

/// Whether a descriptor conveys severity, carries an action, or is informational.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionabilityClass {
    /// Purely visual; conveys neither severity nor an action.
    Informational,
    /// Conveys severity (error, warning, conflict) but is not itself activatable.
    SeverityBearing,
    /// Carries an action the user can invoke.
    Activatable,
    /// Conveys severity and carries an action.
    SeverityBearingActivatable,
}

impl ActionabilityClass {
    /// All actionability classes, in catalog order.
    pub const ALL: [Self; 4] = [
        Self::Informational,
        Self::SeverityBearing,
        Self::Activatable,
        Self::SeverityBearingActivatable,
    ];

    /// Returns the stable schema token for this class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Informational => "informational",
            Self::SeverityBearing => "severity_bearing",
            Self::Activatable => "activatable",
            Self::SeverityBearingActivatable => "severity_bearing_activatable",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Informational => "Informational",
            Self::SeverityBearing => "Severity-bearing",
            Self::Activatable => "Activatable",
            Self::SeverityBearingActivatable => "Severity-bearing & activatable",
        }
    }

    /// Whether this class conveys severity.
    pub const fn is_severity_bearing(self) -> bool {
        matches!(
            self,
            Self::SeverityBearing | Self::SeverityBearingActivatable
        )
    }

    /// Whether this class carries an action.
    pub const fn is_activatable(self) -> bool {
        matches!(self, Self::Activatable | Self::SeverityBearingActivatable)
    }

    /// Whether a descriptor of this class must declare a keyboard path. Every
    /// actionable or severity-bearing class must be reachable without a mouse.
    pub const fn requires_keyboard_path(self) -> bool {
        !matches!(self, Self::Informational)
    }
}

/// Confidence the source places in a descriptor. Drives automatic suppression of
/// low-certainty convenience metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistConfidenceClass {
    /// Deterministic / high-confidence result.
    High,
    /// Probable result; rendered but flagged for the model.
    Probable,
    /// Low-confidence or speculative result; suppressed by default on
    /// convenience channels.
    LowSpeculative,
}

impl AssistConfidenceClass {
    /// All confidence classes, in catalog order.
    pub const ALL: [Self; 3] = [Self::High, Self::Probable, Self::LowSpeculative];

    /// Returns the stable schema token for this class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Probable => "probable",
            Self::LowSpeculative => "low_speculative",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::High => "High",
            Self::Probable => "Probable",
            Self::LowSpeculative => "Low / speculative",
        }
    }

    /// Whether this confidence is low enough to suppress convenience metadata.
    pub const fn is_low_certainty(self) -> bool {
        matches!(self, Self::LowSpeculative)
    }
}

/// Freshness of a descriptor relative to its source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistFreshnessClass {
    /// Computed live from the current buffer / project state.
    Live,
    /// Served from a cache and labeled as such.
    Cached,
    /// Known stale, pending a refresh.
    Stale,
}

impl AssistFreshnessClass {
    /// All freshness classes, in catalog order.
    pub const ALL: [Self; 3] = [Self::Live, Self::Cached, Self::Stale];

    /// Returns the stable schema token for this class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Cached => "cached",
            Self::Stale => "stale",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Live => "Live",
            Self::Cached => "Cached",
            Self::Stale => "Stale",
        }
    }
}

/// Whether a descriptor animates and whether reduced motion turns the animation
/// off.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MotionClass {
    /// No animation; reduced motion is a no-op.
    Static,
    /// Has an animation that reduced motion replaces with a static cue.
    AnimatedReducible,
}

impl MotionClass {
    /// All motion classes, in catalog order.
    pub const ALL: [Self; 2] = [Self::Static, Self::AnimatedReducible];

    /// Returns the stable schema token for this class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Static => "static",
            Self::AnimatedReducible => "animated_reducible",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Static => "Static",
            Self::AnimatedReducible => "Animated (reducible)",
        }
    }
}

/// Editor density tier. Drives compaction of optional convenience metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DensityTier {
    /// Comfortable spacing; full metadata.
    Comfortable,
    /// Compact spacing; full metadata, tighter layout.
    Compact,
    /// Dense spacing; optional metadata is compacted away.
    Dense,
}

impl DensityTier {
    /// All density tiers, in catalog order.
    pub const ALL: [Self; 3] = [Self::Comfortable, Self::Compact, Self::Dense];

    /// Returns the stable schema token for this tier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Comfortable => "comfortable",
            Self::Compact => "compact",
            Self::Dense => "dense",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Comfortable => "Comfortable",
            Self::Compact => "Compact",
            Self::Dense => "Dense",
        }
    }
}

/// Editor zoom tier. High zoom narrows the column budget for inline metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ZoomTier {
    /// Standard zoom.
    Standard,
    /// High zoom; horizontal space is scarce.
    High,
}

impl ZoomTier {
    /// All zoom tiers, in catalog order.
    pub const ALL: [Self; 2] = [Self::Standard, Self::High];

    /// Returns the stable schema token for this tier.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::High => "high",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "Standard",
            Self::High => "High",
        }
    }
}

/// The visibility verdict a resolver assigns to a descriptor in a context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisibilityVerdict {
    /// Drawn at full fidelity.
    Rendered,
    /// Drawn, but reduced or labeled (fallback source, pending index, reduced
    /// decoration).
    Downgraded,
    /// Temporarily held to protect the typing budget; returns once typing
    /// settles or the editing-truth overlap clears.
    Deferred,
    /// Not drawn at all.
    Suppressed,
}

impl VisibilityVerdict {
    /// All verdicts, in catalog order.
    pub const ALL: [Self; 4] = [
        Self::Rendered,
        Self::Downgraded,
        Self::Deferred,
        Self::Suppressed,
    ];

    /// Returns the stable schema token for this verdict.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rendered => "rendered",
            Self::Downgraded => "downgraded",
            Self::Deferred => "deferred",
            Self::Suppressed => "suppressed",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Rendered => "Rendered",
            Self::Downgraded => "Downgraded",
            Self::Deferred => "Deferred",
            Self::Suppressed => "Suppressed",
        }
    }

    /// Whether the descriptor is currently offered to the user (and therefore
    /// keyboard-reachable). Deferred and suppressed descriptors are not.
    pub const fn is_offered(self) -> bool {
        matches!(self, Self::Rendered | Self::Downgraded)
    }
}

/// Why a descriptor was reduced, held, or hidden. Every non-rendered verdict
/// carries one of these so the disappearance is always explainable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuppressionReason {
    /// Drawn at full fidelity; nothing was suppressed.
    NotSuppressed,
    /// Drawn from a labeled fallback source on this surface.
    SourceFallback,
    /// Labeled pending until the semantic index finishes building.
    PartialIndexPending,
    /// An editing-truth decoration reduced (not suppressed) in large-file mode.
    ReducedDecoration,
    /// Optional convenience metadata compacted away at dense spacing.
    DensityCompaction,
    /// Inline metadata dropped to protect the horizontal column budget at high
    /// zoom.
    HighZoomHorizontalBudget,
    /// Layout-shifting metadata held while the user is typing.
    TypingBudget,
    /// Low-confidence convenience metadata suppressed by default.
    LowConfidence,
    /// Suppressed because the file is in large-file / restricted mode.
    LargeFileRestricted,
    /// Not offered on this surface at all.
    UnavailableOnSurface,
    /// Yielded to an overlapping editing-truth descriptor.
    OutrankedByEditingTruth,
}

impl SuppressionReason {
    /// All reasons, in catalog order.
    pub const ALL: [Self; 11] = [
        Self::NotSuppressed,
        Self::SourceFallback,
        Self::PartialIndexPending,
        Self::ReducedDecoration,
        Self::DensityCompaction,
        Self::HighZoomHorizontalBudget,
        Self::TypingBudget,
        Self::LowConfidence,
        Self::LargeFileRestricted,
        Self::UnavailableOnSurface,
        Self::OutrankedByEditingTruth,
    ];

    /// Returns the stable schema token for this reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotSuppressed => "not_suppressed",
            Self::SourceFallback => "source_fallback",
            Self::PartialIndexPending => "partial_index_pending",
            Self::ReducedDecoration => "reduced_decoration",
            Self::DensityCompaction => "density_compaction",
            Self::HighZoomHorizontalBudget => "high_zoom_horizontal_budget",
            Self::TypingBudget => "typing_budget",
            Self::LowConfidence => "low_confidence",
            Self::LargeFileRestricted => "large_file_restricted",
            Self::UnavailableOnSurface => "unavailable_on_surface",
            Self::OutrankedByEditingTruth => "outranked_by_editing_truth",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotSuppressed => "Not suppressed",
            Self::SourceFallback => "Source fallback",
            Self::PartialIndexPending => "Partial-index pending",
            Self::ReducedDecoration => "Reduced decoration",
            Self::DensityCompaction => "Density compaction",
            Self::HighZoomHorizontalBudget => "High-zoom horizontal budget",
            Self::TypingBudget => "Typing budget",
            Self::LowConfidence => "Low confidence",
            Self::LargeFileRestricted => "Large-file / restricted",
            Self::UnavailableOnSurface => "Unavailable on surface",
            Self::OutrankedByEditingTruth => "Outranked by editing truth",
        }
    }
}

// ---------------------------------------------------------------------------
// Descriptor model.
// ---------------------------------------------------------------------------

/// The target span a descriptor annotates. Line and column are zero-based; the
/// span is half-open `[start, end)` on a single logical anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextAnchor {
    /// Zero-based start line.
    pub start_line: u32,
    /// Zero-based start column.
    pub start_column: u32,
    /// Zero-based end line.
    pub end_line: u32,
    /// Zero-based end column.
    pub end_column: u32,
}

impl TextAnchor {
    /// Builds an anchor.
    pub const fn new(start_line: u32, start_column: u32, end_line: u32, end_column: u32) -> Self {
        Self {
            start_line,
            start_column,
            end_line,
            end_column,
        }
    }

    /// Whether this anchor overlaps another on a shared line. Used to resolve
    /// editing-truth-versus-convenience collisions.
    pub fn overlaps(&self, other: &Self) -> bool {
        if self.start_line != other.start_line {
            return false;
        }
        self.start_column < other.end_column && other.start_column < self.end_column
    }
}

/// Provenance for a descriptor: the source label, provider, freshness, and
/// confidence consumers must keep visible and use to decide suppression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescriptorSource {
    /// Stable source-label class shared with the assist source-label model.
    pub source_label_class: AssistSourceLabelClass,
    /// Plain-language source label consumers must keep visible.
    pub source_label: String,
    /// Provider id, when the source came through the language router.
    pub provider_id: Option<String>,
    /// Freshness relative to the source.
    pub freshness: AssistFreshnessClass,
    /// Confidence the source places in this descriptor.
    pub confidence: AssistConfidenceClass,
    /// Whether the descriptor must carry an explicit AI source label.
    pub requires_ai_label: bool,
    /// Whether consumers must keep this source visually distinct from
    /// deterministic language intelligence.
    pub requires_visual_distinction: bool,
}

/// Accessibility truth for a descriptor: keyboard path, screen-reader label,
/// non-color differentiator, and motion posture. Never color-only, never
/// mouse-only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityProfile {
    /// Screen-reader label announced for this descriptor.
    pub screen_reader_label: String,
    /// Non-color cue (glyph, underline style, text label) that differentiates
    /// the descriptor without relying on color.
    pub non_color_differentiator: String,
    /// Keyboard command that reaches or invokes the descriptor, when one exists.
    pub keyboard_path: Option<String>,
    /// Whether the descriptor animates and whether reduced motion reduces it.
    pub motion_class: MotionClass,
}

/// One canonical descriptor for a decoration, code lens, or inlay hint. This is
/// the single shared shape every claimed editor surface renders inline metadata
/// through.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistDescriptor {
    /// Stable descriptor id.
    pub descriptor_id: String,
    /// Micro-surface family.
    pub family: AssistDescriptorFamily,
    /// Stable class token within the family's catalog.
    pub class_token: String,
    /// Human-readable class label.
    pub class_label: String,
    /// Precedence layer that owns this descriptor.
    pub owning_layer: EditorLayerClass,
    /// Truth tier of the owning layer.
    pub truth_tier: TruthTier,
    /// Assist channel this descriptor is drawn on (for matrix degrade lookup).
    pub channel: AssistChannelClass,
    /// Target anchor / span.
    pub anchor: TextAnchor,
    /// Where the descriptor is drawn relative to the text.
    pub placement: PlacementClass,
    /// Whether the descriptor conveys severity, carries an action, or is
    /// informational.
    pub actionability: ActionabilityClass,
    /// Provenance: source label, provider, freshness, confidence.
    pub source: DescriptorSource,
    /// Accessibility truth: keyboard path, screen-reader label, non-color cue.
    pub accessibility: AccessibilityProfile,
    /// Whether painting the descriptor shifts surrounding layout (and therefore
    /// must yield to the typing budget).
    pub layout_shifting: bool,
    /// Whether the descriptor is optional at dense spacing.
    pub density_optional: bool,
    /// Whether the descriptor is optional under a narrow high-zoom column budget.
    pub zoom_optional: bool,
    /// Command invoked when the descriptor is activated, when activatable.
    pub command_ref: Option<String>,
    /// Export-safe note describing the descriptor.
    pub note: String,
}

impl AssistDescriptor {
    /// Precedence rank of the owning layer; lower wins overlap.
    pub fn rank(&self) -> u8 {
        self.owning_layer.rank()
    }
}

// ---------------------------------------------------------------------------
// Render context + resolution.
// ---------------------------------------------------------------------------

/// The runtime context a descriptor is resolved against. The `surface` drives
/// the per-channel degraded-state lookup from the frozen matrix; the remaining
/// fields drive density / zoom / reduced-motion / typing suppression.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderContext {
    /// Stable scenario id.
    pub scenario_id: String,
    /// Human-readable scenario label.
    pub label: String,
    /// Editor surface (also encodes large-file, restricted, generated,
    /// protected, and partial-index states).
    pub surface: EditorSurfaceClass,
    /// Editor density tier.
    pub density: DensityTier,
    /// Editor zoom tier.
    pub zoom: ZoomTier,
    /// Whether reduced motion is active.
    pub reduced_motion: bool,
    /// Whether the user is actively typing (layout-shifting metadata is held).
    pub typing_active: bool,
}

/// The resolved verdict for one descriptor in one context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedDescriptor {
    /// Descriptor id.
    pub descriptor_id: String,
    /// Micro-surface family.
    pub family: AssistDescriptorFamily,
    /// Class token.
    pub class_token: String,
    /// Owning precedence layer.
    pub owning_layer: EditorLayerClass,
    /// Truth tier of the owning layer.
    pub truth_tier: TruthTier,
    /// Precedence rank.
    pub rank: u8,
    /// Visibility verdict.
    pub visibility: VisibilityVerdict,
    /// Effective degraded-state class after resolution.
    pub effective_degrade: AssistDegradeClass,
    /// Why the descriptor was reduced, held, or hidden.
    pub suppression_reason: SuppressionReason,
    /// Human-readable explanation of the reason.
    pub reason_detail: String,
    /// Whether animations are enabled for this descriptor in this context.
    pub animations_enabled: bool,
    /// Whether the descriptor is keyboard-reachable in this context.
    pub keyboard_reachable: bool,
    /// Source-label class (kept on the resolution for support export).
    pub source_label_class: AssistSourceLabelClass,
}

/// One resolution scenario: a render context and the resolved verdict for every
/// catalog descriptor under it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolutionScenario {
    /// Render context for this scenario.
    pub context: RenderContext,
    /// Resolved verdicts, in precedence then descriptor-id order.
    pub resolved: Vec<ResolvedDescriptor>,
    /// Count of rendered descriptors.
    pub rendered_count: usize,
    /// Count of downgraded descriptors.
    pub downgraded_count: usize,
    /// Count of deferred descriptors.
    pub deferred_count: usize,
    /// Count of suppressed descriptors.
    pub suppressed_count: usize,
    /// Export-safe note describing the scenario outcome.
    pub note: String,
}

impl ResolutionScenario {
    /// Returns the resolved verdict for a descriptor id, when present.
    pub fn resolved(&self, descriptor_id: &str) -> Option<&ResolvedDescriptor> {
        self.resolved
            .iter()
            .find(|resolved| resolved.descriptor_id == descriptor_id)
    }
}

/// One interaction-precedence conflict: an editing-truth descriptor and a
/// convenience descriptor that share an anchor. The convenience descriptor
/// yields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrecedenceConflictCase {
    /// Stable case id.
    pub case_id: String,
    /// Human-readable case label.
    pub label: String,
    /// Editing-truth descriptor id.
    pub editing_truth_descriptor_id: String,
    /// Editing-truth owning layer.
    pub editing_truth_layer: EditorLayerClass,
    /// Convenience descriptor id.
    pub convenience_descriptor_id: String,
    /// Convenience owning layer.
    pub convenience_layer: EditorLayerClass,
    /// The shared anchor the two descriptors collide on.
    pub shared_anchor: TextAnchor,
    /// Descriptor id that wins the overlap (always the editing-truth one).
    pub winner_descriptor_id: String,
    /// Descriptor id that yields (always the convenience one).
    pub yielded_descriptor_id: String,
    /// Visibility the yielded descriptor resolves to.
    pub yielded_visibility: VisibilityVerdict,
    /// Reason the yielded descriptor carries.
    pub yielded_reason: SuppressionReason,
    /// Export-safe note describing the resolution.
    pub note: String,
}

/// One frozen invariant the model must satisfy, with the result of evaluating it
/// over the model's own data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelInvariant {
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

/// The canonical, frozen, export-safe assist-descriptor model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistDescriptorModel {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_assist_descriptors_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable model id.
    pub model_id: String,
    /// Capture stamp.
    pub as_of: String,
    /// Descriptor family catalog.
    pub descriptor_families: Vec<ClassDescriptor>,
    /// Placement catalog.
    pub placement_classes: Vec<ClassDescriptor>,
    /// Actionability catalog.
    pub actionability_classes: Vec<ClassDescriptor>,
    /// Confidence catalog.
    pub confidence_classes: Vec<ClassDescriptor>,
    /// Freshness catalog.
    pub freshness_classes: Vec<ClassDescriptor>,
    /// Motion catalog.
    pub motion_classes: Vec<ClassDescriptor>,
    /// Density-tier catalog.
    pub density_tiers: Vec<ClassDescriptor>,
    /// Zoom-tier catalog.
    pub zoom_tiers: Vec<ClassDescriptor>,
    /// Visibility-verdict catalog.
    pub visibility_verdicts: Vec<ClassDescriptor>,
    /// Suppression-reason catalog.
    pub suppression_reasons: Vec<ClassDescriptor>,
    /// One canonical descriptor per decoration / code-lens / inlay-hint class.
    pub descriptor_catalog: Vec<AssistDescriptor>,
    /// Resolution scenarios across surfaces, densities, zoom, motion, and typing.
    pub scenarios: Vec<ResolutionScenario>,
    /// Interaction-precedence conflict cases.
    pub precedence_conflicts: Vec<PrecedenceConflictCase>,
    /// Frozen invariants and whether each holds on this model.
    pub invariants: Vec<ModelInvariant>,
    /// Whether the model is metadata-safe for support export.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl AssistDescriptorModel {
    /// Returns true when every frozen invariant holds on this model.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|invariant| invariant.holds)
    }

    /// Returns true when the model is metadata-safe for support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == M5_ASSIST_DESCRIPTORS_SCHEMA_REF
            && self.record_kind == M5_ASSIST_DESCRIPTORS_RECORD_KIND
    }

    /// Returns the catalog descriptor with the given id, when present.
    pub fn descriptor(&self, descriptor_id: &str) -> Option<&AssistDescriptor> {
        self.descriptor_catalog
            .iter()
            .find(|descriptor| descriptor.descriptor_id == descriptor_id)
    }

    /// Returns the scenario with the given id, when present.
    pub fn scenario(&self, scenario_id: &str) -> Option<&ResolutionScenario> {
        self.scenarios
            .iter()
            .find(|scenario| scenario.context.scenario_id == scenario_id)
    }
}

// ---------------------------------------------------------------------------
// Scenario ids.
// ---------------------------------------------------------------------------

const SCENARIO_CODE_FILE_COMFORTABLE: &str = "code_file_comfortable";
const SCENARIO_CODE_FILE_COMPACT: &str = "code_file_compact";
const SCENARIO_CODE_FILE_DENSE: &str = "code_file_dense";
const SCENARIO_CODE_FILE_HIGH_ZOOM: &str = "code_file_high_zoom";
const SCENARIO_CODE_FILE_TYPING: &str = "code_file_typing";
const SCENARIO_CODE_FILE_REDUCED_MOTION: &str = "code_file_reduced_motion";
const SCENARIO_SQL_EDITOR: &str = "sql_editor_comfortable";
const SCENARIO_DOCS_CODE_BLOCK: &str = "docs_code_block_comfortable";
const SCENARIO_PARTIAL_INDEX: &str = "partial_index";
const SCENARIO_GENERATED_FILE: &str = "generated_file";
const SCENARIO_LARGE_FILE: &str = "large_file_restricted";

// ---------------------------------------------------------------------------
// Builder.
// ---------------------------------------------------------------------------

/// Builds the one canonical assist-descriptor model.
///
/// The build is deterministic and self-contained: it consumes the frozen
/// [editor-assist matrix](crate::m5_editor_assist) for per-surface degraded-state
/// policy, materializes one descriptor per decoration / code-lens / inlay-hint
/// class, resolves every descriptor across the canonical scenarios, resolves the
/// interaction-precedence conflicts, and evaluates every frozen invariant over
/// the assembled data so the record's `invariants[].holds` reflect real checks.
pub fn assist_descriptor_model() -> AssistDescriptorModel {
    let matrix = editor_assist_matrix();

    let descriptor_catalog = build_descriptor_catalog();
    let scenarios = build_scenarios(&matrix, &descriptor_catalog);
    let precedence_conflicts = build_precedence_conflicts(&descriptor_catalog);

    let invariants = evaluate_invariants(&descriptor_catalog, &scenarios, &precedence_conflicts);

    let qualified = invariants.iter().all(|invariant| invariant.holds);
    let summary = if qualified {
        format!(
            "Assist-descriptor model frozen: {descriptors} descriptors \
             ({decorations} decorations, {lenses} code lenses, {hints} inlay hints) \
             resolved across {scenarios} scenarios and {conflicts} precedence conflicts, \
             all {invariants} invariants hold.",
            descriptors = descriptor_catalog.len(),
            decorations = DecorationClass::ALL.len(),
            lenses = CodeLensClass::ALL.len(),
            hints = InlayHintClass::ALL.len(),
            scenarios = scenarios.len(),
            conflicts = precedence_conflicts.len(),
            invariants = invariants.len(),
        )
    } else {
        let failed: Vec<&str> = invariants
            .iter()
            .filter(|invariant| !invariant.holds)
            .map(|invariant| invariant.invariant_id.as_str())
            .collect();
        format!(
            "Assist-descriptor model is inconsistent: failing invariants {}.",
            failed.join(", ")
        )
    };

    AssistDescriptorModel {
        record_kind: M5_ASSIST_DESCRIPTORS_RECORD_KIND.to_owned(),
        m5_assist_descriptors_schema_version: M5_ASSIST_DESCRIPTORS_SCHEMA_VERSION,
        schema_ref: M5_ASSIST_DESCRIPTORS_SCHEMA_REF.to_owned(),
        model_id: M5_ASSIST_DESCRIPTORS_MODEL_ID.to_owned(),
        as_of: M5_ASSIST_DESCRIPTORS_AS_OF.to_owned(),
        descriptor_families: catalog_from(AssistDescriptorFamily::ALL.iter().map(|family| {
            (
                family.as_str(),
                family.label(),
                "Micro-surface family rendered through the shared descriptor model.",
            )
        })),
        placement_classes: catalog_from(PlacementClass::ALL.iter().map(|placement| {
            (
                placement.as_str(),
                placement.label(),
                "Where the descriptor draws relative to the text it annotates.",
            )
        })),
        actionability_classes: catalog_from(ActionabilityClass::ALL.iter().map(|class| {
            (
                class.as_str(),
                class.label(),
                if class.requires_keyboard_path() {
                    "Actionable or severity-bearing; must declare a keyboard path and non-color cue."
                } else {
                    "Informational; carries neither severity nor an action."
                },
            )
        })),
        confidence_classes: catalog_from(AssistConfidenceClass::ALL.iter().map(|class| {
            (
                class.as_str(),
                class.label(),
                if class.is_low_certainty() {
                    "Low certainty; convenience metadata is suppressed by default."
                } else {
                    "Confidence high enough to render convenience metadata."
                },
            )
        })),
        freshness_classes: catalog_from(AssistFreshnessClass::ALL.iter().map(|class| {
            (
                class.as_str(),
                class.label(),
                "Freshness of the descriptor relative to its source.",
            )
        })),
        motion_classes: catalog_from(MotionClass::ALL.iter().map(|class| {
            (
                class.as_str(),
                class.label(),
                "Whether the descriptor animates and whether reduced motion reduces it.",
            )
        })),
        density_tiers: catalog_from(DensityTier::ALL.iter().map(|tier| {
            (
                tier.as_str(),
                tier.label(),
                "Editor density tier driving compaction of optional metadata.",
            )
        })),
        zoom_tiers: catalog_from(ZoomTier::ALL.iter().map(|tier| {
            (
                tier.as_str(),
                tier.label(),
                "Editor zoom tier driving the horizontal column budget.",
            )
        })),
        visibility_verdicts: catalog_from(VisibilityVerdict::ALL.iter().map(|verdict| {
            (
                verdict.as_str(),
                verdict.label(),
                "Visibility verdict the resolver assigns in a context.",
            )
        })),
        suppression_reasons: catalog_from(SuppressionReason::ALL.iter().map(|reason| {
            (reason.as_str(), reason.label(), suppression_reason_note(*reason))
        })),
        descriptor_catalog,
        scenarios,
        precedence_conflicts,
        invariants,
        raw_payload_excluded: true,
        summary,
    }
}

fn catalog_from<'a>(
    entries: impl Iterator<Item = (&'a str, &'a str, &'a str)>,
) -> Vec<ClassDescriptor> {
    entries
        .map(|(token, label, note)| ClassDescriptor {
            class_token: token.to_owned(),
            label: label.to_owned(),
            note: note.to_owned(),
        })
        .collect()
}

fn suppression_reason_note(reason: SuppressionReason) -> &'static str {
    match reason {
        SuppressionReason::NotSuppressed => "Drawn at full fidelity.",
        SuppressionReason::SourceFallback => {
            "Drawn from a labeled fallback source on this surface."
        }
        SuppressionReason::PartialIndexPending => {
            "Labeled pending until the semantic index builds."
        }
        SuppressionReason::ReducedDecoration => {
            "Editing-truth decoration reduced in large-file mode."
        }
        SuppressionReason::DensityCompaction => {
            "Optional metadata compacted away at dense spacing."
        }
        SuppressionReason::HighZoomHorizontalBudget => {
            "Inline metadata dropped under a narrow high-zoom column budget."
        }
        SuppressionReason::TypingBudget => "Layout-shifting metadata held while the user types.",
        SuppressionReason::LowConfidence => {
            "Low-confidence convenience metadata suppressed by default."
        }
        SuppressionReason::LargeFileRestricted => "Suppressed in large-file / restricted mode.",
        SuppressionReason::UnavailableOnSurface => "Not offered on this surface.",
        SuppressionReason::OutrankedByEditingTruth => {
            "Yielded to an overlapping editing-truth descriptor."
        }
    }
}

// ---------------------------------------------------------------------------
// Descriptor catalog construction.
// ---------------------------------------------------------------------------

fn build_descriptor_catalog() -> Vec<AssistDescriptor> {
    let mut catalog = Vec::new();
    for (index, class) in DecorationClass::ALL.iter().enumerate() {
        catalog.push(decoration_descriptor(*class, index));
    }
    for (index, class) in CodeLensClass::ALL.iter().enumerate() {
        catalog.push(code_lens_descriptor(*class, index));
    }
    for (index, class) in InlayHintClass::ALL.iter().enumerate() {
        catalog.push(inlay_hint_descriptor(*class, index));
    }
    catalog
}

/// Specification for one decoration class. Kept as a flat tuple so the catalog
/// reads as a table.
struct DecorationSpec {
    placement: PlacementClass,
    actionability: ActionabilityClass,
    source_label_class: AssistSourceLabelClass,
    source_label: &'static str,
    freshness: AssistFreshnessClass,
    confidence: AssistConfidenceClass,
    motion: MotionClass,
    screen_reader_label: &'static str,
    non_color: &'static str,
    keyboard_path: &'static str,
    command_ref: Option<&'static str>,
}

fn decoration_spec(class: DecorationClass) -> DecorationSpec {
    use AssistSourceLabelClass as Src;
    use DecorationClass as D;
    match class {
        D::DiagnosticUnderline => DecorationSpec {
            placement: PlacementClass::InlineUnderline,
            actionability: ActionabilityClass::SeverityBearingActivatable,
            source_label_class: Src::DeterministicLanguage,
            source_label: "Diagnostics",
            freshness: AssistFreshnessClass::Live,
            confidence: AssistConfidenceClass::High,
            motion: MotionClass::Static,
            screen_reader_label: "Diagnostic, severity error, underlined span",
            non_color: "Wavy underline",
            keyboard_path: "editor.action.marker.next",
            command_ref: Some("editor.action.showHover"),
        },
        D::DiagnosticGutterIcon => DecorationSpec {
            placement: PlacementClass::GutterGlyph,
            actionability: ActionabilityClass::SeverityBearingActivatable,
            source_label_class: Src::DeterministicLanguage,
            source_label: "Diagnostics",
            freshness: AssistFreshnessClass::Live,
            confidence: AssistConfidenceClass::High,
            motion: MotionClass::Static,
            screen_reader_label: "Diagnostic gutter marker, severity-bearing",
            non_color: "Severity glyph (error/warning shape)",
            keyboard_path: "editor.action.marker.next",
            command_ref: Some("editor.action.quickFix"),
        },
        D::DebugCurrentLine => DecorationSpec {
            placement: PlacementClass::LineHighlight,
            actionability: ActionabilityClass::SeverityBearingActivatable,
            source_label_class: Src::DeterministicLanguage,
            source_label: "Debug adapter",
            freshness: AssistFreshnessClass::Live,
            confidence: AssistConfidenceClass::High,
            motion: MotionClass::AnimatedReducible,
            screen_reader_label: "Current debug execution line",
            non_color: "Execution arrow glyph in gutter",
            keyboard_path: "workbench.action.debug.focusCurrentFrame",
            command_ref: None,
        },
        D::BreakpointGutter => DecorationSpec {
            placement: PlacementClass::GutterGlyph,
            actionability: ActionabilityClass::Activatable,
            source_label_class: Src::DeterministicLanguage,
            source_label: "Debug adapter",
            freshness: AssistFreshnessClass::Live,
            confidence: AssistConfidenceClass::High,
            motion: MotionClass::Static,
            screen_reader_label: "Breakpoint set on this line",
            non_color: "Filled circle glyph",
            keyboard_path: "editor.debug.action.toggleBreakpoint",
            command_ref: Some("editor.debug.action.toggleBreakpoint"),
        },
        D::MergeConflictRegion => DecorationSpec {
            placement: PlacementClass::LineHighlight,
            actionability: ActionabilityClass::SeverityBearingActivatable,
            source_label_class: Src::DeterministicLanguage,
            source_label: "Merge conflict",
            freshness: AssistFreshnessClass::Live,
            confidence: AssistConfidenceClass::High,
            motion: MotionClass::Static,
            screen_reader_label: "Merge conflict region, action required",
            non_color: "Conflict band with marker text (<<<< ==== >>>>)",
            keyboard_path: "merge-conflict.next",
            command_ref: Some("merge-conflict.accept.current"),
        },
        D::ReviewChangeGutter => DecorationSpec {
            placement: PlacementClass::GutterGlyph,
            actionability: ActionabilityClass::Activatable,
            source_label_class: Src::ProjectGraph,
            source_label: "Review changes",
            freshness: AssistFreshnessClass::Live,
            confidence: AssistConfidenceClass::High,
            motion: MotionClass::Static,
            screen_reader_label: "Review change marker on this line",
            non_color: "Change bar glyph",
            keyboard_path: "workbench.action.editor.nextChange",
            command_ref: Some("workbench.action.compareChanges"),
        },
        D::SearchMatchHighlight => DecorationSpec {
            placement: PlacementClass::InlineRange,
            actionability: ActionabilityClass::Activatable,
            source_label_class: Src::DeterministicLanguage,
            source_label: "Search",
            freshness: AssistFreshnessClass::Live,
            confidence: AssistConfidenceClass::High,
            motion: MotionClass::AnimatedReducible,
            screen_reader_label: "Search match",
            non_color: "Outlined box around the match",
            keyboard_path: "editor.action.nextMatchFindAction",
            command_ref: Some("editor.action.nextMatchFindAction"),
        },
        D::SelectionOccurrenceHighlight => DecorationSpec {
            placement: PlacementClass::InlineRange,
            actionability: ActionabilityClass::Activatable,
            source_label_class: Src::DeterministicLanguage,
            source_label: "Occurrences",
            freshness: AssistFreshnessClass::Live,
            confidence: AssistConfidenceClass::High,
            motion: MotionClass::Static,
            screen_reader_label: "Matching occurrence of the selection",
            non_color: "Outlined box around the occurrence",
            keyboard_path: "editor.action.moveSelectionToNextFindMatch",
            command_ref: Some("editor.action.moveSelectionToNextFindMatch"),
        },
        D::BracketMatch => DecorationSpec {
            placement: PlacementClass::InlineRange,
            actionability: ActionabilityClass::Activatable,
            source_label_class: Src::DeterministicLanguage,
            source_label: "Bracket matching",
            freshness: AssistFreshnessClass::Live,
            confidence: AssistConfidenceClass::High,
            motion: MotionClass::Static,
            screen_reader_label: "Matching bracket",
            non_color: "Outlined bracket pair",
            keyboard_path: "editor.action.jumpToBracket",
            command_ref: Some("editor.action.jumpToBracket"),
        },
        D::InlineDiffMarker => DecorationSpec {
            placement: PlacementClass::GutterGlyph,
            actionability: ActionabilityClass::Activatable,
            source_label_class: Src::ProjectGraph,
            source_label: "Inline diff",
            freshness: AssistFreshnessClass::Live,
            confidence: AssistConfidenceClass::High,
            motion: MotionClass::Static,
            screen_reader_label: "Changed line, inline diff marker",
            non_color: "Plus / minus gutter glyph",
            keyboard_path: "workbench.action.editor.nextChange",
            command_ref: Some("workbench.action.compareChanges"),
        },
    }
}

fn decoration_descriptor(class: DecorationClass, index: usize) -> AssistDescriptor {
    let spec = decoration_spec(class);
    let requires_visual_distinction = spec.source_label_class.requires_visual_distinction();
    let line = 10 + index as u32;
    AssistDescriptor {
        descriptor_id: format!("decoration:{}", class.as_str()),
        family: AssistDescriptorFamily::Decoration,
        class_token: class.as_str().to_owned(),
        class_label: class.label().to_owned(),
        owning_layer: class.owning_layer(),
        truth_tier: class.owning_layer().truth_tier(),
        channel: AssistChannelClass::Decoration,
        anchor: TextAnchor::new(line, 0, line, 12),
        placement: spec.placement,
        actionability: spec.actionability,
        source: DescriptorSource {
            source_label_class: spec.source_label_class,
            source_label: spec.source_label.to_owned(),
            provider_id: None,
            freshness: spec.freshness,
            confidence: spec.confidence,
            requires_ai_label: false,
            requires_visual_distinction,
        },
        accessibility: AccessibilityProfile {
            screen_reader_label: spec.screen_reader_label.to_owned(),
            non_color_differentiator: spec.non_color.to_owned(),
            keyboard_path: Some(spec.keyboard_path.to_owned()),
            motion_class: spec.motion,
        },
        // Decorations are editing truth: never compacted, never zoom-dropped, and
        // never layout-deferred while typing — they paint over existing layout.
        layout_shifting: false,
        density_optional: false,
        zoom_optional: false,
        command_ref: spec.command_ref.map(str::to_owned),
        note: format!(
            "Editing-truth {} owned by the {} precedence layer.",
            class.label().to_lowercase(),
            class.owning_layer().as_str()
        ),
    }
}

/// Specification for one code-lens class.
struct CodeLensSpec {
    source_label_class: AssistSourceLabelClass,
    source_label: &'static str,
    freshness: AssistFreshnessClass,
    confidence: AssistConfidenceClass,
    density_optional: bool,
    screen_reader_label: &'static str,
    keyboard_path: &'static str,
}

fn code_lens_spec(class: CodeLensClass) -> CodeLensSpec {
    use AssistSourceLabelClass as Src;
    use CodeLensClass as L;
    match class {
        L::ReferenceCount => CodeLensSpec {
            source_label_class: Src::ProjectGraph,
            source_label: "Project graph",
            freshness: AssistFreshnessClass::Live,
            confidence: AssistConfidenceClass::High,
            density_optional: false,
            screen_reader_label: "Reference count code lens",
            keyboard_path: "editor.action.referenceSearch.trigger",
        },
        L::ImplementationCount => CodeLensSpec {
            source_label_class: Src::ProjectGraph,
            source_label: "Project graph",
            freshness: AssistFreshnessClass::Live,
            confidence: AssistConfidenceClass::High,
            density_optional: false,
            screen_reader_label: "Implementation count code lens",
            keyboard_path: "editor.action.goToImplementation",
        },
        L::RunOrDebugAction => CodeLensSpec {
            source_label_class: Src::ToolAdapter,
            source_label: "Test runner",
            freshness: AssistFreshnessClass::Live,
            confidence: AssistConfidenceClass::High,
            density_optional: false,
            screen_reader_label: "Run or debug code lens",
            keyboard_path: "testing.runAtCursor",
        },
        L::TestStatus => CodeLensSpec {
            source_label_class: Src::ToolAdapter,
            source_label: "Test runner",
            freshness: AssistFreshnessClass::Cached,
            confidence: AssistConfidenceClass::High,
            density_optional: false,
            screen_reader_label: "Test status code lens",
            keyboard_path: "testing.reRunLastRun",
        },
        L::VcsAuthorship => CodeLensSpec {
            source_label_class: Src::ProjectGraph,
            source_label: "Version control",
            freshness: AssistFreshnessClass::Cached,
            confidence: AssistConfidenceClass::Probable,
            density_optional: true,
            screen_reader_label: "Authorship code lens",
            keyboard_path: "editor.action.revealDefinition",
        },
        L::AiExplainAction => CodeLensSpec {
            source_label_class: Src::AiInlineAssist,
            source_label: "AI assist",
            freshness: AssistFreshnessClass::Live,
            confidence: AssistConfidenceClass::Probable,
            density_optional: false,
            screen_reader_label: "AI explain code lens (AI-generated)",
            keyboard_path: "aureline.ai.explainSymbol",
        },
        L::GeneratedSourceOrigin => CodeLensSpec {
            source_label_class: Src::FrameworkProvider,
            source_label: "Generated source",
            freshness: AssistFreshnessClass::Live,
            confidence: AssistConfidenceClass::High,
            density_optional: false,
            screen_reader_label: "Generated-source origin code lens",
            keyboard_path: "aureline.generated.revealOrigin",
        },
    }
}

fn code_lens_descriptor(class: CodeLensClass, index: usize) -> AssistDescriptor {
    let spec = code_lens_spec(class);
    let requires_ai_label = class.requires_ai_label();
    let requires_visual_distinction =
        spec.source_label_class.requires_visual_distinction() || requires_ai_label;
    let line = 30 + index as u32 * 2;
    let non_color = if requires_ai_label {
        "Text label above the line, prefixed with an AI badge"
    } else {
        "Text label row above the line"
    };
    AssistDescriptor {
        descriptor_id: format!("hint:code-lens:{}", class.as_str()),
        family: AssistDescriptorFamily::CodeLens,
        class_token: class.as_str().to_owned(),
        class_label: class.label().to_owned(),
        owning_layer: EditorLayerClass::CodeLens,
        truth_tier: EditorLayerClass::CodeLens.truth_tier(),
        channel: AssistChannelClass::CodeLens,
        anchor: TextAnchor::new(line, 0, line, 1),
        placement: PlacementClass::AboveLine,
        actionability: ActionabilityClass::Activatable,
        source: DescriptorSource {
            source_label_class: spec.source_label_class,
            source_label: spec.source_label.to_owned(),
            provider_id: None,
            freshness: spec.freshness,
            confidence: spec.confidence,
            requires_ai_label,
            requires_visual_distinction,
        },
        accessibility: AccessibilityProfile {
            screen_reader_label: spec.screen_reader_label.to_owned(),
            non_color_differentiator: non_color.to_owned(),
            keyboard_path: Some(spec.keyboard_path.to_owned()),
            motion_class: MotionClass::Static,
        },
        // Code lenses add a row above the line: layout-shifting (vertical), and
        // version-control authorship is optional at dense spacing.
        layout_shifting: true,
        density_optional: spec.density_optional,
        zoom_optional: false,
        command_ref: Some(format!("editor.codeLens.activate.{}", class.as_str())),
        note: if requires_ai_label {
            "Convenience code lens; carries an explicit AI source label.".to_owned()
        } else {
            "Convenience code lens; subordinate to editing truth.".to_owned()
        },
    }
}

/// Specification for one inlay-hint class.
struct InlayHintSpec {
    placement: PlacementClass,
    source_label_class: AssistSourceLabelClass,
    source_label: &'static str,
    confidence: AssistConfidenceClass,
    density_optional: bool,
    zoom_optional: bool,
    screen_reader_label: &'static str,
}

fn inlay_hint_spec(class: InlayHintClass) -> InlayHintSpec {
    use AssistSourceLabelClass as Src;
    use InlayHintClass as H;
    match class {
        H::ParameterName => InlayHintSpec {
            placement: PlacementClass::InlineBefore,
            source_label_class: Src::DeterministicLanguage,
            source_label: "Language intelligence",
            confidence: AssistConfidenceClass::High,
            density_optional: false,
            zoom_optional: false,
            screen_reader_label: "Parameter name inlay hint",
        },
        H::InferredType => InlayHintSpec {
            placement: PlacementClass::InlineAfter,
            source_label_class: Src::DeterministicLanguage,
            source_label: "Language intelligence",
            confidence: AssistConfidenceClass::High,
            density_optional: false,
            zoom_optional: true,
            screen_reader_label: "Inferred type inlay hint",
        },
        H::ChainedCallType => InlayHintSpec {
            placement: PlacementClass::InlineAfter,
            source_label_class: Src::DeterministicLanguage,
            source_label: "Language intelligence",
            confidence: AssistConfidenceClass::Probable,
            density_optional: true,
            zoom_optional: true,
            screen_reader_label: "Chained-call type inlay hint",
        },
        H::EnumMemberValue => InlayHintSpec {
            placement: PlacementClass::InlineAfter,
            source_label_class: Src::DeterministicLanguage,
            source_label: "Language intelligence",
            confidence: AssistConfidenceClass::High,
            density_optional: true,
            zoom_optional: false,
            screen_reader_label: "Enum member value inlay hint",
        },
        H::ImplicitConversion => InlayHintSpec {
            placement: PlacementClass::InlineAfter,
            source_label_class: Src::DeterministicLanguage,
            source_label: "Language intelligence",
            confidence: AssistConfidenceClass::Probable,
            density_optional: true,
            zoom_optional: true,
            screen_reader_label: "Implicit conversion inlay hint",
        },
        H::AiInferred => InlayHintSpec {
            placement: PlacementClass::InlineAfter,
            source_label_class: Src::AiInlineAssist,
            source_label: "AI assist",
            confidence: AssistConfidenceClass::LowSpeculative,
            density_optional: false,
            zoom_optional: false,
            screen_reader_label: "AI-inferred inlay hint (AI-generated)",
        },
    }
}

fn inlay_hint_descriptor(class: InlayHintClass, index: usize) -> AssistDescriptor {
    let spec = inlay_hint_spec(class);
    let requires_ai_label = class.requires_ai_label();
    let requires_visual_distinction =
        spec.source_label_class.requires_visual_distinction() || requires_ai_label;
    let line = 50 + index as u32;
    let column = 8 + index as u32;
    let non_color = if requires_ai_label {
        "Dimmed inline text label with an AI badge"
    } else {
        "Dimmed inline text label"
    };
    AssistDescriptor {
        descriptor_id: format!("hint:inlay:{}", class.as_str()),
        family: AssistDescriptorFamily::InlayHint,
        class_token: class.as_str().to_owned(),
        class_label: class.label().to_owned(),
        owning_layer: EditorLayerClass::InlayHint,
        truth_tier: EditorLayerClass::InlayHint.truth_tier(),
        channel: AssistChannelClass::InlayHint,
        anchor: TextAnchor::new(line, column, line, column + 1),
        placement: spec.placement,
        actionability: ActionabilityClass::Activatable,
        source: DescriptorSource {
            source_label_class: spec.source_label_class,
            source_label: spec.source_label.to_owned(),
            provider_id: None,
            freshness: AssistFreshnessClass::Live,
            confidence: spec.confidence,
            requires_ai_label,
            requires_visual_distinction,
        },
        accessibility: AccessibilityProfile {
            screen_reader_label: spec.screen_reader_label.to_owned(),
            non_color_differentiator: non_color.to_owned(),
            keyboard_path: Some("editor.action.inlayHints.reveal".to_owned()),
            motion_class: MotionClass::Static,
        },
        // Inlay hints insert inline text: layout-shifting (horizontal), optional
        // at dense spacing and under a narrow high-zoom column budget per class.
        layout_shifting: true,
        density_optional: spec.density_optional,
        zoom_optional: spec.zoom_optional,
        command_ref: Some("editor.action.inlayHints.reveal".to_owned()),
        note: if requires_ai_label {
            "Convenience inlay hint; carries an explicit AI source label.".to_owned()
        } else {
            "Convenience inlay hint; non-editable annotation.".to_owned()
        },
    }
}

// ---------------------------------------------------------------------------
// Resolver.
// ---------------------------------------------------------------------------

/// Resolves one descriptor against one render context, given the surface's
/// degraded-state policy for the descriptor's channel from the frozen matrix.
///
/// The resolution order is the precedence the spec requires:
///
/// 1. The surface degraded-state policy sets the base verdict (full, fallback,
///    pending, reduced, suppressed, unavailable).
/// 2. Editing truth stops here: a decoration is never suppressed for a
///    convenience reason; its only reduction is the labeled large-file fallback.
/// 3. Convenience metadata is then refined by confidence, density, zoom, and the
///    typing budget — each with its own explicit reason — but never promoted
///    above the surface policy.
fn resolve_descriptor(
    descriptor: &AssistDescriptor,
    context: &RenderContext,
    degrade: AssistDegradeClass,
) -> ResolvedDescriptor {
    let is_convenience = matches!(descriptor.truth_tier, TruthTier::ConvenienceMetadata);

    let (mut visibility, mut reason, mut effective) = match degrade {
        AssistDegradeClass::FullFidelity => (
            VisibilityVerdict::Rendered,
            SuppressionReason::NotSuppressed,
            AssistDegradeClass::FullFidelity,
        ),
        AssistDegradeClass::SourceLabeledFallback => {
            let reason = if is_convenience {
                SuppressionReason::SourceFallback
            } else {
                // An editing-truth decoration reduced in large-file mode.
                SuppressionReason::ReducedDecoration
            };
            (
                VisibilityVerdict::Downgraded,
                reason,
                AssistDegradeClass::SourceLabeledFallback,
            )
        }
        AssistDegradeClass::ReadOnlyNoApply => (
            VisibilityVerdict::Rendered,
            SuppressionReason::NotSuppressed,
            AssistDegradeClass::ReadOnlyNoApply,
        ),
        AssistDegradeClass::PendingPartialIndex => (
            VisibilityVerdict::Downgraded,
            SuppressionReason::PartialIndexPending,
            AssistDegradeClass::PendingPartialIndex,
        ),
        AssistDegradeClass::SuppressedLargeFile => (
            VisibilityVerdict::Suppressed,
            SuppressionReason::LargeFileRestricted,
            AssistDegradeClass::SuppressedLargeFile,
        ),
        AssistDegradeClass::BlockedUnavailable => (
            VisibilityVerdict::Suppressed,
            SuppressionReason::UnavailableOnSurface,
            AssistDegradeClass::BlockedUnavailable,
        ),
    };

    // Convenience-only contextual refinement. Editing truth is never reduced
    // below the surface policy by a convenience condition.
    if is_convenience && visibility != VisibilityVerdict::Suppressed {
        if descriptor.source.confidence.is_low_certainty() {
            visibility = VisibilityVerdict::Suppressed;
            reason = SuppressionReason::LowConfidence;
            effective = AssistDegradeClass::BlockedUnavailable;
        } else if context.density == DensityTier::Dense && descriptor.density_optional {
            visibility = VisibilityVerdict::Suppressed;
            reason = SuppressionReason::DensityCompaction;
            effective = AssistDegradeClass::BlockedUnavailable;
        } else if context.zoom == ZoomTier::High && descriptor.zoom_optional {
            visibility = VisibilityVerdict::Suppressed;
            reason = SuppressionReason::HighZoomHorizontalBudget;
            effective = AssistDegradeClass::BlockedUnavailable;
        } else if context.typing_active && descriptor.layout_shifting {
            // Held, not dropped: returns once typing settles.
            visibility = VisibilityVerdict::Deferred;
            reason = SuppressionReason::TypingBudget;
        }
    }

    let animations_enabled = descriptor.accessibility.motion_class
        == MotionClass::AnimatedReducible
        && !context.reduced_motion;
    let keyboard_reachable = visibility.is_offered();

    ResolvedDescriptor {
        descriptor_id: descriptor.descriptor_id.clone(),
        family: descriptor.family,
        class_token: descriptor.class_token.clone(),
        owning_layer: descriptor.owning_layer,
        truth_tier: descriptor.truth_tier,
        rank: descriptor.rank(),
        visibility,
        effective_degrade: effective,
        suppression_reason: reason,
        reason_detail: resolution_reason_detail(descriptor, context, visibility, reason),
        animations_enabled,
        keyboard_reachable,
        source_label_class: descriptor.source.source_label_class,
    }
}

fn resolution_reason_detail(
    descriptor: &AssistDescriptor,
    context: &RenderContext,
    visibility: VisibilityVerdict,
    reason: SuppressionReason,
) -> String {
    let class = &descriptor.class_label;
    match reason {
        SuppressionReason::NotSuppressed => {
            format!("{class} renders at full fidelity on the {}.", context.label)
        }
        SuppressionReason::SourceFallback => {
            format!(
                "{class} renders from a labeled fallback source on the {}.",
                context.label
            )
        }
        SuppressionReason::PartialIndexPending => {
            format!("{class} is labeled pending until the semantic index finishes building.")
        }
        SuppressionReason::ReducedDecoration => {
            format!("{class} is reduced but still drawn in large-file / restricted mode.")
        }
        SuppressionReason::DensityCompaction => {
            format!("{class} is compacted away at dense spacing; reopen at comfortable density.")
        }
        SuppressionReason::HighZoomHorizontalBudget => {
            format!("{class} is dropped to protect the column budget at high zoom.")
        }
        SuppressionReason::TypingBudget => {
            format!("{class} is held while typing and returns once typing settles.")
        }
        SuppressionReason::LowConfidence => {
            format!("{class} is low-confidence and suppressed by default; enable to reveal.")
        }
        SuppressionReason::LargeFileRestricted => {
            format!("{class} is suppressed in large-file / restricted mode.")
        }
        SuppressionReason::UnavailableOnSurface => {
            format!("{class} is not offered on the {}.", context.label)
        }
        SuppressionReason::OutrankedByEditingTruth => {
            let _ = visibility;
            format!("{class} yields to the overlapping editing-truth decoration.")
        }
    }
}

// ---------------------------------------------------------------------------
// Scenario construction.
// ---------------------------------------------------------------------------

fn scenario_contexts() -> Vec<RenderContext> {
    vec![
        RenderContext {
            scenario_id: SCENARIO_CODE_FILE_COMFORTABLE.to_owned(),
            label: "code file, comfortable".to_owned(),
            surface: EditorSurfaceClass::CodeFile,
            density: DensityTier::Comfortable,
            zoom: ZoomTier::Standard,
            reduced_motion: false,
            typing_active: false,
        },
        RenderContext {
            scenario_id: SCENARIO_CODE_FILE_COMPACT.to_owned(),
            label: "code file, compact".to_owned(),
            surface: EditorSurfaceClass::CodeFile,
            density: DensityTier::Compact,
            zoom: ZoomTier::Standard,
            reduced_motion: false,
            typing_active: false,
        },
        RenderContext {
            scenario_id: SCENARIO_CODE_FILE_DENSE.to_owned(),
            label: "code file, dense".to_owned(),
            surface: EditorSurfaceClass::CodeFile,
            density: DensityTier::Dense,
            zoom: ZoomTier::Standard,
            reduced_motion: false,
            typing_active: false,
        },
        RenderContext {
            scenario_id: SCENARIO_CODE_FILE_HIGH_ZOOM.to_owned(),
            label: "code file, high zoom".to_owned(),
            surface: EditorSurfaceClass::CodeFile,
            density: DensityTier::Comfortable,
            zoom: ZoomTier::High,
            reduced_motion: false,
            typing_active: false,
        },
        RenderContext {
            scenario_id: SCENARIO_CODE_FILE_TYPING.to_owned(),
            label: "code file, typing".to_owned(),
            surface: EditorSurfaceClass::CodeFile,
            density: DensityTier::Comfortable,
            zoom: ZoomTier::Standard,
            reduced_motion: false,
            typing_active: true,
        },
        RenderContext {
            scenario_id: SCENARIO_CODE_FILE_REDUCED_MOTION.to_owned(),
            label: "code file, reduced motion".to_owned(),
            surface: EditorSurfaceClass::CodeFile,
            density: DensityTier::Comfortable,
            zoom: ZoomTier::Standard,
            reduced_motion: true,
            typing_active: false,
        },
        RenderContext {
            scenario_id: SCENARIO_SQL_EDITOR.to_owned(),
            label: "SQL editor, comfortable".to_owned(),
            surface: EditorSurfaceClass::SqlEditor,
            density: DensityTier::Comfortable,
            zoom: ZoomTier::Standard,
            reduced_motion: false,
            typing_active: false,
        },
        RenderContext {
            scenario_id: SCENARIO_DOCS_CODE_BLOCK.to_owned(),
            label: "docs-code block, comfortable".to_owned(),
            surface: EditorSurfaceClass::DocsCodeBlock,
            density: DensityTier::Comfortable,
            zoom: ZoomTier::Standard,
            reduced_motion: false,
            typing_active: false,
        },
        RenderContext {
            scenario_id: SCENARIO_PARTIAL_INDEX.to_owned(),
            label: "partial-index state".to_owned(),
            surface: EditorSurfaceClass::PartialIndexState,
            density: DensityTier::Comfortable,
            zoom: ZoomTier::Standard,
            reduced_motion: false,
            typing_active: false,
        },
        RenderContext {
            scenario_id: SCENARIO_GENERATED_FILE.to_owned(),
            label: "generated file".to_owned(),
            surface: EditorSurfaceClass::GeneratedFile,
            density: DensityTier::Comfortable,
            zoom: ZoomTier::Standard,
            reduced_motion: false,
            typing_active: false,
        },
        RenderContext {
            scenario_id: SCENARIO_LARGE_FILE.to_owned(),
            label: "large-file / restricted mode".to_owned(),
            surface: EditorSurfaceClass::LargeFileRestricted,
            density: DensityTier::Comfortable,
            zoom: ZoomTier::Standard,
            reduced_motion: false,
            typing_active: false,
        },
    ]
}

fn build_scenarios(
    matrix: &EditorAssistMatrix,
    catalog: &[AssistDescriptor],
) -> Vec<ResolutionScenario> {
    scenario_contexts()
        .into_iter()
        .map(|context| build_scenario(matrix, catalog, context))
        .collect()
}

fn build_scenario(
    matrix: &EditorAssistMatrix,
    catalog: &[AssistDescriptor],
    context: RenderContext,
) -> ResolutionScenario {
    let mut resolved: Vec<ResolvedDescriptor> = catalog
        .iter()
        .map(|descriptor| {
            let degrade = matrix
                .cell(context.surface, descriptor.channel)
                .map(|cell| cell.degrade_state)
                .expect("every surface binds every channel in the frozen matrix");
            resolve_descriptor(descriptor, &context, degrade)
        })
        .collect();

    // Stable order: precedence rank first (editing truth before convenience),
    // then descriptor id for determinism.
    resolved.sort_by(|a, b| {
        a.rank
            .cmp(&b.rank)
            .then_with(|| a.descriptor_id.cmp(&b.descriptor_id))
    });

    let count = |verdict: VisibilityVerdict| {
        resolved
            .iter()
            .filter(|item| item.visibility == verdict)
            .count()
    };
    let rendered_count = count(VisibilityVerdict::Rendered);
    let downgraded_count = count(VisibilityVerdict::Downgraded);
    let deferred_count = count(VisibilityVerdict::Deferred);
    let suppressed_count = count(VisibilityVerdict::Suppressed);

    let note = format!(
        "{label}: {rendered} rendered, {downgraded} downgraded, {deferred} deferred, {suppressed} suppressed.",
        label = context.label,
        rendered = rendered_count,
        downgraded = downgraded_count,
        deferred = deferred_count,
        suppressed = suppressed_count,
    );

    ResolutionScenario {
        context,
        resolved,
        rendered_count,
        downgraded_count,
        deferred_count,
        suppressed_count,
        note,
    }
}

// ---------------------------------------------------------------------------
// Precedence conflict construction.
// ---------------------------------------------------------------------------

fn build_precedence_conflicts(catalog: &[AssistDescriptor]) -> Vec<PrecedenceConflictCase> {
    let cases = [
        (
            "diagnostic_outranks_inferred_type",
            "Diagnostic underline overlaps an inferred-type inlay hint",
            "decoration:diagnostic_underline",
            "hint:inlay:inferred_type",
            TextAnchor::new(120, 4, 120, 20),
        ),
        (
            "debug_frame_outranks_reference_lens",
            "Current debug line overlaps a reference-count code lens",
            "decoration:debug_current_line",
            "hint:code-lens:reference_count",
            TextAnchor::new(121, 0, 121, 40),
        ),
        (
            "conflict_outranks_parameter_hint",
            "Merge-conflict region overlaps a parameter-name inlay hint",
            "decoration:merge_conflict_region",
            "hint:inlay:parameter_name",
            TextAnchor::new(122, 0, 122, 30),
        ),
    ];

    cases
        .iter()
        .map(|(case_id, label, truth_id, conv_id, shared_anchor)| {
            let truth = catalog
                .iter()
                .find(|descriptor| descriptor.descriptor_id == *truth_id)
                .expect("editing-truth descriptor present in catalog");
            let conv = catalog
                .iter()
                .find(|descriptor| descriptor.descriptor_id == *conv_id)
                .expect("convenience descriptor present in catalog");
            let (winner, yielded, yielded_visibility, yielded_reason) =
                resolve_overlap(truth, conv);
            PrecedenceConflictCase {
                case_id: (*case_id).to_owned(),
                label: (*label).to_owned(),
                editing_truth_descriptor_id: truth.descriptor_id.clone(),
                editing_truth_layer: truth.owning_layer,
                convenience_descriptor_id: conv.descriptor_id.clone(),
                convenience_layer: conv.owning_layer,
                shared_anchor: *shared_anchor,
                winner_descriptor_id: winner,
                yielded_descriptor_id: yielded,
                yielded_visibility,
                yielded_reason,
                note: format!(
                    "{} ranks {} and outranks {} (rank {}); the convenience descriptor yields.",
                    truth.class_label,
                    truth.rank(),
                    conv.class_label,
                    conv.rank(),
                ),
            }
        })
        .collect()
}

/// Resolves an overlap between an editing-truth and a convenience descriptor on a
/// shared anchor. The lower-ranked (editing-truth) descriptor wins; the
/// convenience descriptor yields, held rather than dropped.
fn resolve_overlap(
    truth: &AssistDescriptor,
    convenience: &AssistDescriptor,
) -> (String, String, VisibilityVerdict, SuppressionReason) {
    // Lower rank wins. Editing truth always ranks below convenience by
    // construction, so the editing-truth descriptor is always the winner.
    if truth.rank() <= convenience.rank() {
        (
            truth.descriptor_id.clone(),
            convenience.descriptor_id.clone(),
            VisibilityVerdict::Deferred,
            SuppressionReason::OutrankedByEditingTruth,
        )
    } else {
        (
            convenience.descriptor_id.clone(),
            truth.descriptor_id.clone(),
            VisibilityVerdict::Deferred,
            SuppressionReason::OutrankedByEditingTruth,
        )
    }
}

// ---------------------------------------------------------------------------
// Invariant evaluation.
// ---------------------------------------------------------------------------

fn evaluate_invariants(
    catalog: &[AssistDescriptor],
    scenarios: &[ResolutionScenario],
    conflicts: &[PrecedenceConflictCase],
) -> Vec<ModelInvariant> {
    vec![
        ModelInvariant {
            invariant_id: "descriptor_catalog_covers_every_class".to_owned(),
            statement:
                "The catalog holds exactly one descriptor per decoration, code-lens, and inlay-hint class."
                    .to_owned(),
            holds: descriptor_catalog_covers_every_class(catalog),
        },
        ModelInvariant {
            invariant_id: "editing_truth_never_convenience_suppressed".to_owned(),
            statement:
                "No editing-truth decoration is ever suppressed or deferred for a convenience reason in any scenario."
                    .to_owned(),
            holds: editing_truth_never_convenience_suppressed(scenarios),
        },
        ModelInvariant {
            invariant_id: "convenience_outranked_by_truth_when_rendered".to_owned(),
            statement:
                "In every scenario, every rendered convenience descriptor ranks below every rendered editing-truth descriptor."
                    .to_owned(),
            holds: convenience_outranked_by_truth_when_rendered(scenarios),
        },
        ModelInvariant {
            invariant_id: "non_rendered_resolutions_carry_reason".to_owned(),
            statement:
                "Every downgraded, deferred, or suppressed resolution carries an explicit reason and detail."
                    .to_owned(),
            holds: non_rendered_resolutions_carry_reason(scenarios),
        },
        ModelInvariant {
            invariant_id: "rendered_resolutions_have_no_suppression_reason".to_owned(),
            statement: "Every fully rendered resolution carries the not-suppressed reason."
                .to_owned(),
            holds: rendered_resolutions_have_no_suppression_reason(scenarios),
        },
        ModelInvariant {
            invariant_id: "actionable_or_severity_decorations_fully_accessible".to_owned(),
            statement:
                "Every actionable or severity-bearing decoration declares a keyboard path, screen-reader label, and non-color differentiator."
                    .to_owned(),
            holds: actionable_or_severity_decorations_fully_accessible(catalog),
        },
        ModelInvariant {
            invariant_id: "every_descriptor_has_non_color_and_screen_reader".to_owned(),
            statement:
                "Every descriptor carries a non-empty screen-reader label and non-color differentiator."
                    .to_owned(),
            holds: every_descriptor_has_non_color_and_screen_reader(catalog),
        },
        ModelInvariant {
            invariant_id: "ai_descriptors_carry_ai_label".to_owned(),
            statement:
                "Every AI-sourced descriptor is labeled AI inline assist and kept visually distinct."
                    .to_owned(),
            holds: ai_descriptors_carry_ai_label(catalog),
        },
        ModelInvariant {
            invariant_id: "reduced_motion_disables_animation".to_owned(),
            statement: "In the reduced-motion scenario, no resolved descriptor enables animation."
                .to_owned(),
            holds: reduced_motion_disables_animation(scenarios),
        },
        ModelInvariant {
            invariant_id: "large_file_suppresses_convenience_keeps_decorations".to_owned(),
            statement:
                "In large-file mode every convenience descriptor is suppressed and every decoration is still drawn."
                    .to_owned(),
            holds: large_file_suppresses_convenience_keeps_decorations(scenarios),
        },
        ModelInvariant {
            invariant_id: "low_confidence_convenience_suppressed".to_owned(),
            statement:
                "On a full-fidelity code file, every low-confidence convenience descriptor is suppressed with the low-confidence reason."
                    .to_owned(),
            holds: low_confidence_convenience_suppressed(scenarios),
        },
        ModelInvariant {
            invariant_id: "typing_defers_layout_shifting_convenience".to_owned(),
            statement:
                "While typing, no layout-shifting convenience descriptor renders; each is deferred or otherwise suppressed."
                    .to_owned(),
            holds: typing_defers_layout_shifting_convenience(scenarios),
        },
        ModelInvariant {
            invariant_id: "keyboard_reachable_iff_offered".to_owned(),
            statement:
                "A resolved descriptor is keyboard-reachable exactly when it is rendered or downgraded."
                    .to_owned(),
            holds: keyboard_reachable_iff_offered(scenarios),
        },
        ModelInvariant {
            invariant_id: "lens_and_hint_ids_reuse_frozen_prefix".to_owned(),
            statement:
                "Every code-lens and inlay-hint descriptor id reuses the frozen hint-descriptor id prefix."
                    .to_owned(),
            holds: lens_and_hint_ids_reuse_frozen_prefix(catalog),
        },
        ModelInvariant {
            invariant_id: "precedence_conflicts_resolve_to_editing_truth".to_owned(),
            statement:
                "Every interaction-precedence conflict is won by the editing-truth descriptor; the convenience descriptor yields."
                    .to_owned(),
            holds: precedence_conflicts_resolve_to_editing_truth(conflicts),
        },
    ]
}

fn descriptor_catalog_covers_every_class(catalog: &[AssistDescriptor]) -> bool {
    let expected =
        DecorationClass::ALL.len() + CodeLensClass::ALL.len() + InlayHintClass::ALL.len();
    if catalog.len() != expected {
        return false;
    }
    let has = |id: &str| catalog.iter().any(|d| d.descriptor_id == id);
    let decorations_ok = DecorationClass::ALL
        .iter()
        .all(|class| has(&format!("decoration:{}", class.as_str())));
    let lenses_ok = CodeLensClass::ALL
        .iter()
        .all(|class| has(&format!("hint:code-lens:{}", class.as_str())));
    let hints_ok = InlayHintClass::ALL
        .iter()
        .all(|class| has(&format!("hint:inlay:{}", class.as_str())));
    decorations_ok && lenses_ok && hints_ok
}

fn editing_truth_never_convenience_suppressed(scenarios: &[ResolutionScenario]) -> bool {
    scenarios.iter().all(|scenario| {
        scenario
            .resolved
            .iter()
            .filter(|resolved| resolved.truth_tier == TruthTier::EditingTruth)
            .all(|resolved| {
                // Editing truth is only ever rendered or reduced (downgraded), and
                // never carries a convenience reason.
                matches!(
                    resolved.visibility,
                    VisibilityVerdict::Rendered | VisibilityVerdict::Downgraded
                ) && matches!(
                    resolved.suppression_reason,
                    SuppressionReason::NotSuppressed | SuppressionReason::ReducedDecoration
                )
            })
    })
}

fn convenience_outranked_by_truth_when_rendered(scenarios: &[ResolutionScenario]) -> bool {
    scenarios.iter().all(|scenario| {
        let max_truth_rank = scenario
            .resolved
            .iter()
            .filter(|r| r.truth_tier == TruthTier::EditingTruth && r.visibility.is_offered())
            .map(|r| r.rank)
            .max();
        let min_convenience_rank = scenario
            .resolved
            .iter()
            .filter(|r| r.truth_tier == TruthTier::ConvenienceMetadata && r.visibility.is_offered())
            .map(|r| r.rank)
            .min();
        match (max_truth_rank, min_convenience_rank) {
            (Some(truth), Some(convenience)) => truth < convenience,
            _ => true,
        }
    })
}

fn non_rendered_resolutions_carry_reason(scenarios: &[ResolutionScenario]) -> bool {
    scenarios.iter().all(|scenario| {
        scenario.resolved.iter().all(|resolved| {
            if resolved.visibility == VisibilityVerdict::Rendered {
                true
            } else {
                resolved.suppression_reason != SuppressionReason::NotSuppressed
                    && !resolved.reason_detail.trim().is_empty()
            }
        })
    })
}

fn rendered_resolutions_have_no_suppression_reason(scenarios: &[ResolutionScenario]) -> bool {
    scenarios.iter().all(|scenario| {
        scenario
            .resolved
            .iter()
            .filter(|resolved| resolved.visibility == VisibilityVerdict::Rendered)
            .all(|resolved| resolved.suppression_reason == SuppressionReason::NotSuppressed)
    })
}

fn actionable_or_severity_decorations_fully_accessible(catalog: &[AssistDescriptor]) -> bool {
    catalog
        .iter()
        .filter(|descriptor| descriptor.family == AssistDescriptorFamily::Decoration)
        .filter(|descriptor| descriptor.actionability.requires_keyboard_path())
        .all(|descriptor| {
            descriptor.accessibility.keyboard_path.is_some()
                && !descriptor
                    .accessibility
                    .screen_reader_label
                    .trim()
                    .is_empty()
                && !descriptor
                    .accessibility
                    .non_color_differentiator
                    .trim()
                    .is_empty()
        })
}

fn every_descriptor_has_non_color_and_screen_reader(catalog: &[AssistDescriptor]) -> bool {
    catalog.iter().all(|descriptor| {
        !descriptor
            .accessibility
            .screen_reader_label
            .trim()
            .is_empty()
            && !descriptor
                .accessibility
                .non_color_differentiator
                .trim()
                .is_empty()
    })
}

fn ai_descriptors_carry_ai_label(catalog: &[AssistDescriptor]) -> bool {
    catalog
        .iter()
        .filter(|descriptor| descriptor.source.requires_ai_label)
        .all(|descriptor| {
            descriptor.source.source_label_class == AssistSourceLabelClass::AiInlineAssist
                && descriptor.source.requires_visual_distinction
        })
}

fn reduced_motion_disables_animation(scenarios: &[ResolutionScenario]) -> bool {
    let Some(scenario) = scenarios
        .iter()
        .find(|scenario| scenario.context.scenario_id == SCENARIO_CODE_FILE_REDUCED_MOTION)
    else {
        return false;
    };
    scenario
        .resolved
        .iter()
        .all(|resolved| !resolved.animations_enabled)
}

fn large_file_suppresses_convenience_keeps_decorations(scenarios: &[ResolutionScenario]) -> bool {
    let Some(scenario) = scenarios
        .iter()
        .find(|scenario| scenario.context.scenario_id == SCENARIO_LARGE_FILE)
    else {
        return false;
    };
    scenario.resolved.iter().all(|resolved| {
        match resolved.truth_tier {
            TruthTier::ConvenienceMetadata => {
                resolved.visibility == VisibilityVerdict::Suppressed
                    && resolved.suppression_reason == SuppressionReason::LargeFileRestricted
            }
            // Decorations stay drawn (rendered or reduced), never suppressed.
            TruthTier::EditingTruth => matches!(
                resolved.visibility,
                VisibilityVerdict::Rendered | VisibilityVerdict::Downgraded
            ),
        }
    })
}

fn low_confidence_convenience_suppressed(scenarios: &[ResolutionScenario]) -> bool {
    let Some(scenario) = scenarios
        .iter()
        .find(|scenario| scenario.context.scenario_id == SCENARIO_CODE_FILE_COMFORTABLE)
    else {
        return false;
    };
    // The AI-inferred inlay hint is the canonical low-confidence convenience
    // descriptor; it must be suppressed with the low-confidence reason on an
    // otherwise full-fidelity code file.
    let Some(ai_hint) = scenario.resolved("hint:inlay:ai_inferred") else {
        return false;
    };
    ai_hint.visibility == VisibilityVerdict::Suppressed
        && ai_hint.suppression_reason == SuppressionReason::LowConfidence
}

fn typing_defers_layout_shifting_convenience(scenarios: &[ResolutionScenario]) -> bool {
    let Some(scenario) = scenarios
        .iter()
        .find(|scenario| scenario.context.scenario_id == SCENARIO_CODE_FILE_TYPING)
    else {
        return false;
    };
    // No convenience descriptor renders while typing: each is either deferred for
    // the typing budget or already suppressed for another reason.
    scenario
        .resolved
        .iter()
        .filter(|resolved| resolved.truth_tier == TruthTier::ConvenienceMetadata)
        .all(|resolved| {
            matches!(
                resolved.visibility,
                VisibilityVerdict::Deferred | VisibilityVerdict::Suppressed
            )
        })
}

fn keyboard_reachable_iff_offered(scenarios: &[ResolutionScenario]) -> bool {
    scenarios.iter().all(|scenario| {
        scenario
            .resolved
            .iter()
            .all(|resolved| resolved.keyboard_reachable == resolved.visibility.is_offered())
    })
}

fn lens_and_hint_ids_reuse_frozen_prefix(catalog: &[AssistDescriptor]) -> bool {
    let prefix = MicroSurfaceKind::HintDescriptor.id_prefix();
    catalog
        .iter()
        .filter(|descriptor| {
            matches!(
                descriptor.family,
                AssistDescriptorFamily::CodeLens | AssistDescriptorFamily::InlayHint
            )
        })
        .all(|descriptor| descriptor.descriptor_id.starts_with(prefix))
}

fn precedence_conflicts_resolve_to_editing_truth(conflicts: &[PrecedenceConflictCase]) -> bool {
    !conflicts.is_empty()
        && conflicts.iter().all(|case| {
            case.editing_truth_layer.truth_tier() == TruthTier::EditingTruth
                && case.convenience_layer.truth_tier() == TruthTier::ConvenienceMetadata
                && case.winner_descriptor_id == case.editing_truth_descriptor_id
                && case.yielded_descriptor_id == case.convenience_descriptor_id
                && case.yielded_visibility == VisibilityVerdict::Deferred
                && case.yielded_reason == SuppressionReason::OutrankedByEditingTruth
        })
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the export-safe, human-readable lines for the assist-descriptor model.
///
/// This is the shared projection consumed by Help/About, the headless CLI
/// emitter, and support export, so they never clone model text from each other.
pub fn assist_descriptor_model_lines(model: &AssistDescriptorModel) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Assist-descriptor model — {} ({})",
        model.model_id, model.as_of
    ));
    lines.push(format!(
        "schema_ref={} version={}",
        model.schema_ref, model.m5_assist_descriptors_schema_version
    ));

    lines.push("Descriptor catalog:".to_owned());
    for descriptor in &model.descriptor_catalog {
        lines.push(format!(
            "  [{rank:02}] {id} ({family}/{tier}) source={source} confidence={confidence} action={action}",
            rank = descriptor.rank(),
            id = descriptor.descriptor_id,
            family = descriptor.family.as_str(),
            tier = descriptor.truth_tier.as_str(),
            source = descriptor.source.source_label_class.as_str(),
            confidence = descriptor.source.confidence.as_str(),
            action = descriptor.actionability.as_str(),
        ));
    }

    lines.push("Scenarios:".to_owned());
    for scenario in &model.scenarios {
        lines.push(format!(
            "  {id} ({surface}): {note}",
            id = scenario.context.scenario_id,
            surface = scenario.context.surface.as_str(),
            note = scenario.note,
        ));
        for resolved in &scenario.resolved {
            lines.push(format!(
                "    {id} = {visibility} reason={reason} keyboard={keyboard} anim={anim}",
                id = resolved.descriptor_id,
                visibility = resolved.visibility.as_str(),
                reason = resolved.suppression_reason.as_str(),
                keyboard = resolved.keyboard_reachable,
                anim = resolved.animations_enabled,
            ));
        }
    }

    lines.push("Precedence conflicts:".to_owned());
    for case in &model.precedence_conflicts {
        lines.push(format!(
            "  {id}: {winner} wins, {yielded} -> {visibility} ({reason})",
            id = case.case_id,
            winner = case.winner_descriptor_id,
            yielded = case.yielded_descriptor_id,
            visibility = case.yielded_visibility.as_str(),
            reason = case.yielded_reason.as_str(),
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

#[cfg(test)]
mod tests;
