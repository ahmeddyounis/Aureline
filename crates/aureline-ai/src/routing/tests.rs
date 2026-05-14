use super::*;

fn policy_context() -> RoutingPolicyContext {
    RoutingPolicyContext {
        policy_epoch_ref: "policy_epoch:alpha:2026-05-13".to_owned(),
        trust_state: PolicyTrustState::Trusted,
        deployment_profile_class: DeploymentProfileClass::ManagedCloud,
        execution_context_ref: Some("execution_context:ai-preview:0001".to_owned()),
    }
}

fn quota(
    state: QuotaStateClass,
    family: QuotaFamilyClass,
    scope: QuotaScopeClass,
) -> QuotaInspector {
    QuotaInspector {
        quota_family_class: family,
        quota_state_class: state,
        quota_scope_class: scope,
        budget_owner_ref: "quota_owner:workspace:alpha-preview".to_owned(),
        quota_meter_ref: Some("quota_meter:workspace:ai-routing-preview".to_owned()),
        quota_forecast_ref: Some("quota_forecast:workspace:ai-routing-preview".to_owned()),
        usage_export_ref: Some("usage_export:ai-routing-preview".to_owned()),
        explanation_label: "Workspace hosted AI entitlement is available for this request."
            .to_owned(),
        local_continuity_label:
            "Local editing, search, Git, and diagnostics continue without hosted AI.".to_owned(),
        recovery_action_ref: Some("action:ai-routing:view-quota".to_owned()),
    }
}

fn envelope(cost: CostEnvelopeClass) -> LatencyCostEnvelope {
    LatencyCostEnvelope {
        latency_envelope_class: LatencyEnvelopeClass::StreamingFirstTokenUnder500Ms,
        cost_envelope_class: cost,
        cost_visibility_class: CostVisibilityClass::BundledNoIncrementalCost,
        token_ceiling_class: TokenCeilingClass::TokensUnder32K,
        tool_call_ceiling_class: ToolCallCeilingClass::BoundedToolCallsUnder4,
        wall_time_ceiling_class: WallTimeCeilingClass::WallTimeUnder30S,
        budget_routing_policy_ref: "budget_policy:ai-routing-preview".to_owned(),
        graduation_packet_ref: "graduation_packet:ai-routing-preview".to_owned(),
        envelope_evidence_ref: "envelope_evidence:ai-routing-preview".to_owned(),
        explanation_label:
            "Route uses the bundled preview band with a streaming interactive latency envelope."
                .to_owned(),
    }
}

fn hosted_candidate(candidate_id: &str, selected: bool) -> AiRouteCandidate {
    AiRouteCandidate {
        candidate_id: candidate_id.to_owned(),
        provider_entry_ref: "provider-entry:first_party_managed:vendor_hosted_chat:0001".to_owned(),
        provider_label: "Aureline managed hosted AI".to_owned(),
        provider_class: AiRouteProviderClass::FirstPartyManaged,
        model_entry_ref: "model-entry:vendor_hosted_chat:general:0001".to_owned(),
        model_label: "Hosted general chat preview".to_owned(),
        execution_locus_class: ExecutionLocusClass::VendorHostedFirstPartyManaged,
        route_origin_class: RouteOriginClass::VendorHostedManaged,
        region_posture_class: RegionPostureClass::SingleRegionPinned,
        retention_stance_class: RetentionStanceClass::NoRetentionPromisedBodyDiscarded,
        quota: quota(
            QuotaStateClass::WithinLimit,
            QuotaFamilyClass::VendorHostedEntitlementQuota,
            QuotaScopeClass::VendorHostedEntitlement,
        ),
        envelope: envelope(CostEnvelopeClass::VendorHostedEntitlementBand),
        route_selection_reason_class: RouteSelectionReasonClass::NoCheaperQualifyingRouteExisted,
        route_selection_override_reason_class:
            RouteSelectionOverrideReasonClass::NoOverrideCheapestWasUsed,
        exhaustion_state_class: ExhaustionStateClass::NotExhaustedRouteAdmitted,
        selected_outcome_class: if selected {
            SelectedOutcomeClass::SelectedThisPath
        } else {
            SelectedOutcomeClass::NotSelectedPolicyPin
        },
        route_selection_disclosure_ref: None,
        originating_approval_ticket_ref: None,
        explanation_label: "Hosted route selected because no cheaper qualifying route exists."
            .to_owned(),
    }
}

