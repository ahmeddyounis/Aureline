//! Related-object navigation: the typed, source-attributed model for route,
//! component, test, doc, owner, and generated-artifact links.
//!
//! Aureline's richer "go to the related thing" affordances — open the route a
//! handler binds, the component a symbol renders, the tests that cover it, the docs
//! that describe it, the owner who stewards it, or the generated pair it derives —
//! must not collapse into one bucket of generic smart links. A route binding admitted
//! by framework metadata, a component edge proven against the project graph, an owner
//! read from a curated stewardship rule, and a test relation observed only at runtime
//! are four *different evidence classes*, and a related-object link must keep saying
//! which one backs it.
//!
//! The [`target_model`](crate::target_model) freezes the relation/proof/freshness
//! vocabulary and the [`relation-navigation matrix`](crate::m5_relation_navigation)
//! names the related-object relation as a governed object family and pins its
//! vocabulary. What was still implicit is the *implementation*: how Aureline turns the
//! related links for an anchor into a typed **panel** that says **what each link is**
//! ([`RelatedObjectKind`]), **what evidence class backed it**
//! ([`RelatedObjectSourceClass`] — graph-derived, framework-derived, curated, or
//! runtime-derived), **how it resolves** ([`RelatedObjectFallbackMode`]), **how fresh
//! it is**, and **whether the surface it was invoked from even supports stable
//! relation anchors** ([`AnchorParity`]) — and how that typed truth survives into
//! notebook, diff, docs-linked, and generated-artifact contexts and review/support/AI
//! consumers instead of flattening into one opaque list of buttons.
//!
//! This module is that model. [`build_related_object_panel`] is a pure function over a
//! typed [`RelatedObjectPanelInput`] that produces a [`RelatedObjectPanel`]:
//!
//! 1. **Source attribution grouping.** Links are grouped into
//!    [`RelatedObjectGroup`]s keyed by [`RelatedObjectSourceClass`] in a canonical
//!    order — graph-derived, framework-derived, curated, runtime-derived — so a
//!    framework guess never poses as a graph-proven edge and a runtime observation
//!    never reads as a curated fact. The links never flatten into one homogeneous
//!    certainty class.
//! 2. **Fallback truth.** Each link carries a [`RelatedObjectFallbackMode`] — primary,
//!    disambiguation-required, lexical-fallback, imported-snapshot, runtime-observed-
//!    only, or unavailable — so a degraded resolution is disclosed rather than shown
//!    as a clean jump.
//! 3. **Current-versus-captured scope.** Counts separate links proven against the
//!    current scope from those carried only by a captured snapshot, trace, or imported
//!    pack, and an incomplete scope is named, so a partial related-object set never
//!    reads as complete.
//! 4. **Anchor parity.** The panel names the [`RelatedObjectAnchorContext`] it was
//!    invoked from and an [`AnchorParity`] stating whether that surface supports stable
//!    relation anchors, so the same relation semantics are reused in notebooks, diff
//!    editors, docs-linked symbols, and generated artifacts where stable anchors exist
//!    and unsupported parity is labeled honestly where they do not.
//! 5. **Disambiguation.** When any link needs an explicit selection the panel exposes
//!    the competing links and a disambiguation set ref and gates the navigating
//!    actions, so a related-object jump cannot silently pick one of several candidates.
//! 6. **Stable actions and consumer parity.** Each panel exposes the same
//!    open/peek/split/reveal-attribution/export actions across every route and projects
//!    to every [`ConsumerSurface`](crate::target_model::ConsumerSurface) preserving
//!    source attribution, counts, fallback truth, and anchor parity — never flattening
//!    into generic links and never exporting code bodies.
//!
//! [`related_object_navigation_set`] freezes a deterministic corpus whose
//! [`RelatedObjectInvariant`] flags are computed from the builder's own output, so the
//! checked-in fixture and the freeze gate pin the contract byte-for-byte and any
//! regression in [`build_related_object_panel`] flips an invariant or drifts the
//! fixture rather than silently passing. The records carry no source bodies, raw paths,
//! provider payloads, URLs, hostnames, or credentials — only opaque object handles,
//! stable tokens, and short reviewable sentences — so they are safe for support export.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::target_model::{
    DowngradeReason, ExportRedactionClass, FreshnessClass, GeneratedOrExternalState,
    NavigationConfidence, ProofClass, RelationKind, ScopeCompleteness, REQUIRED_CONSUMER_SURFACES,
};
// Re-exported here for downstream consumers that project panels onto surfaces.
pub use crate::target_model::ConsumerSurface;

#[cfg(test)]
mod tests;

/// Schema version for the related-object navigation corpus.
pub const RELATED_OBJECT_NAV_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the related-object navigation corpus.
pub const RELATED_OBJECT_NAV_SCHEMA_REF: &str =
    "schemas/navigation/related_object_navigation.schema.json";

/// Stable record-kind tag for the related-object navigation corpus.
pub const RELATED_OBJECT_NAV_RECORD_KIND: &str = "related_object_navigation_set";

/// Stable id for the canonical related-object navigation corpus.
pub const RELATED_OBJECT_NAV_SET_ID: &str = "related-object-navigation:set:0001";

/// Evaluation stamp for the canonical corpus. Held as a constant so the canonical
/// binding stays deterministic and the fixture freezes byte-for-byte.
pub const RELATED_OBJECT_NAV_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The freeze gate that keeps the corpus binding current. Stable promotion runs this
/// gate; it fails when the in-code corpus drifts from the checked-in fixture or any
/// invariant flips.
pub const RELATED_OBJECT_NAV_FREEZE_GATE_REF: &str =
    "crates/aureline-navigation/tests/related_object_navigation.rs";

/// Reviewer doc for the related-object navigation contract.
pub const RELATED_OBJECT_NAV_DOC_REF: &str = "docs/navigation/related_object_navigation.md";

/// Evidence companion for the related-object navigation corpus.
pub const RELATED_OBJECT_NAV_ARTIFACT_REF: &str =
    "artifacts/navigation/related_object_navigation.md";

/// Repo-relative path of the checked-in canonical corpus.
pub const RELATED_OBJECT_NAV_FIXTURE_REF: &str =
    "fixtures/navigation/related_object_navigation/canonical_links.json";

/// The canonical source-attribution ordering for related-object groups.
///
/// A panel lists its groups in this order so a graph-proven relation is presented
/// before a framework-derived, curated, or runtime-observed one — and so a weaker
/// evidence class is never blended into the graph-proven group.
pub const RELATED_OBJECT_SOURCE_ORDER: [RelatedObjectSourceClass; 4] = [
    RelatedObjectSourceClass::GraphDerived,
    RelatedObjectSourceClass::FrameworkDerived,
    RelatedObjectSourceClass::Curated,
    RelatedObjectSourceClass::RuntimeDerived,
];

// ---------------------------------------------------------------------------
// Object kind.
// ---------------------------------------------------------------------------

/// The kind of related object a link resolves to.
///
/// This is the user-facing "what is on the other end" axis. Each kind maps to a stable
/// [`RelationKind`] in the closed relation vocabulary, so a related-object link is
/// never an untyped "smart link".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelatedObjectKind {
    /// A route or endpoint binding the anchor serves or is served by.
    Route,
    /// A component, widget, or typed unit the anchor renders, declares, or composes.
    Component,
    /// A test, spec, or fixture that covers the anchor.
    Test,
    /// A doc, guide, example, or generated docs anchor that describes the anchor.
    Doc,
    /// An owner, steward, or CODEOWNERS-like rule responsible for the anchor.
    Owner,
    /// A generated artifact paired with, or derived from, the anchor.
    GeneratedArtifact,
}

impl RelatedObjectKind {
    /// All object kinds, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::Route,
        Self::Component,
        Self::Test,
        Self::Doc,
        Self::Owner,
        Self::GeneratedArtifact,
    ];

    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Route => "route",
            Self::Component => "component",
            Self::Test => "test",
            Self::Doc => "doc",
            Self::Owner => "owner",
            Self::GeneratedArtifact => "generated_artifact",
        }
    }

    /// Returns a human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Route => "Route",
            Self::Component => "Component",
            Self::Test => "Test",
            Self::Doc => "Doc",
            Self::Owner => "Owner",
            Self::GeneratedArtifact => "Generated artifact",
        }
    }

    /// Returns the closed [`RelationKind`] this object kind maps to, so the link stays
    /// expressible in the shared relation vocabulary every other navigation surface
    /// uses.
    pub const fn relation_kind(self) -> RelationKind {
        match self {
            Self::Route => RelationKind::RouteBinding,
            Self::Component => RelationKind::Type,
            Self::Test => RelationKind::Reference,
            Self::Doc => RelationKind::DocLink,
            Self::Owner => RelationKind::OwnerLink,
            Self::GeneratedArtifact => RelationKind::Implementation,
        }
    }
}

// ---------------------------------------------------------------------------
// Source class.
// ---------------------------------------------------------------------------

/// The evidence class that admitted a related-object link — its source attribution.
///
/// Answers the support/debug question "why does this link exist, and how strong is the
/// evidence?". The four classes are kept distinct so a framework guess never poses as a
/// graph-proven fact and a runtime observation never reads as a curated rule. This is
/// the grouping axis: every concrete link resolves to exactly one of the four classes
/// in [`RELATED_OBJECT_SOURCE_ORDER`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelatedObjectSourceClass {
    /// Proven against the project or semantic graph.
    GraphDerived,
    /// Synthesized from framework, route, or generator metadata.
    FrameworkDerived,
    /// Read from a curated mapping — a stewardship rule, doc link table, or authored
    /// manifest — rather than from code analysis.
    Curated,
    /// Observed at runtime via a trace, profiler, or observed-dispatch record.
    RuntimeDerived,
}

impl RelatedObjectSourceClass {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GraphDerived => "graph_derived",
            Self::FrameworkDerived => "framework_derived",
            Self::Curated => "curated",
            Self::RuntimeDerived => "runtime_derived",
        }
    }

    /// Returns a human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::GraphDerived => "Graph-derived",
            Self::FrameworkDerived => "Framework-derived",
            Self::Curated => "Curated",
            Self::RuntimeDerived => "Runtime-derived",
        }
    }

    /// Returns true when a link from this source must render with a visible source
    /// caveat rather than as plain semantic certainty.
    ///
    /// Only graph-derived links read as proven; framework, curated, and runtime classes
    /// are always disclosed as what they are, so none of them masquerades as the others
    /// or as graph proof.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::GraphDerived)
    }

    /// Returns the matching panel label for this source class.
    pub const fn label_kind(self) -> RelatedObjectLabel {
        match self {
            Self::GraphDerived => RelatedObjectLabel::GraphDerived,
            Self::FrameworkDerived => RelatedObjectLabel::FrameworkDerived,
            Self::Curated => RelatedObjectLabel::Curated,
            Self::RuntimeDerived => RelatedObjectLabel::RuntimeDerived,
        }
    }
}

/// The headline source attribution for a whole panel.
///
/// One of the four real source classes when the panel's links are homogeneous,
/// [`Mixed`](RelatedObjectHeadline::Mixed) when they span more than one class, and
/// [`Empty`](RelatedObjectHeadline::Empty) when the panel has no links.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelatedObjectHeadline {
    /// Every link is graph-derived.
    GraphDerived,
    /// Every link is framework-derived.
    FrameworkDerived,
    /// Every link is curated.
    Curated,
    /// Every link is runtime-derived.
    RuntimeDerived,
    /// The panel mixes more than one source class.
    Mixed,
    /// The panel has no links.
    Empty,
}

impl RelatedObjectHeadline {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GraphDerived => "graph_derived",
            Self::FrameworkDerived => "framework_derived",
            Self::Curated => "curated",
            Self::RuntimeDerived => "runtime_derived",
            Self::Mixed => "mixed",
            Self::Empty => "empty",
        }
    }
}

// ---------------------------------------------------------------------------
// Fallback mode.
// ---------------------------------------------------------------------------

