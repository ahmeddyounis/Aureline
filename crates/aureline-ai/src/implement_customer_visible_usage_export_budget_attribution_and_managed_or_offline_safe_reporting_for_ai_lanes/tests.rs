use super::*;

const PACKET_ID: &str = "usage-reporting:stable:0001";

fn proof_stale_to(narrowed_to: M5AiWorkflowQualificationClass) -> UsageReportDowngradeRule {
    UsageReportDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProofStale,
        narrowed_to,
        auto_enforced: true,
        rationale: "Stale proof narrows the public claim".to_owned(),
    }
}

fn provider_unavailable_to(
    narrowed_to: M5AiWorkflowQualificationClass,
) -> UsageReportDowngradeRule {
    UsageReportDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProviderUnavailable,
        narrowed_to,
        auto_enforced: true,
        rationale: "Managed reporting outage narrows the claim".to_owned(),
    }
}

fn line(
    dimension: AttributionDimensionClass,
    cost_band: CostBandClass,
    charged: ChargedDisclosureClass,
    share: AttributionShareClass,
) -> BudgetAttributionLine {
    BudgetAttributionLine {
        dimension,
        subject_label: format!("{} subject", dimension.as_str()),
        cost_band,
        charged,
        share,
        notes_label: String::new(),
    }
}

