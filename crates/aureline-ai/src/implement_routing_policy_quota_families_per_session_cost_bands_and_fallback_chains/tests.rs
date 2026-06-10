use super::*;

const PACKET_ID: &str = "routing-policy:stable:0001";

fn proof_stale_to(narrowed_to: M5AiWorkflowQualificationClass) -> RoutingPolicyDowngradeRule {
    RoutingPolicyDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProofStale,
        narrowed_to,
        auto_enforced: true,
        rationale: "Stale proof narrows the public claim".to_owned(),
    }
}

fn provider_unavailable_to(
    narrowed_to: M5AiWorkflowQualificationClass,
) -> RoutingPolicyDowngradeRule {
    RoutingPolicyDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProviderUnavailable,
        narrowed_to,
        auto_enforced: true,
        rationale: "Quota or provider exhaustion narrows the claim".to_owned(),
    }
}

fn non_ai_terminal(order: u32) -> FallbackHop {
    FallbackHop {
        order,
        mode: RoutePolicyModeClass::Local,
        reason: FallbackHopReasonClass::NonAiTerminalFallback,
        outcome: FallbackHopOutcomeClass::TerminalReachable,
        label: "Manual command path reachable without any model".to_owned(),
    }
}

fn managed_stable_policy() -> RoutingPolicyRow {
    RoutingPolicyRow {
        policy_id: "composer-managed".to_owned(),
        surface_id: "composer".to_owned(),
        surface_label: "Composer inline assist".to_owned(),
        resolved_mode: RoutePolicyModeClass::Managed,
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        quota: QuotaInspector {
            family: QuotaFamilyClass::ManagedEntitlementQuota,
            state: QuotaStateClass::WithinLimit,
            scope: QuotaScopeClass::PerSession,
            budget_owner_label: "Managed plan entitlement".to_owned(),
            explanation_label: "Within the managed per-session entitlement".to_owned(),
        },
        session_cost_band: PerSessionCostBand {
            band: CostBandClass::FlatFeeSubscriptionBand,
            measurement: CostMeasurementClass::ActualMeasured,
            charged: ChargedDisclosureClass::ChargedUserSubscription,
            session_budget_owner_label: "Subscriber".to_owned(),
            exhausted_this_session: false,
            explanation_label: "Flat subscription; no incremental per-session charge".to_owned(),
        },
        fallback_chain: vec![
            FallbackHop {
                order: 0,
                mode: RoutePolicyModeClass::Managed,
                reason: FallbackHopReasonClass::PrimaryCheapestQualifying,
                outcome: FallbackHopOutcomeClass::Selected,
                label: "Managed flagship route".to_owned(),
            },
            FallbackHop {
                order: 1,
                mode: RoutePolicyModeClass::Local,
                reason: FallbackHopReasonClass::FallbackAfterProviderUnavailable,
                outcome: FallbackHopOutcomeClass::AvailableNotSelected,
                label: "On-device model when the managed route is unavailable".to_owned(),
            },
            non_ai_terminal(2),
        ],
        downgrade_rules: vec![
            proof_stale_to(M5AiWorkflowQualificationClass::Beta),
            provider_unavailable_to(M5AiWorkflowQualificationClass::Unavailable),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::FullyReversible,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:composer-managed:m5".to_owned()],
    }
}

fn byok_beta_policy() -> RoutingPolicyRow {
    RoutingPolicyRow {
        policy_id: "review-byok".to_owned(),
        surface_id: "review".to_owned(),
        surface_label: "Review assist".to_owned(),
        resolved_mode: RoutePolicyModeClass::Byok,
        claimed_qualification: M5AiWorkflowQualificationClass::Beta,
        quota: QuotaInspector {
            family: QuotaFamilyClass::PerUserByokVendorQuota,
            state: QuotaStateClass::Warning,
            scope: QuotaScopeClass::PerSession,
            budget_owner_label: "BYOK credential owner".to_owned(),
            explanation_label: "Approaching the per-session vendor quota".to_owned(),
        },
        session_cost_band: PerSessionCostBand {
            band: CostBandClass::MeteredMediumBand,
            measurement: CostMeasurementClass::ActualMeasured,
            charged: ChargedDisclosureClass::ChargedUserMetered,
            session_budget_owner_label: "BYOK credential owner".to_owned(),
            exhausted_this_session: false,
            explanation_label: "Metered against the user's own vendor account".to_owned(),
        },
        fallback_chain: vec![
            FallbackHop {
                order: 0,
                mode: RoutePolicyModeClass::Byok,
                reason: FallbackHopReasonClass::PrimaryCheapestQualifying,
                outcome: FallbackHopOutcomeClass::Selected,
                label: "BYOK vendor route".to_owned(),
            },
            FallbackHop {
                order: 1,
                mode: RoutePolicyModeClass::Local,
                reason: FallbackHopReasonClass::FallbackAfterBudgetExhausted,
                outcome: FallbackHopOutcomeClass::AvailableNotSelected,
                label: "On-device model after the per-session budget is spent".to_owned(),
            },
            non_ai_terminal(2),
        ],
        downgrade_rules: vec![
            proof_stale_to(M5AiWorkflowQualificationClass::Preview),
            provider_unavailable_to(M5AiWorkflowQualificationClass::Unavailable),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::CheckpointReversible,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:review-byok:m5".to_owned()],
    }
}

fn local_preview_policy() -> RoutingPolicyRow {
    RoutingPolicyRow {
        policy_id: "explain-local".to_owned(),
        surface_id: "explain".to_owned(),
        surface_label: "Explain flow".to_owned(),
        resolved_mode: RoutePolicyModeClass::Local,
        claimed_qualification: M5AiWorkflowQualificationClass::Preview,
        quota: QuotaInspector {
            family: QuotaFamilyClass::PerUserLocalUnmetered,
            state: QuotaStateClass::WithinLimit,
            scope: QuotaScopeClass::LocalDevice,
            budget_owner_label: "Local device".to_owned(),
            explanation_label: "On-device with no enforced ceiling".to_owned(),
        },
        session_cost_band: PerSessionCostBand {
            band: CostBandClass::BundledNoIncrementalCost,
            measurement: CostMeasurementClass::ActualMeasured,
            charged: ChargedDisclosureClass::NotChargedLocal,
            session_budget_owner_label: "Local device".to_owned(),
            exhausted_this_session: false,
            explanation_label: "Runs on-device with no per-session charge".to_owned(),
        },
        fallback_chain: vec![
            FallbackHop {
                order: 0,
                mode: RoutePolicyModeClass::Local,
                reason: FallbackHopReasonClass::PrimaryCheapestQualifying,
                outcome: FallbackHopOutcomeClass::Selected,
                label: "On-device model".to_owned(),
            },
            non_ai_terminal(1),
        ],
        downgrade_rules: vec![
            proof_stale_to(M5AiWorkflowQualificationClass::Experimental),
            provider_unavailable_to(M5AiWorkflowQualificationClass::Unavailable),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::FullyReversible,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:explain-local:m5".to_owned()],
    }
}

fn exhausted_held_policy() -> RoutingPolicyRow {
    RoutingPolicyRow {
        policy_id: "agent-byok".to_owned(),
        surface_id: "agent".to_owned(),
        surface_label: "Background agent".to_owned(),
        resolved_mode: RoutePolicyModeClass::Byok,
        claimed_qualification: M5AiWorkflowQualificationClass::Held,
        quota: QuotaInspector {
            family: QuotaFamilyClass::PerUserByokVendorQuota,
            state: QuotaStateClass::Exhausted,
            scope: QuotaScopeClass::PerSession,
            budget_owner_label: "BYOK credential owner".to_owned(),
            explanation_label: "Per-session vendor quota is exhausted".to_owned(),
        },
        session_cost_band: PerSessionCostBand {
            band: CostBandClass::MeteredHighBand,
            measurement: CostMeasurementClass::ActualMeasured,
            charged: ChargedDisclosureClass::ChargedUserMetered,
            session_budget_owner_label: "BYOK credential owner".to_owned(),
            exhausted_this_session: true,
            explanation_label: "Per-session budget spent; primary route paused".to_owned(),
        },
        fallback_chain: vec![
            FallbackHop {
                order: 0,
                mode: RoutePolicyModeClass::Byok,
                reason: FallbackHopReasonClass::PrimaryCheapestQualifying,
                outcome: FallbackHopOutcomeClass::ExhaustedSkipped,
                label: "BYOK vendor route skipped after exhaustion".to_owned(),
            },
            FallbackHop {
                order: 1,
                mode: RoutePolicyModeClass::Local,
                reason: FallbackHopReasonClass::FallbackAfterQuotaExhausted,
                outcome: FallbackHopOutcomeClass::AvailableNotSelected,
                label: "On-device model after vendor quota exhaustion".to_owned(),
            },
            non_ai_terminal(2),
        ],
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
        ROUTING_POLICY_SCHEMA_REF.to_owned(),
        ROUTING_POLICY_DOC_REF.to_owned(),
        PROVIDER_MODEL_REGISTRY_SCHEMA_REF.to_owned(),
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF.to_owned(),
    ]
}

fn proof_freshness() -> RoutingPolicyProofFreshness {
    RoutingPolicyProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-09T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> RoutingPolicyPacket {
    RoutingPolicyPacket::new(RoutingPolicyPacketInput {
        packet_id: PACKET_ID.to_owned(),
        catalogue_label: "Routing Policy, Quota, Per-Session Cost, And Fallback".to_owned(),
        policies: vec![
            managed_stable_policy(),
            byok_beta_policy(),
            local_preview_policy(),
            exhausted_held_policy(),
        ],
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-09T00:00:00Z".to_owned(),
    })
}

#[test]
fn routing_policy_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn mode_egress_partition() {
    assert!(!RoutePolicyModeClass::Local.is_egress());
    assert!(RoutePolicyModeClass::Byok.is_egress());
    assert!(RoutePolicyModeClass::Managed.is_egress());
    assert!(RoutePolicyModeClass::EnterpriseGateway.is_egress());
}

#[test]
fn quota_blocks_dispatch_partition() {
    assert!(QuotaStateClass::Exhausted.blocks_dispatch());
    assert!(QuotaStateClass::PausedByPolicy.blocks_dispatch());
    assert!(!QuotaStateClass::WithinLimit.blocks_dispatch());
    assert!(!QuotaStateClass::Warning.blocks_dispatch());
}

#[test]
fn charged_band_partition() {
    assert!(CostBandClass::MeteredLowBand.is_charged());
    assert!(CostBandClass::FlatFeeSubscriptionBand.is_charged());
    assert!(!CostBandClass::BundledNoIncrementalCost.is_charged());
    assert!(!CostBandClass::FreeTierRateLimited.is_charged());
    assert!(!CostBandClass::EstimatedUnverifiedBand.is_charged());
}

#[test]
fn counts_match_fixture() {
    let packet = packet();
    assert_eq!(packet.claimed_policy_count(), 3);
    assert_eq!(packet.charged_band_count(), 3);
    assert_eq!(packet.exhausted_policy_count(), 1);
    assert_eq!(packet.non_ai_fallback_count(), 4);
}

#[test]
fn no_policies_fails() {
    let mut packet = packet();
    packet.policies.clear();
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::NoPolicies));
}

#[test]
fn duplicate_policy_fails() {
    let mut packet = packet();
    let first = packet.policies[0].clone();
    packet.policies.push(first);
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::DuplicatePolicy));
}

