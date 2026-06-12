use super::*;

fn packet() -> M5ImpactQueryPacket {
    current_m5_impact_query_packet().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_IMPACT_QUERY_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_IMPACT_QUERY_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_body() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_result_class_is_exercised() {
    // The headline guardrail: the corpus distinguishes every empty state, so empty answers never
    // collapse into one misleading "no impact" message.
    let packet = packet();
    let classes: BTreeSet<ImpactResultClass> =
        packet.queries.iter().map(|q| q.result_class).collect();
    for class in ImpactResultClass::ALL {
        assert!(
            classes.contains(&class),
            "no query exercises result class {}",
            class.as_str()
        );
    }
}

#[test]
fn every_empty_state_carries_a_reason() {
    let packet = packet();
    assert!(packet.all_empty_states_have_reason());
    for query in &packet.queries {
        if query.requires_empty_reason() {
            assert!(
                query
                    .empty_reason
                    .as_ref()
                    .is_some_and(|reason| !reason.trim().is_empty()),
                "query {} ({}) carries no empty_reason",
                query.query_id,
                query.result_class.as_str()
            );
        }
    }
}

#[test]
fn no_impact_never_hides_out_of_scope_or_policy_objects() {
    let packet = packet();
    assert!(packet.no_impact_never_hides_objects());
    for query in &packet.queries {
        if query.result_class.implies_no_impact() {
            assert_eq!(query.out_of_scope_count, 0);
            assert_eq!(query.hidden_count, 0);
            assert!(query.included_objects.is_empty());
        }
    }
}

#[test]
fn every_narrowed_result_offers_a_remediation_action() {
    let packet = packet();
    assert!(packet.all_narrowed_results_offer_remediation());
    for query in &packet.queries {
        if query.requires_remediation() {
            assert!(
                query.remediation_actions.iter().any(|a| a.is_offered()),
                "query {} is narrowed but offers no action",
                query.query_id
            );
            if let Some(required) = query.result_class.required_action() {
                assert!(
                    query.offers(required),
                    "query {} does not offer required action {}",
                    query.query_id,
                    required.as_str()
                );
            }
        } else {
            assert_eq!(
                query.remediation_actions,
                vec![RemediationAction::NoneNeeded],
                "definitive query {} should offer no recovery path",
                query.query_id
            );
        }
    }
}

#[test]
fn included_and_withheld_counts_are_visible_per_query() {
    // Users can see how many affected objects were included versus out of scope or hidden.
    let packet = packet();
    for query in &packet.queries {
        assert_eq!(query.included_count, query.included_objects.len());
    }
}

#[test]
fn every_query_carries_an_export_safe_permalink() {
    let packet = packet();
    for query in &packet.queries {
        assert!(
            query.permalink_is_export_safe(),
            "query {} has an unsafe permalink",
            query.query_id
        );
        assert_eq!(
            packet.permalink_for_query(&query.query_id),
            Some(query.export_permalink.as_str())
        );
        for object in &query.included_objects {
            assert!(
                object.permalink_is_export_safe(),
                "object {} in query {} has an unsafe permalink",
                object.node_id,
                query.query_id
            );
        }
    }
}

#[test]
fn every_nonexact_affected_object_is_labeled() {
    let packet = packet();
    for query in &packet.queries {
        for object in &query.included_objects {
            if object.evidence_class.requires_disclosure() {
                assert!(
                    object
                        .evidence_reason
                        .as_ref()
                        .is_some_and(|reason| !reason.trim().is_empty()),
                    "object {} in query {} ({}) carries no evidence_reason",
                    object.node_id,
                    query.query_id,
                    object.evidence_class.as_str()
                );
            }
        }
    }
}

#[test]
fn evidence_summary_matches_included_objects() {
    let packet = packet();
    for query in &packet.queries {
        assert_eq!(query.evidence_summary, query.computed_evidence_summary());
        assert_eq!(query.evidence_summary.total(), query.included_objects.len());
    }
}

#[test]
fn every_surface_has_exactly_one_binding() {
    let packet = packet();
    assert_eq!(
        packet.consumer_bindings.len(),
        ImpactConsumerSurface::ALL.len(),
        "one binding per surface"
    );
    for surface in ImpactConsumerSurface::ALL {
        assert!(
            packet.consumer_binding(surface).is_some(),
            "missing binding for surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn every_binding_is_stamped_with_the_active_snapshot() {
    let packet = packet();
    for binding in &packet.consumer_bindings {
        assert_eq!(binding.snapshot_id, packet.active_scope.snapshot_id);
        assert_eq!(binding.scope_id, packet.active_scope.scope_id);
    }
}

#[test]
fn every_query_survives_into_support_export() {
    // The answer survives beyond one panel: support export carries every query.
    let packet = packet();
    assert!(packet.every_query_in_support_export());
}

#[test]
fn packet_binds_to_canonical_upstream_packets() {
    let packet = packet();
    assert_eq!(
        packet.governance_matrix_ref,
        M5_IMPACT_QUERY_GOVERNANCE_MATRIX_REF
    );
    assert_eq!(packet.scope_packet_ref, M5_IMPACT_QUERY_SCOPE_PACKET_REF);
    assert_eq!(
        packet.topology_packet_ref,
        M5_IMPACT_QUERY_TOPOLOGY_PACKET_REF
    );
}

#[test]
fn export_projection_reflects_body_and_guardrails() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(projection.snapshot_id, packet.active_scope.snapshot_id);
    assert_eq!(projection.scope_id, packet.active_scope.scope_id);
    assert_eq!(projection.queries.len(), packet.queries.len());
    assert!(projection.all_empty_states_have_reason);
    assert!(projection.no_impact_never_hides_objects);
    assert!(projection.all_narrowed_results_offer_remediation);
    assert!(projection.every_query_in_support_export);
    for row in &projection.queries {
        assert!(!row.permalink.trim().is_empty());
        assert!(row.permalink.contains(&row.query_id));
    }
}

#[test]
fn validate_flags_collapsed_empty_state() {
    let mut packet = packet();
    if let Some(query) = packet
        .queries
        .iter_mut()
        .find(|q| q.requires_empty_reason())
    {
        query.empty_reason = None;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5ImpactQueryViolation::MissingEmptyReason { .. })));
    }
}

