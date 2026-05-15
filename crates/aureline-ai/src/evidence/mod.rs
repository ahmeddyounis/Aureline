//! Evidence packets for the bounded AI-assisted mutation wedge.
//!
//! The evidence lane turns one review-first mutation proposal into a
//! durable, export-safe packet before any apply path can run. It consumes
//! the routing alpha packet from [`crate::routing`], quotes route and spend
//! receipt refs by id, preserves approval lineage, records tainted-context
//! fences, and keeps cited source identity reconstructible for docs-backed
//! answers.
//!
//! This module stores metadata and opaque refs only. It does not carry raw
//! prompt text, raw document bodies, raw diffs, raw file paths, raw URLs,
//! raw provider payloads, exact token counts, exact cost amounts, or
//! credential material.

use aureline_docs::{
    CitationAnchorAlpha, CitationAnchorAvailability as DocsCitationAnchorAvailability,
    CitationSourceClass, DocsFreshnessClass, DocsNodeIdentity,
};
use aureline_history::{
    emit_ai_apply_record, AiApplyLineage, ApprovalRef, MutationJournalEntryRecord,
    MutationProducerEmissionError, MutationProducerInput, PreviewKind, PreviewRef,
};
use serde::{Deserialize, Serialize};

use aureline_graph::GraphFactCuePacket;

use crate::context_inspector::{AiContextEvidenceHandoff, AiContextEvidenceHandoffRow};
use crate::routing::{AiRoutingPacket, RoutingPolicyContext};

/// Stable record-kind tag carried on serialized [`AiMutationEvidencePacket`] payloads.
pub const AI_MUTATION_EVIDENCE_PACKET_RECORD_KIND: &str =
    "ai_mutation_evidence_packet_alpha_record";

/// Stable record-kind tag carried on serialized [`AiMutationEvidenceSupportPacket`] payloads.
pub const AI_MUTATION_EVIDENCE_SUPPORT_PACKET_RECORD_KIND: &str =
    "ai_mutation_evidence_support_packet_alpha_record";

/// Stable record-kind tag carried on serialized [`AiMutationEvidenceReviewRow`] payloads.
pub const AI_MUTATION_EVIDENCE_REVIEW_ROW_RECORD_KIND: &str =
    "ai_mutation_evidence_review_row_alpha_record";

/// Schema version of the alpha evidence packet and support projection.
pub const AI_MUTATION_EVIDENCE_SCHEMA_VERSION: u32 = 1;

/// Lifecycle state of one mutation evidence packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationEvidenceState {
    /// Packet was minted for review before any apply action.
    ReviewPreApply,
    /// The reviewed and approved mutation applied.
    Applied,
    /// The mutation was rejected or blocked and did not apply.
    Rejected,
}

impl MutationEvidenceState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewPreApply => "review_pre_apply",
            Self::Applied => "applied",
            Self::Rejected => "rejected",
        }
    }
}

/// Mutation capability requested by the AI proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationIntentClass {
    /// Reversible local edit reviewed through the patch surface.
    LocalReversibleEdit,
    /// Local destructive edit. The alpha packet records it but validation
    /// requires a review and approval lineage before apply.
    LocalDestructiveEdit,
    /// Background branch-agent dispatch that cannot self-land.
    BranchAgentDispatch,
}

impl MutationIntentClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalReversibleEdit => "local_reversible_edit",
            Self::LocalDestructiveEdit => "local_destructive_edit",
            Self::BranchAgentDispatch => "branch_agent_dispatch",
        }
    }
}

/// Final apply posture recorded by the review lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplyOutcomeClass {
    /// Proposal is waiting for review; no mutation has run.
    NotAppliedPendingReview,
    /// Approved mutation applied successfully.
    AppliedSuccess,
    /// User rejected the proposal; no mutation ran.
    RejectedNoApply,
    /// Policy or tainted-context rules blocked apply.
    BlockedNoApply,
    /// Previously applied mutation was reverted.
    RevertedAfterApply,
}

impl ApplyOutcomeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotAppliedPendingReview => "not_applied_pending_review",
            Self::AppliedSuccess => "applied_success",
            Self::RejectedNoApply => "rejected_no_apply",
            Self::BlockedNoApply => "blocked_no_apply",
            Self::RevertedAfterApply => "reverted_after_apply",
        }
    }
}

/// Validation rollup attached to a mutation review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationOutcomeClass {
    /// No validation has run yet.
    NotRun,
    /// Validation passed.
    Passed,
    /// Validation failed.
    Failed,
    /// Validation was skipped with a recorded reason.
    SkippedWithReason,
}

impl ValidationOutcomeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRun => "not_run",
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::SkippedWithReason => "skipped_with_reason",
        }
    }
}

/// Source class for a cited or referenced evidence source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitedSourceClass {
    /// Workspace file slice.
    WorkspaceFileSlice,
    /// Workspace symbol or graph object.
    WorkspaceSymbol,
    /// Documentation pack excerpt.
    DocsPackExcerpt,
    /// Generated reference documentation.
    GeneratedReference,
    /// Glossary or learning-pack entry.
    GlossaryEntry,
    /// Derived explainer snapshot.
    ExplainerSnapshot,
    /// Search result packet.
    SearchResult,
    /// Diagnostic capture.
    Diagnostic,
    /// Review or patch surface artifact.
    ReviewArtifact,
}

impl CitedSourceClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceFileSlice => "workspace_file_slice",
            Self::WorkspaceSymbol => "workspace_symbol",
            Self::DocsPackExcerpt => "docs_pack_excerpt",
            Self::GeneratedReference => "generated_reference",
            Self::GlossaryEntry => "glossary_entry",
            Self::ExplainerSnapshot => "explainer_snapshot",
            Self::SearchResult => "search_result",
            Self::Diagnostic => "diagnostic",
            Self::ReviewArtifact => "review_artifact",
        }
    }

    /// True when the source must retain docs-pack, document revision, or
    /// hidden/omitted citation truth.
    pub const fn requires_docs_truth(self) -> bool {
        matches!(
            self,
            Self::DocsPackExcerpt
                | Self::GeneratedReference
                | Self::GlossaryEntry
                | Self::ExplainerSnapshot
        )
    }
}

/// Visibility of a citation anchor in the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationVisibilityClass {
    /// Exact citation anchor is available and recorded.
    AnchorAvailable,
    /// Source can be cited, but the exact anchor is unavailable.
    AnchorUnavailableDisclosed,
    /// Citation is hidden by policy and must carry a note.
    HiddenByPolicy,
    /// Citation was omitted and must carry a note.
    OmittedByPolicy,
    /// Source is not citation-bearing.
    NotCitationBearing,
}

impl CitationVisibilityClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AnchorAvailable => "anchor_available",
            Self::AnchorUnavailableDisclosed => "anchor_unavailable_disclosed",
            Self::HiddenByPolicy => "hidden_by_policy",
            Self::OmittedByPolicy => "omitted_by_policy",
            Self::NotCitationBearing => "not_citation_bearing",
        }
    }

    /// True when the packet must explain why an anchor is not visible.
    pub const fn requires_note(self) -> bool {
        matches!(
            self,
            Self::AnchorUnavailableDisclosed | Self::HiddenByPolicy | Self::OmittedByPolicy
        )
    }
}

/// Trust posture of a source reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceSourcePosture {
    /// First-party workspace source.
    TrustedFirstParty,
    /// Trusted authority quote with citation support.
    TrustedAuthority,
    /// Derived source reviewed by the product or user.
    ReviewedDerived,
    /// Derived source not yet reviewed.
    UnreviewedDerived,
    /// Tainted external source that must have a fence.
    TaintedExternal,
}

impl EvidenceSourcePosture {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedFirstParty => "trusted_first_party",
            Self::TrustedAuthority => "trusted_authority",
            Self::ReviewedDerived => "reviewed_derived",
            Self::UnreviewedDerived => "unreviewed_derived",
            Self::TaintedExternal => "tainted_external",
        }
    }
}

/// Freshness class carried by evidence sources and packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshnessClass {
    /// Source was live and authoritative at packet mint time.
    AuthoritativeLive,
    /// Source came from a warm cache with visible freshness.
    WarmCached,
    /// Source came from degraded cached state.
    DegradedCached,
    /// Source was stale but disclosed.
    Stale,
    /// Source freshness was unverified.
    Unverified,
}

impl EvidenceFreshnessClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::DegradedCached => "degraded_cached",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
        }
    }
}

/// Inference posture for derived explanations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InferenceClass {
    /// Statement is a raw cited source claim, not a bridge.
    RawSource,
    /// Statement derives from one cited source.
    DerivedSingleSource,
    /// Statement bridges multiple files, docs, or graph facts.
    DerivedMultipleSources,
    /// Statement is heuristic and must not masquerade as source truth.
    HeuristicBridge,
}

impl InferenceClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawSource => "raw_source",
            Self::DerivedSingleSource => "derived_single_source",
            Self::DerivedMultipleSources => "derived_multiple_sources",
            Self::HeuristicBridge => "heuristic_bridge",
        }
    }
}

/// Confidence label attached to derived explanations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceClass {
    /// Claim is evidence-backed for the cited scope.
    EvidenceBacked,
    /// Claim is explicitly inferred from evidence.
    Inferred,
    /// Claim has low confidence and must not offer direct apply.
    LowConfidence,
    /// Confidence is unavailable or unverified.
    UnknownUnverified,
}

