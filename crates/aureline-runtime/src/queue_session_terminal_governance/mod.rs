//! Queue, restore-continuity, and terminal-boundary governance packet.
//!
//! This module freezes one metadata-only contract for background-heavy,
//! restorable, and terminal-adjacent runtime surfaces. It does not implement
//! scheduling, restore, or PTY behavior directly; it publishes the shared
//! vocabulary that notebook, data, pipeline, preview, profiler, docs-recall,
//! sync/offboarding, companion, incident, and infrastructure surfaces project
//! into activity rows, restore summaries, terminal chrome, support export,
//! release evidence, and docs/help.
//!
//! A stable workload row is only valid when it names:
//!
//! - one canonical queue identity bundle: lane, collapse key, budget domain,
//!   checkpoint policy, retry posture, cancellation posture, and an opaque
//!   queue identity ref;
//! - one restore continuity bundle: restore fidelity, no-hidden-rerun class,
//!   and an opaque restore anchor ref;
//! - one explicit terminal-boundary bundle when the workload crosses an
//!   execution boundary: boundary class, clipboard posture, and an opaque
//!   boundary ref;
//! - one downgrade rule so stale queue metadata, restore fidelity, or boundary
//!   proof narrows the claim instead of inheriting adjacent stable truth.
//!
//! The packet is intentionally metadata-only. Raw commands, transcript bodies,
//! provider payloads, credentials, and ambient authority never cross this
//! boundary.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::queue_governor_and_admission_control::{
    CollapsePolicy as QueueCollapsePolicy, InitiatingSource as QueueInitiatingSource,
    QueueJobScope, StalenessPolicy as QueueStalenessPolicy,
};
use crate::resource_governor::{
    seeded_resource_governor_snapshot, CheckpointMetadata, QueueLane as GovernorQueueLane,
};

/// Stable record-kind tag for [`QueueSessionTerminalGovernancePacket`].
pub const QUEUE_SESSION_TERMINAL_GOVERNANCE_RECORD_KIND: &str =
    "queue_session_terminal_governance_packet";

/// Stable record-kind tag for [`QueueSessionTerminalGovernanceSupportExport`].
pub const QUEUE_SESSION_TERMINAL_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "queue_session_terminal_governance_support_export";

/// Stable record-kind tag for M5 activity job rows.
pub const QUEUE_SESSION_TERMINAL_ACTIVITY_JOB_ROW_RECORD_KIND: &str =
    "queue_session_terminal_activity_job_row";

/// Stable record-kind tag for M5 scheduler lane rows.
pub const QUEUE_SESSION_TERMINAL_SCHEDULER_LANE_ROW_RECORD_KIND: &str =
    "queue_session_terminal_scheduler_lane_row";

/// Integer schema version for the governance packet.
pub const QUEUE_SESSION_TERMINAL_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative schema reference for the governance packet.
pub const QUEUE_SESSION_TERMINAL_GOVERNANCE_SCHEMA_REF: &str =
    "schemas/runtime/queue-session-terminal-governance.schema.json";

/// Repo-relative contract document reference.
pub const QUEUE_SESSION_TERMINAL_GOVERNANCE_DOC_REF: &str =
    "docs/runtime/queue-session-terminal-governance.md";

/// Repo-relative reviewer matrix artifact reference.
pub const QUEUE_SESSION_TERMINAL_GOVERNANCE_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/queue-session-terminal-governance.md";

/// Repo-relative fixture corpus directory.
pub const QUEUE_SESSION_TERMINAL_GOVERNANCE_FIXTURE_DIR: &str =
    "fixtures/runtime/queue_session_terminal_governance";

/// Background-queue contract reference reused by queue identity rows.
pub const BACKGROUND_QUEUE_CONTRACT_DOC_REF: &str = "docs/runtime/background_queue_contract.md";

/// Context-cache and terminal-restore contract reused by restore rows.
pub const CONTEXT_CACHE_TERMINAL_RESTORE_CONTRACT_DOC_REF: &str =
    "docs/runtime/context_cache_and_terminal_restore_contract.md";

/// Foreground budget domain for explicit user-run work.
pub const FOREGROUND_TASK_BUDGET_DOMAIN_REF: &str = "foreground_task_budget";

/// Interactive knowledge-refresh budget domain.
pub const KNOWLEDGE_REFRESH_BUDGET_DOMAIN_REF: &str = "knowledge_refresh_budget";

/// Maintenance budget domain for deferred background work.
pub const MAINTENANCE_BUDGET_DOMAIN_REF: &str = "maintenance_budget";

/// Provider-overlay budget domain for remote or service-backed work.
pub const PROVIDER_OVERLAY_BUDGET_DOMAIN_REF: &str = "provider_overlay_budget";

/// Replication budget domain for uploads, sync, and offboarding.
pub const REPLICATION_BUDGET_DOMAIN_REF: &str = "replication_budget";

/// Reserved hot-path budget that queue rows may protect but never consume.
pub const HOT_PATH_INTERACTIVE_BUDGET_DOMAIN_REF: &str = "hot_path_interactive_budget";

/// Runtime workload class governed by this contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernedWorkloadClass {
    /// Notebook execution surfaces and restored kernel-linked sessions.
    NotebookSession,
    /// Data and query-console execution surfaces.
    DataQueryConsole,
    /// Pipeline execution and run-review surfaces.
    PipelineRun,
    /// Preview routes and attached preview runtimes.
    PreviewRoute,
    /// Profiler capture and replay surfaces.
    ProfilerCapture,
    /// Docs browser recall and captured recall surfaces.
    DocsRecall,
    /// Sync, export, and offboarding continuity flows.
    SyncOffboardingFlow,
    /// Companion handoff surfaces.
    CompanionHandoff,
    /// Incident workspace and shared recovery surfaces.
    IncidentWorkspace,
    /// Infrastructure and terminal-heavy runbook sessions.
    InfrastructureSession,
}

impl GovernedWorkloadClass {
    /// Every workload the checked-in packet must cover.
    pub const REQUIRED: [Self; 10] = [
        Self::NotebookSession,
        Self::DataQueryConsole,
        Self::PipelineRun,
        Self::PreviewRoute,
        Self::ProfilerCapture,
        Self::DocsRecall,
        Self::SyncOffboardingFlow,
        Self::CompanionHandoff,
        Self::IncidentWorkspace,
        Self::InfrastructureSession,
    ];

    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookSession => "notebook_session",
            Self::DataQueryConsole => "data_query_console",
            Self::PipelineRun => "pipeline_run",
            Self::PreviewRoute => "preview_route",
            Self::ProfilerCapture => "profiler_capture",
            Self::DocsRecall => "docs_recall",
            Self::SyncOffboardingFlow => "sync_offboarding_flow",
            Self::CompanionHandoff => "companion_handoff",
            Self::IncidentWorkspace => "incident_workspace",
            Self::InfrastructureSession => "infrastructure_session",
        }
    }

    /// Returns true when the workload must carry a terminal-boundary row.
    pub const fn requires_terminal_boundary(self) -> bool {
        matches!(
            self,
            Self::NotebookSession
                | Self::DataQueryConsole
                | Self::PipelineRun
                | Self::PreviewRoute
                | Self::CompanionHandoff
                | Self::IncidentWorkspace
                | Self::InfrastructureSession
        )
    }
}

/// Queue lane vocabulary admitted by the governance packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueueLaneClass {
    /// User-visible foreground requests.
    Foreground,
    /// Coalescible interactive background work.
    InteractiveBackground,
    /// Deferred maintenance work.
    Maintenance,
    /// Remote or provider overlay work with separate retry budgets.
    ProviderOverlay,
    /// Upload, replication, and export work.
    UploadReplication,
    /// Row does not bind a queue lane.
    NotApplicable,
}

impl QueueLaneClass {
    /// Every queue lane the packet must expose.
    pub const REQUIRED: [Self; 5] = [
        Self::Foreground,
        Self::InteractiveBackground,
        Self::Maintenance,
        Self::ProviderOverlay,
        Self::UploadReplication,
    ];

    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Foreground => "foreground",
            Self::InteractiveBackground => "interactive_background",
            Self::Maintenance => "maintenance",
            Self::ProviderOverlay => "provider_overlay",
            Self::UploadReplication => "upload_replication",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Row class frozen by the governance packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceRowClass {
    /// Headline continuity-quality row for one workload.
    ContinuityQuality,
    /// Queue identity and fairness row for one workload.
    QueueIdentityAdmission,
    /// Restore continuity row for one workload.
    RestoreContinuityAdmission,
    /// Terminal-boundary and clipboard row for one workload.
    TerminalBoundaryAdmission,
    /// Automatic narrowing or blocking rule for one workload.
    DowngradeRule,
}

impl GovernanceRowClass {
    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContinuityQuality => "continuity_quality",
            Self::QueueIdentityAdmission => "queue_identity_admission",
            Self::RestoreContinuityAdmission => "restore_continuity_admission",
            Self::TerminalBoundaryAdmission => "terminal_boundary_admission",
            Self::DowngradeRule => "downgrade_rule",
        }
    }

    /// Returns true when the row class binds queue metadata.
    pub const fn requires_queue_fields(self) -> bool {
        matches!(self, Self::QueueIdentityAdmission)
    }

    /// Returns true when the row class binds restore metadata.
    pub const fn requires_restore_fields(self) -> bool {
        matches!(self, Self::RestoreContinuityAdmission)
    }

    /// Returns true when the row class binds terminal-boundary metadata.
    pub const fn requires_terminal_fields(self) -> bool {
        matches!(self, Self::TerminalBoundaryAdmission)
    }
}

/// Support posture applied to one row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Row may be projected on stable surfaces.
    Stable,
    /// Row is intentionally narrowed below stable.
    StableBelow,
    /// Row is beta only.
    BetaOnly,
    /// Row is preview only.
    PreviewOnly,
    /// Row names an unsupported gap.
    Unsupported,
    /// Row has no support posture bound.
    SupportUnbound,
}

impl SupportClass {
    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::StableBelow => "stable_below",
            Self::BetaOnly => "beta_only",
            Self::PreviewOnly => "preview_only",
            Self::Unsupported => "unsupported",
            Self::SupportUnbound => "support_unbound",
        }
    }

    /// Returns true when the support class is fully bound.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::SupportUnbound)
    }

    /// Returns true when the row must cite a disclosure reference.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Stable)
    }
}

/// Collapse-key vocabulary for queued work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollapseKeyClass {
    /// Collapse by workspace, slice, and target scope.
    WorkspaceSliceTarget,
    /// Collapse by session or surface identity plus target scope.
    SessionSurfaceTarget,
    /// Collapse by provider route and target scope.
    ProviderRouteTarget,
    /// Collapse by export destination and artifact class.
    ArtifactDestinationTarget,
    /// Collapse by handoff subject and destination class.
    HandoffSubject,
    /// Row does not bind a collapse key.
    NotApplicable,
}

impl CollapseKeyClass {
    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceSliceTarget => "workspace_slice_target",
            Self::SessionSurfaceTarget => "session_surface_target",
            Self::ProviderRouteTarget => "provider_route_target",
            Self::ArtifactDestinationTarget => "artifact_destination_target",
            Self::HandoffSubject => "handoff_subject",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Concrete background job kinds covered by the M5 governance packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernedJobKind {
    /// Notebook cell or notebook-run execution.
    NotebookCellExecution,
    /// Docs-pack refresh or catalog refresh.
    DocsPackRefresh,
    /// Retrieval or index refresh for docs recall.
    DocsRetrievalIndexRefresh,
    /// Request collection or API-console execution.
    DataRequestCollectionRun,
    /// Profiler capture or capture-finalization work.
    ProfilerCapture,
    /// Pipeline log refresh or pull work.
    PipelineLogPull,
    /// Pipeline artifact refresh or pull work.
    PipelineArtifactPull,
    /// Preview dev-server lifecycle work.
    PreviewDevServer,
    /// Preview route refresh or binding work.
    PreviewRouteRefresh,
    /// Sync replication job.
    SyncProfileReplication,
    /// Offboarding export or package assembly job.
    SyncOffboardingExport,
    /// Companion handoff package publication.
    CompanionHandoffPackage,
    /// Incident workspace recovery or evidence refresh.
    IncidentRecoveryWorkspaceRefresh,
    /// Infrastructure overlay probe or runbook session binding.
    InfrastructureOverlayProbe,
}

impl GovernedJobKind {
    /// Every concrete job kind the packet must cover.
    pub const REQUIRED: [Self; 14] = [
        Self::NotebookCellExecution,
        Self::DocsPackRefresh,
        Self::DocsRetrievalIndexRefresh,
        Self::DataRequestCollectionRun,
        Self::ProfilerCapture,
        Self::PipelineLogPull,
        Self::PipelineArtifactPull,
        Self::PreviewDevServer,
        Self::PreviewRouteRefresh,
        Self::SyncProfileReplication,
        Self::SyncOffboardingExport,
        Self::CompanionHandoffPackage,
        Self::IncidentRecoveryWorkspaceRefresh,
        Self::InfrastructureOverlayProbe,
    ];

    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookCellExecution => "notebook.cell_execution",
            Self::DocsPackRefresh => "docs.pack_refresh",
            Self::DocsRetrievalIndexRefresh => "docs.retrieval_index_refresh",
            Self::DataRequestCollectionRun => "data.request_collection_run",
            Self::ProfilerCapture => "profiler.capture",
            Self::PipelineLogPull => "pipeline.log_pull",
            Self::PipelineArtifactPull => "pipeline.artifact_pull",
            Self::PreviewDevServer => "preview.dev_server",
            Self::PreviewRouteRefresh => "preview.route_refresh",
            Self::SyncProfileReplication => "sync.profile_replication",
            Self::SyncOffboardingExport => "sync.offboarding_export",
            Self::CompanionHandoffPackage => "companion.handoff_package",
            Self::IncidentRecoveryWorkspaceRefresh => "incident.recovery_workspace_refresh",
            Self::InfrastructureOverlayProbe => "infrastructure.overlay_probe",
        }
    }
}

