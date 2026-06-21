//! Unit tests for the localized claim-status qualification packet.

use super::*;

const ES: &str = "profile:m5:es-MX:desktop";
const JA: &str = "profile:m5:ja-JP:desktop";
const AR: &str = "profile:m5:ar-SA:desktop";

#[test]
fn seeded_packet_validates() {
    seeded_localized_claim_status_packet()
        .validate()
        .expect("seeded claim status validates");
}

#[test]
fn seeded_packet_is_promotion_safe_with_one_green_claim() {
    let packet = seeded_localized_claim_status_packet();
    assert_eq!(packet.summary.promotion_state, MatrixGateState::Green);
    assert_eq!(packet.summary.blocked_profiles, 0);
    assert!(packet.summary.claimed_localized_profiles >= 1);
}

#[test]
fn flagship_profile_holds_a_green_localized_claim() {
    let packet = seeded_localized_claim_status_packet();
    let es = packet.profile(ES).expect("flagship profile exists");
    assert_eq!(es.gate_state, MatrixGateState::Green);
    assert_eq!(
        es.effective_claim_class,
        ProfileClaimClass::ClaimedLocalized
    );
    assert!(!es.narrowed);
    assert!(!es.blocks_promotion);
    assert!(packet.known_limits_for(ES).is_empty());
}

#[test]
fn stale_translated_help_narrows_the_japanese_claim() {
    let packet = seeded_localized_claim_status_packet();
    let ja = packet.profile(JA).expect("japanese profile exists");

    assert_eq!(ja.intended_claim_class, ProfileClaimClass::ClaimedLocalized);
    assert_eq!(ja.gate_state, MatrixGateState::Narrowed);
    assert_eq!(
        ja.effective_claim_class,
        ProfileClaimClass::SourceLanguageFallbackOnly
    );
    assert!(ja.narrowed);
    assert!(!ja.blocks_promotion);
    assert_eq!(
        ja.affected_lane_kinds,
        vec![QualificationLaneKind::TranslatedHelpParity]
    );

    let limits = packet.known_limits_for(JA);
    assert_eq!(limits.len(), 1);
    assert_eq!(limits[0].cause, LaneNarrowCause::EvidenceStale);
    assert_eq!(limits[0].gate_state, MatrixGateState::Narrowed);
    assert_eq!(
        limits[0].lane_kind,
        QualificationLaneKind::TranslatedHelpParity
    );
}

#[test]
fn stale_rtl_evidence_narrows_the_arabic_claim() {
    let packet = seeded_localized_claim_status_packet();
    let ar = packet.profile(AR).expect("arabic profile exists");

    assert_eq!(ar.text_direction, TextDirection::RightToLeft);
    assert_eq!(ar.gate_state, MatrixGateState::Narrowed);
    assert_eq!(ar.affected_lane_kinds, vec![QualificationLaneKind::RtlBidi]);
}

#[test]
fn a_green_claim_cannot_stay_green_once_a_lane_goes_stale() {
    let packet = seeded_localized_claim_status_packet();
    assert_eq!(
        packet.profile(ES).unwrap().gate_state,
        MatrixGateState::Green
    );

    for lane in QualificationLaneKind::all() {
        let narrowed = packet.with_lane_state(ES, lane, LaneEvidenceState::Stale);
        narrowed.validate().expect("narrowed packet validates");
        let es = narrowed.profile(ES).unwrap();
        assert_eq!(
            es.gate_state,
            MatrixGateState::Narrowed,
            "{lane:?} stale must narrow the claim"
        );
        assert_eq!(
            es.effective_claim_class,
            ProfileClaimClass::SourceLanguageFallbackOnly
        );
        assert!(es.narrowed);
        assert!(!es.blocks_promotion);
        assert_eq!(narrowed.known_limits_for(ES).len(), 1);
        assert_eq!(
            narrowed.known_limits_for(ES)[0].cause,
            LaneNarrowCause::EvidenceStale
        );
    }
}

#[test]
fn missing_evidence_narrows_the_claim() {
    let packet = seeded_localized_claim_status_packet();
    let narrowed = packet.with_lane_state(
        ES,
        QualificationLaneKind::Pseudolocalization,
        LaneEvidenceState::Missing,
    );
    narrowed.validate().expect("narrowed packet validates");
    let es = narrowed.profile(ES).unwrap();
    assert_eq!(es.gate_state, MatrixGateState::Narrowed);
    assert_eq!(
        narrowed.known_limits_for(ES)[0].cause,
        LaneNarrowCause::EvidenceMissing
    );
}

