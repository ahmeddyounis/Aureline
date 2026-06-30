//! Inline tests for the M5 assurance consumer-parity lane.

use super::*;

fn packet() -> M5AssuranceConsumerParity {
    seeded_m5_assurance_consumer_parity()
}

#[test]
fn canonical_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_ASSURANCE_CONSUMER_PARITY_PACKET_ID);
    assert_eq!(packet.record_kind, M5_ASSURANCE_CONSUMER_PARITY_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        M5_ASSURANCE_CONSUMER_PARITY_SCHEMA_VERSION
    );
    assert!(packet.conformance.all_hold());
    assert!(packet.vocabulary.matches_canonical());
}

#[test]
fn canonical_packet_governs_every_fact() {
    let packet = packet();
    assert!(!packet.facts.is_empty());
    for fact in &packet.facts {
        assert!(fact.is_governed(), "fact `{}` not governed", fact.ref_key());
        assert_eq!(fact.effective_qualification, QualificationClass::Stable);
        assert_eq!(fact.status, ConsumerStatus::Mapped);
    }
    assert_eq!(packet.summary.governed_facts, packet.summary.total_facts);
    assert_eq!(packet.summary.narrowed_facts, 0);
    assert_eq!(packet.summary.blocked_facts, 0);
    assert!(!packet.blocks_stable_promotion());
}

#[test]
fn facts_cover_every_domain() {
    let packet = packet();
    for domain in TruthDomain::ALL {
        assert!(
            packet.facts.iter().any(|f| f.domain == domain),
            "no fact for domain `{}`",
            domain.as_str()
        );
    }
    assert_eq!(packet.summary.total_domains, TruthDomain::ALL.len() as u32);
}

#[test]
fn every_fact_projects_to_every_consumer() {
    let packet = packet();
    for fact in &packet.facts {
        let consumers: Vec<ParityConsumer> = fact
            .consumer_projections
            .iter()
            .map(|p| p.consumer)
            .collect();
        assert_eq!(consumers, ParityConsumer::ALL.to_vec());
        for projection in &fact.consumer_projections {
            assert_eq!(projection.gate, fact.gate);
            assert_eq!(
                projection.effective_qualification,
                fact.effective_qualification
            );
            assert!(projection.converges_with_fact);
        }
    }
    assert!(packet.conformance.consumers_converge_on_fact);
    assert!(packet.conformance.no_consumer_strengthens_a_fact);
}

#[test]
fn every_consumer_reads_every_fact() {
    let packet = packet();
    assert_eq!(packet.consumer_views.len(), ParityConsumer::ALL.len());
    for view in &packet.consumer_views {
        assert!(
            view.reads_all_facts,
            "consumer `{}`",
            view.consumer.as_str()
        );
        assert_eq!(view.fact_count as usize, packet.facts.len());
        assert_eq!(view.fact_refs.len(), packet.facts.len());
    }
    assert!(packet.conformance.every_consumer_reads_every_fact);
}

#[test]
fn all_five_sources_bind_and_validate_clean() {
    let packet = packet();
    let bound: Vec<SourcePacketKind> = packet
        .source_bindings
        .iter()
        .map(|b| b.source_packet)
        .collect();
    assert_eq!(bound, SourcePacketKind::ALL.to_vec());
    for binding in &packet.source_bindings {
        assert!(binding.validated_clean, "{}", binding.source_label);
        assert!(binding.fact_count > 0);
        assert_eq!(binding.registry_ref, binding.source_packet.registry_ref());
    }
    assert!(packet.conformance.all_sources_bound);
    assert!(packet.conformance.bound_sources_validated_clean);
    // Every fact's source-packet count is accounted for by exactly one binding.
    let total: u32 = packet.source_bindings.iter().map(|b| b.fact_count).sum();
    assert_eq!(total, packet.facts.len() as u32);
}

#[test]
fn claim_narrowed_drill_narrows_claim_facts_for_every_consumer() {
    let packet = seeded_m5_assurance_consumer_parity_claim_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let narrowed_claim_facts = packet
        .facts
        .iter()
        .filter(|f| f.domain == TruthDomain::AssuranceClaim && f.is_narrowed())
        .count();
    assert!(narrowed_claim_facts > 0, "no claim fact narrowed");
    // No fact reads the narrowed claim as Stable on any consumer surface.
    for fact in packet
        .facts
        .iter()
        .filter(|f| f.domain == TruthDomain::AssuranceClaim && f.is_narrowed())
    {
        for projection in &fact.consumer_projections {
            assert_eq!(projection.gate, DescriptorGate::Narrowed);
            assert_eq!(projection.effective_qualification, QualificationClass::Beta);
        }
    }
    // Narrowing does not block promotion.
    assert!(!packet.blocks_stable_promotion());
    assert!(packet.summary.narrowed_facts > 0);
}

