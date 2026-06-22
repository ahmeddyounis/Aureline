//! Canonical advanced-editing micro-surface truth model: selection-summary
//! strips, multi-cursor / column-edit semantics, fold-state risk markers, and
//! minimap / overview-ruler parity bound into one inspectable, keyboard-complete,
//! visually-subordinate orientation contract across the claimed M5 advanced
//! editors.
//!
//! Where the [completion-row model](crate::m5_completion_rows) freezes the shared
//! *suggestion row*, the [signature / snippet model](crate::m5_signature_snippet)
//! freezes the two protected *mid-typing* surfaces, the
//! [hover / peek model](crate::m5_hover_peek) freezes the contextual *inspectors*,
//! and the [editor-assist matrix](crate::m5_editor_assist) freezes the per-surface
//! degraded-state *policy*, this module freezes the **advanced-editing
//! micro-surfaces** — the selection-summary strip, the fold-state risk summary, and
//! the minimap / overview-ruler aid — that orient an editing session without
//! becoming a second hidden truth model. Before it, these surfaces were scattered:
//! one pane let a multi-cursor edit apply silently to a different set of carets than
//! the strip implied, another let a folded region read as clean while it hid a
//! diagnostic or a merge conflict, a third let a minimap carry severity by colour
//! alone and diverge from the main editor's markers. This module folds them into one
//! governed model that carries, for every claimed advanced editor:
//!
//! 1. **Selection-summary truth** — every surface resolves a
//!    [`SelectionSummaryStrip`] that names its [`SelectionModeClass`]
//!    (single caret / multi-cursor / column block), its caret count, its primary
//!    caret, a quick detail, and — crucially — its [`SelectionSemanticsClass`], so a
//!    user can always tell whether the next operation is exact for all selections,
//!    normalized / expanded, primary-caret-only, or blocked. Operations that cannot
//!    apply to every caret are explained with a reason and a fallback route
//!    ([`UnsupportedOperationNote`]).
//! 2. **Fold-state risk markers** — folded regions never falsely appear clean. Each
//!    [`FoldRiskSummary`] reuses the canonical [`HiddenStateCounts`] and derives a
//!    [`FoldRiskClass`] so hidden diagnostics, conflicts, and trust / policy
//!    warnings stay advertised with a non-colour marker and a reveal-detail route.
//! 3. **Overview / minimap parity** — minimap and overview-ruler aids are optional
//!    accelerators, never the sole carrier of critical state. Each
//!    [`OverviewAidParity`] reuses the canonical [`OverviewAidKind`] and
//!    [`OrientationAidAvailability`] vocabulary, pins the *same* marker-semantics
//!    source the main editor uses (so the aid cannot diverge into a second truth
//!    model), names replacement routes, and degrades honestly in constrained
//!    profiles.
//! 4. **Non-colour-only, profile-aware state** — every strip, fold summary, and aid
//!    carries a non-colour differentiator, and a model-level [`RenderAwarenessPolicy`]
//!    set proves that density, high-zoom, and reduced-motion profiles compact only
//!    *optional* chrome and never drop severity / actionability state.
//!
//! Notebook and request editors **reuse** these shared records rather than bolting
//! on their own selection / fold / minimap semantics (the
//! `notebook_and_request_reuse_shared_vocabulary` invariant). Each claimed surface
//! resolves into an [`AdvancedEditorSnapshot`]; the build is static and
//! deterministic: [`advanced_editing_model`] assembles the one canonical record, the
//! checked-in fixture plus the replay gate freeze it byte-for-byte, and the model
//! proves its own honesty invariants over its data. It carries no file contents,
//! credential bodies, or raw provider payloads, so support, AI, and migration
//! surfaces can consume it directly.

use serde::{Deserialize, Serialize};

use crate::m5_assist_descriptors::{DensityTier, MotionClass, ZoomTier};
use crate::m5_editor_assist::{AssistDegradeClass, ClassDescriptor, EditorSurfaceClass};
use crate::orientation::{HiddenStateCounts, OrientationAidAvailability, OverviewAidKind};

/// Schema version for the advanced-editing model record.
pub const M5_ADVANCED_EDITING_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the advanced-editing model record.
pub const M5_ADVANCED_EDITING_SCHEMA_REF: &str = "schemas/editor/m5-advanced-editing.schema.json";

/// Stable record-kind tag for the advanced-editing model record.
pub const M5_ADVANCED_EDITING_RECORD_KIND: &str = "m5_advanced_editing_model";

/// Stable id for the canonical advanced-editing model.
pub const M5_ADVANCED_EDITING_MODEL_ID: &str = "m5-advanced-editing:model:0001";

/// Capture stamp for the canonical model. Held as a constant so the projection
/// stays deterministic and the fixture freezes byte-for-byte.
pub const M5_ADVANCED_EDITING_AS_OF: &str = "2026-06-22T00:00:00Z";

const SELECTION_INSPECT_COMMAND: &str = "command.editor.selection.inspect_summary";
const FOLD_TOGGLE_COMMAND: &str = "command.editor.fold.toggle";
const FOLD_REVEAL_DIAGNOSTIC_ROUTE: &str = "route:problems.reveal_fold_hidden_state";
const FOLD_REVEAL_CONFLICT_ROUTE: &str = "route:review.reveal_conflict_in_fold";
const FOLD_REVEAL_TRUST_ROUTE: &str = "route:trust.reveal_fold_policy_state";

/// The marker-semantics source every overview aid pins, by id. It is the *same*
/// source the main editor's diagnostics / conflict / trust decorations resolve
/// from, so an aid cannot diverge into a second hidden truth model.
const SHARED_MARKER_SEMANTICS_REF: &str = "marker-semantics:main-editor:diagnostics-conflict-trust";

const OVERVIEW_REPLACEMENT_ROUTES: [&str; 3] = [
    "route:problems.panel",
    "route:search.results",
    "route:outline.current_file",
];

// ---------------------------------------------------------------------------
// Selection mode + semantics.
// ---------------------------------------------------------------------------

/// The selection mode a surface's selection-summary strip reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionModeClass {
    /// A single caret / single contiguous selection.
    SingleCaret,
    /// Multiple cursors / selections edited together.
    MultiCursor,
    /// A column / block (box) selection.
    ColumnBlock,
}

impl SelectionModeClass {
    /// All selection modes, in catalog order.
    pub const ALL: [Self; 3] = [Self::SingleCaret, Self::MultiCursor, Self::ColumnBlock];

    /// Returns the stable schema token for this mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleCaret => "single_caret",
            Self::MultiCursor => "multi_cursor",
            Self::ColumnBlock => "column_block",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SingleCaret => "Single caret",
            Self::MultiCursor => "Multiple cursors",
            Self::ColumnBlock => "Column selection",
        }
    }

    /// Returns true when the mode edits more than one caret together, so the strip
    /// must show a caret count and a primary caret.
    pub const fn is_multi(self) -> bool {
        matches!(self, Self::MultiCursor | Self::ColumnBlock)
    }
}

/// How the next operation maps onto the current selection. This is the answer to
/// "can I tell what my edit will actually do?": exact for every selection,
/// normalized / expanded before applying, primary-caret only, or blocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionSemanticsClass {
    /// The operation applies exactly to every selection as shown.
    ExactAllSelections,
    /// Selections were normalized / expanded (e.g. to word or line boundaries)
    /// before the operation; the strip discloses the adjustment.
    NormalizedExpanded,
    /// The operation applies to the primary caret only; the others are unaffected
    /// and the strip discloses it.
    PrimaryCaretOnly,
    /// The operation cannot apply in this selection mode and is blocked with a
    /// disclosed reason.
    Blocked,
}

impl SelectionSemanticsClass {
    /// All selection-semantics classes, in catalog order.
    pub const ALL: [Self; 4] = [
        Self::ExactAllSelections,
        Self::NormalizedExpanded,
        Self::PrimaryCaretOnly,
        Self::Blocked,
    ];

    /// Returns the stable schema token for this class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactAllSelections => "exact_all_selections",
            Self::NormalizedExpanded => "normalized_expanded",
            Self::PrimaryCaretOnly => "primary_caret_only",
            Self::Blocked => "blocked",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ExactAllSelections => "Exact for all selections",
            Self::NormalizedExpanded => "Normalized / expanded",
            Self::PrimaryCaretOnly => "Primary caret only",
            Self::Blocked => "Blocked",
        }
    }

    /// Returns true when the semantics are exact for every selection.
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::ExactAllSelections)
    }

    /// Returns true when the semantics must be disclosed because they differ from a
    /// plain exact-all application.
    pub const fn requires_disclosure(self) -> bool {
        !self.is_exact()
    }

    /// Returns true when at least one caret is left out of the operation, so the
    /// strip must explain the unsupported / narrowed application.
    pub const fn narrows_application(self) -> bool {
        matches!(self, Self::PrimaryCaretOnly | Self::Blocked)
    }
}

// ---------------------------------------------------------------------------
// Fold risk.
// ---------------------------------------------------------------------------

/// The risk a folded region carries, derived from its [`HiddenStateCounts`]. A
/// folded region must never falsely appear clean while it hides a diagnostic, a
/// conflict, or a trust / policy warning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FoldRiskClass {
    /// No hidden critical or informational state.
    Clean,
    /// Hidden informational state only (e.g. search hits) — no severity.
    HiddenInformational,
    /// Hidden critical state: diagnostics, conflicts, or trust / policy warnings.
    HiddenCritical,
}

