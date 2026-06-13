use super::*;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_records_policy_certification_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn seeded_packet_covers_every_family_once() {
    let packet = seeded_m5_records_policy_certification_packet();
    for family in GovernedArtifactFamily::ALL {
        let count = packet
            .rows
            .iter()
            .filter(|row| row.artifact_family == family)
            .count();
        assert_eq!(count, 1, "family {family:?} must appear exactly once");
    }
}

#[test]
fn every_row_certifies_with_all_dimensions_current() {
    let packet = seeded_m5_records_policy_certification_packet();
    for row in &packet.rows {
        assert_eq!(row.verdict, CertificationVerdict::Certified, "{row:?}");
        assert!(row.narrow_reasons.is_empty(), "{row:?}");
        for dimension in CertificationProofDimension::ALL {
            let cell = row.cell(dimension).expect("dimension present");
            assert!(cell.is_current(), "{dimension:?} not current: {cell:?}");
        }
    }
}

#[test]
fn seeded_record_classes_match_canonical_family_mapping() {
    let packet = seeded_m5_records_policy_certification_packet();
    for row in &packet.rows {
        assert_eq!(
            row.record_class_id,
            certified_record_class_for_family(row.artifact_family),
            "{row:?}"
        );
    }
}

#[test]
fn seeded_packet_promotes() {
    let packet = seeded_m5_records_policy_certification_packet();
    assert_eq!(
        packet.promotion.decision,
        CertificationPromotionDecision::Promote
    );
    assert!(packet.promotion.blocking_entry_ids.is_empty());
}

#[test]
fn seeded_packet_is_consistent_with_live_truth() {
    let packet = seeded_m5_records_policy_certification_packet();
    assert!(
        packet.verify_against_live_packets().is_empty(),
        "{:?}",
        packet.verify_against_live_packets()
    );
}

#[test]
fn projections_cover_every_row() {
    let packet = seeded_m5_records_policy_certification_packet();
    assert_eq!(packet.shiproom_projection().len(), packet.rows.len());
    assert_eq!(packet.public_claim_projection().len(), packet.rows.len());
    assert_eq!(packet.cli_headless_projection().len(), packet.rows.len());
    assert_eq!(packet.support_export_projection().len(), packet.rows.len());
}

#[test]
fn shiproom_and_public_projections_share_the_verdict() {
    let packet = seeded_m5_records_policy_certification_packet();
    let shiproom = packet.shiproom_projection();
    let public = packet.public_claim_projection();
    for (s, p) in shiproom.iter().zip(public.iter()) {
        assert_eq!(s.entry_id, p.entry_id);
        assert_eq!(s.verdict, p.verdict);
        assert!(!s.label.trim().is_empty());
        assert!(!p.label.trim().is_empty());
    }
}

#[test]
fn stale_proof_narrows_row_and_holds_promotion() {
    let mut packet = seeded_m5_records_policy_certification_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.release_blocking)
        .expect("a release-blocking row exists");
    let cell = row
        .proof_cells
        .iter_mut()
        .find(|cell| cell.dimension == CertificationProofDimension::Chronology)
        .expect("chronology cell exists");
    cell.freshness = ProofFreshnessClass::Stale;

    // Recompute the verdict for the tampered row only; leave the rest stale so the
    // packet's stored roll-up no longer matches and validate() must object.
    let (verdict, _) = row.computed_verdict();
    assert_eq!(verdict, CertificationVerdict::Narrowed);

    let violations = packet.validate();
    assert!(
        violations.iter().any(|v| matches!(
            v,
            M5RecordsPolicyCertificationViolation::VerdictMismatch { .. }
        )),
        "{violations:?}"
    );
}

#[test]
fn missing_proof_dimension_is_rejected() {
    let mut packet = seeded_m5_records_policy_certification_packet();
    packet.rows[0]
        .proof_cells
        .retain(|cell| cell.dimension != CertificationProofDimension::PolicySimulation);

    let violations = packet.validate();
    assert!(
        violations.iter().any(|v| matches!(
            v,
            M5RecordsPolicyCertificationViolation::ProofDimensionMissing {
                dimension: CertificationProofDimension::PolicySimulation,
                ..
            }
        )),
        "{violations:?}"
    );
}

#[test]
fn local_only_managed_claim_is_rejected() {
    let mut packet = seeded_m5_records_policy_certification_packet();
    if let Some(row) = packet
        .rows
        .iter_mut()
        .find(|row| row.authority_boundary == AuthorityBoundaryClass::LocalOnly)
    {
        row.claims_managed_delete = true;
        let violations = packet.validate();
        assert!(
            violations.iter().any(|v| matches!(
                v,
                M5RecordsPolicyCertificationViolation::ManagedControlOverclaimed { .. }
            )),
            "{violations:?}"
        );
    }
}

#[test]
fn cosmetic_done_label_over_narrowed_row_is_rejected() {
    let mut packet = seeded_m5_records_policy_certification_packet();
    let row = &mut packet.rows[0];
    row.proof_cells[0].observed = false;
    let (verdict, reasons) = row.computed_verdict();
    row.verdict = verdict;
    row.narrow_reasons = reasons;
    // Leave a cosmetically clean "certified" label on a now-narrowed row.
    row.shiproom_label = "M5 record governance certified".to_owned();

    let violations = packet.validate();
    assert!(
        violations.iter().any(|v| matches!(
            v,
            M5RecordsPolicyCertificationViolation::ClaimLabelInconsistent {
                surface: CertificationConsumerSurface::Shiproom,
                ..
            }
        )),
        "{violations:?}"
    );
}

#[test]
fn tampered_proof_source_is_caught_by_live_cross_check() {
    let mut packet = seeded_m5_records_policy_certification_packet();
    let cell = packet.rows[0]
        .proof_cells
        .iter_mut()
        .find(|cell| cell.dimension == CertificationProofDimension::HoldDelete)
        .expect("hold/delete cell exists");
    cell.source_record_kind = "not_the_real_packet".to_owned();

    let findings = packet.verify_against_live_packets();
    assert!(
        findings.iter().any(|f| matches!(
            f,
            CertificationCrossCheckFinding::ProofSourceMismatch {
                dimension: CertificationProofDimension::HoldDelete,
                ..
            }
        )),
        "{findings:?}"
    );
}

#[test]
fn checked_in_canonical_fixture_matches_seeded_packet() {
    let fixture = repo_root()
        .join("fixtures/governance/m5_records_policy_certification/canonical_packet.yaml");
    let raw = std::fs::read_to_string(&fixture).expect("canonical fixture is readable");
    let parsed: M5RecordsPolicyCertificationPacket =
        serde_yaml::from_str(&raw).expect("canonical fixture parses");

    assert!(
        parsed.validate().is_empty(),
        "canonical fixture must validate cleanly: {:?}",
        parsed.validate()
    );
    assert_eq!(
        parsed,
        seeded_m5_records_policy_certification_packet(),
        "canonical fixture drifted from the seeded packet; regenerate it"
    );
}