impl ConfidenceClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceBacked => "evidence_backed",
            Self::Inferred => "inferred",
            Self::LowConfidence => "low_confidence",
            Self::UnknownUnverified => "unknown_unverified",
        }
    }
}

/// Source class for tainted context fences.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaintedEvidenceSourceClass {
    /// Retrieved document from an external or provider-backed source.
    RetrievedDocument,
    /// Terminal or command output snippet.
    TerminalSnippet,
    /// Log snippet.
    LogSnippet,
    /// Request or response payload.
    RequestResponsePayload,
    /// User supplied text or dropped content.
    UserSuppliedText,
    /// External tool return value.
    ToolCallReturnValue,
    /// Extension proposed context.
    ExtensionProposedPayload,
    /// Connected provider payload.
    ConnectedProviderPayload,
    /// Prior AI output carried forward.
    AiPriorTurnOutput,
    /// Policy quarantined source.
    PolicyQuarantinedSource,
}

impl TaintedEvidenceSourceClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetrievedDocument => "retrieved_document",
            Self::TerminalSnippet => "terminal_snippet",
            Self::LogSnippet => "log_snippet",
            Self::RequestResponsePayload => "request_response_payload",
            Self::UserSuppliedText => "user_supplied_text",
            Self::ToolCallReturnValue => "tool_call_return_value",
            Self::ExtensionProposedPayload => "extension_proposed_payload",
            Self::ConnectedProviderPayload => "connected_provider_payload",
            Self::AiPriorTurnOutput => "ai_prior_turn_output",
            Self::PolicyQuarantinedSource => "policy_quarantined_source",
        }
    }
}

/// Fence strategy applied to tainted context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaintFenceStrategy {
    /// Instruction-like content is stripped.
    InstructionStripped,
    /// Content is quoted as data only.
    QuotedAsDataOnly,
    /// Content is summarized and the body is not retained.
    SummaryOnlyNoBody,
    /// Only metadata is retained.
    MetadataOnlyNoContent,
    /// Citation reference is retained without body content.
    CitationReferenceOnly,
}

impl TaintFenceStrategy {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstructionStripped => "instruction_stripped",
            Self::QuotedAsDataOnly => "quoted_as_data_only",
            Self::SummaryOnlyNoBody => "summary_only_no_body",
            Self::MetadataOnlyNoContent => "metadata_only_no_content",
            Self::CitationReferenceOnly => "citation_reference_only",
        }
    }
}

/// Usage constraint preserved by a tainted context fence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaintUsageConstraint {
    /// Tainted content cannot gain tool permission.
    MustNotGainToolPermission,
    /// Tainted content cannot escalate workspace or provider scope.
    MustNotEscalateScope,
    /// Tainted content cannot mint citations.
    MustNotMintCitations,
    /// Tainted content cannot override instruction bundles.
    MustNotOverrideInstructionBundle,
    /// Tainted content cannot publish externally.
    MustNotPublishExternally,
    /// Tainted content cannot commit to the repository.
    MustNotCommitToRepo,
    /// Tainted content cannot dispatch a branch agent.
    MustNotDispatchBranchAgent,
    /// Tainted content cannot route to a higher cost tier.
    MustNotRouteToHigherCostTier,
    /// Downstream packets must preserve the fence.
    MustPreserveFenceInDownstreamPacket,
}

impl TaintUsageConstraint {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MustNotGainToolPermission => "must_not_gain_tool_permission",
            Self::MustNotEscalateScope => "must_not_escalate_scope",
            Self::MustNotMintCitations => "must_not_mint_citations",
            Self::MustNotOverrideInstructionBundle => "must_not_override_instruction_bundle",
            Self::MustNotPublishExternally => "must_not_publish_externally",
            Self::MustNotCommitToRepo => "must_not_commit_to_repo",
            Self::MustNotDispatchBranchAgent => "must_not_dispatch_branch_agent",
            Self::MustNotRouteToHigherCostTier => "must_not_route_to_higher_cost_tier",
            Self::MustPreserveFenceInDownstreamPacket => "must_preserve_fence_in_downstream_packet",
        }
    }
}

/// Reason a tainted or policy-disallowed source was fenced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaintFenceReasonClass {
    /// Source is tainted context.
    TaintedContext,
    /// Policy disallowed the source as instruction-bearing content.
    PolicyDisallowedContext,
    /// Secret projection was denied and only metadata can remain.
    SecretProjectionDenied,
    /// Approval renewal is required before any privileged follow-on.
    ApprovalRenewalRequired,
    /// Source could not be classified and must be treated as tainted.
    UnknownUnclassifiedSource,
}

impl TaintFenceReasonClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaintedContext => "tainted_context",
            Self::PolicyDisallowedContext => "policy_disallowed_context",
            Self::SecretProjectionDenied => "secret_projection_denied",
            Self::ApprovalRenewalRequired => "approval_renewal_required",
            Self::UnknownUnclassifiedSource => "unknown_unclassified_source",
        }
    }
}

/// Decision recorded for one approval lineage entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalDecisionClass {
    /// Approval prompt exists and is waiting for a decision.
    PendingUserReview,
    /// User or admin approved the previewed action.
    Approved,
    /// User rejected the action.
    Rejected,
    /// Policy blocked the action.
    BlockedByPolicy,
    /// Previously valid approval was revoked.
    Revoked,
}

impl ApprovalDecisionClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PendingUserReview => "pending_user_review",
            Self::Approved => "approved",
            Self::Rejected => "rejected",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::Revoked => "revoked",
        }
    }
}

/// Actor class that issued or denied approval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalActorClass {
    /// Local user at the review surface.
    LocalUser,
    /// Admin policy gate.
    AdminPolicy,
    /// Platform control authority.
    PlatformControl,
    /// Automated policy evaluation.
    AutomatedPolicyGate,
}

impl ApprovalActorClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalUser => "local_user",
            Self::AdminPolicy => "admin_policy",
            Self::PlatformControl => "platform_control",
            Self::AutomatedPolicyGate => "automated_policy_gate",
        }
    }
}

/// Redaction class attached to an evidence packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceRedactionClass {
    /// Default metadata-only export posture.
    MetadataSafeDefault,
    /// Operator-only restricted packet.
    OperatorOnlyRestricted,
    /// Internal support restricted packet.
    InternalSupportRestricted,
    /// Signing or provenance evidence only.
    SigningEvidenceOnly,
}

impl EvidenceRedactionClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::OperatorOnlyRestricted => "operator_only_restricted",
            Self::InternalSupportRestricted => "internal_support_restricted",
            Self::SigningEvidenceOnly => "signing_evidence_only",
        }
    }
}

/// Cited source identity and citation truth for a packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitedSourceReference {
    /// Stable source-reference id.
    pub source_reference_id: String,
    /// Source class.
    pub source_class: CitedSourceClass,
    /// Opaque target identity. This is never a raw path or URL.
    pub source_identity_ref: String,
    /// Workspace, document, or generated-reference revision ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_revision_ref: Option<String>,
    /// Docs-pack or glossary-pack id when the source came from a pack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_pack_ref: Option<String>,
    /// Docs-pack revision ref when the source came from a versioned pack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_pack_revision_ref: Option<String>,
    /// Exact citation anchor ref when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_anchor_ref: Option<String>,
    /// Canonical docs-node identity when the source came from docs/help knowledge.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_node_identity: Option<DocsNodeIdentity>,
    /// Canonical citation anchor record when the source can preserve one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation_anchor: Option<CitationAnchorAlpha>,
    /// Citation visibility posture.
    pub citation_visibility_class: CitationVisibilityClass,
    /// Note explaining hidden, omitted, or unavailable citation truth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_or_omitted_citation_note: Option<String>,
    /// Source trust posture.
    pub source_posture: EvidenceSourcePosture,
    /// Freshness posture.
    pub freshness_class: EvidenceFreshnessClass,
}

impl CitedSourceReference {
    /// Builds an AI source reference from canonical docs citation records.
    pub fn from_docs_citation(
        source_reference_id: impl Into<String>,
        docs_node_identity: DocsNodeIdentity,
        citation_anchor: CitationAnchorAlpha,
        source_posture: EvidenceSourcePosture,
    ) -> Self {
        let source_class = cited_source_class_from_docs_source(citation_anchor.source_class);
        let citation_visibility_class =
            citation_visibility_from_docs_anchor(citation_anchor.citation_availability);
        Self {
            source_reference_id: source_reference_id.into(),
            source_class,
            source_identity_ref: docs_node_identity.docs_node_id.clone(),
            source_revision_ref: Some(docs_node_identity.version_or_revision_ref.clone()),
            docs_pack_ref: Some(docs_node_identity.source_pack_ref.clone()),
            docs_pack_revision_ref: Some(docs_node_identity.source_pack_revision_ref.clone()),
            exact_anchor_ref: citation_anchor.exact_anchor_ref.clone(),
            docs_node_identity: Some(docs_node_identity),
            citation_anchor: Some(citation_anchor.clone()),
            citation_visibility_class,
            hidden_or_omitted_citation_note: citation_anchor.hidden_or_omitted_note,
            source_posture,
            freshness_class: evidence_freshness_from_docs(citation_anchor.freshness_class),
        }
    }

