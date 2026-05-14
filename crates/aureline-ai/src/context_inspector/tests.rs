use std::path::Path;

use serde::Deserialize;

use crate::composer::{
    AttachmentKind, AttachmentStatusClass, ComposerAttachment, ComposerDraft, ComposerMention,
    MentionKind, MentionResolutionState, SelectionReasonClass, SourceClass, TrustPosture,
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
        policy_epoch_ref: "policy_epoch:alpha:composer-context".to_owned(),
        trust_state: PolicyTrustState::Trusted,
        deployment_profile_class: DeploymentProfileClass::ManagedCloud,
        execution_context_ref: Some("execution_context:composer-context-alpha".to_owned()),
    }
}

fn quota() -> QuotaInspector {
    QuotaInspector {
        quota_family_class: QuotaFamilyClass::VendorHostedEntitlementQuota,
        quota_state_class: QuotaStateClass::WithinLimit,
        quota_scope_class: QuotaScopeClass::VendorHostedEntitlement,
        budget_owner_ref: "quota_owner:workspace:composer-context-alpha".to_owned(),
        quota_meter_ref: Some("quota_meter:composer-context-alpha".to_owned()),
        quota_forecast_ref: Some("quota_forecast:composer-context-alpha".to_owned()),
        usage_export_ref: Some("usage_export:composer-context-alpha".to_owned()),
        explanation_label: "Hosted composer review quota is available.".to_owned(),
        local_continuity_label: "Manual edit and search remain available without hosted AI."
            .to_owned(),
        recovery_action_ref: Some("action:ai:view-quota".to_owned()),
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
        budget_routing_policy_ref: "budget_policy:composer-context-alpha".to_owned(),
        graduation_packet_ref: "graduation_packet:composer-context-alpha".to_owned(),
        envelope_evidence_ref: "envelope_evidence:composer-context-alpha".to_owned(),
        explanation_label: "Route uses the hosted preview band.".to_owned(),
    }
}

fn routing_packet() -> AiRoutingPacket {
    let candidate = AiRouteCandidate {
        candidate_id: "candidate:hosted:composer-context-alpha".to_owned(),
        provider_entry_ref: "provider-entry:first-party:composer-context-alpha".to_owned(),
        provider_label: "Aureline managed hosted AI".to_owned(),
        provider_class: AiRouteProviderClass::FirstPartyManaged,
        model_entry_ref: "model-entry:hosted:composer-context-alpha".to_owned(),
        model_label: "Hosted context review preview".to_owned(),
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
        originating_approval_ticket_ref: Some("approval-ticket:route:composer-context".to_owned()),
        explanation_label: "Hosted route selected for context review.".to_owned(),
    };
    AiRoutingPacket::new(
        "ai_routing_packet:composer-context-alpha:0001",
        "workflow.alpha.ai.composer_context_review",
        "request_workspace:composer-context-alpha",
        RoutingRunStateClass::PreviewPreDispatch,
        policy_context(),
        "capability_lifecycle:alpha.ai.routing_cost",
        Some("identity_mode:composer-context-alpha".to_owned()),
        vec![candidate.clone()],
        candidate.candidate_id,
        Vec::new(),
        vec![
            "docs/ai/routing_cost_alpha.md".to_owned(),
            "docs/ai/context_assembly_contract.md".to_owned(),
        ],
        "2026-05-14T12:00:00Z",
    )
}

fn draft() -> ComposerDraft {
    let mut draft = ComposerDraft::new(
        "turn-draft:composer-context:0001",
        "composer-session:composer-context:0001",
        "request-workspace:composer-context:0001",
        "Review the payment retry path against the pinned docs.",
    );
    draft.add_mention(ComposerMention {
        mention_id: "mention.docs.retry-policy".to_owned(),
        kind: MentionKind::DocsAnchorMention,
        target_stable_id: Some("docs-node:payments:retry-policy".to_owned()),
        display_label: "@payments/retry-policy".to_owned(),
        resolution_state: MentionResolutionState::Resolved,
    });
    draft.add_attachment(ComposerAttachment {
        attachment_id: "att.docs.retry-policy".to_owned(),
        kind: AttachmentKind::RetrievedDocument,
        source_class: SourceClass::DocsPackExcerpt,
        trust_posture: TrustPosture::ReviewedDerived,
        selection_reason: SelectionReasonClass::UserPinned,
        status: AttachmentStatusClass::Live,
        estimated_byte_size: 2048,
        display_label: "Payments retry policy docs".to_owned(),
        scope_truth: None,
        placed_under_fenced_role: false,
    });
    draft
}

