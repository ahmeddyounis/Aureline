use super::*;

const PACKET_ID: &str = "ai-run-receipt:stable:0001";

fn proof_stale_to(narrowed_to: M5AiWorkflowQualificationClass) -> AiRunReceiptDowngradeRule {
    AiRunReceiptDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProofStale,
        narrowed_to,
        auto_enforced: true,
        rationale: "Stale proof narrows the public claim".to_owned(),
    }
}

fn provider_unavailable_to(
    narrowed_to: M5AiWorkflowQualificationClass,
) -> AiRunReceiptDowngradeRule {
    AiRunReceiptDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProviderUnavailable,
        narrowed_to,
        auto_enforced: true,
        rationale: "Quota or provider exhaustion narrows the claim".to_owned(),
    }
}

fn required_ceilings() -> Vec<CeilingKindClass> {
    vec![
        CeilingKindClass::WallClock,
        CeilingKindClass::TokenBudget,
        CeilingKindClass::ToolCallCount,
    ]
}

fn not_cancelled() -> CancellationExport {
    CancellationExport {
        class: CancellationClass::NotCancelled,
        reason: CancellationReasonClass::NotCancelled,
        cancelled_at_checkpoint: None,
        export_safe_receipt_available: false,
        receipt_ref: String::new(),
        surface_parity: vec![],
        explanation_label: "Run was not cancelled".to_owned(),
    }
}

