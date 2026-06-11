use super::*;

fn packet() -> M5BuildAndHostGovernanceMatrix {
    current_m5_build_and_host_governance_matrix().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_BUILD_AND_HOST_GOVERNANCE_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_BUILD_AND_HOST_GOVERNANCE_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_claimed_surface_has_exactly_one_row() {
    let packet = packet();
    assert_eq!(packet.surfaces.len(), packet.execution_surfaces.len());
    for &surface in &packet.execution_surfaces {
        assert!(
            packet.surface(surface).is_some(),
            "missing row for surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn every_surface_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_surfaces_gate_consistent());
    for row in &packet.surfaces {
        assert_eq!(
            row.published_claim,
            row.effective_claim(),
            "surface {} publishes beyond the gate",
            row.surface_id
        );
        assert_eq!(
            row.claim_decision,
            row.required_decision(),
            "surface {} decision diverges from the gate",
            row.surface_id
        );
        assert_eq!(
            row.narrowing_reasons,
            row.computed_narrowing_reasons(),
            "surface {} narrowing reasons diverge from the gate",
            row.surface_id
        );
    }
}

#[test]
fn every_surface_carries_its_own_evidence() {
    let packet = packet();
    for row in &packet.surfaces {
        assert!(
            row.has_required_evidence(),
            "surface {} is missing required evidence refs",
            row.surface_id
        );
        assert!(
            !row.target_identity_ref.trim().is_empty(),
            "surface {} has no target-identity ref",
            row.surface_id
        );
    }
}

#[test]
fn export_projection_reflects_rows_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.surfaces.len(), packet.surfaces.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_surfaces_gate_consistent,
        packet.all_surfaces_gate_consistent()
    );
    assert_eq!(
        projection.publishable_count,
        packet.publishable_surfaces().count()
    );
    assert_eq!(
        projection.narrowed_count,
        packet.narrowed_surfaces().count()
    );
    assert_eq!(
        projection.withheld_count,
        packet.withheld_surfaces().count()
    );
}

#[test]
fn published_claims_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ExecutionClaim> =
        packet.surfaces.iter().map(|s| s.published_claim).collect();
    for claim in ExecutionClaim::ALL {
        assert!(
            present.contains(&claim),
            "no surface publishes claim {}",
            claim.as_str()
        );
    }
}

#[test]
fn claim_decisions_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ClaimDecision> =
        packet.surfaces.iter().map(|s| s.claim_decision).collect();
    for decision in ClaimDecision::ALL {
        assert!(
            present.contains(&decision),
            "no surface exercises decision {}",
            decision.as_str()
        );
    }
}

#[test]
fn target_discovery_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<TargetDiscoveryClass> = packet
        .surfaces
        .iter()
        .map(|s| s.target_discovery_class)
        .collect();
    for class in TargetDiscoveryClass::ALL {
        assert!(
            present.contains(&class),
            "no surface exercises target discovery {}",
            class.as_str()
        );
    }
}

#[test]
fn adapter_confidences_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<AdapterConfidence> = packet
        .surfaces
        .iter()
        .map(|s| s.adapter_confidence)
        .collect();
    for confidence in AdapterConfidence::ALL {
        assert!(
            present.contains(&confidence),
            "no surface exercises adapter confidence {}",
            confidence.as_str()
        );
    }
}

#[test]
fn host_boundaries_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<HostBoundary> = packet.surfaces.iter().map(|s| s.host_boundary).collect();
    for boundary in HostBoundary::ALL {
        assert!(
            present.contains(&boundary),
            "no surface exercises host boundary {}",
            boundary.as_str()
        );
    }
}

#[test]
fn control_plane_ownerships_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ControlPlaneOwnership> = packet
        .surfaces
        .iter()
        .map(|s| s.control_plane_ownership)
        .collect();
    for ownership in ControlPlaneOwnership::ALL {
        assert!(
            present.contains(&ownership),
            "no surface exercises control-plane ownership {}",
            ownership.as_str()
        );
    }
}

#[test]
fn managed_workspace_lifecycles_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ManagedWorkspaceLifecycle> = packet
        .surfaces
        .iter()
        .map(|s| s.managed_workspace_lifecycle)
        .collect();
    for state in ManagedWorkspaceLifecycle::ALL {
        assert!(
            present.contains(&state),
            "no surface exercises managed-workspace lifecycle {}",
            state.as_str()
        );
    }
}

#[test]
fn mutation_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<MutationClass> =
        packet.surfaces.iter().map(|s| s.mutation_class).collect();
    for class in MutationClass::ALL {
        assert!(
            present.contains(&class),
            "no surface exercises mutation class {}",
            class.as_str()
        );
    }
}

