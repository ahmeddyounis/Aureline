use super::*;

const PACKET_ID: &str = "agent-budget:stable:0001";

fn proof_stale_to(narrowed_to: M5AiWorkflowQualificationClass) -> AgentBudgetDowngradeRule {
    AgentBudgetDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProofStale,
        narrowed_to,
        auto_enforced: true,
        rationale: "Stale proof narrows the public claim".to_owned(),
    }
}

fn provider_unavailable_to(
    narrowed_to: M5AiWorkflowQualificationClass,
) -> AgentBudgetDowngradeRule {
    AgentBudgetDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProviderUnavailable,
        narrowed_to,
        auto_enforced: true,
        rationale: "Quota or provider exhaustion narrows the claim".to_owned(),
    }
}

fn ceiling(
    kind: CeilingKindClass,
    consumption: CeilingConsumptionClass,
    enforcement: CeilingEnforcementClass,
    stop_outcome: CeilingStopOutcomeClass,
) -> AgentCeiling {
    AgentCeiling {
        kind,
        consumption,
        enforcement,
        stop_outcome,
        limit_label: format!("{} budget for the run", kind.as_str()),
        warning_threshold_label: format!("Warns at 80% of the {} budget", kind.as_str()),
        explanation_label: format!("{} ceiling for the run", kind.as_str()),
    }
}