    /// True when the source row has enough docs-pack or revision truth
    /// to reconstruct the citation context.
    pub fn has_reconstructible_docs_identity(&self) -> bool {
        !self.source_identity_ref.trim().is_empty()
            && (self
                .docs_pack_ref
                .as_deref()
                .is_some_and(|value| !value.trim().is_empty())
                || self
                    .docs_pack_revision_ref
                    .as_deref()
                    .is_some_and(|value| !value.trim().is_empty())
                || self
                    .source_revision_ref
                    .as_deref()
                    .is_some_and(|value| !value.trim().is_empty()))
    }
}

/// Lineage for one derived explanation or summary used by a mutation proposal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedExplanationLineage {
    /// Stable explanation id.
    pub explanation_ref: String,
    /// Source-reference ids this explanation bridged.
    pub basis_source_reference_refs: Vec<String>,
    /// Inference posture.
    pub inference_class: InferenceClass,
    /// Confidence posture.
    pub confidence_class: ConfidenceClass,
    /// Export-safe reason for the confidence label.
    pub confidence_reason_label: String,
}

impl DerivedExplanationLineage {
    /// True when this explanation bridges more than one source.
    pub fn bridges_multiple_sources(&self) -> bool {
        self.basis_source_reference_refs.len() > 1
    }
}

/// Fence record for tainted or policy-disallowed context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaintedContextFence {
    /// Stable fence id.
    pub fence_id: String,
    /// Segment id the fence covers.
    pub segment_id_ref: String,
    /// Source-reference id covered by the fence when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_reference_ref: Option<String>,
    /// Tainted source class.
    pub tainted_source_class: TaintedEvidenceSourceClass,
    /// Source trust posture.
    pub source_posture: EvidenceSourcePosture,
    /// Fence strategy.
    pub fence_strategy: TaintFenceStrategy,
    /// Usage constraints that must survive downstream handoff.
    pub usage_constraints: Vec<TaintUsageConstraint>,
    /// Reason the fence exists.
    pub reason_class: TaintFenceReasonClass,
    /// Export-safe explanation shown on review and support surfaces.
    pub user_visible_explanation_label: String,
}

/// Route and spend lineage preserved on the evidence packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteSpendLineage {
    /// Opaque ref to the routing packet that selected the model path.
    pub routing_packet_ref: String,
    /// Opaque provider-route receipt ref.
    pub route_receipt_ref: String,
    /// Opaque spend receipt ref.
    pub spend_receipt_ref: String,
    /// Selected provider registry ref.
    pub selected_provider_entry_ref: String,
    /// Selected model registry ref.
    pub selected_model_entry_ref: String,
    /// Export-safe provider label.
    pub selected_provider_label: String,
    /// Export-safe model label.
    pub selected_model_label: String,
    /// Execution-locus token.
    pub execution_locus_token: String,
    /// Route-origin token.
    pub route_origin_token: String,
    /// Quota-state token.
    pub quota_state_token: String,
    /// Cost-envelope token.
    pub cost_envelope_token: String,
    /// Cost-visibility token.
    pub cost_visibility_token: String,
    /// Budget owner ref from the selected route.
    pub budget_owner_ref: String,
    /// Route-change lineage refs from the selected routing packet.
    pub route_change_lineage_refs: Vec<String>,
    /// Approval ticket ref that admitted the route override when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originating_route_approval_ticket_ref: Option<String>,
}

impl RouteSpendLineage {
    /// Project route and spend lineage from an [`AiRoutingPacket`].
    pub fn from_routing_packet(
        routing_packet: &AiRoutingPacket,
        route_receipt_ref: impl Into<String>,
        spend_receipt_ref: impl Into<String>,
    ) -> Self {
        let selected = routing_packet.selected_route();
        Self {
            routing_packet_ref: routing_packet.routing_packet_id.clone(),
            route_receipt_ref: route_receipt_ref.into(),
            spend_receipt_ref: spend_receipt_ref.into(),
            selected_provider_entry_ref: selected
                .map(|candidate| candidate.provider_entry_ref.clone())
                .unwrap_or_default(),
            selected_model_entry_ref: selected
                .map(|candidate| candidate.model_entry_ref.clone())
                .unwrap_or_default(),
            selected_provider_label: selected
                .map(|candidate| candidate.provider_label.clone())
                .unwrap_or_default(),
            selected_model_label: selected
                .map(|candidate| candidate.model_label.clone())
                .unwrap_or_default(),
            execution_locus_token: selected
                .map(|candidate| candidate.execution_locus_class.as_str().to_owned())
                .unwrap_or_default(),
            route_origin_token: selected
                .map(|candidate| candidate.route_origin_class.as_str().to_owned())
                .unwrap_or_default(),
            quota_state_token: selected
                .map(|candidate| candidate.quota.quota_state_class.as_str().to_owned())
                .unwrap_or_default(),
            cost_envelope_token: selected
                .map(|candidate| candidate.envelope.cost_envelope_class.as_str().to_owned())
                .unwrap_or_default(),
            cost_visibility_token: selected
                .map(|candidate| candidate.envelope.cost_visibility_class.as_str().to_owned())
                .unwrap_or_default(),
            budget_owner_ref: selected
                .map(|candidate| candidate.quota.budget_owner_ref.clone())
                .unwrap_or_default(),
            route_change_lineage_refs: routing_packet
                .route_change_lineage
                .iter()
                .map(|lineage| lineage.lineage_id.clone())
                .collect(),
            originating_route_approval_ticket_ref: selected
                .and_then(|candidate| candidate.originating_approval_ticket_ref.clone()),
        }
    }
}

/// Approval lineage preserved before and after a mutation decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalLineageEntry {
    /// Stable approval-lineage row id.
    pub approval_lineage_id: String,
    /// Opaque approval ticket ref.
    pub approval_ticket_ref: String,
    /// Approval decision.
    pub decision_class: ApprovalDecisionClass,
    /// Actor class that made or blocked the decision.
    pub actor_class: ApprovalActorClass,
    /// Review preview shown before approval.
    pub preview_ref: String,
    /// Policy epoch the decision resolved against.
    pub policy_epoch_ref: String,
    /// Decision timestamp or pending timestamp.
    pub decided_at: String,
    /// Export-safe summary label.
    pub summary_label: String,
}

/// Review and mutation lineage for the proposed patch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationReviewLineage {
    /// Review surface or sheet ref shown before apply.
    pub review_surface_ref: String,
    /// Patch review summary ref.
    pub patch_review_summary_ref: String,
    /// Produced artifact refs such as patch artifacts or generated reports.
    pub produced_artifact_refs: Vec<String>,
    /// Number of files touched by the proposed mutation.
    pub changed_file_count: u32,
    /// Number of generated artifacts touched by the proposal.
    pub generated_artifact_count: u32,
    /// Validation summary refs.
    pub validation_summary_refs: Vec<String>,
    /// Validation rollup.
    pub validation_outcome_class: ValidationOutcomeClass,
    /// Undo checkpoint ref when apply succeeded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint_ref: Option<String>,
    /// Mutation journal ref when apply succeeded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation_journal_ref: Option<String>,
    /// Apply outcome.
    pub apply_outcome_class: ApplyOutcomeClass,
}

/// Canonical evidence packet for the alpha AI-assisted mutation wedge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiMutationEvidencePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable evidence packet id.
    pub evidence_packet_id: String,
    /// Stable mutation wedge ref.
    pub mutation_wedge_ref: String,
    /// Composer session ref.
    pub composer_session_ref: String,
    /// Turn draft ref.
    pub turn_draft_ref: String,
    /// Request workspace ref.
    pub request_workspace_ref: String,
    /// Context assembly ref.
    pub assembly_ref: String,
    /// Packet lifecycle state.
    pub packet_state: MutationEvidenceState,
    /// Mutation intent class.
    pub intent_class: MutationIntentClass,
    /// Route and spend lineage.
    pub route_spend_lineage: RouteSpendLineage,
    /// Approval lineage rows.
    pub approval_lineage: Vec<ApprovalLineageEntry>,
    /// Cited source references.
    pub cited_sources: Vec<CitedSourceReference>,
    /// Derived explanation lineage rows.
    pub derived_explanations: Vec<DerivedExplanationLineage>,
    /// Tainted-context fences.
    pub tainted_context_fences: Vec<TaintedContextFence>,
    /// Context handoff rows consumed from the pre-send composer/context inspector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_handoff: Option<AiContextEvidenceHandoff>,
    /// Review and mutation lineage.
    pub review_lineage: MutationReviewLineage,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Policy context.
    pub policy_context: RoutingPolicyContext,
    /// Running build identity ref.
    pub running_build_identity_ref: String,
    /// Redaction class.
    pub redaction_class: EvidenceRedactionClass,
    /// Packet mint timestamp.
    pub minted_at: String,
    /// Terminal timestamp for applied or rejected packets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

