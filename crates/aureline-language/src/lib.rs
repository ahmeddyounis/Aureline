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
pub mod diagnostics;
pub mod invalidation;
pub mod lsp_router;
pub mod packs;
pub mod python;
pub mod symbol_snapshot;
pub mod target_model;
pub mod tree_sitter;
pub mod tsjs;

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
