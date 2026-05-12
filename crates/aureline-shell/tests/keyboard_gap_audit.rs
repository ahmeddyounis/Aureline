//! Fixture-backed tests for the launch-critical keyboard gap audit.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_commands::registry::seeded_registry;
use aureline_input::keybindings::PlatformClass;
use aureline_input::presets::KeymapPresetId;
use aureline_shell::help::keybinding_inspector::build_inspector_lines;
use aureline_shell::help::keyboard_gap_audit::{
    materialize_alpha_keyboard_gap_audit, AlphaKeyboardGapAudit,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct KeyboardPathFixture {
    fixture_id: String,
    required_surface_ids: Vec<String>,
    required_command_ids: Vec<String>,
    required_gap_surface_ids: Vec<String>,
    required_explicit_non_goal_surface_ids: Vec<String>,
    required_focus_return_surface_ids: Vec<String>,
    required_preset_refs: Vec<String>,
    required_preview_command_id: String,
    required_conflict_preset_ref: String,
    required_conflict_sequence: String,
    acceptance: AcceptanceFixture,
}

#[derive(Debug, Deserialize)]
struct AcceptanceFixture {
    every_claimed_path_has_route_or_explicit_non_goal: bool,
    resolver_source_attribution_required: bool,
    conflict_reporting_required: bool,
    remaining_gaps_are_actionable: bool,
}

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/accessibility/m2_keyboard_paths/launch_keyboard_path_matrix.yaml")
}