fn managed_stable_budget() -> AgentBudgetRow {
    AgentBudgetRow {
        budget_id: "composer-refactor-agent".to_owned(),
        agent_run_id: "run:composer-refactor".to_owned(),
        agent_label: "Composer refactor agent".to_owned(),
        resolved_mode: RoutePolicyModeClass::Managed,
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        run_state: LongRunningAgentRunStateClass::CompletedWithinBudget,
        spend_ledger: SpendLedger {
            running_cost_band: CostBandClass::MeteredMediumBand,
            measurement: CostMeasurementClass::ActualMeasured,
            charged: ChargedDisclosureClass::ChargedUserSubscription,
            budget_owner_label: "Managed plan entitlement".to_owned(),
            budget_exhausted: false,
            entries: vec![
                SpendLedgerEntry {
                    sequence: 0,
                    phase_label: "Plan".to_owned(),
                    delta_cost_band: CostBandClass::MeteredLowBand,
                    running_cost_band: CostBandClass::MeteredLowBand,
                    notes_label: "Planning phase".to_owned(),
                },
                SpendLedgerEntry {
                    sequence: 1,
                    phase_label: "Edit".to_owned(),
                    delta_cost_band: CostBandClass::MeteredLowBand,
                    running_cost_band: CostBandClass::MeteredMediumBand,
                    notes_label: "Edit phase".to_owned(),
                },
            ],
            explanation_label: "Metered against the managed entitlement".to_owned(),
        },
        quota_warning: QuotaWarningSurface {
            family: QuotaFamilyClass::ManagedEntitlementQuota,
            state: QuotaStateClass::WithinLimit,
            scope: QuotaScopeClass::PerSession,
            warning_surfaced: false,
            warning_threshold_label: "Notifies at 80% of the managed entitlement".to_owned(),
            budget_owner_label: "Managed plan entitlement".to_owned(),
            explanation_label: "Within the managed per-session entitlement".to_owned(),
        },
        ceilings: vec![
            ceiling(
                CeilingKindClass::WallClock,
                CeilingConsumptionClass::WellWithinLimit,
                CeilingEnforcementClass::HardStopOnReach,
                CeilingStopOutcomeClass::NotTriggered,
            ),
            ceiling(
                CeilingKindClass::TokenBudget,
                CeilingConsumptionClass::ApproachingWarning,
                CeilingEnforcementClass::HardStopOnReach,
                CeilingStopOutcomeClass::NotTriggered,
            ),
            ceiling(
                CeilingKindClass::ToolCallCount,
                CeilingConsumptionClass::WellWithinLimit,
                CeilingEnforcementClass::HardStopOnReach,
                CeilingStopOutcomeClass::NotTriggered,
            ),
            ceiling(
                CeilingKindClass::CostBudget,
                CeilingConsumptionClass::WellWithinLimit,
                CeilingEnforcementClass::HardStopOnReach,
                CeilingStopOutcomeClass::NotTriggered,
            ),
        ],
        downgrade_rules: vec![
            proof_stale_to(M5AiWorkflowQualificationClass::Beta),
            provider_unavailable_to(M5AiWorkflowQualificationClass::Unavailable),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::FullyReversible,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:composer-refactor-agent:m5".to_owned()],
    }
}

fn byok_beta_budget() -> AgentBudgetRow {
    AgentBudgetRow {
        budget_id: "review-byok-agent".to_owned(),
        agent_run_id: "run:review-byok".to_owned(),
        agent_label: "Review BYOK agent".to_owned(),
        resolved_mode: RoutePolicyModeClass::Byok,
        claimed_qualification: M5AiWorkflowQualificationClass::Beta,
        run_state: LongRunningAgentRunStateClass::PausedAwaitingUser,
        spend_ledger: SpendLedger {
            running_cost_band: CostBandClass::MeteredMediumBand,
            measurement: CostMeasurementClass::ActualMeasured,
            charged: ChargedDisclosureClass::ChargedUserMetered,
            budget_owner_label: "BYOK credential owner".to_owned(),
            budget_exhausted: false,
            entries: vec![
                SpendLedgerEntry {
                    sequence: 0,
                    phase_label: "Scan".to_owned(),
                    delta_cost_band: CostBandClass::MeteredLowBand,
                    running_cost_band: CostBandClass::MeteredLowBand,
                    notes_label: String::new(),
                },
                SpendLedgerEntry {
                    sequence: 1,
                    phase_label: "Review".to_owned(),
                    delta_cost_band: CostBandClass::MeteredLowBand,
                    running_cost_band: CostBandClass::MeteredMediumBand,
                    notes_label: String::new(),
                },
            ],
            explanation_label: "Metered against the user's own vendor account".to_owned(),
        },
        quota_warning: QuotaWarningSurface {
            family: QuotaFamilyClass::PerUserByokVendorQuota,
            state: QuotaStateClass::Warning,
            scope: QuotaScopeClass::PerSession,
            warning_surfaced: true,
            warning_threshold_label: "Warns at 80% of the per-session vendor quota".to_owned(),
            budget_owner_label: "BYOK credential owner".to_owned(),
            explanation_label: "Approaching the per-session vendor quota".to_owned(),
        },
        ceilings: vec![
            ceiling(
                CeilingKindClass::WallClock,
                CeilingConsumptionClass::ApproachingWarning,
                CeilingEnforcementClass::HardStopOnReach,
                CeilingStopOutcomeClass::NotTriggered,
            ),
            ceiling(
                CeilingKindClass::TokenBudget,
                CeilingConsumptionClass::WarningRaised,
                CeilingEnforcementClass::HardStopOnReach,
                CeilingStopOutcomeClass::NotTriggered,
            ),
            ceiling(
                CeilingKindClass::ToolCallCount,
                CeilingConsumptionClass::CeilingReached,
                CeilingEnforcementClass::SoftPromptOnReach,
                CeilingStopOutcomeClass::PausedAwaitingUser,
            ),
        ],
        downgrade_rules: vec![
            proof_stale_to(M5AiWorkflowQualificationClass::Preview),
            provider_unavailable_to(M5AiWorkflowQualificationClass::Unavailable),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::CheckpointReversible,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:review-byok-agent:m5".to_owned()],
    }
}

fn local_preview_budget() -> AgentBudgetRow {
    AgentBudgetRow {
        budget_id: "explain-local-agent".to_owned(),
        agent_run_id: "run:explain-local".to_owned(),
        agent_label: "Explain local agent".to_owned(),
        resolved_mode: RoutePolicyModeClass::Local,
        claimed_qualification: M5AiWorkflowQualificationClass::Preview,
        run_state: LongRunningAgentRunStateClass::Running,
        spend_ledger: SpendLedger {
            running_cost_band: CostBandClass::BundledNoIncrementalCost,
            measurement: CostMeasurementClass::ActualMeasured,
            charged: ChargedDisclosureClass::NotChargedLocal,
            budget_owner_label: "Local device".to_owned(),
            budget_exhausted: false,
            entries: vec![SpendLedgerEntry {
                sequence: 0,
                phase_label: "Explain".to_owned(),
                delta_cost_band: CostBandClass::BundledNoIncrementalCost,
                running_cost_band: CostBandClass::BundledNoIncrementalCost,
                notes_label: "On-device explain pass".to_owned(),
            }],
            explanation_label: "Runs on-device with no per-run charge".to_owned(),
        },
        quota_warning: QuotaWarningSurface {
            family: QuotaFamilyClass::PerUserLocalUnmetered,
            state: QuotaStateClass::WithinLimit,
            scope: QuotaScopeClass::LocalDevice,
            warning_surfaced: false,
            warning_threshold_label: "Local runs are unmetered; no quota warning".to_owned(),
            budget_owner_label: "Local device".to_owned(),
            explanation_label: "On-device with no enforced provider quota".to_owned(),
        },
        ceilings: vec![
            ceiling(
                CeilingKindClass::WallClock,
                CeilingConsumptionClass::WellWithinLimit,
                CeilingEnforcementClass::HardStopOnReach,
                CeilingStopOutcomeClass::NotTriggered,
            ),
            ceiling(
                CeilingKindClass::TokenBudget,
                CeilingConsumptionClass::WellWithinLimit,
                CeilingEnforcementClass::HardStopOnReach,
                CeilingStopOutcomeClass::NotTriggered,
            ),
            ceiling(
                CeilingKindClass::ToolCallCount,
                CeilingConsumptionClass::ApproachingWarning,
                CeilingEnforcementClass::HardStopOnReach,
                CeilingStopOutcomeClass::NotTriggered,
            ),
        ],
        downgrade_rules: vec![
            proof_stale_to(M5AiWorkflowQualificationClass::Experimental),
            provider_unavailable_to(M5AiWorkflowQualificationClass::Unavailable),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::FullyReversible,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:explain-local-agent:m5".to_owned()],
    }
}

fn exhausted_held_budget() -> AgentBudgetRow {
    AgentBudgetRow {
        budget_id: "background-agent".to_owned(),
        agent_run_id: "run:background".to_owned(),
        agent_label: "Background agent".to_owned(),
        resolved_mode: RoutePolicyModeClass::Byok,
        claimed_qualification: M5AiWorkflowQualificationClass::Held,
        run_state: LongRunningAgentRunStateClass::BudgetExhaustedStopped,
        spend_ledger: SpendLedger {
            running_cost_band: CostBandClass::MeteredHighBand,
            measurement: CostMeasurementClass::ActualMeasured,
            charged: ChargedDisclosureClass::ChargedUserMetered,
            budget_owner_label: "BYOK credential owner".to_owned(),
            budget_exhausted: true,
            entries: vec![
                SpendLedgerEntry {
                    sequence: 0,
                    phase_label: "Investigate".to_owned(),
                    delta_cost_band: CostBandClass::MeteredMediumBand,
                    running_cost_band: CostBandClass::MeteredMediumBand,
                    notes_label: String::new(),
                },
                SpendLedgerEntry {
                    sequence: 1,
                    phase_label: "Implement".to_owned(),
                    delta_cost_band: CostBandClass::MeteredMediumBand,
                    running_cost_band: CostBandClass::MeteredHighBand,
                    notes_label: "Budget spent here".to_owned(),
                },
            ],
            explanation_label: "Per-run cost budget spent; the run was stopped".to_owned(),
        },
        quota_warning: QuotaWarningSurface {
            family: QuotaFamilyClass::PerUserByokVendorQuota,
            state: QuotaStateClass::Exhausted,
            scope: QuotaScopeClass::PerSession,
            warning_surfaced: true,
            warning_threshold_label: "Warned at 80% before exhaustion".to_owned(),
            budget_owner_label: "BYOK credential owner".to_owned(),
            explanation_label: "Per-session vendor quota is exhausted".to_owned(),
        },
        ceilings: vec![
            ceiling(
                CeilingKindClass::WallClock,
                CeilingConsumptionClass::WarningRaised,
                CeilingEnforcementClass::HardStopOnReach,
                CeilingStopOutcomeClass::NotTriggered,
            ),
            ceiling(
                CeilingKindClass::TokenBudget,
                CeilingConsumptionClass::CeilingReached,
                CeilingEnforcementClass::HardStopOnReach,
                CeilingStopOutcomeClass::HardStopped,
            ),
            ceiling(
                CeilingKindClass::ToolCallCount,
                CeilingConsumptionClass::WellWithinLimit,
                CeilingEnforcementClass::HardStopOnReach,
                CeilingStopOutcomeClass::NotTriggered,
            ),
            ceiling(
                CeilingKindClass::CostBudget,
                CeilingConsumptionClass::CeilingReached,
                CeilingEnforcementClass::HardStopOnReach,
                CeilingStopOutcomeClass::HardStopped,
            ),
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
        AGENT_BUDGET_SCHEMA_REF.to_owned(),
        AGENT_BUDGET_DOC_REF.to_owned(),
        PROVIDER_MODEL_REGISTRY_SCHEMA_REF.to_owned(),
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF.to_owned(),
        ROUTING_POLICY_SCHEMA_REF.to_owned(),
    ]
}

fn proof_freshness() -> AgentBudgetProofFreshness {
    AgentBudgetProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-09T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> AgentBudgetPacket {
    AgentBudgetPacket::new(AgentBudgetPacketInput {
        packet_id: PACKET_ID.to_owned(),
        catalogue_label: "Spend Ledgers, Quota Warnings, And Agent Ceilings".to_owned(),
        budgets: vec![
            managed_stable_budget(),
            byok_beta_budget(),
            local_preview_budget(),
            exhausted_held_budget(),
        ],
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-09T00:00:00Z".to_owned(),
    })
}

#[test]
fn budget_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn required_ceiling_partition() {
    assert!(CeilingKindClass::WallClock.is_required());
    assert!(CeilingKindClass::TokenBudget.is_required());
    assert!(CeilingKindClass::ToolCallCount.is_required());
    assert!(!CeilingKindClass::CostBudget.is_required());
    assert!(!CeilingKindClass::AgentStepCount.is_required());
}

#[test]
fn enforcement_bounding_partition() {
    assert!(CeilingEnforcementClass::HardStopOnReach.is_bounding());
    assert!(CeilingEnforcementClass::SoftPromptOnReach.is_bounding());
    assert!(!CeilingEnforcementClass::AdvisoryWarnOnly.is_bounding());
    assert!(!CeilingEnforcementClass::Unenforced.is_bounding());
}

#[test]
fn consumption_warning_partition() {
    assert!(CeilingConsumptionClass::WarningRaised.at_or_past_warning());
    assert!(CeilingConsumptionClass::CeilingReached.at_or_past_warning());
    assert!(CeilingConsumptionClass::CeilingReached.is_reached());
    assert!(!CeilingConsumptionClass::ApproachingWarning.at_or_past_warning());
    assert!(!CeilingConsumptionClass::WellWithinLimit.at_or_past_warning());
}

#[test]
fn counts_match_fixture() {
    let packet = packet();
    assert_eq!(packet.claimed_budget_count(), 3);
    // review (paused) + background (stopped) are stopped/paused.
    assert_eq!(packet.stopped_budget_count(), 2);
    // review (quota warning + ceiling reached) + background (ceiling warnings).
    assert_eq!(packet.warning_budget_count(), 2);
}

#[test]
fn no_budgets_fails() {
    let mut packet = packet();
    packet.budgets.clear();
    assert!(packet.validate().contains(&AgentBudgetViolation::NoBudgets));
}

#[test]
fn duplicate_budget_fails() {
    let mut packet = packet();
    let first = packet.budgets[0].clone();
    packet.budgets.push(first);
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::DuplicateBudget));
}

#[test]
fn budget_row_incomplete_fails() {
    let mut packet = packet();
    packet.budgets[0].spend_ledger.budget_owner_label.clear();
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::BudgetRowIncomplete));
}

#[test]
fn ledger_entries_not_ordered_fails() {
    let mut packet = packet();
    packet.budgets[0].spend_ledger.entries[1].sequence = 5;
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::LedgerEntriesNotOrdered));
}

