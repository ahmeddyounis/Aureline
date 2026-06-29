//! Inline tests for the M5 descriptor-certification lane.

use super::*;

fn packet() -> M5DescriptorCertification {
    seeded_m5_descriptor_certification()
}

#[test]
fn canonical_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_DESCRIPTOR_CERTIFICATION_PACKET_ID);
    assert_eq!(packet.record_kind, M5_DESCRIPTOR_CERTIFICATION_RECORD_KIND);
    assert_eq!(packet.lanes.len(), RuntimeLane::ALL.len());
    assert_eq!(packet.consumers.len(), PublicTruthConsumer::ALL.len());
    assert!(packet.conformance.all_hold());
    assert!(packet.vocabulary.matches_canonical());
}

#[test]
fn every_runtime_lane_is_certified_exactly_once() {
    let packet = packet();
    for lane in RuntimeLane::ALL {
        let matches: Vec<&CertifiedLane> = packet.lanes.iter().filter(|c| c.lane == lane).collect();
        assert_eq!(
            matches.len(),
            1,
            "lane `{}` not certified once",
            lane.as_str()
        );
        let certified = matches[0];
        assert_eq!(certified.schema_ref, lane.schema_ref());
        assert_eq!(certified.parity_proof_ref, lane.parity_proof_ref());
        assert_eq!(certified.dimension, lane.dimension());
    }
}

#[test]
fn every_dimension_is_covered() {
    let packet = packet();
    for dimension in CertificationDimension::ALL {
        assert!(
            packet.lanes.iter().any(|l| l.dimension == dimension),
            "dimension `{}` not covered",
            dimension.as_str()
        );
    }
}

#[test]
fn canonical_packet_certifies_every_consumer() {
    // Acceptance criterion: every claimed M5 consumer maps to current shared descriptors and proof.
    let packet = packet();
    for consumer in &packet.consumers {
        assert!(
            consumer.is_certified(),
            "consumer `{}` not certified when every lane is current",
            consumer.consumer.as_str()
        );
        assert_eq!(
            consumer.effective_qualification,
            consumer.claimed_qualification
        );
        assert!(consumer.gaps.is_empty());
    }
    assert_eq!(
        packet.summary.certified_consumer_count,
        PublicTruthConsumer::ALL.len() as u32
    );
    assert_eq!(packet.summary.narrowed_consumer_count, 0);
    assert_eq!(packet.summary.blocked_consumer_count, 0);
    assert!(!packet.blocks_stable_promotion());
}

#[test]
fn every_consumer_maps_to_schemas_badges_rules_and_fixtures() {
    // Acceptance criterion: each consumer is mapped to descriptor schemas, badge families,
    // downgrade rules, and proof fixtures.
    let packet = packet();
    for c in &packet.consumers {
        assert_eq!(c.bound_descriptor_schemas.len(), c.bound_families.len());
        assert_eq!(c.covered_badge_families.len(), c.bound_families.len());
        assert_eq!(c.proof_fixture_refs.len(), c.certified_lanes.len());
        assert!(!c.certified_lanes.is_empty());
        // Schemas resolve to the bound families' schemas.
        for (family, schema) in c
            .bound_families
            .iter()
            .zip(c.bound_descriptor_schemas.iter())
        {
            assert_eq!(schema, family.schema_ref());
        }
        // Every consumer that binds a family carries the matching downgrade rules.
        assert!(!c.applicable_downgrade_rule_ids.is_empty());
        // Proof fixtures resolve to the read lanes' parity proofs.
        for (lane, fixture) in c.certified_lanes.iter().zip(c.proof_fixture_refs.iter()) {
            assert_eq!(fixture, lane.parity_proof_ref());
        }
    }
}

