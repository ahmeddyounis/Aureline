//! Inline tests for the M5 assurance / governance / route-provenance governance lane.

use super::*;

fn packet() -> M5AssuranceRouteGovernance {
    seeded_m5_assurance_route_governance()
}

#[test]
fn canonical_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_ASSURANCE_ROUTE_PACKET_ID);
    assert_eq!(packet.record_kind, M5_ASSURANCE_ROUTE_RECORD_KIND);
    assert_eq!(packet.facets.len(), AssuranceFacet::ALL.len());
    assert_eq!(packet.consumers.len(), AssuranceConsumer::ALL.len());
    assert_eq!(packet.state_families.len(), AssuranceStateFamily::ALL.len());
    assert!(packet.conformance.all_hold());
    assert!(packet.vocabulary.matches_canonical());
}

#[test]
fn every_facet_is_governed_exactly_once() {
    let packet = packet();
    for facet in AssuranceFacet::ALL {
        let matches: Vec<&AssuranceFacetRow> =
            packet.facets.iter().filter(|r| r.facet == facet).collect();
        assert_eq!(
            matches.len(),
            1,
            "facet `{}` not governed once",
            facet.as_str()
        );
        let row = matches[0];
        assert_eq!(row.proof_ref, facet.proof_ref());
        assert_eq!(row.dimension, facet.dimension());
        assert_eq!(row.state_family, facet.state_family());
        assert!(row.state_family.contains_token(&row.current_state_token));
    }
}

#[test]
fn every_dimension_and_state_family_is_covered() {
    let packet = packet();
    for dimension in AssuranceDimension::ALL {
        assert!(
            packet.facets.iter().any(|f| f.dimension == dimension),
            "dimension `{}` not covered",
            dimension.as_str()
        );
    }
    for family in AssuranceStateFamily::ALL {
        assert!(
            packet.facets.iter().any(|f| f.state_family == family),
            "state family `{}` not referenced",
            family.as_str()
        );
    }
}

