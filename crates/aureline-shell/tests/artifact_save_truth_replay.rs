//! Replay gate for artifact save truth fixtures and packet.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_shell::artifact_save_truth::{
    seeded_artifact_save_truth_fixtures, seeded_artifact_save_truth_packet,
    validate_artifact_save_truth_fixture, validate_artifact_save_truth_packet,
    ArtifactSaveTruthFixture, ArtifactSaveTruthPacket, SaveTruthEvidenceClass,
    ARTIFACT_SAVE_TRUTH_DOC_REF, ARTIFACT_SAVE_TRUTH_FIXTURE_DIR,
    ARTIFACT_SAVE_TRUTH_FIXTURE_README_REF, ARTIFACT_SAVE_TRUTH_PACKET_REF,
    ARTIFACT_SAVE_TRUTH_REPORT_REF, ARTIFACT_SAVE_TRUTH_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> ArtifactSaveTruthPacket {
    let path = repo_root().join(ARTIFACT_SAVE_TRUTH_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<ArtifactSaveTruthFixture> {
    let dir = repo_root().join(ARTIFACT_SAVE_TRUTH_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: ArtifactSaveTruthFixture = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()));
        out.push(fixture);
    }
    out.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert!(!out.is_empty(), "expected at least one fixture");
    out
}

#[test]
fn packet_matches_seeded_projection_and_validates() {
    let on_disk = load_packet();
    let seeded = seeded_artifact_save_truth_packet();
    assert_eq!(on_disk, seeded, "artifact save truth packet drifted");
    validate_artifact_save_truth_packet(&on_disk)
        .expect("artifact save truth packet must validate");
}

#[test]
fn fixtures_match_seeded_projection_and_validate() {
    let packet = load_packet();
    let mut seeded = seeded_artifact_save_truth_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    let on_disk = load_fixtures();
    assert_eq!(on_disk, seeded, "artifact save truth fixtures drifted");
    for fixture in &on_disk {
        validate_artifact_save_truth_fixture(&packet, fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn packet_files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        ARTIFACT_SAVE_TRUTH_SCHEMA_REF,
        ARTIFACT_SAVE_TRUTH_DOC_REF,
        ARTIFACT_SAVE_TRUTH_PACKET_REF,
        ARTIFACT_SAVE_TRUTH_REPORT_REF,
        ARTIFACT_SAVE_TRUTH_FIXTURE_README_REF,
    ] {
        let path = root.join(rel);
        assert!(
            path.exists(),
            "required file must exist: {}",
            path.display()
        );
    }
    assert!(
        root.join(ARTIFACT_SAVE_TRUTH_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn fixture_corpus_covers_required_evidence_classes() {
    let fixtures = load_fixtures();
    let covered: BTreeSet<_> = fixtures
        .iter()
        .flat_map(|fixture| fixture.evidence_classes.iter().copied())
        .collect();
    for required in [
        SaveTruthEvidenceClass::LossyDecodeRisk,
        SaveTruthEvidenceClass::MetadataPreservationDisclosure,
        SaveTruthEvidenceClass::ExecuteBitRetention,
        SaveTruthEvidenceClass::ExportOrRegenerateNotExactSave,
        SaveTruthEvidenceClass::LogicalTargetAmbiguityDisclosure,
        SaveTruthEvidenceClass::MidFlightDriftRebaseRequired,
    ] {
        assert!(
            covered.contains(&required),
            "fixture corpus must cover evidence class {}",
            required.as_str()
        );
    }
}
