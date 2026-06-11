//! Unit tests for the M5 first-useful-work entry-routes packet.

use super::*;

#[test]
fn seeded_packet_covers_every_required_lane() {
    let packet = seeded_m5_entry_routes_packet();
    assert!(packet.covers_every_lane());
    assert_eq!(packet.route_count(), M5DepthLane::required_lanes().len());
    for lane in M5DepthLane::required_lanes() {
        assert!(
            packet.routes.iter().any(|route| route.lane == lane),
            "missing lane {}",
            lane.as_str()
        );
    }
}

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_entry_routes_packet();
    assert_eq!(validate_m5_entry_routes_packet(&packet), Ok(()));
}

#[test]
fn every_route_exposes_setup_later_and_local_core_fallback() {
    let packet = seeded_m5_entry_routes_packet();
    for route in &packet.routes {
        assert!(
            route
                .setup_later_actions
                .contains(&ContinueWithoutClass::SetUpLater),
            "route {} is missing set_up_later",
            route.route_id
        );
        assert!(
            route.local_core_fallback,
            "route {} is missing a local-core fallback",
            route.route_id
        );
        assert!(
            route.no_hidden_prerequisite(),
            "route {} declares a hidden prerequisite",
            route.route_id
        );
    }
    assert!(packet.no_hidden_prerequisites);
}

#[test]
fn every_route_explains_what_is_not_yet_done() {
    let packet = seeded_m5_entry_routes_packet();
    for route in &packet.routes {
        assert!(
            !route.deferred_actions.is_empty(),
            "route {} has no deferred-action statements",
            route.route_id
        );
        for deferred in &route.deferred_actions {
            assert!(!deferred.statement.trim().is_empty());
        }
    }
}

#[test]
fn enrichments_are_never_mandatory() {
    let packet = seeded_m5_entry_routes_packet();
    for route in &packet.routes {
        for enrichment in &route.optional_enrichments {
            assert!(
                !enrichment.mandatory,
                "route {} marks {} mandatory",
                route.route_id,
                enrichment.enrichment_class.as_str()
            );
        }
    }
}

#[test]
fn first_useful_work_is_measured_before_optional_setup() {
    let packet = seeded_m5_entry_routes_packet();
    for route in &packet.routes {
        assert!(
            route.first_useful_work.reached_before_optional_setup,
            "route {} defers first useful work behind setup",
            route.route_id
        );
        assert!(route
            .first_useful_work
            .success_states_covered
            .contains(&TaskSuccessState::Completion));
        assert!(route.first_useful_work.no_raw_sensitive_user_content);
    }
}

#[test]
fn packet_round_trips_through_json() {
    let packet = seeded_m5_entry_routes_packet();
    let json = serde_json::to_string(&packet).expect("serialize");
    let decoded: M5EntryRoutesPacket = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(packet, decoded);
}

#[test]
fn support_export_collects_route_and_capture_ids() {
    let packet = seeded_m5_entry_routes_packet();
    let export = M5EntryRoutesSupportExport::from_packet(
        "support-export:m5-entry-routes:001",
        packet.clone(),
    );
    assert!(export.case_ids.contains(&packet.packet_id));
    for route in &packet.routes {
        assert!(export.case_ids.contains(&route.route_id));
        assert!(export
            .case_ids
            .contains(&route.first_useful_work.telemetry_capture_ref));
    }
}

#[test]
fn validation_flags_hidden_prerequisite() {
    let mut packet = seeded_m5_entry_routes_packet();
    packet.routes[0].requires_managed_sync = true;
    let errors = validate_m5_entry_routes_packet(&packet).expect_err("must fail");
    assert!(errors.iter().any(|err| matches!(
        err,
        M5EntryRoutesValidationError::HiddenPrerequisiteRequired { prerequisite, .. }
            if prerequisite == "managed_sync"
    )));
}

#[test]
fn validation_flags_mandatory_enrichment() {
    let mut packet = seeded_m5_entry_routes_packet();
    packet.routes[0].optional_enrichments[0].mandatory = true;
    let errors = validate_m5_entry_routes_packet(&packet).expect_err("must fail");
    assert!(errors.iter().any(|err| matches!(
        err,
        M5EntryRoutesValidationError::MandatoryEnrichment { .. }
    )));
}

#[test]
fn validation_flags_missing_setup_later() {
    let mut packet = seeded_m5_entry_routes_packet();
    packet.routes[0]
        .setup_later_actions
        .retain(|action| *action != ContinueWithoutClass::SetUpLater);
    let errors = validate_m5_entry_routes_packet(&packet).expect_err("must fail");
    assert!(errors.iter().any(|err| matches!(
        err,
        M5EntryRoutesValidationError::SetupLaterActionMissing { .. }
    )));
}

#[test]
fn validation_flags_stale_lane_coverage() {
    let mut packet = seeded_m5_entry_routes_packet();
    packet.routes.pop();
    let errors = validate_m5_entry_routes_packet(&packet).expect_err("must fail");
    assert!(errors
        .iter()
        .any(|err| matches!(err, M5EntryRoutesValidationError::LaneCoverageStale)));
    assert!(errors
        .iter()
        .any(|err| matches!(err, M5EntryRoutesValidationError::MissingLane { .. })));
}

#[test]
fn compact_lines_and_markdown_render() {
    let packet = seeded_m5_entry_routes_packet();
    let lines = packet.compact_lines();
    assert!(lines.iter().any(|line| line.contains("notebook")));
    let markdown = packet.render_markdown();
    assert!(markdown.contains("First-useful-work entry routes for M5 depth lanes"));
    assert!(markdown.contains("Offboarding"));
}
