//! Execution-context object model and resolver seed.
//!
//! This crate owns inspectable execution-context and task-event runtime
//! contracts. It exposes one [`execution_context::ExecutionContext`] object, a
//! small [`execution_context::ExecutionContextResolver`] that mints contexts
//! for launch-capable surfaces, a canonical [`tasks::TaskEventStream`] for
//! task/test/debug lifecycle truth, and [`tests::TestAttemptAlphaPacket`] for
//! launch-wedge test identity, session, attempt, watch, and imported-CI truth.
//! [`managed_alpha::ManagedWorkspaceAlphaRecord`] adds the bounded
//! managed-workspace suspend/resume/reattach inspection lane. Downstream event
//! and export lanes carry
//! [`provenance::ExecutionEventProvenance`] so target truth survives after the
//! live run surface is gone.
//!
//! Surfaces (terminal pane, task seed, debug-prep seed, provider/auth entry
//! points, activity center, status bar, support / export flows) read structured
//! execution-context records through this crate; they do not derive runtime
//! truth from terminal state alone or fork local copies of host / target /
//! toolchain identity.
//!
//! [`testing_triage::TestTrustPacket`] composes the beta test-runner and
//! test-quality projections into release-visible watch, flaky, snapshot, and
//! quarantine debt summaries without re-parsing raw runner output.
//!
//! The reviewer-facing landing page is
//! [`/docs/runtime/execution_context_seed.md`](../../../docs/runtime/execution_context_seed.md).
//! The cross-tool boundary schema is
//! [`/schemas/runtime/execution_context.schema.json`](../../../schemas/runtime/execution_context.schema.json).

#![doc(html_root_url = "https://docs.rs/aureline-runtime/0.0.0")]

pub mod build_intelligence;
pub mod capability_negotiation;
pub mod capsule_resolver;
pub mod debug;
pub mod dependencies;
pub mod detectors;
pub mod diagnostics;
pub mod discovery;
pub mod drift_repair;
pub mod env_inspect;
pub mod execution_context;
pub mod finalize_environment_and_toolchain_manager_parity_across_ui;
pub mod harden_breakpoint_call_stack_variables_watch_evaluate_and;
pub mod harden_coverage_flaky_test_snapshot_golden_and_baseline;
pub mod harden_environment_capsule_resolution;
pub mod host_boundary;
pub mod language_hosts;
pub mod launch_profiles;
pub mod managed_alpha;
pub mod managed_workspace_lifecycle_beta;
pub mod packages;
pub mod preview_drift;
pub mod provenance;
pub mod quality;
pub mod recipes;
pub mod remote_helper_skew_beta;
pub mod request_workspace;
pub mod request_workspace_contracts;
pub mod rerun;
pub mod resource_governor;
pub mod run_lineage;
pub mod scanner_import;
pub mod shared_debug_alpha;
pub mod shared_terminal_alpha;
pub mod stabilize_debugger_host_and_adapter_negotiation;
pub mod stabilize_execution_context_resolver;
pub mod stabilize_task_discovery_launch_profiles_rerun_last_behavior;
pub mod stabilize_the_test_explorer_inline_results_watch_mode;
pub mod support_matrix_beta;
pub mod target_discovery;
pub mod targets;
pub mod task_events;
pub mod tasks;
pub mod testing;
pub mod testing_identity;
pub mod testing_quality;
pub mod testing_triage;
pub mod tests;
pub mod topology_inspector;
pub mod trace_replay_alpha;

