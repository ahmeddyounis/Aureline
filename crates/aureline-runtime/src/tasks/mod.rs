//! Canonical task model, typed event stream, and raw-envelope retention.
//!
//! This module owns the runtime task event grammar shared by build, test,
//! debug, terminal-backed, notebook, package, and AI-tool task lanes. Every
//! event carries task, workspace, trace, target, and execution-context identity
//! plus a retained raw adapter envelope so shell rows, activity-center rows,
//! and support exports can consume the same typed stream.

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::provenance::{
    dedupe_context_provenance, ExecutionEventProvenance, ExecutionProvenanceEvent,
    ExecutionProvenanceEventClass,
};

/// Schema version emitted for task event records.
pub const TASK_EVENT_SCHEMA_VERSION: u32 = 1;
/// Stable record-kind tag for one typed task event.
pub const TASK_EVENT_RECORD_KIND: &str = "task_event_record";
/// Stable record-kind tag for one append-only stream snapshot.
pub const TASK_EVENT_STREAM_RECORD_KIND: &str = "task_event_stream_record";
/// Stable record-kind tag for one projected task state.
pub const TASK_STATE_RECORD_KIND: &str = "task_state_record";
/// Stable record-kind tag for one retained raw task-event envelope.
pub const RAW_TASK_EVENT_ENVELOPE_RECORD_KIND: &str = "raw_task_event_envelope_record";
/// Stable record-kind tag for shell consumer projections.
pub const TASK_SHELL_PROJECTION_RECORD_KIND: &str = "task_event_shell_projection_record";
/// Stable record-kind tag for activity-center consumer projections.
pub const TASK_ACTIVITY_PROJECTION_RECORD_KIND: &str = "task_event_activity_projection_record";
/// Stable record-kind tag for support export projections.
pub const TASK_SUPPORT_EXPORT_RECORD_KIND: &str = "task_event_support_export_record";

/// Canonical task lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStateClass {
    /// Work is accepted and waiting to start.
    Queued,
    /// Work is actively executing.
    Running,
    /// Work cannot continue until a non-input dependency clears.
    Blocked,
    /// Work finished successfully.
    Completed,
    /// Work finished unsuccessfully.
    Failed,
    /// Work was cancelled by a user, subsystem, or policy actor.
    Cancelled,
    /// Work is paused on an explicit input request.
    WaitingForInput,
}

impl TaskStateClass {
    /// All task states required by the alpha task model.
    pub const ALL: [Self; 7] = [
        Self::Queued,
        Self::Running,
        Self::Blocked,
        Self::Completed,
        Self::Failed,
        Self::Cancelled,
        Self::WaitingForInput,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Blocked => "blocked",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::WaitingForInput => "waiting_for_input",
        }
    }

    /// Human-readable label for shell and activity projections.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Queued => "Queued",
            Self::Running => "Running",
            Self::Blocked => "Blocked",
            Self::Completed => "Completed",
            Self::Failed => "Failed",
            Self::Cancelled => "Cancelled",
            Self::WaitingForInput => "Waiting for input",
        }
    }

    /// True when this state is terminal but still reviewable.
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }

    /// True when activity-center and shell consumers should expose attention.
    pub const fn needs_attention(self) -> bool {
        matches!(self, Self::Blocked | Self::Failed | Self::WaitingForInput)
    }
}

/// Typed event kind for task streams.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskEventKind {
    /// A task was accepted by the scheduler.
    TaskQueued,
    /// A task process, adapter session, or remote run started.
    TaskStarted,
    /// A task became blocked on a dependency or boundary.
    TaskBlocked,
    /// A task requested input.
    InputRequested,
    /// A task progress counter changed.
    ProgressUpdated,
    /// Output was appended by an adapter or process stream.
    OutputAppended,
    /// A diagnostic was emitted.
    DiagnosticEmitted,
    /// A build, coverage, debug, or log artifact was published.
    ArtifactPublished,
    /// A task entered or refreshed a typed degraded posture.
    DegradedStateReported,
    /// A task completed successfully.
    TaskCompleted,
    /// A task failed.
    TaskFailed,
    /// A task was cancelled.
    TaskCancelled,
}

impl TaskEventKind {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaskQueued => "task_queued",
            Self::TaskStarted => "task_started",
            Self::TaskBlocked => "task_blocked",
            Self::InputRequested => "input_requested",
            Self::ProgressUpdated => "progress_updated",
            Self::OutputAppended => "output_appended",
            Self::DiagnosticEmitted => "diagnostic_emitted",
            Self::ArtifactPublished => "artifact_published",
            Self::DegradedStateReported => "degraded_state_reported",
            Self::TaskCompleted => "task_completed",
            Self::TaskFailed => "task_failed",
            Self::TaskCancelled => "task_cancelled",
        }
    }
}

/// Launch wedge that produced a task event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskWedgeClass {
    /// Build, compile, or bundle work.
    Build,
    /// Test discovery or execution work.
    Test,
    /// Debug launch or attach work.
    Debug,
    /// Terminal-backed task work.
    Terminal,
    /// Package manager, install, or update work.
    Package,
    /// Notebook kernel execution work.
    Notebook,
    /// AI-tool validation or tool execution work.
    AiTool,
    /// Review-lane validation, comparison, or audit work.
    Review,
    /// Generic task runner work.
    Generic,
}

impl TaskWedgeClass {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Build => "build",
            Self::Test => "test",
            Self::Debug => "debug",
            Self::Terminal => "terminal",
            Self::Package => "package",
            Self::Notebook => "notebook",
            Self::AiTool => "ai_tool",
            Self::Review => "review",
            Self::Generic => "generic",
        }
    }
}

/// Origin class for the normalized event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskEventSourceKind {
    /// First-party or first-class adapter.
    Native,
    /// Build Server Protocol source.
    Bsp,
    /// Bazel Build Event Protocol source.
    BazelBep,
    /// Structured tool output.
    StructuredOutput,
    /// Heuristic parser or problem matcher fallback.
    HeuristicParser,
    /// Replay imported from a retained support bundle.
    SupportBundleReplay,
}

impl TaskEventSourceKind {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::Bsp => "bsp",
            Self::BazelBep => "bazel_bep",
            Self::StructuredOutput => "structured_output",
            Self::HeuristicParser => "heuristic_parser",
            Self::SupportBundleReplay => "support_bundle_replay",
        }
    }
}

/// Confidence of a normalized task event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskEventConfidence {
    /// Native or protocol-backed event with exact identity.
    High,
    /// Structured event with a small lossy surface.
    MediumHigh,
    /// Event is useful but not exact enough for strong claims.
    Medium,
    /// Heuristic fallback event.
    Low,
}

impl TaskEventConfidence {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::MediumHigh => "medium_high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

/// Redaction class attached to retained raw adapter envelopes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskEventRedactionClass {
    /// Metadata is safe by default.
    MetadataSafeDefault,
    /// Operator review is required before export.
    OperatorOnlyRestricted,
    /// Internal support review is required before export.
    InternalSupportRestricted,
    /// Signing or digest evidence only.
    SigningEvidenceOnly,
}

impl TaskEventRedactionClass {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::OperatorOnlyRestricted => "operator_only_restricted",
            Self::InternalSupportRestricted => "internal_support_restricted",
            Self::SigningEvidenceOnly => "signing_evidence_only",
        }
    }
}

/// Retention state for the raw adapter-origin envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RawEnvelopeRetentionState {
    /// Redacted payload is retained inline.
    RetainedInlineRedacted,
    /// Raw envelope is retained by reference only.
    RetainedByReference,
    /// Only metadata and digest are retained.
    MetadataDigestOnly,
    /// Policy prohibited retaining the payload body.
    OmittedPolicyProhibited,
}

