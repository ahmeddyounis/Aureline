use super::*;

fn packet() -> M5GraphGovernanceMatrix {
    current_m5_graph_governance_matrix().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_GRAPH_GOVERNANCE_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_GRAPH_GOVERNANCE_RECORD_KIND);
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
    assert_eq!(packet.lane_rows.len(), packet.lanes.len());
    for &lane in &packet.lanes {
        assert!(
            packet.lane_row(lane).is_some(),
            "missing row for lane {}",
            lane.as_str()
        );
    }
}

#[test]
fn every_lane_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_lanes_gate_consistent());
    for row in &packet.lane_rows {
        assert_eq!(
            row.published_claim,
            row.effective_claim(),
            "lane {} publishes beyond the gate",
            row.lane_id
        );
        assert_eq!(
            row.governance_decision,
            row.required_decision(),
            "lane {} decision diverges from the gate",
            row.lane_id
        );
        assert_eq!(
            row.downgrade_reasons,
            row.computed_downgrade_reasons(),
            "lane {} downgrade reasons diverge from the gate",
            row.lane_id
        );
    }
}

#[test]
fn every_lane_carries_its_own_evidence() {
    let packet = packet();
    for row in &packet.lane_rows {
        assert!(
            row.has_required_evidence(),
            "lane {} is missing required evidence refs",
            row.lane_id
        );
    }
}

#[test]
fn every_lane_binds_to_its_canonical_source_packet() {
    let packet = packet();
    for row in &packet.lane_rows {
        assert_eq!(
            row.packet_ref,
            row.lane.source_packet(),
            "lane {} governs a packet other than its canonical source",
            row.lane_id
        );
    }
}

#[test]
fn narrowed_lanes_offer_a_recovery_and_caveats() {
    let packet = packet();
    for row in &packet.lane_rows {
        if row.governance_decision.is_narrowed() {
            assert!(
                row.downgrade_path.is_offered(),
                "narrowed lane {} must offer a recovery path",
                row.lane_id
            );
            assert!(
                !row.caveats.is_empty(),
                "narrowed lane {} must list a caveat",
                row.lane_id
            );
            assert!(
                !row.stale_or_missing_fields.is_empty(),
                "narrowed lane {} must name a stale or narrowing field",
                row.lane_id
            );
        }
    }
}

#[test]
fn export_projection_reflects_rows_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.lanes.len(), packet.lane_rows.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_lanes_gate_consistent,
        packet.all_lanes_gate_consistent()
    );
    assert_eq!(
        projection.authoritative_count,
        packet.authoritative_lanes().count()
    );
    assert_eq!(projection.narrowed_count, packet.narrowed_lanes().count());
    assert_eq!(projection.withheld_count, packet.withheld_lanes().count());
    for (row, export) in packet.lane_rows.iter().zip(projection.lanes.iter()) {
        assert_eq!(export.packet_ref, row.packet_ref);
        assert_eq!(export.authoritative, row.is_authoritative());
        assert_eq!(export.downgraded, row.is_downgraded());
        assert_eq!(export.scope_sensitive, row.lane.is_scope_sensitive());
        assert_eq!(export.published_claim, row.published_claim.as_str());
        assert_eq!(export.hidden_result_count, row.hidden_result_count);
        assert_eq!(export.out_of_scope_count, row.out_of_scope_count);
    }
}

#[test]
fn published_claims_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<GraphDepthClaim> =
        packet.lane_rows.iter().map(|c| c.published_claim).collect();
    for claim in GraphDepthClaim::ALL {
        assert!(
            present.contains(&claim),
            "no lane publishes claim {}",
            claim.as_str()
        );
    }
}

#[test]
fn governance_decisions_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<GraphGovernanceDecision> = packet
        .lane_rows
        .iter()
        .map(|c| c.governance_decision)
        .collect();
    for decision in GraphGovernanceDecision::ALL {
        assert!(
            present.contains(&decision),
            "no lane exercises decision {}",
            decision.as_str()
        );
    }
}

