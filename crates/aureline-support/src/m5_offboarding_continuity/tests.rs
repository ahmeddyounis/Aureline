use super::*;

use crate::m5_storage_governance::current_m5_artifact_family_storage_matrix;

fn corpus() -> OffboardingContinuityCorpus {
    current_offboarding_continuity_corpus().expect("corpus parses")
}

fn matrix() -> M5ArtifactFamilyStorageMatrix {
    current_m5_artifact_family_storage_matrix().expect("matrix parses")
}

#[test]
fn corpus_parses_every_fixture() {
    let corpus = corpus();
    assert_eq!(corpus.plans.len(), PLAN_FIXTURES.len());
}

#[test]
fn corpus_validates_against_the_safety_contract() {
    let corpus = corpus();
    let violations = corpus.validate();
    assert_eq!(violations, Vec::new(), "{violations:#?}");
}

#[test]
fn every_plan_is_metadata_safe_and_offers_the_actions() {
    let corpus = corpus();
    for entry in &corpus.plans {
        let plan = &entry.plan;
        assert!(
            plan.is_export_safe(),
            "{} must be metadata-safe",
            plan.plan_id
        );
        assert!(!plan.raw_content_exported);
        assert_eq!(
            plan.open_inspector_action_ref,
            OPEN_STORAGE_INSPECTOR_ACTION_REF
        );
        assert_eq!(
            plan.open_clear_data_review_action_ref,
            OPEN_CLEAR_DATA_REVIEW_ACTION_REF
        );
        assert!(!plan.portability_summary.trim().is_empty());
    }
}

#[test]
fn composer_matches_every_seeded_plan() {
    let matrix = matrix();
    let corpus = corpus();
    for request in seeded_offboarding_requests() {
        let composed = compose_offboarding_plan(&matrix, &request);
        let seeded = corpus
            .plan(&request.plan_id)
            .unwrap_or_else(|| panic!("seeded plan {} present", request.plan_id))
            .clone();
        assert_eq!(composed, seeded, "{}", request.plan_id);
        assert!(composed.is_valid(), "{}", request.plan_id);
    }
}

#[test]
fn protected_and_continuity_pinned_families_are_never_silently_disposed() {
    let corpus = corpus();
    for entry in &corpus.plans {
        for row in &entry.plan.disposed_rows {
            if row.protected_continuity || row.continuity_pinned {
                assert!(
                    row.reviewed_away,
                    "{} disposed a protected/continuity-pinned family without an explicit review",
                    row.row_id
                );
                assert_ne!(
                    row.export_before_delete_class,
                    ExportBeforeDeleteClass::ExportNotApplicableDisposable,
                    "{} must offer/require an export path",
                    row.row_id
                );
            }
        }
    }
}

#[test]
fn protected_classes_always_require_export_before_delete() {
    let corpus = corpus();
    for entry in &corpus.plans {
        for row in entry.plan.all_rows() {
            if row.is_protected_class() {
                assert_eq!(
                    row.export_before_delete_class,
                    ExportBeforeDeleteClass::ExportRequiredBeforeDelete,
                    "{} must require export-before-delete",
                    row.row_id
                );
                assert!(row.export_action_ref.is_some(), "{}", row.row_id);
            }
        }
    }
}

#[test]
fn continuity_warnings_track_storage_class_and_pins() {
    let corpus = corpus();
    for entry in &corpus.plans {
        for row in entry.plan.all_rows() {
            let expected =
                continuity_warnings_for(row.storage_class_id, row.pin_source_classes.as_slice());
            assert_eq!(row.continuity_warnings, expected, "{}", row.row_id);
        }
        // The plan-level warnings are exactly the active losses across disposals.
        let active = entry.plan.compute_active_continuity_warnings();
        assert_eq!(
            entry.plan.continuity_warnings, active,
            "{}",
            entry.plan.plan_id
        );
    }
}

