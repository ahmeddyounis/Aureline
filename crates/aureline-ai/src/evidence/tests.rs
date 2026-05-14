use std::path::Path;

use crate::routing::{
    AiRouteCandidate, AiRouteProviderClass, AiRoutingPacket, CostEnvelopeClass,
    CostVisibilityClass, DeploymentProfileClass, ExecutionLocusClass, ExhaustionStateClass,
    LatencyCostEnvelope, LatencyEnvelopeClass, PolicyTrustState, QuotaFamilyClass, QuotaInspector,
    QuotaScopeClass, QuotaStateClass, RegionPostureClass, RetentionStanceClass, RouteOriginClass,
    RouteSelectionOverrideReasonClass, RouteSelectionReasonClass, RoutingPolicyContext,
    RoutingRunStateClass, SelectedOutcomeClass, TokenCeilingClass, ToolCallCeilingClass,
    WallTimeCeilingClass,
};

use super::*;

fn policy_context() -> RoutingPolicyContext {
    RoutingPolicyContext {
        policy_epoch_ref: "policy_epoch:alpha:2026-05-14".to_owned(),
        trust_state: PolicyTrustState::Trusted,
        deployment_profile_class: DeploymentProfileClass::ManagedCloud,
        execution_context_ref: Some("execution_context:ai-mutation-alpha:0001".to_owned()),
    }
}

fn quota() -> QuotaInspector {
    QuotaInspector {
        quota_family_class: QuotaFamilyClass::VendorHostedEntitlementQuota,
        quota_state_class: QuotaStateClass::WithinLimit,
        quota_scope_class: QuotaScopeClass::VendorHostedEntitlement,
        budget_owner_ref: "quota_owner:workspace:ai-mutation-alpha".to_owned(),
        quota_meter_ref: Some("quota_meter:workspace:ai-mutation-alpha".to_owned()),
        quota_forecast_ref: Some("quota_forecast:workspace:ai-mutation-alpha".to_owned()),
        usage_export_ref: Some("usage_export:ai-mutation-alpha".to_owned()),
        explanation_label: "Workspace hosted AI entitlement is available for this mutation review."
            .to_owned(),
        local_continuity_label:
            "Manual edit, Git review, search, and tests remain available without hosted AI."
                .to_owned(),
        recovery_action_ref: Some("action:ai-mutation-alpha:view-quota".to_owned()),
    }
}

fn envelope() -> LatencyCostEnvelope {
    LatencyCostEnvelope {
        latency_envelope_class: LatencyEnvelopeClass::StreamingFirstTokenUnder500Ms,
        cost_envelope_class: CostEnvelopeClass::VendorHostedEntitlementBand,
        cost_visibility_class: CostVisibilityClass::BundledNoIncrementalCost,
        token_ceiling_class: TokenCeilingClass::TokensUnder32K,
        tool_call_ceiling_class: ToolCallCeilingClass::BoundedToolCallsUnder4,
        wall_time_ceiling_class: WallTimeCeilingClass::WallTimeUnder30S,
        budget_routing_policy_ref: "budget_policy:ai-mutation-alpha".to_owned(),
        graduation_packet_ref: "graduation_packet:ai-mutation-alpha".to_owned(),
        envelope_evidence_ref: "envelope_evidence:ai-mutation-alpha".to_owned(),
        explanation_label: "Route uses the bundled preview band for review-first mutation."
            .to_owned(),
    }
}