#[test]
fn policy_row_incomplete_fails() {
    let mut packet = packet();
    packet.policies[0].quota.budget_owner_label.clear();
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::PolicyRowIncomplete));
}

#[test]
fn fallback_chain_empty_fails() {
    let mut packet = packet();
    packet.policies[0].fallback_chain.clear();
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::FallbackChainEmpty));
}

#[test]
fn fallback_chain_not_ordered_fails() {
    let mut packet = packet();
    // Break the strictly-increasing-from-zero ordering.
    packet.policies[0].fallback_chain[1].order = 5;
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::FallbackChainNotOrdered));
}

#[test]
fn missing_non_ai_terminal_fallback_fails() {
    let mut packet = packet();
    // Drop the terminal hop; the chain is now AI-only.
    let chain = &mut packet.policies[2].fallback_chain;
    chain.retain(|hop| !hop.reason.is_non_ai_terminal());
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::MissingNonAiTerminalFallback));
}

#[test]
fn multiple_selected_hops_fails() {
    let mut packet = packet();
    packet.policies[0].fallback_chain[1].outcome = FallbackHopOutcomeClass::Selected;
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::MultipleSelectedHops));
}

#[test]
fn claimed_policy_no_selected_hop_fails() {
    let mut packet = packet();
    packet.policies[0].fallback_chain[0].outcome = FallbackHopOutcomeClass::AvailableNotSelected;
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::ClaimedPolicyNoSelectedHop));
}

