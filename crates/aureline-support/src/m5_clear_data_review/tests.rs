use super::*;

use crate::m5_storage_governance::current_m5_artifact_family_storage_matrix;

fn corpus() -> ClearDataReviewCorpus {
    current_clear_data_review_corpus().expect("corpus parses")
}

fn matrix() -> M5ArtifactFamilyStorageMatrix {
    current_m5_artifact_family_storage_matrix().expect("matrix parses")
}

#[test]
fn corpus_parses_every_fixture() {
    let corpus = corpus();
    assert_eq!(corpus.sheets.len(), SHEET_FIXTURES.len());
}

#[test]
fn corpus_validates_against_the_safety_contract() {
    let corpus = corpus();
    let violations = corpus.validate();
    assert_eq!(violations, Vec::new(), "{violations:#?}");
}

#[test]
fn every_sheet_is_metadata_safe() {
    let corpus = corpus();
    for entry in &corpus.sheets {
        assert!(
            entry.sheet.is_export_safe(),
            "{} must be metadata-safe",
            entry.sheet.sheet_id
        );
        assert!(!entry.sheet.raw_content_exported);
    }
}

#[test]
fn protected_rows_never_admit_a_generic_clear() {
    let corpus = corpus();
    for entry in &corpus.sheets {
        for row in entry.sheet.all_rows() {
            if !row.is_protected_class() {
                continue;
            }
            assert!(
                !matches!(
                    row.clear_data_action,
                    ClearDataActionClass::GenericClearInBulk
                        | ClearDataActionClass::GenericClearExcludingPins
                        | ClearDataActionClass::ClassSelectiveClear
                ),
                "{} must not admit a generic clear",
                row.row_id
            );
            assert_eq!(
                row.export_before_delete_class,
                ExportBeforeDeleteClass::ExportRequiredBeforeDelete,
                "{} must require export-before-delete",
                row.row_id
            );
            assert!(row.reversibility_class.is_irreversible());
        }
    }
}

#[test]
fn pressure_sheets_disclose_order_and_never_select_user_owned() {
    let corpus = corpus();
    for entry in &corpus.sheets {
        let sheet = &entry.sheet;
        if !sheet.cleanup_trigger_class.is_pressure_trigger() {
            continue;
        }
        assert!(sheet.low_disk_order_disclosed, "{}", sheet.sheet_id);
        assert_eq!(
            sheet.low_disk_eviction_order.len(),
            M5ArtifactFamilyStorageMatrix::required_families().len()
        );
        for row in &sheet.selected_rows {
            assert_ne!(
                row.storage_class_id,
                StorageClassId::UserOwnedRecoveryState,
                "disk/quota pressure must never auto-select user-owned recovery state"
            );
        }
    }
}

#[test]
fn offboarding_reset_surfaces_every_protected_family() {
    let corpus = corpus();
    let sheet = corpus
        .sheet("clear_data_review.offboarding_reset_full_export_first.v1")
        .expect("offboarding sheet present");
    let covered: std::collections::BTreeSet<ArtifactFamilyId> =
        sheet.all_rows().map(|r| r.family_id).collect();
    for family in PROTECTED_FAMILIES {
        assert!(
            covered.contains(family),
            "offboarding must surface {}",
            family.as_str()
        );
    }
}

#[test]
fn blocked_sheet_carries_a_guardrail_notice() {
    let corpus = corpus();
    let sheet = corpus
        .sheet("clear_data_review.blocked_quota_pressure_refuses_user_owned.v1")
        .expect("blocked sheet present");
    assert_eq!(sheet.consent_state, ConsentStateClass::BlockedByGuardrail);
    assert!(!sheet.guardrail_notices.is_empty());
    assert_eq!(sheet.total_selected_reclaimable_bytes, 0);
    assert!(sheet.selected_rows.is_empty());
}

#[test]
fn support_export_is_metadata_safe_and_complete() {
    let corpus = corpus();
    let export = corpus.support_export(
        "support_export.m5_clear_data_review.v1",
        "2026-06-14T00:00:00Z",
    );
    assert!(export.is_export_safe());
    assert_eq!(export.sheet_count as usize, corpus.sheets.len());
    assert_eq!(export.sheets.len(), corpus.sheets.len());
    assert!(!export.raw_content_exported);
    assert!(export.protected_preserved_row_count >= 1);
    let json = serde_json::to_string(&export).expect("serialize");
    let back: ClearDataReviewSupportExport = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(export, back);
}