#[test]
fn approval_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ApprovalState> =
        packet.surfaces.iter().map(|s| s.approval_state).collect();
    for state in ApprovalState::ALL {
        assert!(
            present.contains(&state),
            "no surface exercises approval state {}",
            state.as_str()
        );
    }
}

#[test]
fn freshness_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<EvidenceFreshness> = packet
        .surfaces
        .iter()
        .map(|s| s.evidence_freshness)
        .collect();
    for freshness in EvidenceFreshness::ALL {
        assert!(
            present.contains(&freshness),
            "no surface exercises freshness {}",
            freshness.as_str()
        );
    }
}

#[test]
fn rollback_postures_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<RollbackPosture> =
        packet.surfaces.iter().map(|s| s.rollback_posture).collect();
    for posture in RollbackPosture::ALL {
        assert!(
            present.contains(&posture),
            "no surface exercises rollback posture {}",
            posture.as_str()
        );
    }
}

#[test]
fn persistence_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<PersistenceClass> = packet
        .surfaces
        .iter()
        .map(|s| s.persistence_class)
        .collect();
    for class in PersistenceClass::ALL {
        assert!(
            present.contains(&class),
            "no surface exercises persistence class {}",
            class.as_str()
        );
    }
}

#[test]
fn expiry_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ExpiryClass> = packet.surfaces.iter().map(|s| s.expiry_class).collect();
    for class in ExpiryClass::ALL {
        assert!(
            present.contains(&class),
            "no surface exercises expiry class {}",
            class.as_str()
        );
    }
}

#[test]
fn narrowing_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<NarrowingReason> = packet
        .surfaces
        .iter()
        .flat_map(|s| s.narrowing_reasons.iter().copied())
        .collect();
    for reason in NarrowingReason::ALL {
        assert!(
            present.contains(&reason),
            "no surface exercises narrowing reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn publishable_surfaces_are_clean() {
    let packet = packet();
    assert!(
        packet.publishable_surfaces().count() > 0,
        "fixture needs an authoritative surface"
    );
    for row in packet.publishable_surfaces() {
        assert!(row.evidence_freshness.is_current());
        assert_eq!(row.capability_floor(), ExecutionClaim::Authoritative);
        assert!(row.narrowing_reasons.is_empty());
        assert_eq!(row.published_claim, ExecutionClaim::Authoritative);
        assert_eq!(row.claim_decision, ClaimDecision::Publish);
    }
}

#[test]
fn ceilings_hold_for_each_state() {
    assert_eq!(
        TargetDiscoveryClass::Undiscovered.claim_ceiling(),
        ExecutionClaim::Withheld
    );
    assert_eq!(
        TargetDiscoveryClass::UserSupplied.claim_ceiling(),
        ExecutionClaim::Provisional
    );
    assert_eq!(
        AdapterConfidence::Unverified.claim_ceiling(),
        ExecutionClaim::Provisional
    );
    assert_eq!(
        AdapterConfidence::Heuristic.claim_ceiling(),
        ExecutionClaim::Qualified
    );
    assert_eq!(
        HostBoundary::UnboundHost.claim_ceiling(),
        ExecutionClaim::Withheld
    );
    assert_eq!(
        HostBoundary::BridgedHost.claim_ceiling(),
        ExecutionClaim::Provisional
    );
    assert_eq!(
        ControlPlaneOwnership::UnknownOwner.claim_ceiling(),
        ExecutionClaim::Withheld
    );
    assert_eq!(
        ControlPlaneOwnership::ExternalOwned.claim_ceiling(),
        ExecutionClaim::Qualified
    );
    assert_eq!(
        ManagedWorkspaceLifecycle::Terminated.claim_ceiling(),
        ExecutionClaim::Withheld
    );
    assert_eq!(
        MutationClass::DestructiveApply.claim_ceiling(),
        ExecutionClaim::Withheld
    );
    assert_eq!(
        MutationClass::IrreversibleApply.claim_ceiling(),
        ExecutionClaim::Provisional
    );
    assert_eq!(
        ApprovalState::Bypassed.claim_ceiling(),
        ExecutionClaim::Withheld
    );
    assert_eq!(
        EvidenceFreshness::Expired.claim_ceiling(),
        ExecutionClaim::Provisional
    );
    assert_eq!(
        RollbackPosture::Irreversible.claim_ceiling(),
        ExecutionClaim::Provisional
    );
}

#[test]
fn validate_flags_overstated_published_claim() {
    let mut packet = packet();
    if let Some(row) = packet
        .surfaces
        .iter_mut()
        .find(|s| s.effective_claim() != ExecutionClaim::Authoritative)
    {
        row.published_claim = ExecutionClaim::Authoritative;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5BuildAndHostGovernanceViolation::OverstatedPublishedClaim { .. }
        )));
    }
}

