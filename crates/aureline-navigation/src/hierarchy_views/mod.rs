//! Hierarchy views: the typed view and export model over hierarchy edges.
//!
//! The [`target_model`](crate::target_model) freezes the
//! [`hierarchy edge`](crate::target_model::HierarchyEdge) object — one call, type,
//! override, ownership, framework, or documentation relation, with its proof class,
//! depth, scope completeness, freshness, and confidence. The
//! [`relation-navigation matrix`](crate::m5_relation_navigation) names that object
//! family and pins its vocabulary. What was still implicit is the *view and export
//! model*: how Aureline turns a flat set of edges into a **call**, **type**,
//! **override**, or **ownership** hierarchy view that says **how each edge was
//! reached** (a direct / transitive / inferred / runtime-observed legend), **what
//! scope it covers** (scope completeness plus explicitly named missing scopes),
//! **how fresh and confident** the evidence is, and **whether the root or edge set
//! is ambiguous** — and how that typed truth survives into review evidence, support
//! exports, and AI/graph/docs consumers instead of being flattened into one opaque
//! tree snapshot.
//!
//! This module is that model. [`build_hierarchy_view`] is a pure function over a
//! typed [`HierarchyViewInput`] that produces a [`HierarchyView`]:
//!
//! 1. **Legend grouping.** Edges are grouped into [`HierarchyTier`]s keyed by an
//!    [`HierarchyEdgeLegend`] in a canonical order — direct, transitive, inferred,
//!    runtime-observed — so direct proof is never blended with transitive structure,
//!    an inferred framework guess never poses as direct proof, and a
//!    runtime-observed edge stays named as observed rather than proven.
//! 2. **Scope completeness and missing scopes.** Each tier and the view carry a
//!    [`ScopeCompleteness`](crate::target_model::ScopeCompleteness), and the view
//!    names every hidden or missing scope explicitly as a [`HierarchyScopeGap`], so
//!    an incomplete hierarchy never reads as a complete one.
//! 3. **Provider attribution and freshness.** Each tier carries the aggregate
//!    freshness, confidence, and the proof classes behind its edges, so support and
//!    review evidence can say which provider/source admitted an edge and how fresh it
//!    is.
//! 4. **Ambiguity and disambiguation.** The view carries a
//!    [`HierarchyAmbiguityState`] that, when multiple hierarchy roots or edge sets
//!    compete, exposes the competing roots and a disambiguation set ref and gates the
//!    navigating actions, so a user inspects the ambiguity before a hierarchy jump
//!    changes context or meaning.
//! 5. **Stable actions.** Each view exposes the same [`HierarchyActionKind`]s —
//!    open, peek, split-open, expand, export — on every [`HierarchyActionRoute`]
//!    with one stable [`HierarchyHistoryEffect`] each, and the navigating actions are
//!    gated whenever the root is ambiguous.
//! 6. **Consumer parity.** Each view projects to every
//!    [`ConsumerSurface`](crate::target_model::ConsumerSurface) with a
//!    [`HierarchyViewProjection`] that preserves legend grouping, edge counts, scope
//!    completeness, freshness/confidence, and ambiguity state, never flattens to a
//!    single opaque tree, and never exports raw code bodies.
//!
//! [`hierarchy_views_set`] freezes a deterministic corpus of views whose
//! [`HierarchyViewInvariant`] flags are computed from the builder's own output, so
//! the checked-in fixture and the freeze gate pin the contract byte-for-byte and any
//! regression in [`build_hierarchy_view`] flips an invariant or drifts the fixture
//! rather than silently passing. The records carry no source bodies, raw paths,
//! provider payloads, URLs, hostnames, or credentials — only opaque object handles,
//! stable tokens, and short reviewable sentences — so they are safe for support
//! export.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::target_model::{
    AmbiguityClass, ConsumerSurface, DowngradeReason, ExportRedactionClass, FreshnessClass,
    HierarchyEdge, HierarchyEdgeKind, NavigationConfidence, ProofClass, ScopeCompleteness,
    REQUIRED_CONSUMER_SURFACES,
};

#[cfg(test)]
mod tests;

/// Schema version for the hierarchy-views corpus.
pub const HIERARCHY_VIEWS_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the hierarchy-views corpus.
pub const HIERARCHY_VIEWS_SCHEMA_REF: &str = "schemas/navigation/hierarchy_views.schema.json";

/// Stable record-kind tag for the hierarchy-views corpus.
pub const HIERARCHY_VIEWS_RECORD_KIND: &str = "hierarchy_views_set";

/// Stable id for the canonical hierarchy-views corpus.
pub const HIERARCHY_VIEWS_SET_ID: &str = "hierarchy-views:set:0001";

/// Evaluation stamp for the canonical corpus. Held as a constant so the canonical
/// binding stays deterministic and the fixture freezes byte-for-byte.
pub const HIERARCHY_VIEWS_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The freeze gate that keeps the corpus binding current. Stable promotion runs this
/// gate; it fails when the in-code corpus drifts from the checked-in fixture or any
/// invariant flips.
pub const HIERARCHY_VIEWS_FREEZE_GATE_REF: &str =
    "crates/aureline-navigation/tests/hierarchy_views.rs";

/// Reviewer doc for the hierarchy-views contract.
pub const HIERARCHY_VIEWS_DOC_REF: &str = "docs/navigation/hierarchy_views.md";

/// Evidence companion for the hierarchy-views corpus.
pub const HIERARCHY_VIEWS_ARTIFACT_REF: &str = "artifacts/navigation/hierarchy_views.md";

/// Repo-relative path of the checked-in canonical corpus.
pub const HIERARCHY_VIEWS_FIXTURE_REF: &str =
    "fixtures/navigation/hierarchy_views/canonical_views.json";

/// The canonical legend ordering for hierarchy tiers.
///
/// A hierarchy view lists its tiers in this order so direct proof is presented
/// before transitive structure, inferred edges, and runtime-observed corroboration —
/// and so an inferred or runtime edge is never blended into the direct-proof tier.
pub const HIERARCHY_LEGEND_ORDER: [HierarchyEdgeLegend; 4] = [
    HierarchyEdgeLegend::Direct,
    HierarchyEdgeLegend::Transitive,
    HierarchyEdgeLegend::Inferred,
    HierarchyEdgeLegend::RuntimeObserved,
];

// ---------------------------------------------------------------------------
// View kind and direction.
// ---------------------------------------------------------------------------

/// The kind of hierarchy a view presents.
///
/// A view is one of call, type, override, or ownership; the kind constrains which
/// [`HierarchyEdgeKind`]s are admissible, so a call view never silently shows
/// inheritance edges and a type view never shows ownership edges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HierarchyViewKind {
    /// A call hierarchy: callers and callees, including framework/route bindings.
    Call,
    /// A type hierarchy: inheritance and implementation relations.
    Type,
    /// An override hierarchy: method/member override relations.
    Override,
    /// An ownership hierarchy: stewardship and CODEOWNERS-like relations.
    Ownership,
}

impl HierarchyViewKind {
    /// All view kinds, in canonical order.
    pub const ALL: [Self; 4] = [Self::Call, Self::Type, Self::Override, Self::Ownership];

    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Call => "call",
            Self::Type => "type",
            Self::Override => "override",
            Self::Ownership => "ownership",
        }
    }

    /// Returns a human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Call => "Call Hierarchy",
            Self::Type => "Type Hierarchy",
            Self::Override => "Override Hierarchy",
            Self::Ownership => "Ownership Hierarchy",
        }
    }

    /// Returns true when this view kind admits the given hierarchy edge kind.
    ///
    /// Keeping this explicit means a hierarchy view can be checked for a stray edge
    /// kind rather than silently mixing call edges into a type hierarchy.
    pub const fn admits(self, edge_kind: HierarchyEdgeKind) -> bool {
        match self {
            Self::Call => matches!(
                edge_kind,
                HierarchyEdgeKind::Calls
                    | HierarchyEdgeKind::RuntimeCalls
                    | HierarchyEdgeKind::FrameworkBinding
            ),
            Self::Type => matches!(
                edge_kind,
                HierarchyEdgeKind::Inherits | HierarchyEdgeKind::Implements
            ),
            Self::Override => matches!(edge_kind, HierarchyEdgeKind::Overrides),
            Self::Ownership => matches!(edge_kind, HierarchyEdgeKind::Owner),
        }
    }
}

/// The direction a hierarchy view expands from its root.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HierarchyDirection {
    /// Edges point toward the root (callers, subtypes, overriders, owned-by).
    Incoming,
    /// Edges point away from the root (callees, supertypes, overridden, owns).
    Outgoing,
    /// The view shows both directions at once.
    Bidirectional,
}

impl HierarchyDirection {
    /// All directions, in canonical order.
    pub const ALL: [Self; 3] = [Self::Incoming, Self::Outgoing, Self::Bidirectional];

    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Incoming => "incoming",
            Self::Outgoing => "outgoing",
            Self::Bidirectional => "bidirectional",
        }
    }
}

// ---------------------------------------------------------------------------
// Legend.
// ---------------------------------------------------------------------------

/// How a hierarchy edge was reached — its legend.
///
/// Answers the support/debug question "is this edge direct proof, transitive
/// structure, an inferred guess, or runtime corroboration?". Every concrete edge
/// resolves to exactly one of the four grouping legends in [`HIERARCHY_LEGEND_ORDER`];
/// a view whose edges span more than one legend resolves its headline to [`Mixed`],
/// and a view with no edges resolves to [`Empty`].
///
/// [`Mixed`]: HierarchyEdgeLegend::Mixed
/// [`Empty`]: HierarchyEdgeLegend::Empty
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HierarchyEdgeLegend {
    /// Direct semantic proof of an edge adjacent to the root.
    Direct,
    /// Semantic structure reached by transitively walking proven edges.
    Transitive,
    /// An edge inferred from framework/route metadata, an imported snapshot, AI, or a
    /// lexical/syntax fallback — never directly proven.
    Inferred,
    /// An edge corroborated by a runtime trace, profile, or debugger.
    RuntimeObserved,
    /// The view mixes more than one legend (headline only; never a tier).
    Mixed,
    /// The view has no edges (headline only; never a tier).
    Empty,
}