impl AiMutationEvidencePacket {
    /// Builds a packet with the stable record kind and schema version.
    pub fn new(input: AiMutationEvidencePacketInput) -> Self {
        Self {
            record_kind: AI_MUTATION_EVIDENCE_PACKET_RECORD_KIND.to_owned(),
            schema_version: AI_MUTATION_EVIDENCE_SCHEMA_VERSION,
            evidence_packet_id: input.evidence_packet_id,
            mutation_wedge_ref: input.mutation_wedge_ref,
            composer_session_ref: input.composer_session_ref,
            turn_draft_ref: input.turn_draft_ref,
            request_workspace_ref: input.request_workspace_ref,
            assembly_ref: input.assembly_ref,
            packet_state: input.packet_state,
            intent_class: input.intent_class,
            route_spend_lineage: input.route_spend_lineage,
            approval_lineage: input.approval_lineage,
            cited_sources: input.cited_sources,
            derived_explanations: input.derived_explanations,
            tainted_context_fences: input.tainted_context_fences,
            context_handoff: input.context_handoff,
            review_lineage: input.review_lineage,
            source_contract_refs: input.source_contract_refs,
            policy_context: input.policy_context,
            running_build_identity_ref: input.running_build_identity_ref,
            redaction_class: input.redaction_class,
            minted_at: input.minted_at,
            completed_at: input.completed_at,
        }
    }

    /// Project compact review rows for shell, CLI, or support surfaces.
    pub fn review_rows(&self) -> Vec<AiMutationEvidenceReviewRow> {
        let approval_state = self
            .approval_lineage
            .last()
            .map(|lineage| lineage.decision_class.as_str())
            .unwrap_or("missing");
        vec![
            AiMutationEvidenceReviewRow::new(
                "packet_state",
                "Evidence state",
                self.packet_state.as_str(),
                self.packet_state.as_str(),
            ),
            AiMutationEvidenceReviewRow::new(
                "provider_model",
                "Provider and model",
                &format!(
                    "{} / {}",
                    self.route_spend_lineage.selected_provider_label,
                    self.route_spend_lineage.selected_model_label
                ),
                &self.route_spend_lineage.selected_model_entry_ref,
            ),
            AiMutationEvidenceReviewRow::new(
                "route_spend",
                "Route and spend",
                &format!(
                    "{} / {} / {}",
                    self.route_spend_lineage.route_origin_token,
                    self.route_spend_lineage.cost_envelope_token,
                    self.route_spend_lineage.cost_visibility_token
                ),
                &self.route_spend_lineage.spend_receipt_ref,
            ),
            AiMutationEvidenceReviewRow::new(
                "approval",
                "Approval",
                approval_state,
                approval_state,
            ),
            AiMutationEvidenceReviewRow::new(
                "citations",
                "Citations",
                &format!(
                    "{} cited source(s), {} derived explanation(s)",
                    self.cited_sources.len(),
                    self.derived_explanations.len()
                ),
                "citation_summary",
            ),
            AiMutationEvidenceReviewRow::new(
                "tainted_fences",
                "Tainted fences",
                &format!("{} fence(s)", self.tainted_context_fences.len()),
                "tainted_context_fences",
            ),
            AiMutationEvidenceReviewRow::new(
                "apply_outcome",
                "Apply outcome",
                self.review_lineage.apply_outcome_class.as_str(),
                self.review_lineage.apply_outcome_class.as_str(),
            ),
        ]
    }

    /// Project an export-safe support packet from the evidence packet.
    pub fn support_packet(&self) -> AiMutationEvidenceSupportPacket {
        AiMutationEvidenceSupportPacket {
            record_kind: AI_MUTATION_EVIDENCE_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: AI_MUTATION_EVIDENCE_SCHEMA_VERSION,
            support_packet_id: format!("support-export:ai-mutation:{}", self.evidence_packet_id),
            evidence_packet_ref: self.evidence_packet_id.clone(),
            mutation_wedge_ref: self.mutation_wedge_ref.clone(),
            packet_state_token: self.packet_state.as_str().to_owned(),
            route_receipt_ref: self.route_spend_lineage.route_receipt_ref.clone(),
            spend_receipt_ref: self.route_spend_lineage.spend_receipt_ref.clone(),
            routing_packet_ref: self.route_spend_lineage.routing_packet_ref.clone(),
            selected_provider_entry_ref: self
                .route_spend_lineage
                .selected_provider_entry_ref
                .clone(),
            selected_model_entry_ref: self.route_spend_lineage.selected_model_entry_ref.clone(),
            selected_provider_label: self.route_spend_lineage.selected_provider_label.clone(),
            selected_model_label: self.route_spend_lineage.selected_model_label.clone(),
            route_origin_token: self.route_spend_lineage.route_origin_token.clone(),
            cost_envelope_token: self.route_spend_lineage.cost_envelope_token.clone(),
            cost_visibility_token: self.route_spend_lineage.cost_visibility_token.clone(),
            approval_ticket_refs: self
                .approval_lineage
                .iter()
                .map(|lineage| lineage.approval_ticket_ref.clone())
                .collect(),
            approval_decision_tokens: self
                .approval_lineage
                .iter()
                .map(|lineage| lineage.decision_class.as_str().to_owned())
                .collect(),
            citation_rows: self
                .cited_sources
                .iter()
                .map(AiMutationEvidenceCitationSupportRow::from_source)
                .collect(),
            derived_explanation_rows: self
                .derived_explanations
                .iter()
                .map(AiMutationEvidenceDerivedSupportRow::from_lineage)
                .collect(),
            tainted_fence_rows: self
                .tainted_context_fences
                .iter()
                .map(AiMutationEvidenceFenceSupportRow::from_fence)
                .collect(),
            context_handoff_rows: self
                .context_handoff
                .as_ref()
                .map(|handoff| handoff.context_rows.clone())
                .unwrap_or_default(),
            graph_cue_packets: self
                .context_handoff
                .as_ref()
                .map(|handoff| handoff.graph_cue_packets.clone())
                .unwrap_or_default(),
            review_surface_ref: self.review_lineage.review_surface_ref.clone(),
            patch_review_summary_ref: self.review_lineage.patch_review_summary_ref.clone(),
            validation_summary_refs: self.review_lineage.validation_summary_refs.clone(),
            apply_outcome_token: self.review_lineage.apply_outcome_class.as_str().to_owned(),
            rollback_checkpoint_ref: self.review_lineage.rollback_checkpoint_ref.clone(),
            mutation_journal_ref: self.review_lineage.mutation_journal_ref.clone(),
            validation_violation_tokens: self
                .validate()
                .into_iter()
                .map(|violation| violation.as_str().to_owned())
                .collect(),
            source_contract_refs: self.source_contract_refs.clone(),
            running_build_identity_ref: self.running_build_identity_ref.clone(),
            redaction_class_token: self.redaction_class.as_str().to_owned(),
            minted_at: self.minted_at.clone(),
            completed_at: self.completed_at.clone(),
        }
    }

    /// Deterministic export-safe JSON for support bundles.
    ///
    /// # Panics
    ///
    /// Panics only if serializing the metadata-only support packet fails.
    pub fn export_safe_support_json(&self) -> String {
        self.support_packet().export_safe_json()
    }

    /// Deterministic Markdown summary for review and support handoff.
    pub fn render_markdown_summary(&self) -> String {
        let support = self.support_packet();
        let mut out = String::new();
        out.push_str("# AI Mutation Evidence Packet\n\n");
        out.push_str(&format!(
            "- Evidence packet: `{}`\n",
            self.evidence_packet_id
        ));
        out.push_str(&format!("- State: `{}`\n", self.packet_state.as_str()));
        out.push_str(&format!(
            "- Route/spend: `{}` / `{}`\n",
            support.route_receipt_ref, support.spend_receipt_ref
        ));
        out.push_str(&format!(
            "- Provider/model: {} / {}\n",
            support.selected_provider_label, support.selected_model_label
        ));
        out.push_str(&format!(
            "- Approval decisions: `{}`\n",
            support.approval_decision_tokens.join(",")
        ));
        out.push_str(&format!(
            "- Citations: {} source row(s), {} derived explanation row(s)\n",
            support.citation_rows.len(),
            support.derived_explanation_rows.len()
        ));
        out.push_str(&format!(
            "- Tainted fences: {} row(s)\n",
            support.tainted_fence_rows.len()
        ));
        out.push_str(&format!(
            "- Graph cues: {} packet(s)\n",
            support.graph_cue_packets.len()
        ));
        out.push_str(&format!(
            "- Apply outcome: `{}`\n",
            support.apply_outcome_token
        ));
        if !support.validation_violation_tokens.is_empty() {
            out.push_str("\n## Validation\n\n");
            for token in &support.validation_violation_tokens {
                out.push_str(&format!("- `{token}`\n"));
            }
        }
        out
    }

    /// Projects the evidence packet into metadata-only AI apply journal lineage.
    pub fn ai_apply_lineage(&self) -> AiApplyLineage {
        AiApplyLineage::new(
            self.evidence_packet_id.clone(),
            self.route_spend_lineage.route_origin_token.clone(),
            self.route_spend_lineage.spend_receipt_ref.clone(),
            self.tainted_context_fences
                .first()
                .map(|fence| fence.fence_id.clone()),
        )
    }

