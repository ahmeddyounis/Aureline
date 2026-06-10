//! End-to-end coverage for the notebook search, outline, breadcrumb, and
//! cell-target navigation corpus.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use aureline_notebook::{
    NotebookBreadcrumb, NotebookCellTarget, NotebookOutlineItem, NotebookSearchQuery,
};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../")
}

fn fixture_root() -> PathBuf {
    repo_root().join("fixtures/notebook/m5/add_notebook_aware_search_outline_breadcrumbs_and_cell_target_navigation")
}

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: u32,
    case_refs: Vec<String>,
    expected_search_scope_classes: Vec<String>,
    expected_search_match_classes: Vec<String>,
    expected_outline_item_classes: Vec<String>,
    expected_breadcrumb_classes: Vec<String>,
    expected_cell_target_classes: Vec<String>,
    expected_scroll_behavior_classes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    #[serde(rename = "__fixture__")]
    fixture: FixtureMeta,
    notebook_search_query: NotebookSearchQuery,
    notebook_outline_item: NotebookOutlineItem,
    notebook_breadcrumb: NotebookBreadcrumb,
    notebook_cell_target: NotebookCellTarget,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    name: String,
    expected: FixtureExpectations,
}

#[derive(Debug, Deserialize)]
struct FixtureExpectations {
    search_scope_class: aureline_notebook::NotebookSearchScopeClass,
    match_class: aureline_notebook::NotebookSearchMatchClass,
    findings: ExpectedFindings,
}

#[derive(Debug, Deserialize, Default)]
struct ExpectedFindings {
    #[serde(default)]
    notebook_search_query: Vec<String>,
    #[serde(default)]
    notebook_outline_item: Vec<String>,
    #[serde(default)]
    notebook_breadcrumb: Vec<String>,
    #[serde(default)]
    notebook_cell_target: Vec<String>,
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
    findings: &[aureline_notebook::SearchOutlineNavigationFinding],
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
    let mut observed_search_scope = BTreeMap::new();
    let mut observed_search_match = BTreeMap::new();
    let mut observed_outline_item = BTreeMap::new();
    let mut observed_breadcrumb = BTreeMap::new();
    let mut observed_cell_target = BTreeMap::new();
    let mut observed_scroll_behavior = BTreeMap::new();

    for case_path in &manifest.case_refs {
        let case = read_case(case_path);
        let name = case.fixture.name.clone();

        // Validators agree with the expected findings list.
        let query_findings = case.notebook_search_query.validate();
        assert_findings_match(
            &case.fixture.expected.findings.notebook_search_query,
            &query_findings,
        );

        let outline_findings = case.notebook_outline_item.validate();
        assert_findings_match(
            &case.fixture.expected.findings.notebook_outline_item,
            &outline_findings,
        );

        let breadcrumb_findings = case.notebook_breadcrumb.validate();
        assert_findings_match(
            &case.fixture.expected.findings.notebook_breadcrumb,
            &breadcrumb_findings,
        );

        let target_findings = case.notebook_cell_target.validate();
        assert_findings_match(
            &case.fixture.expected.findings.notebook_cell_target,
            &target_findings,
        );

        // Closed-vocabulary expectations are reflected in the records.
        assert_eq!(
            case.notebook_search_query.search_scope_class, case.fixture.expected.search_scope_class,
            "fixture {name} search_scope_class mismatch"
        );
        assert_eq!(
            case.notebook_search_query.match_class, case.fixture.expected.match_class,
            "fixture {name} match_class mismatch"
        );

        // Surface invariants the spec calls out.
        assert!(
            !case.notebook_search_query.query_label.trim().is_empty(),
            "fixture {name}: query_label must be non-empty"
        );
        assert!(
            !case.notebook_outline_item.cell_id_ref.trim().is_empty(),
            "fixture {name}: outline item cell_id_ref must be non-empty"
        );
        assert!(
            !case.notebook_breadcrumb.label.trim().is_empty(),
            "fixture {name}: breadcrumb label must be non-empty"
        );
        assert!(
            !case.notebook_breadcrumb.target_ref.trim().is_empty(),
            "fixture {name}: breadcrumb target_ref must be non-empty"
        );

        // At least one locator on cell target.
        let t = &case.notebook_cell_target;
        let has_locator = t
            .cell_id_ref
            .as_ref()
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false)
            || t.cell_index.is_some()
            || t.output_index.is_some()
            || t.heading_anchor_ref
                .as_ref()
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false)
            || t.search_match_ref
                .as_ref()
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false);
        assert!(
            has_locator,
            "fixture {name}: cell target must have at least one locator"
        );

        observed_search_scope.insert(case.notebook_search_query.search_scope_class.as_str(), ());
        observed_search_match.insert(case.notebook_search_query.match_class.as_str(), ());
        observed_outline_item.insert(case.notebook_outline_item.item_class.as_str(), ());
        observed_breadcrumb.insert(case.notebook_breadcrumb.breadcrumb_class.as_str(), ());
        observed_cell_target.insert(case.notebook_cell_target.target_class.as_str(), ());
        observed_scroll_behavior
            .insert(case.notebook_cell_target.scroll_behavior_class.as_str(), ());
    }

    // The manifest's expected vocabulary lists must be exercised by at least
    // one fixture each, so the corpus is not silently shrunk.
    for expected in &manifest.expected_search_scope_classes {
        assert!(
            observed_search_scope.contains_key(expected.as_str()),
            "no fixture exercises search scope class '{expected}'"
        );
    }
    for expected in &manifest.expected_search_match_classes {
        assert!(
            observed_search_match.contains_key(expected.as_str()),
            "no fixture exercises search match class '{expected}'"
        );
    }
    for expected in &manifest.expected_outline_item_classes {
        assert!(
            observed_outline_item.contains_key(expected.as_str()),
            "no fixture exercises outline item class '{expected}'"
        );
    }
    for expected in &manifest.expected_breadcrumb_classes {
        assert!(
            observed_breadcrumb.contains_key(expected.as_str()),
            "no fixture exercises breadcrumb class '{expected}'"
        );
    }
    for expected in &manifest.expected_cell_target_classes {
        assert!(
            observed_cell_target.contains_key(expected.as_str()),
            "no fixture exercises cell target class '{expected}'"
        );
    }
    for expected in &manifest.expected_scroll_behavior_classes {
        assert!(
            observed_scroll_behavior.contains_key(expected.as_str()),
            "no fixture exercises scroll behavior class '{expected}'"
        );
    }
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = aureline_notebook::current_notebook_search_outline_navigation_packet()
        .expect("embedded packet must parse");
    let findings = packet.validate();
    assert!(
        findings.is_empty(),
        "embedded packet should validate clean: {findings:?}"
    );
}
