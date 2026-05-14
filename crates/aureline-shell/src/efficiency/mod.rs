//! Efficiency-state runtime hooks for power, thermal, and hidden-pane truth.
//!
//! This module is the shell-owned alpha bridge between the shared
//! power/thermal policy, visible shell status, durable activity rows, and
//! benchmark/support evidence. It does not replace the resource governor; it
//! projects the governor's efficiency state, workload decisions, and render
//! visibility decisions into inspectable records.

use serde::{Deserialize, Serialize};

use crate::activity_center::alpha::{
    ActivityCancellabilityClass, ActivityJobFamily, ActivityProgressForm, ActivityRow,
    ActivityRowAction, ActivityRowCollapseState, ActivityRowDisplayState, ActivityRowImpact,
    ActivityRowInput, ActivityRowProgress, ActivityRowStateClass, ActivityRowSupportLink,
    ActivityRowTimeline,
};
use crate::notifications::envelope::{
    PrivacyClass, RedactionClass, SeverityClass, SourceSubsystem,
};
use crate::state_cards::DegradedStateToken;

/// Stable record kind for an efficiency-state shell snapshot.
pub const EFFICIENCY_STATE_SNAPSHOT_RECORD_KIND: &str = "efficiency_state_alpha_snapshot";

/// Schema version for [`EfficiencyStateSnapshot`] records.
pub const EFFICIENCY_STATE_SNAPSHOT_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for efficiency-state status projections.
pub const EFFICIENCY_STATUS_RECORD_KIND: &str = "efficiency_status_projection";

/// Stable record kind for workload-budget decisions.
pub const WORKLOAD_BUDGET_DECISION_RECORD_KIND: &str = "workload_budget_decision";

/// Stable record kind for render-visibility decisions.
pub const RENDER_VISIBILITY_DECISION_RECORD_KIND: &str = "render_visibility_decision";

/// Stable record kind for hidden-pane render audit summaries.
pub const HIDDEN_PANE_RENDER_AUDIT_RECORD_KIND: &str = "hidden_pane_render_audit";

/// Runtime efficiency state published by the power/thermal policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EfficiencyState {
    /// Full published budgets are available subject to ordinary governor rules.
    Nominal,
    /// Battery or power-saver pressure is reducing speculative work.
    EfficiencyAware,
    /// Thermal or sustained CPU pressure is reducing background and visual work.
    ThermalConstrained,
    /// Core interaction is protected by pausing or denying optional work.
    ProtectCore,
    /// Pressure has cleared and deferred work is resuming in stages.
    Recovery,
}

impl Default for EfficiencyState {
    fn default() -> Self {
        Self::Nominal
    }
}

impl EfficiencyState {
    /// Stable token used by benchmark and support evidence.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Nominal => "Nominal",
            Self::EfficiencyAware => "EfficiencyAware",
            Self::ThermalConstrained => "ThermalConstrained",
            Self::ProtectCore => "ProtectCore",
            Self::Recovery => "Recovery",
        }
    }

    /// Human-readable label rendered in shell status.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Nominal => "Nominal",
            Self::EfficiencyAware => "Efficiency aware",
            Self::ThermalConstrained => "Thermal constrained",
            Self::ProtectCore => "Protect core",
            Self::Recovery => "Recovery",
        }
    }

    /// True when ordinary shell chrome should expose this state if behavior changed.
    pub const fn status_item_required(self) -> bool {
        matches!(
            self,
            Self::EfficiencyAware | Self::ThermalConstrained | Self::ProtectCore
        )
    }

    /// Degraded-state token used by shell surfaces.
    pub const fn degraded_token(self) -> Option<DegradedStateToken> {
        match self {
            Self::Nominal => None,
            Self::EfficiencyAware | Self::ThermalConstrained => Some(DegradedStateToken::Limited),
            Self::ProtectCore => Some(DegradedStateToken::Limited),
            Self::Recovery => Some(DegradedStateToken::Warming),
        }
    }
}

/// Source signal that caused an efficiency-state decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EfficiencyPressureSource {
    /// Host is on AC power without material power pressure.
    AcPower,
    /// Host is running on battery.
    Battery,
    /// Operating system battery saver is active.
    OsBatterySaver,
    /// User-selected low-power mode is active.
    UserLowPowerMode,
    /// Remaining battery is low enough to reduce optional work.
    LowBattery,
    /// Remaining battery is critical and core interaction is protected.
    CriticalBattery,
    /// Operating system or host thermal pressure is active.
    ThermalPressure,
    /// Repeated protected-frame misses occurred while under pressure.
    FrameMissPressure,
    /// Admin or local policy capped background work.
    PolicyCap,
    /// Power or thermal pressure cleared and recovery is staged.
    PressureCleared,
}

impl EfficiencyPressureSource {
    /// Stable token recorded in exported evidence.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AcPower => "ac_power",
            Self::Battery => "battery",
            Self::OsBatterySaver => "os_battery_saver",
            Self::UserLowPowerMode => "user_low_power_mode",
            Self::LowBattery => "low_battery",
            Self::CriticalBattery => "critical_battery",
            Self::ThermalPressure => "thermal_pressure",
            Self::FrameMissPressure => "frame_miss_pressure",
            Self::PolicyCap => "policy_cap",
            Self::PressureCleared => "pressure_cleared",
        }
    }

    /// Human-readable label rendered next to the active state.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AcPower => "AC power",
            Self::Battery => "battery",
            Self::OsBatterySaver => "OS battery saver",
            Self::UserLowPowerMode => "user low-power mode",
            Self::LowBattery => "low battery",
            Self::CriticalBattery => "critical battery",
            Self::ThermalPressure => "thermal pressure",
            Self::FrameMissPressure => "frame misses under pressure",
            Self::PolicyCap => "policy cap",
            Self::PressureCleared => "pressure cleared",
        }
    }
}

/// Workload family governed by efficiency-state budget decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkloadFamily {
    /// AI warmups and background model preparation.
    AiWarmup,
    /// Prefetch and cache widening.
    SpeculativePrefetch,
    /// Uploads, replication, and deferred transfer.
    UploadTransfer,
    /// Non-essential motion and decorative animation.
    NonEssentialAnimation,
    /// Indexing, semantic refresh, and workspace scan.
    IndexingRefresh,
    /// Extension timers, polling, and background refresh.
    ExtensionPolling,
    /// Preview, browser-runtime, and canvas refresh.
    PreviewRefresh,
    /// Graph enrichment and non-hot-set semantic widening.
    GraphEnrichment,
    /// Remote reconnect, heartbeat, and session helper work.
    RemoteSessionHelper,
}

