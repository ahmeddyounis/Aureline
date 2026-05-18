//! Durable run lineage, rerun review, and interruption recovery records.
//!
//! Execution surfaces need a stable language for a run from launch through
//! retry, artifact inspection, export, and post-failure diagnosis. This module
//! projects canonical [`crate::execution_context::ExecutionContext`] and
//! [`crate::rerun::RerunTargetComparison`] records into beta-facing run
//! summaries, durable activity rows, artifact detail sheets, rerun review
//! sheets, and support-export packets.
//!
//! The boundary schemas live at
//! [`/schemas/runtime/run_summary.schema.json`](../../../../schemas/runtime/run_summary.schema.json)
//! and
//! [`/schemas/runtime/rerun_review.schema.json`](../../../../schemas/runtime/rerun_review.schema.json).

use serde::{Deserialize, Serialize};

use crate::execution_context::{ExecutionContext, SurfaceClass, TargetClass, ToolchainClass};
use crate::rerun::{RerunDiffRow, RerunTargetComparison};
use crate::tasks::TaskWedgeClass;
use crate::{ExecutionProvenanceRedactionClass, TrustState};

/// Schema version stamped on every run-lineage record.
pub const RUN_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`RunSummaryCard`].
pub const RUN_SUMMARY_CARD_RECORD_KIND: &str = "run_summary_card_record";
/// Stable record-kind tag for [`DurableJobRow`].
pub const DURABLE_JOB_ROW_RECORD_KIND: &str = "durable_job_row_record";
/// Stable record-kind tag for [`RunArtifactDetailSheet`].
pub const RUN_ARTIFACT_DETAIL_SHEET_RECORD_KIND: &str = "run_artifact_detail_sheet_record";
/// Stable record-kind tag for [`RerunReviewSheet`].
pub const RERUN_REVIEW_SHEET_RECORD_KIND: &str = "rerun_review_sheet_record";
/// Stable record-kind tag for [`RunHistorySupportExport`].
pub const RUN_HISTORY_SUPPORT_EXPORT_RECORD_KIND: &str = "run_history_support_export_record";

/// Lifecycle vocabulary shared by run headers, durable job rows, CLI/headless
/// output, and support exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunLifecycleState {
    /// Work was accepted but is not yet preparing.
    Queued,
    /// Resolver, environment, dependency, or admission setup is underway.
    Preparing,
    /// Live execution is running with attributable output or progress.
    Running,
    /// The run is blocked on a bounded input request.
    WaitingInput,
    /// Usable output exists but one or more declared channels did not finish.
    PartiallyComplete,
    /// Declared success condition completed.
    Passed,
    /// Declared failure condition completed.
    Failed,
    /// A user, policy, or subsystem cancelled the attempt.
    Cancelled,
    /// Prior output remains inspectable but is not current or fully parsed.
    StaleOutputPartialParse,
}

impl RunLifecycleState {
    /// All lifecycle states in stable vocabulary order.
    pub const ALL: [Self; 9] = [
        Self::Queued,
        Self::Preparing,
        Self::Running,
        Self::WaitingInput,
        Self::PartiallyComplete,
        Self::Passed,
        Self::Failed,
        Self::Cancelled,
        Self::StaleOutputPartialParse,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Preparing => "preparing",
            Self::Running => "running",
            Self::WaitingInput => "waiting_input",
            Self::PartiallyComplete => "partially_complete",
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::StaleOutputPartialParse => "stale_output_partial_parse",
        }
    }

    /// Short label safe for UI, CLI/headless output, and support exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Queued => "Queued",
            Self::Preparing => "Preparing",
            Self::Running => "Running",
            Self::WaitingInput => "Waiting input",
            Self::PartiallyComplete => "Partially complete",
            Self::Passed => "Passed",
            Self::Failed => "Failed",
            Self::Cancelled => "Cancelled",
            Self::StaleOutputPartialParse => "Stale output / partial parse",
        }
    }

    /// True when the state is terminal for the current attempt.
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::PartiallyComplete
                | Self::Passed
                | Self::Failed
                | Self::Cancelled
                | Self::StaleOutputPartialParse
        )
    }
}

/// Freshness marker used by headers, durable rows, artifacts, and exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunFreshnessClass {
    /// Row reflects the current workspace and current context.
    Current,
    /// Row is prior evidence retained after code, config, toolchain, policy,
    /// target, or another replay-relevant input changed.
    Stale,
    /// Row was imported from a provider, support bundle, or offline packet.
    Imported,
}

impl RunFreshnessClass {
    /// All freshness classes in stable vocabulary order.
    pub const ALL: [Self; 3] = [Self::Current, Self::Stale, Self::Imported];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Imported => "imported",
        }
    }
}

/// Execution boundary projected onto run summaries and job rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunBoundaryClass {
    /// Work runs on the local host.
    Local,
    /// Work runs on a remote host or remote agent.
    Remote,
    /// Work runs inside a local container or devcontainer boundary.
    Container,
    /// Work runs in a managed or hosted workspace boundary.
    Managed,
    /// Work runs inside an AI sandbox boundary.
    AiSandbox,
}

impl RunBoundaryClass {
    /// All boundary classes in stable vocabulary order.
    pub const ALL: [Self; 5] = [
        Self::Local,
        Self::Remote,
        Self::Container,
        Self::Managed,
        Self::AiSandbox,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Remote => "remote",
            Self::Container => "container",
            Self::Managed => "managed",
            Self::AiSandbox => "ai_sandbox",
        }
    }

    /// Projects a boundary class from the canonical target class.
    pub const fn from_target_class(target_class: TargetClass) -> Self {
        match target_class {
            TargetClass::LocalHost | TargetClass::NotebookKernelLocal => Self::Local,
            TargetClass::SshRemote
            | TargetClass::RemoteWorkspaceVm
            | TargetClass::NotebookKernelRemote => Self::Remote,
            TargetClass::ContainerLocal | TargetClass::Devcontainer => Self::Container,
            TargetClass::PrebuildRuntime | TargetClass::ManagedWorkspace => Self::Managed,
            TargetClass::AiSandbox => Self::AiSandbox,
        }
    }
}

/// Relationship between retained prior evidence and the current workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunCurrentRelationshipClass {
    /// The row describes the current attempt.
    CurrentAttempt,
    /// The row is old evidence retained after current context drift.
    StalePriorEvidence,
    /// The row was produced by a current-context rerun of an older run.
    CurrentContextRerun,
    /// The row came from imported provider or support-bundle evidence.
    ImportedEvidence,
    /// The row cannot be replayed automatically and requires manual action.
    ManualReplayRequired,
}

impl RunCurrentRelationshipClass {
    /// All relationship classes in stable vocabulary order.
    pub const ALL: [Self; 5] = [
        Self::CurrentAttempt,
        Self::StalePriorEvidence,
        Self::CurrentContextRerun,
        Self::ImportedEvidence,
        Self::ManualReplayRequired,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentAttempt => "current_attempt",
            Self::StalePriorEvidence => "stale_prior_evidence",
            Self::CurrentContextRerun => "current_context_rerun",
            Self::ImportedEvidence => "imported_evidence",
            Self::ManualReplayRequired => "manual_replay_required",
        }
    }
}

/// Continuity path that caused a durable row to be reopened or refreshed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunContinuityMarker {
    /// The row survived a look-away or activity-center collapse.
    LookAwayReturn,
    /// The row survived host sleep and resume.
    SleepResume,
    /// The row survived switching windows or workspaces.
    WindowSwitch,
    /// The row survived a runtime service restart.
    RuntimeRestart,
    /// The row is restored from imported or support-bundle evidence.
    ImportedReplay,
}

impl RunContinuityMarker {
    /// All continuity markers in stable vocabulary order.
    pub const ALL: [Self; 5] = [
        Self::LookAwayReturn,
        Self::SleepResume,
        Self::WindowSwitch,
        Self::RuntimeRestart,
        Self::ImportedReplay,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LookAwayReturn => "look_away_return",
            Self::SleepResume => "sleep_resume",
            Self::WindowSwitch => "window_switch",
            Self::RuntimeRestart => "runtime_restart",
            Self::ImportedReplay => "imported_replay",
        }
    }
}

