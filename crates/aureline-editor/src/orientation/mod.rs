//! Editor orientation-aid truth records for multi-cursor and navigation chrome.
//!
//! Orientation aids are optional accelerators. The records in this module make
//! their source, degraded state, keyboard route, and accessibility route
//! explicit so minimap, overview, fold, breadcrumb, and multi-cursor chrome do
//! not become a hidden truth model.

use serde::{Deserialize, Serialize};

use crate::outline::FoldVisibilityState;

/// Boundary schema version for [`EditorOrientationTruthRecord`].
pub const ORIENTATION_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Visibility and degraded-state vocabulary for optional orientation aids.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrientationAidAvailability {
    /// Aid is visible and current.
    Available,
    /// Aid is visible but simplified.
    Reduced,
    /// Aid is disabled by large-file mode.
    DisabledLargeFile,
    /// Aid is disabled by low-resource mode.
    DisabledLowResource,
    /// Aid is disabled by accessibility or user setting.
    DisabledBySetting,
}

impl OrientationAidAvailability {
    /// Returns the stable schema token for this availability state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Reduced => "reduced",
            Self::DisabledLargeFile => "disabled_large_file",
            Self::DisabledLowResource => "disabled_low_resource",
            Self::DisabledBySetting => "disabled_by_setting",
        }
    }
}

/// Optional visual aid kind used for coarse editor overview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverviewAidKind {
    /// Scaled document preview.
    Minimap,
    /// Thin semantic marker rail.
    OverviewRuler,
}

impl OverviewAidKind {
    /// Returns the stable schema token for this overview aid kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Minimap => "minimap",
            Self::OverviewRuler => "overview_ruler",
        }
    }
}

/// Hidden-state counts preserved by a fold summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenStateCounts {
    /// Hidden diagnostic count.
    pub diagnostics: u32,
    /// Hidden merge-conflict count.
    pub conflicts: u32,
    /// Hidden trust or policy warning count.
    pub trust_warnings: u32,
    /// Hidden search-hit count.
    pub search_hits: u32,
}

impl HiddenStateCounts {
    /// Returns true when the folded region contains critical hidden state.
    pub const fn has_critical_state(&self) -> bool {
        self.diagnostics > 0 || self.conflicts > 0 || self.trust_warnings > 0
    }
}

/// Multi-cursor or column-selection status strip record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiCursorIndicatorRecord {
    /// Stable indicator id.
    pub indicator_ref: String,
    /// Mode label such as multiple cursors or column selection.
    pub mode_label: String,
    /// Number of carets affected by the next edit.
    pub caret_count: usize,
    /// Primary caret label used at high zoom and by assistive technology.
    pub primary_caret_label: String,
    /// Undo grouping class for the next edit.
    pub undo_grouping_class: String,
    /// Unsupported or review note when a command cannot apply to every caret.
    pub unsupported_note: Option<String>,
    /// Keyboard command refs that reach equivalent actions.
    pub alternate_command_refs: Vec<String>,
    /// Screen-reader label for the indicator.
    pub accessibility_label: String,
}

/// Fold summary row that preserves hidden state and keyboard routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FoldSummaryRecord {
    /// Stable fold id.
    pub fold_id: String,
    /// Human-facing fold label.
    pub label: String,
    /// Current fold visibility state.
    pub visibility_state: FoldVisibilityState,
    /// Number of hidden physical lines.
    pub hidden_line_count: usize,
    /// Hidden critical-state counts.
    pub hidden_state_counts: HiddenStateCounts,
    /// Keyboard command id or route for toggling the fold.
    pub keyboard_toggle_command: String,
    /// Detail route that reveals hidden state.
    pub detail_route_ref: String,
    /// Screen-reader label for the fold summary.
    pub accessibility_label: String,
}

/// Breadcrumb continuity and partial-index truth record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreadcrumbContinuityRecord {
    /// Stable breadcrumb record id.
    pub breadcrumb_ref: String,
    /// File identity ref used by path breadcrumbs.
    pub file_identity_ref: String,
    /// Symbol path freshness state.
    pub symbol_path_state: String,
    /// True when navigation preserves back and forward continuity.
    pub back_forward_preserved: bool,
    /// Keyboard routes that expose the same path and index state.
    pub alternate_command_refs: Vec<String>,
    /// Visible message when the path is partial or degraded.
    pub visible_state_note: String,
    /// Screen-reader label for the breadcrumb state.
    pub accessibility_label: String,
}

