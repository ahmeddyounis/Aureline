use super::*;

fn packet() -> M5TargetDiscoveryMatrix {
    current_m5_target_discovery_matrix().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_TARGET_DISCOVERY_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_TARGET_DISCOVERY_RECORD_KIND);
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
    assert_eq!(packet.lanes.len(), packet.discovery_lanes.len());
    for &lane in &packet.discovery_lanes {
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
            row.published_confidence,
            row.effective_confidence(),
            "lane {} publishes beyond the gate",
            row.lane_id
        );
        assert_eq!(
            row.exactness,
            row.derived_exactness(),
            "lane {} exactness diverges from the gate",
            row.lane_id
        );
        assert_eq!(
            row.discovery_decision,
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
fn changed_lanes_carry_a_reviewable_diff() {
    let packet = packet();
    for row in &packet.lanes {
        if row.change_trigger.is_change() {
            assert!(
                row.diff_review_state.requires_change(),
                "lane {} changed but carries no diff-review state",
                row.lane_id
            );
            assert!(
                !row.previous_target_ref.trim().is_empty(),
                "lane {} changed but has no previous-target ref",
                row.lane_id
            );
            assert!(
                !row.discovery_diff_ref.trim().is_empty(),
                "lane {} changed but has no discovery-diff ref",
                row.lane_id
            );
        } else {
            assert_eq!(
                row.diff_review_state,
                DiffReviewState::NotApplicable,
                "lane {} is unchanged but records a diff-review state",
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
        assert_eq!(export.exact_target, row.is_publishable());
    }
}

#[test]
fn published_confidences_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<DiscoveryConfidence> = packet
        .lanes
        .iter()
        .map(|l| l.published_confidence)
        .collect();
    for confidence in DiscoveryConfidence::ALL {
        assert!(
            present.contains(&confidence),
            "no lane publishes confidence {}",
            confidence.as_str()
        );
    }
}

#[test]
fn discovery_decisions_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<DiscoveryDecision> =
        packet.lanes.iter().map(|l| l.discovery_decision).collect();
    for decision in DiscoveryDecision::ALL {
        assert!(
            present.contains(&decision),
            "no lane exercises decision {}",
            decision.as_str()
        );
    }
}

#[test]
fn discovery_paths_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<DiscoveryPath> = packet.lanes.iter().map(|l| l.discovery_path).collect();
    for path in DiscoveryPath::ALL {
        assert!(
            present.contains(&path),
            "no lane exercises discovery path {}",
            path.as_str()
        );
    }
}

#[test]
fn verification_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<VerificationState> =
        packet.lanes.iter().map(|l| l.verification_state).collect();
    for state in VerificationState::ALL {
        assert!(
            present.contains(&state),
            "no lane exercises verification state {}",
            state.as_str()
        );
    }
}

#[test]
fn exactness_labels_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<Exactness> = packet.lanes.iter().map(|l| l.exactness).collect();
    for label in Exactness::ALL {
        assert!(
            present.contains(&label),
            "no lane exercises exactness {}",
            label.as_str()
        );
    }
}

#[test]
fn change_triggers_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ChangeTrigger> = packet.lanes.iter().map(|l| l.change_trigger).collect();
    for trigger in ChangeTrigger::ALL {
        assert!(
            present.contains(&trigger),
            "no lane exercises change trigger {}",
            trigger.as_str()
        );
    }
}

#[test]
fn diff_review_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<DiffReviewState> =
        packet.lanes.iter().map(|l| l.diff_review_state).collect();
    for state in DiffReviewState::ALL {
        assert!(
            present.contains(&state),
            "no lane exercises diff-review state {}",
            state.as_str()
        );
    }
}

#[test]
fn target_graph_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<TargetGraphState> =
        packet.lanes.iter().map(|l| l.target_graph_state).collect();
    for state in TargetGraphState::ALL {
        assert!(
            present.contains(&state),
            "no lane exercises target-graph state {}",
            state.as_str()
        );
    }
}

#[test]
fn provenance_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ProvenanceState> =
        packet.lanes.iter().map(|l| l.provenance_state).collect();
    for state in ProvenanceState::ALL {
        assert!(
            present.contains(&state),
            "no lane exercises provenance state {}",
            state.as_str()
        );
    }
}