#[test]
fn scope_modes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ScopeMode> = packet.lane_rows.iter().map(|c| c.scope_mode).collect();
    for mode in ScopeMode::ALL {
        assert!(
            present.contains(&mode),
            "no lane exercises scope mode {}",
            mode.as_str()
        );
    }
}

#[test]
fn graph_freshness_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<GraphFreshness> =
        packet.lane_rows.iter().map(|c| c.graph_freshness).collect();
    for state in GraphFreshness::ALL {
        assert!(
            present.contains(&state),
            "no lane exercises freshness {}",
            state.as_str()
        );
    }
}

#[test]
fn relation_fidelities_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<RelationFidelity> = packet
        .lane_rows
        .iter()
        .map(|c| c.relation_fidelity)
        .collect();
    for fidelity in RelationFidelity::ALL {
        assert!(
            present.contains(&fidelity),
            "no lane exercises fidelity {}",
            fidelity.as_str()
        );
    }
}

#[test]
fn evidence_backings_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<EvidenceBacking> = packet
        .lane_rows
        .iter()
        .map(|c| c.evidence_backing)
        .collect();
    for backing in EvidenceBacking::ALL {
        assert!(
            present.contains(&backing),
            "no lane exercises backing {}",
            backing.as_str()
        );
    }
}

#[test]
fn impact_result_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ImpactResultClass> = packet
        .lane_rows
        .iter()
        .map(|c| c.impact_result_class)
        .collect();
    for class in ImpactResultClass::ALL {
        assert!(
            present.contains(&class),
            "no lane exercises impact class {}",
            class.as_str()
        );
    }
}

#[test]
fn downgrade_paths_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<DowngradePath> =
        packet.lane_rows.iter().map(|c| c.downgrade_path).collect();
    for path in DowngradePath::ALL {
        assert!(
            present.contains(&path),
            "no lane exercises recovery path {}",
            path.as_str()
        );
    }
}

