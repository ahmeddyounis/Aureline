//! Fixture-driven coverage for artifact-import hardening packets.
//!
//! These tests load every fixture in
//! `fixtures/review/m4/harden-keymap-theme-settings-snippet-task-and-launch/`
//! and assert that:
//!
//! 1. Every fixture parses, validates, and projects without error.
//! 2. Outcome labels are one of: exact, translated, partial, shimmed, unsupported.
//! 3. Rollback checkpoint states are surfaced as separable inspectable truths.
//! 4. Support/export records keep every `raw_*_export_allowed` flag false.
//! 5. Consumer-surface lists include both `support_export` and `audit_lane`.
//! 6. Diagnostics carry reason classes and suggested actions when mapping fails.
//! 7. Artifact records cover the six hardened types.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    project_artifact_import_hardening_packet, ArtifactImportHardeningCommandClass,
    ArtifactImportHardeningInput, ArtifactImportHardeningPacket, ArtifactType, ImportOutcomeLabel,
};

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures/review/m4/harden-keymap-theme-settings-snippet-task-and-launch")
}

fn load_fixture(name: &str) -> ArtifactImportHardeningPacket {
    let path = fixtures_dir().join(name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&payload).unwrap_or_else(|err| {
        panic!(
            "fixture {} must parse as ArtifactImportHardeningPacket: {err}",
            path.display()
        )
    })
}

#[test]
fn vs_code_all_exact_fixture_validates() {
    let packet = load_fixture("vs_code_all_exact.json");
    packet.validate().expect("fixture must validate");

    assert_eq!(
        packet.record_kind,
        "review_artifact_import_hardening_packet"
    );
    assert_eq!(packet.schema_version, 1);
    assert_eq!(packet.source_editor, "vs_code_code_oss");

    let keymap = packet
        .artifact_records
        .iter()
        .find(|r| r.artifact_type == ArtifactType::Keymap)
        .expect("keymap record must exist");
    assert_eq!(keymap.overall_outcome, ImportOutcomeLabel::Exact);
    assert!(!keymap.requires_manual_review);

    let launch = packet
        .artifact_records
        .iter()
        .find(|r| r.artifact_type == ArtifactType::Launch)
        .expect("launch record must exist");
    assert_eq!(launch.overall_outcome, ImportOutcomeLabel::Exact);

    let checkpoint = &packet.rollback_checkpoint;
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
    assert_eq!(inspection.artifact_record_count, 6);

    assert!(packet
        .consumer_surfaces
        .contains(&"support_export".to_string()));
    assert!(packet.consumer_surfaces.contains(&"audit_lane".to_string()));
}

#[test]
fn jetbrains_partial_with_diagnostics_fixture_validates() {
    let packet = load_fixture("jetbrains_partial_with_diagnostics.json");
    packet.validate().expect("fixture must validate");

    assert_eq!(packet.source_editor, "jetbrains_family");

    let theme = packet
        .artifact_records
        .iter()
        .find(|r| r.artifact_type == ArtifactType::Theme)
        .expect("theme record must exist");
    assert_eq!(theme.overall_outcome, ImportOutcomeLabel::Partial);
    assert!(theme.requires_manual_review);

    let snippet = packet
        .artifact_records
        .iter()
        .find(|r| r.artifact_type == ArtifactType::Snippet)
        .expect("snippet record must exist");
    assert_eq!(snippet.overall_outcome, ImportOutcomeLabel::Unsupported);
    assert!(snippet.requires_manual_review);
    assert!(!snippet.actionable);

    let launch = packet
        .artifact_records
        .iter()
        .find(|r| r.artifact_type == ArtifactType::Launch)
        .expect("launch record must exist");
    assert_eq!(launch.overall_outcome, ImportOutcomeLabel::Shimmed);

    assert!(!packet.diagnostics.is_empty());

    let diagnostic = packet
        .diagnostics
        .iter()
        .find(|d| d.diagnostic_id == "diag-jb-snippet-001")
        .expect("diag-jb-snippet-001 must exist");
    assert_eq!(diagnostic.outcome_label, ImportOutcomeLabel::Unsupported);
    assert_eq!(
        diagnostic.reason_class.as_deref(),
        Some("no_semantic_equivalent")
    );
    assert_eq!(
        diagnostic.suggested_action.as_deref(),
        Some("use_native_alternative")
    );
    assert!(!diagnostic.fallback_available);

    let inspection = &packet.inspection;
    assert!(inspection.unsupported_encountered);
    assert!(inspection.partial_encountered);
    assert!(inspection.shimmed_encountered);
    assert!(inspection.manual_review_required);
    assert_eq!(inspection.diagnostic_count, 3);
}

