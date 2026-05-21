//! Protected tests binding the typed maintenance-control packet to the checked-in
//! artifact, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in packet; the capture cross-check proves the typed
//! model and the Python gate agree on the publication verdict, the
//! hotfix/backport/correction-train/support-window coverage counts, and the
//! packet-freshness counts; the negative cases mutate a parsed copy and the checked-in
//! fixtures to prove that a lane which fails to narrow, a governed lane riding a breached
//! packet, a control backed wider than its public claim's ceiling, a disordered support
//! window, and a publication verdict that disagrees with the firing rules all fail
//! validation.

use std::path::{Path, PathBuf};

use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};
use aureline_release::maintenance_control_packet::{
    current_maintenance_control_packet, ControlState, LaneKind, MaintenanceControlPacket,
    MaintenanceControlPacketViolation, MAINTENANCE_CONTROL_PACKET_RECORD_KIND,
    MAINTENANCE_CONTROL_PACKET_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/maintenance_control_packet_validation_capture.json"
));

fn packet() -> MaintenanceControlPacket {
    current_maintenance_control_packet()
        .expect("checked-in maintenance-control packet parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        MAINTENANCE_CONTROL_PACKET_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, MAINTENANCE_CONTROL_PACKET_RECORD_KIND);
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "checked-in packet must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_lane_kind() {
    let packet = packet();
    for kind in LaneKind::ALL {
        assert!(
            !packet.rows_for_kind(kind).is_empty(),
            "lane kind {} must have at least one control row",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_lane() {
    let packet = packet();
    assert!(!packet.release_blocking_lane_refs.is_empty());
    let covered: Vec<&str> = packet
        .release_blocking_rows()
        .into_iter()
        .map(|row| row.lane_ref.as_str())
        .collect();
    for declared in &packet.release_blocking_lane_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking row"
        );
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let packet = packet();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(packet.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_lanes"].as_u64().unwrap() as usize,
        packet.rows.len(),
        "capture lane count must match the model"
    );
    assert_eq!(
        summary["lanes_governed_stable"].as_u64().unwrap() as usize,
        packet.rows_governed_stable().len(),
        "capture governed count must match the model"
    );
    for (key, kind) in [
        ("hotfix_lanes", LaneKind::Hotfix),
        ("backport_lanes", LaneKind::Backport),
        ("correction_train_lanes", LaneKind::CorrectionTrain),
        ("support_window_lanes", LaneKind::SupportWindow),
    ] {
        assert_eq!(
            summary[key].as_u64().unwrap() as usize,
            packet.rows_for_kind(kind).len(),
            "capture {key} must match the model"
        );
    }
    assert_eq!(
        summary["packets_breached"].as_u64().unwrap() as usize,
        packet.computed_summary().packets_breached,
        "capture breached-packet count must match the model"
    );

    let captured_decision = capture["publication"]["decision"].as_str().unwrap();
    assert_eq!(
        captured_decision,
        packet.publication.decision.as_str(),
        "capture publication decision must match the model"
    );
    assert_eq!(
        packet.publication.decision,
        packet.computed_publication_decision()
    );

    for drill in capture["negative_drills"].as_array().unwrap() {
        assert_eq!(
            drill["status"].as_str(),
            Some("passed"),
            "frozen capture drill {} must have passed",
            drill["drill_id"]
        );
    }
    let fixtures = capture["fixture_cases"].as_array().unwrap();
    assert!(!fixtures.is_empty(), "capture must record fixture cases");
    for case in fixtures {
        assert_eq!(
            case["status"].as_str(),
            Some("passed"),
            "frozen capture fixture case {} must have passed",
            case["case_id"]
        );
    }
}

#[test]
fn packet_narrows_a_lane_under_a_still_stable_claim() {
    let packet = packet();
    // A release-blocking lane whose public claim is still published Stable but is itself
    // ungoverned — the maintenance-level truth beneath an optimistic claim.
    let narrowed = packet.rows.iter().find(|row| {
        row.release_blocking
            && row.claim_holds_stable()
            && !row.governs_stable()
            && row.control_state != ControlState::UngovernedClaimNarrowed
    });
    assert!(
        narrowed.is_some(),
        "the packet must narrow at least one release-blocking lane under a still-stable claim"
    );
}

#[test]
fn narrowing_row_that_does_not_narrow_fails() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| {
            row.control_state == ControlState::UngovernedStale
                && row.claim_label == StableClaimLevel::Stable
        })
        .expect("packet has an ungoverned-stale row under a stable ceiling");
    row.controlled_label = StableClaimLevel::Stable;
    packet.summary = packet.computed_summary();
    packet.publication.decision = packet.computed_publication_decision();
    packet.publication.blocking_rule_ids = packet.computed_blocking_rule_ids();
    packet.publication.blocking_lane_ids = packet.computed_blocking_lane_ids();

    assert!(
        packet
            .validate()
            .iter()
            .any(|v| matches!(v, MaintenanceControlPacketViolation::ControlledLabelNotNarrowed { .. })),
        "a lane that is not governed must narrow below the cutline"
    );
}

#[test]
fn governed_row_on_a_breached_packet_fails() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.holds_control())
        .expect("packet has a governed row");
    row.control_packet.slo_state = FreshnessSloState::Breached;
    packet.summary = packet.computed_summary();

    assert!(
        packet
            .validate()
            .iter()
            .any(|v| matches!(v, MaintenanceControlPacketViolation::HeldOnStalePacket { .. })),
        "a governed row may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn publication_proceed_while_a_rule_fires_fails() {
    let mut packet = packet();
    packet.publication.decision = PromotionDecision::Proceed;

    assert!(
        packet.validate().iter().any(|v| matches!(
            v,
            MaintenanceControlPacketViolation::PublicationDecisionInconsistent { .. }
        )),
        "publication must not proceed while a blocking rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/release/maintenance_control_packet");
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        let candidate: MaintenanceControlPacket =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
    }
}