impl FoldRiskClass {
    /// All fold-risk classes, in catalog order.
    pub const ALL: [Self; 3] = [Self::Clean, Self::HiddenInformational, Self::HiddenCritical];

    /// Returns the stable schema token for this class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::HiddenInformational => "hidden_informational",
            Self::HiddenCritical => "hidden_critical",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Clean => "Clean",
            Self::HiddenInformational => "Hidden search hits",
            Self::HiddenCritical => "Hidden critical state",
        }
    }

    /// Derives the risk class from the hidden-state counts of a folded region.
    pub const fn from_hidden_counts(counts: &HiddenStateCounts) -> Self {
        if counts.has_critical_state() {
            Self::HiddenCritical
        } else if counts.search_hits > 0 {
            Self::HiddenInformational
        } else {
            Self::Clean
        }
    }

    /// Returns true when the region hides severity-bearing state.
    pub const fn is_critical(self) -> bool {
        matches!(self, Self::HiddenCritical)
    }
}

// ---------------------------------------------------------------------------
// Selection-summary strip.
// ---------------------------------------------------------------------------

/// An explanation for an operation that cannot apply to every caret in the current
/// selection, with a reason and a fallback route the user can take instead.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnsupportedOperationNote {
    /// Human-readable operation label (e.g. "Rename symbol").
    pub operation_label: String,
    /// Why the operation cannot apply to every caret in this mode.
    pub reason: String,
    /// A route the user can take instead (apply to primary, open a refactor
    /// preview, request approval, …), referenced by id.
    pub fallback_route_ref: String,
}

/// The selection-summary strip resolved for a surface: the truthful read-out of
/// the current multi-cursor / column-edit session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionSummaryStrip {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable strip id.
    pub strip_id: String,
    /// Current selection mode.
    pub mode_class: SelectionModeClass,
    /// Number of carets / selections the next edit affects.
    pub caret_count: u32,
    /// Primary caret label used at high zoom and by assistive technology.
    pub primary_caret_label: String,
    /// Plain-language quick detail (e.g. "3 carets · 8 selected chars each").
    pub quick_detail: String,
    /// How the next operation maps onto the current selection.
    pub semantics_class: SelectionSemanticsClass,
    /// Whether non-exact semantics are disclosed.
    pub semantics_disclosed: bool,
    /// A preview of what a column / block edit inserts across rows, when in column
    /// mode.
    pub column_edit_preview: Option<String>,
    /// Undo grouping class for the next edit (so a multi-caret edit undoes as one
    /// group, never silently per-caret).
    pub undo_grouping_class: String,
    /// Explanations for operations that cannot apply to every caret.
    pub unsupported_operations: Vec<UnsupportedOperationNote>,
    /// Whether the strip and its detail are reachable by keyboard.
    pub keyboard_reachable: bool,
    /// Command id that inspects the full selection summary.
    pub inspect_command_id_ref: String,
    /// Non-colour differentiator for the semantics state.
    pub non_color_differentiator: String,
    /// Accessible summary for screen readers.
    pub accessibility_label: String,
}

impl SelectionSummaryStrip {
    /// Stable record-kind tag for selection-summary strips.
    pub const RECORD_KIND: &'static str = "m5_selection_summary_strip";

    /// Returns true when a multi-caret strip shows a caret count and a primary
    /// caret, and a single-caret strip is self-consistent.
    pub fn count_and_primary_visible(&self) -> bool {
        if self.mode_class.is_multi() {
            self.caret_count > 1
                && !self.primary_caret_label.trim().is_empty()
                && !self.undo_grouping_class.trim().is_empty()
        } else {
            self.caret_count >= 1 && !self.primary_caret_label.trim().is_empty()
        }
    }

    /// Returns true when non-exact semantics are disclosed.
    pub fn semantics_disclosed_when_inexact(&self) -> bool {
        !self.semantics_class.requires_disclosure() || self.semantics_disclosed
    }

    /// Returns true when every narrowed / blocked application explains at least one
    /// unsupported operation, and every listed note is fully populated.
    pub fn unsupported_operations_explained(&self) -> bool {
        if self.semantics_class.narrows_application() && self.unsupported_operations.is_empty() {
            return false;
        }
        self.unsupported_operations.iter().all(|note| {
            !note.operation_label.trim().is_empty()
                && !note.reason.trim().is_empty()
                && !note.fallback_route_ref.trim().is_empty()
        })
    }
}

/// Initialization data for a [`SelectionSummaryStrip`].
#[derive(Debug, Clone, PartialEq, Eq)]
struct SelectionStripInit {
    surface: EditorSurfaceClass,
    mode_class: SelectionModeClass,
    caret_count: u32,
    primary_caret_label: String,
    quick_detail: String,
    semantics_class: SelectionSemanticsClass,
    column_edit_preview: Option<String>,
    undo_grouping_class: String,
    unsupported_operations: Vec<UnsupportedOperationNote>,
}

fn build_selection_strip(init: SelectionStripInit) -> SelectionSummaryStrip {
    let semantics_disclosed = init.semantics_class.requires_disclosure();
    let non_color_differentiator =
        format!("count badge + \"{}\" text", init.semantics_class.label());
    let accessibility_label = build_selection_accessibility_label(&init);

    SelectionSummaryStrip {
        record_kind: SelectionSummaryStrip::RECORD_KIND.to_owned(),
        strip_id: format!("selection-strip:{}", init.surface.as_str()),
        mode_class: init.mode_class,
        caret_count: init.caret_count,
        primary_caret_label: init.primary_caret_label,
        quick_detail: init.quick_detail,
        semantics_class: init.semantics_class,
        semantics_disclosed,
        column_edit_preview: init.column_edit_preview,
        undo_grouping_class: init.undo_grouping_class,
        unsupported_operations: init.unsupported_operations,
        keyboard_reachable: true,
        inspect_command_id_ref: SELECTION_INSPECT_COMMAND.to_owned(),
        non_color_differentiator,
        accessibility_label,
    }
}

fn build_selection_accessibility_label(init: &SelectionStripInit) -> String {
    let mode = init.mode_class.label();
    let semantics = init.semantics_class.label();
    let caret = if init.mode_class.is_multi() {
        format!(
            "{} carets, primary at {}",
            init.caret_count, init.primary_caret_label
        )
    } else {
        format!("primary caret at {}", init.primary_caret_label)
    };
    if init.unsupported_operations.is_empty() {
        format!("{mode}: {caret}. Next edit: {semantics}.")
    } else {
        format!(
            "{mode}: {caret}. Next edit: {semantics}. {} operation(s) cannot apply to every caret; press the inspect key for details.",
            init.unsupported_operations.len(),
        )
    }
}

// ---------------------------------------------------------------------------
// Fold-state risk summary.
// ---------------------------------------------------------------------------

/// A fold-state risk summary: a folded region that keeps its hidden critical state
/// advertised instead of falsely appearing clean.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FoldRiskSummary {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable fold id.
    pub fold_id: String,
    /// Human-facing fold label.
    pub label: String,
    /// Number of hidden physical lines.
    pub hidden_line_count: u32,
    /// Hidden critical / informational counts (reused canonical packet).
    pub hidden_state_counts: HiddenStateCounts,
    /// Risk class derived from the hidden-state counts.
    pub risk_class: FoldRiskClass,
    /// Whether the fold advertises hidden critical state with a marker.
    pub advertises_hidden_state: bool,
    /// Keyboard command id for toggling the fold.
    pub keyboard_toggle_command_id_ref: String,
    /// Route that reveals the hidden state, when there is critical state to reveal.
    pub reveal_detail_route_ref: Option<String>,
    /// Non-colour marker describing the fold and any hidden state.
    pub non_color_marker: String,
    /// Accessible summary for screen readers.
    pub accessibility_label: String,
}

impl FoldRiskSummary {
    /// Stable record-kind tag for fold-risk summaries.
    pub const RECORD_KIND: &'static str = "m5_fold_risk_summary";

    /// Returns true when the derived risk class matches the hidden-state counts.
    pub fn risk_class_matches_counts(&self) -> bool {
        self.risk_class == FoldRiskClass::from_hidden_counts(&self.hidden_state_counts)
    }

    /// Returns true when a fold hiding critical state advertises it and offers a
    /// reveal route, so it never falsely reads as clean.
    pub fn advertises_critical_state(&self) -> bool {
        if !self.risk_class.is_critical() {
            return true;
        }
        self.advertises_hidden_state
            && self
                .reveal_detail_route_ref
                .as_ref()
                .is_some_and(|route| !route.trim().is_empty())
    }

    /// Returns true when the fold stays keyboard-toggleable and screen-reader
    /// labeled.
    pub fn keyboard_and_label_present(&self) -> bool {
        !self.keyboard_toggle_command_id_ref.trim().is_empty()
            && !self.accessibility_label.trim().is_empty()
            && !self.non_color_marker.trim().is_empty()
    }
}

