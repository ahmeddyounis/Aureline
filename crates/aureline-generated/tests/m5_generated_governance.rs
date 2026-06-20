//! Replay and coverage gate for the generated-artifact-governance packet.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_generated::{
    seeded_m5_generated_governance_fixtures, seeded_m5_generated_governance_packet,
    validate_m5_generated_governance_fixture, validate_m5_generated_governance_packet,
    ArtifactClass, M5GeneratedGovernanceFixture, M5GeneratedGovernancePacket, PublicationChannel,
    RowVerdict, M5_GENERATED_GOVERNANCE_DOC_REF, M5_GENERATED_GOVERNANCE_FIXTURE_DIR,
    M5_GENERATED_GOVERNANCE_FIXTURE_MANIFEST_REF, M5_GENERATED_GOVERNANCE_PACKET_REF,
    M5_GENERATED_GOVERNANCE_REPORT_REF, M5_GENERATED_GOVERNANCE_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> M5GeneratedGovernancePacket {
    let path = repo_root().join(M5_GENERATED_GOVERNANCE_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<M5GeneratedGovernanceFixture> {
    let dir = repo_root().join(M5_GENERATED_GOVERNANCE_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: M5GeneratedGovernanceFixture = serde_json::from_str(&raw)
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
    let seeded = seeded_m5_generated_governance_packet();
    assert_eq!(packet, seeded, "artifact packet drifted from seeded packet");
    validate_m5_generated_governance_packet(&packet)
        .expect("artifact packet must satisfy the frozen contract");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let on_disk = load_fixtures();
    let mut seeded = seeded_m5_generated_governance_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_m5_generated_governance_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        M5_GENERATED_GOVERNANCE_SCHEMA_REF,
        M5_GENERATED_GOVERNANCE_DOC_REF,
        M5_GENERATED_GOVERNANCE_PACKET_REF,
        M5_GENERATED_GOVERNANCE_REPORT_REF,
        M5_GENERATED_GOVERNANCE_FIXTURE_MANIFEST_REF,
    ] {
        assert!(root.join(rel).exists(), "required file must exist: {rel}");
    }
    assert!(
        root.join(M5_GENERATED_GOVERNANCE_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn packet_certifies_every_claimed_class() {
    let packet = load_packet();
    let classes: BTreeSet<_> = packet.rows.iter().map(|row| row.artifact_class).collect();
    for required in ArtifactClass::ALL {
        assert!(
            classes.contains(&required),
            "packet must certify class {}",
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
            row.effective_edit_posture, row.claimed_edit_posture,
            "row {} must not downgrade the writable boundary on current evidence",
            row.row_id
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
fn packet_binds_every_publication_channel() {
    let packet = load_packet();
    let channels: BTreeSet<_> = packet
        .surface_bindings
        .iter()
        .map(|binding| binding.channel)
        .collect();
    for required in PublicationChannel::ALL {
        assert!(
            channels.contains(&required),
            "packet must bind channel {}",
            required.as_str()
        );
    }
}

#[test]
fn drills_cover_narrowed_withheld_and_edit_posture_downgrade() {
    let packet = load_packet();
    let mut verdicts = BTreeSet::new();
    let mut saw_edit_posture_downgrade = false;
    for drill in &packet.drills {
        verdicts.insert(drill.expected_degraded_verdict);
        if drill.expected_degraded_edit_posture != drill.claimed_edit_posture {
            saw_edit_posture_downgrade = true;
        }
        assert_eq!(
            drill.recovers_to_verdict,
            RowVerdict::Certified,
            "drill {} must recover to certified",
            drill.drill_id
        );
    }
    assert!(
        verdicts.contains(&RowVerdict::Narrowed),
        "drills must exercise a narrowed verdict"
    );
    assert!(
        verdicts.contains(&RowVerdict::Withheld),
        "drills must exercise a withheld verdict"
    );
    assert!(
        saw_edit_posture_downgrade,
        "drills must exercise an edit-posture downgrade"
    );
}
