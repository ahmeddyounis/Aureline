//! Canonical seed builders for the M5 AI-message-card / evidence-timeline controls packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce. Clean AI message
//! cards and evidence timelines are built so the shared message-state / source-context / confidence /
//! route / spend grammar and the evidence-kind / lineage / redaction grammar are proven across the
//! editor, review, notebook, AI, support, and product surfaces without any generic completed message,
//! hidden approval, undisclosed source or spend, opaque log, or hidden redaction.

use super::*;

/// Stable packet id for the canonical controls packet.
pub const M5_AI_EVIDENCE_CONTROLS_PACKET_ID: &str =
    "m5-ai-message-card-evidence-timeline-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-12T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn card(input: M5AiMessageCardResolutionInput) -> M5ResolvedAiMessageCard {
    resolve_ai_message_card(input).expect("seed ai-message-card input resolves")
}

fn evidence(input: M5EvidenceTimelineResolutionInput) -> M5ResolvedEvidenceTimeline {
    resolve_evidence_timeline(input).expect("seed evidence-timeline input resolves")
}

// -- Clean AI-message-card examples (shared state/source/route/spend grammar) --------------------

#[allow(clippy::too_many_arguments)]
fn clean_card_base(
    card_id: &str,
    message_label: &str,
    state: M5AiMessageState,
    confidence: M5AiConfidence,
    source: M5AiSourceContext,
    route: M5AiRouteLocality,
    spend: M5AiSpendPosture,
) -> M5AiMessageCardResolutionInput {
    M5AiMessageCardResolutionInput {
        card_id: card_id.to_owned(),
        message_label: message_label.to_owned(),
        message_state: state,
        state_stated: true,
        approval_state_disclosed: true,
        confidence,
        confidence_stated: true,
        source_context: source,
        source_disclosed: true,
        route_locality: route,
        route_distinction_explicit: true,
        spend_posture: spend,
        spend_disclosed: true,
        safe_actions_available: true,
        detail_command_available: true,
        proof_fresh: true,
    }
}

/// Clean draft message grounded in the workspace on a local model.
fn card_draft_local_clean() -> M5ResolvedAiMessageCard {
    card(clean_card_base(
        "card:ai:draft-31",
        "draft: proposed refactor of `parse_header`, awaiting your review",
        M5AiMessageState::Draft,
        M5AiConfidence::GroundedHigh,
        M5AiSourceContext::GroundedInWorkspace,
        M5AiRouteLocality::LocalModel,
        M5AiSpendPosture::NoCost,
    ))
}

/// Clean streaming message from a hosted provider with a disclosed metered cost.
fn card_streaming_hosted_clean() -> M5ResolvedAiMessageCard {
    card(clean_card_base(
        "card:ai:streaming-12",
        "streaming: drafting an explanation grounded in the docs",
        M5AiMessageState::Streaming,
        M5AiConfidence::StreamingPartial,
        M5AiSourceContext::GroundedInDocs,
        M5AiRouteLocality::HostedProvider,
        M5AiSpendPosture::MeteredHosted,
    ))
}

/// Clean review-required message that discloses an external source before it may be applied.
fn card_review_required_clean() -> M5ResolvedAiMessageCard {
    card(clean_card_base(
        "card:review:review-required-8",
        "review required: retrieved from an external reference (disclosed) before apply",
        M5AiMessageState::ReviewRequired,
        M5AiConfidence::GroundedMedium,
        M5AiSourceContext::RetrievedExternal,
        M5AiRouteLocality::HostedProvider,
        M5AiSpendPosture::MeteredHosted,
    ))
}

/// Clean applied message grounded in the workspace on a local model.
fn card_applied_local_clean() -> M5ResolvedAiMessageCard {
    card(clean_card_base(
        "card:editor:applied-19",
        "applied: inserted the generated test for the empty-input case",
        M5AiMessageState::Applied,
        M5AiConfidence::GroundedHigh,
        M5AiSourceContext::GroundedInWorkspace,
        M5AiRouteLocality::LocalModel,
        M5AiSpendPosture::NoCost,
    ))
}

