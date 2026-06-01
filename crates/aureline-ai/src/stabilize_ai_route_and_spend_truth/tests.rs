use super::*;

const ACTION_ID: &str = "ai-action:route-spend:stable:0001";

fn registry_resolution() -> RouteRegistryResolution {
    RouteRegistryResolution {
        resolved: true,
        provider_id: "provider:first-party-managed".to_owned(),
        provider_label: "Aureline managed hosted AI".to_owned(),
        model_id: "model:hosted-review".to_owned(),
        model_version: "2026.05".to_owned(),
        model_label: "Hosted review model".to_owned(),
        transport_class: RegistryTransportClass::RemoteHttps,
        auth_mode: RegistryAuthModeClass::VendorHostedFirstPartyManagedCredential,
        retention_posture: RetentionStanceClass::NoRetentionPromisedBodyDiscarded,
        region_posture: RegionPostureClass::SingleRegionPinned,
        quota_family: QuotaFamilyClass::VendorHostedEntitlementQuota,
        execution_locus: ExecutionLocusClass::VendorHostedFirstPartyManaged,
        local_model_pack_provenance_ref: None,
        external_tool_locus_refs: vec!["tool-locus:docs-connector".to_owned()],
    }
}

fn preflight() -> PreflightEstimateCard {
    PreflightEstimateCard {
        shown_before_send: true,
        intended_route_class: RouteClass::Managed,
        estimated_cost_band: CostEnvelopeClass::VendorHostedEntitlementBand,
        estimated_latency_band: LatencyEnvelopeClass::StreamingFirstTokenUnder500Ms,
        quota_family_flow: AiActionFlowClass::Review,
        quota_family: QuotaFamilyClass::VendorHostedEntitlementQuota,
        local_resource_costs: vec![],
        approval_note_label: "One-time egress approval for managed review".to_owned(),
        policy_note_label: "Managed route allowed under the current policy epoch".to_owned(),
    }
}

fn live_run() -> LiveRunStrip {
    LiveRunStrip {
        present: true,
        phase: RunPhaseClass::Settled,
        current_route_class: RouteClass::Managed,
        current_provider_label: "Aureline managed hosted AI".to_owned(),
        current_model_label: "Hosted review model".to_owned(),
        route_disclosed: true,
    }
}

fn receipt() -> PostRunReceipt {
    PostRunReceipt {
        present: true,
        outcome: RunOutcomeClass::CompletedClean,
        actual_route_class: RouteClass::Managed,
        actual_cost_band: CostEnvelopeClass::VendorHostedEntitlementBand,
        cost_measurement: CostMeasurementClass::ActualMeasured,
        quota_family_flow: AiActionFlowClass::Review,
        quota_family: QuotaFamilyClass::VendorHostedEntitlementQuota,
        route_receipt_ref: "route-receipt:route-spend:stable:0001".to_owned(),
        spend_receipt_ref: "spend-receipt:route-spend:stable:0001".to_owned(),
    }
}

fn quota_summary() -> Vec<QuotaSummaryRow> {
    AiActionFlowClass::required_coverage()
        .into_iter()
        .map(|flow_class| QuotaSummaryRow {
            flow_class,
            quota_family: QuotaFamilyClass::VendorHostedEntitlementQuota,
            quota_scope: QuotaScopeClass::Workspace,
            quota_state: QuotaStateClass::WithinLimit,
            budget_owner_label: format!("Workspace budget for {}", flow_class.as_str()),
            blocked_this_action: false,
        })
        .collect()
}

fn downgrade() -> RouteDowngradeBanner {
    RouteDowngradeBanner {
        downgraded: false,
        cause: RouteChangeCauseClass::NoRouteChange,
        original_route_class: RouteClass::Managed,
        original_provider_label: "Aureline managed hosted AI".to_owned(),
        original_model_label: "Hosted review model".to_owned(),
        current_route_class: RouteClass::Managed,
        current_provider_label: "Aureline managed hosted AI".to_owned(),
        current_model_label: "Hosted review model".to_owned(),
        both_routes_preserved: true,
        silent_switch: false,
        disclosure_ref: None,
    }
}

