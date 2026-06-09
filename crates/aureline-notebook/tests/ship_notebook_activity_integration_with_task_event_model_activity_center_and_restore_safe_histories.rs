//! End-to-end coverage for the notebook activity integration with task-event
//! model, activity center, and restore-safe histories corpus.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use aureline_notebook::{
    NotebookActivityCenterRow, NotebookRestoreSafeHistory, NotebookTaskEvent,
    NotebookTaskEventKind, NotebookTaskStateClass,
};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../")
}

fn fixture_root() -> PathBuf {
    repo_root().join("fixtures/notebook/m5/ship_notebook_activity_integration_with_task_event_model_activity_center_and_restore_safe_histories")
}

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: u32,
    case_refs: Vec<String>,
    expected_task_event_kinds: Vec<String>,
    expected_task_state_classes: Vec<String>,
    expected_activity_actor_kinds: Vec<String>,
    expected_activity_actions: Vec<String>,
    expected_activity_object_kinds: Vec<String>,
    expected_activity_outcomes: Vec<String>,
    expected_restore_classes: Vec<String>,
    expected_restore_postures: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    #[serde(rename = "__fixture__")]
    fixture: FixtureMeta,
    #[serde(default)]
    notebook_task_event: Option<NotebookTaskEvent>,
    #[serde(default)]
    notebook_activity_center_row: Option<NotebookActivityCenterRow>,
    #[serde(default)]
    notebook_restore_safe_history: Option<NotebookRestoreSafeHistory>,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    expected: FixtureExpectations,
}

#[derive(Debug, Deserialize)]
struct FixtureExpectations {
    #[serde(default)]
    task_event_kind: Option<NotebookTaskEventKind>,
    #[serde(default)]
    task_state_class: Option<NotebookTaskStateClass>,
    #[serde(default)]
    terminal_event: Option<bool>,
    #[serde(default)]
    actor_kind: Option<String>,
    #[serde(default)]
    action: Option<String>,
    #[serde(default)]
    object_kind: Option<String>,
    #[serde(default)]
    outcome: Option<String>,
    #[serde(default)]
    restore_class: Option<String>,
    #[serde(default)]
    restore_posture: Option<String>,
    #[serde(default)]
    has_kernel_session: Option<bool>,
    findings: ExpectedFindings,
}

#[derive(Debug, Deserialize, Default)]
struct ExpectedFindings {
    #[serde(default)]
    notebook_task_event: Vec<String>,
    #[serde(default)]
    notebook_activity_center_row: Vec<String>,
    #[serde(default)]
    notebook_restore_safe_history: Vec<String>,
}

fn read_manifest() -> Manifest {
    let path = fixture_root().join("manifest.yaml");
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read manifest {}: {err}", path.display()));
    serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("parse manifest {}: {err}", path.display()))
}

fn read_case(case_path: &str) -> FixtureCase {
    let path = repo_root().join(case_path);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
    serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

fn assert_findings_match(
    check_ids: &[String],
    findings: &[aureline_notebook::ActivityIntegrationFinding],
) {
    let actual: Vec<String> = findings.iter().map(|f| f.check_id.clone()).collect();
    assert_eq!(
        actual, *check_ids,
        "expected findings {check_ids:?}, got {actual:?}"
    );
}

#[test]
fn manifest_lists_all_case_files() {
    let manifest = read_manifest();
    assert_eq!(manifest.schema_version, 1);

    for case in &manifest.case_refs {
        let path = repo_root().join(case);
        assert!(path.exists(), "manifest references missing file: {case}");
    }

    let dir = fixture_root();
    let mut on_disk: Vec<String> = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().into_string().unwrap())
        .filter(|name| name.ends_with(".yaml"))
        .filter(|name| name != "manifest.yaml")
        .collect();
    on_disk.sort();

    let mut referenced: Vec<String> = manifest
        .case_refs
        .iter()
        .map(|case| {
            Path::new(case)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string()
        })
        .collect();
    referenced.sort();

    assert_eq!(
        on_disk, referenced,
        "manifest case_refs must match yaml files on disk"
    );
}