#[test]
fn canonical_packet_certifies_every_consumer() {
    // Acceptance criterion: every claimed consumer maps to current proofs and governed states.
    let packet = packet();
    for consumer in &packet.consumers {
        assert!(
            consumer.is_certified(),
            "consumer `{}` not certified when every facet is current",
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
        AssuranceConsumer::ALL.len() as u32
    );
    assert_eq!(packet.summary.narrowed_consumer_count, 0);
    assert_eq!(packet.summary.blocked_consumer_count, 0);
    assert!(!packet.blocks_stable_promotion());
}

#[test]
fn every_consumer_maps_to_facets_owner_and_proof() {
    // Acceptance criterion: the matrix enumerates every consumer, current owner, and proof path.
    let packet = packet();
    for c in &packet.consumers {
        assert_eq!(c.owner_role, c.consumer.owner_role());
        assert!(!c.read_facets.is_empty());
        assert_eq!(c.proof_refs.len(), c.read_facets.len());
        assert!(!c.disclosed_evidence_classes.is_empty());
        assert!(!c.claimed_postures.is_empty());
        assert!(!c.trust_boundaries.is_empty());
        // Proof refs resolve to the read facets' proof paths.
        for (facet, proof) in c.read_facets.iter().zip(c.proof_refs.iter()) {
            assert_eq!(proof, facet.proof_ref());
        }
    }
}

#[test]
fn canonical_state_vocabulary_is_bound_to_gate() {
    // Acceptance criterion: assurance classes are canonical and bound to descriptor/badge gate rows.
    let packet = packet();
    assert!(packet.conformance.state_vocabulary_bound_to_gate);
    for family in &packet.state_families {
        assert!(!family.states.is_empty());
        for state in &family.states {
            assert_eq!(state.effective_floor, floor_for_posture(state.gate_posture));
            assert!(state
                .message_id
                .starts_with(M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX));
        }
    }
    // The governance dashboard distinguishes pass / stale / waived / blocked, and the blocking
    // states floor at Unavailable.
    let gov = packet
        .state_families
        .iter()
        .find(|f| f.family == AssuranceStateFamily::Governance)
        .unwrap();
    for token in ["pass", "stale", "waived", "blocked"] {
        assert!(
            gov.states.iter().any(|s| s.token == token),
            "governance family missing `{token}`"
        );
    }
    let blocked = gov.states.iter().find(|s| s.token == "blocked").unwrap();
    assert_eq!(blocked.gate_posture, DescriptorGate::Blocked);
    assert_eq!(blocked.effective_floor, QualificationClass::Unavailable);
    let waived = gov.states.iter().find(|s| s.token == "waived").unwrap();
    assert_eq!(waived.gate_posture, DescriptorGate::Narrowed);
}

#[test]
fn stale_facet_proof_narrows_exactly_the_consumers_that_read_it() {
    // Acceptance criterion: a stale proof narrows claims deterministically.
    let packet = seeded_m5_assurance_route_governance_stale_proof_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let stale = AssuranceFacet::RouteHop;
    assert_eq!(
        packet.facet(stale).unwrap().proof_freshness,
        FreshnessState::Stale
    );
    for c in &packet.consumers {
        if c.read_facets.contains(&stale) {
            assert!(
                c.is_narrowed(),
                "consumer `{}` reads the stale facet but did not narrow",
                c.consumer.as_str()
            );
            assert!(c
                .gaps
                .iter()
                .any(|g| g.facet == stale && g.gap_kind == AssuranceGapKind::ProofStale));
            assert_eq!(c.effective_qualification, QualificationClass::Beta);
        } else {
            assert!(
                c.is_certified(),
                "consumer `{}` does not read the stale facet but narrowed",
                c.consumer.as_str()
            );
        }
    }
    assert!(!packet.blocks_stable_promotion());
    assert_eq!(packet.summary.narrowed_consumer_count, 4);
    assert_eq!(packet.summary.certified_consumer_count, 4);
    assert!(packet
        .release_gate
        .drifted_dimensions
        .contains(&AssuranceDimension::RouteProvenance.as_str().to_owned()));
}

#[test]
fn missing_facet_proof_blocks_exactly_the_consumers_that_read_it() {
    // Acceptance criterion: a missing/failing proof blocks Stable promotion deterministically.
    let packet = seeded_m5_assurance_route_governance_missing_proof_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let failing = AssuranceFacet::EventProvenance;
    assert_eq!(
        packet.facet(failing).unwrap().proof_freshness,
        FreshnessState::Missing
    );
    for c in &packet.consumers {
        if c.read_facets.contains(&failing) {
            assert!(
                c.is_blocked(),
                "consumer `{}` reads the failing facet but was not blocked",
                c.consumer.as_str()
            );
            assert_eq!(c.effective_qualification, QualificationClass::Unavailable);
            assert!(c
                .gaps
                .iter()
                .any(|g| g.facet == failing && g.gap_kind == AssuranceGapKind::ProofMissing));
        } else {
            assert!(
                c.is_certified(),
                "consumer `{}` does not read the failing facet but was blocked",
                c.consumer.as_str()
            );
        }
    }
    assert!(packet.blocks_stable_promotion());
    assert_eq!(packet.summary.blocked_consumer_count, 3);
    assert_eq!(packet.summary.certified_consumer_count, 5);
    assert_eq!(packet.release_gate.blocked_consumers.len(), 3);
}

#[test]
fn assurance_state_gap_blocks_when_facet_state_blocks() {
    // Acceptance criterion: gaps in assurance coverage (not just proof) fail the matrix.
    let mut packet = packet();
    // Force the route-hop facet into an unattributed (blocking) assurance state.
    let unattributed = AssuranceFacetRow::new(
        AssuranceFacet::RouteHop,
        CanonicalState::RouteHop(RouteHopState::UnattributedRoute),
        FreshnessState::Current,
        &[EvidenceClass::RouteTimeline, EvidenceClass::ProvenanceLedger],
        &[
            ClaimedPosture::Managed,
            ClaimedPosture::SelfHosted,
            ClaimedPosture::Regulated,
            ClaimedPosture::Sovereign,
        ],
        &[TrustBoundary::LocalFirst, TrustBoundary::ControlPlane],
        DegradedDataBehavior::LocalLineageOnly,
    );
    let idx = packet
        .facets
        .iter()
        .position(|f| f.facet == AssuranceFacet::RouteHop)
        .unwrap();
    packet.facets[idx] = unattributed;
    for consumer in &mut packet.consumers {
        consumer.recompute(&packet.facets);
    }
    let route = packet.consumer(AssuranceConsumer::RouteInspector).unwrap();
    assert!(route.is_blocked());
    assert!(route
        .gaps
        .iter()
        .any(|g| g.facet == AssuranceFacet::RouteHop
            && g.gap_kind == AssuranceGapKind::AssuranceStateBlocked));
}

#[test]
fn coverage_gap_names_its_drifted_dimension() {
    let packet = seeded_m5_assurance_route_governance_stale_proof_narrowed();
    let route = packet.consumer(AssuranceConsumer::RouteInspector).unwrap();
    let gap = route
        .gaps
        .iter()
        .find(|g| g.facet == AssuranceFacet::RouteHop)
        .expect("route inspector reads the stale route-hop facet");
    assert_eq!(gap.dimension, AssuranceDimension::RouteProvenance);
    assert_eq!(gap.gap_kind, AssuranceGapKind::ProofStale);
    assert!(gap
        .cause_message_id
        .starts_with(M5_ASSURANCE_ROUTE_MESSAGE_ID_PREFIX));
}

#[test]
fn channels_produce_identical_output() {
    let packet = packet();
    let desktop = packet.render_for_channel(AssuranceChannel::DesktopUi);
    let cli = packet.render_for_channel(AssuranceChannel::CliHeadless);
    let offline = packet.render_for_channel(AssuranceChannel::OfflineMirror);
    assert_eq!(desktop, cli);
    assert_eq!(cli, offline);
    assert_eq!(desktop, packet.export_safe_json());
}

#[test]
fn controlled_vocabulary_is_frozen() {
    let vocab = AssuranceVocabulary::canonical();
    assert_eq!(vocab.facets.len(), AssuranceFacet::ALL.len());
    assert_eq!(vocab.state_families.len(), AssuranceStateFamily::ALL.len());
    assert_eq!(vocab.consumers.len(), AssuranceConsumer::ALL.len());
    for needle in [
        "assurance_claim",
        "control_proof",
        "exception_waiver",
        "governance_freshness",
        "service_ownership",
        "capability_boundary",
        "route_hop",
        "approval_ticket",
        "event_provenance",
    ] {
        assert!(vocab.facets.contains(&needle.to_owned()));
    }
    for needle in [
        "assurance_claim",
        "governance",
        "capability_boundary",
        "route_hop",
        "approval",
        "provenance",
    ] {
        assert!(vocab.state_families.contains(&needle.to_owned()));
    }
    // Postures match the regulated / sovereign / self-hosted / managed lines.
    for needle in ["managed", "self_hosted", "regulated", "sovereign"] {
        assert!(vocab.claimed_postures.contains(&needle.to_owned()));
    }
}

#[test]
fn packet_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: M5AssuranceRouteGovernance =
        serde_json::from_str(&json).expect("packet deserializes");
    assert_eq!(parsed, packet);
    assert!(parsed.validate().is_empty());
}

