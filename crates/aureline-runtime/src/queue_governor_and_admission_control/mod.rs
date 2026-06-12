//! Stable queue-governor and admission-control truth packet.
//!
//! This module pins the M4 stable contract for background-job identity,
//! queue-lane vocabulary, collapse/checkpoint policy, runtime health, and
//! support-export parity. It does not implement the worker scheduler; it
//! defines the metadata-only packet that scheduler, shell, diagnostics,
//! CLI/headless inspection, and support exports project through.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::resource_governor::{
    GovernorHealthState, GovernorWorkClass, PressureDimension, ProtectedForegroundAction, QueueLane,
};

/// Stable record-kind tag for [`QueueGovernorStablePacket`].
pub const QUEUE_GOVERNOR_STABLE_PACKET_RECORD_KIND: &str =
    "queue_governor_and_admission_control_stable_packet";

/// Stable record-kind tag for [`QueueGovernorSupportExport`].
pub const QUEUE_GOVERNOR_SUPPORT_EXPORT_RECORD_KIND: &str =
    "queue_governor_and_admission_control_support_export";

/// Integer schema version for stable queue-governor records.
pub const QUEUE_GOVERNOR_SCHEMA_VERSION: u32 = 1;

/// Repo-relative boundary schema reference.
pub const QUEUE_GOVERNOR_SCHEMA_REF: &str =
    "schemas/runtime/queue-governor-and-admission-control.schema.json";

/// Repo-relative stable runtime contract document.
pub const QUEUE_GOVERNOR_DOC_REF: &str = "docs/runtime/m4/queue-governor-and-admission-control.md";

/// Repo-relative reviewer artifact.
pub const QUEUE_GOVERNOR_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/queue-governor-and-admission-control.md";

/// Repo-relative fixture directory.
pub const QUEUE_GOVERNOR_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/queue-governor-and-admission-control";

/// Repo-relative checked-in packet reference.
pub const QUEUE_GOVERNOR_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/queue_governor_and_admission_control_packet.json";

/// Canonical background job kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackgroundJobKind {
    /// Hot-set or visible-symbol indexing.
    KnowledgeHotSetScan,
    /// Full workspace index or graph rebuild.
    KnowledgeWorkspaceIndex,
    /// Provider overlay refresh.
    ProviderOverlayRefresh,
    /// AI context expansion or embedding refresh.
    AiContextExpansion,
    /// Extension timer or hidden preview refresh.
    ExtensionTimer,
    /// Upload, sync, telemetry, or support-bundle transfer.
    UploadReplication,
}

impl BackgroundJobKind {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KnowledgeHotSetScan => "knowledge.hot_set_scan",
            Self::KnowledgeWorkspaceIndex => "knowledge.workspace_index",
            Self::ProviderOverlayRefresh => "provider.overlay_refresh",
            Self::AiContextExpansion => "ai.context_expansion",
            Self::ExtensionTimer => "extension.timer",
            Self::UploadReplication => "upload.replication",
        }
    }
}

/// Collapse behavior for duplicate or superseded background work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollapsePolicy {
    /// Keep one in-flight job and fold duplicate arrivals into its next output.
    CoalesceStaleDuplicates,
    /// Newest arrival wins; the superseded job cancels at a safe point.
    ReplaceSuperseded,
    /// Newest arrival restarts from the last-good checkpoint.
    RestartAfterSupersede,
    /// Suppress exact duplicate enqueue only.
    SerializeExactDuplicates,
}

impl CollapsePolicy {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoalesceStaleDuplicates => "coalesce_stale_duplicates",
            Self::ReplaceSuperseded => "replace_superseded",
            Self::RestartAfterSupersede => "restart_after_supersede",
            Self::SerializeExactDuplicates => "serialize_exact_duplicates",
        }
    }
}

/// Checkpoint behavior claimed by a background job.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointPolicy {
    /// Resume at explicit phase boundaries.
    ExplicitPhaseBoundary,
    /// Resume by processed item or declared time interval.
    ItemOrTimeBoundary,
    /// Resume at upload or replication chunk boundaries.
    ResumableChunkBoundary,
}

impl CheckpointPolicy {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitPhaseBoundary => "explicit_phase_boundary",
            Self::ItemOrTimeBoundary => "item_or_time_boundary",
            Self::ResumableChunkBoundary => "resumable_chunk_boundary",
        }
    }
}