#[test]
fn selected_hop_mode_mismatch_fails() {
    let mut packet = packet();
    // The selected hop now routes somewhere other than the resolved mode.
    packet.policies[0].fallback_chain[0].mode = RoutePolicyModeClass::Byok;
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::SelectedHopModeMismatch));
}

#[test]
fn charged_band_undisclosed_fails() {
    let mut packet = packet();
    packet.policies[1].session_cost_band.charged = ChargedDisclosureClass::ChargeUnknownUnverified;
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::ChargedBandUndisclosed));
}

#[test]
fn estimated_band_claims_stable_fails() {
    let mut packet = packet();
    packet.policies[0].session_cost_band.measurement = CostMeasurementClass::EstimateBand;
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::EstimatedBandClaimsStable));
}

#[test]
fn exhausted_policy_claims_stable_fails() {
    let mut packet = packet();
    // A Stable surface whose per-session budget is spent must have narrowed.
    packet.policies[0].session_cost_band.exhausted_this_session = true;
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::ExhaustedPolicyClaimsStable));
}

#[test]
fn claimed_policy_missing_evidence_fails() {
    let mut packet = packet();
    packet.policies[0].evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::ClaimedPolicyMissingEvidence));
}

#[test]
fn claimed_rollback_unverified_fails() {
    let mut packet = packet();
    packet.policies[0].rollback_verified = false;
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::ClaimedRollbackUnverified));
}