/// Controlled interruption taxonomy rendered by UI, activity rows,
/// CLI/headless output, and support exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunInterruptionKind {
    /// The user cancelled the run.
    UserCancel,
    /// The remote route or helper disconnected.
    RemoteDisconnect,
    /// The host or target paused because of thermal pressure.
    ThermalPause,
    /// Policy blocked or revoked the run.
    PolicyBlock,
    /// Authentication expired before the run could complete or resume.
    AuthExpiry,
    /// The process or adapter crashed.
    ProcessCrash,
    /// Source-map lineage was lost or could not be resolved.
    LostSourceMap,
    /// Log output was truncated by policy, transport, or retention.
    TruncatedLog,
    /// The row is imported or stale relative to current workspace truth.
    StaleImport,
    /// Automatic replay is unavailable and manual replay is required.
    ManualReplayRequirement,
}

impl RunInterruptionKind {
    /// All interruption kinds in stable vocabulary order.
    pub const ALL: [Self; 10] = [
        Self::UserCancel,
        Self::RemoteDisconnect,
        Self::ThermalPause,
        Self::PolicyBlock,
        Self::AuthExpiry,
        Self::ProcessCrash,
        Self::LostSourceMap,
        Self::TruncatedLog,
        Self::StaleImport,
        Self::ManualReplayRequirement,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserCancel => "user_cancel",
            Self::RemoteDisconnect => "remote_disconnect",
            Self::ThermalPause => "thermal_pause",
            Self::PolicyBlock => "policy_block",
            Self::AuthExpiry => "auth_expiry",
            Self::ProcessCrash => "process_crash",
            Self::LostSourceMap => "lost_source_map",
            Self::TruncatedLog => "truncated_log",
            Self::StaleImport => "stale_import",
            Self::ManualReplayRequirement => "manual_replay_requirement",
        }
    }

    /// Short label safe for user-facing surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::UserCancel => "User cancel",
            Self::RemoteDisconnect => "Remote disconnect",
            Self::ThermalPause => "Thermal pause",
            Self::PolicyBlock => "Policy block",
            Self::AuthExpiry => "Auth expiry",
            Self::ProcessCrash => "Process crash",
            Self::LostSourceMap => "Lost source map",
            Self::TruncatedLog => "Truncated log",
            Self::StaleImport => "Stale import",
            Self::ManualReplayRequirement => "Manual replay required",
        }
    }
}

/// Artifact kinds surfaced by run artifact detail sheets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunArtifactKind {
    /// Build or bundle output.
    BuildOutput,
    /// Coverage report or coverage data.
    Coverage,
    /// Structured test report.
    TestReport,
    /// Bounded log slice.
    LogSlice,
    /// Trace or profiling artifact.
    Profile,
    /// Debug artifact such as symbols or dump metadata.
    DebugArtifact,
    /// Validation, lint, or quality summary.
    ValidationSummary,
    /// Diagnostic bundle.
    DiagnosticBundle,
    /// Source-map or generated-source mapping artifact.
    SourceMap,
}

impl RunArtifactKind {
    /// All artifact kinds in stable vocabulary order.
    pub const ALL: [Self; 9] = [
        Self::BuildOutput,
        Self::Coverage,
        Self::TestReport,
        Self::LogSlice,
        Self::Profile,
        Self::DebugArtifact,
        Self::ValidationSummary,
        Self::DiagnosticBundle,
        Self::SourceMap,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildOutput => "build_output",
            Self::Coverage => "coverage",
            Self::TestReport => "test_report",
            Self::LogSlice => "log_slice",
            Self::Profile => "profile",
            Self::DebugArtifact => "debug_artifact",
            Self::ValidationSummary => "validation_summary",
            Self::DiagnosticBundle => "diagnostic_bundle",
            Self::SourceMap => "source_map",
        }
    }
}

/// Retention posture for a run artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunArtifactRetentionClass {
    /// Metadata and digest only.
    MetadataOnly,
    /// Redacted payload or preview is retained.
    RetainedRedacted,
    /// Explicitly pinned evidence is retained under policy.
    PinnedEvidence,
    /// Large or sensitive material remains provider-only.
    ProviderOnly,
}

impl RunArtifactRetentionClass {
    /// All retention classes in stable vocabulary order.
    pub const ALL: [Self; 4] = [
        Self::MetadataOnly,
        Self::RetainedRedacted,
        Self::PinnedEvidence,
        Self::ProviderOnly,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata_only",
            Self::RetainedRedacted => "retained_redacted",
            Self::PinnedEvidence => "pinned_evidence",
            Self::ProviderOnly => "provider_only",
        }
    }
}

/// Viewer class exposed for artifact details.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunArtifactViewerClass {
    /// Structured first-party viewer.
    StructuredViewer,
    /// Log viewer or raw text preview.
    LogViewer,
    /// Safe rich-preview viewer.
    SafePreview,
    /// Shared trace or profile viewer.
    TraceViewer,
    /// Checksum and download-only details.
    DownloadOnly,
    /// Raw fallback viewer.
    RawFallback,
}

impl RunArtifactViewerClass {
    /// All viewer classes in stable vocabulary order.
    pub const ALL: [Self; 6] = [
        Self::StructuredViewer,
        Self::LogViewer,
        Self::SafePreview,
        Self::TraceViewer,
        Self::DownloadOnly,
        Self::RawFallback,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StructuredViewer => "structured_viewer",
            Self::LogViewer => "log_viewer",
            Self::SafePreview => "safe_preview",
            Self::TraceViewer => "trace_viewer",
            Self::DownloadOnly => "download_only",
            Self::RawFallback => "raw_fallback",
        }
    }
}

/// Artifact actions exposed in detail sheets and support exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunArtifactActionClass {
    /// Open the compatible viewer.
    OpenViewer,
    /// Open raw fallback.
    OpenRawFallback,
    /// Export with redaction.
    ExportRedacted,
    /// Pin under the declared retention policy.
    PinArtifact,
    /// Review redaction before export.
    ReviewRedaction,
}

impl RunArtifactActionClass {
    /// All artifact action classes in stable vocabulary order.
    pub const ALL: [Self; 5] = [
        Self::OpenViewer,
        Self::OpenRawFallback,
        Self::ExportRedacted,
        Self::PinArtifact,
        Self::ReviewRedaction,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenViewer => "open_viewer",
            Self::OpenRawFallback => "open_raw_fallback",
            Self::ExportRedacted => "export_redacted",
            Self::PinArtifact => "pin_artifact",
            Self::ReviewRedaction => "review_redaction",
        }
    }

    /// Short label safe for UI, CLI/headless output, and support exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenViewer => "Open viewer",
            Self::OpenRawFallback => "Open raw fallback",
            Self::ExportRedacted => "Export redacted",
            Self::PinArtifact => "Pin artifact",
            Self::ReviewRedaction => "Review redaction",
        }
    }
}

/// Rerun mode shown in review sheets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunReviewMode {
    /// Dispatch against the exact captured context.
    RerunExactly,
    /// Dispatch against the freshly resolved current workspace/context.
    RerunWithCurrentContext,
}

impl RerunReviewMode {
    /// All rerun modes in stable vocabulary order.
    pub const ALL: [Self; 2] = [Self::RerunExactly, Self::RerunWithCurrentContext];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RerunExactly => "rerun_exactly",
            Self::RerunWithCurrentContext => "rerun_with_current_context",
        }
    }

    /// Short label safe for UI, CLI/headless output, and support exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RerunExactly => "Rerun exactly",
            Self::RerunWithCurrentContext => "Rerun with current context",
        }
    }
}

/// Changed input dimension shown in a rerun review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunReviewDriftField {
    /// Source code revision or digest changed.
    Code,
    /// Configuration or manifest input changed.
    Config,
    /// Runtime, interpreter, package manager, or build driver changed.
    Toolchain,
    /// Locale or regional formatting input changed.
    Locale,
    /// Policy epoch or policy decision state changed.
    Policy,
    /// Secret-handle class, expiry, or availability changed.
    Secrets,
    /// Git branch changed.
    Branch,
    /// Target identity or execution boundary changed.
    Target,
    /// Trust posture changed.
    Trust,
    /// Environment capsule hash, drift, or prebuild fingerprint changed.
    Environment,
    /// Workset scope or working directory changed.
    WorkspaceScope,
}