/// Clean blocked-by-policy message that discloses approval, a model-prior source, and a capped spend.
fn card_blocked_disclosed_clean() -> M5ResolvedAiMessageCard {
    card(clean_card_base(
        "card:support:blocked-5",
        "blocked by policy: model-prior answer (disclosed), needs approval to apply",
        M5AiMessageState::BlockedByPolicy,
        M5AiConfidence::LowConfidence,
        M5AiSourceContext::ModelPriorOnly,
        M5AiRouteLocality::ByoKeyProvider,
        M5AiSpendPosture::BudgetCapped,
    ))
}

/// Clean stale-evidence message from a mirrored cache.
fn card_stale_evidence_clean() -> M5ResolvedAiMessageCard {
    card(clean_card_base(
        "card:notebook:stale-3",
        "stale evidence: supporting run has since changed; re-run to refresh",
        M5AiMessageState::StaleEvidence,
        M5AiConfidence::GroundedMedium,
        M5AiSourceContext::GroundedInWorkspace,
        M5AiRouteLocality::MirroredCache,
        M5AiSpendPosture::MeteredLocal,
    ))
}

// -- Degraded AI-message-card examples ----------------------------------------------------------

/// Degraded card: the message identity / label is unstated.
fn card_identity_unstated() -> M5ResolvedAiMessageCard {
    let mut input = clean_card_base(
        "card:support:no-label",
        "   ",
        M5AiMessageState::Applied,
        M5AiConfidence::GroundedHigh,
        M5AiSourceContext::GroundedInWorkspace,
        M5AiRouteLocality::LocalModel,
        M5AiSpendPosture::NoCost,
    );
    input.message_label = "   ".to_owned();
    card(input)
}

/// Degraded card: the message state cannot be resolved.
fn card_state_unresolved() -> M5ResolvedAiMessageCard {
    card(clean_card_base(
        "card:notebook:state-unknown",
        "message with no resolvable lifecycle state",
        M5AiMessageState::StateUnknown,
        M5AiConfidence::GroundedMedium,
        M5AiSourceContext::GroundedInWorkspace,
        M5AiRouteLocality::LocalModel,
        M5AiSpendPosture::NoCost,
    ))
}

/// Degraded card: the state is encoded generically as one completed message.
fn card_state_generic() -> M5ResolvedAiMessageCard {
    let mut input = clean_card_base(
        "card:ai:state-generic",
        "message collapsed to a single generic completed bubble",
        M5AiMessageState::Applied,
        M5AiConfidence::GroundedHigh,
        M5AiSourceContext::GroundedInWorkspace,
        M5AiRouteLocality::LocalModel,
        M5AiSpendPosture::NoCost,
    );
    input.state_stated = false;
    card(input)
}

/// Degraded card: a review-required message hides its approval state and reads as applied.
fn card_approval_hidden() -> M5ResolvedAiMessageCard {
    let mut input = clean_card_base(
        "card:review:approval-hidden",
        "review-required message presented as if already applied",
        M5AiMessageState::ReviewRequired,
        M5AiConfidence::GroundedMedium,
        M5AiSourceContext::GroundedInWorkspace,
        M5AiRouteLocality::HostedProvider,
        M5AiSpendPosture::MeteredHosted,
    );
    input.approval_state_disclosed = false;
    card(input)
}

/// Degraded card: the confidence / uncertainty class is unstated.
fn card_confidence_unstated() -> M5ResolvedAiMessageCard {
    let mut input = clean_card_base(
        "card:editor:confidence-unstated",
        "message with no stated confidence / uncertainty class",
        M5AiMessageState::Applied,
        M5AiConfidence::Unverified,
        M5AiSourceContext::GroundedInWorkspace,
        M5AiRouteLocality::LocalModel,
        M5AiSpendPosture::NoCost,
    );
    input.confidence_stated = false;
    card(input)
}

