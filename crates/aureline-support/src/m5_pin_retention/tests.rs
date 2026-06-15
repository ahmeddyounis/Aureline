use super::*;

use crate::m5_storage_governance::current_m5_artifact_family_storage_matrix;

fn corpus() -> PinRetentionManagerCorpus {
    current_pin_retention_manager_corpus().expect("corpus parses")
}

fn matrix() -> M5ArtifactFamilyStorageMatrix {
    current_m5_artifact_family_storage_matrix().expect("matrix parses")
}

#[test]
fn corpus_parses_every_fixture() {
    let corpus = corpus();
    assert_eq!(corpus.managers.len(), MANAGER_FIXTURES.len());
}

#[test]
fn corpus_validates_against_the_safety_contract() {
    let corpus = corpus();
    let violations = corpus.validate();
    assert_eq!(violations, Vec::new(), "{violations:#?}");
}

#[test]
fn every_manager_binds_both_surfaces_and_offers_the_actions() {
    let corpus = corpus();
    for entry in &corpus.managers {
        let manager = &entry.manager;
        assert!(manager
            .surfaces
            .contains(&InspectableSurfaceClass::PinManager));
        assert!(manager
            .surfaces
            .contains(&InspectableSurfaceClass::CleanupHistoryLane));
        assert_eq!(
            manager.open_inspector_action_ref,
            OPEN_STORAGE_INSPECTOR_ACTION_REF
        );
        assert_eq!(
            manager.open_clear_data_review_action_ref,
            OPEN_CLEAR_DATA_REVIEW_ACTION_REF
        );
    }
}

#[test]
fn every_pin_derives_actor_unpin_and_export_from_its_source() {
    let corpus = corpus();
    for entry in &corpus.managers {
        for pin in &entry.manager.pins {
            assert_eq!(
                pin.pin_actor,
                pin_actor_for(pin.pin_source),
                "{} pin_actor",
                pin.pin_id
            );
            assert_eq!(
                pin.unpin_path,
                unpin_path_for(pin.pin_source),
                "{} unpin_path",
                pin.pin_id
            );
            // Protected entries must require export (or already be in an
            // assembly); a finite retention window always carries an expiry.
            if pin.protected_continuity {
                assert!(
                    matches!(
                        pin.export_path,
                        ExportPathClass::ExportRequiredBeforeDelete
                            | ExportPathClass::ExportAlreadyInAssembly
                    ),
                    "{} export_path",
                    pin.pin_id
                );
            }
            assert_eq!(
                pin.expires_at.is_some(),
                pin.retention_state == RetentionStateClass::InRetentionWindow,
                "{} expiry",
                pin.pin_id
            );
        }
    }
}

#[test]
fn cleanup_history_never_touches_authoritative_state_or_captures_payloads() {
    let corpus = corpus();
    for entry in &corpus.managers {
        assert!(
            entry.manager.is_export_safe(),
            "{}",
            entry.manager.manager_id
        );
        for event in &entry.manager.cleanup_history {
            assert!(!event.authoritative_state_touched);
            assert!(!event.raw_payload_captured);
        }
    }
}

#[test]
fn storage_pressure_never_reclaims_user_owned_recovery_bytes() {
    let corpus = corpus();
    for entry in &corpus.managers {
        for event in &entry.manager.cleanup_history {
            if event.storage_class_id == StorageClassId::UserOwnedRecoveryState
                && event.trigger_class.is_pressure()
            {
                assert_eq!(
                    event.reclaimed_bytes, 0,
                    "{} reclaimed recovery bytes under pressure",
                    event.event_id
                );
                assert_eq!(
                    event.disposition,
                    CleanupDispositionClass::BlockedNoOpPinProtected
                );
            }
        }
    }
}

#[test]
fn user_owned_recovery_deletion_requires_an_explicit_exported_user_action() {
    let corpus = corpus();
    let mut saw_explicit_delete = false;
    for entry in &corpus.managers {
        for event in &entry.manager.cleanup_history {
            if event.storage_class_id == StorageClassId::UserOwnedRecoveryState
                && event.reclaimed_bytes > 0
            {
                assert_eq!(event.actor_class, CleanupActorClass::User);
                assert_eq!(
                    event.disposition,
                    CleanupDispositionClass::ExportedThenDeleted
                );
                assert!(matches!(
                    event.trigger_class,
                    CleanupTriggerClass::ExplicitUserClearData
                        | CleanupTriggerClass::OffboardingOrReset
                ));
                saw_explicit_delete = true;
            }
        }
    }
    assert!(
        saw_explicit_delete,
        "the corpus must exercise an explicit, exported recovery delete"
    );
}

#[test]
fn blocked_cleanups_reclaim_zero_and_record_their_blocking_pins() {
    let corpus = corpus();
    let mut saw_block = false;
    for entry in &corpus.managers {
        for event in &entry.manager.cleanup_history {
            if event.disposition == CleanupDispositionClass::BlockedNoOpPinProtected {
                assert_eq!(event.reclaimed_bytes, 0, "{}", event.event_id);
                assert!(event.blocked_pin_count >= 1, "{}", event.event_id);
                assert!(!event.blocked_pin_sources.is_empty(), "{}", event.event_id);
                saw_block = true;
            }
        }
    }
    assert!(saw_block, "the corpus must exercise a pin-blocked cleanup");
}

#[test]
fn evidence_expiry_only_targets_the_evidence_class() {
    let corpus = corpus();
    for entry in &corpus.managers {
        for event in &entry.manager.cleanup_history {
            if event.disposition == CleanupDispositionClass::ExpiredUnpinnedEvidencePastRetention {
                assert_eq!(
                    event.storage_class_id,
                    StorageClassId::EvidenceSupportCache,
                    "{}",
                    event.event_id
                );
            }
        }
    }
}