impl RawEnvelopeRetentionState {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetainedInlineRedacted => "retained_inline_redacted",
            Self::RetainedByReference => "retained_by_reference",
            Self::MetadataDigestOnly => "metadata_digest_only",
            Self::OmittedPolicyProhibited => "omitted_policy_prohibited",
        }
    }
}

/// Output stream carried by an output event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskOutputStreamClass {
    /// Standard output.
    Stdout,
    /// Standard error.
    Stderr,
    /// Adapter or scheduler status stream.
    System,
    /// Protocol-specific adapter stream.
    Adapter,
}

impl TaskOutputStreamClass {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stdout => "stdout",
            Self::Stderr => "stderr",
            Self::System => "system",
            Self::Adapter => "adapter",
        }
    }
}

/// Diagnostic severity emitted by a task adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskDiagnosticSeverity {
    /// Informational diagnostic.
    Info,
    /// Warning diagnostic.
    Warning,
    /// Error diagnostic.
    Error,
    /// Fatal diagnostic.
    Fatal,
}

/// Class of input requested by a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskInputClass {
    /// Plain text input.
    PlainText,
    /// Secret or credential-adjacent input.
    Secret,
    /// Confirmation input.
    Confirmation,
    /// Choice among bounded options.
    Choice,
    /// File or path selection.
    FilePath,
}

/// Reason a task is blocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskBlockReason {
    /// Queue has no capacity for the requested lane.
    QueueCapacity,
    /// Workspace trust review is required.
    TrustReview,
    /// Policy review is required.
    PolicyReview,
    /// Target is unreachable or not admitted.
    TargetUnavailable,
    /// Required tool, target, or capability is missing.
    DependencyMissing,
    /// Explicit approval is required.
    ApprovalRequired,
}

/// Artifact class emitted by a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskArtifactKind {
    /// Build output artifact.
    BuildOutput,
    /// Coverage artifact.
    Coverage,
    /// Test report artifact.
    TestReport,
    /// Log slice artifact.
    LogSlice,
    /// Profiling artifact.
    Profile,
    /// Debug artifact.
    DebugArtifact,
}

/// Typed reason recorded with a `degraded_state_reported` event.
///
/// Captures the partial- or degraded-state semantics that earlier task lanes
/// expressed only through console heuristics, so shell rows, activity-center
/// rows, and support exports can read the same closed vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskDegradationReason {
    /// Output is reaching consumers but progress counters are unavailable.
    ProgressUnavailable,
    /// Adapter advertised fewer capabilities than the canonical request set.
    AdapterCapabilityDropped,
    /// Heuristic or fallback parser is filling in for a missing structured
    /// adapter source.
    FallbackParserActive,
    /// Output bytes were truncated by retention or policy.
    OutputTruncated,
    /// Only a subset of output streams or artifacts is reaching consumers.
    PartialOutputOnly,
    /// Target became unreachable but the task is expected to resume on its own.
    TransientTargetUnreachable,
    /// Policy is hiding part of the run from this surface.
    PolicyPartialVisibility,
}

impl TaskDegradationReason {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProgressUnavailable => "progress_unavailable",
            Self::AdapterCapabilityDropped => "adapter_capability_dropped",
            Self::FallbackParserActive => "fallback_parser_active",
            Self::OutputTruncated => "output_truncated",
            Self::PartialOutputOnly => "partial_output_only",
            Self::TransientTargetUnreachable => "transient_target_unreachable",
            Self::PolicyPartialVisibility => "policy_partial_visibility",
        }
    }
}

/// Failure classification for terminal states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskFailureClass {
    /// Non-zero process exit code.
    ExitCode,
    /// Process or adapter signal.
    Signal,
    /// Adapter failed independently from the launched task.
    AdapterError,
    /// Target connection was lost.
    TargetLost,
    /// Policy denied or revoked the action.
    PolicyDenied,
    /// User cancelled the task.
    CancelledByUser,
    /// Failure could not be classified.
    Unknown,
}

/// Consumer surfaces that read the typed task stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskConsumerSurfaceClass {
    /// Shell status/run surfaces.
    Shell,
    /// Durable activity-center rows.
    ActivityCenter,
    /// Support bundle export.
    SupportBundleExport,
}

/// Identity block repeated on every task event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskEventIdentity {
    /// Stable task id.
    pub task_id: String,
    /// Stable run id for this launch.
    pub run_id: String,
    /// Attempt id within the run.
    pub attempt_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Trace id that joins cross-service events.
    pub trace_id: String,
    /// Execution-context id resolved before dispatch.
    pub execution_context_id: String,
    /// Build target, test target, debug config, or task target id.
    pub target_id: String,
    /// Wedge that produced this task.
    pub wedge: TaskWedgeClass,
}

/// Provenance for the normalized task event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskEventProvenance {
    /// Source kind for this event.
    pub source_kind: TaskEventSourceKind,
    /// Adapter or producer id.
    pub source_adapter_id: String,
    /// Adapter version.
    pub adapter_version: String,
    /// Workspace revision if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_revision: Option<String>,
    /// Confidence of the normalized event.
    pub confidence: TaskEventConfidence,
    /// Execution-context provenance projected from the canonical runtime context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_provenance: Option<ExecutionEventProvenance>,
}

/// Retained raw adapter-origin envelope.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawTaskEventEnvelope {
    /// Stable record kind.
    pub record_kind: String,
    /// Stable raw-envelope reference.
    pub raw_envelope_ref: String,
    /// Task id copied from the typed event.
    pub task_id: String,
    /// Workspace id copied from the typed event.
    pub workspace_id: String,
    /// Trace id copied from the typed event.
    pub trace_id: String,
    /// Source kind copied from the typed event.
    pub source_kind: TaskEventSourceKind,
    /// Adapter-origin record id.
    pub adapter_origin_event_id: String,
    /// Redaction posture for this retained envelope.
    pub redaction_class: TaskEventRedactionClass,
    /// Retention state for raw material.
    pub retention_state: RawEnvelopeRetentionState,
    /// Digest of the adapter-origin payload.
    pub payload_digest: String,
    /// Redacted raw payload, when retained inline.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retained_payload: Option<Value>,
    /// Retention timestamp.
    pub retained_at: String,
    /// Fields needed to reconstruct the adapter-origin record.
    pub reconstruction_fields: Vec<String>,
}

impl RawTaskEventEnvelope {
    /// Returns true when identity fields match the typed event.
    pub fn matches_event(&self, event: &TaskEvent) -> bool {
        self.record_kind == RAW_TASK_EVENT_ENVELOPE_RECORD_KIND
            && self.task_id == event.identity.task_id
            && self.workspace_id == event.identity.workspace_id
            && self.trace_id == event.identity.trace_id
            && self.source_kind == event.provenance.source_kind
    }
}

/// Progress payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskProgress {
    /// Completed unit count.
    pub completed_units: u32,
    /// Total unit count.
    pub total_units: u32,
    /// Unit label, such as `tests` or `steps`.
    pub unit_label: String,
    /// Export-safe progress label.
    pub label: String,
}

impl TaskProgress {
    /// Returns a deterministic integer percentage.
    pub fn percent(&self) -> u32 {
        if self.total_units == 0 {
            0
        } else {
            (u64::from(self.completed_units) * 100 / u64::from(self.total_units)) as u32
        }
    }
}

