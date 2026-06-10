//! End-to-end coverage for the cell-aware diff, metadata filters, output
//! include or exclude state, and raw JSON fallback corpus.

use std::path::{Path, PathBuf};

use aureline_notebook::{
    current_notebook_diff_packet, NotebookDiffCellChange, NotebookDiffCellChangeClass,
    NotebookDiffMergeResolutionClass, NotebookDiffMetadataFilter, NotebookDiffMode,
    NotebookDiffOutputChangeClass, NotebookDiffOutputSummary,
    NotebookDiffReviewSession, NotebookMetadataFilterState, NotebookOutputIncludeState,
    NotebookRawJsonFallback, RawJsonFallbackReason, NOTEBOOK_DIFF_PACKET_RECORD_KIND,
    NOTEBOOK_DIFF_SCHEMA_VERSION,
};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../")
}

fn fixture_root() -> PathBuf {
    repo_root().join(
        "fixtures/notebook/m5/ship_cell_aware_diff_metadata_filters_output_include_or_exclude_state_and_raw_json_fallback",
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
    diff_modes: Vec<String>,
    cell_change_classes: Vec<String>,
    output_change_classes: Vec<String>,
    metadata_filter_states: Vec<String>,
    output_include_states: Vec<String>,
    raw_json_fallback_reasons: Vec<String>,
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
    notebook_diff_review_session: Option<NotebookDiffReviewSession>,
    #[serde(default)]
    notebook_diff_cell_change: Option<NotebookDiffCellChange>,
    #[serde(default)]
    notebook_diff_output_summary: Option<NotebookDiffOutputSummary>,
    #[serde(default)]
    notebook_diff_metadata_filter: Option<NotebookDiffMetadataFilter>,
    #[serde(default)]
    notebook_raw_json_fallback: Option<NotebookRawJsonFallback>,
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
    diff_mode: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    output_include_state: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    stable_cell_anchors: Option<bool>,
    #[serde(default)]
    #[allow(dead_code)]
    metadata_filter_state: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    unknown_namespaces_preserved_on_save: Option<bool>,
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
    assert_eq!(manifest.schema_version, NOTEBOOK_DIFF_SCHEMA_VERSION);
    assert_eq!(manifest.record_kind, NOTEBOOK_DIFF_PACKET_RECORD_KIND);
}

#[test]
fn manifest_vocabularies_are_complete() {
    let manifest = load_manifest();

    let expected_diff_modes: Vec<String> = NotebookDiffMode::ALL.iter().map(|v| v.as_str().to_string()).collect();
    assert_eq!(manifest.diff_modes, expected_diff_modes);

    let expected_cell_change_classes: Vec<String> =
        NotebookDiffCellChangeClass::ALL.iter().map(|v| v.as_str().to_string()).collect();
    assert_eq!(manifest.cell_change_classes, expected_cell_change_classes);

    let expected_output_change_classes: Vec<String> =
        NotebookDiffOutputChangeClass::ALL.iter().map(|v| v.as_str().to_string()).collect();
    assert_eq!(manifest.output_change_classes, expected_output_change_classes);

    let expected_metadata_filter_states: Vec<String> =
        NotebookMetadataFilterState::ALL.iter().map(|v| v.as_str().to_string()).collect();
    assert_eq!(manifest.metadata_filter_states, expected_metadata_filter_states);

    let expected_output_include_states: Vec<String> =
        NotebookOutputIncludeState::ALL.iter().map(|v| v.as_str().to_string()).collect();
    assert_eq!(manifest.output_include_states, expected_output_include_states);

    let expected_raw_json_fallback_reasons: Vec<String> =
        RawJsonFallbackReason::ALL.iter().map(|v| v.as_str().to_string()).collect();
    assert_eq!(manifest.raw_json_fallback_reasons, expected_raw_json_fallback_reasons);

    let expected_merge_resolution_classes: Vec<String> =
        NotebookDiffMergeResolutionClass::ALL.iter().map(|v| v.as_str().to_string()).collect();
    assert_eq!(manifest.merge_resolution_classes, expected_merge_resolution_classes);
}

#[test]
fn fixture_cell_aware_review_validates() {
    let case = load_case("cell_aware_review");
    if let Some(session) = case.notebook_diff_review_session {
        assert!(session.validate().is_empty(), "session findings: {:?}", session.validate());
        assert_eq!(session.diff_mode, NotebookDiffMode::CellAware);
        assert_eq!(session.output_include_state, NotebookOutputIncludeState::Included);
        assert!(session.stable_cell_anchors);
    }
    if let Some(filter) = case.notebook_diff_metadata_filter {
        assert!(filter.validate().is_empty(), "filter findings: {:?}", filter.validate());
        assert_eq!(filter.metadata_filter_state, NotebookMetadataFilterState::AllVisible);
        assert!(filter.unknown_namespaces_preserved_on_save);
    }
    if let Some(output) = case.notebook_diff_output_summary {
        assert!(output.validate().is_empty(), "output findings: {:?}", output.validate());
        assert_eq!(output.output_include_state, NotebookOutputIncludeState::Included);
    }
    if let Some(change) = case.notebook_diff_cell_change {
        assert!(change.validate().is_empty(), "change findings: {:?}", change.validate());
        assert_eq!(change.cell_change_class, NotebookDiffCellChangeClass::SourceChanged);
    }
}

#[test]
fn fixture_raw_json_fallback_review_validates() {
    let case = load_case("raw_json_fallback_review");
    if let Some(session) = case.notebook_diff_review_session {
        assert!(session.validate().is_empty(), "session findings: {:?}", session.validate());
        assert_eq!(session.diff_mode, NotebookDiffMode::RawJsonFallback);
        assert_eq!(session.output_include_state, NotebookOutputIncludeState::Excluded);
        assert!(!session.stable_cell_anchors);
        assert!(session.raw_json_fallback_ref.is_some());
    }
    if let Some(fallback) = case.notebook_raw_json_fallback {
        assert!(fallback.validate().is_empty(), "fallback findings: {:?}", fallback.validate());
        assert_eq!(fallback.fallback_reason, RawJsonFallbackReason::UnsupportedVersion);
        assert!(!fallback.explicit_user_choice);
    }
}

#[test]
fn fixture_metadata_filter_unknown_hidden_validates() {
    let case = load_case("metadata_filter_unknown_hidden");
    if let Some(filter) = case.notebook_diff_metadata_filter {
        assert!(filter.validate().is_empty(), "filter findings: {:?}", filter.validate());
        assert_eq!(filter.metadata_filter_state, NotebookMetadataFilterState::UnknownHidden);
        assert!(filter.unknown_namespaces_preserved_on_save);
    }
    if let Some(session) = case.notebook_diff_review_session {
        assert!(session.validate().is_empty(), "session findings: {:?}", session.validate());
        assert_eq!(session.diff_mode, NotebookDiffMode::MetadataFocused);
    }
}

#[test]
fn fixture_output_collapsed_review_validates() {
    let case = load_case("output_collapsed_review");
    if let Some(output) = case.notebook_diff_output_summary {
        assert!(output.validate().is_empty(), "output findings: {:?}", output.validate());
        assert_eq!(output.output_include_state, NotebookOutputIncludeState::Collapsed);
        assert_eq!(output.output_change_class, NotebookDiffOutputChangeClass::OutputAdded);
    }
    if let Some(session) = case.notebook_diff_review_session {
        assert!(session.validate().is_empty(), "session findings: {:?}", session.validate());
        assert_eq!(session.diff_mode, NotebookDiffMode::OutputAware);
        assert_eq!(session.output_include_state, NotebookOutputIncludeState::Collapsed);
    }
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = current_notebook_diff_packet().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, NOTEBOOK_DIFF_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, NOTEBOOK_DIFF_PACKET_RECORD_KIND);
    assert!(
        packet.validate().is_empty(),
        "packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn packet_vocabularies_cover_all_variants() {
    let packet = current_notebook_diff_packet().expect("embedded packet must parse");

    let expected_diff_modes: Vec<String> = NotebookDiffMode::ALL.iter().map(|v| v.as_str().to_string()).collect();
    let actual_diff_modes: Vec<String> = packet.diff_modes.iter().map(|v| v.as_str().to_string()).collect();
    assert_eq!(actual_diff_modes, expected_diff_modes);

    let expected_cell_change_classes: Vec<String> =
        NotebookDiffCellChangeClass::ALL.iter().map(|v| v.as_str().to_string()).collect();
    let actual_cell_change_classes: Vec<String> =
        packet.cell_change_classes.iter().map(|v| v.as_str().to_string()).collect();
    assert_eq!(actual_cell_change_classes, expected_cell_change_classes);

    let expected_output_change_classes: Vec<String> =
        NotebookDiffOutputChangeClass::ALL.iter().map(|v| v.as_str().to_string()).collect();
    let actual_output_change_classes: Vec<String> =
        packet.output_change_classes.iter().map(|v| v.as_str().to_string()).collect();
    assert_eq!(actual_output_change_classes, expected_output_change_classes);

    let expected_metadata_filter_states: Vec<String> =
        NotebookMetadataFilterState::ALL.iter().map(|v| v.as_str().to_string()).collect();
    let actual_metadata_filter_states: Vec<String> =
        packet.metadata_filter_states.iter().map(|v| v.as_str().to_string()).collect();
    assert_eq!(actual_metadata_filter_states, expected_metadata_filter_states);

    let expected_output_include_states: Vec<String> =
        NotebookOutputIncludeState::ALL.iter().map(|v| v.as_str().to_string()).collect();
    let actual_output_include_states: Vec<String> =
        packet.output_include_states.iter().map(|v| v.as_str().to_string()).collect();
    assert_eq!(actual_output_include_states, expected_output_include_states);

    let expected_raw_json_fallback_reasons: Vec<String> =
        RawJsonFallbackReason::ALL.iter().map(|v| v.as_str().to_string()).collect();
    let actual_raw_json_fallback_reasons: Vec<String> =
        packet.raw_json_fallback_reasons.iter().map(|v| v.as_str().to_string()).collect();
    assert_eq!(actual_raw_json_fallback_reasons, expected_raw_json_fallback_reasons);

    let expected_merge_resolution_classes: Vec<String> =
        NotebookDiffMergeResolutionClass::ALL.iter().map(|v| v.as_str().to_string()).collect();
    let actual_merge_resolution_classes: Vec<String> =
        packet.merge_resolution_classes.iter().map(|v| v.as_str().to_string()).collect();
    assert_eq!(actual_merge_resolution_classes, expected_merge_resolution_classes);
}
