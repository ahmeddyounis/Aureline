use super::*;

fn packet() -> M5HostBoundaryMatrix {
    current_m5_host_boundary_matrix().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_HOST_BOUNDARY_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_HOST_BOUNDARY_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_claimed_lane_has_exactly_one_row() {
    let packet = packet();
    assert_eq!(packet.lanes.len(), packet.execution_lanes.len());
    for &lane in &packet.execution_lanes {
        assert!(
            packet.lane(lane).is_some(),
            "missing row for lane {}",
            lane.as_str()
        );
    }
}

#[test]
fn every_lane_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_lanes_gate_consistent());
    for row in &packet.lanes {
        assert_eq!(
            row.published_attribution,
            row.effective_attribution(),
            "lane {} publishes beyond the gate",
            row.lane_id
        );
        assert_eq!(
            row.published_locus,
            row.derived_locus(),
            "lane {} locus diverges from its host kind",
            row.lane_id
        );
        assert_eq!(
            row.boundary_decision,
            row.required_decision(),
            "lane {} decision diverges from the gate",
            row.lane_id
        );
        assert_eq!(
            row.narrowing_reasons,
            row.computed_narrowing_reasons(),
            "lane {} narrowing reasons diverge from the gate",
            row.lane_id
        );
    }
}

#[test]
fn every_lane_carries_its_own_evidence() {
    let packet = packet();
    for row in &packet.lanes {
        assert!(
            row.has_required_evidence(),
            "lane {} is missing required evidence refs",
            row.lane_id
        );
        assert!(
            !row.execution_ref.trim().is_empty(),
            "lane {} has no execution ref to join in support exports",
            row.lane_id
        );
    }
}

#[test]
fn rebound_lanes_carry_a_reviewable_diff() {
    let packet = packet();
    for row in &packet.lanes {
        if row.host_binding_state.is_rebind() {
            assert!(
                !row.previous_host_ref.trim().is_empty(),
                "lane {} rebound but has no previous-host ref",
                row.lane_id
            );
            assert!(
                !row.rebind_diff_ref.trim().is_empty(),
                "lane {} rebound but has no rebind-diff ref",
                row.lane_id
            );
        }
    }
}

#[test]
fn export_projection_reflects_rows_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.lanes.len(), packet.lanes.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_lanes_gate_consistent,
        packet.all_lanes_gate_consistent()
    );
    assert_eq!(
        projection.publishable_count,
        packet.publishable_lanes().count()
    );
    assert_eq!(projection.narrowed_count, packet.narrowed_lanes().count());
    assert_eq!(projection.withheld_count, packet.withheld_lanes().count());
    for (row, export) in packet.lanes.iter().zip(projection.lanes.iter()) {
        assert_eq!(export.execution_ref, row.execution_ref);
        assert_eq!(export.confirmed_origin, row.is_publishable());
        assert_eq!(export.crossed_boundary, row.host_kind.crosses_boundary());
        assert_eq!(export.published_locus, row.published_locus.as_str());
    }
}

#[test]
fn published_attributions_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<AttributionConfidence> = packet
        .lanes
        .iter()
        .map(|l| l.published_attribution)
        .collect();
    for confidence in AttributionConfidence::ALL {
        assert!(
            present.contains(&confidence),
            "no lane publishes attribution {}",
            confidence.as_str()
        );
    }
}

#[test]
fn boundary_decisions_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<BoundaryDecision> =
        packet.lanes.iter().map(|l| l.boundary_decision).collect();
    for decision in BoundaryDecision::ALL {
        assert!(
            present.contains(&decision),
            "no lane exercises decision {}",
            decision.as_str()
        );
    }
}

#[test]
fn host_kinds_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<HostKind> = packet.lanes.iter().map(|l| l.host_kind).collect();
    for kind in HostKind::ALL {
        assert!(
            present.contains(&kind),
            "no lane exercises host kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn origin_loci_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<OriginLocus> = packet.lanes.iter().map(|l| l.published_locus).collect();
    for locus in OriginLocus::ALL {
        assert!(
            present.contains(&locus),
            "no lane exercises origin locus {}",
            locus.as_str()
        );
    }
}

#[test]
fn origin_receipt_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<OriginReceiptState> = packet
        .lanes
        .iter()
        .map(|l| l.origin_receipt_state)
        .collect();
    for state in OriginReceiptState::ALL {
        assert!(
            present.contains(&state),
            "no lane exercises origin-receipt state {}",
            state.as_str()
        );
    }
}

#[test]
fn connection_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ConnectionState> =
        packet.lanes.iter().map(|l| l.connection_state).collect();
    for state in ConnectionState::ALL {
        assert!(
            present.contains(&state),
            "no lane exercises connection state {}",
            state.as_str()
        );
    }
}