fn fallback() -> NonAiFallbackPath {
    NonAiFallbackPath {
        available: true,
        fallback_label: "Open the manual review checklist".to_owned(),
        fallback_command_ref: "cmd:review.manual_checklist".to_owned(),
        reachable_without_ai: true,
    }
}

fn evidence_export() -> RouteSpendEvidenceExport {
    RouteSpendEvidenceExport {
        evidence_id: "ai-evidence:route-spend:stable:0001".to_owned(),
        json_export_ref: AI_ROUTE_SPEND_TRUTH_ARTIFACT_REF.to_owned(),
        markdown_summary_ref: AI_ROUTE_SPEND_TRUTH_SUMMARY_REF.to_owned(),
        admin_inspector_ref: "admin-inspector:route-spend:stable:0001".to_owned(),
        support_export_ref: "support-export:route-spend:stable:0001".to_owned(),
        export_lineage_refs: vec!["export:operator:route-spend:stable:0001".to_owned()],
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        AI_ROUTE_SPEND_TRUTH_AI_DOC_REF.to_owned(),
        AI_ROUTE_SPEND_TRUTH_RECEIPT_CONTRACT_REF.to_owned(),
        AI_ROUTE_SPEND_TRUTH_BUDGET_CONTRACT_REF.to_owned(),
        AI_ROUTE_SPEND_TRUTH_SCHEMA_REF.to_owned(),
    ]
}

