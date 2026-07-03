//! Inline tests for the M5 boundary-inspector lane.

use super::*;

fn packet() -> M5BoundaryInspector {
    seeded_m5_boundary_inspector()
}

#[test]
fn canonical_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_BOUNDARY_INSPECTOR_PACKET_ID);
    assert_eq!(packet.record_kind, M5_BOUNDARY_INSPECTOR_RECORD_KIND);
    assert_eq!(packet.action_inspectors.len(), HighRiskAction::ALL.len());
    assert!(packet.conformance.all_hold());
    assert!(packet.vocabulary.matches_canonical());
}

#[test]
fn canonical_inspectors_are_all_governed() {
    // Acceptance: with every facet attested, every inspector is governed and Stable promotion stands.
    let packet = packet();
    for inspector in &packet.action_inspectors {
        assert!(inspector.is_governed(), "{}", inspector.action.as_str());
        assert_eq!(inspector.effective_gate, DescriptorGate::Governed);
        assert_eq!(
            inspector.effective_qualification,
            QualificationClass::Stable
        );
        assert!(inspector.gaps.is_empty());
        assert_eq!(
            inspector.boundary_card.effective_gate,
            DescriptorGate::Governed
        );
        assert_eq!(
            inspector.route_timeline.effective_gate,
            DescriptorGate::Governed
        );
        assert_eq!(
            inspector.approval_ticket.effective_gate,
            DescriptorGate::Governed
        );
    }
    assert!(!packet.blocks_stable_promotion());
    assert_eq!(
        packet.summary.governed_actions,
        HighRiskAction::ALL.len() as u32
    );
}

#[test]
fn every_action_is_inspected_once_with_three_cards() {
    // Acceptance: each action carries a boundary card, a route timeline, and an approval ticket.
    let packet = packet();
    for action in HighRiskAction::ALL {
        let inspectors: Vec<&ActionInspector> = packet
            .action_inspectors
            .iter()
            .filter(|i| i.action == action)
            .collect();
        assert_eq!(inspectors.len(), 1, "{}", action.as_str());
        let i = inspectors[0];
        assert_eq!(i.boundary_card.action, action);
        assert_eq!(i.route_timeline.action, action);
        assert_eq!(i.approval_ticket.action, action);
        assert!(!i.boundary_card.sensitive_data_classes.is_empty());
        assert!(!i.route_timeline.hops.is_empty());
        assert!(!i.approval_ticket.revoke_renew_actions.is_empty());
    }
}

#[test]
fn boundary_card_declares_class_actor_target_and_data() {
    // Acceptance: the boundary card declares boundary class, actor/source, target class, sensitive
    // data classes, approval source, and an export-safe summary.
    let packet = packet();
    assert!(
        packet
            .conformance
            .boundary_card_declares_class_actor_target_and_data
    );
    for c in packet.boundary_cards() {
        assert_eq!(c.boundary_class, c.action.boundary_class());
        assert_eq!(c.actor, c.action.actor());
        assert_eq!(c.source_locality, HopLocality::LocalMachine);
        assert_eq!(c.target_class, c.action.target_class());
        assert_eq!(c.approval_authority, c.action.approving_authority());
        assert!(!c.export_safe_summary.trim().is_empty());
        assert_eq!(c.evidence_class, EvidenceClass::BoundaryManifest);
    }
}

#[test]
fn local_action_does_not_cross_the_trust_boundary() {
    // A local-only action stays on the machine and reads local_only — proving where work ran.
    let packet = packet();
    let local = packet
        .inspector(HighRiskAction::LocalModelExecution)
        .unwrap();
    assert!(!local.crosses_trust_boundary);
    assert_eq!(local.route_timeline.route_state, RouteHopState::LocalOnly);
    assert!(!local.route_timeline.crosses_trust_boundary);
    assert!(local.route_timeline.hops.iter().all(|h| h.is_local));

    // A remote action crosses and is fully attributed.
    let remote = packet
        .inspector(HighRiskAction::RemoteModelInference)
        .unwrap();
    assert!(remote.crosses_trust_boundary);
    assert!(remote.route_timeline.crosses_trust_boundary);
    assert_eq!(
        remote.route_timeline.route_state,
        RouteHopState::AttributedRemote
    );
}