#[test]
fn host_binding_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<HostBindingState> =
        packet.lanes.iter().map(|l| l.host_binding_state).collect();
    for state in HostBindingState::ALL {
        assert!(
            present.contains(&state),
            "no lane exercises host-binding state {}",
            state.as_str()
        );
    }
}

#[test]
fn export_continuity_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ExportContinuityState> = packet
        .lanes
        .iter()
        .map(|l| l.export_continuity_state)
        .collect();
    for state in ExportContinuityState::ALL {
        assert!(
            present.contains(&state),
            "no lane exercises export-continuity state {}",
            state.as_str()
        );
    }
}

#[test]
fn narrowing_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<HostNarrowingReason> = packet
        .lanes
        .iter()
        .flat_map(|l| l.narrowing_reasons.iter().copied())
        .collect();
    for reason in HostNarrowingReason::ALL {
        assert!(
            present.contains(&reason),
            "no lane exercises narrowing reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn publishable_lanes_are_clean() {
    let packet = packet();
    assert!(
        packet.publishable_lanes().count() > 0,
        "fixture needs a confirmed lane"
    );
    for row in packet.publishable_lanes() {
        assert_eq!(row.capability_floor(), AttributionConfidence::Confirmed);
        assert!(row.narrowing_reasons.is_empty());
        assert_eq!(row.published_attribution, AttributionConfidence::Confirmed);
        assert_eq!(row.boundary_decision, BoundaryDecision::Publish);
    }
}

#[test]
fn ceilings_hold_for_each_state() {
    assert_eq!(
        OriginReceiptState::Missing.confidence_ceiling(),
        AttributionConfidence::Unattributed
    );
    assert_eq!(
        OriginReceiptState::Inferred.confidence_ceiling(),
        AttributionConfidence::Provisional
    );
    assert_eq!(
        OriginReceiptState::Recorded.confidence_ceiling(),
        AttributionConfidence::Attributed
    );
    assert_eq!(
        ConnectionState::Bridged.confidence_ceiling(),
        AttributionConfidence::Attributed
    );
    assert_eq!(
        ConnectionState::Reconnecting.confidence_ceiling(),
        AttributionConfidence::Provisional
    );
    assert_eq!(
        ConnectionState::Stale.confidence_ceiling(),
        AttributionConfidence::Stale
    );
    assert_eq!(
        HostBindingState::Unbound.confidence_ceiling(),
        AttributionConfidence::Provisional
    );
    assert_eq!(
        ExportContinuityState::Broken.confidence_ceiling(),
        AttributionConfidence::Unattributed
    );
}

#[test]
fn locus_is_pinned_to_host_kind() {
    assert_eq!(HostKind::Local.locus(), OriginLocus::Local);
    assert_eq!(HostKind::Ssh.locus(), OriginLocus::Remote);
    assert_eq!(HostKind::Container.locus(), OriginLocus::Remote);
    assert_eq!(HostKind::ManagedWorkspace.locus(), OriginLocus::Managed);
    assert_eq!(HostKind::BrowserBridge.locus(), OriginLocus::Bridged);
    assert_eq!(HostKind::ServicePlane.locus(), OriginLocus::ServicePlane);
    assert!(HostKind::Local.is_local());
    assert!(!HostKind::Local.crosses_boundary());
    assert!(HostKind::BrowserBridge.crosses_boundary());
}

#[test]
fn bridged_runtime_never_reads_as_local() {
    // The guardrail: a browser/companion bridge surface must never imply that work
    // ran locally; the boundary is flagged and the locus stays bridged.
    let packet = packet();
    let row = packet
        .lane(ExecutionLane::RequestRuntimeMutation)
        .expect("request-runtime-mutation row");
    assert_eq!(row.host_kind, HostKind::BrowserBridge);
    assert_eq!(row.published_locus, OriginLocus::Bridged);
    assert_ne!(row.published_locus, OriginLocus::Local);
    assert_eq!(row.connection_state, ConnectionState::Bridged);
    assert_eq!(row.boundary_decision, BoundaryDecision::FlagForReview);
    assert!(row
        .narrowing_reasons
        .contains(&HostNarrowingReason::BridgedBoundary));
}

#[test]
fn remote_confirmed_origin_is_not_labelled_local() {
    // A remote/managed host can be fully attributed, but its locus must stay managed
    // so a confirmed origin never masquerades as local execution.
    let packet = packet();
    let row = packet
        .lane(ExecutionLane::ManagedWorkspaceRun)
        .expect("managed-workspace-run row");
    assert_eq!(row.published_attribution, AttributionConfidence::Confirmed);
    assert_eq!(row.boundary_decision, BoundaryDecision::Publish);
    assert_eq!(row.published_locus, OriginLocus::Managed);
    assert!(row.host_kind.crosses_boundary());
}

#[test]
fn stale_managed_context_narrows_without_poisoning_local() {
    let packet = packet();
    let row = packet
        .lane(ExecutionLane::ProfilerCapture)
        .expect("profiler-capture row");
    assert_eq!(row.connection_state, ConnectionState::Stale);
    assert_eq!(row.published_attribution, AttributionConfidence::Stale);
    assert_eq!(row.boundary_decision, BoundaryDecision::Narrow);
    assert!(row
        .narrowing_reasons
        .contains(&HostNarrowingReason::StaleContext));
}

#[test]
fn missing_receipt_origin_is_withheld() {
    let packet = packet();
    let row = packet
        .lane(ExecutionLane::ServicePlaneAction)
        .expect("service-plane-action row");
    assert!(row.origin_receipt_state.is_missing_trigger());
    assert_eq!(
        row.published_attribution,
        AttributionConfidence::Unattributed
    );
    assert_eq!(row.boundary_decision, BoundaryDecision::Withhold);
    assert!(row
        .narrowing_reasons
        .contains(&HostNarrowingReason::MissingOriginReceipt));
}

#[test]
fn validate_flags_locus_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .lanes
        .iter_mut()
        .find(|l| l.host_kind.crosses_boundary())
    {
        row.published_locus = OriginLocus::Local;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5HostBoundaryViolation::LocusMismatch { .. })));
    }
}