/// A managed Stable report served live with a verified offline fallback.
fn managed_stable_report() -> UsageReportRow {
    UsageReportRow {
        report_id: "managed-usage-report".to_owned(),
        lane_id: "lane:composer-managed".to_owned(),
        lane_label: "Composer managed lane".to_owned(),
        reporting_period_label: "Current billing month".to_owned(),
        resolved_mode: RoutePolicyModeClass::Managed,
        quota_family: QuotaFamilyClass::ManagedEntitlementQuota,
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        continuity: ReportingContinuityClass::ManagedWithOfflineFallback,
        offline_generatable: true,
        generation_state: ReportGenerationStateClass::Generated,
        region: RouteRegionClass::SingleRegionPinned,
        retention: RouteRetentionClass::BoundedRetentionWithExport,
        usage_export: UsageExportBlock {
            availability: ExportAvailabilityClass::AvailableNow,
            customer_visible: true,
            completeness: ReportCompletenessClass::Complete,
            redaction: ExportRedactionClass::CoarseBandsAndAggregated,
            format_label: "CSV and JSON line items".to_owned(),
            explanation_label: "Customer can download the full period export".to_owned(),
        },
        budget_attribution: BudgetAttributionBlock {
            total_cost_band: CostBandClass::MeteredMediumBand,
            measurement: CostMeasurementClass::ActualMeasured,
            charged: ChargedDisclosureClass::ChargedUserSubscription,
            billing_owner_label: "Managed plan subscriber".to_owned(),
            lines: vec![
                line(
                    AttributionDimensionClass::PerWorkspace,
                    CostBandClass::MeteredMediumBand,
                    ChargedDisclosureClass::ChargedUserSubscription,
                    AttributionShareClass::Dominant,
                ),
                line(
                    AttributionDimensionClass::PerModel,
                    CostBandClass::MeteredLowBand,
                    ChargedDisclosureClass::ChargedUserSubscription,
                    AttributionShareClass::Minor,
                ),
            ],
            explanation_label: "Spend split by workspace and model".to_owned(),
        },
        downgrade_rules: vec![
            proof_stale_to(M5AiWorkflowQualificationClass::Beta),
            provider_unavailable_to(M5AiWorkflowQualificationClass::Preview),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::FullyReversible,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:managed-usage-report:m5".to_owned()],
    }
}

/// A BYOK Beta report whose managed service was down and was served offline.
fn byok_offline_report() -> UsageReportRow {
    UsageReportRow {
        report_id: "byok-offline-report".to_owned(),
        lane_id: "lane:review-byok".to_owned(),
        lane_label: "Review BYOK lane".to_owned(),
        reporting_period_label: "Last seven days".to_owned(),
        resolved_mode: RoutePolicyModeClass::Byok,
        quota_family: QuotaFamilyClass::PerUserByokVendorQuota,
        claimed_qualification: M5AiWorkflowQualificationClass::Beta,
        continuity: ReportingContinuityClass::OfflineSafe,
        offline_generatable: true,
        generation_state: ReportGenerationStateClass::ManagedUnavailableUsedOffline,
        region: RouteRegionClass::VendorDefaultUnpinned,
        retention: RouteRetentionClass::NoRetentionPromised,
        usage_export: UsageExportBlock {
            availability: ExportAvailabilityClass::OnRequest,
            customer_visible: true,
            completeness: ReportCompletenessClass::Complete,
            redaction: ExportRedactionClass::CoarseBandsOnly,
            format_label: "JSON line items".to_owned(),
            explanation_label: "Served from the offline fallback while managed was down".to_owned(),
        },
        budget_attribution: BudgetAttributionBlock {
            total_cost_band: CostBandClass::MeteredLowBand,
            measurement: CostMeasurementClass::ActualMeasured,
            charged: ChargedDisclosureClass::ChargedUserMetered,
            billing_owner_label: "BYOK credential owner".to_owned(),
            lines: vec![line(
                AttributionDimensionClass::PerUser,
                CostBandClass::MeteredLowBand,
                ChargedDisclosureClass::ChargedUserMetered,
                AttributionShareClass::Dominant,
            )],
            explanation_label: "Spend attributed to the BYOK user".to_owned(),
        },
        downgrade_rules: vec![
            proof_stale_to(M5AiWorkflowQualificationClass::Preview),
            provider_unavailable_to(M5AiWorkflowQualificationClass::Experimental),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::CheckpointReversible,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:byok-offline-report:m5".to_owned()],
    }
}

/// A local Preview report generated entirely on-device.
fn local_preview_report() -> UsageReportRow {
    UsageReportRow {
        report_id: "local-usage-report".to_owned(),
        lane_id: "lane:explain-local".to_owned(),
        lane_label: "Explain local lane".to_owned(),
        reporting_period_label: "Current session".to_owned(),
        resolved_mode: RoutePolicyModeClass::Local,
        quota_family: QuotaFamilyClass::PerUserLocalUnmetered,
        claimed_qualification: M5AiWorkflowQualificationClass::Preview,
        continuity: ReportingContinuityClass::LocalOnly,
        offline_generatable: true,
        generation_state: ReportGenerationStateClass::Generated,
        region: RouteRegionClass::OnDeviceOnly,
        retention: RouteRetentionClass::NoRetentionLocalOnly,
        usage_export: UsageExportBlock {
            availability: ExportAvailabilityClass::AvailableNow,
            customer_visible: true,
            completeness: ReportCompletenessClass::Complete,
            redaction: ExportRedactionClass::AggregatedPerDimension,
            format_label: "On-device JSON".to_owned(),
            explanation_label: "Generated locally with no managed service".to_owned(),
        },
        budget_attribution: BudgetAttributionBlock {
            total_cost_band: CostBandClass::BundledNoIncrementalCost,
            measurement: CostMeasurementClass::ActualMeasured,
            charged: ChargedDisclosureClass::NotChargedLocal,
            billing_owner_label: "Local device".to_owned(),
            lines: vec![line(
                AttributionDimensionClass::PerSession,
                CostBandClass::BundledNoIncrementalCost,
                ChargedDisclosureClass::NotChargedLocal,
                AttributionShareClass::Dominant,
            )],
            explanation_label: "No incremental cost for on-device usage".to_owned(),
        },
        downgrade_rules: vec![
            proof_stale_to(M5AiWorkflowQualificationClass::Experimental),
            provider_unavailable_to(M5AiWorkflowQualificationClass::Experimental),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::FullyReversible,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:local-usage-report:m5".to_owned()],
    }
}

/// A managed report whose managed service was down with no offline fallback.
fn managed_unavailable_held_report() -> UsageReportRow {
    UsageReportRow {
        report_id: "managed-unavailable-report".to_owned(),
        lane_id: "lane:background-managed".to_owned(),
        lane_label: "Background managed lane".to_owned(),
        reporting_period_label: "Current billing month".to_owned(),
        resolved_mode: RoutePolicyModeClass::Managed,
        quota_family: QuotaFamilyClass::ManagedEntitlementQuota,
        claimed_qualification: M5AiWorkflowQualificationClass::Held,
        continuity: ReportingContinuityClass::ManagedOnly,
        offline_generatable: false,
        generation_state: ReportGenerationStateClass::ManagedUnavailableNoFallback,
        region: RouteRegionClass::SingleRegionPinned,
        retention: RouteRetentionClass::BoundedRetentionOperatorOnly,
        usage_export: UsageExportBlock {
            availability: ExportAvailabilityClass::Unavailable,
            customer_visible: true,
            completeness: ReportCompletenessClass::PartialDegraded,
            redaction: ExportRedactionClass::CoarseBandsAndAggregated,
            format_label: "CSV line items".to_owned(),
            explanation_label: "Managed reporting service unavailable; no offline fallback"
                .to_owned(),
        },
        budget_attribution: BudgetAttributionBlock {
            total_cost_band: CostBandClass::EstimatedUnverifiedBand,
            measurement: CostMeasurementClass::EstimateBand,
            charged: ChargedDisclosureClass::ChargedUserSubscription,
            billing_owner_label: "Managed plan subscriber".to_owned(),
            lines: vec![line(
                AttributionDimensionClass::PerWorkspace,
                CostBandClass::EstimatedUnverifiedBand,
                ChargedDisclosureClass::ChargedUserSubscription,
                AttributionShareClass::Dominant,
            )],
            explanation_label: "Estimated pending managed reporting recovery".to_owned(),
        },
        downgrade_rules: vec![
            proof_stale_to(M5AiWorkflowQualificationClass::Unavailable),
            provider_unavailable_to(M5AiWorkflowQualificationClass::Unavailable),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::EvidencePreservedNoRevert,
        rollback_verified: false,
        evidence_packet_refs: vec![],
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        USAGE_REPORTING_SCHEMA_REF.to_owned(),
        USAGE_REPORTING_DOC_REF.to_owned(),
        PROVIDER_MODEL_REGISTRY_SCHEMA_REF.to_owned(),
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF.to_owned(),
        ROUTING_POLICY_SCHEMA_REF.to_owned(),
    ]
}

fn proof_freshness() -> UsageReportingProofFreshness {
    UsageReportingProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-09T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> UsageReportingPacket {
    UsageReportingPacket::new(UsageReportingPacketInput {
        packet_id: PACKET_ID.to_owned(),
        catalogue_label: "Customer-Visible AI Usage Reporting".to_owned(),
        reports: vec![
            managed_stable_report(),
            byok_offline_report(),
            local_preview_report(),
            managed_unavailable_held_report(),
        ],
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-09T00:00:00Z".to_owned(),
    })
}

#[test]
fn usage_reporting_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn continuity_offline_partition() {
    assert!(ReportingContinuityClass::OfflineSafe.requires_offline_capability());
    assert!(ReportingContinuityClass::ManagedWithOfflineFallback.requires_offline_capability());
    assert!(ReportingContinuityClass::LocalOnly.requires_offline_capability());
    assert!(!ReportingContinuityClass::ManagedOnly.requires_offline_capability());
    assert!(ReportingContinuityClass::ManagedOnly.is_managed_dependent());
    assert!(ReportingContinuityClass::ManagedWithOfflineFallback.is_managed_dependent());
    assert!(!ReportingContinuityClass::OfflineSafe.is_managed_dependent());
}

#[test]
fn generation_state_stable_partition() {
    assert!(ReportGenerationStateClass::Generated.backs_stable_claim());
    assert!(ReportGenerationStateClass::ManagedUnavailableUsedOffline.backs_stable_claim());
    assert!(!ReportGenerationStateClass::DegradedPartial.backs_stable_claim());
    assert!(!ReportGenerationStateClass::ManagedUnavailableNoFallback.backs_stable_claim());
    assert!(!ReportGenerationStateClass::Pending.backs_stable_claim());
    assert!(ReportGenerationStateClass::ManagedUnavailableNoFallback.is_unavailable());
}

#[test]
fn counts_match_fixture() {
    let packet = packet();
    assert_eq!(packet.claimed_report_count(), 3);
    // The byok offline-fallback report and the managed-unavailable report show
    // degraded state.
    assert_eq!(packet.degraded_report_count(), 2);
    // Every report but the managed-unavailable one is generatable offline.
    assert_eq!(packet.offline_capable_report_count(), 3);
}

#[test]
fn no_reports_fails() {
    let mut packet = packet();
    packet.reports.clear();
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::NoReports));
}

#[test]
fn duplicate_report_fails() {
    let mut packet = packet();
    let first = packet.reports[0].clone();
    packet.reports.push(first);
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::DuplicateReport));
}

#[test]
fn report_row_incomplete_fails() {
    let mut packet = packet();
    packet.reports[0]
        .budget_attribution
        .billing_owner_label
        .clear();
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::ReportRowIncomplete));
}