#[test]
fn route_timeline_is_ordered_with_locality_per_hop() {
    let packet = packet();
    assert!(
        packet
            .conformance
            .route_timeline_ordered_with_locality_per_hop
    );
    for t in packet.route_timelines() {
        for (idx, hop) in t.hops.iter().enumerate() {
            assert_eq!(hop.index, idx as u32);
            assert_eq!(hop.locality_label, hop.locality.label());
            assert_eq!(hop.hop_gate, hop.drift_marker.gate_posture());
        }
        assert_eq!(t.origin_locality, t.hops.first().unwrap().locality);
        assert_eq!(t.final_locality, t.hops.last().unwrap().locality);
    }
}

#[test]
fn approval_ticket_binds_authority_scope_and_expiry() {
    // Acceptance: approval sheets explain who granted what and for how long using the runtime
    // authority vocabulary (ApprovalState).
    let packet = packet();
    assert!(
        packet
            .conformance
            .approval_ticket_binds_authority_scope_and_expiry
    );
    for a in packet.approval_tickets() {
        assert_eq!(a.capability_class, a.action.capability_class());
        assert_eq!(a.approving_authority, a.action.approving_authority());
        assert!(!a.scope_summary.trim().is_empty());
        assert!(!a.expiry.trim().is_empty());
        assert!(ApprovalState::ALL.contains(&a.approval_state));
        assert_eq!(a.evidence_class, EvidenceClass::RuntimeApprovalRecord);
        // A governed ticket offers revoke + renew.
        assert!(a
            .revoke_renew_actions
            .contains(&TicketAction::RevokeApproval));
        assert!(a
            .revoke_renew_actions
            .contains(&TicketAction::RenewApproval));
    }
}

#[test]
fn boundary_edge_narrows_exactly_one_action() {
    // Acceptance: a boundary at its edge narrows only that action; the rest stay governed.
    let packet = seeded_m5_boundary_inspector_boundary_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let narrowed = packet
        .inspector(HighRiskAction::WorkspaceDataExport)
        .unwrap();
    assert_eq!(
        narrowed.boundary_card.boundary_state,
        CapabilityBoundaryState::AtBoundaryEdge
    );
    assert_eq!(
        narrowed.boundary_card.effective_gate,
        DescriptorGate::Narrowed
    );
    assert!(narrowed.is_narrowed());
    assert_eq!(narrowed.effective_qualification, QualificationClass::Beta);
    assert!(narrowed
        .gaps
        .iter()
        .any(|g| g.facet == InspectorFacet::Boundary
            && g.gap_kind == InspectorGapKind::FacetNarrowed));
    // Other actions stay governed; nothing blocks Stable.
    assert!(packet
        .inspector(HighRiskAction::RemoteModelInference)
        .unwrap()
        .is_governed());
    assert!(!packet.blocks_stable_promotion());
    assert_eq!(packet.summary.narrowed_actions, 1);
}

#[test]
fn route_drift_narrows_and_is_disclosed_on_the_timeline() {
    // Acceptance: a route drift narrows deterministically and never reads governed.
    let packet = seeded_m5_boundary_inspector_route_drift_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.conformance.route_drift_narrows_deterministically);
    let inspector = packet
        .inspector(HighRiskAction::RemoteModelInference)
        .unwrap();
    let t = &inspector.route_timeline;
    assert_eq!(t.route_state, RouteHopState::MirroredRoute);
    assert_eq!(t.effective_gate, DescriptorGate::Narrowed);
    assert!(t.drift_marker_count >= 1);
    assert!(t
        .hops
        .iter()
        .any(|h| matches!(h.drift_marker, HopDriftMarker::MirrorSubstitution)));
    assert!(inspector.is_narrowed());
    assert!(!packet.blocks_stable_promotion());
    assert_eq!(packet.summary.drifted_routes, 1);
}

#[test]
fn unattributed_route_blocks_stable_promotion() {
    // Acceptance: an unattributed hop blocks the inspector and holds Stable promotion.
    let packet = seeded_m5_boundary_inspector_route_unattributed_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(
        packet
            .conformance
            .unattributed_route_blocks_stable_promotion
    );
    let inspector = packet
        .inspector(HighRiskAction::SupportBundleHandoff)
        .unwrap();
    assert_eq!(
        inspector.route_timeline.route_state,
        RouteHopState::UnattributedRoute
    );
    assert_eq!(
        inspector.route_timeline.effective_gate,
        DescriptorGate::Blocked
    );
    assert_eq!(inspector.route_timeline.signal, DescriptorSignal::Red);
    assert!(inspector.is_blocked());
    assert_eq!(
        inspector.effective_qualification,
        QualificationClass::Unavailable
    );
    assert!(packet.blocks_stable_promotion());
    assert_eq!(packet.summary.blocked_actions, 1);
}

