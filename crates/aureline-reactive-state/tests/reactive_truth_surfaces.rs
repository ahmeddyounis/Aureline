//! Replay and coverage gate for the reactive-truth-surfaces cue layer.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_reactive_state::reactive_truth_surfaces::{
    seeded_reactive_truth_surfaces_fixtures, seeded_reactive_truth_surfaces_packet,
    validate_reactive_truth_surfaces_fixture, validate_reactive_truth_surfaces_packet, ActionGate,
    ReactiveTruthCueFixture, ReactiveTruthSurfacesPacket, REACTIVE_TRUTH_SURFACES_DOC_REF,
    REACTIVE_TRUTH_SURFACES_FIXTURE_DIR, REACTIVE_TRUTH_SURFACES_FIXTURE_MANIFEST_REF,
    REACTIVE_TRUTH_SURFACES_PACKET_REF, REACTIVE_TRUTH_SURFACES_REPORT_REF,
    REACTIVE_TRUTH_SURFACES_SCHEMA_REF,
};
use aureline_reactive_state::{M5ReactiveDerivationClass, M5ReactiveTruthClaim};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> ReactiveTruthSurfacesPacket {
    let path = repo_root().join(REACTIVE_TRUTH_SURFACES_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<ReactiveTruthCueFixture> {
    let dir = repo_root().join(REACTIVE_TRUTH_SURFACES_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: ReactiveTruthCueFixture = serde_json::from_str(&raw)
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
    let seeded = seeded_reactive_truth_surfaces_packet();
    assert_eq!(packet, seeded, "artifact packet drifted from seeded packet");
    validate_reactive_truth_surfaces_packet(&packet)
        .expect("artifact packet must satisfy the frozen contract");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let on_disk = load_fixtures();
    let mut seeded = seeded_reactive_truth_surfaces_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(on_disk, seeded, "fixture corpus drifted from seeded fixtures");
    for fixture in &on_disk {
        validate_reactive_truth_surfaces_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        REACTIVE_TRUTH_SURFACES_SCHEMA_REF,
        REACTIVE_TRUTH_SURFACES_DOC_REF,
        REACTIVE_TRUTH_SURFACES_PACKET_REF,
        REACTIVE_TRUTH_SURFACES_REPORT_REF,
        REACTIVE_TRUTH_SURFACES_FIXTURE_MANIFEST_REF,
    ] {
        let path = root.join(rel);
        assert!(path.exists(), "required file must exist: {}", path.display());
    }
    assert!(
        root.join(REACTIVE_TRUTH_SURFACES_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn no_surface_overclaims_and_only_consistent_snapshot_keeps_actions_live() {
    let packet = load_packet();
    for audit in &packet.surfaces {
        assert_eq!(audit.derivation_class, M5ReactiveDerivationClass::Derived);
        assert_ne!(
            audit.healthy_claim,
            M5ReactiveTruthClaim::ExactCurrentTruth,
            "surface {} must not present exact current truth",
            audit.surface_class.as_str()
        );
        assert_eq!(audit.healthy_action_gate, ActionGate::Enabled);
        // Every degradation narrows dangerous derived actions away from `enabled`.
        for rule in &audit.gated_narrowing_rules {
            assert_ne!(
                rule.action_gate,
                ActionGate::Enabled,
                "surface {} trigger {} must narrow dangerous actions",
                audit.surface_class.as_str(),
                rule.trigger.as_str()
            );
        }
    }
}