#[test]
fn stale_lane_proof_narrows_exactly_the_consumers_that_read_it() {
    // Acceptance criterion: descriptor / badge-runtime drift narrows claims deterministically.
    let packet = seeded_m5_descriptor_certification_stale_proof_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let stale_lane = RuntimeLane::DescriptorJoin;
    assert_eq!(
        packet.lane(stale_lane).unwrap().proof_freshness,
        FreshnessState::Stale
    );
    for c in &packet.consumers {
        if c.certified_lanes.contains(&stale_lane) {
            assert!(
                c.is_narrowed(),
                "consumer `{}` reads the stale lane but did not narrow",
                c.consumer.as_str()
            );
            assert!(c
                .gaps
                .iter()
                .any(|g| g.lane == stale_lane && g.gap_kind == DescriptorGapKind::ProofStale));
            assert_eq!(c.effective_qualification, QualificationClass::Beta);
        } else {
            assert!(
                c.is_certified(),
                "consumer `{}` does not read the stale lane but narrowed",
                c.consumer.as_str()
            );
        }
    }
    assert!(!packet.blocks_stable_promotion());
    assert_eq!(packet.summary.narrowed_consumer_count, 5);
    assert_eq!(packet.summary.certified_consumer_count, 3);
    assert!(packet
        .release_gate
        .drifted_dimensions
        .contains(&CertificationDimension::BadgeRuntime.as_str().to_owned()));
}

#[test]
fn missing_lane_proof_blocks_exactly_the_consumers_that_read_it() {
    // Acceptance criterion: a missing/failing proof blocks Stable promotion deterministically.
    let packet = seeded_m5_descriptor_certification_missing_proof_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let failing_lane = RuntimeLane::ClientScopeCard;
    assert_eq!(
        packet.lane(failing_lane).unwrap().proof_freshness,
        FreshnessState::Missing
    );
    for c in &packet.consumers {
        if c.certified_lanes.contains(&failing_lane) {
            assert!(
                c.is_blocked(),
                "consumer `{}` reads the failing lane but was not blocked",
                c.consumer.as_str()
            );
            assert_eq!(c.effective_qualification, QualificationClass::Unavailable);
            assert!(c
                .gaps
                .iter()
                .any(|g| g.lane == failing_lane && g.gap_kind == DescriptorGapKind::ProofMissing));
        } else {
            assert!(
                c.is_certified(),
                "consumer `{}` does not read the failing lane but was blocked/narrowed",
                c.consumer.as_str()
            );
        }
    }
    assert!(packet.blocks_stable_promotion());
    assert_eq!(packet.summary.blocked_consumer_count, 6);
    assert_eq!(packet.summary.certified_consumer_count, 2);
    assert_eq!(packet.release_gate.blocked_consumers.len(), 6);
}

#[test]
fn certification_gap_names_its_drifted_dimension() {
    let packet = seeded_m5_descriptor_certification_stale_proof_narrowed();
    let release = packet.consumer(PublicTruthConsumer::ReleaseCenter).unwrap();
    let gap = release
        .gaps
        .iter()
        .find(|g| g.lane == RuntimeLane::DescriptorJoin)
        .expect("release center reads the stale descriptor-join lane");
    assert_eq!(gap.dimension, CertificationDimension::BadgeRuntime);
    assert_eq!(gap.gap_kind, DescriptorGapKind::ProofStale);
    assert!(gap
        .cause_message_id
        .starts_with(M5_DESCRIPTOR_BADGE_MESSAGE_ID_PREFIX));
}

#[test]
fn downgrade_rules_are_the_shared_canonical_rules() {
    // The certification reuses the matrix's downgrade vocabulary rather than inventing its own.
    let packet = packet();
    assert_eq!(packet.downgrade_rules, canonical_downgrade_rules());
    assert!(packet.conformance.downgrade_rules_cover_every_weaker_value);
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
    assert_eq!(vocab.runtime_lanes.len(), RuntimeLane::ALL.len());
    assert_eq!(vocab.dimensions.len(), CertificationDimension::ALL.len());
    for needle in [
        "descriptor_object",
        "descriptor_badge_matrix",
        "badge_vocabulary",
        "claim_narrowing",
        "descriptor_join",
        "omission_guard",
        "client_scope_card",
    ] {
        assert!(vocab.runtime_lanes.contains(&needle.to_owned()));
    }
    for needle in [
        "descriptor_parity",
        "badge_runtime",
        "freshness_integration",
    ] {
        assert!(vocab.dimensions.contains(&needle.to_owned()));
    }
}