/// Staleness behavior when queued work reaches the front of a lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StalenessPolicy {
    /// Drop the job when its identity inputs changed.
    DropIfStale,
    /// Refresh inputs on resume and continue only if still relevant.
    RefreshOnResume,
    /// Re-queue the job if the target still needs work.
    ReQueueIfStillRelevant,
}

impl StalenessPolicy {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DropIfStale => "drop_if_stale",
            Self::RefreshOnResume => "refresh_on_resume",
            Self::ReQueueIfStillRelevant => "re_queue_if_still_relevant",
        }
    }
}

/// Cancellation behavior promised by a background job.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CancellationContract {
    /// Cancel immediately without durable cleanup.
    CancelImmediate,
    /// Write a checkpoint before cancellation completes.
    CheckpointThenCancel,
    /// Cancel after the current phase boundary.
    CancelAfterPhase,
}

impl CancellationContract {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CancelImmediate => "cancel_immediate",
            Self::CheckpointThenCancel => "checkpoint_then_cancel",
            Self::CancelAfterPhase => "cancel_after_phase",
        }
    }
}

/// Stable source that initiated a background job.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InitiatingSource {
    /// User foreground action.
    UserAction,
    /// Session startup or crash recovery startup.
    SessionStartup,
    /// Workspace open or restore.
    WorkspaceOpen,
    /// Profile selection or profile-layer change.
    ProfileChange,
    /// Policy epoch or policy-bundle change.
    PolicyChange,
    /// Focus or visible-workset change.
    FocusChange,
    /// File watcher notification.
    FileChangeNotification,
    /// Remote reconnect.
    RemoteReconnect,
    /// Scheduler timer.
    SchedulerTimer,
    /// Extension host request.
    ExtensionRequest,
    /// AI tool request.
    AiToolCall,
    /// Support export or support-bundle request.
    SupportExportRequest,
    /// Sync trigger.
    SyncTrigger,
    /// Resume from a prior recovery or checkpoint.
    RecoveryResume,
}

impl InitiatingSource {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserAction => "user_action",
            Self::SessionStartup => "session_startup",
            Self::WorkspaceOpen => "workspace_open",
            Self::ProfileChange => "profile_change",
            Self::PolicyChange => "policy_change",
            Self::FocusChange => "focus_change",
            Self::FileChangeNotification => "file_change_notification",
            Self::RemoteReconnect => "remote_reconnect",
            Self::SchedulerTimer => "scheduler_timer",
            Self::ExtensionRequest => "extension_request",
            Self::AiToolCall => "ai_tool_call",
            Self::SupportExportRequest => "support_export_request",
            Self::SyncTrigger => "sync_trigger",
            Self::RecoveryResume => "recovery_resume",
        }
    }
}

/// Scope descriptor for a background job.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueJobScope {
    /// Scope class such as `current_root`, `sparse_slice`, or `ambient`.
    pub scope_class: String,
    /// Opaque scope reference safe for diagnostics and support export.
    pub scope_ref: String,
}

/// Stable background-job object projected by the governor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableBackgroundJob {
    /// Opaque job id safe for logs and exports.
    pub job_id: String,
    /// Canonical job kind.
    pub job_kind: BackgroundJobKind,
    /// Workspace id that owns the job.
    pub workspace_id: String,
    /// Optional workset, slice, or review-workspace id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slice_id: Option<String>,
    /// Root or target scope.
    pub scope: QueueJobScope,
    /// Source that initiated the work.
    pub initiating_source: InitiatingSource,
    /// Structured duplicate/supersede key.
    pub collapse_key: String,
    /// Queue lane.
    pub lane: QueueLane,
    /// Budget domains consumed by the job.
    pub budget_domains: Vec<String>,
    /// Collapse policy.
    pub collapse_policy: CollapsePolicy,
    /// Checkpoint policy.
    pub checkpoint_policy: CheckpointPolicy,
    /// Staleness policy.
    pub staleness_policy: StalenessPolicy,
    /// Cancellation contract.
    pub cancellation_contract: CancellationContract,
    /// Workspace revision used to self-invalidate stale jobs.
    pub workspace_revision: String,
    /// Workset or slice manifest hash used to self-invalidate stale jobs.
    pub manifest_hash: String,
    /// Execution-context hash used to self-invalidate stale jobs.
    pub execution_context_hash: String,
    /// Policy epoch used to self-invalidate stale jobs.
    pub policy_epoch: String,
}

