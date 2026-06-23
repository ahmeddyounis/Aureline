//! References panes: the typed pane and export model over reference occurrences.
//!
//! The [`target_model`](crate::target_model) freezes the
//! [`reference occurrence`](crate::target_model::ReferenceOccurrence) object — one
//! member of a find-references or evidence set, with its access kind, proof class,
//! freshness, scope completeness, and authorship posture. The
//! [`relation-navigation matrix`](crate::m5_relation_navigation) names that object
//! family and pins its vocabulary. What was still implicit is the *pane and export
//! model*: how Aureline turns a flat list of occurrences into a references pane
//! that shows **what kind** of usage it found (access-kind grouping), **from what
//! scope** (current-versus-captured counts), and **how strong the proof really is**
//! (a per-group and per-pane evidence class), and how that typed truth survives
//! into review evidence, support exports, and AI/graph consumers instead of being
//! flattened into one undifferentiated hit list.
//!
//! This module is that model. [`build_reference_pane`] is a pure function over a
//! typed [`ReferencePaneInput`] that produces a [`ReferencePane`]:
//!
//! 1. **Access-kind grouping.** Occurrences are grouped into [`ReferenceGroup`]s
//!    keyed by [`AccessKind`](crate::target_model::AccessKind) in a canonical order
//!    — read, write, call, inherit, import, export, test-only, generated — so a
//!    write is never silently counted as a read and a test-only reference is never
//!    hidden inside a production count.
//! 2. **Current-versus-captured scope counts.** Each group and the pane carry a
//!    [`ReferenceScopeCounts`] separating occurrences proven against the current
//!    scope from those carried only by a captured snapshot, runtime trace, or
//!    imported pack, plus generated/external/test-only/fallback tallies.
//! 3. **Evidence honesty.** Each group and the pane carry a
//!    [`ReferenceEvidenceClass`] — semantic, framework-derived, runtime-observed,
//!    imported-snapshot, lexical fallback, syntax fallback, or mixed — and any
//!    group resting on a lexical/grep fallback carries a fallback note and a
//!    downgrade reason, so a grep fallback never masquerades as semantic certainty.
//! 4. **Stable actions.** Each pane exposes the same four [`PaneActionKind`]s —
//!    open, peek, split-open, export — on every [`ActionRoute`] (references pane,
//!    search panel, docs link, keyboard route) with one stable
//!    [`HistoryEffect`] per action, so open/peek/split/export preserve the same
//!    target and history semantics no matter which surface invokes them.
//! 5. **Consumer parity.** Each pane projects to every
//!    [`ConsumerSurface`](crate::target_model::ConsumerSurface) with a
//!    [`ReferencePaneProjection`] that preserves access-kind grouping, scope
//!    counts, evidence class, and generated/external/test labels, never flattens to
//!    generic hits, and never exports raw code bodies.
//!
//! [`reference_panes_set`] freezes a deterministic corpus of panes whose
//! [`ReferencePaneInvariant`] flags are computed from the builder's own output, so
//! the checked-in fixture and the freeze gate pin the contract byte-for-byte and
//! any regression in [`build_reference_pane`] flips an invariant or drifts the
//! fixture rather than silently passing. The records carry no source bodies, raw
//! paths, provider payloads, URLs, hostnames, or credentials — only opaque object
//! handles, stable tokens, and short reviewable sentences — so they are safe for
//! support export.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::target_model::{
    AccessKind, ConsumerSurface, DowngradeReason, ExportRedactionClass, FreshnessClass,
    GeneratedOrExternalState, NavigationConfidence, ProofClass, ReferenceOccurrence, RelationKind,
    ScopeCompleteness, REQUIRED_CONSUMER_SURFACES,
};

#[cfg(test)]
mod tests;

/// Schema version for the references-pane corpus.
pub const REFERENCE_PANES_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the references-pane corpus.
pub const REFERENCE_PANES_SCHEMA_REF: &str = "schemas/navigation/reference_panes.schema.json";

/// Stable record-kind tag for the references-pane corpus.
pub const REFERENCE_PANES_RECORD_KIND: &str = "reference_panes_set";

/// Stable id for the canonical references-pane corpus.
pub const REFERENCE_PANES_SET_ID: &str = "reference-panes:set:0001";

/// Evaluation stamp for the canonical corpus. Held as a constant so the canonical
/// binding stays deterministic and the fixture freezes byte-for-byte.
pub const REFERENCE_PANES_AS_OF: &str = "2026-06-23T00:00:00Z";

/// The freeze gate that keeps the corpus binding current. Stable promotion runs
/// this gate; it fails when the in-code corpus drifts from the checked-in fixture
/// or any invariant flips.
pub const REFERENCE_PANES_FREEZE_GATE_REF: &str =
    "crates/aureline-navigation/tests/reference_panes.rs";

/// Reviewer doc for the references-pane contract.
pub const REFERENCE_PANES_DOC_REF: &str = "docs/navigation/reference_panes.md";

/// Evidence companion for the references-pane corpus.
pub const REFERENCE_PANES_ARTIFACT_REF: &str = "artifacts/navigation/reference_panes.md";

/// Repo-relative path of the checked-in canonical corpus.
pub const REFERENCE_PANES_FIXTURE_REF: &str =
    "fixtures/navigation/reference_panes/canonical_panes.json";

/// The canonical access-kind grouping order for references panes.
///
/// A references pane lists groups in this order so a write is never counted as a
/// read, a test-only reference is never folded into a production count, and a
/// generated occurrence is never hidden inside authored usage.
pub const REFERENCE_ACCESS_KIND_ORDER: [AccessKind; 8] = [
    AccessKind::Read,
    AccessKind::Write,
    AccessKind::Call,
    AccessKind::Inherit,
    AccessKind::Import,
    AccessKind::Export,
    AccessKind::TestOnly,
    AccessKind::Generated,
];

// ---------------------------------------------------------------------------
// Actions.
// ---------------------------------------------------------------------------

/// A stable action a references pane, search panel, docs link, or keyboard route
/// can invoke on a reference target.
///
/// The action set is closed and identical across every [`ActionRoute`]: open,
/// peek, split-open, and export. Each action has one stable [`HistoryEffect`], so a
/// peek never advances navigation history and an export never mutates the editor,
/// no matter which surface invoked it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaneActionKind {
    /// Open the target in the active editor, replacing the current view.
    Open,
    /// Peek the target inline without leaving the current editor.
    Peek,
    /// Open the target in a split, leaving the current editor in place.
    SplitOpen,
    /// Export the metadata-only reference set; never mutates the editor.
    Export,
}

impl PaneActionKind {
    /// All actions, in canonical order.
    pub const ALL: [Self; 4] = [Self::Open, Self::Peek, Self::SplitOpen, Self::Export];

    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Peek => "peek",
            Self::SplitOpen => "split_open",
            Self::Export => "export",
        }
    }

    /// Returns a human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Open => "Open",
            Self::Peek => "Peek",
            Self::SplitOpen => "Open to the Side",
            Self::Export => "Export References",
        }
    }

    /// Returns the stable history effect this action has on navigation history.
    pub const fn history_effect(self) -> HistoryEffect {
        match self {
            Self::Open | Self::SplitOpen => HistoryEffect::AdvancesHistory,
            Self::Peek => HistoryEffect::PreservesCurrent,
            Self::Export => HistoryEffect::NoEditorHistory,
        }
    }
}