fn input() -> AiRouteSpendTruthPacketInput {
    AiRouteSpendTruthPacketInput {
        packet_id: "ai-route-spend-truth:stable:0001".to_owned(),
        action_id: ACTION_ID.to_owned(),
        display_label: "AI route and spend truth".to_owned(),
        action_flow_class: AiActionFlowClass::Review,
        material_action: true,
        claimed_stable: true,
        trust_state_token: "trusted".to_owned(),
        policy_epoch_ref: "policy-epoch:stable:0004".to_owned(),
        registry_resolution: registry_resolution(),
        preflight: preflight(),
        live_run: live_run(),
        receipt: receipt(),
        quota_summary: quota_summary(),
        downgrade: downgrade(),
        fallback: fallback(),
        cumulative_spend: None,
        evidence_export: evidence_export(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-05-31T23:00:00Z".to_owned(),
    }
}

fn packet() -> AiRouteSpendTruthPacket {
    AiRouteSpendTruthPacket::new(input())
}

#[test]
fn route_spend_truth_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn material_action_requires_estimate_before_send() {
    let mut packet = packet();
    packet.preflight.shown_before_send = false;

    assert!(packet
        .validate()
        .contains(&AiRouteSpendTruthViolation::EstimateNotShownBeforeSend));
}

#[test]
fn material_action_requires_live_route_truth() {
    let mut packet = packet();
    packet.live_run.route_disclosed = false;

    assert!(packet
        .validate()
        .contains(&AiRouteSpendTruthViolation::LiveRouteTruthMissing));
}

#[test]
fn dispatched_action_requires_post_run_receipt() {
    let mut packet = packet();
    packet.receipt.route_receipt_ref = String::new();

    assert!(packet
        .validate()
        .contains(&AiRouteSpendTruthViolation::PostRunReceiptMissing));
}

#[test]
fn dispatched_receipt_must_be_actual_not_estimate() {
    let mut packet = packet();
    packet.receipt.cost_measurement = CostMeasurementClass::EstimateBand;

    assert!(packet
        .validate()
        .contains(&AiRouteSpendTruthViolation::PostRunReceiptMissing));
}

#[test]
fn registry_must_resolve() {
    let mut packet = packet();
    packet.registry_resolution.resolved = false;

    assert!(packet
        .validate()
        .contains(&AiRouteSpendTruthViolation::RegistryResolutionIncomplete));
}

#[test]
fn route_class_must_match_execution_locus() {
    let mut packet = packet();
    // A managed route label over a local locus is a hidden-locus violation.
    packet.registry_resolution.execution_locus = ExecutionLocusClass::LocalInProcess;

    assert!(packet
        .validate()
        .contains(&AiRouteSpendTruthViolation::RouteLocusInconsistent));
}

#[test]
fn local_route_requires_model_pack_provenance() {
    let mut packet = local_packet();
    packet.registry_resolution.local_model_pack_provenance_ref = None;

    assert!(packet
        .validate()
        .contains(&AiRouteSpendTruthViolation::LocalModelPackProvenanceMissing));
}

#[test]
fn local_route_requires_resource_cost_classes() {
    let mut packet = local_packet();
    packet.preflight.local_resource_costs = vec![];

    assert!(packet
        .validate()
        .contains(&AiRouteSpendTruthViolation::LocalResourceCostMissing));
}

#[test]
fn local_route_negligible_only_is_rejected() {
    let mut packet = local_packet();
    // Implying `Local` is free by marking every resource negligible is rejected.
    packet.preflight.local_resource_costs = vec![LocalResourceCostRow {
        resource_class: LocalResourceClass::WallTime,
        cost_band: ResourceCostBandClass::Negligible,
    }];

    assert!(packet
        .validate()
        .contains(&AiRouteSpendTruthViolation::LocalResourceCostMissing));
}

#[test]
fn quota_summary_must_cover_every_flow() {
    let mut packet = packet();
    packet
        .quota_summary
        .retain(|row| row.flow_class != AiActionFlowClass::ToolConnectorAssisted);

    assert!(packet
        .validate()
        .contains(&AiRouteSpendTruthViolation::QuotaFamilyCoverageMissing));
}

#[test]
fn blocked_owning_flow_must_match_outcome() {
    let mut packet = packet();
    // The owning review flow blocked the action but the receipt completed cleanly
    // with no downgrade — an inconsistent story.
    if let Some(row) = packet
        .quota_summary
        .iter_mut()
        .find(|row| row.flow_class == AiActionFlowClass::Review)
    {
        row.blocked_this_action = true;
        row.quota_state = QuotaStateClass::Exhausted;
    }

    assert!(packet
        .validate()
        .contains(&AiRouteSpendTruthViolation::QuotaSummaryInconsistent));
}

#[test]
fn downgrade_must_preserve_both_routes() {
    let mut packet = downgraded_packet();
    packet.downgrade.both_routes_preserved = false;

    assert!(packet
        .validate()
        .contains(&AiRouteSpendTruthViolation::DowngradeRoutesNotPreserved));
}

#[test]
fn silent_route_switch_is_rejected() {
    let mut packet = packet();
    packet.downgrade.silent_switch = true;

    assert!(packet
        .validate()
        .contains(&AiRouteSpendTruthViolation::SilentRouteSwitch));
}

#[test]
fn downgraded_packet_validates() {
    let packet = downgraded_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn non_ai_fallback_must_be_available() {
    let mut packet = packet();
    packet.fallback.available = false;

    assert!(packet
        .validate()
        .contains(&AiRouteSpendTruthViolation::NonAiFallbackMissing));
}

#[test]
fn agent_lane_without_cumulative_truth_narrows() {
    let mut packet = packet();
    packet.action_flow_class = AiActionFlowClass::AgentBackground;
    // A claimed-stable agent lane with no cumulative posture must narrow.
    packet.cumulative_spend = None;

    assert!(packet
        .validate()
        .contains(&AiRouteSpendTruthViolation::CumulativeSpendTruthMissing));
}

#[test]
fn visible_agent_lane_without_receipt_must_narrow() {
    let mut packet = packet();
    packet.action_flow_class = AiActionFlowClass::AgentBackground;
    packet.cumulative_spend = Some(CumulativeSpendPosture {
        lane_visible: true,
        lane_label: "Branch-agent run".to_owned(),
        cumulative_receipt_available: false,
        cumulative_spend_band: CostEnvelopeClass::EstimatedUnverifiedBand,
        remaining_budget_band: CostEnvelopeClass::EnvelopeUnknownUnverifiedCost,
        hop_count: 3,
        // Cannot show cumulative receipts but still claims Stable.
        qualification: StableQualificationClass::Stable,
        claimed_stable: true,
    });

    assert!(packet
        .validate()
        .contains(&AiRouteSpendTruthViolation::CumulativeSpendTruthMissing));
}

#[test]
fn agent_lane_with_cumulative_truth_validates() {
    let mut packet = packet();
    packet.action_flow_class = AiActionFlowClass::AgentBackground;
    packet.preflight.quota_family_flow = AiActionFlowClass::AgentBackground;
    packet.receipt.quota_family_flow = AiActionFlowClass::AgentBackground;
    packet.cumulative_spend = Some(CumulativeSpendPosture {
        lane_visible: true,
        lane_label: "Branch-agent run".to_owned(),
        cumulative_receipt_available: true,
        cumulative_spend_band: CostEnvelopeClass::VendorHostedEntitlementBand,
        remaining_budget_band: CostEnvelopeClass::VendorHostedEntitlementBand,
        hop_count: 3,
        qualification: StableQualificationClass::Stable,
        claimed_stable: true,
    });

    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_evidence_id_is_rejected() {
    let mut packet = packet();
    packet.evidence_export.evidence_id = String::new();

    assert!(packet
        .validate()
        .contains(&AiRouteSpendTruthViolation::EvidenceExportRefsMissing));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != AI_ROUTE_SPEND_TRUTH_SCHEMA_REF);

    assert!(packet
        .validate()
        .contains(&AiRouteSpendTruthViolation::MissingSourceContracts));
}

#[test]
fn raw_boundary_material_is_rejected() {
    let mut packet = packet();
    packet.preflight.policy_note_label = "route via https://provider.example/v1".to_owned();

    assert!(packet
        .validate()
        .contains(&AiRouteSpendTruthViolation::RawBoundaryMaterialInExport));
}

/// A local-route variant used by the local-route invariant tests.
fn local_packet() -> AiRouteSpendTruthPacket {
    let mut input = input();
    input.action_flow_class = AiActionFlowClass::Composer;
    input.preflight.quota_family_flow = AiActionFlowClass::Composer;
    input.receipt.quota_family_flow = AiActionFlowClass::Composer;
    input.registry_resolution.provider_id = "provider:local-pack".to_owned();
    input.registry_resolution.provider_label = "On-device model pack".to_owned();
    input.registry_resolution.model_id = "model:local-small".to_owned();
    input.registry_resolution.model_label = "On-device small model".to_owned();
    input.registry_resolution.transport_class = RegistryTransportClass::InProcessCall;
    input.registry_resolution.auth_mode = RegistryAuthModeClass::SignedManifestOnlyLocalPack;
    input.registry_resolution.retention_posture = RetentionStanceClass::NoRetentionLocalOnly;
    input.registry_resolution.region_posture = RegionPostureClass::LocalDeviceOnly;
    input.registry_resolution.quota_family = QuotaFamilyClass::PerUserLocalNoLimit;
    input.registry_resolution.execution_locus = ExecutionLocusClass::LocalInProcess;
    input.registry_resolution.local_model_pack_provenance_ref =
        Some("model-pack:provenance:local-small:0001".to_owned());
    input.registry_resolution.external_tool_locus_refs = vec![];
    input.preflight.intended_route_class = RouteClass::Local;
    input.preflight.estimated_cost_band = CostEnvelopeClass::BundledNoIncrementalCost;
    input.preflight.quota_family = QuotaFamilyClass::PerUserLocalNoLimit;
    input.preflight.local_resource_costs = vec![
        LocalResourceCostRow {
            resource_class: LocalResourceClass::WallTime,
            cost_band: ResourceCostBandClass::Moderate,
        },
        LocalResourceCostRow {
            resource_class: LocalResourceClass::Memory,
            cost_band: ResourceCostBandClass::High,
        },
        LocalResourceCostRow {
            resource_class: LocalResourceClass::Battery,
            cost_band: ResourceCostBandClass::Low,
        },
        LocalResourceCostRow {
            resource_class: LocalResourceClass::Accelerator,
            cost_band: ResourceCostBandClass::Constrained,
        },
    ];
    input.live_run.current_route_class = RouteClass::Local;
    input.live_run.current_provider_label = "On-device model pack".to_owned();
    input.live_run.current_model_label = "On-device small model".to_owned();
    input.receipt.actual_route_class = RouteClass::Local;
    input.receipt.actual_cost_band = CostEnvelopeClass::BundledNoIncrementalCost;
    input.receipt.quota_family = QuotaFamilyClass::PerUserLocalNoLimit;
    input.downgrade.original_route_class = RouteClass::Local;
    input.downgrade.current_route_class = RouteClass::Local;
    AiRouteSpendTruthPacket::new(input)
}

/// A downgraded-route variant used by the downgrade invariant tests.
fn downgraded_packet() -> AiRouteSpendTruthPacket {
    let mut input = input();
    input.preflight.intended_route_class = RouteClass::Byok;
    input.registry_resolution.execution_locus = ExecutionLocusClass::ByokRemoteVendorDirect;
    input.registry_resolution.auth_mode = RegistryAuthModeClass::ByokApiKey;
    input.registry_resolution.quota_family = QuotaFamilyClass::PerUserByokVendorQuota;
    input.preflight.quota_family = QuotaFamilyClass::PerUserByokVendorQuota;
    // The BYOK quota exhausted, so Aureline fell back to the managed route.
    input.receipt.outcome = RunOutcomeClass::CompletedWithDowngrade;
    input.receipt.actual_route_class = RouteClass::Managed;
    input.live_run.current_route_class = RouteClass::Managed;
    input.downgrade = RouteDowngradeBanner {
        downgraded: true,
        cause: RouteChangeCauseClass::FallbackAfterQuotaExhaustion,
        original_route_class: RouteClass::Byok,
        original_provider_label: "Connected vendor (BYOK)".to_owned(),
        original_model_label: "Vendor large model".to_owned(),
        current_route_class: RouteClass::Managed,
        current_provider_label: "Aureline managed hosted AI".to_owned(),
        current_model_label: "Hosted review model".to_owned(),
        both_routes_preserved: true,
        silent_switch: false,
        disclosure_ref: Some("route-change:route-spend:stable:0001".to_owned()),
    };
    // The owning review flow's BYOK quota blocked, forcing the downgrade.
    if let Some(row) = input
        .quota_summary
        .iter_mut()
        .find(|row| row.flow_class == AiActionFlowClass::Review)
    {
        row.quota_family = QuotaFamilyClass::PerUserByokVendorQuota;
        row.quota_scope = QuotaScopeClass::ByokProvider;
        row.quota_state = QuotaStateClass::Exhausted;
        row.blocked_this_action = true;
        row.budget_owner_label = "Your connected vendor account (BYOK)".to_owned();
    }
    AiRouteSpendTruthPacket::new(input)
}

#[test]
#[ignore = "run manually to regenerate the checked artifact"]
fn emit_artifact() {
    let packet = packet();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let dir = format!("{root}/artifacts/ai/m4/stabilize_ai_route_and_spend_truth");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        format!("{dir}/support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
    std::fs::write(format!("{dir}/summary.md"), packet.render_markdown_summary()).unwrap();
    let fixture_dir = format!("{root}/fixtures/ai/m4/stabilize_ai_route_and_spend_truth");
    std::fs::create_dir_all(&fixture_dir).unwrap();
    std::fs::write(
        format!("{fixture_dir}/route_spend_truth_packet.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
    // The downgrade fixture proves the quota-blocked, route-downgraded path.
    std::fs::write(
        format!("{fixture_dir}/route_downgrade_packet.json"),
        format!("{}\n", downgraded_packet().export_safe_json()),
    )
    .unwrap();
}

#[test]
fn checked_artifact_validates() {
    let packet = current_stable_ai_route_spend_truth_export()
        .expect("checked ai route/spend truth export validates");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}
