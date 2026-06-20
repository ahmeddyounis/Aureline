//! Replay and coverage gate for the reactive-command-parity packet.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_reactive_state::{
    seeded_reactive_command_parity_fixtures, seeded_reactive_command_parity_packet,
    validate_reactive_command_parity_fixture, validate_reactive_command_parity_packet,
    ReactiveCommandParityDivergenceResolution, ReactiveCommandParityFixture,
    ReactiveCommandParityMutatingSurface, ReactiveCommandParityMutationKind,
    ReactiveCommandParityOptimisticPosture, ReactiveCommandParityPacket,
    REACTIVE_COMMAND_PARITY_DOC_REF, REACTIVE_COMMAND_PARITY_DRILLS_REF,
    REACTIVE_COMMAND_PARITY_FIXTURE_DIR, REACTIVE_COMMAND_PARITY_FIXTURE_MANIFEST_REF,
    REACTIVE_COMMAND_PARITY_PACKET_REF, REACTIVE_COMMAND_PARITY_REPORT_REF,
    REACTIVE_COMMAND_PARITY_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> ReactiveCommandParityPacket {
    let path = repo_root().join(REACTIVE_COMMAND_PARITY_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<ReactiveCommandParityFixture> {
    let dir = repo_root().join(REACTIVE_COMMAND_PARITY_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: ReactiveCommandParityFixture = serde_json::from_str(&raw)
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
    let seeded = seeded_reactive_command_parity_packet();
    assert_eq!(packet, seeded, "artifact packet drifted from seeded packet");
    validate_reactive_command_parity_packet(&packet)
        .expect("artifact packet must satisfy the frozen contract");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let packet = load_packet();
    let on_disk = load_fixtures();
    let mut seeded = seeded_reactive_command_parity_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_reactive_command_parity_fixture(&packet, fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        REACTIVE_COMMAND_PARITY_SCHEMA_REF,
        REACTIVE_COMMAND_PARITY_DOC_REF,
        REACTIVE_COMMAND_PARITY_PACKET_REF,
        REACTIVE_COMMAND_PARITY_REPORT_REF,
        REACTIVE_COMMAND_PARITY_DRILLS_REF,
        REACTIVE_COMMAND_PARITY_FIXTURE_MANIFEST_REF,
    ] {
        let path = root.join(rel);
        assert!(
            path.exists(),
            "required file must exist: {}",
            path.display()
        );
    }
    assert!(
        root.join(REACTIVE_COMMAND_PARITY_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn packet_covers_all_surfaces_kinds_postures_and_resolutions() {
    let packet = load_packet();
    let surfaces: BTreeSet<_> = packet
        .flows
        .iter()
        .map(|row| row.mutating_surface)
        .collect();
    for required in [
        ReactiveCommandParityMutatingSurface::AiApply,
        ReactiveCommandParityMutatingSurface::ReviewAction,
        ReactiveCommandParityMutatingSurface::ScaffoldUpdate,
        ReactiveCommandParityMutatingSurface::ProviderMutation,
        ReactiveCommandParityMutatingSurface::NotebookResultMutation,
        ReactiveCommandParityMutatingSurface::SupportRepair,
    ] {
        assert!(
            surfaces.contains(&required),
            "packet must cover mutating surface {}",
            required.as_str()
        );
    }

    let kinds: BTreeSet<_> = packet.flows.iter().map(|row| row.mutation_kind).collect();
    for required in [
        ReactiveCommandParityMutationKind::ApplyEdit,
        ReactiveCommandParityMutationKind::ApproveAction,
        ReactiveCommandParityMutationKind::ScaffoldArtifact,
        ReactiveCommandParityMutationKind::ProviderConfigChange,
        ReactiveCommandParityMutationKind::ExecuteCell,
        ReactiveCommandParityMutationKind::RepairState,
    ] {
        assert!(
            kinds.contains(&required),
            "packet must cover mutation kind {}",
            required.as_str()
        );
    }

    let postures: BTreeSet<_> = packet
        .flows
        .iter()
        .map(|row| row.optimistic_posture)
        .collect();
    for required in [
        ReactiveCommandParityOptimisticPosture::NeverOptimistic,
        ReactiveCommandParityOptimisticPosture::OptimisticRemoved,
        ReactiveCommandParityOptimisticPosture::OptimisticQuarantined,
    ] {
        assert!(
            postures.contains(&required),
            "packet must cover optimistic posture {}",
            required.as_str()
        );
    }

    let resolutions: BTreeSet<_> = packet
        .flows
        .iter()
        .map(|row| row.divergence_resolution)
        .collect();
    for required in [
        ReactiveCommandParityDivergenceResolution::DegradeSurface,
        ReactiveCommandParityDivergenceResolution::HoldAndWait,
        ReactiveCommandParityDivergenceResolution::RevertToCanonical,
    ] {
        assert!(
            resolutions.contains(&required),
            "packet must cover divergence resolution {}",
            required.as_str()
        );
    }
}

#[test]
fn no_flow_claims_success_before_publish() {
    let packet = load_packet();
    for row in &packet.flows {
        assert!(
            !row.claims_success_before_publish,
            "flow {} must not claim success before publication",
            row.flow_id
        );
        assert!(
            row.publishes_after_command_commit && row.publishes_after_journal_commit,
            "flow {} must publish only after the command and journal commit",
            row.flow_id
        );
        assert!(
            row.support_correlatable,
            "flow {} must keep its published state support-correlatable",
            row.flow_id
        );
    }
}

#[test]
fn drills_are_present_and_bound_to_flows() {
    let packet = load_packet();
    let flow_ids: BTreeSet<_> = packet
        .flows
        .iter()
        .map(|row| row.flow_id.as_str())
        .collect();
    let drilled: BTreeSet<_> = packet
        .drills
        .iter()
        .map(|drill| drill.mutating_surface)
        .collect();
    for required in [
        ReactiveCommandParityMutatingSurface::AiApply,
        ReactiveCommandParityMutatingSurface::ReviewAction,
        ReactiveCommandParityMutatingSurface::ScaffoldUpdate,
        ReactiveCommandParityMutatingSurface::ProviderMutation,
        ReactiveCommandParityMutatingSurface::NotebookResultMutation,
        ReactiveCommandParityMutatingSurface::SupportRepair,
    ] {
        assert!(
            drilled.contains(&required),
            "packet must drill mutating surface {}",
            required.as_str()
        );
    }
    for drill in &packet.drills {
        assert!(
            flow_ids.contains(drill.exercised_flow_id.as_str()),
            "drill {} must exercise a real flow",
            drill.drill_id
        );
        assert!(drill.asserts_no_optimistic_truth_before_publish);
        assert!(drill.asserts_lineage_correlatable);
    }
}
