//! Unit tests for the M5 depth-import migration & compatibility report.

use super::*;

#[test]
fn seeded_report_covers_every_required_class() {
    let report = seeded_m5_depth_import_report();
    assert!(report.covers_every_class());
    assert_eq!(
        report.row_count(),
        DepthArtifactClass::required_classes().len()
    );
    for class in DepthArtifactClass::required_classes() {
        assert!(
            report.rows.iter().any(|row| row.artifact_class == class),
            "missing class {}",
            class.as_str()
        );
    }
}

#[test]
fn seeded_report_validates() {
    let report = seeded_m5_depth_import_report();
    assert_eq!(validate_m5_depth_import_report(&report), Ok(()));
}

#[test]
fn every_mutating_apply_is_checkpointed() {
    let report = seeded_m5_depth_import_report();
    for row in &report.rows {
        assert!(
            row.apply_is_reversible(),
            "row {} mutates durable state without a checkpoint",
            row.row_id
        );
    }
    assert!(report.every_apply_reversible);
}

#[test]
fn no_non_native_row_is_marketed_as_parity() {
    let report = seeded_m5_depth_import_report();
    assert!(report.no_overclaimed_parity);
    for row in &report.rows {
        if row.continuity_scope.claims_native_parity() {
            assert!(matches!(
                row.outcome,
                InteropOutcome::Imported | InteropOutcome::Mapped
            ));
            assert!(matches!(
                row.fidelity,
                ImportMappingClassification::Exact | ImportMappingClassification::Translated
            ));
        } else {
            assert!(
                row.compatibility_note.is_some(),
                "non-native row {} must disclose a compatibility note",
                row.row_id
            );
        }
    }
}

#[test]
fn bridge_rows_carry_a_bridge_requirement() {
    let report = seeded_m5_depth_import_report();
    for row in &report.rows {
        if matches!(row.outcome, InteropOutcome::BridgeRequired) {
            assert!(
                row.bridge_requirement.is_some(),
                "bridge_required row {} must carry a bridge requirement",
                row.row_id
            );
        }
    }
}

#[test]
fn outcome_summary_matches_rows() {
    let report = seeded_m5_depth_import_report();
    let recomputed = OutcomeSummary::from_rows(&report.rows);
    assert_eq!(recomputed, report.outcome_summary);
    assert_eq!(report.outcome_summary.total_rows, report.rows.len());
}

#[test]
fn report_round_trips_through_json() {
    let report = seeded_m5_depth_import_report();
    let json = serde_json::to_string(&report).expect("serialize");
    let decoded: DepthImportReport = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(report, decoded);
}

#[test]
fn support_export_collects_row_and_checkpoint_ids() {
    let report = seeded_m5_depth_import_report();
    let export = DepthImportSupportExport::from_report(
        "support-export:m5-depth-imports:001",
        report.clone(),
    );
    assert!(export.case_ids.contains(&report.report_id));
    for row in &report.rows {
        assert!(export.case_ids.contains(&row.row_id));
        if let Some(checkpoint) = &row.restore_checkpoint {
            assert!(export.case_ids.contains(&checkpoint.checkpoint_id));
        }
    }
}

#[test]
fn validation_flags_mutating_apply_without_checkpoint() {
    let mut report = seeded_m5_depth_import_report();
    let row = report
        .rows
        .iter_mut()
        .find(|row| row.mutates_durable_state)
        .expect("a mutating row");
    row.restore_checkpoint = None;
    let errors = validate_m5_depth_import_report(&report).expect_err("must fail");
    assert!(errors.iter().any(|err| matches!(
        err,
        DepthImportValidationError::MutatingApplyWithoutCheckpoint { .. }
    )));
}

#[test]
fn validation_flags_overclaimed_parity() {
    let mut report = seeded_m5_depth_import_report();
    // Force a non-native, low-fidelity row to claim native parity.
    let row = report
        .rows
        .iter_mut()
        .find(|row| !row.continuity_scope.claims_native_parity())
        .expect("a non-native row");
    row.continuity_scope = ContinuityScope::Native;
    let errors = validate_m5_depth_import_report(&report).expect_err("must fail");
    assert!(errors.iter().any(|err| matches!(
        err,
        DepthImportValidationError::NativeParityOverclaimed { .. }
    )));
}

#[test]
fn validation_flags_missing_compatibility_note() {
    let mut report = seeded_m5_depth_import_report();
    let row = report
        .rows
        .iter_mut()
        .find(|row| !row.continuity_scope.claims_native_parity())
        .expect("a non-native row");
    row.compatibility_note = None;
    let errors = validate_m5_depth_import_report(&report).expect_err("must fail");
    assert!(errors.iter().any(|err| matches!(
        err,
        DepthImportValidationError::CompatibilityNoteMissing { .. }
    )));
}

#[test]
fn validation_flags_missing_escalation_refs() {
    let mut report = seeded_m5_depth_import_report();
    report.export_refs.clear();
    let errors = validate_m5_depth_import_report(&report).expect_err("must fail");
    assert!(errors
        .iter()
        .any(|err| matches!(err, DepthImportValidationError::EscalationRefsMissing)));
}

#[test]
fn validation_flags_stale_class_coverage() {
    let mut report = seeded_m5_depth_import_report();
    report.rows.pop();
    let errors = validate_m5_depth_import_report(&report).expect_err("must fail");
    assert!(errors
        .iter()
        .any(|err| matches!(err, DepthImportValidationError::ClassCoverageStale)));
    assert!(errors
        .iter()
        .any(|err| matches!(err, DepthImportValidationError::MissingArtifactClass { .. })));
}

#[test]
fn compact_lines_and_markdown_render() {
    let report = seeded_m5_depth_import_report();
    let lines = report.compact_lines();
    assert!(lines.iter().any(|line| line.contains("notebook_handoff")));
    let markdown = report.render_markdown();
    assert!(markdown.contains("M5 depth-import migration & compatibility report"));
    assert!(markdown.contains("Companion / export packet"));
}