impl WorkloadFamily {
    /// Stable token used by policy fixtures and support evidence.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiWarmup => "ai_warmup",
            Self::SpeculativePrefetch => "speculative_prefetch",
            Self::UploadTransfer => "upload_transfer",
            Self::NonEssentialAnimation => "non_essential_animation",
            Self::IndexingRefresh => "indexing_refresh",
            Self::ExtensionPolling => "extension_polling",
            Self::PreviewRefresh => "preview_refresh",
            Self::GraphEnrichment => "graph_enrichment",
            Self::RemoteSessionHelper => "remote_session_helper",
        }
    }

    /// Human-readable capability label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AiWarmup => "AI warmups",
            Self::SpeculativePrefetch => "prefetch",
            Self::UploadTransfer => "uploads and replication",
            Self::NonEssentialAnimation => "non-essential animation",
            Self::IndexingRefresh => "indexing refresh",
            Self::ExtensionPolling => "extension background refresh",
            Self::PreviewRefresh => "preview refresh",
            Self::GraphEnrichment => "semantic graph enrichment",
            Self::RemoteSessionHelper => "remote/session helpers",
        }
    }

    /// Shared resource-governor work class.
    pub const fn work_class(self) -> WorkClass {
        match self {
            Self::AiWarmup => WorkClass::OptionalAssistance,
            Self::SpeculativePrefetch
            | Self::IndexingRefresh
            | Self::ExtensionPolling
            | Self::PreviewRefresh
            | Self::GraphEnrichment
            | Self::RemoteSessionHelper => WorkClass::BackgroundKnowledgeWork,
            Self::UploadTransfer => WorkClass::UploadAndReplication,
            Self::NonEssentialAnimation => WorkClass::CoreInteraction,
        }
    }

    /// Shared queue lane.
    pub const fn queue_lane(self) -> QueueLane {
        match self {
            Self::AiWarmup | Self::SpeculativePrefetch => QueueLane::Maintenance,
            Self::UploadTransfer => QueueLane::UploadReplication,
            Self::NonEssentialAnimation => QueueLane::Foreground,
            Self::IndexingRefresh | Self::PreviewRefresh | Self::GraphEnrichment => {
                QueueLane::InteractiveBackground
            }
            Self::ExtensionPolling | Self::RemoteSessionHelper => QueueLane::ProviderOverlay,
        }
    }

    /// Source subsystem that owns the affected work.
    pub const fn source_subsystem(self) -> SourceSubsystem {
        match self {
            Self::AiWarmup => SourceSubsystem::AiApply,
            Self::SpeculativePrefetch => SourceSubsystem::Shell,
            Self::UploadTransfer => SourceSubsystem::SyncMirror,
            Self::NonEssentialAnimation => SourceSubsystem::Shell,
            Self::IndexingRefresh => SourceSubsystem::Indexer,
            Self::ExtensionPolling => SourceSubsystem::ExtensionHost,
            Self::PreviewRefresh => SourceSubsystem::ReviewAndDiff,
            Self::GraphEnrichment => SourceSubsystem::Indexer,
            Self::RemoteSessionHelper => SourceSubsystem::RemoteAgent,
        }
    }
}

/// Shared resource-governor work class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkClass {
    /// Typing, paint, save, and other reserved interaction work.
    CoreInteraction,
    /// Quick open, symbol jump, and current navigation work.
    CoreNavigation,
    /// Explicit short foreground work.
    ShortForegroundTask,
    /// Indexing, graph, preview, and refresh work.
    BackgroundKnowledgeWork,
    /// Optional AI and speculative help.
    OptionalAssistance,
    /// Upload, replication, and telemetry transfer work.
    UploadAndReplication,
}

impl WorkClass {
    /// Stable token used by runtime and benchmark records.
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

/// Shared background queue lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueueLane {
    /// Reserved foreground work lane.
    Foreground,
    /// Interactive background lane.
    InteractiveBackground,
    /// Maintenance lane.
    Maintenance,
    /// Provider or extension overlay lane.
    ProviderOverlay,
    /// Upload and replication lane.
    UploadReplication,
}

impl QueueLane {
    /// Stable token used by runtime and benchmark records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Foreground => "foreground",
            Self::InteractiveBackground => "interactive_background",
            Self::Maintenance => "maintenance",
            Self::ProviderOverlay => "provider_overlay",
            Self::UploadReplication => "upload_replication",
        }
    }
}

/// Budget action chosen by the efficiency-state hook.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkloadAction {
    /// Work may proceed within its ordinary budget.
    Admit,
    /// Work may proceed with reduced cadence, scope, or parallelism.
    Throttle,
    /// Work is deferred until a safer boundary.
    Defer,
    /// Work is paused while preserving a checkpoint or row.
    Pause,
    /// New work is denied with an explicit retry path.
    Deny,
    /// Work resumes gradually after pressure clears.
    StagedResume,
}

impl WorkloadAction {
    /// Stable token used by runtime and benchmark records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admit => "admit",
            Self::Throttle => "throttle",
            Self::Defer => "defer",
            Self::Pause => "pause",
            Self::Deny => "deny",
            Self::StagedResume => "staged_resume",
        }
    }

    /// True when this action changes user-visible cadence, freshness, or admission.
    pub const fn changes_behavior(self) -> bool {
        !matches!(self, Self::Admit)
    }
}

/// User-visible capability state after a workload decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisibleCapabilityState {
    /// Capability is ready under the current budget.
    Ready,
    /// Capability is warming or staged.
    Warming,
    /// Capability is usable with partial scope or freshness.
    Partial,
    /// Capability is available with reduced guarantees.
    Degraded,
    /// Capability is temporarily overloaded or denied.
    Overloaded,
}

impl VisibleCapabilityState {
    /// Stable token used in capability rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Warming => "warming",
            Self::Partial => "partial",
            Self::Degraded => "degraded",
            Self::Overloaded => "overloaded",
        }
    }
}

/// Visible explanation contract projected by shell surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisibleExplanationContract {
    /// Runtime status pill naming the active state and source.
    EfficiencyStatePill,
    /// Capability row explaining throttled or paused work.
    ThrottledCapabilityRow,
    /// Chip showing partial scope.
    PartialScopeChip,
    /// Badge showing a stale snapshot.
    StaleSnapshotBadge,
    /// Durable activity row for deferred work.
    DurableDeferredJobRow,
    /// Recovery note while queues resume in stages.
    RecoveryNote,
}

impl VisibleExplanationContract {
    /// Stable token used in fixtures and support evidence.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EfficiencyStatePill => "efficiency_state_pill",
            Self::ThrottledCapabilityRow => "throttled_capability_row",
            Self::PartialScopeChip => "partial_scope_chip",
            Self::StaleSnapshotBadge => "stale_snapshot_badge",
            Self::DurableDeferredJobRow => "durable_deferred_job_row",
            Self::RecoveryNote => "recovery_note",
        }
    }
}

/// State transition event emitted by the efficiency runtime hook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyStateTransitionEvent {
    /// Stable event id.
    pub event_id: String,
    /// Previous efficiency state.
    pub previous_state: String,
    /// New efficiency state.
    pub new_state: String,
    /// Source signal that caused the transition.
    pub source_signal: String,
    /// Human-readable reason.
    pub reason: String,
    /// Top contributors that were throttled or staged.
    pub top_throttled_contributors: Vec<String>,
    /// User or admin override, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_or_admin_override: Option<String>,
    /// Observation timestamp.
    pub observed_at: String,
}

/// One visible row explaining changed capability behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThrottledCapabilityRow {
    /// Stable capability id.
    pub capability_id: String,
    /// Human-readable capability label.
    pub capability_label: String,
    /// Source subsystem label.
    pub host_owner_label: String,
    /// Current capability state token.
    pub visible_state: String,
    /// Current state sentence.
    pub current_state_label: String,
    /// User impact sentence.
    pub user_impact_label: String,
    /// Visible explanation contracts this row satisfies.
    pub visible_explanation_contracts: Vec<String>,
    /// User-facing actions exposed by the row.
    pub actions: Vec<String>,
    /// True when recovery is automatic once pressure clears.
    pub automatic_recovery: bool,
    /// True when a user override is offered.
    pub override_allowed: bool,
    /// Policy source that blocks override, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_blocked_override_ref: Option<String>,
}

