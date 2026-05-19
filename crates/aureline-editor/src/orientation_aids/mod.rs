//! Beta editor-orientation truth across editor, diff, and review surfaces.
//!
//! Where the [`crate::orientation`] module froze the bounded alpha record for
//! help, accessibility, and support flows, this module promotes orientation
//! aids to a single beta truth model that editor, diff, and review surfaces
//! all project from. Multi-cursor state, fold summaries, breadcrumbs, gutter
//! markers, and minimap or overview-ruler chrome share one marker vocabulary;
//! degraded postures stay explicit; and back/forward continuity, keyboard
//! routes, and accessibility labels remain available even when visual aids are
//! reduced or disabled.

use serde::{Deserialize, Serialize};

use crate::outline::FoldVisibilityState;

/// Boundary schema version for [`OrientationAidStateRecord`].
pub const ORIENTATION_AID_STATE_SCHEMA_VERSION: u32 = 1;

/// Boundary schema version for [`FoldSummaryStateRecord`].
pub const FOLD_SUMMARY_STATE_SCHEMA_VERSION: u32 = 1;

/// Surface family that projects the orientation aid record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrientationSurfaceClass {
    /// Source-editor surface.
    EditorSource,
    /// Editor diff surface (working tree, staged, or three-way).
    EditorDiff,
    /// Review thread surface that anchors orientation aids over a buffer.
    ReviewThread,
}

impl OrientationSurfaceClass {
    /// Returns the stable schema token for this surface class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorSource => "editor_source",
            Self::EditorDiff => "editor_diff",
            Self::ReviewThread => "review_thread",
        }
    }
}

/// Closed marker-family vocabulary shared by gutter, minimap, overview ruler,
/// breadcrumbs, and accessible alternate routes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarkerFamilyClass {
    /// Diagnostic at error severity.
    DiagnosticError,
    /// Diagnostic at warning severity.
    DiagnosticWarning,
    /// Diagnostic at info severity.
    DiagnosticInfo,
    /// Diagnostic at hint severity.
    DiagnosticHint,
    /// Merge or rebase conflict cluster.
    MergeConflict,
    /// Staged hunk awaiting commit or review.
    StagedHunk,
    /// Working-tree change cluster.
    VcsChange,
    /// Active in-file search hit.
    SearchHit,
    /// Review thread or comment anchor.
    ReviewThread,
    /// Trust, policy, or restricted-mode warning.
    TrustOrPolicyWarning,
    /// Debugger breakpoint anchor.
    Breakpoint,
    /// Folded-region hidden-state marker.
    FoldHiddenState,
    /// Generated or read-only span marker.
    GeneratedOrReadOnly,
    /// Freshness or stale-marker callout.
    FreshnessOrStale,
}

impl MarkerFamilyClass {
    /// Returns the stable schema token for this marker family.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiagnosticError => "diagnostic_error",
            Self::DiagnosticWarning => "diagnostic_warning",
            Self::DiagnosticInfo => "diagnostic_info",
            Self::DiagnosticHint => "diagnostic_hint",
            Self::MergeConflict => "merge_conflict",
            Self::StagedHunk => "staged_hunk",
            Self::VcsChange => "vcs_change",
            Self::SearchHit => "search_hit",
            Self::ReviewThread => "review_thread",
            Self::TrustOrPolicyWarning => "trust_or_policy_warning",
            Self::Breakpoint => "breakpoint",
            Self::FoldHiddenState => "fold_hidden_state",
            Self::GeneratedOrReadOnly => "generated_or_read_only",
            Self::FreshnessOrStale => "freshness_or_stale",
        }
    }

    /// Returns true when the marker family represents critical hidden state
    /// that a fold summary or degraded orientation aid must still surface.
    pub const fn is_critical_state(self) -> bool {
        matches!(
            self,
            Self::DiagnosticError
                | Self::DiagnosticWarning
                | Self::MergeConflict
                | Self::TrustOrPolicyWarning
                | Self::StagedHunk
        )
    }
}