/// Degraded card: the source context cannot be resolved.
fn card_source_unresolved() -> M5ResolvedAiMessageCard {
    card(clean_card_base(
        "card:support:source-unknown",
        "message with no resolvable source context",
        M5AiMessageState::Applied,
        M5AiConfidence::GroundedMedium,
        M5AiSourceContext::SourceUnresolved,
        M5AiRouteLocality::LocalModel,
        M5AiSpendPosture::NoCost,
    ))
}

/// Degraded card: a non-workspace source context is not disclosed.
fn card_source_not_disclosed() -> M5ResolvedAiMessageCard {
    let mut input = clean_card_base(
        "card:review:source-not-disclosed",
        "external answer presented as if workspace-grounded",
        M5AiMessageState::Applied,
        M5AiConfidence::GroundedMedium,
        M5AiSourceContext::RetrievedExternal,
        M5AiRouteLocality::HostedProvider,
        M5AiSpendPosture::MeteredHosted,
    );
    input.source_disclosed = false;
    card(input)
}

/// Degraded card: the route / provider locality cannot be resolved.
fn card_route_unresolved() -> M5ResolvedAiMessageCard {
    card(clean_card_base(
        "card:product:route-unknown",
        "message with no resolvable route / provider locality",
        M5AiMessageState::Applied,
        M5AiConfidence::GroundedMedium,
        M5AiSourceContext::GroundedInWorkspace,
        M5AiRouteLocality::LocalityUnresolved,
        M5AiSpendPosture::NoCost,
    ))
}

/// Degraded card: the local-versus-hosted-provider distinction is implicit.
fn card_route_implicit() -> M5ResolvedAiMessageCard {
    let mut input = clean_card_base(
        "card:product:route-implicit",
        "hosted answer leaving local-vs-hosted provider implicit",
        M5AiMessageState::Applied,
        M5AiConfidence::GroundedMedium,
        M5AiSourceContext::GroundedInDocs,
        M5AiRouteLocality::HostedProvider,
        M5AiSpendPosture::MeteredHosted,
    );
    input.route_distinction_explicit = false;
    card(input)
}

/// Degraded card: the spend posture cannot be resolved.
fn card_spend_unresolved() -> M5ResolvedAiMessageCard {
    card(clean_card_base(
        "card:product:spend-unknown",
        "message with no resolvable spend posture",
        M5AiMessageState::Applied,
        M5AiConfidence::GroundedMedium,
        M5AiSourceContext::GroundedInWorkspace,
        M5AiRouteLocality::HostedProvider,
        M5AiSpendPosture::SpendUnresolved,
    ))
}

/// Degraded card: a metered / over-budget spend is not disclosed.
fn card_spend_not_disclosed() -> M5ResolvedAiMessageCard {
    let mut input = clean_card_base(
        "card:support:spend-not-disclosed",
        "over-budget hosted answer presented as if free",
        M5AiMessageState::Applied,
        M5AiConfidence::GroundedMedium,
        M5AiSourceContext::GroundedInDocs,
        M5AiRouteLocality::HostedProvider,
        M5AiSpendPosture::OverBudget,
    );
    input.spend_disclosed = false;
    card(input)
}

/// Degraded card: no safe actions are offered.
fn card_safe_actions_missing() -> M5ResolvedAiMessageCard {
    let mut input = clean_card_base(
        "card:product:no-safe-actions",
        "message that offers no safe action to trust or apply",
        M5AiMessageState::Applied,
        M5AiConfidence::GroundedMedium,
        M5AiSourceContext::GroundedInWorkspace,
        M5AiRouteLocality::LocalModel,
        M5AiSpendPosture::NoCost,
    );
    input.safe_actions_available = false;
    card(input)
}

