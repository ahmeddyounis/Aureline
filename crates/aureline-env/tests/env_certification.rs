//! Replay and coverage gate for the environment-certification packet.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_env::{
    seeded_env_certification_fixtures, seeded_env_certification_packet,
    validate_env_certification_fixture, validate_env_certification_packet, EnvCertificationFixture,
    EnvCertificationPacket, RowVerdict, TargetClass, ENV_CERTIFICATION_DOC_REF,
    ENV_CERTIFICATION_FIXTURE_DIR, ENV_CERTIFICATION_FIXTURE_MANIFEST_REF,
    ENV_CERTIFICATION_PACKET_REF, ENV_CERTIFICATION_REPORT_REF, ENV_CERTIFICATION_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> EnvCertificationPacket {
    let path = repo_root().join(ENV_CERTIFICATION_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<EnvCertificationFixture> {
    let dir = repo_root().join(ENV_CERTIFICATION_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: EnvCertificationFixture = serde_json::from_str(&raw)
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
    let seeded = seeded_env_certification_packet();
    assert_eq!(packet, seeded, "artifact packet drifted from seeded packet");
    validate_env_certification_packet(&packet)
        .expect("artifact packet must satisfy the frozen contract");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let on_disk = load_fixtures();
    let mut seeded = seeded_env_certification_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_env_certification_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        ENV_CERTIFICATION_SCHEMA_REF,
        ENV_CERTIFICATION_DOC_REF,
        ENV_CERTIFICATION_PACKET_REF,
        ENV_CERTIFICATION_REPORT_REF,
        ENV_CERTIFICATION_FIXTURE_MANIFEST_REF,
    ] {
        assert!(root.join(rel).exists(), "required file must exist: {rel}");
    }
    assert!(
        root.join(ENV_CERTIFICATION_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn packet_certifies_every_claimed_target_class() {
    let packet = load_packet();
    let target_classes: BTreeSet<_> = packet.rows.iter().map(|row| row.target_class).collect();
    for required in TargetClass::ALL {
        assert!(
            target_classes.contains(&required),
            "packet must certify target class {}",
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
            RowVerdict::Certified,
            "row {} must certify on current evidence",
            row.row_id
        );
        assert_eq!(
            row.effective_maturity, row.claimed_maturity,
            "row {} must not narrow on current evidence",
            row.row_id
        );
        assert_eq!(
            row.effective_warm_start_posture, row.claimed_warm_start_posture,
            "row {} must not downgrade warm start on current evidence",
            row.row_id
        );
    }
    assert!(
        !packet.promotion.promotion_blocked,
        "current evidence must not block promotion"
    );
}

#[test]
fn lane_evidence_refs_point_at_real_artifacts() {
    let packet = load_packet();
    assert!(
        !packet.lane_evidence_refs.is_empty(),
        "packet must cite upstream environment lane evidence"
    );
    let root = repo_root();
    for reference in &packet.lane_evidence_refs {
        assert!(
            root.join(reference).exists(),
            "lane evidence ref must exist on disk: {reference}"
        );
    }
}

#[test]
fn every_aspect_evidence_ref_exists_on_disk() {
    let packet = load_packet();
    let root = repo_root();
    for row in &packet.rows {
        for aspect in &row.aspects {
            for reference in &aspect.evidence_refs {
                assert!(
                    root.join(reference).exists(),
                    "aspect {} evidence ref must exist on disk: {reference}",
                    aspect.aspect.as_str()
                );
            }
        }
    }
}
