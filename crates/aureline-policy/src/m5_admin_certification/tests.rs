//! Unit tests for the M5 admin-certification bundle.

use super::*;
use crate::m5_admin_plane::admin_plane_matrix;

#[test]
fn bundle_is_deterministic() {
    assert_eq!(admin_certification_bundle(), admin_certification_bundle());
}

#[test]
fn bundle_validates_and_all_invariants_hold() {
    let bundle = admin_certification_bundle();
    bundle.validate().expect("bundle validates");
    assert!(bundle.all_invariants_hold());
    assert!(!bundle.invariants.is_empty());
}

#[test]
fn bundle_round_trips_through_json() {
    let bundle = admin_certification_bundle();
    let json = serde_json::to_string(&bundle).expect("serialize");
    let back: AdminCertificationBundle = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(bundle, back);
}

#[test]
fn bundle_covers_every_profile_and_family() {
    let bundle = admin_certification_bundle();
    assert_eq!(bundle.profiles.len(), CERTIFIED_PROFILES.len());
    for profile in CERTIFIED_PROFILES {
        let packet = bundle.packet(profile).expect("profile present");
        assert_eq!(packet.profile_id, profile.path_id());
        for family in CertifiedFamilyClass::ALL {
            let row = packet.row(family).expect("family row present");
            assert_eq!(row.row_id, super::row_id(profile, family));
            assert!(!row.bound_surfaces.is_empty());
        }
    }
}

#[test]
fn every_bound_surface_is_present_and_typed_in_the_matrix() {
    let bundle = admin_certification_bundle();
    let matrix = admin_plane_matrix();
    for row in bundle.rows() {
        for surface in &row.bound_surfaces {
            let entry = matrix.surface(*surface).expect("surface present");
            assert!(
                entry.locally_explainable,
                "{} portal-only",
                surface.as_str()
            );
            assert!(
                entry.typed_not_portal_only,
                "{} not typed",
                surface.as_str()
            );
        }
    }
}

#[test]
fn every_claim_state_is_in_the_matrix_vocabulary() {
    let bundle = admin_certification_bundle();
    let matrix = admin_plane_matrix();
    for packet in &bundle.profiles {
        assert!(matrix.state_term(packet.claim_state).is_some());
        for row in &packet.families {
            assert!(matrix.state_term(row.claim_state).is_some());
        }
    }
    for row in &bundle.release_evidence {
        assert!(matrix.state_term(row.claim_state).is_some());
    }
}

#[test]
fn managed_cloud_is_fully_confirmed() {
    let bundle = admin_certification_bundle();
    let managed = bundle
        .packet(AdminPathClass::ManagedCloud)
        .expect("managed present");
    assert!(managed.claim_confirmed());
    assert!(managed.narrow_reasons.is_empty());
    assert!(managed.families.iter().all(|r| r.is_qualified()));
    assert_eq!(managed.claim_state, AdminStateClass::ActiveEnforced);
}

#[test]
fn failing_audit_proof_narrows_the_self_hosted_claim() {
    let bundle = admin_certification_bundle();
    let self_hosted = bundle
        .packet(AdminPathClass::SelfHosted)
        .expect("self-hosted present");
    assert!(!self_hosted.claim_confirmed());
    assert!(self_hosted
        .narrow_reasons
        .contains(&ClaimNarrowReasonClass::FamilyProofFailing));

    let audit = self_hosted
        .row(CertifiedFamilyClass::DecisionHistory)
        .expect("decision-history row");
    assert!(audit.proof_failing);
    assert_eq!(
        audit.qualification,
        QualificationClass::NarrowedFailingProof
    );
    assert_eq!(audit.claim_state, AdminStateClass::UnconfirmedStale);

    // The other families on this profile still qualify; only the failing one narrows.
    let still_green = self_hosted
        .row(CertifiedFamilyClass::PolicyExplainability)
        .expect("policy row");
    assert!(still_green.is_qualified());
}

#[test]
fn stale_evidence_narrows_the_sovereign_claim() {
    let bundle = admin_certification_bundle();
    let sovereign = bundle
        .packet(AdminPathClass::SovereignAirGapped)
        .expect("sovereign present");
    assert!(!sovereign.claim_confirmed());
    assert!(sovereign
        .narrow_reasons
        .contains(&ClaimNarrowReasonClass::FamilyEvidenceStale));
    let posture = sovereign
        .row(CertifiedFamilyClass::EndpointPosture)
        .expect("endpoint-posture row");
    assert!(posture.proof_freshness.is_stale());
    assert_eq!(
        posture.qualification,
        QualificationClass::NarrowedStaleEvidence
    );
}