/// Degraded card: no command-backed detail path is reachable.
fn card_detail_missing() -> M5ResolvedAiMessageCard {
    let mut input = clean_card_base(
        "card:product:detail-missing",
        "message with no command-backed detail path",
        M5AiMessageState::Applied,
        M5AiConfidence::GroundedMedium,
        M5AiSourceContext::GroundedInWorkspace,
        M5AiRouteLocality::LocalModel,
        M5AiSpendPosture::NoCost,
    );
    input.detail_command_available = false;
    card(input)
}

// -- Clean evidence-timeline examples -----------------------------------------------------------

fn clean_evidence_base(
    timeline_id: &str,
    entry_label: &str,
    kind: M5EvidenceKind,
    lineage: M5EvidenceLineageClass,
    disclosure: M5EvidenceDisclosure,
) -> M5EvidenceTimelineResolutionInput {
    M5EvidenceTimelineResolutionInput {
        timeline_id: timeline_id.to_owned(),
        entry_label: entry_label.to_owned(),
        has_timestamp: true,
        evidence_kind: kind,
        lineage_class: lineage,
        lineage_stated: true,
        related_ref_present: true,
        disclosure,
        redaction_disclosed: true,
        structured_not_opaque: true,
        replay_export_actions_available: true,
        detail_command_available: true,
        proof_fresh: true,
    }
}

/// Clean tool-invocation evidence with tool lineage, fully expanded.
fn evidence_tool_clean() -> M5ResolvedEvidenceTimeline {
    evidence(clean_evidence_base(
        "timeline:ai:tool-7",
        "ran `cargo test parse_header` (tool lineage, related run r-1188)",
        M5EvidenceKind::ToolInvocation,
        M5EvidenceLineageClass::ToolLineage,
        M5EvidenceDisclosure::ExpandedFull,
    ))
}

/// Clean validation-run evidence with validation lineage, fully expanded.
fn evidence_validation_clean() -> M5ResolvedEvidenceTimeline {
    evidence(clean_evidence_base(
        "timeline:ai:validation-4",
        "validated the diff against the empty-input suite (validation lineage)",
        M5EvidenceKind::ValidationRun,
        M5EvidenceLineageClass::ValidationLineage,
        M5EvidenceDisclosure::ExpandedFull,
    ))
}

/// Clean retrieval evidence with resource lineage, collapsed to a summary.
fn evidence_retrieval_clean() -> M5ResolvedEvidenceTimeline {
    evidence(clean_evidence_base(
        "timeline:notebook:retrieval-9",
        "retrieved the header spec (resource lineage; collapsed summary)",
        M5EvidenceKind::Retrieval,
        M5EvidenceLineageClass::ResourceLineage,
        M5EvidenceDisclosure::CollapsedSummary,
    ))
}

/// Clean user-edit evidence with change lineage, fully expanded.
fn evidence_user_edit_clean() -> M5ResolvedEvidenceTimeline {
    evidence(clean_evidence_base(
        "timeline:editor:user-edit-2",
        "user accepted the generated test (change lineage, related change c-42)",
        M5EvidenceKind::UserEdit,
        M5EvidenceLineageClass::ChangeLineage,
        M5EvidenceDisclosure::ExpandedFull,
    ))
}

/// Clean external-reference evidence with run lineage, fully expanded.
fn evidence_external_clean() -> M5ResolvedEvidenceTimeline {
    evidence(clean_evidence_base(
        "timeline:review:external-6",
        "cited an external reference (run lineage, related run r-1201)",
        M5EvidenceKind::ExternalReference,
        M5EvidenceLineageClass::RunLineage,
        M5EvidenceDisclosure::ExpandedFull,
    ))
}

