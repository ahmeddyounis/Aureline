use super::*;

fn sample_comment() -> NotebookComment {
    NotebookComment {
        record_kind: NOTEBOOK_COMMENT_RECORD_KIND.to_owned(),
        notebook_comment_anchor_schema_version: NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION,
        comment_id: "nb.comment.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        anchor_ref: "nb.anchor.cell.intro".to_owned(),
        comment_target_class: NotebookCommentTargetClass::Cell,
        comment_status: NotebookCommentStatusClass::Active,
        thread_state: NotebookCommentThreadState::Single,
        author_ref: "actor.alice".to_owned(),
        reply_refs: vec![],
        redaction_profile_ref: None,
        summary: "Active comment on the intro cell.".to_owned(),
    }
}

fn sample_cell_anchor() -> NotebookAnchor {
    NotebookAnchor {
        record_kind: NOTEBOOK_ANCHOR_RECORD_KIND.to_owned(),
        notebook_comment_anchor_schema_version: NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION,
        anchor_id: "nb.anchor.cell.intro".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        anchor_kind: NotebookAnchorKind::Cell,
        cell_id_ref: "nb.cell.intro".to_owned(),
        output_handle_ref: None,
        summary: "Stable anchor to the intro cell.".to_owned(),
    }
}

fn sample_output_anchor() -> NotebookAnchor {
    NotebookAnchor {
        record_kind: NOTEBOOK_ANCHOR_RECORD_KIND.to_owned(),
        notebook_comment_anchor_schema_version: NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION,
        anchor_id: "nb.anchor.output.plot".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        anchor_kind: NotebookAnchorKind::Output,
        cell_id_ref: "nb.cell.plot".to_owned(),
        output_handle_ref: Some("output.plot.01".to_owned()),
        summary: "Stable anchor to the plot output.".to_owned(),
    }
}

fn sample_review_workspace_parity_full() -> NotebookReviewWorkspaceParity {
    NotebookReviewWorkspaceParity {
        record_kind: NOTEBOOK_REVIEW_WORKSPACE_PARITY_RECORD_KIND.to_owned(),
        notebook_comment_anchor_schema_version: NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION,
        parity_id: "nb.parity.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        parity_class: NotebookReviewWorkspaceParityClass::Full,
        downgrade_reasons: vec![],
        anchor_refs: vec!["nb.anchor.cell.intro".to_owned()],
        comment_refs: vec!["nb.comment.01".to_owned()],
        summary: "Full review-workspace parity with stable cell anchors.".to_owned(),
    }
}

fn sample_review_workspace_parity_degraded() -> NotebookReviewWorkspaceParity {
    NotebookReviewWorkspaceParity {
        record_kind: NOTEBOOK_REVIEW_WORKSPACE_PARITY_RECORD_KIND.to_owned(),
        notebook_comment_anchor_schema_version: NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION,
        parity_id: "nb.parity.02".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        parity_class: NotebookReviewWorkspaceParityClass::Degraded,
        downgrade_reasons: vec![
            NotebookReviewWorkspaceDowngradeReason::MissingStableIds,
            NotebookReviewWorkspaceDowngradeReason::RuntimeBound,
        ],
        anchor_refs: vec![],
        comment_refs: vec![],
        summary:
            "Degraded parity because stable cell IDs are missing and the notebook is runtime-bound."
                .to_owned(),
    }
}

#[test]
fn comment_validates_clean() {
    let c = sample_comment();
    assert!(
        c.validate().is_empty(),
        "comment should be clean: {:?}",
        c.validate()
    );
}

#[test]
fn cell_anchor_validates_clean() {
    let a = sample_cell_anchor();
    assert!(
        a.validate().is_empty(),
        "cell anchor should be clean: {:?}",
        a.validate()
    );
}

#[test]
fn output_anchor_validates_clean() {
    let a = sample_output_anchor();
    assert!(
        a.validate().is_empty(),
        "output anchor should be clean: {:?}",
        a.validate()
    );
}

#[test]
fn review_workspace_parity_full_validates_clean() {
    let p = sample_review_workspace_parity_full();
    assert!(
        p.validate().is_empty(),
        "full parity should be clean: {:?}",
        p.validate()
    );
}