#[test]
fn support_export_matches_checked_in_golden() {
    const GOLDEN: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/storage/m5_clear_data_review/support_export.golden.json"
    ));
    let corpus = corpus();
    let export = corpus.support_export(
        "support_export.m5_clear_data_review.v1",
        "2026-06-14T00:00:00Z",
    );
    let golden: ClearDataReviewSupportExport = serde_json::from_str(GOLDEN).expect("golden parses");
    assert_eq!(
        export, golden,
        "projected support export drifted from the checked-in golden; \
         regenerate with `cargo run -p aureline-support \
         --example dump_m5_clear_data_review_support_export`"
    );
}

// -- Matrix-backed composer (the first real consumer) ----------------------

fn workspace() -> WorkspaceScope {
    WorkspaceScope {
        scope_ref: "ws.alpha".to_owned(),
        label: "Project Alpha".to_owned(),
    }
}

#[test]
fn composer_excludes_protected_families_unless_explicit() {
    let matrix = matrix();
    let request = ClearDataReviewRequest {
        sheet_id: "compose.user".to_owned(),
        emitted_at: "2026-06-14T00:00:00Z".to_owned(),
        title: "Compose user cleanup".to_owned(),
        flow: CleanupFlowClass::UserDrivenCleanup,
        trigger: CleanupTriggerClass::ManualUserRequest,
        initiator: InitiatorClass::LocalUser,
        workspaces: vec![workspace()],
        selections: vec![
            ClearDataSelection {
                family_id: ArtifactFamilyId::NotebookOutput,
                workspace_scope_ref: "ws.alpha".to_owned(),
                workspace_label: "Project Alpha".to_owned(),
                explicit: true,
                total_bytes: 5000,
                preserved_pinned_bytes: 1000,
                preserved_pin_source_classes: vec![PinSourceClass::ExplicitUserPin],
            },
            // Protected, not explicitly selected -> must be excluded.
            ClearDataSelection {
                family_id: ArtifactFamilyId::UserOwnedRecoveryState,
                workspace_scope_ref: "ws.alpha".to_owned(),
                workspace_label: "Project Alpha".to_owned(),
                explicit: false,
                total_bytes: 8000,
                preserved_pinned_bytes: 0,
                preserved_pin_source_classes: vec![],
            },
        ],
        note: String::new(),
    };
    let sheet = compose_review_sheet(&matrix, &request);
    assert_eq!(sheet.validate(), Vec::new(), "{:#?}", sheet.validate());

    // The notebook output is selected and frees its unpinned bytes.
    let notebook = sheet
        .selected_rows
        .iter()
        .find(|r| r.family_id == ArtifactFamilyId::NotebookOutput)
        .expect("notebook selected");
    assert_eq!(notebook.freed_bytes, 4000);
    assert_eq!(notebook.preserved_bytes, 1000);

    // The user-owned recovery state is excluded, not selected.
    assert!(sheet
        .selected_rows
        .iter()
        .all(|r| r.storage_class_id != StorageClassId::UserOwnedRecoveryState));
    let recovery = sheet
        .retained_rows
        .iter()
        .find(|r| r.storage_class_id == StorageClassId::UserOwnedRecoveryState)
        .expect("recovery retained");
    assert_eq!(
        recovery.retention_reason,
        Some(RetentionReasonClass::ProtectedRecoveryStateExcludedByDefault)
    );
    assert_eq!(
        recovery.clear_data_action,
        ClearDataActionClass::ExplicitPerItemReviewRequired
    );
    assert_eq!(sheet.total_selected_reclaimable_bytes, 4000);
}

#[test]
fn composer_refuses_user_owned_under_pressure_even_when_requested() {
    let matrix = matrix();
    let request = ClearDataReviewRequest {
        sheet_id: "compose.pressure".to_owned(),
        emitted_at: "2026-06-14T00:00:00Z".to_owned(),
        title: "Compose under pressure".to_owned(),
        flow: CleanupFlowClass::UserDrivenCleanup,
        trigger: CleanupTriggerClass::ManagedQuotaPressure,
        initiator: InitiatorClass::AdminOrTenantPolicy,
        workspaces: vec![workspace()],
        selections: vec![ClearDataSelection {
            // Even with explicit=true, pressure must never auto-dispose it.
            family_id: ArtifactFamilyId::UserOwnedRecoveryState,
            workspace_scope_ref: "ws.alpha".to_owned(),
            workspace_label: "Project Alpha".to_owned(),
            explicit: true,
            total_bytes: 12000,
            preserved_pinned_bytes: 0,
            preserved_pin_source_classes: vec![],
        }],
        note: String::new(),
    };
    let sheet = compose_review_sheet(&matrix, &request);
    assert_eq!(sheet.validate(), Vec::new(), "{:#?}", sheet.validate());
    assert!(sheet.selected_rows.is_empty());
    assert!(!sheet.guardrail_notices.is_empty());
    assert_eq!(sheet.consent_state, ConsentStateClass::BlockedByGuardrail);
    assert!(sheet.low_disk_order_disclosed);
}