/// Clean redacted evidence that discloses it is export-safe redacted.
fn evidence_redacted_disclosed_clean() -> M5ResolvedEvidenceTimeline {
    evidence(clean_evidence_base(
        "timeline:support:redacted-5",
        "tool output redacted for export (disclosed as redacted; tool lineage)",
        M5EvidenceKind::ToolInvocation,
        M5EvidenceLineageClass::ToolLineage,
        M5EvidenceDisclosure::RedactedExportSafe,
    ))
}

/// Clean partially-loaded evidence that discloses it is incomplete.
fn evidence_partial_disclosed_clean() -> M5ResolvedEvidenceTimeline {
    evidence(clean_evidence_base(
        "timeline:support:partial-11",
        "retrieval trail partially loaded (disclosed as incomplete; run lineage)",
        M5EvidenceKind::Retrieval,
        M5EvidenceLineageClass::RunLineage,
        M5EvidenceDisclosure::PartiallyLoaded,
    ))
}

// -- Degraded evidence-timeline examples --------------------------------------------------------

/// Degraded evidence: the entry identity / label is unstated.
fn evidence_identity_unstated() -> M5ResolvedEvidenceTimeline {
    let mut input = clean_evidence_base(
        "timeline:support:no-label",
        "   ",
        M5EvidenceKind::ToolInvocation,
        M5EvidenceLineageClass::ToolLineage,
        M5EvidenceDisclosure::ExpandedFull,
    );
    input.entry_label = "   ".to_owned();
    evidence(input)
}

/// Degraded evidence: the timestamp is missing.
fn evidence_timestamp_missing() -> M5ResolvedEvidenceTimeline {
    let mut input = clean_evidence_base(
        "timeline:product:no-timestamp",
        "evidence entry with no timestamp",
        M5EvidenceKind::ToolInvocation,
        M5EvidenceLineageClass::ToolLineage,
        M5EvidenceDisclosure::ExpandedFull,
    );
    input.has_timestamp = false;
    evidence(input)
}

/// Degraded evidence: the evidence kind cannot be resolved.
fn evidence_kind_unresolved() -> M5ResolvedEvidenceTimeline {
    evidence(clean_evidence_base(
        "timeline:notebook:kind-unknown",
        "evidence entry with no resolvable kind",
        M5EvidenceKind::KindUnresolved,
        M5EvidenceLineageClass::ToolLineage,
        M5EvidenceDisclosure::ExpandedFull,
    ))
}

/// Degraded evidence: the lineage cannot be resolved.
fn evidence_lineage_unresolved() -> M5ResolvedEvidenceTimeline {
    evidence(clean_evidence_base(
        "timeline:notebook:lineage-unknown",
        "evidence entry with no resolvable lineage",
        M5EvidenceKind::ToolInvocation,
        M5EvidenceLineageClass::LineageUnresolved,
        M5EvidenceDisclosure::ExpandedFull,
    ))
}

/// Degraded evidence: the tool / validation lineage is not stated.
fn evidence_lineage_not_stated() -> M5ResolvedEvidenceTimeline {
    let mut input = clean_evidence_base(
        "timeline:editor:lineage-not-stated",
        "evidence entry that omits its tool / validation lineage",
        M5EvidenceKind::ToolInvocation,
        M5EvidenceLineageClass::ToolLineage,
        M5EvidenceDisclosure::ExpandedFull,
    );
    input.lineage_stated = false;
    evidence(input)
}

/// Degraded evidence: no related run / change / resource is named.
fn evidence_related_missing() -> M5ResolvedEvidenceTimeline {
    let mut input = clean_evidence_base(
        "timeline:support:related-missing",
        "evidence entry with no related run / change / resource",
        M5EvidenceKind::ToolInvocation,
        M5EvidenceLineageClass::ToolLineage,
        M5EvidenceDisclosure::ExpandedFull,
    );
    input.related_ref_present = false;
    evidence(input)
}