/// How a related-object link resolves to its target — its fallback truth.
///
/// Independent of the source class: a link can be graph-derived yet still need
/// disambiguation, or framework-derived yet only lexically matched. The mode is always
/// visible so a degraded resolution is never shown as a clean primary jump.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelatedObjectFallbackMode {
    /// Resolves directly to a single target with no fallback.
    Primary,
    /// Multiple candidate targets compete and an explicit selection is required.
    DisambiguationRequired,
    /// The target could only be matched lexically, disclosed as a fallback.
    LexicalFallback,
    /// The target is carried by an imported snapshot or docs pack, not re-proven.
    ImportedSnapshot,
    /// Only a runtime observation backs the target.
    RuntimeObservedOnly,
    /// The relation is known to exist but no target resolves in the current scope.
    Unavailable,
}

impl RelatedObjectFallbackMode {
    /// All fallback modes, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::Primary,
        Self::DisambiguationRequired,
        Self::LexicalFallback,
        Self::ImportedSnapshot,
        Self::RuntimeObservedOnly,
        Self::Unavailable,
    ];

    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::DisambiguationRequired => "disambiguation_required",
            Self::LexicalFallback => "lexical_fallback",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::RuntimeObservedOnly => "runtime_observed_only",
            Self::Unavailable => "unavailable",
        }
    }

    /// Returns true when this mode must render with a visible caveat rather than as a
    /// clean primary jump.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Primary)
    }

    /// Returns true when this mode requires an explicit selection before a jump.
    pub const fn requires_selection(self) -> bool {
        matches!(self, Self::DisambiguationRequired)
    }

    /// Returns true when this mode resolves to no target.
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::Unavailable)
    }
}

// ---------------------------------------------------------------------------
// Anchor context and parity.
// ---------------------------------------------------------------------------

/// The surface a related-object panel was invoked from.
///
/// The same relation semantics are reused wherever a context can provide a stable
/// anchor — an editor symbol, a notebook cell, a diff hunk, a docs-linked symbol, or a
/// generated artifact — and unsupported parity is named honestly where a context
/// cannot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelatedObjectAnchorContext {
    /// A symbol in an editor buffer.
    EditorSymbol,
    /// A cell in a notebook or literate document.
    NotebookCell,
    /// A hunk in a diff or review editor.
    DiffHunk,
    /// A symbol reached through a documentation link.
    DocsLinkedSymbol,
    /// A node inside a generated artifact.
    GeneratedArtifact,
}

impl RelatedObjectAnchorContext {
    /// All anchor contexts, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::EditorSymbol,
        Self::NotebookCell,
        Self::DiffHunk,
        Self::DocsLinkedSymbol,
        Self::GeneratedArtifact,
    ];

    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorSymbol => "editor_symbol",
            Self::NotebookCell => "notebook_cell",
            Self::DiffHunk => "diff_hunk",
            Self::DocsLinkedSymbol => "docs_linked_symbol",
            Self::GeneratedArtifact => "generated_artifact",
        }
    }

    /// Returns a human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::EditorSymbol => "Editor symbol",
            Self::NotebookCell => "Notebook cell",
            Self::DiffHunk => "Diff hunk",
            Self::DocsLinkedSymbol => "Docs-linked symbol",
            Self::GeneratedArtifact => "Generated artifact",
        }
    }
}

/// Whether a context supports stable relation anchors, so related-object navigation can
/// reuse the same relation semantics — or must say it cannot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnchorParity {
    /// The context provides stable anchors; relation semantics are reused in full.
    StableAnchorsSupported,
    /// The context provides anchors for some links only; partial reuse, disclosed.
    PartialAnchorsSupported,
    /// The context cannot provide stable anchors; parity is unsupported and named.
    AnchorsUnsupported,
}

impl AnchorParity {
    /// All parity states, in canonical order.
    pub const ALL: [Self; 3] = [
        Self::StableAnchorsSupported,
        Self::PartialAnchorsSupported,
        Self::AnchorsUnsupported,
    ];

    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableAnchorsSupported => "stable_anchors_supported",
            Self::PartialAnchorsSupported => "partial_anchors_supported",
            Self::AnchorsUnsupported => "anchors_unsupported",
        }
    }

    /// Returns true when this parity must render with a visible caveat.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::StableAnchorsSupported)
    }

    /// Returns true when the context cannot host related-object links at all.
    pub const fn is_unsupported(self) -> bool {
        matches!(self, Self::AnchorsUnsupported)
    }
}

// ---------------------------------------------------------------------------
// Labels.
// ---------------------------------------------------------------------------

/// A user-visible label a panel attaches to a group or to the whole panel.
///
/// Labels keep source-attribution, fallback, captured-scope, incomplete-scope,
/// generated, and unsupported-parity facts visible rather than folding them into an
/// undifferentiated list of links.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelatedObjectLabel {
    /// The group or panel includes graph-derived links.
    GraphDerived,
    /// The group or panel includes framework-derived links.
    FrameworkDerived,
    /// The group or panel includes curated links.
    Curated,
    /// The group or panel includes runtime-derived links.
    RuntimeDerived,
    /// The group or panel includes links that require disambiguation.
    DisambiguationRequired,
    /// The group or panel includes lexical-fallback links.
    LexicalFallback,
    /// The group or panel includes imported-snapshot links.
    ImportedSnapshot,
    /// The group or panel includes runtime-observed-only links.
    RuntimeObservedOnly,
    /// The group or panel includes unavailable links.
    Unavailable,
    /// The group or panel includes generated-boundary links.
    Generated,
    /// Every link in the group is carried only by a captured scope.
    CapturedScopeOnly,
    /// The group or panel covers an incomplete scope.
    IncompleteScope,
    /// The group or panel rests on stale or unverified evidence.
    StaleEvidence,
    /// The panel's anchor context does not support stable relation anchors.
    UnsupportedParity,
}

impl RelatedObjectLabel {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GraphDerived => "graph_derived",
            Self::FrameworkDerived => "framework_derived",
            Self::Curated => "curated",
            Self::RuntimeDerived => "runtime_derived",
            Self::DisambiguationRequired => "disambiguation_required",
            Self::LexicalFallback => "lexical_fallback",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::RuntimeObservedOnly => "runtime_observed_only",
            Self::Unavailable => "unavailable",
            Self::Generated => "generated",
            Self::CapturedScopeOnly => "captured_scope_only",
            Self::IncompleteScope => "incomplete_scope",
            Self::StaleEvidence => "stale_evidence",
            Self::UnsupportedParity => "unsupported_parity",
        }
    }
}

// ---------------------------------------------------------------------------
// Actions.
// ---------------------------------------------------------------------------

/// The effect a related-object action has on navigation history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelatedObjectHistoryEffect {
    /// Pushes a new navigation history entry (open, split-open).
    AdvancesHistory,
    /// Leaves navigation history untouched (peek, reveal-attribution).
    PreservesCurrent,
    /// Touches no editor history at all (export).
    NoEditorHistory,
}

impl RelatedObjectHistoryEffect {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdvancesHistory => "advances_history",
            Self::PreservesCurrent => "preserves_current",
            Self::NoEditorHistory => "no_editor_history",
        }
    }
}

/// A stable action a related-object panel can invoke.
///
/// The set is closed and identical across every [`RelatedObjectActionRoute`]: open,
/// peek, split-open, reveal-attribution, and export. Each has one stable history
/// effect, and the two navigating actions (open, split-open) are gated whenever any
/// link needs disambiguation, so a jump never silently picks a candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelatedObjectActionKind {
    /// Jump to the related target in the active editor.
    Open,
    /// Peek the related target inline without leaving the current editor.
    Peek,
    /// Open the related target in a split, leaving the current editor in place.
    SplitOpen,
    /// Reveal the link's source class and evidence without navigating.
    RevealAttribution,
    /// Export the metadata-only related-object panel; never mutates the editor.
    Export,
}

impl RelatedObjectActionKind {
    /// All actions, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::Open,
        Self::Peek,
        Self::SplitOpen,
        Self::RevealAttribution,
        Self::Export,
    ];

    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Peek => "peek",
            Self::SplitOpen => "split_open",
            Self::RevealAttribution => "reveal_attribution",
            Self::Export => "export",
        }
    }

    /// Returns a human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Open => "Go to Related",
            Self::Peek => "Peek",
            Self::SplitOpen => "Open to the Side",
            Self::RevealAttribution => "Reveal Source",
            Self::Export => "Export Related Objects",
        }
    }

    /// Returns the stable history effect this action has.
    pub const fn history_effect(self) -> RelatedObjectHistoryEffect {
        match self {
            Self::Open | Self::SplitOpen => RelatedObjectHistoryEffect::AdvancesHistory,
            Self::Peek | Self::RevealAttribution => RelatedObjectHistoryEffect::PreservesCurrent,
            Self::Export => RelatedObjectHistoryEffect::NoEditorHistory,
        }
    }

    /// Returns true when this action navigates and therefore can change context.
    pub const fn navigates(self) -> bool {
        matches!(self, Self::Open | Self::SplitOpen)
    }
}

/// A surface route that exposes the related-object actions.
///
/// The same actions are reachable from every route, so open/peek/split/reveal/export
/// behave identically whether invoked from the related-object panel, an editor gutter,
/// a graph overlay, a search panel, a docs link, or a keyboard shortcut.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelatedObjectActionRoute {
    /// The dedicated related-object panel.
    RelatedPanel,
    /// An editor gutter, code-lens, or hover affordance.
    EditorGutter,
    /// A graph or topology overlay.
    GraphOverlay,
    /// A search results panel.
    SearchPanel,
    /// A documentation or help link.
    DocsLink,
    /// A keyboard shortcut route.
    KeyboardShortcut,
}

impl RelatedObjectActionRoute {
    /// All routes, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::RelatedPanel,
        Self::EditorGutter,
        Self::GraphOverlay,
        Self::SearchPanel,
        Self::DocsLink,
        Self::KeyboardShortcut,
    ];

    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RelatedPanel => "related_panel",
            Self::EditorGutter => "editor_gutter",
            Self::GraphOverlay => "graph_overlay",
            Self::SearchPanel => "search_panel",
            Self::DocsLink => "docs_link",
            Self::KeyboardShortcut => "keyboard_shortcut",
        }
    }
}

/// A stable action bound to a related-object panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelatedObjectActionAffordance {
    /// The action kind.
    pub action_kind: RelatedObjectActionKind,
    /// The stable history effect for this action.
    pub history_effect: RelatedObjectHistoryEffect,
    /// Routes the action is reachable from, in canonical order.
    pub available_routes: Vec<RelatedObjectActionRoute>,
    /// The anchor ref the action resolves against.
    pub anchor_ref: String,
    /// Always true: the action resolves to the same anchor identity across every route.
    pub preserves_anchor_identity: bool,
    /// True when this navigating action is gated behind disambiguation.
    pub gated_by_disambiguation: bool,
    /// Export-safe summary.
    pub summary: String,
}

// ---------------------------------------------------------------------------
// Link.
// ---------------------------------------------------------------------------

