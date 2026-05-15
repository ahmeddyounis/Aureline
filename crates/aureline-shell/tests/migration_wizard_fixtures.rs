//! Protected fixture checks for the beta migration-wizard projection.
//!
//! The integration test replays every JSON fixture under
//! `fixtures/migration/m3/migration_wizard/` through the Rust types and
//! asserts the contract invariants. The page fixture is also asserted
//! bit-for-bit equal to the page minted by `seeded_migration_wizard_page`
//! so the headless inspector remains the only mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_shell::migration_wizard::{
    seeded_migration_wizard_page, validate_migration_wizard_page, MigrationWizardPage,
    MigrationWizardSupportExport, UnsupportedGapRow, WizardCompareAction, WizardMappingReport,
    WizardMappingReportRow, WizardRollbackCheckpointBinding, WizardStage, WizardStageTransition,
    WizardUndoAction, MIGRATION_WIZARD_PAGE_RECORD_KIND, MIGRATION_WIZARD_SHARED_CONTRACT_REF,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/migration/m3/migration_wizard")
}

fn load_json<T: serde::de::DeserializeOwned>(file: &str) -> T {
    let path = fixtures_root().join(file);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn fixture_page_is_bit_for_bit_equal_to_seed() {
    let on_disk: MigrationWizardPage = load_json("page.json");
    let seeded = seeded_migration_wizard_page();
    assert_eq!(on_disk, seeded, "fixture page diverged from seeded page");
    assert_eq!(seeded.record_kind, MIGRATION_WIZARD_PAGE_RECORD_KIND);
    assert_eq!(
        seeded.shared_contract_ref,
        MIGRATION_WIZARD_SHARED_CONTRACT_REF
    );
}

#[test]
fn fixture_page_passes_validation() {
    let page: MigrationWizardPage = load_json("page.json");
    validate_migration_wizard_page(&page).expect("fixture page must validate");
}

#[test]
fn fixture_page_classifies_every_required_class() {
    let page: MigrationWizardPage = load_json("page.json");
    assert!(page.mapping_report.covers_every_required_classification());
    assert!(page.mapping_report.every_row_classified());
}

#[test]
fn fixture_mapping_report_matches_page() {
    let page: MigrationWizardPage = load_json("page.json");
    let report: WizardMappingReport = load_json("mapping_report.json");
    assert_eq!(page.mapping_report, report);
    assert!(report.has_required_reopen_surfaces());
}

#[test]
fn fixture_unsupported_gaps_match_page() {
    let page: MigrationWizardPage = load_json("page.json");
    let gaps: Vec<UnsupportedGapRow> = load_json("unsupported_gaps.json");
    assert_eq!(page.mapping_report.unsupported_gaps, gaps);
    assert!(
        gaps.iter()
            .all(|gap| gap.visible_before_apply && gap.retained_after_apply),
        "unsupported gaps must remain visible before and after apply"
    );
}

#[test]
fn fixture_compare_actions_match_page() {
    let page: MigrationWizardPage = load_json("page.json");
    let compare: Vec<WizardCompareAction> = load_json("compare_actions.json");
    assert_eq!(page.compare_actions, compare);
    assert!(!compare.is_empty());
}

#[test]
fn fixture_undo_actions_match_page() {
    let page: MigrationWizardPage = load_json("page.json");
    let undo: Vec<WizardUndoAction> = load_json("undo_actions.json");
    assert_eq!(page.undo_actions, undo);
    assert!(undo
        .iter()
        .any(|action| action.action_token == "restore_from_checkpoint"));
    assert!(undo
        .iter()
        .any(|action| action.action_token == "export_for_support"));
}

#[test]
fn fixture_stage_history_records_checkpoint_before_durable_writes() {
    let page: MigrationWizardPage = load_json("page.json");
    let history: Vec<WizardStageTransition> = load_json("stage_history.json");
    assert_eq!(page.stage_history, history);

    let mut seen_checkpoint = false;
    for transition in &history {
        if matches!(transition.stage, WizardStage::CheckpointReady) {
            seen_checkpoint = true;
        }
        if transition.durable_writes_authorized {
            assert!(
                seen_checkpoint,
                "stage {} authorized writes before checkpoint",
                transition.stage.as_str()
            );
        }
    }
}

#[test]
fn fixture_rollback_checkpoint_matches_page() {
    let page: MigrationWizardPage = load_json("page.json");
    let checkpoint: WizardRollbackCheckpointBinding = load_json("rollback_checkpoint.json");
    assert_eq!(page.rollback_checkpoint, checkpoint);
    assert!(checkpoint.created_before_apply);
    assert!(checkpoint.protects_every_domain);
}

#[test]
fn fixture_support_export_quotes_every_case_id() {
    let page: MigrationWizardPage = load_json("page.json");
    let export: MigrationWizardSupportExport = load_json("support_export.json");
    let expected =
        MigrationWizardSupportExport::from_page(export.support_export_id.clone(), page.clone());
    assert_eq!(export, expected);

    assert!(export.case_ids.contains(&page.wizard_session_id));
    assert!(export
        .case_ids
        .contains(&page.mapping_report.mapping_report_id));
    assert!(export
        .case_ids
        .contains(&page.rollback_checkpoint.checkpoint_ref));
    for row in &page.mapping_report.rows {
        assert!(export.case_ids.contains(&row.row_id));
    }
    for gap in &page.mapping_report.unsupported_gaps {
        assert!(export.case_ids.contains(&gap.gap_id));
    }
    for compare in &page.compare_actions {
        assert!(export.case_ids.contains(&compare.compare_action_id));
    }
    for undo in &page.undo_actions {
        assert!(export.case_ids.contains(&undo.undo_action_id));
    }
}

#[test]
fn fixture_rows_all_carry_checkpoint_ref() {
    let page: MigrationWizardPage = load_json("page.json");
    let report_rows: &[WizardMappingReportRow] = &page.mapping_report.rows;
    let checkpoint_ref = &page.rollback_checkpoint.checkpoint_ref;
    for row in report_rows {
        assert_eq!(&row.rollback_checkpoint_ref, checkpoint_ref);
    }
}
