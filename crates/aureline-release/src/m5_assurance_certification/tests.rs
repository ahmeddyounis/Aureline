//! Inline tests for the M5 assurance certification lane.

use super::*;

fn packet() -> M5AssuranceCertification {
    seeded_m5_assurance_certification()
}

/// Expected number of claimed profiles.
const EXPECTED_PROFILES: usize = ClaimedPosture::ALL.len();

#[test]
fn canonical_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_ASSURANCE_CERTIFICATION_PACKET_ID);
    assert_eq!(packet.record_kind, M5_ASSURANCE_CERTIFICATION_RECORD_KIND);
    assert_eq!(packet.profiles.len(), EXPECTED_PROFILES);
    assert_eq!(packet.consumers.len(), CertificationConsumer::ALL.len());
    assert!(packet.conformance.all_hold());
    assert!(packet.vocabulary.matches_canonical());
}

#[test]
fn grid_covers_every_profile() {
    // Acceptance criterion: every claimed M5 profile is mapped to proof for every applicable
    // dimension.
    let packet = packet();
    for profile in ClaimedPosture::ALL {
        let claim = packet
            .profile(profile)
            .unwrap_or_else(|| panic!("{} claimed", profile.as_str()));
        assert_eq!(claim.cells.len(), CertificationDimension::ALL.len());
        for cell in &claim.cells {
            if cell.is_applicable() {
                assert!(
                    !cell.proof_refs.is_empty(),
                    "{} {} applicable but has no proof",
                    profile.as_str(),
                    cell.dimension.as_str()
                );
                assert_eq!(cell.proof_refs.len(), cell.backing_facets.len());
            }
        }
    }
}

#[test]
fn canonical_certifies_every_profile_and_consumer() {
    // Acceptance criterion: with fresh proof, every profile stands at its claimed Stable
    // qualification.
    let packet = packet();
    for claim in &packet.profiles {
        assert!(
            claim.is_certified(),
            "profile `{}` not certified when every facet is current",
            claim.claim_ref
        );
        assert_eq!(claim.effective_qualification, QualificationClass::Stable);
        assert!(claim.cells.iter().all(|c| c.gap_kind.is_none()));
    }
    for c in &packet.consumers {
        assert!(
            c.is_certified(),
            "consumer `{}` not certified",
            c.consumer.as_str()
        );
        assert!(c.narrowed_profile_refs.is_empty());
        assert!(c.blocked_profile_refs.is_empty());
    }
    assert_eq!(packet.summary.certified_profiles, EXPECTED_PROFILES as u32);
    assert_eq!(packet.summary.narrowed_profiles, 0);
    assert_eq!(packet.summary.blocked_profiles, 0);
    assert_eq!(
        packet.summary.certified_consumers,
        CertificationConsumer::ALL.len() as u32
    );
    assert!(!packet.blocks_stable_promotion());
}

#[test]
fn every_named_dimension_applies_to_every_profile() {
    // In this lane every claimed profile is scoped by at least one facet in every dimension, so the
    // grid carries no hidden not-applicable cells; the honest result is full applicability.
    let packet = packet();
    for claim in &packet.profiles {
        assert_eq!(
            claim.applicable_dimensions.len(),
            CertificationDimension::ALL.len(),
            "profile `{}` has a non-applicable dimension",
            claim.claim_ref
        );
        for cell in &claim.cells {
            assert!(cell.is_applicable());
            assert_ne!(cell.outcome, CertificationOutcome::NotApplicable);
        }
    }
}

#[test]
fn sovereign_drops_non_scoping_facets() {
    // The exception / waiver and approval-ticket facets do not scope to a sovereign / air-gapped
    // profile, so the projection drops them from that profile's backing set rather than overstating
    // — while a managed profile keeps them.
    let packet = packet();
    let sovereign = packet.profile(ClaimedPosture::Sovereign).unwrap();
    let managed = packet.profile(ClaimedPosture::Managed).unwrap();

    let sov_assurance = sovereign
        .cell(CertificationDimension::AssuranceCenter)
        .unwrap();
    assert!(!sov_assurance
        .backing_facets
        .contains(&AssuranceFacet::ExceptionWaiver));
    let managed_assurance = managed
        .cell(CertificationDimension::AssuranceCenter)
        .unwrap();
    assert!(managed_assurance
        .backing_facets
        .contains(&AssuranceFacet::ExceptionWaiver));

    let sov_boundary = sovereign
        .cell(CertificationDimension::BoundaryRoute)
        .unwrap();
    assert!(!sov_boundary
        .backing_facets
        .contains(&AssuranceFacet::ApprovalTicket));
    let managed_boundary = managed.cell(CertificationDimension::BoundaryRoute).unwrap();
    assert!(managed_boundary
        .backing_facets
        .contains(&AssuranceFacet::ApprovalTicket));

    // Dropping a non-scoping facet never blocks the dimension — its other facets still apply.
    assert!(sov_assurance.is_applicable());
    assert!(sov_boundary.is_applicable());
}