#[test]
fn portability_honesty_never_overpromises_when_only_caches_removed() {
    let corpus = corpus();
    // The offline-pack offboarding only removed rebuildable packs/caches; even
    // though continuity was broken, no durable state was exported away.
    let plan = corpus
        .plan("offboarding_continuity.offline_bundle_reviewed_away_continuity_warned.v1")
        .expect("plan present");
    assert_eq!(
        plan.portability_honesty_class,
        PortabilityHonestyClass::CachesOnlyRemovedDurableRetained
    );
    assert!(!plan.continuity_warnings.is_empty());
    assert!(plan
        .disposed_rows
        .iter()
        .all(|row| !row.portability_class.is_durable()));

    // The workspace wipe reviewed durable evidence + recovery away, exported first.
    let wipe = corpus
        .plan("offboarding_continuity.workspace_wipe_reviewed_away_export_first.v1")
        .expect("plan present");
    assert_eq!(
        wipe.portability_honesty_class,
        PortabilityHonestyClass::DurableStateExportedBeforeRemoval
    );
    for row in &wipe.disposed_rows {
        if row.portability_class.is_durable() {
            assert!(row.reviewed_away);
            assert_eq!(
                row.export_before_delete_class,
                ExportBeforeDeleteClass::ExportRequiredBeforeDelete
            );
        }
    }
}

#[test]
fn support_export_is_metadata_safe_and_complete() {
    let corpus = corpus();
    let export = corpus.support_export(
        "support_export.m5_offboarding_continuity.v1",
        "2026-06-14T00:00:00Z",
    );
    assert!(export.is_export_safe());
    assert_eq!(export.plan_count as usize, corpus.plans.len());
    assert_eq!(export.plans.len(), corpus.plans.len());
    assert!(!export.raw_content_exported);
    assert!(export.protected_retained_family_count >= 1);
    let json = serde_json::to_string(&export).expect("serialize");
    let back: OffboardingContinuitySupportExport =
        serde_json::from_str(&json).expect("deserialize");
    assert_eq!(export, back);
}

#[test]
fn support_export_matches_checked_in_golden() {
    const GOLDEN: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/storage/m5_offboarding_continuity/support_export.golden.json"
    ));
    let corpus = corpus();
    let export = corpus.support_export(
        "support_export.m5_offboarding_continuity.v1",
        "2026-06-14T00:00:00Z",
    );
    let golden: OffboardingContinuitySupportExport =
        serde_json::from_str(GOLDEN).expect("golden parses");
    assert_eq!(
        export, golden,
        "projected support export drifted from the checked-in golden; \
         regenerate with `cargo run -p aureline-support \
         --example dump_m5_offboarding_continuity_support_export`"
    );
}

// -- Matrix-backed composer (the first real consumer) ----------------------

fn ws_alpha() -> WorkspaceScope {
    WorkspaceScope {
        scope_ref: "ws.alpha".to_owned(),
        label: "Project Alpha".to_owned(),
    }
}

#[test]
fn composer_retains_offline_pin_requested_without_review() {
    let matrix = matrix();
    let request = OffboardingContinuityRequest {
        plan_id: "compose.offline_no_review".to_owned(),
        emitted_at: "2026-06-14T00:00:00Z".to_owned(),
        title: "Compose offline no review".to_owned(),
        offboarding_flow_class: OffboardingFlowClass::AccountOffboarding,
        initiator_class: InitiatorClass::LocalUser,
        workspaces: vec![ws_alpha()],
        selections: vec![OffboardingFamilySelection {
            family_id: ArtifactFamilyId::DocsPack,
            workspace_scope_ref: "ws.alpha".to_owned(),
            workspace_label: "Project Alpha".to_owned(),
            total_bytes: 1000,
            // Requested for removal but NOT reviewed away → must stay retained.
            requested_disposal: true,
            reviewed_away: false,
            pin_source_classes: vec![PinSourceClass::OfflineBundleRef],
        }],
        note: String::new(),
    };
    let plan = compose_offboarding_plan(&matrix, &request);
    assert_eq!(plan.validate(), Vec::new(), "{:#?}", plan.validate());
    assert!(plan.disposed_rows.is_empty());
    let row = &plan.retained_rows[0];
    assert_eq!(
        row.disposition,
        OffboardingDispositionClass::RetainedForOfflineContinuity
    );
    assert!(!row.reviewed_away);
    assert_eq!(row.retained_bytes, 1000);
    assert!(!plan.guardrail_notices.is_empty());
    assert_eq!(
        plan.protected_families_retained,
        vec![ArtifactFamilyId::DocsPack]
    );
}

