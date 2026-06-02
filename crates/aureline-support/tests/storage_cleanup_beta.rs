//! Protected beta-baseline for the storage inspector, class-selective
//! clear-data review, and cleanup receipt corpus.
//!
//! These tests pin the storage-class registry, the seeded scenarios, the
//! protected-class exclusion contract, the low-disk ordering contract, and
//! the metadata-safe support-export projection so the chrome storage
//! inspector and support exports cannot relabel ordinary cleanup as a
//! destructive umbrella.

use std::path::{Path, PathBuf};

use aureline_support::storage_inspector::{
    current_storage_class_registry, current_storage_cleanup_corpus, load_storage_cleanup_scenario,
    ActorLineageClass, AuthorityClass, CleanupResultClass, ClearDataReview, ConsentState,
    ExportBeforeDeleteClass, GcPolicyClass, LowDiskStateClass, StorageClassId,
    StorageClassRegistry, StorageCleanupReceipt, StorageCleanupScenario, TriggerClass,
    CLEAR_DATA_REVIEW_RECORD_KIND, CLEAR_DATA_REVIEW_SCHEMA_REF,
    STORAGE_CLASS_REGISTRY_RECORD_KIND, STORAGE_CLASS_SCHEMA_REF, STORAGE_CLEANUP_CORPUS_DIR,
    STORAGE_CLEANUP_CORPUS_MANIFEST_REF, STORAGE_CLEANUP_DOC_REF,
    STORAGE_CLEANUP_RECEIPT_RECORD_KIND, STORAGE_CLEANUP_RECEIPT_SCHEMA_REF,
    STORAGE_CLEANUP_REGISTRY_REF, STORAGE_CLEANUP_SCENARIO_RECORD_KIND,
    STORAGE_CLEANUP_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

#[test]
fn registry_parses_and_pins_protected_classes() {
    let registry: StorageClassRegistry = current_storage_class_registry().expect("registry parses");
    assert_eq!(registry.record_kind, STORAGE_CLASS_REGISTRY_RECORD_KIND);
    assert_eq!(registry.schema_ref, STORAGE_CLASS_SCHEMA_REF);
    assert_eq!(registry.doc_ref, STORAGE_CLEANUP_DOC_REF);

    for required in StorageClassRegistry::required_class_ids() {
        let entry = registry.entry(*required).expect("required class present");
        assert_eq!(entry.class_id, *required);
    }

    let evidence = registry
        .entry(StorageClassId::EvidenceSupportCache)
        .expect("evidence cache entry present");
    assert!(evidence.protected_default);
    assert!(matches!(
        evidence.gc_policy_class,
        GcPolicyClass::NeverEvictSilently
    ));
    assert!(matches!(
        evidence.authority_class,
        AuthorityClass::EvidenceGrade
    ));

    let recovery = registry
        .entry(StorageClassId::UserOwnedRecoveryState)
        .expect("recovery state entry present");
    assert!(recovery.protected_default);
    assert!(matches!(
        recovery.gc_policy_class,
        GcPolicyClass::NeverEvictSilently
    ));
    assert!(matches!(
        recovery.authority_class,
        AuthorityClass::UserAuthoredRecovery
    ));

    let priorities: Vec<u32> = {
        let mut v: Vec<u32> = registry
            .entries
            .iter()
            .map(|entry| entry.low_disk_eviction_priority)
            .collect();
        v.sort();
        v
    };
    assert_eq!(priorities, vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn corpus_validates_against_the_safety_contract() {
    let corpus = current_storage_cleanup_corpus().expect("corpus parses");
    let violations = corpus.validate();
    assert!(violations.is_empty(), "corpus violations: {violations:#?}");
}

#[test]
fn every_scenario_pins_protected_classes_and_metadata_safe_baseline() {
    let corpus = current_storage_cleanup_corpus().expect("corpus parses");
    for entry in &corpus.entries {
        let scenario: &StorageCleanupScenario = &entry.scenario;
        assert_eq!(scenario.record_kind, STORAGE_CLEANUP_SCENARIO_RECORD_KIND);

        let review: &ClearDataReview = &scenario.review;
        assert_eq!(review.record_kind, CLEAR_DATA_REVIEW_RECORD_KIND);
        assert_eq!(review.schema_ref, CLEAR_DATA_REVIEW_SCHEMA_REF);
        assert!(!review.raw_content_exported);
        assert_eq!(review.redaction_class, "metadata_safe_default");
        assert_eq!(review.storage_class_registry_ref, STORAGE_CLASS_SCHEMA_REF);

        let receipt: &StorageCleanupReceipt = &scenario.receipt;
        assert_eq!(receipt.record_kind, STORAGE_CLEANUP_RECEIPT_RECORD_KIND);
        assert_eq!(receipt.schema_ref, STORAGE_CLEANUP_RECEIPT_SCHEMA_REF);
        assert!(!receipt.raw_content_exported);
        assert_eq!(receipt.redaction_class, "metadata_safe_default");
        assert_eq!(receipt.review_ref, review.review_id);

        let override_classes: Vec<StorageClassId> = review
            .override_protected_class_refs
            .iter()
            .map(|row| row.class_id)
            .collect();

        // Protected classes must always be surfaced — either in the review's
        // protected_class_rows or, when explicitly overridden, in the
        // override list. The receipt must record them in skipped or pair
        // them with an override.
        for required in [
            StorageClassId::EvidenceSupportCache,
            StorageClassId::UserOwnedRecoveryState,
        ] {
            let in_protected = review
                .protected_class_rows
                .iter()
                .any(|row| row.class_id == required);
            let in_override = override_classes.contains(&required);
            assert!(
                in_protected || in_override,
                "{}: protected class {:?} missing from review",
                scenario.scenario_id,
                required
            );
            let in_skipped = receipt
                .skipped_protected_class_rows
                .iter()
                .any(|row| row.class_id == required);
            assert!(
                in_skipped || in_override,
                "{}: protected class {:?} missing from receipt",
                scenario.scenario_id,
                required
            );
        }
    }
}

#[test]
fn low_disk_pressure_scenarios_record_ordered_eviction_and_paused_work() {
    let corpus = current_storage_cleanup_corpus().expect("corpus parses");
    let mut saw_low_disk = false;
    for scenario in corpus.scenarios() {
        if scenario.trigger_class != TriggerClass::LowDiskPressure {
            continue;
        }
        saw_low_disk = true;
        let ctx = scenario
            .receipt
            .low_disk_context
            .as_ref()
            .expect("low_disk_pressure receipt must carry context");
        assert!(matches!(
            ctx.state_class,
            LowDiskStateClass::Warning
                | LowDiskStateClass::Critical
                | LowDiskStateClass::QuotaPressure
        ));
        let orders: Vec<u32> = ctx
            .ordered_eviction_steps
            .iter()
            .map(|step| step.order)
            .collect();
        let mut sorted = orders.clone();
        sorted.sort();
        assert_eq!(
            orders, sorted,
            "ordered_eviction_steps not in increasing order"
        );
        // Protected classes are not in the eviction list.
        for step in &ctx.ordered_eviction_steps {
            assert!(
                !matches!(
                    step.class_id,
                    StorageClassId::EvidenceSupportCache | StorageClassId::UserOwnedRecoveryState
                ),
                "{}: low-disk eviction includes protected class {:?}",
                scenario.scenario_id,
                step.class_id
            );
        }
        assert!(
            !ctx.paused_work_rows.is_empty(),
            "{}: low-disk receipt should record paused work",
            scenario.scenario_id
        );
        assert!(scenario.receipt.actor_lineage_class == ActorLineageClass::LowDiskEvictionPrompt);
    }
    assert!(
        saw_low_disk,
        "corpus must include a low_disk_pressure scenario"
    );
}

#[test]
fn export_before_delete_required_for_overrides_on_durable_classes() {
    let corpus = current_storage_cleanup_corpus().expect("corpus parses");
    for scenario in corpus.scenarios() {
        for override_row in &scenario.review.override_protected_class_refs {
            // Find the matching selected_class_refs row. If
            // export_before_delete_class is required_before_delete, an
            // export_target_ref must be present (already enforced by
            // validation; assert directly here as well).
            if let Some(selected) = scenario
                .review
                .selected_class_refs
                .iter()
                .find(|row| row.class_id == override_row.class_id)
            {
                if matches!(
                    selected.export_before_delete_class,
                    ExportBeforeDeleteClass::RequiredBeforeDelete
                ) {
                    assert!(
                        selected.export_target_ref.is_some(),
                        "{}: export_target_ref required for required_before_delete on {:?}",
                        scenario.scenario_id,
                        override_row.class_id
                    );
                }
            }
        }
    }
}

#[test]
fn cancelled_review_results_in_no_bytes_reclaimed() {
    let corpus = current_storage_cleanup_corpus().expect("corpus parses");
    let mut saw_cancelled = false;
    for scenario in corpus.scenarios() {
        if !matches!(scenario.review.consent_state, ConsentState::Cancelled) {
            continue;
        }
        saw_cancelled = true;
        assert!(matches!(
            scenario.receipt.result_class,
            CleanupResultClass::Cancelled
        ));
        for outcome in &scenario.receipt.class_outcomes {
            assert_eq!(
                outcome.bytes_reclaimed, 0,
                "{}: cancelled receipt reclaimed bytes for {:?}",
                scenario.scenario_id, outcome.class_id
            );
        }
    }
    assert!(
        saw_cancelled,
        "corpus must include a cancelled-override scenario"
    );
}

#[test]
fn corpus_fixture_paths_exist_on_disk() {
    let corpus = current_storage_cleanup_corpus().expect("corpus parses");
    let root = repo_root();
    assert!(root.join(STORAGE_CLEANUP_CORPUS_DIR).is_dir());
    assert!(root.join(STORAGE_CLEANUP_CORPUS_MANIFEST_REF).is_file());
    assert!(root.join(STORAGE_CLEANUP_REGISTRY_REF).is_file());
    assert!(root.join(STORAGE_CLEANUP_DOC_REF).is_file());
    for entry in &corpus.entries {
        assert!(
            root.join(&entry.fixture_ref).is_file(),
            "fixture {} missing",
            entry.fixture_ref
        );
    }
}

#[test]
fn yaml_round_trip_load_matches_corpus_entry() {
    let corpus = current_storage_cleanup_corpus().expect("corpus parses");
    let root = repo_root();
    for entry in &corpus.entries {
        let yaml = std::fs::read_to_string(root.join(&entry.fixture_ref))
            .unwrap_or_else(|err| panic!("read {}: {err}", entry.fixture_ref));
        let parsed = load_storage_cleanup_scenario(&yaml)
            .unwrap_or_else(|err| panic!("parse {}: {err}", entry.fixture_ref));
        assert_eq!(parsed, entry.scenario);
    }
}

#[test]
fn support_export_envelope_is_metadata_safe() {
    let corpus = current_storage_cleanup_corpus().expect("corpus parses");
    let export = corpus.support_export(
        "support_export.storage_cleanup.beta",
        "2026-05-19T15:30:00Z",
    );
    assert_eq!(
        export.record_kind,
        STORAGE_CLEANUP_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND
    );
    assert!(export.is_export_safe());
    assert_eq!(export.rows.len(), corpus.entries.len());
    for row in &export.rows {
        assert!(!row.raw_content_exported);
        assert!(!row.reopen_inspector_action_ref.is_empty());
        assert!(!row.selected_class_ids.is_empty());
        assert!(!row.skipped_protected_class_ids.is_empty());
    }
}

#[test]
fn validate_refuses_a_review_that_drops_a_protected_class() {
    let mut corpus = current_storage_cleanup_corpus().expect("corpus parses");
    // Pick the user-requested cleanup scenario and remove the user-owned
    // recovery state row from protected_class_rows. The validator must
    // flag it.
    let entry = corpus
        .entries
        .iter_mut()
        .find(|entry| {
            entry.scenario.scenario_id
                == "support.m3.storage_cleanup.user_requested_hot_and_knowledge_cleanup"
        })
        .expect("seed scenario present");
    entry
        .scenario
        .review
        .protected_class_rows
        .retain(|row| row.class_id != StorageClassId::UserOwnedRecoveryState);
    let violations = corpus.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "review.required_protected_class_missing"),
        "expected review.required_protected_class_missing in {violations:#?}"
    );
}

#[test]
fn validate_refuses_a_low_disk_receipt_without_low_disk_context() {
    let mut corpus = current_storage_cleanup_corpus().expect("corpus parses");
    let entry = corpus
        .entries
        .iter_mut()
        .find(|entry| {
            entry.scenario.scenario_id == "support.m3.storage_cleanup.low_disk_ordered_eviction"
        })
        .expect("low-disk scenario present");
    entry.scenario.receipt.low_disk_context = None;
    let violations = corpus.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "receipt.low_disk_context_required"),
        "expected receipt.low_disk_context_required in {violations:#?}"
    );
}

