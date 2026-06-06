//! Fixture coverage for marketplace discovery ranking and anti-abuse truth.

use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize)]
struct Fixture {
    case_name: String,
    input: MarketplaceDiscoveryInput,
    expected: Expected,
}

#[derive(Debug, Deserialize)]
struct Expected {
    effective_tier: String,
    discovery_posture_class: String,
    support_claim_class: String,
    ranking_score: i16,
    missing_ranking_signal_count: usize,
    review_control_count: usize,
    blocking_control_count: usize,
    narrowed: bool,
    narrowing_reasons: Vec<String>,
    mirror_safe: bool,
    blocks_stable_discovery: bool,
}

fn all_fixtures() -> Vec<Fixture> {
    let raws = [
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-marketplace-discovery-ranking-and-anti-abuse/",
            "verified_publisher_prominent_stable.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-marketplace-discovery-ranking-and-anti-abuse/",
            "stale_compatibility_and_runtime_regression_narrows.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-marketplace-discovery-ranking-and-anti-abuse/",
            "typosquat_lookalike_quarantined_withdrawn.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-marketplace-discovery-ranking-and-anti-abuse/",
            "review_install_fraud_under_review_preview.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-marketplace-discovery-ranking-and-anti-abuse/",
            "enterprise_approved_mirror_stable.json"
        )),
        include_str!(concat!(
            "../../../../fixtures/extensions/m4/stabilize-marketplace-discovery-ranking-and-anti-abuse/",
            "rapid_revocation_removed_from_ranking.json"
        )),
    ];
    raws.iter()
        .map(|raw| serde_json::from_str(raw).expect("fixture must deserialize"))
        .collect()
}

#[test]
fn fixtures_build_validate_and_project_expected_truth() {
    for fixture in all_fixtures() {
        let packet = MarketplaceDiscoveryPacket::from_input(fixture.input)
            .unwrap_or_else(|err| panic!("{} must build: {err}", fixture.case_name));
        packet
            .validate()
            .unwrap_or_else(|errors| panic!("{} must validate: {errors:?}", fixture.case_name));

        let payload = serde_json::to_string(&packet).expect("serialize packet");
        let export = project_marketplace_discovery(&payload).expect("project support export");

        assert_eq!(
            packet.claim.effective_tier, fixture.expected.effective_tier,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.discovery_posture_class, fixture.expected.discovery_posture_class,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.claim.support_claim_class, fixture.expected.support_claim_class,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.ranking_score, fixture.expected.ranking_score,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.missing_ranking_signal_count,
            fixture.expected.missing_ranking_signal_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.review_control_count, fixture.expected.review_control_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.inspection.blocking_control_count, fixture.expected.blocking_control_count,
            "{}",
            fixture.case_name
        );
        assert_eq!(
            packet.claim.narrowed, fixture.expected.narrowed,
            "{}",
            fixture.case_name
        );

        let mut got = packet.claim.narrowing_reasons.clone();
        got.sort();
        let mut want = fixture.expected.narrowing_reasons;
        want.sort();
        assert_eq!(got, want, "{}", fixture.case_name);

        assert_eq!(export.effective_tier, packet.claim.effective_tier);
        assert_eq!(
            export.discovery_posture_class,
            packet.discovery_posture_class
        );
        assert_eq!(export.mirror_safe, fixture.expected.mirror_safe);
        assert_eq!(
            export.blocks_stable_discovery,
            fixture.expected.blocks_stable_discovery
        );
    }
}

#[test]
fn closed_vocabularies_hold_their_required_controls() {
    assert!(PUBLISHER_TIER_CLASSES.contains(&"verified_publisher"));
    assert!(PUBLISHER_TIER_CLASSES.contains(&"official_pack"));
    assert!(PUBLISHER_TIER_CLASSES.contains(&"enterprise_approved"));
    assert!(PUBLISHER_TIER_CLASSES.contains(&"under_review"));
    assert!(REQUIRED_RANKING_SIGNAL_CLASSES.contains(&"runtime_health"));
    assert!(REQUIRED_ABUSE_CONTROL_CLASSES.contains(&"typosquat_detection"));
    assert!(REQUIRED_ABUSE_CONTROL_CLASSES.contains(&"look_alike_detection"));
    assert!(REQUIRED_ABUSE_CONTROL_CLASSES.contains(&"review_install_fraud_detection"));
    assert!(REQUIRED_ABUSE_CONTROL_CLASSES.contains(&"rapid_revocation"));

    for reason in DISCOVERY_NARROWING_REASONS {
        let bucket_count = WITHDRAWN_REASONS.contains(reason) as u8
            + PREVIEW_REASONS.contains(reason) as u8
            + BETA_REASONS.contains(reason) as u8;
        assert_eq!(bucket_count, 1, "{reason} must be in exactly one bucket");
    }
}

#[test]
fn raw_install_count_cannot_drive_stable_prominence() {
    let fixture = all_fixtures()
        .into_iter()
        .find(|fixture| fixture.case_name == "verified_publisher_prominent_stable")
        .expect("stable fixture present");
    let mut input = fixture.input;
    input.ranking_signals[0].raw_install_count_primary = true;

    let packet = MarketplaceDiscoveryPacket::from_input(input).expect("packet builds");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .narrowing_reasons
        .contains(&"ranking_uses_vanity_metric".to_string()));
    assert_eq!(packet.discovery_posture_class, "under_review");
}

#[test]
fn missing_anti_abuse_control_narrows_to_preview() {
    let fixture = all_fixtures()
        .into_iter()
        .find(|fixture| fixture.case_name == "verified_publisher_prominent_stable")
        .expect("stable fixture present");
    let mut input = fixture.input;
    input
        .anti_abuse_controls
        .retain(|control| control.control_class != "rapid_revocation");

    let packet = MarketplaceDiscoveryPacket::from_input(input).expect("packet builds");
    assert_eq!(packet.claim.effective_tier, "preview");
    assert!(packet
        .claim
        .narrowing_reasons
        .contains(&"anti_abuse_control_missing".to_string()));
}

#[test]
fn invalid_vocabularies_are_rejected() {
    let fixture = all_fixtures()
        .into_iter()
        .find(|fixture| fixture.case_name == "verified_publisher_prominent_stable")
        .expect("stable fixture present");
    let mut input = fixture.input;
    input.publisher.tier_class = "popular_publisher".to_string();

    let err = MarketplaceDiscoveryPacket::from_input(input).expect_err("invalid tier rejected");
    assert!(err.to_string().contains("publisher.tier_class"));
}