    /// Emits an AI apply mutation-journal entry with this packet's lineage attached.
    ///
    /// The caller supplies the mutation target, checkpoint, timestamp, and diff
    /// identity metadata. This method fills the evidence, route, spend, preview,
    /// and approval refs that are owned by the AI evidence packet.
    ///
    /// # Errors
    ///
    /// Returns [`MutationProducerEmissionError`] when required journal metadata
    /// is missing, including an empty evidence-packet ref.
    pub fn emit_ai_apply_mutation_journal_record(
        &self,
        mut input: MutationProducerInput,
    ) -> Result<MutationJournalEntryRecord, MutationProducerEmissionError> {
        if input.preview_ref.is_none() {
            input.preview_ref = Some(PreviewRef {
                preview_id: self.review_lineage.review_surface_ref.clone(),
                preview_kind: Some(PreviewKind::AiPatchPreview),
            });
        }
        if input.approval_ref.is_none() {
            if let Some(approval) = self.approval_lineage.last() {
                input.approval_ref = Some(ApprovalRef {
                    approval_id: approval.approval_ticket_ref.clone(),
                    approval_policy: Some("ai.evidence.required".to_owned()),
                });
            }
        }
        input.ai_apply_lineage = Some(self.ai_apply_lineage());
        emit_ai_apply_record(input)
    }

    /// Validate the packet against the bounded mutation-evidence invariants.
    pub fn validate(&self) -> Vec<AiMutationEvidenceViolation> {
        let mut violations = Vec::new();
        if self.record_kind != AI_MUTATION_EVIDENCE_PACKET_RECORD_KIND {
            violations.push(AiMutationEvidenceViolation::WrongRecordKind);
        }
        if self.schema_version != AI_MUTATION_EVIDENCE_SCHEMA_VERSION {
            violations.push(AiMutationEvidenceViolation::WrongSchemaVersion);
        }
        if self.evidence_packet_id.trim().is_empty() {
            violations.push(AiMutationEvidenceViolation::MissingEvidencePacketId);
        }
        if self.mutation_wedge_ref.trim().is_empty() {
            violations.push(AiMutationEvidenceViolation::MissingMutationWedgeRef);
        }
        if self.composer_session_ref.trim().is_empty() {
            violations.push(AiMutationEvidenceViolation::MissingComposerSessionRef);
        }
        if self.turn_draft_ref.trim().is_empty() {
            violations.push(AiMutationEvidenceViolation::MissingTurnDraftRef);
        }
        if self.request_workspace_ref.trim().is_empty() {
            violations.push(AiMutationEvidenceViolation::MissingRequestWorkspaceRef);
        }
        if self.assembly_ref.trim().is_empty() {
            violations.push(AiMutationEvidenceViolation::MissingAssemblyRef);
        }
        if self.running_build_identity_ref.trim().is_empty() {
            violations.push(AiMutationEvidenceViolation::MissingRunningBuildIdentityRef);
        }
        if self.source_contract_refs.is_empty() {
            violations.push(AiMutationEvidenceViolation::MissingSourceContractRefs);
        }
        if self.route_spend_lineage.route_receipt_ref.trim().is_empty()
            || self
                .route_spend_lineage
                .routing_packet_ref
                .trim()
                .is_empty()
        {
            violations.push(AiMutationEvidenceViolation::MissingRouteLineage);
        }
        if self.route_spend_lineage.spend_receipt_ref.trim().is_empty() {
            violations.push(AiMutationEvidenceViolation::MissingSpendLineage);
        }
        if self.approval_lineage.is_empty() {
            violations.push(AiMutationEvidenceViolation::MissingApprovalLineage);
        }
        for approval in &self.approval_lineage {
            if approval.approval_ticket_ref.trim().is_empty()
                || approval.preview_ref.trim().is_empty()
                || approval.policy_epoch_ref.trim().is_empty()
            {
                violations.push(AiMutationEvidenceViolation::MissingApprovalLineage);
                break;
            }
        }
        if self.review_lineage.review_surface_ref.trim().is_empty()
            || self
                .review_lineage
                .patch_review_summary_ref
                .trim()
                .is_empty()
        {
            violations.push(AiMutationEvidenceViolation::MissingPreApplyReviewSurface);
        }

        match self.packet_state {
            MutationEvidenceState::ReviewPreApply => {
                if self.review_lineage.apply_outcome_class
                    != ApplyOutcomeClass::NotAppliedPendingReview
                {
                    violations.push(AiMutationEvidenceViolation::PreApplyPacketClaimsApply);
                }
            }
            MutationEvidenceState::Applied => {
                if self.review_lineage.apply_outcome_class != ApplyOutcomeClass::AppliedSuccess {
                    violations.push(AiMutationEvidenceViolation::AppliedPacketMissingApplyLineage);
                }
                if self
                    .review_lineage
                    .rollback_checkpoint_ref
                    .as_deref()
                    .map_or(true, |value| value.trim().is_empty())
                    || self
                        .review_lineage
                        .mutation_journal_ref
                        .as_deref()
                        .map_or(true, |value| value.trim().is_empty())
                {
                    violations.push(AiMutationEvidenceViolation::AppliedPacketMissingApplyLineage);
                }
                let has_approval = self
                    .approval_lineage
                    .iter()
                    .any(|lineage| lineage.decision_class == ApprovalDecisionClass::Approved);
                if !has_approval {
                    violations.push(AiMutationEvidenceViolation::AppliedPacketMissingApproval);
                }
            }
            MutationEvidenceState::Rejected => {
                let outcome_blocks = matches!(
                    self.review_lineage.apply_outcome_class,
                    ApplyOutcomeClass::RejectedNoApply | ApplyOutcomeClass::BlockedNoApply
                );
                let approval_blocks = self.approval_lineage.iter().any(|lineage| {
                    matches!(
                        lineage.decision_class,
                        ApprovalDecisionClass::Rejected
                            | ApprovalDecisionClass::BlockedByPolicy
                            | ApprovalDecisionClass::Revoked
                    )
                });
                if !outcome_blocks || !approval_blocks {
                    violations
                        .push(AiMutationEvidenceViolation::RejectedPacketMissingRejectionLineage);
                }
            }
        }

        for source in &self.cited_sources {
            if source.source_identity_ref.trim().is_empty() {
                violations.push(AiMutationEvidenceViolation::CitationSourceIdentityMissing);
            }
            if source.source_class.requires_docs_truth() {
                if !source.has_reconstructible_docs_identity() {
                    violations.push(AiMutationEvidenceViolation::DocsCitationRevisionMissing);
                }
                if source.citation_visibility_class == CitationVisibilityClass::AnchorAvailable
                    && source
                        .exact_anchor_ref
                        .as_deref()
                        .map_or(true, |value| value.trim().is_empty())
                {
                    violations.push(AiMutationEvidenceViolation::CitationAnchorMissing);
                }
            }
            if source.citation_visibility_class.requires_note()
                && source
                    .hidden_or_omitted_citation_note
                    .as_deref()
                    .map_or(true, |value| value.trim().is_empty())
            {
                violations.push(AiMutationEvidenceViolation::HiddenOrOmittedCitationNoteMissing);
            }
            if source
                .docs_node_identity
                .as_ref()
                .is_some_and(|identity| !identity.validate().is_empty())
            {
                violations.push(AiMutationEvidenceViolation::DocsCitationRecordInvalid);
            }
            if source
                .citation_anchor
                .as_ref()
                .is_some_and(|anchor| !anchor.validate().is_empty())
            {
                violations.push(AiMutationEvidenceViolation::DocsCitationRecordInvalid);
            }
            if let (Some(identity), Some(anchor)) =
                (&source.docs_node_identity, &source.citation_anchor)
            {
                if identity.docs_node_id != anchor.docs_node_ref
                    || identity.source_pack_revision_ref != anchor.source_pack_revision_ref
                    || source.source_identity_ref != identity.docs_node_id
                {
                    violations.push(AiMutationEvidenceViolation::DocsCitationRecordMismatch);
                }
            }
            if source.source_posture == EvidenceSourcePosture::TaintedExternal {
                let has_fence = self.tainted_context_fences.iter().any(|fence| {
                    fence.source_reference_ref.as_deref()
                        == Some(source.source_reference_id.as_str())
                });
                if !has_fence {
                    violations.push(AiMutationEvidenceViolation::TaintedSourceMissingFence);
                }
            }
        }

        for explanation in &self.derived_explanations {
            if explanation.explanation_ref.trim().is_empty()
                || explanation.basis_source_reference_refs.is_empty()
                || explanation.confidence_reason_label.trim().is_empty()
            {
                violations.push(AiMutationEvidenceViolation::DerivedExplanationMissingConfidence);
            }
            if explanation.bridges_multiple_sources()
                && matches!(explanation.inference_class, InferenceClass::RawSource)
            {
                violations.push(AiMutationEvidenceViolation::DerivedExplanationMissingInference);
            }
            if explanation.bridges_multiple_sources()
                && matches!(
                    explanation.confidence_class,
                    ConfidenceClass::UnknownUnverified
                )
            {
                violations.push(AiMutationEvidenceViolation::DerivedExplanationMissingConfidence);
            }
        }

        for fence in &self.tainted_context_fences {
            if fence.fence_id.trim().is_empty()
                || fence.segment_id_ref.trim().is_empty()
                || fence.usage_constraints.is_empty()
                || fence.user_visible_explanation_label.trim().is_empty()
            {
                violations.push(AiMutationEvidenceViolation::TaintedFenceMissingExplanation);
            }
            if matches!(
                fence.reason_class,
                TaintFenceReasonClass::PolicyDisallowedContext
                    | TaintFenceReasonClass::SecretProjectionDenied
                    | TaintFenceReasonClass::ApprovalRenewalRequired
            ) && fence.user_visible_explanation_label.trim().is_empty()
            {
                violations.push(AiMutationEvidenceViolation::PolicyDisallowedContextUnexplained);
            }
        }

        if let Some(handoff) = &self.context_handoff {
            if !handoff.validate().is_empty() {
                violations.push(AiMutationEvidenceViolation::ContextHandoffInvalid);
            }
            if handoff.composer_session_ref != self.composer_session_ref
                || handoff.turn_draft_ref != self.turn_draft_ref
                || handoff.request_workspace_ref != self.request_workspace_ref
            {
                violations.push(AiMutationEvidenceViolation::ContextHandoffIdentityMismatch);
            }
        }

        if packet_contains_forbidden_boundary_material(self) {
            violations.push(AiMutationEvidenceViolation::RawBoundaryMaterialInExport);
        }

        violations
    }
}