fn docs_identity() -> DocsKnowledgeIdentity {
    DocsKnowledgeIdentity {
        docs_node_ref: "docs-node:payments:retry-policy".to_owned(),
        source_class: DocsKnowledgeSourceClass::MirroredDocsPack,
        version_or_revision_ref: "docs-revision:payments:2026-05-01".to_owned(),
        docs_pack_ref: Some("docs-pack:payments".to_owned()),
        exact_anchor_ref: Some("citation-anchor:payments#retry-policy".to_owned()),
        citation_availability_class: CitationAnchorAvailabilityClass::ExactAnchorAvailable,
        citation_note: None,
        source_language: "en-US".to_owned(),
        active_language: "fr-FR".to_owned(),
        source_language_fallback_class: SourceLanguageFallbackClass::FallbackToSourceLanguage,
    }
}

fn review_lock() -> ComposerContextReviewLock {
    ComposerContextReviewLock {
        lock_class: ReviewLockClass::FrozenForReview,
        context_snapshot_ref: "context-snapshot:composer-context:0001".to_owned(),
        route_snapshot_ref: "ai_routing_packet:composer-context-alpha:0001".to_owned(),
        review_started_at: Some("2026-05-14T12:01:00Z".to_owned()),
    }
}

fn pinned_docs_context_item() -> ComposerContextItem {
    ComposerContextItem {
        context_item_id: "ctx.docs.retry-policy".to_owned(),
        group_class: ContextGroupClass::DocsKnowledgeSources,
        state_class: ContextItemStateClass::Pinned,
        source_class: SourceClass::DocsPackExcerpt,
        stable_identity_ref: "docs-node:payments:retry-policy".to_owned(),
        display_label: "Payments retry policy".to_owned(),
        freshness_class: ContextFreshnessClass::WarmCached,
        trust_class: ContextTrustClass::TrustedAuthority,
        locality_class: ContextLocalityClass::MirroredDocsPack,
        estimated_byte_size: 2048,
        omission_reason_class: None,
        source_attachment_ref: Some("att.docs.retry-policy".to_owned()),
        source_mention_ref: Some("mention.docs.retry-policy".to_owned()),
        docs_identity: Some(docs_identity()),
    }
}

#[test]
fn pinned_docs_context_surfaces_citation_and_source_language_fallback() {
    let draft = draft();
    let identity = docs_identity();
    let snapshot = ComposerContextAlphaSnapshot::project(
        &draft,
        &routing_packet(),
        ComposerContextAlphaInput {
            intent_mode: IntentModeClass::ReviewDiff,
            scope_label: "Current diff".to_owned(),
            execution_boundary_class: ExecutionBoundaryClass::ManagedHosted,
            action_identity_ref: Some("cmd:ai.review_diff".to_owned()),
            mention_previews: vec![ComposerMentionPreview {
                mention_id: "mention.docs.retry-policy".to_owned(),
                kind: MentionKind::DocsAnchorMention,
                preview_state: MentionPreviewStateClass::ResolvedExact,
                target_stable_id: Some("docs-node:payments:retry-policy".to_owned()),
                candidate_target_refs: Vec::new(),
                display_label: "@payments/retry-policy".to_owned(),
                docs_identity: Some(identity.clone()),
            }],
            attachment_pills: vec![ComposerAttachmentPill {
                attachment_id: "att.docs.retry-policy".to_owned(),
                kind: AttachmentKind::RetrievedDocument,
                source_class: SourceClass::DocsPackExcerpt,
                trust_posture: TrustPosture::ReviewedDerived,
                selection_reason: SelectionReasonClass::UserPinned,
                status: AttachmentStatusClass::Live,
                context_state: ContextItemStateClass::Pinned,
                display_label: "Payments retry policy docs".to_owned(),
                estimated_byte_size: 2048,
                removable: true,
                docs_identity: Some(identity),
            }],
            context_items: vec![
                ComposerContextItem {
                    context_item_id: "ctx.file.retry".to_owned(),
                    group_class: ContextGroupClass::OpenFiles,
                    state_class: ContextItemStateClass::Included,
                    source_class: SourceClass::WorkspaceFileSlice,
                    stable_identity_ref: "vfs-slice:payments:retry.rs#L18-L92".to_owned(),
                    display_label: "retry.rs lines 18-92".to_owned(),
                    freshness_class: ContextFreshnessClass::AuthoritativeLive,
                    trust_class: ContextTrustClass::TrustedFirstParty,
                    locality_class: ContextLocalityClass::LocalWorkspace,
                    estimated_byte_size: 4096,
                    omission_reason_class: None,
                    source_attachment_ref: None,
                    source_mention_ref: None,
                    docs_identity: None,
                },
                pinned_docs_context_item(),
            ],
            review_lock: review_lock(),
        },
    );

    assert_eq!(
        snapshot.review_state,
        ComposerContextReviewState::ReadyToSend
    );
    assert!(snapshot.validate().is_empty());
    assert_eq!(
        snapshot.budget_strip.included_context_group_tokens,
        vec!["open_files".to_owned(), "docs_knowledge_sources".to_owned()]
    );

    let handoff = snapshot.evidence_handoff("context-handoff:composer-context:0001");
    assert!(handoff.validate().is_empty());
    let docs_row = handoff
        .context_rows
        .iter()
        .find(|row| row.context_item_id == "ctx.docs.retry-policy")
        .expect("docs row");
    assert_eq!(
        docs_row.docs_node_ref.as_deref(),
        Some("docs-node:payments:retry-policy")
    );
    assert_eq!(
        docs_row.exact_anchor_ref.as_deref(),
        Some("citation-anchor:payments#retry-policy")
    );
    assert_eq!(
        docs_row.source_language_fallback_token.as_deref(),
        Some("fallback_to_source_language")
    );
}

