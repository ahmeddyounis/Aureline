//! End-to-end coverage for the notebook variable explorer, live or snapshot or
//! stale labels, and typed export corpus.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use aureline_notebook::{
    NotebookVariableExplorer, VariableExplorerTypedExport,
};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../")
}

fn fixture_root() -> PathBuf {
    repo_root().join(
        "fixtures/notebook/m5/implement_the_notebook_variable_explorer_live_or_snapshot_or_stale_labels_and_typed_export",
    )
}

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: u32,
    case_refs: Vec<String>,
    expected_sort_classes: Vec<String>,
    expected_filter_classes: Vec<String>,
    expected_export_format_classes: Vec<String>,
    expected_export_posture_classes: Vec<String>,
    expected_export_scope_classes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    #[serde(rename = "__fixture__")]
    fixture: FixtureMeta,
    notebook_variable_explorer: NotebookVariableExplorer,
    variable_explorer_typed_export: VariableExplorerTypedExport,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    expected: FixtureExpectations,
}

#[derive(Debug, Deserialize)]
struct FixtureExpectations {
    sort_class: Option<String>,
    filter_class: Option<String>,
    entry_count_visible: Option<u32>,
    entry_count_total: Option<u32>,
    has_more_entries: Option<bool>,
    truncation_notice_visible: Option<bool>,
    export_format_class: Option<String>,
    export_posture_class: Option<String>,
    export_scope_class: Option<String>,
    redaction_required: Option<bool>,
    findings: ExpectedFindings,
}

#[derive(Debug, Deserialize, Default)]
struct ExpectedFindings {
    #[serde(default)]
    notebook_variable_explorer: Vec<String>,
    #[serde(default)]
    variable_explorer_typed_export: Vec<String>,
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

fn assert_findings_match(check_ids: &[String], findings: &[aureline_notebook::VariableExplorerFinding]) {
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
    let mut observed_sorts = BTreeMap::new();
    let mut observed_filters = BTreeMap::new();
    let mut observed_formats = BTreeMap::new();
    let mut observed_postures = BTreeMap::new();
    let mut observed_scopes = BTreeMap::new();

    for case_path in &manifest.case_refs {
        let case = read_case(case_path);
        let name = case.fixture.name.clone();

        // Validators agree with the expected findings list.
        let explorer_findings = case.notebook_variable_explorer.validate();
        assert_findings_match(
            &case.fixture.expected.findings.notebook_variable_explorer,
            &explorer_findings,
        );

        let export_findings = case.variable_explorer_typed_export.validate();
        assert_findings_match(
            &case.fixture.expected.findings.variable_explorer_typed_export,
            &export_findings,
        );

        // Closed-vocabulary expectations are reflected in the records.
        if let Some(expected) = &case.fixture.expected.sort_class {
            assert_eq!(
                case.notebook_variable_explorer.sort_class.as_str(),
                expected.as_str(),
                "fixture {name} sort_class mismatch"
            );
        }
        if let Some(expected) = &case.fixture.expected.filter_class {
            assert_eq!(
                case.notebook_variable_explorer.filter_class.as_str(),
                expected.as_str(),
                "fixture {name} filter_class mismatch"
            );
        }
        if let Some(expected) = case.fixture.expected.entry_count_visible {
            assert_eq!(
                case.notebook_variable_explorer.entry_count_visible,
                expected,
                "fixture {name} entry_count_visible mismatch"
            );
        }
        if let Some(expected) = case.fixture.expected.entry_count_total {
            assert_eq!(
                case.notebook_variable_explorer.entry_count_total,
                expected,
                "fixture {name} entry_count_total mismatch"
            );
        }
        if let Some(expected) = case.fixture.expected.has_more_entries {
            assert_eq!(
                case.notebook_variable_explorer.has_more_entries,
                expected,
                "fixture {name} has_more_entries mismatch"
            );
        }
        if let Some(expected) = case.fixture.expected.truncation_notice_visible {
            assert_eq!(
                case.notebook_variable_explorer.truncation_notice_visible,
                expected,
                "fixture {name} truncation_notice_visible mismatch"
            );
        }

        if let Some(expected) = &case.fixture.expected.export_format_class {
            assert_eq!(
                case.variable_explorer_typed_export.export_format_class.as_str(),
                expected.as_str(),
                "fixture {name} export_format_class mismatch"
            );
        }
        if let Some(expected) = &case.fixture.expected.export_posture_class {
            assert_eq!(
                case.variable_explorer_typed_export.export_posture_class.as_str(),
                expected.as_str(),
                "fixture {name} export_posture_class mismatch"
            );
        }
        if let Some(expected) = &case.fixture.expected.export_scope_class {
            assert_eq!(
                case.variable_explorer_typed_export.export_scope_class.as_str(),
                expected.as_str(),
                "fixture {name} export_scope_class mismatch"
            );
        }
        if let Some(expected) = case.fixture.expected.redaction_required {
            assert_eq!(
                case.variable_explorer_typed_export.redaction_required,
                expected,
                "fixture {name} redaction_required mismatch"
            );
        }

        // Surface invariants the spec calls out.
        assert!(
            !case.notebook_variable_explorer.summary.trim().is_empty(),
            "fixture {name}: explorer summary must not be empty"
        );
        assert!(
            !case.variable_explorer_typed_export.summary.trim().is_empty(),
            "fixture {name}: export summary must not be empty"
        );

        observed_sorts.insert(case.notebook_variable_explorer.sort_class.as_str(), ());
        observed_filters.insert(case.notebook_variable_explorer.filter_class.as_str(), ());
        observed_formats.insert(case.variable_explorer_typed_export.export_format_class.as_str(), ());
        observed_postures.insert(case.variable_explorer_typed_export.export_posture_class.as_str(), ());
        observed_scopes.insert(case.variable_explorer_typed_export.export_scope_class.as_str(), ());
    }

    // The manifest's expected vocabulary lists must be exercised by at least
    // one fixture each, so the corpus is not silently shrunk.
    for expected in &manifest.expected_sort_classes {
        assert!(
            observed_sorts.contains_key(expected.as_str()),
            "no fixture exercises sort class '{expected}'"
        );
    }
    for expected in &manifest.expected_filter_classes {
        assert!(
            observed_filters.contains_key(expected.as_str()),
            "no fixture exercises filter class '{expected}'"
        );
    }
    for expected in &manifest.expected_export_format_classes {
        assert!(
            observed_formats.contains_key(expected.as_str()),
            "no fixture exercises export format class '{expected}'"
        );
    }
    for expected in &manifest.expected_export_posture_classes {
        assert!(
            observed_postures.contains_key(expected.as_str()),
            "no fixture exercises export posture class '{expected}'"
        );
    }
    for expected in &manifest.expected_export_scope_classes {
        assert!(
            observed_scopes.contains_key(expected.as_str()),
            "no fixture exercises export scope class '{expected}'"
        );
    }
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = aureline_notebook::current_notebook_variable_explorer_packet()
        .expect("embedded packet must parse");
    let findings = packet.validate();
    assert!(
        findings.is_empty(),
        "embedded packet should validate clean: {findings:?}"
    );
}