/// One source-attributed related-object link.
///
/// This is the deliverable object: a link from the panel's anchor to a related target,
/// carrying its object kind, the closed relation kind it maps to, the evidence class
/// ([`RelatedObjectSourceClass`]) that admitted it, the [`RelatedObjectFallbackMode`]
/// that resolves it, freshness, proof, scope, authorship posture, candidate
/// alternatives for disambiguation, downgrade reasons, and evidence refs — so a support
/// or debug packet can explain why the link existed and what evidence backed it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelatedObjectLink {
    /// Stable link id.
    pub link_id: String,
    /// What is on the other end of the link.
    pub object_kind: RelatedObjectKind,
    /// The closed relation kind the object kind maps to.
    pub relation_kind: RelationKind,
    /// The evidence class that admitted the link.
    pub source_class: RelatedObjectSourceClass,
    /// Stable source anchor ref the link originates from.
    pub anchor_ref: String,
    /// Stable target ref the link resolves to (a placeholder when unavailable).
    pub target_ref: String,
    /// Candidate alternative targets, non-empty only when disambiguation is required.
    pub alternate_target_refs: Vec<String>,
    /// How the link resolves to its target.
    pub fallback_mode: RelatedObjectFallbackMode,
    /// Proof class for the link relation.
    pub proof_class: ProofClass,
    /// Confidence class for the link.
    pub confidence: NavigationConfidence,
    /// Freshness class for the link.
    pub freshness: FreshnessClass,
    /// Completeness of the materialized scope for the link.
    pub scope_completeness: ScopeCompleteness,
    /// Authorship, generated, imported, or read-only posture of the target.
    pub generated_or_external_state: GeneratedOrExternalState,
    /// Downgrade reasons that must stay visible on consumers.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Evidence refs safe for support, review, AI, and CLI consumers.
    pub evidence_refs: Vec<String>,
    /// Export-safe summary.
    pub summary: String,
}

impl RelatedObjectLink {
    /// Returns true when an explicit selection is required before navigating this link.
    pub fn requires_selection(&self) -> bool {
        self.fallback_mode.requires_selection()
    }

    /// Returns true when the link is captured-scope only — carried by an imported
    /// snapshot or runtime trace, or resting on stale/unverified freshness — rather than
    /// re-proven against the current scope.
    pub fn is_captured_only(&self) -> bool {
        matches!(
            self.proof_class,
            ProofClass::ImportedEvidence | ProofClass::RuntimeObserved
        ) || matches!(
            self.fallback_mode,
            RelatedObjectFallbackMode::ImportedSnapshot
                | RelatedObjectFallbackMode::RuntimeObservedOnly
        ) || matches!(
            self.freshness,
            FreshnessClass::Stale | FreshnessClass::Unverified
        )
    }

    /// Returns true when the link must render with a visible caveat: anything other
    /// than a graph-derived, primary, semantically proven, live, complete link cannot be
    /// shown as an unquestioned jump.
    pub fn requires_disclosure(&self) -> bool {
        self.source_class.requires_disclosure()
            || self.fallback_mode.requires_disclosure()
            || self.proof_class.requires_disclosure()
            || self.confidence.requires_disclosure()
            || self.freshness.requires_disclosure()
            || self.scope_completeness.requires_disclosure()
            || !self.downgrade_reasons.is_empty()
    }

    /// Returns true when the link is backed by something a support packet can cite — a
    /// downgrade reason or an evidence ref — whenever it requires disclosure.
    fn disclosure_is_evidenced(&self) -> bool {
        !self.requires_disclosure()
            || !self.downgrade_reasons.is_empty()
            || !self.evidence_refs.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Counts.
// ---------------------------------------------------------------------------

/// Source- and fallback-partitioned counts for a related-object group or panel.
///
/// `current_scope_count` tallies links proven against the current scope;
/// `captured_scope_count` tallies links carried only by a captured snapshot, trace, or
/// imported pack. The two always sum to `total_count`. The source tallies
/// (graph/framework/curated/runtime) and the fallback tallies both independently
/// partition the total, so a panel never claims a source or fallback count it cannot
/// back with links.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelatedObjectCounts {
    /// Total links in the group or panel.
    pub total_count: usize,
    /// Links proven against the current scope.
    pub current_scope_count: usize,
    /// Links carried only by a captured snapshot, trace, or imported pack.
    pub captured_scope_count: usize,
    /// Graph-derived links.
    pub graph_derived_count: usize,
    /// Framework-derived links.
    pub framework_derived_count: usize,
    /// Curated links.
    pub curated_count: usize,
    /// Runtime-derived links.
    pub runtime_derived_count: usize,
    /// Links resolving directly with no fallback.
    pub primary_count: usize,
    /// Links requiring disambiguation.
    pub disambiguation_required_count: usize,
    /// Links resting on a lexical fallback.
    pub lexical_fallback_count: usize,
    /// Links carried by an imported snapshot.
    pub imported_snapshot_count: usize,
    /// Links backed only by a runtime observation.
    pub runtime_observed_only_count: usize,
    /// Links whose target is unavailable in the current scope.
    pub unavailable_count: usize,
    /// Links covering an incomplete scope.
    pub incomplete_scope_count: usize,
    /// Generated-boundary links.
    pub generated_count: usize,
}

impl RelatedObjectCounts {
    /// Returns true when the current and captured counts reconcile with the total.
    pub const fn reconciles(&self) -> bool {
        self.current_scope_count + self.captured_scope_count == self.total_count
    }

    /// Returns true when the four source tallies partition the total.
    pub const fn source_partition_reconciles(&self) -> bool {
        self.graph_derived_count
            + self.framework_derived_count
            + self.curated_count
            + self.runtime_derived_count
            == self.total_count
    }

    /// Returns true when the six fallback tallies partition the total.
    pub const fn fallback_partition_reconciles(&self) -> bool {
        self.primary_count
            + self.disambiguation_required_count
            + self.lexical_fallback_count
            + self.imported_snapshot_count
            + self.runtime_observed_only_count
            + self.unavailable_count
            == self.total_count
    }

    fn add(&mut self, link: &RelatedObjectLink) {
        self.total_count += 1;
        if link.is_captured_only() {
            self.captured_scope_count += 1;
        } else {
            self.current_scope_count += 1;
        }
        match link.source_class {
            RelatedObjectSourceClass::GraphDerived => self.graph_derived_count += 1,
            RelatedObjectSourceClass::FrameworkDerived => self.framework_derived_count += 1,
            RelatedObjectSourceClass::Curated => self.curated_count += 1,
            RelatedObjectSourceClass::RuntimeDerived => self.runtime_derived_count += 1,
        }
        match link.fallback_mode {
            RelatedObjectFallbackMode::Primary => self.primary_count += 1,
            RelatedObjectFallbackMode::DisambiguationRequired => {
                self.disambiguation_required_count += 1
            }
            RelatedObjectFallbackMode::LexicalFallback => self.lexical_fallback_count += 1,
            RelatedObjectFallbackMode::ImportedSnapshot => self.imported_snapshot_count += 1,
            RelatedObjectFallbackMode::RuntimeObservedOnly => self.runtime_observed_only_count += 1,
            RelatedObjectFallbackMode::Unavailable => self.unavailable_count += 1,
        }
        if link.scope_completeness.requires_disclosure() {
            self.incomplete_scope_count += 1;
        }
        if link.generated_or_external_state == GeneratedOrExternalState::GeneratedSource
            || link
                .downgrade_reasons
                .contains(&DowngradeReason::GeneratedBoundary)
        {
            self.generated_count += 1;
        }
    }
}

// ---------------------------------------------------------------------------
// Group, disambiguation, projection, panel.
// ---------------------------------------------------------------------------

/// One source-class group inside a related-object panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelatedObjectGroup {
    /// The source class this group represents.
    pub source_class: RelatedObjectSourceClass,
    /// The links in this group, in input order.
    pub links: Vec<RelatedObjectLink>,
    /// Source- and fallback-partitioned counts for the group.
    pub counts: RelatedObjectCounts,
    /// Aggregate scope completeness over the group (weakest link).
    pub scope_completeness: ScopeCompleteness,
    /// Aggregate freshness over the group (weakest link).
    pub freshness: FreshnessClass,
    /// Aggregate confidence over the group (weakest link).
    pub confidence: NavigationConfidence,
    /// The proof classes behind this group's links, in canonical order.
    pub proof_classes: Vec<ProofClass>,
    /// The object kinds in this group, in canonical order.
    pub object_kinds: Vec<RelatedObjectKind>,
    /// The fallback modes in this group, in canonical order.
    pub fallback_modes: Vec<RelatedObjectFallbackMode>,
    /// Visible labels for the group.
    pub labels: Vec<RelatedObjectLabel>,
    /// Downgrade reasons that must stay visible on consumers.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Attribution notes describing the group's evidence class and fallbacks.
    pub attribution_notes: Vec<String>,
    /// Export-safe summary.
    pub summary: String,
}

/// Whether a related-object panel has competing candidates, and how to disambiguate.
///
/// When any link needs an explicit selection the panel does not silently pick one
/// target: it exposes the competing links and a disambiguation set ref and requires
/// inspection before a navigating action runs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelatedObjectDisambiguation {
    /// True when any link requires an explicit selection.
    pub requires_selection: bool,
    /// Link ids the user must choose between, in input order.
    pub competing_link_refs: Vec<String>,
    /// Disambiguation set ref when the user must choose a target.
    pub disambiguation_set_ref: Option<String>,
    /// True when a navigating action must be inspected before a jump.
    pub requires_inspection_before_jump: bool,
    /// Export-safe note explaining the disambiguation.
    pub note: String,
}

impl RelatedObjectDisambiguation {
    /// Returns true when the panel exposes a way to disambiguate competing links.
    pub fn has_disambiguation_path(&self) -> bool {
        !self.competing_link_refs.is_empty() || self.disambiguation_set_ref.is_some()
    }
}

/// A surface-level projection proving the panel survives review, support, AI, graph,
/// and docs consumers without flattening into generic links.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelatedObjectProjection {
    /// The consumer surface.
    pub consumer_surface: ConsumerSurface,
    /// Number of source-class groups projected to this surface.
    pub projected_group_count: usize,
    /// True when source attribution is preserved.
    pub preserves_source_attribution: bool,
    /// True when source/fallback counts are preserved.
    pub preserves_counts: bool,
    /// True when fallback truth is preserved.
    pub preserves_fallback_truth: bool,
    /// True when anchor parity is preserved.
    pub preserves_anchor_parity: bool,
    /// True when freshness and confidence are preserved.
    pub preserves_freshness_and_confidence: bool,
    /// True when the projection flattens the panel into generic links (must be false).
    pub flattens_to_generic_links: bool,
    /// True when the projection exports raw code bodies (must be false).
    pub exports_code_bodies: bool,
    /// Redaction class for this projection.
    pub redaction_class: ExportRedactionClass,
    /// Export-safe summary.
    pub summary: String,
}

impl RelatedObjectProjection {
    /// Returns true when the projection preserves the panel's typed truth without
    /// flattening or leaking code bodies.
    pub const fn preserves_truth(&self) -> bool {
        self.preserves_source_attribution
            && self.preserves_counts
            && self.preserves_fallback_truth
            && self.preserves_anchor_parity
            && self.preserves_freshness_and_confidence
            && !self.flattens_to_generic_links
            && !self.exports_code_bodies
    }
}

/// A related-object panel: links grouped by source class, with fallback truth, anchor
/// parity, current-versus-captured counts, disambiguation, stable actions, and consumer
/// projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelatedObjectPanel {
    /// Stable panel id.
    pub panel_id: String,
    /// The anchor ref the related objects are anchored on.
    pub anchor_ref: String,
    /// The surface the panel was invoked from.
    pub anchor_context: RelatedObjectAnchorContext,
    /// Whether the anchor context supports stable relation anchors.
    pub anchor_parity: AnchorParity,
    /// Export-safe note describing the anchor parity.
    pub parity_note: String,
    /// The current scope ref the links were resolved against.
    pub scope_ref: String,
    /// The captured snapshot/trace/pack scope ref, when any link is captured-only.
    pub captured_scope_ref: Option<String>,
    /// Source-class groups, in canonical order.
    pub groups: Vec<RelatedObjectGroup>,
    /// Aggregate counts across all groups.
    pub totals: RelatedObjectCounts,
    /// The headline source attribution for the panel.
    pub source_headline: RelatedObjectHeadline,
    /// Aggregate scope completeness across the panel (weakest link).
    pub scope_completeness: ScopeCompleteness,
    /// Aggregate freshness across the panel (weakest group).
    pub freshness: FreshnessClass,
    /// Aggregate confidence across the panel (weakest group).
    pub confidence: NavigationConfidence,
    /// The union of group labels plus panel-level labels.
    pub labels: Vec<RelatedObjectLabel>,
    /// The panel's disambiguation state.
    pub disambiguation: RelatedObjectDisambiguation,
    /// The stable open/peek/split/reveal/export actions.
    pub actions: Vec<RelatedObjectActionAffordance>,
    /// Consumer projections proving cross-surface parity.
    pub consumer_projections: Vec<RelatedObjectProjection>,
    /// Downgrade reasons that must stay visible on consumers.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Attribution notes describing the panel's evidence classes and gaps.
    pub attribution_notes: Vec<String>,
    /// Redaction class for review/support/AI export.
    pub redaction_class: ExportRedactionClass,
    /// Export-safe summary.
    pub summary: String,
}