/// Initialization data for a [`FoldRiskSummary`].
#[derive(Debug, Clone, PartialEq, Eq)]
struct FoldSummaryInit {
    fold_id: String,
    label: String,
    hidden_line_count: u32,
    hidden_state_counts: HiddenStateCounts,
}

fn build_fold_summary(init: FoldSummaryInit) -> FoldRiskSummary {
    let risk_class = FoldRiskClass::from_hidden_counts(&init.hidden_state_counts);
    let advertises_hidden_state = risk_class.is_critical();
    let counts = &init.hidden_state_counts;
    let reveal_detail_route_ref = if risk_class.is_critical() {
        if counts.conflicts > 0 {
            Some(FOLD_REVEAL_CONFLICT_ROUTE.to_owned())
        } else if counts.trust_warnings > 0 && counts.diagnostics == 0 {
            Some(FOLD_REVEAL_TRUST_ROUTE.to_owned())
        } else {
            Some(FOLD_REVEAL_DIAGNOSTIC_ROUTE.to_owned())
        }
    } else {
        None
    };

    let non_color_marker = match risk_class {
        FoldRiskClass::HiddenCritical => format!(
            "fold glyph + \"{} hidden\" badge ({} diagnostics, {} conflicts, {} trust)",
            init.hidden_line_count, counts.diagnostics, counts.conflicts, counts.trust_warnings,
        ),
        FoldRiskClass::HiddenInformational => format!(
            "fold glyph + \"{} hidden\" badge ({} search hits)",
            init.hidden_line_count, counts.search_hits,
        ),
        FoldRiskClass::Clean => format!("fold glyph + \"{} hidden\" badge", init.hidden_line_count),
    };

    let accessibility_label = match risk_class {
        FoldRiskClass::HiddenCritical => format!(
            "{} folded, {} hidden lines; {} diagnostics, {} conflicts, {} trust warnings inside.",
            init.label,
            init.hidden_line_count,
            counts.diagnostics,
            counts.conflicts,
            counts.trust_warnings,
        ),
        FoldRiskClass::HiddenInformational => format!(
            "{} folded, {} hidden lines; {} search hits inside.",
            init.label, init.hidden_line_count, counts.search_hits,
        ),
        FoldRiskClass::Clean => format!(
            "{} folded, {} hidden lines; no diagnostics, conflicts, or trust warnings inside.",
            init.label, init.hidden_line_count,
        ),
    };

    FoldRiskSummary {
        record_kind: FoldRiskSummary::RECORD_KIND.to_owned(),
        fold_id: init.fold_id,
        label: init.label,
        hidden_line_count: init.hidden_line_count,
        hidden_state_counts: init.hidden_state_counts,
        risk_class,
        advertises_hidden_state,
        keyboard_toggle_command_id_ref: FOLD_TOGGLE_COMMAND.to_owned(),
        reveal_detail_route_ref,
        non_color_marker,
        accessibility_label,
    }
}

// ---------------------------------------------------------------------------
// Overview / minimap parity.
// ---------------------------------------------------------------------------

/// A minimap or overview-ruler aid, modeled as an optional accelerator that stays
/// aligned with the main editor's marker semantics and never becomes the sole
/// carrier of critical state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverviewAidParity {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Stable aid id.
    pub aid_id: String,
    /// Minimap or overview ruler.
    pub aid_kind: OverviewAidKind,
    /// Current availability.
    pub availability: OrientationAidAvailability,
    /// Visible degraded-state message when the aid is reduced or disabled.
    pub degraded_state_message: Option<String>,
    /// The shared marker-semantics source the aid pins, by id.
    pub marker_semantics_ref: String,
    /// Whether the aid's markers are aligned with the main editor's semantics.
    pub aligned_with_main_editor: bool,
    /// Whether the aid is the sole carrier of any critical state (must be false).
    pub is_sole_carrier_of_critical_state: bool,
    /// Whether the aid is an optional accelerator (must be true).
    pub is_optional_accelerator: bool,
    /// Keyboard routes that expose equivalent critical state.
    pub replacement_route_refs: Vec<String>,
    /// Non-colour differentiator for severity / actionability markers.
    pub non_color_differentiator: String,
    /// Accessible summary for screen readers.
    pub accessibility_label: String,
}

impl OverviewAidParity {
    /// Stable record-kind tag for overview-aid parity records.
    pub const RECORD_KIND: &'static str = "m5_overview_aid_parity";

    /// Returns true when the aid is an optional accelerator that is not the sole
    /// carrier of critical state and names replacement routes.
    pub fn not_sole_carrier(&self) -> bool {
        self.is_optional_accelerator
            && !self.is_sole_carrier_of_critical_state
            && !self.replacement_route_refs.is_empty()
    }

    /// Returns true when the aid is aligned with the main editor's marker semantics
    /// via the shared source ref (no second hidden truth model).
    pub fn aligned_with_main(&self) -> bool {
        self.aligned_with_main_editor && self.marker_semantics_ref == SHARED_MARKER_SEMANTICS_REF
    }

    /// Returns true when a reduced / disabled aid degrades honestly: it carries a
    /// visible message and names an alternate path.
    pub fn degrades_honestly(&self) -> bool {
        if matches!(self.availability, OrientationAidAvailability::Available) {
            return true;
        }
        self.degraded_state_message
            .as_ref()
            .is_some_and(|message| !message.trim().is_empty())
            && !self.replacement_route_refs.is_empty()
            && !self.accessibility_label.trim().is_empty()
    }
}

/// Initialization data for an [`OverviewAidParity`].
#[derive(Debug, Clone, PartialEq, Eq)]
struct OverviewAidInit {
    surface: EditorSurfaceClass,
    aid_kind: OverviewAidKind,
    availability: OrientationAidAvailability,
    degraded_state_message: Option<String>,
}

fn build_overview_aid(init: OverviewAidInit) -> OverviewAidParity {
    let non_color_differentiator = format!(
        "{} severity glyphs + position ticks (shape and position, not colour)",
        init.aid_kind.as_str(),
    );
    let availability_label = match init.availability {
        OrientationAidAvailability::Available => "available",
        OrientationAidAvailability::Reduced => "reduced",
        OrientationAidAvailability::DisabledLargeFile => "disabled in large-file mode",
        OrientationAidAvailability::DisabledLowResource => "disabled in low-resource mode",
        OrientationAidAvailability::DisabledBySetting => "disabled by setting",
    };
    let kind_label = match init.aid_kind {
        OverviewAidKind::Minimap => "Minimap",
        OverviewAidKind::OverviewRuler => "Overview ruler",
    };
    let accessibility_label = format!(
        "{kind_label} {availability_label}; Problems, Search, and Outline expose the same critical state."
    );

    OverviewAidParity {
        record_kind: OverviewAidParity::RECORD_KIND.to_owned(),
        aid_id: format!(
            "overview-aid:{}:{}",
            init.surface.as_str(),
            init.aid_kind.as_str()
        ),
        aid_kind: init.aid_kind,
        availability: init.availability,
        degraded_state_message: init.degraded_state_message,
        marker_semantics_ref: SHARED_MARKER_SEMANTICS_REF.to_owned(),
        aligned_with_main_editor: true,
        is_sole_carrier_of_critical_state: false,
        is_optional_accelerator: true,
        replacement_route_refs: OVERVIEW_REPLACEMENT_ROUTES
            .iter()
            .map(|route| (*route).to_owned())
            .collect(),
        non_color_differentiator,
        accessibility_label,
    }
}

// ---------------------------------------------------------------------------
// Render-profile awareness.
// ---------------------------------------------------------------------------

/// One render-profile awareness policy: how a density / zoom / motion tier
/// compacts *optional* chrome while preserving severity / actionability state and
/// non-colour differentiation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderAwarenessPolicy {
    /// The profile dimension ("density", "zoom", or "motion").
    pub dimension: String,
    /// The tier token within the dimension.
    pub tier_token: String,
    /// Human-readable tier label.
    pub tier_label: String,
    /// What optional chrome this tier compacts away.
    pub compaction_note: String,
    /// Whether severity / actionability (critical) state is preserved in this tier.
    pub critical_state_preserved: bool,
    /// Whether non-colour differentiation is preserved in this tier.
    pub non_color_only_preserved: bool,
}

