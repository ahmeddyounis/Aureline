//! Fixture-driven coverage for migration-wizard import-fidelity packets.
//!
//! These tests load every fixture in
//! `fixtures/review/m4/stabilize-migration-wizard-import-fidelity-for-vs-code/`
//! and assert that:
//!
//! 1. Every fixture parses, validates, and projects without error.
//! 2. Outcome labels are one of: exact, translated, partial, shimmed, unsupported.
//! 3. Rollback checkpoint states are surfaced as separable inspectable truths.
//! 4. Support/export records keep every `raw_*_export_allowed` flag false.
//! 5. Consumer-surface lists include both `support_export` and `audit_lane`.
//! 6. Diagnostics carry reason classes and suggested actions when mapping fails.
//! 7. Launch path states are previewable or checkpointed before destructive apply.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    project_migration_wizard_import_fidelity_packet, ImportOutcomeLabel,
    MigrationWizardImportFidelityInput, MigrationWizardImportFidelityPacket,
};
fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/review/m4/stabilize-migration-wizard-import-fidelity-for-vs-code")
}

fn load_fixture(name: &str) -> MigrationWizardImportFidelityPacket {
    let path = fixtures_dir().join(name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&payload).unwrap_or_else(|err| {
        panic!(
            "fixture {} must parse as MigrationWizardImportFidelityPacket: {err}",
            path.display()
        )
    })
}

#[test]
fn vs_code_settings_keybindings_exact_fixture_validates() {
    let packet = load_fixture("vs_code_settings_keybindings_exact.json");
    packet.validate().expect("fixture must validate");

    assert_eq!(
        packet.record_kind,
        "workspace_migration_wizard_import_fidelity_packet"
    );
    assert_eq!(packet.schema_version, 1);
    assert_eq!(packet.fidelity_record.source_editor, "vs_code_code_oss");
    assert_eq!(
        packet.fidelity_record.overall_outcome,
        ImportOutcomeLabel::Exact
    );

    let editor = &packet.editor_launch_paths[0];
    assert_eq!(editor.source_editor, "vs_code_code_oss");
    assert!(!editor.requires_manual_review);
    assert!(!editor.requires_browser_handoff);
    assert!(editor.actionable);

    let checkpoint = &editor.rollback_checkpoint;
    assert_eq!(checkpoint.checkpoint_state.to_string(), "captured_ready");
    assert!(checkpoint.auto_restore_available);

    let support = &packet.support_export;
    assert!(!support.raw_source_profile_paths_export_allowed);
    assert!(!support.raw_source_profile_bodies_export_allowed);
    assert!(!support.secret_bearing_values_export_allowed);

    let inspection = &packet.inspection;
    assert!(inspection.previewable);
    assert!(inspection.checkpoint_ready);
    assert!(!inspection.applied);
    assert!(!inspection.unsupported_encountered);
    assert!(!inspection.partial_encountered);
    assert!(!inspection.shimmed_encountered);

    assert!(packet
        .consumer_surfaces
        .contains(&"support_export".to_string()));
    assert!(packet.consumer_surfaces.contains(&"audit_lane".to_string()));
}

#[test]
fn jetbrains_partial_with_diagnostics_fixture_validates() {
    let packet = load_fixture("jetbrains_partial_with_diagnostics.json");
    packet.validate().expect("fixture must validate");

    assert_eq!(packet.fidelity_record.source_editor, "jetbrains_family");
    assert_eq!(
        packet.fidelity_record.overall_outcome,
        ImportOutcomeLabel::Partial
    );

    let editor = &packet.editor_launch_paths[0];
    assert!(editor.requires_manual_review);
    assert!(!editor.diagnostics.is_empty());

    let diagnostic = editor
        .diagnostics
        .iter()
        .find(|d| d.diagnostic_id == "diag-jb-001")
        .expect("diag-jb-001 must exist");
    assert_eq!(diagnostic.outcome_label, ImportOutcomeLabel::Unsupported);
    assert_eq!(
        diagnostic.reason_class.as_deref(),
        Some("no_semantic_equivalent")
    );
    assert_eq!(
        diagnostic.suggested_action.as_deref(),
        Some("use_native_alternative")
    );
    assert!(diagnostic.fallback_available);

    let inspection = &packet.inspection;
    assert!(inspection.unsupported_encountered);
    assert!(inspection.partial_encountered);
    assert_eq!(inspection.diagnostic_count, 2);
}