/// Workload-budget decision emitted by the efficiency runtime hook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkloadBudgetDecision {
    /// Stable record kind.
    pub record_kind: String,
    /// Stable event id.
    pub event_id: String,
    /// Workload family token.
    pub workload_id: String,
    /// Shared work class token.
    pub work_class: String,
    /// Queue lane token.
    pub queue_lane: String,
    /// Active efficiency state.
    pub efficiency_state: String,
    /// Chosen budget action.
    pub action: String,
    /// Checkpoint state or checkpoint requirement.
    pub checkpoint_state: String,
    /// Visible explanation contracts this decision requires.
    pub visible_explanation_contracts: Vec<String>,
    /// Source pressure token.
    pub pressure_source: String,
    /// The row shell or activity surfaces can render.
    pub capability_row: ThrottledCapabilityRow,
    /// Protected interactions this decision may not narrow.
    pub protected_interactions_preserved: Vec<String>,
    /// True when save durability and user-owned artifacts are preserved.
    pub durability_preserved: bool,
    /// Observation timestamp.
    pub observed_at: String,
}

impl WorkloadBudgetDecision {
    /// Builds a decision for the active efficiency state.
    pub fn for_state(
        workload: WorkloadFamily,
        state: EfficiencyState,
        source: EfficiencyPressureSource,
        observed_at: impl Into<String>,
    ) -> Self {
        let (action, visible_state, contracts) = workload_policy(workload, state);
        let action_token = action.as_str().to_owned();
        let contract_tokens = contracts
            .iter()
            .map(|contract| contract.as_str().to_owned())
            .collect::<Vec<_>>();
        let capability_row = capability_row_for(
            workload,
            state,
            source,
            action,
            visible_state,
            contract_tokens.clone(),
        );
        Self {
            record_kind: WORKLOAD_BUDGET_DECISION_RECORD_KIND.to_owned(),
            event_id: "workload_budget_decision".to_owned(),
            workload_id: workload.as_str().to_owned(),
            work_class: workload.work_class().as_str().to_owned(),
            queue_lane: workload.queue_lane().as_str().to_owned(),
            efficiency_state: state.as_str().to_owned(),
            action: action_token,
            checkpoint_state: checkpoint_state_for(workload, action).to_owned(),
            visible_explanation_contracts: contract_tokens,
            pressure_source: source.as_str().to_owned(),
            capability_row,
            protected_interactions_preserved: protected_interactions(),
            durability_preserved: true,
            observed_at: observed_at.into(),
        }
    }

    /// True when the decision changed cadence, freshness, or admission.
    pub fn changed_behavior(&self) -> bool {
        self.action != WorkloadAction::Admit.as_str()
    }

    /// Builds a durable activity-center row for an indexing budget decision.
    pub fn indexing_activity_row(
        &self,
        workspace_id: impl AsRef<str>,
        minted_at: impl Into<String>,
    ) -> Option<ActivityRow> {
        if self.workload_id != WorkloadFamily::IndexingRefresh.as_str() {
            return None;
        }
        let minted_at = minted_at.into();
        let workspace_id = workspace_id.as_ref();
        let row_id = format!("ux:activity-row:efficiency:indexing:{workspace_id}");
        let durable_job_id = format!("ux:durable-job:efficiency:indexing:{workspace_id}");
        let target_identity = row_id.clone();
        let state_class = if self.action == WorkloadAction::Pause.as_str()
            || self.action == WorkloadAction::Defer.as_str()
        {
            ActivityRowStateClass::QueuedWaiting
        } else {
            ActivityRowStateClass::Running
        };
        Some(ActivityRow::from_input(ActivityRowInput {
            activity_row_id: row_id.clone(),
            durable_job_id: durable_job_id.clone(),
            canonical_event_id: format!("ux:event:efficiency:indexing:{workspace_id}"),
            canonical_object_target_ref: durable_job_id.clone(),
            exact_reopen_identity_ref: target_identity.clone(),
            job_family: ActivityJobFamily::Indexing,
            source_subsystem: SourceSubsystem::RuntimePowerManager,
            actor_identity_ref: "id:actor:system:runtime_power_manager".to_owned(),
            actor_or_subsystem_label: "Runtime power manager".to_owned(),
            execution_origin_class: "system_background".to_owned(),
            severity_class: SeverityClass::Degraded,
            privacy_class: PrivacyClass::WorkspaceSensitive,
            summary_label: "Indexing refresh throttled by efficiency state".to_owned(),
            target_label: "Indexing refresh".to_owned(),
            target_scope_label: "Active workspace".to_owned(),
            state_class,
            progress: ActivityRowProgress {
                forms: vec![
                    ActivityProgressForm::QueueReason,
                    ActivityProgressForm::PhaseOnly,
                ],
                phase_label: self.capability_row.current_state_label.clone(),
                progress_bar: None,
                queue_reason_label: Some(self.capability_row.user_impact_label.clone()),
                approval_source_label: None,
                actor_or_subsystem_label: "Indexer".to_owned(),
                age_label: "current".to_owned(),
                indeterminate_reason_label: None,
                expected_boundary_class: "local".to_owned(),
                cancellability_class: ActivityCancellabilityClass::NotCancellable,
                detail_or_evidence_ref: Some(format!(
                    "efficiency-state:{}:{}",
                    self.efficiency_state, self.workload_id
                )),
            },
            timeline: ActivityRowTimeline {
                minted_at: minted_at.clone(),
                queued_at: Some(minted_at.clone()),
                started_at: None,
                last_observed_at: self.observed_at.clone(),
                finished_at: None,
                archived_at: None,
                superseded_by_row_id_ref: None,
                retention_label: "Retained until resolved or archived".to_owned(),
            },
            impact: ActivityRowImpact {
                affects_cost: false,
                affects_policy: false,
                affects_network: false,
                affects_trust: false,
                affects_provider_state: false,
                affects_recovery_posture: false,
                detail_or_evidence_required: true,
                impact_summary_sentence:
                    "The indexer keeps open files and hot-set navigation truthful while wider refresh is reduced."
                        .to_owned(),
            },
            actions: vec![ActivityRowAction::open_details(
                format!("action:efficiency:indexing:{workspace_id}:open"),
                "Open efficiency details",
                target_identity,
            )],
            display: ActivityRowDisplayState {
                collapse_state: ActivityRowCollapseState::CollapsedSummary,
                can_expand_inline: true,
                expand_reveals_label: "state, pressure source, and paused work".to_owned(),
            },
            support_link: ActivityRowSupportLink {
                exportable: true,
                support_pack_item_id: Some(format!("support.item.efficiency.indexing.{workspace_id}")),
                bundle_member_path_ref: Some(format!(
                    "manifest/efficiency/indexing/{workspace_id}.json"
                )),
                redaction_class: RedactionClass::MetadataSafeDefault,
                raw_private_material_excluded: true,
                export_field_refs: vec![
                    "export.efficiency.state".to_owned(),
                    "export.efficiency.workload".to_owned(),
                    "export.activity.reopen".to_owned(),
                ],
            },
            git_review_context: None,
            occurrence_count: 1,
        }))
    }
}

/// Visibility state for a shell render surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisibilityState {
    /// Surface is visible and focused.
    VisibleFocused,
    /// Surface is visible but not focused.
    VisibleBackground,
    /// Window is occluded.
    OccludedWindow,
    /// Pane or tab is hidden.
    HiddenTab,
    /// Split pane is collapsed.
    CollapsedSplit,
    /// Detached window is off-screen.
    DetachedOffscreen,
}

