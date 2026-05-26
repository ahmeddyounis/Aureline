//! Language-platform runtime foundations.
//!
//! This crate owns the first launch-language syntax substrate: a curated
//! Tree-sitter grammar registry plus a parser lifecycle that exposes startup,
//! parse, degraded, failure, and shutdown states as reusable records. Editor,
//! search, support, and future router surfaces should consume these records
//! rather than embedding grammar metadata or parser fallback rules privately.
//! Definition, reference, and rename-preview outputs project into the shared
//! [`aureline_navigation::target_model`] contract before crossing into UI,
//! CLI/headless, AI, review, or support evidence.

#![doc(html_root_url = "https://docs.rs/aureline-language/0.0.0")]

pub mod code_actions;
pub mod daily_driver_quality_truth_packet;
pub mod diagnostics;
pub mod invalidation;
pub mod lsp_router;
pub mod next_js_expert_workflow_pack_truth_packet;
pub mod packs;
pub mod provider_arbitration;
pub mod python;
pub mod react_expert_workflow_pack_truth_packet;
pub mod refactor_preview;
pub mod symbol_snapshot;
pub mod target_model;
pub mod tree_sitter;
pub mod tsjs;
pub mod vue_advanced_workflow_pack_truth_packet;

pub use daily_driver_quality_truth_packet::{
    current_stable_daily_driver_quality_truth_packet,
    ConsumerSurface as DailyDriverQualityConsumerSurface,
    DailyDriverConfidenceClass,
    DailyDriverQualityConsumerProjection, DailyDriverQualityRow,
    DailyDriverQualityTruthArtifactError, DailyDriverQualityTruthPacket,
    DailyDriverQualityTruthPacketInput, DailyDriverQualityTruthSupportExport,
    DailyDriverRowClass, DailyLoopStepClass,
    DowngradeAutomationClass as DailyDriverDowngradeAutomationClass,
    EvidenceClass as DailyDriverEvidenceClass,
    FindingKind as DailyDriverQualityFindingKind,
    FindingSeverity as DailyDriverQualityFindingSeverity,
    KnownLimitClass as DailyDriverKnownLimitClass, LanguageLaneClass,
    PromotionState as DailyDriverQualityPromotionState, SupportClass as DailyDriverSupportClass,
    ValidationFinding as DailyDriverQualityValidationFinding,
    DAILY_DRIVER_QUALITY_TRUTH_ARTIFACT_DOC_REF, DAILY_DRIVER_QUALITY_TRUTH_DOC_REF,
    DAILY_DRIVER_QUALITY_TRUTH_FIXTURE_DIR, DAILY_DRIVER_QUALITY_TRUTH_PACKET_ARTIFACT_REF,
    DAILY_DRIVER_QUALITY_TRUTH_PACKET_RECORD_KIND, DAILY_DRIVER_QUALITY_TRUTH_SCHEMA_REF,
    DAILY_DRIVER_QUALITY_TRUTH_SCHEMA_VERSION,
    DAILY_DRIVER_QUALITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use code_actions::{
    ActionClass as CodeActionClass, ApplyPostureClass as CodeActionApplyPostureClass,
    BlockingReasonClass as CodeActionBlockingReasonClass, CodeActionAdmissionRecord,
    CodeActionAlphaAggregateCounts, CodeActionAlphaSchemaVersion, CodeActionAlphaSnapshot,
    CodeActionCatalog, CodeActionContentIntegrityReview, CodeActionContractError,
    CodeActionEpochBinding, CodeActionEpochRoleClass, CodeActionFreshnessClass,
    CodeActionMutationCounts, CodeActionPolicyContext, CodeActionProviderDescriptor,
    CodeActionRecord, CodeActionRefactorScopeBinding, CodeActionSafetyClass,
    CodeActionScopeWideningReview, CodeActionSideEffectClass, CodeActionSnapshotRequest,
    CodeActionSourceKindClass, CodeActionSupportClass, CodeActionSurfaceClass,
    CodeActionSurfaceProjection, CodeActionTrustState, CodeActionUndoGroup,
    CodeActionValidationPlan, MutationScopeClass as CodeActionMutationScopeClass,
    PreviewRequirementClass as CodeActionPreviewRequirementClass,
    RefactorScopeAdmissionClass as CodeActionRefactorScopeAdmissionClass, RefactorScopeCandidate,
    RefactorScopeTargetRow, ReplayHintClass as CodeActionReplayHintClass,
    ScopeWideningReviewTriggerClass as CodeActionScopeWideningReviewTriggerClass,
    SemanticLayerStateClass as CodeActionSemanticLayerStateClass,
    UndoReversalClass as CodeActionUndoReversalClass,
    ValidationHintClass as CodeActionValidationHintClass, CODE_ACTION_ALPHA_SCHEMA_VERSION,
};
pub use diagnostics::{
    DiagnosticAnchor, DiagnosticAnchorRemapStateClass, DiagnosticBus, DiagnosticBusAggregateCounts,
    DiagnosticBusSchemaVersion, DiagnosticBusSnapshot, DiagnosticBusSnapshotRequest,
    DiagnosticEnvelope, DiagnosticEvidencePlaneClass, DiagnosticEvidenceRef,
    DiagnosticEvidenceRoleClass, DiagnosticFreshness, DiagnosticFreshnessClass,
    DiagnosticOriginClass, DiagnosticProviderAvailabilityRow, DiagnosticScope,
    DiagnosticSeverityClass, DiagnosticSourceDescriptor, DiagnosticSourceFamily,
    DiagnosticSurfaceClass, DiagnosticSurfaceProjection, DIAGNOSTIC_BUS_SCHEMA_VERSION,
};
pub use invalidation::{
    EditOperationRecord, EditWorkloadClass, IncrementalParseBuffer,
    IncrementalParseInvalidationRecord, IncrementalParseUpdate, InvalidationBenchmarkSample,
    InvalidationDecisionClass, InvalidationError, ParseInvalidationSchemaVersion, TextEdit,
};
pub use lsp_router::{
    CapabilityClass as RouterCapabilityClass, CompletenessClass as RouterCompletenessClass,
    CoordinateTranslationRequirementClass, DecisionOutcome as RouterDecisionOutcome,
    DegradedStateClass as RouterDegradedStateClass, EpochBinding as RouterEpochBinding,
    EpochRoleClass as RouterEpochRoleClass, FallbackClass as RouterFallbackClass,
    FaultDomainId as RouterFaultDomainId, FreshnessClass as RouterFreshnessClass,
    HealthState as RouterHealthState, LaneClass as RouterLaneClass, LanguageServerHostIdentity,
    LanguageServerHostStatus, LocalityClass as RouterLocalityClass, LspRouter,
    PlacementPreferenceClass, PrecedenceBand as RouterPrecedenceBand, ProviderFamily,
    ProviderKind as RouterProviderKind, ProviderPolicyContext, ProviderRoleClass,
    ProviderStackRow as RouterProviderStackRow, ProviderStatusRowRecord, RedactionClass,
    RequestedAuthorityFloorClass, RouterDecisionRecord, RouterDecisionSchemaVersion, RouterRequest,
    RouterRequestContext, RouterTrustState, RoutingContext,
    ScopeClaimClass as RouterScopeClaimClass, ScopeLimitClass, SupportClass as RouterSupportClass,
    SurfaceClass as RouterSurfaceClass, SurfaceReport as RouterSurfaceReport, SurfaceSupportClass,
    SurfaceSupportClassRow, WorkspaceLocalRouterRequest, ROUTER_DECISION_SCHEMA_VERSION,
};
pub use packs::{
    PythonServiceClaimDepthClass, PythonServiceDiagnosticsDefault, PythonServiceDocsPackRef,
    PythonServiceEnablementFlow, PythonServiceGitSurfaceClass, PythonServiceGitSurfaceRow,
    PythonServiceIconRow, PythonServiceKnownGapRow, PythonServiceLanguagePack,
    PythonServiceLanguagePackEnablementRequest, PythonServiceLanguagePackEnablementSnapshot,
    PythonServiceLanguagePackEnablementStateClass, PythonServiceLanguagePackManifest,
    PythonServiceLanguagePackSchemaVersion, PythonServiceLanguageRow,
    PythonServiceLanguageSupportClass, PythonServiceLaunchBundleReportRef,
    PythonServiceProviderRoute, PythonServiceToolHook, PythonServiceTrustAndIntegrityPolicy,
    TsJsWebClaimDepthClass, TsJsWebDiagnosticsDefault, TsJsWebDocsPackRef, TsJsWebEnablementFlow,
    TsJsWebIconRow, TsJsWebKnownGapRow, TsJsWebLanguagePack, TsJsWebLanguagePackEnablementRequest,
    TsJsWebLanguagePackEnablementSnapshot, TsJsWebLanguagePackEnablementStateClass,
    TsJsWebLanguagePackManifest, TsJsWebLanguagePackSchemaVersion, TsJsWebLanguageRow,
    TsJsWebLanguageSupportClass, TsJsWebProviderRoute, TsJsWebToolHook,
    TsJsWebTrustAndIntegrityPolicy, PYTHON_SERVICE_LANGUAGE_PACK_SCHEMA_VERSION,
    TSJS_WEB_LANGUAGE_PACK_SCHEMA_VERSION,
};
pub use provider_arbitration::{
    build_downgraded_semantic_claims_matrix, classify_claim_status, classify_proof_scenario,
    current_provider_arbitration_corpus, current_provider_arbitration_fixture_refs,
    current_provider_arbitration_proof_corpus, current_provider_arbitration_proof_fixture_refs,
    load_provider_arbitration_case, ApplyGateClass as ArbitrationApplyGateClass,
    ArbitrationCompletenessClass, ArbitrationCorpusValidationDefect,
    ArbitrationCorpusValidationReport, ArbitrationDecisionAggregateCounts,
    ArbitrationDecisionRecord, ArbitrationDecisionReportRow, ArbitrationDecisionSchemaVersion,
    ArbitrationEpochBinding, ArbitrationEpochRoleClass, ArbitrationFreshnessClass,
    ArbitrationHealthState, ArbitrationInspector, ArbitrationInspectorBetaReport,
    ArbitrationLocalityClass, ArbitrationPolicyContext, ArbitrationRedactionClass,
    ArbitrationScopeClaimClass, ArbitrationTrustState, ClaimStatusClass, ConfidenceOutcomeClass,
    ConflictClass as ArbitrationConflictClass, ConsumerRoutingRow, ConsumerSurfaceClass,
    DisagreementBlock, DisagreementVisibilityClass, DowngradedPromiseBlock,
    DowngradedPromiseReasonClass, DowngradedSemanticClaimRow, DowngradedSemanticClaimsMatrix,
    FallbackLabelClass, FaultDomainClass, IsolateActionClass, LaneSupportClass, LaneSupportRow,
    LanguageActionLaneClass, LinkedRecordRefs, ProofScenarioClass, ProviderArbitrationCorpus,
    ProviderArbitrationCorpusEntry, ProviderFamily as ArbitrationProviderFamily,
    ProviderHealthStateRecord, ProviderHealthStateSchemaVersion, ProviderOrderRow,
    ProviderRoleClass as ArbitrationProviderRoleClass, RecoveryHintClass,
    RequestedAuthorityFloorClass as ArbitrationRequestedAuthorityFloorClass, RetryActionClass,
    RetryIsolateControls, ARBITRATION_DECISION_RECORD_KIND, ARBITRATION_DECISION_SCHEMA_REF,
    ARBITRATION_DECISION_SCHEMA_VERSION, DOWNGRADED_SEMANTIC_CLAIMS_MATRIX_ARTIFACT_REF,
    DOWNGRADED_SEMANTIC_CLAIMS_MATRIX_RECORD_KIND, DOWNGRADED_SEMANTIC_CLAIMS_MATRIX_ROW_RECORD_KIND,
    PROVIDER_ARBITRATION_BETA_DOC_REF, PROVIDER_ARBITRATION_BETA_REPORT_RECORD_KIND,
    PROVIDER_ARBITRATION_CLAIM_QUALIFICATION_DOC_REF, PROVIDER_ARBITRATION_CORPUS_DIR,
    PROVIDER_ARBITRATION_PROOF_CORPUS_DIR, PROVIDER_ARBITRATION_PROOF_REPORT_ARTIFACT_REF,
    PROVIDER_ARBITRATION_PROOF_REPORT_RECORD_KIND, PROVIDER_HEALTH_STATE_RECORD_KIND,
    PROVIDER_HEALTH_STATE_SCHEMA_REF, PROVIDER_HEALTH_STATE_SCHEMA_VERSION,
};
pub use next_js_expert_workflow_pack_truth_packet::{
    current_stable_next_js_expert_workflow_pack_truth_packet,
    ConsumerSurface as NextJsExpertWorkflowPackConsumerSurface,
    DowngradeAutomationClass as NextJsExpertWorkflowPackDowngradeAutomationClass,
    EvidenceClass as NextJsExpertWorkflowPackEvidenceClass,
    FindingKind as NextJsExpertWorkflowPackFindingKind,
    FindingSeverity as NextJsExpertWorkflowPackFindingSeverity,
    KnownLimitClass as NextJsExpertWorkflowPackKnownLimitClass,
    NextJsExpertWorkflowPackConsumerProjection, NextJsExpertWorkflowPackTruthArtifactError,
    NextJsExpertWorkflowPackTruthPacket, NextJsExpertWorkflowPackTruthPacketInput,
    NextJsExpertWorkflowPackTruthSupportExport,
    PromotionState as NextJsExpertWorkflowPackPromotionState,
    SupportClass as NextJsExpertWorkflowPackSupportClass,
    ValidationFinding as NextJsExpertWorkflowPackValidationFinding,
    WorkflowLoopClass as NextJsExpertWorkflowLoopClass,
    WorkflowPackClass as NextJsExpertWorkflowPackClass,
    WorkflowPackConfidenceClass as NextJsExpertWorkflowPackConfidenceClass,
    WorkflowPackRow as NextJsExpertWorkflowPackRow,
    WorkflowPackRowClass as NextJsExpertWorkflowPackRowClass,
    NEXT_JS_EXPERT_WORKFLOW_PACK_TRUTH_ARTIFACT_DOC_REF,
    NEXT_JS_EXPERT_WORKFLOW_PACK_TRUTH_DOC_REF,
    NEXT_JS_EXPERT_WORKFLOW_PACK_TRUTH_FIXTURE_DIR,
    NEXT_JS_EXPERT_WORKFLOW_PACK_TRUTH_PACKET_ARTIFACT_REF,
    NEXT_JS_EXPERT_WORKFLOW_PACK_TRUTH_PACKET_RECORD_KIND,
    NEXT_JS_EXPERT_WORKFLOW_PACK_TRUTH_SCHEMA_REF,
    NEXT_JS_EXPERT_WORKFLOW_PACK_TRUTH_SCHEMA_VERSION,
    NEXT_JS_EXPERT_WORKFLOW_PACK_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use react_expert_workflow_pack_truth_packet::{
    current_stable_react_expert_workflow_pack_truth_packet,
    ConsumerSurface as ReactExpertWorkflowPackConsumerSurface,
    DowngradeAutomationClass as ReactExpertWorkflowPackDowngradeAutomationClass,
    EvidenceClass as ReactExpertWorkflowPackEvidenceClass,
    FindingKind as ReactExpertWorkflowPackFindingKind,
    FindingSeverity as ReactExpertWorkflowPackFindingSeverity,
    KnownLimitClass as ReactExpertWorkflowPackKnownLimitClass,
    PromotionState as ReactExpertWorkflowPackPromotionState,
    ReactExpertWorkflowPackConsumerProjection, ReactExpertWorkflowPackTruthArtifactError,
    ReactExpertWorkflowPackTruthPacket, ReactExpertWorkflowPackTruthPacketInput,
    ReactExpertWorkflowPackTruthSupportExport, SupportClass as ReactExpertWorkflowPackSupportClass,
    ValidationFinding as ReactExpertWorkflowPackValidationFinding, WorkflowLoopClass,
    WorkflowPackClass, WorkflowPackConfidenceClass, WorkflowPackRow, WorkflowPackRowClass,
    REACT_EXPERT_WORKFLOW_PACK_TRUTH_ARTIFACT_DOC_REF,
    REACT_EXPERT_WORKFLOW_PACK_TRUTH_DOC_REF, REACT_EXPERT_WORKFLOW_PACK_TRUTH_FIXTURE_DIR,
    REACT_EXPERT_WORKFLOW_PACK_TRUTH_PACKET_ARTIFACT_REF,
    REACT_EXPERT_WORKFLOW_PACK_TRUTH_PACKET_RECORD_KIND,
    REACT_EXPERT_WORKFLOW_PACK_TRUTH_SCHEMA_REF, REACT_EXPERT_WORKFLOW_PACK_TRUTH_SCHEMA_VERSION,
    REACT_EXPERT_WORKFLOW_PACK_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use python::{
    PythonAccessKindClass, PythonAmbiguityDescriptor, PythonAnchorRef, PythonAnswerLayerClass,
    PythonApplyPostureClass, PythonCheckpointClass, PythonCompletenessClass,
    PythonEnvironmentManagerClass, PythonGeneratedOrExternalStateClass, PythonHoverRecord,
    PythonInlineVisibilityClass, PythonInterpreterContext, PythonInterpreterReadinessClass,
    PythonInterpreterSelectionStateClass, PythonLaunchWedge, PythonLaunchWedgeSnapshot,
    PythonNavigationError, PythonOccurrenceSeed, PythonProviderSnapshot, PythonQualityActionClass,
    PythonQualityAggregateCounts, PythonQualityAlphaSchemaVersion, PythonQualityDiagnosticSeed,
    PythonQualityExecutionPlaneProjection, PythonQualityExecutionTaskHook,
    PythonQualityPreviewRequirementClass, PythonQualityRerunPostureClass, PythonQualitySafetyClass,
    PythonQualitySeedSnapshot, PythonQualitySnapshot, PythonQualitySnapshotRequest,
    PythonQualityTaskHookSeed, PythonQualityToolKindClass, PythonQualityToolStatusRow,
    PythonQualityTriggerClass, PythonQualityWedge, PythonReferenceCountSummary,
    PythonReferenceSetRecord, PythonRelationClass, PythonRenameAffectedScopeRow,
    PythonRenameCheckpointDescriptor, PythonRenameCountSummary, PythonRenameCoverageLimitClass,
    PythonRenameEvidenceBinding, PythonRenamePreviewCompletenessClass, PythonRenamePreviewRecord,
    PythonRenamePreviewSchemaVersion, PythonRenameWarningClass, PythonRenameWarningRow,
    PythonResultConfidenceClass, PythonRollbackPathClass, PythonScopeDescriptor,
    PythonSemanticEvidenceBinding, PythonSemanticResultIdentityClass, PythonSemanticResultRecord,
    PythonSemanticResultSchemaVersion, PythonSourceAnchor, PythonSourceAnchorKindClass,
    PythonSymbolKindClass, PythonSymbolSeed, PythonWorkspaceContext,
    PYTHON_NAV_ALPHA_SCHEMA_VERSION, PYTHON_QUALITY_ALPHA_SCHEMA_VERSION,
};
pub use refactor_preview::{
    current_refactor_preview_corpus, current_refactor_preview_fixture_refs,
    load_refactor_preview_case, GeneratedArtifactPostureClass, GroupedMutationLineageClass,
    RefactorApplyPostureClass, RefactorConfidenceClass, RefactorCorpusRowState,
    RefactorDependencyImpactClass, RefactorEpochBinding, RefactorEpochRoleClass,
    RefactorEvidenceBinding, RefactorFallbackLabel, RefactorFallbackReasonClass,
    RefactorGeneratedDependencyNote, RefactorPolicyContext, RefactorPreviewAggregateCounts,
    RefactorPreviewBetaReport, RefactorPreviewCorpus, RefactorPreviewCorpusEntry,
    RefactorPreviewCorpusValidationReport, RefactorPreviewEvaluator, RefactorPreviewRecord,
    RefactorPreviewReportRow, RefactorPreviewSchemaVersion, RefactorPreviewValidationDefect,
    RefactorRedactionClass, RefactorRollbackDrillOutcomeClass, RefactorRollbackHandle,
    RefactorRollbackPathClass, RefactorRuntimeConditionClass, RefactorSemanticSourceClass,
    RefactorSupportClaimClass, RefactorTargetSet, RefactorTrustState, RefactorValidationCheckClass,
    RefactorValidationFinding, RefactorValidationHookClass, RefactorValidationResult,
    RefactorValidationResultSchemaVersion, RefactorValidationSeverityClass,
    RefactorValidationStateClass, REFACTOR_PREVIEW_BETA_DOC_REF,
    REFACTOR_PREVIEW_BETA_REPORT_RECORD_KIND, REFACTOR_PREVIEW_CORPUS_DIR,
    REFACTOR_PREVIEW_RECORD_KIND, REFACTOR_PREVIEW_SCHEMA_REF, REFACTOR_PREVIEW_SCHEMA_VERSION,
    REFACTOR_VALIDATION_RESULT_RECORD_KIND, REFACTOR_VALIDATION_RESULT_SCHEMA_REF,
    REFACTOR_VALIDATION_RESULT_SCHEMA_VERSION,
};
pub use symbol_snapshot::{
    SourcePoint, SourceRange, SymbolKindClass, SymbolProviderClass, SymbolRecord,
    SymbolSnapshotCompletenessClass, SymbolSnapshotExportRequest, SymbolSnapshotExporter,
    SymbolSnapshotRecord, SymbolSnapshotSchemaVersion, SymbolSnapshotState,
};
pub use target_model::{
    python_navigation_target, python_reference_occurrences, python_rename_preview_set,
    tsjs_navigation_target, tsjs_reference_occurrences, tsjs_rename_preview_set,
};
pub use tsjs::{
    TsJsAccessKindClass, TsJsAmbiguityDescriptor, TsJsAnchorRef, TsJsAnswerLayerClass,
    TsJsApplyPostureClass, TsJsCheckpointClass, TsJsCompletenessClass,
    TsJsGeneratedOrExternalStateClass, TsJsHoverRecord, TsJsInlineVisibilityClass, TsJsLaunchWedge,
    TsJsLaunchWedgeSnapshot, TsJsNavigationError, TsJsOccurrenceSeed, TsJsProviderSnapshot,
    TsJsQualityActionClass, TsJsQualityAggregateCounts, TsJsQualityAlphaSchemaVersion,
    TsJsQualityDiagnosticSeed, TsJsQualityExecutionPlaneProjection, TsJsQualityExecutionTaskHook,
    TsJsQualityPreviewRequirementClass, TsJsQualityRerunPostureClass, TsJsQualitySafetyClass,
    TsJsQualitySeedSnapshot, TsJsQualitySnapshot, TsJsQualitySnapshotRequest,
    TsJsQualityTaskHookSeed, TsJsQualityToolKindClass, TsJsQualityToolStatusRow,
    TsJsQualityTriggerClass, TsJsQualityWedge, TsJsReferenceCountSummary, TsJsReferenceSetRecord,
    TsJsRelationClass, TsJsRenameAffectedScopeRow, TsJsRenameCheckpointDescriptor,
    TsJsRenameCountSummary, TsJsRenameCoverageLimitClass, TsJsRenameEvidenceBinding,
    TsJsRenamePreviewCompletenessClass, TsJsRenamePreviewRecord, TsJsRenamePreviewSchemaVersion,
    TsJsRenameWarningClass, TsJsRenameWarningRow, TsJsResultConfidenceClass, TsJsRollbackPathClass,
    TsJsScopeDescriptor, TsJsSemanticEvidenceBinding, TsJsSemanticResultIdentityClass,
    TsJsSemanticResultRecord, TsJsSemanticResultSchemaVersion, TsJsSourceAnchor,
    TsJsSourceAnchorKindClass, TsJsSymbolKindClass, TsJsSymbolSeed, TsJsWorkspaceContext,
    TSJS_NAV_ALPHA_SCHEMA_VERSION, TSJS_QUALITY_ALPHA_SCHEMA_VERSION,
};

pub use vue_advanced_workflow_pack_truth_packet::{
    current_stable_vue_advanced_workflow_pack_truth_packet,
    ConsumerSurface as VueAdvancedWorkflowPackConsumerSurface,
    DowngradeAutomationClass as VueAdvancedWorkflowPackDowngradeAutomationClass,
    EvidenceClass as VueAdvancedWorkflowPackEvidenceClass,
    FindingKind as VueAdvancedWorkflowPackFindingKind,
    FindingSeverity as VueAdvancedWorkflowPackFindingSeverity,
    KnownLimitClass as VueAdvancedWorkflowPackKnownLimitClass,
    PromotionState as VueAdvancedWorkflowPackPromotionState,
    SupportClass as VueAdvancedWorkflowPackSupportClass,
    ValidationFinding as VueAdvancedWorkflowPackValidationFinding,
    VueAdvancedWorkflowPackConsumerProjection, VueAdvancedWorkflowPackTruthArtifactError,
    VueAdvancedWorkflowPackTruthPacket, VueAdvancedWorkflowPackTruthPacketInput,
    VueAdvancedWorkflowPackTruthSupportExport,
    WorkflowLoopClass as VueAdvancedWorkflowLoopClass,
    WorkflowPackClass as VueAdvancedWorkflowPackClass,
    WorkflowPackConfidenceClass as VueAdvancedWorkflowPackConfidenceClass,
    WorkflowPackRow as VueAdvancedWorkflowPackRow,
    WorkflowPackRowClass as VueAdvancedWorkflowPackRowClass,
    VUE_ADVANCED_WORKFLOW_PACK_TRUTH_ARTIFACT_DOC_REF,
    VUE_ADVANCED_WORKFLOW_PACK_TRUTH_DOC_REF, VUE_ADVANCED_WORKFLOW_PACK_TRUTH_FIXTURE_DIR,
    VUE_ADVANCED_WORKFLOW_PACK_TRUTH_PACKET_ARTIFACT_REF,
    VUE_ADVANCED_WORKFLOW_PACK_TRUTH_PACKET_RECORD_KIND,
    VUE_ADVANCED_WORKFLOW_PACK_TRUTH_SCHEMA_REF, VUE_ADVANCED_WORKFLOW_PACK_TRUTH_SCHEMA_VERSION,
    VUE_ADVANCED_WORKFLOW_PACK_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};

pub use aureline_navigation::target_model as navigation_target_model;
pub use tree_sitter::{
    default_launch_grammar_registry, BudgetPolicyClass, BufferRef, CacheRecord, CacheStatusClass,
    DerivedCueClass, DerivedCuePostureClass, DerivedCueRecord, EpochBinding, EpochRoleClass,
    ExportPolicy, ExportPolicyClass, FailureReasonClass, GrammarDescriptor, GrammarRegistryEntry,
    GrammarRegistryError, GrammarRegistryRecord, GrammarResolution, GrammarResolutionStateClass,
    GrammarSourceClass, IncrementalBudget, ParseCacheContext, ParseFreshnessClass,
    ParseLifecycleStateClass, ParseOutput, ParseQualityClass, ParseRequest, ParseRequestClass,
    ParseSessionRecord, ParseSessionSchemaVersion, ParseState, ParserHost, ParserHostClass,
    ParserLifecycleSnapshot, ParserRuntimeHandle, ParserRuntimeStateClass, ParserStartupError,
    ParserSubstrateClass, SyntaxTreeIdentity, TreeSitterGrammarRegistry,
    TreeSitterParserSupervisor, TrustState, TREE_SITTER_GRAMMAR_REGISTRY_SCHEMA_VERSION,
};
