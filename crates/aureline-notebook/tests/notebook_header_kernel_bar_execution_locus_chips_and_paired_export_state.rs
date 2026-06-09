//! End-to-end coverage for the notebook header, kernel bar, execution-locus
//! chips, and paired-export state corpus.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use aureline_notebook::{
    ExecutionLocusChip, ExecutionLocusChipClass, ExecutionLocusChipState,
    NotebookHeaderKernelBarState,
};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../")
}

fn fixture_root() -> PathBuf {
    repo_root().join("fixtures/notebook/m5/implement_the_notebook_header_kernel_bar_execution_locus_chips_and_paired_export_state")
}

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: u32,
    case_refs: Vec<String>,
    expected_execution_locus_chip_classes: Vec<String>,
    expected_execution_locus_chip_states: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    #[serde(rename = "__fixture__")]
    fixture: FixtureMeta,
    execution_locus_chip: ExecutionLocusChip,
    notebook_header_kernel_bar_state: NotebookHeaderKernelBarState,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    expected: FixtureExpectations,
}

#[derive(Debug, Deserialize)]
struct FixtureExpectations {
    chip_class: ExecutionLocusChipClass,
    chip_state: ExecutionLocusChipState,
    kernel_origin_class: String,
    kernel_selection_state: String,
    document_trust_class: String,
    dirty_state_class: String,
    paired_export_posture: String,
    findings: ExpectedFindings,
}

#[derive(Debug, Deserialize, Default)]
struct ExpectedFindings {
    #[serde(default)]
    execution_locus_chip: Vec<String>,
    #[serde(default)]
    notebook_header_kernel_bar_state: Vec<String>,
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
    findings: &[aureline_notebook::HeaderKernelBarFinding],
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
    let mut observed_chip_classes = BTreeMap::new();
    let mut observed_chip_states = BTreeMap::new();

    for case_path in &manifest.case_refs {
        let case = read_case(case_path);
        let name = case.fixture.name.clone();

        // Validators agree with the expected findings list.
        let chip_findings = case.execution_locus_chip.validate();
        assert_findings_match(
            &case.fixture.expected.findings.execution_locus_chip,
            &chip_findings,
        );

        let state_findings = case.notebook_header_kernel_bar_state.validate();
        assert_findings_match(
            &case
                .fixture
                .expected
                .findings
                .notebook_header_kernel_bar_state,
            &state_findings,
        );

        // Closed-vocabulary expectations are reflected in the records.
        assert_eq!(
            case.execution_locus_chip.chip_class, case.fixture.expected.chip_class,
            "fixture {name} chip_class mismatch"
        );
        assert_eq!(
            case.execution_locus_chip.chip_state, case.fixture.expected.chip_state,
            "fixture {name} chip_state mismatch"
        );
        assert_eq!(
            case.notebook_header_kernel_bar_state
                .kernel_origin_class
                .as_str(),
            case.fixture.expected.kernel_origin_class,
            "fixture {name} kernel_origin_class mismatch"
        );
        assert_eq!(
            case.notebook_header_kernel_bar_state
                .kernel_selection_state
                .as_str(),
            case.fixture.expected.kernel_selection_state,
            "fixture {name} kernel_selection_state mismatch"
        );
        assert_eq!(
            case.notebook_header_kernel_bar_state
                .document_trust_class
                .as_str(),
            case.fixture.expected.document_trust_class,
            "fixture {name} document_trust_class mismatch"
        );
        assert_eq!(
            case.notebook_header_kernel_bar_state
                .dirty_state_class
                .as_str(),
            case.fixture.expected.dirty_state_class,
            "fixture {name} dirty_state_class mismatch"
        );
        assert_eq!(
            case.notebook_header_kernel_bar_state
                .paired_export_posture
                .as_str(),
            case.fixture.expected.paired_export_posture,
            "fixture {name} paired_export_posture mismatch"
        );

        // Surface invariants the spec calls out.
        assert!(
            !case
                .notebook_header_kernel_bar_state
                .available_actions
                .is_empty(),
            "fixture {name}: kernel bar must expose at least one action"
        );
        assert!(
            case.notebook_header_kernel_bar_state.auto_rerun_forbidden,
            "fixture {name}: auto_rerun_forbidden must be true"
        );

        observed_chip_classes.insert(case.execution_locus_chip.chip_class.as_str(), ());
        observed_chip_states.insert(case.execution_locus_chip.chip_state.as_str(), ());
    }

    // The manifest's expected vocabulary lists must be exercised by at least
    // one fixture each, so the corpus is not silently shrunk.
    for expected in &manifest.expected_execution_locus_chip_classes {
        assert!(
            observed_chip_classes.contains_key(expected.as_str()),
            "no fixture exercises chip class '{expected}'"
        );
    }
    for expected in &manifest.expected_execution_locus_chip_states {
        assert!(
            observed_chip_states.contains_key(expected.as_str()),
            "no fixture exercises chip state '{expected}'"
        );
    }
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = aureline_notebook::current_notebook_header_kernel_bar_packet()
        .expect("embedded packet must parse");
    let findings = packet.validate();
    assert!(
        findings.is_empty(),
        "embedded packet should validate clean: {findings:?}"
    );
}