fn local_candidate(candidate_id: &str, selected: bool) -> AiRouteCandidate {
    AiRouteCandidate {
        candidate_id: candidate_id.to_owned(),
        provider_entry_ref: "provider-entry:local_pack:general:0001".to_owned(),
        provider_label: "Local signed pack".to_owned(),
        provider_class: AiRouteProviderClass::FirstPartySelfHosted,
        model_entry_ref: "model-entry:local_pack:general:0001".to_owned(),
        model_label: "Local general pack".to_owned(),
        execution_locus_class: ExecutionLocusClass::LocalCompanionService,
        route_origin_class: RouteOriginClass::StayedLocal,
        region_posture_class: RegionPostureClass::LocalDeviceOnly,
        retention_stance_class: RetentionStanceClass::NoRetentionLocalOnly,
        quota: quota(
            QuotaStateClass::WithinLimit,
            QuotaFamilyClass::PerUserLocalNoLimit,
            QuotaScopeClass::LocalDevice,
        ),
        envelope: envelope(CostEnvelopeClass::BundledNoIncrementalCost),
        route_selection_reason_class: RouteSelectionReasonClass::FallbackAfterCheapestExhausted,
        route_selection_override_reason_class:
            RouteSelectionOverrideReasonClass::CheapestRouteQuotaExhausted,
        exhaustion_state_class: ExhaustionStateClass::QuotaFamilyExhausted,
        selected_outcome_class: if selected {
            SelectedOutcomeClass::SelectedThisPath
        } else {
            SelectedOutcomeClass::NotSelectedQuotaExhausted
        },
        route_selection_disclosure_ref: Some(
            "route_disclosure:fallback:quota-exhausted".to_owned(),
        ),
        originating_approval_ticket_ref: None,
        explanation_label: "Local route selected because the hosted entitlement is exhausted."
            .to_owned(),
    }
}

fn valid_hosted_packet() -> AiRoutingPacket {
    AiRoutingPacket::new(
        "ai_routing_packet:hosted-preview:0001",
        "workflow.alpha.ai.hosted_model_use",
        "request_workspace:alpha:hosted-preview",
        RoutingRunStateClass::PreviewPreDispatch,
        policy_context(),
        "capability_lifecycle:alpha.ai.routing_cost",
        Some("identity_mode_baseline:alpha:local_vs_managed".to_owned()),
        vec![hosted_candidate("candidate:hosted-managed", true)],
        "candidate:hosted-managed",
        Vec::new(),
        vec![
            "docs/ai/provider_model_registry_contract.md".to_owned(),
            "docs/ai/spend_and_route_receipt_contract.md".to_owned(),
            "docs/ai/model_graduation_and_budget_contract.md".to_owned(),
        ],
        "2026-05-13T12:00:00Z",
    )
}