#[test]
fn stale_mirror_narrows_the_mirrored_claim() {
    let bundle = admin_certification_bundle();
    let mirrored = bundle
        .packet(AdminPathClass::MirroredOffline)
        .expect("mirrored present");
    assert!(!mirrored.claim_confirmed());
    assert!(mirrored
        .narrow_reasons
        .contains(&ClaimNarrowReasonClass::MirrorEvidenceStale));
    let retention = mirrored
        .row(CertifiedFamilyClass::RetentionDelete)
        .expect("retention row");
    assert!(retention.mirror_backed);
    assert_eq!(
        retention.narrow_reason,
        Some(ClaimNarrowReasonClass::MirrorEvidenceStale)
    );
}

#[test]
fn a_row_is_green_only_when_proof_is_fresh_and_passing() {
    let bundle = admin_certification_bundle();
    for row in bundle.rows() {
        if row.is_qualified() {
            assert!(!row.proof_failing);
            assert!(!row.proof_freshness.is_stale());
            assert!(row.proof_lane.is_proven());
            assert_eq!(row.claim_state, AdminStateClass::ActiveEnforced);
            assert!(row.narrow_reason.is_none());
        } else {
            assert_ne!(row.claim_state, AdminStateClass::ActiveEnforced);
            assert!(row.narrow_reason.is_some());
        }
    }
}

#[test]
fn every_family_row_cites_a_proven_proof_lane() {
    let bundle = admin_certification_bundle();
    for row in bundle.rows() {
        assert!(
            row.proof_lane.is_proven(),
            "{} cites an unproven lane",
            row.row_id
        );
        assert!(!row.proof_lane.record_kind.is_empty());
    }
}

#[test]
fn release_evidence_has_every_dimension_and_reflects_the_worst_case() {
    let bundle = admin_certification_bundle();
    assert_eq!(
        bundle.release_evidence.len(),
        ReleaseEvidenceDimensionClass::ALL.len()
    );
    for dimension in ReleaseEvidenceDimensionClass::ALL {
        let row = bundle
            .release_evidence_row(dimension)
            .expect("dimension present");
        assert!(!row.families.is_empty());
        assert_eq!(
            row.worst_qualification,
            worst_qualification(&bundle.profiles, &row.families)
        );
        assert_eq!(row.claim_state, row.worst_qualification.claim_state());
    }

    // The audit-history dimension surfaces the failing self-hosted proof.
    let audit = bundle
        .release_evidence_row(ReleaseEvidenceDimensionClass::AuditHistory)
        .expect("audit dimension");
    assert_eq!(
        audit.worst_qualification,
        QualificationClass::NarrowedFailingProof
    );
}

#[test]
fn bundle_is_support_export_safe() {
    let bundle = admin_certification_bundle();
    assert!(bundle.raw_payload_excluded);
    assert!(bundle.is_support_export_safe());
}

#[test]
fn parity_consumers_render_every_packet() {
    let bundle = admin_certification_bundle();
    for packet in &bundle.profiles {
        for consumer in PARITY_CONSUMERS {
            assert!(
                packet.consumers.contains(&consumer),
                "{} missing consumer {:?}",
                packet.profile.as_str(),
                consumer
            );
        }
    }
}

#[test]
fn human_readable_projection_mentions_every_profile() {
    let bundle = admin_certification_bundle();
    let lines = admin_certification_lines(&bundle);
    assert!(lines
        .iter()
        .any(|l| l.contains("Admin-certification bundle")));
    for profile in CERTIFIED_PROFILES {
        assert!(
            lines.iter().any(|l| l.contains(profile.as_str())),
            "projection must mention profile {}",
            profile.as_str()
        );
    }
}

#[test]
fn validate_rejects_a_row_marked_green_with_stale_proof() {
    let mut bundle = admin_certification_bundle();
    // Force a qualified verdict onto a row whose proof is actually stale.
    let row = bundle
        .profiles
        .iter_mut()
        .flat_map(|p| p.families.iter_mut())
        .find(|r| r.proof_freshness.is_stale())
        .expect("a stale row exists");
    row.qualification = QualificationClass::Qualified;
    row.claim_state = AdminStateClass::ActiveEnforced;
    row.narrow_reason = None;

    let rebuilt = AdminCertificationBundle {
        invariants: compute_invariants(&bundle.profiles, &bundle.release_evidence),
        ..bundle
    };
    assert!(rebuilt.validate().is_err());
}