#[test]
fn downgrade_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<DowngradeReason> = packet
        .lane_rows
        .iter()
        .flat_map(|c| c.downgrade_reasons.iter().copied())
        .collect();
    for reason in DowngradeReason::ALL {
        assert!(
            present.contains(&reason),
            "no lane exercises downgrade reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn authoritative_lanes_are_whole() {
    let packet = packet();
    assert!(
        packet.authoritative_lanes().count() >= 2,
        "fixture needs at least two authoritative lanes to prove the gate is not a blanket downgrade"
    );
    for row in packet.authoritative_lanes() {
        assert_eq!(row.capability_floor(), GraphDepthClaim::Authoritative);
        assert_eq!(row.scope_mode, ScopeMode::FullWorkspace);
        assert!(row.downgrade_reasons.is_empty());
        assert!(row.caveats.is_empty());
        assert!(row.stale_or_missing_fields.is_empty());
        assert!(!row.downgrade_path.is_offered());
        assert!(!row.supported_scopes.is_empty());
        assert!(!row.is_downgraded());
        assert_eq!(row.hidden_result_count, 0);
        assert_eq!(row.out_of_scope_count, 0);
    }
}

#[test]
fn ceilings_hold_for_each_state() {
    assert_eq!(
        ScopeMode::Workset.claim_ceiling(),
        GraphDepthClaim::ScopeQualified
    );
    assert_eq!(
        ScopeMode::Unscoped.claim_ceiling(),
        GraphDepthClaim::Withheld
    );
    assert_eq!(
        GraphFreshness::Stale.claim_ceiling(),
        GraphDepthClaim::Provisional
    );
    assert_eq!(
        GraphFreshness::Expired.claim_ceiling(),
        GraphDepthClaim::Withheld
    );
    assert_eq!(
        RelationFidelity::Approximate.claim_ceiling(),
        GraphDepthClaim::Provisional
    );
    assert_eq!(
        RelationFidelity::Unresolved.claim_ceiling(),
        GraphDepthClaim::Withheld
    );
    assert_eq!(
        EvidenceBacking::Generated.claim_ceiling(),
        GraphDepthClaim::Provisional
    );
    assert_eq!(
        EvidenceBacking::Uncited.claim_ceiling(),
        GraphDepthClaim::Withheld
    );
}

#[test]
fn workset_lane_is_qualified_not_left_authoritative() {
    // A workset-only lane is narrowed to its slice rather than implying whole-workspace
    // certainty.
    let packet = packet();
    let row = packet
        .lane_row(GraphDepthLane::WorksetScope)
        .expect("workset-scope row");
    assert!(row.scope_mode.is_narrow_trigger());
    assert!(row.is_downgraded());
    assert!(row.lane.is_scope_sensitive());
    assert!(row
        .downgrade_reasons
        .contains(&DowngradeReason::ScopeNarrowed));
    assert_eq!(row.published_claim, GraphDepthClaim::ScopeQualified);
    assert!(row.out_of_scope_count > 0);
}

#[test]
fn clean_lane_publishes() {
    // A full-workspace, fresh, exact, curated lane publishes authoritative — the gate is not
    // a blanket downgrade.
    let packet = packet();
    let row = packet
        .lane_row(GraphDepthLane::GraphTopology)
        .expect("graph-topology row");
    assert_eq!(row.published_claim, GraphDepthClaim::Authoritative);
    assert_eq!(row.governance_decision, GraphGovernanceDecision::Publish);
    assert!(row.downgrade_reasons.is_empty());
}

#[test]
fn dead_recall_is_withheld() {
    // An unscoped, expired, unresolved, uncited lane drops to withheld rather than implying
    // whole-workspace recall.
    let packet = packet();
    let row = packet
        .lane_row(GraphDepthLane::NavigationRecall)
        .expect("navigation-recall row");
    assert_eq!(row.published_claim, GraphDepthClaim::Withheld);
    assert_eq!(row.governance_decision, GraphGovernanceDecision::Withhold);
    assert!(row.lane.is_scope_sensitive());
    assert!(row.downgrade_path.is_offered());
    assert!(row.supported_scopes.is_empty());
    assert_eq!(row.downgrade_reasons, DowngradeReason::ALL.to_vec());
}

#[test]
fn scope_sensitive_lanes_never_publish_above_their_evidence() {
    // A workset, impact-query, or navigation-recall lane narrows safely instead of inheriting
    // a broader whole-workspace claim.
    let packet = packet();
    for row in &packet.lane_rows {
        if row.lane.is_scope_sensitive() {
            assert_eq!(
                row.published_claim,
                row.effective_claim(),
                "scope-sensitive lane {} publishes beyond its evidence",
                row.lane_id
            );
        }
    }
}

#[test]
fn impact_result_classes_stay_distinct() {
    // no-impact never hides an out-of-scope result; out-of-scope and policy-limited always
    // report a non-zero count.
    let packet = packet();
    for row in &packet.lane_rows {
        match row.impact_result_class {
            ImpactResultClass::NoImpact => assert_eq!(row.out_of_scope_count, 0),
            ImpactResultClass::OutOfScope => assert!(row.out_of_scope_count > 0),
            ImpactResultClass::PolicyLimited => assert!(row.hidden_result_count > 0),
            ImpactResultClass::InScopeImpact => {}
        }
    }
}

#[test]
fn topology_identities_are_namespaced() {
    let packet = packet();
    assert!(!packet.topology_identity_scheme.trim().is_empty());
    for row in &packet.lane_rows {
        assert!(
            !row.node_id_namespace.trim().is_empty(),
            "lane {} has no node identity namespace",
            row.lane_id
        );
        assert!(
            !row.edge_id_namespace.trim().is_empty(),
            "lane {} has no edge identity namespace",
            row.lane_id
        );
    }
}

#[test]
fn validate_flags_overstated_claim() {
    let mut packet = packet();
    if let Some(row) = packet
        .lane_rows
        .iter_mut()
        .find(|c| c.effective_claim() != GraphDepthClaim::Authoritative)
    {
        row.published_claim = GraphDepthClaim::Authoritative;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5GraphGovernanceViolation::OverstatedClaim { .. })));
    }
}

#[test]
fn validate_flags_decision_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .lane_rows
        .iter_mut()
        .find(|c| c.governance_decision != GraphGovernanceDecision::Withhold)
    {
        row.governance_decision = GraphGovernanceDecision::Withhold;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5GraphGovernanceViolation::DecisionMismatch { .. })));
    }
}