fn inline_assist_stable() -> AiRunReceiptRow {
    AiRunReceiptRow {
        receipt_id: "inline-assist-edit".to_owned(),
        agent_run_id: "run:inline-assist-edit".to_owned(),
        run_label: "Inline assist quick edit".to_owned(),
        lane: M5AiWorkflowLane::InlineAssist,
        resolved_mode: RoutePolicyModeClass::Managed,
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        run_state: LongRunningAgentRunStateClass::CompletedWithinBudget,
        preflight: BudgetBandPreflight {
            projected_cost_band: CostBandClass::MeteredLowBand,
            projected_measurement: CostMeasurementClass::EstimateBand,
            charged: ChargedDisclosureClass::ChargedUserSubscription,
            budget_band_label: "Low metered band for one inline edit".to_owned(),
            budget_owner_label: "Managed plan entitlement".to_owned(),
            acknowledgement: PreflightAcknowledgementClass::AutoApprovedWithinBudget,
            projected_ceilings: required_ceilings(),
            explanation_label: "Priced before dispatch against the managed entitlement".to_owned(),
        },
        route_receipt: RouteReceipt {
            resolved_mode: RoutePolicyModeClass::Managed,
            provider_label: "Managed first-party endpoint".to_owned(),
            model_label: "Managed default model".to_owned(),
            selected_hop_reason: FallbackHopReasonClass::PrimaryCheapestQualifying,
            selected_hop_outcome: FallbackHopOutcomeClass::Selected,
            route_change: RouteChangeClass::NoChange,
            downgrade_reason: RouteDowngradeReasonClass::NoDowngrade,
            explanation_label: "Primary managed route served the run".to_owned(),
        },
        spend_receipt: PostRunSpendReceipt {
            final_cost_band: CostBandClass::MeteredLowBand,
            measurement: CostMeasurementClass::ActualMeasured,
            charged: ChargedDisclosureClass::ChargedUserSubscription,
            budget_owner_label: "Managed plan entitlement".to_owned(),
            reconciliation: SpendReconciliationClass::WithinProjection,
            budget_exhausted: false,
            explanation_label: "Measured cost landed inside the projected band".to_owned(),
        },
        cumulative_ceiling: None,
        checkpoints: vec![],
        cancellation: not_cancelled(),
        downgrade_rules: vec![
            proof_stale_to(M5AiWorkflowQualificationClass::Beta),
            provider_unavailable_to(M5AiWorkflowQualificationClass::Unavailable),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::FullyReversible,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:inline-assist-edit:m5".to_owned()],
    }
}

fn patch_review_beta() -> AiRunReceiptRow {
    AiRunReceiptRow {
        receipt_id: "patch-review-byok".to_owned(),
        agent_run_id: "run:patch-review-byok".to_owned(),
        run_label: "Patch review BYOK pass".to_owned(),
        lane: M5AiWorkflowLane::PatchReview,
        resolved_mode: RoutePolicyModeClass::Byok,
        claimed_qualification: M5AiWorkflowQualificationClass::Beta,
        run_state: LongRunningAgentRunStateClass::CompletedWithinBudget,
        preflight: BudgetBandPreflight {
            projected_cost_band: CostBandClass::MeteredMediumBand,
            projected_measurement: CostMeasurementClass::EstimateBand,
            charged: ChargedDisclosureClass::ChargedUserMetered,
            budget_band_label: "Medium metered band for a review pass".to_owned(),
            budget_owner_label: "BYOK credential owner".to_owned(),
            acknowledgement: PreflightAcknowledgementClass::AcknowledgedByUser,
            projected_ceilings: required_ceilings(),
            explanation_label: "Priced before dispatch against the vendor account".to_owned(),
        },
        route_receipt: RouteReceipt {
            resolved_mode: RoutePolicyModeClass::Byok,
            provider_label: "BYOK vendor".to_owned(),
            model_label: "BYOK review model".to_owned(),
            selected_hop_reason: FallbackHopReasonClass::FallbackAfterQuotaExhausted,
            selected_hop_outcome: FallbackHopOutcomeClass::Selected,
            route_change: RouteChangeClass::FallbackDowngrade,
            downgrade_reason: RouteDowngradeReasonClass::QuotaExhausted,
            explanation_label: "Primary route quota exhausted; fell back to the BYOK model"
                .to_owned(),
        },
        spend_receipt: PostRunSpendReceipt {
            final_cost_band: CostBandClass::MeteredMediumBand,
            measurement: CostMeasurementClass::ActualMeasured,
            charged: ChargedDisclosureClass::ChargedUserMetered,
            budget_owner_label: "BYOK credential owner".to_owned(),
            reconciliation: SpendReconciliationClass::WithinProjection,
            budget_exhausted: false,
            explanation_label: "Measured cost landed inside the projected band".to_owned(),
        },
        cumulative_ceiling: None,
        checkpoints: vec![],
        cancellation: not_cancelled(),
        downgrade_rules: vec![
            proof_stale_to(M5AiWorkflowQualificationClass::Preview),
            provider_unavailable_to(M5AiWorkflowQualificationClass::Unavailable),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::CheckpointReversible,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:patch-review-byok:m5".to_owned()],
    }
}

fn branch_agent_stable() -> AiRunReceiptRow {
    AiRunReceiptRow {
        receipt_id: "branch-agent-refactor".to_owned(),
        agent_run_id: "run:branch-agent-refactor".to_owned(),
        run_label: "Side-branch refactor agent".to_owned(),
        lane: M5AiWorkflowLane::BranchOrWorktreeAgent,
        resolved_mode: RoutePolicyModeClass::Managed,
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        run_state: LongRunningAgentRunStateClass::CompletedWithinBudget,
        preflight: BudgetBandPreflight {
            projected_cost_band: CostBandClass::MeteredMediumBand,
            projected_measurement: CostMeasurementClass::EstimateBand,
            charged: ChargedDisclosureClass::ChargedUserSubscription,
            budget_band_label: "Medium metered band for a multi-step branch run".to_owned(),
            budget_owner_label: "Managed plan entitlement".to_owned(),
            acknowledgement: PreflightAcknowledgementClass::AutoApprovedWithinBudget,
            projected_ceilings: vec![
                CeilingKindClass::WallClock,
                CeilingKindClass::TokenBudget,
                CeilingKindClass::ToolCallCount,
                CeilingKindClass::CostBudget,
            ],
            explanation_label: "Priced before dispatch with a cumulative cost budget".to_owned(),
        },
        route_receipt: RouteReceipt {
            resolved_mode: RoutePolicyModeClass::Managed,
            provider_label: "Managed first-party endpoint".to_owned(),
            model_label: "Managed agent model".to_owned(),
            selected_hop_reason: FallbackHopReasonClass::PrimaryCheapestQualifying,
            selected_hop_outcome: FallbackHopOutcomeClass::Selected,
            route_change: RouteChangeClass::NoChange,
            downgrade_reason: RouteDowngradeReasonClass::NoDowngrade,
            explanation_label: "Primary managed route served every step".to_owned(),
        },
        spend_receipt: PostRunSpendReceipt {
            final_cost_band: CostBandClass::MeteredMediumBand,
            measurement: CostMeasurementClass::ActualMeasured,
            charged: ChargedDisclosureClass::ChargedUserSubscription,
            budget_owner_label: "Managed plan entitlement".to_owned(),
            reconciliation: SpendReconciliationClass::WithinProjection,
            budget_exhausted: false,
            explanation_label: "Cumulative measured cost landed inside the projected band"
                .to_owned(),
        },
        cumulative_ceiling: Some(CumulativeCeiling {
            kind: CeilingKindClass::CostBudget,
            cumulative_consumption: CeilingConsumptionClass::ApproachingWarning,
            enforcement: CeilingEnforcementClass::HardStopOnReach,
            cumulative_limit_label: "Cumulative cost budget across the whole branch run".to_owned(),
            explanation_label: "Bounds the run across every checkpoint".to_owned(),
        }),
        checkpoints: vec![
            RunCheckpoint {
                sequence: 0,
                checkpoint_label: "Plan".to_owned(),
                cumulative_cost_band: CostBandClass::MeteredLowBand,
                cumulative_consumption: CeilingConsumptionClass::WellWithinLimit,
                cancellable: true,
                notes_label: "Planning checkpoint".to_owned(),
            },
            RunCheckpoint {
                sequence: 1,
                checkpoint_label: "Implement".to_owned(),
                cumulative_cost_band: CostBandClass::MeteredMediumBand,
                cumulative_consumption: CeilingConsumptionClass::ApproachingWarning,
                cancellable: true,
                notes_label: String::new(),
            },
        ],
        cancellation: not_cancelled(),
        downgrade_rules: vec![
            proof_stale_to(M5AiWorkflowQualificationClass::Beta),
            provider_unavailable_to(M5AiWorkflowQualificationClass::Unavailable),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::FullyReversible,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:branch-agent-refactor:m5".to_owned()],
    }
}

fn cancelled_branch_agent_held() -> AiRunReceiptRow {
    AiRunReceiptRow {
        receipt_id: "branch-agent-cancelled".to_owned(),
        agent_run_id: "run:branch-agent-cancelled".to_owned(),
        run_label: "Cancelled side-branch agent".to_owned(),
        lane: M5AiWorkflowLane::BranchOrWorktreeAgent,
        resolved_mode: RoutePolicyModeClass::Byok,
        claimed_qualification: M5AiWorkflowQualificationClass::Held,
        run_state: LongRunningAgentRunStateClass::CancelledByUser,
        preflight: BudgetBandPreflight {
            projected_cost_band: CostBandClass::MeteredMediumBand,
            projected_measurement: CostMeasurementClass::EstimateBand,
            charged: ChargedDisclosureClass::ChargedUserMetered,
            budget_band_label: "Medium metered band for a branch run".to_owned(),
            budget_owner_label: "BYOK credential owner".to_owned(),
            acknowledgement: PreflightAcknowledgementClass::AcknowledgedByUser,
            projected_ceilings: required_ceilings(),
            explanation_label: "Priced before dispatch against the vendor account".to_owned(),
        },
        route_receipt: RouteReceipt {
            resolved_mode: RoutePolicyModeClass::Byok,
            provider_label: "BYOK vendor".to_owned(),
            model_label: "BYOK agent model".to_owned(),
            selected_hop_reason: FallbackHopReasonClass::PrimaryCheapestQualifying,
            selected_hop_outcome: FallbackHopOutcomeClass::Selected,
            route_change: RouteChangeClass::NoChange,
            downgrade_reason: RouteDowngradeReasonClass::NoDowngrade,
            explanation_label: "Primary BYOK route served the run until cancellation".to_owned(),
        },
        spend_receipt: PostRunSpendReceipt {
            final_cost_band: CostBandClass::MeteredLowBand,
            measurement: CostMeasurementClass::ActualMeasured,
            charged: ChargedDisclosureClass::ChargedUserMetered,
            budget_owner_label: "BYOK credential owner".to_owned(),
            reconciliation: SpendReconciliationClass::UnderProjection,
            budget_exhausted: false,
            explanation_label: "Cancelled early; measured cost landed below the projected band"
                .to_owned(),
        },
        cumulative_ceiling: Some(CumulativeCeiling {
            kind: CeilingKindClass::CostBudget,
            cumulative_consumption: CeilingConsumptionClass::WellWithinLimit,
            enforcement: CeilingEnforcementClass::HardStopOnReach,
            cumulative_limit_label: "Cumulative cost budget across the branch run".to_owned(),
            explanation_label: "Bounds the run across every checkpoint".to_owned(),
        }),
        checkpoints: vec![
            RunCheckpoint {
                sequence: 0,
                checkpoint_label: "Plan".to_owned(),
                cumulative_cost_band: CostBandClass::MeteredLowBand,
                cumulative_consumption: CeilingConsumptionClass::WellWithinLimit,
                cancellable: true,
                notes_label: String::new(),
            },
            RunCheckpoint {
                sequence: 1,
                checkpoint_label: "Implement".to_owned(),
                cumulative_cost_band: CostBandClass::MeteredLowBand,
                cumulative_consumption: CeilingConsumptionClass::WellWithinLimit,
                cancellable: true,
                notes_label: "Cancelled here".to_owned(),
            },
        ],
        cancellation: CancellationExport {
            class: CancellationClass::CancelledByUser,
            reason: CancellationReasonClass::UserRequested,
            cancelled_at_checkpoint: Some(1),
            export_safe_receipt_available: true,
            receipt_ref: "receipt:branch-agent-cancelled:cancellation".to_owned(),
            surface_parity: vec![
                M5AiWorkflowConsumerSurface::DesktopReviewWorkspace,
                M5AiWorkflowConsumerSurface::CliHeadless,
                M5AiWorkflowConsumerSurface::SupportExport,
            ],
            explanation_label:
                "User cancelled at the implement checkpoint with an export-safe receipt".to_owned(),
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
        AI_RUN_RECEIPT_SCHEMA_REF.to_owned(),
        AI_RUN_RECEIPT_DOC_REF.to_owned(),
        PROVIDER_MODEL_REGISTRY_SCHEMA_REF.to_owned(),
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF.to_owned(),
        ROUTING_POLICY_SCHEMA_REF.to_owned(),
        AGENT_BUDGET_SCHEMA_REF.to_owned(),
    ]
}

fn proof_freshness() -> AiRunReceiptProofFreshness {
    AiRunReceiptProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-12T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> AiRunReceiptPacket {
    AiRunReceiptPacket::new(AiRunReceiptPacketInput {
        packet_id: PACKET_ID.to_owned(),
        catalogue_label: "AI Spend And Route Receipts".to_owned(),
        receipts: vec![
            inline_assist_stable(),
            patch_review_beta(),
            branch_agent_stable(),
            cancelled_branch_agent_held(),
        ],
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-12T00:00:00Z".to_owned(),
    })
}

#[test]
fn receipt_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn acknowledgement_partition() {
    assert!(PreflightAcknowledgementClass::AcknowledgedByUser.is_acknowledged());
    assert!(PreflightAcknowledgementClass::AutoApprovedWithinBudget.is_acknowledged());
    assert!(PreflightAcknowledgementClass::NotRequiredLocalUnmetered.is_acknowledged());
    assert!(!PreflightAcknowledgementClass::PendingAcknowledgement.is_acknowledged());
}

#[test]
fn route_change_and_reason_partition() {
    assert!(RouteChangeClass::FallbackDowngrade.is_downgrade());
    assert!(RouteChangeClass::QuotaRedirect.is_downgrade());
    assert!(!RouteChangeClass::NoChange.is_downgrade());
    assert!(RouteDowngradeReasonClass::GenericProviderError.is_generic());
    assert!(!RouteDowngradeReasonClass::QuotaExhausted.is_generic());
    assert!(CancellationReasonClass::GenericProviderError.is_generic());
    assert!(!CancellationReasonClass::UserRequested.is_generic());
}

#[test]
fn counts_match_fixture() {
    let packet = packet();
    assert_eq!(packet.claimed_receipt_count(), 3);
    assert_eq!(packet.branch_agent_receipt_count(), 2);
    assert_eq!(packet.cancelled_receipt_count(), 1);
}

#[test]
fn no_receipts_fails() {
    let mut packet = packet();
    packet.receipts.clear();
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::NoReceipts));
}

#[test]
fn duplicate_receipt_fails() {
    let mut packet = packet();
    let first = packet.receipts[0].clone();
    packet.receipts.push(first);
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::DuplicateReceipt));
}

#[test]
fn lane_coverage_incomplete_fails() {
    let mut packet = packet();
    // Drop every patch-review row, leaving the lane uncovered.
    packet
        .receipts
        .retain(|r| r.lane != M5AiWorkflowLane::PatchReview);
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::LaneCoverageIncomplete));
}

