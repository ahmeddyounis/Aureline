//! End-to-end coverage for notebook merge flows, lineage, and conflict-review
//! sheets.

use std::path::{Path, PathBuf};

use aureline_notebook::{
    current_notebook_merge_packet, NotebookConflictClass, NotebookConflictReviewSheet,
    NotebookConflictReviewSheetAction, NotebookDiffMergeResolutionClass, NotebookMergeFlow,
    NotebookMergeKind, NotebookMergeLineage, NotebookMergePacket, NotebookMergeResolutionStrategy,
    NOTEBOOK_MERGE_PACKET_JSON, NOTEBOOK_MERGE_PACKET_RECORD_KIND, NOTEBOOK_MERGE_SCHEMA_VERSION,
};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../")
}

fn fixture_root() -> PathBuf {
    repo_root().join(
        "fixtures/notebook/m5/implement_notebook_merge_flows_base_or_ours_or_theirs_lineage_and_conflict_review_sheets",
    )
}

#[derive(Debug, Deserialize)]
struct Manifest {
    schema_version: u32,
    record_kind: String,
    #[allow(dead_code)]
    packet_id: String,
    #[allow(dead_code)]
    as_of: String,
    merge_kinds: Vec<String>,
    resolution_strategies: Vec<String>,
    conflict_classes: Vec<String>,
    sheet_actions: Vec<String>,
    merge_resolution_classes: Vec<String>,
    #[allow(dead_code)]
    case_refs: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    #[serde(rename = "__fixture__")]
    #[allow(dead_code)]
    fixture: FixtureMeta,
    #[serde(default)]
    notebook_merge_flow: Option<NotebookMergeFlow>,
    #[serde(default)]
    notebook_merge_lineage: Option<NotebookMergeLineage>,
    #[serde(default)]
    notebook_conflict_review_sheet: Option<NotebookConflictReviewSheet>,
}

#[derive(Debug, Deserialize)]
struct FixtureMeta {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    expected: FixtureExpectations,
}

#[derive(Debug, Deserialize)]
struct FixtureExpectations {
    #[serde(default)]
    #[allow(dead_code)]
    merge_kind: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    resolution_strategy: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    unresolved_count: Option<u32>,
    #[serde(default)]
    #[allow(dead_code)]
    conflict_class: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    suggested_resolution: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    available_actions_count: Option<usize>,
    #[allow(dead_code)]
    findings: String,
}

fn load_manifest() -> Manifest {
    let path = fixture_root().join("manifest.yaml");
    let text = std::fs::read_to_string(&path).expect("manifest must exist");
    serde_yaml::from_str(&text).expect("manifest must parse")
}

fn load_case(name: &str) -> FixtureCase {
    let path = fixture_root().join(format!("{}.yaml", name));
    let text = std::fs::read_to_string(&path).expect("case file must exist");
    serde_yaml::from_str(&text).expect("case file must parse")
}

#[test]
fn manifest_matches_schema_version() {
    let manifest = load_manifest();
    assert_eq!(manifest.schema_version, NOTEBOOK_MERGE_SCHEMA_VERSION);
    assert_eq!(manifest.record_kind, NOTEBOOK_MERGE_PACKET_RECORD_KIND);
}

#[test]
fn manifest_vocabularies_are_complete() {
    let manifest = load_manifest();

    let expected_merge_kinds: Vec<String> = NotebookMergeKind::ALL
        .iter()
        .map(|v| v.as_str().to_string())
        .collect();
    assert_eq!(manifest.merge_kinds, expected_merge_kinds);

    let expected_resolution_strategies: Vec<String> = NotebookMergeResolutionStrategy::ALL
        .iter()
        .map(|v| v.as_str().to_string())
        .collect();
    assert_eq!(
        manifest.resolution_strategies,
        expected_resolution_strategies
    );

    let expected_conflict_classes: Vec<String> = NotebookConflictClass::ALL
        .iter()
        .map(|v| v.as_str().to_string())
        .collect();
    assert_eq!(manifest.conflict_classes, expected_conflict_classes);

    let expected_sheet_actions: Vec<String> = NotebookConflictReviewSheetAction::ALL
        .iter()
        .map(|v| v.as_str().to_string())
        .collect();
    assert_eq!(manifest.sheet_actions, expected_sheet_actions);

    let expected_merge_resolution_classes: Vec<String> = NotebookDiffMergeResolutionClass::ALL
        .iter()
        .map(|v| v.as_str().to_string())
        .collect();
    assert_eq!(
        manifest.merge_resolution_classes,
        expected_merge_resolution_classes
    );
}

#[test]
fn manifest_lists_all_case_files() {
    let manifest = load_manifest();
    for case_ref in &manifest.case_refs {
        let path = fixture_root().join(format!("{}.yaml", case_ref));
        assert!(path.exists(), "case file must exist: {}", path.display());
    }
}

#[test]
fn fixture_three_way_merge_unresolved_validates() {
    let case = load_case("three_way_merge_unresolved");
    if let Some(flow) = case.notebook_merge_flow {
        assert!(
            flow.validate().is_empty(),
            "flow findings: {:?}",
            flow.validate()
        );
        assert_eq!(flow.merge_kind, NotebookMergeKind::ThreeWayMerge);
        assert_eq!(
            flow.resolution_strategy,
            NotebookMergeResolutionStrategy::CellAware
        );
        assert_eq!(flow.unresolved_count, 2);
    }
    if let Some(lineage) = case.notebook_merge_lineage {
        assert!(
            lineage.validate().is_empty(),
            "lineage findings: {:?}",
            lineage.validate()
        );
        assert_eq!(
            lineage.resolution_class,
            NotebookDiffMergeResolutionClass::Result
        );
    }
    if let Some(sheet) = case.notebook_conflict_review_sheet {
        assert!(
            sheet.validate().is_empty(),
            "sheet findings: {:?}",
            sheet.validate()
        );
        assert_eq!(sheet.conflict_class, NotebookConflictClass::SourceConflict);
        assert_eq!(
            sheet.suggested_resolution,
            NotebookDiffMergeResolutionClass::Unresolved
        );
        assert!(!sheet.available_actions.is_empty());
    }
}

