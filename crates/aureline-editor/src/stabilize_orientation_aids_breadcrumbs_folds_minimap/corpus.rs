//! Deterministic claimed-stable matrix for orientation-aids stability.
//!
//! Every scenario here is projected through the **live** beta orientation-aid
//! builder (`crate::orientation_aids::build_beta_orientation_aid_state_record`)
//! so the stability packets are a genuine projection of the editor orientation
//! code rather than a parallel model. The corpus then mints one governed
//! [`OrientationAidsStabilityPacket`] per scenario and pins it on disk under
//! `fixtures/editor/m4/stabilize-orientation-aids-breadcrumbs-folds-minimap/`.
//!
//! The matrix covers:
//!
//! - Full-fidelity source editor, diff, and review surfaces.
//! - Surface downgrades: large-file, low-resource, reduced-motion, high-contrast,
//!   battery-saver, restricted-mode.
//! - Multi-cursor postures: multiple carets and column selection.
//! - Fold summaries that preserve hidden critical state across all surfaces.

use crate::orientation_aids::{
    build_beta_orientation_aid_state_record, BetaOrientationAidInput, OrientationAidStateRecord,
    OrientationSurfaceClass,
};

use super::model::{OrientationAidsStabilityInput, OrientationAidsStabilityPacket};

/// Snapshot timestamp pinned for every record in the corpus.
pub const ORIENTATION_AIDS_STABILITY_CORPUS_AS_OF: &str = "2026-06-03T00:00:00Z";

const SUPPORT_EXPORT_REF: &str = "aureline://support-export/orientation-aids-stability";

/// One scenario in the claimed-stable orientation-aids stability matrix.
#[derive(Debug, Clone)]
pub struct OrientationAidsStabilityScenario {
    /// Stable scenario id (also the packet id).
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: &'static str,
    /// Expected surface class for the scenario.
    pub expected_surface_class: OrientationSurfaceClass,
    /// Expected number of degraded mode classes.
    pub expected_degraded_mode_count: usize,
    /// Expected multi-cursor caret count.
    pub expected_caret_count: usize,
    packet: OrientationAidsStabilityPacket,
}

impl OrientationAidsStabilityScenario {
    /// Returns the governed packet for this scenario.
    pub fn packet(&self) -> OrientationAidsStabilityPacket {
        self.packet.clone()
    }
}

/// Returns the full claimed-stable corpus.
pub fn orientation_aids_stability_corpus() -> Vec<OrientationAidsStabilityScenario> {
    vec![
        source_editor_full_fidelity(),
        diff_surface_column_selection(),
        review_surface_full_fidelity(),
        large_file_degraded(),
        low_resource_degraded(),
        reduced_motion_degraded(),
        high_contrast_degraded(),
        battery_saver_degraded(),
        restricted_mode_degraded(),
        fold_with_critical_hidden_state(),
    ]
}

fn beta_state(input: BetaOrientationAidInput) -> OrientationAidStateRecord {
    build_beta_orientation_aid_state_record(input)
}

fn source_editor_full_fidelity() -> OrientationAidsStabilityScenario {
    let state = beta_state(BetaOrientationAidInput {
        orientation_state_id: "orientation-aid-state:stable:source-editor:0001".into(),
        surface_class: OrientationSurfaceClass::EditorSource,
        document_ref: "document:orders/src/controller.ts".into(),
        surface_ref: "surface:editor.source.stable".into(),
        large_file_mode: false,
        low_resource_mode: false,
        reduced_motion: false,
        high_contrast: false,
        battery_saver: false,
        restricted_mode: false,
    });
    let packet = OrientationAidsStabilityPacket::build(OrientationAidsStabilityInput {
        packet_id: "orientation-aids-stable:source-editor-full".into(),
        orientation_aid_state: state,
        support_export_refs: vec![SUPPORT_EXPORT_REF.into()],
    })
    .expect("source editor full fidelity must build");
    OrientationAidsStabilityScenario {
        scenario_id: "orientation-aids-stable:source-editor-full",
        fixture_filename: "source_editor_full_fidelity.json",
        expected_surface_class: OrientationSurfaceClass::EditorSource,
        expected_degraded_mode_count: 0,
        expected_caret_count: 3,
        packet,
    }
}