#[test]
fn receipt_row_incomplete_fails() {
    let mut packet = packet();
    packet.receipts[0].preflight.budget_band_label.clear();
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::ReceiptRowIncomplete));
}

#[test]
fn preflight_unacknowledged_fails() {
    let mut packet = packet();
    packet.receipts[0].preflight.acknowledgement =
        PreflightAcknowledgementClass::PendingAcknowledgement;
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::PreflightUnacknowledged));
}

#[test]
fn preflight_missing_required_ceiling_fails() {
    let mut packet = packet();
    packet.receipts[0]
        .preflight
        .projected_ceilings
        .retain(|kind| *kind != CeilingKindClass::TokenBudget);
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::PreflightMissingRequiredCeiling));
}

#[test]
fn route_receipt_mode_mismatch_fails() {
    let mut packet = packet();
    packet.receipts[0].route_receipt.resolved_mode = RoutePolicyModeClass::Byok;
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::RouteReceiptModeMismatch));
}

#[test]
fn route_downgrade_reason_missing_fails() {
    let mut packet = packet();
    // A fallback downgrade with no reason hides the cause.
    packet.receipts[1].route_receipt.downgrade_reason = RouteDowngradeReasonClass::NoDowngrade;
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::RouteDowngradeReasonMissing));
}

