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
//! refs, and approval lineage before apply. The scoped-apply hardening lane
//! exposes one [`harden_ai_scoped_apply::AiScopedApplyHardeningPacket`] object
//! binding the preview → approval → apply → revert lifecycle, scoped-apply and
//! multi-file patch honesty, cross-wedge command parity, route/spend authority
//! truth, and exportable evidence/rollback lineage on claimed stable apply
//! paths. The repo-instruction hardening lane exposes one
//! [`harden_repo_ai_instructions::RepoAiInstructionHardeningPacket`] object
//! binding repo-defined instruction precedence and trust, policy-interaction
//! outcomes that deny repo widening while admitting repo narrowing, a
//! provider-neutral kill switch that fails closed across every provider, model,
//! and external-tool route, a fully reversible backout posture, cross-wedge
//! command parity, and exportable evidence/rollback lineage on claimed stable
//! runs. The optional AI-adjacent surface audit lane exposes one
//! [`audit_optional_ai_adjacent_surfaces::OptionalAiAdjacentSurfaceAuditPacket`]
//! object that enforces every exposed optional AI surface family — notebook,
//! voice, browser companion, preview/designer, and background branch
//! automation — carries its own current qualification proof or is visibly
//! labeled below Stable instead of inheriting Stable from core AI graduation.
//! The memory-state lane exposes one [`memory::AiMemoryStatePacket`] object
//! binding AI state classes, cache-key invalidation, visible thread/inspector
//! disclosures, delete/export drills, and support-safe manifests on claimed
//! stable AI thread and memory surfaces.
//! The AI-pack rollout lane exposes one
//! [`ai_pack_rollout::AiRolloutPublicationPacket`] object binding provider/model
//! enablement, prompt packs, tool-schema packs, local-model packs, feature-level
//! AI rollout objects, independent downgrade receipts, and mirror/offline
//! publication truth for claimed stable AI routes.
//! The background branch-agent lifecycle lane exposes one
//! [`qualify_background_branch_agent_lifecycle::BackgroundBranchAgentLifecyclePacket`]
//! object binding launch review, active rows, checkpoints, re-review drift,
//! operator takeover, completion review, cleanup posture, and support/export
//! attribution to one stable run id on any claimed stable branch-agent lane.
//! The AI test-generation truth lane exposes one
//! [`ai_test_generation::AiTestGenerationTruthPacket`] object binding concrete
//! proposal triggers, assumption-review sheets, generated-test diff risk
//! classes, sandbox-validation lineage, and measured-versus-estimated coverage
//! impact without letting generated tests count as trusted coverage proof.
//! The AI review-assist truth lane exposes one
//! [`ai_review_assist::AiReviewAssistTruthPacket`] object binding durable
//! finding rows, scope selectors, publish-to-review sheets, and resolution
//! memory to the same review-pack digest and evidence packet lineage across
//! desktop, CLI/headless, browser/companion, and support/export lanes.
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

pub mod ai_pack_rollout;
pub mod ai_review_assist;
pub mod ai_test_generation;
pub mod audit_optional_ai_adjacent_surfaces;
pub mod composer;
pub mod context_inspector;
pub mod evidence;
pub mod finalize_ai_evidence_packets;
pub mod finalize_tainted_context_fences;
pub mod freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents;
pub mod implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru;
pub mod graduation;
pub mod harden_ai_scoped_apply;
pub mod harden_repo_ai_instructions;
pub mod memory;
pub mod prompt_composer;
pub mod publish_stable_ai_graduation_packets;
pub mod qualify_background_branch_agent_lifecycle;
pub mod registry;
pub mod routing;
pub mod routing_policy;
pub mod run_history;
pub mod stabilize_ai_route_and_spend_truth;
pub mod stabilize_prompt_composer;
pub mod tainted_context;
pub mod tool_gateway;