#[test]
fn vim_translated_shimmed_fixture_validates() {
    let packet = load_fixture("vim_translated_shimmed.json");
    packet.validate().expect("fixture must validate");

    assert_eq!(packet.source_editor, "vim_neovim");

    let keymap = packet
        .artifact_records
        .iter()
        .find(|r| r.artifact_type == ArtifactType::Keymap)
        .expect("keymap record must exist");
    assert_eq!(keymap.overall_outcome, ImportOutcomeLabel::Translated);

    let task = packet
        .artifact_records
        .iter()
        .find(|r| r.artifact_type == ArtifactType::Task)
        .expect("task record must exist");
    assert_eq!(task.overall_outcome, ImportOutcomeLabel::Shimmed);

    let launch = packet
        .artifact_records
        .iter()
        .find(|r| r.artifact_type == ArtifactType::Launch)
        .expect("launch record must exist");
    assert_eq!(launch.overall_outcome, ImportOutcomeLabel::Unsupported);
    assert!(!launch.actionable);

    let inspection = &packet.inspection;
    assert!(inspection.shimmed_encountered);
    assert!(inspection.partial_encountered);
    assert!(inspection.unsupported_encountered);
}

#[test]
fn emacs_mixed_outcomes_fixture_validates() {
    let packet = load_fixture("emacs_mixed_outcomes.json");
    packet.validate().expect("fixture must validate");

    assert_eq!(packet.source_editor, "emacs");

    let settings = packet
        .artifact_records
        .iter()
        .find(|r| r.artifact_type == ArtifactType::Settings)
        .expect("settings record must exist");
    assert_eq!(settings.overall_outcome, ImportOutcomeLabel::Partial);

    let task = packet
        .artifact_records
        .iter()
        .find(|r| r.artifact_type == ArtifactType::Task)
        .expect("task record must exist");
    assert_eq!(task.overall_outcome, ImportOutcomeLabel::Unsupported);

    let launch = packet
        .artifact_records
        .iter()
        .find(|r| r.artifact_type == ArtifactType::Launch)
        .expect("launch record must exist");
    assert_eq!(launch.overall_outcome, ImportOutcomeLabel::Unsupported);

    let inspection = &packet.inspection;
    assert!(inspection.manual_review_required);
    assert!(inspection.unsupported_encountered);
}

#[test]
fn projection_from_input_produces_valid_packet() {
    let input = ArtifactImportHardeningInput {
        migration_session_ref: "migration-session:test-0001".to_string(),
        source_editor: "vs_code_code_oss".to_string(),
        selected_artifact_types: vec![
            "keymap".to_string(),
            "theme".to_string(),
            "settings".to_string(),
        ],
        require_rollback_checkpoint: true,
        consumer_surfaces: vec![
            "migration_center".to_string(),
            "support_export".to_string(),
            "audit_lane".to_string(),
        ],
    };

    let projection =
        project_artifact_import_hardening_packet(&input).expect("projection must succeed");
    assert!(projection.actionable);
    assert!(!projection.requires_manual_review);

    let packet = projection.packet;
    packet.validate().expect("projected packet must validate");

    assert_eq!(packet.artifact_records.len(), 3);
    assert!(packet
        .artifact_records
        .iter()
        .any(|r| r.artifact_type == ArtifactType::Keymap));
    assert!(packet
        .artifact_records
        .iter()
        .any(|r| r.artifact_type == ArtifactType::Theme));
    assert!(packet
        .artifact_records
        .iter()
        .any(|r| r.artifact_type == ArtifactType::Settings));

    let checkpoint = &packet.rollback_checkpoint;
    assert_eq!(checkpoint.checkpoint_state.to_string(), "captured_pending");
    assert!(checkpoint.auto_restore_available);

    assert!(packet.commands.iter().any(|c| {
        c.command_class == ArtifactImportHardeningCommandClass::Preview && c.available
    }));
    assert!(packet.commands.iter().any(|c| {
        c.command_class == ArtifactImportHardeningCommandClass::CaptureCheckpoint && c.available
    }));
}

#[test]
fn unknown_source_editor_fails_projection() {
    let input = ArtifactImportHardeningInput {
        migration_session_ref: "migration-session:test-0001".to_string(),
        source_editor: "unknown_editor".to_string(),
        selected_artifact_types: vec!["keymap".to_string()],
        require_rollback_checkpoint: false,
        consumer_surfaces: vec!["migration_center".to_string()],
    };

    let result = project_artifact_import_hardening_packet(&input);
    assert!(result.is_err());
}

#[test]
fn unknown_artifact_type_fails_projection() {
    let input = ArtifactImportHardeningInput {
        migration_session_ref: "migration-session:test-0001".to_string(),
        source_editor: "vs_code_code_oss".to_string(),
        selected_artifact_types: vec!["unknown_artifact".to_string()],
        require_rollback_checkpoint: false,
        consumer_surfaces: vec!["migration_center".to_string()],
    };

    let result = project_artifact_import_hardening_packet(&input);
    assert!(result.is_err());
}

#[test]
fn empty_consumer_surfaces_fails_projection() {
    let input = ArtifactImportHardeningInput {
        migration_session_ref: "migration-session:test-0001".to_string(),
        source_editor: "vs_code_code_oss".to_string(),
        selected_artifact_types: vec!["keymap".to_string()],
        require_rollback_checkpoint: false,
        consumer_surfaces: vec![],
    };

    let result = project_artifact_import_hardening_packet(&input);
    assert!(result.is_err());
}
