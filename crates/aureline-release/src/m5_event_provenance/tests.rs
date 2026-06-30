//! Inline tests for the M5 event-provenance lane.

use super::*;

fn packet() -> M5EventProvenance {
    seeded_m5_event_provenance()
}

#[test]
fn canonical_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_EVENT_PROVENANCE_PACKET_ID);
    assert_eq!(packet.record_kind, M5_EVENT_PROVENANCE_RECORD_KIND);
    assert_eq!(packet.deferred_events.len(), DeferredAction::ALL.len());
    assert!(packet.conformance.all_hold());
    assert!(packet.vocabulary.matches_canonical());
}

#[test]
fn canonical_events_are_all_governed() {
    // With every facet attested and no drift, every event is governed and may replay as-is.
    let packet = packet();
    for event in &packet.deferred_events {
        assert!(event.is_governed(), "{}", event.action.as_str());
        assert_eq!(event.effective_gate, DescriptorGate::Governed);
        assert_eq!(event.effective_qualification, QualificationClass::Stable);
        assert!(event.gaps.is_empty());
        assert_eq!(
            event.provenance_row.effective_gate,
            DescriptorGate::Governed
        );
        assert_eq!(event.drift_banner.effective_gate, DescriptorGate::Governed);
        assert_eq!(
            event.reapproval_gate.effective_gate,
            DescriptorGate::Governed
        );
        assert_eq!(
            event.reapproval_gate.decision,
            ReapprovalDecision::ReplayAsIs
        );
        assert!(!event.drift_banner.has_drift);
    }
    assert!(!packet.blocks_stable_promotion());
    assert_eq!(
        packet.summary.governed_events,
        DeferredAction::ALL.len() as u32
    );
}

#[test]
fn every_action_is_inspected_once_with_three_facets() {
    // Each action carries a provenance row, a route-drift banner, and a reapproval gate.
    let packet = packet();
    for action in DeferredAction::ALL {
        let events: Vec<&DeferredEvent> = packet
            .deferred_events
            .iter()
            .filter(|e| e.action == action)
            .collect();
        assert_eq!(events.len(), 1, "{}", action.as_str());
        let e = events[0];
        assert_eq!(e.provenance_row.action, action);
        assert_eq!(e.drift_banner.action, action);
        assert_eq!(e.reapproval_gate.action, action);
    }
}

#[test]
fn provenance_row_links_event_mutation_run_session() {
    // Implementation requirement: stable provenance rows with event id, mutation/run/session linkage,
    // host lane, retrieval epoch, and redaction posture.
    let packet = packet();
    assert!(
        packet
            .conformance
            .provenance_row_links_event_mutation_run_session
    );
    assert!(
        packet
            .conformance
            .provenance_row_declares_host_lane_epoch_and_redaction
    );
    for r in packet.provenance_rows() {
        assert_eq!(r.event_id, r.action.event_id());
        assert!(!r.mutation_ref.trim().is_empty());
        assert!(!r.run_ref.trim().is_empty());
        assert!(!r.session_ref.trim().is_empty());
        assert!(!r.retrieval_epoch.trim().is_empty());
        assert_eq!(r.host_lane, r.action.host_lane());
        assert_eq!(r.redaction_posture, r.action.redaction_posture());
        assert_eq!(r.evidence_class, EvidenceClass::ProvenanceLedger);
        assert_eq!(r.surface, r.action.surface());
    }
}

#[test]
fn local_event_does_not_cross_the_trust_boundary() {
    // A local event stays on the machine and reads local_only; a remote event crosses and is
    // attributed — proving where work ran.
    let packet = packet();
    let local = packet.event(DeferredAction::ReplayedAuditExport).unwrap();
    assert!(!local.crosses_trust_boundary);
    assert!(local.provenance_row.is_local);
    assert_eq!(local.drift_banner.route_state, RouteHopState::LocalOnly);
    assert_eq!(local.trust_boundaries, vec![TrustBoundary::LocalFirst]);

    let remote = packet.event(DeferredAction::QueuedPromptReplay).unwrap();
    assert!(remote.crosses_trust_boundary);
    assert!(!remote.provenance_row.is_local);
    assert_eq!(
        remote.drift_banner.route_state,
        RouteHopState::AttributedRemote
    );
}

#[test]
fn flows_span_ai_provider_remote_and_support() {
    // The deferred actions span the AI, provider, remote, and support flows.
    let packet = packet();
    let flows: std::collections::BTreeSet<&str> = packet
        .deferred_events
        .iter()
        .map(|e| e.flow.as_str())
        .collect();
    for flow in ActionFlow::ALL {
        assert!(
            flows.contains(flow.as_str()),
            "missing flow {}",
            flow.as_str()
        );
    }
}

