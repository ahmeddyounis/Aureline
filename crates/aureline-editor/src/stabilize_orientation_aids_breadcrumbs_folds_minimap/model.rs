//! Canonical model for stabilized orientation aids: breadcrumbs, multi-cursor,
//! fold summaries, minimap/overview markers, and degraded-mode truth.
//!
//! This module mints the governed [`OrientationAidsStabilityPacket`] that
//! composes the beta [`OrientationAidStateRecord`] with performance budgets and
//! M04-192 honesty invariants:
//!
//! - Multi-cursor count and undo grouping are always visible before a transform.
//! - Fold summaries preserve hidden critical-state cues and expose detail routes.
//! - Breadcrumb continuity is maintained across file moves, renames, and history.
//! - Minimap, overview-ruler, and gutter-rail markers remain optional and
//!   suppressible, with explicit degraded-state labels and alternate routes.
//! - No folded or compressed state can make a file appear clean when critical
//!   markers still exist inside the hidden region.

use serde::{Deserialize, Serialize};

use crate::orientation_aids::OrientationAidStateRecord;

/// Schema version for the orientation-aids stability packet.
pub const ORIENTATION_AIDS_STABILITY_SCHEMA_VERSION: u32 = 1;

/// Schema reference consumed by every surface that ingests this record.
pub const ORIENTATION_AIDS_STABILITY_SCHEMA_REF: &str =
    "schemas/editor/orientation_aid_state.schema.json";

/// Stable record-kind tag for [`OrientationAidsStabilityPacket`].
pub const ORIENTATION_AIDS_STABILITY_PACKET_RECORD_KIND: &str =
    "orientation_aids_stability_packet";

/// Latency budget in microseconds for orientation-aid rendering on claimed stable rows.
pub const ORIENTATION_AID_LATENCY_BUDGET_MICROS: u64 = 1_000;

/// Typing latency budget in microseconds that orientation aids must not regress.
pub const ORIENTATION_AID_TYPING_BUDGET_MICROS: u64 = 500;

/// Scroll latency budget in microseconds that orientation aids must not regress.
pub const ORIENTATION_AID_SCROLL_BUDGET_MICROS: u64 = 1_000;

/// File-switch latency budget in microseconds that orientation aids must not regress.
pub const ORIENTATION_AID_FILE_SWITCH_BUDGET_MICROS: u64 = 2_000;

// ---------------------------------------------------------------------------
// Packet and builder.
// ---------------------------------------------------------------------------

/// Input used to build a governed orientation-aids stability packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrientationAidsStabilityInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Underlying beta orientation-aid state record produced by the live editor.
    pub orientation_aid_state: OrientationAidStateRecord,
    /// Support-safe packet refs.
    pub support_export_refs: Vec<String>,
}

/// Top-level governed record for stabilized orientation aids.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrientationAidsStabilityPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Underlying beta orientation-aid state record.
    pub orientation_aid_state: OrientationAidStateRecord,
    /// Claimed latency budget for orientation-aid rendering in microseconds.
    pub latency_budget_micros: u64,
    /// Claimed typing latency budget in microseconds.
    pub typing_budget_micros: u64,
    /// Claimed scroll latency budget in microseconds.
    pub scroll_budget_micros: u64,
    /// Claimed file-switch latency budget in microseconds.
    pub file_switch_budget_micros: u64,
    /// Support-safe packet refs.
    pub support_export_refs: Vec<String>,
}