#[test]
fn validate_flags_no_impact_hiding_objects() {
    let mut packet = packet();
    if let Some(query) = packet
        .queries
        .iter_mut()
        .find(|q| q.result_class.implies_no_impact())
    {
        query.out_of_scope_count = 3;
        packet.summary = packet.computed_summary();
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5ImpactQueryViolation::NoImpactHidesObjects { .. })));
    }
}

#[test]
fn validate_flags_missing_remediation_action() {
    let mut packet = packet();
    if let Some(query) = packet
        .queries
        .iter_mut()
        .find(|q| q.result_class.requires_remediation())
    {
        query.remediation_actions = vec![RemediationAction::NoneNeeded];
        packet.summary = packet.computed_summary();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ImpactQueryViolation::MissingRemediationAction { .. }
                | M5ImpactQueryViolation::MissingRequiredAction { .. }
        )));
    }
}

#[test]
fn validate_flags_unlabeled_affected_evidence() {
    let mut packet = packet();
    'outer: for query in &mut packet.queries {
        for object in &mut query.included_objects {
            if object.evidence_class.requires_disclosure() {
                object.evidence_reason = None;
                break 'outer;
            }
        }
    }
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5ImpactQueryViolation::UnlabeledAffectedEvidence { .. })));
}

#[test]
fn validate_flags_unsafe_query_permalink() {
    let mut packet = packet();
    if let Some(query) = packet.queries.first_mut() {
        query.export_permalink = "aureline://workspace:aureline/impact/query/mismatch".to_owned();
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5ImpactQueryViolation::UnsafeQueryPermalink { .. })));
    }
}

#[test]
fn validate_flags_evidence_summary_mismatch() {
    let mut packet = packet();
    if let Some(query) = packet
        .queries
        .iter_mut()
        .find(|q| !q.included_objects.is_empty())
    {
        query.evidence_summary.exact = query.evidence_summary.exact.wrapping_add(1);
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5ImpactQueryViolation::EvidenceSummaryMismatch { .. })));
    }
}

#[test]
fn validate_flags_query_missing_from_support_export() {
    let mut packet = packet();
    if let Some(binding) = packet
        .consumer_bindings
        .iter_mut()
        .find(|b| b.surface == ImpactConsumerSurface::SupportExport)
    {
        binding.carries_query_ids.clear();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5ImpactQueryViolation::QueryMissingFromSupportExport { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_surface_binding() {
    let mut packet = packet();
    packet
        .consumer_bindings
        .retain(|b| b.surface != ImpactConsumerSurface::ReviewExplanation);
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5ImpactQueryViolation::MissingSurfaceBinding { .. })));
}

#[test]
fn validate_flags_snapshot_binding_mismatch() {
    let mut packet = packet();
    if let Some(binding) = packet.consumer_bindings.first_mut() {
        binding.snapshot_id = "workset-scope:snapshot:stale".to_owned();
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5ImpactQueryViolation::SnapshotBindingMismatch { .. })));
    }
}

#[test]
fn validate_flags_governance_ref_mismatch() {
    let mut packet = packet();
    packet.governance_matrix_ref = "artifacts/graph/m5/not-the-matrix.json".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&M5ImpactQueryViolation::GovernanceMatrixRefMismatch));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.query_count = packet.summary.query_count.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5ImpactQueryViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(ImpactResultClass::InScopeImpact.as_str(), "in_scope_impact");
    assert_eq!(ImpactResultClass::NoImpact.as_str(), "no_impact");
    assert_eq!(ImpactResultClass::Unknown.as_str(), "unknown");
    assert_eq!(ImpactResultClass::OutOfScope.as_str(), "out_of_scope");
    assert_eq!(ImpactResultClass::PolicyLimited.as_str(), "policy_limited");
    assert_eq!(
        ImpactResultClass::ProviderUnavailable.as_str(),
        "provider_unavailable"
    );
    assert_eq!(ImpactResultClass::StaleGraph.as_str(), "stale_graph");
    assert_eq!(RemediationAction::WidenScope.as_str(), "widen_scope");
    assert_eq!(RemediationAction::RefreshIndex.as_str(), "refresh_index");
    assert_eq!(RemediationAction::NoneNeeded.as_str(), "none");
    assert_eq!(
        ImpactConsumerSurface::RefactorPlanning.as_str(),
        "refactor_planning"
    );
    assert_eq!(
        ImpactConsumerSurface::SupportExport.as_str(),
        "support_export"
    );
    assert!(ImpactResultClass::NoImpact.implies_no_impact());
    assert!(!ImpactResultClass::OutOfScope.implies_no_impact());
    assert!(ImpactResultClass::InScopeImpact.is_definitive());
    assert!(ImpactResultClass::StaleGraph.requires_remediation());
    assert!(RemediationAction::WidenScope.is_offered());
    assert!(!RemediationAction::NoneNeeded.is_offered());
    assert!(ImpactConsumerSurface::ImpactPanel.is_origin_panel());
    assert!(ImpactConsumerSurface::SupportExport.is_support_export());
}