/// Input request payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskInputRequest {
    /// Stable input request id.
    pub request_id: String,
    /// Input class.
    pub input_class: TaskInputClass,
    /// Actor or adapter asking for input.
    pub source_label: String,
    /// Scope affected by the input.
    pub scope_ref: String,
    /// Optional expiry timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// Exit status payload for terminal states.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskExitStatus {
    /// Process exit code, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    /// Failure class, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_class: Option<TaskFailureClass>,
}

/// Typed payload carried by one task event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "payload_kind", rename_all = "snake_case")]
pub enum TaskEventPayload {
    /// Lifecycle state change.
    Lifecycle {
        /// Optional lifecycle reason.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        lifecycle_reason: Option<String>,
        /// Optional terminal exit status.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        exit_status: Option<TaskExitStatus>,
    },
    /// Progress state change.
    Progress {
        /// Progress counters.
        progress: TaskProgress,
    },
    /// Output chunk reference.
    Output {
        /// Output stream class.
        stream: TaskOutputStreamClass,
        /// Opaque chunk reference.
        chunk_ref: String,
        /// Line count represented by the chunk.
        line_count: u32,
        /// Byte count represented by the chunk.
        byte_count: u32,
    },
    /// Diagnostic reference.
    Diagnostic {
        /// Stable diagnostic reference.
        diagnostic_ref: String,
        /// Diagnostic severity.
        severity: TaskDiagnosticSeverity,
        /// Tool that emitted the diagnostic.
        tool_ref: String,
    },
    /// Published artifact reference.
    Artifact {
        /// Stable artifact reference.
        artifact_ref: String,
        /// Artifact kind.
        artifact_kind: TaskArtifactKind,
        /// Retention reference for the artifact.
        retention_ref: String,
    },
    /// Input request.
    InputRequest {
        /// Input request details.
        request: TaskInputRequest,
    },
    /// Blocked state detail.
    Blocked {
        /// Block reason.
        reason: TaskBlockReason,
        /// Optional unblock or review reference.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        unblock_ref: Option<String>,
    },
    /// Degraded or partial state reported as a typed event so consumers no
    /// longer have to parse free-form console output.
    Degraded {
        /// Typed degradation reason.
        reason: TaskDegradationReason,
        /// Export-safe label describing the affected scope (stream, surface,
        /// or capability) for shell and activity rows.
        scope_label: String,
        /// Optional pointer to a recovery action, doctor row, or operator
        /// note that explains how to restore full fidelity.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        recovery_hint_ref: Option<String>,
    },
}

impl TaskEventPayload {
    /// Stable payload-kind token.
    pub const fn payload_kind(&self) -> &'static str {
        match self {
            Self::Lifecycle { .. } => "lifecycle",
            Self::Progress { .. } => "progress",
            Self::Output { .. } => "output",
            Self::Diagnostic { .. } => "diagnostic",
            Self::Artifact { .. } => "artifact",
            Self::InputRequest { .. } => "input_request",
            Self::Blocked { .. } => "blocked",
            Self::Degraded { .. } => "degraded",
        }
    }

    /// Returns the typed degradation reason, if this payload carries one.
    pub const fn degradation_reason(&self) -> Option<TaskDegradationReason> {
        match self {
            Self::Degraded { reason, .. } => Some(*reason),
            _ => None,
        }
    }

    fn progress(&self) -> Option<&TaskProgress> {
        match self {
            Self::Progress { progress } => Some(progress),
            _ => None,
        }
    }

    fn input_request(&self) -> Option<&TaskInputRequest> {
        match self {
            Self::InputRequest { request } => Some(request),
            _ => None,
        }
    }

    fn block_reason(&self) -> Option<TaskBlockReason> {
        match self {
            Self::Blocked { reason, .. } => Some(*reason),
            _ => None,
        }
    }

    fn degradation(&self) -> Option<(TaskDegradationReason, String, Option<String>)> {
        match self {
            Self::Degraded {
                reason,
                scope_label,
                recovery_hint_ref,
            } => Some((*reason, scope_label.clone(), recovery_hint_ref.clone())),
            _ => None,
        }
    }

    fn exit_status(&self) -> Option<&TaskExitStatus> {
        match self {
            Self::Lifecycle { exit_status, .. } => exit_status.as_ref(),
            _ => None,
        }
    }
}

/// One typed task event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskEvent {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub task_event_schema_version: u32,
    /// Unique event id.
    pub event_id: String,
    /// Stream id this event belongs to.
    pub stream_id: String,
    /// Monotonic sequence within the stream.
    pub stream_sequence: u64,
    /// Identity block.
    pub identity: TaskEventIdentity,
    /// Event kind.
    pub event_kind: TaskEventKind,
    /// State after applying this event.
    pub state_after: TaskStateClass,
    /// Event timestamp.
    pub occurred_at: String,
    /// Export-safe summary.
    pub summary: String,
    /// Typed event payload.
    pub payload: TaskEventPayload,
    /// Normalization provenance.
    pub provenance: TaskEventProvenance,
    /// Retained raw adapter envelope.
    pub raw_envelope: RawTaskEventEnvelope,
}

impl TaskEvent {
    /// True when the event leaves the task in a terminal state.
    pub fn is_terminal(&self) -> bool {
        self.state_after.is_terminal()
    }

    /// Stable payload-kind token.
    pub fn payload_kind(&self) -> &'static str {
        self.payload.payload_kind()
    }
}

/// Current state for one task.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskState {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub task_event_schema_version: u32,
    /// Identity block.
    pub identity: TaskEventIdentity,
    /// Current state.
    pub current_state: TaskStateClass,
    /// Current state token.
    pub state_token: String,
    /// Last event id applied to this state.
    pub last_event_id: String,
    /// First event timestamp.
    pub first_seen_at: String,
    /// Last event timestamp.
    pub last_observed_at: String,
    /// Last progress payload, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<TaskProgress>,
    /// Current input request, when waiting for input.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_request: Option<TaskInputRequest>,
    /// Current block reason, when blocked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_reason: Option<TaskBlockReason>,
    /// Active degradation reason, when a typed degraded posture is in effect.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degradation_reason: Option<TaskDegradationReason>,
    /// Export-safe label describing the degraded scope, when set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degradation_scope_label: Option<String>,
    /// Optional recovery hint reference for the active degraded posture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degradation_recovery_hint_ref: Option<String>,
    /// Exit status, when terminal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_status: Option<TaskExitStatus>,
    /// Export-safe summary.
    pub summary: String,
}

impl TaskState {
    fn apply(previous: Option<&Self>, event: &TaskEvent) -> Self {
        let progress = event
            .payload
            .progress()
            .cloned()
            .or_else(|| previous.and_then(|state| state.progress.clone()));
        let input_request = if event.state_after == TaskStateClass::WaitingForInput {
            event
                .payload
                .input_request()
                .cloned()
                .or_else(|| previous.and_then(|state| state.input_request.clone()))
        } else {
            None
        };
        let block_reason = if event.state_after == TaskStateClass::Blocked {
            event
                .payload
                .block_reason()
                .or_else(|| previous.and_then(|state| state.block_reason))
        } else {
            None
        };
        let (degradation_reason, degradation_scope_label, degradation_recovery_hint_ref) =
            if event.state_after.is_terminal() {
                (None, None, None)
            } else if let Some((reason, scope, hint)) = event.payload.degradation() {
                (Some(reason), Some(scope), hint)
            } else {
                (
                    previous.and_then(|state| state.degradation_reason),
                    previous.and_then(|state| state.degradation_scope_label.clone()),
                    previous.and_then(|state| state.degradation_recovery_hint_ref.clone()),
                )
            };
        let exit_status = if event.state_after.is_terminal() {
            event
                .payload
                .exit_status()
                .cloned()
                .or_else(|| previous.and_then(|state| state.exit_status.clone()))
        } else {
            None
        };

        Self {
            record_kind: TASK_STATE_RECORD_KIND.to_owned(),
            task_event_schema_version: TASK_EVENT_SCHEMA_VERSION,
            identity: event.identity.clone(),
            current_state: event.state_after,
            state_token: event.state_after.as_str().to_owned(),
            last_event_id: event.event_id.clone(),
            first_seen_at: previous
                .map(|state| state.first_seen_at.clone())
                .unwrap_or_else(|| event.occurred_at.clone()),
            last_observed_at: event.occurred_at.clone(),
            progress,
            input_request,
            block_reason,
            degradation_reason,
            degradation_scope_label,
            degradation_recovery_hint_ref,
            exit_status,
            summary: event.summary.clone(),
        }
    }