impl VisibilityState {
    /// Stable token used by render and benchmark records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VisibleFocused => "visible_focused",
            Self::VisibleBackground => "visible_background",
            Self::OccludedWindow => "occluded_window",
            Self::HiddenTab => "hidden_tab",
            Self::CollapsedSplit => "collapsed_split",
            Self::DetachedOffscreen => "detached_offscreen",
        }
    }

    /// True when committed paint and animation must be suppressed.
    pub const fn is_hidden_or_offscreen(self) -> bool {
        matches!(
            self,
            Self::OccludedWindow | Self::HiddenTab | Self::CollapsedSplit | Self::DetachedOffscreen
        )
    }
}

/// Protected shell surface class used by render suppression audits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtectedSurfaceClass {
    /// Editor viewport.
    EditorViewport,
    /// Terminal viewport.
    TerminalViewport,
    /// Diff or review viewport.
    DiffReviewViewport,
    /// Search result-list viewport.
    SearchResultsViewport,
    /// Task or log viewport.
    TaskLogViewport,
    /// Preview viewport.
    PreviewViewport,
    /// Graph panel.
    GraphPanel,
}

impl ProtectedSurfaceClass {
    /// Stable token used by render and benchmark records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorViewport => "editor_viewport",
            Self::TerminalViewport => "terminal_viewport",
            Self::DiffReviewViewport => "diff_review_viewport",
            Self::SearchResultsViewport => "search_results_viewport",
            Self::TaskLogViewport => "task_log_viewport",
            Self::PreviewViewport => "preview_viewport",
            Self::GraphPanel => "graph_panel",
        }
    }
}

/// Input describing one render or animation request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderVisibilityInput {
    /// Stable surface id.
    pub surface_id: String,
    /// Protected surface class.
    pub surface_class: ProtectedSurfaceClass,
    /// Visibility state.
    pub visibility_state: VisibilityState,
    /// Paint passes requested before suppression.
    pub requested_paint_count: u32,
    /// Animation ticks requested before suppression.
    pub requested_animation_tick_count: u32,
    /// True when correctness polling may continue without render budget.
    pub correctness_polling_required: bool,
}

/// One committed render sample used by hidden-pane audits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderVisibilitySample {
    /// Stable surface id.
    pub surface_id: String,
    /// Protected surface class token.
    pub surface_class: String,
    /// Visibility state token.
    pub visibility_state: String,
    /// Committed paint count after suppression.
    pub committed_paint_count: u32,
    /// Hidden-pane render work counter.
    pub hidden_pane_work: u32,
    /// Off-screen suppression audit counter.
    pub offscreen_suppression_eligible: u32,
}

/// Render-visibility decision emitted by the efficiency runtime hook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderVisibilityDecision {
    /// Stable record kind.
    pub record_kind: String,
    /// Stable event id.
    pub event_id: String,
    /// Stable surface id.
    pub surface_id: String,
    /// Protected surface class token.
    pub surface_class: String,
    /// Visibility state token.
    pub visibility_state: String,
    /// Active efficiency state.
    pub efficiency_state: String,
    /// True when paint was suppressed.
    pub paint_suppressed: bool,
    /// True when animation ticks were suppressed.
    pub animation_suppressed: bool,
    /// Polling mode after suppression.
    pub polling_mode: String,
    /// Committed paint count after suppression.
    pub committed_paint_count: u32,
    /// Hidden-pane work counter after suppression.
    pub hidden_pane_work: u32,
    /// Off-screen suppression audit counter after suppression.
    pub offscreen_suppression_eligible: u32,
    /// True when this decision violates hidden-pane policy.
    pub hidden_pane_violation: bool,
}

impl RenderVisibilityDecision {
    /// Converts the decision into an audit sample.
    pub fn as_sample(&self) -> RenderVisibilitySample {
        RenderVisibilitySample {
            surface_id: self.surface_id.clone(),
            surface_class: self.surface_class.clone(),
            visibility_state: self.visibility_state.clone(),
            committed_paint_count: self.committed_paint_count,
            hidden_pane_work: self.hidden_pane_work,
            offscreen_suppression_eligible: self.offscreen_suppression_eligible,
        }
    }
}

/// Hidden-pane render audit summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenPaneRenderAudit {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Number of surfaces audited.
    pub audited_surface_count: usize,
    /// Number of hidden or off-screen surfaces audited.
    pub hidden_surface_count: usize,
    /// Visible-pane work count.
    pub visible_pane_work: u32,
    /// Hidden-pane work count.
    pub hidden_pane_work: u32,
    /// Off-screen suppression audit count.
    pub offscreen_suppression_eligible: u32,
    /// Number of hidden-pane render violations.
    pub hidden_pane_render_violation_count: u32,
    /// True when no hidden-pane render violations were found.
    pub passes_hidden_pane_policy: bool,
    /// Samples included in the audit.
    pub samples: Vec<RenderVisibilitySample>,
}

impl HiddenPaneRenderAudit {
    /// Builds an audit from render-visibility decisions.
    pub fn from_decisions(decisions: &[RenderVisibilityDecision]) -> Self {
        let samples = decisions
            .iter()
            .map(RenderVisibilityDecision::as_sample)
            .collect::<Vec<_>>();
        Self::from_samples(samples)
    }

    /// Builds an audit from committed render samples.
    pub fn from_samples(samples: Vec<RenderVisibilitySample>) -> Self {
        let mut hidden_surface_count = 0usize;
        let mut visible_pane_work = 0u32;
        let mut hidden_pane_work = 0u32;
        let mut offscreen_suppression_eligible = 0u32;
        let mut hidden_pane_render_violation_count = 0u32;
        for sample in &samples {
            let hidden = is_hidden_visibility_token(&sample.visibility_state);
            if hidden {
                hidden_surface_count = hidden_surface_count.saturating_add(1);
                hidden_pane_work = hidden_pane_work.saturating_add(sample.hidden_pane_work);
                offscreen_suppression_eligible = offscreen_suppression_eligible
                    .saturating_add(sample.offscreen_suppression_eligible);
                if sample.committed_paint_count > 0 || sample.hidden_pane_work > 0 {
                    hidden_pane_render_violation_count =
                        hidden_pane_render_violation_count.saturating_add(1);
                }
            } else {
                visible_pane_work = visible_pane_work.saturating_add(sample.committed_paint_count);
            }
        }
        Self {
            record_kind: HIDDEN_PANE_RENDER_AUDIT_RECORD_KIND.to_owned(),
            schema_version: 1,
            audited_surface_count: samples.len(),
            hidden_surface_count,
            visible_pane_work,
            hidden_pane_work,
            offscreen_suppression_eligible,
            hidden_pane_render_violation_count,
            passes_hidden_pane_policy: hidden_pane_render_violation_count == 0,
            samples,
        }
    }
}

/// Status-bar projection for the active efficiency state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyStatusSnapshot {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Active efficiency state token.
    pub active_state: String,
    /// Pressure-source tokens.
    pub pressure_sources: Vec<String>,
    /// True when runtime behavior changed.
    pub behavior_changed: bool,
    /// Number of affected capability rows.
    pub affected_capability_count: usize,
    /// Label rendered as the current status value.
    pub current_value_label: String,
    /// Explanation shown in status details.
    pub explanation: String,
    /// Screen-reader label.
    pub accessibility_label: String,
    /// Primary command id for details.
    pub primary_command_id: String,
    /// Surface ref opened by details.
    pub opens_surface_ref: String,
    /// Degraded-state token for shell chrome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_token: Option<String>,
    /// True when the status row should be treated as recovery-critical.
    pub is_recovery_critical: bool,
}