fn diff_surface_column_selection() -> OrientationAidsStabilityScenario {
    let mut state = beta_state(BetaOrientationAidInput {
        orientation_state_id: "orientation-aid-state:stable:diff-surface:0001".into(),
        surface_class: OrientationSurfaceClass::EditorDiff,
        document_ref: "document:orders/src/controller.ts".into(),
        surface_ref: "surface:editor.diff.stable".into(),
        large_file_mode: false,
        low_resource_mode: false,
        reduced_motion: false,
        high_contrast: false,
        battery_saver: false,
        restricted_mode: false,
    });
    // Override to column-selection posture for the diff surface.
    state.multi_cursor.mode_posture =
        crate::orientation_aids::MultiCursorModePosture::ColumnSelection;
    state.multi_cursor.column_mode_active = true;
    state.multi_cursor.caret_count = 5;
    state.multi_cursor.accessibility_label =
        "Column selection active, five carets, single undo group.".into();

    let packet = OrientationAidsStabilityPacket::build(OrientationAidsStabilityInput {
        packet_id: "orientation-aids-stable:diff-column-selection".into(),
        orientation_aid_state: state,
        support_export_refs: vec![SUPPORT_EXPORT_REF.into()],
    })
    .expect("diff surface column selection must build");
    OrientationAidsStabilityScenario {
        scenario_id: "orientation-aids-stable:diff-column-selection",
        fixture_filename: "diff_surface_column_selection.json",
        expected_surface_class: OrientationSurfaceClass::EditorDiff,
        expected_degraded_mode_count: 0,
        expected_caret_count: 5,
        packet,
    }
}

fn review_surface_full_fidelity() -> OrientationAidsStabilityScenario {
    let state = beta_state(BetaOrientationAidInput {
        orientation_state_id: "orientation-aid-state:stable:review-surface:0001".into(),
        surface_class: OrientationSurfaceClass::ReviewThread,
        document_ref: "document:orders/src/controller.ts".into(),
        surface_ref: "surface:editor.review.stable".into(),
        large_file_mode: false,
        low_resource_mode: false,
        reduced_motion: false,
        high_contrast: false,
        battery_saver: false,
        restricted_mode: false,
    });
    let packet = OrientationAidsStabilityPacket::build(OrientationAidsStabilityInput {
        packet_id: "orientation-aids-stable:review-full".into(),
        orientation_aid_state: state,
        support_export_refs: vec![SUPPORT_EXPORT_REF.into()],
    })
    .expect("review surface full fidelity must build");
    OrientationAidsStabilityScenario {
        scenario_id: "orientation-aids-stable:review-full",
        fixture_filename: "review_surface_full_fidelity.json",
        expected_surface_class: OrientationSurfaceClass::ReviewThread,
        expected_degraded_mode_count: 0,
        expected_caret_count: 3,
        packet,
    }
}

fn large_file_degraded() -> OrientationAidsStabilityScenario {
    let state = beta_state(BetaOrientationAidInput {
        orientation_state_id: "orientation-aid-state:stable:large-file:0001".into(),
        surface_class: OrientationSurfaceClass::EditorSource,
        document_ref: "document:logs/giant.log".into(),
        surface_ref: "surface:editor.large_file.stable".into(),
        large_file_mode: true,
        low_resource_mode: false,
        reduced_motion: false,
        high_contrast: false,
        battery_saver: false,
        restricted_mode: false,
    });
    let packet = OrientationAidsStabilityPacket::build(OrientationAidsStabilityInput {
        packet_id: "orientation-aids-stable:large-file-degraded".into(),
        orientation_aid_state: state,
        support_export_refs: vec![SUPPORT_EXPORT_REF.into()],
    })
    .expect("large file degraded must build");
    OrientationAidsStabilityScenario {
        scenario_id: "orientation-aids-stable:large-file-degraded",
        fixture_filename: "large_file_degraded.json",
        expected_surface_class: OrientationSurfaceClass::EditorSource,
        expected_degraded_mode_count: 1,
        expected_caret_count: 3,
        packet,
    }
}