#[test]
fn attribution_lines_missing_fails() {
    let mut packet = packet();
    packet.reports[0].budget_attribution.lines.clear();
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::AttributionLinesMissing));
}

#[test]
fn attribution_line_exceeds_total_fails() {
    let mut packet = packet();
    // A slice priced above the period total cannot reconcile.
    packet.reports[0].budget_attribution.lines[1].cost_band = CostBandClass::MeteredHighBand;
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::AttributionLineExceedsTotal));
}

#[test]
fn attribution_charge_undisclosed_fails() {
    let mut packet = packet();
    packet.reports[0].budget_attribution.charged = ChargedDisclosureClass::ChargeUnknownUnverified;
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::AttributionChargeUndisclosed));
}

#[test]
fn estimated_attribution_claims_stable_fails() {
    let mut packet = packet();
    packet.reports[0].budget_attribution.measurement = CostMeasurementClass::EstimateBand;
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::EstimatedAttributionClaimsStable));
}

#[test]
fn export_label_missing_fails() {
    let mut packet = packet();
    packet.reports[0].usage_export.format_label.clear();
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::ExportLabelMissing));
}

#[test]
fn export_available_but_report_unavailable_fails() {
    let mut packet = packet();
    // The unavailable report cannot also advertise a reachable export.
    packet.reports[3].usage_export.availability = ExportAvailabilityClass::AvailableNow;
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::ExportAvailableButReportUnavailable));
}