#[test]
fn composer_retains_user_owned_recovery_requested_without_review() {
    let matrix = matrix();
    let request = OffboardingContinuityRequest {
        plan_id: "compose.recovery_no_review".to_owned(),
        emitted_at: "2026-06-14T00:00:00Z".to_owned(),
        title: "Compose recovery no review".to_owned(),
        offboarding_flow_class: OffboardingFlowClass::DeviceReset,
        initiator_class: InitiatorClass::LocalUser,
        workspaces: vec![ws_alpha()],
        selections: vec![OffboardingFamilySelection {
            family_id: ArtifactFamilyId::UserOwnedRecoveryState,
            workspace_scope_ref: "ws.alpha".to_owned(),
            workspace_label: "Project Alpha".to_owned(),
            total_bytes: 4000,
            requested_disposal: true,
            reviewed_away: false,
            pin_source_classes: vec![PinSourceClass::ExplicitUserPin],
        }],
        note: String::new(),
    };
    let plan = compose_offboarding_plan(&matrix, &request);
    assert_eq!(plan.validate(), Vec::new(), "{:#?}", plan.validate());
    assert!(plan.disposed_rows.is_empty());
    let row = &plan.retained_rows[0];
    assert_eq!(
        row.disposition,
        OffboardingDispositionClass::RetainedProtectedContinuity
    );
    assert!(row.protected_continuity);
    assert_eq!(
        row.export_before_delete_class,
        ExportBeforeDeleteClass::ExportRequiredBeforeDelete
    );
    assert_eq!(
        plan.portability_honesty_class,
        PortabilityHonestyClass::NothingDisposedAllRetained
    );
}

#[test]
fn composer_exports_durable_state_when_reviewed_away() {
    let matrix = matrix();
    let request = OffboardingContinuityRequest {
        plan_id: "compose.evidence_reviewed_away".to_owned(),
        emitted_at: "2026-06-14T00:00:00Z".to_owned(),
        title: "Compose evidence reviewed away".to_owned(),
        offboarding_flow_class: OffboardingFlowClass::WorkspaceWipe,
        initiator_class: InitiatorClass::LocalUser,
        workspaces: vec![ws_alpha()],
        selections: vec![OffboardingFamilySelection {
            family_id: ArtifactFamilyId::ReviewIncidentEvidence,
            workspace_scope_ref: "ws.alpha".to_owned(),
            workspace_label: "Project Alpha".to_owned(),
            total_bytes: 9000,
            requested_disposal: true,
            reviewed_away: true,
            pin_source_classes: vec![PinSourceClass::ReviewPackRef],
        }],
        note: String::new(),
    };
    let plan = compose_offboarding_plan(&matrix, &request);
    assert_eq!(plan.validate(), Vec::new(), "{:#?}", plan.validate());
    let row = &plan.disposed_rows[0];
    assert_eq!(
        row.disposition,
        OffboardingDispositionClass::ExportThenDispose
    );
    assert!(row.reviewed_away);
    assert_eq!(
        row.export_before_delete_class,
        ExportBeforeDeleteClass::ExportRequiredBeforeDelete
    );
    assert_eq!(
        plan.portability_honesty_class,
        PortabilityHonestyClass::DurableStateExportedBeforeRemoval
    );
    assert!(plan
        .continuity_warnings
        .contains(&ContinuityWarningClass::EvidenceContinuityLost));
}

#[test]
fn composer_drops_pins_inadmissible_under_the_matrix() {
    let matrix = matrix();
    // A generated preview admits no pins; an offered offline pin must be ignored.
    let request = OffboardingContinuityRequest {
        plan_id: "compose.inadmissible_pin".to_owned(),
        emitted_at: "2026-06-14T00:00:00Z".to_owned(),
        title: "Compose inadmissible pin".to_owned(),
        offboarding_flow_class: OffboardingFlowClass::DeviceReset,
        initiator_class: InitiatorClass::LocalUser,
        workspaces: vec![ws_alpha()],
        selections: vec![OffboardingFamilySelection {
            family_id: ArtifactFamilyId::GeneratedPreview,
            workspace_scope_ref: "ws.alpha".to_owned(),
            workspace_label: "Project Alpha".to_owned(),
            total_bytes: 1000,
            requested_disposal: true,
            reviewed_away: false,
            pin_source_classes: vec![PinSourceClass::OfflineBundleRef],
        }],
        note: String::new(),
    };
    let plan = compose_offboarding_plan(&matrix, &request);
    assert_eq!(plan.validate(), Vec::new(), "{:#?}", plan.validate());
    let row = &plan.disposed_rows[0];
    assert!(row.pin_source_classes.is_empty());
    assert!(!row.continuity_pinned);
    assert_eq!(
        row.disposition,
        OffboardingDispositionClass::DisposeRebuildable
    );
}

