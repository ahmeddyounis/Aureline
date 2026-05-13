//! Execution-context object model and resolver seed.
//!
//! This crate owns inspectable execution-context and task-event runtime
//! contracts. It exposes one [`execution_context::ExecutionContext`] object, a
//! small [`execution_context::ExecutionContextResolver`] that mints contexts
//! for launch-capable surfaces, a canonical [`tasks::TaskEventStream`] for
//! task/test/debug lifecycle truth, and [`tests::TestAttemptAlphaPacket`] for
//! launch-wedge test identity, session, attempt, watch, and imported-CI truth.
//! Downstream event and export lanes carry
//! [`provenance::ExecutionEventProvenance`] so target truth survives after the
//! live run surface is gone.
//!
//! Surfaces (terminal pane, task seed, debug-prep seed, provider/auth entry
//! points, activity center, status bar, support / export flows) read structured
//! execution-context records through this crate; they do not derive runtime
//! truth from terminal state alone or fork local copies of host / target /
//! toolchain identity.
//!
//! The reviewer-facing landing page is
//! [`/docs/runtime/execution_context_seed.md`](../../../docs/runtime/execution_context_seed.md).
//! The cross-tool boundary schema is
//! [`/schemas/runtime/execution_context.schema.json`](../../../schemas/runtime/execution_context.schema.json).

#![doc(html_root_url = "https://docs.rs/aureline-runtime/0.0.0")]

pub mod detectors;
pub mod discovery;
pub mod execution_context;
pub mod language_hosts;
pub mod provenance;
pub mod rerun;
pub mod tasks;
pub mod tests;

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
pub use execution_context::{
    ActorClass, CacheDisposition, CapsuleDriftState, ConfidenceLevel, DegradedFieldReason,
    DegradedFieldRecord, EnvironmentCapsuleRef, ExecutionContext, ExecutionContextEffectClass,
    ExecutionContextExplanation, ExecutionContextReasonCode, ExecutionContextReasonSource,
    ExecutionContextRequest, ExecutionContextResolver, ExecutionContextResolverConfig,
    IdentityMode, InvocationSubject, MixedVersionDrift, MixedVersionDriftState, MixedVersionReason,
    PolicyAndTrust, PrebuildInvalidationReason, PrebuildMetadata, PrebuildReuseState, Provenance,
    ReachabilityState, ResolverInputDecision, ResolverInputField, ResolverInputSource, ScopeClass,
    SurfaceClass, TargetClass, TargetConfidence, TargetConfidenceReason, TargetIdentity,
    ToolchainClass, ToolchainIdentity, TrustState, EXECUTION_CONTEXT_RECORD_KIND,
    EXECUTION_CONTEXT_SCHEMA_VERSION,
};
pub use language_hosts::{
    LanguageHostEventClass, LanguageHostExitReasonClass, LanguageHostIdentity,
    LanguageHostLaunchSpec, LanguageHostRuntimeStateClass, LanguageHostScopeKey,
    LanguageHostSnapshot, LanguageHostSupervisor, LanguageHostSupervisorConfig,
    LanguageHostSupervisorError, LanguageHostSupervisorEvent, LanguageHostSupportPacket,
    LANGUAGE_HOST_SUPERVISION_SCHEMA_VERSION,
};
pub use provenance::{
    dedupe_context_provenance, ExecutionEventProvenance, ExecutionProvenanceEvent,
    ExecutionProvenanceEventClass, ExecutionProvenanceInputDecision,
    ExecutionProvenanceRedactionClass, EXECUTION_EVENT_PROVENANCE_RECORD_KIND,
    EXECUTION_EVENT_PROVENANCE_SCHEMA_VERSION, EXECUTION_PROVENANCE_EVENT_RECORD_KIND,
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
pub use tasks::{
    RawEnvelopeRetentionState, RawTaskEventEnvelope, TaskActivityProjection, TaskArtifactKind,
    TaskBlockReason, TaskConsumerSurfaceClass, TaskDiagnosticSeverity, TaskEvent,
    TaskEventConfidence, TaskEventIdentity, TaskEventKind, TaskEventPayload, TaskEventProvenance,
    TaskEventRedactionClass, TaskEventSourceKind, TaskEventStream, TaskEventStreamError,
    TaskExitStatus, TaskFailureClass, TaskInputClass, TaskInputRequest, TaskOutputStreamClass,
    TaskProgress, TaskShellProjection, TaskState, TaskStateClass, TaskSupportEventRow,
    TaskSupportExport, TaskWedgeClass, RAW_TASK_EVENT_ENVELOPE_RECORD_KIND,
    TASK_ACTIVITY_PROJECTION_RECORD_KIND, TASK_EVENT_RECORD_KIND, TASK_EVENT_SCHEMA_VERSION,
    TASK_EVENT_STREAM_RECORD_KIND, TASK_SHELL_PROJECTION_RECORD_KIND, TASK_STATE_RECORD_KIND,
    TASK_SUPPORT_EXPORT_RECORD_KIND,
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