impl RerunReviewDriftField {
    /// All rerun drift fields in stable vocabulary order.
    pub const ALL: [Self; 11] = [
        Self::Code,
        Self::Config,
        Self::Toolchain,
        Self::Locale,
        Self::Policy,
        Self::Secrets,
        Self::Branch,
        Self::Target,
        Self::Trust,
        Self::Environment,
        Self::WorkspaceScope,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Config => "config",
            Self::Toolchain => "toolchain",
            Self::Locale => "locale",
            Self::Policy => "policy",
            Self::Secrets => "secrets",
            Self::Branch => "branch",
            Self::Target => "target",
            Self::Trust => "trust",
            Self::Environment => "environment",
            Self::WorkspaceScope => "workspace_scope",
        }
    }

    fn from_target_diff(row: &RerunDiffRow) -> Self {
        match row.field_path.as_str() {
            "target_identity.target_class"
            | "target_identity.canonical_target_id"
            | "target_identity.reachability_state"
            | "target_identity.local_vs_managed_boundary_visible"
            | "target_confidence.level"
            | "target_confidence.reasons"
            | "mixed_version_drift.state"
            | "mixed_version_drift.reason" => Self::Target,
            "target_identity.working_directory" | "workset_scope_class" => Self::WorkspaceScope,
            "toolchain_identity.toolchain_class"
            | "toolchain_identity.toolchain_id"
            | "toolchain_identity.resolved_version" => Self::Toolchain,
            "policy_and_trust.policy_epoch" => Self::Policy,
            "policy_and_trust.trust_state" => Self::Trust,
            "cache_disposition"
            | "prebuild_metadata.reuse_state"
            | "prebuild_metadata.compatibility_fingerprint"
            | "prebuild_metadata.invalidation_reason" => Self::Environment,
            "degraded_fields" => Self::Environment,
            _ => Self::Environment,
        }
    }
}

/// Build or commit identity shown in run headers and export packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunBuildIdentity {
    /// Git branch or source branch label.
    pub branch: String,
    /// Commit, build, or source revision token.
    pub commit_ref: String,
    /// Build identity, bundle id, or provider build id.
    pub build_id: String,
}

impl RunBuildIdentity {
    /// Builds a stable build identity projection.
    pub fn new(
        branch: impl Into<String>,
        commit_ref: impl Into<String>,
        build_id: impl Into<String>,
    ) -> Self {
        Self {
            branch: branch.into(),
            commit_ref: commit_ref.into(),
            build_id: build_id.into(),
        }
    }
}

/// One action exposed by run history, durable rows, or artifact sheets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunActionRef {
    /// Action class token.
    pub action_token: String,
    /// Stable action reference.
    pub action_ref: String,
    /// Short label safe for UI and CLI/headless output.
    pub label: String,
}

impl RunActionRef {
    /// Builds a run action reference.
    pub fn new(
        action_token: impl Into<String>,
        action_ref: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        Self {
            action_token: action_token.into(),
            action_ref: action_ref.into(),
            label: label.into(),
        }
    }
}

/// Compact execution-context projection used by run headers and support rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunContextSummary {
    /// Execution-context reference.
    pub execution_context_ref: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Surface that initiated the run.
    pub surface: SurfaceClass,
    /// Stable surface token.
    pub surface_token: String,
    /// Command id that initiated the run.
    pub command_id: String,
    /// Target class.
    pub target_class: TargetClass,
    /// Stable target-class token.
    pub target_class_token: String,
    /// Canonical target id.
    pub canonical_target_id: String,
    /// Working directory when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    /// Boundary class.
    pub boundary_class: RunBoundaryClass,
    /// Stable boundary token.
    pub boundary_token: String,
    /// True when local-vs-remote or managed boundary chrome must be visible.
    pub boundary_cue_visible: bool,
    /// Toolchain class.
    pub toolchain_class: ToolchainClass,
    /// Stable toolchain token.
    pub toolchain_class_token: String,
    /// Toolchain id.
    pub toolchain_id: String,
    /// Resolved toolchain version.
    pub resolved_toolchain_version: String,
    /// Environment capsule reference.
    pub environment_capsule_ref: String,
    /// Environment capsule hash.
    pub environment_capsule_hash: String,
    /// Environment capsule drift token.
    pub environment_capsule_drift_token: String,
    /// Trust state token.
    pub trust_state_token: String,
    /// Policy epoch.
    pub policy_epoch: u64,
}

impl RunContextSummary {
    /// Projects a compact run context summary from the canonical execution
    /// context.
    pub fn from_context(context: &ExecutionContext) -> Self {
        let target_class = context.target_identity.target_class;
        let boundary_class = RunBoundaryClass::from_target_class(target_class);
        Self {
            execution_context_ref: context.execution_context_id.clone(),
            workspace_id: context.invocation_subject.workspace_id.clone(),
            surface: context.invocation_subject.surface,
            surface_token: context.invocation_subject.surface.as_str().to_owned(),
            command_id: context.invocation_subject.command_id.clone(),
            target_class,
            target_class_token: target_class.as_str().to_owned(),
            canonical_target_id: context.target_identity.canonical_target_id.clone(),
            working_directory: context.target_identity.working_directory.clone(),
            boundary_class,
            boundary_token: boundary_class.as_str().to_owned(),
            boundary_cue_visible: context.target_identity.local_vs_managed_boundary_visible,
            toolchain_class: context.toolchain_identity.toolchain_class,
            toolchain_class_token: context
                .toolchain_identity
                .toolchain_class
                .as_str()
                .to_owned(),
            toolchain_id: context.toolchain_identity.toolchain_id.clone(),
            resolved_toolchain_version: context.toolchain_identity.resolved_version.clone(),
            environment_capsule_ref: context.environment_capsule_ref.capsule_id.clone(),
            environment_capsule_hash: context.environment_capsule_ref.capsule_hash.clone(),
            environment_capsule_drift_token: context
                .environment_capsule_ref
                .drift_state
                .as_str()
                .to_owned(),
            trust_state_token: trust_state_token(context.policy_and_trust.trust_state).to_owned(),
            policy_epoch: context.policy_and_trust.policy_epoch,
        }
    }
}

/// Run header or summary card used by execution details, activity center,
/// CLI/headless output, and support exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunSummaryCard {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable run id.
    pub run_id: String,
    /// Stable latest attempt id.
    pub latest_attempt_id: String,
    /// Human-readable run title.
    pub title: String,
    /// Initiating actor label.
    pub initiator: String,
    /// Wedge that produced the run.
    pub wedge: TaskWedgeClass,
    /// Stable wedge token.
    pub wedge_token: String,
    /// Exact execution-context summary for the latest attempt.
    pub context: RunContextSummary,
    /// Build or commit identity for the run.
    pub build_identity: RunBuildIdentity,
    /// Start timestamp.
    pub started_at: String,
    /// Finish timestamp when the current attempt is terminal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    /// Current lifecycle state.
    pub lifecycle_state: RunLifecycleState,
    /// Stable lifecycle token.
    pub lifecycle_state_token: String,
    /// Freshness marker.
    pub freshness: RunFreshnessClass,
    /// Stable freshness token.
    pub freshness_token: String,
    /// Current relationship to retained prior evidence.
    pub current_relationship: RunCurrentRelationshipClass,
    /// Stable current-relationship token.
    pub current_relationship_token: String,
    /// Interruption kind when the attempt is interrupted or partially
    /// recoverable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interruption_kind: Option<RunInterruptionKind>,
    /// Stable interruption token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interruption_token: Option<String>,
    /// Short interruption label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interruption_label: Option<String>,
    /// Count of artifact detail sheets linked to this run.
    pub artifact_count: u32,
    /// True when old evidence is retained rather than overwritten by a rerun.
    pub old_evidence_preserved: bool,
    /// Stable evidence refs linked to this run.
    pub evidence_refs: Vec<String>,
}

impl RunSummaryCard {
    /// Builds a run summary card from a canonical execution context.
    #[allow(clippy::too_many_arguments)]
    pub fn from_context(
        run_id: impl Into<String>,
        latest_attempt_id: impl Into<String>,
        title: impl Into<String>,
        initiator: impl Into<String>,
        wedge: TaskWedgeClass,
        context: &ExecutionContext,
        build_identity: RunBuildIdentity,
        started_at: impl Into<String>,
        finished_at: Option<String>,
        lifecycle_state: RunLifecycleState,
        freshness: RunFreshnessClass,
        current_relationship: RunCurrentRelationshipClass,
        interruption_kind: Option<RunInterruptionKind>,
        artifact_count: u32,
        evidence_refs: Vec<String>,
    ) -> Self {
        let interruption_token = interruption_kind.map(|kind| kind.as_str().to_owned());
        let interruption_label = interruption_kind.map(|kind| kind.label().to_owned());
        Self {
            record_kind: RUN_SUMMARY_CARD_RECORD_KIND.to_owned(),
            schema_version: RUN_LINEAGE_SCHEMA_VERSION,
            run_id: run_id.into(),
            latest_attempt_id: latest_attempt_id.into(),
            title: title.into(),
            initiator: initiator.into(),
            wedge,
            wedge_token: wedge.as_str().to_owned(),
            context: RunContextSummary::from_context(context),
            build_identity,
            started_at: started_at.into(),
            finished_at,
            lifecycle_state,
            lifecycle_state_token: lifecycle_state.as_str().to_owned(),
            freshness,
            freshness_token: freshness.as_str().to_owned(),
            current_relationship,
            current_relationship_token: current_relationship.as_str().to_owned(),
            interruption_kind,
            interruption_token,
            interruption_label,
            artifact_count,
            old_evidence_preserved: matches!(
                current_relationship,
                RunCurrentRelationshipClass::StalePriorEvidence
                    | RunCurrentRelationshipClass::ImportedEvidence
                    | RunCurrentRelationshipClass::ManualReplayRequired
            ),
            evidence_refs,
        }
    }