fn hosted_candidate() -> AiRouteCandidate {
    AiRouteCandidate {
        candidate_id: "candidate:hosted-managed:mutation-alpha".to_owned(),
        provider_entry_ref: "provider-entry:first_party_managed:mutation-alpha".to_owned(),
        provider_label: "Aureline managed hosted AI".to_owned(),
        provider_class: AiRouteProviderClass::FirstPartyManaged,
        model_entry_ref: "model-entry:hosted-general:mutation-alpha".to_owned(),
        model_label: "Hosted general patch preview".to_owned(),
        execution_locus_class: ExecutionLocusClass::VendorHostedFirstPartyManaged,
        route_origin_class: RouteOriginClass::VendorHostedManaged,
        region_posture_class: RegionPostureClass::SingleRegionPinned,
        retention_stance_class: RetentionStanceClass::NoRetentionPromisedBodyDiscarded,
        quota: quota(),
        envelope: envelope(),
        route_selection_reason_class: RouteSelectionReasonClass::NoCheaperQualifyingRouteExisted,
        route_selection_override_reason_class:
            RouteSelectionOverrideReasonClass::NoOverrideCheapestWasUsed,
        exhaustion_state_class: ExhaustionStateClass::NotExhaustedRouteAdmitted,
        selected_outcome_class: SelectedOutcomeClass::SelectedThisPath,
        route_selection_disclosure_ref: None,
        originating_approval_ticket_ref: Some(
            "approval-ticket:route:mutation-alpha:0001".to_owned(),
        ),
        explanation_label: "Hosted route selected for the bounded mutation review.".to_owned(),
    }
}

fn routing_packet() -> AiRoutingPacket {
    let candidate = hosted_candidate();
    AiRoutingPacket::new(
        "ai_routing_packet:mutation-alpha:0001",
        "workflow.alpha.ai.mutation_review",
        "request_workspace:alpha:mutation",
        RoutingRunStateClass::PreviewPreDispatch,
        policy_context(),
        "capability_lifecycle:alpha.ai.routing_cost",
        Some("identity_mode_baseline:alpha:local-vs-managed".to_owned()),
        vec![candidate.clone()],
        candidate.candidate_id,
        Vec::new(),
        vec![
            "docs/ai/provider_model_registry_contract.md".to_owned(),
            "docs/ai/spend_and_route_receipt_contract.md".to_owned(),
            "docs/ai/context_assembly_contract.md".to_owned(),
        ],
        "2026-05-14T10:00:00Z",
    )
}

fn docs_sources() -> Vec<CitedSourceReference> {
    vec![
        CitedSourceReference {
            source_reference_id: "source-ref:workspace-symbol:payments-handler".to_owned(),
            source_class: CitedSourceClass::WorkspaceSymbol,
            source_identity_ref: "symbol:PaymentsHandler.createCharge".to_owned(),
            source_revision_ref: Some("git-tree:alpha:payments-current".to_owned()),
            docs_pack_ref: None,
            docs_pack_revision_ref: None,
            exact_anchor_ref: None,
            citation_visibility_class: CitationVisibilityClass::NotCitationBearing,
            hidden_or_omitted_citation_note: None,
            source_posture: EvidenceSourcePosture::TrustedFirstParty,
            freshness_class: EvidenceFreshnessClass::AuthoritativeLive,
        },
        CitedSourceReference {
            source_reference_id: "source-ref:docs-pack:payments-guide".to_owned(),
            source_class: CitedSourceClass::DocsPackExcerpt,
            source_identity_ref: "docs-node:payments-guide:create-charge".to_owned(),
            source_revision_ref: Some("doc-revision:payments-guide:2026-05-01".to_owned()),
            docs_pack_ref: Some("docs-pack:payments-guide".to_owned()),
            docs_pack_revision_ref: Some("docs-pack-revision:payments-guide:2026-05-01".to_owned()),
            exact_anchor_ref: Some("citation-anchor:payments-guide#create-charge".to_owned()),
            citation_visibility_class: CitationVisibilityClass::AnchorAvailable,
            hidden_or_omitted_citation_note: None,
            source_posture: EvidenceSourcePosture::TrustedAuthority,
            freshness_class: EvidenceFreshnessClass::AuthoritativeLive,
        },
        CitedSourceReference {
            source_reference_id: "source-ref:glossary:charge-state".to_owned(),
            source_class: CitedSourceClass::GlossaryEntry,
            source_identity_ref: "glossary-entry:payments:charge-state".to_owned(),
            source_revision_ref: Some("glossary-revision:payments:2026-05-01".to_owned()),
            docs_pack_ref: Some("glossary-pack:payments".to_owned()),
            docs_pack_revision_ref: Some("glossary-pack-revision:payments:2026-05-01".to_owned()),
            exact_anchor_ref: None,
            citation_visibility_class: CitationVisibilityClass::AnchorUnavailableDisclosed,
            hidden_or_omitted_citation_note: Some(
                "Glossary entry came from a mirrored pack that does not expose section anchors."
                    .to_owned(),
            ),
            source_posture: EvidenceSourcePosture::ReviewedDerived,
            freshness_class: EvidenceFreshnessClass::WarmCached,
        },
    ]
}