/// Minimap or overview-ruler availability and marker truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverviewAidRecord {
    /// Stable overview aid id.
    pub aid_ref: String,
    /// Minimap or overview ruler.
    pub aid_kind: OverviewAidKind,
    /// Current availability.
    pub availability: OrientationAidAvailability,
    /// Visible degraded-state note.
    pub degraded_state_message: String,
    /// Shared marker semantics source ref.
    pub marker_semantics_ref: String,
    /// Keyboard routes that expose equivalent critical state.
    pub replacement_route_refs: Vec<String>,
    /// Screen-reader label for the aid.
    pub accessibility_label: String,
}

/// Input used to build a bounded alpha orientation truth record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlphaOrientationInput {
    /// Stable record id.
    pub orientation_record_id: String,
    /// Document ref represented by the orientation aids.
    pub document_ref: String,
    /// Editor surface ref.
    pub surface_ref: String,
    /// Low-resource posture used by overview degradation.
    pub low_resource_mode: bool,
}

/// Canonical editor orientation-aid state for help, accessibility, and support.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorOrientationTruthRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this record.
    pub schema_version: u32,
    /// Stable orientation record id.
    pub orientation_record_id: String,
    /// Document ref represented by the aids.
    pub document_ref: String,
    /// Editor surface ref.
    pub surface_ref: String,
    /// Multi-cursor status strip.
    pub multi_cursor: MultiCursorIndicatorRecord,
    /// Fold summary rows.
    pub fold_summaries: Vec<FoldSummaryRecord>,
    /// Breadcrumb continuity state.
    pub breadcrumbs: BreadcrumbContinuityRecord,
    /// One minimap or overview-ruler aid.
    pub overview_aid: OverviewAidRecord,
    /// Support-safe packet refs that explain the state.
    pub support_export_refs: Vec<String>,
}

impl EditorOrientationTruthRecord {
    /// Stable record-kind tag carried in serialized orientation records.
    pub const RECORD_KIND: &'static str = "editor_orientation_truth_record";

    /// Returns true when multiple carets have a visible count and undo grouping.
    pub fn multi_cursor_count_is_visible(&self) -> bool {
        self.multi_cursor.caret_count > 1
            && !self.multi_cursor.mode_label.trim().is_empty()
            && !self.multi_cursor.undo_grouping_class.trim().is_empty()
    }

    /// Returns true when fold summaries preserve hidden critical-state cues.
    pub fn fold_summaries_preserve_hidden_state(&self) -> bool {
        self.fold_summaries.iter().all(|fold| {
            fold.hidden_line_count > 0
                && !fold.keyboard_toggle_command.trim().is_empty()
                && !fold.accessibility_label.trim().is_empty()
                && (!fold.hidden_state_counts.has_critical_state()
                    || !fold.detail_route_ref.trim().is_empty())
        })
    }

    /// Returns true when breadcrumb jumps preserve continuity and fallback routes.
    pub fn breadcrumbs_preserve_continuity(&self) -> bool {
        self.breadcrumbs.back_forward_preserved
            && !self.breadcrumbs.alternate_command_refs.is_empty()
            && !self.breadcrumbs.visible_state_note.trim().is_empty()
    }

    /// Returns true when reduced or disabled visual aids name an alternate route.
    pub fn overview_degradation_has_alternate_path(&self) -> bool {
        let narrowed = !matches!(
            self.overview_aid.availability,
            OrientationAidAvailability::Available
        );
        !narrowed
            || (!self.overview_aid.degraded_state_message.trim().is_empty()
                && !self.overview_aid.replacement_route_refs.is_empty()
                && !self.overview_aid.accessibility_label.trim().is_empty())
    }
}