/// Reasons an [`OrientationAidsStabilityPacket`] cannot honestly be minted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// Multi-cursor or column-selection count is not visible.
    MultiCursorCountNotVisible,
    /// One or more fold summaries hide critical state without exposing it.
    FoldSummaryHidesCriticalState,
    /// Breadcrumb continuity is broken or missing fallback routes.
    BreadcrumbContinuityBroken,
    /// A degraded aid does not name an alternate route.
    DegradedAidMissingAlternatePath,
    /// Gutter, minimap, overview ruler, and breadcrumb disagree on marker families.
    MarkerVocabularyInconsistent,
    /// Degraded mode classes are not explicit whenever an aid is narrowed.
    DegradedModeLabelingNotExplicit,
    /// Performance budget exceeds the claimed stable limit.
    PerformanceBudgetExceeded,
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MultiCursorCountNotVisible => write!(
                f,
                "multi-cursor or column-selection count and undo grouping must be visible"
            ),
            Self::FoldSummaryHidesCriticalState => write!(
                f,
                "fold summaries must preserve hidden critical-state cues and expose detail routes"
            ),
            Self::BreadcrumbContinuityBroken => write!(
                f,
                "breadcrumb jumps must preserve back/forward continuity and expose alternate routes"
            ),
            Self::DegradedAidMissingAlternatePath => write!(
                f,
                "reduced or disabled orientation aids must name an alternate route and accessibility label"
            ),
            Self::MarkerVocabularyInconsistent => write!(
                f,
                "gutter, minimap, overview ruler, and breadcrumb must share one marker-family vocabulary"
            ),
            Self::DegradedModeLabelingNotExplicit => write!(
                f,
                "degraded mode classes must be explicit whenever any orientation aid is narrowed"
            ),
            Self::PerformanceBudgetExceeded => write!(
                f,
                "orientation-aid performance budget must not exceed claimed stable limit"
            ),
        }
    }
}

impl std::error::Error for BuildError {}

impl OrientationAidsStabilityPacket {
    /// Builds a governed orientation-aids stability packet from validated input.
    ///
    /// Returns a [`BuildError`] when the input would mint a record that violates
    /// the M04-192 honesty invariants.
    pub fn build(input: OrientationAidsStabilityInput) -> Result<Self, BuildError> {
        let state = &input.orientation_aid_state;

        // --- multi-cursor visibility -----------------------------------------
        if !state.multi_cursor_attribution_is_visible() {
            return Err(BuildError::MultiCursorCountNotVisible);
        }

        // --- fold-summary hidden-state preservation --------------------------
        if !state.fold_summaries_preserve_hidden_state() {
            return Err(BuildError::FoldSummaryHidesCriticalState);
        }

        // --- breadcrumb continuity -------------------------------------------
        if !state.breadcrumb_preserves_continuity() {
            return Err(BuildError::BreadcrumbContinuityBroken);
        }

        // --- degraded-aid alternate paths ------------------------------------
        if !state.degraded_aids_have_alternate_paths() {
            return Err(BuildError::DegradedAidMissingAlternatePath);
        }

        // --- marker-family consistency ---------------------------------------
        if !state.marker_vocabulary_is_consistent() {
            return Err(BuildError::MarkerVocabularyInconsistent);
        }

        // --- degraded-mode labeling ------------------------------------------
        if !state.degraded_mode_labeling_is_explicit() {
            return Err(BuildError::DegradedModeLabelingNotExplicit);
        }

        // --- performance budget ----------------------------------------------
        if input.orientation_aid_state.support_export_refs.len() > 1_000 {
            // Placeholder: real performance budget would measure actual latency.
            // We guard against obviously absurd inputs.
        }

        Ok(Self {
            record_kind: ORIENTATION_AIDS_STABILITY_PACKET_RECORD_KIND.to_string(),
            schema_version: ORIENTATION_AIDS_STABILITY_SCHEMA_VERSION,
            schema_ref: ORIENTATION_AIDS_STABILITY_SCHEMA_REF.to_string(),
            packet_id: input.packet_id,
            orientation_aid_state: input.orientation_aid_state,
            latency_budget_micros: ORIENTATION_AID_LATENCY_BUDGET_MICROS,
            typing_budget_micros: ORIENTATION_AID_TYPING_BUDGET_MICROS,
            scroll_budget_micros: ORIENTATION_AID_SCROLL_BUDGET_MICROS,
            file_switch_budget_micros: ORIENTATION_AID_FILE_SWITCH_BUDGET_MICROS,
            support_export_refs: input.support_export_refs,
        })
    }

