//! Unit tests for the relation-navigation qualification certification: the
//! all-green canonical binding, the auto-narrowing function, surface-claim
//! aggregation, release evidence, consumer projections, and export safety.

use super::*;

fn posture(
    family: RelationNavQualificationFamily,
    proof_state: ProofState,
    proof_freshness: ProofFreshness,
) -> FamilyProofPosture {
    FamilyProofPosture {
        family,
        proof_state,
        proof_freshness,
    }
}

#[test]
fn canonical_certification_validates_and_is_all_green() {
    let cert = relation_navigation_qualification();
    cert.validate().expect("canonical certification validates");
    assert!(cert.all_invariants_hold());
    assert!(!cert.invariants.is_empty());
    assert!(cert.all_claims_qualified);
    assert!(cert.rows.iter().all(|r| r.claim_state.is_green()));
    assert!(cert.narrowed_rows().next().is_none());
    assert!(cert.withdrawn_rows().next().is_none());
}

#[test]
fn certification_is_deterministic() {
    assert_eq!(
        relation_navigation_qualification(),
        relation_navigation_qualification()
    );
}

#[test]
fn certification_is_support_export_safe() {
    let cert = relation_navigation_qualification();
    assert!(cert.raw_payload_excluded);
    assert!(cert.is_support_export_safe());
}

#[test]
fn every_family_is_present_once_with_proof() {
    let cert = relation_navigation_qualification();
    assert_eq!(
        cert.families.len(),
        RelationNavQualificationFamily::ALL.len()
    );
    for family in RelationNavQualificationFamily::ALL {
        let entry = cert.family(family).expect("family present");
        assert_eq!(entry.family_id, family.family_id());
        assert!(!entry.certified_object_refs.is_empty());
        assert!(entry
            .certified_object_refs
            .iter()
            .all(|r| r.starts_with("relation_nav_object.")));
        assert!(!entry.produced_by_refs.is_empty());
        assert!(!entry.canonical_schema_refs.is_empty());
        assert!(!entry.proof_packet_ref.is_empty());
        assert!(!entry.freeze_gate_ref.is_empty());
        assert!(!entry.relation_kinds.is_empty());
        assert!(!entry.claimed_surfaces.is_empty());
    }
}

/// The narrowing function is the heart of the lane: a claim is green only when its
/// proof is passing and current; everything else narrows or withdraws.
#[test]
fn narrow_claim_covers_every_proof_and_freshness() {
    // Passing + current => qualified.
    assert_eq!(
        narrow_claim(ProofState::Passing, ProofFreshness::Live),
        ClaimState::Qualified
    );
    assert_eq!(
        narrow_claim(ProofState::Passing, ProofFreshness::Warm),
        ClaimState::Qualified
    );
    // Passing but degraded / stale => narrowed.
    assert_eq!(
        narrow_claim(ProofState::Passing, ProofFreshness::Degraded),
        ClaimState::NarrowedDisclosed
    );
    assert_eq!(
        narrow_claim(ProofState::Passing, ProofFreshness::Stale),
        ClaimState::NarrowedStale
    );
    // Passing but unverified => withdrawn pending proof.
    assert_eq!(
        narrow_claim(ProofState::Passing, ProofFreshness::Unverified),
        ClaimState::WithdrawnPendingProof
    );
    // Pending / missing => withdrawn pending proof regardless of freshness.
    for freshness in ProofFreshness::ALL {
        assert_eq!(
            narrow_claim(ProofState::Pending, freshness),
            ClaimState::WithdrawnPendingProof
        );
        assert_eq!(
            narrow_claim(ProofState::Missing, freshness),
            ClaimState::WithdrawnPendingProof
        );
        // Failing => withdrawn failing regardless of freshness.
        assert_eq!(
            narrow_claim(ProofState::Failing, freshness),
            ClaimState::WithdrawnFailing
        );
    }
}