#[test]
fn one_matrix_across_all_surfaces() {
    // Acceptance criterion: assurance, governance, route, admin, help, procurement, and support
    // consume one matrix.
    let packet = packet();
    let expected = tokens(&AssuranceConsumer::ALL, |c| c.as_str());
    assert_eq!(packet.consumer_tokens, expected);
    assert!(packet.disclosure.all_consume());
    assert!(packet.conformance.surfaces_consume_one_matrix);
}

#[test]
fn matrix_csv_enumerates_consumer_owner_and_proof() {
    let csv = packet().render_matrix_csv();
    let header = csv.lines().next().unwrap();
    assert!(header.starts_with("consumer,consumer_owner,"));
    assert!(header.contains("proof_ref"));
    assert!(header.contains("gap_kind"));
    // Every consumer appears with its owner and a proof path.
    for c in AssuranceConsumer::ALL {
        assert!(csv.contains(&format!("{},{}", c.as_str(), c.owner_role())));
    }
    assert!(csv.contains("artifacts/release-proof/m5-assurance-route-governance/"));
}

#[test]
fn governance_markdown_names_states_facets_and_consumers() {
    let md = seeded_m5_assurance_route_governance_stale_proof_narrowed().render_governance_markdown();
    assert!(md.contains("# M5 Assurance / Governance / Route-Provenance Governance Matrix"));
    assert!(md.contains("Canonical assurance state families"));
    assert!(md.contains("Governed facets"));
    assert!(md.contains("Claimed consumers"));
    assert!(md.contains("route_hop"));
    assert!(md.contains("gap:"));
}

