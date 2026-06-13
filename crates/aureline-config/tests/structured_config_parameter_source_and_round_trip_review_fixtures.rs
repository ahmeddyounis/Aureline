//! Fixture replay and invariant tests for the M5 structured-config
//! parameter-source and round-trip-review packet.

use std::collections::BTreeSet;

use aureline_config::structured_config_parameter_source_and_round_trip_review::{
    seeded_structured_config_parameter_source_and_round_trip_review, OutputDisclosureClass,
    RoundTripRiskFlag, StructuredConfigParameterSourceRoundTripReviewPacket, ValueChipClass,
    STRUCTURED_CONFIG_PARAMETER_SOURCE_ROUND_TRIP_REVIEW_RECORD_KIND,
    STRUCTURED_CONFIG_PARAMETER_SOURCE_ROUND_TRIP_REVIEW_SHARED_CONTRACT_REF,
};

const FIXTURE_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/config/structured_config_parameter_source_and_round_trip_review/canonical.json",
);

fn load_packet() -> StructuredConfigParameterSourceRoundTripReviewPacket {
    let body = std::fs::read_to_string(FIXTURE_PATH)
        .unwrap_or_else(|err| panic!("failed to read {FIXTURE_PATH}: {err}"));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {FIXTURE_PATH}: {err}"))
}

#[test]
fn fixture_matches_in_code_projection() {
    assert_eq!(
        load_packet(),
        seeded_structured_config_parameter_source_and_round_trip_review(),
        "fixture drifted; re-emit with `cargo run -q -p aureline-config --bin aureline_config_structured_parameter_source_and_round_trip_review -- json > fixtures/config/structured_config_parameter_source_and_round_trip_review/canonical.json`",
    );
}

#[test]
fn packet_identity_and_rollups_are_stable() {
    let packet = load_packet();
    assert_eq!(
        packet.record_kind,
        STRUCTURED_CONFIG_PARAMETER_SOURCE_ROUND_TRIP_REVIEW_RECORD_KIND
    );
    assert_eq!(
        packet.shared_contract_ref,
        STRUCTURED_CONFIG_PARAMETER_SOURCE_ROUND_TRIP_REVIEW_SHARED_CONTRACT_REF
    );
    assert_eq!(
        packet.summary.artifact_review_count,
        packet.artifact_reviews.len()
    );
}

#[test]
fn chip_output_and_round_trip_coverage_are_complete() {
    let packet = load_packet();
    let chip_classes: BTreeSet<_> = packet
        .artifact_reviews
        .iter()
        .flat_map(|review| review.value_chips.iter().map(|chip| chip.chip_class))
        .collect();
    for required in ValueChipClass::ALL {
        assert!(
            chip_classes.contains(&required),
            "missing chip class {required:?}"
        );
    }

    let disclosure_classes: BTreeSet<_> = packet
        .artifact_reviews
        .iter()
        .flat_map(|review| {
            review
                .export_summary
                .output_disclosure_classes
                .iter()
                .copied()
        })
        .collect();
    for required in OutputDisclosureClass::ALL {
        assert!(
            disclosure_classes.contains(&required),
            "missing output disclosure {required:?}"
        );
    }

    let round_trip_flags: BTreeSet<_> = packet
        .artifact_reviews
        .iter()
        .filter_map(|review| review.round_trip_risk_banner.as_ref())
        .flat_map(|banner| banner.risk_flags.iter().copied())
        .collect();
    for required in RoundTripRiskFlag::ALL {
        assert!(
            round_trip_flags.contains(&required),
            "missing round-trip risk {required:?}"
        );
    }
}

#[test]
fn support_and_export_reuse_the_same_metadata() {
    let packet = load_packet();
    assert!(packet.summary.support_export_metadata_reused_everywhere);
    for review in &packet.artifact_reviews {
        let review_disclosures: BTreeSet<_> = review
            .effective_value_review_sheet
            .output_disclosure_classes
            .iter()
            .copied()
            .collect();
        let export_disclosures: BTreeSet<_> = review
            .export_summary
            .output_disclosure_classes
            .iter()
            .copied()
            .collect();
        assert_eq!(
            review_disclosures, export_disclosures,
            "{:?}",
            review.family
        );
    }
}