#[test]
fn validate_flags_overstated_attribution() {
    let mut packet = packet();
    if let Some(row) = packet
        .lanes
        .iter_mut()
        .find(|l| l.effective_attribution() != AttributionConfidence::Confirmed)
    {
        row.published_attribution = AttributionConfidence::Confirmed;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5HostBoundaryViolation::OverstatedAttribution { .. })));
    }
}

#[test]
fn validate_flags_decision_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .lanes
        .iter_mut()
        .find(|l| l.boundary_decision != BoundaryDecision::Withhold)
    {
        row.boundary_decision = BoundaryDecision::Withhold;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5HostBoundaryViolation::DecisionMismatch { .. })));
    }
}

#[test]
fn validate_flags_narrowing_reasons_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet.lanes.iter_mut().find(|l| {
        !l.narrowing_reasons
            .contains(&HostNarrowingReason::ExportContinuityBroken)
    }) {
        row.narrowing_reasons
            .push(HostNarrowingReason::ExportContinuityBroken);
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5HostBoundaryViolation::NarrowingReasonsMismatch { .. })));
    }
}

#[test]
fn validate_flags_missing_lane_row() {
    let mut packet = packet();
    let removed = packet.lanes.pop();
    assert!(removed.is_some());
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5HostBoundaryViolation::MissingLaneRow { .. })));
}

#[test]
fn validate_flags_unclaimed_lane_row() {
    let mut packet = packet();
    packet
        .execution_lanes
        .retain(|l| *l != ExecutionLane::ServicePlaneAction);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5HostBoundaryViolation::UnclaimedLaneRow { .. })));
    assert!(violations.iter().any(|v| matches!(
        v,
        M5HostBoundaryViolation::ClosedVocabularyMismatch {
            field: "execution_lanes"
        }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_lanes = packet.summary.total_lanes.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5HostBoundaryViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(ExecutionLane::NotebookRun.as_str(), "notebook_run");
    assert_eq!(
        ExecutionLane::ServicePlaneAction.as_str(),
        "service_plane_action"
    );
    assert_eq!(HostKind::Local.as_str(), "local");
    assert_eq!(HostKind::BrowserBridge.as_str(), "browser_bridge");
    assert_eq!(HostKind::ServicePlane.as_str(), "service_plane");
    assert_eq!(OriginLocus::Bridged.as_str(), "bridged");
    assert_eq!(AttributionConfidence::Confirmed.as_str(), "confirmed");
    assert_eq!(AttributionConfidence::Unattributed.as_str(), "unattributed");
    assert_eq!(OriginReceiptState::Missing.as_str(), "missing");
    assert_eq!(ConnectionState::Bridged.as_str(), "bridged");
    assert_eq!(HostBindingState::Rebound.as_str(), "rebound");
    assert_eq!(ExportContinuityState::Broken.as_str(), "broken");
    assert_eq!(
        HostNarrowingReason::MissingOriginReceipt.as_str(),
        "missing_origin_receipt"
    );
    assert_eq!(BoundaryDecision::FlagForReview.as_str(), "flag_for_review");
}

#[test]
fn attribution_rank_orders_low_to_high() {
    assert!(AttributionConfidence::Unattributed.rank() < AttributionConfidence::Stale.rank());
    assert!(AttributionConfidence::Stale.rank() < AttributionConfidence::Provisional.rank());
    assert!(AttributionConfidence::Provisional.rank() < AttributionConfidence::Attributed.rank());
    assert!(AttributionConfidence::Attributed.rank() < AttributionConfidence::Confirmed.rank());
    assert_eq!(
        AttributionConfidence::Confirmed.min(AttributionConfidence::Stale),
        AttributionConfidence::Stale
    );
}