/// One concrete, export-safe queue identity bound to a queue row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernedJobIdentity {
    /// Stable concrete job kind.
    pub job_kind: GovernedJobKind,
    /// Opaque job identity ref safe for diagnostics and support export.
    pub job_identity_ref: String,
    /// Opaque workspace id owning the job.
    pub workspace_id_ref: String,
    /// Optional slice or workset identity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slice_id_ref: Option<String>,
    /// Root or target scope bound to the job.
    pub scope: QueueJobScope,
    /// Stable initiating source.
    pub initiating_source: QueueInitiatingSource,
    /// Structured duplicate or supersede key.
    pub collapse_key: String,
    /// Concrete duplicate or supersede behavior.
    pub collapse_policy: QueueCollapsePolicy,
    /// Concrete staleness behavior when resumed or dequeued.
    pub staleness_policy: QueueStalenessPolicy,
    /// Exact budget domains this job consumes.
    #[serde(default)]
    pub budget_domain_refs: Vec<String>,
    /// Workspace revision ref used to self-invalidate stale work.
    pub workspace_revision_ref: String,
    /// Optional manifest or slice hash used to self-invalidate stale work.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_hash_ref: Option<String>,
    /// Optional execution-context hash used to self-invalidate stale work.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_hash_ref: Option<String>,
    /// Optional policy epoch used to self-invalidate stale work.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_epoch_ref: Option<String>,
}

impl GovernedJobIdentity {
    fn is_valid(&self) -> bool {
        !self.job_identity_ref.trim().is_empty()
            && !self.workspace_id_ref.trim().is_empty()
            && !self.scope.scope_class.trim().is_empty()
            && !self.scope.scope_ref.trim().is_empty()
            && !self.collapse_key.trim().is_empty()
            && !self.workspace_revision_ref.trim().is_empty()
            && !self.budget_domain_refs.is_empty()
            && self
                .budget_domain_refs
                .iter()
                .all(|budget_domain| is_known_budget_domain_ref(budget_domain))
            && !self
                .budget_domain_refs
                .iter()
                .any(|budget_domain| budget_domain == HOT_PATH_INTERACTIVE_BUDGET_DOMAIN_REF)
    }
}

/// Budget-domain vocabulary for queued work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BudgetDomainClass {
    /// CPU, memory, and disposable disk cache budget.
    CpuMemoryDisk,
    /// Network, remote provider, or API quota budget.
    NetworkProviderQuota,
    /// Battery and thermal budget.
    BatteryThermal,
    /// Reserved interactive budget that background work may not borrow.
    ProtectedInteractiveReserve,
    /// Durable evidence or storage budget.
    DurableStorage,
    /// Optional-service or auxiliary-provider budget.
    OptionalServiceQuota,
    /// Row does not bind a budget domain.
    NotApplicable,
}

impl BudgetDomainClass {
    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CpuMemoryDisk => "cpu_memory_disk",
            Self::NetworkProviderQuota => "network_provider_quota",
            Self::BatteryThermal => "battery_thermal",
            Self::ProtectedInteractiveReserve => "protected_interactive_reserve",
            Self::DurableStorage => "durable_storage",
            Self::OptionalServiceQuota => "optional_service_quota",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Checkpoint-policy vocabulary for queued work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointPolicyClass {
    /// No checkpoint exists beyond an explicit user-visible rerun.
    NoneDeclared,
    /// Resume from item boundaries.
    ItemBoundary,
    /// Resume from time boundaries.
    TimeBoundary,
    /// Resume from explicit phase boundaries.
    ExplicitPhaseBoundary,
    /// Resume from chunk boundaries.
    ResumableChunkBoundary,
    /// Row does not bind a checkpoint policy.
    NotApplicable,
}

impl CheckpointPolicyClass {
    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDeclared => "none_declared",
            Self::ItemBoundary => "item_boundary",
            Self::TimeBoundary => "time_boundary",
            Self::ExplicitPhaseBoundary => "explicit_phase_boundary",
            Self::ResumableChunkBoundary => "resumable_chunk_boundary",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Retry vocabulary for queued work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetryClass {
    /// No automatic retry occurs.
    NoneDeclared,
    /// Local retry budget applies.
    LocalRetryBudget,
    /// Separate provider retry budget applies.
    ProviderRetryBudget,
    /// Reauthorize or reconnect before retry.
    ReconnectAfterReauth,
    /// Manual review is required before requeue.
    ManualRequeueOnly,
    /// Row does not bind a retry posture.
    NotApplicable,
}

impl RetryClass {
    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDeclared => "none_declared",
            Self::LocalRetryBudget => "local_retry_budget",
            Self::ProviderRetryBudget => "provider_retry_budget",
            Self::ReconnectAfterReauth => "reconnect_after_reauth",
            Self::ManualRequeueOnly => "manual_requeue_only",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Cancellation vocabulary for queued work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CancellationClass {
    /// Immediate cancellation is safe.
    ImmediateAbortSafe,
    /// A checkpoint is written before cancellation completes.
    CheckpointThenCancel,
    /// Cancellation completes after the active phase.
    CancelAfterPhase,
    /// Cleanup runs before cancellation completes.
    CleanupThenCancel,
    /// Rollback or compensation runs before cancellation completes.
    RollbackThenCancel,
    /// Row does not bind a cancellation posture.
    NotApplicable,
}

impl CancellationClass {
    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImmediateAbortSafe => "immediate_abort_safe",
            Self::CheckpointThenCancel => "checkpoint_then_cancel",
            Self::CancelAfterPhase => "cancel_after_phase",
            Self::CleanupThenCancel => "cleanup_then_cancel",
            Self::RollbackThenCancel => "rollback_then_cancel",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Restore-fidelity vocabulary for session continuity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreFidelityClass {
    /// Exact restore with live context preserved.
    ExactRestore,
    /// Compatible restore with declared adjustments.
    CompatibleRestore,
    /// Layout and surrounding context restored.
    LayoutOnly,
    /// Transcript or durable history restored without live execution.
    TranscriptOnly,
    /// Evidence, links, and provenance survived.
    EvidenceOnly,
    /// Placeholder card preserved the slot and recovery path.
    PlaceholderOnly,
    /// Row does not bind restore fidelity.
    NotApplicable,
}

impl RestoreFidelityClass {
    /// Every restore fidelity class the packet must expose.
    pub const REQUIRED: [Self; 6] = [
        Self::ExactRestore,
        Self::CompatibleRestore,
        Self::LayoutOnly,
        Self::TranscriptOnly,
        Self::EvidenceOnly,
        Self::PlaceholderOnly,
    ];

    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactRestore => "exact_restore",
            Self::CompatibleRestore => "compatible_restore",
            Self::LayoutOnly => "layout_only",
            Self::TranscriptOnly => "transcript_only",
            Self::EvidenceOnly => "evidence_only",
            Self::PlaceholderOnly => "placeholder_only",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// No-hidden-rerun vocabulary for restore continuity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoHiddenRerunClass {
    /// Live continuity is preserved without widening authority.
    LiveContinuityPreserved,
    /// Metadata is rebound but execution stays stopped.
    MetadataOnlyResume,
    /// Transcript survives, but no command reruns.
    TranscriptPreservedNoRerun,
    /// Reconnect review is required before execution resumes.
    ReconnectReviewRequired,
    /// Reauthorization is required before execution resumes.
    ReauthorizeBeforeResume,
    /// Explicit rerun is the only path back to execution.
    ExplicitRerunOnly,
    /// Manual review blocks any resume path.
    BlockedUntilManualReview,
    /// Row does not bind a no-hidden-rerun posture.
    NotApplicable,
}

impl NoHiddenRerunClass {
    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveContinuityPreserved => "live_continuity_preserved",
            Self::MetadataOnlyResume => "metadata_only_resume",
            Self::TranscriptPreservedNoRerun => "transcript_preserved_no_rerun",
            Self::ReconnectReviewRequired => "reconnect_review_required",
            Self::ReauthorizeBeforeResume => "reauthorize_before_resume",
            Self::ExplicitRerunOnly => "explicit_rerun_only",
            Self::BlockedUntilManualReview => "blocked_until_manual_review",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Terminal-boundary vocabulary preserved by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalBoundaryClass {
    /// Local machine execution boundary.
    Local,
    /// Remote host or helper boundary.
    Remote,
    /// Container or devcontainer boundary.
    Container,
    /// Managed workspace or hosted runtime boundary.
    Managed,
    /// Shared-control boundary with distinct viewer and writer roles.
    SharedControl,
    /// Policy-blocked boundary where terminal authority is withheld.
    PolicyBlocked,
    /// Row does not bind a terminal boundary.
    NotApplicable,
}

impl TerminalBoundaryClass {
    /// Every terminal-boundary class the packet must expose.
    pub const REQUIRED: [Self; 6] = [
        Self::Local,
        Self::Remote,
        Self::Container,
        Self::Managed,
        Self::SharedControl,
        Self::PolicyBlocked,
    ];

    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Remote => "remote",
            Self::Container => "container",
            Self::Managed => "managed",
            Self::SharedControl => "shared_control",
            Self::PolicyBlocked => "policy_blocked",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Clipboard and paste posture vocabulary for terminal-boundary rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClipboardPostureClass {
    /// Local clipboard stays local and direct.
    LocalDirect,
    /// Bracketed paste and high-risk review are active.
    BracketedPasteReview,
    /// Remote clipboard bridging requires review.
    RemoteBridgeReview,
    /// Shared control requires an explicit input grant.
    SharedControlGrantRequired,
    /// Only metadata or hashes may be exported by default.
    MetadataOnlyExport,
    /// Policy denied the action and surfaces a safe alternative.
    PolicyDeniedSafeAlternative,
    /// Row does not bind clipboard posture.
    NotApplicable,
}

impl ClipboardPostureClass {
    /// Every clipboard posture class the packet must expose.
    pub const REQUIRED: [Self; 6] = [
        Self::LocalDirect,
        Self::BracketedPasteReview,
        Self::RemoteBridgeReview,
        Self::SharedControlGrantRequired,
        Self::MetadataOnlyExport,
        Self::PolicyDeniedSafeAlternative,
    ];

    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDirect => "local_direct",
            Self::BracketedPasteReview => "bracketed_paste_review",
            Self::RemoteBridgeReview => "remote_bridge_review",
            Self::SharedControlGrantRequired => "shared_control_grant_required",
            Self::MetadataOnlyExport => "metadata_only_export",
            Self::PolicyDeniedSafeAlternative => "policy_denied_safe_alternative",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Evidence class backing one governance row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceClass {
    /// Automated functional or contract tests back the row.
    AutomatedFunctionalEvidence,
    /// Design review or UX validation backs the row.
    DesignQaEvidence,
    /// Failure and recovery drills back the row.
    FailureRecoveryDrillEvidence,
    /// Security or privacy review backs the row.
    SecurityPrivacyReviewEvidence,
    /// Release evidence review backs the row.
    ReleaseEvidenceReview,
    /// Schema and fixture corpus coverage back the row.
    SchemaFixtureEvidence,
    /// Docs disclosure alone backs a narrowed row.
    DocsDisclosureEvidence,
    /// Row has no bound evidence class.
    EvidenceUnbound,
}

impl EvidenceClass {
    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AutomatedFunctionalEvidence => "automated_functional_evidence",
            Self::DesignQaEvidence => "design_qa_evidence",
            Self::FailureRecoveryDrillEvidence => "failure_recovery_drill_evidence",
            Self::SecurityPrivacyReviewEvidence => "security_privacy_review_evidence",
            Self::ReleaseEvidenceReview => "release_evidence_review",
            Self::SchemaFixtureEvidence => "schema_fixture_evidence",
            Self::DocsDisclosureEvidence => "docs_disclosure_evidence",
            Self::EvidenceUnbound => "evidence_unbound",
        }
    }

    /// Returns true when the evidence class is fully bound.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::EvidenceUnbound)
    }
}

/// Known-limit vocabulary attached to a governance row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond the canonical contract.
    NoneDeclared,
    /// Queue metadata is a subset-only claim.
    QueueMetadataSubsetOnly,
    /// Restore fidelity remains placeholder or evidence biased.
    RestorePlaceholderOnly,
    /// Terminal boundary is only partially proven.
    TerminalBoundarySubsetOnly,
    /// Policy review is still pending for the row.
    PolicyReviewPending,
    /// Imported evidence exists without live continuity proof.
    ImportedEvidenceOnly,
    /// Row has no known-limit class bound.
    LimitUnbound,
}

impl KnownLimitClass {
    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDeclared => "none_declared",
            Self::QueueMetadataSubsetOnly => "queue_metadata_subset_only",
            Self::RestorePlaceholderOnly => "restore_placeholder_only",
            Self::TerminalBoundarySubsetOnly => "terminal_boundary_subset_only",
            Self::PolicyReviewPending => "policy_review_pending",
            Self::ImportedEvidenceOnly => "imported_evidence_only",
            Self::LimitUnbound => "limit_unbound",
        }
    }

    /// Returns true when the known-limit class is fully bound.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::LimitUnbound)
    }

    /// Returns true when the row must cite a disclosure reference.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::NoneDeclared | Self::LimitUnbound)
    }
}

/// Downgrade-rule vocabulary attached to a governance row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeRuleClass {
    /// No downgrade automation is required for the row.
    None,
    /// Narrow when queue metadata or collapse identity goes stale.
    AutoNarrowOnQueueMetadataStale,
    /// Narrow when restore fidelity or no-hidden-rerun proof goes stale.
    AutoNarrowOnRestoreFidelityStale,
    /// Narrow when terminal-boundary or clipboard proof goes stale.
    AutoNarrowOnTerminalBoundaryStale,
    /// Narrow when checkpoint proof is missing or expired.
    AutoNarrowOnMissingCheckpointProof,
    /// Narrow when the retry budget is exhausted.
    AutoNarrowOnRetryBudgetExhausted,
    /// Block when required evidence is missing.
    AutoBlockOnMissingEvidence,
    /// Manual review remains required.
    ManualOnlyPendingReview,
    /// Row has no downgrade-rule class bound.
    AutomationUnbound,
}