/// The effect an action has on navigation history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryEffect {
    /// Pushes a new navigation history entry (open, split-open).
    AdvancesHistory,
    /// Leaves navigation history untouched (peek).
    PreservesCurrent,
    /// Touches no editor history at all (export).
    NoEditorHistory,
}

impl HistoryEffect {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdvancesHistory => "advances_history",
            Self::PreservesCurrent => "preserves_current",
            Self::NoEditorHistory => "no_editor_history",
        }
    }
}

/// A surface route that exposes the references-pane actions.
///
/// The same actions are reachable from every route, so open/peek/split/export
/// behave identically whether they were invoked from the references pane, a search
/// panel, a docs link, or a keyboard shortcut.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionRoute {
    /// The dedicated references pane.
    ReferencesPane,
    /// A search results panel.
    SearchPanel,
    /// A documentation or help link.
    DocsLink,
    /// A keyboard shortcut route.
    KeyboardShortcut,
}

impl ActionRoute {
    /// All routes, in canonical order.
    pub const ALL: [Self; 4] = [
        Self::ReferencesPane,
        Self::SearchPanel,
        Self::DocsLink,
        Self::KeyboardShortcut,
    ];

    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReferencesPane => "references_pane",
            Self::SearchPanel => "search_panel",
            Self::DocsLink => "docs_link",
            Self::KeyboardShortcut => "keyboard_shortcut",
        }
    }
}

/// A stable action bound to a references pane.
///
/// The affordance records the action kind, its stable history effect, the routes
/// it is reachable from (always the full canonical set), and the target ref the
/// action acts on — so a support packet can confirm an action keeps the same target
/// and history semantics across every surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneActionAffordance {
    /// The action kind.
    pub action_kind: PaneActionKind,
    /// The stable history effect for this action.
    pub history_effect: HistoryEffect,
    /// Routes the action is reachable from, in canonical order.
    pub available_routes: Vec<ActionRoute>,
    /// The target ref the action acts on.
    pub target_ref: String,
    /// Always true: the action resolves to the same target across every route.
    pub preserves_target_identity: bool,
    /// Export-safe summary.
    pub summary: String,
}

// ---------------------------------------------------------------------------
// Evidence and labels.
// ---------------------------------------------------------------------------

/// The evidence class for a reference group or pane.
///
/// Answers the support/debug question "was this reference set semantic,
/// framework-derived, runtime-observed, imported, or a lexical fallback?". A group
/// whose members rest on more than one evidence class resolves to [`Mixed`].
///
/// [`Mixed`]: ReferenceEvidenceClass::Mixed
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceEvidenceClass {
    /// Direct or indexed semantic evidence over the declared scope.
    Semantic,
    /// Framework pack, route metadata, or generator evidence.
    FrameworkDerived,
    /// Runtime trace, debugger, or observed-dispatch evidence.
    RuntimeObserved,
    /// Imported snapshot, docs pack, or provider overlay evidence.
    ImportedSnapshot,
    /// Lexical or grep fallback evidence.
    LexicalFallback,
    /// Syntax-tree-only fallback evidence.
    SyntaxFallback,
    /// The group mixes more than one evidence class.
    Mixed,
    /// No admissible evidence for the group.
    Unavailable,
}

impl ReferenceEvidenceClass {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Semantic => "semantic",
            Self::FrameworkDerived => "framework_derived",
            Self::RuntimeObserved => "runtime_observed",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::LexicalFallback => "lexical_fallback",
            Self::SyntaxFallback => "syntax_fallback",
            Self::Mixed => "mixed",
            Self::Unavailable => "unavailable",
        }
    }

    /// Returns true when this evidence class must render with a visible caveat
    /// rather than as plain semantic certainty.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Semantic)
    }

    /// Returns true when this evidence class rests on a lexical/syntax fallback.
    pub const fn is_fallback(self) -> bool {
        matches!(self, Self::LexicalFallback | Self::SyntaxFallback)
    }
}

/// A user-visible label a references pane attaches to a group or to the whole pane.
///
/// Labels keep generated, external, read-only, imported, test-only, fallback,
/// runtime, framework, and captured-scope occurrences visible instead of folding
/// them into an undifferentiated production count.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceLabel {
    /// The group or pane includes generated or paired-artifact occurrences.
    Generated,
    /// The group or pane includes external-dependency occurrences.
    External,
    /// The group or pane includes read-only or protected occurrences.
    ReadOnly,
    /// The group or pane includes imported-snapshot occurrences.
    ImportedSnapshot,
    /// The group or pane includes test-only occurrences.
    TestOnly,
    /// The group or pane includes lexical/grep fallback occurrences.
    LexicalFallback,
    /// The group or pane includes syntax-fallback occurrences.
    SyntaxFallback,
    /// The group or pane includes runtime-observed occurrences.
    RuntimeObserved,
    /// The group or pane includes framework-derived occurrences.
    FrameworkDerived,
    /// Every occurrence in the group is carried only by a captured scope, not the
    /// current scope.
    CapturedScopeOnly,
}

impl ReferenceLabel {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Generated => "generated",
            Self::External => "external",
            Self::ReadOnly => "read_only",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::TestOnly => "test_only",
            Self::LexicalFallback => "lexical_fallback",
            Self::SyntaxFallback => "syntax_fallback",
            Self::RuntimeObserved => "runtime_observed",
            Self::FrameworkDerived => "framework_derived",
            Self::CapturedScopeOnly => "captured_scope_only",
        }
    }
}

// ---------------------------------------------------------------------------
// Counts.
// ---------------------------------------------------------------------------

/// Current-versus-captured scope counts for a reference group or pane.
///
/// `current_scope_count` tallies occurrences proven against the current source or
/// index; `captured_scope_count` tallies occurrences carried only by a captured
/// snapshot, runtime trace, or imported pack. The two always sum to `total_count`.
/// The remaining tallies keep generated, external, test-only, fallback, runtime,
/// and framework occurrences individually visible.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceScopeCounts {
    /// Total occurrences in the group or pane.
    pub total_count: usize,
    /// Occurrences proven against the current scope.
    pub current_scope_count: usize,
    /// Occurrences carried only by a captured snapshot, trace, or imported pack.
    pub captured_scope_count: usize,
    /// Generated or paired-artifact occurrences.
    pub generated_count: usize,
    /// External-dependency occurrences.
    pub external_count: usize,
    /// Test-only occurrences.
    pub test_only_count: usize,
    /// Occurrences resting on a lexical or syntax fallback.
    pub fallback_count: usize,
    /// Runtime-observed occurrences.
    pub runtime_observed_count: usize,
    /// Framework-derived occurrences.
    pub framework_derived_count: usize,
}

impl ReferenceScopeCounts {
    /// Returns true when the current and captured counts reconcile with the total.
    pub const fn reconciles(&self) -> bool {
        self.current_scope_count + self.captured_scope_count == self.total_count
    }

    fn add(&mut self, occurrence: &ReferenceOccurrence) {
        self.total_count += 1;
        if is_captured_only(occurrence) {
            self.captured_scope_count += 1;
        } else {
            self.current_scope_count += 1;
        }
        if is_generated(occurrence) {
            self.generated_count += 1;
        }
        if occurrence.generated_or_external_state == GeneratedOrExternalState::ExternalDependency {
            self.external_count += 1;
        }
        if occurrence.access_kind == AccessKind::TestOnly {
            self.test_only_count += 1;
        }
        if is_fallback_proof(occurrence.proof_class) {
            self.fallback_count += 1;
        }
        if occurrence.proof_class == ProofClass::RuntimeObserved {
            self.runtime_observed_count += 1;
        }
        if occurrence.proof_class == ProofClass::FrameworkDerived {
            self.framework_derived_count += 1;
        }
    }
}

