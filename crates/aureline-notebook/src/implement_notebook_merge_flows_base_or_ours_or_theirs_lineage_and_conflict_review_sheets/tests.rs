use super::*;

fn sample_merge_flow() -> NotebookMergeFlow {
    NotebookMergeFlow {
        record_kind: NOTEBOOK_MERGE_FLOW_RECORD_KIND.to_owned(),
        notebook_merge_schema_version: NOTEBOOK_MERGE_SCHEMA_VERSION,
        merge_flow_id: "nb.merge.flow.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        merge_kind: NotebookMergeKind::ThreeWayMerge,
        base_ref: "git.ref.base".to_owned(),
        ours_ref: "git.ref.ours".to_owned(),
        theirs_ref: "git.ref.theirs".to_owned(),
        result_ref: None,
        resolution_strategy: NotebookMergeResolutionStrategy::CellAware,
        unresolved_count: 2,
        rollback_checkpoint_ref: "nb.checkpoints.merge.01".to_owned(),
        lineage_refs: vec![
            "nb.merge.lineage.01".to_owned(),
            "nb.merge.lineage.02".to_owned(),
        ],
        summary: "Three-way cell-aware merge with two unresolved conflicts.".to_owned(),
    }
}

fn sample_merge_lineage() -> NotebookMergeLineage {
    NotebookMergeLineage {
        record_kind: NOTEBOOK_MERGE_LINEAGE_RECORD_KIND.to_owned(),
        notebook_merge_schema_version: NOTEBOOK_MERGE_SCHEMA_VERSION,
        lineage_id: "nb.merge.lineage.01".to_owned(),
        merge_flow_ref: "nb.merge.flow.01".to_owned(),
        cell_id_ref: "nb.cell.intro".to_owned(),
        base_cell_ref: Some("nb.cell.intro.base".to_owned()),
        ours_cell_ref: Some("nb.cell.intro.ours".to_owned()),
        theirs_cell_ref: Some("nb.cell.intro.theirs".to_owned()),
        result_cell_ref: Some("nb.cell.intro.result".to_owned()),
        resolution_class: NotebookDiffMergeResolutionClass::Result,
        metadata_field_lineage_refs: vec![],
        summary: "Intro cell resolved as edited result.".to_owned(),
    }
}

fn sample_conflict_sheet() -> NotebookConflictReviewSheet {
    NotebookConflictReviewSheet {
        record_kind: NOTEBOOK_CONFLICT_REVIEW_SHEET_RECORD_KIND.to_owned(),
        notebook_merge_schema_version: NOTEBOOK_MERGE_SCHEMA_VERSION,
        sheet_id: "nb.merge.sheet.01".to_owned(),
        merge_flow_ref: "nb.merge.flow.01".to_owned(),
        conflict_cell_ref: "nb.cell.compute".to_owned(),
        conflict_class: NotebookConflictClass::SourceConflict,
        base_preview_ref: Some("preview.compute.base".to_owned()),
        ours_preview_ref: Some("preview.compute.ours".to_owned()),
        theirs_preview_ref: Some("preview.compute.theirs".to_owned()),
        suggested_resolution: NotebookDiffMergeResolutionClass::Unresolved,
        available_actions: vec![
            NotebookConflictReviewSheetAction::AcceptOurs,
            NotebookConflictReviewSheetAction::AcceptTheirs,
            NotebookConflictReviewSheetAction::AcceptBase,
            NotebookConflictReviewSheetAction::EditResult,
            NotebookConflictReviewSheetAction::RawMerge,
        ],
        rollback_path_ref: "nb.rollback.merge.01".to_owned(),
        redaction_profile_ref: Some("redaction.default".to_owned()),
        summary: "Compute cell has a source conflict; review base/ours/theirs.".to_owned(),
    }
}

#[test]
fn merge_flow_validates_clean() {
    let f = sample_merge_flow();
    assert!(
        f.validate().is_empty(),
        "merge flow should be clean: {:?}",
        f.validate()
    );
}

