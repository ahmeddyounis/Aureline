use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_commands::alpha::alpha_command_registry;
use aureline_commands::registry::seeded_registry;
use aureline_input::keybindings::PlatformClass;
use aureline_input::presets::KeymapPresetId;
use aureline_shell::help::keybinding_inspector::build_inspector_lines;
use aureline_shell::keybindings::{
    alpha_keybinding_command_ids, materialize_alpha_keybinding_truth, KeybindingBridgeOutcomeClass,
};
use serde_json::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_json(path: &str) -> Value {
    let path = repo_root().join(path);
    let payload = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn alpha_keybinding_truth_covers_claimed_commands_and_presets() {
    let report = materialize_alpha_keybinding_truth(
        seeded_registry(),
        KeymapPresetId::VsCode,
        PlatformClass::Macos,
    );
    let claimed = alpha_keybinding_command_ids();

    assert_eq!(report.record_kind, "alpha_keybinding_truth_report");
    assert_eq!(report.summary.status, "pass");
    assert_eq!(
        report.summary.claimed_command_count,
        alpha_command_registry().claimed_commands.len()
    );
    assert_eq!(
        report.summary.preset_profile_count,
        KeymapPresetId::all().len()
    );

    let winning_ids = report
        .winning_bindings
        .iter()
        .map(|row| row.command_id.as_str())
        .collect::<BTreeSet<_>>();
    for command_id in &claimed {
        assert!(
            winning_ids.contains(command_id.as_str()),
            "missing winning row for {command_id}"
        );
    }

    for row in &report.winning_bindings {
        assert_ne!(row.literal_sequence, "unassigned", "{}", row.command_id);
        assert!(
            row.winning_source_ref
                .as_deref()
                .is_some_and(|source| source.starts_with("preset:keymap:")),
            "{} should carry preset source attribution",
            row.command_id
        );
        assert!(!row.preview_class.trim().is_empty());
        assert!(!row.authority_class.trim().is_empty());
    }

    for profile in &report.preset_profiles {
        assert_eq!(
            profile.expected_claimed_command_count,
            report.summary.claimed_command_count
        );
        assert_eq!(
            profile.bound_claimed_command_count, report.summary.claimed_command_count,
            "{} must bind the claimed command set",
            profile.preset_ref
        );
        assert_eq!(
            profile.translations.len(),
            report.summary.claimed_command_count
        );
    }

    assert!(report
        .preset_profiles
        .iter()
        .flat_map(|profile| profile.translations.iter())
        .any(|row| row.bridge_outcome_class == KeybindingBridgeOutcomeClass::Exact));
    assert!(report
        .preset_profiles
        .iter()
        .flat_map(|profile| profile.translations.iter())
        .any(|row| row.bridge_outcome_class != KeybindingBridgeOutcomeClass::Exact));
}

#[test]
fn conflicts_and_settings_rows_are_reopenable_without_raw_logs() {
    let report = materialize_alpha_keybinding_truth(
        seeded_registry(),
        KeymapPresetId::VsCode,
        PlatformClass::Macos,
    );

    assert!(
        report
            .conflict_inspections
            .iter()
            .any(|row| row.preset_ref == "preset:keymap:vim"
                && row.literal_sequence == "Ctrl+Shift+Y"
                && !row.losing_command_ids.is_empty()
                && row
                    .linked_migration_report_ref
                    .ends_with("keymap_translation_report_sample.json")),
        "expected Vim conflict row linked to retained migration report"
    );

    for row in &report.settings_inspection_rows {
        assert_eq!(row.record_kind, "keybinding_setting_inspection_record");
        assert!(row.resolver_packet_ref.is_some(), "{}", row.command_id);
        assert!(
            row.retained_report_ref
                .as_deref()
                .is_some_and(|value| value.ends_with("keymap_translation_report_sample.json")),
            "{} should reopen retained keymap translation evidence",
            row.command_id
        );
        assert!(
            row.source_chain.iter().any(|source| source.winner),
            "{} should expose a winning source row",
            row.command_id
        );
    }
}

#[test]
fn palette_menu_and_keybinding_parity_preserve_command_semantics() {
    let report = materialize_alpha_keybinding_truth(
        seeded_registry(),
        KeymapPresetId::VsCode,
        PlatformClass::Macos,
    );

    for parity in &report.parity_rows {
        assert_eq!(parity.parity_status, "pass", "{}", parity.command_id);
        assert!(parity.stable_command_id_preserved, "{}", parity.command_id);
        assert_eq!(
            parity.palette_projected_command_id.as_deref(),
            Some(parity.command_id.as_str())
        );
        assert_eq!(
            parity.menu_projected_command_id.as_deref(),
            Some(parity.command_id.as_str())
        );
        assert_eq!(
            parity.keybinding_projected_command_id.as_deref(),
            Some(parity.command_id.as_str())
        );
        assert!(!parity.preview_class.trim().is_empty());
        assert!(!parity.authority_class.trim().is_empty());
    }
}

#[test]
fn help_inspector_surfaces_alpha_keybinding_truth() {
    let lines = build_inspector_lines(
        seeded_registry(),
        KeymapPresetId::VsCode,
        PlatformClass::Macos,
    );
    assert!(
        lines
            .iter()
            .any(|line| line.contains("Alpha keybinding truth")),
        "help inspector should include alpha keybinding truth summary"
    );
    assert!(
        lines
            .iter()
            .any(|line| line.contains("Conflicts linked to migration report")),
        "help inspector should expose conflict/migration linkage"
    );
}

#[test]
fn checked_in_keybinding_artifacts_are_parseable_and_bounded() {
    let claimed = alpha_keybinding_command_ids();

    let parity = read_json("artifacts/commands/alpha_keybinding_parity_report.json");
    assert_eq!(parity["record_kind"], "alpha_keybinding_parity_report");
    assert_eq!(parity["summary"]["status"], "pass");

    let translation = read_json("artifacts/migration/keymap_translation_report_sample.json");
    assert_eq!(
        translation["record_kind"],
        "keymap_translation_report_sample"
    );

    for file_name in ["vs_code.json", "intellij.json", "vim.json", "emacs.json"] {
        let fixture = read_json(&format!("fixtures/keybindings/alpha_presets/{file_name}"));
        assert_eq!(fixture["record_kind"], "alpha_keymap_preset_fixture");
        let rows = fixture["translations"]
            .as_array()
            .expect("translations should be an array");
        let fixture_ids = rows
            .iter()
            .filter_map(|row| row["command_id"].as_str())
            .collect::<BTreeSet<_>>();
        for command_id in &claimed {
            assert!(
                fixture_ids.contains(command_id.as_str()),
                "{file_name} missing {command_id}"
            );
        }
        assert!(rows.iter().all(|row| {
            matches!(
                row["bridge_outcome_class"].as_str(),
                Some("exact" | "translated" | "partial" | "shimmed" | "unsupported")
            )
        }));
    }
}