fn derived_explanations() -> Vec<DerivedExplanationLineage> {
    vec![DerivedExplanationLineage {
        explanation_ref: "derived-explanation:payments-charge-path".to_owned(),
        basis_source_reference_refs: vec![
            "source-ref:workspace-symbol:payments-handler".to_owned(),
            "source-ref:docs-pack:payments-guide".to_owned(),
            "source-ref:glossary:charge-state".to_owned(),
        ],
        inference_class: InferenceClass::DerivedMultipleSources,
        confidence_class: ConfidenceClass::Inferred,
        confidence_reason_label:
            "Inference bridges one workspace symbol, one docs-pack anchor, and one glossary entry."
                .to_owned(),
    }]
}

fn approval(decision_class: ApprovalDecisionClass) -> Vec<ApprovalLineageEntry> {
    vec![ApprovalLineageEntry {
        approval_lineage_id: "approval-lineage:mutation-alpha:0001".to_owned(),
        approval_ticket_ref: "approval-ticket:ai-apply:mutation-alpha:0001".to_owned(),
        decision_class,
        actor_class: ApprovalActorClass::LocalUser,
        preview_ref: "review-surface:ai-mutation:alpha:0001".to_owned(),
        policy_epoch_ref: "policy_epoch:alpha:2026-05-14".to_owned(),
        decided_at: "2026-05-14T10:03:00Z".to_owned(),
        summary_label: "User reviewed the AI patch proposal in the foreground review sheet."
            .to_owned(),
    }]
}

fn review_lineage(apply_outcome_class: ApplyOutcomeClass) -> MutationReviewLineage {
    let applied = apply_outcome_class == ApplyOutcomeClass::AppliedSuccess;
    MutationReviewLineage {
        review_surface_ref: "review-surface:ai-mutation:alpha:0001".to_owned(),
        patch_review_summary_ref: "patch-review:summary:mutation-alpha:0001".to_owned(),
        produced_artifact_refs: vec!["patch-artifact:mutation-alpha:0001".to_owned()],
        changed_file_count: 2,
        generated_artifact_count: 0,
        validation_summary_refs: vec!["patch-validation:mutation-alpha:0001".to_owned()],
        validation_outcome_class: ValidationOutcomeClass::Passed,
        rollback_checkpoint_ref: applied.then(|| "checkpoint:ai-mutation:alpha:0001".to_owned()),
        mutation_journal_ref: applied.then(|| "mutation-journal:ai-mutation:alpha:0001".to_owned()),
        apply_outcome_class,
    }
}

