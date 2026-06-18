//! Inline unit tests for the imported-theme migration report.

use super::*;

#[test]
fn seeded_report_passes_validation() {
    let report = seeded_theme_import_report();
    validate_theme_import_report(&report).expect("seeded report must validate");
}

#[test]
fn seeded_report_is_clean() {
    let report = seeded_theme_import_report();
    assert!(report.is_clean());
    assert!(report.every_import_reversible);
    assert!(report.no_overclaimed_parity);
    assert!(report.unresolved_counts_disclosed);
    assert!(report.no_raw_theme_content);
}

#[test]
fn every_row_carries_provenance_and_reversible_rollback() {
    let report = seeded_theme_import_report();
    for row in &report.rows {
        assert!(
            !row.source_tool.source_tool_name.trim().is_empty(),
            "row {} must name its source tool",
            row.row_id
        );
        assert!(
            !row.source_tool.source_tool_version.trim().is_empty(),
            "row {} must name its source tool version",
            row.row_id
        );
        assert!(
            !row.source_tool.source_theme_identifier.trim().is_empty(),
            "row {} must carry a source-theme identifier",
            row.row_id
        );
        assert!(
            row.import_is_reversible(),
            "row {} must be reversible",
            row.row_id
        );
        assert!(
            !row.rollback.rollback_ref.trim().is_empty(),
            "row {} must carry a rollback ref",
            row.row_id
        );
    }
}

#[test]
fn mapping_and_syntax_counts_are_internally_consistent() {
    let report = seeded_theme_import_report();
    for row in &report.rows {
        assert!(
            row.mapping_summary.counts_are_consistent(),
            "row {} mapping summary must sum to its total",
            row.row_id
        );
        assert!(
            row.syntax_coverage.is_consistent(),
            "row {} syntax coverage must be consistent",
            row.row_id
        );
    }
}

#[test]
fn only_clean_rows_claim_full_parity() {
    let report = seeded_theme_import_report();
    for row in &report.rows {
        if row.claims_full_parity() {
            assert!(
                row.full_parity_is_backed(),
                "row {} claims parity it does not back",
                row.row_id
            );
            assert_eq!(row.mapping_summary.unresolved_mapping_count, 0);
        }
    }
    // At least one row claims full parity and at least one denies it.
    assert!(report.rows.iter().any(ThemeImportRow::claims_full_parity));
    assert!(report.rows.iter().any(|row| matches!(
        row.parity_claim_state,
        ParityClaimState::DeniedUnresolvedOrBlocked
    )));
}

#[test]
fn unresolved_counts_are_disclosed_with_slots() {
    let report = seeded_theme_import_report();
    for row in &report.rows {
        if row.mapping_summary.unresolved_mapping_count > 0 {
            assert!(
                !row.unresolved_slots.is_empty(),
                "row {} hides its unresolved slots",
                row.row_id
            );
            assert!(
                row.compatibility_note.is_some(),
                "row {} with unresolved slots must carry a compatibility note",
                row.row_id
            );
        }
    }
}

#[test]
fn outcome_and_aggregate_summaries_match_rows() {
    let report = seeded_theme_import_report();
    assert_eq!(report.outcome_summary.total_rows, report.rows.len());
    assert_eq!(
        report.outcome_summary.applied
            + report.outcome_summary.applied_with_warnings
            + report.outcome_summary.blocked
            + report.outcome_summary.rolled_back
            + report.outcome_summary.review_required
            + report.outcome_summary.preview_ready
            + report.outcome_summary.cancelled
            + report.outcome_summary.policy_denied,
        report.rows.len()
    );
    let translated: usize = report
        .rows
        .iter()
        .map(ThemeImportRow::translated_token_count)
        .sum();
    assert_eq!(report.aggregate_tokens.total_translated_slots, translated);
}

#[test]
fn covers_the_full_spectrum_of_outcomes() {
    let report = seeded_theme_import_report();
    let outcomes: Vec<ImportOutcomeState> =
        report.rows.iter().map(|row| row.import_outcome).collect();
    for expected in [
        ImportOutcomeState::Applied,
        ImportOutcomeState::AppliedWithWarnings,
        ImportOutcomeState::RolledBack,
        ImportOutcomeState::ReviewRequired,
        ImportOutcomeState::Blocked,
    ] {
        assert!(
            outcomes.contains(&expected),
            "seeded report must cover outcome {}",
            expected.as_str()
        );
    }
}

#[test]
fn support_export_quotes_provenance_and_rollback_refs() {
    let report = seeded_theme_import_report();
    let export = ThemeImportSupportExport::from_report(
        "support-export:m5-theme-import-reports:001",
        report.clone(),
    );
    assert!(export.case_ids.contains(&report.report_id));
    for row in &report.rows {
        assert!(
            export.case_ids.contains(&row.row_id),
            "support export must quote row id {}",
            row.row_id
        );
        assert!(
            export
                .case_ids
                .contains(&row.source_tool.source_theme_identifier),
            "support export must quote provenance for {}",
            row.row_id
        );
        assert!(
            export.case_ids.contains(&row.rollback.rollback_ref),
            "support export must quote rollback ref for {}",
            row.row_id
        );
    }
}

#[test]
fn validation_catches_overclaimed_parity() {
    let mut report = seeded_theme_import_report();
    // Force a partial row to claim full parity it cannot back.
    let row = report
        .rows
        .iter_mut()
        .find(|row| row.mapping_summary.unresolved_mapping_count > 0)
        .expect("a row with unresolved slots");
    row.parity_claim_state = ParityClaimState::ClaimedWithReport;
    let errors = validate_theme_import_report(&report).expect_err("must reject overclaim");
    assert!(errors
        .iter()
        .any(|err| matches!(err, ThemeImportValidationError::ParityOverclaimed { .. })));
}

#[test]
fn validation_catches_irreversible_applied_import() {
    let mut report = seeded_theme_import_report();
    let row = report
        .rows
        .iter_mut()
        .find(|row| row.import_outcome == ImportOutcomeState::Applied)
        .expect("an applied row");
    row.rollback.rollback_path_class = RollbackPathClass::RollbackUnavailableDenied;
    let errors = validate_theme_import_report(&report).expect_err("must reject irreversible apply");
    assert!(errors
        .iter()
        .any(|err| matches!(err, ThemeImportValidationError::RollbackPathMissing { .. })));
}

#[test]
fn validation_catches_hidden_unresolved_counts() {
    let mut report = seeded_theme_import_report();
    let row = report
        .rows
        .iter_mut()
        .find(|row| row.mapping_summary.unresolved_mapping_count > 0)
        .expect("a row with unresolved slots");
    row.unresolved_slots.clear();
    let errors = validate_theme_import_report(&report).expect_err("must reject hidden unresolved");
    assert!(errors.iter().any(|err| matches!(
        err,
        ThemeImportValidationError::UnresolvedCountHidden { .. }
    )));
}

#[test]
fn render_markdown_and_compact_are_non_empty() {
    let report = seeded_theme_import_report();
    let markdown = report.render_markdown();
    assert!(markdown.contains("M5 imported-theme mapping & rollback report"));
    assert!(markdown.contains("Unresolved slots"));
    let compact = report.compact_lines();
    assert!(compact.iter().any(|line| line.starts_with("report:")));
}