#[test]
fn fixture_fast_forward_resolved_validates() {
    let case = load_case("fast_forward_resolved");
    if let Some(flow) = case.notebook_merge_flow {
        assert!(
            flow.validate().is_empty(),
            "flow findings: {:?}",
            flow.validate()
        );
        assert_eq!(flow.merge_kind, NotebookMergeKind::FastForward);
        assert_eq!(flow.unresolved_count, 0);
        assert!(flow.result_ref.is_some());
    }
}

#[test]
fn fixture_conflict_review_source_validates() {
    let case = load_case("conflict_review_source");
    if let Some(sheet) = case.notebook_conflict_review_sheet {
        assert!(
            sheet.validate().is_empty(),
            "sheet findings: {:?}",
            sheet.validate()
        );
        assert_eq!(sheet.conflict_class, NotebookConflictClass::SourceConflict);
        assert_eq!(
            sheet.suggested_resolution,
            NotebookDiffMergeResolutionClass::Unresolved
        );
        assert_eq!(sheet.available_actions.len(), 6);
    }
}

#[test]
fn fixture_conflict_review_output_validates() {
    let case = load_case("conflict_review_output");
    if let Some(sheet) = case.notebook_conflict_review_sheet {
        assert!(
            sheet.validate().is_empty(),
            "sheet findings: {:?}",
            sheet.validate()
        );
        assert_eq!(sheet.conflict_class, NotebookConflictClass::OutputConflict);
        assert_eq!(
            sheet.suggested_resolution,
            NotebookDiffMergeResolutionClass::Ours
        );
        assert_eq!(sheet.available_actions.len(), 4);
    }
}

#[test]
fn embedded_merge_packet_is_valid_json() {
    let parsed: NotebookMergePacket =
        serde_json::from_str(NOTEBOOK_MERGE_PACKET_JSON).expect("packet JSON must parse");
    assert_eq!(parsed.schema_version, NOTEBOOK_MERGE_SCHEMA_VERSION);
    assert_eq!(parsed.record_kind, NOTEBOOK_MERGE_PACKET_RECORD_KIND);
}

#[test]
fn current_merge_packet_validates_clean() {
    let packet = current_notebook_merge_packet().expect("current packet must parse");
    let findings = packet.validate();
    assert!(
        findings.is_empty(),
        "current packet should validate cleanly: {:?}",
        findings
    );
}

#[test]
fn merge_packet_covers_all_closed_vocabularies() {
    let packet = current_notebook_merge_packet().expect("current packet must parse");
    assert_eq!(packet.merge_kinds.len(), NotebookMergeKind::ALL.len());
    assert_eq!(
        packet.resolution_strategies.len(),
        NotebookMergeResolutionStrategy::ALL.len()
    );
    assert_eq!(
        packet.conflict_classes.len(),
        NotebookConflictClass::ALL.len()
    );
    assert_eq!(
        packet.sheet_actions.len(),
        NotebookConflictReviewSheetAction::ALL.len()
    );
}

#[test]
fn merge_flow_round_trips_through_json() {
    let flow = NotebookMergeFlow {
        record_kind: "notebook_merge_flow".to_owned(),
        notebook_merge_schema_version: NOTEBOOK_MERGE_SCHEMA_VERSION,
        merge_flow_id: "nb.merge.flow.rt".to_owned(),
        document_id_ref: "nb.doc.rt".to_owned(),
        merge_kind: NotebookMergeKind::ThreeWayMerge,
        base_ref: "git.base".to_owned(),
        ours_ref: "git.ours".to_owned(),
        theirs_ref: "git.theirs".to_owned(),
        result_ref: None,
        resolution_strategy: NotebookMergeResolutionStrategy::CellAware,
        unresolved_count: 1,
        rollback_checkpoint_ref: "cp.1".to_owned(),
        lineage_refs: vec!["lineage.1".to_owned()],
        summary: "Round-trip merge flow.".to_owned(),
    };
    let json = serde_json::to_string(&flow).expect("must serialize");
    let back: NotebookMergeFlow = serde_json::from_str(&json).expect("must deserialize");
    assert_eq!(back, flow);
}

#[test]
fn conflict_sheet_requires_non_empty_actions() {
    let sheet = NotebookConflictReviewSheet {
        record_kind: "notebook_conflict_review_sheet".to_owned(),
        notebook_merge_schema_version: NOTEBOOK_MERGE_SCHEMA_VERSION,
        sheet_id: "nb.merge.sheet.empty".to_owned(),
        merge_flow_ref: "nb.merge.flow.1".to_owned(),
        conflict_cell_ref: "nb.cell.1".to_owned(),
        conflict_class: NotebookConflictClass::SourceConflict,
        base_preview_ref: None,
        ours_preview_ref: None,
        theirs_preview_ref: None,
        suggested_resolution: NotebookDiffMergeResolutionClass::Unresolved,
        available_actions: vec![],
        rollback_path_ref: "rollback.1".to_owned(),
        redaction_profile_ref: None,
        summary: "Empty actions.".to_owned(),
    };
    let findings = sheet.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_conflict_review_sheet.available_actions_required"));
}