    /// True when the summary names an interruption directly.
    pub const fn names_interruption(&self) -> bool {
        self.interruption_kind.is_some()
    }
}

/// Durable activity-center row for a run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableJobRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable durable row id.
    pub job_row_id: String,
    /// Run id.
    pub run_id: String,
    /// Latest attempt id.
    pub latest_attempt_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Current lifecycle state.
    pub lifecycle_state: RunLifecycleState,
    /// Stable lifecycle token.
    pub lifecycle_state_token: String,
    /// Last update timestamp.
    pub last_update_at: String,
    /// Artifact count linked to the run.
    pub artifact_count: u32,
    /// Freshness marker.
    pub freshness: RunFreshnessClass,
    /// Stable freshness token.
    pub freshness_token: String,
    /// Relationship to the current workspace/context.
    pub current_relationship: RunCurrentRelationshipClass,
    /// Stable current-relationship token.
    pub current_relationship_token: String,
    /// Interruption kind when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interruption_kind: Option<RunInterruptionKind>,
    /// Stable interruption token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interruption_token: Option<String>,
    /// Continuity markers proving the row survived restore paths.
    pub continuity_markers: Vec<RunContinuityMarker>,
    /// Stable continuity marker tokens.
    pub continuity_marker_tokens: Vec<String>,
    /// Retry action reference.
    pub retry_action: RunActionRef,
    /// Exact rerun action reference.
    pub rerun_exact_action: RunActionRef,
    /// Current-context rerun action reference.
    pub rerun_current_context_action: RunActionRef,
    /// Detail-sheet action reference.
    pub open_details_action: RunActionRef,
    /// True when the row is inspectable after live surface dismissal.
    pub durable_after_surface_close: bool,
    /// True when raw private material is excluded by default.
    pub raw_private_material_excluded: bool,
}

impl DurableJobRow {
    /// Projects a durable job row from a run summary.
    pub fn from_summary(
        summary: &RunSummaryCard,
        last_update_at: impl Into<String>,
        continuity_markers: Vec<RunContinuityMarker>,
    ) -> Self {
        let continuity_marker_tokens = continuity_markers
            .iter()
            .map(|marker| marker.as_str().to_owned())
            .collect();
        Self {
            record_kind: DURABLE_JOB_ROW_RECORD_KIND.to_owned(),
            schema_version: RUN_LINEAGE_SCHEMA_VERSION,
            job_row_id: format!("durable-job:{}", summary.run_id),
            run_id: summary.run_id.clone(),
            latest_attempt_id: summary.latest_attempt_id.clone(),
            workspace_id: summary.context.workspace_id.clone(),
            lifecycle_state: summary.lifecycle_state,
            lifecycle_state_token: summary.lifecycle_state_token.clone(),
            last_update_at: last_update_at.into(),
            artifact_count: summary.artifact_count,
            freshness: summary.freshness,
            freshness_token: summary.freshness_token.clone(),
            current_relationship: summary.current_relationship,
            current_relationship_token: summary.current_relationship_token.clone(),
            interruption_kind: summary.interruption_kind,
            interruption_token: summary.interruption_token.clone(),
            continuity_markers,
            continuity_marker_tokens,
            retry_action: RunActionRef::new(
                "retry_failed_step",
                format!("action:run:{}:retry_failed_step", summary.run_id),
                "Retry failed step",
            ),
            rerun_exact_action: RunActionRef::new(
                RerunReviewMode::RerunExactly.as_str(),
                format!("action:run:{}:rerun_exactly", summary.run_id),
                RerunReviewMode::RerunExactly.label(),
            ),
            rerun_current_context_action: RunActionRef::new(
                RerunReviewMode::RerunWithCurrentContext.as_str(),
                format!("action:run:{}:rerun_current_context", summary.run_id),
                RerunReviewMode::RerunWithCurrentContext.label(),
            ),
            open_details_action: RunActionRef::new(
                "open_run_details",
                format!("action:run:{}:open_details", summary.run_id),
                "Open details",
            ),
            durable_after_surface_close: true,
            raw_private_material_excluded: true,
        }
    }

    /// True when the row retains prior evidence instead of presenting it as
    /// current.
    pub const fn preserves_old_evidence(&self) -> bool {
        matches!(
            self.current_relationship,
            RunCurrentRelationshipClass::StalePriorEvidence
                | RunCurrentRelationshipClass::ImportedEvidence
                | RunCurrentRelationshipClass::ManualReplayRequired
        )
    }
}

/// Artifact detail sheet that preserves producing-run lineage and fallback
/// viewing/export posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunArtifactDetailSheet {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable artifact id.
    pub artifact_id: String,
    /// Artifact kind.
    pub artifact_kind: RunArtifactKind,
    /// Stable artifact-kind token.
    pub artifact_kind_token: String,
    /// Producing run id.
    pub producing_run_id: String,
    /// Producing attempt id.
    pub producing_attempt_id: String,
    /// Producing execution-context ref.
    pub producing_execution_context_ref: String,
    /// Freshness marker.
    pub freshness: RunFreshnessClass,
    /// Stable freshness token.
    pub freshness_token: String,
    /// Retention class.
    pub retention_class: RunArtifactRetentionClass,
    /// Stable retention token.
    pub retention_class_token: String,
    /// Redaction class.
    pub redaction_class: ExecutionProvenanceRedactionClass,
    /// Stable redaction token.
    pub redaction_class_token: String,
    /// Preferred viewer.
    pub preferred_viewer: RunArtifactViewerClass,
    /// Stable preferred-viewer token.
    pub preferred_viewer_token: String,
    /// Raw fallback viewer.
    pub raw_fallback_viewer: RunArtifactViewerClass,
    /// Stable raw-fallback token.
    pub raw_fallback_viewer_token: String,
    /// True when a compatible viewer is available.
    pub compatible_viewer_available: bool,
    /// True when raw fallback is available.
    pub raw_fallback_available: bool,
    /// Artifact actions.
    pub actions: Vec<RunActionRef>,
    /// Export-safe summary line.
    pub summary: String,
}

impl RunArtifactDetailSheet {
    /// Builds an artifact detail sheet for a producing run summary.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        artifact_id: impl Into<String>,
        artifact_kind: RunArtifactKind,
        producing_run: &RunSummaryCard,
        freshness: RunFreshnessClass,
        retention_class: RunArtifactRetentionClass,
        redaction_class: ExecutionProvenanceRedactionClass,
        preferred_viewer: RunArtifactViewerClass,
        raw_fallback_available: bool,
    ) -> Self {
        let artifact_id = artifact_id.into();
        let raw_fallback_viewer = RunArtifactViewerClass::RawFallback;
        let actions = artifact_actions_for(
            &artifact_id,
            preferred_viewer,
            raw_fallback_available,
            retention_class,
        );
        let summary = format!(
            "artifact={}; kind={}; run={}; attempt={}; freshness={}; retention={}",
            artifact_id,
            artifact_kind.as_str(),
            producing_run.run_id,
            producing_run.latest_attempt_id,
            freshness.as_str(),
            retention_class.as_str(),
        );
        Self {
            record_kind: RUN_ARTIFACT_DETAIL_SHEET_RECORD_KIND.to_owned(),
            schema_version: RUN_LINEAGE_SCHEMA_VERSION,
            artifact_id,
            artifact_kind,
            artifact_kind_token: artifact_kind.as_str().to_owned(),
            producing_run_id: producing_run.run_id.clone(),
            producing_attempt_id: producing_run.latest_attempt_id.clone(),
            producing_execution_context_ref: producing_run.context.execution_context_ref.clone(),
            freshness,
            freshness_token: freshness.as_str().to_owned(),
            retention_class,
            retention_class_token: retention_class.as_str().to_owned(),
            redaction_class,
            redaction_class_token: redaction_class.as_str().to_owned(),
            preferred_viewer,
            preferred_viewer_token: preferred_viewer.as_str().to_owned(),
            raw_fallback_viewer,
            raw_fallback_viewer_token: raw_fallback_viewer.as_str().to_owned(),
            compatible_viewer_available: !matches!(
                preferred_viewer,
                RunArtifactViewerClass::DownloadOnly
            ),
            raw_fallback_available,
            actions,
            summary,
        }
    }
}