/// Degraded evidence: the disclosure state cannot be resolved.
fn evidence_disclosure_unresolved() -> M5ResolvedEvidenceTimeline {
    evidence(clean_evidence_base(
        "timeline:support:disclosure-unknown",
        "evidence entry with no resolvable disclosure state",
        M5EvidenceKind::ToolInvocation,
        M5EvidenceLineageClass::ToolLineage,
        M5EvidenceDisclosure::DisclosureUnknown,
    ))
}

/// Degraded evidence: a redacted timeline hides that it is incomplete.
fn evidence_redaction_hidden() -> M5ResolvedEvidenceTimeline {
    let mut input = clean_evidence_base(
        "timeline:review:redaction-hidden",
        "redacted trail presented as if complete",
        M5EvidenceKind::ToolInvocation,
        M5EvidenceLineageClass::ToolLineage,
        M5EvidenceDisclosure::RedactedExportSafe,
    );
    input.redaction_disclosed = false;
    evidence(input)
}

/// Degraded evidence: the trail is an opaque log rather than an inspectable structure.
fn evidence_opaque() -> M5ResolvedEvidenceTimeline {
    let mut input = clean_evidence_base(
        "timeline:ai:opaque-log",
        "evidence hidden in an opaque log with no inspectable structure",
        M5EvidenceKind::ToolInvocation,
        M5EvidenceLineageClass::ToolLineage,
        M5EvidenceDisclosure::ExpandedFull,
    );
    input.structured_not_opaque = false;
    evidence(input)
}

/// Degraded evidence: no open / replay / export action is offered.
fn evidence_replay_missing() -> M5ResolvedEvidenceTimeline {
    let mut input = clean_evidence_base(
        "timeline:product:replay-missing",
        "evidence entry offering no open / replay / export action",
        M5EvidenceKind::ToolInvocation,
        M5EvidenceLineageClass::ToolLineage,
        M5EvidenceDisclosure::ExpandedFull,
    );
    input.replay_export_actions_available = false;
    evidence(input)
}

/// Degraded evidence: no command-backed detail path is reachable.
fn evidence_detail_missing() -> M5ResolvedEvidenceTimeline {
    let mut input = clean_evidence_base(
        "timeline:product:detail-missing",
        "evidence entry with no command-backed detail path",
        M5EvidenceKind::ToolInvocation,
        M5EvidenceLineageClass::ToolLineage,
        M5EvidenceDisclosure::ExpandedFull,
    );
    input.detail_command_available = false;
    evidence(input)
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5AiEvidenceConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5EditorInlineDowngradeTrigger>,
    card_examples: Vec<M5ResolvedAiMessageCard>,
    evidence_examples: Vec<M5ResolvedEvidenceTimeline>,
) -> M5AiEvidenceControlsRow {
    M5AiEvidenceControlsRow {
        consumer_surface,
        qualification: M5EditorInlineQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5EditorInlineDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5EditorInlineRequiredLabel::Identity,
            M5EditorInlineRequiredLabel::State,
            M5EditorInlineRequiredLabel::KeyboardRoute,
            M5EditorInlineRequiredLabel::ConfidenceAndSource,
            M5EditorInlineRequiredLabel::EvidenceLineage,
        ],
        accessibility_routes: M5EditorInlineAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5AiEvidenceAnatomyPart::ALL.to_vec(),
        export_fields: M5AiEvidenceExportField::ALL.to_vec(),
        downgrade_triggers,
        card_examples,
        evidence_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_AI_EVIDENCE_CONTROLS_SCHEMA_REF,
            M5_AI_MESSAGE_CARD_SCHEMA_REF,
            M5_EVIDENCE_TIMELINE_SCHEMA_REF,
        ]),
        ai_message_state_or_source_context_silently_generic: false,
        ai_route_or_spend_posture_silently_drifts: false,
        evidence_timeline_hidden_in_opaque_log: false,
        evidence_lineage_or_redaction_truth_silently_drifts: false,
    }
}

