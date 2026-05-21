//! Protected tests for the frozen beta release-candidate fitness packet.

use aureline_support::fitness::{
    current_fitness_packet_beta, FitnessFunctionCatalog, FitnessPacketAlphaViolation,
    FitnessPacketBeta, FitnessStateRows, PROTECTED_FITNESS_PACKET_BETA_RECORD_KIND,
    PROTECTED_FITNESS_PACKET_BETA_SCHEMA_VERSION,
};

const BETA_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/protected_fitness_packet_beta.yaml"
));
const BASE_YAML: &str = include_str!(concat!(
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

/// Loads the checked-in beta packet (with its base wired) for in-test mutation.
fn loaded_beta() -> FitnessPacketBeta {
    let mut packet =
        serde_yaml::from_str::<FitnessPacketBeta>(BETA_YAML).expect("beta packet parses");
    packet.base = serde_yaml::from_str(BASE_YAML).expect("base packet parses");
    packet
}

fn catalog() -> FitnessFunctionCatalog {
    serde_yaml::from_str(CATALOG_YAML).expect("catalog parses")
}

fn state_rows() -> FitnessStateRows {
    serde_yaml::from_str(STATE_ROWS_YAML).expect("state rows parse")
}

fn validate(packet: &FitnessPacketBeta) -> Vec<FitnessPacketAlphaViolation> {
    packet.validate_with_catalogs(&catalog(), &state_rows())
}

fn assert_has_check(violations: &[FitnessPacketAlphaViolation], expected: &str) {
    assert!(
        violations.iter().any(|v| v.check_id == expected),
        "missing {expected} in {violations:#?}"
    );
}

#[test]
fn checked_in_beta_packet_validates_and_summarizes() {
    let packet = current_fitness_packet_beta().expect("checked-in beta packet validates");
    assert_eq!(
        packet.record_kind,
        PROTECTED_FITNESS_PACKET_BETA_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        PROTECTED_FITNESS_PACKET_BETA_SCHEMA_VERSION
    );
    assert_eq!(
        packet.base_packet_ref,
        "artifacts/release/protected_fitness_packet_alpha.yaml"
    );
    // Five catalog-linked rows carry a release-candidate bar; the support
    // scorecard row carries no catalog row and therefore no bar.
    assert_eq!(packet.release_candidate_thresholds.len(), 5);
    assert_eq!(packet.base.protected_function_rows.len(), 6);

    let summary = packet.release_candidate_summary();
    assert_eq!(
        summary.record_kind,
        PROTECTED_FITNESS_PACKET_BETA_RECORD_KIND
    );
    assert_eq!(summary.threshold_count, 5);
    assert_eq!(summary.within_bar_count, 3);
    assert_eq!(summary.over_bar_count, 2);
    assert_eq!(summary.over_bar_held_by_active_waiver_count, 0);
    assert_eq!(summary.expired_waiver_count, 0);
}

#[test]
fn over_threshold_without_active_waiver_fails() {
    let mut packet = loaded_beta();
    // First paint is passing and within its bar with no active waiver. Flip
    // it over the bar: a passing metric over its release-candidate bar with
    // no active waiver must fail.
    let threshold = packet
        .release_candidate_thresholds
        .iter_mut()
        .find(|t| t.protected_function_ref == "ff.first_paint")
        .expect("first-paint threshold present");
    threshold.within_release_candidate_bar = false;

    let violations = validate(&packet);
    assert_has_check(
        &violations,
        "release_candidate_thresholds.over_threshold_without_active_waiver",
    );
}

#[test]
fn over_threshold_held_by_active_waiver_is_visible() {
    let mut packet = loaded_beta();
    // Put first paint over its bar but held open by an active, visible waiver
    // rendered as waived. This is the admissible active-waiver-visible state.
    {
        let threshold = packet
            .release_candidate_thresholds
            .iter_mut()
            .find(|t| t.protected_function_ref == "ff.first_paint")
            .expect("first-paint threshold present");
        threshold.within_release_candidate_bar = false;
    }
    {
        let row = packet
            .base
            .protected_function_rows
            .iter_mut()
            .find(|r| r.protected_function_ref == "ff.first_paint")
            .expect("first-paint base row present");
        row.current_result = "waived".to_owned();
        row.last_passed_at = None;
        row.waiver.waiver_state = "active_waiver".to_owned();
        row.waiver.waiver_record_ref = Some("waiver.protected_fitness.first_paint".to_owned());
        row.waiver.expiry_at = Some("2026-08-01T00:00:00Z".to_owned());
    }

    let violations = validate(&packet);
    assert!(
        violations.is_empty(),
        "an over-bar metric held by a visible active waiver must validate, got {violations:#?}"
    );
}

#[test]
fn over_threshold_active_waiver_must_stay_visible() {
    let mut packet = loaded_beta();
    {
        let threshold = packet
            .release_candidate_thresholds
            .iter_mut()
            .find(|t| t.protected_function_ref == "ff.first_paint")
            .expect("first-paint threshold present");
        threshold.within_release_candidate_bar = false;
    }
    {
        // Active waiver but the waiver record is hidden: the row must not pass
        // the visibility check.
        let row = packet
            .base
            .protected_function_rows
            .iter_mut()
            .find(|r| r.protected_function_ref == "ff.first_paint")
            .expect("first-paint base row present");
        row.current_result = "waived".to_owned();
        row.last_passed_at = None;
        row.waiver.waiver_state = "active_waiver".to_owned();
        row.waiver.waiver_record_ref = None;
        row.waiver.expiry_at = Some("2026-08-01T00:00:00Z".to_owned());
    }

    let violations = validate(&packet);
    assert_has_check(
        &violations,
        "release_candidate_thresholds.active_waiver_visibility",
    );
}

#[test]
fn expired_waiver_degrades_rather_than_passing() {
    let mut packet = loaded_beta();
    // An expired waiver on a row that still renders passing must degrade.
    let row = packet
        .base
        .protected_function_rows
        .iter_mut()
        .find(|r| r.protected_function_ref == "ff.first_paint")
        .expect("first-paint base row present");
    row.waiver.waiver_state = "expired_waiver".to_owned();
    row.waiver.waiver_record_ref = Some("waiver.protected_fitness.first_paint".to_owned());
    row.waiver.expiry_at = Some("2026-05-01T00:00:00Z".to_owned());

    let violations = validate(&packet);
    assert_has_check(
        &violations,
        "release_candidate_thresholds.expired_waiver_degrades",
    );
}

#[test]
fn release_candidate_bar_required_per_catalog_row() {
    let mut packet = loaded_beta();
    packet
        .release_candidate_thresholds
        .retain(|t| t.protected_function_ref != "ff.input_to_paint");

    let violations = validate(&packet);
    assert_has_check(&violations, "release_candidate_thresholds.coverage");
}

#[test]
fn threshold_mode_must_match_catalog() {
    let mut packet = loaded_beta();
    let threshold = packet
        .release_candidate_thresholds
        .iter_mut()
        .find(|t| t.protected_function_ref == "ff.first_paint")
        .expect("first-paint threshold present");
    threshold.threshold_mode = "boolean_gate".to_owned();

    let violations = validate(&packet);
    assert_has_check(&violations, "release_candidate_thresholds.threshold_mode");
}

#[test]
fn unknown_comparator_is_rejected() {
    let mut packet = loaded_beta();
    let threshold = packet
        .release_candidate_thresholds
        .iter_mut()
        .find(|t| t.protected_function_ref == "ff.first_paint")
        .expect("first-paint threshold present");
    threshold.comparator = "vibes_based_gate".to_owned();

    let violations = validate(&packet);
    assert_has_check(&violations, "release_candidate_thresholds.comparator");
}
