//! Inline tests for the M5 runbook certification packet.

use super::*;

fn canonical() -> M5RunbookCertificationPacket {
    seeded_m5_runbook_certification_packet()
}

#[test]
fn canonical_packet_validates() {
    let packet = canonical();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_RUNBOOK_CERTIFICATION_PACKET_ID);
    assert_eq!(packet.record_kind, M5_RUNBOOK_CERTIFICATION_RECORD_KIND);
    assert!(!packet.rows.is_empty());
    assert_eq!(packet.proof_lanes.len(), RunbookProofLane::ALL.len());
    assert!(packet.conformance.all_hold());
    assert!(packet.vocabulary.matches_canonical());
    assert!(packet.disclosure.all_expose());
}

#[test]
fn every_facet_is_covered_by_a_proof_lane() {
    let packet = canonical();
    for facet in CertificationFacet::ALL {
        assert!(
            packet.proof_lanes.iter().any(|l| l.facet == facet),
            "facet {} not covered",
            facet.as_str()
        );
    }
    // Each lane derives its refs from the lane itself.
    for lane in &packet.proof_lanes {
        assert!(lane.validate().is_empty(), "{:?}", lane.validate());
        assert_eq!(lane.schema_ref, lane.lane.schema_ref());
        assert_eq!(lane.proof_ref, lane.lane.proof_ref());
        assert_eq!(lane.register_ref, lane.lane.register_ref());
    }
}

#[test]
fn canonical_rows_are_all_certified() {
    let packet = canonical();
    for row in &packet.rows {
        assert!(row.is_certified(), "{} not certified", row.row_id);
        assert!(row.gaps.is_empty(), "{} has gaps", row.row_id);
        assert_eq!(row.effective_class, RunbookClaimClass::Stable);
        assert_eq!(row.status, RunbookSurfaceStatus::Mapped);
        assert!(!row.bound_lanes.is_empty());
        assert!(!row.covered_facets.is_empty());
    }
    assert!(!packet.blocks_stable_promotion());
    assert_eq!(
        packet.summary.certified_row_count as usize,
        packet.rows.len()
    );
    assert_eq!(packet.summary.blocked_row_count, 0);
    assert_eq!(packet.summary.narrowed_row_count, 0);
    assert_eq!(
        packet.summary.current_lane_count,
        RunbookProofLane::ALL.len() as u32
    );
}

#[test]
fn stale_lane_proof_narrows_bound_rows_deterministically() {
    let packet = seeded_m5_runbook_certification_packet_stale_proof_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // The handoff lane is stale; it does not block (nothing is missing).
    assert!(!packet.blocks_stable_promotion());

    let stale_lane = packet
        .lane_contract(RunbookProofLane::Handoffs)
        .expect("handoff lane");
    assert_eq!(stale_lane.proof_freshness, ProofFreshnessState::Stale);

    for row in &packet.rows {
        let binds_handoffs = row.bound_lanes.contains(&RunbookProofLane::Handoffs);
        if binds_handoffs {
            assert!(row.is_narrowed(), "{} should narrow", row.row_id);
            assert_eq!(row.effective_class, RunbookClaimClass::Beta);
            assert_eq!(row.status, RunbookSurfaceStatus::Provisional);
            assert!(row.gaps.iter().any(|g| g.lane == RunbookProofLane::Handoffs
                && g.gap_kind == RunbookGapKind::ProofStale));
        } else {
            assert!(row.is_certified(), "{} should stay certified", row.row_id);
            assert_eq!(row.effective_class, RunbookClaimClass::Stable);
        }
    }
    assert!(packet.summary.narrowed_row_count > 0);
    assert_eq!(packet.summary.stale_lane_count, 1);
}