#[test]
fn stale_route_proof_narrows_boundary_route_per_profile() {
    // Acceptance criterion: stale proof narrows the profile deterministically — only on the
    // boundary / route dimension the stale route-hop facet backs, not behind a generic stable badge.
    let packet = seeded_m5_assurance_certification_stale_proof_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    for claim in &packet.profiles {
        assert!(
            claim.is_narrowed(),
            "profile `{}` did not narrow under a stale route-hop proof",
            claim.claim_ref
        );
        let boundary = claim.cell(CertificationDimension::BoundaryRoute).unwrap();
        assert_eq!(boundary.outcome, CertificationOutcome::Narrowed);
        assert_eq!(boundary.gap_kind, Some(AssuranceGapKind::ProofStale));
        // The profile can never be more permissive than Beta after narrowing.
        assert_eq!(claim.effective_qualification, QualificationClass::Beta);
        // The other three dimensions stay certified — narrowing is per dimension.
        for dim in [
            CertificationDimension::AssuranceCenter,
            CertificationDimension::Governance,
            CertificationDimension::EventProvenance,
        ] {
            assert_eq!(
                claim.cell(dim).unwrap().outcome,
                CertificationOutcome::Certified,
                "profile `{}` dimension `{}` should stay certified",
                claim.claim_ref,
                dim.as_str()
            );
        }
    }
    assert_eq!(packet.summary.narrowed_profiles, EXPECTED_PROFILES as u32);
    assert_eq!(packet.summary.certified_profiles, 0);
    assert_eq!(packet.summary.blocked_profiles, 0);
    assert!(!packet.blocks_stable_promotion());
    assert_eq!(
        packet.release_gate.drifted_dimensions,
        vec![CertificationDimension::BoundaryRoute.as_str().to_owned()]
    );
}

#[test]
fn missing_event_proof_blocks_every_profile() {
    // Acceptance criterion: missing proof blocks Stable promotion deterministically.
    let packet = seeded_m5_assurance_certification_missing_proof_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    for claim in &packet.profiles {
        assert!(
            claim.is_blocked(),
            "profile `{}` not blocked",
            claim.claim_ref
        );
        assert_eq!(
            claim.effective_qualification,
            QualificationClass::Unavailable
        );
        let cell = claim.cell(CertificationDimension::EventProvenance).unwrap();
        assert_eq!(cell.outcome, CertificationOutcome::Blocked);
        assert_eq!(cell.gap_kind, Some(AssuranceGapKind::ProofMissing));
    }
    assert_eq!(packet.summary.blocked_profiles, EXPECTED_PROFILES as u32);
    assert!(packet.blocks_stable_promotion());

    // Every consumer that surfaces event provenance blocks; About / help does not read it and stays
    // certified — proving the block is scoped to the surfaces that depend on the missing proof.
    for c in &packet.consumers {
        if c.read_dimensions
            .contains(&CertificationDimension::EventProvenance)
        {
            assert!(
                c.is_blocked(),
                "consumer `{}` did not block",
                c.consumer.as_str()
            );
        } else {
            assert!(
                c.is_certified(),
                "consumer `{}` blocked but does not surface event provenance",
                c.consumer.as_str()
            );
        }
    }
    assert_eq!(
        packet.summary.certified_consumers, 1,
        "only About / help stays certified"
    );
    assert_eq!(packet.summary.blocked_consumers, 4);
    assert!(packet
        .consumer(CertificationConsumer::HelpAbout)
        .unwrap()
        .is_certified());
}

#[test]
fn consumers_bind_their_declared_dimensions() {
    let packet = packet();
    for c in &packet.consumers {
        assert_eq!(c.owner_role, c.consumer.owner_role());
        assert_eq!(c.read_dimensions, c.consumer.read_dimensions());
        assert!(!c.read_dimensions.is_empty());
    }
}

#[test]
fn channels_produce_identical_output() {
    let packet = packet();
    let desktop = packet.render_for_channel(CertificationChannel::DesktopUi);
    let cli = packet.render_for_channel(CertificationChannel::CliHeadless);
    let offline = packet.render_for_channel(CertificationChannel::OfflineMirror);
    assert_eq!(desktop, cli);
    assert_eq!(cli, offline);
    assert_eq!(desktop, packet.export_safe_json());
}

#[test]
fn controlled_vocabulary_is_frozen() {
    let vocab = CertificationVocabulary::canonical();
    assert_eq!(vocab.profiles.len(), ClaimedPosture::ALL.len());
    assert_eq!(vocab.dimensions.len(), CertificationDimension::ALL.len());
    assert_eq!(vocab.consumers.len(), CertificationConsumer::ALL.len());
    assert_eq!(vocab.facets.len(), AssuranceFacet::ALL.len());
    for needle in ["managed", "self_hosted", "regulated", "sovereign"] {
        assert!(vocab.profiles.contains(&needle.to_owned()));
    }
    for needle in [
        "assurance_center",
        "governance",
        "boundary_route",
        "event_provenance",
    ] {
        assert!(vocab.dimensions.contains(&needle.to_owned()));
    }
    for needle in ["proof_stale", "proof_missing", "assurance_state_blocked"] {
        assert!(vocab.gap_kinds.contains(&needle.to_owned()));
    }
}