#[test]
fn composer_matches_every_seeded_manager() {
    let matrix = matrix();
    let corpus = corpus();
    for signal in seeded_manager_signals() {
        let composed = compose_manager(&matrix, &signal);
        let seeded = corpus
            .manager(&signal.manager_id)
            .unwrap_or_else(|| panic!("seeded manager {} present", signal.manager_id))
            .clone();
        assert_eq!(composed, seeded, "{}", signal.manager_id);
        // A composed manager must validate cleanly.
        assert!(composed.is_valid(), "{}", signal.manager_id);
    }
}

#[test]
fn composer_derives_protection_from_the_frozen_matrix() {
    let matrix = matrix();
    // A checkpoint pin folds onto the user-owned recovery class and is protected.
    let signal = ManagerSignal {
        manager_id: "composed.protection".to_owned(),
        emitted_at: "2026-06-14T00:00:00Z".to_owned(),
        scope_ref: "ws.test".to_owned(),
        scope_label: "Test".to_owned(),
        pins: vec![
            PinInput {
                pin_id: "pin.checkpoint".to_owned(),
                label: "Checkpoint".to_owned(),
                family_id: ArtifactFamilyId::UserOwnedRecoveryState,
                pin_source: PinSourceClass::ExplicitUserPin,
                referenced_object_class: ReferencedObjectClass::LocalCheckpointOrHistory,
                referenced_object_ref: "checkpoint.v1".to_owned(),
                retention_state: RetentionStateClass::RetainedUntilExplicitReset,
                expires_at: None,
                pinned_by_ref: None,
                on_disk_bytes: 1,
            },
            PinInput {
                pin_id: "pin.preview".to_owned(),
                label: "Preview".to_owned(),
                family_id: ArtifactFamilyId::GeneratedPreview,
                pin_source: PinSourceClass::ExplicitUserPin,
                referenced_object_class: ReferencedObjectClass::WorkspaceArtifact,
                referenced_object_ref: "preview.v1".to_owned(),
                retention_state: RetentionStateClass::PinnedByExplicitUserChoice,
                expires_at: None,
                pinned_by_ref: None,
                on_disk_bytes: 1,
            },
        ],
        cleanups: vec![],
    };
    let manager = compose_manager(&matrix, &signal);
    let checkpoint = manager.pin("pin.checkpoint").expect("checkpoint pin");
    assert!(checkpoint.protected_continuity);
    assert_eq!(
        checkpoint.storage_class_id,
        StorageClassId::UserOwnedRecoveryState
    );
    assert_eq!(
        checkpoint.export_path,
        ExportPathClass::ExportRequiredBeforeDelete
    );
    let preview = manager.pin("pin.preview").expect("preview pin");
    assert!(!preview.protected_continuity);
    assert_eq!(
        preview.export_path,
        ExportPathClass::ExportOfferedBeforeDelete
    );
    assert!(manager.is_valid());
}

#[test]
fn validator_rejects_a_silent_recovery_delete_under_pressure() {
    let corpus = corpus();
    let mut manager = corpus
        .manager("pin_retention.managed_quota_preserves_user_state.v1")
        .expect("manager present")
        .clone();
    // Tamper: make managed quota pressure reclaim user-owned recovery bytes.
    let event = manager
        .cleanup_history
        .iter_mut()
        .find(|event| event.storage_class_id == StorageClassId::UserOwnedRecoveryState)
        .expect("recovery cleanup present");
    event.reclaimed_bytes = 140_000_000;
    event.disposition = CleanupDispositionClass::TrimmedUnpinnedArtifact;
    event.resulting_state = ResultingStateClass::FullyReclaimedNoResidual;
    let mut violations = Vec::new();
    manager.validate_into(&mut violations, "tampered");
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "cleanup.recovery.pressure_delete"),
        "{violations:#?}"
    );
}

#[test]
fn validator_rejects_a_derived_field_mismatch() {
    let corpus = corpus();
    let mut manager = corpus
        .manager("pin_retention.evidence_and_checkpoints.v1")
        .expect("manager present")
        .clone();
    // Tamper: claim a release pin can be unpinned directly.
    manager.pins[0].unpin_path = UnpinPathClass::UserUnpinsDirectly;
    let mut violations = Vec::new();
    manager.validate_into(&mut violations, "tampered");
    assert!(
        violations.iter().any(|v| v.check_id == "pin.unpin_path"),
        "{violations:#?}"
    );
}

#[test]
fn support_export_matches_checked_in_golden() {
    let corpus = corpus();
    let export =
        corpus.support_export("support_export.m5_pin_retention.v1", "2026-06-14T00:00:00Z");
    const GOLDEN: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/storage/m5_pin_retention/support_export.golden.json"
    ));
    let golden: PinRetentionSupportExport = serde_json::from_str(GOLDEN).expect("golden parses");
    assert_eq!(
        export, golden,
        "projected support export drifted from the checked-in golden; \
         regenerate with `cargo run -p aureline-support --example \
         dump_m5_pin_retention_support_export`"
    );
    assert!(export.is_export_safe());
}

#[test]
fn support_export_is_metadata_safe_and_reports_no_loss() {
    let corpus = corpus();
    let export = corpus.support_export("envelope.test", "2026-06-14T00:00:00Z");
    assert!(!export.raw_content_exported);
    assert_eq!(export.redaction_class, METADATA_SAFE_DEFAULT);
    assert_eq!(export.authoritative_state_loss_count, 0);
    assert_eq!(export.raw_payload_capture_count, 0);
    assert_eq!(export.manager_count, corpus.managers.len() as u32);
}
