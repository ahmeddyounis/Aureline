//! Unit tests for the cross-surface subscription contract.

use super::*;
use crate::m5_reactive_governance::{
    BackpressureMode, Completeness, Freshness, TerminalReason, TruthClaim,
};

fn healthy_frame(scope_id: &str) -> PublishedFrame {
    PublishedFrame {
        scope_id: scope_id.to_owned(),
        frame_class: FrameClass::Snapshot,
        snapshot_epoch: 1,
        delta_seq: 0,
        freshness: Freshness::Authoritative,
        completeness: Completeness::Full,
        backpressure_mode: BackpressureMode::Realtime,
        terminal_reason: None,
        policy_limited: false,
        producer_id: "aureline.test".to_owned(),
        producer_instance: "synthetic-host/pid-1/boot-0".to_owned(),
        observed_at: "mono:1".to_owned(),
    }
}

#[test]
fn packet_validates_and_covers_required_vocabulary() {
    let packet = seeded_cross_surface_subscription_packet();
    validate_cross_surface_subscription_packet(&packet).expect("seeded packet validates");

    let surfaces: BTreeSet<_> = packet
        .bindings
        .iter()
        .flat_map(|b| b.consumer_surfaces.iter().copied())
        .collect();
    for required in ConsumerSurface::all() {
        assert!(surfaces.contains(&required), "missing surface {required:?}");
    }
}

#[test]
fn one_publish_fans_identical_stable_fields_to_every_surface() {
    let packet = seeded_cross_surface_subscription_packet();
    let mut bus = CrossSurfaceSubscriptionBus::from_packet(&packet);
    let outcome = bus
        .publish("binding:workspace_tree", &healthy_frame("ws-1"))
        .expect("publish");

    // All six surfaces received a view.
    assert_eq!(outcome.views.len(), 6);
    let surfaces: Vec<_> = outcome.views.iter().map(|v| v.consumer_surface).collect();
    assert_eq!(surfaces, ConsumerSurface::all().to_vec());

    // The stable fields are byte-for-byte identical across surfaces: no
    // surface forks the shared envelope.
    for view in &outcome.views {
        assert_eq!(view.subscription, outcome.stable);
    }
    assert_eq!(outcome.stable.authority_class, AuthorityClass::WorkspaceVfs);
    assert_eq!(outcome.stable.scope_id, "ws-1");
}

#[test]
fn ambient_unscoped_subscription_is_rejected() {
    let packet = seeded_cross_surface_subscription_packet();
    let mut bus = CrossSurfaceSubscriptionBus::from_packet(&packet);
    let err = bus
        .publish("binding:workspace_tree", &healthy_frame(""))
        .expect_err("ambient scope must be rejected");
    assert_eq!(
        err,
        SubscriptionError::AmbientScopeForbidden("binding:workspace_tree".to_owned())
    );
}

#[test]
fn unknown_binding_is_rejected() {
    let packet = seeded_cross_surface_subscription_packet();
    let mut bus = CrossSurfaceSubscriptionBus::from_packet(&packet);
    let err = bus
        .publish("binding:nope", &healthy_frame("ws-1"))
        .expect_err("unknown binding must be rejected");
    assert!(matches!(err, SubscriptionError::UnknownBinding(_)));
}

#[test]
fn the_shared_envelope_is_the_canonical_subscription_envelope() {
    let packet = seeded_cross_surface_subscription_packet();
    let mut bus = CrossSurfaceSubscriptionBus::from_packet(&packet);
    let outcome = bus
        .publish("binding:workspace_tree", &healthy_frame("ws-1"))
        .expect("publish");
    // The frame the surfaces consumed is the canonical envelope JSON.
    assert!(outcome
        .envelope_json
        .contains("\"subscription_schema_version\""));
    assert!(outcome
        .envelope_json
        .contains("\"query_family\": \"vfs.workspace_tree\""));
    assert!(outcome
        .envelope_json
        .contains("\"authority_class\": \"workspace_vfs\""));
    assert!(outcome.envelope_json.contains("\"scope_ref\""));
    assert!(outcome.envelope_json.contains("\"snapshot_epoch\""));
}

#[test]
fn degraded_frame_narrows_identically_for_all_subscribers() {
    let packet = seeded_cross_surface_subscription_packet();
    let mut bus = CrossSurfaceSubscriptionBus::from_packet(&packet);
    let mut frame = healthy_frame("ws-1");
    frame.freshness = Freshness::Stale;
    let outcome = bus
        .publish("binding:search_index", &frame)
        .expect("publish");
    assert_eq!(outcome.stable.truth_claim, TruthClaim::StaleSnapshot);
    for view in &outcome.views {
        assert_eq!(view.subscription.truth_claim, TruthClaim::StaleSnapshot);
    }
}

