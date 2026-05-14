//! Settings projection rows for keyboard-mode and orientation truth.
//!
//! The settings crate stores only the transport-safe projection shape. Editor
//! and shell consumers provide the source mode, register, macro, and
//! orientation records.

use serde::{Deserialize, Serialize};

/// Boundary schema version for [`ModeStateSettingsInspectionRecord`].
pub const MODE_STATE_SETTINGS_SCHEMA_VERSION: u32 = 1;

/// One register or clipboard route projected into settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeStateSettingsRouteRow {
    /// Stable route ref from the editor mode-state record.
    pub route_ref: String,
    /// Canonical route kind token.
    pub route_kind: String,
    /// Availability token.
    pub availability: String,
    /// True when the route is blocked or unsupported and may not approximate.
    pub fail_closed: bool,
    /// Export-safe visible reason.
    pub visible_reason: String,
}

/// Macro replay safety row projected into settings and support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeStateSettingsMacroRow {
    /// Stable macro review ref.
    pub review_ref: String,
    /// Outcome class such as requires_review or rejected.
    pub outcome_class: String,
    /// Side-effect classes touched by the replay.
    pub write_classes_touched: Vec<String>,
    /// Export-safe visible reason.
    pub visible_reason: String,
}

/// Orientation-aid summary projected alongside keyboard-mode settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeStateOrientationSettingsSummary {
    /// Number of carets affected by the next edit.
    pub multi_cursor_count: usize,
    /// True when fold summaries expose hidden critical state and keyboard routes.
    pub fold_summary_truth_preserved: bool,
    /// Breadcrumb partial or exact state token.
    pub breadcrumb_state: String,
    /// Minimap or overview availability token.
    pub overview_availability: String,
    /// Alternate command or route refs that expose equivalent critical state.
    pub alternate_route_refs: Vec<String>,
}

/// Settings-row projection for one active preset mode-state surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeStateSettingsInspectionRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version for the settings projection.
    pub schema_version: u32,
    /// Stable settings row id.
    pub row_id: String,
    /// Source preset ref whose key semantics are active.
    pub source_preset_ref: String,
    /// Current user-visible editor mode token.
    pub current_mode: String,
    /// Surface whose modal/keymap behavior is projected.
    pub surface_ref: String,
    /// Sequence state tokens visible for the surface.
    pub sequence_states: Vec<String>,
    /// Register and clipboard route rows visible before paste or replay.
    pub register_routes: Vec<ModeStateSettingsRouteRow>,
    /// Macro safety rows.
    pub macro_safety_rows: Vec<ModeStateSettingsMacroRow>,
    /// Orientation-aid truth summary.
    pub orientation: ModeStateOrientationSettingsSummary,
    /// Recovery actions reachable from settings/help.
    pub recovery_action_refs: Vec<String>,
    /// Support-safe packet refs.
    pub support_export_refs: Vec<String>,
}

impl ModeStateSettingsInspectionRecord {
    /// Creates a settings projection row with the stable record discriminator.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        row_id: impl Into<String>,
        source_preset_ref: impl Into<String>,
        current_mode: impl Into<String>,
        surface_ref: impl Into<String>,
        sequence_states: Vec<String>,
        register_routes: Vec<ModeStateSettingsRouteRow>,
        macro_safety_rows: Vec<ModeStateSettingsMacroRow>,
        orientation: ModeStateOrientationSettingsSummary,
        recovery_action_refs: Vec<String>,
        support_export_refs: Vec<String>,
    ) -> Self {
        Self {
            record_kind: "mode_state_settings_inspection_record".to_string(),
            schema_version: MODE_STATE_SETTINGS_SCHEMA_VERSION,
            row_id: row_id.into(),
            source_preset_ref: source_preset_ref.into(),
            current_mode: current_mode.into(),
            surface_ref: surface_ref.into(),
            sequence_states,
            register_routes,
            macro_safety_rows,
            orientation,
            recovery_action_refs,
            support_export_refs,
        }
    }

    /// Returns true when settings can explain all blocked routes and recovery.
    pub fn explains_blocked_routes_and_recovery(&self) -> bool {
        self.register_routes
            .iter()
            .all(|route| !route.fail_closed || !route.visible_reason.trim().is_empty())
            && !self.recovery_action_refs.is_empty()
    }
}