/// One changed dimension in a rerun review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunReviewDriftRow {
    /// Stable field dimension.
    pub field: RerunReviewDriftField,
    /// Stable field token.
    pub field_token: String,
    /// Dotted source path where the drift came from.
    pub source_field_path: String,
    /// Human-readable label.
    pub label: String,
    /// Exact or prior value token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_value_token: Option<String>,
    /// Current value token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_value_token: Option<String>,
    /// True when this difference affects whether exact reproduction is still
    /// possible.
    pub changes_reproduction_context: bool,
    /// True when review is required before dispatch.
    pub requires_review_before_dispatch: bool,
}

impl RerunReviewDriftRow {
    /// Builds a drift row from a target comparison diff.
    pub fn from_target_diff(row: &RerunDiffRow) -> Self {
        let field = RerunReviewDriftField::from_target_diff(row);
        Self {
            field,
            field_token: field.as_str().to_owned(),
            source_field_path: row.field_path.clone(),
            label: row.label.clone(),
            exact_value_token: row.exact_value_token.clone(),
            current_value_token: row.current_value_token.clone(),
            changes_reproduction_context: row.requires_review_before_dispatch,
            requires_review_before_dispatch: row.requires_review_before_dispatch,
        }
    }

    /// Builds a manual non-target drift row.
    pub fn manual(
        field: RerunReviewDriftField,
        source_field_path: impl Into<String>,
        label: impl Into<String>,
        exact_value_token: Option<String>,
        current_value_token: Option<String>,
    ) -> Self {
        Self {
            field,
            field_token: field.as_str().to_owned(),
            source_field_path: source_field_path.into(),
            label: label.into(),
            exact_value_token,
            current_value_token,
            changes_reproduction_context: true,
            requires_review_before_dispatch: true,
        }
    }
}

/// One available rerun mode option in the review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunReviewModeOption {
    /// Rerun mode.
    pub mode: RerunReviewMode,
    /// Stable mode token.
    pub mode_token: String,
    /// Short label.
    pub label: String,
    /// Execution-context ref the option would dispatch against.
    pub execution_context_ref: String,
    /// True when the option preserves original context truth.
    pub preserves_original_context: bool,
    /// True when the option uses the current workspace/context.
    pub uses_current_workspace_context: bool,
    /// Dispatch action ref.
    pub dispatch_action_ref: String,
}

impl RerunReviewModeOption {
    /// Builds a mode option.
    pub fn new(
        mode: RerunReviewMode,
        run_id: &str,
        execution_context_ref: impl Into<String>,
    ) -> Self {
        Self {
            mode,
            mode_token: mode.as_str().to_owned(),
            label: mode.label().to_owned(),
            execution_context_ref: execution_context_ref.into(),
            preserves_original_context: matches!(mode, RerunReviewMode::RerunExactly),
            uses_current_workspace_context: matches!(
                mode,
                RerunReviewMode::RerunWithCurrentContext
            ),
            dispatch_action_ref: format!("action:run:{run_id}:{}", mode.as_str()),
        }
    }
}

/// Review sheet shown before dispatching an exact or current-context rerun.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunReviewSheet {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable review id.
    pub review_id: String,
    /// Source run id.
    pub source_run_id: String,
    /// Source attempt id.
    pub source_attempt_id: String,
    /// Exact-prior execution context ref.
    pub exact_context_ref: String,
    /// Current execution context ref.
    pub current_context_ref: String,
    /// Timestamp when the review was generated.
    pub generated_at: String,
    /// Available rerun modes.
    pub mode_options: Vec<RerunReviewModeOption>,
    /// Changed dimensions.
    pub drift_rows: Vec<RerunReviewDriftRow>,
    /// Stable drift field tokens.
    pub drift_field_tokens: Vec<String>,
    /// True when exact-vs-current drift exists.
    pub has_drift: bool,
    /// True when dispatch must wait for visible review.
    pub requires_review_before_dispatch: bool,
    /// True when prior evidence remains preserved.
    pub old_evidence_preserved: bool,
    /// Relationship between old evidence and current context.
    pub old_current_relationship: RunCurrentRelationshipClass,
    /// Stable relationship token.
    pub old_current_relationship_token: String,
    /// Summary safe for UI, CLI/headless output, and support exports.
    pub summary: String,
}

impl RerunReviewSheet {
    /// Builds a rerun review sheet from an exact-prior/current target
    /// comparison and optional non-context drift rows.
    pub fn from_comparison(
        review_id: impl Into<String>,
        source_run: &RunSummaryCard,
        comparison: &RerunTargetComparison,
        mut extra_drift_rows: Vec<RerunReviewDriftRow>,
        generated_at: impl Into<String>,
    ) -> Self {
        let mut drift_rows: Vec<RerunReviewDriftRow> = comparison
            .diff_rows
            .iter()
            .map(RerunReviewDriftRow::from_target_diff)
            .collect();
        drift_rows.append(&mut extra_drift_rows);
        let drift_field_tokens = unique_tokens(
            drift_rows
                .iter()
                .map(|row| row.field_token.clone())
                .collect(),
        );
        let requires_review_before_dispatch = comparison.requires_review_before_dispatch
            || drift_rows
                .iter()
                .any(|row| row.requires_review_before_dispatch);
        let old_current_relationship = if source_run.current_relationship
            == RunCurrentRelationshipClass::ManualReplayRequired
        {
            RunCurrentRelationshipClass::ManualReplayRequired
        } else if requires_review_before_dispatch {
            RunCurrentRelationshipClass::StalePriorEvidence
        } else {
            RunCurrentRelationshipClass::CurrentAttempt
        };
        let summary = if drift_rows.is_empty() {
            format!(
                "Rerun review {}: exact context matches current context for run {}",
                source_run.run_id, source_run.context.canonical_target_id
            )
        } else {
            format!(
                "Rerun review for run {} requires review: drift=[{}]",
                source_run.run_id,
                drift_field_tokens.join(","),
            )
        };
        Self {
            record_kind: RERUN_REVIEW_SHEET_RECORD_KIND.to_owned(),
            schema_version: RUN_LINEAGE_SCHEMA_VERSION,
            review_id: review_id.into(),
            source_run_id: source_run.run_id.clone(),
            source_attempt_id: source_run.latest_attempt_id.clone(),
            exact_context_ref: comparison.exact_context_ref.clone(),
            current_context_ref: comparison.current_context_ref.clone(),
            generated_at: generated_at.into(),
            mode_options: vec![
                RerunReviewModeOption::new(
                    RerunReviewMode::RerunExactly,
                    &source_run.run_id,
                    comparison.exact_context_ref.clone(),
                ),
                RerunReviewModeOption::new(
                    RerunReviewMode::RerunWithCurrentContext,
                    &source_run.run_id,
                    comparison.current_context_ref.clone(),
                ),
            ],
            drift_rows,
            drift_field_tokens,
            has_drift: !comparison.diff_rows.is_empty() || requires_review_before_dispatch,
            requires_review_before_dispatch,
            old_evidence_preserved: requires_review_before_dispatch,
            old_current_relationship,
            old_current_relationship_token: old_current_relationship.as_str().to_owned(),
            summary,
        }
    }

    /// True when both exact and current-context modes are visible.
    pub fn distinguishes_exact_and_current_modes(&self) -> bool {
        let has_exact = self
            .mode_options
            .iter()
            .any(|option| option.mode == RerunReviewMode::RerunExactly);
        let has_current = self
            .mode_options
            .iter()
            .any(|option| option.mode == RerunReviewMode::RerunWithCurrentContext);
        has_exact && has_current
    }
}

