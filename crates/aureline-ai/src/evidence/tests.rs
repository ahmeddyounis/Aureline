use std::path::Path;

use aureline_docs::{
    CitationAnchorAlpha, CitationAnchorAlphaInput,
    CitationAnchorAvailability as DocsCitationAnchorAvailability, CitationConfidenceClass,
    CitationInferenceMarker, CitationLocalityClass, CitationSourceClass,
    DocsFreshnessClass as CanonicalDocsFreshnessClass, DocsNodeIdentity, DocsNodeIdentityInput,
    DocsNodeKind, DocsScopeClass, LocaleOverlayState, VersionMatchState as DocsVersionMatchState,
};

use crate::context_inspector::{
    AiContextEvidenceHandoff, AiContextEvidenceHandoffRow, AI_CONTEXT_EVIDENCE_HANDOFF_RECORD_KIND,
    COMPOSER_CONTEXT_ALPHA_SCHEMA_VERSION,
};
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

fn payments_docs_node_identity() -> DocsNodeIdentity {
    DocsNodeIdentity::new(DocsNodeIdentityInput {
        docs_node_id: "docs-node:payments-guide:create-charge".to_owned(),
        doc_kind: DocsNodeKind::ReferencePage,
        source_class: CitationSourceClass::MirroredOfficialDocs,
        scope_class: DocsScopeClass::AiEvidence,
        source_pack_ref: "docs-pack:payments-guide".to_owned(),
        source_pack_revision_ref: "docs-pack-revision:payments-guide:2026-05-01".to_owned(),
        version_or_revision_ref: "doc-revision:payments-guide:2026-05-01".to_owned(),
        version_match_state: DocsVersionMatchState::ExactBuildMatch,
        freshness_class: CanonicalDocsFreshnessClass::AuthoritativeLive,
        locality_class: CitationLocalityClass::MirroredOffline,
        source_locale: "en-US".to_owned(),
        requested_locale: "en-US".to_owned(),
        effective_locale: "en-US".to_owned(),
        locale_overlay_state: LocaleOverlayState::SourceLanguageOriginal,
        source_language_fallback_ref: None,
        citation_availability: DocsCitationAnchorAvailability::ExactAnchorAvailable,
        citation_anchor_refs: vec!["citation-anchor:payments-guide#create-charge".to_owned()],
        exact_reopen_ref: "reopen:docs-node:payments-guide:create-charge@docs-pack-revision:payments-guide:2026-05-01#en-US".to_owned(),
        hidden_or_omitted_note: None,
    })
}

fn payments_docs_citation_anchor() -> CitationAnchorAlpha {
    CitationAnchorAlpha::new(CitationAnchorAlphaInput {
        anchor_id: "citation-anchor:payments-guide#create-charge".to_owned(),
        docs_node_ref: "docs-node:payments-guide:create-charge".to_owned(),
        source_class: CitationSourceClass::MirroredOfficialDocs,
        source_pack_ref: "docs-pack:payments-guide".to_owned(),
        source_pack_revision_ref: "docs-pack-revision:payments-guide:2026-05-01".to_owned(),
        target_ref: "docs-node:payments-guide:create-charge".to_owned(),
        exact_anchor_ref: Some("citation-anchor:payments-guide#create-charge".to_owned()),
        locale: "en-US".to_owned(),
        version_match_state: DocsVersionMatchState::ExactBuildMatch,
        freshness_class: CanonicalDocsFreshnessClass::AuthoritativeLive,
        locality_class: CitationLocalityClass::MirroredOffline,
        citation_availability: DocsCitationAnchorAvailability::ExactAnchorAvailable,
        inference_marker: CitationInferenceMarker::RawSource,
        confidence_class: CitationConfidenceClass::EvidenceBacked,
        hidden_or_omitted_note: None,
    })
}

