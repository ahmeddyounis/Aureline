//! Frozen M5 lifecycle-state, degraded-vocabulary, and critical-journey
//! checkpoint matrix.
//!
//! This module locks Aureline's canonical M5 object-state model and its
//! protected-journey checkpoint inventory into one export-safe packet. Every
//! long-lived M5 object family — the workspace, the extension, the remote
//! session, the collaboration session, the AI action, the update/rollback, the
//! notebook runtime, the request/API run, the preview session, the pipeline run,
//! the data session, the profiler capture, and the companion session — is bound
//! to an explicit state machine drawn from one controlled state vocabulary, one
//! visible primary status surface, one exportable status code, one controlled
//! last-failure reason, and one named recovery affordance.
//!
//! The critical-journey inventory names, for each protected journey, the ordered
//! milestone checkpoints it shows instead of an anonymous spinner, so support,
//! automation, docs, and telemetry share the same checkpoint boundaries.
//!
//! The matrix is the single source of truth for whether a claimed M5 surface may
//! publish a lifecycle-state or checkpoint claim. Product UI, CLI, docs/help,
//! diagnostics, support export, telemetry, and claim tooling consume this packet
//! rather than inventing private state vocabularies or anonymous checkpoints:
//! `stale` always means the same reserved state, a status code always exports the
//! same way, a last-failure reason is always named from one controlled set, and a
//! recovery affordance is always the named action the user can take.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5LifecycleVocabularySet`] rather than minted per surface. Raw URLs, raw
//! local paths, raw usernames, raw hostnames, tokens, raw diagnostics, private
//! endpoints, credentials, and user text bodies stay outside the support
//! boundary.
//!
//! The object-state boundary schema is
//! [`schemas/lifecycle/m5-object-state.schema.json`](../../../../schemas/lifecycle/m5-object-state.schema.json)
//! and the journey-checkpoint schema is
//! [`schemas/lifecycle/m5-journey-checkpoint.schema.json`](../../../../schemas/lifecycle/m5-journey-checkpoint.schema.json).
//! The contract doc is
//! [`docs/lifecycle/m5_lifecycle_matrix_contract.md`](../../../../docs/lifecycle/m5_lifecycle_matrix_contract.md).
//! The protected fixture directory is
//! [`fixtures/state/m5-lifecycle-scenarios/`](../../../../fixtures/state/m5-lifecycle-scenarios/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_lifecycle_matrix, seeded_m5_lifecycle_matrix_notebook_runtime_retest_narrowed,
    seeded_m5_lifecycle_matrix_remote_session_degraded_narrowed, M5_LIFECYCLE_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5LifecycleMatrixPacket`].
pub const M5_LIFECYCLE_MATRIX_RECORD_KIND: &str =
    "freeze_m5_lifecycle_state_and_journey_checkpoint_matrix";

/// Schema version for M5 lifecycle-matrix records.
pub const M5_LIFECYCLE_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the object-state boundary schema.
pub const M5_LIFECYCLE_OBJECT_STATE_SCHEMA_REF: &str =
    "schemas/lifecycle/m5-object-state.schema.json";

/// Repo-relative path of the journey-checkpoint schema.
pub const M5_LIFECYCLE_JOURNEY_CHECKPOINT_SCHEMA_REF: &str =
    "schemas/lifecycle/m5-journey-checkpoint.schema.json";

/// Repo-relative path of the M5 lifecycle matrix contract doc.
pub const M5_LIFECYCLE_MATRIX_DOC_REF: &str = "docs/lifecycle/m5_lifecycle_matrix_contract.md";

/// Repo-relative path of the state-object inventory this matrix builds on.
pub const M5_LIFECYCLE_STATE_OBJECT_INVENTORY_REF: &str = "docs/state/state_object_inventory.md";

/// Repo-relative path of the state-class recovery reference this matrix mirrors.
pub const M5_LIFECYCLE_STATE_CLASS_RECOVERY_REF: &str = "docs/state/state_class_recovery.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_LIFECYCLE_MATRIX_FIXTURE_DIR: &str = "fixtures/state/m5-lifecycle-scenarios";

/// Repo-relative path of the checked support-export artifact.
pub const M5_LIFECYCLE_MATRIX_ARTIFACT_REF: &str =
    "artifacts/release/m5-lifecycle-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_LIFECYCLE_MATRIX_CSV_REF: &str = "artifacts/release/m5-lifecycle-proof/matrix.csv";

/// Repo-relative path of the checked Markdown lifecycle report.
pub const M5_LIFECYCLE_MATRIX_REPORT_REF: &str = "artifacts/lifecycle/m5-lifecycle-matrix.md";

/// One of the thirteen governed long-lived M5 object families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LifecycleObjectFamily {
    /// Workspace / window session.
    Workspace,
    /// Installed extension / capability.
    Extension,
    /// Remote / tunnel / SSH session.
    RemoteSession,
    /// Live collaboration session.
    CollaborationSession,
    /// AI assistant action / run.
    AiAction,
    /// Update / rollback lifecycle.
    UpdateRollback,
    /// Notebook kernel runtime.
    NotebookRuntime,
    /// Request / API run.
    RequestApiRun,
    /// Preview / live-server session.
    PreviewSession,
    /// Pipeline / task run.
    PipelineRun,
    /// Data / database session.
    DataSession,
    /// Profiler / trace capture.
    ProfilerCapture,
    /// Companion / paired device session.
    CompanionSession,
}

impl M5LifecycleObjectFamily {
    /// Every governed object family, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::Workspace,
        Self::Extension,
        Self::RemoteSession,
        Self::CollaborationSession,
        Self::AiAction,
        Self::UpdateRollback,
        Self::NotebookRuntime,
        Self::RequestApiRun,
        Self::PreviewSession,
        Self::PipelineRun,
        Self::DataSession,
        Self::ProfilerCapture,
        Self::CompanionSession,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Workspace => "workspace",
            Self::Extension => "extension",
            Self::RemoteSession => "remote_session",
            Self::CollaborationSession => "collaboration_session",
            Self::AiAction => "ai_action",
            Self::UpdateRollback => "update_rollback",
            Self::NotebookRuntime => "notebook_runtime",
            Self::RequestApiRun => "request_api_run",
            Self::PreviewSession => "preview_session",
            Self::PipelineRun => "pipeline_run",
            Self::DataSession => "data_session",
            Self::ProfilerCapture => "profiler_capture",
            Self::CompanionSession => "companion_session",
        }
    }
}

/// Controlled lifecycle state — the frozen degraded-vocabulary vocabulary every
/// governed object machine draws from.
///
/// These fifteen tokens keep the same meaning across UI, CLI, docs, support
/// exports, and telemetry. A later M5 row may not invent a private state term.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LifecycleState {
    /// Fully ready for its declared scope.
    Ready,
    /// Warming up; not yet complete but progressing.
    Warming,
    /// Partially ready; a subset of the object is usable.
    Partial,
    /// Prior value shown after its freshness floor was lost.
    Stale,
    /// Rebuilding / recomputing derived state.
    Rebuilding,
    /// Access restricted by scope or entitlement.
    Restricted,
    /// Blocked by policy or legal control.
    PolicyBlocked,
    /// Connection dropped; actively reconnecting.
    Reconnecting,
    /// Degraded; reduced capability with disclosure.
    Degraded,
    /// Read-only degraded; safe read path preserved, writes withheld.
    ReadOnlyDegraded,
    /// Unavailable on this build or context.
    Unavailable,
    /// A safe rollback is available.
    RollbackAvailable,
    /// Deprecated; superseded and scheduled for removal.
    Deprecated,
    /// Experimental; not claimed for the current milestone.
    Experimental,
    /// Awaiting a re-test before it can re-qualify.
    RetestPending,
}

impl M5LifecycleState {
    /// Every controlled state, in declaration order.
    pub const ALL: [Self; 15] = [
        Self::Ready,
        Self::Warming,
        Self::Partial,
        Self::Stale,
        Self::Rebuilding,
        Self::Restricted,
        Self::PolicyBlocked,
        Self::Reconnecting,
        Self::Degraded,
        Self::ReadOnlyDegraded,
        Self::Unavailable,
        Self::RollbackAvailable,
        Self::Deprecated,
        Self::Experimental,
        Self::RetestPending,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Warming => "warming",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Rebuilding => "rebuilding",
            Self::Restricted => "restricted",
            Self::PolicyBlocked => "policy_blocked",
            Self::Reconnecting => "reconnecting",
            Self::Degraded => "degraded",
            Self::ReadOnlyDegraded => "read_only_degraded",
            Self::Unavailable => "unavailable",
            Self::RollbackAvailable => "rollback_available",
            Self::Deprecated => "deprecated",
            Self::Experimental => "experimental",
            Self::RetestPending => "retest_pending",
        }
    }
}

/// The single visible primary status surface an object binds its state to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PrimaryStatusSurface {
    /// Global status-bar indicator.
    StatusBarIndicator,
    /// Object header / title badge.
    ObjectHeaderBadge,
    /// Panel header status region.
    PanelHeaderStatus,
    /// Inline action status chip.
    InlineActionStatus,
    /// Activity-center row.
    ActivityCenterRow,
    /// Update-center row.
    UpdateCenterRow,
    /// Remote presence indicator.
    RemotePresenceIndicator,
    /// Collaboration presence badge.
    CollaborationPresenceBadge,
}

impl M5PrimaryStatusSurface {
    /// Every primary status surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::StatusBarIndicator,
        Self::ObjectHeaderBadge,
        Self::PanelHeaderStatus,
        Self::InlineActionStatus,
        Self::ActivityCenterRow,
        Self::UpdateCenterRow,
        Self::RemotePresenceIndicator,
        Self::CollaborationPresenceBadge,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StatusBarIndicator => "status_bar_indicator",
            Self::ObjectHeaderBadge => "object_header_badge",
            Self::PanelHeaderStatus => "panel_header_status",
            Self::InlineActionStatus => "inline_action_status",
            Self::ActivityCenterRow => "activity_center_row",
            Self::UpdateCenterRow => "update_center_row",
            Self::RemotePresenceIndicator => "remote_presence_indicator",
            Self::CollaborationPresenceBadge => "collaboration_presence_badge",
        }
    }
}

/// The one named recovery affordance an object surfaces for its degraded states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RecoveryAffordance {
    /// Retry the last operation.
    RetryAction,
    /// Reconnect a dropped session.
    ReconnectAction,
    /// Rebuild / recompute derived state.
    RebuildAction,
    /// Restore from the last durable snapshot.
    RestoreAction,
    /// Roll back to the prior known-good version.
    RollbackAction,
    /// Request access / elevated scope.
    RequestAccessAction,
    /// Re-run the qualifying test.
    RetestAction,
    /// Reopen in a safe read-only mode.
    ReopenReadOnlyAction,
    /// Review the policy or legal block.
    ReviewPolicyAction,
    /// Reinstall / re-enable the capability.
    ReinstallAction,
    /// Apply the available update now.
    UpdateNowAction,
    /// Contact support with an export.
    ContactSupportAction,
}

impl M5RecoveryAffordance {
    /// Every recovery affordance, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::RetryAction,
        Self::ReconnectAction,
        Self::RebuildAction,
        Self::RestoreAction,
        Self::RollbackAction,
        Self::RequestAccessAction,
        Self::RetestAction,
        Self::ReopenReadOnlyAction,
        Self::ReviewPolicyAction,
        Self::ReinstallAction,
        Self::UpdateNowAction,
        Self::ContactSupportAction,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetryAction => "retry_action",
            Self::ReconnectAction => "reconnect_action",
            Self::RebuildAction => "rebuild_action",
            Self::RestoreAction => "restore_action",
            Self::RollbackAction => "rollback_action",
            Self::RequestAccessAction => "request_access_action",
            Self::RetestAction => "retest_action",
            Self::ReopenReadOnlyAction => "reopen_read_only_action",
            Self::ReviewPolicyAction => "review_policy_action",
            Self::ReinstallAction => "reinstall_action",
            Self::UpdateNowAction => "update_now_action",
            Self::ContactSupportAction => "contact_support_action",
        }
    }
}

/// Controlled last-failure reason class an object reports; never raw text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LastFailureReasonClass {
    /// No failure recorded.
    NoFailure,
    /// A required dependency was unavailable.
    DependencyUnavailable,
    /// Permission was denied.
    PermissionDenied,
    /// A policy or legal control blocked the operation.
    PolicyBlock,
    /// A connection was lost.
    ConnectionLost,
    /// A timeout was exceeded.
    TimeoutExceeded,
    /// A version was incompatible.
    VersionIncompatible,
    /// A resource was exhausted.
    ResourceExhausted,
    /// An integrity check failed.
    IntegrityCheckFailed,
    /// An upstream object degraded.
    UpstreamDegraded,
    /// The user cancelled.
    UserCancelled,
}

impl M5LastFailureReasonClass {
    /// Every last-failure reason class, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::NoFailure,
        Self::DependencyUnavailable,
        Self::PermissionDenied,
        Self::PolicyBlock,
        Self::ConnectionLost,
        Self::TimeoutExceeded,
        Self::VersionIncompatible,
        Self::ResourceExhausted,
        Self::IntegrityCheckFailed,
        Self::UpstreamDegraded,
        Self::UserCancelled,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoFailure => "no_failure",
            Self::DependencyUnavailable => "dependency_unavailable",
            Self::PermissionDenied => "permission_denied",
            Self::PolicyBlock => "policy_block",
            Self::ConnectionLost => "connection_lost",
            Self::TimeoutExceeded => "timeout_exceeded",
            Self::VersionIncompatible => "version_incompatible",
            Self::ResourceExhausted => "resource_exhausted",
            Self::IntegrityCheckFailed => "integrity_check_failed",
            Self::UpstreamDegraded => "upstream_degraded",
            Self::UserCancelled => "user_cancelled",
        }
    }
}

/// Controlled milestone checkpoint a protected journey shows instead of an
/// anonymous spinner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5JourneyCheckpoint {
    /// Queued / requested.
    Queued,
    /// Authorizing / checking entitlement.
    Authorizing,
    /// Preparing / provisioning.
    Preparing,
    /// Connecting to a remote or dependency.
    Connecting,
    /// Restoring durable state.
    Restoring,
    /// Building / compiling.
    Building,
    /// Warming caches / runtimes.
    Warming,
    /// Verifying integrity / readiness.
    Verifying,
    /// Finalizing / committing.
    Finalizing,
    /// Ready terminal.
    Ready,
    /// Partial-ready terminal (a subset is usable).
    PartialReady,
    /// Recoverable-failure terminal (a named recovery affordance applies).
    RecoverableFailure,
}

impl M5JourneyCheckpoint {
    /// Every checkpoint, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Queued,
        Self::Authorizing,
        Self::Preparing,
        Self::Connecting,
        Self::Restoring,
        Self::Building,
        Self::Warming,
        Self::Verifying,
        Self::Finalizing,
        Self::Ready,
        Self::PartialReady,
        Self::RecoverableFailure,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Authorizing => "authorizing",
            Self::Preparing => "preparing",
            Self::Connecting => "connecting",
            Self::Restoring => "restoring",
            Self::Building => "building",
            Self::Warming => "warming",
            Self::Verifying => "verifying",
            Self::Finalizing => "finalizing",
            Self::Ready => "ready",
            Self::PartialReady => "partial_ready",
            Self::RecoverableFailure => "recoverable_failure",
        }
    }

    /// Whether this checkpoint is a valid terminal milestone.
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Ready | Self::PartialReady | Self::RecoverableFailure
        )
    }
}

/// A protected critical journey that must show named checkpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CriticalJourney {
    /// Restoring a workspace / window session.
    WorkspaceRestore,
    /// Reconnecting a remote session.
    RemoteReconnect,
    /// Activating an extension.
    ExtensionActivation,
    /// Joining a collaboration session.
    CollaborationJoin,
    /// Running an AI action.
    AiActionRun,
    /// Applying an update or rollback.
    UpdateRollbackJourney,
    /// Executing a notebook.
    NotebookExecution,
    /// Running a request / API call.
    RequestRun,
    /// Building a preview session.
    PreviewBuild,
    /// Running a pipeline.
    PipelineRunJourney,
    /// Connecting a data session.
    DataSessionConnect,
    /// Capturing a profiler trace.
    ProfilerCaptureJourney,
    /// Attaching a companion session.
    CompanionAttach,
}

impl M5CriticalJourney {
    /// Every protected journey, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::WorkspaceRestore,
        Self::RemoteReconnect,
        Self::ExtensionActivation,
        Self::CollaborationJoin,
        Self::AiActionRun,
        Self::UpdateRollbackJourney,
        Self::NotebookExecution,
        Self::RequestRun,
        Self::PreviewBuild,
        Self::PipelineRunJourney,
        Self::DataSessionConnect,
        Self::ProfilerCaptureJourney,
        Self::CompanionAttach,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceRestore => "workspace_restore",
            Self::RemoteReconnect => "remote_reconnect",
            Self::ExtensionActivation => "extension_activation",
            Self::CollaborationJoin => "collaboration_join",
            Self::AiActionRun => "ai_action_run",
            Self::UpdateRollbackJourney => "update_rollback_journey",
            Self::NotebookExecution => "notebook_execution",
            Self::RequestRun => "request_run",
            Self::PreviewBuild => "preview_build",
            Self::PipelineRunJourney => "pipeline_run_journey",
            Self::DataSessionConnect => "data_session_connect",
            Self::ProfilerCaptureJourney => "profiler_capture_journey",
            Self::CompanionAttach => "companion_attach",
        }
    }
}

/// Qualification class for an M5 lifecycle object or journey.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LifecycleQualificationClass {
    /// Object qualifies for the Stable claim.
    Stable,
    /// Object is narrowed to Beta.
    Beta,
    /// Object is narrowed to Preview.
    Preview,
    /// Object is experimental and not claimed.
    Experimental,
    /// Object is unavailable on this build.
    Unavailable,
    /// Object is held pending upstream resolution.
    Held,
}

impl M5LifecycleQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the object may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Evidence requirement level for a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LifecycleEvidenceRequirement {
    /// At least one proof packet is required.
    Required,
    /// Proof is recommended but not blocking.
    Recommended,
    /// Proof is optional.
    Optional,
    /// Not applicable for this object's current qualification.
    NotApplicable,
}

impl M5LifecycleEvidenceRequirement {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Recommended => "recommended",
            Self::Optional => "optional",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Downgrade trigger that narrows a lifecycle object or journey below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LifecycleDowngradeTrigger {
    /// The object's state machine drifted from the controlled vocabulary.
    StateVocabularyDrift,
    /// The object lost its one visible primary status surface.
    StatusSurfaceMissing,
    /// The object's status code stopped exporting.
    StatusCodeUnexportable,
    /// The object stopped reporting a controlled last-failure reason.
    LastFailureReasonMissing,
    /// The object lost its named recovery affordance.
    RecoveryAffordanceMissing,
    /// A protected journey fell back to an anonymous checkpoint / spinner.
    AnonymousCheckpoint,
    /// The proof packet has gone stale.
    ProofStale,
    /// A policy or legal block applies.
    PolicyBlocked,
    /// An upstream dependency object narrowed.
    UpstreamDependencyNarrowed,
}

impl M5LifecycleDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::StateVocabularyDrift,
        Self::StatusSurfaceMissing,
        Self::StatusCodeUnexportable,
        Self::LastFailureReasonMissing,
        Self::RecoveryAffordanceMissing,
        Self::AnonymousCheckpoint,
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateVocabularyDrift => "state_vocabulary_drift",
            Self::StatusSurfaceMissing => "status_surface_missing",
            Self::StatusCodeUnexportable => "status_code_unexportable",
            Self::LastFailureReasonMissing => "last_failure_reason_missing",
            Self::RecoveryAffordanceMissing => "recovery_affordance_missing",
            Self::AnonymousCheckpoint => "anonymous_checkpoint",
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must project a lifecycle object's state truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LifecycleConsumerSurface {
    /// Product UI surface.
    ProductUi,
    /// Command-line interface.
    Cli,
    /// Docs / help surface.
    DocsHelp,
    /// Diagnostics / doctor surface.
    Diagnostics,
    /// Support / export packet.
    SupportExport,
    /// Telemetry stream.
    Telemetry,
    /// Claim publication tooling.
    ClaimTooling,
    /// Release notes.
    ReleaseNotes,
}

impl M5LifecycleConsumerSurface {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductUi => "product_ui",
            Self::Cli => "cli",
            Self::DocsHelp => "docs_help",
            Self::Diagnostics => "diagnostics",
            Self::SupportExport => "support_export",
            Self::Telemetry => "telemetry",
            Self::ClaimTooling => "claim_tooling",
            Self::ReleaseNotes => "release_notes",
        }
    }
}

/// One row in the object-state section of the matrix: one governed object family
/// bound to its state machine, status surface, status code, last-failure reason,
/// and recovery affordance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ObjectStateRow {
    /// Governed object family.
    pub object_family: M5LifecycleObjectFamily,
    /// Qualification class earned by this object.
    pub qualification: M5LifecycleQualificationClass,
    /// Owner role accountable for keeping this object governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Required fields the object must carry.
    pub required_fields: Vec<String>,
    /// Controlled states this object's explicit machine admits (must include
    /// [`M5LifecycleState::Ready`]).
    pub admitted_states: Vec<M5LifecycleState>,
    /// The single visible primary status surface.
    pub primary_status_surface: M5PrimaryStatusSurface,
    /// The one exportable status-code field name.
    pub status_code_export_field: String,
    /// The one last-failure-reason field name.
    pub last_failure_reason_field: String,
    /// Controlled last-failure reason classes this object reports.
    pub last_failure_reason_classes: Vec<M5LastFailureReasonClass>,
    /// The one named recovery affordance.
    pub recovery_affordance: M5RecoveryAffordance,
    /// Evidence requirement level.
    pub evidence_requirement: M5LifecycleEvidenceRequirement,
    /// Proof packet refs that keep this object current.
    pub required_proof_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this object.
    pub downgrade_triggers: Vec<M5LifecycleDowngradeTrigger>,
    /// Source contract refs consumed by this object.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this object's state.
    pub consumer_surfaces: Vec<M5LifecycleConsumerSurface>,
}

/// One row in the journey-checkpoint section: one protected journey and the
/// ordered milestone checkpoints it shows instead of an anonymous spinner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5JourneyCheckpointRow {
    /// Protected journey.
    pub journey: M5CriticalJourney,
    /// Object family this journey drives.
    pub object_family: M5LifecycleObjectFamily,
    /// Owner role accountable for the journey.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Ordered milestone checkpoints (at least two, unique, ending in a terminal).
    pub checkpoints: Vec<M5JourneyCheckpoint>,
    /// True when the journey shows named checkpoints rather than an anonymous
    /// spinner; a hard invariant.
    pub shows_named_checkpoints: bool,
    /// The success terminal state.
    pub success_state: M5LifecycleState,
    /// The named recovery affordance offered on a recoverable failure.
    pub recovery_affordance: M5RecoveryAffordance,
    /// Controlled last-failure reason classes this journey reports.
    pub last_failure_reason_classes: Vec<M5LastFailureReasonClass>,
    /// Source contract refs consumed by this journey.
    pub source_contract_refs: Vec<String>,
}

impl M5JourneyCheckpointRow {
    /// True when checkpoints are unique, at least two, and end in a terminal.
    fn checkpoints_well_formed(&self) -> bool {
        if self.checkpoints.len() < 2 {
            return false;
        }
        let unique: BTreeSet<M5JourneyCheckpoint> = self.checkpoints.iter().copied().collect();
        if unique.len() != self.checkpoints.len() {
            return false;
        }
        self.checkpoints
            .last()
            .is_some_and(|checkpoint| checkpoint.is_terminal())
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
///
/// Each field lists every canonical token for one controlled vocabulary, in
/// declaration order. The matrix validates each list against the typed `ALL`
/// arrays so the frozen vocabulary cannot silently drift.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LifecycleVocabularySet {
    /// Lifecycle-state tokens.
    pub lifecycle_states: Vec<String>,
    /// Primary-status-surface tokens.
    pub primary_status_surfaces: Vec<String>,
    /// Recovery-affordance tokens.
    pub recovery_affordances: Vec<String>,
    /// Last-failure-reason-class tokens.
    pub last_failure_reason_classes: Vec<String>,
    /// Journey-checkpoint tokens.
    pub journey_checkpoints: Vec<String>,
}

impl M5LifecycleVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            lifecycle_states: M5LifecycleState::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            primary_status_surfaces: M5PrimaryStatusSurface::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            recovery_affordances: M5RecoveryAffordance::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            last_failure_reason_classes: M5LastFailureReasonClass::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            journey_checkpoints: M5JourneyCheckpoint::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// State-binding review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LifecycleStateBindingReview {
    /// Every object family has an explicit state machine.
    pub every_object_has_explicit_state_machine: bool,
    /// Every object has one visible primary status surface.
    pub every_object_has_one_primary_status_surface: bool,
    /// Every object has one exportable status code.
    pub every_object_has_one_exportable_status_code: bool,
    /// Every object has one controlled last-failure reason.
    pub every_object_has_one_last_failure_reason: bool,
    /// Every object has one named recovery affordance.
    pub every_object_has_one_named_recovery_affordance: bool,
    /// Controlled terms keep one meaning across UI, CLI, docs, support, telemetry.
    pub controlled_terms_shared_across_ui_cli_docs_support_telemetry: bool,
    /// Protected journeys show named checkpoints, not anonymous spinners.
    pub protected_journeys_show_named_checkpoints: bool,
    /// A Ready state is defined for every object.
    pub ready_state_defined_for_every_object: bool,
    /// Downgrade narrows the claim rather than hiding the object.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Later M5 rows cannot invent private state vocabularies.
    pub later_rows_cannot_invent_private_state_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LifecycleConsumerProjection {
    /// Product UI consumes the shared lifecycle matrix.
    pub product_ui_consumes_lifecycle_matrix: bool,
    /// CLI shows the exportable status code.
    pub cli_shows_status_code: bool,
    /// Docs / help use the controlled state vocabulary.
    pub docs_help_use_controlled_state_vocabulary: bool,
    /// Diagnostics show the controlled last-failure reason.
    pub diagnostics_show_last_failure_reason: bool,
    /// Support export shows the shared object-state model.
    pub support_export_shows_object_state_model: bool,
    /// Telemetry uses the controlled state codes.
    pub telemetry_uses_controlled_state_codes: bool,
    /// Claim tooling reads a single source.
    pub claim_tooling_reads_single_source: bool,
    /// Release notes use the controlled vocabulary.
    pub release_notes_use_controlled_vocabulary: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LifecycleProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the object.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the lifecycle lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LifecycleReleasePosture {
    /// Ref of the supporting release packet for the lane.
    pub release_packet_ref: String,
    /// Ref of the supporting recovery-drill packet for the lane.
    pub recovery_drill_packet_ref: String,
    /// True when support/export parity is required for every object.
    pub support_export_parity_required: bool,
    /// True when telemetry parity is required for every object.
    pub telemetry_parity_required: bool,
}

/// Constructor input for [`M5LifecycleMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5LifecycleMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Object-state rows.
    pub object_state_rows: Vec<M5ObjectStateRow>,
    /// Journey-checkpoint rows.
    pub journey_checkpoint_rows: Vec<M5JourneyCheckpointRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5LifecycleVocabularySet,
    /// State-binding review block.
    pub state_binding_review: M5LifecycleStateBindingReview,
    /// Consumer projection block.
    pub consumer_projection: M5LifecycleConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5LifecycleProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5LifecycleReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 lifecycle-state and journey-checkpoint matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LifecycleMatrixPacket {
    /// Record kind; must equal [`M5_LIFECYCLE_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_LIFECYCLE_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Object-state rows.
    pub object_state_rows: Vec<M5ObjectStateRow>,
    /// Journey-checkpoint rows.
    pub journey_checkpoint_rows: Vec<M5JourneyCheckpointRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5LifecycleVocabularySet,
    /// State-binding review block.
    pub state_binding_review: M5LifecycleStateBindingReview,
    /// Consumer projection block.
    pub consumer_projection: M5LifecycleConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5LifecycleProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5LifecycleReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5LifecycleMatrixPacket {
    /// Builds an M5 lifecycle matrix packet from stable-lane input.
    pub fn new(input: M5LifecycleMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_LIFECYCLE_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_LIFECYCLE_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            object_state_rows: input.object_state_rows,
            journey_checkpoint_rows: input.journey_checkpoint_rows,
            vocabulary_set: input.vocabulary_set,
            state_binding_review: input.state_binding_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 lifecycle matrix invariants.
    pub fn validate(&self) -> Vec<M5LifecycleMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_LIFECYCLE_MATRIX_RECORD_KIND {
            violations.push(M5LifecycleMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_LIFECYCLE_MATRIX_SCHEMA_VERSION {
            violations.push(M5LifecycleMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5LifecycleMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_object_rows(self, &mut violations);
        validate_journey_rows(self, &mut violations);
        validate_state_binding_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 lifecycle matrix packet serializes"),
        ) {
            violations.push(M5LifecycleMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 lifecycle matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed object,
    /// naming its qualification, owner, status surface, status code, last-failure
    /// reason, recovery affordance, and consumer surfaces.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "object_family,qualification,owner,primary_status_surface,status_code_export_field,last_failure_reason_field,recovery_affordance,admitted_states,consumer_surfaces\n",
        );
        for row in &self.object_state_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.object_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.primary_status_surface.as_str(),
                csv_field(&row.status_code_export_field),
                csv_field(&row.last_failure_reason_field),
                row.recovery_affordance.as_str(),
                join_tokens(&row.admitted_states, |s| s.as_str()),
                join_tokens(&row.consumer_surfaces, |s| s.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown lifecycle report for support, docs, or review
    /// handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_objects = self
            .object_state_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Lifecycle-State and Critical-Journey Checkpoint Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Object families: {} ({} stable)\n",
            self.object_state_rows.len(),
            stable_objects
        ));
        out.push_str(&format!(
            "- Protected journeys: {}\n",
            self.journey_checkpoint_rows.len()
        ));
        out.push_str(&format!(
            "- Controlled states: {}\n",
            self.vocabulary_set.lifecycle_states.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Object families\n\n");
        for row in &self.object_state_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.object_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Primary status surface: `{}`\n",
                row.primary_status_surface.as_str()
            ));
            out.push_str(&format!(
                "  - Status code field: `{}`\n",
                row.status_code_export_field
            ));
            out.push_str(&format!(
                "  - Last-failure reason field: `{}`\n",
                row.last_failure_reason_field
            ));
            out.push_str(&format!(
                "  - Recovery affordance: `{}`\n",
                row.recovery_affordance.as_str()
            ));
            out.push_str(&format!(
                "  - Admitted states: {}\n",
                row.admitted_states
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out.push_str("\n## Protected journeys\n\n");
        for row in &self.journey_checkpoint_rows {
            out.push_str(&format!(
                "- **{}** ({})\n",
                row.journey.as_str(),
                row.object_family.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Checkpoints: {}\n",
                row.checkpoints
                    .iter()
                    .map(|c| c.as_str())
                    .collect::<Vec<_>>()
                    .join(" → ")
            ));
            out.push_str(&format!(
                "  - Recovery affordance: `{}`\n",
                row.recovery_affordance.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 lifecycle matrix export.
#[derive(Debug)]
pub enum M5LifecycleMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5LifecycleMatrixViolation>),
}

impl fmt::Display for M5LifecycleMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 lifecycle matrix export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 lifecycle matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5LifecycleMatrixArtifactError {}

/// Validation failures emitted by [`M5LifecycleMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5LifecycleMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed object family is missing from the matrix.
    RequiredObjectMissing,
    /// An object row is incomplete.
    ObjectRowIncomplete,
    /// An object's state machine omits the Ready state.
    ObjectMissingReadyState,
    /// An object has no admitted states.
    ObjectHasNoStates,
    /// An object omits its exportable status-code field.
    StatusCodeFieldMissing,
    /// An object omits its last-failure-reason field or classes.
    LastFailureReasonMissing,
    /// An object claiming Stable is missing required proof packet refs.
    StableObjectMissingProof,
    /// An object has no downgrade triggers.
    DowngradeTriggersMissing,
    /// An object has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A required protected journey is missing from the matrix.
    RequiredJourneyMissing,
    /// A journey row is incomplete.
    JourneyRowIncomplete,
    /// A journey shows anonymous checkpoints or a malformed checkpoint sequence.
    AnonymousOrMalformedCheckpoints,
    /// State-binding review does not satisfy required invariants.
    StateBindingReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5LifecycleMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredObjectMissing => "required_object_missing",
            Self::ObjectRowIncomplete => "object_row_incomplete",
            Self::ObjectMissingReadyState => "object_missing_ready_state",
            Self::ObjectHasNoStates => "object_has_no_states",
            Self::StatusCodeFieldMissing => "status_code_field_missing",
            Self::LastFailureReasonMissing => "last_failure_reason_missing",
            Self::StableObjectMissingProof => "stable_object_missing_proof",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::RequiredJourneyMissing => "required_journey_missing",
            Self::JourneyRowIncomplete => "journey_row_incomplete",
            Self::AnonymousOrMalformedCheckpoints => "anonymous_or_malformed_checkpoints",
            Self::StateBindingReviewIncomplete => "state_binding_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 lifecycle matrix export.
pub fn current_stable_m5_lifecycle_matrix_export(
) -> Result<M5LifecycleMatrixPacket, M5LifecycleMatrixArtifactError> {
    let packet: M5LifecycleMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-lifecycle-proof/support_export.json"
    )))
    .map_err(M5LifecycleMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5LifecycleMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5LifecycleMatrixPacket,
    violations: &mut Vec<M5LifecycleMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_LIFECYCLE_OBJECT_STATE_SCHEMA_REF,
        M5_LIFECYCLE_JOURNEY_CHECKPOINT_SCHEMA_REF,
        M5_LIFECYCLE_MATRIX_DOC_REF,
        M5_LIFECYCLE_STATE_OBJECT_INVENTORY_REF,
        M5_LIFECYCLE_STATE_CLASS_RECOVERY_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5LifecycleMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5LifecycleMatrixPacket,
    violations: &mut Vec<M5LifecycleMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5LifecycleMatrixViolation::VocabularySetDrift);
    }
}

fn validate_object_rows(
    packet: &M5LifecycleMatrixPacket,
    violations: &mut Vec<M5LifecycleMatrixViolation>,
) {
    let present: BTreeSet<M5LifecycleObjectFamily> = packet
        .object_state_rows
        .iter()
        .map(|row| row.object_family)
        .collect();
    for required in M5LifecycleObjectFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5LifecycleMatrixViolation::RequiredObjectMissing);
            return;
        }
    }

    for row in &packet.object_state_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.required_fields.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5LifecycleMatrixViolation::ObjectRowIncomplete);
        }

        if row.admitted_states.is_empty() {
            violations.push(M5LifecycleMatrixViolation::ObjectHasNoStates);
        } else if !row.admitted_states.contains(&M5LifecycleState::Ready) {
            violations.push(M5LifecycleMatrixViolation::ObjectMissingReadyState);
        }

        if row.status_code_export_field.trim().is_empty() {
            violations.push(M5LifecycleMatrixViolation::StatusCodeFieldMissing);
        }

        if row.last_failure_reason_field.trim().is_empty()
            || row.last_failure_reason_classes.is_empty()
        {
            violations.push(M5LifecycleMatrixViolation::LastFailureReasonMissing);
        }

        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5LifecycleMatrixViolation::StableObjectMissingProof);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5LifecycleMatrixViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5LifecycleMatrixViolation::ConsumerSurfacesMissing);
        }
    }
}

