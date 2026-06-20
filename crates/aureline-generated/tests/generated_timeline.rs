//! Replay and coverage gate for the generated-artifact timeline packet.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_generated::{
    seeded_generated_timeline_fixtures, seeded_generated_timeline_packet,
    validate_generated_timeline_entry_fixture, validate_generated_timeline_packet, CaptureMode,
    GeneratedTimelineEntryFixture, GeneratedTimelinePacket, RestoreFidelity, TimelineSurface,
    GENERATED_TIMELINE_DOC_REF, GENERATED_TIMELINE_FIXTURE_DIR,
    GENERATED_TIMELINE_FIXTURE_MANIFEST_REF, GENERATED_TIMELINE_PACKET_REF,
    GENERATED_TIMELINE_REPORT_REF, GENERATED_TIMELINE_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> GeneratedTimelinePacket {
    let path = repo_root().join(GENERATED_TIMELINE_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<GeneratedTimelineEntryFixture> {
    let dir = repo_root().join(GENERATED_TIMELINE_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: GeneratedTimelineEntryFixture = serde_json::from_str(&raw)
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
    let seeded = seeded_generated_timeline_packet();
    assert_eq!(packet, seeded, "timeline packet drifted from seeded packet");
    validate_generated_timeline_packet(&packet)
        .expect("timeline packet must satisfy the frozen contract");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let on_disk = load_fixtures();
    let mut seeded = seeded_generated_timeline_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_generated_timeline_entry_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        GENERATED_TIMELINE_SCHEMA_REF,
        GENERATED_TIMELINE_DOC_REF,
        GENERATED_TIMELINE_PACKET_REF,
        GENERATED_TIMELINE_REPORT_REF,
        GENERATED_TIMELINE_FIXTURE_MANIFEST_REF,
    ] {
        assert!(root.join(rel).exists(), "required file must exist: {rel}");
    }
    assert!(
        root.join(GENERATED_TIMELINE_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn packet_covers_every_capture_mode_and_fidelity() {
    let packet = load_packet();
    let captures: BTreeSet<_> = packet.entries.iter().map(|e| e.capture_mode).collect();
    for required in CaptureMode::ALL {
        assert!(
            captures.contains(&required),
            "packet must cover capture mode {}",
            required.as_str()
        );
    }
    let fidelities: BTreeSet<_> = packet
        .entries
        .iter()
        .map(|e| e.outcome.restore_fidelity)
        .collect();
    for required in RestoreFidelity::ALL {
        assert!(
            fidelities.contains(&required),
            "packet must cover restore fidelity {}",
            required.as_str()
        );
    }
}

#[test]
fn evidence_packet_refs_point_at_real_artifacts() {
    let packet = load_packet();
    assert!(
        !packet.evidence_packet_refs.is_empty(),
        "packet must cite upstream evidence"
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
    for required in TimelineSurface::ALL {
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
fn only_full_unredacted_snapshots_claim_exact_continuity() {
    let packet = load_packet();
    let mut saw_exact = false;
    for entry in &packet.entries {
        if entry.outcome.exact_byte_continuity_claimed {
            saw_exact = true;
            assert_eq!(
                entry.capture_mode,
                CaptureMode::FullSnapshot,
                "entry {} claims exact continuity without a full snapshot",
                entry.entry_id
            );
            assert_eq!(
                entry.outcome.restore_fidelity,
                RestoreFidelity::ExactSnapshot,
                "entry {} must hold exact-snapshot fidelity",
                entry.entry_id
            );
        }
    }
    assert!(saw_exact, "packet must cover an exact-continuity entry");
}