#[test]
fn ledger_running_not_monotonic_fails() {
    let mut packet = packet();
    // Running band drops back from medium to low between entries.
    packet.budgets[0].spend_ledger.entries[0].running_cost_band = CostBandClass::MeteredHighBand;
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::LedgerRunningNotMonotonic));
}

#[test]
fn ledger_total_mismatch_fails() {
    let mut packet = packet();
    packet.budgets[0].spend_ledger.running_cost_band = CostBandClass::MeteredHighBand;
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::LedgerTotalMismatch));
}

#[test]
fn charged_ledger_undisclosed_fails() {
    let mut packet = packet();
    packet.budgets[1].spend_ledger.charged = ChargedDisclosureClass::ChargeUnknownUnverified;
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::ChargedLedgerUndisclosed));
}

#[test]
fn estimated_ledger_claims_stable_fails() {
    let mut packet = packet();
    packet.budgets[0].spend_ledger.measurement = CostMeasurementClass::EstimateBand;
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::EstimatedLedgerClaimsStable));
}

#[test]
fn missing_required_ceiling_fails() {
    let mut packet = packet();
    packet.budgets[0]
        .ceilings
        .retain(|ceiling| ceiling.kind != CeilingKindClass::TokenBudget);
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::MissingRequiredCeiling));
}