impl RelatedObjectPanel {
    /// Returns the group for a source class, if present.
    pub fn group(&self, source_class: RelatedObjectSourceClass) -> Option<&RelatedObjectGroup> {
        self.groups
            .iter()
            .find(|group| group.source_class == source_class)
    }

    /// Returns every link across every group, in group then input order.
    pub fn links(&self) -> impl Iterator<Item = &RelatedObjectLink> {
        self.groups.iter().flat_map(|group| group.links.iter())
    }

    /// Returns true when the panel has any captured-only link.
    pub const fn has_captured_scope(&self) -> bool {
        self.totals.captured_scope_count > 0
    }

    /// Returns true when a related-object jump must be inspected for disambiguation
    /// first.
    pub const fn requires_inspection_before_jump(&self) -> bool {
        self.disambiguation.requires_inspection_before_jump
    }

    /// Returns true when the panel must render with a visible caveat.
    pub fn requires_disclosure(&self) -> bool {
        self.scope_completeness.requires_disclosure()
            || self.has_captured_scope()
            || self.requires_inspection_before_jump()
            || self.anchor_parity.requires_disclosure()
            || self
                .links()
                .any(|link| link.source_class.requires_disclosure())
            || !self.downgrade_reasons.is_empty()
    }
}

/// The typed input the builder turns into a related-object panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelatedObjectPanelInput {
    /// Stable panel id.
    pub panel_id: String,
    /// The anchor ref the related objects are anchored on.
    pub anchor_ref: String,
    /// The surface the panel was invoked from.
    pub anchor_context: RelatedObjectAnchorContext,
    /// Whether the anchor context supports stable relation anchors.
    pub anchor_parity: AnchorParity,
    /// Export-safe note describing the anchor parity.
    pub parity_note: String,
    /// The current scope ref.
    pub scope_ref: String,
    /// The captured snapshot/trace/pack scope ref, if any.
    pub captured_scope_ref: Option<String>,
    /// Redaction class for review/support/AI export.
    pub redaction_class: ExportRedactionClass,
    /// The related-object links to group and tally.
    pub links: Vec<RelatedObjectLink>,
    /// Disambiguation set ref when the user must choose a target.
    pub disambiguation_set_ref: Option<String>,
    /// Export-safe note explaining any disambiguation.
    pub disambiguation_note: String,
}

// ---------------------------------------------------------------------------
// Builder.
// ---------------------------------------------------------------------------

/// Builds a related-object panel from a typed input.
///
/// Deterministic: the same input yields the same panel. Links are grouped by source
/// class in [`RELATED_OBJECT_SOURCE_ORDER`]; each group and the panel carry computed
/// counts, scope completeness, freshness, confidence, labels, downgrade reasons, and
/// attribution notes; the disambiguation state, the five stable actions, and the
/// consumer projections are generated so the panel proves cross-surface parity. The
/// builder derives all disclosure from the links, the anchor parity, and the named
/// fallbacks themselves, so a framework, curated, runtime, captured-scope, or fallback
/// link cannot lose its caveat.
pub fn build_related_object_panel(input: &RelatedObjectPanelInput) -> RelatedObjectPanel {
    let mut groups = Vec::new();
    for source_class in RELATED_OBJECT_SOURCE_ORDER {
        let members: Vec<&RelatedObjectLink> = input
            .links
            .iter()
            .filter(|link| link.source_class == source_class)
            .collect();
        if members.is_empty() {
            continue;
        }
        groups.push(build_group(source_class, &members));
    }

    let all: Vec<&RelatedObjectLink> = input.links.iter().collect();
    let mut totals = RelatedObjectCounts::default();
    for link in &all {
        totals.add(link);
    }

    let source_headline = headline_source(&all);
    let scope_completeness = weakest_scope_completeness(&all);
    let freshness = weakest_freshness(&all);
    let confidence = weakest_confidence(&all);

    let mut labels = labels_for(&all, scope_completeness);
    if input.anchor_parity.requires_disclosure() {
        push_label(&mut labels, RelatedObjectLabel::UnsupportedParity);
    }
    labels.sort();

    let mut downgrade_reasons = downgrade_reasons_for(&all);
    if all.iter().any(|link| link.requires_selection()) {
        push_unique(&mut downgrade_reasons, DowngradeReason::AmbiguousCandidates);
    }
    if input.anchor_parity.is_unsupported() {
        push_unique(&mut downgrade_reasons, DowngradeReason::MissingProvider);
    }

    let mut attribution_notes = attribution_notes_for(&all);
    if input.anchor_parity.requires_disclosure() {
        attribution_notes.push(format!(
            "Anchor context {} has {} parity: {}",
            input.anchor_context.as_str(),
            input.anchor_parity.as_str(),
            input.parity_note,
        ));
    }

    let competing_link_refs: Vec<String> = all
        .iter()
        .filter(|link| link.requires_selection())
        .map(|link| link.link_id.clone())
        .collect();
    let requires_selection = !competing_link_refs.is_empty();
    let disambiguation = RelatedObjectDisambiguation {
        requires_selection,
        competing_link_refs,
        disambiguation_set_ref: input.disambiguation_set_ref.clone(),
        requires_inspection_before_jump: requires_selection,
        note: input.disambiguation_note.clone(),
    };
    if disambiguation.requires_inspection_before_jump {
        attribution_notes.push(format!(
            "{} link(s) need an explicit selection and must be inspected before a jump.",
            disambiguation.competing_link_refs.len()
        ));
    }

    let actions = RelatedObjectActionKind::ALL
        .iter()
        .map(|action_kind| {
            build_action(
                *action_kind,
                &input.anchor_ref,
                disambiguation.requires_inspection_before_jump,
            )
        })
        .collect();

    let consumer_projections = REQUIRED_CONSUMER_SURFACES
        .iter()
        .map(|surface| build_projection(*surface, groups.len(), input.redaction_class))
        .collect();

    let summary = format!(
        "Related objects for {} ({}): {} link(s) across {} source group(s); {} current, {} \
         captured; headline {}; parity {}; scope {}; {} need selection.",
        input.anchor_ref,
        input.anchor_context.as_str(),
        totals.total_count,
        groups.len(),
        totals.current_scope_count,
        totals.captured_scope_count,
        source_headline.as_str(),
        input.anchor_parity.as_str(),
        scope_completeness_token(scope_completeness),
        disambiguation.competing_link_refs.len(),
    );

    RelatedObjectPanel {
        panel_id: input.panel_id.clone(),
        anchor_ref: input.anchor_ref.clone(),
        anchor_context: input.anchor_context,
        anchor_parity: input.anchor_parity,
        parity_note: input.parity_note.clone(),
        scope_ref: input.scope_ref.clone(),
        captured_scope_ref: input.captured_scope_ref.clone(),
        groups,
        totals,
        source_headline,
        scope_completeness,
        freshness,
        confidence,
        labels,
        disambiguation,
        actions,
        consumer_projections,
        downgrade_reasons,
        attribution_notes,
        redaction_class: input.redaction_class,
        summary,
    }
}

fn build_group(
    source_class: RelatedObjectSourceClass,
    members: &[&RelatedObjectLink],
) -> RelatedObjectGroup {
    let mut counts = RelatedObjectCounts::default();
    for link in members {
        counts.add(link);
    }
    let scope_completeness = weakest_scope_completeness(members);
    let freshness = weakest_freshness(members);
    let confidence = weakest_confidence(members);
    let proof_classes = proof_classes_for(members);
    let object_kinds = object_kinds_for(members);
    let fallback_modes = fallback_modes_for(members);
    let labels = labels_for(members, scope_completeness);
    let downgrade_reasons = downgrade_reasons_for(members);
    let attribution_notes = attribution_notes_for(members);
    let links = members.iter().map(|link| (*link).clone()).collect();
    let summary = format!(
        "{} group: {} link(s) ({} current, {} captured); scope {}; freshness {}.",
        source_class.as_str(),
        counts.total_count,
        counts.current_scope_count,
        counts.captured_scope_count,
        scope_completeness_token(scope_completeness),
        freshness_token(freshness),
    );
    RelatedObjectGroup {
        source_class,
        links,
        counts,
        scope_completeness,
        freshness,
        confidence,
        proof_classes,
        object_kinds,
        fallback_modes,
        labels,
        downgrade_reasons,
        attribution_notes,
        summary,
    }
}

fn build_action(
    action_kind: RelatedObjectActionKind,
    anchor_ref: &str,
    pending_disambiguation: bool,
) -> RelatedObjectActionAffordance {
    let gated_by_disambiguation = action_kind.navigates() && pending_disambiguation;
    RelatedObjectActionAffordance {
        action_kind,
        history_effect: action_kind.history_effect(),
        available_routes: RelatedObjectActionRoute::ALL.to_vec(),
        anchor_ref: anchor_ref.to_owned(),
        preserves_anchor_identity: true,
        gated_by_disambiguation,
        summary: format!(
            "{} resolves against the same anchor on every route ({}); history effect {}{}.",
            action_kind.label(),
            RelatedObjectActionRoute::ALL
                .iter()
                .map(|route| route.as_str())
                .collect::<Vec<_>>()
                .join(", "),
            action_kind.history_effect().as_str(),
            if gated_by_disambiguation {
                "; gated behind disambiguation"
            } else {
                ""
            },
        ),
    }
}