/// Availability vocabulary for one orientation aid, including the explicit
/// degraded-mode classes the beta editor surface must label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrientationAidAvailabilityClass {
    /// Aid is visible and current.
    Available,
    /// Aid is visible but simplified.
    Reduced,
    /// Aid is suppressed because the document is in large-file mode.
    DisabledLargeFile,
    /// Aid is suppressed because the host is in low-resource mode.
    DisabledLowResource,
    /// Aid is simplified or suppressed because reduced-motion is active.
    DisabledReducedMotion,
    /// Aid is simplified or suppressed because high-contrast or forced colors are active.
    DisabledHighContrast,
    /// Aid is suppressed because battery-saver is active.
    DisabledBatterySaver,
    /// Aid is suppressed because restricted mode is active.
    DisabledRestrictedMode,
    /// Aid is suppressed by user or workspace setting.
    DisabledBySetting,
}

impl OrientationAidAvailabilityClass {
    /// Returns the stable schema token for this availability class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Reduced => "reduced",
            Self::DisabledLargeFile => "disabled_large_file",
            Self::DisabledLowResource => "disabled_low_resource",
            Self::DisabledReducedMotion => "disabled_reduced_motion",
            Self::DisabledHighContrast => "disabled_high_contrast",
            Self::DisabledBatterySaver => "disabled_battery_saver",
            Self::DisabledRestrictedMode => "disabled_restricted_mode",
            Self::DisabledBySetting => "disabled_by_setting",
        }
    }

    /// Returns true when the aid is reduced or disabled and therefore requires
    /// an explicit degraded-state label and alternate route.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::Available)
    }
}

/// Multi-cursor or column-selection posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MultiCursorModePosture {
    /// One caret; no multi-cursor indicator required.
    SingleCaret,
    /// Multiple discrete carets active.
    MultipleCarets,
    /// Column or block selection produced multiple insertion points.
    ColumnSelection,
}

impl MultiCursorModePosture {
    /// Returns the stable schema token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleCaret => "single_caret",
            Self::MultipleCarets => "multiple_carets",
            Self::ColumnSelection => "column_selection",
        }
    }

    /// Returns true when the indicator must be visible because the next edit
    /// applies to more than one insertion point.
    pub const fn requires_indicator(self) -> bool {
        matches!(self, Self::MultipleCarets | Self::ColumnSelection)
    }
}

/// Undo-grouping class for the next edit applied through a multi-cursor or
/// column-selection posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UndoGroupingClass {
    /// Single-caret edit; one undo step per edit.
    SingleCaretEdit,
    /// All caret edits collapse into one undo group.
    MultiCursorSingleGroup,
    /// Structural action batched across carets through a reviewed transaction.
    StructuralBatchedTransaction,
    /// Operation cannot apply to every caret atomically; review is required.
    UnsupportedAtomicity,
}

impl UndoGroupingClass {
    /// Returns the stable schema token for this grouping class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleCaretEdit => "single_caret_edit",
            Self::MultiCursorSingleGroup => "multi_cursor_single_group",
            Self::StructuralBatchedTransaction => "structural_batched_transaction",
            Self::UnsupportedAtomicity => "unsupported_atomicity",
        }
    }
}

/// Overview aid kind. The same vocabulary covers minimap, overview ruler, the
/// shared gutter rail, and breadcrumb chrome that the beta record cross-checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverviewAidKindClass {
    /// Scaled document minimap.
    Minimap,
    /// Thin semantic marker rail.
    OverviewRuler,
    /// Shared gutter rail used by all surfaces.
    GutterRail,
    /// Breadcrumb row carrying file or symbol path identity.
    Breadcrumb,
}

impl OverviewAidKindClass {
    /// Returns the stable schema token for this overview aid kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Minimap => "minimap",
            Self::OverviewRuler => "overview_ruler",
            Self::GutterRail => "gutter_rail",
            Self::Breadcrumb => "breadcrumb",
        }
    }
}

/// One marker family count hidden inside a folded region.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenMarkerCount {
    /// Marker family hidden by the fold.
    pub family: MarkerFamilyClass,
    /// Number of markers of this family hidden by the fold.
    pub count: u32,
}