/// Durability invariants preserved by efficiency adaptation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyDurabilityInvariants {
    /// True when save durability remains protected.
    pub save_durability_preserved: bool,
    /// True when dirty buffers remain protected.
    pub dirty_buffers_preserved: bool,
    /// True when user-owned artifacts remain present and attributable.
    pub user_owned_artifacts_preserved: bool,
    /// Human-readable invariant summary.
    pub invariant_summary: String,
}

impl Default for EfficiencyDurabilityInvariants {
    fn default() -> Self {
        Self {
            save_durability_preserved: true,
            dirty_buffers_preserved: true,
            user_owned_artifacts_preserved: true,
            invariant_summary:
                "Efficiency adaptation may pause optional work but may not skip save durability, lose dirty buffers, or hide user-owned artifacts."
                    .to_owned(),
        }
    }
}

/// Exportable shell snapshot of the active efficiency state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyStateSnapshot {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Active workspace id.
    pub workspace_id: String,
    /// Active efficiency state token.
    pub active_state: String,
    /// Pressure-source tokens.
    pub pressure_sources: Vec<String>,
    /// True when runtime behavior changed.
    pub behavior_changed: bool,
    /// Status projection when shell chrome should show one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EfficiencyStatusSnapshot>,
    /// Rows explaining throttled or paused capabilities.
    pub throttled_capabilities: Vec<ThrottledCapabilityRow>,
    /// Workload decisions emitted for this snapshot.
    pub workload_decisions: Vec<WorkloadBudgetDecision>,
    /// Hidden-pane render audit.
    pub hidden_pane_audit: HiddenPaneRenderAudit,
    /// Protected interactions preserved by the adaptation.
    pub protected_interactions_preserved: Vec<String>,
    /// Durability invariants preserved by the adaptation.
    pub durability_invariants: EfficiencyDurabilityInvariants,
    /// Observation timestamp.
    pub observed_at: String,
}

impl EfficiencyStateSnapshot {
    /// Builds an exportable snapshot from workload and render decisions.
    pub fn from_decisions(
        workspace_id: impl Into<String>,
        state: EfficiencyState,
        pressure_sources: Vec<EfficiencyPressureSource>,
        behavior_changed: bool,
        workload_decisions: Vec<WorkloadBudgetDecision>,
        hidden_pane_audit: HiddenPaneRenderAudit,
        observed_at: impl Into<String>,
    ) -> Self {
        let observed_at = observed_at.into();
        let throttled_capabilities = workload_decisions
            .iter()
            .filter(|decision| decision.changed_behavior())
            .map(|decision| decision.capability_row.clone())
            .collect::<Vec<_>>();
        let status = build_status_projection(
            state,
            &pressure_sources,
            behavior_changed,
            throttled_capabilities.len(),
        );
        Self {
            record_kind: EFFICIENCY_STATE_SNAPSHOT_RECORD_KIND.to_owned(),
            schema_version: EFFICIENCY_STATE_SNAPSHOT_SCHEMA_VERSION,
            workspace_id: workspace_id.into(),
            active_state: state.as_str().to_owned(),
            pressure_sources: pressure_sources
                .iter()
                .map(|source| source.as_str().to_owned())
                .collect(),
            behavior_changed,
            status,
            throttled_capabilities,
            workload_decisions,
            hidden_pane_audit,
            protected_interactions_preserved: protected_interactions(),
            durability_invariants: EfficiencyDurabilityInvariants::default(),
            observed_at,
        }
    }

    /// True when the snapshot proves the no-hidden-data-loss rule.
    pub fn preserves_durability_truth(&self) -> bool {
        self.durability_invariants.save_durability_preserved
            && self.durability_invariants.dirty_buffers_preserved
            && self.durability_invariants.user_owned_artifacts_preserved
            && self
                .workload_decisions
                .iter()
                .all(|decision| decision.durability_preserved)
    }
}

/// Stateful alpha runtime that emits efficiency telemetry hooks.
#[derive(Debug, Clone)]
pub struct EfficiencyStateRuntime {
    current_state: EfficiencyState,
    events: Vec<EfficiencyStateTransitionEvent>,
}

impl Default for EfficiencyStateRuntime {
    fn default() -> Self {
        Self {
            current_state: EfficiencyState::Nominal,
            events: Vec::new(),
        }
    }
}

impl EfficiencyStateRuntime {
    /// Builds a runtime in `Nominal` state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the current efficiency state.
    pub const fn current_state(&self) -> EfficiencyState {
        self.current_state
    }

    /// Emits a state-transition event and updates the runtime state.
    pub fn transition_to(
        &mut self,
        new_state: EfficiencyState,
        source: EfficiencyPressureSource,
        reason: impl Into<String>,
        observed_at: impl Into<String>,
    ) -> EfficiencyStateTransitionEvent {
        let previous = self.current_state;
        self.current_state = new_state;
        let event = EfficiencyStateTransitionEvent {
            event_id: "efficiency_state_transition".to_owned(),
            previous_state: previous.as_str().to_owned(),
            new_state: new_state.as_str().to_owned(),
            source_signal: source.as_str().to_owned(),
            reason: reason.into(),
            top_throttled_contributors: default_throttled_contributors(new_state),
            user_or_admin_override: None,
            observed_at: observed_at.into(),
        };
        self.events.push(event.clone());
        event
    }

    /// Emits a workload-budget decision for the current state.
    pub fn decide_workload(
        &self,
        workload: WorkloadFamily,
        source: EfficiencyPressureSource,
        observed_at: impl Into<String>,
    ) -> WorkloadBudgetDecision {
        WorkloadBudgetDecision::for_state(workload, self.current_state, source, observed_at)
    }

    /// Emits a render-visibility decision for the current state.
    pub fn decide_render(&self, input: RenderVisibilityInput) -> RenderVisibilityDecision {
        let hidden = input.visibility_state.is_hidden_or_offscreen();
        let paint_suppressed = hidden && input.requested_paint_count > 0;
        let animation_suppressed = hidden && input.requested_animation_tick_count > 0;
        let committed_paint_count = if hidden {
            0
        } else {
            input.requested_paint_count
        };
        let polling_mode = if hidden {
            if input.correctness_polling_required {
                "correctness_only"
            } else {
                "paused"
            }
        } else {
            "within_visible_budget"
        };
        RenderVisibilityDecision {
            record_kind: RENDER_VISIBILITY_DECISION_RECORD_KIND.to_owned(),
            event_id: "render_visibility_decision".to_owned(),
            surface_id: input.surface_id,
            surface_class: input.surface_class.as_str().to_owned(),
            visibility_state: input.visibility_state.as_str().to_owned(),
            efficiency_state: self.current_state.as_str().to_owned(),
            paint_suppressed,
            animation_suppressed,
            polling_mode: polling_mode.to_owned(),
            committed_paint_count,
            hidden_pane_work: 0,
            offscreen_suppression_eligible: 0,
            hidden_pane_violation: false,
        }
    }