impl StableBackgroundJob {
    /// Returns true when this job is stale against current identity inputs.
    pub fn is_stale_against(&self, current: &StalenessInputs) -> bool {
        self.workspace_revision != current.workspace_revision
            || self.manifest_hash != current.manifest_hash
            || self.execution_context_hash != current.execution_context_hash
            || self.policy_epoch != current.policy_epoch
    }

    /// Returns true when the job consumes the reserved hot-path budget.
    pub fn borrows_hot_path_budget(&self) -> bool {
        self.budget_domains
            .iter()
            .any(|domain| domain == "hot_path_interactive_budget")
    }
}

/// Current identity inputs used to invalidate stale queued work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StalenessInputs {
    /// Current workspace revision.
    pub workspace_revision: String,
    /// Current workset or slice manifest hash.
    pub manifest_hash: String,
    /// Current execution-context hash.
    pub execution_context_hash: String,
    /// Current policy epoch.
    pub policy_epoch: String,
}

/// Per-lane collapse, retry, checkpoint, and starvation rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueLaneRule {
    /// Queue lane.
    pub lane: QueueLane,
    /// Primary work class.
    pub work_class: GovernorWorkClass,
    /// Budget domain reserved for this lane.
    pub budget_domain: String,
    /// Collapse policy applied to duplicate work.
    pub collapse_policy: CollapsePolicy,
    /// Retry budget name, separate for provider and replication lanes.
    pub retry_budget: String,
    /// Starvation protection rule.
    pub starvation_rule: String,
    /// True if this lane is forbidden from reserved interactive capacity.
    pub forbids_interactive_capacity_borrow: bool,
}

/// Lane summary projected to shell, diagnostics, and support export.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueueLaneSummary {
    /// Queue lane.
    pub lane: QueueLane,
    /// Queue depth.
    pub queue_depth: u32,
    /// Oldest job age in seconds.
    pub oldest_age_seconds: u32,
    /// Number of collapsed jobs.
    pub collapse_count: u32,
    /// Last checkpoint reference or boundary.
    pub last_checkpoint: String,
    /// Shed work label.
    pub shed_work: String,
    /// Protected data class.
    pub protected_data_class: String,
    /// Pause reason shared by every projection.
    pub pause_reason: String,
    /// Resume owner shared by every projection.
    pub resume_owner: String,
    /// Resume condition shared by every projection.
    pub resume_condition: String,
}

/// Health state projection shared across runtime surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeHealthProjection {
    /// Current governor health state.
    pub state: GovernorHealthState,
    /// Shared pause reason.
    pub pause_reason: String,
    /// Shared resume owner.
    pub resume_owner: String,
    /// Affected lanes.
    pub affected_lanes: Vec<QueueLane>,
    /// Protected foreground path.
    pub protected_foreground: Vec<ProtectedForegroundAction>,
}

/// Consumer projection for shell, diagnostics, CLI, and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueGovernorConsumerProjection {
    /// Consumer surface id.
    pub surface_id: String,
    /// Projected packet id.
    pub packet_id: String,
    /// Shared pause reason.
    pub pause_reason: String,
    /// Shared resume owner.
    pub resume_owner: String,
    /// Affected lane tokens.
    pub affected_lane_tokens: Vec<String>,
    /// True when no raw user content is exposed.
    pub excludes_user_content_by_default: bool,
}

/// Fixture lab proving a pressure transition and protected foreground path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueueGovernorLab {
    /// Lab id.
    pub lab_id: String,
    /// Trigger pressure dimension.
    pub pressure_dimension: PressureDimension,
    /// Expected governor state.
    pub expected_state: GovernorHealthState,
    /// Protected action exercised during the lab.
    pub protected_action: ProtectedForegroundAction,
    /// Paused or coalesced lane.
    pub affected_lane: QueueLane,
}

/// Metadata-only support export for queue-governor state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueueGovernorSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Export id.
    pub export_id: String,
    /// Packet id.
    pub packet_id: String,
    /// Runtime health state.
    pub health: RuntimeHealthProjection,
    /// Lane summaries.
    pub lane_summaries: Vec<QueueLaneSummary>,
    /// True when raw user content is excluded.
    pub raw_private_material_excluded: bool,
}