/// Builds the bounded alpha orientation-aid record for a source editor.
pub fn build_alpha_orientation_truth_record(
    input: AlphaOrientationInput,
) -> EditorOrientationTruthRecord {
    let availability = if input.low_resource_mode {
        OrientationAidAvailability::DisabledLowResource
    } else {
        OrientationAidAvailability::Reduced
    };
    let degraded_state_message = if input.low_resource_mode {
        "Overview aid is disabled in low-resource mode; Problems, Search, breadcrumbs, and outline expose the same critical state."
    } else {
        "Overview aid is reduced; marker semantics still come from the main editor sources."
    };

    EditorOrientationTruthRecord {
        record_kind: EditorOrientationTruthRecord::RECORD_KIND.to_string(),
        schema_version: ORIENTATION_TRUTH_SCHEMA_VERSION,
        orientation_record_id: input.orientation_record_id,
        document_ref: input.document_ref,
        surface_ref: input.surface_ref,
        multi_cursor: MultiCursorIndicatorRecord {
            indicator_ref: "multi-cursor:alpha:three-carets".to_string(),
            mode_label: "Multiple cursors".to_string(),
            caret_count: 3,
            primary_caret_label: "Line 12".to_string(),
            undo_grouping_class: "multi_cursor_text_edit_single_undo_group".to_string(),
            unsupported_note: Some(
                "Refactor preview is required before applying a structural action to all carets."
                    .to_string(),
            ),
            alternate_command_refs: vec![
                "cmd:editor.undo".to_string(),
                "cmd:command_palette.open".to_string(),
            ],
            accessibility_label: "Multiple cursors active, three carets".to_string(),
        },
        fold_summaries: vec![
            FoldSummaryRecord {
                fold_id: "fold:orders-controller:alpha".to_string(),
                label: "OrdersController".to_string(),
                visibility_state: FoldVisibilityState::Folded,
                hidden_line_count: 48,
                hidden_state_counts: HiddenStateCounts {
                    diagnostics: 1,
                    conflicts: 0,
                    trust_warnings: 1,
                    search_hits: 3,
                },
                keyboard_toggle_command: "route:editor.fold.toggle".to_string(),
                detail_route_ref: "route:problems.reveal_fold_hidden_state".to_string(),
                accessibility_label:
                    "OrdersController folded, forty eight hidden lines, one diagnostic and one trust warning inside."
                        .to_string(),
            },
            FoldSummaryRecord {
                fold_id: "fold:generated-region:alpha".to_string(),
                label: "Generated region".to_string(),
                visibility_state: FoldVisibilityState::SummaryOnly,
                hidden_line_count: 22,
                hidden_state_counts: HiddenStateCounts {
                    diagnostics: 0,
                    conflicts: 1,
                    trust_warnings: 0,
                    search_hits: 0,
                },
                keyboard_toggle_command: "route:editor.fold.toggle".to_string(),
                detail_route_ref: "route:review.reveal_conflict_in_fold".to_string(),
                accessibility_label:
                    "Generated region summarized, twenty two hidden lines, one conflict inside."
                        .to_string(),
            },
        ],
        breadcrumbs: BreadcrumbContinuityRecord {
            breadcrumb_ref: "breadcrumb:alpha:orders-controller".to_string(),
            file_identity_ref: "file-identity:workspace:orders/src/controller.ts".to_string(),
            symbol_path_state: "partial_index_current_file_exact".to_string(),
            back_forward_preserved: true,
            alternate_command_refs: vec![
                "cmd:quick_open.toggle".to_string(),
                "cmd:command_palette.open".to_string(),
            ],
            visible_state_note:
                "File path is exact; symbol path is limited while workspace index warms."
                    .to_string(),
            accessibility_label:
                "Breadcrumb path exact for file, partial for symbols, navigation history preserved."
                    .to_string(),
        },
        overview_aid: OverviewAidRecord {
            aid_ref: "overview-aid:alpha:minimap".to_string(),
            aid_kind: OverviewAidKind::Minimap,
            availability,
            degraded_state_message: degraded_state_message.to_string(),
            marker_semantics_ref: "docs/ux/editor_viewport_summary_contract.md#source-authority"
                .to_string(),
            replacement_route_refs: vec![
                "route:problems.panel".to_string(),
                "route:search.results".to_string(),
                "route:outline.current_file".to_string(),
            ],
            accessibility_label:
                "Minimap reduced; Problems, Search, and Outline provide equivalent state."
                    .to_string(),
        },
        support_export_refs: vec![
            "artifacts/commands/alpha_mode_state_parity_report.json".to_string(),
            "fixtures/editor/mode_and_orientation/alpha_mode_and_orientation_cases.json"
                .to_string(),
            "docs/ux/editor_viewport_summary_contract.md".to_string(),
        ],
    }
}
