//! Replay and coverage gate for the state-class recovery packet.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_reactive_state::{
    seeded_state_class_recovery_fixtures, seeded_state_class_recovery_packet,
    validate_state_class_recovery_fixture, validate_state_class_recovery_packet,
    StateClassRecoveryFailureMode, StateClassRecoveryFixture, StateClassRecoveryPacket,
    StateClassRecoveryRoute, StateClassRecoveryStateClass, STATE_CLASS_RECOVERY_DOC_REF,
    STATE_CLASS_RECOVERY_FIXTURE_DIR, STATE_CLASS_RECOVERY_FIXTURE_MANIFEST_REF,
    STATE_CLASS_RECOVERY_PACKET_REF, STATE_CLASS_RECOVERY_REPORT_REF,
    STATE_CLASS_RECOVERY_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> StateClassRecoveryPacket {
    let path = repo_root().join(STATE_CLASS_RECOVERY_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<StateClassRecoveryFixture> {
    let dir = repo_root().join(STATE_CLASS_RECOVERY_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: StateClassRecoveryFixture = serde_json::from_str(&raw)
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
    let seeded = seeded_state_class_recovery_packet();
    assert_eq!(packet, seeded, "artifact packet drifted from seeded packet");
    validate_state_class_recovery_packet(&packet)
        .expect("artifact packet must satisfy the frozen contract");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let packet = load_packet();
    let on_disk = load_fixtures();
    let mut seeded = seeded_state_class_recovery_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_state_class_recovery_fixture(&packet, fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        STATE_CLASS_RECOVERY_SCHEMA_REF,
        STATE_CLASS_RECOVERY_DOC_REF,
        STATE_CLASS_RECOVERY_PACKET_REF,
        STATE_CLASS_RECOVERY_REPORT_REF,
        STATE_CLASS_RECOVERY_FIXTURE_MANIFEST_REF,
    ] {
        let path = root.join(rel);
        assert!(
            path.exists(),
            "required file must exist: {}",
            path.display()
        );
    }
    assert!(
        root.join(STATE_CLASS_RECOVERY_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn packet_covers_all_state_classes_routes_and_failure_modes() {
    let packet = load_packet();
    let state_classes: BTreeSet<_> = packet.families.iter().map(|row| row.state_class).collect();
    for required in [
        StateClassRecoveryStateClass::DurableUserState,
        StateClassRecoveryStateClass::WorkspaceState,
        StateClassRecoveryStateClass::DerivedCacheState,
        StateClassRecoveryStateClass::GeneratedArtifactState,
        StateClassRecoveryStateClass::RecoveryJournalState,
        StateClassRecoveryStateClass::SecurityTrustState,
    ] {
        assert!(
            state_classes.contains(&required),
            "packet must cover state class {}",
            required.as_str()
        );
    }

    let routes: BTreeSet<_> = packet
        .families
        .iter()
        .map(|row| row.primary_recovery_route)
        .collect();
    for required in [
        StateClassRecoveryRoute::RebuildAutomatically,
        StateClassRecoveryRoute::GuidedRepair,
        StateClassRecoveryRoute::RollbackToPreservedArtifact,
        StateClassRecoveryRoute::FailClosedPrivilegedOperations,
    ] {
        assert!(
            routes.contains(&required),
            "packet must cover recovery route {}",
            required.as_str()
        );
    }

    let failure_modes: BTreeSet<_> = packet
        .families
        .iter()
        .flat_map(|row| row.supported_failure_modes.iter().copied())
        .collect();
    for required in [
        StateClassRecoveryFailureMode::PartialCorruption,
        StateClassRecoveryFailureMode::MissingDependency,
        StateClassRecoveryFailureMode::StaleDerivedOverlay,
        StateClassRecoveryFailureMode::BrokenCacheShard,
        StateClassRecoveryFailureMode::JournalPreservedUnsavedWork,
        StateClassRecoveryFailureMode::QuarantinedTrustState,
    ] {
        assert!(
            failure_modes.contains(&required),
            "packet must cover failure mode {}",
            required.as_str()
        );
    }
}

#[test]
fn every_family_has_one_fixture() {
    let packet = load_packet();
    let fixtures = load_fixtures();
    let family_ids: BTreeSet<_> = packet
        .families
        .iter()
        .map(|row| row.family_id.as_str())
        .collect();
    let fixture_families: BTreeSet<_> = fixtures
        .iter()
        .map(|fixture| fixture.expected_family_id.as_str())
        .collect();
    assert_eq!(
        family_ids, fixture_families,
        "fixture corpus must bind exactly one scenario to every family"
    );
}
