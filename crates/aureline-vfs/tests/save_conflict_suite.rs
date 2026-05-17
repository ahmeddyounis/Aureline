//! Integration drill for the difficult filesystem, external-change, and
//! save-conflict regression suite.
//!
//! Loads the checked-in corpus, re-proves the closed safety contract,
//! and ensures the report projection preserves the per-platform matrix
//! truth across every claimed desktop row.

use std::path::PathBuf;

use aureline_vfs::identity_beta::{BetaCompareOutcome, BetaResolutionAction};
use aureline_vfs::save_conflict_suite::{
    current_save_conflict_suite_corpus, current_save_conflict_suite_fixture_refs,
    load_save_conflict_suite_case, DowngradeLabel, OpenGapClass, PlatformRowClass,
    RegressionOutcome, SaveConflictSuiteCorpusEntry, SaveConflictSuiteEvaluator,
    SaveConflictSuiteReport, ScenarioClass, REQUIRED_PLATFORM_ROW_CLASSES,
    REQUIRED_SCENARIO_CLASSES, SAVE_CONFLICT_SUITE_CORPUS_DIR,
    SAVE_CONFLICT_SUITE_CORPUS_MANIFEST_REF, SAVE_CONFLICT_SUITE_FILESYSTEM_IDENTITY_BETA_DOC_REF,
    SAVE_CONFLICT_SUITE_MATRIX_DOC_REF, SAVE_CONFLICT_SUITE_REPORT_RECORD_KIND,
    SAVE_CONFLICT_SUITE_REPORT_REF, SAVE_CONFLICT_SUITE_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn corpus_entries() -> Vec<SaveConflictSuiteCorpusEntry> {
    current_save_conflict_suite_corpus()
        .expect("checked-in save-conflict suite corpus must parse")
        .entries
}

#[test]
fn corpus_loads_and_validates() {
    let corpus = current_save_conflict_suite_corpus().expect("checked-in corpus must parse");
    SaveConflictSuiteEvaluator::new()
        .validate_corpus(&corpus)
        .expect("checked-in corpus must validate");
    assert!(!corpus.entries.is_empty(), "corpus must not be empty");
}

#[test]
fn corpus_covers_every_scenario_class_per_required_platform() {
    let entries = corpus_entries();
    for scenario in REQUIRED_SCENARIO_CLASSES {
        for platform in REQUIRED_PLATFORM_ROW_CLASSES {
            assert!(
                entries.iter().any(|entry| {
                    entry.case.scenario_class == scenario
                        && entry.case.platform_row_class == platform
                }),
                "required (scenario_class={}, platform_row_class={}) tuple has no seeded case",
                scenario.as_str(),
                platform.as_str()
            );
        }
    }
}

#[test]
fn fixture_files_exist_on_disk() {
    let root = repo_root();
    let manifest_path = root.join(SAVE_CONFLICT_SUITE_CORPUS_MANIFEST_REF);
    assert!(
        manifest_path.exists(),
        "manifest must exist on disk: {}",
        manifest_path.display()
    );
    let schema_path = root.join(SAVE_CONFLICT_SUITE_SCHEMA_REF);
    assert!(
        schema_path.exists(),
        "schema must exist on disk: {}",
        schema_path.display()
    );
    let matrix_path = root.join(SAVE_CONFLICT_SUITE_MATRIX_DOC_REF);
    assert!(
        matrix_path.exists(),
        "matrix doc must exist on disk: {}",
        matrix_path.display()
    );
    let report_path = root.join(SAVE_CONFLICT_SUITE_REPORT_REF);
    assert!(
        report_path.exists(),
        "baseline report must exist on disk: {}",
        report_path.display()
    );
    let fsid_doc_path = root.join(SAVE_CONFLICT_SUITE_FILESYSTEM_IDENTITY_BETA_DOC_REF);
    assert!(
        fsid_doc_path.exists(),
        "filesystem-identity beta doc must exist on disk: {}",
        fsid_doc_path.display()
    );
    let corpus_dir = root.join(SAVE_CONFLICT_SUITE_CORPUS_DIR);
    assert!(
        corpus_dir.is_dir(),
        "corpus directory must exist on disk: {}",
        corpus_dir.display()
    );
    for fixture_ref in current_save_conflict_suite_fixture_refs() {
        let path = root.join(fixture_ref);
        assert!(
            path.exists(),
            "fixture must exist on disk: {}",
            path.display()
        );
    }
}

#[test]
fn anchor_fixtures_exist_on_disk() {
    let root = repo_root();
    for entry in corpus_entries() {
        let path = root.join(&entry.case.anchor_fixture_ref);
        assert!(
            path.exists(),
            "anchor fixture must exist on disk: {} (case_id={})",
            path.display(),
            entry.case.case_id
        );
    }
}

#[test]
fn cases_round_trip_through_serde() {
    for entry in corpus_entries() {
        let yaml = serde_yaml::to_string(&entry.case).expect("case must serialize to yaml");
        let restored =
            load_save_conflict_suite_case(&yaml).expect("case must round-trip through yaml");
        assert_eq!(restored, entry.case, "{} round-trip", entry.case.case_id);
    }
}

#[test]
fn report_preserves_matrix_truth() {
    let corpus = current_save_conflict_suite_corpus().unwrap();
    let report: SaveConflictSuiteReport = SaveConflictSuiteEvaluator::new()
        .report(
            "report:save_conflict_suite:drill",
            "2026-05-16T10:00:00Z",
            &corpus,
        )
        .expect("report must build");
    assert_eq!(report.record_kind, SAVE_CONFLICT_SUITE_REPORT_RECORD_KIND);
    assert!(report.is_export_safe(), "report must be export-safe");
    assert!(report.raw_private_material_excluded);
    assert!(report.ambient_authority_excluded);
    assert_eq!(report.matrix_rows.len(), corpus.entries.len());
    assert_eq!(
        report.platform_summaries.len(),
        REQUIRED_PLATFORM_ROW_CLASSES.len()
    );
    let total_cases: u32 = report.platform_summaries.iter().map(|p| p.case_count).sum();
    assert_eq!(total_cases as usize, corpus.entries.len());
    for summary in &report.platform_summaries {
        assert_eq!(
            summary.case_count,
            summary.pass_count + summary.downgrade_required_count + summary.blocked_until_fix_count,
            "platform {} summary must reconcile pass/downgrade/blocked counts",
            summary.platform_row_class.as_str()
        );
    }
}

#[test]
fn external_change_rows_forbid_silent_overwrite_on_every_platform() {
    for entry in corpus_entries() {
        if entry.case.scenario_class != ScenarioClass::ExternalChange {
            continue;
        }
        let behavior = &entry.case.expected_behavior;
        assert!(
            behavior.silent_overwrite_forbidden,
            "external-change row {} must forbid silent overwrite",
            entry.case.case_id
        );
        assert_eq!(
            behavior.compare_outcome,
            BetaCompareOutcome::ExternalChangeDetected,
            "external-change row {} must declare compare_outcome = external_change_detected",
            entry.case.case_id
        );
        assert!(
            !behavior
                .resolution_actions
                .contains(&BetaResolutionAction::Write),
            "external-change row {} must not offer a direct write action",
            entry.case.case_id
        );
    }
}

#[test]
fn permission_loss_rows_record_a_blocker_on_every_platform() {
    for entry in corpus_entries() {
        if entry.case.scenario_class != ScenarioClass::PermissionLoss {
            continue;
        }
        let blockers = &entry.case.expected_behavior.required_blockers;
        assert!(
            !blockers.is_empty(),
            "permission-loss row {} must record at least one save-target blocker",
            entry.case.case_id
        );
    }
}

#[test]
fn downgraded_rows_record_an_open_gap_from_the_closed_vocabulary() {
    for entry in corpus_entries() {
        match entry.case.expected_outcome {
            RegressionOutcome::Pass => {
                assert_eq!(
                    entry.case.downgrade_label,
                    DowngradeLabel::None,
                    "pass row {} must declare downgrade_label = none",
                    entry.case.case_id
                );
            }
            RegressionOutcome::DowngradeRequired | RegressionOutcome::BlockedUntilFix => {
                assert_ne!(
                    entry.case.downgrade_label,
                    DowngradeLabel::None,
                    "downgraded row {} must declare a non-none downgrade_label",
                    entry.case.case_id
                );
                assert!(
                    entry
                        .case
                        .open_gaps
                        .iter()
                        .any(|gap| gap.gap_class != OpenGapClass::None),
                    "downgraded row {} must record an open_gap from the closed vocabulary",
                    entry.case.case_id
                );
            }
        }
    }
}

#[test]
fn refuses_pass_with_downgrade_label() {
    let mut corpus = current_save_conflict_suite_corpus().unwrap();
    for entry in corpus.entries.iter_mut() {
        if entry.case.expected_outcome == RegressionOutcome::Pass {
            entry.case.downgrade_label = DowngradeLabel::YellowPlatformSkew;
            break;
        }
    }
    let err = SaveConflictSuiteEvaluator::new()
        .validate_corpus(&corpus)
        .expect_err("pass with downgrade must fail");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "case.outcome.pass_must_not_carry_downgrade"));
}