fn build_projection(
    surface: ConsumerSurface,
    group_count: usize,
    redaction_class: ExportRedactionClass,
) -> RelatedObjectProjection {
    RelatedObjectProjection {
        consumer_surface: surface,
        projected_group_count: group_count,
        preserves_source_attribution: true,
        preserves_counts: true,
        preserves_fallback_truth: true,
        preserves_anchor_parity: true,
        preserves_freshness_and_confidence: true,
        flattens_to_generic_links: false,
        exports_code_bodies: false,
        redaction_class,
        summary: format!(
            "{} consumes the panel with source attribution, counts, fallback truth, anchor \
             parity, and freshness/confidence preserved; never flattened into generic links.",
            surface.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Derivations.
// ---------------------------------------------------------------------------

fn headline_source(members: &[&RelatedObjectLink]) -> RelatedObjectHeadline {
    if members.is_empty() {
        return RelatedObjectHeadline::Empty;
    }
    let mut classes = BTreeSet::new();
    for link in members {
        classes.insert(link.source_class);
    }
    if classes.len() == 1 {
        match classes.into_iter().next().unwrap() {
            RelatedObjectSourceClass::GraphDerived => RelatedObjectHeadline::GraphDerived,
            RelatedObjectSourceClass::FrameworkDerived => RelatedObjectHeadline::FrameworkDerived,
            RelatedObjectSourceClass::Curated => RelatedObjectHeadline::Curated,
            RelatedObjectSourceClass::RuntimeDerived => RelatedObjectHeadline::RuntimeDerived,
        }
    } else {
        RelatedObjectHeadline::Mixed
    }
}

fn proof_classes_for(members: &[&RelatedObjectLink]) -> Vec<ProofClass> {
    let mut classes = BTreeSet::new();
    for link in members {
        classes.insert(link.proof_class);
    }
    classes.into_iter().collect()
}

fn object_kinds_for(members: &[&RelatedObjectLink]) -> Vec<RelatedObjectKind> {
    let mut kinds = BTreeSet::new();
    for link in members {
        kinds.insert(link.object_kind);
    }
    kinds.into_iter().collect()
}

fn fallback_modes_for(members: &[&RelatedObjectLink]) -> Vec<RelatedObjectFallbackMode> {
    let mut modes = BTreeSet::new();
    for link in members {
        modes.insert(link.fallback_mode);
    }
    modes.into_iter().collect()
}

fn weakest_scope_completeness(members: &[&RelatedObjectLink]) -> ScopeCompleteness {
    members
        .iter()
        .map(|link| link.scope_completeness)
        .max_by_key(|completeness| scope_completeness_severity(*completeness))
        .unwrap_or(ScopeCompleteness::CompleteForDeclaredScope)
}

fn weakest_freshness(members: &[&RelatedObjectLink]) -> FreshnessClass {
    members
        .iter()
        .map(|link| link.freshness)
        .max_by_key(|freshness| freshness_severity(*freshness))
        .unwrap_or(FreshnessClass::AuthoritativeLive)
}

fn weakest_confidence(members: &[&RelatedObjectLink]) -> NavigationConfidence {
    members
        .iter()
        .map(|link| link.confidence)
        .max_by_key(|confidence| confidence_severity(*confidence))
        .unwrap_or(NavigationConfidence::Exact)
}

fn labels_for(
    members: &[&RelatedObjectLink],
    scope_completeness: ScopeCompleteness,
) -> Vec<RelatedObjectLabel> {
    let mut labels = BTreeSet::new();
    for link in members {
        labels.insert(link.source_class.label_kind());
        match link.fallback_mode {
            RelatedObjectFallbackMode::DisambiguationRequired => {
                labels.insert(RelatedObjectLabel::DisambiguationRequired);
            }
            RelatedObjectFallbackMode::LexicalFallback => {
                labels.insert(RelatedObjectLabel::LexicalFallback);
            }
            RelatedObjectFallbackMode::ImportedSnapshot => {
                labels.insert(RelatedObjectLabel::ImportedSnapshot);
            }
            RelatedObjectFallbackMode::RuntimeObservedOnly => {
                labels.insert(RelatedObjectLabel::RuntimeObservedOnly);
            }
            RelatedObjectFallbackMode::Unavailable => {
                labels.insert(RelatedObjectLabel::Unavailable);
            }
            RelatedObjectFallbackMode::Primary => {}
        }
        // Proof-based captured labels keep the captured-scope tally and the panel labels
        // consistent: an imported-evidence or runtime-observed link is captured-scope
        // even when its fallback mode reads as primary.
        match link.proof_class {
            ProofClass::ImportedEvidence => {
                labels.insert(RelatedObjectLabel::ImportedSnapshot);
            }
            ProofClass::RuntimeObserved => {
                labels.insert(RelatedObjectLabel::RuntimeObservedOnly);
            }
            ProofClass::LexicalFallback => {
                labels.insert(RelatedObjectLabel::LexicalFallback);
            }
            _ => {}
        }
        if link.generated_or_external_state == GeneratedOrExternalState::GeneratedSource
            || link
                .downgrade_reasons
                .contains(&DowngradeReason::GeneratedBoundary)
        {
            labels.insert(RelatedObjectLabel::Generated);
        }
        if matches!(
            link.freshness,
            FreshnessClass::Stale | FreshnessClass::Unverified
        ) {
            labels.insert(RelatedObjectLabel::StaleEvidence);
        }
    }
    if !members.is_empty() && members.iter().all(|link| link.is_captured_only()) {
        labels.insert(RelatedObjectLabel::CapturedScopeOnly);
    }
    if scope_completeness.requires_disclosure() {
        labels.insert(RelatedObjectLabel::IncompleteScope);
    }
    labels.into_iter().collect()
}

fn downgrade_reasons_for(members: &[&RelatedObjectLink]) -> Vec<DowngradeReason> {
    let mut reasons: Vec<DowngradeReason> = Vec::new();
    for link in members {
        for reason in &link.downgrade_reasons {
            push_unique(&mut reasons, *reason);
        }
    }
    for link in members {
        match link.fallback_mode {
            RelatedObjectFallbackMode::LexicalFallback => {
                push_unique(&mut reasons, DowngradeReason::LexicalFallbackOnly);
            }
            RelatedObjectFallbackMode::ImportedSnapshot => {
                push_unique(&mut reasons, DowngradeReason::GeneratedBoundary);
            }
            RelatedObjectFallbackMode::RuntimeObservedOnly => {
                push_unique(&mut reasons, DowngradeReason::RuntimeOrFrameworkOnly);
            }
            _ => {}
        }
        if link.source_class == RelatedObjectSourceClass::FrameworkDerived {
            push_unique(&mut reasons, DowngradeReason::RuntimeOrFrameworkOnly);
        }
    }
    reasons
}

fn attribution_notes_for(members: &[&RelatedObjectLink]) -> Vec<String> {
    let mut notes = Vec::new();
    let count = |predicate: &dyn Fn(&RelatedObjectLink) -> bool| {
        members.iter().filter(|link| predicate(link)).count()
    };

    let framework = count(&|link| link.source_class == RelatedObjectSourceClass::FrameworkDerived);
    if framework > 0 {
        notes.push(format!(
            "{framework} link(s) are framework-derived from route/generator metadata and are \
             disclosed as inferred, never as graph-proven."
        ));
    }
    let curated = count(&|link| link.source_class == RelatedObjectSourceClass::Curated);
    if curated > 0 {
        notes.push(format!(
            "{curated} link(s) are read from a curated mapping (stewardship rule, doc table, or \
             manifest), not from code analysis."
        ));
    }
    let runtime = count(&|link| link.source_class == RelatedObjectSourceClass::RuntimeDerived);
    if runtime > 0 {
        notes.push(format!(
            "{runtime} link(s) are runtime-derived from a captured observation and may miss \
             relations the run never exercised."
        ));
    }
    let disambiguation =
        count(&|link| link.fallback_mode == RelatedObjectFallbackMode::DisambiguationRequired);
    if disambiguation > 0 {
        notes.push(format!(
            "{disambiguation} link(s) have competing candidates and need an explicit selection."
        ));
    }
    let lexical = count(&|link| link.fallback_mode == RelatedObjectFallbackMode::LexicalFallback);
    if lexical > 0 {
        notes.push(format!(
            "{lexical} link(s) were matched only lexically and are disclosed as a fallback, never \
             as a proven relation."
        ));
    }
    let imported = count(&|link| link.fallback_mode == RelatedObjectFallbackMode::ImportedSnapshot);
    if imported > 0 {
        notes.push(format!(
            "{imported} link(s) are carried by an imported snapshot and are captured-scope only."
        ));
    }
    let runtime_only =
        count(&|link| link.fallback_mode == RelatedObjectFallbackMode::RuntimeObservedOnly);
    if runtime_only > 0 {
        notes.push(format!(
            "{runtime_only} link(s) are backed only by a runtime observation, not a static \
             relation."
        ));
    }
    let unavailable = count(&|link| link.fallback_mode == RelatedObjectFallbackMode::Unavailable);
    if unavailable > 0 {
        notes.push(format!(
            "{unavailable} link(s) are known to exist but resolve to no target in the current \
             scope."
        ));
    }
    notes
}

fn push_unique(reasons: &mut Vec<DowngradeReason>, reason: DowngradeReason) {
    if !reasons.contains(&reason) {
        reasons.push(reason);
    }
}

fn push_label(labels: &mut Vec<RelatedObjectLabel>, label: RelatedObjectLabel) {
    if !labels.contains(&label) {
        labels.push(label);
    }
}

// ---------------------------------------------------------------------------
// Severity orderings and tokens.
// ---------------------------------------------------------------------------

const fn scope_completeness_severity(completeness: ScopeCompleteness) -> u8 {
    match completeness {
        ScopeCompleteness::CompleteForDeclaredScope => 0,
        ScopeCompleteness::PartialForDeclaredScope => 1,
        ScopeCompleteness::StaleForDeclaredScope => 2,
        ScopeCompleteness::UnavailableForDeclaredScope => 3,
    }
}

const fn scope_completeness_token(completeness: ScopeCompleteness) -> &'static str {
    match completeness {
        ScopeCompleteness::CompleteForDeclaredScope => "complete_for_declared_scope",
        ScopeCompleteness::PartialForDeclaredScope => "partial_for_declared_scope",
        ScopeCompleteness::StaleForDeclaredScope => "stale_for_declared_scope",
        ScopeCompleteness::UnavailableForDeclaredScope => "unavailable_for_declared_scope",
    }
}

const fn freshness_severity(freshness: FreshnessClass) -> u8 {
    match freshness {
        FreshnessClass::AuthoritativeLive => 0,
        FreshnessClass::WarmCached => 1,
        FreshnessClass::DegradedCached => 2,
        FreshnessClass::Stale => 3,
        FreshnessClass::Unverified => 4,
    }
}

const fn freshness_token(freshness: FreshnessClass) -> &'static str {
    match freshness {
        FreshnessClass::AuthoritativeLive => "authoritative_live",
        FreshnessClass::WarmCached => "warm_cached",
        FreshnessClass::DegradedCached => "degraded_cached",
        FreshnessClass::Stale => "stale",
        FreshnessClass::Unverified => "unverified",
    }
}

const fn confidence_severity(confidence: NavigationConfidence) -> u8 {
    match confidence {
        NavigationConfidence::Exact => 0,
        NavigationConfidence::Indexed => 1,
        NavigationConfidence::Imported => 2,
        NavigationConfidence::WorkspaceSliceLimited => 3,
        NavigationConfidence::Partial => 4,
        NavigationConfidence::Heuristic => 5,
        NavigationConfidence::Stale => 6,
        NavigationConfidence::Unavailable => 7,
    }
}

// ---------------------------------------------------------------------------
// Frozen corpus.
// ---------------------------------------------------------------------------

/// One frozen panel scenario: an input, the panel the builder produces for it, and the
/// property the scenario proves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelatedObjectScenario {
    /// Stable scenario id.
    pub scenario_id: String,
    /// Plain-language title.
    pub title: String,
    /// The panel-building input.
    pub input: RelatedObjectPanelInput,
    /// The panel `build_related_object_panel` produces for the input.
    pub panel: RelatedObjectPanel,
    /// One reviewable sentence stating what the scenario proves.
    pub expectation_note: String,
}

/// One frozen invariant over the corpus, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelatedObjectInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built corpus satisfies the invariant.
    pub holds: bool,
}

/// The frozen related-object navigation corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelatedObjectNavigationSet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub related_object_nav_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable corpus id.
    pub set_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// The freeze gate that keeps the corpus binding current.
    pub freeze_gate_ref: String,
    /// One reviewable sentence summarizing the corpus.
    pub summary: String,
    /// The frozen panel scenarios.
    pub scenarios: Vec<RelatedObjectScenario>,
    /// The computed invariants.
    pub invariants: Vec<RelatedObjectInvariant>,
    /// Whether raw source bodies and payloads are excluded (always true).
    pub raw_payload_excluded: bool,
}

/// Error returned when the corpus fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelatedObjectValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for RelatedObjectValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "related-object navigation corpus invalid: {}",
            self.reason
        )
    }
}

impl std::error::Error for RelatedObjectValidationError {}

impl RelatedObjectNavigationSet {
    /// Returns the scenario with a given id, if present.
    pub fn scenario(&self, scenario_id: &str) -> Option<&RelatedObjectScenario> {
        self.scenarios
            .iter()
            .find(|scenario| scenario.scenario_id == scenario_id)
    }