// ---------------------------------------------------------------------------
// Group, projection, pane.
// ---------------------------------------------------------------------------

/// One access-kind group inside a references pane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceGroup {
    /// The access kind this group represents.
    pub access_kind: AccessKind,
    /// Occurrence ids in this group, in pane order.
    pub occurrence_refs: Vec<String>,
    /// Current-versus-captured scope counts for the group.
    pub counts: ReferenceScopeCounts,
    /// The evidence class for the group.
    pub evidence_class: ReferenceEvidenceClass,
    /// Visible labels for the group.
    pub labels: Vec<ReferenceLabel>,
    /// Downgrade reasons that must stay visible on consumers.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Fallback notes describing lexical/syntax/runtime/framework/imported evidence.
    pub fallback_notes: Vec<String>,
    /// Export-safe summary.
    pub summary: String,
}

/// A surface-level projection proving the pane survives review, support, AI, and
/// graph consumers without flattening into generic search hits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferencePaneProjection {
    /// The consumer surface.
    pub consumer_surface: ConsumerSurface,
    /// Number of access-kind groups projected to this surface.
    pub projected_group_count: usize,
    /// True when access-kind grouping is preserved.
    pub preserves_access_kind_grouping: bool,
    /// True when current-versus-captured scope counts are preserved.
    pub preserves_scope_counts: bool,
    /// True when the evidence class is preserved.
    pub preserves_evidence_class: bool,
    /// True when generated/external/test-only labels are preserved.
    pub preserves_generated_external_test_labels: bool,
    /// True when the projection flattens the set into generic hits (must be false).
    pub flattens_to_generic_hits: bool,
    /// True when the projection exports raw code bodies (must be false).
    pub exports_code_bodies: bool,
    /// Redaction class for this projection.
    pub redaction_class: ExportRedactionClass,
    /// Export-safe summary.
    pub summary: String,
}

impl ReferencePaneProjection {
    /// Returns true when the projection preserves the pane's typed truth without
    /// flattening or leaking code bodies.
    pub const fn preserves_truth(&self) -> bool {
        self.preserves_access_kind_grouping
            && self.preserves_scope_counts
            && self.preserves_evidence_class
            && self.preserves_generated_external_test_labels
            && !self.flattens_to_generic_hits
            && !self.exports_code_bodies
    }
}

/// A references pane: occurrences grouped by access kind, with scope counts,
/// evidence class, labels, stable actions, and consumer projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferencePane {
    /// Stable pane id.
    pub pane_id: String,
    /// The root target ref whose references this pane lists.
    pub root_target_ref: String,
    /// The relation kind the pane lists (always [`RelationKind::Reference`]).
    pub root_relation_kind: RelationKind,
    /// The current scope ref the references were resolved against.
    pub scope_ref: String,
    /// The captured snapshot/trace/pack scope ref, when any occurrence is
    /// captured-only.
    pub captured_scope_ref: Option<String>,
    /// Access-kind groups, in canonical order.
    pub groups: Vec<ReferenceGroup>,
    /// Aggregate scope counts across all groups.
    pub totals: ReferenceScopeCounts,
    /// The aggregate evidence class for the pane.
    pub pane_evidence_class: ReferenceEvidenceClass,
    /// The union of group labels.
    pub labels: Vec<ReferenceLabel>,
    /// The stable open/peek/split/export actions.
    pub actions: Vec<PaneActionAffordance>,
    /// Consumer projections proving cross-surface parity.
    pub consumer_projections: Vec<ReferencePaneProjection>,
    /// Downgrade reasons that must stay visible on consumers.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Fallback notes describing the pane's weakest evidence.
    pub fallback_notes: Vec<String>,
    /// Redaction class for review/support/AI export.
    pub redaction_class: ExportRedactionClass,
    /// Export-safe summary.
    pub summary: String,
}

impl ReferencePane {
    /// Returns the group for an access kind, if present.
    pub fn group(&self, access_kind: AccessKind) -> Option<&ReferenceGroup> {
        self.groups
            .iter()
            .find(|group| group.access_kind == access_kind)
    }

    /// Returns true when the pane has any captured-only occurrence.
    pub const fn has_captured_scope(&self) -> bool {
        self.totals.captured_scope_count > 0
    }

    /// Returns true when the pane must render with a visible caveat.
    pub fn requires_disclosure(&self) -> bool {
        self.pane_evidence_class.requires_disclosure()
            || self.has_captured_scope()
            || !self.downgrade_reasons.is_empty()
            || !self.labels.is_empty()
    }
}

/// The typed input the builder turns into a references pane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferencePaneInput {
    /// Stable pane id.
    pub pane_id: String,
    /// The root target ref whose references this pane lists.
    pub root_target_ref: String,
    /// The current scope ref.
    pub scope_ref: String,
    /// The captured snapshot/trace/pack scope ref, if any.
    pub captured_scope_ref: Option<String>,
    /// Redaction class for review/support/AI export.
    pub redaction_class: ExportRedactionClass,
    /// The reference occurrences to group and tally.
    pub occurrences: Vec<ReferenceOccurrence>,
}

// ---------------------------------------------------------------------------
// Builder.
// ---------------------------------------------------------------------------

/// Builds a references pane from a typed input.
///
/// Deterministic: the same input yields the same pane. Occurrences are grouped by
/// access kind in [`REFERENCE_ACCESS_KIND_ORDER`]; each group and the pane carry
/// computed scope counts, an evidence class, labels, downgrade reasons, and
/// fallback notes; the four stable actions and the consumer projections are
/// generated so the pane proves cross-surface parity. The builder derives all
/// disclosure from the occurrences themselves, so a fallback or captured-scope
/// occurrence cannot lose its caveat.
pub fn build_reference_pane(input: &ReferencePaneInput) -> ReferencePane {
    let mut groups = Vec::new();
    for access_kind in REFERENCE_ACCESS_KIND_ORDER {
        let members: Vec<&ReferenceOccurrence> = input
            .occurrences
            .iter()
            .filter(|occurrence| occurrence.access_kind == access_kind)
            .collect();
        if members.is_empty() {
            continue;
        }
        groups.push(build_group(access_kind, &members));
    }

    let all: Vec<&ReferenceOccurrence> = input.occurrences.iter().collect();
    let mut totals = ReferenceScopeCounts::default();
    for occurrence in &all {
        totals.add(occurrence);
    }

    let pane_evidence_class = evidence_class_for(&all);
    let labels = labels_for(&all);
    let downgrade_reasons = downgrade_reasons_for(&all);
    let fallback_notes = fallback_notes_for(&all);

    let actions = PaneActionKind::ALL
        .iter()
        .map(|action_kind| build_action(*action_kind, &input.root_target_ref))
        .collect();

    let consumer_projections = REQUIRED_CONSUMER_SURFACES
        .iter()
        .map(|surface| build_projection(*surface, groups.len(), input.redaction_class))
        .collect();

    let summary = format!(
        "References pane for {} target: {} occurrence(s) across {} access-kind group(s); \
         {} current, {} captured-scope; evidence {}.",
        input.root_target_ref,
        totals.total_count,
        groups.len(),
        totals.current_scope_count,
        totals.captured_scope_count,
        pane_evidence_class.as_str(),
    );

    ReferencePane {
        pane_id: input.pane_id.clone(),
        root_target_ref: input.root_target_ref.clone(),
        root_relation_kind: RelationKind::Reference,
        scope_ref: input.scope_ref.clone(),
        captured_scope_ref: input.captured_scope_ref.clone(),
        groups,
        totals,
        pane_evidence_class,
        labels,
        actions,
        consumer_projections,
        downgrade_reasons,
        fallback_notes,
        redaction_class: input.redaction_class,
        summary,
    }
}