// -- Mutation / failure drills ---------------------------------------------

#[test]
fn mutating_a_protected_row_into_the_disposed_bucket_is_rejected() {
    let mut corpus = corpus();
    let entry = corpus
        .plans
        .iter_mut()
        .find(|e| {
            e.plan.plan_id == "offboarding_continuity.account_offboarding_durable_retained.v1"
        })
        .expect("plan present");
    // Move the retained recovery row into the disposed bucket without review.
    let pos = entry
        .plan
        .retained_rows
        .iter()
        .position(|r| r.storage_class_id == StorageClassId::UserOwnedRecoveryState)
        .expect("recovery row present");
    let row = entry.plan.retained_rows.remove(pos);
    entry.plan.disposed_rows.push(row);
    let violations = corpus.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "plan.row_bucket_mismatch"),
        "expected a row_bucket_mismatch violation, got {violations:#?}"
    );
}

#[test]
fn mutating_a_disposed_protected_row_to_drop_review_is_rejected() {
    let mut corpus = corpus();
    let entry = corpus
        .plans
        .iter_mut()
        .find(|e| {
            e.plan.plan_id == "offboarding_continuity.workspace_wipe_reviewed_away_export_first.v1"
        })
        .expect("plan present");
    let row = entry
        .plan
        .disposed_rows
        .iter_mut()
        .find(|r| r.storage_class_id == StorageClassId::UserOwnedRecoveryState)
        .expect("recovery row present");
    row.reviewed_away = false;
    let violations = corpus.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "row.export_then_dispose_unreviewed"),
        "expected an export_then_dispose_unreviewed violation, got {violations:#?}"
    );
}

#[test]
fn mutating_the_portability_honesty_class_is_rejected() {
    let mut corpus = corpus();
    let entry = corpus
        .plans
        .iter_mut()
        .find(|e| e.plan.plan_id == "offboarding_continuity.device_reset_caches_only.v1")
        .expect("plan present");
    entry.plan.portability_honesty_class =
        PortabilityHonestyClass::DurableStateExportedBeforeRemoval;
    let violations = corpus.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "plan.portability_honesty"),
        "expected a portability_honesty violation, got {violations:#?}"
    );
}

#[test]
fn mutating_a_row_to_hide_continuity_note_is_rejected() {
    let mut corpus = corpus();
    let entry = corpus
        .plans
        .iter_mut()
        .find(|e| e.plan.plan_id == "offboarding_continuity.device_reset_caches_only.v1")
        .expect("plan present");
    entry.plan.disposed_rows[0].continuity_note = "   ".to_owned();
    let violations = corpus.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "row.continuity_note"),
        "expected a continuity_note violation, got {violations:#?}"
    );
}

#[test]
fn mutating_the_disposed_total_is_rejected() {
    let mut corpus = corpus();
    let entry = corpus
        .plans
        .iter_mut()
        .find(|e| e.plan.plan_id == "offboarding_continuity.device_reset_caches_only.v1")
        .expect("plan present");
    entry.plan.total_disposed_bytes += 1;
    let violations = corpus.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "plan.disposed_total"),
        "expected a disposed_total violation, got {violations:#?}"
    );
}

#[test]
fn mutating_a_rows_portability_class_is_rejected() {
    let mut corpus = corpus();
    let entry = corpus
        .plans
        .iter_mut()
        .find(|e| e.plan.plan_id == "offboarding_continuity.device_reset_caches_only.v1")
        .expect("plan present");
    entry.plan.disposed_rows[0].portability_class = PortabilityClass::ExportableDurableState;
    let violations = corpus.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "row.portability_class"),
        "expected a portability_class violation, got {violations:#?}"
    );
}