fn build_render_awareness() -> Vec<RenderAwarenessPolicy> {
    let mut policies = Vec::new();

    for tier in DensityTier::ALL {
        let compaction_note = match tier {
            DensityTier::Comfortable => {
                "Comfortable spacing; full strip detail, fold labels, and aid markers shown."
            }
            DensityTier::Compact => {
                "Compact spacing; quick detail tightened but caret count, semantics, and fold risk stay."
            }
            DensityTier::Dense => {
                "Dense spacing; optional quick-detail text compacts to a badge, but selection \
                 semantics, fold risk markers, and aid severity glyphs are never dropped."
            }
        };
        policies.push(RenderAwarenessPolicy {
            dimension: "density".to_owned(),
            tier_token: tier.as_str().to_owned(),
            tier_label: tier.label().to_owned(),
            compaction_note: compaction_note.to_owned(),
            critical_state_preserved: true,
            non_color_only_preserved: true,
        });
    }

    for tier in ZoomTier::ALL {
        let compaction_note = match tier {
            ZoomTier::Standard => {
                "Standard zoom; full inline detail across strip, folds, and aids."
            }
            ZoomTier::High => {
                "High zoom; horizontal budget is scarce, so quick detail collapses to the primary \
                 caret label and a count, but semantics, fold risk, and aid markers stay reachable."
            }
        };
        policies.push(RenderAwarenessPolicy {
            dimension: "zoom".to_owned(),
            tier_token: tier.as_str().to_owned(),
            tier_label: tier.label().to_owned(),
            compaction_note: compaction_note.to_owned(),
            critical_state_preserved: true,
            non_color_only_preserved: true,
        });
    }

    for class in MotionClass::ALL {
        let compaction_note = match class {
            MotionClass::Static => {
                "No animation; fold and selection state are shown with static glyphs and text."
            }
            MotionClass::AnimatedReducible => {
                "Animated transitions (e.g. fold collapse) are replaced with a static cue under \
                 reduced motion; no critical state depends on the animation."
            }
        };
        policies.push(RenderAwarenessPolicy {
            dimension: "motion".to_owned(),
            tier_token: class.as_str().to_owned(),
            tier_label: class.label().to_owned(),
            compaction_note: compaction_note.to_owned(),
            critical_state_preserved: true,
            non_color_only_preserved: true,
        });
    }

    policies
}

// ---------------------------------------------------------------------------
// Surface snapshot.
// ---------------------------------------------------------------------------

/// One claimed advanced editor resolved into its selection strip, fold-risk
/// summaries, and overview aids.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvancedEditorSnapshot {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub m5_advanced_editing_schema_version: u32,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Editor surface covered by this snapshot.
    pub surface_class: EditorSurfaceClass,
    /// Workspace id covered by the snapshot.
    pub workspace_id: String,
    /// Document ref covered by the snapshot.
    pub document_ref: String,
    /// Language id resolved for the document.
    pub language_id: String,
    /// Degraded-state posture for the surface.
    pub degrade_class: AssistDegradeClass,
    /// Visible degrade label.
    pub degrade_label: String,
    /// The selection-summary strip for this surface.
    pub selection_strip: SelectionSummaryStrip,
    /// Fold-risk summaries for this surface (may be empty when folding is
    /// suppressed, e.g. in large-file mode).
    pub fold_summaries: Vec<FoldRiskSummary>,
    /// Minimap / overview-ruler aids for this surface.
    pub overview_aids: Vec<OverviewAidParity>,
    /// Density tier the snapshot was captured under.
    pub density_tier: DensityTier,
    /// Zoom tier the snapshot was captured under.
    pub zoom_tier: ZoomTier,
    /// Motion class the snapshot was captured under.
    pub motion_class: MotionClass,
    /// Whether critical state survives the captured render profile.
    pub critical_state_preserved_in_profile: bool,
    /// Whether the snapshot needs selection / fold / aid disclosure.
    pub disclosure_required: bool,
    /// Accessible summary for screen readers.
    pub accessibility_summary: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl AdvancedEditorSnapshot {
    /// Stable record-kind tag for advanced-editor snapshots.
    pub const RECORD_KIND: &'static str = "m5_advanced_editing_snapshot";
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

/// One frozen honesty invariant the model must satisfy, with the result of
/// evaluating it over the model's own data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvancedEditingInvariant {
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

/// The canonical, frozen, export-safe advanced-editing micro-surface model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvancedEditingModel {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_advanced_editing_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable model id.
    pub model_id: String,
    /// Capture stamp.
    pub as_of: String,
    /// Selection-mode catalog.
    pub selection_mode_classes: Vec<ClassDescriptor>,
    /// Selection-semantics catalog.
    pub selection_semantics_classes: Vec<ClassDescriptor>,
    /// Fold-risk catalog.
    pub fold_risk_classes: Vec<ClassDescriptor>,
    /// Overview-aid-kind catalog.
    pub overview_aid_classes: Vec<ClassDescriptor>,
    /// Aid-availability catalog.
    pub aid_availability_classes: Vec<ClassDescriptor>,
    /// Render-profile awareness policies.
    pub render_awareness: Vec<RenderAwarenessPolicy>,
    /// One snapshot per claimed advanced editor surface.
    pub surface_snapshots: Vec<AdvancedEditorSnapshot>,
    /// Frozen invariants and whether each holds on this model.
    pub invariants: Vec<AdvancedEditingInvariant>,
    /// Whether the model is metadata-safe for support export.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl AdvancedEditingModel {
    /// Returns true when every frozen invariant holds on this model.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|invariant| invariant.holds)
    }

    /// Returns true when the model is metadata-safe for support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == M5_ADVANCED_EDITING_SCHEMA_REF
            && self.record_kind == M5_ADVANCED_EDITING_RECORD_KIND
    }

    /// Returns the snapshot for the given surface, when present.
    pub fn snapshot(&self, surface: EditorSurfaceClass) -> Option<&AdvancedEditorSnapshot> {
        self.surface_snapshots
            .iter()
            .find(|snapshot| snapshot.surface_class == surface)
    }

    /// Returns every selection strip across every snapshot.
    pub fn all_selection_strips(&self) -> impl Iterator<Item = &SelectionSummaryStrip> {
        self.surface_snapshots
            .iter()
            .map(|snapshot| &snapshot.selection_strip)
    }

    /// Returns every fold-risk summary across every snapshot.
    pub fn all_fold_summaries(&self) -> impl Iterator<Item = &FoldRiskSummary> {
        self.surface_snapshots
            .iter()
            .flat_map(|snapshot| snapshot.fold_summaries.iter())
    }

    /// Returns every overview aid across every snapshot.
    pub fn all_overview_aids(&self) -> impl Iterator<Item = &OverviewAidParity> {
        self.surface_snapshots
            .iter()
            .flat_map(|snapshot| snapshot.overview_aids.iter())
    }
}

// ---------------------------------------------------------------------------
// Helpers.
// ---------------------------------------------------------------------------

fn document_ref_for(surface: EditorSurfaceClass) -> String {
    let path = match surface {
        EditorSurfaceClass::CodeFile => "src/render.rs",
        EditorSurfaceClass::ConfigFile => "config/services.toml",
        EditorSurfaceClass::NotebookCell => "analysis.ipynb#cell-3",
        EditorSurfaceClass::RequestEditor => "requests/list_users.http",
        EditorSurfaceClass::SqlEditor => "queries/active_users.sql",
        EditorSurfaceClass::DocsCodeBlock => "docs/guide.md#example-2",
        EditorSurfaceClass::GeneratedFile => "target/generated/schema.rs",
        EditorSurfaceClass::ProtectedFile => "infra/policy.toml",
        EditorSurfaceClass::PartialIndexState => "src/pipeline.rs",
        EditorSurfaceClass::LargeFileRestricted => "logs/trace.log",
    };
    format!("doc:{path}")
}

const fn language_id_for(surface: EditorSurfaceClass) -> &'static str {
    match surface {
        EditorSurfaceClass::CodeFile
        | EditorSurfaceClass::GeneratedFile
        | EditorSurfaceClass::PartialIndexState => "rust",
        EditorSurfaceClass::ConfigFile | EditorSurfaceClass::ProtectedFile => "toml",
        EditorSurfaceClass::NotebookCell => "python",
        EditorSurfaceClass::RequestEditor => "http",
        EditorSurfaceClass::SqlEditor => "sql",
        EditorSurfaceClass::DocsCodeBlock => "markdown",
        EditorSurfaceClass::LargeFileRestricted => "log",
    }
}

fn class_descriptor(token: &str, label: &str, note: &str) -> ClassDescriptor {
    ClassDescriptor {
        class_token: token.to_owned(),
        label: label.to_owned(),
        note: note.to_owned(),
    }
}

