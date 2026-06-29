//! Inline tests for the M5 update / support-lifecycle certification lane.

use super::*;

fn packet() -> M5UpdateLifecycleCertification {
    seeded_m5_update_lifecycle_certification()
}

/// Expected number of claimed channel × profile pairs (5 channels × 2 profiles).
const EXPECTED_CLAIMS: usize = ChannelScope::ALL.len() * DeploymentProfile::ALL.len();

#[test]
fn canonical_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_UPDATE_LIFECYCLE_CERTIFICATION_PACKET_ID
    );
    assert_eq!(
        packet.record_kind,
        M5_UPDATE_LIFECYCLE_CERTIFICATION_RECORD_KIND
    );
    assert_eq!(packet.claims.len(), EXPECTED_CLAIMS);
    assert_eq!(packet.consumers.len(), CertificationConsumer::ALL.len());
    assert!(packet.conformance.all_hold());
    assert!(packet.vocabulary.matches_canonical());
}

#[test]
fn grid_covers_every_channel_and_profile() {
    // Acceptance criterion: every claimed M5 channel / profile is mapped to proof.
    let packet = packet();
    for channel in ChannelScope::ALL {
        for profile in DeploymentProfile::ALL {
            let claim = packet
                .claim(channel, profile)
                .unwrap_or_else(|| panic!("{}/{} claimed", channel.as_str(), profile.as_str()));
            assert_eq!(claim.cells.len(), CertificationDimension::ALL.len());
            // Every applicable cell carries at least one proof ref.
            for cell in &claim.cells {
                if cell.is_applicable() {
                    assert!(
                        !cell.proof_refs.is_empty(),
                        "{}/{} {} applicable but has no proof",
                        channel.as_str(),
                        profile.as_str(),
                        cell.dimension.as_str()
                    );
                }
            }
        }
    }
}

#[test]
fn canonical_certifies_every_claim_and_consumer() {
    // Acceptance criterion: with fresh proof, every claim stands at its claimed qualification.
    let packet = packet();
    for claim in &packet.claims {
        assert!(
            claim.is_certified(),
            "claim `{}` not certified when every facet is current",
            claim.claim_ref
        );
        assert_eq!(claim.effective_qualification, claim.claimed_qualification);
        assert!(claim.cells.iter().all(|c| c.gap_kind.is_none()));
    }
    for c in &packet.consumers {
        assert!(
            c.is_certified(),
            "consumer `{}` not certified",
            c.consumer.as_str()
        );
        assert!(c.narrowed_claim_refs.is_empty());
        assert!(c.blocked_claim_refs.is_empty());
    }
    assert_eq!(packet.summary.certified_claims, EXPECTED_CLAIMS as u32);
    assert_eq!(packet.summary.narrowed_claims, 0);
    assert_eq!(packet.summary.blocked_claims, 0);
    assert_eq!(
        packet.summary.certified_consumers,
        CertificationConsumer::ALL.len() as u32
    );
    assert!(!packet.blocks_stable_promotion());
}

#[test]
fn dimensions_not_applicable_are_labeled_not_gaps() {
    // Honest labeling: preview and nightly have no lifecycle / migration facet coverage, so those
    // dimensions are explicitly not-applicable rather than hidden gaps.
    let packet = packet();
    let preview = packet
        .claim(ChannelScope::Preview, DeploymentProfile::Managed)
        .unwrap();
    let migration = preview
        .cell(CertificationDimension::MigrationGuidance)
        .unwrap();
    assert_eq!(migration.outcome, CertificationOutcome::NotApplicable);
    assert!(migration.gap_kind.is_none());
    assert!(!migration.is_applicable());
    let windows = preview
        .cell(CertificationDimension::LifecycleWindows)
        .unwrap();
    assert_eq!(windows.outcome, CertificationOutcome::NotApplicable);
    // Update communication and stale-data behavior DO apply to preview.
    assert!(preview
        .cell(CertificationDimension::UpdateCommunication)
        .unwrap()
        .is_applicable());
    assert!(preview
        .cell(CertificationDimension::StaleDataBehavior)
        .unwrap()
        .is_applicable());
    // A not-applicable dimension is not counted among applicable dimensions.
    assert!(!preview
        .applicable_dimensions
        .contains(&CertificationDimension::MigrationGuidance));
}