#[test]
fn vim_modal_editing_translated_fixture_validates() {
    let packet = load_fixture("vim_modal_editing_translated.json");
    packet.validate().expect("fixture must validate");

    assert_eq!(packet.fidelity_record.source_editor, "vim_neovim");
    assert_eq!(
        packet.fidelity_record.overall_outcome,
        ImportOutcomeLabel::Translated
    );

    let editor = &packet.editor_launch_paths[0];
    let clipboard = editor
        .outcome_breakdown
        .iter()
        .find(|o| o.target_family == "clipboard_search_defaults")
        .expect("clipboard_search_defaults must be in breakdown");
    assert_eq!(clipboard.shimmed_count, 2);
    assert_eq!(clipboard.unsupported_count, 1);

    let inspection = &packet.inspection;
    assert!(inspection.shimmed_encountered);
    assert!(inspection.unsupported_encountered);
}

#[test]
fn emacs_keyboard_navigation_mixed_fixture_validates() {
    let packet = load_fixture("emacs_keyboard_navigation_mixed.json");
    packet.validate().expect("fixture must validate");

    assert_eq!(packet.fidelity_record.source_editor, "emacs");
    assert_eq!(
        packet.fidelity_record.overall_outcome,
        ImportOutcomeLabel::Partial
    );

    let editor = &packet.editor_launch_paths[0];
    assert!(editor.requires_manual_review);

    let unsupported_diag = editor
        .diagnostics
        .iter()
        .find(|d| d.outcome_label == ImportOutcomeLabel::Unsupported)
        .expect("at least one unsupported diagnostic must exist");
    assert!(
        unsupported_diag.reason_class.is_some(),
        "unsupported diagnostic must have a reason class"
    );

    let inspection = &packet.inspection;
    assert!(inspection.unsupported_encountered);
    assert!(inspection.partial_encountered);
    assert_eq!(inspection.diagnostic_count, 3);
}

#[test]
fn all_fixtures_have_redaction_safe_support_export() {
    for name in [
        "vs_code_settings_keybindings_exact.json",
        "jetbrains_partial_with_diagnostics.json",
        "vim_modal_editing_translated.json",
        "emacs_keyboard_navigation_mixed.json",
    ] {
        let packet = load_fixture(name);
        let support = &packet.support_export;
        assert!(
            !support.raw_source_profile_paths_export_allowed,
            "{name}: raw paths must not be exportable"
        );
        assert!(
            !support.raw_source_profile_bodies_export_allowed,
            "{name}: raw bodies must not be exportable"
        );
        assert!(
            !support.secret_bearing_values_export_allowed,
            "{name}: secrets must not be exportable"
        );
    }
}

#[test]
fn projection_rejects_unknown_source_editor() {
    let input = MigrationWizardImportFidelityInput {
        migration_session_ref: "test-session".to_string(),
        source_editor: "unknown_editor".to_string(),
        selected_target_families: vec!["settings".to_string()],
        detected_source_profile_refs: vec![],
        require_rollback_checkpoint: true,
        consumer_surfaces: vec!["migration_center".to_string()],
    };
    let result = project_migration_wizard_import_fidelity_packet(&input);
    assert!(result.is_err(), "unknown source editor must be rejected");
}

#[test]
fn projection_rejects_unknown_import_target() {
    let input = MigrationWizardImportFidelityInput {
        migration_session_ref: "test-session".to_string(),
        source_editor: "vs_code_code_oss".to_string(),
        selected_target_families: vec!["unknown_target".to_string()],
        detected_source_profile_refs: vec![],
        require_rollback_checkpoint: true,
        consumer_surfaces: vec!["migration_center".to_string()],
    };
    let result = project_migration_wizard_import_fidelity_packet(&input);
    assert!(result.is_err(), "unknown import target must be rejected");
}

#[test]
fn projection_rejects_empty_consumer_surfaces() {
    let input = MigrationWizardImportFidelityInput {
        migration_session_ref: "test-session".to_string(),
        source_editor: "vs_code_code_oss".to_string(),
        selected_target_families: vec!["settings".to_string()],
        detected_source_profile_refs: vec![],
        require_rollback_checkpoint: true,
        consumer_surfaces: vec![],
    };
    let result = project_migration_wizard_import_fidelity_packet(&input);
    assert!(result.is_err(), "empty consumer surfaces must be rejected");
}

#[test]
fn packet_validate_rejects_missing_editor_launch_paths() {
    let mut packet = load_fixture("vs_code_settings_keybindings_exact.json");
    packet.editor_launch_paths.clear();
    let result = packet.validate();
    assert!(
        result.is_err(),
        "missing editor launch paths must fail validation"
    );
}

#[test]
fn packet_validate_rejects_missing_consumer_surfaces() {
    let mut packet = load_fixture("vs_code_settings_keybindings_exact.json");
    packet.consumer_surfaces.clear();
    let result = packet.validate();
    assert!(
        result.is_err(),
        "missing consumer surfaces must fail validation"
    );
}
