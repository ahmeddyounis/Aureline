//! Freeze gate for the M5 admin-certification bundle.
//!
//! The checked-in fixture
//! `fixtures/admin/m5-admin-certification/canonical_certification.json` is the
//! published qualification capstone. This gate rebuilds the bundle in code and
//! asserts it equals the fixture after a serialize round-trip, so the admin-plane
//! qualification state cannot drift from the published artifact without failing
//! CI. It also re-proves support-export safety, full profile and family coverage,
//! that every certified surface is one the frozen matrix admits and every claim
//! state is in its vocabulary, that no row reads green on stale or failing proof,
//! that stale or failing evidence auto-narrows the managed claim, that the
//! release-evidence rows carry the worst case, and every frozen invariant.

use std::path::{Path, PathBuf};

use aureline_policy::m5_admin_certification::{
    admin_certification_bundle, admin_certification_lines, AdminCertificationBundle,
    CertifiedFamilyClass, QualificationClass, ReleaseEvidenceDimensionClass, CERTIFIED_PROFILES,
    M5_ADMIN_CERTIFICATION_RECORD_KIND, M5_ADMIN_CERTIFICATION_SCHEMA_REF,
};
use aureline_policy::m5_admin_plane::{admin_plane_matrix, AdminStateClass};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/admin/m5-admin-certification/canonical_certification.json")
}

fn load_fixture() -> AdminCertificationBundle {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_bundle_matches_checked_in_fixture() {
    let built = admin_certification_bundle();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code admin-certification bundle drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-policy --example dump_m5_admin_certification`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_ADMIN_CERTIFICATION_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_ADMIN_CERTIFICATION_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: AdminCertificationBundle =
        serde_json::from_str(&serde_json::to_string(&fixture).expect("serializes"))
            .expect("round-trips");
    assert_eq!(roundtrip, fixture);
}

#[test]
fn every_frozen_invariant_holds() {
    let fixture = load_fixture();
    assert!(!fixture.invariants.is_empty());
    for invariant in &fixture.invariants {
        assert!(
            invariant.holds,
            "frozen invariant must hold: {}",
            invariant.invariant_id
        );
    }
    assert!(fixture.all_invariants_hold());
}

#[test]
fn bundle_certifies_every_profile_and_family() {
    let fixture = load_fixture();
    assert_eq!(fixture.profiles.len(), CERTIFIED_PROFILES.len());
    for profile in CERTIFIED_PROFILES {
        let packet = fixture.packet(profile).expect("profile present");
        assert_eq!(packet.profile_id, profile.path_id());
        for family in CertifiedFamilyClass::ALL {
            assert!(
                packet.row(family).is_some(),
                "family {} missing",
                family.as_str()
            );
        }
    }
}

#[test]
fn certified_surfaces_stay_within_the_frozen_matrix() {
    let fixture = load_fixture();
    let matrix = admin_plane_matrix();
    for row in fixture.rows() {
        for surface in &row.bound_surfaces {
            let entry = matrix.surface(*surface).expect("surface present in matrix");
            assert!(entry.locally_explainable);
            assert!(entry.typed_not_portal_only);
        }
        assert!(matrix.state_term(row.claim_state).is_some());
    }
    for packet in &fixture.profiles {
        assert!(matrix.state_term(packet.claim_state).is_some());
    }
}

#[test]
fn no_row_reads_green_on_stale_or_failing_proof() {
    let fixture = load_fixture();
    for row in fixture.rows() {
        if row.is_qualified() {
            assert!(!row.proof_failing);
            assert!(!row.proof_freshness.is_stale());
            assert!(row.proof_lane.is_proven());
            assert_eq!(row.claim_state, AdminStateClass::ActiveEnforced);
        } else {
            assert_ne!(row.claim_state, AdminStateClass::ActiveEnforced);
            assert!(row.narrow_reason.is_some());
        }
    }
}

#[test]
fn stale_or_failing_evidence_auto_narrows_the_managed_claim() {
    let fixture = load_fixture();
    for packet in &fixture.profiles {
        let all_qualified = packet.claimed_rows().all(|r| r.is_qualified());
        if all_qualified {
            assert!(
                packet.claim_confirmed(),
                "{} should be confirmed",
                packet.profile.as_str()
            );
        } else {
            assert!(
                !packet.claim_confirmed(),
                "{} should narrow",
                packet.profile.as_str()
            );
            assert!(!packet.narrow_reasons.is_empty());
        }
    }
}

#[test]
fn release_evidence_covers_every_dimension_and_reflects_worst() {
    let fixture = load_fixture();
    assert_eq!(
        fixture.release_evidence.len(),
        ReleaseEvidenceDimensionClass::ALL.len()
    );
    for dimension in ReleaseEvidenceDimensionClass::ALL {
        let row = fixture
            .release_evidence_row(dimension)
            .unwrap_or_else(|| panic!("dimension {} present", dimension.as_str()));
        assert!(!row.families.is_empty());
        assert_eq!(row.claim_state, row.worst_qualification.claim_state());
    }
    // Audit history surfaces the failing self-hosted proof.
    let audit = fixture
        .release_evidence_row(ReleaseEvidenceDimensionClass::AuditHistory)
        .expect("audit dimension");
    assert_eq!(
        audit.worst_qualification,
        QualificationClass::NarrowedFailingProof
    );
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = admin_certification_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Admin-certification bundle")));
    for profile in CERTIFIED_PROFILES {
        assert!(
            lines.iter().any(|line| line.contains(profile.as_str())),
            "projection must mention profile {}",
            profile.as_str()
        );
    }
}