fn build_group(access_kind: AccessKind, members: &[&ReferenceOccurrence]) -> ReferenceGroup {
    let mut counts = ReferenceScopeCounts::default();
    for occurrence in members {
        counts.add(occurrence);
    }
    let evidence_class = evidence_class_for(members);
    let labels = labels_for(members);
    let downgrade_reasons = downgrade_reasons_for(members);
    let fallback_notes = fallback_notes_for(members);
    let occurrence_refs = members
        .iter()
        .map(|occurrence| occurrence.occurrence_id.clone())
        .collect();
    let summary = format!(
        "{} group: {} occurrence(s) ({} current, {} captured); evidence {}.",
        access_kind.as_str(),
        counts.total_count,
        counts.current_scope_count,
        counts.captured_scope_count,
        evidence_class.as_str(),
    );
    ReferenceGroup {
        access_kind,
        occurrence_refs,
        counts,
        evidence_class,
        labels,
        downgrade_reasons,
        fallback_notes,
        summary,
    }
}

fn build_action(action_kind: PaneActionKind, root_target_ref: &str) -> PaneActionAffordance {
    PaneActionAffordance {
        action_kind,
        history_effect: action_kind.history_effect(),
        available_routes: ActionRoute::ALL.to_vec(),
        target_ref: root_target_ref.to_owned(),
        preserves_target_identity: true,
        summary: format!(
            "{} resolves to the same target on every route ({}); history effect {}.",
            action_kind.label(),
            ActionRoute::ALL
                .iter()
                .map(|route| route.as_str())
                .collect::<Vec<_>>()
                .join(", "),
            action_kind.history_effect().as_str(),
        ),
    }
}

fn build_projection(
    surface: ConsumerSurface,
    group_count: usize,
    redaction_class: ExportRedactionClass,
) -> ReferencePaneProjection {
    ReferencePaneProjection {
        consumer_surface: surface,
        projected_group_count: group_count,
        preserves_access_kind_grouping: true,
        preserves_scope_counts: true,
        preserves_evidence_class: true,
        preserves_generated_external_test_labels: true,
        flattens_to_generic_hits: false,
        exports_code_bodies: false,
        redaction_class,
        summary: format!(
            "{} consumes the pane with access-kind grouping, scope counts, evidence class, and \
             generated/external/test labels preserved; never flattened to generic hits.",
            surface.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Derivations.
// ---------------------------------------------------------------------------

/// Returns true when an occurrence is carried only by a captured scope — a snapshot,
/// runtime trace, or imported pack — rather than re-proven against current source.
fn is_captured_only(occurrence: &ReferenceOccurrence) -> bool {
    matches!(
        occurrence.proof_class,
        ProofClass::ImportedEvidence | ProofClass::RuntimeObserved
    ) || occurrence.generated_or_external_state == GeneratedOrExternalState::ImportedSnapshot
        || matches!(
            occurrence.freshness,
            FreshnessClass::Stale | FreshnessClass::Unverified
        )
}

/// Returns true when an occurrence is generated or a paired artifact.
fn is_generated(occurrence: &ReferenceOccurrence) -> bool {
    occurrence.access_kind == AccessKind::Generated
        || occurrence.generated_or_external_state == GeneratedOrExternalState::GeneratedSource
}

/// Returns true when a proof class is a lexical/syntax fallback.
fn is_fallback_proof(proof: ProofClass) -> bool {
    matches!(
        proof,
        ProofClass::LexicalFallback | ProofClass::SyntaxFallback
    )
}

fn evidence_class_of_proof(proof: ProofClass) -> ReferenceEvidenceClass {
    match proof {
        ProofClass::DirectSemantic | ProofClass::IndexedSemantic => {
            ReferenceEvidenceClass::Semantic
        }
        ProofClass::FrameworkDerived => ReferenceEvidenceClass::FrameworkDerived,
        ProofClass::RuntimeObserved => ReferenceEvidenceClass::RuntimeObserved,
        ProofClass::ImportedEvidence => ReferenceEvidenceClass::ImportedSnapshot,
        ProofClass::LexicalFallback => ReferenceEvidenceClass::LexicalFallback,
        ProofClass::SyntaxFallback => ReferenceEvidenceClass::SyntaxFallback,
        ProofClass::AiInferred | ProofClass::Unavailable => ReferenceEvidenceClass::Unavailable,
    }
}

fn evidence_class_for(members: &[&ReferenceOccurrence]) -> ReferenceEvidenceClass {
    if members.is_empty() {
        return ReferenceEvidenceClass::Unavailable;
    }
    let mut classes = BTreeSet::new();
    for occurrence in members {
        classes.insert(evidence_class_of_proof(occurrence.proof_class));
    }
    if classes.len() == 1 {
        classes.into_iter().next().unwrap()
    } else {
        ReferenceEvidenceClass::Mixed
    }
}

fn labels_for(members: &[&ReferenceOccurrence]) -> Vec<ReferenceLabel> {
    let mut labels = BTreeSet::new();
    for occurrence in members {
        if is_generated(occurrence) {
            labels.insert(ReferenceLabel::Generated);
        }
        match occurrence.generated_or_external_state {
            GeneratedOrExternalState::ExternalDependency => {
                labels.insert(ReferenceLabel::External);
            }
            GeneratedOrExternalState::ReadOnlySource => {
                labels.insert(ReferenceLabel::ReadOnly);
            }
            GeneratedOrExternalState::ImportedSnapshot => {
                labels.insert(ReferenceLabel::ImportedSnapshot);
            }
            _ => {}
        }
        if occurrence.access_kind == AccessKind::TestOnly {
            labels.insert(ReferenceLabel::TestOnly);
        }
        match occurrence.proof_class {
            ProofClass::LexicalFallback => {
                labels.insert(ReferenceLabel::LexicalFallback);
            }
            ProofClass::SyntaxFallback => {
                labels.insert(ReferenceLabel::SyntaxFallback);
            }
            ProofClass::RuntimeObserved => {
                labels.insert(ReferenceLabel::RuntimeObserved);
            }
            ProofClass::FrameworkDerived => {
                labels.insert(ReferenceLabel::FrameworkDerived);
            }
            _ => {}
        }
    }
    if !members.is_empty()
        && members
            .iter()
            .all(|occurrence| is_captured_only(occurrence))
    {
        labels.insert(ReferenceLabel::CapturedScopeOnly);
    }
    labels.into_iter().collect()
}

fn downgrade_reasons_for(members: &[&ReferenceOccurrence]) -> Vec<DowngradeReason> {
    let mut reasons: Vec<DowngradeReason> = Vec::new();
    for occurrence in members {
        for reason in &occurrence.downgrade_reasons {
            push_unique(&mut reasons, *reason);
        }
    }
    for occurrence in members {
        match occurrence.proof_class {
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
        if is_generated(occurrence) {
            push_unique(&mut reasons, DowngradeReason::GeneratedBoundary);
        }
    }
    reasons
}

fn fallback_notes_for(members: &[&ReferenceOccurrence]) -> Vec<String> {
    let mut notes = Vec::new();
    let count = |predicate: &dyn Fn(&ReferenceOccurrence) -> bool| {
        members
            .iter()
            .filter(|occurrence| predicate(occurrence))
            .count()
    };

    let lexical = count(&|occurrence| occurrence.proof_class == ProofClass::LexicalFallback);
    if lexical > 0 {
        notes.push(format!(
            "{lexical} occurrence(s) rest on a lexical/grep fallback and are disclosed as such, \
             never shown as semantic certainty."
        ));
    }
    let syntax = count(&|occurrence| occurrence.proof_class == ProofClass::SyntaxFallback);
    if syntax > 0 {
        notes.push(format!(
            "{syntax} occurrence(s) rest on a syntax-only fallback and stay labeled as a fallback."
        ));
    }
    let runtime = count(&|occurrence| occurrence.proof_class == ProofClass::RuntimeObserved);
    if runtime > 0 {
        notes.push(format!(
            "{runtime} occurrence(s) are runtime-observed from a captured trace, not static source."
        ));
    }
    let framework = count(&|occurrence| occurrence.proof_class == ProofClass::FrameworkDerived);
    if framework > 0 {
        notes.push(format!(
            "{framework} occurrence(s) are framework-derived from route/generator metadata."
        ));
    }
    let imported = count(&|occurrence| occurrence.proof_class == ProofClass::ImportedEvidence);
    if imported > 0 {
        notes.push(format!(
            "{imported} occurrence(s) come from an imported snapshot and are captured-scope only."
        ));
    }
    let captured = count(&|occurrence| is_captured_only(occurrence));
    if captured > 0 {
        notes.push(format!(
            "{captured} occurrence(s) are carried only by a captured scope and are not re-proven \
             against current source."
        ));
    }
    notes
}

fn push_unique(reasons: &mut Vec<DowngradeReason>, reason: DowngradeReason) {
    if !reasons.contains(&reason) {
        reasons.push(reason);
    }
}

// ---------------------------------------------------------------------------
// Frozen corpus.
// ---------------------------------------------------------------------------

/// One frozen pane scenario: an input, the pane the builder produces for it, and
/// the property the scenario proves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferencePaneScenario {
    /// Stable scenario id.
    pub scenario_id: String,
    /// Plain-language title.
    pub title: String,
    /// The pane-building input.
    pub input: ReferencePaneInput,
    /// The pane `build_reference_pane` produces for the input.
    pub pane: ReferencePane,
    /// One reviewable sentence stating what the scenario proves.
    pub expectation_note: String,
}

/// One frozen invariant over the corpus, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferencePaneInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built corpus satisfies the invariant.
    pub holds: bool,
}

/// The frozen references-pane corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferencePaneSet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub reference_panes_schema_version: u32,
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
    /// The frozen pane scenarios.
    pub scenarios: Vec<ReferencePaneScenario>,
    /// The computed invariants.
    pub invariants: Vec<ReferencePaneInvariant>,
    /// Whether raw source bodies and payloads are excluded (always true).
    pub raw_payload_excluded: bool,
}

/// Error returned when the corpus fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferencePaneValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for ReferencePaneValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "references-pane corpus invalid: {}", self.reason)
    }
}

