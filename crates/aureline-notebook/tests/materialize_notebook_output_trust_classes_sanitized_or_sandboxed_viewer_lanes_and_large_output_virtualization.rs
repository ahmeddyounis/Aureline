//! End-to-end coverage for the notebook output trust classes, sanitized or
//! sandboxed viewer lanes, and large-output virtualization corpus.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use aureline_notebook::{LargeOutputVirtualizationRecord, NotebookOutputViewerLane};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../")
}

fn fixture_root() -> PathBuf {
    repo_root().join(
        "fixtures/notebook/m5/materialize_notebook_output_trust_classes_sanitized_or_sandboxed_viewer_lanes_and_large_output_virtualization",
    )
}

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: u32,
    case_refs: Vec<String>,
    expected_output_viewer_lane_classes: Vec<String>,
    expected_output_size_buckets: Vec<String>,
    expected_output_virtualization_state_classes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    #[serde(rename = "__fixture__")]
    fixture: FixtureMeta,
    notebook_output_viewer_lane: NotebookOutputViewerLane,
    large_output_virtualization: LargeOutputVirtualizationRecord,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    expected: FixtureExpectations,
}

#[derive(Debug, Deserialize)]
struct FixtureExpectations {
    trust_class: String,
    viewer_lane_class: String,
    size_bucket: String,
    virtualization_state_class: String,
    compatible_viewer_available: bool,
    findings: ExpectedFindings,
}

#[derive(Debug, Deserialize, Default)]
struct ExpectedFindings {
    #[serde(default)]
    notebook_output_viewer_lane: Vec<String>,
    #[serde(default)]
    large_output_virtualization: Vec<String>,
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
    findings: &[aureline_notebook::OutputViewerFinding],
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
    let mut observed_lanes = BTreeMap::new();
    let mut observed_buckets = BTreeMap::new();
    let mut observed_states = BTreeMap::new();

    for case_path in &manifest.case_refs {
        let case = read_case(case_path);
        let name = case.fixture.name.clone();

        // Validators agree with the expected findings list.
        let lane_findings = case.notebook_output_viewer_lane.validate();
        assert_findings_match(
            &case.fixture.expected.findings.notebook_output_viewer_lane,
            &lane_findings,
        );

        let virt_findings = case.large_output_virtualization.validate();
        assert_findings_match(
            &case.fixture.expected.findings.large_output_virtualization,
            &virt_findings,
        );

        // Closed-vocabulary expectations are reflected in the records.
        assert_eq!(
            case.notebook_output_viewer_lane.trust_class.as_str(),
            case.fixture.expected.trust_class,
            "fixture {name} trust_class mismatch"
        );
        assert_eq!(
            case.notebook_output_viewer_lane.viewer_lane_class.as_str(),
            case.fixture.expected.viewer_lane_class,
            "fixture {name} viewer_lane_class mismatch"
        );
        assert_eq!(
            case.notebook_output_viewer_lane.size_bucket.as_str(),
            case.fixture.expected.size_bucket,
            "fixture {name} size_bucket mismatch"
        );
        assert_eq!(
            case.notebook_output_viewer_lane
                .virtualization_state_class
                .as_str(),
            case.fixture.expected.virtualization_state_class,
            "fixture {name} virtualization_state_class mismatch"
        );
        assert_eq!(
            case.notebook_output_viewer_lane.compatible_viewer_available,
            case.fixture.expected.compatible_viewer_available,
            "fixture {name} compatible_viewer_available mismatch"
        );

        // Surface invariants the spec calls out.
        assert!(
            !case.notebook_output_viewer_lane.summary.trim().is_empty(),
            "fixture {name}: viewer lane summary must not be empty"
        );
        assert!(
            !case.large_output_virtualization.summary.trim().is_empty(),
            "fixture {name}: virtualization summary must not be empty"
        );

        observed_lanes.insert(
            case.notebook_output_viewer_lane.viewer_lane_class.as_str(),
            (),
        );
        observed_buckets.insert(case.large_output_virtualization.size_bucket.as_str(), ());
        observed_states.insert(
            case.large_output_virtualization
                .virtualization_state_class
                .as_str(),
            (),
        );
    }

    // The manifest's expected vocabulary lists must be exercised by at least
    // one fixture each, so the corpus is not silently shrunk.
    for expected in &manifest.expected_output_viewer_lane_classes {
        assert!(
            observed_lanes.contains_key(expected.as_str()),
            "no fixture exercises viewer lane class '{expected}'"
        );
    }
    for expected in &manifest.expected_output_size_buckets {
        assert!(
            observed_buckets.contains_key(expected.as_str()),
            "no fixture exercises size bucket '{expected}'"
        );
    }
    for expected in &manifest.expected_output_virtualization_state_classes {
        assert!(
            observed_states.contains_key(expected.as_str()),
            "no fixture exercises virtualization state class '{expected}'"
        );
    }
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = aureline_notebook::current_notebook_output_viewer_packet()
        .expect("embedded packet must parse");
    let findings = packet.validate();
    assert!(
        findings.is_empty(),
        "embedded packet should validate clean: {findings:?}"
    );
}