#[test]
fn expired_approval_blocks_stable_promotion() {
    // Acceptance: an expired approval blocks the inspector; the ticket offers a reapproval path.
    let packet = seeded_m5_boundary_inspector_approval_expired_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.conformance.expired_approval_blocks_stable_promotion);
    let inspector = packet
        .inspector(HighRiskAction::ProviderCredentialRotation)
        .unwrap();
    let a = &inspector.approval_ticket;
    assert_eq!(a.expiry_standing, ExpiryStanding::Expired);
    assert_eq!(a.effective_gate, DescriptorGate::Blocked);
    assert!(a
        .revoke_renew_actions
        .contains(&TicketAction::RequireReapproval));
    assert!(inspector.is_blocked());
    assert!(packet.blocks_stable_promotion());
    assert_eq!(packet.summary.expired_approvals, 1);
}

#[test]
fn inspector_gate_is_the_worst_of_its_facets() {
    // Guardrail: the inspector never reads safer than its least-attested facet.
    for packet in [
        seeded_m5_boundary_inspector_boundary_narrowed(),
        seeded_m5_boundary_inspector_route_drift_narrowed(),
        seeded_m5_boundary_inspector_route_unattributed_blocked(),
        seeded_m5_boundary_inspector_approval_expired_blocked(),
    ] {
        assert!(packet.conformance.inspector_gate_is_worst_of_facets);
        for i in &packet.action_inspectors {
            let worst = [
                i.boundary_card.effective_gate,
                i.route_timeline.effective_gate,
                i.approval_ticket.effective_gate,
            ]
            .into_iter()
            .max_by_key(|g| match g {
                DescriptorGate::Governed => 0,
                DescriptorGate::Narrowed => 1,
                DescriptorGate::Blocked => 2,
            })
            .unwrap();
            assert_eq!(i.effective_gate, worst, "{}", i.action.as_str());
        }
    }
}

#[test]
fn evaluation_packet_reuses_the_ui_vocabulary() {
    // Acceptance: exports reuse the same boundary / route / approval vocabulary the cards show.
    for packet in [
        packet(),
        seeded_m5_boundary_inspector_route_drift_narrowed(),
        seeded_m5_boundary_inspector_approval_expired_blocked(),
    ] {
        assert!(packet.conformance.evaluation_packet_reuses_ui_vocabulary);
        let export = &packet.evaluation_packet;
        assert!(export.vocabulary.matches_canonical());
        for entry in &export.actions {
            let inspector = packet.inspector(entry.action).unwrap();
            assert_eq!(entry.boundary_state, inspector.boundary_card.boundary_state);
            assert_eq!(entry.route_state, inspector.route_timeline.route_state);
            assert_eq!(
                entry.approval_state,
                inspector.approval_ticket.approval_state
            );
            assert_eq!(entry.effective_gate, inspector.effective_gate);
            assert_eq!(entry.proof_refs.len(), 3);
        }
    }
}

#[test]
fn channels_produce_identical_output() {
    let packet = packet();
    let desktop = packet.render_for_channel(BoundaryInspectorChannel::DesktopUi);
    let cli = packet.render_for_channel(BoundaryInspectorChannel::CliHeadless);
    let offline = packet.render_for_channel(BoundaryInspectorChannel::OfflineMirror);
    assert_eq!(desktop, cli);
    assert_eq!(cli, offline);
    assert_eq!(desktop, packet.export_safe_json());
}

#[test]
fn controlled_vocabulary_is_frozen() {
    let vocab = BoundaryInspectorVocabulary::canonical();
    assert_eq!(vocab.actions.len(), HighRiskAction::ALL.len());
    assert_eq!(
        vocab.boundary_states.len(),
        CapabilityBoundaryState::ALL.len()
    );
    assert_eq!(vocab.route_states.len(), RouteHopState::ALL.len());
    assert_eq!(vocab.approval_states.len(), ApprovalState::ALL.len());
    for needle in [
        "local_only",
        "attributed_remote",
        "mirrored_route",
        "unattributed_route",
    ] {
        assert!(vocab.route_states.contains(&needle.to_owned()));
    }
    for needle in ["pre_authorized", "approved", "approval_denied"] {
        assert!(vocab.approval_states.contains(&needle.to_owned()));
    }
}