pub use build_intelligence::{
    AdapterHealthReason, AdapterHealthState, AdapterHealthStrip, AdapterIdentity,
    ArtifactSourceClass, BuildIntelligenceAction, BuildIntelligenceActionClass,
    BuildIntelligenceCoverageManifest, BuildIntelligenceLaneType, BuildIntelligenceReceipt,
    BuildIntelligenceRunConfigCard, BuildIntelligenceSupportExport, BuildIntelligenceTargetRow,
    DiscoveryDiffChangeClass, DiscoveryDiffItem, DiscoveryDiffReview, HighTrustActionPosture,
    ImportedLiveState, RefreshLineage, TargetExactnessStatus, ADAPTER_HEALTH_STRIP_RECORD_KIND,
    BUILD_INTELLIGENCE_COVERAGE_MANIFEST_RECORD_KIND, BUILD_INTELLIGENCE_RECEIPT_RECORD_KIND,
    BUILD_INTELLIGENCE_RUN_CONFIG_CARD_RECORD_KIND, BUILD_INTELLIGENCE_SCHEMA_VERSION,
    BUILD_INTELLIGENCE_SUPPORT_EXPORT_RECORD_KIND, BUILD_INTELLIGENCE_TARGET_ROW_RECORD_KIND,
    DISCOVERY_DIFF_REVIEW_RECORD_KIND,
};
pub use capability_negotiation::{
    CapabilityEffectClass, CapabilityNegotiationParseError, CapabilityRequirementClass,
    CompatibilityWindow, CompatibilityWindowStatus, DroppedHelperCapability,
    EffectiveCapabilityPosture, HelperCapabilityRequest, HelperCapabilityRequirement,
    HelperCapabilityResponse, MissingCapabilityReasonClass, NegotiationOutcome,
    HELPER_CAPABILITY_NEGOTIATION_SCHEMA_VERSION,
};
pub use capsule_resolver::beta::{
    evaluate_capsule_drift, CapsuleBetaDriftOutcome, CapsuleBetaDriftRow, CapsuleBetaParsedFields,
    CapsuleBetaPrecedenceRow, CapsuleBetaSourceBaseline, CapsuleBetaSourceClass,
    CapsuleBetaSourceConfidence, CapsuleBetaSourceCoverageRow, CapsuleBetaSourceNote,
    CapsuleBetaSourceParse, ComposeParsedFields, DevcontainerParsedFields,
    EnvironmentCapsuleBetaCoverageManifest, EnvironmentCapsuleBetaDriftEvaluation,
    EnvironmentCapsuleBetaResolution, EnvironmentCapsuleBetaResolver,
    EnvironmentCapsuleBetaResolverConfig, EnvironmentCapsuleBetaSupportExport, NixParsedFields,
    NodeParsedFields, PythonParsedFields, ENVIRONMENT_CAPSULE_BETA_COVERAGE_MANIFEST_RECORD_KIND,
    ENVIRONMENT_CAPSULE_BETA_DRIFT_RECORD_KIND, ENVIRONMENT_CAPSULE_BETA_RESOLUTION_RECORD_KIND,
    ENVIRONMENT_CAPSULE_BETA_RESOLVER_VERSION, ENVIRONMENT_CAPSULE_BETA_SCHEMA_VERSION,
    ENVIRONMENT_CAPSULE_BETA_SUPPORT_EXPORT_RECORD_KIND,
};
pub use capsule_resolver::{
    EnvironmentCapsuleHint, EnvironmentCapsuleResolution, EnvironmentCapsuleResolver,
    EnvironmentCapsuleResolverConfig, PrebuildFingerprintStub, ProjectArchetypeHint,
    ENVIRONMENT_CAPSULE_RESOLUTION_RECORD_KIND, ENVIRONMENT_CAPSULE_RESOLUTION_SCHEMA_VERSION,
    ENVIRONMENT_CAPSULE_RESOLVER_VERSION, PREBUILD_FINGERPRINT_STUB_RECORD_KIND,
};
pub use debug::{
    DapHostSupervisor, DapHostSupervisorConfig, DapHostSupervisorError,
    DebugAdapterCapabilityClass, DebugAdapterCapabilityRequest, DebugAdapterCapabilityResponse,
    DebugAdapterIdentity, DebugAdapterNegotiationInput, DebugAdapterNegotiationOutcome,
    DebugAdapterTransportClass, DebugSessionEventClass, DebugSessionExitReasonClass,
    DebugSessionIdentity, DebugSessionLaunchSpec, DebugSessionLifecycleEvent, DebugSessionMode,
    DebugSessionRestartCause, DebugSessionSnapshot, DebugSessionStateClass,
    DebugSessionSupportPacket, DebugSessionTargetIdentity, DEBUG_SESSION_EVENT_RECORD_KIND,
    DEBUG_SESSION_LIFECYCLE_SCHEMA_VERSION, DEBUG_SESSION_RECORD_KIND,
    DEBUG_SESSION_SUPPORT_PACKET_RECORD_KIND,
};
pub use dependencies::{
    manifest_delta_token, validation_task_tokens, AdvisoryAffectedRange, AdvisoryLifecycleClass,
    AdvisorySeverityClass, AdvisorySourceClass, AdvisoryTruthClass, DebtReleaseVisibilityClass,
    DependencyAdvisoryRecord, DependencyAdvisoryRecordSeed, DependencyDebtKindClass,
    DependencyDebtPacket, DependencyDebtPacketSeed, DependencyDebtRow, DependencyEdgeRecord,
    DependencyFreshnessClass, DependencyGraphRecord, DependencyIntelligenceViolation,
    DependencyProvenanceClass, DependencyRecord, DependencyRecordSeed, DependencyRelationshipClass,
    DependencyResolutionClass, DependencySourceClass, LicenseDecisionClass,
    LockfileMutationPreview, LockfilePreviewActionClass, LockfilePreviewOutcomeClass,
    SuppressionRef, SuppressionStateClass, DEPENDENCY_ADVISORY_RECORD_KIND,
    DEPENDENCY_DEBT_PACKET_RECORD_KIND, DEPENDENCY_GRAPH_RECORD_KIND,
    DEPENDENCY_INTELLIGENCE_REVIEWER_VERSION, DEPENDENCY_INTELLIGENCE_SCHEMA_VERSION,
    DEPENDENCY_RECORD_KIND, LOCKFILE_MUTATION_PREVIEW_RECORD_KIND,
};
pub use detectors::node::{
    NodePackageManagerKind, NodePackageManagerRequirement, NodePackageManagerResolution,
    NodeRuntimeResolution, NodeToolchainAmbiguity, NodeToolchainDetection, NodeToolchainDetector,
    NodeToolchainDetectorConfig, NodeToolchainFallbackPath, NodeToolchainProvenanceCard,
    NodeToolchainProvenanceDisposition, NodeToolchainResolutionState, NodeToolchainSourceKind,
    NodeToolchainSubject, NODE_TOOLCHAIN_DETECTION_RECORD_KIND,
    NODE_TOOLCHAIN_DETECTION_SCHEMA_VERSION, NODE_TOOLCHAIN_DETECTOR_VERSION,
};
pub use detectors::python::{
    PythonEnvironmentAmbiguity, PythonEnvironmentDetection, PythonEnvironmentDetector,
    PythonEnvironmentDetectorConfig, PythonEnvironmentFallbackPath, PythonEnvironmentManagerKind,
    PythonEnvironmentManagerRequirement, PythonEnvironmentManagerResolution,
    PythonEnvironmentProvenanceCard, PythonEnvironmentProvenanceDisposition,
    PythonEnvironmentResolutionState, PythonEnvironmentSourceKind, PythonEnvironmentSubject,
    PythonInterpreterResolution, PYTHON_ENVIRONMENT_DETECTION_RECORD_KIND,
    PYTHON_ENVIRONMENT_DETECTION_SCHEMA_VERSION, PYTHON_ENVIRONMENT_DETECTOR_VERSION,
};
pub use discovery::package_scripts::{
    PackageScriptBlockReason, PackageScriptDescriptor, PackageScriptDiscoverer,
    PackageScriptDiscovererConfig, PackageScriptDiscovery, PackageScriptDiscoveryState,
    PackageScriptDispatch, PackageScriptLaunchReadiness, PackageScriptLifecycleHook,
    PackageScriptMissingRuntimeState, PackageScriptRerunLineage, PackageScriptRerunMode,
    PackageScriptRunContract, PackageScriptRunner, PackageScriptRuntimeStatus,
    PackageScriptShellMode, PackageScriptSource, PackageScriptSourceKind,
    PackageScriptWarningClass, PACKAGE_SCRIPT_DISCOVERER_VERSION,
    PACKAGE_SCRIPT_DISCOVERY_RECORD_KIND, PACKAGE_SCRIPT_DISCOVERY_SCHEMA_VERSION,
    PACKAGE_SCRIPT_RUN_CONTRACT_RECORD_KIND,
};
pub use discovery::pytest::{
    PytestBlockReason, PytestDiscoverer, PytestDiscovererConfig, PytestDiscovery,
    PytestDiscoveryIssue, PytestDiscoveryIssueKind, PytestDiscoveryState, PytestDispatch,
    PytestInvocationMode, PytestLaunchReadiness, PytestMissingRuntimeState, PytestRerunLineage,
    PytestRerunMode, PytestRunContract, PytestRunSelection, PytestRunner, PytestRuntimeStatus,
    PytestSelectionKind, PytestSourceKind, PytestTestDescriptor, PytestTestFileDescriptor,
    PytestTestKind, PytestWarningClass, PYTEST_DISCOVERER_VERSION, PYTEST_DISCOVERY_RECORD_KIND,
    PYTEST_DISCOVERY_SCHEMA_VERSION, PYTEST_RUN_CONTRACT_RECORD_KIND,
};
pub use discovery::toolchains::{
    ToolchainDetectionEntry, ToolchainDetectionEvidence, ToolchainDetectionSourceKind,
    ToolchainPresenceState, WorkspaceToolchainDetector, WorkspaceToolchainDetectorConfig,
    WorkspaceToolchainDiscovery, WorkspaceToolchainKind, WORKSPACE_TOOLCHAIN_DETECTOR_VERSION,
    WORKSPACE_TOOLCHAIN_DISCOVERY_RECORD_KIND, WORKSPACE_TOOLCHAIN_DISCOVERY_SCHEMA_VERSION,
};
pub use drift_repair::{
    DriftReasonClass, DriftRepairAction, DriftRepairActionClass, DriftRepairAuthorityImpactClass,
    RemoteDriftRepairDiagnosticsPacket, RemoteDriftRepairGuidance,
    REMOTE_DRIFT_REPAIR_BETA_DIAGNOSTICS_PACKET_RECORD_KIND,
    REMOTE_DRIFT_REPAIR_BETA_GUIDANCE_RECORD_KIND, REMOTE_DRIFT_REPAIR_BETA_SCHEMA_VERSION,
};
pub use env_inspect::{
    seeded_env_inspect_resolver, seeded_env_inspect_snapshot, seeded_env_inspect_support_export,
    EnvInspectCoreField, EnvInspectDegradationLabel, EnvInspectDegradationSeverity,
    EnvInspectRedactionClass, EnvInspectSection, EnvInspectSeededScenario, EnvInspectSnapshot,
    EnvInspectSupportExport, ENV_INSPECT_SCHEMA_VERSION, ENV_INSPECT_SNAPSHOT_RECORD_KIND,
    ENV_INSPECT_SUPPORT_EXPORT_RECORD_KIND,
};
pub use execution_context::beta::{
    evaluate_ticket_drift, lane_for_context, lane_for_target_class,
    ExecutionContextBetaCoverageManifest, ExecutionContextBetaLane,
    ExecutionContextBetaLaneCoverageRow, ExecutionContextBetaLaneSample,
    ExecutionContextBetaSupportExport, TicketDriftBinding, TicketDriftEvaluation, TicketDriftField,
    TicketDriftOutcome, TicketDriftRow, EXECUTION_CONTEXT_BETA_COVERAGE_MANIFEST_RECORD_KIND,
    EXECUTION_CONTEXT_BETA_SCHEMA_VERSION, EXECUTION_CONTEXT_BETA_SUPPORT_EXPORT_RECORD_KIND,
    EXECUTION_CONTEXT_TICKET_DRIFT_RECORD_KIND,
};
pub use execution_context::{
    ActorClass, CacheDisposition, CapsuleDriftState, ConfidenceLevel, DegradedFieldReason,
    DegradedFieldRecord, EnvironmentCapsuleRef, ExecutionContext, ExecutionContextEffectClass,
    ExecutionContextExplanation, ExecutionContextReasonCode, ExecutionContextReasonSource,
    ExecutionContextRequest, ExecutionContextResolver, ExecutionContextResolverConfig,
    ExecutionRouteClass, ExecutionRouteOrigin, IdentityMode, InvocationSubject, MixedVersionDrift,
    MixedVersionDriftState, MixedVersionReason, PolicyAndTrust, PrebuildInvalidationReason,
    PrebuildMetadata, PrebuildReuseState, Provenance, ReachabilityState, ResolverInputDecision,
    ResolverInputField, ResolverInputSource, ScopeClass, SurfaceClass, TargetClass,
    TargetConfidence, TargetConfidenceReason, TargetIdentity, ToolchainClass, ToolchainIdentity,
    TrustState, EXECUTION_CONTEXT_RECORD_KIND, EXECUTION_CONTEXT_SCHEMA_VERSION,
};
pub use finalize_environment_and_toolchain_manager_parity_across_ui::{
    current_stable_inspector_parity_truth_packet, InspectorFieldClass,
    InspectorParityConfidenceClass, InspectorParityConsumerProjection,
    InspectorParityConsumerSurface, InspectorParityDowngradeAutomationClass,
    InspectorParityEvidenceClass, InspectorParityFindingKind, InspectorParityFindingSeverity,
    InspectorParityKnownLimitClass, InspectorParityLaneClass, InspectorParityPromotionState,
    InspectorParityRow, InspectorParityRowClass, InspectorParitySupportClass,
    InspectorParityTruthArtifactError, InspectorParityTruthPacket, InspectorParityTruthPacketInput,
    InspectorParityTruthSupportExport, InspectorParityValidationFinding, ParitySurfaceClass,
    RecoveryStateClass, INSPECTOR_PARITY_TRUTH_ARTIFACT_DOC_REF, INSPECTOR_PARITY_TRUTH_DOC_REF,
    INSPECTOR_PARITY_TRUTH_FIXTURE_DIR, INSPECTOR_PARITY_TRUTH_PACKET_ARTIFACT_REF,
    INSPECTOR_PARITY_TRUTH_PACKET_RECORD_KIND, INSPECTOR_PARITY_TRUTH_SCHEMA_REF,
    INSPECTOR_PARITY_TRUTH_SCHEMA_VERSION, INSPECTOR_PARITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use harden_breakpoint_call_stack_variables_watch_evaluate_and::{
    current_stable_debug_fidelity_truth_packet,
    ConsumerSurface as DebugFidelityConsumerSurface, DebugFidelityConfidenceClass,
    DebugFidelityConsumerProjection, DebugFidelityLaneClass, DebugFidelityRow,
    DebugFidelityRowClass, DebugFidelityTruthArtifactError, DebugFidelityTruthPacket,
    DebugFidelityTruthPacketInput, DebugFidelityTruthSupportExport,
    DowngradeAutomationClass as DebugFidelityDowngradeAutomationClass,
    EvidenceClass as DebugFidelityEvidenceClass, FindingKind as DebugFidelityFindingKind,
    FindingSeverity as DebugFidelityFindingSeverity, InspectorStateClass as DebugFidelityInspectorStateClass,
    InspectorSurfaceClass as DebugFidelityInspectorSurfaceClass,
    KnownLimitClass as DebugFidelityKnownLimitClass,
    MappingFidelityBadgeClass as DebugFidelityMappingFidelityBadgeClass,
    PromotionState as DebugFidelityPromotionState, SupportClass as DebugFidelitySupportClass,
    ValidationFinding as DebugFidelityValidationFinding,
    WedgeClass as DebugFidelityWedgeClass, DEBUG_FIDELITY_TRUTH_ARTIFACT_DOC_REF,
    DEBUG_FIDELITY_TRUTH_DOC_REF, DEBUG_FIDELITY_TRUTH_FIXTURE_DIR,
    DEBUG_FIDELITY_TRUTH_PACKET_ARTIFACT_REF, DEBUG_FIDELITY_TRUTH_PACKET_RECORD_KIND,
    DEBUG_FIDELITY_TRUTH_SCHEMA_REF, DEBUG_FIDELITY_TRUTH_SCHEMA_VERSION,
    DEBUG_FIDELITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use harden_coverage_flaky_test_snapshot_golden_and_baseline::{
    current_stable_coverage_quality_truth_packet,
    CandidateLineageClass as CoverageQualityCandidateLineageClass,
    ConsumerSurface as CoverageQualityConsumerSurface,
    ConsumerSurfaceBindingClass as CoverageQualityConsumerSurfaceBindingClass,
    CoverageImpactClass as CoverageQualityCoverageImpactClass,
    CoverageQualityConfidenceClass, CoverageQualityConsumerProjection,
    CoverageQualityLaneClass, CoverageQualityRow, CoverageQualityRowClass,
    CoverageQualityTruthArtifactError, CoverageQualityTruthPacket,
    CoverageQualityTruthPacketInput, CoverageQualityTruthSupportExport,
    DowngradeAutomationClass as CoverageQualityDowngradeAutomationClass,
    EvidenceClass as CoverageQualityEvidenceClass,
    FindingKind as CoverageQualityFindingKind,
    FindingSeverity as CoverageQualityFindingSeverity,
    KnownLimitClass as CoverageQualityKnownLimitClass,
    PromotionState as CoverageQualityPromotionState,
    QuarantineMuteStateClass as CoverageQualityQuarantineMuteStateClass,
    StabilityVerdictClass as CoverageQualityStabilityVerdictClass,
    SupportClass as CoverageQualitySupportClass,
    TestSourceClass as CoverageQualityTestSourceClass,
    ValidationFinding as CoverageQualityValidationFinding,
    WedgeClass as CoverageQualityWedgeClass,
    COVERAGE_QUALITY_TRUTH_ARTIFACT_DOC_REF, COVERAGE_QUALITY_TRUTH_DOC_REF,
    COVERAGE_QUALITY_TRUTH_FIXTURE_DIR, COVERAGE_QUALITY_TRUTH_PACKET_ARTIFACT_REF,
    COVERAGE_QUALITY_TRUTH_PACKET_RECORD_KIND, COVERAGE_QUALITY_TRUTH_SCHEMA_REF,
    COVERAGE_QUALITY_TRUTH_SCHEMA_VERSION, COVERAGE_QUALITY_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use harden_environment_capsule_resolution::{
    current_stable_capsule_resolution_truth_packet, CapsuleFieldClass,
    CapsuleResolutionConfidenceClass, CapsuleResolutionConsumerProjection,
    CapsuleResolutionConsumerSurface, CapsuleResolutionDowngradeAutomationClass,
    CapsuleResolutionEvidenceClass, CapsuleResolutionFindingKind, CapsuleResolutionFindingSeverity,
    CapsuleResolutionKnownLimitClass, CapsuleResolutionLaneClass, CapsuleResolutionPromotionState,
    CapsuleResolutionRow, CapsuleResolutionRowClass, CapsuleResolutionSupportClass,
    CapsuleResolutionTruthArtifactError, CapsuleResolutionTruthPacket,
    CapsuleResolutionTruthPacketInput, CapsuleResolutionTruthSupportExport,
    CapsuleResolutionValidationFinding, InvalidationReasonClass, PrebuildFingerprintComponentClass,
    ProjectDoctorFindingClass, CAPSULE_RESOLUTION_TRUTH_ARTIFACT_DOC_REF,
    CAPSULE_RESOLUTION_TRUTH_DOC_REF, CAPSULE_RESOLUTION_TRUTH_FIXTURE_DIR,
    CAPSULE_RESOLUTION_TRUTH_PACKET_ARTIFACT_REF, CAPSULE_RESOLUTION_TRUTH_PACKET_RECORD_KIND,
    CAPSULE_RESOLUTION_TRUTH_SCHEMA_REF, CAPSULE_RESOLUTION_TRUTH_SCHEMA_VERSION,
    CAPSULE_RESOLUTION_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use host_boundary::{
    evaluate_host_boundary_reapproval, ActionExposureClass, ActionOriginClass, ActionRouteClass,
    ActionTargetClass, AdapterConfidenceClass, AdapterConfidencePlaceholder, AdapterKind,
    AuthorityLinkageClass, BoundaryFreshnessClass, BoundaryManagedLifecycleState,
    BoundaryReachabilityClass, BoundaryRedactionClass, DiscoveryAuthorityBlock, ExpiryReasonClass,
    ExportInclusionPosture, HostBoundaryDriftField, HostBoundaryDriftRow,
    HostBoundaryIdentityChips, HostBoundaryReapprovalEvaluation, HostBoundaryReapprovalOutcome,
    HostBoundaryReviewBinding, HostBoundarySupportExport, HostBoundarySurfaceClass,
    HostBoundarySurfaceProjection, HostBoundaryTruthOptions, HostBoundaryTruthRecord,
    HostBoundaryTruthViolation, LocalOnlyContinuationReasonClass, ManagedLifecycleTruth,
    ManagedWorkspaceReviewerLabel, ReapprovalRequirementClass, RouteChangeReasonCode,
    WrongTargetCorrectionClass, HOST_BOUNDARY_AND_LIFECYCLE_SCHEMA_VERSION,
    HOST_BOUNDARY_REAPPROVAL_EVALUATION_RECORD_KIND, HOST_BOUNDARY_SUPPORT_EXPORT_RECORD_KIND,
    HOST_BOUNDARY_SURFACE_PROJECTION_RECORD_KIND, HOST_BOUNDARY_TRUTH_RECORD_KIND,
};
pub use language_hosts::{
    LanguageHostEventClass, LanguageHostExitReasonClass, LanguageHostIdentity,
    LanguageHostLaunchSpec, LanguageHostRuntimeStateClass, LanguageHostScopeKey,
    LanguageHostSnapshot, LanguageHostSupervisor, LanguageHostSupervisorConfig,
    LanguageHostSupervisorError, LanguageHostSupervisorEvent, LanguageHostSupportPacket,
    LANGUAGE_HOST_SUPERVISION_SCHEMA_VERSION,
};
pub use launch_profiles::{
    LaunchProfile, LaunchProfileAdapterBinding, LaunchProfileArguments, LaunchProfileCreateRequest,
    LaunchProfileDisclosureRow, LaunchProfileEdit, LaunchProfileEditClass,
    LaunchProfileEnvironmentBinding, LaunchProfileInvalidReason, LaunchProfileKind,
    LaunchProfileMode, LaunchProfileMutable, LaunchProfilePreview, LaunchProfilePreviewState,
    LaunchProfileRevision, LaunchProfileSideEffectClass, LaunchProfileStore,
    LaunchProfileStoreError, LaunchProfileSupportExport, LaunchProfileSupportRow,
    LaunchProfileTargetBinding, LAUNCH_PROFILE_EDIT_RECORD_KIND,
    LAUNCH_PROFILE_PREVIEW_RECORD_KIND, LAUNCH_PROFILE_RECORD_KIND,
    LAUNCH_PROFILE_REVISION_RECORD_KIND, LAUNCH_PROFILE_SCHEMA_VERSION,
    LAUNCH_PROFILE_SUPPORT_EXPORT_RECORD_KIND,
};
pub use managed_alpha::{
    ManagedReachabilityClass, ManagedReapprovalRequirementClass, ManagedRerunPostureClass,
    ManagedRuntimeInspectionLabel, ManagedRuntimePlacementClass, ManagedTargetFreshnessClass,
    ManagedWorkspaceAlphaRecord, ManagedWorkspaceAlphaViolation, ManagedWorkspaceBoundary,
    ManagedWorkspaceContinuity, ManagedWorkspaceInspectionSurface, ManagedWorkspaceLaneScope,
    ManagedWorkspaceLifecycleState, ManagedWorkspaceRuntimeInspection, ManagedWorkspaceStateClass,
    ManagedWorkspaceSupportExport, ManagedWorkspaceTransition, ManagedWorkspaceTransitionReason,
    MANAGED_WORKSPACE_ALPHA_LANE_ID, MANAGED_WORKSPACE_ALPHA_RECORD_KIND,
    MANAGED_WORKSPACE_ALPHA_SCHEMA_VERSION, MANAGED_WORKSPACE_RUNTIME_INSPECTION_RECORD_KIND,
    MANAGED_WORKSPACE_SUPPORT_EXPORT_RECORD_KIND,
};
pub use managed_workspace_lifecycle_beta::{
    ManagedLifecycleLineageEntry, ManagedLifecyclePhaseClass, ManagedLifecycleStateClass,
    ManagedLocalEditingContinuityClass, ManagedSurfaceClass, ManagedWorkspaceLifecycleBetaRecord,
    ManagedWorkspaceLifecycleBetaSupportExport, ManagedWorkspaceLifecycleBetaSurfaceProjection,
    ManagedWorkspaceLifecycleBetaViolation, MANAGED_WORKSPACE_LIFECYCLE_BETA_RECORD_KIND,
    MANAGED_WORKSPACE_LIFECYCLE_BETA_SCHEMA_VERSION,
    MANAGED_WORKSPACE_LIFECYCLE_BETA_SUPPORT_EXPORT_RECORD_KIND,
    MANAGED_WORKSPACE_LIFECYCLE_BETA_SURFACE_PROJECTION_RECORD_KIND,
};
pub use packages::{
    DependencySection, LockfileAlphaRef, LockfileCouplingClass, LockfileImpactAlphaRecord,
    LockfileImpactClass, LockfileMutationMode, ManifestDeltaClass, ManifestRequirementState,
    ManifestScopeAlphaDescriptor, ManifestScopeClass, MirrorOrOfflineStateClass,
    NodePackageMutationReviewRequest, NodePackageMutationReviewer,
    NodePackageMutationReviewerConfig, PackageAuditResultClass, PackageManagerFamily,
    PackageOperationAlphaPacket, PackageOperationAlphaViolation, PackageOperationAuditLineage,
    PackageOperationAuditPacket, PackageOperationClass, PackageOperationNoHiddenMutationGuards,
    PackageOperationSupportExport, PackageOperationSupportExportRow, PackageRedactionClass,
    PackageResolverIdentity, PackageReviewOutcomeClass, RegistryAuthModeClass,
    RegistryFreshnessClass, RegistryRevocationStateClass, RegistrySourceAlphaDescriptor,
    RegistrySourceClass, RollbackCheckpointAlphaSummary, RollbackPostureClass,
    ScriptRiskAlphaDescriptor, ScriptRiskClass, TransitiveImpactClass, ValidationTaskClass,
    LOCKFILE_IMPACT_ALPHA_RECORD_KIND, MANIFEST_SCOPE_ALPHA_RECORD_KIND,
    PACKAGE_MUTATION_REVIEWER_VERSION, PACKAGE_OPERATION_ALPHA_RECORD_KIND,
    PACKAGE_OPERATION_ALPHA_SCHEMA_VERSION, PACKAGE_OPERATION_AUDIT_RECORD_KIND,
    PACKAGE_OPERATION_SUPPORT_EXPORT_RECORD_KIND, REGISTRY_SOURCE_ALPHA_RECORD_KIND,
};
pub use preview_drift::{
    evaluate_preview_commit_guard, seeded_preview_commit_guard_scenario, ApprovalTicketBinding,
    GuardedActionClass, PolicySnapshotBinding, PreviewApprovalState,
    PreviewCommitAdmissionDecision, PreviewCommitAuditEventClass, PreviewCommitBasis,
    PreviewCommitCliOutput, PreviewCommitContext, PreviewCommitGuard, PreviewCommitGuardAuditEvent,
    PreviewCommitGuardEvaluation, PreviewCommitGuardScenario, PreviewCommitGuardSupportExport,
    PreviewCommitSurfaceProjection, PreviewInvalidationReason, PreviewInvalidationRow,
    PreviewLifecycleState, PreviewRepresentationClass, PreviewScalarBinding, PreviewTargetBinding,
    PREVIEW_COMMIT_GUARD_AUDIT_EVENT_RECORD_KIND, PREVIEW_COMMIT_GUARD_EVALUATION_RECORD_KIND,
    PREVIEW_COMMIT_GUARD_RECORD_KIND, PREVIEW_COMMIT_GUARD_SCHEMA_VERSION,
    PREVIEW_COMMIT_GUARD_SUPPORT_EXPORT_RECORD_KIND, PREVIEW_COMMIT_SURFACE_PROJECTION_RECORD_KIND,
};
pub use provenance::evidence_packet::{
    seeded_runtime_evidence_packet, seeded_runtime_evidence_packet_support_export,
    ReplayCompatibilityClass, ReplayIncompatibilityReason, RuntimeEvidenceKind,
    RuntimeEvidenceLane, RuntimeEvidencePacket, RuntimeEvidencePacketSeededScenario,
    RuntimeEvidencePacketSupportExport, RuntimeEvidenceReplayComparison,
    RUNTIME_EVIDENCE_PACKET_RECORD_KIND, RUNTIME_EVIDENCE_PACKET_SCHEMA_VERSION,
    RUNTIME_EVIDENCE_PACKET_SUPPORT_EXPORT_RECORD_KIND,
    RUNTIME_EVIDENCE_REPLAY_COMPARISON_RECORD_KIND,
};
pub use provenance::{
    dedupe_context_provenance, ExecutionEventProvenance, ExecutionProvenanceEvent,
    ExecutionProvenanceEventClass, ExecutionProvenanceInputDecision,
    ExecutionProvenanceRedactionClass, EXECUTION_EVENT_PROVENANCE_RECORD_KIND,
    EXECUTION_EVENT_PROVENANCE_SCHEMA_VERSION, EXECUTION_PROVENANCE_EVENT_RECORD_KIND,
};
pub use quality::{
    BaselineCompatibilityStateClass, BaselineRecord, BaselineRecordRequest,
    EffectiveQualityProfile, QualityActionClass, QualityActionDisclosureClass,
    QualityActionProposal, QualityActionProposalRequest, QualityActorClass,
    QualityApplyPostureClass, QualityDebtReopenStateClass, QualityGovernanceError,
    QualityGovernanceSupportExport, QualityLockReasonClass, QualityLockStateClass,
    QualityMutationScopeClass, QualityOwnerClass, QualityPolicyLockStateClass,
    QualityPreviewRequirementClass, QualityProfileResolutionRequest, QualityProfileResolver,
    QualityProfileSourceCandidate, QualityProfileSourceLayer, QualityProfileSourceRow,
    QualityProfileSourceStateClass, QualityProfileSurfaceProjection, QualityReopenRuleClass,
    QualityRollbackBoundaryClass, QualitySafetyClass, QualitySession, QualitySessionOutcomeClass,
    QualitySessionRequest, QualitySessionTriggerClass, QualitySurfaceClass,
    QualityTargetScopeClass, QualityToolFamilyClass, QualityTruthMutationClass, SuppressionRecord,
    SuppressionRecordRequest, BASELINE_RECORD_KIND, EFFECTIVE_QUALITY_PROFILE_RECORD_KIND,
    QUALITY_ACTION_PROPOSAL_RECORD_KIND, QUALITY_GOVERNANCE_SCHEMA_VERSION,
    QUALITY_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND, QUALITY_SESSION_RECORD_KIND,
    SUPPRESSION_RECORD_KIND,
};
pub use recipes::{
    RecipeAlphaContractRefs, RecipeAlphaCoverage, RecipeAlphaFinding, RecipeAlphaFindingSeverity,
    RecipeAlphaFixtureMetadata, RecipeAlphaPage, RecipeAlphaSupportExport,
    RecipeAlphaValidationReport, RecipeApprovalClass, RecipeAttribution, RecipeAttributionSummary,
    RecipeAttributionSurfaceClass, RecipeAuditEvent, RecipeAuditEventClass,
    RecipeAuditEventSummary, RecipeDefinition, RecipeDefinitionSummary, RecipeDenialReasonClass,
    RecipePreviewRequirementClass, RecipeRun, RecipeRunDispositionClass, RecipeRunSummary,
    RecipeStep, RecipeStepDisposition, RecipeStepDispositionClass, RecipeTrustGateClass,
    RecipeWriteClass, StepCommandLineageClass, StepModeRequirementClass,
    RECIPE_ALPHA_ATTRIBUTION_RECORD_KIND, RECIPE_ALPHA_AUDIT_EVENT_RECORD_KIND,
    RECIPE_ALPHA_DEFINITION_RECORD_KIND, RECIPE_ALPHA_PAGE_RECORD_KIND,
    RECIPE_ALPHA_RUN_RECORD_KIND, RECIPE_ALPHA_SCHEMA_VERSION, RECIPE_ALPHA_SHARED_CONTRACT_REF,
    RECIPE_ALPHA_SUPPORT_EXPORT_RECORD_KIND, RECIPE_ALPHA_VALIDATION_REPORT_RECORD_KIND,
};
pub use remote_helper_skew_beta::{
    RemoteHelperBetaCompatibilityRow, RemoteHelperBetaRecord, RemoteHelperBetaSupportExport,
    RemoteHelperLifecyclePhaseClass, RemoteHelperRepairPathClass, RemoteHelperSkewVisibilityClass,
    RemoteHelperVisibleVersionState, REMOTE_HELPER_SKEW_BETA_COMPATIBILITY_ROW_RECORD_KIND,
    REMOTE_HELPER_SKEW_BETA_RECORD_KIND, REMOTE_HELPER_SKEW_BETA_SCHEMA_VERSION,
    REMOTE_HELPER_SKEW_BETA_SUPPORT_EXPORT_RECORD_KIND,
};
pub use request_workspace::{
    seeded_request_workspace_record, seeded_request_workspace_support_export,
    seeded_send_inspector_report, AssertionDescriptor, AssertionKind, AssertionOutcomeClass,
    AssertionResultRow, AuthProfile, AuthStrategyKind, CredentialClass, EnvironmentLayerKind,
    EnvironmentSet, EnvironmentVariableLayer, ExpectedSideEffectRow, LatencyBandClass,
    RequestDocument, RequestMethodClass, RequestWorkspaceAlphaRecord,
    RequestWorkspaceAlphaViolation, RequestWorkspaceSeededScenario, RequestWorkspaceSupportExport,
    ResponseArtifact, ResponsePreviewClass, ResponseRedactionClass, SchemaSnapshot,
    SchemaSnapshotFreshness, SchemaSnapshotKind, SchemaSnapshotSourceClass, SendInspectorBanner,
    SendInspectorReadiness, SendInspectorReport, SideEffectClass, REQUEST_WORKSPACE_ALPHA_LANE_ID,
    REQUEST_WORKSPACE_ALPHA_RECORD_KIND, REQUEST_WORKSPACE_ALPHA_SCHEMA_VERSION,
    REQUEST_WORKSPACE_SEND_INSPECTOR_RECORD_KIND, REQUEST_WORKSPACE_SUPPORT_EXPORT_RECORD_KIND,
};
pub use request_workspace_contracts::{
    AssertionEvidenceState, AssertionSuite, AssertionSuiteLineageClass, AuthSourceClass,
    EndpointIdentity, EndpointSourceClass, EnvironmentFingerprintState, FingerprintDigestClass,
    PortableExportClass, PortableExportContract, RequestEnvironmentFingerprint,
    RequestHistoryPosture, RequestHistoryRetentionClass, ResponseCopyExportClass,
    ResponsePayloadSizeClass, ResponsePreviewComponentClass, ResponsePreviewRule,
    ResponseSafePreviewClass, REQUEST_ASSERTION_SUITE_SCHEMA_ID,
    REQUEST_ENVIRONMENT_FINGERPRINT_SCHEMA_ID, REQUEST_RESPONSE_PREVIEW_SCHEMA_ID,
};
pub use rerun::{
    built_in_rerun_command_bindings, RerunAttemptSummary, RerunCommandBinding, RerunContractKind,
    RerunDiffClass, RerunDiffRow, RerunDispatchState, RerunKeyboardRoute, RerunLane,
    RerunLastLaunch, RerunLastLoop, RerunPreparedAttempt, RerunRunContract, RerunSupportExport,
    RerunTargetComparison, RerunTargetMode, RerunTargetSnapshot, RerunUnavailableReason,
    RERUN_COMMAND_BINDING_RECORD_KIND, RERUN_LAST_LAUNCH_RECORD_KIND, RERUN_LAST_TASK_COMMAND_ID,
    RERUN_LAST_TEST_COMMAND_ID, RERUN_LOOP_SCHEMA_VERSION, RERUN_PREPARED_ATTEMPT_RECORD_KIND,
    RERUN_SUPPORT_EXPORT_RECORD_KIND, RERUN_TARGET_COMPARISON_RECORD_KIND,
};
pub use resource_governor::{
    seeded_resource_governor_snapshot, seeded_resource_governor_support_export,
    AdmissionControlDecision, AdmissionDecisionClass, CheckpointMetadata, GovernorHealthState,
    GovernorTransition, GovernorWorkClass, OverrideDecisionClass, OverrideScope, OverrideSheet,
    PressureDimension, PressureInput, ProtectedForegroundAction, QueueLane, QueueLaneState,
    QueueLaneStateFlag, ResourceGovernorSnapshot, ResourceGovernorSupportExport,
    ResourceGovernorValidationReport, ResourceGovernorValidationViolation, VisibleHealthState,
    QUEUE_LANE_STATE_RECORD_KIND, RESOURCE_GOVERNOR_SCHEMA_VERSION,
    RESOURCE_GOVERNOR_SNAPSHOT_RECORD_KIND, RESOURCE_GOVERNOR_SUPPORT_EXPORT_RECORD_KIND,
};
pub use run_lineage::{
    seeded_run_history_support_export, DurableJobRow, RerunReviewDriftField, RerunReviewDriftRow,
    RerunReviewMode, RerunReviewModeOption, RerunReviewSheet, RunActionRef, RunArtifactActionClass,
    RunArtifactDetailSheet, RunArtifactKind, RunArtifactRetentionClass, RunArtifactViewerClass,
    RunBoundaryClass, RunBuildIdentity, RunContextSummary, RunContinuityMarker,
    RunCurrentRelationshipClass, RunFreshnessClass, RunHistorySupportExport, RunInterruptionKind,
    RunLifecycleState, RunLineageSeededScenario, RunSummaryCard, DURABLE_JOB_ROW_RECORD_KIND,
    RERUN_REVIEW_SHEET_RECORD_KIND, RUN_ARTIFACT_DETAIL_SHEET_RECORD_KIND,
    RUN_HISTORY_SUPPORT_EXPORT_RECORD_KIND, RUN_LINEAGE_SCHEMA_VERSION,
    RUN_SUMMARY_CARD_RECORD_KIND,
};
pub use shared_debug_alpha::{
    LocalDebugContinuityClass, LocalDebugContinuityObservation,
    LocalDebugContinuityObservationSummary, SharedDebugAlphaContractRefs, SharedDebugAlphaCoverage,
    SharedDebugAlphaFinding, SharedDebugAlphaFindingSeverity, SharedDebugAlphaFixtureMetadata,
    SharedDebugAlphaPage, SharedDebugAlphaSupportExport, SharedDebugAlphaValidationReport,
    SharedDebugAuditEvent, SharedDebugAuditEventClass, SharedDebugAuditEventSummary,
    SharedDebugBinding, SharedDebugControlState, SharedDebugControlStateClass,
    SharedDebugControlStateSummary, SHARED_DEBUG_ALPHA_AUDIT_EVENT_RECORD_KIND,
    SHARED_DEBUG_ALPHA_CONTINUITY_OBSERVATION_RECORD_KIND,
    SHARED_DEBUG_ALPHA_CONTROL_STATE_RECORD_KIND, SHARED_DEBUG_ALPHA_PAGE_RECORD_KIND,
    SHARED_DEBUG_ALPHA_PRESENTER_HANDOFF_RECORD_KIND, SHARED_DEBUG_ALPHA_SCHEMA_VERSION,
    SHARED_DEBUG_ALPHA_SHARED_CONTRACT_REF, SHARED_DEBUG_ALPHA_SUPPORT_EXPORT_RECORD_KIND,
    SHARED_DEBUG_ALPHA_VALIDATION_REPORT_RECORD_KIND,
};
pub use shared_terminal_alpha::{
    ControlRevocationCauseClass, LocalContinuityClass, LocalTerminalContinuityObservation,
    LocalTerminalContinuityObservationSummary, ParticipantRoleClass, PresenterHandoffEvent,
    PresenterHandoffOutcomeClass, PresenterHandoffSummary, SharedTerminalAlphaContractRefs,
    SharedTerminalAlphaCoverage, SharedTerminalAlphaFinding, SharedTerminalAlphaFindingSeverity,
    SharedTerminalAlphaFixtureMetadata, SharedTerminalAlphaPage, SharedTerminalAlphaSupportExport,
    SharedTerminalAlphaValidationReport, SharedTerminalAuditEvent, SharedTerminalAuditEventClass,
    SharedTerminalAuditEventSummary, SharedTerminalBinding, SharedTerminalControlState,
    SharedTerminalControlStateClass, SharedTerminalControlStateSummary,
    SHARED_TERMINAL_ALPHA_AUDIT_EVENT_RECORD_KIND,
    SHARED_TERMINAL_ALPHA_CONTINUITY_OBSERVATION_RECORD_KIND,
    SHARED_TERMINAL_ALPHA_CONTROL_STATE_RECORD_KIND, SHARED_TERMINAL_ALPHA_PAGE_RECORD_KIND,
    SHARED_TERMINAL_ALPHA_PRESENTER_HANDOFF_RECORD_KIND, SHARED_TERMINAL_ALPHA_SCHEMA_VERSION,
    SHARED_TERMINAL_ALPHA_SHARED_CONTRACT_REF, SHARED_TERMINAL_ALPHA_SUPPORT_EXPORT_RECORD_KIND,
    SHARED_TERMINAL_ALPHA_VALIDATION_REPORT_RECORD_KIND,
};
pub use stabilize_debugger_host_and_adapter_negotiation::{
    current_stable_debugger_stabilization_truth_packet,
    AdapterDescriptorFieldClass as DebuggerStabilizationAdapterDescriptorFieldClass,
    AttachLaunchParitySurfaceClass as DebuggerStabilizationAttachLaunchParitySurfaceClass,
    AttachLaunchPostureClass as DebuggerStabilizationAttachLaunchPostureClass,
    ConsumerSurface as DebuggerStabilizationConsumerSurface,
    CrashIsolationAssertionClass as DebuggerStabilizationCrashIsolationAssertionClass,
    DebuggerStabilizationConfidenceClass, DebuggerStabilizationConsumerProjection,
    DebuggerStabilizationLaneClass, DebuggerStabilizationRow, DebuggerStabilizationRowClass,
    DebuggerStabilizationTruthArtifactError, DebuggerStabilizationTruthPacket,
    DebuggerStabilizationTruthPacketInput, DebuggerStabilizationTruthSupportExport,
    DowngradeAutomationClass as DebuggerStabilizationDowngradeAutomationClass,
    EvidenceClass as DebuggerStabilizationEvidenceClass,
    FindingKind as DebuggerStabilizationFindingKind,
    FindingSeverity as DebuggerStabilizationFindingSeverity,
    KnownLimitClass as DebuggerStabilizationKnownLimitClass,
    PromotionState as DebuggerStabilizationPromotionState,
    SupportClass as DebuggerStabilizationSupportClass,
    ValidationFinding as DebuggerStabilizationValidationFinding,
    WedgeClass as DebuggerStabilizationWedgeClass,
    DEBUGGER_STABILIZATION_TRUTH_ARTIFACT_DOC_REF, DEBUGGER_STABILIZATION_TRUTH_DOC_REF,
    DEBUGGER_STABILIZATION_TRUTH_FIXTURE_DIR, DEBUGGER_STABILIZATION_TRUTH_PACKET_ARTIFACT_REF,
    DEBUGGER_STABILIZATION_TRUTH_PACKET_RECORD_KIND, DEBUGGER_STABILIZATION_TRUTH_SCHEMA_REF,
    DEBUGGER_STABILIZATION_TRUTH_SCHEMA_VERSION,
    DEBUGGER_STABILIZATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use stabilize_execution_context_resolver::{
    current_stable_stabilize_execution_context_resolver_truth_packet,
    ConsumerSurface as StabilizeExecutionContextResolverConsumerSurface,
    DowngradeAutomationClass as StabilizeExecutionContextResolverDowngradeAutomationClass,
    EvidenceClass as StabilizeExecutionContextResolverEvidenceClass,
    ExecutionContextConfidenceClass as StabilizeExecutionContextResolverConfidenceClass,
    ExecutionContextRowClass as StabilizeExecutionContextResolverRowClass,
    ExecutionLaneClass as StabilizeExecutionContextResolverLaneClass,
    FindingKind as StabilizeExecutionContextResolverFindingKind,
    FindingSeverity as StabilizeExecutionContextResolverFindingSeverity,
    KnownLimitClass as StabilizeExecutionContextResolverKnownLimitClass,
    PromotionState as StabilizeExecutionContextResolverPromotionState,
    ResolverStateClass as StabilizeExecutionContextResolverStateClass,
    StabilizeExecutionContextResolverConsumerProjection, StabilizeExecutionContextResolverRow,
    StabilizeExecutionContextResolverTruthArtifactError,
    StabilizeExecutionContextResolverTruthPacket,
    StabilizeExecutionContextResolverTruthPacketInput,
    StabilizeExecutionContextResolverTruthSupportExport,
    SupportClass as StabilizeExecutionContextResolverSupportClass,
    SurfaceBindingClass as StabilizeExecutionContextResolverSurfaceBindingClass,
    ValidationFinding as StabilizeExecutionContextResolverValidationFinding,
    STABILIZE_EXECUTION_CONTEXT_RESOLVER_TRUTH_ARTIFACT_DOC_REF,
    STABILIZE_EXECUTION_CONTEXT_RESOLVER_TRUTH_DOC_REF,
    STABILIZE_EXECUTION_CONTEXT_RESOLVER_TRUTH_FIXTURE_DIR,
    STABILIZE_EXECUTION_CONTEXT_RESOLVER_TRUTH_PACKET_ARTIFACT_REF,
    STABILIZE_EXECUTION_CONTEXT_RESOLVER_TRUTH_PACKET_RECORD_KIND,
    STABILIZE_EXECUTION_CONTEXT_RESOLVER_TRUTH_SCHEMA_REF,
    STABILIZE_EXECUTION_CONTEXT_RESOLVER_TRUTH_SCHEMA_VERSION,
    STABILIZE_EXECUTION_CONTEXT_RESOLVER_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use stabilize_task_discovery_launch_profiles_rerun_last_behavior::{
    current_stable_task_event_truth_packet, ConsumerSurface as TaskEventTruthConsumerSurface,
    DowngradeAutomationClass as TaskEventTruthDowngradeAutomationClass,
    DownstreamSurfaceClass as TaskEventTruthDownstreamSurfaceClass,
    EnvelopeFieldClass as TaskEventTruthEnvelopeFieldClass,
    EvidenceClass as TaskEventTruthEvidenceClass, FindingKind as TaskEventTruthFindingKind,
    FindingSeverity as TaskEventTruthFindingSeverity,
    KnownLimitClass as TaskEventTruthKnownLimitClass,
    PromotionState as TaskEventTruthPromotionState, SupportClass as TaskEventTruthSupportClass,
    TaskEventTruthArtifactError, TaskEventTruthConfidenceClass, TaskEventTruthConsumerProjection,
    TaskEventTruthLaneClass, TaskEventTruthPacket, TaskEventTruthPacketInput, TaskEventTruthRow,
    TaskEventTruthRowClass, TaskEventTruthSupportExport,
    ValidationFinding as TaskEventTruthValidationFinding, WedgeClass as TaskEventTruthWedgeClass,
    TASK_EVENT_TRUTH_ARTIFACT_DOC_REF, TASK_EVENT_TRUTH_DOC_REF, TASK_EVENT_TRUTH_FIXTURE_DIR,
    TASK_EVENT_TRUTH_PACKET_ARTIFACT_REF, TASK_EVENT_TRUTH_PACKET_RECORD_KIND,
    TASK_EVENT_TRUTH_SCHEMA_REF, TASK_EVENT_TRUTH_SCHEMA_VERSION,
    TASK_EVENT_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use stabilize_the_test_explorer_inline_results_watch_mode::{
    current_stable_test_explorer_stabilization_truth_packet,
    ConsumerSurface as TestExplorerStabilizationConsumerSurface,
    ConsumerSurfaceBindingClass as TestExplorerStabilizationConsumerSurfaceBindingClass,
    DiscoveryPostureClass as TestExplorerStabilizationDiscoveryPostureClass,
    DowngradeAutomationClass as TestExplorerStabilizationDowngradeAutomationClass,
    EvidenceClass as TestExplorerStabilizationEvidenceClass,
    FindingKind as TestExplorerStabilizationFindingKind,
    FindingSeverity as TestExplorerStabilizationFindingSeverity,
    KnownLimitClass as TestExplorerStabilizationKnownLimitClass,
    PromotionState as TestExplorerStabilizationPromotionState,
    SelectorDurabilityClass as TestExplorerStabilizationSelectorDurabilityClass,
    SupportClass as TestExplorerStabilizationSupportClass,
    TestExplorerConfidenceClass as TestExplorerStabilizationConfidenceClass,
    TestExplorerConsumerProjection as TestExplorerStabilizationConsumerProjection,
    TestExplorerLaneClass as TestExplorerStabilizationLaneClass,
    TestExplorerRow as TestExplorerStabilizationRow,
    TestExplorerRowClass as TestExplorerStabilizationRowClass,
    TestExplorerStabilizationTruthArtifactError, TestExplorerStabilizationTruthPacket,
    TestExplorerStabilizationTruthPacketInput, TestExplorerStabilizationTruthSupportExport,
    TestIdentityClass as TestExplorerStabilizationTestIdentityClass,
    ValidationFinding as TestExplorerStabilizationValidationFinding,
    WatchModeSupportClass as TestExplorerStabilizationWatchModeSupportClass,
    WedgeClass as TestExplorerStabilizationWedgeClass,
    TEST_EXPLORER_STABILIZATION_TRUTH_ARTIFACT_DOC_REF,
    TEST_EXPLORER_STABILIZATION_TRUTH_DOC_REF,
    TEST_EXPLORER_STABILIZATION_TRUTH_FIXTURE_DIR,
    TEST_EXPLORER_STABILIZATION_TRUTH_PACKET_ARTIFACT_REF,
    TEST_EXPLORER_STABILIZATION_TRUTH_PACKET_RECORD_KIND,
    TEST_EXPLORER_STABILIZATION_TRUTH_SCHEMA_REF,
    TEST_EXPLORER_STABILIZATION_TRUTH_SCHEMA_VERSION,
    TEST_EXPLORER_STABILIZATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use support_matrix_beta::{
    SupportMatrixAttachSupport, SupportMatrixBetaManifest, SupportMatrixBetaSupportExport,
    SupportMatrixClass, SupportMatrixContextLane, SupportMatrixContextLaneExpectation,
    SupportMatrixContextLaneSupport, SupportMatrixContextSupport, SupportMatrixDowngradeRule,
    SupportMatrixInputMismatch, SupportMatrixLaunchSupport, SupportMatrixTestSupport,
    SupportMatrixWedgeId, SupportMatrixWedgeInput, SupportMatrixWedgeRow,
    SUPPORT_MATRIX_BETA_MANIFEST_RECORD_KIND, SUPPORT_MATRIX_BETA_SCHEMA_VERSION,
    SUPPORT_MATRIX_BETA_SUPPORT_EXPORT_RECORD_KIND, SUPPORT_MATRIX_BETA_WEDGE_INPUT_RECORD_KIND,
    SUPPORT_MATRIX_BETA_WEDGE_ROW_RECORD_KIND,
};
pub use target_discovery::{
    DiscoveryFreshnessClass, DiscoverySourceClass, ProtectedActionClass,
    ProtectedActionDecisionClass, ProtectedActionDecisionRow, SupportedCapabilityClass,
    TargetDiscoveryBetaCoverageManifest, TargetDiscoveryBetaCoverageRow,
    TargetDiscoveryBetaProjection, TargetDiscoveryBetaRow, TargetDiscoveryBetaSupportExport,
    TARGET_DISCOVERY_BETA_COVERAGE_MANIFEST_RECORD_KIND,
    TARGET_DISCOVERY_BETA_PROJECTION_RECORD_KIND, TARGET_DISCOVERY_BETA_ROW_RECORD_KIND,
    TARGET_DISCOVERY_BETA_SCHEMA_VERSION, TARGET_DISCOVERY_BETA_SUPPORT_EXPORT_RECORD_KIND,
};
pub use targets::{
    HostBoundaryCueClass, TargetConfidenceCard, TargetConfidenceExplanationRow,
    TargetConfidenceLaneClass, TargetConfidenceReviewPacket, TargetConfidenceReviewRow,
    TargetConfidenceSupportExport, TargetDiscoveryConfidenceClass, TargetHostBoundaryRow,
    TARGET_CONFIDENCE_ALPHA_SCHEMA_VERSION, TARGET_CONFIDENCE_CARD_RECORD_KIND,
    TARGET_CONFIDENCE_REVIEW_PACKET_RECORD_KIND, TARGET_CONFIDENCE_SUPPORT_EXPORT_RECORD_KIND,
};
pub use task_events::{
    lane_for_event, lane_for_wedge, TaskEventBetaCoverageManifest, TaskEventBetaLane,
    TaskEventBetaLaneCoverageRow, TASK_EVENT_BETA_COVERAGE_MANIFEST_RECORD_KIND,
};
pub use tasks::{
    RawEnvelopeRetentionState, RawTaskEventEnvelope, TaskActivityProjection, TaskArtifactKind,
    TaskBlockReason, TaskConsumerSurfaceClass, TaskDegradationReason, TaskDiagnosticSeverity,
    TaskEvent, TaskEventConfidence, TaskEventIdentity, TaskEventKind, TaskEventPayload,
    TaskEventProvenance, TaskEventRedactionClass, TaskEventSourceKind, TaskEventStream,
    TaskEventStreamError, TaskExitStatus, TaskFailureClass, TaskInputClass, TaskInputRequest,
    TaskOutputStreamClass, TaskProgress, TaskShellProjection, TaskState, TaskStateClass,
    TaskSupportEventRow, TaskSupportExport, TaskWedgeClass, RAW_TASK_EVENT_ENVELOPE_RECORD_KIND,
    TASK_ACTIVITY_PROJECTION_RECORD_KIND, TASK_EVENT_RECORD_KIND, TASK_EVENT_SCHEMA_VERSION,
    TASK_EVENT_STREAM_RECORD_KIND, TASK_SHELL_PROJECTION_RECORD_KIND, TASK_STATE_RECORD_KIND,
    TASK_SUPPORT_EXPORT_RECORD_KIND,
};
pub use testing::{
    InlineTestResultProjection, InlineTestResultRow, TestArtifactIdentity, TestArtifactKind,
    TestRunnerBetaCoverageManifest, TestRunnerBetaCoverageRow, TestRunnerBetaFramework,
    TestRunnerBetaParityState, TestRunnerBetaProjection, TestRunnerBetaRerunParity,
    TestRunnerBetaSupportExport, TestTreeProjection, TestTreeRow, TestTreeRowKind,
    TEST_RUNNER_BETA_ARTIFACT_IDENTITY_RECORD_KIND, TEST_RUNNER_BETA_COVERAGE_MANIFEST_RECORD_KIND,
    TEST_RUNNER_BETA_INLINE_PROJECTION_RECORD_KIND, TEST_RUNNER_BETA_INLINE_ROW_RECORD_KIND,
    TEST_RUNNER_BETA_RERUN_PARITY_RECORD_KIND, TEST_RUNNER_BETA_SCHEMA_VERSION,
    TEST_RUNNER_BETA_SUPPORT_EXPORT_RECORD_KIND, TEST_RUNNER_BETA_TREE_PROJECTION_RECORD_KIND,
    TEST_RUNNER_BETA_TREE_ROW_RECORD_KIND,
};
pub use testing_identity::{
    CanonicalTestAttempt, CanonicalTestItem, CanonicalTestItemKind, CanonicalTestSession,
    ImportedCiTruthClass, ImportedCiTruthOverlay, TestAdapterKind, TestAttemptLineageClass,
    TestEvidenceClass, TestIdentityBetaBundle, TestIdentityLedgerError, TestIdentitySupportExport,
    TestIdentitySurface, TestItemIdentityClass, TestResultFreshnessClass, TestSelectionOrigin,
    TestSelectorBinding, TestSurfaceIdentityBinding, TestTargetEnvironmentClass,
    TestTargetEnvironmentIdentity, CANONICAL_TEST_ATTEMPT_RECORD_KIND,
    CANONICAL_TEST_ITEM_RECORD_KIND, CANONICAL_TEST_SESSION_RECORD_KIND,
    IMPORTED_CI_TRUTH_OVERLAY_RECORD_KIND, TEST_IDENTITY_BETA_BUNDLE_RECORD_KIND,
    TEST_IDENTITY_BETA_SCHEMA_VERSION, TEST_IDENTITY_SUPPORT_EXPORT_RECORD_KIND,
    TEST_SELECTOR_BINDING_RECORD_KIND, TEST_SURFACE_IDENTITY_BINDING_RECORD_KIND,
};
pub use testing_quality::{
    BaselineTruthPacket, CoverageTruthPacket, FlakyTruthPacket, SnapshotTruthPacket,
    TestQualityBetaCoverageManifest, TestQualityBetaCoverageRow, TestQualityBetaSupportExport,
    TestQualityFreshness, TestQualityKind, TestQualityPacketIdentity, TestQualityProjection,
    TestQualityProvenanceSource, TestQualityRowTruth, TestQualitySupportClass,
    TEST_QUALITY_BASELINE_PACKET_RECORD_KIND, TEST_QUALITY_BETA_COVERAGE_MANIFEST_RECORD_KIND,
    TEST_QUALITY_BETA_PROJECTION_RECORD_KIND, TEST_QUALITY_BETA_SUPPORT_EXPORT_RECORD_KIND,
    TEST_QUALITY_COVERAGE_PACKET_RECORD_KIND, TEST_QUALITY_FLAKY_PACKET_RECORD_KIND,
    TEST_QUALITY_ROW_TRUTH_RECORD_KIND, TEST_QUALITY_SNAPSHOT_PACKET_RECORD_KIND,
    TEST_QUALITY_TRUTH_BETA_SCHEMA_VERSION,
};
pub use testing_triage::{
    FlakyVerdictAttemptInput, FlakyVerdictPacket, SnapshotFileChangePreview,
    SnapshotMutationReview, SnapshotMutationReviewState, TestEvidenceTrustClass,
    TestQuarantineReason, TestQuarantineRecord, TestQuarantineReopenBehavior,
    TestQuarantineScopeClass, TestQuarantineStatus, TestQuarantineTreatmentKind,
    TestReleaseDebtClass, TestTriageIdentity, TestTrustPacket, TestTrustRowSummary,
    WatchModeDowngradeReason, WatchModeState, WatchStatePacket, FLAKY_VERDICT_PACKET_RECORD_KIND,
    SNAPSHOT_MUTATION_REVIEW_RECORD_KIND, TEST_QUARANTINE_RECORD_KIND,
    TEST_TRIAGE_TRUST_SCHEMA_VERSION, TEST_TRUST_PACKET_RECORD_KIND,
    WATCH_STATE_PACKET_RECORD_KIND,
};
pub use tests::{
    AiTestGenerationGateState, CoverageMergeClass, FlakyVerdictState, ImportedCiProjection,
    ImportedCiProjectionClass, ImportedSignalAuthority, TestAttemptAlphaOptions,
    TestAttemptAlphaPacket, TestAttemptKind, TestAttemptRecord, TestAttemptResultState,
    TestAttemptSupportExport, TestConsumerSurface, TestIdentityStability,
    TestItemIdentityProjection, TestLaunchWedgeProjection, TestSessionMode, TestSessionPlan,
    TestSourceDriftState, TestStabilityVerdict, TestWatchController, TestWatchDegradationReason,
    TestWatchState, IMPORTED_CI_PROJECTION_RECORD_KIND, TEST_ATTEMPT_ALPHA_PACKET_RECORD_KIND,
    TEST_ATTEMPT_ALPHA_SCHEMA_VERSION, TEST_ATTEMPT_RECORD_KIND,
    TEST_ATTEMPT_SUPPORT_EXPORT_RECORD_KIND, TEST_ITEM_IDENTITY_PROJECTION_RECORD_KIND,
    TEST_LAUNCH_WEDGE_PROJECTION_RECORD_KIND, TEST_SESSION_PLAN_RECORD_KIND,
    TEST_STABILITY_VERDICT_RECORD_KIND, TEST_WATCH_CONTROLLER_RECORD_KIND,
};
pub use topology_inspector::{
    seeded_host_lanes, seeded_host_topology_inspector, seeded_lane_filtered_event_viewer,
    seeded_reattach_review_sheet, CrashLoopQuarantineBanner, FaultDomainClass,
    FaultDomainNextSafeActionClass, FaultDomainRestartCard, HostBadgeGroup, HostBoundaryBadge,
    HostBoundaryBadgeClass, HostDetailAction, HostDetailOpenTarget, HostLaneFamily,
    HostLaneHealthClass, HostLaneRecord, HostLaneSeed, HostResultFreshnessClass, LaneEventRow,
    LaneFilteredEventViewer, ReattachDriftFieldClass, ReattachDriftRow, ReattachReplayRiskClass,
    ReattachReviewDecisionClass, ReattachReviewInput, ReattachReviewSheet, RerunRequirementClass,
    RestartBudgetStateClass, RestartMarkerClass, RuntimeResultSeed, RuntimeSurfaceClass,
    RuntimeSurfaceResult, TopologyInspectorRecord, TopologyInspectorViolation,
    CRASH_LOOP_QUARANTINE_BANNER_RECORD_KIND, FAULT_DOMAIN_RESTART_CARD_RECORD_KIND,
    HOST_BADGE_GROUP_RECORD_KIND, HOST_LANE_RECORD_KIND, HOST_TOPOLOGY_SCHEMA_VERSION,
    LANE_FILTERED_EVENT_VIEWER_RECORD_KIND, REATTACH_REVIEW_SHEET_RECORD_KIND,
    TOPOLOGY_INSPECTOR_RECORD_KIND,
};
pub use trace_replay_alpha::{
    BuildRuntimeIdentity, CaptureMode, CaptureSource, CaptureWindow, ComparisonClass,
    ComparisonClassAlphaPacket, ComparisonRuntimeToolchain, ComparisonSourceClass,
    DerivedTraceView, DerivedViewKind, DigestAlgorithm, DigestEntry, HardwarePowerProfile,
    MappingQualityState, MappingQualitySummary, OverheadClass, ProfileCaptureDescriptor,
    ProfileExportPolicy, ProfileSessionAlpha, ProfileTargetIdentity, RawTraceBundle,
    ReplayBackendIdentity, ReplayCapabilityAlphaDescriptor, ReplayExportPosture,
    ReplayFeatureState, ReplayFeatureSupport, ReplayLaneState, ReplayOverheadStorageBand,
    ReplayRuntimeToolchainRange, ReplaySupportMatrix, RuntimeEvidenceAlphaPacket,
    RuntimeEvidenceDataClass, RuntimeEvidenceDataPosture, RuntimeEvidenceSupportExport,
    TraceBundleAlphaManifest, TraceBundleImmutability, TraceBundleRedaction, TraceBundleRetention,
    TraceMetricFamily, TraceRedactionMode, TraceRetentionClass, VarianceWindow,
    COMPARISON_CLASS_ALPHA_RECORD_KIND, PROFILE_SESSION_ALPHA_RECORD_KIND,
    REPLAY_CAPABILITY_ALPHA_RECORD_KIND, RUNTIME_EVIDENCE_ALPHA_PACKET_RECORD_KIND,
    RUNTIME_EVIDENCE_ALPHA_SCHEMA_VERSION, RUNTIME_EVIDENCE_SUPPORT_EXPORT_RECORD_KIND,
    SUPPORT_ITEM_RUNTIME_TRACES, TRACE_BUNDLE_ALPHA_RECORD_KIND,
};
