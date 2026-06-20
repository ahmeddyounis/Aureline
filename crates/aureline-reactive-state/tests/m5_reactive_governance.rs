//! Replay and coverage gate for the M5 reactive-governance matrix.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_reactive_state::{
    narrow_m5_reactive_truth_claim, seeded_m5_reactive_governance_fixtures,
    seeded_m5_reactive_governance_packet, validate_m5_reactive_governance_fixture,
    validate_m5_reactive_governance_packet, M5ReactiveAuthorityClass, M5ReactiveDerivationClass,
    M5ReactiveGovernanceFixture, M5ReactiveGovernancePacket, M5ReactiveObservedState,
    M5ReactiveTruthClaim, M5ReactiveViewClass, M5_REACTIVE_GOVERNANCE_DOC_REF,
    M5_REACTIVE_GOVERNANCE_FIXTURE_DIR, M5_REACTIVE_GOVERNANCE_FIXTURE_MANIFEST_REF,
    M5_REACTIVE_GOVERNANCE_PACKET_REF, M5_REACTIVE_GOVERNANCE_REPORT_REF,
    M5_REACTIVE_GOVERNANCE_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> M5ReactiveGovernancePacket {
    let path = repo_root().join(M5_REACTIVE_GOVERNANCE_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<M5ReactiveGovernanceFixture> {
    let dir = repo_root().join(M5_REACTIVE_GOVERNANCE_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: M5ReactiveGovernanceFixture = serde_json::from_str(&raw)
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
    let seeded = seeded_m5_reactive_governance_packet();
    assert_eq!(packet, seeded, "artifact packet drifted from seeded packet");
    validate_m5_reactive_governance_packet(&packet)
        .expect("artifact packet must satisfy the frozen contract");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let packet = load_packet();
    let on_disk = load_fixtures();
    let mut seeded = seeded_m5_reactive_governance_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_m5_reactive_governance_fixture(&packet, fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        M5_REACTIVE_GOVERNANCE_SCHEMA_REF,
        M5_REACTIVE_GOVERNANCE_DOC_REF,
        M5_REACTIVE_GOVERNANCE_PACKET_REF,
        M5_REACTIVE_GOVERNANCE_REPORT_REF,
        M5_REACTIVE_GOVERNANCE_FIXTURE_MANIFEST_REF,
    ] {
        let path = root.join(rel);
        assert!(
            path.exists(),
            "required file must exist: {}",
            path.display()
        );
    }
    assert!(
        root.join(M5_REACTIVE_GOVERNANCE_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn matrix_covers_required_authority_and_view_vocabularies() {
    let packet = load_packet();
    let authorities: BTreeSet<_> = packet.surfaces.iter().map(|r| r.authority_class).collect();
    for required in [
        M5ReactiveAuthorityClass::WorkspaceVfs,
        M5ReactiveAuthorityClass::BufferEditor,
        M5ReactiveAuthorityClass::DerivedKnowledge,
        M5ReactiveAuthorityClass::Execution,
        M5ReactiveAuthorityClass::PolicyEntitlement,
        M5ReactiveAuthorityClass::ProviderOverlay,
    ] {
        assert!(
            authorities.contains(&required),
            "matrix must cover authority {}",
            required.as_str()
        );
    }
    let views: BTreeSet<_> = packet.surfaces.iter().map(|r| r.view_class).collect();
    for required in [
        M5ReactiveViewClass::EphemeralProjection,
        M5ReactiveViewClass::DurableLocalMaterialization,
        M5ReactiveViewClass::ExportableSnapshot,
        M5ReactiveViewClass::ManagedReplicatedView,
    ] {
        assert!(
            views.contains(&required),
            "matrix must cover view class {}",
            required.as_str()
        );
    }
}

#[test]
fn no_surface_overclaims_exact_current_truth() {
    let packet = load_packet();
    for row in &packet.surfaces {
        assert_eq!(row.derivation_class, M5ReactiveDerivationClass::Derived);
        assert_ne!(
            row.healthy_claim,
            M5ReactiveTruthClaim::ExactCurrentTruth,
            "surface {} must not present exact current truth",
            row.surface_class.as_str()
        );
    }
}

#[test]
fn release_tooling_narrows_underqualified_rows() {
    // A release/support reader can ask the canonical engine to downgrade a
    // claim from any observed state without surface-specific prose.
    let stale = M5ReactiveObservedState {
        freshness: aureline_reactive_state::M5ReactiveFreshness::Stale,
        completeness: aureline_reactive_state::M5ReactiveCompleteness::Full,
        backpressure_mode: aureline_reactive_state::M5ReactiveBackpressureMode::Realtime,
        terminal_reason: None,
        policy_limited: false,
    };
    let narrowed = narrow_m5_reactive_truth_claim(M5ReactiveDerivationClass::Derived, &stale);
    assert_eq!(narrowed.claim, M5ReactiveTruthClaim::StaleSnapshot);
}