impl QueueGovernorSupportExport {
    /// Returns a compact plaintext rendering for support bundles.
    pub fn render_plaintext(&self) -> String {
        let mut lines = vec![
            format!("Queue governor state: {}", self.health.state.label()),
            format!("Pause reason: {}", self.health.pause_reason),
            format!("Resume owner: {}", self.health.resume_owner),
        ];
        for lane in &self.lane_summaries {
            lines.push(format!(
                "{} depth={} oldest={}s collapsed={} checkpoint={} shed={} resume={}",
                lane.lane.label(),
                lane.queue_depth,
                lane.oldest_age_seconds,
                lane.collapse_count,
                lane.last_checkpoint,
                lane.shed_work,
                lane.resume_condition
            ));
        }
        lines.join("\n")
    }
}

/// Stable queue-governor packet for M4 evidence and supportability.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueueGovernorStablePacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Packet id.
    pub packet_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Runtime health projection.
    pub health: RuntimeHealthProjection,
    /// Stable background jobs.
    pub jobs: Vec<StableBackgroundJob>,
    /// Queue-lane rules.
    pub lane_rules: Vec<QueueLaneRule>,
    /// Lane summaries.
    pub lane_summaries: Vec<QueueLaneSummary>,
    /// Lab fixtures covered by the packet.
    pub labs: Vec<QueueGovernorLab>,
    /// Consumer projections.
    pub consumer_projections: Vec<QueueGovernorConsumerProjection>,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

impl QueueGovernorStablePacket {
    /// Materializes the canonical stable packet.
    pub fn stable_example() -> Self {
        let packet_id = "packet:m4:queue_governor_and_admission_control:stable".to_owned();
        let health = RuntimeHealthProjection {
            state: GovernorHealthState::ProtectCore,
            pause_reason: "battery saver and thermal pressure are protecting typing, save, navigation, and quick open".to_owned(),
            resume_owner: "resource_governor".to_owned(),
            affected_lanes: vec![
                QueueLane::InteractiveBackground,
                QueueLane::Maintenance,
                QueueLane::ProviderOverlay,
                QueueLane::UploadReplication,
            ],
            protected_foreground: ProtectedForegroundAction::ALL.to_vec(),
        };
        let summaries = vec![
            lane_summary(QueueLane::Foreground, 0, 0, 0, "none", "none", "active"),
            lane_summary(
                QueueLane::InteractiveBackground,
                2,
                42,
                3,
                "checkpoint:hot-set:visible-symbols",
                "hot-set scan narrowed",
                "pressure clears or user opens stale symbol result",
            ),
            lane_summary(
                QueueLane::Maintenance,
                7,
                310,
                11,
                "checkpoint:index:phase-imports",
                "AI context expansion and full index paused",
                "governor enters recovery",
            ),
            lane_summary(
                QueueLane::ProviderOverlay,
                1,
                185,
                4,
                "checkpoint:provider:quota-window",
                "provider overlay refresh paused",
                "provider quota circuit closes",
            ),
            lane_summary(
                QueueLane::UploadReplication,
                3,
                600,
                2,
                "checkpoint:upload:chunk-19",
                "support upload and replication deferred",
                "AC power or explicit foreground upload",
            ),
        ];
        let affected_tokens = health
            .affected_lanes
            .iter()
            .map(|lane| lane.as_str().to_owned())
            .collect::<Vec<_>>();
        Self {
            record_kind: QUEUE_GOVERNOR_STABLE_PACKET_RECORD_KIND.to_owned(),
            schema_version: QUEUE_GOVERNOR_SCHEMA_VERSION,
            packet_id: packet_id.clone(),
            generated_at: "2026-06-04T19:00:00Z".to_owned(),
            health,
            jobs: stable_jobs(),
            lane_rules: lane_rules(),
            lane_summaries: summaries,
            labs: lab_cases(),
            consumer_projections: [
                "shell_status",
                "activity_center",
                "diagnostics",
                "cli_headless",
                "support_export",
            ]
            .into_iter()
            .map(|surface_id| QueueGovernorConsumerProjection {
                surface_id: surface_id.to_owned(),
                packet_id: packet_id.clone(),
                pause_reason: "battery saver and thermal pressure are protecting typing, save, navigation, and quick open".to_owned(),
                resume_owner: "resource_governor".to_owned(),
                affected_lane_tokens: affected_tokens.clone(),
                excludes_user_content_by_default: true,
            })
            .collect(),
            raw_private_material_excluded: true,
        }
    }