#[test]
fn route_downgrade_reason_too_generic_fails() {
    let mut packet = packet();
    // A fallback downgrade may not collapse into a generic provider error.
    packet.receipts[1].route_receipt.downgrade_reason =
        RouteDowngradeReasonClass::GenericProviderError;
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::RouteDowngradeReasonTooGeneric));
}

#[test]
fn spend_receipt_not_reconciled_fails() {
    let mut packet = packet();
    packet.receipts[0].spend_receipt.reconciliation = SpendReconciliationClass::NotReconciled;
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::SpendReceiptNotReconciled));
}

#[test]
fn charged_receipt_undisclosed_fails() {
    let mut packet = packet();
    packet.receipts[0].spend_receipt.charged = ChargedDisclosureClass::ChargeUnknownUnverified;
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::ChargedReceiptUndisclosed));
}

#[test]
fn estimated_receipt_claims_stable_fails() {
    let mut packet = packet();
    packet.receipts[0].spend_receipt.measurement = CostMeasurementClass::EstimateBand;
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::EstimatedReceiptClaimsStable));
}

#[test]
fn spend_overrun_not_disclosed_fails() {
    let mut packet = packet();
    // Final cost rose above the projected band without an overrun disclosure.
    packet.receipts[0].spend_receipt.final_cost_band = CostBandClass::MeteredHighBand;
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::SpendOverrunNotDisclosed));
}