#[test]
fn packet_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: M5AssuranceCertification =
        serde_json::from_str(&json).expect("packet deserializes");
    assert_eq!(parsed, packet);
    assert!(parsed.validate().is_empty());
}

#[test]
fn one_certification_across_all_surfaces() {
    // Acceptance criterion: release, help, support, shiproom, and evaluation consume one
    // certification.
    let packet = packet();
    let expected = tokens(&CertificationConsumer::ALL, |c| c.as_str());
    assert_eq!(packet.consumer_tokens, expected);
    assert!(packet.disclosure.one_certification_across_surfaces);
    assert!(packet.conformance.surfaces_consume_one_certification);
    // Provenance: the certification names the governance matrix it was projected from.
    assert!(!packet.governance_packet_id.is_empty());
    assert_eq!(packet.governance_ref, M5_ASSURANCE_ROUTE_REF);
    assert!(packet.conformance.generated_from_governance_matrix);
}

#[test]
fn grid_csv_enumerates_profile_and_dimension() {
    let csv = packet().render_grid_csv();
    let header = csv.lines().next().unwrap();
    assert!(header.starts_with("profile,"));
    assert!(header.contains("dimension"));
    assert!(header.contains("outcome"));
    assert!(header.contains("gap_kind"));
    // One row per (profile, dimension) cell, plus the header.
    let expected_rows = EXPECTED_PROFILES * CertificationDimension::ALL.len();
    assert_eq!(csv.lines().count(), expected_rows + 1);
    for profile in ClaimedPosture::ALL {
        assert!(csv.contains(&format!("{},stable,", profile.as_str())));
    }
}

#[test]
fn certification_markdown_names_grid_and_consumers() {
    let md =
        seeded_m5_assurance_certification_stale_proof_narrowed().render_certification_markdown();
    assert!(md.contains("# M5 assurance / governance / route-provenance certification"));
    assert!(md.contains("Profile qualification grid"));
    assert!(md.contains("Narrowed / blocked profiles"));
    assert!(md.contains("Consumers"));
    assert!(md.contains("regulated"));
    assert!(md.contains("boundary_route"));
}

#[test]
fn tampered_profile_verdict_is_rejected() {
    let mut packet = seeded_m5_assurance_certification_stale_proof_narrowed();
    let idx = packet
        .profiles
        .iter()
        .position(|c| c.is_narrowed())
        .expect("a narrowed profile exists");
    packet.profiles[idx].gate = DescriptorGate::Governed;
    packet.profiles[idx].effective_qualification = QualificationClass::Stable;
    let violations = packet.validate();
    assert!(
        violations.contains(&M5AssuranceCertificationViolation::ClaimVerdictDrift)
            || violations.contains(&M5AssuranceCertificationViolation::SummaryDrift)
            || violations.contains(&M5AssuranceCertificationViolation::ReleaseGateDrift),
        "{violations:?}"
    );
}

#[test]
fn tampered_consumer_verdict_is_rejected() {
    let mut packet = seeded_m5_assurance_certification_missing_proof_blocked();
    let idx = packet
        .consumers
        .iter()
        .position(|c| c.is_blocked())
        .expect("a blocked consumer exists");
    packet.consumers[idx].gate = DescriptorGate::Governed;
    packet.consumers[idx].status = ConsumerStatus::Mapped;
    let violations = packet.validate();
    assert!(violations.contains(&M5AssuranceCertificationViolation::ConsumerVerdictDrift));
}

#[test]
fn tampered_cell_outcome_is_rejected() {
    let mut packet = packet();
    // Force a certified cell to claim it is blocked without changing its proof.
    let claim = &mut packet.profiles[0];
    let cell = claim
        .cells
        .iter_mut()
        .find(|c| c.is_applicable())
        .expect("an applicable cell exists");
    cell.outcome = CertificationOutcome::Blocked;
    let violations = packet.validate();
    assert!(violations.contains(&M5AssuranceCertificationViolation::CellOutcomeDrift));
}

#[test]
fn dropping_a_profile_is_rejected() {
    let mut packet = packet();
    packet.profiles.truncate(EXPECTED_PROFILES - 1);
    let violations = packet.validate();
    assert!(
        violations.contains(&M5AssuranceCertificationViolation::SummaryDrift)
            || violations.contains(&M5AssuranceCertificationViolation::ConsumerVerdictDrift),
        "{violations:?}"
    );
}

#[test]
fn export_carries_no_raw_material() {
    for packet in [
        seeded_m5_assurance_certification(),
        seeded_m5_assurance_certification_stale_proof_narrowed(),
        seeded_m5_assurance_certification_missing_proof_blocked(),
    ] {
        assert!(packet.conformance.export_carries_no_raw_material);
        assert!(packet.conformance.export_preserves_route_evidence_lineage);
        assert!(!packet
            .export_safe_json()
            .to_ascii_lowercase()
            .contains("bearer_token"));
    }
}