pub use ai_pack_rollout::{
    AiDowngradeReceipt, AiFallbackContract, AiFallbackRouteClass, AiMirrorPublication,
    AiPackRevocationStateClass, AiRolloutObject, AiRolloutObjectKind, AiRolloutPacketViolation,
    AiRolloutPublicationPacket, AiRolloutRingClass, AiRolloutStateClass, AiRouteOriginClass,
    StableAiRouteRow, AI_ROLLOUT_GOVERNANCE_DOC_REF, AI_ROLLOUT_PACKET_RECORD_KIND,
    AI_ROLLOUT_PACKET_SCHEMA_REF, AI_ROLLOUT_PACKET_SCHEMA_VERSION,
    AI_ROLLOUT_PUBLICATION_ARTIFACT_REF, AI_ROLLOUT_SUMMARY_REF, AI_ROLLOUT_SUPPORT_EXPORT_REF,
    LOCAL_MODEL_PACK_PUBLICATION_MANIFEST_REF,
};
pub use ai_review_assist::{
    current_stable_ai_review_assist_truth_export, AffectedReviewHunk,
    AiReviewAssistTruthArtifactError, AiReviewAssistTruthPacket, AiReviewAssistTruthPacketInput,
    AiReviewAssistTruthViolation, AiReviewConfidenceClass, AiReviewConsumerProjection,
    AiReviewConsumerSurface, AiReviewFindingClass, AiReviewFindingRow, AiReviewObjectLineage,
    AiReviewResolutionState, AiReviewSeverityClass, AttributionStateClass,
    ProviderWriteAccessClass, PublishActionClass, PublishDestinationClass, PublishToReviewSheet,
    RedactionNoteClass, RepoInstructionCheckSourceClass, ResolutionMemoryRow, ReviewScopeClass,
    ReviewScopeFreshnessClass, ReviewScopeRerunActionClass, ReviewScopeSelector,
    AI_REVIEW_ASSIST_REVIEW_PACK_CONTRACT_REF, AI_REVIEW_ASSIST_TRUTH_AI_DOC_REF,
    AI_REVIEW_ASSIST_TRUTH_ARTIFACT_REF, AI_REVIEW_ASSIST_TRUTH_FIXTURE_DIR,
    AI_REVIEW_ASSIST_TRUTH_RECORD_KIND, AI_REVIEW_ASSIST_TRUTH_SCHEMA_REF,
    AI_REVIEW_ASSIST_TRUTH_SCHEMA_VERSION, AI_REVIEW_ASSIST_TRUTH_SUMMARY_REF,
};
pub use ai_test_generation::{
    current_stable_ai_test_generation_truth_export, AiTestCandidateRow,
    AiTestGenerationConsumerProjection, AiTestGenerationConsumerSurface, AiTestGenerationLineage,
    AiTestGenerationTruthArtifactError, AiTestGenerationTruthPacket,
    AiTestGenerationTruthPacketInput, AiTestGenerationTruthViolation,
    AiTestGenerationValidationStatus, AssumptionClass, AssumptionReviewRow, AssumptionReviewSheet,
    AssumptionRiskClass, BulkApplyPostureClass, CoverageImpactClass, CoverageImpactNote,
    GeneratedTestDiffClass, GeneratedTestDiffClassRow, GeneratedTestDiffRecord,
    SandboxOutcomeClass, SandboxTargetClass, SandboxValidationRecord, TestCandidateConfidenceClass,
    TestCandidateFlakyRiskClass, TestCandidateReviewState, TestGenerationBrief,
    TestProposalTriggerClass, AI_TEST_GENERATION_GATE_SCHEMA_REF,
    AI_TEST_GENERATION_TESTING_CONTRACT_REF, AI_TEST_GENERATION_TRUTH_AI_DOC_REF,
    AI_TEST_GENERATION_TRUTH_ARTIFACT_REF, AI_TEST_GENERATION_TRUTH_FIXTURE_DIR,
    AI_TEST_GENERATION_TRUTH_RECORD_KIND, AI_TEST_GENERATION_TRUTH_SCHEMA_REF,
    AI_TEST_GENERATION_TRUTH_SCHEMA_VERSION, AI_TEST_GENERATION_TRUTH_SUMMARY_REF,
};
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
pub use finalize_ai_evidence_packets::{
    current_stable_ai_evidence_packet_finalization_export, AbsenceRow, AbsenceStateClass,
    AiEvidenceBranchClass, AiEvidencePacketFinalization, AiEvidencePacketFinalizationArtifactError,
    AiEvidencePacketFinalizationInput, AiEvidencePacketFinalizationViolation, ContextInputsBlock,
    DiffWriteScopeBlock, EvidenceBranchRow, EvidenceOriginClass, EvidencePacketClass,
    FinalizedValidationOutcomeClass, IntentScopeBlock, OutboundRedactionPostureClass,
    OutboundTargetClass, PacketClassRow, RecallLocalityClass, RedactionManifestRow,
    RedactionReasonClass, ReplayLineage, ReplayPostureClass, ReproducibilityImpactClass,
    RetainedArtifactClass, RetainedArtifactRow, RetrievalLaneClass, RetrievalProvenance,
    RollbackExportBlock, ToolPolicyBlock, ToolPolicyDecisionRow, ValidationBlock,
    AI_EVIDENCE_PACKET_FINALIZATION_AI_DOC_REF, AI_EVIDENCE_PACKET_FINALIZATION_ARTIFACT_REF,
    AI_EVIDENCE_PACKET_FINALIZATION_BASE_CONTRACT_REF, AI_EVIDENCE_PACKET_FINALIZATION_FIXTURE_DIR,
    AI_EVIDENCE_PACKET_FINALIZATION_RECORD_KIND, AI_EVIDENCE_PACKET_FINALIZATION_SCHEMA_REF,
    AI_EVIDENCE_PACKET_FINALIZATION_SCHEMA_VERSION, AI_EVIDENCE_PACKET_FINALIZATION_SUMMARY_REF,
};
pub use finalize_tainted_context_fences::{
    current_stable_finalize_tainted_context_export, BoundaryEnforcementClass,
    CommandSurfaceParityRow, ContentBoundaryClass, ContentBoundaryRow,
    FinalizedTaintedContextArtifactError, FinalizedTaintedContextPacket,
    FinalizedTaintedContextPacketInput, FinalizedTaintedContextViolation,
    ImportAuthorityDowngradeClass, ImportMappingOutcomeClass, ImportedDataDowngradeRow,
    TaintedContextEvidenceExport, TaintedFenceRow, FINALIZE_TAINTED_CONTEXT_AI_DOC_REF,
    FINALIZE_TAINTED_CONTEXT_ARTIFACT_REF, FINALIZE_TAINTED_CONTEXT_ASSEMBLY_CONTRACT_REF,
    FINALIZE_TAINTED_CONTEXT_FIXTURE_DIR, FINALIZE_TAINTED_CONTEXT_RECORD_KIND,
    FINALIZE_TAINTED_CONTEXT_SCHEMA_REF, FINALIZE_TAINTED_CONTEXT_SCHEMA_VERSION,
    FINALIZE_TAINTED_CONTEXT_SUMMARY_REF, FINALIZE_TAINTED_CONTEXT_TAINT_CONTRACT_REF,
};
pub use freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::{
    current_stable_m5_ai_workflow_matrix_export, M5AiWorkflowConsumerSurface,
    M5AiWorkflowDowngradeTrigger, M5AiWorkflowEvidenceRequirement, M5AiWorkflowLane,
    M5AiWorkflowMatrixArtifactError, M5AiWorkflowMatrixConsumerProjection,
    M5AiWorkflowMatrixLaneRow, M5AiWorkflowMatrixPacket, M5AiWorkflowMatrixPacketInput,
    M5AiWorkflowMatrixProofFreshness, M5AiWorkflowQualificationClass,
    M5AiWorkflowRollbackPosture, M5AiWorkflowMatrixSecurityReview,
    M5AiWorkflowMatrixViolation, M5_AI_WORKFLOW_MATRIX_ARTIFACT_REF,
    M5_AI_WORKFLOW_MATRIX_BRANCH_AGENT_CONTRACT_REF, M5_AI_WORKFLOW_MATRIX_DOC_REF,
    M5_AI_WORKFLOW_MATRIX_FIXTURE_DIR, M5_AI_WORKFLOW_MATRIX_INLINE_ASSIST_CONTRACT_REF,
    M5_AI_WORKFLOW_MATRIX_PATCH_REVIEW_CONTRACT_REF, M5_AI_WORKFLOW_MATRIX_PATCH_SEQUENCE_REF,
    M5_AI_WORKFLOW_MATRIX_RECORD_KIND, M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
    M5_AI_WORKFLOW_MATRIX_SCHEMA_VERSION, M5_AI_WORKFLOW_MATRIX_SUMMARY_REF,
};
pub use implement_a_richer_prompt_composer_with_intent_modes_typed_attachments_context_pinning_and_omitted_context_tru::{
    current_richer_prompt_composer_export, AttachmentProvenanceClass, AttachmentSemanticRoleClass,
    ExclusionFreshnessClass, IntentModeBehaviorConstraint, OmittedContextRestorationClass,
    PinAutoRefreshClass, PinPolicyClass, RicherAttachmentRow, RicherBudgetDecisionRow,
    RicherBudgetStrip, RicherOmittedContextRow, RicherPinnedContextRow, RicherPromptComposerArtifactError,
    RicherPromptComposerInput, RicherPromptComposerPacket, RicherPromptComposerViolation,
    RicherSurfaceConsistencyRow, RicherThreadHeader, RicherIntentModeRow,
    RICHER_PROMPT_COMPOSER_ARTIFACT_REF, RICHER_PROMPT_COMPOSER_BASE_CONTRACT_REF,
    RICHER_PROMPT_COMPOSER_BETA_ARTIFACT_REF, RICHER_PROMPT_COMPOSER_DOC_REF,
    RICHER_PROMPT_COMPOSER_FIXTURE_DIR, RICHER_PROMPT_COMPOSER_RECORD_KIND,
    RICHER_PROMPT_COMPOSER_SCHEMA_REF, RICHER_PROMPT_COMPOSER_SCHEMA_VERSION,
    RICHER_PROMPT_COMPOSER_STABLE_ARTIFACT_REF, RICHER_PROMPT_COMPOSER_SUMMARY_REF,
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
pub use harden_ai_scoped_apply::{
    current_stable_ai_scoped_apply_hardening_export, AiScopedApplyHardeningArtifactError,
    AiScopedApplyHardeningPacket, AiScopedApplyHardeningPacketInput,
    AiScopedApplyHardeningViolation, ApplyLifecycleBlock, ApplyLifecycleStateClass,
    ApplyWriteScopeClass, CommandSurfaceClass, EvidenceExportBlock, PatchChangeKind, PatchFileRow,
    PatchHonestyBlock, RouteSpendAuthorityBlock, ScopeContractBlock, SurfaceParityRow,
    SurfaceQualificationClass, AI_SCOPED_APPLY_HARDENING_AI_DOC_REF,
    AI_SCOPED_APPLY_HARDENING_ARTIFACT_REF, AI_SCOPED_APPLY_HARDENING_FIXTURE_DIR,
    AI_SCOPED_APPLY_HARDENING_PARITY_CONTRACT_REF,
    AI_SCOPED_APPLY_HARDENING_PREVIEW_APPLY_REVERT_CONTRACT_REF,
    AI_SCOPED_APPLY_HARDENING_RECORD_KIND, AI_SCOPED_APPLY_HARDENING_SCHEMA_REF,
    AI_SCOPED_APPLY_HARDENING_SCHEMA_VERSION, AI_SCOPED_APPLY_HARDENING_SUMMARY_REF,
};
pub use harden_repo_ai_instructions::{
    current_stable_harden_repo_ai_instructions_export, BackoutCompletenessClass, BackoutPosture,
    InstructionTrustPostureClass, KillSwitchPosture, KillSwitchScopeClass, KillSwitchStateClass,
    PolicyInteractionOutcomeClass, PolicyInteractionRow, RepoAiInstructionHardeningArtifactError,
    RepoAiInstructionHardeningPacket, RepoAiInstructionHardeningPacketInput,
    RepoAiInstructionHardeningViolation, RepoInstructionEvidenceExport, RepoInstructionRow,
    RepoInstructionSourceClass, RepoProhibitedCaseClass, HARDEN_REPO_AI_INSTRUCTIONS_AI_DOC_REF,
    HARDEN_REPO_AI_INSTRUCTIONS_ARTIFACT_REF, HARDEN_REPO_AI_INSTRUCTIONS_FIXTURE_DIR,
    HARDEN_REPO_AI_INSTRUCTIONS_KILL_SWITCH_CONTRACT_REF, HARDEN_REPO_AI_INSTRUCTIONS_RECORD_KIND,
    HARDEN_REPO_AI_INSTRUCTIONS_SCHEMA_REF, HARDEN_REPO_AI_INSTRUCTIONS_SCHEMA_VERSION,
    HARDEN_REPO_AI_INSTRUCTIONS_SUMMARY_REF, HARDEN_REPO_AI_INSTRUCTIONS_TAINT_CONTRACT_REF,
};
pub use memory::{
    current_stable_ai_memory_state_export, AiMemoryStateArtifactError, AiMemoryStatePacket,
    AiMemoryStatePacketInput, AiMemoryStateViolation, AiStateClass, CacheKeyComponentClass,
    DeleteExportDrillRow, DeletePostureClass, DurabilityClass, DurableCacheKeyContract,
    ExportPostureClass, InvalidationReasonCode, MemoryScopeClass, MemoryStateClassRow,
    MemorySurfaceClass, MemorySurfaceProjectionRow, OwnerPolicyClass, RetentionModeClass,
    ReusableMemoryFence, SensitivityTierClass, SupportSafeMemoryManifest,
    AI_MEMORY_OBJECT_SCHEMA_REF, AI_MEMORY_RECONCILIATION_CONTRACT_REF, AI_MEMORY_STATE_AI_DOC_REF,
    AI_MEMORY_STATE_ARTIFACT_REF, AI_MEMORY_STATE_MATRIX_REF, AI_MEMORY_STATE_RECORD_KIND,
    AI_MEMORY_STATE_SCHEMA_REF, AI_MEMORY_STATE_SCHEMA_VERSION,
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
pub use qualify_background_branch_agent_lifecycle::{
    current_stable_background_branch_agent_lifecycle_export,
    BackgroundBranchAgentLifecycleArtifactError, BackgroundBranchAgentLifecyclePacket,
    BackgroundBranchAgentLifecycleViolation, BranchAgentActiveRunRow,
    BranchAgentCancellationPosture, BranchAgentCheckpointRow, BranchAgentCleanupDisposition,
    BranchAgentCleanupRow, BranchAgentCompletionReview, BranchAgentConsumerProjection,
    BranchAgentDriftDrillRow, BranchAgentDriftTrigger, BranchAgentExecutionLocus,
    BranchAgentLaunchReviewSheet, BranchAgentOperatorAction, BranchAgentRunState,
    BranchAgentSecurityReview, BranchAgentTakeoverRow, BACKGROUND_BRANCH_AGENT_BASE_CONTRACT_REF,
    BACKGROUND_BRANCH_AGENT_LIFECYCLE_AI_DOC_REF, BACKGROUND_BRANCH_AGENT_LIFECYCLE_ARTIFACT_REF,
    BACKGROUND_BRANCH_AGENT_LIFECYCLE_FIXTURE_DIR, BACKGROUND_BRANCH_AGENT_LIFECYCLE_RECORD_KIND,
    BACKGROUND_BRANCH_AGENT_LIFECYCLE_SCHEMA_REF, BACKGROUND_BRANCH_AGENT_LIFECYCLE_SCHEMA_VERSION,
    BACKGROUND_BRANCH_AGENT_LIFECYCLE_SUMMARY_REF,
};
pub use registry::{
    AiFeatureClass, ClaimedAiSurface, ExternalToolExecutionLocusClass, ExternalToolRegistryEntry,
    ExternalToolRegistrySupportSummary, ExternalToolSideEffectClass, ExternalToolTransportClass,
    LocalModelPackRegistryEntry, LocalModelPackRegistrySupportSummary, LocalModelPackStorageClass,
    ModelRegistryEntry, ModelRegistrySupportSummary, PromptPackRegistryEntry,
    PromptPackRegistrySupportSummary, ProviderModelRegistryPacket,
    ProviderModelRegistrySupportExport, ProviderModelRegistrySurfaceRow,
    ProviderModelRegistryViolation, ProviderRegistryEntry, ProviderRegistrySupportSummary,
    RegistryApprovalPostureClass, RegistryAuthModeClass, RegistryConsumerProjection,
    RegistryConsumerSurfaceClass, RegistryDataClass, RegistryDisclosureKind,
    RegistryLifecycleStateClass, RegistryRouteCandidate, RegistryRoutePolicy,
    RegistryRouteReasonClass, RegistryRoutingPolicyClass, RegistrySurfaceSupportSummary,
    RegistryTransportClass, RetrievalTruthStateClass, RouteEligibilityClass,
    ToolSchemaPackRegistryEntry, ToolSchemaPackRegistrySupportSummary,
    PROVIDER_MODEL_REGISTRY_CLAIMED_SURFACE_RECORD_KIND,
    PROVIDER_MODEL_REGISTRY_EXTERNAL_TOOL_ENTRY_RECORD_KIND,
    PROVIDER_MODEL_REGISTRY_LOCAL_MODEL_PACK_ENTRY_RECORD_KIND,
    PROVIDER_MODEL_REGISTRY_MODEL_ENTRY_RECORD_KIND, PROVIDER_MODEL_REGISTRY_PACKET_RECORD_KIND,
    PROVIDER_MODEL_REGISTRY_PROMPT_PACK_ENTRY_RECORD_KIND,
    PROVIDER_MODEL_REGISTRY_PROVIDER_ENTRY_RECORD_KIND, PROVIDER_MODEL_REGISTRY_SCHEMA_VERSION,
    PROVIDER_MODEL_REGISTRY_SUPPORT_EXPORT_RECORD_KIND,
    PROVIDER_MODEL_REGISTRY_TOOL_SCHEMA_PACK_ENTRY_RECORD_KIND,
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
pub use run_history::{
    current_beta_ai_run_history_parity_packet, AiRerunReview, AiRunActorClass, AiRunCostBandClass,
    AiRunEvidenceLineage, AiRunExecutionBoundaryClass, AiRunHistoryActions,
    AiRunHistoryArtifactError, AiRunHistoryEntry, AiRunHistoryParityPacket,
    AiRunHistoryParityPacketInput, AiRunHistoryRedactionClass, AiRunHistoryStateClass,
    AiRunHistorySupportEntryRow, AiRunHistorySupportPacket, AiRunHistorySupportRerunRow,
    AiRunHistorySurfaceClass, AiRunHistorySurfaceRow, AiRunHistoryViolation, AiRunOutcomeClass,
    AiRunQuotaBandClass, AiRunThreadLineage, AiRunValidationOutcomeClass, ApprovalEventActorClass,
    ApprovalEventDecisionClass, ApprovalObjectClass, ApprovalScopeClass, ApprovalTimelineEvent,
    EvidenceCompletenessClass, EvidenceIncompletenessReasonClass, RerunActionOffer,
    RerunAdmissionClass, RerunApprovalResolution, RerunApprovalResolutionClass,
    RerunDeniedReasonClass, RerunDriftAxisClass, RerunDriftClass, RerunDriftRow,
    AI_RERUN_REVIEW_RECORD_KIND, AI_RERUN_REVIEW_SCHEMA_REF, AI_RUN_HISTORY_ENTRY_RECORD_KIND,
    AI_RUN_HISTORY_ENTRY_SCHEMA_REF, AI_RUN_HISTORY_FIXTURE_DIR,
    AI_RUN_HISTORY_PARITY_ARTIFACT_REF, AI_RUN_HISTORY_PARITY_PACKET_RECORD_KIND,
    AI_RUN_HISTORY_SCHEMA_VERSION, AI_RUN_HISTORY_SUPPORT_PACKET_RECORD_KIND,
    AI_RUN_HISTORY_SURFACE_ROW_RECORD_KIND,
};
pub use stabilize_ai_route_and_spend_truth::{
    current_stable_ai_route_spend_truth_export, AiActionFlowClass, AiRouteSpendTruthArtifactError,
    AiRouteSpendTruthPacket, AiRouteSpendTruthPacketInput, AiRouteSpendTruthViolation,
    CostMeasurementClass, CumulativeSpendPosture, LiveRunStrip, LocalResourceClass,
    LocalResourceCostRow, NonAiFallbackPath, PostRunReceipt, PreflightEstimateCard,
    QuotaSummaryRow, ResourceCostBandClass, RouteClass, RouteDecisionCauseClass,
    RouteDowngradeBanner, RouteRegistryResolution, RouteSpendEvidenceExport, RunOutcomeClass,
    RunPhaseClass, StableQualificationClass, AI_ROUTE_SPEND_TRUTH_AI_DOC_REF,
    AI_ROUTE_SPEND_TRUTH_ARTIFACT_REF, AI_ROUTE_SPEND_TRUTH_BUDGET_CONTRACT_REF,
    AI_ROUTE_SPEND_TRUTH_FIXTURE_DIR, AI_ROUTE_SPEND_TRUTH_RECEIPT_CONTRACT_REF,
    AI_ROUTE_SPEND_TRUTH_RECORD_KIND, AI_ROUTE_SPEND_TRUTH_SCHEMA_REF,
    AI_ROUTE_SPEND_TRUTH_SCHEMA_VERSION, AI_ROUTE_SPEND_TRUTH_SUMMARY_REF,
};
pub use stabilize_prompt_composer::{
    current_stable_prompt_composer_stabilization_export, AttachmentTaintClass, CompareAnswerRow,
    ComposerSurfaceClass, ContextDriftBanner, ContextParityClass, DriftSourceClass,
    ForkedThreadLineage, InclusionPostureClass, OmittedContextReviewRow, PinnedContextRow,
    PinnedFreshnessStateClass, PromptComposerStabilizationArtifactError,
    PromptComposerStabilizationInput, PromptComposerStabilizationPacket,
    PromptComposerStabilizationViolation, RememberPreview, RetentionLocusClass, ReuseAudienceClass,
    StableAttachmentSemanticRow, StableAttachmentSourceClass, SurfaceConsistencyRow,
    ThreadHeaderRow, ThreadRetentionModeClass, PROMPT_COMPOSER_STABILIZATION_AI_DOC_REF,
    PROMPT_COMPOSER_STABILIZATION_ARTIFACT_REF, PROMPT_COMPOSER_STABILIZATION_BASE_CONTRACT_REF,
    PROMPT_COMPOSER_STABILIZATION_BETA_ARTIFACT_REF, PROMPT_COMPOSER_STABILIZATION_FIXTURE_DIR,
    PROMPT_COMPOSER_STABILIZATION_RECORD_KIND, PROMPT_COMPOSER_STABILIZATION_SCHEMA_REF,
    PROMPT_COMPOSER_STABILIZATION_SCHEMA_VERSION, PROMPT_COMPOSER_STABILIZATION_SUMMARY_REF,
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
pub use tool_gateway::{
    current_beta_tool_gateway_conformance_packet, FirstUseReviewStateClass,
    ToolApprovalPostureClass, ToolAvailabilityStateClass, ToolCallClassificationStateClass,
    ToolCallOutcomeClass, ToolCallTaintPostureClass, ToolCallTimelineEntry, ToolCapabilityClass,
    ToolCredentialPostureClass, ToolGatewayArtifactError, ToolGatewayConformancePacket,
    ToolGatewayConformancePacketInput, ToolGatewayDescriptor, ToolGatewayLifecycleStateClass,
    ToolGatewaySurfaceClass, ToolGatewaySurfaceRow, ToolGatewayViolation, ToolNetworkBehaviorClass,
    ToolOutputTrustPostureClass, ToolPublisherSourceClass, ToolRuntimeBoundaryClass,
    ToolSideEffectClass, TOOL_CALL_TIMELINE_ENTRY_RECORD_KIND, TOOL_CALL_TIMELINE_ENTRY_SCHEMA_REF,
    TOOL_GATEWAY_CONFORMANCE_ARTIFACT_REF, TOOL_GATEWAY_CONFORMANCE_PACKET_RECORD_KIND,
    TOOL_GATEWAY_DESCRIPTOR_RECORD_KIND, TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF,
    TOOL_GATEWAY_FIXTURE_DIR, TOOL_GATEWAY_SCHEMA_VERSION, TOOL_GATEWAY_SURFACE_ROW_RECORD_KIND,
};