fn load_fixture() -> KeyboardPathFixture {
    let path = fixture_path();
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

fn audit() -> AlphaKeyboardGapAudit {
    materialize_alpha_keyboard_gap_audit(
        seeded_registry(),
        KeymapPresetId::VsCode,
        PlatformClass::Macos,
    )
}

#[test]
fn keyboard_gap_audit_covers_launch_critical_alpha_paths() {
    let fixture = load_fixture();
    let audit = audit();
    assert_eq!(audit.record_kind, "alpha_keyboard_gap_audit_record");

    let row_ids: BTreeSet<&str> = audit
        .rows
        .iter()
        .map(|row| row.surface_id.as_str())
        .collect();
    for required in &fixture.required_surface_ids {
        assert!(
            row_ids.contains(required.as_str()),
            "fixture {} missing required surface row {required}",
            fixture.fixture_id
        );
    }

    let exposed_command_ids: BTreeSet<&str> = audit
        .rows
        .iter()
        .flat_map(|row| row.command_exposures.iter())
        .map(|exposure| exposure.command_id.as_str())
        .collect();
    for required in &fixture.required_command_ids {
        assert!(
            exposed_command_ids.contains(required.as_str()),
            "fixture {} missing command exposure {required}",
            fixture.fixture_id
        );
    }

    assert_preview_required_command_is_claimed(&audit, &fixture.required_preview_command_id);
    assert_focus_return_is_recorded(&audit, &fixture);
    assert_every_claimed_path_has_route_or_gap(&audit, &fixture);
    assert_gaps_are_actionable(&audit, &fixture);
    assert_preset_profiles_are_covered(&audit, &fixture);
    assert_conflicts_are_reported(&audit, &fixture);
}

#[test]
fn keybinding_inspector_renders_keyboard_gap_audit_summary() {
    let lines = build_inspector_lines(
        seeded_registry(),
        KeymapPresetId::VsCode,
        PlatformClass::Macos,
    );
    assert!(
        lines
            .iter()
            .any(|line| line.contains("Alpha keyboard audit")),
        "reachable keybinding help must include the keyboard gap audit"
    );
    assert!(
        lines
            .iter()
            .any(|line| line.contains("Trust and restricted-mode review")),
        "audit summary should expose trust-surface command gap"
    );
}

fn assert_preview_required_command_is_claimed(audit: &AlphaKeyboardGapAudit, command_id: &str) {
    let row = audit
        .rows
        .iter()
        .find(|row| row.surface_id == "command_preview.import_profile")
        .expect("preview-required row should exist");
    assert!(
        row.command_exposures
            .iter()
            .any(|exposure| exposure.command_id == command_id),
        "preview-required row should cite {command_id}"
    );
    let entry = seeded_registry()
        .get(command_id)
        .expect("preview command should exist in registry");
    assert_eq!(entry.descriptor.preview_class, "structured_diff_preview");
}

fn assert_focus_return_is_recorded(audit: &AlphaKeyboardGapAudit, fixture: &KeyboardPathFixture) {
    for surface_id in &fixture.required_focus_return_surface_ids {
        let row = audit
            .rows
            .iter()
            .find(|row| row.surface_id == *surface_id)
            .unwrap_or_else(|| panic!("missing focus row {surface_id}"));
        assert!(
            !row.focus_return_state.trim().is_empty(),
            "surface {surface_id} must record a focus-return state"
        );
        assert!(
            !row.focus_return_target_ref.trim().is_empty(),
            "surface {surface_id} must record a focus-return target"
        );
    }
}

fn assert_every_claimed_path_has_route_or_gap(
    audit: &AlphaKeyboardGapAudit,
    fixture: &KeyboardPathFixture,
) {
    assert!(
        fixture
            .acceptance
            .every_claimed_path_has_route_or_explicit_non_goal
    );
    for row in &audit.rows {
        if row.coverage_state == "explicit_non_goal" {
            assert!(
                row.explicit_non_goal.is_some(),
                "{} should carry a non-goal explanation",
                row.surface_id
            );
            continue;
        }
        assert!(
            !row.keyboard_route.trim().is_empty(),
            "{} should document a keyboard route or explicit gap",
            row.surface_id
        );
        if row.coverage_state == "gap_action_required" {
            assert!(
                row.actionable_gap.is_some(),
                "{} should name an actionable gap",
                row.surface_id
            );
        }
    }
}

fn assert_gaps_are_actionable(audit: &AlphaKeyboardGapAudit, fixture: &KeyboardPathFixture) {
    assert!(fixture.acceptance.remaining_gaps_are_actionable);
    let gap_ids: BTreeSet<&str> = audit
        .remaining_gaps
        .iter()
        .map(|gap| gap.surface_id.as_str())
        .collect();
    for required in &fixture.required_gap_surface_ids {
        assert!(
            gap_ids.contains(required.as_str()),
            "expected actionable gap for {required}"
        );
    }
    for required in &fixture.required_explicit_non_goal_surface_ids {
        let row = audit
            .rows
            .iter()
            .find(|row| row.surface_id == *required)
            .unwrap_or_else(|| panic!("missing non-goal row {required}"));
        assert_eq!(row.coverage_state, "explicit_non_goal");
        assert!(row.explicit_non_goal.is_some());
    }
}

fn assert_preset_profiles_are_covered(
    audit: &AlphaKeyboardGapAudit,
    fixture: &KeyboardPathFixture,
) {
    assert!(fixture.acceptance.resolver_source_attribution_required);
    let preset_refs: BTreeSet<&str> = audit
        .preset_coverage
        .iter()
        .map(|row| row.preset_ref.as_str())
        .collect();
    for required in &fixture.required_preset_refs {
        assert!(
            preset_refs.contains(required.as_str()),
            "missing preset coverage for {required}"
        );
    }
    for coverage in &audit.preset_coverage {
        assert_eq!(coverage.coverage_state, "covers_claimed_command_set");
    }
    for row in audit
        .rows
        .iter()
        .filter(|row| row.coverage_state == "covered")
    {
        for exposure in &row.command_exposures {
            assert_eq!(exposure.exposure_state, "registered_bound");
            assert!(
                exposure
                    .winning_source_ref
                    .as_deref()
                    .is_some_and(|source| source.starts_with("preset:keymap:")),
                "{} should carry preset-backed winning-source attribution",
                exposure.command_id
            );
        }
    }
}

fn assert_conflicts_are_reported(audit: &AlphaKeyboardGapAudit, fixture: &KeyboardPathFixture) {
    assert!(fixture.acceptance.conflict_reporting_required);
    assert!(
        audit.conflict_reports.iter().any(|conflict| {
            conflict.preset_ref == fixture.required_conflict_preset_ref
                && conflict.literal_sequence == fixture.required_conflict_sequence
                && !conflict.losing_command_ids.is_empty()
        }),
        "expected resolver conflict for {} {}",
        fixture.required_conflict_preset_ref,
        fixture.required_conflict_sequence
    );
}