impl HierarchyEdgeLegend {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Transitive => "transitive",
            Self::Inferred => "inferred",
            Self::RuntimeObserved => "runtime_observed",
            Self::Mixed => "mixed",
            Self::Empty => "empty",
        }
    }

    /// Returns true when this legend must render with a visible caveat rather than as
    /// plain direct proof.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Direct | Self::Transitive)
    }

    /// Returns true when this legend is a tier legend (one of the four grouping
    /// legends) rather than a headline-only legend.
    pub const fn is_tier_legend(self) -> bool {
        matches!(
            self,
            Self::Direct | Self::Transitive | Self::Inferred | Self::RuntimeObserved
        )
    }
}

/// Returns the legend a concrete hierarchy edge resolves to.
///
/// Proof class wins first: a runtime-observed edge is always
/// [`RuntimeObserved`](HierarchyEdgeLegend::RuntimeObserved); a framework, imported,
/// AI, or lexical/syntax edge is always [`Inferred`](HierarchyEdgeLegend::Inferred).
/// Only a direct or indexed semantic edge is direct proof, and it is
/// [`Direct`](HierarchyEdgeLegend::Direct) when adjacent to the root (`depth <= 1`)
/// or [`Transitive`](HierarchyEdgeLegend::Transitive) when deeper.
pub fn edge_legend(edge: &HierarchyEdge) -> HierarchyEdgeLegend {
    match edge.proof_class {
        ProofClass::RuntimeObserved => HierarchyEdgeLegend::RuntimeObserved,
        ProofClass::FrameworkDerived
        | ProofClass::ImportedEvidence
        | ProofClass::AiInferred
        | ProofClass::LexicalFallback
        | ProofClass::SyntaxFallback
        | ProofClass::Unavailable => HierarchyEdgeLegend::Inferred,
        ProofClass::DirectSemantic | ProofClass::IndexedSemantic => {
            if edge.depth <= 1 {
                HierarchyEdgeLegend::Direct
            } else {
                HierarchyEdgeLegend::Transitive
            }
        }
    }
}

/// A user-visible label a hierarchy view attaches to a tier or to the whole view.
///
/// Labels keep weaker-proof, captured-scope, incomplete-scope, and ambiguous-root
/// facts visible rather than folding them into an undifferentiated tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HierarchyLabel {
    /// The tier or view includes transitive (multi-hop) structure.
    Transitive,
    /// The tier or view includes inferred edges.
    Inferred,
    /// The tier or view includes runtime-observed edges.
    RuntimeObserved,
    /// The tier or view includes framework-derived edges.
    FrameworkDerived,
    /// The tier or view includes imported-snapshot edges.
    ImportedSnapshot,
    /// The tier or view includes lexical/grep fallback edges.
    LexicalFallback,
    /// The tier or view includes syntax-fallback edges.
    SyntaxFallback,
    /// The tier or view includes generated-boundary edges.
    Generated,
    /// Every edge in the tier is carried only by a captured scope, not the current
    /// scope.
    CapturedScopeOnly,
    /// The tier or view covers an incomplete scope.
    IncompleteScope,
    /// The tier or view rests on stale or unverified evidence.
    StaleEvidence,
    /// The view's root or edge set is ambiguous and has competing candidates.
    CompetingRoots,
}

impl HierarchyLabel {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Transitive => "transitive",
            Self::Inferred => "inferred",
            Self::RuntimeObserved => "runtime_observed",
            Self::FrameworkDerived => "framework_derived",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::LexicalFallback => "lexical_fallback",
            Self::SyntaxFallback => "syntax_fallback",
            Self::Generated => "generated",
            Self::CapturedScopeOnly => "captured_scope_only",
            Self::IncompleteScope => "incomplete_scope",
            Self::StaleEvidence => "stale_evidence",
            Self::CompetingRoots => "competing_roots",
        }
    }
}

// ---------------------------------------------------------------------------
// Actions.
// ---------------------------------------------------------------------------

/// The effect a hierarchy action has on navigation history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HierarchyHistoryEffect {
    /// Pushes a new navigation history entry (open, split-open).
    AdvancesHistory,
    /// Leaves navigation history untouched (peek, expand).
    PreservesCurrent,
    /// Touches no editor history at all (export).
    NoEditorHistory,
}

impl HierarchyHistoryEffect {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdvancesHistory => "advances_history",
            Self::PreservesCurrent => "preserves_current",
            Self::NoEditorHistory => "no_editor_history",
        }
    }
}

/// A stable action a hierarchy view can invoke on a hierarchy node.
///
/// The action set is closed and identical across every [`HierarchyActionRoute`]:
/// open, peek, split-open, expand, and export. Each action has one stable
/// [`HierarchyHistoryEffect`], and the two navigating actions (open, split-open) are
/// gated whenever the view's root is ambiguous, so a jump never silently changes
/// context before the ambiguity is inspected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HierarchyActionKind {
    /// Jump to the node in the active editor, replacing the current view.
    Open,
    /// Peek the node inline without leaving the current editor.
    Peek,
    /// Open the node in a split, leaving the current editor in place.
    SplitOpen,
    /// Expand the node's subtree in the hierarchy view without navigating.
    Expand,
    /// Export the metadata-only hierarchy view; never mutates the editor.
    Export,
}

impl HierarchyActionKind {
    /// All actions, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::Open,
        Self::Peek,
        Self::SplitOpen,
        Self::Expand,
        Self::Export,
    ];

    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Peek => "peek",
            Self::SplitOpen => "split_open",
            Self::Expand => "expand",
            Self::Export => "export",
        }
    }

    /// Returns a human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Open => "Go to Node",
            Self::Peek => "Peek",
            Self::SplitOpen => "Open to the Side",
            Self::Expand => "Expand Subtree",
            Self::Export => "Export Hierarchy",
        }
    }

    /// Returns the stable history effect this action has on navigation history.
    pub const fn history_effect(self) -> HierarchyHistoryEffect {
        match self {
            Self::Open | Self::SplitOpen => HierarchyHistoryEffect::AdvancesHistory,
            Self::Peek | Self::Expand => HierarchyHistoryEffect::PreservesCurrent,
            Self::Export => HierarchyHistoryEffect::NoEditorHistory,
        }
    }

    /// Returns true when this action navigates and therefore can change context.
    pub const fn navigates(self) -> bool {
        matches!(self, Self::Open | Self::SplitOpen)
    }
}

/// A surface route that exposes the hierarchy-view actions.
///
/// The same actions are reachable from every route, so open/peek/split/expand/export
/// behave identically whether invoked from the hierarchy view, a graph overlay, a
/// search panel, a docs link, or a keyboard shortcut.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HierarchyActionRoute {
    /// The dedicated hierarchy view.
    HierarchyView,
    /// A graph or topology overlay.
    GraphOverlay,
    /// A search results panel.
    SearchPanel,
    /// A documentation or help link.
    DocsLink,
    /// A keyboard shortcut route.
    KeyboardShortcut,
}

impl HierarchyActionRoute {
    /// All routes, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::HierarchyView,
        Self::GraphOverlay,
        Self::SearchPanel,
        Self::DocsLink,
        Self::KeyboardShortcut,
    ];

    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HierarchyView => "hierarchy_view",
            Self::GraphOverlay => "graph_overlay",
            Self::SearchPanel => "search_panel",
            Self::DocsLink => "docs_link",
            Self::KeyboardShortcut => "keyboard_shortcut",
        }
    }
}

/// A stable action bound to a hierarchy view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HierarchyActionAffordance {
    /// The action kind.
    pub action_kind: HierarchyActionKind,
    /// The stable history effect for this action.
    pub history_effect: HierarchyHistoryEffect,
    /// Routes the action is reachable from, in canonical order.
    pub available_routes: Vec<HierarchyActionRoute>,
    /// The target ref the action acts on.
    pub target_ref: String,
    /// Always true: the action resolves to the same target across every route.
    pub preserves_target_identity: bool,
    /// True when this navigating action is gated behind ambiguity inspection.
    pub gated_by_ambiguity: bool,
    /// Export-safe summary.
    pub summary: String,
}

// ---------------------------------------------------------------------------
// Counts.
// ---------------------------------------------------------------------------

/// Legend- and scope-partitioned counts for a hierarchy tier or view.
///
/// `current_scope_count` tallies edges proven against the current source or index;
/// `captured_scope_count` tallies edges carried only by a captured snapshot, runtime
/// trace, or imported pack. The two always sum to `total_count`. The legend tallies
/// (`direct`/`transitive`/`inferred`/`runtime_observed`) also partition the total, so
/// a view never claims a direct count it cannot back with direct-proof edges.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct HierarchyEdgeCounts {
    /// Total edges in the tier or view.
    pub total_count: usize,
    /// Edges proven against the current scope.
    pub current_scope_count: usize,
    /// Edges carried only by a captured snapshot, trace, or imported pack.
    pub captured_scope_count: usize,
    /// Direct-proof edges adjacent to the root.
    pub direct_count: usize,
    /// Transitive (multi-hop) semantic edges.
    pub transitive_count: usize,
    /// Inferred edges (framework/import/AI/lexical/syntax).
    pub inferred_count: usize,
    /// Runtime-observed edges.
    pub runtime_observed_count: usize,
    /// Framework-derived edges.
    pub framework_count: usize,
    /// Imported-snapshot edges.
    pub imported_count: usize,
    /// Edges resting on a lexical or syntax fallback.
    pub fallback_count: usize,
    /// Edges covering an incomplete scope.
    pub incomplete_scope_count: usize,
    /// Greatest edge depth in the tier or view.
    pub max_depth: u32,
}