#[test]
fn governance_blocked_drill_blocks_every_consumer_and_holds_promotion() {
    let packet = seeded_m5_assurance_consumer_parity_governance_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.blocks_stable_promotion());
    let blocked_governance = packet
        .facts
        .iter()
        .filter(|f| f.domain == TruthDomain::GovernanceFitness && f.is_blocked())
        .count();
    assert!(blocked_governance > 0, "no governance fact blocked");
    // Every consumer view reflects the block.
    for view in &packet.consumer_views {
        assert_eq!(view.worst_gate, DescriptorGate::Blocked);
        assert_eq!(
            view.effective_qualification,
            QualificationClass::Unavailable
        );
    }
    // The governance source binding records that it blocks.
    let binding = packet
        .source_bindings
        .iter()
        .find(|b| b.source_packet == SourcePacketKind::GovernanceDashboard)
        .expect("governance binding");
    assert!(binding.blocks_stable_promotion);
}

#[test]
fn boundary_route_blocked_drill_blocks_route_facts() {
    let packet = seeded_m5_assurance_consumer_parity_boundary_route_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.blocks_stable_promotion());
    let blocked_routes = packet
        .facts
        .iter()
        .filter(|f| f.domain == TruthDomain::RouteTimeline && f.is_blocked())
        .count();
    assert!(blocked_routes > 0, "no route fact blocked");
}

#[test]
fn event_blocked_drill_blocks_event_facts() {
    let packet = seeded_m5_assurance_consumer_parity_event_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.blocks_stable_promotion());
    let blocked_events = packet
        .facts
        .iter()
        .filter(|f| f.domain == TruthDomain::EventProvenance && f.is_blocked())
        .count();
    assert!(blocked_events > 0, "no event fact blocked");
}

#[test]
fn export_preview_mirrors_facts_and_carries_no_raw_material() {
    let packet = packet();
    assert_eq!(packet.export_preview.entries.len(), packet.facts.len());
    for (entry, fact) in packet
        .export_preview
        .entries
        .iter()
        .zip(packet.facts.iter())
    {
        assert_eq!(entry.domain, fact.domain);
        assert_eq!(entry.subject, fact.subject);
        assert_eq!(entry.gate, fact.gate);
        assert_eq!(entry.evidence_refs, fact.evidence_refs);
    }
    assert_eq!(
        packet.export_preview.record_kind,
        M5_ASSURANCE_CONSUMER_PARITY_EXPORT_RECORD_KIND
    );
    assert!(packet.conformance.export_mirrors_live_facts);
    assert!(packet.conformance.export_carries_no_raw_material);
    let value = serde_json::to_value(&packet).expect("serializes");
    assert!(!json_contains_forbidden_material(&value));
}

#[test]
fn facts_preserve_evidence_lineage() {
    let packet = packet();
    for fact in &packet.facts {
        assert!(!fact.evidence_refs.is_empty(), "fact `{}`", fact.ref_key());
        assert!(fact.evidence_refs.iter().all(|r| !r.trim().is_empty()));
        assert!(!fact.owner_role.trim().is_empty());
    }
    assert!(packet.conformance.facts_preserve_evidence_lineage);
}

#[test]
fn render_outputs_are_deterministic_and_nonempty() {
    let packet = packet();
    assert_eq!(packet.export_safe_json(), packet.export_safe_json());
    assert_eq!(packet.render_facts_csv(), packet.render_facts_csv());
    assert_eq!(
        packet.render_overview_markdown(),
        packet.render_overview_markdown()
    );
    assert!(packet.render_facts_csv().starts_with("domain,subject"));
    assert!(packet
        .render_overview_markdown()
        .contains("# M5 Assurance Consumer-Parity"));
    assert!(packet.render_markdown_summary().contains("Export Proof"));
    // Channel rendering is byte-identical across channels.
    for channel in AssuranceConsumerParityChannel::ALL {
        assert_eq!(
            packet.render_for_channel(channel),
            packet.export_safe_json()
        );
    }
}

#[test]
fn lookups_resolve() {
    let packet = packet();
    let fact = &packet.facts[0];
    assert!(packet.fact(fact.domain, &fact.subject).is_some());
    assert!(packet.fact(fact.domain, "no-such-subject").is_none());
    for consumer in ParityConsumer::ALL {
        assert!(packet.consumer_view(consumer).is_some());
    }
}