/// Multi-cursor or column-selection attribution record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiCursorAttributionRecord {
    /// Stable indicator id.
    pub indicator_ref: String,
    /// Posture of the next edit.
    pub mode_posture: MultiCursorModePosture,
    /// Number of carets affected by the next edit.
    pub caret_count: usize,
    /// Short label for the primary caret, used at high zoom and by assistive tech.
    pub primary_caret_label: String,
    /// True when the carets came from a column or block selection.
    pub column_mode_active: bool,
    /// Undo-grouping class for the next edit.
    pub undo_grouping_class: UndoGroupingClass,
    /// True when the grouped-undo posture is visibly attributed in chrome.
    pub group_undo_visible: bool,
    /// Optional reviewer note when a command cannot apply to every caret.
    pub unsupported_note: Option<String>,
    /// Keyboard command refs that reach equivalent actions.
    pub alternate_command_refs: Vec<String>,
    /// Screen-reader label for the indicator.
    pub accessibility_label: String,
}

/// Fold-summary record. Doubles as the schema for
/// [`schemas/editor/fold_summary_state.schema.json`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FoldSummaryStateRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this fold-summary record.
    pub schema_version: u32,
    /// Stable fold id.
    pub fold_id: String,
    /// Human-facing fold label.
    pub label: String,
    /// Current fold visibility state.
    pub visibility_state: FoldVisibilityState,
    /// Number of hidden physical lines.
    pub hidden_line_count: usize,
    /// Hidden marker counts by family.
    pub hidden_marker_counts: Vec<HiddenMarkerCount>,
    /// True when every critical family is still discoverable.
    pub critical_state_preserved: bool,
    /// Keyboard command id or route for toggling the fold.
    pub keyboard_toggle_command: String,
    /// Detail route that reveals hidden state.
    pub detail_route_ref: String,
    /// Route the editor returns focus to after the fold detail closes.
    pub focus_return_route_ref: String,
    /// Screen-reader label for the fold summary.
    pub accessibility_label: String,
}

impl FoldSummaryStateRecord {
    /// Stable record-kind tag carried in serialized fold-summary records.
    pub const RECORD_KIND: &'static str = "fold_summary_state_record";

    /// Returns true when the summary reports every critical marker family
    /// hidden in the fold and exposes the detail route.
    pub fn preserves_hidden_critical_state(&self) -> bool {
        let critical_hidden = self
            .hidden_marker_counts
            .iter()
            .any(|count| count.family.is_critical_state() && count.count > 0);
        let detail_route_present = !self.detail_route_ref.trim().is_empty()
            && !self.keyboard_toggle_command.trim().is_empty()
            && !self.accessibility_label.trim().is_empty();

        match (critical_hidden, self.critical_state_preserved) {
            (true, preserved) => preserved && detail_route_present,
            (false, _) => detail_route_present,
        }
    }
}

/// Breadcrumb continuity record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreadcrumbContinuityStateRecord {
    /// Stable breadcrumb record id.
    pub breadcrumb_ref: String,
    /// File identity ref used by the path row.
    pub file_identity_ref: String,
    /// Symbol path freshness state.
    pub symbol_path_state: String,
    /// True when navigation preserves back and forward continuity.
    pub back_forward_preserved: bool,
    /// Route the editor returns focus to after a breadcrumb jump closes.
    pub focus_return_route_ref: String,
    /// Keyboard routes that expose the same path and index state.
    pub alternate_command_refs: Vec<String>,
    /// Visible message when the path is partial or degraded.
    pub visible_state_note: String,
    /// Screen-reader label for the breadcrumb row.
    pub accessibility_label: String,
}

/// Gutter-marker record. The marker families listed here are the canonical
/// vocabulary every other orientation aid on this surface must reuse.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GutterMarkerStateRecord {
    /// Stable gutter rail id.
    pub gutter_ref: String,
    /// Marker families this gutter rail projects.
    pub marker_families: Vec<MarkerFamilyClass>,
    /// Current availability of the gutter rail.
    pub availability: OrientationAidAvailabilityClass,
    /// Visible degraded-state note.
    pub degraded_state_message: String,
    /// Keyboard or panel routes that expose equivalent critical state.
    pub alternate_route_refs: Vec<String>,
    /// Screen-reader label for the rail.
    pub accessibility_label: String,
}

/// Overview-aid state for minimap, overview ruler, breadcrumb, or gutter rail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverviewAidStateRecord {
    /// Stable overview aid id.
    pub aid_ref: String,
    /// Minimap, overview ruler, gutter rail, or breadcrumb row.
    pub aid_kind: OverviewAidKindClass,
    /// Marker families this aid projects.
    pub marker_families: Vec<MarkerFamilyClass>,
    /// Current availability.
    pub availability: OrientationAidAvailabilityClass,
    /// Visible degraded-state note.
    pub degraded_state_message: String,
    /// Keyboard or panel routes that expose equivalent critical state.
    pub replacement_route_refs: Vec<String>,
    /// Route the editor returns focus to after the aid is dismissed.
    pub focus_return_route_ref: String,
    /// Screen-reader label for the aid.
    pub accessibility_label: String,
}