#[test]
fn spend_overrun_disclosed_passes() {
    let mut packet = packet();
    packet.receipts[0].spend_receipt.final_cost_band = CostBandClass::MeteredHighBand;
    packet.receipts[0].spend_receipt.reconciliation =
        SpendReconciliationClass::OverProjectionDisclosed;
    assert!(!packet
        .validate()
        .contains(&AiRunReceiptViolation::SpendOverrunNotDisclosed));
}

#[test]
fn branch_agent_missing_cumulative_ceiling_fails() {
    let mut packet = packet();
    packet.receipts[2].cumulative_ceiling = None;
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::BranchAgentMissingCumulativeCeiling));
}

#[test]
fn claimed_cumulative_ceiling_not_bounding_fails() {
    let mut packet = packet();
    if let Some(ceiling) = packet.receipts[2].cumulative_ceiling.as_mut() {
        ceiling.enforcement = CeilingEnforcementClass::AdvisoryWarnOnly;
    }
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::ClaimedCumulativeCeilingNotBounding));
}

#[test]
fn branch_agent_checkpoints_missing_fails() {
    let mut packet = packet();
    packet.receipts[2].checkpoints.clear();
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::BranchAgentCheckpointsMissing));
}

#[test]
fn checkpoints_not_ordered_fails() {
    let mut packet = packet();
    packet.receipts[2].checkpoints[1].sequence = 5;
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::CheckpointsNotOrdered));
}

