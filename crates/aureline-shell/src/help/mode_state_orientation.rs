//! Help projection for keyboard-mode safety and editor orientation truth.
//!
//! This module joins editor-owned mode/register/macro records, orientation
//! records, and settings projection rows into one support-safe packet that the
//! keybinding inspector can render.

use std::collections::BTreeSet;

use aureline_editor::{
    build_alpha_mode_state_record, build_alpha_orientation_truth_record, AlphaModeStateInput,
    AlphaOrientationInput, EditorModeClass, EditorModeStateRecord, EditorOrientationTruthRecord,
    MacroReplayOutcomeClass, RegisterRouteAvailability,
};
use aureline_input::keybindings::PlatformClass;
use aureline_input::presets::KeymapPresetId;
use aureline_settings::{
    ModeStateOrientationSettingsSummary, ModeStateSettingsInspectionRecord,
    ModeStateSettingsMacroRow, ModeStateSettingsRouteRow,
};
use serde::{Deserialize, Serialize};

/// Boundary schema version for [`AlphaModeOrientationReport`].
pub const ALPHA_MODE_ORIENTATION_REPORT_SCHEMA_VERSION: u32 = 1;

/// Top-level alpha keyboard-mode and orientation truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaModeOrientationReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version for this projection.
    pub schema_version: u32,
    /// Stable report id.
    pub report_id: String,
    /// Active platform used for shortcut copy.
    pub platform_class: String,
    /// Active preset selected by the visible shell inspector.
    pub active_preset_ref: String,
    /// Source artifacts consumed by this packet.
    pub source_refs: Vec<String>,
    /// Mode, sequence, register, and macro records for the claimed preset lanes.
    pub mode_state_records: Vec<EditorModeStateRecord>,
    /// Orientation-aid record for the bounded source editor surface.
    pub orientation_record: EditorOrientationTruthRecord,
    /// Settings rows that make the state inspectable outside raw logs.
    pub settings_projection_rows: Vec<ModeStateSettingsInspectionRecord>,
    /// Summary fields used by tests and support exports.
    pub summary: AlphaModeOrientationSummary,
}

/// Compact summary for the alpha keyboard-mode and orientation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaModeOrientationSummary {
    /// Number of preset lanes with mode state.
    pub preset_lane_count: usize,
    /// Number of distinct register route kinds disclosed.
    pub register_route_kind_count: usize,
    /// Number of blocked or unsupported routes that fail closed.
    pub fail_closed_route_count: usize,
    /// Number of macro reviews that require review or rejection.
    pub unsafe_macro_review_count: usize,
    /// Number of sequence guides in partial or unsupported states.
    pub narrowed_sequence_count: usize,
    /// Number of orientation checks that passed.
    pub orientation_truth_checks_passed: usize,
    /// Overall status token.
    pub status: String,
}

