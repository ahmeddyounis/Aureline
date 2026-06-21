//! Fixture replay for the localized-profile qualification claim-status packet.

use std::path::{Path, PathBuf};

use aureline_i18n::{
    seeded_localized_claim_status_packet, LaneEvidenceState, LocalizedClaimStatusPacket,
    MatrixGateState, ProfileClaimClass, QualificationLaneKind,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/i18n/m5-localized-profile-qualification/claim_status.json")
}

fn load_packet() -> LocalizedClaimStatusPacket {
    let path = fixture_path();
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn fixture_matches_seeded_packet() {
    let from_file = load_packet();
    let from_code = seeded_localized_claim_status_packet();
    assert_eq!(from_file, from_code);
    from_file
        .validate()
        .expect("localized claim status packet validates");
}

#[test]
fn fixture_holds_one_green_localized_claim_and_publishes_known_limits() {
    let packet = load_packet();
    assert!(packet.summary.claimed_localized_profiles >= 1);
    assert_eq!(packet.summary.promotion_state, MatrixGateState::Green);
    assert_eq!(packet.summary.blocked_profiles, 0);
    assert_eq!(
        packet.summary.published_known_limits,
        packet.known_limits.len()
    );
    assert!(!packet.known_limits.is_empty());
}

#[test]
fn fixture_proves_evidence_staleness_drops_a_green_claim() {
    let packet = load_packet();
    let es = "profile:m5:es-MX:desktop";
    assert_eq!(
        packet.profile(es).unwrap().effective_claim_class,
        ProfileClaimClass::ClaimedLocalized
    );

    let stale = packet.with_lane_state(
        es,
        QualificationLaneKind::TranslatedHelpParity,
        LaneEvidenceState::Stale,
    );
    assert_ne!(
        stale.profile(es).unwrap().effective_claim_class,
        ProfileClaimClass::ClaimedLocalized
    );
    assert_eq!(
        stale.profile(es).unwrap().gate_state,
        MatrixGateState::Narrowed
    );
}
