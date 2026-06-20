//! Replay and coverage gate for the generated-artifact descriptor packet.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_generated::{
    seeded_generated_artifact_descriptor_fixtures, seeded_generated_artifact_descriptor_packet,
    validate_generated_artifact_descriptor_fixture, validate_generated_artifact_descriptor_packet,
    ArtifactClass, GeneratedArtifactDescriptorFixture, GeneratedArtifactDescriptorPacket,
    PresentedAuthority, SurfaceKind, GENERATED_ARTIFACT_DESCRIPTOR_DOC_REF,
    GENERATED_ARTIFACT_DESCRIPTOR_FIXTURE_DIR, GENERATED_ARTIFACT_DESCRIPTOR_FIXTURE_MANIFEST_REF,
    GENERATED_ARTIFACT_DESCRIPTOR_PACKET_REF, GENERATED_ARTIFACT_DESCRIPTOR_REPORT_REF,
    GENERATED_ARTIFACT_DESCRIPTOR_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> GeneratedArtifactDescriptorPacket {
    let path = repo_root().join(GENERATED_ARTIFACT_DESCRIPTOR_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<GeneratedArtifactDescriptorFixture> {
    let dir = repo_root().join(GENERATED_ARTIFACT_DESCRIPTOR_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: GeneratedArtifactDescriptorFixture = serde_json::from_str(&raw)
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
    let seeded = seeded_generated_artifact_descriptor_packet();
    assert_eq!(packet, seeded, "artifact packet drifted from seeded packet");
    validate_generated_artifact_descriptor_packet(&packet)
        .expect("artifact packet must satisfy the frozen contract");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let on_disk = load_fixtures();
    let mut seeded = seeded_generated_artifact_descriptor_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_generated_artifact_descriptor_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        GENERATED_ARTIFACT_DESCRIPTOR_SCHEMA_REF,
        GENERATED_ARTIFACT_DESCRIPTOR_DOC_REF,
        GENERATED_ARTIFACT_DESCRIPTOR_PACKET_REF,
        GENERATED_ARTIFACT_DESCRIPTOR_REPORT_REF,
        GENERATED_ARTIFACT_DESCRIPTOR_FIXTURE_MANIFEST_REF,
    ] {
        assert!(root.join(rel).exists(), "required file must exist: {rel}");
    }
    assert!(
        root.join(GENERATED_ARTIFACT_DESCRIPTOR_FIXTURE_DIR)
            .is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn packet_describes_every_class() {
    let packet = load_packet();
    let classes: BTreeSet<_> = packet
        .descriptors
        .iter()
        .map(|d| d.artifact_class)
        .collect();
    for required in ArtifactClass::ALL {
        assert!(
            classes.contains(&required),
            "packet must describe class {}",
            required.as_str()
        );
    }
}

#[test]
fn evidence_packet_refs_point_at_real_artifacts() {
    let packet = load_packet();
    assert!(
        !packet.evidence_packet_refs.is_empty(),
        "packet must cite upstream generated-artifact evidence"
    );
    let root = repo_root();
    for reference in &packet.evidence_packet_refs {
        assert!(
            root.join(reference).exists(),
            "evidence packet ref must exist on disk: {reference}"
        );
    }
}

#[test]
fn packet_binds_every_surface_with_real_consumers() {
    let packet = load_packet();
    let surfaces: BTreeSet<_> = packet
        .surface_bindings
        .iter()
        .map(|binding| binding.surface)
        .collect();
    for required in SurfaceKind::ALL {
        assert!(
            surfaces.contains(&required),
            "packet must bind surface {}",
            required.as_str()
        );
    }
    let root = repo_root();
    for binding in &packet.surface_bindings {
        assert!(
            root.join(&binding.consumer_ref).exists(),
            "surface consumer ref must exist on disk: {}",
            binding.consumer_ref
        );
    }
}

#[test]
fn every_surface_renders_identical_identity_fields() {
    let packet = load_packet();
    for descriptor in &packet.descriptors {
        let identity = descriptor.identity_fields();
        let projections = descriptor.project_all();
        assert_eq!(projections.len(), SurfaceKind::ALL.len());
        for projection in &projections {
            assert_eq!(
                projection.identity,
                identity,
                "surface {} drifted from the descriptor identity in {}",
                projection.surface.as_str(),
                descriptor.descriptor_id
            );
        }
    }
}

#[test]
fn hidden_or_missing_canonical_source_blocks_ordinary_source() {
    let fixtures = load_fixtures();
    let mut saw_block = false;
    for fixture in &fixtures {
        if fixture
            .descriptor
            .canonical_source
            .state
            .blocks_ordinary_source()
        {
            saw_block = true;
            assert!(
                !fixture
                    .descriptor
                    .presentation
                    .ordinary_source_claim_allowed,
                "fixture {} must block the ordinary-source claim",
                fixture.fixture_id
            );
            assert_eq!(
                fixture.descriptor.presentation.presented_authority,
                PresentedAuthority::ProvenanceWithheld,
                "fixture {} must withhold provenance",
                fixture.fixture_id
            );
        }
    }
    assert!(
        saw_block,
        "fixtures must cover a hidden/missing canonical-source block"
    );
}
