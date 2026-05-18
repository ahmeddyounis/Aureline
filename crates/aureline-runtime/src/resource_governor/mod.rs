//! Resource-governor, queue-lane, and admission-control truth records.
//!
//! This module does not implement a scheduler. It defines the beta runtime
//! record that the scheduler, shell, status surfaces, diagnostics, and support
//! export all project through when pressure narrows background work. The record
//! preserves the shared governor vocabulary, the five queue lanes, collapse and
//! checkpoint metadata, override explanations, and the protected foreground
//! actions that must keep priority.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Schema version stamped on resource-governor truth records.
pub const RESOURCE_GOVERNOR_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ResourceGovernorSnapshot`].
pub const RESOURCE_GOVERNOR_SNAPSHOT_RECORD_KIND: &str = "resource_governor_snapshot";

/// Stable record-kind tag for [`QueueLaneState`].
pub const QUEUE_LANE_STATE_RECORD_KIND: &str = "queue_lane_state";

/// Stable record-kind tag for [`ResourceGovernorSupportExport`].
pub const RESOURCE_GOVERNOR_SUPPORT_EXPORT_RECORD_KIND: &str = "resource_governor_support_export";

/// Resource dimensions sampled by the shared governor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PressureDimension {
    /// CPU saturation, runnable worker share, or hidden background CPU pressure.
    Cpu,
    /// Memory soft-cap or hard-cap pressure.
    Memory,
    /// Disk pressure, low-disk floors, save flush pressure, or derived-cache caps.
    Disk,
    /// Battery saver, low battery, critical battery, or thermal pressure.
    BatteryThermal,
    /// Network RTT, disconnect, or error-rate pressure.
    Network,
    /// Optional service quota, latency, circuit-breaker, or provider retry pressure.
    OptionalServiceQuota,
}

impl PressureDimension {
    /// All pressure dimensions required by the beta truth packet.
    pub const ALL: [Self; 6] = [
        Self::Cpu,
        Self::Memory,
        Self::Disk,
        Self::BatteryThermal,
        Self::Network,
        Self::OptionalServiceQuota,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cpu => "cpu",
            Self::Memory => "memory",
            Self::Disk => "disk",
            Self::BatteryThermal => "battery_thermal",
            Self::Network => "network",
            Self::OptionalServiceQuota => "optional_service_quota",
        }
    }

    /// Human-readable label for shell and diagnostic surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Cpu => "CPU",
            Self::Memory => "Memory",
            Self::Disk => "Disk",
            Self::BatteryThermal => "Battery / thermal",
            Self::Network => "Network",
            Self::OptionalServiceQuota => "Optional-service quota",
        }
    }
}

/// Runtime health state owned by the shared resource governor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernorHealthState {
    /// No sustained budget violation.
    Nominal,
    /// Brief misses or rising queue pressure; duplicates should collapse.
    Constrained,
    /// Persistent pressure; optional and maintenance work narrows or pauses.
    Degraded,
    /// Core interaction is protected by pausing or denying non-core work.
    ProtectCore,
    /// Pressure cleared; deferred work resumes with bounded ramp-up.
    Recovery,
}

impl GovernorHealthState {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Nominal => "nominal",
            Self::Constrained => "constrained",
            Self::Degraded => "degraded",
            Self::ProtectCore => "protect_core",
            Self::Recovery => "recovery",
        }
    }

    /// Human-readable label for shell surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Nominal => "Nominal",
            Self::Constrained => "Constrained",
            Self::Degraded => "Degraded",
            Self::ProtectCore => "Protect core",
            Self::Recovery => "Recovery",
        }
    }

    /// True when background work must be named if it is narrowed.
    pub const fn requires_visible_background_truth(self) -> bool {
        matches!(self, Self::Constrained | Self::Degraded | Self::ProtectCore)
    }
}

/// Primary visible state projected to shell and diagnostics surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisibleHealthState {
    /// Capability is meeting its declared scope.
    Ready,
    /// Capability is progressing toward ready.
    Warming,
    /// A truthful subset is available.
    Partial,
    /// Capability works with reduced freshness, fidelity, or authority.
    Degraded,
    /// A required remote or managed dependency is unavailable.
    Offline,
    /// No truthful safe mapping exists for the current profile or target.
    Unsupported,
    /// The local governor is shedding work to preserve core interaction.
    Overloaded,
}

impl VisibleHealthState {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Warming => "warming",
            Self::Partial => "partial",
            Self::Degraded => "degraded",
            Self::Offline => "offline",
            Self::Unsupported => "unsupported",
            Self::Overloaded => "overloaded",
        }
    }
}

/// Shared resource-governor work classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernorWorkClass {
    /// Typing, selection, save, undo, redo, and visible paint.
    CoreInteraction,
    /// Quick open, symbol jump, outline, and hot-local navigation.
    CoreNavigation,
    /// Explicit user-run command, test, debug, Git, or search request.
    ShortForegroundTask,
    /// Indexing, graph refresh, semantic refresh, cache rebuild, or repo scan.
    BackgroundKnowledgeWork,
    /// AI context expansion, model warmup, marketplace refresh, or speculative help.
    OptionalAssistance,
    /// Upload, replication, opt-in telemetry, crash upload, or support transfer.
    UploadAndReplication,
}

impl GovernorWorkClass {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoreInteraction => "core_interaction",
            Self::CoreNavigation => "core_navigation",
            Self::ShortForegroundTask => "short_foreground_task",
            Self::BackgroundKnowledgeWork => "background_knowledge_work",
            Self::OptionalAssistance => "optional_assistance",
            Self::UploadAndReplication => "upload_and_replication",
        }
    }
}

/// Shared queue lanes exposed by the resource governor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueueLane {
    /// Reserved foreground lane for current user work.
    Foreground,
    /// Hot-set indexing, visible diagnostics, and preview refresh lane.
    InteractiveBackground,
    /// Full scans, AI maintenance, extension warmup, and cache maintenance lane.
    Maintenance,
    /// Provider overlays, remote refresh, and connector retry lane.
    ProviderOverlay,
    /// Upload, replication, support transfer, and sync lane.
    UploadReplication,
}

impl QueueLane {
    /// All queue lanes required by the beta truth packet.
    pub const ALL: [Self; 5] = [
        Self::Foreground,
        Self::InteractiveBackground,
        Self::Maintenance,
        Self::ProviderOverlay,
        Self::UploadReplication,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Foreground => "foreground",
            Self::InteractiveBackground => "interactive_background",
            Self::Maintenance => "maintenance",
            Self::ProviderOverlay => "provider_overlay",
            Self::UploadReplication => "upload_replication",
        }
    }

    /// Human-readable label for shell surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Foreground => "Foreground",
            Self::InteractiveBackground => "Interactive background",
            Self::Maintenance => "Maintenance",
            Self::ProviderOverlay => "Provider overlay",
            Self::UploadReplication => "Upload / replication",
        }
    }

    /// Default budget domain for the lane when no dominant job is running.
    pub const fn default_budget_domain(self) -> &'static str {
        match self {
            Self::Foreground => "foreground_task_budget",
            Self::InteractiveBackground => "knowledge_refresh_budget",
            Self::Maintenance => "maintenance_budget",
            Self::ProviderOverlay => "provider_overlay_budget",
            Self::UploadReplication => "replication_budget",
        }
    }
}

/// Queue-lane state flags that may be shown together on a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueueLaneStateFlag {
    /// Lane is admitted and making progress.
    Running,
    /// Lane scope, cadence, or parallelism is narrowed.
    Narrowed,
    /// Lane is paused until pressure clears or an explicit resume condition fires.
    Paused,
    /// Duplicate or superseded work collapsed instead of growing the queue.
    Coalesced,
    /// Lane has a checkpoint or resume boundary.
    Checkpointed,
    /// New work was denied by admission control.
    Denied,
    /// Lane is resuming gradually after pressure cleared.
    StagedResume,
}

impl QueueLaneStateFlag {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Narrowed => "narrowed",
            Self::Paused => "paused",
            Self::Coalesced => "coalesced",
            Self::Checkpointed => "checkpointed",
            Self::Denied => "denied",
            Self::StagedResume => "staged_resume",
        }
    }

    /// True when this state changes user-visible cadence, freshness, or admission.
    pub const fn changes_behavior(self) -> bool {
        !matches!(self, Self::Running)
    }
}

/// Protected foreground actions that outrank speculative or maintenance work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtectedForegroundAction {
    /// Active editor typing, selection, undo, redo, and local buffer mutation.
    Editing,
    /// Save and autosave durability work.
    Save,
    /// Explicit user cancellation of running or queued work.
    ExplicitCancellation,
    /// Quick open and hot-local lookup.
    QuickOpen,
    /// Direct navigation to known files, symbols, and active surfaces.
    Navigation,
}

impl ProtectedForegroundAction {
    /// All protected foreground actions required by the beta truth packet.
    pub const ALL: [Self; 5] = [
        Self::Editing,
        Self::Save,
        Self::ExplicitCancellation,
        Self::QuickOpen,
        Self::Navigation,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editing => "editing",
            Self::Save => "save",
            Self::ExplicitCancellation => "explicit_cancellation",
            Self::QuickOpen => "quick_open",
            Self::Navigation => "navigation",
        }
    }
}

/// Admission decision applied to a work request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionDecisionClass {
    /// Work is admitted now.
    Admit,
    /// Work is deferred but remains queued and attributable.
    Defer,
    /// Active work pauses at a safe boundary.
    Pause,
    /// New work is denied with an explicit reason.
    Deny,
}

impl AdmissionDecisionClass {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admit => "admit",
            Self::Defer => "defer",
            Self::Pause => "pause",
            Self::Deny => "deny",
        }
    }
}

/// Scope of an override sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverrideScope {
    /// Override is scoped to the current workspace.
    Workspace,
    /// Override is scoped to the active profile.
    Profile,
}

/// Decision attached to a workspace or profile override sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverrideDecisionClass {
    /// Override is allowed by policy.
    Allowed,
    /// Override is blocked by administrator or workspace policy.
    BlockedByPolicy,
    /// Override is blocked by system or user power-saver state.
    BlockedByPowerSaver,
    /// Override is blocked by optional-service quota or provider limit.
    BlockedByQuota,
    /// Override is blocked while protect-core is active.
    BlockedByProtectCore,
}

impl OverrideDecisionClass {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::BlockedByPowerSaver => "blocked_by_power_saver",
            Self::BlockedByQuota => "blocked_by_quota",
            Self::BlockedByProtectCore => "blocked_by_protect_core",
        }
    }

    /// True when the sheet must explain why its controls are blocked.
    pub const fn is_blocked(self) -> bool {
        !matches!(self, Self::Allowed)
    }
}

/// One sampled pressure input that affected a governor decision.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PressureInput {
    /// Pressure dimension sampled by the governor.
    pub dimension: PressureDimension,
    /// Health state implied by this pressure input.
    pub state: GovernorHealthState,
    /// Stable source token or metric id.
    pub source_token: String,
    /// Human-readable current value, unit included when meaningful.
    pub current_value_label: String,
    /// Human-readable threshold or trigger label.
    pub threshold_label: String,
    /// Reviewable reason shown in shell details and diagnostics.
    pub reason: String,
    /// Queue lanes affected by this input.
    pub affected_lanes: Vec<QueueLane>,
}

/// Checkpoint or resume metadata attached to a queue lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckpointMetadata {
    /// Stable checkpoint class.
    pub checkpoint_class: String,
    /// Opaque checkpoint reference when a durable checkpoint exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    /// Phase or item boundary used for resume.
    pub boundary_label: String,
    /// Checkpoint capture timestamp.
    pub captured_at: String,
    /// Replay note for support and diagnostics.
    pub replay_note: String,
    /// Resume note shown to users and support tooling.
    pub resume_note: String,
}

/// Workspace or profile override sheet shown when policy permits or blocks tuning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverrideSheet {
    /// Stable override id.
    pub override_id: String,
    /// Override scope.
    pub scope: OverrideScope,
    /// Queue lane the override applies to.
    pub lane: QueueLane,
    /// Decision applied to the override.
    pub decision: OverrideDecisionClass,
    /// Human-readable override label.
    pub label: String,
    /// Reviewable decision reason.
    pub reason: String,
    /// Policy, power, or quota reference explaining blocked decisions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocking_ref: Option<String>,
    /// Explanation rendered instead of a dead disabled control.
    pub blocked_control_explanation: String,
    /// Action label offered when the override is allowed or can be inspected.
    pub action_label: String,
}

/// Queue-lane state exposed across shell, status, diagnostics, and support export.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueueLaneState {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Active workspace id.
    pub workspace_id: String,
    /// Queue lane.
    pub lane: QueueLane,
    /// Stable queue-lane token.
    pub lane_token: String,
    /// Primary work class currently dominating this lane.
    pub work_class: GovernorWorkClass,
    /// Budget domains currently constrained for the lane.
    pub budget_domains: Vec<String>,
    /// Visible state projected to shell and diagnostics.
    pub visible_state: VisibleHealthState,
    /// Primary lane state shown to users.
    pub primary_state: QueueLaneStateFlag,
    /// Additional state flags that explain pause, coalesce, checkpoint, or deny.
    pub state_flags: Vec<QueueLaneStateFlag>,
    /// Current enqueued plus in-flight job count.
    pub lane_depth: u32,
    /// Seconds since the oldest enqueued job was created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oldest_age_seconds: Option<f64>,
    /// Total collapsed enqueue count.
    pub collapse_count: u32,
    /// Collapses under coalesce policy.
    pub coalesce_count: u32,
    /// Collapses under replace-superseded policy.
    pub replace_count: u32,
    /// Collapses under restart-from-checkpoint policy.
    pub restart_count: u32,
    /// Cancellation lag in milliseconds when sampled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_lag_ms: Option<f64>,
    /// Human-readable names of work slowed, paused, coalesced, or denied.
    pub affected_work_labels: Vec<String>,
    /// Surface refs affected by the lane state.
    pub affected_surface_refs: Vec<String>,
    /// Reason the lane is running, narrowed, paused, coalesced, checkpointed, or denied.
    pub reason: String,
    /// Capabilities that remain usable in the current workspace.
    pub remains_usable: Vec<String>,
    /// Checkpoint or resume metadata when work is checkpointed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint: Option<CheckpointMetadata>,
    /// Replay or resume note shown when work did not complete yet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replay_resume_note: Option<String>,
    /// Denial reason shown when new work is blocked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denied_reason: Option<String>,
    /// Override sheet for this lane when policy allows or blocks tuning.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub override_sheet: Option<OverrideSheet>,
    /// Protected foreground actions preserved while this lane is constrained.
    pub protected_actions_preserved: Vec<ProtectedForegroundAction>,
    /// Observation timestamp.
    pub observed_at: String,
}

impl QueueLaneState {
    /// Returns true when this lane changed user-visible cadence or admission.
    pub fn changes_behavior(&self) -> bool {
        self.primary_state.changes_behavior()
            || self
                .state_flags
                .iter()
                .any(|state| state.changes_behavior())
    }

    /// Returns true when this lane has a checkpoint boundary.
    pub fn has_checkpoint_truth(&self) -> bool {
        self.checkpoint.is_some()
            || self
                .state_flags
                .iter()
                .any(|state| *state == QueueLaneStateFlag::Checkpointed)
    }

    /// Returns true when collapse counters are internally consistent.
    pub fn collapse_counts_match(&self) -> bool {
        self.collapse_count
            == self
                .coalesce_count
                .saturating_add(self.replace_count)
                .saturating_add(self.restart_count)
    }
}

/// Admission-control decision preserved for diagnostics and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdmissionControlDecision {
    /// Stable request id.
    pub request_id: String,
    /// Request label safe for shell and support surfaces.
    pub request_label: String,
    /// Work class of the request.
    pub work_class: GovernorWorkClass,
    /// Queue lane the request would use.
    pub lane: QueueLane,
    /// Admission decision.
    pub decision: AdmissionDecisionClass,
    /// Reviewable reason for the decision.
    pub reason: String,
    /// Protected foreground actions that outrank this request.
    pub outranked_by: Vec<ProtectedForegroundAction>,
    /// Observation timestamp.
    pub observed_at: String,
}

/// Last transition into or out of a constrained governor state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernorTransition {
    /// Previous governor state.
    pub previous_state: GovernorHealthState,
    /// Current governor state.
    pub current_state: GovernorHealthState,
    /// Timestamp when the current state was entered.
    pub entered_at: String,
    /// Trigger dimensions that caused the transition.
    pub trigger_dimensions: Vec<PressureDimension>,
    /// Reviewable transition reason.
    pub reason: String,
    /// Exit or recovery condition shown to users and support tooling.
    pub exit_condition: String,
}

/// Exportable resource-governor truth packet for one workspace sample.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceGovernorSnapshot {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Active workspace id.
    pub workspace_id: String,
    /// Active profile id.
    pub profile_id: String,
    /// Current governor state.
    pub governor_state: GovernorHealthState,
    /// Pressure inputs sampled by the governor.
    pub pressure_inputs: Vec<PressureInput>,
    /// Queue-lane states exposed by the governor.
    pub lane_states: Vec<QueueLaneState>,
    /// Admission decisions sampled for protected and background requests.
    pub admission_decisions: Vec<AdmissionControlDecision>,
    /// Workspace or profile override sheets.
    pub override_sheets: Vec<OverrideSheet>,
    /// Last transition into or out of a constrained state.
    pub last_transition: GovernorTransition,
    /// Protected foreground actions preserved by the snapshot.
    pub protected_actions_preserved: Vec<ProtectedForegroundAction>,
    /// User-facing status summary.
    pub status_summary: String,
    /// Diagnostics reference containing the same packet.
    pub diagnostics_ref: String,
    /// Observation timestamp.
    pub observed_at: String,
}

impl ResourceGovernorSnapshot {
    /// Builds the support-export projection for this snapshot.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> ResourceGovernorSupportExport {
        ResourceGovernorSupportExport {
            record_kind: RESOURCE_GOVERNOR_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: RESOURCE_GOVERNOR_SCHEMA_VERSION,
            export_id: export_id.into(),
            snapshot_id: self.snapshot_id.clone(),
            workspace_id: self.workspace_id.clone(),
            profile_id: self.profile_id.clone(),
            governor_state: self.governor_state,
            pressure_inputs: self.pressure_inputs.clone(),
            lane_states: self.lane_states.clone(),
            admission_decisions: self.admission_decisions.clone(),
            override_sheets: self.override_sheets.clone(),
            last_transition: self.last_transition.clone(),
            protected_actions_preserved: self.protected_actions_preserved.clone(),
            raw_private_material_excluded: true,
            generated_at: generated_at.into(),
        }
    }

    /// Validates that visible pressure and queue truth is complete.
    pub fn validate(&self) -> ResourceGovernorValidationReport {
        let mut violations = Vec::new();
        let present_dimensions = self
            .pressure_inputs
            .iter()
            .map(|input| input.dimension)
            .collect::<BTreeSet<_>>();
        for dimension in PressureDimension::ALL {
            if !present_dimensions.contains(&dimension) {
                violations.push(
                    ResourceGovernorValidationViolation::MissingPressureDimension { dimension },
                );
            }
        }

        let present_lanes = self
            .lane_states
            .iter()
            .map(|state| state.lane)
            .collect::<BTreeSet<_>>();
        for lane in QueueLane::ALL {
            if !present_lanes.contains(&lane) {
                violations.push(ResourceGovernorValidationViolation::MissingQueueLane { lane });
            }
        }

        for lane in &self.lane_states {
            if lane.changes_behavior()
                && (lane.reason.trim().is_empty() || lane.affected_work_labels.is_empty())
            {
                violations.push(
                    ResourceGovernorValidationViolation::ChangedLaneMissingNamedWork {
                        lane: lane.lane,
                    },
                );
            }
            if lane.changes_behavior() && lane.remains_usable.is_empty() {
                violations.push(
                    ResourceGovernorValidationViolation::ChangedLaneMissingUsableWork {
                        lane: lane.lane,
                    },
                );
            }
            if !lane.collapse_counts_match() {
                violations.push(ResourceGovernorValidationViolation::CollapseCountMismatch {
                    lane: lane.lane,
                });
            }
            let coalesced = lane.primary_state == QueueLaneStateFlag::Coalesced
                || lane
                    .state_flags
                    .iter()
                    .any(|state| *state == QueueLaneStateFlag::Coalesced);
            if coalesced && (lane.collapse_count == 0 || lane.oldest_age_seconds.is_none()) {
                violations.push(
                    ResourceGovernorValidationViolation::CoalescedLaneMissingQueueMetadata {
                        lane: lane.lane,
                    },
                );
            }
            let checkpointed = lane.primary_state == QueueLaneStateFlag::Checkpointed
                || lane.primary_state == QueueLaneStateFlag::StagedResume
                || lane
                    .state_flags
                    .iter()
                    .any(|state| *state == QueueLaneStateFlag::Checkpointed);
            if checkpointed && lane.checkpoint.is_none() {
                violations.push(
                    ResourceGovernorValidationViolation::CheckpointedLaneMissingMetadata {
                        lane: lane.lane,
                    },
                );
            }
            if let Some(sheet) = &lane.override_sheet {
                validate_override(sheet, &mut violations);
            }
        }
        for sheet in &self.override_sheets {
            validate_override(sheet, &mut violations);
        }

        let protected = self
            .protected_actions_preserved
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        for action in ProtectedForegroundAction::ALL {
            if !protected.contains(&action) {
                violations
                    .push(ResourceGovernorValidationViolation::MissingProtectedAction { action });
            }
        }

        for decision in &self.admission_decisions {
            if matches!(
                decision.work_class,
                GovernorWorkClass::CoreInteraction | GovernorWorkClass::CoreNavigation
            ) && decision.decision != AdmissionDecisionClass::Admit
            {
                violations.push(
                    ResourceGovernorValidationViolation::ProtectedAdmissionNotAdmitted {
                        request_id: decision.request_id.clone(),
                    },
                );
            }
        }

        ResourceGovernorValidationReport { violations }
    }
}

/// Support-export projection for resource-governor truth.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceGovernorSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Source snapshot id.
    pub snapshot_id: String,
    /// Active workspace id.
    pub workspace_id: String,
    /// Active profile id.
    pub profile_id: String,
    /// Current governor state.
    pub governor_state: GovernorHealthState,
    /// Pressure inputs sampled by the governor.
    pub pressure_inputs: Vec<PressureInput>,
    /// Queue-lane states exposed by the governor.
    pub lane_states: Vec<QueueLaneState>,
    /// Admission decisions sampled for protected and background requests.
    pub admission_decisions: Vec<AdmissionControlDecision>,
    /// Workspace or profile override sheets.
    pub override_sheets: Vec<OverrideSheet>,
    /// Last transition into or out of a constrained state.
    pub last_transition: GovernorTransition,
    /// Protected foreground actions preserved by the export.
    pub protected_actions_preserved: Vec<ProtectedForegroundAction>,
    /// True when raw content, paths, command lines, and provider payloads are excluded.
    pub raw_private_material_excluded: bool,
    /// Export generation timestamp.
    pub generated_at: String,
}

impl ResourceGovernorSupportExport {
    /// Renders a compact plaintext summary for diagnostics and CLI output.
    pub fn render_plaintext(&self) -> String {
        let mut lines = vec![format!(
            "Resource governor {} for workspace {}",
            self.governor_state.as_str(),
            self.workspace_id
        )];
        lines.push(format!(
            "Last transition: {} -> {} ({})",
            self.last_transition.previous_state.as_str(),
            self.last_transition.current_state.as_str(),
            self.last_transition.reason
        ));
        for lane in &self.lane_states {
            lines.push(format!(
                "lane={} state={} visible={} depth={} oldest_age={:?} collapsed={} checkpoint={}",
                lane.lane.as_str(),
                lane.primary_state.as_str(),
                lane.visible_state.as_str(),
                lane.lane_depth,
                lane.oldest_age_seconds,
                lane.collapse_count,
                lane.checkpoint
                    .as_ref()
                    .map(|checkpoint| checkpoint.boundary_label.as_str())
                    .unwrap_or("none")
            ));
        }
        for sheet in &self.override_sheets {
            lines.push(format!(
                "override={} decision={} reason={}",
                sheet.override_id,
                sheet.decision.as_str(),
                sheet.reason
            ));
        }
        lines.join("\n")
    }
}

/// Validation report for a resource-governor snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceGovernorValidationReport {
    /// Validation violations.
    pub violations: Vec<ResourceGovernorValidationViolation>,
}

impl ResourceGovernorValidationReport {
    /// Returns true when no validation violations were found.
    pub fn is_ok(&self) -> bool {
        self.violations.is_empty()
    }
}

/// Validation failure for resource-governor truth packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceGovernorValidationViolation {
    /// A required pressure dimension is missing.
    MissingPressureDimension {
        /// Missing pressure dimension.
        dimension: PressureDimension,
    },
    /// A required queue lane is missing.
    MissingQueueLane {
        /// Missing queue lane.
        lane: QueueLane,
    },
    /// A constrained lane does not name the affected work.
    ChangedLaneMissingNamedWork {
        /// Queue lane with missing work labels.
        lane: QueueLane,
    },
    /// A constrained lane does not say what remains usable.
    ChangedLaneMissingUsableWork {
        /// Queue lane with missing usability labels.
        lane: QueueLane,
    },
    /// Collapse totals do not equal the subcounts.
    CollapseCountMismatch {
        /// Queue lane with mismatched collapse counters.
        lane: QueueLane,
    },
    /// A coalesced lane is missing queue age or collapse metadata.
    CoalescedLaneMissingQueueMetadata {
        /// Queue lane with missing queue metadata.
        lane: QueueLane,
    },
    /// A checkpointed lane is missing checkpoint metadata.
    CheckpointedLaneMissingMetadata {
        /// Queue lane with missing checkpoint metadata.
        lane: QueueLane,
    },
    /// A blocked override would render as a dead control.
    BlockedOverrideMissingExplanation {
        /// Override id with missing explanation.
        override_id: String,
    },
    /// A protected action is missing from the packet.
    MissingProtectedAction {
        /// Missing protected action.
        action: ProtectedForegroundAction,
    },
    /// Core interaction or navigation was not admitted.
    ProtectedAdmissionNotAdmitted {
        /// Request id that violated foreground protection.
        request_id: String,
    },
}

fn validate_override(
    sheet: &OverrideSheet,
    violations: &mut Vec<ResourceGovernorValidationViolation>,
) {
    if sheet.decision.is_blocked() && sheet.blocked_control_explanation.trim().is_empty() {
        violations.push(
            ResourceGovernorValidationViolation::BlockedOverrideMissingExplanation {
                override_id: sheet.override_id.clone(),
            },
        );
    }
}

/// Builds a seeded resource-governor snapshot for fixture and surface tests.
pub fn seeded_resource_governor_snapshot(
    snapshot_id: impl Into<String>,
    workspace_id: impl Into<String>,
    profile_id: impl Into<String>,
    observed_at: impl Into<String>,
) -> ResourceGovernorSnapshot {
    let snapshot_id = snapshot_id.into();
    let workspace_id = workspace_id.into();
    let profile_id = profile_id.into();
    let observed_at = observed_at.into();
    let protected_actions = ProtectedForegroundAction::ALL.to_vec();
    let pressure_inputs = seeded_pressure_inputs();
    let interactive_override = OverrideSheet {
        override_id: "override.workspace.indexing.hot_set_only".to_owned(),
        scope: OverrideScope::Workspace,
        lane: QueueLane::InteractiveBackground,
        decision: OverrideDecisionClass::BlockedByPowerSaver,
        label: "Resume full-workspace indexing".to_owned(),
        reason: "System battery saver is active, so full-workspace indexing cannot widen yet."
            .to_owned(),
        blocking_ref: Some("power:os_battery_saver".to_owned()),
        blocked_control_explanation:
            "The control remains visible with this explanation instead of acting as a disabled dead end."
                .to_owned(),
        action_label: "Open power details".to_owned(),
    };
    let maintenance_override = OverrideSheet {
        override_id: "override.profile.ai_context_background".to_owned(),
        scope: OverrideScope::Profile,
        lane: QueueLane::Maintenance,
        decision: OverrideDecisionClass::BlockedByPolicy,
        label: "Allow background AI context refresh".to_owned(),
        reason: "Profile policy blocks background AI context expansion while protect-core is active."
            .to_owned(),
        blocking_ref: Some("policy:background_ai_context.blocked".to_owned()),
        blocked_control_explanation:
            "Policy explains the block and leaves foreground AI actions reviewable; no hidden retry starts."
                .to_owned(),
        action_label: "Review policy".to_owned(),
    };
    let lane_states = vec![
        QueueLaneState {
            record_kind: QUEUE_LANE_STATE_RECORD_KIND.to_owned(),
            schema_version: RESOURCE_GOVERNOR_SCHEMA_VERSION,
            workspace_id: workspace_id.clone(),
            lane: QueueLane::Foreground,
            lane_token: QueueLane::Foreground.as_str().to_owned(),
            work_class: GovernorWorkClass::CoreInteraction,
            budget_domains: vec!["hot_path_interactive_budget".to_owned()],
            visible_state: VisibleHealthState::Ready,
            primary_state: QueueLaneStateFlag::Running,
            state_flags: vec![QueueLaneStateFlag::Running],
            lane_depth: 1,
            oldest_age_seconds: Some(0.2),
            collapse_count: 0,
            coalesce_count: 0,
            replace_count: 0,
            restart_count: 0,
            cancellation_lag_ms: Some(8.0),
            affected_work_labels: vec!["save and active editor commands".to_owned()],
            affected_surface_refs: vec!["surface.editor.active".to_owned()],
            reason: "Foreground actions remain admitted while pressure sheds lower lanes."
                .to_owned(),
            remains_usable: vec![
                "typing".to_owned(),
                "save".to_owned(),
                "undo".to_owned(),
                "quick open".to_owned(),
            ],
            checkpoint: None,
            replay_resume_note: None,
            denied_reason: None,
            override_sheet: None,
            protected_actions_preserved: protected_actions.clone(),
            observed_at: observed_at.clone(),
        },
        QueueLaneState {
            record_kind: QUEUE_LANE_STATE_RECORD_KIND.to_owned(),
            schema_version: RESOURCE_GOVERNOR_SCHEMA_VERSION,
            workspace_id: workspace_id.clone(),
            lane: QueueLane::InteractiveBackground,
            lane_token: QueueLane::InteractiveBackground.as_str().to_owned(),
            work_class: GovernorWorkClass::BackgroundKnowledgeWork,
            budget_domains: vec!["knowledge_refresh_budget".to_owned()],
            visible_state: VisibleHealthState::Partial,
            primary_state: QueueLaneStateFlag::Narrowed,
            state_flags: vec![
                QueueLaneStateFlag::Narrowed,
                QueueLaneStateFlag::Coalesced,
                QueueLaneStateFlag::Checkpointed,
            ],
            lane_depth: 8,
            oldest_age_seconds: Some(134.0),
            collapse_count: 7,
            coalesce_count: 7,
            replace_count: 0,
            restart_count: 0,
            cancellation_lag_ms: Some(35.0),
            affected_work_labels: vec![
                "hot-set indexing".to_owned(),
                "semantic graph enrichment".to_owned(),
            ],
            affected_surface_refs: vec![
                "surface.search.results".to_owned(),
                "surface.graph.status".to_owned(),
            ],
            reason:
                "Whole-workspace refresh is narrowed to open files and the hot set under pressure."
                    .to_owned(),
            remains_usable: vec![
                "open-file diagnostics".to_owned(),
                "quick open from hot local state".to_owned(),
            ],
            checkpoint: Some(CheckpointMetadata {
                checkpoint_class: "disk_persisted_checkpoint".to_owned(),
                checkpoint_ref: Some("checkpoint.index.hot_set.phase_3".to_owned()),
                boundary_label: "symbol-index phase 3".to_owned(),
                captured_at: observed_at.clone(),
                replay_note:
                    "Coalesced file-change scans replay from the phase boundary instead of implying completion."
                        .to_owned(),
                resume_note:
                    "Full-workspace refresh resumes from phase 3 when battery saver clears."
                        .to_owned(),
            }),
            replay_resume_note: Some(
                "Seven duplicate refreshes are collapsed into the next checkpointed index pass."
                    .to_owned(),
            ),
            denied_reason: None,
            override_sheet: Some(interactive_override.clone()),
            protected_actions_preserved: protected_actions.clone(),
            observed_at: observed_at.clone(),
        },
        QueueLaneState {
            record_kind: QUEUE_LANE_STATE_RECORD_KIND.to_owned(),
            schema_version: RESOURCE_GOVERNOR_SCHEMA_VERSION,
            workspace_id: workspace_id.clone(),
            lane: QueueLane::Maintenance,
            lane_token: QueueLane::Maintenance.as_str().to_owned(),
            work_class: GovernorWorkClass::OptionalAssistance,
            budget_domains: vec!["maintenance_budget".to_owned()],
            visible_state: VisibleHealthState::Overloaded,
            primary_state: QueueLaneStateFlag::Denied,
            state_flags: vec![
                QueueLaneStateFlag::Paused,
                QueueLaneStateFlag::Denied,
                QueueLaneStateFlag::Checkpointed,
            ],
            lane_depth: 11,
            oldest_age_seconds: Some(410.0),
            collapse_count: 3,
            coalesce_count: 0,
            replace_count: 3,
            restart_count: 0,
            cancellation_lag_ms: Some(42.0),
            affected_work_labels: vec![
                "AI background context expansion".to_owned(),
                "marketplace refresh".to_owned(),
                "extension warm-up".to_owned(),
            ],
            affected_surface_refs: vec![
                "surface.ai.context".to_owned(),
                "surface.extensions.status".to_owned(),
            ],
            reason: "Protect-core denies new optional maintenance so editing and save stay funded."
                .to_owned(),
            remains_usable: vec![
                "foreground AI review can still request explicit approval".to_owned(),
                "already-open extension UI remains inspectable".to_owned(),
            ],
            checkpoint: Some(CheckpointMetadata {
                checkpoint_class: "supervisor_acknowledged_epoch".to_owned(),
                checkpoint_ref: Some("checkpoint.maintenance.optional.epoch_18".to_owned()),
                boundary_label: "optional-maintenance epoch 18".to_owned(),
                captured_at: observed_at.clone(),
                replay_note: "Speculative jobs are not replayed automatically after denial."
                    .to_owned(),
                resume_note: "Maintenance restarts with a fresh admission check after recovery."
                    .to_owned(),
            }),
            replay_resume_note: Some(
                "Paused optional jobs remain visible and do not masquerade as complete."
                    .to_owned(),
            ),
            denied_reason: Some("protect_core_pauses_optional_assistance".to_owned()),
            override_sheet: Some(maintenance_override.clone()),
            protected_actions_preserved: protected_actions.clone(),
            observed_at: observed_at.clone(),
        },
        QueueLaneState {
            record_kind: QUEUE_LANE_STATE_RECORD_KIND.to_owned(),
            schema_version: RESOURCE_GOVERNOR_SCHEMA_VERSION,
            workspace_id: workspace_id.clone(),
            lane: QueueLane::ProviderOverlay,
            lane_token: QueueLane::ProviderOverlay.as_str().to_owned(),
            work_class: GovernorWorkClass::BackgroundKnowledgeWork,
            budget_domains: vec!["provider_overlay_budget".to_owned()],
            visible_state: VisibleHealthState::Degraded,
            primary_state: QueueLaneStateFlag::Paused,
            state_flags: vec![
                QueueLaneStateFlag::Paused,
                QueueLaneStateFlag::Coalesced,
                QueueLaneStateFlag::Checkpointed,
            ],
            lane_depth: 4,
            oldest_age_seconds: Some(88.0),
            collapse_count: 2,
            coalesce_count: 0,
            replace_count: 2,
            restart_count: 0,
            cancellation_lag_ms: Some(60.0),
            affected_work_labels: vec![
                "provider overlay refresh".to_owned(),
                "remote ruleset refresh".to_owned(),
            ],
            affected_surface_refs: vec!["surface.provider.overlay".to_owned()],
            reason:
                "Provider quota pressure opened the circuit breaker; local Git and cached review truth remain usable."
                    .to_owned(),
            remains_usable: vec![
                "local Git status".to_owned(),
                "cached provider overlay with stale label".to_owned(),
            ],
            checkpoint: Some(CheckpointMetadata {
                checkpoint_class: "session_handoff_checkpoint".to_owned(),
                checkpoint_ref: Some("checkpoint.provider.overlay.ruleset_fetch".to_owned()),
                boundary_label: "provider ruleset fetch phase".to_owned(),
                captured_at: observed_at.clone(),
                replay_note:
                    "Replaced provider refreshes keep the stale label until fresh provider data arrives."
                        .to_owned(),
                resume_note: "Provider overlay refresh resumes after quota reset or manual retry."
                    .to_owned(),
            }),
            replay_resume_note: Some(
                "Collapsed provider refreshes do not claim current mergeability.".to_owned(),
            ),
            denied_reason: None,
            override_sheet: None,
            protected_actions_preserved: protected_actions.clone(),
            observed_at: observed_at.clone(),
        },
        QueueLaneState {
            record_kind: QUEUE_LANE_STATE_RECORD_KIND.to_owned(),
            schema_version: RESOURCE_GOVERNOR_SCHEMA_VERSION,
            workspace_id: workspace_id.clone(),
            lane: QueueLane::UploadReplication,
            lane_token: QueueLane::UploadReplication.as_str().to_owned(),
            work_class: GovernorWorkClass::UploadAndReplication,
            budget_domains: vec!["replication_budget".to_owned()],
            visible_state: VisibleHealthState::Overloaded,
            primary_state: QueueLaneStateFlag::Paused,
            state_flags: vec![QueueLaneStateFlag::Paused, QueueLaneStateFlag::Checkpointed],
            lane_depth: 6,
            oldest_age_seconds: Some(905.0),
            collapse_count: 0,
            coalesce_count: 0,
            replace_count: 0,
            restart_count: 0,
            cancellation_lag_ms: None,
            affected_work_labels: vec![
                "support bundle upload".to_owned(),
                "replication transfer".to_owned(),
            ],
            affected_surface_refs: vec!["surface.transfer.status".to_owned()],
            reason:
                "Uploads are paused before they compete with save, navigation, or local editing."
                    .to_owned(),
            remains_usable: vec![
                "manual local export".to_owned(),
                "queued transfer review".to_owned(),
            ],
            checkpoint: Some(CheckpointMetadata {
                checkpoint_class: "collector_ingested_checkpoint".to_owned(),
                checkpoint_ref: Some("checkpoint.upload.chunk_12".to_owned()),
                boundary_label: "resumable upload chunk 12".to_owned(),
                captured_at: observed_at.clone(),
                replay_note:
                    "Queued upload chunks stay attributable and are not sent silently on recovery."
                        .to_owned(),
                resume_note: "Replication resumes with backoff after protect-core exits."
                    .to_owned(),
            }),
            replay_resume_note: Some(
                "Transfers remain queued until staged resume or explicit foreground send.".to_owned(),
            ),
            denied_reason: None,
            override_sheet: None,
            protected_actions_preserved: protected_actions.clone(),
            observed_at: observed_at.clone(),
        },
    ];

    ResourceGovernorSnapshot {
        record_kind: RESOURCE_GOVERNOR_SNAPSHOT_RECORD_KIND.to_owned(),
        schema_version: RESOURCE_GOVERNOR_SCHEMA_VERSION,
        snapshot_id,
        workspace_id: workspace_id.clone(),
        profile_id: profile_id.clone(),
        governor_state: GovernorHealthState::ProtectCore,
        pressure_inputs,
        lane_states,
        admission_decisions: seeded_admission_decisions(&observed_at),
        override_sheets: vec![interactive_override, maintenance_override],
        last_transition: GovernorTransition {
            previous_state: GovernorHealthState::Degraded,
            current_state: GovernorHealthState::ProtectCore,
            entered_at: observed_at.clone(),
            trigger_dimensions: vec![
                PressureDimension::BatteryThermal,
                PressureDimension::Cpu,
                PressureDimension::OptionalServiceQuota,
            ],
            reason:
                "Thermal pressure and optional-service quota exhaustion crossed protect-core thresholds."
                    .to_owned(),
            exit_condition:
                "Exit after key-to-paint, thermal pressure, and oldest queue age return below degraded thresholds."
                    .to_owned(),
        },
        protected_actions_preserved: protected_actions,
        status_summary:
            "Protect core: indexing, provider overlay, maintenance, and uploads are constrained; editing, save, cancellation, quick open, and navigation remain admitted."
                .to_owned(),
        diagnostics_ref: format!(
            "diagnostics.resource_governor.snapshot.{}.{}",
            workspace_id, profile_id
        ),
        observed_at,
    }
}

/// Builds the seeded support export for resource-governor fixture tests.
pub fn seeded_resource_governor_support_export(
    export_id: impl Into<String>,
    generated_at: impl Into<String>,
) -> ResourceGovernorSupportExport {
    let generated_at = generated_at.into();
    seeded_resource_governor_snapshot(
        "resource-governor:snapshot:beta-seed",
        "workspace.resource-governor.beta",
        "profile.standard",
        generated_at.clone(),
    )
    .support_export(export_id, generated_at)
}

fn seeded_pressure_inputs() -> Vec<PressureInput> {
    vec![
        PressureInput {
            dimension: PressureDimension::Cpu,
            state: GovernorHealthState::Degraded,
            source_token: "hidden_background_cpu_logical_cores".to_owned(),
            current_value_label: "1.2 logical cores".to_owned(),
            threshold_label: ">= 1.0 logical core for the protected window".to_owned(),
            reason: "Hidden background CPU stayed above the degraded floor.".to_owned(),
            affected_lanes: vec![QueueLane::InteractiveBackground, QueueLane::Maintenance],
        },
        PressureInput {
            dimension: PressureDimension::Memory,
            state: GovernorHealthState::Constrained,
            source_token: "service_class_soft_cap_percent".to_owned(),
            current_value_label: "88% soft cap".to_owned(),
            threshold_label: "85% constrained floor".to_owned(),
            reason: "Disposable cache trimming is active before dirty buffers are touched."
                .to_owned(),
            affected_lanes: vec![QueueLane::Maintenance],
        },
        PressureInput {
            dimension: PressureDimension::Disk,
            state: GovernorHealthState::Constrained,
            source_token: "background_io_queue_oldest_age_seconds".to_owned(),
            current_value_label: "42s oldest background I/O".to_owned(),
            threshold_label: "30s constrained queue-age floor".to_owned(),
            reason: "Background I/O is yielding before save latency is affected.".to_owned(),
            affected_lanes: vec![
                QueueLane::InteractiveBackground,
                QueueLane::UploadReplication,
            ],
        },
        PressureInput {
            dimension: PressureDimension::BatteryThermal,
            state: GovernorHealthState::ProtectCore,
            source_token: "thermal_pressure_and_os_battery_saver".to_owned(),
            current_value_label: "thermal serious, battery saver on".to_owned(),
            threshold_label: "protect-core thermal clamp".to_owned(),
            reason: "Thermal pressure and battery saver require visible background narrowing."
                .to_owned(),
            affected_lanes: vec![
                QueueLane::InteractiveBackground,
                QueueLane::Maintenance,
                QueueLane::UploadReplication,
            ],
        },
        PressureInput {
            dimension: PressureDimension::Network,
            state: GovernorHealthState::Degraded,
            source_token: "remote_error_rate_percent".to_owned(),
            current_value_label: "31% remote errors".to_owned(),
            threshold_label: "25% degraded remote error rate".to_owned(),
            reason: "Remote helper retries are backed off within freshness bounds.".to_owned(),
            affected_lanes: vec![QueueLane::ProviderOverlay],
        },
        PressureInput {
            dimension: PressureDimension::OptionalServiceQuota,
            state: GovernorHealthState::ProtectCore,
            source_token: "optional_service_quota_exhausted".to_owned(),
            current_value_label: "quota exhausted".to_owned(),
            threshold_label: "new optional work denied".to_owned(),
            reason: "Optional provider work is denied instead of borrowing hot-path budget."
                .to_owned(),
            affected_lanes: vec![QueueLane::Maintenance, QueueLane::ProviderOverlay],
        },
    ]
}

fn seeded_admission_decisions(observed_at: &str) -> Vec<AdmissionControlDecision> {
    vec![
        AdmissionControlDecision {
            request_id: "admission.save.active_buffer".to_owned(),
            request_label: "Save active buffer".to_owned(),
            work_class: GovernorWorkClass::CoreInteraction,
            lane: QueueLane::Foreground,
            decision: AdmissionDecisionClass::Admit,
            reason: "Save is protected and stays ahead of background work.".to_owned(),
            outranked_by: Vec::new(),
            observed_at: observed_at.to_owned(),
        },
        AdmissionControlDecision {
            request_id: "admission.cancel.index_refresh".to_owned(),
            request_label: "Cancel index refresh".to_owned(),
            work_class: GovernorWorkClass::CoreInteraction,
            lane: QueueLane::Foreground,
            decision: AdmissionDecisionClass::Admit,
            reason: "Explicit cancellation preempts checkpointed background work.".to_owned(),
            outranked_by: Vec::new(),
            observed_at: observed_at.to_owned(),
        },
        AdmissionControlDecision {
            request_id: "admission.ai.background_context".to_owned(),
            request_label: "AI background context expansion".to_owned(),
            work_class: GovernorWorkClass::OptionalAssistance,
            lane: QueueLane::Maintenance,
            decision: AdmissionDecisionClass::Deny,
            reason: "Optional assistance is denied while protect-core is active.".to_owned(),
            outranked_by: ProtectedForegroundAction::ALL.to_vec(),
            observed_at: observed_at.to_owned(),
        },
        AdmissionControlDecision {
            request_id: "admission.provider.overlay_refresh".to_owned(),
            request_label: "Provider overlay refresh".to_owned(),
            work_class: GovernorWorkClass::BackgroundKnowledgeWork,
            lane: QueueLane::ProviderOverlay,
            decision: AdmissionDecisionClass::Defer,
            reason: "Provider quota pressure defers overlay refresh and keeps cached truth labeled stale."
                .to_owned(),
            outranked_by: ProtectedForegroundAction::ALL.to_vec(),
            observed_at: observed_at.to_owned(),
        },
        AdmissionControlDecision {
            request_id: "admission.support.upload".to_owned(),
            request_label: "Support bundle upload".to_owned(),
            work_class: GovernorWorkClass::UploadAndReplication,
            lane: QueueLane::UploadReplication,
            decision: AdmissionDecisionClass::Pause,
            reason: "Background transfer pauses before competing with save or local editing.".to_owned(),
            outranked_by: ProtectedForegroundAction::ALL.to_vec(),
            observed_at: observed_at.to_owned(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_snapshot_covers_pressure_lanes_and_protected_actions() {
        let snapshot = seeded_resource_governor_snapshot(
            "snapshot",
            "workspace",
            "profile",
            "2026-05-18T18:00:00Z",
        );
        let report = snapshot.validate();
        assert!(report.is_ok(), "violations: {:?}", report.violations);
        assert_eq!(snapshot.lane_states.len(), QueueLane::ALL.len());
        assert!(snapshot
            .lane_states
            .iter()
            .any(|lane| lane.state_flags.contains(&QueueLaneStateFlag::Coalesced)));
        assert!(snapshot
            .lane_states
            .iter()
            .any(|lane| lane.state_flags.contains(&QueueLaneStateFlag::Denied)));
    }

    #[test]
    fn support_export_preserves_queue_age_collapse_and_checkpoint_truth() {
        let export = seeded_resource_governor_support_export("export", "2026-05-18T18:00:00Z");
        assert!(export.raw_private_material_excluded);
        let text = export.render_plaintext();
        assert!(text.contains("lane=interactive_background"));
        assert!(text.contains("collapsed=7"));
        assert!(text.contains("symbol-index phase 3"));
        assert!(text.contains("override.workspace.indexing.hot_set_only"));
    }
}