    /// Returns transition events emitted by this runtime.
    pub fn transition_events(&self) -> &[EfficiencyStateTransitionEvent] {
        &self.events
    }
}

fn workload_policy(
    workload: WorkloadFamily,
    state: EfficiencyState,
) -> (
    WorkloadAction,
    VisibleCapabilityState,
    Vec<VisibleExplanationContract>,
) {
    use EfficiencyState as State;
    use VisibleCapabilityState as Visible;
    use VisibleExplanationContract as Contract;
    use WorkloadAction as Action;
    match (workload, state) {
        (_, State::Nominal) => (Action::Admit, Visible::Ready, vec![]),
        (WorkloadFamily::AiWarmup, State::EfficiencyAware) => (
            Action::Defer,
            Visible::Degraded,
            vec![
                Contract::EfficiencyStatePill,
                Contract::ThrottledCapabilityRow,
            ],
        ),
        (WorkloadFamily::AiWarmup, State::ThermalConstrained) => (
            Action::Pause,
            Visible::Degraded,
            vec![
                Contract::EfficiencyStatePill,
                Contract::ThrottledCapabilityRow,
            ],
        ),
        (WorkloadFamily::AiWarmup, State::ProtectCore) => (
            Action::Deny,
            Visible::Overloaded,
            vec![
                Contract::EfficiencyStatePill,
                Contract::ThrottledCapabilityRow,
            ],
        ),
        (WorkloadFamily::AiWarmup, State::Recovery) => (
            Action::StagedResume,
            Visible::Warming,
            vec![Contract::RecoveryNote, Contract::ThrottledCapabilityRow],
        ),
        (WorkloadFamily::SpeculativePrefetch, State::EfficiencyAware) => (
            Action::Throttle,
            Visible::Warming,
            vec![
                Contract::EfficiencyStatePill,
                Contract::ThrottledCapabilityRow,
                Contract::PartialScopeChip,
            ],
        ),
        (WorkloadFamily::SpeculativePrefetch, State::ThermalConstrained) => (
            Action::Pause,
            Visible::Partial,
            vec![
                Contract::EfficiencyStatePill,
                Contract::ThrottledCapabilityRow,
                Contract::PartialScopeChip,
            ],
        ),
        (WorkloadFamily::SpeculativePrefetch, State::ProtectCore) => (
            Action::Deny,
            Visible::Overloaded,
            vec![
                Contract::EfficiencyStatePill,
                Contract::ThrottledCapabilityRow,
            ],
        ),
        (WorkloadFamily::SpeculativePrefetch, State::Recovery) => (
            Action::StagedResume,
            Visible::Warming,
            vec![Contract::RecoveryNote, Contract::PartialScopeChip],
        ),
        (WorkloadFamily::UploadTransfer, State::EfficiencyAware) => (
            Action::Defer,
            Visible::Ready,
            vec![
                Contract::ThrottledCapabilityRow,
                Contract::DurableDeferredJobRow,
            ],
        ),
        (WorkloadFamily::UploadTransfer, State::ThermalConstrained) => (
            Action::Pause,
            Visible::Degraded,
            vec![
                Contract::ThrottledCapabilityRow,
                Contract::DurableDeferredJobRow,
            ],
        ),
        (WorkloadFamily::UploadTransfer, State::ProtectCore) => (
            Action::Deny,
            Visible::Overloaded,
            vec![
                Contract::EfficiencyStatePill,
                Contract::ThrottledCapabilityRow,
                Contract::DurableDeferredJobRow,
            ],
        ),
        (WorkloadFamily::UploadTransfer, State::Recovery) => (
            Action::StagedResume,
            Visible::Warming,
            vec![Contract::RecoveryNote, Contract::DurableDeferredJobRow],
        ),
        (WorkloadFamily::NonEssentialAnimation, State::EfficiencyAware) => (
            Action::Throttle,
            Visible::Ready,
            vec![Contract::EfficiencyStatePill],
        ),
        (WorkloadFamily::NonEssentialAnimation, State::ThermalConstrained) => (
            Action::Throttle,
            Visible::Ready,
            vec![Contract::EfficiencyStatePill],
        ),
        (WorkloadFamily::NonEssentialAnimation, State::ProtectCore) => (
            Action::Pause,
            Visible::Ready,
            vec![Contract::EfficiencyStatePill],
        ),
        (WorkloadFamily::NonEssentialAnimation, State::Recovery) => (
            Action::StagedResume,
            Visible::Ready,
            vec![Contract::RecoveryNote],
        ),
        (WorkloadFamily::IndexingRefresh, State::EfficiencyAware) => (
            Action::Throttle,
            Visible::Partial,
            vec![Contract::ThrottledCapabilityRow, Contract::PartialScopeChip],
        ),
        (WorkloadFamily::IndexingRefresh, State::ThermalConstrained) => (
            Action::Throttle,
            Visible::Partial,
            vec![Contract::ThrottledCapabilityRow, Contract::PartialScopeChip],
        ),
        (WorkloadFamily::IndexingRefresh, State::ProtectCore) => (
            Action::Pause,
            Visible::Overloaded,
            vec![Contract::ThrottledCapabilityRow, Contract::PartialScopeChip],
        ),
        (WorkloadFamily::IndexingRefresh, State::Recovery) => (
            Action::StagedResume,
            Visible::Warming,
            vec![Contract::RecoveryNote, Contract::PartialScopeChip],
        ),
        (WorkloadFamily::ExtensionPolling, State::EfficiencyAware) => (
            Action::Throttle,
            Visible::Degraded,
            vec![Contract::ThrottledCapabilityRow],
        ),
        (WorkloadFamily::ExtensionPolling, State::ThermalConstrained) => (
            Action::Pause,
            Visible::Degraded,
            vec![Contract::ThrottledCapabilityRow],
        ),
        (WorkloadFamily::ExtensionPolling, State::ProtectCore) => (
            Action::Pause,
            Visible::Overloaded,
            vec![Contract::ThrottledCapabilityRow],
        ),
        (WorkloadFamily::ExtensionPolling, State::Recovery) => (
            Action::StagedResume,
            Visible::Warming,
            vec![Contract::RecoveryNote, Contract::ThrottledCapabilityRow],
        ),
        (WorkloadFamily::PreviewRefresh, State::EfficiencyAware) => (
            Action::Throttle,
            Visible::Degraded,
            vec![
                Contract::ThrottledCapabilityRow,
                Contract::StaleSnapshotBadge,
            ],
        ),
        (WorkloadFamily::PreviewRefresh, State::ThermalConstrained) => (
            Action::Pause,
            Visible::Degraded,
            vec![
                Contract::ThrottledCapabilityRow,
                Contract::StaleSnapshotBadge,
            ],
        ),
        (WorkloadFamily::PreviewRefresh, State::ProtectCore) => (
            Action::Pause,
            Visible::Overloaded,
            vec![
                Contract::ThrottledCapabilityRow,
                Contract::StaleSnapshotBadge,
            ],
        ),
        (WorkloadFamily::PreviewRefresh, State::Recovery) => (
            Action::StagedResume,
            Visible::Warming,
            vec![Contract::RecoveryNote, Contract::StaleSnapshotBadge],
        ),
        (WorkloadFamily::GraphEnrichment, State::EfficiencyAware) => (
            Action::Throttle,
            Visible::Partial,
            vec![Contract::ThrottledCapabilityRow, Contract::PartialScopeChip],
        ),
        (WorkloadFamily::GraphEnrichment, State::ThermalConstrained) => (
            Action::Throttle,
            Visible::Partial,
            vec![Contract::ThrottledCapabilityRow, Contract::PartialScopeChip],
        ),
        (WorkloadFamily::GraphEnrichment, State::ProtectCore) => (
            Action::Pause,
            Visible::Overloaded,
            vec![Contract::ThrottledCapabilityRow, Contract::PartialScopeChip],
        ),
        (WorkloadFamily::GraphEnrichment, State::Recovery) => (
            Action::StagedResume,
            Visible::Warming,
            vec![Contract::RecoveryNote, Contract::PartialScopeChip],
        ),
        (WorkloadFamily::RemoteSessionHelper, State::EfficiencyAware) => (
            Action::Throttle,
            Visible::Degraded,
            vec![Contract::ThrottledCapabilityRow],
        ),
        (WorkloadFamily::RemoteSessionHelper, State::ThermalConstrained) => (
            Action::Throttle,
            Visible::Degraded,
            vec![Contract::ThrottledCapabilityRow],
        ),
        (WorkloadFamily::RemoteSessionHelper, State::ProtectCore) => (
            Action::Pause,
            Visible::Degraded,
            vec![
                Contract::EfficiencyStatePill,
                Contract::ThrottledCapabilityRow,
            ],
        ),
        (WorkloadFamily::RemoteSessionHelper, State::Recovery) => (
            Action::StagedResume,
            Visible::Warming,
            vec![Contract::RecoveryNote, Contract::ThrottledCapabilityRow],
        ),
    }
}