/// Canonical beta orientation-aid state for editor, diff, and review surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrientationAidStateRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this record.
    pub schema_version: u32,
    /// Stable orientation state id.
    pub orientation_state_id: String,
    /// Surface class projecting the orientation aids.
    pub surface_class: OrientationSurfaceClass,
    /// Document ref represented by the aids.
    pub document_ref: String,
    /// Editor, diff, or review surface ref.
    pub surface_ref: String,
    /// Multi-cursor or column-selection attribution.
    pub multi_cursor: MultiCursorAttributionRecord,
    /// Fold summaries with hidden marker counts.
    pub fold_summaries: Vec<FoldSummaryStateRecord>,
    /// Breadcrumb continuity state.
    pub breadcrumb: BreadcrumbContinuityStateRecord,
    /// Shared gutter marker rail.
    pub gutter: GutterMarkerStateRecord,
    /// Minimap, overview ruler, and breadcrumb overview projections.
    pub overview_aids: Vec<OverviewAidStateRecord>,
    /// Marker families shared across gutter, minimap, overview, and breadcrumb.
    pub shared_marker_families: Vec<MarkerFamilyClass>,
    /// Degraded-mode classes currently active on this surface.
    pub degraded_mode_classes: Vec<OrientationAidAvailabilityClass>,
    /// Support-safe packet refs that explain the state.
    pub support_export_refs: Vec<String>,
}

impl OrientationAidStateRecord {
    /// Stable record-kind tag carried in serialized orientation aid records.
    pub const RECORD_KIND: &'static str = "orientation_aid_state_record";

    /// Returns true when multi-cursor or column-selection state is visibly
    /// attributable: count, posture, and undo grouping are all surfaced.
    pub fn multi_cursor_attribution_is_visible(&self) -> bool {
        let posture = self.multi_cursor.mode_posture;
        if !posture.requires_indicator() {
            return self.multi_cursor.caret_count <= 1;
        }

        let column_consistent = match posture {
            MultiCursorModePosture::ColumnSelection => self.multi_cursor.column_mode_active,
            _ => true,
        };

        self.multi_cursor.caret_count > 1
            && self.multi_cursor.group_undo_visible
            && !self.multi_cursor.primary_caret_label.trim().is_empty()
            && !self.multi_cursor.accessibility_label.trim().is_empty()
            && column_consistent
    }

    /// Returns true when every fold summary preserves hidden critical state.
    pub fn fold_summaries_preserve_hidden_state(&self) -> bool {
        self.fold_summaries
            .iter()
            .all(FoldSummaryStateRecord::preserves_hidden_critical_state)
    }

    /// Returns true when breadcrumb jumps preserve back/forward continuity and
    /// always expose an alternate keyboard route.
    pub fn breadcrumb_preserves_continuity(&self) -> bool {
        self.breadcrumb.back_forward_preserved
            && !self.breadcrumb.alternate_command_refs.is_empty()
            && !self.breadcrumb.focus_return_route_ref.trim().is_empty()
            && !self.breadcrumb.visible_state_note.trim().is_empty()
    }

    /// Returns true when every reduced or disabled overview/gutter aid names
    /// its degraded-state message and at least one alternate route.
    pub fn degraded_aids_have_alternate_paths(&self) -> bool {
        let gutter_ok = !self.gutter.availability.is_narrowed()
            || (!self.gutter.degraded_state_message.trim().is_empty()
                && !self.gutter.alternate_route_refs.is_empty()
                && !self.gutter.accessibility_label.trim().is_empty());

        let aids_ok = self.overview_aids.iter().all(|aid| {
            !aid.availability.is_narrowed()
                || (!aid.degraded_state_message.trim().is_empty()
                    && !aid.replacement_route_refs.is_empty()
                    && !aid.focus_return_route_ref.trim().is_empty()
                    && !aid.accessibility_label.trim().is_empty())
        });

        gutter_ok && aids_ok
    }

