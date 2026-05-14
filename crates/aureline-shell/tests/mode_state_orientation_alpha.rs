use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_commands::registry::seeded_registry;
use aureline_input::keybindings::PlatformClass;
use aureline_input::presets::KeymapPresetId;
use aureline_shell::help::keybinding_inspector::build_inspector_lines;
use aureline_shell::help::mode_state_orientation::materialize_alpha_mode_orientation_report;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
struct ModeOrientationFixture {
    record_kind: String,
    schema_version: u32,
    expected: ExpectedModeOrientation,
}

#[derive(Debug, Deserialize)]
struct ExpectedModeOrientation {
    preset_refs: Vec<String>,
    register_route_kinds: Vec<String>,
    fail_closed_route_refs: Vec<String>,
    sequence_states: Vec<String>,
    macro_outcomes: Vec<String>,
    recovery_action_refs: Vec<String>,
}

#[test]
fn mode_orientation_report_covers_preset_lanes_and_support_exports() {
    let fixture = load_fixture();
    let report =
        materialize_alpha_mode_orientation_report(KeymapPresetId::Vim, PlatformClass::Macos);

    assert_eq!(fixture.record_kind, "alpha_mode_orientation_fixture");
    assert_eq!(fixture.schema_version, 1);
    assert_eq!(report.record_kind, "alpha_mode_orientation_report");
    assert_eq!(report.summary.status, "pass");
    assert_eq!(
        report.summary.preset_lane_count,
        fixture.expected.preset_refs.len()
    );
    assert_eq!(report.summary.register_route_kind_count, 7);
    assert_eq!(report.summary.orientation_truth_checks_passed, 4);

    let preset_refs = report
        .mode_state_records
        .iter()
        .map(|record| record.source_preset_ref.as_str())
        .collect::<BTreeSet<_>>();
    for required in &fixture.expected.preset_refs {
        assert!(
            preset_refs.contains(required.as_str()),
            "missing preset lane {required}"
        );
    }

    for record in &report.mode_state_records {
        assert!(record.covers_required_register_routes());
        assert!(record.blocked_or_unsupported_routes_fail_closed());
        assert!(record.unsafe_macro_replays_are_bounded());
        assert!(record.has_required_recovery_paths());
    }

    assert_eq!(
        report.settings_projection_rows.len(),
        report.mode_state_records.len()
    );
    for row in &report.settings_projection_rows {
        assert!(row.explains_blocked_routes_and_recovery());
    }

    let route_kinds = report
        .mode_state_records
        .iter()
        .flat_map(|record| record.register_routes.iter())
        .map(|route| route.route_kind.as_str())
        .collect::<BTreeSet<_>>();
    for required in &fixture.expected.register_route_kinds {
        assert!(
            route_kinds.contains(required.as_str()),
            "missing route kind {required}"
        );
    }

    let fail_closed_refs = report
        .mode_state_records
        .iter()
        .flat_map(|record| record.register_routes.iter())
        .filter(|route| route.fail_closed)
        .map(|route| route.route_ref.as_str())
        .collect::<BTreeSet<_>>();
    for required in &fixture.expected.fail_closed_route_refs {
        assert!(
            fail_closed_refs.contains(required.as_str()),
            "missing fail-closed route {required}"
        );
    }

    let sequence_states = report
        .mode_state_records
        .iter()
        .flat_map(|record| record.sequence_guides.iter())
        .map(|guide| guide.sequence_state.as_str())
        .collect::<BTreeSet<_>>();
    for required in &fixture.expected.sequence_states {
        assert!(
            sequence_states.contains(required.as_str()),
            "missing sequence state {required}"
        );
    }

    let macro_outcomes = report
        .mode_state_records
        .iter()
        .flat_map(|record| record.macro_replay_reviews.iter())
        .map(|review| review.outcome_class.as_str())
        .collect::<BTreeSet<_>>();
    for required in &fixture.expected.macro_outcomes {
        assert!(
            macro_outcomes.contains(required.as_str()),
            "missing macro outcome {required}"
        );
    }

    let recovery_refs = report
        .mode_state_records
        .iter()
        .flat_map(|record| record.recovery_actions.iter())
        .map(|action| action.action_ref.as_str())
        .collect::<BTreeSet<_>>();
    for required in &fixture.expected.recovery_action_refs {
        assert!(
            recovery_refs.contains(required.as_str()),
            "missing recovery action {required}"
        );
    }
}

#[test]
fn keybinding_inspector_renders_mode_orientation_truth() {
    let lines = build_inspector_lines(seeded_registry(), KeymapPresetId::Vim, PlatformClass::Macos);
    assert!(
        lines
            .iter()
            .any(|line| line.contains("Alpha mode and orientation truth")),
        "keybinding inspector should include mode/orientation truth"
    );
    assert!(
        lines.iter().any(|line| {
            line.contains("remote_clipboard_bridge") && line.contains("unsupported")
        }),
        "inspector should expose unsupported remote clipboard route"
    );
    assert!(
        lines
            .iter()
            .any(|line| line.contains("Orientation: 3 carets")),
        "inspector should expose multi-cursor orientation state"
    );
}

#[test]
fn checked_in_mode_orientation_artifacts_are_parseable() {
    let parity = read_json("artifacts/commands/alpha_mode_state_parity_report.json");
    assert_eq!(parity["record_kind"], "alpha_mode_state_parity_report");
    assert_eq!(parity["summary"]["status"], "pass");
    assert_eq!(parity["summary"]["register_route_kind_count"], 7);
    assert!(
        parity["orientation_aids"]["overview_degraded_with_alternate_routes"]
            .as_bool()
            .unwrap_or(false)
    );

    let fixture =
        read_json("fixtures/editor/mode_and_orientation/alpha_mode_and_orientation_cases.json");
    assert_eq!(fixture["record_kind"], "alpha_mode_orientation_fixture");
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_fixture() -> ModeOrientationFixture {
    let path = repo_root()
        .join("fixtures/editor/mode_and_orientation/alpha_mode_and_orientation_cases.json");
    let payload = fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn read_json(path: &str) -> Value {
    let path = repo_root().join(path);
    let payload = fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}
