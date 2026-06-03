//! Deterministic claimed-stable matrix for modal-editing safety.
//!
//! Every scenario here is projected through the **live** mode-state builder
//! (`crate::modes::build_alpha_mode_state_record`) so the safety packets are a
//! genuine projection of the editor mode-state code rather than a parallel
//! model. The corpus then mints one governed [`ModalEditingSafetyPacket`] per
//! scenario and pins it on disk under
//! `fixtures/editor/m4/stabilize-modal-editing-leader-register-safety/`.
//!
//! The matrix covers:
//!
//! - Full-fidelity modal surfaces (Vim normal mode baseline).
//! - Surface downgrades: IME, accessibility, browser companion, restricted mode,
//!   large file.
//! - Import regression outcomes: exact, translated, partial, shimmed, unsupported.
//! - Macro replay safety: cross-file review, run-capable/settings-mutating rejection.

use crate::modes::{
    build_alpha_mode_state_record, AlphaModeStateInput, EditorModeClass, EditorModeStateRecord,
};

use super::model::{
    KeymapImportOutcomeClass, KeymapImportRegressionRecord, ModalEditingSafetyInput,
    ModalEditingSafetyPacket, SurfaceDowngradeKind, SurfaceDowngradeRecord,
};

/// Snapshot timestamp pinned for every record in the corpus.
pub const MODAL_EDITING_SAFETY_CORPUS_AS_OF: &str = "2026-06-03T00:00:00Z";

const SUPPORT_EXPORT_REF: &str = "aureline://support-export/modal-editing-safety";
const DIAGNOSTICS_ROUTE: &str = "surface:help.keybinding_inspector";

/// One scenario in the claimed-stable modal-editing safety matrix.
#[derive(Debug, Clone)]
pub struct ModalEditingSafetyScenario {
    /// Stable scenario id (also the packet id).
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: &'static str,
    /// Expected mode class for the scenario.
    pub expected_mode: EditorModeClass,
    /// Expected number of surface downgrades.
    pub expected_downgrade_count: usize,
    /// Expected number of import regressions.
    pub expected_regression_count: usize,
    packet: ModalEditingSafetyPacket,
}

impl ModalEditingSafetyScenario {
    /// Returns the governed packet for this scenario.
    pub fn packet(&self) -> ModalEditingSafetyPacket {
        self.packet.clone()
    }
}

/// Returns the full claimed-stable corpus.
pub fn modal_editing_safety_corpus() -> Vec<ModalEditingSafetyScenario> {
    vec![
        vim_normal_full_fidelity(),
        ime_downgrade(),
        accessibility_downgrade(),
        browser_companion_downgrade(),
        restricted_mode_downgrade(),
        large_file_downgrade(),
        import_exact(),
        import_translated(),
        import_partial(),
        import_shimmed(),
        import_unsupported(),
        macro_cross_file_review(),
        macro_run_settings_rejected(),
    ]
}

fn vim_mode_state(mode: EditorModeClass) -> EditorModeStateRecord {
    build_alpha_mode_state_record(AlphaModeStateInput {
        mode_state_id: format!("mode-state:corpus:{}", mode.as_str()),
        source_preset_ref: "preset:keymap:vim".to_string(),
        source_preset_label: "Vim".to_string(),
        current_mode: mode,
        surface_ref: "surface:editor.source.alpha".to_string(),
        platform_class: "macos".to_string(),
    })
}