    /// Returns true when the gutter, minimap, overview ruler, and breadcrumb
    /// agree on the shared marker-family vocabulary.
    pub fn marker_vocabulary_is_consistent(&self) -> bool {
        let shared: std::collections::BTreeSet<_> = self.shared_marker_families.iter().collect();
        if shared.is_empty() {
            return false;
        }

        let gutter_subset = self
            .gutter
            .marker_families
            .iter()
            .all(|family| shared.contains(family));
        let overview_subset = self.overview_aids.iter().all(|aid| {
            aid.marker_families
                .iter()
                .all(|family| shared.contains(family))
        });

        gutter_subset && overview_subset
    }

    /// Returns true when degraded mode classes are explicit whenever any aid is
    /// reduced or disabled.
    pub fn degraded_mode_labeling_is_explicit(&self) -> bool {
        let any_narrowed = self.gutter.availability.is_narrowed()
            || self
                .overview_aids
                .iter()
                .any(|aid| aid.availability.is_narrowed());
        if !any_narrowed {
            return true;
        }

        if self.degraded_mode_classes.is_empty() {
            return false;
        }

        let mut covered = self
            .degraded_mode_classes
            .contains(&self.gutter.availability)
            || !self.gutter.availability.is_narrowed();
        for aid in &self.overview_aids {
            if aid.availability.is_narrowed() {
                covered = covered && self.degraded_mode_classes.contains(&aid.availability);
            }
        }

        covered
    }
}

/// Input used to project a beta orientation-aid state record onto one surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BetaOrientationAidInput {
    /// Stable record id.
    pub orientation_state_id: String,
    /// Surface family producing the record.
    pub surface_class: OrientationSurfaceClass,
    /// Document ref.
    pub document_ref: String,
    /// Surface ref.
    pub surface_ref: String,
    /// True when the host is in large-file mode.
    pub large_file_mode: bool,
    /// True when low-resource mode is active.
    pub low_resource_mode: bool,
    /// True when reduced-motion is active.
    pub reduced_motion: bool,
    /// True when high-contrast or forced colors are active.
    pub high_contrast: bool,
    /// True when battery-saver is active.
    pub battery_saver: bool,
    /// True when restricted mode is active.
    pub restricted_mode: bool,
}