#[test]
fn validate_flags_downgrade_reasons_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .lane_rows
        .iter_mut()
        .find(|c| !c.downgrade_reasons.contains(&DowngradeReason::StaleGraph))
    {
        row.downgrade_reasons.push(DowngradeReason::StaleGraph);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5GraphGovernanceViolation::DowngradeReasonsMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_source_packet_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet.lane_rows.first_mut() {
        row.packet_ref = "artifacts/graph/m5/not-the-source.json".to_owned();
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5GraphGovernanceViolation::SourcePacketMismatch { .. })));
    }
}

#[test]
fn validate_flags_missing_downgrade_path() {
    let mut packet = packet();
    if let Some(row) = packet
        .lane_rows
        .iter_mut()
        .find(|c| c.governance_decision.is_narrowed())
    {
        row.downgrade_path = DowngradePath::NoneNeeded;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5GraphGovernanceViolation::MissingDowngradePath { .. })));
    }
}

#[test]
fn validate_flags_impact_count_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .lane_rows
        .iter_mut()
        .find(|c| c.impact_result_class == ImpactResultClass::OutOfScope)
    {
        row.out_of_scope_count = 0;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5GraphGovernanceViolation::ImpactCountMismatch { .. })));
    }
}

#[test]
fn validate_flags_missing_lane_row() {
    let mut packet = packet();
    let removed = packet.lane_rows.pop();
    assert!(removed.is_some());
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5GraphGovernanceViolation::MissingLaneRow { .. })));
}

#[test]
fn validate_flags_unclaimed_lane_row() {
    let mut packet = packet();
    packet
        .lanes
        .retain(|l| *l != GraphDepthLane::NavigationRecall);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5GraphGovernanceViolation::UnclaimedLaneRow { .. })));
    assert!(violations.iter().any(|v| matches!(
        v,
        M5GraphGovernanceViolation::ClosedVocabularyMismatch { field: "lanes" }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_lanes = packet.summary.total_lanes.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5GraphGovernanceViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(GraphDepthLane::WorksetScope.as_str(), "workset_scope");
    assert_eq!(
        GraphDepthLane::NavigationRecall.as_str(),
        "navigation_recall"
    );
    assert_eq!(GraphDepthClaim::Authoritative.as_str(), "authoritative");
    assert_eq!(GraphDepthClaim::Withheld.as_str(), "withheld");
    assert_eq!(ScopeMode::HotSet.as_str(), "hot_set");
    assert_eq!(GraphFreshness::Expired.as_str(), "expired");
    assert_eq!(RelationFidelity::Approximate.as_str(), "approximate");
    assert_eq!(EvidenceBacking::Uncited.as_str(), "uncited");
    assert_eq!(ImpactResultClass::PolicyLimited.as_str(), "policy_limited");
    assert_eq!(DowngradePath::WidenScope.as_str(), "widen_scope");
    assert_eq!(DowngradePath::NoneNeeded.as_str(), "none");
    assert_eq!(
        DowngradeReason::ApproximateRelations.as_str(),
        "approximate_relations"
    );
    assert_eq!(
        GraphGovernanceDecision::MarkProvisional.as_str(),
        "mark_provisional"
    );
}

#[test]
fn claim_rank_orders_low_to_high() {
    assert!(GraphDepthClaim::Withheld.rank() < GraphDepthClaim::Provisional.rank());
    assert!(GraphDepthClaim::Provisional.rank() < GraphDepthClaim::ScopeQualified.rank());
    assert!(GraphDepthClaim::ScopeQualified.rank() < GraphDepthClaim::Authoritative.rank());
    assert_eq!(
        GraphDepthClaim::Authoritative.min(GraphDepthClaim::Provisional),
        GraphDepthClaim::Provisional
    );
}

#[test]
fn source_packets_point_at_real_graph_packets() {
    for lane in GraphDepthLane::ALL {
        assert!(
            lane.source_packet().starts_with("artifacts/"),
            "lane {} does not bind to a checked-in graph packet",
            lane.as_str()
        );
        assert!(lane.source_packet().ends_with(".json"));
    }
}