#[test]
fn duplicate_ceiling_fails() {
    let mut packet = packet();
    let dup = packet.budgets[0].ceilings[0].clone();
    packet.budgets[0].ceilings.push(dup);
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::DuplicateCeiling));
}

#[test]
fn claimed_ceiling_not_bounding_fails() {
    let mut packet = packet();
    // A claimed run may not leave a required ceiling unbounded.
    packet.budgets[0].ceilings[0].enforcement = CeilingEnforcementClass::AdvisoryWarnOnly;
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::ClaimedCeilingNotBounding));
}

#[test]
fn ceiling_reached_not_stopped_fails() {
    let mut packet = packet();
    // The reached, hard-stop tool-call ceiling did not stop the run.
    packet.budgets[3].ceilings[1].stop_outcome = CeilingStopOutcomeClass::NotTriggered;
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::CeilingReachedNotStopped));
}

#[test]
fn ceiling_warning_not_surfaced_fails() {
    let mut packet = packet();
    // A ceiling past its warning threshold must carry a warning label.
    packet.budgets[1].ceilings[1]
        .warning_threshold_label
        .clear();
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::CeilingWarningNotSurfaced));
}

#[test]
fn quota_warning_not_surfaced_fails() {
    let mut packet = packet();
    packet.budgets[1].quota_warning.warning_surfaced = false;
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::QuotaWarningNotSurfaced));
}