fn unsupported(
    operation_label: &str,
    reason: &str,
    fallback_route_ref: &str,
) -> UnsupportedOperationNote {
    UnsupportedOperationNote {
        operation_label: operation_label.to_owned(),
        reason: reason.to_owned(),
        fallback_route_ref: fallback_route_ref.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Builder.
// ---------------------------------------------------------------------------

/// Builds the one canonical advanced-editing micro-surface model.
///
/// The build is deterministic and self-contained: it materializes one
/// [`AdvancedEditorSnapshot`] per claimed advanced editor surface, each resolving a
/// [`SelectionSummaryStrip`], its [`FoldRiskSummary`] rows, and its
/// [`OverviewAidParity`] aids from the shared vocabulary, and evaluates every frozen
/// honesty invariant over the assembled data so the record's `invariants[].holds`
/// reflect real checks.
pub fn advanced_editing_model() -> AdvancedEditingModel {
    let surface_snapshots = build_surface_snapshots();
    let render_awareness = build_render_awareness();
    let invariants = evaluate_invariants(&surface_snapshots, &render_awareness);

    let qualified = invariants.iter().all(|invariant| invariant.holds);
    let summary = if qualified {
        format!(
            "Advanced-editing model frozen: {surfaces} advanced editors each resolve a \
             selection-summary strip, fold-risk summaries, and minimap / overview-ruler aids. \
             Every strip names its selection mode, caret count, primary caret, and whether the \
             next operation is exact for all selections, normalized / expanded, primary-only, or \
             blocked, and explains operations that cannot apply to every caret. Folded regions \
             advertise hidden diagnostics, conflicts, and trust warnings instead of appearing \
             clean. Minimap and overview aids stay aligned with the main editor's markers, never \
             become the sole carrier of critical state, and degrade honestly. Every micro-surface \
             is keyboard-complete, non-colour-only, and density / zoom / motion aware. All \
             {invariants} invariants hold.",
            surfaces = surface_snapshots.len(),
            invariants = invariants.len(),
        )
    } else {
        format!(
            "Advanced-editing model INVALID: {failing} of {total} invariants do not hold.",
            failing = invariants.iter().filter(|i| !i.holds).count(),
            total = invariants.len(),
        )
    };

    AdvancedEditingModel {
        record_kind: M5_ADVANCED_EDITING_RECORD_KIND.to_owned(),
        m5_advanced_editing_schema_version: M5_ADVANCED_EDITING_SCHEMA_VERSION,
        schema_ref: M5_ADVANCED_EDITING_SCHEMA_REF.to_owned(),
        model_id: M5_ADVANCED_EDITING_MODEL_ID.to_owned(),
        as_of: M5_ADVANCED_EDITING_AS_OF.to_owned(),
        selection_mode_classes: build_selection_mode_catalog(),
        selection_semantics_classes: build_selection_semantics_catalog(),
        fold_risk_classes: build_fold_risk_catalog(),
        overview_aid_classes: build_overview_aid_catalog(),
        aid_availability_classes: build_aid_availability_catalog(),
        render_awareness,
        surface_snapshots,
        invariants,
        raw_payload_excluded: true,
        summary,
    }
}

/// Builds the human-readable projection of the model for support and headless use.
pub fn advanced_editing_model_lines(model: &AdvancedEditingModel) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Advanced-editing model — {} ({})",
        model.model_id, model.as_of
    ));
    lines.push(format!(
        "schema_ref={} version={}",
        model.schema_ref, model.m5_advanced_editing_schema_version
    ));

    lines.push("Surface snapshots:".to_owned());
    for snapshot in &model.surface_snapshots {
        lines.push(format!(
            "  {surface}: {degrade} ({label}) — disclosure={disclosure}",
            surface = snapshot.surface_class.as_str(),
            degrade = snapshot.degrade_class.as_str(),
            label = snapshot.degrade_label,
            disclosure = snapshot.disclosure_required,
        ));
        let strip = &snapshot.selection_strip;
        lines.push(format!(
            "    selection: mode={mode} carets={carets} semantics={semantics} unsupported={unsupported}",
            mode = strip.mode_class.as_str(),
            carets = strip.caret_count,
            semantics = strip.semantics_class.as_str(),
            unsupported = strip.unsupported_operations.len(),
        ));
        for fold in &snapshot.fold_summaries {
            lines.push(format!(
                "    fold {id}: risk={risk} hidden_lines={lines} advertises={advertises}",
                id = fold.fold_id,
                risk = fold.risk_class.as_str(),
                lines = fold.hidden_line_count,
                advertises = fold.advertises_hidden_state,
            ));
        }
        for aid in &snapshot.overview_aids {
            lines.push(format!(
                "    aid {kind}: availability={availability} aligned={aligned} sole_carrier={sole}",
                kind = aid.aid_kind.as_str(),
                availability = aid.availability.as_str(),
                aligned = aid.aligned_with_main_editor,
                sole = aid.is_sole_carrier_of_critical_state,
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

fn build_selection_mode_catalog() -> Vec<ClassDescriptor> {
    SelectionModeClass::ALL
        .iter()
        .map(|mode| {
            let note = if mode.is_multi() {
                "Multi-caret mode; the strip shows caret count, primary caret, and undo grouping."
            } else {
                "Single caret / contiguous selection."
            };
            class_descriptor(mode.as_str(), mode.label(), note)
        })
        .collect()
}

fn build_selection_semantics_catalog() -> Vec<ClassDescriptor> {
    SelectionSemanticsClass::ALL
        .iter()
        .map(|semantics| {
            let note = if semantics.is_exact() {
                "The next operation applies exactly to every selection as shown."
            } else if semantics.narrows_application() {
                "The operation leaves carets out; the strip discloses it and explains the fallback."
            } else {
                "Selections were normalized / expanded before applying; the strip discloses it."
            };
            class_descriptor(semantics.as_str(), semantics.label(), note)
        })
        .collect()
}

fn build_fold_risk_catalog() -> Vec<ClassDescriptor> {
    FoldRiskClass::ALL
        .iter()
        .map(|risk| {
            let note = if risk.is_critical() {
                "Hidden diagnostics / conflicts / trust warnings; advertised with a marker and a reveal route."
            } else if matches!(risk, FoldRiskClass::HiddenInformational) {
                "Hidden search hits only; no severity, still counted in the fold marker."
            } else {
                "No hidden critical or informational state."
            };
            class_descriptor(risk.as_str(), risk.label(), note)
        })
        .collect()
}

fn build_overview_aid_catalog() -> Vec<ClassDescriptor> {
    [OverviewAidKind::Minimap, OverviewAidKind::OverviewRuler]
        .iter()
        .map(|kind| {
            let (label, note) = match kind {
                OverviewAidKind::Minimap => (
                    "Minimap",
                    "Scaled document preview; optional accelerator aligned with main-editor markers.",
                ),
                OverviewAidKind::OverviewRuler => (
                    "Overview ruler",
                    "Thin semantic marker rail; optional accelerator aligned with main-editor markers.",
                ),
            };
            class_descriptor(kind.as_str(), label, note)
        })
        .collect()
}

fn build_aid_availability_catalog() -> Vec<ClassDescriptor> {
    [
        OrientationAidAvailability::Available,
        OrientationAidAvailability::Reduced,
        OrientationAidAvailability::DisabledLargeFile,
        OrientationAidAvailability::DisabledLowResource,
        OrientationAidAvailability::DisabledBySetting,
    ]
    .iter()
    .map(|availability| {
        let (label, note) = match availability {
            OrientationAidAvailability::Available => ("Available", "Aid is visible and current."),
            OrientationAidAvailability::Reduced => {
                ("Reduced", "Aid is visible but simplified; markers still come from the main editor.")
            }
            OrientationAidAvailability::DisabledLargeFile => (
                "Disabled (large file)",
                "Aid is disabled in large-file mode; alternate routes carry the state.",
            ),
            OrientationAidAvailability::DisabledLowResource => (
                "Disabled (low resource)",
                "Aid is disabled in low-resource mode; alternate routes carry the state.",
            ),
            OrientationAidAvailability::DisabledBySetting => (
                "Disabled (by setting)",
                "Aid is disabled by accessibility or user setting; alternate routes carry the state.",
            ),
        };
        class_descriptor(availability.as_str(), label, note)
    })
    .collect()
}

// ---------------------------------------------------------------------------
// Snapshot assembly.
// ---------------------------------------------------------------------------

struct SnapshotSpec {
    surface: EditorSurfaceClass,
    workspace_id: &'static str,
    degrade_class: AssistDegradeClass,
    degrade_label: &'static str,
    selection_strip: SelectionSummaryStrip,
    fold_summaries: Vec<FoldRiskSummary>,
    overview_aids: Vec<OverviewAidParity>,
    density_tier: DensityTier,
    zoom_tier: ZoomTier,
    motion_class: MotionClass,
}

fn assemble_snapshot(spec: SnapshotSpec) -> AdvancedEditorSnapshot {
    let surface = spec.surface;
    let strip = spec.selection_strip;
    let folds = spec.fold_summaries;
    let aids = spec.overview_aids;

    let any_critical_fold = folds.iter().any(|fold| fold.risk_class.is_critical());
    let any_narrowed_aid = aids
        .iter()
        .any(|aid| !matches!(aid.availability, OrientationAidAvailability::Available));
    let disclosure_required = spec.degrade_class != AssistDegradeClass::FullFidelity
        || strip.semantics_disclosed
        || any_critical_fold
        || any_narrowed_aid;

    let accessibility_summary = format!(
        "{surface}: {strip} {folds} folded regions, {aids} overview aids.",
        surface = surface.label(),
        strip = strip.accessibility_label,
        folds = folds.len(),
        aids = aids.len(),
    );
    let export_safe_summary = format!(
        "{surface} resolves a {mode} selection strip ({semantics}), {folds} fold summaries, {aids} \
         overview aids; posture {posture}.",
        surface = surface.as_str(),
        mode = strip.mode_class.as_str(),
        semantics = strip.semantics_class.as_str(),
        folds = folds.len(),
        aids = aids.len(),
        posture = spec.degrade_class.as_str(),
    );

    AdvancedEditorSnapshot {
        record_kind: AdvancedEditorSnapshot::RECORD_KIND.to_owned(),
        m5_advanced_editing_schema_version: M5_ADVANCED_EDITING_SCHEMA_VERSION,
        snapshot_id: format!("advanced-editing:{}", surface.as_str()),
        surface_class: surface,
        workspace_id: spec.workspace_id.to_owned(),
        document_ref: document_ref_for(surface),
        language_id: language_id_for(surface).to_owned(),
        degrade_class: spec.degrade_class,
        degrade_label: spec.degrade_label.to_owned(),
        selection_strip: strip,
        fold_summaries: folds,
        overview_aids: aids,
        density_tier: spec.density_tier,
        zoom_tier: spec.zoom_tier,
        motion_class: spec.motion_class,
        critical_state_preserved_in_profile: true,
        disclosure_required,
        accessibility_summary,
        export_safe_summary,
    }
}

fn build_surface_snapshots() -> Vec<AdvancedEditorSnapshot> {
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

fn both_overview_aids(
    surface: EditorSurfaceClass,
    minimap: OrientationAidAvailability,
    minimap_message: Option<&str>,
    ruler: OrientationAidAvailability,
    ruler_message: Option<&str>,
) -> Vec<OverviewAidParity> {
    vec![
        build_overview_aid(OverviewAidInit {
            surface,
            aid_kind: OverviewAidKind::Minimap,
            availability: minimap,
            degraded_state_message: minimap_message.map(str::to_owned),
        }),
        build_overview_aid(OverviewAidInit {
            surface,
            aid_kind: OverviewAidKind::OverviewRuler,
            availability: ruler,
            degraded_state_message: ruler_message.map(str::to_owned),
        }),
    ]
}

fn build_code_file_snapshot() -> AdvancedEditorSnapshot {
    let surface = EditorSurfaceClass::CodeFile;
    // Multi-cursor edit that applies exactly to every caret.
    let selection_strip = build_selection_strip(SelectionStripInit {
        surface,
        mode_class: SelectionModeClass::MultiCursor,
        caret_count: 4,
        primary_caret_label: "Line 42, Col 8".to_owned(),
        quick_detail: "4 carets · same column · 6 chars selected each".to_owned(),
        semantics_class: SelectionSemanticsClass::ExactAllSelections,
        column_edit_preview: None,
        undo_grouping_class: "multi_caret_text_edit_single_undo_group".to_owned(),
        unsupported_operations: Vec::new(),
    });
    let fold_summaries = vec![
        build_fold_summary(FoldSummaryInit {
            fold_id: "fold:code:orders-controller".to_owned(),
            label: "OrdersController".to_owned(),
            hidden_line_count: 48,
            hidden_state_counts: HiddenStateCounts {
                diagnostics: 1,
                conflicts: 0,
                trust_warnings: 1,
                search_hits: 3,
            },
        }),
        build_fold_summary(FoldSummaryInit {
            fold_id: "fold:code:helpers".to_owned(),
            label: "helpers".to_owned(),
            hidden_line_count: 16,
            hidden_state_counts: HiddenStateCounts {
                diagnostics: 0,
                conflicts: 0,
                trust_warnings: 0,
                search_hits: 0,
            },
        }),
    ];
    let overview_aids = both_overview_aids(
        surface,
        OrientationAidAvailability::Available,
        None,
        OrientationAidAvailability::Available,
        None,
    );
    assemble_snapshot(SnapshotSpec {
        surface,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::FullFidelity,
        degrade_label: "Full fidelity",
        selection_strip,
        fold_summaries,
        overview_aids,
        density_tier: DensityTier::Comfortable,
        zoom_tier: ZoomTier::Standard,
        motion_class: MotionClass::AnimatedReducible,
    })
}

fn build_config_file_snapshot() -> AdvancedEditorSnapshot {
    let surface = EditorSurfaceClass::ConfigFile;
    // Column / block edit across aligned config values; exact for all rows.
    let selection_strip = build_selection_strip(SelectionStripInit {
        surface,
        mode_class: SelectionModeClass::ColumnBlock,
        caret_count: 6,
        primary_caret_label: "Line 10, Col 14".to_owned(),
        quick_detail: "6-row column · width 0 · insert at col 14".to_owned(),
        semantics_class: SelectionSemanticsClass::ExactAllSelections,
        column_edit_preview: Some(
            "Inserts \"timeout = \" at column 14 on all 6 selected rows.".to_owned(),
        ),
        undo_grouping_class: "column_edit_single_undo_group".to_owned(),
        unsupported_operations: Vec::new(),
    });
    let fold_summaries = vec![build_fold_summary(FoldSummaryInit {
        fold_id: "fold:config:services-table".to_owned(),
        label: "[services]".to_owned(),
        hidden_line_count: 22,
        hidden_state_counts: HiddenStateCounts {
            diagnostics: 0,
            conflicts: 0,
            trust_warnings: 0,
            search_hits: 2,
        },
    })];
    let overview_aids = both_overview_aids(
        surface,
        OrientationAidAvailability::Available,
        None,
        OrientationAidAvailability::Available,
        None,
    );
    assemble_snapshot(SnapshotSpec {
        surface,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::FullFidelity,
        degrade_label: "Full fidelity",
        selection_strip,
        fold_summaries,
        overview_aids,
        density_tier: DensityTier::Compact,
        zoom_tier: ZoomTier::Standard,
        motion_class: MotionClass::Static,
    })
}

fn build_notebook_cell_snapshot() -> AdvancedEditorSnapshot {
    let surface = EditorSurfaceClass::NotebookCell;
    // Multi-cursor inside a cell; carets are normalized to cell boundaries, and a
    // cross-cell refactor cannot apply to every caret. Reuses the shared strip.
    let selection_strip = build_selection_strip(SelectionStripInit {
        surface,
        mode_class: SelectionModeClass::MultiCursor,
        caret_count: 3,
        primary_caret_label: "Cell 3, Line 2".to_owned(),
        quick_detail: "3 carets · normalized to this cell".to_owned(),
        semantics_class: SelectionSemanticsClass::NormalizedExpanded,
        column_edit_preview: None,
        undo_grouping_class: "multi_caret_text_edit_single_undo_group".to_owned(),
        unsupported_operations: vec![unsupported(
            "Rename across cells",
            "Carets are normalized to the active cell; a rename that spans cells is not applied to the out-of-cell occurrences.",
            "route:notebook.open_cross_cell_refactor_preview",
        )],
    });
    let fold_summaries = vec![build_fold_summary(FoldSummaryInit {
        fold_id: "fold:notebook:cell-3-output".to_owned(),
        label: "Cell 3 output".to_owned(),
        hidden_line_count: 30,
        hidden_state_counts: HiddenStateCounts {
            diagnostics: 1,
            conflicts: 0,
            trust_warnings: 0,
            search_hits: 0,
        },
    })];
    let overview_aids = both_overview_aids(
        surface,
        OrientationAidAvailability::Reduced,
        Some("Minimap is reduced in the notebook; cell markers still come from the main editor sources."),
        OrientationAidAvailability::Available,
        None,
    );
    assemble_snapshot(SnapshotSpec {
        surface,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::FullFidelity,
        degrade_label: "Full fidelity",
        selection_strip,
        fold_summaries,
        overview_aids,
        density_tier: DensityTier::Comfortable,
        zoom_tier: ZoomTier::Standard,
        motion_class: MotionClass::Static,
    })
}

fn build_request_editor_snapshot() -> AdvancedEditorSnapshot {
    let surface = EditorSurfaceClass::RequestEditor;
    // Multi-cursor where some carets land in a read-only resolved section, so the
    // edit applies to the primary caret only. Reuses the shared strip.
    let selection_strip = build_selection_strip(SelectionStripInit {
        surface,
        mode_class: SelectionModeClass::MultiCursor,
        caret_count: 2,
        primary_caret_label: "Header line 4".to_owned(),
        quick_detail: "2 carets · 1 in a resolved (read-only) section".to_owned(),
        semantics_class: SelectionSemanticsClass::PrimaryCaretOnly,
        column_edit_preview: None,
        undo_grouping_class: "primary_caret_text_edit".to_owned(),
        unsupported_operations: vec![unsupported(
            "Type at all carets",
            "One caret is in a resolved request preview that is read-only, so the edit applies to the primary caret only.",
            "route:request.edit_raw_template",
        )],
    });
    let fold_summaries = vec![build_fold_summary(FoldSummaryInit {
        fold_id: "fold:request:body".to_owned(),
        label: "Request body".to_owned(),
        hidden_line_count: 12,
        hidden_state_counts: HiddenStateCounts {
            diagnostics: 0,
            conflicts: 0,
            trust_warnings: 0,
            search_hits: 0,
        },
    })];
    let overview_aids = both_overview_aids(
        surface,
        OrientationAidAvailability::DisabledBySetting,
        Some("Minimap is off for request editors by setting; Problems, Search, and Outline carry the state."),
        OrientationAidAvailability::Reduced,
        Some("Overview ruler is reduced for the compact request editor; markers still come from the main editor sources."),
    );
    assemble_snapshot(SnapshotSpec {
        surface,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::FullFidelity,
        degrade_label: "Full fidelity",
        selection_strip,
        fold_summaries,
        overview_aids,
        density_tier: DensityTier::Compact,
        zoom_tier: ZoomTier::Standard,
        motion_class: MotionClass::Static,
    })
}

fn build_sql_editor_snapshot() -> AdvancedEditorSnapshot {
    let surface = EditorSurfaceClass::SqlEditor;
    // Column / block edit across a column list; exact for all rows.
    let selection_strip = build_selection_strip(SelectionStripInit {
        surface,
        mode_class: SelectionModeClass::ColumnBlock,
        caret_count: 5,
        primary_caret_label: "Line 6, Col 5".to_owned(),
        quick_detail: "5-row column · prefix \"u.\" before each column".to_owned(),
        semantics_class: SelectionSemanticsClass::ExactAllSelections,
        column_edit_preview: Some(
            "Inserts \"u.\" at column 5 on all 5 selected column rows.".to_owned(),
        ),
        undo_grouping_class: "column_edit_single_undo_group".to_owned(),
        unsupported_operations: Vec::new(),
    });
    let fold_summaries = vec![build_fold_summary(FoldSummaryInit {
        fold_id: "fold:sql:active-users-cte".to_owned(),
        label: "WITH active_users".to_owned(),
        hidden_line_count: 18,
        hidden_state_counts: HiddenStateCounts {
            diagnostics: 1,
            conflicts: 0,
            trust_warnings: 0,
            search_hits: 1,
        },
    })];
    let overview_aids = both_overview_aids(
        surface,
        OrientationAidAvailability::Available,
        None,
        OrientationAidAvailability::Available,
        None,
    );
    assemble_snapshot(SnapshotSpec {
        surface,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::FullFidelity,
        degrade_label: "Full fidelity",
        selection_strip,
        fold_summaries,
        overview_aids,
        density_tier: DensityTier::Comfortable,
        zoom_tier: ZoomTier::Standard,
        motion_class: MotionClass::Static,
    })
}

fn build_docs_code_block_snapshot() -> AdvancedEditorSnapshot {
    let surface = EditorSurfaceClass::DocsCodeBlock;
    // A selection inside a fenced code block is expanded to the block boundaries.
    let selection_strip = build_selection_strip(SelectionStripInit {
        surface,
        mode_class: SelectionModeClass::SingleCaret,
        caret_count: 1,
        primary_caret_label: "Example 2, Line 3".to_owned(),
        quick_detail: "1 caret · scoped to this fenced block".to_owned(),
        semantics_class: SelectionSemanticsClass::NormalizedExpanded,
        column_edit_preview: None,
        undo_grouping_class: "single_caret_text_edit".to_owned(),
        unsupported_operations: vec![unsupported(
            "Select to end of document",
            "Selection is scoped to the fenced code block; it expands to the block boundary rather than into the surrounding prose.",
            "route:docs.edit_block_in_full_editor",
        )],
    });
    let fold_summaries = vec![build_fold_summary(FoldSummaryInit {
        fold_id: "fold:docs:example-2".to_owned(),
        label: "Example 2".to_owned(),
        hidden_line_count: 9,
        hidden_state_counts: HiddenStateCounts {
            diagnostics: 0,
            conflicts: 0,
            trust_warnings: 0,
            search_hits: 1,
        },
    })];
    let overview_aids = both_overview_aids(
        surface,
        OrientationAidAvailability::DisabledBySetting,
        Some("Minimap is off for the docs pane by setting; Outline and Search carry navigation."),
        OrientationAidAvailability::Reduced,
        Some("Overview ruler is reduced for the docs pane; markers still come from the main editor sources."),
    );
    assemble_snapshot(SnapshotSpec {
        surface,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::FullFidelity,
        degrade_label: "Full fidelity",
        selection_strip,
        fold_summaries,
        overview_aids,
        density_tier: DensityTier::Comfortable,
        zoom_tier: ZoomTier::Standard,
        motion_class: MotionClass::Static,
    })
}

fn build_generated_file_snapshot() -> AdvancedEditorSnapshot {
    let surface = EditorSurfaceClass::GeneratedFile;
    // Generated output is read-only; edits route through the generator, so a direct
    // multi-caret edit is blocked.
    let selection_strip = build_selection_strip(SelectionStripInit {
        surface,
        mode_class: SelectionModeClass::MultiCursor,
        caret_count: 3,
        primary_caret_label: "Line 20, Col 1".to_owned(),
        quick_detail: "3 carets · selection allowed for reading, edits route through the generator".to_owned(),
        semantics_class: SelectionSemanticsClass::Blocked,
        column_edit_preview: None,
        undo_grouping_class: "no_direct_edit".to_owned(),
        unsupported_operations: vec![unsupported(
            "Edit at all carets",
            "This is generated output; direct edits are blocked because writes route through the generator source.",
            "route:generated.open_generator_source",
        )],
    });
    let fold_summaries = vec![build_fold_summary(FoldSummaryInit {
        fold_id: "fold:generated:schema-region".to_owned(),
        label: "Generated schema region".to_owned(),
        hidden_line_count: 64,
        hidden_state_counts: HiddenStateCounts {
            diagnostics: 0,
            conflicts: 0,
            trust_warnings: 2,
            search_hits: 0,
        },
    })];
    let overview_aids = both_overview_aids(
        surface,
        OrientationAidAvailability::Available,
        None,
        OrientationAidAvailability::Available,
        None,
    );
    assemble_snapshot(SnapshotSpec {
        surface,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::ReadOnlyNoApply,
        degrade_label: "Read-only — generated output, regenerate via the generator",
        selection_strip,
        fold_summaries,
        overview_aids,
        density_tier: DensityTier::Comfortable,
        zoom_tier: ZoomTier::Standard,
        motion_class: MotionClass::Static,
    })
}

fn build_protected_file_snapshot() -> AdvancedEditorSnapshot {
    let surface = EditorSurfaceClass::ProtectedFile;
    // Writes require staged review, so a direct multi-caret edit is blocked.
    let selection_strip = build_selection_strip(SelectionStripInit {
        surface,
        mode_class: SelectionModeClass::MultiCursor,
        caret_count: 2,
        primary_caret_label: "Line 8, Col 3".to_owned(),
        quick_detail: "2 carets · protected path, writes require review".to_owned(),
        semantics_class: SelectionSemanticsClass::Blocked,
        column_edit_preview: None,
        undo_grouping_class: "no_direct_edit".to_owned(),
        unsupported_operations: vec![unsupported(
            "Edit at all carets",
            "This is a protected path; direct edits are blocked because writes require staged review.",
            "route:protected.request_approval",
        )],
    });
    let fold_summaries = vec![build_fold_summary(FoldSummaryInit {
        fold_id: "fold:protected:policy-block".to_owned(),
        label: "policy block".to_owned(),
        hidden_line_count: 14,
        hidden_state_counts: HiddenStateCounts {
            diagnostics: 0,
            conflicts: 1,
            trust_warnings: 1,
            search_hits: 0,
        },
    })];
    let overview_aids = both_overview_aids(
        surface,
        OrientationAidAvailability::Available,
        None,
        OrientationAidAvailability::Available,
        None,
    );
    assemble_snapshot(SnapshotSpec {
        surface,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::ReadOnlyNoApply,
        degrade_label: "Read-only — writes require staged review",
        selection_strip,
        fold_summaries,
        overview_aids,
        density_tier: DensityTier::Comfortable,
        zoom_tier: ZoomTier::Standard,
        motion_class: MotionClass::Static,
    })
}

fn build_partial_index_snapshot() -> AdvancedEditorSnapshot {
    let surface = EditorSurfaceClass::PartialIndexState;
    // Selection is exact, but the fold's hidden diagnostics are partial while the
    // index builds, and the overview ruler's markers are pending.
    let selection_strip = build_selection_strip(SelectionStripInit {
        surface,
        mode_class: SelectionModeClass::MultiCursor,
        caret_count: 3,
        primary_caret_label: "Line 30, Col 5".to_owned(),
        quick_detail: "3 carets · same token".to_owned(),
        semantics_class: SelectionSemanticsClass::ExactAllSelections,
        column_edit_preview: None,
        undo_grouping_class: "multi_caret_text_edit_single_undo_group".to_owned(),
        unsupported_operations: Vec::new(),
    });
    let fold_summaries = vec![build_fold_summary(FoldSummaryInit {
        fold_id: "fold:partial-index:pipeline".to_owned(),
        label: "pipeline".to_owned(),
        hidden_line_count: 40,
        hidden_state_counts: HiddenStateCounts {
            diagnostics: 2,
            conflicts: 0,
            trust_warnings: 0,
            search_hits: 1,
        },
    })];
    let overview_aids = both_overview_aids(
        surface,
        OrientationAidAvailability::Reduced,
        Some("Minimap markers are partial while the index builds; counts may grow."),
        OrientationAidAvailability::Reduced,
        Some("Overview-ruler markers are pending while the index builds; Problems shows the current count."),
    );
    assemble_snapshot(SnapshotSpec {
        surface,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::PendingPartialIndex,
        degrade_label: "Pending — index still building",
        selection_strip,
        fold_summaries,
        overview_aids,
        density_tier: DensityTier::Comfortable,
        zoom_tier: ZoomTier::High,
        motion_class: MotionClass::Static,
    })
}

fn build_large_file_snapshot() -> AdvancedEditorSnapshot {
    let surface = EditorSurfaceClass::LargeFileRestricted;
    // Large-file / restricted mode: multi-cursor and folding are suppressed; only
    // the primary caret is available, and overview aids are disabled.
    let selection_strip = build_selection_strip(SelectionStripInit {
        surface,
        mode_class: SelectionModeClass::SingleCaret,
        caret_count: 1,
        primary_caret_label: "Line 1,204,990".to_owned(),
        quick_detail: "Single caret · multi-cursor suppressed in large-file mode".to_owned(),
        semantics_class: SelectionSemanticsClass::PrimaryCaretOnly,
        column_edit_preview: None,
        undo_grouping_class: "single_caret_text_edit".to_owned(),
        unsupported_operations: vec![unsupported(
            "Add cursor below",
            "Multi-cursor is suppressed in large-file / restricted mode to protect responsiveness; only the primary caret is available.",
            "route:search.find_in_file",
        )],
    });
    // Folding is suppressed in large-file mode; no fold summaries are produced.
    let fold_summaries = Vec::new();
    let overview_aids = both_overview_aids(
        surface,
        OrientationAidAvailability::DisabledLargeFile,
        Some("Minimap is disabled in large-file mode; Search and Go-to-line carry navigation."),
        OrientationAidAvailability::DisabledLargeFile,
        Some("Overview ruler is disabled in large-file mode; Problems and Search carry the state."),
    );
    assemble_snapshot(SnapshotSpec {
        surface,
        workspace_id: "workspace:demo",
        degrade_class: AssistDegradeClass::SuppressedLargeFile,
        degrade_label: "Suppressed — large-file mode",
        selection_strip,
        fold_summaries,
        overview_aids,
        density_tier: DensityTier::Comfortable,
        zoom_tier: ZoomTier::Standard,
        motion_class: MotionClass::Static,
    })
}

// ---------------------------------------------------------------------------
// Invariant evaluation.
// ---------------------------------------------------------------------------

fn evaluate_invariants(
    snapshots: &[AdvancedEditorSnapshot],
    render_awareness: &[RenderAwarenessPolicy],
) -> Vec<AdvancedEditingInvariant> {
    let strips: Vec<&SelectionSummaryStrip> =
        snapshots.iter().map(|s| &s.selection_strip).collect();
    let folds: Vec<&FoldRiskSummary> = snapshots
        .iter()
        .flat_map(|s| s.fold_summaries.iter())
        .collect();
    let aids: Vec<&OverviewAidParity> = snapshots
        .iter()
        .flat_map(|s| s.overview_aids.iter())
        .collect();

    let mut invariants = Vec::new();

    invariants.push(AdvancedEditingInvariant {
        invariant_id: "every_surface_resolves_a_snapshot".into(),
        statement: "Each claimed advanced editor surface resolves exactly one snapshot.".into(),
        holds: !snapshots.is_empty()
            && EditorSurfaceClass::ALL.iter().all(|surface| {
                snapshots
                    .iter()
                    .filter(|s| s.surface_class == *surface)
                    .count()
                    == 1
            }),
    });

    invariants.push(AdvancedEditingInvariant {
        invariant_id: "selection_semantics_always_disclosed".into(),
        statement: "Every selection strip names its semantics class so a user can tell whether \
                    the next operation is exact, normalized / expanded, primary-only, or blocked, \
                    and discloses any non-exact semantics."
            .into(),
        holds: strips
            .iter()
            .all(|strip| strip.semantics_disclosed_when_inexact()),
    });

    invariants.push(AdvancedEditingInvariant {
        invariant_id: "multi_caret_strips_show_count_and_primary".into(),
        statement: "Every multi-cursor / column strip shows a caret count, a primary caret, and \
                    an undo grouping class."
            .into(),
        holds: strips.iter().all(|strip| strip.count_and_primary_visible()),
    });

    invariants.push(AdvancedEditingInvariant {
        invariant_id: "unsupported_operations_explained".into(),
        statement: "Every narrowed / blocked selection explains at least one unsupported \
                    operation with a reason and a fallback route."
            .into(),
        holds: strips
            .iter()
            .all(|strip| strip.unsupported_operations_explained()),
    });

    invariants.push(AdvancedEditingInvariant {
        invariant_id: "folds_advertise_hidden_critical_state".into(),
        statement:
            "Every folded region hiding diagnostics / conflicts / trust warnings advertises \
                    it with a marker and a reveal route instead of falsely appearing clean."
                .into(),
        holds: folds.iter().all(|fold| fold.advertises_critical_state()),
    });

    invariants.push(AdvancedEditingInvariant {
        invariant_id: "fold_risk_class_matches_counts".into(),
        statement: "Every fold-risk class is derived correctly from its hidden-state counts."
            .into(),
        holds: folds.iter().all(|fold| fold.risk_class_matches_counts()),
    });

    invariants.push(AdvancedEditingInvariant {
        invariant_id: "every_fold_keyboard_toggleable".into(),
        statement: "Every fold summary stays keyboard-toggleable, screen-reader labeled, and \
                    carries a non-colour marker."
            .into(),
        holds: folds.iter().all(|fold| fold.keyboard_and_label_present()),
    });

    invariants.push(AdvancedEditingInvariant {
        invariant_id: "overview_aids_not_sole_carrier".into(),
        statement: "Every minimap / overview aid is an optional accelerator, never the sole \
                    carrier of critical state, and names replacement routes."
            .into(),
        holds: aids.iter().all(|aid| aid.not_sole_carrier()),
    });

    invariants.push(AdvancedEditingInvariant {
        invariant_id: "overview_aids_aligned_with_main_editor".into(),
        statement: "Every overview aid pins the shared main-editor marker-semantics source, so it \
                    cannot diverge into a second hidden truth model."
            .into(),
        holds: aids.iter().all(|aid| aid.aligned_with_main()),
    });

    invariants.push(AdvancedEditingInvariant {
        invariant_id: "degraded_aids_have_alternate_path".into(),
        statement: "Every reduced / disabled overview aid carries a visible message and names an \
                    alternate path (degrades honestly)."
            .into(),
        holds: aids.iter().all(|aid| aid.degrades_honestly()),
    });

    invariants.push(AdvancedEditingInvariant {
        invariant_id: "severity_state_non_color_only".into(),
        statement: "Every selection strip, fold summary, and overview aid carries a non-colour \
                    differentiator for its severity / actionability state."
            .into(),
        holds: strips
            .iter()
            .all(|strip| !strip.non_color_differentiator.trim().is_empty())
            && folds
                .iter()
                .all(|fold| !fold.non_color_marker.trim().is_empty())
            && aids
                .iter()
                .all(|aid| !aid.non_color_differentiator.trim().is_empty()),
    });

    invariants.push(AdvancedEditingInvariant {
        invariant_id: "render_awareness_preserves_critical_state".into(),
        statement: "Every density / zoom / motion policy preserves severity / actionability state \
                    and non-colour differentiation, compacting only optional chrome."
            .into(),
        holds: !render_awareness.is_empty()
            && render_awareness
                .iter()
                .all(|policy| policy.critical_state_preserved && policy.non_color_only_preserved),
    });

    invariants.push(AdvancedEditingInvariant {
        invariant_id: "snapshots_preserve_critical_state_in_profile".into(),
        statement: "Every snapshot preserves critical state under its captured density / zoom / \
                    motion profile."
            .into(),
        holds: snapshots
            .iter()
            .all(|s| s.critical_state_preserved_in_profile),
    });

    invariants.push(AdvancedEditingInvariant {
        invariant_id: "every_surface_screen_reader_meaningful".into(),
        statement: "Every selection strip, fold summary, and overview aid carries a non-empty \
                    screen-reader label."
            .into(),
        holds: strips
            .iter()
            .all(|strip| !strip.accessibility_label.trim().is_empty())
            && folds
                .iter()
                .all(|fold| !fold.accessibility_label.trim().is_empty())
            && aids
                .iter()
                .all(|aid| !aid.accessibility_label.trim().is_empty()),
    });

    invariants.push(AdvancedEditingInvariant {
        invariant_id: "degraded_surfaces_label_and_disclose".into(),
        statement: "Every surface that is not full fidelity carries a visible degrade label and \
                    flags disclosure."
            .into(),
        holds: snapshots
            .iter()
            .filter(|s| s.degrade_class != AssistDegradeClass::FullFidelity)
            .all(|s| !s.degrade_label.trim().is_empty() && s.disclosure_required),
    });

    invariants.push(AdvancedEditingInvariant {
        invariant_id: "every_surface_has_strip_and_overview_aids".into(),
        statement: "Every snapshot resolves a selection strip and at least one overview aid; fold \
                    summaries are present unless folding is suppressed."
            .into(),
        holds: snapshots
            .iter()
            .all(|s| !s.selection_strip.strip_id.trim().is_empty() && !s.overview_aids.is_empty()),
    });

    invariants.push(AdvancedEditingInvariant {
        invariant_id: "notebook_and_request_reuse_shared_vocabulary".into(),
        statement: "Notebook and request editors reuse the shared selection-strip, fold-summary, \
                    and overview-aid record kinds rather than forking their own semantics."
            .into(),
        holds: [
            EditorSurfaceClass::NotebookCell,
            EditorSurfaceClass::RequestEditor,
        ]
        .iter()
        .all(|surface| {
            snapshots
                .iter()
                .find(|s| s.surface_class == *surface)
                .is_some_and(|s| {
                    s.selection_strip.record_kind == SelectionSummaryStrip::RECORD_KIND
                        && s.fold_summaries
                            .iter()
                            .all(|f| f.record_kind == FoldRiskSummary::RECORD_KIND)
                        && s.overview_aids
                            .iter()
                            .all(|a| a.record_kind == OverviewAidParity::RECORD_KIND)
                })
        }),
    });

    invariants
}

#[cfg(test)]
mod tests;