fn capability_row_for(
    workload: WorkloadFamily,
    state: EfficiencyState,
    source: EfficiencyPressureSource,
    action: WorkloadAction,
    visible_state: VisibleCapabilityState,
    visible_explanation_contracts: Vec<String>,
) -> ThrottledCapabilityRow {
    let current_state_label = match action {
        WorkloadAction::Admit => {
            format!("{} is running within the current budget.", workload.label())
        }
        WorkloadAction::Throttle => format!(
            "{} is reduced while {} is active.",
            workload.label(),
            source.label()
        ),
        WorkloadAction::Defer => format!(
            "{} is deferred while {} is active.",
            workload.label(),
            source.label()
        ),
        WorkloadAction::Pause => format!(
            "{} is paused while {} is active.",
            workload.label(),
            source.label()
        ),
        WorkloadAction::Deny => format!(
            "{} is not starting while {} protects core work.",
            workload.label(),
            source.label()
        ),
        WorkloadAction::StagedResume => {
            format!(
                "{} is resuming in stages after pressure cleared.",
                workload.label()
            )
        }
    };
    let user_impact_label = user_impact_for(workload, state, action);
    ThrottledCapabilityRow {
        capability_id: format!("capability.efficiency.{}", workload.as_str()),
        capability_label: workload.label().to_owned(),
        host_owner_label: source_subsystem_label(workload.source_subsystem()).to_owned(),
        visible_state: visible_state.as_str().to_owned(),
        current_state_label,
        user_impact_label,
        visible_explanation_contracts,
        actions: actions_for(workload, action),
        automatic_recovery: !matches!(action, WorkloadAction::Deny),
        override_allowed: matches!(
            source,
            EfficiencyPressureSource::Battery | EfficiencyPressureSource::UserLowPowerMode
        ) && !matches!(state, EfficiencyState::ProtectCore),
        policy_blocked_override_ref: matches!(source, EfficiencyPressureSource::PolicyCap)
            .then(|| "policy:efficiency.override_blocked".to_owned()),
    }
}

fn user_impact_for(
    workload: WorkloadFamily,
    state: EfficiencyState,
    action: WorkloadAction,
) -> String {
    match workload {
        WorkloadFamily::AiWarmup => {
            "Assistant warmups may stay cold; no cached or stale model context is relabeled current."
                .to_owned()
        }
        WorkloadFamily::SpeculativePrefetch => {
            "Hot-local data remains usable; full-scope freshness is warming or partial until refresh resumes."
                .to_owned()
        }
        WorkloadFamily::UploadTransfer => {
            "Queued transfers remain unsent and attributable until an allowed foreground send or staged resume."
                .to_owned()
        }
        WorkloadFamily::NonEssentialAnimation => {
            "Focus, cursor position, and state conveyance remain intact while decorative motion stops."
                .to_owned()
        }
        WorkloadFamily::IndexingRefresh => match state {
            EfficiencyState::ProtectCore => {
                "Open files remain current. Whole-workspace indexing is paused and broad results may be partial."
                    .to_owned()
            }
            _ => {
                "Open files and hot-set navigation remain current while whole-workspace refresh uses a reduced lane."
                    .to_owned()
            }
        },
        WorkloadFamily::ExtensionPolling => {
            "User-invoked extension commands remain attributable; optional background polling is shown as throttled."
                .to_owned()
        }
        WorkloadFamily::PreviewRefresh => {
            "Preview surfaces keep the last truthful snapshot and show stale or paused refresh truth."
                .to_owned()
        }
        WorkloadFamily::GraphEnrichment => {
            "Graph, search, AI, and impact surfaces label hot-set-only or partial scope until enrichment resumes."
                .to_owned()
        }
        WorkloadFamily::RemoteSessionHelper => {
            "Reconnect, heartbeat, or sync helpers back off within freshness bounds without hiding remote state."
                .to_owned()
        }
    }
    .tap_if(matches!(action, WorkloadAction::Deny), |message| {
        message.push_str(" Retry remains explicit and does not replay hidden side effects.");
    })
}

trait TapIf {
    fn tap_if(self, condition: bool, f: impl FnOnce(&mut String)) -> String;
}

impl TapIf for String {
    fn tap_if(mut self, condition: bool, f: impl FnOnce(&mut String)) -> String {
        if condition {
            f(&mut self);
        }
        self
    }
}

fn actions_for(workload: WorkloadFamily, action: WorkloadAction) -> Vec<String> {
    let mut actions = vec!["Open details".to_owned()];
    if matches!(
        action,
        WorkloadAction::Deny | WorkloadAction::Pause | WorkloadAction::Defer
    ) {
        actions.push("Resume when normal".to_owned());
    }
    if matches!(
        workload,
        WorkloadFamily::IndexingRefresh
            | WorkloadFamily::PreviewRefresh
            | WorkloadFamily::GraphEnrichment
    ) {
        actions.push("Keep throttled".to_owned());
    }
    actions
}

fn checkpoint_state_for(workload: WorkloadFamily, action: WorkloadAction) -> &'static str {
    match (workload, action) {
        (_, WorkloadAction::Admit) => "not_applicable",
        (WorkloadFamily::IndexingRefresh, _) => "phase_boundary_checkpoint_preserved",
        (WorkloadFamily::UploadTransfer, _) => "queued_transfer_preserved",
        (WorkloadFamily::GraphEnrichment, _) => "hot_set_checkpoint_preserved",
        (WorkloadFamily::PreviewRefresh, _) => "last_truthful_snapshot_preserved",
        (_, WorkloadAction::StagedResume) => "staged_resume_checkpoint",
        _ => "checkpoint_or_cancel_safe_boundary_required",
    }
}