/// No proof state and freshness combination ever leaves a claim green unless the
/// proof is passing and current.
#[test]
fn green_claim_requires_passing_and_current_proof() {
    for proof in ProofState::ALL {
        for freshness in ProofFreshness::ALL {
            let state = narrow_claim(proof, freshness);
            if state.is_green() {
                assert_eq!(proof, ProofState::Passing);
                assert!(freshness.is_current());
            }
        }
    }
}

/// Feeding a stale posture for one family narrows that family's rows and the
/// surfaces it backs, while leaving the other surfaces qualified.
#[test]
fn stale_proof_narrows_only_the_affected_family_and_surfaces() {
    let input = RelationNavQualificationInput {
        as_of: RELATION_NAV_QUALIFICATION_AS_OF.to_owned(),
        postures: vec![posture(
            RelationNavQualificationFamily::HierarchyProofClasses,
            ProofState::Passing,
            ProofFreshness::Stale,
        )],
    };
    let cert = certify(&input);
    cert.validate()
        .expect("narrowed certification still validates");
    assert!(!cert.all_claims_qualified);

    // Every hierarchy row narrowed; every other family stays qualified.
    for row in &cert.rows {
        if row.family == RelationNavQualificationFamily::HierarchyProofClasses {
            assert_eq!(row.claim_state, ClaimState::NarrowedStale);
            assert!(row.narrowing_reason.is_some());
            assert!(row.disclosure_note.is_some());
        } else {
            assert_eq!(row.claim_state, ClaimState::Qualified);
        }
    }

    // The hierarchy family backs graph/topology, editor, and docs — those surfaces
    // narrow; search/navigation (which hierarchy does not back) stays qualified.
    assert_eq!(
        cert.surface_claim_state(ClaimedSurface::GraphTopology),
        ClaimState::NarrowedStale
    );
    assert_eq!(
        cert.surface_claim_state(ClaimedSurface::SearchNavigation),
        ClaimState::Qualified
    );

    // The release evidence row for the family reflects the narrowing.
    let evidence = cert
        .release_evidence
        .iter()
        .find(|e| e.family == RelationNavQualificationFamily::HierarchyProofClasses)
        .expect("hierarchy evidence present");
    assert_eq!(evidence.claim_state, ClaimState::NarrowedStale);
    assert!(!evidence.holds);
}

/// Feeding a failing posture withdraws the claim entirely and the surface
/// aggregates to its worst row.
#[test]
fn failing_proof_withdraws_the_claim() {
    let input = RelationNavQualificationInput {
        as_of: RELATION_NAV_QUALIFICATION_AS_OF.to_owned(),
        postures: vec![posture(
            RelationNavQualificationFamily::RenamePreviewCompleteness,
            ProofState::Failing,
            ProofFreshness::Live,
        )],
    };
    let cert = certify(&input);
    cert.validate()
        .expect("withdrawn certification still validates");
    assert!(!cert.all_claims_qualified);

    for row in cert
        .rows
        .iter()
        .filter(|r| r.family == RelationNavQualificationFamily::RenamePreviewCompleteness)
    {
        assert_eq!(row.claim_state, ClaimState::WithdrawnFailing);
        assert!(row.claim_state.is_withdrawn());
    }

    // Editor assist is backed by rename-preview, so its claim withdraws.
    assert_eq!(
        cert.surface_claim_state(ClaimedSurface::EditorAssist),
        ClaimState::WithdrawnFailing
    );
    assert!(cert.withdrawn_rows().count() >= 2);
}