/// Constructor input for [`AiMutationEvidencePacket`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiMutationEvidencePacketInput {
    /// Stable evidence packet id.
    pub evidence_packet_id: String,
    /// Stable mutation wedge ref.
    pub mutation_wedge_ref: String,
    /// Composer session ref.
    pub composer_session_ref: String,
    /// Turn draft ref.
    pub turn_draft_ref: String,
    /// Request workspace ref.
    pub request_workspace_ref: String,
    /// Context assembly ref.
    pub assembly_ref: String,
    /// Packet lifecycle state.
    pub packet_state: MutationEvidenceState,
    /// Mutation intent class.
    pub intent_class: MutationIntentClass,
    /// Route and spend lineage.
    pub route_spend_lineage: RouteSpendLineage,
    /// Approval lineage.
    pub approval_lineage: Vec<ApprovalLineageEntry>,
    /// Cited source refs.
    pub cited_sources: Vec<CitedSourceReference>,
    /// Derived explanation refs.
    pub derived_explanations: Vec<DerivedExplanationLineage>,
    /// Tainted context fences.
    pub tainted_context_fences: Vec<TaintedContextFence>,
    /// Context handoff from the pre-send composer/context inspector.
    pub context_handoff: Option<AiContextEvidenceHandoff>,
    /// Review lineage.
    pub review_lineage: MutationReviewLineage,
    /// Source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Policy context.
    pub policy_context: RoutingPolicyContext,
    /// Build identity ref.
    pub running_build_identity_ref: String,
    /// Redaction class.
    pub redaction_class: EvidenceRedactionClass,
    /// Mint timestamp.
    pub minted_at: String,
    /// Terminal timestamp.
    pub completed_at: Option<String>,
}

/// One compact review row projected from an evidence packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiMutationEvidenceReviewRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Export-safe display label.
    pub label: String,
    /// Export-safe display value.
    pub value_label: String,
    /// Stable token or opaque ref backing the value.
    pub value_token: String,
}

impl AiMutationEvidenceReviewRow {
    fn new(row_id: &str, label: &str, value_label: &str, value_token: &str) -> Self {
        Self {
            record_kind: AI_MUTATION_EVIDENCE_REVIEW_ROW_RECORD_KIND.to_owned(),
            schema_version: AI_MUTATION_EVIDENCE_SCHEMA_VERSION,
            row_id: row_id.to_owned(),
            label: label.to_owned(),
            value_label: value_label.to_owned(),
            value_token: value_token.to_owned(),
        }
    }
}

/// Export-safe support projection for one mutation evidence packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiMutationEvidenceSupportPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable support packet id.
    pub support_packet_id: String,
    /// Source evidence packet ref.
    pub evidence_packet_ref: String,
    /// Mutation wedge ref.
    pub mutation_wedge_ref: String,
    /// Packet-state token.
    pub packet_state_token: String,
    /// Route receipt ref.
    pub route_receipt_ref: String,
    /// Spend receipt ref.
    pub spend_receipt_ref: String,
    /// Routing packet ref.
    pub routing_packet_ref: String,
    /// Selected provider ref.
    pub selected_provider_entry_ref: String,
    /// Selected model ref.
    pub selected_model_entry_ref: String,
    /// Selected provider label.
    pub selected_provider_label: String,
    /// Selected model label.
    pub selected_model_label: String,
    /// Route-origin token.
    pub route_origin_token: String,
    /// Cost-envelope token.
    pub cost_envelope_token: String,
    /// Cost-visibility token.
    pub cost_visibility_token: String,
    /// Approval ticket refs.
    pub approval_ticket_refs: Vec<String>,
    /// Approval decision tokens.
    pub approval_decision_tokens: Vec<String>,
    /// Citation support rows.
    pub citation_rows: Vec<AiMutationEvidenceCitationSupportRow>,
    /// Derived explanation rows.
    pub derived_explanation_rows: Vec<AiMutationEvidenceDerivedSupportRow>,
    /// Tainted fence rows.
    pub tainted_fence_rows: Vec<AiMutationEvidenceFenceSupportRow>,
    /// Context handoff rows consumed from the composer/context inspector.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context_handoff_rows: Vec<AiContextEvidenceHandoffRow>,
    /// Graph cue packets inherited from the context handoff.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub graph_cue_packets: Vec<GraphFactCuePacket>,
    /// Review surface ref.
    pub review_surface_ref: String,
    /// Patch review summary ref.
    pub patch_review_summary_ref: String,
    /// Validation summary refs.
    pub validation_summary_refs: Vec<String>,
    /// Apply outcome token.
    pub apply_outcome_token: String,
    /// Rollback checkpoint ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint_ref: Option<String>,
    /// Mutation journal ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation_journal_ref: Option<String>,
    /// Validation violations on the source packet.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validation_violation_tokens: Vec<String>,
    /// Source contracts consumed by the packet.
    pub source_contract_refs: Vec<String>,
    /// Running build identity ref.
    pub running_build_identity_ref: String,
    /// Redaction-class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
    /// Terminal timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

impl AiMutationEvidenceSupportPacket {
    /// Deterministic export-safe JSON. The projection carries metadata only.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only support packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("mutation evidence support packet serializes")
    }
}

/// Citation row inside [`AiMutationEvidenceSupportPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiMutationEvidenceCitationSupportRow {
    /// Source-reference id.
    pub source_reference_id: String,
    /// Source-class token.
    pub source_class_token: String,
    /// Source identity ref.
    pub source_identity_ref: String,
    /// Source revision ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_revision_ref: Option<String>,
    /// Docs-pack ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_pack_ref: Option<String>,
    /// Docs-pack revision ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_pack_revision_ref: Option<String>,
    /// Exact anchor ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_anchor_ref: Option<String>,
    /// Canonical docs-node identity preserved for support reconstruction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_node_identity: Option<DocsNodeIdentity>,
    /// Canonical citation anchor preserved for support reconstruction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation_anchor: Option<CitationAnchorAlpha>,
    /// Citation visibility token.
    pub citation_visibility_token: String,
    /// Hidden or omitted citation note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_or_omitted_citation_note: Option<String>,
    /// Source posture token.
    pub source_posture_token: String,
    /// Freshness token.
    pub freshness_token: String,
}

impl AiMutationEvidenceCitationSupportRow {
    fn from_source(source: &CitedSourceReference) -> Self {
        Self {
            source_reference_id: source.source_reference_id.clone(),
            source_class_token: source.source_class.as_str().to_owned(),
            source_identity_ref: source.source_identity_ref.clone(),
            source_revision_ref: source.source_revision_ref.clone(),
            docs_pack_ref: source.docs_pack_ref.clone(),
            docs_pack_revision_ref: source.docs_pack_revision_ref.clone(),
            exact_anchor_ref: source.exact_anchor_ref.clone(),
            docs_node_identity: source.docs_node_identity.clone(),
            citation_anchor: source.citation_anchor.clone(),
            citation_visibility_token: source.citation_visibility_class.as_str().to_owned(),
            hidden_or_omitted_citation_note: source.hidden_or_omitted_citation_note.clone(),
            source_posture_token: source.source_posture.as_str().to_owned(),
            freshness_token: source.freshness_class.as_str().to_owned(),
        }
    }
}

/// Derived explanation row inside [`AiMutationEvidenceSupportPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiMutationEvidenceDerivedSupportRow {
    /// Explanation ref.
    pub explanation_ref: String,
    /// Basis source-reference ids.
    pub basis_source_reference_refs: Vec<String>,
    /// Inference token.
    pub inference_token: String,
    /// Confidence token.
    pub confidence_token: String,
    /// Confidence reason.
    pub confidence_reason_label: String,
}

impl AiMutationEvidenceDerivedSupportRow {
    fn from_lineage(lineage: &DerivedExplanationLineage) -> Self {
        Self {
            explanation_ref: lineage.explanation_ref.clone(),
            basis_source_reference_refs: lineage.basis_source_reference_refs.clone(),
            inference_token: lineage.inference_class.as_str().to_owned(),
            confidence_token: lineage.confidence_class.as_str().to_owned(),
            confidence_reason_label: lineage.confidence_reason_label.clone(),
        }
    }
}

/// Tainted fence row inside [`AiMutationEvidenceSupportPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiMutationEvidenceFenceSupportRow {
    /// Fence id.
    pub fence_id: String,
    /// Segment id.
    pub segment_id_ref: String,
    /// Source-reference id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_reference_ref: Option<String>,
    /// Tainted-source token.
    pub tainted_source_token: String,
    /// Fence strategy token.
    pub fence_strategy_token: String,
    /// Usage constraint tokens.
    pub usage_constraint_tokens: Vec<String>,
    /// Reason token.
    pub reason_token: String,
    /// Export-safe explanation.
    pub user_visible_explanation_label: String,
}