    /// True when the task currently carries a typed degraded posture.
    pub const fn is_degraded(&self) -> bool {
        self.degradation_reason.is_some()
    }
}

/// Append-only stream of typed task events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskEventStream {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub task_event_schema_version: u32,
    /// Stream id.
    pub stream_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Trace id.
    pub trace_id: String,
    /// Ordered task events.
    pub events: Vec<TaskEvent>,
    /// Current state by task id.
    #[serde(default)]
    pub task_states: BTreeMap<String, TaskState>,
}

impl TaskEventStream {
    /// Builds an empty task-event stream.
    pub fn new(
        stream_id: impl Into<String>,
        workspace_id: impl Into<String>,
        trace_id: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: TASK_EVENT_STREAM_RECORD_KIND.to_owned(),
            task_event_schema_version: TASK_EVENT_SCHEMA_VERSION,
            stream_id: stream_id.into(),
            workspace_id: workspace_id.into(),
            trace_id: trace_id.into(),
            events: Vec::new(),
            task_states: BTreeMap::new(),
        }
    }

    /// Replays events into a stream, validating identity and ordering.
    ///
    /// # Errors
    ///
    /// Returns [`TaskEventStreamError`] when an event belongs to another
    /// stream, workspace, trace, or is not strictly ordered.
    pub fn from_events(
        stream_id: impl Into<String>,
        workspace_id: impl Into<String>,
        trace_id: impl Into<String>,
        events: impl IntoIterator<Item = TaskEvent>,
    ) -> Result<Self, TaskEventStreamError> {
        let mut stream = Self::new(stream_id, workspace_id, trace_id);
        for event in events {
            stream.append(event)?;
        }
        Ok(stream)
    }

    /// Appends one typed event and updates the task state index.
    ///
    /// # Errors
    ///
    /// Returns [`TaskEventStreamError`] when event identity, raw-envelope
    /// identity, or stream ordering does not match this stream.
    pub fn append(&mut self, event: TaskEvent) -> Result<(), TaskEventStreamError> {
        self.validate_event(&event)?;
        let key = event.identity.task_id.clone();
        let next_state = TaskState::apply(self.task_states.get(&key), &event);
        self.task_states.insert(key, next_state);
        self.events.push(event);
        Ok(())
    }

    /// Returns the current task state, if known.
    pub fn state_for_task(&self, task_id: &str) -> Option<&TaskState> {
        self.task_states.get(task_id)
    }

    /// Projects events for shell consumers.
    pub fn shell_projection(&self) -> Vec<TaskShellProjection> {
        self.events
            .iter()
            .map(TaskShellProjection::from_event)
            .collect()
    }

    /// Projects one durable activity row per current task state.
    pub fn activity_projection(&self) -> Vec<TaskActivityProjection> {
        self.task_states
            .values()
            .map(TaskActivityProjection::from_state)
            .collect()
    }

    /// Projects a support-export packet from the retained event stream.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> TaskSupportExport {
        let export_id = export_id.into();
        let generated_at = generated_at.into();
        let context_provenance = dedupe_context_provenance(
            self.events
                .iter()
                .filter_map(|event| event.provenance.context_provenance.clone()),
        );
        let context_provenance_events = context_provenance
            .iter()
            .map(|provenance| {
                ExecutionProvenanceEvent::new(
                    format!(
                        "execution-provenance-event:support-export:{}:{}",
                        export_id, provenance.context_provenance_id
                    ),
                    ExecutionProvenanceEventClass::SupportExport,
                    export_id.clone(),
                    generated_at.clone(),
                    provenance.clone(),
                )
            })
            .collect();
        TaskSupportExport {
            record_kind: TASK_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            task_event_schema_version: TASK_EVENT_SCHEMA_VERSION,
            export_id,
            workspace_id: self.workspace_id.clone(),
            trace_id: self.trace_id.clone(),
            generated_at,
            consumer_surfaces: vec![
                TaskConsumerSurfaceClass::Shell,
                TaskConsumerSurfaceClass::ActivityCenter,
                TaskConsumerSurfaceClass::SupportBundleExport,
            ],
            events: self
                .events
                .iter()
                .map(TaskSupportEventRow::from_event)
                .collect(),
            raw_envelopes: self
                .events
                .iter()
                .map(|event| event.raw_envelope.clone())
                .collect(),
            context_provenance,
            context_provenance_events,
        }
    }

    fn validate_event(&self, event: &TaskEvent) -> Result<(), TaskEventStreamError> {
        if event.record_kind != TASK_EVENT_RECORD_KIND {
            return Err(TaskEventStreamError::InvalidRecordKind {
                expected: TASK_EVENT_RECORD_KIND.to_owned(),
                actual: event.record_kind.clone(),
            });
        }
        if event.task_event_schema_version != TASK_EVENT_SCHEMA_VERSION {
            return Err(TaskEventStreamError::UnsupportedSchemaVersion {
                expected: TASK_EVENT_SCHEMA_VERSION,
                actual: event.task_event_schema_version,
            });
        }
        if event.stream_id != self.stream_id {
            return Err(TaskEventStreamError::StreamMismatch {
                expected: self.stream_id.clone(),
                actual: event.stream_id.clone(),
            });
        }
        if event.identity.workspace_id != self.workspace_id {
            return Err(TaskEventStreamError::WorkspaceMismatch {
                expected: self.workspace_id.clone(),
                actual: event.identity.workspace_id.clone(),
            });
        }
        if event.identity.trace_id != self.trace_id {
            return Err(TaskEventStreamError::TraceMismatch {
                expected: self.trace_id.clone(),
                actual: event.identity.trace_id.clone(),
            });
        }
        if let Some(last) = self.events.last() {
            if event.stream_sequence <= last.stream_sequence {
                return Err(TaskEventStreamError::SequenceNotIncreasing {
                    previous: last.stream_sequence,
                    next: event.stream_sequence,
                });
            }
        }
        if !event.raw_envelope.matches_event(event) {
            return Err(TaskEventStreamError::RawEnvelopeIdentityMismatch {
                event_id: event.event_id.clone(),
                raw_envelope_ref: event.raw_envelope.raw_envelope_ref.clone(),
            });
        }
        if let Some(context_provenance) = &event.provenance.context_provenance {
            if !context_provenance.matches_event_identity(
                &event.identity.execution_context_id,
                &event.identity.target_id,
            ) {
                return Err(TaskEventStreamError::ContextProvenanceIdentityMismatch {
                    event_id: event.event_id.clone(),
                    context_provenance_ref: context_provenance.context_provenance_id.clone(),
                });
            }
        }
        Ok(())
    }
}