fn glossary_docs_node_identity() -> DocsNodeIdentity {
    DocsNodeIdentity::new(DocsNodeIdentityInput {
        docs_node_id: "glossary-entry:payments:charge-state".to_owned(),
        doc_kind: DocsNodeKind::GlossaryItem,
        source_class: CitationSourceClass::CuratedKnowledgePack,
        scope_class: DocsScopeClass::HelpPack,
        source_pack_ref: "glossary-pack:payments".to_owned(),
        source_pack_revision_ref: "glossary-pack-revision:payments:2026-05-01".to_owned(),
        version_or_revision_ref: "glossary-revision:payments:2026-05-01".to_owned(),
        version_match_state: DocsVersionMatchState::CompatibleMinorDrift,
        freshness_class: CanonicalDocsFreshnessClass::WarmCached,
        locality_class: CitationLocalityClass::CachedLocal,
        source_locale: "en-US".to_owned(),
        requested_locale: "en-US".to_owned(),
        effective_locale: "en-US".to_owned(),
        locale_overlay_state: LocaleOverlayState::SourceLanguageOriginal,
        source_language_fallback_ref: None,
        citation_availability: DocsCitationAnchorAvailability::AnchorUnavailableDisclosed,
        citation_anchor_refs: Vec::new(),
        exact_reopen_ref: "reopen:glossary-entry:payments:charge-state@glossary-pack-revision:payments:2026-05-01#en-US".to_owned(),
        hidden_or_omitted_note: Some(
            "Glossary entry came from a mirrored pack that does not expose section anchors."
                .to_owned(),
        ),
    })
}

fn glossary_docs_citation_anchor() -> CitationAnchorAlpha {
    CitationAnchorAlpha::new(CitationAnchorAlphaInput {
        anchor_id: "citation-anchor:glossary:payments:charge-state:missing".to_owned(),
        docs_node_ref: "glossary-entry:payments:charge-state".to_owned(),
        source_class: CitationSourceClass::CuratedKnowledgePack,
        source_pack_ref: "glossary-pack:payments".to_owned(),
        source_pack_revision_ref: "glossary-pack-revision:payments:2026-05-01".to_owned(),
        target_ref: "glossary-entry:payments:charge-state".to_owned(),
        exact_anchor_ref: None,
        locale: "en-US".to_owned(),
        version_match_state: DocsVersionMatchState::CompatibleMinorDrift,
        freshness_class: CanonicalDocsFreshnessClass::WarmCached,
        locality_class: CitationLocalityClass::CachedLocal,
        citation_availability: DocsCitationAnchorAvailability::AnchorUnavailableDisclosed,
        inference_marker: CitationInferenceMarker::GeneratedSummary,
        confidence_class: CitationConfidenceClass::Inferred,
        hidden_or_omitted_note: Some(
            "Glossary entry came from a mirrored pack that does not expose section anchors."
                .to_owned(),
        ),
    })
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
            docs_node_identity: None,
            citation_anchor: None,
            citation_visibility_class: CitationVisibilityClass::NotCitationBearing,
            hidden_or_omitted_citation_note: None,
            source_posture: EvidenceSourcePosture::TrustedFirstParty,
            freshness_class: EvidenceFreshnessClass::AuthoritativeLive,
        },
        CitedSourceReference::from_docs_citation(
            "source-ref:docs-pack:payments-guide",
            payments_docs_node_identity(),
            payments_docs_citation_anchor(),
            EvidenceSourcePosture::TrustedAuthority,
        ),
        CitedSourceReference::from_docs_citation(
            "source-ref:glossary:charge-state",
            glossary_docs_node_identity(),
            glossary_docs_citation_anchor(),
            EvidenceSourcePosture::ReviewedDerived,
        ),
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
        context_handoff: None,
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
        docs_node_identity: None,
        citation_anchor: None,
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