#[test]
fn hosted_preview_surfaces_provider_model_quota_and_envelope() {
    let packet = valid_hosted_packet();

    assert!(packet.validate().is_empty());
    let rows = packet.surface_rows();
    assert!(rows
        .iter()
        .any(|row| row.row_id == "provider" && row.value_label == "Aureline managed hosted AI"));
    assert!(rows
        .iter()
        .any(|row| row.row_id == "model" && row.value_label == "Hosted general chat preview"));
    assert!(rows
        .iter()
        .any(|row| row.row_id == "quota_state" && row.value_token == "within_limit"));
    assert!(rows.iter().any(|row| row.row_id == "latency_envelope"));
    assert!(rows.iter().any(|row| row.row_id == "cost_envelope"));

    let support = packet.support_packet();
    assert_eq!(
        support.selected_provider_entry_ref,
        "provider-entry:first_party_managed:vendor_hosted_chat:0001"
    );
    assert_eq!(support.quota_state_token, "within_limit");
    assert_eq!(support.quota_scope_token, "vendor_hosted_entitlement");
    assert_eq!(support.cost_visibility_token, "bundled_no_incremental_cost");
    assert!(support.validation_violation_tokens.is_empty());
    assert!(!support.export_safe_json().contains("://"));
}

#[test]
fn policy_pinned_route_requires_visible_lineage_and_disclosure() {
    let mut selected = hosted_candidate("candidate:enterprise-gateway", true);
    selected.provider_entry_ref = "provider-entry:enterprise_gateway:pooled:0001".to_owned();
    selected.provider_label = "Enterprise gateway".to_owned();
    selected.execution_locus_class = ExecutionLocusClass::EnterpriseGatewayBrokered;
    selected.route_origin_class = RouteOriginClass::EnterpriseGateway;
    selected.quota = quota(
        QuotaStateClass::WithinLimit,
        QuotaFamilyClass::EnterpriseGatewayPooledQuota,
        QuotaScopeClass::EnterprisePool,
    );
    selected.envelope = envelope(CostEnvelopeClass::EnterprisePooledQuotaBand);
    selected.route_selection_reason_class = RouteSelectionReasonClass::PolicyPinnedSpecificRoute;
    selected.route_selection_override_reason_class =
        RouteSelectionOverrideReasonClass::PolicyPinnedMoreExpensiveRoute;
    selected.route_selection_disclosure_ref =
        Some("route_disclosure:policy-pin:enterprise-gateway".to_owned());
    selected.explanation_label =
        "Policy pinned the enterprise gateway route for this workspace.".to_owned();

    let mut blocked_hosted = hosted_candidate("candidate:hosted-managed", false);
    blocked_hosted.selected_outcome_class = SelectedOutcomeClass::NotSelectedPolicyPin;
    blocked_hosted.explanation_label =
        "Hosted managed route was not selected because policy pinned the gateway.".to_owned();

    let mut missing_lineage = valid_hosted_packet();
    missing_lineage.candidates = vec![blocked_hosted.clone(), selected.clone()];
    missing_lineage.selected_candidate_ref = selected.candidate_id.clone();
    assert!(missing_lineage
        .validate()
        .contains(&AiRoutingViolation::RouteChangeMissingVisibleLineage));

    let packet = AiRoutingPacket::new(
        "ai_routing_packet:policy-pin:0001",
        "workflow.alpha.ai.hosted_model_use",
        "request_workspace:alpha:policy-pin",
        RoutingRunStateClass::PreviewPreDispatch,
        policy_context(),
        "capability_lifecycle:alpha.ai.routing_cost",
        Some("identity_mode_baseline:alpha:local_vs_managed".to_owned()),
        vec![blocked_hosted, selected.clone()],
        selected.candidate_id.clone(),
        vec![RouteChangeLineage {
            lineage_id: "route_lineage:policy-pin:0001".to_owned(),
            cause_class: RouteChangeCauseClass::PolicyOverride,
            from_candidate_ref: Some("candidate:hosted-managed".to_owned()),
            to_candidate_ref: selected.candidate_id.clone(),
            route_selection_disclosure_ref: "route_disclosure:policy-pin:enterprise-gateway"
                .to_owned(),
            policy_epoch_ref: "policy_epoch:alpha:2026-05-13".to_owned(),
            visible_disclosure_label:
                "Workspace policy pins this request to the enterprise gateway.".to_owned(),
        }],
        vec![
            "docs/ai/provider_model_registry_contract.md".to_owned(),
            "docs/ai/model_graduation_and_budget_contract.md".to_owned(),
        ],
        "2026-05-13T12:05:00Z",
    );

    assert!(packet.validate().is_empty());
    assert_eq!(
        packet.support_packet().route_change_rows[0].cause_token,
        "policy_override"
    );
}