impl HierarchyEdgeCounts {
    /// Returns true when the current and captured counts reconcile with the total.
    pub const fn reconciles(&self) -> bool {
        self.current_scope_count + self.captured_scope_count == self.total_count
    }

    /// Returns true when the four legend tallies partition the total.
    pub const fn legend_partition_reconciles(&self) -> bool {
        self.direct_count
            + self.transitive_count
            + self.inferred_count
            + self.runtime_observed_count
            == self.total_count
    }

    fn add(&mut self, edge: &HierarchyEdge) {
        self.total_count += 1;
        if is_captured_only(edge) {
            self.captured_scope_count += 1;
        } else {
            self.current_scope_count += 1;
        }
        match edge_legend(edge) {
            HierarchyEdgeLegend::Direct => self.direct_count += 1,
            HierarchyEdgeLegend::Transitive => self.transitive_count += 1,
            HierarchyEdgeLegend::Inferred => self.inferred_count += 1,
            HierarchyEdgeLegend::RuntimeObserved => self.runtime_observed_count += 1,
            HierarchyEdgeLegend::Mixed | HierarchyEdgeLegend::Empty => {}
        }
        if edge.proof_class == ProofClass::FrameworkDerived {
            self.framework_count += 1;
        }
        if edge.proof_class == ProofClass::ImportedEvidence {
            self.imported_count += 1;
        }
        if is_fallback_proof(edge.proof_class) {
            self.fallback_count += 1;
        }
        if edge.scope_completeness.requires_disclosure() {
            self.incomplete_scope_count += 1;
        }
        if edge.depth > self.max_depth {
            self.max_depth = edge.depth;
        }
    }
}

// ---------------------------------------------------------------------------
// Scope gaps.
// ---------------------------------------------------------------------------

/// One explicitly named hidden or missing scope a hierarchy view does not cover.
///
/// A hierarchy provider names the scopes it could not materialize — an unindexed
/// crate, a remote shard, a branch outside the workset, a generated boundary — so a
/// partial hierarchy never reads as a complete one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HierarchyScopeGap {
    /// Stable scope ref the view could not cover.
    pub scope_ref: String,
    /// Completeness class for the missing scope.
    pub completeness: ScopeCompleteness,
    /// Downgrade reason explaining why the scope is missing.
    pub reason: DowngradeReason,
    /// Export-safe note naming what is hidden or missing.
    pub note: String,
}

// ---------------------------------------------------------------------------
// Ambiguity.
// ---------------------------------------------------------------------------

/// Whether a hierarchy view's root or edge set is ambiguous, and how to disambiguate.
///
/// When more than one hierarchy root or edge set competes, the view does not silently
/// pick one tree: it exposes the competing roots and a disambiguation set ref and
/// requires inspection before a navigating action runs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HierarchyAmbiguityState {
    /// Ambiguity class for the view's root selection.
    pub ambiguity_class: AmbiguityClass,
    /// Competing root or edge-set refs the user can choose between.
    pub competing_root_refs: Vec<String>,
    /// Disambiguation set ref when the user must choose a successor.
    pub disambiguation_set_ref: Option<String>,
    /// True when ambiguity must be inspected before a hierarchy jump.
    pub requires_inspection_before_jump: bool,
    /// Export-safe note explaining the ambiguity.
    pub note: String,
}

impl HierarchyAmbiguityState {
    /// Returns true when the view exposes a way to disambiguate competing roots.
    pub fn has_disambiguation_path(&self) -> bool {
        !self.competing_root_refs.is_empty() || self.disambiguation_set_ref.is_some()
    }
}

// ---------------------------------------------------------------------------
// Tier, projection, view.
// ---------------------------------------------------------------------------

/// One legend tier inside a hierarchy view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HierarchyTier {
    /// The legend this tier represents.
    pub legend: HierarchyEdgeLegend,
    /// Edge ids in this tier, in view order.
    pub edge_refs: Vec<String>,
    /// Legend- and scope-partitioned counts for the tier.
    pub counts: HierarchyEdgeCounts,
    /// Aggregate scope completeness over the tier (weakest edge).
    pub scope_completeness: ScopeCompleteness,
    /// Aggregate freshness over the tier (weakest edge).
    pub freshness: FreshnessClass,
    /// Aggregate confidence over the tier (weakest edge).
    pub confidence: NavigationConfidence,
    /// The proof classes behind this tier's edges, in canonical order.
    pub proof_classes: Vec<ProofClass>,
    /// Visible labels for the tier.
    pub labels: Vec<HierarchyLabel>,
    /// Downgrade reasons that must stay visible on consumers.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Attribution notes describing inferred/runtime/framework/imported evidence.
    pub attribution_notes: Vec<String>,
    /// Export-safe summary.
    pub summary: String,
}

/// A surface-level projection proving the view survives review, support, AI, graph,
/// and docs consumers without flattening into a single opaque tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HierarchyViewProjection {
    /// The consumer surface.
    pub consumer_surface: ConsumerSurface,
    /// Number of legend tiers projected to this surface.
    pub projected_tier_count: usize,
    /// True when legend grouping is preserved.
    pub preserves_legend_grouping: bool,
    /// True when edge counts are preserved.
    pub preserves_edge_counts: bool,
    /// True when scope completeness and named scope gaps are preserved.
    pub preserves_scope_completeness: bool,
    /// True when freshness and confidence are preserved.
    pub preserves_freshness_and_confidence: bool,
    /// True when the ambiguity/disambiguation state is preserved.
    pub preserves_ambiguity_state: bool,
    /// True when the projection flattens the view into a single opaque tree (must be
    /// false).
    pub flattens_to_single_tree: bool,
    /// True when the projection exports raw code bodies (must be false).
    pub exports_code_bodies: bool,
    /// Redaction class for this projection.
    pub redaction_class: ExportRedactionClass,
    /// Export-safe summary.
    pub summary: String,
}

impl HierarchyViewProjection {
    /// Returns true when the projection preserves the view's typed truth without
    /// flattening or leaking code bodies.
    pub const fn preserves_truth(&self) -> bool {
        self.preserves_legend_grouping
            && self.preserves_edge_counts
            && self.preserves_scope_completeness
            && self.preserves_freshness_and_confidence
            && self.preserves_ambiguity_state
            && !self.flattens_to_single_tree
            && !self.exports_code_bodies
    }
}

/// A hierarchy view: edges grouped by legend, with scope completeness, named scope
/// gaps, freshness/confidence, ambiguity state, stable actions, and consumer
/// projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HierarchyView {
    /// Stable view id.
    pub view_id: String,
    /// The root target ref the hierarchy is anchored on.
    pub root_target_ref: String,
    /// The kind of hierarchy this view presents.
    pub view_kind: HierarchyViewKind,
    /// The direction the view expands.
    pub direction: HierarchyDirection,
    /// The current scope ref the edges were resolved against.
    pub scope_ref: String,
    /// The captured snapshot/trace/pack scope ref, when any edge is captured-only.
    pub captured_scope_ref: Option<String>,
    /// Legend tiers, in canonical order.
    pub tiers: Vec<HierarchyTier>,
    /// Aggregate counts across all tiers.
    pub totals: HierarchyEdgeCounts,
    /// The headline legend for the view.
    pub view_legend: HierarchyEdgeLegend,
    /// Aggregate scope completeness across the view (weakest edge or gap).
    pub scope_completeness: ScopeCompleteness,
    /// Explicitly named hidden or missing scopes.
    pub scope_gaps: Vec<HierarchyScopeGap>,
    /// Aggregate freshness across the view (weakest tier).
    pub freshness: FreshnessClass,
    /// Aggregate confidence across the view (weakest tier).
    pub confidence: NavigationConfidence,
    /// The union of tier labels plus view-level labels.
    pub labels: Vec<HierarchyLabel>,
    /// The view's ambiguity/disambiguation state.
    pub ambiguity: HierarchyAmbiguityState,
    /// The stable open/peek/split/expand/export actions.
    pub actions: Vec<HierarchyActionAffordance>,
    /// Consumer projections proving cross-surface parity.
    pub consumer_projections: Vec<HierarchyViewProjection>,
    /// Downgrade reasons that must stay visible on consumers.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Attribution notes describing the view's weaker-proof evidence and gaps.
    pub attribution_notes: Vec<String>,
    /// Redaction class for review/support/AI export.
    pub redaction_class: ExportRedactionClass,
    /// Export-safe summary.
    pub summary: String,
}

impl HierarchyView {
    /// Returns the tier for a legend, if present.
    pub fn tier(&self, legend: HierarchyEdgeLegend) -> Option<&HierarchyTier> {
        self.tiers.iter().find(|tier| tier.legend == legend)
    }

    /// Returns true when the view has any captured-only edge.
    pub const fn has_captured_scope(&self) -> bool {
        self.totals.captured_scope_count > 0
    }

    /// Returns true when a hierarchy jump must be inspected for ambiguity first.
    pub const fn requires_inspection_before_jump(&self) -> bool {
        self.ambiguity.requires_inspection_before_jump
    }

    /// Returns true when the view must render with a visible caveat.
    pub fn requires_disclosure(&self) -> bool {
        self.scope_completeness.requires_disclosure()
            || self.has_captured_scope()
            || self.requires_inspection_before_jump()
            || self.totals.inferred_count > 0
            || self.totals.runtime_observed_count > 0
            || !self.downgrade_reasons.is_empty()
            || !self.scope_gaps.is_empty()
    }
}