/// A surface aggregates to the most-severe claim across all the families backing
/// it.
#[test]
fn surface_claim_aggregates_to_worst_row() {
    let input = RelationNavQualificationInput {
        as_of: RELATION_NAV_QUALIFICATION_AS_OF.to_owned(),
        postures: vec![
            posture(
                RelationNavQualificationFamily::TargetKindHonesty,
                ProofState::Passing,
                ProofFreshness::Degraded,
            ),
            posture(
                RelationNavQualificationFamily::ReferenceAccessKindTruth,
                ProofState::Failing,
                ProofFreshness::Live,
            ),
        ],
    };
    let cert = certify(&input);
    // Search/navigation is backed by both target-kind (narrowed) and references
    // (withdrawn-failing); it aggregates to the worse of the two.
    assert_eq!(
        cert.surface_claim_state(ClaimedSurface::SearchNavigation),
        ClaimState::WithdrawnFailing
    );
}

#[test]
fn release_evidence_names_required_families() {
    let cert = relation_navigation_qualification();
    for family in RelationNavQualificationFamily::NAMED_RELEASE_EVIDENCE_FAMILIES {
        assert!(
            cert.release_evidence.iter().any(|e| e.family == family),
            "release evidence must name {}",
            family.as_str()
        );
    }
    // Canonical evidence all holds.
    assert!(cert.release_evidence.iter().all(|e| e.holds));
}

#[test]
fn every_consumer_surface_consumes_shared_state() {
    let cert = relation_navigation_qualification();
    for consumer in QualificationConsumer::ALL {
        let projection = cert
            .consumer_projections
            .iter()
            .find(|p| p.consumer == consumer)
            .expect("consumer projection present");
        assert!(projection.consumes_shared_state);
        assert!(!projection.restates_manually);
        assert!(!projection.surfaced_family_tokens.is_empty());
    }
}

#[test]
fn every_claimed_surface_is_governed() {
    let cert = relation_navigation_qualification();
    for surface in ClaimedSurface::ALL {
        assert!(
            cert.rows.iter().any(|r| r.claimed_surface == surface),
            "surface {} must be governed by a row",
            surface.as_str()
        );
    }
}

#[test]
fn freshness_classes_carry_provenance() {
    for freshness in ProofFreshness::ALL {
        let provenance = freshness.derived_from_ref();
        assert!(provenance.starts_with("crates/aureline-navigation/src/target_model/mod.rs#"));
        // The mapped upstream class is consistent with the disclosure rule.
        assert_eq!(
            freshness.requires_disclosure(),
            freshness.freshness_class().requires_disclosure()
        );
    }
}

#[test]
fn lines_projection_is_non_empty_and_lists_families() {
    let cert = relation_navigation_qualification();
    let lines = relation_navigation_qualification_lines(&cert);
    assert!(lines
        .iter()
        .any(|l| l.contains("Relation-navigation qualification")));
    for family in RelationNavQualificationFamily::ALL {
        assert!(
            lines.iter().any(|l| l.contains(family.as_str())),
            "projection should mention {}",
            family.as_str()
        );
    }
}

#[test]
fn roundtrips_through_json() {
    let cert = relation_navigation_qualification();
    let json = serde_json::to_string(&cert).expect("serializes");
    let back: RelationNavQualificationCertification =
        serde_json::from_str(&json).expect("deserializes");
    assert_eq!(cert, back);
}

#[test]
fn validation_rejects_a_dropped_proof_packet() {
    let mut cert = relation_navigation_qualification();
    cert.families[0].proof_packet_ref.clear();
    let err = cert
        .validate()
        .expect_err("must reject a dropped proof packet");
    assert!(err.to_string().contains("proof packet") || err.to_string().contains("not support"));
}

#[test]
fn validation_rejects_a_hand_forced_green_claim() {
    // Tamper a row to claim qualified while its proof is failing — the narrowing
    // invariant and validate() must reject it.
    let mut cert = relation_navigation_qualification();
    cert.rows[0].proof_state = ProofState::Failing;
    cert.rows[0].claim_state = ClaimState::Qualified;
    let err = cert
        .validate()
        .expect_err("must reject a hand-forced green claim");
    assert!(err.to_string().contains("does not match narrow_claim"));
}
