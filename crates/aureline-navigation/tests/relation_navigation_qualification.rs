//! Freeze gate for the relation-navigation qualification certification.
//!
//! The checked-in fixture
//! `fixtures/navigation/relation_navigation_qualification/canonical_certification.json`
//! is the published certification. This gate rebuilds the certification in code and
//! asserts it equals the fixture after a serialize round-trip, so the relation-
//! navigation qualification contract cannot drift from the published artifact
//! without failing CI. It also re-proves support-export safety, full family/surface
//! coverage, that every family maps to a proof packet, that the narrowing function
//! is applied to every row, that no row stays green without current passing proof,
//! and every frozen invariant. This test runs under `cargo test --workspace`, so
//! stable promotion cannot harden a relation-navigation claim without current proof.

use std::path::{Path, PathBuf};

use aureline_navigation::relation_navigation_qualification::{
    certify, narrow_claim, relation_navigation_qualification, ClaimState, ClaimedSurface,
    FamilyProofPosture, ProofFreshness, ProofState, QualificationConsumer,
    RelationNavQualificationCertification, RelationNavQualificationFamily,
    RelationNavQualificationInput, RELATION_NAV_QUALIFICATION_AS_OF,
    RELATION_NAV_QUALIFICATION_RECORD_KIND, RELATION_NAV_QUALIFICATION_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../fixtures/navigation/relation_navigation_qualification/canonical_certification.json",
    )
}

fn load_fixture() -> RelationNavQualificationCertification {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_certification_matches_checked_in_fixture() {
    let built = relation_navigation_qualification();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code relation-navigation qualification certification drifted from the checked-in \
         fixture; regenerate it with `cargo run -p aureline-navigation --example \
         dump_relation_navigation_qualification`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, RELATION_NAV_QUALIFICATION_RECORD_KIND);
    assert_eq!(fixture.schema_ref, RELATION_NAV_QUALIFICATION_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: RelationNavQualificationCertification =
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
fn certification_covers_every_family_and_surface() {
    let fixture = load_fixture();
    assert_eq!(
        fixture.families.len(),
        RelationNavQualificationFamily::ALL.len()
    );
    for family in RelationNavQualificationFamily::ALL {
        assert!(
            fixture.family(family).is_some(),
            "missing family {}",
            family.as_str()
        );
    }
    for surface in ClaimedSurface::ALL {
        assert!(
            fixture.rows.iter().any(|r| r.claimed_surface == surface),
            "surface {} is governed by no row",
            surface.as_str()
        );
    }
    for consumer in QualificationConsumer::ALL {
        assert!(
            fixture
                .consumer_projections
                .iter()
                .any(|p| p.consumer == consumer),
            "consumer {} consumes nothing",
            consumer.as_str()
        );
    }
}

#[test]
fn every_family_maps_to_a_proof_packet_and_gate() {
    let fixture = load_fixture();
    for family in &fixture.families {
        assert!(
            !family.proof_packet_ref.is_empty() && !family.freeze_gate_ref.is_empty(),
            "family {} lacks a mapped proof packet or freeze gate",
            family.family.as_str()
        );
    }
}

#[test]
fn fixture_narrowing_is_applied_to_every_row() {
    let fixture = load_fixture();
    for row in &fixture.rows {
        assert_eq!(
            row.claim_state,
            narrow_claim(row.proof_state, row.proof_freshness),
            "row {} claim_state must equal the narrowing of its proof",
            row.row_id
        );
        if row.claim_state.is_green() {
            assert_eq!(row.proof_state, ProofState::Passing);
            assert!(row.proof_freshness.is_current());
            assert!(row.narrowing_reason.is_none());
            assert!(row.disclosure_note.is_none());
        } else {
            assert!(row.narrowing_reason.is_some());
            assert!(row.disclosure_note.is_some());
        }
    }
}

#[test]
fn release_evidence_names_the_required_families() {
    let fixture = load_fixture();
    for family in RelationNavQualificationFamily::NAMED_RELEASE_EVIDENCE_FAMILIES {
        assert!(
            fixture.release_evidence.iter().any(|e| e.family == family),
            "release evidence missing required family {}",
            family.as_str()
        );
    }
}

/// The release-automation behavior the lane exists for: a stale or failing family
/// posture narrows or withdraws the affected surface claim, and the certification
/// still validates so the downgrade is governed rather than a crash.
#[test]
fn stale_or_failing_posture_narrows_the_surface_claim() {
    let stale = certify(&RelationNavQualificationInput {
        as_of: RELATION_NAV_QUALIFICATION_AS_OF.to_owned(),
        postures: vec![FamilyProofPosture {
            family: RelationNavQualificationFamily::TargetKindHonesty,
            proof_state: ProofState::Passing,
            proof_freshness: ProofFreshness::Stale,
        }],
    });
    stale.validate().expect("stale certification validates");
    assert!(!stale.all_claims_qualified);
    assert_eq!(
        stale.surface_claim_state(ClaimedSurface::SearchNavigation),
        ClaimState::NarrowedStale
    );

    let failing = certify(&RelationNavQualificationInput {
        as_of: RELATION_NAV_QUALIFICATION_AS_OF.to_owned(),
        postures: vec![FamilyProofPosture {
            family: RelationNavQualificationFamily::TargetKindHonesty,
            proof_state: ProofState::Failing,
            proof_freshness: ProofFreshness::Live,
        }],
    });
    failing.validate().expect("failing certification validates");
    assert_eq!(
        failing.surface_claim_state(ClaimedSurface::SearchNavigation),
        ClaimState::WithdrawnFailing
    );
}
