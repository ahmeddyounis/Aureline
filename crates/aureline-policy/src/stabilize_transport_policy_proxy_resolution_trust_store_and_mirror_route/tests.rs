use super::*;

#[test]
fn seeded_page_qualifies_stable() {
    let page = seeded_transport_policy_inspector_page();
    assert!(
        page.qualifies_stable(),
        "seeded page must qualify stable; defects: {:?}",
        page.defects
    );
    assert!(page.defects.is_empty());
}

#[test]
fn seeded_page_covers_required_truth_sets() {
    let page = seeded_transport_policy_inspector_page();
    assert!(page.covers_all_endpoint_classes());
    assert!(page.covers_all_trust_layers());
    assert_eq!(
        page.route_source_precedence,
        RouteSourceClass::PRECEDENCE.to_vec()
    );
    assert_eq!(page.policies.len(), EndpointClass::ALL.len());
    assert_eq!(page.network_events.len(), EndpointClass::ALL.len());
}

#[test]
fn seeded_page_exposes_distinct_failure_vocabulary() {
    let page = seeded_transport_policy_inspector_page();
    let decisions: BTreeSet<&str> = page
        .network_events
        .iter()
        .map(|event| event.egress_decision_token.as_str())
        .collect();
    for expected in [
        "deny_policy",
        "deny_contract_mismatch",
        "deny_trust",
        "deny_transport",
        "allow_mirror",
        "offline_deferred",
    ] {
        assert!(
            decisions.contains(expected),
            "missing decision token {expected}"
        );
    }
}

#[test]
fn seeded_page_excludes_raw_secret_material() {
    let page = seeded_transport_policy_inspector_page();
    assert!(page.excludes_raw_secret_material());
    assert!(page.summary.raw_secret_material_excluded);
}

#[test]
fn raw_secret_material_triggers_withdrawal() {
    let mut policies = seeded_policies();
    policies[0].raw_secret_material_excluded = false;
    let page = TransportPolicyInspectorPage::new(
        "test:raw-secret",
        "raw secret drill",
        "2026-06-01T00:00:00Z",
        RouteSourceClass::PRECEDENCE.to_vec(),
        policies,
        seeded_network_events(),
        seeded_trust_layers(),
    );

    assert_eq!(
        page.summary.overall_qualification_token,
        TransportPolicyInspectorQualificationClass::Withdrawn.as_str()
    );
    assert_eq!(page.defects.len(), 1);
    assert_eq!(
        page.defects[0].narrow_reason,
        TransportPolicyInspectorNarrowReasonClass::RawSecretMaterialExposed
    );
}

#[test]
fn missing_endpoint_coverage_narrows_to_preview() {
    let mut policies = seeded_policies();
    policies.retain(|policy| policy.endpoint_class != EndpointClass::Ai);
    let page = TransportPolicyInspectorPage::new(
        "test:missing-endpoint",
        "missing endpoint drill",
        "2026-06-01T00:00:00Z",
        RouteSourceClass::PRECEDENCE.to_vec(),
        policies,
        seeded_network_events(),
        seeded_trust_layers(),
    );

    assert_eq!(
        page.summary.overall_qualification_token,
        TransportPolicyInspectorQualificationClass::Preview.as_str()
    );
    assert!(page.defects.iter().any(|defect| {
        defect.narrow_reason == TransportPolicyInspectorNarrowReasonClass::MissingEndpointCoverage
            && defect.source == "ai"
    }));
}

#[test]
fn route_precedence_drift_narrows_to_preview() {
    let mut precedence = RouteSourceClass::PRECEDENCE.to_vec();
    precedence.swap(0, 1);
    let page = TransportPolicyInspectorPage::new(
        "test:bad-precedence",
        "bad precedence drill",
        "2026-06-01T00:00:00Z",
        precedence,
        seeded_policies(),
        seeded_network_events(),
        seeded_trust_layers(),
    );

    assert_eq!(
        page.summary.overall_qualification_token,
        TransportPolicyInspectorQualificationClass::Preview.as_str()
    );
    assert!(page.defects.iter().any(|defect| {
        defect.narrow_reason
            == TransportPolicyInspectorNarrowReasonClass::RouteSourcePrecedenceIncomplete
    }));
}

#[test]
fn non_allow_event_without_recovery_hint_narrows_to_beta() {
    let mut events = seeded_network_events();
    let event = events
        .iter_mut()
        .find(|event| event.endpoint_class == EndpointClass::Ai)
        .expect("seeded AI event exists");
    event.recovery_action_hint_token.clear();

    let page = TransportPolicyInspectorPage::new(
        "test:no-recovery",
        "missing recovery drill",
        "2026-06-01T00:00:00Z",
        RouteSourceClass::PRECEDENCE.to_vec(),
        seeded_policies(),
        events,
        seeded_trust_layers(),
    );

    assert_eq!(
        page.summary.overall_qualification_token,
        TransportPolicyInspectorQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|defect| {
        defect.narrow_reason == TransportPolicyInspectorNarrowReasonClass::RecoveryActionMissing
    }));
}

#[test]
fn missing_local_core_continuity_narrows_to_beta() {
    let mut policies = seeded_policies();
    policies[0].local_core_continuity_explicit = false;

    let page = TransportPolicyInspectorPage::new(
        "test:no-local-core",
        "missing local core drill",
        "2026-06-01T00:00:00Z",
        RouteSourceClass::PRECEDENCE.to_vec(),
        policies,
        seeded_network_events(),
        seeded_trust_layers(),
    );

    assert_eq!(
        page.summary.overall_qualification_token,
        TransportPolicyInspectorQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|defect| {
        defect.narrow_reason
            == TransportPolicyInspectorNarrowReasonClass::LocalCoreContinuityMissing
    }));
}