#[test]
fn packet_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: M5DescriptorCertification =
        serde_json::from_str(&json).expect("packet deserializes");
    assert_eq!(parsed, packet);
    assert!(parsed.validate().is_empty());
}

#[test]
fn one_certification_output_across_all_surfaces() {
    // Acceptance criterion: release, support, docs, and evaluation consume one certification output.
    let packet = packet();
    let expected: Vec<String> = PublicTruthConsumer::ALL
        .iter()
        .map(|c| c.as_str().to_owned())
        .collect();
    assert_eq!(packet.consumer_tokens, expected);
    assert!(packet.disclosure.all_consume());
    assert!(packet.conformance.surfaces_consume_one_certification);
}

#[test]
fn tampered_consumer_verdict_is_rejected() {
    let mut packet = seeded_m5_descriptor_certification_stale_proof_narrowed();
    // Force a narrowed consumer to read as certified at Stable — the recompute must reject it.
    let idx = packet
        .consumers
        .iter()
        .position(|c| c.is_narrowed())
        .expect("a narrowed consumer exists");
    packet.consumers[idx].gate_decision = DescriptorGate::Governed;
    packet.consumers[idx].effective_qualification = QualificationClass::Stable;
    let violations = packet.validate();
    assert!(violations.contains(&M5DescriptorCertificationViolation::ConsumerVerdictDrift));
}

#[test]
fn tampered_lane_freshness_is_rejected() {
    let mut packet = packet();
    // A lane whose proof freshness is changed without recomputing the dependent consumers must be
    // caught by the summary / release-gate / consumer-verdict drift checks.
    packet.lanes[0].proof_freshness = FreshnessState::Stale;
    let violations = packet.validate();
    assert!(
        violations.contains(&M5DescriptorCertificationViolation::LaneStatusDrift)
            || violations.contains(&M5DescriptorCertificationViolation::SummaryDrift)
            || violations.contains(&M5DescriptorCertificationViolation::ConsumerVerdictDrift),
        "{violations:?}"
    );
}

#[test]
fn dropping_a_lane_is_rejected() {
    let mut packet = packet();
    packet
        .lanes
        .retain(|l| l.lane != RuntimeLane::OmissionGuard);
    let violations = packet.validate();
    assert!(violations.contains(&M5DescriptorCertificationViolation::LaneNotCertified));
}

#[test]
fn markdown_report_names_lanes_consumers_and_gaps() {
    let md = seeded_m5_descriptor_certification_stale_proof_narrowed().render_markdown_summary();
    assert!(md.contains("# M5 Descriptor / Badge Certification"));
    assert!(md.contains("Certified runtime lanes"));
    assert!(md.contains("Certified consumers"));
    assert!(md.contains("descriptor_join"));
    assert!(md.contains("gap:"));
    assert!(md.contains("Drifted dimensions"));
}

#[test]
fn summary_counts_match_canonical() {
    let packet = packet();
    let s = &packet.summary;
    assert_eq!(s.total_lanes, RuntimeLane::ALL.len() as u32);
    assert_eq!(s.current_lanes, RuntimeLane::ALL.len() as u32);
    assert_eq!(s.stale_lanes, 0);
    assert_eq!(s.missing_lanes, 0);
    assert_eq!(s.total_consumers, PublicTruthConsumer::ALL.len() as u32);
    assert_eq!(
        s.total_downgrade_rules,
        canonical_downgrade_rules().len() as u32
    );
    assert!(!s.blocks_stable_promotion);
}

#[test]
fn export_carries_no_raw_material() {
    for packet in [
        seeded_m5_descriptor_certification(),
        seeded_m5_descriptor_certification_stale_proof_narrowed(),
        seeded_m5_descriptor_certification_missing_proof_blocked(),
    ] {
        assert!(packet.conformance.export_carries_no_raw_material);
        assert!(!packet
            .export_safe_json()
            .to_ascii_lowercase()
            .contains("bearer_token"));
    }
}