fn packet(
    packet_state: MutationEvidenceState,
    decision_class: ApprovalDecisionClass,
    apply_outcome_class: ApplyOutcomeClass,
) -> AiMutationEvidencePacket {
    let routing = routing_packet();
    assert!(routing.validate().is_empty());
    AiMutationEvidencePacket::new(AiMutationEvidencePacketInput {
        evidence_packet_id: "evidence-packet:ai-mutation:alpha:0001".to_owned(),
        mutation_wedge_ref: "ai-mutation-wedge:patch-review-alpha".to_owned(),
        composer_session_ref: "composer-session:ai-mutation:alpha:0001".to_owned(),
        turn_draft_ref: "turn-draft:ai-mutation:alpha:0001".to_owned(),
        request_workspace_ref: "request-workspace:ai-mutation:alpha:0001".to_owned(),
        assembly_ref: "assembly:ai-mutation:alpha:0001".to_owned(),
        packet_state,
        intent_class: MutationIntentClass::LocalReversibleEdit,
        route_spend_lineage: RouteSpendLineage::from_routing_packet(
            &routing,
            "route-receipt:ai-mutation:alpha:0001",
            "spend-receipt:ai-mutation:alpha:0001",
        ),
        approval_lineage: approval(decision_class),
        cited_sources: docs_sources(),
        derived_explanations: derived_explanations(),
        tainted_context_fences: Vec::new(),
        review_lineage: review_lineage(apply_outcome_class),
        source_contract_refs: vec![
            "docs/ai/context_assembly_contract.md".to_owned(),
            "docs/ai/evidence_replayability_contract.md".to_owned(),
            "docs/ai/spend_and_route_receipt_contract.md".to_owned(),
            "docs/ai/prompt_injection_and_taint_contract.md".to_owned(),
        ],
        policy_context: policy_context(),
        running_build_identity_ref: "build-identity:aureline:alpha:2026-05-14".to_owned(),
        redaction_class: EvidenceRedactionClass::MetadataSafeDefault,
        minted_at: "2026-05-14T10:02:00Z".to_owned(),
        completed_at: if packet_state == MutationEvidenceState::ReviewPreApply {
            None
        } else {
            Some("2026-05-14T10:04:00Z".to_owned())
        },
    })
}

fn tainted_rejected_packet() -> AiMutationEvidencePacket {
    let mut packet = packet(
        MutationEvidenceState::Rejected,
        ApprovalDecisionClass::BlockedByPolicy,
        ApplyOutcomeClass::BlockedNoApply,
    );
    packet.cited_sources.push(CitedSourceReference {
        source_reference_id: "source-ref:terminal:tainted-output".to_owned(),
        source_class: CitedSourceClass::SearchResult,
        source_identity_ref: "terminal-capture:build-output:0001".to_owned(),
        source_revision_ref: Some("terminal-capture-revision:0001".to_owned()),
        docs_pack_ref: None,
        docs_pack_revision_ref: None,
        exact_anchor_ref: None,
        citation_visibility_class: CitationVisibilityClass::NotCitationBearing,
        hidden_or_omitted_citation_note: None,
        source_posture: EvidenceSourcePosture::TaintedExternal,
        freshness_class: EvidenceFreshnessClass::WarmCached,
    });
    packet.tainted_context_fences.push(TaintedContextFence {
        fence_id: "tainted-fence:terminal-output:0001".to_owned(),
        segment_id_ref: "segment:terminal-output:0001".to_owned(),
        source_reference_ref: Some("source-ref:terminal:tainted-output".to_owned()),
        tainted_source_class: TaintedEvidenceSourceClass::TerminalSnippet,
        source_posture: EvidenceSourcePosture::TaintedExternal,
        fence_strategy: TaintFenceStrategy::QuotedAsDataOnly,
        usage_constraints: vec![
            TaintUsageConstraint::MustNotGainToolPermission,
            TaintUsageConstraint::MustNotEscalateScope,
            TaintUsageConstraint::MustNotCommitToRepo,
            TaintUsageConstraint::MustPreserveFenceInDownstreamPacket,
        ],
        reason_class: TaintFenceReasonClass::PolicyDisallowedContext,
        user_visible_explanation_label:
            "Terminal output contained imperative text, so it stayed fenced as data and could not approve the mutation."
                .to_owned(),
    });
    packet
}