impl DowngradeRuleClass {
    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::AutoNarrowOnQueueMetadataStale => "auto_narrow_on_queue_metadata_stale",
            Self::AutoNarrowOnRestoreFidelityStale => "auto_narrow_on_restore_fidelity_stale",
            Self::AutoNarrowOnTerminalBoundaryStale => "auto_narrow_on_terminal_boundary_stale",
            Self::AutoNarrowOnMissingCheckpointProof => "auto_narrow_on_missing_checkpoint_proof",
            Self::AutoNarrowOnRetryBudgetExhausted => "auto_narrow_on_retry_budget_exhausted",
            Self::AutoBlockOnMissingEvidence => "auto_block_on_missing_evidence",
            Self::ManualOnlyPendingReview => "manual_only_pending_review",
            Self::AutomationUnbound => "automation_unbound",
        }
    }

    /// Returns true when the automation class is fully bound.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::AutomationUnbound)
    }

    /// Returns true when the row must cite a disclosure reference.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::None | Self::AutomationUnbound)
    }
}

/// Confidence class attached to one row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceClass {
    /// High confidence backs the row.
    High,
    /// Medium confidence narrows the row.
    Medium,
    /// Low confidence leaves the row under explicit review.
    Low,
}

impl ConfidenceClass {
    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

/// Consumer surface that must preserve this packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Activity-center rows and background-work summaries.
    ActivityCenter,
    /// Restore summary, placeholder, and crash-recovery surfaces.
    RestoreSummary,
    /// Terminal header and boundary-detail surfaces.
    TerminalHeader,
    /// Runtime-heavy notebook, query, preview, or incident surfaces.
    RuntimeSurface,
    /// Support export and support preview surfaces.
    SupportExport,
    /// Release proof index surfaces.
    ReleaseProofIndex,
    /// Docs/help surfaces.
    DocsHelp,
    /// Conformance dashboard or reviewer matrix surfaces.
    ConformanceDashboard,
}

impl ConsumerSurface {
    /// Every consumer projection the packet must preserve.
    pub const REQUIRED: [Self; 8] = [
        Self::ActivityCenter,
        Self::RestoreSummary,
        Self::TerminalHeader,
        Self::RuntimeSurface,
        Self::SupportExport,
        Self::ReleaseProofIndex,
        Self::DocsHelp,
        Self::ConformanceDashboard,
    ];

    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActivityCenter => "activity_center",
            Self::RestoreSummary => "restore_summary",
            Self::TerminalHeader => "terminal_header",
            Self::RuntimeSurface => "runtime_surface",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
            Self::DocsHelp => "docs_help",
            Self::ConformanceDashboard => "conformance_dashboard",
        }
    }
}

/// Promotion state derived from packet validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionState {
    /// Packet may project stable truth.
    Stable,
    /// Packet narrows below stable.
    NarrowedBelowStable,
    /// Packet blocks stable publication.
    BlocksStable,
}

impl PromotionState {
    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity attached to one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    /// Informational finding.
    Info,
    /// Warning-level finding.
    Warning,
    /// Blocker-level finding.
    Blocker,
}

/// Closed validation-finding vocabulary for this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the frozen tag.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Packet or row identity is missing.
    MissingIdentity,
    /// One required workload is missing entirely.
    MissingWorkloadCoverage,
    /// Stable workload lacks its queue identity row.
    MissingQueueIdentityAdmission,
    /// Stable workload lacks its restore continuity row.
    MissingRestoreContinuityAdmission,
    /// Stable workload lacks its terminal boundary row.
    MissingTerminalBoundaryAdmission,
    /// Stable workload lacks its downgrade rule row.
    MissingDowngradeRule,
    /// Row has no bound support class.
    MissingSupportClass,
    /// Row has no bound known-limit class.
    MissingKnownLimit,
    /// Row has no bound downgrade rule class.
    MissingDowngradeRuleClass,
    /// Row has no bound evidence class.
    MissingEvidenceClass,
    /// Stable row claims a missing binding.
    StableWithUnboundBinding,
    /// Narrowed or unsupported row lacks a disclosure reference.
    NarrowedRowMissingDisclosureRef,
    /// Known limit lacks a disclosure reference.
    KnownLimitMissingDisclosureRef,
    /// Downgrade rule lacks a disclosure reference.
    DowngradeRuleMissingDisclosureRef,
    /// Row carries no evidence references.
    MissingEvidenceRefs,
    /// Queue row omitted required queue fields.
    QueueFieldNotApplicable,
    /// Queue row omitted concrete job identities.
    MissingJobIdentities,
    /// Queue row carries an invalid concrete job identity bundle.
    InvalidJobIdentity,
    /// Queue row covers an unknown concrete budget-domain ref.
    UnknownBudgetDomainRef,
    /// Queue row tries to consume protected hot-path budget.
    ProtectedBudgetConsumedByQueueJob,
    /// Non-queue row bound queue-only fields.
    QueueFieldNotPermittedOnRowClass,
    /// Restore row omitted required restore fields.
    RestoreFieldNotApplicable,
    /// Non-restore row bound restore-only fields.
    RestoreFieldNotPermittedOnRowClass,
    /// Terminal row omitted required terminal fields.
    TerminalFieldNotApplicable,
    /// Non-terminal row bound terminal-only fields.
    TerminalFieldNotPermittedOnRowClass,
    /// Packet no longer covers every required queue lane.
    MissingRequiredQueueLaneCoverage,
    /// Packet no longer covers every required concrete job kind.
    MissingRequiredJobKindCoverage,
    /// Packet no longer covers every required restore fidelity class.
    MissingRequiredRestoreFidelityCoverage,
    /// Packet no longer covers every required terminal boundary class.
    MissingRequiredTerminalBoundaryCoverage,
    /// Packet no longer covers every required clipboard posture class.
    MissingRequiredClipboardPostureCoverage,
    /// Packet no longer covers every required durable activity state.
    MissingRequiredActivityStateCoverage,
    /// One covered workload has no durable activity row.
    MissingActivityJobRow,
    /// Activity row cites a job identity ref that the queue rows do not own.
    UnknownActivityJobIdentityRef,
    /// Activity row lane disagrees with the workload's queue-admission lane.
    ActivityJobRowLaneDrift,
    /// Activity row omitted a required reopen, inspect, or next-action ref.
    MissingActivityActionRef,
    /// Packet no longer covers every required scheduler lane row.
    MissingSchedulerLaneCoverage,
    /// Raw source material crossed the boundary.
    RawSourceMaterialPresent,
    /// Secrets crossed the boundary.
    SecretsPresent,
    /// Ambient authority crossed the boundary.
    AmbientAuthorityPresent,
    /// Required consumer projection is missing.
    MissingConsumerProjection,
    /// Consumer projection does not preserve packet truth.
    ConsumerProjectionDrift,
    /// Consumer projection collapsed workload vocabulary.
    WorkloadVocabularyCollapsed,
    /// Consumer projection collapsed lane vocabulary.
    LaneVocabularyCollapsed,
    /// Consumer projection collapsed row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// Consumer projection collapsed restore vocabulary.
    RestoreVocabularyCollapsed,
    /// Consumer projection collapsed terminal-boundary vocabulary.
    TerminalBoundaryVocabularyCollapsed,
    /// Consumer projection collapsed clipboard vocabulary.
    ClipboardVocabularyCollapsed,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl FindingKind {
    /// Returns the stable token used in fixtures, schemas, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingWorkloadCoverage => "missing_workload_coverage",
            Self::MissingQueueIdentityAdmission => "missing_queue_identity_admission",
            Self::MissingRestoreContinuityAdmission => "missing_restore_continuity_admission",
            Self::MissingTerminalBoundaryAdmission => "missing_terminal_boundary_admission",
            Self::MissingDowngradeRule => "missing_downgrade_rule",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeRuleClass => "missing_downgrade_rule_class",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::StableWithUnboundBinding => "stable_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeRuleMissingDisclosureRef => "downgrade_rule_missing_disclosure_ref",
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::QueueFieldNotApplicable => "queue_field_not_applicable",
            Self::MissingJobIdentities => "missing_job_identities",
            Self::InvalidJobIdentity => "invalid_job_identity",
            Self::UnknownBudgetDomainRef => "unknown_budget_domain_ref",
            Self::ProtectedBudgetConsumedByQueueJob => "protected_budget_consumed_by_queue_job",
            Self::QueueFieldNotPermittedOnRowClass => "queue_field_not_permitted_on_row_class",
            Self::MissingRequiredJobKindCoverage => "missing_required_job_kind_coverage",
            Self::RestoreFieldNotApplicable => "restore_field_not_applicable",
            Self::RestoreFieldNotPermittedOnRowClass => "restore_field_not_permitted_on_row_class",
            Self::TerminalFieldNotApplicable => "terminal_field_not_applicable",
            Self::TerminalFieldNotPermittedOnRowClass => {
                "terminal_field_not_permitted_on_row_class"
            }
            Self::MissingRequiredQueueLaneCoverage => "missing_required_queue_lane_coverage",
            Self::MissingRequiredRestoreFidelityCoverage => {
                "missing_required_restore_fidelity_coverage"
            }
            Self::MissingRequiredTerminalBoundaryCoverage => {
                "missing_required_terminal_boundary_coverage"
            }
            Self::MissingRequiredClipboardPostureCoverage => {
                "missing_required_clipboard_posture_coverage"
            }
            Self::MissingRequiredActivityStateCoverage => {
                "missing_required_activity_state_coverage"
            }
            Self::MissingActivityJobRow => "missing_activity_job_row",
            Self::UnknownActivityJobIdentityRef => "unknown_activity_job_identity_ref",
            Self::ActivityJobRowLaneDrift => "activity_job_row_lane_drift",
            Self::MissingActivityActionRef => "missing_activity_action_ref",
            Self::MissingSchedulerLaneCoverage => "missing_scheduler_lane_coverage",
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::WorkloadVocabularyCollapsed => "workload_vocabulary_collapsed",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::RestoreVocabularyCollapsed => "restore_vocabulary_collapsed",
            Self::TerminalBoundaryVocabularyCollapsed => "terminal_boundary_vocabulary_collapsed",
            Self::ClipboardVocabularyCollapsed => "clipboard_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by packet validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    /// Stable finding token.
    pub finding_kind: FindingKind,
    /// Finding severity.
    pub severity: FindingSeverity,
    /// Human-readable detail.
    pub message: String,
}

impl ValidationFinding {
    /// Returns a new validation finding.
    pub fn new(
        finding_kind: FindingKind,
        severity: FindingSeverity,
        message: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            message: message.into(),
        }
    }
}

/// One governed workload row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueSessionTerminalGovernanceRow {
    /// Stable row identity.
    pub row_id: String,
    /// Workload covered by the row.
    pub workload_class: GovernedWorkloadClass,
    /// Row class frozen by the contract.
    pub row_class: GovernanceRowClass,
    /// Support posture.
    pub support_class: SupportClass,
    /// Queue lane admitted by the row.
    pub queue_lane_class: QueueLaneClass,
    /// Collapse key admitted by the row.
    pub collapse_key_class: CollapseKeyClass,
    /// Budget-domain guard admitted by the row.
    pub budget_domain_class: BudgetDomainClass,
    /// Checkpoint policy admitted by the row.
    pub checkpoint_policy_class: CheckpointPolicyClass,
    /// Retry posture admitted by the row.
    pub retry_class: RetryClass,
    /// Cancellation posture admitted by the row.
    pub cancellation_class: CancellationClass,
    /// Opaque queue identity ref admitted by the row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue_identity_ref: Option<String>,
    /// Concrete stable job identities governed by this queue row.
    #[serde(default)]
    pub job_identities: Vec<GovernedJobIdentity>,
    /// Restore fidelity admitted by the row.
    pub restore_fidelity_class: RestoreFidelityClass,
    /// Restore no-hidden-rerun posture admitted by the row.
    pub no_hidden_rerun_class: NoHiddenRerunClass,
    /// Opaque restore anchor ref admitted by the row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore_anchor_ref: Option<String>,
    /// Terminal boundary admitted by the row.
    pub terminal_boundary_class: TerminalBoundaryClass,
    /// Clipboard posture admitted by the row.
    pub clipboard_posture_class: ClipboardPostureClass,
    /// Opaque terminal-boundary ref admitted by the row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boundary_ref: Option<String>,
    /// Known limit attached to the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade rule attached to the row.
    pub downgrade_rule_class: DowngradeRuleClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Confidence class backing the row.
    pub confidence_class: ConfidenceClass,
    /// Evidence references backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Disclosure reference surfaced when the row narrows or carries a known limit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// Capture timestamp for the row.
    pub captured_at: String,
    /// True when raw source material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when secrets are excluded.
    pub secrets_excluded: bool,
    /// True when ambient authority is excluded.
    pub ambient_authority_excluded: bool,
}

impl QueueSessionTerminalGovernanceRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_rule_class.is_bound()
            && self.evidence_class.is_bound()
    }
}

/// Durable M5 activity-job states projected by the queue/activity inspector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityJobStateClass {
    /// The job is queued and waiting on admission or an existing checkpoint.
    Queued,
    /// The job is actively progressing.
    Running,
    /// The job is paused by direct user intent.
    PausedByUser,
    /// The job is paused by policy, admin posture, or capability narrowing.
    PausedByPolicy,
    /// The job is paused by power-saver or thermal protection.
    PausedByPowerThermal,
    /// The job stalled on an error or bounded dependency failure.
    StalledError,
    /// The job resumed from an admitted checkpoint.
    Resumed,
    /// The job was cancelled and remains reviewable.
    Cancelled,
    /// The job was superseded by a newer authoritative job.
    Superseded,
}

impl ActivityJobStateClass {
    /// Every durable activity state the checked-in packet must cover.
    pub const REQUIRED: [Self; 9] = [
        Self::Queued,
        Self::Running,
        Self::PausedByUser,
        Self::PausedByPolicy,
        Self::PausedByPowerThermal,
        Self::StalledError,
        Self::Resumed,
        Self::Cancelled,
        Self::Superseded,
    ];

    /// Returns the stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::PausedByUser => "paused_by_user",
            Self::PausedByPolicy => "paused_by_policy",
            Self::PausedByPowerThermal => "paused_by_power_thermal",
            Self::StalledError => "stalled_error",
            Self::Resumed => "resumed",
            Self::Cancelled => "cancelled",
            Self::Superseded => "superseded",
        }
    }
}