#[test]
fn stale_change_impact_proof_narrows_claims_per_channel() {
    // Acceptance criterion: stale proof narrows claims deterministically, not behind a generic
    // stable label — and only the channels the stale facet scopes to.
    let packet = seeded_m5_update_lifecycle_certification_stale_proof_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    for claim in &packet.claims {
        match claim.channel {
            // Change-impact backs update communication and scopes to these channels.
            ChannelScope::Stable
            | ChannelScope::Beta
            | ChannelScope::Preview
            | ChannelScope::Nightly => {
                assert!(
                    claim.is_narrowed(),
                    "claim `{}` did not narrow under a stale change-impact proof",
                    claim.claim_ref
                );
                let cell = claim
                    .cell(CertificationDimension::UpdateCommunication)
                    .unwrap();
                assert_eq!(cell.outcome, CertificationOutcome::Narrowed);
                assert_eq!(cell.gap_kind, Some(CertificationGapKind::ProofStale));
                // The claim can never be more permissive than Beta after narrowing.
                assert!(claim.effective_qualification >= QualificationClass::Beta);
            }
            // LTS is outside the change-impact scope and stays certified.
            ChannelScope::Lts => {
                assert!(
                    claim.is_certified(),
                    "LTS claim `{}` narrowed but the stale facet does not scope to it",
                    claim.claim_ref
                );
                assert_eq!(claim.effective_qualification, QualificationClass::Stable);
            }
        }
    }
    assert_eq!(packet.summary.narrowed_claims, 8);
    assert_eq!(packet.summary.certified_claims, 2);
    assert_eq!(packet.summary.blocked_claims, 0);
    assert!(!packet.blocks_stable_promotion());
    assert!(packet.release_gate.drifted_dimensions.contains(
        &CertificationDimension::UpdateCommunication
            .as_str()
            .to_owned()
    ));
}

#[test]
fn missing_service_health_proof_blocks_every_claim() {
    // Acceptance criterion: missing proof blocks Stable promotion deterministically.
    let packet = seeded_m5_update_lifecycle_certification_missing_proof_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    for claim in &packet.claims {
        assert!(
            claim.is_blocked(),
            "claim `{}` not blocked",
            claim.claim_ref
        );
        assert_eq!(
            claim.effective_qualification,
            QualificationClass::Unavailable
        );
        let cell = claim
            .cell(CertificationDimension::StaleDataBehavior)
            .unwrap();
        assert_eq!(cell.outcome, CertificationOutcome::Blocked);
        assert_eq!(cell.gap_kind, Some(CertificationGapKind::ProofMissing));
    }
    assert_eq!(packet.summary.blocked_claims, EXPECTED_CLAIMS as u32);
    assert!(packet.blocks_stable_promotion());

    // Every consumer that surfaces stale-data behavior blocks; docs/help does not read it and stays
    // certified — proving the block is scoped to the surfaces that depend on the missing proof.
    for c in &packet.consumers {
        if c.read_dimensions
            .contains(&CertificationDimension::StaleDataBehavior)
        {
            assert!(
                c.is_blocked(),
                "consumer `{}` did not block",
                c.consumer.as_str()
            );
        } else {
            assert!(
                c.is_certified(),
                "consumer `{}` blocked but does not surface stale-data behavior",
                c.consumer.as_str()
            );
        }
    }
    assert_eq!(
        packet.summary.certified_consumers, 1,
        "only docs/help stays certified"
    );
    assert_eq!(packet.summary.blocked_consumers, 5);
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
    assert_eq!(vocab.channels.len(), ChannelScope::ALL.len());
    assert_eq!(vocab.profiles.len(), DeploymentProfile::ALL.len());
    assert_eq!(vocab.dimensions.len(), CertificationDimension::ALL.len());
    assert_eq!(vocab.consumers.len(), CertificationConsumer::ALL.len());
    for needle in ["stable", "beta", "preview", "nightly", "lts"] {
        assert!(vocab.channels.contains(&needle.to_owned()));
    }
    for needle in ["managed", "self_hosted"] {
        assert!(vocab.profiles.contains(&needle.to_owned()));
    }
    for needle in [
        "update_communication",
        "migration_guidance",
        "lifecycle_windows",
        "stale_data_behavior",
    ] {
        assert!(vocab.dimensions.contains(&needle.to_owned()));
    }
}

#[test]
fn packet_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: M5UpdateLifecycleCertification =
        serde_json::from_str(&json).expect("packet deserializes");
    assert_eq!(parsed, packet);
    assert!(parsed.validate().is_empty());
}