fn vim_normal_full_fidelity() -> ModalEditingSafetyScenario {
    let packet = ModalEditingSafetyPacket::build(ModalEditingSafetyInput {
        packet_id: "modal-safety:vim-normal-full".to_string(),
        mode_state: vim_mode_state(EditorModeClass::Normal),
        surface_downgrades: Vec::new(),
        import_regressions: Vec::new(),
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("vim normal full fidelity must build");
    ModalEditingSafetyScenario {
        scenario_id: "modal-safety:vim-normal-full",
        fixture_filename: "vim_normal_full_fidelity.json",
        expected_mode: EditorModeClass::Normal,
        expected_downgrade_count: 0,
        expected_regression_count: 0,
        packet,
    }
}

fn ime_downgrade() -> ModalEditingSafetyScenario {
    let packet = ModalEditingSafetyPacket::build(ModalEditingSafetyInput {
        packet_id: "modal-safety:ime-downgrade".to_string(),
        mode_state: vim_mode_state(EditorModeClass::Insert),
        surface_downgrades: vec![SurfaceDowngradeRecord {
            downgrade_ref: "downgrade:ime:composition".to_string(),
            downgrade_kind: SurfaceDowngradeKind::Ime,
            surface_ref: "surface:editor.source.alpha".to_string(),
            visible_label: "IME composition active".to_string(),
            visible_reason:
                "IME preedit is swallowing keystrokes; modal commands are deferred until composition ends."
                    .to_string(),
            reversible: true,
            keyboard_route_to_restore: "Escape to exit IME preedit, then resume modal input."
                .to_string(),
            accessibility_announcement:
                "IME composition is active; modal editing commands are temporarily deferred."
                    .to_string(),
        }],
        import_regressions: Vec::new(),
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("ime downgrade must build");
    ModalEditingSafetyScenario {
        scenario_id: "modal-safety:ime-downgrade",
        fixture_filename: "ime_downgrade.json",
        expected_mode: EditorModeClass::Insert,
        expected_downgrade_count: 1,
        expected_regression_count: 0,
        packet,
    }
}

fn accessibility_downgrade() -> ModalEditingSafetyScenario {
    let packet = ModalEditingSafetyPacket::build(ModalEditingSafetyInput {
        packet_id: "modal-safety:accessibility-downgrade".to_string(),
        mode_state: vim_mode_state(EditorModeClass::Normal),
        surface_downgrades: vec![SurfaceDowngradeRecord {
            downgrade_ref: "downgrade:accessibility:screen_reader".to_string(),
            downgrade_kind: SurfaceDowngradeKind::Accessibility,
            surface_ref: "surface:editor.source.alpha".to_string(),
            visible_label: "Screen-reader mode".to_string(),
            visible_reason:
                "Screen-reader focus mode simplifies leader overlays to single-stroke announcements."
                    .to_string(),
            reversible: true,
            keyboard_route_to_restore:
                "Toggle screen-reader focus mode in accessibility settings to restore full overlays."
                    .to_string(),
            accessibility_announcement:
                "Screen-reader mode is active; leader overlays are simplified to single-stroke announcements."
                    .to_string(),
        }],
        import_regressions: Vec::new(),
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("accessibility downgrade must build");
    ModalEditingSafetyScenario {
        scenario_id: "modal-safety:accessibility-downgrade",
        fixture_filename: "accessibility_downgrade.json",
        expected_mode: EditorModeClass::Normal,
        expected_downgrade_count: 1,
        expected_regression_count: 0,
        packet,
    }
}

fn browser_companion_downgrade() -> ModalEditingSafetyScenario {
    let packet = ModalEditingSafetyPacket::build(ModalEditingSafetyInput {
        packet_id: "modal-safety:browser-companion-downgrade".to_string(),
        mode_state: vim_mode_state(EditorModeClass::Normal),
        surface_downgrades: vec![SurfaceDowngradeRecord {
            downgrade_ref: "downgrade:browser:companion".to_string(),
            downgrade_kind: SurfaceDowngradeKind::BrowserCompanion,
            surface_ref: "surface:editor.browser.companion".to_string(),
            visible_label: "Browser companion limited".to_string(),
            visible_reason:
                "Browser host intercepts certain chords; some leader sequences are unavailable."
                    .to_string(),
            reversible: false,
            keyboard_route_to_restore:
                "Open in desktop Aureline to restore full leader-key fidelity."
                    .to_string(),
            accessibility_announcement:
                "Browser companion limits some leader sequences; open desktop Aureline for full fidelity."
                    .to_string(),
        }],
        import_regressions: Vec::new(),
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("browser companion downgrade must build");
    ModalEditingSafetyScenario {
        scenario_id: "modal-safety:browser-companion-downgrade",
        fixture_filename: "browser_companion_downgrade.json",
        expected_mode: EditorModeClass::Normal,
        expected_downgrade_count: 1,
        expected_regression_count: 0,
        packet,
    }
}

fn restricted_mode_downgrade() -> ModalEditingSafetyScenario {
    let packet = ModalEditingSafetyPacket::build(ModalEditingSafetyInput {
        packet_id: "modal-safety:restricted-mode-downgrade".to_string(),
        mode_state: vim_mode_state(EditorModeClass::Normal),
        surface_downgrades: vec![SurfaceDowngradeRecord {
            downgrade_ref: "downgrade:restricted:policy".to_string(),
            downgrade_kind: SurfaceDowngradeKind::RestrictedMode,
            surface_ref: "surface:editor.source.alpha".to_string(),
            visible_label: "Restricted mode".to_string(),
            visible_reason:
                "Restricted workspace policy disables run-capable commands and macro replay."
                    .to_string(),
            reversible: true,
            keyboard_route_to_restore:
                "Trust the workspace through the trust gate to restore full modal fidelity."
                    .to_string(),
            accessibility_announcement:
                "Restricted mode disables run-capable commands and macro replay."
                    .to_string(),
        }],
        import_regressions: Vec::new(),
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("restricted mode downgrade must build");
    ModalEditingSafetyScenario {
        scenario_id: "modal-safety:restricted-mode-downgrade",
        fixture_filename: "restricted_mode_downgrade.json",
        expected_mode: EditorModeClass::Normal,
        expected_downgrade_count: 1,
        expected_regression_count: 0,
        packet,
    }
}

fn large_file_downgrade() -> ModalEditingSafetyScenario {
    let packet = ModalEditingSafetyPacket::build(ModalEditingSafetyInput {
        packet_id: "modal-safety:large-file-downgrade".to_string(),
        mode_state: vim_mode_state(EditorModeClass::Normal),
        surface_downgrades: vec![SurfaceDowngradeRecord {
            downgrade_ref: "downgrade:large_file:limited".to_string(),
            downgrade_kind: SurfaceDowngradeKind::LargeFile,
            surface_ref: "surface:editor.large_file.viewer".to_string(),
            visible_label: "Large-file limited mode".to_string(),
            visible_reason:
                "Large-file posture disables multi-cursor modal operators and macro replay for performance."
                    .to_string(),
            reversible: true,
            keyboard_route_to_restore:
                "Switch to normal editing mode after the file is classified as safe."
                    .to_string(),
            accessibility_announcement:
                "Large-file mode disables multi-cursor operators and macro replay for performance."
                    .to_string(),
        }],
        import_regressions: Vec::new(),
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("large file downgrade must build");
    ModalEditingSafetyScenario {
        scenario_id: "modal-safety:large-file-downgrade",
        fixture_filename: "large_file_downgrade.json",
        expected_mode: EditorModeClass::Normal,
        expected_downgrade_count: 1,
        expected_regression_count: 0,
        packet,
    }
}

fn import_exact() -> ModalEditingSafetyScenario {
    let packet = ModalEditingSafetyPacket::build(ModalEditingSafetyInput {
        packet_id: "modal-safety:import-exact".to_string(),
        mode_state: vim_mode_state(EditorModeClass::Normal),
        surface_downgrades: Vec::new(),
        import_regressions: vec![KeymapImportRegressionRecord {
            regression_ref: "regression:vim:gg".to_string(),
            source_preset_ref: "preset:keymap:vim".to_string(),
            sequence_or_command_ref: "gg".to_string(),
            outcome_class: KeymapImportOutcomeClass::Exact,
            visible_reason: " gg maps exactly to editor.go_to_first_line.".to_string(),
            fallback_command_id: None,
            diagnostics_route_ref: DIAGNOSTICS_ROUTE.to_string(),
        }],
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("import exact must build");
    ModalEditingSafetyScenario {
        scenario_id: "modal-safety:import-exact",
        fixture_filename: "import_exact.json",
        expected_mode: EditorModeClass::Normal,
        expected_downgrade_count: 0,
        expected_regression_count: 1,
        packet,
    }
}

fn import_translated() -> ModalEditingSafetyScenario {
    let packet = ModalEditingSafetyPacket::build(ModalEditingSafetyInput {
        packet_id: "modal-safety:import-translated".to_string(),
        mode_state: vim_mode_state(EditorModeClass::Normal),
        surface_downgrades: Vec::new(),
        import_regressions: vec![KeymapImportRegressionRecord {
            regression_ref: "regression:vim:ctrl_w_h".to_string(),
            source_preset_ref: "preset:keymap:vim".to_string(),
            sequence_or_command_ref: "Ctrl+W h".to_string(),
            outcome_class: KeymapImportOutcomeClass::Translated,
            visible_reason:
                "Ctrl+W h is translated to editor.navigate_to_left_group (native pane command)."
                    .to_string(),
            fallback_command_id: Some("cmd:editor.navigate_to_left_group".to_string()),
            diagnostics_route_ref: DIAGNOSTICS_ROUTE.to_string(),
        }],
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("import translated must build");
    ModalEditingSafetyScenario {
        scenario_id: "modal-safety:import-translated",
        fixture_filename: "import_translated.json",
        expected_mode: EditorModeClass::Normal,
        expected_downgrade_count: 0,
        expected_regression_count: 1,
        packet,
    }
}

fn import_partial() -> ModalEditingSafetyScenario {
    let packet = ModalEditingSafetyPacket::build(ModalEditingSafetyInput {
        packet_id: "modal-safety:import-partial".to_string(),
        mode_state: vim_mode_state(EditorModeClass::Normal),
        surface_downgrades: Vec::new(),
        import_regressions: vec![KeymapImportRegressionRecord {
            regression_ref: "regression:vim:gd".to_string(),
            source_preset_ref: "preset:keymap:vim".to_string(),
            sequence_or_command_ref: "gd".to_string(),
            outcome_class: KeymapImportOutcomeClass::Partial,
            visible_reason:
                "gd maps to editor.go_to_definition but lacks the preview split behavior present in Vim."
                    .to_string(),
            fallback_command_id: Some("cmd:editor.go_to_definition".to_string()),
            diagnostics_route_ref: DIAGNOSTICS_ROUTE.to_string(),
        }],
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("import partial must build");
    ModalEditingSafetyScenario {
        scenario_id: "modal-safety:import-partial",
        fixture_filename: "import_partial.json",
        expected_mode: EditorModeClass::Normal,
        expected_downgrade_count: 0,
        expected_regression_count: 1,
        packet,
    }
}

fn import_shimmed() -> ModalEditingSafetyScenario {
    let packet = ModalEditingSafetyPacket::build(ModalEditingSafetyInput {
        packet_id: "modal-safety:import-shimmed".to_string(),
        mode_state: vim_mode_state(EditorModeClass::Normal),
        surface_downgrades: Vec::new(),
        import_regressions: vec![KeymapImportRegressionRecord {
            regression_ref: "regression:vim:ctrl_a".to_string(),
            source_preset_ref: "preset:keymap:vim".to_string(),
            sequence_or_command_ref: "Ctrl+A".to_string(),
            outcome_class: KeymapImportOutcomeClass::Shimmed,
            visible_reason:
                "Ctrl+A is shimmed to editor.increment_number; it works for decimal integers but not hex or octal."
                    .to_string(),
            fallback_command_id: Some("cmd:editor.increment_number".to_string()),
            diagnostics_route_ref: DIAGNOSTICS_ROUTE.to_string(),
        }],
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("import shimmed must build");
    ModalEditingSafetyScenario {
        scenario_id: "modal-safety:import-shimmed",
        fixture_filename: "import_shimmed.json",
        expected_mode: EditorModeClass::Normal,
        expected_downgrade_count: 0,
        expected_regression_count: 1,
        packet,
    }
}

fn import_unsupported() -> ModalEditingSafetyScenario {
    let packet = ModalEditingSafetyPacket::build(ModalEditingSafetyInput {
        packet_id: "modal-safety:import-unsupported".to_string(),
        mode_state: vim_mode_state(EditorModeClass::Normal),
        surface_downgrades: Vec::new(),
        import_regressions: vec![KeymapImportRegressionRecord {
            regression_ref: "regression:vim:q".to_string(),
            source_preset_ref: "preset:keymap:vim".to_string(),
            sequence_or_command_ref: ":q".to_string(),
            outcome_class: KeymapImportOutcomeClass::Unsupported,
            visible_reason:
                ":q quit semantics are unsupported; use workbench.close_active_editor instead."
                    .to_string(),
            fallback_command_id: Some("cmd:workbench.close_active_editor".to_string()),
            diagnostics_route_ref: DIAGNOSTICS_ROUTE.to_string(),
        }],
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("import unsupported must build");
    ModalEditingSafetyScenario {
        scenario_id: "modal-safety:import-unsupported",
        fixture_filename: "import_unsupported.json",
        expected_mode: EditorModeClass::Normal,
        expected_downgrade_count: 0,
        expected_regression_count: 1,
        packet,
    }
}

fn macro_cross_file_review() -> ModalEditingSafetyScenario {
    let packet = ModalEditingSafetyPacket::build(ModalEditingSafetyInput {
        packet_id: "modal-safety:macro-cross-file".to_string(),
        mode_state: vim_mode_state(EditorModeClass::Normal),
        surface_downgrades: Vec::new(),
        import_regressions: Vec::new(),
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("macro cross file review must build");
    ModalEditingSafetyScenario {
        scenario_id: "modal-safety:macro-cross-file",
        fixture_filename: "macro_cross_file_review.json",
        expected_mode: EditorModeClass::Normal,
        expected_downgrade_count: 0,
        expected_regression_count: 0,
        packet,
    }
}

fn macro_run_settings_rejected() -> ModalEditingSafetyScenario {
    let packet = ModalEditingSafetyPacket::build(ModalEditingSafetyInput {
        packet_id: "modal-safety:macro-run-settings".to_string(),
        mode_state: vim_mode_state(EditorModeClass::Normal),
        surface_downgrades: Vec::new(),
        import_regressions: Vec::new(),
        support_export_refs: vec![SUPPORT_EXPORT_REF.to_string()],
    })
    .expect("macro run settings rejected must build");
    ModalEditingSafetyScenario {
        scenario_id: "modal-safety:macro-run-settings",
        fixture_filename: "macro_run_settings_rejected.json",
        expected_mode: EditorModeClass::Normal,
        expected_downgrade_count: 0,
        expected_regression_count: 0,
        packet,
    }
}