impl AiMutationEvidenceFenceSupportRow {
    fn from_fence(fence: &TaintedContextFence) -> Self {
        Self {
            fence_id: fence.fence_id.clone(),
            segment_id_ref: fence.segment_id_ref.clone(),
            source_reference_ref: fence.source_reference_ref.clone(),
            tainted_source_token: fence.tainted_source_class.as_str().to_owned(),
            fence_strategy_token: fence.fence_strategy.as_str().to_owned(),
            usage_constraint_tokens: fence
                .usage_constraints
                .iter()
                .map(|constraint| constraint.as_str().to_owned())
                .collect(),
            reason_token: fence.reason_class.as_str().to_owned(),
            user_visible_explanation_label: fence.user_visible_explanation_label.clone(),
        }
    }
}

/// Validation failures emitted by [`AiMutationEvidencePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AiMutationEvidenceViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Evidence packet id is missing.
    MissingEvidencePacketId,
    /// Mutation wedge ref is missing.
    MissingMutationWedgeRef,
    /// Composer session ref is missing.
    MissingComposerSessionRef,
    /// Turn draft ref is missing.
    MissingTurnDraftRef,
    /// Request workspace ref is missing.
    MissingRequestWorkspaceRef,
    /// Context assembly ref is missing.
    MissingAssemblyRef,
    /// Route lineage is missing.
    MissingRouteLineage,
    /// Spend lineage is missing.
    MissingSpendLineage,
    /// Approval lineage is missing or incomplete.
    MissingApprovalLineage,
    /// Review surface was not recorded before apply.
    MissingPreApplyReviewSurface,
    /// Pre-apply packet claims an apply outcome.
    PreApplyPacketClaimsApply,
    /// Applied packet lacks approval.
    AppliedPacketMissingApproval,
    /// Applied packet lacks checkpoint or mutation-journal lineage.
    AppliedPacketMissingApplyLineage,
    /// Rejected packet lacks rejection or block lineage.
    RejectedPacketMissingRejectionLineage,
    /// Cited source identity is missing.
    CitationSourceIdentityMissing,
    /// Docs-backed source lacks docs-pack or revision truth.
    DocsCitationRevisionMissing,
    /// Source claimed an available anchor but did not record it.
    CitationAnchorMissing,
    /// Hidden or omitted citation lacks an explanation note.
    HiddenOrOmittedCitationNoteMissing,
    /// Tainted source lacks a matching fence.
    TaintedSourceMissingFence,
    /// Tainted fence lacks constraints or visible explanation.
    TaintedFenceMissingExplanation,
    /// Policy-disallowed context lacks an explanation.
    PolicyDisallowedContextUnexplained,
    /// Derived explanation bridging sources lacks inference marking.
    DerivedExplanationMissingInference,
    /// Derived explanation lacks confidence marking.
    DerivedExplanationMissingConfidence,
    /// Running build identity ref is missing.
    MissingRunningBuildIdentityRef,
    /// Source contract refs are missing.
    MissingSourceContractRefs,
    /// Packet contains raw boundary material forbidden in exports.
    RawBoundaryMaterialInExport,
    /// Context handoff rows failed their own validation.
    ContextHandoffInvalid,
    /// Context handoff points at a different composer session, draft, or workspace.
    ContextHandoffIdentityMismatch,
    /// Canonical docs citation records failed their own validation.
    DocsCitationRecordInvalid,
    /// Canonical docs citation records disagree with the AI source row identity.
    DocsCitationRecordMismatch,
}

impl AiMutationEvidenceViolation {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingEvidencePacketId => "missing_evidence_packet_id",
            Self::MissingMutationWedgeRef => "missing_mutation_wedge_ref",
            Self::MissingComposerSessionRef => "missing_composer_session_ref",
            Self::MissingTurnDraftRef => "missing_turn_draft_ref",
            Self::MissingRequestWorkspaceRef => "missing_request_workspace_ref",
            Self::MissingAssemblyRef => "missing_assembly_ref",
            Self::MissingRouteLineage => "missing_route_lineage",
            Self::MissingSpendLineage => "missing_spend_lineage",
            Self::MissingApprovalLineage => "missing_approval_lineage",
            Self::MissingPreApplyReviewSurface => "missing_pre_apply_review_surface",
            Self::PreApplyPacketClaimsApply => "pre_apply_packet_claims_apply",
            Self::AppliedPacketMissingApproval => "applied_packet_missing_approval",
            Self::AppliedPacketMissingApplyLineage => "applied_packet_missing_apply_lineage",
            Self::RejectedPacketMissingRejectionLineage => {
                "rejected_packet_missing_rejection_lineage"
            }
            Self::CitationSourceIdentityMissing => "citation_source_identity_missing",
            Self::DocsCitationRevisionMissing => "docs_citation_revision_missing",
            Self::CitationAnchorMissing => "citation_anchor_missing",
            Self::HiddenOrOmittedCitationNoteMissing => "hidden_or_omitted_citation_note_missing",
            Self::TaintedSourceMissingFence => "tainted_source_missing_fence",
            Self::TaintedFenceMissingExplanation => "tainted_fence_missing_explanation",
            Self::PolicyDisallowedContextUnexplained => "policy_disallowed_context_unexplained",
            Self::DerivedExplanationMissingInference => "derived_explanation_missing_inference",
            Self::DerivedExplanationMissingConfidence => "derived_explanation_missing_confidence",
            Self::MissingRunningBuildIdentityRef => "missing_running_build_identity_ref",
            Self::MissingSourceContractRefs => "missing_source_contract_refs",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
            Self::ContextHandoffInvalid => "context_handoff_invalid",
            Self::ContextHandoffIdentityMismatch => "context_handoff_identity_mismatch",
            Self::DocsCitationRecordInvalid => "docs_citation_record_invalid",
            Self::DocsCitationRecordMismatch => "docs_citation_record_mismatch",
        }
    }
}

fn cited_source_class_from_docs_source(source_class: CitationSourceClass) -> CitedSourceClass {
    match source_class {
        CitationSourceClass::ProjectDocs
        | CitationSourceClass::MirroredOfficialDocs
        | CitationSourceClass::VendorProviderDocs
        | CitationSourceClass::SupportRunbook => CitedSourceClass::DocsPackExcerpt,
        CitationSourceClass::GeneratedReference => CitedSourceClass::GeneratedReference,
        CitationSourceClass::CuratedKnowledgePack => CitedSourceClass::GlossaryEntry,
        CitationSourceClass::DerivedExplanation => CitedSourceClass::ExplainerSnapshot,
    }
}

fn citation_visibility_from_docs_anchor(
    availability: DocsCitationAnchorAvailability,
) -> CitationVisibilityClass {
    match availability {
        DocsCitationAnchorAvailability::ExactAnchorAvailable => {
            CitationVisibilityClass::AnchorAvailable
        }
        DocsCitationAnchorAvailability::AnchorUnavailableDisclosed => {
            CitationVisibilityClass::AnchorUnavailableDisclosed
        }
        DocsCitationAnchorAvailability::HiddenByPolicy => CitationVisibilityClass::HiddenByPolicy,
        DocsCitationAnchorAvailability::OmittedByPolicy => CitationVisibilityClass::OmittedByPolicy,
        DocsCitationAnchorAvailability::NotCitationBearing => {
            CitationVisibilityClass::NotCitationBearing
        }
    }
}

fn evidence_freshness_from_docs(freshness_class: DocsFreshnessClass) -> EvidenceFreshnessClass {
    match freshness_class {
        DocsFreshnessClass::AuthoritativeLive => EvidenceFreshnessClass::AuthoritativeLive,
        DocsFreshnessClass::WarmCached => EvidenceFreshnessClass::WarmCached,
        DocsFreshnessClass::DegradedCached => EvidenceFreshnessClass::DegradedCached,
        DocsFreshnessClass::Stale => EvidenceFreshnessClass::Stale,
        DocsFreshnessClass::Unverified => EvidenceFreshnessClass::Unverified,
    }
}