/// Builds the beta orientation-aid state record for one surface. The shared
/// marker vocabulary, fold-summary critical-state preservation, and degraded
/// labeling are populated consistently across editor, diff, and review.
pub fn build_beta_orientation_aid_state_record(
    input: BetaOrientationAidInput,
) -> OrientationAidStateRecord {
    let availability = resolve_availability(&input);
    let degraded_message = degraded_message_for(availability);

    let mut degraded_mode_classes = Vec::new();
    if availability.is_narrowed() {
        degraded_mode_classes.push(availability);
    }

    let shared_marker_families = vec![
        MarkerFamilyClass::DiagnosticError,
        MarkerFamilyClass::DiagnosticWarning,
        MarkerFamilyClass::MergeConflict,
        MarkerFamilyClass::StagedHunk,
        MarkerFamilyClass::SearchHit,
        MarkerFamilyClass::ReviewThread,
        MarkerFamilyClass::TrustOrPolicyWarning,
        MarkerFamilyClass::FoldHiddenState,
    ];

    let multi_cursor = MultiCursorAttributionRecord {
        indicator_ref: "multi-cursor:beta:three-carets".into(),
        mode_posture: MultiCursorModePosture::MultipleCarets,
        caret_count: 3,
        primary_caret_label: "Line 12".into(),
        column_mode_active: false,
        undo_grouping_class: UndoGroupingClass::MultiCursorSingleGroup,
        group_undo_visible: true,
        unsupported_note: Some(
            "Refactor preview is required before applying a structural action to all carets."
                .into(),
        ),
        alternate_command_refs: vec!["cmd:editor.undo".into(), "cmd:command_palette.open".into()],
        accessibility_label: "Multiple cursors active, three carets, single undo group.".into(),
    };

    let fold_summaries = vec![
        FoldSummaryStateRecord {
            record_kind: FoldSummaryStateRecord::RECORD_KIND.into(),
            schema_version: FOLD_SUMMARY_STATE_SCHEMA_VERSION,
            fold_id: "fold:orders-controller:beta".into(),
            label: "OrdersController".into(),
            visibility_state: FoldVisibilityState::Folded,
            hidden_line_count: 48,
            hidden_marker_counts: vec![
                HiddenMarkerCount {
                    family: MarkerFamilyClass::DiagnosticError,
                    count: 1,
                },
                HiddenMarkerCount {
                    family: MarkerFamilyClass::TrustOrPolicyWarning,
                    count: 1,
                },
                HiddenMarkerCount {
                    family: MarkerFamilyClass::SearchHit,
                    count: 3,
                },
            ],
            critical_state_preserved: true,
            keyboard_toggle_command: "route:editor.fold.toggle".into(),
            detail_route_ref: "route:problems.reveal_fold_hidden_state".into(),
            focus_return_route_ref: "route:editor.source.focus".into(),
            accessibility_label:
                "OrdersController folded, forty eight hidden lines, one error and one trust warning inside.".into(),
        },
        FoldSummaryStateRecord {
            record_kind: FoldSummaryStateRecord::RECORD_KIND.into(),
            schema_version: FOLD_SUMMARY_STATE_SCHEMA_VERSION,
            fold_id: "fold:generated-region:beta".into(),
            label: "Generated region".into(),
            visibility_state: FoldVisibilityState::SummaryOnly,
            hidden_line_count: 22,
            hidden_marker_counts: vec![
                HiddenMarkerCount {
                    family: MarkerFamilyClass::MergeConflict,
                    count: 1,
                },
                HiddenMarkerCount {
                    family: MarkerFamilyClass::StagedHunk,
                    count: 2,
                },
            ],
            critical_state_preserved: true,
            keyboard_toggle_command: "route:editor.fold.toggle".into(),
            detail_route_ref: "route:review.reveal_conflict_in_fold".into(),
            focus_return_route_ref: "route:editor.source.focus".into(),
            accessibility_label:
                "Generated region summarized, twenty two hidden lines, one conflict and two staged hunks inside.".into(),
        },
    ];

    let breadcrumb = BreadcrumbContinuityStateRecord {
        breadcrumb_ref: "breadcrumb:beta:orders-controller".into(),
        file_identity_ref: "file-identity:workspace:orders/src/controller.ts".into(),
        symbol_path_state: "partial_index_current_file_exact".into(),
        back_forward_preserved: true,
        focus_return_route_ref: "route:editor.source.focus".into(),
        alternate_command_refs: vec![
            "cmd:quick_open.toggle".into(),
            "cmd:command_palette.open".into(),
        ],
        visible_state_note:
            "File path is exact; symbol path is limited while workspace index warms.".into(),
        accessibility_label:
            "Breadcrumb path exact for file, partial for symbols, navigation history preserved."
                .into(),
    };

    let gutter = GutterMarkerStateRecord {
        gutter_ref: "gutter:beta:source-editor".into(),
        marker_families: shared_marker_families.clone(),
        availability,
        degraded_state_message: degraded_message.to_string(),
        alternate_route_refs: vec![
            "route:problems.panel".into(),
            "route:search.results".into(),
            "route:review.thread_list".into(),
        ],
        accessibility_label:
            "Gutter rail projects diagnostic, conflict, change, search, and review state.".into(),
    };

    let overview_aids = vec![
        OverviewAidStateRecord {
            aid_ref: "overview-aid:beta:minimap".into(),
            aid_kind: OverviewAidKindClass::Minimap,
            marker_families: vec![
                MarkerFamilyClass::DiagnosticError,
                MarkerFamilyClass::DiagnosticWarning,
                MarkerFamilyClass::SearchHit,
                MarkerFamilyClass::ReviewThread,
                MarkerFamilyClass::FoldHiddenState,
            ],
            availability,
            degraded_state_message: degraded_message.to_string(),
            replacement_route_refs: vec![
                "route:problems.panel".into(),
                "route:search.results".into(),
                "route:outline.current_file".into(),
            ],
            focus_return_route_ref: "route:editor.source.focus".into(),
            accessibility_label:
                "Minimap mirrors diagnostic, search, review, and fold state; Problems, Search, and Outline remain canonical."
                    .into(),
        },
        OverviewAidStateRecord {
            aid_ref: "overview-aid:beta:overview-ruler".into(),
            aid_kind: OverviewAidKindClass::OverviewRuler,
            marker_families: vec![
                MarkerFamilyClass::DiagnosticError,
                MarkerFamilyClass::MergeConflict,
                MarkerFamilyClass::StagedHunk,
                MarkerFamilyClass::TrustOrPolicyWarning,
                MarkerFamilyClass::FoldHiddenState,
            ],
            availability,
            degraded_state_message: degraded_message.to_string(),
            replacement_route_refs: vec![
                "route:problems.panel".into(),
                "route:review.thread_list".into(),
                "route:source-control.changes".into(),
            ],
            focus_return_route_ref: "route:editor.source.focus".into(),
            accessibility_label:
                "Overview ruler mirrors error, conflict, staged, trust, and fold-hidden state ticks."
                    .into(),
        },
    ];

    OrientationAidStateRecord {
        record_kind: OrientationAidStateRecord::RECORD_KIND.into(),
        schema_version: ORIENTATION_AID_STATE_SCHEMA_VERSION,
        orientation_state_id: input.orientation_state_id,
        surface_class: input.surface_class,
        document_ref: input.document_ref,
        surface_ref: input.surface_ref,
        multi_cursor,
        fold_summaries,
        breadcrumb,
        gutter,
        overview_aids,
        shared_marker_families,
        degraded_mode_classes,
        support_export_refs: vec![
            "fixtures/editor/m3/orientation_aids/source_editor_beta.json".into(),
            "fixtures/editor/m3/orientation_aids/diff_surface_beta.json".into(),
            "fixtures/editor/m3/orientation_aids/review_surface_beta.json".into(),
            "docs/editor/m3/orientation_aids_beta.md".into(),
            "schemas/editor/orientation_aid_state.schema.json".into(),
            "schemas/editor/fold_summary_state.schema.json".into(),
        ],
    }
}