impl std::error::Error for ReferencePaneValidationError {}

impl ReferencePaneSet {
    /// Returns the scenario with a given id, if present.
    pub fn scenario(&self, scenario_id: &str) -> Option<&ReferencePaneScenario> {
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
            for occurrence in &scenario.input.occurrences {
                refs.push(occurrence.target_ref.as_str());
                refs.push(occurrence.anchor_ref.as_str());
                refs.push(occurrence.scope_ref.as_str());
                refs.extend(occurrence.evidence_refs.iter().map(String::as_str));
            }
            refs.push(scenario.pane.root_target_ref.as_str());
            refs.push(scenario.pane.scope_ref.as_str());
            if let Some(captured) = &scenario.pane.captured_scope_ref {
                refs.push(captured.as_str());
            }
            for action in &scenario.pane.actions {
                refs.push(action.target_ref.as_str());
            }
        }
        refs
    }

    /// Re-checks structural consistency and returns an error on the first failure.
    pub fn validate(&self) -> Result<(), ReferencePaneValidationError> {
        let fail = |reason: String| Err(ReferencePaneValidationError { reason });

        if self.record_kind != REFERENCE_PANES_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != REFERENCE_PANES_SCHEMA_REF {
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

        // Every scenario's stored pane equals what the builder produces, so the
        // fixture cannot drift from the builder.
        for scenario in &self.scenarios {
            let produced = build_reference_pane(&scenario.input);
            if produced != scenario.pane {
                return fail(format!(
                    "scenario {} pane drifted from builder output",
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

/// Builds the canonical references-pane corpus.
///
/// Deterministic: the same bytes every call. Each scenario's pane is the builder's
/// own output, and the invariant `holds` flags are computed from those panes, so a
/// regression in [`build_reference_pane`] flips an invariant or drifts the fixture
/// rather than silently passing.
pub fn reference_panes_set() -> ReferencePaneSet {
    let scenarios = build_scenarios();
    let invariants = compute_invariants(&scenarios);

    ReferencePaneSet {
        record_kind: REFERENCE_PANES_RECORD_KIND.to_owned(),
        reference_panes_schema_version: REFERENCE_PANES_SCHEMA_VERSION,
        schema_ref: REFERENCE_PANES_SCHEMA_REF.to_owned(),
        set_id: REFERENCE_PANES_SET_ID.to_owned(),
        as_of: REFERENCE_PANES_AS_OF.to_owned(),
        freeze_gate_ref: REFERENCE_PANES_FREEZE_GATE_REF.to_owned(),
        summary: "Frozen references-pane corpus: every Find References result is a typed pane that \
                  groups occurrences by access kind, separates current-scope from captured-scope \
                  counts, names whether the evidence is semantic, framework-derived, runtime-observed, \
                  imported, or a lexical fallback, keeps generated/external/test-only labels visible, \
                  exposes stable open/peek/split/export actions identically across the references \
                  pane, search panel, docs links, and keyboard routes, and projects to review, \
                  support, AI, and graph consumers without flattening into generic search hits or \
                  exporting code bodies."
            .to_owned(),
        scenarios,
        invariants,
        raw_payload_excluded: true,
    }
}

/// Renders the corpus as human-readable lines for CLI/headless and support.
pub fn reference_panes_lines(set: &ReferencePaneSet) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "References-pane corpus — {} ({})",
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
        let pane = &scenario.pane;
        lines.push(format!("  - {} [{}]", scenario.scenario_id, scenario.title));
        lines.push(format!(
            "      groups={} total={} current={} captured={} evidence={}",
            pane.groups.len(),
            pane.totals.total_count,
            pane.totals.current_scope_count,
            pane.totals.captured_scope_count,
            pane.pane_evidence_class.as_str(),
        ));
        let group_summary = pane
            .groups
            .iter()
            .map(|group| {
                format!(
                    "{}:{}({})",
                    group.access_kind.as_str(),
                    group.counts.total_count,
                    group.evidence_class.as_str()
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        lines.push(format!("      {group_summary}"));
        if !pane.labels.is_empty() {
            lines.push(format!(
                "      labels={}",
                pane.labels
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

/// Compact seed for a candidate [`ReferenceOccurrence`], so each scenario reads as
/// a small table rather than a wall of struct fields.
struct OccSeed {
    occurrence_id: &'static str,
    access: AccessKind,
    proof: ProofClass,
    confidence: NavigationConfidence,
    freshness: FreshnessClass,
    scope: ScopeCompleteness,
    generated: GeneratedOrExternalState,
    downgrades: &'static [DowngradeReason],
    summary: &'static str,
}

fn occurrence(root: &str, seed: OccSeed) -> ReferenceOccurrence {
    ReferenceOccurrence {
        occurrence_id: seed.occurrence_id.to_owned(),
        target_ref: root.to_owned(),
        anchor_ref: format!("aureline://anchor/{}", seed.occurrence_id),
        access_kind: seed.access,
        scope_ref: "aureline://scope/workspace".to_owned(),
        generated_or_external_state: seed.generated,
        proof_class: seed.proof,
        confidence: seed.confidence,
        freshness: seed.freshness,
        scope_completeness: seed.scope,
        downgrade_reasons: seed.downgrades.to_vec(),
        evidence_refs: vec![format!("aureline://evidence/{}", seed.occurrence_id)],
        summary: seed.summary.to_owned(),
    }
}

fn input(
    pane_id: &str,
    root: &str,
    captured_scope_ref: Option<&str>,
    redaction_class: ExportRedactionClass,
    occurrences: Vec<ReferenceOccurrence>,
) -> ReferencePaneInput {
    ReferencePaneInput {
        pane_id: pane_id.to_owned(),
        root_target_ref: format!("aureline://object/{root}"),
        scope_ref: "aureline://scope/workspace".to_owned(),
        captured_scope_ref: captured_scope_ref.map(str::to_owned),
        redaction_class,
        occurrences,
    }
}

fn scenario(
    scenario_id: &str,
    title: &str,
    input: ReferencePaneInput,
    expectation_note: &str,
) -> ReferencePaneScenario {
    let pane = build_reference_pane(&input);
    ReferencePaneScenario {
        scenario_id: scenario_id.to_owned(),
        title: title.to_owned(),
        input,
        pane,
        expectation_note: expectation_note.to_owned(),
    }
}

fn build_scenarios() -> Vec<ReferencePaneScenario> {
    use AccessKind::*;
    use ExportRedactionClass::*;
    use FreshnessClass::*;
    use NavigationConfidence::*;
    use ProofClass::*;
    use ScopeCompleteness::*;

    let authored = GeneratedOrExternalState::AuthoredSource;

    vec![
        // 1. A clean semantic pane with read, write, and call groups.
        scenario(
            "pane.semantic_read_write_call",
            "Semantic references pane groups read, write, and call",
            input(
                "pane:handler:0001",
                "symbol.handler",
                None,
                MetadataSafeDefault,
                vec![
                    occurrence(
                        "aureline://object/symbol.handler",
                        OccSeed {
                            occurrence_id: "occ.handler.read.1",
                            access: Read,
                            proof: DirectSemantic,
                            confidence: Exact,
                            freshness: AuthoritativeLive,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            downgrades: &[],
                            summary: "Reads the handler in the router.",
                        },
                    ),
                    occurrence(
                        "aureline://object/symbol.handler",
                        OccSeed {
                            occurrence_id: "occ.handler.read.2",
                            access: Read,
                            proof: IndexedSemantic,
                            confidence: Indexed,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            downgrades: &[],
                            summary: "Reads the handler in middleware.",
                        },
                    ),
                    occurrence(
                        "aureline://object/symbol.handler",
                        OccSeed {
                            occurrence_id: "occ.handler.write.1",
                            access: Write,
                            proof: DirectSemantic,
                            confidence: Exact,
                            freshness: AuthoritativeLive,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            downgrades: &[],
                            summary: "Rebinds the handler during setup.",
                        },
                    ),
                    occurrence(
                        "aureline://object/symbol.handler",
                        OccSeed {
                            occurrence_id: "occ.handler.call.1",
                            access: Call,
                            proof: DirectSemantic,
                            confidence: Exact,
                            freshness: AuthoritativeLive,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            downgrades: &[],
                            summary: "Invokes the handler from the dispatcher.",
                        },
                    ),
                ],
            ),
            "Read, write, and call occurrences land in distinct access-kind groups with semantic \
             evidence and no fallback, so a write is never counted as a read.",
        ),
        // 2. Generated, test-only, and external labels stay visible.
        scenario(
            "pane.generated_test_external_labels",
            "Generated, test-only, and external occurrences keep their labels",
            input(
                "pane:config:0002",
                "symbol.config",
                None,
                MetadataSafeDefault,
                vec![
                    occurrence(
                        "aureline://object/symbol.config",
                        OccSeed {
                            occurrence_id: "occ.config.read.1",
                            access: Read,
                            proof: DirectSemantic,
                            confidence: Exact,
                            freshness: AuthoritativeLive,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            downgrades: &[],
                            summary: "Reads config in the app entrypoint.",
                        },
                    ),
                    occurrence(
                        "aureline://object/symbol.config",
                        OccSeed {
                            occurrence_id: "occ.config.test.1",
                            access: TestOnly,
                            proof: IndexedSemantic,
                            confidence: Indexed,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            downgrades: &[],
                            summary: "Uses config inside an integration test.",
                        },
                    ),
                    occurrence(
                        "aureline://object/symbol.config",
                        OccSeed {
                            occurrence_id: "occ.config.gen.1",
                            access: Generated,
                            proof: FrameworkDerived,
                            confidence: Imported,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            generated: GeneratedOrExternalState::GeneratedSource,
                            downgrades: &[],
                            summary: "Generated config accessor in a build artifact.",
                        },
                    ),
                    occurrence(
                        "aureline://object/symbol.config",
                        OccSeed {
                            occurrence_id: "occ.config.import.1",
                            access: Import,
                            proof: IndexedSemantic,
                            confidence: Indexed,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            generated: GeneratedOrExternalState::ExternalDependency,
                            downgrades: &[],
                            summary: "Imported by an external dependency shim.",
                        },
                    ),
                ],
            ),
            "Generated, test-only, and external occurrences keep their labels and counts rather than \
             being folded into one undifferentiated production count.",
        ),
        // 3. Current-versus-captured scope counts with runtime and imported evidence.
        scenario(
            "pane.current_versus_captured",
            "Current and captured-scope occurrences are counted apart",
            input(
                "pane:service:0003",
                "symbol.service",
                Some("aureline://scope/captured-trace"),
                MetadataSafeDefault,
                vec![
                    occurrence(
                        "aureline://object/symbol.service",
                        OccSeed {
                            occurrence_id: "occ.service.read.current",
                            access: Read,
                            proof: DirectSemantic,
                            confidence: Exact,
                            freshness: AuthoritativeLive,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            downgrades: &[],
                            summary: "Reads the service from current source.",
                        },
                    ),
                    occurrence(
                        "aureline://object/symbol.service",
                        OccSeed {
                            occurrence_id: "occ.service.call.runtime",
                            access: Call,
                            proof: RuntimeObserved,
                            confidence: Imported,
                            freshness: WarmCached,
                            scope: PartialForDeclaredScope,
                            generated: authored,
                            downgrades: &[DowngradeReason::RuntimeOrFrameworkOnly],
                            summary: "Calls the service observed in a captured trace.",
                        },
                    ),
                    occurrence(
                        "aureline://object/symbol.service",
                        OccSeed {
                            occurrence_id: "occ.service.import.imported",
                            access: Import,
                            proof: ImportedEvidence,
                            confidence: Imported,
                            freshness: DegradedCached,
                            scope: PartialForDeclaredScope,
                            generated: GeneratedOrExternalState::ImportedSnapshot,
                            downgrades: &[DowngradeReason::StaleShard],
                            summary: "Imports the service from an imported snapshot.",
                        },
                    ),
                ],
            ),
            "Current-scope occurrences are counted apart from captured-scope (runtime trace and \
             imported snapshot) ones, with a captured scope ref and runtime/imported evidence \
             disclosed, so the pane never claims captured counts as current.",
        ),
        // 4. A lexical/grep fallback is disclosed, never shown as semantic certainty.
        scenario(
            "pane.lexical_fallback_disclosed",
            "Lexical and syntax fallbacks stay disclosed",
            input(
                "pane:macro:0004",
                "symbol.macro_target",
                None,
                MetadataSafeDefault,
                vec![
                    occurrence(
                        "aureline://object/symbol.macro_target",
                        OccSeed {
                            occurrence_id: "occ.macro.read.lexical",
                            access: Read,
                            proof: LexicalFallback,
                            confidence: Heuristic,
                            freshness: WarmCached,
                            scope: PartialForDeclaredScope,
                            generated: authored,
                            downgrades: &[DowngradeReason::LexicalFallbackOnly],
                            summary: "Lexical match for a macro-defined symbol.",
                        },
                    ),
                    occurrence(
                        "aureline://object/symbol.macro_target",
                        OccSeed {
                            occurrence_id: "occ.macro.write.syntax",
                            access: Write,
                            proof: SyntaxFallback,
                            confidence: Partial,
                            freshness: WarmCached,
                            scope: PartialForDeclaredScope,
                            generated: authored,
                            downgrades: &[DowngradeReason::SyntaxFallbackOnly],
                            summary: "Syntax-tree match for a macro-assigned symbol.",
                        },
                    ),
                ],
            ),
            "Grep and syntax fallbacks carry a fallback note, a downgrade reason, and a fallback \
             evidence class, so a lexical match never masquerades as semantic certainty.",
        ),
        // 5. Inherit, import, and export groups with framework evidence.
        scenario(
            "pane.inherit_import_export_framework",
            "Inherit, import, and export occurrences keep their access kinds",
            input(
                "pane:trait:0005",
                "symbol.trait",
                None,
                InternalSupportRestricted,
                vec![
                    occurrence(
                        "aureline://object/symbol.trait",
                        OccSeed {
                            occurrence_id: "occ.trait.inherit.1",
                            access: Inherit,
                            proof: IndexedSemantic,
                            confidence: Indexed,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            downgrades: &[],
                            summary: "A type inherits the trait.",
                        },
                    ),
                    occurrence(
                        "aureline://object/symbol.trait",
                        OccSeed {
                            occurrence_id: "occ.trait.import.1",
                            access: Import,
                            proof: DirectSemantic,
                            confidence: Exact,
                            freshness: AuthoritativeLive,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            downgrades: &[],
                            summary: "The trait is imported by a consumer module.",
                        },
                    ),
                    occurrence(
                        "aureline://object/symbol.trait",
                        OccSeed {
                            occurrence_id: "occ.trait.export.1",
                            access: Export,
                            proof: FrameworkDerived,
                            confidence: Imported,
                            freshness: WarmCached,
                            scope: CompleteForDeclaredScope,
                            generated: authored,
                            downgrades: &[DowngradeReason::RuntimeOrFrameworkOnly],
                            summary: "The trait is re-exported via a framework binding.",
                        },
                    ),
                ],
            ),
            "Inherit, import, and export occurrences keep distinct access-kind groups, and the \
             framework-derived export is named as framework evidence rather than plain semantic.",
        ),
    ]
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> ReferencePaneInvariant {
    ReferencePaneInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(scenarios: &[ReferencePaneScenario]) -> Vec<ReferencePaneInvariant> {
    let panes: Vec<&ReferencePane> = scenarios.iter().map(|s| &s.pane).collect();

    let mut out = Vec::new();

    // Access-kind grouping: every occurrence appears in exactly one group keyed by
    // its access kind, and groups are in canonical order.
    out.push(invariant(
        "reference_pane.access_kind_grouping_present",
        "Every pane groups its occurrences by access kind in the canonical read/write/call/inherit/\
         import/export/test-only/generated order, places each occurrence in exactly the group for \
         its access kind, and never flattens the set into one undifferentiated hit list.",
        panes.iter().all(|pane| {
            groups_in_canonical_order(pane)
                && pane.groups.iter().all(|group| {
                    scenario_occurrences_for(scenarios, pane).iter().all(|occ| {
                        if occ.access_kind == group.access_kind {
                            group.occurrence_refs.contains(&occ.occurrence_id)
                        } else {
                            !group.occurrence_refs.contains(&occ.occurrence_id)
                        }
                    })
                })
                && total_grouped(pane) == pane.totals.total_count
        }),
    ));

    // Scope counts reconcile across groups and the pane.
    out.push(invariant(
        "reference_pane.scope_counts_reconcile",
        "Every group and pane reconciles its current-scope and captured-scope counts with its \
         total, and the group totals sum to the pane total, so a current-versus-captured count is \
         always internally consistent.",
        panes.iter().all(|pane| {
            pane.totals.reconciles()
                && pane.groups.iter().all(|group| group.counts.reconciles())
                && pane
                    .groups
                    .iter()
                    .map(|group| group.counts.total_count)
                    .sum::<usize>()
                    == pane.totals.total_count
                && pane
                    .groups
                    .iter()
                    .map(|group| group.counts.captured_scope_count)
                    .sum::<usize>()
                    == pane.totals.captured_scope_count
        }),
    ));

    // Evidence class is disclosed, and fallbacks never masquerade as semantic.
    out.push(invariant(
        "reference_pane.evidence_class_disclosed_no_grep_as_semantic",
        "Every pane and group names its evidence class (semantic, framework-derived, \
         runtime-observed, imported, lexical, syntax, or mixed), and any group resting on a lexical \
         or syntax fallback carries a fallback note and a downgrade reason, so a grep fallback is \
         never shown as semantic certainty.",
        panes.iter().all(|pane| {
            pane.groups.iter().all(|group| {
                !group.evidence_class.is_fallback()
                    || (!group.fallback_notes.is_empty() && !group.downgrade_reasons.is_empty())
            }) && (!pane.pane_evidence_class.is_fallback() || !pane.fallback_notes.is_empty())
        }),
    ));

    // Generated/external/test-only/imported occurrences keep a visible label.
    out.push(invariant(
        "reference_pane.generated_external_test_labels_visible",
        "Every generated, external, read-only, imported, or test-only occurrence contributes a \
         visible label to its group and to the pane, so non-authored and test-only usage is never \
         hidden inside a production count.",
        scenarios.iter().all(|scenario| {
            let pane = &scenario.pane;
            scenario.input.occurrences.iter().all(|occ| {
                let expected = expected_labels_for(occ);
                expected.iter().all(|label| pane.labels.contains(label))
                    && pane.group(occ.access_kind).is_some_and(|group| {
                        expected.iter().all(|label| group.labels.contains(label))
                    })
            })
        }),
    ));

    // Captured-scope divergence is always disclosed.
    out.push(invariant(
        "reference_pane.captured_scope_disclosed",
        "Whenever a pane has captured-scope occurrences it carries a captured scope ref or a \
         downgrade reason and a captured/imported/runtime label, so current-versus-captured \
         divergence is never hidden.",
        panes.iter().all(|pane| {
            if pane.totals.captured_scope_count == 0 {
                true
            } else {
                (pane.captured_scope_ref.is_some() || !pane.downgrade_reasons.is_empty())
                    && pane.labels.iter().any(|label| {
                        matches!(
                            label,
                            ReferenceLabel::CapturedScopeOnly
                                | ReferenceLabel::ImportedSnapshot
                                | ReferenceLabel::RuntimeObserved
                        )
                    })
                    && !pane.fallback_notes.is_empty()
            }
        }),
    ));

    // Actions are stable across every route.
    out.push(invariant(
        "reference_pane.actions_stable_across_routes",
        "Every pane exposes the four open/peek/split/export actions, each reachable from the \
         references pane, search panel, docs link, and keyboard routes, each with one stable history \
         effect and preserved target identity, so an action behaves identically on every surface.",
        panes.iter().all(|pane| {
            PaneActionKind::ALL.iter().all(|action_kind| {
                pane.actions.iter().filter(|a| a.action_kind == *action_kind).count() == 1
                    && pane.actions.iter().any(|a| {
                        a.action_kind == *action_kind
                            && a.history_effect == action_kind.history_effect()
                            && a.preserves_target_identity
                            && a.target_ref == pane.root_target_ref
                            && routes_match(&a.available_routes)
                    })
            })
        }),
    ));

    // History semantics: open/split advance, peek preserves, export touches none.
    out.push(invariant(
        "reference_pane.history_semantics_stable",
        "Open and split-open always advance navigation history, peek always preserves the current \
         location, and export always touches no editor history, identically on every pane.",
        panes.iter().all(|pane| {
            pane.actions
                .iter()
                .all(|action| action.history_effect == action.action_kind.history_effect())
        }),
    ));

    // Consumers preserve the typed truth without flattening.
    out.push(invariant(
        "reference_pane.consumers_preserve_truth",
        "Every consumer projection preserves access-kind grouping, scope counts, evidence class, and \
         generated/external/test labels, never flattens the set into generic hits, and never exports \
         raw code bodies, so review, support, AI, and graph consumers see typed references rather \
         than search hits.",
        panes.iter().all(|pane| {
            !pane.consumer_projections.is_empty()
                && pane
                    .consumer_projections
                    .iter()
                    .all(ReferencePaneProjection::preserves_truth)
                && required_surfaces_covered(&pane.consumer_projections)
        }),
    ));

    // The corpus covers every access kind, action, and evidence answer.
    out.push(invariant(
        "reference_pane.corpus_covers_vocabulary",
        "The corpus exercises every access kind, every open/peek/split/export action, and the \
         semantic, framework-derived, runtime-observed, imported, and lexical evidence answers, so \
         the pane model is proven across its whole vocabulary.",
        REFERENCE_ACCESS_KIND_ORDER
            .iter()
            .all(|access_kind| panes.iter().any(|pane| pane.group(*access_kind).is_some()))
            && every_action_covered(&panes)
            && every_evidence_answer_covered(&panes),
    ));

    // The pane is replayable and answers the support question.
    out.push(invariant(
        "reference_pane.replayable_support_answer",
        "Every pane carries a non-empty id and summary, a named pane evidence class, and a relation \
         kind of reference, so a support or debug packet can state whether the reference set was \
         semantic, framework-derived, runtime-observed, imported, or a lexical fallback.",
        panes.iter().all(|pane| {
            !pane.pane_id.trim().is_empty()
                && !pane.summary.trim().is_empty()
                && pane.root_relation_kind == RelationKind::Reference
        }),
    ));

    out
}

// ---------------------------------------------------------------------------
// Invariant helpers.
// ---------------------------------------------------------------------------

fn groups_in_canonical_order(pane: &ReferencePane) -> bool {
    let order = |access_kind: AccessKind| {
        REFERENCE_ACCESS_KIND_ORDER
            .iter()
            .position(|candidate| *candidate == access_kind)
            .unwrap_or(usize::MAX)
    };
    pane.groups
        .windows(2)
        .all(|pair| order(pair[0].access_kind) < order(pair[1].access_kind))
}

fn total_grouped(pane: &ReferencePane) -> usize {
    pane.groups
        .iter()
        .map(|group| group.occurrence_refs.len())
        .sum()
}

fn scenario_occurrences_for<'a>(
    scenarios: &'a [ReferencePaneScenario],
    pane: &ReferencePane,
) -> &'a [ReferenceOccurrence] {
    scenarios
        .iter()
        .find(|scenario| scenario.pane.pane_id == pane.pane_id)
        .map(|scenario| scenario.input.occurrences.as_slice())
        .unwrap_or(&[])
}

fn expected_labels_for(occurrence: &ReferenceOccurrence) -> Vec<ReferenceLabel> {
    let mut labels = Vec::new();
    if is_generated(occurrence) {
        labels.push(ReferenceLabel::Generated);
    }
    match occurrence.generated_or_external_state {
        GeneratedOrExternalState::ExternalDependency => labels.push(ReferenceLabel::External),
        GeneratedOrExternalState::ReadOnlySource => labels.push(ReferenceLabel::ReadOnly),
        GeneratedOrExternalState::ImportedSnapshot => labels.push(ReferenceLabel::ImportedSnapshot),
        _ => {}
    }
    if occurrence.access_kind == AccessKind::TestOnly {
        labels.push(ReferenceLabel::TestOnly);
    }
    labels
}

fn routes_match(routes: &[ActionRoute]) -> bool {
    routes.len() == ActionRoute::ALL.len()
        && ActionRoute::ALL.iter().all(|route| routes.contains(route))
}

fn every_action_covered(panes: &[&ReferencePane]) -> bool {
    PaneActionKind::ALL.iter().all(|action_kind| {
        panes.iter().any(|pane| {
            pane.actions
                .iter()
                .any(|action| action.action_kind == *action_kind)
        })
    })
}

fn every_evidence_answer_covered(panes: &[&ReferencePane]) -> bool {
    let answers = [
        ReferenceEvidenceClass::Semantic,
        ReferenceEvidenceClass::FrameworkDerived,
        ReferenceEvidenceClass::RuntimeObserved,
        ReferenceEvidenceClass::ImportedSnapshot,
        ReferenceEvidenceClass::LexicalFallback,
    ];
    answers.iter().all(|answer| {
        panes.iter().any(|pane| {
            pane.pane_evidence_class == *answer
                || pane
                    .groups
                    .iter()
                    .any(|group| group.evidence_class == *answer)
        })
    })
}

fn required_surfaces_covered(projections: &[ReferencePaneProjection]) -> bool {
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
