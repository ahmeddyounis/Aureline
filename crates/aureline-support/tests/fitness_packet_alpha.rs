//! Protected tests for the alpha protected fitness packet consumer.

use std::fs::File;

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

#[test]
fn in_memory_fitness_documents_are_size_bounded() {
    let oversized = " ".repeat(4 * 1024 * 1024 + 1);

    let error = FitnessPacketAlpha::from_yaml_documents(&oversized, CATALOG_YAML, STATE_ROWS_YAML)
        .expect_err("oversized packet must fail");

    assert!(matches!(
        error,
        FitnessPacketAlphaError::ResourceLimitExceeded {
            resource: "input bytes",
            ..
        }
    ));
}

#[test]
fn in_memory_fitness_documents_bound_parsed_shape_before_typed_projection() {
    let oversized_sequence = format!("rows:\n{}", "  - value\n".repeat(4_097));
    let error =
        FitnessPacketAlpha::from_yaml_documents(&oversized_sequence, CATALOG_YAML, STATE_ROWS_YAML)
            .expect_err("oversized sequence must fail");
    assert!(matches!(
        error,
        FitnessPacketAlphaError::ResourceLimitExceeded {
            resource: "sequence entries",
            ..
        }
    ));

    let oversized_scalar = format!("value: {}\n", "x".repeat(256 * 1024 + 1));
    let error =
        FitnessPacketAlpha::from_yaml_documents(&oversized_scalar, CATALOG_YAML, STATE_ROWS_YAML)
            .expect_err("oversized scalar must fail");
    assert!(matches!(
        error,
        FitnessPacketAlphaError::ResourceLimitExceeded {
            resource: "scalar bytes",
            ..
        }
    ));
}

#[test]
fn malformed_yaml_does_not_echo_private_scalar_values() {
    let private_value = "private-customer-value-must-not-escape";
    let malformed = format!("schema_version: {private_value}\n");

    let error = FitnessPacketAlpha::from_yaml_documents(&malformed, CATALOG_YAML, STATE_ROWS_YAML)
        .expect_err("malformed packet must fail");

    assert!(matches!(error, FitnessPacketAlphaError::PacketYaml(_)));
    assert!(!error.to_string().contains(private_value));
}

#[test]
fn file_load_errors_do_not_disclose_host_paths() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let missing = directory
        .path()
        .join("private-customer-fitness-packet.yaml");
    let missing_text = missing.to_string_lossy().into_owned();

    let error = FitnessPacketAlpha::from_paths(&missing, &missing, &missing)
        .expect_err("missing input must fail");

    assert!(matches!(error, FitnessPacketAlphaError::Io { .. }));
    assert!(!error.to_string().contains(&missing_text));
    assert!(!error.to_string().contains("private-customer"));
}

#[test]
fn oversized_file_inputs_fail_before_yaml_parsing() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let packet_path = directory.path().join("packet.yaml");
    let packet_file = File::create(&packet_path).expect("create sparse packet");
    packet_file
        .set_len(4 * 1024 * 1024 + 1)
        .expect("extend sparse packet");

    let error = FitnessPacketAlpha::from_paths(&packet_path, &packet_path, &packet_path)
        .expect_err("oversized input must fail");

    assert!(matches!(
        error,
        FitnessPacketAlphaError::ResourceLimitExceeded {
            resource: "input bytes",
            ..
        }
    ));
}

#[cfg(unix)]
#[test]
fn symbolic_link_inputs_fail_closed() {
    use std::os::unix::fs::symlink;

    let directory = tempfile::tempdir().expect("temporary directory");
    let target = directory.path().join("target.yaml");
    let link = directory.path().join("packet.yaml");
    std::fs::write(&target, PACKET_YAML).expect("write target");
    symlink(&target, &link).expect("create symlink");

    let error = FitnessPacketAlpha::from_paths(&link, &target, &target)
        .expect_err("symbolic link must fail");

    assert!(matches!(
        error,
        FitnessPacketAlphaError::UnsafeFileType { .. }
    ));
}