#[test]
fn composed_actions_stay_admissible_under_the_matrix() {
    let matrix = matrix();
    for family in M5ArtifactFamilyStorageMatrix::required_families() {
        let request = ClearDataReviewRequest {
            sheet_id: format!("compose.{}", family.as_str()),
            emitted_at: "2026-06-14T00:00:00Z".to_owned(),
            title: "Compose one family".to_owned(),
            flow: CleanupFlowClass::UserDrivenCleanup,
            trigger: CleanupTriggerClass::ManualUserRequest,
            initiator: InitiatorClass::LocalUser,
            workspaces: vec![workspace()],
            selections: vec![ClearDataSelection {
                family_id: *family,
                workspace_scope_ref: "ws.alpha".to_owned(),
                workspace_label: "Project Alpha".to_owned(),
                explicit: true,
                total_bytes: 1000,
                preserved_pinned_bytes: 0,
                preserved_pin_source_classes: vec![],
            }],
            note: String::new(),
        };
        let sheet = compose_review_sheet(&matrix, &request);
        assert_eq!(
            sheet.validate(),
            Vec::new(),
            "{}: {:#?}",
            family.as_str(),
            sheet.validate()
        );
        let matrix_row = matrix.family(*family).expect("row");
        for row in sheet.all_rows() {
            assert!(
                matrix_row
                    .allowed_clear_data_actions
                    .contains(&row.clear_data_action),
                "{} composed an action outside the matrix",
                family.as_str()
            );
        }
    }
}

// -- Mutation / failure drills ---------------------------------------------

#[test]
fn mutating_a_protected_row_to_generic_clear_is_rejected() {
    let mut corpus = corpus();
    let entry = corpus
        .sheets
        .iter_mut()
        .find(|e| e.sheet.sheet_id == "clear_data_review.user_cleanup_rebuildable_caches.v1")
        .expect("sheet present");
    let row = entry
        .sheet
        .retained_rows
        .iter_mut()
        .find(|r| r.storage_class_id == StorageClassId::UserOwnedRecoveryState)
        .expect("recovery row present");
    row.clear_data_action = ClearDataActionClass::GenericClearInBulk;
    let violations = corpus.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "row.protected_generic_clear"),
        "expected a protected_generic_clear violation, got {violations:#?}"
    );
}

#[test]
fn mutating_a_pressure_sheet_to_select_user_owned_is_rejected() {
    let mut corpus = corpus();
    let entry = corpus
        .sheets
        .iter_mut()
        .find(|e| e.sheet.sheet_id == "clear_data_review.low_disk_pressure_disposable_first.v1")
        .expect("sheet present");
    // Move the excluded recovery row into the selected bucket.
    let mut row = entry.sheet.retained_rows.remove(0);
    row.selection_state = SelectionStateClass::SelectedForCleanup;
    entry.sheet.selected_rows.push(row);
    let violations = corpus.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "sheet.pressure_user_owned_selected"),
        "expected a pressure_user_owned_selected violation, got {violations:#?}"
    );
}

#[test]
fn mutating_an_offboarding_sheet_to_drop_a_protected_family_is_rejected() {
    let mut corpus = corpus();
    let entry = corpus
        .sheets
        .iter_mut()
        .find(|e| e.sheet.sheet_id == "clear_data_review.offboarding_reset_full_export_first.v1")
        .expect("sheet present");
    entry
        .sheet
        .retained_rows
        .retain(|r| r.family_id != ArtifactFamilyId::ProfilerTrace);
    let violations = corpus.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "sheet.offboarding_protected_uncovered"),
        "expected an offboarding_protected_uncovered violation, got {violations:#?}"
    );
}

#[test]
fn mutating_a_row_to_hide_rebuild_cost_is_rejected() {
    let mut corpus = corpus();
    let entry = corpus
        .sheets
        .iter_mut()
        .find(|e| e.sheet.sheet_id == "clear_data_review.user_cleanup_rebuildable_caches.v1")
        .expect("sheet present");
    entry.sheet.selected_rows[0].rebuild_disclosure = "   ".to_owned();
    let violations = corpus.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "row.rebuild_disclosure"),
        "expected a rebuild_disclosure violation, got {violations:#?}"
    );
}

#[test]
fn mutating_the_reclaimable_total_is_rejected() {
    let mut corpus = corpus();
    let entry = corpus
        .sheets
        .iter_mut()
        .find(|e| e.sheet.sheet_id == "clear_data_review.user_cleanup_rebuildable_caches.v1")
        .expect("sheet present");
    entry.sheet.total_selected_reclaimable_bytes += 1;
    let violations = corpus.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "sheet.reclaimable_total"),
        "expected a reclaimable_total violation, got {violations:#?}"
    );
}