/// Next-step posture surfaced by one durable activity row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityNextActionClass {
    /// Inspect the queue or job detail without replaying side effects.
    InspectQueue,
    /// Resume the same admitted job identity.
    ResumeJob,
    /// Review policy or trust posture before retry.
    ReviewPolicy,
    /// Wait for power or thermal recovery.
    WaitForPowerThermalRecovery,
    /// Retry from the last admitted checkpoint.
    RetryFromCheckpoint,
    /// Open the exact durable target this row refers to.
    ReopenTarget,
    /// Open the successor that superseded this row.
    OpenReplacement,
    /// Open evidence or history without restarting the job.
    OpenEvidence,
}

impl ActivityNextActionClass {
    /// Returns the stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectQueue => "inspect_queue",
            Self::ResumeJob => "resume_job",
            Self::ReviewPolicy => "review_policy",
            Self::WaitForPowerThermalRecovery => "wait_for_power_thermal_recovery",
            Self::RetryFromCheckpoint => "retry_from_checkpoint",
            Self::ReopenTarget => "reopen_target",
            Self::OpenReplacement => "open_replacement",
            Self::OpenEvidence => "open_evidence",
        }
    }
}

/// Retry-state rollup shown on one scheduler lane row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchedulerLaneRetryStateClass {
    /// No queued retry budget is currently active on this lane.
    NoRetryPending,
    /// The lane carries local retry budget state.
    LocalRetryBudgetTracked,
    /// The lane carries provider retry budget state.
    ProviderRetryBudgetTracked,
    /// The lane requires reauthorization before retry.
    ReauthorizeBeforeRetry,
    /// The lane requires explicit manual review before retry.
    ManualReviewBeforeRetry,
}

impl SchedulerLaneRetryStateClass {
    /// Returns the stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoRetryPending => "no_retry_pending",
            Self::LocalRetryBudgetTracked => "local_retry_budget_tracked",
            Self::ProviderRetryBudgetTracked => "provider_retry_budget_tracked",
            Self::ReauthorizeBeforeRetry => "reauthorize_before_retry",
            Self::ManualReviewBeforeRetry => "manual_review_before_retry",
        }
    }
}

/// Durable job row projected to queue/activity inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueSessionActivityJobRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Stable row identity.
    pub row_id: String,
    /// Workload covered by the row.
    pub workload_class: GovernedWorkloadClass,
    /// Primary concrete job kind backing the row.
    pub primary_job_kind: GovernedJobKind,
    /// Stable job identities quoted by the row.
    pub job_identity_refs: Vec<String>,
    /// Stable queue lane token.
    pub queue_lane_class: QueueLaneClass,
    /// Durable job state.
    pub state_class: ActivityJobStateClass,
    /// Stable state token.
    pub state_token: String,
    /// Reviewable state reason.
    pub state_reason: String,
    /// Queue age in whole seconds.
    pub queue_age_seconds: u64,
    /// Human-readable queue age label.
    pub queue_age_label: String,
    /// Retry posture inherited from the queue row.
    pub retry_class: RetryClass,
    /// Next durable action exposed by the row.
    pub next_action_class: ActivityNextActionClass,
    /// Stable next-action target.
    pub next_action_ref: String,
    /// Exact-target reopen route.
    pub exact_target_reopen_ref: String,
    /// Exact target identity preserved by reopen.
    pub exact_target_identity_ref: String,
    /// Inspector/detail route for the row.
    pub inspect_ref: String,
    /// Last checkpoint metadata when the job is queued, paused, resumed, or stalled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_checkpoint: Option<CheckpointMetadata>,
}

/// Scheduler-lane row projected to queue/activity inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueSessionSchedulerLaneRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Queue lane covered by the row.
    pub queue_lane_class: QueueLaneClass,
    /// Stable lane token.
    pub queue_lane_token: String,
    /// Current queue depth for the lane.
    pub queue_depth: u32,
    /// Oldest queued age in whole seconds when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oldest_age_seconds: Option<u64>,
    /// Human-readable oldest age label.
    pub oldest_age_label: String,
    /// Total collapsed job count.
    pub collapse_count: u32,
    /// Retry-state rollup for the lane.
    pub retry_state_class: SchedulerLaneRetryStateClass,
    /// Stable retry-state token.
    pub retry_state_token: String,
    /// Last checkpoint metadata surfaced by the scheduler.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_checkpoint: Option<CheckpointMetadata>,
    /// Durable activity rows currently attributed to the lane.
    pub activity_row_refs: Vec<String>,
    /// Workloads currently attributed to the lane.
    pub workload_classes: Vec<GovernedWorkloadClass>,
    /// Inspectable scheduler truth source.
    pub inspect_ref: String,
}

/// Consumer projection that must preserve this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueSessionTerminalGovernanceConsumerProjection {
    /// Consumer surface that reads this packet.
    pub consumer_surface: ConsumerSurface,
    /// Opaque projection reference.
    pub projection_ref: String,
    /// Packet id preserved by the projection.
    pub packet_id_ref: String,
    /// True when the projection preserves workload vocabulary.
    pub preserves_workload_vocabulary: bool,
    /// True when the projection preserves lane vocabulary.
    pub preserves_lane_vocabulary: bool,
    /// True when the projection preserves row-class vocabulary.
    pub preserves_row_class_vocabulary: bool,
    /// True when the projection preserves restore vocabulary.
    pub preserves_restore_vocabulary: bool,
    /// True when the projection preserves terminal-boundary vocabulary.
    pub preserves_terminal_boundary_vocabulary: bool,
    /// True when the projection preserves clipboard vocabulary.
    pub preserves_clipboard_vocabulary: bool,
    /// True when the projection supports JSON export parity.
    pub supports_json_export: bool,
    /// True when raw private material remains excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority remains excluded.
    pub ambient_authority_excluded: bool,
}

impl QueueSessionTerminalGovernanceConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.packet_id_ref == packet_id
            && self.preserves_workload_vocabulary
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_restore_vocabulary
            && self.preserves_terminal_boundary_vocabulary
            && self.preserves_clipboard_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`QueueSessionTerminalGovernancePacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueSessionTerminalGovernancePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or contract id.
    pub workflow_or_surface_id: String,
    /// Packet capture timestamp.
    pub generated_at: String,
    /// Workloads covered by the packet.
    #[serde(default)]
    pub covered_workloads: Vec<GovernedWorkloadClass>,
    /// Governance rows.
    #[serde(default)]
    pub rows: Vec<QueueSessionTerminalGovernanceRow>,
    /// Durable activity-job rows projected from the same queue truth.
    #[serde(default)]
    pub activity_job_rows: Vec<QueueSessionActivityJobRow>,
    /// Scheduler lane rows projected from the same queue truth.
    #[serde(default)]
    pub scheduler_lane_rows: Vec<QueueSessionSchedulerLaneRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<QueueSessionTerminalGovernanceConsumerProjection>,
    /// Source contracts used by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Runtime-owned queue, restore, and terminal governance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueSessionTerminalGovernancePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Packet capture timestamp.
    pub generated_at: String,
    /// Workloads covered by the packet.
    #[serde(default)]
    pub covered_workloads: Vec<GovernedWorkloadClass>,
    /// Governance rows.
    #[serde(default)]
    pub rows: Vec<QueueSessionTerminalGovernanceRow>,
    /// Durable activity-job rows projected from the same queue truth.
    #[serde(default)]
    pub activity_job_rows: Vec<QueueSessionActivityJobRow>,
    /// Scheduler lane rows projected from the same queue truth.
    #[serde(default)]
    pub scheduler_lane_rows: Vec<QueueSessionSchedulerLaneRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<QueueSessionTerminalGovernanceConsumerProjection>,
    /// Source contract references used by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl QueueSessionTerminalGovernancePacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: QueueSessionTerminalGovernancePacketInput) -> Self {
        let mut packet = Self {
            record_kind: QUEUE_SESSION_TERMINAL_GOVERNANCE_RECORD_KIND.to_owned(),
            schema_version: QUEUE_SESSION_TERMINAL_GOVERNANCE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_workloads: input.covered_workloads,
            rows: input.rows,
            activity_job_rows: input.activity_job_rows,
            scheduler_lane_rows: input.scheduler_lane_rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: PromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against frozen governance invariants.
    pub fn validate(&self) -> Vec<ValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns the unique workload tokens observed across rows.
    pub fn workload_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.workload_class);
        }
        set.into_iter().map(GovernedWorkloadClass::as_str).collect()
    }

    /// Returns the unique queue-lane tokens observed across rows.
    pub fn queue_lane_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.queue_lane_class);
        }
        set.into_iter().map(QueueLaneClass::as_str).collect()
    }

    /// Returns the unique concrete job-kind tokens observed across queue rows.
    pub fn job_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            for identity in &row.job_identities {
                set.insert(identity.job_kind);
            }
        }
        set.into_iter().map(GovernedJobKind::as_str).collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter().map(GovernanceRowClass::as_str).collect()
    }

    /// Returns the unique restore-fidelity tokens observed across rows.
    pub fn restore_fidelity_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.restore_fidelity_class);
        }
        set.into_iter().map(RestoreFidelityClass::as_str).collect()
    }

    /// Returns the unique terminal-boundary tokens observed across rows.
    pub fn terminal_boundary_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.terminal_boundary_class);
        }
        set.into_iter().map(TerminalBoundaryClass::as_str).collect()
    }

    /// Returns the unique clipboard-posture tokens observed across rows.
    pub fn clipboard_posture_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.clipboard_posture_class);
        }
        set.into_iter().map(ClipboardPostureClass::as_str).collect()
    }

    /// Returns the unique durable activity-state tokens observed across rows.
    pub fn activity_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.activity_job_rows {
            set.insert(row.state_class);
        }
        set.into_iter().map(ActivityJobStateClass::as_str).collect()
    }

    /// Returns the unique scheduler retry-state tokens observed across rows.
    pub fn scheduler_retry_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.scheduler_lane_rows {
            set.insert(row.retry_state_class);
        }
        set.into_iter()
            .map(SchedulerLaneRetryStateClass::as_str)
            .collect()
    }

    /// Returns true when the packet preserves a projection for the surface.
    pub fn has_projection_for(&self, surface: ConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns a support export that preserves the product packet verbatim.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> QueueSessionTerminalGovernanceSupportExport {
        QueueSessionTerminalGovernanceSupportExport {
            record_kind: QUEUE_SESSION_TERMINAL_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: QUEUE_SESSION_TERMINAL_GOVERNANCE_SCHEMA_VERSION,
            export_id: export_id.into(),
            packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            governance_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != QUEUE_SESSION_TERMINAL_GOVERNANCE_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "queue/session/terminal governance packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != QUEUE_SESSION_TERMINAL_GOVERNANCE_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "queue/session/terminal governance packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(ValidationFinding::new(
                FindingKind::MissingIdentity,
                FindingSeverity::Blocker,
                "packet, workflow, and timestamp refs are required",
            ));
        }

        for workload in GovernedWorkloadClass::REQUIRED {
            let declared = self.covered_workloads.contains(&workload);
            let present = self.rows.iter().any(|row| row.workload_class == workload);
            if !declared || !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingWorkloadCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "workload {} is not fully covered by the packet",
                        workload.as_str()
                    ),
                ));
            }
        }

        for row in &self.rows {
            if row.row_id.trim().is_empty() || row.captured_at.trim().is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingIdentity,
                    FindingSeverity::Blocker,
                    format!("row {} identity or timestamp is empty", row.row_id),
                ));
            }
            if !row.raw_private_material_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::RawSourceMaterialPresent,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} admits raw source material past the boundary",
                        row.row_id
                    ),
                ));
            }
            if !row.secrets_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::SecretsPresent,
                    FindingSeverity::Blocker,
                    format!("row {} admits secrets past the boundary", row.row_id),
                ));
            }
            if !row.ambient_authority_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::AmbientAuthorityPresent,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} admits ambient authority past the boundary",
                        row.row_id
                    ),
                ));
            }
            if !row.support_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingSupportClass,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound support class", row.row_id),
                ));
            }
            if !row.known_limit_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingKnownLimit,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound known limit class", row.row_id),
                ));
            }
            if !row.downgrade_rule_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingDowngradeRuleClass,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound downgrade rule class", row.row_id),
                ));
            }
            if !row.evidence_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingEvidenceClass,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound evidence class", row.row_id),
                ));
            }
            if matches!(row.support_class, SupportClass::Stable) && !row.all_bindings_satisfied() {
                findings.push(ValidationFinding::new(
                    FindingKind::StableWithUnboundBinding,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} claims stable while support, limit, downgrade, or evidence bindings are unbound",
                        row.row_id
                    ),
                ));
            }
            if row.support_class.requires_disclosure() && row.disclosure_ref.is_none() {
                findings.push(ValidationFinding::new(
                    FindingKind::NarrowedRowMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has support class {} without a disclosure ref",
                        row.row_id,
                        row.support_class.as_str()
                    ),
                ));
            }
            if row.known_limit_class.requires_disclosure() && row.disclosure_ref.is_none() {
                findings.push(ValidationFinding::new(
                    FindingKind::KnownLimitMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has known limit {} without a disclosure ref",
                        row.row_id,
                        row.known_limit_class.as_str()
                    ),
                ));
            }
            if row.downgrade_rule_class.requires_disclosure() && row.disclosure_ref.is_none() {
                findings.push(ValidationFinding::new(
                    FindingKind::DowngradeRuleMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds downgrade rule {} without a disclosure ref",
                        row.row_id,
                        row.downgrade_rule_class.as_str()
                    ),
                ));
            }
            if row.evidence_refs.is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingEvidenceRefs,
                    FindingSeverity::Blocker,
                    format!("row {} carries no evidence refs", row.row_id),
                ));
            }

            if row.row_class.requires_queue_fields() {
                if matches!(row.queue_lane_class, QueueLaneClass::NotApplicable)
                    || matches!(row.collapse_key_class, CollapseKeyClass::NotApplicable)
                    || matches!(row.budget_domain_class, BudgetDomainClass::NotApplicable)
                    || matches!(
                        row.checkpoint_policy_class,
                        CheckpointPolicyClass::NotApplicable
                    )
                    || matches!(row.retry_class, RetryClass::NotApplicable)
                    || matches!(row.cancellation_class, CancellationClass::NotApplicable)
                    || row
                        .queue_identity_ref
                        .as_deref()
                        .map(str::trim)
                        .map(str::is_empty)
                        .unwrap_or(true)
                {
                    findings.push(ValidationFinding::new(
                        FindingKind::QueueFieldNotApplicable,
                        FindingSeverity::Blocker,
                        format!("row {} omits required queue identity fields", row.row_id),
                    ));
                }
                if row.job_identities.is_empty() {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingJobIdentities,
                        FindingSeverity::Blocker,
                        format!("row {} omits concrete job identities", row.row_id),
                    ));
                }
                for identity in &row.job_identities {
                    if !identity.is_valid() {
                        findings.push(ValidationFinding::new(
                            FindingKind::InvalidJobIdentity,
                            FindingSeverity::Blocker,
                            format!(
                                "row {} carries an invalid job identity for {}",
                                row.row_id,
                                identity.job_kind.as_str()
                            ),
                        ));
                    }
                    for budget_domain_ref in &identity.budget_domain_refs {
                        if !is_known_budget_domain_ref(budget_domain_ref) {
                            findings.push(ValidationFinding::new(
                                FindingKind::UnknownBudgetDomainRef,
                                FindingSeverity::Blocker,
                                format!(
                                    "row {} uses unknown budget domain {} for {}",
                                    row.row_id,
                                    budget_domain_ref,
                                    identity.job_kind.as_str()
                                ),
                            ));
                        }
                        if budget_domain_ref == HOT_PATH_INTERACTIVE_BUDGET_DOMAIN_REF {
                            findings.push(ValidationFinding::new(
                                FindingKind::ProtectedBudgetConsumedByQueueJob,
                                FindingSeverity::Blocker,
                                format!(
                                    "row {} tries to consume protected hot-path budget for {}",
                                    row.row_id,
                                    identity.job_kind.as_str()
                                ),
                            ));
                        }
                    }
                }
            } else if !matches!(row.queue_lane_class, QueueLaneClass::NotApplicable)
                || !matches!(row.collapse_key_class, CollapseKeyClass::NotApplicable)
                || !matches!(row.budget_domain_class, BudgetDomainClass::NotApplicable)
                || !matches!(
                    row.checkpoint_policy_class,
                    CheckpointPolicyClass::NotApplicable
                )
                || !matches!(row.retry_class, RetryClass::NotApplicable)
                || !matches!(row.cancellation_class, CancellationClass::NotApplicable)
                || row.queue_identity_ref.is_some()
                || !row.job_identities.is_empty()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::QueueFieldNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds queue fields on row class {}",
                        row.row_id,
                        row.row_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_restore_fields() {
                if matches!(
                    row.restore_fidelity_class,
                    RestoreFidelityClass::NotApplicable
                ) || matches!(row.no_hidden_rerun_class, NoHiddenRerunClass::NotApplicable)
                    || row
                        .restore_anchor_ref
                        .as_deref()
                        .map(str::trim)
                        .map(str::is_empty)
                        .unwrap_or(true)
                {
                    findings.push(ValidationFinding::new(
                        FindingKind::RestoreFieldNotApplicable,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} omits required restore continuity fields",
                            row.row_id
                        ),
                    ));
                }
            } else if !matches!(
                row.restore_fidelity_class,
                RestoreFidelityClass::NotApplicable
            ) || !matches!(row.no_hidden_rerun_class, NoHiddenRerunClass::NotApplicable)
                || row.restore_anchor_ref.is_some()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::RestoreFieldNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds restore fields on row class {}",
                        row.row_id,
                        row.row_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_terminal_fields() {
                if matches!(
                    row.terminal_boundary_class,
                    TerminalBoundaryClass::NotApplicable
                ) || matches!(
                    row.clipboard_posture_class,
                    ClipboardPostureClass::NotApplicable
                ) || row
                    .boundary_ref
                    .as_deref()
                    .map(str::trim)
                    .map(str::is_empty)
                    .unwrap_or(true)
                {
                    findings.push(ValidationFinding::new(
                        FindingKind::TerminalFieldNotApplicable,
                        FindingSeverity::Blocker,
                        format!("row {} omits required terminal boundary fields", row.row_id),
                    ));
                }
            } else if !matches!(
                row.terminal_boundary_class,
                TerminalBoundaryClass::NotApplicable
            ) || !matches!(
                row.clipboard_posture_class,
                ClipboardPostureClass::NotApplicable
            ) || row.boundary_ref.is_some()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::TerminalFieldNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds terminal fields on row class {}",
                        row.row_id,
                        row.row_class.as_str()
                    ),
                ));
            }
        }

        for workload in &self.covered_workloads {
            let stable_quality = self.rows.iter().any(|row| {
                row.workload_class == *workload
                    && matches!(row.row_class, GovernanceRowClass::ContinuityQuality)
                    && matches!(row.support_class, SupportClass::Stable)
            });
            if !stable_quality {
                continue;
            }

            if !self.rows.iter().any(|row| {
                row.workload_class == *workload
                    && matches!(row.row_class, GovernanceRowClass::QueueIdentityAdmission)
            }) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingQueueIdentityAdmission,
                    FindingSeverity::Blocker,
                    format!(
                        "workload {} claims stable continuity without queue identity admission",
                        workload.as_str()
                    ),
                ));
            }
            if !self.rows.iter().any(|row| {
                row.workload_class == *workload
                    && matches!(
                        row.row_class,
                        GovernanceRowClass::RestoreContinuityAdmission
                    )
            }) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingRestoreContinuityAdmission,
                    FindingSeverity::Blocker,
                    format!(
                        "workload {} claims stable continuity without restore continuity admission",
                        workload.as_str()
                    ),
                ));
            }
            if workload.requires_terminal_boundary()
                && !self.rows.iter().any(|row| {
                    row.workload_class == *workload
                        && matches!(row.row_class, GovernanceRowClass::TerminalBoundaryAdmission)
                })
            {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingTerminalBoundaryAdmission,
                    FindingSeverity::Blocker,
                    format!(
                        "workload {} claims stable continuity without terminal boundary admission",
                        workload.as_str()
                    ),
                ));
            }
            if !self.rows.iter().any(|row| {
                row.workload_class == *workload
                    && matches!(row.row_class, GovernanceRowClass::DowngradeRule)
            }) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingDowngradeRule,
                    FindingSeverity::Blocker,
                    format!(
                        "workload {} claims stable continuity without a downgrade rule row",
                        workload.as_str()
                    ),
                ));
            }
        }

        let known_job_identity_refs = self
            .rows
            .iter()
            .filter(|row| matches!(row.row_class, GovernanceRowClass::QueueIdentityAdmission))
            .flat_map(|row| {
                row.job_identities
                    .iter()
                    .map(|identity| identity.job_identity_ref.as_str())
            })
            .collect::<BTreeSet<_>>();
        for workload in GovernedWorkloadClass::REQUIRED {
            if !self
                .activity_job_rows
                .iter()
                .any(|row| row.workload_class == workload)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingActivityJobRow,
                    FindingSeverity::Blocker,
                    format!(
                        "workload {} is missing a durable activity row",
                        workload.as_str()
                    ),
                ));
            }
        }
        for state in ActivityJobStateClass::REQUIRED {
            if !self
                .activity_job_rows
                .iter()
                .any(|row| row.state_class == state)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingRequiredActivityStateCoverage,
                    FindingSeverity::Blocker,
                    format!("activity state {} is not covered", state.as_str()),
                ));
            }
        }
        for activity_row in &self.activity_job_rows {
            if activity_row.row_id.trim().is_empty()
                || activity_row.state_reason.trim().is_empty()
                || activity_row.next_action_ref.trim().is_empty()
                || activity_row.exact_target_reopen_ref.trim().is_empty()
                || activity_row.exact_target_identity_ref.trim().is_empty()
                || activity_row.inspect_ref.trim().is_empty()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingActivityActionRef,
                    FindingSeverity::Blocker,
                    format!(
                        "activity row {} omits a required reopen, inspect, or next-action ref",
                        activity_row.row_id
                    ),
                ));
            }
            if activity_row.job_identity_refs.is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::UnknownActivityJobIdentityRef,
                    FindingSeverity::Blocker,
                    format!(
                        "activity row {} carries no job identity refs",
                        activity_row.row_id
                    ),
                ));
            }
            for job_identity_ref in &activity_row.job_identity_refs {
                if !known_job_identity_refs.contains(job_identity_ref.as_str()) {
                    findings.push(ValidationFinding::new(
                        FindingKind::UnknownActivityJobIdentityRef,
                        FindingSeverity::Blocker,
                        format!(
                            "activity row {} cites unknown job identity {}",
                            activity_row.row_id, job_identity_ref
                        ),
                    ));
                }
            }
            if let Some(queue_row) = self.rows.iter().find(|row| {
                row.workload_class == activity_row.workload_class
                    && matches!(row.row_class, GovernanceRowClass::QueueIdentityAdmission)
            }) {
                if queue_row.queue_lane_class != activity_row.queue_lane_class {
                    findings.push(ValidationFinding::new(
                        FindingKind::ActivityJobRowLaneDrift,
                        FindingSeverity::Blocker,
                        format!(
                            "activity row {} drifts from workload {} queue lane",
                            activity_row.row_id,
                            activity_row.workload_class.as_str()
                        ),
                    ));
                }
            }
        }
        for lane in QueueLaneClass::REQUIRED {
            if !self
                .scheduler_lane_rows
                .iter()
                .any(|row| row.queue_lane_class == lane)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingSchedulerLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("scheduler lane {} is not covered", lane.as_str()),
                ));
            }
        }
        for lane_row in &self.scheduler_lane_rows {
            if lane_row.activity_row_refs.is_empty()
                || lane_row.workload_classes.is_empty()
                || lane_row.inspect_ref.trim().is_empty()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingSchedulerLaneCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "scheduler lane {} omits activity coverage or inspect refs",
                        lane_row.queue_lane_token
                    ),
                ));
            }
        }

        for queue_lane in QueueLaneClass::REQUIRED {
            if !self.rows.iter().any(|row| {
                matches!(row.row_class, GovernanceRowClass::QueueIdentityAdmission)
                    && row.queue_lane_class == queue_lane
            }) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingRequiredQueueLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("queue lane {} is not covered", queue_lane.as_str()),
                ));
            }
        }
        for job_kind in GovernedJobKind::REQUIRED {
            if !self.rows.iter().any(|row| {
                matches!(row.row_class, GovernanceRowClass::QueueIdentityAdmission)
                    && row
                        .job_identities
                        .iter()
                        .any(|identity| identity.job_kind == job_kind)
            }) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingRequiredJobKindCoverage,
                    FindingSeverity::Blocker,
                    format!("job kind {} is not covered", job_kind.as_str()),
                ));
            }
        }
        for fidelity in RestoreFidelityClass::REQUIRED {
            if !self.rows.iter().any(|row| {
                matches!(
                    row.row_class,
                    GovernanceRowClass::RestoreContinuityAdmission
                ) && row.restore_fidelity_class == fidelity
            }) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingRequiredRestoreFidelityCoverage,
                    FindingSeverity::Blocker,
                    format!("restore fidelity {} is not covered", fidelity.as_str()),
                ));
            }
        }
        for boundary in TerminalBoundaryClass::REQUIRED {
            if !self.rows.iter().any(|row| {
                matches!(row.row_class, GovernanceRowClass::TerminalBoundaryAdmission)
                    && row.terminal_boundary_class == boundary
            }) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingRequiredTerminalBoundaryCoverage,
                    FindingSeverity::Blocker,
                    format!("terminal boundary {} is not covered", boundary.as_str()),
                ));
            }
        }
        for posture in ClipboardPostureClass::REQUIRED {
            if !self.rows.iter().any(|row| {
                matches!(row.row_class, GovernanceRowClass::TerminalBoundaryAdmission)
                    && row.clipboard_posture_class == posture
            }) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingRequiredClipboardPostureCoverage,
                    FindingSeverity::Blocker,
                    format!("clipboard posture {} is not covered", posture.as_str()),
                ));
            }
        }

        for required_surface in ConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingConsumerProjection,
                    FindingSeverity::Blocker,
                    format!(
                        "packet {} is missing a preserved {} projection",
                        self.packet_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }
        for projection in &self.consumer_projections {
            if !projection.preserves_truth_for(&self.packet_id) {
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerProjectionDrift,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve packet truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_workload_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::WorkloadVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses workload vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_lane_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::LaneVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses lane vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_row_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::RowClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses row-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_restore_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::RestoreVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses restore vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_terminal_boundary_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::TerminalBoundaryVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses terminal-boundary vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_clipboard_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::ClipboardVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses clipboard vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion
                .retain(|finding| finding.finding_kind != FindingKind::PromotionStateMismatch);
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(ValidationFinding::new(
                    FindingKind::PromotionStateMismatch,
                    FindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

fn promotion_state_for_findings(findings: &[ValidationFinding]) -> PromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Blocker)
    {
        PromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Warning)
    {
        PromotionState::NarrowedBelowStable
    } else {
        PromotionState::Stable
    }
}

fn is_known_budget_domain_ref(budget_domain_ref: &str) -> bool {
    matches!(
        budget_domain_ref,
        FOREGROUND_TASK_BUDGET_DOMAIN_REF
            | KNOWLEDGE_REFRESH_BUDGET_DOMAIN_REF
            | MAINTENANCE_BUDGET_DOMAIN_REF
            | PROVIDER_OVERLAY_BUDGET_DOMAIN_REF
            | REPLICATION_BUDGET_DOMAIN_REF
            | HOT_PATH_INTERACTIVE_BUDGET_DOMAIN_REF
    )
}

/// Support export wrapper that preserves the exact packet shown in product surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueSessionTerminalGovernanceSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Preserved packet id.
    pub packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material stays excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority stays excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub governance_packet: QueueSessionTerminalGovernancePacket,
}

impl QueueSessionTerminalGovernanceSupportExport {
    /// Returns true when the export safely preserves the packet verbatim.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == QUEUE_SESSION_TERMINAL_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == QUEUE_SESSION_TERMINAL_GOVERNANCE_SCHEMA_VERSION
            && self.packet_id_ref == self.governance_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.governance_packet.validate().is_empty()
    }
}

fn stable_projection(surface: ConsumerSurface) -> QueueSessionTerminalGovernanceConsumerProjection {
    QueueSessionTerminalGovernanceConsumerProjection {
        consumer_surface: surface,
        projection_ref: format!("projection:{}", surface.as_str()),
        packet_id_ref: "packet:queue-session-terminal-governance".to_owned(),
        preserves_workload_vocabulary: true,
        preserves_lane_vocabulary: true,
        preserves_row_class_vocabulary: true,
        preserves_restore_vocabulary: true,
        preserves_terminal_boundary_vocabulary: true,
        preserves_clipboard_vocabulary: true,
        supports_json_export: true,
        raw_private_material_excluded: true,
        ambient_authority_excluded: true,
    }
}

fn row_base(
    row_id: &str,
    workload_class: GovernedWorkloadClass,
    row_class: GovernanceRowClass,
) -> QueueSessionTerminalGovernanceRow {
    QueueSessionTerminalGovernanceRow {
        row_id: row_id.to_owned(),
        workload_class,
        row_class,
        support_class: SupportClass::Stable,
        queue_lane_class: QueueLaneClass::NotApplicable,
        collapse_key_class: CollapseKeyClass::NotApplicable,
        budget_domain_class: BudgetDomainClass::NotApplicable,
        checkpoint_policy_class: CheckpointPolicyClass::NotApplicable,
        retry_class: RetryClass::NotApplicable,
        cancellation_class: CancellationClass::NotApplicable,
        queue_identity_ref: None,
        job_identities: Vec::new(),
        restore_fidelity_class: RestoreFidelityClass::NotApplicable,
        no_hidden_rerun_class: NoHiddenRerunClass::NotApplicable,
        restore_anchor_ref: None,
        terminal_boundary_class: TerminalBoundaryClass::NotApplicable,
        clipboard_posture_class: ClipboardPostureClass::NotApplicable,
        boundary_ref: None,
        known_limit_class: KnownLimitClass::NoneDeclared,
        downgrade_rule_class: DowngradeRuleClass::None,
        evidence_class: EvidenceClass::SchemaFixtureEvidence,
        confidence_class: ConfidenceClass::High,
        evidence_refs: vec![
            QUEUE_SESSION_TERMINAL_GOVERNANCE_SCHEMA_REF.to_owned(),
            QUEUE_SESSION_TERMINAL_GOVERNANCE_DOC_REF.to_owned(),
        ],
        disclosure_ref: None,
        captured_at: "2026-06-12T00:00:00Z".to_owned(),
        raw_private_material_excluded: true,
        secrets_excluded: true,
        ambient_authority_excluded: true,
    }
}

fn quality_row(workload: GovernedWorkloadClass) -> QueueSessionTerminalGovernanceRow {
    let mut row = row_base(
        &format!("row:{}:quality", workload.as_str()),
        workload,
        GovernanceRowClass::ContinuityQuality,
    );
    row.evidence_class = EvidenceClass::ReleaseEvidenceReview;
    row.evidence_refs
        .push(QUEUE_SESSION_TERMINAL_GOVERNANCE_ARTIFACT_DOC_REF.to_owned());
    row
}

fn workspace_scope(scope_class: &str, scope_ref: &str) -> QueueJobScope {
    QueueJobScope {
        scope_class: scope_class.to_owned(),
        scope_ref: scope_ref.to_owned(),
    }
}

fn job_identity(
    job_kind: GovernedJobKind,
    job_identity_ref: &str,
    workspace_id_ref: &str,
    slice_id_ref: Option<&str>,
    scope: QueueJobScope,
    initiating_source: QueueInitiatingSource,
    collapse_key: &str,
    collapse_policy: QueueCollapsePolicy,
    staleness_policy: QueueStalenessPolicy,
    budget_domain_refs: &[&str],
    workspace_revision_ref: &str,
    manifest_hash_ref: Option<&str>,
    execution_context_hash_ref: Option<&str>,
    policy_epoch_ref: Option<&str>,
) -> GovernedJobIdentity {
    GovernedJobIdentity {
        job_kind,
        job_identity_ref: job_identity_ref.to_owned(),
        workspace_id_ref: workspace_id_ref.to_owned(),
        slice_id_ref: slice_id_ref.map(str::to_owned),
        scope,
        initiating_source,
        collapse_key: collapse_key.to_owned(),
        collapse_policy,
        staleness_policy,
        budget_domain_refs: budget_domain_refs
            .iter()
            .map(|budget| (*budget).to_owned())
            .collect(),
        workspace_revision_ref: workspace_revision_ref.to_owned(),
        manifest_hash_ref: manifest_hash_ref.map(str::to_owned),
        execution_context_hash_ref: execution_context_hash_ref.map(str::to_owned),
        policy_epoch_ref: policy_epoch_ref.map(str::to_owned),
    }
}

fn queue_row(
    workload: GovernedWorkloadClass,
    queue_lane_class: QueueLaneClass,
    collapse_key_class: CollapseKeyClass,
    budget_domain_class: BudgetDomainClass,
    checkpoint_policy_class: CheckpointPolicyClass,
    retry_class: RetryClass,
    cancellation_class: CancellationClass,
    job_identities: Vec<GovernedJobIdentity>,
) -> QueueSessionTerminalGovernanceRow {
    let mut row = row_base(
        &format!("row:{}:queue", workload.as_str()),
        workload,
        GovernanceRowClass::QueueIdentityAdmission,
    );
    row.queue_lane_class = queue_lane_class;
    row.collapse_key_class = collapse_key_class;
    row.budget_domain_class = budget_domain_class;
    row.checkpoint_policy_class = checkpoint_policy_class;
    row.retry_class = retry_class;
    row.cancellation_class = cancellation_class;
    row.queue_identity_ref = Some(format!("queue:{}", workload.as_str()));
    row.job_identities = job_identities;
    row.evidence_class = EvidenceClass::AutomatedFunctionalEvidence;
    row
}

fn restore_row(
    workload: GovernedWorkloadClass,
    restore_fidelity_class: RestoreFidelityClass,
    no_hidden_rerun_class: NoHiddenRerunClass,
) -> QueueSessionTerminalGovernanceRow {
    let mut row = row_base(
        &format!("row:{}:restore", workload.as_str()),
        workload,
        GovernanceRowClass::RestoreContinuityAdmission,
    );
    row.restore_fidelity_class = restore_fidelity_class;
    row.no_hidden_rerun_class = no_hidden_rerun_class;
    row.restore_anchor_ref = Some(format!("restore:{}", workload.as_str()));
    row.evidence_class = EvidenceClass::FailureRecoveryDrillEvidence;
    row
}

fn terminal_row(
    workload: GovernedWorkloadClass,
    terminal_boundary_class: TerminalBoundaryClass,
    clipboard_posture_class: ClipboardPostureClass,
) -> QueueSessionTerminalGovernanceRow {
    let mut row = row_base(
        &format!("row:{}:terminal", workload.as_str()),
        workload,
        GovernanceRowClass::TerminalBoundaryAdmission,
    );
    row.terminal_boundary_class = terminal_boundary_class;
    row.clipboard_posture_class = clipboard_posture_class;
    row.boundary_ref = Some(format!("boundary:{}", workload.as_str()));
    row.evidence_class = EvidenceClass::SecurityPrivacyReviewEvidence;
    row
}

fn downgrade_row(
    workload: GovernedWorkloadClass,
    downgrade_rule_class: DowngradeRuleClass,
) -> QueueSessionTerminalGovernanceRow {
    let mut row = row_base(
        &format!("row:{}:downgrade", workload.as_str()),
        workload,
        GovernanceRowClass::DowngradeRule,
    );
    row.downgrade_rule_class = downgrade_rule_class;
    row.evidence_class = EvidenceClass::DocsDisclosureEvidence;
    row.disclosure_ref = Some(format!("disclosure:{}", workload.as_str()));
    row
}

#[derive(Debug, Clone, Copy)]
struct ActivityRowSeed {
    workload_class: GovernedWorkloadClass,
    primary_job_kind: GovernedJobKind,
    state_class: ActivityJobStateClass,
    next_action_class: ActivityNextActionClass,
    state_reason: &'static str,
    exact_target_identity_ref: &'static str,
}

fn stable_activity_row_seeds() -> [ActivityRowSeed; 10] {
    [
        ActivityRowSeed {
            workload_class: GovernedWorkloadClass::NotebookSession,
            primary_job_kind: GovernedJobKind::NotebookCellExecution,
            state_class: ActivityJobStateClass::Queued,
            next_action_class: ActivityNextActionClass::InspectQueue,
            state_reason:
                "Notebook execution is queued behind the current interactive checkpoint and preserves the exact cell/run target.",
            exact_target_identity_ref: "target:notebook:training:run",
        },
        ActivityRowSeed {
            workload_class: GovernedWorkloadClass::DataQueryConsole,
            primary_job_kind: GovernedJobKind::DataRequestCollectionRun,
            state_class: ActivityJobStateClass::Running,
            next_action_class: ActivityNextActionClass::ReopenTarget,
            state_reason:
                "The query session is actively running against its current remote request context.",
            exact_target_identity_ref: "target:data:orders:query_session",
        },
        ActivityRowSeed {
            workload_class: GovernedWorkloadClass::PipelineRun,
            primary_job_kind: GovernedJobKind::PipelineLogPull,
            state_class: ActivityJobStateClass::StalledError,
            next_action_class: ActivityNextActionClass::RetryFromCheckpoint,
            state_reason:
                "Provider-backed pipeline refresh stalled on a bounded remote failure and can resume from the last admitted checkpoint.",
            exact_target_identity_ref: "target:pipeline:deploy_run",
        },
        ActivityRowSeed {
            workload_class: GovernedWorkloadClass::PreviewRoute,
            primary_job_kind: GovernedJobKind::PreviewDevServer,
            state_class: ActivityJobStateClass::PausedByPowerThermal,
            next_action_class: ActivityNextActionClass::WaitForPowerThermalRecovery,
            state_reason:
                "Preview warm-up is paused by power and thermal protection to preserve protected-path latency.",
            exact_target_identity_ref: "target:preview:webapp:route",
        },
        ActivityRowSeed {
            workload_class: GovernedWorkloadClass::ProfilerCapture,
            primary_job_kind: GovernedJobKind::ProfilerCapture,
            state_class: ActivityJobStateClass::Cancelled,
            next_action_class: ActivityNextActionClass::OpenEvidence,
            state_reason:
                "Profiler capture was cancelled explicitly and remains reviewable through its durable evidence packet.",
            exact_target_identity_ref: "target:profiler:startup_capture",
        },
        ActivityRowSeed {
            workload_class: GovernedWorkloadClass::DocsRecall,
            primary_job_kind: GovernedJobKind::DocsPackRefresh,
            state_class: ActivityJobStateClass::Resumed,
            next_action_class: ActivityNextActionClass::ReopenTarget,
            state_reason:
                "Docs recall resumed from the last admitted refresh boundary instead of rescanning from zero.",
            exact_target_identity_ref: "target:docs:rust:recall_session",
        },
        ActivityRowSeed {
            workload_class: GovernedWorkloadClass::SyncOffboardingFlow,
            primary_job_kind: GovernedJobKind::SyncOffboardingExport,
            state_class: ActivityJobStateClass::PausedByPolicy,
            next_action_class: ActivityNextActionClass::ReviewPolicy,
            state_reason:
                "Offboarding export is paused by policy until export scope and retention posture are re-reviewed.",
            exact_target_identity_ref: "target:sync:offboarding_export",
        },
        ActivityRowSeed {
            workload_class: GovernedWorkloadClass::CompanionHandoff,
            primary_job_kind: GovernedJobKind::CompanionHandoffPackage,
            state_class: ActivityJobStateClass::PausedByUser,
            next_action_class: ActivityNextActionClass::ResumeJob,
            state_reason:
                "The companion handoff package is paused by explicit user choice and can resume under the same durable identity.",
            exact_target_identity_ref: "target:companion:handoff_packet",
        },
        ActivityRowSeed {
            workload_class: GovernedWorkloadClass::IncidentWorkspace,
            primary_job_kind: GovernedJobKind::IncidentRecoveryWorkspaceRefresh,
            state_class: ActivityJobStateClass::Superseded,
            next_action_class: ActivityNextActionClass::OpenReplacement,
            state_reason:
                "A newer incident workspace refresh superseded this row; the history remains durable and links to the successor.",
            exact_target_identity_ref: "target:incident:sev1:workspace",
        },
        ActivityRowSeed {
            workload_class: GovernedWorkloadClass::InfrastructureSession,
            primary_job_kind: GovernedJobKind::InfrastructureOverlayProbe,
            state_class: ActivityJobStateClass::Running,
            next_action_class: ActivityNextActionClass::ReopenTarget,
            state_reason:
                "Infrastructure overlay refresh is running with boundary-labelled reconnect and inspect affordances.",
            exact_target_identity_ref: "target:infra:cluster-a:overlay_session",
        },
    ]
}

fn format_age_label(seconds: Option<u64>) -> String {
    match seconds {
        Some(seconds) if seconds >= 60 => format!("{}m {}s", seconds / 60, seconds % 60),
        Some(seconds) => format!("{seconds}s"),
        None => "n/a".to_owned(),
    }
}

fn governor_lane_for(queue_lane_class: QueueLaneClass) -> GovernorQueueLane {
    match queue_lane_class {
        QueueLaneClass::Foreground => GovernorQueueLane::Foreground,
        QueueLaneClass::InteractiveBackground => GovernorQueueLane::InteractiveBackground,
        QueueLaneClass::Maintenance => GovernorQueueLane::Maintenance,
        QueueLaneClass::ProviderOverlay => GovernorQueueLane::ProviderOverlay,
        QueueLaneClass::UploadReplication => GovernorQueueLane::UploadReplication,
        QueueLaneClass::NotApplicable => GovernorQueueLane::Foreground,
    }
}

fn retry_state_for_lane(rows: &[QueueSessionActivityJobRow]) -> SchedulerLaneRetryStateClass {
    if rows
        .iter()
        .any(|row| matches!(row.retry_class, RetryClass::ManualRequeueOnly))
    {
        SchedulerLaneRetryStateClass::ManualReviewBeforeRetry
    } else if rows
        .iter()
        .any(|row| matches!(row.retry_class, RetryClass::ReconnectAfterReauth))
    {
        SchedulerLaneRetryStateClass::ReauthorizeBeforeRetry
    } else if rows
        .iter()
        .any(|row| matches!(row.retry_class, RetryClass::ProviderRetryBudget))
    {
        SchedulerLaneRetryStateClass::ProviderRetryBudgetTracked
    } else if rows
        .iter()
        .any(|row| matches!(row.retry_class, RetryClass::LocalRetryBudget))
    {
        SchedulerLaneRetryStateClass::LocalRetryBudgetTracked
    } else {
        SchedulerLaneRetryStateClass::NoRetryPending
    }
}

fn build_activity_job_rows(
    rows: &[QueueSessionTerminalGovernanceRow],
) -> Vec<QueueSessionActivityJobRow> {
    let scheduler_snapshot = seeded_resource_governor_snapshot(
        "resource-governor:snapshot:queue-session-terminal-governance",
        "workspace:runtime-governance",
        "profile:runtime-governance",
        "2026-06-12T00:00:00Z",
    );
    stable_activity_row_seeds()
        .into_iter()
        .map(|seed| {
            let queue_row = rows
                .iter()
                .find(|row| {
                    row.workload_class == seed.workload_class
                        && matches!(row.row_class, GovernanceRowClass::QueueIdentityAdmission)
                })
                .expect("stable packet must include a queue row per workload");
            let lane_state = scheduler_snapshot
                .lane_states
                .iter()
                .find(|lane| lane.lane == governor_lane_for(queue_row.queue_lane_class))
                .expect("stable packet must include a scheduler lane per queue lane");
            let mut ordered_job_identity_refs = queue_row
                .job_identities
                .iter()
                .map(|identity| identity.job_identity_ref.clone())
                .collect::<Vec<_>>();
            if let Some(index) = queue_row
                .job_identities
                .iter()
                .position(|identity| identity.job_kind == seed.primary_job_kind)
            {
                ordered_job_identity_refs.swap(0, index);
            }
            QueueSessionActivityJobRow {
                record_kind: QUEUE_SESSION_TERMINAL_ACTIVITY_JOB_ROW_RECORD_KIND.to_owned(),
                row_id: format!("activity:{}", seed.workload_class.as_str()),
                workload_class: seed.workload_class,
                primary_job_kind: seed.primary_job_kind,
                job_identity_refs: ordered_job_identity_refs,
                queue_lane_class: queue_row.queue_lane_class,
                state_class: seed.state_class,
                state_token: seed.state_class.as_str().to_owned(),
                state_reason: seed.state_reason.to_owned(),
                queue_age_seconds: lane_state.oldest_age_seconds.unwrap_or_default().round() as u64,
                queue_age_label: format_age_label(
                    lane_state
                        .oldest_age_seconds
                        .map(|seconds| seconds.round() as u64),
                ),
                retry_class: queue_row.retry_class,
                next_action_class: seed.next_action_class,
                next_action_ref: format!(
                    "action:{}:{}",
                    seed.workload_class.as_str(),
                    seed.next_action_class.as_str()
                ),
                exact_target_reopen_ref: format!(
                    "reopen:{}:{}",
                    seed.workload_class.as_str(),
                    seed.primary_job_kind.as_str()
                ),
                exact_target_identity_ref: seed.exact_target_identity_ref.to_owned(),
                inspect_ref: format!("inspect:{}", seed.workload_class.as_str()),
                last_checkpoint: lane_state.checkpoint.clone(),
            }
        })
        .collect()
}

fn build_scheduler_lane_rows(
    activity_job_rows: &[QueueSessionActivityJobRow],
) -> Vec<QueueSessionSchedulerLaneRow> {
    let scheduler_snapshot = seeded_resource_governor_snapshot(
        "resource-governor:snapshot:queue-session-terminal-governance",
        "workspace:runtime-governance",
        "profile:runtime-governance",
        "2026-06-12T00:00:00Z",
    );
    QueueLaneClass::REQUIRED
        .into_iter()
        .map(|queue_lane_class| {
            let lane_state = scheduler_snapshot
                .lane_states
                .iter()
                .find(|lane| lane.lane == governor_lane_for(queue_lane_class))
                .expect("stable packet must include a scheduler lane per queue lane");
            let lane_rows = activity_job_rows
                .iter()
                .filter(|row| row.queue_lane_class == queue_lane_class)
                .cloned()
                .collect::<Vec<_>>();
            let retry_state_class = retry_state_for_lane(&lane_rows);
            QueueSessionSchedulerLaneRow {
                record_kind: QUEUE_SESSION_TERMINAL_SCHEDULER_LANE_ROW_RECORD_KIND.to_owned(),
                queue_lane_class,
                queue_lane_token: queue_lane_class.as_str().to_owned(),
                queue_depth: lane_state.lane_depth,
                oldest_age_seconds: lane_state.oldest_age_seconds.map(|age| age.round() as u64),
                oldest_age_label: format_age_label(
                    lane_state.oldest_age_seconds.map(|age| age.round() as u64),
                ),
                collapse_count: lane_state.collapse_count,
                retry_state_class,
                retry_state_token: retry_state_class.as_str().to_owned(),
                last_checkpoint: lane_state.checkpoint.clone(),
                activity_row_refs: lane_rows.iter().map(|row| row.row_id.clone()).collect(),
                workload_classes: lane_rows.iter().map(|row| row.workload_class).collect(),
                inspect_ref: format!("inspect:scheduler:{}", queue_lane_class.as_str()),
            }
        })
        .collect()
}

fn stable_input() -> QueueSessionTerminalGovernancePacketInput {
    let covered_workloads = GovernedWorkloadClass::REQUIRED.to_vec();
    let workspace_id_ref = "workspace:runtime-governance";
    let rows = vec![
        quality_row(GovernedWorkloadClass::NotebookSession),
        queue_row(
            GovernedWorkloadClass::NotebookSession,
            QueueLaneClass::InteractiveBackground,
            CollapseKeyClass::WorkspaceSliceTarget,
            BudgetDomainClass::CpuMemoryDisk,
            CheckpointPolicyClass::ItemBoundary,
            RetryClass::LocalRetryBudget,
            CancellationClass::CheckpointThenCancel,
            vec![job_identity(
                GovernedJobKind::NotebookCellExecution,
                "job:notebook.cell_execution:active_run",
                workspace_id_ref,
                Some("slice:notebook:training"),
                workspace_scope("current_root", "scope:notebook:training_root"),
                QueueInitiatingSource::UserAction,
                "collapse:notebook.cell_execution:workspace:runtime-governance:slice:notebook:training",
                QueueCollapsePolicy::RestartAfterSupersede,
                QueueStalenessPolicy::RefreshOnResume,
                &[KNOWLEDGE_REFRESH_BUDGET_DOMAIN_REF],
                "rev:notebook:42",
                Some("manifest:notebook:training:v4"),
                Some("ctx:notebook:kernel:python311"),
                Some("policy:interactive:2026-06-12"),
            )],
        ),
        restore_row(
            GovernedWorkloadClass::NotebookSession,
            RestoreFidelityClass::CompatibleRestore,
            NoHiddenRerunClass::ExplicitRerunOnly,
        ),
        terminal_row(
            GovernedWorkloadClass::NotebookSession,
            TerminalBoundaryClass::Local,
            ClipboardPostureClass::LocalDirect,
        ),
        downgrade_row(
            GovernedWorkloadClass::NotebookSession,
            DowngradeRuleClass::AutoNarrowOnQueueMetadataStale,
        ),
        quality_row(GovernedWorkloadClass::DataQueryConsole),
        queue_row(
            GovernedWorkloadClass::DataQueryConsole,
            QueueLaneClass::Foreground,
            CollapseKeyClass::SessionSurfaceTarget,
            BudgetDomainClass::ProtectedInteractiveReserve,
            CheckpointPolicyClass::NoneDeclared,
            RetryClass::ManualRequeueOnly,
            CancellationClass::ImmediateAbortSafe,
            vec![job_identity(
                GovernedJobKind::DataRequestCollectionRun,
                "job:data.request_collection_run:active_request",
                workspace_id_ref,
                Some("slice:data:orders"),
                workspace_scope("named_workset", "scope:data:orders_collection"),
                QueueInitiatingSource::UserAction,
                "collapse:data.request_collection_run:workspace:runtime-governance:slice:data:orders",
                QueueCollapsePolicy::ReplaceSuperseded,
                QueueStalenessPolicy::RefreshOnResume,
                &[FOREGROUND_TASK_BUDGET_DOMAIN_REF],
                "rev:data:17",
                Some("manifest:data:orders:v2"),
                Some("ctx:data:http_client:v5"),
                Some("policy:data:api:2026-06-12"),
            )],
        ),
        restore_row(
            GovernedWorkloadClass::DataQueryConsole,
            RestoreFidelityClass::ExactRestore,
            NoHiddenRerunClass::MetadataOnlyResume,
        ),
        terminal_row(
            GovernedWorkloadClass::DataQueryConsole,
            TerminalBoundaryClass::Remote,
            ClipboardPostureClass::BracketedPasteReview,
        ),
        downgrade_row(
            GovernedWorkloadClass::DataQueryConsole,
            DowngradeRuleClass::AutoNarrowOnRestoreFidelityStale,
        ),
        quality_row(GovernedWorkloadClass::PipelineRun),
        queue_row(
            GovernedWorkloadClass::PipelineRun,
            QueueLaneClass::ProviderOverlay,
            CollapseKeyClass::ProviderRouteTarget,
            BudgetDomainClass::NetworkProviderQuota,
            CheckpointPolicyClass::ExplicitPhaseBoundary,
            RetryClass::ProviderRetryBudget,
            CancellationClass::CancelAfterPhase,
            vec![
                job_identity(
                    GovernedJobKind::PipelineLogPull,
                    "job:pipeline.log_pull:tail",
                    workspace_id_ref,
                    Some("slice:pipeline:deploy"),
                    workspace_scope("review_workspace", "scope:pipeline:deploy_run"),
                    QueueInitiatingSource::UserAction,
                    "collapse:pipeline.log_pull:workspace:runtime-governance:slice:pipeline:deploy",
                    QueueCollapsePolicy::ReplaceSuperseded,
                    QueueStalenessPolicy::RefreshOnResume,
                    &[PROVIDER_OVERLAY_BUDGET_DOMAIN_REF],
                    "rev:pipeline:31",
                    Some("manifest:pipeline:deploy:v6"),
                    Some("ctx:pipeline:runner:remote"),
                    Some("policy:pipeline:overlay:2026-06-12"),
                ),
                job_identity(
                    GovernedJobKind::PipelineArtifactPull,
                    "job:pipeline.artifact_pull:latest_bundle",
                    workspace_id_ref,
                    Some("slice:pipeline:deploy"),
                    workspace_scope("review_workspace", "scope:pipeline:deploy_artifacts"),
                    QueueInitiatingSource::UserAction,
                    "collapse:pipeline.artifact_pull:workspace:runtime-governance:slice:pipeline:deploy",
                    QueueCollapsePolicy::CoalesceStaleDuplicates,
                    QueueStalenessPolicy::ReQueueIfStillRelevant,
                    &[PROVIDER_OVERLAY_BUDGET_DOMAIN_REF],
                    "rev:pipeline:31",
                    Some("manifest:pipeline:deploy:v6"),
                    Some("ctx:pipeline:runner:remote"),
                    Some("policy:pipeline:overlay:2026-06-12"),
                ),
            ],
        ),
        restore_row(
            GovernedWorkloadClass::PipelineRun,
            RestoreFidelityClass::EvidenceOnly,
            NoHiddenRerunClass::BlockedUntilManualReview,
        ),
        terminal_row(
            GovernedWorkloadClass::PipelineRun,
            TerminalBoundaryClass::Managed,
            ClipboardPostureClass::MetadataOnlyExport,
        ),
        downgrade_row(
            GovernedWorkloadClass::PipelineRun,
            DowngradeRuleClass::AutoNarrowOnRetryBudgetExhausted,
        ),
        quality_row(GovernedWorkloadClass::PreviewRoute),
        queue_row(
            GovernedWorkloadClass::PreviewRoute,
            QueueLaneClass::Maintenance,
            CollapseKeyClass::WorkspaceSliceTarget,
            BudgetDomainClass::BatteryThermal,
            CheckpointPolicyClass::TimeBoundary,
            RetryClass::LocalRetryBudget,
            CancellationClass::CleanupThenCancel,
            vec![
                job_identity(
                    GovernedJobKind::PreviewDevServer,
                    "job:preview.dev_server:workspace",
                    workspace_id_ref,
                    Some("slice:preview:webapp"),
                    workspace_scope("named_workset", "scope:preview:webapp"),
                    QueueInitiatingSource::WorkspaceOpen,
                    "collapse:preview.dev_server:workspace:runtime-governance:slice:preview:webapp",
                    QueueCollapsePolicy::RestartAfterSupersede,
                    QueueStalenessPolicy::RefreshOnResume,
                    &[MAINTENANCE_BUDGET_DOMAIN_REF],
                    "rev:preview:24",
                    Some("manifest:preview:webapp:v3"),
                    Some("ctx:preview:node20"),
                    Some("policy:preview:local:2026-06-12"),
                ),
                job_identity(
                    GovernedJobKind::PreviewRouteRefresh,
                    "job:preview.route_refresh:workspace",
                    workspace_id_ref,
                    Some("slice:preview:webapp"),
                    workspace_scope("named_workset", "scope:preview:webapp_route"),
                    QueueInitiatingSource::FileChangeNotification,
                    "collapse:preview.route_refresh:workspace:runtime-governance:slice:preview:webapp",
                    QueueCollapsePolicy::CoalesceStaleDuplicates,
                    QueueStalenessPolicy::DropIfStale,
                    &[KNOWLEDGE_REFRESH_BUDGET_DOMAIN_REF],
                    "rev:preview:24",
                    Some("manifest:preview:webapp:v3"),
                    Some("ctx:preview:node20"),
                    Some("policy:preview:local:2026-06-12"),
                ),
            ],
        ),
        restore_row(
            GovernedWorkloadClass::PreviewRoute,
            RestoreFidelityClass::PlaceholderOnly,
            NoHiddenRerunClass::ReconnectReviewRequired,
        ),
        terminal_row(
            GovernedWorkloadClass::PreviewRoute,
            TerminalBoundaryClass::Container,
            ClipboardPostureClass::RemoteBridgeReview,
        ),
        downgrade_row(
            GovernedWorkloadClass::PreviewRoute,
            DowngradeRuleClass::AutoNarrowOnTerminalBoundaryStale,
        ),
        quality_row(GovernedWorkloadClass::ProfilerCapture),
        queue_row(
            GovernedWorkloadClass::ProfilerCapture,
            QueueLaneClass::Maintenance,
            CollapseKeyClass::SessionSurfaceTarget,
            BudgetDomainClass::DurableStorage,
            CheckpointPolicyClass::ExplicitPhaseBoundary,
            RetryClass::NoneDeclared,
            CancellationClass::RollbackThenCancel,
            vec![job_identity(
                GovernedJobKind::ProfilerCapture,
                "job:profiler.capture:active_trace",
                workspace_id_ref,
                Some("slice:profiler:startup"),
                workspace_scope("current_root", "scope:profiler:startup_hot_path"),
                QueueInitiatingSource::UserAction,
                "collapse:profiler.capture:workspace:runtime-governance:slice:profiler:startup",
                QueueCollapsePolicy::SerializeExactDuplicates,
                QueueStalenessPolicy::DropIfStale,
                &[MAINTENANCE_BUDGET_DOMAIN_REF],
                "rev:profiler:09",
                Some("manifest:profiler:startup:v1"),
                Some("ctx:profiler:local_sampler"),
                Some("policy:profiler:2026-06-12"),
            )],
        ),
        restore_row(
            GovernedWorkloadClass::ProfilerCapture,
            RestoreFidelityClass::EvidenceOnly,
            NoHiddenRerunClass::BlockedUntilManualReview,
        ),
        downgrade_row(
            GovernedWorkloadClass::ProfilerCapture,
            DowngradeRuleClass::AutoNarrowOnMissingCheckpointProof,
        ),
        quality_row(GovernedWorkloadClass::DocsRecall),
        queue_row(
            GovernedWorkloadClass::DocsRecall,
            QueueLaneClass::InteractiveBackground,
            CollapseKeyClass::WorkspaceSliceTarget,
            BudgetDomainClass::OptionalServiceQuota,
            CheckpointPolicyClass::TimeBoundary,
            RetryClass::LocalRetryBudget,
            CancellationClass::CheckpointThenCancel,
            vec![
                job_identity(
                    GovernedJobKind::DocsPackRefresh,
                    "job:docs.pack_refresh:workspace",
                    workspace_id_ref,
                    Some("slice:docs:rust"),
                    workspace_scope("named_workset", "scope:docs:rust_pack"),
                    QueueInitiatingSource::WorkspaceOpen,
                    "collapse:docs.pack_refresh:workspace:runtime-governance:slice:docs:rust",
                    QueueCollapsePolicy::ReplaceSuperseded,
                    QueueStalenessPolicy::RefreshOnResume,
                    &[MAINTENANCE_BUDGET_DOMAIN_REF, PROVIDER_OVERLAY_BUDGET_DOMAIN_REF],
                    "rev:docs:55",
                    Some("manifest:docs:rust:v8"),
                    Some("ctx:docs:browser"),
                    Some("policy:docs:catalog:2026-06-12"),
                ),
                job_identity(
                    GovernedJobKind::DocsRetrievalIndexRefresh,
                    "job:docs.retrieval_index_refresh:workspace",
                    workspace_id_ref,
                    Some("slice:docs:rust"),
                    workspace_scope("named_workset", "scope:docs:rust_retrieval"),
                    QueueInitiatingSource::SchedulerTimer,
                    "collapse:docs.retrieval_index_refresh:workspace:runtime-governance:slice:docs:rust",
                    QueueCollapsePolicy::CoalesceStaleDuplicates,
                    QueueStalenessPolicy::DropIfStale,
                    &[KNOWLEDGE_REFRESH_BUDGET_DOMAIN_REF],
                    "rev:docs:55",
                    Some("manifest:docs:rust:v8"),
                    Some("ctx:docs:browser"),
                    Some("policy:docs:catalog:2026-06-12"),
                ),
            ],
        ),
        restore_row(
            GovernedWorkloadClass::DocsRecall,
            RestoreFidelityClass::ExactRestore,
            NoHiddenRerunClass::LiveContinuityPreserved,
        ),
        downgrade_row(
            GovernedWorkloadClass::DocsRecall,
            DowngradeRuleClass::AutoNarrowOnQueueMetadataStale,
        ),
        quality_row(GovernedWorkloadClass::SyncOffboardingFlow),
        queue_row(
            GovernedWorkloadClass::SyncOffboardingFlow,
            QueueLaneClass::UploadReplication,
            CollapseKeyClass::ArtifactDestinationTarget,
            BudgetDomainClass::DurableStorage,
            CheckpointPolicyClass::ResumableChunkBoundary,
            RetryClass::ProviderRetryBudget,
            CancellationClass::CheckpointThenCancel,
            vec![
                job_identity(
                    GovernedJobKind::SyncProfileReplication,
                    "job:sync.profile_replication:workspace",
                    workspace_id_ref,
                    None,
                    workspace_scope("full_workspace", "scope:sync:profile_replication"),
                    QueueInitiatingSource::SyncTrigger,
                    "collapse:sync.profile_replication:workspace:runtime-governance",
                    QueueCollapsePolicy::SerializeExactDuplicates,
                    QueueStalenessPolicy::ReQueueIfStillRelevant,
                    &[REPLICATION_BUDGET_DOMAIN_REF],
                    "rev:sync:12",
                    Some("manifest:profile:runtime:v5"),
                    Some("ctx:sync:profile"),
                    Some("policy:sync:2026-06-12"),
                ),
                job_identity(
                    GovernedJobKind::SyncOffboardingExport,
                    "job:sync.offboarding_export:workspace",
                    workspace_id_ref,
                    None,
                    workspace_scope("full_workspace", "scope:sync:offboarding_export"),
                    QueueInitiatingSource::SupportExportRequest,
                    "collapse:sync.offboarding_export:workspace:runtime-governance",
                    QueueCollapsePolicy::SerializeExactDuplicates,
                    QueueStalenessPolicy::ReQueueIfStillRelevant,
                    &[REPLICATION_BUDGET_DOMAIN_REF],
                    "rev:sync:12",
                    Some("manifest:profile:runtime:v5"),
                    Some("ctx:sync:offboarding"),
                    Some("policy:sync:2026-06-12"),
                ),
            ],
        ),
        restore_row(
            GovernedWorkloadClass::SyncOffboardingFlow,
            RestoreFidelityClass::LayoutOnly,
            NoHiddenRerunClass::ExplicitRerunOnly,
        ),
        downgrade_row(
            GovernedWorkloadClass::SyncOffboardingFlow,
            DowngradeRuleClass::AutoNarrowOnRetryBudgetExhausted,
        ),
        quality_row(GovernedWorkloadClass::CompanionHandoff),
        queue_row(
            GovernedWorkloadClass::CompanionHandoff,
            QueueLaneClass::UploadReplication,
            CollapseKeyClass::HandoffSubject,
            BudgetDomainClass::NetworkProviderQuota,
            CheckpointPolicyClass::ResumableChunkBoundary,
            RetryClass::ReconnectAfterReauth,
            CancellationClass::CleanupThenCancel,
            vec![job_identity(
                GovernedJobKind::CompanionHandoffPackage,
                "job:companion.handoff_package:workspace",
                workspace_id_ref,
                Some("slice:companion:handoff"),
                workspace_scope("companion_surface", "scope:companion:handoff"),
                QueueInitiatingSource::UserAction,
                "collapse:companion.handoff_package:workspace:runtime-governance:slice:companion:handoff",
                QueueCollapsePolicy::ReplaceSuperseded,
                QueueStalenessPolicy::RefreshOnResume,
                &[REPLICATION_BUDGET_DOMAIN_REF, PROVIDER_OVERLAY_BUDGET_DOMAIN_REF],
                "rev:companion:07",
                Some("manifest:companion:handoff:v2"),
                Some("ctx:companion:remote"),
                Some("policy:companion:2026-06-12"),
            )],
        ),
        restore_row(
            GovernedWorkloadClass::CompanionHandoff,
            RestoreFidelityClass::CompatibleRestore,
            NoHiddenRerunClass::ReauthorizeBeforeResume,
        ),
        terminal_row(
            GovernedWorkloadClass::CompanionHandoff,
            TerminalBoundaryClass::Remote,
            ClipboardPostureClass::MetadataOnlyExport,
        ),
        downgrade_row(
            GovernedWorkloadClass::CompanionHandoff,
            DowngradeRuleClass::AutoNarrowOnRestoreFidelityStale,
        ),
        quality_row(GovernedWorkloadClass::IncidentWorkspace),
        queue_row(
            GovernedWorkloadClass::IncidentWorkspace,
            QueueLaneClass::ProviderOverlay,
            CollapseKeyClass::SessionSurfaceTarget,
            BudgetDomainClass::NetworkProviderQuota,
            CheckpointPolicyClass::ExplicitPhaseBoundary,
            RetryClass::ProviderRetryBudget,
            CancellationClass::CancelAfterPhase,
            vec![job_identity(
                GovernedJobKind::IncidentRecoveryWorkspaceRefresh,
                "job:incident.recovery_workspace_refresh:workspace",
                workspace_id_ref,
                Some("slice:incident:sev1"),
                workspace_scope("review_workspace", "scope:incident:sev1"),
                QueueInitiatingSource::RecoveryResume,
                "collapse:incident.recovery_workspace_refresh:workspace:runtime-governance:slice:incident:sev1",
                QueueCollapsePolicy::CoalesceStaleDuplicates,
                QueueStalenessPolicy::RefreshOnResume,
                &[PROVIDER_OVERLAY_BUDGET_DOMAIN_REF],
                "rev:incident:88",
                Some("manifest:incident:sev1:v3"),
                Some("ctx:incident:shared_control"),
                Some("policy:incident:2026-06-12"),
            )],
        ),
        restore_row(
            GovernedWorkloadClass::IncidentWorkspace,
            RestoreFidelityClass::TranscriptOnly,
            NoHiddenRerunClass::TranscriptPreservedNoRerun,
        ),
        terminal_row(
            GovernedWorkloadClass::IncidentWorkspace,
            TerminalBoundaryClass::SharedControl,
            ClipboardPostureClass::SharedControlGrantRequired,
        ),
        downgrade_row(
            GovernedWorkloadClass::IncidentWorkspace,
            DowngradeRuleClass::AutoNarrowOnTerminalBoundaryStale,
        ),
        quality_row(GovernedWorkloadClass::InfrastructureSession),
        queue_row(
            GovernedWorkloadClass::InfrastructureSession,
            QueueLaneClass::Foreground,
            CollapseKeyClass::SessionSurfaceTarget,
            BudgetDomainClass::ProtectedInteractiveReserve,
            CheckpointPolicyClass::ItemBoundary,
            RetryClass::ReconnectAfterReauth,
            CancellationClass::ImmediateAbortSafe,
            vec![job_identity(
                GovernedJobKind::InfrastructureOverlayProbe,
                "job:infrastructure.overlay_probe:workspace",
                workspace_id_ref,
                Some("slice:infra:cluster-a"),
                workspace_scope("policy_limited_view", "scope:infra:cluster-a"),
                QueueInitiatingSource::RemoteReconnect,
                "collapse:infrastructure.overlay_probe:workspace:runtime-governance:slice:infra:cluster-a",
                QueueCollapsePolicy::ReplaceSuperseded,
                QueueStalenessPolicy::RefreshOnResume,
                &[FOREGROUND_TASK_BUDGET_DOMAIN_REF, PROVIDER_OVERLAY_BUDGET_DOMAIN_REF],
                "rev:infra:63",
                Some("manifest:infra:cluster-a:v5"),
                Some("ctx:infra:ssh"),
                Some("policy:infra:2026-06-12"),
            )],
        ),
        restore_row(
            GovernedWorkloadClass::InfrastructureSession,
            RestoreFidelityClass::TranscriptOnly,
            NoHiddenRerunClass::ReconnectReviewRequired,
        ),
        terminal_row(
            GovernedWorkloadClass::InfrastructureSession,
            TerminalBoundaryClass::PolicyBlocked,
            ClipboardPostureClass::PolicyDeniedSafeAlternative,
        ),
        downgrade_row(
            GovernedWorkloadClass::InfrastructureSession,
            DowngradeRuleClass::AutoBlockOnMissingEvidence,
        ),
    ];
    let activity_job_rows = build_activity_job_rows(&rows);
    let scheduler_lane_rows = build_scheduler_lane_rows(&activity_job_rows);
    let consumer_projections = ConsumerSurface::REQUIRED
        .into_iter()
        .map(stable_projection)
        .collect();
    QueueSessionTerminalGovernancePacketInput {
        packet_id: "packet:queue-session-terminal-governance".to_owned(),
        workflow_or_surface_id: "runtime.queue_session_terminal_governance".to_owned(),
        generated_at: "2026-06-12T00:00:00Z".to_owned(),
        covered_workloads,
        rows,
        activity_job_rows,
        scheduler_lane_rows,
        consumer_projections,
        source_contract_refs: vec![
            QUEUE_SESSION_TERMINAL_GOVERNANCE_SCHEMA_REF.to_owned(),
            QUEUE_SESSION_TERMINAL_GOVERNANCE_DOC_REF.to_owned(),
            QUEUE_SESSION_TERMINAL_GOVERNANCE_ARTIFACT_DOC_REF.to_owned(),
            BACKGROUND_QUEUE_CONTRACT_DOC_REF.to_owned(),
            CONTEXT_CACHE_TERMINAL_RESTORE_CONTRACT_DOC_REF.to_owned(),
        ],
    }
}

/// Returns the checked-in stable governance packet.
pub fn current_queue_session_terminal_governance_packet() -> QueueSessionTerminalGovernancePacket {
    QueueSessionTerminalGovernancePacket::materialize(stable_input())
}