#[test]
fn validate_refuses_a_registry_missing_a_required_class() {
    let mut corpus = current_storage_cleanup_corpus().expect("corpus parses");
    corpus
        .registry
        .entries
        .retain(|entry| entry.class_id != StorageClassId::EvidenceSupportCache);
    let violations = corpus.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "registry.required_class_missing"
                && v.target_ref == "evidence_support_cache"),
        "expected registry.required_class_missing in {violations:#?}"
    );
}

#[test]
fn validate_refuses_selecting_a_protected_class_without_override() {
    let mut corpus = current_storage_cleanup_corpus().expect("corpus parses");
    let entry = corpus
        .entries
        .iter_mut()
        .find(|entry| {
            entry.scenario.scenario_id
                == "support.m3.storage_cleanup.user_requested_hot_and_knowledge_cleanup"
        })
        .expect("seed scenario present");
    // Inject the user-owned recovery state into selected_class_refs without
    // an override; validator must flag it.
    entry.scenario.review.selected_class_refs.push(
        aureline_support::storage_inspector::SelectedClassRow {
            class_id: StorageClassId::UserOwnedRecoveryState,
            bytes_in_scope: 1,
            consequence_class:
                aureline_support::storage_inspector::ConsequenceClass::LostWithNoRebuildPath,
            consequence_summary: "would lose local history".to_owned(),
            export_before_delete_class: ExportBeforeDeleteClass::NotApplicable,
            export_target_ref: None,
        },
    );
    let violations = corpus.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "review.protected_class_in_selected_without_override"),
        "expected review.protected_class_in_selected_without_override in {violations:#?}"
    );
}
