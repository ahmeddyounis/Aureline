//! End-to-end coverage for the notebook cell chrome, run-scope controls, and
//! durable execution-state rows corpus.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use aureline_notebook::{
    CellChromeStatusClass, DurableExecutionStateRow, NotebookCellChrome, RunScopeControl,
    RunScopeControlLockReasonClass,
};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../")
}

fn fixture_root() -> PathBuf {
    repo_root().join("fixtures/notebook/m5/ship_notebook_cell_chrome_run_scope_controls_and_durable_execution_state_rows")
}

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: u32,
    case_refs: Vec<String>,
    expected_cell_chrome_status_classes: Vec<String>,
    expected_cell_chrome_action_classes: Vec<String>,
    expected_run_scope_control_lock_reason_classes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    #[serde(rename = "__fixture__")]
    fixture: FixtureMeta,
    notebook_cell_chrome: NotebookCellChrome,
    run_scope_control: RunScopeControl,
    durable_execution_state_row: DurableExecutionStateRow,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    expected: FixtureExpectations,
}

#[derive(Debug, Deserialize)]
struct FixtureExpectations {
    cell_status_class: CellChromeStatusClass,
    cell_chrome_action_count: usize,
    run_scope_control_changeable: bool,
    lock_reason_class: RunScopeControlLockReasonClass,
    durable_outcome_class: String,
    stale_output: bool,
    findings: ExpectedFindings,
}

#[derive(Debug, Deserialize, Default)]
struct ExpectedFindings {
    #[serde(default)]
    notebook_cell_chrome: Vec<String>,
    #[serde(default)]
    run_scope_control: Vec<String>,
    #[serde(default)]
    durable_execution_state_row: Vec<String>,
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
    findings: &[aureline_notebook::CellChromeFinding],
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
    let mut observed_status_classes = BTreeMap::new();
    let mut observed_action_classes = BTreeMap::new();
    let mut observed_lock_reason_classes = BTreeMap::new();

    for case_path in &manifest.case_refs {
        let case = read_case(case_path);
        let name = case.fixture.name.clone();

        // Validators agree with the expected findings list.
        let chrome_findings = case.notebook_cell_chrome.validate();
        assert_findings_match(
            &case.fixture.expected.findings.notebook_cell_chrome,
            &chrome_findings,
        );

        let control_findings = case.run_scope_control.validate();
        assert_findings_match(
            &case.fixture.expected.findings.run_scope_control,
            &control_findings,
        );

        let row_findings = case.durable_execution_state_row.validate();
        assert_findings_match(
            &case.fixture.expected.findings.durable_execution_state_row,
            &row_findings,
        );

        // Closed-vocabulary expectations are reflected in the records.
        assert_eq!(
            case.notebook_cell_chrome.cell_status_class,
            case.fixture.expected.cell_status_class,
            "fixture {name} cell_status_class mismatch"
        );
        assert_eq!(
            case.notebook_cell_chrome.available_actions.len(),
            case.fixture.expected.cell_chrome_action_count,
            "fixture {name} cell_chrome_action_count mismatch"
        );
        assert_eq!(
            case.run_scope_control.scope_changeable,
            case.fixture.expected.run_scope_control_changeable,
            "fixture {name} run_scope_control_changeable mismatch"
        );
        assert_eq!(
            case.run_scope_control.lock_reason_class,
            case.fixture.expected.lock_reason_class,
            "fixture {name} lock_reason_class mismatch"
        );
        assert_eq!(
            case.durable_execution_state_row
                .durable_outcome_class
                .as_str(),
            case.fixture.expected.durable_outcome_class,
            "fixture {name} durable_outcome_class mismatch"
        );
        assert_eq!(
            case.durable_execution_state_row.stale_output,
            case.fixture.expected.stale_output,
            "fixture {name} stale_output mismatch"
        );

        // Surface invariants the spec calls out.
        assert!(
            !case.notebook_cell_chrome.execution_badge_label.is_empty(),
            "fixture {name}: execution_badge_label must be non-empty"
        );
        assert!(
            !case.run_scope_control.available_scopes.is_empty(),
            "fixture {name}: available_scopes must not be empty"
        );
        assert!(
            case.run_scope_control
                .available_scopes
                .contains(&case.run_scope_control.current_scope),
            "fixture {name}: current_scope must be in available_scopes"
        );

        observed_status_classes.insert(case.notebook_cell_chrome.cell_status_class.as_str(), ());
        for action in &case.notebook_cell_chrome.available_actions {
            observed_action_classes.insert(action.as_str(), ());
        }
        observed_lock_reason_classes
            .insert(case.run_scope_control.lock_reason_class.as_str(), ());
    }

    // The manifest's expected vocabulary lists must be exercised by at least
    // one fixture each, so the corpus is not silently shrunk.
    for expected in &manifest.expected_cell_chrome_status_classes {
        assert!(
            observed_status_classes.contains_key(expected.as_str()),
            "no fixture exercises cell chrome status class '{expected}'"
        );
    }
    for expected in &manifest.expected_cell_chrome_action_classes {
        assert!(
            observed_action_classes.contains_key(expected.as_str()),
            "no fixture exercises cell chrome action class '{expected}'"
        );
    }
    for expected in &manifest.expected_run_scope_control_lock_reason_classes {
        assert!(
            observed_lock_reason_classes.contains_key(expected.as_str()),
            "no fixture exercises run-scope lock reason class '{expected}'"
        );
    }
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = aureline_notebook::current_notebook_cell_chrome_packet()
        .expect("embedded packet must parse");
    let findings = packet.validate();
    assert!(
        findings.is_empty(),
        "embedded packet should validate clean: {findings:?}"
    );
}