#[test]
fn downgrade_rules_missing_fails() {
    let mut packet = packet();
    packet.policies[0].downgrade_rules.clear();
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::DowngradeRulesMissing));
}

#[test]
fn downgrade_rule_missing_proof_stale_fails() {
    let mut packet = packet();
    packet.policies[0]
        .downgrade_rules
        .retain(|rule| rule.trigger != M5AiWorkflowDowngradeTrigger::ProofStale);
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::DowngradeRuleMissingProofStale));
}

#[test]
fn downgrade_rule_missing_provider_unavailable_fails() {
    let mut packet = packet();
    packet.policies[0]
        .downgrade_rules
        .retain(|rule| rule.trigger != M5AiWorkflowDowngradeTrigger::ProviderUnavailable);
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::DowngradeRuleMissingProviderUnavailable));
}

#[test]
fn downgrade_rule_not_narrowing_fails() {
    let mut packet = packet();
    // Narrowing "to" Stable from a Stable claim does not narrow.
    packet.policies[0].downgrade_rules[0].narrowed_to = M5AiWorkflowQualificationClass::Stable;
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::DowngradeRuleNotNarrowing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::MissingSourceContracts));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::ProofFreshnessIncomplete));
}

#[test]
fn raw_boundary_material_in_export_fails() {
    let mut packet = packet();
    // A raw provider endpoint must never cross the support boundary.
    packet.policies[1].quota.explanation_label = "https://api.vendor.example/quota".to_owned();
    assert!(packet
        .validate()
        .contains(&RoutingPolicyViolation::RawBoundaryMaterialInExport));
}

#[test]
fn narrowed_qualification_applies_matching_rule() {
    let policy = byok_beta_policy();
    assert_eq!(
        policy.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProofStale),
        M5AiWorkflowQualificationClass::Preview
    );
    assert_eq!(
        policy.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProviderUnavailable),
        M5AiWorkflowQualificationClass::Unavailable
    );
    // A trigger with no rule leaves the claim unchanged.
    assert_eq!(
        policy.narrowed_qualification(M5AiWorkflowDowngradeTrigger::TrustNarrowing),
        M5AiWorkflowQualificationClass::Beta
    );
}

#[test]
fn render_inspector_lists_every_posture() {
    let card = byok_beta_policy().render_inspector();
    assert!(card.contains("review-byok"));
    assert!(card.contains("byok"));
    assert!(card.contains("per_user_byok_vendor_quota"));
    assert!(card.contains("metered_medium_band"));
    assert!(card.contains("charged_user_metered"));
    assert!(card.contains("non_ai_terminal_fallback"));
}

#[test]
fn markdown_summary_lists_every_policy() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("Policy inspectors"));
    for policy in &packet().policies {
        assert!(
            summary.contains(&policy.policy_id),
            "missing {}",
            policy.policy_id
        );
    }
}

#[test]
fn quota_exhausted_fallback_fixture_validates() {
    let packet: RoutingPolicyPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains/quota_exhausted_fallback_to_local.json"
    )))
    .expect("quota-exhausted fixture parses");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // The exhausted surface is no longer claimed, falls back to a local hop, and
    // keeps a reachable non-AI terminal fallback.
    let exhausted: Vec<&RoutingPolicyRow> = packet
        .policies
        .iter()
        .filter(|policy| policy.is_exhausted())
        .collect();
    assert!(!exhausted.is_empty());
    for policy in exhausted {
        assert!(!policy.is_claimed());
        assert!(policy.has_non_ai_terminal_fallback());
        assert_eq!(
            policy.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProviderUnavailable),
            M5AiWorkflowQualificationClass::Unavailable
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_routing_policy_export().expect("checked routing-policy export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
    assert!(!packet.policies.is_empty());
    assert!(packet.non_ai_fallback_count() == packet.policies.len());
    // Every surface carries a non-AI terminal fallback.
    assert!(packet
        .policies
        .iter()
        .all(|policy| policy.has_non_ai_terminal_fallback()));
}