#[test]
fn review_workspace_parity_degraded_validates_clean() {
    let p = sample_review_workspace_parity_degraded();
    assert!(
        p.validate().is_empty(),
        "degraded parity should be clean: {:?}",
        p.validate()
    );
}

#[test]
fn comment_rejects_empty_document_id_ref() {
    let mut c = sample_comment();
    c.document_id_ref = "".to_owned();
    let findings = c.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_comment.document_id_ref_required"));
}

#[test]
fn comment_rejects_empty_anchor_ref() {
    let mut c = sample_comment();
    c.anchor_ref = "".to_owned();
    let findings = c.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_comment.anchor_ref_required"));
}

#[test]
fn anchor_rejects_empty_cell_id_ref() {
    let mut a = sample_cell_anchor();
    a.cell_id_ref = "".to_owned();
    let findings = a.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_anchor.cell_id_ref_required"));
}

#[test]
fn output_anchor_rejects_missing_output_handle_ref() {
    let mut a = sample_output_anchor();
    a.output_handle_ref = None;
    let findings = a.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_anchor.output_handle_ref_required_for_output"));
}

#[test]
fn parity_rejects_empty_downgrade_reasons_when_not_full() {
    let mut p = sample_review_workspace_parity_degraded();
    p.downgrade_reasons = vec![];
    let findings = p.validate();
    assert!(findings.iter().any(|f| f.check_id
        == "notebook_review_workspace_parity.downgrade_reasons_required_when_not_full"));
}

#[test]
fn parity_rejects_non_empty_downgrade_reasons_when_full() {
    let mut p = sample_review_workspace_parity_full();
    p.downgrade_reasons = vec![NotebookReviewWorkspaceDowngradeReason::Redacted];
    let findings = p.validate();
    assert!(findings.iter().any(|f| f.check_id
        == "notebook_review_workspace_parity.downgrade_reasons_must_be_empty_when_full"));
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(NotebookCommentTargetClass::Cell.as_str(), "cell");
    assert_eq!(NotebookCommentTargetClass::Output.as_str(), "output");
    assert_eq!(NotebookCommentStatusClass::Resolved.as_str(), "resolved");
    assert_eq!(NotebookCommentThreadState::Stale.as_str(), "stale");
    assert_eq!(NotebookAnchorKind::Output.as_str(), "output");
    assert_eq!(
        NotebookReviewWorkspaceParityClass::PartialCellAware.as_str(),
        "partial_cell_aware"
    );
    assert_eq!(
        NotebookReviewWorkspaceDowngradeReason::KernelUnavailable.as_str(),
        "kernel_unavailable"
    );
}

#[test]
fn packet_validates_clean() {
    let packet = NotebookCommentAnchorPacket {
        schema_version: NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION,
        record_kind: NOTEBOOK_COMMENT_ANCHOR_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.comment_anchor.packet.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        comment_target_classes: NotebookCommentTargetClass::ALL.to_vec(),
        comment_status_classes: NotebookCommentStatusClass::ALL.to_vec(),
        comment_thread_states: NotebookCommentThreadState::ALL.to_vec(),
        anchor_kinds: NotebookAnchorKind::ALL.to_vec(),
        review_workspace_parity_classes: NotebookReviewWorkspaceParityClass::ALL.to_vec(),
        review_workspace_downgrade_reasons: NotebookReviewWorkspaceDowngradeReason::ALL.to_vec(),
        example_comments: vec![sample_comment()],
        example_anchors: vec![sample_cell_anchor(), sample_output_anchor()],
        example_review_workspace_parities: vec![
            sample_review_workspace_parity_full(),
            sample_review_workspace_parity_degraded(),
        ],
        summary: "Comment/anchor/parity packet v1.".to_owned(),
    };
    assert!(
        packet.validate().is_empty(),
        "packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn embedded_packet_parses() {
    let packet = current_notebook_comment_anchor_packet().expect("embedded packet must parse");
    assert_eq!(
        packet.schema_version,
        NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        NOTEBOOK_COMMENT_ANCHOR_PACKET_RECORD_KIND
    );
}