#[test]
fn ambiguity_blocked_context_taint_and_budget_overflow_are_visible() {
    let mut draft = draft();
    draft.budget_byte_ceiling = 3_000;
    draft.add_attachment(ComposerAttachment {
        attachment_id: "att.runtime.tainted".to_owned(),
        kind: AttachmentKind::TerminalLogCapture,
        source_class: SourceClass::TerminalTranscriptExcerpt,
        trust_posture: TrustPosture::UntrustedExternal,
        selection_reason: SelectionReasonClass::TerminalCaptureAttached,
        status: AttachmentStatusClass::TaintedOutsideFencedSection,
        estimated_byte_size: 2_000,
        display_label: "terminal output with instructions".to_owned(),
        scope_truth: None,
        placed_under_fenced_role: true,
    });

    let snapshot = ComposerContextAlphaSnapshot::project(
        &draft,
        &routing_packet(),
        ComposerContextAlphaInput {
            intent_mode: IntentModeClass::DraftPatch,
            scope_label: "Selected workset".to_owned(),
            execution_boundary_class: ExecutionBoundaryClass::ManagedHosted,
            action_identity_ref: Some("cmd:ai.draft_patch".to_owned()),
            mention_previews: vec![ComposerMentionPreview {
                mention_id: "mention.route".to_owned(),
                kind: MentionKind::SymbolMention,
                preview_state: MentionPreviewStateClass::Ambiguous,
                target_stable_id: None,
                candidate_target_refs: vec![
                    "symbol:Route.retry".to_owned(),
                    "symbol:Route.retryBackoff".to_owned(),
                ],
                display_label: "@Route".to_owned(),
                docs_identity: None,
            }],
            attachment_pills: Vec::new(),
            context_items: vec![
                pinned_docs_context_item(),
                ComposerContextItem {
                    context_item_id: "ctx.symbol.ambiguous".to_owned(),
                    group_class: ContextGroupClass::SymbolsGraphEntities,
                    state_class: ContextItemStateClass::Blocked,
                    source_class: SourceClass::WorkspaceSymbol,
                    stable_identity_ref: "symbol-set:route-ambiguous".to_owned(),
                    display_label: "Route candidates".to_owned(),
                    freshness_class: ContextFreshnessClass::AuthoritativeLive,
                    trust_class: ContextTrustClass::TrustedFirstParty,
                    locality_class: ContextLocalityClass::LocalWorkspace,
                    estimated_byte_size: 512,
                    omission_reason_class: Some(ContextOmissionReasonClass::Blocked),
                    source_attachment_ref: None,
                    source_mention_ref: Some("mention.route".to_owned()),
                    docs_identity: None,
                },
                ComposerContextItem {
                    context_item_id: "ctx.diff.omitted".to_owned(),
                    group_class: ContextGroupClass::DiffsHistory,
                    state_class: ContextItemStateClass::Omitted,
                    source_class: SourceClass::WorkspaceSearchResult,
                    stable_identity_ref: "diff-history:payments:large".to_owned(),
                    display_label: "Large prior diff".to_owned(),
                    freshness_class: ContextFreshnessClass::WarmCached,
                    trust_class: ContextTrustClass::ReviewedDerived,
                    locality_class: ContextLocalityClass::LocalCache,
                    estimated_byte_size: 4096,
                    omission_reason_class: Some(ContextOmissionReasonClass::Budget),
                    source_attachment_ref: None,
                    source_mention_ref: None,
                    docs_identity: None,
                },
                ComposerContextItem {
                    context_item_id: "ctx.runtime.tainted".to_owned(),
                    group_class: ContextGroupClass::RuntimeArtifacts,
                    state_class: ContextItemStateClass::Tainted,
                    source_class: SourceClass::TerminalTranscriptExcerpt,
                    stable_identity_ref: "terminal-capture:retry-run:0001".to_owned(),
                    display_label: "Terminal output".to_owned(),
                    freshness_class: ContextFreshnessClass::WarmCached,
                    trust_class: ContextTrustClass::UntrustedExternal,
                    locality_class: ContextLocalityClass::RemoteRuntime,
                    estimated_byte_size: 2048,
                    omission_reason_class: Some(ContextOmissionReasonClass::Tainted),
                    source_attachment_ref: Some("att.runtime.tainted".to_owned()),
                    source_mention_ref: None,
                    docs_identity: None,
                },
            ],
            review_lock: review_lock(),
        },
    );

    assert_eq!(
        snapshot.review_state,
        ComposerContextReviewState::BudgetReviewRequired
    );
    assert_eq!(
        snapshot.budget_strip.pressure_class,
        BudgetPressureClass::Overflow
    );
    assert!(snapshot.validate().is_empty());
    assert!(snapshot
        .mention_previews
        .iter()
        .any(|mention| mention.preview_state == MentionPreviewStateClass::Ambiguous));
    assert!(snapshot
        .budget_strip
        .omitted_or_trimmed_group_tokens
        .contains(&"runtime_artifacts".to_owned()));

    let handoff = snapshot.evidence_handoff("context-handoff:overflow:0001");
    let state_tokens: Vec<_> = handoff
        .context_rows
        .iter()
        .map(|row| row.state_token.as_str())
        .collect();
    assert!(state_tokens.contains(&"pinned"));
    assert!(state_tokens.contains(&"blocked"));
    assert!(state_tokens.contains(&"omitted"));
    assert!(state_tokens.contains(&"tainted"));
}

