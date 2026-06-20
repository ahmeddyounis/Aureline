//! Replay and coverage gate for the reactive-recovery packet.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_reactive_state::{
    seeded_reactive_recovery_fixtures, seeded_reactive_recovery_packet,
    validate_reactive_recovery_fixture, validate_reactive_recovery_packet,
    ReactiveRecoveryConsumerSurface, ReactiveRecoveryFixture, ReactiveRecoveryLagCondition,
    ReactiveRecoveryPacket, ReactiveRecoveryStrategy, REACTIVE_RECOVERY_DOC_REF,
    REACTIVE_RECOVERY_DRILLS_REF, REACTIVE_RECOVERY_FIXTURE_DIR,
    REACTIVE_RECOVERY_FIXTURE_MANIFEST_REF, REACTIVE_RECOVERY_PACKET_REF,
    REACTIVE_RECOVERY_REPORT_REF, REACTIVE_RECOVERY_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> ReactiveRecoveryPacket {
    let path = repo_root().join(REACTIVE_RECOVERY_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<ReactiveRecoveryFixture> {
    let dir = repo_root().join(REACTIVE_RECOVERY_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: ReactiveRecoveryFixture = serde_json::from_str(&raw)
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
    let seeded = seeded_reactive_recovery_packet();
    assert_eq!(packet, seeded, "artifact packet drifted from seeded packet");
    validate_reactive_recovery_packet(&packet)
        .expect("artifact packet must satisfy the frozen contract");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let packet = load_packet();
    let on_disk = load_fixtures();
    let mut seeded = seeded_reactive_recovery_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_reactive_recovery_fixture(&packet, fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        REACTIVE_RECOVERY_SCHEMA_REF,
        REACTIVE_RECOVERY_DOC_REF,
        REACTIVE_RECOVERY_PACKET_REF,
        REACTIVE_RECOVERY_REPORT_REF,
        REACTIVE_RECOVERY_DRILLS_REF,
        REACTIVE_RECOVERY_FIXTURE_MANIFEST_REF,
    ] {
        let path = root.join(rel);
        assert!(
            path.exists(),
            "required file must exist: {}",
            path.display()
        );
    }
    assert!(
        root.join(REACTIVE_RECOVERY_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn packet_covers_all_surfaces_conditions_and_strategies() {
    let packet = load_packet();
    let surfaces: BTreeSet<_> = packet
        .flows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in [
        ReactiveRecoveryConsumerSurface::DesktopShell,
        ReactiveRecoveryConsumerSurface::CliHeadless,
        ReactiveRecoveryConsumerSurface::AiInspector,
        ReactiveRecoveryConsumerSurface::ReviewWorkspace,
        ReactiveRecoveryConsumerSurface::CompanionSnapshot,
    ] {
        assert!(
            surfaces.contains(&required),
            "packet must cover consumer surface {}",
            required.as_str()
        );
    }

    let conditions: BTreeSet<_> = packet.flows.iter().map(|row| row.lag_condition).collect();
    for required in [
        ReactiveRecoveryLagCondition::RapidInvalidationBurst,
        ReactiveRecoveryLagCondition::ConsumerLag,
        ReactiveRecoveryLagCondition::BackpressureOverflow,
        ReactiveRecoveryLagCondition::InvalidationGap,
        ReactiveRecoveryLagCondition::ReconnectAfterDrop,
        ReactiveRecoveryLagCondition::ProviderOverlayDisappeared,
    ] {
        assert!(
            conditions.contains(&required),
            "packet must cover lag condition {}",
            required.as_str()
        );
    }

    let strategies: BTreeSet<_> = packet
        .flows
        .iter()
        .map(|row| row.primary_strategy)
        .collect();
    for required in [
        ReactiveRecoveryStrategy::CoalesceDeltas,
        ReactiveRecoveryStrategy::RequestFreshSnapshot,
        ReactiveRecoveryStrategy::Resubscribe,
        ReactiveRecoveryStrategy::MarkStaleEpoch,
    ] {
        assert!(
            strategies.contains(&required),
            "packet must use recovery strategy {} as a primary",
            required.as_str()
        );
    }
}

#[test]
fn no_lagging_flow_offers_exact_truth_action() {
    let packet = load_packet();
    for row in &packet.flows {
        assert!(
            !row.offers_exact_truth_action,
            "lagging flow {} must not offer an exact-truth action",
            row.flow_id
        );
        assert!(
            !row.silent_retry_allowed,
            "lagging flow {} must not retry silently after a material posture change",
            row.flow_id
        );
    }
}

#[test]
fn named_drills_are_present_and_bound_to_flows() {
    let packet = load_packet();
    let flow_ids: BTreeSet<_> = packet
        .flows
        .iter()
        .map(|row| row.flow_id.as_str())
        .collect();
    let drilled: BTreeSet<_> = packet
        .drills
        .iter()
        .map(|drill| drill.lag_condition)
        .collect();
    for required in [
        ReactiveRecoveryLagCondition::RapidInvalidationBurst,
        ReactiveRecoveryLagCondition::ConsumerLag,
        ReactiveRecoveryLagCondition::ReconnectAfterDrop,
        ReactiveRecoveryLagCondition::ProviderOverlayDisappeared,
    ] {
        assert!(
            drilled.contains(&required),
            "packet must drill lag condition {}",
            required.as_str()
        );
    }
    for drill in &packet.drills {
        assert!(
            flow_ids.contains(drill.exercised_flow_id.as_str()),
            "drill {} must exercise a real flow",
            drill.drill_id
        );
        assert!(drill.asserts_no_stale_exact_action);
        assert!(drill.asserts_recovery_visible);
    }
}
