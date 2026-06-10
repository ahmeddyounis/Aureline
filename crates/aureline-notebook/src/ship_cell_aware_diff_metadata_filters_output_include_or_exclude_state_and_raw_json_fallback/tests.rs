use super::*;

fn sample_output_summary() -> NotebookDiffOutputSummary {
    NotebookDiffOutputSummary {
        record_kind: NOTEBOOK_DIFF_OUTPUT_SUMMARY_RECORD_KIND.to_owned(),
        notebook_diff_schema_version: NOTEBOOK_DIFF_SCHEMA_VERSION,
        output_summary_id: "nb.diff.output.01".to_owned(),
        owner_cell_ref: "nb.cell.intro".to_owned(),
        output_change_class: NotebookDiffOutputChangeClass::OutputAdded,
        output_include_state: NotebookOutputIncludeState::Included,
        truncated_in_review: false,
        output_trust_state_ref: "trust.output.01".to_owned(),
        summary: "Output added to intro cell.".to_owned(),
    }
}

fn sample_metadata_filter() -> NotebookDiffMetadataFilter {
    NotebookDiffMetadataFilter {
        record_kind: NOTEBOOK_DIFF_METADATA_FILTER_RECORD_KIND.to_owned(),
        notebook_diff_schema_version: NOTEBOOK_DIFF_SCHEMA_VERSION,
        metadata_filter_id: "nb.diff.filter.01".to_owned(),
        metadata_filter_state: NotebookMetadataFilterState::AllVisible,
        visible_namespace_refs: vec!["kernelspec".to_owned(), "aureline".to_owned()],
        hidden_namespace_refs: vec![],
        unknown_namespaces_preserved_on_save: true,
        summary: "All metadata namespaces visible in review.".to_owned(),
    }
}

fn sample_cell_change() -> NotebookDiffCellChange {
    NotebookDiffCellChange {
        record_kind: NOTEBOOK_DIFF_CELL_CHANGE_RECORD_KIND.to_owned(),
        notebook_diff_schema_version: NOTEBOOK_DIFF_SCHEMA_VERSION,
        cell_change_id: "nb.diff.change.01".to_owned(),
        cell_id_ref: "nb.cell.intro".to_owned(),
        cell_change_class: NotebookDiffCellChangeClass::SourceChanged,
        source_edit_summary_ref: Some("nb.diff.source.01".to_owned()),
        output_summary_refs: vec!["nb.diff.output.01".to_owned()],
        metadata_filter_ref: "nb.diff.filter.01".to_owned(),
        merge_resolution_class: None,
        collapsed_in_diff: false,
        summary: "Intro cell source changed.".to_owned(),
    }
}

fn sample_raw_json_fallback() -> NotebookRawJsonFallback {
    NotebookRawJsonFallback {
        record_kind: NOTEBOOK_RAW_JSON_FALLBACK_RECORD_KIND.to_owned(),
        notebook_diff_schema_version: NOTEBOOK_DIFF_SCHEMA_VERSION,
        fallback_id: "nb.diff.fallback.01".to_owned(),
        fallback_reason: RawJsonFallbackReason::UnsupportedVersion,
        fallback_explanation: "Notebook format version 3 is not supported for cell-aware diff."
            .to_owned(),
        explicit_user_choice: false,
        canonical_document_ref: "nb.doc.legacy".to_owned(),
        summary: "Raw JSON fallback due to unsupported version.".to_owned(),
    }
}

fn sample_review_session() -> NotebookDiffReviewSession {
    NotebookDiffReviewSession {
        record_kind: NOTEBOOK_DIFF_REVIEW_SESSION_RECORD_KIND.to_owned(),
        notebook_diff_schema_version: NOTEBOOK_DIFF_SCHEMA_VERSION,
        session_id: "nb.diff.session.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        diff_mode: NotebookDiffMode::CellAware,
        metadata_filter_ref: "nb.diff.filter.01".to_owned(),
        output_include_state: NotebookOutputIncludeState::Included,
        cell_change_refs: vec!["nb.diff.change.01".to_owned()],
        raw_json_fallback_ref: None,
        stable_cell_anchors: true,
        summary: "Cell-aware review session for example notebook.".to_owned(),
    }
}

#[test]
fn output_summary_validates_clean() {
    let s = sample_output_summary();
    assert!(
        s.validate().is_empty(),
        "output summary should be clean: {:?}",
        s.validate()
    );
}