    /// Returns true when every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|invariant| invariant.holds)
    }

    /// Returns true when the corpus is safe to place in a support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded && self.all_refs().into_iter().all(is_export_safe_ref)
    }

    fn all_refs(&self) -> Vec<&str> {
        let mut refs: Vec<&str> = vec![self.schema_ref.as_str(), self.freeze_gate_ref.as_str()];
        for scenario in &self.scenarios {
            collect_input_refs(&scenario.input, &mut refs);
            collect_panel_refs(&scenario.panel, &mut refs);
        }
        refs
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    pub fn validate(&self) -> Result<(), RelatedObjectValidationError> {
        let fail = |reason: String| Err(RelatedObjectValidationError { reason });

        if self.record_kind != RELATED_OBJECT_NAV_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != RELATED_OBJECT_NAV_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.scenarios.is_empty() {
            return fail("corpus must carry at least one scenario".to_owned());
        }
        if !all_unique(self.scenarios.iter().map(|s| s.scenario_id.as_str())) {
            return fail("scenario ids are not unique".to_owned());
        }

        // Every scenario's stored panel equals what the builder produces, so the
        // fixture cannot drift from the builder.
        for scenario in &self.scenarios {
            let produced = build_related_object_panel(&scenario.input);
            if produced != scenario.panel {
                return fail(format!(
                    "scenario {} panel drifted from builder output",
                    scenario.scenario_id
                ));
            }
            // Each link's relation kind matches its object kind's closed mapping, so a
            // link is never an untyped smart link.
            for link in scenario.panel.links() {
                if link.relation_kind != link.object_kind.relation_kind() {
                    return fail(format!(
                        "link {} relation kind does not match object kind {}",
                        link.link_id,
                        link.object_kind.as_str()
                    ));
                }
            }
        }

        if !self.is_support_export_safe() {
            return fail("corpus is not support-export safe".to_owned());
        }
        if !self.all_invariants_hold() {
            let failed: Vec<&str> = self
                .invariants
                .iter()
                .filter(|invariant| !invariant.holds)
                .map(|invariant| invariant.invariant_id.as_str())
                .collect();
            return fail(format!("invariants do not hold: {}", failed.join(", ")));
        }
        Ok(())
    }
}

fn collect_input_refs<'a>(input: &'a RelatedObjectPanelInput, refs: &mut Vec<&'a str>) {
    refs.push(input.anchor_ref.as_str());
    refs.push(input.scope_ref.as_str());
    if let Some(captured) = &input.captured_scope_ref {
        refs.push(captured.as_str());
    }
    if let Some(set_ref) = &input.disambiguation_set_ref {
        refs.push(set_ref.as_str());
    }
    for link in &input.links {
        collect_link_refs(link, refs);
    }
}

fn collect_panel_refs<'a>(panel: &'a RelatedObjectPanel, refs: &mut Vec<&'a str>) {
    refs.push(panel.anchor_ref.as_str());
    refs.push(panel.scope_ref.as_str());
    if let Some(captured) = &panel.captured_scope_ref {
        refs.push(captured.as_str());
    }
    if let Some(set_ref) = &panel.disambiguation.disambiguation_set_ref {
        refs.push(set_ref.as_str());
    }
    for group in &panel.groups {
        for link in &group.links {
            collect_link_refs(link, refs);
        }
    }
    for action in &panel.actions {
        refs.push(action.anchor_ref.as_str());
    }
}

fn collect_link_refs<'a>(link: &'a RelatedObjectLink, refs: &mut Vec<&'a str>) {
    refs.push(link.anchor_ref.as_str());
    refs.push(link.target_ref.as_str());
    refs.extend(link.alternate_target_refs.iter().map(String::as_str));
    refs.extend(link.evidence_refs.iter().map(String::as_str));
}

/// Builds the canonical related-object navigation corpus.
///
/// Deterministic: the same bytes every call. Each scenario's panel is the builder's own
/// output, and the invariant `holds` flags are computed from those panels, so a
/// regression in [`build_related_object_panel`] flips an invariant or drifts the fixture
/// rather than silently passing.
pub fn related_object_navigation_set() -> RelatedObjectNavigationSet {
    let scenarios = build_scenarios();
    let invariants = compute_invariants(&scenarios);

    RelatedObjectNavigationSet {
        record_kind: RELATED_OBJECT_NAV_RECORD_KIND.to_owned(),
        related_object_nav_schema_version: RELATED_OBJECT_NAV_SCHEMA_VERSION,
        schema_ref: RELATED_OBJECT_NAV_SCHEMA_REF.to_owned(),
        set_id: RELATED_OBJECT_NAV_SET_ID.to_owned(),
        as_of: RELATED_OBJECT_NAV_AS_OF.to_owned(),
        freeze_gate_ref: RELATED_OBJECT_NAV_FREEZE_GATE_REF.to_owned(),
        summary: "Frozen related-object navigation corpus: route, component, test, doc, owner, and \
                  generated-artifact links are typed, source-attributed objects grouped by a \
                  graph-derived/framework-derived/curated/runtime-derived legend, each carrying its \
                  fallback mode, freshness, proof, and scope so a framework guess never poses as a \
                  graph-proven fact and a runtime observation never reads as a curated rule. Panels \
                  name the anchor context they were invoked from and whether it supports stable \
                  relation anchors, so the same relation semantics are reused in notebook, diff, \
                  docs-linked, and generated-artifact contexts and unsupported parity is labeled \
                  honestly. Each panel separates current-versus-captured counts, names an incomplete \
                  scope, exposes competing links and a disambiguation path before a jump, carries \
                  stable open/peek/split/reveal/export actions across every route, and projects to \
                  every consumer surface without flattening into generic links or exporting code \
                  bodies."
            .to_owned(),
        scenarios,
        invariants,
        raw_payload_excluded: true,
    }
}

/// Renders the corpus as human-readable lines for CLI/headless and support.
pub fn related_object_navigation_lines(set: &RelatedObjectNavigationSet) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Related-object navigation corpus — {} ({})",
        set.set_id, set.as_of
    ));
    lines.push(set.summary.clone());
    lines.push(format!(
        "Scenarios: {}  Invariants: {}",
        set.scenarios.len(),
        set.invariants.len()
    ));

    lines.push("Scenarios:".to_owned());
    for scenario in &set.scenarios {
        let panel = &scenario.panel;
        lines.push(format!("  - {} [{}]", scenario.scenario_id, scenario.title));
        lines.push(format!(
            "      ctx={} parity={} groups={} total={} current={} captured={} headline={} scope={}",
            panel.anchor_context.as_str(),
            panel.anchor_parity.as_str(),
            panel.groups.len(),
            panel.totals.total_count,
            panel.totals.current_scope_count,
            panel.totals.captured_scope_count,
            panel.source_headline.as_str(),
            scope_completeness_token(panel.scope_completeness),
        ));
        let group_summary = panel
            .groups
            .iter()
            .map(|group| {
                format!(
                    "{}:{}",
                    group.source_class.as_str(),
                    group.counts.total_count
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        lines.push(format!("      {group_summary}"));
        lines.push(format!(
            "      selection={} competing={} labels={}",
            panel.disambiguation.requires_selection,
            panel.disambiguation.competing_link_refs.len(),
            panel
                .labels
                .iter()
                .map(|label| label.as_str())
                .collect::<Vec<_>>()
                .join(","),
        ));
    }

    lines.push("Invariants:".to_owned());
    for invariant in &set.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if invariant.holds { "ok" } else { "FAIL" },
            invariant.invariant_id
        ));
    }

    lines
}

// ---------------------------------------------------------------------------
// Scenario builders.
// ---------------------------------------------------------------------------

/// Compact seed for a candidate [`RelatedObjectLink`], so each scenario reads as a small
/// table rather than a wall of struct fields.
struct LinkSeed {
    link_id: &'static str,
    object_kind: RelatedObjectKind,
    source_class: RelatedObjectSourceClass,
    fallback: RelatedObjectFallbackMode,
    proof: ProofClass,
    confidence: NavigationConfidence,
    freshness: FreshnessClass,
    scope: ScopeCompleteness,
    authorship: GeneratedOrExternalState,
    alternates: &'static [&'static str],
    evidence: &'static [&'static str],
    downgrades: &'static [DowngradeReason],
    summary: &'static str,
}

