//! Replay and coverage gate for the M5 mutation-lineage packet.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_reactive_state::{
    seeded_m5_mutation_lineage_fixtures, seeded_m5_mutation_lineage_packet,
    validate_m5_mutation_lineage_fixture, validate_m5_mutation_lineage_packet,
    M5MutationArtifactClass, M5MutationLineageFixture, M5MutationLineagePacket,
    M5MutationReversalClass, M5MutationSurfaceClass, M5_MUTATION_LINEAGE_DOC_REF,
    M5_MUTATION_LINEAGE_FIXTURE_DIR,
    M5_MUTATION_LINEAGE_FIXTURE_MANIFEST_REF, M5_MUTATION_LINEAGE_PACKET_REF,
    M5_MUTATION_LINEAGE_REPORT_REF, M5_MUTATION_LINEAGE_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> M5MutationLineagePacket {
    let path = repo_root().join(M5_MUTATION_LINEAGE_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<M5MutationLineageFixture> {
    let dir = repo_root().join(M5_MUTATION_LINEAGE_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: M5MutationLineageFixture = serde_json::from_str(&raw)
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
    let seeded = seeded_m5_mutation_lineage_packet();
    assert_eq!(packet, seeded, "artifact packet drifted from seeded packet");
    validate_m5_mutation_lineage_packet(&packet)
        .expect("artifact packet must satisfy the frozen contract");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let packet = load_packet();
    let on_disk = load_fixtures();
    let mut seeded = seeded_m5_mutation_lineage_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_m5_mutation_lineage_fixture(&packet, fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        M5_MUTATION_LINEAGE_SCHEMA_REF,
        M5_MUTATION_LINEAGE_DOC_REF,
        M5_MUTATION_LINEAGE_PACKET_REF,
        M5_MUTATION_LINEAGE_REPORT_REF,
        M5_MUTATION_LINEAGE_FIXTURE_MANIFEST_REF,
    ] {
        let path = root.join(rel);
        assert!(
            path.exists(),
            "required file must exist: {}",
            path.display()
        );
    }
    assert!(
        root.join(M5_MUTATION_LINEAGE_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn packet_covers_required_surface_and_reversal_vocabularies() {
    let packet = load_packet();
    let surfaces: BTreeSet<_> = packet.entries.iter().map(|entry| entry.surface_class).collect();
    for required in [
        M5MutationSurfaceClass::NotebookDocument,
        M5MutationSurfaceClass::NotebookOutput,
        M5MutationSurfaceClass::RequestWorkspace,
        M5MutationSurfaceClass::DataExportArtifact,
        M5MutationSurfaceClass::PreviewOutput,
        M5MutationSurfaceClass::SyncPacket,
        M5MutationSurfaceClass::RepairTransaction,
        M5MutationSurfaceClass::ProviderDraft,
        M5MutationSurfaceClass::WorkflowBundle,
        M5MutationSurfaceClass::ProfilerTrace,
        M5MutationSurfaceClass::AiEvidencePacket,
        M5MutationSurfaceClass::IncidentAction,
    ] {
        assert!(
            surfaces.contains(&required),
            "packet must cover surface {}",
            required.as_str()
        );
    }

    let reversals: BTreeSet<_> = packet
        .entries
        .iter()
        .map(|entry| entry.reversal_class)
        .chain(packet.groups.iter().map(|group| group.reversal_class))
        .collect();
    for required in [
        M5MutationReversalClass::Exact,
        M5MutationReversalClass::GroupedExact,
        M5MutationReversalClass::Compensate,
        M5MutationReversalClass::Regenerate,
        M5MutationReversalClass::Manual,
        M5MutationReversalClass::AuditOnly,
    ] {
        assert!(
            reversals.contains(&required),
            "packet must cover reversal {}",
            required.as_str()
        );
    }
}

#[test]
fn fixture_coverage_spans_required_artifact_classes() {
    let fixtures = load_fixtures();
    let artifact_classes: BTreeSet<_> = fixtures
        .iter()
        .flat_map(|fixture| fixture.artifact_classes.iter().copied())
        .collect();
    for required in [
        M5MutationArtifactClass::NotebookFile,
        M5MutationArtifactClass::NotebookOutputBundle,
        M5MutationArtifactClass::RequestDocument,
        M5MutationArtifactClass::QueryExport,
        M5MutationArtifactClass::PreviewSnapshot,
        M5MutationArtifactClass::SyncManifest,
        M5MutationArtifactClass::RepairReceipt,
        M5MutationArtifactClass::ProviderDraft,
        M5MutationArtifactClass::WorkflowBundle,
        M5MutationArtifactClass::TraceCapture,
        M5MutationArtifactClass::AiEvidencePacket,
        M5MutationArtifactClass::IncidentPacket,
    ] {
        assert!(
            artifact_classes.contains(&required),
            "fixtures must cover artifact class {}",
            required.as_str()
        );
    }
}