#[test]
fn metadata_filter_validates_clean() {
    let f = sample_metadata_filter();
    assert!(
        f.validate().is_empty(),
        "metadata filter should be clean: {:?}",
        f.validate()
    );
}

#[test]
fn cell_change_validates_clean() {
    let c = sample_cell_change();
    assert!(
        c.validate().is_empty(),
        "cell change should be clean: {:?}",
        c.validate()
    );
}

#[test]
fn raw_json_fallback_validates_clean() {
    let r = sample_raw_json_fallback();
    assert!(
        r.validate().is_empty(),
        "raw JSON fallback should be clean: {:?}",
        r.validate()
    );
}

#[test]
fn review_session_validates_clean() {
    let s = sample_review_session();
    assert!(
        s.validate().is_empty(),
        "review session should be clean: {:?}",
        s.validate()
    );
}

#[test]
fn review_session_requires_fallback_ref_when_raw_json() {
    let mut s = sample_review_session();
    s.diff_mode = NotebookDiffMode::RawJsonFallback;
    let findings = s.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_diff_review_session.raw_json_fallback_ref_required"));
}

#[test]
fn review_session_rejects_fallback_ref_when_not_raw_json() {
    let mut s = sample_review_session();
    s.raw_json_fallback_ref = Some("nb.diff.fallback.01".to_owned());
    let findings = s.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_diff_review_session.raw_json_fallback_ref_unexpected"));
}

#[test]
fn output_summary_rejects_empty_summary() {
    let mut s = sample_output_summary();
    s.summary = "".to_owned();
    let findings = s.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_diff_output_summary.summary_required"));
}

#[test]
fn metadata_filter_rejects_empty_summary() {
    let mut f = sample_metadata_filter();
    f.summary = "".to_owned();
    let findings = f.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_diff_metadata_filter.summary_required"));
}

#[test]
fn cell_change_rejects_empty_cell_id_ref() {
    let mut c = sample_cell_change();
    c.cell_id_ref = "".to_owned();
    let findings = c.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_diff_cell_change.cell_id_ref_required"));
}

#[test]
fn raw_json_fallback_rejects_empty_explanation() {
    let mut r = sample_raw_json_fallback();
    r.fallback_explanation = "".to_owned();
    let findings = r.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_raw_json_fallback.fallback_explanation_required"));
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(NotebookDiffMode::CellAware.as_str(), "cell_aware");
    assert_eq!(
        NotebookDiffCellChangeClass::CellAdded.as_str(),
        "cell_added"
    );
    assert_eq!(
        NotebookDiffOutputChangeClass::OutputRemoved.as_str(),
        "output_removed"
    );
    assert_eq!(
        NotebookMetadataFilterState::UnknownHidden.as_str(),
        "unknown_hidden"
    );
    assert_eq!(NotebookOutputIncludeState::Collapsed.as_str(), "collapsed");
    assert_eq!(
        RawJsonFallbackReason::CorruptStructure.as_str(),
        "corrupt_structure"
    );
    assert_eq!(
        NotebookDiffMergeResolutionClass::Unresolved.as_str(),
        "unresolved"
    );
}

#[test]
fn packet_validates_clean() {
    let packet = NotebookDiffPacket {
        schema_version: NOTEBOOK_DIFF_SCHEMA_VERSION,
        record_kind: NOTEBOOK_DIFF_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.diff.packet.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        diff_modes: NotebookDiffMode::ALL.to_vec(),
        cell_change_classes: NotebookDiffCellChangeClass::ALL.to_vec(),
        output_change_classes: NotebookDiffOutputChangeClass::ALL.to_vec(),
        metadata_filter_states: NotebookMetadataFilterState::ALL.to_vec(),
        output_include_states: NotebookOutputIncludeState::ALL.to_vec(),
        raw_json_fallback_reasons: RawJsonFallbackReason::ALL.to_vec(),
        merge_resolution_classes: NotebookDiffMergeResolutionClass::ALL.to_vec(),
        example_review_sessions: vec![sample_review_session()],
        example_cell_changes: vec![sample_cell_change()],
        example_output_summaries: vec![sample_output_summary()],
        example_metadata_filters: vec![sample_metadata_filter()],
        example_raw_json_fallbacks: vec![sample_raw_json_fallback()],
        summary: "Diff/review packet v1.".to_owned(),
    };
    assert!(
        packet.validate().is_empty(),
        "packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn embedded_packet_parses() {
    let packet = current_notebook_diff_packet().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, NOTEBOOK_DIFF_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, NOTEBOOK_DIFF_PACKET_RECORD_KIND);
}