#[test]
fn invalid_docs_anchor_and_missing_source_language_fallback_are_rejected() {
    let mut item = pinned_docs_context_item();
    let docs = item.docs_identity.as_mut().expect("docs identity");
    docs.exact_anchor_ref = None;
    docs.source_language_fallback_class = SourceLanguageFallbackClass::SourceLanguageUnavailable;

    let snapshot = ComposerContextAlphaSnapshot::project(
        &draft(),
        &routing_packet(),
        ComposerContextAlphaInput {
            intent_mode: IntentModeClass::Ask,
            scope_label: "Current root".to_owned(),
            execution_boundary_class: ExecutionBoundaryClass::ManagedHosted,
            action_identity_ref: None,
            mention_previews: Vec::new(),
            attachment_pills: Vec::new(),
            context_items: vec![item],
            review_lock: review_lock(),
        },
    );
    let violations = snapshot.validate();
    assert!(violations.contains(&ComposerContextAlphaViolation::DocsCitationAnchorMissing));
    assert!(violations.contains(&ComposerContextAlphaViolation::SourceLanguageFallbackMissing));
}

#[test]
fn fixtures_describe_protected_alpha_cases() {
    let fixture_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/ai/composer_context_alpha");
    for fixture_name in [
        "pinned_docs_citation_fallback.json",
        "ambiguity_blocked_budget_overflow.json",
    ] {
        let payload =
            std::fs::read_to_string(fixture_dir.join(fixture_name)).expect("fixture reads");
        let fixture: AlphaFixture = serde_json::from_str(&payload).expect("fixture parses");
        assert_eq!(fixture.record_kind, "ai_composer_context_alpha_case");
        assert_eq!(
            fixture.schema_version,
            COMPOSER_CONTEXT_ALPHA_SCHEMA_VERSION
        );
        assert!(
            !fixture.expected_context_state_tokens.is_empty(),
            "{fixture_name} must name expected context states"
        );
        assert!(
            !fixture.expected_handoff_state_tokens.is_empty(),
            "{fixture_name} must name handoff states"
        );
        if fixture_name == "pinned_docs_citation_fallback.json" {
            assert_eq!(
                fixture.expected_docs_node_ref.as_deref(),
                Some("docs-node:payments:retry-policy")
            );
            assert_eq!(
                fixture.expected_citation_availability.as_deref(),
                Some("exact_anchor_available")
            );
            assert_eq!(
                fixture.expected_source_language_fallback.as_deref(),
                Some("fallback_to_source_language")
            );
        }
    }
}

#[derive(Debug, Deserialize)]
struct AlphaFixture {
    record_kind: String,
    schema_version: u32,
    expected_context_state_tokens: Vec<String>,
    expected_handoff_state_tokens: Vec<String>,
    #[serde(default)]
    expected_docs_node_ref: Option<String>,
    #[serde(default)]
    expected_citation_availability: Option<String>,
    #[serde(default)]
    expected_source_language_fallback: Option<String>,
}