#[test]
fn missing_lane_proof_blocks_bound_rows() {
    let packet = seeded_m5_runbook_certification_packet_missing_proof_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.blocks_stable_promotion());

    let missing_lane = packet
        .lane_contract(RunbookProofLane::Companion)
        .expect("companion lane");
    assert_eq!(missing_lane.proof_freshness, ProofFreshnessState::Missing);

    for row in &packet.rows {
        let binds_companion = row.bound_lanes.contains(&RunbookProofLane::Companion);
        if binds_companion {
            assert!(row.is_blocked(), "{} should block", row.row_id);
            assert_eq!(row.effective_class, RunbookClaimClass::Held);
            assert_eq!(row.status, RunbookSurfaceStatus::Unmapped);
            assert!(row
                .gaps
                .iter()
                .any(|g| g.lane == RunbookProofLane::Companion
                    && g.gap_kind == RunbookGapKind::ProofMissing));
        } else {
            assert!(!row.is_blocked(), "{} should not block", row.row_id);
        }
    }
    assert!(packet.summary.blocked_row_count > 0);
    assert_eq!(packet.summary.missing_lane_count, 1);
    assert!(packet
        .release_gate
        .blocked_row_ids
        .contains(&"companion-runbook-follow".to_owned()));
}

#[test]
fn row_verdict_drift_is_detected() {
    let mut packet = canonical();
    // Hand-edit a row's effective class to a stale value; validation must catch it.
    packet.rows[0].effective_class = RunbookClaimClass::Preview;
    let violations = packet.validate();
    assert!(violations.contains(&M5RunbookCertificationViolation::RowVerdictDrift));
}

#[test]
fn unmapped_bound_lane_blocks_row() {
    // A row that binds a lane the packet does not govern must block, with a named gap.
    let lanes: Vec<RunbookProofLaneContract> = RunbookProofLane::ALL
        .iter()
        .filter(|l| **l != RunbookProofLane::Handoffs)
        .map(|l| RunbookProofLaneContract::for_lane(*l, ProofFreshnessState::Current))
        .collect();
    let mut row = IncidentOperatorRow {
        row_id: "probe".to_owned(),
        row_label: "probe".to_owned(),
        consumer: RunbookConsumer::OperatorDashboard,
        owner_role: "operator_console_owner".to_owned(),
        claimed_class: RunbookClaimClass::Stable,
        bound_lanes: vec![RunbookProofLane::Governance, RunbookProofLane::Handoffs],
        covered_facets: Vec::new(),
        effective_class: RunbookClaimClass::Stable,
        status: RunbookSurfaceStatus::Mapped,
        signal: RunbookSignal::Green,
        gate_decision: RunbookGate::Governed,
        gaps: Vec::new(),
        status_message_id: format!("{}probe.status", M5_RUNBOOK_CERTIFICATION_MESSAGE_ID_PREFIX),
        gate_message_id: format!("{}probe.gate", M5_RUNBOOK_CERTIFICATION_MESSAGE_ID_PREFIX),
    };
    row.recompute(&lanes);
    assert!(row.is_blocked());
    assert!(row.gaps.iter().any(|g| g.lane == RunbookProofLane::Handoffs
        && g.gap_kind == RunbookGapKind::ObjectMappingMissing));
}

#[test]
fn export_carries_no_raw_boundary_material() {
    let packet = canonical();
    let json = packet.export_safe_json();
    let lower = json.to_ascii_lowercase();
    for needle in [
        "credential",
        "secret",
        "password",
        "api_key",
        "bearer_token",
    ] {
        assert!(!lower.contains(needle), "export leaked `{needle}`");
    }
}

#[test]
fn markdown_summary_renders_lanes_and_rows() {
    let packet = canonical();
    let md = packet.render_markdown_summary();
    assert!(md.contains("# M5 Runbook Certification"));
    assert!(md.contains("Runbook proof lanes"));
    assert!(md.contains("Claimed incident/operator rows"));
    assert!(md.contains("Help/About, shiproom, support exports, incident/operator"));
}

#[test]
fn drill_packets_serialize_round_trip() {
    for packet in [
        seeded_m5_runbook_certification_packet(),
        seeded_m5_runbook_certification_packet_stale_proof_narrowed(),
        seeded_m5_runbook_certification_packet_missing_proof_blocked(),
    ] {
        let json = packet.export_safe_json();
        let back: M5RunbookCertificationPacket = serde_json::from_str(&json).expect("round trips");
        assert_eq!(back, packet);
        assert!(back.validate().is_empty(), "{:?}", back.validate());
    }
}