#[test]
fn checkpoint_cumulative_not_monotonic_fails() {
    let mut packet = packet();
    // The cumulative band drops back from medium to low across checkpoints.
    packet.receipts[2].checkpoints[0].cumulative_cost_band = CostBandClass::MeteredHighBand;
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::CheckpointCumulativeNotMonotonic));
}

#[test]
fn cancellation_state_mismatch_fails() {
    let mut packet = packet();
    // A cancelled-by-user run state must carry a matching cancellation class.
    packet.receipts[3].cancellation.class = CancellationClass::CancelledByPolicy;
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::CancellationStateMismatch));
}

#[test]
fn cancellation_not_export_safe_fails() {
    let mut packet = packet();
    packet.receipts[3]
        .cancellation
        .export_safe_receipt_available = false;
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::CancellationNotExportSafe));
}

#[test]
fn cancellation_checkpoint_missing_fails() {
    let mut packet = packet();
    packet.receipts[3].cancellation.cancelled_at_checkpoint = None;
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::CancellationCheckpointMissing));
}

#[test]
fn cancellation_checkpoint_unknown_fails() {
    let mut packet = packet();
    packet.receipts[3].cancellation.cancelled_at_checkpoint = Some(9);
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::CancellationCheckpointMissing));
}

#[test]
fn cancellation_reason_too_generic_fails() {
    let mut packet = packet();
    packet.receipts[3].cancellation.reason = CancellationReasonClass::GenericProviderError;
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::CancellationReasonTooGeneric));
}

#[test]
fn cancellation_surface_parity_incomplete_fails() {
    let mut packet = packet();
    packet.receipts[3]
        .cancellation
        .surface_parity
        .retain(|surface| *surface != M5AiWorkflowConsumerSurface::SupportExport);
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::CancellationSurfaceParityIncomplete));
}

#[test]
fn claimed_receipt_missing_evidence_fails() {
    let mut packet = packet();
    packet.receipts[0].evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::ClaimedReceiptMissingEvidence));
}

#[test]
fn claimed_rollback_unverified_fails() {
    let mut packet = packet();
    packet.receipts[0].rollback_verified = false;
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::ClaimedRollbackUnverified));
}

#[test]
fn downgrade_rules_missing_fails() {
    let mut packet = packet();
    packet.receipts[0].downgrade_rules.clear();
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::DowngradeRulesMissing));
}

