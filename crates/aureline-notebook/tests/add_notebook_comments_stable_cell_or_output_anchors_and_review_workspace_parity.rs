//! End-to-end coverage for notebook comments, stable cell or output anchors,
//! and review-workspace parity.

use std::path::{Path, PathBuf};

use aureline_notebook::{
    current_notebook_comment_anchor_packet, NotebookAnchor, NotebookAnchorKind, NotebookComment,
    NotebookCommentAnchorPacket, NotebookCommentStatusClass, NotebookCommentTargetClass,
    NotebookCommentThreadState, NotebookReviewWorkspaceDowngradeReason,
    NotebookReviewWorkspaceParity, NotebookReviewWorkspaceParityClass,
    NOTEBOOK_COMMENT_ANCHOR_PACKET_JSON, NOTEBOOK_COMMENT_ANCHOR_PACKET_RECORD_KIND,
    NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION,
};
use serde::Deserialize;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../")
}

fn fixture_root() -> PathBuf {
    repo_root().join(
        "fixtures/notebook/m5/add_notebook_comments_stable_cell_or_output_anchors_and_review_workspace_parity",
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
    comment_target_classes: Vec<String>,
    comment_status_classes: Vec<String>,
    comment_thread_states: Vec<String>,
    anchor_kinds: Vec<String>,
    review_workspace_parity_classes: Vec<String>,
    review_workspace_downgrade_reasons: Vec<String>,
    #[allow(dead_code)]
    case_refs: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    #[serde(rename = "__fixture__")]
    #[allow(dead_code)]
    fixture: FixtureMeta,
    #[serde(default)]
    notebook_comment: Option<NotebookComment>,
    #[serde(default)]
    notebook_anchor: Option<NotebookAnchor>,
    #[serde(default)]
    notebook_review_workspace_parity: Option<NotebookReviewWorkspaceParity>,
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
    comment_target_class: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    comment_status: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    anchor_kind: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    parity_class: Option<String>,
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
    assert_eq!(
        manifest.schema_version,
        NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION
    );
    assert_eq!(
        manifest.record_kind,
        NOTEBOOK_COMMENT_ANCHOR_PACKET_RECORD_KIND
    );
}

#[test]
fn manifest_vocabularies_are_complete() {
    let manifest = load_manifest();

    let expected_comment_target_classes: Vec<String> = NotebookCommentTargetClass::ALL
        .iter()
        .map(|v| v.as_str().to_string())
        .collect();
    assert_eq!(
        manifest.comment_target_classes,
        expected_comment_target_classes
    );

    let expected_comment_status_classes: Vec<String> = NotebookCommentStatusClass::ALL
        .iter()
        .map(|v| v.as_str().to_string())
        .collect();
    assert_eq!(
        manifest.comment_status_classes,
        expected_comment_status_classes
    );

    let expected_comment_thread_states: Vec<String> = NotebookCommentThreadState::ALL
        .iter()
        .map(|v| v.as_str().to_string())
        .collect();
    assert_eq!(
        manifest.comment_thread_states,
        expected_comment_thread_states
    );

    let expected_anchor_kinds: Vec<String> = NotebookAnchorKind::ALL
        .iter()
        .map(|v| v.as_str().to_string())
        .collect();
    assert_eq!(manifest.anchor_kinds, expected_anchor_kinds);

    let expected_parity_classes: Vec<String> = NotebookReviewWorkspaceParityClass::ALL
        .iter()
        .map(|v| v.as_str().to_string())
        .collect();
    assert_eq!(
        manifest.review_workspace_parity_classes,
        expected_parity_classes
    );

    let expected_downgrade_reasons: Vec<String> = NotebookReviewWorkspaceDowngradeReason::ALL
        .iter()
        .map(|v| v.as_str().to_string())
        .collect();
    assert_eq!(
        manifest.review_workspace_downgrade_reasons,
        expected_downgrade_reasons
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
fn fixture_comment_on_cell_validates() {
    let case = load_case("comment_on_cell");
    if let Some(comment) = case.notebook_comment {
        assert!(
            comment.validate().is_empty(),
            "comment findings: {:?}",
            comment.validate()
        );
        assert_eq!(
            comment.comment_target_class,
            NotebookCommentTargetClass::Cell
        );
        assert_eq!(comment.comment_status, NotebookCommentStatusClass::Active);
    }
    if let Some(anchor) = case.notebook_anchor {
        assert!(
            anchor.validate().is_empty(),
            "anchor findings: {:?}",
            anchor.validate()
        );
        assert_eq!(anchor.anchor_kind, NotebookAnchorKind::Cell);
    }
}

#[test]
fn fixture_comment_on_output_validates() {
    let case = load_case("comment_on_output");
    if let Some(comment) = case.notebook_comment {
        assert!(
            comment.validate().is_empty(),
            "comment findings: {:?}",
            comment.validate()
        );
        assert_eq!(
            comment.comment_target_class,
            NotebookCommentTargetClass::Output
        );
        assert_eq!(comment.comment_status, NotebookCommentStatusClass::Resolved);
    }
    if let Some(anchor) = case.notebook_anchor {
        assert!(
            anchor.validate().is_empty(),
            "anchor findings: {:?}",
            anchor.validate()
        );
        assert_eq!(anchor.anchor_kind, NotebookAnchorKind::Output);
        assert!(anchor.output_handle_ref.is_some());
    }
}

#[test]
fn fixture_review_workspace_full_parity_validates() {
    let case = load_case("review_workspace_full_parity");
    if let Some(parity) = case.notebook_review_workspace_parity {
        assert!(
            parity.validate().is_empty(),
            "parity findings: {:?}",
            parity.validate()
        );
        assert_eq!(
            parity.parity_class,
            NotebookReviewWorkspaceParityClass::Full
        );
        assert!(parity.downgrade_reasons.is_empty());
    }
}

#[test]
fn fixture_review_workspace_degraded_validates() {
    let case = load_case("review_workspace_degraded");
    if let Some(parity) = case.notebook_review_workspace_parity {
        assert!(
            parity.validate().is_empty(),
            "parity findings: {:?}",
            parity.validate()
        );
        assert_eq!(
            parity.parity_class,
            NotebookReviewWorkspaceParityClass::Degraded
        );
        assert!(!parity.downgrade_reasons.is_empty());
    }
}

#[test]
fn fixture_review_workspace_raw_fallback_validates() {
    let case = load_case("review_workspace_raw_fallback");
    if let Some(parity) = case.notebook_review_workspace_parity {
        assert!(
            parity.validate().is_empty(),
            "parity findings: {:?}",
            parity.validate()
        );
        assert_eq!(
            parity.parity_class,
            NotebookReviewWorkspaceParityClass::RawFallback
        );
        assert!(!parity.downgrade_reasons.is_empty());
    }
}

#[test]
fn embedded_comment_anchor_packet_is_valid_json() {
    let parsed: NotebookCommentAnchorPacket =
        serde_json::from_str(NOTEBOOK_COMMENT_ANCHOR_PACKET_JSON).expect("packet JSON must parse");
    assert_eq!(
        parsed.schema_version,
        NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION
    );
    assert_eq!(
        parsed.record_kind,
        NOTEBOOK_COMMENT_ANCHOR_PACKET_RECORD_KIND
    );
}

#[test]
fn current_comment_anchor_packet_validates_clean() {
    let packet = current_notebook_comment_anchor_packet().expect("current packet must parse");
    let findings = packet.validate();
    assert!(
        findings.is_empty(),
        "current packet should validate cleanly: {:?}",
        findings
    );
}

#[test]
fn comment_anchor_packet_covers_all_closed_vocabularies() {
    let packet = current_notebook_comment_anchor_packet().expect("current packet must parse");
    assert_eq!(
        packet.comment_target_classes.len(),
        NotebookCommentTargetClass::ALL.len()
    );
    assert_eq!(
        packet.comment_status_classes.len(),
        NotebookCommentStatusClass::ALL.len()
    );
    assert_eq!(
        packet.comment_thread_states.len(),
        NotebookCommentThreadState::ALL.len()
    );
    assert_eq!(packet.anchor_kinds.len(), NotebookAnchorKind::ALL.len());
    assert_eq!(
        packet.review_workspace_parity_classes.len(),
        NotebookReviewWorkspaceParityClass::ALL.len()
    );
    assert_eq!(
        packet.review_workspace_downgrade_reasons.len(),
        NotebookReviewWorkspaceDowngradeReason::ALL.len()
    );
}

#[test]
fn comment_round_trips_through_json() {
    let comment = NotebookComment {
        record_kind: "notebook_comment".to_owned(),
        notebook_comment_anchor_schema_version: NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION,
        comment_id: "nb.comment.rt".to_owned(),
        document_id_ref: "nb.doc.rt".to_owned(),
        anchor_ref: "nb.anchor.rt".to_owned(),
        comment_target_class: NotebookCommentTargetClass::Cell,
        comment_status: NotebookCommentStatusClass::Active,
        thread_state: NotebookCommentThreadState::Open,
        author_ref: "actor.rt".to_owned(),
        reply_refs: vec!["reply.1".to_owned()],
        redaction_profile_ref: None,
        summary: "Round-trip comment.".to_owned(),
    };
    let json = serde_json::to_string(&comment).expect("must serialize");
    let back: NotebookComment = serde_json::from_str(&json).expect("must deserialize");
    assert_eq!(back, comment);
}

#[test]
fn anchor_round_trips_through_json() {
    let anchor = NotebookAnchor {
        record_kind: "notebook_anchor".to_owned(),
        notebook_comment_anchor_schema_version: NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION,
        anchor_id: "nb.anchor.rt".to_owned(),
        document_id_ref: "nb.doc.rt".to_owned(),
        anchor_kind: NotebookAnchorKind::Output,
        cell_id_ref: "nb.cell.rt".to_owned(),
        output_handle_ref: Some("output.rt.1".to_owned()),
        summary: "Round-trip output anchor.".to_owned(),
    };
    let json = serde_json::to_string(&anchor).expect("must serialize");
    let back: NotebookAnchor = serde_json::from_str(&json).expect("must deserialize");
    assert_eq!(back, anchor);
}

#[test]
fn review_workspace_parity_round_trips_through_json() {
    let parity = NotebookReviewWorkspaceParity {
        record_kind: "notebook_review_workspace_parity".to_owned(),
        notebook_comment_anchor_schema_version: NOTEBOOK_COMMENT_ANCHOR_SCHEMA_VERSION,
        parity_id: "nb.parity.rt".to_owned(),
        document_id_ref: "nb.doc.rt".to_owned(),
        parity_class: NotebookReviewWorkspaceParityClass::PartialCellAware,
        downgrade_reasons: vec![NotebookReviewWorkspaceDowngradeReason::Redacted],
        anchor_refs: vec!["anchor.1".to_owned()],
        comment_refs: vec!["comment.1".to_owned()],
        summary: "Round-trip parity.".to_owned(),
    };
    let json = serde_json::to_string(&parity).expect("must serialize");
    let back: NotebookReviewWorkspaceParity =
        serde_json::from_str(&json).expect("must deserialize");
    assert_eq!(back, parity);
}