    /// Validates stable queue-governor invariants.
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut findings = Vec::new();
        if self.record_kind != QUEUE_GOVERNOR_STABLE_PACKET_RECORD_KIND {
            findings.push("record_kind_mismatch".to_owned());
        }
        if self.schema_version != QUEUE_GOVERNOR_SCHEMA_VERSION {
            findings.push("schema_version_mismatch".to_owned());
        }
        if !self.raw_private_material_excluded {
            findings.push("raw_private_material_not_excluded".to_owned());
        }
        let rule_lanes = self
            .lane_rules
            .iter()
            .map(|rule| rule.lane)
            .collect::<BTreeSet<_>>();
        for lane in QueueLane::ALL {
            if !rule_lanes.contains(&lane) {
                findings.push(format!("missing_lane_rule:{}", lane.as_str()));
            }
        }
        if self
            .jobs
            .iter()
            .any(StableBackgroundJob::borrows_hot_path_budget)
        {
            findings.push("background_job_borrows_hot_path_budget".to_owned());
        }
        let mut collapse_counts = BTreeMap::<&str, u32>::new();
        for job in &self.jobs {
            *collapse_counts
                .entry(job.collapse_key.as_str())
                .or_default() += 1;
        }
        if collapse_counts.values().all(|count| *count == 1) {
            findings.push("duplicate_collapse_not_evidenced".to_owned());
        }
        if !self
            .jobs
            .iter()
            .any(|job| job.checkpoint_policy == CheckpointPolicy::ResumableChunkBoundary)
        {
            findings.push("checkpoint_resume_not_evidenced".to_owned());
        }
        let retry_budgets = self
            .lane_rules
            .iter()
            .map(|rule| rule.retry_budget.as_str())
            .collect::<BTreeSet<_>>();
        if !retry_budgets.contains("provider_overlay_retry_budget")
            || !retry_budgets.contains("replication_retry_budget")
        {
            findings.push("retry_budget_separation_missing".to_owned());
        }
        if self.lane_rules.iter().any(|rule| {
            rule.lane != QueueLane::Foreground && !rule.forbids_interactive_capacity_borrow
        }) {
            findings.push("interactive_capacity_borrow_allowed".to_owned());
        }
        let protected = self
            .health
            .protected_foreground
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        for action in ProtectedForegroundAction::ALL {
            if !protected.contains(&action) {
                findings.push(format!("missing_protected_action:{}", action.as_str()));
            }
        }
        for projection in &self.consumer_projections {
            if projection.packet_id != self.packet_id
                || projection.pause_reason != self.health.pause_reason
                || projection.resume_owner != self.health.resume_owner
                || !projection.excludes_user_content_by_default
            {
                findings.push(format!("projection_drift:{}", projection.surface_id));
            }
        }
        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }

    /// Builds the support export projection.
    pub fn support_export(&self, export_id: impl Into<String>) -> QueueGovernorSupportExport {
        QueueGovernorSupportExport {
            record_kind: QUEUE_GOVERNOR_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: self.schema_version,
            export_id: export_id.into(),
            packet_id: self.packet_id.clone(),
            health: self.health.clone(),
            lane_summaries: self.lane_summaries.clone(),
            raw_private_material_excluded: self.raw_private_material_excluded,
        }
    }
}

fn lane_summary(
    lane: QueueLane,
    queue_depth: u32,
    oldest_age_seconds: u32,
    collapse_count: u32,
    last_checkpoint: &str,
    shed_work: &str,
    resume_condition: &str,
) -> QueueLaneSummary {
    QueueLaneSummary {
        lane,
        queue_depth,
        oldest_age_seconds,
        collapse_count,
        last_checkpoint: last_checkpoint.to_owned(),
        shed_work: shed_work.to_owned(),
        protected_data_class: "metadata_only_no_user_content".to_owned(),
        pause_reason: "battery saver and thermal pressure are protecting typing, save, navigation, and quick open".to_owned(),
        resume_owner: "resource_governor".to_owned(),
        resume_condition: resume_condition.to_owned(),
    }
}