#[test]
fn actions_csv_enumerates_action_boundary_route_and_approval() {
    let csv = packet().render_actions_csv();
    let header = csv.lines().next().unwrap();
    assert!(header.starts_with("action,boundary_class,boundary_state,actor,target_class,"));
    assert!(header.contains("route_state"));
    assert!(header.contains("approval_state"));
    assert!(header.contains("effective_gate"));
    for action in HighRiskAction::ALL {
        assert!(csv.contains(&format!(
            "{},{}",
            action.as_str(),
            action.boundary_class().as_str()
        )));
    }
}

#[test]
fn overview_markdown_names_every_section() {
    let md = seeded_m5_boundary_inspector_route_drift_narrowed().render_overview_markdown();
    assert!(md.contains("# M5 Boundary Inspector"));
    assert!(md.contains("Action inspectors"));
    assert!(md.contains("Boundary summary cards"));
    assert!(md.contains("Route-hop timelines"));
    assert!(md.contains("Approval tickets"));
    assert!(md.contains("mirror_substitution"));
}

#[test]
fn packet_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: M5BoundaryInspector = serde_json::from_str(&json).expect("packet deserializes");
    assert_eq!(parsed, packet);
    assert!(parsed.validate().is_empty());
}

#[test]
fn tampered_boundary_state_is_rejected() {
    let mut packet = seeded_m5_boundary_inspector_boundary_narrowed();
    let idx = packet
        .action_inspectors
        .iter()
        .position(|i| i.boundary_card.boundary_state == CapabilityBoundaryState::AtBoundaryEdge)
        .expect("a narrowed boundary exists");
    packet.action_inspectors[idx].boundary_card.boundary_state =
        CapabilityBoundaryState::WithinBoundary;
    packet.action_inspectors[idx].boundary_card.effective_gate = DescriptorGate::Governed;
    let violations = packet.validate();
    assert!(
        violations.contains(&M5BoundaryInspectorViolation::BoundaryCardDrift)
            || violations.contains(&M5BoundaryInspectorViolation::InspectorGateDrift),
        "{violations:?}"
    );
}

#[test]
fn route_state_that_understates_drift_is_rejected() {
    // A governed route state with a drifting hop is a contradiction the validator catches.
    let mut packet = packet();
    let idx = packet
        .action_inspectors
        .iter()
        .position(|i| i.action == HighRiskAction::RemoteModelInference)
        .unwrap();
    let t = &mut packet.action_inspectors[idx].route_timeline;
    if let Some(last) = t.hops.last_mut() {
        last.drift_marker = HopDriftMarker::UnattributedHop;
        last.hop_gate = HopDriftMarker::UnattributedHop.gate_posture();
    }
    // route_state stays AttributedRemote (governed) while a hop is unattributed (blocked).
    let violations = packet.validate();
    assert!(
        violations.contains(&M5BoundaryInspectorViolation::RouteStateDriftMismatch)
            || violations.contains(&M5BoundaryInspectorViolation::RouteGateDrift),
        "{violations:?}"
    );
}

#[test]
fn dropping_an_action_is_rejected() {
    let mut packet = packet();
    packet
        .action_inspectors
        .retain(|i| i.action != HighRiskAction::ControlPlaneSync);
    let violations = packet.validate();
    assert!(violations.contains(&M5BoundaryInspectorViolation::ActionNotInspected));
}

#[test]
fn export_carries_no_raw_material() {
    for packet in [
        packet(),
        seeded_m5_boundary_inspector_route_drift_narrowed(),
        seeded_m5_boundary_inspector_approval_expired_blocked(),
        seeded_m5_boundary_inspector_route_unattributed_blocked(),
    ] {
        assert!(packet.conformance.export_carries_no_raw_material);
        let lower = packet.export_safe_json().to_ascii_lowercase();
        assert!(!lower.contains("bearer_token"));
        assert!(!lower.contains("\"secret\""));
    }
}