#[test]
fn exhausted_quota_claims_stable_fails() {
    let mut packet = packet();
    // A Stable run whose provider quota is exhausted must have narrowed.
    packet.budgets[0].quota_warning.state = QuotaStateClass::Exhausted;
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::ExhaustedQuotaClaimsStable));
}

#[test]
fn run_state_stop_mismatch_fails() {
    let mut packet = packet();
    // A hard-stopped ceiling must agree with a terminal run state.
    packet.budgets[3].run_state = LongRunningAgentRunStateClass::Running;
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::RunStateStopMismatch));
}

#[test]
fn run_state_pause_mismatch_fails() {
    let mut packet = packet();
    // A paused soft-prompt ceiling must agree with a paused run state.
    packet.budgets[1].run_state = LongRunningAgentRunStateClass::Running;
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::RunStatePauseMismatch));
}

#[test]
fn claimed_budget_missing_evidence_fails() {
    let mut packet = packet();
    packet.budgets[0].evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::ClaimedBudgetMissingEvidence));
}

#[test]
fn claimed_rollback_unverified_fails() {
    let mut packet = packet();
    packet.budgets[0].rollback_verified = false;
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::ClaimedRollbackUnverified));
}

#[test]
fn downgrade_rules_missing_fails() {
    let mut packet = packet();
    packet.budgets[0].downgrade_rules.clear();
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::DowngradeRulesMissing));
}

