//! Fixture replay and invariant tests for portable-state restore certification.

use aureline_settings::m5_portable_state_and_restore::{
    is_canonical_object_ref, portable_state_restore_corpus, M5PortableStateRestoreCertification,
    MigrationLabel, PortableArtifactClass, PortableRestoreClaim, SurfaceClass,
    M5_PORTABLE_STATE_RESTORE_RECORD_KIND, M5_PORTABLE_STATE_RESTORE_SHARED_CONTRACT_REF,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/settings/m5/m5-portable-state-and-restore",
);

fn load_record(filename: &str) -> M5PortableStateRestoreCertification {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn fixtures_match_in_code_projection() {
    for scenario in portable_state_restore_corpus() {
        let on_disk = load_record(&scenario.fixture_filename);
        assert_eq!(
            on_disk,
            scenario.record(),
            "{} drifted",
            scenario.scenario_id
        );
    }
}

#[test]
fn record_identity_and_rollups_are_stable() {
    for scenario in portable_state_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(record.record_kind, M5_PORTABLE_STATE_RESTORE_RECORD_KIND);
        assert_eq!(
            record.shared_contract_ref,
            M5_PORTABLE_STATE_RESTORE_SHARED_CONTRACT_REF
        );
        assert_eq!(
            record.fidelity_qualification.claim_class,
            scenario.expected_claim_class
        );
        assert_eq!(
            record.fidelity_qualification.effective_fidelity_ceiling,
            scenario.expected_fidelity_ceiling
        );
    }
}

#[test]
fn every_artifact_class_is_classified_with_a_canonical_ref() {
    for scenario in portable_state_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for class in PortableArtifactClass::REQUIRED {
            assert!(
                record
                    .package_classes
                    .iter()
                    .any(|row| row.artifact_class == class),
                "{} missing {class:?}",
                scenario.scenario_id
            );
        }
        for row in &record.package_classes {
            assert!(is_canonical_object_ref(&row.content_ref));
            assert!(!row.rationale.trim().is_empty());
        }
    }
}

#[test]
fn exact_cards_never_carry_missing_dependencies_or_schema_mismatch() {
    for scenario in portable_state_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for card in &record.restore_cards {
            if card.migration_label == MigrationLabel::Exact {
                assert!(
                    card.placeholders.is_empty(),
                    "{}: exact card {} has placeholders",
                    scenario.scenario_id,
                    card.card_id
                );
                assert_eq!(
                    card.source_schema_version, card.target_schema_version,
                    "{}: exact card {} crosses a schema mismatch",
                    scenario.scenario_id, card.card_id
                );
            }
        }
    }
}

#[test]
fn missing_dependencies_are_always_visible_placeholders() {
    for scenario in portable_state_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for card in &record.restore_cards {
            for placeholder in &card.placeholders {
                assert!(
                    placeholder.visible_in_layout && !placeholder.silently_dropped,
                    "{}: placeholder {:?} was dropped",
                    scenario.scenario_id,
                    placeholder.kind
                );
            }
        }
    }
}

#[test]
fn overwriting_restores_are_previewable_and_checkpointed() {
    for scenario in portable_state_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for card in &record.restore_cards {
            assert!(card.previewable_before_apply);
            if card.overwrites_local_state {
                assert!(
                    is_canonical_object_ref(&card.rollback_checkpoint_ref),
                    "{}: card {} overwrites without a checkpoint",
                    scenario.scenario_id,
                    card.card_id
                );
            }
        }
    }
}

#[test]
fn all_required_surfaces_consume_the_same_record() {
    for scenario in portable_state_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for surface in SurfaceClass::REQUIRED {
            let row = record
                .surface_truth
                .iter()
                .find(|row| row.surface_class == surface)
                .unwrap_or_else(|| panic!("{} missing {surface:?}", scenario.scenario_id));
            assert!(row.consumes_shared_record);
            assert!(row.shows_disposition);
            assert!(row.shows_migration_label);
            assert!(row.shows_placeholders);
            assert!(row.shows_rollback_checkpoint);
        }
    }
}

#[test]
fn exact_baseline_qualifies_and_drills_are_degraded() {
    for scenario in portable_state_restore_corpus() {
        let record = load_record(&scenario.fixture_filename);
        match scenario.scenario_id {
            "exact_local_restore" => {
                assert_eq!(
                    record.fidelity_qualification.claim_class,
                    PortableRestoreClaim::ExactRestore
                );
                assert!(record.fidelity_qualification.qualifies_exact);
            }
            _ => {
                assert_eq!(
                    record.fidelity_qualification.claim_class,
                    PortableRestoreClaim::DegradedRestore
                );
                assert!(!record.fidelity_qualification.qualifies_exact);
            }
        }
    }
}
