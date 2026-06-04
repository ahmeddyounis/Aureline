//! Fixture replay and invariant tests for structured config, manifest,
//! environment-file, and live-state editor qualification records.

use std::collections::BTreeSet;

use aureline_config::structured_config_manifest_environment_editor_qualification::{
    structured_editor_corpus, ClaimClass, CopyExportMode, RoundTripRisk,
    StructuredEditorQualification, SurfaceClass, TruthLayer, ValidationClass, WritePosture,
    STRUCTURED_EDITOR_RECORD_KIND, STRUCTURED_EDITOR_SHARED_CONTRACT_REF,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/config/m4/structured-config-manifest-environment-editor-qualification",
);

fn load_record(filename: &str) -> StructuredEditorQualification {
    let path = format!("{FIXTURE_DIR}/{filename}");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn every_scenario_fixture_matches_in_code_projection() {
    for scenario in structured_editor_corpus() {
        let on_disk = load_record(&scenario.fixture_filename);
        let in_code = scenario.record();
        assert_eq!(
            on_disk, in_code,
            "{} fixture drifted; re-emit with `cargo run -q -p aureline-config --bin aureline_config_structured_editor_qualification -- emit-fixtures fixtures/config/m4/structured-config-manifest-environment-editor-qualification`",
            scenario.fixture_filename,
        );
    }
}

#[test]
fn record_identity_and_claim_rollups_are_derived() {
    for scenario in structured_editor_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert_eq!(record.record_kind, STRUCTURED_EDITOR_RECORD_KIND);
        assert_eq!(
            record.shared_contract_ref,
            STRUCTURED_EDITOR_SHARED_CONTRACT_REF
        );
        assert_eq!(
            record.qualification.claim_class, scenario.expected_claim_class,
            "{} claim class",
            scenario.scenario_id
        );
        assert_eq!(
            record.qualification.qualifies_stable, scenario.expected_qualifies_stable,
            "{} stable verdict",
            scenario.scenario_id
        );
        assert_eq!(
            record.qualification.qualifies_stable,
            record.qualification.claim_class == ClaimClass::Stable,
            "{} stable claim consistency",
            scenario.scenario_id
        );
    }
}

#[test]
fn stable_records_expose_source_effective_preview_and_live_truth() {
    for scenario in structured_editor_corpus() {
        let record = load_record(&scenario.fixture_filename);
        if record.qualification.qualifies_stable {
            let truth_layers: BTreeSet<TruthLayer> =
                record.available_truth_layers.iter().copied().collect();
            for required in [
                TruthLayer::Source,
                TruthLayer::EffectiveValue,
                TruthLayer::PlannedPreview,
                TruthLayer::LiveObserved,
            ] {
                assert!(
                    truth_layers.contains(&required),
                    "{} missing truth layer {:?}",
                    scenario.scenario_id,
                    required
                );
            }
        }
    }
}

#[test]
fn preservation_or_downgrade_is_explicit() {
    for scenario in structured_editor_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let fully_preserving = record.preservation.preserves_comments
            && record.preservation.preserves_unknown_fields
            && record.preservation.preserves_ordering
            && record.preservation.preserves_extension_namespaces;
        if matches!(
            record.round_trip_risk,
            RoundTripRisk::CompareOnly | RoundTripRisk::RawSourceOnly
        ) {
            assert!(
                matches!(
                    record.write_posture,
                    WritePosture::InspectOnly | WritePosture::SourceOnlyEdit
                ),
                "{} compare/raw fallback must not expose structured write",
                scenario.scenario_id
            );
        }
        if record.qualification.qualifies_stable {
            assert!(
                fully_preserving,
                "{} must prove full preservation",
                scenario.scenario_id
            );
        }
        assert!(
            !record.preservation.fixture_refs.is_empty(),
            "{} must cite preservation or downgrade fixtures",
            scenario.scenario_id
        );
    }
}

#[test]
fn parameters_have_effective_winners_and_layer_specific_resets() {
    for scenario in structured_editor_corpus() {
        let record = load_record(&scenario.fixture_filename);
        assert!(
            record
                .parameter_sources
                .iter()
                .any(|row| row.wins_effective_value),
            "{} must name the effective winner",
            scenario.scenario_id
        );
        assert!(
            record
                .parameter_sources
                .iter()
                .all(|row| row.layer_specific_reset),
            "{} reset/remove override actions must be layer-specific",
            scenario.scenario_id
        );
    }
}

#[test]
fn copy_export_does_not_leak_raw_secret_values() {
    for scenario in structured_editor_corpus() {
        let record = load_record(&scenario.fixture_filename);
        for chip in &record.secret_chips {
            assert!(
                chip.raw_secret_export_blocked_by_default,
                "{} raw secret export must be blocked",
                scenario.scenario_id
            );
            assert!(
                matches!(
                    chip.copy_export_mode,
                    CopyExportMode::ReferenceHandle
                        | CopyExportMode::RedactedPlaceholder
                        | CopyExportMode::KeyPathOnly
                ),
                "{} secret chip must not copy a raw literal",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn shared_validation_model_covers_all_required_classes() {
    for scenario in structured_editor_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let present: BTreeSet<ValidationClass> = record
            .validations
            .iter()
            .map(|row| row.validation_class)
            .collect();
        for required in ValidationClass::REQUIRED {
            assert!(
                present.contains(&required),
                "{} missing validation class {:?}",
                scenario.scenario_id,
                required
            );
        }
        assert!(
            record.validations.iter().all(|row| !row.reason.is_empty()),
            "{} validation rows must have exact reasons",
            scenario.scenario_id
        );
    }
}

#[test]
fn required_surfaces_share_the_contract_or_narrow() {
    for scenario in structured_editor_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let present: BTreeSet<SurfaceClass> = record
            .surface_parity
            .iter()
            .map(|row| row.surface_class)
            .collect();
        for required in SurfaceClass::REQUIRED {
            assert!(
                present.contains(&required),
                "{} missing surface {:?}",
                scenario.scenario_id,
                required
            );
        }
        let all_shared = record.surface_parity.iter().all(|row| {
            row.consumes_shared_contract
                && row.redaction_safe
                && row.exposes_preview_apply_revert_lineage
        });
        assert_eq!(
            all_shared,
            !record
                .qualification
                .narrowing_reasons
                .iter()
                .any(|reason| reason == "surface_vocabulary_not_shared"),
            "{} surface parity should drive narrowing",
            scenario.scenario_id
        );
    }
}

#[test]
fn risky_drills_block_before_mutation_or_narrow() {
    for scenario in structured_editor_corpus() {
        let record = load_record(&scenario.fixture_filename);
        let all_block = record
            .risk_drills
            .iter()
            .all(|row| row.blocks_before_mutation && !row.visible_reason.is_empty());
        assert_eq!(
            all_block,
            !record
                .qualification
                .narrowing_reasons
                .iter()
                .any(|reason| reason == "risk_drill_does_not_block_before_mutation"),
            "{} drill blocking should drive narrowing",
            scenario.scenario_id
        );
    }
}