/// Error raised while ingesting a task-event stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskEventStreamError {
    /// Event record kind was not accepted.
    InvalidRecordKind { expected: String, actual: String },
    /// Event schema version was not accepted.
    UnsupportedSchemaVersion { expected: u32, actual: u32 },
    /// Event stream id did not match.
    StreamMismatch { expected: String, actual: String },
    /// Event workspace id did not match.
    WorkspaceMismatch { expected: String, actual: String },
    /// Event trace id did not match.
    TraceMismatch { expected: String, actual: String },
    /// Event sequence was not strictly increasing.
    SequenceNotIncreasing { previous: u64, next: u64 },
    /// Raw envelope identity did not match the typed event.
    RawEnvelopeIdentityMismatch {
        event_id: String,
        raw_envelope_ref: String,
    },
    /// Execution-context provenance did not match the typed event identity.
    ContextProvenanceIdentityMismatch {
        event_id: String,
        context_provenance_ref: String,
    },
}

impl fmt::Display for TaskEventStreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRecordKind { expected, actual } => {
                write!(
                    f,
                    "task event record kind {actual} did not match {expected}"
                )
            }
            Self::UnsupportedSchemaVersion { expected, actual } => {
                write!(
                    f,
                    "task event schema version {actual} did not match {expected}"
                )
            }
            Self::StreamMismatch { expected, actual } => {
                write!(f, "task event stream {actual} did not match {expected}")
            }
            Self::WorkspaceMismatch { expected, actual } => {
                write!(f, "task event workspace {actual} did not match {expected}")
            }
            Self::TraceMismatch { expected, actual } => {
                write!(f, "task event trace {actual} did not match {expected}")
            }
            Self::SequenceNotIncreasing { previous, next } => write!(
                f,
                "task event sequence {next} must be greater than previous sequence {previous}"
            ),
            Self::RawEnvelopeIdentityMismatch {
                event_id,
                raw_envelope_ref,
            } => write!(
                f,
                "task event {event_id} raw envelope {raw_envelope_ref} did not match event identity"
            ),
            Self::ContextProvenanceIdentityMismatch {
                event_id,
                context_provenance_ref,
            } => write!(
                f,
                "task event {event_id} context provenance {context_provenance_ref} did not match event identity"
            ),
        }
    }
}

impl std::error::Error for TaskEventStreamError {}

/// Shell projection for one task event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskShellProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub task_event_schema_version: u32,
    /// Event id.
    pub event_id: String,
    /// Task id.
    pub task_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Execution-context ref.
    pub execution_context_ref: String,
    /// Target id.
    pub target_id: String,
    /// Wedge class.
    pub wedge: TaskWedgeClass,
    /// State after the event.
    pub state_after: TaskStateClass,
    /// State token.
    pub state_token: String,
    /// Event kind.
    pub event_kind: TaskEventKind,
    /// Event kind token.
    pub event_kind_token: String,
    /// Payload kind token.
    pub payload_kind_token: String,
    /// Export-safe summary.
    pub summary: String,
    /// Raw envelope ref retained for support.
    pub raw_envelope_ref: String,
    /// True when the task needs attention.
    pub needs_attention: bool,
    /// True when the task is terminal.
    pub is_terminal: bool,
    /// Typed degradation reason carried on this event, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degradation_reason: Option<TaskDegradationReason>,
    /// Inspect-context action ref.
    pub inspect_execution_context_action_ref: String,
}

impl TaskShellProjection {
    /// Builds a shell projection from one typed event.
    pub fn from_event(event: &TaskEvent) -> Self {
        let degradation_reason = event.payload.degradation_reason();
        Self {
            record_kind: TASK_SHELL_PROJECTION_RECORD_KIND.to_owned(),
            task_event_schema_version: TASK_EVENT_SCHEMA_VERSION,
            event_id: event.event_id.clone(),
            task_id: event.identity.task_id.clone(),
            workspace_id: event.identity.workspace_id.clone(),
            execution_context_ref: event.identity.execution_context_id.clone(),
            target_id: event.identity.target_id.clone(),
            wedge: event.identity.wedge,
            state_after: event.state_after,
            state_token: event.state_after.as_str().to_owned(),
            event_kind: event.event_kind,
            event_kind_token: event.event_kind.as_str().to_owned(),
            payload_kind_token: event.payload_kind().to_owned(),
            summary: event.summary.clone(),
            raw_envelope_ref: event.raw_envelope.raw_envelope_ref.clone(),
            needs_attention: event.state_after.needs_attention() || degradation_reason.is_some(),
            is_terminal: event.state_after.is_terminal(),
            degradation_reason,
            inspect_execution_context_action_ref: format!(
                "action:execution-context:inspect:{}",
                event.identity.execution_context_id
            ),
        }
    }
}

/// Activity-center projection for one current task state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskActivityProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub task_event_schema_version: u32,
    /// Activity row id.
    pub activity_row_id: String,
    /// Canonical event id.
    pub canonical_event_id: String,
    /// Task id.
    pub task_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Trace id.
    pub trace_id: String,
    /// Execution-context ref.
    pub execution_context_ref: String,
    /// Target id.
    pub target_id: String,
    /// Wedge class.
    pub wedge: TaskWedgeClass,
    /// Current state.
    pub state: TaskStateClass,
    /// State token.
    pub state_token: String,
    /// State label.
    pub state_label: String,
    /// Summary label.
    pub summary_label: String,
    /// Last observed timestamp.
    pub last_observed_at: String,
    /// Progress, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<TaskProgress>,
    /// Current input request, when waiting for input.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_request: Option<TaskInputRequest>,
    /// Current block reason, when blocked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_reason: Option<TaskBlockReason>,
    /// Active degradation reason, when a typed degraded posture is in effect.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degradation_reason: Option<TaskDegradationReason>,
    /// Export-safe label describing the degraded scope, when set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degradation_scope_label: Option<String>,
    /// Recovery hint reference for the active degraded posture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degradation_recovery_hint_ref: Option<String>,
    /// True when terminal.
    pub is_terminal: bool,
    /// True when the activity center should place the row in an attention partition.
    pub needs_attention: bool,
    /// True when a typed degraded posture is currently in effect.
    pub is_degraded: bool,
    /// Exact reopen identity for the durable row.
    pub exact_reopen_identity_ref: String,
    /// Support-pack item id.
    pub support_pack_item_id: String,
    /// True because raw private material is retained behind redaction refs.
    pub raw_private_material_excluded: bool,
}

impl TaskActivityProjection {
    /// Builds an activity-center projection from a current task state.
    pub fn from_state(state: &TaskState) -> Self {
        let exact_reopen_identity_ref = format!("task:{}", state.identity.task_id);
        let is_degraded = state.is_degraded();
        Self {
            record_kind: TASK_ACTIVITY_PROJECTION_RECORD_KIND.to_owned(),
            task_event_schema_version: TASK_EVENT_SCHEMA_VERSION,
            activity_row_id: format!("activity:task:{}", state.identity.task_id),
            canonical_event_id: format!("task:event:{}", state.identity.task_id),
            task_id: state.identity.task_id.clone(),
            workspace_id: state.identity.workspace_id.clone(),
            trace_id: state.identity.trace_id.clone(),
            execution_context_ref: state.identity.execution_context_id.clone(),
            target_id: state.identity.target_id.clone(),
            wedge: state.identity.wedge,
            state: state.current_state,
            state_token: state.current_state.as_str().to_owned(),
            state_label: state.current_state.label().to_owned(),
            summary_label: state.summary.clone(),
            last_observed_at: state.last_observed_at.clone(),
            progress: state.progress.clone(),
            input_request: state.input_request.clone(),
            block_reason: state.block_reason,
            degradation_reason: state.degradation_reason,
            degradation_scope_label: state.degradation_scope_label.clone(),
            degradation_recovery_hint_ref: state.degradation_recovery_hint_ref.clone(),
            is_terminal: state.current_state.is_terminal(),
            needs_attention: state.current_state.needs_attention() || is_degraded,
            is_degraded,
            exact_reopen_identity_ref,
            support_pack_item_id: format!("support.item.task_event.{}", state.identity.task_id),
            raw_private_material_excluded: true,
        }
    }
}