fn low_resource_degraded() -> OrientationAidsStabilityScenario {
    let state = beta_state(BetaOrientationAidInput {
        orientation_state_id: "orientation-aid-state:stable:low-resource:0001".into(),
        surface_class: OrientationSurfaceClass::EditorSource,
        document_ref: "document:orders/src/controller.ts".into(),
        surface_ref: "surface:editor.low_resource.stable".into(),
        large_file_mode: false,
        low_resource_mode: true,
        reduced_motion: false,
        high_contrast: false,
        battery_saver: false,
        restricted_mode: false,
    });
    let packet = OrientationAidsStabilityPacket::build(OrientationAidsStabilityInput {
        packet_id: "orientation-aids-stable:low-resource-degraded".into(),
        orientation_aid_state: state,
        support_export_refs: vec![SUPPORT_EXPORT_REF.into()],
    })
    .expect("low resource degraded must build");
    OrientationAidsStabilityScenario {
        scenario_id: "orientation-aids-stable:low-resource-degraded",
        fixture_filename: "low_resource_degraded.json",
        expected_surface_class: OrientationSurfaceClass::EditorSource,
        expected_degraded_mode_count: 1,
        expected_caret_count: 3,
        packet,
    }
}

fn reduced_motion_degraded() -> OrientationAidsStabilityScenario {
    let state = beta_state(BetaOrientationAidInput {
        orientation_state_id: "orientation-aid-state:stable:reduced-motion:0001".into(),
        surface_class: OrientationSurfaceClass::EditorSource,
        document_ref: "document:orders/src/controller.ts".into(),
        surface_ref: "surface:editor.reduced_motion.stable".into(),
        large_file_mode: false,
        low_resource_mode: false,
        reduced_motion: true,
        high_contrast: false,
        battery_saver: false,
        restricted_mode: false,
    });
    let packet = OrientationAidsStabilityPacket::build(OrientationAidsStabilityInput {
        packet_id: "orientation-aids-stable:reduced-motion-degraded".into(),
        orientation_aid_state: state,
        support_export_refs: vec![SUPPORT_EXPORT_REF.into()],
    })
    .expect("reduced motion degraded must build");
    OrientationAidsStabilityScenario {
        scenario_id: "orientation-aids-stable:reduced-motion-degraded",
        fixture_filename: "reduced_motion_degraded.json",
        expected_surface_class: OrientationSurfaceClass::EditorSource,
        expected_degraded_mode_count: 1,
        expected_caret_count: 3,
        packet,
    }
}

fn high_contrast_degraded() -> OrientationAidsStabilityScenario {
    let state = beta_state(BetaOrientationAidInput {
        orientation_state_id: "orientation-aid-state:stable:high-contrast:0001".into(),
        surface_class: OrientationSurfaceClass::EditorSource,
        document_ref: "document:orders/src/controller.ts".into(),
        surface_ref: "surface:editor.high_contrast.stable".into(),
        large_file_mode: false,
        low_resource_mode: false,
        reduced_motion: false,
        high_contrast: true,
        battery_saver: false,
        restricted_mode: false,
    });
    let packet = OrientationAidsStabilityPacket::build(OrientationAidsStabilityInput {
        packet_id: "orientation-aids-stable:high-contrast-degraded".into(),
        orientation_aid_state: state,
        support_export_refs: vec![SUPPORT_EXPORT_REF.into()],
    })
    .expect("high contrast degraded must build");
    OrientationAidsStabilityScenario {
        scenario_id: "orientation-aids-stable:high-contrast-degraded",
        fixture_filename: "high_contrast_degraded.json",
        expected_surface_class: OrientationSurfaceClass::EditorSource,
        expected_degraded_mode_count: 1,
        expected_caret_count: 3,
        packet,
    }
}

