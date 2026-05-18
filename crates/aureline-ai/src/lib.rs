//! AI composer, context-inspector, provider-routing, and evidence primitives.
//!
//! This crate owns inspectable AI records consumed by shell, diagnostics,
//! support export, and evidence surfaces. The composer lane exposes one
//! [`composer::ComposerDraft`] object plus typed mention, attachment,
//! slash-command, and route-placeholder vocabularies. The registry lane exposes
//! one [`registry::ProviderModelRegistryPacket`] object for provider/model,
//! external-tool, execution-location, and route-policy truth. The graduation
//! lane exposes one [`graduation::AiGraduationState`] object for packet
//! freshness, eval-threshold, owner, cost-profile, and kill-switch gates on
//! claimed beta AI surfaces. The routing lane exposes one
//! [`routing::AiRoutingPacket`] object for provider/model identity, quota
//! explainability, latency/cost envelopes, and visible route-change lineage on
//! claimed hosted-model paths. The evidence lane exposes one
//! [`evidence::AiMutationEvidencePacket`] object for review-first mutation
//! packets that preserve cited-source truth, tainted-context fences, route/spend
//! refs, and approval lineage before apply.
//!
//! These records carry no credential bodies, raw provider payloads, raw
//! endpoint URLs, exact token counts, exact cost amounts, or raw diff bodies.
//! Consumers project the typed packets directly and never re-derive authority,
//! lifecycle state, provider identity, quota state, citation truth, approval
//! lineage, or route explanations locally.
//!
//! The frozen cross-tool contracts the seed projects against are
//! [`/docs/ai/prompt_composer_contract.md`](../../../docs/ai/prompt_composer_contract.md)
//! and
//! [`/docs/ai/context_assembly_contract.md`](../../../docs/ai/context_assembly_contract.md).
//! The routing alpha entry point is
//! [`/docs/ai/routing_cost_alpha.md`](../../../docs/ai/routing_cost_alpha.md).
//! The records cover bounded, honest subsets of the frozen vocabularies and
//! grow additively without forking truth.
//! Semantic navigation citations use [`aureline_navigation::target_model`] so
//! AI context rows carry relation, access, proof, freshness, ambiguity, and
//! scope-completeness labels instead of copied UI strings.

#![doc(html_root_url = "https://docs.rs/aureline-ai/0.0.0")]

pub mod composer;
pub mod context_inspector;
pub mod evidence;
pub mod graduation;
pub mod prompt_composer;
pub mod registry;
pub mod routing;
pub mod routing_policy;
pub mod tainted_context;