/// Support/export packet carrying run history, durable rows, artifact detail
/// sheets, rerun reviews, and the interruption taxonomy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunHistorySupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Export timestamp.
    pub generated_at: String,
    /// Run summary cards.
    pub run_summaries: Vec<RunSummaryCard>,
    /// Durable job rows.
    pub durable_job_rows: Vec<DurableJobRow>,
    /// Artifact detail sheets.
    pub artifact_sheets: Vec<RunArtifactDetailSheet>,
    /// Rerun review sheets.
    pub rerun_reviews: Vec<RerunReviewSheet>,
    /// Full interruption taxonomy tokens emitted for support/export parity.
    pub interruption_taxonomy_tokens: Vec<String>,
    /// Consumer surfaces covered by this export.
    pub consumer_surface_tokens: Vec<String>,
    /// True when all rows avoid raw private material by default.
    pub raw_private_material_excluded: bool,
    /// Summary lines in stable order.
    pub summary_lines: Vec<String>,
}

impl RunHistorySupportExport {
    /// Builds a run-history support export.
    pub fn new(
        export_id: impl Into<String>,
        workspace_id: impl Into<String>,
        generated_at: impl Into<String>,
        run_summaries: Vec<RunSummaryCard>,
        durable_job_rows: Vec<DurableJobRow>,
        artifact_sheets: Vec<RunArtifactDetailSheet>,
        rerun_reviews: Vec<RerunReviewSheet>,
    ) -> Self {
        let summary_lines = run_summaries
            .iter()
            .map(|summary| {
                format!(
                    "run={} attempt={} state={} freshness={} relationship={} interruption={}",
                    summary.run_id,
                    summary.latest_attempt_id,
                    summary.lifecycle_state_token,
                    summary.freshness_token,
                    summary.current_relationship_token,
                    summary.interruption_token.as_deref().unwrap_or("none"),
                )
            })
            .collect();
        Self {
            record_kind: RUN_HISTORY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: RUN_LINEAGE_SCHEMA_VERSION,
            export_id: export_id.into(),
            workspace_id: workspace_id.into(),
            generated_at: generated_at.into(),
            run_summaries,
            durable_job_rows,
            artifact_sheets,
            rerun_reviews,
            interruption_taxonomy_tokens: RunInterruptionKind::ALL
                .into_iter()
                .map(|kind| kind.as_str().to_owned())
                .collect(),
            consumer_surface_tokens: vec![
                "desktop_ui".to_owned(),
                "activity_center".to_owned(),
                "cli_headless".to_owned(),
                "support_export".to_owned(),
            ],
            raw_private_material_excluded: true,
            summary_lines,
        }
    }

    /// Renders deterministic plaintext for CLI/headless and support export
    /// parity checks.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("Run history support export: {}\n", self.export_id));
        out.push_str(&format!("Workspace: {}\n", self.workspace_id));
        out.push_str(&format!("Generated at: {}\n", self.generated_at));
        out.push_str(&format!(
            "Consumers: {}\n",
            self.consumer_surface_tokens.join(",")
        ));
        out.push_str(&format!(
            "Interruption taxonomy: {}\n",
            self.interruption_taxonomy_tokens.join(",")
        ));
        for summary in &self.run_summaries {
            out.push_str(&format!(
                "Run {} attempt={} state={} target={} boundary={} freshness={} relationship={} interruption={}\n",
                summary.run_id,
                summary.latest_attempt_id,
                summary.lifecycle_state_token,
                summary.context.canonical_target_id,
                summary.context.boundary_token,
                summary.freshness_token,
                summary.current_relationship_token,
                summary.interruption_token.as_deref().unwrap_or("none"),
            ));
        }
        for artifact in &self.artifact_sheets {
            out.push_str(&format!(
                "Artifact {} run={} kind={} freshness={} retention={} viewer={} raw_fallback={}\n",
                artifact.artifact_id,
                artifact.producing_run_id,
                artifact.artifact_kind_token,
                artifact.freshness_token,
                artifact.retention_class_token,
                artifact.preferred_viewer_token,
                artifact.raw_fallback_available,
            ));
        }
        for review in &self.rerun_reviews {
            out.push_str(&format!(
                "Rerun review {} run={} exact={} current={} review={} fields=[{}]\n",
                review.review_id,
                review.source_run_id,
                review.exact_context_ref,
                review.current_context_ref,
                review.requires_review_before_dispatch,
                review.drift_field_tokens.join(","),
            ));
        }
        out
    }

    /// True when every durable job row remains inspectable after live surface
    /// closure.
    pub fn all_jobs_are_durable(&self) -> bool {
        self.durable_job_rows
            .iter()
            .all(|row| row.durable_after_surface_close)
    }
}

/// Seed scenario identifiers used by fixtures and integration tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunLineageSeededScenario {
    /// Current local run completed and survived sleep/resume.
    CurrentLocalPassedSleepResume,
    /// Remote run disconnected and current context drift requires review.
    RemoteDisconnectCurrentContextReview,
    /// Auth expiry stale row preserves old evidence after policy drift.
    AuthExpiryStaleEvidence,
    /// Imported stale run requires manual replay and raw fallback.
    StaleImportManualReplay,
}

impl RunLineageSeededScenario {
    /// All seeded scenarios in canonical order.
    pub const ALL: [Self; 4] = [
        Self::CurrentLocalPassedSleepResume,
        Self::RemoteDisconnectCurrentContextReview,
        Self::AuthExpiryStaleEvidence,
        Self::StaleImportManualReplay,
    ];

    /// Stable scenario token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentLocalPassedSleepResume => "current_local_passed_sleep_resume",
            Self::RemoteDisconnectCurrentContextReview => {
                "remote_disconnect_current_context_review"
            }
            Self::AuthExpiryStaleEvidence => "auth_expiry_stale_evidence",
            Self::StaleImportManualReplay => "stale_import_manual_replay",
        }
    }
}

/// Builds the canonical seeded run-history export used by fixtures and tests.
pub fn seeded_run_history_support_export(
    export_id: impl Into<String>,
    generated_at: impl Into<String>,
) -> RunHistorySupportExport {
    let generated_at = generated_at.into();
    let mut summaries = Vec::new();
    let mut jobs = Vec::new();
    let mut artifacts = Vec::new();
    let mut reviews = Vec::new();

    for scenario in RunLineageSeededScenario::ALL {
        let seed = seeded_scenario(scenario, &generated_at);
        artifacts.extend(seed.artifacts);
        if let Some(review) = seed.review {
            reviews.push(review);
        }
        jobs.push(seed.job);
        summaries.push(seed.summary);
    }

    RunHistorySupportExport::new(
        export_id,
        "ws-run-lineage-beta",
        generated_at,
        summaries,
        jobs,
        artifacts,
        reviews,
    )
}

struct SeededScenarioRecords {
    summary: RunSummaryCard,
    job: DurableJobRow,
    artifacts: Vec<RunArtifactDetailSheet>,
    review: Option<RerunReviewSheet>,
}

fn seeded_scenario(
    scenario: RunLineageSeededScenario,
    generated_at: &str,
) -> SeededScenarioRecords {
    let (mut exact_resolver, mut current_resolver) = seeded_resolvers_for(scenario);
    let (exact_request, current_request) = seeded_requests_for(scenario);
    let exact_context = exact_resolver.resolve(exact_request);
    let _ = current_resolver.resolve(
        crate::execution_context::ExecutionContextRequest::local_terminal_seed(
            "run.context.sequence_seed",
            TrustState::Trusted,
            "2026-05-18T15:59:59Z",
        ),
    );
    let current_context = current_resolver.resolve(current_request);
    let comparison = RerunTargetComparison::compare(&exact_context, &current_context);

    let run_id = format!("run:lineage:{}", scenario.as_str());
    let attempt_id = format!("attempt:lineage:{}:1", scenario.as_str());
    let (state, freshness, relationship, interruption, artifact_kind, retention, viewer, fallback) =
        seeded_surface_state(scenario);
    let summary = RunSummaryCard::from_context(
        run_id.clone(),
        attempt_id,
        seeded_title(scenario),
        "user_command",
        seeded_wedge(scenario),
        &exact_context,
        seeded_build_identity(scenario),
        "2026-05-18T16:00:00Z",
        Some("2026-05-18T16:02:00Z".to_owned()),
        state,
        freshness,
        relationship,
        interruption,
        1,
        vec![format!("evidence:{}", scenario.as_str())],
    );
    let job = DurableJobRow::from_summary(&summary, generated_at, seeded_continuity(scenario));
    let artifacts = vec![RunArtifactDetailSheet::new(
        format!("artifact:lineage:{}", scenario.as_str()),
        artifact_kind,
        &summary,
        freshness,
        retention,
        ExecutionProvenanceRedactionClass::MetadataSafeDefault,
        viewer,
        fallback,
    )];
    let review = if comparison.has_drift || requires_seed_review(scenario) {
        Some(RerunReviewSheet::from_comparison(
            format!("rerun-review:{}", scenario.as_str()),
            &summary,
            &comparison,
            seeded_extra_drift_rows(scenario),
            generated_at,
        ))
    } else {
        None
    };
    SeededScenarioRecords {
        summary,
        job,
        artifacts,
        review,
    }
}

