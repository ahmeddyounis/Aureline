//! Protected tests for the shared service-health feed contract.

use std::path::{Path, PathBuf};

use aureline_service_health::{
    canonical_service_health_feed, ServiceHealthContractState, ServiceHealthFeed,
    ServiceHealthOutageScope, ServiceHealthSourceClass, ServiceHealthSurface,
    SERVICE_HEALTH_FEED_CANONICAL_FIXTURE_REF, SERVICE_HEALTH_FEED_RECORD_KIND,
    SERVICE_HEALTH_FEED_SCHEMA_REF, SERVICE_HEALTH_FEED_SCHEMA_VERSION,
};

const FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/help/m4/stabilize-service-health-feed-objects-outage-scope/canonical_feed.json"
));

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn canonical_feed_validates_cleanly() {
    let feed = canonical_service_health_feed();
    assert_eq!(feed.record_kind, SERVICE_HEALTH_FEED_RECORD_KIND);
    assert_eq!(feed.schema_version, SERVICE_HEALTH_FEED_SCHEMA_VERSION);
    let report = feed.validate();
    assert!(report.passed, "feed findings: {:#?}", report.findings);
}

#[test]
fn checked_in_fixture_matches_canonical_feed() {
    let fixture: ServiceHealthFeed = serde_json::from_str(FIXTURE).expect("fixture parses");
    assert_eq!(fixture, canonical_service_health_feed());
}

#[test]
fn canonical_feed_covers_all_contract_states_and_surfaces() {
    let report = canonical_service_health_feed().validate();
    for state in ServiceHealthContractState::ALL {
        assert!(
            report.coverage.contract_states.contains(&state),
            "missing contract state {}",
            state.as_str()
        );
    }
    for surface in ServiceHealthSurface::REQUIRED {
        assert!(
            report.coverage.surfaces.contains(&surface),
            "missing surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn cached_mirrored_and_offline_rows_never_claim_live_reachability() {
    for item in canonical_service_health_feed().items {
        if matches!(
            item.freshness.source_class,
            ServiceHealthSourceClass::CachedData
                | ServiceHealthSourceClass::MirroredNotice
                | ServiceHealthSourceClass::OfflineBundle
        ) {
            assert!(!item.freshness.live_reachability_claim_allowed);
        }
    }
}

#[test]
fn partial_outage_rows_preserve_explicit_unaffected_workflows() {
    let feed = canonical_service_health_feed();
    let partial = feed
        .items
        .iter()
        .filter(|item| item.outage_scope == ServiceHealthOutageScope::PartialService)
        .collect::<Vec<_>>();
    assert!(!partial.is_empty(), "expected at least one partial-outage row");
    assert!(partial.iter().all(|item| !item.unaffected_workflows.is_empty()));
}

#[test]
fn schema_artifact_exists_at_declared_path() {
    let path = repo_root().join(SERVICE_HEALTH_FEED_SCHEMA_REF);
    assert!(path.exists(), "{} must exist", path.display());
}

#[test]
fn canonical_fixture_ref_is_repo_relative() {
    let path = repo_root().join(SERVICE_HEALTH_FEED_CANONICAL_FIXTURE_REF);
    assert!(path.exists(), "{} must exist", path.display());
}