#[test]
fn unavailable_provider_narrows_to_provider_unavailable() {
    let packet = seeded_cross_surface_subscription_packet();
    let mut bus = CrossSurfaceSubscriptionBus::from_packet(&packet);
    let mut frame = healthy_frame("review-1");
    frame.frame_class = FrameClass::Terminal;
    frame.completeness = Completeness::Unavailable;
    frame.terminal_reason = Some(TerminalReason::Unavailable);
    let outcome = bus
        .publish("binding:review_overlay", &frame)
        .expect("publish");
    assert_eq!(outcome.stable.truth_claim, TruthClaim::ProviderUnavailable);
}

#[test]
fn subscription_id_is_stable_per_binding_and_scope() {
    let packet = seeded_cross_surface_subscription_packet();
    let mut bus = CrossSurfaceSubscriptionBus::from_packet(&packet);
    let first = bus
        .publish("binding:workspace_tree", &healthy_frame("ws-1"))
        .expect("publish");
    let mut second_frame = healthy_frame("ws-1");
    second_frame.delta_seq = 1;
    second_frame.frame_class = FrameClass::Delta;
    let second = bus
        .publish("binding:workspace_tree", &second_frame)
        .expect("publish");
    // Same (binding, scope) keeps its subscription id across frames.
    assert_eq!(first.stable.subscription_id, second.stable.subscription_id);
    // A different scope gets a fresh subscription id.
    let other = bus
        .publish("binding:workspace_tree", &healthy_frame("ws-2"))
        .expect("publish");
    assert_ne!(first.stable.subscription_id, other.stable.subscription_id);
}

#[test]
fn inspector_names_authority_scope_and_epoch() {
    let packet = seeded_cross_surface_subscription_packet();
    let mut bus = CrossSurfaceSubscriptionBus::from_packet(&packet);
    bus.publish("binding:workspace_tree", &healthy_frame("ws-1"))
        .expect("publish");
    let mut policy = healthy_frame("ws-1");
    policy.policy_limited = true;
    bus.publish("binding:policy_trust", &policy)
        .expect("publish");

    let report = bus.inspector_report();
    assert_eq!(report.rows.len(), 2);
    let policy_row = report
        .rows
        .iter()
        .find(|r| r.subscription.binding_id == "binding:policy_trust")
        .expect("policy row present");
    assert_eq!(
        policy_row.subscription.authority_class,
        AuthorityClass::PolicyEntitlement
    );
    assert_eq!(policy_row.subscription.scope_id, "ws-1");
    assert_eq!(
        policy_row.subscription.truth_claim,
        TruthClaim::PolicyLimitedProjection
    );
}

#[test]
fn consumer_view_and_inspector_round_trip_through_serde() {
    let packet = seeded_cross_surface_subscription_packet();
    let mut bus = CrossSurfaceSubscriptionBus::from_packet(&packet);
    bus.publish("binding:review_overlay", &healthy_frame("review-1"))
        .expect("publish");
    let report = bus.inspector_report();
    let json = serde_json::to_string(&report).expect("serialize");
    let decoded: SubscriptionInspectorReport = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(report, decoded);
}

#[test]
fn seeded_fixtures_validate_and_replay() {
    let packet = seeded_cross_surface_subscription_packet();
    let fixtures = seeded_cross_surface_subscription_fixtures();
    assert_eq!(fixtures.len(), 8);
    for fixture in &fixtures {
        validate_cross_surface_subscription_fixture(&packet, fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));

        // Replaying the fixture through the bus produces the expected claim
        // for every subscribed surface.
        let mut bus = CrossSurfaceSubscriptionBus::from_packet(&packet);
        let outcome = bus
            .publish(&fixture.binding_id, &fixture.frame)
            .unwrap_or_else(|err| panic!("fixture {} must publish: {err}", fixture.fixture_id));
        assert_eq!(outcome.stable.truth_claim, fixture.expected_truth_claim);
        let surfaces: Vec<_> = outcome.views.iter().map(|v| v.consumer_surface).collect();
        assert_eq!(surfaces, fixture.expected_consumer_surfaces);
    }
}

#[test]
fn a_packet_with_no_cross_surface_binding_fails_validation() {
    let mut packet = seeded_cross_surface_subscription_packet();
    // Drop the all-six binding's reach so no binding covers every surface.
    for binding in &mut packet.bindings {
        if binding.binding_id == "binding:workspace_tree" {
            binding.consumer_surfaces = vec![ConsumerSurface::Shell];
        }
    }
    let report = validate_cross_surface_subscription_packet(&packet)
        .expect_err("packet without a cross-surface binding must fail");
    assert!(report
        .violations
        .iter()
        .any(|v| v.check_id == "packet.cross_surface_binding"));
}