#[test]
fn provenance_stale_narrows_only_its_event() {
    // Acceptance: a stale provenance ledger narrows the one event; the rest stay governed.
    let packet = seeded_m5_event_provenance_provenance_stale_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let drilled = packet.event(DeferredAction::QueuedPromptReplay).unwrap();
    assert!(drilled.is_narrowed());
    assert_eq!(
        drilled.provenance_row.provenance_state,
        ProvenanceState::ProvenanceStale
    );
    assert_eq!(
        drilled.provenance_row.effective_gate,
        DescriptorGate::Narrowed
    );
    assert!(drilled
        .gaps
        .iter()
        .any(|g| matches!(g.facet, EventFacet::Provenance)));
    assert!(!packet.blocks_stable_promotion());
    assert_eq!(packet.summary.narrowed_events, 1);
}

#[test]
fn region_drift_names_the_changed_fact_and_narrows() {
    // Acceptance: drift conditions are visible and actionable, not inferred from generic failures.
    let packet = seeded_m5_event_provenance_drift_region_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let drilled = packet
        .event(DeferredAction::PublishLaterDataExport)
        .unwrap();
    assert!(drilled.is_narrowed());
    assert!(drilled.drift_banner.has_drift);
    assert_eq!(drilled.drift_banner.drift_count, 1);
    let fact = &drilled.drift_banner.drifted_facets[0];
    assert_eq!(fact.facet, DriftFacet::Region);
    assert_eq!(fact.facet_gate, DescriptorGate::Narrowed);
    assert!(!fact.planned_ref.trim().is_empty());
    assert!(!fact.current_ref.trim().is_empty());
    // The reapproval gate also re-approves because the boundary cannot be assumed unchanged.
    assert_eq!(packet.summary.drifted_events, 1);
    assert!(!packet.blocks_stable_promotion());
}

#[test]
fn tenant_drift_blocks_stable_promotion() {
    // Acceptance: queued/replayable actions do not cross changed boundaries invisibly — a tenant
    // change blocks.
    let packet = seeded_m5_event_provenance_drift_tenant_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.conformance.tenant_drift_blocks_stable_promotion);
    let drilled = packet
        .event(DeferredAction::QueuedControlPlaneSync)
        .unwrap();
    assert!(drilled.is_blocked());
    assert_eq!(drilled.drift_banner.effective_gate, DescriptorGate::Blocked);
    let fact = &drilled.drift_banner.drifted_facets[0];
    assert_eq!(fact.facet, DriftFacet::Tenant);
    assert_eq!(fact.baseline, DriftBaseline::Plan);
    assert!(packet.blocks_stable_promotion());
    assert_eq!(packet.summary.blocked_events, 1);
}

#[test]
fn narrowed_boundary_requires_reapproval() {
    // Implementation requirement: require replay/publish-later/approve-again gates when current
    // boundary facts invalidate the earlier assumptions.
    let packet = seeded_m5_event_provenance_reapproval_required_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.conformance.changed_boundary_facts_require_reapproval);
    let drilled = packet
        .event(DeferredAction::ScheduledCredentialRotation)
        .unwrap();
    assert!(drilled.is_narrowed());
    assert_eq!(
        drilled.reapproval_gate.boundary_state,
        CapabilityBoundaryState::AtBoundaryEdge
    );
    assert_eq!(
        drilled.reapproval_gate.decision,
        ReapprovalDecision::RequireReapproval
    );
    assert_eq!(
        drilled.reapproval_gate.deferred_kind,
        DeferredKind::ApproveAgain
    );
    assert_eq!(packet.summary.reapproval_required, 1);
    assert!(!packet.blocks_stable_promotion());
}

#[test]
fn denied_approval_holds_the_action() {
    let packet = seeded_m5_event_provenance_reapproval_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let drilled = packet.event(DeferredAction::RetriedPolicyPush).unwrap();
    assert!(drilled.is_blocked());
    assert_eq!(
        drilled.reapproval_gate.approval_state,
        ApprovalState::ApprovalDenied
    );
    assert_eq!(
        drilled.reapproval_gate.decision,
        ReapprovalDecision::HoldBlocked
    );
    assert_eq!(packet.summary.reapproval_blocked, 1);
    assert!(packet.blocks_stable_promotion());
}