#[test]
fn docs_backed_pre_apply_packet_exports_route_spend_approval_and_citations() {
    let packet = packet(
        MutationEvidenceState::ReviewPreApply,
        ApprovalDecisionClass::PendingUserReview,
        ApplyOutcomeClass::NotAppliedPendingReview,
    );

    assert!(packet.validate().is_empty());
    let support = packet.support_packet();
    assert_eq!(
        support.route_receipt_ref,
        "route-receipt:ai-mutation:alpha:0001"
    );
    assert_eq!(
        support.spend_receipt_ref,
        "spend-receipt:ai-mutation:alpha:0001"
    );
    assert_eq!(
        support.approval_ticket_refs,
        vec!["approval-ticket:ai-apply:mutation-alpha:0001"]
    );
    assert!(support
        .citation_rows
        .iter()
        .any(
            |row| row.docs_pack_ref.as_deref() == Some("docs-pack:payments-guide")
                && row.exact_anchor_ref.as_deref()
                    == Some("citation-anchor:payments-guide#create-charge")
        ));
    assert!(support
        .citation_rows
        .iter()
        .any(|row| row.hidden_or_omitted_citation_note.is_some()));
    assert_eq!(
        support.derived_explanation_rows[0].inference_token,
        "derived_multiple_sources"
    );
    assert_eq!(
        support.derived_explanation_rows[0].confidence_token,
        "inferred"
    );
    assert!(!packet.export_safe_support_json().contains("://"));
    assert!(!packet
        .export_safe_support_json()
        .contains("rejected before charge creation"));
    assert!(packet
        .review_rows()
        .iter()
        .any(|row| row.row_id == "approval" && row.value_token == "pending_user_review"));
}

#[test]
fn applied_packet_requires_approval_checkpoint_and_mutation_journal() {
    let packet = packet(
        MutationEvidenceState::Applied,
        ApprovalDecisionClass::Approved,
        ApplyOutcomeClass::AppliedSuccess,
    );
    assert!(packet.validate().is_empty());
    let support = packet.support_packet();
    assert_eq!(
        support.rollback_checkpoint_ref.as_deref(),
        Some("checkpoint:ai-mutation:alpha:0001")
    );
    assert_eq!(support.apply_outcome_token, "applied_success");

    let mut missing_checkpoint = packet.clone();
    missing_checkpoint.review_lineage.rollback_checkpoint_ref = None;
    assert!(missing_checkpoint
        .validate()
        .contains(&AiMutationEvidenceViolation::AppliedPacketMissingApplyLineage));

    let mut missing_approval = packet;
    missing_approval.approval_lineage[0].decision_class = ApprovalDecisionClass::PendingUserReview;
    assert!(missing_approval
        .validate()
        .contains(&AiMutationEvidenceViolation::AppliedPacketMissingApproval));
}

#[test]
fn tainted_or_policy_disallowed_context_is_fenced_and_rejection_keeps_lineage() {
    let packet = tainted_rejected_packet();
    assert!(packet.validate().is_empty());
    let support = packet.support_packet();
    assert_eq!(support.apply_outcome_token, "blocked_no_apply");
    assert_eq!(
        support.approval_decision_tokens,
        vec!["blocked_by_policy".to_owned()]
    );
    assert_eq!(
        support.tainted_fence_rows[0].reason_token,
        "policy_disallowed_context"
    );
    assert_eq!(
        support.route_receipt_ref,
        "route-receipt:ai-mutation:alpha:0001"
    );
    assert_eq!(
        support.spend_receipt_ref,
        "spend-receipt:ai-mutation:alpha:0001"
    );

    let mut missing_fence = packet;
    missing_fence.tainted_context_fences.clear();
    assert!(missing_fence
        .validate()
        .contains(&AiMutationEvidenceViolation::TaintedSourceMissingFence));
}

#[test]
fn fixture_packets_round_trip_and_validate() {
    let fixture_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/ai/mutation_wedge_alpha");
    for fixture_name in [
        "docs_backed_review_pre_apply.json",
        "applied_after_approval.json",
        "tainted_context_rejected.json",
    ] {
        let payload =
            std::fs::read_to_string(fixture_dir.join(fixture_name)).expect("fixture reads");
        let packet: AiMutationEvidencePacket =
            serde_json::from_str(&payload).expect("fixture parses");
        assert!(
            packet.validate().is_empty(),
            "{fixture_name} should validate: {:?}",
            packet.validate()
        );
        let support_json = packet.export_safe_support_json();
        assert!(!support_json.contains("://"));
        assert!(support_json.contains("route-receipt:"));
        assert!(support_json.contains("spend-receipt:"));
        assert!(support_json.contains("approval-ticket:"));
    }
}