#[test]
fn narrowing_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<NarrowingReason> = packet
        .lanes
        .iter()
        .flat_map(|l| l.narrowing_reasons.iter().copied())
        .collect();
    for reason in NarrowingReason::ALL {
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
        "fixture needs an exact lane"
    );
    for row in packet.publishable_lanes() {
        assert_eq!(row.capability_floor(), DiscoveryConfidence::Exact);
        assert!(row.narrowing_reasons.is_empty());
        assert_eq!(row.published_confidence, DiscoveryConfidence::Exact);
        assert_eq!(row.exactness, Exactness::Exact);
        assert_eq!(row.discovery_decision, DiscoveryDecision::Publish);
    }
}

#[test]
fn ceilings_hold_for_each_state() {
    assert_eq!(
        DiscoveryPath::Undiscovered.confidence_ceiling(),
        DiscoveryConfidence::Unresolved
    );
    assert_eq!(
        DiscoveryPath::Heuristic.confidence_ceiling(),
        DiscoveryConfidence::Heuristic
    );
    assert_eq!(
        DiscoveryPath::BuildEventStream.confidence_ceiling(),
        DiscoveryConfidence::Structured
    );
    assert_eq!(
        VerificationState::Unverified.confidence_ceiling(),
        DiscoveryConfidence::Heuristic
    );
    assert_eq!(
        VerificationState::SingleSignal.confidence_ceiling(),
        DiscoveryConfidence::Structured
    );
    assert_eq!(
        DiffReviewState::AutoAppliedUnreviewed.confidence_ceiling(),
        DiscoveryConfidence::Heuristic
    );
    assert_eq!(
        DiffReviewState::ReviewedRejected.confidence_ceiling(),
        DiscoveryConfidence::Unresolved
    );
    assert_eq!(
        ProvenanceState::Dropped.confidence_ceiling(),
        DiscoveryConfidence::Heuristic
    );
    assert_eq!(
        TargetGraphState::MissingSnapshot.confidence_ceiling(),
        DiscoveryConfidence::Imported
    );
}

#[test]
fn unverified_native_target_never_reads_as_exact() {
    // The guardrail: an approximate or unverified target must not masquerade as a
    // confident exact native target merely because it produced a runnable fallback.
    let packet = packet();
    let row = packet
        .lane(DiscoveryLane::ApiRuntime)
        .expect("api-runtime row");
    assert_eq!(row.discovery_path, DiscoveryPath::NativeAdapter);
    assert_eq!(row.declared_confidence, DiscoveryConfidence::Exact);
    assert!(row.verification_state.is_low_trigger());
    assert!(row.published_confidence.rank() < DiscoveryConfidence::Exact.rank());
    assert_eq!(row.exactness, Exactness::Approximate);
    assert!(row
        .narrowing_reasons
        .contains(&NarrowingReason::LowVerification));
}

#[test]
fn unreviewed_target_change_is_flagged_not_silent() {
    let packet = packet();
    let row = packet
        .lane(DiscoveryLane::RequestRuntime)
        .expect("request-runtime row");
    assert!(row.change_trigger.is_change());
    assert_eq!(
        row.diff_review_state,
        DiffReviewState::AutoAppliedUnreviewed
    );
    assert_eq!(row.discovery_decision, DiscoveryDecision::FlagForReview);
    assert!(row
        .narrowing_reasons
        .contains(&NarrowingReason::UnreviewedTargetChange));
}

#[test]
fn undiscovered_target_is_withheld() {
    let packet = packet();
    let row = packet
        .lane(DiscoveryLane::IncidentRerun)
        .expect("incident-rerun row");
    assert!(row.discovery_path.is_unresolved_trigger());
    assert_eq!(row.published_confidence, DiscoveryConfidence::Unresolved);
    assert_eq!(row.discovery_decision, DiscoveryDecision::Withhold);
    assert!(row
        .narrowing_reasons
        .contains(&NarrowingReason::TargetUnresolved));
}