#[test]
fn refuses_dropped_user_authored_files_preservation() {
    let mut corpus = current_save_conflict_suite_corpus().unwrap();
    corpus.entries[0].case.safety.preserves_user_authored_files = false;
    let err = SaveConflictSuiteEvaluator::new()
        .validate_corpus(&corpus)
        .expect_err("dropped user-files preservation must fail validation");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "case.safety.preserves_user_authored_files"));
}

#[test]
fn refuses_admitted_destructive_reset() {
    let mut corpus = current_save_conflict_suite_corpus().unwrap();
    corpus.entries[0].case.safety.destructive_resets_present = true;
    let err = SaveConflictSuiteEvaluator::new()
        .validate_corpus(&corpus)
        .expect_err("admitted destructive reset must fail validation");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "case.safety.destructive_resets_present"));
}

#[test]
fn refuses_corpus_missing_scenario_platform_tuple() {
    let full = current_save_conflict_suite_corpus().unwrap();
    let mut truncated = full.clone();
    truncated.entries.retain(|entry| {
        !(entry.case.scenario_class == ScenarioClass::PermissionLoss
            && entry.case.platform_row_class == PlatformRowClass::WindowsDesktop)
    });
    let err = SaveConflictSuiteEvaluator::new()
        .validate_corpus(&truncated)
        .expect_err("removing a required tuple must fail");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "corpus.required_scenario_platform_missing"));
}
