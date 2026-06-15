use super::*;

fn corpus() -> CacheRepairPlanCorpus {
    current_cache_repair_plan_corpus().expect("corpus parses")
}

fn profiles() -> RuntimeStorageClassProfiles {
    current_runtime_profiles().expect("runtime profiles parse")
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
fn every_plan_is_export_safe() {
    let corpus = corpus();
    for entry in &corpus.plans {
        assert!(
            entry.plan.is_export_safe(),
            "{} must be export-safe",
            entry.plan.plan_id
        );
    }
}

#[test]
fn no_plan_offers_a_factory_reset_or_resets_everything() {
    let corpus = corpus();
    for entry in &corpus.plans {
        let plan = &entry.plan;
        assert!(!plan.factory_reset_offered, "{}", plan.plan_id);
        assert!(plan.reset_everything_avoided, "{}", plan.plan_id);
        assert!(plan.narrowest_sufficient_scope, "{}", plan.plan_id);
        assert!(!plan.scope_class.is_global(), "{}", plan.plan_id);
        assert!(
            !plan.fallback_action.is_reset_everything(),
            "{}",
            plan.plan_id
        );
        assert!(plan.repair_action.is_targeted(), "{}", plan.plan_id);
    }
}

#[test]
fn protected_classes_always_quarantine_the_suspect_copy() {
    let corpus = corpus();
    for entry in &corpus.plans {
        let plan = &entry.plan;
        if matches!(
            plan.storage_class_id,
            StorageClassId::EvidenceSupportCache | StorageClassId::UserOwnedRecoveryState
        ) {
            assert!(
                plan.quarantine_disposition.preserves_suspect_copy(),
                "{} must quarantine the suspect copy",
                plan.plan_id
            );
            assert!(plan.quarantine_ref.is_some(), "{}", plan.plan_id);
        }
    }
}

#[test]
fn user_owned_recovery_repair_preserves_user_owned_data_and_repairs_in_place() {
    let corpus = corpus();
    let plan = corpus
        .plan("cache_repair.recovery_state_torn.v1")
        .expect("recovery plan present");
    assert_eq!(
        plan.storage_class_id,
        StorageClassId::UserOwnedRecoveryState
    );
    assert!(plan.preserves_user_owned_data);
    assert_eq!(
        plan.repair_action,
        RepairActionClass::RepairInPlaceFromCheckpoint
    );
    assert!(!plan.repair_action.clears_suspect_data());
    assert_eq!(
        plan.quarantine_disposition,
        QuarantineDispositionClass::QuarantinedUserOwnedDataPreserved
    );
}

#[test]
fn evidence_repair_quarantines_for_review_and_never_clears() {
    let corpus = corpus();
    let plan = corpus
        .plan("cache_repair.evidence_trace_corrupt.v1")
        .expect("evidence plan present");
    assert_eq!(plan.storage_class_id, StorageClassId::EvidenceSupportCache);
    assert!(plan.preserves_forensics_value);
    assert_eq!(
        plan.repair_action,
        RepairActionClass::QuarantineThenManualReview
    );
    assert!(!plan.repair_action.clears_suspect_data());
    assert_eq!(
        plan.quarantine_disposition,
        QuarantineDispositionClass::QuarantinedPendingExport
    );
}

#[test]
fn surface_labels_stay_active_until_repair_completes() {
    let corpus = corpus();
    for entry in &corpus.plans {
        let plan = &entry.plan;
        let expected_active = !plan.repair_state.is_complete();
        for label in &plan.affected_surface_labels {
            assert_eq!(
                label.label_active, expected_active,
                "{} surface {} active mismatch",
                plan.plan_id, label.surface_ref
            );
            assert!(label.clears_on_repair_complete, "{}", plan.plan_id);
            assert_eq!(label.repair_label, plan.repair_label, "{}", plan.plan_id);
            assert_eq!(
                label.posture,
                label.repair_label.posture(),
                "{}",
                plan.plan_id
            );
        }
        // No completed plan exists in the seeded corpus, so every plan keeps its
        // labels active and not healthy.
        assert!(!plan.repair_label.is_healthy(), "{}", plan.plan_id);
    }
}

#[test]
fn disposable_classes_need_no_quarantine_copy() {
    let corpus = corpus();
    for plan_id in [
        "cache_repair.knowledge_cache_corrupt_index.v1",
        "cache_repair.artifact_pack_checksum_mismatch.v1",
        "cache_repair.generated_preview_torn.v1",
    ] {
        let plan = corpus.plan(plan_id).expect("disposable plan present");
        assert_eq!(
            plan.quarantine_disposition,
            QuarantineDispositionClass::NoQuarantineDisposableOnly,
            "{plan_id}"
        );
        assert!(plan.quarantine_ref.is_none(), "{plan_id}");
        assert!(!plan.preserves_user_owned_data, "{plan_id}");
        assert!(!plan.preserves_forensics_value, "{plan_id}");
    }
}

#[test]
fn knowledge_cache_corrupt_index_is_reindexed_and_propagated() {
    let corpus = corpus();
    let plan = corpus
        .plan("cache_repair.knowledge_cache_corrupt_index.v1")
        .expect("knowledge plan present");
    assert_eq!(plan.repair_action, RepairActionClass::RebuildFromSource);
    assert_eq!(plan.repair_label, RepairLabelClass::ReindexNeeded);
    assert_eq!(plan.posture, StoragePostureClass::RebuildPending);
    assert!(plan.surface_label("surface.search").is_some());
    assert!(plan.surface_label("surface.code_graph").is_some());
}

#[test]
fn failed_repair_offers_a_targeted_fallback_not_a_reset() {
    let corpus = corpus();
    let plan = corpus
        .plan("cache_repair.prebuild_missing_backing.v1")
        .expect("prebuild plan present");
    assert!(plan.repair_state.is_failed());
    assert_eq!(
        plan.fallback_action,
        FallbackActionClass::RetryTargetedRepair
    );
    assert!(!plan.fallback_action.is_reset_everything());
    assert!(!plan.factory_reset_offered);
}

#[test]
fn composer_reproduces_every_seeded_plan() {
    let profiles = profiles();
    let corpus = corpus();
    for signal in seeded_repair_signals() {
        let composed = compose_plan(&profiles, &signal);
        let seeded = corpus
            .plan(&signal.plan_id)
            .unwrap_or_else(|| panic!("seeded plan {} present", signal.plan_id))
            .clone();
        assert_eq!(composed, seeded, "composer drifted for {}", signal.plan_id);
    }
}

#[test]
fn composer_never_offers_a_factory_reset() {
    let profiles = profiles();
    for signal in seeded_repair_signals() {
        let plan = compose_plan(&profiles, &signal);
        assert!(!plan.factory_reset_offered, "{}", plan.plan_id);
        assert!(plan.reset_everything_avoided, "{}", plan.plan_id);
        assert!(plan.is_export_safe(), "{}", plan.plan_id);
    }
}

#[test]
fn composer_quarantines_before_clearing_user_owned_data() {
    let profiles = profiles();
    // A knowledge cache that happens to hold user-owned data must quarantine the
    // suspect copy before the rebuild clears it.
    let signal = RepairSignal {
        plan_id: "composed.knowledge_with_user_data".to_owned(),
        emitted_at: "2026-06-14T00:00:00Z".to_owned(),
        storage_class_id: StorageClassId::KnowledgeCache,
        scope_class: RepairScopeClass::SingleClassSingleWorkspace,
        workspace_ref: Some("ws.alpha".to_owned()),
        workspace_label: Some("Project Alpha".to_owned()),
        fault_class: FaultClass::CorruptIndex,
        repair_state: RepairStateClass::RepairInProgress,
        holds_user_owned_data: true,
        holds_forensics_value: false,
        quarantine_ref: None,
        affected_surfaces: vec![AffectedSurfaceInput {
            surface_ref: "surface.search".to_owned(),
            surface_label: "Search".to_owned(),
            detail: "Reindexing.".to_owned(),
        }],
    };
    let plan = compose_plan(&profiles, &signal);
    assert!(plan.preserves_user_owned_data);
    assert_eq!(plan.repair_action, RepairActionClass::QuarantineThenRebuild);
    assert!(plan.repair_action.clears_suspect_data());
    assert!(plan.quarantine_ref.is_some());
    assert_eq!(
        plan.quarantine_disposition,
        QuarantineDispositionClass::QuarantinedUserOwnedDataPreserved
    );
    assert!(plan.is_export_safe());
}

#[test]
fn validator_rejects_an_offered_factory_reset() {
    let corpus = corpus();
    let mut plan = corpus
        .plan("cache_repair.generated_preview_torn.v1")
        .expect("plan present")
        .clone();
    plan.factory_reset_offered = true;
    let mut violations = Vec::new();
    plan.validate_into(&mut violations, "negative");
    assert!(violations
        .iter()
        .any(|violation| violation.check_id == "plan.factory_reset_offered"));
}

#[test]
fn validator_rejects_clearing_a_label_before_repair_completes() {
    let corpus = corpus();
    let mut plan = corpus
        .plan("cache_repair.knowledge_cache_corrupt_index.v1")
        .expect("plan present")
        .clone();
    // The repair is still in progress, but a surface dropped its stale label.
    plan.affected_surface_labels[0].label_active = false;
    let mut violations = Vec::new();
    plan.validate_into(&mut violations, "negative");
    assert!(violations
        .iter()
        .any(|violation| violation.check_id == "plan.label.active_mismatch"));
}

#[test]
fn validator_rejects_a_protected_class_without_quarantine() {
    let corpus = corpus();
    let mut plan = corpus
        .plan("cache_repair.evidence_trace_corrupt.v1")
        .expect("plan present")
        .clone();
    plan.quarantine_disposition = QuarantineDispositionClass::NoQuarantineDisposableOnly;
    plan.quarantine_ref = None;
    plan.preserves_forensics_value = false;
    let mut violations = Vec::new();
    plan.validate_into(&mut violations, "negative");
    assert!(violations
        .iter()
        .any(|violation| violation.check_id == "plan.quarantine.protected_requires_copy"));
}

#[test]
fn support_export_matches_checked_in_golden() {
    let corpus = corpus();
    let export = corpus.support_export("support_export.m5_cache_repair.v1", "2026-06-14T00:00:00Z");
    const GOLDEN: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/storage/m5_cache_repair/support_export.golden.json"
    ));
    let golden: CacheRepairSupportExport = serde_json::from_str(GOLDEN).expect("golden parses");
    assert_eq!(
        export, golden,
        "projected support export drifted from the checked-in golden; \
         regenerate with `cargo run -p aureline-support --example \
         dump_m5_cache_repair_support_export`"
    );
    assert!(export.is_export_safe());
}

#[test]
fn support_export_is_metadata_safe_and_offers_no_reset() {
    let corpus = corpus();
    let export = corpus.support_export("envelope.test", "2026-06-14T00:00:00Z");
    assert!(!export.raw_content_exported);
    assert_eq!(export.redaction_class, METADATA_SAFE_DEFAULT);
    assert_eq!(export.factory_reset_offered_count, 0);
    assert_eq!(export.plan_count, corpus.plans.len() as u32);
    assert_eq!(export.failed_count, 1);
}