pub use aureline_navigation::target_model as navigation_target_model;
pub use composer::beta::{
    current_beta_composer_context_evidence_support_export,
    ComposerContextEvidenceBetaArtifactError, ComposerContextEvidenceBetaInput,
    ComposerContextEvidenceBetaPacket, ComposerContextEvidenceBetaViolation,
    ComposerContextEvidenceContextRow, ComposerContextEvidenceSurfaceClass,
    ComposerContextEvidenceSurfaceRow, COMPOSER_CONTEXT_EVIDENCE_BETA_AI_DOC_REF,
    COMPOSER_CONTEXT_EVIDENCE_BETA_ARTIFACT_REF, COMPOSER_CONTEXT_EVIDENCE_BETA_FIXTURE_DIR,
    COMPOSER_CONTEXT_EVIDENCE_BETA_PACKET_RECORD_KIND, COMPOSER_CONTEXT_EVIDENCE_BETA_SCHEMA_REF,
    COMPOSER_CONTEXT_EVIDENCE_BETA_SCHEMA_VERSION, COMPOSER_CONTEXT_EVIDENCE_BETA_UX_DOC_REF,
};
pub use composer::{
    AttachmentKind, AttachmentStatusClass, BlockReason, ComposerAttachment, ComposerDraft,
    ComposerDraftState, ComposerMention, ComposerSlashCommandInvocation, DispatchTargetClass,
    MentionKind, MentionResolutionState, PrototypeLabel, ProviderClass, RoutePathClass,
    RoutePlaceholder, SelectionReasonClass, SlashCommandResolutionState, SourceClass, TrustPosture,
    ValidationOutcome, COMPOSER_DRAFT_RECORD_KIND, COMPOSER_DRAFT_SCHEMA_VERSION,
};
pub use context_inspector::{
    AiContextEvidenceHandoff, AiContextEvidenceHandoffRow, AiContextRetrievalExport,
    AiContextRetrievalExportViolation, AiContextSearchOperatorTruthExport,
    AiContextSearchOperatorTruthExportViolation, BudgetPressureClass,
    CitationAnchorAvailabilityClass, ComposerAttachmentPill, ComposerBudgetStrip,
    ComposerContextAlphaInput, ComposerContextAlphaSnapshot, ComposerContextAlphaViolation,
    ComposerContextItem, ComposerContextReviewLock, ComposerContextReviewState,
    ComposerMentionPreview, ContextFreshnessClass, ContextGroupClass, ContextItemStateClass,
    ContextLocalityClass, ContextOmissionReasonClass, ContextTrustClass, DocsKnowledgeIdentity,
    DocsKnowledgeSourceClass, ExecutionBoundaryClass, IntentModeClass, MentionPreviewStateClass,
    ReviewLockClass, SourceLanguageFallbackClass, AI_CONTEXT_EVIDENCE_HANDOFF_RECORD_KIND,
    AI_CONTEXT_OPERATOR_TRUTH_EXPORT_RECORD_KIND, AI_CONTEXT_OPERATOR_TRUTH_EXPORT_SCHEMA_VERSION,
    AI_CONTEXT_RETRIEVAL_EXPORT_RECORD_KIND, AI_CONTEXT_RETRIEVAL_EXPORT_SCHEMA_VERSION,
    COMPOSER_CONTEXT_ALPHA_RECORD_KIND, COMPOSER_CONTEXT_ALPHA_SCHEMA_VERSION,
};
pub use evidence::{
    AiMutationEvidenceCitationSupportRow, AiMutationEvidenceDerivedSupportRow,
    AiMutationEvidenceFenceSupportRow, AiMutationEvidencePacket, AiMutationEvidencePacketInput,
    AiMutationEvidenceReviewRow, AiMutationEvidenceSupportPacket, AiMutationEvidenceViolation,
    ApplyOutcomeClass, ApprovalActorClass, ApprovalDecisionClass, ApprovalLineageEntry,
    CitationVisibilityClass, CitedSourceClass, CitedSourceReference, ConfidenceClass,
    DerivedExplanationLineage, EvidenceFreshnessClass, EvidenceRedactionClass,
    EvidenceSourcePosture, InferenceClass, MutationEvidenceState, MutationIntentClass,
    MutationReviewLineage, RouteSpendLineage, TaintFenceReasonClass, TaintFenceStrategy,
    TaintUsageConstraint, TaintedContextFence, TaintedEvidenceSourceClass, ValidationOutcomeClass,
    AI_MUTATION_EVIDENCE_PACKET_RECORD_KIND, AI_MUTATION_EVIDENCE_REVIEW_ROW_RECORD_KIND,
    AI_MUTATION_EVIDENCE_SCHEMA_VERSION, AI_MUTATION_EVIDENCE_SUPPORT_PACKET_RECORD_KIND,
};
pub use graduation::{
    current_beta_graduation_packet_artifacts, current_beta_graduation_state,
    AiGraduationConsumerProjection, AiGraduationConsumerSurfaceClass, AiGraduationEvidenceEntry,
    AiGraduationFreshnessClass, AiGraduationGateState, AiGraduationPacket,
    AiGraduationPolicyContext, AiGraduationRollbackPlan, AiGraduationState,
    AiGraduationSupportClass, AiGraduationSurfaceStatus, AiGraduationSurfaceSupportSummary,
    AiGraduationViolation, AI_GRADUATION_PACKET_RECORD_KIND, AI_GRADUATION_STATE_RECORD_KIND,
    AI_GRADUATION_STATE_SCHEMA_VERSION, REQUIRED_BETA_EVIDENCE_KINDS,
};
pub use prompt_composer::{
    current_beta_prompt_composer_conformance_export, DraftRetentionScopeClass,
    PreviewBranchComposerRow, PromptBudgetActionClass, PromptBudgetDecisionRow, PromptBudgetStrip,
    PromptComposerConformanceArtifactError, PromptComposerConformanceInput,
    PromptComposerConformancePacket, PromptComposerConformanceViolation,
    PromptComposerEdgeCaseClass, PromptComposerEdgeCaseRow, PromptComposerSafeFallbackClass,
    PromptContextAttachment, PromptDraftPersistence, PromptEvidenceLineage,
    PromptEvidencePacketClass, PromptInputSemantics, PromptIntentRow, PromptMentionKind,
    PromptMentionResolution, PromptMentionResolutionClass, PromptSlashCommandBinding,
    PROMPT_COMPOSER_AI_DOC_REF, PROMPT_COMPOSER_BETA_UX_DOC_REF,
    PROMPT_COMPOSER_CONFORMANCE_ARTIFACT_REF, PROMPT_COMPOSER_CONFORMANCE_RECORD_KIND,
    PROMPT_COMPOSER_CONFORMANCE_SCHEMA_VERSION, PROMPT_COMPOSER_CONFORMANCE_SUMMARY_REF,
    PROMPT_COMPOSER_DRAFT_SCHEMA_REF, PROMPT_COMPOSER_DRILL_FIXTURE_DIR,
    PROMPT_CONTEXT_ATTACHMENT_SCHEMA_REF,
};
pub use registry::{
    AiFeatureClass, ClaimedAiSurface, ExternalToolExecutionLocusClass, ExternalToolRegistryEntry,
    ExternalToolRegistrySupportSummary, ExternalToolSideEffectClass, ExternalToolTransportClass,
    ModelRegistryEntry, ModelRegistrySupportSummary, ProviderModelRegistryPacket,
    ProviderModelRegistrySupportExport, ProviderModelRegistrySurfaceRow,
    ProviderModelRegistryViolation, ProviderRegistryEntry, ProviderRegistrySupportSummary,
    RegistryApprovalPostureClass, RegistryAuthModeClass, RegistryConsumerProjection,
    RegistryConsumerSurfaceClass, RegistryDataClass, RegistryDisclosureKind,
    RegistryLifecycleStateClass, RegistryRouteCandidate, RegistryRoutePolicy,
    RegistryRouteReasonClass, RegistryRoutingPolicyClass, RegistrySurfaceSupportSummary,
    RegistryTransportClass, RetrievalTruthStateClass, RouteEligibilityClass,
    PROVIDER_MODEL_REGISTRY_CLAIMED_SURFACE_RECORD_KIND,
    PROVIDER_MODEL_REGISTRY_EXTERNAL_TOOL_ENTRY_RECORD_KIND,
    PROVIDER_MODEL_REGISTRY_MODEL_ENTRY_RECORD_KIND, PROVIDER_MODEL_REGISTRY_PACKET_RECORD_KIND,
    PROVIDER_MODEL_REGISTRY_PROVIDER_ENTRY_RECORD_KIND, PROVIDER_MODEL_REGISTRY_SCHEMA_VERSION,
    PROVIDER_MODEL_REGISTRY_SUPPORT_EXPORT_RECORD_KIND,
};
pub use routing::{
    AiRouteCandidate, AiRouteProviderClass, AiRoutingExecutionContextSummary, AiRoutingPacket,
    AiRoutingSupportPacket, AiRoutingSupportRouteChangeRow, AiRoutingSurfaceRow,
    AiRoutingViolation, CostEnvelopeClass, CostVisibilityClass, DeploymentProfileClass,
    ExecutionLocusClass, ExhaustionStateClass, LatencyCostEnvelope, LatencyEnvelopeClass,
    PolicyTrustState, QuotaFamilyClass, QuotaInspector, QuotaScopeClass, QuotaStateClass,
    RetentionStanceClass, RouteChangeCauseClass, RouteChangeLineage, RouteOriginClass,
    RouteSelectionOverrideReasonClass, RouteSelectionReasonClass, RoutingPolicyContext,
    RoutingRunStateClass, SelectedOutcomeClass, TokenCeilingClass, ToolCallCeilingClass,
    WallTimeCeilingClass, AI_ROUTING_PACKET_RECORD_KIND, AI_ROUTING_SCHEMA_VERSION,
    AI_ROUTING_SUPPORT_PACKET_RECORD_KIND,
};
pub use routing_policy::{
    current_beta_cost_routing_packet, BudgetScopeClass, BudgetScopeOutcomeClass,
    BudgetScopeOutcomeRow, CostRoutingBetaArtifactError, CostRoutingBetaPacket,
    CostRoutingBetaViolation, CostRoutingSurfaceRow, RedactionClass,
    SpendAttributionDimensionClass, SpendAttributionValueRow, SpendPolicyContext,
    SpendReceiptRecord, WasChargedToUserClass, COST_ROUTING_BETA_PACKET_RECORD_KIND,
    COST_ROUTING_BETA_SCHEMA_VERSION, SPEND_RECEIPT_RECORD_KIND, SPEND_RECEIPT_SCHEMA_VERSION,
};
pub use tainted_context::{
    current_beta_tainted_context_support_export, suspicious_detector_tokens,
    TaintedContextApprovalFenceRow, TaintedContextApprovalRequirementClass,
    TaintedContextBetaArtifactError, TaintedContextBetaInput, TaintedContextBetaPacket,
    TaintedContextBetaViolation, TaintedContextInputSourceClass,
    TaintedContextNarrowingDecisionRow, TaintedContextOriginLocusClass,
    TaintedContextPolicyContext, TaintedContextPolicyNarrowingClass,
    TaintedContextPromotionGateClass, TaintedContextReasonClass, TaintedContextRetrievalTruthClass,
    TaintedContextRunModeClass, TaintedContextSourceRow, TaintedContextSurfaceClass,
    TaintedContextSurfaceRow, TaintedContextTaintClass, TAINTED_CONTEXT_BETA_AI_DOC_REF,
    TAINTED_CONTEXT_BETA_ARTIFACT_REF, TAINTED_CONTEXT_BETA_FIXTURE_DIR,
    TAINTED_CONTEXT_BETA_PACKET_RECORD_KIND, TAINTED_CONTEXT_BETA_SCHEMA_REF,
    TAINTED_CONTEXT_BETA_SCHEMA_VERSION,
};
