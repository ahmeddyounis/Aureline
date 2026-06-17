//! Inline coverage for topology-aware lane review sheets.

use super::*;

use aureline_git::{current_topology_action_review_packet, TopologyActionSheet};

fn sheets() -> Vec<TopologyActionSheet> {
    current_topology_action_review_packet()
        .expect("git action packet validates")
        .sheets
}

fn wrong_root_sheet(sheets: &[TopologyActionSheet]) -> &TopologyActionSheet {
    sheets
        .iter()
        .find(|sheet| sheet.wrong_root_guard.blocks())
        .expect("a wrong-root sheet exists in the canonical packet")
}

fn in_scope_sheet(sheets: &[TopologyActionSheet]) -> &TopologyActionSheet {
    sheets
        .iter()
        .find(|sheet| !sheet.wrong_root_guard.blocks())
        .expect("an in-scope sheet exists in the canonical packet")
}

fn packet(
    rows: Vec<TopologyReviewSheetRow>,
    action_sheets: Vec<TopologyActionSheet>,
) -> GitTopologyReviewPacket {
    let support_export = GitTopologyReviewSupportExport {
        record_kind: GIT_TOPOLOGY_REVIEW_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        export_id: "git-topology-review-export:test".to_owned(),
        row_refs: rows.iter().map(|row| row.row_id.clone()).collect(),
        reconstruction_fields: GIT_TOPOLOGY_REVIEW_REQUIRED_RECONSTRUCTION_FIELDS
            .iter()
            .map(|field| (*field).to_owned())
            .collect(),
        raw_paths_redacted: true,
        raw_object_bytes_redacted: true,
    };
    GitTopologyReviewPacket {
        record_kind: GIT_TOPOLOGY_REVIEW_PACKET_RECORD_KIND.to_owned(),
        schema_version: GIT_TOPOLOGY_REVIEW_SCHEMA_VERSION,
        packet_id: "git-topology-review:test".to_owned(),
        generated_at: "2026-06-17T00:00:00Z".to_owned(),
        action_sheets,
        rows,
        support_export,
    }
}

#[test]
fn every_lane_surfaces_explicit_labels_not_generic_states() {
    let sheets = sheets();
    let mut rows = Vec::new();
    for lane in TopologyReviewLane::ALL {
        for sheet in &sheets {
            rows.push(TopologyReviewSheetRow::for_lane_and_sheet(
                lane,
                sheet,
                format!("review-{}-{}", lane.as_str(), sheet.sheet_id),
            ));
        }
    }
    for row in &rows {
        assert!(row.generic_state_suppressed);
        assert!(!row.mutation_applied);
    }
    let packet = packet(rows, sheets);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn in_scope_row_recommends_the_reviewed_verb_without_mutating() {
    let sheets = sheets();
    let sheet = in_scope_sheet(&sheets);
    let row = TopologyReviewSheetRow::for_lane_and_sheet(TopologyReviewLane::Search, sheet, "row");
    assert_eq!(row.recommended_action, Some(sheet.action_kind));
    assert!(!row.mutation_applied);
}

#[test]
fn wrong_root_row_recommends_nothing() {
    let sheets = sheets();
    let sheet = wrong_root_sheet(&sheets);
    let row = TopologyReviewSheetRow::for_lane_and_sheet(TopologyReviewLane::Ai, sheet, "row");
    assert_eq!(row.recommended_action, None);
    assert_eq!(row.scope_limit_label, ScopeLimitLabel::WrongTargetRoot);
}

#[test]
fn implicit_mutation_is_rejected() {
    let sheets = sheets();
    let sheet = in_scope_sheet(&sheets);
    let mut row =
        TopologyReviewSheetRow::for_lane_and_sheet(TopologyReviewLane::Review, sheet, "row");
    row.mutation_applied = true;
    let packet = packet(vec![row], sheets);
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        GitTopologyReviewValidationError::MutationAppliedInLane { .. }
    )));
}

#[test]
fn generic_empty_state_is_rejected() {
    let sheets = sheets();
    let sheet = in_scope_sheet(&sheets);
    let mut row =
        TopologyReviewSheetRow::for_lane_and_sheet(TopologyReviewLane::Search, sheet, "row");
    row.generic_state_suppressed = false;
    let packet = packet(vec![row], sheets);
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        GitTopologyReviewValidationError::GenericStateNotSuppressed { .. }
    )));
}

#[test]
fn label_must_match_the_reviewed_sheet() {
    let sheets = sheets();
    let sheet = in_scope_sheet(&sheets);
    let mut row =
        TopologyReviewSheetRow::for_lane_and_sheet(TopologyReviewLane::Blame, sheet, "row");
    // Force a label that does not match the sheet's repaired state.
    row.scope_limit_label = match row.scope_limit_label {
        ScopeLimitLabel::PointerOnly => ScopeLimitLabel::Unfetched,
        _ => ScopeLimitLabel::PointerOnly,
    };
    let packet = packet(vec![row], sheets);
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        GitTopologyReviewValidationError::LabelMismatch { .. }
    )));
}

#[test]
fn recommending_across_a_wrong_root_is_rejected() {
    let sheets = sheets();
    let sheet = wrong_root_sheet(&sheets);
    let mut row =
        TopologyReviewSheetRow::for_lane_and_sheet(TopologyReviewLane::Search, sheet, "row");
    row.recommended_action = Some(sheet.action_kind);
    let packet = packet(vec![row], sheets);
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        GitTopologyReviewValidationError::RecommendationAcrossWrongRoot { .. }
    )));
}

#[test]
fn packet_round_trips_through_json() {
    let sheets = sheets();
    let rows = vec![TopologyReviewSheetRow::for_lane_and_sheet(
        TopologyReviewLane::Search,
        in_scope_sheet(&sheets),
        "row",
    )];
    let packet = packet(rows, sheets);
    let json = packet.export_safe_json();
    let parsed = GitTopologyReviewPacket::parse_json(&json).expect("round-trips");
    assert_eq!(parsed, packet);
}