#[test]
fn incomplete_export_claims_stable_fails() {
    let mut packet = packet();
    packet.reports[0].usage_export.completeness = ReportCompletenessClass::PartialDegraded;
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::IncompleteExportClaimsStable));
}

#[test]
fn offline_continuity_not_generatable_fails() {
    let mut packet = packet();
    // An offline-safe lane that is not generatable offline is dishonest.
    packet.reports[1].offline_generatable = false;
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::OfflineContinuityNotGeneratable));
}

#[test]
fn local_route_managed_only_fails() {
    let mut packet = packet();
    packet.reports[2].continuity = ReportingContinuityClass::ManagedOnly;
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::LocalRouteManagedOnly));
}

#[test]
fn offline_fallback_without_capability_fails() {
    let mut packet = packet();
    // An offline fallback served the report, but the lane is managed-only with no
    // offline capability.
    packet.reports[1].continuity = ReportingContinuityClass::ManagedOnly;
    packet.reports[1].offline_generatable = false;
    let violations = packet.validate();
    assert!(violations.contains(&UsageReportingViolation::OfflineFallbackWithoutCapability));
}

#[test]
fn degraded_report_claims_stable_fails() {
    let mut packet = packet();
    // A Stable report that degraded to partial must have narrowed.
    packet.reports[0].generation_state = ReportGenerationStateClass::DegradedPartial;
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::DegradedReportClaimsStable));
}

#[test]
fn local_route_region_retention_mismatch_fails() {
    let mut packet = packet();
    packet.reports[2].retention = RouteRetentionClass::BoundedRetentionWithExport;
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::LocalRouteRegionRetentionMismatch));
}

#[test]
fn region_locality_mismatch_fails() {
    let mut packet = packet();
    // A managed (egress) lane may not claim its bytes stay on-device.
    packet.reports[0].region = RouteRegionClass::OnDeviceOnly;
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::RegionLocalityMismatch));
}

#[test]
fn policy_blocked_claims_stable_fails() {
    let mut packet = packet();
    packet.reports[0].region = RouteRegionClass::PolicyBlocked;
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::PolicyBlockedClaimsStable));
}

#[test]
fn claimed_report_missing_evidence_fails() {
    let mut packet = packet();
    packet.reports[0].evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::ClaimedReportMissingEvidence));
}

#[test]
fn claimed_rollback_unverified_fails() {
    let mut packet = packet();
    packet.reports[0].rollback_verified = false;
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::ClaimedRollbackUnverified));
}

#[test]
fn downgrade_rules_missing_fails() {
    let mut packet = packet();
    packet.reports[0].downgrade_rules.clear();
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::DowngradeRulesMissing));
}