#[test]
fn a_failing_lane_blocks_promotion() {
    let packet = seeded_localized_claim_status_packet();
    let blocked = packet.with_lane_state(
        ES,
        QualificationLaneKind::ImeComposition,
        LaneEvidenceState::CurrentFailing,
    );
    blocked
        .validate()
        .expect("blocked packet validates structurally");

    let es = blocked.profile(ES).unwrap();
    assert_eq!(es.gate_state, MatrixGateState::Blocked);
    assert!(es.blocks_promotion);
    assert_ne!(
        es.effective_claim_class,
        ProfileClaimClass::ClaimedLocalized
    );

    assert_eq!(blocked.summary.blocked_profiles, 1);
    assert_eq!(blocked.summary.promotion_state, MatrixGateState::Blocked);

    let limits = blocked.known_limits_for(ES);
    assert_eq!(limits.len(), 1);
    assert_eq!(limits[0].cause, LaneNarrowCause::EvidenceFailing);
    assert_eq!(limits[0].gate_state, MatrixGateState::Blocked);
}

#[test]
fn a_bounded_waiver_keeps_a_lane_satisfied() {
    let packet = seeded_localized_claim_status_packet();
    let waived = packet.with_lane_state(
        ES,
        QualificationLaneKind::TextExpansion,
        LaneEvidenceState::WaivedBounded,
    );
    waived.validate().expect("waived packet validates");
    let es = waived.profile(ES).unwrap();
    assert_eq!(es.gate_state, MatrixGateState::Green);
    assert_eq!(
        es.effective_claim_class,
        ProfileClaimClass::ClaimedLocalized
    );
    assert_eq!(es.waived_lane_count, 1);
    assert!(waived.known_limits_for(ES).is_empty());
}

#[test]
fn every_known_limit_is_published_to_help_about_and_release_center() {
    let packet = seeded_localized_claim_status_packet();
    assert!(!packet.known_limits.is_empty());
    for limit in &packet.known_limits {
        assert!(limit.published_to.contains(&ConsumerKind::HelpAbout));
        assert!(limit.published_to.contains(&ConsumerKind::ReleaseCenter));
        assert!(!limit.affected_surface_families.is_empty());
        assert!(!limit.summary.is_empty());
    }
}

#[test]
fn claimed_localized_profiles_are_never_narrowed_or_blocked() {
    let packet = seeded_localized_claim_status_packet();
    let claimed = packet
        .claimed_profiles
        .iter()
        .filter(|p| p.effective_claim_class == ProfileClaimClass::ClaimedLocalized)
        .collect::<Vec<_>>();
    assert!(!claimed.is_empty());
    for profile in claimed {
        assert!(!profile.narrowed);
        assert!(!profile.blocks_promotion);
        assert_eq!(profile.gate_state, MatrixGateState::Green);
    }
}

#[test]
fn every_profile_carries_all_required_lanes() {
    let packet = seeded_localized_claim_status_packet();
    let required = QualificationLaneKind::all();
    for profile in &packet.claimed_profiles {
        for lane in &required {
            assert!(
                profile.lane_results.iter().any(|l| l.lane_kind == *lane),
                "{} missing lane {lane:?}",
                profile.profile_id
            );
        }
    }
}

#[test]
fn evidence_lanes_bind_to_upstream_truth_packets() {
    let packet = seeded_localized_claim_status_packet();
    assert_eq!(
        packet
            .evidence_lane_refs
            .get("translated_help_parity")
            .map(String::as_str),
        Some(M5_TRANSLATED_HELP_PARITY_REPORT_ID)
    );
    assert_eq!(
        packet
            .evidence_lane_refs
            .get("locale_pack_compatibility")
            .map(String::as_str),
        Some(LOCALE_PACK_COMPATIBILITY_REPORT_ID)
    );
    assert_eq!(
        packet
            .evidence_lane_refs
            .get("pseudolocalization")
            .map(String::as_str),
        Some(M5_DENSE_SURFACE_LAB_PACKET_ID)
    );
}

#[test]
fn downstream_consumers_are_bound_to_the_register() {
    let packet = seeded_localized_claim_status_packet();
    let kinds = packet
        .consumption_bindings
        .iter()
        .map(|b| b.consumer_kind)
        .collect::<BTreeSet<_>>();
    for required in [
        ConsumerKind::ReleaseCenter,
        ConsumerKind::HelpAbout,
        ConsumerKind::Diagnostics,
        ConsumerKind::ClaimNarrowing,
        ConsumerKind::SupportExport,
    ] {
        assert!(kinds.contains(&required), "missing consumer {required:?}");
    }
}
