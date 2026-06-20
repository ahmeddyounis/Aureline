//! Replay and coverage gate for the generated-artifact certification packet.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_generated::{
    seeded_m5_generated_certification_fixtures, seeded_m5_generated_certification_packet,
    validate_m5_generated_certification_fixture, validate_m5_generated_certification_packet,
    CertificationDomain, CertifiedProfile, M5GeneratedCertificationFixture,
    M5GeneratedCertificationPacket, PromotionDecision, PublicationChannel, RowVerdict,
    M5_GENERATED_CERTIFICATION_DOC_REF, M5_GENERATED_CERTIFICATION_FIXTURE_DIR,
    M5_GENERATED_CERTIFICATION_FIXTURE_MANIFEST_REF, M5_GENERATED_CERTIFICATION_PACKET_REF,
    M5_GENERATED_CERTIFICATION_REPORT_REF, M5_GENERATED_CERTIFICATION_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn load_packet() -> M5GeneratedCertificationPacket {
    let path = repo_root().join(M5_GENERATED_CERTIFICATION_PACKET_REF);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet {} must read: {err}", path.display()));
    serde_json::from_str(&raw)
        .unwrap_or_else(|err| panic!("packet {} must parse: {err}", path.display()))
}

fn load_fixtures() -> Vec<M5GeneratedCertificationFixture> {
    let dir = repo_root().join(M5_GENERATED_CERTIFICATION_FIXTURE_DIR);
    let mut out = Vec::new();
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: M5GeneratedCertificationFixture = serde_json::from_str(&raw)
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
    let seeded = seeded_m5_generated_certification_packet();
    assert_eq!(packet, seeded, "artifact packet drifted from seeded packet");
    validate_m5_generated_certification_packet(&packet)
        .expect("artifact packet must satisfy the frozen contract");
}

#[test]
fn fixture_corpus_matches_seeded_projection_and_validates() {
    let on_disk = load_fixtures();
    let mut seeded = seeded_m5_generated_certification_fixtures();
    seeded.sort_by(|a, b| a.fixture_id.cmp(&b.fixture_id));
    assert_eq!(
        on_disk, seeded,
        "fixture corpus drifted from seeded fixtures"
    );
    for fixture in &on_disk {
        validate_m5_generated_certification_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn files_exist_on_disk() {
    let root = repo_root();
    for rel in [
        M5_GENERATED_CERTIFICATION_SCHEMA_REF,
        M5_GENERATED_CERTIFICATION_DOC_REF,
        M5_GENERATED_CERTIFICATION_PACKET_REF,
        M5_GENERATED_CERTIFICATION_REPORT_REF,
        M5_GENERATED_CERTIFICATION_FIXTURE_MANIFEST_REF,
    ] {
        assert!(root.join(rel).exists(), "required file must exist: {rel}");
    }
    assert!(
        root.join(M5_GENERATED_CERTIFICATION_FIXTURE_DIR).is_dir(),
        "fixture directory must exist"
    );
}

#[test]
fn packet_certifies_every_claimed_profile() {
    let packet = load_packet();
    let profiles: BTreeSet<_> = packet.rows.iter().map(|row| row.profile).collect();
    for required in CertifiedProfile::ALL {
        assert!(
            profiles.contains(&required),
            "packet must certify profile {}",
            required.as_str()
        );
    }
}

#[test]
fn every_seeded_row_promotes_on_current_evidence() {
    let packet = load_packet();
    for row in &packet.rows {
        assert_eq!(
            row.verdict,
            RowVerdict::Certified,
            "row {} must certify on current evidence",
            row.row_id
        );
        assert_eq!(
            row.certified_maturity, row.published_claim_maturity,
            "row {} must not narrow on current evidence",
            row.row_id
        );
        assert_eq!(
            row.promotion_decision,
            PromotionDecision::Promote,
            "row {} must promote on current evidence",
            row.row_id
        );
    }
}

#[test]
fn evidence_and_publication_refs_point_at_real_artifacts() {
    let packet = load_packet();
    let root = repo_root();
    assert!(
        !packet.evidence_packet_refs.is_empty(),
        "packet must cite generated-artifact evidence"
    );
    for reference in &packet.evidence_packet_refs {
        assert!(
            root.join(reference).exists(),
            "evidence packet ref must exist on disk: {reference}"
        );
    }
    for row in &packet.rows {
        assert!(
            root.join(&row.claim_publication_ref).exists(),
            "row {} claim-publication object must exist on disk: {}",
            row.row_id,
            row.claim_publication_ref
        );
        assert!(
            root.join(&row.governance_evidence_ref).exists(),
            "row {} governance evidence ref must exist on disk: {}",
            row.row_id,
            row.governance_evidence_ref
        );
    }
}

#[test]
fn each_domain_evidence_packet_exists() {
    let root = repo_root();
    for domain in CertificationDomain::ALL {
        for reference in domain.evidence_packet_refs() {
            assert!(
                root.join(reference).exists(),
                "domain {} evidence packet must exist on disk: {reference}",
                domain.as_str()
            );
        }
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
fn drills_cover_narrowed_withheld_and_held_promotion() {
    let packet = load_packet();
    let mut verdicts = BTreeSet::new();
    let mut saw_hold = false;
    for drill in &packet.drills {
        verdicts.insert(drill.expected_degraded_verdict);
        if drill.expected_degraded_promotion_decision == PromotionDecision::Hold {
            saw_hold = true;
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
    assert!(saw_hold, "drills must exercise a held promotion");
}

#[test]
fn certification_does_not_outrun_governance() {
    let packet = load_packet();
    let governance =
        aureline_generated::m5_generated_governance::seeded_m5_generated_governance_packet();
    for row in &packet.rows {
        let governance_row = governance
            .rows
            .iter()
            .find(|gov| gov.artifact_class == row.backing_artifact_class)
            .unwrap_or_else(|| panic!("governance must certify the class backing {}", row.row_id));
        assert!(
            row.published_claim_maturity.severity() >= governance_row.claimed_maturity.severity(),
            "certification for {} must not outrun the governance claim",
            row.row_id
        );
    }
}