#[test]
fn downgrade_rule_missing_proof_stale_fails() {
    let mut packet = packet();
    packet.receipts[0]
        .downgrade_rules
        .retain(|rule| rule.trigger != M5AiWorkflowDowngradeTrigger::ProofStale);
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::DowngradeRuleMissingProofStale));
}

#[test]
fn downgrade_rule_missing_provider_unavailable_fails() {
    let mut packet = packet();
    packet.receipts[0]
        .downgrade_rules
        .retain(|rule| rule.trigger != M5AiWorkflowDowngradeTrigger::ProviderUnavailable);
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::DowngradeRuleMissingProviderUnavailable));
}

#[test]
fn downgrade_rule_not_narrowing_fails() {
    let mut packet = packet();
    packet.receipts[0].downgrade_rules[0].narrowed_to = M5AiWorkflowQualificationClass::Stable;
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::DowngradeRuleNotNarrowing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::MissingSourceContracts));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::ProofFreshnessIncomplete));
}

#[test]
fn raw_boundary_material_in_export_fails() {
    let mut packet = packet();
    packet.receipts[1].route_receipt.explanation_label =
        "https://api.vendor.example/usage".to_owned();
    assert!(packet
        .validate()
        .contains(&AiRunReceiptViolation::RawBoundaryMaterialInExport));
}

#[test]
fn narrowed_qualification_applies_matching_rule() {
    let receipt = patch_review_beta();
    assert_eq!(
        receipt.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProofStale),
        M5AiWorkflowQualificationClass::Preview
    );
    assert_eq!(
        receipt.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProviderUnavailable),
        M5AiWorkflowQualificationClass::Unavailable
    );
    // A trigger with no rule leaves the claim unchanged.
    assert_eq!(
        receipt.narrowed_qualification(M5AiWorkflowDowngradeTrigger::TrustNarrowing),
        M5AiWorkflowQualificationClass::Beta
    );
}

#[test]
fn render_inspector_lists_every_posture() {
    let card = cancelled_branch_agent_held().render_inspector();
    assert!(card.contains("branch-agent-cancelled"));
    assert!(card.contains("branch_or_worktree_agent"));
    assert!(card.contains("cancelled_by_user"));
    assert!(card.contains("user_requested"));
    assert!(card.contains("Implement"));
}

#[test]
fn markdown_summary_lists_every_receipt() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("Receipt inspectors"));
    for receipt in &packet().receipts {
        assert!(
            summary.contains(&receipt.receipt_id),
            "missing {}",
            receipt.receipt_id
        );
    }
}

#[test]
fn cancelled_branch_agent_fixture_validates() {
    let packet: AiRunReceiptPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/implement_spend_and_route_receipts_budget_band_preflights_cumulative_branch_agent_ceilings_and_checkpoint_aware_cancella/cancelled_branch_agent.json"
    )))
    .expect("cancelled branch agent fixture parses");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // The cancelled branch agent names the checkpoint it stopped at, carries an
    // export-safe receipt readable on the support-export surface, and gives a
    // precise reason rather than a generic provider error.
    let cancelled = packet
        .receipt("background-branch-agent")
        .expect("background branch agent present");
    assert!(cancelled.is_branch_agent());
    assert!(cancelled.is_cancelled());
    assert_eq!(cancelled.cancellation.cancelled_at_checkpoint, Some(2));
    assert!(cancelled.cancellation.export_safe_receipt_available);
    assert!(cancelled
        .cancellation
        .surface_parity
        .contains(&M5AiWorkflowConsumerSurface::SupportExport));
    assert!(!cancelled.cancellation.reason.is_generic());
}

#[test]
fn checked_support_export_validates() {
    let packet = current_ai_run_receipt_export().expect("checked receipt export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
    assert!(!packet.receipts.is_empty());
    // Every workflow lane is covered by at least one receipt.
    for lane in M5AiWorkflowLane::ALL {
        assert!(
            packet.receipts.iter().any(|r| r.lane == lane),
            "lane {} uncovered",
            lane.as_str()
        );
    }
}
