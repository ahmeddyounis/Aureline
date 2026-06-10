use super::*;

const PACKET_ID: &str = "provider-route-disclosure:stable:0001";

fn proof_stale_to(narrowed_to: M5AiWorkflowQualificationClass) -> RouteDowngradeRule {
    RouteDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProofStale,
        narrowed_to,
        auto_enforced: true,
        rationale: "Stale proof narrows the public claim".to_owned(),
    }
}

fn local_route() -> RouteInspectorRow {
    RouteInspectorRow {
        route_id: "local-on-device".to_owned(),
        provider_id: "local-runtime".to_owned(),
        model_id: "local-pack-small".to_owned(),
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        execution_mode: ExecutionModeClass::Local,
        locality: RouteLocalityClass::OnDeviceInProcess,
        region: RouteRegionClass::OnDeviceOnly,
        retention: RouteRetentionClass::NoRetentionLocalOnly,
        cost_disclosure: RouteCostDisclosureClass::NoCostLocalCompute,
        tool_side_effect: RouteToolSideEffectClass::InspectOnly,
        automation_authority: RouteAutomationAuthorityClass::ReadOnlyNoApply,
        mode_disclosure_label: "Runs entirely on this device".to_owned(),
        downgrade_rules: vec![
            proof_stale_to(M5AiWorkflowQualificationClass::Beta),
            RouteDowngradeRule {
                trigger: M5AiWorkflowDowngradeTrigger::PolicyBlocked,
                narrowed_to: M5AiWorkflowQualificationClass::Unavailable,
                auto_enforced: true,
                rationale: "A policy block makes the route unavailable".to_owned(),
            },
        ],
        evidence_packet_refs: vec!["evidence:local-model-pack:m5".to_owned()],
    }
}

fn byok_route() -> RouteInspectorRow {
    RouteInspectorRow {
        route_id: "byok-vendor-direct".to_owned(),
        provider_id: "byok-vendor".to_owned(),
        model_id: "vendor-mid".to_owned(),
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        execution_mode: ExecutionModeClass::Byok,
        locality: RouteLocalityClass::ByokDirectVendor,
        region: RouteRegionClass::SingleRegionPinned,
        retention: RouteRetentionClass::NoRetentionPromised,
        cost_disclosure: RouteCostDisclosureClass::MeteredPerTokenDisclosed,
        tool_side_effect: RouteToolSideEffectClass::LocalReversibleEdit,
        automation_authority: RouteAutomationAuthorityClass::ScopedApplyHumanApproved,
        mode_disclosure_label: "Uses your key to call the vendor directly".to_owned(),
        downgrade_rules: vec![proof_stale_to(M5AiWorkflowQualificationClass::Beta)],
        evidence_packet_refs: vec!["evidence:byok-route-receipt:m5".to_owned()],
    }
}

fn managed_route() -> RouteInspectorRow {
    RouteInspectorRow {
        route_id: "managed-vendor-hosted".to_owned(),
        provider_id: "managed-first-party".to_owned(),
        model_id: "managed-large".to_owned(),
        claimed_qualification: M5AiWorkflowQualificationClass::Beta,
        execution_mode: ExecutionModeClass::Managed,
        locality: RouteLocalityClass::ManagedVendorHosted,
        region: RouteRegionClass::MultiRegionPinned,
        retention: RouteRetentionClass::BoundedRetentionWithExport,
        cost_disclosure: RouteCostDisclosureClass::BudgetCappedDisclosed,
        tool_side_effect: RouteToolSideEffectClass::ExternalReversibleComment,
        automation_authority: RouteAutomationAuthorityClass::BackgroundAgentHumanGatedMerge,
        mode_disclosure_label: "Brokered through the managed service".to_owned(),
        downgrade_rules: vec![proof_stale_to(M5AiWorkflowQualificationClass::Preview)],
        evidence_packet_refs: vec!["evidence:managed-route-receipt:m5".to_owned()],
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        PROVIDER_ROUTE_DISCLOSURE_SCHEMA_REF.to_owned(),
        PROVIDER_ROUTE_DISCLOSURE_DOC_REF.to_owned(),
        PROVIDER_MODEL_REGISTRY_SCHEMA_REF.to_owned(),
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF.to_owned(),
    ]
}