fn build_status_projection(
    state: EfficiencyState,
    sources: &[EfficiencyPressureSource],
    behavior_changed: bool,
    affected_capability_count: usize,
) -> Option<EfficiencyStatusSnapshot> {
    if !state.status_item_required() || !behavior_changed {
        return None;
    }
    let pressure_sources = sources
        .iter()
        .map(|source| source.as_str().to_owned())
        .collect::<Vec<_>>();
    let source_label = sources
        .first()
        .map(|source| source.label())
        .unwrap_or("unknown pressure");
    let current_value_label = format!("{} · {source_label}", state.label());
    let explanation = format!(
        "{} changed background work because of {source_label}; {affected_capability_count} capability rows name what paused or reduced.",
        state.label()
    );
    Some(EfficiencyStatusSnapshot {
        record_kind: EFFICIENCY_STATUS_RECORD_KIND.to_owned(),
        schema_version: 1,
        active_state: state.as_str().to_owned(),
        pressure_sources,
        behavior_changed,
        affected_capability_count,
        current_value_label: current_value_label.clone(),
        explanation: explanation.clone(),
        accessibility_label: format!("Efficiency state: {current_value_label}. {explanation}"),
        primary_command_id: "cmd:runtime.efficiency_state.inspect".to_owned(),
        opens_surface_ref: "surface.runtime.efficiency_state".to_owned(),
        degraded_token: state.degraded_token().map(|token| token.token().to_owned()),
        is_recovery_critical: matches!(state, EfficiencyState::ProtectCore),
    })
}

fn protected_interactions() -> Vec<String> {
    [
        "typing",
        "save",
        "undo",
        "local_navigation",
        "terminal_correctness",
        "current_task_visibility",
    ]
    .iter()
    .map(|item| (*item).to_owned())
    .collect()
}

fn default_throttled_contributors(state: EfficiencyState) -> Vec<String> {
    match state {
        EfficiencyState::Nominal => Vec::new(),
        EfficiencyState::EfficiencyAware => vec![
            "ai_warmup".to_owned(),
            "speculative_prefetch".to_owned(),
            "upload_transfer".to_owned(),
            "non_essential_animation".to_owned(),
        ],
        EfficiencyState::ThermalConstrained => vec![
            "non_essential_animation".to_owned(),
            "indexing_refresh".to_owned(),
            "extension_polling".to_owned(),
            "preview_refresh".to_owned(),
            "graph_enrichment".to_owned(),
        ],
        EfficiencyState::ProtectCore => vec![
            "non_essential_animation".to_owned(),
            "ai_warmup".to_owned(),
            "speculative_prefetch".to_owned(),
            "upload_transfer".to_owned(),
            "indexing_refresh".to_owned(),
            "extension_polling".to_owned(),
            "preview_refresh".to_owned(),
            "graph_enrichment".to_owned(),
        ],
        EfficiencyState::Recovery => vec![
            "indexing_refresh".to_owned(),
            "graph_enrichment".to_owned(),
            "preview_refresh".to_owned(),
            "extension_polling".to_owned(),
        ],
    }
}

fn source_subsystem_label(source: SourceSubsystem) -> &'static str {
    match source {
        SourceSubsystem::Editor => "Editor",
        SourceSubsystem::Terminal => "Terminal",
        SourceSubsystem::ReviewAndDiff => "Review and diff",
        SourceSubsystem::PaletteAndSearch => "Palette and search",
        SourceSubsystem::InstallUpdateAttach => "Install/update",
        SourceSubsystem::AiApply => "AI runtime",
        SourceSubsystem::Collaboration => "Collaboration",
        SourceSubsystem::ProviderBearing => "Provider",
        SourceSubsystem::DocsHelpServiceHealth => "Docs/help",
        SourceSubsystem::SupportExport => "Support export",
        SourceSubsystem::BuildSystem => "Build system",
        SourceSubsystem::TestRunner => "Test runner",
        SourceSubsystem::DebugSession => "Debug session",
        SourceSubsystem::TaskRunner => "Task runner",
        SourceSubsystem::Indexer => "Indexer",
        SourceSubsystem::VfsSave => "VFS/save",
        SourceSubsystem::SyncMirror => "Sync/mirror",
        SourceSubsystem::NotebookKernel => "Notebook kernel",
        SourceSubsystem::RemoteAgent => "Remote agent",
        SourceSubsystem::ExtensionHost => "Extension host",
        SourceSubsystem::WorkspaceTrust => "Workspace trust",
        SourceSubsystem::PolicyResolver => "Policy resolver",
        SourceSubsystem::AdminPolicy => "Admin policy",
        SourceSubsystem::SecretBroker => "Secret broker",
        SourceSubsystem::RuntimePowerManager => "Runtime power manager",
        SourceSubsystem::Shell => "Shell",
    }
}

fn is_hidden_visibility_token(token: &str) -> bool {
    matches!(
        token,
        "occluded_window" | "hidden_tab" | "collapsed_split" | "detached_offscreen"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thermal_state_names_throttled_work_without_data_loss_escape_hatch() {
        let mut runtime = EfficiencyStateRuntime::new();
        runtime.transition_to(
            EfficiencyState::ThermalConstrained,
            EfficiencyPressureSource::ThermalPressure,
            "OS thermal pressure reported serious",
            "2026-05-14T08:40:00Z",
        );
        let decision = runtime.decide_workload(
            WorkloadFamily::IndexingRefresh,
            EfficiencyPressureSource::ThermalPressure,
            "2026-05-14T08:40:01Z",
        );
        assert_eq!(decision.action, "throttle");
        assert_eq!(decision.capability_row.visible_state, "partial");
        assert!(decision
            .protected_interactions_preserved
            .contains(&"typing".to_owned()));
        assert!(decision.durability_preserved);
    }

    #[test]
    fn hidden_render_request_is_suppressed_before_audit() {
        let mut runtime = EfficiencyStateRuntime::new();
        runtime.transition_to(
            EfficiencyState::EfficiencyAware,
            EfficiencyPressureSource::OsBatterySaver,
            "OS battery saver active",
            "2026-05-14T08:41:00Z",
        );
        let decision = runtime.decide_render(RenderVisibilityInput {
            surface_id: "terminal.hidden".to_owned(),
            surface_class: ProtectedSurfaceClass::TerminalViewport,
            visibility_state: VisibilityState::HiddenTab,
            requested_paint_count: 3,
            requested_animation_tick_count: 10,
            correctness_polling_required: true,
        });
        assert!(decision.paint_suppressed);
        assert!(decision.animation_suppressed);
        assert_eq!(decision.committed_paint_count, 0);
        let audit = HiddenPaneRenderAudit::from_decisions(&[decision]);
        assert_eq!(audit.hidden_pane_render_violation_count, 0);
        assert!(audit.passes_hidden_pane_policy);
    }

    #[test]
    fn indexing_budget_decision_can_be_recorded_as_activity_row() {
        let decision = WorkloadBudgetDecision::for_state(
            WorkloadFamily::IndexingRefresh,
            EfficiencyState::ProtectCore,
            EfficiencyPressureSource::CriticalBattery,
            "2026-05-14T08:42:00Z",
        );
        let row = decision
            .indexing_activity_row("workspace-alpha", "2026-05-14T08:42:00Z")
            .expect("indexing decision should map to activity row");
        assert_eq!(row.job_family, ActivityJobFamily::Indexing);
        assert_eq!(row.source_subsystem, SourceSubsystem::RuntimePowerManager);
        assert!(row.has_exact_reopen_identity());
        assert!(row.satisfies_sensitive_detail_rule());
        assert_eq!(row.state_class, ActivityRowStateClass::QueuedWaiting);
    }
}
