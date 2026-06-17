//! Inline coverage for history-surgery decision rows.

use super::*;

use aureline_git::{
    current_history_surgery_review_sheets, HistorySurgeryReviewPacket, HistorySurgeryReviewSheet,
};

const FORCE_PUSH_BLOCKED_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/rebase-cherry-pick-reset/force_push_protected_blocked.json"
));

const PROVIDER_OUTAGE_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/rebase-cherry-pick-reset/reset_provider_outage_local_only.json"
));

fn sheets() -> Vec<HistorySurgeryReviewSheet> {
    current_history_surgery_review_sheets()
        .expect("git history-surgery packet validates")
        .sheets
}

fn fixture_sheet(raw: &str) -> HistorySurgeryReviewSheet {
    HistorySurgeryReviewPacket::parse_json(raw)
        .expect("fixture parses")
        .sheets
        .into_iter()
        .next()
        .expect("fixture has a sheet")
}

fn packet(
    rows: Vec<HistorySurgeryDecisionRow>,
    sheets: Vec<HistorySurgeryReviewSheet>,
) -> GitHistoryReviewPacket {
    let support_export = GitHistoryReviewSupportExport {
        record_kind: GIT_HISTORY_REVIEW_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        export_id: "git-history-review-export:test".to_owned(),
        row_refs: rows.iter().map(|row| row.row_id.clone()).collect(),
        reconstruction_fields: GIT_HISTORY_REVIEW_REQUIRED_RECONSTRUCTION_FIELDS
            .iter()
            .map(|field| (*field).to_owned())
            .collect(),
        raw_paths_redacted: true,
        raw_patch_bodies_redacted: true,
        raw_provider_payloads_redacted: true,
    };
    GitHistoryReviewPacket {
        record_kind: GIT_HISTORY_REVIEW_PACKET_RECORD_KIND.to_owned(),
        schema_version: GIT_HISTORY_REVIEW_SCHEMA_VERSION,
        packet_id: "git-history-review:test".to_owned(),
        generated_at: "2026-06-17T00:00:00Z".to_owned(),
        repo_ref: "repo-ref:main".to_owned(),
        sheets,
        rows,
        support_export,
    }
}

#[test]
fn checked_artifact_validates() {
    let packet = current_git_history_review_packet().expect("checked packet validates");
    assert_eq!(packet.packet_id, "git-history-review:0001");
}

#[test]
fn every_surface_restates_the_same_decision() {
    let sheets = sheets();
    let mut rows = Vec::new();
    for surface in HistoryReviewSurface::ALL {
        for sheet in &sheets {
            rows.push(HistorySurgeryDecisionRow::for_surface_and_sheet(
                surface,
                sheet,
                format!("row-{}-{}", surface.as_str(), sheet.sheet_id),
            ));
        }
    }
    // Every surface carries the identical outcome and reason for a given sheet.
    for sheet in &sheets {
        let restatements: Vec<_> = rows
            .iter()
            .filter(|row| row.sheet_ref == sheet.sheet_id)
            .collect();
        assert_eq!(restatements.len(), HistoryReviewSurface::ALL.len());
        for row in restatements {
            assert_eq!(row.outcome, sheet.decision.outcome);
            assert_eq!(row.primary_reason, sheet.decision.primary_reason);
            assert_eq!(row.recovery_visible, sheet.decision.recovery_visible);
        }
    }
    let packet = packet(rows, sheets);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn only_mutation_surfaces_execute_an_allowed_decision() {
    let sheets = sheets();
    for sheet in &sheets {
        for surface in HistoryReviewSurface::ALL {
            let row = HistorySurgeryDecisionRow::for_surface_and_sheet(surface, sheet, "row");
            let expected = surface.is_mutation_surface()
                && sheet.decision.outcome == ReviewDecisionOutcome::Allowed;
            assert_eq!(
                row.execution_permitted,
                expected,
                "{} / {} execution gate wrong",
                surface.as_str(),
                sheet.sheet_id
            );
        }
    }
}

#[test]
fn blocked_force_push_never_executes_but_still_explains() {
    let sheet = fixture_sheet(FORCE_PUSH_BLOCKED_FIXTURE);
    for surface in HistoryReviewSurface::ALL {
        let row = HistorySurgeryDecisionRow::for_surface_and_sheet(surface, &sheet, "row");
        // A blocked mutation never executes on any surface...
        assert!(!row.execution_permitted);
        // ...yet every surface still explains why and keeps local truth visible.
        assert_eq!(row.outcome, ReviewDecisionOutcome::Blocked);
        assert_eq!(row.primary_reason, "blocked_protected_branch");
        assert!(row.local_truth_available_offline);
    }
}

#[test]
fn provider_outage_row_keeps_local_truth_and_downgrades() {
    let sheet = fixture_sheet(PROVIDER_OUTAGE_FIXTURE);
    let row = HistorySurgeryDecisionRow::for_surface_and_sheet(
        HistoryReviewSurface::Review,
        &sheet,
        "row",
    );
    assert_ne!(row.outcome, ReviewDecisionOutcome::Blocked);
    assert!(row.local_truth_available_offline);
}

#[test]
fn read_only_surface_marked_executable_is_rejected() {
    let sheets = sheets();
    let mut row = HistorySurgeryDecisionRow::for_surface_and_sheet(
        HistoryReviewSurface::SupportExport,
        &sheets[0],
        "row",
    );
    row.execution_permitted = true;
    let packet = packet(vec![row], sheets);
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        GitHistoryReviewValidationError::RowDoesNotMatchSheet { .. }
            | GitHistoryReviewValidationError::ReadOnlySurfaceExecutable { .. }
    )));
}