fn context_handoff() -> AiContextEvidenceHandoff {
    AiContextEvidenceHandoff {
        record_kind: AI_CONTEXT_EVIDENCE_HANDOFF_RECORD_KIND.to_owned(),
        schema_version: COMPOSER_CONTEXT_ALPHA_SCHEMA_VERSION,
        handoff_id: "context-handoff:mutation-alpha:0001".to_owned(),
        composer_context_snapshot_ref: "context-snapshot:mutation-alpha:0001".to_owned(),
        composer_session_ref: "composer-session:ai-mutation:alpha:0001".to_owned(),
        turn_draft_ref: "turn-draft:ai-mutation:alpha:0001".to_owned(),
        request_workspace_ref: "request-workspace:ai-mutation:alpha:0001".to_owned(),
        context_rows: vec![
            AiContextEvidenceHandoffRow {
                context_item_id: "ctx.docs.payments-guide".to_owned(),
                group_token: "docs_knowledge_sources".to_owned(),
                state_token: "pinned".to_owned(),
                source_class_token: "docs_pack_excerpt".to_owned(),
                stable_identity_ref: "docs-node:payments-guide:create-charge".to_owned(),
                freshness_token: "authoritative_live".to_owned(),
                trust_token: "trusted_authority".to_owned(),
                locality_token: "mirrored_docs_pack".to_owned(),
                omission_reason_token: None,
                source_attachment_ref: Some("att.docs.payments-guide".to_owned()),
                source_mention_ref: Some("mention.docs.payments-guide".to_owned()),
                docs_node_ref: Some("docs-node:payments-guide:create-charge".to_owned()),
                docs_source_class_token: Some("mirrored_docs_pack".to_owned()),
                version_or_revision_ref: Some("doc-revision:payments-guide:2026-05-01".to_owned()),
                exact_anchor_ref: Some("citation-anchor:payments-guide#create-charge".to_owned()),
                citation_availability_token: Some("exact_anchor_available".to_owned()),
                source_language_fallback_token: Some("fallback_to_source_language".to_owned()),
            },
            AiContextEvidenceHandoffRow {
                context_item_id: "ctx.history.omitted".to_owned(),
                group_token: "diffs_history".to_owned(),
                state_token: "omitted".to_owned(),
                source_class_token: "workspace_search_result".to_owned(),
                stable_identity_ref: "diff-history:payments:large".to_owned(),
                freshness_token: "warm_cached".to_owned(),
                trust_token: "reviewed_derived".to_owned(),
                locality_token: "local_cache".to_owned(),
                omission_reason_token: Some("budget".to_owned()),
                source_attachment_ref: None,
                source_mention_ref: None,
                docs_node_ref: None,
                docs_source_class_token: None,
                version_or_revision_ref: None,
                exact_anchor_ref: None,
                citation_availability_token: None,
                source_language_fallback_token: None,
            },
        ],
        graph_cue_packets: Vec::new(),
    }
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
                && row
                    .docs_node_identity
                    .as_ref()
                    .is_some_and(|identity| identity.docs_node_id
                        == "docs-node:payments-guide:create-charge")
                && row
                    .citation_anchor
                    .as_ref()
                    .is_some_and(|anchor| anchor.citation_availability
                        == DocsCitationAnchorAvailability::ExactAnchorAvailable)
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
fn evidence_packet_ingests_context_handoff_without_restating_context_truth() {
    let mut packet = packet(
        MutationEvidenceState::ReviewPreApply,
        ApprovalDecisionClass::PendingUserReview,
        ApplyOutcomeClass::NotAppliedPendingReview,
    );
    packet.context_handoff = Some(context_handoff());

    assert!(packet.validate().is_empty());
    let support = packet.support_packet();
    assert!(support
        .context_handoff_rows
        .iter()
        .any(|row| row.state_token == "pinned"
            && row.docs_node_ref.as_deref() == Some("docs-node:payments-guide:create-charge")
            && row.exact_anchor_ref.as_deref()
                == Some("citation-anchor:payments-guide#create-charge")));
    assert!(support
        .context_handoff_rows
        .iter()
        .any(|row| row.state_token == "omitted"
            && row.omission_reason_token.as_deref() == Some("budget")));

    let mut mismatch = packet;
    mismatch
        .context_handoff
        .as_mut()
        .expect("handoff")
        .request_workspace_ref = "request-workspace:other".to_owned();
    assert!(mismatch
        .validate()
        .contains(&AiMutationEvidenceViolation::ContextHandoffIdentityMismatch));
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
