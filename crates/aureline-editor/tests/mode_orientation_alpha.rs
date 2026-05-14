use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_editor::{
    build_alpha_mode_state_record, build_alpha_orientation_truth_record, AlphaModeStateInput,
    AlphaOrientationInput, EditorModeClass, EditorModeStateRecord, EditorOrientationTruthRecord,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ModeOrientationFixture {
    record_kind: String,
    schema_version: u32,
    expected: ExpectedModeOrientation,
}

#[derive(Debug, Deserialize)]
struct ExpectedModeOrientation {
    register_route_kinds: Vec<String>,
    fail_closed_route_refs: Vec<String>,
    sequence_states: Vec<String>,
    macro_outcomes: Vec<String>,
    recovery_action_refs: Vec<String>,
    orientation: ExpectedOrientation,
}

#[derive(Debug, Deserialize)]
struct ExpectedOrientation {
    multi_cursor_count: usize,
    min_fold_summary_count: usize,
    overview_availability: String,
    required_replacement_routes: Vec<String>,
    breadcrumb_state: String,
}

#[test]
fn mode_state_record_exposes_sequence_register_macro_and_recovery_truth() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, "alpha_mode_orientation_fixture");
    assert_eq!(fixture.schema_version, 1);

    let mode = mode_record();
    assert_eq!(mode.record_kind, EditorModeStateRecord::RECORD_KIND);
    assert!(mode.covers_required_register_routes());
    assert!(mode.blocked_or_unsupported_routes_fail_closed());
    assert!(mode.unsafe_macro_replays_are_bounded());
    assert!(mode.exposes_partial_and_unsupported_sequences());
    assert!(mode.has_required_recovery_paths());

    let route_kinds = mode
        .register_routes
        .iter()
        .map(|route| route.route_kind.as_str())
        .collect::<BTreeSet<_>>();
    for required in &fixture.expected.register_route_kinds {
        assert!(
            route_kinds.contains(required.as_str()),
            "missing register route kind {required}"
        );
    }

    let fail_closed_refs = mode
        .register_routes
        .iter()
        .filter(|route| route.fail_closed)
        .map(|route| route.route_ref.as_str())
        .collect::<BTreeSet<_>>();
    for required in &fixture.expected.fail_closed_route_refs {
        assert!(
            fail_closed_refs.contains(required.as_str()),
            "missing fail-closed route {required}"
        );
    }

    let sequence_states = mode
        .sequence_guides
        .iter()
        .map(|guide| guide.sequence_state.as_str())
        .collect::<BTreeSet<_>>();
    for required in &fixture.expected.sequence_states {
        assert!(
            sequence_states.contains(required.as_str()),
            "missing sequence state {required}"
        );
    }

    let macro_outcomes = mode
        .macro_replay_reviews
        .iter()
        .map(|review| review.outcome_class.as_str())
        .collect::<BTreeSet<_>>();
    for required in &fixture.expected.macro_outcomes {
        assert!(
            macro_outcomes.contains(required.as_str()),
            "missing macro outcome {required}"
        );
    }

    let recovery_refs = mode
        .recovery_actions
        .iter()
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
fn orientation_truth_keeps_visual_aids_optional_and_accessible() {
    let fixture = load_fixture();
    let orientation = orientation_record();
    assert_eq!(
        orientation.record_kind,
        EditorOrientationTruthRecord::RECORD_KIND
    );
    assert!(orientation.multi_cursor_count_is_visible());
    assert!(orientation.fold_summaries_preserve_hidden_state());
    assert!(orientation.breadcrumbs_preserve_continuity());
    assert!(orientation.overview_degradation_has_alternate_path());

    assert_eq!(
        orientation.multi_cursor.caret_count,
        fixture.expected.orientation.multi_cursor_count
    );
    assert!(
        orientation.fold_summaries.len() >= fixture.expected.orientation.min_fold_summary_count
    );
    assert_eq!(
        orientation.overview_aid.availability.as_str(),
        fixture.expected.orientation.overview_availability
    );
    assert_eq!(
        orientation.breadcrumbs.symbol_path_state,
        fixture.expected.orientation.breadcrumb_state
    );

    let replacement_routes = orientation
        .overview_aid
        .replacement_route_refs
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    for required in &fixture.expected.orientation.required_replacement_routes {
        assert!(
            replacement_routes.contains(required.as_str()),
            "missing replacement route {required}"
        );
    }
}

fn mode_record() -> EditorModeStateRecord {
    build_alpha_mode_state_record(AlphaModeStateInput {
        mode_state_id: "mode-state:test:vim".to_string(),
        source_preset_ref: "preset:keymap:vim".to_string(),
        source_preset_label: "Vim".to_string(),
        current_mode: EditorModeClass::Normal,
        surface_ref: "surface:editor.source.alpha".to_string(),
        platform_class: "macos".to_string(),
    })
}

fn orientation_record() -> EditorOrientationTruthRecord {
    build_alpha_orientation_truth_record(AlphaOrientationInput {
        orientation_record_id: "orientation:test:source-editor".to_string(),
        document_ref: "doc:test:source".to_string(),
        surface_ref: "surface:editor.source.alpha".to_string(),
        low_resource_mode: true,
    })
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