#[test]
fn validate_flags_decision_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .surfaces
        .iter_mut()
        .find(|s| s.claim_decision != ClaimDecision::Withhold)
    {
        row.claim_decision = ClaimDecision::Withhold;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5BuildAndHostGovernanceViolation::DecisionMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_narrowing_reasons_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .surfaces
        .iter_mut()
        .find(|s| !s.narrowing_reasons.contains(&NarrowingReason::HostUnbound))
    {
        row.narrowing_reasons.push(NarrowingReason::HostUnbound);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5BuildAndHostGovernanceViolation::NarrowingReasonsMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_surface_row() {
    let mut packet = packet();
    let removed = packet.surfaces.pop();
    assert!(removed.is_some());
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5BuildAndHostGovernanceViolation::MissingSurfaceRow { .. }
    )));
}

#[test]
fn validate_flags_unclaimed_surface_row() {
    let mut packet = packet();
    packet
        .execution_surfaces
        .retain(|s| *s != ExecutionSurface::IncidentReplayTarget);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5BuildAndHostGovernanceViolation::UnclaimedSurfaceRow { .. }
    )));
    assert!(violations.iter().any(|v| matches!(
        v,
        M5BuildAndHostGovernanceViolation::ClosedVocabularyMismatch {
            field: "execution_surfaces"
        }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_surfaces = packet.summary.total_surfaces.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5BuildAndHostGovernanceViolation::SummaryMismatch));
}

#[test]
fn withheld_surfaces_carry_no_claim() {
    let packet = packet();
    let withheld: Vec<_> = packet.withheld_surfaces().collect();
    assert!(!withheld.is_empty(), "fixture needs a withheld surface");
    for row in withheld {
        assert_eq!(row.published_claim, ExecutionClaim::Withheld);
        assert_eq!(row.claim_decision, ClaimDecision::Withhold);
    }
}

#[test]
fn live_resource_mutation_never_outruns_preview_or_rollback() {
    let packet = packet();
    let row = packet
        .surface(ExecutionSurface::LiveResourceTarget)
        .expect("live-resource row");
    assert!(row.mutation_class.is_unsafe_trigger());
    assert!(row.approval_state.is_bypassed_trigger());
    assert_eq!(row.published_claim, ExecutionClaim::Withheld);
    assert!(row
        .narrowing_reasons
        .contains(&NarrowingReason::UnsafeMutation));
    assert!(row
        .narrowing_reasons
        .contains(&NarrowingReason::ApprovalBypassed));
}

#[test]
fn undiscovered_target_never_implies_runs_here() {
    let packet = packet();
    let row = packet
        .surface(ExecutionSurface::IncidentReplayTarget)
        .expect("incident-replay row");
    assert!(row.target_discovery_class.is_undiscovered_trigger());
    assert!(row.host_boundary.is_unbound_trigger());
    assert!(row.published_claim.rank() < ExecutionClaim::Authoritative.rank());
    assert!(row
        .narrowing_reasons
        .contains(&NarrowingReason::TargetUndiscovered));
    assert!(row
        .narrowing_reasons
        .contains(&NarrowingReason::HostUnbound));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(
        ExecutionSurface::LocalBuildTarget.as_str(),
        "local_build_target"
    );
    assert_eq!(
        ExecutionSurface::IncidentReplayTarget.as_str(),
        "incident_replay_target"
    );
    assert_eq!(ExecutionClaim::Authoritative.as_str(), "authoritative");
    assert_eq!(TargetDiscoveryClass::Undiscovered.as_str(), "undiscovered");
    assert_eq!(HostBoundary::UnboundHost.as_str(), "unbound_host");
    assert_eq!(
        MutationClass::DestructiveApply.as_str(),
        "destructive_apply"
    );
    assert_eq!(
        NarrowingReason::ControlPlaneUnknown.as_str(),
        "control_plane_unknown"
    );
    assert_eq!(ClaimDecision::Withhold.as_str(), "withhold");
}

#[test]
fn claim_rank_orders_low_to_high() {
    assert!(ExecutionClaim::Withheld.rank() < ExecutionClaim::Provisional.rank());
    assert!(ExecutionClaim::Provisional.rank() < ExecutionClaim::Qualified.rank());
    assert!(ExecutionClaim::Qualified.rank() < ExecutionClaim::Authoritative.rank());
    assert_eq!(
        ExecutionClaim::Authoritative.min(ExecutionClaim::Qualified),
        ExecutionClaim::Qualified
    );
}
