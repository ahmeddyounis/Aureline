//! Replay and coverage gate for the cross-surface subscription contract.
//!
//! Asserts the on-disk artifact and fixtures match the seeded projection,
//! that the runtime bus fans one published frame out to every subscribed
//! consumer surface with identical stable fields, that every subscription
//! is scoped, and that a degraded frame narrows identically on every
//! surface.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_reactive_state::{
    seeded_cross_surface_subscription_fixtures, seeded_cross_surface_subscription_packet,
    validate_cross_surface_subscription_fixture, validate_cross_surface_subscription_packet,
    ConsumerSurface, CrossSurfaceSubscriptionBus, CrossSurfaceSubscriptionFixture,
    CrossSurfaceSubscriptionPacket, SubscriptionError, CROSS_SURFACE_SUBSCRIPTION_DOC_REF,
    CROSS_SURFACE_SUBSCRIPTION_FIXTURE_DIR, CROSS_SURFACE_SUBSCRIPTION_FIXTURE_MANIFEST_REF,
    CROSS_SURFACE_SUBSCRIPTION_PACKET_REF, CROSS_SURFACE_SUBSCRIPTION_PROOF_REF,
    CROSS_SURFACE_SUBSCRIPTION_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> CrossSurfaceSubscriptionPacket {
    let path = repo_root().join(CROSS_SURFACE_SUBSCRIPTION_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<CrossSurfaceSubscriptionFixture> {
    let dir = repo_root().join(CROSS_SURFACE_SUBSCRIPTION_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: CrossSurfaceSubscriptionFixture = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()));
        out.push(fixture);
    }
    out.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert!(!out.is_empty(), "expected at least one fixture");
    out
}

#[test]
fn packet_matches_seeded_projection_and_validates() {
    let packet = load_packet();
    let seeded = seeded_cross_surface_subscription_packet();
    assert_eq!(packet, seeded, "artifact packet drifted from seeded packet");
    validate_cross_surface_subscription_packet(&packet)
        .expect("artifact packet must satisfy the frozen contract");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let packet = load_packet();
    let on_disk = load_fixtures();
    let mut seeded = seeded_cross_surface_subscription_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_cross_surface_subscription_fixture(&packet, fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        CROSS_SURFACE_SUBSCRIPTION_SCHEMA_REF,
        CROSS_SURFACE_SUBSCRIPTION_DOC_REF,
        CROSS_SURFACE_SUBSCRIPTION_PACKET_REF,
        CROSS_SURFACE_SUBSCRIPTION_PROOF_REF,
        CROSS_SURFACE_SUBSCRIPTION_FIXTURE_MANIFEST_REF,
    ] {
        let path = root.join(rel);
        assert!(
            path.exists(),
            "required file must exist: {}",
            path.display()
        );
    }
    assert!(
        root.join(CROSS_SURFACE_SUBSCRIPTION_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn every_fixture_fans_out_with_identical_stable_fields() {
    let packet = load_packet();
    for fixture in load_fixtures() {
        let mut bus = CrossSurfaceSubscriptionBus::from_packet(&packet);
        let outcome = bus
            .publish(&fixture.binding_id, &fixture.frame)
            .unwrap_or_else(|err| panic!("fixture {} must publish: {err}", fixture.fixture_id));

        // Each subscribed surface observed an identical projection of one
        // shared envelope — the core anti-drift guarantee.
        for view in &outcome.views {
            assert_eq!(
                view.subscription, outcome.stable,
                "fixture {} surface {:?} forked the shared envelope",
                fixture.fixture_id, view.consumer_surface
            );
        }

        let surfaces: Vec<ConsumerSurface> =
            outcome.views.iter().map(|v| v.consumer_surface).collect();
        assert_eq!(
            surfaces, fixture.expected_consumer_surfaces,
            "fixture {} fanned out to the wrong surfaces",
            fixture.fixture_id
        );
        assert_eq!(
            outcome.stable.truth_claim, fixture.expected_truth_claim,
            "fixture {} narrowed to the wrong claim",
            fixture.fixture_id
        );
        // The shared frame is the canonical subscription envelope.
        assert!(
            outcome
                .envelope_json
                .contains("\"subscription_schema_version\""),
            "fixture {} did not consume a canonical envelope",
            fixture.fixture_id
        );
    }
}

#[test]
fn contract_covers_all_six_consumer_surfaces_and_authorities() {
    let packet = load_packet();
    let surfaces: BTreeSet<_> = packet
        .bindings
        .iter()
        .flat_map(|b| b.consumer_surfaces.iter().copied())
        .collect();
    for required in ConsumerSurface::all() {
        assert!(
            surfaces.contains(&required),
            "contract must wire consumer surface {}",
            required.as_str()
        );
    }
    // At least one binding is subscribed by all six surfaces.
    assert!(
        packet
            .bindings
            .iter()
            .any(|b| b.consumer_surfaces.len() == ConsumerSurface::all().len()),
        "contract must declare a binding subscribed by all six surfaces"
    );
}

#[test]
fn ambient_unscoped_publish_fails_review() {
    let packet = load_packet();
    let mut bus = CrossSurfaceSubscriptionBus::from_packet(&packet);
    let mut frame = seeded_cross_surface_subscription_fixtures()[0]
        .frame
        .clone();
    frame.scope_id = String::new();
    let err = bus
        .publish("binding:workspace_tree", &frame)
        .expect_err("ambient subscription must fail review");
    assert!(matches!(err, SubscriptionError::AmbientScopeForbidden(_)));
}