#[test]
fn quota_exhausted_hosted_route_must_fallback_or_deny_visibly() {
    let mut blocked = valid_hosted_packet();
    blocked.candidates[0].quota.quota_state_class = QuotaStateClass::Exhausted;
    assert!(blocked
        .validate()
        .contains(&AiRoutingViolation::HostedRouteQuotaBlockedButSelected));

    let mut exhausted_hosted = hosted_candidate("candidate:hosted-managed", false);
    exhausted_hosted.quota.quota_state_class = QuotaStateClass::Exhausted;
    exhausted_hosted.exhaustion_state_class = ExhaustionStateClass::QuotaFamilyExhausted;
    exhausted_hosted.selected_outcome_class = SelectedOutcomeClass::NotSelectedQuotaExhausted;
    exhausted_hosted.explanation_label =
        "Hosted managed route was not selected because entitlement quota is exhausted.".to_owned();

    let selected_local = local_candidate("candidate:local-pack", true);
    let packet = AiRoutingPacket::new(
        "ai_routing_packet:quota-fallback:0001",
        "workflow.alpha.ai.hosted_model_use",
        "request_workspace:alpha:quota-fallback",
        RoutingRunStateClass::PreviewPreDispatch,
        policy_context(),
        "capability_lifecycle:alpha.ai.routing_cost",
        Some("identity_mode_baseline:alpha:local_vs_managed".to_owned()),
        vec![exhausted_hosted, selected_local.clone()],
        selected_local.candidate_id.clone(),
        vec![RouteChangeLineage {
            lineage_id: "route_lineage:quota-fallback:0001".to_owned(),
            cause_class: RouteChangeCauseClass::FallbackAfterQuotaExhaustion,
            from_candidate_ref: Some("candidate:hosted-managed".to_owned()),
            to_candidate_ref: selected_local.candidate_id.clone(),
            route_selection_disclosure_ref: "route_disclosure:fallback:quota-exhausted".to_owned(),
            policy_epoch_ref: "policy_epoch:alpha:2026-05-13".to_owned(),
            visible_disclosure_label:
                "Hosted quota is exhausted, so this request stays on the local route.".to_owned(),
        }],
        vec![
            "docs/ai/provider_model_registry_contract.md".to_owned(),
            "docs/ai/spend_and_route_receipt_contract.md".to_owned(),
            "docs/ai/model_graduation_and_budget_contract.md".to_owned(),
        ],
        "2026-05-13T12:10:00Z",
    );

    assert!(packet.validate().is_empty());
    let support = packet.support_packet();
    assert_eq!(support.execution_locus_token, "local_companion_service");
    assert_eq!(support.exhaustion_state_token, "quota_family_exhausted");
    assert_eq!(
        support.route_change_rows[0].cause_token,
        "fallback_after_quota_exhaustion"
    );
}

#[test]
fn fixture_packets_round_trip_and_validate() {
    let fixture_paths = [
        include_str!("../../../../fixtures/ai/routing_cost_alpha/managed_hosted_chat_preview.json"),
        include_str!(
            "../../../../fixtures/ai/routing_cost_alpha/policy_forced_enterprise_route_change.json"
        ),
        include_str!(
            "../../../../fixtures/ai/routing_cost_alpha/quota_exhausted_fallback_visible.json"
        ),
    ];

    for fixture in fixture_paths {
        let packet: AiRoutingPacket = serde_json::from_str(fixture).expect("fixture parses");
        assert!(
            packet.validate().is_empty(),
            "fixture should be valid: {:?}",
            packet.validate()
        );
        assert!(!packet.export_safe_support_json().contains("://"));
    }
}