fn seeded_resolvers_for(
    scenario: RunLineageSeededScenario,
) -> (
    crate::execution_context::ExecutionContextResolver,
    crate::execution_context::ExecutionContextResolver,
) {
    use crate::execution_context::{
        CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextResolver,
        ExecutionContextResolverConfig, IdentityMode, ScopeClass,
    };

    let (
        exact_target,
        current_target,
        exact_policy,
        current_policy,
        exact_capsule,
        current_capsule,
    ) = match scenario {
        RunLineageSeededScenario::CurrentLocalPassedSleepResume => (
            TargetClass::LocalHost,
            TargetClass::LocalHost,
            1,
            1,
            ("sha256:lineage-current", CapsuleDriftState::InSync),
            ("sha256:lineage-current", CapsuleDriftState::InSync),
        ),
        RunLineageSeededScenario::RemoteDisconnectCurrentContextReview => (
            TargetClass::SshRemote,
            TargetClass::ManagedWorkspace,
            2,
            3,
            ("sha256:lineage-remote", CapsuleDriftState::InSync),
            ("sha256:lineage-managed", CapsuleDriftState::StaleInputs),
        ),
        RunLineageSeededScenario::AuthExpiryStaleEvidence => (
            TargetClass::ManagedWorkspace,
            TargetClass::ManagedWorkspace,
            4,
            5,
            ("sha256:lineage-auth-old", CapsuleDriftState::InSync),
            ("sha256:lineage-auth-current", CapsuleDriftState::InSync),
        ),
        RunLineageSeededScenario::StaleImportManualReplay => (
            TargetClass::RemoteWorkspaceVm,
            TargetClass::LocalHost,
            6,
            6,
            ("sha256:lineage-imported", CapsuleDriftState::UnknownLineage),
            ("sha256:lineage-current-local", CapsuleDriftState::InSync),
        ),
    };

    let resolver = |target_class, policy_epoch, capsule_hash: &str, drift_state| {
        ExecutionContextResolver::new(ExecutionContextResolverConfig {
            workspace_id: "ws-run-lineage-beta".to_owned(),
            profile_id: Some("profile:run-lineage".to_owned()),
            identity_mode: IdentityMode::AccountFreeLocal,
            policy_epoch,
            workspace_default_target_class: target_class,
            workspace_default_working_directory: Some("/workspace/run-lineage".to_owned()),
            workspace_default_scope_class: ScopeClass::CurrentRoot,
            local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
            environment_capsule_ref: EnvironmentCapsuleRef {
                capsule_id: "caps:run-lineage".to_owned(),
                capsule_hash: capsule_hash.to_owned(),
                resolved_schema_version: "1".to_owned(),
                drift_state,
            },
            resolver_version: "run-lineage-beta-seed".to_owned(),
        })
    };

    (
        resolver(exact_target, exact_policy, exact_capsule.0, exact_capsule.1),
        resolver(
            current_target,
            current_policy,
            current_capsule.0,
            current_capsule.1,
        ),
    )
}

fn seeded_requests_for(
    scenario: RunLineageSeededScenario,
) -> (
    crate::execution_context::ExecutionContextRequest<'static>,
    crate::execution_context::ExecutionContextRequest<'static>,
) {
    use crate::execution_context::ExecutionContextRequest;

    match scenario {
        RunLineageSeededScenario::CurrentLocalPassedSleepResume => {
            let exact = ExecutionContextRequest::task_seed(
                "task.run.seeded",
                TrustState::Trusted,
                "2026-05-18T16:00:00Z",
            );
            let current = ExecutionContextRequest::task_seed(
                "task.run.seeded",
                TrustState::Trusted,
                "2026-05-18T16:01:00Z",
            );
            (exact, current)
        }
        RunLineageSeededScenario::RemoteDisconnectCurrentContextReview => {
            let mut exact = ExecutionContextRequest::task_seed(
                "task.run.remote",
                TrustState::Trusted,
                "2026-05-18T16:10:00Z",
            );
            exact.override_target_class = Some(TargetClass::SshRemote);
            let mut current = ExecutionContextRequest::task_seed(
                "task.run.remote",
                TrustState::Restricted,
                "2026-05-18T16:11:00Z",
            );
            current.override_target_class = Some(TargetClass::ManagedWorkspace);
            (exact, current)
        }
        RunLineageSeededScenario::AuthExpiryStaleEvidence => {
            let mut exact = ExecutionContextRequest::test_seed(
                "test.run.managed",
                TrustState::Trusted,
                "2026-05-18T16:20:00Z",
            );
            exact.override_target_class = Some(TargetClass::ManagedWorkspace);
            let mut current = ExecutionContextRequest::test_seed(
                "test.run.managed",
                TrustState::PendingEvaluation,
                "2026-05-18T16:21:00Z",
            );
            current.override_target_class = Some(TargetClass::ManagedWorkspace);
            (exact, current)
        }
        RunLineageSeededScenario::StaleImportManualReplay => {
            let mut exact = ExecutionContextRequest::debug_prep_seed(
                "debug.run.imported",
                TrustState::Trusted,
                "2026-05-18T16:30:00Z",
            );
            exact.override_target_class = Some(TargetClass::RemoteWorkspaceVm);
            let mut current = ExecutionContextRequest::debug_prep_seed(
                "debug.run.imported",
                TrustState::Trusted,
                "2026-05-18T16:31:00Z",
            );
            current.override_target_class = Some(TargetClass::LocalHost);
            (exact, current)
        }
    }
}

fn seeded_surface_state(
    scenario: RunLineageSeededScenario,
) -> (
    RunLifecycleState,
    RunFreshnessClass,
    RunCurrentRelationshipClass,
    Option<RunInterruptionKind>,
    RunArtifactKind,
    RunArtifactRetentionClass,
    RunArtifactViewerClass,
    bool,
) {
    match scenario {
        RunLineageSeededScenario::CurrentLocalPassedSleepResume => (
            RunLifecycleState::Passed,
            RunFreshnessClass::Current,
            RunCurrentRelationshipClass::CurrentAttempt,
            None,
            RunArtifactKind::ValidationSummary,
            RunArtifactRetentionClass::RetainedRedacted,
            RunArtifactViewerClass::StructuredViewer,
            true,
        ),
        RunLineageSeededScenario::RemoteDisconnectCurrentContextReview => (
            RunLifecycleState::Failed,
            RunFreshnessClass::Stale,
            RunCurrentRelationshipClass::StalePriorEvidence,
            Some(RunInterruptionKind::RemoteDisconnect),
            RunArtifactKind::LogSlice,
            RunArtifactRetentionClass::RetainedRedacted,
            RunArtifactViewerClass::LogViewer,
            true,
        ),
        RunLineageSeededScenario::AuthExpiryStaleEvidence => (
            RunLifecycleState::Cancelled,
            RunFreshnessClass::Stale,
            RunCurrentRelationshipClass::StalePriorEvidence,
            Some(RunInterruptionKind::AuthExpiry),
            RunArtifactKind::TestReport,
            RunArtifactRetentionClass::MetadataOnly,
            RunArtifactViewerClass::StructuredViewer,
            true,
        ),
        RunLineageSeededScenario::StaleImportManualReplay => (
            RunLifecycleState::StaleOutputPartialParse,
            RunFreshnessClass::Imported,
            RunCurrentRelationshipClass::ManualReplayRequired,
            Some(RunInterruptionKind::ManualReplayRequirement),
            RunArtifactKind::SourceMap,
            RunArtifactRetentionClass::ProviderOnly,
            RunArtifactViewerClass::DownloadOnly,
            true,
        ),
    }
}

