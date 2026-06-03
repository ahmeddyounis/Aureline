//! Protected tests binding the typed security-response packet to the checked-in
//! artifact, the frozen CI validation capture, and the negative fixtures.
//!
//! The positive case is the checked-in packet; the capture cross-check proves the
//! typed model and the Python gate agree on the publication verdict, the
//! security-response/advisory-publication/CVE-GHSA-publication/emergency-disable/
//! mirror-offline-drill coverage counts, and the packet-freshness counts; the
//! negative cases mutate a parsed copy and the checked-in fixtures to prove that
//! a lane which fails to narrow, a ready row riding a breached packet, a row
//! backed wider than its public claim's ceiling, an emergency-disable lane with
//! an unsatisfied control, a mirror/offline-drill lane with an unverified
//! checkpoint, and a publication verdict that disagrees with the firing rules all
//! fail validation.

use std::path::{Path, PathBuf};

use aureline_release::finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills::{
    current_security_response_packet, EmergencyControl, ResponseKind,
    ResponseState, SecurityResponsePacket, SecurityResponsePacketViolation,
    SECURITY_RESPONSE_PACKET_RECORD_KIND, SECURITY_RESPONSE_PACKET_SCHEMA_VERSION,
};
use aureline_release::stable_claim_manifest::FreshnessSloState;
use aureline_release::stable_claim_matrix::{PromotionDecision, StableClaimLevel};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills_validation_capture.json"
));

fn packet() -> SecurityResponsePacket {
    current_security_response_packet()
        .expect("checked-in security-response packet parses into the model")
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
        SECURITY_RESPONSE_PACKET_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, SECURITY_RESPONSE_PACKET_RECORD_KIND);
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "checked-in packet must validate cleanly: {violations:#?}"
    );
}

#[test]
fn covers_every_response_kind() {
    let packet = packet();
    for kind in ResponseKind::ALL {
        assert!(
            !packet.rows_for_kind(kind).is_empty(),
            "response kind {} must have at least one row",
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
    let capture: serde_json::Value = serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(packet.as_of.as_str()));

    let summary = &capture["summary"];
    assert_eq!(
        summary["total_rows"].as_u64().unwrap() as usize,
        packet.rows.len(),
        "capture row count must match the model"
    );
    assert_eq!(
        summary["rows_ready"].as_u64().unwrap() as usize,
        packet.rows_ready().len(),
        "capture ready count must match the model"
    );
    for (key, kind) in [
        ("security_response_rows", ResponseKind::SecurityResponse),
        ("advisory_publication_rows", ResponseKind::AdvisoryPublication),
        ("cve_ghsa_publication_rows", ResponseKind::CveGhsaPublication),
        ("emergency_disable_rows", ResponseKind::EmergencyDisable),
        ("mirror_offline_drill_rows", ResponseKind::MirrorOfflineDrill),
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
    let narrowed = packet.rows.iter().find(|row| {
        row.release_blocking
            && row.claim_holds_stable()
            && !row.holds_stable()
            && row.response_state != ResponseState::NotReadyClaimNarrowed
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
            row.response_state == ResponseState::NotReadyStale
                && row.claim_label == StableClaimLevel::Stable
        })
        .expect("packet has a not-ready-stale row under a stable ceiling");
    row.effective_label = StableClaimLevel::Stable;
    packet.summary = packet.computed_summary();
    packet.publication.decision = packet.computed_publication_decision();
    packet.publication.blocking_rule_ids = packet.computed_blocking_rule_ids();
    packet.publication.blocking_entry_ids = packet.computed_blocking_entry_ids();

    assert!(
        packet.validate().iter().any(|v| matches!(
            v,
            SecurityResponsePacketViolation::EffectiveLabelNotNarrowed { .. }
        )),
        "a lane that is not ready must narrow below the cutline"
    );
}

#[test]
fn ready_row_on_a_breached_packet_fails() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.holds_label())
        .expect("packet has a ready row");
    row.response_packet.slo_state = FreshnessSloState::Breached;
    packet.summary = packet.computed_summary();

    assert!(
        packet.validate().iter().any(|v| matches!(
            v,
            SecurityResponsePacketViolation::HeldOnStalePacket { .. }
        )),
        "a ready row may not ride a packet outside its freshness SLO"
    );
}

#[test]
fn emergency_disable_with_unsatisfied_control_fails() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.response_kind == ResponseKind::EmergencyDisable && row.holds_label())
        .expect("packet has a ready emergency-disable row");
    row.emergency_controls.push(EmergencyControl {
        control_id: "control:unsatisfied_test".to_owned(),
        title: "Unsatisfied test control".to_owned(),
        control_ref: "docs/security/test.md".to_owned(),
        satisfied: false,
    });
    packet.summary = packet.computed_summary();

    assert!(
        packet.validate().iter().any(|v| matches!(
            v,
            SecurityResponsePacketViolation::HeldWithUnsatisfiedEmergencyControl { .. }
        )),
        "a ready emergency-disable row may not carry an unsatisfied control"
    );
}

#[test]
fn publication_proceed_while_a_rule_fires_fails() {
    let mut packet = packet();
    packet.publication.decision = PromotionDecision::Proceed;

    assert!(
        packet.validate().iter().any(|v| matches!(
            v,
            SecurityResponsePacketViolation::PublicationDecisionInconsistent { .. }
        )),
        "publication must not proceed while a blocking rule fires"
    );
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join(
        "fixtures/release/m4/finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills",
    );
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
        let candidate: SecurityResponsePacket =
            serde_json::from_str(&raw).unwrap_or_else(|_| panic!("fixture {file} parses"));
        assert!(
            !candidate.validate().is_empty(),
            "fixture {file} must be rejected by the typed model"
        );
    }
}