#[test]
fn event_gate_is_worst_of_three_facets() {
    // The event's gate never reads safer than its least-attested facet.
    for packet in [
        seeded_m5_event_provenance(),
        seeded_m5_event_provenance_provenance_stale_narrowed(),
        seeded_m5_event_provenance_drift_region_narrowed(),
        seeded_m5_event_provenance_drift_tenant_blocked(),
        seeded_m5_event_provenance_reapproval_required_narrowed(),
        seeded_m5_event_provenance_reapproval_blocked(),
    ] {
        assert!(packet.conformance.event_gate_is_worst_of_facets);
        for e in &packet.deferred_events {
            let expected = [
                e.provenance_row.effective_gate,
                e.drift_banner.effective_gate,
                e.reapproval_gate.effective_gate,
            ]
            .into_iter()
            .max_by_key(|g| gate_rank(*g))
            .unwrap();
            assert_eq!(e.effective_gate, expected, "{}", e.action.as_str());
        }
    }
}

#[test]
fn export_preview_reuses_ui_vocabulary_and_is_refs_only() {
    // Acceptance: event provenance stays exportable and useful without leaking secrets.
    let packet = packet();
    assert!(packet.conformance.export_preview_reuses_ui_vocabulary);
    assert!(packet.conformance.export_carries_no_raw_material);
    let preview = &packet.export_preview;
    assert_eq!(preview.record_kind, M5_EVENT_PROVENANCE_EXPORT_RECORD_KIND);
    assert_eq!(preview.events.len(), packet.deferred_events.len());
    assert!(preview.reuses_canonical_vocabulary());
    for entry in &preview.events {
        // Lineage is preserved as refs only — three proof refs per event.
        assert_eq!(entry.proof_refs.len(), 3);
        for r in &entry.proof_refs {
            assert!(!r.trim().is_empty());
        }
    }
    // The preview re-derives byte-identically from the events.
    let regenerated = M5EventProvenance::new(M5EventProvenanceInput {
        packet_id: packet.packet_id.clone(),
        report_label: packet.report_label.clone(),
        evaluated_at: packet.evaluated_at.clone(),
        event_seeds: Vec::new(),
        redaction_class_token: packet.redaction_class_token.clone(),
        minted_at: packet.minted_at.clone(),
    });
    assert!(regenerated.deferred_events.is_empty());
}

#[test]
fn vocabulary_reuses_frozen_governance_tokens() {
    // The lane binds to the frozen provenance / route / approval / boundary vocabularies the
    // governance matrix froze, not a parallel grammar.
    let vocab = EventProvenanceVocabulary::canonical();
    assert_eq!(
        vocab.provenance_states,
        ProvenanceState::ALL
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        vocab.route_states,
        RouteHopState::ALL
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        vocab.approval_states,
        ApprovalState::ALL
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        vocab.boundary_states,
        CapabilityBoundaryState::ALL
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
    );
}

#[test]
fn channels_render_byte_identically() {
    let packet = packet();
    let desktop = packet.render_for_channel(EventProvenanceChannel::DesktopUi);
    for channel in EventProvenanceChannel::ALL {
        assert_eq!(packet.render_for_channel(channel), desktop);
    }
}

#[test]
fn renderers_are_deterministic_and_nonempty() {
    let packet = packet();
    assert_eq!(packet.export_safe_json(), packet.export_safe_json());
    assert_eq!(packet.render_events_csv(), packet.render_events_csv());
    assert!(packet
        .render_overview_markdown()
        .contains("# M5 Event Provenance"));
    assert!(packet.render_markdown_summary().contains("Proof"));
    assert!(packet
        .render_events_csv()
        .lines()
        .count()
        .eq(&(DeferredAction::ALL.len() + 1)));
}

#[test]
fn detects_tampered_packet() {
    // Editing a row's state without re-deriving its gate makes the row read safer than its proof,
    // which validation catches.
    let mut state_tamper = packet();
    state_tamper.deferred_events[0]
        .provenance_row
        .provenance_state = ProvenanceState::ProvenanceMissing;
    assert!(!state_tamper.validate().is_empty());

    // Forcing an event's verdict below the worst of its facets is also caught.
    let mut gate_tamper = packet();
    gate_tamper.deferred_events[0].effective_gate = DescriptorGate::Blocked;
    assert!(!gate_tamper.validate().is_empty());
}

#[test]
fn export_carries_no_forbidden_keys() {
    for packet in [
        seeded_m5_event_provenance(),
        seeded_m5_event_provenance_drift_tenant_blocked(),
        seeded_m5_event_provenance_reapproval_blocked(),
    ] {
        let value = serde_json::to_value(&packet).unwrap();
        assert!(!json_contains_forbidden_material(&value));
    }
}
