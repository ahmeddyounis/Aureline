//! Protected tests for the correction-train / hotfix / backport packet model.
//!
//! The positive fixture is the frozen, checked-in correction packet; the
//! negative cases mutate a parsed copy to prove that a packet missing rollback
//! linkage or affected-build scope fails validation.

use std::collections::BTreeSet;

use aureline_release::correction_train::{
    BackportDecision, CorrectionTrainPacket, TriageLane, CORRECTION_TRAIN_PACKET_RECORD_KIND,
    CORRECTION_TRAIN_PACKET_SCHEMA_VERSION,
};

const PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m3/correction_train/packet.json"
));
const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m3/captures/correction_train_validation_capture.json"
));

fn parse_packet() -> CorrectionTrainPacket {
    serde_json::from_str(PACKET_JSON).expect("checked-in correction packet parses into the model")
}

fn check_ids(packet: &CorrectionTrainPacket) -> BTreeSet<String> {
    packet
        .validate()
        .into_iter()
        .map(|violation| violation.check_id)
        .collect()
}

#[test]
fn checked_in_packet_parses_and_validates() {
    let packet = parse_packet();

    assert_eq!(packet.record_kind, CORRECTION_TRAIN_PACKET_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        CORRECTION_TRAIN_PACKET_SCHEMA_VERSION
    );
    assert!(!packet.exact_build_identity_refs.is_empty());
    assert_eq!(packet.correction_items.len(), 4);

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "checked-in correction packet must validate cleanly: {violations:#?}"
    );
}

#[test]
fn checked_in_packet_exercises_every_triage_lane() {
    let packet = parse_packet();
    let lanes = packet.observed_lanes();
    let expected = BTreeSet::from([
        TriageLane::Hotfix,
        TriageLane::Backport,
        TriageLane::CorrectionTrainOnly,
        TriageLane::NextCycle,
    ]);
    assert_eq!(lanes, expected, "canonical packet must exercise every lane");
}

#[test]
fn model_matches_frozen_validation_capture() {
    let packet = parse_packet();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    let coverage = &capture["coverage"];
    assert_eq!(
        coverage["correction_item_count"].as_u64().unwrap() as usize,
        packet.correction_items.len(),
        "capture item count must match the model"
    );
    assert_eq!(capture["passed"].as_bool(), Some(true));

    let yes_count = packet
        .correction_items
        .iter()
        .flat_map(|item| &item.backport_matrix)
        .filter(|row| row.decision == BackportDecision::Yes)
        .count();
    assert_eq!(
        coverage["backport_decision_counts"]["yes"]
            .as_u64()
            .unwrap() as usize,
        yes_count,
        "capture yes-decision count must match the model"
    );
}

#[test]
fn packet_missing_rollback_linkage_fails() {
    let mut packet = parse_packet();
    // Strip the rollback target from the security hotfix row, which ships on a
    // release lane and therefore must keep rollback linkage.
    let item = packet
        .correction_items
        .iter_mut()
        .find(|item| item.triage.lane_decision == TriageLane::Hotfix)
        .expect("packet has a hotfix row");
    item.scope.rollback_target_ref = None;

    assert!(
        check_ids(&packet).contains("correction_scope.rollback_target_missing"),
        "removing rollback linkage must fail validation"
    );
}

#[test]
fn packet_missing_affected_build_scope_fails() {
    let mut packet = parse_packet();
    packet.correction_items[0]
        .scope
        .affected_release_lines
        .clear();

    assert!(
        check_ids(&packet).contains("correction_scope.affected_build_scope_missing"),
        "removing affected release lines must fail validation"
    );
}

#[test]
fn packet_missing_exact_build_identities_fails() {
    let mut packet = parse_packet();
    packet.exact_build_identity_refs.clear();

    assert!(
        check_ids(&packet).contains("packet.exact_build_identity_refs_missing"),
        "an empty affected-build identity set must fail validation"
    );
}

#[test]
fn security_row_demoted_off_hotfix_fails() {
    let mut packet = parse_packet();
    let item = packet
        .correction_items
        .iter_mut()
        .find(|item| item.issue_class == "security_policy_escape")
        .expect("packet has a security row");
    item.triage.lane_decision = TriageLane::NextCycle;

    let ids = check_ids(&packet);
    assert!(
        ids.contains("triage.security_or_trust_requires_hotfix"),
        "demoting a security defect off the hotfix lane must fail validation"
    );
}

#[test]
fn affected_supported_line_without_decision_fails() {
    let mut packet = parse_packet();
    // Find an affected supported (stable/lts) backport row and blank its decision.
    let row = packet
        .correction_items
        .iter_mut()
        .flat_map(|item| &mut item.backport_matrix)
        .find(|row| row.affected && matches!(row.support_line_class.as_str(), "stable" | "lts"))
        .expect("packet has an affected supported-line row");
    row.decision = BackportDecision::NotApplicable;

    assert!(
        check_ids(&packet).contains("backport_matrix.affected_line_no_decision"),
        "an affected supported line marked not_applicable must fail validation"
    );
}