#[test]
fn downgrade_rule_missing_proof_stale_fails() {
    let mut packet = packet();
    packet.reports[0]
        .downgrade_rules
        .retain(|rule| rule.trigger != M5AiWorkflowDowngradeTrigger::ProofStale);
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::DowngradeRuleMissingProofStale));
}

#[test]
fn downgrade_rule_missing_provider_unavailable_fails() {
    let mut packet = packet();
    packet.reports[0]
        .downgrade_rules
        .retain(|rule| rule.trigger != M5AiWorkflowDowngradeTrigger::ProviderUnavailable);
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::DowngradeRuleMissingProviderUnavailable));
}

#[test]
fn downgrade_rule_not_narrowing_fails() {
    let mut packet = packet();
    packet.reports[0].downgrade_rules[0].narrowed_to = M5AiWorkflowQualificationClass::Stable;
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::DowngradeRuleNotNarrowing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::MissingSourceContracts));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::ProofFreshnessIncomplete));
}

#[test]
fn raw_boundary_material_in_export_fails() {
    let mut packet = packet();
    packet.reports[1].usage_export.explanation_label =
        "https://api.vendor.example/usage".to_owned();
    assert!(packet
        .validate()
        .contains(&UsageReportingViolation::RawBoundaryMaterialInExport));
}

#[test]
fn narrowed_qualification_applies_matching_rule() {
    let report = managed_stable_report();
    assert_eq!(
        report.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProofStale),
        M5AiWorkflowQualificationClass::Beta
    );
    assert_eq!(
        report.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProviderUnavailable),
        M5AiWorkflowQualificationClass::Preview
    );
    // A trigger with no rule leaves the claim unchanged.
    assert_eq!(
        report.narrowed_qualification(M5AiWorkflowDowngradeTrigger::TrustNarrowing),
        M5AiWorkflowQualificationClass::Stable
    );
}

#[test]
fn attribution_line_lookup_finds_dimension() {
    let report = managed_stable_report();
    assert!(report
        .attribution_line(AttributionDimensionClass::PerWorkspace)
        .is_some());
    assert!(report
        .attribution_line(AttributionDimensionClass::PerProvider)
        .is_none());
}

#[test]
fn render_inspector_lists_every_posture() {
    let card = byok_offline_report().render_inspector();
    assert!(card.contains("byok-offline-report"));
    assert!(card.contains("offline_safe"));
    assert!(card.contains("managed_unavailable_used_offline"));
    assert!(card.contains("metered_low_band"));
    assert!(card.contains("per_user"));
    assert!(card.contains("on_request"));
}

#[test]
fn markdown_summary_lists_every_report() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("Report inspectors"));
    for report in &packet().reports {
        assert!(
            summary.contains(&report.report_id),
            "missing {}",
            report.report_id
        );
    }
}

#[test]
fn managed_outage_offline_fixture_validates() {
    let packet: UsageReportingPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/implement_customer_visible_usage_export_budget_attribution_and_managed_or_offline_safe_reporting_for_ai_lanes/managed_outage_offline_fallback.json"
    )))
    .expect("managed outage offline fallback fixture parses");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // The offline-safe lane served the report from its offline fallback while the
    // managed service was down, and it keeps its claim because a clean offline
    // fallback is the continuity feature working.
    let offline = packet
        .report("offline-fallback-report")
        .expect("offline fallback report present");
    assert!(offline.is_claimed());
    assert!(offline.offline_generatable);
    assert_eq!(
        offline.generation_state,
        ReportGenerationStateClass::ManagedUnavailableUsedOffline
    );
    // The managed-only lane with no fallback dropped out of every claimed lane.
    let stranded = packet
        .report("managed-only-stranded-report")
        .expect("managed-only stranded report present");
    assert!(!stranded.is_claimed());
    assert!(stranded.has_degraded_state());
}

#[test]
fn checked_support_export_validates() {
    let packet =
        current_usage_reporting_export().expect("checked usage reporting export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
    assert!(!packet.reports.is_empty());
    // The managed-unavailable report dropped out of every claimed lane.
    let unavailable = packet
        .report("managed-unavailable-report")
        .expect("managed unavailable report present");
    assert!(!unavailable.is_claimed());
    assert!(unavailable.has_degraded_state());
    assert_eq!(
        unavailable.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProviderUnavailable),
        M5AiWorkflowQualificationClass::Unavailable
    );
}