/// The typed input the builder turns into a hierarchy view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HierarchyViewInput {
    /// Stable view id.
    pub view_id: String,
    /// The root target ref the hierarchy is anchored on.
    pub root_target_ref: String,
    /// The kind of hierarchy this view presents.
    pub view_kind: HierarchyViewKind,
    /// The direction the view expands.
    pub direction: HierarchyDirection,
    /// The current scope ref.
    pub scope_ref: String,
    /// The captured snapshot/trace/pack scope ref, if any.
    pub captured_scope_ref: Option<String>,
    /// Redaction class for review/support/AI export.
    pub redaction_class: ExportRedactionClass,
    /// The hierarchy edges to group and tally.
    pub edges: Vec<HierarchyEdge>,
    /// Explicitly named hidden or missing scopes.
    pub scope_gaps: Vec<HierarchyScopeGap>,
    /// The view's root ambiguity class.
    pub ambiguity_class: AmbiguityClass,
    /// Competing root or edge-set refs when the root is ambiguous.
    pub competing_root_refs: Vec<String>,
    /// Disambiguation set ref when the user must choose a root.
    pub disambiguation_set_ref: Option<String>,
    /// Export-safe note explaining any ambiguity.
    pub ambiguity_note: String,
}

// ---------------------------------------------------------------------------
// Builder.
// ---------------------------------------------------------------------------

/// Builds a hierarchy view from a typed input.
///
/// Deterministic: the same input yields the same view. Edges are grouped by legend in
/// [`HIERARCHY_LEGEND_ORDER`]; each tier and the view carry computed counts, scope
/// completeness, freshness, confidence, labels, downgrade reasons, and attribution
/// notes; the ambiguity state, the five stable actions, and the consumer projections
/// are generated so the view proves cross-surface parity. The builder derives all
/// disclosure from the edges and the named scope gaps themselves, so an inferred,
/// runtime, captured-scope, or incomplete-scope edge cannot lose its caveat.
pub fn build_hierarchy_view(input: &HierarchyViewInput) -> HierarchyView {
    let mut tiers = Vec::new();
    for legend in HIERARCHY_LEGEND_ORDER {
        let members: Vec<&HierarchyEdge> = input
            .edges
            .iter()
            .filter(|edge| edge_legend(edge) == legend)
            .collect();
        if members.is_empty() {
            continue;
        }
        tiers.push(build_tier(legend, &members));
    }

    let all: Vec<&HierarchyEdge> = input.edges.iter().collect();
    let mut totals = HierarchyEdgeCounts::default();
    for edge in &all {
        totals.add(edge);
    }

    let view_legend = headline_legend(&all);
    let scope_completeness = view_scope_completeness(&all, &input.scope_gaps);
    let freshness = weakest_freshness(&all);
    let confidence = weakest_confidence(&all);

    let mut labels = labels_for(&all, scope_completeness);
    if input.ambiguity_class.requires_disambiguation() {
        push_label(&mut labels, HierarchyLabel::CompetingRoots);
    }
    if !input.scope_gaps.is_empty() {
        push_label(&mut labels, HierarchyLabel::IncompleteScope);
    }
    labels.sort();

    let mut downgrade_reasons = downgrade_reasons_for(&all);
    for gap in &input.scope_gaps {
        push_unique(&mut downgrade_reasons, gap.reason);
    }
    if input.ambiguity_class.requires_disambiguation() {
        push_unique(&mut downgrade_reasons, DowngradeReason::AmbiguousCandidates);
    }

    let mut attribution_notes = attribution_notes_for(&all);
    for gap in &input.scope_gaps {
        attribution_notes.push(format!(
            "Scope {} is {} and not covered: {}",
            gap.scope_ref,
            scope_completeness_token(gap.completeness),
            gap.note
        ));
    }

    let ambiguity = HierarchyAmbiguityState {
        ambiguity_class: input.ambiguity_class,
        competing_root_refs: input.competing_root_refs.clone(),
        disambiguation_set_ref: input.disambiguation_set_ref.clone(),
        requires_inspection_before_jump: input.ambiguity_class.requires_disambiguation(),
        note: input.ambiguity_note.clone(),
    };
    if ambiguity.requires_inspection_before_jump {
        attribution_notes.push(format!(
            "Root is ambiguous ({}): {} competing candidate(s) must be inspected before a jump.",
            ambiguity_token(ambiguity.ambiguity_class),
            ambiguity.competing_root_refs.len()
        ));
    }

    let actions = HierarchyActionKind::ALL
        .iter()
        .map(|action_kind| {
            build_action(
                *action_kind,
                &input.root_target_ref,
                ambiguity.requires_inspection_before_jump,
            )
        })
        .collect();

    let consumer_projections = REQUIRED_CONSUMER_SURFACES
        .iter()
        .map(|surface| build_projection(*surface, tiers.len(), input.redaction_class))
        .collect();

    let summary = format!(
        "{} for {} ({}): {} edge(s) across {} legend tier(s); {} current, {} captured; \
         legend {}; scope {}; {} named gap(s); ambiguity {}.",
        input.view_kind.label(),
        input.root_target_ref,
        input.direction.as_str(),
        totals.total_count,
        tiers.len(),
        totals.current_scope_count,
        totals.captured_scope_count,
        view_legend.as_str(),
        scope_completeness_token(scope_completeness),
        input.scope_gaps.len(),
        ambiguity_token(input.ambiguity_class),
    );

    HierarchyView {
        view_id: input.view_id.clone(),
        root_target_ref: input.root_target_ref.clone(),
        view_kind: input.view_kind,
        direction: input.direction,
        scope_ref: input.scope_ref.clone(),
        captured_scope_ref: input.captured_scope_ref.clone(),
        tiers,
        totals,
        view_legend,
        scope_completeness,
        scope_gaps: input.scope_gaps.clone(),
        freshness,
        confidence,
        labels,
        ambiguity,
        actions,
        consumer_projections,
        downgrade_reasons,
        attribution_notes,
        redaction_class: input.redaction_class,
        summary,
    }
}

fn build_tier(legend: HierarchyEdgeLegend, members: &[&HierarchyEdge]) -> HierarchyTier {
    let mut counts = HierarchyEdgeCounts::default();
    for edge in members {
        counts.add(edge);
    }
    let scope_completeness = weakest_scope_completeness(members);
    let freshness = weakest_freshness(members);
    let confidence = weakest_confidence(members);
    let proof_classes = proof_classes_for(members);
    let labels = labels_for(members, scope_completeness);
    let downgrade_reasons = downgrade_reasons_for(members);
    let attribution_notes = attribution_notes_for(members);
    let edge_refs = members.iter().map(|edge| edge.edge_id.clone()).collect();
    let summary = format!(
        "{} tier: {} edge(s) ({} current, {} captured, max depth {}); scope {}; freshness {}.",
        legend.as_str(),
        counts.total_count,
        counts.current_scope_count,
        counts.captured_scope_count,
        counts.max_depth,
        scope_completeness_token(scope_completeness),
        freshness_token(freshness),
    );
    HierarchyTier {
        legend,
        edge_refs,
        counts,
        scope_completeness,
        freshness,
        confidence,
        proof_classes,
        labels,
        downgrade_reasons,
        attribution_notes,
        summary,
    }
}

fn build_action(
    action_kind: HierarchyActionKind,
    root_target_ref: &str,
    root_ambiguous: bool,
) -> HierarchyActionAffordance {
    let gated_by_ambiguity = action_kind.navigates() && root_ambiguous;
    HierarchyActionAffordance {
        action_kind,
        history_effect: action_kind.history_effect(),
        available_routes: HierarchyActionRoute::ALL.to_vec(),
        target_ref: root_target_ref.to_owned(),
        preserves_target_identity: true,
        gated_by_ambiguity,
        summary: format!(
            "{} resolves to the same node on every route ({}); history effect {}{}.",
            action_kind.label(),
            HierarchyActionRoute::ALL
                .iter()
                .map(|route| route.as_str())
                .collect::<Vec<_>>()
                .join(", "),
            action_kind.history_effect().as_str(),
            if gated_by_ambiguity {
                "; gated behind ambiguity inspection"
            } else {
                ""
            },
        ),
    }
}