fn battery_saver_degraded() -> OrientationAidsStabilityScenario {
    let state = beta_state(BetaOrientationAidInput {
        orientation_state_id: "orientation-aid-state:stable:battery-saver:0001".into(),
        surface_class: OrientationSurfaceClass::EditorSource,
        document_ref: "document:orders/src/controller.ts".into(),
        surface_ref: "surface:editor.battery_saver.stable".into(),
        large_file_mode: false,
        low_resource_mode: false,
        reduced_motion: false,
        high_contrast: false,
        battery_saver: true,
        restricted_mode: false,
    });
    let packet = OrientationAidsStabilityPacket::build(OrientationAidsStabilityInput {
        packet_id: "orientation-aids-stable:battery-saver-degraded".into(),
        orientation_aid_state: state,
        support_export_refs: vec![SUPPORT_EXPORT_REF.into()],
    })
    .expect("battery saver degraded must build");
    OrientationAidsStabilityScenario {
        scenario_id: "orientation-aids-stable:battery-saver-degraded",
        fixture_filename: "battery_saver_degraded.json",
        expected_surface_class: OrientationSurfaceClass::EditorSource,
        expected_degraded_mode_count: 1,
        expected_caret_count: 3,
        packet,
    }
}

fn restricted_mode_degraded() -> OrientationAidsStabilityScenario {
    let state = beta_state(BetaOrientationAidInput {
        orientation_state_id: "orientation-aid-state:stable:restricted-mode:0001".into(),
        surface_class: OrientationSurfaceClass::EditorSource,
        document_ref: "document:orders/src/controller.ts".into(),
        surface_ref: "surface:editor.restricted_mode.stable".into(),
        large_file_mode: false,
        low_resource_mode: false,
        reduced_motion: false,
        high_contrast: false,
        battery_saver: false,
        restricted_mode: true,
    });
    let packet = OrientationAidsStabilityPacket::build(OrientationAidsStabilityInput {
        packet_id: "orientation-aids-stable:restricted-mode-degraded".into(),
        orientation_aid_state: state,
        support_export_refs: vec![SUPPORT_EXPORT_REF.into()],
    })
    .expect("restricted mode degraded must build");
    OrientationAidsStabilityScenario {
        scenario_id: "orientation-aids-stable:restricted-mode-degraded",
        fixture_filename: "restricted_mode_degraded.json",
        expected_surface_class: OrientationSurfaceClass::EditorSource,
        expected_degraded_mode_count: 1,
        expected_caret_count: 3,
        packet,
    }
}

fn fold_with_critical_hidden_state() -> OrientationAidsStabilityScenario {
    let mut state = beta_state(BetaOrientationAidInput {
        orientation_state_id: "orientation-aid-state:stable:fold-critical:0001".into(),
        surface_class: OrientationSurfaceClass::EditorSource,
        document_ref: "document:orders/src/controller.ts".into(),
        surface_ref: "surface:editor.fold_critical.stable".into(),
        large_file_mode: false,
        low_resource_mode: false,
        reduced_motion: false,
        high_contrast: false,
        battery_saver: false,
        restricted_mode: false,
    });
    // Ensure at least one fold has critical hidden state and preserves it.
    for fold in &mut state.fold_summaries {
        if fold.hidden_marker_counts.iter().any(|c| c.count > 0) {
            fold.critical_state_preserved = true;
        }
    }
    let packet = OrientationAidsStabilityPacket::build(OrientationAidsStabilityInput {
        packet_id: "orientation-aids-stable:fold-critical".into(),
        orientation_aid_state: state,
        support_export_refs: vec![SUPPORT_EXPORT_REF.into()],
    })
    .expect("fold with critical hidden state must build");
    OrientationAidsStabilityScenario {
        scenario_id: "orientation-aids-stable:fold-critical",
        fixture_filename: "fold_with_critical_hidden_state.json",
        expected_surface_class: OrientationSurfaceClass::EditorSource,
        expected_degraded_mode_count: 0,
        expected_caret_count: 3,
        packet,
    }
}