fn controls_rows() -> Vec<M5AiEvidenceControlsRow> {
    use M5EditorInlineConsumerSurface as C;
    use M5EditorInlineDowngradeTrigger as D;

    vec![
        base_row(
            C::AiUi,
            "AI surface owner",
            "The AI surface names draft, streaming, review-required, blocked, applied, reverted, failed, and stale-evidence message states with one controlled vocabulary and renders evidence as an inspectable timeline; both degrade honestly when a state is encoded generically or the evidence is hidden in an opaque log",
            "evidence:m5-ai-evidence-ai-ui:001",
            vec![
                D::AiConfidenceUnstated,
                D::EvidenceTimelineOpaqueLog,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                card_draft_local_clean(),
                card_streaming_hosted_clean(),
                card_state_generic(),
            ],
            vec![
                evidence_tool_clean(),
                evidence_validation_clean(),
                evidence_opaque(),
            ],
        ),
        base_row(
            C::EditorUi,
            "Editor AI owner",
            "The editor renders AI cards with the same state / source / confidence grammar and evidence entries with stated lineage, degrading honestly when a card leaves its confidence unstated or an evidence entry omits its tool / validation lineage",
            "evidence:m5-ai-evidence-editor-ui:001",
            vec![
                D::AiConfidenceUnstated,
                D::EvidencePointerDriftedSilently,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![card_applied_local_clean(), card_confidence_unstated()],
            vec![evidence_user_edit_clean(), evidence_lineage_not_stated()],
        ),
        base_row(
            C::ReviewUi,
            "Review AI owner",
            "The review surface keeps approval state and external-source context inspectable before an AI output is trusted or applied, and preserves redaction truth on evidence, degrading honestly when approval is hidden, a source reads as workspace-grounded, or a redacted trail reads as complete",
            "evidence:m5-ai-evidence-review-ui:001",
            vec![
                D::EvidencePointerDriftedSilently,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                card_review_required_clean(),
                card_approval_hidden(),
                card_source_not_disclosed(),
            ],
            vec![evidence_external_clean(), evidence_redaction_hidden()],
        ),
        base_row(
            C::NotebookUi,
            "Notebook AI owner",
            "The notebook reuses the same message and evidence grammar in code cells, discloses stale evidence rather than reading as fresh, and degrades honestly when a message state or an evidence kind / lineage cannot be resolved",
            "evidence:m5-ai-evidence-notebook-ui:001",
            vec![
                D::AiConfidenceUnstated,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![card_stale_evidence_clean(), card_state_unresolved()],
            vec![
                evidence_retrieval_clean(),
                evidence_kind_unresolved(),
                evidence_lineage_unresolved(),
            ],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved card and evidence truth, so an undisclosed source, an over-budget spend read as free, an unstated identity, a missing related resource, or an unresolved disclosure state is visible in evidence rather than hidden behind compact chrome, and redacted / partial trails stay disclosed",
            "evidence:m5-ai-evidence-support-export:001",
            vec![
                D::EvidenceTimelineOpaqueLog,
                D::EvidencePointerDriftedSilently,
                D::GenericChromeWordingUsed,
                D::ProofStale,
            ],
            vec![
                card_blocked_disclosed_clean(),
                card_identity_unstated(),
                card_spend_not_disclosed(),
                card_source_unresolved(),
            ],
            vec![
                evidence_redacted_disclosed_clean(),
                evidence_partial_disclosed_clean(),
                evidence_identity_unstated(),
                evidence_related_missing(),
                evidence_disclosure_unresolved(),
            ],
        ),
        base_row(
            C::ProductUi,
            "In-product AI owner",
            "In-product surfaces reuse the same card and evidence grammar a user sees in the AI panel, always offering the command-backed detail path and safe actions, and degrading honestly when the trace path is missing, the route or spend posture is unresolved or implicit, or an evidence entry lacks a timestamp or replay / export action",
            "evidence:m5-ai-evidence-product-ui:001",
            vec![
                D::GenericChromeWordingUsed,
                D::EvidencePointerDriftedSilently,
                D::ProofStale,
            ],
            vec![
                card_draft_local_clean(),
                card_route_implicit(),
                card_route_unresolved(),
                card_spend_unresolved(),
                card_safe_actions_missing(),
                card_detail_missing(),
            ],
            vec![
                evidence_tool_clean(),
                evidence_timestamp_missing(),
                evidence_replay_missing(),
                evidence_detail_missing(),
            ],
        ),
    ]
}

fn governance_review() -> M5AiEvidenceGovernanceReview {
    M5AiEvidenceGovernanceReview {
        card_names_state_source_confidence_route_and_spend: true,
        message_states_stay_explicit: true,
        approval_state_always_inspectable: true,
        route_locality_and_spend_stay_explicit: true,
        evidence_names_timestamp_kind_lineage_and_resource: true,
        evidence_keeps_inspectable_structure: true,
        redaction_and_partial_truth_preserved: true,
        evidence_pointers_never_silently_drift: true,
        message_and_evidence_grammar_holds_across_surfaces: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5AiEvidenceConsumerProjection {
    M5AiEvidenceConsumerProjection {
        ai_surfaces_consume_message_and_evidence_vocabulary: true,
        editor_and_review_surfaces_consume_shared_vocabulary: true,
        notebook_surfaces_consume_evidence_lineage_vocabulary: true,
        browser_handoff_and_export_preserve_source_and_lineage: true,
        facts_trace_to_single_component_contract: true,
        support_export_reads_single_editor_source: true,
    }
}

fn proof_freshness() -> M5AiEvidenceProofFreshness {
    M5AiEvidenceProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AiEvidenceReleasePosture {
    M5AiEvidenceReleasePosture {
        proof_packet_ref: M5_AI_EVIDENCE_CONTROLS_ARTIFACT_REF.to_owned(),
        component_audit_ref: M5_AI_EVIDENCE_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_AI_EVIDENCE_CONTROLS_SCHEMA_REF,
        M5_AI_EVIDENCE_CONTROLS_DOC_REF,
        M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF,
        M5_EDITOR_INLINE_COMPONENT_DOC_REF,
        M5_AI_MESSAGE_CARD_SCHEMA_REF,
        M5_EVIDENCE_TIMELINE_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 AI-message-card / evidence-timeline controls packet.
pub fn seeded_m5_ai_evidence_controls() -> M5AiEvidenceControlsPacket {
    M5AiEvidenceControlsPacket::new(M5AiEvidenceControlsPacketInput {
        packet_id: M5_AI_EVIDENCE_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 AI-message-card and evidence-timeline controls with source context, confidence / uncertainty class, route / provider locality, spend / cost posture, safe actions, and timestamp / evidence-kind / lineage / redaction truth aligned across editor, review, notebook, AI, support, and product surfaces"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5AiEvidenceVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the AI-UI row is held at Beta pending message-state / evidence parity on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_ai_evidence_controls_ai_ui_beta_narrowed() -> M5AiEvidenceControlsPacket {
    let mut packet = seeded_m5_ai_evidence_controls();
    packet.packet_id = "m5-ai-message-card-evidence-timeline-controls:ai-ui-beta:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EditorInlineConsumerSurface::AiUi)
        .expect("ai-ui row present");
    row.qualification = M5EditorInlineQualificationClass::Beta;
    packet
}

/// Narrowed variant: the support-export row is narrowed to Preview pending redaction / lineage parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_ai_evidence_controls_support_export_preview_narrowed() -> M5AiEvidenceControlsPacket
{
    let mut packet = seeded_m5_ai_evidence_controls();
    packet.packet_id =
        "m5-ai-message-card-evidence-timeline-controls:support-export-preview:0001".to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5EditorInlineConsumerSurface::SupportExport)
        .expect("support-export row present");
    row.qualification = M5EditorInlineQualificationClass::Preview;
    packet
}