fn build_projection(
    surface: ConsumerSurface,
    tier_count: usize,
    redaction_class: ExportRedactionClass,
) -> HierarchyViewProjection {
    HierarchyViewProjection {
        consumer_surface: surface,
        projected_tier_count: tier_count,
        preserves_legend_grouping: true,
        preserves_edge_counts: true,
        preserves_scope_completeness: true,
        preserves_freshness_and_confidence: true,
        preserves_ambiguity_state: true,
        flattens_to_single_tree: false,
        exports_code_bodies: false,
        redaction_class,
        summary: format!(
            "{} consumes the view with legend grouping, edge counts, scope completeness, \
             freshness/confidence, and ambiguity state preserved; never flattened to a single tree.",
            surface.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Derivations.
// ---------------------------------------------------------------------------

/// Returns true when an edge is carried only by a captured scope — a snapshot,
/// runtime trace, or imported pack — rather than re-proven against current source.
fn is_captured_only(edge: &HierarchyEdge) -> bool {
    matches!(
        edge.proof_class,
        ProofClass::ImportedEvidence | ProofClass::RuntimeObserved
    ) || matches!(
        edge.freshness,
        FreshnessClass::Stale | FreshnessClass::Unverified
    )
}

/// Returns true when a proof class is a lexical/syntax fallback.
fn is_fallback_proof(proof: ProofClass) -> bool {
    matches!(
        proof,
        ProofClass::LexicalFallback | ProofClass::SyntaxFallback
    )
}

fn headline_legend(members: &[&HierarchyEdge]) -> HierarchyEdgeLegend {
    if members.is_empty() {
        return HierarchyEdgeLegend::Empty;
    }
    let mut legends = BTreeSet::new();
    for edge in members {
        legends.insert(edge_legend(edge));
    }
    if legends.len() == 1 {
        legends.into_iter().next().unwrap()
    } else {
        HierarchyEdgeLegend::Mixed
    }
}

fn proof_classes_for(members: &[&HierarchyEdge]) -> Vec<ProofClass> {
    let mut classes = BTreeSet::new();
    for edge in members {
        classes.insert(edge.proof_class);
    }
    classes.into_iter().collect()
}

fn weakest_scope_completeness(members: &[&HierarchyEdge]) -> ScopeCompleteness {
    members
        .iter()
        .map(|edge| edge.scope_completeness)
        .max_by_key(|completeness| scope_completeness_severity(*completeness))
        .unwrap_or(ScopeCompleteness::CompleteForDeclaredScope)
}

fn view_scope_completeness(
    members: &[&HierarchyEdge],
    scope_gaps: &[HierarchyScopeGap],
) -> ScopeCompleteness {
    let mut worst = weakest_scope_completeness(members);
    for gap in scope_gaps {
        if scope_completeness_severity(gap.completeness) > scope_completeness_severity(worst) {
            worst = gap.completeness;
        }
    }
    // A named gap always means the view is at least partial for its declared scope.
    if !scope_gaps.is_empty()
        && scope_completeness_severity(worst)
            < scope_completeness_severity(ScopeCompleteness::PartialForDeclaredScope)
    {
        worst = ScopeCompleteness::PartialForDeclaredScope;
    }
    worst
}

fn weakest_freshness(members: &[&HierarchyEdge]) -> FreshnessClass {
    members
        .iter()
        .map(|edge| edge.freshness)
        .max_by_key(|freshness| freshness_severity(*freshness))
        .unwrap_or(FreshnessClass::AuthoritativeLive)
}

fn weakest_confidence(members: &[&HierarchyEdge]) -> NavigationConfidence {
    members
        .iter()
        .map(|edge| edge.confidence)
        .max_by_key(|confidence| confidence_severity(*confidence))
        .unwrap_or(NavigationConfidence::Exact)
}

fn labels_for(
    members: &[&HierarchyEdge],
    scope_completeness: ScopeCompleteness,
) -> Vec<HierarchyLabel> {
    let mut labels = BTreeSet::new();
    for edge in members {
        match edge_legend(edge) {
            HierarchyEdgeLegend::Transitive => {
                labels.insert(HierarchyLabel::Transitive);
            }
            HierarchyEdgeLegend::Inferred => {
                labels.insert(HierarchyLabel::Inferred);
            }
            HierarchyEdgeLegend::RuntimeObserved => {
                labels.insert(HierarchyLabel::RuntimeObserved);
            }
            HierarchyEdgeLegend::Direct
            | HierarchyEdgeLegend::Mixed
            | HierarchyEdgeLegend::Empty => {}
        }
        match edge.proof_class {
            ProofClass::FrameworkDerived => {
                labels.insert(HierarchyLabel::FrameworkDerived);
            }
            ProofClass::ImportedEvidence => {
                labels.insert(HierarchyLabel::ImportedSnapshot);
            }
            ProofClass::LexicalFallback => {
                labels.insert(HierarchyLabel::LexicalFallback);
            }
            ProofClass::SyntaxFallback => {
                labels.insert(HierarchyLabel::SyntaxFallback);
            }
            _ => {}
        }
        if edge
            .downgrade_reasons
            .contains(&DowngradeReason::GeneratedBoundary)
        {
            labels.insert(HierarchyLabel::Generated);
        }
        if matches!(
            edge.freshness,
            FreshnessClass::Stale | FreshnessClass::Unverified
        ) {
            labels.insert(HierarchyLabel::StaleEvidence);
        }
    }
    if !members.is_empty() && members.iter().all(|edge| is_captured_only(edge)) {
        labels.insert(HierarchyLabel::CapturedScopeOnly);
    }
    if scope_completeness.requires_disclosure() {
        labels.insert(HierarchyLabel::IncompleteScope);
    }
    labels.into_iter().collect()
}

fn downgrade_reasons_for(members: &[&HierarchyEdge]) -> Vec<DowngradeReason> {
    let mut reasons: Vec<DowngradeReason> = Vec::new();
    for edge in members {
        for reason in &edge.downgrade_reasons {
            push_unique(&mut reasons, *reason);
        }
    }
    for edge in members {
        match edge.proof_class {
            ProofClass::LexicalFallback => {
                push_unique(&mut reasons, DowngradeReason::LexicalFallbackOnly);
            }
            ProofClass::SyntaxFallback => {
                push_unique(&mut reasons, DowngradeReason::SyntaxFallbackOnly);
            }
            ProofClass::RuntimeObserved | ProofClass::FrameworkDerived => {
                push_unique(&mut reasons, DowngradeReason::RuntimeOrFrameworkOnly);
            }
            _ => {}
        }
    }
    reasons
}

fn attribution_notes_for(members: &[&HierarchyEdge]) -> Vec<String> {
    let mut notes = Vec::new();
    let count = |predicate: &dyn Fn(&HierarchyEdge) -> bool| {
        members.iter().filter(|edge| predicate(edge)).count()
    };

    let transitive = members
        .iter()
        .filter(|edge| edge_legend(edge) == HierarchyEdgeLegend::Transitive)
        .count();
    if transitive > 0 {
        notes.push(format!(
            "{transitive} edge(s) are transitive structure reached through proven intermediate \
             edges, not direct proof adjacent to the root."
        ));
    }
    let runtime = count(&|edge| edge.proof_class == ProofClass::RuntimeObserved);
    if runtime > 0 {
        notes.push(format!(
            "{runtime} edge(s) are runtime-observed from a captured trace, not static proof, and may \
             miss paths the trace never exercised."
        ));
    }
    let framework = count(&|edge| edge.proof_class == ProofClass::FrameworkDerived);
    if framework > 0 {
        notes.push(format!(
            "{framework} edge(s) are framework-derived from route/generator metadata and are \
             disclosed as inferred, never as direct proof."
        ));
    }
    let imported = count(&|edge| edge.proof_class == ProofClass::ImportedEvidence);
    if imported > 0 {
        notes.push(format!(
            "{imported} edge(s) come from an imported snapshot and are captured-scope only."
        ));
    }
    let lexical = count(&|edge| edge.proof_class == ProofClass::LexicalFallback);
    if lexical > 0 {
        notes.push(format!(
            "{lexical} edge(s) rest on a lexical/grep fallback and are disclosed as such, never shown \
             as semantic certainty."
        ));
    }
    let syntax = count(&|edge| edge.proof_class == ProofClass::SyntaxFallback);
    if syntax > 0 {
        notes.push(format!(
            "{syntax} edge(s) rest on a syntax-only fallback and stay labeled as a fallback."
        ));
    }
    let ai = count(&|edge| edge.proof_class == ProofClass::AiInferred);
    if ai > 0 {
        notes.push(format!(
            "{ai} edge(s) are AI-inferred hypotheses, not authoritative proof."
        ));
    }
    notes
}

fn push_unique(reasons: &mut Vec<DowngradeReason>, reason: DowngradeReason) {
    if !reasons.contains(&reason) {
        reasons.push(reason);
    }
}

fn push_label(labels: &mut Vec<HierarchyLabel>, label: HierarchyLabel) {
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

const fn ambiguity_token(ambiguity: AmbiguityClass) -> &'static str {
    match ambiguity {
        AmbiguityClass::Unambiguous => "unambiguous",
        AmbiguityClass::AmbiguousNeedsSelection => "ambiguous_needs_selection",
        AmbiguityClass::MultipleCandidatesRanked => "multiple_candidates_ranked",
        AmbiguityClass::DriftedNeedsReview => "drifted_needs_review",
        AmbiguityClass::MissingTarget => "missing_target",
        AmbiguityClass::ScopeUnavailable => "scope_unavailable",
    }
}

// ---------------------------------------------------------------------------
// Frozen corpus.
// ---------------------------------------------------------------------------

/// One frozen view scenario: an input, the view the builder produces for it, and the
/// property the scenario proves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HierarchyViewScenario {
    /// Stable scenario id.
    pub scenario_id: String,
    /// Plain-language title.
    pub title: String,
    /// The view-building input.
    pub input: HierarchyViewInput,
    /// The view `build_hierarchy_view` produces for the input.
    pub view: HierarchyView,
    /// One reviewable sentence stating what the scenario proves.
    pub expectation_note: String,
}

/// One frozen invariant over the corpus, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HierarchyViewInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built corpus satisfies the invariant.
    pub holds: bool,
}

/// The frozen hierarchy-views corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HierarchyViewSet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub hierarchy_views_schema_version: u32,
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
    /// The frozen view scenarios.
    pub scenarios: Vec<HierarchyViewScenario>,
    /// The computed invariants.
    pub invariants: Vec<HierarchyViewInvariant>,
    /// Whether raw source bodies and payloads are excluded (always true).
    pub raw_payload_excluded: bool,
}

/// Error returned when the corpus fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HierarchyViewValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for HierarchyViewValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "hierarchy-views corpus invalid: {}", self.reason)
    }
}

impl std::error::Error for HierarchyViewValidationError {}

