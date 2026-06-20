//! Replay and coverage gate for the reactive-state certification packet.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_reactive_state::{
    seeded_m5_reactive_certification_fixtures, seeded_m5_reactive_certification_packet,
    validate_m5_reactive_certification_fixture, validate_m5_reactive_certification_packet,
    M5ReactiveCertificationFixture, M5ReactiveCertificationPacket,
    M5ReactiveCertificationPublicationChannel, M5ReactiveCertificationRowVerdict,
    M5ReactiveCertificationSurfaceProfile, M5_REACTIVE_CERTIFICATION_DOC_REF,
    M5_REACTIVE_CERTIFICATION_FIXTURE_DIR, M5_REACTIVE_CERTIFICATION_FIXTURE_MANIFEST_REF,
    M5_REACTIVE_CERTIFICATION_PACKET_REF, M5_REACTIVE_CERTIFICATION_REPORT_REF,
    M5_REACTIVE_CERTIFICATION_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> M5ReactiveCertificationPacket {
    let path = repo_root().join(M5_REACTIVE_CERTIFICATION_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<M5ReactiveCertificationFixture> {
    let dir = repo_root().join(M5_REACTIVE_CERTIFICATION_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: M5ReactiveCertificationFixture = serde_json::from_str(&raw)
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
    let seeded = seeded_m5_reactive_certification_packet();
    assert_eq!(packet, seeded, "artifact packet drifted from seeded packet");
    validate_m5_reactive_certification_packet(&packet)
        .expect("artifact packet must satisfy the frozen contract");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let on_disk = load_fixtures();
    let mut seeded = seeded_m5_reactive_certification_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_m5_reactive_certification_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        M5_REACTIVE_CERTIFICATION_SCHEMA_REF,
        M5_REACTIVE_CERTIFICATION_DOC_REF,
        M5_REACTIVE_CERTIFICATION_PACKET_REF,
        M5_REACTIVE_CERTIFICATION_REPORT_REF,
        M5_REACTIVE_CERTIFICATION_FIXTURE_MANIFEST_REF,
    ] {
        assert!(root.join(rel).exists(), "required file must exist: {rel}");
    }
    assert!(
        root.join(M5_REACTIVE_CERTIFICATION_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn packet_certifies_every_claimed_profile() {
    let packet = load_packet();
    let profiles: BTreeSet<_> = packet.rows.iter().map(|row| row.surface_profile).collect();
    for required in [
        M5ReactiveCertificationSurfaceProfile::Shell,
        M5ReactiveCertificationSurfaceProfile::Search,
        M5ReactiveCertificationSurfaceProfile::Graph,
        M5ReactiveCertificationSurfaceProfile::Ai,
        M5ReactiveCertificationSurfaceProfile::Review,
        M5ReactiveCertificationSurfaceProfile::Support,
    ] {
        assert!(
            profiles.contains(&required),
            "packet must certify profile {}",
            required.as_str()
        );
    }
}

#[test]
fn every_seeded_row_certifies_on_current_evidence() {
    let packet = load_packet();
    for row in &packet.rows {
        assert_eq!(
            row.verdict,
            M5ReactiveCertificationRowVerdict::Certified,
            "row {} must certify on current evidence",
            row.row_id
        );
        assert_eq!(
            row.effective_maturity, row.claimed_maturity,
            "row {} must not narrow on current evidence",
            row.row_id
        );
    }
}

#[test]
fn evidence_packet_refs_point_at_real_artifacts() {
    let packet = load_packet();
    assert!(
        !packet.evidence_packet_refs.is_empty(),
        "packet must cite upstream reactive-state evidence"
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
fn packet_binds_every_publication_channel() {
    let packet = load_packet();
    let channels: BTreeSet<_> = packet
        .surface_bindings
        .iter()
        .map(|binding| binding.channel)
        .collect();
    for required in [
        M5ReactiveCertificationPublicationChannel::ReleaseShiproom,
        M5ReactiveCertificationPublicationChannel::SupportExport,
        M5ReactiveCertificationPublicationChannel::Docs,
        M5ReactiveCertificationPublicationChannel::Help,
    ] {
        assert!(
            channels.contains(&required),
            "packet must bind channel {}",
            required.as_str()
        );
    }
}

#[test]
fn drills_cover_narrowed_and_withheld_verdicts() {
    let packet = load_packet();
    let mut verdicts = BTreeSet::new();
    for drill in &packet.drills {
        verdicts.insert(drill.expected_degraded_verdict);
        assert_eq!(
            drill.recovers_to_verdict,
            M5ReactiveCertificationRowVerdict::Certified,
            "drill {} must recover to certified",
            drill.drill_id
        );
    }
    assert!(
        verdicts.contains(&M5ReactiveCertificationRowVerdict::Narrowed),
        "drills must exercise a narrowed verdict"
    );
    assert!(
        verdicts.contains(&M5ReactiveCertificationRowVerdict::Withheld),
        "drills must exercise a withheld verdict"
    );
}