fn proof_freshness() -> ProviderRouteDisclosureProofFreshness {
    ProviderRouteDisclosureProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> ProviderRouteDisclosurePacket {
    ProviderRouteDisclosurePacket::new(ProviderRouteDisclosurePacketInput {
        packet_id: PACKET_ID.to_owned(),
        disclosure_label: "Provider Route Disclosure And Inspectors".to_owned(),
        routes: vec![local_route(), byok_route(), managed_route()],
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

#[test]
fn provider_route_disclosure_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn locality_maps_to_declared_mode() {
    assert_eq!(
        RouteLocalityClass::OnDeviceInProcess.execution_mode(),
        ExecutionModeClass::Local
    );
    assert_eq!(
        RouteLocalityClass::ByokSelfHosted.execution_mode(),
        ExecutionModeClass::Byok
    );
    assert_eq!(
        RouteLocalityClass::ManagedEnterpriseGateway.execution_mode(),
        ExecutionModeClass::Managed
    );
}

#[test]
fn mode_counts_match_fixture() {
    let packet = packet();
    assert_eq!(packet.local_route_count(), 1);
    assert_eq!(packet.byok_route_count(), 1);
    assert_eq!(packet.managed_route_count(), 1);
}

#[test]
fn no_routes_fails() {
    let mut packet = packet();
    packet.routes.clear();
    assert!(packet
        .validate()
        .contains(&ProviderRouteDisclosureViolation::NoRoutes));
}

#[test]
fn duplicate_route_fails() {
    let mut packet = packet();
    let first = packet.routes[0].clone();
    packet.routes.push(first);
    assert!(packet
        .validate()
        .contains(&ProviderRouteDisclosureViolation::DuplicateRoute));
}

#[test]
fn route_row_incomplete_fails() {
    let mut packet = packet();
    packet.routes[0].mode_disclosure_label.clear();
    assert!(packet
        .validate()
        .contains(&ProviderRouteDisclosureViolation::RouteRowIncomplete));
}

#[test]
fn mode_locality_mismatch_fails() {
    let mut packet = packet();
    // Claim a managed mode while the locality is on-device.
    packet.routes[0].execution_mode = ExecutionModeClass::Managed;
    assert!(packet
        .validate()
        .contains(&ProviderRouteDisclosureViolation::ModeLocalityMismatch));
}

#[test]
fn local_mode_not_fully_local_fails() {
    let mut packet = packet();
    // A local route may not pin to a remote region.
    packet.routes[0].region = RouteRegionClass::SingleRegionPinned;
    assert!(packet
        .validate()
        .contains(&ProviderRouteDisclosureViolation::LocalModeNotFullyLocal));
}

#[test]
fn undisclosed_trust_posture_fails() {
    let mut packet = packet();
    // A claimed managed route may not hide its region.
    packet.routes[2].region = RouteRegionClass::UnknownUnverified;
    assert!(packet
        .validate()
        .contains(&ProviderRouteDisclosureViolation::UndisclosedTrustPosture));
}

#[test]
fn side_effect_without_apply_authority_fails() {
    let mut packet = packet();
    // A mutating tool may not run under read-only authority.
    packet.routes[1].automation_authority = RouteAutomationAuthorityClass::ReadOnlyNoApply;
    assert!(packet
        .validate()
        .contains(&ProviderRouteDisclosureViolation::SideEffectWithoutApplyAuthority));
}

#[test]
fn claimed_route_missing_evidence_fails() {
    let mut packet = packet();
    packet.routes[0].evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&ProviderRouteDisclosureViolation::ClaimedRouteMissingEvidence));
}

#[test]
fn downgrade_rules_missing_fails() {
    let mut packet = packet();
    packet.routes[0].downgrade_rules.clear();
    assert!(packet
        .validate()
        .contains(&ProviderRouteDisclosureViolation::DowngradeRulesMissing));
}

#[test]
fn downgrade_rule_missing_proof_stale_fails() {
    let mut packet = packet();
    packet.routes[0]
        .downgrade_rules
        .retain(|rule| rule.trigger != M5AiWorkflowDowngradeTrigger::ProofStale);
    assert!(packet
        .validate()
        .contains(&ProviderRouteDisclosureViolation::DowngradeRuleMissingProofStale));
}

#[test]
fn downgrade_rule_not_narrowing_fails() {
    let mut packet = packet();
    // Narrowing "to" Stable from a Stable claim does not narrow.
    packet.routes[0].downgrade_rules[0].narrowed_to = M5AiWorkflowQualificationClass::Stable;
    assert!(packet
        .validate()
        .contains(&ProviderRouteDisclosureViolation::DowngradeRuleNotNarrowing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ProviderRouteDisclosureViolation::MissingSourceContracts));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ProviderRouteDisclosureViolation::ProofFreshnessIncomplete));
}

#[test]
fn narrowed_qualification_applies_matching_rule() {
    let route = byok_route();
    assert_eq!(
        route.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProofStale),
        M5AiWorkflowQualificationClass::Beta
    );
    // A trigger with no rule leaves the claim unchanged.
    assert_eq!(
        route.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProviderUnavailable),
        M5AiWorkflowQualificationClass::Stable
    );
}

#[test]
fn render_inspector_lists_every_posture() {
    let card = managed_route().render_inspector();
    assert!(card.contains("managed-vendor-hosted"));
    assert!(card.contains("multi_region_pinned"));
    assert!(card.contains("bounded_retention_with_export"));
    assert!(card.contains("budget_capped_disclosed"));
    assert!(card.contains("background_agent_human_gated_merge"));
}

#[test]
fn markdown_summary_lists_every_route() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("Route inspectors"));
    for route in &packet().routes {
        assert!(
            summary.contains(&route.route_id),
            "missing {}",
            route.route_id
        );
    }
}

#[test]
fn held_managed_lane_fixture_validates() {
    let packet: ProviderRouteDisclosurePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors/managed_held_pending_graduation.json"
    )))
    .expect("held managed lane fixture parses");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // The two managed routes are held, not claimed; both narrow to unavailable.
    let managed_held: Vec<&RouteInspectorRow> = packet
        .routes
        .iter()
        .filter(|route| route.execution_mode == ExecutionModeClass::Managed && !route.is_claimed())
        .collect();
    assert_eq!(managed_held.len(), 2);
    for route in managed_held {
        assert_eq!(
            route.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProofStale),
            M5AiWorkflowQualificationClass::Unavailable
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_provider_route_disclosure_export()
        .expect("checked provider route disclosure export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
    assert!(!packet.routes.is_empty());
    // Local, BYOK, and managed modes are all materialized.
    assert!(packet.local_route_count() >= 1);
    assert!(packet.byok_route_count() >= 1);
    assert!(packet.managed_route_count() >= 1);
}
