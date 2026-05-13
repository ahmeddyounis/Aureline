//! Execution-context object model and resolver seed.
//!
//! This crate owns inspectable execution-context and task-event runtime
//! contracts. It exposes one [`execution_context::ExecutionContext`] object, a
//! small [`execution_context::ExecutionContextResolver`] that mints contexts
//! for launch-capable surfaces, and a canonical [`tasks::TaskEventStream`] for
//! task/test/debug lifecycle truth.
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
pub mod execution_context;
pub mod language_hosts;
pub mod tasks;

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
