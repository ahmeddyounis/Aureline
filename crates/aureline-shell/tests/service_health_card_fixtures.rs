//! Fixture replay for the service-health aggregator and contract-state
//! cards. The fixtures live under `fixtures/ops/m3/service_health_cards/`
//! and are minted by the `aureline_shell_service_health_inspect`
//! headless inspector, so the checked-in JSON stays a literal projection
//! of the seeded aggregator.

use aureline_shell::service_health::aggregator::{
    LocalContinuityClass, ServiceContractStateClass, ServiceHealthAggregator,
};
use aureline_shell::service_health::seed::seeded_aggregator;

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ops/m3/service_health_cards"
);

fn load<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let path = format!("{}/{}", FIXTURE_DIR, filename);
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn service_health_aggregator_seeded_fixture_matches_code() {
    let from_file: ServiceHealthAggregator = load("seeded_aggregator.json");
    let from_code = seeded_aggregator();
    assert_eq!(
        from_file, from_code,
        "fixture must be a literal projection of the seeded aggregator; regenerate via \
         aureline_shell_service_health_inspect aggregator",
    );
}

#[test]
fn all_ready_fixture_does_not_light_honesty_marker() {
    let agg: ServiceHealthAggregator = load("all_ready_aggregator.json");
    assert_eq!(agg.overall_contract_state, ServiceContractStateClass::Ready);
    assert_eq!(
        agg.overall_local_continuity,
        LocalContinuityClass::LocalSafe
    );
    assert!(!agg.honesty_marker_present);
    for card in &agg.cards {
        assert_eq!(card.contract_state, ServiceContractStateClass::Ready);
        assert!(!card.honesty_marker_present);
    }
}

#[test]
fn hosted_outage_keeps_overall_local_continuity_safe() {
    let agg: ServiceHealthAggregator = load("hosted_outage_keeps_local_safe.json");
    // A hosted outage MUST be honest about the family (unavailable) but
    // MUST NOT downgrade overall local-continuity.
    assert_eq!(
        agg.overall_contract_state,
        ServiceContractStateClass::Unavailable
    );
    assert_eq!(
        agg.overall_local_continuity,
        LocalContinuityClass::LocalSafe
    );
    assert!(agg.honesty_marker_present);
}

#[test]
fn sync_outage_downgrades_overall_continuity_to_read_only() {
    let agg: ServiceHealthAggregator = load("sync_outage_downgrades_to_read_only.json");
    assert_eq!(
        agg.overall_contract_state,
        ServiceContractStateClass::LocalOnly
    );
    assert_eq!(
        agg.overall_local_continuity,
        LocalContinuityClass::LocalSafeReadOnly
    );
    assert!(agg.honesty_marker_present);
}