#[test]
fn merge_lineage_validates_clean() {
    let l = sample_merge_lineage();
    assert!(
        l.validate().is_empty(),
        "merge lineage should be clean: {:?}",
        l.validate()
    );
}

#[test]
fn conflict_sheet_validates_clean() {
    let s = sample_conflict_sheet();
    assert!(
        s.validate().is_empty(),
        "conflict sheet should be clean: {:?}",
        s.validate()
    );
}

#[test]
fn merge_flow_rejects_empty_document_id_ref() {
    let mut f = sample_merge_flow();
    f.document_id_ref = "".to_owned();
    let findings = f.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_merge_flow.document_id_ref_required"));
}

#[test]
fn merge_flow_rejects_result_ref_with_unresolved_conflicts() {
    let mut f = sample_merge_flow();
    f.result_ref = Some("git.ref.result".to_owned());
    let findings = f.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_merge_flow.result_ref_with_unresolved_conflicts"));
}

#[test]
fn merge_flow_allows_result_ref_when_resolved() {
    let mut f = sample_merge_flow();
    f.unresolved_count = 0;
    f.result_ref = Some("git.ref.result".to_owned());
    assert!(
        f.validate().is_empty(),
        "resolved merge flow should be clean: {:?}",
        f.validate()
    );
}

#[test]
fn merge_lineage_rejects_empty_cell_id_ref() {
    let mut l = sample_merge_lineage();
    l.cell_id_ref = "".to_owned();
    let findings = l.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_merge_lineage.cell_id_ref_required"));
}

#[test]
fn conflict_sheet_rejects_empty_actions() {
    let mut s = sample_conflict_sheet();
    s.available_actions = vec![];
    let findings = s.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_conflict_review_sheet.available_actions_required"));
}

#[test]
fn conflict_sheet_rejects_empty_rollback_path_ref() {
    let mut s = sample_conflict_sheet();
    s.rollback_path_ref = "".to_owned();
    let findings = s.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_conflict_review_sheet.rollback_path_ref_required"));
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(NotebookMergeKind::ThreeWayMerge.as_str(), "three_way_merge");
    assert_eq!(NotebookMergeKind::CherryPick.as_str(), "cherry_pick");
    assert_eq!(
        NotebookMergeResolutionStrategy::MetadataAware.as_str(),
        "metadata_aware"
    );
    assert_eq!(
        NotebookConflictClass::CellAddedBoth.as_str(),
        "cell_added_both"
    );
    assert_eq!(
        NotebookConflictReviewSheetAction::RawMerge.as_str(),
        "raw_merge"
    );
    assert_eq!(NotebookDiffMergeResolutionClass::Theirs.as_str(), "theirs");
}

#[test]
fn packet_validates_clean() {
    let packet = NotebookMergePacket {
        schema_version: NOTEBOOK_MERGE_SCHEMA_VERSION,
        record_kind: NOTEBOOK_MERGE_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.merge.packet.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        merge_kinds: NotebookMergeKind::ALL.to_vec(),
        resolution_strategies: NotebookMergeResolutionStrategy::ALL.to_vec(),
        conflict_classes: NotebookConflictClass::ALL.to_vec(),
        sheet_actions: NotebookConflictReviewSheetAction::ALL.to_vec(),
        merge_resolution_classes: NotebookDiffMergeResolutionClass::ALL.to_vec(),
        example_merge_flows: vec![sample_merge_flow()],
        example_lineages: vec![sample_merge_lineage()],
        example_conflict_sheets: vec![sample_conflict_sheet()],
        summary: "Merge/lineage/conflict-review packet v1.".to_owned(),
    };
    assert!(
        packet.validate().is_empty(),
        "packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn embedded_packet_parses() {
    let packet = current_notebook_merge_packet().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, NOTEBOOK_MERGE_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, NOTEBOOK_MERGE_PACKET_RECORD_KIND);
}