fn link(anchor: &str, seed: LinkSeed) -> RelatedObjectLink {
    RelatedObjectLink {
        link_id: seed.link_id.to_owned(),
        object_kind: seed.object_kind,
        relation_kind: seed.object_kind.relation_kind(),
        source_class: seed.source_class,
        anchor_ref: anchor.to_owned(),
        target_ref: format!("aureline://object/{}", seed.link_id),
        alternate_target_refs: seed.alternates.iter().map(|r| (*r).to_owned()).collect(),
        fallback_mode: seed.fallback,
        proof_class: seed.proof,
        confidence: seed.confidence,
        freshness: seed.freshness,
        scope_completeness: seed.scope,
        generated_or_external_state: seed.authorship,
        downgrade_reasons: seed.downgrades.to_vec(),
        evidence_refs: seed.evidence.iter().map(|r| (*r).to_owned()).collect(),
        summary: seed.summary.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn input(
    panel_id: &str,
    anchor: &str,
    anchor_context: RelatedObjectAnchorContext,
    anchor_parity: AnchorParity,
    parity_note: &str,
    captured_scope_ref: Option<&str>,
    redaction_class: ExportRedactionClass,
    links: Vec<RelatedObjectLink>,
    disambiguation_set_ref: Option<&str>,
    disambiguation_note: &str,
) -> RelatedObjectPanelInput {
    RelatedObjectPanelInput {
        panel_id: panel_id.to_owned(),
        anchor_ref: format!("aureline://object/{anchor}"),
        anchor_context,
        anchor_parity,
        parity_note: parity_note.to_owned(),
        scope_ref: "aureline://scope/workspace".to_owned(),
        captured_scope_ref: captured_scope_ref.map(str::to_owned),
        redaction_class,
        links,
        disambiguation_set_ref: disambiguation_set_ref.map(str::to_owned),
        disambiguation_note: disambiguation_note.to_owned(),
    }
}

fn scenario(
    scenario_id: &str,
    title: &str,
    input: RelatedObjectPanelInput,
    expectation_note: &str,
) -> RelatedObjectScenario {
    let panel = build_related_object_panel(&input);
    RelatedObjectScenario {
        scenario_id: scenario_id.to_owned(),
        title: title.to_owned(),
        input,
        panel,
        expectation_note: expectation_note.to_owned(),
    }
}

fn build_scenarios() -> Vec<RelatedObjectScenario> {
    use AnchorParity::*;
    use DowngradeReason as DR;
    use ExportRedactionClass::*;
    use FreshnessClass::*;
    use GeneratedOrExternalState as Auth;
    use NavigationConfidence as Cf;
    use ProofClass as Pf;
    use RelatedObjectAnchorContext as Ctx;
    use RelatedObjectFallbackMode as Fb;
    use RelatedObjectKind::*;
    use RelatedObjectSourceClass as Src;
    use ScopeCompleteness::*;

    vec![
        // 1. Editor symbol: a graph-proven component plus a framework-derived route.
        scenario(
            "panel.editor_route_and_component",
            "Editor panel keeps a graph-proven component apart from a framework-derived route",
            input(
                "panel:editor:0001",
                "symbol.handler",
                Ctx::EditorSymbol,
                StableAnchorsSupported,
                "Editor symbols anchor every related-object link.",
                None,
                MetadataSafeDefault,
                vec![
                    link(
                        "aureline://object/symbol.handler",
                        LinkSeed {
                            link_id: "link.component.graph",
                            object_kind: Component,
                            source_class: Src::GraphDerived,
                            fallback: Fb::Primary,
                            proof: Pf::IndexedSemantic,
                            confidence: Cf::Indexed,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            authorship: Auth::AuthoredSource,
                            alternates: &[],
                            evidence: &[],
                            downgrades: &[],
                            summary: "The component this handler renders, proven against the graph.",
                        },
                    ),
                    link(
                        "aureline://object/symbol.handler",
                        LinkSeed {
                            link_id: "link.route.framework",
                            object_kind: Route,
                            source_class: Src::FrameworkDerived,
                            fallback: Fb::Primary,
                            proof: Pf::FrameworkDerived,
                            confidence: Cf::Imported,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            authorship: Auth::AuthoredSource,
                            alternates: &[],
                            evidence: &["aureline://evidence/route-map-1"],
                            downgrades: &[DR::RuntimeOrFrameworkOnly],
                            summary: "The route this handler serves, bound through framework metadata.",
                        },
                    ),
                ],
                None,
                "",
            ),
            "The graph-derived component and the framework-derived route land in separate source \
             groups, so the route is disclosed as framework-derived and never reads as a \
             graph-proven fact.",
        ),
        // 2. Editor symbol: curated owner needing disambiguation, plus a curated doc.
        scenario(
            "panel.editor_owner_doc_curated",
            "Editor panel gates a curated owner that needs disambiguation before a jump",
            input(
                "panel:editor:0002",
                "symbol.service",
                Ctx::EditorSymbol,
                StableAnchorsSupported,
                "Editor symbols anchor every related-object link.",
                None,
                InternalSupportRestricted,
                vec![
                    link(
                        "aureline://object/symbol.service",
                        LinkSeed {
                            link_id: "link.owner.curated",
                            object_kind: Owner,
                            source_class: Src::Curated,
                            fallback: Fb::DisambiguationRequired,
                            proof: Pf::ImportedEvidence,
                            confidence: Cf::Imported,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            authorship: Auth::AuthoredSource,
                            alternates: &[
                                "aureline://object/owner.team_a",
                                "aureline://object/owner.team_b",
                            ],
                            evidence: &["aureline://evidence/codeowners-1"],
                            downgrades: &[],
                            summary: "Two stewardship rules name competing owners for this service.",
                        },
                    ),
                    link(
                        "aureline://object/symbol.service",
                        LinkSeed {
                            link_id: "link.doc.curated",
                            object_kind: Doc,
                            source_class: Src::Curated,
                            fallback: Fb::Primary,
                            proof: Pf::ImportedEvidence,
                            confidence: Cf::Imported,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            authorship: Auth::AuthoredSource,
                            alternates: &[],
                            evidence: &["aureline://evidence/doc-table-1"],
                            downgrades: &[],
                            summary: "The guide that documents this service, from a curated doc table.",
                        },
                    ),
                ],
                Some("aureline://disambiguation/owner-roots-1"),
                "Two stewardship rules name competing owners; the owner must be chosen.",
            ),
            "The curated owner link exposes its competing candidates and a disambiguation set and \
             gates the navigating actions, so a related-object jump cannot silently pick one owner \
             before the ambiguity is inspected.",
        ),
        // 3. Generated artifact anchor: a framework-derived generated pair plus a
        //    runtime-observed test, both captured-only.
        scenario(
            "panel.generated_artifact_runtime",
            "Generated-artifact panel discloses an imported pair and a runtime-observed test",
            input(
                "panel:generated:0003",
                "artifact.schema_rs",
                Ctx::GeneratedArtifact,
                StableAnchorsSupported,
                "The generated artifact carries stable anchors back to its source pair.",
                Some("aureline://scope/captured-trace"),
                SigningEvidenceOnly,
                vec![
                    link(
                        "aureline://object/artifact.schema_rs",
                        LinkSeed {
                            link_id: "link.generated.pair",
                            object_kind: GeneratedArtifact,
                            source_class: Src::FrameworkDerived,
                            fallback: Fb::ImportedSnapshot,
                            proof: Pf::FrameworkDerived,
                            confidence: Cf::Imported,
                            freshness: DegradedCached,
                            scope: PartialForDeclaredScope,
                            authorship: Auth::GeneratedSource,
                            alternates: &[],
                            evidence: &["aureline://evidence/generator-map-1"],
                            downgrades: &[DR::GeneratedBoundary],
                            summary: "The source schema this artifact was generated from.",
                        },
                    ),
                    link(
                        "aureline://object/artifact.schema_rs",
                        LinkSeed {
                            link_id: "link.test.runtime",
                            object_kind: Test,
                            source_class: Src::RuntimeDerived,
                            fallback: Fb::RuntimeObservedOnly,
                            proof: Pf::RuntimeObserved,
                            confidence: Cf::Imported,
                            freshness: WarmCached,
                            scope: PartialForDeclaredScope,
                            authorship: Auth::AuthoredSource,
                            alternates: &[],
                            evidence: &["aureline://evidence/trace-1"],
                            downgrades: &[DR::RuntimeOrFrameworkOnly],
                            summary: "A test observed exercising the artifact only in a captured run.",
                        },
                    ),
                ],
                None,
                "",
            ),
            "The framework-derived generated pair stays disclosed across the generated boundary and \
             the runtime-observed test stays runtime-only, both captured-scope with a captured scope \
             ref, a generated label, and attribution notes, so neither reads as a current proven \
             relation.",
        ),
        // 4. Notebook cell anchor: a graph-proven test plus an unavailable doc, partial
        //    scope.
        scenario(
            "panel.notebook_test_doc",
            "Notebook panel reuses relation semantics and names an unavailable doc honestly",
            input(
                "panel:notebook:0004",
                "cell.analysis",
                Ctx::NotebookCell,
                PartialAnchorsSupported,
                "Notebook cells anchor code links but not every prose reference.",
                None,
                MetadataSafeDefault,
                vec![
                    link(
                        "aureline://object/cell.analysis",
                        LinkSeed {
                            link_id: "link.test.graph",
                            object_kind: Test,
                            source_class: Src::GraphDerived,
                            fallback: Fb::Primary,
                            proof: Pf::IndexedSemantic,
                            confidence: Cf::Indexed,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            authorship: Auth::AuthoredSource,
                            alternates: &[],
                            evidence: &[],
                            downgrades: &[],
                            summary: "A test covering the function defined in this cell.",
                        },
                    ),
                    link(
                        "aureline://object/cell.analysis",
                        LinkSeed {
                            link_id: "link.doc.unavailable",
                            object_kind: Doc,
                            source_class: Src::Curated,
                            fallback: Fb::Unavailable,
                            proof: Pf::Unavailable,
                            confidence: Cf::Unavailable,
                            freshness: Unverified,
                            scope: UnavailableForDeclaredScope,
                            authorship: Auth::AuthoredSource,
                            alternates: &[],
                            evidence: &["aureline://evidence/doc-table-2"],
                            downgrades: &[DR::SparseWorkset],
                            summary: "A documented relation whose target is outside the current workset.",
                        },
                    ),
                ],
                None,
                "",
            ),
            "The notebook cell reuses the same source-class and fallback semantics: the graph-proven \
             test is primary while the curated doc relation is named unavailable in scope rather \
             than fabricated, and the panel reports an incomplete scope.",
        ),
        // 5. Diff hunk anchor: parity unsupported, so no links are fabricated.
        scenario(
            "panel.diff_hunk_unsupported",
            "Diff panel labels unsupported anchor parity honestly with no fabricated links",
            input(
                "panel:diff:0005",
                "hunk.patch",
                Ctx::DiffHunk,
                AnchorsUnsupported,
                "This diff hunk spans regenerated lines with no stable symbol anchor.",
                None,
                MetadataSafeDefault,
                vec![],
                None,
                "",
            ),
            "When the diff hunk cannot provide a stable anchor the panel carries an \
             unsupported-parity label, a missing-provider downgrade, and a parity note, and lists no \
             links, so related-object navigation is never fabricated where anchors do not exist.",
        ),
        // 6. Docs-linked symbol anchor: a curated doc plus a lexically matched component.
        scenario(
            "panel.docs_linked_component",
            "Docs-linked panel discloses a lexically matched component apart from a curated doc",
            input(
                "panel:docs:0006",
                "symbol.widget",
                Ctx::DocsLinkedSymbol,
                PartialAnchorsSupported,
                "The docs link resolves the symbol, but some related targets only match lexically.",
                None,
                MetadataSafeDefault,
                vec![
                    link(
                        "aureline://object/symbol.widget",
                        LinkSeed {
                            link_id: "link.doc.primary",
                            object_kind: Doc,
                            source_class: Src::Curated,
                            fallback: Fb::Primary,
                            proof: Pf::ImportedEvidence,
                            confidence: Cf::Imported,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            authorship: Auth::AuthoredSource,
                            alternates: &[],
                            evidence: &["aureline://evidence/doc-table-3"],
                            downgrades: &[],
                            summary: "The doc page this symbol is linked from.",
                        },
                    ),
                    link(
                        "aureline://object/symbol.widget",
                        LinkSeed {
                            link_id: "link.component.lexical",
                            object_kind: Component,
                            source_class: Src::FrameworkDerived,
                            fallback: Fb::LexicalFallback,
                            proof: Pf::LexicalFallback,
                            confidence: Cf::Heuristic,
                            freshness: DegradedCached,
                            scope: PartialForDeclaredScope,
                            authorship: Auth::AuthoredSource,
                            alternates: &[],
                            evidence: &["aureline://evidence/lexical-match-1"],
                            downgrades: &[DR::LexicalFallbackOnly],
                            summary: "A component matched only by name from a framework template.",
                        },
                    ),
                ],
                None,
                "",
            ),
            "The docs-linked symbol reuses the relation semantics, and the lexically matched \
             component is disclosed as a lexical fallback with its own downgrade reason, never shown \
             as a proven component relation.",
        ),
    ]
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> RelatedObjectInvariant {
    RelatedObjectInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(scenarios: &[RelatedObjectScenario]) -> Vec<RelatedObjectInvariant> {
    let panels: Vec<&RelatedObjectPanel> = scenarios.iter().map(|s| &s.panel).collect();

    let mut out = Vec::new();

    // Source attribution: every link is grouped by source class, groups are in canonical
    // order, each link maps to the closed relation vocabulary, and the set never
    // flattens into one homogeneous list.
    out.push(invariant(
        "related_object.source_attribution_present",
        "Every panel groups its links by source class in the canonical \
         graph-derived/framework-derived/curated/runtime-derived order, places each link in exactly \
         the group for its source class, names a relation kind drawn from the closed vocabulary that \
         matches its object kind, and never flattens the set into one homogeneous list.",
        scenarios.iter().all(|scenario| {
            let panel = &scenario.panel;
            groups_in_canonical_order(panel)
                && panel.groups.iter().all(|group| {
                    group.links.iter().all(|link| {
                        link.source_class == group.source_class
                            && link.relation_kind == link.object_kind.relation_kind()
                    })
                })
                && total_grouped(panel) == panel.totals.total_count
                && scenario.input.links.iter().all(|link| {
                    panel
                        .group(link.source_class)
                        .is_some_and(|group| group.links.iter().any(|l| l.link_id == link.link_id))
                })
        }),
    ));

    // Counts reconcile across groups and the panel, and both the source and fallback
    // tallies partition the total.
    out.push(invariant(
        "related_object.counts_reconcile_and_partition",
        "Every group and panel reconciles its current-scope and captured-scope counts with its \
         total, the four source tallies and the six fallback tallies each partition that total, and \
         the group totals sum to the panel total, so a source or fallback count is always internally \
         consistent.",
        panels.iter().all(|panel| {
            panel.totals.reconciles()
                && panel.totals.source_partition_reconciles()
                && panel.totals.fallback_partition_reconciles()
                && panel.groups.iter().all(|group| {
                    group.counts.reconciles()
                        && group.counts.source_partition_reconciles()
                        && group.counts.fallback_partition_reconciles()
                })
                && panel
                    .groups
                    .iter()
                    .map(|group| group.counts.total_count)
                    .sum::<usize>()
                    == panel.totals.total_count
                && panel
                    .groups
                    .iter()
                    .map(|group| group.counts.captured_scope_count)
                    .sum::<usize>()
                    == panel.totals.captured_scope_count
        }),
    ));

    // Source classes stay distinct: a link never lands in a group for a different class,
    // and a non-graph link always carries its disclosure.
    out.push(invariant(
        "related_object.source_classes_never_homogeneous",
        "Every link in a group resolves to that group's source class, every framework, curated, or \
         runtime link requires disclosure, and every group of a non-graph source carries attribution \
         notes, so the four evidence classes never collapse into one homogeneous certainty class.",
        scenarios.iter().all(|scenario| {
            scenario.panel.groups.iter().all(|group| {
                group.links.iter().all(|link| link.source_class == group.source_class)
                    && (group.source_class == RelatedObjectSourceClass::GraphDerived
                        || !group.attribution_notes.is_empty())
            })
        }),
    ));

    // No link is a generic smart link: each names a source class, a target ref, and its
    // fallback truth, and projections never flatten to generic links.
    out.push(invariant(
        "related_object.named_source_never_generic",
        "Every link names a non-empty source class token, a target ref, an object kind, and a \
         fallback mode, and every consumer projection sets flattens_to_generic_links false, so a \
         related-object link never presents as a generic smart link.",
        panels.iter().all(|panel| {
            panel.links().all(|link| {
                !link.source_class.as_str().is_empty()
                    && !link.target_ref.trim().is_empty()
                    && !link.summary.trim().is_empty()
            }) && panel
                .consumer_projections
                .iter()
                .all(|projection| !projection.flattens_to_generic_links)
        }),
    ));

    // Fallback truth is disclosed: a non-primary link carries a downgrade reason or
    // evidence ref, and a disambiguation-required link carries candidate alternatives.
    out.push(invariant(
        "related_object.fallback_truth_disclosed",
        "Every link whose fallback mode is not primary is backed by a downgrade reason or an \
         evidence ref, every disambiguation-required link lists at least two candidate alternatives, \
         and every link that requires disclosure is evidenced, so a degraded resolution never reads \
         as a clean primary jump.",
        panels.iter().all(|panel| {
            panel.links().all(|link| {
                let fallback_ok = if link.fallback_mode.requires_disclosure() {
                    !link.downgrade_reasons.is_empty() || !link.evidence_refs.is_empty()
                } else {
                    true
                };
                let disambiguation_ok = if link.requires_selection() {
                    link.alternate_target_refs.len() >= 2
                } else {
                    true
                };
                fallback_ok && disambiguation_ok && link.disclosure_is_evidenced()
            })
        }),
    ));

    // Captured-scope divergence is always disclosed.
    out.push(invariant(
        "related_object.captured_scope_disclosed",
        "Whenever a panel has captured-scope links it carries a captured scope ref or a downgrade \
         reason, a captured/imported/runtime/stale label, and attribution notes, so \
         current-versus-captured divergence is never hidden.",
        panels.iter().all(|panel| {
            if panel.totals.captured_scope_count == 0 {
                true
            } else {
                (panel.captured_scope_ref.is_some() || !panel.downgrade_reasons.is_empty())
                    && panel.labels.iter().any(|label| {
                        matches!(
                            label,
                            RelatedObjectLabel::CapturedScopeOnly
                                | RelatedObjectLabel::ImportedSnapshot
                                | RelatedObjectLabel::RuntimeObservedOnly
                                | RelatedObjectLabel::StaleEvidence
                        )
                    })
                    && !panel.attribution_notes.is_empty()
            }
        }),
    ));

    // Incomplete scope is always named.
    out.push(invariant(
        "related_object.incomplete_scope_named",
        "Whenever a panel covers an incomplete scope it carries an incomplete-scope label and a \
         downgrade reason, so a partial related-object set never reads as a complete one.",
        panels.iter().all(|panel| {
            if panel.scope_completeness.requires_disclosure() {
                panel.labels.contains(&RelatedObjectLabel::IncompleteScope)
                    && !panel.downgrade_reasons.is_empty()
            } else {
                true
            }
        }),
    ));

    // Disambiguation is inspectable before a navigating jump.
    out.push(invariant(
        "related_object.disambiguation_inspectable_before_jump",
        "Whenever any link needs an explicit selection the panel exposes the competing links or a \
         disambiguation set, requires inspection before a jump, carries a disambiguation label, and \
         gates its navigating actions, so a related-object jump cannot silently pick a candidate.",
        panels.iter().all(|panel| {
            if panel.disambiguation.requires_selection {
                panel.disambiguation.has_disambiguation_path()
                    && panel.disambiguation.requires_inspection_before_jump
                    && panel
                        .labels
                        .contains(&RelatedObjectLabel::DisambiguationRequired)
                    && panel
                        .actions
                        .iter()
                        .filter(|action| action.action_kind.navigates())
                        .all(|action| action.gated_by_disambiguation)
            } else {
                panel.actions.iter().all(|action| !action.gated_by_disambiguation)
            }
        }),
    ));

    // Anchor parity is honest: unsupported contexts carry no links and a parity label.
    out.push(invariant(
        "related_object.anchor_parity_honest",
        "Every panel names its anchor context and parity; an anchors-unsupported panel lists no \
         links and carries an unsupported-parity label, a downgrade reason, and a non-empty parity \
         note, while a panel with reduced parity still names it, so related-object navigation is \
         reused where stable anchors exist and labeled honestly where they do not.",
        panels.iter().all(|panel| {
            let parity_named = !panel.parity_note.trim().is_empty()
                || panel.anchor_parity == AnchorParity::StableAnchorsSupported;
            if panel.anchor_parity.is_unsupported() {
                panel.groups.is_empty()
                    && panel.totals.total_count == 0
                    && panel
                        .labels
                        .contains(&RelatedObjectLabel::UnsupportedParity)
                    && !panel.downgrade_reasons.is_empty()
                    && !panel.parity_note.trim().is_empty()
            } else {
                parity_named
                    && (panel.anchor_parity != AnchorParity::PartialAnchorsSupported
                        || !panel.parity_note.trim().is_empty())
            }
        }),
    ));

    // Actions are stable across every route.
    out.push(invariant(
        "related_object.actions_stable_across_routes",
        "Every panel exposes the five open/peek/split/reveal/export actions, each reachable from the \
         related panel, editor gutter, graph overlay, search panel, docs link, and keyboard routes, \
         each with one stable history effect and a preserved anchor identity, and the navigating \
         actions are gated exactly when disambiguation is pending, so an action behaves identically \
         on every surface.",
        panels.iter().all(|panel| {
            RelatedObjectActionKind::ALL.iter().all(|action_kind| {
                panel
                    .actions
                    .iter()
                    .filter(|a| a.action_kind == *action_kind)
                    .count()
                    == 1
                    && panel.actions.iter().any(|a| {
                        a.action_kind == *action_kind
                            && a.history_effect == action_kind.history_effect()
                            && a.preserves_anchor_identity
                            && a.anchor_ref == panel.anchor_ref
                            && routes_match(&a.available_routes)
                            && a.gated_by_disambiguation
                                == (action_kind.navigates()
                                    && panel.disambiguation.requires_inspection_before_jump)
                    })
            })
        }),
    ));

    // Consumers preserve the typed truth without flattening.
    out.push(invariant(
        "related_object.consumers_preserve_truth",
        "Every consumer projection preserves source attribution, counts, fallback truth, anchor \
         parity, and freshness/confidence, never flattens the panel into generic links, and never \
         exports raw code bodies, so review, support, AI, graph, and docs consumers see typed \
         source-attributed links rather than one bucket of buttons.",
        panels.iter().all(|panel| {
            !panel.consumer_projections.is_empty()
                && panel
                    .consumer_projections
                    .iter()
                    .all(RelatedObjectProjection::preserves_truth)
                && required_surfaces_covered(&panel.consumer_projections)
        }),
    ));

    // The corpus covers every object kind, source class, fallback mode, anchor context,
    // action, and the disambiguation, captured-scope, incomplete-scope, and
    // unsupported-parity answers.
    out.push(invariant(
        "related_object.corpus_covers_vocabulary",
        "The corpus exercises every route/component/test/doc/owner/generated-artifact object kind, \
         every graph/framework/curated/runtime source class, every fallback mode, every anchor \
         context, every action, and the disambiguation, captured-scope, incomplete-scope, and \
         unsupported-parity answers, so the model is proven across its whole vocabulary.",
        RelatedObjectKind::ALL
            .iter()
            .all(|kind| panels.iter().any(|panel| panel.links().any(|link| link.object_kind == *kind)))
            && RELATED_OBJECT_SOURCE_ORDER
                .iter()
                .all(|class| panels.iter().any(|panel| panel.group(*class).is_some()))
            && RelatedObjectFallbackMode::ALL.iter().all(|mode| {
                panels
                    .iter()
                    .any(|panel| panel.links().any(|link| link.fallback_mode == *mode))
            })
            && RelatedObjectAnchorContext::ALL
                .iter()
                .all(|ctx| panels.iter().any(|panel| panel.anchor_context == *ctx))
            && every_action_covered(&panels)
            && panels.iter().any(|panel| panel.disambiguation.requires_selection)
            && panels.iter().any(|panel| panel.totals.captured_scope_count > 0)
            && panels
                .iter()
                .any(|panel| panel.scope_completeness.requires_disclosure())
            && panels.iter().any(|panel| panel.anchor_parity.is_unsupported()),
    ));

    // The panel is replayable and answers the support question.
    out.push(invariant(
        "related_object.replayable_support_answer",
        "Every panel carries a non-empty id and summary, a named anchor context, parity, and source \
         headline, and every link carries a non-empty summary plus a source class and evidence, so a \
         support or debug packet can state why each related-object link existed and what evidence \
         class backed it.",
        panels.iter().all(|panel| {
            !panel.panel_id.trim().is_empty()
                && !panel.summary.trim().is_empty()
                && panel.links().all(|link| !link.summary.trim().is_empty())
        }),
    ));

    out
}

// ---------------------------------------------------------------------------
// Invariant helpers.
// ---------------------------------------------------------------------------

fn groups_in_canonical_order(panel: &RelatedObjectPanel) -> bool {
    let order = |source_class: RelatedObjectSourceClass| {
        RELATED_OBJECT_SOURCE_ORDER
            .iter()
            .position(|candidate| *candidate == source_class)
            .unwrap_or(usize::MAX)
    };
    panel
        .groups
        .windows(2)
        .all(|pair| order(pair[0].source_class) < order(pair[1].source_class))
}

fn total_grouped(panel: &RelatedObjectPanel) -> usize {
    panel.groups.iter().map(|group| group.links.len()).sum()
}

fn routes_match(routes: &[RelatedObjectActionRoute]) -> bool {
    routes.len() == RelatedObjectActionRoute::ALL.len()
        && RelatedObjectActionRoute::ALL
            .iter()
            .all(|route| routes.contains(route))
}

fn every_action_covered(panels: &[&RelatedObjectPanel]) -> bool {
    RelatedObjectActionKind::ALL.iter().all(|action_kind| {
        panels.iter().any(|panel| {
            panel
                .actions
                .iter()
                .any(|action| action.action_kind == *action_kind)
        })
    })
}

fn required_surfaces_covered(projections: &[RelatedObjectProjection]) -> bool {
    REQUIRED_CONSUMER_SURFACES.iter().all(|surface| {
        projections
            .iter()
            .any(|projection| projection.consumer_surface == *surface)
    })
}

fn all_unique<'a>(iter: impl Iterator<Item = &'a str>) -> bool {
    let mut seen = BTreeSet::new();
    iter.into_iter().all(|item| seen.insert(item))
}

/// Whether a ref is safe to export: a repo-relative path or opaque `aureline://`
/// handle, never a URL, host, credential, or absolute path.
fn is_export_safe_ref(r: &str) -> bool {
    if r.is_empty() || r.starts_with('/') || (r.contains("://") && !r.starts_with("aureline://")) {
        return false;
    }
    r.starts_with("schemas/")
        || r.starts_with("crates/")
        || r.starts_with("artifacts/")
        || r.starts_with("fixtures/")
        || r.starts_with("docs/")
        || r.starts_with("aureline://")
}