#[test]
fn validate_flags_overstated_confidence() {
    let mut packet = packet();
    if let Some(row) = packet
        .lanes
        .iter_mut()
        .find(|l| l.effective_confidence() != DiscoveryConfidence::Exact)
    {
        row.published_confidence = DiscoveryConfidence::Exact;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5TargetDiscoveryViolation::OverstatedConfidence { .. })));
    }
}

#[test]
fn validate_flags_exactness_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .lanes
        .iter_mut()
        .find(|l| l.exactness == Exactness::Approximate)
    {
        row.exactness = Exactness::Exact;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5TargetDiscoveryViolation::ExactnessMismatch { .. })));
    }
}

#[test]
fn validate_flags_decision_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .lanes
        .iter_mut()
        .find(|l| l.discovery_decision != DiscoveryDecision::Withhold)
    {
        row.discovery_decision = DiscoveryDecision::Withhold;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5TargetDiscoveryViolation::DecisionMismatch { .. })));
    }
}

#[test]
fn validate_flags_narrowing_reasons_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet.lanes.iter_mut().find(|l| {
        !l.narrowing_reasons
            .contains(&NarrowingReason::ProvenanceDropped)
    }) {
        row.narrowing_reasons
            .push(NarrowingReason::ProvenanceDropped);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5TargetDiscoveryViolation::NarrowingReasonsMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_change_review_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .lanes
        .iter_mut()
        .find(|l| l.change_trigger == ChangeTrigger::Unchanged)
    {
        row.change_trigger = ChangeTrigger::WorkspaceChanged;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5TargetDiscoveryViolation::ChangeReviewMismatch { .. })));
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
        .any(|v| matches!(v, M5TargetDiscoveryViolation::MissingLaneRow { .. })));
}

#[test]
fn validate_flags_unclaimed_lane_row() {
    let mut packet = packet();
    packet
        .discovery_lanes
        .retain(|l| *l != DiscoveryLane::IncidentRerun);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5TargetDiscoveryViolation::UnclaimedLaneRow { .. })));
    assert!(violations.iter().any(|v| matches!(
        v,
        M5TargetDiscoveryViolation::ClosedVocabularyMismatch {
            field: "discovery_lanes"
        }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_lanes = packet.summary.total_lanes.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5TargetDiscoveryViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(DiscoveryLane::BuildTarget.as_str(), "build_target");
    assert_eq!(DiscoveryLane::IncidentRerun.as_str(), "incident_rerun");
    assert_eq!(DiscoveryConfidence::Exact.as_str(), "exact");
    assert_eq!(DiscoveryConfidence::Unresolved.as_str(), "unresolved");
    assert_eq!(DiscoveryPath::NativeAdapter.as_str(), "native_adapter");
    assert_eq!(DiscoveryPath::Undiscovered.as_str(), "undiscovered");
    assert_eq!(VerificationState::Unverified.as_str(), "unverified");
    assert_eq!(Exactness::Approximate.as_str(), "approximate");
    assert_eq!(
        DiffReviewState::AutoAppliedUnreviewed.as_str(),
        "auto_applied_unreviewed"
    );
    assert_eq!(
        NarrowingReason::UnreviewedTargetChange.as_str(),
        "unreviewed_target_change"
    );
    assert_eq!(DiscoveryDecision::FlagForReview.as_str(), "flag_for_review");
}

#[test]
fn confidence_rank_orders_low_to_high() {
    assert!(DiscoveryConfidence::Unresolved.rank() < DiscoveryConfidence::Heuristic.rank());
    assert!(DiscoveryConfidence::Heuristic.rank() < DiscoveryConfidence::Imported.rank());
    assert!(DiscoveryConfidence::Imported.rank() < DiscoveryConfidence::Structured.rank());
    assert!(DiscoveryConfidence::Structured.rank() < DiscoveryConfidence::Exact.rank());
    assert_eq!(
        DiscoveryConfidence::Exact.min(DiscoveryConfidence::Heuristic),
        DiscoveryConfidence::Heuristic
    );
    assert_eq!(DiscoveryConfidence::Exact.exactness(), Exactness::Exact);
    assert_eq!(
        DiscoveryConfidence::Structured.exactness(),
        Exactness::Approximate
    );
}