fn resolve_availability(input: &BetaOrientationAidInput) -> OrientationAidAvailabilityClass {
    if input.large_file_mode {
        OrientationAidAvailabilityClass::DisabledLargeFile
    } else if input.low_resource_mode {
        OrientationAidAvailabilityClass::DisabledLowResource
    } else if input.battery_saver {
        OrientationAidAvailabilityClass::DisabledBatterySaver
    } else if input.restricted_mode {
        OrientationAidAvailabilityClass::DisabledRestrictedMode
    } else if input.high_contrast {
        OrientationAidAvailabilityClass::DisabledHighContrast
    } else if input.reduced_motion {
        OrientationAidAvailabilityClass::DisabledReducedMotion
    } else {
        OrientationAidAvailabilityClass::Available
    }
}

fn degraded_message_for(availability: OrientationAidAvailabilityClass) -> &'static str {
    match availability {
        OrientationAidAvailabilityClass::Available =>
            "Orientation aids are visible; Problems, Search, Review, and Outline remain canonical.",
        OrientationAidAvailabilityClass::Reduced =>
            "Orientation aids are simplified; canonical Problems, Search, Review, and Outline routes expose the same state.",
        OrientationAidAvailabilityClass::DisabledLargeFile =>
            "Orientation aids are disabled in large-file mode; Problems, Search, Review, and Outline remain available for critical state.",
        OrientationAidAvailabilityClass::DisabledLowResource =>
            "Orientation aids are disabled in low-resource mode; Problems, Search, Review, and Outline remain available for critical state.",
        OrientationAidAvailabilityClass::DisabledReducedMotion =>
            "Orientation aids are simplified for reduced-motion; static Problems, Search, Review, and Outline routes carry the same state.",
        OrientationAidAvailabilityClass::DisabledHighContrast =>
            "Orientation aids are simplified for high-contrast or forced colors; non-color channels and Problems, Search, Review, and Outline preserve marker meaning.",
        OrientationAidAvailabilityClass::DisabledBatterySaver =>
            "Orientation aids are reduced under battery-saver; Problems, Search, Review, and Outline preserve critical state.",
        OrientationAidAvailabilityClass::DisabledRestrictedMode =>
            "Orientation aids are restricted by trust policy; Problems, Search, and Review still expose policy-allowed state.",
        OrientationAidAvailabilityClass::DisabledBySetting =>
            "Orientation aids are disabled by user or workspace setting; Problems, Search, Review, and Outline remain available.",
    }
}