#[test]
fn every_case_validates_and_matches_expectations() {
    let manifest = read_manifest();
    let mut observed_task_event_kinds = BTreeMap::new();
    let mut observed_task_state_classes = BTreeMap::new();
    let mut observed_actor_kinds = BTreeMap::new();
    let mut observed_actions = BTreeMap::new();
    let mut observed_object_kinds = BTreeMap::new();
    let mut observed_outcomes = BTreeMap::new();
    let mut observed_restore_classes = BTreeMap::new();
    let mut observed_restore_postures = BTreeMap::new();

    for case_path in &manifest.case_refs {
        let case = read_case(case_path);
        let name = case.fixture.name.clone();

        if let Some(event) = case.notebook_task_event {
            let findings = event.validate();
            assert_findings_match(
                &case.fixture.expected.findings.notebook_task_event,
                &findings,
            );

            if let Some(expected_kind) = case.fixture.expected.task_event_kind {
                assert_eq!(
                    event.task_event_kind, expected_kind,
                    "fixture {name} task_event_kind mismatch"
                );
            }
            if let Some(expected_state) = case.fixture.expected.task_state_class {
                assert_eq!(
                    event.task_state_class, expected_state,
                    "fixture {name} task_state_class mismatch"
                );
            }
            if let Some(expected_terminal) = case.fixture.expected.terminal_event {
                assert_eq!(
                    event.task_event_kind.is_terminal(), expected_terminal,
                    "fixture {name} terminal_event mismatch"
                );
            }

            observed_task_event_kinds.insert(event.task_event_kind.as_str(), ());
            observed_task_state_classes.insert(event.task_state_class.as_str(), ());
        }

        if let Some(row) = case.notebook_activity_center_row {
            let findings = row.validate();
            assert_findings_match(
                &case.fixture.expected.findings.notebook_activity_center_row,
                &findings,
            );

            if let Some(ref expected_actor) = case.fixture.expected.actor_kind {
                assert_eq!(
                    row.actor_kind.as_str(), expected_actor.as_str(),
                    "fixture {name} actor_kind mismatch"
                );
            }
            if let Some(ref expected_action) = case.fixture.expected.action {
                assert_eq!(
                    row.action.as_str(), expected_action.as_str(),
                    "fixture {name} action mismatch"
                );
            }
            if let Some(ref expected_object) = case.fixture.expected.object_kind {
                assert_eq!(
                    row.object_kind.as_str(), expected_object.as_str(),
                    "fixture {name} object_kind mismatch"
                );
            }
            if let Some(ref expected_outcome) = case.fixture.expected.outcome {
                assert_eq!(
                    row.outcome.as_str(), expected_outcome.as_str(),
                    "fixture {name} outcome mismatch"
                );
            }

            observed_actor_kinds.insert(row.actor_kind.as_str(), ());
            observed_actions.insert(row.action.as_str(), ());
            observed_object_kinds.insert(row.object_kind.as_str(), ());
            observed_outcomes.insert(row.outcome.as_str(), ());
        }

        if let Some(history) = case.notebook_restore_safe_history {
            let findings = history.validate();
            assert_findings_match(
                &case.fixture.expected.findings.notebook_restore_safe_history,
                &findings,
            );

            if let Some(ref expected_class) = case.fixture.expected.restore_class {
                assert_eq!(
                    history.restore_class.as_str(), expected_class.as_str(),
                    "fixture {name} restore_class mismatch"
                );
            }
            if let Some(ref expected_posture) = case.fixture.expected.restore_posture {
                assert_eq!(
                    history.restore_posture.as_str(), expected_posture.as_str(),
                    "fixture {name} restore_posture mismatch"
                );
            }
            if let Some(expected_has_session) = case.fixture.expected.has_kernel_session {
                assert_eq!(
                    history.kernel_session_id_ref.is_some(), expected_has_session,
                    "fixture {name} has_kernel_session mismatch"
                );
            }

            observed_restore_classes.insert(history.restore_class.as_str(), ());
            observed_restore_postures.insert(history.restore_posture.as_str(), ());
        }
    }

    // The manifest's expected vocabulary lists must be exercised by at least
    // one fixture each, so the corpus is not silently shrunk.
    for expected in &manifest.expected_task_event_kinds {
        assert!(
            observed_task_event_kinds.contains_key(expected.as_str()),
            "no fixture exercises task event kind '{expected}'"
        );
    }
    for expected in &manifest.expected_task_state_classes {
        assert!(
            observed_task_state_classes.contains_key(expected.as_str()),
            "no fixture exercises task state class '{expected}'"
        );
    }
    for expected in &manifest.expected_activity_actor_kinds {
        assert!(
            observed_actor_kinds.contains_key(expected.as_str()),
            "no fixture exercises actor kind '{expected}'"
        );
    }
    for expected in &manifest.expected_activity_actions {
        assert!(
            observed_actions.contains_key(expected.as_str()),
            "no fixture exercises action '{expected}'"
        );
    }
    for expected in &manifest.expected_activity_object_kinds {
        assert!(
            observed_object_kinds.contains_key(expected.as_str()),
            "no fixture exercises object kind '{expected}'"
        );
    }
    for expected in &manifest.expected_activity_outcomes {
        assert!(
            observed_outcomes.contains_key(expected.as_str()),
            "no fixture exercises outcome '{expected}'"
        );
    }
    for expected in &manifest.expected_restore_classes {
        assert!(
            observed_restore_classes.contains_key(expected.as_str()),
            "no fixture exercises restore class '{expected}'"
        );
    }
    for expected in &manifest.expected_restore_postures {
        assert!(
            observed_restore_postures.contains_key(expected.as_str()),
            "no fixture exercises restore posture '{expected}'"
        );
    }
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = aureline_notebook::current_notebook_activity_integration_packet();
    let findings = packet.validate();
    assert!(
        findings.is_empty(),
        "embedded packet should validate clean: {findings:?}"
    );
}