fn packet_contains_forbidden_boundary_material(packet: &AiMutationEvidencePacket) -> bool {
    let mut values = vec![
        packet.evidence_packet_id.as_str(),
        packet.mutation_wedge_ref.as_str(),
        packet.composer_session_ref.as_str(),
        packet.turn_draft_ref.as_str(),
        packet.request_workspace_ref.as_str(),
        packet.assembly_ref.as_str(),
        packet.running_build_identity_ref.as_str(),
        packet.minted_at.as_str(),
        packet.route_spend_lineage.routing_packet_ref.as_str(),
        packet.route_spend_lineage.route_receipt_ref.as_str(),
        packet.route_spend_lineage.spend_receipt_ref.as_str(),
        packet
            .route_spend_lineage
            .selected_provider_entry_ref
            .as_str(),
        packet.route_spend_lineage.selected_model_entry_ref.as_str(),
        packet.route_spend_lineage.selected_provider_label.as_str(),
        packet.route_spend_lineage.selected_model_label.as_str(),
        packet.route_spend_lineage.budget_owner_ref.as_str(),
        packet.review_lineage.review_surface_ref.as_str(),
        packet.review_lineage.patch_review_summary_ref.as_str(),
        packet.policy_context.policy_epoch_ref.as_str(),
    ];
    if let Some(completed_at) = &packet.completed_at {
        values.push(completed_at.as_str());
    }
    if let Some(exec) = &packet.policy_context.execution_context_ref {
        values.push(exec.as_str());
    }
    if let Some(ticket) = &packet
        .route_spend_lineage
        .originating_route_approval_ticket_ref
    {
        values.push(ticket.as_str());
    }
    if let Some(checkpoint) = &packet.review_lineage.rollback_checkpoint_ref {
        values.push(checkpoint.as_str());
    }
    if let Some(journal) = &packet.review_lineage.mutation_journal_ref {
        values.push(journal.as_str());
    }
    values.extend(
        packet
            .route_spend_lineage
            .route_change_lineage_refs
            .iter()
            .map(String::as_str),
    );
    values.extend(
        packet
            .review_lineage
            .produced_artifact_refs
            .iter()
            .map(String::as_str),
    );
    values.extend(
        packet
            .review_lineage
            .validation_summary_refs
            .iter()
            .map(String::as_str),
    );
    values.extend(packet.source_contract_refs.iter().map(String::as_str));

    values
        .iter()
        .any(|value| contains_forbidden_boundary_material(value))
        || packet
            .approval_lineage
            .iter()
            .any(approval_contains_forbidden_boundary_material)
        || packet
            .cited_sources
            .iter()
            .any(source_contains_forbidden_boundary_material)
        || packet
            .derived_explanations
            .iter()
            .any(explanation_contains_forbidden_boundary_material)
        || packet
            .tainted_context_fences
            .iter()
            .any(fence_contains_forbidden_boundary_material)
        || packet
            .context_handoff
            .as_ref()
            .is_some_and(context_handoff_contains_forbidden_boundary_material)
}

fn approval_contains_forbidden_boundary_material(approval: &ApprovalLineageEntry) -> bool {
    [
        approval.approval_lineage_id.as_str(),
        approval.approval_ticket_ref.as_str(),
        approval.preview_ref.as_str(),
        approval.policy_epoch_ref.as_str(),
        approval.decided_at.as_str(),
        approval.summary_label.as_str(),
    ]
    .iter()
    .any(|value| contains_forbidden_boundary_material(value))
}

fn source_contains_forbidden_boundary_material(source: &CitedSourceReference) -> bool {
    [
        source.source_reference_id.as_str(),
        source.source_identity_ref.as_str(),
    ]
    .iter()
    .any(|value| contains_forbidden_boundary_material(value))
        || source
            .source_revision_ref
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
        || source
            .docs_pack_ref
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
        || source
            .docs_pack_revision_ref
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
        || source
            .exact_anchor_ref
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
        || source
            .hidden_or_omitted_citation_note
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
        || source
            .docs_node_identity
            .as_ref()
            .is_some_and(docs_node_identity_contains_forbidden_boundary_material)
        || source
            .citation_anchor
            .as_ref()
            .is_some_and(citation_anchor_contains_forbidden_boundary_material)
}

fn docs_node_identity_contains_forbidden_boundary_material(identity: &DocsNodeIdentity) -> bool {
    [
        identity.record_kind.as_str(),
        identity.docs_node_id.as_str(),
        identity.source_pack_ref.as_str(),
        identity.source_pack_revision_ref.as_str(),
        identity.version_or_revision_ref.as_str(),
        identity.source_locale.as_str(),
        identity.requested_locale.as_str(),
        identity.effective_locale.as_str(),
        identity.exact_reopen_ref.as_str(),
    ]
    .iter()
    .any(|value| contains_forbidden_boundary_material(value))
        || identity
            .source_language_fallback_ref
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
        || identity
            .hidden_or_omitted_note
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
        || identity
            .citation_anchor_refs
            .iter()
            .any(|value| contains_forbidden_boundary_material(value))
}

fn citation_anchor_contains_forbidden_boundary_material(anchor: &CitationAnchorAlpha) -> bool {
    [
        anchor.record_kind.as_str(),
        anchor.anchor_id.as_str(),
        anchor.docs_node_ref.as_str(),
        anchor.source_pack_ref.as_str(),
        anchor.source_pack_revision_ref.as_str(),
        anchor.target_ref.as_str(),
        anchor.locale.as_str(),
    ]
    .iter()
    .any(|value| contains_forbidden_boundary_material(value))
        || anchor
            .exact_anchor_ref
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
        || anchor
            .hidden_or_omitted_note
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
}

fn explanation_contains_forbidden_boundary_material(
    explanation: &DerivedExplanationLineage,
) -> bool {
    contains_forbidden_boundary_material(&explanation.explanation_ref)
        || contains_forbidden_boundary_material(&explanation.confidence_reason_label)
        || explanation
            .basis_source_reference_refs
            .iter()
            .any(|value| contains_forbidden_boundary_material(value))
}

fn fence_contains_forbidden_boundary_material(fence: &TaintedContextFence) -> bool {
    [
        fence.fence_id.as_str(),
        fence.segment_id_ref.as_str(),
        fence.user_visible_explanation_label.as_str(),
    ]
    .iter()
    .any(|value| contains_forbidden_boundary_material(value))
        || fence
            .source_reference_ref
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
}

fn context_handoff_contains_forbidden_boundary_material(
    handoff: &AiContextEvidenceHandoff,
) -> bool {
    [
        handoff.handoff_id.as_str(),
        handoff.composer_context_snapshot_ref.as_str(),
        handoff.composer_session_ref.as_str(),
        handoff.turn_draft_ref.as_str(),
        handoff.request_workspace_ref.as_str(),
    ]
    .iter()
    .any(|value| contains_forbidden_boundary_material(value))
        || handoff.context_rows.iter().any(|row| {
            [
                row.context_item_id.as_str(),
                row.group_token.as_str(),
                row.state_token.as_str(),
                row.source_class_token.as_str(),
                row.stable_identity_ref.as_str(),
                row.freshness_token.as_str(),
                row.trust_token.as_str(),
                row.locality_token.as_str(),
            ]
            .iter()
            .any(|value| contains_forbidden_boundary_material(value))
                || row
                    .omission_reason_token
                    .as_deref()
                    .is_some_and(contains_forbidden_boundary_material)
                || row
                    .source_attachment_ref
                    .as_deref()
                    .is_some_and(contains_forbidden_boundary_material)
                || row
                    .source_mention_ref
                    .as_deref()
                    .is_some_and(contains_forbidden_boundary_material)
                || row
                    .docs_node_ref
                    .as_deref()
                    .is_some_and(contains_forbidden_boundary_material)
                || row
                    .docs_source_class_token
                    .as_deref()
                    .is_some_and(contains_forbidden_boundary_material)
                || row
                    .version_or_revision_ref
                    .as_deref()
                    .is_some_and(contains_forbidden_boundary_material)
                || row
                    .exact_anchor_ref
                    .as_deref()
                    .is_some_and(contains_forbidden_boundary_material)
                || row
                    .citation_availability_token
                    .as_deref()
                    .is_some_and(contains_forbidden_boundary_material)
                || row
                    .source_language_fallback_token
                    .as_deref()
                    .is_some_and(contains_forbidden_boundary_material)
        })
        || handoff
            .graph_cue_packets
            .iter()
            .any(graph_cue_packet_contains_forbidden_boundary_material)
}

fn graph_cue_packet_contains_forbidden_boundary_material(packet: &GraphFactCuePacket) -> bool {
    [
        packet.record_kind.as_str(),
        packet.packet_id.as_str(),
        packet.source_packet_ref.as_str(),
        packet.query_request_id.as_str(),
        packet.workspace_id.as_str(),
        packet.readiness.as_str(),
        packet.emitted_at.as_str(),
    ]
    .iter()
    .any(|value| contains_forbidden_boundary_material(value))
        || packet
            .workspace_graph_id
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
        || packet
            .query_class
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
        || packet
            .query_family_tag
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
        || packet.cues.iter().any(|cue| {
            [
                cue.cue_id.as_str(),
                cue.display_label.as_str(),
                cue.row_class.as_str(),
                cue.readiness.as_str(),
            ]
            .iter()
            .any(|value| contains_forbidden_boundary_material(value))
                || cue
                    .graph_ref
                    .as_deref()
                    .is_some_and(contains_forbidden_boundary_material)
                || cue
                    .confidence_level
                    .as_deref()
                    .is_some_and(contains_forbidden_boundary_material)
                || cue
                    .freshness
                    .as_deref()
                    .is_some_and(contains_forbidden_boundary_material)
                || cue
                    .evidence_state
                    .as_deref()
                    .is_some_and(contains_forbidden_boundary_material)
                || cue
                    .relative_path
                    .as_deref()
                    .is_some_and(contains_forbidden_boundary_material)
                || cue
                    .symbol_ref
                    .as_deref()
                    .is_some_and(contains_forbidden_boundary_material)
                || cue
                    .partial_truth_causes
                    .iter()
                    .any(|cause| contains_forbidden_boundary_material(cause))
                || cue
                    .export_labels
                    .iter()
                    .any(|label| contains_forbidden_boundary_material(label))
        })
}

fn contains_forbidden_boundary_material(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("api_key")
        || lower.contains("oauth_token")
        || lower.contains("bearer ")
}

#[cfg(test)]
mod tests;