/// Materializes the alpha keyboard-mode and orientation truth packet.
pub fn materialize_alpha_mode_orientation_report(
    active_preset: KeymapPresetId,
    platform: PlatformClass,
) -> AlphaModeOrientationReport {
    let platform_class = platform_token(platform).to_string();
    let mode_state_records = KeymapPresetId::all()
        .into_iter()
        .map(|preset| {
            build_alpha_mode_state_record(AlphaModeStateInput {
                mode_state_id: format!("mode-state:alpha:{}", safe_ref(preset.preset_ref())),
                source_preset_ref: preset.preset_ref().to_string(),
                source_preset_label: preset.display_name().to_string(),
                current_mode: mode_for_preset(preset),
                surface_ref: "surface:editor.source.alpha".to_string(),
                platform_class: platform_class.clone(),
            })
        })
        .collect::<Vec<_>>();

    let orientation_record = build_alpha_orientation_truth_record(AlphaOrientationInput {
        orientation_record_id: "orientation:alpha:source-editor".to_string(),
        document_ref: "doc:alpha:launch-language-source".to_string(),
        surface_ref: "surface:editor.source.alpha".to_string(),
        low_resource_mode: true,
    });

    let settings_projection_rows = mode_state_records
        .iter()
        .map(|record| settings_projection_from(record, &orientation_record))
        .collect::<Vec<_>>();

    let register_route_kind_count = mode_state_records
        .iter()
        .flat_map(|record| record.register_routes.iter())
        .map(|route| route.route_kind.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let fail_closed_route_count = mode_state_records
        .iter()
        .flat_map(|record| record.register_routes.iter())
        .filter(|route| {
            matches!(
                route.availability,
                RegisterRouteAvailability::BlockedByPolicy | RegisterRouteAvailability::Unsupported
            ) && route.fail_closed
        })
        .count();
    let unsafe_macro_review_count = mode_state_records
        .iter()
        .flat_map(|record| record.macro_replay_reviews.iter())
        .filter(|review| {
            matches!(
                review.outcome_class,
                MacroReplayOutcomeClass::RequiresReview | MacroReplayOutcomeClass::Rejected
            )
        })
        .count();
    let narrowed_sequence_count = mode_state_records
        .iter()
        .flat_map(|record| record.sequence_guides.iter())
        .filter(|guide| {
            matches!(
                guide.sequence_state.as_str(),
                "partial" | "unsupported_surface"
            )
        })
        .count();
    let orientation_truth_checks_passed = [
        orientation_record.multi_cursor_count_is_visible(),
        orientation_record.fold_summaries_preserve_hidden_state(),
        orientation_record.breadcrumbs_preserve_continuity(),
        orientation_record.overview_degradation_has_alternate_path(),
    ]
    .into_iter()
    .filter(|passed| *passed)
    .count();
    let mode_state_pass = mode_state_records.iter().all(|record| {
        record.covers_required_register_routes()
            && record.blocked_or_unsupported_routes_fail_closed()
            && record.unsafe_macro_replays_are_bounded()
            && record.exposes_partial_and_unsupported_sequences()
            && record.has_required_recovery_paths()
    });
    let settings_pass = settings_projection_rows
        .iter()
        .all(ModeStateSettingsInspectionRecord::explains_blocked_routes_and_recovery);
    let status = if mode_state_pass && settings_pass && orientation_truth_checks_passed == 4 {
        "pass"
    } else {
        "needs_review"
    };

    AlphaModeOrientationReport {
        record_kind: "alpha_mode_orientation_report".to_string(),
        schema_version: ALPHA_MODE_ORIENTATION_REPORT_SCHEMA_VERSION,
        report_id: "mode-orientation:alpha:launch-wedge".to_string(),
        platform_class,
        active_preset_ref: active_preset.preset_ref().to_string(),
        source_refs: vec![
            "crates/aureline-editor/src/modes/mod.rs".to_string(),
            "crates/aureline-editor/src/orientation/mod.rs".to_string(),
            "crates/aureline-settings/src/keybindings/mode_state.rs".to_string(),
            "fixtures/editor/mode_and_orientation/alpha_mode_and_orientation_cases.json"
                .to_string(),
            "artifacts/commands/alpha_mode_state_parity_report.json".to_string(),
            "docs/ux/keyboard_mode_orientation_alpha.md".to_string(),
        ],
        mode_state_records,
        orientation_record,
        settings_projection_rows,
        summary: AlphaModeOrientationSummary {
            preset_lane_count: KeymapPresetId::all().len(),
            register_route_kind_count,
            fail_closed_route_count,
            unsafe_macro_review_count,
            narrowed_sequence_count,
            orientation_truth_checks_passed,
            status: status.to_string(),
        },
    }
}

/// Builds compact lines for the keybinding help inspector.
pub fn build_alpha_mode_orientation_lines(
    active_preset: KeymapPresetId,
    platform: PlatformClass,
) -> Vec<String> {
    let report = materialize_alpha_mode_orientation_report(active_preset, platform);
    let mut lines = vec![
        "".to_string(),
        format!(
            "Alpha mode and orientation truth - preset: {} - status: {}",
            report.active_preset_ref, report.summary.status
        ),
        format!(
            "Mode lanes: {} - register route kinds: {} - fail-closed routes: {}",
            report.summary.preset_lane_count,
            report.summary.register_route_kind_count,
            report.summary.fail_closed_route_count
        ),
        format!(
            "Macro safety rows: {} - narrowed sequences: {} - orientation checks: {}/4",
            report.summary.unsafe_macro_review_count,
            report.summary.narrowed_sequence_count,
            report.summary.orientation_truth_checks_passed
        ),
    ];

    if let Some(active) = report
        .mode_state_records
        .iter()
        .find(|record| record.source_preset_ref == report.active_preset_ref)
    {
        lines.push(format!(
            "- Mode strip: {} from {} with {} recovery actions",
            active.current_mode.as_str(),
            active.source_preset_ref,
            active.recovery_actions.len()
        ));
        for route in active
            .register_routes
            .iter()
            .filter(|route| route.fail_closed)
            .take(3)
        {
            lines.push(format!(
                "- Register route: {} - {} - {}",
                route.route_kind.as_str(),
                route.availability.as_str(),
                route.visible_reason
            ));
        }
    }

    lines.push(format!(
        "- Orientation: {} carets, overview {}, replacement routes={}",
        report.orientation_record.multi_cursor.caret_count,
        report.orientation_record.overview_aid.availability.as_str(),
        report
            .orientation_record
            .overview_aid
            .replacement_route_refs
            .join(",")
    ));

    lines
}

fn settings_projection_from(
    mode: &EditorModeStateRecord,
    orientation: &EditorOrientationTruthRecord,
) -> ModeStateSettingsInspectionRecord {
    ModeStateSettingsInspectionRecord::new(
        format!("mode-state-setting:{}", safe_ref(&mode.source_preset_ref)),
        mode.source_preset_ref.clone(),
        mode.current_mode.as_str().to_string(),
        mode.surface_ref.clone(),
        mode.sequence_guides
            .iter()
            .map(|guide| guide.sequence_state.as_str().to_string())
            .collect(),
        mode.register_routes
            .iter()
            .map(|route| ModeStateSettingsRouteRow {
                route_ref: route.route_ref.clone(),
                route_kind: route.route_kind.as_str().to_string(),
                availability: route.availability.as_str().to_string(),
                fail_closed: route.fail_closed,
                visible_reason: route.visible_reason.clone(),
            })
            .collect(),
        mode.macro_replay_reviews
            .iter()
            .map(|review| ModeStateSettingsMacroRow {
                review_ref: review.review_ref.clone(),
                outcome_class: review.outcome_class.as_str().to_string(),
                write_classes_touched: review.write_classes_touched.clone(),
                visible_reason: review.visible_reason.clone(),
            })
            .collect(),
        ModeStateOrientationSettingsSummary {
            multi_cursor_count: orientation.multi_cursor.caret_count,
            fold_summary_truth_preserved: orientation.fold_summaries_preserve_hidden_state(),
            breadcrumb_state: orientation.breadcrumbs.symbol_path_state.clone(),
            overview_availability: orientation.overview_aid.availability.as_str().to_string(),
            alternate_route_refs: orientation
                .overview_aid
                .replacement_route_refs
                .iter()
                .chain(orientation.breadcrumbs.alternate_command_refs.iter())
                .cloned()
                .collect(),
        },
        mode.recovery_actions
            .iter()
            .map(|action| action.action_ref.clone())
            .collect(),
        mode.support_export_refs
            .iter()
            .chain(orientation.support_export_refs.iter())
            .cloned()
            .collect(),
    )
}

fn mode_for_preset(preset: KeymapPresetId) -> EditorModeClass {
    match preset {
        KeymapPresetId::Vim => EditorModeClass::Normal,
        KeymapPresetId::Emacs => EditorModeClass::Command,
        KeymapPresetId::VsCode | KeymapPresetId::IntelliJ => EditorModeClass::Modeless,
    }
}

fn platform_token(platform: PlatformClass) -> &'static str {
    match platform {
        PlatformClass::Macos => "macos",
        PlatformClass::Windows => "windows",
        PlatformClass::Linux => "linux",
        PlatformClass::Web => "web",
        PlatformClass::CrossPlatform => "cross_platform",
    }
}

fn safe_ref(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect()
}