fn stable_jobs() -> Vec<StableBackgroundJob> {
    vec![
        stable_job(
            "job:hot-set:001",
            BackgroundJobKind::KnowledgeHotSetScan,
            QueueLane::InteractiveBackground,
            "knowledge_refresh_budget",
            CollapsePolicy::CoalesceStaleDuplicates,
            CheckpointPolicy::ItemOrTimeBoundary,
            StalenessPolicy::RefreshOnResume,
            CancellationContract::CheckpointThenCancel,
            "collapse:knowledge.hot_set_scan:workspace:fixture:root",
            InitiatingSource::FileChangeNotification,
        ),
        stable_job(
            "job:hot-set:002",
            BackgroundJobKind::KnowledgeHotSetScan,
            QueueLane::InteractiveBackground,
            "knowledge_refresh_budget",
            CollapsePolicy::CoalesceStaleDuplicates,
            CheckpointPolicy::ItemOrTimeBoundary,
            StalenessPolicy::RefreshOnResume,
            CancellationContract::CheckpointThenCancel,
            "collapse:knowledge.hot_set_scan:workspace:fixture:root",
            InitiatingSource::FileChangeNotification,
        ),
        stable_job(
            "job:index:001",
            BackgroundJobKind::KnowledgeWorkspaceIndex,
            QueueLane::Maintenance,
            "knowledge_refresh_budget",
            CollapsePolicy::RestartAfterSupersede,
            CheckpointPolicy::ExplicitPhaseBoundary,
            StalenessPolicy::DropIfStale,
            CancellationContract::CancelAfterPhase,
            "collapse:knowledge.workspace_index:workspace:fixture",
            InitiatingSource::WorkspaceOpen,
        ),
        stable_job(
            "job:provider:001",
            BackgroundJobKind::ProviderOverlayRefresh,
            QueueLane::ProviderOverlay,
            "provider_overlay_budget",
            CollapsePolicy::ReplaceSuperseded,
            CheckpointPolicy::ExplicitPhaseBoundary,
            StalenessPolicy::RefreshOnResume,
            CancellationContract::CheckpointThenCancel,
            "collapse:provider.overlay_refresh:workspace:fixture",
            InitiatingSource::RemoteReconnect,
        ),
        stable_job(
            "job:ai:001",
            BackgroundJobKind::AiContextExpansion,
            QueueLane::Maintenance,
            "maintenance_budget",
            CollapsePolicy::CoalesceStaleDuplicates,
            CheckpointPolicy::ItemOrTimeBoundary,
            StalenessPolicy::DropIfStale,
            CancellationContract::CancelImmediate,
            "collapse:ai.context_expansion:workspace:fixture",
            InitiatingSource::AiToolCall,
        ),
        stable_job(
            "job:upload:001",
            BackgroundJobKind::UploadReplication,
            QueueLane::UploadReplication,
            "replication_budget",
            CollapsePolicy::SerializeExactDuplicates,
            CheckpointPolicy::ResumableChunkBoundary,
            StalenessPolicy::ReQueueIfStillRelevant,
            CancellationContract::CheckpointThenCancel,
            "collapse:upload.replication:workspace:fixture:support",
            InitiatingSource::SyncTrigger,
        ),
    ]
}

fn stable_job(
    job_id: &str,
    job_kind: BackgroundJobKind,
    lane: QueueLane,
    budget_domain: &str,
    collapse_policy: CollapsePolicy,
    checkpoint_policy: CheckpointPolicy,
    staleness_policy: StalenessPolicy,
    cancellation_contract: CancellationContract,
    collapse_key: &str,
    initiating_source: InitiatingSource,
) -> StableBackgroundJob {
    StableBackgroundJob {
        job_id: job_id.to_owned(),
        job_kind,
        workspace_id: "workspace.queue-governor.fixture".to_owned(),
        slice_id: None,
        scope: QueueJobScope {
            scope_class: "current_root".to_owned(),
            scope_ref: "scope:workspace-root".to_owned(),
        },
        initiating_source,
        collapse_key: collapse_key.to_owned(),
        lane,
        budget_domains: vec![budget_domain.to_owned()],
        collapse_policy,
        checkpoint_policy,
        staleness_policy,
        cancellation_contract,
        workspace_revision: "rev:42".to_owned(),
        manifest_hash: "manifest:hotset:v7".to_owned(),
        execution_context_hash: "ctx:stable:v3".to_owned(),
        policy_epoch: "policy:2026-06-04".to_owned(),
    }
}