fn validate_journey_rows(
    packet: &M5LifecycleMatrixPacket,
    violations: &mut Vec<M5LifecycleMatrixViolation>,
) {
    let present: BTreeSet<M5CriticalJourney> = packet
        .journey_checkpoint_rows
        .iter()
        .map(|row| row.journey)
        .collect();
    for required in M5CriticalJourney::ALL {
        if !present.contains(&required) {
            violations.push(M5LifecycleMatrixViolation::RequiredJourneyMissing);
            return;
        }
    }

    for row in &packet.journey_checkpoint_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.last_failure_reason_classes.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5LifecycleMatrixViolation::JourneyRowIncomplete);
        }
        if !row.shows_named_checkpoints || !row.checkpoints_well_formed() {
            violations.push(M5LifecycleMatrixViolation::AnonymousOrMalformedCheckpoints);
        }
    }
}

fn validate_state_binding_review(
    packet: &M5LifecycleMatrixPacket,
    violations: &mut Vec<M5LifecycleMatrixViolation>,
) {
    let review = &packet.state_binding_review;
    for ok in [
        review.every_object_has_explicit_state_machine,
        review.every_object_has_one_primary_status_surface,
        review.every_object_has_one_exportable_status_code,
        review.every_object_has_one_last_failure_reason,
        review.every_object_has_one_named_recovery_affordance,
        review.controlled_terms_shared_across_ui_cli_docs_support_telemetry,
        review.protected_journeys_show_named_checkpoints,
        review.ready_state_defined_for_every_object,
        review.downgrade_narrows_instead_of_hides,
        review.later_rows_cannot_invent_private_state_vocabulary,
    ] {
        if !ok {
            violations.push(M5LifecycleMatrixViolation::StateBindingReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5LifecycleMatrixPacket,
    violations: &mut Vec<M5LifecycleMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.product_ui_consumes_lifecycle_matrix,
        projection.cli_shows_status_code,
        projection.docs_help_use_controlled_state_vocabulary,
        projection.diagnostics_show_last_failure_reason,
        projection.support_export_shows_object_state_model,
        projection.telemetry_uses_controlled_state_codes,
        projection.claim_tooling_reads_single_source,
        projection.release_notes_use_controlled_vocabulary,
    ] {
        if !ok {
            violations.push(M5LifecycleMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5LifecycleMatrixPacket,
    violations: &mut Vec<M5LifecycleMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5LifecycleMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5LifecycleMatrixPacket,
    violations: &mut Vec<M5LifecycleMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.recovery_drill_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.telemetry_parity_required
    {
        violations.push(M5LifecycleMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items
        .iter()
        .map(|item| to_token(item))
        .collect::<Vec<_>>()
        .join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
