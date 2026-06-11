//! Fixture replay for the M5 first-useful-work entry-routes packet.
//!
//! The fixtures live under `fixtures/ux/m5/entry-and-resume/` and are generated
//! by the `aureline_shell_m5_entry_routes` headless inspector so the checked-in
//! JSON stays a literal projection of the seeded `M5EntryRoutesPacket`.

use aureline_shell::m5_entry_routes::{
    seeded_m5_entry_routes_packet, validate_m5_entry_routes_packet, LaneCoverageSummary,
    M5DepthLane, M5EntryRoute, M5EntryRoutesPacket, M5EntryRoutesSupportExport,
    M5_ENTRY_ROUTES_SCHEMA_VERSION, M5_ENTRY_ROUTES_SHARED_CONTRACT_REF,
};
use aureline_shell::onboarding_metrics::TaskSuccessState;
use aureline_workspace::ContinueWithoutClass;

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m5/entry-and-resume"
);

fn load<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let path = format!("{}/{}", FIXTURE_DIR, filename);
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn packet_fixture_matches_seeded_builder() {
    let from_file: M5EntryRoutesPacket = load("packet.json");
    let from_code = seeded_m5_entry_routes_packet();
    assert_eq!(
        from_file, from_code,
        "fixture must be a literal projection of the seeded packet; regenerate through the headless inspector"
    );
}

#[test]
fn packet_fixture_validates_and_covers_every_lane() {
    let packet: M5EntryRoutesPacket = load("packet.json");
    assert_eq!(
        packet.shared_contract_ref,
        M5_ENTRY_ROUTES_SHARED_CONTRACT_REF
    );
    assert_eq!(packet.schema_version, M5_ENTRY_ROUTES_SCHEMA_VERSION);
    assert!(packet.covers_every_lane());
    validate_m5_entry_routes_packet(&packet).expect("packet must validate");

    for lane in M5DepthLane::required_lanes() {
        assert!(
            packet.routes.iter().any(|route| route.lane == lane),
            "packet must include lane {}",
            lane.as_str()
        );
    }
}

#[test]
fn routes_keep_setup_later_fallback_and_no_hidden_prerequisite() {
    let routes: Vec<M5EntryRoute> = load("routes.json");
    assert_eq!(routes.len(), M5DepthLane::required_lanes().len());

    for route in &routes {
        assert_eq!(
            route.shared_contract_ref,
            M5_ENTRY_ROUTES_SHARED_CONTRACT_REF
        );
        assert!(route.local_first_claim);
        assert!(route.local_core_fallback);
        assert!(!route.local_core_summary.is_empty());
        assert!(route
            .setup_later_actions
            .contains(&ContinueWithoutClass::SetUpLater));
        assert!(
            route.no_hidden_prerequisite(),
            "{} must not require a hidden prerequisite",
            route.route_id
        );
        assert!(!route.requires_browser_auth);
        assert!(!route.requires_provider_attachment);
        assert!(!route.requires_kernel_execution);
        assert!(!route.requires_managed_sync);

        assert!(
            !route.deferred_actions.is_empty(),
            "{} must explain what is not yet done",
            route.route_id
        );

        for enrichment in &route.optional_enrichments {
            assert!(
                !enrichment.mandatory,
                "{} marks an enrichment mandatory",
                route.route_id
            );
        }
    }
}

#[test]
fn routes_measure_first_useful_work_before_optional_setup() {
    let routes: Vec<M5EntryRoute> = load("routes.json");
    for route in &routes {
        let measurement = &route.first_useful_work;
        assert!(
            measurement.reached_before_optional_setup,
            "{} defers first useful work behind setup",
            route.route_id
        );
        assert!(measurement.no_raw_sensitive_user_content);
        assert!(measurement
            .success_states_covered
            .contains(&TaskSuccessState::Completion));
        assert!(measurement
            .success_states_covered
            .contains(&TaskSuccessState::Fallback));
        assert!(!measurement.telemetry_capture_ref.is_empty());
        assert!(!measurement.measurement_surface_ref.is_empty());
    }
}

#[test]
fn coverage_fixture_matches_routes() {
    let coverage: LaneCoverageSummary = load("coverage.json");
    let routes: Vec<M5EntryRoute> = load("routes.json");
    assert_eq!(coverage.covered_lanes.len(), routes.len());
    assert!(coverage.covers_every_lane());
    assert_eq!(coverage.local_first_lanes, routes.len());
}

#[test]
fn support_export_collects_route_and_capture_ids() {
    let export: M5EntryRoutesSupportExport = load("support_export.json");
    let packet = seeded_m5_entry_routes_packet();
    assert_eq!(export.packet, packet);
    assert!(export.case_ids.contains(&packet.packet_id));
    for route in &packet.routes {
        assert!(export.case_ids.contains(&route.route_id));
        assert!(export
            .case_ids
            .contains(&route.first_useful_work.telemetry_capture_ref));
    }
}