impl HierarchyViewSet {
    /// Returns the scenario with a given id, if present.
    pub fn scenario(&self, scenario_id: &str) -> Option<&HierarchyViewScenario> {
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
            refs.push(scenario.input.root_target_ref.as_str());
            refs.push(scenario.input.scope_ref.as_str());
            if let Some(captured) = &scenario.input.captured_scope_ref {
                refs.push(captured.as_str());
            }
            for edge in &scenario.input.edges {
                refs.push(edge.source_ref.as_str());
                refs.push(edge.target_ref.as_str());
                refs.extend(
                    edge.runtime_or_framework_evidence_refs
                        .iter()
                        .map(String::as_str),
                );
            }
            for gap in &scenario.input.scope_gaps {
                refs.push(gap.scope_ref.as_str());
            }
            for competing in &scenario.input.competing_root_refs {
                refs.push(competing.as_str());
            }
            if let Some(set_ref) = &scenario.input.disambiguation_set_ref {
                refs.push(set_ref.as_str());
            }
            refs.push(scenario.view.root_target_ref.as_str());
            refs.push(scenario.view.scope_ref.as_str());
            if let Some(captured) = &scenario.view.captured_scope_ref {
                refs.push(captured.as_str());
            }
            for action in &scenario.view.actions {
                refs.push(action.target_ref.as_str());
            }
        }
        refs
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    pub fn validate(&self) -> Result<(), HierarchyViewValidationError> {
        let fail = |reason: String| Err(HierarchyViewValidationError { reason });

        if self.record_kind != HIERARCHY_VIEWS_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != HIERARCHY_VIEWS_SCHEMA_REF {
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

        // Every scenario's stored view equals what the builder produces, so the
        // fixture cannot drift from the builder.
        for scenario in &self.scenarios {
            let produced = build_hierarchy_view(&scenario.input);
            if produced != scenario.view {
                return fail(format!(
                    "scenario {} view drifted from builder output",
                    scenario.scenario_id
                ));
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

/// Builds the canonical hierarchy-views corpus.
///
/// Deterministic: the same bytes every call. Each scenario's view is the builder's
/// own output, and the invariant `holds` flags are computed from those views, so a
/// regression in [`build_hierarchy_view`] flips an invariant or drifts the fixture
/// rather than silently passing.
pub fn hierarchy_views_set() -> HierarchyViewSet {
    let scenarios = build_scenarios();
    let invariants = compute_invariants(&scenarios);

    HierarchyViewSet {
        record_kind: HIERARCHY_VIEWS_RECORD_KIND.to_owned(),
        hierarchy_views_schema_version: HIERARCHY_VIEWS_SCHEMA_VERSION,
        schema_ref: HIERARCHY_VIEWS_SCHEMA_REF.to_owned(),
        set_id: HIERARCHY_VIEWS_SET_ID.to_owned(),
        as_of: HIERARCHY_VIEWS_AS_OF.to_owned(),
        freeze_gate_ref: HIERARCHY_VIEWS_FREEZE_GATE_REF.to_owned(),
        summary: "Frozen hierarchy-views corpus: every call, type, override, and ownership \
                  hierarchy is a typed view that groups edges by a direct/transitive/inferred/\
                  runtime-observed legend, separates current-scope from captured-scope counts, names \
                  every hidden or missing scope explicitly, preserves provider attribution, \
                  freshness, and confidence, exposes the competing roots and a disambiguation path \
                  before a jump when the root is ambiguous, carries stable open/peek/split/expand/\
                  export actions across the hierarchy view, graph overlay, search panel, docs link, \
                  and keyboard routes, and projects to review, support, AI, graph, and docs consumers \
                  without flattening into a single opaque tree or exporting code bodies."
            .to_owned(),
        scenarios,
        invariants,
        raw_payload_excluded: true,
    }
}

/// Renders the corpus as human-readable lines for CLI/headless and support.
pub fn hierarchy_views_lines(set: &HierarchyViewSet) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Hierarchy-views corpus — {} ({})",
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
        let view = &scenario.view;
        lines.push(format!("  - {} [{}]", scenario.scenario_id, scenario.title));
        lines.push(format!(
            "      kind={} dir={} tiers={} total={} current={} captured={} legend={} scope={}",
            view.view_kind.as_str(),
            view.direction.as_str(),
            view.tiers.len(),
            view.totals.total_count,
            view.totals.current_scope_count,
            view.totals.captured_scope_count,
            view.view_legend.as_str(),
            scope_completeness_token(view.scope_completeness),
        ));
        let tier_summary = view
            .tiers
            .iter()
            .map(|tier| format!("{}:{}", tier.legend.as_str(), tier.counts.total_count))
            .collect::<Vec<_>>()
            .join(" ");
        lines.push(format!("      {tier_summary}"));
        lines.push(format!(
            "      ambiguity={} competing={} gaps={}",
            ambiguity_token(view.ambiguity.ambiguity_class),
            view.ambiguity.competing_root_refs.len(),
            view.scope_gaps.len(),
        ));
        if !view.labels.is_empty() {
            lines.push(format!(
                "      labels={}",
                view.labels
                    .iter()
                    .map(|label| label.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            ));
        }
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

/// Compact seed for a candidate [`HierarchyEdge`], so each scenario reads as a small
/// table rather than a wall of struct fields.
struct EdgeSeed {
    edge_id: &'static str,
    kind: HierarchyEdgeKind,
    proof: ProofClass,
    depth: u32,
    confidence: NavigationConfidence,
    freshness: FreshnessClass,
    scope: ScopeCompleteness,
    runtime_or_framework_evidence: &'static [&'static str],
    downgrades: &'static [DowngradeReason],
    summary: &'static str,
}

fn edge(root: &str, seed: EdgeSeed) -> HierarchyEdge {
    HierarchyEdge {
        edge_id: seed.edge_id.to_owned(),
        source_ref: root.to_owned(),
        target_ref: format!("aureline://node/{}", seed.edge_id),
        edge_kind: seed.kind,
        proof_class: seed.proof,
        depth: seed.depth,
        scope_completeness: seed.scope,
        freshness: seed.freshness,
        confidence: seed.confidence,
        runtime_or_framework_evidence_refs: seed
            .runtime_or_framework_evidence
            .iter()
            .map(|r| (*r).to_owned())
            .collect(),
        downgrade_reasons: seed.downgrades.to_vec(),
        summary: seed.summary.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn input(
    view_id: &str,
    root: &str,
    view_kind: HierarchyViewKind,
    direction: HierarchyDirection,
    captured_scope_ref: Option<&str>,
    redaction_class: ExportRedactionClass,
    edges: Vec<HierarchyEdge>,
    scope_gaps: Vec<HierarchyScopeGap>,
    ambiguity_class: AmbiguityClass,
    competing_root_refs: Vec<&str>,
    disambiguation_set_ref: Option<&str>,
    ambiguity_note: &str,
) -> HierarchyViewInput {
    HierarchyViewInput {
        view_id: view_id.to_owned(),
        root_target_ref: format!("aureline://object/{root}"),
        view_kind,
        direction,
        scope_ref: "aureline://scope/workspace".to_owned(),
        captured_scope_ref: captured_scope_ref.map(str::to_owned),
        redaction_class,
        edges,
        scope_gaps,
        ambiguity_class,
        competing_root_refs: competing_root_refs.into_iter().map(str::to_owned).collect(),
        disambiguation_set_ref: disambiguation_set_ref.map(str::to_owned),
        ambiguity_note: ambiguity_note.to_owned(),
    }
}

fn gap(
    scope_ref: &str,
    completeness: ScopeCompleteness,
    reason: DowngradeReason,
    note: &str,
) -> HierarchyScopeGap {
    HierarchyScopeGap {
        scope_ref: scope_ref.to_owned(),
        completeness,
        reason,
        note: note.to_owned(),
    }
}

fn scenario(
    scenario_id: &str,
    title: &str,
    input: HierarchyViewInput,
    expectation_note: &str,
) -> HierarchyViewScenario {
    let view = build_hierarchy_view(&input);
    HierarchyViewScenario {
        scenario_id: scenario_id.to_owned(),
        title: title.to_owned(),
        input,
        view,
        expectation_note: expectation_note.to_owned(),
    }
}

fn build_scenarios() -> Vec<HierarchyViewScenario> {
    use AmbiguityClass::*;
    use ExportRedactionClass::*;
    use FreshnessClass::*;
    use HierarchyEdgeKind::*;
    use NavigationConfidence::*;
    use ProofClass::*;
    use ScopeCompleteness::*;

    vec![
        // 1. A clean call hierarchy: direct callers plus transitive structure.
        scenario(
            "view.call_direct_and_transitive",
            "Call hierarchy distinguishes direct callers from transitive structure",
            input(
                "view:call:0001",
                "symbol.handler",
                HierarchyViewKind::Call,
                HierarchyDirection::Incoming,
                None,
                MetadataSafeDefault,
                vec![
                    edge(
                        "aureline://object/symbol.handler",
                        EdgeSeed {
                            edge_id: "edge.call.direct.1",
                            kind: Calls,
                            proof: DirectSemantic,
                            depth: 1,
                            confidence: Exact,
                            freshness: AuthoritativeLive,
                            scope: CompleteForDeclaredScope,
                            runtime_or_framework_evidence: &[],
                            downgrades: &[],
                            summary: "Router calls the handler directly.",
                        },
                    ),
                    edge(
                        "aureline://object/symbol.handler",
                        EdgeSeed {
                            edge_id: "edge.call.direct.2",
                            kind: Calls,
                            proof: IndexedSemantic,
                            depth: 1,
                            confidence: Indexed,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            runtime_or_framework_evidence: &[],
                            downgrades: &[],
                            summary: "Middleware calls the handler directly.",
                        },
                    ),
                    edge(
                        "aureline://object/symbol.handler",
                        EdgeSeed {
                            edge_id: "edge.call.transitive.1",
                            kind: Calls,
                            proof: IndexedSemantic,
                            depth: 3,
                            confidence: Indexed,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            runtime_or_framework_evidence: &[],
                            downgrades: &[],
                            summary: "Dispatcher reaches the handler through two proven hops.",
                        },
                    ),
                ],
                vec![],
                Unambiguous,
                vec![],
                None,
                "",
            ),
            "Direct callers land in the direct tier and the multi-hop caller lands in the \
             transitive tier, so direct proof is never blended with transitive structure.",
        ),
        // 2. A call hierarchy mixing runtime-observed and framework-inferred edges.
        scenario(
            "view.call_runtime_and_inferred",
            "Call hierarchy names runtime-observed and inferred edges apart from proof",
            input(
                "view:call:0002",
                "symbol.dispatch",
                HierarchyViewKind::Call,
                HierarchyDirection::Outgoing,
                Some("aureline://scope/captured-trace"),
                MetadataSafeDefault,
                vec![
                    edge(
                        "aureline://object/symbol.dispatch",
                        EdgeSeed {
                            edge_id: "edge.call.direct.3",
                            kind: Calls,
                            proof: DirectSemantic,
                            depth: 1,
                            confidence: Exact,
                            freshness: AuthoritativeLive,
                            scope: CompleteForDeclaredScope,
                            runtime_or_framework_evidence: &[],
                            downgrades: &[],
                            summary: "Dispatch directly calls the registered service.",
                        },
                    ),
                    edge(
                        "aureline://object/symbol.dispatch",
                        EdgeSeed {
                            edge_id: "edge.call.runtime.1",
                            kind: RuntimeCalls,
                            proof: RuntimeObserved,
                            depth: 2,
                            confidence: Imported,
                            freshness: WarmCached,
                            scope: PartialForDeclaredScope,
                            runtime_or_framework_evidence: &["aureline://evidence/trace-1"],
                            downgrades: &[DowngradeReason::RuntimeOrFrameworkOnly],
                            summary: "A callee observed only in a captured runtime trace.",
                        },
                    ),
                    edge(
                        "aureline://object/symbol.dispatch",
                        EdgeSeed {
                            edge_id: "edge.call.framework.1",
                            kind: FrameworkBinding,
                            proof: FrameworkDerived,
                            depth: 1,
                            confidence: Imported,
                            freshness: WarmCached,
                            scope: PartialForDeclaredScope,
                            runtime_or_framework_evidence: &["aureline://evidence/route-map-1"],
                            downgrades: &[DowngradeReason::RuntimeOrFrameworkOnly],
                            summary: "A callee bound through a framework route map.",
                        },
                    ),
                ],
                vec![gap(
                    "aureline://scope/untraced-paths",
                    PartialForDeclaredScope,
                    DowngradeReason::RuntimeOrFrameworkOnly,
                    "Callees on paths the captured trace and route map never exercised are not in \
                     this view.",
                )],
                Unambiguous,
                vec![],
                None,
                "",
            ),
            "Runtime-observed and framework-inferred callees stay in their own legend tiers with \
             evidence refs and downgrade reasons, and the untraced remainder is named as a scope \
             gap, so neither poses as direct proof and the partial coverage is explicit.",
        ),
        // 3. A type hierarchy with an incomplete scope named explicitly.
        scenario(
            "view.type_incomplete_scope",
            "Type hierarchy names its hidden and missing scope explicitly",
            input(
                "view:type:0003",
                "symbol.trait",
                HierarchyViewKind::Type,
                HierarchyDirection::Incoming,
                None,
                InternalSupportRestricted,
                vec![
                    edge(
                        "aureline://object/symbol.trait",
                        EdgeSeed {
                            edge_id: "edge.type.impl.1",
                            kind: Implements,
                            proof: DirectSemantic,
                            depth: 1,
                            confidence: Exact,
                            freshness: AuthoritativeLive,
                            scope: CompleteForDeclaredScope,
                            runtime_or_framework_evidence: &[],
                            downgrades: &[],
                            summary: "A workspace type implements the trait.",
                        },
                    ),
                    edge(
                        "aureline://object/symbol.trait",
                        EdgeSeed {
                            edge_id: "edge.type.inherit.transitive.1",
                            kind: Inherits,
                            proof: IndexedSemantic,
                            depth: 2,
                            confidence: Partial,
                            freshness: WarmCached,
                            scope: PartialForDeclaredScope,
                            runtime_or_framework_evidence: &[],
                            downgrades: &[DowngradeReason::SparseWorkset],
                            summary: "A subtype inherits the trait through an intermediate type.",
                        },
                    ),
                ],
                vec![gap(
                    "aureline://scope/external-crate",
                    UnavailableForDeclaredScope,
                    DowngradeReason::SparseWorkset,
                    "Implementors in an unindexed external crate are not in the current workset.",
                )],
                Unambiguous,
                vec![],
                None,
                "",
            ),
            "The type hierarchy reports a partial scope and names the unindexed external crate as a \
             missing scope, so an incomplete hierarchy never reads as complete.",
        ),
        // 4. An override hierarchy whose root is ambiguous between competing candidates.
        scenario(
            "view.override_ambiguous_roots",
            "Override hierarchy exposes competing roots before a jump",
            input(
                "view:override:0004",
                "symbol.method",
                HierarchyViewKind::Override,
                HierarchyDirection::Bidirectional,
                None,
                MetadataSafeDefault,
                vec![
                    edge(
                        "aureline://object/symbol.method",
                        EdgeSeed {
                            edge_id: "edge.override.1",
                            kind: Overrides,
                            proof: DirectSemantic,
                            depth: 1,
                            confidence: Exact,
                            freshness: AuthoritativeLive,
                            scope: CompleteForDeclaredScope,
                            runtime_or_framework_evidence: &[],
                            downgrades: &[],
                            summary: "A subclass overrides the method.",
                        },
                    ),
                    edge(
                        "aureline://object/symbol.method",
                        EdgeSeed {
                            edge_id: "edge.override.2",
                            kind: Overrides,
                            proof: IndexedSemantic,
                            depth: 1,
                            confidence: Indexed,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            runtime_or_framework_evidence: &[],
                            downgrades: &[],
                            summary: "A second subclass overrides the method.",
                        },
                    ),
                ],
                vec![],
                AmbiguousNeedsSelection,
                vec![
                    "aureline://object/symbol.method.base_a",
                    "aureline://object/symbol.method.base_b",
                ],
                Some("aureline://disambiguation/override-roots-1"),
                "Two base declarations define this method; the override root must be chosen.",
            ),
            "When two base declarations compete, the view exposes both competing roots and a \
             disambiguation set and gates the navigating actions, so a jump cannot silently pick a \
             tree before the ambiguity is inspected.",
        ),
        // 5. An ownership hierarchy resting on inferred and imported evidence.
        scenario(
            "view.ownership_inferred_imported",
            "Ownership hierarchy keeps inferred and imported edges disclosed",
            input(
                "view:ownership:0005",
                "symbol.module",
                HierarchyViewKind::Ownership,
                HierarchyDirection::Incoming,
                Some("aureline://scope/imported-pack"),
                SigningEvidenceOnly,
                vec![
                    edge(
                        "aureline://object/symbol.module",
                        EdgeSeed {
                            edge_id: "edge.owner.framework.1",
                            kind: Owner,
                            proof: FrameworkDerived,
                            depth: 1,
                            confidence: Imported,
                            freshness: WarmCached,
                            scope: PartialForDeclaredScope,
                            runtime_or_framework_evidence: &["aureline://evidence/codeowners-1"],
                            downgrades: &[
                                DowngradeReason::RuntimeOrFrameworkOnly,
                                DowngradeReason::GeneratedBoundary,
                            ],
                            summary: "Ownership inferred from a CODEOWNERS-like rule.",
                        },
                    ),
                    edge(
                        "aureline://object/symbol.module",
                        EdgeSeed {
                            edge_id: "edge.owner.imported.1",
                            kind: Owner,
                            proof: ImportedEvidence,
                            depth: 1,
                            confidence: Imported,
                            freshness: DegradedCached,
                            scope: StaleForDeclaredScope,
                            runtime_or_framework_evidence: &[],
                            downgrades: &[DowngradeReason::StaleShard],
                            summary: "Ownership carried by an imported stewardship snapshot.",
                        },
                    ),
                ],
                vec![gap(
                    "aureline://scope/archived-branch",
                    StaleForDeclaredScope,
                    DowngradeReason::StaleShard,
                    "Stewardship on an archived branch is stale and excluded from the live view.",
                )],
                Unambiguous,
                vec![],
                None,
                "",
            ),
            "Inferred (framework) and imported ownership edges stay disclosed with a captured scope \
             ref, an imported/stale label, attribution notes, and a named stale scope gap, so an \
             inferred ownership claim never reads as current direct proof.",
        ),
    ]
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> HierarchyViewInvariant {
    HierarchyViewInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(scenarios: &[HierarchyViewScenario]) -> Vec<HierarchyViewInvariant> {
    let views: Vec<&HierarchyView> = scenarios.iter().map(|s| &s.view).collect();

    let mut out = Vec::new();

    // Legend grouping: every edge appears in exactly one tier keyed by its legend,
    // and tiers are in canonical order.
    out.push(invariant(
        "hierarchy_view.legend_grouping_present",
        "Every view groups its edges by legend in the canonical direct/transitive/inferred/\
         runtime-observed order, places each edge in exactly the tier for its legend, and never \
         flattens the set into a single opaque tree.",
        scenarios.iter().all(|scenario| {
            let view = &scenario.view;
            tiers_in_canonical_order(view)
                && view.tiers.iter().all(|tier| {
                    scenario.input.edges.iter().all(|edge| {
                        if edge_legend(edge) == tier.legend {
                            tier.edge_refs.contains(&edge.edge_id)
                        } else {
                            !tier.edge_refs.contains(&edge.edge_id)
                        }
                    })
                })
                && total_grouped(view) == view.totals.total_count
        }),
    ));

    // Counts reconcile across tiers and the view, and the legend tallies partition.
    out.push(invariant(
        "hierarchy_view.legend_counts_reconcile",
        "Every tier and view reconciles its current-scope and captured-scope counts with its total, \
         the four legend tallies partition that total, and the tier totals sum to the view total, so \
         a direct/transitive/inferred/runtime count is always internally consistent.",
        views.iter().all(|view| {
            view.totals.reconciles()
                && view.totals.legend_partition_reconciles()
                && view.tiers.iter().all(|tier| {
                    tier.counts.reconciles() && tier.counts.legend_partition_reconciles()
                })
                && view
                    .tiers
                    .iter()
                    .map(|tier| tier.counts.total_count)
                    .sum::<usize>()
                    == view.totals.total_count
                && view
                    .tiers
                    .iter()
                    .map(|tier| tier.counts.captured_scope_count)
                    .sum::<usize>()
                    == view.totals.captured_scope_count
        }),
    ));

    // Direct proof is distinguished from transitive/inferred/runtime edges.
    out.push(invariant(
        "hierarchy_view.direct_distinguished_from_derived",
        "Every edge in a tier resolves to that tier's legend: the direct tier holds only direct/\
         indexed semantic edges adjacent to the root, the transitive tier only deeper semantic \
         edges, the inferred tier only framework/import/AI/lexical/syntax edges, and the \
         runtime-observed tier only runtime edges, so direct proof is never confused with derived \
         structure.",
        scenarios.iter().all(|scenario| {
            scenario.view.tiers.iter().all(|tier| {
                tier.edge_refs.iter().all(|edge_ref| {
                    scenario
                        .input
                        .edges
                        .iter()
                        .find(|edge| &edge.edge_id == edge_ref)
                        .is_some_and(|edge| edge_legend(edge) == tier.legend)
                })
            })
        }),
    ));

    // Inferred and runtime tiers are always disclosed and carry evidence.
    out.push(invariant(
        "hierarchy_view.inferred_and_runtime_disclosed",
        "Every inferred or runtime-observed tier carries attribution notes and downgrade reasons, \
         and every runtime or framework edge preserves its runtime/framework evidence refs, so an \
         inferred guess or a runtime observation never appears as direct proof.",
        scenarios.iter().all(|scenario| {
            let tiers_ok = scenario.view.tiers.iter().all(|tier| {
                if matches!(
                    tier.legend,
                    HierarchyEdgeLegend::Inferred | HierarchyEdgeLegend::RuntimeObserved
                ) {
                    !tier.attribution_notes.is_empty() && !tier.downgrade_reasons.is_empty()
                } else {
                    true
                }
            });
            let edges_ok = scenario.input.edges.iter().all(|edge| {
                if matches!(
                    edge.proof_class,
                    ProofClass::RuntimeObserved | ProofClass::FrameworkDerived
                ) {
                    !edge.runtime_or_framework_evidence_refs.is_empty()
                } else {
                    true
                }
            });
            tiers_ok && edges_ok
        }),
    ));

    // Missing or hidden scope is always named explicitly.
    out.push(invariant(
        "hierarchy_view.missing_scope_named",
        "Whenever a view covers an incomplete scope it names at least one hidden or missing scope \
         gap and carries an incomplete-scope label and a downgrade reason, so a partial hierarchy \
         never reads as a complete one.",
        views.iter().all(|view| {
            if view.scope_completeness.requires_disclosure() {
                !view.scope_gaps.is_empty()
                    && view.labels.contains(&HierarchyLabel::IncompleteScope)
                    && !view.downgrade_reasons.is_empty()
            } else {
                true
            }
        }),
    ));

    // Captured-scope divergence is always disclosed.
    out.push(invariant(
        "hierarchy_view.captured_scope_disclosed",
        "Whenever a view has captured-scope edges it carries a captured scope ref or a downgrade \
         reason, an imported/runtime/captured label, and attribution notes, so current-versus-\
         captured divergence is never hidden.",
        views.iter().all(|view| {
            if view.totals.captured_scope_count == 0 {
                true
            } else {
                (view.captured_scope_ref.is_some() || !view.downgrade_reasons.is_empty())
                    && view.labels.iter().any(|label| {
                        matches!(
                            label,
                            HierarchyLabel::CapturedScopeOnly
                                | HierarchyLabel::ImportedSnapshot
                                | HierarchyLabel::RuntimeObserved
                                | HierarchyLabel::StaleEvidence
                        )
                    })
                    && !view.attribution_notes.is_empty()
            }
        }),
    ));

    // Ambiguity is inspectable before a navigating jump.
    out.push(invariant(
        "hierarchy_view.ambiguity_inspectable_before_jump",
        "Whenever a view's root is ambiguous it exposes competing roots or a disambiguation set, \
         requires inspection before a jump, and gates its navigating actions, so a hierarchy jump \
         cannot silently change context before the ambiguity is inspected.",
        views.iter().all(|view| {
            if view.ambiguity.ambiguity_class.requires_disambiguation() {
                view.ambiguity.has_disambiguation_path()
                    && view.ambiguity.requires_inspection_before_jump
                    && view.labels.contains(&HierarchyLabel::CompetingRoots)
                    && view
                        .actions
                        .iter()
                        .filter(|action| action.action_kind.navigates())
                        .all(|action| action.gated_by_ambiguity)
            } else {
                view.actions.iter().all(|action| !action.gated_by_ambiguity)
            }
        }),
    ));

    // Actions are stable across every route.
    out.push(invariant(
        "hierarchy_view.actions_stable_across_routes",
        "Every view exposes the five open/peek/split/expand/export actions, each reachable from the \
         hierarchy view, graph overlay, search panel, docs link, and keyboard routes, each with one \
         stable history effect and preserved target identity, so an action behaves identically on \
         every surface.",
        views.iter().all(|view| {
            HierarchyActionKind::ALL.iter().all(|action_kind| {
                view.actions
                    .iter()
                    .filter(|a| a.action_kind == *action_kind)
                    .count()
                    == 1
                    && view.actions.iter().any(|a| {
                        a.action_kind == *action_kind
                            && a.history_effect == action_kind.history_effect()
                            && a.preserves_target_identity
                            && a.target_ref == view.root_target_ref
                            && routes_match(&a.available_routes)
                    })
            })
        }),
    ));

    // Consumers preserve the typed truth without flattening.
    out.push(invariant(
        "hierarchy_view.consumers_preserve_truth",
        "Every consumer projection preserves legend grouping, edge counts, scope completeness, \
         freshness/confidence, and ambiguity state, never flattens the view into a single opaque \
         tree, and never exports raw code bodies, so review, support, AI, graph, and docs consumers \
         see typed hierarchy edges rather than one tree snapshot.",
        views.iter().all(|view| {
            !view.consumer_projections.is_empty()
                && view
                    .consumer_projections
                    .iter()
                    .all(HierarchyViewProjection::preserves_truth)
                && required_surfaces_covered(&view.consumer_projections)
        }),
    ));

    // Every edge kind matches the view kind.
    out.push(invariant(
        "hierarchy_view.edges_match_view_kind",
        "Every edge in a view is admitted by the view kind, so a call view never shows inheritance \
         edges and an ownership view never shows call edges.",
        scenarios.iter().all(|scenario| {
            scenario
                .input
                .edges
                .iter()
                .all(|edge| scenario.input.view_kind.admits(edge.edge_kind))
        }),
    ));

    // The corpus covers every view kind, legend, action, and the ambiguity, gap, and
    // captured-scope answers.
    out.push(invariant(
        "hierarchy_view.corpus_covers_vocabulary",
        "The corpus exercises every call/type/override/ownership view kind, every \
         direct/transitive/inferred/runtime-observed legend, every open/peek/split/expand/export \
         action, and the ambiguous-root, named-scope-gap, and captured-scope answers, so the view \
         model is proven across its whole vocabulary.",
        HierarchyViewKind::ALL
            .iter()
            .all(|kind| views.iter().any(|view| view.view_kind == *kind))
            && HIERARCHY_LEGEND_ORDER
                .iter()
                .all(|legend| views.iter().any(|view| view.tier(*legend).is_some()))
            && every_action_covered(&views)
            && views
                .iter()
                .any(|view| view.ambiguity.ambiguity_class.requires_disambiguation())
            && views.iter().any(|view| !view.scope_gaps.is_empty())
            && views
                .iter()
                .any(|view| view.totals.captured_scope_count > 0),
    ));

    // The view is replayable and answers the support question.
    out.push(invariant(
        "hierarchy_view.replayable_support_answer",
        "Every view carries a non-empty id and summary, a named view kind, direction, and headline \
         legend, so a support or debug packet can state which hierarchy was navigated and whether \
         its edges were direct, transitive, inferred, or runtime-observed.",
        views.iter().all(|view| {
            !view.view_id.trim().is_empty()
                && !view.summary.trim().is_empty()
                && view.view_legend != HierarchyEdgeLegend::Empty
        }),
    ));

    out
}

// ---------------------------------------------------------------------------
// Invariant helpers.
// ---------------------------------------------------------------------------

fn tiers_in_canonical_order(view: &HierarchyView) -> bool {
    let order = |legend: HierarchyEdgeLegend| {
        HIERARCHY_LEGEND_ORDER
            .iter()
            .position(|candidate| *candidate == legend)
            .unwrap_or(usize::MAX)
    };
    view.tiers
        .windows(2)
        .all(|pair| order(pair[0].legend) < order(pair[1].legend))
}

fn total_grouped(view: &HierarchyView) -> usize {
    view.tiers.iter().map(|tier| tier.edge_refs.len()).sum()
}

fn routes_match(routes: &[HierarchyActionRoute]) -> bool {
    routes.len() == HierarchyActionRoute::ALL.len()
        && HierarchyActionRoute::ALL
            .iter()
            .all(|route| routes.contains(route))
}

fn every_action_covered(views: &[&HierarchyView]) -> bool {
    HierarchyActionKind::ALL.iter().all(|action_kind| {
        views.iter().any(|view| {
            view.actions
                .iter()
                .any(|action| action.action_kind == *action_kind)
        })
    })
}

fn required_surfaces_covered(projections: &[HierarchyViewProjection]) -> bool {
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
