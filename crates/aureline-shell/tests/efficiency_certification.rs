//! Fixture-driven coverage for the M5 efficiency certification proof packet.
//!
//! The checked-in proof packet at `artifacts/efficiency/m5-efficiency-proof-packet.json`
//! is the canonical truth source release, support, docs, and help consume for the
//! low-power claim. This test re-derives the packet from the seeded energy/thermal
//! and session-pressure evidence, proving the artifact never drifts from the code,
//! and asserts the promotion gating, automatic claim narrowing, and
//! no-protected-path-regression guarantees the certification lane depends on.

use std::path::Path;

use aureline_shell::efficiency::certification::{
    certify_m5_efficiency, seeded_certification_subjects, seeded_proof_packet, CertificationDrill,
    CertificationState, CertifiedSubjectKind, EfficiencyClaimLevel, EvidenceFreshness,
    M5EfficiencyProofPacket, REQUIRED_PUBLICATION_SURFACES,
};
use aureline_shell::efficiency::energy_lab::seeded_lab_cases;
use aureline_shell::efficiency::session_pressure::seeded_session_pressure_cases;

fn artifact_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/efficiency/m5-efficiency-proof-packet.json")
}

#[test]
fn checked_in_proof_packet_does_not_drift() {
    let raw = std::fs::read_to_string(artifact_path())
        .expect("proof packet artifact exists; run tools/regenerate_efficiency_certification.py");
    let stored: M5EfficiencyProofPacket =
        serde_json::from_str(&raw).expect("proof packet artifact parses");
    let rebuilt = seeded_proof_packet();
    assert_eq!(rebuilt, stored, "proof packet drifted from the seeded code");
}

#[test]
fn seeded_packet_certifies_both_axes_and_proceeds() {
    let packet = seeded_proof_packet();

    // Promotion proceeds and no claim outruns its evidence.
    assert!(packet.promotion_proceeds(), "{:?}", packet.promotion_gate);
    assert!(packet.no_claim_outruns_evidence());

    // Both axes the spec keeps separate are covered.
    let profiles: Vec<_> = packet
        .rows
        .iter()
        .filter(|r| r.subject_kind == CertifiedSubjectKind::LaptopOrDesktopProfile.as_str())
        .collect();
    let surfaces: Vec<_> = packet
        .rows
        .iter()
        .filter(|r| r.subject_kind == CertifiedSubjectKind::M5SurfaceFamily.as_str())
        .collect();
    assert!(
        profiles.len() >= 4,
        "every claimed laptop/desktop profile certified"
    );
    assert!(
        surfaces.len() >= 8,
        "every long-running surface family certified"
    );

    // Every certified claim-bearing row reaches release, support, docs, and help.
    for row in &packet.rows {
        let ceiling = EfficiencyClaimLevel::from_token(&row.published_claim_ceiling).unwrap();
        if row.is_certified() && ceiling.is_claim_bearing() {
            assert_eq!(
                row.publication_targets,
                REQUIRED_PUBLICATION_SURFACES
                    .iter()
                    .map(|s| (*s).to_owned())
                    .collect::<Vec<_>>()
            );
        }
    }
}

#[test]
fn every_certified_row_holds_its_protected_paths() {
    let packet = seeded_proof_packet();
    for row in &packet.rows {
        if !row.is_certified() {
            continue;
        }
        // A certified row required and passed protected-path preservation and
        // hidden-work suppression when it bound trace evidence.
        if row.required_drills.contains(
            &CertificationDrill::ProtectedPathPreservation
                .as_str()
                .to_owned(),
        ) {
            assert!(
                row.protected_paths_preserved,
                "certified row {} regressed a protected path",
                row.row_id
            );
        }
        if row.required_drills.contains(
            &CertificationDrill::HiddenWorkSuppression
                .as_str()
                .to_owned(),
        ) {
            assert!(
                row.hidden_work_suppressed,
                "row {} leaked hidden work",
                row.row_id
            );
        }
        for result in &row.drill_results {
            assert_eq!(result.freshness, EvidenceFreshness::Current.as_str());
            assert!(
                result.passed,
                "certified row {} has a failing drill",
                row.row_id
            );
            assert!(result.narrowing_reason.is_none());
        }
    }
}

#[test]
fn stale_evidence_fails_promotion() {
    let lab = seeded_lab_cases();
    let session = seeded_session_pressure_cases();
    let subjects = seeded_certification_subjects();
    let packet = certify_m5_efficiency(
        "test.stale",
        "2028-01-01",
        "2028-01-01T00:00:00Z",
        &subjects,
        &lab,
        &session,
    );

    // A claim cannot outrun its evidence: stale rows narrow and hold promotion.
    assert!(!packet.promotion_proceeds());
    assert!(packet.summary_counts.rows_blocking_promotion > 0);
    for row in &packet.rows {
        let ceiling = EfficiencyClaimLevel::from_token(&row.published_claim_ceiling).unwrap();
        if ceiling.is_claim_bearing() {
            assert_ne!(
                row.certification_state,
                CertificationState::Certified.as_str(),
                "stale claim-bearing row {} must not stay certified",
                row.row_id
            );
            assert!(row.blocks_promotion());
        }
    }
}

#[test]
fn summary_counts_are_consistent() {
    let packet = seeded_proof_packet();
    let c = &packet.summary_counts;
    assert_eq!(
        c.total_rows,
        c.rows_certified + c.rows_narrowed + c.rows_quarantined
    );
    assert_eq!(c.total_rows, c.profile_rows + c.surface_family_rows);
    assert_eq!(c.total_rows, packet.rows.len());
    assert_eq!(
        c.rows_blocking_promotion,
        packet.rows.iter().filter(|r| r.blocks_promotion()).count()
    );
}