/// Support-export event row.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskSupportEventRow {
    /// Event id.
    pub event_id: String,
    /// Task id.
    pub task_id: String,
    /// Run id.
    pub run_id: String,
    /// Attempt id.
    pub attempt_id: String,
    /// Execution-context ref.
    pub execution_context_ref: String,
    /// Target id.
    pub target_id: String,
    /// Wedge class.
    pub wedge: TaskWedgeClass,
    /// Event kind.
    pub event_kind: TaskEventKind,
    /// State after the event.
    pub state_after: TaskStateClass,
    /// Source kind.
    pub source_kind: TaskEventSourceKind,
    /// Confidence.
    pub confidence: TaskEventConfidence,
    /// Redaction class.
    pub redaction_class: TaskEventRedactionClass,
    /// Raw envelope ref.
    pub raw_envelope_ref: String,
    /// Payload digest from the raw envelope.
    pub payload_digest: String,
    /// Execution-context provenance projection ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_provenance_ref: Option<String>,
    /// Typed degradation reason carried on this event, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degradation_reason: Option<TaskDegradationReason>,
    /// Export-safe summary.
    pub summary: String,
}

impl TaskSupportEventRow {
    /// Builds a support-export row from one typed event.
    pub fn from_event(event: &TaskEvent) -> Self {
        Self {
            event_id: event.event_id.clone(),
            task_id: event.identity.task_id.clone(),
            run_id: event.identity.run_id.clone(),
            attempt_id: event.identity.attempt_id.clone(),
            execution_context_ref: event.identity.execution_context_id.clone(),
            target_id: event.identity.target_id.clone(),
            wedge: event.identity.wedge,
            event_kind: event.event_kind,
            state_after: event.state_after,
            source_kind: event.provenance.source_kind,
            confidence: event.provenance.confidence,
            redaction_class: event.raw_envelope.redaction_class,
            raw_envelope_ref: event.raw_envelope.raw_envelope_ref.clone(),
            payload_digest: event.raw_envelope.payload_digest.clone(),
            context_provenance_ref: event
                .provenance
                .context_provenance
                .as_ref()
                .map(|provenance| provenance.context_provenance_id.clone()),
            degradation_reason: event.payload.degradation_reason(),
            summary: event.summary.clone(),
        }
    }
}