fn lane_rules() -> Vec<QueueLaneRule> {
    vec![
        QueueLaneRule {
            lane: QueueLane::Foreground,
            work_class: GovernorWorkClass::ShortForegroundTask,
            budget_domain: "foreground_task_budget".to_owned(),
            collapse_policy: CollapsePolicy::SerializeExactDuplicates,
            retry_budget: "foreground_retry_budget".to_owned(),
            starvation_rule: "reserved foreground work is admitted before background work"
                .to_owned(),
            forbids_interactive_capacity_borrow: false,
        },
        QueueLaneRule {
            lane: QueueLane::InteractiveBackground,
            work_class: GovernorWorkClass::BackgroundKnowledgeWork,
            budget_domain: "knowledge_refresh_budget".to_owned(),
            collapse_policy: CollapsePolicy::CoalesceStaleDuplicates,
            retry_budget: "knowledge_refresh_retry_budget".to_owned(),
            starvation_rule:
                "progresses during nominal/recovery without consuming hot-path capacity".to_owned(),
            forbids_interactive_capacity_borrow: true,
        },
        QueueLaneRule {
            lane: QueueLane::Maintenance,
            work_class: GovernorWorkClass::OptionalAssistance,
            budget_domain: "maintenance_budget".to_owned(),
            collapse_policy: CollapsePolicy::RestartAfterSupersede,
            retry_budget: "maintenance_retry_budget".to_owned(),
            starvation_rule: "runs opportunistically after visible work has budget".to_owned(),
            forbids_interactive_capacity_borrow: true,
        },
        QueueLaneRule {
            lane: QueueLane::ProviderOverlay,
            work_class: GovernorWorkClass::BackgroundKnowledgeWork,
            budget_domain: "provider_overlay_budget".to_owned(),
            collapse_policy: CollapsePolicy::ReplaceSuperseded,
            retry_budget: "provider_overlay_retry_budget".to_owned(),
            starvation_rule: "separate circuit breaker prevents overlay starvation of local work"
                .to_owned(),
            forbids_interactive_capacity_borrow: true,
        },
        QueueLaneRule {
            lane: QueueLane::UploadReplication,
            work_class: GovernorWorkClass::UploadAndReplication,
            budget_domain: "replication_budget".to_owned(),
            collapse_policy: CollapsePolicy::SerializeExactDuplicates,
            retry_budget: "replication_retry_budget".to_owned(),
            starvation_rule: "chunked progress resumes only after core paths remain healthy"
                .to_owned(),
            forbids_interactive_capacity_borrow: true,
        },
    ]
}

fn lab_cases() -> Vec<QueueGovernorLab> {
    vec![
        lab(
            "low_memory",
            PressureDimension::Memory,
            QueueLane::Maintenance,
        ),
        lab(
            "low_disk",
            PressureDimension::Disk,
            QueueLane::UploadReplication,
        ),
        lab(
            "battery_saver",
            PressureDimension::BatteryThermal,
            QueueLane::Maintenance,
        ),
        lab(
            "thermal_pressure",
            PressureDimension::BatteryThermal,
            QueueLane::InteractiveBackground,
        ),
        lab(
            "provider_quota_exhaustion",
            PressureDimension::OptionalServiceQuota,
            QueueLane::ProviderOverlay,
        ),
        lab(
            "offline_transition",
            PressureDimension::Network,
            QueueLane::ProviderOverlay,
        ),
    ]
}

fn lab(
    lab_id: &str,
    pressure_dimension: PressureDimension,
    affected_lane: QueueLane,
) -> QueueGovernorLab {
    QueueGovernorLab {
        lab_id: lab_id.to_owned(),
        pressure_dimension,
        expected_state: GovernorHealthState::ProtectCore,
        protected_action: ProtectedForegroundAction::Save,
        affected_lane,
    }
}

/// Returns the canonical stable queue-governor packet.
pub fn current_stable_queue_governor_packet() -> QueueGovernorStablePacket {
    QueueGovernorStablePacket::stable_example()
}
