//! Protected tests for the alpha protected fitness packet consumer.

use aureline_support::fitness::{
    current_fitness_packet_alpha, FitnessPacketAlpha, FitnessPacketAlphaError,
    PROTECTED_FITNESS_PACKET_ALPHA_RECORD_KIND,
};
use aureline_support::release_evidence::{
    current_alpha_artifact_graph, ALPHA_RELEASE_EVIDENCE_PACKET_RECORD_KIND,
};

const PACKET_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/protected_fitness_packet_alpha.yaml"
));
const CATALOG_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/bench/fitness_function_catalog.yaml"
));
const STATE_ROWS_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/fitness_state_rows.yaml"
));

fn parse_packet(packet_yaml: &str) -> Result<FitnessPacketAlpha, FitnessPacketAlphaError> {
    FitnessPacketAlpha::from_yaml_documents(packet_yaml, CATALOG_YAML, STATE_ROWS_YAML)
}

fn assert_invalid_check(packet_yaml: &str, expected_check_id: &str) {
    let err = parse_packet(packet_yaml).expect_err("packet must fail validation");
    let FitnessPacketAlphaError::Invalid(violations) = err else {
        panic!("expected validation error, got {err:?}");
    };
    assert!(
        violations
            .iter()
            .any(|violation| violation.check_id == expected_check_id),
        "missing {expected_check_id} in {violations:#?}"
    );
}

#[test]
fn checked_in_packet_validates_and_projects_for_support_export() {
    let packet = current_fitness_packet_alpha().expect("checked-in packet validates");
    assert_eq!(
        packet.record_kind,
        PROTECTED_FITNESS_PACKET_ALPHA_RECORD_KIND
    );
    assert_eq!(packet.protected_function_rows.len(), 6);
    assert_eq!(packet.overall_result, "evidence_stale");

    let support_projection = packet.support_bundle_projection();
    let release_projection = packet.release_evidence_projection();
    assert_eq!(
        support_projection.record_kind,
        PROTECTED_FITNESS_PACKET_ALPHA_RECORD_KIND
    );
    assert_eq!(
        release_projection.record_kind,
        PROTECTED_FITNESS_PACKET_ALPHA_RECORD_KIND
    );
    assert_eq!(
        support_projection.record_kind,
        release_projection.record_kind
    );
    assert!(support_projection.raw_private_material_excluded);
    assert_eq!(support_projection.protected_function_count, 6);
    assert_eq!(support_projection.result_counts["passing"], 2);
    assert_eq!(support_projection.result_counts["evidence_stale"], 2);
    assert!(support_projection
        .source_refs
        .contains(&"dashboards/m1/hot_path_fitness.json".to_owned()));
}

#[test]
fn release_evidence_packet_carries_typed_fitness_projection() {
    let release_packet = current_alpha_artifact_graph()
        .expect("artifact graph parses")
        .release_evidence_packet(
            "release.evidence.alpha.seed.preview",
            "2026-05-14T07:30:00Z",
        );
    let fitness_projection = release_packet
        .protected_fitness_packet
        .expect("fitness projection is linked");

    assert_eq!(
        release_packet.record_kind,
        ALPHA_RELEASE_EVIDENCE_PACKET_RECORD_KIND
    );
    assert_eq!(
        fitness_projection.record_kind,
        PROTECTED_FITNESS_PACKET_ALPHA_RECORD_KIND
    );
    assert_eq!(
        fitness_projection.packet_ref,
        "artifacts/release/protected_fitness_packet_alpha.yaml"
    );
    assert_eq!(fitness_projection.overall_result, "evidence_stale");
}

#[test]
fn missing_owner_on_waived_row_returns_typed_error() {
    let packet_yaml = PACKET_YAML
        .replace(
            "    owner_dri: \"@ahmeddyounis\"\n    owning_lane: benchmark_lab\n    co_owning_lane: aureline-render\n    waiver_authority_ref: performance_council\n    current_result: passing",
            "    owning_lane: benchmark_lab\n    co_owning_lane: aureline-render\n    waiver_authority_ref: performance_council\n    current_result: waived",
        )
        .replace(
            "      waiver_state: no_active_waiver\n      waiver_record_ref: null\n      waiver_authority_ref: performance_council\n      expiry_at: null\n      summary: No active waiver; row is passing on current dashboard evidence.",
            "      waiver_state: active_waiver\n      waiver_record_ref: waiver.protected_fitness.first_paint\n      waiver_authority_ref: performance_council\n      expiry_at: \"2026-06-01T00:00:00Z\"\n      summary: Active waiver held by performance council until expiry.",
        );

    assert_invalid_check(&packet_yaml, "protected_function_rows.owner_dri");
}

#[test]
fn expired_active_waiver_returns_typed_error() {
    let packet_yaml = PACKET_YAML.replace(
        "      waiver_state: no_active_waiver\n      waiver_record_ref: null\n      waiver_authority_ref: performance_council\n      expiry_at: null\n      summary: No active waiver; row is passing on current dashboard evidence.",
        "      waiver_state: active_waiver\n      waiver_record_ref: waiver.protected_fitness.first_paint\n      waiver_authority_ref: performance_council\n      expiry_at: \"2026-05-01T00:00:00Z\"\n      summary: Active waiver held by performance council until expiry.",
    );

    assert_invalid_check(
        &packet_yaml,
        "protected_function_rows.waiver.active_expired",
    );
}

#[test]
fn regression_history_must_track_result_source() {
    let packet_yaml = PACKET_YAML.replacen(
        "      history_source_ref: dashboards/m1/hot_path_fitness.json",
        "      history_source_ref: dashboards/m1/other_hot_path_history.json",
        1,
    );

    assert_invalid_check(
        &packet_yaml,
        "protected_function_rows.regression_history.history_source_ref",
    );
}