/// Support-export projection for task events and retained raw envelopes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub task_event_schema_version: u32,
    /// Export id.
    pub export_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Trace id.
    pub trace_id: String,
    /// Export timestamp.
    pub generated_at: String,
    /// Consumer surfaces this packet is shaped for.
    pub consumer_surfaces: Vec<TaskConsumerSurfaceClass>,
    /// Typed support event rows.
    pub events: Vec<TaskSupportEventRow>,
    /// Raw adapter-origin envelopes retained for reconstruction.
    pub raw_envelopes: Vec<RawTaskEventEnvelope>,
    /// Execution-context provenance objects needed to inspect target truth.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context_provenance: Vec<ExecutionEventProvenance>,
    /// Support-export provenance events carrying the same context objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub context_provenance_events: Vec<ExecutionProvenanceEvent>,
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn identity(task_id: &str, wedge: TaskWedgeClass) -> TaskEventIdentity {
        TaskEventIdentity {
            task_id: task_id.to_owned(),
            run_id: format!("run:{task_id}:1"),
            attempt_id: format!("attempt:{task_id}:1"),
            workspace_id: "workspace:payments".to_owned(),
            trace_id: "trace:task-event-alpha".to_owned(),
            execution_context_id: "exec:workspace:task:0".to_owned(),
            target_id: format!("target:{task_id}"),
            wedge,
        }
    }

    fn context_provenance(identity: &TaskEventIdentity) -> ExecutionEventProvenance {
        ExecutionEventProvenance {
            record_kind: crate::provenance::EXECUTION_EVENT_PROVENANCE_RECORD_KIND.to_owned(),
            schema_version: crate::provenance::EXECUTION_EVENT_PROVENANCE_SCHEMA_VERSION,
            context_provenance_id: format!("ctx-prov:{}", identity.execution_context_id),
            execution_context_ref: identity.execution_context_id.clone(),
            provenance_record_ref: format!("prov:{}", identity.execution_context_id),
            recorded_at: "2026-05-13T16:00:00Z".to_owned(),
            resolver_version: "task-event-test".to_owned(),
            workspace_id: identity.workspace_id.clone(),
            command_id: "task.run.fixture".to_owned(),
            surface: crate::SurfaceClass::Task,
            surface_token: "task".to_owned(),
            actor_class: crate::ActorClass::UserCommand,
            actor_class_token: "user_command".to_owned(),
            target_id: identity.target_id.clone(),
            target_class: crate::TargetClass::LocalHost,
            target_class_token: "local_host".to_owned(),
            reachability_state: crate::ReachabilityState::Reachable,
            reachability_state_token: "reachable".to_owned(),
            boundary_cue_visible: false,
            target_confidence_level: crate::ConfidenceLevel::High,
            target_confidence_level_token: "high".to_owned(),
            target_confidence_reason_tokens: vec!["exact_local_target".to_owned()],
            toolchain_class: crate::ToolchainClass::BuildDriverRuntime,
            toolchain_class_token: "build_driver_runtime".to_owned(),
            toolchain_id: "toolchain:build-driver".to_owned(),
            resolved_version: "seed".to_owned(),
            environment_capsule_ref: "capsule:task-event-test".to_owned(),
            environment_capsule_hash: "sha256:capsule".to_owned(),
            environment_capsule_drift_token: "in_sync".to_owned(),
            working_directory_present: true,
            working_directory_digest: Some(
                "sha256:0000000000000000000000000000000000000000000000000000000000000001"
                    .to_owned(),
            ),
            prebuild_reuse_state: crate::PrebuildReuseState::NotApplicable,
            prebuild_reuse_state_token: "not_applicable".to_owned(),
            prebuild_snapshot_ref: None,
            prebuild_compatibility_fingerprint: "prebuild:not_applicable".to_owned(),
            prebuild_invalidation_reason_token: None,
            trust_state: crate::TrustState::Trusted,
            trust_state_token: "trusted".to_owned(),
            identity_mode_token: "account_free_local".to_owned(),
            policy_epoch: 1,
            scope_class: crate::ScopeClass::CurrentRoot,
            scope_class_token: "current_root".to_owned(),
            cache_disposition: crate::CacheDisposition::Cold,
            cache_disposition_token: "cold".to_owned(),
            mixed_version_state_token: "not_applicable".to_owned(),
            mixed_version_reason_token: "local_only".to_owned(),
            client_protocol: "runtime-seed".to_owned(),
            helper_protocol: None,
            input_decisions: Vec::new(),
            degraded_field_count: 0,
            degraded_field_tokens: Vec::new(),
            explanation_reason_code_tokens: vec!["shared_context_contract".to_owned()],
            redaction_class: crate::ExecutionProvenanceRedactionClass::MetadataSafeDefault,
            redaction_safe: true,
            reconstruction_fields: vec![
                "execution_context_ref".to_owned(),
                "target_id".to_owned(),
                "target_class_token".to_owned(),
            ],
        }
    }

    // Keep the fixture helper field-shaped so tests vary task identity, state,
    // source, payload, and redaction dimensions independently.
    #[allow(clippy::too_many_arguments)]
    fn event(
        seq: u64,
        task_id: &str,
        wedge: TaskWedgeClass,
        kind: TaskEventKind,
        state: TaskStateClass,
        payload: TaskEventPayload,
        source_kind: TaskEventSourceKind,
        redaction_class: TaskEventRedactionClass,
    ) -> TaskEvent {
        let identity = identity(task_id, wedge);
        let event_id = format!("event:{task_id}:{seq}");
        TaskEvent {
            record_kind: TASK_EVENT_RECORD_KIND.to_owned(),
            task_event_schema_version: TASK_EVENT_SCHEMA_VERSION,
            event_id: event_id.clone(),
            stream_id: "stream:task-event-alpha".to_owned(),
            stream_sequence: seq,
            identity: identity.clone(),
            event_kind: kind,
            state_after: state,
            occurred_at: format!("2026-05-13T16:00:{seq:02}Z"),
            summary: format!("{} {}", task_id, state.as_str()),
            payload,
            provenance: TaskEventProvenance {
                source_kind,
                source_adapter_id: format!("adapter:{}", source_kind.as_str()),
                adapter_version: "1.0.0".to_owned(),
                workspace_revision: Some("rev:abc123".to_owned()),
                confidence: match source_kind {
                    TaskEventSourceKind::HeuristicParser => TaskEventConfidence::Low,
                    TaskEventSourceKind::StructuredOutput => TaskEventConfidence::MediumHigh,
                    _ => TaskEventConfidence::High,
                },
                context_provenance: Some(context_provenance(&identity)),
            },
            raw_envelope: RawTaskEventEnvelope {
                record_kind: RAW_TASK_EVENT_ENVELOPE_RECORD_KIND.to_owned(),
                raw_envelope_ref: format!("raw:{event_id}"),
                task_id: identity.task_id,
                workspace_id: identity.workspace_id,
                trace_id: identity.trace_id,
                source_kind,
                adapter_origin_event_id: format!("adapter-origin:{event_id}"),
                redaction_class,
                retention_state: RawEnvelopeRetentionState::RetainedInlineRedacted,
                payload_digest: format!("sha256:{seq:064x}"),
                retained_payload: Some(serde_json::json!({
                    "adapter_event": kind.as_str(),
                    "task": task_id,
                    "redacted": true
                })),
                retained_at: format!("2026-05-13T16:01:{seq:02}Z"),
                reconstruction_fields: vec![
                    "adapter_event".to_owned(),
                    "task".to_owned(),
                    "redacted".to_owned(),
                ],
            },
        }
    }

    #[test]
    fn state_model_covers_all_acceptance_states() {
        let mut stream = TaskEventStream::new(
            "stream:task-event-alpha",
            "workspace:payments",
            "trace:task-event-alpha",
        );
        for (idx, state) in TaskStateClass::ALL.into_iter().enumerate() {
            let seq = idx as u64 + 1;
            let kind = match state {
                TaskStateClass::Queued => TaskEventKind::TaskQueued,
                TaskStateClass::Running => TaskEventKind::TaskStarted,
                TaskStateClass::Blocked => TaskEventKind::TaskBlocked,
                TaskStateClass::Completed => TaskEventKind::TaskCompleted,
                TaskStateClass::Failed => TaskEventKind::TaskFailed,
                TaskStateClass::Cancelled => TaskEventKind::TaskCancelled,
                TaskStateClass::WaitingForInput => TaskEventKind::InputRequested,
            };
            let payload = match state {
                TaskStateClass::Blocked => TaskEventPayload::Blocked {
                    reason: TaskBlockReason::DependencyMissing,
                    unblock_ref: Some("doctor:task:dependency".to_owned()),
                },
                TaskStateClass::WaitingForInput => TaskEventPayload::InputRequest {
                    request: TaskInputRequest {
                        request_id: "input:deploy:confirm".to_owned(),
                        input_class: TaskInputClass::Confirmation,
                        source_label: "deploy adapter".to_owned(),
                        scope_ref: "target:staging".to_owned(),
                        expires_at: Some("2026-05-13T16:05:00Z".to_owned()),
                    },
                },
                TaskStateClass::Failed => TaskEventPayload::Lifecycle {
                    lifecycle_reason: Some("adapter exited unsuccessfully".to_owned()),
                    exit_status: Some(TaskExitStatus {
                        exit_code: Some(2),
                        failure_class: Some(TaskFailureClass::ExitCode),
                    }),
                },
                TaskStateClass::Cancelled => TaskEventPayload::Lifecycle {
                    lifecycle_reason: Some("user cancelled".to_owned()),
                    exit_status: Some(TaskExitStatus {
                        exit_code: None,
                        failure_class: Some(TaskFailureClass::CancelledByUser),
                    }),
                },
                _ => TaskEventPayload::Lifecycle {
                    lifecycle_reason: Some(state.as_str().to_owned()),
                    exit_status: None,
                },
            };
            stream
                .append(event(
                    seq,
                    &format!("task-{seq}"),
                    TaskWedgeClass::Generic,
                    kind,
                    state,
                    payload,
                    TaskEventSourceKind::Native,
                    TaskEventRedactionClass::MetadataSafeDefault,
                ))
                .expect("state event appends");
        }

        for state in TaskStateClass::ALL {
            assert!(stream
                .task_states
                .values()
                .any(|task| task.current_state == state));
        }
    }

    #[test]
    fn typed_stream_feeds_shell_activity_and_support_export() {
        let mut stream = TaskEventStream::new(
            "stream:task-event-alpha",
            "workspace:payments",
            "trace:task-event-alpha",
        );
        stream
            .append(event(
                1,
                "test-web",
                TaskWedgeClass::Test,
                TaskEventKind::TaskQueued,
                TaskStateClass::Queued,
                TaskEventPayload::Lifecycle {
                    lifecycle_reason: Some("queued by user command".to_owned()),
                    exit_status: None,
                },
                TaskEventSourceKind::Native,
                TaskEventRedactionClass::MetadataSafeDefault,
            ))
            .expect("queued appends");
        stream
            .append(event(
                2,
                "test-web",
                TaskWedgeClass::Test,
                TaskEventKind::ProgressUpdated,
                TaskStateClass::Running,
                TaskEventPayload::Progress {
                    progress: TaskProgress {
                        completed_units: 2,
                        total_units: 4,
                        unit_label: "tests".to_owned(),
                        label: "Running test cases".to_owned(),
                    },
                },
                TaskEventSourceKind::Native,
                TaskEventRedactionClass::OperatorOnlyRestricted,
            ))
            .expect("progress appends");

        let shell = stream.shell_projection();
        assert_eq!(shell.len(), 2);
        assert_eq!(shell[1].state_after, TaskStateClass::Running);
        assert_eq!(shell[1].payload_kind_token, "progress");
        assert_eq!(
            shell[1].inspect_execution_context_action_ref,
            "action:execution-context:inspect:exec:workspace:task:0"
        );

        let activity = stream.activity_projection();
        assert_eq!(activity.len(), 1);
        assert_eq!(activity[0].task_id, "test-web");
        assert_eq!(activity[0].state, TaskStateClass::Running);
        assert_eq!(
            activity[0].progress.as_ref().map(TaskProgress::percent),
            Some(50)
        );
        assert!(activity[0].raw_private_material_excluded);

        let support =
            stream.support_export("support-export:task-events:alpha", "2026-05-13T16:02:00Z");
        assert_eq!(support.events.len(), 2);
        assert_eq!(support.raw_envelopes.len(), 2);
        assert_eq!(support.context_provenance.len(), 1);
        assert_eq!(support.context_provenance_events.len(), 1);
        assert_eq!(
            support.events[1].context_provenance_ref.as_deref(),
            Some("ctx-prov:exec:workspace:task:0")
        );
        assert!(support
            .consumer_surfaces
            .contains(&TaskConsumerSurfaceClass::Shell));
        assert!(support
            .consumer_surfaces
            .contains(&TaskConsumerSurfaceClass::ActivityCenter));
        assert!(support
            .consumer_surfaces
            .contains(&TaskConsumerSurfaceClass::SupportBundleExport));
    }

    #[test]
    fn raw_envelope_identity_is_validated() {
        let mut stream = TaskEventStream::new(
            "stream:task-event-alpha",
            "workspace:payments",
            "trace:task-event-alpha",
        );
        let mut bad = event(
            1,
            "build-api",
            TaskWedgeClass::Build,
            TaskEventKind::TaskQueued,
            TaskStateClass::Queued,
            TaskEventPayload::Lifecycle {
                lifecycle_reason: Some("queued".to_owned()),
                exit_status: None,
            },
            TaskEventSourceKind::Bsp,
            TaskEventRedactionClass::MetadataSafeDefault,
        );
        bad.raw_envelope.trace_id = "trace:other".to_owned();
        let err = stream.append(bad).expect_err("identity mismatch must fail");
        assert!(matches!(
            err,
            TaskEventStreamError::RawEnvelopeIdentityMismatch { .. }
        ));
    }

    #[test]
    fn context_provenance_identity_is_validated() {
        let mut stream = TaskEventStream::new(
            "stream:task-event-alpha",
            "workspace:payments",
            "trace:task-event-alpha",
        );
        let mut bad = event(
            1,
            "build-api",
            TaskWedgeClass::Build,
            TaskEventKind::TaskQueued,
            TaskStateClass::Queued,
            TaskEventPayload::Lifecycle {
                lifecycle_reason: Some("queued".to_owned()),
                exit_status: None,
            },
            TaskEventSourceKind::Bsp,
            TaskEventRedactionClass::MetadataSafeDefault,
        );
        bad.provenance
            .context_provenance
            .as_mut()
            .expect("context provenance")
            .target_id = "target:other".to_owned();
        let err = stream.append(bad).expect_err("context mismatch must fail");
        assert!(matches!(
            err,
            TaskEventStreamError::ContextProvenanceIdentityMismatch { .. }
        ));
    }

    #[test]
    fn degraded_payload_threads_through_state_and_projections() {
        let mut stream = TaskEventStream::new(
            "stream:task-event-alpha",
            "workspace:payments",
            "trace:task-event-alpha",
        );
        stream
            .append(event(
                1,
                "review-audit",
                TaskWedgeClass::Review,
                TaskEventKind::TaskStarted,
                TaskStateClass::Running,
                TaskEventPayload::Lifecycle {
                    lifecycle_reason: Some("review adapter started".to_owned()),
                    exit_status: None,
                },
                TaskEventSourceKind::Native,
                TaskEventRedactionClass::MetadataSafeDefault,
            ))
            .expect("started appends");
        stream
            .append(event(
                2,
                "review-audit",
                TaskWedgeClass::Review,
                TaskEventKind::DegradedStateReported,
                TaskStateClass::Running,
                TaskEventPayload::Degraded {
                    reason: TaskDegradationReason::FallbackParserActive,
                    scope_label: "structured output".to_owned(),
                    recovery_hint_ref: Some("doctor:review:fallback".to_owned()),
                },
                TaskEventSourceKind::HeuristicParser,
                TaskEventRedactionClass::MetadataSafeDefault,
            ))
            .expect("degraded appends");

        let state = stream
            .state_for_task("review-audit")
            .expect("review task tracked");
        assert!(state.is_degraded());
        assert_eq!(
            state.degradation_reason,
            Some(TaskDegradationReason::FallbackParserActive)
        );
        assert_eq!(
            state.degradation_scope_label.as_deref(),
            Some("structured output")
        );

        let activity = stream.activity_projection();
        let row = activity
            .iter()
            .find(|row| row.task_id == "review-audit")
            .expect("review row");
        assert!(row.is_degraded);
        assert!(row.needs_attention);
        assert_eq!(
            row.degradation_reason,
            Some(TaskDegradationReason::FallbackParserActive)
        );

        let shell = stream.shell_projection();
        let shell_row = shell
            .iter()
            .find(|row| row.event_kind == TaskEventKind::DegradedStateReported)
            .expect("shell carries degraded event");
        assert!(shell_row.needs_attention);
        assert_eq!(
            shell_row.degradation_reason,
            Some(TaskDegradationReason::FallbackParserActive)
        );

        let support = stream.support_export("support-export:degraded", "2026-05-13T16:30:00Z");
        let support_row = support
            .events
            .iter()
            .find(|row| row.event_kind == TaskEventKind::DegradedStateReported)
            .expect("support retains degraded row");
        assert_eq!(
            support_row.degradation_reason,
            Some(TaskDegradationReason::FallbackParserActive)
        );

        stream
            .append(event(
                3,
                "review-audit",
                TaskWedgeClass::Review,
                TaskEventKind::TaskCompleted,
                TaskStateClass::Completed,
                TaskEventPayload::Lifecycle {
                    lifecycle_reason: Some("review completed".to_owned()),
                    exit_status: Some(TaskExitStatus {
                        exit_code: Some(0),
                        failure_class: None,
                    }),
                },
                TaskEventSourceKind::Native,
                TaskEventRedactionClass::MetadataSafeDefault,
            ))
            .expect("completed appends");

        let state = stream
            .state_for_task("review-audit")
            .expect("review task tracked after completion");
        assert!(!state.is_degraded(), "terminal state clears degradation");
        assert!(state.degradation_reason.is_none());
    }

    #[test]
    fn fixtures_replay_with_one_event_grammar_across_wedges() {
        let fixture_root =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/runtime/task_event_alpha");
        let fixtures = ["state_coverage_stream.json", "multi_wedge_stream.json"];
        let mut wedges = Vec::new();

        for fixture in fixtures {
            let path = fixture_root.join(fixture);
            let payload = std::fs::read_to_string(&path)
                .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
            let fixture_stream: TaskEventStream = serde_json::from_str(&payload)
                .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()));
            assert_eq!(fixture_stream.record_kind, TASK_EVENT_STREAM_RECORD_KIND);
            assert_eq!(
                fixture_stream.task_event_schema_version,
                TASK_EVENT_SCHEMA_VERSION
            );

            let stream = TaskEventStream::from_events(
                fixture_stream.stream_id.clone(),
                fixture_stream.workspace_id.clone(),
                fixture_stream.trace_id.clone(),
                fixture_stream.events.clone(),
            )
            .unwrap_or_else(|err| panic!("replay fixture {}: {err}", path.display()));
            assert_eq!(stream.events.len(), fixture_stream.events.len());
            let support =
                stream.support_export(format!("support-export:{fixture}"), "2026-05-13T16:30:00Z");
            assert_eq!(support.raw_envelopes.len(), stream.events.len());
            for event in &stream.events {
                wedges.push(event.identity.wedge);
                assert_eq!(event.record_kind, TASK_EVENT_RECORD_KIND);
                assert!(event.raw_envelope.matches_event(event));
                assert!(!event.raw_envelope.reconstruction_fields.is_empty());
            }
        }

        wedges.sort();
        wedges.dedup();
        assert!(wedges.contains(&TaskWedgeClass::Build));
        assert!(wedges.contains(&TaskWedgeClass::Test));
        assert!(wedges.contains(&TaskWedgeClass::Debug));
    }
}