#[test]
fn downgrade_rule_missing_proof_stale_fails() {
    let mut packet = packet();
    packet.budgets[0]
        .downgrade_rules
        .retain(|rule| rule.trigger != M5AiWorkflowDowngradeTrigger::ProofStale);
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::DowngradeRuleMissingProofStale));
}

#[test]
fn downgrade_rule_missing_provider_unavailable_fails() {
    let mut packet = packet();
    packet.budgets[0]
        .downgrade_rules
        .retain(|rule| rule.trigger != M5AiWorkflowDowngradeTrigger::ProviderUnavailable);
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::DowngradeRuleMissingProviderUnavailable));
}

#[test]
fn downgrade_rule_not_narrowing_fails() {
    let mut packet = packet();
    packet.budgets[0].downgrade_rules[0].narrowed_to = M5AiWorkflowQualificationClass::Stable;
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::DowngradeRuleNotNarrowing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::MissingSourceContracts));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::ProofFreshnessIncomplete));
}

#[test]
fn raw_boundary_material_in_export_fails() {
    let mut packet = packet();
    packet.budgets[1].spend_ledger.explanation_label =
        "https://api.vendor.example/usage".to_owned();
    assert!(packet
        .validate()
        .contains(&AgentBudgetViolation::RawBoundaryMaterialInExport));
}

#[test]
fn narrowed_qualification_applies_matching_rule() {
    let budget = byok_beta_budget();
    assert_eq!(
        budget.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProofStale),
        M5AiWorkflowQualificationClass::Preview
    );
    assert_eq!(
        budget.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProviderUnavailable),
        M5AiWorkflowQualificationClass::Unavailable
    );
    // A trigger with no rule leaves the claim unchanged.
    assert_eq!(
        budget.narrowed_qualification(M5AiWorkflowDowngradeTrigger::TrustNarrowing),
        M5AiWorkflowQualificationClass::Beta
    );
}

#[test]
fn render_inspector_lists_every_posture() {
    let card = byok_beta_budget().render_inspector();
    assert!(card.contains("review-byok-agent"));
    assert!(card.contains("byok"));
    assert!(card.contains("metered_medium_band"));
    assert!(card.contains("per_user_byok_vendor_quota"));
    assert!(card.contains("tool_call_count"));
    assert!(card.contains("paused_awaiting_user"));
}

#[test]
fn markdown_summary_lists_every_budget() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("Budget inspectors"));
    for budget in &packet().budgets {
        assert!(
            summary.contains(&budget.budget_id),
            "missing {}",
            budget.budget_id
        );
    }
}

#[test]
fn wall_clock_ceiling_stop_fixture_validates() {
    let packet: AgentBudgetPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/add_spend_ledgers_quota_warning_surfaces_and_wall_clock_or_token_or_tool_call_ceilings_for_long_running_agents/wall_clock_ceiling_stop.json"
    )))
    .expect("wall-clock ceiling stop fixture parses");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // The migration agent reached its wall-clock ceiling, hard-stopped, and the
    // run state agrees — yet it keeps its claim because a clean ceiling stop is
    // the feature working.
    let migration = packet.budget("migration-agent").expect("migration present");
    assert!(migration.is_claimed());
    assert!(migration.is_stopped());
    let wall_clock = migration
        .ceiling(CeilingKindClass::WallClock)
        .expect("wall-clock ceiling present");
    assert!(wall_clock.consumption.is_reached());
    assert_eq!(
        wall_clock.stop_outcome,
        CeilingStopOutcomeClass::HardStopped
    );
    // Every run configures the three required ceilings.
    assert!(packet
        .budgets
        .iter()
        .all(|budget| budget.has_required_ceilings()));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_agent_budget_export().expect("checked budget export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
    assert!(!packet.budgets.is_empty());
    // Every run configures the wall-clock, token, and tool-call ceilings.
    assert!(packet
        .budgets
        .iter()
        .all(|budget| budget.has_required_ceilings()));
    // The exhausted run dropped out of every claimed lane and was stopped.
    let exhausted = packet
        .budget("background-agent")
        .expect("background agent present");
    assert!(!exhausted.is_claimed());
    assert!(exhausted.is_stopped());
    assert_eq!(
        exhausted.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProviderUnavailable),
        M5AiWorkflowQualificationClass::Unavailable
    );
}