#[test]
fn one_certification_across_all_surfaces() {
    // Acceptance criterion: release, help, support, and shiproom consume one certification.
    let packet = packet();
    let expected = tokens(&CertificationConsumer::ALL, |c| c.as_str());
    assert_eq!(packet.consumer_tokens, expected);
    assert!(packet.disclosure.one_certification_across_surfaces);
    assert!(packet.conformance.surfaces_consume_one_certification);
    // Provenance: the certification names the governance matrix it was projected from.
    assert!(!packet.governance_packet_id.is_empty());
    assert_eq!(
        packet.governance_ref,
        crate::m5_update_lifecycle::M5_UPDATE_LIFECYCLE_REF
    );
    assert!(packet.conformance.generated_from_governance_matrix);
}

#[test]
fn grid_csv_enumerates_channel_profile_and_dimension() {
    let csv = packet().render_grid_csv();
    let header = csv.lines().next().unwrap();
    assert!(header.starts_with("channel,profile,"));
    assert!(header.contains("dimension"));
    assert!(header.contains("outcome"));
    assert!(header.contains("gap_kind"));
    // One row per (claim, dimension) cell, plus the header.
    let expected_rows = EXPECTED_CLAIMS * CertificationDimension::ALL.len();
    assert_eq!(csv.lines().count(), expected_rows + 1);
    for channel in ChannelScope::ALL {
        assert!(csv.contains(&format!("{},managed,", channel.as_str())));
    }
}

#[test]
fn certification_markdown_names_grid_and_consumers() {
    let md = seeded_m5_update_lifecycle_certification_stale_proof_narrowed()
        .render_certification_markdown();
    assert!(md.contains("# M5 update / support-lifecycle certification"));
    assert!(md.contains("Channel / profile qualification grid"));
    assert!(md.contains("Narrowed / blocked claims"));
    assert!(md.contains("Consumers"));
    assert!(md.contains("stable:managed"));
    assert!(md.contains("update_communication"));
}

#[test]
fn tampered_claim_verdict_is_rejected() {
    let mut packet = seeded_m5_update_lifecycle_certification_stale_proof_narrowed();
    let idx = packet
        .claims
        .iter()
        .position(|c| c.is_narrowed())
        .expect("a narrowed claim exists");
    packet.claims[idx].gate = DescriptorGate::Governed;
    packet.claims[idx].effective_qualification = QualificationClass::Stable;
    let violations = packet.validate();
    assert!(
        violations.contains(&M5UpdateLifecycleCertificationViolation::ClaimVerdictDrift)
            || violations.contains(&M5UpdateLifecycleCertificationViolation::SummaryDrift)
            || violations.contains(&M5UpdateLifecycleCertificationViolation::ReleaseGateDrift),
        "{violations:?}"
    );
}

#[test]
fn tampered_consumer_verdict_is_rejected() {
    let mut packet = seeded_m5_update_lifecycle_certification_missing_proof_blocked();
    let idx = packet
        .consumers
        .iter()
        .position(|c| c.is_blocked())
        .expect("a blocked consumer exists");
    packet.consumers[idx].gate = DescriptorGate::Governed;
    packet.consumers[idx].status = ConsumerStatus::Mapped;
    let violations = packet.validate();
    assert!(violations.contains(&M5UpdateLifecycleCertificationViolation::ConsumerVerdictDrift));
}

#[test]
fn tampered_cell_outcome_is_rejected() {
    let mut packet = packet();
    // Force a certified cell to claim it is blocked without changing its proof.
    let claim = &mut packet.claims[0];
    let cell = claim
        .cells
        .iter_mut()
        .find(|c| c.is_applicable())
        .expect("an applicable cell exists");
    cell.outcome = CertificationOutcome::Blocked;
    let violations = packet.validate();
    assert!(violations.contains(&M5UpdateLifecycleCertificationViolation::CellOutcomeDrift));
}

#[test]
fn dropping_a_claim_is_rejected() {
    let mut packet = packet();
    packet.claims.truncate(EXPECTED_CLAIMS - 1);
    let violations = packet.validate();
    assert!(
        violations.contains(&M5UpdateLifecycleCertificationViolation::SummaryDrift)
            || violations.contains(&M5UpdateLifecycleCertificationViolation::ConsumerVerdictDrift),
        "{violations:?}"
    );
}

#[test]
fn export_carries_no_raw_material() {
    for packet in [
        seeded_m5_update_lifecycle_certification(),
        seeded_m5_update_lifecycle_certification_stale_proof_narrowed(),
        seeded_m5_update_lifecycle_certification_missing_proof_blocked(),
    ] {
        assert!(packet.conformance.export_carries_no_raw_material);
        assert!(!packet
            .export_safe_json()
            .to_ascii_lowercase()
            .contains("bearer_token"));
    }
}