#[test]
fn tampered_consumer_verdict_is_rejected() {
    let mut packet = seeded_m5_assurance_route_governance_stale_proof_narrowed();
    let idx = packet
        .consumers
        .iter()
        .position(|c| c.is_narrowed())
        .expect("a narrowed consumer exists");
    packet.consumers[idx].gate_decision = DescriptorGate::Governed;
    packet.consumers[idx].effective_qualification = QualificationClass::Stable;
    let violations = packet.validate();
    assert!(violations.contains(&M5AssuranceRouteViolation::ConsumerVerdictDrift));
}

#[test]
fn tampered_facet_freshness_is_rejected() {
    let mut packet = packet();
    packet.facets[0].proof_freshness = FreshnessState::Stale;
    let violations = packet.validate();
    assert!(
        violations.contains(&M5AssuranceRouteViolation::FacetStatusDrift)
            || violations.contains(&M5AssuranceRouteViolation::SummaryDrift)
            || violations.contains(&M5AssuranceRouteViolation::ConsumerVerdictDrift),
        "{violations:?}"
    );
}

#[test]
fn dropping_a_facet_is_rejected() {
    let mut packet = packet();
    packet
        .facets
        .retain(|f| f.facet != AssuranceFacet::ApprovalTicket);
    let violations = packet.validate();
    assert!(violations.contains(&M5AssuranceRouteViolation::FacetNotGoverned));
}

#[test]
fn summary_counts_match_canonical() {
    let packet = packet();
    let s = &packet.summary;
    assert_eq!(s.total_facets, AssuranceFacet::ALL.len() as u32);
    assert_eq!(s.current_facets, AssuranceFacet::ALL.len() as u32);
    assert_eq!(s.stale_facets, 0);
    assert_eq!(s.missing_facets, 0);
    assert_eq!(
        s.total_state_families,
        AssuranceStateFamily::ALL.len() as u32
    );
    assert_eq!(s.total_consumers, AssuranceConsumer::ALL.len() as u32);
    assert!(!s.blocks_stable_promotion);
}

#[test]
fn export_carries_no_raw_material() {
    for packet in [
        seeded_m5_assurance_route_governance(),
        seeded_m5_assurance_route_governance_stale_proof_narrowed(),
        seeded_m5_assurance_route_governance_missing_proof_blocked(),
    ] {
        assert!(packet.conformance.export_carries_no_raw_material);
        assert!(!packet
            .export_safe_json()
            .to_ascii_lowercase()
            .contains("bearer_token"));
    }
}
