//! End-to-end coverage for the notebook debugger-support states, breakpoint
//! affordances, and unsupported-state cues corpus.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use aureline_notebook::{BreakpointAffordance, NotebookDebuggerSupportState, UnsupportedStateCue};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../")
}

fn fixture_root() -> PathBuf {
    repo_root().join(
        "fixtures/notebook/m5/implement_notebook_debugger_support_states_breakpoint_affordances_and_unsupported_state_cues",
    )
}

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: u32,
    case_refs: Vec<String>,
    expected_debugger_support_state_classes: Vec<String>,
    expected_breakpoint_affordance_classes: Vec<String>,
    expected_breakpoint_affordance_posture_classes: Vec<String>,
    expected_unsupported_state_cue_classes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    #[serde(rename = "__fixture__")]
    fixture: FixtureMeta,
    #[serde(default)]
    notebook_debugger_support_state: Option<NotebookDebuggerSupportState>,
    #[serde(default)]
    breakpoint_affordance: Option<BreakpointAffordance>,
    #[serde(default)]
    unsupported_state_cue: Option<UnsupportedStateCue>,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    expected: FixtureExpectations,
}

#[derive(Debug, Deserialize)]
struct FixtureExpectations {
    debugger_support_state_class: Option<String>,
    breakpoint_affordance_posture_class: Option<String>,
    unsupported_state_cue_class: Option<String>,
    findings: ExpectedFindings,
}

#[derive(Debug, Deserialize, Default)]
struct ExpectedFindings {
    #[serde(default)]
    notebook_debugger_support_state: Vec<String>,
    #[serde(default)]
    breakpoint_affordance: Vec<String>,
    #[serde(default)]
    unsupported_state_cue: Vec<String>,
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
    findings: &[aureline_notebook::DebuggerSupportFinding],
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
    let mut observed_state_classes = BTreeMap::new();
    let mut observed_affordance_classes = BTreeMap::new();
    let mut observed_posture_classes = BTreeMap::new();
    let mut observed_cue_classes = BTreeMap::new();

    for case_path in &manifest.case_refs {
        let case = read_case(case_path);
        let name = case.fixture.name.clone();

        if let Some(state) = &case.notebook_debugger_support_state {
            let findings = state.validate();
            assert_findings_match(
                &case
                    .fixture
                    .expected
                    .findings
                    .notebook_debugger_support_state,
                &findings,
            );
            if let Some(expected) = &case.fixture.expected.debugger_support_state_class {
                assert_eq!(
                    state.debugger_support_state_class.as_str(),
                    expected.as_str(),
                    "fixture {name} debugger_support_state_class mismatch"
                );
            }
            observed_state_classes.insert(state.debugger_support_state_class.as_str(), ());
        }

        if let Some(affordance) = &case.breakpoint_affordance {
            let findings = affordance.validate();
            assert_findings_match(
                &case.fixture.expected.findings.breakpoint_affordance,
                &findings,
            );
            if let Some(expected) = &case.fixture.expected.breakpoint_affordance_posture_class {
                assert_eq!(
                    affordance.posture_class.as_str(),
                    expected.as_str(),
                    "fixture {name} breakpoint_affordance_posture_class mismatch"
                );
            }
            observed_affordance_classes.insert(affordance.breakpoint_affordance_class.as_str(), ());
            observed_posture_classes.insert(affordance.posture_class.as_str(), ());
        }

        if let Some(cue) = &case.unsupported_state_cue {
            let findings = cue.validate();
            assert_findings_match(
                &case.fixture.expected.findings.unsupported_state_cue,
                &findings,
            );
            if let Some(expected) = &case.fixture.expected.unsupported_state_cue_class {
                assert_eq!(
                    cue.unsupported_state_cue_class.as_str(),
                    expected.as_str(),
                    "fixture {name} unsupported_state_cue_class mismatch"
                );
            }
            observed_cue_classes.insert(cue.unsupported_state_cue_class.as_str(), ());
        }

        // Surface invariants the spec calls out.
        if let Some(state) = &case.notebook_debugger_support_state {
            assert!(
                !state.summary.trim().is_empty(),
                "fixture {name}: state summary must not be empty"
            );
        }
        if let Some(affordance) = &case.breakpoint_affordance {
            assert!(
                !affordance.summary.trim().is_empty(),
                "fixture {name}: affordance summary must not be empty"
            );
        }
        if let Some(cue) = &case.unsupported_state_cue {
            assert!(
                !cue.summary.trim().is_empty(),
                "fixture {name}: cue summary must not be empty"
            );
        }
    }

    // The manifest's expected vocabulary lists must be exercised by at least
    // one fixture each, so the corpus is not silently shrunk.
    for expected in &manifest.expected_debugger_support_state_classes {
        assert!(
            observed_state_classes.contains_key(expected.as_str()),
            "no fixture exercises debugger_support_state_class '{expected}'"
        );
    }
    for expected in &manifest.expected_breakpoint_affordance_classes {
        assert!(
            observed_affordance_classes.contains_key(expected.as_str()),
            "no fixture exercises breakpoint_affordance_class '{expected}'"
        );
    }
    for expected in &manifest.expected_breakpoint_affordance_posture_classes {
        assert!(
            observed_posture_classes.contains_key(expected.as_str()),
            "no fixture exercises breakpoint_affordance_posture_class '{expected}'"
        );
    }
    for expected in &manifest.expected_unsupported_state_cue_classes {
        assert!(
            observed_cue_classes.contains_key(expected.as_str()),
            "no fixture exercises unsupported_state_cue_class '{expected}'"
        );
    }
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = aureline_notebook::current_notebook_debugger_support_packet()
        .expect("embedded packet must parse");
    let findings = packet.validate();
    assert!(
        findings.is_empty(),
        "embedded packet should validate clean: {findings:?}"
    );
}