#[test]
fn tampered_row_outcome_is_rejected() {
    let sheets = sheets();
    let mut row = HistorySurgeryDecisionRow::for_surface_and_sheet(
        HistoryReviewSurface::Review,
        &sheets[0],
        "row",
    );
    row.primary_reason = "downgraded_raw_inspection_only".to_owned();
    let packet = packet(vec![row], sheets);
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        GitHistoryReviewValidationError::RowDoesNotMatchSheet { .. }
    )));
}

#[test]
fn unknown_sheet_ref_is_rejected() {
    let sheets = sheets();
    let mut row = HistorySurgeryDecisionRow::for_surface_and_sheet(
        HistoryReviewSurface::Review,
        &sheets[0],
        "row",
    );
    row.sheet_ref = "ghost".to_owned();
    let packet = packet(vec![row], sheets);
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        GitHistoryReviewValidationError::UnknownSheetRef { .. }
            | GitHistoryReviewValidationError::RowDoesNotMatchSheet { .. }
    )));
}

#[test]
fn embedded_invalid_sheet_is_rejected() {
    let mut sheets = sheets();
    // Corrupt a sheet so the embedded Git contract rejects it.
    sheets[0].primary_target_ref = String::new();
    let row = HistorySurgeryDecisionRow::for_surface_and_sheet(
        HistoryReviewSurface::Review,
        &sheets[0],
        "row",
    );
    let packet = packet(vec![row], sheets);
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        GitHistoryReviewValidationError::EmbeddedSheetInvalid { .. }
    )));
}

#[test]
fn packet_round_trips_through_json() {
    let sheets = sheets();
    let rows = vec![HistorySurgeryDecisionRow::for_surface_and_sheet(
        HistoryReviewSurface::Review,
        &sheets[0],
        "row",
    )];
    let packet = packet(rows, sheets);
    let json = packet.export_safe_json();
    let parsed = GitHistoryReviewPacket::parse_json(&json).expect("round-trips");
    assert_eq!(parsed, packet);
}

#[test]
fn support_export_missing_field_is_rejected() {
    let sheets = sheets();
    let row = HistorySurgeryDecisionRow::for_surface_and_sheet(
        HistoryReviewSurface::Review,
        &sheets[0],
        "row",
    );
    let mut packet = packet(vec![row], sheets);
    packet
        .support_export
        .reconstruction_fields
        .retain(|field| field != "decision_outcome");
    assert!(packet.validate().iter().any(|error| matches!(
        error,
        GitHistoryReviewValidationError::SupportExportMissingField { .. }
    )));
}
