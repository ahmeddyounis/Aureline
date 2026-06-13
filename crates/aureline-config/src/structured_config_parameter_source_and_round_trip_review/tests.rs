use std::fs;

use super::{
    audit_structured_config_parameter_source_and_round_trip_review,
    parse_structured_config_parameter_source_and_round_trip_review,
    seeded_structured_config_parameter_source_and_round_trip_review, ConsumerSurfaceClass,
    OutputDisclosureClass, ParameterRowField, RoundTripRiskFlag, ValueChipClass,
    STRUCTURED_CONFIG_PARAMETER_SOURCE_ROUND_TRIP_REVIEW_PATH,
};

const FIXTURE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/config/structured_config_parameter_source_and_round_trip_review/canonical.json",
);

#[test]
fn seeded_packet_passes_validation() {
    let packet = seeded_structured_config_parameter_source_and_round_trip_review();
    let defects = audit_structured_config_parameter_source_and_round_trip_review(&packet);
    assert!(defects.is_empty(), "validation defects: {defects:?}");
}

#[test]
fn checked_in_artifact_matches_seeded_packet() {
    let path = format!(
        "{}/../../{}",
        env!("CARGO_MANIFEST_DIR"),
        STRUCTURED_CONFIG_PARAMETER_SOURCE_ROUND_TRIP_REVIEW_PATH
    );
    let body = fs::read_to_string(path).expect("artifact exists");
    let artifact = parse_structured_config_parameter_source_and_round_trip_review(&body)
        .expect("artifact parses");
    assert_eq!(
        artifact,
        seeded_structured_config_parameter_source_and_round_trip_review()
    );
}

#[test]
fn checked_in_fixture_matches_seeded_packet() {
    let body = fs::read_to_string(FIXTURE_PATH).expect("fixture exists");
    let fixture = parse_structured_config_parameter_source_and_round_trip_review(&body)
        .expect("fixture parses");
    assert_eq!(
        fixture,
        seeded_structured_config_parameter_source_and_round_trip_review()
    );
}

#[test]
fn vocabularies_cover_all_required_fields_classes_and_outputs() {
    let packet = seeded_structured_config_parameter_source_and_round_trip_review();
    let fields: Vec<_> = packet
        .parameter_row_vocabulary
        .iter()
        .map(|row| row.field)
        .collect();
    for required in ParameterRowField::ALL {
        assert!(fields.contains(&required), "missing field {required:?}");
    }

    let chip_classes: Vec<_> = packet
        .value_chip_vocabulary
        .iter()
        .map(|row| row.chip_class)
        .collect();
    for required in ValueChipClass::ALL {
        assert!(
            chip_classes.contains(&required),
            "missing chip class {required:?}"
        );
    }

    let output_classes: Vec<_> = packet
        .output_disclosure_vocabulary
        .iter()
        .map(|row| row.output_class)
        .collect();
    for required in OutputDisclosureClass::ALL {
        assert!(
            output_classes.contains(&required),
            "missing output disclosure class {required:?}"
        );
    }
}

#[test]
fn shared_surfaces_render_full_review_contract() {
    let packet = seeded_structured_config_parameter_source_and_round_trip_review();
    let surfaces: Vec<_> = packet
        .surface_vocabulary
        .iter()
        .map(|row| row.surface)
        .collect();
    for required in ConsumerSurfaceClass::ALL {
        assert!(surfaces.contains(&required), "missing surface {required:?}");
    }
    assert!(packet.surface_vocabulary.iter().all(|row| {
        row.renders_parameter_rows
            && row.renders_value_chips
            && row.renders_round_trip_banner
            && row.renders_compare_before_save_sheet
            && row.renders_effective_value_review_sheet
            && row.renders_export_summary
    }));
}

#[test]
fn compare_before_save_reviews_cover_all_round_trip_risk_flags() {
    let packet = seeded_structured_config_parameter_source_and_round_trip_review();
    let covered: Vec<_> = packet
        .artifact_reviews
        .iter()
        .filter_map(|review| review.round_trip_risk_banner.as_ref())
        .flat_map(|banner| banner.risk_flags.iter().copied())
        .collect();
    for required in RoundTripRiskFlag::ALL {
        assert!(
            covered.contains(&required),
            "missing round-trip coverage for {required:?}"
        );
    }
    for review in &packet.artifact_reviews {
        if review.round_trip_risk_banner.is_some() {
            assert!(review.compare_before_save_sheet.is_some());
        }
    }
}

#[test]
fn export_summaries_match_effective_value_review_disclosures() {
    let packet = seeded_structured_config_parameter_source_and_round_trip_review();
    for review in &packet.artifact_reviews {
        let review_set: std::collections::BTreeSet<_> = review
            .effective_value_review_sheet
            .output_disclosure_classes
            .iter()
            .copied()
            .collect();
        let export_set: std::collections::BTreeSet<_> = review
            .export_summary
            .output_disclosure_classes
            .iter()
            .copied()
            .collect();
        assert_eq!(review_set, export_set, "{:?}", review.family);
        assert!(
            review.support_export_reuses_export_summary,
            "{:?}",
            review.family
        );
    }
}

#[test]
fn secret_and_policy_chips_block_raw_secret_export() {
    let packet = seeded_structured_config_parameter_source_and_round_trip_review();
    for review in &packet.artifact_reviews {
        for chip in &review.value_chips {
            if matches!(
                chip.chip_class,
                ValueChipClass::SecretHandle | ValueChipClass::PolicyInjected
            ) {
                assert!(
                    chip.raw_secret_export_blocked_by_default,
                    "{:?} {:?}",
                    review.family, chip.key_path
                );
            }
        }
    }
}

#[test]
fn lifecycle_markers_and_hidden_flag_guards_stay_visible() {
    let packet = seeded_structured_config_parameter_source_and_round_trip_review();
    assert!(packet.summary.lifecycle_dependency_marker_count > 0);
    assert!(packet.summary.hidden_flag_guarded_family_count > 0);
    for review in &packet.artifact_reviews {
        if review.hidden_flag_spill_guard.verdict
            != super::HiddenFlagSpillVerdict::ClearStableSurface
        {
            assert!(
                review
                    .parameter_source_rows
                    .iter()
                    .any(|row| row.lifecycle_dependency.is_some()),
                "{:?}",
                review.family
            );
        }
    }
}

#[test]
fn mutation_reviews_are_scope_explicit_and_checkpointed_or_denied() {
    let packet = seeded_structured_config_parameter_source_and_round_trip_review();
    assert!(packet.summary.mutation_review_count > 0);
    assert!(packet.summary.policy_denied_mutation_review_count > 0);
    for review in &packet.artifact_reviews {
        assert!(!review.mutation_reviews.is_empty(), "{:?}", review.family);
        for mutation_review in &review.mutation_reviews {
            assert!(
                !mutation_review.scope_label.is_empty(),
                "{:?}",
                review.family
            );
            assert!(
                !mutation_review.preview_ref.is_empty(),
                "{:?}",
                review.family
            );
            if mutation_review.policy_denied_reason.is_none() {
                assert!(
                    mutation_review.rollback_checkpoint_ref.is_some(),
                    "{:?}",
                    review.family
                );
            }
        }
    }
}