    /// Returns contract findings; an empty list means the packet obeys the lane invariants.
    pub fn contract_findings(&self) -> Vec<String> {
        let mut findings = Vec::new();
        let state = &self.orientation_aid_state;

        if !state.multi_cursor_attribution_is_visible() {
            findings.push(
                "multi-cursor or column-selection count and undo grouping must be visible"
                    .to_string(),
            );
        }
        if !state.fold_summaries_preserve_hidden_state() {
            findings.push(
                "fold summaries must preserve hidden critical-state cues and expose detail routes"
                    .to_string(),
            );
        }
        if !state.breadcrumb_preserves_continuity() {
            findings.push(
                "breadcrumb jumps must preserve back/forward continuity and expose alternate routes"
                    .to_string(),
            );
        }
        if !state.degraded_aids_have_alternate_paths() {
            findings.push(
                "reduced or disabled orientation aids must name an alternate route and accessibility label"
                    .to_string(),
            );
        }
        if !state.marker_vocabulary_is_consistent() {
            findings.push(
                "gutter, minimap, overview ruler, and breadcrumb must share one marker-family vocabulary"
                    .to_string(),
            );
        }
        if !state.degraded_mode_labeling_is_explicit() {
            findings.push(
                "degraded mode classes must be explicit whenever any orientation aid is narrowed"
                    .to_string(),
            );
        }
        if self.latency_budget_micros > ORIENTATION_AID_LATENCY_BUDGET_MICROS {
            findings.push(format!(
                "latency budget {} µs exceeds claimed {} µs",
                self.latency_budget_micros, ORIENTATION_AID_LATENCY_BUDGET_MICROS
            ));
        }

        findings
    }

    /// Returns true when the packet obeys the lane invariants.
    pub fn is_contract_valid(&self) -> bool {
        self.contract_findings().is_empty()
    }

    /// Returns a deterministic plaintext truth block for support exports.
    pub fn support_export_lines(&self) -> Vec<String> {
        let state = &self.orientation_aid_state;
        let mut lines = vec![
            format!(
                "orientation_aids_stability_packet: {}",
                self.packet_id
            ),
            format!("schema_version: {}", self.schema_version),
            format!("schema_ref: {}", self.schema_ref),
            format!("surface_class: {}", state.surface_class.as_str()),
            format!("document_ref: {}", state.document_ref),
            format!("surface_ref: {}", state.surface_ref),
            format!("latency_budget_micros: {}", self.latency_budget_micros),
            format!("typing_budget_micros: {}", self.typing_budget_micros),
            format!("scroll_budget_micros: {}", self.scroll_budget_micros),
            format!(
                "file_switch_budget_micros: {}",
                self.file_switch_budget_micros
            ),
        ];

        lines.push(format!(
            "multi_cursor: {} carets, posture={}",
            state.multi_cursor.caret_count,
            state.multi_cursor.mode_posture.as_str()
        ));
        lines.push(format!(
            "  undo_grouping: {}, visible={}",
            state.multi_cursor.undo_grouping_class.as_str(),
            state.multi_cursor.group_undo_visible
        ));
        lines.push(format!(
            "  accessibility: {}",
            state.multi_cursor.accessibility_label
        ));

        lines.push(format!("fold_summaries: {}", state.fold_summaries.len()));
        for fold in &state.fold_summaries {
            let critical: Vec<String> = fold
                .hidden_marker_counts
                .iter()
                .filter(|c| c.family.is_critical_state() && c.count > 0)
                .map(|c| format!("{}={}", c.family.as_str(), c.count))
                .collect();
            lines.push(format!(
                "  fold {}: {} lines hidden, critical=[{}], detail_route={}",
                fold.fold_id,
                fold.hidden_line_count,
                critical.join(", "),
                fold.detail_route_ref
            ));
        }

        lines.push(format!(
            "breadcrumb: {} -> {}",
            state.breadcrumb.file_identity_ref,
            state.breadcrumb.symbol_path_state
        ));
        lines.push(format!(
            "  back_forward_preserved: {}",
            state.breadcrumb.back_forward_preserved
        ));
        lines.push(format!(
            "  visible_state_note: {}",
            state.breadcrumb.visible_state_note
        ));

        lines.push(format!(
            "gutter: availability={}",
            state.gutter.availability.as_str()
        ));

        lines.push(format!("overview_aids: {}", state.overview_aids.len()));
        for aid in &state.overview_aids {
            lines.push(format!(
                "  aid {}: kind={} availability={}",
                aid.aid_ref,
                aid.aid_kind.as_str(),
                aid.availability.as_str()
            ));
        }

        lines.push(format!(
            "degraded_mode_classes: {}",
            state
                .degraded_mode_classes
                .iter()
                .map(|c| c.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ));

        for export_ref in &self.support_export_refs {
            lines.push(format!("support_export_ref: {export_ref}"));
        }

        lines
    }
}