fn seeded_extra_drift_rows(scenario: RunLineageSeededScenario) -> Vec<RerunReviewDriftRow> {
    match scenario {
        RunLineageSeededScenario::CurrentLocalPassedSleepResume => Vec::new(),
        RunLineageSeededScenario::RemoteDisconnectCurrentContextReview => vec![
            RerunReviewDriftRow::manual(
                RerunReviewDriftField::Code,
                "build_identity.commit_ref",
                "Commit",
                Some("commit:old-remote".to_owned()),
                Some("commit:current-managed".to_owned()),
            ),
            RerunReviewDriftRow::manual(
                RerunReviewDriftField::Branch,
                "build_identity.branch",
                "Branch",
                Some("feature/remote".to_owned()),
                Some("main".to_owned()),
            ),
            RerunReviewDriftRow::manual(
                RerunReviewDriftField::Secrets,
                "secret_handles.runtime",
                "Secret handle",
                Some("secret:managed:valid".to_owned()),
                Some("secret:managed:expired".to_owned()),
            ),
        ],
        RunLineageSeededScenario::AuthExpiryStaleEvidence => vec![
            RerunReviewDriftRow::manual(
                RerunReviewDriftField::Config,
                "workspace.config_digest",
                "Config digest",
                Some("sha256:old-config".to_owned()),
                Some("sha256:new-config".to_owned()),
            ),
            RerunReviewDriftRow::manual(
                RerunReviewDriftField::Locale,
                "workspace.locale",
                "Locale",
                Some("en-US".to_owned()),
                Some("fr-FR".to_owned()),
            ),
            RerunReviewDriftRow::manual(
                RerunReviewDriftField::Secrets,
                "secret_handles.runtime",
                "Secret handle",
                Some("secret:managed:expired".to_owned()),
                Some("secret:managed:reauth_required".to_owned()),
            ),
        ],
        RunLineageSeededScenario::StaleImportManualReplay => vec![
            RerunReviewDriftRow::manual(
                RerunReviewDriftField::Code,
                "imported.commit_ref",
                "Imported commit",
                Some("commit:provider-only".to_owned()),
                Some("commit:local-current".to_owned()),
            ),
            RerunReviewDriftRow::manual(
                RerunReviewDriftField::Branch,
                "imported.branch",
                "Imported branch",
                Some("provider/main".to_owned()),
                Some("local/main".to_owned()),
            ),
        ],
    }
}

fn seeded_continuity(scenario: RunLineageSeededScenario) -> Vec<RunContinuityMarker> {
    match scenario {
        RunLineageSeededScenario::CurrentLocalPassedSleepResume => vec![
            RunContinuityMarker::LookAwayReturn,
            RunContinuityMarker::SleepResume,
            RunContinuityMarker::WindowSwitch,
        ],
        RunLineageSeededScenario::RemoteDisconnectCurrentContextReview => vec![
            RunContinuityMarker::LookAwayReturn,
            RunContinuityMarker::RuntimeRestart,
        ],
        RunLineageSeededScenario::AuthExpiryStaleEvidence => vec![
            RunContinuityMarker::SleepResume,
            RunContinuityMarker::RuntimeRestart,
        ],
        RunLineageSeededScenario::StaleImportManualReplay => vec![
            RunContinuityMarker::ImportedReplay,
            RunContinuityMarker::WindowSwitch,
        ],
    }
}

fn seeded_title(scenario: RunLineageSeededScenario) -> &'static str {
    match scenario {
        RunLineageSeededScenario::CurrentLocalPassedSleepResume => "Local validation run",
        RunLineageSeededScenario::RemoteDisconnectCurrentContextReview => "Remote build run",
        RunLineageSeededScenario::AuthExpiryStaleEvidence => "Managed test run",
        RunLineageSeededScenario::StaleImportManualReplay => "Imported debug run",
    }
}

fn seeded_wedge(scenario: RunLineageSeededScenario) -> TaskWedgeClass {
    match scenario {
        RunLineageSeededScenario::CurrentLocalPassedSleepResume => TaskWedgeClass::Build,
        RunLineageSeededScenario::RemoteDisconnectCurrentContextReview => TaskWedgeClass::Build,
        RunLineageSeededScenario::AuthExpiryStaleEvidence => TaskWedgeClass::Test,
        RunLineageSeededScenario::StaleImportManualReplay => TaskWedgeClass::Debug,
    }
}

fn seeded_build_identity(scenario: RunLineageSeededScenario) -> RunBuildIdentity {
    match scenario {
        RunLineageSeededScenario::CurrentLocalPassedSleepResume => {
            RunBuildIdentity::new("main", "commit:current", "build:local:001")
        }
        RunLineageSeededScenario::RemoteDisconnectCurrentContextReview => {
            RunBuildIdentity::new("feature/remote", "commit:old-remote", "build:remote:002")
        }
        RunLineageSeededScenario::AuthExpiryStaleEvidence => {
            RunBuildIdentity::new("main", "commit:auth-old", "build:managed:003")
        }
        RunLineageSeededScenario::StaleImportManualReplay => RunBuildIdentity::new(
            "provider/main",
            "commit:provider-only",
            "build:provider:004",
        ),
    }
}

fn requires_seed_review(scenario: RunLineageSeededScenario) -> bool {
    !matches!(
        scenario,
        RunLineageSeededScenario::CurrentLocalPassedSleepResume
    )
}

fn artifact_actions_for(
    artifact_id: &str,
    preferred_viewer: RunArtifactViewerClass,
    raw_fallback_available: bool,
    retention_class: RunArtifactRetentionClass,
) -> Vec<RunActionRef> {
    let mut actions = Vec::new();
    if !matches!(preferred_viewer, RunArtifactViewerClass::DownloadOnly) {
        actions.push(RunActionRef::new(
            RunArtifactActionClass::OpenViewer.as_str(),
            format!("action:artifact:{artifact_id}:open_viewer"),
            RunArtifactActionClass::OpenViewer.label(),
        ));
    }
    if raw_fallback_available {
        actions.push(RunActionRef::new(
            RunArtifactActionClass::OpenRawFallback.as_str(),
            format!("action:artifact:{artifact_id}:open_raw"),
            RunArtifactActionClass::OpenRawFallback.label(),
        ));
    }
    actions.push(RunActionRef::new(
        RunArtifactActionClass::ExportRedacted.as_str(),
        format!("action:artifact:{artifact_id}:export_redacted"),
        RunArtifactActionClass::ExportRedacted.label(),
    ));
    if matches!(retention_class, RunArtifactRetentionClass::PinnedEvidence) {
        actions.push(RunActionRef::new(
            RunArtifactActionClass::PinArtifact.as_str(),
            format!("action:artifact:{artifact_id}:pin"),
            RunArtifactActionClass::PinArtifact.label(),
        ));
    }
    actions.push(RunActionRef::new(
        RunArtifactActionClass::ReviewRedaction.as_str(),
        format!("action:artifact:{artifact_id}:review_redaction"),
        RunArtifactActionClass::ReviewRedaction.label(),
    ));
    actions
}

fn unique_tokens(tokens: Vec<String>) -> Vec<String> {
    let mut unique = Vec::new();
    for token in tokens {
        if !unique.contains(&token) {
            unique.push(token);
        }
    }
    unique
}

const fn trust_state_token(state: TrustState) -> &'static str {
    match state {
        TrustState::Trusted => "trusted",
        TrustState::Restricted => "restricted",
        TrustState::PendingEvaluation => "pending_evaluation",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_export_covers_every_interruption_token() {
        let export =
            seeded_run_history_support_export("run-history-support:test", "2026-05-18T16:45:00Z");
        for kind in RunInterruptionKind::ALL {
            assert!(export
                .interruption_taxonomy_tokens
                .contains(&kind.as_str().to_owned()));
        }
        let text = export.render_plaintext();
        assert!(text.contains("remote_disconnect"));
        assert!(text.contains("auth_expiry"));
        assert!(text.contains("manual_replay_requirement"));
    }

    #[test]
    fn stale_runs_preserve_old_evidence() {
        let export =
            seeded_run_history_support_export("run-history-support:test", "2026-05-18T16:45:00Z");
        assert!(export
            .durable_job_rows
            .iter()
            .any(DurableJobRow::preserves_old_evidence));
        assert!(export
            .run_summaries
            .iter()
            .any(|summary| summary.old_evidence_preserved));
    }

    #[test]
    fn rerun_reviews_distinguish_exact_and_current_modes() {
        let export =
            seeded_run_history_support_export("run-history-support:test", "2026-05-18T16:45:00Z");
        assert!(!export.rerun_reviews.is_empty());
        for review in &export.rerun_reviews {
            assert!(review.distinguishes_exact_and_current_modes());
            assert!(review.requires_review_before_dispatch);
            assert!(review.old_evidence_preserved);
        }
    }
}
